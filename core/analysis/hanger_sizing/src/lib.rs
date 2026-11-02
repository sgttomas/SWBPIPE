//! Hanger sizing arithmetic (DEL-04-11 `Work/hanger-sizing.md` §6 and §7).
//!
//! Pure functions on the hot-load basis and the positions that the derived
//! solves supply: the loads of a variable spring set at the installed position,
//! the fit tests in their stated order, the selection, and the verification
//! arithmetic. The crate solves nothing, reads no catalogue of its own, ships no
//! limit, allowance or rule, and judges no hanger's suitability or code
//! compliance. Every method choice arrives in [`Method`].
//!
//! Units are SI throughout: N, m and N/m. Every position is measured from the
//! installed position (the drawn geometry, u = 0) along the candidate's positive
//! DOF axis, and a variable spring set to `F_cold` carries `F(u) = F_cold − k·u`
//! (Design §4.2).

use std::cmp::Ordering;
use std::collections::{BTreeMap, HashSet};

pub mod derived;
pub mod library;

/// A candidate's single translational DOF, or a variable record's stiffness DOF.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dof {
    Ux,
    Uy,
    Uz,
}

/// Where a variable spring carries the hot-load basis W (Design §6.1).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoadDesign {
    /// W in the design operating case: `F_cold = W + k·u_D`.
    Hot,
    /// W in the cold condition: `F_cold = W + k·u_C`.
    Cold,
}

/// The reference load of the load variation (Design §6.2).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VariationReference {
    /// Each operating case's own load, `F_i`.
    OperatingLoad,
    /// The hot-load basis, W.
    DesignLoad,
}

/// The rule that chooses among fitting records (Design §6.5).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectionRule {
    /// Smallest `F_max`, then smallest maximum variation, then `hanger_id`.
    SmallestCapacity,
    /// Smallest maximum variation, then smallest `F_max`, then `hanger_id`.
    LeastVariation,
}

/// Which record families a candidate may take (Design §2, §6.5).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FamilyPolicy {
    Variable,
    Constant,
    /// Constant records are tested only when no variable record fits.
    VariableElseConstant,
}

/// A library record's family.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HangerType {
    VariableSpring,
    ConstantEffort,
}

/// The person's method choices. Every field is required; nothing is defaulted.
#[derive(Debug, Clone, PartialEq)]
pub struct Method {
    pub load_design: LoadDesign,
    pub variation_reference: VariationReference,
    /// λ, a positive finite ratio.
    pub variation_limit: f64,
    /// a, a finite length ≥ 0 applied at each end of the travel span.
    pub travel_allowance_m: f64,
    pub selection_rule: SelectionRule,
}

/// One library record, its quantities already normalized to SI.
#[derive(Debug, Clone, PartialEq)]
pub struct Record {
    pub hanger_id: String,
    pub hanger_type: HangerType,
    pub stiffness_n_per_m: Option<f64>,
    pub stiffness_dof: Option<Dof>,
    /// `[F_min, F_max]`.
    pub working_load_range_n: Option<(f64, f64)>,
    /// T, the record's total working travel.
    pub travel_range_m: Option<f64>,
}

/// A candidate's positions from the derived solves (or the verification solves).
#[derive(Debug, Clone, PartialEq)]
pub struct Positions {
    /// u_C, the cold condition.
    pub u_cold_m: f64,
    /// u_i, one per operating case, in request order.
    pub u_operating_m: Vec<f64>,
}

/// One candidate with its hot-load basis and positions.
#[derive(Debug, Clone, PartialEq)]
pub struct Candidate {
    pub id: String,
    pub dof: Dof,
    pub family_policy: FamilyPolicy,
    /// W, the reaction of the rigid candidate in the weight case.
    pub w_n: f64,
    pub positions: Positions,
    /// The design operating case's index into `positions.u_operating_m`.
    pub design_index: usize,
}

/// The fit tests, in the order they are applied (Design §6.4).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Test {
    /// The record carries the quantities sizing needs ("not sizable" otherwise).
    E1,
    /// A variable record's stiffness DOF equals the candidate's DOF.
    E2,
    /// The cold-condition load and every operating load are positive.
    V0,
    /// The loads lie within the working load range.
    L,
    /// The required travel is within the travel range.
    T,
    /// Every variation is within the limit.
    V,
}

