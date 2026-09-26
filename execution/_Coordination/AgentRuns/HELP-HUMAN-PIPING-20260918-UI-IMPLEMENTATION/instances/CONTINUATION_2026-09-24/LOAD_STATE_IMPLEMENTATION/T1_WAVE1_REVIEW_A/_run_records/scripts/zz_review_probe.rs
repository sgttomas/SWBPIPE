//! Reviewer probe runner (scratch copy only). Reads every *.json in
//! $PROBE_DIR and prints one JSON line of outcomes per file.
use open_pipe_stress_result_export::semantic_contract as s;
use serde_json::{json, Value};

fn out<T>(r: Result<T, String>) -> String {
    match r {
        Ok(_) => "accept".into(),
        Err(e) => e,
    }
}

#[test]
fn probe() {
    let Ok(dir) = std::env::var("PROBE_DIR") else {
        return;
    };
    let mut paths: Vec<_> = std::fs::read_dir(&dir)
        .unwrap()
        .map(|e| e.unwrap().path())
        .filter(|p| p.extension().is_some_and(|x| x == "json"))
        .collect();
    paths.sort();
    let mut lines = Vec::new();
    for p in paths {
        let text = std::fs::read_to_string(&p).unwrap();
        let name = p.file_name().unwrap().to_string_lossy().to_string();
        let v: Value = match serde_json::from_str(&text) {
            Ok(v) => v,
            Err(_) => {
                lines.push(json!({"id": name, "dispatch": "JSON_PARSE_REJECTED", "validator": "JSON_PARSE_REJECTED", "transport": "JSON_PARSE_REJECTED", "standing": "JSON_PARSE_REJECTED"}));
                continue;
            }
        };
        let dispatch = out(s::for_source(&v));
        let validator = out(std::panic::catch_unwind(|| s::validate_load_reference_source_evidence(&v)).unwrap_or(Err("PANIC".into())));
        let mut meta = json!({});
        for key in ["schema_version", "producer", "numerical_quality", "formulation_basis",
                    "contract_evidence", "source_block_recovery", "carrier_evidence"] {
            if let Some(x) = v.get(key) {
                meta[key] = x.clone();
            }
        }
        let transport = out(std::panic::catch_unwind(|| s::validate_load_reference_source_transport_metadata(&meta)).unwrap_or(Err("PANIC".into())));
        let refs: Vec<Value> = v["numerical_quality"]["cases"]
            .as_array()
            .map(|a| a.iter().map(|c| c["basis_ref"].clone()).collect())
            .unwrap_or_default();
        let standing = std::panic::catch_unwind(|| s::numerical_use_standing(&v, &refs))
            .map(|x| x.to_string())
            .unwrap_or("PANIC".into());
        lines.push(json!({"id": name, "dispatch": dispatch, "validator": validator, "transport": transport, "standing": standing}));
    }
    let target = std::env::var("PROBE_OUT").unwrap();
    std::fs::write(
        target,
        lines.iter().map(|l| l.to_string()).collect::<Vec<_>>().join("\n") + "\n",
    )
    .unwrap();
}
