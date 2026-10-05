# I30 routing derivation 02 — proposed finite control contract

Revisable return for RV41 backcheck; no code, byte allowance or runtime qualification.
Receipt 2026-10-02 19:49:03 UTC; analysis cutoff 20:24:03; hard return 20:34:03.
TASK `/root/i30_f2a_checkpoint0` under ROOT `/root`, native follow-up; no delegation.
P=`projects/chirality-piping`, PP=P/core/product_physics, HL=P/core/runner/headless.
Source main49034a940f; supplied records through NUM6a964324bc. See _run_records.

## 1. Fixed choices and scope

ROOT's latest RV41 disposition selects M1 only as the derivation basis, typed bypass,
faithful S5-R, source-based mixed combinations, and specific native unsafe-custody
refusal. It grants no native inspection/Current/export state or ordinary withholding.
This packet resolves the routing/control disagreement and enumerates native owners.
I29's P1–P5 arithmetic and the finite native reply profile remain prerequisites to
implementation; naming these interfaces does not supply their byte coefficients.

The two ledgers stay distinct: legacy SourceRecoveryBudget (4M or 8M case, 64M
invocation, current units/precedence) and W1 InvocationMeter (20B/60B LME stop
thresholds). Newly reached ordinary/W2 work has neither of those LME prices.
No ledger reset, refund, replay of a prefix solve, or repeated solve to rebuild a
fallback is proposed. Existing internal W2 evaluations and exact-source replay
remain their existing algorithms, called once at their existing phase boundaries.

## 2. Owned state sufficient to preserve the legacy result

| Proposed private state | Contents, owner and invariant |
|---|---|
| RouteState | Fixed discriminant: LegacyPrefix, ExactBypass, DeferredSuffix, ReadyW1, Draft, Complete. Case cursor and exact_selected_ever latch. No source batch is built during coexistence arbitration. |
| LegacyTerminal | First terminal case/index and branch tag (blocked_envelope or solver_blocked), original typed error if that branch needs it, immutable diagnostic prefix, normalized model identity and fixed SourceBudgetSnapshot. Keep model owned once by the invocation; do not clone it to make a fallback. |
| FailedOrdinarySeed | Case/basis index and mode; initial OrdinaryFailure (StructuralError or deferred NumericalRange); optional RangeTrigger and actual ForceScalingFailure; load-row finding/diagnostic reference; exact-source disposition and actual spent-work reference. Move existing typed payloads, including direction vectors, after their last legacy use. Never parse a Display string to recover error type or witness. |
| Per-case ordinary seed | Compact live-derived Passed/Sensitive/rejected/not-attempted facts, diagnostic span and case/basis identity. It is input to I32's faithful ordinary-reference projection, not a rewritten successful report. A failed pre-W2 formation is not a solve that ran. |
| RoutingAudit | Bounded private counters: prefix snapshot, actual legacy-source counters, newly continued cases/bases/ordinary phases/W2 calls/source attempts, admission-stop reason and W1 meter. No public envelope field or unbounded trace is required. |

`solve_load_case` currently returns failed empty-row values before OrdinaryAttempt
is built (PP lib.rs:3739–3810). Capture the seed at the actual error branch, before
returning/moving its last typed error. A deferred Formation failure and a structural
failure must remain distinguishable. The existing receipt helper rejected() only
accepts StructuralError; do not fabricate that variant for a formation failure.
I32 must define the corresponding ordinary-reference projection from this seed.

The prefix diagnostics can be **moved** with mem::take into LegacyTerminal when
continuation is admitted. Use a separate admitted suffix diagnostic vector: passing
the old blocking prefix to the next solve would make has_blocking return immediately.
The prefix is never edited to enable continuation. On original fallback, move it
through the original blocked_envelope/solver_blocked construction exactly once;
keep its preview sanitization, diagnostic ordering, error text and quality assessment.
For successful W1 replacement, build a separately admitted diagnostic projection
with ordinary failure evidence intact; only its selected-method presentation changes.
No row-less failed case is treated as an ordinarily publishable LoadCaseSolve.