impl Test {
    pub fn name(self) -> &'static str {
        match self {
            Test::E1 => "E1",
            Test::E2 => "E2",
            Test::V0 => "V0",
            Test::L => "L",
            Test::T => "T",
            Test::V => "V",
        }
    }
}

const VARIABLE_TESTS: [Test; 6] = [Test::E1, Test::E2, Test::V0, Test::L, Test::T, Test::V];
const CONSTANT_TESTS: [Test; 3] = [Test::E1, Test::L, Test::T];

/// A refusal of the whole request, with every named candidate or record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Refusal {
    pub code: &'static str,
    pub names: Vec<String>,
    pub message: String,
}

/// The travel span over the installed position, the cold condition and every
/// operating case (Design §6.3).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Span {
    pub min_m: f64,
    pub max_m: f64,
}

impl Span {
    pub fn of(positions: &Positions) -> Span {
        let mut min_m = 0.0_f64.min(positions.u_cold_m);
        let mut max_m = 0.0_f64.max(positions.u_cold_m);
        for &u in &positions.u_operating_m {
            min_m = min_m.min(u);
            max_m = max_m.max(u);
        }
        Span { min_m, max_m }
    }

    /// span + 2a.
    pub fn required_travel_m(&self, allowance_m: f64) -> f64 {
        (self.max_m - self.min_m) + 2.0 * allowance_m
    }

    /// The loads at the ends of the span widened by k·a at each end:
    /// `[F_cold − k·(max + a), F_cold − k·(min − a)]`.
    pub fn envelope_n(&self, f_cold_n: f64, k: f64, allowance_m: f64) -> (f64, f64) {
        (
            f_cold_n - k * (self.max_m + allowance_m),
            f_cold_n - k * (self.min_m - allowance_m),
        )
    }
}

/// The loads of a variable record that fits.
#[derive(Debug, Clone, PartialEq)]
pub struct VariableLoads {
    /// The setting at the installed position, written to `cold_load` on application.
    pub f_cold_n: f64,
    /// F_C, the load in the cold condition.
    pub f_cold_condition_n: f64,
    /// F_i, one per operating case.
    pub f_operating_n: Vec<f64>,
    /// v_i, one per operating case.
    pub variation: Vec<f64>,
    pub envelope_n: (f64, f64),
}

impl VariableLoads {
    fn max_variation(&self) -> f64 {
        self.variation.iter().copied().fold(0.0, f64::max)
    }
}

/// A record's evaluation for one candidate: its loads, or its first failed test.
#[derive(Debug, Clone, PartialEq)]
pub enum Evaluation {
    Variable(Result<VariableLoads, Test>),
    Constant(Result<(), Test>),
}

impl Evaluation {
    pub fn failed(&self) -> Option<Test> {
        match self {
            Evaluation::Variable(Err(test)) | Evaluation::Constant(Err(test)) => Some(*test),
            _ => None,
        }
    }
}

/// The outcome of one candidate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Outcome {
    Selected,
    SelectedByFallback,
    NoRecordFits,
}

/// The selected record and what sizing found for it.
#[derive(Debug, Clone, PartialEq)]
pub enum Selected {
    Variable { record_index: usize, k: f64, loads: VariableLoads },
    /// A constant-effort support carries W everywhere.
    Constant { record_index: usize, load_n: f64 },
}

impl Selected {
    pub fn record_index(&self) -> usize {
        match self {
            Selected::Variable { record_index, .. } | Selected::Constant { record_index, .. } => *record_index,
        }
    }
}

/// One candidate's row of the hanger table, before verification.
#[derive(Debug, Clone, PartialEq)]
pub struct CandidateResult {
    pub id: String,
    pub outcome: Outcome,
    pub selected: Option<Selected>,
    pub span: Span,
    pub required_travel_m: f64,
    /// Per tested family, the number of records whose first failed test is each test.
    pub variable_failures: Option<BTreeMap<Test, usize>>,
    pub constant_failures: Option<BTreeMap<Test, usize>>,
}

fn finite_positive(value: f64) -> bool {
    value.is_finite() && value > 0.0
}

