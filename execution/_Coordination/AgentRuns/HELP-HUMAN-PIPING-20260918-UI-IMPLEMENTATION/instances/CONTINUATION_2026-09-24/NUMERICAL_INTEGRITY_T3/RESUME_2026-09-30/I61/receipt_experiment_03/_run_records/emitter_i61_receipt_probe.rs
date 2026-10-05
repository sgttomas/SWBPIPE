//! I61 disposable projection experiment 03 (archive copy only; never maintained source).
//! The experiment-02 emitter, now driven from the actual facade
//! (`run_linear_static_preview_value_with_retained_direct`) with capture installed in the
//! single actual ordinary run behind a TEST-ONLY permit stub (ROOT decision 7: the stub
//! exists only in this disposable archive). The legacy RecoveryFailure is captured typed
//! (G-l) instead of being parsed from diagnostic text. No reader logic is used.
#![allow(dead_code, unused_variables)]
use super::*;
use super::retained_product as rp;
use super::retained_receipt as rr;
use open_pipe_stress_canonical_json::canonical_json_checked_v1_text;
use open_pipe_stress_frame_kernel::structural::retained_api as k;
use serde_json::{json, Map, Value};
use sha2::{Digest, Sha256};

const REQUEST: &str = include_str!("../../../fixtures/product_preview/rf_skew_t_cant_off_122_r1e-04.request.json");
const DEFINITION_ID: &str = "RP-PREPARED-ORDINARY-DUAL-v1";
const METHOD: &str = "contribution_preserving_multiprecision_v1";
const RETAINED_ID: &str = "openpipestress.result_semantics/0.3.0/preview-physics-retained-1";
const RETAINED_PROFILE: &str = "product_preview_retained_w1a_v2";

// ---- TEST-ONLY permit stub (decision 7; disposable archive only) ----
thread_local! { static PERMIT: std::cell::Cell<bool> = const { std::cell::Cell::new(false) }; }
pub(super) fn test_permit() -> bool { PERMIT.with(|p| p.get()) }
fn with_permit<T>(f: impl FnOnce() -> T) -> T {
    PERMIT.with(|p| p.set(true));
    let out = f();
    PERMIT.with(|p| p.set(false));
    out
}

/// What the facade's permit path returns besides the (ordinary) envelope.
pub(crate) struct FacadeSuccessor {
    /// Serialized bytes of the actual single ordinary run, taken before preparation.
    pub ordinary_bytes: Vec<u8>,
    /// The typed G-l capture as observed (None when the legacy Err arm never ran).
    pub legacy: Option<rp::LegacyCapture>,
    /// The successor envelope and its provenance, when a candidate was certified.
    pub probe: Option<Probe>,
    /// The route actually taken by the facade.
    pub outcome: String,
}

/// The facade's W1 completion window (experiment shape of U3): coexistence bypass, then
/// preparation, native, proof and emission on the capture taken from the single ordinary run.
pub(super) fn facade_w1(mut observer: rp::ProductCapture, ordinary: MechanicsEnvelope,
    capture: &source_receipt::CapturedInvocation, mode: PreviewSolverMode) -> (MechanicsEnvelope, FacadeSuccessor) {
    let ordinary_bytes = serde_json::to_vec(&ordinary).unwrap();
    let mut legacy = observer.legacy_failure.take();
    // Experiment-local negative controls for G-l (selected by env; unset in the receipt runs).
    match std::env::var("I61_MUTANT").as_deref() {
        Ok("gl_charged") => if let Some(l) = legacy.as_mut() { l.charged += 1 },
        Ok("gl_helper") => if let Some(l) = legacy.as_mut() { l.helper_stage = "solve" },
        Ok("gl_drop") => legacy = None,
        Ok("gl_unattempted") => if let Some(l) = legacy.as_mut() { l.attempted = false },
        Ok(other) => panic!("unknown I61_MUTANT {other}"),
        Err(_) => {}
    }
    let mut succ = FacadeSuccessor { ordinary_bytes, legacy: legacy.clone(), probe: None, outcome: String::new() };
    // Coexistence rule: an exact source-block publication on the ordinary route is never displaced.
    if ordinary.source_block_recovery.is_some() {
        succ.outcome = "bypass:source_block_recovery".into();
        return (ordinary, succ);
    }
    let kept = ordinary.clone();
    let mut prepared = match observer.prepare_case(ordinary) {
        Ok(p) => p,
        Err(f) => { succ.outcome = format!("w1_unavailable:preparation:{:?}", f.capture.error); return (kept, succ); }
    };
    if let Err(e) = prepared.solve_native() { succ.outcome = format!("w1_unavailable:native:{e:?}"); return (kept, succ); }
    let candidate = match prepared.project_candidate() {
        Ok(c) => c,
        Err(e) => { succ.outcome = format!("w1_unavailable:candidate:{:?}", e.error); return (kept, succ); }
    };
    succ.probe = Some(emit_selected(&candidate, capture.borrowed_raw(), capture, mode, legacy.as_ref()));
    succ.outcome = "selected".into();
    (kept, succ)
}

