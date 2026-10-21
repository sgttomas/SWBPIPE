//! Regenerate the design's committed-publication census through the actual W1 body.
//! Only the explicitly authorized DEF-O-bound hashes may differ from a prior
//! nodal publication. No numeric, work, evidence, verdict or publication field is masked.
use super::*;
#[path = "../../reporting/result_export/tests/support/retained_precision_corpus.rs"]
mod storage;

fn mode(value: &Value) -> PreviewSolverMode {
    match value.as_str().unwrap() {
        "sparse_interactive" => PreviewSolverMode::SparseInteractive,
        "dense_scrutiny" => PreviewSolverMode::DenseScrutiny,
        other => panic!("unknown mode {other}"),
    }
}

fn erase_authorized_definition_hashes(value: &mut Value) {
    let receipt = &mut value["retained_precision"];
    receipt["receipt_sha256"] = Value::Null;
    let body = &mut receipt["body"];
    for source in body["sources"].as_array_mut().unwrap() {
        if let Some(preparation) = source.get_mut("preparation") {
            preparation["sha256"] = Value::Null;
        }
        if let Some(operands) = source.get_mut("operands").and_then(Value::as_array_mut) {
            for operand in operands { operand["source_identity_sha256"] = Value::Null; }
        }
    }
    for case in body["cases"].as_array_mut().unwrap() {
        if case["status"] == "selected" { case["source_identity_sha256"] = Value::Null; }
    }
    if let Some(combinations) = body.get_mut("combinations").and_then(Value::as_array_mut) {
        for combination in combinations {
            if combination["disposition"] == "retained_selected" {
                combination["source_identity_sha256"] = Value::Null;
            }
        }
    }
}

thread_local! {
    /// The census entries whose live publication differs (beyond the authorized hashes) from the
    /// committed one; asserted empty once every entry has run (and, with `W1_CENSUS_OUT`, been written).
    static CENSUS_MISMATCHES: std::cell::RefCell<Vec<String>> = const { std::cell::RefCell::new(Vec::new()) };
}

fn reproduce(label: &str, old: &Value, invocation: &Value, hook: Option<&str>, exact: bool) -> Value {
    let solver = mode(&invocation["solver_mode"]);
    let raw = &invocation["request"];
    let (capture, observer, ordinary) = observed(solver, raw);
    let ordinary_bytes = serde_json::to_vec(&ordinary).unwrap();
    match hook {
        Some("b2_operand_preparation_failure") => hooks::fail_operand_preparation(1),
        Some("b2_operand_source_unavailable") => hooks::fail_preparation_of_case(1),
        None => (),
        _ => panic!("unrecognized controlled producer {label}"),
    }
    let ((envelope, retained), _, captured) = hooks::counted_with_successor(|| retained_w1(observer, ordinary, &capture));
    assert!(hooks::armed_names().is_empty(), "{label}: hook consumed");
    let current = captured.unwrap_or_else(|| panic!("{label}: no successor: {retained:?}"));
    if hook.is_none() {
        assert!(retained.is_ok(), "{label}: {retained:?}");
        assert_eq!(serde_json::to_vec(&envelope).unwrap(), ordinary_bytes, "{label}: ordinary owner");
    } else {
        assert!(matches!(retained, Err(W1Fallback::Precommit {gate:"G8", ..})), "{label}: unchanged controlled precommit refusal: {retained:?}");
    }
    // The regeneration writer emits the live publication before the comparison below decides.
    if let Ok(dir) = std::env::var("W1_CENSUS_OUT") {
        let name = label.replace('/', "__");
        let mut record = json!({"id":label,"source":current,"invocation":invocation,
            "successor_bytes_sha256":sha(&serde_json::to_vec(&current).unwrap())});
        if hook.is_some() {
            // A controlled precommit refusal publishes the ordinary envelope (the corpus records it).
            record["public_ordinary"] = serde_json::to_value(&envelope).unwrap();
        }
        std::fs::write(format!("{dir}/{name}.json"), serde_json::to_vec_pretty(&record).unwrap()).unwrap();
    }
    let mut before = old.clone();
    let mut after = current.clone();
    if !exact {
        erase_authorized_definition_hashes(&mut before);
        erase_authorized_definition_hashes(&mut after);
    }
    if serde_json::to_vec(&after).unwrap() != serde_json::to_vec(&before).unwrap() {
        CENSUS_MISMATCHES.with(|m| m.borrow_mut().push(label.to_owned()));
    }
    println!("W1_COMMITTED_CENSUS {label} source_sha256={} ordinary_sha256={} exact={exact}",
        sha(&serde_json::to_vec(&current).unwrap()),sha(&ordinary_bytes));
    current
}