/// E1 for a variable record: rate, range and travel present and valid. A
/// non-finite or non-positive quantity, or a range whose minimum is not below its
/// maximum, makes the record not sizable; the import boundary refuses such
/// records, and this keeps the arithmetic total if one reaches it.
fn sizable_range(record: &Record) -> Option<(f64, f64, f64)> {
    let (f_min, f_max) = record.working_load_range_n?;
    let travel = record.travel_range_m?;
    (finite_positive(f_min) && finite_positive(f_max) && f_min < f_max && finite_positive(travel))
        .then_some((f_min, f_max, travel))
}

/// The variation of each operating case, `k·|u_i − u_C| / R_i`. `None` where the
/// reference load is not positive, so no ratio is formed (Design §6.2).
fn variations(k: f64, positions: &Positions, f_operating: &[f64], w_n: f64, reference: VariationReference) -> Vec<Option<f64>> {
    positions
        .u_operating_m
        .iter()
        .zip(f_operating)
        .map(|(&u, &f)| {
            let r = match reference {
                VariationReference::OperatingLoad => f,
                VariationReference::DesignLoad => w_n,
            };
            (r > 0.0).then(|| k * (u - positions.u_cold_m).abs() / r)
        })
        .collect()
}

fn f_cold_for(candidate: &Candidate, k: f64, design: LoadDesign) -> f64 {
    let positions = &candidate.positions;
    match design {
        LoadDesign::Hot => candidate.w_n + k * positions.u_operating_m[candidate.design_index],
        LoadDesign::Cold => candidate.w_n + k * positions.u_cold_m,
    }
}

/// Evaluates one record for one candidate, applying the tests in order and
/// stopping at the first failure. This is the function the detail view lists
/// every record with (Design §8.1).
pub fn evaluate(candidate: &Candidate, record: &Record, method: &Method) -> Evaluation {
    let positions = &candidate.positions;
    let span = Span::of(positions);
    let a = method.travel_allowance_m;
    match record.hanger_type {
        HangerType::VariableSpring => Evaluation::Variable((|| {
            let k = record.stiffness_n_per_m.filter(|k| finite_positive(*k)).ok_or(Test::E1)?;
            let (f_min, f_max, travel) = sizable_range(record).ok_or(Test::E1)?;
            if record.stiffness_dof != Some(candidate.dof) {
                return Err(Test::E2);
            }
            let f_cold_n = f_cold_for(candidate, k, method.load_design);
            let f_cold_condition_n = f_cold_n - k * positions.u_cold_m;
            let f_operating_n: Vec<f64> = positions.u_operating_m.iter().map(|u| f_cold_n - k * u).collect();
            if !(f_cold_condition_n > 0.0) || f_operating_n.iter().any(|f| !(*f > 0.0)) {
                return Err(Test::V0);
            }
            let envelope_n = span.envelope_n(f_cold_n, k, a);
            if envelope_n.0 < f_min || envelope_n.1 > f_max {
                return Err(Test::L);
            }
            if span.required_travel_m(a) > travel {
                return Err(Test::T);
            }
            // V0 has made every operating load positive, and W is positive, so
            // every ratio is formed.
            let variation: Vec<f64> = variations(k, positions, &f_operating_n, candidate.w_n, method.variation_reference)
                .into_iter()
                .map(|v| v.expect("positive reference load after V0"))
                .collect();
            if variation.iter().any(|v| *v > method.variation_limit) {
                return Err(Test::V);
            }
            Ok(VariableLoads { f_cold_n, f_cold_condition_n, f_operating_n, variation, envelope_n })
        })()),
        HangerType::ConstantEffort => Evaluation::Constant((|| {
            let (f_min, f_max, travel) = sizable_range(record).ok_or(Test::E1)?;
            if candidate.w_n < f_min || candidate.w_n > f_max {
                return Err(Test::L);
            }
            if span.required_travel_m(a) > travel {
                return Err(Test::T);
            }
            Ok(())
        })()),
    }
}

