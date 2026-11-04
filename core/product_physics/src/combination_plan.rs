//! The operand graph of a document's combinations (DEL-05-02 Work
//! `absolute-sum-srss-combinations.md` §2, §5): how an operand id resolves, the
//! evaluation order, each combination's fixed-load count κ, the load cases each
//! reaches and decision OQ-1. Both product routes read it.
//!
//! - **Resolution (§2).** A term names its operand by kind (`load_case` or
//!   `combination`). A `minuend_id`, `subtrahend_id` or `operand_ids` entry
//!   resolves to the load case when one has that id, and to a combination only
//!   when no load case has it, so a flat document keeps its meaning.
//! - **Order (§2).** The topological order that takes, among the ready
//!   combinations, the one earliest in the array. Cycles are refused before the
//!   solve (`LOAD_COMBINATION_OPERAND_CYCLE`); a combination left on one is never
//!   evaluated.
//! - **κ (§5.1)** is computed for every combination, whether or not the model has
//!   a fixed-load source, by `load_case_algebra`'s exact rules.
//! - **Reached load cases (§5.3)**: every load case reached through the operand
//!   graph, subtractions and envelopes included, in first-reached order.

use super::*;
use open_pipe_stress_load_case_algebra::{solved_state_plus_increments, FixedLoadCount};

/// One operand of a combination.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum Target<'a> {
    Case(&'a str),
    Combination(&'a str),
}

impl<'a> Target<'a> {
    pub(crate) fn id(&self) -> &'a str {
        match self {
            Target::Case(id) | Target::Combination(id) => id,
        }
    }
    /// The `basis_ref.ref_type` of the operand's own rows.
    pub(crate) fn ref_type(&self) -> &'static str {
        match self {
            Target::Case(_) => "load_case",
            Target::Combination(_) => "combination",
        }
    }
    /// Whether `row` is one of this operand's own published rows.
    pub(crate) fn owns(&self, row: &ResultItem) -> bool {
        row.basis_ref.as_ref().is_some_and(|b| b.ref_type == self.ref_type() && b.ref_id == self.id())
    }
}

/// Id resolution: the load case first.
pub(crate) struct Resolver<'m> {
    cases: HashSet<&'m str>,
}

impl<'m> Resolver<'m> {
    pub(crate) fn new(model: &'m PreviewModel) -> Self {
        Resolver { cases: model.load_cases.iter().map(|case| case.id.as_str()).collect() }
    }
    /// A resolver over the given load-case ids only (tests that build a
    /// combination without a model).
    #[cfg(test)]
    pub(crate) fn from_cases(cases: impl IntoIterator<Item = &'m str>) -> Self {
        Resolver { cases: cases.into_iter().collect() }
    }
    pub(crate) fn target<'s>(&self, id: &'s str) -> Target<'s> {
        if self.cases.contains(id) {
            Target::Case(id)
        } else {
            Target::Combination(id)
        }
    }
}

/// The plan of one document's combinations.
pub(crate) struct Plan<'m> {
    /// Indices into `model.combinations`, in evaluation order.
    pub order: Vec<usize>,
    /// κ by combination index.
    pub kappa: Vec<FixedLoadCount>,
    /// The load cases each combination reaches, by index.
    pub reached: Vec<Vec<&'m str>>,
    /// Whether another combination names this one.
    pub named: Vec<bool>,
    index_of: HashMap<&'m str, usize>,
    pub resolver: Resolver<'m>,
}

