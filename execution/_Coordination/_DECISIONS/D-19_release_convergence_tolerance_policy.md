# D-19 — Release Convergence Tolerance Policy

**Status:** AWAITING_RULING — packet drafted; the human project authority has not ruled. This packet is a `PROPOSAL`. It confers no authority, changes no lifecycle state, and creates no release, professional, certification, sealing, authentication, or code-compliance claim.
**Prepared:** 2026-06-18, decision-preparation subscope, per the Application Integration And Issuance Loop decision-escalation step. Surfaced by the R4/R5 scope research recorded in `plans/PLAN_2026-06-17_prd_completion.md` §2.3 row D-19.
**Register row:** `execution/_Coordination/_DECISIONS/_REGISTER.md` row D-19.
**Plan basis:** `plans/PLAN_2026-06-17_prd_completion.md` §2.3 row D-19; Phase D rows D6 (`:590`) and D9 (`:593`).
**Epistemic posture:** evidence below is `FACT` with citations (pinned at repo HEAD `1c27081c5`; symbol names are the durable anchors if line numbers drift); inferences are labeled `ASSUMPTION`; the recommendation is labeled `PROPOSAL`; unknowns stay `TBD`. This packet decides nothing.

---

## 1. Decision statement and scope

**Decide:** the governed **solver convergence tolerance policy** — the residual/iteration *stopping criteria* for the assembled nonlinear iteration loop (and the iteration cap that bounds it) — closing the `TolerancePolicyTbd` solver diagnostic for the convergence dimension.

This is **distinct from the analytic verification tolerance** already ruled. The crux of this packet:

- **Verification tolerance (already policy — not reopened here).** The permitted variance when a *computed result is compared against a reference value* (analytic hand-calc, cross-engine corpus, regression golden). DEC-024 (Part 2) + DEC-026 (Part 1) rule this as a class-tiered structure with per-quantity-kind governed relative+absolute pairs, the analytic class seeded at the measured `1.0e-9` (`execution/_Decomposition/SOFTWARE_DECOMP.md:594, 596`). D-19 does **not** reopen DEC-024/DEC-026.
- **Convergence tolerance (the open item — this packet).** The *stopping criterion* of the nonlinear iteration loop: how small the residual must be (and within how many iterations) before the assembled nonlinear solve is declared converged. This is currently **caller-supplied, not policy-bound** (§2.3), and the unresolved policy is surfaced — not defaulted — by the `TolerancePolicyTbd` diagnostic (§2.4).

**In scope:** the residual convergence criterion for the assembled active-set nonlinear loop; the iteration cap; whether the criterion is single/global or class-tiered by nonlinearity kind (gap / one-way / lift-off / friction); the policy's ownership/record mechanism; the no-silent-fallback posture; how the criterion maps onto the existing nonconvergence diagnostics taxonomy.

**Out of scope:** the DEC-024/DEC-026 *verification* tolerance classes (not reopened); the internal dense-verification epsilons that already disclaim policy status (`core/solver/frame_kernel/src/lib.rs:19–21` — `AXIS_TOLERANCE`/`DENSE_SOLVE_ZERO_PIVOT_GUARD` at `1.0e-12`, the latter commented "Internal dense verification pivot guard; not the project solver tolerance policy"); coverage thresholds (DEC-024 Part 2 / D-04b); ownership of the assembled loop itself and its iteration-control *method* (that is **D-16**, with which this packet is paired — §6); sparse-solver live-path timing (D-17); professional/code-compliance acceptance criteria, which this software never decides.

**What it blocks:** Phase D items **D6** (assembled nonlinear iterative solve — "the product hinge of R4"; gates on D-16 + D-19, `plans/PLAN_2026-06-17_prd_completion.md:590`) and **D9** (R4 validation set — converged global-solve cases; gates on D-19, `:593`); the **R4 exit criterion "Nonlinear support validation cases converge"** (`docs/PRD.md:1236`); and **RGAP-004** (release thresholds unresolved, BLOCKER_TO_RELEASE_CLAIM, OPEN_OR_HUMAN_GATED — `execution/_Aggregation/TP-RELEASE-GAP-REGISTER-REFRESH-001_2026-05-31/Gap_Disposition_Register.csv:5`). DEC-026 began closing RGAP-004 on the verification axis; the convergence axis is the remaining open thread.

---

## 2. Current state evidence

