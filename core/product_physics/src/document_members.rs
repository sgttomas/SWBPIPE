//! Model document 0.9.0's document-level rules (DEL-02-01 Design
//! `model-document-0.9.0.md` §1.3, §2.1, §2.2 and §2.4; DEL-05-02 Design
//! `absolute-sum-srss-combinations.md` §2), implemented once. The applier's
//! document validator (`operation_applier::document_validator`) and the solve
//! ([`crate::operating_conditions::lower`]) give the same code and message, as
//! for `model_settings.shear_model` (0.8.0 §1.1): the solve reads the typed model
//! ([`model_findings`]), the validator the document's JSON ([`json_findings`]),
//! and both reach the same rules through the views below.
//!
//! The rules: the condition sets' (§1.3, `crate::operating_conditions`); a
//! support's `direction` (§2.1) and the snubber family (§2.4); a combination's
//! `difference_basis` and `subtrahend_support_state` (§2.2); the combination
//! operands, which name a load case first and a combination only when no load
//! case has the id, with no diagnostic for a flat document (absolute-sum-srss §2,
//! r1 F3); and the spectra (§2.4, `crate::spectra`). Malformed shapes are not
//! judged here: they take the closed-record paths (the typed decode at the
//! solve, `MODEL-SLOT-SHAPE` at the validator, which [`json_findings`] reports as
//! [`JsonFinding::Shape`]).
//!
//! The interim solve behaviour of S1 is here too: the snubber's static refusal,
//! the skew-direction refusal, the exact route's difference-basis refusal, the
//! withholding of `minuend_final` combinations and of those naming one, and the
//! removal of an exactly axis-aligned `direction`.
use crate::operating_conditions::{self, ConditionSetInput, ConditionUseInput, TestConditionInput};
use crate::spectra::{self, SpectrumInput};
use crate::{Diagnostic, PreviewModel};
use open_pipe_stress_units::{unit_by_symbol, Dimension};
use open_pipe_stress_frame_kernel::restraint_basis::{self, DirectionError, SkewCode, SkewRecordFacts};
use serde::Deserialize;
use serde_json::{Map, Value};
use std::collections::{HashMap, HashSet};

/// `direction` is null, not an object, or has a missing, non-number or non-finite component.
pub const SUPPORT_DIRECTION_INVALID: &str = "SUPPORT_DIRECTION_INVALID";
/// Every component of `direction` is ±0.
pub const SUPPORT_DIRECTION_ZERO: &str = "SUPPORT_DIRECTION_ZERO";
/// `direction` on another family, an inferred family, more than one DOF or a `nonlinear` member.
pub const SUPPORT_DIRECTION_RECORD_UNSUPPORTED: &str = "SUPPORT_DIRECTION_RECORD_UNSUPPORTED";
/// `direction` on a record whose DOF is rotational.
pub const SUPPORT_DIRECTION_ROTATIONAL_UNSUPPORTED: &str = "SUPPORT_DIRECTION_ROTATIONAL_UNSUPPORTED";
/// Solve and applier only; interim until the skew kernel: two or more nonzero components.
/// No longer emitted since S-K1 (DEL-04-03 Work `skew-restraints.md` §2.2); kept for the
/// history of its refusals.
pub const SUPPORT_DIRECTION_SKEW_UNSUPPORTED: &str = "SUPPORT_DIRECTION_SKEW_UNSUPPORTED";
/// §2.2 item 2: a skewed record on a document with a `pressure_contract` (S-K5 removes it).
pub const SUPPORT_DIRECTION_SKEW_EXACT_UNSUPPORTED: &str = "SUPPORT_DIRECTION_SKEW_EXACT_UNSUPPORTED";
/// §2.2 item 3: a skewed record in a model with any nonlinear support (S-K4 removes it).
pub const SUPPORT_DIRECTION_SKEW_NONLINEAR_MODEL_UNSUPPORTED: &str = "SUPPORT_DIRECTION_SKEW_NONLINEAR_MODEL_UNSUPPORTED";
/// §2.2 item 4 (§4.2): a nonzero component of the canonical scaling 2^s·d is subnormal.
pub const SUPPORT_DIRECTION_RANGE_UNSUPPORTED: &str = "SUPPORT_DIRECTION_RANGE_UNSUPPORTED";
/// §2.2 item 5: an `imposed_displacement` primitive targeting a skewed record (S-K3 removes it).
pub const IMPOSED_DISPLACEMENT_SKEW_SUPPORT_UNSUPPORTED: &str = "IMPOSED_DISPLACEMENT_SKEW_SUPPORT_UNSUPPORTED";
/// §2.2 item 6: a skewed record on a node with no element.
pub const SUPPORT_DIRECTION_NODE_UNCONNECTED: &str = "SUPPORT_DIRECTION_NODE_UNCONNECTED";
/// §2.2 item 7: a node with two or more skewed stops (S-K2 removes it).
pub const SUPPORT_SKEW_STOPS_PER_NODE_UNSUPPORTED: &str = "SUPPORT_SKEW_STOPS_PER_NODE_UNSUPPORTED";
/// §2.2 item 8: a node's exact constraint rank is below its row count.
pub const SUPPORT_CONSTRAINT_REDUNDANT: &str = "SUPPORT_CONSTRAINT_REDUNDANT";
/// §2.2 item 9: a nonzero prescribed translation on an axis restraint at a skewed stop's node.
pub const SUPPORT_SKEW_NODE_PRESCRIBED_MOTION_UNSUPPORTED: &str = "SUPPORT_SKEW_NODE_PRESCRIBED_MOTION_UNSUPPORTED";
/// §2.2 item 10: a skew model's ordinary formation leaves binary64 or would need W2/K2b.
pub const SUPPORT_DIRECTION_SKEW_FORCE_SCALED_UNSUPPORTED: &str = "SUPPORT_DIRECTION_SKEW_FORCE_SCALED_UNSUPPORTED";
/// §2.2 item 11: a published attributed value is outside binary64.
pub const SUPPORT_SKEW_REACTION_RANGE_UNSUPPORTED: &str = "SUPPORT_SKEW_REACTION_RANGE_UNSUPPORTED";

