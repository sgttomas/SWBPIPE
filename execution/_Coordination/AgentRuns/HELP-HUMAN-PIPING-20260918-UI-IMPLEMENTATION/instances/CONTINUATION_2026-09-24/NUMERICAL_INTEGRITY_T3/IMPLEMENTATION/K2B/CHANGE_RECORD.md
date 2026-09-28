# K2b change record: W2 force-radix scaling and the kernel half of formation-time scaling

This is the draft PR record for slice K2b of T3 (numerical integrity), following `.agents/skills/chirality-change/SKILL.md`. It was implemented by I10 (TASK).

- **Branch:** `codex/piping-k2b-20260928`, from main `eb52114e9`.
- **Commits (made by ROOT):**
  - `6ce4d694b`: checkpoint A;
  - `70828d4d6`: ROOT's checkpoint-A rulings A–C;
  - `8e6698282`: checkpoint C, two assertion groups, tests only.
- **Proposed next commits:**
  1. the pin test, tests only: SA `k2b_tests.rs`, +78 lines;
  2. these records.
- **Size:** 11 files, +4,768 −10 at `8e6698282`; with the pin test, +4,846 −10.
- **Basis:**
  - D1 `DESIGN.md` revision 5a.2 (`fb62ef4a…`): §4.7 (W2) in full, with the K2b row of §6 and §5 item 6;
  - `ROOT_RULINGS_V1.md`: SCALE-W, K2a's product reach, and the K2b rulings on I10's checkpoint-0 plan (`05047845a`), its checkpoint-A stop (`a95adb540`), and the b-rule's window (`97000ab9f`);
  - the I10 brief (`TASK_BRIEFS/I10_K2B_IMPLEMENTATION.md`, `2e40319a…`) with `_COMMON.md` (`6bb845bf…`);
  - K1's F1b interface (`IMPLEMENTATION/K1/RETURN.md` §12).
- **Platform:** Mac (aarch64-apple-darwin), rustc 1.97.1. T9 is Mac-only.

## What changes

**Kernel only.** K2b adds new entries and one option, and changes no existing behaviour. Every existing entry keeps today's behaviour byte for byte (b = 0). Nothing outside `FK` and `SA` calls a new entry. The only callers are tests and site lists (`_run_records/callers.txt`). So K2b changes no published byte, and the both-entry gate is not run (ruling 1).

- **`FK/lib.rs`:**
  - `ForceScale`, an even exponent b; b = 0 is `UNSCALED`.
  - Exact scaling helpers: `force_scaled_value` and `force_scaled_matrix`. A value that is not normal, or would not stay normal, is refused, never rounded.
  - `FrameElement::force_scaled` multiplies E and G by 2^b. K2a's unchanged, checked `local_stiffness` then forms the element at scale.
  - `UserStiffnessElement::force_scaled` scales the element's four stiffnesses.
  - `ForceScaleCensus` implements the b-rule's steps 2–3 (census, subnormal refusal, window). It applies ruling 1's even-b parity rule.