#[test]
fn w1_committed_publications_preserve_all_but_authorized_definition_hashes() {
    let mut count = 0;
    for (name, bytes) in [
        ("milestone/sparse_interactive",include_str!("../../../fixtures/results/retained_precision_milestone_successor_sparse_interactive.json")),
        ("milestone/dense_scrutiny",include_str!("../../../fixtures/results/retained_precision_milestone_successor_dense_scrutiny.json")),
        ("l0/sparse_interactive",include_str!("../../../fixtures/results/retained_precision_l0_successor_sparse_interactive.json")),
        ("l0/dense_scrutiny",include_str!("../../../fixtures/results/retained_precision_l0_successor_dense_scrutiny.json")),
        ("w_c2/sparse_interactive",include_str!("../../../fixtures/results/retained_precision_w_c2_successor_sparse_interactive.json")),
        ("w_c2/dense_scrutiny",include_str!("../../../fixtures/results/retained_precision_w_c2_successor_dense_scrutiny.json")),
        ("combination/sparse_interactive",include_str!("../../../fixtures/results/retained_precision_combination_successor_sparse_interactive.json")),
        ("combination/dense_scrutiny",include_str!("../../../fixtures/results/retained_precision_combination_successor_dense_scrutiny.json")),
        ("exact/sparse_interactive",include_str!("../../../fixtures/results/retained_precision_exact_successor_sparse_interactive.json")),
        ("exact/dense_scrutiny",include_str!("../../../fixtures/results/retained_precision_exact_successor_dense_scrutiny.json")),
    ] {
        let document:Value=serde_json::from_str(bytes).unwrap();
        reproduce(&format!("fixture/{name}"),&document["source"],&document["invocation"],None,name.starts_with("exact/"));
        count+=1;
    }
    for (name, bytes) in [
        ("failed_ordinary_cxf",include_str!("../../../fixtures/results/retained_precision_failed_ordinary_cxf.json")),
        ("failed_ordinary_n06",include_str!("../../../fixtures/results/retained_precision_failed_ordinary_n06.json")),
    ] {
        let document:Value=serde_json::from_str(bytes).unwrap();
        for (i,case) in document["cases"].as_array().unwrap().iter().enumerate() {
            let invocation=json!({"request":case["request"],"solver_mode":case["solver_mode"]});
            reproduce(&format!("fixture/{name}/{i}/{}",case["solver_mode"].as_str().unwrap()),&case["successor"],&invocation,None,false);
            count+=1;
        }
    }
    let corpus=storage::expand(serde_json::from_str(include_str!("../../../fixtures/results/retained_precision_cases.json")).unwrap());
    for case in corpus["cases"].as_array().unwrap() {
        let kind=case["provenance"]["kind"].as_str();
        if !matches!(kind,Some("producer_solved"|"hook_produced")) {continue;}
        let id=case["id"].as_str().unwrap();
        // New member-load bases have their separate consuming controls. This is
        // the original nodal-only preservation census, whose membership is fixed.
        if id.starts_with("w1_member_") {continue;}
        // O4 (W1-E): the two dual-basis Timoshenko bases are new publications, held by
        // `o4_w1e_successor_fixtures_are_the_live_successors`; the census membership stays fixed.
        if id.starts_with("o4_w1e_") {continue;}
        let hook=if kind==Some("hook_produced") {Some(id)}else{None};
        reproduce(&format!("corpus/{id}"),&case["source"],&case["invocation"],hook,false);
        count+=1;
    }
    assert_eq!(count,43,"the complete original committed-publication census");
    let mismatches=CENSUS_MISMATCHES.with(|m| m.take());
    assert!(mismatches.is_empty(),"only authorized definition-bound hashes may move: {mismatches:?}");
}

