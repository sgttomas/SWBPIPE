//! RT-A4 (DEL-05-02 Work `absolute-sum-srss-combinations.md` §2-§5), the readers'
//! view of a combination's operands: a term names a load case or a combination
//! by kind; a subtraction's or envelope's id names the load case when one has it
//! and a combination otherwise. Over published records (each a combination's id,
//! basis and expression, as the evidence states them), in their order, it
//! re-derives the fixed-load count κ, the load cases each combination reaches,
//! decision OQ-1 and the unsigned row values, with the kernel's definitions
//! (`load_case_algebra`): the readers re-derive what the producer publishes from
//! the same exact rules, never from a producer result.
use open_pipe_stress_load_case_algebra::{
    evaluate_absolute_sum, evaluate_srss, solved_state_plus_increments, AlgebraOperand, AlgebraQuantity, AnalysisStatus,
    CombinationTerm, FixedLoadCount, FixedLoadCountUndefined,
};
use open_pipe_stress_load_case_algebra::AlgebraDimension as LoadDimension;
use serde_json::Value;
use std::collections::{HashMap, HashSet};

pub(crate) const ABSOLUTE_SUM_BASIS: &str = "explicit_user_absolute_sum";
pub(crate) const SRSS_BASIS: &str = "explicit_user_srss";
pub(crate) const ABSOLUTE_SUM_SIGN: &str =
    "unsigned value: the absolute sum of |factor × source result| over explicit user terms; not an equilibrium state";
pub(crate) const SRSS_SIGN: &str =
    "unsigned value: the square root of the sum of squares of factor × source result over explicit user terms; not an equilibrium state";
pub(crate) const SOLVED_STATE_PLUS_INCREMENTS_SIGN: &str =
    "positive value follows explicit user linear combination of matching source result sign conventions; a solved state plus derived differences or bounds; not an equilibrium state";
/// The metadata of an unsigned or nested combination's `displacement_magnitude` row,
/// which states its derivation only (product `derived_magnitude_metadata`); a load
/// case's and a flat combination's carry none.
pub(crate) fn derived_magnitude_metadata(basis: &str, sign_convention: &str) -> Value {
    serde_json::json!({"component": "displacement_magnitude", "coordinate_system": "global", "location": "node", "basis": basis, "sign_convention": sign_convention})
}
pub(crate) const OPERAND_WITHHELD: &str = "LOAD_COMBINATION_OPERAND_WITHHELD";
pub(crate) const FIXED_LOAD_COUNT_UNREPRESENTABLE: &str = "COMBINATION_FIXED_LOAD_COUNT_UNREPRESENTABLE";

/// One operand.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum Operand<'a> {
    Case(&'a str),
    Combination(&'a str),
}

impl<'a> Operand<'a> {
    pub(crate) fn id(&self) -> &'a str {
        match self {
            Operand::Case(id) | Operand::Combination(id) => id,
        }
    }
    pub(crate) fn ref_type(&self) -> &'static str {
        match self {
            Operand::Case(_) => "load_case",
            Operand::Combination(_) => "combination",
        }
    }
    /// Whether `row` is one of this operand's own rows.
    pub(crate) fn owns(&self, row: &Value) -> bool {
        row["basis_ref"]["ref_type"] == self.ref_type() && row["basis_ref"]["ref_id"] == self.id()
    }
}

/// Whether the basis takes terms (`mechanics`, `absolute_sum`, `srss`).
pub(crate) fn term_basis(basis: &str) -> bool {
    matches!(basis, "mechanics" | "absolute_sum" | "srss")
}

/// A term of an evidence expression: `{load_case_id, factor}` or
/// `{combination_id, factor}`. `None` for any other shape.
pub(crate) fn evidence_term(term: &Value) -> Option<(Operand<'_>, f64)> {
    let object = term.as_object()?;
    if object.len() != 2 {
        return None;
    }
    let factor = term["factor"].as_f64().filter(|f| f.is_finite())?;
    let id = |key: &str| term[key].as_str().filter(|s| !s.is_empty());
    match (id("load_case_id"), id("combination_id")) {
        (Some(case), None) => Some((Operand::Case(case), factor)),
        (None, Some(combination)) => Some((Operand::Combination(combination), factor)),
        _ => None,
    }
}

/// The κ and reached load cases of the combinations read so far, in order.
#[derive(Default)]
pub(crate) struct Graph<'a> {
    kappa: HashMap<&'a str, FixedLoadCount>,
    reached: HashMap<&'a str, Vec<&'a str>>,
}

