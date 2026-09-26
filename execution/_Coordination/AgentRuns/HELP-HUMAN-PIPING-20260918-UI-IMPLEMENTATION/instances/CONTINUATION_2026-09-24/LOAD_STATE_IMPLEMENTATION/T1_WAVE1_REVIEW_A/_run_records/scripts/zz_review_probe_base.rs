//! Reviewer probe runner for the base commit (dispatch only). Scratch copy only.
use open_pipe_stress_result_export::semantic_contract as s;
use serde_json::{json, Value};
#[test]
fn probe_base() {
    let Ok(dir) = std::env::var("PROBE_DIR") else { return; };
    let mut paths: Vec<_> = std::fs::read_dir(&dir).unwrap().map(|e| e.unwrap().path()).collect();
    paths.sort();
    let mut lines = Vec::new();
    for p in paths {
        let v: Value = serde_json::from_str(&std::fs::read_to_string(&p).unwrap()).unwrap();
        let d = match s::for_source(&v) { Ok(_) => "accept".to_string(), Err(e) => e };
        lines.push(json!({"id": p.file_name().unwrap().to_string_lossy(), "dispatch": d}).to_string());
    }
    std::fs::write(std::env::var("PROBE_OUT").unwrap(), lines.join("\n") + "\n").unwrap();
}