/// Maintained producer writer for the normalized-unit, multi-member and
/// member-operand combination corpus bases. The full native reader consumes
/// each genuine successor before its bytes can be emitted.
#[test]
fn w1_member_load_multi_member_units_and_combination_successors() {
    use open_pipe_stress_result_export::retained_precision as reader;
    let mut raw=n05_two_cases();
    let original_pipe=raw["model"]["pipe_segments"][0]["id"].as_str().unwrap().to_owned();
    let provenance="invented_w1_member_load_reader_control_no_library_data";
    for (id,x) in [("aux-root",3.0),("aux-tip",4.0)] {
        let mut node=raw["model"]["nodes"][1].clone();
        node["id"]=json!(id);node["position"]["x"]=json!(3.0);node["position"]["y"]=json!(x-3.0);
        raw["model"]["nodes"].as_array_mut().unwrap().push(node);
        raw["model"]["supports"].as_array_mut().unwrap().push(json!({
            "id":format!("fixed:{id}"),"node":id,"family":"anchor",
            "restraints":["UX","UY","UZ","RX","RY","RZ"],
            "provenance":provenance}));
    }
    let mut auxiliary=raw["model"]["pipe_segments"][0].clone();
    auxiliary["id"]=json!("auxiliary");auxiliary["from"]=json!("aux-root");auxiliary["to"]=json!("aux-tip");
    auxiliary["y_reference"]=json!({"x":0.0,"y":0.0,"z":1.0});
    raw["model"]["pipe_segments"].as_array_mut().unwrap().push(auxiliary);
    for (index,force) in [(0,0.01),(1,-0.007)] {
        let case=&mut raw["model"]["load_cases"][index];
        let id=case["id"].as_str().unwrap().to_owned();
        for (pipe,axis,value) in [(original_pipe.as_str(),"global_x",force),("auxiliary","global_y",0.002)] {
            case["primitive_loads"].as_array_mut().unwrap().push(json!({
                "id":format!("{id}:udl:{pipe}:{axis}"),"category":"distributed_force",
                "target":{"type":"element","pipe":pipe},"direction":axis,
                "magnitude":{"value":value,"unit":"kN/m"},"dimension":"force_per_length","provenance":provenance}));
        }
    }
    for combining in [false,true] {
        let mut request=raw.clone();
        if combining {
            request["model"]["combinations"]=json!([{"id":"combination:members","label":"member operands",
                "basis":"mechanics","terms":[{"load_case":"case","factor":1.0},{"load_case":"case-2","factor":1.0}],
                "provenance":provenance}]);
        }
        for solver in MODES {
            let (capture,observer,ordinary)=observed(solver,&request);
            let ((_,retained),_,captured)=hooks::counted_with_successor(||retained_w1(observer,ordinary,&capture));
            let successor=retained.unwrap_or_else(|e| {
                let (_,observer,ordinary)=observed(solver,&request);
                let mut prepared=observer.prepare_cases(ordinary,2,&[0,1]).unwrap_or_else(|f|panic!("{:?}",f.error));
                prepared.native();prepared.freeze();
                let failures:Vec<_>=prepared.attempts.iter().map(|a|match &a.end {
                    rp::AttemptEnd::Candidate(f)=>format!("{}: {}",a.request,format!("{:?}",f.error).split(", work:").next().unwrap()),
                    rp::AttemptEnd::Frozen(_)=>format!("{}: Frozen",a.request),
                    _=>format!("{}: other",a.request),
                }).collect();
                panic!("combining={combining} {solver:?}: {e:?}: {failures:?}")
            }).value().clone();
            assert_eq!(captured,Some(successor.clone()));
            let invocation=json!({"request":request,"solver_mode":solver.as_str()});
            let verdict=reader::validate(&successor,Some(&invocation)).unwrap();
            assert!(verdict.invocation_bound);
            let body=&successor["retained_precision"]["body"];
            assert!(body["cases"].as_array().unwrap().iter().all(|c|c["status"]=="selected"));
            for source in body["sources"].as_array().unwrap().iter().filter(|s|s["owner"]["kind"]=="case") {
                assert_eq!(source["member_load_terms"].as_array().unwrap().len(),2);
            }
            if combining {
                let combination=&body["combinations"][0];
                assert_eq!(combination["disposition"],"retained_unavailable");
                assert!(serde_json::to_string(&combination["reason"]).unwrap().contains("operand_member_loads"));
            } else {assert!(verdict.numerical_eligible);}
            if let Ok(dir)=std::env::var("W1_MEMBER_CORPUS_OUT") {
                let kind=if combining {"combination"}else{"multi"};
                let id=format!("w1_member_{kind}_{}",solver.as_str());
                std::fs::write(format!("{dir}/{id}.json"),serde_json::to_string_pretty(&json!({
                    "id":id,"source":successor,"invocation":invocation})).unwrap()+"\n").unwrap();
            }
        }
    }
}