impl<'a> Graph<'a> {
    pub(crate) fn kappa_of(&self, operand: &Operand<'_>) -> FixedLoadCount {
        match operand {
            Operand::Case(_) => FixedLoadCount::load_case(),
            Operand::Combination(id) => self.kappa.get(id).cloned().unwrap_or(FixedLoadCount::Undefined(FixedLoadCountUndefined::Operand)),
        }
    }

    /// Whether `id` was read (an operand combination must precede its dependants).
    pub(crate) fn knows(&self, id: &str) -> bool {
        self.kappa.contains_key(id)
    }

    /// Reads one combination: its κ by §5.1 and the load cases it reaches (§5.3).
    /// `factors` are the terms' factors (term bases only), in operand order.
    pub(crate) fn add(&mut self, id: &'a str, basis: &str, operands: &[Operand<'a>], factors: &[f64]) -> FixedLoadCount {
        let kappas: Vec<FixedLoadCount> = operands.iter().map(|o| self.kappa_of(o)).collect();
        let kappa = match basis {
            "mechanics" => FixedLoadCount::mechanics(factors.iter().copied().zip(kappas.iter())),
            "result_state_subtraction" if kappas.len() == 2 => FixedLoadCount::subtraction(&kappas[0], &kappas[1]),
            "range_envelope" => FixedLoadCount::envelope(kappas.iter()),
            "absolute_sum" | "srss" => FixedLoadCount::unsigned(kappas.iter()),
            _ => FixedLoadCount::Undefined(FixedLoadCountUndefined::Operand),
        };
        let mut cases: Vec<&'a str> = Vec::new();
        for operand in operands {
            let more = match operand {
                Operand::Case(case) => vec![*case],
                Operand::Combination(c) => self.reached.get(c).cloned().unwrap_or_default(),
            };
            for case in more {
                if !cases.contains(&case) {
                    cases.push(case);
                }
            }
        }
        self.reached.insert(id, cases);
        self.kappa.insert(id, kappa.clone());
        kappa
    }

    pub(crate) fn reached(&self, id: &str) -> &[&'a str] {
        self.reached.get(id).map(Vec::as_slice).unwrap_or(&[])
    }

    /// Decision OQ-1 (§5.2) over a mechanics combination's terms.
    pub(crate) fn solved_state_plus_increments(&self, terms: &[(Operand<'_>, f64)]) -> bool {
        let kappas: Vec<(bool, f64, FixedLoadCount)> = terms.iter().map(|(o, f)| (matches!(o, Operand::Case(_)), *f, self.kappa_of(o))).collect();
        solved_state_plus_increments(kappas.iter().map(|(case, f, k)| (*case, *f, k)))
    }
}

/// The evaluation order a model's combinations take (§2): topological, the
/// earliest ready combination in the array first; a combination on a cycle is
/// never ordered. Returns indices into `combinations`.
pub(crate) fn model_order(combinations: &[Value], cases: &HashSet<&str>) -> Vec<usize> {
    let index: HashMap<&str, usize> = combinations.iter().enumerate().rev().filter_map(|(i, c)| c["id"].as_str().map(|id| (id, i))).collect();
    let dependencies: Vec<Vec<usize>> = combinations
        .iter()
        .map(|c| model_operands(c, cases).iter().filter_map(|o| match o {
            Operand::Combination(id) => index.get(id).copied(),
            Operand::Case(_) => None,
        }).collect())
        .collect();
    let mut done = vec![false; combinations.len()];
    let mut order = Vec::new();
    while let Some(next) = (0..combinations.len()).find(|&i| !done[i] && dependencies[i].iter().all(|&d| done[d] && d != i)) {
        done[next] = true;
        order.push(next);
    }
    order
}

/// A model combination's operands in evaluation order (terms; minuend, subtrahend;
/// sorted envelope ids).
pub(crate) fn model_operands<'a>(combination: &'a Value, cases: &HashSet<&str>) -> Vec<Operand<'a>> {
    let resolve = |id: &'a str| if cases.contains(id) { Operand::Case(id) } else { Operand::Combination(id) };
    let ids = |v: &'a Value| v.as_array().map(|a| a.iter().filter_map(Value::as_str).collect::<Vec<_>>()).unwrap_or_default();
    match combination["basis"].as_str().unwrap_or_default() {
        "result_state_subtraction" => [&combination["minuend_id"], &combination["subtrahend_id"]].into_iter().filter_map(Value::as_str).map(resolve).collect(),
        "range_envelope" => {
            let mut sorted = ids(&combination["operand_ids"]);
            sorted.sort_unstable();
            sorted.into_iter().map(resolve).collect()
        }
        _ => combination["terms"].as_array().into_iter().flatten().filter_map(|t| match (t["combination"].as_str(), t["load_case"].as_str()) {
            (Some(c), _) => Some(Operand::Combination(c)),
            (None, Some(case)) => Some(Operand::Case(case)),
            _ => None,
        }).collect(),
    }
}

