//! T4-U4b (DEL-04-07 `u4b-u5b-implementation.md` §2.7): the algebra-class marks.
//! Every row of the pressure-2, preview-physics-2 and inheriting tables carries
//! `combination_algebra`, one class per kind; the committed corpora publish no
//! `never_combined` kind in a combination, and each `recomputed_from_combined`
//! row of a combination is the norm of its own combination's component rows,
//! not a combination of operand magnitudes, bit for bit with `norm3`. A range
//! envelope's rows (`preview_physics_range_envelope_sparse`, the producer's
//! envelope of an invented two-case model with a max-mode range envelope) are
//! each its own operands' mode-selected row, magnitudes included: a magnitude
//! keeps its per-row selection, independently of the component envelopes, under
//! the reader's existing exemption.
use open_pipe_stress_correct_norm::norm3;
use serde_json::Value;
use std::collections::{BTreeMap, HashMap};
use std::path::{Path, PathBuf};

const CLASSES: [&str; 3] = ["signed_linear", "recomputed_from_combined", "never_combined"];
const RANGE_BASIS: &str = "explicit_user_range_envelope";
const RANGE_MODES: [&str; 4] = ["max_abs", "min_abs", "max", "min"];
const TABLES: [(&str, &str); 4] = [
    ("openpipestress.result_semantics/0.3.0/pressure-2", "semantic_contract_v0_3_pressure_2.json"),
    ("openpipestress.result_semantics/0.3.0/preview-physics-2", "semantic_contract_v0_3_preview_physics_2.json"),
    ("openpipestress.result_semantics/0.3.0/physics-retained-1", "semantic_contract_v0_3_physics_retained_1.json"),
    ("openpipestress.result_semantics/0.3.0/preview-physics-retained-1", "semantic_contract_v0_3_preview_physics_retained_1.json"),
];

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..")
}
fn read(path: &Path) -> Option<Value> {
    serde_json::from_str(&std::fs::read_to_string(path).ok()?).ok()
}

/// kind -> class of one table, every row marked and each kind in one class.
fn classes(table: &str) -> BTreeMap<String, String> {
    let table = read(&root().join("fixtures/results").join(table)).unwrap();
    let mut out = BTreeMap::new();
    for row in table["rows"].as_array().unwrap() {
        let kind = row["kind"].as_str().unwrap().to_string();
        let class = row["combination_algebra"].as_str().unwrap_or_else(|| panic!("{kind}: unmarked"));
        assert!(CLASSES.contains(&class), "{kind}: {class}");
        let previous = out.insert(kind.clone(), class.to_string());
        assert!(previous.is_none_or(|p| p == class), "{kind}: two classes");
    }
    out
}

#[test]
fn every_marked_table_row_carries_one_class_per_kind() {
    for (_, table) in TABLES {
        let classes = classes(table);
        for kind in ["displacement_magnitude"] {
            assert_eq!(classes[kind], "recomputed_from_combined", "{table}");
        }
        for (kind, class) in &classes {
            let derived = ["element_local_normal_stress_extreme", "element_local_transverse_shear_stress_maximum"];
            if derived.contains(&kind.as_str()) || kind.contains("maximum") || kind.contains("intensified") {
                assert_eq!(class, "never_combined", "{table} {kind}");
            }
            if kind.ends_with("_magnitude_v2") || kind == "displacement_magnitude" {
                assert_eq!(class, "recomputed_from_combined", "{table} {kind}");
            }
        }
    }
}

fn envelopes(dir: &Path, out: &mut Vec<(PathBuf, Value)>) {
    for entry in std::fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            envelopes(&path, out);
        } else if path.extension().is_some_and(|e| e == "json") {
            if let Some(doc) = read(&path) {
                if doc["results"].is_array() && doc["producer"]["semantic_contract_id"].is_string() {
                    out.push((path, doc));
                }
            }
        }
    }
}

