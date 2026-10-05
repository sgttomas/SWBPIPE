//! RV88 (U6a review): downgrade guards and refusal codes, every form I could
//! construct. Candidate lane only. Records every outcome to $RV88_REFUSALS_OUT
//! and asserts the fail-closed property (never Ok / never a non-refusal).
use open_pipe_stress_result_export::{derivative as d, retained_precision as rp, semantic_contract as s, source_blocks};
use serde_json::{json, Value};
use std::fmt::Write as _;

const SPARSE: &str = include_str!("../../../../fixtures/results/retained_precision_milestone_successor_sparse_interactive.json");
const DENSE: &str = include_str!("../../../../fixtures/results/retained_precision_milestone_successor_dense_scrutiny.json");
const DG: &str = "RETAINED_PRECISION_DOWNGRADE_FORBIDDEN";
const BM: &str = "RETAINED_PRECISION_RECEIPT_BINDING_MISMATCH";

fn load(t: &str) -> (Value, Value) {
    let v: Value = serde_json::from_str(t).unwrap();
    (v["source"].clone(), v["invocation"].clone())
}
fn refs(inv: &Value) -> Vec<Value> {
    inv["request"]["model"]["load_cases"].as_array().unwrap().iter().map(|c| json!({"ref_type":"load_case","ref_id":c["id"]})).collect()
}
fn template() -> Value {
    serde_json::from_str(include_str!("../../../../fixtures/results/invented/tp_phys_015_canonical_solve_result_envelope.json")).unwrap()
}
fn derive_seeded(raw: &Value, seed: Option<Value>) -> Result<Value, String> {
    let model = raw["model_ref"].as_str().unwrap_or("m");
    let carrier = d::checksum(raw, "attested_headless_producer_carrier", d::reference("test_carrier", "rv88"))?;
    let origin = json!({"origin_id":"rv88","origin_class":"attested_headless_producer","qualification_ref":d::reference("test_fixture","not-authentication"),"authentic_producer_available":true,"received_carrier_checksum":carrier,"original_producer_checksum":carrier,"origin_limit":"RV88","actual_model_ref":d::reference("model_payload",model),"mechanics_run_ref":d::reference("mechanics_run",raw["run_id"].as_str().unwrap_or("r")),"request_model_ref":null,"request_run_ref":null,"request_alias_disclosure":null});
    let mut base = template();
    base["result_envelope"]["envelope_id"] = json!("envelope:rv88");
    if let Some(seed) = seed {
        base["result_envelope"]["retained_precision"] = seed;
    }
    d::derive_document(base, &json!({"project":{"id":model}}), raw, origin, None)
}
fn derive(raw: &Value) -> Result<Value, String> {
    derive_seeded(raw, None)
}
fn projection(raw: &Value) -> Value {
    let mut p = raw.clone();
    p.as_object_mut().unwrap().remove("retained_precision");
    p["producer"]["semantic_contract_id"] = json!(s::PREVIEW_PHYSICS_ID);
    p["formulation_basis"]["profile_id"] = json!("product_preview_mechanics_v1");
    for r in p["results"].as_array_mut().unwrap() {
        r.as_object_mut().unwrap().remove("recovery_method");
    }
    p
}
fn rehash(mut raw: Value) -> Value {
    let mut public = raw.clone();
    public.as_object_mut().unwrap().remove("retained_precision");
    raw["retained_precision"]["body"]["publication_sha256"] = json!(source_blocks::domain_hash("retained_precision_publication_mp_v2", &public).unwrap());
    let body = raw["retained_precision"]["body"].clone();
    raw["retained_precision"]["receipt_sha256"] = json!(source_blocks::domain_hash("retained_precision_receipt_mp_v2", &body).unwrap());
    raw
}
fn e<T>(r: Result<T, String>) -> String {
    match r {
        Ok(_) => "OK".into(),
        Err(x) => x,
    }
}

/// Every probe of one envelope form: dispatch (raw, transport), standing (with
/// and without the invocation), derive, classification summary, binding.
fn probe(log: &mut String, name: &str, env: &Value, inv: &Value) -> [String; 6] {
    let r = refs(inv);
    let out = [
        e(s::for_source(env)),
        e(s::for_source_metadata(env)),
        s::numerical_use_standing(env, &r).to_string(),
        s::numerical_use_standing_with_context(env, &r, Some(inv)).to_string(),
        e(derive(env)),
        format!("{}", s::classification_summary(env, Some(inv)).len()),
    ];
    writeln!(log, "{name}\tfor_source={}\tmetadata={}\tstanding={}\tstanding_inv={}\tderive={}\tsummary_len={}", out[0], out[1], out[2], out[3], out[4], out[5]).unwrap();
    out
}

