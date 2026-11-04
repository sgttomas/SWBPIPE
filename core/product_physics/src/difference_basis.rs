//! Operating-case slice 1b (DEL-05-02 Work `operating-case.md` §1.2, §1.3 c5, §5 B
//! option 3; DEL-02-01 Work `model-document-0.9.0.md` §2.2): a
//! `result_state_subtraction` declaring `difference_basis = "operating_minus_sustained"`
//! on the pressure-free route, its gates G-1 and G-2, its row sign convention and the
//! derivation evidence a reader without the model re-derives G-1 from.
//!
//! - **G-1** (`EXPANSION_BASIS_SUSTAINED_NOT_IN_OPERATING`): the subtrahend's primitive
//!   loads, as a multiset, are contained in the minuend's by content. The content of a
//!   primitive is its category, target, direction, dimension, and its magnitude's value
//!   bits and unit, read from the lowered model after the solve's own unit
//!   normalization (`normalize_model_units`), so condition-generated primitives count
//!   by content (E7). Ids and provenance are not content. Multiplicity counts: for
//!   each content key the subtrahend's count must not exceed the minuend's. Equal final
//!   active sets are not required. Tokens are compared as authored (no alias mapping),
//!   value bits exactly (so `+0.0` and `-0.0` are different contents).
//! - **G-2** (`EXPANSION_BASIS_MODULUS_MIXED`): the two load cases' modulus-basis keys
//!   (the resolved elastic basis, `modulus_basis_key`) differ.
//!
//! The gates run in `preview_physics::gate_reason` after the withheld-operand rule
//! (which a declared subtraction never meets: its operands are load cases,
//! `LOAD_COMBINATION_DIFFERENCE_BASIS_OPERAND_NOT_LOAD_CASE`); G-1 first, then G-2. The
//! nonlinear, κ and modulus gates of the term bases do not apply to a subtraction.
//!
//! The evidence (`difference_basis_derivation`) and the gate read one function,
//! [`contents`], over the one lowered and normalized model the solve used: there is no
//! second resolver.
use crate::{PreviewCombination, PreviewLoadCase, PreviewModel, PreviewPrimitiveLoad, LoadTargetInput};
use std::collections::BTreeMap;

/// G-1's withholding code.
pub(crate) const SUSTAINED_NOT_IN_OPERATING: &str = "EXPANSION_BASIS_SUSTAINED_NOT_IN_OPERATING";
/// G-2's withholding code.
pub(crate) const MODULUS_MIXED: &str = "EXPANSION_BASIS_MODULUS_MIXED";
/// The evidence member (contract evidence) a reader without the model re-derives G-1 from.
pub(crate) const DERIVATION_MEMBER: &str = "difference_basis_derivation";

/// The sign convention of an admitted declared subtraction's rows: the plain
/// subtraction's positive-value clause, then §1.3 c5 under §5 option 3's name.
pub(crate) const SIGN: &str = "positive value follows explicit user result-state subtraction (minuend minus subtrahend) of matching source result sign conventions; operating minus sustained difference: a signed difference of two independently solved states, each using its own reported support state and carrying its own numerical standing under the static, history-free friction model; derived from case results; not an equilibrium state";

pub(crate) const SUSTAINED_NOT_IN_OPERATING_MESSAGE: &str = "Combination withheld: it declares an operating minus sustained difference, and the subtrahend case's primitive loads, as a multiset by content (category, target, direction, dimension, value and unit after normalization; ids ignored), are not contained in the minuend case's.";
pub(crate) const MODULUS_MIXED_MESSAGE: &str = "Combination withheld: it declares an operating minus sustained difference, and its two load cases were solved on different modulus bases.";

/// A primitive load's target, as content.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum TargetKey {
    /// `(type token, the target's ids)`: `node` `[node]`, `element` `[pipe]`,
    /// `support` `[support, dof]`.
    Typed(&'static str, Vec<String>),
}

/// The six content fields of one primitive load (§2.2). The derived order is the
/// canonical sort: byte-wise over category, target type, target ids, direction,
/// dimension, value bits (as an unsigned integer, the order of their fixed-width
/// lowercase hexadecimal), unit.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct Content {
    pub category: String,
    pub target: TargetKey,
    pub direction: String,
    pub dimension: String,
    pub value_bits: u64,
    pub unit: String,
}

fn content(load: &PreviewPrimitiveLoad) -> Content {
    let target = match &load.target {
        LoadTargetInput::Node { node } => TargetKey::Typed("node", vec![node.clone()]),
        LoadTargetInput::Element { pipe } => TargetKey::Typed("element", vec![pipe.clone()]),
        LoadTargetInput::Support { support, dof } => TargetKey::Typed("support", vec![support.clone(), dof.clone()]),
    };
    Content {
        category: load.category.clone(),
        target,
        direction: load.direction.clone(),
        dimension: load.dimension.clone(),
        value_bits: load.magnitude.value.to_bits(),
        unit: load.magnitude.unit.clone(),
    }
}

/// A load case's primitive loads as a multiset by content, in the canonical order.
pub(crate) fn contents(case: &PreviewLoadCase) -> BTreeMap<Content, usize> {
    let mut out = BTreeMap::new();
    for load in &case.primitive_loads {
        *out.entry(content(load)).or_insert(0) += 1;
    }
    out
}

/// G-1: every subtrahend content occurs in the minuend at least as often.
pub(crate) fn contained(minuend: &BTreeMap<Content, usize>, subtrahend: &BTreeMap<Content, usize>) -> bool {
    subtrahend.iter().all(|(key, count)| minuend.get(key).is_some_and(|m| m >= count))
}

