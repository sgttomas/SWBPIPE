//! Rule-pack format 0.5.0 row selection and required load cases (W-17;
//! DEL-06-01 Design `rule-pack-0.5.0.md` §2.5, §2.6, §5). Packs and models are
//! invented; envelopes are the committed result and witness fixtures. Nothing
//! here is a professional or code-compliance claim.

use super::*;
use serde_json::json;
use std::path::{Path, PathBuf};

fn project_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..")
}

fn read_json(relative: &str) -> Value {
    let path = project_root().join(relative);
    let text = std::fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("{}: {error}", path.display()));
    serde_json::from_str(&text).unwrap_or_else(|error| panic!("{}: {error}", path.display()))
}

/// Every object holding a `results` array with at least one basis-bearing row.
fn envelopes<'a>(value: &'a Value, out: &mut Vec<&'a Value>) {
    match value {
        Value::Object(map) => {
            if map
                .get("results")
                .and_then(Value::as_array)
                .is_some_and(|rows| rows.iter().any(|row| row.get("basis_ref").is_some()))
            {
                out.push(value);
            }
            for child in map.values() {
                envelopes(child, out);
            }
        }
        Value::Array(items) => {
            for child in items {
                envelopes(child, out);
            }
        }
        _ => {}
    }
}

fn json_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let mut entries: Vec<_> = std::fs::read_dir(dir)
        .unwrap_or_else(|error| panic!("{}: {error}", dir.display()))
        .map(|entry| entry.unwrap().path())
        .collect();
    entries.sort();
    for path in entries {
        if path.is_dir() {
            json_files(&path, out);
        } else if path.extension().is_some_and(|ext| ext == "json") {
            out.push(path);
        }
    }
}

fn selector(value: Value) -> SolverResultSelector {
    parse_solver_result_ref(&value).expect("a well-formed invented selector")
}

fn load_case(id: &str) -> BasisRef {
    BasisRef::new("load_case", id)
}

/// §5: in every committed result and witness envelope, every row that has a
/// `basis_ref` is selected by exactly one selector within its basis, its own
/// (the connector reader corpus included).
#[test]
fn every_basis_bearing_committed_row_is_selected_by_exactly_its_own_selector() {
    let mut files = Vec::new();
    for dir in ["fixtures/results", "validation/witness"] {
        json_files(&project_root().join(dir), &mut files);
    }
    let corpus = project_root().join("fixtures/results/pressure_v3_connector_reader_corpus.json");
    assert!(files.contains(&corpus));
    let (mut file_count, mut envelope_count, mut row_count, mut rod_rows) = (0, 0, 0, 0);
    let mut corpus_rows = 0;
    for path in &files {
        let text = std::fs::read_to_string(path).unwrap();
        let Ok(document) = serde_json::from_str::<Value>(&text) else {
            continue;
        };
        let mut found = Vec::new();
        envelopes(&document, &mut found);
        if found.is_empty() {
            continue;
        }
        file_count += 1;
        for envelope in found {
            envelope_count += 1;
            let index = RowIndex::new(envelope);
            for row in envelope["results"].as_array().unwrap() {
                let Some(basis) = row.get("basis_ref").and_then(BasisRef::from_value) else {
                    continue;
                };
                let selector = selector_for_row(envelope, row).unwrap_or_else(|| {
                    panic!("{}: no selector names {}", path.display(), row["id"])
                });
                let selected = index.select(&selector, &basis);
                assert_eq!(
                    selected.len(),
                    1,
                    "{}: {} rows match the selector of {} ({:?})",
                    path.display(),
                    selected.len(),
                    row["id"],
                    selected
                        .iter()
                        .map(|row| row["id"].clone())
                        .collect::<Vec<_>>()
                );
                assert!(
                    std::ptr::eq(selected[0], row),
                    "{}: {}",
                    path.display(),
                    row["id"]
                );
                row_count += 1;
                rod_rows += usize::from(selector.tie_rod.is_some());
                corpus_rows += usize::from(path == &corpus);
            }
        }
    }
    // Coverage at the time of writing (W-17): 37 files, 55 envelopes, 20 856
    // basis-bearing rows, 6 tie-rod rows. More committed rows only widen it.
    assert!(file_count >= 37, "{file_count} files");
    assert!(envelope_count >= 55, "{envelope_count} envelopes");
    assert!(row_count >= 20_856, "{row_count} rows");
    assert!(rod_rows >= 6, "{rod_rows} tie-rod rows");
    assert!(corpus_rows > 0);
    eprintln!("selector uniqueness: {file_count} files, {envelope_count} envelopes, {row_count} rows, {rod_rows} tie-rod rows");
}

