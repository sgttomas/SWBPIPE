//! I61 U1 grant 1 (with U2): the private retained-precision serializer.
//!
//! Maps one completed, certified, one-case prepared attempt and its typed
//! ordinary capture (G-b, G-l) to the successor envelope carrying
//! `retained_precision {body, receipt_sha256}` (C1 §3-4, C2 §5, C3), or to a
//! typed [`ReceiptFailure`]. Production-unreachable: no public entrypoint calls
//! it; U3 installs it behind the capture permit. It never reads diagnostic text
//! or Debug output: a variant this grant does not translate refuses typed
//! (`ReceiptCheck::Untranslated`, closed in grant 2 with G-i and D38).
//!
//! The experiments' emitter (R/I61/receipt_experiment_02-03) is the reference;
//! the differences are the closed gaps G-a (fixed product text), G-b, G-c (D6a,
//! decision 2), G-d, G-e, G-j, G-l, A1 (decision 1), D39 and U2.
// Production-unreachable until U3 installs it behind the capture permit.
#![allow(dead_code)]
use super::retained_product as rp;
use super::retained_receipt as rr;
use super::*;
use open_pipe_stress_canonical_json::canonical_json_checked_v1_text;
use open_pipe_stress_frame_kernel::structural::exact_boundary::functionals::AttemptStage;
use open_pipe_stress_frame_kernel::structural::retained_api as k;
use serde_json::{json, Map, Value};
use sha2::{Digest, Sha256};
use std::cell::RefCell;

/// Successor identity and profile (C1 §3; the semantic table fixture).
pub(super) const RETAINED_SEMANTIC_ID: &str =
    "openpipestress.result_semantics/0.3.0/preview-physics-retained-1";
pub(super) const RETAINED_PROFILE_ID: &str = "product_preview_retained_w1a_v2";
/// The row method token (C1 §4 `selected`).
pub(super) const METHOD: &str = "contribution_preserving_multiprecision_v1";
pub(super) const POLICY: &str = "M03-INTEGRITY-MP-v2";
pub(super) const PROJECTION_POLICY: &str = "RP-LOGICAL-ATTEMPTS-v1";
pub(super) const WORK_POLICY: &str = "W1-LME-20B-60B-v1";
pub(super) const FACADE_POLICY: &str = "RP-FACADE-SI-v2";
pub(super) const CANONICALIZATION: &str = "openpipestress_jcs_ijson_v1";
/// The registered prepared-ordinary formation definition and its
/// H("retained_precision_formation_v1", definition). Bound to the in-tree
/// definition fixture and to the semantic table's entry by tests.
pub(super) const DEFINITION_ID: &str = "RP-PREPARED-ORDINARY-DUAL-v1";
pub(super) const DEFINITION_SHA256: &str =
    "a7ed7ca0bf0bba6e8b821ca4befa00a0fa9541a83694be8b28ac63e39b1d0349";
/// W1-LME-20B-60B-v1 (C1 §2): the case limit `solve_native` passes and the
/// invocation limit its meter carries.
pub(super) const CASE_LIMIT: u64 = 20_000_000_000;
pub(super) const INVOCATION_LIMIT: u64 = 60_000_000_000;
/// The fixed product text of the selected-case diagnostic (G-a).
pub(super) const SELECTED_CODE: &str = "RETAINED_PRECISION_SELECTED";
pub(super) const SELECTED_MESSAGE: &str = "Retained-precision recovery (contribution_preserving_multiprecision_v1) is selected for this load case. Its published rows carry recovery_method; the retained_precision receipt binds their certified classes, the native attempts and work, and the ordinary-attempt evidence.";
const MAX_SAFE: u64 = (1 << 53) - 1;
const LEGACY_CODE: &str = "SOURCE_BLOCK_RECOVERY_UNAVAILABLE";

