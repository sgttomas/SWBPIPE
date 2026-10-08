//! RV108 probe harness (reviewer scratch only; never committed): each entry point over each probe.
use open_pipe_stress_result_export::{retained_precision as rp, semantic_contract as s};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::io::Write;

fn hex(d: impl AsRef<[u8]>) -> String { d.as_ref().iter().map(|b| format!("{b:02x}")).collect() }
fn res(r: std::thread::Result<Result<rp::Validation, rp::ValidationError>>) -> Value {
    match r {
        Ok(Ok(v)) => json!({"ok": true, "eligible": v.numerical_eligible, "sha": hex(Sha256::digest(format!("{v:?}").as_bytes()))}),
        Ok(Err(e)) => json!({"gate": e.gate, "code": e.code, "detail": e.detail}),
        Err(_) => json!({"panic": true}),
    }
}
fn disp(r: std::thread::Result<Result<(&'static Value, &'static str), String>>) -> Value {
    match r {
        Ok(Ok((t, v))) => json!({"ok": true, "contract": t["semantic_contract_id"], "version": v}),
        Ok(Err(e)) => json!({"error": e}),
        Err(_) => json!({"panic": true}),
    }
}
#[test]
fn rv108_probe() {
    let input = std::env::var("RV108_IN").expect("RV108_IN");
    let out = std::env::var("RV108_OUT").expect("RV108_OUT");
    let probes: Value = serde_json::from_str(&std::fs::read_to_string(input).unwrap()).unwrap();
    let mut f = std::io::BufWriter::new(std::fs::File::create(out).unwrap());
    std::panic::set_hook(Box::new(|_| {}));
    for p in probes.as_array().unwrap() {
        let (src, inv) = (&p["source"], &p["invocation"]);
        let u = std::panic::AssertUnwindSafe(());
        let _ = u;
        let row = json!({"id": p["id"], "family": p["family"],
            "raw_inv": res(std::panic::catch_unwind(|| rp::validate(src, Some(inv)))),
            "raw_none": res(std::panic::catch_unwind(|| rp::validate(src, None))),
            "transport": res(std::panic::catch_unwind(|| rp::validate_transport_metadata(src))),
            "c_raw": disp(std::panic::catch_unwind(|| s::for_source(src))),
            "c_transport": disp(std::panic::catch_unwind(|| s::for_source_metadata(src))),
        });
        writeln!(f, "{row}").unwrap();
    }
}