/// The candidate ordering of the selection rule; `None` for a record that fails.
fn selection_key(record: &Record, evaluation: &Evaluation) -> Option<(f64, f64)> {
    let f_max = record.working_load_range_n.map(|(_, max)| max)?;
    match evaluation {
        Evaluation::Variable(Ok(loads)) => Some((f_max, loads.max_variation())),
        Evaluation::Constant(Ok(())) => Some((f_max, 0.0)),
        _ => None,
    }
}

fn rule_order(rule: SelectionRule, a: (f64, f64, &str), b: (f64, f64, &str)) -> Ordering {
    let (cap_a, var_a, id_a) = a;
    let (cap_b, var_b, id_b) = b;
    let first = match rule {
        SelectionRule::SmallestCapacity => cap_a.total_cmp(&cap_b).then(var_a.total_cmp(&var_b)),
        SelectionRule::LeastVariation => var_a.total_cmp(&var_b).then(cap_a.total_cmp(&cap_b)),
    };
    first.then_with(|| id_a.as_bytes().cmp(id_b.as_bytes()))
}

struct FamilyPass {
    best: Option<(usize, Evaluation)>,
    failures: BTreeMap<Test, usize>,
}

fn family_pass(candidate: &Candidate, records: &[Record], method: &Method, family: HangerType) -> FamilyPass {
    let tests: &[Test] = match family {
        HangerType::VariableSpring => &VARIABLE_TESTS,
        HangerType::ConstantEffort => &CONSTANT_TESTS,
    };
    let mut failures: BTreeMap<Test, usize> = tests.iter().map(|t| (*t, 0)).collect();
    let mut best: Option<(usize, Evaluation, (f64, f64))> = None;
    for (index, record) in records.iter().enumerate().filter(|(_, r)| r.hanger_type == family) {
        let evaluation = evaluate(candidate, record, method);
        if let Some(test) = evaluation.failed() {
            *failures.get_mut(&test).expect("family test") += 1;
            continue;
        }
        let key = selection_key(record, &evaluation).expect("a fitting record has a range");
        let better = match &best {
            None => true,
            Some((best_index, _, best_key)) => {
                rule_order(
                    method.selection_rule,
                    (key.0, key.1, &record.hanger_id),
                    (best_key.0, best_key.1, &records[*best_index].hanger_id),
                ) == Ordering::Less
            }
        };
        if better {
            best = Some((index, evaluation, key));
        }
    }
    FamilyPass { best: best.map(|(index, evaluation, _)| (index, evaluation)), failures }
}

fn to_selected(index: usize, evaluation: Evaluation, records: &[Record], candidate: &Candidate) -> Selected {
    match evaluation {
        Evaluation::Variable(Ok(loads)) => Selected::Variable {
            record_index: index,
            k: records[index].stiffness_n_per_m.expect("fitting variable record has a rate"),
            loads,
        },
        Evaluation::Constant(Ok(())) => Selected::Constant { record_index: index, load_n: candidate.w_n },
        _ => unreachable!("only fitting records are selected"),
    }
}

/// Sizes one candidate under its family policy (Design §6.5).
pub fn size_candidate(candidate: &Candidate, records: &[Record], method: &Method) -> CandidateResult {
    let span = Span::of(&candidate.positions);
    let mut result = CandidateResult {
        id: candidate.id.clone(),
        outcome: Outcome::NoRecordFits,
        selected: None,
        span,
        required_travel_m: span.required_travel_m(method.travel_allowance_m),
        variable_failures: None,
        constant_failures: None,
    };
    if matches!(candidate.family_policy, FamilyPolicy::Variable | FamilyPolicy::VariableElseConstant) {
        let pass = family_pass(candidate, records, method, HangerType::VariableSpring);
        result.variable_failures = Some(pass.failures);
        if let Some((index, evaluation)) = pass.best {
            result.outcome = Outcome::Selected;
            result.selected = Some(to_selected(index, evaluation, records, candidate));
            return result;
        }
    }
    if matches!(candidate.family_policy, FamilyPolicy::Constant | FamilyPolicy::VariableElseConstant) {
        let pass = family_pass(candidate, records, method, HangerType::ConstantEffort);
        result.constant_failures = Some(pass.failures);
        if let Some((index, evaluation)) = pass.best {
            result.outcome = match candidate.family_policy {
                FamilyPolicy::VariableElseConstant => Outcome::SelectedByFallback,
                _ => Outcome::Selected,
            };
            result.selected = Some(to_selected(index, evaluation, records, candidate));
        }
    }
    result
}