/// The tie-rod matcher restates the reader's binding (`connector_evidence.rs`
/// `tie_rod_row_tail` / `is_tie_rod_row_id`, from `joint.rs` `tie_rod_row_id`).
/// On every corpus envelope the reader admits (so it bound each rod row to one
/// rod of its record), the matcher binds each rod row to exactly one rod of
/// the same record; a row naming no record rod is refused by the reader and
/// matched by no rod here.
#[test]
fn tie_rod_matcher_agrees_with_the_reader_binding_on_the_corpus() {
    let corpus = read_json("fixtures/results/pressure_v3_connector_reader_corpus.json");
    let mut checked = 0;
    for case in corpus["cases"].as_array().unwrap() {
        let envelope = &case["envelope"];
        let rod_rows: Vec<&Value> = envelope["results"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|row| row["kind"] == TIE_ROD_KIND)
            .collect();
        if rod_rows.is_empty() {
            continue;
        }
        open_pipe_stress_result_export::semantic_contract::validate_pressure_evidence(envelope)
            .unwrap_or_else(|error| panic!("{}: {error}", case["label"]));
        for row in &rod_rows {
            let entity = row["entity_ref"].as_str().unwrap();
            let case_id = row["basis_ref"]["ref_id"].as_str().unwrap();
            let rods: Vec<&str> = envelope["contract_evidence"]["connector"]
                .as_array()
                .unwrap()
                .iter()
                .filter(|record| record["component_id"] == entity)
                .flat_map(|record| record["tie_rods"].as_array().into_iter().flatten())
                .filter_map(|rod| rod["id"].as_str())
                .collect();
            let bound: Vec<&&str> = rods
                .iter()
                .filter(|rod| {
                    is_tie_rod_row_id(
                        row["id"].as_str().unwrap(),
                        case_id,
                        &tie_rod_row_tail(entity, rod),
                    )
                })
                .collect();
            assert_eq!(bound.len(), 1, "{}: {}", case["label"], row["id"]);
            // The selector side resolves the same exact rod id from the record.
            assert_eq!(
                row_tie_rod(envelope, row, &load_case(case_id)),
                Some(*bound[0]),
                "{}: {}",
                case["label"],
                row["id"]
            );
            checked += 1;
        }
        // A rod row renamed to a rod the record does not hold: the reader
        // refuses it by name, and no record rod matches it here.
        let mut forged = envelope.clone();
        let row = forged["results"]
            .as_array_mut()
            .unwrap()
            .iter_mut()
            .find(|row| row["kind"] == TIE_ROD_KIND)
            .unwrap();
        let entity = row["entity_ref"].as_str().unwrap().to_string();
        row["id"] = json!(format!(
            "result:{}",
            tie_rod_row_tail(&entity, "rod:invented-absent")
        ));
        let error =
            open_pipe_stress_result_export::semantic_contract::validate_pressure_evidence(&forged)
                .unwrap_err();
        assert!(
            error.contains("TIE_ROD_UNBOUND"),
            "{}: {error}",
            case["label"]
        );
        let forged_row = forged["results"]
            .as_array()
            .unwrap()
            .iter()
            .find(|row| row["kind"] == TIE_ROD_KIND)
            .unwrap();
        assert!(selector_for_row(&forged, forged_row).is_none());
    }
    assert!(checked >= 6, "{checked} rod rows");
}

