//! I61 U1 grant 1 (with U2): the private retained-precision serializer.
//!
//! Maps one completed, certified, one-case prepared attempt and its typed
//! ordinary capture (G-b, G-l) to the successor envelope carrying
//! `retained_precision {body, receipt_sha256}` (C1 §3-4, C2 §5, C3), or to a
//! typed [`ReceiptFailure`]. In production it is reached only behind the capture permit
//! (U3, `retained_w1`): D1 Direct calls in the registered build. It never reads diagnostic text
//! or Debug output: a variant this grant does not translate refuses typed
//! (`ReceiptCheck::Untranslated`, closed in grant 2 with G-i and D38).
//!
//! The experiments' emitter (R/I61/receipt_experiment_02-03) is the reference;
//! the differences are the closed gaps G-a (fixed product text), G-b, G-c (D6a,
//! decision 2), G-d, G-e, G-j, G-l, A1 (decision 1), D39 and U2.
// In production, reached only behind the capture permit (U3): D1 Direct calls in the registered build (G6).
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
/// B3b-P (B3-D P-9): the exact successor's identity and profile (reserved 2026-10-03;
/// `semantic_contract_v0_3_physics_retained_1.json`), and its formation definition DEF-E with
/// its H("retained_precision_formation_v1", definition) (B3-D REVISION_01 §1.2). Bound to the
/// in-tree definition and to the table's entry by tests.
pub(super) const EXACT_SEMANTIC_ID: &str = "openpipestress.result_semantics/0.3.0/physics-retained-1";
pub(super) const EXACT_PROFILE_ID: &str = "exact_straight_retained_w1a_v2";
pub(super) const EXACT_DEFINITION_ID: &str = "RP-PREPARED-EXACT-DUAL-v1";
pub(super) const EXACT_DEFINITION_SHA256: &str =
    "5a3bac430df9bbc77484d5419c75880ad40ae209b439e5f928374458025281af";
/// B3b-P (B3-D P-9; REVISION_01 §2, S-1): the route descriptor the serializer takes its
/// route-specific members from, never a module constant: the successor's identity and profile,
/// every product attempt's `definition_id`, the preparation payload's `definition_sha256` (C3
/// §2: "the table-bound H(definition)") and the section terms' `geometry.route`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct RouteWire {
    pub semantic_id: &'static str,
    pub profile_id: &'static str,
    pub definition_id: &'static str,
    pub definition_sha256: &'static str,
    pub geometry_route: &'static str,
}
pub(super) fn route_wire(route: rp::W1Route) -> RouteWire {
    match route {
        rp::W1Route::Preview => RouteWire { semantic_id: RETAINED_SEMANTIC_ID, profile_id: RETAINED_PROFILE_ID,
            definition_id: DEFINITION_ID, definition_sha256: DEFINITION_SHA256, geometry_route: "preview" },
        rp::W1Route::Exact => RouteWire { semantic_id: EXACT_SEMANTIC_ID, profile_id: EXACT_PROFILE_ID,
            definition_id: EXACT_DEFINITION_ID, definition_sha256: EXACT_DEFINITION_SHA256, geometry_route: "exact" },
    }
}
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

/// RV82-N7, the precedence: `Enc` keeps every failure it records and returns the
/// first of them at `finish`. A structural early return (`?` on a Scope,
/// Association or Untranslated refusal from a later step, such as `legacy_source`,
/// `successor_envelope` or `material_basis`) takes precedence over failures the
/// encoder recorded before it. Either way the cause is typed and maps onto
/// C1:68's vocabulary; no successor is emitted.
///
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
    /// A native variant with no wire form (C1 §4): the receipt cannot encode it.
    fn unencodable(&self, path: &'static str) -> Value {
        self.fail(ReceiptCheck::Encoding, path);
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
        k::QuantityId::DirectionalSpringAction { .. } => {
            e.fail(ReceiptCheck::Association, "QuantityId.directional_spring_action");
            Value::Null
        }
        k::QuantityId::SupportForceMagnitude(s) => json!({"tag":"support_force_magnitude","support":s}),
        k::QuantityId::SupportMomentMagnitude(s) => json!({"tag":"support_moment_magnitude","support":s}),
    }
}
/// Outcome (C1 §4): a rejected/failed reason is the attempt-space Reason (G-i).
fn outcome(e: &Enc, o: &k::AttemptOutcome) -> Value {
    match o {
        k::AttemptOutcome::Accepted => json!({"kind":"accepted"}),
        k::AttemptOutcome::Verified => json!({"kind":"verified"}),
        k::AttemptOutcome::Solved => json!({"kind":"solved"}),
        k::AttemptOutcome::Rejected(r) => json!({"kind":"rejected","reason":attempt_reason(e,r)}),
        k::AttemptOutcome::Failed(r) => json!({"kind":"failed","reason":attempt_reason(e,r)}),
    }
}

// ---- G-i: the closed translations (C1 §4 Reason; C2 §2 and §4; C3 §3). Every
// variant is mapped; a variant without a wire form fails `encoding` (C1 §4:
// "unknown variants fail encoding to the named ordinary fallback"). No Debug text.

fn fault_tag(f: k::WorkFault) -> &'static str {
    fault_name(f)
}
fn budget_scope(s: k::BudgetScope) -> &'static str {
    match s {
        k::BudgetScope::Case => "case",
        k::BudgetScope::Invocation => "invocation",
    }
}
fn certificate_issue(i: k::CertificateIssue) -> &'static str {
    match i {
        k::CertificateIssue::Shape => "shape",
        k::CertificateIssue::PairIdentity => "pair_identity",
        k::CertificateIssue::Precision => "precision",
        k::CertificateIssue::RowIdentity => "row_identity",
        k::CertificateIssue::MissingField => "missing_field",
        k::CertificateIssue::NegativeField => "negative_field",
        k::CertificateIssue::NonFinite => "non_finite",
        k::CertificateIssue::NonCanonicalZero => "non_canonical_zero",
        k::CertificateIssue::RadiusClassMismatch => "radius_class_mismatch",
    }
}
fn publication_predicate(p: k::PublicationPredicate) -> &'static str {
    match p {
        k::PublicationPredicate::AbsoluteBound => "absolute_bound",
        k::PublicationPredicate::PublicRelative => "public_relative",
        k::PublicationPredicate::SharperExact => "sharper_exact",
        k::PublicationPredicate::SharperBinary64 => "sharper_binary64",
    }
}
fn sum_error(err: &open_pipe_stress_frame_kernel::exact_sum::SumError) -> Value {
    use open_pipe_stress_frame_kernel::exact_sum::SumError as S;
    json!({"tag":match err { S::NonFinite=>"non_finite", S::AccumulatorOverflow=>"accumulator_overflow", S::NonRepresentable=>"non_representable" }})
}
/// WideError (C2 §2). CountRange and WorkAccounting have no wire form.
fn wide_error(e: &Enc, err: &k::WideError) -> Value {
    let tag = |t: &str| json!({"tag":t});
    match err {
        k::WideError::InvalidPrecision(p) => json!({"tag":"invalid_precision","precision":p}),
        k::WideError::NonFinite => tag("non_finite"),
        k::WideError::ExponentRange => tag("exponent_range"),
        k::WideError::DivisionByZero => tag("division_by_zero"),
        k::WideError::NegativeSqrt => tag("negative_sqrt"),
        k::WideError::NotNormalized => tag("not_normalized"),
        k::WideError::AngleDomain => tag("angle_domain"),
        k::WideError::ArctangentLimit => tag("arctangent_limit"),
        k::WideError::SplitOverflow => tag("split_overflow"),
        k::WideError::OperandPrecision => tag("operand_precision"),
        k::WideError::Accumulator(s) => json!({"tag":"accumulator","error":sum_error(s)}),
        k::WideError::CountRange(_) => e.unencodable("WideError.count_range"),
        k::WideError::WorkAccounting(_) => e.unencodable("WideError.work_accounting"),
    }
}
/// AttemptStop as the Stop union (space "stop"; C2 §2, C3 §3).
fn stop(e: &Enc, s: &k::AttemptStop) -> Value {
    let tag = |t: &str| json!({"space":"stop","tag":t});
    match s {
        k::AttemptStop::CountRange(name) => json!({"space":"stop","tag":"count_range","name":name}),
        k::AttemptStop::WorkAccounting(f) => json!({"space":"stop","tag":"work_accounting","fault":fault_tag(*f)}),
        k::AttemptStop::Budget(scope) => json!({"space":"stop","tag":"budget","scope":budget_scope(*scope)}),
        k::AttemptStop::Span => tag("span"),
        k::AttemptStop::Exponent => tag("exponent"),
        k::AttemptStop::Arithmetic(w) => json!({"space":"stop","tag":"arithmetic","error":wide_error(e,w)}),
        k::AttemptStop::Structure => tag("structure"),
        k::AttemptStop::Pivot { global_dof } => json!({"space":"stop","tag":"pivot","global_dof":global_dof}),
        k::AttemptStop::ZeroDiagonal { global_dof } => json!({"space":"stop","tag":"zero_diagonal","global_dof":global_dof}),
        k::AttemptStop::NegativeEnergy { i, j } => json!({"space":"stop","tag":"negative_energy","i":i,"j":j}),
        k::AttemptStop::Condition => tag("condition"),
        k::AttemptStop::ResidualGate { global_dof } => json!({"space":"stop","tag":"residual_gate","global_dof":global_dof}),
        k::AttemptStop::ResolutionScale { body, kind } => json!({"space":"stop","tag":"resolution_scale","body":body,"kind":kind_name(*kind)}),
        k::AttemptStop::PublicationCertificate { index, issue } => {
            json!({"space":"stop","tag":"publication_certificate","index":index,"issue":certificate_issue(*issue)})
        }
    }
}
/// AttemptReason (space "attempt"; C2 §2).
fn attempt_reason(e: &Enc, r: &k::AttemptReason) -> Value {
    let qbk = |tag: &str, q: &k::QuantityId, body: &u32, kind: &k::Kind| {
        json!({"space":"attempt","tag":tag,"quantity":quantity(e,*q),"body":body,"kind":kind_name(*kind)})
    };
    match r {
        k::AttemptReason::Stop(s) => json!({"space":"attempt","tag":"stop","stop":stop(e,s)}),
        k::AttemptReason::StopRule { quantity: q, body, kind } => qbk("stop_rule", q, body, kind),
        k::AttemptReason::VerificationEstimate { quantity: q, body, kind } => qbk("verification_estimate", q, body, kind),
        k::AttemptReason::Charge { quantity: q, body, kind } => qbk("charge", q, body, kind),
        k::AttemptReason::VerificationFailed => json!({"space":"attempt","tag":"verification_failed"}),
        k::AttemptReason::PublicationEnclosure { quantity: q, body, kind, predicate } => json!({"space":"attempt","tag":"publication_enclosure",
            "quantity":quantity(e,*q),"body":body,"kind":kind_name(*kind),"predicate":publication_predicate(*predicate)}),
        k::AttemptReason::Uc { body } => json!({"space":"attempt","tag":"uc","body":body}),
        k::AttemptReason::Theta { body } => json!({"space":"attempt","tag":"theta","body":body}),
        k::AttemptReason::GValidity { member } => json!({"space":"attempt","tag":"g_validity","member":member}),
    }
}
/// UnresolvedReason (space "unresolved"; C2 §2). The schema's work_accounting
/// carries the fault only; the private prior stop is not a wire member.
fn unresolved(e: &Enc, r: &k::UnresolvedReason) -> Value {
    let tag = |t: &str| json!({"space":"unresolved","tag":t});
    match r {
        k::UnresolvedReason::CountRange(name) => json!({"space":"unresolved","tag":"count_range","name":name}),
        k::UnresolvedReason::WorkAccounting { fault, .. } => json!({"space":"unresolved","tag":"work_accounting","fault":fault_tag(*fault)}),
        k::UnresolvedReason::Ceiling => tag("ceiling"),
        k::UnresolvedReason::Budget(scope) => json!({"space":"unresolved","tag":"budget","scope":budget_scope(*scope)}),
        k::UnresolvedReason::ExactSumSpan => tag("exact_sum_span"),
        k::UnresolvedReason::ExponentRange => tag("exponent_range"),
        k::UnresolvedReason::ZeroDiagonal { global_dof } => json!({"space":"unresolved","tag":"zero_diagonal","global_dof":global_dof}),
        k::UnresolvedReason::Arithmetic(w) => json!({"space":"unresolved","tag":"arithmetic","error":wide_error(e,w)}),
        k::UnresolvedReason::ResolutionScaleUnencodable { body, kind } => {
            json!({"space":"unresolved","tag":"resolution_scale_unencodable","body":body,"kind":kind_name(*kind)})
        }
        k::UnresolvedReason::CertifiedBoundUnencodable { body } => json!({"space":"unresolved","tag":"certified_bound_unencodable","body":body}),
        k::UnresolvedReason::PublicationCertificate { index, issue } => {
            json!({"space":"unresolved","tag":"publication_certificate","index":index,"issue":certificate_issue(*issue)})
        }
    }
}
/// Refusal (space "refusal"; C2 §2).
fn refusal(e: &Enc, r: &k::Refusal) -> Value {
    const P: &str = "Refusal";
    match r {
        k::Refusal::MechanismWitnessed { body, rigid_parameters } => json!({"space":"refusal","tag":"mechanism_witnessed","body":body,
            "rigid_parameters":rigid_parameters.iter().map(|x|e.bits(*x,P)).collect::<Vec<_>>()}),
        k::Refusal::GeometryUnavailable { body, error } => {
            json!({"space":"refusal","tag":"geometry_unavailable","body":body,"error":structural_error(e,error)})
        }
        k::Refusal::NegativeEnergy { i, j } => json!({"space":"refusal","tag":"negative_energy","i":i,"j":j}),
        k::Refusal::LedgerUnavailable(k::LedgerRefusal::Accumulator(s)) => {
            json!({"space":"refusal","tag":"ledger_unavailable","error":{"tag":"accumulator","error":sum_error(s)}})
        }
        k::Refusal::Structure => json!({"space":"refusal","tag":"structure"}),
    }
}
/// BlockRefusal (C1 §4): a per-block `usize::MAX` row is the `block_step` sentinel.
fn block_refusal(b: &k::BlockRefusal) -> Value {
    let pass = match b.refusal.pass {
        k::BoundPass::Forward => "forward",
        k::BoundPass::Pivot => "pivot",
        k::BoundPass::Backward => "backward",
        k::BoundPass::NlColumn => "nl_column",
        k::BoundPass::NlScale => "nl_scale",
        k::BoundPass::NlRow => "nl_row",
        k::BoundPass::Form => "form",
        k::BoundPass::Need => "need",
        k::BoundPass::Sigma => "sigma",
        k::BoundPass::ShiftFactor => "shift_factor",
        k::BoundPass::ShiftForm => "shift_form",
    };
    let location = if b.refusal.row == usize::MAX { json!({"kind":"block_step"}) } else { json!({"kind":"row","index":b.refusal.row}) };
    json!({"block":b.block,"bound":match b.bound { k::CertifiedBound::Uc=>"uc", k::CertifiedBound::S=>"s" },
        "kind":match b.refusal.kind { k::RefusalKind::Span=>"span", k::RefusalKind::Exponent=>"exponent" },"pass":pass,"location":location})
}
/// NumericError (C3 §3).
fn numeric_error(e: &Enc, err: &k::NumericError) -> Value {
    let kind = |t: &str| json!({"kind":t});
    match err {
        k::NumericError::Arithmetic(s) => json!({"kind":"arithmetic","cause":stop(e,s)}),
        k::NumericError::NonFinite => kind("non_finite"),
        k::NumericError::NonpositiveSource => kind("nonpositive_source"),
        k::NumericError::InvalidGeometry => kind("invalid_geometry"),
        k::NumericError::InvalidMaterial => kind("invalid_material"),
        k::NumericError::MaterialBits => kind("material_bits"),
        k::NumericError::TemperatureOrder => kind("temperature_order"),
        k::NumericError::NonpositiveDenominator => kind("nonpositive_denominator"),
        k::NumericError::AxisBits => kind("axis_bits"),
        k::NumericError::Binary64Range => kind("binary64_range"),
    }
}
/// ViewIssue (C3 §3).
fn view_issue(v: &k::ViewFailure) -> Value {
    let kind = |t: &str| json!({"kind":t});
    match v {
        k::ViewFailure::Certificate(i) => json!({"kind":"certificate","issue":certificate_issue(*i)}),
        k::ViewFailure::ForeignOwner => kind("foreign_owner"),
        k::ViewFailure::UnsupportedCombination => kind("unsupported_combination"),
        k::ViewFailure::VerificationCache => kind("verification_cache"),
        k::ViewFailure::Ordering => kind("ordering"),
        k::ViewFailure::BodyBound => kind("body_bound"),
        k::ViewFailure::CountRange => kind("count_range"),
        k::ViewFailure::Work(f) => json!({"kind":"work","fault":fault_tag(*f)}),
    }
}
/// BridgeError (C3 §3). Alpha's private endpoint is never serialized.
fn bridge_error(e: &Enc, b: &k::BridgeFailure<'_>) -> Value {
    let kind = |t: &str| json!({"kind":t});
    match b {
        k::BridgeFailure::View(v) => json!({"kind":"view","issue":view_issue(v)}),
        k::BridgeFailure::Numeric(n) => json!({"kind":"numeric","cause":numeric_error(e,n)}),
        k::BridgeFailure::MemberOwner => kind("member_owner"),
        k::BridgeFailure::UnsupportedDirectionalSpring => kind("unsupported_directional_spring"),
        k::BridgeFailure::MissingRadius(row) => json!({"kind":"missing_radius","row":row}),
        k::BridgeFailure::MissingUniquenessWarrant(body) => json!({"kind":"missing_uniqueness_warrant","body":body}),
        k::BridgeFailure::RowIdentity(row) => json!({"kind":"row_identity","row":row}),
        k::BridgeFailure::CountRange => kind("count_range"),
        k::BridgeFailure::Storage => kind("storage"),
        k::BridgeFailure::AlphaCondition { block } => json!({"kind":"alpha_condition","block":block}),
    }
}
/// ProductError (C3 §3) through the kernel's typed view.
fn product_error(e: &Enc, f: &k::ProductFailure) -> Value {
    let mut copies = k::TraceCopyWork::default();
    let v = product_error_view(e, f.typed_cause(&mut copies));
    if !copies.status().is_exact() {
        e.fail(ReceiptCheck::Encoding, "ProductError");
    }
    v
}
fn product_error_view(e: &Enc, view: k::ProductFailureView<'_>) -> Value {
    match view {
        k::ProductFailureView::Association(d) => json!({"kind":"association","detail":d}),
        k::ProductFailureView::Numeric(n) => json!({"kind":"numeric","cause":numeric_error(e,n)}),
        k::ProductFailureView::Native(b) => json!({"kind":"native_source","cause":bridge_error(e,&b)}),
        k::ProductFailureView::Accounting(fault) => json!({"kind":"work_accounting","fault":fault_tag(fault)}),
        k::ProductFailureView::CountRange(d) => json!({"kind":"count_range","detail":d}),
        k::ProductFailureView::Storage => json!({"kind":"storage"}),
        k::ProductFailureView::G5a(d) => json!({"kind":"g5a","detail":d}),
        k::ProductFailureView::Predicate { row, predicate } => json!({"kind":"numeric_predicate","row":row,"predicate":match predicate {
            k::ProductPredicate::Absolute=>"absolute", k::ProductPredicate::SharperExact=>"sharper_exact",
            k::ProductPredicate::SharperBinary64=>"sharper_binary64", k::ProductPredicate::DecimalSi=>"decimal_si",
            k::ProductPredicate::DecimalRaw=>"decimal_raw", k::ProductPredicate::InputDerived=>"input_derived" }}),
        k::ProductFailureView::Helper(h) => json!({"kind":"numeric_helper","cause":match h {
            k::HelperFailure::Arithmetic(s)=>json!({"kind":"arithmetic","cause":stop(e,s)}),
            k::HelperFailure::InvalidSmallBoundInput=>json!({"kind":"invalid_small_bound_input"}),
            k::HelperFailure::Binary64Range=>json!({"kind":"binary64_range"}),
            k::HelperFailure::Invariant=>json!({"kind":"invariant"}) }}),
    }
}
fn member_property(p: k::MemberProperty) -> &'static str {
    match p {
        k::MemberProperty::ElasticModulus => "elastic_modulus",
        k::MemberProperty::ShearModulus => "shear_modulus",
        k::MemberProperty::Area => "area",
        k::MemberProperty::SecondMomentY => "second_moment_y",
        k::MemberProperty::SecondMomentZ => "second_moment_z",
        k::MemberProperty::TorsionConstant => "torsion_constant",
        k::MemberProperty::YReference => "y_reference",
    }
}
/// SourceError (C2 §3). The directional variants are impossible from the declared
/// empty directional-spring list: an association failure (C2), never a wire form.
fn source_error(e: &Enc, err: &k::SourceError) -> Value {
    use k::SourceError as S;
    let id = |t: &str, id: &u32| json!({"tag":t,"id":id});
    let member = |t: &str, m: &u32| json!({"tag":t,"member":m});
    let prop = |t: &str, m: &u32, p: &k::MemberProperty| json!({"tag":t,"member":m,"property":member_property(*p)});
    match err {
        S::CountRange(name) => json!({"tag":"count_range","name":name}),
        S::NoNodes => json!({"tag":"no_nodes"}),
        S::NonFiniteCoordinate { node } => json!({"tag":"non_finite_coordinate","node":node}),
        S::NodeOutOfRange { node } => json!({"tag":"node_out_of_range","node":node}),
        S::DuplicateMemberId { id: i } => id("duplicate_member_id", i),
        S::RepeatedMemberNode { member: m } => member("repeated_member_node", m),
        S::NonFiniteProperty { member: m, property } => prop("non_finite_property", m, property),
        S::NonPositiveProperty { member: m, property } => prop("non_positive_property", m, property),
        S::SubnormalDerivedPrimitive { member: m, property } => prop("subnormal_derived_primitive", m, property),
        S::ZeroLength { member: m } => member("zero_length", m),
        S::DegenerateAxis { member: m } => member("degenerate_axis", m),
        S::DuplicateSpringId { id: i } => id("duplicate_spring_id", i),
        S::NonPositiveSpring { id: i } => id("non_positive_spring", i),
        S::ZeroDirection { .. } | S::NonFiniteDirection { .. } => {
            e.fail(ReceiptCheck::Association, "SourceError.directional_spring");
            Value::Null
        }
        S::DuplicateConstraint { dof: d } => json!({"tag":"duplicate_constraint","dof":dof(*d)}),
        S::NonFiniteValue { dof: d } => json!({"tag":"non_finite_value","dof":dof(*d)}),
        S::EmptyLoadSource { dof: d } => json!({"tag":"empty_load_source","dof":dof(*d)}),
        S::DuplicateStationId { id: i } => id("duplicate_station_id", i),
        S::UnknownMember { station, member: m } => json!({"tag":"unknown_member","station":station,"member":m}),
        S::StationOutOfRange { id: i } => id("station_out_of_range", i),
        S::DuplicateSupportId { id: i } => id("duplicate_support_id", i),
        S::SupportMismatch { id: i } => id("support_mismatch", i),
    }
}
/// OriginError (C3 §3).
fn origin_error(err: &k::OriginError) -> Value {
    match err {
        k::OriginError::CountRange(d) => json!({"kind":"count_range","detail":d}),
        k::OriginError::Capacity => json!({"kind":"capacity"}),
        k::OriginError::Allocation => json!({"kind":"allocation"}),
        k::OriginError::MissingSelectedOrigin { operand } => json!({"kind":"missing_selected_origin","operand":operand}),
    }
}
fn adapter_event(e: &Enc, ev: rp::AdapterEvent) -> Value {
    EVENTS.get(ev as usize).map_or_else(|| { e.fail(ReceiptCheck::Association, "AdapterEvent"); Value::Null }, |n| json!(n))
}
/// CaptureError (C3 §3). Original strings stay strings.
fn capture_error(e: &Enc, err: &rp::CaptureError) -> Value {
    match err {
        rp::CaptureError::Association(d) => json!({"kind":"association","detail":d}),
        rp::CaptureError::CountRange(d) => json!({"kind":"count_range","detail":d}),
        rp::CaptureError::Storage(d) => json!({"kind":"storage","detail":d}),
        rp::CaptureError::Accounting(rp::AdapterFault::Overflow(ev)) => json!({"kind":"accounting","event":adapter_event(e,*ev)}),
        rp::CaptureError::Source(s) => json!({"kind":"source","cause":source_error(e,s)}),
        rp::CaptureError::Origin(o) => json!({"kind":"origin","cause":origin_error(o)}),
        rp::CaptureError::NativeUnavailable => json!({"kind":"native_unavailable"}),
        rp::CaptureError::PreparedArithmetic(o) => json!({"kind":"prepared_arithmetic","cause":op_error(o)}),
        rp::CaptureError::PreparedProof(f) => json!({"kind":"prepared_proof","cause":product_error(e,f)}),
        rp::CaptureError::PreparedAttemptConsumed => json!({"kind":"prepared_attempt_consumed"}),
    }
}
/// G5aError (C3 §3; the schema names the force/moment index `quantity_kind`, 0 or 1).
fn g5a_error(e: &Enc, err: &rp::G5aFailure) -> Value {
    let quantity_kind = |k: &usize| if *k <= 1 { json!(k) } else { e.unencodable("G5aError.quantity_kind") };
    match err {
        rp::G5aFailure::Accounting(rp::AdapterFault::Overflow(ev)) => json!({"kind":"accounting","event":adapter_event(e,*ev)}),
        rp::G5aFailure::Shape(d) => json!({"kind":"shape","detail":d}),
        rp::G5aFailure::Summary(d) => json!({"kind":"summary","detail":d}),
        rp::G5aFailure::Zero { row } => json!({"kind":"zero","row":row}),
        rp::G5aFailure::Sanity { body, kind } => json!({"kind":"sanity","body":body,"quantity_kind":quantity_kind(kind)}),
        rp::G5aFailure::Lower { member, kind } => json!({"kind":"lower","member":member,"quantity_kind":quantity_kind(kind)}),
        rp::G5aFailure::Operational { member, cause } => json!({"kind":"operational","member_index":member,"cause":op_error(cause)}),
        rp::G5aFailure::Arithmetic(cause) => json!({"kind":"arithmetic","cause":op_error(cause)}),
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
            None => e.unencodable("SectionError.property"),
        },
        k::SectionPreparationError::PrimitiveRange(p) => match P.get(*p) {
            Some(name) => json!({"kind":"primitive_range","property":name}),
            None => e.unencodable("SectionError.property"),
        },
        k::SectionPreparationError::Arithmetic(c) => json!({"kind":"arithmetic","cause":numeric_error(e,c.cause())}),
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
        _ => e.unencodable("ordinary_attempts[].w2.trigger"),
    }
}