fn refusal(code: &'static str, names: Vec<String>, message: &str) -> Refusal {
    Refusal { code, names, message: message.to_string() }
}

/// Checks the method's values (Design §2).
pub fn check_method(method: &Method) -> Vec<Refusal> {
    let mut refusals = Vec::new();
    if !finite_positive(method.variation_limit) {
        refusals.push(refusal("HANGER_SIZING_VARIATION_LIMIT_INVALID", Vec::new(),
            "the variation limit must be a positive finite ratio"));
    }
    if !(method.travel_allowance_m.is_finite() && method.travel_allowance_m >= 0.0) {
        refusals.push(refusal("HANGER_SIZING_TRAVEL_ALLOWANCE_INVALID", Vec::new(),
            "the travel allowance must be a finite length of at least zero"));
    }
    refusals
}

/// Checks the method and the request's candidates and records. Every refusal
/// refuses the whole request, with every finding listed (Design §3).
pub fn check_request(candidates: &[Candidate], records: &[Record], method: &Method) -> Vec<Refusal> {
    let mut refusals = check_method(method);
    if candidates.is_empty() {
        refusals.push(refusal("HANGER_SIZING_CANDIDATES_REQUIRED", Vec::new(),
            "at least one candidate support is required"));
    }
    let mut seen = HashSet::new();
    let duplicates: Vec<String> = candidates.iter().filter(|c| !seen.insert(c.id.as_str())).map(|c| c.id.clone()).collect();
    if !duplicates.is_empty() {
        refusals.push(refusal("HANGER_SIZING_CANDIDATE_DUPLICATE", duplicates,
            "each candidate support may be named once"));
    }
    let mut seen = HashSet::new();
    let duplicates: Vec<String> = records.iter().filter(|r| !seen.insert(r.hanger_id.as_str())).map(|r| r.hanger_id.clone()).collect();
    if !duplicates.is_empty() {
        refusals.push(refusal("HANGER_SIZING_LIBRARY_RECORD_DUPLICATE", duplicates,
            "hanger_id values must be unique, because the selection breaks ties by hanger_id"));
    }
    let no_design: Vec<String> = candidates
        .iter()
        .filter(|c| c.design_index >= c.positions.u_operating_m.len())
        .map(|c| c.id.clone())
        .collect();
    if !no_design.is_empty() {
        refusals.push(refusal("HANGER_SIZING_DESIGN_OPERATING_CASE_REQUIRED", no_design,
            "each candidate needs a position in the design operating case"));
    }
    let non_finite: Vec<String> = candidates
        .iter()
        .filter(|c| {
            !c.w_n.is_finite()
                || !c.positions.u_cold_m.is_finite()
                || c.positions.u_operating_m.iter().any(|u| !u.is_finite())
        })
        .map(|c| c.id.clone())
        .collect();
    if !non_finite.is_empty() {
        refusals.push(refusal("HANGER_SIZING_DERIVED_SOLVE_FAILED", non_finite,
            "a derived solve gave a non-finite reaction or displacement at a candidate"));
    }
    let not_positive: Vec<String> = candidates.iter().filter(|c| c.w_n <= 0.0).map(|c| c.id.clone()).collect();
    if !not_positive.is_empty() {
        refusals.push(refusal("HANGER_SIZING_LOAD_NOT_POSITIVE", not_positive,
            "the rigid reaction at each named candidate is not positive along its DOF; sizing never flips a sign, so remove the candidate and run again"));
    }
    refusals
}

/// Sizes every candidate, or refuses the request with every finding listed.
pub fn size(candidates: &[Candidate], records: &[Record], method: &Method) -> Result<Vec<CandidateResult>, Vec<Refusal>> {
    let refusals = check_request(candidates, records, method);
    if !refusals.is_empty() {
        return Err(refusals);
    }
    Ok(candidates.iter().map(|c| size_candidate(c, records, method)).collect())
}