### 2.1 The verification tolerance is already policy (boundary — not reopened)

- **FACT:** DEC-024 (2026-06-11) accepted D-04 Part 2 (C-C coverage) and an initial Part 1; Part 1 was **superseded the same day by DEC-026**, which adopts class-tiered verification tolerances — "numerical verification tolerances are organized by reference-result class (analytic benchmark; cross-engine-exact; regression-golden-exact), each class with its own governed tier and per-quantity-kind values within class recorded as governed relative+absolute pairs (the absolute member is the explicit near-zero floor); the analytic class is seeded at the measured 1.0e-9 relative where suites pass today; unmeasured per-kind values remain `TBD` with the existing `tolerance_policy_tbd_diagnostic` continuing to fire until filled by measurement; fixture-local `tolerance_policy` overrides may only tighten — any loosening … is itself a governance event recorded in the governed tolerance record" (`execution/_Decomposition/SOFTWARE_DECOMP.md:596`; DEC-024 at `:594`).
- **FACT:** the `1.0e-9` analytic seed is wired as the internal assertion level in the benchmark crates: `const INTERNAL_ASSERTION_EPSILON: f64 = 1.0e-9` (`validation/benchmarks/mechanics/src/lib.rs:48`, used at `:1613–1732`; `validation/benchmarks/stress/src/lib.rs:22`). These compare a computed result to a reference — the *verification* axis, not a stopping criterion.

### 2.2 The verification tolerance ≠ the convergence tolerance (the crux)

The two are different physical quantities:

| | Verification tolerance (DEC-024/026) | Convergence tolerance (D-19, open) |
|---|---|---|
| Compares | computed result ↔ reference value | iteration residual ↔ stopping threshold |
| Question | "is the answer correct?" | "is the iteration done?" |
| Status | governed (class-tiered relative+absolute) | caller-supplied; **`TBD`** |
| Seed | `1.0e-9` analytic relative (`mechanics/src/lib.rs:48`) | none ruled |
| Near-zero handling | absolute floor (DEC-026 rider) | n/a — residual is a magnitude |

`ASSUMPTION`: the `1.0e-9` verification seed does **not** automatically transfer to a convergence criterion — it was measured as the level at which result-vs-reference assertions pass for the current invented SI-basis fixtures, not as an iteration stopping point. D-04 §3 Q4 flagged the same non-transfer concern for mixed-unit/larger models (`D-04_tolerance_coverage_thresholds.md:72`).

### 2.3 Convergence tolerance is caller-supplied, not policy-bound (FACT)

- **FACT:** the nonlinear-support active-set solver takes the convergence tolerance as a *runtime input*, not from policy: `ActiveSetIterationInput { iteration, max_iterations, tolerance: f64, … }` (`core/solver/nonlinear_supports/src/lib.rs:221–228`). Convergence is decided as `converged: residual_norm <= input.tolerance` (`:508`); the report record echoes both `tolerance` and `max_iterations` (`:255–259`, `:303–308`).
- **FACT:** the crate explicitly disclaims owning the policy and the assembled loop: its limitations record states "This crate does not assemble or solve the global nonlinear system." and "Production tolerance policy, sparse solver selection, final constraint strategy, and result-envelope integration remain outside this bounded crate." (`core/solver/nonlinear_supports/src/lib.rs:335–337`). This is the D-16 ownership seam.
- **`ASSUMPTION` / caveat:** in the *current* bounded classifier the residual is a placeholder — `residual_norm = changed_supports.len() as f64` (`:474`), i.e. a count of supports that changed active-set state this iteration, declared so in the assumptions (`:327`). A residual-of-forces convergence norm does not yet exist because no assembled global solve consumes it; that assembled loop is the D-16/D6 work this policy must be ready for.

### 2.4 The unresolved policy is a live, visible diagnostic — no silent fallback (FACT)

