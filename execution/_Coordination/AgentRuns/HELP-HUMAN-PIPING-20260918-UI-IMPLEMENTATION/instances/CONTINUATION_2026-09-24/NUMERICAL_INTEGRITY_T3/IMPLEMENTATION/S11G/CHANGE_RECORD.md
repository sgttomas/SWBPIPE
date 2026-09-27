# S11-G: the formation-noise guard (change record / PR body)

**Branch.** `codex/piping-s11g-20260927`, cut from main at `43b8f83aa` (S11-F merged, PR1000). The manager merged `origin/main` `72d5ff864` (PR1002) forward, giving `3d844fea4`. The candidate is `3d844fea4` plus I5's uncommitted write set (RETURN §1).

**Basis.**
- **The selected design:** `T3/DESIGN_NUMERICS/S11G_GUARD.md` revision 2.2, selected by ROOT ("Selection: S11-G revision 2.2", `ROOT_RULINGS_V1.md`; sha256 `680fecdd…`, `3c80158e9`), with D22-1 as a binding condition. Also in the basis:
  - the 2.2 erratum §0.2 (E-1, the zero-work decline; `6e5e7f03b`, sha256 `afe5f256…`);
  - V1's delta-2.2 PASS (`REVIEW/S11G_CHECK.md`).
  - Revision 2.1 (V1's delta-2.1 PASS, NOTEs D21-1 and D21-2) stands except for its routing gate, which 2.2 supersedes.
- **The brief:** `T3/TASK_BRIEFS/I5_S11G_IMPLEMENTATION.md` (`14dcf9103`), with Addendum 1 (`c0d090172`).
- **ROOT's rulings on I5's items:**
  - "S11-G implementation: I5 rulings";
  - "S11-G ruling 3, revision 1" (`70907245b`);
  - the note on T18–T20 and T6a (`0019a552e`).

**Author.** Type 2 TASK I5 (Claude). This is not owner review. It needs:
- independent review of the full diff (RV4);
- hosted CI;
- the manager's GEN-8 check.

## What changes (no value changes)

**The load-row guard (design §3).**
- **Records in the ledger.** Each formed ledger term now carries a formation record in a vector parallel to the ledger terms (FK `load_ledger.rs`: `Formation`, `push_formed`, `push_formed_product`). A record is one of: `Exact` (the SP formula over exact expansions, scale 1 or 3), `RoundedProduct` (fl(a·b), exact defect −k·lo), `Bounded` or `CannotBound`.
- **Exact defects per row.** `AssembledForce::formation_rows` gives each loaded row's exact 12-scaled defects in two accumulators, net and self-equilibrated, with no binary64 intermediate. It also gives the bound sum B, the self-equilibrated magnitude P, and a lower bound of the intended net.
- **The decision** (new PP module `formation_guard.rs`) is exact. A row fires on any of:
  - `B > 0 && B ≥ T0` (ROOT ruling 2, an erratum; see the deviations below);
  - `|A_net| + 12B − 12T0 > 0`, with ±12B and ∓12T0 added exactly into a copy of the accumulator (D21-1);
  - `|A_se| − 12Tf > 0`;
  - a `CannotBound` term;
  - a range failure.

  T0 = RD(10⁻⁹)·max(|n|, S\*) and Tf = RD(10⁻⁹)·max(|n|, S\*, 2⁻¹⁰P), both rounded downward. S\* is the §4.1.6 coupled body scale of the intended nets: free rows over the body's free rows, restrained rows over all its loaded rows. The floor applies to the self-equilibrated defect only (DB-1).
- **Families.**
  - Straight uniform loads: `Exact`. SP gains `equivalent_global_nodal_loads_with_spans_formed`, which returns today's values bit for bit plus the exact intended formula.
  - Straight thrust, thermal and eigen pairs, curved caps and curved thermal K_rc·fl(ε·c): `RoundedProduct`, self-equilibrated.
  - Exact-pressure group operands: `Bounded`, γ₂₀·|t|. The rounding count k = 10 comes from the source; since k > 8, the design's γ₂ₖ rule applies.
  - Curved consistent vectors (uniform, pressure wall): `CannotBound`.
  - Equivalent-static generated intensities carry the operand bound γ₄·|t|.
  - Nodal loads and constant effort are inputs.

**The recovery guard R-b′ (design §4).**
- SP gains `bending_formation_bound`: γ₁₆·(Σ|K|·Σ|T||u| for the RY and RZ rows), each sum exact and rounded upward.
- PP records (member, end, q, B) for each straight member end published from the formed K_e·u.
- After the element-recovery loop, `formation_guard` forms S\*_moment from the case's published rows (DESIGN §4.1.6.1: bodies, the closed kind table, units, L_b). It fires when all three hold:
  - B > RD(1e-9)·q, exactly;
  - q > 2¹⁰·B;
  - q ≥ 2⁻³⁴·S\*_moment.

**What firing does.**
- **Where.** The load-row finding demotes at `append_integrity_report`, on both the linear and nonlinear calls. The R-b′ finding amends the same integrity record after the recovery loop (ROOT ruling 1).
- **How.** The code becomes `NUMERICAL_INTEGRITY_SENSITIVE` (warning), and one reason sentence is appended to the existing message. The `StructuralReport` text stays truthful (`quality: Passed`).
- **The no-op rule.** A case already Sensitive or weaker is left byte for byte as it is.
- **The receipt's publication reservation (RV4-N3).** In a multi-case captured invocation with a receipt, the added info diagnostic (E-1's decline, `SOURCE_BLOCK_RECOVERY_UNAVAILABLE`) grows the publication reservation by 32 + 12 × its size, plus the difference between the entries. The attempt ledger itself equals main's (T22). This matters only within a few kB of the 64e6 invocation limit.
- **Nothing else changes.** No value changes, no field or code is added, and the guard itself never produces an `Err`. The one exception is the ruled, fail-closed receipt residual under Residuals below: a pre-0.4 captured invocation with per-case modulus bases can be refused.

