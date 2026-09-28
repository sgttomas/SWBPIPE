# I10: implement slice K2b (W2 force-radix scaling and the kernel half of formation-time scaling)

This is an implementation TASK. Read Root `AGENTS.md`, `agents/AGENT_TASK.md` and `_COMMON.md` first. The Mac host rules in `I8R_K1_RESUME.md` ("The Mac host") override `_COMMON.md`'s host section, and apply to you in full.

## Roles

ROOT (HELP_HUMAN) dispatches you directly as a background subagent and is your return path. There is no separate T3 manager on the Mac. Make no Git writes; ROOT commits.

## Purpose

K2b is the next kernel slice. The order is S11-K ✓ → K3a ✓ → K-D5 ✓ → K2a ✓ → K1 ✓ → **K2b** → K5. It implements:
- **W2** (`DESIGN.md` revision 5a.2 §4.7): exact force-radix scaling of the ordinary system in the kernel, with the normative b-rule and unscaling for publication;
- **the kernel half of formation-time scaling** (SCALE-W; ROOT_RULINGS_V1 lines 111 and 128): when checked formation (K2a) raises `NumericalRange`, choose b from the predicted exponent of each coefficient, then form the elements with E, G, spring stiffnesses and load contributions scaled by 2^b, so that every scaled operand and coefficient is normal and every scaling is exact.

## ROOT rulings for this slice (2026-09-28)

1. **Kernel only, like K1.**
   - K2b adds the scaled evidence, the b-rule and scaled formation as **kernel entries and options**:
     - `SparseAssemblyOptions` gains b, as K1 designed it (non-exhaustive, so no caller changes);
     - the dense and SA paths gain opt-in, scale-aware siblings or an options form.
   - **Every existing entry keeps today's behaviour byte for byte (b = 0).** No product path calls a new entry: PP's wiring, and the `range_scaling:` evidence line, are F1b's.
   - So K2b changes no published byte, and **the both-entry gate is not run.** The parity tests and T9 are the evidence, as for K1.
   - If you find that the design cannot be met without changing an existing entry's behaviour, **stop and report.**
2. **The LEF expectation, restated** (resolving `I7_F1_IMPLEMENTATION.md` addendum 1 for the kernel half).
   - RF-RANGE **LEF-small never reaches formation.** `FrameElement::new` refuses it with `DegenerateAxis` at FK's 1e-12 m axis tolerance (I8R's finding, verified in K1's K2a-interaction tests).
   - The design's "LEF-small and LEF-large solved" is therefore restated for K2b at kernel level:
     - **LEF-large** reaches `local_stiffness` on the typed entry. It must be **solved** at kernel level through scaled formation.
     - **K2a's product-reach formation-range cases with normal geometry** must also be **solved** at kernel level through scaled formation, each accurate to the 1e-9 criterion against an exact reference:
       - reach_zero;
       - reach_lef;
       - the spring-carried G = 1e-300 case, which K2a refuses today and which was recorded "for K2b's scaling to restore";
       - the partial-underflow case.
     - LEF-small stays a geometry refusal; it is not a K2b failure.
   - F1b's brief restates the product level separately.
3. **Checkpoint 0 (plan before code).** Before writing product code, end your turn with a short plan for ROOT's approval:
   - where the b-rule lives;
   - the new entry and option signatures, and how the existing entries stay byte-identical;
   - how scaled formation reaches K2a's checked `local_stiffness`, and user, curved and spring elements;
   - how S11-K's exact load ledger, K-D5's formation check and K1's pattern path carry b;
   - the unscaling outcomes;
   - your test list.

## Basis (read in this order)