- **FACT:** `tolerance_policy_tbd_diagnostic()` emits a `Warning` from `DiagnosticSource::SolverConfiguration`: "solver tolerance policy remains TBD and must be accepted before release-quality performance claims" (`core/solver/diagnostics/src/lib.rs:625–632`), with remediation "Bind the solve path to an accepted tolerance policy before release-quality claims." (`:679–681`).
- **FACT:** its code `SolverDiagnosticCode::TolerancePolicyTbd` (`core/solver/diagnostics/src/lib.rs:147`) is classified `SolverDiagnosticClass::Tbd` → analysis-boundary class `"TBD"` (`:48–62`, `:650–651`).
- **FACT:** the performance harness attaches it (alongside `sparse_solver_tbd_diagnostic()`) to every harness run: `let mut diagnostics = vec![ sparse_solver_tbd_diagnostic(), tolerance_policy_tbd_diagnostic() ];` (`core/solver/performance_harness/src/lib.rs:416–418`); harness tests assert its presence (`:940`, `:1123`). This is the existing no-silent-fallback mechanism: an unruled tolerance is surfaced, not defaulted — the same posture DEC-026 preserves for the verification axis. D-19 should retire it only for the convergence surfaces whose policy entries become ruled and non-`TBD`.

### 2.5 The nonconvergence diagnostics taxonomy (FACT — the seam D-19 maps onto)

The convergence criterion's outcomes already have a typed home in `core/solver/diagnostics`:

- **FACT:** `convergence_diagnostic(iteration_count, max_iterations, residual_norm, tolerance)` (`core/solver/diagnostics/src/lib.rs:494–526`) is the single decision point: returns `Ok(None)` when `residual_norm <= tolerance` (converged, `:503–504`); a **`Failure`**-severity `NonConvergence` diagnostic from `DiagnosticSource::SolverIteration` when `iteration_count >= max_iterations` and still over tolerance — "solver did not converge after {n} iterations; residual {r} exceeds tolerance {t}" (`:507–516`); and a **`Warning`**-severity `NonConvergence` when over tolerance but still under the iteration cap (`:518–525`).
- **FACT:** severity taxonomy: `DiagnosticSeverity {Info, Warning, Blocking, Failure}` (`:21–26`); source taxonomy: `DiagnosticSource {ModelValidation, MechanicsSolver, SolverConfiguration, SolverIteration}` (`:29–34`). A `Warning`-severity `NonConvergence` from `SolverIteration` is classified `SolverDiagnosticClass::NonlinearWarning` → boundary class `"NONLINEAR_WARNING"`; `Failure` severity routes to `SolveBlocking` → `"SOLVE_BLOCKING"` (`:653–658`, `:660–667`, `:56–62`). Default remediation for `NonConvergence`: "Review the nonlinear iteration inputs, active-set state, and accepted tolerance policy." (`:682–684`) — already names the accepted tolerance policy as the missing input.
- **FACT:** the nonlinear-supports crate consumes this taxonomy: `evaluate_active_set_iteration` calls `convergence_diagnostic(...)` (`core/solver/nonlinear_supports/src/lib.rs:476–501`), decorates the resulting `NonConvergence` with active-set context (changed supports + per-support states, `:581–613`), exposes `nonconvergence_code()` (`:615–620`), and tests both the converged path and the iteration-limit `NonConvergence`/blocked path (`:894–942`).

**Implication:** the convergence policy is two governed numbers consumed at one existing call site — the residual `tolerance` and the `max_iterations` cap fed to `convergence_diagnostic`. The taxonomy that classifies the outcome (warn-before-cap vs. fail-at-cap) already exists; D-19 supplies the governed thresholds, it does not build the diagnostic machinery.

### 2.6 What this decision gates (FACT)

- **FACT:** PRD R4 exit — "Nonlinear support validation cases converge." (`docs/PRD.md:1236`); R4 is §22.5 "Piping Components and Nonlinear Supports" (`:1223`). R5 reproduction — "External engineers can reproduce validation examples." (`:1252`).
- **FACT:** Plan D6 — "wrap the existing active-set classifier in a global iteration loop … with convergence criteria mapped to the existing nonconvergence diagnostics taxonomy and a governed convergence tolerance. The product hinge of R4." gating on `D-16` + `D-19` (`plans/PLAN_2026-06-17_prd_completion.md:590`). D9 — "R4 exit needs *converged global-solve* cases" gating on `D-19` (`:593`). Phase-D exit evidence: "nonlinear validation cases converge under the governed tolerance" (`:599`).
- **FACT:** the nonlinear benchmark crate already carries unresolved convergence slots: `tolerance_policy: Option<&'static str>` is `None` across all fixtures (`validation/benchmarks/nonlinear/src/lib.rs:162, 332–610`), `tolerance_policy_is_unresolved()` exists (`:215–218`), fixtures carry `expected_converged: bool` (`:190`) and run at `tolerance: 0.0` (`:292, 358`), and the fixture unit basis records a `residual_tolerance` only as a dimensionless "count" with "project unit catalog remains TBD" (`:126–153`). These slots fill with the governed criterion once ruled — the same slot-fill pattern DEC-026 uses for verification.