**Routing (revision 2.2 G-1 to G-3, with erratum E-1 / D22-1).**
- **G-1.** `source_eligible` is main's predicate (captured, no nonlinear supports, no combinations). Revision 2.1's load-row gate is gone.
- **G-2.** `needs_source_recovery` includes the load-row finding. The receipt's ordinary attempt records the published verdict: `OrdinaryAttempt::passed(…, formation_sensitive)`.
- **G-3.** A retained-source response of a guard-fired case is never selected. `SelectedSourceRecovery::decline_formation()` turns it into the existing `unsupported` receipt entry (stage `source_validation`, code `unsupported_family`), with the executed work charged, before the 0.4.0 replay reservation.
- **E-1 / D22-1.** A guard-fired case that the ordinary route would not attempt (report Passed, no `Err`) is declined **without an attempt**. Its entry has zero work, and the invocation ledger equals the unguarded one (T22).
- No receipt, reader or schema is edited. Every wire form already exists on main.

## Which cases change standing, and why

- **The 14 formation rows** (T16, and the run-evidence gate table in RETURN §5). All 14 are published `NUMERICAL_INTEGRITY_SENSITIVE`:
  - the 6 UDL rows (UDL-W1e8 captured and typed, UDL-W1e80 typed; both modes), by the load-row guard;
  - the 8 INPLANE rows (F- and M-G1e80-GnG-INPLANE, typed, both modes), by R-b′. R-b′ is the only INPLANE catch after S11-F (V1 DS-1).
  - No `/results` leaf changes. The captured UDL-W1e8 case also gains one info `SOURCE_BLOCK_RECOVERY_UNAVAILABLE`, which is E-1's zero-work decline.
- **Frozen synthetic false demotions under R-b′,** forecast by D1 and accepted by ROOT: LFRAME ×4 and WEAK-W-3D/AX-rho1e-08. These are frozen references, not committed bytes.
- **Loaded realized curved spans (CannotBound availability loss, ROOT-accepted).** A case with a uniform load or pressure on a realized curved span is always demoted, whatever its accuracy, until W1c/T4 gives an exact form or a proven bound. No committed model realizes a loaded curved span (T9).
- **No committed case changes standing** (T9: every committed request is byte-identical through base and candidate).

## Committed fixtures: zero diff

- **T9.** Every committed request or model under `fixtures/`, `core/` and `validation/` ran through base (`3d844fea4`) and the candidate: 218 runs per tree, both modes, the captured and typed entries. **All 218 outputs are byte-identical,** and the output manifests are identical. No committed envelope carries an S11-G sentence.
- **T13.** `load_reference_fallback_uz` is byte-identical.
- No fixture is regenerated. The one new committed fixture is S11-G's own test input, `core/product_physics/tests/fixtures/s11g/rb_controls.json`, which has a generator in the records.

## Deviations from the selected design (ROOT-ruled or disclosed)

1. **The first clause at B = T0 = 0 (ROOT ruling 2, an erratum).**
   - **The change.** §3.4 fires on `B ≥ T0`. It is implemented as `B > 0 && B ≥ T0`.
   - **Why.** At a free row whose body's free-row intended nets are all zero, S\* = 0 and n = 0 give T0 = 0, and RoundedProduct terms give B = 0. The design's own T6 is such a row: a collinear skew pair with equal thermal axial loads, whose defects cancel exactly and which T6 pins at CHECKS_PASSED. `B ≥ T0` would demote it.
   - **Scope.** Only that degenerate point changes. A nonzero defect at T0 = 0 still fires, through the unchanged second and third tests.
   - **Tests.** Boundary tests; MR2 (restoring `B ≥ T0` alone) is killed.
