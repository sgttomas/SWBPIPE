# D-04 — Numerical Tolerance + Coverage Thresholds (RGAP-004)

**Status:** RULED — 2026-06-11 the human project authority initially selected T-B + C-C (`DEC-024`), then revised Part 1 the same day after reviewing the recorded T-B residuals and their interaction with the `DEC-023` in-house solver choice: the standing ruling is **T-C + C-C with two riders** (`DEC-026`) — class-tiered governed tolerances with per-kind relative+absolute pairs (absolute member is the near-zero floor), analytic class seeded at the measured 1.0e-9, unmeasured values stay TBD with the diagnostic firing, fixture overrides may only tighten (loosening is a governance event recorded centrally); blocking inventory gates plus recorded-not-blocking numeric coverage, tooling to a named D-04b. Recorded in `execution/_Decomposition/SOFTWARE_DECOMP.md` §12; register row updated.
**Prepared:** 2026-06-11 by TASK (Type 2), requested by WORKING_ITEMS (Type 1), tranche TP-DECIDE-PREP-001 decision-preparation subscope.
**Register row:** `execution/_Coordination/_DECISIONS/_REGISTER.md` row D-04.
**Plan basis:** `plans/PLAN_2026-06-10_prd_completion.md` §2 row D-04.
**Epistemic posture:** evidence below is `FACT` with citations; inferences are labeled `ASSUMPTION`; the recommendation is labeled `PROPOSAL`; unknowns stay `TBD`. This packet decides nothing.

---

## 1. Decision statement and scope

**Decide:** the two governed threshold policies that RGAP-004 leaves open — kept clearly two-part:

- **Part 1 — numerical tolerance policy** for solver/mechanics verification evidence: what permitted variance applies when a computed result is compared against a reference value, where those tolerances live, and how a tolerance change is itself evidence-tracked.
- **Part 2 — coverage thresholds**: what "enough test coverage" means as a release-quality gate input — numeric floors, enforced surface inventories, or a hybrid — and with what tooling, under the local-only constraint.

**In scope:** benchmark permitted variance for solver, stress-recovery, and nonlinear evidence; the tolerance ownership/record mechanism; the no-silent-fallback posture; coverage gate shape per language lane (Rust, TS/Vitest, Python); local evidence-sweep enforcement.

**Out of scope:** unit-conversion witness tolerances — already ruled by DEC-018 and **not reopened here** (§2.3); CI provider and hosted workflow location (D-05, noted as an interaction in §3 Q8); performance thresholds (`docs/RELEASE_QUALITY_GATES.md:158` keeps these a separate TBD); fatigue/allowable/code-compliance criteria, which are professional-domain content this software never decides (`docs/VALIDATION_STRATEGY.md:139–142`).

D-04 blocks the R1/R4/R5 evidence claims and the release-quality gates (`_REGISTER.md` row D-04; `plans/PLAN_2026-06-10_prd_completion.md:43` — timing "Before first external evidence bundle (mid-plan)"). Plan items B3, E2, and E7 consume the ruling (`plans/PLAN_2026-06-10_prd_completion.md:84, 121, 126`).

---

## 2. Current state evidence

Citations pinned at repo HEAD `5079a8fa7`. Symbol names are the durable anchors if line numbers drift.

### 2.1 The gap as registered