---

## 3. Open questions awaiting ruling

1. **Residual definition.** The governed criterion presumes a residual norm. The current bounded classifier uses a state-change *count* as a placeholder residual (`nonlinear_supports/src/lib.rs:474`); the assembled D6 loop will produce a force/displacement residual. Is the convergence tolerance defined on a normalized force residual, an incremental-displacement norm, or both (energy)? `TBD` until the D-16/D6 assembled loop fixes the residual quantity.
2. **Single vs. class-tiered.** One global convergence tolerance + iteration cap, or per-nonlinearity-class criteria (gap / one-way / lift-off / friction may converge at different rates; friction with slip is the hardest)? No in-repo measurement distinguishes per-class convergence rates yet — values `TBD`.
3. **Iteration cap.** What `max_iterations` is the governed default, and is it a hard `Failure` cap (current behavior at `:507–516`) or a warn-then-continue tier? The taxonomy supports both (`Warning` below cap, `Failure` at cap, `:518–525` / `:507–516`).
4. **Relative vs. absolute residual / near-zero floor.** DEC-026 gave the *verification* axis a relative+absolute pair with an explicit near-zero absolute floor. Does the convergence criterion need the same pairing (an absolute floor so a near-zero-load model is not chased to round-off)? `ASSUMPTION`: yes, by analogy — but unmeasured, so `TBD`.
5. **Change-provenance.** Does DEC-026's tighten-only rider extend here (loosening the convergence tolerance or raising the iteration cap is a governance event, not a fixture edit)? Recommended yes (§5), for parity.
6. **Diagnostic retirement scope.** `tolerance_policy_tbd_diagnostic` currently fires unconditionally in the harness (`performance_harness/src/lib.rs:416–418`). Once a convergence criterion is ruled and bound, does it retire for the assembled-solve surface, or stay until *both* the verification (DEC-026 unmeasured entries) and convergence axes are fully non-`TBD`? `TBD` — recommend per-surface retirement mirroring DEC-026.

---

## 4. Options

### Option CV-A — Single global residual tolerance + iteration cap

One governed pair — a single relative residual tolerance and one `max_iterations` cap — recorded as a governed value, fed to the existing `convergence_diagnostic` call site; fixtures cite it; the `NonConvergence` taxonomy classifies the outcome unchanged.

- **Mechanism:** governed record holds `{residual_tolerance, max_iterations}`; D6's assembled loop reads them; `convergence_diagnostic(iter, max_iterations, residual_norm, tolerance)` (`diagnostics/src/lib.rs:494`) is already the consumer.
- **Pros:** cheapest to state and adopt; one number to report at R4 exit; immediately fills the `nonlinear` crate's `tolerance_policy` slots (`nonlinear/src/lib.rs:162`); the diagnostic seam is untouched.
- **Cons:** one tolerance must fit gap/one-way/lift-off/friction at once — friction-slip cases plausibly need a looser or differently-normed criterion; no near-zero absolute floor; sits in tension with DEC-026's deliberate move *away* from a single global verification tolerance (the T-B → T-C reconsideration, `SOFTWARE_DECOMP.md:594, 596`).
- **Risk:** medium — a global value chosen before the D6 assembled loop produces real residuals would be ungrounded (the D-04 §3 Q4 "no measurement" hazard, `D-04_tolerance_coverage_thresholds.md:72`); risk of silent loosening to force convergence.

### Option CV-B — Class-tiered convergence tolerances mirroring DEC-026 (recommended)

Extend the DEC-026 governed-record pattern to the convergence axis: per-nonlinearity-class criteria (gap / one-way / lift-off / friction), each a governed relative+absolute residual pair plus an iteration cap, with the absolute member as the explicit near-zero floor — exactly the structure DEC-026 ruled for verification, applied to the stopping criterion.