2. **R-b′ amends after the recovery loop (ROOT ruling 1).**
   - **Why.** Both `append_integrity_report` calls precede the straight element-recovery loop, and S\*_moment needs the published rows.
   - **No reader in between.** No reader of the integrity code, `solve_quality` or standing lies between the append and the amendment (enumeration in RETURN §7).
3. **Directed roundings and range fallbacks (implementation choices within the design).**
   - The underflow fallback of a scaled rounded product is γ₂·|value| + |k|·2⁻¹⁰⁷⁴ (D21-2).
   - The SP range fallback adds an absolute 64·2⁻¹⁰⁷⁴ to γ₁₆·Σ|monomials|.
   - The intended net on the threshold side is rounded toward zero, and the S\* coupling terms round downward.
   - Each of these only makes the decision stricter.
4. **T6a's thrust run uses the historical pressure scope (ROOT note `0019a552e`; manager's direction).**
   - **Why.** On main a fresh solve refuses any legacy nonzero pressure with `PRESSURE_MODEL_REAUTHOR_REQUIRED` (`pressure_runtime.rs:207-224`), and the exact profile refuses pressure primitives altogether. So the straight-thrust `RoundedProduct` family carries a nonzero value only in the `#[cfg(test)]` historical scope (`lib.rs:97-98`, `historical_pressure_reference.rs:12-22`). A zero-valued legacy pressure gives a thrust of exactly 0 and a zero defect.
   - **What the test does.** T6a's thrust run executes inside `historical_pressure_reference::with_scope`, as S11-F's F10 did. The family's defect is also pinned at unit level (`load_ledger::tests::s11g_rounded_product_defect_is_exact`).
   - **Disclosed:** a fresh-solve run of the thrust family is unreachable, so no test performs one.
5. **The curved pressure families are just as unreachable (RV4-N5).** The curved pressure caps (`RoundedProduct`) and the curved wall vector (`CannotBound`), both pushed by `add_curved_bend_pressure_thrust_load` (`lib.rs:9129-9185`), are produced only from the same legacy pressure primitive. A fresh solve refuses that primitive at a nonzero value by the same check (`pressure_runtime.rs:207-224`), and the exact profile refuses it at any value. So they too carry a nonzero value only in the test-only historical scope, and no S11-G product test solves them.
   - The caps use the same `RoundedProduct` record as the straight thrust, whose defect is pinned at unit level (`load_ledger::tests::s11g_rounded_product_defect_is_exact`).
   - The wall vector's `CannotBound` handling is pinned by T15 (the curved uniform load, the same record).
6. **R-b′ is silent below a moment scale of 2⁻⁹⁸⁸ (RV4-N9).**
   - **The deviation.** When S\*_moment < 2⁻⁹⁸⁸ (about 1.5e-298 N·m), the floor clause is false (`formation_guard.rs:54`, `:430`), so R-b′ never fires. §4's literal rule q ≥ 2⁻³⁴·S\* would still fire there.
   - **Why.** This keeps 2⁻³⁴·S\* out of the subnormal range. It is fail-open only at an absurd scale, where the guard's own arithmetic loses its relative meaning.
7. **Formatting.** rustfmt (stable 1.8.0; the pinned 1.97.1 toolchain has no rustfmt, and none was installed) ran on every touched file whose base was clean.
   - **PP `lib.rs`:** only the hunks that overlap S11-G's own lines were formatted, after RV4 found 9 there (RV4-N6). One of those hunks re-indents the body of E-1's else block, which S11-G wrapped. The file's rustfmt diff count now equals base's (78 with `skip_children`), and the change is whitespace and layout only.
   - The rest of the file (base: 147 diffs across the crate with its child modules) is left as it is on main.
8. **Operand bounds of generated loads (RV4-N7, a reading).** The γ₄ operand bound applies to `{case}:generated:` loads, the equivalent-static generated intensities. Applied self-weight loads (`load:self-weight:…`) are stored inputs, behind the held-operand boundary (V1 N-3), and carry none. The bound is additive only, so it can only make the decision stricter.

## Residuals, disclosed