/// §5: two tie rods of one connector are selected separately; without
/// `tie_rod` neither is selected (an absent member matches only a row without
/// a rod).
#[test]
fn two_tie_rods_of_one_connector_are_selected_separately() {
    let corpus = read_json("fixtures/results/pressure_v3_connector_reader_corpus.json");
    let envelope = corpus["cases"]
        .as_array()
        .unwrap()
        .iter()
        .map(|case| &case["envelope"])
        .find(|envelope| {
            envelope["results"]
                .as_array()
                .unwrap()
                .iter()
                .filter(|row| row["kind"] == TIE_ROD_KIND)
                .count()
                == 2
        })
        .expect("a corpus envelope with two rods on one connector");
    let index = RowIndex::new(envelope);
    let basis = load_case("load:U5A");
    let base = json!({"result_kind": TIE_ROD_KIND, "entity_ref": "component:C-J", "location": "tie_rod", "component": "t_r"});
    let mut ids = Vec::new();
    for rod in ["rod:yp", "rod:ym"] {
        let mut member = base.clone();
        member["tie_rod"] = json!(rod);
        let rows = index.select(&selector(member), &basis);
        assert_eq!(rows.len(), 1, "{rod}");
        ids.push(rows[0]["id"].as_str().unwrap().to_string());
    }
    assert_ne!(ids[0], ids[1]);
    assert!(
        ids[0].ends_with("rod-yp:t_r") && ids[1].ends_with("rod-ym:t_r"),
        "{ids:?}"
    );
    assert!(index.select(&selector(base), &basis).is_empty());
}

/// The corpus envelope with two rods (`rod:yp`, `rod:ym`) on `component:C-J`.
fn two_rod_envelope() -> Value {
    let corpus = read_json("fixtures/results/pressure_v3_connector_reader_corpus.json");
    corpus["cases"]
        .as_array()
        .unwrap()
        .iter()
        .map(|case| case["envelope"].clone())
        .find(|envelope| {
            envelope["results"]
                .as_array()
                .unwrap()
                .iter()
                .filter(|row| row["kind"] == TIE_ROD_KIND)
                .count()
                == 2
        })
        .expect("a corpus envelope with two rods on one connector")
}

/// The ids of the rows a `tie_rod` selector selects on `load:U5A`.
fn rod_rows(envelope: &Value, rod: &str) -> Vec<String> {
    let member = json!({"result_kind": TIE_ROD_KIND, "entity_ref": "component:C-J", "location": "tie_rod", "component": "t_r", "tie_rod": rod});
    RowIndex::new(envelope)
        .select(&selector(member), &load_case("load:U5A"))
        .iter()
        .map(|row| row["id"].as_str().unwrap().to_string())
        .collect()
}

/// The `rod:yp` entry of the connector record's `tie_rods`.
fn record_rod(envelope: &mut Value, index: usize) -> &mut Value {
    &mut envelope["contract_evidence"]["connector"][0]["tie_rods"][index]
}