/// The one source of the skew code strings (M-1; §11.2): the kernel's codes by name.
pub fn skew_code(code: SkewCode) -> &'static str {
    match code {
        SkewCode::DirectionInvalid => SUPPORT_DIRECTION_INVALID,
        SkewCode::DirectionZero => SUPPORT_DIRECTION_ZERO,
        SkewCode::ExactUnsupported => SUPPORT_DIRECTION_SKEW_EXACT_UNSUPPORTED,
        SkewCode::NonlinearModelUnsupported => SUPPORT_DIRECTION_SKEW_NONLINEAR_MODEL_UNSUPPORTED,
        SkewCode::RangeUnsupported => SUPPORT_DIRECTION_RANGE_UNSUPPORTED,
        SkewCode::ImposedDisplacementSkewSupport => IMPOSED_DISPLACEMENT_SKEW_SUPPORT_UNSUPPORTED,
        SkewCode::NodeUnconnected => SUPPORT_DIRECTION_NODE_UNCONNECTED,
        SkewCode::StopsPerNode => SUPPORT_SKEW_STOPS_PER_NODE_UNSUPPORTED,
        SkewCode::ConstraintRedundant => SUPPORT_CONSTRAINT_REDUNDANT,
        SkewCode::NodePrescribedMotion => SUPPORT_SKEW_NODE_PRESCRIBED_MOTION_UNSUPPORTED,
        SkewCode::ForceScaledUnsupported => SUPPORT_DIRECTION_SKEW_FORCE_SCALED_UNSUPPORTED,
        SkewCode::ReactionRange => SUPPORT_SKEW_REACTION_RANGE_UNSUPPORTED,
    }
}

/// Whether a support carries a skewed `direction` (two or more nonzero components)
/// that the record rules admit on a static solve (not a snubber, whose refusal governs).
pub fn is_skewed_record(support: &crate::PreviewSupport) -> bool {
    support.family.as_deref() != Some("snubber") && support.direction.as_ref().is_some_and(is_skew)
}
/// A snubber restraint on a rotational DOF.
pub const SNUBBER_ROTATIONAL_RESTRAINT_UNSUPPORTED: &str = "SNUBBER_ROTATIONAL_RESTRAINT_UNSUPPORTED";
/// A snubber's stiffness is not on its one restraint, or not a positive finite linear stiffness.
pub const SNUBBER_STIFFNESS_INVALID: &str = "SNUBBER_STIFFNESS_INVALID";
/// A `hanger`, or a non-null `nonlinear`, on a snubber.
pub const SNUBBER_MEMBER_CONFLICT: &str = "SNUBBER_MEMBER_CONFLICT";
/// Interim, every static solve: a model containing a snubber.
pub const SNUBBER_DYNAMIC_ONLY_UNSUPPORTED: &str = "SNUBBER_DYNAMIC_ONLY_UNSUPPORTED";
/// The message of [`SNUBBER_DYNAMIC_ONLY_UNSUPPORTED`] after the support's name (§2.4).
pub const SNUBBER_DYNAMIC_ONLY_MESSAGE: &str =
    "snubbers are dynamic-only restraints; the response-spectrum analysis that uses them is not built yet";
/// `difference_basis` on a basis other than `result_state_subtraction`.
pub const LOAD_COMBINATION_DIFFERENCE_BASIS_MISPLACED: &str = "LOAD_COMBINATION_DIFFERENCE_BASIS_MISPLACED";
/// `difference_basis` is null or not `operating_minus_sustained`.
pub const LOAD_COMBINATION_DIFFERENCE_BASIS_UNKNOWN: &str = "LOAD_COMBINATION_DIFFERENCE_BASIS_UNKNOWN";
/// Interim (S1 until the solver lane's 1b, which removed it from the solve and the
/// applier: G-1 and G-2 gate a declared subtraction, `crate::difference_basis`); the code
/// is kept for the history of its refusals and is no longer emitted.
pub const LOAD_COMBINATION_DIFFERENCE_BASIS_UNSUPPORTED: &str = "LOAD_COMBINATION_DIFFERENCE_BASIS_UNSUPPORTED";
/// `difference_basis` on a document with `pressure_contract`.
pub const LOAD_COMBINATION_DIFFERENCE_BASIS_EXACT_UNSUPPORTED: &str = "LOAD_COMBINATION_DIFFERENCE_BASIS_EXACT_UNSUPPORTED";
/// A declared operating comparison whose minuend or subtrahend resolves to a combination.
pub const LOAD_COMBINATION_DIFFERENCE_BASIS_OPERAND_NOT_LOAD_CASE: &str = "LOAD_COMBINATION_DIFFERENCE_BASIS_OPERAND_NOT_LOAD_CASE";
/// `subtrahend_support_state` without `difference_basis`.
pub const LOAD_COMBINATION_SUBTRAHEND_SUPPORT_STATE_MISPLACED: &str = "LOAD_COMBINATION_SUBTRAHEND_SUPPORT_STATE_MISPLACED";
/// `subtrahend_support_state` is null or not `own` or `minuend_final`.
pub const LOAD_COMBINATION_SUBTRAHEND_SUPPORT_STATE_UNKNOWN: &str = "LOAD_COMBINATION_SUBTRAHEND_SUPPORT_STATE_UNKNOWN";
/// Interim until RT-A5 (DEL-05-02 `operating-case.md` §7.5), which removed it from the solve
/// and the applier: a `minuend_final` combination was withheld. Kept for its history.
pub const LOAD_COMBINATION_SUBTRAHEND_SUPPORT_STATE_UNSUPPORTED: &str = "LOAD_COMBINATION_SUBTRAHEND_SUPPORT_STATE_UNSUPPORTED";
/// Interim until the RT-A4 wiring slice, which removed it from the solve; the
/// applier keeps the code for its own refusals' history and no longer emits it.
pub const LOAD_COMBINATION_UNSIGNED_BASIS_UNSUPPORTED: &str = "LOAD_COMBINATION_UNSIGNED_BASIS_UNSUPPORTED";
/// Interim until the RT-A4 wiring slice, which removed it from the solve and the applier.
pub const LOAD_COMBINATION_NESTED_UNSUPPORTED: &str = "LOAD_COMBINATION_NESTED_UNSUPPORTED";
/// An operand id names neither a load case nor a combination (a term's `load_case` keeps
/// `LOAD_COMBINATION_LOAD_CASE_UNKNOWN`).
pub const LOAD_COMBINATION_OPERAND_UNKNOWN: &str = "LOAD_COMBINATION_OPERAND_UNKNOWN";
/// Combinations that name each other in a cycle.
pub const LOAD_COMBINATION_OPERAND_CYCLE: &str = "LOAD_COMBINATION_OPERAND_CYCLE";
/// absolute-sum-srss §4: a combination naming a withheld combination is withheld whole.
pub const LOAD_COMBINATION_OPERAND_WITHHELD: &str = "LOAD_COMBINATION_OPERAND_WITHHELD";

/// `difference_basis`'s one token.
pub const OPERATING_MINUS_SUSTAINED: &str = "operating_minus_sustained";

/// One document-level finding: its code, the JSON path of the offending member, the
/// message (the same at the validator and the solve) and the entity ids it concerns.
/// `id` is the solve's diagnostic id.
#[derive(Debug, Clone, PartialEq)]
pub struct Finding {
    pub id: String,
    pub code: &'static str,
    pub path: String,
    pub message: String,
    pub refs: Vec<String>,
}