fn bits(x: f64) -> Value { json!(format!("{:016x}", x.to_bits())) }
fn ubits(x: u64) -> Value { json!(format!("{:016x}", x)) }
fn sha_hex(bytes: &[u8]) -> String { format!("{:x}", Sha256::digest(bytes)) }
fn h(domain: &str, payload: &Value) -> String {
    let text = serde_json::to_string(&json!({"domain":domain,"payload":payload})).unwrap();
    let canon = canonical_json_checked_v1_text(&text).expect("canonical");
    sha_hex(canon.as_bytes())
}
fn fault_name(f: k::WorkFault) -> &'static str { match f { k::WorkFault::Overflow=>"overflow", k::WorkFault::Inconsistent=>"inconsistent", k::WorkFault::Both=>"both" } }
fn count(w: k::WorkTotal) -> Value { match w.exact() { Ok(v)=>json!({"kind":"exact","value":v}), Err(f)=>json!({"kind":"unavailable","fault":fault_name(f)}) } }
fn exact(w: k::WorkTotal, what: &str) -> u64 { w.exact().unwrap_or_else(|f| panic!("inexact {what}: {f:?}")) }
fn sticky(s: k::WorkStatus) -> &'static str { match s.fault() { None=>"exact", Some(f)=>fault_name(f) } }
fn kind_name(kind: k::Kind) -> &'static str { match kind { k::Kind::Translation=>"translation", k::Kind::Rotation=>"rotation", k::Kind::Force=>"force", k::Kind::Moment=>"moment" } }
fn comp(c: k::Component) -> &'static str { ["UX","UY","UZ","RX","RY","RZ"][c.index()] }
fn dof(d: k::Dof) -> Value { json!({"node":d.node,"component":comp(d.component)}) }
fn stages(s: &k::StageWork) -> Value {
    json!({"formation":s.formation,"assembly":s.assembly,"residual_formation":s.residual_formation,"factor":s.factor,
        "condition":s.condition,"rhs":s.rhs,"solve":s.solve,"refinement":s.refinement,"recovery":s.recovery,"stop_rule":s.stop_rule,
        "bounded_gate":s.bounded_gate,"scale":s.scale,"estimate":s.estimate,"charge":s.charge,"bound":s.bound,"shift":s.shift,
        "bounded_formation":s.bounded_formation,"wide_formation":s.wide_formation,"uc":s.uc})
}
fn numeric(n: &k::NumericTrace) -> Value {
    json!({"wide_lme":count(n.wide_lme),"exact_sum_lme":count(n.exact_sum_lme),"entries":n.entries.iter().map(|e|count(*e)).collect::<Vec<_>>(),
        "f64_arithmetic":count(n.f64_arithmetic),"sticky_status":sticky(n.sticky_status)})
}
fn conversion(o: &k::Binary64Outcome) -> Value {
    match *o {
        k::Binary64Outcome::Normal(v)=>json!({"kind":"normal","value":bits(v)}),
        k::Binary64Outcome::Subnormal{value,relative_precision}=>json!({"kind":"subnormal","value":bits(value),"relative_precision":bits(relative_precision)}),
        k::Binary64Outcome::Underflow{negative}=>json!({"kind":"underflow","negative":negative}),
        k::Binary64Outcome::Overflow{negative}=>json!({"kind":"overflow","negative":negative}),
    }
}
fn quantity(q: k::QuantityId) -> Value {
    match q {
        k::QuantityId::Displacement(d)=>json!({"tag":"displacement","dof":dof(d)}),
        k::QuantityId::Reaction(d)=>json!({"tag":"reaction","dof":dof(d)}),
        k::QuantityId::DisplacementMagnitude(n)=>json!({"tag":"displacement_magnitude","node":n}),
        k::QuantityId::EndAction{member,end,component}=>json!({"tag":"end_action","member":member,"end":match end{k::End::I=>"i",k::End::J=>"j"},"component":comp(component)}),
        k::QuantityId::StationAction{station,component}=>json!({"tag":"station_action","station":station,"component":comp(component)}),
        k::QuantityId::SpringAction{spring,component}=>json!({"tag":"spring_action","spring":spring,"component":comp(component)}),
        k::QuantityId::DirectionalSpringAction{..}=>panic!("directional spring outside ordinary scope"),
        k::QuantityId::SupportForceMagnitude(s)=>json!({"tag":"support_force_magnitude","support":s}),
        k::QuantityId::SupportMomentMagnitude(s)=>json!({"tag":"support_moment_magnitude","support":s}),
    }
}
fn outcome(o: &k::AttemptOutcome) -> Value {
    match o {
        k::AttemptOutcome::Accepted=>json!({"kind":"accepted"}), k::AttemptOutcome::Verified=>json!({"kind":"verified"}),
        k::AttemptOutcome::Solved=>json!({"kind":"solved"}),
        k::AttemptOutcome::Rejected(r)=>json!({"kind":"rejected","reason":format!("UNMAPPED {r:?}")}),
        k::AttemptOutcome::Failed(r)=>json!({"kind":"failed","reason":format!("UNMAPPED {r:?}")}),
    }
}
fn slot_name(s: k::OriginSlot) -> &'static str {
    match s { k::OriginSlot::S128=>"s128",k::OriginSlot::S256=>"s256",k::OriginSlot::S512=>"s512",k::OriginSlot::S1024=>"s1024",
        k::OriginSlot::V256=>"v256",k::OriginSlot::V512=>"v512",k::OriginSlot::V1024=>"v1024" }
}
fn slots(s: &k::SlotSnapshot) -> Value {
    Value::Array(k::OriginSlot::ALL.iter().zip(s.iter()).filter_map(|(slot,b)|b.map(|b|json!({"slot":slot_name(*slot),"build":b}))).collect())
}
fn scalar(w: &rp::ScalarWork) -> Value { json!({"entered":w.entered,"checks":w.checks,"lost":w.lost}) }
fn op_name(o: rp::ScalarOperation) -> &'static str { match o { rp::ScalarOperation::Add=>"add",rp::ScalarOperation::Sub=>"sub",rp::ScalarOperation::Mul=>"mul",rp::ScalarOperation::Div=>"div",rp::ScalarOperation::Sqrt=>"sqrt" } }
fn op_error(e: &rp::OperationalError) -> Value {
    match e {
        rp::OperationalError::MissingOrForeign=>json!({"kind":"missing_or_foreign"}), rp::OperationalError::Input=>json!({"kind":"input"}),
        rp::OperationalError::Degenerate=>json!({"kind":"degenerate"}), rp::OperationalError::Accounting=>json!({"kind":"accounting"}),
        rp::OperationalError::NonFinite{operation,entered}=>json!({"kind":"non_finite","operation":op_name(*operation),"entered":entered}),
        rp::OperationalError::CoefficientRange{coefficient,operation}=>json!({"kind":"coefficient_range","coefficient":coefficient,"operation":op_name(*operation)}),
    }
}
fn operational(o: &rp::OperationalSpent) -> Value {
    let result = match &o.result {
        Ok(v)=>json!({"kind":"ready","length":bits(v.length),"axial_stiffness":bits(v.axial),"torsional_stiffness":bits(v.torsion),
            "normalization":v.normalization_check.iter().map(|x|bits(*x)).collect::<Vec<_>>()}),
        Err(e)=>json!({"kind":"refused","error":op_error(e)}),
    };
    json!({"member":o.member.expect("member ordinal"),"inputs":o.inputs.iter().map(|x|ubits(*x)).collect::<Vec<_>>(),"result":result,"work":scalar(&o.work)})
}
const EVENTS: [&str;10]=["source_visit","row_visit","map_write","validation_entry","identity_byte_read","key_probe","allocation_request","library_boundary","requested_copy_bytes","rust_capacity_bytes"];
fn section_error(e: &k::SectionPreparationError) -> Value {
    const P:[&str;5]=["area","second_moment","polar_moment","section_modulus","radius"];
    match e {
        k::SectionPreparationError::InvalidGeometry=>json!({"kind":"invalid_geometry"}),
        k::SectionPreparationError::Accounting=>json!({"kind":"accounting"}),
        k::SectionPreparationError::AmbiguousRounding(p)=>json!({"kind":"ambiguous_rounding","property":P[*p]}),
        k::SectionPreparationError::PrimitiveRange(p)=>json!({"kind":"primitive_range","property":P[*p]}),
        k::SectionPreparationError::Arithmetic(c)=>json!({"kind":"arithmetic","cause":format!("UNMAPPED {:?}",c.cause())}),
    }
}
fn capture_error(e: &rp::CaptureError) -> Value {
    match e {
        rp::CaptureError::Association(s)=>json!({"kind":"association","detail":s}),
        rp::CaptureError::CountRange(s)=>json!({"kind":"count_range","detail":s}),
        rp::CaptureError::Storage(s)=>json!({"kind":"storage","detail":s}),
        rp::CaptureError::Accounting(rp::AdapterFault::Overflow(ev))=>json!({"kind":"accounting","event":EVENTS[*ev as usize]}),
        rp::CaptureError::NativeUnavailable=>json!({"kind":"native_unavailable"}),
        rp::CaptureError::PreparedAttemptConsumed=>json!({"kind":"prepared_attempt_consumed"}),
        rp::CaptureError::PreparedArithmetic(o)=>json!({"kind":"prepared_arithmetic","cause":op_error(o)}),
        other=>json!(format!("UNMAPPED {other:?}")),
    }
}
fn stage_name(s: rr::StageState) -> &'static str { match s { rr::StageState::NotEntered=>"not_entered", rr::StageState::Completed=>"completed", rr::StageState::Failed=>"failed", rr::StageState::Entered=>"ENTERED_UNREPRESENTABLE" } }

/// The producer's actual result for one request through the private prepared driver.
pub(super) struct Probe {
    pub source: Value,
    pub invocation: Value,
    pub prov: Vec<(String,String)>,
    pub peak: String,
    pub verdicts: Vec<String>,
}
fn prov(p: &mut Vec<(String,String)>, path: &str, from: &str) { p.push((path.into(), from.into())); }