/// §2.5 (PR #1629 review r1 finding 1): a row id encodes its rod lossily
/// (`:` becomes `-`), so a selector binds a rod row only when its `tie_rod`
/// equals, exactly, the rod id the connector record holds for that row. An
/// alias that encodes to the same row id, a rod the record no longer holds,
/// and a row two record rods (or two records) could own all bind nothing.
#[test]
fn a_tie_rod_selector_binds_only_the_exact_rod_id_of_the_record() {
    let envelope = two_rod_envelope();
    assert_eq!(
        envelope["contract_evidence"]["connector"]
            .as_array()
            .unwrap()
            .len(),
        1
    );
    assert_eq!(record_rod(&mut envelope.clone(), 0)["id"], "rod:yp");
    let yp = rod_rows(&envelope, "rod:yp");
    let ym = rod_rows(&envelope, "rod:ym");
    assert_eq!(yp.len(), 1);
    assert_eq!(ym.len(), 1);
    assert_ne!(yp, ym);

    // Alias: `rod-yp` is not a rod of the record although it encodes to the
    // same row id as `rod:yp`.
    assert!(rod_rows(&envelope, "rod-yp").is_empty());

    // Renamed: the record's `rod:yp` is now `rod-yp` (same encoded row id).
    // The old name binds nothing; the record's exact id binds the row.
    let mut renamed = envelope.clone();
    record_rod(&mut renamed, 0)["id"] = json!("rod-yp");
    assert!(rod_rows(&renamed, "rod:yp").is_empty());
    assert_eq!(rod_rows(&renamed, "rod-yp"), yp);
    assert_eq!(rod_rows(&renamed, "rod:ym"), ym);

    // Removed: the record no longer holds a rod for the `rod-yp` row; neither
    // the old name nor the new rod's name binds that row.
    let mut removed = envelope.clone();
    record_rod(&mut removed, 0)["id"] = json!("rod:zz");
    assert!(rod_rows(&removed, "rod:yp").is_empty());
    assert!(rod_rows(&removed, "rod:zz").is_empty());
    assert_eq!(rod_rows(&removed, "rod:ym"), ym);

    // Ambiguous record: two of its rods encode to the row's id; the row
    // belongs to neither, so no selector binds it (the reader refuses such a
    // record as TIE_ROD_DUPLICATE).
    let mut doubled = envelope.clone();
    let mut alias = record_rod(&mut doubled, 0).clone();
    alias["id"] = json!("rod-yp");
    doubled["contract_evidence"]["connector"][0]["tie_rods"]
        .as_array_mut()
        .unwrap()
        .push(alias);
    assert!(rod_rows(&doubled, "rod:yp").is_empty());
    assert!(rod_rows(&doubled, "rod-yp").is_empty());
    assert_eq!(rod_rows(&doubled, "rod:ym"), ym);

    // Two records for one connector: no rod row of it binds.
    let mut twice = envelope.clone();
    let record = twice["contract_evidence"]["connector"][0].clone();
    twice["contract_evidence"]["connector"]
        .as_array_mut()
        .unwrap()
        .push(record);
    assert!(rod_rows(&twice, "rod:yp").is_empty());
    assert!(rod_rows(&twice, "rod:ym").is_empty());

    // Through the run resolution: the alias is RULE_SOLVER_RESULT_ROW_MISSING
    // on its basis, never a binding.
    let model = json!({"load_cases": [{"id": "load:U5A"}]});
    for (rod, binds) in [("rod:yp", true), ("rod-yp", false)] {
        let document = json!({
            "required_load_cases": [{"required_load_case_id": "rlc", "basis_ref": {"ref_type": "load_case", "ref_id": "load:U5A"}}],
            "required_inputs": [{"input_id": "actual", "source_kind": "solver_result", "solver_result_ref":
                {"result_kind": TIE_ROD_KIND, "entity_ref": "component:C-J", "location": "tie_rod", "component": "t_r", "tie_rod": rod}}]
        });
        let resolution = resolve_rule_pack_solver_inputs(&document, &model, &envelope);
        let reasons: Vec<&str> = resolution
            .refused_solver_results
            .iter()
            .map(|r| r.reason.as_str())
            .collect();
        if binds {
            // The row is matched: bound, or refused by the per-row gate only.
            assert_eq!(
                resolution.solver_results.len() + reasons.len(),
                1,
                "{rod}: {reasons:?}"
            );
            assert!(
                !reasons.contains(&RULE_SOLVER_RESULT_ROW_MISSING)
                    && !reasons.contains(&RULE_SOLVER_RESULT_ROW_AMBIGUOUS),
                "{rod}: {reasons:?}"
            );
        } else {
            assert!(resolution.solver_results.is_empty(), "{rod}");
            assert_eq!(reasons, vec![RULE_SOLVER_RESULT_ROW_MISSING], "{rod}");
        }
    }
}

