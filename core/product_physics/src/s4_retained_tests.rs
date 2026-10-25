//! T4-U4b S4 (DEL-04-07 Design `u4b-u5b-implementation.md` §4.1, §4.2; the owner's Q2,
//! "Form from own resultants"): the straight-member derived kinds on W1-L and W1-E successors.
//!
//! - Every derived row (σ⁺, σ⁻ and τ_v,max at five stations on each basis) is a recipe-free
//!   `not_covered` row: the kernel's verdict is class `None` on a `NotCovered` recipe, the receipt
//!   lists it under `selection.not_covered`, and the Rust reader classifies it `not_covered`.
//! - Its value is formed after projection from the successor's own rows: σ± recomputed here bit
//!   for bit; τ by the base reader holding the captured model (`reader_verdict::validate`, G2).
//! - Deliberately divergent ordinary resultants are not the derived rows' operands.
//! - No not-covered row changes a verified row's class, bound or scale.
use super::*;
use open_pipe_stress_correct_norm::norm2;
use open_pipe_stress_result_export::{reader_verdict, retained_precision as reader};

/// The two derived kinds W1 carries (straight members only; arcs never reach W1).
pub(super) const DERIVED: [&str; 2] = ["element_local_normal_stress_extreme", "element_local_transverse_shear_stress_maximum"];
/// W-7 (DEL-04-01 Design `global-end-actions.md`, GLOBAL_END_ROWS_NOT_COVERED): the global end
/// actions, the other recipe-free not-covered rows W1 carries (twelve per member).
pub(super) const GLOBAL_END: [&str; 2] = ["element_global_end_force_v1", "element_global_end_moment_v1"];
const STATIONS: [&str; 5] = ["end_i", "end_j", "quarter_1", "midspan", "quarter_3"];

fn basis_of(row: &Value) -> Option<&str> {
    row["metadata"]["basis"].as_str().and_then(|b| b.rsplit_once("; section_basis=")).map(|(_, b)| b)
}
fn one<'a>(rows: &'a [Value], of: &Value, kind: &str, basis: Option<&str>) -> &'a Value {
    let found: Vec<&Value> = rows.iter().filter(|r| r["kind"] == kind && r["entity_ref"] == of["entity_ref"]
        && r["basis_ref"] == of["basis_ref"] && r["metadata"]["location"] == of["metadata"]["location"] && basis_of(r) == basis).collect();
    assert_eq!(found.len(), 1, "{kind} beside {}", of["id"]);
    found[0]
}
fn bits(v: &Value) -> u64 {
    v.as_f64().unwrap().to_bits()
}

/// Every derived row of `document` is recomputed from its own rows: σ⁺/σ⁻ bit for bit from the
/// same station's and basis's axial and bending stress rows (§2.3), and τ checked against its two
/// shear rows' norm being finite (its c_b is the reader's, `base_read_with_model`). Returns the
/// number of derived rows, per member and station: two σ and one τ per basis.
pub(super) fn derived_rows_follow_own_rows(label: &str, document: &Value) -> usize {
    let rows = document["results"].as_array().unwrap();
    let mut count = 0;
    for row in rows.iter().filter(|r| DERIVED.contains(&r["kind"].as_str().unwrap())) {
        count += 1;
        assert!(STATIONS.contains(&row["metadata"]["location"].as_str().unwrap()), "{label}: {}", row["id"]);
        if row["kind"] == DERIVED[0] {
            let basis = basis_of(row);
            let n = one(rows, row, "element_local_axial_normal_stress", basis)["value"].as_f64().unwrap();
            let y = one(rows, row, "element_local_bending_normal_stress_y", basis)["value"].as_f64().unwrap();
            let z = one(rows, row, "element_local_bending_normal_stress_z", basis)["value"].as_f64().unwrap();
            let expected = if row["metadata"]["component"] == "sigma_plus" { n + norm2(y, z) } else { n - norm2(y, z) };
            assert_eq!(bits(&row["value"]), expected.to_bits(), "{label}: {} from its own stress rows", row["id"]);
        } else {
            let vy = one(rows, row, "element_local_shear_force_y", None)["value"].as_f64().unwrap();
            let vz = one(rows, row, "element_local_shear_force_z", None)["value"].as_f64().unwrap();
            assert!(norm2(vy, vz).is_finite() && row["value"].as_f64().unwrap() >= 0.0, "{label}: {}", row["id"]);
        }
    }
    count
}