/// The typed cause of an abandoned projection (C2 `receipt_failure`). Never
/// serialized in grant 1: a failure means U3 publishes the ordinary envelope.
/// [`ReceiptCheck::wire`] maps every variant onto the accepted `check` enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum ReceiptCheck {
    Encoding,
    PublicationHashRange,
    /// The native work total's own fault, kept typed.
    WorkCounter(k::WorkFault),
    /// An exact amount above 2^53-1.
    WorkCounterRange,
    WorkCounterInconsistent,
    /// A legacy `rejected` beyond the safe range: only `saturating_add` reaches it.
    SaturationNotExcluded,
    Association,
    /// Outside grant 1's one-case selected scope (G-j; coexistence; D9b).
    Scope,
    /// A variant whose closed translation is grant 2's (G-i, D38).
    Untranslated,
}
impl ReceiptCheck {
    /// D-4b (ROOT, NUM fe38ea55bc): the accepted `receipt_failure.check` token,
    /// C1:68's vocabulary only. Total over the typed variants.
    pub(super) fn wire(self) -> &'static str {
        match self {
            Self::Encoding => "encoding",
            Self::PublicationHashRange => "publication_hash_range",
            Self::WorkCounter(k::WorkFault::Overflow) => "work_counter_range",
            Self::WorkCounter(k::WorkFault::Inconsistent | k::WorkFault::Both) => "work_counter_inconsistent",
            Self::WorkCounterRange => "work_counter_range",
            Self::WorkCounterInconsistent => "work_counter_inconsistent",
            Self::SaturationNotExcluded => "saturation_not_excluded",
            // A candidate outside the one-case invocation it was given is not associated with it.
            Self::Association | Self::Scope => "association",
            // A payload this grant cannot translate cannot be encoded.
            Self::Untranslated => "encoding",
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct ReceiptFailure {
    pub check: ReceiptCheck,
    pub field_path: &'static str,
}
fn fail(check: ReceiptCheck, field_path: &'static str) -> ReceiptFailure {
    ReceiptFailure { check, field_path }
}

/// First-failure encoder: every Bits value is finite, every counter exact and
/// safe. It keeps the projection total so the first failure is reported.
#[derive(Default)]
struct Enc {
    failures: RefCell<Vec<ReceiptFailure>>,
}
impl Enc {
    fn fail(&self, check: ReceiptCheck, path: &'static str) {
        self.failures.borrow_mut().push(fail(check, path));
    }
    fn untranslated(&self, path: &'static str) -> Value {
        self.fail(ReceiptCheck::Untranslated, path);
        Value::Null
    }
    fn bits(&self, x: f64, path: &'static str) -> Value {
        if !x.is_finite() {
            self.fail(ReceiptCheck::Encoding, path);
        }
        json!(format!("{:016x}", x.to_bits()))
    }
    fn ubits(&self, x: u64, path: &'static str) -> Value {
        self.bits(f64::from_bits(x), path)
    }
    /// A trace Count: exact or abandon (D-4 §3 item 2); a selected successor
    /// carries no unavailable count.
    fn count(&self, w: k::WorkTotal) -> Value {
        match w.exact() {
            Ok(v) => {
                if v > MAX_SAFE {
                    self.fail(ReceiptCheck::WorkCounterRange, "Count.value");
                }
                json!({"kind":"exact","value":v})
            }
            Err(f) => {
                self.fail(ReceiptCheck::WorkCounter(f), "Count.value");
                json!({"kind":"unavailable","fault":fault_name(f)})
            }
        }
    }
    fn exact(&self, w: k::WorkTotal, path: &'static str) -> u64 {
        match w.exact() {
            Ok(v) => {
                if v > MAX_SAFE {
                    self.fail(ReceiptCheck::WorkCounterRange, path);
                }
                v
            }
            Err(f) => {
                self.fail(ReceiptCheck::WorkCounter(f), path);
                0
            }
        }
    }
    fn sum(&self, parts: &[u64], path: &'static str) -> u64 {
        let total = parts.iter().try_fold(0u64, |a, b| a.checked_add(*b));
        match total {
            Some(v) if v <= MAX_SAFE => v,
            _ => {
                self.fail(ReceiptCheck::WorkCounterRange, path);
                0
            }
        }
    }
    fn diff(&self, a: u64, b: u64, path: &'static str) -> u64 {
        a.checked_sub(b).unwrap_or_else(|| {
            self.fail(ReceiptCheck::WorkCounterInconsistent, path);
            0
        })
    }
    fn finish(self) -> Result<(), ReceiptFailure> {
        match self.failures.into_inner().first() {
            Some(f) => Err(*f),
            None => Ok(()),
        }
    }
}

fn sha_hex(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}
/// H(d,p) = sha256(UTF8(JCS_checked({"domain":d,"payload":p}))) (C1 §3).
pub(super) fn domain_hash(domain: &str, payload: &Value) -> Option<String> {
    let text = serde_json::to_string(&json!({"domain":domain,"payload":payload})).ok()?;
    canonical_json_checked_v1_text(&text).ok().map(|canon| sha_hex(canon.as_bytes()))
}
fn fault_name(f: k::WorkFault) -> &'static str {
    match f {
        k::WorkFault::Overflow => "overflow",
        k::WorkFault::Inconsistent => "inconsistent",
        k::WorkFault::Both => "both",
    }
}
fn sticky(s: k::WorkStatus) -> &'static str {
    match s.fault() {
        None => "exact",
        Some(f) => fault_name(f),
    }
}
fn kind_name(kind: k::Kind) -> &'static str {
    match kind {
        k::Kind::Translation => "translation",
        k::Kind::Rotation => "rotation",
        k::Kind::Force => "force",
        k::Kind::Moment => "moment",
    }
}
fn comp(c: k::Component) -> &'static str {
    ["UX", "UY", "UZ", "RX", "RY", "RZ"][c.index()]
}
fn dof(d: k::Dof) -> Value {
    json!({"node":d.node,"component":comp(d.component)})
}
fn helper_stage(stage: AttemptStage) -> &'static str {
    match stage {
        AttemptStage::SourceClosure => "source_closure",
        AttemptStage::Preparation => "preparation",
        AttemptStage::Solve => "solve",
        AttemptStage::Plan => "plan",
        AttemptStage::Evaluation => "evaluation",
        AttemptStage::Projection => "projection",
        AttemptStage::Retention => "retention",
        AttemptStage::Replay => "replay",
    }
}
/// The 19 native stage slots, emitted only when the `StageWork` itself is
/// exact: `checked_total` joins its status latch (`set` stores saturated slots).
fn stages(e: &Enc, s: &k::StageWork, path: &'static str) -> Value {
    if let Err(fault) = s.checked_total().exact() {
        e.fail(ReceiptCheck::WorkCounter(fault), path);
        return Value::Null;
    }
    json!({"formation":s.formation,"assembly":s.assembly,"residual_formation":s.residual_formation,"factor":s.factor,
        "condition":s.condition,"rhs":s.rhs,"solve":s.solve,"refinement":s.refinement,"recovery":s.recovery,"stop_rule":s.stop_rule,
        "bounded_gate":s.bounded_gate,"scale":s.scale,"estimate":s.estimate,"charge":s.charge,"bound":s.bound,"shift":s.shift,
        "bounded_formation":s.bounded_formation,"wide_formation":s.wide_formation,"uc":s.uc})
}
fn numeric(e: &Enc, n: &k::NumericTrace) -> Value {
    json!({"wide_lme":e.count(n.wide_lme),"exact_sum_lme":e.count(n.exact_sum_lme),"entries":n.entries.iter().map(|x|e.count(*x)).collect::<Vec<_>>(),
        "f64_arithmetic":e.count(n.f64_arithmetic),"sticky_status":sticky(n.sticky_status)})
}
fn conversion(e: &Enc, o: &k::Binary64Outcome) -> Value {
    const P: &str = "Conversion";
    match *o {
        k::Binary64Outcome::Normal(v) => json!({"kind":"normal","value":e.bits(v,P)}),
        k::Binary64Outcome::Subnormal { value, relative_precision } => {
            json!({"kind":"subnormal","value":e.bits(value,P),"relative_precision":e.bits(relative_precision,P)})
        }
        k::Binary64Outcome::Underflow { negative } => json!({"kind":"underflow","negative":negative}),
        k::Binary64Outcome::Overflow { negative } => json!({"kind":"overflow","negative":negative}),
    }
}
fn quantity(e: &Enc, q: k::QuantityId) -> Value {
    match q {
        k::QuantityId::Displacement(d) => json!({"tag":"displacement","dof":dof(d)}),
        k::QuantityId::Reaction(d) => json!({"tag":"reaction","dof":dof(d)}),
        k::QuantityId::DisplacementMagnitude(n) => json!({"tag":"displacement_magnitude","node":n}),
        k::QuantityId::EndAction { member, end, component } => json!({"tag":"end_action","member":member,
            "end":match end { k::End::I=>"i", k::End::J=>"j" },"component":comp(component)}),
        k::QuantityId::StationAction { station, component } => {
            json!({"tag":"station_action","station":station,"component":comp(component)})
        }
        k::QuantityId::SpringAction { spring, component } => {
            json!({"tag":"spring_action","spring":spring,"component":comp(component)})
        }
        // The ordinary prepared source declares no directional springs (C2 SourceError note).
        k::QuantityId::DirectionalSpringAction { .. } => e.untranslated("sources[].layout[].quantity"),
        k::QuantityId::SupportForceMagnitude(s) => json!({"tag":"support_force_magnitude","support":s}),
        k::QuantityId::SupportMomentMagnitude(s) => json!({"tag":"support_moment_magnitude","support":s}),
    }
}
/// Outcome; a rejected/failed reason is the closed Reason translation (G-i, grant 2).
fn outcome(e: &Enc, o: &k::AttemptOutcome) -> Value {
    match o {
        k::AttemptOutcome::Accepted => json!({"kind":"accepted"}),
        k::AttemptOutcome::Verified => json!({"kind":"verified"}),
        k::AttemptOutcome::Solved => json!({"kind":"solved"}),
        k::AttemptOutcome::Rejected(_) | k::AttemptOutcome::Failed(_) => e.untranslated("Outcome.reason"),
    }
}
fn slot_name(s: k::OriginSlot) -> &'static str {
    match s {
        k::OriginSlot::S128 => "s128",
        k::OriginSlot::S256 => "s256",
        k::OriginSlot::S512 => "s512",
        k::OriginSlot::S1024 => "s1024",
        k::OriginSlot::V256 => "v256",
        k::OriginSlot::V512 => "v512",
        k::OriginSlot::V1024 => "v1024",
    }
}
fn slots(s: &k::SlotSnapshot) -> Value {
    Value::Array(
        k::OriginSlot::ALL
            .iter()
            .zip(s.iter())
            .filter_map(|(slot, b)| b.map(|b| json!({"slot":slot_name(*slot),"build":b})))
            .collect(),
    )
}
fn scalar(w: &rp::ScalarWork) -> Value {
    json!({"entered":w.entered,"checks":w.checks,"lost":w.lost})
}
fn op_name(o: rp::ScalarOperation) -> &'static str {
    match o {
        rp::ScalarOperation::Add => "add",
        rp::ScalarOperation::Sub => "sub",
        rp::ScalarOperation::Mul => "mul",
        rp::ScalarOperation::Div => "div",
        rp::ScalarOperation::Sqrt => "sqrt",
    }
}
fn op_error(e: &rp::OperationalError) -> Value {
    match e {
        rp::OperationalError::MissingOrForeign => json!({"kind":"missing_or_foreign"}),
        rp::OperationalError::Input => json!({"kind":"input"}),
        rp::OperationalError::Degenerate => json!({"kind":"degenerate"}),
        rp::OperationalError::Accounting => json!({"kind":"accounting"}),
        rp::OperationalError::NonFinite { operation, entered } => {
            json!({"kind":"non_finite","operation":op_name(*operation),"entered":entered})
        }
        rp::OperationalError::CoefficientRange { coefficient, operation } => {
            json!({"kind":"coefficient_range","coefficient":coefficient,"operation":op_name(*operation)})
        }
    }
}
fn operational(e: &Enc, o: &rp::OperationalSpent) -> Value {
    const P: &str = "product_attempts[].operational";
    let result = match &o.result {
        Ok(v) => json!({"kind":"ready","length":e.bits(v.length,P),"axial_stiffness":e.bits(v.axial,P),"torsional_stiffness":e.bits(v.torsion,P),
            "normalization":v.normalization_check.iter().map(|x|e.bits(*x,P)).collect::<Vec<_>>()}),
        Err(err) => json!({"kind":"refused","error":op_error(err)}),
    };
    let member = match o.member {
        Some(m) => json!(m),
        None => {
            e.fail(ReceiptCheck::Association, "product_attempts[].operational[].member");
            Value::Null
        }
    };
    json!({"member":member,"inputs":o.inputs.iter().map(|x|e.ubits(*x,P)).collect::<Vec<_>>(),"result":result,"work":scalar(&o.work)})
}
const EVENTS: [&str; 10] = [
    "source_visit", "row_visit", "map_write", "validation_entry", "identity_byte_read",
    "key_probe", "allocation_request", "library_boundary", "requested_copy_bytes", "rust_capacity_bytes",
];
fn section_error(e: &Enc, err: &k::SectionPreparationError) -> Value {
    const P: [&str; 5] = ["area", "second_moment", "polar_moment", "section_modulus", "radius"];
    match err {
        k::SectionPreparationError::InvalidGeometry => json!({"kind":"invalid_geometry"}),
        k::SectionPreparationError::Accounting => json!({"kind":"accounting"}),
        k::SectionPreparationError::AmbiguousRounding(p) => match P.get(*p) {
            Some(name) => json!({"kind":"ambiguous_rounding","property":name}),
            None => e.untranslated("SectionError.property"),
        },
        k::SectionPreparationError::PrimitiveRange(p) => match P.get(*p) {
            Some(name) => json!({"kind":"primitive_range","property":name}),
            None => e.untranslated("SectionError.property"),
        },
        k::SectionPreparationError::Arithmetic(_) => e.untranslated("SectionError.arithmetic.cause"),
    }
}
fn stage_name(e: &Enc, s: rr::StageState) -> Value {
    match s {
        rr::StageState::NotEntered => json!("not_entered"),
        rr::StageState::Completed => json!("completed"),
        rr::StageState::Failed => json!("failed"),
        rr::StageState::Entered => {
            e.fail(ReceiptCheck::Association, "product_attempts[].stages");
            Value::Null
        }
    }
}
/// StructuralError (C2 §4 "Ordinary and W2 nested errors").
fn structural_error(e: &Enc, err: &StructuralError) -> Value {
    const P: &str = "StructuralError";
    match err {
        StructuralError::InvalidInput(detail) => json!({"tag":"invalid_input","detail":detail}),
        StructuralError::Range(detail) => json!({"tag":"range","detail":detail}),
        StructuralError::Asymmetric { row, col, relative_skew } => {
            json!({"tag":"asymmetric","row":row,"col":col,"relative_skew":e.bits(*relative_skew,P)})
        }
        StructuralError::NumericallyUnresolved { reason, global_dof } => {
            json!({"tag":"numerically_unresolved","detail":reason,"global_dof":global_dof})
        }
        StructuralError::NegativeEnergy { direction, energy, allowance } => json!({"tag":"negative_energy",
            "direction":direction.iter().map(|x|e.bits(*x,P)).collect::<Vec<_>>(),"energy":e.bits(*energy,P),"allowance":e.bits(*allowance,P)}),
        StructuralError::Mechanism { direction } => {
            json!({"tag":"mechanism","direction":direction.iter().map(|x|e.bits(*x,P)).collect::<Vec<_>>()})
        }
    }
}
/// RangeTrigger (C2 §4): formation `NumericalRange` or evaluation `StructuralError::Range`.
fn range_trigger(e: &Enc, trigger: &RangeTrigger) -> Value {
    match trigger {
        RangeTrigger::Formation(FrameKernelError::NumericalRange { name }) => {
            json!({"tag":"formation","error":{"tag":"numerical_range","name":name}})
        }
        RangeTrigger::Evaluation(StructuralError::Range(detail)) => {
            json!({"tag":"evaluation","error":{"tag":"range","detail":detail}})
        }
        _ => e.untranslated("ordinary_attempts[].w2.trigger"),
    }
}