impl Finding {
    pub(crate) fn new(code: &'static str, path: &str, message: String, refs: Vec<String>) -> Self {
        Finding { id: format!("diagnostic:model:{path}:{}", code.to_ascii_lowercase()), code, path: path.into(), message, refs }
    }

    /// `EMPTY_ID` as `validation::detect_empty_ids` forms it.
    pub(crate) fn empty_id(entity: &str, path: &str) -> Self {
        Finding {
            id: format!("diagnostic:id:{entity}:empty"),
            code: "EMPTY_ID",
            path: path.into(),
            message: format!("{entity} ID must be explicit and non-empty"),
            refs: vec![entity.to_string()],
        }
    }

    /// `DUPLICATE_ID` as `validation::detect_duplicate_ids` forms it.
    pub(crate) fn duplicate_id(entity: &str, id: &str, path: &str) -> Self {
        Finding {
            id: format!("diagnostic:id:{entity}:{}", crate::stable_suffix(id)),
            code: "DUPLICATE_ID",
            path: path.into(),
            message: format!("{entity} IDs must be unique within the preview mechanics model"),
            refs: vec![id.to_string()],
        }
    }

    pub(crate) fn into_diagnostic(self, severity: &str) -> Diagnostic {
        crate::diag(&self.id, self.code, severity, self.message, self.refs)
    }
}

/// The findings of one document, in document order.
#[derive(Debug, Default)]
pub struct MemberFindings(Vec<Finding>);

impl MemberFindings {
    pub(crate) fn push(&mut self, finding: Finding) {
        self.0.push(finding);
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    pub fn into_vec(self) -> Vec<Finding> {
        self.0
    }
}

// ---- decoded members ---------------------------------------------------------

/// An authored `direction`: three finite components, or a value the rules refuse
/// as `SUPPORT_DIRECTION_INVALID` (null, not an object, a missing, non-number or
/// non-finite component). An undeclared key is a closed-record refusal.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DirectionInput {
    Vector([f64; 3]),
    Invalid,
}

impl DirectionInput {
    fn of_components(components: [Option<f64>; 3]) -> Self {
        match components {
            [Some(x), Some(y), Some(z)] if x.is_finite() && y.is_finite() && z.is_finite() => DirectionInput::Vector([x, y, z]),
            _ => DirectionInput::Invalid,
        }
    }

    /// The value as the validator reads it; `Err` names an undeclared key.
    pub fn of_json(value: &Value) -> Result<Self, String> {
        let Some(members) = value.as_object() else { return Ok(DirectionInput::Invalid) };
        if let Some(key) = members.keys().find(|key| !matches!(key.as_str(), "x" | "y" | "z")) {
            return Err(key.clone());
        }
        Ok(Self::of_components(["x", "y", "z"].map(|axis| members.get(axis).and_then(Value::as_f64))))
    }
}

/// A JSON number, or another value (drained), for a component.
struct Component(Option<f64>);

impl<'de> Deserialize<'de> for Component {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = Value::deserialize(deserializer)?;
        Ok(Component(value.as_f64()))
    }
}

/// Any present `direction`, null included, decoded for its named rules.
pub(crate) fn deserialize_direction<'de, D: serde::Deserializer<'de>>(deserializer: D) -> Result<Option<DirectionInput>, D::Error> {
    struct DirectionVisitor;
    impl<'de> serde::de::Visitor<'de> for DirectionVisitor {
        type Value = DirectionInput;
        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("any JSON value")
        }
        fn visit_map<A: serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            let mut components = [None, None, None];
            let mut seen = [false; 3];
            while let Some(key) = map.next_key::<String>()? {
                let slot = match key.as_str() {
                    "x" => 0,
                    "y" => 1,
                    "z" => 2,
                    other => return Err(serde::de::Error::unknown_field(other, &["x", "y", "z"])),
                };
                if seen[slot] {
                    return Err(serde::de::Error::duplicate_field(["x", "y", "z"][slot]));
                }
                seen[slot] = true;
                components[slot] = map.next_value::<Component>()?.0;
            }
            Ok(DirectionInput::of_components(components))
        }
        fn visit_seq<A: serde::de::SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
            while seq.next_element::<serde::de::IgnoredAny>()?.is_some() {}
            Ok(DirectionInput::Invalid)
        }
        fn visit_bool<E>(self, _: bool) -> Result<Self::Value, E> { Ok(DirectionInput::Invalid) }
        fn visit_i64<E>(self, _: i64) -> Result<Self::Value, E> { Ok(DirectionInput::Invalid) }
        fn visit_u64<E>(self, _: u64) -> Result<Self::Value, E> { Ok(DirectionInput::Invalid) }
        fn visit_f64<E>(self, _: f64) -> Result<Self::Value, E> { Ok(DirectionInput::Invalid) }
        fn visit_str<E>(self, _: &str) -> Result<Self::Value, E> { Ok(DirectionInput::Invalid) }
        fn visit_unit<E>(self) -> Result<Self::Value, E> { Ok(DirectionInput::Invalid) }
        fn visit_none<E>(self) -> Result<Self::Value, E> { Ok(DirectionInput::Invalid) }
    }
    deserializer.deserialize_any(DirectionVisitor).map(Some)
}

/// An authored `difference_basis`: its one token, or any other value (null included).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DifferenceBasisSetting {
    OperatingMinusSustained,
    Unknown,
}

impl DifferenceBasisSetting {
    pub fn of(value: &Value) -> Self {
        match value.as_str() {
            Some(OPERATING_MINUS_SUSTAINED) => DifferenceBasisSetting::OperatingMinusSustained,
            _ => DifferenceBasisSetting::Unknown,
        }
    }
}

/// An authored `subtrahend_support_state`: `own`, `minuend_final`, or any other value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SubtrahendSupportStateSetting {
    Own,
    MinuendFinal,
    Unknown,
}

impl SubtrahendSupportStateSetting {
    pub fn of(value: &Value) -> Self {
        match value.as_str() {
            Some("own") => SubtrahendSupportStateSetting::Own,
            Some("minuend_final") => SubtrahendSupportStateSetting::MinuendFinal,
            _ => SubtrahendSupportStateSetting::Unknown,
        }
    }
}

/// Any present value, null included, kept for its named refusal.
pub(crate) fn deserialize_difference_basis<'de, D: serde::Deserializer<'de>>(deserializer: D) -> Result<Option<DifferenceBasisSetting>, D::Error> {
    Value::deserialize(deserializer).map(|value| Some(DifferenceBasisSetting::of(&value)))
}

pub(crate) fn deserialize_subtrahend_support_state<'de, D: serde::Deserializer<'de>>(deserializer: D) -> Result<Option<SubtrahendSupportStateSetting>, D::Error> {
    Value::deserialize(deserializer).map(|value| Some(SubtrahendSupportStateSetting::of(&value)))
}