/// A dense pressure envelope: nominal and reduced section twins differ only by
/// the section basis; a selector without it matches neither, each named basis
/// matches its own twin; a malformed token matches nothing.
#[test]
fn section_basis_separates_the_twins_and_a_malformed_token_matches_nothing() {
    let envelope = read_json("fixtures/results/pressure_connected_mechanics_dense.json");
    let row = envelope["results"]
        .as_array()
        .unwrap()
        .iter()
        .find(|row| {
            matches!(
                row_section_basis(row),
                RowSectionBasis::Named("nominal_section")
            )
        })
        .expect("a nominal-section row");
    let basis = BasisRef::from_value(&row["basis_ref"]).unwrap();
    let mut member = serde_json::to_value(selector_for_row(&envelope, row).unwrap()).unwrap();
    let index = RowIndex::new(&envelope);
    assert_eq!(
        index.select(&selector(member.clone()), &basis)[0]["id"],
        row["id"]
    );
    member["section_basis"] = json!("reduced_section");
    let twin = index.select(&selector(member.clone()), &basis);
    assert_eq!(twin.len(), 1);
    assert_ne!(twin[0]["id"], row["id"]);
    member.as_object_mut().unwrap().remove("section_basis");
    assert!(index.select(&selector(member.clone()), &basis).is_empty());

    let mut malformed = envelope.clone();
    let rows = malformed["results"].as_array_mut().unwrap();
    let target = rows
        .iter_mut()
        .find(|candidate| candidate["id"] == row["id"])
        .unwrap();
    target["metadata"]["basis"] = json!("section_basis=nominal_section");
    assert_eq!(row_section_basis(target), RowSectionBasis::Malformed);
    member["section_basis"] = json!("nominal_section");
    assert!(RowIndex::new(&malformed)
        .select(&selector(member), &basis)
        .is_empty());
}

/// An invented two-case model with a combination, and an envelope whose rows
/// differ by case.
fn two_case_model() -> Value {
    json!({
        "project": {"id": "project:invented-bases"},
        "load_cases": [{"id": "load:A"}, {"id": "load:B"}],
        "combinations": [{"id": "combination:AB"}, {"id": "combination:withheld"}]
    })
}

fn row(id: &str, basis: (&str, &str), value: f64, location: Option<&str>) -> Value {
    let mut row = json!({
        "id": id, "kind": "invented_stress", "entity_ref": "pipe:P-1", "value": value, "unit": "MPa",
        "basis_ref": {"ref_type": basis.0, "ref_id": basis.1},
        "metadata": {"component": "sigma"}
    });
    if let Some(location) = location {
        row["metadata"]["location"] = json!(location);
    }
    row
}

fn two_case_envelope() -> Value {
    json!({"results": [
        row("result:a:end-i", ("load_case", "load:A"), 10.0, Some("end_i")),
        row("result:a:end-j", ("load_case", "load:A"), 11.0, Some("end_j")),
        row("result:b:end-i", ("load_case", "load:B"), 20.0, Some("end_i")),
        row("result:b:end-j", ("load_case", "load:B"), 21.0, Some("end_j")),
        row("result:ab:end-i", ("combination", "combination:AB"), 30.0, Some("end_i")),
        row("result:ab:end-j", ("combination", "combination:AB"), 31.0, Some("end_j")),
        // A model-input echo row without basis_ref is never selected.
        {"id": "result:echo", "kind": "invented_stress", "entity_ref": "pipe:P-1", "value": 99.0, "unit": "MPa",
         "metadata": {"component": "sigma", "location": "end_i"}}
    ]})
}