/// G7's base projection, then the base reader holding the captured model (G2): σ± and τ bit for
/// bit from the successor's own rows, τ's c_b from the model (preview) or the regenerated
/// evidence's ν and sections (exact).
pub(super) fn base_read_with_model(label: &str, successor: &Value, raw: &Value) {
    let mut projected = successor.clone();
    projected.as_object_mut().unwrap().remove("retained_precision");
    let exact = successor["producer"]["semantic_contract_id"] == "openpipestress.result_semantics/0.3.0/physics-retained-1";
    let (base, profile, reader) = if exact {
        (exact_admission::PRESSURE_SEMANTIC_CONTRACT_ID, "exact_pressure_v3", "pressure")
    } else {
        (preview_physics::ID, "product_preview_mechanics_v1", "preview-physics")
    };
    projected["producer"]["semantic_contract_id"] = json!(base);
    projected["formulation_basis"]["profile_id"] = json!(profile);
    projected["results"].as_array_mut().unwrap().retain(|r| r["kind"] != rp::REQUESTED_MODE_KIND);
    let verdict = reader_verdict::validate(reader, &projected, Some(&raw["model"]), None);
    assert_eq!(verdict["ok"], json!(true), "{label}: the base reader with the model: {verdict}");
}

/// The successor's receipt lists exactly its derived rows (and any other uncovered rows) under
/// `not_covered`; the reader classifies each derived row `not_covered`.
pub(super) fn receipt_and_reader_classify(label: &str, successor: &Value, raw: &Value, mode: PreviewSolverMode) -> reader::Validation {
    let invocation = json!({"request": raw, "solver_mode": mode.as_str()});
    let validation = reader::validate(successor, Some(&invocation)).unwrap_or_else(|e| panic!("{label}: reader {e:?}"));
    assert!(validation.invocation_bound, "{label}: bound");
    let cases = successor["retained_precision"]["body"]["cases"].as_array().unwrap();
    let not_covered: Vec<Value> = cases.iter().flat_map(|c| c["selection"]["not_covered"].as_array().cloned().unwrap_or_default()).collect();
    // A `not_required` case has no retained selection; its rows stay ordinary.
    let selected: Vec<&Value> = cases.iter().filter(|c| c["status"] == "selected").map(|c| &c["basis_ref"]).collect();
    // W-7: the global end rows are not_covered beside the derived rows.
    let derived: Vec<&Value> = successor["results"].as_array().unwrap().iter()
        .filter(|r| (DERIVED.contains(&r["kind"].as_str().unwrap()) || GLOBAL_END.contains(&r["kind"].as_str().unwrap())) && selected.contains(&&r["basis_ref"])).collect();
    assert!(!derived.is_empty(), "{label}: derived rows carried");
    assert_eq!(not_covered.len(), derived.len(), "{label}: not_covered lists the selected cases' derived and global end rows and nothing else");
    for row in &derived {
        assert!(not_covered.contains(&row["id"]), "{label}: {} listed not_covered", row["id"]);
        let class = validation.classifications.iter().find(|c| c.result_id == row["id"].as_str().unwrap())
            .unwrap_or_else(|| panic!("{label}: {} classified", row["id"]));
        assert_eq!(class.class, reader::AccuracyClass::NotCovered, "{label}: {}", row["id"]);
        assert!(class.scale_bits.is_none(), "{label}: {} has no body scale", row["id"]);
    }
    validation
}