/// G-a with T1 (a): identity, profile, method token, the omitted legacy
/// disclosure (by its captured id) and the selected-case diagnostic.
fn successor_envelope(env: &mut Value, case_id: &str, omit: Option<&str>) -> Result<String, ReceiptFailure> {
    let assoc = |p| fail(ReceiptCheck::Association, p);
    env["producer"]["semantic_contract_id"] = json!(RETAINED_SEMANTIC_ID);
    env["formulation_basis"]["profile_id"] = json!(RETAINED_PROFILE_ID);
    for row in env["results"].as_array_mut().ok_or(assoc("results"))? {
        if row["basis_ref"]["ref_id"] == json!(case_id) {
            row["recovery_method"] = json!(METHOD);
        }
    }
    let diags = env["diagnostics"].as_array_mut().ok_or(assoc("diagnostics"))?;
    let names = |d: &Value| d["affected_refs"].as_array().is_some_and(|a| a.iter().any(|r| r == case_id));
    if let Some(id) = omit {
        let found: Vec<usize> = diags.iter().enumerate().filter(|(_, d)| d["id"] == json!(id)).map(|(i, _)| i).collect();
        let &[i] = found.as_slice() else { return Err(assoc("diagnostics[-legacy]")) };
        if diags[i]["code"] != json!(LEGACY_CODE) || !names(&diags[i]) {
            return Err(assoc("diagnostics[-legacy]"));
        }
        diags.remove(i);
    }
    // G4: no legacy-unavailable disclosure may still name a selected case.
    if diags.iter().any(|d| d["code"] == json!(LEGACY_CODE) && names(d)) {
        return Err(assoc("diagnostics[legacy]"));
    }
    let id = format!("diagnostic:retained-precision:{case_id}:selected");
    if diags.iter().any(|d| d["id"] == json!(id)) {
        return Err(assoc("diagnostics[+selected].id"));
    }
    diags.push(json!({"id":id,"code":SELECTED_CODE,"severity":"info","message":SELECTED_MESSAGE,
        "source":"core/product_physics","affected_refs":[case_id]}));
    Ok(id)
}

/// D39 with T1 (a): the legacy disposition; on the selected case a captured
/// disclosure is omitted and its actual WorkReport goes to legacy_source_work.
fn legacy_source(e: &Enc, seed: Option<&rp::LegacySeed>, case_index: usize, work: &mut Vec<Value>)
    -> Result<(Value, Option<String>), ReceiptFailure> {
    let entry = |w: &rp::LegacyWork| {
        // D-4 §3 item 4: `charged` and `limit` are guarded exact; `rejected` is
        // projected only within the safe range (beyond it only saturation reaches).
        let beyond = |amount: usize| u64::try_from(amount).map_or(true, |a| a > MAX_SAFE);
        for (amount, path) in [(w.charged, "legacy_source_work[].charged"), (w.limit, "legacy_source_work[].limit")] {
            if beyond(amount) {
                e.fail(ReceiptCheck::WorkCounterRange, path);
            }
        }
        if beyond(w.rejected) {
            e.fail(ReceiptCheck::SaturationNotExcluded, "legacy_source_work[].rejected");
        }
        json!({"case_index":case_index,"stage":w.stage,"helper_stage":helper_stage(w.helper_stage),
            "charged":w.charged,"rejected":w.rejected,"limit":w.limit,"settlement":"booked"})
    };
    Ok(match seed {
        Some(rp::LegacySeed::NotEligible) => (json!({"disposition":"not_eligible","diagnostic_ref":null,"work_ref":null}), None),
        Some(rp::LegacySeed::NotRequired) => (json!({"disposition":"not_required","diagnostic_ref":null,"work_ref":null}), None),
        Some(rp::LegacySeed::DeclinedWithoutAttempt { work: w, diagnostic_ref }) => {
            work.push(entry(w));
            (json!({"disposition":"declined_without_attempt","diagnostic_ref":null,"work_ref":work.len()-1}), Some(diagnostic_ref.clone()))
        }
        Some(rp::LegacySeed::Unavailable { work: w, diagnostic_ref }) => {
            work.push(entry(w));
            (json!({"disposition":"unavailable","diagnostic_ref":null,"work_ref":work.len()-1}), Some(diagnostic_ref.clone()))
        }
        // Coexistence (D-15): exact-block selected, so W1 is not attempted.
        Some(rp::LegacySeed::ExactSelected) => return Err(fail(ReceiptCheck::Scope, "ordinary_attempts[].legacy_source")),
        None => { e.untranslated("ordinary_attempts[].legacy_source"); (Value::Null, None) }
    })
}

/// G-b (C2 §5): `initial`, `w2` and `formation` from the typed seed.
fn ordinary_members(e: &Enc, seed: &rp::OrdinarySeed) -> (Value, Value, Value) {
    let initial = match &seed.initial {
        Some(rp::InitialSeed::Report { code, report_diagnostic_ref }) => {
            let outcome = match code.as_str() {
                "NUMERICAL_INTEGRITY_SENSITIVE" => json!("sensitive"),
                "NUMERICAL_INTEGRITY_CHECKS_PASSED" => json!("checks_passed"),
                _ => e.untranslated("ordinary_attempts[].initial.outcome"),
            };
            json!({"kind":"report","report_diagnostic_ref":report_diagnostic_ref,"outcome":outcome})
        }
        Some(rp::InitialSeed::StructuralFailure { error, diagnostic_ref }) => {
            json!({"kind":"structural_failure","error":structural_error(e,error),"diagnostic_ref":diagnostic_ref})
        }
        // basis_index is not captured in grant 1.
        Some(rp::InitialSeed::FormationFailure { .. }) => e.untranslated("ordinary_attempts[].initial.formation_failure"),
        // The not-attempted cause is not captured in grant 1.
        None => e.untranslated("ordinary_attempts[].initial.not_attempted"),
    };
    let w2 = match &seed.w2 {
        rp::W2Seed::NotTriggered => json!({"kind":"not_triggered"}),
        rp::W2Seed::Published { trigger, force_scale_exponent, report_diagnostic_ref: Some(report) } => {
            json!({"kind":"published","trigger":range_trigger(e,trigger),"force_scale_exponent":force_scale_exponent,"report_diagnostic_ref":report})
        }
        rp::W2Seed::Published { report_diagnostic_ref: None, .. } => {
            e.fail(ReceiptCheck::Association, "ordinary_attempts[].w2.report_diagnostic_ref");
            Value::Null
        }
        // ForceScalingFailure's closed translation is grant 2's.
        rp::W2Seed::Failed { .. } => e.untranslated("ordinary_attempts[].w2.failure"),
    };
    if seed.recovery_demoted {
        e.untranslated("ordinary_attempts[].formation.recovery_finding");
    }
    let finding = seed.load_row_finding.as_ref().map(|f| {
        json!({"guard":"load_row","sentence":f.sentence,"fired":f.fired,"diagnostic_ref":f.diagnostic_ref})
    });
    let formation = json!({"load_row_finding":finding,"d5_diagnostic_ref":seed.d5_diagnostic_ref});
    (initial, w2, formation)
}