- **FACT:** RGAP-004 (gap register row): "Release thresholds and evidence bundle remain unresolved." / "TP-INTEGRATED-VERIFY-002 records passing tests, but no accepted release thresholds or release evidence-bundle policy were created." Disposition: BLOCKER_TO_RELEASE_CLAIM, OPEN_OR_HUMAN_GATED (`execution/_Aggregation/TP-RELEASE-GAP-REGISTER-REFRESH-001_2026-05-31/Gap_Disposition_Register.csv:5`).
- **FACT:** `docs/RELEASE_QUALITY_GATES.md` §10 lists the open decisions verbatim: "TBD: final numerical tolerance policy for solver, stress, and nonlinear benchmarks" (`:156–157`) and "TBD: coverage thresholds for Rust, Python, GUI, validation, and protected-content gates" (`:159–160`). The solver gate requires "tolerance source is named, or the tolerance remains `TBD`" (`:69`) and forbids "selecting final public release tolerances without a human governance record" (`:74`).
- **FACT:** `docs/VALIDATION_STRATEGY.md` defines verification as correctness "within declared tolerances" (`:19`), keeps "final release tolerances, CI gates, and professional reliance policy as `TBD` until human approval" for both benchmark suites (`:60–63, :69–71`), requires hand-calc witnesses to record "tolerance basis or `TBD`" (`:132–136`), and states "The final benchmark tolerance policy … remain `TBD`" (`:117–119`). "Unapproved thresholds … remain explicit `TBD` items until a governed decision or evidence record exists" (`:95–97`).
- **FACT:** R1 status row: "Tolerance thresholds for benchmark evidence remain TBD (Decision D-04)" (`plans/PLAN_2026-06-10_prd_completion.md:26`); FR-008 row carries "(thresholds: D-04)" (`:151`).

### 2.2 Tolerance practice already in code (FACT inventory)

- **Benchmark crates carry an explicit, structurally unresolved tolerance slot.** `ExpectedValue.tolerance_policy: Option<&'static str>` exists in all three benchmark crates and is `None` everywhere; each crate exposes `tolerance_policy_is_unresolved()` (`validation/benchmarks/mechanics/src/lib.rs:227, 242–246`; `validation/benchmarks/stress/src/lib.rs:171, 292–295`; `validation/benchmarks/nonlinear/src/lib.rs:162`). The stress crate additionally hard-codes `final_tolerance_policy: "TBD"` in its readiness record (`validation/benchmarks/stress/src/lib.rs:187, 216`). Required readiness TBD markers include "final tolerance policy", "release thresholds", "CI gate policy" (`validation/benchmarks/mechanics/src/lib.rs:54–61`).
- **Internal assertion epsilons exist but disclaim policy status.** `INTERNAL_ASSERTION_EPSILON: f64 = 1.0e-9` in mechanics and stress benchmarks (`validation/benchmarks/mechanics/src/lib.rs:48`; `validation/benchmarks/stress/src/lib.rs:22`); test helper `assert_close` at `1.0e-9` in loads/pipe crates (`core/loads/primitive_loads/src/lib.rs:2410–2414`; `core/solver/straight_pipe/src/lib.rs:1512`); `ASSERT_TOLERANCE: f64 = 1.0e-9` plus `AXIS_TOLERANCE`/`DENSE_SOLVE_ZERO_PIVOT_GUARD` at `1.0e-12` in the frame kernel, the latter commented "Internal dense verification pivot guard; not the project solver tolerance policy" (`core/solver/frame_kernel/src/lib.rs:19–21, 1044`).
- **Convergence tolerance is caller-supplied, not policy-bound.** The nonlinear-support solver takes `tolerance: f64` as input and reports `converged: residual_norm <= input.tolerance` (`core/solver/nonlinear_supports/src/lib.rs:224, 305, 508`); its readiness note says "Production tolerance policy, sparse solver selection, final constraint strategy, and result-envelope integration remain outside this bounded crate" (`:336`).
- **The unresolved policy is a live, visible diagnostic.** `tolerance_policy_tbd_diagnostic()` emits a Warning: "solver tolerance policy remains TBD and must be accepted before release-quality performance claims" (`core/solver/diagnostics/src/lib.rs:535–541`), and the performance harness attaches it to runs (`core/solver/performance_harness/src/lib.rs:274`). This is the existing no-silent-fallback mechanism: an unruled tolerance is surfaced, not defaulted.

### 2.3 What DEC-018 already ruled (boundary — not reopened)