// ---- views -------------------------------------------------------------------

/// The members of a support the rules read.
pub(crate) struct SupportView<'a> {
    pub id: &'a str,
    pub family: Option<&'a str>,
    pub restraints: Vec<&'a str>,
    /// `stiffness`, else the hanger's (`support_stiffness_input`): its DOF, value and unit.
    pub stiffness: Option<(&'a str, f64, &'a str)>,
    pub hanger: bool,
    pub nonlinear: bool,
    pub direction: Option<DirectionInput>,
}

/// A combination term's `combination` operand as authored (a term's `load_case` is
/// the solve's own check, `LOAD_COMBINATION_LOAD_CASE_UNKNOWN`).
pub(crate) struct TermView<'a> {
    pub combination: Option<&'a str>,
}

/// The members of a combination the rules read.
pub(crate) struct CombinationView<'a> {
    pub id: &'a str,
    pub basis: &'a str,
    pub terms: Vec<TermView<'a>>,
    pub minuend: Option<&'a str>,
    pub subtrahend: Option<&'a str>,
    pub operand_ids: Vec<&'a str>,
    pub difference_basis: Option<DifferenceBasisSetting>,
    pub subtrahend_support_state: Option<SubtrahendSupportStateSetting>,
}

/// The DOF a token names: (translational, axis 0..3), in the spellings of `Dof`.
pub(crate) fn dof_axis(token: &str) -> Option<(bool, usize)> {
    match token {
        "UX" | "Ux" | "ux" => Some((true, 0)),
        "UY" | "Uy" | "uy" => Some((true, 1)),
        "UZ" | "Uz" | "uz" => Some((true, 2)),
        "RX" | "Rx" | "rx" => Some((false, 0)),
        "RY" | "Ry" | "ry" => Some((false, 1)),
        "RZ" | "Rz" | "rz" => Some((false, 2)),
        _ => None,
    }
}

/// The DOF tokens that are `UX`, `UY` and `UZ`.
pub(crate) const TRANSLATION_TOKENS: [&str; 3] = ["UX", "UY", "UZ"];

impl SupportView<'_> {
    /// The distinct DOFs the record acts on (restraints and stiffness), or `None` when a
    /// token is not a DOF.
    fn dofs(&self) -> Option<Vec<(bool, usize)>> {
        let mut dofs = Vec::new();
        for token in self.restraints.iter().copied().chain(self.stiffness.map(|(dof, _, _)| dof)) {
            let dof = dof_axis(token)?;
            if !dofs.contains(&dof) {
                dofs.push(dof);
            }
        }
        Some(dofs)
    }
}

fn supports_findings(supports: &[SupportView<'_>], found: &mut MemberFindings) {
    for (index, support) in supports.iter().enumerate() {
        let at = format!("supports[{index}]");
        let refs = vec![support.id.to_string()];
        let snubber = support.family == Some("snubber");
        if snubber {
            snubber_findings(support, &at, &refs, found);
        }
        let Some(direction) = support.direction else { continue };
        let path = format!("{at}.direction");
        let refuse = |code: &'static str, reason: &str| Finding::new(code, &path, format!("{path} of support `{}` {reason}.", support.id), refs.clone());
        let DirectionInput::Vector(d) = direction else {
            found.push(refuse(SUPPORT_DIRECTION_INVALID, "is not three finite numbers {x, y, z}"));
            continue;
        };
        if d.iter().all(|component| *component == 0.0) {
            found.push(refuse(SUPPORT_DIRECTION_ZERO, "is zero; a direction is a non-zero vector, and absence means axis-aligned"));
            continue;
        }
        let dofs = support.dofs();
        let admitted_family = matches!(support.family, Some("line_stop" | "spring" | "snubber"));
        let one = dofs.as_ref().filter(|dofs| dofs.len() == 1).map(|dofs| dofs[0]);
        // A snubber is admitted with exactly one restraint, any stiffness being on that DOF.
        let snubber_shape = !snubber || support.restraints.len() == 1;
        if !admitted_family {
            found.push(refuse(SUPPORT_DIRECTION_RECORD_UNSUPPORTED, "is admitted only on an explicit line_stop, spring or snubber family"));
        } else if support.nonlinear {
            found.push(refuse(SUPPORT_DIRECTION_RECORD_UNSUPPORTED, "is not admitted on a support with a nonlinear member"));
        } else {
            match one {
                _ if !snubber_shape => found.push(refuse(SUPPORT_DIRECTION_RECORD_UNSUPPORTED,
                    "is admitted on a snubber only with exactly one translational restraint")),
                None => found.push(refuse(SUPPORT_DIRECTION_RECORD_UNSUPPORTED,
                    "is admitted only on a support that acts on exactly one DOF")),
                Some((false, _)) => found.push(refuse(SUPPORT_DIRECTION_ROTATIONAL_UNSUPPORTED,
                    "is on a rotational DOF; a direction is admitted only on UX, UY or UZ")),
                Some((true, _)) => {}
            }
        }
    }
}

fn snubber_findings(support: &SupportView<'_>, at: &str, refs: &[String], found: &mut MemberFindings) {
    let refuse = |code: &'static str, path: String, reason: String| Finding::new(code, &path, format!("{path} of snubber `{}` {reason}.", support.id), refs.to_vec());
    for (k, token) in support.restraints.iter().enumerate() {
        if matches!(dof_axis(token), Some((false, _))) {
            found.push(refuse(SNUBBER_ROTATIONAL_RESTRAINT_UNSUPPORTED, format!("{at}.restraints[{k}]"),
                format!("`{token}` is rotational; a snubber restrains UX, UY or UZ only")));
        }
    }
    if let Some((dof, value, unit)) = support.stiffness {
        let on_its_restraint = support.restraints.len() == 1 && dof_axis(support.restraints[0]).is_some() && dof_axis(support.restraints[0]) == dof_axis(dof);
        let stiff = value.is_finite() && value > 0.0 && unit_by_symbol(unit, Dimension::LinearStiffness).is_ok();
        if !on_its_restraint || !stiff {
            found.push(refuse(SNUBBER_STIFFNESS_INVALID, format!("{at}.stiffness"),
                "must be on its one restraint (restraints exactly [stiffness.dof]) with a positive finite value in a linear-stiffness unit".into()));
        }
    }
    if support.hanger || support.nonlinear {
        found.push(refuse(SNUBBER_MEMBER_CONFLICT, at.to_string(), "carries a hanger or nonlinear member; a snubber takes neither".into()));
    }
}

/// What an operand id names: a load case first, a combination (its first index)
/// only when no load case has the id (absolute-sum-srss §2, r1 F3), or nothing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Operand {
    LoadCase,
    Combination(usize),
    Unknown,
}