impl<'m> Plan<'m> {
    pub(crate) fn new(model: &'m PreviewModel) -> Self {
        let resolver = Resolver::new(model);
        let mut index_of = HashMap::new();
        for (index, combination) in model.combinations.iter().enumerate() {
            index_of.entry(combination.id.as_str()).or_insert(index);
        }
        let count = model.combinations.len();
        let targets: Vec<Vec<Target<'m>>> = model
            .combinations
            .iter()
            .map(|combination| match CombinationExpression::resolve(combination) {
                Some(expression) => {
                    // The targets borrow from the model, not from the expression,
                    // except a range envelope's sorted copy of its ids: re-borrow
                    // those from the combination.
                    expression
                        .targets(&resolver)
                        .iter()
                        .map(|target| {
                            let id = combination_id_in(combination, target.id());
                            match target {
                                Target::Case(_) => Target::Case(id),
                                Target::Combination(_) => Target::Combination(id),
                            }
                        })
                        .collect()
                }
                None => Vec::new(),
            })
            .collect();
        let dependencies: Vec<Vec<usize>> = targets
            .iter()
            .map(|operands| {
                let mut out = Vec::new();
                for target in operands {
                    if let Target::Combination(id) = target {
                        if let Some(&at) = index_of.get(id) {
                            if !out.contains(&at) {
                                out.push(at);
                            }
                        }
                    }
                }
                out
            })
            .collect();
        let mut named = vec![false; count];
        for operands in &dependencies {
            for &at in operands {
                named[at] = true;
            }
        }
        // Kahn's order, the earliest ready combination in the array first.
        let mut done = vec![false; count];
        let mut order = Vec::with_capacity(count);
        loop {
            let ready = (0..count).find(|&index| !done[index] && dependencies[index].iter().all(|&at| done[at] && at != index));
            let Some(index) = ready else { break };
            done[index] = true;
            order.push(index);
        }
        let mut kappa = vec![FixedLoadCount::Undefined(open_pipe_stress_load_case_algebra::FixedLoadCountUndefined::Operand); count];
        let mut reached: Vec<Vec<&'m str>> = vec![Vec::new(); count];
        for &index in &order {
            let combination = &model.combinations[index];
            let operand_kappa = |target: &Target<'m>| -> FixedLoadCount {
                match target {
                    Target::Case(_) => FixedLoadCount::load_case(),
                    Target::Combination(id) => index_of.get(id).map_or(
                        FixedLoadCount::Undefined(open_pipe_stress_load_case_algebra::FixedLoadCountUndefined::Operand),
                        |&at| kappa[at].clone(),
                    ),
                }
            };
            let operands: Vec<FixedLoadCount> = targets[index].iter().map(operand_kappa).collect();
            kappa[index] = match combination.basis.as_str() {
                "mechanics" => FixedLoadCount::mechanics(combination.terms.iter().map(|t| t.factor).zip(operands.iter())),
                "result_state_subtraction" if operands.len() == 2 => FixedLoadCount::subtraction(&operands[0], &operands[1]),
                "range_envelope" => FixedLoadCount::envelope(operands.iter()),
                "absolute_sum" | "srss" => FixedLoadCount::unsigned(operands.iter()),
                _ => FixedLoadCount::Undefined(open_pipe_stress_load_case_algebra::FixedLoadCountUndefined::Operand),
            };
            let mut cases: Vec<&'m str> = Vec::new();
            for target in &targets[index] {
                let more: Vec<&'m str> = match target {
                    Target::Case(id) => vec![id],
                    Target::Combination(id) => index_of.get(id).map(|&at| reached[at].clone()).unwrap_or_default(),
                };
                for case in more {
                    if !cases.contains(&case) {
                        cases.push(case);
                    }
                }
            }
            reached[index] = cases;
        }
        Plan { order, kappa, reached, named, index_of, resolver }
    }

    pub(crate) fn index(&self, id: &str) -> Option<usize> {
        self.index_of.get(id).copied()
    }

    /// κ of one operand.
    pub(crate) fn kappa_of(&self, target: &Target<'_>) -> FixedLoadCount {
        match target {
            Target::Case(_) => FixedLoadCount::load_case(),
            Target::Combination(id) => self.index(id).map_or(
                FixedLoadCount::Undefined(open_pipe_stress_load_case_algebra::FixedLoadCountUndefined::Operand),
                |at| self.kappa[at].clone(),
            ),
        }
    }

    /// Decision OQ-1 (§5.2): a `mechanics` combination with one load-case term of
    /// factor exactly 1, at least one other term, and every other term naming a
    /// combination whose κ is defined and exactly 0.
    pub(crate) fn solved_state_plus_increments(&self, combination: &PreviewCombination) -> bool {
        if combination.basis != "mechanics" {
            return false;
        }
        let kappas: Vec<(bool, f64, FixedLoadCount)> = combination
            .terms
            .iter()
            .map(|term| {
                let target = match &term.combination {
                    Some(id) => Target::Combination(id.as_str()),
                    None => Target::Case(term.load_case.as_str()),
                };
                (term.combination.is_none(), term.factor, self.kappa_of(&target))
            })
            .collect();
        solved_state_plus_increments(kappas.iter().map(|(case, factor, kappa)| (*case, *factor, kappa)))
    }
}