fn material_basis(e: &Enc, capture: &rp::ProductCapture, raw: &Value, case_index: usize) -> Result<Value, ReceiptFailure> {
    let assoc = |p| fail(ReceiptCheck::Association, p);
    let case = &raw["model"]["load_cases"][case_index];
    // Grant 1 maps the base selector; a named or temperature basis is wider scope.
    if !case.get("modulus_basis_ref").is_none_or(Value::is_null) || !case.get("modulus_basis_temperature").is_none_or(Value::is_null) {
        return Err(fail(ReceiptCheck::Untranslated, "material_bases[].selector"));
    }
    if !capture.selections.is_empty() || capture.basis_record.is_some() {
        return Err(fail(ReceiptCheck::Untranslated, "material_bases[].materials[].selection"));
    }
    let inputs = match raw.get("materials").and_then(Value::as_array).filter(|m| !m.is_empty()) {
        Some(m) => m,
        None => raw["model"]["materials"].as_array().ok_or(assoc("material_bases[].materials"))?,
    };
    let mut used: Vec<usize> = capture.members.iter().map(|m| m.material).collect();
    used.sort_unstable();
    used.dedup();
    let mut materials = Vec::with_capacity(used.len());
    for i in used {
        let (id, modulus, shear) = capture.materials.get(i).ok_or(assoc("material_bases[].materials[].input_index"))?;
        let input = inputs.get(i).ok_or(assoc("material_bases[].materials[].input_index"))?;
        if input["id"].as_str() != Some(id.as_str()) {
            return Err(assoc("material_bases[].materials[].id"));
        }
        // An explicit G is the request's own member; a derived E/nu basis is wider scope.
        if input.get("shear_modulus").is_none_or(Value::is_null) {
            return Err(fail(ReceiptCheck::Untranslated, "material_bases[].materials[].shear_origin"));
        }
        const P: &str = "material_bases[].materials[]";
        materials.push(json!({"input_index":i,"id":id,"elastic_modulus":e.bits(*modulus,P),"shear_modulus":e.bits(*shear,P),
            "shear_origin":{"kind":"explicit_g"},"selection":{"kind":"base"}}));
    }
    Ok(json!({"index":0,"selector":{"kind":"base"},"materials":materials,"case_indices":[case_index]}))
}

#[allow(clippy::too_many_arguments)]
fn case_source(e: &Enc, capture: &rp::ProductCapture, owner: &k::RetainedSolve, inv: &k::RecordedInvocation,
    run: &k::RunOrigins, case_index: usize, case_id: &str, attempt: &rr::PreparedAttemptView<'_>) -> Result<Value, ReceiptFailure> {
    let assoc = |p| fail(ReceiptCheck::Association, p);
    const P: &str = "sources[]";
    let source = owner.source();
    let origin = inv.sources().get(run.source).ok_or(assoc("sources[].kernel_source_sha256"))?;
    let mut nodes = Vec::with_capacity(capture.nodes.len());
    for (i, (id, c)) in capture.nodes.iter().enumerate() {
        if source.nodes().get(i) != Some(c) {
            return Err(assoc("sources[].id_maps.nodes"));
        }
        nodes.push(json!({"model_index":i,"kernel_node":i,"id":id,"coordinates":c.iter().map(|x|e.bits(*x,P)).collect::<Vec<_>>()}));
    }
    let mut members = Vec::with_capacity(capture.members.len());
    for (i, m) in capture.members.iter().enumerate() {
        let km = source.members().get(i).ok_or(assoc("sources[].id_maps.members"))?;
        members.push(json!({"model_index":m.model,"built_pipe_index":m.built,"kernel_member":km.id,"node_i":km.node_i,"node_j":km.node_j,
            "material_index":m.material,"id":m.id,"y_reference":km.y_reference.iter().map(|x|e.bits(*x,P)).collect::<Vec<_>>(),
            "E":e.bits(km.elastic_modulus,P),"G":e.bits(km.shear_modulus,P),"A_K":e.bits(km.area,P),"Iy_K":e.bits(km.second_moment_y,P),
            "Iz_K":e.bits(km.second_moment_z,P),"J_K":e.bits(km.torsion_constant,P)}));
    }
    let mut springs = Vec::with_capacity(capture.spring_map.len());
    for (i, s) in capture.spring_map.iter().enumerate() {
        let ks = source.springs().get(i).ok_or(assoc("sources[].id_maps.springs"))?;
        springs.push(json!({"boundary_index":s.boundary,"kernel_spring":ks.id,"support_index":s.support,"node":ks.dof.node,
            "component":comp(ks.dof.component),"stiffness":e.bits(ks.stiffness,P)}));
    }
    let mut support_ids = Vec::with_capacity(capture.supports.len());
    for (i, (id, node)) in capture.supports.iter().enumerate() {
        let group = source.supports().get(i).ok_or(assoc("sources[].id_maps.support_ids"))?;
        support_ids.push(json!({"model_index":i,"kernel_support":group.id,"id":id,"node":node}));
    }
    let body_membership: Vec<Value> = (0..source.body_count()).map(|b| {
        let ms: Vec<u32> = source.members().iter().filter(|m| source.body_of_node(m.node_i) == b).map(|m| m.id).collect();
        json!({"body":b,"nodes":source.body_nodes(b),"members":ms})
    }).collect();
    let layout: Vec<Value> = k::layout(source).iter().enumerate().map(|(i, m)| {
        json!({"index":i,"quantity":quantity(e,m.id),"kind":kind_name(m.kind),"body":m.body,"input_derived":m.input_derived})
    }).collect();
    let stations: Vec<Value> = source.stations().iter().map(|s| {
        let location = if s.fraction == 0.25 { json!("quarter_1") } else if s.fraction == 0.5 { json!("midspan") }
            else if s.fraction == 0.75 { json!("quarter_3") } else { e.untranslated("sources[].stations[].location") };
        json!({"id":s.id,"member":s.member,"location":location,"fraction":e.bits(s.fraction,P)})
    }).collect();
    let supports: Vec<Value> = source.supports().iter().map(|s| {
        json!({"id":s.id,"node":s.node,"restrained":s.restrained,"springs":s.springs,"directional_springs":s.directional_springs})
    }).collect();
    // G-d: the support build's own unique rigid owner of each constrained DOF
    // (capture_supports refuses ambiguous ownership); exactly one is required.
    let mut constraints = Vec::with_capacity(source.constraints().len());
    for c in source.constraints() {
        let owners: Vec<usize> = capture.supports.iter().zip(&capture.support_fixed).enumerate()
            .filter(|(_, ((_, node), fixed))| *node == c.dof.node as usize && fixed[c.dof.component.index()])
            .map(|(i, _)| i).collect();
        if owners.len() != 1 {
            return Err(assoc("sources[].constraints[].support_indices"));
        }
        constraints.push(json!({"dof":dof(c.dof),"value":e.bits(c.value,P),"support_indices":owners}));
    }
    let mut nodal_terms = Vec::with_capacity(source.loads().len());
    for (i, l) in source.loads().iter().enumerate() {
        let t = capture.terms.iter().find(|t| t.canonical == i).ok_or(assoc("sources[].nodal_terms[].primitive_load_index"))?;
        nodal_terms.push(json!({"constructor_ordinal":i,"source_id":l.source_id,"primitive_load_index":t.original,"dof":dof(l.dof),"value":e.bits(l.value,P)}));
    }
    let mut section_terms = Vec::with_capacity(capture.facts.len());
    for (i, f) in capture.facts.iter().enumerate() {
        let op = attempt.operational_new.get(i).and_then(|o| o.result.as_ref().ok()).ok_or(assoc("sources[].section_terms[]"))?;
        section_terms.push(json!({"member":i,"area":e.bits(f.area,P),"section_modulus":e.bits(f.section_modulus,P),"length":e.bits(op.length,P),
            "axial_stiffness":e.bits(op.axial,P),"torsional_stiffness":e.bits(op.torsion,P),
            "geometry":{"route":"preview","normalized_od":e.bits(f.diameter,P),"effective_wall":e.bits(f.effective_wall,P),"actual_radius":e.bits(f.radius,P),
                "actual_second_moment":e.bits(f.second_moment,P),"actual_polar_moment":e.bits(f.torsion_constant,P)}}));
    }
    Ok(json!({"index":0,"owner":{"kind":"case","case_index":case_index,"case_id":case_id},"material_basis_ref":0,
        "kernel_source_sha256":sha_hex(&origin.identity),"stiffness_sha256":sha_hex(&origin.stiffness),
        "id_maps":{"nodes":nodes,"members":members,"springs":springs,"support_ids":support_ids},"body_membership":body_membership,
        "layout":layout,"stations":stations,"supports":supports,"constraints":constraints,"nodal_terms":nodal_terms,"section_terms":section_terms,
        "preparation":null}))
}