/// The id resolution of one document.
pub(crate) struct Operands<'a> {
    load_cases: HashSet<&'a str>,
    combinations: HashMap<&'a str, usize>,
}

impl<'a> Operands<'a> {
    pub(crate) fn new(load_cases: impl Iterator<Item = &'a str>, combinations: impl Iterator<Item = &'a str>) -> Self {
        let mut by_id = HashMap::new();
        for (index, id) in combinations.enumerate() {
            by_id.entry(id).or_insert(index);
        }
        Operands { load_cases: load_cases.collect(), combinations: by_id }
    }

    /// A `minuend_id`, `subtrahend_id` or `operand_ids` entry.
    pub(crate) fn id(&self, id: &str) -> Operand {
        if self.load_cases.contains(id) {
            Operand::LoadCase
        } else {
            self.combination(id)
        }
    }

    /// A term's `combination`, which names a combination by kind.
    pub(crate) fn combination(&self, id: &str) -> Operand {
        self.combinations.get(id).map_or(Operand::Unknown, |index| Operand::Combination(*index))
    }
}

/// The operand references of a combination that its basis reads: (path, id, resolution).
/// A term's `load_case` names a load case and is the solve's own check.
fn operand_refs<'v>(view: &'v CombinationView<'_>, at: &str, operands: &Operands<'_>) -> Vec<(String, &'v str, Operand)> {
    let mut refs = Vec::new();
    match view.basis {
        "mechanics" | "absolute_sum" | "srss" => {
            for (j, term) in view.terms.iter().enumerate() {
                if let Some(id) = term.combination {
                    refs.push((format!("{at}.terms[{j}].combination"), id, operands.combination(id)));
                }
            }
        }
        "result_state_subtraction" => {
            for (key, id) in [("minuend_id", view.minuend), ("subtrahend_id", view.subtrahend)] {
                if let Some(id) = id.filter(|id| !id.is_empty()) {
                    refs.push((format!("{at}.{key}"), id, operands.id(id)));
                }
            }
        }
        "range_envelope" => {
            for (k, id) in view.operand_ids.iter().enumerate() {
                refs.push((format!("{at}.operand_ids[{k}]"), *id, operands.id(id)));
            }
        }
        _ => {}
    }
    refs
}

fn combinations_findings(views: &[CombinationView<'_>], load_cases: &[&str], found: &mut MemberFindings) {
    let operands = Operands::new(load_cases.iter().copied(), views.iter().map(|view| view.id));
    let mut edges: Vec<Vec<usize>> = vec![Vec::new(); views.len()];
    for (index, view) in views.iter().enumerate() {
        let at = format!("combinations[{index}]");
        let refs = vec![view.id.to_string()];
        let subtraction = view.basis == "result_state_subtraction";
        let mut declared = false;
        if let Some(setting) = view.difference_basis {
            let path = format!("{at}.difference_basis");
            if setting == DifferenceBasisSetting::Unknown {
                found.push(Finding::new(LOAD_COMBINATION_DIFFERENCE_BASIS_UNKNOWN, &path,
                    format!("{path} of combination `{}` is not `{OPERATING_MINUS_SUSTAINED}`; absence means a plain subtraction.", view.id), refs.clone()));
            } else if !subtraction {
                found.push(Finding::new(LOAD_COMBINATION_DIFFERENCE_BASIS_MISPLACED, &path,
                    format!("{path} of combination `{}` is admitted only with basis `result_state_subtraction`.", view.id), refs.clone()));
            } else {
                declared = true;
            }
        }
        if let Some(setting) = view.subtrahend_support_state {
            let path = format!("{at}.subtrahend_support_state");
            if setting == SubtrahendSupportStateSetting::Unknown {
                found.push(Finding::new(LOAD_COMBINATION_SUBTRAHEND_SUPPORT_STATE_UNKNOWN, &path,
                    format!("{path} of combination `{}` is not `own` or `minuend_final`; absence means own.", view.id), refs.clone()));
            } else if view.difference_basis.is_none() {
                found.push(Finding::new(LOAD_COMBINATION_SUBTRAHEND_SUPPORT_STATE_MISPLACED, &path,
                    format!("{path} of combination `{}` is admitted only with difference_basis.", view.id), refs.clone()));
            }
        }
        for (path, id, operand) in operand_refs(view, &at, &operands) {
            match operand {
                Operand::Unknown => found.push(Finding::new(LOAD_COMBINATION_OPERAND_UNKNOWN, &path,
                    format!("{path} `{id}` of combination `{}` names no load case and no combination.", view.id),
                    vec![view.id.to_string(), id.to_string()])),
                Operand::Combination(target) => {
                    if !edges[index].contains(&target) {
                        edges[index].push(target);
                    }
                    if declared && (path.ends_with(".minuend_id") || path.ends_with(".subtrahend_id")) {
                        found.push(Finding::new(LOAD_COMBINATION_DIFFERENCE_BASIS_OPERAND_NOT_LOAD_CASE, &path,
                            format!("{path} `{id}` of combination `{}` is a combination; a declared operating comparison names load cases only.", view.id),
                            vec![view.id.to_string(), id.to_string()]));
                    }
                }
                Operand::LoadCase => {}
            }
        }
    }
    // Cycles: each strongly connected set of combinations with an edge inside it, once,
    // at its first combination in the array, naming every combination on it.
    let reach: Vec<Vec<bool>> = (0..views.len()).map(|from| reachable(&edges, from)).collect();
    let mut reported = vec![false; views.len()];
    for first in 0..views.len() {
        if reported[first] || !reach[first][first] {
            continue;
        }
        let members: Vec<usize> = (0..views.len()).filter(|&other| reach[first][other] && reach[other][first]).collect();
        for &member in &members {
            reported[member] = true;
        }
        let names: Vec<String> = members.iter().map(|&member| format!("`{}`", views[member].id)).collect();
        let path = format!("combinations[{first}]");
        found.push(Finding::new(LOAD_COMBINATION_OPERAND_CYCLE, &path,
            format!("combinations {} name each other in a cycle; a combination's operands must not lead back to it.", names.join(", ")),
            members.iter().map(|&member| views[member].id.to_string()).collect()));
    }
}

/// Which combinations `from` reaches by one or more edges.
fn reachable(edges: &[Vec<usize>], from: usize) -> Vec<bool> {
    let mut seen = vec![false; edges.len()];
    let mut stack = edges[from].clone();
    while let Some(next) = stack.pop() {
        if !seen[next] {
            seen[next] = true;
            stack.extend(edges[next].iter().copied());
        }
    }
    seen
}

/// Every document-level rule of the 0.9.0 members, over the views, in document order.
#[allow(clippy::too_many_arguments)]
fn all_findings(
    sets: &[ConditionSetInput],
    pipes: &[(&str, Option<&str>)],
    cases: &[(&str, Option<&ConditionUseInput>)],
    supports: &[SupportView<'_>],
    combinations: &[CombinationView<'_>],
    spectra: &[SpectrumInput],
) -> MemberFindings {
    let mut found = MemberFindings::default();
    operating_conditions::findings(sets, pipes, cases, &mut found);
    supports_findings(supports, &mut found);
    let load_cases: Vec<&str> = cases.iter().map(|(id, _)| *id).collect();
    combinations_findings(combinations, &load_cases, &mut found);
    spectra::findings(spectra, &mut found);
    found
}

pub(crate) fn support_view(support: &crate::PreviewSupport) -> SupportView<'_> {
    SupportView {
        id: &support.id,
        family: support.family.as_deref(),
        restraints: support.restraints.iter().map(String::as_str).collect(),
        stiffness: crate::support_stiffness_input(support).map(|s| (s.dof.as_str(), s.value.value, s.value.unit.as_str())),
        hanger: support.hanger.is_some(),
        nonlinear: support.nonlinear.is_some(),
        direction: support.direction,
    }
}

pub(crate) fn combination_view(combination: &crate::PreviewCombination) -> CombinationView<'_> {
    CombinationView {
        id: &combination.id,
        basis: &combination.basis,
        terms: combination
            .terms
            .iter()
            .map(|term| TermView { combination: term.combination.as_deref() })
            .collect(),
        minuend: combination.minuend_id.as_deref(),
        subtrahend: combination.subtrahend_id.as_deref(),
        operand_ids: combination.operand_ids.iter().flatten().map(String::as_str).collect(),
        difference_basis: combination.difference_basis,
        subtrahend_support_state: combination.subtrahend_support_state,
    }
}

/// The solve's reading: the typed model's members.
pub(crate) fn model_findings(model: &PreviewModel) -> MemberFindings {
    let pipes: Vec<_> = model.pipe_segments.iter().map(|pipe| (pipe.id.as_str(), pipe.condition_set_ref.as_deref())).collect();
    let cases: Vec<_> = model.load_cases.iter().map(|case| (case.id.as_str(), case.condition_use.as_ref())).collect();
    let supports: Vec<_> = model.supports.iter().map(support_view).collect();
    let combinations: Vec<_> = model.combinations.iter().map(combination_view).collect();
    all_findings(&model.condition_sets, &pipes, &cases, &supports, &combinations, &model.spectra)
}

/// One finding of the validator's reading.
#[derive(Debug, Clone, PartialEq)]
pub enum JsonFinding {
    /// A document-level rule, with the solve's code and message.
    Rule(Finding),
    /// A malformed new member or record: the validator's `MODEL-SLOT-SHAPE`.
    Shape { path: String, message: String, refs: Vec<String> },
}

fn array<'a>(root: &'a Map<String, Value>, member: &str) -> &'a [Value] {
    root.get(member).and_then(Value::as_array).map(Vec::as_slice).unwrap_or(&[])
}