- **Mechanism:** one governed convergence record keyed by nonlinearity class; fixtures (`nonlinear/src/lib.rs`) cite class+criterion ids via the existing `tolerance_policy` slots; unmeasured class values stay `TBD` and keep `tolerance_policy_tbd_diagnostic` firing for those surfaces; `convergence_diagnostic` consumes the class-resolved pair + cap at `:494`. Fixture-local overrides may only tighten; any loosening is a governance event recorded centrally (DEC-026 rider parity).
- **Pros:** matches the active-set behavior — the four nonlinearity classes are already first-class in the classifier (`classify_support_state`, `nonlinear_supports/src/lib.rs:519–569`) and the taxonomy carries active-set context per class (`:581–613`); reuses the no-silent-fallback seam exactly as DEC-026 does (TBD entries keep the diagnostic live); near-zero floor handled; tighten-only rider closes the silent-loosening path structurally; tolerance changes are evidence-tracked by construction.
- **Cons:** more classes to govern and explain than CV-A; per-class convergence values are genuinely unmeasured at ruling time (`TBD` until D6 produces residuals), so part of the record launches empty — same posture DEC-026 accepted for unmeasured verification kinds; slightly more record-keeping.
- **Risk:** low — it inherits a ruled, working pattern (DEC-026) and its enforcement seam rather than inventing one; the only residual risk is over-tiering before measurement, mitigated by leaving unmeasured classes `TBD`.

### Option CV-C — Convergence criterion + iteration-control (relaxation / line-search) governance

CV-B's class-tiered residual criterion **plus** governed iteration-control parameters — under-relaxation factor and/or a line-search/damping switch — recorded as part of the convergence policy, on the premise that a tolerance alone cannot make friction/lift-off cycling cases converge.

- **Mechanism:** governed record holds, per class, `{residual_tolerance, absolute_floor, max_iterations, relaxation_factor, line_search: on/off}`; D6's loop reads the full control set.
- **Pros:** most complete — addresses the *real* nonconvergence failure mode (active-set chattering / oscillation between states), which a tolerance alone cannot fix; gives the D6 loop a governed, reproducible control posture; byte-reproducibility (DEC-023 fixed-operation-order posture) is preserved because the controls are governed constants, not adaptive heuristics.
- **Cons:** ranges furthest beyond a pure "tolerance" decision and overlaps **D-16's iteration-control *method* question** (§6) — risks deciding in D-19 what D-16 owns; relaxation/line-search parameters are entirely unmeasured and method-dependent, so they would launch almost wholly `TBD`; couples the tolerance ruling to a loop design that does not yet exist (D6/DEL-04-01 integration tranche).
- **Risk:** medium-high — premature: it governs control parameters for an assembled loop whose method D-16 has not selected; better sequenced *after* D-16 fixes the method.

---

## 5. Recommendation — `PROPOSAL`

Adopt **Option CV-B** — extend the DEC-026 class-tiered governed pattern to a *convergence* tolerance record:

1. **Structure:** a governed convergence-tolerance record (one file, the DEC-026 governed-record pattern), keyed by nonlinearity class (gap / one-way / lift-off / friction), each entry a relative+absolute residual pair (absolute member = explicit near-zero floor) plus a governed `max_iterations` cap. This is structurally the DEC-026 ruling applied to the stopping criterion rather than the comparison criterion — explicitly cross-referenced to DEC-026, not modifying it.
2. **Seeding and TBD discipline:** seed only what is measurable today; every unmeasured class entry stays `TBD`, keeps the nonlinear crate's `tolerance_policy_is_unresolved()` true for its fixtures (`nonlinear/src/lib.rs:215`), and keeps `tolerance_policy_tbd_diagnostic` firing for the assembled-solve surface (`diagnostics/src/lib.rs:625`) — no invented numbers. Because the D6 assembled loop does not yet emit a force residual (§2.3), the *residual definition* (Q1) is fixed by D-16/D6 first; the governed values are seeded from the first converged D6/D9 runs, not guessed now.
3. **Consumption:** the governed pair + cap feed the **existing** `convergence_diagnostic` call site (`diagnostics/src/lib.rs:494`); the `NonConvergence` taxonomy (`Warning` below cap → `NONLINEAR_WARNING`; `Failure` at cap → `SOLVE_BLOCKING`, `:653–667`) classifies the outcome unchanged. D-19 supplies thresholds; it builds no new diagnostic machinery.
4. **Change-provenance rider (DEC-026 parity):** fixture-local convergence overrides may only *tighten*; any loosening of a governed convergence value or raising of an iteration cap is itself a governance event recorded in the governed convergence record, never a fixture-local edit.
5. **Iteration-control deferral:** the relaxation/line-search controls of CV-C are **not** ruled here; they are routed to **D-16** (the loop-method owner). D-19 governs the *stopping* criterion; D-16 governs *how* the loop reaches it.