/// A physical record's work amounts, each read through its checked accessor
/// (never the legacy saturating `u64` fields): the shared, stop-rule and
/// verification views join the record's `work_status` latch (C1 §1-2).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct RecordWork {
    wide: u64,
    exact_sum: u64,
    own: u64,
    shared: u64,
    stop_rule: u64,
    verification: u64,
    verification_shared: u64,
    case_charge: u64,
    invocation_increment: u64,
}
fn record_work(e: &Enc, r: &k::AttemptRecord) -> RecordWork {
    RecordWork {
        wide: e.exact(r.work.checked_lme(), "run.records[].work.wide_lme"),
        exact_sum: e.exact(r.k4_work.checked_lme(), "run.records[].work.exact_sum_lme"),
        own: e.exact(r.checked_own_work(), "run.records[].work.own_lme"),
        shared: e.exact(r.checked_shared_work(), "run.records[].work.shared_lme"),
        stop_rule: e.exact(r.checked_stop_rule_work(), "run.records[].work.stop_rule_lme"),
        verification: e.exact(r.checked_verification_work(), "run.records[].work.verification_lme"),
        verification_shared: e.exact(r.checked_verification_shared_work(), "run.records[].work.verification_shared_lme"),
        case_charge: e.exact(r.checked_case_charge(), "run.attempts[].case_charge"),
        invocation_increment: e.exact(r.checked_invocation_increment(), "run.attempts[].invocation_increment"),
    }
}
fn physical(e: &Enc, i: usize, r: &k::AttemptRecord, links: k::RecordBuildLinks) -> Value {
    const P: &str = "run.records[]";
    let work = record_work(e, r);
    // C1 §1/§4 conservation: own = W+K; own stages sum to O; shared stages to
    // S+V; D+Q <= O (D-4 §3 item 5, before emission).
    let inconsistent = |ok: bool, path| if !ok { e.fail(ReceiptCheck::WorkCounterInconsistent, path) };
    inconsistent(e.sum(&[work.wide, work.exact_sum], "run.records[].work.own_lme") == work.own, "run.records[].work.own_lme");
    if let (Ok(own_stages), Ok(shared_stages)) = (r.stages.checked_total().exact(), r.shared_stages.checked_total().exact()) {
        inconsistent(own_stages == work.own, "run.records[].work.own_stages");
        inconsistent(shared_stages == e.sum(&[work.shared, work.verification_shared], "run.records[].work.shared_stages"), "run.records[].work.shared_stages");
    }
    inconsistent(e.sum(&[work.stop_rule, work.verification], "run.records[].work.stop_rule_lme") <= work.own, "run.records[].work.stop_rule_lme");
    let gate = match r.gate {
        None => Value::Null,
        Some(k::GateTest::Coalesced) => json!({"kind":"coalesced"}),
        Some(k::GateTest::Bounded { state, evaluated }) => json!({"kind":"bounded","state":state,"evaluated":evaluated}),
    };
    let verification = r.verification.as_ref().map(|v| json!({
        "resolution":v.resolution.iter().enumerate().map(|(b,x)|json!({"body":b,"force":e.bits(x[0],P),"moment":e.bits(x[1],P)})).collect::<Vec<_>>(),
        "theta":v.theta.iter().enumerate().map(|(b,t)|json!({"body":b,"value":e.bits(*t,P)})).collect::<Vec<_>>(),
        "bound":v.bound.iter().enumerate().map(|(b,x)|json!({"body":b,"value":x.map(|x|e.bits(x,P))})).collect::<Vec<_>>(),
        "data_blocks":v.data_blocks,"shift_factorizations":v.shift_factorizations,"g_max":v.g_max,"uc_missing":v.uc_missing,"g_violation":v.g_violation}))
        .unwrap_or(Value::Null);
    let role = match r.role {
        k::AttemptRole::Candidate => "candidate",
        k::AttemptRole::Verification => "verification",
        k::AttemptRole::VerificationThenCandidate => "verification_then_candidate",
    };
    // BlockRefusal's closed translation is grant 2's.
    if !r.bound_refusals.is_empty() {
        e.untranslated("run.records[].bound_refusals");
    }
    json!({"index":i,"precision":r.precision,"role":role,"outcome":outcome(e,&r.outcome),"residual_basis":r.residual_basis,"corrections":r.corrections,
        "pivot_margin_min":r.pivot_margin_min.map(|x|e.bits(x,P)),"rcond":r.rcond.map(|x|e.bits(x,P)),"residual_worst":r.residual_worst.map(|x|e.bits(x,P)),"gate":gate,
        "work":{"wide_lme":work.wide,"exact_sum_lme":work.exact_sum,"own_lme":work.own,"shared_lme":work.shared,"stop_rule_lme":work.stop_rule,
            "verification_lme":work.verification,"verification_shared_lme":work.verification_shared,
            "own_stages":stages(e,&r.stages,"run.records[].work.own_stages"),"shared_stages":stages(e,&r.shared_stages,"run.records[].work.shared_stages"),
            "shared_built_here":r.shared_built_here,"verification_shared_built_here":r.verification_shared_built_here},
        "storage":{"pattern_entries":r.storage.pattern_entries,"profile_entries":r.storage.profile_entries,"limbs_per_entry":r.storage.limbs_per_entry},
        "verification":verification,"bound_refusals":[],"shared_build_ref":links.shared,"verification_shared_build_ref":links.verification_shared})
}

/// C1 §1 projection (RP-LOGICAL-ATTEMPTS-v1), deterministic in native record order.
fn logical(e: &Enc, records: &[k::AttemptRecord]) -> Vec<Value> {
    let mut out: Vec<Value> = Vec::new();
    let mut parts: Vec<(Vec<Value>, u64, u64)> = Vec::new();
    for (i, r) in records.iter().enumerate() {
        let work = record_work(e, r);
        // B = (O-D, S, V) and T = (D, 0, 0) (C1 §1), from the record's checked
        // case charge O+S+V and invocation increment O+bS+vV.
        let d = work.stop_rule;
        let b_case = e.diff(work.case_charge, d, "run.attempts[].case_charge");
        let b_inv = e.diff(work.invocation_increment, d, "run.attempts[].invocation_increment");
        match r.role {
            k::AttemptRole::Candidate => {
                out.push(json!({"precision":r.precision,"candidate_record":i,"origin":{"kind":"fresh"},"verification":null,"outcome":outcome(e,&r.outcome)}));
                parts.push((vec![json!({"record":i,"part":"solve_and_verification"}), json!({"record":i,"part":"candidate_stop"})],
                    work.case_charge, work.invocation_increment));
            }
            k::AttemptRole::Verification | k::AttemptRole::VerificationThenCandidate => {
                let Some(last) = out.len().checked_sub(1) else {
                    e.fail(ReceiptCheck::Association, "run.attempts[].verification");
                    continue;
                };
                let phase = if matches!(r.role, k::AttemptRole::VerificationThenCandidate)
                    || matches!(r.outcome, k::AttemptOutcome::Verified | k::AttemptOutcome::Solved) { "completed" } else { "failed" };
                out[last]["verification"] = json!({"record":i,"precision":r.precision,"phase":phase,"reason":null});
                parts[last].0.push(json!({"record":i,"part":"solve_and_verification"}));
                parts[last].1 = e.sum(&[parts[last].1, b_case], "run.attempts[].case_charge");
                parts[last].2 = e.sum(&[parts[last].2, b_inv], "run.attempts[].invocation_increment");
                if matches!(r.role, k::AttemptRole::VerificationThenCandidate) {
                    out.push(json!({"precision":r.precision,"candidate_record":i,"origin":{"kind":"reused_verification","attempt":last},
                        "verification":null,"outcome":outcome(e,&r.outcome)}));
                    parts.push((vec![json!({"record":i,"part":"candidate_stop"})], d, d));
                }
            }
        }
    }
    for (a, (charges, case, inv)) in out.iter_mut().zip(parts) {
        a["charges"] = json!(charges);
        a["case_charge"] = json!(case);
        a["invocation_increment"] = json!(inv);
    }
    out
}
/// C1 §1: the logical partition conserves the run's case charge and
/// invocation increment, and after = before + increment.
fn run_conservation(e: &Enc, attempts: &[Value], run: &k::RunOrigins) {
    let total = |key: &str, path| {
        let parts: Vec<u64> = attempts.iter().map(|a| a[key].as_u64().unwrap_or(u64::MAX)).collect();
        e.sum(&parts, path)
    };
    let case = e.exact(run.work.case(), "cases[].run.case_charge");
    let before = e.exact(run.work.invocation_before(), "cases[].run.invocation_before");
    let increment = e.exact(run.work.invocation_increment(), "cases[].run.invocation_increment");
    let after = e.exact(run.work.invocation_after(), "cases[].run.invocation_after");
    for (ok, path) in [
        (total("case_charge", "cases[].run.case_charge") == case, "cases[].run.case_charge"),
        (total("invocation_increment", "cases[].run.invocation_increment") == increment, "cases[].run.invocation_increment"),
        (e.sum(&[before, increment], "cases[].run.invocation_after") == after, "cases[].run.invocation_after"),
    ] {
        if !ok {
            e.fail(ReceiptCheck::WorkCounterInconsistent, path);
        }
    }
}

