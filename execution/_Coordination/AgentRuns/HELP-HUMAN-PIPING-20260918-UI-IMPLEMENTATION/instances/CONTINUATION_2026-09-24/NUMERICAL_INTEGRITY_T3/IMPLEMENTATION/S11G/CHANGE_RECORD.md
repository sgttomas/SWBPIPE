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
- **Nothing else changes.** No value changes, no field or code is added, and the guard itself never produces an `Err`.

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
5. **Formatting.** rustfmt ran on the touched files whose base was already clean. PP `lib.rs` was not reformatted: base has 147 rustfmt diffs across that crate, and reformatting would touch unrelated code.

## Residuals, disclosed

- **Receipt coverage on the pre-0.4 captured entry (ROOT ruling 3, revision 1; revision 2.2).**
  - **Path 2 (the 2.1 routing gate on an already-Sensitive case) is removed by G-1.** T18 confirms it by run: the invocation is not refused. Case B keeps main's real refused attempt, and T18's whole envelope is byte-identical to base in both modes.
  - **Path 1, load-row variant, is removed by G-2 and E-1.** T19 confirms it by run. It uses per-case modulus bases, and case A is selected, which settles V1's N3. The invocation is not refused; case B is `SENSITIVE`, and its entry is the zero-work `unsupported` decline.
  - **Path 1, R-b′ variant: a disclosed, fail-closed residual, not demonstrated reachable.**
    - R-b′ acts after routing, so a case demoted from Passed by R-b′ beside a source-selected case would fail finalization with `SOURCE_BLOCKS_FINALIZATION_FAILED`, and no envelope would be published.
    - Three constructions failed to select case A: an exact radix-range refusal, `UnsupportedBlock{order: 3}`, and `UnsupportedBlock{order: 4}` (RETURN §6).
    - T20 characterizes the fourth, a two-body construction. It publishes with no receipt, because case A's recovery reports `UnsupportedBlock`, and case B is demoted by R-b′.
    - RV4 attempts it independently. The owner is T3's composite `SOURCE_BLOCKS_FINALIZATION_FAILED` item (the "demoted-ordinary" receipt form), which retries once K-D5 merges.
  - **Reach per slice (facts from the code).**
    - S11-F's `LOAD_CONTRIBUTION_ABSORBED` and K-D5's formation check both demote at the kernel `StructuralReport`, before routing, so they take retained-source recovery: **no residual.**
    - S11-G's load-row guard now routes the same way (G-2): **no residual.**
    - Only R-b′ remains, as above.
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

`GATE/FORMATION_EXCEPTIONS.json` is to be emptied by its generator (`GATE/pin_s11_exceptions.py`). S11-G's gate run supports that: T16 passes, and the probe table in RETURN §5 shows all 14 rows published non-Passed on their entries and both modes. The manager runs the generator and commits the GATE change on the T3 branch. S11-F's code-level `FORMATION_PINS` stay bit-identical, since no value changes.

## No in-band marker

No envelope, receipt or serialized struct gains a field. The ledger's formation records are not serialized, and are excluded from `Debug`.
