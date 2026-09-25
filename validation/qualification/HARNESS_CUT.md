# Smallest next harness cut

Build one required-output gate around the existing production `solve` route.
ROOT has selected this route for the next implementation slice after the current
mapping review/freeze. Do not replace the solver, the current result schemas, the regression suites,
or the PKG-14 diagnostic comparison engine. This is a concrete next-cut proposal
with ROOT allocation recorded; no implementation or run is performed by this mapping.

The primary homes are DEL-10-05 for execution, DEL-14-02/04/05 for immutable run
and comparison contracts, and DEL-09-04/05 for the profile assessment. Production
semantics remain owned by the solver/result managers. Source acquisition and
oracle derivations remain independent. The first gate does not need a new
application framework, generic model format, or global tolerance default.

## Reusable interfaces and actual gaps

| Existing surface | Reuse | Gap to close in the thin gate |
|---|---|---|
| `openpipestress-runner solve --input` | Existing `RunnerRequest` plus `solve.preview_model`, actual raw `MechanicsEnvelope` | stdout is `ControlledExport`, not bare results. Inspect blocked/null payload, validators, operation/run/model/status and diagnostics; exit0 alone is insufficient. |
| `benchmark_binding.rs` | Suite-owned fixture accessors and exact existing regression predicates | Counts denominate returned cases, explicit duplicates are accepted, and case errors can lose expected value rows. Preserve regression evidence; do not pretend it is a complete profile. |
| `MechanicsEnvelope.results` / exact semantic tables | Existing kind/entity/basis/component/frame/location/sign/unit fields | Resolve every required selector exactly once. Raw `id/value` is not PKG-14's `result_id/magnitude` envelope; silent projection would yield zero comparisons. |
| Existing audit manifest / AnalysisRun identity | Actual model/run/received-value/semantic hashes and structured provenance | Capture real executable digest, revision/dirty state, arguments/environment, raw stdout/stderr/exit and profile/reference/criterion identities too. Opaque same-solve proof is library-only; parsed CLI JSON cannot manufacture it. |
| PKG-14 diagnostic comparator | Optional diagnostic deltas and mapping/tolerance metadata | Auto-map is intersection-only; missing/unmapped rows do not form a required denominator. Current classification is a symmetric maximum rule, not the supplied report's additive reference-sided proposal. Do not silently change it. |

Source tests encode mechanics25cases/206values, stress15cases with3blocked,
and nonlinear5cases including an expected-nonconvergence regression. Those are
source expectations, not observations from this mapping. An expected diagnostic
regression may pass its purpose without being a successful physical-solution
case. `ExecutionFailed` currently preserves the case but returns `values: []`;
required quantities must exist before execution. The existing CLI has no durable
per-case interruption ledger or atomic run completion. These limitations motivate
the thin gate, not a claim that the existing diagnostic tools are broken merely
because they do not implement a qualification programme.

## Owned implementation and sequencing

The proposed writer owns a small module/script under the existing
`tools/validation/` and its focused tests, plus the selected case/reference
bindings under `validation/qualification/`. It reads existing product interfaces;
no product/schema change is implicit. If a needed metadata/producer seam is absent,
return the concrete change to its existing owner before editing it.

1. Define a minimal locked case manifest that references existing production
   request files, independent reference artifacts, exact output selectors and
   governing predicate identities. It is a case selection/obligation record,
   not another model/result schema. Require nonzero unique case and assertion
   IDs; preserve reference readiness separately. Do not accept unresolved
   reference or criterion records as ready cases.
2. Materialize the complete required case/quantity ledger before invoking the
   existing CLI. Each row retains its obligation even when the executable is
   unavailable, a solve refuses, output is truncated, a timeout occurs or a case
   is interrupted. Missing values stay unavailable; they never become zero.
3. Run a single foreground local process with explicit executable/input/output
   locations, bounded execution and retained raw bytes. No network, runtime
   building, source fetch, hidden reference regeneration or home scanning.
   Record command, exit/signal, actual artifact hash and candidate identity.
4. Read the controlled wrapper and exact-version raw envelope. Each required
   selector must resolve once with matching case/entity/kind/component/frame/
   location/sign/unit/definition. Reject duplicates, nonfinite inputs and
   arithmetic overflow during conversion/subtraction, unexpected required-case
   substitution, and insufficient method standing. Preserve extra raw rows;
   an explicit manifest rule defines what is outside the comparison scope.
5. Call or bind the existing governed predicate for that assertion. Preserve
   historical suite accessors for regression evidence. For new production
   targets, independently establish the appropriate criterion before admitting
   the case; unmeasured budgets remain unresolved. No universal `1e-9`, no
   transplanted report sample tolerance, and no unreviewed conversion policy.
6. Generate JSON and a short Markdown assessment from the same full ledger,
   showing required, executed, matched, failed, blocked, error and not-run
   counts/IDs separately. Required work passes only when all obligations are
   present and satisfied with matching build/input/reference/criterion identity.
   App Current, fresh execution, physical validity and release remain distinct.

First implement/test the gate using bounded synthetic comparator controls and
already captured source-identified raw packets; this tests the harness only.
Then bind a genuine axial and a bending/torsion production case from the
selected static profile after independent reference/selector/criterion freeze.
This is a small implementation batch with one full review, not a requirement
for per-function freezes. Necessary mechanics/interface repairs can proceed
alongside the harness; never make the production solver write its own target.

## Required checks for that cut

The future focused checks must demonstrate a correct complete record, wrong
value/sign, missing required case/quantity, duplicate case/selector, wrong
loadcase/frame/unit, finite-input arithmetic overflow, nonnumeric/nonfinite/null,
empty manifest, malformed/truncated output, no output/nonzero exit/interruption,
and changed input/reference/build/manifest identity. Expected nonconvergence
covers only the labelled diagnostic case. Reference-target files must remain
unchanged after ordinary execution. A real run must produce the same complete
ledger shape as a failed run. These are concrete acceptance obligations; none
has been executed by this preparation.

The open decisions are exact first case bindings and per-quantity criteria where
not already owned, the selected source/semantic version for the new engine,
and eventual distributed-build/native/live/external profile obligations. A
licensed engine is not required for the initial open analytical path. Dynamics
and measured experiments need separate production/data homes. Required basic
static mechanics remain explicit even when their first cases are blocked.

Mode dependency: the current CLI implicitly dispatches sparse_interactive. Dense
scrutiny exists in the library; a closed --solver-mode extension belongs to the
physics headless/context join and is not yet an executed interface. The first
gate is sparse-only and must record/refuse mode mismatches. Consume the future
flag only against its actual reviewed candidate and result binding.