/// A plainly subnormal old annulus is declined by the existing source guard,
/// before the preparation helper. It cannot stand in for a Preparation input.
#[test]
fn w1_subnormal_annulus_is_declined_before_preparation() {
    let mut raw=n05_two_cases();
    let scale=2.0f64.powi(-256);
    raw["model"]["pipe_segments"][0]["section"]["outside_diameter"]["value"]=json!(0.2*scale);
    raw["model"]["pipe_segments"][0]["section"]["wall_thickness"]["value"]=json!(0.01*scale);
    for solver in MODES {
        let (capture,observer,ordinary)=observed(solver,&raw);
        let ids=w1_case_ids(&capture).unwrap();
        let triggers:Vec<_>=case_triggers(&ordinary.numerical_quality,&observer.ordinary,ids.ids()).collect();
        let attempted=ids.attempted(triggers.iter().copied());
        assert!(!attempted.requests().is_empty(),"{solver:?}: real ordinary trigger");
        let error=match observer.prepare_cases(ordinary,ids.ids().len(),attempted.requests()) {
            Err(failure)=>format!("{:?}",failure.error),
            Ok(_)=>panic!("{solver:?}: old subnormal source must refuse before preparation"),
        };
        assert!(error.contains("SubnormalDerivedPrimitive")&&error.contains("SecondMomentY"),"{solver:?}: {error}");
        println!("W1_SUBNORMAL_SOURCE {solver:?} section_scale=2^-256 error={error}");
    }
}

