//! RV90 review harness (review copy only): validate pre-applied corpus entries.
use open_pipe_stress_result_export::retained_precision as rp;
use serde_json::{json, Value};
use std::io::{BufRead, Write};
#[test]
fn rv90_dump() {
    let inp = std::env::var("RV90_APPLIED").unwrap();
    let out = std::env::var("RV90_OUT").unwrap();
    let mut res = serde_json::Map::new();
    for line in std::io::BufReader::new(std::fs::File::open(inp).unwrap()).lines() {
        let d: Value = serde_json::from_str(&line.unwrap()).unwrap();
        let got = match rp::validate(&d["source"], Some(&d["invocation"])) {
            Err(e) => json!([e.gate, e.code]),
            Ok(v) => {
                let mut k = [0usize; 5];
                for c in &v.classifications {
                    k[match c.class { rp::AccuracyClass::RelativeVerified => 0, rp::AccuracyClass::AbsoluteVerified { .. } => 1, rp::AccuracyClass::InputDerived => 2, rp::AccuracyClass::NonQuantity => 3, rp::AccuracyClass::NotCovered => 4 }] += 1;
                }
                json!({"pass": true, "eligible": v.numerical_eligible, "classes": &k[..4], "not_covered": k[4]})
            }
        };
        res.insert(format!("{}:{}", d["kind"].as_str().unwrap(), d["id"].as_str().unwrap()), got);
    }
    let mut f = std::fs::File::create(out).unwrap();
    f.write_all(serde_json::to_string_pretty(&Value::Object(res)).unwrap().as_bytes()).unwrap();
}