fn text<'a>(record: &'a Value, key: &str) -> Option<&'a str> {
    record.get(key).and_then(Value::as_str)
}

/// A new record decoded with the solve's own closed types; a failure is a shape finding.
fn decoded<T: serde::de::DeserializeOwned>(value: &Value, path: &str, refs: Vec<String>, shapes: &mut Vec<JsonFinding>) -> Option<T> {
    match serde_json::from_value::<T>(value.clone()) {
        Ok(decoded) => Some(decoded),
        Err(error) => {
            shapes.push(JsonFinding::Shape { path: path.to_string(), message: format!("{path}: {error}"), refs });
            None
        }
    }
}

/// The validator's reading of a document's JSON: the same rules as the solve's, and
/// a shape finding for each malformed new member (the solve refuses those at decode).
/// Records the rules cannot read (not an object, or an existing member of the wrong
/// type) are left to the readers' own checks, as before 0.9.0.
pub fn json_findings(root: &Map<String, Value>) -> Vec<JsonFinding> {
    let mut shapes = Vec::new();
    let id_refs = |record: &Value| text(record, "id").map(|id| vec![id.to_string()]).unwrap_or_default();

    let mut sets = Vec::new();
    match root.get("condition_sets") {
        None => {}
        Some(Value::Array(items)) => {
            for (index, item) in items.iter().enumerate() {
                if let Some(set) = decoded::<ConditionSetInput>(item, &format!("condition_sets[{index}]"), id_refs(item), &mut shapes) {
                    sets.push(set);
                }
            }
        }
        Some(_) => shapes.push(JsonFinding::Shape { path: "condition_sets".into(), message: "condition_sets: an array of condition sets is required".into(), refs: Vec::new() }),
    }
    let mut spectra_records = Vec::new();
    match root.get("spectra") {
        None => {}
        Some(Value::Array(items)) => {
            for (index, item) in items.iter().enumerate() {
                if let Some(spectrum) = decoded::<SpectrumInput>(item, &format!("spectra[{index}]"), id_refs(item), &mut shapes) {
                    spectra_records.push(spectrum);
                }
            }
        }
        Some(_) => shapes.push(JsonFinding::Shape { path: "spectra".into(), message: "spectra: an array of spectra is required".into(), refs: Vec::new() }),
    }
    let mut pipes = Vec::new();
    for (index, pipe) in array(root, "pipe_segments").iter().enumerate() {
        let reference = match pipe.get("condition_set_ref") {
            None => None,
            Some(Value::String(reference)) => Some(reference.as_str()),
            Some(_) => {
                let path = format!("pipe_segments[{index}].condition_set_ref");
                shapes.push(JsonFinding::Shape { path: path.clone(), message: format!("{path}: a condition-set id (string) is required"), refs: id_refs(pipe) });
                None
            }
        };
        pipes.push((text(pipe, "id").unwrap_or(""), reference));
    }
    let mut uses: Vec<Option<ConditionUseInput>> = Vec::new();
    for (index, case) in array(root, "load_cases").iter().enumerate() {
        uses.push(case.get("condition_use").and_then(|value| decoded(value, &format!("load_cases[{index}].condition_use"), id_refs(case), &mut shapes)));
        if let Some(value) = case.get("test_condition") {
            decoded::<TestConditionInput>(value, &format!("load_cases[{index}].test_condition"), id_refs(case), &mut shapes);
        }
    }
    let cases: Vec<_> = array(root, "load_cases").iter().zip(&uses).map(|(case, using)| (text(case, "id").unwrap_or(""), using.as_ref())).collect();
    let mut supports = Vec::new();
    for (index, support) in array(root, "supports").iter().enumerate() {
        let direction = match support.get("direction") {
            None => None,
            Some(value) => match DirectionInput::of_json(value) {
                Ok(direction) => Some(direction),
                Err(key) => {
                    let path = format!("supports[{index}].direction.{key}");
                    shapes.push(JsonFinding::Shape { path: path.clone(), message: format!("{path} is not a member of this record."), refs: id_refs(support) });
                    None
                }
            },
        };
        let stiffness = support.get("stiffness").filter(|value| !value.is_null()).or_else(|| support.pointer("/hanger/stiffness")).and_then(|stiffness| {
            Some((text(stiffness, "dof")?, stiffness.pointer("/value/value")?.as_f64()?, stiffness.pointer("/value/unit")?.as_str()?))
        });
        supports.push(SupportView {
            id: text(support, "id").unwrap_or(""),
            family: text(support, "family"),
            restraints: support.get("restraints").and_then(Value::as_array).into_iter().flatten().filter_map(Value::as_str).collect(),
            stiffness,
            hanger: support.get("hanger").is_some_and(|value| !value.is_null()),
            nonlinear: support.get("nonlinear").is_some_and(|value| !value.is_null()),
            direction,
        });
    }
    let mut combinations = Vec::new();
    for combination in array(root, "combinations") {
        combinations.push(CombinationView {
            id: text(combination, "id").unwrap_or(""),
            basis: text(combination, "basis").unwrap_or(""),
            terms: combination
                .get("terms")
                .and_then(Value::as_array)
                .into_iter()
                .flatten()
                .map(|term| TermView { combination: text(term, "combination") })
                .collect(),
            minuend: text(combination, "minuend_id"),
            subtrahend: text(combination, "subtrahend_id"),
            operand_ids: combination.get("operand_ids").and_then(Value::as_array).into_iter().flatten().filter_map(Value::as_str).collect(),
            difference_basis: combination.get("difference_basis").map(DifferenceBasisSetting::of),
            subtrahend_support_state: combination.get("subtrahend_support_state").map(SubtrahendSupportStateSetting::of),
        });
    }
    let found = all_findings(&sets, &pipes, &cases, &supports, &combinations, &spectra_records);
    let mut out: Vec<JsonFinding> = shapes;
    out.extend(found.into_vec().into_iter().map(JsonFinding::Rule));
    out
}