fn pack(cases: &[(&str, &str, &str)], solver_ref: Value) -> Value {
    json!({
        "required_load_cases": cases.iter().map(|(id, kind, ref_id)| json!({
            "required_load_case_id": id, "basis_ref": {"ref_type": kind, "ref_id": ref_id}
        })).collect::<Vec<_>>(),
        "required_inputs": [
            {"input_id": "actual", "source_kind": "solver_result", "solver_result_ref": solver_ref},
            {"input_id": "limit", "source_kind": "user_supplied_rule_value"}
        ]
    })
}

#[test]
fn each_basis_binds_its_own_row_and_unavailable_bases_are_named() {
    let cases = [
        ("rlc:a", "load_case", "load:A"),
        ("rlc:b", "load_case", "load:B"),
        ("rlc:ab", "combination", "combination:AB"),
        ("rlc:missing", "load_case", "load:absent"),
        ("rlc:withheld", "combination", "combination:withheld"),
    ];
    let document = pack(
        &cases,
        json!({"result_kind": "invented_stress", "entity_ref": "pipe:P-1", "location": "end_i", "component": "sigma"}),
    );
    let resolution =
        resolve_rule_pack_solver_inputs(&document, &two_case_model(), &two_case_envelope());
    let bound: Vec<(String, String, f64)> = resolution
        .solver_results
        .iter()
        .map(|b| (b.basis_ref.ref_id.clone(), b.result_id.clone(), b.value))
        .collect();
    assert_eq!(
        bound,
        vec![
            ("load:A".to_string(), "result:a:end-i".to_string(), 10.0),
            ("load:B".to_string(), "result:b:end-i".to_string(), 20.0),
            (
                "combination:AB".to_string(),
                "result:ab:end-i".to_string(),
                30.0
            ),
        ]
    );
    assert!(resolution.refused_solver_results.is_empty());
    let unavailable: Vec<(&str, &str, &str)> = resolution
        .unavailable_bases
        .iter()
        .map(|u| {
            (
                u.required_load_case_id.as_str(),
                u.basis_ref.ref_id.as_str(),
                u.code.as_str(),
            )
        })
        .collect();
    assert_eq!(
        unavailable,
        vec![
            (
                "rlc:missing",
                "load:absent",
                RULE_REQUIRED_LOAD_CASE_MISSING
            ),
            (
                "rlc:withheld",
                "combination:withheld",
                RULE_REQUIRED_LOAD_CASE_UNSOLVED
            ),
        ]
    );
}

#[test]
fn a_missing_or_ambiguous_row_is_refused_on_its_basis_only() {
    let cases = [
        ("rlc:a", "load_case", "load:A"),
        ("rlc:b", "load_case", "load:B"),
    ];
    // No location: an absent member matches only rows without it, and every
    // row carries metadata.location, so this selector matches nothing on
    // either basis (never the first row found).
    let none = pack(
        &cases,
        json!({"result_kind": "invented_stress", "entity_ref": "pipe:P-1", "component": "sigma"}),
    );
    let resolution =
        resolve_rule_pack_solver_inputs(&none, &two_case_model(), &two_case_envelope());
    assert!(resolution.solver_results.is_empty());
    assert!(resolution
        .refused_solver_results
        .iter()
        .all(|r| r.reason == RULE_SOLVER_RESULT_ROW_MISSING));
    assert_eq!(resolution.refused_solver_results.len(), 2);

    let mut envelope = two_case_envelope();
    // A duplicate of load:B's end-i row makes that basis ambiguous; load:A
    // still binds.
    let duplicate = row(
        "result:b:end-i:copy",
        ("load_case", "load:B"),
        25.0,
        Some("end_i"),
    );
    envelope["results"].as_array_mut().unwrap().push(duplicate);
    let document = pack(
        &cases,
        json!({"result_kind": "invented_stress", "entity_ref": "pipe:P-1", "location": "end_i", "component": "sigma"}),
    );
    let resolution = resolve_rule_pack_solver_inputs(&document, &two_case_model(), &envelope);
    assert_eq!(resolution.solver_results.len(), 1);
    assert_eq!(resolution.solver_results[0].basis_ref, load_case("load:A"));
    assert_eq!(resolution.refused_solver_results.len(), 1);
    let refusal = &resolution.refused_solver_results[0];
    assert_eq!(
        (refusal.reason.as_str(), refusal.basis_ref.clone()),
        (RULE_SOLVER_RESULT_ROW_AMBIGUOUS, load_case("load:B"))
    );
    assert!(refusal
        .message
        .as_deref()
        .unwrap()
        .contains("result:b:end-i:copy"));
}