/// G-e: `not_covered` from the certificate's verdicts: a quantity row (not a
/// record/observation recipe) with no class. A certified proof refuses an
/// unpublishable row, so Unpublishable never reaches here; it is refused typed.
#[allow(clippy::too_many_arguments)]
fn selection(e: &Enc, owner: &k::RetainedSolve, capture: &rp::ProductCapture,
    attempt: &rr::PreparedAttemptView<'_>, verdicts: &[k::ProductRowVerdict], recipes: &[k::ProductRecipe], row_ids: &[String])
    -> Result<Value, ReceiptFailure> {
    let assoc = |p| fail(ReceiptCheck::Association, p);
    const P: &str = "cases[].selection";
    let ev = owner.evidence();
    let mut scales: std::collections::BTreeMap<u32, [u64; 4]> = Default::default();
    for (b, kind, v) in &ev.body_scales {
        scales.entry(*b).or_default()[*kind as usize] = *v;
    }
    let mut section_terms = Vec::with_capacity(capture.facts.len());
    for (i, f) in capture.facts.iter().enumerate() {
        let op = attempt.operational_new.get(i).and_then(|o| o.result.as_ref().ok()).ok_or(assoc("cases[].selection.section_terms[]"))?;
        let member = capture.members.get(i).ok_or(assoc("cases[].selection.section_terms[].member_id"))?;
        section_terms.push(json!({"member_id":member.id,"area":e.bits(f.area,P),"section_modulus":e.bits(f.section_modulus,P),"length":e.bits(op.length,P),
            "axial_stiffness":e.bits(op.axial,P),"torsional_stiffness":e.bits(op.torsion,P)}));
    }
    let mut absolute = Vec::new();
    let mut not_covered = Vec::new();
    for v in verdicts {
        let row = row_ids.get(v.row).ok_or(assoc("cases[].selection.absolute_verified[].result_id"))?;
        let recipe = recipes.get(v.row).ok_or(assoc("cases[].selection.not_covered"))?;
        match v.class {
            Some(k::RowClass::AbsoluteVerified { bound_bits }) => absolute.push(json!({"result_id":row,"bound":e.ubits(bound_bits,P)})),
            Some(k::RowClass::RelativeVerified | k::RowClass::InputDerived) => {}
            Some(k::RowClass::Unpublishable) => return Err(fail(ReceiptCheck::Untranslated, "cases[].selection.unpublishable")),
            None if matches!(recipe, k::ProductRecipe::NonQuantity | k::ProductRecipe::ModulusBasisRecord | k::ProductRecipe::DenseParityObservation) => {}
            None => not_covered.push(json!(row)),
        }
    }
    let input_derived_dofs = ev.input_derived_dofs.iter().map(|d| {
        capture.nodes.get(d.node as usize).map(|n| json!({"node_id":n.0,"component":comp(d.component)}))
    }).collect::<Option<Vec<_>>>().ok_or(assoc("cases[].selection.input_derived_dofs"))?;
    let kind_value = |list: &[(u32, k::Kind, f64)]| -> Vec<Value> {
        list.iter().map(|(b, kd, v)| json!({"body":b,"kind":kind_name(*kd),"value":e.bits(*v,P)})).collect()
    };
    Ok(json!({"precision":ev.selected_precision,"verification_precision":ev.verification_precision,
        "ledger_sha256":sha_hex(&ev.ledger_encoding),
        // A1 (decision 1): the raw SHA256 of the K4RST\x01 state bytes (C1 §3).
        "retained_state_sha256":sha_hex(&ev.retained_state_encoding),
        "stop_rule":kind_value(&ev.stop_rule),
        "floor_ratio":e.ubits(ev.floor_ratio_bits,P),
        "body_scales":scales.iter().map(|(b,s)|json!({"body":b,"translation":e.ubits(s[0],P),"rotation":e.ubits(s[1],P),"force":e.ubits(s[2],P),"moment":e.ubits(s[3],P)})).collect::<Vec<_>>(),
        "input_derived_dofs":input_derived_dofs,
        "section_terms":section_terms,
        // E1: the published-row classes are the certificate's own verdicts.
        "absolute_verified":absolute,
        "not_covered":not_covered,
        "pivot_margin_min":e.bits(ev.pivot_margin_min,P),"rcond":e.bits(ev.rcond,P),"residual_worst":e.bits(ev.residual_worst,P),"rcond_label":ev.rcond_label,"corrections":ev.corrections,
        "resolution_scale":ev.resolution_scale.iter().map(|(b,f,m)|json!({"body":b,"force":e.ubits(*f,P),"moment":e.ubits(*m,P)})).collect::<Vec<_>>(),
        "verification_estimate":kind_value(&ev.verification_estimate),
        "verification_charge":kind_value(&ev.verification_charge),
        "theta":ev.theta.iter().map(|(b,v)|json!({"body":b,"value":e.bits(*v,P)})).collect::<Vec<_>>(),
        "certified_bound":ev.certified_bound.iter().map(|(b,v)|json!({"body":b,"value":e.ubits(*v,P)})).collect::<Vec<_>>(),
        "floor":ev.floor.as_ref().map(|f|f.iter().map(|(b,fo,mo)|json!({"body":b,"force":e.ubits(*fo,P),"moment":e.ubits(*mo,P)})).collect::<Vec<_>>())}))
}

fn lane_work(e: &Enc, w: &k::LaneWorkTrace<'_>) -> Value {
    json!({"numeric":numeric(e,&w.numeric),"point_lme":e.count(w.point_lme),
        "view":{"visits":e.count(w.view_visits),"f64_operations":e.count(w.view_f64_operations),"prescribed_capacity":w.prescribed_capacity,"data_capacity":w.view_data_capacity},
        "correction":{"cast_lme":e.count(w.correction_cast_lme),"factor_lme":e.count(w.correction_factor_lme),"visits":e.count(w.correction_visits),"calls":e.count(w.correction_calls),
            "rhs_capacity":w.correction_capacities[0],"output_capacity":w.correction_capacities[1],"converted_capacity":w.correction_capacities[2]},
        "visits":e.count(w.visits),"member_builds":e.count(w.member_builds),"frame_builds":e.count(w.frame_builds),"b_products":e.count(w.b_products),
        "d_products":e.count(w.d_products),"h_products":e.count(w.h_products),
        "capacities":w.capacities.iter().map(|(n,c)|json!({"name":n,"capacity":c})).collect::<Vec<_>>(),"data_capacity":w.data_capacity})
}
fn check(e: &Enc, c: &rr::CheckRef<'_>) -> Value {
    match c {
        rr::CheckRef::NotEntered => json!({"kind":"not_entered"}),
        rr::CheckRef::Passed => json!({"kind":"passed"}),
        rr::CheckRef::Failed(_) => e.untranslated("product_attempts[].proof.checks"),
    }
}
fn proof_trace(e: &Enc, view: &rr::PreparedAttemptView<'_>) -> Value {
    const P: &str = "product_attempts[].proof";
    let Some(p) = view.proof.as_ref() else { return Value::Null };
    let lanes: Vec<Value> = p.lanes.iter().flatten().map(|l| {
        let error = match &l.result { Ok(()) => Value::Null, Err(_) => e.untranslated("product_attempts[].proof.lanes[].error") };
        json!({"law":match l.law { k::ReadoutLaw::AdmittedK=>"admitted_k", k::ReadoutLaw::AnnularSource=>"annular_source" },
            "state":if l.result.is_ok() {"completed"} else {"failed"},"error":error,"work":lane_work(e,&l.work)})
    }).collect();
    let completion = match view.completion {
        rr::CompletionRef::NotEntered => json!({"kind":"not_entered"}),
        rr::CompletionRef::Merged => json!({"kind":"merged"}),
        rr::CompletionRef::SeparateFailure { visits, capacity_bytes } => {
            json!({"kind":"separate_failure","visits":e.count(visits),"capacity_bytes":capacity_bytes})
        }
    };
    let coverage = match &view.summary_coverage {
        Some(rr::SummaryCoverage::Complete(c)) => json!(c.bodies().map(|b|json!({"body":b.body,"stop":b.stop,"has_data":b.has_data})).collect::<Vec<_>>()),
        _ => Value::Null,
    };
    let _ = P;
    json!({"lanes":lanes,"numeric":numeric(e,&p.numeric),"comparisons_lme":e.count(p.comparisons_lme),"visits":e.count(p.visits),"scalar_operations":e.count(p.scalar_operations),
        "projection_conversions":e.count(p.projection_conversions),
        "projection_outcomes":p.projection_outcomes.iter().map(|(r,o)|json!({"row_index":r,"outcome":conversion(e,o)})).collect::<Vec<_>>(),
        "capacities":p.capacities,"prepared_capacity_bytes":p.prepared_capacity_bytes,"completion":completion,
        "checks":{"certificate":check(e,&view.checks[0]),"observables":check(e,&view.checks[1]),"g5a":check(e,&view.checks[2])},"summary_coverage":coverage})
}
fn product_attempt(e: &Enc, view: &rr::PreparedAttemptView<'_>, case_index: usize, source_ref: usize, run_ref: usize) -> Value {
    const PROPS: [&str; 5] = ["area", "second_moment", "polar_moment", "section_modulus", "radius"];
    const P: &str = "product_attempts[].preparation";
    let mut copies = k::TraceCopyWork::default();
    let members: Vec<Value> = view.members.iter().map(|m| {
        let Some(w) = view.preparation_work.get(m.work_index) else {
            e.fail(ReceiptCheck::Association, "product_attempts[].preparation.members[].work");
            return Value::Null;
        };
        let result = match &m.result {
            rr::PreparationResult::Prepared(b) => json!({"kind":"prepared","section":b.bits().iter().map(|x|e.ubits(*x,P)).collect::<Vec<_>>()}),
            rr::PreparationResult::Refused(err) => json!({"kind":"refused","error":section_error(e,err)}),
            rr::PreparationResult::Entered => {
                e.fail(ReceiptCheck::Association, "product_attempts[].preparation.members[].result");
                Value::Null
            }
        };
        json!({"member":m.member,"old_source":m.old_source.iter().map(|x|e.ubits(*x,P)).collect::<Vec<_>>(),"old_facts":m.old_facts.iter().map(|x|e.ubits(*x,P)).collect::<Vec<_>>(),
            "result":result,"work":{"numeric":numeric(e,&w.numeric_trace(&mut copies)),"initialized_endpoints":e.count(w.initialized_endpoints),"conversions":e.count(w.conversions),
                "checks":e.count(w.checks),"endpoint_assignments":e.count(w.endpoint_assignments),"layout_bytes":w.layout_bytes},
            "conversions":w.conversion_outcomes().iter().flatten().map(|c|json!({"property":PROPS.get(c.property).map_or_else(||e.untranslated("product_attempts[].preparation.members[].conversions[].property"),|p|json!(p)),
                "endpoint":match c.endpoint { k::PreparationEndpoint::Lo=>"lo", k::PreparationEndpoint::Hi=>"hi", k::PreparationEndpoint::Exact=>"exact" },
                "outcome":conversion(e,&c.outcome)})).collect::<Vec<_>>()})
    }).collect();
    let names = ["preparation", "native", "proof_start", "projection", "maxima", "values", "aliases", "certificate", "observables", "g5a"];
    let mut stages_v = Map::new();
    for (n, s) in names.iter().zip(view.stages.iter()) {
        stages_v.insert((*n).into(), stage_name(e, *s));
    }
    let a = view.adapter;
    json!({"id":0,"definition_id":DEFINITION_ID,"owner_ref":{"kind":"case","index":case_index},"ordinary_attempt_ref":case_index,"material_basis_ref":0,
        "source_ref":source_ref,"run_ref":run_ref,"result":{"kind":"ready"},"preparation":{"members":members},"stages":stages_v,"proof":proof_trace(e,view),
        "adapter":{"counts":a.counts,"fault":a.fault.map(|rp::AdapterFault::Overflow(ev)|json!({"kind":"overflow","event":EVENTS.get(ev as usize).map_or_else(||e.untranslated("product_attempts[].adapter.fault.event"),|n|json!(n))})),
            "prepared_capacity_bytes":a.prepared_capacity_bytes,"observation_capacity_bytes":a.observation_capacity_bytes,"support_capacity_bytes":a.support_capacity_bytes},
        "operational":{"old_coverage":match view.old_coverage { rr::OldCoverage::Complete=>"complete", rr::OldCoverage::CapturedPrefix=>"captured_prefix" },
            "old":view.operational_old.iter().map(|o|operational(e,o)).collect::<Vec<_>>(),"new":view.operational_new.iter().map(|o|operational(e,o)).collect::<Vec<_>>()},
        "overlay_work":scalar(view.overlay_work),"g5a_work":scalar(view.g5a_work)})
}
fn preparation_payload(a: &Value) -> Value {
    let members = a["preparation"]["members"].as_array().map(|ms| ms.iter().map(|m| json!({
        "member":m["member"],"old_source":m["old_source"],"old_facts":m["old_facts"],"section":m["result"]["section"]})).collect::<Vec<_>>());
    json!({"definition_id":a["definition_id"],"definition_sha256":DEFINITION_SHA256,"owner_ref":a["owner_ref"],"ordinary_attempt_ref":a["ordinary_attempt_ref"],
        "material_basis_ref":a["material_basis_ref"],"members":members})
}
/// Every body integer is a safe JSON integer; the body carries no float.
fn safe_integers(v: &Value) -> bool {
    match v {
        Value::Number(n) => n.as_u64().map_or_else(|| n.as_i64().is_some_and(|i| i.unsigned_abs() <= MAX_SAFE), |u| u <= MAX_SAFE),
        Value::Array(a) => a.iter().all(safe_integers),
        Value::Object(o) => o.values().all(safe_integers),
        _ => true,
    }
}

/// Serialize one selected, certified, one-case prepared candidate (grant 1).
/// Returns the successor envelope with `retained_precision` as its last member.
pub(super) fn serialize_selected(candidate: &rp::PrivatePreparedCandidate, invocation: &source_receipt::CapturedInvocation)
    -> Result<Value, ReceiptFailure> {
    let assoc = |p| fail(ReceiptCheck::Association, p);
    let scope = |p| fail(ReceiptCheck::Scope, p);
    let e = Enc::default();
    let mode = invocation.mode();
    let raw = invocation.borrowed_raw();
    let pc = candidate.capture();
    if pc.invocation_mode != Some(mode) {
        return Err(assoc("invocation"));
    }
    // G-j: one requested case, one native call/run/source and one ordinary capture.
    let request_cases = raw["model"]["load_cases"].as_array().ok_or(assoc("cases"))?;
    let combinations = raw["model"]["combinations"].as_array().map_or(0, Vec::len);
    if request_cases.len() != 1 || combinations != 0 {
        return Err(scope("cases"));
    }
    let case_index = 0usize;
    let case_id = request_cases[case_index]["id"].as_str().ok_or(assoc("cases[].basis_ref"))?.to_owned();
    let (inv, case) = pc.native.as_ref().ok_or(assoc("cases[].run"))?;
    let k::ExecutionOutcome::Selected(owner) = &case.outcome else { return Err(scope("cases[].status")) };
    // U2 (RV77-N4): the proof must have started on this selected owner's own solve.
    let certificate = candidate.certificate();
    if !certificate.owner_matches(owner) {
        return Err(assoc("cases[].selection.owner"));
    }
    if !certificate.passed() {
        return Err(assoc("cases[].selection.certificate"));
    }
    if inv.runs().len() != 1 || inv.calls().len() != 1 || inv.sources().len() != 1 || case.run != 0 {
        return Err(scope("cases[].run"));
    }
    let [seed] = &pc.ordinary[..] else { return Err(scope("ordinary_attempts")) };
    if seed.case != case_id {
        return Err(assoc("ordinary_attempts[].case_id"));
    }
    let run = &inv.runs()[case.run];
    let mut costs = rr::ProjectionWork::default();
    let view = candidate.typed_trace(&mut costs).map_err(|_| assoc("product_attempts[]"))?;

    // The successor envelope (G-a; T1 (a)).
    let mut env = serde_json::to_value(candidate.envelope()).map_err(|_| fail(ReceiptCheck::Encoding, "envelope"))?;
    let mut legacy_source_work = Vec::new();
    let (legacy, omit) = legacy_source(&e, seed.legacy.as_ref(), case_index, &mut legacy_source_work)?;
    successor_envelope(&mut env, &case_id, omit.as_deref())?;

    // Row bindings: the producer's own QuantityId/recipe binding in envelope row order.
    let rows = pc.bind_rows(candidate.envelope(), owner).map_err(|_| assoc("cases[].selection.absolute_verified"))?;
    let row_ids: Vec<String> = env["results"].as_array().ok_or(assoc("results"))?.iter()
        .map(|r| r["id"].as_str().map(str::to_owned)).collect::<Option<_>>().ok_or(assoc("results[].id"))?;
    if rows.len() != row_ids.len() || rows.iter().zip(&row_ids).any(|(r, id)| r.id != id.as_str()) {
        return Err(assoc("cases[].selection.absolute_verified"));
    }
    let recipes: Vec<k::ProductRecipe> = rows.iter().map(|r| r.recipe).collect();

    // The native run (C1 §1).
    let records = &owner.evidence().attempts;
    // Only the first `physical_records` links describe actual records.
    if records.len() != run.physical_records || records.len() > run.records.len() {
        return Err(assoc("cases[].run.records"));
    }
    if run.id != case.run || !matches!(run.owner, k::NativeOwner::Case(i) if i == case_index) {
        return Err(assoc("cases[].run.origin"));
    }
    let physical_v: Vec<Value> = records.iter().enumerate().map(|(i, r)| physical(&e, i, r, run.records[i])).collect();
    let attempts_v = logical(&e, records);
    run_conservation(&e, &attempts_v, run);
    let run_v = json!({"id":run.id,"origin":{"call":run.call,"position":run.position,"group":run.group,"source_ref":run.source,"owner_ref":{"kind":"case","index":case_index}},
        "cache_before":slots(&run.cache_before),"cache_after":slots(&run.cache_after),"kernel_terminal":{"kind":"selected","reason":null},
        "records":physical_v,"attempts":attempts_v,"case_charge":e.exact(run.work.case(),"cases[].run.case_charge"),
        "invocation_before":e.exact(run.work.invocation_before(),"cases[].run.invocation_before"),
        "invocation_increment":e.exact(run.work.invocation_increment(),"cases[].run.invocation_increment"),
        "invocation_after":e.exact(run.work.invocation_after(),"cases[].run.invocation_after")});
    let mut source = case_source(&e, pc, owner, inv, run, case_index, &case_id, &view)?;
    let attempt = product_attempt(&e, &view, case_index, run.source, case.run);
    let preparation = domain_hash("retained_precision_preparation_v1", &preparation_payload(&attempt)).ok_or(fail(ReceiptCheck::Encoding, "sources[].preparation.sha256"))?;
    source["preparation"] = json!({"attempt_ref":0,"sha256":preparation});
    let source_identity = {
        let mut binding = source.clone();
        binding.as_object_mut().ok_or(assoc("sources[]"))?.remove("index");
        domain_hash("retained_precision_source_mp_v2", &binding).ok_or(fail(ReceiptCheck::Encoding, "cases[].source_identity_sha256"))?
    };
    let selection_v = selection(&e, owner, pc, &view, certificate.verdicts(), &recipes, &row_ids)?;

    // The ordinary attempt (G-b, G-c, G-l).
    let quality_index = env["numerical_quality"]["cases"].as_array().and_then(|q| {
        let found: Vec<usize> = q.iter().enumerate().filter(|(_, x)| x["basis_ref"]["ref_id"] == json!(case_id)).map(|(i, _)| i).collect();
        if let &[i] = found.as_slice() { Some(i) } else { None }
    }).ok_or(assoc("cases[].ordinary.quality_binding"))?;
    let (initial, w2, formation) = ordinary_members(&e, seed);
    // C2 §5: the report's outcome is the case's unchanged assessed quality.
    if initial["kind"] == "report" && env["numerical_quality"]["cases"][quality_index]["solve_quality"] != initial["outcome"] {
        return Err(assoc("ordinary_attempts[].initial.outcome"));
    }
    // D6a (decision 2): the diagnostics naming the case, once each, in envelope
    // order, excluding RETAINED_PRECISION_* (the T1 (a) omission is already applied).
    let diagnostic_refs: Vec<Value> = env["diagnostics"].as_array().ok_or(assoc("diagnostics"))?.iter()
        .filter(|d| d["affected_refs"].as_array().is_some_and(|a| a.iter().any(|r| r == case_id.as_str())))
        .filter(|d| !d["code"].as_str().is_some_and(|c| c.starts_with("RETAINED_PRECISION_")))
        .map(|d| d["id"].clone()).collect();
    let ordinary = json!({"case_index":case_index,"case_id":case_id,"material_basis_ref":0,"requested_mode":mode.as_str(),
        "initial":initial,"w2":w2,"formation":formation,"legacy_source":legacy,"diagnostic_refs":diagnostic_refs});

    let case_v = json!({"basis_ref":{"ref_type":"load_case","ref_id":case_id},"ordinary":{"attempt_ref":case_index,"quality_binding":{"kind":"present","index":quality_index}},
        "product_attempt_ref":0,"status":"selected","method":METHOD,"run":run_v,"source_ref":run.source,"source_identity_sha256":source_identity,"selection":selection_v});
    let calls: Vec<Value> = inv.calls().iter().map(|c| json!({"id":c.id,"kind":"case_batch",
        "owner_refs":c.owners.iter().map(|o|match o { k::NativeOwner::Case(i)=>json!({"kind":"case","index":i}), k::NativeOwner::Combination(i)=>json!({"kind":"combination","index":i}) }).collect::<Vec<_>>(),
        "source_refs":c.sources,"run_refs":c.runs,"invocation_before":e.exact(c.invocation_before,"calls[].invocation_before"),
        "invocation_after":e.exact(c.invocation_after,"calls[].invocation_after"),"result":{"kind":"runs"}})).collect();
    let groups: Vec<Value> = inv.groups().iter().map(|g| {
        let preparation = match &g.preparation { k::GroupPreparation::Ready => json!({"kind":"ready"}), k::GroupPreparation::Refused(_) => e.untranslated("groups[].preparation") };
        let stiffness = inv.sources().get(g.first_source).map(|s| sha_hex(&s.stiffness));
        if stiffness.is_none() { e.fail(ReceiptCheck::Association, "groups[].stiffness_sha256"); }
        json!({"id":g.id,"call":g.call,"first_source_ref":g.first_source,
            "source_refs":inv.runs().iter().filter(|r|r.group==Some(g.id)).map(|r|r.source).collect::<Vec<_>>(),
            "stiffness_sha256":stiffness,"preparation":preparation})
    }).collect();
    let builds: Vec<Value> = inv.builds().iter().map(|b| json!({"id":b.id,"group":b.group,"slot":slot_name(b.slot),
        "origin":{"call":b.call,"run":b.run,"physical_record":b.physical_record,"phase":match b.phase { k::BuildPhase::Shared=>"shared", k::BuildPhase::VerificationShared=>"verification_shared" }},
        "state":match b.state { k::BuildState::Success=>"success", k::BuildState::NonbudgetFailure=>"nonbudget_failure", k::BuildState::BudgetFailure=>"budget_failure" },
        "reason":b.reason.as_ref().map(|_|e.untranslated("builds[].reason")),"work":e.exact(b.work,"builds[].work"),"stages":stages(&e,&b.stages,"builds[].stages")})).collect();
    // W1-LME-20B-60B-v1: the meter's own limit and its exact final charge.
    if inv.meter().limit() != INVOCATION_LIMIT {
        return Err(assoc("work.invocation_limit"));
    }
    // C1 §4 body.work: charged equals the final after (before = 0).
    let charged = e.exact(inv.meter().checked_charged(), "work.charged");
    if charged != e.exact(run.work.invocation_after(), "work.charged") {
        return Err(fail(ReceiptCheck::WorkCounterInconsistent, "work.charged"));
    }
    let material = material_basis(&e, pc, raw, case_index)?;
    let mut body = json!({"receipt_version":1,"policy":POLICY,"projection_policy":PROJECTION_POLICY,"work_policy":WORK_POLICY,
        "facade_policy":FACADE_POLICY,"canonicalization":CANONICALIZATION,
        "invocation":{"algorithm":"sha256","profile":CANONICALIZATION,"scope":"actual_request_and_solver_mode","domain":"source_blocks_invocation_v1","value":invocation.borrowed_digest()},
        "publication_sha256":"",
        "work":{"case_limit":CASE_LIMIT,"invocation_limit":INVOCATION_LIMIT,"charged":charged,"execution_order":[{"kind":"case","index":case_index}]},
        "cases":[case_v],"combinations":[],"sources":[source],"material_bases":[material],"calls":calls,"groups":groups,"builds":builds,
        "ordinary_attempts":[ordinary],"product_attempts":[attempt],"legacy_source_work":legacy_source_work});
    e.finish()?;
    if !safe_integers(&body) {
        return Err(fail(ReceiptCheck::Encoding, "body"));
    }
    // Hashes (C1 §3): publication over the final envelope without the receipt, then the body.
    let publication = domain_hash("retained_precision_publication_mp_v2", &env).ok_or(fail(ReceiptCheck::PublicationHashRange, "publication_sha256"))?;
    body["publication_sha256"] = json!(publication);
    let receipt = domain_hash("retained_precision_receipt_mp_v2", &body).ok_or(fail(ReceiptCheck::Encoding, "receipt_sha256"))?;
    env.as_object_mut().ok_or(assoc("envelope"))?.insert("retained_precision".into(), json!({"body":body,"receipt_sha256":receipt}));
    Ok(env)
}

/// Test access to the D39 mapping with its first-failure encoder.
#[cfg(test)]
pub(super) fn test_legacy_source(seed: Option<&rp::LegacySeed>, case_index: usize, work: &mut Vec<Value>)
    -> Result<(Value, Option<String>), ReceiptFailure> {
    let e = Enc::default();
    let mapped = legacy_source(&e, seed, case_index, work)?;
    e.finish()?;
    Ok(mapped)
}

/// Test access to the checked record view and its failure paths.
#[cfg(test)]
pub(super) fn test_record_work(r: &k::AttemptRecord) -> (Vec<u64>, Vec<ReceiptFailure>) {
    let e = Enc::default();
    let w = record_work(&e, r);
    let values = vec![w.wide, w.exact_sum, w.own, w.shared, w.stop_rule, w.verification, w.verification_shared, w.case_charge, w.invocation_increment];
    (values, e.failures.into_inner())
}
/// Test access to the stage-status gate.
#[cfg(test)]
pub(super) fn test_stages(s: &k::StageWork) -> (Value, Vec<ReceiptFailure>) {
    let e = Enc::default();
    let v = stages(&e, s, "stages");
    (v, e.failures.into_inner())
}
/// Test access to the trace Count's exact-or-abandon rule.
#[cfg(test)]
pub(super) fn test_count(w: k::WorkTotal) -> (Value, Vec<ReceiptFailure>) {
    let e = Enc::default();
    let v = e.count(w);
    (v, e.failures.into_inner())
}
/// Test access to a physical record's conservation checks.
#[cfg(test)]
pub(super) fn test_physical(r: &k::AttemptRecord, links: k::RecordBuildLinks) -> Vec<ReceiptFailure> {
    let e = Enc::default();
    let _ = physical(&e, 0, r, links);
    e.failures.into_inner()
}
/// Test access to the run-level conservation checks.
#[cfg(test)]
pub(super) fn test_run_conservation(attempts: &[Value], run: &k::RunOrigins) -> Vec<ReceiptFailure> {
    let e = Enc::default();
    run_conservation(&e, attempts, run);
    e.failures.into_inner()
}
