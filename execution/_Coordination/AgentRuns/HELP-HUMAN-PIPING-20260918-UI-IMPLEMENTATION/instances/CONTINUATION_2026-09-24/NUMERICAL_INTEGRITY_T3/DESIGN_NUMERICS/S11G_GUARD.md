# S11-G — the formation-noise guard (design note, revision 2.2)

D1 (Type 2 design TASK), 2026-09-27. Commissioned by ROOT ("I4's F12 stop", `ROOT_RULINGS_V1.md`, option (c)). Brief: `TASK_BRIEFS/D1_S11G_GUARD_NOTE.md` at `c24f8d5b4`, with ROOT's addendum at `42b300344`. Revision 1 (`633460fb4`, sha256 `5fa3be8f…`) is archived as `_run_records/S11G_GUARD_revision1.md`. Revision 2 (`a5137da0f`, sha256 `b414b88e…`) is archived as `_run_records/S11G_GUARD_revision2.md`. Revision 2.1 (sha256 `7c052c9e…`) is archived as `_run_records/S11G_GUARD_revision2_1.md`.

## 0.2 What changed in revision 2.2 (the routing gate only)

Revision 2.2 answers ROOT's "S11-G ruling 3, revision 1" and the brief `TASK_BRIEFS/D1_S11G_REV22.md` (both at `70907245b`). It changes only §3.5 item 2 (routing) and what follows from it: two further routing lines, one decline in the attempt chain, and the tests, mutations, write-set rows and forecast notes below. Everything else in revision 2.1 is unchanged.