New metadata retention itself is admitted. Use a fixed stack route discriminator
and an optional bounded observer/capture store; the legacy observer=None path has
no added deep report/model clone. Reserve any side vector, retained error payload,
diagnostic split capacity and ordinary-proof copy before extending their lifetime.
If its census/reservation fails, continue the original legacy flow with W1 disabled;
do not add a blocking error to a previously publishable ordinary invocation.

## 3. Terminal eligibility — freeze the explicit classifier

Classification consumes the actual enum/payload and validated family facts, not
diagnostic code substrings. An unrecognized reason stays legacy; do not infer new
numerical authority from the broad name NumericallyUnresolved.

| Actual outcome | Proposed action before any suffix continuation |
|---|---|
| Parse/model/material/basis/load validation error; FrameKernelError other than the already deferred linear NumericalRange; invalid index/shape/orientation/nonfinite input | Immediate original terminal. No W1 result can repair invalid/unadmitted inputs under this scope. |
| StructuralError::Mechanism, Asymmetric, InvalidInput; rank-unresolved geometry (`rigid-restraint rank unresolved`, `constrained-body rank unresolved`) | Immediate original terminal. Geometry rank ambiguity is not on D1's numerical-trigger list. |
| NumericallyUnresolved from positive contribution absorption, assembly perturbation amplification, scaled-condition boundary, intended free-action failure, bounded-refinement original residual, or unresolved structural pivot/zero original diagonal | Recoverable trigger only for an independently admitted W1a source, captured digest available, qualified caller and no exact selection. Pin this finite reason-to-category table in tests. |
| `contribution-preserved prescribed coupling changes zero reduced load` | No F2a extension to nonzero prescribed-motion product cases; preserve terminal. Any claimed W1a reachability requires a source-backed follow-up, not a new broad trigger. |
| Supported-family NegativeEnergy | Potential W1 trigger under D1 §4.3; preserve direction/energy/allowance as ordinary evidence. Never convert a mechanism into it. |
| Range after the one existing W2 call fails | Eligible only if the independent W1a family/source gate passes. Preserve initial trigger AND W2 failure. A W2 failure proving invalid input/asymmetry/mechanism is immediately terminal; an unsupported family remains unsupported. NotEngaged is an invariant failure, not recovery permission. |
| Ordinary Passed/Sensitive or successful W2 publication | Keep rows/report. Sensitive, load loss and D5 demotion can become W1 candidates after complete coexistence arbitration. Passed non-trigger stays ordinary. |
| Later recovery/render/publication `?` error outside the accepted trigger seam (e.g. finish_case_ledger NonFiniteInput) | Preserve actual terminal; do not rename it a post-W2 Range. Bounded future recovery would require its precise accepted trigger. |
| Any nonlinear invocation, 0.4.0 load-state route, missing digest, unsupported caller profile | Original path; no exploratory continuation. S5-R handles missing digest as method-unavailable, not invalid model. |

The reason strings above occur at FK structural.rs:816,896,1285,1537,1641,1689,
1718,1783 and structural_adapter.rs:1331,1406. This proposes a reviewed concrete
classifier; the zero-diagonal/pivot grouping and W2 terminal precedence must be
backchecked against D1 before coding. Unknown additions must fail its closed test.

## 4. Transition table, including exact selection on either side of failure

