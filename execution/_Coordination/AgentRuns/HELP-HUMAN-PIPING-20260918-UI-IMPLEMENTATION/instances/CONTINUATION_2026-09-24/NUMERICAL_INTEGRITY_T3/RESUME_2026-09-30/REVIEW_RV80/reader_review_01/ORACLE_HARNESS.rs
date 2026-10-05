//! RV80 reviewer-only harness (not part of the candidate): compares the public
//! retained-precision helpers with RV80's exact-rational vectors.
use open_pipe_stress_result_export::retained_precision as rp;
use serde_json::Value;
fn d(v: &Value) -> f64 {
    f64::from_bits(u64::from_str_radix(v.as_str().unwrap(), 16).unwrap())
}
#[test]
fn rv80_exact_rational_oracle() {
    let path = std::env::var("RV80_VECTORS").expect("RV80_VECTORS");
    let doc: Value = serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap();
    let mut checked = std::collections::BTreeMap::<String, usize>::new();
    let mut bad = Vec::new();
    for v in doc["vectors"].as_array().unwrap() {
        let f = v["fn"].as_str().unwrap();
        let a = v["args"].as_array().unwrap();
        let got: Value = match f {
            "upward_product" => rp::upward_product(d(&a[0]), d(&a[1]))
                .map(|x| Value::from(format!("{:016x}", x.to_bits())))
                .unwrap_or(Value::Null),
            "upward_small_sum" => rp::upward_small_sum(d(&a[0]), d(&a[1]))
                .map(|x| Value::from(format!("{:016x}", x.to_bits())))
                .unwrap_or(Value::Null),
            "absolute_bound" => rp::absolute_bound(d(&a[0]), d(&a[1]))
                .map(|x| Value::from(format!("{:016x}", x.to_bits())))
                .unwrap_or(Value::Null),
            "phi_512" => Value::from(format!("{:016x}", rp::phi_512(d(&a[0])).to_bits())),
            "e_hat" => {
                let r = rp::e_hat([d(&a[0]), d(&a[1])], d(&a[2]));
                serde_json::json!([format!("{:016x}", r[0].to_bits()), format!("{:016x}", r[1].to_bits())])
            }
            _ => panic!("unknown fn"),
        };
        *checked.entry(f.to_string()).or_default() += 1;
        if got != v["expected"] {
            bad.push(format!("{f} {a:?}: got {got} want {}", v["expected"]));
        }
    }
    println!("RV80 oracle checked {checked:?}; mismatches {}", bad.len());
    for b in bad.iter().take(40) {
        println!("MISMATCH {b}");
    }
    assert!(bad.is_empty(), "{} mismatches", bad.len());
}