#[test]
fn rv88_refusals() {
    let mut log = String::new();
    let mut failures = Vec::new();
    let ids = [
        s::PREVIEW_PHYSICS_ID,
        s::PHYSICS_ID,
        s::PHYSICS_SOURCE_ID,
        s::PRECISION_ID,
        s::LOAD_REFERENCE_ID,
        s::LOAD_REFERENCE_SOURCE_ID,
        source_blocks::CONTRACT_ID,
        "openpipestress.result_semantics/0.3.0/unknown-rv88",
    ];
    for (mode, text) in [("sparse", SPARSE), ("dense", DENSE)] {
        let (src, inv) = load(text);
        let other = load(if mode == "sparse" { DENSE } else { SPARSE }).0;
        // (1) Relabelled successor, every identity, every member form.
        for id in ids {
            for profile_kept in [false, true] {
                let mut base = src.clone();
                base["producer"]["semantic_contract_id"] = json!(id);
                if !profile_kept {
                    base["formulation_basis"]["profile_id"] = json!("product_preview_mechanics_v1");
                }
                let forms: Vec<(&str, Value)> = vec![
                    ("receipt", base.clone()),
                    ("receipt_rehashed", rehash(base.clone())),
                    ("null", { let mut b = base.clone(); b["retained_precision"] = Value::Null; b }),
                    ("empty_object", { let mut b = base.clone(); b["retained_precision"] = json!({}); b }),
                    ("string", { let mut b = base.clone(); b["retained_precision"] = json!("x"); b }),
                    ("tokens_only", { let mut b = base.clone(); b.as_object_mut().unwrap().remove("retained_precision"); b }),
                    ("one_token_last", {
                        let mut b = projection(&src);
                        b["producer"]["semantic_contract_id"] = json!(id);
                        let n = b["results"].as_array().unwrap().len();
                        b["results"][n - 1]["recovery_method"] = json!(rp::METHOD);
                        b
                    }),
                ];
                for (form, env) in forms {
                    let name = format!("{mode}/relabel/{id}/profile_kept={profile_kept}/{form}");
                    let o = probe(&mut log, &name, &env, &inv);
                    let has_member = env.get("retained_precision").is_some();
                    // Raw dispatch must refuse; the guard's code wherever the member or a token is present
                    // and the identity's own checks did not fail first.
                    if o[0] == "OK" { failures.push(format!("{name}: for_source admitted")); }
                    if o[0] != DG { writeln!(log, "{name}\tNOTE_raw_first_code={}", o[0]).unwrap(); }
                    if has_member && o[1] != DG { failures.push(format!("{name}: metadata={}", o[1])); }
                    if o[2] != "unsupported" || o[3] != "unsupported" { failures.push(format!("{name}: standing {} {}", o[2], o[3])); }
                    if o[4] == "OK" { failures.push(format!("{name}: derive admitted")); }
                    if o[5] != "0" { failures.push(format!("{name}: summary non-empty")); }
                }
            }
        }
        // (2) Successor id, but schema_version or receipt structure wrong.
        for (form, env) in [
            ("schema_0.3.0", { let mut b = src.clone(); b["schema_version"] = json!("0.3.0"); b }),
            ("schema_0.4.0", { let mut b = src.clone(); b["schema_version"] = json!("0.4.0"); b }),
            ("receipt_dropped", { let mut b = src.clone(); b.as_object_mut().unwrap().remove("retained_precision"); b }),
            ("receipt_null", { let mut b = src.clone(); b["retained_precision"] = Value::Null; b }),
            ("tokens_dropped", { let mut b = src.clone(); for r in b["results"].as_array_mut().unwrap() { r.as_object_mut().unwrap().remove("recovery_method"); } b }),
            ("tokens_dropped_rehashed", { let mut b = src.clone(); for r in b["results"].as_array_mut().unwrap() { r.as_object_mut().unwrap().remove("recovery_method"); } rehash(b) }),
            ("other_mode_receipt", { let mut b = src.clone(); b["retained_precision"] = other["retained_precision"].clone(); b }),
            ("profile_ordinary", { let mut b = src.clone(); b["formulation_basis"]["profile_id"] = json!("product_preview_mechanics_v1"); rehash(b) }),
            ("producer_id_missing_receipt_kept", { let mut b = src.clone(); b["producer"].as_object_mut().unwrap().remove("semantic_contract_id"); b }),
        ] {
            let name = format!("{mode}/successor/{form}");
            let o = probe(&mut log, &name, &env, &inv);
            // Transport cannot see raw rows or authenticate the publication digest (by design).
            let transport_blind = matches!(form, "tokens_dropped" | "tokens_dropped_rehashed" | "other_mode_receipt");
            if o[0] == "OK" || (o[1] == "OK" && !transport_blind) { failures.push(format!("{name}: admitted {o:?}")); }
            if o[2] != "unsupported" || o[3] != "unsupported" { failures.push(format!("{name}: standing {o:?}")); }
            if o[4] == "OK" { failures.push(format!("{name}: derive admitted")); }
        }
        // (3) Derivative receipt and disclosure mutations.
        let doc = derive(&src).unwrap();
        d::validate_document(&doc, &src).unwrap();
        let disclosures = doc["result_envelope"]["row_disclosures"].as_array().unwrap().clone();
        let abs = disclosures.iter().position(|x| x["reason_code"] == d::RETAINED_ABSOLUTE_VERIFIED).unwrap();
        let plain = disclosures.iter().position(|x| x["reason_code"] != d::RETAINED_ABSOLUTE_VERIFIED).unwrap();
        let msg = disclosures[abs]["message"].as_str().unwrap().to_string();
        let bits_pos = msg.find("(binary64 ").unwrap() + 10;
        let mut flipped_bits = msg.clone();
        let c = if &msg[bits_pos + 15..bits_pos + 16] == "0" { "1" } else { "0" };
        flipped_bits.replace_range(bits_pos + 15..bits_pos + 16, c);
        let mut doc_cases: Vec<(&str, Value, &Value, &str)> = Vec::new();
        let mut put = |name: &'static str, f: &dyn Fn(&mut Value)| {
            let mut x = doc.clone();
            f(&mut x);
            x
        };
        let dropped = put("dropped", &|x| { x["result_envelope"].as_object_mut().unwrap().remove("retained_precision"); });
        let nulled = put("null", &|x| { x["result_envelope"]["retained_precision"] = Value::Null; });
        let swapped = put("swapped", &|x| { x["result_envelope"]["retained_precision"] = other["retained_precision"].clone(); });
        let sha_only = put("sha", &|x| { x["result_envelope"]["retained_precision"]["receipt_sha256"] = json!("0".repeat(64)); });
        let body_edit = put("body", &|x| { x["result_envelope"]["retained_precision"]["body"]["cases"][0]["selection"]["absolute_verified"][0]["bound"] = json!("0000000000000000"); });
        let extra_key = put("extra", &|x| { x["result_envelope"]["retained_precision"]["rv88"] = json!(1); });
        let msg_bits = put("msg_bits", &|x| { x["result_envelope"]["row_disclosures"][abs]["message"] = json!(flipped_bits.clone()); });
        let msg_trunc = put("msg_trunc", &|x| { x["result_envelope"]["row_disclosures"][abs]["message"] = json!(msg[..msg.len() - 1].to_string()); });
        let code_nc = put("code_nc", &|x| { x["result_envelope"]["row_disclosures"][abs]["reason_code"] = json!(d::RETAINED_NOT_COVERED); });
        let claim_plain = put("claim_plain", &|x| { x["result_envelope"]["row_disclosures"][plain]["reason_code"] = json!(d::RETAINED_ABSOLUTE_VERIFIED); });
        let claim_plain_msg = put("claim_plain_msg", &|x| { x["result_envelope"]["row_disclosures"][plain]["message"] = json!(msg.clone()); });
        let value_edit = put("value_edit", &|x| { x["result_envelope"]["row_disclosures"][abs]["source_value"] = json!(1.0); });
        let ref_edit = put("ref_edit", &|x| { x["result_envelope"]["semantic_contract_ref"]["ref_id"] = json!(s::PREVIEW_PHYSICS_ID); });
        doc_cases.push(("dropped", dropped, &src, BM));
        doc_cases.push(("null", nulled, &src, BM));
        doc_cases.push(("swapped_other_mode", swapped, &src, BM));
        doc_cases.push(("receipt_sha_edit", sha_only, &src, BM));
        doc_cases.push(("receipt_body_edit", body_edit, &src, BM));
        doc_cases.push(("receipt_extra_key", extra_key, &src, BM));
        doc_cases.push(("message_bits_flip", msg_bits, &src, "DISCLOSURE_SEMANTICS_MISMATCH"));
        doc_cases.push(("message_truncated", msg_trunc, &src, "DISCLOSURE_SEMANTICS_MISMATCH"));
        doc_cases.push(("code_to_not_covered", code_nc, &src, "DISCLOSURE_SEMANTICS_MISMATCH"));
        doc_cases.push(("plain_claims_class_code", claim_plain, &src, "DISCLOSURE_SEMANTICS_MISMATCH"));
        doc_cases.push(("plain_claims_class_message", claim_plain_msg, &src, "*"));
        doc_cases.push(("class_value_edit", value_edit, &src, "*"));
        doc_cases.push(("semantic_ref_to_base", ref_edit, &src, "*"));
        let proj = projection(&src);
        doc_cases.push(("successor_doc_vs_projection_source", doc.clone(), &proj, DG));
        let pdoc = derive(&proj).unwrap();
        let mut pdoc_r = pdoc.clone();
        pdoc_r["result_envelope"]["retained_precision"] = src["retained_precision"].clone();
        doc_cases.push(("projection_doc_plus_receipt_vs_projection", pdoc_r.clone(), &proj, DG));
        doc_cases.push(("projection_doc_plus_receipt_vs_successor", pdoc_r, &src, "*"));
        let mut pdoc_n = pdoc.clone();
        pdoc_n["result_envelope"]["retained_precision"] = Value::Null;
        doc_cases.push(("projection_doc_null_member", pdoc_n, &proj, DG));
        doc_cases.push(("projection_doc_vs_successor", pdoc.clone(), &src, "*"));
        let other_doc = derive(&other).unwrap();
        doc_cases.push(("other_mode_doc_vs_source", other_doc, &src, "*"));
        for (name, x, source, want) in doc_cases {
            let got = e(d::validate_document(&x, source));
            writeln!(log, "{mode}/doc/{name}\tvalidate_document={got}\twant={want}").unwrap();
            if got == "OK" || (want != "*" && got != want) {
                failures.push(format!("{mode}/doc/{name}: {got} (want {want})"));
            }
        }
        // (4) Base sources and base derivatives given a receipt before derivation.
        for seed in [src["retained_precision"].clone(), Value::Null, json!({})] {
            let got = e(derive_seeded(&proj, Some(seed.clone())));
            writeln!(log, "{mode}/seeded_base_derive/{}\t{got}", if seed.is_null() { "null" } else if seed == json!({}) { "empty" } else { "receipt" }).unwrap();
            if got != DG { failures.push(format!("{mode}/seeded_base_derive: {got}")); }
        }
        // A successor derive seeded with a different receipt is overwritten by the source's (copy wins).
        let seeded = derive_seeded(&src, Some(other["retained_precision"].clone()));
        let ok = seeded.as_ref().is_ok_and(|x| x["result_envelope"]["retained_precision"] == src["retained_precision"]);
        writeln!(log, "{mode}/seeded_successor_derive_other_receipt\toverwritten_by_source={ok}").unwrap();
        // (5) Binding: a refused statement refuses every row (F5); a relabelled one binds nothing new.
        let mut broken = src.clone();
        broken["results"][1]["value"] = json!(7.0);
        let codes: std::collections::BTreeSet<_> = src["results"].as_array().unwrap().iter().map(|r| format!("{:?}", s::rule_binding_refusal(&broken, r))).collect();
        writeln!(log, "{mode}/binding/refused_statement\t{codes:?}").unwrap();
        let mut idless = src["results"][0].clone();
        idless.as_object_mut().unwrap().remove("id");
        writeln!(log, "{mode}/binding/row_without_id\t{:?}", s::rule_binding_refusal(&src, &idless)).unwrap();
        let foreign = json!({"id":"result:rv88:not-in-envelope","kind":"element_local_axial_force","value":1.0,"unit":"N"});
        writeln!(log, "{mode}/binding/row_not_in_envelope\t{:?}", s::rule_binding_refusal(&src, &foreign)).unwrap();
        let mut relabelled = src.clone();
        relabelled["producer"]["semantic_contract_id"] = json!(s::PREVIEW_PHYSICS_ID);
        let codes: std::collections::BTreeSet<_> = src["results"].as_array().unwrap().iter().map(|r| format!("{:?}", s::rule_binding_refusal(&relabelled, r))).collect();
        writeln!(log, "{mode}/binding/relabelled_preview_physics_1\t{codes:?}").unwrap();
    }
    // (6) A legacy 0.1.0 source (no producer) carrying a receipt, a null member or a token.
    let legacy: Value = serde_json::from_str(include_str!("../../../../fixtures/product_preview/invented_mechanics_result.json")).unwrap();
    let (src, inv) = load(SPARSE);
    let plain = e(s::for_source(&legacy));
    writeln!(log, "legacy/plain\tfor_source={plain}").unwrap();
    for (form, env) in [
        ("receipt", { let mut b = legacy.clone(); b["retained_precision"] = src["retained_precision"].clone(); b }),
        ("null", { let mut b = legacy.clone(); b["retained_precision"] = Value::Null; b }),
        ("token", { let mut b = legacy.clone(); b["results"][0]["recovery_method"] = json!(rp::METHOD); b }),
    ] {
        let name = format!("legacy/{form}");
        let o = probe(&mut log, &name, &env, &inv);
        if plain == "OK" && o[0] != DG { failures.push(format!("{name}: for_source={}", o[0])); }
        if form != "token" && o[1] != DG { failures.push(format!("{name}: metadata={}", o[1])); }
        if o[4] == "OK" { failures.push(format!("{name}: derive admitted")); }
    }
    if let Ok(path) = std::env::var("RV88_REFUSALS_OUT") {
        std::fs::write(path, &log).unwrap();
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}
