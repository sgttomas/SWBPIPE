// M3-T runner (harness, not reference). It calls only the two public entry points
//   run_linear_static_preview_with_mode        (typed: the request is deserialized first)
//   run_linear_static_preview_value_with_mode  (captured: the request JSON value)
// in both solver modes, and writes one JSON record per (model, entry, mode) with the result
// rows (kind, value, entity_ref, case, location, component, id, metadata), diagnostics
// (which carry the K-D5 standing), numerical_quality, summary and contract_evidence.
//
// Environment: KD5_IN (input JSONL, lines {tag, request, ...}), KD5_OUT (output JSONL),
// KD5_ENTRIES (default "typed,captured"), KD5_MODES (default "sparse,dense"),
// KD5_SLIM (set: omit result rows and contract evidence, for standings-only census runs).
use open_pipe_stress_product_physics::{
    run_linear_static_preview_value_with_mode, run_linear_static_preview_with_mode,
    LinearStaticPreviewRequest, PreviewSolverMode,
};
use serde_json::{json, Value};
use std::io::{BufRead, Write};

fn main() {
    let input = std::env::var("KD5_IN").expect("KD5_IN");
    let output = std::env::var("KD5_OUT").expect("KD5_OUT");
    let entries = std::env::var("KD5_ENTRIES").unwrap_or("typed,captured".into());
    let modes_text = std::env::var("KD5_MODES").unwrap_or("sparse,dense".into());
    let mut modes = Vec::new();
    for m in modes_text.split(',') {
        match m {
            "sparse" => modes.push(PreviewSolverMode::SparseInteractive),
            "dense" => modes.push(PreviewSolverMode::DenseScrutiny),
            other => panic!("unknown mode {other}"),
        }
    }
    let slim = std::env::var("KD5_SLIM").is_ok();
    let mut out = std::io::BufWriter::new(std::fs::File::create(output).unwrap());
    let reader = std::io::BufReader::new(std::fs::File::open(input).unwrap());
    for line in reader.lines() {
        let item: Value = serde_json::from_str(&line.unwrap()).unwrap();
        let tag = item["tag"].as_str().unwrap().to_string();
        for entry in entries.split(',') {
            for mode in modes.iter().copied() {
                let request = item["request"].clone();
                // Labels for an instrumented (probe) build of the product, which may read them to tag its own dump.
                std::env::set_var("KD5_ACTIVE_TAG", &tag);
                std::env::set_var("KD5_ACTIVE_ENTRY", entry);
                std::env::set_var("KD5_ACTIVE_MODE", format!("{mode:?}"));
                let now = std::time::Instant::now();
                let env = match entry {
                    "captured" => run_linear_static_preview_value_with_mode(request, mode),
                    "typed" => serde_json::from_value::<LinearStaticPreviewRequest>(request)
                        .map_err(|e| format!("DTO: {e}"))
                        .map(|q| run_linear_static_preview_with_mode(q, mode)),
                    other => panic!("unknown entry {other}"),
                };
                let ns = now.elapsed().as_nanos();
                let record = match env {
                    Ok(env) => {
                        let v = serde_json::to_value(env).unwrap();
                        let results_count = v["results"].as_array().map(Vec::len).unwrap_or(0);
                        let rows: Vec<Value> = if slim {
                            Vec::new()
                        } else {
                            v["results"]
                                .as_array()
                                .map(|a| {
                                    a.iter()
                                        .map(|r| {
                                            json!({"kind":r["kind"],"value":r["value"],"unit":r["unit"],
                                            "entity_ref":r["entity_ref"],"case":r["basis_ref"]["ref_id"],
                                            "location":r["metadata"]["location"],"component":r["metadata"]["component"],
                                            "id":r["id"],"metadata":r["metadata"]})
                                        })
                                        .collect()
                                })
                                .unwrap_or_default()
                        };
                        let evidence = if slim { Value::Null } else { v["contract_evidence"].clone() };
                        json!({"tag":tag,"entry":entry,"mode":format!("{mode:?}"),"ns":ns,
                        "diagnostics":v["diagnostics"],"results":rows,"results_count":results_count,
                        "numerical_quality":v["numerical_quality"],"summary":v["summary"],
                        "contract_evidence":evidence,"publication":"ordinary"})
                    }
                    Err(error) => json!({"tag":tag,"entry":entry,"mode":format!("{mode:?}"),"ns":ns,"error":error}),
                };
                writeln!(out, "{}", serde_json::to_string(&record).unwrap()).unwrap();
            }
        }
    }
    out.flush().unwrap();
}