/// S4 round 2 (§4.1, §5.1): the shipped reader entry recomputes route L's τ with the invocation.
/// One τ row one ulp up, its publication and receipt resealed, is refused at G7 with the base's
/// code; without the invocation (no model, so no c_b) the read admits it but is never eligible.
pub(super) fn tau_tamper_is_refused_by_the_reader(label: &str, successor: &Value, raw: &Value, mode: PreviewSolverMode) {
    let mut doc = successor.clone();
    let at = doc["results"].as_array().unwrap().iter().position(|r| r["kind"] == DERIVED[1])
        .unwrap_or_else(|| panic!("{label}: a τ row"));
    let value = doc["results"][at]["value"].as_f64().unwrap();
    doc["results"][at]["value"] = json!(value.next_up());
    let mut publication = doc.clone();
    publication.as_object_mut().unwrap().remove("retained_precision");
    doc["retained_precision"]["body"]["publication_sha256"] = json!(crate::retained_wire::domain_hash("retained_precision_publication_mp_v2", &publication).unwrap());
    doc["retained_precision"]["receipt_sha256"] = json!(crate::retained_wire::domain_hash("retained_precision_receipt_mp_v2", &doc["retained_precision"]["body"]).unwrap());
    let invocation = json!({"request": raw, "solver_mode": mode.as_str()});
    let error = reader::validate(&doc, Some(&invocation)).err().unwrap_or_else(|| panic!("{label}: the tampered τ is refused"));
    assert_eq!((error.gate, error.code.as_str()), ("G7", "SOURCE_PREVIEW_PHYSICS_SHEAR_VALUE"), "{label}: {error:?}");
    let unbound = reader::validate(&doc, None).unwrap_or_else(|e| panic!("{label}: unbound {e:?}"));
    assert!(!unbound.numerical_eligible, "{label}: unbound is never eligible");
    reader::validate(successor, Some(&invocation)).unwrap_or_else(|e| panic!("{label}: the genuine successor {e:?}"));
}

/// W1-L (milestone, both modes): the passthrough through the whole transaction: capture,
/// NotCovered final rows (class-None verdicts), the receipt's `not_covered` entries, the reader's
/// classes and the values formed from the successor's own rows.
#[test]
fn s4_w1l_derived_rows_are_a_not_covered_passthrough_formed_from_own_rows() {
    let raw = raw();
    for mode in MODES {
        let label = format!("W1-L {}", mode.as_str());
        let (capture, observer, ordinary) = observed(mode, &raw);
        let members = raw["model"]["pipe_segments"].as_array().unwrap().len();
        let ((_, retained), _, captured) = hooks::counted_with_successor(|| retained_w1(observer, ordinary, &capture));
        let successor = captured.unwrap_or_else(|| panic!("{label}: {retained:?}"));
        assert_eq!(derived_rows_follow_own_rows(&label, &successor), 15 * members, "{label}: 15 derived rows per member");
        base_read_with_model(&label, &successor, &raw);
        let validation = receipt_and_reader_classify(&label, &successor, &raw, mode);
        assert!(validation.numerical_eligible, "{label}: eligible with the invocation");
        tau_tamper_is_refused_by_the_reader(&label, &successor, &raw, mode);
    }
}