| State and event | Action, next state and preserved result |
|---|---|
| LegacyPrefix, ordinary case completes, no exact selection | Keep normal result and ordinary seed; advance in original case/basis order. Existing allocations are legacy work; new observer storage is separately reserved. |
| LegacyPrefix, exact recovery selects | Set exact_selected_ever at selection, before any later row/finalization failure. Enter ExactBypass: finish the **unchanged** remaining legacy flow, its finalization or first terminal. W1 allocations/attempts/diagnostics stay absent. |
| LegacyPrefix, immediate terminal from §3 | Return that exact original terminal. Do not inspect later cases merely to find an exact selection. |
| LegacyPrefix, recoverable terminal, no earlier exact selection | While its payload/diagnostics are still live, take a fixed scalar snapshot. Run bounded borrowed census and request ContinuationPermit BEFORE preserving extra state or building any later basis/case. On denial, return the original terminal with no W1 diagnostic. On grant, move prefix into LegacyTerminal and enter DeferredSuffix. |
| DeferredSuffix, next ordinary case completes without exact selection | Retain admitted result/seed and its separate diagnostic suffix, update actual source ledger/counters, advance. No W1 solve yet. |
| DeferredSuffix, later recoverable terminal | Capture its typed seed under the same upfront permit; keep the FIRST LegacyTerminal unchanged; continue only within the already reserved suffix plan. |
| DeferredSuffix, later invalid/ineligible terminal, guard failure or arithmetic/count overflow | Stop further exploration. Drop suffix-only owners and return FIRST LegacyTerminal. No W1 attempt and no rewritten later error replaces the legacy first failure. |
| DeferredSuffix, later exact recovery selects | Stop at a narrow exploration-only hook immediately after the actual selected recovery returns. Debit its actual completed work once (no unperformed finalization/replay charge); retain a private abandoned-at-selection audit fact, drop the recovery/suffix, return FIRST LegacyTerminal byte-identically. No W1 token/diagnostic. Do not publish the later source selection as an invocation result. |
| All original cases complete, no terminal/exact selection | Finalize the full ordinary base once. It remains owned through draft formation. If no W1 trigger, return it unchanged. Otherwise request remaining M1 preparation/execution/completion permits. |
| DeferredSuffix exhausted, no exact selection | ReadyW1 only if every terminal is an admitted recoverable candidate and the complete source/batch/caller bound is granted. No fictitious complete ordinary envelope exists: the rollback base is FIRST LegacyTerminal plus retained ordinary successes. |
| ReadyW1, preparation/execution admission fails before W1 | Drop new owners; return the complete ordinary base or FIRST LegacyTerminal as applicable. No ordinary recomputation or new block. |
| Draft, every legacy-blocking case is recovered and final certificate/wire checks pass | Commit the fully admitted successor, preserving unselected ordinary rows/standing and request order; only then release rollback base. T6 partial-case standing is not introduced. |
| Draft, one original blocking case is not recovered, or invocation receipt fails | Return original ordinary base/terminal; preserve its original diagnostic prefix and quality. Only required W1 unavailable diagnostics may be appended after that prefix for work that actually ran, with case refs only and no invented rows. In a later-exact/admission-denied path no such work ran, so exact fallback bytes remain mandatory. |

The exploration-only exact-selection hook is needed to avoid inventing a later
public exact-source receipt after the invocation already terminated. It is not
used in LegacyPrefix/ExactBypass. Current exact failed attempts debit at PP:3660;
successful normal cases debit their actual finalization work at :4889. The hook
instead settles the selected recovery's already known WorkReport once and marks
that work **not finalized**. No copied prefix budget is ever restored into the live
ledger. Its snapshot explains legacy bytes; the live ledger records actual work.
At errors before normal source debit, preserve the known pending WorkReport in
the private audit; never call missing/unbooked work zero. Tests discriminate booked
and pending work and prevent charging the same successful report a second time.

The explicit two adversarial orders therefore have different faithful outcomes:
early exact selection → later failure follows the original ordinary/source path;
early recoverable failure → later exact selection returns the earlier terminal,
with no W1, while retaining actual exploratory costs privately. Neither swaps
diagnostic precedence, publishes partial rows, resets the old budget or reruns a case.

## 5. Newly introduced work and allocations — admission before continuation