- **FACT:** DEC-018 (2026-06-10) accepted the D-01 recommendation including "a two-tier conversion-witness tolerance policy" (`execution/_Decomposition/SOFTWARE_DECOMP.md:588`): identity conversions bit-exact; exact-by-definition factors round-trip ≤ 1 ULP; chained/derived and affine round-trips relative error ≤ 1e-12 (`execution/_Coordination/_DECISIONS/D-01_unit_catalog_acceptance.md:110`). D-01e's scope boundary is explicit: "this rules *conversion* testing only. Solver/benchmark variance and coverage thresholds are D-04" (`D-01_unit_catalog_acceptance.md:106`).
- **FACT:** DEC-018's implementation is pending Phase B work: `core/units/` contains only `README.md` (no crate), and that README keeps "the canonical basis, conversion constants, numeric representation, and tolerance policy … `TBD` until accepted by a human review gate" with conversion tests "gated until the unit catalog, factor representation, and tolerance policy are accepted" (`core/units/README.md:16, 46, 81`). D-04 must neither reopen DEC-018's tiers nor block on their implementation.

### 2.4 Contract-corpus precedent: exact comparison + enforced inventory floor

- **FACT:** the cross-engine operation corpus uses **exact equality on semantic fields** — no numeric tolerance at all — by restricting corpus numbers to a canonical range (`|v| < 1e15`, `|v| >= 1e-6` or 0) and a deterministic decimal rendering (`fixtures/model_operations/contract_corpus/README.md:44–54, 85–95`; `core/model_operations/operation_applier/tests/contract_corpus.rs:110–117`). All 44 cases reproduced exactly across engines (`README.md:141, 143`).
- **FACT:** the corpus enforces a **programmatic coverage floor**: `contract_corpus_covers_the_required_kinds_and_block_classes` fails with "coverage floor violated" if any of the 10 `REQUIRED_ACCEPTED_KINDS`, required blocking-diagnostic codes, or dynamic paths lacks a corpus case (`contract_corpus.rs:42, 355, 422–441`). The latest verification record cites it: "446 passed / 0 failed (incl. corpus runner with programmatic coverage-floor enforcement)" (`plans/VERIFICATION_2026-06-11_operation_seam_unification.md:13`). This is the repo's only operating coverage gate — an inventory gate, not a numeric percentage.

### 2.5 Coverage tooling state

- **FACT:** no numeric coverage measurement is configured anywhere: no tarpaulin/llvm-cov/grcov in any `Cargo.toml`; no coverage configuration in `package.json`/`apps/desktop/package.json` (Vitest runs as `"test": "vitest run"`, `apps/desktop/package.json:11`); no `.github/` or CI directory exists; `tools/release/check_release_readiness.py` runs pytest and per-crate `cargo test` steps (search roots `core`, `validation/benchmarks`) but measures no coverage. (Established by repo-wide search at HEAD; absence of evidence of any other config is `ASSUMPTION` only beyond the named locations.)
- **FACT:** DEC-011 accepted the test baseline with "CI provider, coverage thresholds, and performance thresholds remain TBD"; DEC-012 keeps coverage thresholds an implementation-level TBD "unless a sealed brief or later human ruling resolves them" (`execution/_Decomposition/SOFTWARE_DECOMP.md:581–582`).

### 2.6 What this decision gates

- **FACT:** PRD exit criteria — R1: "Required benchmark suite passes within documented tolerances" (`docs/PRD.md:1188`); R4: "Nonlinear support validation cases converge" (`:1236`); R5: validation manual deliverable and "External engineers can reproduce validation examples" (`:1243, 1252`).
- **FACT:** plan E7 requires "Gate records: release-quality gate outcomes … with thresholds from D-04" (`plans/PLAN_2026-06-10_prd_completion.md:126`); E2 assembles the validation manual "from benchmark witnesses + tolerances accepted under D-04" (`:121`).
- **FACT:** D-10 (RULED as DEC-021) established that tolerance/coverage evidence wording in rendered reports is **slot data, not template prose**, so D-04's ruling changes data, not the renderer (`execution/_Coordination/_DECISIONS/D-10_report_rendering_target.md:75, 138`).

---

## 3. Open questions awaiting ruling