/// Envelope transformation (provisional, experiment-local): the readers' successor identity,
/// row tokens on selected rows, and one RETAINED_PRECISION_* diagnostic per case.
fn successor_envelope(mut env: Value, case_id: &str, selected: bool, prov_: &mut Vec<(String,String)>) -> (Value, String, Option<Value>) {
    env["producer"]["semantic_contract_id"] = json!(RETAINED_ID);
    env["formulation_basis"]["profile_id"] = json!(RETAINED_PROFILE);
    prov(prov_, "envelope.producer.semantic_contract_id", "CONTRACT-ONLY: semantic table id (READER fixture); producer has no constant");
    prov(prov_, "envelope.formulation_basis.profile_id", "CONTRACT-ONLY: table formulation_profile_id; limitations left as produced");
    if selected {
        for row in env["results"].as_array_mut().unwrap() {
            if row["basis_ref"]["ref_id"] == json!(case_id) { row["recovery_method"] = json!(METHOD); }
        }
        prov(prov_, "envelope.results[].recovery_method", "CONTRACT-ONLY: METHOD token on every row of the selected case");
    }
    // T1 option (a), owner-confirmed (ROOT ruling "T1 confirmed by the owner"): on a
    // retained-selected case the successor omits the legacy disclosure naming it.
    let mut legacy_omitted = None;
    if selected {
        let diags = env["diagnostics"].as_array_mut().unwrap();
        if let Some(i) = diags.iter().position(|d| d["code"]=="SOURCE_BLOCK_RECOVERY_UNAVAILABLE"
            && d["affected_refs"].as_array().is_some_and(|a| a.iter().any(|r| r==case_id))) {
            legacy_omitted = Some(diags.remove(i));
            assert!(!diags.iter().any(|d| d["code"]=="SOURCE_BLOCK_RECOVERY_UNAVAILABLE"
                && d["affected_refs"].as_array().is_some_and(|a| a.iter().any(|r| r==case_id))), "single legacy disclosure");
        }
        prov(prov_, "envelope.diagnostics[-SOURCE_BLOCK_RECOVERY_UNAVAILABLE]", "T1 (a): omitted on the retained-selected case (owner-confirmed); unselected/fallback bytes unchanged");
    }
    let id = format!("diagnostic:retained-precision:{case_id}");
    let (code, severity, message) = if selected {
        ("RETAINED_PRECISION_SELECTED","info","I61 disposable experiment: prepared private producer statement; not a public publication.")
    } else {
        ("RETAINED_PRECISION_UNAVAILABLE","warning","I61 disposable experiment: retained precision unavailable for this case.")
    };
    env["diagnostics"].as_array_mut().unwrap().push(json!({"id":id,"code":code,"severity":severity,"message":message,"source":"core/product_physics","affected_refs":[case_id]}));
    prov(prov_, "envelope.diagnostics[+RETAINED_PRECISION_*]", "EXPERIMENT TEXT: id/message/severity chosen by the emitter (no producer or table text)");
    (env, id, legacy_omitted)
}

/// G-l: the typed legacy RecoveryFailure/WorkReport captured at its actual site.
fn legacy_work(case_index: usize, l: &rp::LegacyCapture) -> Value {
    json!({"case_index":case_index,"stage":l.stage,"helper_stage":l.helper_stage,"charged":l.charged,"rejected":l.rejected,"limit":l.limit,"settlement":"booked"})
}
/// Experiment-02 message parse, kept ONLY as a cross-check of the typed capture (never emitted).
fn legacy_work_from_message(case_index: usize, message: &str) -> Value {
    let after = |key: &str| -> &str { let i = message.find(key).unwrap_or_else(|| panic!("{key} in {message}")) + key.len(); &message[i..] };
    let stage = { let t = after("stage: \""); &t[..t.find('"').unwrap()] };
    let helper = { let t = after("helper_stage: "); &t[..t.find(|c: char| c==',' || c==' ').unwrap()] };
    let helper_snake: String = helper.chars().enumerate().flat_map(|(i,c)| if c.is_uppercase() && i>0 {vec!['_', c.to_ascii_lowercase()]} else {vec![c.to_ascii_lowercase()]}).collect();
    let work = after("WorkReport {");
    let num = |key: &str| -> u64 { let i = work.find(key).unwrap() + key.len(); work[i..].trim_start().split(|c: char| !c.is_ascii_digit()).next().unwrap().parse().unwrap() };
    json!({"case_index":case_index,"stage":stage,"helper_stage":helper_snake,"charged":num("charged:"),"rejected":num("rejected:"),"limit":num("limit:"),"settlement":"booked"})
}
fn ordinary_attempt(env: &Value, case_index: usize, case_id: &str, mode: PreviewSolverMode, legacy_omitted: Option<&Value>, typed: Option<&rp::LegacyCapture>, work: &mut Vec<Value>, prov_: &mut Vec<(String,String)>) -> Value {
    // D6a reading (assumption A2): the case's own ordinary diagnostics are those naming it.
    let diags: Vec<&Value> = env["diagnostics"].as_array().unwrap().iter()
        .filter(|d| d["affected_refs"].as_array().is_some_and(|a| a.iter().any(|r| r==case_id)))
        .filter(|d| !d["code"].as_str().unwrap().starts_with("RETAINED_PRECISION_")).collect();
    let refs: Vec<Value> = diags.iter().map(|d| d["id"].clone()).collect();
    let quality = env["numerical_quality"]["cases"].as_array().unwrap().iter().find(|q| q["basis_ref"]["ref_id"]==case_id).cloned().unwrap_or(Value::Null);
    let solve = quality["solve_quality"].as_str().unwrap_or("");
    let report = quality["evidence_refs"].as_array().and_then(|a| a.first()).cloned().unwrap_or(Value::Null);
    let initial = match solve {
        "sensitive"|"checks_passed" => json!({"kind":"report","report_diagnostic_ref":report,"outcome":solve}),
        other => json!({"kind":"report","report_diagnostic_ref":report,"outcome":format!("UNMAPPED {other}")}),
    };
    prov(prov_, "ordinary_attempts[].initial", "PRODUCER GAP: read back from envelope numerical_quality.cases[].solve_quality/evidence_refs (no typed capture)");
    prov(prov_, "ordinary_attempts[].w2/formation", "PRODUCER GAP: no typed capture; emitted as not_triggered/null after checking the envelope has no W2/K-D5 diagnostics");
    let codes: Vec<&str> = env["diagnostics"].as_array().unwrap().iter().map(|d| d["code"].as_str().unwrap()).collect();
    println!("I61_ORDINARY_DIAGNOSTICS {codes:?}");
    assert!(!codes.iter().any(|c| c.contains("W2") || c.contains("FORMATION_CHECK")), "unmapped ordinary diagnostics present: {codes:?}");
    // The actual ordinary route declined legacy source-block recovery for this case.
    let legacy = env["diagnostics"].as_array().unwrap().iter().find(|d| d["code"]=="SOURCE_BLOCK_RECOVERY_UNAVAILABLE"
        && d["affected_refs"].as_array().is_some_and(|a| a.iter().any(|r| r==case_id)));
    // G-l: the disposition and work come from the typed capture; the disclosure text is only
    // cross-checked. A typed capture without an actual attempt is not mapped here (a contract
    // reading would be needed); the milestone does not reach it.
    let legacy_v = match (typed, legacy, legacy_omitted) {
        (Some(l), None, Some(omitted)) if l.attempted => {
            // T1 (a): selected case — the disclosure is omitted; the work goes to legacy_source_work[k].
            assert_eq!(l.case, case_id);
            let w = legacy_work(case_index, l);
            assert_eq!(w, legacy_work_from_message(case_index, omitted["message"].as_str().unwrap()), "typed G-l equals the disclosed failure");
            work.push(w);
            json!({"disposition":"unavailable","diagnostic_ref":null,"work_ref":work.len()-1})
        }
        (Some(l), Some(d), None) if l.attempted => {
            assert_eq!(l.case, case_id);
            let w = legacy_work(case_index, l);
            assert_eq!(w, legacy_work_from_message(case_index, d["message"].as_str().unwrap()), "typed G-l equals the disclosed failure");
            work.push(w);
            json!({"disposition":"unavailable","diagnostic_ref":d["id"],"work_ref":work.len()-1})
        }
        (None, None, None) => json!({"disposition":"not_eligible","diagnostic_ref":null,"work_ref":null}),
        other => panic!("UNMAPPED legacy outcome (contract reading needed): typed={:?} kept={} omitted={}", other.0, other.1.is_some(), other.2.is_some()),
    };
    prov(prov_, "ordinary_attempts[].legacy_source", "T1 (a): selected case {unavailable, null, work_ref}; unselected keeps diagnostic_ref. PRODUCER (G-l): typed LegacyCapture from the Err(failure) arm (archive-only cfg(test) field)");
    prov(prov_, "body.legacy_source_work[]", "PRODUCER (G-l): typed RecoveryFailure{stage, helper_stage, work: WorkReport{charged, rejected, limit}} captured at the actual site; message text only cross-checked; settlement booked (T1 ruling)");
    prov(prov_, "ordinary_attempts[].diagnostic_refs", "ASSUMPTION A2 (D6a): diagnostics whose affected_refs name the case, excluding RETAINED_PRECISION_*");
    json!({"case_index":case_index,"case_id":case_id,"material_basis_ref":0,"requested_mode":mode.as_str(),"initial":initial,
        "w2":{"kind":"not_triggered"},"formation":{"load_row_finding":null,"d5_diagnostic_ref":null},
        "legacy_source":legacy_v,"diagnostic_refs":refs})
}

