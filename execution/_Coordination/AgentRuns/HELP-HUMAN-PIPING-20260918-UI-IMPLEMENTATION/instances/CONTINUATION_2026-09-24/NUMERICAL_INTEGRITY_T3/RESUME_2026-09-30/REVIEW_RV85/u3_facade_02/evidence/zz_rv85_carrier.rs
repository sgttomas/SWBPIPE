//! RV85 (disposable, archive-only, grant 1b): R-1's public carrier over the same
//! inputs. Without a permit every retained call must name the ordinary publication,
//! with exactly the value route's bytes, and no successor.
use open_pipe_stress_product_physics::*;
use serde_json::{json, Value};

#[test]
fn zz_rv85_carrier_is_ordinary_without_a_permit() {
    let p = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let list = std::fs::read_to_string(std::env::var("RV85_SWEEP_LIST").unwrap()).unwrap();
    let (mut rows, mut checked, mut errs) = (Vec::new(), 0, 0);
    for line in list.lines().filter(|l| !l.trim().is_empty()) {
        let (kind, rel) = line.split_once(' ').unwrap();
        let v: Value = serde_json::from_str(&std::fs::read_to_string(p.join(rel)).unwrap()).unwrap();
        let raw = if kind == "MOD" { json!({"model": v}) } else { v };
        for mode in [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny] {
            let plain = run_linear_static_preview_value_with_mode(raw.clone(), mode).map(|e| serde_json::to_vec(&e).unwrap());
            let invocation = json!({"request": &raw, "solver_mode": mode.as_str()});
            let rid = format!("rv85-{rel}");
            for route in ["direct", "headless"] {
                let out = if route == "direct" { run_linear_static_preview_value_with_retained_direct(raw.clone(), mode) } else {
                    run_linear_static_preview_value_with_retained_headless(raw.clone(), mode, RetainedHeadlessContext::from_borrowed_roots(&raw, &invocation, &rid)) };
                match (out, &plain) {
                    (Ok(o), Ok(plain)) => {
                        assert!(o.successor().is_none(), "{rel} {route}: a successor without a permit");
                        let env = serde_json::to_vec(o.envelope()).unwrap();
                        match o.into_publication() {
                            RetainedPublication::Ordinary(e) => {
                                let published = serde_json::to_vec(&e).unwrap();
                                assert_eq!(&published, plain, "{rel} {route}: Ordinary with the plain bytes");
                                assert_eq!(published, env, "{rel} {route}: the publication is the envelope");
                            }
                            RetainedPublication::Successor(_) => panic!("{rel} {route}: Successor without a permit"),
                        }
                        checked += 1;
                        rows.push(format!("{rel}\t{}\t{route}\tOrdinary=plain", mode.as_str()));
                    }
                    (Err(a), Err(b)) => { assert_eq!(&a, b); errs += 1; rows.push(format!("{rel}\t{}\t{route}\terr=plain_err", mode.as_str())); }
                    (a, b) => panic!("{rel} {route}: route outcome differs: {:?} vs {:?}", a.is_ok(), b.is_ok()),
                }
            }
        }
    }
    rows.push(format!("# checked={checked} equal_errors={errs}"));
    std::fs::write(std::env::var("RV85_CARRIER_OUT").unwrap(), rows.join("\n") + "\n").unwrap();
}