#[test]
fn selection_is_stable_when_the_envelope_rows_are_reordered() {
    let cases = [
        ("rlc:a", "load_case", "load:A"),
        ("rlc:b", "load_case", "load:B"),
        ("rlc:ab", "combination", "combination:AB"),
    ];
    let document = pack(
        &cases,
        json!({"result_kind": "invented_stress", "entity_ref": "pipe:P-1", "location": "end_j", "component": "sigma"}),
    );
    let forward =
        resolve_rule_pack_solver_inputs(&document, &two_case_model(), &two_case_envelope());
    let mut reversed = two_case_envelope();
    reversed["results"].as_array_mut().unwrap().reverse();
    let backward = resolve_rule_pack_solver_inputs(&document, &two_case_model(), &reversed);
    let key = |r: &SolverInputResolution| {
        r.solver_results
            .iter()
            .map(|b| (b.basis_ref.clone(), b.result_id.clone(), b.value.to_bits()))
            .collect::<Vec<_>>()
    };
    assert_eq!(key(&forward), key(&backward));
    assert_eq!(forward.solver_results.len(), 3);
}

/// §2.5: a retained row that `rule_binding_refusal` refuses is refused on its
/// basis, with the reader's reason, and never bound; the shared helper is the
/// gate, applied per matched row.
#[test]
fn a_refused_retained_row_is_refused_on_its_basis_and_never_bound() {
    let mut files = Vec::new();
    json_files(&project_root().join("fixtures/results"), &mut files);
    json_files(&project_root().join("validation/witness"), &mut files);
    let mut seen = 0;
    for path in files {
        let Ok(document) = serde_json::from_str::<Value>(&std::fs::read_to_string(&path).unwrap())
        else {
            continue;
        };
        let mut found = Vec::new();
        envelopes(&document, &mut found);
        for envelope in found {
            for row in envelope["results"].as_array().unwrap() {
                let Some(reason) =
                    open_pipe_stress_result_export::semantic_contract::rule_binding_refusal(
                        envelope, row,
                    )
                else {
                    continue;
                };
                let Some(basis) = row.get("basis_ref").and_then(BasisRef::from_value) else {
                    continue;
                };
                let selector = selector_for_row(envelope, row).unwrap();
                let model = json!({"load_cases": [{"id": basis.ref_id}], "combinations": [{"id": basis.ref_id}]});
                let document = json!({
                    "required_load_cases": [{"required_load_case_id": "rlc", "basis_ref": basis}],
                    "required_inputs": [{"input_id": "actual", "source_kind": "solver_result", "solver_result_ref": selector}]
                });
                let resolution = resolve_rule_pack_solver_inputs(&document, &model, envelope);
                assert!(
                    resolution.solver_results.is_empty(),
                    "{}: {}",
                    path.display(),
                    row["id"]
                );
                assert_eq!(resolution.refused_solver_results.len(), 1);
                assert_eq!(resolution.refused_solver_results[0].reason, reason);
                assert_eq!(resolution.refused_solver_results[0].basis_ref, basis);
                seen += 1;
                if seen >= 25 {
                    return;
                }
            }
        }
    }
    assert!(
        seen > 0,
        "no committed retained row is refused by rule_binding_refusal"
    );
}