fn material_basis(capture: &rp::ProductCapture, req: &Value, prov_: &mut Vec<(String,String)>) -> Value {
    // Base selector: the milestone case names no modulus basis (checked against the actual request).
    let case = &req["model"]["load_cases"][0];
    assert!(case.get("modulus_basis_ref").map_or(true,|v|v.is_null()) && case.get("modulus_basis_temperature").map_or(true,|v|v.is_null()));
    let inputs: Vec<Value> = req.get("materials").and_then(|m|m.as_array()).filter(|m|!m.is_empty()).cloned()
        .unwrap_or_else(|| req["model"]["materials"].as_array().cloned().unwrap_or_default());
    let mut used: Vec<usize> = capture.members.iter().map(|m| m.material).collect(); used.sort(); used.dedup();
    let materials: Vec<Value> = used.iter().map(|&i| {
        let (id,e,g) = &capture.materials[i];
        assert_eq!(inputs[i]["id"].as_str().unwrap(), id);
        json!({"input_index":i,"id":id,"elastic_modulus":bits(*e),"shear_modulus":bits(*g),"shear_origin":{"kind":"explicit_g"},"selection":{"kind":"base"}})
    }).collect();
    prov(prov_, "material_bases[]", "PRODUCER: ProductCapture.materials (id,E,G) and members[].material; selector base from the actual request; shear_origin explicit_g (request has G)");
    json!({"index":0,"selector":{"kind":"base"},"materials":materials,"case_indices":[0]})
}

fn case_source(capture: &rp::ProductCapture, owner: &k::RetainedSolve, inv: &k::RecordedInvocation, run: &k::RunOrigins,
    req: &Value, case_id: &str, attempt: &rr::PreparedAttemptView<'_>, prov_: &mut Vec<(String,String)>) -> Value {
    let source = owner.source();
    let origin = &inv.sources()[run.source];
    let nodes: Vec<Value> = capture.nodes.iter().enumerate().map(|(i,(id,c))| {
        assert_eq!(source.nodes()[i], *c);
        json!({"model_index":i,"kernel_node":i,"id":id,"coordinates":c.iter().map(|x|bits(*x)).collect::<Vec<_>>()})
    }).collect();
    let members: Vec<Value> = capture.members.iter().enumerate().map(|(i,m)| {
        let km = &source.members()[i];
        json!({"model_index":m.model,"built_pipe_index":m.built,"kernel_member":km.id,"node_i":km.node_i,"node_j":km.node_j,
            "material_index":m.material,"id":m.id,"y_reference":km.y_reference.iter().map(|x|bits(*x)).collect::<Vec<_>>(),
            "E":bits(km.elastic_modulus),"G":bits(km.shear_modulus),"A_K":bits(km.area),"Iy_K":bits(km.second_moment_y),
            "Iz_K":bits(km.second_moment_z),"J_K":bits(km.torsion_constant)})
    }).collect();
    let springs: Vec<Value> = capture.spring_map.iter().enumerate().map(|(i,s)| {
        let ks = &source.springs()[i];
        json!({"boundary_index":s.boundary,"kernel_spring":ks.id,"support_index":s.support,"node":ks.dof.node,"component":comp(ks.dof.component),"stiffness":bits(ks.stiffness)})
    }).collect();
    let support_ids: Vec<Value> = capture.supports.iter().enumerate().map(|(i,(id,node))| json!({"model_index":i,"kernel_support":source.supports().get(i).map(|s|s.id).unwrap_or(u32::MAX),"id":id,"node":node})).collect();
    prov(prov_, "sources[].id_maps", "PRODUCER: ProductCapture nodes/members/spring_map/supports joined with the bound kernel PrimitiveSource (owner.source())");
    let body_membership: Vec<Value> = (0..source.body_count()).map(|b| {
        let ns = source.body_nodes(b);
        let ms: Vec<u32> = source.members().iter().filter(|m| source.body_of_node(m.node_i)==b).map(|m|m.id).collect();
        json!({"body":b,"nodes":ns,"members":ms})
    }).collect();
    let layout: Vec<Value> = k::layout(source).iter().enumerate().map(|(i,m)| json!({"index":i,"quantity":quantity(m.id),"kind":kind_name(m.kind),"body":m.body,"input_derived":m.input_derived})).collect();
    prov(prov_, "sources[].body_membership/layout", "PRODUCER: PrimitiveSource body maps; recover::layout(owner.source())");
    let stations: Vec<Value> = source.stations().iter().map(|s| {
        let loc = if s.fraction==0.25 {"quarter_1"} else if s.fraction==0.5 {"midspan"} else if s.fraction==0.75 {"quarter_3"} else {"UNMAPPED"};
        json!({"id":s.id,"member":s.member,"location":loc,"fraction":bits(s.fraction)})
    }).collect();
    let supports: Vec<Value> = source.supports().iter().map(|s| json!({"id":s.id,"node":s.node,"restrained":s.restrained,"springs":s.springs,"directional_springs":s.directional_springs})).collect();
    let constraints: Vec<Value> = source.constraints().iter().map(|c| {
        let idx: Vec<usize> = source.supports().iter().enumerate().filter(|(_,s)| s.node==c.dof.node && s.restrained[c.dof.component.index()]).map(|(i,_)|i).collect();
        json!({"dof":dof(c.dof),"value":bits(c.value),"support_indices":idx})
    }).collect();
    prov(prov_, "sources[].constraints[].support_indices", "DERIVED: supports whose node/restraint covers the constrained DOF (no producer map)");
    let nodal_terms: Vec<Value> = source.loads().iter().enumerate().map(|(i,l)| {
        let t = capture.terms.iter().find(|t| t.canonical==i).expect("term map");
        json!({"constructor_ordinal":i,"source_id":l.source_id,"primitive_load_index":t.original,"dof":dof(l.dof),"value":bits(l.value)})
    }).collect();
    prov(prov_, "sources[].nodal_terms", "PRODUCER: PrimitiveSource loads + ProductCapture.terms (canonical->original)");
    let section_terms: Vec<Value> = capture.facts.iter().enumerate().map(|(i,f)| {
        let op = attempt.operational_new.get(i).and_then(|o|o.result.as_ref().ok());
        let (length,axial,torsion) = op.map(|o|(bits(o.length),bits(o.axial),bits(o.torsion))).unwrap_or((json!("UNMAPPED"),json!("UNMAPPED"),json!("UNMAPPED")));
        json!({"member":i,"area":bits(f.area),"section_modulus":bits(f.section_modulus),"length":length,"axial_stiffness":axial,"torsional_stiffness":torsion,
            "geometry":{"route":"preview","normalized_od":bits(f.diameter),"effective_wall":bits(f.effective_wall),"actual_radius":bits(f.radius),
                "actual_second_moment":bits(f.second_moment),"actual_polar_moment":bits(f.torsion_constant)}})
    }).collect();
    prov(prov_, "sources[].section_terms", "PRODUCER: ProductCapture.facts (prepared facts) + new operational (length/axial/torsion); geometry from facts");
    let mut s = json!({"index":0,"owner":{"kind":"case","case_index":0,"case_id":case_id},"material_basis_ref":0,
        "kernel_source_sha256":sha_hex(&origin.identity),"stiffness_sha256":sha_hex(&origin.stiffness),
        "id_maps":{"nodes":nodes,"members":members,"springs":springs,"support_ids":support_ids},"body_membership":body_membership,
        "layout":layout,"stations":stations,"supports":supports,"constraints":constraints,"nodal_terms":nodal_terms,"section_terms":section_terms,
        "preparation":null});
    prov(prov_, "sources[].kernel_source_sha256/stiffness_sha256", "PRODUCER: raw SHA256 of SourceOrigin.identity (K4SRC) / .stiffness (K4STF)");
    s
}