/// The kernel side of the same transaction: each derived row binds a `NotCovered { row }` recipe at
/// its own position, its verdict is class `None` and passed, and every other verdict is exactly
/// the one the case's verified rows had without it (§7.8: no not-covered entry changes a
/// verified row's class, bound or scale).
#[test]
fn s4_w1l_not_covered_recipes_are_position_bound_class_none_verdicts() {
    let raw = raw();
    for mode in MODES {
        let label = format!("W1-L recipes {}", mode.as_str());
        let (request, capture) = source_receipt::CapturedInvocation::parse(raw.clone(), mode).unwrap();
        let mut prepared = rp::PreparedCase::prepare_observed(request, mode, &capture).unwrap_or_else(|e| panic!("{:?}", e.capture.error));
        let ordinary = prepared.ordinary().clone();
        prepared.solve_native().unwrap();
        let candidate = match prepared.project_candidate() { Ok(c) => c, Err(e) => panic!("{label}: {:?}", e.error) };
        let (_, case) = candidate.capture().native_pair().unwrap();
        let k::ExecutionOutcome::Selected(owner) = &case.outcome else { panic!("{label}: selected") };
        let bound = candidate.capture().bind_rows(candidate.envelope(), owner).unwrap();
        let verdicts = candidate.certificate().verdicts();
        assert_eq!(bound.len(), verdicts.len());
        let (mut derived, mut global) = (0, 0);
        for (i, (row, verdict)) in bound.iter().zip(verdicts).enumerate() {
            let item = &ordinary.results[i];
            assert_eq!(row.id, item.id);
            if DERIVED.contains(&item.kind.as_str()) || GLOBAL_END.contains(&item.kind.as_str()) {
                if GLOBAL_END.contains(&item.kind.as_str()) { global += 1; } else { derived += 1; }
                assert_eq!(row.recipe, k::ProductRecipe::NotCovered { row: i as u32 }, "{label}: {}", row.id);
                assert_eq!((verdict.class, verdict.passed, verdict.scale_bits), (None, true, 0), "{label}: {}", row.id);
            } else {
                assert!(!matches!(row.recipe, k::ProductRecipe::NotCovered { .. }), "{label}: {}", row.id);
            }
        }
        assert_eq!(derived, 15 * raw["model"]["pipe_segments"].as_array().unwrap().len());
        assert_eq!(global, 12 * raw["model"]["pipe_segments"].as_array().unwrap().len(), "{label}: W-7 global end rows");
        assert!(candidate.certificate().passed(), "{label}");
    }
}

/// §4.1's divergence control: the owned ordinary envelope's resultants, stresses and derived rows
/// are deliberately changed before the freeze. The successor is byte-identical to the
/// unperturbed one (every value is the retained owner's), its derived rows follow its own
/// retained resultants, and they differ from the formula applied to the perturbed ordinary rows.
#[test]
fn s4_w1l_divergent_ordinary_resultants_are_not_the_derived_operands() {
    let raw = raw();
    for mode in MODES {
        let label = format!("divergent {}", mode.as_str());
        let candidate = |perturb: bool| {
            let (request, capture) = source_receipt::CapturedInvocation::parse(raw.clone(), mode).unwrap();
            let mut prepared = rp::PreparedCase::prepare_observed(request, mode, &capture).unwrap_or_else(|e| panic!("{:?}", e.capture.error));
            let mut perturbed = Vec::new();
            if perturb {
                for row in &mut prepared.ordinary_mut().results {
                    let operand = row.kind.starts_with("element_local_") || DERIVED.contains(&row.kind.as_str());
                    if operand && row.kind != "element_local_torsional_shear_stress" {
                        row.value = 3.0 * row.value + 1.0;
                        perturbed.push(row.id.clone());
                    }
                }
            }
            let ordinary = serde_json::to_value(prepared.ordinary()).unwrap();
            prepared.solve_native().unwrap();
            let candidate = match prepared.project_candidate() { Ok(c) => c, Err(e) => panic!("{label}: {:?}", e.error) };
            (serde_json::to_value(candidate.envelope()).unwrap(), ordinary, perturbed)
        };
        let (reference, _, _) = candidate(false);
        let (successor, ordinary, perturbed) = candidate(true);
        assert!(perturbed.len() > 15, "{label}: the ordinary operands diverge");
        assert_eq!(successor["results"], reference["results"], "{label}: every successor value is the retained owner's");
        derived_rows_follow_own_rows(&label, &successor);
        let rows = ordinary["results"].as_array().unwrap();
        let mut differ = 0;
        for (row, own) in rows.iter().zip(successor["results"].as_array().unwrap()) {
            if row["kind"] != DERIVED[0] {
                continue;
            }
            let n = one(rows, row, "element_local_axial_normal_stress", None)["value"].as_f64().unwrap();
            let h = norm2(one(rows, row, "element_local_bending_normal_stress_y", None)["value"].as_f64().unwrap(),
                one(rows, row, "element_local_bending_normal_stress_z", None)["value"].as_f64().unwrap());
            let from_ordinary = if row["metadata"]["component"] == "sigma_plus" { n + h } else { n - h };
            if own["value"].as_f64().unwrap().to_bits() != from_ordinary.to_bits() {
                differ += 1;
            }
        }
        assert_eq!(differ, rows.iter().filter(|r| r["kind"] == DERIVED[0]).count(), "{label}: no σ row follows the ordinary operands");
    }
}

