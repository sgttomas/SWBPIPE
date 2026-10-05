//! RV88 (U6 repair confirmation): independent probes on `da274dd961`.
//! Writes facts to $RV88_REP_OUT for the Python oracle (rv88_rep_check.py) and
//! asserts only fail-closed properties.
use open_pipe_stress_result_export::{derivative as d, retained_precision as rp, semantic_contract as s};
use serde_json::{json, Value};
use std::fmt::Write as _;

fn p() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..")
}
fn read(rel: &str) -> Value {
    serde_json::from_str(&std::fs::read_to_string(p().join(rel)).unwrap()).unwrap()
}
fn derive(raw: &Value) -> Result<Value, String> {
    let model = raw["model_ref"].as_str().unwrap_or("m");
    let carrier = d::checksum(raw, "attested_headless_producer_carrier", d::reference("test_carrier", "rv88"))?;
    let origin = json!({"origin_id":"rv88","origin_class":"attested_headless_producer","qualification_ref":d::reference("test_fixture","not-authentication"),"authentic_producer_available":true,"received_carrier_checksum":carrier,"original_producer_checksum":carrier,"origin_limit":"RV88","actual_model_ref":d::reference("model_payload",model),"mechanics_run_ref":d::reference("mechanics_run",raw["run_id"].as_str().unwrap_or("r")),"request_model_ref":null,"request_run_ref":null,"request_alias_disclosure":null});
    let mut base = read("fixtures/results/invented/tp_phys_015_canonical_solve_result_envelope.json");
    base["result_envelope"]["envelope_id"] = json!("envelope:rv88");
    d::derive_document(base, &json!({"project":{"id":model}}), raw, origin, None)
}
fn refs(inv: &Value) -> Vec<Value> {
    inv["request"]["model"]["load_cases"].as_array().unwrap().iter().map(|c| json!({"ref_type":"load_case","ref_id":c["id"]})).collect()
}
fn set(target: &mut Value, path: &[Value], value: Value) {
    let mut at = target;
    for k in path {
        at = match k {
            Value::String(s) => &mut at[s.as_str()],
            Value::Number(i) => &mut at[i.as_u64().unwrap() as usize],
            _ => panic!(),
        };
    }
    *at = value;
}