fn physical(i: usize, r: &k::AttemptRecord, links: k::RecordBuildLinks) -> Value {
    let w = exact(r.work.checked_lme(),"W"); let kk = exact(r.k4_work.checked_lme(),"K");
    let gate = match r.gate { None=>Value::Null, Some(k::GateTest::Coalesced)=>json!({"kind":"coalesced"}),
        Some(k::GateTest::Bounded{state,evaluated})=>json!({"kind":"bounded","state":state,"evaluated":evaluated}) };
    let verification = r.verification.as_ref().map(|v| json!({
        "resolution":v.resolution.iter().enumerate().map(|(b,e)|json!({"body":b,"force":bits(e[0]),"moment":bits(e[1])})).collect::<Vec<_>>(),
        "theta":v.theta.iter().enumerate().map(|(b,t)|json!({"body":b,"value":bits(*t)})).collect::<Vec<_>>(),
        "bound":v.bound.iter().enumerate().map(|(b,x)|json!({"body":b,"value":x.map(bits)})).collect::<Vec<_>>(),
        "data_blocks":v.data_blocks,"shift_factorizations":v.shift_factorizations,"g_max":v.g_max,"uc_missing":v.uc_missing,"g_violation":v.g_violation})).unwrap_or(Value::Null);
    let role = match r.role { k::AttemptRole::Candidate=>"candidate", k::AttemptRole::Verification=>"verification", k::AttemptRole::VerificationThenCandidate=>"verification_then_candidate" };
    assert!(r.bound_refusals.is_empty(), "bound refusals not mapped in this experiment");
    json!({"index":i,"precision":r.precision,"role":role,"outcome":outcome(&r.outcome),"residual_basis":r.residual_basis,"corrections":r.corrections,
        "pivot_margin_min":r.pivot_margin_min.map(bits),"rcond":r.rcond.map(bits),"residual_worst":r.residual_worst.map(bits),"gate":gate,
        "work":{"wide_lme":w,"exact_sum_lme":kk,"own_lme":w+kk,"shared_lme":r.shared_work,"stop_rule_lme":r.stop_rule_work,
            "verification_lme":r.verification_work,"verification_shared_lme":r.verification_shared_work,
            "own_stages":stages(&r.stages),"shared_stages":stages(&r.shared_stages),
            "shared_built_here":r.shared_built_here,"verification_shared_built_here":r.verification_shared_built_here},
        "storage":{"pattern_entries":r.storage.pattern_entries,"profile_entries":r.storage.profile_entries,"limbs_per_entry":r.storage.limbs_per_entry},
        "verification":verification,"bound_refusals":[],"shared_build_ref":links.shared,"verification_shared_build_ref":links.verification_shared})
}
/// C1 §1 projection (RP-LOGICAL-ATTEMPTS-v1), deterministic in native record order.
fn logical(records: &[k::AttemptRecord]) -> Vec<Value> {
    let o = |r: &k::AttemptRecord| exact(r.work.checked_lme(),"W")+exact(r.k4_work.checked_lme(),"K");
    let mut out: Vec<Value> = Vec::new();
    let mut parts: Vec<(Vec<Value>,u64,u64)> = Vec::new();
    for (i,r) in records.iter().enumerate() {
        let (d,s,v) = (r.stop_rule_work, r.shared_work, r.verification_shared_work);
        let b_case = o(r)-d+s+v; let b_inv = o(r)-d+if r.shared_built_here{s}else{0}+if r.verification_shared_built_here{v}else{0};
        match r.role {
            k::AttemptRole::Candidate => {
                out.push(json!({"precision":r.precision,"candidate_record":i,"origin":{"kind":"fresh"},"verification":null,"outcome":outcome(&r.outcome)}));
                parts.push((vec![json!({"record":i,"part":"solve_and_verification"}),json!({"record":i,"part":"candidate_stop"})], b_case+d, b_inv+d));
            }
            k::AttemptRole::Verification | k::AttemptRole::VerificationThenCandidate => {
                let last = out.len()-1;
                let phase = if matches!(r.role, k::AttemptRole::VerificationThenCandidate) || matches!(r.outcome, k::AttemptOutcome::Verified|k::AttemptOutcome::Solved) {"completed"} else {"failed"};
                out[last]["verification"] = json!({"record":i,"precision":r.precision,"phase":phase,"reason":null});
                parts[last].0.push(json!({"record":i,"part":"solve_and_verification"})); parts[last].1 += b_case; parts[last].2 += b_inv;
                if matches!(r.role, k::AttemptRole::VerificationThenCandidate) {
                    out.push(json!({"precision":r.precision,"candidate_record":i,"origin":{"kind":"reused_verification","attempt":last},"verification":null,"outcome":outcome(&r.outcome)}));
                    parts.push((vec![json!({"record":i,"part":"candidate_stop"})], d, d));
                }
            }
        }
    }
    for (a,(charges,case,inv)) in out.iter_mut().zip(parts) { a["charges"]=json!(charges); a["case_charge"]=json!(case); a["invocation_increment"]=json!(inv); }
    out
}

