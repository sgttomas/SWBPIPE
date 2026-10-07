//! Shared synthetic statement controls plus listed producer-solved bases (07l); no native
//! Current evidence. These do not establish execution.
use open_pipe_stress_result_export::retained_precision as rp;
use serde_json::Value;
fn corpus() -> Value {
    serde_json::from_str(include_str!(
        "../../../../fixtures/results/retained_precision_cases.json"
    ))
    .unwrap()
}
fn decode(v: &Value) -> f64 {
    f64::from_bits(u64::from_str_radix(v.as_str().unwrap(), 16).unwrap())
}
#[test]
fn fixed_small_bound_vectors() {
    for row in corpus()["arithmetic"]["small_bounds"].as_array().unwrap() {
        assert_eq!(
            rp::absolute_bound(decode(&row["value"]), decode(&row["scale"]))
                .unwrap()
                .to_bits(),
            decode(&row["expected"]).to_bits(),
            "{}",
            row["id"]
        );
    }
}
#[test]
fn finite_product_vectors_and_ranges() {
    for row in corpus()["arithmetic"]["products"].as_array().unwrap() {
        let result = rp::upward_product(decode(&row["a"]), decode(&row["b"]));
        if row["expected"].is_null() {
            assert!(result.is_err());
        } else {
            assert_eq!(
                result.unwrap().to_bits(),
                decode(&row["expected"]).to_bits()
            );
        }
    }
    assert_eq!(
        rp::upward_small_sum(1.0, f64::from_bits(1))
            .unwrap()
            .to_bits(),
        0x3ff0000000000001
    );
    assert_eq!(rp::upward_product(-0.0, 1.0).unwrap().to_bits(), 0);
    for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY, -1.0] {
        assert!(rp::upward_product(value, 1.0).is_err());
    }
}

