//! RV93 scratch example (never committed): the actual Direct entry from a NON-test build of PP
//! (examples link the library without cfg(test)), on the milestone in both modes. Prints the
//! publication kind, the U1 document's sha256 and the published value's sha256.
use open_pipe_stress_product_physics::*;
use sha2::{Digest, Sha256};
fn main() {
    let raw: serde_json::Value = serde_json::from_str(include_str!("../../../fixtures/product_preview/rf_skew_t_cant_off_122_r1e-04.request.json")).unwrap();
    for mode in [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny] {
        let out = run_linear_static_preview_value_with_retained_direct(raw.clone(), mode).unwrap();
        let profile = out.admission().map(|r| format!("{:?}", r.profile));
        let env = format!("{:x}", Sha256::digest(serde_json::to_vec(out.envelope()).unwrap()));
        match out.into_publication() {
            RetainedPublication::Successor(v) => {
                let doc = serde_json::to_string_pretty(&serde_json::json!({"id": format!("u1_milestone_{}", mode.as_str()), "source": v,
                    "invocation": {"request": raw, "solver_mode": mode.as_str()}})).unwrap();
                println!("RV93_NONTEST {} profile={profile:?} Successor doc_sha={:x} value_sha={:x} envelope_sha={env}", mode.as_str(),
                    Sha256::digest(doc.as_bytes()), Sha256::digest(serde_json::to_vec(&v).unwrap()));
            }
            RetainedPublication::Ordinary(e) => println!("RV93_NONTEST {} profile={profile:?} Ordinary sha={:x}", mode.as_str(), Sha256::digest(serde_json::to_vec(&e).unwrap())),
        }
    }
}