fn selection(owner: &k::RetainedSolve, capture: &rp::ProductCapture, ids: &[(k::QuantityId,String)], attempt: &rr::PreparedAttemptView<'_>,
    verdicts: &[k::ProductRowVerdict], row_ids: &[String], prov_: &mut Vec<(String,String)>) -> Value {
    let e = owner.evidence();
    let mut scales: std::collections::BTreeMap<u32,[u64;4]> = Default::default();
    for (b,kind,v) in &e.body_scales { scales.entry(*b).or_default()[*kind as usize] = *v; }
    let section_terms: Vec<Value> = capture.facts.iter().enumerate().map(|(i,f)| {
        let op = attempt.operational_new[i].result.as_ref().unwrap();
        json!({"member_id":capture.members[i].id,"area":bits(f.area),"section_modulus":bits(f.section_modulus),"length":bits(op.length),"axial_stiffness":bits(op.axial),"torsional_stiffness":bits(op.torsion)})
    }).collect();
    prov(prov_, "cases[].selection", "PRODUCER: RetainedEvidence (selected owner); result ids via bind_rows QuantityId->row id; section_terms from facts + new operational");
    prov(prov_, "cases[].selection.retained_state_sha256", "ASSUMPTION A1: raw SHA256(RetainedEvidence.retained_state_encoding) (C1 §3 K4RST)");
    json!({"precision":e.selected_precision,"verification_precision":e.verification_precision,
        "ledger_sha256":sha_hex(&e.ledger_encoding),"retained_state_sha256":sha_hex(&e.retained_state_encoding),
        "stop_rule":e.stop_rule.iter().map(|(b,kd,v)|json!({"body":b,"kind":kind_name(*kd),"value":bits(*v)})).collect::<Vec<_>>(),
        "floor_ratio":ubits(e.floor_ratio_bits),
        "body_scales":scales.iter().map(|(b,s)|json!({"body":b,"translation":ubits(s[0]),"rotation":ubits(s[1]),"force":ubits(s[2]),"moment":ubits(s[3])})).collect::<Vec<_>>(),
        "input_derived_dofs":e.input_derived_dofs.iter().map(|d|json!({"node_id":capture.nodes[d.node as usize].0,"component":comp(d.component)})).collect::<Vec<_>>(),
        "section_terms":section_terms,
        // Iteration 3 (emitter bug fix): the published-row classes are the certificate's own
        // per-row verdicts (CertifiedProductProof::verdicts), not the native RetainedEvidence lists.
        "absolute_verified":verdicts.iter().filter_map(|v| match v.class { Some(k::RowClass::AbsoluteVerified{bound_bits})=>Some(json!({"result_id":row_ids[v.row],"bound":ubits(bound_bits)})), _=>None }).collect::<Vec<_>>(),
        "not_covered":Vec::<Value>::new(),
        "pivot_margin_min":bits(e.pivot_margin_min),"rcond":bits(e.rcond),"residual_worst":bits(e.residual_worst),"rcond_label":e.rcond_label,"corrections":e.corrections,
        "resolution_scale":e.resolution_scale.iter().map(|(b,f,m)|json!({"body":b,"force":ubits(*f),"moment":ubits(*m)})).collect::<Vec<_>>(),
        "verification_estimate":e.verification_estimate.iter().map(|(b,kd,v)|json!({"body":b,"kind":kind_name(*kd),"value":bits(*v)})).collect::<Vec<_>>(),
        "verification_charge":e.verification_charge.iter().map(|(b,kd,v)|json!({"body":b,"kind":kind_name(*kd),"value":bits(*v)})).collect::<Vec<_>>(),
        "theta":e.theta.iter().map(|(b,v)|json!({"body":b,"value":bits(*v)})).collect::<Vec<_>>(),
        "certified_bound":e.certified_bound.iter().map(|(b,v)|json!({"body":b,"value":ubits(*v)})).collect::<Vec<_>>(),
        "floor":e.floor.as_ref().map(|f|f.iter().map(|(b,fo,mo)|json!({"body":b,"force":ubits(*fo),"moment":ubits(*mo)})).collect::<Vec<_>>())})
}