// ---- interim solve behaviour (S1) ------------------------------------------------

/// The axis of an exactly axis-aligned direction (one nonzero component).
fn single_axis(d: [f64; 3]) -> Option<usize> {
    let nonzero: Vec<usize> = (0..3).filter(|&axis| d[axis] != 0.0).collect();
    (nonzero.len() == 1).then(|| nonzero[0])
}

/// §2.4's snubber refusal on every static solve, and §2.1's skew refusal (interim
/// until the skew kernel). A skewed snubber is under the snubber's refusal.
pub(crate) fn interim_support_refusals(model: &PreviewModel, diagnostics: &mut Vec<Diagnostic>) {
    for support in &model.supports {
        if support.family.as_deref() == Some("snubber") {
            diagnostics.push(crate::diag(
                &format!("diagnostic:support:{}:snubber-dynamic-only", crate::stable_suffix(&support.id)),
                SNUBBER_DYNAMIC_ONLY_UNSUPPORTED,
                "blocking",
                format!("support `{}`: {SNUBBER_DYNAMIC_ONLY_MESSAGE}", support.id),
                vec![support.id.clone()],
            ));
        } else if let (true, Some(DirectionInput::Vector(authored))) = (is_skewed_record(support), support.direction) {
            skew_admission(model, support, authored, diagnostics);
        }
    }
}

/// S-K1 skew admission (DEL-04-03 Work `skew-restraints.md` §2.2 items 2 to 4), one code
/// per record, in document order: the first that applies of a pressure contract (EX), a
/// nonlinear support anywhere in the model, and §4.2's range rule. An admitted record
/// solves on the pressure-free linear route.
fn skew_admission(model: &PreviewModel, support: &crate::PreviewSupport, authored: [f64; 3], diagnostics: &mut Vec<Diagnostic>) {
    let facts = SkewRecordFacts {
        authored,
        pressure_contract: model.pressure_contract.is_some(),
        nonlinear_model: model.supports.iter().any(|s| s.nonlinear.is_some()),
    };
    let Err(refusal) = restraint_basis::judge(&facts) else { return };
    let (slug, reason) = match refusal.code {
        SkewCode::ExactUnsupported => ("direction-skew-exact", "is on a document with a pressure contract; skewed restraints are built on the pressure-free route only, so nothing is solved".to_string()),
        SkewCode::NonlinearModelUnsupported => ("direction-skew-nonlinear-model", "is in a model with a nonlinear support; skewed restraints are built for linear models only, so nothing is solved".to_string()),
        SkewCode::RangeUnsupported => {
            let named = match refusal.direction {
                Some(DirectionError::RangeUnsupported { components, scale_exponent }) => {
                    let axes: Vec<&str> = (0..3).filter(|&i| components[i]).map(restraint_basis::axis_name).collect();
                    format!("component(s) {} are subnormal after the exact canonical scaling 2^{scale_exponent}", axes.join(", "))
                }
                _ => "has a subnormal canonical component".to_string(),
            };
            ("direction-range", format!("has a direction whose {named}; an axis-aligned direction is the authored form if that axis is meant"))
        }
        // The record rules give the invalid and zero codes first (§2.2 item 1).
        _ => return,
    };
    diagnostics.push(crate::diag(
        &format!("diagnostic:support:{}:{slug}", crate::stable_suffix(&support.id)),
        skew_code(refusal.code),
        "blocking",
        format!("support `{}` {reason}", support.id),
        vec![support.id.clone()],
    ));
}

/// The skew refusal's message, at the solve and the applier.
pub fn skew_message(support: &str) -> String {
    format!("support `{support}` has a direction with two or more nonzero components; the skewed-restraint kernel is not built yet, so a skewed support is not solved")
}

/// Whether an authored direction is skewed (two or more nonzero components).
pub fn is_skew(direction: &DirectionInput) -> bool {
    matches!(direction, DirectionInput::Vector(d) if single_axis(*d).is_none() && d.iter().any(|c| *c != 0.0))
}

/// §2.2: a declared operating comparison is refused by name on the exact route.
pub(crate) fn interim_exact_combination_refusals(model: &PreviewModel, diagnostics: &mut Vec<Diagnostic>) {
    if model.pressure_contract.is_none() {
        return;
    }
    for combination in &model.combinations {
        if combination.difference_basis.is_some() {
            diagnostics.push(crate::diag(
                &format!("diagnostic:combination:{}:difference-basis-exact", crate::stable_suffix(&combination.id)),
                LOAD_COMBINATION_DIFFERENCE_BASIS_EXACT_UNSUPPORTED,
                "blocking",
                format!("combination `{}` declares a difference basis on a document with a pressure contract; the exact route's operating comparison is not built", combination.id),
                vec![combination.id.clone()],
            ));
        }
    }
}