/// At this binary64 boundary the old section rounds I to the least normal,
/// while closed annulus preparation rounds it subnormal and refuses. Inputs
/// were found by a bounded independent exact-input circle-geometry probe.
#[test]
fn w1_annulus_boundary_is_a_real_preparation_fallback() {
    let mut raw=n05_two_cases();
    let diameter=f64::from_bits(0x300f_ffff_ffff_ffc2);
    let wall=f64::from_bits(0x2fc7_62b2_8a18_eb2c);
    raw["model"]["pipe_segments"][0]["section"]["outside_diameter"]["value"]=json!(diameter);
    raw["model"]["pipe_segments"][0]["section"]["wall_thickness"]["value"]=json!(wall);
    for solver in MODES {
        let (capture,mut observer,ordinary)=observed(solver,&raw);
        let old_i=observer.with_case(0,|c|c.source.as_ref().map(|s|s.members()[0].second_moment_y.to_bits()));
        assert_eq!(old_i,Some(f64::MIN_POSITIVE.to_bits()),"{solver:?}: old normal I");
        let ids=w1_case_ids(&capture).unwrap();
        let triggers:Vec<_>=case_triggers(&ordinary.numerical_quality,&observer.ordinary,ids.ids()).collect();
        let attempted=ids.attempted(triggers.iter().copied());
        assert!(!attempted.requests().is_empty(),"{solver:?}: ordinary trigger");
        let mut prepared=observer.prepare_cases(ordinary,ids.ids().len(),attempted.requests())
            .unwrap_or_else(|e|panic!("{solver:?}: custody {:?}",e.error));
        assert!(prepared.attempts.iter().all(|a|matches!(a.end,rp::AttemptEnd::Preparation)),"{solver:?}: Preparation");
        for &index in attempted.requests() {
            let error=prepared.capture.with_case(index,|c|format!("{:?}",c.error));
            assert!(error.contains("annulus preparation refused"),"{solver:?}: {error}");
        }
        let (capture,observer,ordinary)=observed(solver,&raw);
        let (_,result)=retained_w1(observer,ordinary,&capture);
        assert!(matches!(result,Err(W1Fallback::Preparation)),"{solver:?}: genuine fallback {result:?}");
        println!("W1_REAL_PREPARATION {solver:?} diameter={:016x} wall={:016x}",diameter.to_bits(),wall.to_bits());
    }
}