/// G-a with T1 (a): identity, profile, method token, the omitted legacy
/// disclosure (by its captured id) and the selected-case diagnostic.
fn successor_envelope(env: &mut Value, route: rp::W1Route, case_id: &str, omit: Option<&str>) -> Result<String, ReceiptFailure> {
    successor_identity(env, route);
    selected_case_envelope(env, case_id, omit)
}
/// The successor's identity and profile (G-a), set once per successor: the route's (B3b-P, P-9;
/// the profile's `limitations` stay the base producer's).
fn successor_identity(env: &mut Value, route: rp::W1Route) {
    let wire = route_wire(route);
    env["producer"]["semantic_contract_id"] = json!(wire.semantic_id);
    env["formulation_basis"]["profile_id"] = json!(wire.profile_id);
}
/// One selected case's part of G-a with T1 (a): its rows' method token, its omitted legacy
/// disclosure and its selected-case diagnostic (appended).
fn selected_case_envelope(env: &mut Value, case_id: &str, omit: Option<&str>) -> Result<String, ReceiptFailure> {
    let assoc = |p| fail(ReceiptCheck::Association, p);
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
/// On an unselected (unavailable) case the disclosure is kept and referenced
/// (C2:160 "the original diagnostic_ref preserves original source failure disclosure").
fn legacy_source(e: &Enc, seed: Option<&rp::LegacySeed>, case_index: usize, work: &mut Vec<Value>, selected: bool)
    -> Result<(Value, Option<String>), ReceiptFailure> {
    let disclosed = |d: &String| if selected { (Value::Null, Some(d.clone())) } else { (json!(d), None) };
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
            let (reference, omit) = disclosed(diagnostic_ref);
            (json!({"disposition":"declined_without_attempt","diagnostic_ref":reference,"work_ref":work.len()-1}), omit)
        }
        Some(rp::LegacySeed::Unavailable { work: w, diagnostic_ref }) => {
            work.push(entry(w));
            let (reference, omit) = disclosed(diagnostic_ref);
            (json!({"disposition":"unavailable","diagnostic_ref":reference,"work_ref":work.len()-1}), omit)
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
                _ => {
                    e.fail(ReceiptCheck::Association, "ordinary_attempts[].initial.outcome");
                    Value::Null
                }
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

/// The one material basis (D1.5) of the requested cases `0..cases` (B1 SP: every case's base
/// selector; at c = 1 the one case).
fn material_basis(e: &Enc, capture: &rp::ProductCapture, raw: &Value, cases: usize) -> Result<Value, ReceiptFailure> {
    let assoc = |p| fail(ReceiptCheck::Association, p);
    for case_index in 0..cases {
        let case = &raw["model"]["load_cases"][case_index];
        // Grant 1 maps the base selector; a named or temperature basis is wider scope.
        if !case.get("modulus_basis_ref").is_none_or(Value::is_null) || !case.get("modulus_basis_temperature").is_none_or(Value::is_null) {
            return Err(fail(ReceiptCheck::Untranslated, "material_bases[].selector"));
        }
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
        const P: &str = "material_bases[].materials[]";
        let shear_origin = match capture.route() {
            rp::W1Route::Preview => explicit_g_origin(input)?,
            rp::W1Route::Exact => derived_e_nu_origin(e, capture, i)?,
        };
        materials.push(json!({"input_index":i,"id":id,"elastic_modulus":e.bits(*modulus,P),"shear_modulus":e.bits(*shear,P),
            "shear_origin":shear_origin,"selection":{"kind":"base"}}));
    }
    Ok(json!({"index":0,"selector":{"kind":"base"},"materials":materials,"case_indices":(0..cases).collect::<Vec<_>>()}))
}
/// The preview route's shear origin: an explicit G is the request's own member; a derived E/ν
/// basis there is wider scope.
fn explicit_g_origin(input: &Value) -> Result<Value, ReceiptFailure> {
    if input.get("shear_modulus").is_none_or(Value::is_null) {
        return Err(fail(ReceiptCheck::Untranslated, "material_bases[].materials[].shear_origin"));
    }
    Ok(json!({"kind":"explicit_g"}))
}
/// B3b-P (B3-D P-9, §1.2): the exact route's shear origin: Ĝ derived from the base common E and
/// ν (`homogeneous_isotropic_E_nu_v1`), with ν's bits as captured (any authored G is ignored by
/// the exact route, which publishes Ĝ as `shear_modulus`).
fn derived_e_nu_origin(e: &Enc, capture: &rp::ProductCapture, input_index: usize) -> Result<Value, ReceiptFailure> {
    let nu = capture.material_nu.get(input_index).copied().flatten()
        .ok_or(fail(ReceiptCheck::Association, "material_bases[].materials[].shear_origin.poisson_ratio"))?;
    Ok(json!({"kind":"derived_e_nu","poisson_ratio":e.bits(nu,"material_bases[].materials[].shear_origin"),
        "constitutive_basis":rp::EXACT_CONSTITUTIVE_BASIS}))
}

#[allow(clippy::too_many_arguments)]
/// The CaseSource binding (C1 §3; C2 §3) of the source the kernel registered for
/// this run: the selected owner's bound source, or the prepared source of an
/// unavailable run (which must equal the registered identity bytes' owner).
fn case_source(e: &Enc, capture: &rp::ProductCapture, source: &k::PrimitiveSource, inv: &k::RecordedInvocation,
    run: &k::RunOrigins, case_index: usize, case_id: &str, attempt: &rr::PreparedAttemptView<'_>) -> Result<Value, ReceiptFailure> {
    case_source_at(e, capture, source, inv, run.source, case_index, case_id, attempt.operational_new)
}
/// A CaseSource at its registered source id (B1: its Run's source; B2-P: also an operand
/// preparation's registration, which has no Run), with the section terms of its preparation's
/// new operational records.
#[allow(clippy::too_many_arguments)]
fn case_source_at(e: &Enc, capture: &rp::ProductCapture, source: &k::PrimitiveSource, inv: &k::RecordedInvocation,
    source_id: usize, case_index: usize, case_id: &str, operational_new: &[rp::OperationalSpent]) -> Result<Value, ReceiptFailure> {
    // B1 SP (T-11): `sources[]` is in registration order, so a source's index is the kernel's
    // id of the source its Run used (0 at c = 1).
    let assoc = |p| fail(ReceiptCheck::Association, p);
    const P: &str = "sources[]";
    let origin = inv.sources().get(source_id).ok_or(assoc("sources[].kernel_source_sha256"))?;
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
            else if s.fraction == 0.75 { json!("quarter_3") } else { e.unencodable("sources[].stations[].location") };
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
        // C2 (CONTRACT_DELTA:104): the stable `constructor_ordinal` is the term's ordinal in the
        // constructor's input, its authored primitive-load index, not its canonical position (RR
        // "I98's B2-W verified; …"); the array order stays kernel canonical.
        nodal_terms.push(json!({"constructor_ordinal":t.original,"source_id":l.source_id,"primitive_load_index":t.original,"dof":dof(l.dof),"value":e.bits(l.value,P)}));
    }
    let mut section_terms = Vec::with_capacity(capture.facts.len());
    for (i, f) in capture.facts.iter().enumerate() {
        let op = operational_new.get(i).and_then(|o| o.result.as_ref().ok()).ok_or(assoc("sources[].section_terms[]"))?;
        section_terms.push(json!({"member":i,"area":e.bits(f.area,P),"section_modulus":e.bits(f.section_modulus,P),"length":e.bits(op.length,P),
            "axial_stiffness":e.bits(op.axial,P),"torsional_stiffness":e.bits(op.torsion,P),
            "geometry":{"route":route_wire(capture.route()).geometry_route,"normalized_od":e.bits(f.diameter,P),"effective_wall":e.bits(f.effective_wall,P),"actual_radius":e.bits(f.radius,P),
                "actual_second_moment":e.bits(f.second_moment,P),"actual_polar_moment":e.bits(f.torsion_constant,P)}}));
    }
    Ok(json!({"index":source_id,"owner":{"kind":"case","case_index":case_index,"case_id":case_id},"material_basis_ref":0,
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
    json!({"index":i,"precision":r.precision,"role":role,"outcome":outcome(e,&r.outcome),"residual_basis":r.residual_basis,"corrections":r.corrections,
        "pivot_margin_min":r.pivot_margin_min.map(|x|e.bits(x,P)),"rcond":r.rcond.map(|x|e.bits(x,P)),"residual_worst":r.residual_worst.map(|x|e.bits(x,P)),"gate":gate,
        "work":{"wide_lme":work.wide,"exact_sum_lme":work.exact_sum,"own_lme":work.own,"shared_lme":work.shared,"stop_rule_lme":work.stop_rule,
            "verification_lme":work.verification,"verification_shared_lme":work.verification_shared,
            "own_stages":stages(e,&r.stages,"run.records[].work.own_stages"),"shared_stages":stages(e,&r.shared_stages,"run.records[].work.shared_stages"),
            "shared_built_here":r.shared_built_here,"verification_shared_built_here":r.verification_shared_built_here},
        "storage":{"pattern_entries":r.storage.pattern_entries,"profile_entries":r.storage.profile_entries,"limbs_per_entry":r.storage.limbs_per_entry},
        "verification":verification,"bound_refusals":r.bound_refusals.iter().map(block_refusal).collect::<Vec<_>>(),"shared_build_ref":links.shared,"verification_shared_build_ref":links.verification_shared})
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
                // C1 §1 item 2: a failed verification keeps its failure reason; a
                // VerificationThenCandidate's earlier phase completed.
                let (phase, reason) = match (&r.role, &r.outcome) {
                    (k::AttemptRole::VerificationThenCandidate, _) | (_, k::AttemptOutcome::Verified | k::AttemptOutcome::Solved) => ("completed", Value::Null),
                    (_, k::AttemptOutcome::Failed(reason)) => ("failed", attempt_reason(e, reason)),
                    _ => {
                        e.fail(ReceiptCheck::Association, "run.attempts[].verification.phase");
                        ("failed", Value::Null)
                    }
                };
                out[last]["verification"] = json!({"record":i,"precision":r.precision,"phase":phase,"reason":reason});
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
    let case = e.exact(run.work.case(), "cases[].run.case_charge");
    let before = e.exact(run.work.invocation_before(), "cases[].run.invocation_before");
    let increment = e.exact(run.work.invocation_increment(), "cases[].run.invocation_increment");
    let after = e.exact(run.work.invocation_after(), "cases[].run.invocation_after");
    run_conservation_amounts(e, attempts, case, before, increment, after);
}

/// The run-level conservation checks over the run's four exact amounts. Split
/// from `run_conservation` (RV82 N1′) so that each check, including R08's
/// `invocation_after` at its call site, is pinned by a test: `RunWork` cannot be
/// constructed from PP, and native runs are self-consistent.
fn run_conservation_amounts(e: &Enc, attempts: &[Value], case: u64, before: u64, increment: u64, after: u64) {
    let total = |key: &str, path| {
        let parts: Vec<u64> = attempts.iter().map(|a| a[key].as_u64().unwrap_or(u64::MAX)).collect();
        e.sum(&parts, path)
    };
    for (ok, path) in [
        (total("case_charge", "cases[].run.case_charge") == case, "cases[].run.case_charge"),
        (total("invocation_increment", "cases[].run.invocation_increment") == increment, "cases[].run.invocation_increment"),
        (after_conserved(e, before, increment, after), "cases[].run.invocation_after"),
    ] {
        if !ok {
            e.fail(ReceiptCheck::WorkCounterInconsistent, path);
        }
    }
}

/// C1 §1: invocation_after = invocation_before + invocation_increment.
fn after_conserved(e: &Enc, before: u64, increment: u64, after: u64) -> bool {
    e.sum(&[before, increment], "cases[].run.invocation_after") == after
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
fn check(e: &Enc, c: &rr::CheckRef<'_>, capture: &rp::ProductCapture) -> Value {
    match c {
        rr::CheckRef::NotEntered => json!({"kind":"not_entered"}),
        rr::CheckRef::Passed => json!({"kind":"passed"}),
        rr::CheckRef::Failed(f) => json!({"kind":"failed","error":public_error(e,f,capture)}),
    }
}
/// PublicError (C3 §3 PublicFailure). A native failure with a recorded Run is
/// `native{run_ref}`; before any Run it is D38's `capture{cause}` with the actual
/// cause. Observable/G5a/Numeric unwrap their actually captured errors.
fn public_error(e: &Enc, f: &rr::FailureRef<'_>, capture: &rp::ProductCapture) -> Value {
    let captured = |cause: Option<Value>, path: &'static str| cause.unwrap_or_else(|| {
        e.fail(ReceiptCheck::Association, path);
        Value::Null
    });
    match f {
        rr::FailureRef::Preparation { capture: c, section } => json!({"kind":"preparation","capture":capture_error(e,c),
            "section":section.map(|s|section_error(e,s))}),
        rr::FailureRef::Native(c) => match capture.native.as_ref() {
            Some(case) => json!({"kind":"native","run_ref":case.run}),
            None => json!({"kind":"capture","cause":capture_error(e,c)}),
        },
        rr::FailureRef::Candidate(err) => match err {
            rp::PreparedCandidateError::Capture(c) => json!({"kind":"capture","cause":capture_error(e,c)}),
            rp::PreparedCandidateError::Proof(p) => json!({"kind":"proof","cause":product_error(e,p.failure())}),
            rp::PreparedCandidateError::Values { failure, proof } => {
                json!({"kind":"values","cause":product_error(e,&failure.failure),"proof":product_error(e,proof.failure())})
            }
            rp::PreparedCandidateError::Abandoned { cause, proof } => {
                json!({"kind":"abandoned","cause":capture_error(e,cause),"proof":product_error(e,proof.failure())})
            }
            rp::PreparedCandidateError::Numeric => json!({"kind":"numeric","cause":capture.numeric_failure.as_ref().map(|n|product_error(e,n))}),
            rp::PreparedCandidateError::Observable => json!({"kind":"observable",
                "cause":captured(capture.observable_error.as_ref().map(|c|capture_error(e,c)), "PublicError.observable")}),
            rp::PreparedCandidateError::G5a => json!({"kind":"g5a",
                "cause":captured(capture.g5a_error.as_ref().map(|g|g5a_error(e,g)), "PublicError.g5a")}),
        },
        rr::FailureRef::Proof(p) => json!({"kind":"proof","cause":product_error(e,p)}),
        rr::FailureRef::Observable(c) => json!({"kind":"observable","cause":capture_error(e,c)}),
        rr::FailureRef::G5a(g) => json!({"kind":"g5a","cause":g5a_error(e,g)}),
    }
}
fn proof_trace(e: &Enc, view: &rr::PreparedAttemptView<'_>, capture: &rp::ProductCapture) -> Value {
    const P: &str = "product_attempts[].proof";
    let Some(p) = view.proof.as_ref() else { return Value::Null };
    let lanes: Vec<Value> = p.lanes.iter().flatten().map(|l| {
        let error = match &l.result { Ok(()) => Value::Null, Err(b) => bridge_error(e,b) };
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
        "checks":{"certificate":check(e,&view.checks[0],capture),"observables":check(e,&view.checks[1],capture),"g5a":check(e,&view.checks[2],capture)},"summary_coverage":coverage})
}
#[allow(clippy::too_many_arguments)]
fn product_attempt(e: &Enc, view: &rr::PreparedAttemptView<'_>, id: usize, case_index: usize, source_ref: Option<usize>, run_ref: Option<usize>,
    result: Value, capture: &rp::ProductCapture) -> Value {
    let members = preparation_members(e, view.members, view.preparation_work);
    let names = ["preparation", "native", "proof_start", "projection", "maxima", "values", "aliases", "certificate", "observables", "g5a"];
    let mut stages_v = Map::new();
    for (n, s) in names.iter().zip(view.stages.iter()) {
        stages_v.insert((*n).into(), stage_name(e, *s));
    }
    json!({"id":id,"definition_id":route_wire(capture.route()).definition_id,"owner_ref":{"kind":"case","index":case_index},"ordinary_attempt_ref":case_index,"material_basis_ref":0,
        "source_ref":source_ref,"run_ref":run_ref,"result":result,"preparation":{"members":members},"stages":stages_v,"proof":proof_trace(e,view,capture),
        "adapter":adapter_value(e,view.adapter),
        "operational":operational_value(e,view.old_coverage,view.operational_old,view.operational_new),
        "overlay_work":scalar(view.overlay_work),"g5a_work":scalar(view.g5a_work)})
}
/// A C3 attempt's `adapter` member: its terminal snapshot's counts, fault and capacities.
fn adapter_value(e: &Enc, a: &rr::PrivateAdapterSnapshot) -> Value {
    json!({"counts":a.counts,"fault":a.fault.map(|rp::AdapterFault::Overflow(ev)|json!({"kind":"overflow","event":EVENTS.get(ev as usize).map_or_else(||e.untranslated("product_attempts[].adapter.fault.event"),|n|json!(n))})),
        "prepared_capacity_bytes":a.prepared_capacity_bytes,"observation_capacity_bytes":a.observation_capacity_bytes,"support_capacity_bytes":a.support_capacity_bytes})
}
/// A C3 attempt's `operational` member.
fn operational_value(e: &Enc, coverage: rr::OldCoverage, old: &[rp::OperationalSpent], new: &[rp::OperationalSpent]) -> Value {
    json!({"old_coverage":match coverage { rr::OldCoverage::Complete=>"complete", rr::OldCoverage::CapturedPrefix=>"captured_prefix" },
        "old":old.iter().map(|o|operational(e,o)).collect::<Vec<_>>(),"new":new.iter().map(|o|operational(e,o)).collect::<Vec<_>>()})
}
/// C3's `preparation.members`: each member's old source and facts, its section result and work.
fn preparation_members(e: &Enc, entries: &[rr::PreparationEntry], preparation_work: &[k::SectionPreparationWork]) -> Vec<Value> {
    const PROPS: [&str; 5] = ["area", "second_moment", "polar_moment", "section_modulus", "radius"];
    const P: &str = "product_attempts[].preparation";
    let mut copies = k::TraceCopyWork::default();
    entries.iter().map(|m| {
        let Some(w) = preparation_work.get(m.work_index) else {
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
    }).collect()
}
/// C3 §2's preparation payload; its `definition_sha256` is the route's table-bound definition H
/// (B3b-P; B3-D REVISION_01 S-1): DEF-E's on the exact route, DEF-O's on the preview route.
fn preparation_payload(a: &Value, route: rp::W1Route) -> Value {
    let members = a["preparation"]["members"].as_array().map(|ms| ms.iter().map(|m| json!({
        "member":m["member"],"old_source":m["old_source"],"old_facts":m["old_facts"],"section":m["result"]["section"]})).collect::<Vec<_>>());
    json!({"definition_id":a["definition_id"],"definition_sha256":route_wire(route).definition_sha256,"owner_ref":a["owner_ref"],"ordinary_attempt_ref":a["ordinary_attempt_ref"],
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

/// G-j scope shared by every one-case serialization: the actual invocation, one
/// requested case and no combination, and that case's single ordinary seed.
fn one_case<'a>(pc: &'a rp::ProductCapture, invocation: &source_receipt::CapturedInvocation)
    -> Result<(usize, String, &'a rp::OrdinarySeed), ReceiptFailure> {
    let assoc = |p| fail(ReceiptCheck::Association, p);
    let scope = |p| fail(ReceiptCheck::Scope, p);
    if pc.invocation_mode != Some(invocation.mode()) {
        return Err(assoc("invocation"));
    }
    // RV82-S2: the supplied invocation must be the one this capture observed
    // (its digest), not merely one with the same mode and case id.
    if pc.invocation_digest.as_deref() != Some(invocation.borrowed_digest().as_str()) {
        return Err(assoc("invocation"));
    }
    let raw = invocation.borrowed_raw();
    let request_cases = raw["model"]["load_cases"].as_array().ok_or(assoc("cases"))?;
    let combinations = raw["model"]["combinations"].as_array().map_or(0, Vec::len);
    if request_cases.len() != 1 || combinations != 0 {
        return Err(scope("cases"));
    }
    let case_index = 0usize;
    let case_id = request_cases[case_index]["id"].as_str().ok_or(assoc("cases[].basis_ref"))?.to_owned();
    let [seed] = &pc.ordinary[..] else { return Err(scope("ordinary_attempts")) };
    if seed.case != case_id {
        return Err(assoc("ordinary_attempts[].case_id"));
    }
    Ok((case_index, case_id, seed))
}
/// One recorded native call with one run of this case (G-j).
fn one_run<'a>(inv: &'a k::RecordedInvocation, case: &k::RecordedCase, case_index: usize) -> Result<&'a k::RunOrigins, ReceiptFailure> {
    if inv.runs().len() != 1 || inv.calls().len() != 1 || inv.sources().len() != 1 || case.run != 0 {
        return Err(fail(ReceiptCheck::Scope, "cases[].run"));
    }
    let run = &inv.runs()[case.run];
    if run.id != case.run || !matches!(run.owner, k::NativeOwner::Case(i) if i == case_index) {
        return Err(fail(ReceiptCheck::Association, "cases[].run.origin"));
    }
    Ok(run)
}
/// The native records of any kernel outcome and its C1 §4 terminal.
fn kernel_outcome<'a>(e: &Enc, outcome: &'a k::ExecutionOutcome) -> (&'a [k::AttemptRecord], Value) {
    match outcome {
        k::ExecutionOutcome::Selected(owner) => (&owner.evidence().attempts, json!({"kind":"selected","reason":null})),
        k::ExecutionOutcome::Refused { refusal: r, attempts, .. } => (attempts, json!({"kind":"refused","reason":refusal(e,r)})),
        k::ExecutionOutcome::Unresolved { reason, attempts, .. } => (attempts, json!({"kind":"unresolved","reason":unresolved(e,reason)})),
    }
}
/// The Run (C1 §1, §4; C2 §4) for selected and unavailable kernel outcomes alike.
/// `case_index` is the Run's owner as a request index (B1 SP, PLAN_v2 N-2: the kernel's
/// batch ordinal mapped to the request).
fn run_value(e: &Enc, run: &k::RunOrigins, records: &[k::AttemptRecord], terminal: Value, case_index: usize) -> Result<Value, ReceiptFailure> {
    run_value_owned(e, run, records, terminal, json!({"kind":"case","index":case_index}))
}
/// `run_value` with the Run's owner reference given (B2-P: `{kind: "combination", index}` for a
/// combination's Run, by its authored index).
fn run_value_owned(e: &Enc, run: &k::RunOrigins, records: &[k::AttemptRecord], terminal: Value, owner_ref: Value) -> Result<Value, ReceiptFailure> {
    // Only the first `physical_records` links describe actual records.
    if records.len() != run.physical_records || records.len() > run.records.len() {
        return Err(fail(ReceiptCheck::Association, "cases[].run.records"));
    }
    let physical_v: Vec<Value> = records.iter().enumerate().map(|(i, r)| physical(e, i, r, run.records[i])).collect();
    let attempts_v = logical(e, records);
    run_conservation(e, &attempts_v, run);
    Ok(json!({"id":run.id,"origin":{"call":run.call,"position":run.position,"group":run.group,"source_ref":run.source,"owner_ref":owner_ref},
        "cache_before":slots(&run.cache_before),"cache_after":slots(&run.cache_after),"kernel_terminal":terminal,
        "records":physical_v,"attempts":attempts_v,"case_charge":e.exact(run.work.case(),"cases[].run.case_charge"),
        "invocation_before":e.exact(run.work.invocation_before(),"cases[].run.invocation_before"),
        "invocation_increment":e.exact(run.work.invocation_increment(),"cases[].run.invocation_increment"),
        "invocation_after":e.exact(run.work.invocation_after(),"cases[].run.invocation_after")}))
}
/// The invocation's calls, groups and builds (C2 §4), and its exact final charge, which
/// `run`, the last Run, ends at. The kernel names a case owner by its batch ordinal;
/// `requests[ordinal]` is its request index (B1 SP, PLAN_v2 N-2), so `calls[].owner_refs` name
/// request indices.
fn invocation_arrays(e: &Enc, inv: &k::RecordedInvocation, run: &k::RunOrigins, requests: &[usize])
    -> Result<(Vec<Value>, Vec<Value>, Vec<Value>, u64), ReceiptFailure> {
    invocation_arrays_mapped(e, inv, run, &OwnerMap { requests, prepared: &[], combinations: &[] })
}
/// B2-P (B2-C §2.5's ordinal mapping): the kernel's native owners as authored indices. A
/// `Case(j)` below the batch's length is batch ordinal j (`requests[j]`); at or above it, the
/// (j − n)-th registered operand preparation's owner (`prepared`); `Combination(k)` is the
/// k-th mechanics Call, refused Calls included (`combinations[k]`, authored indices).
struct OwnerMap<'a> { requests: &'a [usize], prepared: &'a [usize], combinations: &'a [usize] }
impl OwnerMap<'_> {
    fn case(&self, ordinal: usize) -> Option<usize> {
        self.requests.get(ordinal).copied().or_else(|| ordinal.checked_sub(self.requests.len()).and_then(|j| self.prepared.get(j).copied()))
    }
    fn owner(&self, owner: &k::NativeOwner) -> Option<Value> {
        match owner {
            k::NativeOwner::Case(i) => self.case(*i).map(|request| json!({"kind":"case","index":request})),
            k::NativeOwner::Combination(i) => self.combinations.get(*i).map(|index| json!({"kind":"combination","index":index})),
        }
    }
}
/// The CombinationReason of a `pre_source_refusal` (SCHEMA `CombinationReason`; C2 §4; RV115
/// N-4). The Run-terminal tags are not pre-source reasons (B2-C §2.5): untranslated.
fn combination_reason(e: &Enc, r: &k::CombinationReason) -> Value {
    match r {
        k::CombinationReason::NoOperands => json!({"space":"combination","tag":"no_operands"}),
        k::CombinationReason::NestedCombination => json!({"space":"combination","tag":"nested_combination"}),
        k::CombinationReason::OperandsDiffer => json!({"space":"combination","tag":"operands_differ"}),
        k::CombinationReason::NoSelectedOperand => json!({"space":"combination","tag":"no_selected_operand"}),
        k::CombinationReason::LedgerUnavailable(k::LedgerRefusal::Accumulator(s)) => {
            json!({"space":"combination","tag":"ledger_unavailable","error":{"tag":"accumulator","error":sum_error(s)}})
        }
        k::CombinationReason::CountRange(name) => json!({"space":"combination","tag":"count_range","name":name}),
        _ => e.untranslated("calls[].result.reason"),
    }
}
fn invocation_arrays_mapped(e: &Enc, inv: &k::RecordedInvocation, run: &k::RunOrigins, owners: &OwnerMap<'_>)
    -> Result<(Vec<Value>, Vec<Value>, Vec<Value>, u64), ReceiptFailure> {
    let owner = |o: &k::NativeOwner| owners.owner(o).unwrap_or_else(|| {
        e.fail(ReceiptCheck::Association, "calls[].owner_refs");
        Value::Null
    });
    let calls: Vec<Value> = inv.calls().iter().map(|c| match c.kind {
        k::CallKind::CaseBatch => json!({"id":c.id,"kind":"case_batch",
            "owner_refs":c.owners.iter().map(owner).collect::<Vec<_>>(),
            "source_refs":c.sources,"run_refs":c.runs,"invocation_before":e.exact(c.invocation_before,"calls[].invocation_before"),
            "invocation_after":e.exact(c.invocation_after,"calls[].invocation_after"),"result":{"kind":"runs"}}),
        // B2-P (B2-C §2.5; C2 §4): a MechanicsCombinationCall, its authored terms as requested.
        k::CallKind::MechanicsCombination => {
            let result = match &c.result {
                k::CallResult::Runs => json!({"kind":"runs"}),
                k::CallResult::PreSourceRefusal { stage, reason } => json!({"kind":"pre_source_refusal",
                    "stage":match stage { k::CombinationStage::OperandValidation=>"operand_validation", k::CombinationStage::CombinedPreparation=>"combined_preparation" },
                    "reason":combination_reason(e,reason)}),
                // N-5: a combination's origin refusal abandons; it is never serialized.
                k::CallResult::OriginRefusal(_) => { e.fail(ReceiptCheck::Association, "calls[].result"); Value::Null }
            };
            let requested = c.requested_operands.iter().map(|o| json!({"source_ref":o.source.map_or_else(|| {
                e.fail(ReceiptCheck::Association, "calls[].requested_operands[].source_ref");
                Value::Null
            }, |s| json!(s)),"factor":e.ubits(o.factor_bits,"calls[].requested_operands[].factor")})).collect::<Vec<_>>();
            json!({"id":c.id,"kind":"mechanics_combination","owner_refs":c.owners.iter().map(owner).collect::<Vec<_>>(),
                "requested_operands":requested,"source_refs":c.sources,"run_refs":c.runs,
                "invocation_before":e.exact(c.invocation_before,"calls[].invocation_before"),
                "invocation_after":e.exact(c.invocation_after,"calls[].invocation_after"),"result":result})
        }
    }).collect();
    let groups: Vec<Value> = inv.groups().iter().map(|g| {
        let preparation = match &g.preparation { k::GroupPreparation::Ready => json!({"kind":"ready"}), k::GroupPreparation::Refused(r) => json!({"kind":"refused","reason":refusal(e,r)}) };
        let stiffness = inv.sources().get(g.first_source).map(|s| sha_hex(&s.stiffness));
        if stiffness.is_none() { e.fail(ReceiptCheck::Association, "groups[].stiffness_sha256"); }
        let mut group = json!({"id":g.id,"call":g.call,"first_source_ref":g.first_source,
            "source_refs":inv.runs().iter().filter(|r|r.group==Some(g.id)).map(|r|r.source).collect::<Vec<_>>(),
            "stiffness_sha256":stiffness,"preparation":preparation});
        // B2-P (C2 §4): a combination's call-local group carries its imports, in slot order.
        if inv.calls().get(g.call).is_some_and(|c| matches!(c.kind, k::CallKind::MechanicsCombination)) {
            group["imports"] = json!(g.imports.iter().flatten().map(|i| json!({"operand_index":i.operand_index,"selected_run":i.selected_run,
                "slot":slot_name(i.slot),"build":i.build})).collect::<Vec<_>>());
        }
        group
    }).collect();
    let builds: Vec<Value> = inv.builds().iter().map(|b| json!({"id":b.id,"group":b.group,"slot":slot_name(b.slot),
        "origin":{"call":b.call,"run":b.run,"physical_record":b.physical_record,"phase":match b.phase { k::BuildPhase::Shared=>"shared", k::BuildPhase::VerificationShared=>"verification_shared" }},
        "state":match b.state { k::BuildState::Success=>"success", k::BuildState::NonbudgetFailure=>"nonbudget_failure", k::BuildState::BudgetFailure=>"budget_failure" },
        "reason":b.reason.as_ref().map(|r|stop(e,r)),"work":e.exact(b.work,"builds[].work"),"stages":stages(e,&b.stages,"builds[].stages")})).collect();
    // W1-LME-20B-60B-v1: the meter's own limit and its exact final charge.
    if inv.meter().limit() != INVOCATION_LIMIT {
        return Err(fail(ReceiptCheck::Association, "work.invocation_limit"));
    }
    // C1 §4 body.work: charged equals the final after (before = 0): the last Run's, and
    // (DESIGN_v2 T-11) the call's `invocation_after`. B2-P (B2-C §2.7): with combination
    // Calls, the last Call's `invocation_after`, also the last Run's when that Call made one,
    // and each Call's `invocation_before` the previous Call's `invocation_after` (the meter chain).
    let charged = e.exact(inv.meter().checked_charged(), "work.charged");
    let last = inv.calls().last();
    if last.is_none_or(|last| !last.runs.is_empty()) && charged != e.exact(run.work.invocation_after(), "work.charged") {
        return Err(fail(ReceiptCheck::WorkCounterInconsistent, "work.charged"));
    }
    if last.is_some_and(|last| charged != e.exact(last.invocation_after, "work.charged"))
        || inv.calls().windows(2).any(|pair| e.exact(pair[1].invocation_before, "work.charged") != e.exact(pair[0].invocation_after, "work.charged")) {
        return Err(fail(ReceiptCheck::WorkCounterInconsistent, "work.charged"));
    }
    Ok((calls, groups, builds, charged))
}
/// The ordinary attempt entry (G-b, G-c, G-l) and the case's quality index.
fn ordinary_value(e: &Enc, env: &Value, case_id: &str, case_index: usize, mode: PreviewSolverMode, seed: &rp::OrdinarySeed, legacy: Value)
    -> Result<(Value, usize), ReceiptFailure> {
    // D6a (decision 2): the diagnostics naming the case, once each, in envelope
    // order, excluding RETAINED_PRECISION_* (a T1 (a) omission is already applied).
    let [diagnostic_refs] = case_diagnostic_refs(env, &[case_id])?;
    ordinary_entry(e, env, case_id, case_index, mode, seed, legacy, diagnostic_refs)
}
/// One requested case's ordinary attempt entry, with its D6a references already collected.
#[allow(clippy::too_many_arguments)]
fn ordinary_entry(e: &Enc, env: &Value, case_id: &str, case_index: usize, mode: PreviewSolverMode, seed: &rp::OrdinarySeed, legacy: Value,
    diagnostic_refs: Vec<Value>) -> Result<(Value, usize), ReceiptFailure> {
    let assoc = |p| fail(ReceiptCheck::Association, p);
    let quality_index = env["numerical_quality"]["cases"].as_array().and_then(|q| {
        let found: Vec<usize> = q.iter().enumerate().filter(|(_, x)| x["basis_ref"]["ref_id"] == json!(case_id)).map(|(i, _)| i).collect();
        if let &[i] = found.as_slice() { Some(i) } else { None }
    }).ok_or(assoc("cases[].ordinary.quality_binding"))?;
    let (initial, w2, formation) = ordinary_members(e, seed);
    // C2 §5: the report's outcome is the case's unchanged assessed quality.
    if initial["kind"] == "report" && env["numerical_quality"]["cases"][quality_index]["solve_quality"] != initial["outcome"] {
        return Err(assoc("ordinary_attempts[].initial.outcome"));
    }
    Ok((json!({"case_index":case_index,"case_id":case_id,"material_basis_ref":0,"requested_mode":mode.as_str(),
        "initial":initial,"w2":w2,"formation":formation,"legacy_source":legacy,"diagnostic_refs":diagnostic_refs}), quality_index))
}
/// D6a for each of `case_ids` in one pass over the diagnostics (B1 SP; I82's c² guidance):
/// per case, the ids of the diagnostics naming it, once each, in envelope order, excluding
/// RETAINED_PRECISION_*.
fn case_diagnostic_refs<const N: usize>(env: &Value, case_ids: &[&str; N]) -> Result<[Vec<Value>; N], ReceiptFailure> {
    let mut refs: [Vec<Value>; N] = std::array::from_fn(|_| Vec::new());
    case_diagnostic_refs_into(env, case_ids, &mut refs)?;
    Ok(refs)
}
fn case_diagnostic_refs_into(env: &Value, case_ids: &[&str], refs: &mut [Vec<Value>]) -> Result<(), ReceiptFailure> {
    for d in env["diagnostics"].as_array().ok_or(fail(ReceiptCheck::Association, "diagnostics"))? {
        if d["code"].as_str().is_some_and(|c| c.starts_with("RETAINED_PRECISION_")) {
            continue;
        }
        let Some(affected) = d["affected_refs"].as_array() else { continue };
        for (case_id, refs) in case_ids.iter().zip(refs.iter_mut()) {
            if affected.iter().any(|r| r == *case_id) {
                refs.push(d["id"].clone());
            }
        }
    }
    Ok(())
}
/// The body around its members, then the two hashes (C1 §3): publication over the
/// final envelope without the receipt, then the body.
#[allow(clippy::too_many_arguments)]
fn finish(e: Enc, env: Value, invocation: &source_receipt::CapturedInvocation, charged: u64, execution_order: Value, cases: Vec<Value>,
    sources: Vec<Value>, material: Value, arrays: (Vec<Value>, Vec<Value>, Vec<Value>), ordinary: Vec<Value>, attempts: Vec<Value>,
    legacy_source_work: Vec<Value>) -> Result<Value, ReceiptFailure> {
    finish_with(e, env, invocation, charged, execution_order, cases, sources, material, arrays, ordinary, attempts, legacy_source_work, Vec::new(), Vec::new())
}
/// `finish` with B2-P's members: `combinations[]` (one per model combination) and
/// `operand_preparations[]`, present only when non-empty (C-6).
#[allow(clippy::too_many_arguments)]
fn finish_with(e: Enc, mut env: Value, invocation: &source_receipt::CapturedInvocation, charged: u64, execution_order: Value, cases: Vec<Value>,
    sources: Vec<Value>, material: Value, (calls, groups, builds): (Vec<Value>, Vec<Value>, Vec<Value>), ordinary: Vec<Value>, attempts: Vec<Value>,
    legacy_source_work: Vec<Value>, combinations: Vec<Value>, operand_preparations: Vec<Value>) -> Result<Value, ReceiptFailure> {
    let mut body = json!({"receipt_version":1,"policy":POLICY,"projection_policy":PROJECTION_POLICY,"work_policy":WORK_POLICY,
        "facade_policy":FACADE_POLICY,"canonicalization":CANONICALIZATION,
        "invocation":{"algorithm":"sha256","profile":CANONICALIZATION,"scope":"actual_request_and_solver_mode","domain":"source_blocks_invocation_v1","value":invocation.borrowed_digest()},
        "publication_sha256":"",
        "work":{"case_limit":CASE_LIMIT,"invocation_limit":INVOCATION_LIMIT,"charged":charged,"execution_order":execution_order},
        "cases":cases,"combinations":combinations,"sources":sources,"material_bases":[material],"calls":calls,"groups":groups,"builds":builds,
        "ordinary_attempts":ordinary,"product_attempts":attempts,"legacy_source_work":legacy_source_work});
    if !operand_preparations.is_empty() {
        body["operand_preparations"] = json!(operand_preparations);
    }
    e.finish()?;
    if !safe_integers(&body) {
        return Err(fail(ReceiptCheck::Encoding, "body"));
    }
    let publication = domain_hash("retained_precision_publication_mp_v2", &env).ok_or(fail(ReceiptCheck::PublicationHashRange, "publication_sha256"))?;
    body["publication_sha256"] = json!(publication);
    let receipt = domain_hash("retained_precision_receipt_mp_v2", &body).ok_or(fail(ReceiptCheck::Encoding, "receipt_sha256"))?;
    env.as_object_mut().ok_or(fail(ReceiptCheck::Association, "envelope"))?.insert("retained_precision".into(), json!({"body":body,"receipt_sha256":receipt}));
    Ok(env)
}
/// The source's preparation reference (C3 §2): the product attempt that prepared it, by its
/// index in `product_attempts[]` (B1 SP, DESIGN_v2 T-7: the attempt's own index; 0 at c = 1).
fn bind_preparation(source: &mut Value, attempt: &Value, attempt_ref: usize, route: rp::W1Route) -> Result<(), ReceiptFailure> {
    let preparation = domain_hash("retained_precision_preparation_v1", &preparation_payload(attempt, route)).ok_or(fail(ReceiptCheck::Encoding, "sources[].preparation.sha256"))?;
    source["preparation"] = json!({"attempt_ref":attempt_ref,"sha256":preparation});
    Ok(())
}

/// Serialize one selected, certified, one-case prepared candidate (grant 1).
/// Returns the successor envelope carrying the `retained_precision` member (PP's
/// `serde_json::Map` is key-sorted, so it sits between `results` and `status`;
/// every hash is canonical, so the position is immaterial).
pub(super) fn serialize_selected(candidate: &rp::PrivatePreparedCandidate, invocation: &source_receipt::CapturedInvocation)
    -> Result<Value, ReceiptFailure> {
    serialize_selected_from(candidate, candidate.envelope(), invocation)
}

/// What the selected serialization reads from a certified candidate.
pub(super) trait SelectedCandidate {
    fn capture(&self) -> &rp::ProductCapture;
    fn certificate(&self) -> &k::CertifiedProductProof;
    fn typed_trace<'a>(&'a self, costs: &mut rr::ProjectionWork) -> Result<rr::PreparedAttemptView<'a>, rr::TraceProjectionError>;
}
impl SelectedCandidate for rp::PrivatePreparedCandidate {
    fn capture(&self) -> &rp::ProductCapture { rp::PrivatePreparedCandidate::capture(self) }
    fn certificate(&self) -> &k::CertifiedProductProof { rp::PrivatePreparedCandidate::certificate(self) }
    fn typed_trace<'a>(&'a self, costs: &mut rr::ProjectionWork) -> Result<rr::PreparedAttemptView<'a>, rr::TraceProjectionError> {
        rp::PrivatePreparedCandidate::typed_trace(self, costs)
    }
}