/// Whether a (validated) combination is a declared operating comparison. Validation
/// refused every other present value (`…_UNKNOWN`, `…_MISPLACED`) and a combination
/// operand (`…_OPERAND_NOT_LOAD_CASE`) before the gates run.
pub(crate) fn declared(combination: &PreviewCombination) -> bool {
    combination.basis == "result_state_subtraction"
        && combination.difference_basis == Some(crate::document_members::DifferenceBasisSetting::OperatingMinusSustained)
}

fn case<'m>(model: &'m PreviewModel, id: Option<&str>) -> Option<&'m PreviewLoadCase> {
    id.and_then(|id| model.load_cases.iter().find(|case| case.id == id))
}

/// The declared subtraction's gate (G-1, then G-2), or `None` when it is admitted.
/// `modulus_key` is the solve's own `modulus_basis_key`.
pub(crate) fn gate(model: &PreviewModel, combination: &PreviewCombination, mut modulus_key: impl FnMut(&PreviewLoadCase) -> Option<String>) -> Option<&'static str> {
    let (Some(minuend), Some(subtrahend)) = (case(model, combination.minuend_id.as_deref()), case(model, combination.subtrahend_id.as_deref())) else {
        // Unreachable after validation (both operands are load cases); fail closed.
        return Some(SUSTAINED_NOT_IN_OPERATING);
    };
    if !contained(&contents(minuend), &contents(subtrahend)) {
        return Some(SUSTAINED_NOT_IN_OPERATING);
    }
    if modulus_key(minuend) != modulus_key(subtrahend) {
        return Some(MODULUS_MIXED);
    }
    None
}

fn contents_evidence(contents: &BTreeMap<Content, usize>) -> Vec<serde_json::Value> {
    contents
        .iter()
        .map(|(c, count)| {
            let TargetKey::Typed(kind, ids) = &c.target;
            let target = match (*kind, ids.as_slice()) {
                ("support", [support, dof]) => serde_json::json!({"type": "support", "support": support, "dof": dof}),
                ("element", [pipe]) => serde_json::json!({"type": "element", "pipe": pipe}),
                (_, [node]) => serde_json::json!({"type": "node", "node": node}),
                _ => unreachable!("a target key has its type's ids"),
            };
            serde_json::json!({
                "category": c.category,
                "target": target,
                "direction": c.direction,
                "dimension": c.dimension,
                "magnitude": {"value_bits": format!("{:016x}", c.value_bits), "unit": c.unit},
                "count": count,
            })
        })
        .collect()
}

/// The evidence member: one record per declared subtraction among the gates, in gate
/// (evaluation) order, withheld or admitted, each binding its combination id and both
/// operand case ids to that case's content multiset. `None` when no gate is a declared
/// subtraction's, so every other document keeps its bytes. Its size is bounded by the
/// admitted model: at most one record per combination and, per operand, at most one
/// entry per primitive load of that case.
pub(crate) fn derivation(model: &PreviewModel, gates: &[serde_json::Value]) -> Option<serde_json::Value> {
    let records: Vec<serde_json::Value> = gates
        .iter()
        .filter_map(|g| model.combinations.iter().find(|c| g["combination_id"] == c.id.as_str()))
        .filter(|c| declared(c))
        .filter_map(|c| {
            let minuend = case(model, c.minuend_id.as_deref())?;
            let subtrahend = case(model, c.subtrahend_id.as_deref())?;
            Some(serde_json::json!({
                "combination_id": c.id,
                "difference_basis": crate::document_members::OPERATING_MINUS_SUSTAINED,
                "minuend": {"load_case_id": minuend.id, "primitive_contents": contents_evidence(&contents(minuend))},
                "subtrahend": {"load_case_id": subtrahend.id, "primitive_contents": contents_evidence(&contents(subtrahend))},
            }))
        })
        .collect();
    (!records.is_empty()).then(|| serde_json::Value::Array(records))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn load(id: &str, value: f64, unit: &str) -> PreviewPrimitiveLoad {
        serde_json::from_value(serde_json::json!({"id": id, "category": "weight", "target": {"type": "node", "node": "n"},
            "direction": "-Y", "magnitude": {"value": value, "unit": unit}, "dimension": "force"})).unwrap()
    }
    fn multiset(loads: &[PreviewPrimitiveLoad]) -> BTreeMap<Content, usize> {
        let mut out = BTreeMap::new();
        for l in loads {
            *out.entry(content(l)).or_insert(0) += 1;
        }
        out
    }

    #[test]
    fn g1_counts_multiplicity_and_ignores_ids() {
        let o = multiset(&[load("o1", 2.0, "N")]);
        assert!(contained(&o, &multiset(&[load("s-other-id", 2.0, "N")])), "ids are not content");
        assert!(!contained(&o, &multiset(&[load("s1", 2.0, "N"), load("s2", 2.0, "N")])), "two against one");
        assert!(contained(&multiset(&[load("o1", 2.0, "N"), load("o2", 2.0, "N")]), &multiset(&[load("s1", 2.0, "N"), load("s2", 2.0, "N")])));
        assert!(!contained(&o, &multiset(&[load("s1", f64::from_bits(2f64.to_bits() + 1), "N")])), "one ulp");
        assert!(!contained(&o, &multiset(&[load("s1", 2.0, "kN")])), "unit");
        assert!(!contained(&multiset(&[load("o1", 0.0, "N")]), &multiset(&[load("s1", -0.0, "N")])), "signed zero is bits");
        assert!(contained(&o, &BTreeMap::new()), "an empty subtrahend is contained");
    }
}