/// The combination's own copy of an operand id (a range envelope's sorted ids
/// are a copy; every id is one of the combination's authored strings).
fn combination_id_in<'m>(combination: &'m PreviewCombination, id: &str) -> &'m str {
    let authored = combination
        .minuend_id
        .iter()
        .chain(combination.subtrahend_id.iter())
        .chain(combination.operand_ids.iter().flatten())
        .map(String::as_str)
        .chain(combination.terms.iter().flat_map(|t| std::iter::once(t.load_case.as_str()).chain(t.combination.as_deref())));
    for candidate in authored {
        if candidate == id {
            return candidate;
        }
    }
    unreachable!("an operand id is one of the combination's authored ids")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn model(combinations: serde_json::Value) -> PreviewModel {
        serde_json::from_value(serde_json::json!({
            "schema_version":"0.9.0","document_kind":"openpipestress.product_preview.model",
            "project":{"id":"project:t","units":{"length":"m","force":"N","angle":"rad","pressure":"Pa","stress":"Pa","temperature":"degC"}},
            "analysis_status":{"mechanics":"ready_for_preview_diagnostics","rule_check":"not_performed_user_rule_inputs_missing","professional_acceptance":"not_provided"},
            "nodes":[],"pipe_segments":[],"supports":[],"components":[],"materials":[],
            "load_cases":[{"id":"A","primitive_loads":[]},{"id":"B","primitive_loads":[]},{"id":"shared","primitive_loads":[]}],
            "combinations":combinations
        }))
        .unwrap()
    }

    /// §2: a later-declared operand is evaluated first; ties keep the array order;
    /// an id shared by a load case and a combination resolves to the load case.
    #[test]
    fn evaluation_order_and_resolution() {
        let m = model(serde_json::json!([
            {"id":"E","basis":"srss","terms":[{"combination":"D","factor":1.0}]},
            {"id":"F","basis":"mechanics","terms":[{"load_case":"A","factor":1.0}]},
            {"id":"D","basis":"result_state_subtraction","minuend_id":"A","subtrahend_id":"B"},
            {"id":"shared","basis":"mechanics","terms":[{"load_case":"B","factor":2.0}]},
            {"id":"G","basis":"range_envelope","operand_ids":["shared","E"],"mode":"max"}
        ]));
        let plan = Plan::new(&m);
        assert_eq!(plan.order, vec![1, 2, 0, 3, 4]);
        assert_eq!(plan.resolver.target("shared"), Target::Case("shared"));
        assert_eq!(plan.resolver.target("D"), Target::Combination("D"));
        // G reaches E's cases (sorted operands: E first) and the load case `shared`, not the combination.
        assert_eq!(plan.reached[4], vec!["A", "B", "shared"]);
        assert!(plan.kappa[2].is_exact_zero() && plan.kappa[0].is_exact_zero());
        assert_eq!(plan.kappa[3], FixedLoadCount::Defined(open_pipe_stress_load_case_algebra::Dyadic::from_f64(2.0).unwrap()));
        assert_eq!(plan.named, vec![true, false, true, false, false]);
    }

    #[test]
    fn a_cycle_is_never_evaluated() {
        let m = model(serde_json::json!([
            {"id":"X","basis":"mechanics","terms":[{"combination":"Y","factor":1.0}]},
            {"id":"Y","basis":"mechanics","terms":[{"combination":"X","factor":1.0}]},
            {"id":"Z","basis":"mechanics","terms":[{"combination":"Z","factor":1.0}]},
            {"id":"W","basis":"mechanics","terms":[{"load_case":"A","factor":1.0}]}
        ]));
        assert_eq!(Plan::new(&m).order, vec![3]);
    }

    #[test]
    fn decision_oq1() {
        let m = model(serde_json::json!([
            {"id":"D","basis":"result_state_subtraction","minuend_id":"A","subtrahend_id":"B"},
            {"id":"E","basis":"srss","terms":[{"combination":"D","factor":1.0}]},
            {"id":"O","basis":"mechanics","terms":[{"load_case":"B","factor":1.0},{"combination":"E","factor":1.0}]},
            {"id":"S","basis":"srss","terms":[{"load_case":"A","factor":1.0}]},
            {"id":"P","basis":"mechanics","terms":[{"load_case":"B","factor":1.0},{"combination":"S","factor":1.0}]},
            {"id":"Q","basis":"mechanics","terms":[{"load_case":"B","factor":1.0}]}
        ]));
        let plan = Plan::new(&m);
        let admitted: Vec<bool> = m.combinations.iter().map(|c| plan.solved_state_plus_increments(c)).collect();
        assert_eq!(admitted, vec![false, false, true, false, false, false]);
    }
}