/// §2.1: an exactly axis-aligned direction takes today's path, byte for byte. It is
/// removed, and when its axis is not the record's DOF axis, the record's DOF tokens
/// (its single restraint and its stiffness DOF) are set to `UX`, `UY` or `UZ` for it.
pub(crate) fn lower_axis_aligned_directions(model: &mut PreviewModel) {
    for support in &mut model.supports {
        let Some(DirectionInput::Vector(d)) = support.direction else { continue };
        let Some(axis) = single_axis(d) else { continue };
        support.direction = None;
        let token = TRANSLATION_TOKENS[axis];
        let differs = |dof: &str| dof_axis(dof).is_some_and(|(_, current)| current != axis);
        for restraint in &mut support.restraints {
            if differs(restraint) {
                *restraint = token.to_string();
            }
        }
        if let Some(stiffness) = support.stiffness.as_mut() {
            if differs(&stiffness.dof) {
                stiffness.dof = token.to_string();
            }
        }
        if let Some(stiffness) = support.hanger.as_mut().and_then(|hanger| hanger.stiffness.as_mut()) {
            if differs(&stiffness.dof) {
                stiffness.dof = token.to_string();
            }
        }
    }
}

/// The interim withholding codes of one combination, in the order they are stated. RT-A5
/// (DEL-05-02 `operating-case.md` §7.5) removed the last, `minuend_final`'s: the solve forms
/// it, and its gates are G-1, G-2 and the held-status gates at the solve. None remains.
fn interim_reasons(_combination: &crate::PreviewCombination) -> Vec<(&'static str, String)> {
    Vec::new()
}

/// S1's withholding (§2.2, §2.4) as RT-A5 leaves it: no combination has an interim reason
/// (a declared difference basis and a `minuend_final` subtrahend are gated at the solve),
/// so nothing is withheld here; a reason would withhold its combination and each
/// combination that names one, directly or through others
/// (`LOAD_COMBINATION_OPERAND_WITHHELD`, absolute-sum-srss §4, naming the operand and
/// its own code). The unsigned bases and combination operands are evaluated. The
/// cases and the other combinations publish. Runs after validation, which judged
/// every combination (so no cycle remains).
pub(crate) fn withhold_interim_combinations(model: &mut PreviewModel, diagnostics: &mut Vec<Diagnostic>) {
    let mut withheld: HashMap<usize, &'static str> = HashMap::new();
    for (index, combination) in model.combinations.iter().enumerate() {
        let reasons = interim_reasons(combination);
        for (code, message) in &reasons {
            diagnostics.push(crate::diag(
                &format!("diagnostic:combination:{}:{}", crate::stable_suffix(&combination.id), code.to_ascii_lowercase()),
                code,
                "warning",
                message.clone(),
                vec![combination.id.clone()],
            ));
        }
        if let Some((code, _)) = reasons.first() {
            withheld.insert(index, code);
        }
    }
    if withheld.is_empty() {
        return;
    }
    // Dependants, in evaluation order, so that each names its own withheld operands.
    let plan = crate::combination_plan::Plan::new(model);
    for &index in &plan.order {
        if withheld.contains_key(&index) {
            continue;
        }
        let combination = &model.combinations[index];
        let Some(expression) = crate::CombinationExpression::resolve(combination) else { continue };
        let named: Vec<(String, &'static str)> = expression
            .targets(&plan.resolver)
            .iter()
            .filter_map(|target| match target {
                crate::combination_plan::Target::Combination(id) => plan.index(id).and_then(|at| withheld.get(&at)).map(|code| (id.to_string(), *code)),
                crate::combination_plan::Target::Case(_) => None,
            })
            .fold(Vec::new(), |mut out, (id, code)| {
                if !out.iter().any(|(named, _)| *named == id) {
                    out.push((id, code));
                }
                out
            });
        if named.is_empty() {
            continue;
        }
        let pairs: Vec<(&str, &str)> = named.iter().map(|(id, code)| (id.as_str(), *code)).collect();
        let mut refs = vec![combination.id.clone()];
        refs.extend(named.iter().map(|(id, _)| id.clone()));
        diagnostics.push(crate::diag(
            &format!("diagnostic:combination:{}:{}", crate::stable_suffix(&combination.id), LOAD_COMBINATION_OPERAND_WITHHELD.to_ascii_lowercase()),
            LOAD_COMBINATION_OPERAND_WITHHELD,
            "warning",
            crate::preview_physics::operand_withheld_message(&pairs),
            refs,
        ));
        withheld.insert(index, LOAD_COMBINATION_OPERAND_WITHHELD);
    }
    let mut index = 0;
    model.combinations.retain(|_| {
        let keep = !withheld.contains_key(&index);
        index += 1;
        keep
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    fn support(value: Value) -> crate::PreviewSupport {
        serde_json::from_value(value).unwrap()
    }

    #[test]
    fn a_present_direction_decodes_for_its_named_rule() {
        let decoded = |direction: Value| support(serde_json::json!({"id": "s", "node": "n", "restraints": ["UX"], "direction": direction})).direction;
        assert_eq!(decoded(serde_json::json!({"x": 1, "y": 0.5, "z": -2})), Some(DirectionInput::Vector([1.0, 0.5, -2.0])));
        for invalid in [Value::Null, serde_json::json!([1, 0, 0]), serde_json::json!("x"), serde_json::json!({"x": 1, "y": 0}), serde_json::json!({"x": 1, "y": "0", "z": 0})] {
            assert_eq!(decoded(invalid.clone()), Some(DirectionInput::Invalid), "{invalid}");
            assert_eq!(DirectionInput::of_json(&invalid), Ok(DirectionInput::Invalid), "{invalid}");
        }
        let error = serde_json::from_value::<crate::PreviewSupport>(serde_json::json!({"id": "s", "node": "n", "restraints": ["UX"], "direction": {"x": 1, "y": 0, "z": 0, "w": 1}})).unwrap_err();
        assert!(error.to_string().contains("unknown field `w`"), "{error}");
        assert_eq!(DirectionInput::of_json(&serde_json::json!({"x": 1, "y": 0, "z": 0, "w": 1})), Err("w".into()));
        assert!(support(serde_json::json!({"id": "s", "node": "n", "restraints": ["UX"]})).direction.is_none());
    }

    #[test]
    fn operands_resolve_to_the_load_case_first() {
        let operands = Operands::new(["shared", "case"].into_iter(), ["shared", "combo", "combo"].into_iter());
        assert_eq!(operands.id("shared"), Operand::LoadCase);
        assert_eq!(operands.id("combo"), Operand::Combination(1));
        assert_eq!(operands.combination("shared"), Operand::Combination(0));
        assert_eq!(operands.id("none"), Operand::Unknown);
    }
}
