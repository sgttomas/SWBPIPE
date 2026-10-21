//! R-A's gating test (DEL-04-08 Design `source-work-nondyadic-inputs.md` §3.2): every row
//! that B1 or B2 takes equals the long evaluation's Ratio bit for bit. For one request and
//! mode, the direct method's own steps run to its plan, then the same response is evaluated
//! twice on one ledger: with R-A (the product path) and through the test-only long entry
//! (`Response::evaluate_functionals_long`). The row trace (`exact_operation_count::
//! take_row_paths`) records which rows took B1 and how many terms B2 skipped, so a
//! byte-equality pass cannot hide a silent fallback. Every model is invented.
//! Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
use super::*;
use open_pipe_stress_frame_kernel::structural::exact_boundary::exact_operation_count::{self as count, RowPath};

/// One gated evaluation.
#[derive(Debug)]
pub(crate) struct Gate {
    /// The descriptors' quantities, in order (`MemberSection`'s component is kept).
    pub kinds: Vec<String>,
    pub paths: Vec<RowPath>,
    /// Indices of rows whose Ratio differs from the long evaluation's.
    pub mismatches: Vec<usize>,
}
impl Gate {
    pub(crate) fn negated(&self) -> usize {
        self.paths.iter().filter(|p| **p == RowPath::Negated).count()
    }
    pub(crate) fn b2_skipped(&self) -> u64 {
        self.paths.iter().map(|p| match p { RowPath::Long { b2_skipped } => u64::from(*b2_skipped), RowPath::Negated => 0 }).sum()
    }
}

/// Gates one request's first load case, or `None` when the method declines before its
/// plan (no evaluation to compare). Both evaluations must complete, on one ledger raised
/// to the invocation limit; a refusal on either side is an error.
pub(crate) fn gate(raw: &Value, mode: PreviewSolverMode) -> Option<Result<Gate, String>> {
    let owned = Owned::actual(raw, mode).ok()?;
    let input = owned.input();
    // Test-only: both evaluations share one ledger, so it is raised to the invocation limit.
    let mut budget = AttemptBudget::new(exact::Limits { operations: 64_000_000, ..Default::default() });
    let source = prepare_sources(&input, &mut budget).ok()?;
    let context = exact::Context::prepare_with_budget(
        &source.system(&input),
        &source.identity,
        exact::ForceBasis::IdentifiedContributions(&source.force_terms),
        &source.congruence,
        &mut budget,
    )
    .result
    .ok()?;
    let response = context.solve_with_budget(&mut budget).result.ok()?;
    let plan = FunctionalPlan::new_with_hints(&context, &source.descriptors, &|index| source.hint(index), &mut budget).result.ok()?;
    let _ = count::take_row_paths();
    let short = response.evaluate_functionals(&plan, &mut budget).result;
    let mut paths = count::take_row_paths();
    let long = response.evaluate_functionals_long(&plan, &mut budget).result;
    let _ = count::take_row_paths();
    let kinds = source.descriptors.iter().map(|d| match &d.key.quantity {
        FunctionalQuantity::MemberSection { component, station_bits, .. } => format!("section:{component}:{}", f64::from_bits(*station_bits)),
        other => format!("{other:?}").split([' ', '{']).next().unwrap_or_default().to_owned(),
    }).collect();
    Some(match (short, long) {
        (Ok(a), Ok(b)) => Ok(Gate {
            kinds,
            paths: paths.pop().unwrap_or_default(),
            mismatches: (0..a.ratios().len()).filter(|&i| a.ratios()[i] != b.ratios()[i]).collect(),
        }),
        (a, b) => Err(format!("shortcut {:?} against long {:?}", a.err(), b.err())),
    })
}

/// R-A's change to one attempt's counted work (Design §3.1 "As built", §4), measured
/// against the pre-R-A path on `input`: the direct method's steps run to its plan on one
/// ledger; the plan is then formed with B1's hints and without them, and the response is
/// evaluated by the product path (hinted plan, B2 active) and by the test-only long entry
/// (plain plan, B1 and B2 off), which is the pre-R-A evaluation. The two evaluations must
/// agree bit for bit.
#[derive(Debug, Clone, Copy)]
pub(crate) struct RaWork {
    /// The descriptor count: the hint rule is charged and applied once per descriptor.
    pub hints: i64,
    /// The hinted plan's charge minus the plain plan's.
    pub plan: i64,
    /// The product evaluation's charge minus the long evaluation's.
    pub evaluation: i64,
}