**The defect in 2.1 (I5's path 2, verified here by code reading on main `72d5ff864`).** The 2.1 gate `source_eligible … && load_row_finding.is_none()` withholds the retained-source *attempt*. On a captured multi-case invocation where one case is source-selected, an already-Sensitive case whose load-row guard fires then gets no failure record, falls to `FinalizedSourceBlockCase::ordinary`, which refuses a non-checks-passed outcome (`source_receipt.rs:716-718`), and so has no receipt entry. `finalize_for` then fails coverage (`source_receipt.rs:955-961`), PP pushes the blocking `SOURCE_BLOCK_RECOVERY_FINALIZATION_FAILED` (PP:2057), and the captured entry returns `Err("SOURCE_BLOCKS_FINALIZATION_FAILED")` (PP:1484-1488).
- **Introduced by S11-G, not on main.** On main the same case is attempted (PP:2592-2593), retained scope refuses its element load (`source_recovery.rs:519-528`, `:590-601`), the `Err(failure)` arm records the failure and its `SOURCE_BLOCK_RECOVERY_UNAVAILABLE` diagnostic (PP:2625-2640), and `FinalizedSourceBlockCase::failed` gives an `unsupported` entry (PP:3684; `source_receipt.rs:752-791`). Coverage holds. This is S11-F's F6 shape (`s11f_tests.rs:1236`, passing on main); path 2 differs only in case B's loads, which make the guard fire and so engage the gate.

| ID | Change | Sections |
|---|---|---|
| G-1 | **The gate is removed.** `source_eligible` is main's predicate again (capture, no nonlinear support, no combinations) | §3.5 |
| G-2 | **A load-row finding routes like an ordinary Sensitive verdict:** `needs_source_recovery = report Sensitive ∨ attempted_linear Err ∨ load_row_finding.is_some()`, and the receipt's `OrdinaryAttempt` records outcome `sensitive` (expected code `NUMERICAL_INTEGRITY_SENSITIVE`) when the finding is `Some`. Both match what the envelope publishes after the guard's demotion | §3.5 |
| G-3 | **Selection is declined, not the attempt:** if an attempt would be selected while the finding is `Some`, it is converted into a failure through the existing decline pattern (`RecoveryFailure`, `RecoveryError::Unsupported`), giving the existing `failed`/`unsupported` receipt entry. By the admissibility argument this is reachable only in range-fallback corners; it replaces the gate as the defensive invariant | §3.5 |
| Path 2 | **Removed** (no refusal; the guard-fired case publishes Sensitive; for an already-Sensitive case the bytes equal main's) | §3.5, §8 (T18) |
| Path 1 | **Load-row variant removed** (G-2). **R-b′ variant remains** a disclosed, fail-closed residual under ruling 3, with a characterization test | §3.5, §8 (T19, T20), §11 |
| Tests | T1's diagnostic-set pin restated; T10 and T10b restated; new T18 (path 2), T19 (path 1, load-row variant), T20 (R-b′ residual characterization), T21 (decline unit test); M7 withdrawn; new M19 to M22 (each reintroduces path 2 or path 1) | §8 |
| Evidence | `_run_records/s11g_rev22_trace.py` pins every file:line claim of §0.2 and §3.5 item 2 against the product code of main `72d5ff864` (20 checks after erratum E-4, all true; file hashes recorded) | §10 |
| Forecast | Committed-byte forecast unchanged (zero). Frozen-reference harness: UDL-W1e8 on the captured entry gains one info diagnostic (`SOURCE_BLOCK_RECOVERY_UNAVAILABLE`, from the zero-work formation decline; erratum E-1); verdict and standing unchanged | §6.6 |


**Erratum to revision 2.2 (after V1's delta-2.2 check; ROOT "Selection: S11-G revision 2.2", binding condition D22-1).**
- **E-1 (D22-1). The claim that retained scope refuses the element load "before any charged work" is false.** `source_recovery.rs` precharges n²·16 + 32000·members + supports·(24n + 512) + 128·(loads + springs) at SRec:457-473, *before* its scope checks (SRec:474-528), and PP debits a failed attempt's charged work into the invocation ledger (PP:2628). As written, G-2's new attempt on a **Passed**, guard-fired case (one main does not attempt) would therefore consume invocation budget that neither main nor 2.1 spends: about 7.2e4 at UDL-W1e8's size, about 2.3e6 for a 40-member model against the 4e6 per-case limit. Near the 64e6 invocation limit, that could cost a later case its selection or refuse the invocation through the publication reservation: the same class of availability regression as path 2.
- **The fix (binding; V1's preferred remedy).** When main would not attempt the case (report Passed, `attempted_linear` not `Err`) and the load-row finding is `Some`, the formation decline is **recorded without running the attempt**: a zero-work `RecoveryFailure` (stage "formation guard", `AttemptStage::SourceClosure`, `RecoveryError::Unsupported`, work {limit 0, charged 0, rejected 0}), plus the same `SOURCE_BLOCK_RECOVERY_UNAVAILABLE` (info) diagnostic. `failure_fields` maps it to the existing `source_validation` / `unsupported_family`; the reader accepts it under `WORK_LEDGER` (0 ≤ 0 ≤ limit, rejected 0) and `FAILURE_CATEGORY` (`unsupported_family` ⇔ unsupported). The invocation ledger then equals main's. **Already-Sensitive guard-fired cases keep the real attempt** (main makes it too), so T18's byte-equality with main holds. G-3 stays for the admissible corners (V1 found it load-bearing for a 0.4.0 subnormal eigen load).
- **Tests added by E-1.** **T22:** on a Passed, guard-fired case (UDL-W1e8, captured), the invocation budget (`charged`, `failed_charged`, `attempts`) equals main's for the same request, and the receipt entry, where a receipt exists, is `unsupported` with zero work. **T23:** the reader (`result_export` and the desktop `sourceBlockRecovery.ts`) accepts the zero-work `unsupported` entry. **M23:** restore the charged attempt for a Passed, guard-fired case; killed by T22.
- **E-2 (D22-N1). The R-b′ residual's reach is wider than per-case modulus bases.** On main, FK's load audit makes a single case Sensitive whatever the stiffness (`structural.rs:1384-1404`), for example through an unaudited range row such as nodal terms (1e15, −1e15, 1e-300), which the captured entry accepts; after K-D5, its formation check does the same. So a single-basis captured invocation can pair a selected, load-audit-Sensitive case A with an INPLANE-type case B. From the code; not run. T19 and T20 may use this single-basis construction (which also settles D22-N3 if multi-basis selection does not confirm).
- **E-3 (D22-N4). T20 is pre-0.4 only.** On a 0.4.0 captured invocation the case is republished on the ordinary route (CP3 SF-1), not refused; R-b′ demotes on the rerun. T20 carries that as a comment and a one-line assertion.
- **E-4.** The trace check formerly labelled "retained scope refuses element uniform loads before charged work" is relabelled, and two checks now pin the precharge (SRec:457-473) and the ledger debit (PP:2626-2628): 20 checks, all true.

## 0.1 What changed in revision 2.1

Revision 2.1 answers V1's delta check (`REVIEW/S11G_CHECK.md`, delta section, at `a3ddc4dea`: DB-1, DS-1, DN-1 to DN-6) and ROOT's rulings ("S11-G revision 2 after V1's delta check", `ROOT_RULINGS_V1.md` at `21e1190b9`). Everything else in revision 2 is kept; V1 confirmed R-b′ (8 catches, 0 committed firings, the 12 synthetic), SF-1, SF-2, SF-3 and the K-D5 boundary.

| ID | Change | Sections |
|---|---|---|
| **DB-1** | **The SF-4 floor applies only to the self-equilibrated part of the defect.** Two exact accumulators per row: A_se (self-equilibrated terms' defects, floored) and A_net (every other formed term's defect, never floored). V1's counterexample (a UDL-type fixed-end defect at a free row, 14901× the criterion, masked at 0.0076 by two anchored thermal members) now fires at 14901×, and so do the other two (14901×, 93×). The collinear and pressure runs stay silent (same figures), the 6 UDL catches are unchanged, and SF-3's gate stays unreachable. New test T6b and mutation M17 | §3.3, §3.4, §6.7, §8 |
| DN-4 | **Residual disclosed:** a junction of self-equilibrated terms only, with a genuine small net, is hidden by any noise-silencing floor (126× in V1's probe). Routed to W1/F2, which removes it; recorded for S11-G's CHANGE_RECORD | §3.4, §6.7, §11 |
| **DS-1** | **K-D5 does not demote the INPLANE cases after S11-F.** Revision 2's 1.6× and 2.0× came from `recal_d5.json`'s pre-S11-F folded-force solve. In the S11-F state, 2·EF = 1.7e-6 to 3.2e-6 of the trigger (recal's folded-force-free figure; V1's exact-residual EF: 1.3e-6 to 3.2e-6). **R-b′ is the only catch** for the INPLANE rows: its tests are load-bearing | §1, §5, §6.4, §11 |
| DN-1 | A skew-member unit test of B (T17), which kills M14's \|Tu\| variant that survives T11 on the axis-aligned INPLANE members | §8 |
| DN-2 | The FMA error term is exact only without underflow (\|a·b\| ≥ 2⁻⁹⁶⁹ or a·b = 0); otherwise the term falls back to `Bounded`. The 12× scaling of a value above about 1.5e307 overflows into `SumError`, which fires (fail-closed) | §3.3, T14 |
| DN-3 | The criterion constant is the exact rational 10⁻⁹ in the exact decision, or RD(1e-9) in binary64 (the binary64 literal 1e-9 is 6.2e-17 above 10⁻⁹) | §3.4 |
| DN-5 | A source pin ties the routing site to the tested `source_eligible` predicate (T10b) | §8 |
| DN-6 | Recorded: R-b′'s floor margin on the F-case INPLANE rows is 1.72 and depends on the product forming S\*_moment = 100 from its published rows | §5 |
| Inputs | I4's `formation_rows.json` is now pinned by full hash in `_run_records/inputs/i4_formation_rows.json` (ROOT); the forecast reads it there | §10 |


## 0. What changed in revision 2

Revision 2 answers:
- V1's check (`REVIEW/S11G_CHECK.md` at `30233f93a`: NOT READY, B-1 and SF-1 to SF-4);
- ROOT's rulings ("S11-G note: rulings" and "S11-G after V1's S11G_CHECK", `ROOT_RULINGS_V1.md`);
- the manager's relays of those rulings.

Everything else in revision 1 is kept.

| ID | Change | Sections |
|---|---|---|
| **B-1** | **The recovery guard now ships.** R-b is added to the design, together with the variant R-b′: R-b restricted to rows at or above the DESIGN floor 2⁻³⁴·S\*. See the note after this table. | §1, §4, §5, §6.4, §6.5, §6.6 |
| SF-1 | The exact defect is **one exact accumulator of 12·Σε with no binary64 intermediate**. The fire decision is made exactly against the threshold. | §3.3, §3.4 |
| SF-2 | The bound fallbacks are now conservative. Exact-pressure operands use γ₁₆·\|t\| (a products-only chain), not half an ulp. The curved consistent vectors become **cannot-bound**, which demotes (ROOT). | §3.2 |
| SF-3 | The `source_eligible` gate is **unreachable end to end** once SF-4's floor is in place, and this is shown. It is kept as a defensive invariant with a unit test. T10 and the M7 and M8 claims are restated. | §3.5, §8 |
| **SF-4** | **Fixed, not only disclosed (ROOT).** A per-row S\* floor is added: 2⁻¹⁰·Σ\|self-equilibrated formed terms\| at the row. V1's collinear straight runs (pure thermal and pure pressure) go silent, with margin ≥ 1.35e4. The six UDL catches are unchanged. No demotion is added. V1's probe becomes a test that must stay silent. | §3.4, §6.7, §8 |
| NOTEs | **N-1:** the INPLANE per-row labels are re-taken from I4's measured records (F case: M1.j 3635 dense and 2214 sparse; M2.i 628 dense and 4346 sparse). **"The error is in u" is qualified:** exact recovery still leaves 28–2949×, and F2's stop rule resolves it (5.42e-18 < 1e-17). N-3 to N-7 are addressed. | §2, §3.1, §3.3, §4, §5 |

**The B-1 forecast.** Both R-b and R-b′ are forecast by script over three sets:
- the committed product outputs (every committed result envelope);
- the committed models without an envelope (by exact emulation);
- the frozen references, including the 221 and the 8 INPLANE rows.

For each firing row the forecast gives today's predicate, whether the case is Passed today, its class (committed, frozen realistic or synthetic) and K-D5's overlap.

**The B-1 result.**
- Both variants catch all 8 INPLANE rows.
- Neither demotes a committed case, and neither touches a committed byte.
- R-b fires on one committed row, which is correct and sits in an already-Sensitive case. R-b′ fires on none.
- On the frozen references, R-b falsely demotes 16 Passed case-modes (4 of them realistic) and R-b′ falsely demotes 12 (all synthetic).
- **ROOT's default therefore selects R-b′.**

**Additions to the basis.**
- `REVIEW/S11G_CHECK.md` (`fb8a879d`), and V1's probes in `REVIEW/_run_records/s11g_check/`.
- I4's measured post-S11-F formation rows: `IMPLEMENTATION/S11F/_run_records/formation_rows/formation_rows.json` on the S11-F branch. It was read from I4's worktree, uncommitted, as sha256 `c548f519…`. Every INPLANE figure in §2 and §5 comes from it.
- `_run_records/recal_d5.json`, for K-D5's overlap.

No product code was written or built.

---

## 1. Answer in brief

**The load-row guard.** The design is unchanged; SF-1, SF-2 and SF-4 are repairs.
- It catches all 6 load-formation rows: UDL-W1e80 (typed) and UDL-W1e8 (captured and typed), in both modes.
- It demotes none of the following:
  - the 221;
  - any frozen-reference row that passes today, including UDL-W1e5;
  - S11-F's probe A;
  - V1's collinear straight runs, now silenced by the SF-4 floor, which applies only to the self-equilibrated part of a row's defect (revision 2.1, DB-1), so it cannot hide a net formation defect;
  - any committed model.

**The recovery guard R-b′ (B-1; ROOT's default).**
- **The rule.** R-b′ fires at a straight member end when all three hold:
  - B > 1e-9·q;
  - q > 2¹⁰·B;
  - q ≥ 2⁻³⁴·S\*_moment.
- **The quantities.** q is the end's published bending magnitude. B is the formation-noise bound of the formed K_e·u.
- **What it catches.** All 8 INPLANE rows. Each is a published Passed breach under today's predicate, measured by I4 at 628–5767× the criterion.
- **Committed fixtures.** It fires on no committed product output and no committed model.
- **Frozen false demotions: 12 synthetic case-modes,** whose rows are correct under today's predicate:
  - RF-INVARIANCE-LFRAME-BASE, -OFF-1e3, -OFF-1e6 and -RELABEL (8 case-modes, borderline at 1.04× the R-b threshold);
  - RF-WEAK-W-3D-rho1e-08 and -AX-rho1e-08 (4 case-modes).

**R-b as written** adds to that:
- 4 realistic-scale false demotions (RF-LARGE-CONT-n00100-AX and -ROT, below the floor), 16 in all;
- one firing on a committed row, in `load_reference_fallback_uz`. The case is already Sensitive and the row is correct, so no verdict and no byte changes.

**The committed-byte forecast is zero** for the load-row guard plus either recovery variant (§6.6).
- The load-row guard and R-b′ fire on no committed case.
- R-b's only committed firing is on an already-Sensitive case. There the demotion is a no-op by rule (§3.5), so no text is appended.

**No envelope field is added, so there was no stop.**

**K-D5 is disjoint** in the PP functions it edits (§9).
- **K-D5 does not demote the INPLANE cases after S11-F** (revision 2.1, V1 DS-1): 2·EF = 1.7e-6 to 3.2e-6 of its trigger. **R-b′ is the only catch** for those rows. Revision 2's 1.6× and 2.0× came from a pre-S11-F folded-force solve and are withdrawn.
- It does not demote LFRAME or WEAK-W-3D/AX either.
- I3 should expect K-D5 to stay silent on the INPLANE cases and not "fix" that (ROOT).

---

## 2. The seven triples after S11-F (I4's measured values)

| Triple (entry) | Rows | Class | Published after S11-F (I4) | Ratio to the criterion (dense / sparse) | S11-G |
|---|---|---|---|---|---|
| UDL-W1e80 th.S1.RZ (typed) | 2 | Load formation | 1.2184480799204924e57 rad | 2.63e81 / 2.63e81 | **Caught** (load-row guard) |
| UDL-W1e8 th.S1.RZ (captured, typed) | 4 | Load formation | 2.2754051790793294e-8 rad | 47.99 / 47.99 | **Caught** (load-row guard) |
| F-G1e80-GnG-INPLANE Mb.M1.j (typed) | 2 | Formed K_e·u | 1.0000036354540498e-8 / 1.0000022143685783e-8 N·m | **3635 / 2214** | **Caught** (R-b′; R-b) |
| F-G1e80-GnG-INPLANE Mb.M2.i (typed) | 2 | Formed K_e·u | 9.999993721976352e-9 / 1.0000043459967856e-8 | **628 / 4346** | **Caught** (R-b′; R-b) |
| M-G1e80-GnG-INPLANE Mb.M2.i (typed) | 2 | Formed K_e·u | 1.000005767082257e-8 / 1.0000043459967856e-8 | 5767 / 4346 | **Caught** (R-b′; R-b) |
| M-G1e80-GnG-INPLANE Mb.M2.j (typed) | 2 | Formed K_e·u | 9.999993721976352e-9 / 9.999993721976352e-9 | 628 / 628 | **Caught** (R-b′; R-b) |

**Coverage: 14 of 14 formation rows caught.** The formation list can be empty when S11-G lands. ROOT then removes the rows from the pin, since this note does not edit GATE files.
- **The UDL rows.** The load-row guard acts on the ledger before the solve, so it catches them in every entry and mode that builds the case.
- **The INPLANE rows.** R-b′ acts on each case-mode's published end actions, so it catches them on the published values above. B is about 3.6e-13 to 8.9e-13. That gives B/(1e-9·q) = 11259–35527 and q/B = 11259–28147, both far from either threshold.

Revision 1's INPLANE per-row labels came from D1's emulation, with the F-case labels swapped. I4's measurement replaces them (V1 N-1).

---

## 3. The load-row guard

### 3.1 Formed terms and input terms (unchanged)

- **Formed term.** A ledger term whose binary64 value the product computes from held operands, with at least one rounding. Its **formation defect** is ε_t = value_t − formula_t(held operands), evaluated exactly.
- **Input term.** Nodal loads and constant effort. It carries no defect.
- **Held operands.** The SI binary64 values the product holds and uses on both the force side and the recovery side:
  - `LoadApplication` and resolved-case quantities;
  - L, (a, b) and T;
  - a load record's `axial_load`.

The consequences are those of revision 1 §3.1:
- Exact input cancellation (the 221) cannot move the statistic.
- Same-expression negated terms cancel together with their defects (probe A).
- Different-expression terms keep their defect (UDL-W1e80).

**The held-operand boundary (V1 N-3).** Formation error made before a held operand is invisible to this guard:
- `axial_load` = fl(E·A·α·ΔT) or fl(p·A);
- decimal-to-SI conversion;
- T.

For example, equal intended thermal loads formed through different factorizations can meet at a junction without the guard seeing it. That is W1's class, like the model representation (L, A). Because these chains are products only, a later slice can extend `RoundedProduct` upstream cheaply. S11-G does not.

**A row-wise load criterion (V1 N-4)** does not bound the response error under ill-conditioning. That is K-D5's and F2's concern. K-D5's ρ could later use f − E to become response-level (§9).

### 3.2 Per-family formation records (SF-2 applied)

| Site (S11-F candidate) | Family | Class | Defect form | Operand bound β (relative to \|t\|) | Self-equilibrated (SF-4) |
|---|---|---|---|---|---|
| `push_nodal_loads` PP:2204 | Nodal loads | Input | none | 0 | — |
| `add_constant_effort_support_loads` PP:10739 | Constant effort | Input | none | 0 | — |
| straight `add_uniform_element_loads` PP:8635 | Straight uniform fixed-end terms | Formed | **Exact** (SP formula over exact expansions; scale 3) | 0; γ₄ for generated intensities (self-weight) | no |
| `add_pressure_thrust_loads` PP:8846–8847 | Straight thrust fl(P_ax·x) | Formed | **Exact** rounded product: −lo(a·b) | 0 | **yes** (±, same member) |
| `add_thermal_equivalent_loads` PP:8924–8925 | Straight thermal and eigen fl(N·x) | Formed | **Exact** rounded product | 0 | **yes** |
| curved thermal `push_product` PP:8948 | K_rc·fl(ε·chord_c) | Formed | **Exact** scaled rounded product: −K_rc·lo(ε·c) | 0 | **yes** (K·u_free of a rigid-free field) |
| curved pressure caps PP:8883–8884 | fl(P_ax·tangent) | Formed | **Exact** rounded product | 0 | **yes** (with the wall vector) |
| curved uniform PP:8595 and curved wall vector PP:8888 | CB `consistent_uniform_nodal_loads`, `consistent_radial_pressure_nodal_loads` | Formed | **CannotBound** (SF-2, ROOT) | — | no |
| `push_exact_pressure_operands` PP:2285 | D-14 exact-pressure group operands | Formed | **Bounded**: γ₁₆·\|t\| (SF-2) | — | no |

**Exact-pressure operands (SF-2).**
- **The chain.** `pressure_group_value` (`pressure_exact/source_geometry.rs`) forms `fluid_force_scaled(kernel, p)·Scaled(coefficient)·Scaled(direction)`. The coefficient is a correctly rounded sum, and the fluid force is p·A_i from the source bore. `Scaled::mul` rounds its mantissa once per product, and the output is rounded once more.
- **Why γ₁₆ is sound.** The chain is products only, so its relative error is at most γ_k for k roundings. V1 counts at least 5, and the fluid-force sub-chain adds a few more; γ₁₆ covers k ≤ 16.
- **The implementation obligation.** It counts k from the source and uses γ₂ₖ if k exceeds 8. Alternatively, it carries the chain as an exact product expansion, which is sound because the chain is products only.
- **Effect on the forecast.** None. The committed screen already charges every formed term 17·γ₁₆.

**Curved consistent vectors (SF-2; ROOT: cannot bound, which demotes).**
- **Why no bound is given.** A conservative bound would need three things this note does not have:
  - cond₁(F) of the arc flexibility (348 for elbows, 1.7e4 for R5-4's small-angle bend);
  - the cancellation in p_i = H·X + W_i;
  - a stated accuracy for libm's sin, cos and atan2.
- **The rule.** Every term of these two families is pushed as `Formation::CannotBound`. A case whose ledger holds any `CannotBound` term demotes to Sensitive, with a reason naming the curved span and the family.
- **Forecast effect.** No committed model or frozen reference loads a realized curved span, so no forecast changes (§6.3).
- **Cost, disclosed for ROOT.** An availability loss on real curved spans carrying uniform loads or pressure, until F3 gives an exact form or a proven bound.

### 3.3 Carriage in the ledger; `AssembledForce` unchanged (SF-1 applied)

```rust
pub enum Formation {
    /// scaled_intended: exact nonoverlapping expansion of scale*formula(held operands); scale is 1 or 3.
    Exact { scale: f64, scaled_intended: Vec<f64> },
    /// value = k*fl(a*b), k exact: defect = -k*lo(a*b), exact.
    RoundedProduct { k: f64, a: f64, b: f64 },
    /// |defect| <= bound (rounded upward).
    Bounded { bound: f64 },
    /// No conservative bound is available: the case demotes.
    CannotBound,
}
impl LoadLedger {
    pub fn push_formed(&mut self, source: impl Into<String>, dof: usize, value: f64,
                       formation: Formation, operand_bound: f64, self_equilibrated: bool);
}
impl AssembledForce {
    pub fn formation_rows(&self) -> Result<Vec<FormationRow>, SumError>;
}
```

**Storage and `Debug`.**
- Formation records sit in vectors parallel to `terms`. `ForceTerm`, `ForceTermKind`, `values`, `by_dof`, `evidence` and `finish` are unchanged.
- The new vectors are kept out of `AssembledForce`'s `Debug` (V1 N-6). Either `AssembledForce` gets a hand-written `Debug` that prints only today's fields, or the vectors live in a sibling struct without `Debug`. T9 pins zero committed-byte change.

**Two exact accumulators per row (SF-1; split by DB-1 in revision 2.1).** For each DOF d with a formed term, two `ExactAccumulator`s receive 12·Σε exactly, with no binary64 intermediate:
- **A_net,d:** the defects of every formed term that is **not** self-equilibrated (the Exact, RoundedProduct and scaled RoundedProduct terms of non-self-equilibrated families);
- **A_se,d:** the defects of the self-equilibrated formed terms (§3.2's last column).

Each term's defect enters its accumulator as follows:
- **Exact term:** `add_product(12, value)`, then `add_product(-12/scale, c)` for every component c of `scaled_intended`. The factor 12/scale ∈ {12, 4} is exact.
- **RoundedProduct term:** three `add_product(-4k, lo)`, with lo = fma(a, b, −fl(a·b)). The factor 4k is exact; if it would overflow, the term becomes `Bounded { bound: γ₂·|value| }`.
- **The FMA condition (V1 DN-2).** lo is the exact error of fl(a·b) only if a·b does not underflow: |a·b| ≥ 2⁻⁹⁶⁹, or a·b = 0. Otherwise the term becomes `Bounded { bound: γ₂·|value| + 2⁻¹⁰⁷⁴ }`.
- **Overflow of the 12× scaling (DN-2).** A |value| above about 1.5e307 makes 12·value overflow the accumulator's range; that is a `SumError`, and the row fires (fail-closed).

**Nothing is rounded before the fire decision (§3.4).** `FormationRow` carries:
- A_net,d and A_se,d themselves (or equivalently the exact comparison results);
- round(A_net,d)/12 and round(A_se,d)/12, for the diagnostic text only;
- the bound sum B_d, rounded upward;
- the self-equilibrated sum P_d = Σ|value_t| over self-equilibrated formed terms, rounded upward;
- the intended net n_d = round(12·Σ values − A_net,d − A_se,d)/12, used on the threshold side, which is rounded downward (§3.4).

**Range failure is fail-closed; never `Err`.**
- If an expansion component overflows or underflows, the term becomes `Bounded { γ₁₆·Σ|monomials| }`.
- If that bound is not finite, or a `SumError` occurs, the row fires.
- A test pins these paths (V1 N-5).

### 3.4 The statistic, the scale, the S\* floor (SF-4) and the exact decision (SF-1)

For each loaded row d of a connected body, two thresholds (revision 2.1, DB-1):

  T0_d = 10⁻⁹ · max(|n_d|, S\*_d)   (unfloored, for the net formation defect and the bounds)
  Tf_d = 10⁻⁹ · max(|n_d|, S\*_d, 2⁻¹⁰·P_d)   (floored, for the self-equilibrated defect only)

both rounded downward. The row fires if any of:
- B_d ≥ T0_d;
- |A_net,d| > 12·(T0_d − B_d);
- |A_se,d| > 12·Tf_d.

Each comparison is decided exactly: a copy of the accumulator receives `add_product(∓12, threshold)`, and its signum is read. **A net formation defect is never floored.** A `CannotBound` term makes the case fire.

**The constant (V1 DN-3).** The binary64 literal 1e-9 lies 6.2e-17 (relative) above 10⁻⁹. The threshold is formed as RD(10⁻⁹)·max(…), rounded downward, so that the decision is never less strict than the exact criterion.

**S\* (unchanged).**
- Free rows use the DESIGN §4.1.6 coupling over the body's free-row intended nets: fo = max(F, M/L_b) and mo = max(M, L_b·F).
- Restrained rows use the same formula over all of the body's loaded rows.
- The free-row-only S\* is what makes UDL-W1e80 fire. Its fixed-end rows carry 3.3e79; a threshold over all rows would be 3.3e70, above the 2.6e64 defect.

**The floor (SF-4, a required fix).** floor_d = 2⁻¹⁰·P_d, where P_d is Σ|t| over the **self-equilibrated formed terms at row d**: the straight thrust, thermal and eigen axial pairs, curved thermal K·u_free, and the curved pressure caps.

**Why the floor is sound.**
- A self-equilibrated formed term balances within its own element. At a row, its only defect is the final product rounding, |ε_t| ≤ u·|t| (the `RoundedProduct` family).
- So |A_se,d|/12 ≤ u·P_d, and |A_se,d|/(12·Tf_d) ≤ 2⁻⁵³/(1e-9·2⁻¹⁰) ≈ 1.1e-4. The self-equilibrated part can never fire on its own.
- **The floor touches nothing else (DB-1).** Revision 2 applied it to the whole row, and V1 showed that two cancelling self-equilibrated terms at a row then raise the threshold for a genuine net formation defect at the same row. In revision 2.1 the net defect is judged against the unfloored T0_d, exactly as without the floor.
- In V1's collinear runs, the free-row intended nets are N·Δx, where Δx is a one-ulp difference in held direction cosines. The nets are themselves held-operand noise, and that noise is what collapsed S\*.
- The physical response there is zero interior displacement with an exact member force N. That force is published and verified on the force-kind scale.

**What the floor can hide (residual, V1 DN-4; ROOT: accepted for now, routed to W1/F2).** Only self-equilibrated defects, each at most u·|t|.
- At a junction where **only** self-equilibrated terms meet and their intended net is genuinely small, that rounding can be large relative to the net: V1's example, two collinear skew thermal members with N1 = 1e6 and N2 = 999999.999, is 126× the criterion relative to the load row (§6.7), and is hidden.
- Any floor large enough to silence held-operand noise (above about 1.1e-7·P) hides every such case, since each defect is at most u·P. So no floor separates the two.
- Such rows need N1 and N2 to agree to about 1e-9, which is contrived. W1/F2's precision-p formation removes the class. The residual is disclosed here and in S11-G's CHANGE_RECORD.

**What the floor does not touch.**
- Any net formation defect (A_net), including one at a row where self-equilibrated terms also meet (V1's counterexample: 14901× fires).
- UDL fixed-end terms and nodal inputs are not self-equilibrated, so A_se = 0 and P_d = 0 for all 6 UDL rows. Their statistic and threshold are unchanged: 47.99 and 2.6e81 fire; UDL-W1e5 stays at 0.0395; probe A stays silent.
- The floor only raises the threshold for A_se, so it adds no demotion anywhere (§6.7).

**Precision.**
- T_d is rounded downward and B_d upward.
- There is no rounding on the defect side: A_net,d and A_se,d are exact.
- The decision is therefore conservative (fail-closed) and bit-reproducible.

### 3.5 What firing does (the no-op rule; routing revised in 2.2)

1. **Where the findings are formed.** PP computes the load-row finding after `finish_case_ledger` (PP:2490), and the recovery finding (§4) after the straight end actions are formed. Both go into one `FormationFinding` per case, in the new module `product_physics/src/formation_guard.rs`.
2. **Routing (revision 2.2; replaces 2.1's gate).**
   - **G-1: no gate.** `source_eligible` is main's predicate (`capture.is_some() && built.nonlinear_supports.is_empty() && model.combinations.is_empty()`, PP:2592). The load-row finding does not enter it.
   - **G-2: the finding routes as Sensitive.** `needs_source_recovery = (report Sensitive) || attempted_linear is Err || load_row_finding.is_some()`, and `ordinary_attempt` is formed with outcome `sensitive` and expected code `NUMERICAL_INTEGRITY_SENSITIVE` when the report is Sensitive **or** the finding is `Some` (a constructor argument on `OrdinaryAttempt::passed`; the wire form is unchanged). The load-row finding is computed before both (after `finish_case_ledger`), so both are known at routing. **Erratum E-1:** for a case main would not attempt (report Passed, no `Err`), the decline is recorded as a zero-work `RecoveryFailure` without running the attempt; only already-Sensitive (or `Err`) cases are attempted, as on main.
   - **G-3: selection is declined.** In the attempt chain, before the 0.4.0 replay reservation: `.and_then(|recovery| if load_row_finding.is_some() { Err(recovery.decline_formation()) } else { Ok(recovery) })`. `decline_formation` mirrors `decline_withheld` (`source_recovery.rs:222-231`): stage "formation guard", `AttemptStage::SourceClosure`, `RecoveryError::Unsupported`, the executed work kept charged. `failure_fields` maps it to the existing stage `source_validation` and code `unsupported_family`. **No receipt field, code or wire form changes.**
   - **Why 2.1 withheld the attempt, and why that is not needed.** The gate protected against one event: retained-source recovery *selecting* a guard-fired case. Recovery solves the same represented `AssembledForce`, so a selected case would be receipt-qualified (standing `numerically_eligible`, as P1 shows for every Sensitive case with a selected recovery) while its load row is wrong. SF-3's numeric claim stands: recovery admits only nodal loads and load-state eigen terms (`source_recovery.rs:486-528`, `:590-601`), and on such a case the guard cannot fire (A_net = 0; A_se at most 1.1e-4 of its floored threshold). **SF-3's conclusion was wrong:** eligibility (the gate) is decided *before* admissibility, which is decided *inside* the attempt. The gate is therefore reached on exactly the guard-fired cases recovery would attempt and refuse, and what it withheld was a refused attempt whose `failed` entry keeps receipt coverage. The protected event is prevented instead by admissibility, and, for the range-fallback corners where a self-equilibrated eigen term takes a `Bounded` fallback on the unfloored side (an FMA underflow at |a·b| < 2⁻⁹⁶⁹, or a `SumError` above about 1.5e307), by G-3.
   - **Soundness of giving the case the attempt (brief item 2).**
     - *No guard-fired case publishes a value that is Passed or better than Sensitive.* The guard's demotion is applied at `append_integrity_report` whatever the attempt's outcome. If the attempt fails (always, except the corners), publication is the ordinary route with code `SENSITIVE` and a non-qualified (`unsupported`) receipt entry. If it would be selected, G-3 declines it, with the same result. No path selects, qualifies or exports a guard-fired case as better than Sensitive.
     - *The wire equalities hold.* `OrdinaryAttempt::wire` (`source_receipt.rs:527-563`) requires outcome `sensitive` ⇔ published `solve_quality = sensitive`, and the referenced diagnostic's code equal to `NUMERICAL_INTEGRITY_SENSITIVE`. With G-2, both hold whether the case was Sensitive by its report or by the guard. The readers check the same equalities (`result_export/src/source_blocks.rs:226-260`; the desktop's `sourceBlockRecovery.ts:178-183`), and `physics_source.rs:616`'s `SOURCE_FALLBACK_TRIGGER` requires an attempted case to record `sensitive` or `rejected`, which G-2 guarantees.
     - *The no-op rule and the diagnostic byte layout are unchanged.* `append_integrity_report` is untouched by 2.2. For an already-Sensitive guard-fired case the rule is a no-op, and the case's diagnostics, rows and receipt entry are exactly main's (the refused attempt main already makes). For a Passed guard-fired case, the demotion and sentence are 2.1's; the only addition is the `SOURCE_BLOCK_RECOVERY_UNAVAILABLE` (info) diagnostic, on the captured entry only, from the zero-work formation decline recorded without an attempt (erratum E-1).
   - **Path 1 (brief item 4).** Path 1 needs a guard-demoted Passed case beside a source-selected case, so the second case must have a different ordinary verdict: per-case modulus bases (I5's construction), or a per-case Sensitive from S11-K's load audit or K-D5's formation check (both on the kernel report, per ROOT's corrected reach). From the code, not run.
     - **The load-row variant is removed by G-2:** the case is attempted, refused (its guard-firing family is outside retained scope), and gets a `failed` entry whose ordinary outcome `sensitive` matches the envelope.
     - **The R-b′ variant remains.** R-b′ is formed after routing, from the published end actions, so the case's ordinary outcome was recorded `checks_passed` and no attempt was made; the only entry available is `ordinary`, whose wire check then fails ("ordinary outcome changed"). Removing it in-design would need R-b′ before routing (from the ordinary solution, with S\* from recomputed end actions) and would route R-b′-fired nodal-only cases to exact recovery, which may then *select* them: a change of R-b′'s semantics, not a routing-gate delta. It stays a disclosed, fail-closed residual under ruling 3 (owned by `SOURCE_BLOCKS_FINALIZATION_FAILED`), with the characterization test T20; the option is recorded for ROOT (§11).
   - **The recovery finding is formed after routing and does not enter it.** Where retained-source recovery is selected, the published member rows are its exact projections, not the formed K_e·u. R-b does not apply to them.
3. **The verdict.** `append_integrity_report` takes `formation: Option<&FormationFinding>`, at both call sites (PP:2790 linear, PP:2830 nonlinear).
   - When the ordinary code would be `CHECKS_PASSED`, the code becomes `NUMERICAL_INTEGRITY_SENSITIVE` (warning) and one reason sentence is appended to the existing message. The `StructuralReport` stays truthful (`quality: Passed`).
   - **The no-op rule (new).** If the case is already `SENSITIVE` or weaker, nothing is changed and no sentence is appended. The case's verdict and bytes are exactly today's. This is what keeps R-b's firing on the already-Sensitive committed `load_reference_fallback_uz` byte-neutral (§6.6).
4. **The effect.** `NumericalQualityStatus::Sensitive`, with standing `needs_recompute`. No refusal, no `Err`, and no new field, code or enum value.

**Granularity is the load case** (ROOT (c), accepted).

---

## 4. The recovery guard R-b and R-b′ (B-1)

**Scope.** The end bending actions of straight members whose published rows come from the formed K_e·u (PP:3075, `recover_local_forces_from_global_model`), on the linear and nonlinear ordinary routes.

**Not covered:**
- curved spans (E8/E9), whose recovery is CB's;
- torsion, axial and shear end rows;
- station rows, which derive from the end actions;
- reactions and displacements. V1 N-7: the same mechanism reaches LARGE-CONT's R.S14, R.S34 and u.C37, and F2/W1a is the full repair;
- member rows published from retained-source projections.

**The rule, per end e ∈ {i, j}:**
- **q** = hypot(My, Mz), the end's two published bending rows, taken after the exact subtraction of the fixed-end and axial terms (S11-F E5).
- **B** = γ₁₆·(Σ_k |K_loc[ry][k]|·Σ_c |T_kc|·|u_c| + the same for rz), with the product's own K_loc, T and binary64 u, rounded upward. It bounds the rounding of the formed K_e·u; the exact E5 subtraction adds nothing to it.
- **R-b** fires when B > 1e-9·q **and** q > 2¹⁰·B. The second clause requires the row to be resolved above noise: a row with q at noise level carries no relative claim for R-b to test.
- **R-b′** fires when R-b fires **and** q ≥ 2⁻³⁴·S\*_moment.
  - S\*_moment = max(S(moment), L_b·S(force)) is the body's coupled scale from the case's published rows (DESIGN §4.1.6.1: body membership, closed kind table, L_b).
  - 2⁻³⁴ is the V1-S8/S8-R floor. Below it, DESIGN guarantees only the absolute bound 2⁻⁶⁴·S\*.
  - The older constant 2⁻⁶⁴/1e-9 ≈ 5.42e-11 gives the same decision on every forecast row.

**Cost.** Per end: 24 absolute multiply-adds and one hypot, from quantities recovery already forms. S\* is formed once per case from its published rows.

**Where it lives.**
- SP gains `bending_formation_bound(&self, global_model_displacements) -> [f64; 2]`.
- PP records (member, end, q, B) at PP:3075, after `exact_straight_end_forces`.
- `formation_guard.rs` computes S\*_moment from the case's published rows and body membership, and applies the rule.

**What R-b′ is not.** It does not repair the INPLANE rows; F2's W1a does. It withholds them from Current until then.

---

## 5. The INPLANE rows are now in S11-G's scope

**The mechanism** (V1 (b)(i): independent of G after S11-F, and reachable on the captured entry at realistic scale).
- A small net (1e-8 N·m) is carried through a body in which the N1 load puts about 1e-13 N·m of formation noise on M2, relative to its rigid motion.
- Exact recovery from the product's binary64 u still leaves the rows 28–2949× over the criterion: 28–2774 in V1's product-faithful emulation, 145–2949 in D1's revision 1. Formation and u contribute comparably.
- F2's stop rule repairs them: 2⁻⁶⁴·S\* = 5.42e-18 < 1e-17 in the F case (margin 1.85), and 1.08e-18 in the M case. Their q is above the floor, with margin 1.7 (F) and 8.6 (M).

**Per row** (I4's measured values; B from R1's exact u):

| Row (typed, both modes) | I4 ratio (dense / sparse) | Passed breach today | B/(1e-9·q) | q/B | q/S\* (floor 5.82e-11) | R-b | R-b′ | K-D5 (S11-F state) |
|---|---|---|---|---|---|---|---|---|
| F Mb.M1.j | 3635 / 2214 | yes | 35527 | 28147 | 1.0e-10 | fires | fires | **silent** (2·EF = 2.1e-6 / 3.2e-6) |
| F Mb.M2.i | 628 / 4346 | yes | 88818 | 11259 | 1.0e-10 | fires | fires | **silent** |
| M Mb.M2.i | 5767 / 4346 | yes | 63949 | 15637 | 5.0e-10 | fires | fires | **silent** (2·EF = 1.7e-6 / 3.1e-6) |
| M Mb.M2.j | 628 / 628 | yes | 63949 | 15637 | 5.0e-10 | fires | fires | **silent** |

**K-D5's overlap: none (revision 2.1, V1 DS-1; ROOT amended its ruling 5).**
- **The correction.** Revision 2 read 2·EF = 1.60 and 2.00 from `recal_d5.json`'s `EF_ratio_coupled_Sstar`. For the RF-CANCEL cases that figure comes from a solve with the **pre-S11-F binary64-folded force** (`recal_d5.py` around l.126–151), so it measured the absorbed 1e-8 load that S11-F repairs, not the formation rows.
- **In the S11-F state** the solve sees the exact net. recal's matching figure (`EF_ratio_coupled_with_folded_f`) gives 2·EF = 1.7e-6 to 3.2e-6 of the trigger, and V1's exact-residual EF with the S11-F force gives 1.3e-6 to 3.2e-6. K-D5 stays silent.
- **So R-b′ is the only catch** for the INPLANE rows; there is no defence in depth. Its tests (T11, T17, M14) are load-bearing, and the implementation review mutation-checks them hard (ROOT).
- **For I3:** expect K-D5 silent on these cases after S11-F, and do not "fix" that.
- **Margins (V1 DN-6).** R-b′'s floor clause holds on the F-case rows by 1.72, which depends on the product forming S\*_moment = 100 from its published rows.

---

## 6. Forecast

**Scripts.**
- `s11g_forecast.py` (revision 1): Parts A, A2, B and C.
- `s11g_rb_forecast.py` (new): Parts C, E, F and S. It runs from T3/ with `<P>` = `projects/chirality-piping` and I4's formation_rows record, and it is deterministic under different `PYTHONHASHSEED` values.

### 6.1 Load-row guard on the frozen references (unchanged)

- UDL-W1e8 (47.99) and UDL-W1e80 (2.6e81) fire.
- UDL-W1e5 (0.0395) and probe A (margin ≥ 3e7) stay silent.
- Every other R1 case is nodal-only, with E = 0.
- The SF-4 floor is 0 on all of these rows.

### 6.2 The 221 and RF-SKEW

- **Load-row guard.** The 221 are nodal-only, so E = B = 0 by construction. RF-SKEW is nodal-only too.
- **R-b and R-b′ on the 221.** On the 18 cases of the 221, in the post-S11-F state (R1's exact u), they fire on **none**. The two INPLANE cases are not among the 18; their only post-S11-F breaches are the formation rows.
- **R-b and R-b′ on RF-SKEW.** They fire on RF-SKEW-T-PIN-OFF-122 and -345 at r1e-08. Those rows are breaches today, in cases already Sensitive, and K-D5 demotes them too, so nothing changes. They fire on no other RF-SKEW case.

### 6.3 Load-row guard on the committed models (unchanged, plus SF-2 and SF-4)

- 164 entries in 98 files carry formed loads (622 rows).
- The minimum screen margin is 1577 under 17·γ₁₆ per term, and the floor only raises thresholds.
- **SF-2, curved `CannotBound`.** No committed model realizes a loaded curved span (bends are node markers), so no case is affected.
- **SF-2, exact pressure at γ₁₆.** This is within the screen's 17·γ₁₆.

### 6.4 R-b and R-b′ on the frozen references (Part F: 196 cases evaluated)

**Not evaluated: 20 cases.**
- The 6 RF-LARGE-*-n01000 cases: their exact displacement sets are incomplete. P1 records a mismatch or timeout for them, except CONT-n01000, which passes.
- The 6 n10000 generator cases.
- The 8 mechanism cases.

| Case | Class | Rows | B/(1e-9·q) | Today's predicate (P1) | Case Passed today | R-b | R-b′ | K-D5 |
|---|---|---|---|---|---|---|---|---|
| F- and M-G1e80-GnG-INPLANE (4 case-modes) | synthetic | 8 formation rows | 11259–88818 | **breach** (I4) | yes | **catches** | **catches** | **silent** in the S11-F state (2·EF ≤ 3.2e-6); R-b′ is the only catch |
| RF-INVARIANCE-LFRAME-BASE, -OFF-1e3, -OFF-1e6, -RELABEL (8) | synthetic | Mb.M2.i / Mb.E1.j | **1.04** (borderline) | correct | yes | false demotion | false demotion | no |
| RF-WEAK-W-3D-rho1e-08, -AX-rho1e-08 (4) | synthetic | Mb.A01.j, Mb.A2.j | 197, 237 | correct (row-relatively 4–13× wrong, per V1) | yes | false demotion | false demotion | no |
| RF-LARGE-CONT-n00100-AX, -ROT (4) | **realistic scale** | Mb.B14.j, A15.i, A37.j, B37.i | 103–2805 | correct | yes | false demotion | **silent** (q/S\* = 1.3e-11 to 3.8e-11, below the floor) | not emulated (> 12 members) |
| RF-LARGE-TREE-n00100-ROT (2) | realistic scale | Mb.Q37/39/44/46.i | 1.05–1.35 (borderline) | correct | **no** (Sensitive) | no change | no change | not emulated |
| RF-SKEW-T-PIN-OFF-122 and -345 at r1e-08; RF-WEAK-W-L-r1e-08 (6) | synthetic | Mb.M1.i, Mb.M2.i | 2666–6743 | breach | **no** (Sensitive) | no change | no change | demotes |
| RF-INVARIANCE-LFRAME-ROT-Q3, -ROT-Q9, -UNITS-mm | synthetic | Mb.M2.i | 1.04–1.75 | not authorable (P1 did not run them) | — | no product run | no product run | — |

**Totals.** A false demotion is a row correct under today's predicate, in a case Passed today.
- **R-b:** 16 case-modes. 12 are synthetic (8 LFRAME, 4 WEAK) and 4 are frozen realistic (LARGE-CONT). This matches V1's 16.
- **R-b′:** 12 case-modes, all synthetic.

**Borderline.** The LFRAME rows sit at 1.04× R-b's threshold on both R1's exact u and P1's observed u. The product's own B, with its exact K and section, decides them, and they may fall on either side. They are listed as false demotions, which is the conservative choice.

### 6.5 R-b and R-b′ on the committed models

**C: committed product outputs.** This part uses every committed result envelope, with the product's own published u, q and S\* rows.
- **Coverage.** 87 case-envelopes with member rows, holding 248 straight end rows.
- **Skipped (3 envelopes).** One synthetic receipt control with no node positions, and two envelopes with no displacement rows.
- **Result.** R-b fires on exactly one row, in both modes:

| Envelope | Case | Row | q (published) | q (exact) | Row-relative error | B/(1e-9·q) | q/B | Floor | Case today | R-b | R-b′ |
|---|---|---|---|---|---|---|---|---|---|---|---|
| `core/reporting/result_export/tests/fixtures/load_reference_fallback_uz-{dense_scrutiny,sparse_interactive}.raw.json` | case:join | member end i | 2.000000000002531e-6 (dense); 2.0000000000000025e-6 (sparse) | 2e-6 | 1.3e-12; 1.2e-15: **correct** | 17272 | 57896 | below (2⁻³⁴·S\* = 1.04e-5) | **Sensitive** (ordinary route, no source recovery) | fires, **no verdict change** (no-op rule) | silent |

**The exact reference for that row.**
- The model is a 2 m member with a free tip and a root anchor. The root has prescribed motions (UY 1 mm, RZ 1e-4). The tip carries the added UZ load of 1e-6 N and a torque.
- The prescribed motions and the eigen strain produce only rigid motion and axial response, with no bending, because the cantilever is determinate.
- So My(root) = 1e-6 × 2 = 2e-6 N·m exactly, and Mz = 0.
- B is large only because of the millimetre-scale rigid motion times the member stiffness.

**The rest of Part C.**
- K-D5 adds nothing to this case, which its ordinary report already makes Sensitive.
- No other committed end row that is resolved above noise (q > 16·B) comes within 10³ of R-b's first threshold (B > 1e-12·q).

**E: committed models without an envelope.** Exact emulation of 96 distinct committed models; 70 load cases were emulated exactly.
- **Result.** R-b and R-b′ fire on none.
- **Not emulated,** with the reason for each:
  - pressure regions and element pressure loads: all these models have committed envelopes, covered in Part C;
  - nonlinear or `line_stop` supports and constant-effort supports: all covered by envelopes in Part C;
  - the model-operations contract corpus: a single member pinned in three translations only. That is a mechanism, which the product refuses, so there are no member rows;
  - models with no member or no load case;
  - the synthetic receipt control;
  - the two desktop UI workloads (`analysis_status.mechanics = not_run_generated_ui_workload`), which no committed path solves.

**The committed-fixture answer for ROOT.**
- R-b′ fires on no committed row.
- R-b fires on one correct committed row, in a case that is already Sensitive. Under the no-op rule that demotes nothing and changes no byte.
- **Neither variant demotes a committed row that is correct under today's predicate.**

### 6.6 Committed-byte forecast

**The forecast is zero, for the load-row guard plus either variant.**
- The ledger and SP values, and their `Debug` output, are unchanged (§3.3).
- The load-row guard fires on no committed case.
- R-b′ fires on no committed case.
- R-b fires only on `load_reference_fallback_uz`, which is already Sensitive. By the no-op rule (§3.5), its code, its message and every other byte stay as they are.

**The zero-regeneration-diff rule stands, and any diff is a stop.** ROOT's B-1 approval is not needed for a committed demotion, because there is none.

**Revision 2.2 leaves this forecast unchanged.** G-1 to G-3 act only on a case whose load-row guard fires, and the load-row guard fires on no committed case (§6.3), so no committed attempt, receipt entry or diagnostic changes. R-b′ is untouched.

**What does change is frozen-reference harness output,** which is not committed bytes:
- UDL-W1e8 (both entries) and UDL-W1e80 (typed), by the load-row guard. Revision 2.2 adds, on UDL-W1e8's captured entry only, one `SOURCE_BLOCK_RECOVERY_UNAVAILABLE` (info) diagnostic from the zero-work formation decline (erratum E-1: recorded without running the attempt, so the invocation ledger equals main's); verdict `SENSITIVE` and standing `needs_recompute` are unchanged. UDL-W1e80 (typed entry, no capture) and the INPLANE cases (typed) are unchanged;
- the INPLANE cases (typed), by R-b′;
- LFRAME ×4 and WEAK-W-3D/AX-rho1e-08, by R-b′ (false demotions);
- for R-b, add LARGE-CONT-n00100 AX and ROT.

### 6.7 SF-4: the floor, split by DB-1 (Part S)

Three rules are evaluated on every probe row: no floor; revision 2's whole-row floor; and revision 2.1's split rule (the floor on A_se only). D1's own arithmetic in SP operation order; V1's `delta_r2/probe_sf4` gives the same figures.

**Must stay silent: V1's collinear runs and a pressure run** (every term self-equilibrated, so A_net = 0):

| Run (end point; stations) | Load | No floor | Revision 2 (whole row) | **Revision 2.1 (split)** |
|---|---|---|---|---|
| (12, 5, 0); 0, .13, .4, .55, .81, 1 | thermal N = 1.296e6 | 4.0e8 (fires) | 6.2e-5 | **6.2e-5** (silent) |
| (10, 3.7, 2.2); 0, .3, .55, .7, 1 | thermal | 1.9e8 (fires) | 3.2e-5 | **3.2e-5** |
| (6, 6, 0); quarters | thermal | 0 | 0 | **0** |
| (9, 0, 0); 0, .13, .4, .55, .81, 1 | thermal | 6.2e8 (fires) | 3.5e-5 | **3.5e-5** |
| (12, 5, 0); as the first run | pressure thrust 2 MPa·π·0.09² | 2.3e8 (fires) | 7.4e-5 | **7.4e-5** |

**Must fire: V1's DB-1 counterexamples.** Model: S0 (0,0,0), S2 (5,0,0), S3 (3,−2,0) and S4 (3,2,0) anchored; S1 (3,0,0) with free translations and restrained rotations. Member A: S0→S1 (L = 3, q_A along y). Member B: S1→S2 (L = 2, q_B along y). Members C: S3→S1 and D: S1→S4, along y, with thermal `axial_load` N each (self-equilibrated, cancelling exactly at S1). A nodal input n₀ at S1.UY. S1.UY is the body's only moving DOF, so the published displacement carries the load row's relative error.

| q_A; q_B; n₀; N | A_net/12 | No floor | Revision 2 (whole row) | **Revision 2.1 (split)** |
|---|---|---|---|---|
| 100000000.1; −150000000.15; 1e-3; 1e6 | −1.49e-8 | 14901 (fires) | 0.0076 (**hidden**) | **14901 (fires)** |
| same; N = 1e4 | −1.49e-8 | 14901 | 0.76 (**hidden**) | **14901 (fires)** |
| 12345678.9; −18518518.35; 1e-2; 5e5 | 9.3e-10 | 93 | 0.00095 (**hidden**) | **93 (fires)** |

**Residual (DN-4, disclosed): a self-equilibrated-only junction with a genuine small net.** Two collinear skew thermal members (direction (3, 1.7, 0.4)) with N1 = 1e6 and N2 = 999999.999 meeting at a free node: no floor 126 (fires); revision 2 and revision 2.1 both 6.5e-5 (hidden). Every noise-silencing floor hides this class (§3.4); W1/F2 removes it.

**Unchanged by the split floor:**
- UDL-W1e8 (47.99), UDL-W1e80 (2.6e81), UDL-W1e5 (0.0395) and probe A: A_se = 0 and P = 0 on their rows, so all three rules coincide;
- the 221, which are inputs;
- the committed screen: the net part is judged as in revision 1 (margin ≥ 1577 under 17·γ₁₆ on all terms), and the floor only raises the self-equilibrated part's threshold;
- SF-3: source-eligible rows have A_net = 0, so the gate stays unreachable.

**No demotion is added,** because the floored threshold only rises and the net part's threshold is revision 1's.

---

## 7. Write set (S11-G slice, on main after S11-F)

| File | Change |
|---|---|
| `FK/load_ledger.rs` | `Formation` (with `CannotBound`), `push_formed(…, self_equilibrated)`, parallel vectors, `formation_rows` (two exact accumulators per row: net and self-equilibrated, DB-1), `FormationRow`, a `Debug` that excludes the new vectors; unit tests |
| `FK/lib.rs` | Re-export only |
| `straight_pipe/src/lib.rs` (SP) | `equivalent_global_nodal_loads_with_spans_formed` (today's values plus `Formation::Exact`); `bending_formation_bound` (R-b's B) |
| `product_physics/src/formation_guard.rs` (new) | The load-row decision (§3.4, exact); the recovery decision (§4: R-b′ by default, R-b if ROOT so rules); body S\*; `FormationFinding`; reason sentences |
| `product_physics/src/source_recovery.rs` (revision 2.2) | `decline_formation`, mirroring `decline_withheld` (G-3) |
| `product_physics/src/source_receipt.rs` (revision 2.2) | `OrdinaryAttempt::passed` takes the formation verdict (G-2); constructor only, the wire form is unchanged |
| `product_physics/src/lib.rs` (PP) | Push sites PP:8595, 8635, 8846–8847, 8883–8888, 8924–8925, 8948 and 2285; the guard call after PP:2490; the recovery record at PP:3075; the routing lines in `solve_load_case` (revision 2.2: `needs_source_recovery` takes the finding, `ordinary_attempt` takes the finding, `decline_formation` in the attempt chain; `source_eligible` is left as on main); `append_integrity_report` (with the no-op rule) and its two call sites |
| `product_physics/src/s11g_tests.rs` (new) and `tests/s11f_site_test.rs` | The tests in §8 |

**Not touched:** SA, FK `structural.rs`, `solve_preview_reduced_system`, CB, `pressure_runtime`, the GATE files (ROOT's), and any library or code-rule data.

---

## 8. Tests and mutations

Every verdict pin carries a **paths-differ precondition**.

### 8.1 Behavioural tests

| ID | Test | Precondition (paths differ) | Pin |
|---|---|---|---|
| T1 | UDL-W1e8, captured and typed, both modes | The ordinary report is `Passed`, and A at S1.RZ is nonzero and exceeds the threshold. Without the guard the code would be `CHECKS_PASSED` | Code `SENSITIVE` and standing `needs_recompute`. The message's `StructuralReport` text shows `quality: Passed`. **Diagnostic code set (revision 2.2):** equal to the unguarded run's except for the integrity code, **plus**, on the captured entry only, one `SOURCE_BLOCK_RECOVERY_UNAVAILABLE` (info) from the zero-work formation decline (G-2, erratum E-1), and no other `SOURCE_BLOCK_*` diagnostic |
| T2 | UDL-W1e80, typed, both modes (captured asserted to refuse) | As T1 | As T1 |
| T3 | UDL-W1e5, both entries and modes | A ≠ 0 (1.94e-11) | `CHECKS_PASSED`; stat/threshold < 0.1 |
| T4 | Nodal (G, n, −G) at a free DOF (F-G1e80-GnG typed; F-G1e8-GnG both entries) | The binary64 fold differs from the exact net, and `formation_rows()` has no defect term | Not demoted by the load-row guard |
| T5 | Probe A: (G, 0.3, −G) and (0.3, G, −G) at 1e8 and 1e80 | The individual G-term defects are nonzero, and their sum is 0 | `CHECKS_PASSED` |
| T6 | A synthetic collinear skew pair with equal thermal `axial_load` | Each end term's defect is nonzero | `CHECKS_PASSED` (signed cancellation) |
| **T6a** (SF-4) | **V1's collinear runs (`probe_thermal_skew`) and the pressure run**, as product requests: straight runs between two anchors, with free interior nodes at irregular decimal stations | Without the floor the statistic is ≥ 1.9e8, computed in the test from the ledger rows | `CHECKS_PASSED`; stat/threshold < 1e-3 with the floor |
| **T6b** (DB-1) | **V1's DB-1 counterexample** as a product request: S0, S2, S3, S4 anchored; S1 with free translations and restrained rotations; A (L = 3, q_A = 100000000.1 N/m) and B (L = 2, q_B = −150000000.15 N/m) along x with uniform loads along y; C and D along y with thermal `axial_load` N = 1e6 each; nodal 1e-3 N at S1.UY | Revision 2's whole-row floor would be silent (0.0076, computed in the test from the ledger rows), while A_net at S1.UY is nonzero (−1.49e-8) and 14901× the unfloored threshold | `SENSITIVE`; the reason names S1.UY and loads A and B. Repeated with N = 1e4 (0.76 under the whole-row floor) |
| T7 | The SP formation variant against the existing function | — | Values bit-identical; the intended expansion equals the rational oracle (3·rotation_i at b = 1, a = 0 is qL²/4) |
| T8 | Site table: every `ledger.push*` is classified as input, formed family, self-equilibrated or CannotBound | — | A formed site using plain `push` fails |
| T9 | Committed regeneration | — | Zero committed-byte diff |
| **T10** (revision 2.2) | Unit tests on the routing predicates: `source_eligible(capture, nonlinear, combinations)` (main's; no finding argument) and `needs_source_recovery(report_quality, attempt_err, load_row_finding)`; and on `OrdinaryAttempt::passed(…, formation_sensitive)` | With the finding `None` and a Passed report, `needs_source_recovery` is false and the outcome is `checks_passed` | With the finding `Some`: `needs_source_recovery` true, outcome `sensitive`, expected code `NUMERICAL_INTEGRITY_SENSITIVE`; `source_eligible` is unaffected by the finding |
| **T10b** (DN-5, revision 2.2) | Source pin (site table in `tests/s11f_site_test.rs`, by function name and call count): the routing site in `solve_load_case` calls the tested `source_eligible` and `needs_source_recovery`, forms `ordinary_attempt` with the finding, and applies `decline_formation` in the attempt chain before the replay reservation | — | Fails if the routing site inlines, bypasses or reorders any of them, or reintroduces the finding into `source_eligible` |
| **T11** (B-1) | F- and M-G1e80-GnG-INPLANE, typed, both modes | The load-row guard does not fire (all N2 terms are inputs) and the ordinary report is `Passed`. Without R-b′ the case is `CHECKS_PASSED`, with I4's pinned breach values | `SENSITIVE`, with a reason naming the member end, q, B and S\*. I4's `FORMATION_PINS` values stay bit-identical; only the verdict changes |
| **T12** (B-1) | An accurate small-moment row below the floor: RF-LARGE-CONT-n00100-AX (captured, both modes) | R-b's two clauses hold on the product's B and q, computed in the test, so the two paths differ only by the floor clause | `CHECKS_PASSED` under R-b′. This kills the drop-the-floor mutation |
| **T13** (B-1) | The committed request `load_reference_fallback_uz`, which is already Sensitive | R-b's clauses hold at end i | The envelope is byte-identical to today's (the no-op rule) |
| **T14** (N-5, DN-2) | Range failure: a uniform load near 1e307, whose intended expansion overflows | The expansion overflow is observed | The row falls back to `Bounded`. A non-finite bound fires. `SumError` fires. Never `Err`. Also: a RoundedProduct whose a·b underflows (\|a·b\| < 2⁻⁹⁶⁹) takes the `Bounded` fallback; a value above 1.5e307 makes the 12× scaling overflow into `SumError`, which fires |
| **T15** (SF-2) | A realized curved span carrying a uniform load | The ordinary report is `Passed` | `SENSITIVE`, with the `CannotBound` reason |
| T16 | I4's F1/F11/F12 test, extended | — | Every formation-list breach is published non-Passed: the 6 UDL rows and the 8 INPLANE rows |
| **T17** (DN-1) | Unit test of `bending_formation_bound` on a skew member (direction (3, 1.7, 0.4), nonzero u at both ends with mixed signs) | Σ_c\|T_kc\|\|u_c\| differs from \|Σ_c T_kc u_c\| on that member (asserted) | B equals the hand-derived γ₁₆·Σ\|K\|·Σ\|T\|\|u\| (rounded upward) bit for bit |
| **T18** (path 2, revision 2.2) | I5's path-2 model on the captured entry, both modes: an N05-type invocation (the soft torsion spring makes both cases Sensitive), case A the nodal tip torques (1e8, 0.3, −1e8), case B a uniform load 1e8 N/m along y cancelled at the tip by a nodal −1e8 N, plus a tip RZ moment 1e8/3 + 0.2 N·m | Case B's report is Sensitive and its load-row guard fires (both computed in the test); with 2.1's gate the invocation returns `Err("SOURCE_BLOCKS_FINALIZATION_FAILED")` (M19) | `Ok(envelope)` with a receipt: case A `qualified` (selected), case B `unsupported` with ordinary outcome `sensitive`; no blocking diagnostic; case B's integrity code `SENSITIVE`; case B's diagnostics and rows byte-equal to the same request on the unguarded route (the no-op rule) |
| **T19** (path 1, load-row variant; revision 2.2) | A two-case captured invocation in which case A is Sensitive and source-selected and case B's report is Passed, with a load-row finding on case B. The second verdict comes from per-case modulus bases (I5's construction) | Case B's report is Passed and its guard fires; the invocation has a selected case. Without G-2 the receipt entry would be `ordinary` with outcome `checks_passed` and the wire check would refuse (M20, M21) | No refusal; case B `SENSITIVE`, receipt entry `unsupported` with ordinary outcome `sensitive`; case A `qualified` |
| **T20** (residual characterization; ruling 3) | The same two-case construction with case B INPLANE-type (R-b′ fires; the load-row guard is silent). **Labelled a characterization of a known residual, not desired behaviour; its comment references `SOURCE_BLOCKS_FINALIZATION_FAILED`** | R-b′ fires on case B (computed in the test) and case A is selected | The captured invocation returns `Err("SOURCE_BLOCKS_FINALIZATION_FAILED")`, no case value is published; the single-case captured invocation of case B alone is demoted (`SENSITIVE`) and not refused |
| **T21** (G-3, unit) | A recovery input admissible to retained scope (nodal only) with a synthetic `load_row_finding = Some` | Without G-3 the attempt is selected (asserted with the finding `None`) | With the finding `Some`: `Err(RecoveryFailure)`, `failure_fields` gives (`source_validation`, `unsupported_family`); the receipt entry is `failed`; the work stays charged |

### 8.2 Mutations (each must be killed)

| ID | Mutation | Killed by |
|---|---|---|
| M1 | Drop a formed term's defect (scaled_intended = scale·value) | T1, T2 |
| M2 | Tag a formed term as input (`push` at PP:8635) | T1, T2, T8 |
| M3 | Give input terms a bound of β = u | T4 |
| M4 | Replace the exact defect with the a-priori c = 16 bound | T3, T5 |
| M5 | Take the free-row S\* over all rows | T1, T2 |
| M6 | Sum \|ε\| instead of Σε | T5, T6 |
| M7 | *(withdrawn in revision 2.2: the gate no longer exists)* | — |
| M8 | Demote through `report.quality` before routing | T1 (report text shows `quality: Passed`) |
| M9 | Map a fired case to `Err` or a blocking diagnostic | T1 |
| **M10** (SF-1) | Combine the families in binary64 (revision 1's fl(round(A12)/12 + round(A1))) | A unit test with one Exact term and one RoundedProduct term whose defects cancel to below u·max. The exact decision is silent; the binary64 one fires |
| **M11** (SF-4) | Drop the floor | T6a |
| **M12** (SF-4) | Take the floor from all formed terms, not only self-equilibrated ones | T1 (for UDL-W1e8 the floor 2⁻¹⁰·6.7e7 hides the 2.36e-8 defect), T2 |
| **M13** (B-1) | Drop R-b′'s floor clause, so that R-b ships | T12 |
| **M14** (B-1) | Drop B's \|T\| factor (use \|Tu\|), or use signed sums in B | Signed sums: T11 (B collapses, and INPLANE goes silent). \|Tu\|: T17 (T11 cannot see it, because the INPLANE members are axis-aligned and T is a signed permutation; V1 DN-1) |
| **M15** (B-1) | Demote an already-Sensitive case again and append text | T13 |
| **M16** (SF-2) | Treat CannotBound as a zero defect | T15 |
| **M17** (DB-1) | Apply the floor to the whole row's defect (revision 2's rule: \|A_net + A_se\| against the floored threshold) | T6b |
| **M18** (DN-3) | Use the binary64 literal 1e-9 rounded to nearest in the threshold | A unit test at a row whose exact statistic lies between 10⁻⁹ and fl(1e-9) times the scale: the exact decision fires |
| **M19** (path 2) | Reintroduce 2.1's gate: `source_eligible … && load_row_finding.is_none()` | T18 (refusal), T10b |
| **M20** (path 1) | `needs_source_recovery` ignores the finding (report only) | T19 (the case gets an `ordinary` entry with `checks_passed`; the wire check refuses), T10 |
| **M21** (path 1) | `ordinary_attempt` from the report only (outcome `checks_passed` while the envelope is demoted) | T19 ("ordinary outcome changed"; with an attempt, the readers' `SOURCE_FALLBACK_TRIGGER` too), T10 |
| **M22** (G-3) | Drop `decline_formation` from the attempt chain | T21 |

---

## 9. PP conflict boundary with K-D5

| | K-D5 (I3) | S11-G |
|---|---|---|
| PP | Only the solve call inside `solve_preview_reduced_system` (PP:3965 in I3's tree) | Ledger push sites; one call after `finish_case_ledger`; the recovery record at PP:3075 (the straight branch of `solve_load_case`'s element recovery loop); the routing lines in `solve_load_case` (`needs_source_recovery`, `ordinary_attempt`, the attempt chain; revision 2.2); `append_integrity_report` and its two calls. Also `source_recovery.rs` (`decline_formation`) and `source_receipt.rs` (`OrdinaryAttempt::passed`), which K-D5 does not edit |
| FK | `structural.rs` (`finish_checked_factor`), `formation_check.rs`, `retained/*` | `load_ledger.rs` |
| SA | `structural_adapter.rs` | none |
| SP | none | The formation variant; `bending_formation_bound` |

- **No function is edited by both slices,** so the merge is clean in either order.
- **The two compose semantically.** Demotion is idempotent, and the no-op rule keeps S11-G silent on a case K-D5 has already made Sensitive.
- **A later option, not proposed here:** K-D5's ρ could use f − E, to become response-level (V1 N-4).

---

## 10. Run records (`_run_records/`)

| File | Content |
|---|---|
| `s11g_forecast.py`, `.json`, `.stdout.json` | Revision 1: load-row guard Parts A, A2, B and C (unchanged) |
| `s11g_rb_forecast.py` | Revision 2, updated in 2.1: Part C (committed envelopes, the product's own values); Part E (exact emulation of committed models without an envelope); Part F (frozen references on R1's exact u and P1's observed u, today's predicate, the 221, INPLANE with I4's values, K-D5 from `recal_d5.json`, with the S11-F-state figure for the RF-CANCEL cases, DS-1); Part S (the SF-4 floor under three rules: V1's collinear and pressure runs, the DB-1 counterexamples, the DN-4 residual). Standard library; it imports `withheld_rows.py` and `sweep_d5_r1.py` |
| `s11g_rb_forecast.json`, `.stdout.json` | Its output (revision 2.1). Input: `inputs/i4_formation_rows.json`, I4's record pinned by full hash (sha256 `c548f51999b4df161fb6feb1109ec1a8fa7d40f822f9f497aafc050e9a82d200`) until I4's commit lands |
| `S11G_GUARD_revision1.md`, `S11G_GUARD_revision2.md`, `S11G_GUARD_revision2_1.md` | Revisions 1, 2 and 2.1, archived |
| `s11g_rev22_trace.py`, `.json`, `.stdout.json` | Revision 2.2: source-trace pins for the routing-gate analysis, read from the product code of main `72d5ff864`; 20 checks after erratum E-4, all true |

Run from T3/:

    PYTHONDONTWRITEBYTECODE=1 nice -n 19 python3 DESIGN_NUMERICS/_run_records/s11g_rb_forecast.py <P> DESIGN_NUMERICS/_run_records/inputs/i4_formation_rows.json <out.json>

Here `<P>` is `projects/chirality-piping`, given relatively as `../../../../../../..`. Python 3.11.15.

---

## 11. Open items and ROOT decisions

**Decided by ROOT** (`ROOT_RULINGS_V1.md`, rulings on revision 2 and on V1's delta check):
- **R-b′ is selected.** It catches all 8 INPLANE Passed breaches, demotes no committed case, changes no committed byte, and falsely demotes 12 synthetic frozen case-modes (8 LFRAME, borderline; 4 WEAK).
- **The SF-4 floor is accepted, applied only to the self-equilibrated part of the defect** (DB-1, revision 2.1).
- **SF-2's CannotBound** availability loss on loaded curved spans is accepted; disclosed in S11-G's CHANGE_RECORD; W1c with T4 removes it.
- **The formation list is emptied when S11-G merges,** provided its gate run shows all 14 rows published non-Passed on both entries and both modes.
- **K-D5 does not demote the INPLANE cases after S11-F** (DS-1). R-b′ is the only catch; I3 expects K-D5 silent there.

**Open:**
1. **V1's delta check** of revision 2.2 (the routing gate; §0.2).
1a. **ROOT: path 1's R-b′ variant** remains a disclosed, fail-closed residual under ruling 3 (characterization test T20; owner `SOURCE_BLOCKS_FINALIZATION_FAILED`, closing before T3 closes). The load-row variant is removed by G-2. An option, not proposed here: form R-b′ before routing from the ordinary solution and let it set `needs_source_recovery`, so that R-b′-fired nodal-only cases go to exact retained-source recovery. That would remove the residual and could publish exact values for INPLANE-type rows, but it changes R-b′'s semantics (demotion becomes a recovery route) and its forecast, and belongs to a separate ruling.
1b. **Runs to confirm** (from the code, not run here): T18 on I5's path-2 model (I5's `ruling3_characterization_multi_case_captured_receipt_residual` becomes T18's paths-differ counterpart), and T19/T20's multi-basis construction, which I5 has not yet shown selects case A.
2. **For S11-G's CHANGE_RECORD:** the DN-4 residual (self-equilibrated-only junctions with a genuine small net are hidden; routed to W1/F2) and the CannotBound availability loss.
3. **Confirmations during implementation:** the exact-pressure rounding count (§3.2, V1 counts 9–10 against γ₁₆); the product's own B on the LFRAME rows (borderline, §6.4); the product's S\*_moment = 100 on the F-case INPLANE rows (floor margin 1.72, DN-6).
4. **Recorded, no action:** F2 resolves the INPLANE rows with modest margins (V1 N-2: 1.85 on the stop rule and 1.7 on the floor, F case). R-b′'s coverage limit (V1 N-7) is stated in §4.