- **Receipt coverage on the pre-0.4 captured entry (ROOT ruling 3, revision 1; revision 2.2).**
  - **Path 2 (the 2.1 routing gate on an already-Sensitive case) is removed by G-1.** T18 confirms it by run: the invocation is not refused. Case B keeps main's real refused attempt, and T18's whole envelope is byte-identical to base in both modes.
  - **Path 1, load-row variant, is removed by G-2 and E-1.** T19 confirms it by run. It uses per-case modulus bases, and case A is selected, which settles V1's N3. The invocation is not refused; case B is `SENSITIVE`, and its entry is the zero-work `unsupported` decline.
  - **Path 1, R-b′ variant: a disclosed residual. It is reachable (C1), fail-closed, and needs per-case modulus bases and a pre-0.4 captured invocation** (ROOT: ruling 3 stands).
    - **The mechanism.** R-b′ acts after routing. A case whose report is Passed and which R-b′ demotes, beside a source-selected case, keeps an `ordinary` receipt entry whose outcome no longer matches the published verdict. Receipt finalization then fails. The captured entry returns `Err("SOURCE_BLOCKS_FINALIZATION_FAILED")`: no envelope and no published value.
    - **Construction C1, found by RV4 (the independent reviewer).**
      - The model is N05's cantilever with T19's per-case bases.
      - Case A has the cancelling tip torques on the base basis, and is Sensitive and selected.
      - Case B is on the invented soft basis (E = 1 Pa, G = 0.4 Pa), with nodal inputs only: tip F_y = 1 N and M_z = 1e-7 N·m.
      - Case B alone is Passed in its report and demoted by R-b′ on a genuinely inaccurate tip moment (relative error about 6.1e-9 against the exact 1e-7 N·m).
    - **Where it does not occur.** The typed entry publishes case B `SENSITIVE`, with no receipt. A single-case captured invocation is demoted and not refused. A 0.4.0 captured invocation is republished under CP3 SF-1.
    - **T20** now characterizes C1:
      - the refusal, in both modes;
      - the precondition, including the genuine relative error computed in the test;
      - the typed entry and the single-case captured invocation;
      - the control at m = 0.5 (R-b′ silent), which returns Ok with a receipt.
    - **History.** I5's three earlier constructions failed to select case A (RETURN §6a). They are superseded by C1.
    - **Owner.** T3's composite `SOURCE_BLOCKS_FINALIZATION_FAILED` item (the "demoted-ordinary" receipt form), which must close before T3 closes.
  - **Reach per slice (facts from the code).**
    - S11-F's `LOAD_CONTRIBUTION_ABSORBED` and K-D5's formation check both demote at the kernel `StructuralReport`, before routing, so they take retained-source recovery: **no residual.**
    - S11-G's load-row guard now routes the same way (G-2): **no residual.**
    - Only R-b′ remains: reachable (C1) and fail-closed, as above.
- **Desktop reader coverage (a disclosed gap).**
  - **What exists.** No committed desktop fixture carries a non-qualified source-block receipt entry (`failed` or `unsupported`, of any work shape). Among committed JSON, only the schemas mention those outcomes. The desktop reader's tests therefore never exercise the zero-work decline shape.
  - **What was run instead.**
    - The Rust reader `source_blocks::validate` accepts the live T23 envelope with and without the invocation (the headless test T23).
    - A scratch-only run of the stock desktop reader (`validateSourceBlockRecovery`) on the same envelopes returns the same standing as main's own `unsupported`-entry shape (T18's base-identical envelope), with or without the caller model: `{eligible: false, findings: [SOURCE_BLOCKS_CASE_UNQUALIFIED, SOURCE_BLOCKS_PHYSICAL_ROW_INSPECTION_ONLY, SOURCE_BLOCKS_ENVELOPE_UNQUALIFIED]}`.
  - No desktop fixture is added (manager's ruling).
- **The DN-4 residual, routed to W1/F2.**
  - **The class.** A junction where only self-equilibrated terms meet, with a genuine small net, is hidden by any noise-silencing floor (V1's example: 126× the criterion). Each such defect is at most u·|t|.
  - **Where it arises.** Such rows need nearly equal thermal axial loads. W1/F2's precision-p formation removes the class.
- **The held-operand boundary (V1 N-3), unchanged.** Formation error made before a held operand (`axial_load`, the SI conversion, T) is invisible to the guard.
- **R-b′'s coverage limit (V1 N-7).** It covers member-end bending only. Reactions and displacements that carry the same mechanism belong to F2/W1a.

## The formation list

`GATE/FORMATION_EXCEPTIONS.json` **is empty**. The manager emptied it with its generator (`GATE/pin_s11_exceptions.py`) at `759dccf35` on this branch (`37bdc2edd` on the T3 branch), on S11-G's gate run:
- T16 passes, and the probe table in RETURN §5 shows all 14 rows published non-Passed on their entries and both modes.
- RV4 reproduced the emptied file byte for byte.
- S11-F's code-level `FORMATION_PINS` stay bit-identical, since no value changes.

## No in-band marker

No envelope, receipt or serialized struct gains a field. The ledger's formation records are not serialized, and are excluded from `Debug`.