fn serialize_selected_from(candidate: &impl SelectedCandidate, overlaid: &MechanicsEnvelope, invocation: &source_receipt::CapturedInvocation)
    -> Result<Value, ReceiptFailure> {
    let assoc = |p| fail(ReceiptCheck::Association, p);
    let scope = |p| fail(ReceiptCheck::Scope, p);
    let e = Enc::default();
    let mode = invocation.mode();
    let raw = invocation.borrowed_raw();
    let pc = candidate.capture();
    let (case_index, case_id, seed) = one_case(pc, invocation)?;
    let (inv, case) = pc.native_pair().ok_or(assoc("cases[].run"))?;
    let k::ExecutionOutcome::Selected(owner) = &case.outcome else { return Err(scope("cases[].status")) };
    // U2 (RV77-N4): the proof must have started on this selected owner's own solve.
    let certificate = candidate.certificate();
    if !certificate.owner_matches(owner) {
        return Err(assoc("cases[].selection.owner"));
    }
    if !certificate.passed() {
        return Err(assoc("cases[].selection.certificate"));
    }
    let run = one_run(inv, case, case_index)?;
    let mut costs = rr::ProjectionWork::default();
    let view = candidate.typed_trace(&mut costs).map_err(|_| assoc("product_attempts[]"))?;

    // The successor envelope (G-a; T1 (a)).
    let mut env = serde_json::to_value(overlaid).map_err(|_| fail(ReceiptCheck::Encoding, "envelope"))?;
    let mut legacy_source_work = Vec::new();
    let (legacy, omit) = legacy_source(&e, seed.legacy.as_ref(), case_index, &mut legacy_source_work, true)?;
    successor_envelope(&mut env, pc.route(), &case_id, omit.as_deref())?;

    // Row bindings: the producer's own QuantityId/recipe binding in envelope row order.
    let rows = pc.bind_rows(overlaid, owner).map_err(|_| assoc("cases[].selection.absolute_verified"))?;
    let row_ids: Vec<String> = env["results"].as_array().ok_or(assoc("results"))?.iter()
        .map(|r| r["id"].as_str().map(str::to_owned)).collect::<Option<_>>().ok_or(assoc("results[].id"))?;
    if rows.len() != row_ids.len() || rows.iter().zip(&row_ids).any(|(r, id)| r.id != id.as_str()) {
        return Err(assoc("cases[].selection.absolute_verified"));
    }
    let recipes: Vec<k::ProductRecipe> = rows.iter().map(|r| r.recipe).collect();

    // The native run (C1 §1).
    let (records, terminal) = kernel_outcome(&e, &case.outcome);
    let run_v = run_value(&e, run, records, terminal, case_index)?;
    let mut source = case_source(&e, pc, owner.source(), inv, run, case_index, &case_id, &view)?;
    let attempt = product_attempt(&e, &view, 0, case_index, Some(run.source), Some(case.run), json!({"kind":"ready"}), pc);
    bind_preparation(&mut source, &attempt, 0, pc.route())?;
    let source_identity = {
        let mut binding = source.clone();
        binding.as_object_mut().ok_or(assoc("sources[]"))?.remove("index");
        domain_hash("retained_precision_source_mp_v2", &binding).ok_or(fail(ReceiptCheck::Encoding, "cases[].source_identity_sha256"))?
    };
    let selection_v = selection(&e, owner, pc, &view, certificate.verdicts(), &recipes, &row_ids)?;
    let (ordinary, quality_index) = ordinary_value(&e, &env, &case_id, case_index, mode, seed, legacy)?;
    let case_v = json!({"basis_ref":{"ref_type":"load_case","ref_id":case_id},"ordinary":{"attempt_ref":case_index,"quality_binding":{"kind":"present","index":quality_index}},
        "product_attempt_ref":0,"status":"selected","method":METHOD,"run":run_v,"source_ref":run.source,"source_identity_sha256":source_identity,"selection":selection_v});
    let (calls, groups, builds, charged) = invocation_arrays(&e, inv, run, &[case_index])?;
    let material = material_basis(&e, pc, raw, 1)?;
    finish(e, env, invocation, charged, json!([{"kind":"case","index":case_index}]), vec![case_v], vec![source], material,
        (calls, groups, builds), vec![ordinary], vec![attempt], legacy_source_work)
}

