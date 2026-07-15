<!-- chirality-architecture-basis/v1 -->
# DEL-00-08 — Layered software test and acceptance strategy — Architecture Basis

Classification: `ARCHITECTURE_BASIS_REFERENCE`
Consolidates: `Datasheet.md`, `Specification.md`, `Procedure.md`, `Guidance.md` (four-document kit, consolidated 2026-07-15 per piping decision D-43; source bytes hash-bound in the package `CONSOLIDATION_MANIFEST.md` and preserved in git history)
Authority basis: `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.9, architecture-basis row AB-00-08
Dependency direction: one-way — project packages depend on this member; this member depends on no package deliverable (HUMAN-STEER-PKG00-EXCLUSION-001)

## Purpose

Layered software test strategy and architecture acceptance gates for the software architecture runway (scope SOW-063, objective OBJ-013, type TEST_SUITE). This member defines test architecture and acceptance gates only; it does not itself implement tests, CI jobs, benchmarks, solvers, GUI tests, or packaging automation. Per the PDU-054 current declaration (D-41 R5 T7), the layered test strategy is now the current verification basis for implemented slices; passing verification evidence is not a validation ruling, release approval, or lifecycle transition. Architecture roles covered: architecture gate matrix, schema test layer, service contract test layer, solver verification layer, GUI workflow layer, report/security/package gate.

## Normative requirements

| ID | Requirement | Evidence |
|---|---|---|
| REQ-08-01 | Define layered tests for architecture, schemas, units, services, solver mechanics, load/stress recovery, rule packs, GUI, CLI, reports, adapters, packaging, security, and regressions. | Acceptance review |
| REQ-08-02 | Require solver and rule-engine implementation deliverables to include deterministic verification before release use. | Acceptance review |
| REQ-08-03 | Require public examples, reports, templates, and tests to pass protected-content and provenance checks where applicable. | Acceptance review |
| REQ-08-04 | Define architecture acceptance gates for PKG-00 readiness without claiming product release quality. | Acceptance review |
| REQ-08-05 | Record CI and tooling choices as TBD unless a human ruling is cited. | Human review |

REQ-08-04 remains normative as stated: architecture acceptance gates do not claim product release quality. Release-quality claims are governed separately (`docs/RELEASE_QUALITY_GATES.md` and its human gates).

### Required invariants

- `OPS-K-IP-1`: Public artifacts must not contain protected standards text, tables, figures, copied formulas, material allowables, SIF/flexibility tables, protected dimensional tables, or proprietary commercial data.
- `OPS-K-DATA-2`: Missing solve-required or rule-check-required values remain explicit findings, never silent defaults.
- `OPS-K-AUTH-1`: Software and agents must not claim to certify, seal, approve, authenticate, or declare engineering code compliance for reliance.
- `OPS-K-MECH-1`: Global analysis architecture remains a 3D centerline/frame model; local FEA is a handoff path.
- `OPS-K-AGENT-1`: Unknown engineering or architecture facts become `TBD`.
- `OPS-K-AGENT-3`: Type 2 execution stays within sealed deliverable scope.

### Interpretation guidance

- Map each quality obligation from CONTRACT/SPEC to a test layer and acceptance-evidence type; separate mechanics verification from user rule checking and from code compliance and professional approval.
- Prefer explicit contracts over package-local assumptions; treat diagnostics, provenance, units, and data-boundary checks as cross-cutting obligations.
- Record a choice as `TBD` when no cited human ruling exists; record `PROPOSAL` only when explicitly framed for review; downstream packages cite the accepted decision record or note the awaiting ruling.
- Boundary: this member scopes PKG-00 architecture only. It does not advance PKG-01 through PKG-17 work, does not claim code compliance or professional approval, and introduces no protected or proprietary content.

## Resolved decisions (former TBD and human-ruling queue)

The kit's original TBD slots (CI platform, test runner, coverage thresholds, GUI automation tooling, release matrix) have been resolved by cited human rulings recorded in `execution/_Coordination/_DECISIONS/_REGISTER.md` and codified in SOFTWARE_DECOMP.md §12:

- **CI platform:** hosted CI deferred; the five-surface local evidence sweep is the commit-bound merge gate (with the F-4 atomic-build rider) — D-05 ruling, codified as `DEC-025`. Public sanitized-export repo CI (GitHub Actions) activates conditionally at first public publication behind the export pipeline — D-05b ruling, codified as `DEC-059`.
- **Coverage tooling and thresholds:** `cargo-llvm-cov` / Vitest `--coverage` / `coverage.py`, telemetry recorded and never blocking; floor promotion only via the ruled trigger mechanism, no floor values set — D-04 ruling (`DEC-026`, class-tiered governed tolerance pairs) and D-04b ruling (`DEC-060`).
- **Release matrix and installer/publication posture:** D-06 ruling, codified as `DEC-057` (including the `.opsproj` naming rider and evidence-gated matrix expansion).
- **Convergence tolerances:** class-tiered convergence tolerance record by nonlinear support class with relative+absolute residual thresholds and iteration caps; unmeasured entries stay `TBD` — D-19 ruling, codified as `DEC-046`.

## Realized artifacts

| Anticipated (setup era) | Realized | Owner |
|---|---|---|
| `docs/architecture/test_strategy.md` | Never created under that name. The layered-test architecture is realized in the configured harness: `projects/chirality-piping/software-workflow.json` and `tools/release/run_evidence_sweep.py` (five-surface sweep), with on the order of 75–100 test files spanning schemas, security, GUI, and adapters (plus inline Rust test modules) | Release/evidence tooling |
| Acceptance gate matrix | Never created under that name. Realized as `docs/RELEASE_QUALITY_GATES.md` | Release governance docs |

The realized layered-test harness matches this strategy and, per the PDU-054 declaration, is the current verification basis for implemented slices.

## Open holds and routed questions

- No former kit TBD slot remains open: CI posture, coverage tooling/thresholds, release matrix, and convergence tolerances are all ruled (above). GUI automation tooling is exercised in the realized harness (Playwright/Vitest surfaces of the DEC-025 sweep).
- Coverage floors remain deliberately unset: values are recorded-not-blocking until the `DEC-060` floor-promotion trigger is met; unmeasured `DEC-046` convergence entries stay `TBD`.
- Any release-quality, validation, lifecycle, or professional/code-compliance claim remains a human ruling outside this reference.

## Currency and provenance

Consolidated 2026-07-15 per piping decision D-43 from the four-document kit (`Datasheet.md`, `Specification.md`, `Procedure.md`, `Guidance.md`) as reconciled by D-41 R5 T7 (PDU-054 currentness declarations). Prior wording, including setup-era framing, is preserved in git history; `MEMORY.md` is retained unchanged. Current upstream authority is SOFTWARE_DECOMP revision 0.9 with DAG-007 coordination.