| New category after a previously terminal prefix | Required upfront bound / accounting |
|---|---|
| Census and saved error/diagnostic/route state | Bounded borrowed visitor workspace/depth/visit count; checked case/UTF-8/capacity counts. No encoding(), cloning raw/model, graph builder or layout() used as free census. Retained Vec error payload and every prefix/suffix owner counted. |
| Previously unreached material/modulus bases | At most remaining-case count distinct bases; material/point/provenance clones, BuiltModel/maps/sections/boundary metadata, sparse assembly and deferred errors. Bound raw counts and worst dimensions before calling materials_for_modulus_basis/build_model/form_basis_stiffness. |
| Each later ordinary case | Load builders/maps, exact ledger and term storage, finite check, reduction, prescribed/free vectors, ordinary factor/scratch/refinement/formation checks and allowed observation lanes. Preserve current dense/observation guards; their ceilings are not the new suffix composition. |
| W2 in later cases | All current census/scaled-evaluation/publication owners and bounded internal evaluations, including failures. One W2 orchestrator call per actual range trigger; no extra replay to reconstruct the initial error. |
| Legacy exact-source attempts | Conditional dense source view (≤256 global DOFs), preparation/recovery, existing operation budget, failure/selection report and any normally reached case-finalization work. Use the same remaining SourceRecoveryBudget, with actual new charges and the exploration-stop rule above. |
| Later ordinary row/render retention | Results/metadata/ids/source_refs, preview side records, support vectors and exact evidence, diagnostic strings and error formatting. Full base-envelope assembly when normally reachable, plus any new proof seed copies. |
| W1 sources/batch/draft and caller output | Existing M1 S1–S6, now composed with saved prefix, suffix outcomes and their new lifetimes. Include I31 recipe scratch and I32 terminal/refusal evidence; no dropped Refused attempts re-created from a label. |

ContinuationPermit is a private finite descriptor bound to invocation/case cursor,
mode, source/profile revision, suffix-case and basis ceilings, raw populations,
required diagnostic widths, and **both requested and moving** peak expressions.
The guard must fit saved prefix + all kept suffix objects + maximum active ordinary/
W2/source scratch + its own preparation and eventual W1/caller reservation. It is
not merely a sum of new kernel scratch or a limit checked after a later basis exists.
Unknown source/format/allocation term, arithmetic overflow or unqualified caller
means no continuation. Proposed counts/step/span caps are policy inputs needing
ROOT review; this packet sets no numeric allowance or ordinary time limit.

Record continuation's real case/phase counts and legacy exact-source WorkReports.
Ordinary arithmetic, formatting and W2 have a separately derived finite cost/step
contract; do not bill invented LME or claim zero cost. W1's one meter starts only
when its first numeric method work runs, and survives every case/combination/refusal.
I32's proposed terminal-evidence seam must return actual Refused attempts/work;
current finish_terminal/combine discard some attempts. Meter increments are never
undone while waiting for that seam, and absent detailed evidence is an open gap.

## 6. Return and remaining proof cells

I32 C1 at885ed83a6a is an **unaccepted dependency**, not a routed implementation.
Its PreparedCaseSource/CombinationOperand and solve_sources proposal fits ReadyW1:
prepare ordinary operands under M1 without solving them; reuse only actual retained
operand snapshots in authored order. Ordinary-only mechanics remains ordinary;
subtraction/range retain their separate algebra and no automatic retained token.
Before Draft commit, a required retained mechanics combination with an unsupported
source or failed certificate cannot silently disappear. C1 proposes whole ordinary
base rollback with explicit W1 decline; this matches this table's rollback owner
and preserves previously published ordinary combinations. Where the base is the
first blocked terminal it does not establish successful recovery; return any
protected recovery/standing conflict to ROOT after RV43, rather than accepting
new withholding. Its Refused trace seam must supply attempts before current loss;
the routing observer cannot synthesize that evidence from meter differences.

The transition/ownership choices above are concrete proposals for RV41, not tested
implementations. Required before code: closed trigger/precedence backcheck; exact
legacy fallback byte equivalence; pending/source charge settlement; P1/P2 bounds
for metadata and the entire new suffix; and all caller cells in CALLERS_AND_TESTS.md.
The full byte derivation follows frozen routing/certificate/wire interfaces. No
broader transaction manager, registry cleanup policy or cancellation engine is needed.