1. **Reference-result classes:** do analytic/hand-calc benchmarks, cross-engine comparisons, and regression goldens each get their own tolerance tier (the corpus already proves exact equality is achievable for the cross-engine class, §2.4), or does one policy span all classes?
2. **Where tolerances live:** governed values in a policy doc that fixtures cite by id (DEC-018's conversion-constant pattern), or fixture-local values per `ExpectedValue.tolerance_policy` slot (§2.2)? `TBD` until ruled; the slots accept either.
3. **Tolerance-change provenance:** is a tolerance change itself a gated evidence event (named record + re-run witness), or an ordinary code change? RELEASE_QUALITY_GATES `:74` forbids ungoverned selection but is silent on *changes* after first acceptance. `TBD`.
4. **Status of the existing `1.0e-9` internal epsilons:** promoted to the initial governed value (it is the measured passing assertion level for current invented SI-basis fixtures, §2.2 — `FACT` that tests pass at it; `ASSUMPTION` that it transfers to mixed-unit or larger models), replaced, or left internal-only beneath a separate governed policy?
5. **Per-quantity-kind structure:** do displacement, rotation, force/moment, and stress quantities need distinct relative+absolute pairs (absolute floors matter near zero)? No in-repo measurement distinguishes them yet — values `TBD`.
6. **Coverage gate shape:** numeric line/branch floors per language lane, enforced surface inventories (corpus-floor precedent, §2.4), or hybrid? If numeric: which tooling per lane, and are the numbers blocking or telemetry? Any specific percentage is `TBD` — no baseline measurement exists (§2.5).
7. **Inventory gate scope:** which inventories become blocking — every operation kind in corpus (already enforced), every benchmark family present per `docs/VALIDATION_STRATEGY.md` §2 table, every Must-FR with named test evidence? `TBD` enumeration.
8. **D-05 interaction (noted, not resolved):** coverage and tolerance sweeps must run in **local** evidence sweeps now (`tools/release/check_release_readiness.py` is the seam); whether/where they later run hosted is D-05 (`_REGISTER.md` row D-05; plan `:44`). D-04 should rule thresholds in a provider-neutral form.
9. **Report wording:** threshold statements rendered into reports stay slot data per DEC-021 (§2.6) — confirm D-04's record format is machine-consumable for those slots.

---

## 4. Options

Part 1 and Part 2 are separable; the ruling may mix (e.g., T-C + C-C).

### Part 1 — Numerical tolerance policy

#### Option T-A — Per-quantity-kind relative+absolute tolerance table as governed values

One governed table (policy doc + machine-readable record), keyed by quantity kind (displacement, rotation, force, moment, stress, …), each entry a relative tolerance plus absolute floor. Fixtures cite the table id in `tolerance_policy`; changes require a new governed record.

- For: single authority, DEC-018-style precedent (governed constants); fixtures stay thin; report slots cite one id; change tracking is automatic (new record = new evidence event).
- Against: one table must fit analytic, cross-engine, and regression evidence at once — the corpus's exact-equality class (§2.4) would be artificially loosened or need a carve-out; initial per-kind values are unmeasured (`TBD`), so the table launches mostly empty.

#### Option T-B — Single global relative tolerance with per-case fixture overrides

One global relative tolerance (e.g., the measured `1.0e-9`, §2.2) recorded as governed; any fixture may override locally in its `tolerance_policy` slot with a recorded reason.

- For: cheapest to state and adopt; current suites already pass at `1.0e-9` (`FACT`, §2.2), so the gate is immediately green; overrides keep hard cases honest.
- Against: D-01e rejected the analogous T2 for conversions as hiding drift and weakening regression sensitivity (`D-01_unit_catalog_acceptance.md:111`); per-case overrides scattered across fixtures invert the governed-values pattern and make silent loosening easy — exactly the fallback posture the diagnostics surface exists to prevent (§2.2); no absolute floor near zero.

#### Option T-C — Reference-result classes, each with its own tier; per-kind values within class

Three governed classes: (i) **analytic/hand-calc benchmarks** — relative+absolute per quantity kind, initial values seeded from the measured `1.0e-9` assertion level where suites already pass, others `TBD` until measured; (ii) **cross-engine/corpus comparisons** — exact equality on the canonical semantic projection, as already enforced (§2.4), codified rather than invented; (iii) **regression goldens** — byte/hash-exact on canonical artifacts (the `audit_manifest` discipline), no numeric tolerance. Tolerances live in one governed record; fixtures cite class+kind ids; fixture-local override only with a named justification record. Unresolved entries stay `TBD` and keep emitting `tolerance_policy_tbd_diagnostic` (§2.2).

- For: matches what the repo already does in each class instead of averaging across them; preserves the exact-equality strength where it is proven; keeps the no-silent-fallback diagnostic as the enforcement seam; tolerance changes are evidence-tracked by construction (governed record + witness re-run).
- Against: three classes to govern and explain; per-kind values for stress/nonlinear classes are genuinely unmeasured (`TBD`), so part of the table stays open at ruling time; slightly more record-keeping than T-B.

### Part 2 — Coverage thresholds

#### Option C-A — No numeric coverage gate; enforced surface inventories only

Extend the corpus-floor precedent (§2.4): blocking inventory tests assert (i) every operation kind/blocking class in corpus (existing), (ii) every benchmark family of `docs/VALIDATION_STRATEGY.md` §2 present with at least one passing fixture, (iii) every Must-FR names its test evidence in a tracked map. No percentage anywhere.

- For: deterministic, tooling-free, already proven in-repo; inventories measure *what matters* (named surfaces) rather than incidental line counts; zero new dependencies; immune to coverage-percentage gaming.
- Against: blind to untested branches inside covered surfaces; "every FR has *a* test" can be satisfied thinly; reviewers external to the project may expect a conventional coverage number at R5 (`ASSUMPTION`).

#### Option C-B — Numeric line/branch floors per language lane with named tooling

Adopt per-lane floors (Rust via cargo-llvm-cov or tarpaulin; TS via Vitest `--coverage` (v8); Python via coverage.py), wired into local sweeps; numbers blocking.

- For: industry-legible; catches dead branches inventories miss; one number per lane is easy to report.
- Against: **no tooling is configured and no baseline measurement exists** (§2.5) — any floor chosen now would be ungrounded (`TBD` by this packet's own rule); new toolchain dependencies (llvm-cov components, instrumentation) raise determinism and maintenance cost in a local-only posture; numeric floors invite false confidence and threshold-chasing; per-crate non-workspace layout (25 manifests, §2.4) makes aggregate Rust coverage awkward.

#### Option C-C — Hybrid: blocking inventory gates + numeric coverage as recorded-not-blocking telemetry

C-A's inventory gates are the blocking release-quality input. Numeric coverage is additionally *measured and recorded* in local evidence sweeps once tooling is selected (a named follow-up, not this ruling), with the numbers filed in gate records as telemetry — explicitly non-blocking until a later human ruling promotes any floor based on observed baselines.

- For: blocking gates stay deterministic and grounded today; numbers accumulate evidence for an informed future floor instead of an invented one; matches the gate doc's posture that evidence is collected or explicitly waived (`docs/RELEASE_QUALITY_GATES.md:43–55`); D-05-neutral (telemetry runs locally).
- Against: two mechanisms to maintain; "recorded-not-blocking" requires discipline so telemetry is not quietly treated as a gate (or quietly ignored); tooling selection still owed as follow-up work.

---

## 5. Recommendation — `PROPOSAL`

Adopt **T-C + C-C**:

1. **Part 1:** rule the three reference-result classes (analytic / cross-engine-exact / regression-golden-exact) as the governed tolerance structure. Record the policy as a machine-readable governed record (one file, DEC-018 constant-record pattern) that fixtures cite by class+kind id via the existing `tolerance_policy` slots (§2.2). Seed the analytic class at relative `1.0e-9` with absolute floor `TBD` for the quantity kinds whose suites measurably pass today (§2.2); every unmeasured entry stays `TBD`, keeps `tolerance_policy_is_unresolved()` true for its fixtures, and keeps `tolerance_policy_tbd_diagnostic` emitting — no silent fallback, no invented numbers. A tolerance change requires a new governed record plus re-run witnesses (evidence-tracked by construction). DEC-018's conversion tiers are referenced, not modified.
2. **Part 2:** rule inventory gates as the blocking coverage mechanism — (i) corpus kind/block-class floor (existing, §2.4), (ii) benchmark-family presence per the `docs/VALIDATION_STRATEGY.md` §2 table, (iii) Must-FR→test-evidence map — enforced in local sweeps via `tools/release/check_release_readiness.py` steps. Numeric coverage becomes recorded-not-blocking telemetry once a tooling follow-up (suggested D-04b) selects per-lane tools; no percentage floor is asserted now because none can be grounded in repo measurement (§2.5). CI residency of any of this remains D-05's question.
3. **Report/gate wiring:** the governed record's ids and values feed report slots per DEC-021 (§2.6) and the E7 gate records; `docs/RELEASE_QUALITY_GATES.md` §10 bullets 1 and 3 close to "ruled" with pointers, leaving performance thresholds (bullet 2) and CI (bullet 4) untouched.

Rationale: T-C is the only Part 1 option that neither loosens the proven exact-equality classes nor invents unmeasured numbers, and it turns the repo's existing TBD machinery (slots + diagnostic) into the enforcement seam instead of fighting it. C-C keeps today's gate deterministic and grounded (C-A's strength) while building the measured baseline C-B would need — the floor decision is deferred to evidence, not skipped. This recommendation is a `PROPOSAL` only. It confers no authority and changes no state.

---

## 6. Downstream impact map

| Surface | Impact of this ruling |
|---|---|
| **Plan B3 (mixed-unit round-trip + tolerance tests)** | Consumes Part 1's class structure alongside DEC-018's conversion tiers; "tolerance policy per D-04" (`plans/PLAN_2026-06-10_prd_completion.md:84`) becomes concrete. |
| **Benchmark crates (mechanics/stress/nonlinear)** | `ExpectedValue.tolerance_policy` slots fill with governed class+kind ids; `tolerance_policy_is_unresolved()` flips per fixture as entries leave `TBD`; stress crate's `final_tolerance_policy: "TBD"` gets a pointer (§2.2). |
| **Solver diagnostics / performance harness** | `tolerance_policy_tbd_diagnostic` retires only for surfaces whose class entries are ruled and non-`TBD`; it remains the no-silent-fallback guard for the rest (§2.2). |
| **R1 exit criterion** | "passes within documented tolerances" (`docs/PRD.md:1188`) gains its documents: governed record + witness runs. |
| **R4 / nonlinear evidence** | Caller-supplied convergence `tolerance` input (§2.2) gets a policy-bound default source; convergence evidence cites it. |
| **R5 validation manual + reproduction** | E2 assembles witnesses + accepted tolerances (`plan:121`); external reproduction (`docs/PRD.md:1252`) needs the governed record published with the manual. |
| **E7 gate records / RELEASE_QUALITY_GATES** | Gate records cite thresholds from D-04 (`plan:126`); §10 TBD bullets 1 and 3 close with pointers (`docs/RELEASE_QUALITY_GATES.md:156–160`). |
| **Local evidence sweeps** | `tools/release/check_release_readiness.py` gains inventory-gate steps and (post-D-04b) telemetry capture; stays provider-neutral pending D-05. |
| **D-05 (CI)** | Inherits ready-made, provider-neutral gate commands; D-04 asserts nothing about where they run (§3 Q8). |
| **Reports (DEC-021)** | Threshold statements render from governed-record slot data, not template prose (§2.6); renderer unchanged. |
| **Register row D-04** | `NOT_PREPARED` → `AWAITING_RULING` at fan-in (dispatching persona; not this packet). |

---

## 7. Authority and ruling record

Only the **human project authority** rules on D-04. Agents prepared this packet and may not certify, approve, or adopt it.

Per existing decision practice, the accepted ruling is recorded as a `DEC` entry in `execution/_Decomposition/SOFTWARE_DECOMP.md` §12 — D-01, D-08, and D-10 were recorded this way as `DEC-018`, `DEC-019`, and `DEC-021` (`_REGISTER.md` rows D-01/D-08/D-10) — after which the dispatching persona updates `execution/_Coordination/_DECISIONS/_REGISTER.md` row D-04 from `AWAITING_RULING` to `RULED` with a pointer. Any numeric value left `TBD` at ruling time remains `TBD` in code and diagnostics until a later governed record supplies it. This packet does not edit the register and does not resolve the decision.