pub(crate) fn ra_work(input: &Input<'_>) -> Result<RaWork, String> {
    let mut budget = AttemptBudget::new(exact::Limits { operations: 64_000_000, ..Default::default() });
    let source = prepare_sources(input, &mut budget).map_err(|e| format!("source closure: {e:?}"))?;
    let context = exact::Context::prepare_with_budget(
        &source.system(input),
        &source.identity,
        exact::ForceBasis::IdentifiedContributions(&source.force_terms),
        &source.congruence,
        &mut budget,
    )
    .result
    .map_err(|e| format!("context: {e:?}"))?;
    let response = context.solve_with_budget(&mut budget).result.map_err(|e| format!("solve: {e:?}"))?;
    let charged = |b: &AttemptBudget| b.report().charged as i64;
    let c0 = charged(&budget);
    let hinted = FunctionalPlan::new_with_hints(&context, &source.descriptors, &|index| source.hint(index), &mut budget)
        .result
        .map_err(|e| format!("hinted plan: {e:?}"))?;
    let c1 = charged(&budget);
    let plain = FunctionalPlan::new(&context, &source.descriptors, &mut budget).result.map_err(|e| format!("plain plan: {e:?}"))?;
    let c2 = charged(&budget);
    let short = response.evaluate_functionals(&hinted, &mut budget).result.map_err(|e| format!("evaluation: {e:?}"))?;
    let c3 = charged(&budget);
    let long = response.evaluate_functionals_long(&plain, &mut budget).result.map_err(|e| format!("long evaluation: {e:?}"))?;
    let c4 = charged(&budget);
    if short.ratios() != long.ratios() {
        return Err("the product evaluation differs from the long evaluation".into());
    }
    Ok(RaWork { hints: source.descriptors.len() as i64, plan: (c1 - c0) - (c2 - c1), evaluation: (c3 - c2) - (c4 - c3) })
}

/// §3.2's gating test over the J7 RF census and the J2 corpus's genuine source
/// invocations (the 24 raw source controls and the RF bases), in both modes. 86 of these
/// evaluations reach a plan at this revision; the load/reference-state controls do not, since
/// J7's direct method builds no load state, and are covered by their committed bytes.
#[test]
fn source_negated_rows_equal_the_long_evaluation() {
    let mut requests: Vec<(String, Value, PreviewSolverMode)> = Vec::new();
    for case in inputs()["cases"].as_array().unwrap() {
        for mode in MODES {
            requests.push((format!("J7 {} {}", case["id"].as_str().unwrap(), mode.as_str()), current_request(case), mode));
        }
    }
    let corpus = corpus_storage::expand(serde_json::from_str(J2).unwrap());
    for entry in corpus["entries"].as_array().unwrap() {
        let id = entry["id"].as_str().unwrap();
        if id.ends_with(":genuine") && entry["invocation"].is_object() {
            let mode = if entry["invocation"]["solver_mode"] == "dense_scrutiny" { PreviewSolverMode::DenseScrutiny } else { PreviewSolverMode::SparseInteractive };
            requests.push((format!("J2 {id}"), entry["invocation"]["request"].clone(), mode));
        }
    }
    let (mut gated, mut negated, mut skipped) = (0usize, 0usize, 0u64);
    for (label, raw, mode) in &requests {
        let Some(gate) = gate(raw, *mode) else { continue };
        let gate = gate.unwrap_or_else(|e| panic!("{label}: {e}"));
        assert!(gate.mismatches.is_empty(), "{label}: rows {:?} differ from the long evaluation", gate.mismatches);
        assert_eq!(gate.paths.len(), gate.kinds.len(), "{label}: one recorded path per row");
        for (path, kind) in gate.paths.iter().zip(&gate.kinds) {
            if *path == RowPath::Negated {
                assert!(kind.starts_with("section:"), "{label}: B1 on a {kind} row");
            }
        }
        gated += 1;
        negated += gate.negated();
        skipped += gate.b2_skipped();
    }
    eprintln!("R-A gating: {gated} evaluations, {negated} rows by B1, {skipped} terms skipped by B2");
    // Exactly the 86 evaluations that reach a plan, so none can drop out of the gate
    // unnoticed (Opus review, round 1).
    assert!(gated == 86 && negated > 0 && skipped > 0, "the gate must cover all 86 evaluations and exercise both parts: {gated} {negated} {skipped}");
}