/// A refused one-case prepared attempt: the three actual refusal owners.
pub(super) enum Refused<'a> {
    /// Preparation refused before any native call (`prepare_case`).
    Preparation(&'a rp::PreparedCaseFailure),
    /// `solve_native` returned an error: with a recorded Run (non-selected kernel
    /// outcome), or before any kernel schedule (D38).
    Native(&'a rp::PreparedCase),
    /// The candidate was refused after its native stage (proof, facade or capture).
    Candidate(&'a rp::PreparedCandidateRefusal),
}

/// The fixed product text of the unavailable-case diagnostic (G-a; D1 §4.1.4: one
/// info diagnostic per case).
pub(super) const UNAVAILABLE_CODE: &str = "RETAINED_PRECISION_UNAVAILABLE";
pub(super) const UNAVAILABLE_MESSAGE: &str = "Retained-precision recovery is unavailable for this load case. Its published rows keep their ordinary values, standing and diagnostics; the retained_precision receipt records the actual attempt and its typed cause.";

/// G-a for an unavailable case: identity and profile, no method token, the
/// legacy disclosure kept, and the unavailable diagnostic.
fn unavailable_envelope(env: &mut Value, route: rp::W1Route, case_id: &str) -> Result<String, ReceiptFailure> {
    successor_identity(env, route);
    unavailable_case_envelope(env, case_id)
}
/// One unavailable case's part of G-a: its unavailable-case diagnostic (appended); its
/// legacy disclosure is kept.
fn unavailable_case_envelope(env: &mut Value, case_id: &str) -> Result<String, ReceiptFailure> {
    let assoc = |p| fail(ReceiptCheck::Association, p);
    let diags = env["diagnostics"].as_array_mut().ok_or(assoc("diagnostics"))?;
    let id = format!("diagnostic:retained-precision:{case_id}:unavailable");
    if diags.iter().any(|d| d["id"] == json!(id)) {
        return Err(assoc("diagnostics[+unavailable].id"));
    }
    diags.push(json!({"id":id,"code":UNAVAILABLE_CODE,"severity":"info","message":UNAVAILABLE_MESSAGE,
        "source":"core/product_physics","affected_refs":[case_id]}));
    Ok(id)
}

/// The C1-C3 unavailable representation of a refused one-case prepared attempt:
/// the unavailable case (reason by the readers' accepted D4d table), its product
/// attempt with the typed PublicError, the Run when the kernel recorded one, and
/// D38's `run:null`/`run_ref:null`/`capture` cause when it failed first.
///
/// **Not a publication.** Under T3 (ROOT) a one-case invocation whose case is
/// unavailable has no successor publication: the caller publishes the ordinary
/// envelope. The value exists for wider F2a and the reader round, and every
/// accepted reader refuses it at G3 (no selected case).
pub(super) fn serialize_unavailable(refused: Refused<'_>, invocation: &source_receipt::CapturedInvocation) -> Result<Value, ReceiptFailure> {
    let assoc = |p| fail(ReceiptCheck::Association, p);
    let e = Enc::default();
    let mode = invocation.mode();
    let raw = invocation.borrowed_raw();
    let (pc, ordinary_env) = match &refused {
        Refused::Preparation(f) => (&f.capture, &f.ordinary),
        Refused::Native(p) => (p.capture(), p.owned_ordinary().ok_or(assoc("envelope"))?),
        Refused::Candidate(r) => (r.capture(), &r.ordinary),
    };
    let (case_index, case_id, seed) = one_case(pc, invocation)?;
    let selected_owner = pc.native_pair().and_then(|(_, case)| match &case.outcome { k::ExecutionOutcome::Selected(o) => Some(&**o), _ => None });
    // U2 on the failure path: refused proof work and any certificate bind to the
    // selected owner structurally (before any projection reads them).
    if let Refused::Candidate(r) = &refused {
        let bound = |matches: &dyn Fn(&k::RetainedSolve) -> bool| selected_owner.is_some_and(matches);
        if r.proof_failure().is_some_and(|f| !bound(&|o| f.owner_matches(o))) {
            return Err(assoc("product_attempts[].proof.owner"));
        }
        if r.certificate().is_some_and(|c| !bound(&|o| c.owner_matches(o))) {
            return Err(assoc("product_attempts[].proof.owner"));
        }
    }
    let mut costs = rr::ProjectionWork::default();
    let view = match &refused {
        Refused::Preparation(f) => f.typed_trace(&mut costs),
        Refused::Native(p) => p.native_refusal_trace(&mut costs),
        Refused::Candidate(r) => r.typed_trace(&mut costs),
    }.map_err(|_| assoc("product_attempts[]"))?;
    let rr::ResultRef::Unavailable(failure) = &view.result else { return Err(assoc("product_attempts[].result")) };
    let mut env = serde_json::to_value(ordinary_env).map_err(|_| fail(ReceiptCheck::Encoding, "envelope"))?;
    let mut legacy_source_work = Vec::new();
    let (legacy, _) = legacy_source(&e, seed.legacy.as_ref(), case_index, &mut legacy_source_work, false)?;
    let diagnostic_ref = unavailable_envelope(&mut env, pc.route(), &case_id)?;
    let (ordinary, quality_index) = ordinary_value(&e, &env, &case_id, case_index, mode, seed, legacy)?;
    let error = public_error(&e, failure, pc);
    let material = material_basis(&e, pc, raw, 1)?;
    let (case_v, attempt, sources, arrays, charged, order) = match pc.native_pair() {
        // D38: no kernel schedule ran, so no Run, run_ref or execution entry.
        None => {
            // A source-constructor refusal carries C2's `source_decline` (constructor
            // counts are not captured typed here; fail closed).
            if matches!(failure, rr::FailureRef::Preparation { capture: rp::CaptureError::Source(_), .. }) {
                return Err(fail(ReceiptCheck::Untranslated, "cases[].source_decline"));
            }
            if view.prepared_source.is_some() {
                // A prepared source with no native call: its CaseSource would need identity
                // bytes the kernel registers only at the call (not decided here; fail closed).
                return Err(fail(ReceiptCheck::Untranslated, "sources[].kernel_source_sha256"));
            }
            let code = ("source_unavailable", "preparation");
            let attempt = product_attempt(&e, &view, 0, case_index, None, None, json!({"kind":"unavailable","error":error}), pc);
            let case_v = json!({"basis_ref":{"ref_type":"load_case","ref_id":case_id},"ordinary":{"attempt_ref":case_index,"quality_binding":{"kind":"present","index":quality_index}},
                "product_attempt_ref":0,"status":"unavailable","reason":{"code":code.0,"phase":code.1,"cause":{"kind":"prepared_product_failure","product_attempt_ref":0}},
                "diagnostic_ref":diagnostic_ref,"run":null,"source_ref":null});
            (case_v, attempt, Vec::new(), (Vec::new(), Vec::new(), Vec::new()), 0u64, json!([]))
        }
        Some((inv, case)) => {
            let run = one_run(inv, case, case_index)?;
            let (records, terminal) = kernel_outcome(&e, &case.outcome);
            let kind = terminal["kind"].as_str().unwrap_or_default().to_owned();
            let run_v = run_value(&e, run, records, terminal, case_index)?;
            let source = match selected_owner { Some(owner) => owner.source(), None => view.prepared_source.ok_or(assoc("sources[]"))? };
            let mut source_v = case_source(&e, pc, source, inv, run, case_index, &case_id, &view)?;
            // The readers' accepted D4d table (S06): a nonselected Run is the kernel's;
            // after a selected Run, the facade's.
            let code = match (error["kind"].as_str(), kind.as_str()) {
                (Some("preparation"), _) => return Err(assoc("product_attempts[].result")),
                (_, "selected") => ("facade_certificate".to_owned(), "facade"),
                (_, other) => (format!("kernel_{other}"), "kernel"),
            };
            let attempt = product_attempt(&e, &view, 0, case_index, Some(run.source), Some(case.run), json!({"kind":"unavailable","error":error}), pc);
            if view.members.iter().all(|m| matches!(m.result, rr::PreparationResult::Prepared(_))) {
                bind_preparation(&mut source_v, &attempt, 0, pc.route())?;
            }
            let case_v = json!({"basis_ref":{"ref_type":"load_case","ref_id":case_id},"ordinary":{"attempt_ref":case_index,"quality_binding":{"kind":"present","index":quality_index}},
                "product_attempt_ref":0,"status":"unavailable","reason":{"code":code.0,"phase":code.1,"cause":{"kind":"prepared_product_failure","product_attempt_ref":0}},
                "diagnostic_ref":diagnostic_ref,"run":run_v,"source_ref":run.source});
            let (calls, groups, builds, charged) = invocation_arrays(&e, inv, run, &[case_index])?;
            (case_v, attempt, vec![source_v], (calls, groups, builds), charged, json!([{"kind":"case","index":case_index}]))
        }
    };
    finish(e, env, invocation, charged, order, vec![case_v], sources, material, arrays, vec![ordinary], vec![attempt], legacy_source_work)
}

/// B1 SP (DESIGN_v2 T-11): the successor of an invocation whose transaction has at least one
/// selected (frozen) case. `staged` is the staging copy: the ordinary owner with the frozen
/// cases' overlays. The receipt covers every requested case:
/// - `cases[]`, in request order: `selected` (frozen), `unavailable` (in A and not frozen) or
///   `not_required` (not in A, its published verdict `checks_passed`);
/// - `ordinary_attempts[]`, one per requested case; `product_attempts[]`, in start order;
///   `sources[]`, in the call's registration order; the one material basis (D1.5);
/// - the one call, its groups and builds; `work.execution_order`, every Run in actual order;
///   `work.charged`, the call's `invocation_after`;
/// - every Run and call owner named by its request index: the kernel's batch ordinal `k` is the
///   `k`-th prepared attempt (PLAN_v2 N-2);
/// - each attempt's adapter snapshot as its trace recorded it, at its terminal stage.
///
/// The envelope (G-a; T1 (a)): the identity once; each selected case's method token, omitted
/// legacy disclosure and selected diagnostic, in request order; then each unavailable case's
/// diagnostic, in request order. At c = 1 these are `serialize_selected_from`'s bytes. Any
/// refusal abandons the successor (decision 5).
pub(super) fn serialize_cases(cases: &mut rp::PreparedCases, staged: &MechanicsEnvelope, invocation: &source_receipt::CapturedInvocation)
    -> Result<Value, ReceiptFailure> {
    let rp::PreparedCases { capture, attempts, combinations, operand_preparations, .. } = cases;
    let native = capture.native_invocation.take();
    let serialized = serialize_cases_with(capture, attempts, combinations, operand_preparations, native.as_ref(), staged, invocation);
    capture.native_invocation = native;
    serialized
}

/// One requested case's status in the successor.
#[derive(Clone, Copy, PartialEq, Eq)]
enum CaseStatus { Selected, Unavailable, NotRequired }

/// B2-P (B2-C §2.7): the combination diagnostics' messages.
pub(super) const COMBINATION_SELECTED_MESSAGE: &str = "Retained-precision recovery (contribution_preserving_multiprecision_v1) is selected for this load combination, from its own solve and certificate. Its published rows carry recovery_method; the retained_precision receipt binds their certified classes, the native attempts and work, and its operands' sources.";
pub(super) const COMBINATION_UNAVAILABLE_MESSAGE: &str = "Retained-precision recovery is unavailable for this load combination. Its published rows keep their ordinary values and diagnostics and are withheld from rule binding; the retained_precision receipt records the actual attempt and its typed cause.";
/// B2-P (DEF-C; B2-C §4): the combination formation definition's id.
pub(super) const COMBINATION_DEFINITION_ID: &str = "RP-PREPARED-COMBINATION-DUAL-v1";

#[allow(clippy::too_many_arguments)]
fn serialize_cases_with(pc: &mut rp::ProductCapture, attempts: &[rp::CaseAttempt], combos: &mut [rp::CombinationAttempt],
    preparations: &[rp::OperandPreparation], inv: Option<&k::RecordedInvocation>,
    staged: &MechanicsEnvelope, invocation: &source_receipt::CapturedInvocation) -> Result<Value, ReceiptFailure> {
    let assoc = |p| fail(ReceiptCheck::Association, p);
    let scope = |p| fail(ReceiptCheck::Scope, p);
    let e = Enc::default();
    let mode = invocation.mode();
    let raw = invocation.borrowed_raw();
    // G-j: the invocation this capture observed (RV82-S2), its requested cases, one ordinary
    // seed per case, in request order, and (B2-P) one record per model combination.
    if pc.invocation_mode != Some(mode) || pc.invocation_digest.as_deref() != Some(invocation.borrowed_digest().as_str()) {
        return Err(assoc("invocation"));
    }
    let request_cases = raw["model"]["load_cases"].as_array().ok_or(assoc("cases"))?;
    let model_combinations = raw["model"]["combinations"].as_array().map_or(&[][..], Vec::as_slice);
    let count = request_cases.len();
    if count == 0 || count != pc.cases_seen() {
        return Err(scope("cases"));
    }
    if model_combinations.len() != combos.len() || combos.len() != pc.combinations.len()
        || combos.iter().enumerate().any(|(k, combination)| combination.index != k) {
        return Err(scope("combinations"));
    }
    let case_ids = request_cases.iter().map(|case| case["id"].as_str()).collect::<Option<Vec<_>>>().ok_or(assoc("cases[].basis_ref"))?;
    let combination_ids = model_combinations.iter().map(|c| c["id"].as_str()).collect::<Option<Vec<_>>>().ok_or(assoc("combinations[].basis_ref"))?;
    if combination_ids.iter().zip(&pc.combinations).any(|(id, captured)| *id != captured.id) {
        return Err(assoc("combinations[].basis_ref"));
    }
    if pc.ordinary.len() != count {
        return Err(scope("ordinary_attempts"));
    }
    if pc.ordinary.iter().zip(&case_ids).any(|(seed, id)| seed.case != *id) {
        return Err(assoc("ordinary_attempts[].case_id"));
    }
    // Each requested case's attempt (A) and status; every attempt has ended.
    let mut attempt_of: Vec<Option<&rp::CaseAttempt>> = vec![None; count];
    for attempt in attempts {
        let slot = attempt_of.get_mut(attempt.request).ok_or(assoc("product_attempts[].owner_ref"))?;
        if slot.replace(attempt).is_some() {
            return Err(assoc("product_attempts[].owner_ref"));
        }
    }
    let status: Vec<CaseStatus> = attempt_of.iter().map(|attempt| match attempt.map(|a| &a.end) {
        Some(rp::AttemptEnd::Frozen(_)) => CaseStatus::Selected,
        Some(_) => CaseStatus::Unavailable,
        None => CaseStatus::NotRequired,
    }).collect();
    if !status.contains(&CaseStatus::Selected) {
        return Err(scope("cases[].status"));
    }
    // The call's batch ordinals: the prepared attempts, in request order.
    let requests: Vec<usize> = attempts.iter().filter(|attempt| attempt.prepared).map(|attempt| attempt.request).collect();
    // B2-P: the registered operand preparations' owners (FK's prepared case ordinals) and the
    // combinations that made a Call, in Call order (FK's combination ordinals).
    let prepared_owners: Vec<usize> = preparations.iter().filter(|record| record.source.is_some()).map(|record| record.owner).collect();
    let called: Vec<usize> = combos.iter().filter(|combination| combination.call.is_some()).map(|combination| combination.index).collect();
    let with_run = combos.iter().filter(|combination| combination.native().is_some()).count();
    let inv = inv.ok_or(assoc("calls"))?;
    if inv.calls().len() != 1 + called.len() || inv.runs().len() != requests.len() + with_run
        || inv.sources().len() != requests.len() + prepared_owners.len() + with_run {
        return Err(scope("cases[].run"));
    }
    let owners = OwnerMap { requests: &requests, prepared: &prepared_owners, combinations: &called };
    // D39: each case's legacy disposition, in request order (a selected case's disclosure is
    // omitted and its work recorded).
    let mut legacy_source_work = Vec::new();
    let mut legacy = Vec::with_capacity(count);
    for (index, seed) in pc.ordinary.iter().enumerate() {
        legacy.push(legacy_source(&e, seed.legacy.as_ref(), index, &mut legacy_source_work, status[index] == CaseStatus::Selected)?);
    }
    // The successor envelope (G-a; T1 (a)).
    let mut env = serde_json::to_value(staged).map_err(|_| fail(ReceiptCheck::Encoding, "envelope"))?;
    successor_identity(&mut env, pc.route());
    let mut diagnostic_ids: Vec<Option<String>> = vec![None; count];
    for (index, (_, omit)) in legacy.iter().enumerate() {
        if status[index] == CaseStatus::Selected {
            diagnostic_ids[index] = Some(selected_case_envelope(&mut env, case_ids[index], omit.as_deref())?);
        }
    }
    for index in 0..count {
        if status[index] == CaseStatus::Unavailable {
            diagnostic_ids[index] = Some(unavailable_case_envelope(&mut env, case_ids[index])?);
        }
    }
    // B2-P (B2-C §2.7): after the cases', each retained combination's diagnostic, in authored
    // order; a selected combination's rows carry `recovery_method`.
    let mut combination_diagnostics: Vec<Option<String>> = vec![None; combos.len()];
    for combination in combos.iter() {
        if !combination.retained() {
            continue;
        }
        let selected = matches!(combination.end, rp::CombinationEnd::Frozen(_));
        combination_diagnostics[combination.index] = Some(combination_envelope(&mut env, combination_ids[combination.index], selected)?);
    }
    // The ordinary attempts (G-b, G-c, G-l), with D6a's references in one pass.
    let mut diagnostic_refs = vec![Vec::new(); count];
    case_diagnostic_refs_into(&env, &case_ids, &mut diagnostic_refs)?;
    let mut ordinary = Vec::with_capacity(count);
    let mut quality = Vec::with_capacity(count);
    for (index, ((seed, (legacy_v, _)), refs)) in pc.ordinary.iter().zip(legacy).zip(diagnostic_refs).enumerate() {
        let (entry, quality_index) = ordinary_entry(&e, &env, case_ids[index], index, mode, seed, legacy_v, refs)?;
        // DESIGN_v2 T-4: a requested case outside A is `not_required` by its published verdict.
        if status[index] == CaseStatus::NotRequired && env["numerical_quality"]["cases"][quality_index]["solve_quality"] != "checks_passed" {
            return Err(assoc("cases[].status"));
        }
        ordinary.push(entry);
        quality.push(quality_index);
    }
    // Each product attempt, in start order, on its case's own slot.
    let mut sources: Vec<Option<Value>> = vec![None; inv.sources().len()];
    let mut product_attempts = Vec::with_capacity(attempts.len() + with_run);
    let mut case_parts: Vec<Option<Value>> = vec![None; count];
    for attempt in attempts {
        let request = attempt.request;
        let ordinal = requests.iter().position(|r| *r == request).filter(|_| attempt.prepared);
        let case_scope = pc.case_scope(request);
        let diagnostic_ref = diagnostic_ids[request].clone();
        let (attempt_v, source, part) = pc.with_case(request, |pc| {
            serialize_attempt(&e, pc, attempt, inv, ordinal, &case_scope, staged, &env, case_ids[request], diagnostic_ref)
        })?;
        if let Some((index, source)) = source {
            let slot = sources.get_mut(index).ok_or(assoc("sources[]"))?;
            if slot.replace(source).is_some() {
                return Err(assoc("sources[]"));
            }
        }
        product_attempts.push(attempt_v);
        case_parts[request] = Some(part);
    }
    // B2-P (C3a): the operand preparations, in first-need order, and each prepared one's
    // CaseSource at its registered index (C3a-2, C3a-5).
    let mut operand_preparations = Vec::with_capacity(preparations.len());
    for record in preparations {
        let (record_v, source) = pc.with_case(record.owner, |pc| serialize_operand_preparation(&e, pc, record, inv, case_ids[record.owner]))?;
        if let Some((index, source)) = source {
            let slot = sources.get_mut(index).ok_or(assoc("sources[]"))?;
            if slot.replace(source).is_some() {
                return Err(assoc("sources[]"));
            }
        }
        operand_preparations.push(record_v);
    }
    // B2-P (B2-C §2.5, §4): each combination with a Run: its CombinationSource (after every
    // case source, so its operands' identities are recomputed from them), its attempt, and its
    // entry's Run members.
    let mut combination_parts: Vec<Option<Value>> = vec![None; combos.len()];
    for combination in combos.iter_mut() {
        if combination.native().is_none() {
            continue;
        }
        let index = combination.index;
        let rows = pc.combination_rows.get(index).cloned().ok_or(assoc("combinations[].result_ids"))?;
        let id = combination_ids[index];
        let parts = rp::PreparedCases::with_combination(pc, combination, |pc, combination| {
            serialize_combination_attempt(&e, pc, combination, inv, &owners, &sources, rows.clone(), staged, &env, id)
        }).ok_or(assoc("combinations[].operands"))??;
        let (attempt_v, (source_index, source_v), part) = parts;
        let slot = sources.get_mut(source_index).ok_or(assoc("sources[]"))?;
        if slot.replace(source_v).is_some() {
            return Err(assoc("sources[]"));
        }
        product_attempts.push(attempt_v);
        combination_parts[index] = Some(part);
    }
    let sources = sources.into_iter().collect::<Option<Vec<_>>>().ok_or(assoc("sources[]"))?;
    // cases[]: one per requested case, in request order.
    let mut cases_v = Vec::with_capacity(count);
    for (index, part) in case_parts.into_iter().enumerate() {
        let attempt_ref = attempt_of[index].map(|attempt| attempt.attempt);
        let mut case_v = json!({"basis_ref":{"ref_type":"load_case","ref_id":case_ids[index]},
            "ordinary":{"attempt_ref":index,"quality_binding":{"kind":"present","index":quality[index]}},"product_attempt_ref":attempt_ref});
        let object = case_v.as_object_mut().ok_or(assoc("cases[]"))?;
        match part {
            Some(Value::Object(part)) => object.extend(part),
            Some(_) => return Err(assoc("cases[]")),
            None => {
                object.insert("status".into(), json!("not_required"));
            }
        }
        cases_v.push(case_v);
    }
    // B2-P (B2-C §2.7): combinations[], one per model combination, in authored order.
    let mut combination_refs = vec![Vec::new(); combos.len()];
    case_diagnostic_refs_into(&env, &combination_ids, &mut combination_refs)?;
    let mut combinations_v = Vec::with_capacity(combos.len());
    for (combination, ((raw_combination, refs), part)) in combos.iter().zip(model_combinations.iter().zip(combination_refs).zip(combination_parts)) {
        combinations_v.push(combination_entry(&e, combination, raw_combination, &env, refs, part, combination_diagnostics[combination.index].clone(), &preparations[..])?);
    }
    let last = inv.runs().last().ok_or(scope("cases[].run"))?;
    let (calls, groups, builds, charged) = invocation_arrays_mapped(&e, inv, last, &owners)?;
    let execution_order = inv.runs().iter().map(|run| owners.owner(&run.owner)).collect::<Option<Vec<_>>>().ok_or(assoc("work.execution_order"))?;
    let material = material_basis(&e, pc, raw, count)?;
    finish_with(e, env, invocation, charged, json!(execution_order), cases_v, sources, material, (calls, groups, builds), ordinary,
        product_attempts, legacy_source_work, combinations_v, operand_preparations)
}

/// B2-P (B2-C §2.7): a retained combination's diagnostic, after the case diagnostics: `selected`
/// (its rows then carry `recovery_method`) or `unavailable`. A collision with an existing id is
/// a serializer refusal (`Association`), as for cases.
fn combination_envelope(env: &mut Value, combination_id: &str, selected: bool) -> Result<String, ReceiptFailure> {
    let assoc = |p| fail(ReceiptCheck::Association, p);
    if selected {
        for row in env["results"].as_array_mut().ok_or(assoc("results"))? {
            if row["basis_ref"]["ref_type"] == json!("combination") && row["basis_ref"]["ref_id"] == json!(combination_id) {
                row["recovery_method"] = json!(METHOD);
            }
        }
    }
    let diags = env["diagnostics"].as_array_mut().ok_or(assoc("diagnostics"))?;
    let (id, code, message) = if selected {
        (format!("diagnostic:retained-precision:{combination_id}:selected"), SELECTED_CODE, COMBINATION_SELECTED_MESSAGE)
    } else {
        (format!("diagnostic:retained-precision:{combination_id}:unavailable"), UNAVAILABLE_CODE, COMBINATION_UNAVAILABLE_MESSAGE)
    };
    if diags.iter().any(|d| d["id"] == json!(id)) {
        return Err(assoc("diagnostics[+combination].id"));
    }
    diags.push(json!({"id":id,"code":code,"severity":"info","message":message,"source":"core/product_physics","affected_refs":[combination_id]}));
    Ok(id)
}

/// B2-P (B2-C §2.7; C1 §5): a combination's expression from the invocation: mechanics terms
/// with their factors' bits in authored order; subtraction's minuend then subtrahend; a range's
/// operand ids sorted as the producer sorts them (Rust `String` order: UTF-8 bytes) and its mode.
fn combination_expression(e: &Enc, raw: &Value) -> Result<Value, ReceiptFailure> {
    let assoc = |p| fail(ReceiptCheck::Association, p);
    Ok(match raw["basis"].as_str() {
        Some("mechanics") => {
            let terms = raw["terms"].as_array().ok_or(assoc("combinations[].expression.terms"))?;
            json!({"kind":"mechanics","terms":terms.iter().map(|term| Ok(json!({"case_id":term["load_case"].as_str().ok_or(assoc("combinations[].expression.terms[].case_id"))?,
                "factor":e.bits(term["factor"].as_f64().ok_or(assoc("combinations[].expression.terms[].factor"))?,"combinations[].expression.terms[].factor")})))
                .collect::<Result<Vec<_>, ReceiptFailure>>()?})
        }
        Some("result_state_subtraction") => json!({"kind":"result_state_subtraction",
            "minuend_id":raw["minuend_id"].as_str().ok_or(assoc("combinations[].expression.minuend_id"))?,
            "subtrahend_id":raw["subtrahend_id"].as_str().ok_or(assoc("combinations[].expression.subtrahend_id"))?}),
        Some("range_envelope") => {
            let mut ids = raw["operand_ids"].as_array().ok_or(assoc("combinations[].expression.operand_ids"))?.iter()
                .map(|id| id.as_str().map(str::to_owned)).collect::<Option<Vec<String>>>().ok_or(assoc("combinations[].expression.operand_ids"))?;
            ids.sort();
            json!({"kind":"range_envelope","operand_ids":ids,"mode":raw["mode"].as_str().ok_or(assoc("combinations[].expression.mode"))?})
        }
        _ => return Err(fail(ReceiptCheck::Untranslated, "combinations[].expression")),
    })
}

/// B2-P (B2-C §2.7; SCHEMA `Combination`): one combination's entry by its disposition.
#[allow(clippy::too_many_arguments)]
fn combination_entry(e: &Enc, combination: &rp::CombinationAttempt, raw: &Value, env: &Value, diagnostic_refs: Vec<Value>, part: Option<Value>,
    diagnostic: Option<String>, preparations: &[rp::OperandPreparation]) -> Result<Value, ReceiptFailure> {
    let assoc = |p| fail(ReceiptCheck::Association, p);
    let id = raw["id"].as_str().ok_or(assoc("combinations[].basis_ref"))?;
    let result_ids: Vec<Value> = env["results"].as_array().ok_or(assoc("results"))?.iter()
        .filter(|row| row["basis_ref"]["ref_type"] == json!("combination") && row["basis_ref"]["ref_id"] == json!(id))
        .map(|row| row["id"].clone()).collect();
    let mut entry = json!({"basis_ref":{"ref_type":"combination","ref_id":id},"expression":combination_expression(e, raw)?,
        "result_ids":result_ids,"diagnostic_refs":diagnostic_refs});
    let object = entry.as_object_mut().ok_or(assoc("combinations[]"))?;
    let unavailable = |code: &str, phase: &str, cause: Value| json!({"disposition":"retained_unavailable","reason":{"code":code,"phase":phase,"cause":cause},
        "diagnostic_ref":diagnostic.clone(),"call_ref":combination.call,"product_attempt_ref":null,"run":null,"source_ref":null});
    let members = match &combination.end {
        rp::CombinationEnd::Ordinary => json!({"disposition":"ordinary","reason":"no_retained_mechanics"}),
        rp::CombinationEnd::BaseWithheld(code) => json!({"disposition":"base_withheld","reason":code}),
        rp::CombinationEnd::OperandSourceUnavailable { operand_index } =>
            unavailable("combination_unresolved", "preparation", json!({"kind":"operand_source_unavailable","operand_index":operand_index})),
        rp::CombinationEnd::OperandPreparationFailure { operand_preparation } => {
            preparations.get(*operand_preparation).ok_or(assoc("combinations[].reason.cause"))?;
            unavailable("combination_unresolved", "preparation", json!({"kind":"operand_preparation_failure","operand_preparation_ref":operand_preparation}))
        }
        rp::CombinationEnd::PreSource { reason, .. } => unavailable("combination_unresolved", "preparation", combination_reason(e, reason)),
        rp::CombinationEnd::Native | rp::CombinationEnd::Frozen(_) | rp::CombinationEnd::Candidate(_) => part.ok_or(assoc("combinations[].run"))?,
        rp::CombinationEnd::Retained => return Err(assoc("combinations[].disposition")),
    };
    match members {
        Value::Object(members) => object.extend(members),
        _ => return Err(assoc("combinations[]")),
    }
    Ok(entry)
}

/// B2-P (C3a; SCHEMA `OperandPreparation`): one operand preparation's record, on its owner's
/// slot, and its CaseSource (with `preparation: {operand_preparation_ref, sha256}`) when prepared.
fn serialize_operand_preparation(e: &Enc, pc: &rp::ProductCapture, record: &rp::OperandPreparation, inv: &k::RecordedInvocation, case_id: &str)
    -> Result<(Value, Option<(usize, Value)>), ReceiptFailure> {
    let assoc = |p| fail(ReceiptCheck::Association, p);
    let adapter = record.trace.adapter.as_ref().ok_or(assoc("operand_preparations[].adapter"))?;
    let (old, new) = if record.trace.old_vector_swapped { (&record.parts.old_operational[..], &pc.operational[..]) } else { (&pc.operational[..], &[][..]) };
    let members = preparation_members(e, &record.trace.members, &record.parts.preparation_work);
    let prepared = record.trace.source_ready && record.source.is_some();
    let result = if prepared {
        json!({"kind":"prepared"})
    } else {
        let capture = pc.error.as_ref().ok_or(assoc("operand_preparations[].result"))?;
        json!({"kind":"refused","error":{"kind":"preparation","capture":capture_error(e,capture),"section":record.parts.preparation_error.as_ref().map(|s|section_error(e,s))}})
    };
    let record_v = json!({"id":record.id,"definition_id":DEFINITION_ID,"owner_ref":{"kind":"case","index":record.owner},"ordinary_attempt_ref":record.owner,
        "material_basis_ref":0,"purpose":"combination_operand","requested_by":record.requested_by,"source_ref":record.source,"result":result,
        "stage":if prepared {"completed"} else {"failed"},"preparation":{"members":members},"adapter":adapter_value(e, adapter),
        "operational":operational_value(e, record.trace.old_coverage, old, new)});
    let Some(source_id) = record.source.filter(|_| prepared) else { return Ok((record_v, None)) };
    let primitive = pc.source.as_ref().ok_or(assoc("sources[]"))?;
    let mut source = case_source_at(e, pc, primitive, inv, source_id, record.owner, case_id, new)?;
    // C3a-5: H(retained_precision_operand_preparation_v1, {definition_id, definition_sha256,
    // owner_ref, ordinary_attempt_ref, material_basis_ref, purpose, members}).
    let payload = {
        let mut payload = preparation_payload(&record_v, rp::W1Route::Preview);
        payload["purpose"] = json!("combination_operand");
        payload
    };
    let sha = domain_hash("retained_precision_operand_preparation_v1", &payload).ok_or(fail(ReceiptCheck::Encoding, "sources[].preparation.sha256"))?;
    source["preparation"] = json!({"operand_preparation_ref":record.id,"sha256":sha});
    Ok((record_v, Some((source_id, source))))
}

/// The identity of a source entry: H(`retained_precision_source_mp_v2`, it without `index`).
fn source_identity(source: &Value) -> Result<String, ReceiptFailure> {
    let mut binding = source.clone();
    binding.as_object_mut().ok_or(fail(ReceiptCheck::Association, "sources[]"))?.remove("index");
    domain_hash("retained_precision_source_mp_v2", &binding).ok_or(fail(ReceiptCheck::Encoding, "sources[].source_identity_sha256"))
}

/// B2-P (B2-C §2.5, §4): one combination with a Run, on operand 0's slot with its own result
/// fields swapped in: its `CombinationAttempt`, its `CombinationSource` (index and value), and
/// its entry's members (`retained_selected`, or `retained_unavailable` with its Run).
#[allow(clippy::too_many_arguments)]
fn serialize_combination_attempt(e: &Enc, pc: &rp::ProductCapture, combination: &rp::CombinationAttempt, inv: &k::RecordedInvocation,
    owners: &OwnerMap<'_>, sources: &[Option<Value>], rows: std::ops::Range<usize>, staged: &MechanicsEnvelope, env: &Value, combination_id: &str)
    -> Result<(Value, (usize, Value), Value), ReceiptFailure> {
    let assoc = |p| fail(ReceiptCheck::Association, p);
    // The combination's own Run is in the capture's fields (swapped in with its results).
    let case = pc.native.as_ref().ok_or(assoc("combinations[].run"))?;
    let run = inv.runs().get(case.run).filter(|run| run.id == case.run).ok_or(assoc("combinations[].run"))?;
    if !matches!(owners.owner(&run.owner), Some(owner) if owner == json!({"kind":"combination","index":combination.index})) {
        return Err(assoc("combinations[].run.origin"));
    }
    let attempt_id = combination.attempt.ok_or(assoc("combinations[].product_attempt_ref"))?;
    let mut costs = rr::ProjectionWork::default();
    let view = combination.typed_trace(pc, &mut costs).map_err(|_| assoc("product_attempts[]"))?;
    // The CombinationSource (C2 §3): K4CMB, its ledger and stiffness, and its operands.
    let origin = inv.sources().get(run.source).ok_or(assoc("sources[]"))?;
    let ledger = origin.combination_ledger.as_ref().ok_or(assoc("sources[].ledger_sha256"))?;
    let call = combination.call.and_then(|call| inv.calls().get(call)).ok_or(assoc("calls[]"))?;
    if call.requested_operands.len() != combination.operands.len() {
        return Err(assoc("sources[].operands"));
    }
    let mut operands = Vec::with_capacity(combination.operands.len());
    for (operand, requested) in combination.operands.iter().zip(&call.requested_operands) {
        let source_ref = requested.source.ok_or(assoc("sources[].operands[].source_ref"))?;
        let operand_source = sources.get(source_ref).and_then(Option::as_ref).ok_or(assoc("sources[].operands[].source_identity_sha256"))?;
        operands.push(json!({"case_index":operand.case,"factor":e.ubits(requested.factor_bits,"sources[].operands[].factor"),
            "source_ref":source_ref,"source_identity_sha256":source_identity(operand_source)?}));
    }
    let representative = operands.first().map(|operand| operand["source_ref"].clone()).ok_or(assoc("sources[].operands"))?;
    let source_v = json!({"index":run.source,"owner":{"kind":"combination","combination_index":combination.index,"combination_id":combination_id},
        "kernel_source_sha256":sha_hex(&origin.identity),"ledger_sha256":sha_hex(ledger),"stiffness_sha256":sha_hex(&origin.stiffness),
        "representative_source_ref":representative,"operands":operands});
    let (records, terminal) = kernel_outcome(e, &case.outcome);
    let run_v = run_value_owned(e, run, records, terminal, json!({"kind":"combination","index":combination.index}))?;
    let (result, part) = match &combination.end {
        rp::CombinationEnd::Frozen(frozen) => {
            let k::ExecutionOutcome::Selected(owner) = &case.outcome else { return Err(fail(ReceiptCheck::Scope, "combinations[].disposition")) };
            let certificate = frozen.certificate();
            if !certificate.owner_matches(owner) {
                return Err(assoc("combinations[].selection.owner"));
            }
            if !certificate.passed() {
                return Err(assoc("combinations[].selection.certificate"));
            }
            let scope = rp::CaseScope::block(rows.clone(), pc.route());
            let bound = pc.bind_combination_rows_scoped(staged, &scope, owner, combination_id).map_err(|_| assoc("combinations[].selection.absolute_verified"))?;
            let results = env["results"].as_array().ok_or(assoc("results"))?;
            let results = results.get(rows).ok_or(assoc("results"))?;
            let row_ids: Vec<String> = results.iter().map(|r| r["id"].as_str().map(str::to_owned)).collect::<Option<_>>().ok_or(assoc("results[].id"))?;
            if bound.len() != row_ids.len() || bound.iter().zip(&row_ids).any(|(r, id)| r.id != id.as_str()) {
                return Err(assoc("combinations[].selection.absolute_verified"));
            }
            let recipes: Vec<k::ProductRecipe> = bound.iter().map(|r| r.recipe).collect();
            let selection_v = selection(e, owner, pc, &view, certificate.verdicts(), &recipes, &row_ids)?;
            let source_identity_sha256 = source_identity(&source_v)?;
            (json!({"kind":"ready"}), json!({"disposition":"retained_selected","method":METHOD,"call_ref":call.id,"product_attempt_ref":attempt_id,
                "run":run_v,"source_ref":run.source,"source_identity_sha256":source_identity_sha256,"selection":selection_v}))
        }
        rp::CombinationEnd::Native | rp::CombinationEnd::Candidate(_) => {
            if let rp::CombinationEnd::Candidate(refused) = &combination.end {
                let owner = match &case.outcome { k::ExecutionOutcome::Selected(owner) => Some(&**owner), _ => None };
                let bound = |matches: &dyn Fn(&k::RetainedSolve) -> bool| owner.is_some_and(matches);
                if refused.proof_failure().is_some_and(|f| !bound(&|o| f.owner_matches(o))) || refused.certificate().is_some_and(|c| !bound(&|o| c.owner_matches(o))) {
                    return Err(assoc("product_attempts[].proof.owner"));
                }
            }
            let rr::ResultRef::Unavailable(failure) = &view.result else { return Err(assoc("product_attempts[].result")) };
            let error = public_error(e, failure, pc);
            // B2-C §4's reason table: a non-selected Run is the kernel's; after a selected Run,
            // the facade's. A preparation error is refused.
            let selected = matches!(case.outcome, k::ExecutionOutcome::Selected(_));
            let (code, phase) = match (error["kind"].as_str(), selected) {
                (Some("preparation"), _) => return Err(assoc("product_attempts[].result")),
                (_, true) => ("facade_certificate", "facade"),
                (_, false) => ("combination_unresolved", "kernel"),
            };
            (json!({"kind":"unavailable","error":error}), json!({"disposition":"retained_unavailable","call_ref":call.id,"product_attempt_ref":attempt_id,
                "reason":{"code":code,"phase":phase,"cause":{"kind":"prepared_product_failure","product_attempt_ref":attempt_id}},
                "diagnostic_ref":format!("diagnostic:retained-precision:{combination_id}:unavailable"),"run":run_v,"source_ref":run.source}))
        }
        _ => return Err(assoc("combinations[].disposition")),
    };
    let names = ["native", "proof_start", "projection", "maxima", "values", "aliases", "certificate", "observables", "g5a"];
    let mut stages_v = Map::new();
    for (n, s) in names.iter().zip(view.stages.iter().skip(1)) {
        stages_v.insert((*n).into(), stage_name(e, *s));
    }
    let attempt_v = json!({"id":attempt_id,"definition_id":COMBINATION_DEFINITION_ID,"owner_ref":{"kind":"combination","index":combination.index},
        "material_basis_ref":0,"source_ref":run.source,"run_ref":case.run,"result":result,"stages":stages_v,"proof":proof_trace(e,&view,pc),
        "adapter":adapter_value(e,view.adapter),"overlay_work":scalar(view.overlay_work),"g5a_work":scalar(view.g5a_work)});
    Ok((attempt_v, (run.source, source_v), part))
}

/// One case's Run in the invocation's one call (G-j): the kernel's Run for this case, owned by
/// its batch ordinal.
fn case_run<'a>(inv: &'a k::RecordedInvocation, case: &k::RecordedCase, ordinal: Option<usize>) -> Result<&'a k::RunOrigins, ReceiptFailure> {
    let run = inv.runs().get(case.run).ok_or(fail(ReceiptCheck::Scope, "cases[].run"))?;
    if run.id != case.run || run.call != 0 || !matches!((run.owner, ordinal), (k::NativeOwner::Case(i), Some(o)) if i == o) {
        return Err(fail(ReceiptCheck::Association, "cases[].run.origin"));
    }
    Ok(run)
}

/// One product attempt's entries (T-11), on the capture with its case in place: its
/// `product_attempts[]` entry, its `CaseSource` (by its index in `sources[]`) when it was
/// submitted, and its case's status members.
#[allow(clippy::too_many_arguments)]
fn serialize_attempt(e: &Enc, pc: &rp::ProductCapture, attempt: &rp::CaseAttempt, inv: &k::RecordedInvocation, ordinal: Option<usize>,
    case_scope: &rp::CaseScope, staged: &MechanicsEnvelope, env: &Value, case_id: &str, diagnostic_ref: Option<String>)
    -> Result<(Value, Option<(usize, Value)>, Value), ReceiptFailure> {
    let assoc = |p| fail(ReceiptCheck::Association, p);
    let request = attempt.request;
    let mut costs = rr::ProjectionWork::default();
    if let rp::AttemptEnd::Frozen(frozen) = &attempt.end {
        let case = pc.native.as_ref().ok_or(assoc("cases[].run"))?;
        let k::ExecutionOutcome::Selected(owner) = &case.outcome else { return Err(fail(ReceiptCheck::Scope, "cases[].status")) };
        // U2 (RV77-N4): the proof must have started on this selected owner's own solve.
        let certificate = frozen.certificate();
        if !certificate.owner_matches(owner) {
            return Err(assoc("cases[].selection.owner"));
        }
        if !certificate.passed() {
            return Err(assoc("cases[].selection.certificate"));
        }
        let run = case_run(inv, case, ordinal)?;
        let view = attempt.typed_trace(pc, &mut costs).map_err(|_| assoc("product_attempts[]"))?;
        // Row bindings: the producer's own QuantityId/recipe binding of the case's staged rows.
        let rows = pc.bind_rows_scoped(staged, case_scope, owner).map_err(|_| assoc("cases[].selection.absolute_verified"))?;
        let results = env["results"].as_array().ok_or(assoc("results"))?;
        let results = results.get(case_scope.row_range(results.len())).ok_or(assoc("results"))?;
        let row_ids: Vec<String> = results.iter().map(|r| r["id"].as_str().map(str::to_owned)).collect::<Option<_>>().ok_or(assoc("results[].id"))?;
        if rows.len() != row_ids.len() || rows.iter().zip(&row_ids).any(|(r, id)| r.id != id.as_str()) {
            return Err(assoc("cases[].selection.absolute_verified"));
        }
        let recipes: Vec<k::ProductRecipe> = rows.iter().map(|r| r.recipe).collect();
        let (records, terminal) = kernel_outcome(e, &case.outcome);
        let run_v = run_value(e, run, records, terminal, request)?;
        let mut source = case_source(e, pc, owner.source(), inv, run, request, case_id, &view)?;
        let attempt_v = product_attempt(e, &view, attempt.attempt, request, Some(run.source), Some(case.run), json!({"kind":"ready"}), pc);
        bind_preparation(&mut source, &attempt_v, attempt.attempt, pc.route())?;
        let source_identity = {
            let mut binding = source.clone();
            binding.as_object_mut().ok_or(assoc("sources[]"))?.remove("index");
            domain_hash("retained_precision_source_mp_v2", &binding).ok_or(fail(ReceiptCheck::Encoding, "cases[].source_identity_sha256"))?
        };
        let selection_v = selection(e, owner, pc, &view, certificate.verdicts(), &recipes, &row_ids)?;
        let part = json!({"status":"selected","method":METHOD,"run":run_v,"source_ref":run.source,"source_identity_sha256":source_identity,"selection":selection_v});
        return Ok((attempt_v, Some((run.source, source)), part));
    }
    let diagnostic_ref = diagnostic_ref.ok_or(assoc("cases[].diagnostic_ref"))?;
    let selected_owner = pc.native.as_ref().and_then(|case| match &case.outcome { k::ExecutionOutcome::Selected(o) => Some(&**o), _ => None });
    // U2 on the failure path: refused proof work and any certificate bind to the selected
    // owner structurally (before any projection reads them).
    if let rp::AttemptEnd::Candidate(refused) = &attempt.end {
        let bound = |matches: &dyn Fn(&k::RetainedSolve) -> bool| selected_owner.is_some_and(matches);
        if refused.proof_failure().is_some_and(|f| !bound(&|o| f.owner_matches(o))) {
            return Err(assoc("product_attempts[].proof.owner"));
        }
        if refused.certificate().is_some_and(|c| !bound(&|o| c.owner_matches(o))) {
            return Err(assoc("product_attempts[].proof.owner"));
        }
    }
    let view = attempt.typed_trace(pc, &mut costs).map_err(|_| assoc("product_attempts[]"))?;
    let rr::ResultRef::Unavailable(failure) = &view.result else { return Err(assoc("product_attempts[].result")) };
    let error = public_error(e, failure, pc);
    let cause = json!({"kind":"prepared_product_failure","product_attempt_ref":attempt.attempt});
    match pc.native.as_ref() {
        // T-7: preparation refused, so no CaseSource, Run or execution entry.
        None => {
            if matches!(failure, rr::FailureRef::Preparation { capture: rp::CaptureError::Source(_), .. }) {
                return Err(fail(ReceiptCheck::Untranslated, "cases[].source_decline"));
            }
            if view.prepared_source.is_some() || ordinal.is_some() {
                return Err(fail(ReceiptCheck::Untranslated, "sources[].kernel_source_sha256"));
            }
            let attempt_v = product_attempt(e, &view, attempt.attempt, request, None, None, json!({"kind":"unavailable","error":error}), pc);
            let part = json!({"status":"unavailable","reason":{"code":"source_unavailable","phase":"preparation","cause":cause},
                "diagnostic_ref":diagnostic_ref,"run":null,"source_ref":null});
            Ok((attempt_v, None, part))
        }
        // T-8 (a Run not selected) or T-9 (refused after a selected Run).
        Some(case) => {
            let run = case_run(inv, case, ordinal)?;
            let (records, terminal) = kernel_outcome(e, &case.outcome);
            let kind = terminal["kind"].as_str().unwrap_or_default().to_owned();
            let run_v = run_value(e, run, records, terminal, request)?;
            let source = match selected_owner { Some(owner) => owner.source(), None => view.prepared_source.ok_or(assoc("sources[]"))? };
            let mut source_v = case_source(e, pc, source, inv, run, request, case_id, &view)?;
            // The readers' accepted D4d table (S06): a nonselected Run is the kernel's;
            // after a selected Run, the facade's.
            let code = match (error["kind"].as_str(), kind.as_str()) {
                (Some("preparation"), _) => return Err(assoc("product_attempts[].result")),
                (_, "selected") => ("facade_certificate".to_owned(), "facade"),
                (_, other) => (format!("kernel_{other}"), "kernel"),
            };
            let attempt_v = product_attempt(e, &view, attempt.attempt, request, Some(run.source), Some(case.run), json!({"kind":"unavailable","error":error}), pc);
            if view.members.iter().all(|m| matches!(m.result, rr::PreparationResult::Prepared(_))) {
                bind_preparation(&mut source_v, &attempt_v, attempt.attempt, pc.route())?;
            }
            let part = json!({"status":"unavailable","reason":{"code":code.0,"phase":code.1,"cause":cause},
                "diagnostic_ref":diagnostic_ref,"run":run_v,"source_ref":run.source});
            Ok((attempt_v, Some((run.source, source_v)), part))
        }
    }
}

/// Test access to the D39 mapping with its first-failure encoder.
#[cfg(test)]
pub(super) fn test_legacy_source(seed: Option<&rp::LegacySeed>, case_index: usize, work: &mut Vec<Value>)
    -> Result<(Value, Option<String>), ReceiptFailure> {
    let e = Enc::default();
    let mapped = legacy_source(&e, seed, case_index, work, true)?;
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
/// Test access to the run-level conservation checks over explicit amounts (RV82 N1′).
#[cfg(test)]
pub(super) fn test_run_conservation_amounts(attempts: &[Value], case: u64, before: u64, increment: u64, after: u64) -> Vec<ReceiptFailure> {
    let e = Enc::default();
    run_conservation_amounts(&e, attempts, case, before, increment, after);
    e.failures.into_inner()
}

/// G-i translation corpus (test only): every variant constructible outside the
/// kernel, labelled by its schema `$def`, with the encoder failures it records.
#[cfg(test)]
pub(super) mod corpus {
    use super::*;
    use open_pipe_stress_frame_kernel::exact_sum::SumError;
    pub(in super::super) struct Entry {
        pub def: &'static str,
        pub label: String,
        pub value: Value,
        pub failures: Vec<ReceiptFailure>,
    }
    fn one(out: &mut Vec<Entry>, def: &'static str, label: impl Into<String>, f: impl FnOnce(&Enc) -> Value) {
        let e = Enc::default();
        let value = f(&e);
        out.push(Entry { def, label: label.into(), value, failures: e.failures.into_inner() });
    }
    fn dof() -> k::Dof {
        k::Dof { node: 3, component: k::Component::Ry }
    }
    pub(in super::super) fn wide_errors() -> Vec<k::WideError> {
        vec![k::WideError::CountRange("w"), k::WideError::WorkAccounting(k::WorkFault::Overflow), k::WideError::InvalidPrecision(96),
            k::WideError::NonFinite, k::WideError::ExponentRange, k::WideError::DivisionByZero, k::WideError::NegativeSqrt,
            k::WideError::NotNormalized, k::WideError::AngleDomain, k::WideError::ArctangentLimit, k::WideError::SplitOverflow,
            k::WideError::Accumulator(SumError::NonFinite), k::WideError::Accumulator(SumError::AccumulatorOverflow),
            k::WideError::Accumulator(SumError::NonRepresentable), k::WideError::OperandPrecision]
    }
    pub(in super::super) fn stops() -> Vec<k::AttemptStop> {
        let issues = [k::CertificateIssue::Shape, k::CertificateIssue::PairIdentity, k::CertificateIssue::Precision, k::CertificateIssue::RowIdentity,
            k::CertificateIssue::MissingField, k::CertificateIssue::NegativeField, k::CertificateIssue::NonFinite, k::CertificateIssue::NonCanonicalZero,
            k::CertificateIssue::RadiusClassMismatch];
        let mut v = vec![k::AttemptStop::CountRange("rows"), k::AttemptStop::WorkAccounting(k::WorkFault::Overflow),
            k::AttemptStop::WorkAccounting(k::WorkFault::Inconsistent), k::AttemptStop::WorkAccounting(k::WorkFault::Both),
            k::AttemptStop::Budget(k::BudgetScope::Case), k::AttemptStop::Budget(k::BudgetScope::Invocation), k::AttemptStop::Span,
            k::AttemptStop::Exponent, k::AttemptStop::Arithmetic(k::WideError::DivisionByZero), k::AttemptStop::Structure,
            k::AttemptStop::Pivot { global_dof: 7 }, k::AttemptStop::ZeroDiagonal { global_dof: 8 }, k::AttemptStop::NegativeEnergy { i: 1, j: 2 },
            k::AttemptStop::Condition, k::AttemptStop::ResidualGate { global_dof: 9 },
            k::AttemptStop::ResolutionScale { body: 1, kind: k::Kind::Translation }, k::AttemptStop::ResolutionScale { body: 1, kind: k::Kind::Rotation },
            k::AttemptStop::ResolutionScale { body: 1, kind: k::Kind::Force }, k::AttemptStop::ResolutionScale { body: 1, kind: k::Kind::Moment },
            k::AttemptStop::PublicationCertificate { index: None, issue: k::CertificateIssue::Shape }];
        v.extend(issues.into_iter().map(|issue| k::AttemptStop::PublicationCertificate { index: Some(4), issue }));
        v
    }
    pub(in super::super) fn build() -> Vec<Entry> {
        let mut out = Vec::new();
        for (i, w) in wide_errors().into_iter().enumerate() {
            one(&mut out, "WideError", format!("wide_{i}"), |e| wide_error(e, &w));
        }
        for (i, s) in stops().into_iter().enumerate() {
            one(&mut out, "Stop", format!("stop_{i}"), |e| stop(e, &s));
        }
        let q = k::QuantityId::Displacement(dof());
        let quantities = [q, k::QuantityId::Reaction(dof()), k::QuantityId::DisplacementMagnitude(2),
            k::QuantityId::EndAction { member: 1, end: k::End::I, component: k::Component::Ux },
            k::QuantityId::EndAction { member: 1, end: k::End::J, component: k::Component::Rz },
            k::QuantityId::StationAction { station: 3, component: k::Component::Uy },
            k::QuantityId::SpringAction { spring: 0, component: k::Component::Uz },
            k::QuantityId::SupportForceMagnitude(1), k::QuantityId::SupportMomentMagnitude(1)];
        for (i, q) in quantities.into_iter().enumerate() {
            one(&mut out, "Quantity", format!("quantity_{i}"), |e| quantity(e, q));
        }
        let reasons = vec![k::AttemptReason::Stop(k::AttemptStop::Condition),
            k::AttemptReason::StopRule { quantity: q, body: 0, kind: k::Kind::Translation },
            k::AttemptReason::VerificationEstimate { quantity: q, body: 0, kind: k::Kind::Force },
            k::AttemptReason::Charge { quantity: q, body: 1, kind: k::Kind::Moment }, k::AttemptReason::VerificationFailed,
            k::AttemptReason::PublicationEnclosure { quantity: q, body: 0, kind: k::Kind::Rotation, predicate: k::PublicationPredicate::AbsoluteBound },
            k::AttemptReason::PublicationEnclosure { quantity: q, body: 0, kind: k::Kind::Rotation, predicate: k::PublicationPredicate::PublicRelative },
            k::AttemptReason::PublicationEnclosure { quantity: q, body: 0, kind: k::Kind::Rotation, predicate: k::PublicationPredicate::SharperExact },
            k::AttemptReason::PublicationEnclosure { quantity: q, body: 0, kind: k::Kind::Rotation, predicate: k::PublicationPredicate::SharperBinary64 },
            k::AttemptReason::Uc { body: 2 }, k::AttemptReason::Theta { body: 3 }, k::AttemptReason::GValidity { member: 4 }];
        for (i, r) in reasons.iter().enumerate() {
            one(&mut out, "Reason", format!("attempt_{i}"), |e| attempt_reason(e, r));
        }
        for (i, r) in reasons.into_iter().enumerate() {
            one(&mut out, "Outcome", format!("outcome_rejected_{i}"), |e| outcome(e, &k::AttemptOutcome::Rejected(r.clone())));
            one(&mut out, "Outcome", format!("outcome_failed_{i}"), |e| outcome(e, &k::AttemptOutcome::Failed(r)));
        }
        for (i, o) in [k::AttemptOutcome::Accepted, k::AttemptOutcome::Verified, k::AttemptOutcome::Solved].into_iter().enumerate() {
            one(&mut out, "Outcome", format!("outcome_{i}"), |e| outcome(e, &o));
        }
        let unresolved_reasons = vec![k::UnresolvedReason::CountRange("cases"),
            k::UnresolvedReason::WorkAccounting { fault: k::WorkFault::Both, prior: Some(k::AttemptStop::Span) },
            k::UnresolvedReason::Ceiling, k::UnresolvedReason::Budget(k::BudgetScope::Invocation), k::UnresolvedReason::ExactSumSpan,
            k::UnresolvedReason::ExponentRange, k::UnresolvedReason::ZeroDiagonal { global_dof: 5 },
            k::UnresolvedReason::Arithmetic(k::WideError::InvalidPrecision(8)),
            k::UnresolvedReason::ResolutionScaleUnencodable { body: 1, kind: k::Kind::Moment },
            k::UnresolvedReason::CertifiedBoundUnencodable { body: 2 },
            k::UnresolvedReason::PublicationCertificate { index: Some(3), issue: k::CertificateIssue::RowIdentity }];
        for (i, r) in unresolved_reasons.iter().enumerate() {
            one(&mut out, "Unresolved", format!("unresolved_{i}"), |e| unresolved(e, r));
        }
        let refusals = vec![k::Refusal::MechanismWitnessed { body: 0, rigid_parameters: [1.0, 0.0, -0.5, 0.25, 2.0, 0.0] },
            k::Refusal::GeometryUnavailable { body: 1, error: StructuralError::Range("geometry") },
            k::Refusal::NegativeEnergy { i: 3, j: 4 }, k::Refusal::LedgerUnavailable(k::LedgerRefusal::Accumulator(SumError::NonRepresentable)),
            k::Refusal::Structure];
        for (i, r) in refusals.iter().enumerate() {
            one(&mut out, "Refusal", format!("refusal_{i}"), |e| refusal(e, r));
        }
        let passes = [k::BoundPass::Forward, k::BoundPass::Pivot, k::BoundPass::Backward, k::BoundPass::NlColumn, k::BoundPass::NlScale,
            k::BoundPass::NlRow, k::BoundPass::Form, k::BoundPass::Need, k::BoundPass::Sigma, k::BoundPass::ShiftFactor, k::BoundPass::ShiftForm];
        for (i, pass) in passes.into_iter().enumerate() {
            let b = k::BlockRefusal { block: i as u32, bound: if i % 2 == 0 { k::CertifiedBound::Uc } else { k::CertifiedBound::S },
                refusal: k::BoundRefusal { kind: if i % 3 == 0 { k::RefusalKind::Span } else { k::RefusalKind::Exponent }, pass,
                    row: if i == 0 { usize::MAX } else { i } } };
            one(&mut out, "BlockRefusal", format!("block_{i}"), |_| block_refusal(&b));
        }
        let numerics = vec![k::NumericError::Arithmetic(k::AttemptStop::Exponent), k::NumericError::NonFinite, k::NumericError::NonpositiveSource,
            k::NumericError::InvalidGeometry, k::NumericError::InvalidMaterial, k::NumericError::MaterialBits, k::NumericError::TemperatureOrder,
            k::NumericError::NonpositiveDenominator, k::NumericError::AxisBits, k::NumericError::Binary64Range];
        for (i, n) in numerics.iter().enumerate() {
            one(&mut out, "NumericError", format!("numeric_{i}"), |e| numeric_error(e, n));
        }
        let views = vec![k::ViewFailure::Certificate(k::CertificateIssue::NonFinite), k::ViewFailure::ForeignOwner, k::ViewFailure::UnsupportedCombination,
            k::ViewFailure::VerificationCache, k::ViewFailure::Ordering, k::ViewFailure::BodyBound, k::ViewFailure::CountRange,
            k::ViewFailure::Work(k::WorkFault::Inconsistent)];
        for (i, v) in views.iter().enumerate() {
            one(&mut out, "ViewIssue", format!("view_{i}"), |_| view_issue(v));
        }
        let numeric = k::NumericError::AxisBits;
        let bridges = vec![k::BridgeFailure::View(k::ViewFailure::Ordering), k::BridgeFailure::Numeric(&numeric), k::BridgeFailure::MemberOwner,
            k::BridgeFailure::UnsupportedDirectionalSpring, k::BridgeFailure::MissingRadius(11), k::BridgeFailure::MissingUniquenessWarrant(2),
            k::BridgeFailure::RowIdentity(12), k::BridgeFailure::CountRange, k::BridgeFailure::Storage, k::BridgeFailure::AlphaCondition { block: 3 }];
        for (i, b) in bridges.iter().enumerate() {
            one(&mut out, "BridgeError", format!("bridge_{i}"), |e| bridge_error(e, b));
        }
        let helper_stop = k::AttemptStop::Structure;
        let predicates = [k::ProductPredicate::Absolute, k::ProductPredicate::SharperExact, k::ProductPredicate::SharperBinary64,
            k::ProductPredicate::DecimalSi, k::ProductPredicate::DecimalRaw, k::ProductPredicate::InputDerived];
        let mut products = vec![k::ProductFailureView::Association("assoc"), k::ProductFailureView::Numeric(&numeric),
            k::ProductFailureView::Native(k::BridgeFailure::Storage), k::ProductFailureView::Accounting(k::WorkFault::Overflow),
            k::ProductFailureView::CountRange("count"), k::ProductFailureView::Storage, k::ProductFailureView::G5a("g5a"),
            k::ProductFailureView::Helper(k::HelperFailure::Arithmetic(&helper_stop)), k::ProductFailureView::Helper(k::HelperFailure::InvalidSmallBoundInput),
            k::ProductFailureView::Helper(k::HelperFailure::Binary64Range), k::ProductFailureView::Helper(k::HelperFailure::Invariant)];
        products.extend(predicates.into_iter().map(|predicate| k::ProductFailureView::Predicate { row: 6, predicate }));
        for (i, p) in products.into_iter().enumerate() {
            one(&mut out, "ProductError", format!("product_{i}"), |e| product_error_view(e, p));
        }
        let props = [k::MemberProperty::ElasticModulus, k::MemberProperty::ShearModulus, k::MemberProperty::Area, k::MemberProperty::SecondMomentY,
            k::MemberProperty::SecondMomentZ, k::MemberProperty::TorsionConstant, k::MemberProperty::YReference];
        let mut sources = vec![k::SourceError::CountRange("nodes"), k::SourceError::NoNodes, k::SourceError::NonFiniteCoordinate { node: 1 },
            k::SourceError::NodeOutOfRange { node: 9 }, k::SourceError::DuplicateMemberId { id: 2 }, k::SourceError::RepeatedMemberNode { member: 3 },
            k::SourceError::ZeroLength { member: 4 }, k::SourceError::DegenerateAxis { member: 5 }, k::SourceError::DuplicateSpringId { id: 6 },
            k::SourceError::NonPositiveSpring { id: 7 }, k::SourceError::ZeroDirection { id: 8 }, k::SourceError::NonFiniteDirection { id: 9 },
            k::SourceError::DuplicateConstraint { dof: dof() }, k::SourceError::NonFiniteValue { dof: dof() }, k::SourceError::EmptyLoadSource { dof: dof() },
            k::SourceError::DuplicateStationId { id: 10 }, k::SourceError::UnknownMember { station: 11, member: 12 },
            k::SourceError::StationOutOfRange { id: 13 }, k::SourceError::DuplicateSupportId { id: 14 }, k::SourceError::SupportMismatch { id: 15 }];
        for property in props {
            sources.push(k::SourceError::NonFiniteProperty { member: 1, property });
            sources.push(k::SourceError::NonPositiveProperty { member: 1, property });
            sources.push(k::SourceError::SubnormalDerivedPrimitive { member: 1, property });
        }
        for (i, s) in sources.iter().enumerate() {
            one(&mut out, "SourceError", format!("source_{i}"), |e| source_error(e, s));
        }
        let origins = [k::OriginError::CountRange("calls"), k::OriginError::Capacity, k::OriginError::Allocation, k::OriginError::MissingSelectedOrigin { operand: 1 }];
        for (i, o) in origins.iter().enumerate() {
            one(&mut out, "OriginError", format!("origin_{i}"), |_| origin_error(o));
        }
        let events = [rp::AdapterEvent::SourceVisit, rp::AdapterEvent::RowVisit, rp::AdapterEvent::MapWrite, rp::AdapterEvent::ValidationEntry,
            rp::AdapterEvent::IdentityByteRead, rp::AdapterEvent::KeyProbe, rp::AdapterEvent::AllocationRequest, rp::AdapterEvent::LibraryBoundary,
            rp::AdapterEvent::RequestedCopyBytes, rp::AdapterEvent::RustCapacityBytes];
        let op = rp::OperationalError::NonFinite { operation: rp::ScalarOperation::Div, entered: 3 };
        let mut captures = vec![rp::CaptureError::Association("assoc".into()), rp::CaptureError::CountRange("count"), rp::CaptureError::Storage("store"),
            rp::CaptureError::Source(k::SourceError::NoNodes), rp::CaptureError::Origin(k::OriginError::Capacity), rp::CaptureError::NativeUnavailable,
            rp::CaptureError::PreparedArithmetic(op.clone()), rp::CaptureError::PreparedProof(k::ProductFailure::from(k::NumericError::NonFinite)),
            rp::CaptureError::PreparedAttemptConsumed];
        captures.extend(events.iter().map(|ev| rp::CaptureError::Accounting(rp::AdapterFault::Overflow(*ev))));
        for (i, c) in captures.iter().enumerate() {
            one(&mut out, "CaptureError", format!("capture_{i}"), |e| capture_error(e, c));
        }
        let ops = [rp::OperationalError::MissingOrForeign, rp::OperationalError::Input, rp::OperationalError::Degenerate, rp::OperationalError::Accounting,
            rp::OperationalError::NonFinite { operation: rp::ScalarOperation::Add, entered: 1 },
            rp::OperationalError::NonFinite { operation: rp::ScalarOperation::Sub, entered: 2 },
            rp::OperationalError::NonFinite { operation: rp::ScalarOperation::Mul, entered: 3 },
            rp::OperationalError::CoefficientRange { coefficient: "EA/L", operation: rp::ScalarOperation::Sqrt }];
        for (i, o) in ops.iter().enumerate() {
            one(&mut out, "OperationalError", format!("operational_{i}"), |_| op_error(o));
        }
        let g5as = [rp::G5aFailure::Accounting(rp::AdapterFault::Overflow(rp::AdapterEvent::KeyProbe)), rp::G5aFailure::Shape("shape"),
            rp::G5aFailure::Summary("summary"), rp::G5aFailure::Zero { row: 4 }, rp::G5aFailure::Sanity { body: 1, kind: 0 },
            rp::G5aFailure::Sanity { body: 1, kind: 1 }, rp::G5aFailure::Lower { member: 2, kind: 1 }, rp::G5aFailure::Sanity { body: 1, kind: 2 },
            rp::G5aFailure::Operational { member: 3, cause: op.clone() }, rp::G5aFailure::Arithmetic(rp::OperationalError::Input)];
        for (i, g) in g5as.iter().enumerate() {
            one(&mut out, "G5aError", format!("g5a_{i}"), |e| g5a_error(e, g));
        }
        let sections = [k::SectionPreparationError::InvalidGeometry, k::SectionPreparationError::Accounting,
            k::SectionPreparationError::AmbiguousRounding(4), k::SectionPreparationError::PrimitiveRange(0), k::SectionPreparationError::PrimitiveRange(5)];
        for (i, s) in sections.iter().enumerate() {
            one(&mut out, "SectionError", format!("section_{i}"), |e| section_error(e, s));
        }
        let structurals = [StructuralError::InvalidInput("input"), StructuralError::Range("range"),
            StructuralError::Asymmetric { row: 1, col: 2, relative_skew: 1e-3 },
            StructuralError::NumericallyUnresolved { reason: "zero original diagonal", global_dof: Some(5) },
            StructuralError::NumericallyUnresolved { reason: "pivot", global_dof: None },
            StructuralError::NegativeEnergy { direction: vec![0.5, -0.5, 0.0], energy: -1.0, allowance: 1e-9 },
            StructuralError::Mechanism { direction: vec![1.0, 0.0] }];
        for (i, s) in structurals.iter().enumerate() {
            one(&mut out, "StructuralError", format!("structural_{i}"), |e| structural_error(e, s));
        }
        // PublicError over the capture-free FailureRef variants.
        let capture = rp::ProductCapture::default();
        let cap = rp::CaptureError::Origin(k::OriginError::Allocation);
        let section = k::SectionPreparationError::InvalidGeometry;
        let proof = k::ProductFailure::from(k::NumericError::InvalidMaterial);
        let g5a = rp::G5aFailure::Zero { row: 1 };
        let candidate_capture = rp::PreparedCandidateError::Capture(rp::CaptureError::NativeUnavailable);
        let candidate_numeric = rp::PreparedCandidateError::Numeric;
        let refs = [rr::FailureRef::Preparation { capture: &cap, section: Some(&section) }, rr::FailureRef::Preparation { capture: &cap, section: None },
            rr::FailureRef::Native(&cap), rr::FailureRef::Candidate(&candidate_capture), rr::FailureRef::Candidate(&candidate_numeric),
            rr::FailureRef::Proof(&proof), rr::FailureRef::Observable(&cap), rr::FailureRef::G5a(&g5a)];
        for (i, f) in refs.iter().enumerate() {
            one(&mut out, "PublicError", format!("public_{i}"), |e| public_error(e, f, &capture));
        }
        out
    }
}
/// Test access to the logical projection and the kernel-outcome terminal.
#[cfg(test)]
pub(super) fn test_logical(records: &[k::AttemptRecord]) -> (Vec<Value>, Vec<ReceiptFailure>) {
    let e = Enc::default();
    let v = logical(&e, records);
    (v, e.failures.into_inner())
}
#[cfg(test)]
pub(super) fn test_kernel_terminal(outcome: &k::ExecutionOutcome) -> (usize, Value) {
    let e = Enc::default();
    let (records, terminal) = kernel_outcome(&e, outcome);
    (records.len(), terminal)
}
/// Test access to the structural checks RV82-N1 names.
#[cfg(test)]
pub(super) fn test_successor_envelope(env: &mut Value, case_id: &str, omit: Option<&str>) -> Result<String, ReceiptFailure> {
    successor_envelope(env, rp::W1Route::Preview, case_id, omit)
}
#[cfg(test)]
pub(super) fn test_after_conserved(before: u64, increment: u64, after: u64) -> (bool, Vec<ReceiptFailure>) {
    let e = Enc::default();
    let ok = after_conserved(&e, before, increment, after);
    (ok, e.failures.into_inner())
}
#[cfg(test)]
pub(super) fn test_invocation_arrays(inv: &k::RecordedInvocation, run: &k::RunOrigins) -> Result<u64, ReceiptFailure> {
    let e = Enc::default();
    // The one-case call: its one Run's owner is request index 0.
    let (_, _, _, charged) = invocation_arrays(&e, inv, run, &[0])?;
    e.finish()?;
    Ok(charged)
}
#[cfg(test)]
pub(super) fn test_ordinary_value(env: &Value, case_id: &str, seed: &rp::OrdinarySeed) -> Result<Value, ReceiptFailure> {
    let e = Enc::default();
    let (v, _) = ordinary_value(&e, env, case_id, 0, PreviewSolverMode::SparseInteractive, seed, Value::Null)?;
    Ok(v)
}