- **`FK/structural.rs`:**
  - The named refusals (`ForceScaleReason`, with the design's texts), `RangeTrigger`, `ForceScalingRefusal` and `ForceScaledError`.
  - The publication outcomes: `Representability`, `PublishedValue`, `RecordRepresentability`, `RecordOutcome` and `ForceScaledSolution`.
  - `unscale_for_publication`, `unscale_descriptive`, `unscale_structural_solution` and `unscale_structural_error`.
  - `binary_exponent` becomes `pub(crate)`. That is the file's only changed line.
  - No new variant is added to `StructuralError` or `FrameKernelError`.
- **`FK/structural/sparse.rs`:**
  - `SparseAssemblyOptions` gains a private `force_scale`, with `with_force_scale` and `force_scale()`. It stays non-exhaustive, and `new()` is unscaled.
  - `assemble_sparse_stiffness` scales its inputs exactly when b ≠ 0.
  - `SparseStiffness::force_scaled_reactions` gives E12 reactions at 2^b, unscaled by a single rounding with step 5's outcomes.
- **`FK/load_ledger.rs` (declared extension L1, ruling 4):** `AssembledForce::force_scaled`.
  - Each term is scaled exactly, and each net is rounded once.
  - Nothing is pushed to a ledger.
  - S11-G's formation records are dropped. A scan and a test show that the kernel never reads them.
- **`SA` (`structural_adapter.rs`):**
  - Evidence and constructors:
    - a private `force_scale` on `AssemblyEvidence` and `SparseAssemblyEvidence`;
    - `new_force_scaled`, which builds the evidence from the primitives times 2^b.
  - Solve entries:
    - `solve_force_scaled` and `solve_force_scaled_with_formation_check` in both impls;
    - the orchestrator `solve_with_force_scaling` (`ForceScalingCase`, `EvidenceRepresentation`, `ForceScalingOutcome`), which runs the b-rule's steps 1–5.
  - Existing entries:
    - they refuse an evidence formed at b ≠ 0 (fail closed);
    - an evidence built by `new` is always unscaled, so this never fires today.
- **Pin and site-list extensions (ruling 5; declared, additive, tests only):**
  - NI `s11k_tests.rs`;
  - FK `s11_site_table.rs` (three rows with zero binary64 accumulations);
  - PP `s11f_site_test.rs` (three force-function rows).
  - No existing row, count or assertion changes meaning.

## What differs from the design's letter (ROOT's rulings)

- **Even b (ruling 1).** §4.7's promises hold only for even b: scaling is exact, u is unchanged, and the screens are equivalent. With even b, the gate's power-of-two equilibration shifts by b/2 and prepares a bit-identical matrix. The parity rule takes the floored midpoint m if it is even, else m − 1, else m + 1. A single odd point in the window is refused. The step-by-step derivation is in `RETURN.md` §4.
- **Residual records (ruling B, amending ruling 3).**
  - Step 5's refusal ("range: publication outside binary64") applies to published actions and reactions only.
  - The physical fields of residual and intended-action rows are unscaled by a single rounding. Each carries an explicit outcome: subnormal (with its precision), underflow (a signed zero) or overflow (a signed infinity). Every non-normal outcome is listed in `ForceScaledSolution::records`. These fields never refuse the case.
  - This departs from §4.7 step 5, which lists residual records among the refusals. The field-by-field treatment is in `RETURN.md` §6.
- **The census (ruling 2):** the predicted exponents of the 24 values that K2a checks and that scale with b, plus E, G, users, springs, curved entries and load terms. See `RETURN.md` §5.

## Results

- **Formation-range cases, solved at kernel level** (both modes, both representations, with bitwise agreement between the two representations):
  - reach_zero at b = 734 and reach_lef at b = 734: within 1e-9 of the exact references.
  - Partial underflow at b = 898: within 1e-9. It carries three residual-record outcomes (ruling B).
  - A synthetic PHYS-R4 element at b = 536 (the design says "≈ 500"): Passed and accurate.
  - R1's LEF-large cases at b = −702: bit-identical to their RF base cases times exact powers of two, with the base cases' standing (ruling C). CONT is Passed and within 1e-9. CHAIN and SKEW are Sensitive, as their bases are, and are not claimed accurate.
- **Cases that stay refused:**
  - The spring-carried case (G = 1e-300 Pa) stays a named refusal, with K2a's `GJ/L: G*J` as its trigger. The limit is M03's contribution audit, and ratio invariance means no b helps (ruling A). It is routed to W1/K4.
  - LEF-small stays a geometry refusal.
- **b = 0:**
  - Suites: all 39 manifests `--no-fail-fast` against a fresh Mac baseline of `eb52114e9`. 0 changed, 0 removed, 28 added (14 FK, 13 NI, 1 NI pin). The only failures are 3 Mac platform tests, and their failure output is byte-identical to main's.
  - A `Debug` probe of today's public entries on the paths K2b touches, on 24 invented models in both modes, built from archives of main and the candidate: 439 of 439 outputs identical.
  - **T9 (Mac-only): 112 of 112 committed outputs byte-identical.**
- **Forced even b (±2, ±64, ±400)** on K-D5's 13 models, in both modes and both representations:
  - displacements bit-identical;
  - the unscaled report identical in `Debug`;
  - reactions and actions unscale exactly;
  - dense and pattern agree.
- **Mutations** (clean archives, NONE control first, at most three at once at -j 4):
  - All 32 counted K2b mutants are killed by behavioural or pin assertions. That is 30 at checkpoint C and the two third-attempt forms after it.
  - K2B-LEAK-OPTIONS is not counted. Its only kill was a stack-overflow abort; LEAK-PUBLISH and LEAK-EVIDENCE are its behavioural replacements.
  - All 15 of the original pins' mutants are still killed, at the same sites as in K1's table: 0 sites lost, 0 added.
- **The "third attempt" mutant is not equivalent. ROOT's ruling (`97000ab9f`) is carried out.**
  - **The finding:** a constructible case is refused at the rule's b (312) and solved Passed at b = 500 and 572. The census does not cover the gate's equilibrated right-hand side, which moves by 2^(b/2). The realistic reach is nil (`RETURN.md` §13.3).
  - **Step 4 kept:** K2b keeps §4.7 step 4 ("There is no third attempt").
  - **The pin:** the case is pinned as a documented limitation of the b-rule, not desired behaviour, by `k2b_the_rules_b_refuses_a_case_another_b_in_its_window_solves` (tests only, added after C).
  - **The kill:** the mutant (a bounded retry within the window) ran from a clean archive after the NONE control. It is killed by that test at `k2b_tests.rs:1030`. An iterator form, which FK's site table cannot see, is killed only by that test.
  - **The refinement:** it goes to the T3-close list and to F1b's brief; none lands in K2b.
- **Checks on these records:** GEN-8 passes, and a direct scan of `IMPLEMENTATION/K2B/` finds no machine-specific path.

## Limits

- **The loop:** the nonlinear loop stays on the unscaled binary64 path (option (c)). This is pinned.
- **Left to F1b:**
  - PP's wiring and the `range_scaling:` evidence line;
  - loads that PP forms at b = 0 from out-of-range products. These have lost bits before the kernel sees them (`RETURN.md` §14);
  - the b-rule refinement, which F1b's brief must consider before b is wired into the product. It is also on the T3-close list (ROOT `97000ab9f`; `RETURN.md` §14).
- **Formation outside K2b's coverage:** curved bends are realized at b = 0 by `curved_bend`, and their own formation range is not checked by K2a or K2b.
- **Accuracy:** K2b adds no accuracy beyond the ordinary path. W1 is the route to accuracy.
- **Pending the reviewer's check:**
  - the even-b derivation (ruling 1's condition), and whether its premise is adequately disclosed. The premise is that every rounded operation is zero or normal at both scales, and the dense Cholesky factor and triangular solves are unchecked (`RETURN.md` §4);
  - the ruling-B departure;
  - the third-attempt finding and its pin.