/// A model combination's expression as the evidence states it.
pub(crate) fn evidence_expression(combination: &Value) -> Value {
    match combination["basis"].as_str() {
        Some("mechanics" | "absolute_sum" | "srss") => serde_json::json!({"terms": combination["terms"].as_array().into_iter().flatten().map(|t| match t.get("combination") {
            Some(c) => serde_json::json!({"combination_id": c, "factor": t["factor"]}),
            None => serde_json::json!({"load_case_id": t["load_case"], "factor": t["factor"]}),
        }).collect::<Vec<_>>()}),
        Some("result_state_subtraction") => serde_json::json!({"minuend_id": combination["minuend_id"], "subtrahend_id": combination["subtrahend_id"]}),
        _ => serde_json::json!({"operand_ids": combination["operand_ids"], "mode": combination["mode"]}),
    }
}

/// An unsigned row value by the kernel (§3): the absolute sum or the SRSS of the
/// factors times the source values. `None` when the kernel refuses it
/// (`NonRepresentable` included).
pub(crate) fn unsigned_value(srss: bool, factors: &[f64], values: &[f64]) -> Option<f64> {
    if factors.len() != values.len() {
        return None;
    }
    let tokens: Vec<String> = (0..values.len()).map(|i| i.to_string()).collect();
    let operands = tokens
        .iter()
        .zip(values)
        .map(|(token, value)| {
            AlgebraQuantity::new(*value, LoadDimension::Force)
                .ok()
                .map(|q| AlgebraOperand::new(token.clone(), token.clone(), q, vec![AnalysisStatus::MechanicsSolved, AnalysisStatus::HumanReviewRequired]))
        })
        .collect::<Option<Vec<_>>>()?;
    let by_id: HashMap<&str, &AlgebraOperand> = operands.iter().map(|o| (o.operand_id.as_str(), o)).collect();
    let terms: Vec<CombinationTerm> = tokens.iter().zip(factors).map(|(t, f)| CombinationTerm::new(t.clone(), *f).ok()).collect::<Option<_>>()?;
    let result = if srss { evaluate_srss(&by_id, &terms) } else { evaluate_absolute_sum(&by_id, &terms) };
    if result.is_blocked() {
        return None;
    }
    result.quantity.map(|q| q.value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_unsigned_values_are_the_kernels() {
        assert_eq!(unsigned_value(false, &[1.0, -2.0], &[3.0, 4.0]), Some(11.0));
        assert_eq!(unsigned_value(true, &[1.0, 1.0], &[3.0, -4.0]), Some(5.0));
        assert_eq!(unsigned_value(false, &[f64::MAX], &[2.0]), None);
        assert_eq!(unsigned_value(true, &[1.0, 1.0], &[0.0, -0.0]).map(f64::to_bits), Some(0));
    }

    #[test]
    fn kappa_and_oq1_over_records() {
        let mut graph = Graph::default();
        graph.add("D", "result_state_subtraction", &[Operand::Case("A"), Operand::Case("B")], &[]);
        let e = graph.add("E", "srss", &[Operand::Combination("D")], &[1.0]);
        assert!(e.is_exact_zero());
        assert!(graph.solved_state_plus_increments(&[(Operand::Case("B"), 1.0), (Operand::Combination("E"), 1.0)]));
        assert!(!graph.solved_state_plus_increments(&[(Operand::Case("B"), 1.0)]));
        assert_eq!(graph.reached("E"), ["A", "B"]);
        // RT-A3's reference shape mechanics(1·A + 1·srss(A − B)) is admitted; a singleton,
        // two load cases, a factor other than 1, an undefined κ and a nonzero κ are not.
        assert!(graph.solved_state_plus_increments(&[(Operand::Case("A"), 1.0), (Operand::Combination("E"), 1.0)]));
        graph.add("S", "srss", &[Operand::Case("A")], &[1.0]);
        graph.add("R", "range_envelope", &[Operand::Case("A"), Operand::Case("B")], &[]);
        for terms in [
            vec![(Operand::Case("A"), 1.0)],
            vec![(Operand::Case("A"), 1.0), (Operand::Case("B"), 1.0), (Operand::Combination("E"), 1.0)],
            vec![(Operand::Case("A"), 1.5), (Operand::Combination("E"), 1.0)],
            vec![(Operand::Case("B"), 1.0), (Operand::Combination("S"), 1.0)],
            vec![(Operand::Case("B"), 1.0), (Operand::Combination("R"), 1.0)],
        ] {
            assert!(!graph.solved_state_plus_increments(&terms), "{terms:?}");
        }
    }
}