#[test]
fn committed_combinations_follow_their_algebra_class() {
    let tables: HashMap<&str, BTreeMap<String, String>> = TABLES.iter().map(|(id, t)| (*id, classes(t))).collect();
    let mut docs = Vec::new();
    envelopes(&root().join("fixtures/results"), &mut docs);
    envelopes(&root().join("fixtures/product_preview"), &mut docs);
    let (mut combined, mut recomputed, mut ranged, mut range_magnitudes, mut selected_not_norm) = (0, 0, 0, 0, 0);
    for (path, doc) in &docs {
        let Some(classes) = tables.get(doc["producer"]["semantic_contract_id"].as_str().unwrap()) else { continue };
        let rows: HashMap<&str, &Value> = doc["results"].as_array().unwrap().iter().filter_map(|r| Some((r["id"].as_str()?, r))).collect();
        // A range envelope is told apart as the reader tells it: any of its rows
        // names the range basis (magnitude rows carry no metadata); its mode is
        // that row's sign convention.
        let mut range_modes: HashMap<&str, &str> = HashMap::new();
        for row in doc["results"].as_array().unwrap().iter().filter(|r| r["metadata"]["basis"] == RANGE_BASIS) {
            let mode = RANGE_MODES.iter().find(|m| row["metadata"]["sign_convention"].as_str().is_some_and(|s| s.starts_with(&format!("value is the {m} mode-selected")))).expect("a range mode");
            let previous = range_modes.insert(row["basis_ref"]["ref_id"].as_str().unwrap(), mode);
            assert!(previous.is_none_or(|p| p == *mode), "{}: one mode per range envelope", row["id"]);
        }
        for row in doc["results"].as_array().unwrap() {
            if row["basis_ref"]["ref_type"] != "combination" {
                continue;
            }
            let kind = row["kind"].as_str().unwrap();
            if kind == "combination_modulus_basis_record" {
                // The combination's own modulus-basis record (a basis row, classed
                // never_combined): it states the combination's basis and combines no
                // operand value.
                continue;
            }
            combined += 1;
            let class = classes.get(kind).map(String::as_str);
            assert!(matches!(class, Some("signed_linear" | "recomputed_from_combined")), "{}: {kind} {class:?}", path.display());
            if let Some(mode) = range_modes.get(row["basis_ref"]["ref_id"].as_str().unwrap()) {
                // §2.7: every range-envelope row, a magnitude included, is its own
                // operands' mode-selected row.
                ranged += 1;
                let operands: Vec<f64> = row["source_result_refs"].as_array().unwrap().iter().map(|id| {
                    let operand = rows[id.as_str().unwrap()];
                    assert!(operand["kind"] == row["kind"] && operand["entity_ref"] == row["entity_ref"] && operand["basis_ref"]["ref_type"] == "load_case", "{}", row["id"]);
                    operand["value"].as_f64().unwrap()
                }).collect();
                let value = row["value"].as_f64().unwrap();
                assert!(operands.len() >= 2 && operands.iter().any(|v| v.to_bits() == value.to_bits()), "{}: a selected operand", row["id"]);
                assert!(operands.iter().all(|v| match *mode {
                    "max" => *v <= value,
                    "min" => *v >= value,
                    "max_abs" => v.abs() <= value.abs(),
                    _ => v.abs() >= value.abs(),
                }), "{}: the {mode} selection", row["id"]);
                if class == Some("recomputed_from_combined") {
                    range_magnitudes += 1;
                    // Not the norm of the envelope's own components: a per-row selection.
                    let own = |component: &str| doc["results"].as_array().unwrap().iter().find(|r| r["basis_ref"] == row["basis_ref"]
                        && r["entity_ref"] == row["entity_ref"] && r["metadata"]["component"] == component).and_then(|r| r["value"].as_f64());
                    let components: &[&str] = match row["metadata"]["component"].as_str() {
                        Some("force_magnitude") => &["Fx", "Fy", "Fz"],
                        Some("moment_magnitude") => &["Mx", "My", "Mz"],
                        _ => &["nodal_displacement_x", "nodal_displacement_y", "nodal_displacement_z"],
                    };
                    if let [Some(a), Some(b), Some(c)] = [own(components[0]), own(components[1]), own(components[2])] {
                        if norm3(a, b, c).to_bits() != value.to_bits() {
                            selected_not_norm += 1;
                        }
                    }
                }
                continue;
            }
            if class == Some("recomputed_from_combined") {
                recomputed += 1;
                // A support magnitude names all six components; it is the norm of its own three.
                let own: &[&str] = match row["metadata"]["component"].as_str() {
                    Some("force_magnitude") => &["Fx", "Fy", "Fz"],
                    Some("moment_magnitude") => &["Mx", "My", "Mz"],
                    _ => &[],
                };
                let parts: Vec<&Value> = row["source_result_refs"].as_array().unwrap().iter()
                    .map(|id| rows[id.as_str().unwrap()])
                    .filter(|p| own.is_empty() || own.contains(&p["metadata"]["component"].as_str().unwrap_or("")))
                    .collect();
                assert_eq!(parts.len(), 3, "{}", row["id"]);
                // The norm of its own combination's components, not of operand magnitudes.
                assert!(parts.iter().all(|p| p["basis_ref"] == row["basis_ref"] && classes[p["kind"].as_str().unwrap()] == "signed_linear"), "{}", row["id"]);
                let v: Vec<f64> = parts.iter().map(|p| p["value"].as_f64().unwrap()).collect();
                let norm = norm3(v[0], v[1], v[2]);
                let value = row["value"].as_f64().unwrap();
                assert_eq!(value.to_bits(), norm.to_bits(), "{}: {value} vs norm3 {norm}", row["id"]);
            }
        }
    }
    assert!(combined > 0 && recomputed > 0, "the committed corpora exercise combinations ({combined}, {recomputed})");
    assert!(ranged > 0 && range_magnitudes > 0, "the corpora carry a range envelope with magnitudes ({ranged}, {range_magnitudes})");
    assert!(selected_not_norm > 0, "a range magnitude differs from the norm of its envelope's components, so the selection is per row");
}
