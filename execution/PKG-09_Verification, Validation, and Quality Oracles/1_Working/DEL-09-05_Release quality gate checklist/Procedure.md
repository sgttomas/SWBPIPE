# Procedure: DEL-09-05 Release quality gate checklist

## Purpose
Define the operational procedure for applying release quality gates to solver changes, rule-engine changes, GUI releases, and report-template releases.

## Prerequisites
- The release candidate scope is identified by changed packages, deliverables, source paths, or artifacts.
- The relevant benchmark, regression, validation, GUI, rule-engine, report, and protected-content checks are available or explicitly marked `TBD`.
- Benchmark sources and public examples have documented provenance and redistribution status.
- Any suspected protected or private data is quarantined before gate evidence is assembled.
- Thresholds not approved by human authority remain `TBD`; this procedure does not invent them.

## Steps
1. Classify the release candidate.
   - Mark all applicable families: solver, rule-engine, GUI, report-template, or mixed.
   - If classification is uncertain, route to the broader family set and record the uncertainty.
   - If a change is mixed, run the union of applicable gate families unless a human release-authority waiver is recorded with risk disposition.

2. Assemble the common evidence bundle.
   - Record commit/artifact identifiers, software version, solver version if applicable, model or fixture hashes where applicable, commands run, results, warnings, open risks, and unresolved `TBD` decisions.
   - Record provenance for benchmark sources, public examples, report templates, and rule-pack references.
   - Record that governance acceptance is not professional engineering approval.

3. Apply solver-change gates when routed.
   - Verify mechanics benchmarks required for the affected solver behavior.
   - Verify stress-recovery benchmarks when stresses, resultants, or report-facing stress values are affected.
   - Verify nonlinear support convergence/state traces when nonlinear behavior is affected.
   - Verify unit-aware calculations, deterministic repeatability, numerical diagnostics, and warning/result-envelope behavior.
   - Keep final tolerances and performance thresholds as `TBD` unless a human ruling exists.

4. Apply rule-engine gates when routed.
   - Verify sandboxing and arbitrary-code exclusion.
   - Verify unit-aware expression handling and deterministic evaluation.
   - Verify missing required inputs produce explicit findings, not silent defaults.
   - Verify invented public examples remain non-code and protected-content free.
   - Verify rule-pack identity, version, checksum, source note, and public/private marking are preserved.

5. Apply GUI release gates when routed.
   - Verify visible distinction among solve-blocking missing data, rule-check-blocking missing data, provenance warnings, assumption warnings, nonlinear warnings, and IP-boundary warnings.
   - Verify workflows do not present mechanics solved, user-rule checked, and professional approval as the same state.
   - Verify expected model editing, solve execution, result viewing, warning, and report-preview/export workflows for the release scope.
   - Verify accessibility/usability evidence where GUI surfaces are affected, with detailed thresholds remaining `TBD` unless approved elsewhere.

6. Apply report-template gates when routed.
   - Verify report reproducibility and checksum/hash stability.
   - Verify warning, assumption, limitation, provenance, rule-pack reference, and professional-boundary notices are present.
   - Run or require protected-content lint for public templates/examples.
   - Record the protected-content lint command/tool as `TBD` until DEL-10-04 or a human release-governance ruling supplies the automation detail.
   - Confirm public templates do not embed protected standards text, copied tables, protected formulas, proprietary data, or certification/compliance claims.

7. Review open risks and unresolved decisions.
   - List all open risks, `TBD` thresholds, missing evidence, failing checks, and protected-data concerns.
   - A release may not proceed when required evidence is missing unless the human release authority explicitly accepts that risk for the relevant maturity label.

8. Record the gate outcome.
   - Use `PASS` only when required evidence is present and all applicable gates pass.
   - Use `FAIL` when evidence exists and shows a gate failure.
   - Use `BLOCKED_TBD` when a necessary threshold, authority decision, or required check is unresolved.
   - Use `HUMAN_REVIEW_REQUIRED` when professional-boundary, protected-content, private-data, or release-label questions require human disposition.

## Verification
- Confirm the final checklist covers solver, rule-engine, GUI, and report-template gate families.
- Confirm the procedure does not modify CI workflows, tests, release files outside this deliverable, or repo-level artifacts.
- Confirm no final thresholds are asserted without human authority.
- Confirm no protected standards text, protected examples, copied formulas, allowables, SIF/flexibility tables, proprietary values, or private user data are introduced.
- Confirm no software or agent output claims certification, sealing, endorsement, official compliance, or professional approval.
- Confirm setup artifacts include semantic matrices, semantic lensing, dependency artifacts, run records, and `SEMANTIC_READY` status only after validation passes.

## Records
Preserve these records in the release gate evidence bundle or deliverable-local setup record as applicable:
- release-gate checklist result;
- commands and outputs for relevant deterministic checks;
- benchmark/regression fixture provenance;
- protected-content/provenance review notes;
- open risk and `TBD` decision log;
- human maintainer governance acceptance record, including validation status, known limitations, open risks, unresolved `TBD` decisions, and professional-boundary notice;
- statement that professional reliance requires competent human review.
