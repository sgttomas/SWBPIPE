// RV95 scratch (never committed): WT/rv95/P/core/product_physics/tests/zz_rv95_physr4.rs was
// P/core/product_physics/tests/f1b_w2_runtime.rs at the PR head with every #[test] also marked #[ignore]
// (sed 's/^#\[test\]$/#[test]
#[ignore]/'), followed by this test:
/// RV95 scratch (never committed): G8 (c) sample. The PHYS-R4 pair through the
/// public Direct retained entry, both modes, compared with the value route's bytes.
#[test]
fn zz_rv95_phys_r4_direct() {
    use open_pipe_stress_product_physics::{run_linear_static_preview_value_with_retained_direct, RetainedPublication};
    for pressurized in [true, false] {
        for mode in MODES {
            let raw = phys_r4(pressurized);
            let value = serde_json::to_vec(&run_linear_static_preview_value_with_mode(raw.clone(), mode).unwrap()).unwrap();
            let out = run_linear_static_preview_value_with_retained_direct(raw, mode).unwrap();
            let profile = out.admission().map(|r| format!("{:?}", r.profile));
            let ordinary_equal = serde_json::to_vec(out.envelope()).unwrap() == value;
            let successor = out.successor().is_some();
            let published = match out.into_publication() {
                RetainedPublication::Successor(v) => serde_json::to_vec(&v).unwrap(),
                RetainedPublication::Ordinary(e) => serde_json::to_vec(&e).unwrap(),
            };
            let doc: Value = serde_json::from_slice(&published).unwrap();
            let notices = doc["diagnostics"].as_array().unwrap().iter().filter(|d| d["code"] == "RETAINED_PRECISION_UNAVAILABLE").count();
            let exact = published == value;
            println!("RV95_PHYSR4 pressurized={pressurized} mode={} profile={profile:?} ordinary_equal={ordinary_equal} successor={successor} exact={exact} notices={notices} status={}",
                mode.as_str(), doc["status"]["mechanics"]);
            assert!(ordinary_equal);
            if pressurized {
                assert!(!successor && exact && notices == 0, "pressure: the ordinary refusal bytes, no notice");
            } else {
                assert!(successor || exact || notices == 1, "no-pressure: successor, exact bytes, or exactly one notice");
            }
        }
    }
}
