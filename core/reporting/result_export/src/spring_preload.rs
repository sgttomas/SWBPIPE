//! DEL-04-11 REQ-014 (variable spring preload), as the readers re-derive it.
//!
//! result_export does not depend on product_physics, so this restates the
//! producer's `spring_preload::dispositions` from the captured model, the way
//! the readers restate constant-effort consumption: a support whose
//! `support_hanger_type` (the explicit `hanger.hanger_type` trimmed when
//! non-empty, else the trimmed `family`) is `variable_spring_hanger` or
//! `spring_hanger` applies its `hanger.cold_load` as a constant force along the
//! positive axis of its translational stiffness DOF, in every solved case,
//! unless (in this order) it carries a `nonlinear` member, it is a skewed
//! record (a non-snubber `direction` with two or more nonzero components), it
//! has no cold load, the cold load is not a finite positive force, no stiffness
//! DOF can be read (`stiffness.dof`, else `hanger.stiffness.dof`, else its first
//! restraint), that DOF is rotational, or its node is not a model node.
//!
//! The published fact's name ([`COLD_LOAD_FACT`]) and the applied row's kind
//! ([`APPLIED_LOAD`]) are stated here once for every reader.
use serde_json::Value;
use std::collections::HashSet;

/// The `combination_derivation` member, beside `consuming_constant_effort`,
/// stating that a spring applies its cold load in the solve (PP
/// `spring_preload::COLD_LOAD_FACT`). Present, and `true`, only where it holds;
/// a document without it reads as before.
pub(crate) const COLD_LOAD_FACT: &str = "applying_variable_spring_cold_load";
/// The producer's per-case row of an applying spring (PP `APPLIED_LOAD_KIND`).
pub(crate) const APPLIED_LOAD: &str = "variable_spring_hanger_applied_load";

/// One applying spring as the producer resolves it.
#[derive(Debug, PartialEq)]
pub(crate) struct SpringPreload<'a> {
    pub id: &'a str,
    pub node: &'a str,
    /// The normalized name of its translational stiffness DOF.
    pub dof: &'static str,
    /// The cold load as `cold_load` reads it: `normalize` applied to the entered quantity.
    pub force: f64,
}

fn translation(token: &str) -> Option<Option<&'static str>> {
    match token {
        "UX" | "Ux" | "ux" => Some(Some("UX")),
        "UY" | "Uy" | "uy" => Some(Some("UY")),
        "UZ" | "Uz" | "uz" => Some(Some("UZ")),
        "RX" | "Rx" | "rx" | "RY" | "Ry" | "ry" | "RZ" | "Rz" | "rz" => Some(None),
        _ => None,
    }
}

fn hanger_type(support: &Value) -> Option<&str> {
    support["hanger"]["hanger_type"]
        .as_str()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .or_else(|| support["family"].as_str().map(str::trim))
}

/// A skewed record: a non-snubber `direction` of three finite numbers with two or
/// more nonzero components (PP `document_members::is_skewed_record`).
fn skewed(support: &Value) -> bool {
    if support["family"].as_str() == Some("snubber") {
        return false;
    }
    let d = &support["direction"];
    let components: Option<Vec<f64>> = ["x", "y", "z"].iter().map(|k| d[*k].as_f64().filter(|c| c.is_finite())).collect();
    components.is_some_and(|c| c.iter().filter(|c| **c != 0.0).count() >= 2)
}

/// Whether `support` applies its cold load, and with which values. `cold_load`
/// reads the entered cold load as the reader's own route normalizes it (`None`
/// when it cannot be read as a force).
pub(crate) fn decide<'a>(support: &'a Value, nodes: &HashSet<&str>, cold_load: impl Fn(&Value) -> Option<f64>) -> Option<SpringPreload<'a>> {
    if !matches!(hanger_type(support), Some("variable_spring_hanger" | "spring_hanger")) {
        return None;
    }
    if support.get("nonlinear").is_some_and(|n| !n.is_null()) || skewed(support) {
        return None;
    }
    let cold = &support["hanger"]["cold_load"];
    if cold.is_null() {
        return None;
    }
    let force = cold_load(cold).filter(|f| f.is_finite() && *f > 0.0)?;
    let token = support["stiffness"]["dof"]
        .as_str()
        .or_else(|| support["hanger"]["stiffness"]["dof"].as_str())
        .filter(|t| translation(t).is_some())
        .or_else(|| support["restraints"][0].as_str().filter(|t| translation(t).is_some()))?;
    let dof = translation(token).flatten()?;
    let node = support["node"].as_str().filter(|n| nodes.contains(n))?;
    Some(SpringPreload { id: support["id"].as_str()?, node, dof, force })
}