**Rationale:** CV-B is the only option that neither invents unmeasured convergence numbers (CV-A's hazard) nor pre-empts D-16's method ownership (CV-C's hazard), while reusing a *ruled, working* pattern (DEC-026) and its no-silent-fallback enforcement seam (the `TolerancePolicyTbd` + `NonConvergence` taxonomy already in `core/solver/diagnostics`). It keeps the verification/convergence distinction clean: same governance shape, different governed quantity. This recommendation is a `PROPOSAL` only.

---

## 6. Relationship to D-16 (loop ownership) and downstream impact

**D-16 owns the loop that consumes this policy.** D-16 (`plans/PLAN_2026-06-17_prd_completion.md:92`) decides *which deliverable owns the assembled iterative loop* wrapping the active-set classifier — DEL-04-04 (the classifier crate, which "does not assemble or solve the global nonlinear system", `nonlinear_supports/src/lib.rs:335`) or a frame-assembly integration tranche in DEL-04-01 — **and the iteration-control method** (active-set with convergence/relaxation). The clean split:

- **D-16:** *who* builds the loop and *how* it iterates (method, active-set update, relaxation/line-search).
- **D-19 (this packet):** *when* that loop stops (governed residual tolerance + iteration cap) and *how* the stop is classified (the existing taxonomy).

They are paired in D6 (which gates on both, `:590`) and should ideally be ruled at the same sitting: D-16 fixes the residual quantity and method (resolving D-19 §3 Q1), then D-19's governed values are seeded from that loop's first converged runs. Ruling D-19 *before* D-16 is possible (the governed-record structure is method-independent) but its numeric entries stay `TBD` until D-16/D6 produce real residuals — which is the recommended, honest posture anyway.

| Surface | Impact of this ruling |
|---|---|
| **D6 (assembled nonlinear solve)** | Gains its "governed convergence tolerance" (`:590`); reads the governed record at the `convergence_diagnostic` call site (`diagnostics/src/lib.rs:494`). |
| **D9 (R4 validation set)** | "converged global-solve cases" (`:593`) cite the governed convergence criterion; `nonlinear` crate `tolerance_policy` slots fill, `expected_converged` fixtures bind to a policy source (`nonlinear/src/lib.rs:162, 190`). |
| **Nonlinear-supports crate** | The caller-supplied `tolerance`/`max_iterations` inputs (`nonlinear_supports/src/lib.rs:224, 223`) gain a policy-bound default source; the crate's "outside this crate" limitation (`:336`) is partly discharged by the governed record. |
| **Solver diagnostics / harness** | `tolerance_policy_tbd_diagnostic` retires only for convergence surfaces whose class entries are ruled non-`TBD` (mirroring DEC-026's per-surface retirement); the `NonConvergence` taxonomy is unchanged. |
| **R4 exit / RGAP-004** | "validation cases converge" (`docs/PRD.md:1236`) gains its governed criterion; RGAP-004's convergence thread closes alongside DEC-026's verification thread. |
| **DEC-026 / D-16** | DEC-026 referenced, not modified; D-16 inherits the residual-definition and method questions D-19 defers to it (§3 Q1, §5.5). |
| **Register row D-19** | `NOT_PREPARED` → `AWAITING_RULING` at fan-in (dispatching persona; not this packet). |

---

## 7. Authority and ruling record

Only the **human project authority** rules on D-19. Agents prepared this packet and may not certify, approve, or adopt it. Per existing decision practice (`DEC-018`..`DEC-040`), the accepted ruling is recorded as the next `DEC` entry (next available is **`DEC-041`**) in `execution/_Decomposition/SOFTWARE_DECOMP.md` §12 citing this packet; the dispatching persona then updates `execution/_Coordination/_DECISIONS/_REGISTER.md` row D-19 from `AWAITING_RULING` to `RULED` with the pointer. Any convergence value left `TBD` at ruling time remains `TBD` in code and diagnostics until a later governed record supplies it (the DEC-026 discipline). This packet does not edit the register and does not resolve the decision.