/// The verified values of a variable selection (Design §7).
#[derive(Debug, Clone, PartialEq)]
pub struct VerifiedVariable {
    /// F_C^v = F_cold − k·u_C^v.
    pub f_cold_condition_n: f64,
    /// F_i^v = F_cold − k·u_i^v.
    pub f_operating_n: Vec<f64>,
    /// v_i^v = k·|u_i^v − u_C^v| / R_i^v; `None` where the reference load is not positive.
    pub variation: Vec<Option<f64>>,
    /// F_D^v − W (hot design) or F_C^v − W (cold design).
    pub departure_n: f64,
}

/// One candidate's verification (Design §7).
#[derive(Debug, Clone, PartialEq)]
pub struct Verification {
    pub positions: Positions,
    pub span: Span,
    pub required_travel_m: f64,
    pub t_pass: bool,
    pub variable: Option<VerifiedVariable>,
    /// Every failing test, formed with the verified values: V0, L, T and V for a
    /// variable selection, T for a constant one.
    pub failed_tests: Vec<Test>,
}

/// Verifies one sized candidate from its verified positions. A candidate with
/// no fit has no verification; it stays a force W in the verification solves,
/// which makes the request's verification conditional (`verification_standing`).
pub fn verify(
    candidate: &Candidate,
    result: &CandidateResult,
    records: &[Record],
    method: &Method,
    verified: &Positions,
) -> Result<Option<Verification>, Refusal> {
    let Some(selected) = &result.selected else { return Ok(None) };
    if verified.u_operating_m.len() != candidate.positions.u_operating_m.len()
        || !verified.u_cold_m.is_finite()
        || verified.u_operating_m.iter().any(|u| !u.is_finite())
    {
        return Err(refusal("HANGER_SIZING_DERIVED_SOLVE_FAILED", vec![candidate.id.clone()],
            "a verification solve gave a non-finite or missing displacement at a candidate"));
    }
    let a = method.travel_allowance_m;
    let span = Span::of(verified);
    let required_travel_m = span.required_travel_m(a);
    let record = &records[selected.record_index()];
    let travel = record.travel_range_m.expect("selected record has a travel range");
    let t_pass = required_travel_m <= travel;
    let mut failed_tests = Vec::new();
    let variable = match selected {
        Selected::Constant { .. } => {
            if !t_pass {
                failed_tests.push(Test::T);
            }
            None
        }
        Selected::Variable { k, loads, .. } => {
            let k = *k;
            let f_cold = loads.f_cold_n;
            let f_cold_condition_n = f_cold - k * verified.u_cold_m;
            let f_operating_n: Vec<f64> = verified.u_operating_m.iter().map(|u| f_cold - k * u).collect();
            let variation = variations(k, verified, &f_operating_n, candidate.w_n, method.variation_reference);
            let departure_n = match method.load_design {
                LoadDesign::Hot => f_operating_n[candidate.design_index] - candidate.w_n,
                LoadDesign::Cold => f_cold_condition_n - candidate.w_n,
            };
            let (f_min, f_max) = record.working_load_range_n.expect("selected record has a range");
            if !(f_cold_condition_n > 0.0) || f_operating_n.iter().any(|f| !(*f > 0.0)) {
                failed_tests.push(Test::V0);
            }
            let envelope = span.envelope_n(f_cold, k, a);
            if envelope.0 < f_min || envelope.1 > f_max {
                failed_tests.push(Test::L);
            }
            if !t_pass {
                failed_tests.push(Test::T);
            }
            if variation.iter().flatten().any(|v| *v > method.variation_limit) {
                failed_tests.push(Test::V);
            }
            Some(VerifiedVariable { f_cold_condition_n, f_operating_n, variation, departure_n })
        }
    };
    Ok(Some(Verification { positions: verified.clone(), span, required_travel_m, t_pass, variable, failed_tests }))
}

/// Whether the verification describes a configuration that applying the
/// selections reproduces (Design §7).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerificationStanding {
    Unconditional,
    /// At least one candidate had no fit and was verified as a force W.
    Conditional,
}

pub fn verification_standing(results: &[CandidateResult]) -> VerificationStanding {
    if results.iter().any(|r| r.outcome == Outcome::NoRecordFits) {
        VerificationStanding::Conditional
    } else {
        VerificationStanding::Unconditional
    }
}

#[cfg(test)]
mod tests;