/// Every applying spring of `model`, in document order.
pub(crate) fn applying<'a>(model: &'a Value, cold_load: impl Fn(&Value) -> Option<f64>) -> Vec<SpringPreload<'a>> {
    let items = |v: &'a Value| -> &'a [Value] { v.as_array().map(Vec::as_slice).unwrap_or(&[]) };
    let nodes: HashSet<&str> = items(&model["nodes"]).iter().filter_map(|n| n["id"].as_str()).collect();
    items(&model["supports"]).iter().filter_map(|s| decide(s, &nodes, &cold_load)).collect()
}

/// A cold load read as its entered value (the preview readers' reading of
/// `constant_load`: positivity does not depend on a positive unit scale).
pub(crate) fn entered_value(cold: &Value) -> Option<f64> {
    cold["value"].as_f64()
}

/// The producer's fixed-load gate message (PP `spring_preload::fixed_load_gate_message`),
/// re-derived: a model whose only fixed-load source is a constant-effort support keeps
/// `existing` byte for byte.
pub(crate) fn fixed_load_gate_message(code: &str, constant_effort: bool, springs: bool, existing: &str) -> String {
    if !springs {
        return existing.to_string();
    }
    let (source, acts, each) = if constant_effort {
        ("a constant-effort support's force and a variable spring hanger's cold load", "act", "each")
    } else {
        ("a variable spring hanger's cold load", "acts", "it")
    };
    match code {
        "CONSTANT_EFFORT_UNSIGNED_COMBINATION_OPERAND" => format!("Combination withheld: {source} {acts} in every case state; an absolute sum or SRSS of operands whose fixed-load count is not exactly 0 would count {each} once per term with its sign discarded. The difference of two solved cases is available as a result state subtraction."),
        "CONSTANT_EFFORT_COMBINATION_OPERAND_MIXED" => format!("Combination withheld: an operand's fixed-load count is undefined, so the number of times this combination counts {source} cannot be decided."),
        "CONSTANT_EFFORT_COMBINATION_REQUIRES_SOLVE" => format!("Combination withheld: {source} would be counted by the sum of factors instead of once. The difference of two solved cases is available as a result state subtraction."),
        // Every other gate's text does not name a fixed-load source.
        _ => existing.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn model(support: Value) -> Value {
        json!({"nodes": [{"id": "n"}], "supports": [support]})
    }
    fn hanger(patch: Value) -> Value {
        let mut s = json!({"id": "sh", "node": "n", "family": "variable_spring_hanger", "restraints": ["UZ"],
            "hanger": {"hanger_type": "variable_spring_hanger", "stiffness": {"dof": "UZ", "value": {"value": 1.0e5, "unit": "N/m"}},
                "cold_load": {"value": 500.0, "unit": "N"}}});
        for (k, v) in patch.as_object().unwrap() {
            if v.is_null() { s.as_object_mut().unwrap().remove(k); } else { s[k] = v.clone(); }
        }
        s
    }

    #[test]
    fn the_restated_decision_applies_and_refuses_as_the_producer_does() {
        let m = model(hanger(json!({})));
        assert_eq!(applying(&m, entered_value), vec![SpringPreload { id: "sh", node: "n", dof: "UZ", force: 500.0 }]);
        let refused = |patch: Value| applying(&model(hanger(patch)), entered_value).is_empty();
        assert!(refused(json!({"nonlinear": {"behavior": "one_way"}})));
        assert!(refused(json!({"direction": {"x": 1.0, "y": 0.0, "z": 1.0}})));
        assert!(refused(json!({"hanger": {"hanger_type": "variable_spring_hanger", "stiffness": {"dof": "UZ"}}})));
        assert!(refused(json!({"hanger": {"hanger_type": "variable_spring_hanger", "stiffness": {"dof": "UZ"}, "cold_load": {"value": -1.0, "unit": "N"}}})));
        assert!(refused(json!({"restraints": [], "hanger": {"hanger_type": "variable_spring_hanger", "cold_load": {"value": 5.0, "unit": "N"}}})));
        assert!(refused(json!({"restraints": ["RY"], "hanger": {"hanger_type": "variable_spring_hanger", "stiffness": {"dof": "RY"}, "cold_load": {"value": 5.0, "unit": "N"}}})));
        assert!(refused(json!({"node": "elsewhere"})));
        assert!(refused(json!({"family": "spring", "hanger": null})));
        // A single-axis direction is not skewed.
        assert!(!refused(json!({"direction": {"x": 0.0, "y": 0.0, "z": 1.0}})));
    }

    #[test]
    fn a_constant_effort_only_gate_keeps_its_text() {
        assert_eq!(fixed_load_gate_message("CONSTANT_EFFORT_COMBINATION_REQUIRES_SOLVE", true, false, "old"), "old");
        assert_eq!(
            fixed_load_gate_message("CONSTANT_EFFORT_COMBINATION_REQUIRES_SOLVE", false, true, "old"),
            "Combination withheld: a variable spring hanger's cold load would be counted by the sum of factors instead of once. The difference of two solved cases is available as a result state subtraction."
        );
    }
}