/// D32 (RV79-N-e) and the 07e format rule (RV78-N1): a rehash or edit-path
/// index is a strict integral value: a JSON number, never a boolean, finite,
/// integral, >= 0 and not -0.
fn index(v: &Value) -> Option<usize> {
    let n = v.as_f64()?;
    (n.is_finite() && n >= 0.0 && n.fract() == 0.0 && !(n == 0.0 && n.is_sign_negative()))
        .then_some(n as usize)
}
/// The 07e format rule (SHARED_SNAPSHOT_07E `format_rule`): 1 preparation
/// hashes, for each `sources[*].preparation.attempt_ref` that resolves and
/// whose members are all prepared; 2 source identities, for each selected
/// case whose `source_ref` resolves; 3 publication hash; 4 receipt hash. A
/// reference that is not an index, or does not resolve, is skipped.
fn rehash(source: &mut Value) {
    use open_pipe_stress_result_export::source_blocks::domain_hash;
    // Snapshot 07 format: an entry that removes retained_precision or its body
    // (a G0 pin) has nothing to rehash.
    if !source
        .get("retained_precision")
        .and_then(|r| r.get("body"))
        .is_some_and(Value::is_object)
    {
        return;
    }
    let b = &mut source["retained_precision"]["body"];
    let attempts = b["product_attempts"].clone();
    for s in b["sources"].as_array_mut().unwrap() {
        if let Some(ai) = index(&s["preparation"]["attempt_ref"]) {
            let a = &attempts[ai];
            // As G1: only an addressable attempt has a preparation digest.
            if !a.is_object() {
                continue;
            }
            if a["preparation"]["members"]
                .as_array()
                .unwrap()
                .iter()
                .all(|m| m["result"]["kind"] == "prepared")
            {
                let members: Vec<_> = a["preparation"]["members"].as_array().unwrap().iter().map(|m| serde_json::json!({"member":m["member"],"old_source":m["old_source"],"old_facts":m["old_facts"],"section":m["result"]["section"]})).collect();
                let payload = serde_json::json!({"definition_id":a["definition_id"],"definition_sha256":rp::DEFINITION_HASH,"owner_ref":a["owner_ref"],"ordinary_attempt_ref":a["ordinary_attempt_ref"],"material_basis_ref":a["material_basis_ref"],"members":members});
                s["preparation"]["sha256"] =
                    domain_hash("retained_precision_preparation_v1", &payload)
                        .unwrap()
                        .into();
            }
        }
    }
    let sources = b["sources"].clone();
    for c in b["cases"].as_array_mut().unwrap() {
        if c["status"] == "selected" {
            // Only a resolving source has an identity digest to recompute.
            let Some(mut s) = index(&c["source_ref"])
                .and_then(|i| sources.get(i))
                .filter(|s| s.is_object())
                .cloned()
            else {
                continue;
            };
            s.as_object_mut().unwrap().remove("index");
            c["source_identity_sha256"] = domain_hash("retained_precision_source_mp_v2", &s)
                .unwrap()
                .into();
        }
    }
    let mut public = source.clone();
    public.as_object_mut().unwrap().remove("retained_precision");
    source["retained_precision"]["body"]["publication_sha256"] =
        domain_hash("retained_precision_publication_mp_v2", &public)
            .unwrap()
            .into();
    source["retained_precision"]["receipt_sha256"] = domain_hash(
        "retained_precision_receipt_mp_v2",
        &source["retained_precision"]["body"],
    )
    .unwrap()
    .into();
}
fn edit(source: &mut Value, e: &Value) {
    let path = e["path"].as_array().unwrap();
    let mut parent = source;
    for p in &path[..path.len() - 1] {
        parent = if let Some(i) = index(p) {
            &mut parent[i]
        } else {
            &mut parent[p.as_str().unwrap()]
        };
    }
    let last = path.last().unwrap();
    if e["op"] == "remove" {
        if let Some(i) = index(last) {
            parent.as_array_mut().unwrap().remove(i);
        } else {
            parent
                .as_object_mut()
                .unwrap()
                .remove(last.as_str().unwrap());
        }
    } else if let Some(i) = index(last) {
        parent[i] = e["value"].clone();
    } else {
        parent[last.as_str().unwrap()] = e["value"].clone();
    }
}
/// The reader's own expectation: `expected_by_reader.rust` when present (the
/// per-language G7 base code), else the shared `expected` (snapshot 06b format).
fn expected_for(m: &Value) -> &Value {
    m.get("expected_by_reader")
        .and_then(|e| e.get("rust"))
        .unwrap_or(&m["expected"])
}
/// Apply a shared mutation or must-pass entry exactly as SHARED_SNAPSHOT_06C
/// `format_change` specifies: edit a copy of the base source; edit a copy of
/// the base invocation; when invocation edits exist, bind the receipt's
/// invocation digest to the edited invocation; then rehash per `rehash`.
/// Returns the edited source and the invocation to validate against.
fn apply_entry(shared: &Value, entry: &Value) -> (Value, Value) {
    use open_pipe_stress_result_export::source_blocks::domain_hash;
    let case = shared["cases"]
        .as_array()
        .unwrap()
        .iter()
        .find(|c| c["id"] == entry["base"])
        .unwrap();
    let mut source = case["source"].clone();
    for e in entry["edits"].as_array().unwrap() {
        edit(&mut source, e);
    }
    let mut invocation = case["invocation"].clone();
    let invocation_edits = entry["invocation_edits"].as_array().cloned().unwrap_or_default();
    for e in &invocation_edits {
        edit(&mut invocation, e);
    }
    if !invocation_edits.is_empty() {
        source["retained_precision"]["body"]["invocation"]["value"] =
            domain_hash("source_blocks_invocation_v1", &invocation)
                .unwrap()
                .into();
    }
    // The shared format admits only rehash:"all" (D11).
    assert_eq!(entry["rehash"], "all", "{}", entry["id"]);
    rehash(&mut source);
    // D24: the optional `after_rehash` edit list is applied after the rehash.
    for e in entry["after_rehash"].as_array().cloned().unwrap_or_default() {
        edit(&mut source, &e);
    }
    (source, invocation)
}
#[test]
fn complete_synthetic_controls_carry_their_shared_eligibility() {
    let shared = corpus();
    for case in shared["cases"].as_array().unwrap() {
        let got = rp::validate(&case["source"], Some(&case["invocation"]))
            .unwrap_or_else(|e| panic!("{}: {e:?}", case["id"]));
        assert!(got.invocation_bound);
        // U7 (07i, D-U7-2): 15 bases are eligible (07l: 13 synthetic plus the two
        // producer-solved L = 0 bases); the two with an unavailable case are not.
        assert_eq!(
            got.numerical_eligible,
            case["expected"]["numerical_eligible"].as_bool().unwrap(),
            "{}: the shared eligibility",
            case["id"]
        );
        assert_eq!(
            got.publication_sha256,
            case["source"]["retained_precision"]["body"]["publication_sha256"]
        );
        let expected = case["expected_classifications"].as_array().unwrap();
        assert_eq!(got.classifications.len(), expected.len());
        for (got, want) in got.classifications.iter().zip(expected) {
            assert_eq!(got.result_id, want["result_id"]);
            assert_eq!(got.basis_ref, want["basis_ref"]);
            assert_eq!(
                format!("{:016x}", got.normalized_bits),
                want["normalized_bits"]
            );
            assert_eq!(
                got.scale_bits.map(|b| format!("{b:016x}")),
                want["scale_bits"].as_str().map(str::to_owned)
            );
            let name = match got.class {
                rp::AccuracyClass::RelativeVerified => "relative_verified",
                rp::AccuracyClass::AbsoluteVerified { bound_bits } => {
                    assert_eq!(format!("{bound_bits:016x}"), want["bound_bits"]);
                    "absolute_verified"
                }
                rp::AccuracyClass::InputDerived => "input_derived",
                rp::AccuracyClass::NonQuantity => "non_quantity",
                rp::AccuracyClass::NotCovered => "not_covered",
            };
            assert_eq!(name, want["class"]);
        }
        let unbound = rp::validate(&case["source"], None).unwrap();
        assert!(!unbound.invocation_bound && !unbound.numerical_eligible);
        let transport = rp::validate_transport_metadata(&case["source"]).unwrap();
        assert!(
            !transport.invocation_bound
                && !transport.numerical_eligible
                && transport.classifications.is_empty()
        );
    }
}
#[test]
fn shared_rehashed_first_failure_mutations() {
    let shared = corpus();
    let mut failures = Vec::new();
    for mutation in shared["mutations"].as_array().unwrap() {
        let (source, invocation) = apply_entry(&shared, mutation);
        let expected = expected_for(mutation);
        match rp::validate(&source, Some(&invocation)) {
            Err(e) if e.gate == expected["gate"] && e.code == expected["code"] => {}
            Err(got) => failures.push(format!(
                "{} expected {} got {got:?}",
                mutation["id"], expected
            )),
            Ok(got) => failures.push(format!(
                "{} expected {} got admitted statement (eligible={})",
                mutation["id"], expected, got.numerical_eligible
            )),
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

/// Observe one slice of the shared mutations against this reader's own
/// expectation, print one outcome per mutation (visible with --nocapture) and
/// check the slice tally. 07h held 277 mutations; RV94 N-3's G7 probe makes 278;
/// 07l (U8-2) appends the 8 L = 0 mutations, making 286; 07m (B6) appends 8 G7
/// mutations, making 294.
fn slice_outcomes(tag: &str, range: std::ops::Range<usize>, want: &[(&str, usize)]) {
    use std::collections::BTreeMap;
    let shared = corpus();
    let mutations = shared["mutations"].as_array().unwrap();
    assert_eq!(mutations.len(), 294);
    let mut tally = BTreeMap::new();
    let mut matched = 0;
    for mutation in &mutations[range.clone()] {
        let observed = observe(&shared, mutation);
        let expected = expected_for(mutation);
        let ok = observed == *expected;
        matched += usize::from(ok);
        *tally
            .entry(format!(
                "{} {}",
                expected["gate"].as_str().unwrap(),
                expected["code"].as_str().unwrap()
            ))
            .or_insert(0) += 1;
        println!(
            "{tag} {}",
            serde_json::json!({"id":mutation["id"],"base":mutation["base"],"expected":expected,"observed":observed,"match":ok})
        );
    }
    let want: BTreeMap<String, usize> = want.iter().map(|(k, n)| (k.to_string(), *n)).collect();
    assert_eq!(tally, want, "{tag}");
    assert_eq!(matched, range.len(), "{tag}");
}

/// Snapshot-03 controls (the first 30 shared mutations), so the outcome
/// listing covers all 178 (D15, RV78-N9).
#[test]
fn snapshot_03_mutation_outcomes() {
    slice_outcomes(
        "I63_OUTCOME_03",
        0..30,
        &[
            ("G0 SOURCE_PRODUCER_CONTRACT_UNSUPPORTED", 1),
            ("G1 RETAINED_PRECISION_RECEIPT_MISMATCH", 6),
            ("G2 RETAINED_PRECISION_ENCODING_MISMATCH", 3),
            ("G3 RETAINED_PRECISION_COVERAGE_MISMATCH", 2),
            ("G4 RETAINED_PRECISION_DIAGNOSTIC_MISMATCH", 1),
            ("G5 RETAINED_PRECISION_ATTEMPT_MISMATCH", 1),
            ("G5 RETAINED_PRECISION_WORK_MISMATCH", 4),
            ("G5b RETAINED_PRECISION_SCALE_MISMATCH", 1),
            ("G5b RETAINED_PRECISION_SECTION_MISMATCH", 1),
            ("G5c RETAINED_PRECISION_CLASSIFICATION_MISMATCH", 1),
            ("G5c RETAINED_PRECISION_INPUT_DOF_MISMATCH", 1),
            ("G6 RETAINED_PRECISION_ROW_METHOD_MISMATCH", 3),
            ("G8 RETAINED_PRECISION_INVOCATION_MISMATCH", 1),
            ("G8 RETAINED_PRECISION_PREPARATION_MISMATCH", 4),
        ],
    );
}

/// Snapshot-04 summary-coverage controls (I57 s4/s5), mutations 30..77.
#[test]
fn snapshot_04_coverage_mutation_outcomes() {
    slice_outcomes(
        "I63_OUTCOME",
        30..77,
        &[
            ("G1 RETAINED_PRECISION_RECEIPT_MISMATCH", 14),
            ("G2 RETAINED_PRECISION_ENCODING_MISMATCH", 4),
            ("G3 RETAINED_PRECISION_COVERAGE_MISMATCH", 6),
            ("G5 RETAINED_PRECISION_PRODUCT_ATTEMPT_MISMATCH", 2),
            ("G5 RETAINED_PRECISION_ATTEMPT_MISMATCH", 1),
            ("G5a RETAINED_PRECISION_SCALE_MISMATCH", 20),
        ],
    );
}

/// Snapshot-05a controls (I62 C1a), mutations 77..104.
#[test]
fn snapshot_05a_mutation_outcomes() {
    slice_outcomes(
        "I63_OUTCOME_05A",
        77..104,
        &[
            ("G3 RETAINED_PRECISION_COVERAGE_MISMATCH", 3),
            ("G5 RETAINED_PRECISION_ATTEMPT_MISMATCH", 1),
            ("G5 RETAINED_PRECISION_PRODUCT_ATTEMPT_MISMATCH", 5),
            ("G5 RETAINED_PRECISION_WORK_MISMATCH", 2),
            ("G5a RETAINED_PRECISION_SCALE_MISMATCH", 16),
        ],
    );
}

/// Snapshot-05b controls (I62 C1b, unchanged in 05c), mutations 104..121.
#[test]
fn snapshot_05b_mutation_outcomes() {
    slice_outcomes(
        "I63_OUTCOME_05B",
        104..121,
        &[
            ("G3 RETAINED_PRECISION_COVERAGE_MISMATCH", 2),
            ("G5 RETAINED_PRECISION_PRODUCT_ATTEMPT_MISMATCH", 1),
            ("G5a RETAINED_PRECISION_SCALE_MISMATCH", 13),
            ("G5b RETAINED_PRECISION_SCALE_MISMATCH", 1),
        ],
    );
}

/// Snapshot-06a checklist controls (I62 C2-1), mutations 121..150; 06b moved
/// `prefix_old_inputs_unbound` to the must-pass entries (P7 settlement), and
/// the G7 entry uses this reader's own base code (`expected_by_reader`).
#[test]
fn snapshot_06a_mutation_outcomes() {
    slice_outcomes(
        "I63_OUTCOME_06A",
        121..150,
        &[
            ("G2 RETAINED_PRECISION_ENCODING_MISMATCH", 1),
            ("G3 RETAINED_PRECISION_COVERAGE_MISMATCH", 3),
            ("G5 RETAINED_PRECISION_ATTEMPT_MISMATCH", 10),
            ("G5 RETAINED_PRECISION_PRODUCT_ATTEMPT_MISMATCH", 10),
            ("G5 RETAINED_PRECISION_WORK_MISMATCH", 2),
            ("G7 SOURCE_PREVIEW_PHYSICS_EXTREMA_BOUNDS", 1),
            ("G8 RETAINED_PRECISION_PREPARATION_MISMATCH", 2),
        ],
    );
}

/// Snapshot-06b settlements (I62 C2-2), mutations 150..163, with
/// `ceiling_before_last_slot` under its 06c name.
#[test]
fn snapshot_06b_mutation_outcomes() {
    slice_outcomes(
        "I63_OUTCOME_06B",
        150..163,
        &[
            ("G1 RETAINED_PRECISION_RECEIPT_MISMATCH", 1),
            ("G5 RETAINED_PRECISION_ATTEMPT_MISMATCH", 8),
            ("G5 RETAINED_PRECISION_PRODUCT_ATTEMPT_MISMATCH", 1),
            ("G8 RETAINED_PRECISION_PREPARATION_MISMATCH", 3),
        ],
    );
}

/// Snapshot-06c (I62 C2-3), mutations 163..173: WorkAccounting terminals
/// rejected, and the invocation-level G8 refusals through `invocation_edits`.
#[test]
fn snapshot_06c_mutation_outcomes() {
    slice_outcomes(
        "I63_OUTCOME_06C",
        163..173,
        &[
            ("G5 RETAINED_PRECISION_ATTEMPT_MISMATCH", 3),
            ("G8 RETAINED_PRECISION_PREPARATION_MISMATCH", 7),
        ],
    );
}


/// Snapshot-06d (I62 C2-5), mutations 173..178: R1-R3 at G5 WORK, and the
/// tightened idle sibling at the exhaustion rule.
#[test]
fn snapshot_06d_mutation_outcomes() {
    slice_outcomes(
        "I63_OUTCOME_06D",
        173..178,
        &[
            ("G5 RETAINED_PRECISION_ATTEMPT_MISMATCH", 1),
            ("G5 RETAINED_PRECISION_WORK_MISMATCH", 4),
        ],
    );
}

/// Snapshot-07/07a review-repair pins (I62 B2; D18 in 07a), mutations 178..236.
#[test]
fn snapshot_07_mutation_outcomes() {
    slice_outcomes(
        "I63_OUTCOME_07",
        178..236,
        &[
            ("G0 SOURCE_PRODUCER_CONTRACT_UNSUPPORTED", 6),
            ("G1 RETAINED_PRECISION_RECEIPT_MISMATCH", 4),
            ("G3 RETAINED_PRECISION_COVERAGE_MISMATCH", 7),
            ("G4 RETAINED_PRECISION_DIAGNOSTIC_MISMATCH", 1),
            ("G5 RETAINED_PRECISION_ATTEMPT_MISMATCH", 16),
            ("G5 RETAINED_PRECISION_PRODUCT_ATTEMPT_MISMATCH", 11),
            ("G5 RETAINED_PRECISION_WORK_MISMATCH", 7),
            ("G5b RETAINED_PRECISION_SECTION_MISMATCH", 2),
            ("G8 RETAINED_PRECISION_PREPARATION_MISMATCH", 4),
        ],
    );
}

/// Snapshot-07b/07c confirmation-repair pins (I62; D19-D30, and 07c's D21
/// verification-summary pin), mutations 236..254.
#[test]
fn snapshot_07b_mutation_outcomes() {
    slice_outcomes(
        "I63_OUTCOME_07B",
        236..254,
        &[
            ("G1 RETAINED_PRECISION_RECEIPT_MISMATCH", 4),
            ("G3 RETAINED_PRECISION_COVERAGE_MISMATCH", 2),
            ("G5 RETAINED_PRECISION_ATTEMPT_MISMATCH", 5),
            ("G5 RETAINED_PRECISION_PRODUCT_ATTEMPT_MISMATCH", 5),
            ("G5 RETAINED_PRECISION_WORK_MISMATCH", 1),
            ("G5a RETAINED_PRECISION_SCALE_MISMATCH", 1),
        ],
    );
}

/// Snapshot-07d repair pins (I62; D31, D32, D33 and RV78-N1's two D19 Ready
/// negatives), mutations 254..259.
#[test]
fn snapshot_07d_mutation_outcomes() {
    slice_outcomes(
        "I63_OUTCOME_07D",
        254..259,
        &[
            ("G1 RETAINED_PRECISION_RECEIPT_MISMATCH", 1),
            ("G5 RETAINED_PRECISION_ATTEMPT_MISMATCH", 1),
            ("G5 RETAINED_PRECISION_PRODUCT_ATTEMPT_MISMATCH", 2),
            ("G8 RETAINED_PRECISION_INVOCATION_MISMATCH", 1),
        ],
    );
}

/// Snapshot-07e pins (I62; RV79-S1's three non-integral G0 values and
/// RV81-N1's D33 rotation row), mutations 259..263.
#[test]
fn snapshot_07e_mutation_outcomes() {
    slice_outcomes(
        "I63_OUTCOME_07E",
        259..263,
        &[
            ("G0 SOURCE_PRODUCER_CONTRACT_UNSUPPORTED", 3),
            ("G5 RETAINED_PRECISION_ATTEMPT_MISMATCH", 1),
        ],
    );
}

/// Snapshot-07f pins (I62; D37, RV79's five X1 probes on F', which include
/// RV78's Y1, Y2 and Y4), mutations 263..268.
#[test]
fn snapshot_07f_mutation_outcomes() {
    slice_outcomes(
        "I63_OUTCOME_07F",
        263..268,
        &[("G5 RETAINED_PRECISION_PRODUCT_ATTEMPT_MISMATCH", 5)],
    );
}

/// Snapshot-07g pins (I61 U6e; F5, D-U6-7): A2's exact ordinary list, the
/// omission, order, RETAINED_PRECISION_* (U1 M09), invocation-level (U1 M10),
/// another case and 07f's relaxed form, mutations 268..274.
#[test]
fn snapshot_07g_mutation_outcomes() {
    slice_outcomes(
        "I61_OUTCOME_07G",
        268..274,
        &[("G5 RETAINED_PRECISION_ATTEMPT_MISMATCH", 6)],
    );
}

/// Snapshot-07h pins (I61 U6e repair; RV90 S1, N1, N2): a non-array
/// `affected_refs` names no case, F5 on the second case, and a strict prefix
/// of A2's list, mutations 274..277.
#[test]
fn snapshot_07h_mutation_outcomes() {
    slice_outcomes(
        "I61_OUTCOME_07H",
        274..277,
        &[("G5 RETAINED_PRECISION_ATTEMPT_MISMATCH", 3)],
    );
}

/// Snapshot-07l pins (U8-2; I69) on the two producer-solved L = 0 bases:
/// SNAPSHOT_05_PLAN §1.2's isolated rotation stop, has_data and coupled estimate
/// (sparse, then dense), then I69's translation-plus-rotation stop (sparse, then
/// dense), mutations 278..286.
#[test]
fn snapshot_07l_mutation_outcomes() {
    slice_outcomes(
        "I70_OUTCOME_07L",
        278..286,
        &[("G5a RETAINED_PRECISION_SCALE_MISMATCH", 8)],
    );
}

/// Mutation 277, RV94 N-3's G7 probe (07k), as its own one-entry slice (B6;
/// I70's item 2): an invalid enum in a not_required case's quality, at G7
/// with the base code, which TS now shares (PLAN decision 11).
#[test]
fn snapshot_07k_mutation_outcomes() {
    // The slice's entry, by id, as Python's and TS's slices check (a reorder
    // of same-code entries leaves the tally unchanged).
    assert_eq!(corpus()["mutations"][277]["id"], "g7_not_required_quality_enum_invalid");
    slice_outcomes(
        "I83_OUTCOME_07K",
        277..278,
        &[("G7 SOURCE_NUMERICAL_CASE_INVALID", 1)],
    );
}

/// Snapshot-07m pins (B6): the N-3 class at its full width (a selected and an
/// unavailable case's quality enum, an empty evidence ref, an extra case
/// member), then the sibling base header classes (quality status, empty
/// limitations, null contract_evidence, a source_block_recovery member),
/// mutations 286..294, each at G7 with this reader's base header code.
#[test]
fn snapshot_07m_mutation_outcomes() {
    let shared = corpus();
    let ids: Vec<&str> = shared["mutations"].as_array().unwrap()[286..]
        .iter()
        .map(|m| m["id"].as_str().unwrap())
        .collect();
    assert_eq!(
        ids,
        [
            "g7_selected_quality_enum_invalid",
            "g7_unavailable_quality_enum_invalid",
            "g7_quality_case_evidence_ref_empty",
            "g7_quality_case_extra_member",
            "g7_quality_status_invalid",
            "g7_formulation_limitations_empty",
            "g7_contract_evidence_null",
            "g7_source_block_recovery_present",
        ]
    );
    slice_outcomes(
        "I83_OUTCOME_07M",
        286..294,
        &[
            ("G7 SOURCE_NUMERICAL_CASE_INVALID", 4),
            ("G7 SOURCE_NUMERICAL_QUALITY_INVALID", 1),
            ("G7 SOURCE_FORMULATION_BASIS_UNSUPPORTED", 1),
            ("G7 SOURCE_PREVIEW_PHYSICS_EVIDENCE_REQUIRED", 1),
            ("G7 SOURCE_BLOCKS_LEGACY_DOWNGRADE_FORBIDDEN", 1),
        ],
    );
}

/// F5 (D-U6-7; A2) on the real milestone receipts: U1's producer mutants M09
/// (RETAINED_PRECISION_* listed), M10 (diagnostics that do not name the case
/// listed) and M20 (another row method token) are refused by this reader, each
/// resealed as a corpus entry would be.
#[test]
fn f5_kills_u1_m09_m10_m20_on_the_real_milestone_receipts() {
    use serde_json::json;
    for (mode, text) in [
        ("sparse_interactive", include_str!("../../../../fixtures/results/retained_precision_milestone_successor_sparse_interactive.json")),
        ("dense_scrutiny", include_str!("../../../../fixtures/results/retained_precision_milestone_successor_dense_scrutiny.json")),
    ] {
        let doc: Value = serde_json::from_str(text).unwrap();
        let (source, invocation) = (&doc["source"], &doc["invocation"]);
        let case = source["retained_precision"]["body"]["cases"][0]["basis_ref"]["ref_id"].clone();
        let names = |d: &Value| d["affected_refs"].as_array().is_some_and(|a| a.contains(&case));
        let retained = |d: &Value| d["code"].as_str().unwrap().starts_with("RETAINED_PRECISION_");
        let ds = source["diagnostics"].as_array().unwrap();
        let exact: Vec<Value> = ds.iter().filter(|d| names(d) && !retained(d)).map(|d| d["id"].clone()).collect();
        assert_eq!(source["retained_precision"]["body"]["ordinary_attempts"][0]["diagnostic_refs"], json!(exact), "{mode}");
        let m09: Vec<Value> = ds.iter().filter(|d| names(d)).map(|d| d["id"].clone()).collect();
        let m10: Vec<Value> = ds.iter().filter(|d| !retained(d)).map(|d| d["id"].clone()).collect();
        assert!(m09 != exact && m10 != exact, "{mode}: the mutants differ from the exact list");
        let refs = json!(["retained_precision", "body", "ordinary_attempts", 0, "diagnostic_refs"]);
        for (edit_, want) in [
            (set(refs.clone(), json!(m09)), ("G5", ATTEMPT)),
            (set(refs.clone(), json!(m10)), ("G5", ATTEMPT)),
            (set(json!(["results", 0, "recovery_method"]), json!("other")), ("G6", "RETAINED_PRECISION_ROW_METHOD_MISMATCH")),
        ] {
            let mut edited = source.clone();
            edit(&mut edited, &edit_);
            rehash(&mut edited);
            let got = rp::validate(&edited, Some(invocation)).err().map(|e| (e.gate, e.code));
            assert_eq!(got.as_ref().map(|(g, c)| (*g, c.as_str())), Some(want), "{mode} {edit_}");
        }
        let mut resealed = source.clone();
        rehash(&mut resealed);
        assert_eq!(resealed, *source, "{mode}: the receipt reseals to itself");
    }
}

fn observe(shared: &Value, mutation: &Value) -> Value {
    let (source, invocation) = apply_entry(shared, mutation);
    match rp::validate(&source, Some(&invocation)) {
        Err(e) => serde_json::json!({"gate":e.gate,"code":e.code}),
        Ok(_) => serde_json::json!(null),
    }
}




fn attempt_mismatch(got: Result<(), rp::ValidationError>) -> bool {
    matches!(got, Err(e) if e.gate == "G5" && e.code == "RETAINED_PRECISION_ATTEMPT_MISMATCH")
}

/// Rust reader-logic controls mirroring I62's Python-only tests for checklist
/// N5, N6, N8 and N10, which have no native-faithful shared base yet (the
/// Ceiling, an idle/pre-schedule Run, a verification-pass terminal). They run
/// the same schedule replay `validate` uses, on one Run.
#[test]
fn schedule_replay_terminal_branches_reader_logic() {
    use serde_json::json;
    let shared = corpus();
    let schedule = rp::reader_logic::schedule;
    let run_of = |id: &str| {
        shared["cases"]
            .as_array()
            .unwrap()
            .iter()
            .find(|c| c["id"] == id)
            .unwrap()["source"]["retained_precision"]["body"]["cases"][0]["run"]
            .clone()
    };
    let selected = run_of("ordinary_prepared_synthetic");
    assert!(schedule(&selected).is_ok());
    let ceiling = json!({"kind":"unresolved","reason":{"space":"unresolved","tag":"ceiling"}});
    // N10: an idle (pre-schedule) Run has no records, no charge and a reasoned
    // non-selected terminal.
    let mut idle = selected.clone();
    idle["records"] = json!([]);
    idle["attempts"] = json!([]);
    idle["case_charge"] = json!(0);
    idle["invocation_increment"] = json!(0);
    idle["kernel_terminal"] =
        json!({"kind":"unresolved","reason":{"space":"unresolved","tag":"budget","scope":"invocation"}});
    assert!(schedule(&idle).is_ok());
    let mut bad = idle.clone();
    bad["kernel_terminal"] = json!({"kind":"selected","reason":null});
    assert!(attempt_mismatch(schedule(&bad)), "idle Run cannot select");
    let mut bad = idle.clone();
    bad["case_charge"] = json!(1);
    assert!(attempt_mismatch(schedule(&bad)), "idle Run carries no charge");
    // 06b/06c ruling: no WorkAccounting terminal is emitted, idle or not.
    let mut bad = idle.clone();
    bad["kernel_terminal"] = json!({"kind":"unresolved","reason":{"space":"unresolved","tag":"work_accounting","fault":"overflow"}});
    assert!(attempt_mismatch(schedule(&bad)), "idle WorkAccounting is not emitted");
    // N6: a rejected p128 candidate must hand its verification to a reused p256.
    let reason = json!({"space":"attempt","tag":"stop_rule","quantity":{"tag":"displacement","dof":{"node":1,"component":"UX"}},"body":0,"kind":"translation"});
    let mut rejected = selected.clone();
    rejected["attempts"][0]["outcome"] = json!({"kind":"rejected","reason":reason});
    rejected["records"][0]["outcome"] = json!({"kind":"rejected","reason":reason});
    rejected["records"][1]["outcome"] = json!({"kind":"solved"});
    rejected["kernel_terminal"] = ceiling.clone();
    assert!(attempt_mismatch(schedule(&rejected)), "rejected must continue");
    // N5: a non-escalating verification-pass failure is terminal.
    let stop = json!({"space":"attempt","tag":"stop","stop":{"space":"stop","tag":"structure"}});
    let mut vfail = selected.clone();
    let verification_failed = json!({"kind":"rejected","reason":{"space":"attempt","tag":"verification_failed"}});
    vfail["attempts"][0]["outcome"] = verification_failed.clone();
    vfail["records"][0]["outcome"] = verification_failed;
    vfail["attempts"][0]["verification"] =
        json!({"record":1,"precision":256,"phase":"failed","reason":stop});
    vfail["records"][1]["outcome"] = json!({"kind":"failed","reason":stop});
    vfail["kernel_terminal"] = json!({"kind":"refused","reason":{"space":"refusal","tag":"structure"}});
    assert!(schedule(&vfail).is_ok());
    let mut bad = vfail.clone();
    bad["kernel_terminal"] = json!({"kind":"selected","reason":null});
    assert!(attempt_mismatch(schedule(&bad)), "verification-pass failure cannot select");
    // N8: the Ceiling, when the reused p512 candidate is rejected and its p1024
    // verification only solved.
    let mut ladder = run_of("p512_ladder_synthetic");
    assert!(schedule(&ladder).is_ok());
    ladder["attempts"][2]["outcome"] = json!({"kind":"rejected","reason":reason});
    ladder["records"][2]["outcome"] = json!({"kind":"rejected","reason":reason});
    ladder["records"][3]["outcome"] = json!({"kind":"solved"});
    ladder["kernel_terminal"] = ceiling;
    assert!(schedule(&ladder).is_ok());
    let mut bad = ladder.clone();
    bad["kernel_terminal"] = json!({"kind":"refused","reason":{"space":"refusal","tag":"structure"}});
    assert!(attempt_mismatch(schedule(&bad)), "exhausted ladder is the Ceiling");
}

/// 06c WorkAccounting rejections fail in the schedule replay itself (the
/// emitted-terminal rule), not only through a later check on the same code:
/// the replay alone, without the statement, rejects each WorkAccounting Run.
#[test]
fn work_accounting_mutations_fail_at_the_terminal_rule() {
    let shared = corpus();
    for id in [
        "work_accounting_after_escalating_stop",
        "idle_work_accounting_run",
        "work_accounting_at_last_slot",
    ] {
        let mutation = shared["mutations"]
            .as_array()
            .unwrap()
            .iter()
            .find(|m| m["id"] == id)
            .unwrap();
        let (source, _) = apply_entry(&shared, mutation);
        let runs: Vec<&Value> = source["retained_precision"]["body"]["cases"]
            .as_array()
            .unwrap()
            .iter()
            .map(|c| &c["run"])
            .filter(|r| r["kernel_terminal"]["reason"]["tag"] == "work_accounting")
            .collect();
        assert_eq!(runs.len(), 1, "{id}");
        assert!(attempt_mismatch(rp::reader_logic::schedule(runs[0])), "{id}");
    }
}

/// Rust reader-logic control mirroring I62's Python-only test for checklist O5
/// (no native-faithful source_decline base yet).
#[test]
fn source_decline_relation_reader_logic() {
    use serde_json::json;
    let shared = corpus();
    let fixture = shared["cases"]
        .as_array()
        .unwrap()
        .iter()
        .find(|c| c["id"] == "two_case_preparation_failure_synthetic")
        .unwrap();
    let mut source = fixture["source"].clone();
    let decline = json!({
        "input_owner": {"case_index": 1, "case_id": "case:unavailable-row", "material_basis_ref": 0},
        "constructor_counts": {"nodes": 2, "members": 1, "springs": 0, "constraints": 6, "nodal_terms": 6, "stations": 3, "supports": 1, "id_utf8_bytes": 0, "directional_springs": 0},
        "error": {"tag": "no_nodes"}
    });
    source["retained_precision"]["body"]["cases"][1]["source_decline"] = decline.clone();
    assert!(rp::reader_logic::ordinary(&source).is_ok());
    let mut wrong = decline;
    wrong["input_owner"]["case_index"] = json!(0);
    source["retained_precision"]["body"]["cases"][1]["source_decline"] = wrong;
    assert!(attempt_mismatch(rp::reader_logic::ordinary(&source)));
}

/// Reader-local controls (not shared corpus) for checklist checks added in the
/// I63 audit that no shared mutation decides first: P2 (observables and G5a
/// enter together), P6 (a failed maxima merges its work) and P9 (an
/// unavailable error matches the first failed stage), each on a shared
/// must-pass base; and the G7 bare code with the Rust base code as detail.
#[test]
fn g5_audit_local_controls() {
    use serde_json::json;
    let shared = corpus();
    let entry = |id: &str| {
        shared["must_pass"]
            .as_array()
            .unwrap()
            .iter()
            .find(|m| m["id"] == id)
            .unwrap()
            .clone()
    };
    let attempt = json!(["retained_precision", "body", "product_attempts", 1]);
    let at = |tail: Value| {
        let mut p = attempt.as_array().unwrap().clone();
        p.extend(tail.as_array().unwrap().iter().cloned());
        Value::Array(p)
    };
    for (name, base, extra) in [
        (
            "P9: maxima failure reported as a proof error",
            "maxima_abandoned",
            vec![(
                at(json!(["result"])),
                json!({"kind":"unavailable","error":{"kind":"proof","cause":{"kind":"work_accounting","fault":"overflow"}}}),
            )],
        ),
        (
            "P6: failed maxima without merged completion",
            "maxima_abandoned",
            vec![(at(json!(["proof", "completion"])), json!({"kind":"not_entered"}))],
        ),
        (
            "P2: observables entered without G5a",
            "cert_failed_before_summary",
            vec![
                (at(json!(["stages", "observables"])), json!("failed")),
                (
                    at(json!(["proof", "checks", "observables"])),
                    json!({"kind":"failed","error":{"kind":"observable","cause":{"kind":"accounting","event":"map_write"}}}),
                ),
            ],
        ),
    ] {
        let mut mutation = entry(base);
        for (path, value) in extra {
            mutation["edits"]
                .as_array_mut()
                .unwrap()
                .push(json!({"path":path,"op":"set","value":value}));
        }
        mutation["expected"] = json!(null);
        assert_eq!(
            observe(&shared, &mutation),
            json!({"gate":"G5","code":"RETAINED_PRECISION_PRODUCT_ATTEMPT_MISMATCH"}),
            "{name}"
        );
    }
    // G7: the bare base code is the error code.
    let g7 = shared["mutations"]
        .as_array()
        .unwrap()
        .iter()
        .find(|m| m["id"] == "g7_maximum_off_enclosure")
        .unwrap();
    let case = shared["cases"]
        .as_array()
        .unwrap()
        .iter()
        .find(|c| c["id"] == g7["base"])
        .unwrap();
    let mut source = case["source"].clone();
    for e in g7["edits"].as_array().unwrap() {
        edit(&mut source, e);
    }
    rehash(&mut source);
    let got = rp::validate(&source, Some(&case["invocation"])).unwrap_err();
    // G7 settlement (06b): this reader's own bare base code, detail separate.
    assert_eq!(
        (got.gate, got.code.as_str()),
        ("G7", "SOURCE_PREVIEW_PHYSICS_EXTREMA_BOUNDS")
    );
}

/// Snapshot-05a shared must-pass entries: each rehashed rewrite keeps every
/// public relation, so the reader admits it with the base case's
/// classifications and the eligibility the entry states (07i, U7).
#[test]
fn shared_must_pass_entries_validate() {
    let shared = corpus();
    let entries = shared["must_pass"].as_array().unwrap();
    // Snapshot 07: 06d's 18 plus the equal-E bracket control; 07h adds F5's
    // reordered-envelope exact-list control (RV90 N2); 07j adds C04's not_required case;
    // 07l (U8-2) appends the 4 L = 0 entries on the producer-solved bases, 24 to 28.
    assert_eq!(entries.len(), 28);
    let mut failures = Vec::new();
    for entry in entries {
        assert_eq!(entry["expected"], "pass");
        assert_eq!(entry["rehash"], "all");
        let case = shared["cases"]
            .as_array()
            .unwrap()
            .iter()
            .find(|c| c["id"] == entry["base"])
            .unwrap();
        let (source, invocation) = apply_entry(&shared, entry);
        let got = match rp::validate(&source, Some(&invocation)) {
            Ok(got) => got,
            Err(e) => {
                failures.push(format!("{} rejected {e:?}", entry["id"]));
                continue;
            }
        };
        let expected = case["expected_classifications"].as_array().unwrap();
        // U7 (07i): each must-pass entry states its eligibility.
        let same = got.numerical_eligible
            == entry["expected_eligibility"]["numerical_eligible"].as_bool().unwrap()
            && got.invocation_bound
            && got.classifications.len() == expected.len()
            && got.classifications.iter().zip(expected).all(|(g, w)| {
                let class = match g.class {
                    rp::AccuracyClass::RelativeVerified => "relative_verified",
                    rp::AccuracyClass::AbsoluteVerified { bound_bits } => {
                        if format!("{bound_bits:016x}") != w["bound_bits"] {
                            return false;
                        }
                        "absolute_verified"
                    }
                    rp::AccuracyClass::InputDerived => "input_derived",
                    rp::AccuracyClass::NonQuantity => "non_quantity",
                    rp::AccuracyClass::NotCovered => "not_covered",
                };
                g.result_id == w["result_id"]
                    && g.basis_ref == w["basis_ref"]
                    && format!("{:016x}", g.normalized_bits) == w["normalized_bits"]
                    && g.scale_bits.map(|b| format!("{b:016x}"))
                        == w["scale_bits"].as_str().map(str::to_owned)
                    && class == w["class"]
            });
        println!("I63_MUST_PASS {} {}", entry["id"], same);
        if !same {
            failures.push(format!("{} classifications differ", entry["id"]));
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

/// C1 G5b "same E/ê/Φ at p512" (verify.rs `e_hat` through `phi_512`). No p512 corpus base
/// exists until C1b, so the rounding is pinned directly. Expected bits were
/// derived independently as the least binary64 >= e·2^-438 with an exact
/// rational oracle (hand-checked for the subnormal ties).
#[test]
fn p512_floor_phi_follows_native_rounding() {
    for (input, expected) in [
        (0x0000000000000000u64, 0x0000000000000000u64),
        (0x3ff0000000000000, 0x2490000000000000), // 1 -> 2^-438 exactly
        (0x1a70000000000001, 0x0000001000000001), // 2^-600(1+2^-52): nearest is below, next up
        (0x7fefffffffffffff, 0x648fffffffffffff), // MAX scales exactly
        (0x0000000000000001, 0x0000000000000001), // underflows to 0, next up is 2^-1074
        (0x1838000000000000, 0x0000000000000002), // 1.5 ulp ties to even 2: not below
        (0x1844000000000000, 0x0000000000000003), // 2.5 ulp ties to even 2: below, next up
    ] {
        assert_eq!(
            rp::phi_512(f64::from_bits(input)).to_bits(),
            expected,
            "{input:016x}"
        );
    }
    assert_eq!(rp::e_hat([3.0, 0.0], 0.0), [3.0, 0.0]);
    assert_eq!(rp::e_hat([3.0, 0.0], 2.0), [3.0, 6.0]);
    assert_eq!(rp::e_hat([0.0, 8.0], 2.0), [4.0, 8.0]);
}

/// Reader-local synthetic controls (not shared corpus entries), mirroring the
/// three Python-only I62 checkpoint-B layout controls: G5a rederives the
/// canonical layout from the bound source maps, so a purported input-derived
/// force row, an unmarked constrained displacement or a nonzero prescription
/// (outside this C3 D=false scope) fails at G5a before the later G8 binding.
/// The fourth control is Rust-specific: the full canonical rebuild also rejects
/// a non-input kind relabel at G5a (the Python draft reaches G8 for it).
#[test]
fn coverage_layout_controls_fail_at_g5a() {
    use serde_json::json;
    let shared = corpus();
    let c = &shared["cases"][0];
    for (name, tail, value) in [
        (
            "reaction force row flagged input-derived",
            json!(["layout", 44, "input_derived"]),
            json!(true),
        ),
        (
            "constrained displacement not flagged input-derived",
            json!(["layout", 0, "input_derived"]),
            json!(false),
        ),
        (
            "nonzero prescription",
            json!(["constraints", 0, "value"]),
            json!("3ff0000000000000"),
        ),
        (
            "rust-specific: end-action force row relabelled translation",
            json!(["layout", 14, "kind"]),
            json!("translation"),
        ),
    ] {
        let mut path = json!(["retained_precision", "body", "sources", 0]);
        path.as_array_mut()
            .unwrap()
            .extend(tail.as_array().unwrap().iter().cloned());
        let mut source = c["source"].clone();
        edit(&mut source, &json!({"path":path,"op":"set","value":value}));
        rehash(&mut source);
        let got = rp::validate(&source, Some(&c["invocation"])).unwrap_err();
        assert_eq!(
            (got.gate, got.code.as_str()),
            ("G5a", "RETAINED_PRECISION_SCALE_MISMATCH"),
            "{name}"
        );
    }
}

/// I57 s4/s5 over-rejection guard, mirroring I62's Python-only controls: these
/// rewrites keep every public relation (feasibility, rederived estimate and
/// charge, exact rosters, record binding, direct data facts), so the reader must
/// admit them with the base classifications; only producer custody or replay
/// can catch such attested private flags. Since U7 they keep the base's eligibility.
#[test]
fn publicly_consistent_coverage_attestations_are_not_rejected() {
    use serde_json::json;
    let shared = corpus();
    let zero = "0000000000000000";
    let coverage = json!(["retained_precision", "body", "product_attempts", 0, "proof", "summary_coverage", 0]);
    let selection = json!(["retained_precision", "body", "cases", 0, "selection"]);
    let verification = json!(["retained_precision", "body", "cases", 0, "run", "records", 1, "verification"]);
    let at = |base: &Value, tail: Value| {
        let mut p = base.as_array().unwrap().clone();
        p.extend(tail.as_array().unwrap().iter().cloned());
        Value::Array(p)
    };
    let four = json!(["translation", "rotation", "force", "moment"])
        .as_array()
        .unwrap()
        .iter()
        .map(|k| json!({"body":0,"kind":k,"value":zero}))
        .collect::<Vec<_>>();
    let variants = [
        (
            "loaded_stop_without_force_moment",
            "ordinary_prepared_synthetic",
            vec![
                (at(&coverage, json!(["stop"])), json!([true, true, false, false])),
                (at(&selection, json!(["stop_rule"])), json!(four[..2])),
            ],
        ),
        (
            "loaded_all_stop_false",
            "ordinary_prepared_synthetic",
            vec![
                (at(&coverage, json!(["stop"])), json!([false, false, false, false])),
                (at(&selection, json!(["stop_rule"])), json!([])),
            ],
        ),
        (
            "no_data_all_stop_true",
            "ordinary_prepared_no_data_synthetic",
            vec![
                (at(&coverage, json!(["stop"])), json!([true, true, true, true])),
                (at(&selection, json!(["stop_rule"])), json!(four)),
            ],
        ),
        (
            "no_data_attested_data_block",
            "ordinary_prepared_no_data_synthetic",
            vec![
                (at(&coverage, json!(["has_data"])), json!(true)),
                (
                    at(&selection, json!(["certified_bound"])),
                    json!([{"body":0,"value":"3ff0000000000000"}]),
                ),
                (
                    at(&verification, json!(["bound"])),
                    json!([{"body":0,"value":"3ff0000000000000"}]),
                ),
                (at(&verification, json!(["data_blocks"])), json!(1)),
            ],
        ),
    ];
    for (name, base, edits) in variants {
        let case = shared["cases"]
            .as_array()
            .unwrap()
            .iter()
            .find(|c| c["id"] == base)
            .unwrap();
        let mut source = case["source"].clone();
        for (path, value) in edits {
            edit(&mut source, &json!({"path":path,"op":"set","value":value}));
        }
        rehash(&mut source);
        let got = rp::validate(&source, Some(&case["invocation"]))
            .unwrap_or_else(|e| panic!("{name}: {e:?}"));
        assert!(case["expected"]["numerical_eligible"].as_bool().unwrap(), "{name}: an eligible base");
        assert!(got.numerical_eligible, "{name}");
        let expected = case["expected_classifications"].as_array().unwrap();
        assert_eq!(got.classifications.len(), expected.len(), "{name}");
        for (got, want) in got.classifications.iter().zip(expected) {
            assert_eq!(got.result_id, want["result_id"], "{name}");
            assert_eq!(
                format!("{:016x}", got.normalized_bits),
                want["normalized_bits"],
                "{name}"
            );
            assert_eq!(
                got.scale_bits.map(|b| format!("{b:016x}")),
                want["scale_bits"].as_str().map(str::to_owned),
                "{name}"
            );
        }
    }
}

#[test]
fn canonical_zero_and_invalid_helper_operands() {
    for zero in [0.0, -0.0] {
        for value in [0.0, -0.0, 1.0, -1.0, f64::MAX, -f64::MAX] {
            assert_eq!(rp::absolute_bound(value, zero).unwrap().to_bits(), 0);
        }
        assert_eq!(rp::upward_product(zero, 1.0).unwrap().to_bits(), 0);
        assert_eq!(rp::upward_product(1.0, zero).unwrap().to_bits(), 0);
        assert_eq!(rp::upward_small_sum(zero, zero).unwrap().to_bits(), 1);
    }
    for bad in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY, -1.0] {
        assert!(rp::absolute_bound(1.0, bad).is_err());
        assert!(rp::upward_small_sum(bad, 0.0).is_err());
        assert!(rp::upward_small_sum(0.0, bad).is_err());
        assert!(rp::upward_product(1.0, bad).is_err());
    }
    for bad in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        assert!(rp::absolute_bound(bad, 1.0).is_err());
    }
    assert!(rp::upward_small_sum(f64::MAX, 0.0).is_err());
    assert!(rp::upward_small_sum(f64::MAX, f64::MAX).is_err());
}

// Expected values independently computed with Fraction and least-upper binary search.
// Every expected result and its immediate predecessor were checked in the rational oracle.
#[test]
fn independent_rational_product_and_single_round_sum_controls() {
    let cases: &[(u64, u64, Option<u64>, Option<u64>)] = &[
        (
            0x0000000000000000,
            0x0000000000000000,
            Some(0x0000000000000000),
            Some(0x0000000000000001),
        ),
        (
            0x0000000000000000,
            0x0000000000000001,
            Some(0x0000000000000000),
            Some(0x0000000000000002),
        ),
        (
            0x0000000000000000,
            0x0000000000000002,
            Some(0x0000000000000000),
            Some(0x0000000000000003),
        ),
        (
            0x0000000000000000,
            0x000fffffffffffff,
            Some(0x0000000000000000),
            Some(0x0010000000000000),
        ),
        (
            0x0000000000000000,
            0x0010000000000000,
            Some(0x0000000000000000),
            Some(0x0010000000000001),
        ),
        (
            0x0000000000000000,
            0x3ff0000000000000,
            Some(0x0000000000000000),
            Some(0x3ff0000000000001),
        ),
        (
            0x0000000000000000,
            0x3ff0000000000001,
            Some(0x0000000000000000),
            Some(0x3ff0000000000002),
        ),
        (
            0x0000000000000000,
            0x7fefffffffffffff,
            Some(0x0000000000000000),
            None,
        ),
        (
            0x0000000000000001,
            0x0000000000000000,
            Some(0x0000000000000000),
            Some(0x0000000000000002),
        ),
        (
            0x0000000000000001,
            0x0000000000000001,
            Some(0x0000000000000001),
            Some(0x0000000000000003),
        ),
        (
            0x0000000000000001,
            0x0000000000000002,
            Some(0x0000000000000001),
            Some(0x0000000000000004),
        ),
        (
            0x0000000000000001,
            0x000fffffffffffff,
            Some(0x0000000000000001),
            Some(0x0010000000000001),
        ),
        (
            0x0000000000000001,
            0x0010000000000000,
            Some(0x0000000000000001),
            Some(0x0010000000000002),
        ),
        (
            0x0000000000000001,
            0x3ff0000000000000,
            Some(0x0000000000000001),
            Some(0x3ff0000000000001),
        ),
        (
            0x0000000000000001,
            0x3ff0000000000001,
            Some(0x0000000000000002),
            Some(0x3ff0000000000002),
        ),
        (
            0x0000000000000001,
            0x7fefffffffffffff,
            Some(0x3ccfffffffffffff),
            None,
        ),
        (
            0x0000000000000002,
            0x0000000000000000,
            Some(0x0000000000000000),
            Some(0x0000000000000003),
        ),
        (
            0x0000000000000002,
            0x0000000000000001,
            Some(0x0000000000000001),
            Some(0x0000000000000004),
        ),
        (
            0x0000000000000002,
            0x0000000000000002,
            Some(0x0000000000000001),
            Some(0x0000000000000005),
        ),
        (
            0x0000000000000002,
            0x000fffffffffffff,
            Some(0x0000000000000001),
            Some(0x0010000000000002),
        ),
        (
            0x0000000000000002,
            0x0010000000000000,
            Some(0x0000000000000001),
            Some(0x0010000000000003),
        ),
        (
            0x0000000000000002,
            0x3ff0000000000000,
            Some(0x0000000000000002),
            Some(0x3ff0000000000001),
        ),
        (
            0x0000000000000002,
            0x3ff0000000000001,
            Some(0x0000000000000003),
            Some(0x3ff0000000000002),
        ),
        (
            0x0000000000000002,
            0x7fefffffffffffff,
            Some(0x3cdfffffffffffff),
            None,
        ),
        (
            0x000fffffffffffff,
            0x0000000000000000,
            Some(0x0000000000000000),
            Some(0x0010000000000000),
        ),
        (
            0x000fffffffffffff,
            0x0000000000000001,
            Some(0x0000000000000001),
            Some(0x0010000000000001),
        ),
        (
            0x000fffffffffffff,
            0x0000000000000002,
            Some(0x0000000000000001),
            Some(0x0010000000000002),
        ),
        (
            0x000fffffffffffff,
            0x000fffffffffffff,
            Some(0x0000000000000001),
            Some(0x001fffffffffffff),
        ),
        (
            0x000fffffffffffff,
            0x0010000000000000,
            Some(0x0000000000000001),
            Some(0x0020000000000000),
        ),
        (
            0x000fffffffffffff,
            0x3ff0000000000000,
            Some(0x000fffffffffffff),
            Some(0x3ff0000000000001),
        ),
        (
            0x000fffffffffffff,
            0x3ff0000000000001,
            Some(0x0010000000000000),
            Some(0x3ff0000000000002),
        ),
        (
            0x000fffffffffffff,
            0x7fefffffffffffff,
            Some(0x400ffffffffffffe),
            None,
        ),
        (
            0x0010000000000000,
            0x0000000000000000,
            Some(0x0000000000000000),
            Some(0x0010000000000001),
        ),
        (
            0x0010000000000000,
            0x0000000000000001,
            Some(0x0000000000000001),
            Some(0x0010000000000002),
        ),
        (
            0x0010000000000000,
            0x0000000000000002,
            Some(0x0000000000000001),
            Some(0x0010000000000003),
        ),
        (
            0x0010000000000000,
            0x000fffffffffffff,
            Some(0x0000000000000001),
            Some(0x0020000000000000),
        ),
        (
            0x0010000000000000,
            0x0010000000000000,
            Some(0x0000000000000001),
            Some(0x0020000000000001),
        ),
        (
            0x0010000000000000,
            0x3ff0000000000000,
            Some(0x0010000000000000),
            Some(0x3ff0000000000001),
        ),
        (
            0x0010000000000000,
            0x3ff0000000000001,
            Some(0x0010000000000001),
            Some(0x3ff0000000000002),
        ),
        (
            0x0010000000000000,
            0x7fefffffffffffff,
            Some(0x400fffffffffffff),
            None,
        ),
        (
            0x3ff0000000000000,
            0x0000000000000000,
            Some(0x0000000000000000),
            Some(0x3ff0000000000001),
        ),
        (
            0x3ff0000000000000,
            0x0000000000000001,
            Some(0x0000000000000001),
            Some(0x3ff0000000000001),
        ),
        (
            0x3ff0000000000000,
            0x0000000000000002,
            Some(0x0000000000000002),
            Some(0x3ff0000000000001),
        ),
        (
            0x3ff0000000000000,
            0x000fffffffffffff,
            Some(0x000fffffffffffff),
            Some(0x3ff0000000000001),
        ),
        (
            0x3ff0000000000000,
            0x0010000000000000,
            Some(0x0010000000000000),
            Some(0x3ff0000000000001),
        ),
        (
            0x3ff0000000000000,
            0x3ff0000000000000,
            Some(0x3ff0000000000000),
            Some(0x4000000000000001),
        ),
        (
            0x3ff0000000000000,
            0x3ff0000000000001,
            Some(0x3ff0000000000001),
            Some(0x4000000000000001),
        ),
        (
            0x3ff0000000000000,
            0x7fefffffffffffff,
            Some(0x7fefffffffffffff),
            None,
        ),
        (
            0x3ff0000000000001,
            0x0000000000000000,
            Some(0x0000000000000000),
            Some(0x3ff0000000000002),
        ),
        (
            0x3ff0000000000001,
            0x0000000000000001,
            Some(0x0000000000000002),
            Some(0x3ff0000000000002),
        ),
        (
            0x3ff0000000000001,
            0x0000000000000002,
            Some(0x0000000000000003),
            Some(0x3ff0000000000002),
        ),
        (
            0x3ff0000000000001,
            0x000fffffffffffff,
            Some(0x0010000000000000),
            Some(0x3ff0000000000002),
        ),
        (
            0x3ff0000000000001,
            0x0010000000000000,
            Some(0x0010000000000001),
            Some(0x3ff0000000000002),
        ),
        (
            0x3ff0000000000001,
            0x3ff0000000000000,
            Some(0x3ff0000000000001),
            Some(0x4000000000000001),
        ),
        (
            0x3ff0000000000001,
            0x3ff0000000000001,
            Some(0x3ff0000000000003),
            Some(0x4000000000000002),
        ),
        (0x3ff0000000000001, 0x7fefffffffffffff, None, None),
        (
            0x7fefffffffffffff,
            0x0000000000000000,
            Some(0x0000000000000000),
            None,
        ),
        (
            0x7fefffffffffffff,
            0x0000000000000001,
            Some(0x3ccfffffffffffff),
            None,
        ),
        (
            0x7fefffffffffffff,
            0x0000000000000002,
            Some(0x3cdfffffffffffff),
            None,
        ),
        (
            0x7fefffffffffffff,
            0x000fffffffffffff,
            Some(0x400ffffffffffffe),
            None,
        ),
        (
            0x7fefffffffffffff,
            0x0010000000000000,
            Some(0x400fffffffffffff),
            None,
        ),
        (
            0x7fefffffffffffff,
            0x3ff0000000000000,
            Some(0x7fefffffffffffff),
            None,
        ),
        (0x7fefffffffffffff, 0x3ff0000000000001, None, None),
        (0x7fefffffffffffff, 0x7fefffffffffffff, None, None),
        (
            0x1e14c1df32b7eb67,
            0x58f5a9f6bd147bdf,
            Some(0x371c1af4d22d76fc),
            Some(0x58f5a9f6bd147be0),
        ),
        (
            0x45a1bd06d6f37315,
            0x69dc42e529de3bce,
            Some(0x6f8f54f5c43d5caf),
            Some(0x69dc42e529de3bcf),
        ),
        (
            0x32ffc4f8841a1663,
            0x00758dfe049896e7,
            Some(0x0000000000000001),
            Some(0x32ffc4f8841a1664),
        ),
        (
            0x0f44b3af157d881f,
            0x69e910987501504d,
            Some(0x3940371d418648e8),
            Some(0x69e910987501504e),
        ),
        (
            0x59dafd6f66bb2720,
            0x7bdaf27a12034453,
            None,
            Some(0x7bdaf27a12034454),
        ),
        (
            0x3c085d7ec865e9bb,
            0x5d502e64cc7f5fdd,
            Some(0x5968a42514a87bce),
            Some(0x5d502e64cc7f5fde),
        ),
        (
            0x7379354794a89760,
            0x0890bd8e67b2714e,
            Some(0x3c1a5fed4e700641),
            Some(0x7379354794a89761),
        ),
        (
            0x790e76d10ab72b48,
            0x1a967111d5ad54f5,
            Some(0x53b55d544416b4ef),
            Some(0x790e76d10ab72b49),
        ),
        (
            0x47af280bbabd5668,
            0x1df0c52f6a03ee99,
            Some(0x25b054028f9dfe01),
            Some(0x47af280bbabd5669),
        ),
        (
            0x3555abfd56407372,
            0x0b1c4f0d8c08c319,
            Some(0x00832c07719aa2a5),
            Some(0x3555abfd56407373),
        ),
        (
            0x50b0a66a03cac061,
            0x5912f88f59c516a1,
            Some(0x69d3bddfdc3d2732),
            Some(0x5912f88f59c516a2),
        ),
        (
            0x54f8810202bf0b6b,
            0x40f703cfeaebab2f,
            Some(0x56019fa4c0926913),
            Some(0x54f8810202bf0b6c),
        ),
        (
            0x007756f928519d84,
            0x72cc29c001c3ee89,
            Some(0x33548a8d7da3a412),
            Some(0x72cc29c001c3ee8a),
        ),
        (
            0x0e72468450ddfafb,
            0x7a802b184971bb78,
            Some(0x490277bd92216cb4),
            Some(0x7a802b184971bb79),
        ),
        (
            0x130f0192f1925d73,
            0x2fee14a73fa050a2,
            Some(0x030d257ccc2d7f16),
            Some(0x2fee14a73fa050a3),
        ),
        (
            0x1cd97741ba5d4455,
            0x6ffd93d326ff751e,
            Some(0x4ce789b77036f0cb),
            Some(0x6ffd93d326ff751f),
        ),
        (
            0x2ae0cdbd939da8a2,
            0x14a94c27b30e8d35,
            Some(0x00001a91732cae9e),
            Some(0x2ae0cdbd939da8a3),
        ),
        (
            0x0ea67c3523d9183d,
            0x761964445d6b5c9a,
            Some(0x44d1d77db3a6f85c),
            Some(0x761964445d6b5c9b),
        ),
        (
            0x5c5d0bd8dad67520,
            0x481cee444497234f,
            Some(0x648a42a3d3c66f50),
            Some(0x5c5d0bd8dad67521),
        ),
        (
            0x7c00a178e98d4d09,
            0x59d31f8813d0a5a2,
            None,
            Some(0x7c00a178e98d4d0a),
        ),
        (
            0x263bbd3068423790,
            0x6bb0413a9a5064ad,
            Some(0x51fc2e46964190ea),
            Some(0x6bb0413a9a5064ae),
        ),
        (
            0x19b7b224d143a01a,
            0x0165f75fe299c99c,
            Some(0x0000000000000001),
            Some(0x19b7b224d143a01b),
        ),
        (
            0x58c28aaeb07713b7,
            0x34c5ff5d30c5acca,
            Some(0x4d997df386553534),
            Some(0x58c28aaeb07713b8),
        ),
        (
            0x2d3b3176f09ec037,
            0x1becffd5ce90505f,
            Some(0x0938a4afef211b6e),
            Some(0x2d3b3176f09ec038),
        ),
        (
            0x6549797e3d6a6a81,
            0x7b0537e3bbd836f7,
            None,
            Some(0x7b0537e3bbd836f8),
        ),
        (
            0x3b8bbfe507c4f700,
            0x76869f95022c94a1,
            Some(0x72239e5026bb17b7),
            Some(0x76869f95022c94a2),
        ),
        (
            0x36b01599fe419fd4,
            0x120289ac726e2bbc,
            Some(0x08c2a2b38fdafca6),
            Some(0x36b01599fe419fd5),
        ),
        (
            0x0800247f1281a7fa,
            0x44ed643569f39d0b,
            Some(0x0cfda7404f88e2c2),
            Some(0x44ed643569f39d0c),
        ),
        (
            0x7804a4d869e9f8dd,
            0x3b3934efaed29390,
            Some(0x735042ef92636f6b),
            Some(0x7804a4d869e9f8de),
        ),
        (
            0x5463e746c0715d9b,
            0x2dd5839525b71047,
            Some(0x424ac33bf889d840),
            Some(0x5463e746c0715d9c),
        ),
        (
            0x491b06f77cf06eca,
            0x0f8df37aec7fcf2e,
            Some(0x18b94bf502e58661),
            Some(0x491b06f77cf06ecb),
        ),
        (
            0x7169074ddfe7a1bb,
            0x136ce4aa9178cf47,
            Some(0x44e6993d975b4bc4),
            Some(0x7169074ddfe7a1bc),
        ),
        (
            0x675b4f86f0c4ded9,
            0x3d809336a53a79d3,
            Some(0x64ec4acedef84bff),
            Some(0x675b4f86f0c4deda),
        ),
        (
            0x7f373395edbad3ca,
            0x60934def1d9f089f,
            None,
            Some(0x7f373395edbad3cb),
        ),
        (
            0x0a4b341bdadc9113,
            0x4a070eed184bbb09,
            Some(0x14639a2460140bbd),
            Some(0x4a070eed184bbb0a),
        ),
        (
            0x1ba58a2050d87045,
            0x132176b02c8f040b,
            Some(0x0000000000000001),
            Some(0x1ba58a2050d87046),
        ),
        (
            0x044696ceeca9e87c,
            0x23a82c9f981c63e9,
            Some(0x0000000000000001),
            Some(0x23a82c9f981c63ea),
        ),
        (
            0x24e2f0b4be902c75,
            0x6cc7800493d19287,
            Some(0x51bbd18ee30c8334),
            Some(0x6cc7800493d19288),
        ),
        (
            0x31be23921ba29e40,
            0x4b473dd6511f23eb,
            Some(0x3d15e3ceacaa06ab),
            Some(0x4b473dd6511f23ec),
        ),
        (
            0x4ff3f108b47aac5d,
            0x6aa0d78ae01a34ee,
            Some(0x7aa4fdacafae14f1),
            Some(0x6aa0d78ae01a34ef),
        ),
        (
            0x1c6c501b45b372b2,
            0x01a48b91afde62af,
            Some(0x0000000000000001),
            Some(0x1c6c501b45b372b3),
        ),
        (
            0x22a05475fb83ade1,
            0x595ac6798ebcbe19,
            Some(0x3c0b53d0fd28efb1),
            Some(0x595ac6798ebcbe1a),
        ),
        (
            0x28439aa40bcb1e76,
            0x1307fe164a6e6ff1,
            Some(0x0000000000000001),
            Some(0x28439aa40bcb1e77),
        ),
        (
            0x34f06c9efed7f618,
            0x6b73f608142a485f,
            Some(0x60747d8b26315a3b),
            Some(0x6b73f608142a4860),
        ),
        (
            0x489f43e2559e4d15,
            0x520c1729a6c579ee,
            Some(0x5abb720787832214),
            Some(0x520c1729a6c579ef),
        ),
        (
            0x6c5d233d42db4c15,
            0x313d400eb7f55ba7,
            Some(0x5daaa24366083063),
            Some(0x6c5d233d42db4c16),
        ),
        (
            0x7a4b5d6de1e2e0fa,
            0x48b69fcdce093260,
            None,
            Some(0x7a4b5d6de1e2e0fb),
        ),
        (
            0x072bdf0fd674a571,
            0x4eee53d76e643698,
            Some(0x162a6a250cfa1197),
            Some(0x4eee53d76e643699),
        ),
        (
            0x415c1c6ddd1638c3,
            0x6f93e0107648a5e7,
            Some(0x710175b6b27062a1),
            Some(0x6f93e0107648a5e8),
        ),
        (
            0x4d674000a80223da,
            0x76d35940f3e72527,
            None,
            Some(0x76d35940f3e72528),
        ),
        (
            0x0815afd3c1eb0758,
            0x27c52fcb3aadec7f,
            Some(0x0000000000000001),
            Some(0x27c52fcb3aadec80),
        ),
        (
            0x08342bb3953f38c0,
            0x54e5da9bfdbca2ec,
            Some(0x1d2b8cf3c9f10440),
            Some(0x54e5da9bfdbca2ed),
        ),
        (
            0x14776c2e4838c43a,
            0x4c6897d24047e28c,
            Some(0x20f20043158c1368),
            Some(0x4c6897d24047e28d),
        ),
        (
            0x19f43e99520159ff,
            0x0c50b78289be9a30,
            Some(0x0000000000000001),
            Some(0x19f43e9952015a00),
        ),
        (
            0x20b3b14903307b17,
            0x17e03a1ca5cb7113,
            Some(0x0000000000000001),
            Some(0x20b3b14903307b18),
        ),
        (
            0x08ef2e045830e0e9,
            0x41a7871907eeed9a,
            Some(0x0aa6ecb5a5657379),
            Some(0x41a7871907eeed9b),
        ),
        (
            0x43c2720f29b1d4ad,
            0x2f13781dcab28a9e,
            Some(0x32e671ebcba53200),
            Some(0x43c2720f29b1d4ae),
        ),
        (
            0x1f14d4c069b24d7d,
            0x042a240fea98575a,
            Some(0x0000000000000001),
            Some(0x1f14d4c069b24d7e),
        ),
        (
            0x26bdd27ef6f335f1,
            0x494242d6993ea82b,
            Some(0x301104b18f480b0e),
            Some(0x494242d6993ea82c),
        ),
        (
            0x1679aa7860758b66,
            0x7e78facff51ee5f0,
            Some(0x55040904e01e60b6),
            Some(0x7e78facff51ee5f1),
        ),
        (
            0x2f27f35569148f18,
            0x7626f920f17d9aad,
            Some(0x656131c0d0ebd0f9),
            Some(0x7626f920f17d9aae),
        ),
        (
            0x02b8fef331f932e9,
            0x161744a438bbb1b5,
            Some(0x0000000000000001),
            Some(0x161744a438bbb1b6),
        ),
        (
            0x3f264fe9b297eb7a,
            0x13b03343d03c421c,
            Some(0x12e69766fc86b184),
            Some(0x3f264fe9b297eb7b),
        ),
        (
            0x7d42a9e9a20deee9,
            0x7f80203059681946,
            None,
            Some(0x7f802030596943e5),
        ),
        (
            0x3cff89f8637b6b63,
            0x3f7019b9dae50d21,
            Some(0x3c7fbcae525a335e),
            Some(0x3f7019b9dae52cab),
        ),
        (
            0x798d4764731a53aa,
            0x70d8ab12a99eb0cb,
            None,
            Some(0x798d4764731a53ab),
        ),
        (
            0x254b680bc66a8821,
            0x32272dbc557fe470,
            Some(0x1783d9f413aaad35),
            Some(0x32272dbc557fe471),
        ),
        (
            0x458f4e911a472b06,
            0x26b8a2c4a611ca00,
            Some(0x2c581a2af5fc2198),
            Some(0x458f4e911a472b07),
        ),
        (
            0x2091ddf2e1b3504d,
            0x3492cc77dc34ea7d,
            Some(0x1534fe04ef4cba0b),
            Some(0x3492cc77dc34ea7e),
        ),
        (
            0x2a99110bb4e7347d,
            0x063d43f35af9d728,
            Some(0x0000000000000001),
            Some(0x2a99110bb4e7347e),
        ),
        (
            0x4195dd912bb6f27d,
            0x559cef52c6638fb3,
            Some(0x5743c56669d18c7e),
            Some(0x559cef52c6638fb4),
        ),
        (
            0x62d788b7c1c6f2c1,
            0x3fac2238d89b3ab6,
            Some(0x6294b0cbdb5a8cab),
            Some(0x62d788b7c1c6f2c2),
        ),
        (
            0x530c07ab017c2eed,
            0x2edd8bc158e575aa,
            Some(0x41f9e15da444731d),
            Some(0x530c07ab017c2eee),
        ),
        (
            0x2502a8d101bf661b,
            0x6f2b2cd270774238,
            Some(0x543fb1265c97cd6f),
            Some(0x6f2b2cd270774239),
        ),
        (
            0x35d44331b54ed27d,
            0x23393fe1bca3aeda,
            Some(0x191ff9e41ed4e6f1),
            Some(0x35d44331b54ed27e),
        ),
        (
            0x69be8e05ce0e47fb,
            0x06e0cb2a4a82de6a,
            Some(0x30b009003d674fcc),
            Some(0x69be8e05ce0e47fc),
        ),
        (
            0x33c906276ad8821f,
            0x7e74cee6da78d8e9,
            Some(0x725045a4c79ae049),
            Some(0x7e74cee6da78d8ea),
        ),
        (
            0x3f04ac5c61624c5a,
            0x28427ee165a76fed,
            Some(0x2757e5d88038f43e),
            Some(0x3f04ac5c61624c5b),
        ),
        (
            0x2f29e8cd37c88e0d,
            0x5b9c064005dc6080,
            Some(0x4ad6b0c30da3302d),
            Some(0x5b9c064005dc6081),
        ),
        (
            0x2ca3f454236f95df,
            0x29fdfe6a8fc801d6,
            Some(0x16b2b4120ef5ab1b),
            Some(0x2ca3f454236f999f),
        ),
        (
            0x1a5a58292a12e8b4,
            0x049cdf322628936d,
            Some(0x0000000000000001),
            Some(0x1a5a58292a12e8b5),
        ),
        (
            0x022f314ff57d5c37,
            0x7de9e797fbca04f6,
            Some(0x40294046971f16b1),
            Some(0x7de9e797fbca04f7),
        ),
        (
            0x4bcbf7a7483b2a60,
            0x49ce869a0f2a1f3f,
            Some(0x55aaadd0854be1dd),
            Some(0x4bcbf7a74859b0fb),
        ),
        (
            0x37be91e7024a654f,
            0x1fa2e2231fc59290,
            Some(0x17720a19eeb8c6ed),
            Some(0x37be91e7024a6550),
        ),
        (
            0x3ff9aad67a26435c,
            0x164e08ae165ad7ce,
            Some(0x1658171f6afef1f0),
            Some(0x3ff9aad67a26435d),
        ),
        (
            0x1a9ca6b3b2722423,
            0x4559d50f457b1698,
            Some(0x200720ffabded818),
            Some(0x4559d50f457b1699),
        ),
        (
            0x71fe9de609f1343a,
            0x618ced671552302d,
            None,
            Some(0x71fe9de609f1343b),
        ),
        (
            0x0bea4d22585cc8b1,
            0x736061ccf40390f8,
            Some(0x3f5aede6e1288a59),
            Some(0x736061ccf40390f9),
        ),
        (
            0x712b888fd8ecb9c3,
            0x0954ae5d09cb0d4a,
            Some(0x3a91cb60830ea7f1),
            Some(0x712b888fd8ecb9c4),
        ),
        (
            0x018ac9dcff27c830,
            0x5521dc1d8f6be511,
            Some(0x16bde703ebd002d0),
            Some(0x5521dc1d8f6be512),
        ),
        (
            0x4e0655348b22bed2,
            0x217433eac7a3de61,
            Some(0x2f8c32f8fa2fd813),
            Some(0x4e0655348b22bed3),
        ),
        (
            0x730004d9a1f9a596,
            0x4ee90d54da62dab9,
            None,
            Some(0x730004d9a1f9a597),
        ),
        (
            0x13b641e174d4bdaa,
            0x3041de7d346986c2,
            Some(0x0408db7fcff15a94),
            Some(0x3041de7d346986c3),
        ),
        (
            0x09986c4f138daa1c,
            0x30146570995132d1,
            Some(0x0000000000000001),
            Some(0x30146570995132d2),
        ),
        (
            0x0e4763221e174bac,
            0x0fce64ce62412cf7,
            Some(0x0000000000000001),
            Some(0x0fce64ce79a44f16),
        ),
        (
            0x58c821e97a783236,
            0x62b626ea5c5d45c7,
            Some(0x7b90b4a98717373e),
            Some(0x62b626ea5c5d45c8),
        ),
    ];
    for &(a, b, product, total) in cases {
        assert_eq!(
            rp::upward_product(f64::from_bits(a), f64::from_bits(b))
                .ok()
                .map(f64::to_bits),
            product,
            "product {a:016x} {b:016x}"
        );
        assert_eq!(
            rp::upward_small_sum(f64::from_bits(a), f64::from_bits(b))
                .ok()
                .map(f64::to_bits),
            total,
            "single-round sum {a:016x} {b:016x}"
        );
    }
}

#[test]
fn audit_mutations_retain_native_and_product_first_failures() {
    use serde_json::json;
    let shared = corpus();
    let c = &shared["cases"][0];
    let mutations = [
        (
            "residual basis",
            json!([
                "retained_precision",
                "body",
                "cases",
                0,
                "run",
                "records",
                0,
                "residual_basis"
            ]),
            json!(128),
            "RETAINED_PRECISION_ATTEMPT_MISMATCH",
        ),
        (
            "refinement count",
            json!([
                "retained_precision",
                "body",
                "cases",
                0,
                "run",
                "records",
                0,
                "corrections"
            ]),
            json!(4),
            "RETAINED_PRECISION_ATTEMPT_MISMATCH",
        ),
        (
            "stop stage",
            json!([
                "retained_precision",
                "body",
                "cases",
                0,
                "run",
                "records",
                0,
                "work",
                "stop_rule_lme"
            ]),
            json!(0),
            "RETAINED_PRECISION_WORK_MISMATCH",
        ),
        (
            "missing entered lane",
            json!([
                "retained_precision",
                "body",
                "product_attempts",
                0,
                "proof",
                "lanes"
            ]),
            json!([]),
            "RETAINED_PRECISION_PRODUCT_ATTEMPT_MISMATCH",
        ),
        (
            "missing completed projection",
            json!([
                "retained_precision",
                "body",
                "product_attempts",
                0,
                "proof",
                "projection_outcomes"
            ]),
            json!([]),
            "RETAINED_PRECISION_PRODUCT_ATTEMPT_MISMATCH",
        ),
        (
            "unmerged completed values",
            json!([
                "retained_precision",
                "body",
                "product_attempts",
                0,
                "proof",
                "completion"
            ]),
            json!({"kind":"not_entered"}),
            "RETAINED_PRECISION_PRODUCT_ATTEMPT_MISMATCH",
        ),
    ];
    for (name, path, value, code) in mutations {
        let mut source = c["source"].clone();
        edit(&mut source, &json!({"path":path,"op":"set","value":value}));
        rehash(&mut source);
        let got = rp::validate(&source, Some(&c["invocation"])).unwrap_err();
        assert_eq!((got.gate, got.code.as_str()), ("G5", code), "{name}");
    }
    let mut source = c["source"].clone();
    source["retained_precision"]["body"]["product_attempts"][0]["preparation"]["members"][0]
        ["work"]["conversions"]["value"] = json!(8);
    source["retained_precision"]["body"]["product_attempts"][0]["proof"]["completion"] =
        json!({"kind":"not_entered"});
    rehash(&mut source);
    let got = rp::validate(&source, Some(&c["invocation"])).unwrap_err();
    assert_eq!(
        (got.gate, got.code.as_str()),
        ("G5", "RETAINED_PRECISION_PRODUCT_ATTEMPT_MISMATCH"),
        "C3 association must precede earlier member work defect"
    );
}

#[test]
fn negative_zero_wire_scale_remains_g2() {
    let shared = corpus();
    let c = &shared["cases"][0];
    let mut source = c["source"].clone();
    source["retained_precision"]["body"]["cases"][0]["selection"]["body_scales"][0]
        ["translation"] = "8000000000000000".into();
    source["results"][0]["recovery_method"] = "wrong".into();
    rehash(&mut source);
    let got = rp::validate(&source, Some(&c["invocation"])).unwrap_err();
    assert_eq!(
        (got.gate, got.code.as_str()),
        ("G2", "RETAINED_PRECISION_ENCODING_MISMATCH")
    );
}

// ---------------------------------------------------------------------------
// Review repair wave 07, phase 1 (ROOT ruling "Reader review RV78-RV81",
// D1-D15): one reader-local test per relation, built from RV78's PROBES.json
// edits where one exists. These are not shared corpus entries.

fn rb(tail: Value) -> Value {
    let mut p = vec![Value::from("retained_precision"), Value::from("body")];
    p.extend(tail.as_array().unwrap().iter().cloned());
    Value::Array(p)
}
fn set(path: Value, value: Value) -> Value {
    serde_json::json!({"path": path, "op": "set", "value": value})
}
fn remove(path: Value) -> Value {
    serde_json::json!({"path": path, "op": "remove"})
}
fn base_source(shared: &Value, id: &str) -> Value {
    shared["cases"]
        .as_array()
        .unwrap()
        .iter()
        .find(|c| c["id"] == id)
        .unwrap()["source"]
        .clone()
}
/// Observe a reader-local probe: `edits` on `base`, fully rehashed.
fn probe(shared: &Value, base: &str, edits: Vec<Value>) -> Value {
    observe(
        shared,
        &serde_json::json!({"id": "i63_probe", "base": base, "edits": edits, "rehash": "all"}),
    )
}
fn gate(g: &str, code: &str) -> Value {
    serde_json::json!({"gate": g, "code": code})
}
const COVERAGE: &str = "RETAINED_PRECISION_COVERAGE_MISMATCH";
const ATTEMPT: &str = "RETAINED_PRECISION_ATTEMPT_MISMATCH";
const PRODUCT: &str = "RETAINED_PRECISION_PRODUCT_ATTEMPT_MISMATCH";
const UNSUPPORTED: &str = "SOURCE_PRODUCER_CONTRACT_UNSUPPORTED";
const P_BASE: &str = "two_case_preparation_failure_synthetic";
const F_BASE: &str = "two_case_facade_after_certificate_synthetic";
const ORD: &str = "ordinary_prepared_synthetic";

/// D1: G3 member ids, complete old inventories, captured_prefix split, run ids
/// and execution order; Run origin owner in G5 class 1.
#[test]
fn d1_g3_coverage_relations() {
    use serde_json::json;
    let shared = corpus();
    let p = base_source(&shared, P_BASE);
    let old0 = p["retained_precision"]["body"]["product_attempts"][1]["operational"]["old"][0].clone();
    let mut old1 = old0.clone();
    old1["member"] = json!(1);
    let cases = [
        // RV78 R2b / RV80-S2: an unsourced old id outside 0..len (was G8).
        ("unsourced old id 1", P_BASE, vec![set(rb(json!(["product_attempts", 1, "operational", "old", 0, "member"])), json!(1))], gate("G3", COVERAGE)),
        // A prepared member id outside 0..len.
        ("prepared id 1", ORD, vec![set(rb(json!(["product_attempts", 0, "preparation", "members", 0, "member"])), json!(1))], gate("G3", COVERAGE)),
        // Unsourced complete old: empty, or a count other than the CaseSource's.
        ("unsourced complete old longer than the model", P_BASE, vec![set(rb(json!(["product_attempts", 1, "operational", "old"])), json!([old0, old1]))], gate("G3", COVERAGE)),
        // RV78 R3/R3b: sourced complete old against the source member map.
        ("sourced old short of source", "two_body_synthetic", vec![remove(rb(json!(["product_attempts", 0, "operational", "old", 1])))], gate("G3", COVERAGE)),
        // RV78 R1a/R1b: run id = execution-order position.
        ("execution order swapped", "two_case_synthetic", vec![set(rb(json!(["work", "execution_order"])), json!([{"kind":"case","index":1},{"kind":"case","index":0}]))], gate("G3", COVERAGE)),
        ("run id not its position", "two_case_synthetic", vec![set(rb(json!(["cases", 1, "run", "id"])), json!(5)), set(rb(json!(["calls", 0, "run_refs"])), json!([0, 5]))], gate("G3", COVERAGE)),
        // D1: Run origin owner is G5 class 1, not G3.
        ("run origin owner moved (G5 class 1)", "two_case_synthetic", vec![
            set(rb(json!(["cases", 1, "run", "origin", "owner_ref"])), json!({"kind":"case","index":0})),
            set(rb(json!(["calls", 0, "owner_refs", 1])), json!({"kind":"case","index":0})),
        ], gate("G5", ATTEMPT)),
    ];
    let mut misses = Vec::new();
    for (name, base, edits, want) in cases {
        let got = probe(&shared, base, edits);
        if got != want {
            misses.push(format!("{name}: got {got} want {want}"));
        }
    }
    assert!(misses.is_empty(), "{}", misses.join("\n"));
    // D1 (checkpoint A): an empty unsourced complete old list is not a G3
    // failure by itself; here the receipt has a one-member CaseSource, so the
    // count comparison rejects it at G3.
    assert_eq!(
        probe(&shared, P_BASE, vec![set(rb(json!(["product_attempts", 1, "operational", "old"])), json!([]))]),
        gate("G3", COVERAGE),
        "empty list against a one-member CaseSource"
    );
    // D1: captured_prefix references are G5 PRODUCT_ATTEMPT, not G3.
    let mut captured = shared["must_pass"]
        .as_array()
        .unwrap()
        .iter()
        .find(|m| m["id"] == "prefix_captured")
        .unwrap()
        .clone();
    captured["edits"].as_array_mut().unwrap().push(set(rb(json!(["product_attempts", 1, "run_ref"])), json!(0)));
    assert_eq!(observe(&shared, &captured), gate("G5", PRODUCT), "captured prefix with a run_ref");
}

/// D2: the G0 union; absent or wrong-typed G0 fields fail G0, other shape
/// defects wait for G1.
#[test]
fn d2_g0_union() {
    use serde_json::json;
    let shared = corpus();
    let mut misses = Vec::new();
    let g0 = gate("G0", UNSUPPORTED);
    for (name, edits) in [
        ("canonicalization absent", vec![remove(rb(json!(["canonicalization"])))]),
        ("policy absent", vec![remove(rb(json!(["policy"])))]),
        ("receipt_version 2", vec![set(rb(json!(["receipt_version"])), json!(2))]),
        ("receipt_version as text", vec![set(rb(json!(["receipt_version"])), json!("1"))]),
        ("case_limit absent", vec![remove(rb(json!(["work", "case_limit"])))]),
        ("invocation_limit changed", vec![set(rb(json!(["work", "invocation_limit"])), json!(60000000001u64))]),
        ("component_version", vec![set(json!(["producer", "component_version"]), json!("0.3.0"))]),
        ("schema_version", vec![set(json!(["schema_version"]), json!("0.3.0"))]),
        ("definition_id absent", vec![remove(rb(json!(["product_attempts", 0, "definition_id"])))]),
    ] {
        let got = probe(&shared, ORD, edits);
        if got != g0 {
            misses.push(format!("{name}: got {got}"));
        }
    }
    assert!(misses.is_empty(), "{}", misses.join("\n"));
    // A shape defect outside the G0 union is G1.
    assert_eq!(
        probe(&shared, ORD, vec![set(rb(json!(["unexpected_member"])), json!(0))]),
        gate("G1", "RETAINED_PRECISION_RECEIPT_MISMATCH")
    );
}

/// D3: inside native class 1 an ATTEMPT defect wins over an earlier WORK one.
#[test]
fn d3_native_attempt_before_work() {
    use serde_json::json;
    let shared = corpus();
    assert_eq!(
        probe(&shared, ORD, vec![
            set(rb(json!(["calls", 0, "invocation_before"])), json!(1)),
            set(rb(json!(["cases", 0, "run", "records", 0, "corrections"])), json!(4)),
            set(rb(json!(["cases", 0, "selection", "corrections"])), json!(4)),
        ]),
        gate("G5", ATTEMPT)
    );
    // D16: a dangling native build reference is WORK (C1 build provenance),
    // deferred, so a class-1 ATTEMPT defect still wins.
    let dangling = set(rb(json!(["cases", 0, "run", "records", 0, "shared_build_ref"])), json!(99));
    assert_eq!(probe(&shared, ORD, vec![dangling.clone()]), gate("G5", "RETAINED_PRECISION_WORK_MISMATCH"));
    assert_eq!(
        probe(&shared, ORD, vec![
            dangling,
            set(rb(json!(["cases", 0, "run", "records", 0, "corrections"])), json!(4)),
            set(rb(json!(["cases", 0, "selection", "corrections"])), json!(4)),
        ]),
        gate("G5", ATTEMPT)
    );
    // A dangling class-1 reference of an ATTEMPT check (a group's call) is ATTEMPT.
    assert_eq!(
        probe(&shared, "two_case_synthetic", vec![
            set(rb(json!(["calls", 0, "invocation_before"])), json!(1)),
            set(rb(json!(["groups", 0, "call"])), json!(3)),
        ]),
        gate("G5", ATTEMPT)
    );
    // The WORK defect alone still reports WORK at the end of class 1.
    assert_eq!(
        probe(&shared, ORD, vec![set(rb(json!(["calls", 0, "invocation_before"])), json!(1))]),
        gate("G5", "RETAINED_PRECISION_WORK_MISMATCH")
    );
}

/// D4 a-e: association relations (RV78 R4, R5, R6a; RV80 PR5).
#[test]
fn d4_association_relations() {
    use serde_json::json;
    let shared = corpus();
    let mut misses = Vec::new();
    let f = base_source(&shared, F_BASE);
    let attempt0 = f["retained_precision"]["body"]["product_attempts"][0].clone();
    for (name, base, edits) in [
        ("D4a source back-reference foreign (R4)", F_BASE, vec![set(rb(json!(["sources", 1, "preparation", "attempt_ref"])), json!(0))]),
        ("D4a source preparation null", F_BASE, vec![set(rb(json!(["sources", 1, "preparation"])), json!(null))]),
        ("D4b basis not the ordinary's (R5)", "two_case_two_groups_synthetic", vec![
            set(rb(json!(["product_attempts", 1, "material_basis_ref"])), json!(0)),
            set(rb(json!(["sources", 1, "material_basis_ref"])), json!(0)),
        ]),
        ("D4c cause without its own attempt (RV81-B2)", F_BASE, vec![
            set(rb(json!(["product_attempts"])), json!([attempt0])),
            set(rb(json!(["cases", 1, "product_attempt_ref"])), json!(null)),
        ]),
        ("D4d native with a selected Run (R6a)", F_BASE, vec![set(rb(json!(["product_attempts", 1, "result", "error"])), json!({"kind":"native","run_ref":1}))]),
        ("D4d native with a foreign run_ref (PR5)", F_BASE, vec![set(rb(json!(["product_attempts", 1, "result", "error"])), json!({"kind":"native","run_ref":0}))]),
        ("D4e run_ref without a native call", P_BASE, vec![set(rb(json!(["product_attempts", 1, "run_ref"])), json!(0))]),
    ] {
        let got = probe(&shared, base, edits);
        if got != gate("G5", PRODUCT) {
            misses.push(format!("{name}: got {got}"));
        }
    }
    assert!(misses.is_empty(), "{}", misses.join("\n"));
}

/// D5 a-e: native record relations (RV78 T1, T2, T3, R8).
#[test]
fn d5_native_record_relations() {
    use serde_json::json;
    let shared = corpus();
    let mut misses = Vec::new();
    let verification_failed = json!({"kind":"rejected","reason":{"space":"attempt","tag":"verification_failed"}});
    for (name, base, edits) in [
        ("D5a candidate with a verification (T3)", ORD, vec![set(rb(json!(["cases", 0, "run", "records", 0, "verification"])), shared["cases"][0]["source"]["retained_precision"]["body"]["cases"][0]["run"]["records"][1]["verification"].clone())]),
        ("D5a candidate with verification-pass work", ORD, vec![set(rb(json!(["cases", 0, "run", "records", 0, "work", "verification_lme"])), json!(1))]),
        ("D5b escalating failed verification shows a pass (T1)", "verification_failure_skip_synthetic", vec![set(rb(json!(["cases", 0, "run", "records", 1, "work", "verification_lme"])), json!(1))]),
        ("D5c verification_failed with a completed phase", ORD, vec![
            set(rb(json!(["cases", 0, "run", "attempts", 0, "outcome"])), verification_failed.clone()),
            set(rb(json!(["cases", 0, "run", "records", 0, "outcome"])), verification_failed.clone()),
        ]),
        ("D5d stop-rule quantity of another body (T2)", "p512_ladder_synthetic", vec![
            set(rb(json!(["cases", 0, "run", "records", 0, "outcome", "reason", "quantity"])), json!({"tag":"displacement","dof":{"node":3,"component":"UX"}})),
            set(rb(json!(["cases", 0, "run", "attempts", 0, "outcome", "reason", "quantity"])), json!({"tag":"displacement","dof":{"node":3,"component":"UX"}})),
        ]),
        ("D5e group call out of range (R8)", "two_case_synthetic", vec![set(rb(json!(["groups", 0, "call"])), json!(3))]),
    ] {
        let got = probe(&shared, base, edits);
        if got != gate("G5", ATTEMPT) {
            misses.push(format!("{name}: got {got}"));
        }
    }
    assert!(misses.is_empty(), "{}", misses.join("\n"));
}

/// D6b (checks_passed part), D6c, D6d and D7.
#[test]
fn d6_d7_ordinary_and_diagnostic_relations() {
    use serde_json::json;
    let shared = corpus();
    // D6b (RV78 T4e): a selected case with checks_passed ordinary quality.
    assert_eq!(
        probe(&shared, ORD, vec![
            set(json!(["numerical_quality", "cases", 0, "solve_quality"]), json!("checks_passed")),
            set(rb(json!(["ordinary_attempts", 0, "initial", "outcome"])), json!("checks_passed")),
        ]),
        gate("G5", ATTEMPT)
    );
    // D6b (checkpoint A): not_assessed does not route to retained precision.
    assert_eq!(
        probe(&shared, ORD, vec![set(json!(["numerical_quality", "cases", 0, "solve_quality"]), json!("not_assessed"))]),
        gate("G5", ATTEMPT)
    );
    // D6a as amended by F5 (D-U6-7; A2): the untyped list is exactly the
    // diagnostics naming the case, in envelope order, excluding
    // RETAINED_PRECISION_*. Checkpoint A's relaxed form (a listed diagnostic of
    // another scope is admitted) is refused since snapshot 07g.
    let other = base_source(&shared, ORD)["diagnostics"]
        .as_array()
        .unwrap()
        .iter()
        .find(|d| {
            !d["affected_refs"]
                .as_array()
                .is_some_and(|a| a.contains(&json!("case:six-component-load")))
        })
        .map(|d| d["id"].clone());
    {
        let other = other.expect("a diagnostic of another scope in the base");
        // RV90 N3 (07h): the base's exact list plus the other-scope element, so
        // that element is the only defect (`[integrity, other]` was also 07f's
        // relaxed form, refused under F5 without `other`).
        let mut refs = base_source(&shared, ORD)["retained_precision"]["body"]["ordinary_attempts"][0]["diagnostic_refs"]
            .as_array()
            .unwrap()
            .clone();
        assert!(!refs.contains(&other));
        refs.push(other);
        assert_eq!(
            probe(&shared, ORD, vec![set(rb(json!(["ordinary_attempts", 0, "diagnostic_refs"])), Value::Array(refs))]),
            gate("G5", ATTEMPT),
            "F5: a listed diagnostic of another scope is refused"
        );
        assert_eq!(probe(&shared, ORD, vec![]), Value::Null, "F5: the repaired base's exact list is admitted");
    }
    for (name, refs) in [
        ("duplicate (RV78 T4a)", json!(["diagnostic:numerical-integrity:case:six-component-load", "diagnostic:numerical-integrity:case:six-component-load"])),
        ("dangling (RV78 T4b)", json!(["diagnostic:numerical-integrity:case:six-component-load", "diagnostic:rv78:absent"])),
    ] {
        assert_eq!(
            probe(&shared, ORD, vec![set(rb(json!(["ordinary_attempts", 0, "diagnostic_refs"])), refs)]),
            gate("G5", ATTEMPT),
            "{name}"
        );
    }
    // D6c: a published W2 preserving a Formation initial failure is admitted
    // with a nonzero exponent b and rejected with b = 0 (C2:158).
    let trigger_error = json!({"tag":"numerical_range","name":"synthetic"});
    let w2 = |b: i64| {
        vec![
            set(rb(json!(["ordinary_attempts", 0, "initial"])), json!({"kind":"formation_failure","error":trigger_error,"basis_index":0})),
            set(rb(json!(["ordinary_attempts", 0, "w2"])), json!({"kind":"published","trigger":{"tag":"formation","error":trigger_error},"force_scale_exponent":b,"report_diagnostic_ref":"diagnostic:numerical-integrity:case:six-component-load"})),
        ]
    };
    assert_eq!(probe(&shared, ORD, w2(1)), Value::Null, "W2 published, b = 1");
    assert_eq!(probe(&shared, ORD, w2(0)), gate("G5", ATTEMPT), "W2 published, b = 0");
    // RV78 T-4e/T4d: ordinary (class 2) precedes the deferred C3 work list.
    assert_eq!(
        probe(&shared, F_BASE, vec![
            set(rb(json!(["ordinary_attempts", 1, "diagnostic_refs"])), json!(["diagnostic:numerical-integrity:case:unavailable-row", "diagnostic:rv78:absent"])),
            set(rb(json!(["product_attempts", 1, "adapter", "fault"])), json!({"kind":"overflow","event":"allocation_request"})),
        ]),
        gate("G5", ATTEMPT)
    );
    // D6d: a legacy work_ref that does not resolve is a reference check (ATTEMPT).
    assert_eq!(
        probe(&shared, ORD, vec![set(rb(json!(["ordinary_attempts", 0, "legacy_source", "work_ref"])), json!(0))]),
        gate("G5", ATTEMPT)
    );
    // D7: a RETAINED_PRECISION_SELECTED diagnostic naming no requested case.
    let diagnostics = base_source(&shared, ORD)["diagnostics"].clone();
    let i = diagnostics
        .as_array()
        .unwrap()
        .iter()
        .position(|d| d["code"] == "RETAINED_PRECISION_SELECTED")
        .unwrap();
    assert_eq!(
        probe(&shared, ORD, vec![set(json!(["diagnostics", i, "affected_refs"]), json!(["case:absent"]))]),
        gate("G4", "RETAINED_PRECISION_DIAGNOSTIC_MISMATCH")
    );
}

/// D13 reader-local pins, and kills for RV80's surviving mutants where the
/// rule is implemented (M07 by d1 above, M12, M13-M15 by d4 above, M08, M16,
/// M18).
#[test]
fn d13_reader_local_pins_and_mutant_kills() {
    use serde_json::json;
    let shared = corpus();
    // theta = +0 on a no-data body (selection and record theta both moved).
    assert_eq!(
        probe(&shared, "ordinary_prepared_no_data_synthetic", vec![
            set(rb(json!(["cases", 0, "selection", "theta", 0, "value"])), json!("3fd0000000000000")),
            set(rb(json!(["cases", 0, "run", "records", 1, "verification", "theta", 0, "value"])), json!("3fd0000000000000")),
        ]),
        gate("G5a", "RETAINED_PRECISION_SCALE_MISMATCH")
    );
    // M12 (RV80 PR3): an unavailable attempt's record bound must follow has_data.
    assert_eq!(
        probe(&shared, F_BASE, vec![set(rb(json!(["cases", 1, "run", "records", 1, "verification", "bound", 0, "value"])), json!(null))]),
        gate("G5a", "RETAINED_PRECISION_SCALE_MISMATCH")
    );
    // The Ceiling after a p128 verification-solve failure: v256 fails with an
    // escalating stop (two slots), the fresh p512 candidate is rejected and its
    // v1024 only solved.
    let schedule = rp::reader_logic::schedule;
    let mut run = base_source(&shared, "verification_failure_skip_synthetic")["retained_precision"]["body"]["cases"][0]["run"].clone();
    assert!(schedule(&run).is_ok());
    let stop_rule = json!({"kind":"rejected","reason":{"space":"attempt","tag":"stop_rule","quantity":{"tag":"displacement","dof":{"node":1,"component":"UX"}},"body":0,"kind":"translation"}});
    run["attempts"][1]["outcome"] = stop_rule.clone();
    run["records"][2]["outcome"] = stop_rule;
    run["records"][3]["outcome"] = json!({"kind":"solved"});
    run["kernel_terminal"] = json!({"kind":"unresolved","reason":{"space":"unresolved","tag":"ceiling"}});
    assert!(schedule(&run).is_ok(), "Ceiling after a p128 verification-solve failure");
    run["kernel_terminal"] = json!({"kind":"refused","reason":{"space":"refusal","tag":"structure"}});
    assert!(attempt_mismatch(schedule(&run)));
    // M16: a terminal Budget stop ends on its exact translation, scope included.
    let mut budget = base_source(&shared, ORD)["retained_precision"]["body"]["cases"][0]["run"].clone();
    let stop = json!({"space":"attempt","tag":"stop","stop":{"space":"stop","tag":"budget","scope":"case"}});
    budget["records"] = json!([budget["records"][0].clone()]);
    budget["records"][0]["outcome"] = json!({"kind":"failed","reason":stop});
    budget["attempts"][0]["outcome"] = json!({"kind":"failed","reason":stop});
    budget["attempts"][0]["verification"] = json!(null);
    budget["kernel_terminal"] = json!({"kind":"unresolved","reason":{"space":"unresolved","tag":"budget","scope":"case"}});
    assert!(schedule(&budget).is_ok());
    budget["kernel_terminal"]["reason"]["scope"] = json!("invocation");
    assert!(attempt_mismatch(schedule(&budget)), "Budget scope is part of the translation");
    // M08: an idle group-null Run is Budget(invocation) exactly at exhaustion.
    let body = json!({"work":{"invocation_limit":100},"groups":[]});
    let idle = |before: u64| {
        json!({"records":[],"attempts":[],"case_charge":0,"invocation_increment":0,
               "invocation_before":before,"origin":{"group":null},
               "kernel_terminal":{"kind":"unresolved","reason":{"space":"unresolved","tag":"budget","scope":"invocation"}}})
    };
    assert!(rp::reader_logic::schedule_in(&idle(100), &body).is_ok());
    assert!(attempt_mismatch(rp::reader_logic::schedule_in(&idle(99), &body)));
    // R3' with `both`: both faults must be emitted by the cause's owner (here
    // the ProofTrace, for a proof cause).
    let attempt = |proof: Value| {
        json!({"adapter":{"fault":null},
               "result":{"kind":"unavailable","error":{"kind":"proof","cause":{"kind":"work_accounting","fault":"both"}}},
               "proof":proof})
    };
    assert_eq!(
        rp::reader_logic::accounting(&attempt(json!({"numeric":{"kind":"unavailable","fault":"overflow"}}))),
        [true, true, false, true]
    );
    assert_eq!(
        rp::reader_logic::accounting(&attempt(json!({"numeric":{"kind":"unavailable","fault":"overflow"},"sticky_status":"inconsistent"}))),
        [true, true, true, true]
    );
    // R3' binds to the owner: a fault emitted elsewhere in the attempt does not count.
    let mut elsewhere = attempt(json!({"numeric":{"kind":"exact","value":0}}));
    elsewhere["overlay_work"] = json!({"sticky_status":"both"});
    assert_eq!(rp::reader_logic::accounting(&elsewhere), [true, true, false, true]);
    // R1' and R2' (lost; OperationalError accounting on an old operational entry).
    assert_eq!(
        rp::reader_logic::accounting(&json!({"adapter":{"fault":null},"x":{"kind":"accounting","event":"map_write"},"y":{"lost":true}})),
        [false, false, true, true]
    );
    assert_eq!(
        rp::reader_logic::accounting(&json!({"adapter":{"fault":null},"operational":{"old":[{"result":{"kind":"refused","error":{"kind":"accounting"}}}],"new":[]}})),
        [true, false, true, true]
    );
    // R4: a SectionError accounting needs a non-exact status in its member's work.
    let member = |work: Value| {
        json!({"adapter":{"fault":null},"preparation":{"members":[{"result":{"kind":"refused","error":{"kind":"accounting"}},"work":work}]}})
    };
    assert_eq!(rp::reader_logic::accounting(&member(json!({"sticky_status":"exact"}))), [true, true, true, false]);
    assert_eq!(rp::reader_logic::accounting(&member(json!({"sticky_status":"overflow"}))), [true, true, true, true]);
    // M18: G7 keeps the bare base code and carries any further text as detail.
    let e = rp::reader_logic::g7_error("SOURCE_PREVIEW_PHYSICS_ROW_SIGNATURE: bad row");
    assert_eq!((e.gate, e.code.as_str()), ("G7", "SOURCE_PREVIEW_PHYSICS_ROW_SIGNATURE"));
    assert_eq!(e.detail.as_deref(), Some("SOURCE_PREVIEW_PHYSICS_ROW_SIGNATURE: bad row"));
    let e = rp::reader_logic::g7_error("SOURCE_PREVIEW_PHYSICS_EXTREMA_BOUNDS");
    assert_eq!((e.code.as_str(), e.detail), ("SOURCE_PREVIEW_PHYSICS_EXTREMA_BOUNDS", None));
}

/// D13: the 2^-988 switch in the absolute bound, on both sides. Expected bits
/// from an exact rational oracle (least binary64 at or above the exact value).
#[test]
fn d13_absolute_bound_small_scale_switch() {
    for (scale, value, expected) in [
        (0x0230000000000000u64, 0x0000000000000000u64, 0x0000000000400000u64),
        (0x0230000000000000, 0x3ff0000000000000, 0x0000000000400000),
        (0x0230000000000000, 0x4330000000000000, 0x0000000000400000),
        (0x022fffffffffffff, 0x0000000000000000, 0x0000000000400001),
        (0x022fffffffffffff, 0x3ff0000000000000, 0x3ca0000000000001),
        (0x022fffffffffffff, 0x4330000000000000, 0x3fe0000000000001),
    ] {
        assert_eq!(
            rp::absolute_bound(f64::from_bits(value), f64::from_bits(scale))
                .unwrap()
                .to_bits(),
            expected,
            "{scale:016x} {value:016x}"
        );
    }
}

/// D8 kernel scope (checkpoint A; C1:66-68): a work_accounting stop anywhere in
/// a Run, a build or a group preparation is a class-1 ATTEMPT defect, ahead of
/// the deferred native WORK checks (a failed build reason here would be WORK).
#[test]
fn d8_kernel_scope_work_accounting_anywhere() {
    use serde_json::json;
    let shared = corpus();
    let wa = json!({"space":"stop","tag":"work_accounting","fault":"overflow"});
    for (name, edits) in [
        ("build reason", vec![set(rb(json!(["builds", 0, "reason"])), wa.clone())]),
        ("record outcome", vec![
            set(rb(json!(["cases", 0, "run", "records", 0, "outcome"])), json!({"kind":"failed","reason":{"space":"attempt","tag":"stop","stop":wa}})),
        ]),
    ] {
        assert_eq!(probe(&shared, ORD, edits), gate("G5", ATTEMPT), "{name}");
    }
}

/// D8 R2' and R4 on shared bases (also pinned by 07's shared mutations).
#[test]
fn d8_accounting_rules_on_shared_bases() {
    use serde_json::json;
    let shared = corpus();
    assert_eq!(
        probe(&shared, P_BASE, vec![set(rb(json!(["product_attempts", 1, "operational", "old", 0, "result"])), json!({"kind":"refused","error":{"kind":"accounting"}}))]),
        gate("G5", "RETAINED_PRECISION_WORK_MISMATCH"),
        "R2' old operational accounting without a lost trace"
    );
}

// ---------------------------------------------------------------------------
// Confirmation repair round (ROOT rulings D19-D30): reader-local tests.

/// D19: the converse of D4c. An unavailable C3 attempt needs a
/// prepared_product_failure cause naming it (RV79-C1); a Ready attempt's case is
/// selected or unavailable with receipt_failure.
#[test]
fn d19_converse_cause_binding() {
    use serde_json::json;
    let shared = corpus();
    let preparation_error = base_source(&shared, P_BASE)["retained_precision"]["body"]["product_attempts"][1]["result"]["error"].clone();
    let receipt_failure = json!({"kind":"receipt_failure","check":"association","field_path":"retained_precision.body"});
    let precondition = json!({"kind":"unavailable_precondition","precondition":"capture","affected_refs":[]});
    for (name, base, edits) in [
        ("unavailable attempt under receipt_failure", F_BASE, vec![set(rb(json!(["cases", 1, "reason", "cause"])), receipt_failure.clone())]),
        ("unavailable attempt under unavailable_precondition", F_BASE, vec![set(rb(json!(["cases", 1, "reason", "cause"])), precondition.clone())]),
        ("preparation error with a selected Run under receipt_failure (RV79-B1c)", F_BASE, vec![
            set(rb(json!(["cases", 1, "reason", "cause"])), receipt_failure.clone()),
            set(rb(json!(["product_attempts", 1, "result", "error"])), preparation_error.clone()),
        ]),
        ("P' preparation failure under unavailable_precondition", P_BASE, vec![set(rb(json!(["cases", 1, "reason", "cause"])), precondition.clone())]),
    ] {
        assert_eq!(probe(&shared, base, edits), gate("G5", PRODUCT), "{name}");
    }
    // A Ready attempt in an unavailable case: receipt_failure only.
    let unavailable_case = |cause: Value| {
        vec![
            remove(rb(json!(["cases", 1, "method"]))),
            remove(rb(json!(["cases", 1, "selection"]))),
            remove(rb(json!(["cases", 1, "source_identity_sha256"]))),
            set(rb(json!(["cases", 1, "status"])), json!("unavailable")),
            set(rb(json!(["cases", 1, "reason"])), json!({"code":"facade_certificate","phase":"facade","cause":cause})),
            set(rb(json!(["cases", 1, "diagnostic_ref"])), json!("diagnostic:retained:synthetic-zero-load")),
            set(json!(["diagnostics", base_source(&shared, "two_case_synthetic")["diagnostics"].as_array().unwrap().iter().position(|d| d["id"] == "diagnostic:retained:synthetic-zero-load").unwrap(), "code"]), json!("RETAINED_PRECISION_UNAVAILABLE")),
        ]
    };
    assert_eq!(probe(&shared, "two_case_synthetic", unavailable_case(precondition)), gate("G5", PRODUCT), "Ready attempt under a C2 cause");
    // With receipt_failure the D19 relation holds; the next defect is the
    // unavailable case's rows still carrying a recovery method (G6).
    assert_eq!(probe(&shared, "two_case_synthetic", unavailable_case(receipt_failure)), gate("G6", "RETAINED_PRECISION_ROW_METHOD_MISMATCH"), "Ready attempt under receipt_failure");
}

/// D20: a selected case without a C3 attempt is a C3 association defect
/// (PRODUCT_ATTEMPT), split out of the ordinary check.
#[test]
fn d20_selected_without_c3_attempt() {
    use serde_json::json;
    let shared = corpus();
    assert_eq!(
        probe(&shared, ORD, vec![
            set(rb(json!(["product_attempts"])), json!([])),
            set(rb(json!(["cases", 0, "product_attempt_ref"])), json!(null)),
        ]),
        gate("G5", PRODUCT)
    );
}

/// D21: a verification shared build on an escalating failed verification is
/// evidence that the verification pass ran (RV79-C2).
#[test]
fn d21_verification_shared_build_is_pass_evidence() {
    use serde_json::json;
    let shared = corpus();
    assert_eq!(
        probe(&shared, "verification_failure_skip_synthetic", vec![set(rb(json!(["cases", 0, "run", "records", 1, "verification_shared_build_ref"])), json!(0))]),
        gate("G5", ATTEMPT)
    );
}

/// D22: a dangling attempt source_ref is G5 PRODUCT_ATTEMPT; the dependent G3
/// checks are skipped.
#[test]
fn d22_dangling_attempt_source_ref() {
    use serde_json::json;
    let shared = corpus();
    assert_eq!(
        probe(&shared, F_BASE, vec![set(rb(json!(["product_attempts", 1, "source_ref"])), json!(9))]),
        gate("G5", PRODUCT)
    );
}

/// D24: the harness applies `after_rehash` edits after the rehash; forged
/// receipt and publication hashes give G1.
#[test]
fn d24_after_rehash_edits() {
    use serde_json::json;
    let shared = corpus();
    for (name, path) in [
        ("receipt hash", json!(["retained_precision", "receipt_sha256"])),
        ("publication hash", rb(json!(["publication_sha256"]))),
    ] {
        let entry = json!({"id":"i63_after_rehash","base":ORD,"edits":[],"rehash":"all",
            "after_rehash":[{"path":path,"op":"set","value":"0".repeat(64)}]});
        assert_eq!(observe(&shared, &entry), gate("G1", "RETAINED_PRECISION_RECEIPT_MISMATCH"), "{name}");
    }
    let entry = json!({"id":"i63_after_rehash_none","base":ORD,"edits":[],"rehash":"all","after_rehash":[]});
    assert_eq!(observe(&shared, &entry), Value::Null);
}

/// D25: readers validate parsed values; an integral float counter is the same
/// number as the integer (I-JSON/JCS), so the receipt still validates.
#[test]
fn d25_integral_float_is_the_same_number() {
    use serde_json::json;
    let shared = corpus();
    assert_eq!(
        probe(&shared, ORD, vec![
            set(rb(json!(["cases", 0, "run", "case_charge"])), json!(17.0)),
            set(rb(json!(["work", "charged"])), json!(17.0)),
        ]),
        Value::Null
    );
}

/// D27: the idle (group-null) Run rule reads the recorded invocation_before;
/// a broken meter chain is native WORK (RV80 PR14).
#[test]
fn d27_idle_rule_reads_recorded_invocation_before() {
    use serde_json::json;
    let shared = corpus();
    let run = |k: &str| rb(json!(["cases", 1, "run", k]));
    assert_eq!(
        probe(&shared, F_BASE, vec![
            set(run("records"), json!([])),
            set(run("attempts"), json!([])),
            set(run("case_charge"), json!(0)),
            set(run("invocation_increment"), json!(0)),
            set(run("invocation_before"), json!(60000000000u64)),
            set(run("invocation_after"), json!(60000000000u64)),
            set(run("cache_before"), json!([])),
            set(run("cache_after"), json!([])),
            set(rb(json!(["cases", 1, "run", "origin", "group"])), json!(null)),
            set(rb(json!(["cases", 1, "run", "kernel_terminal"])), json!({"kind":"unresolved","reason":{"space":"unresolved","tag":"budget","scope":"invocation"}})),
            set(rb(json!(["groups", 0, "source_refs"])), json!([0])),
        ]),
        gate("G5", "RETAINED_PRECISION_WORK_MISMATCH")
    );
}

/// D28: every quantity-bearing attempt reason resolves to a layout row with the
/// same body and kind (RV80 PR12).
#[test]
fn d28_quantity_reasons_resolve_to_layout() {
    use serde_json::json;
    let shared = corpus();
    let mut misses = Vec::new();
    for (tag, extra) in [
        ("verification_estimate", json!({})),
        ("charge", json!({})),
        ("publication_enclosure", json!({"predicate":"absolute_bound"})),
    ] {
        let mut reason = json!({"space":"attempt","tag":tag,"quantity":{"tag":"displacement","dof":{"node":99,"component":"UX"}},"body":0,"kind":"translation"});
        for (k, v) in extra.as_object().unwrap() {
            reason[k] = v.clone();
        }
        let got = probe(&shared, "p512_ladder_synthetic", vec![
            set(rb(json!(["cases", 0, "run", "records", 0, "outcome", "reason"])), reason.clone()),
            set(rb(json!(["cases", 0, "run", "attempts", 0, "outcome", "reason"])), reason),
        ]);
        if got != gate("G5", ATTEMPT) {
            misses.push(format!("{tag}: got {got}"));
        }
    }
    assert!(misses.is_empty(), "{}", misses.join("\n"));
}

/// D29: an empty body inventory under a coverage roster fails G3 (RV80 PR11).
#[test]
fn d29_empty_body_inventory() {
    use serde_json::json;
    let shared = corpus();
    assert_eq!(
        probe(&shared, ORD, vec![
            set(rb(json!(["sources", 0, "body_membership"])), json!([])),
            set(rb(json!(["product_attempts", 0, "proof", "summary_coverage"])), json!([])),
        ]),
        gate("G3", COVERAGE)
    );
}

/// D29: any CaseSource with an empty body inventory fails G3, with or without
/// a coverage roster (here on cert_failed_before_summary, coverage null).
#[test]
fn d29_empty_body_inventory_without_roster() {
    use serde_json::json;
    let shared = corpus();
    let mut entry = shared["must_pass"]
        .as_array()
        .unwrap()
        .iter()
        .find(|m| m["id"] == "cert_failed_before_summary")
        .unwrap()
        .clone();
    assert_eq!(observe(&shared, &entry), Value::Null);
    entry["edits"]
        .as_array_mut()
        .unwrap()
        .push(set(rb(json!(["sources", 1, "body_membership"])), json!([])));
    assert_eq!(observe(&shared, &entry), gate("G3", COVERAGE));
}

/// D30: the native run_ref on a nonselected Run (kills M13; no shared base has
/// a nonselected native Run).
#[test]
fn d30_native_run_ref_on_nonselected_run() {
    use serde_json::json;
    let case = |run_id: u64| {
        json!({"status":"unavailable","reason":{"code":"kernel_unresolved","phase":"kernel","cause":{"kind":"prepared_product_failure","product_attempt_ref":0}},
               "run":{"id":run_id,"kernel_terminal":{"kind":"unresolved","reason":{"space":"unresolved","tag":"ceiling"}}}})
    };
    let attempt = json!({"result":{"kind":"unavailable","error":{"kind":"native","run_ref":3}},"stages":{"native":"failed"},"proof":null});
    assert!(rp::reader_logic::reason_table(&case(3), &attempt).is_ok());
    let got = rp::reader_logic::reason_table(&case(2), &attempt).unwrap_err();
    assert_eq!((got.gate, got.code.as_str()), ("G5", PRODUCT));
}

/// D31: G8 admits model schema_version 0.1.0, 0.2.0 or 0.3.0; 0.4.0 stays
/// excluded. The invocation edit rebinds the receipt's invocation digest.
#[test]
fn d31_model_schema_versions_at_g8() {
    use serde_json::json;
    let shared = corpus();
    let entry = |version: &str| {
        json!({"id":"i63_d31","base":ORD,"edits":[],"rehash":"all",
            "invocation_edits":[{"path":["request","model","schema_version"],"op":"set","value":version}]})
    };
    let mut misses = Vec::new();
    for (version, want) in [
        ("0.1.0", Value::Null),
        ("0.2.0", Value::Null),
        ("0.3.0", Value::Null),
        ("0.4.0", gate("G8", "RETAINED_PRECISION_INVOCATION_MISMATCH")),
        ("0.0.9", gate("G8", "RETAINED_PRECISION_INVOCATION_MISMATCH")),
    ] {
        let got = observe(&shared, &entry(version));
        if got != want {
            misses.push(format!("{version}: got {got}, want {want}"));
        }
    }
    assert!(misses.is_empty(), "{}", misses.join("\n"));
}

/// Every integer in `v` rewritten as the integral float of the same value.
fn as_integral_floats(v: &mut Value) {
    match v {
        Value::Number(n) if !n.is_f64() => *v = Value::from(n.as_f64().unwrap()),
        Value::Array(a) => a.iter_mut().for_each(as_integral_floats),
        Value::Object(o) => o.values_mut().for_each(as_integral_floats),
        _ => {}
    }
}

/// D32: integers by value everywhere. G0's receipt_version and work limits,
/// references and every other receipt integer written as integral floats
/// validate; a non-integral, -0 or wrong value still fails where it did.
#[test]
fn d32_integers_by_value() {
    use serde_json::json;
    let shared = corpus();
    let g0 = gate("G0", UNSUPPORTED);
    let mut misses = Vec::new();
    for (name, edits, want) in [
        ("receipt_version 1.0", vec![set(rb(json!(["receipt_version"])), json!(1.0))], Value::Null),
        ("float limits", vec![
            set(rb(json!(["work", "case_limit"])), json!(20000000000.0)),
            set(rb(json!(["work", "invocation_limit"])), json!(60000000000.0)),
        ], Value::Null),
        ("float source_ref", vec![set(rb(json!(["cases", 0, "source_ref"])), json!(0.0))], Value::Null),
        ("float attempt refs", vec![
            set(rb(json!(["cases", 0, "product_attempt_ref"])), json!(0.0)),
            set(rb(json!(["sources", 0, "preparation", "attempt_ref"])), json!(0.0)),
        ], Value::Null),
        ("float quality binding", vec![set(rb(json!(["cases", 0, "ordinary", "quality_binding", "index"])), json!(0.0))], Value::Null),
        ("receipt_version 1.5", vec![set(rb(json!(["receipt_version"])), json!(1.5))], g0.clone()),
        ("receipt_version -0", vec![set(rb(json!(["receipt_version"])), json!(-0.0))], g0.clone()),
        ("receipt_version 2.0", vec![set(rb(json!(["receipt_version"])), json!(2.0))], g0.clone()),
        ("case_limit 20000000000.5", vec![set(rb(json!(["work", "case_limit"])), json!(20000000000.5))], g0.clone()),
        ("invocation_limit 6e10+1", vec![set(rb(json!(["work", "invocation_limit"])), json!(60000000001.0))], g0.clone()),
        ("source_ref -0", vec![set(rb(json!(["cases", 0, "source_ref"])), json!(-0.0))], gate("G2", "RETAINED_PRECISION_ENCODING_MISMATCH")),
        ("source_ref 0.5", vec![set(rb(json!(["cases", 0, "source_ref"])), json!(0.5))], gate("G2", "RETAINED_PRECISION_ENCODING_MISMATCH")),
    ] {
        let got = probe(&shared, ORD, edits);
        if got != want {
            misses.push(format!("{name}: got {got}, want {want}"));
        }
    }
    assert!(misses.is_empty(), "{}", misses.join("\n"));
    // Every receipt integer at once, on every complete base.
    for case in shared["cases"].as_array().unwrap() {
        let mut source = case["source"].clone();
        as_integral_floats(&mut source["retained_precision"]["body"]);
        rehash(&mut source);
        assert!(
            rp::validate(&source, Some(&case["invocation"])).is_ok(),
            "{}: {:?}",
            case["id"],
            rp::validate(&source, Some(&case["invocation"])).err()
        );
    }
}

/// D32 shared pin shape: a forged source identity under `source_ref: 0.0`
/// fails G1, exactly as under `source_ref: 0`.
#[test]
fn d32_forged_source_identity_under_float_ref() {
    use open_pipe_stress_result_export::source_blocks::domain_hash;
    use serde_json::json;
    let shared = corpus();
    for (name, forge) in [("control", false), ("forged", true)] {
        for source_ref in [json!(0), json!(0.0)] {
            let entry = json!({"id":"i63_d32_identity","base":ORD,"rehash":"all",
                "edits":[{"path":["retained_precision","body","cases",0,"source_ref"],"op":"set","value":source_ref}]});
            let (mut source, invocation) = apply_entry(&shared, &entry);
            if forge {
                // The digest of a different source statement, receipt rehashed.
                let mut other = source["retained_precision"]["body"]["sources"][0].clone();
                other.as_object_mut().unwrap().remove("index");
                other["stiffness_sha256"] = json!("0".repeat(64));
                source["retained_precision"]["body"]["cases"][0]["source_identity_sha256"] =
                    domain_hash("retained_precision_source_mp_v2", &other).unwrap().into();
                source["retained_precision"]["receipt_sha256"] = domain_hash(
                    "retained_precision_receipt_mp_v2",
                    &source["retained_precision"]["body"],
                )
                .unwrap()
                .into();
            }
            let got = match rp::validate(&source, Some(&invocation)) {
                Err(e) => json!({"gate":e.gate,"code":e.code}),
                Ok(_) => Value::Null,
            };
            let want = if forge { gate("G1", "RETAINED_PRECISION_RECEIPT_MISMATCH") } else { Value::Null };
            assert_eq!(got, want, "{name} source_ref {source_ref}");
        }
    }
}

/// D33 (RV80-N1, PR16): a verification_estimate reason names a Force or Moment
/// layout row; on an existing translation or rotation row it is G5 ATTEMPT.
/// `charge` is unrestricted.
#[test]
fn d33_verification_estimate_names_force_or_moment() {
    use serde_json::json;
    let shared = corpus();
    let mut misses = Vec::new();
    for (tag, quantity, kind, want) in [
        ("verification_estimate", json!({"tag":"displacement","dof":{"node":1,"component":"UX"}}), "translation", gate("G5", ATTEMPT)),
        ("verification_estimate", json!({"tag":"displacement","dof":{"node":1,"component":"RX"}}), "rotation", gate("G5", ATTEMPT)),
        ("verification_estimate", json!({"tag":"end_action","member":0,"end":"i","component":"UX"}), "force", Value::Null),
        ("verification_estimate", json!({"tag":"end_action","member":0,"end":"i","component":"RX"}), "moment", Value::Null),
        ("charge", json!({"tag":"displacement","dof":{"node":1,"component":"UX"}}), "translation", Value::Null),
    ] {
        let reason = json!({"space":"attempt","tag":tag,"quantity":quantity,"body":0,"kind":kind});
        let got = probe(&shared, "p512_ladder_synthetic", vec![
            set(rb(json!(["cases", 0, "run", "records", 0, "outcome", "reason"])), reason.clone()),
            set(rb(json!(["cases", 0, "run", "attempts", 0, "outcome", "reason"])), reason),
        ]);
        if got != want {
            misses.push(format!("{tag} on {kind}: got {got}, want {want}"));
        }
    }
    assert!(misses.is_empty(), "{}", misses.join("\n"));
}

/// RV80-N2 (kills M39): D21's last-slot case. A Ceiling-shaped Run whose last
/// attempt is a p256 candidate whose escalating v512 verification solve fails:
/// no fresh attempt follows, so the replay's own shared-build check never
/// runs and only the D21 record check catches a verification shared build.
/// The control's class-1 checks (ATTEMPT and WORK) all pass; it fails later,
/// because no Ceiling base with a native-faithful case exists (deferred).
#[test]
fn d21_last_slot_verification_shared_build() {
    use serde_json::json;
    let shared = corpus();
    let run = |tail: Value| {
        let mut p = json!(["cases", 0, "run"]);
        p.as_array_mut().unwrap().extend(tail.as_array().unwrap().iter().cloned());
        rb(p)
    };
    let stop = json!({"space":"attempt","tag":"stop","stop":{"space":"stop","tag":"condition"}});
    let rejected = json!({"kind":"rejected","reason":{"space":"attempt","tag":"verification_failed"}});
    let zero = |w: &mut Value| {
        for (_, v) in w.as_object_mut().unwrap() {
            *v = json!(0);
        }
    };
    let base = base_source(&shared, "p512_ladder_synthetic");
    let r = &base["retained_precision"]["body"]["cases"][0]["run"];
    let mut rec2 = r["records"][2].clone();
    rec2["role"] = json!("verification");
    rec2["outcome"] = json!({"kind":"failed","reason":stop});
    rec2["verification"] = Value::Null;
    rec2["verification_shared_build_ref"] = Value::Null;
    let w = &mut rec2["work"];
    zero(&mut w["own_stages"]);
    w["own_stages"]["solve"] = json!(3);
    w["wide_lme"] = json!(3);
    w["own_lme"] = json!(3);
    w["stop_rule_lme"] = json!(0);
    w["verification_lme"] = json!(0);
    w["verification_shared_lme"] = json!(0);
    w["verification_shared_built_here"] = json!(false);
    zero(&mut w["shared_stages"]);
    w["shared_stages"]["formation"] = json!(6);
    let mut att1 = r["attempts"][1].clone();
    att1["verification"] = json!({"record":2,"precision":512,"phase":"failed","reason":stop});
    att1["outcome"] = rejected.clone();
    att1["case_charge"] = json!(10);
    att1["invocation_increment"] = json!(10);
    let mut edits = vec![
        remove(run(json!(["records", 3]))),
        remove(run(json!(["attempts", 2]))),
        set(run(json!(["records", 2])), rec2),
        set(run(json!(["records", 1, "outcome"])), rejected),
        set(run(json!(["attempts", 1])), att1),
        set(run(json!(["kernel_terminal"])), json!({"kind":"unresolved","reason":{"space":"unresolved","tag":"ceiling"}})),
        set(run(json!(["cache_after"])), json!([{"slot":"s128","build":0},{"slot":"s256","build":1},{"slot":"s512","build":3},{"slot":"v256","build":2}])),
        set(run(json!(["case_charge"])), json!(27)),
        set(run(json!(["invocation_increment"])), json!(27)),
        set(run(json!(["invocation_after"])), json!(27)),
        set(rb(json!(["calls", 0, "invocation_after"])), json!(27)),
        set(rb(json!(["work", "charged"])), json!(27)),
        remove(rb(json!(["builds", 6]))),
        remove(rb(json!(["builds", 5]))),
        remove(rb(json!(["builds", 4]))),
    ];
    let control = probe(&shared, "p512_ladder_synthetic", edits.clone());
    assert!(
        control != gate("G5", ATTEMPT) && control != gate("G5", "RETAINED_PRECISION_WORK_MISMATCH"),
        "control must clear G5 class 1: {control}"
    );
    // The schedule replay alone accepts the Ceiling run with or without the build.
    edits.push(set(run(json!(["records", 2, "verification_shared_build_ref"])), json!(2)));
    let entry = json!({"id":"i63_d21_last_slot","base":"p512_ladder_synthetic","edits":edits,"rehash":"all"});
    let (mutated, _) = apply_entry(&shared, &entry);
    let replay = &mutated["retained_precision"]["body"]["cases"][0]["run"];
    assert!(rp::reader_logic::schedule(replay).is_ok());
    assert_eq!(observe(&shared, &entry), gate("G5", ATTEMPT));
}

/// RV80-N1 (M56): the same entry on the metadata-only transport path.
fn transport(shared: &Value, entry: &Value) -> Value {
    let (source, _) = apply_entry(shared, entry);
    match rp::validate_transport_metadata(&source) {
        Err(e) => serde_json::json!({"gate":e.gate,"code":e.code}),
        Ok(_) => serde_json::json!(null),
    }
}
fn transport_probe(shared: &Value, base: &str, edits: Vec<Value>) -> Value {
    transport(
        shared,
        &serde_json::json!({"id": "i63_transport_probe", "base": base, "edits": edits, "rehash": "all"}),
    )
}

/// D34 (RV79-N1): a JSON number equal to -0 anywhere in the receipt fails G2
/// ENCODING, including the integer fields the schema writes as enum or const
/// values, which no base carries: `G5aError.quantity_kind` (on F_BASE's
/// unavailable attempt) and `source_decline.constructor_counts.directional_springs`
/// (on the shared `unavailable_attempt_under_source_error_cause` entry). Each
/// control with +0 passes G1 and G2 and fails where it did.
#[test]
fn d34_negative_zero_anywhere_in_receipt_fails_g2() {
    use serde_json::json;
    let shared = corpus();
    let g2 = gate("G2", "RETAINED_PRECISION_ENCODING_MISMATCH");
    let mut misses = Vec::new();
    let mut check = |name: &str, got: Value, want: &Value| {
        if got != *want {
            misses.push(format!("{name}: got {got}, want {want}"));
        }
    };
    let error = rb(json!(["product_attempts", 1, "result", "error"]));
    for kind in ["sanity", "lower"] {
        let cause = |q: Value| {
            let mut c = json!({"kind":kind,"quantity_kind":q});
            c[if kind == "sanity" { "body" } else { "member" }] = json!(0);
            json!({"kind":"g5a","cause":c})
        };
        let control = probe(&shared, F_BASE, vec![set(error.clone(), cause(json!(0)))]);
        assert!(
            !matches!(control["gate"].as_str(), Some("G0" | "G1" | "G2")) && !control.is_null(),
            "{kind} control: {control}"
        );
        check(&format!("{kind} quantity_kind -0"), probe(&shared, F_BASE, vec![set(error.clone(), cause(json!(-0.0)))]), &g2);
        // RV80-N1 (M56): the transport path applies the same G2.
        check(&format!("{kind} quantity_kind -0 transport"), transport_probe(&shared, F_BASE, vec![set(error.clone(), cause(json!(-0.0)))]), &g2);
        let t = transport_probe(&shared, F_BASE, vec![set(error.clone(), cause(json!(0)))]);
        assert!(t != g2 && !matches!(t["gate"].as_str(), Some("G0" | "G1")), "{kind} transport control: {t}");
        check(&format!("{kind} quantity_kind 1"), probe(&shared, F_BASE, vec![set(error.clone(), cause(json!(1)))]), &control);
        // The JSON text "-0" parses as -0 and is caught the same way.
        let entry = json!({"id":"i63_d34_text","base":F_BASE,"rehash":"all","edits":[set(error.clone(), cause(json!(-0.0)))]});
        let (source, invocation) = apply_entry(&shared, &entry);
        let text = serde_json::to_string(&source).unwrap().replace("\"quantity_kind\":-0.0", "\"quantity_kind\":-0");
        assert!(text.contains("\"quantity_kind\":-0}") || text.contains("\"quantity_kind\":-0,"));
        let parsed: Value = serde_json::from_str(&text).unwrap();
        let got = rp::validate(&parsed, Some(&invocation)).unwrap_err();
        check(&format!("{kind} text -0"), json!({"gate":got.gate,"code":got.code}), &g2);
        let got = rp::validate(&parsed, None).unwrap_err();
        check(&format!("{kind} text -0 without invocation"), json!({"gate":got.gate,"code":got.code}), &g2);
        let got = rp::validate_transport_metadata(&parsed).unwrap_err();
        check(&format!("{kind} text -0 transport"), json!({"gate":got.gate,"code":got.code}), &g2);
    }
    let mut entry = shared["mutations"]
        .as_array()
        .unwrap()
        .iter()
        .find(|m| m["id"] == "unavailable_attempt_under_source_error_cause")
        .unwrap()
        .clone();
    check("source_decline control", observe(&shared, &entry), expected_for(&entry));
    entry["edits"][1]["value"]["constructor_counts"]["directional_springs"] = json!(-0.0);
    check("directional_springs -0", observe(&shared, &entry), &g2);
    check("directional_springs -0 transport", transport(&shared, &entry), &g2);
    entry["edits"][1]["value"]["constructor_counts"]["directional_springs"] = json!(1);
    check("directional_springs 1", observe(&shared, &entry), &gate("G1", "RETAINED_PRECISION_RECEIPT_MISMATCH"));
    // An encoded U field written as -0 stays G2.
    check(
        "case_charge -0",
        probe(&shared, ORD, vec![set(rb(json!(["cases", 0, "run", "case_charge"])), json!(-0.0))]),
        &g2,
    );
    check(
        "case_charge -0 transport",
        transport_probe(&shared, ORD, vec![set(rb(json!(["cases", 0, "run", "case_charge"])), json!(-0.0))]),
        &g2,
    );
    assert!(misses.is_empty(), "{}", misses.join("\n"));
}

/// RV78-N1 (07e format rule): the harness rehash indexes only a strict integral
/// value: a JSON number, never a boolean, finite, integral, >= 0 and not -0
/// (0.0 is index 0; 0.5, true and -0.0 are not). A reference that is not an
/// index, or does not resolve, is skipped and left for the reader to report.
#[test]
fn rehash_index_rule_07e() {
    use serde_json::json;
    for (v, want) in [(json!(0), 0), (json!(1), 1), (json!(1.0), 1), (json!(0.0), 0), (json!(7), 7)] {
        assert_eq!(index(&v), Some(want), "{v}");
    }
    for v in [json!(true), json!(false), json!(0.5), json!(-0.0), json!(-1), json!(-1.0), json!(null), json!("0"), json!([0]), json!({"index":0})] {
        assert_eq!(index(&v), None, "{v}");
    }
    // Unresolvable references are skipped: the rehash leaves the stated digest
    // for the reader, which reports the dangling or malformed reference.
    let shared = corpus();
    let mut source = base_source(&shared, ORD);
    let before = source["retained_precision"]["body"]["cases"][0]["source_identity_sha256"].clone();
    for r in [json!(true), json!(0.5), json!(-0.0), json!(9)] {
        source["retained_precision"]["body"]["cases"][0]["source_ref"] = r.clone();
        source["retained_precision"]["body"]["sources"][0]["preparation"]["attempt_ref"] = r.clone();
        rehash(&mut source);
        assert_eq!(source["retained_precision"]["body"]["cases"][0]["source_identity_sha256"], before, "{r}");
    }
}

/// D37 (D35 widened; RV78-S1/S2, RV79-X1): every product-attempt error kind
/// agrees with the stage record in both directions, at G5 PRODUCT_ATTEMPT.
/// Probes on F_BASE's unavailable attempt 1 (every pipeline stage completed,
/// certificate passed, observables and G5a not entered, a capture error), on
/// P_BASE's preparation failure, and on the must-pass shapes for the other
/// kinds. Each consistent shape stays clear of PRODUCT_ATTEMPT.
#[test]
fn d37_error_kind_agrees_with_stage_record() {
    use serde_json::json;
    let shared = corpus();
    let product = gate("G5", PRODUCT);
    let a1 = |k: &str| rb(json!(["product_attempts", 1, k]));
    let error = rb(json!(["product_attempts", 1, "result", "error"]));
    let storage = json!({"kind":"storage","detail":"adapter vector"});
    let proof_error = |k: &str| json!({"kind":k,"cause":{"kind":"storage"}});
    let g5a_error = json!({"kind":"g5a","cause":{"kind":"zero","row":0}});
    let observable_error = json!({"kind":"observable","cause":storage});
    let mut stages = base_source(&shared, F_BASE)["retained_precision"]["body"]["product_attempts"][1]["stages"].clone();
    let checks = |obs: Value, g: Value| json!({"certificate":{"kind":"passed"},"observables":obs,"g5a":g});
    let mut misses = Vec::new();
    let mut run = |name: &str, base: &str, edits: Vec<Value>, rejected: bool| {
        let got = probe(&shared, base, edits);
        if (got == product) != rejected {
            misses.push(format!("{name}: got {got}, rejected wanted {rejected}"));
        }
    };
    // The base shape (capture after a completed certificate) is consistent.
    run("capture after certificate (base)", F_BASE, vec![], false);
    // X1 / Y-direction: the kind presupposes stages the record does not show.
    run("g5a with G5a not entered", F_BASE, vec![set(error.clone(), g5a_error.clone())], true);
    run("observable with observables not entered", F_BASE, vec![set(error.clone(), observable_error.clone())], true);
    run("proof with certificate passed", F_BASE, vec![set(error.clone(), proof_error("proof"))], true);
    run("values with values completed", F_BASE, vec![set(error.clone(), json!({"kind":"values","cause":{"kind":"storage"},"proof":{"kind":"storage"}}))], true);
    run("numeric with checks not entered", F_BASE, vec![set(error.clone(), json!({"kind":"numeric","cause":null}))], true);
    run("abandoned with certificate completed", F_BASE, vec![set(error.clone(), json!({"kind":"abandoned","cause":storage,"proof":{"kind":"storage"}}))], true);
    // After a completed certificate both checks are entered.
    stages["observables"] = json!("completed");
    stages["g5a"] = json!("completed");
    let all_passed = vec![set(a1("stages"), stages.clone()), set(rb(json!(["product_attempts", 1, "proof", "checks"])), checks(json!({"kind":"passed"}), json!({"kind":"passed"})))];
    let with = |mut v: Vec<Value>, e: Value| {
        v.push(set(error.clone(), e));
        v
    };
    run("numeric with both checks passed", F_BASE, with(all_passed.clone(), json!({"kind":"numeric","cause":null})), false);
    run("capture at the commit (every stage completed)", F_BASE, with(all_passed.clone(), json!({"kind":"capture","cause":storage})), false);
    run("g5a with G5a passed", F_BASE, with(all_passed.clone(), g5a_error.clone()), true);
    run("observable with observables passed", F_BASE, with(all_passed.clone(), observable_error.clone()), true);
    let mut g5a_failed = stages.clone();
    g5a_failed["g5a"] = json!("failed");
    let g5a_shape = vec![set(a1("stages"), g5a_failed.clone()), set(rb(json!(["product_attempts", 1, "proof", "checks"])), checks(json!({"kind":"passed"}), json!({"kind":"failed","error":g5a_error})))];
    run("g5a with observables passed and G5a failed", F_BASE, with(g5a_shape.clone(), g5a_error.clone()), false);
    run("numeric with G5a failed", F_BASE, with(g5a_shape.clone(), json!({"kind":"numeric","cause":null})), true);
    run("capture with G5a failed", F_BASE, with(g5a_shape, json!({"kind":"capture","cause":storage})), true);
    let mut observable_failed = stages.clone();
    observable_failed["observables"] = json!("failed");
    let observable_shape = vec![set(a1("stages"), observable_failed.clone()), set(rb(json!(["product_attempts", 1, "proof", "checks"])), checks(json!({"kind":"failed","error":observable_error}), json!({"kind":"passed"})))];
    run("observable with observables failed", F_BASE, with(observable_shape.clone(), observable_error.clone()), false);
    run("g5a with observables failed", F_BASE, with(observable_shape.clone(), g5a_error.clone()), true);
    run("numeric with observables failed", F_BASE, with(observable_shape, json!({"kind":"numeric","cause":null})), true);
    // The other direction on a failed stage (P9): P_BASE's failed preparation.
    let p_error = rb(json!(["product_attempts", 1, "result", "error"]));
    run("preparation failure (base)", P_BASE, vec![], false);
    for (name, e) in [
        ("proof after failed preparation", proof_error("proof")),
        ("numeric after failed preparation", json!({"kind":"numeric","cause":null})),
        ("g5a after failed preparation", g5a_error.clone()),
        ("capture after failed preparation", json!({"kind":"capture","cause":{"kind":"storage","detail":"prepared vector"}})),
    ] {
        run(name, P_BASE, vec![set(p_error.clone(), e)], true);
    }
    assert!(misses.is_empty(), "{}", misses.join("\n"));
}

/// RV94 S-1 on 07j's two-case statement whose second case is not_required: with
/// the receipt's case order the standing is eligible and the summary Current;
/// with the not_required case omitted, or the two reordered, the standing is
/// `needs_recompute` and the summary not-Current ([73, 0]).
#[test]
fn u7_07j_not_required_case_omitted_or_reordered_is_not_current() {
    use open_pipe_stress_result_export::semantic_contract as s;
    let shared = corpus();
    let entry = shared["must_pass"].as_array().unwrap().iter().find(|e| e["id"] == "not_required_second_case_checks_passed").unwrap();
    let (source, invocation) = apply_entry(&shared, entry);
    let cases = source["retained_precision"]["body"]["cases"].as_array().unwrap();
    assert_eq!(cases.iter().map(|c| c["status"].as_str().unwrap()).collect::<Vec<_>>(), ["selected", "not_required"]);
    let order: Vec<Value> = cases.iter().map(|c| c["basis_ref"].clone()).collect();
    assert_eq!(s::numerical_use_standing_with_context(&source, &order, Some(&invocation)), "numerically_eligible");
    let withheld = |refs: &[Value]| -> Vec<u64> {
        s::classification_summary(&source, Some(&invocation), refs).iter().map(|c| c["withheld"].as_u64().unwrap()).collect()
    };
    let current = withheld(&order);
    assert_eq!(withheld(&[]), [73, 0]);
    assert_ne!(current, [73, 0]);
    let reversed: Vec<Value> = order.iter().rev().cloned().collect();
    for refs in [order[..1].to_vec(), reversed] {
        assert_eq!(s::numerical_use_standing_with_context(&source, &refs, Some(&invocation)), "needs_recompute", "{refs:?}");
        assert_eq!(withheld(&refs), [73, 0], "{refs:?}");
    }
}