1. `T3/DESIGN_NUMERICS/DESIGN.md` revision 5a.2 (`fb62ef4a…`; hash-pinned, don't edit):
   - §4.7 (W2), all of it: the b-rule steps 1–5; the admitted range; the refusal texts; formation range; the capture boundary;
   - the K2b row of §6, and the F1, K5 and V-K rows;
   - §5 item 6;
   - the §7.3 mutations touching W2 (b = 0 bit-identity, unscaling).
2. `T3/ROOT_RULINGS_V1.md`:
   - SCALE-W (lines 111–128);
   - "K2a: product reach" and corrections 1–3. The authoritative figures are in K2a's RETURN §5 as scoped by RETURN_ADDENDUM_1, including the **axis-aligned scoping of M03's floor**;
   - the F1 split;
   - the K1 sections.
3. **K2a's records:** `IMPLEMENTATION/K2A/` (RETURN, RETURN_ADDENDUM_1, the reach tests) and `REVIEW/K2A_REVIEW.md`.
4. **K1's records:** `IMPLEMENTATION/K1/RETURN.md`: §12, the F1b interface (`SparseAssemblyOptions`, `assemble_sparse_stiffness`, `SparseAssemblyEvidence`); addendum 1's N3.
5. **The code on your base** (main `eb52114e9`):
   - `FK/lib.rs` (`local_stiffness` with K2a's checked intermediates; `FrameKernelError::NumericalRange`);
   - `FK/structural.rs` (`binary_exponent`, `transform_roundoff`, the gate stages);
   - `FK/structural/sparse.rs`;
   - `SA structural_adapter.rs`;
   - `sparse_direct`.

## Base, branch and paths

- **Branch:** `codex/piping-k2b-20260928`, from main `eb52114e9`. ROOT creates it in `<wt>/k2b`.
- **Target:** `<wt>/k2b-target`.
- **Scratch:** `<wt>/scratch/i10`.
- **Coordination:** I9 (the skew M03 pin, tests only) runs in parallel on another branch, with a disjoint write set. If its tests land first, ROOT merges main into K2b, and your pins must keep passing.

## Write set (from the K2b row; re-locate every line on your base)

- **`FK/lib.rs`:**
  - scaled formation from E, G, spring stiffnesses and load contributions times 2^b;
  - the predicted-exponent b choice, from the sum of operand exponents;
  - the checked path unchanged at b = 0.
- **`FK/structural.rs`:** helpers for the scaled system and the unscaling outcomes.
- **`FK/structural/sparse.rs`:** the sparse assembly entry that takes b, through `SparseAssemblyOptions`.
- **`SA`:**
  - a private `force_scale_exponent` on the evidence;
  - scale-aware solve siblings or options: K and f scaled by 2^b; actions, reactions and residual records unscaled for publication, with the design's outcomes;
  - S11-K's force terms and K-D5's formation primitives carried under b.
- **Not in scope. Stop and ask before touching any of these:**
  - `PP` and the facade;
  - `nonlinear_integration/src/lib.rs` (the loop stays option (c));
  - `curved_bend`;
  - `diagnostics`, unless a new error variant truly needs an exhaustive-match arm; ask first;
  - K5's witness;
  - the committed fixtures.

## Tests (all required)

- **b = 0 bit-identity.**
  - All existing suites pass unchanged.
  - Every existing entry's `Debug` report is byte-identical to main, in both representations, on the kernel references, the K-D5 models and product-shaped models.
  - T9 (Mac-only) is 112 of 112 byte-identical. **Any committed-byte change stops the work.**
- **Exactness of scaling.**
  - On normal-range models solved with a forced b ≠ 0 (a test hook), displacements are bitwise equal to b = 0.
  - Actions, reactions and residual records unscale exactly (normal outcomes).
  - M03's screens give the same outcomes (componentwise-relative).
- **The b-rule's branches (§4.7):**
  - step 1 passes with b = 0;
  - the subnormal-at-formation refusal;
  - the infeasible window refusal, with its exact reason text;
  - the second-evaluation `Range` refusal;
  - the unscaling outcomes: normal exact, subnormal outcome with its stated precision, and underflow or overflow → NUMERICAL_INTEGRITY_UNRESOLVED "range: publication outside binary64", never flushed.
- **Formation-time scaling:**
  - LEF-large solved at kernel level;
  - reach_zero, reach_lef, spring-carried and partial underflow solved and accurate to 1e-9 against exact references;
  - a synthetic PHYS-R4 element (b ≈ 500).
- **Both representations.** Dense and K1's pattern path give identical outcomes and bits under the same b.
- **Interactions:**
  - K-D5's formation check under b: the same demotion outcomes as the unscaled equivalents where both exist;
  - S11-K's exact RHS and audit under b;
  - K1's parity under b;
  - K2a's refusals, where no feasible b exists, keep their names;
  - the nonlinear-loop pin still passes: the loop reaches no scaled entry.
- **Mutations** (clean archives, NONE control first):
  - b ≠ 0 leaking into the b = 0 path;
  - unscaling skipped or inexact;
  - b chosen from formed coefficients instead of predicted exponents;
  - the subnormal refusal skipped;
  - scaling K but not f;
  - your own.
- **The S11 site table and the pins:** any new accumulation site gets a disposition, and the existing source pins still hold. Any extension follows K1's precedent: declared, additive, with killed mutants.

## Return

- **Files:** `T3/IMPLEMENTATION/K2B/` (CHANGE_RECORD and RETURN, `_run_records/`, SHA256SUMS), with placeholders only, no model identifiers, and the platform stated (T9 Mac-only).
- **RETURN has an "F1b interface" section with exact signatures,** the way K1's §12 does.
- **Checkpoints:** end your turn at checkpoint 0 (the plan), then at the same checkpoints I8R used:
  - A: compile and targeted tests;
  - B: suites against a Mac baseline of main, and T9;
  - C: mutations;
  - D: records.
