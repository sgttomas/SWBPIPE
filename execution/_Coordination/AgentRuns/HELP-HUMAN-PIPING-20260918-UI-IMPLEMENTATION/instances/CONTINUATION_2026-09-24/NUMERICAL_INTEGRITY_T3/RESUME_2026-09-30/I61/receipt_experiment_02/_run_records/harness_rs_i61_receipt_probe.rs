//! I61 disposable harness (archive copy only): runs the unchanged Rust reader on emitted receipts.
use open_pipe_stress_result_export::retained_precision as rp;
use serde_json::Value;
#[test]
fn i61_rust_reader_on_emitted_receipts() {
    let list = std::env::var("I61_RECEIPTS").expect("I61_RECEIPTS");
    for path in list.split(':').filter(|p| !p.is_empty()) {
        let case: Value = serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap();
        match rp::validate(&case["source"], Some(&case["invocation"])) {
            Ok(v) => {
                println!("I61_RS {} PASS eligible={} invocation_bound={} classes={}", case["id"], v.numerical_eligible, v.invocation_bound, v.classifications.len());
                let rows: Vec<String> = v.classifications.iter().map(|c| format!("{}|{:016x}|{}|{:?}", c.result_id, c.normalized_bits, c.scale_bits.map(|s| format!("{s:016x}")).unwrap_or("null".into()), c.class)).collect();
                std::fs::write(format!("{path}.rs_classes.txt"), rows.join("\n")).unwrap();
            }
            Err(e) => println!("I61_RS {} FIRST {} {} detail={:?}", case["id"], e.gate, e.code, e.detail),
        }
    }
}