#[test]
fn rv88_rep() {
    let Ok(out) = std::env::var("RV88_REP_OUT") else { return };
    let mut log = String::new();
    // (1) S-2: every absolute row's message in both milestone derivatives.
    for mode in ["sparse_interactive", "dense_scrutiny"] {
        let doc = read(&format!("fixtures/results/retained_precision_milestone_successor_{mode}.json"));
        let src = doc["source"].clone();
        let der = derive(&src).unwrap();
        d::validate_document(&der, &src).unwrap();
        for x in der["result_envelope"]["row_disclosures"].as_array().unwrap() {
            if x["reason_code"] == d::RETAINED_ABSOLUTE_VERIFIED || x["reason_code"] == d::RETAINED_NOT_COVERED {
                writeln!(log, "msg\t{mode}\t{}\t{}\t{}\t{}", x["source_result_id"].as_str().unwrap(), x["source_unit"].as_str().unwrap(), x["reason_code"].as_str().unwrap(), x["message"].as_str().unwrap()).unwrap();
            }
        }
        // A message whose unit is swapped is refused.
        let dis = der["result_envelope"]["row_disclosures"].as_array().unwrap();
        for (from, to) in [(" m (binary64", " mm (binary64"), (" Pa (binary64", " MPa (binary64"), (" N*m (binary64", " N\u{b7}m (binary64"), (" N (binary64", " kN (binary64")] {
            if let Some(i) = dis.iter().position(|x| x["message"].as_str().unwrap().contains(from)) {
                let mut bad = der.clone();
                let m = bad["result_envelope"]["row_disclosures"][i]["message"].as_str().unwrap().replace(from, to);
                bad["result_envelope"]["row_disclosures"][i]["message"] = json!(m);
                writeln!(log, "swap\t{mode}\t{}\t{:?}", from.trim(), d::validate_document(&bad, &src)).unwrap();
                assert!(d::validate_document(&bad, &src).is_err());
            }
        }
    }
    // (2) class_disclosure over units and bounds, including non-negative exponents.
    let units = ["m", "mm", "rad", "N", "kN", "N*m", "kN*m", "Pa", "MPa", "unitless", "mode_code", "", "deg", "kPa", "N\u{b7}m"];
    let bounds: [u64; 9] = [0x3b1a378ea78c5ce9, 0, 1, 0x3ff0000000000000, 0x3ff8000000000000, 0x4415af1d78b58c40, 0x7fefffffffffffff, 0x3eb0c6f7a0b5ed8d, 0x0010000000000000];
    for u in units {
        for b in bounds {
            let r = d::class_disclosure("element_local_axial_force", u, Some(&rp::AccuracyClass::AbsoluteVerified { bound_bits: b }));
            writeln!(log, "cd\t{u}\t{b:016x}\t{r:?}").unwrap();
        }
        let r = d::class_disclosure("element_local_axial_force", u, Some(&rp::AccuracyClass::NotCovered));
        writeln!(log, "cdnc\t{u}\t{r:?}").unwrap();
    }
    // (3) The 20 shared cases, by RV88's own Rust mapping.
    let cases = read("fixtures/results/retained_precision_carrier_cases.json");
    for c in cases["cases"].as_array().unwrap() {
        let f = &cases["fixtures"][c["fixture"].as_str().unwrap()];
        let doc = read(f["path"].as_str().unwrap());
        let (mut src, mut inv) = if f["shape"] == "milestone" { (doc["source"].clone(), doc["invocation"].clone()) } else { (doc.clone(), Value::Null) };
        for e in c["edits"].as_array().unwrap() {
            let path = e["path"].as_array().unwrap().clone();
            if e["target"] == "source" { set(&mut src, &path, e["value"].clone()) } else { set(&mut inv, &path, e["value"].clone()) }
        }
        let req = if c["requested"] == "invocation" { refs(&inv) } else { c["requested"].as_array().unwrap().clone() };
        let invocation = (!c["invocation"].is_null()).then_some(&inv);
        let standing = s::numerical_use_standing_with_context(&src, &req, invocation);
        let dispatch = match s::for_source(&src) { Ok(_) => "ok".to_string(), Err(e) => e };
        writeln!(log, "case\t{}\t{standing}\t{dispatch}\t{}\t{}", c["id"].as_str().unwrap(), c["expected_standing"].as_str().unwrap(), c["expected_dispatch"].as_str().unwrap()).unwrap();
    }
    // (4) The declared differences, computed for Rust independently of the file's expectation.
    for dd in cases["declared_differences"].as_array().unwrap() {
        for fx in dd["fixtures"].as_array().unwrap() {
            let f = &cases["fixtures"][fx.as_str().unwrap()];
            let doc = read(f["path"].as_str().unwrap());
            let (mut src, inv) = (doc["source"].clone(), doc["invocation"].clone());
            for e in dd["edits"].as_array().unwrap() {
                set(&mut src, &e["path"].as_array().unwrap().clone(), e["value"].clone());
            }
            let listed: std::collections::HashSet<String> = doc["source"]["retained_precision"]["body"]["cases"][0]["selection"]["absolute_verified"].as_array().unwrap().iter().map(|x| x["result_id"].as_str().unwrap().to_string()).collect();
            let got = match dd["subject"].as_str().unwrap() {
                "standing" => s::numerical_use_standing_with_context(&src, &refs(&inv), None).to_string(),
                "transport" => match s::for_source_metadata(&src) { Ok(_) => "ok".into(), Err(e) => e },
                "binding" => {
                    let codes: Vec<Option<&str>> = src["results"].as_array().unwrap().iter().map(|r| s::rule_binding_refusal(&src, r)).collect();
                    if codes.iter().all(|c| *c == Some("RULE_QUANTITY_NOT_COVERED")) {
                        "every_row:RULE_QUANTITY_NOT_COVERED".into()
                    } else if src["results"].as_array().unwrap().iter().zip(&codes).all(|(r, c)| (*c == Some("RULE_QUANTITY_BELOW_VERIFIED_FLOOR")) == listed.contains(r["id"].as_str().unwrap()) && (c.is_none() || *c == Some("RULE_QUANTITY_BELOW_VERIFIED_FLOOR"))) {
                        "by_validated_class".into()
                    } else {
                        "OTHER".into()
                    }
                }
                other => format!("UNKNOWN_SUBJECT:{other}"),
            };
            writeln!(log, "dd\t{}\t{}\t{got}\t{}", dd["id"].as_str().unwrap(), fx.as_str().unwrap(), dd["expected"]["rust"]).unwrap();
        }
    }
    std::fs::write(out, log).unwrap();
}