/// W1-E (the m3x exact successor, both modes): both bases' derived rows are not-covered rows at
/// their own positions (no twin pairing), formed from the successor's own rows of their basis;
/// the base reader recomputes τ from the regenerated evidence's ν and sections.
#[test]
fn s4_w1e_derived_rows_of_both_bases_are_not_covered_and_formed_from_own_rows() {
    let raw = m3x();
    for mode in MODES {
        let label = format!("W1-E {}", mode.as_str());
        let (envelope, retained, captured) = exact_w1(&raw, mode);
        let successor = captured.unwrap_or_else(|| panic!("{label}: {retained:?}"));
        assert!(retained.is_ok(), "{label}: {retained:?}");
        drop(envelope);
        let members = raw["model"]["pipe_segments"].as_array().unwrap().len();
        assert_eq!(derived_rows_follow_own_rows(&label, &successor), 30 * members, "{label}: 15 derived rows per member and basis");
        let reduced = successor["results"].as_array().unwrap().iter()
            .filter(|r| DERIVED.contains(&r["kind"].as_str().unwrap()) && basis_of(r) == Some("reduced_section")).count();
        assert_eq!(reduced, 15 * members, "{label}: each basis's own rows");
        base_read_with_model(&label, &successor, &raw);
        let validation = receipt_and_reader_classify(&label, &successor, &raw, mode);
        assert!(validation.numerical_eligible, "{label}: eligible with the invocation");
    }
}

