
// ---- I70 records-only probe (scratch copy only; never in the candidate) ----
// Lists every 07l case, mutation and must-pass entry against its expectation,
// with the same comparisons the contract tests make, one JSON line each.
fn i70_class_name(c: &rp::AccuracyClass) -> &'static str {
    match c {
        rp::AccuracyClass::RelativeVerified => "relative_verified",
        rp::AccuracyClass::AbsoluteVerified { .. } => "absolute_verified",
        rp::AccuracyClass::InputDerived => "input_derived",
        rp::AccuracyClass::NonQuantity => "non_quantity",
        rp::AccuracyClass::NotCovered => "not_covered",
    }
}
fn i70_rows_match(got: &[rp::RowClassification], want: &[Value]) -> bool {
    got.len() == want.len()
        && got.iter().zip(want).all(|(g, w)| {
            let bound_ok = match g.class {
                rp::AccuracyClass::AbsoluteVerified { bound_bits } => {
                    format!("{bound_bits:016x}") == w["bound_bits"]
                }
                _ => true,
            };
            bound_ok
                && g.result_id == w["result_id"]
                && g.basis_ref == w["basis_ref"]
                && format!("{:016x}", g.normalized_bits) == w["normalized_bits"]
                && g.scale_bits.map(|b| format!("{b:016x}"))
                    == w["scale_bits"].as_str().map(str::to_owned)
                && i70_class_name(&g.class) == w["class"]
        })
}
fn i70_counts(got: &[rp::RowClassification]) -> [usize; 5] {
    let mut n = [0; 5];
    for g in got {
        n[match g.class {
            rp::AccuracyClass::RelativeVerified => 0,
            rp::AccuracyClass::AbsoluteVerified { .. } => 1,
            rp::AccuracyClass::InputDerived => 2,
            rp::AccuracyClass::NonQuantity => 3,
            rp::AccuracyClass::NotCovered => 4,
        }] += 1;
    }
    n
}
#[test]
fn i70_full_07l_listing() {
    use serde_json::json;
    let shared = corpus();
    let (mut cases_ok, mut muts_ok, mut mp_ok) = (0, 0, 0);
    let cases = shared["cases"].as_array().unwrap();
    for (i, case) in cases.iter().enumerate() {
        let want_rows = case["expected_classifications"].as_array().unwrap();
        let want_eligible = case["expected"]["numerical_eligible"].as_bool().unwrap();
        let line = match rp::validate(&case["source"], Some(&case["invocation"])) {
            Ok(got) => {
                let unbound = rp::validate(&case["source"], None)
                    .map(|u| !u.invocation_bound && !u.numerical_eligible)
                    .unwrap_or(false);
                let transport = rp::validate_transport_metadata(&case["source"])
                    .map(|t| !t.invocation_bound && !t.numerical_eligible && t.classifications.is_empty())
                    .unwrap_or(false);
                let ok = got.invocation_bound
                    && got.numerical_eligible == want_eligible
                    && got.publication_sha256
                        == case["source"]["retained_precision"]["body"]["publication_sha256"]
                    && i70_rows_match(&got.classifications, want_rows)
                    && unbound
                    && transport;
                cases_ok += usize::from(ok);
                json!({"i":i,"id":case["id"],"eligible":got.numerical_eligible,"expected_eligible":want_eligible,
                       "rows":got.classifications.len(),"expected_rows":want_rows.len(),
                       "counts_rel_abs_inp_nq_nc":i70_counts(&got.classifications),
                       "unbound_not_eligible":unbound,"transport_empty":transport,"match":ok})
            }
            Err(e) => json!({"i":i,"id":case["id"],"rejected":{"gate":e.gate,"code":e.code},"match":false}),
        };
        println!("I70_CASE {line}");
    }
    let mutations = shared["mutations"].as_array().unwrap();
    for (i, m) in mutations.iter().enumerate() {
        let observed = observe(&shared, m);
        let expected = expected_for(m).clone();
        let ok = observed == expected;
        muts_ok += usize::from(ok);
        println!(
            "I70_MUTATION {}",
            json!({"i":i,"id":m["id"],"base":m["base"],"expected":expected,"observed":observed,"match":ok})
        );
    }
    let must = shared["must_pass"].as_array().unwrap();
    for (i, entry) in must.iter().enumerate() {
        let case = cases.iter().find(|c| c["id"] == entry["base"]).unwrap();
        let (source, invocation) = apply_entry(&shared, entry);
        let want_eligible = entry["expected_eligibility"]["numerical_eligible"].as_bool().unwrap();
        let line = match rp::validate(&source, Some(&invocation)) {
            Ok(got) => {
                let ok = entry["expected"] == "pass"
                    && got.invocation_bound
                    && got.numerical_eligible == want_eligible
                    && i70_rows_match(&got.classifications, case["expected_classifications"].as_array().unwrap());
                mp_ok += usize::from(ok);
                json!({"i":i,"id":entry["id"],"base":entry["base"],"admitted":true,"eligible":got.numerical_eligible,
                       "expected_eligible":want_eligible,"counts_rel_abs_inp_nq_nc":i70_counts(&got.classifications),"match":ok})
            }
            Err(e) => json!({"i":i,"id":entry["id"],"base":entry["base"],"rejected":{"gate":e.gate,"code":e.code},"match":false}),
        };
        println!("I70_MUST_PASS {line}");
    }
    println!(
        "I70_SUMMARY {}",
        json!({"cases":[cases_ok,cases.len()],"mutations":[muts_ok,mutations.len()],"must_pass":[mp_ok,must.len()]})
    );
    assert_eq!((cases_ok, muts_ok, mp_ok), (cases.len(), mutations.len(), must.len()));
}