fn bridge_error(e: &k::BridgeFailure<'_>) -> Value { json!(format!("UNMAPPED {e:?}")) }
fn lane_work(w: &k::LaneWorkTrace<'_>) -> Value {
    json!({"numeric":numeric(&w.numeric),"point_lme":count(w.point_lme),
        "view":{"visits":count(w.view_visits),"f64_operations":count(w.view_f64_operations),"prescribed_capacity":w.prescribed_capacity,"data_capacity":w.view_data_capacity},
        "correction":{"cast_lme":count(w.correction_cast_lme),"factor_lme":count(w.correction_factor_lme),"visits":count(w.correction_visits),"calls":count(w.correction_calls),
            "rhs_capacity":w.correction_capacities[0],"output_capacity":w.correction_capacities[1],"converted_capacity":w.correction_capacities[2]},
        "visits":count(w.visits),"member_builds":count(w.member_builds),"frame_builds":count(w.frame_builds),"b_products":count(w.b_products),
        "d_products":count(w.d_products),"h_products":count(w.h_products),
        "capacities":w.capacities.iter().map(|(n,c)|json!({"name":n,"capacity":c})).collect::<Vec<_>>(),"data_capacity":w.data_capacity})
}
fn check(c: &rr::CheckRef<'_>) -> Value {
    match c { rr::CheckRef::NotEntered=>json!({"kind":"not_entered"}), rr::CheckRef::Passed=>json!({"kind":"passed"}),
        rr::CheckRef::Failed(f)=>json!({"kind":"failed","error":format!("UNMAPPED {f:?}")}) }
}
fn proof_trace(view: &rr::PreparedAttemptView<'_>) -> Value {
    let Some(p) = view.proof.as_ref() else { return Value::Null };
    let lanes: Vec<Value> = p.lanes.iter().flatten().map(|l| json!({"law":match l.law{k::ReadoutLaw::AdmittedK=>"admitted_k",k::ReadoutLaw::AnnularSource=>"annular_source"},
        "state":if l.result.is_ok(){"completed"}else{"failed"},"error":match &l.result{Ok(())=>Value::Null,Err(e)=>bridge_error(e)},"work":lane_work(&l.work)})).collect();
    let completion = match view.completion { rr::CompletionRef::NotEntered=>json!({"kind":"not_entered"}), rr::CompletionRef::Merged=>json!({"kind":"merged"}),
        rr::CompletionRef::SeparateFailure{visits,capacity_bytes}=>json!({"kind":"separate_failure","visits":count(visits),"capacity_bytes":capacity_bytes}) };
    let coverage = match &view.summary_coverage { Some(rr::SummaryCoverage::Complete(c))=>json!(c.bodies().map(|b|json!({"body":b.body,"stop":b.stop,"has_data":b.has_data})).collect::<Vec<_>>()), _=>Value::Null };
    json!({"lanes":lanes,"numeric":numeric(&p.numeric),"comparisons_lme":count(p.comparisons_lme),"visits":count(p.visits),"scalar_operations":count(p.scalar_operations),
        "projection_conversions":count(p.projection_conversions),
        "projection_outcomes":p.projection_outcomes.iter().map(|(r,o)|json!({"row_index":r,"outcome":conversion(o)})).collect::<Vec<_>>(),
        "capacities":p.capacities,"prepared_capacity_bytes":p.prepared_capacity_bytes,"completion":completion,
        "checks":{"certificate":check(&view.checks[0]),"observables":check(&view.checks[1]),"g5a":check(&view.checks[2])},"summary_coverage":coverage})
}
fn product_attempt(view: &rr::PreparedAttemptView<'_>, source_ref: Option<usize>, run_ref: Option<usize>, result: Value, prov_: &mut Vec<(String,String)>) -> Value {
    const PROPS:[&str;5]=["area","second_moment","polar_moment","section_modulus","radius"];
    let mut copies = k::TraceCopyWork::default();
    let members: Vec<Value> = view.members.iter().map(|m| {
        let w = &view.preparation_work[m.work_index];
        let result = match &m.result { rr::PreparationResult::Prepared(b)=>json!({"kind":"prepared","section":b.bits().iter().map(|x|ubits(*x)).collect::<Vec<_>>()}),
            rr::PreparationResult::Refused(e)=>json!({"kind":"refused","error":section_error(e)}), rr::PreparationResult::Entered=>json!("ENTERED_UNREPRESENTABLE") };
        json!({"member":m.member,"old_source":m.old_source.iter().map(|x|ubits(*x)).collect::<Vec<_>>(),"old_facts":m.old_facts.iter().map(|x|ubits(*x)).collect::<Vec<_>>(),
            "result":result,"work":{"numeric":numeric(&w.numeric_trace(&mut copies)),"initialized_endpoints":count(w.initialized_endpoints),"conversions":count(w.conversions),
                "checks":count(w.checks),"endpoint_assignments":count(w.endpoint_assignments),"layout_bytes":w.layout_bytes},
            "conversions":w.conversion_outcomes().iter().flatten().map(|c|json!({"property":PROPS[c.property],"endpoint":match c.endpoint{k::PreparationEndpoint::Lo=>"lo",k::PreparationEndpoint::Hi=>"hi",k::PreparationEndpoint::Exact=>"exact"},"outcome":conversion(&c.outcome)})).collect::<Vec<_>>()})
    }).collect();
    let names = ["preparation","native","proof_start","projection","maxima","values","aliases","certificate","observables","g5a"];
    let mut stages_v = Map::new(); for (n,s) in names.iter().zip(view.stages.iter()) { stages_v.insert((*n).into(), json!(stage_name(*s))); }
    let a = view.adapter;
    prov(prov_, "product_attempts[]", "PRODUCER: retained_receipt::PreparedAttemptView (typed C3 seam) incl. summary_coverage; indices assigned by the emitter");
    json!({"id":0,"definition_id":DEFINITION_ID,"owner_ref":{"kind":"case","index":0},"ordinary_attempt_ref":0,"material_basis_ref":0,
        "source_ref":source_ref,"run_ref":run_ref,"result":result,"preparation":{"members":members},"stages":stages_v,"proof":proof_trace(view),
        "adapter":{"counts":a.counts,"fault":a.fault.map(|rp::AdapterFault::Overflow(ev)|json!({"kind":"overflow","event":EVENTS[ev as usize]})),
            "prepared_capacity_bytes":a.prepared_capacity_bytes,"observation_capacity_bytes":a.observation_capacity_bytes,"support_capacity_bytes":a.support_capacity_bytes},
        "operational":{"old_coverage":match view.old_coverage{rr::OldCoverage::Complete=>"complete",rr::OldCoverage::CapturedPrefix=>"captured_prefix"},
            "old":view.operational_old.iter().map(operational).collect::<Vec<_>>(),"new":view.operational_new.iter().map(operational).collect::<Vec<_>>()},
        "overlay_work":scalar(view.overlay_work),"g5a_work":scalar(view.g5a_work)})
}
fn preparation_payload(a: &Value, definition_sha256: &str) -> Value {
    json!({"definition_id":a["definition_id"],"definition_sha256":definition_sha256,"owner_ref":a["owner_ref"],"ordinary_attempt_ref":a["ordinary_attempt_ref"],
        "material_basis_ref":a["material_basis_ref"],"members":a["preparation"]["members"].as_array().unwrap().iter().map(|m|json!({
            "member":m["member"],"old_source":m["old_source"],"old_facts":m["old_facts"],"section":m["result"]["section"]})).collect::<Vec<_>>()})
}
fn definition_sha256() -> String {
    let path = std::env::var("I61_DEFINITION").expect("I61_DEFINITION path");
    let v: Value = serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap();
    h("retained_precision_formation_v1", &v)
}
fn finish(mut env: Value, mut body: Value, prov_: &mut Vec<(String,String)>) -> Value {
    let publication = h("retained_precision_publication_mp_v2", &env);
    body["publication_sha256"] = json!(publication);
    let receipt = h("retained_precision_receipt_mp_v2", &body);
    env["retained_precision"] = json!({"body":body,"receipt_sha256":receipt});
    prov(prov_, "hashes", "PRODUCER canonicalizer (canonical_json_checked_v1_text) + sha2: publication over envelope sans member, receipt over body");
    env
}
fn invocation_body(capture: &source_receipt::CapturedInvocation) -> Value {
    json!({"algorithm":"sha256","profile":"openpipestress_jcs_ijson_v1","scope":"actual_request_and_solver_mode","domain":"source_blocks_invocation_v1","value":capture.borrowed_digest()})
}

/// The experiment-02 milestone emission, now over a candidate produced by the facade.
fn emit_selected(candidate: &rp::PrivatePreparedCandidate, raw: &Value, capture: &source_receipt::CapturedInvocation,
    mode: PreviewSolverMode, typed_legacy: Option<&rp::LegacyCapture>) -> Probe {
    let mut p = Vec::new();
    let raw = raw.clone();
    let capture = capture;
    p.push(("ROUTE".into(),"FACADE: run_linear_static_preview_value_with_retained_direct; capture installed in the single actual ordinary run (test-only permit stub, archive only)".into()));
    let mut costs = rr::ProjectionWork::default();
    let view = candidate.typed_trace(&mut costs).unwrap();
    let pc = candidate.capture();
    let (inv, case) = pc.native.as_ref().unwrap();
    let k::ExecutionOutcome::Selected(owner) = &case.outcome else { panic!("not selected") };
    let case_id = raw["model"]["load_cases"][0]["id"].as_str().unwrap().to_owned();
    let env0 = serde_json::to_value(candidate.envelope()).unwrap();
    let (env, _diag, legacy_omitted) = successor_envelope(env0, &case_id, true, &mut p);
    // Result ids: the producer's own row binding (bind_rows) maps QuantityId -> row id.
    let rows = pc.bind_rows(candidate.envelope(), owner).unwrap();
    let ids: Vec<(k::QuantityId,String)> = rows.iter().filter_map(|r| match r.recipe { k::ProductRecipe::Native(q)=>Some((q,r.id.to_owned())), _=>None }).collect();
    let run = &inv.runs()[case.run];
    let records = &owner.evidence().attempts;
    let physical_v: Vec<Value> = records.iter().enumerate().map(|(i,r)| physical(i, r, run.records[i])).collect();
    let run_v = json!({"id":0,"origin":{"call":run.call,"position":run.position,"group":run.group,"source_ref":0,"owner_ref":{"kind":"case","index":0}},
        "cache_before":slots(&run.cache_before),"cache_after":slots(&run.cache_after),"kernel_terminal":{"kind":"selected","reason":null},
        "records":physical_v,"attempts":logical(records),"case_charge":exact(run.work.case(),"case"),
        "invocation_before":exact(run.work.invocation_before(),"before"),"invocation_increment":exact(run.work.invocation_increment(),"inc"),
        "invocation_after":exact(run.work.invocation_after(),"after")});
    p.push(("cases[].run".into(),"PRODUCER: RecordedInvocation.runs()[case.run] (origin, cache, links, RunWork) + RetainedEvidence.attempts; logical attempts by C1 §1 projection in the emitter".into()));
    let mut source = case_source(pc, owner, inv, run, &raw, &case_id, &view, &mut p);
    let attempt = product_attempt(&view, Some(0), Some(0), json!({"kind":"ready"}), &mut p);
    let def = definition_sha256();
    source["preparation"] = json!({"attempt_ref":0,"sha256":h("retained_precision_preparation_v1",&preparation_payload(&attempt,&def))});
    p.push(("sources[].preparation.sha256".into(),"PRODUCER hash; definition_sha256 from READER definition fixture (ASSUMPTION A4)".into()));
    let source_identity = h("retained_precision_source_mp_v2", &{ let mut s=source.clone(); s.as_object_mut().unwrap().remove("index"); s });
    let row_ids: Vec<String> = env["results"].as_array().unwrap().iter().map(|r| r["id"].as_str().unwrap().to_owned()).collect();
    let sel = selection(owner, pc, &ids, &view, candidate.certificate.verdicts(), &row_ids, &mut p);
    let verdict_lines: Vec<String> = candidate.certificate.verdicts().iter().map(|v| {
        let class = match v.class { Some(k::RowClass::RelativeVerified)=>"relative_verified".to_string(), Some(k::RowClass::AbsoluteVerified{..})=>"absolute_verified".into(),
            Some(k::RowClass::InputDerived)=>"input_derived".into(), Some(k::RowClass::Unpublishable)=>"unpublishable".into(), None=>"none".into() };
        format!("{}|{:016x}|{:016x}|{}|passed={}", row_ids[v.row], v.normalized_bits, v.scale_bits, class, v.passed)
    }).collect();
    p.push(("cases[].selection.absolute_verified".into(),"PRODUCER: CertifiedProductProof::verdicts() class AbsoluteVerified{bound_bits} in row order; not_covered emitted empty (no producer NotCovered class; verdict class None rows are non-quantity)".into()));
    let case_v = json!({"basis_ref":{"ref_type":"load_case","ref_id":case_id},"ordinary":{"attempt_ref":0,"quality_binding":{"kind":"present","index":0}},
        "product_attempt_ref":0,"status":"selected","method":METHOD,"run":run_v,"source_ref":0,"source_identity_sha256":source_identity,"selection":sel});
    let calls: Vec<Value> = inv.calls().iter().map(|c| json!({"id":c.id,"kind":"case_batch","owner_refs":c.owners.iter().map(|o|match o{k::NativeOwner::Case(i)=>json!({"kind":"case","index":i}),k::NativeOwner::Combination(i)=>json!({"kind":"combination","index":i})}).collect::<Vec<_>>(),
        "source_refs":c.sources,"run_refs":c.runs,"invocation_before":exact(c.invocation_before,"call before"),"invocation_after":exact(c.invocation_after,"call after"),"result":{"kind":"runs"}})).collect();
    let groups: Vec<Value> = inv.groups().iter().map(|g| json!({"id":g.id,"call":g.call,"first_source_ref":g.first_source,
        "source_refs":inv.runs().iter().filter(|r|r.group==Some(g.id)).map(|r|r.source).collect::<Vec<_>>(),
        "stiffness_sha256":sha_hex(&inv.sources()[g.first_source].stiffness),"preparation":match &g.preparation{k::GroupPreparation::Ready=>json!({"kind":"ready"}),k::GroupPreparation::Refused(r)=>json!(format!("UNMAPPED {r:?}"))}})).collect();
    let builds: Vec<Value> = inv.builds().iter().map(|b| json!({"id":b.id,"group":b.group,"slot":slot_name(b.slot),
        "origin":{"call":b.call,"run":b.run,"physical_record":b.physical_record,"phase":match b.phase{k::BuildPhase::Shared=>"shared",k::BuildPhase::VerificationShared=>"verification_shared"}},
        "state":match b.state{k::BuildState::Success=>"success",k::BuildState::NonbudgetFailure=>"nonbudget_failure",k::BuildState::BudgetFailure=>"budget_failure"},
        "reason":b.reason.as_ref().map(|r|json!(format!("UNMAPPED {r:?}"))),"work":exact(b.work,"build"),"stages":stages(&b.stages)})).collect();
    p.push(("calls/groups/builds".into(),"PRODUCER: RecordedInvocation calls()/groups()/builds(); group source_refs = runs naming the group".into()));
    let mut legacy_source_work = Vec::new();
    let ordinary = ordinary_attempt(&env, 0, &case_id, mode, legacy_omitted.as_ref(), typed_legacy, &mut legacy_source_work, &mut p);
    let body = json!({"receipt_version":1,"policy":"M03-INTEGRITY-MP-v2","projection_policy":"RP-LOGICAL-ATTEMPTS-v1","work_policy":"W1-LME-20B-60B-v1",
        "facade_policy":"RP-FACADE-SI-v2","canonicalization":"openpipestress_jcs_ijson_v1","invocation":invocation_body(capture),"publication_sha256":"",
        "work":{"case_limit":20_000_000_000u64,"invocation_limit":60_000_000_000u64,"charged":inv.meter().charged(),"execution_order":[{"kind":"case","index":0}]},
        "cases":[case_v],"combinations":[],"sources":[source],"material_bases":[material_basis(pc,&raw,&mut p)],"calls":calls,"groups":groups,"builds":builds,
        "ordinary_attempts":[ordinary],"product_attempts":[attempt],"legacy_source_work":legacy_source_work});
    p.push(("work".into(),"PRODUCER: RecordedInvocation::new(60e9)/CaseLimit(20e9) (retained_product.rs:3282-3283); charged = meter().charged()".into()));
    let source_env = finish(env, body, &mut p);
    Probe { source: source_env, invocation: json!({"request":raw,"solver_mode":mode.as_str()}), prov: p, peak: String::new(), verdicts: verdict_lines }
}

#[test]
fn i61_facade_receipts() {
    let out = std::path::PathBuf::from(std::env::var("I61_OUT").expect("I61_OUT"));
    let raw: Value = serde_json::from_str(REQUEST).unwrap();
    assert_eq!(raw["model"]["schema_version"], json!("0.1.0"), "milestone request unchanged at 0.1.0");
    for mode in [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny] {
        let name = format!("milestone_{}", mode.as_str());
        // Control A: without the permit, the direct retained entry is byte-identical to the
        // shared/native compatibility route (the unchanged ordinary path).
        let plain = serde_json::to_vec(&run_linear_static_preview_value_with_mode(raw.clone(), mode).unwrap()).unwrap();
        let direct = run_linear_static_preview_value_with_retained_direct(raw.clone(), mode).unwrap();
        assert!(direct.successor.is_none(), "no permit, no successor");
        assert_eq!(serde_json::to_vec(direct.envelope()).unwrap(), plain, "CONTROL A: no-permit direct bytes");
        let adm = direct.admission().expect("census entered");
        println!("I61_ADMISSION {name} profile={:?} allowance={:?}", adm.profile, adm.allowance);
        // The permit path: one actual ordinary run with capture installed.
        let permitted = with_permit(|| run_linear_static_preview_value_with_retained_direct(raw.clone(), mode)).unwrap();
        assert!(!test_permit(), "permit stub cleared");
        let succ = permitted.successor.as_ref().expect("permit path taken");
        println!("I61_OUTCOME {name} {}", succ.outcome);
        // Control B: the ordinary run inside the permit path is byte-identical to the plain run.
        assert_eq!(succ.ordinary_bytes, plain, "CONTROL B: ordinary bytes inside the permit path");
        assert_eq!(serde_json::to_vec(permitted.envelope()).unwrap(), plain, "CONTROL B': returned ordinary envelope");
        println!("I61_CONTROLS {name} A=equal B=equal ordinary_bytes={} ordinary_sha256={}", plain.len(), sha_hex(&plain));
        println!("I61_LEGACY {name} {:?}", succ.legacy);
        let probe = succ.probe.as_ref().expect("selected candidate");
        std::fs::write(out.join(format!("{name}.json")), serde_json::to_string_pretty(&json!({"id":name,"source":probe.source,"invocation":probe.invocation})).unwrap()).unwrap();
        std::fs::write(out.join(format!("{name}.provenance.json")), serde_json::to_string_pretty(&probe.prov).unwrap()).unwrap();
        std::fs::write(out.join(format!("{name}.verdicts.txt")), probe.verdicts.join("\n")).unwrap();
        std::fs::write(out.join(format!("{name}.ordinary_envelope.bytes")), &plain).unwrap();
        println!("I61_EMITTED {name}");
    }
}