/// W1-E binds each derived row at its own position whatever the producer's σ order: the S2 order
/// (nominal σ⁺, nominal σ⁻, reduced σ⁺, reduced σ⁻) and the D-5 twin-adjacent order (nominal σ⁺,
/// reduced σ⁺, nominal σ⁻, reduced σ⁻) freeze to the same values, each formed from its own basis's
/// rows; neither order is a twin pairing. The producer publishes the twin-adjacent order (T4-U4b
/// review item 2); the test swaps each station into the other order.
#[test]
fn s4_w1e_derived_rows_bind_in_either_sigma_order() {
    let raw = m3x();
    for mode in MODES {
        let label = format!("W1-E order {}", mode.as_str());
        let run = |adjacent: bool| {
            let (request, capture) = source_receipt::CapturedInvocation::parse(raw.clone(), mode).unwrap();
            let mut prepared = rp::PreparedCase::prepare_observed(request, mode, &capture).unwrap_or_else(|e| panic!("{:?}", e.capture.error));
            if adjacent {
                let rows = &mut prepared.ordinary_mut().results;
                let mut i = 0;
                let mut swapped = 0;
                while i + 3 < rows.len() {
                    let is = |r: &ResultItem, c: &str, b: &str| r.kind == DERIVED[0] && r.metadata.as_ref().is_some_and(|m| m.component == c && m.basis.ends_with(b));
                    let s2 = is(&rows[i + 1], "sigma_minus", "nominal_section") && is(&rows[i + 2], "sigma_plus", "reduced_section");
                    let twin_adjacent = is(&rows[i + 1], "sigma_plus", "reduced_section") && is(&rows[i + 2], "sigma_minus", "nominal_section");
                    if is(&rows[i], "sigma_plus", "nominal_section") && (s2 || twin_adjacent) && is(&rows[i + 3], "sigma_minus", "reduced_section") {
                        rows.swap(i + 1, i + 2);
                        swapped += 1;
                        i += 4;
                    } else {
                        i += 1;
                    }
                }
                assert_eq!(swapped, 5 * raw["model"]["pipe_segments"].as_array().unwrap().len(), "{label}: every station reordered");
            }
            prepared.solve_native().unwrap();
            let candidate = match prepared.project_candidate() { Ok(c) => c, Err(e) => panic!("{label}: {:?}", e.error) };
            assert!(candidate.certificate().passed(), "{label}");
            serde_json::to_value(candidate.envelope()).unwrap()
        };
        let (s2, adjacent) = (run(false), run(true));
        derived_rows_follow_own_rows(&label, &s2);
        derived_rows_follow_own_rows(&label, &adjacent);
        let by_id = |doc: &Value| doc["results"].as_array().unwrap().iter().map(|r| (r["id"].as_str().unwrap().to_owned(), r["value"].as_f64().unwrap().to_bits()))
            .collect::<std::collections::BTreeMap<_, _>>();
        assert_eq!(by_id(&s2), by_id(&adjacent), "{label}: the same values in either order");
    }
}

/// §4.2: a retained-selected mechanics combination publishes no never_combined derived row, and
/// REQ-001's 7n + 50m + 8g rows per combination stay exact, plus (W-7) each member's 12 global end
/// rows, which are signed linear and combined; its operand cases carry theirs.
#[test]
fn s4_combinations_publish_no_derived_rows_and_keep_7n_50m_8g() {
    let raw = w_cb3();
    let n = raw["model"]["nodes"].as_array().unwrap().len();
    let m = raw["model"]["pipe_segments"].as_array().unwrap().len();
    for mode in MODES {
        let label = format!("W-CB3 {}", mode.as_str());
        let (capture, observer, ordinary) = observed(mode, &raw);
        let ((_, retained), _, captured) = hooks::counted_with_successor(|| retained_w1(observer, ordinary, &capture));
        let successor = captured.unwrap_or_else(|| panic!("{label}: {retained:?}"));
        let rows = successor["results"].as_array().unwrap();
        let mut selected = 0;
        for combination in successor["retained_precision"]["body"]["combinations"].as_array().unwrap() {
            if combination["disposition"] != "retained_selected" {
                continue;
            }
            selected += 1;
            let id = combination["basis_ref"]["ref_id"].as_str().unwrap();
            let block: Vec<&Value> = rows.iter().filter(|r| r["basis_ref"]["ref_type"] == "combination" && r["basis_ref"]["ref_id"] == id).collect();
            assert!(block.iter().all(|r| !DERIVED.contains(&r["kind"].as_str().unwrap())), "{label}: {id} publishes no derived row");
            let g = block.iter().filter(|r| r["kind"] == "support_reaction_component_v2").count() / 6;
            assert_eq!(block.len(), 7 * n + (50 + 12) * m + 8 * g, "{label}: {id} has 7n + 50m + 8g rows and 12m global end rows");
            assert_eq!(combination["result_ids"].as_array().unwrap().len(), block.len(), "{label}: {id} owner lists its block");
        }
        assert!(selected > 0, "{label}: a retained-selected combination");
        assert_eq!(derived_rows_follow_own_rows(&label, &successor) % 15, 0, "{label}: the cases carry their derived rows");
        receipt_and_reader_classify(&label, &successor, &raw, mode);
    }
}