/// W1 coverage repair §2.2's gating check (corner 2): on every committed successor and
/// producer-solved or hook-produced corpus base (the hook-driven genuine-exhaustion
/// control only under `--features loaded-maximum-hooks`), the receipt's body scales, now the
/// final scales the product gate used, equal the committed ones (formerly native), body
/// by body and kind by kind, and the unchanged reader gives the regenerated document
/// exactly the verdict it gives the committed one (both pass, or both refuse at the same
/// gate and code). The census ran the same check over every census input where it is
/// observable (Design `w1-coverage-repair.md` §3.5).
#[test]
fn w1_final_scales_equal_reader_scales() {
    let scales = |s: &Value| -> Vec<Value> {
        let b = &s["retained_precision"]["body"];
        let mut v = Vec::new();
        for list in ["cases", "combinations"] {
            for item in b[list].as_array().into_iter().flatten() {
                if item.get("selection").is_some() { v.push(item["selection"]["body_scales"].clone()); }
            }
        }
        v
    };
    let mut checked = 0;
    let mut documents: Vec<(String, Value, Option<String>)> = Vec::new();
    for (name, bytes) in [
        ("milestone/sparse_interactive", include_str!("../../../fixtures/results/retained_precision_milestone_successor_sparse_interactive.json")),
        ("milestone/dense_scrutiny", include_str!("../../../fixtures/results/retained_precision_milestone_successor_dense_scrutiny.json")),
        ("l0/sparse_interactive", include_str!("../../../fixtures/results/retained_precision_l0_successor_sparse_interactive.json")),
        ("l0/dense_scrutiny", include_str!("../../../fixtures/results/retained_precision_l0_successor_dense_scrutiny.json")),
        ("w_c2/sparse_interactive", include_str!("../../../fixtures/results/retained_precision_w_c2_successor_sparse_interactive.json")),
        ("w_c2/dense_scrutiny", include_str!("../../../fixtures/results/retained_precision_w_c2_successor_dense_scrutiny.json")),
        ("combination/sparse_interactive", include_str!("../../../fixtures/results/retained_precision_combination_successor_sparse_interactive.json")),
        ("combination/dense_scrutiny", include_str!("../../../fixtures/results/retained_precision_combination_successor_dense_scrutiny.json")),
        ("exact/sparse_interactive", include_str!("../../../fixtures/results/retained_precision_exact_successor_sparse_interactive.json")),
        ("exact/dense_scrutiny", include_str!("../../../fixtures/results/retained_precision_exact_successor_dense_scrutiny.json")),
        ("o4_w1e/sparse_interactive", include_str!("../../../fixtures/results/retained_precision_o4_w1e_successor_sparse_interactive.json")),
        ("o4_w1e/dense_scrutiny", include_str!("../../../fixtures/results/retained_precision_o4_w1e_successor_dense_scrutiny.json")),
        ("o4_w1e_mixed/sparse_interactive", include_str!("../../../fixtures/results/retained_precision_o4_w1e_mixed_successor_sparse_interactive.json")),
    ] {
        let d: Value = serde_json::from_str(bytes).unwrap();
        documents.push((format!("fixture/{name}"), json!({"source": d["source"], "invocation": d["invocation"]}), None));
    }
    let corpus = storage::expand(serde_json::from_str(include_str!("../../../fixtures/results/retained_precision_cases.json")).unwrap());
    for case in corpus["cases"].as_array().unwrap() {
        let kind = case["provenance"]["kind"].as_str();
        if !matches!(kind, Some("producer_solved" | "hook_produced")) { continue; }
        let id = case["id"].as_str().unwrap().to_owned();
        let hook = (kind == Some("hook_produced")).then(|| id.clone());
        documents.push((format!("corpus/{id}"), case.clone(), hook));
    }
    for (label, d, hook) in documents {
        let invocation = &d["invocation"];
        let (capture, observer, ordinary) = observed(mode(&invocation["solver_mode"]), &invocation["request"]);
        match hook.as_deref() {
            Some("b2_operand_preparation_failure") => hooks::fail_operand_preparation(1),
            Some("b2_operand_source_unavailable") => hooks::fail_preparation_of_case(1),
            Some(other) if other.starts_with("w1_member_values_failure_") => hooks::fault_case(1, crate::retained_receipt::TraceFault::ValuesCompletion),
            Some(other) if other.starts_with("o4_w1e_unavailable_") => hooks::fail_freeze_of_case(1),
            _ => (),
        }
        // W1 coverage repair §1.4, option 2: the genuine-exhaustion control is hook-driven
        // at reduced search limits. Its hook exists only under `loaded-maximum-hooks`, so the
        // ordinary build cannot produce it and that command covers it.
        let exhaustion = hook.as_deref().is_some_and(|h| h.starts_with("w1_member_exhaustion_"));
        let run = || hooks::counted_with_successor(|| retained_w1(observer, ordinary, &capture));
        #[cfg(feature = "loaded-maximum-hooks")]
        let ((_, _), _, captured) = if exhaustion {
            open_pipe_stress_frame_kernel::structural::retained_api::loaded_maximum_hooks::with(W1_EXHAUSTION_CONTROL_LIMITS, run)
        } else { run() };
        #[cfg(not(feature = "loaded-maximum-hooks"))]
        let ((_, _), _, captured) = if exhaustion { continue } else { run() };
        let current = captured.unwrap_or_else(|| panic!("{label}: no successor"));
        assert_eq!(scales(&current), scales(&d["source"]), "{label}: final scales equal the committed scales");
        let verdict = |doc: &Value| match open_pipe_stress_result_export::retained_precision::validate(doc, Some(invocation)) {
            Ok(_) => "ok".to_owned(),
            Err(e) => format!("{} {}", e.gate, e.code),
        };
        let (now, committed) = (verdict(&current), verdict(&d["source"]));
        assert_eq!(now, committed, "{label}: the reader's verdict on the regenerated document");
        assert!(now == "ok" || !now.starts_with("G5b"), "{label}: no G5b refusal ({now})");
        checked += 1;
    }
    // 13 fixtures; the corpus's 44 producer-solved bases (the halfway and D = 0.2 m
    // controls among them); and its 7 hook-produced bases (b2 ×2, values-failure ×2,
    // o4_w1e_unavailable and the two exhaustion bases, which run only under
    // `loaded-maximum-hooks`).
    let exhaustion = if cfg!(feature = "loaded-maximum-hooks") { 2 } else { 0 };
    assert_eq!(checked, 13 + 49 + exhaustion, "every committed successor fixture and producer or hook corpus base");
}
