# V1: targeted verification of D1 revision 5a and D2 revision 5b

This resumes V1 under ROOT's process ruling (`ROOT_RULINGS_V1.md`, BACKCHECK_R5: "V1 verifies only those fixes and R5-4's bound … no further full review round"). Read `_COMMON.md`; your earlier briefs and write rules still apply.

## Scope: only these items

Verify that each fix is present, correct, and consistent between D1 and D2. **Do not reopen settled design.** A new defect you notice in passing is reported as a NOTE unless it is blocking.

**From BACKCHECK_R5:**

| Item | What to confirm | D1 / D2 |
|---|---|---|
| R5-1 | B's reaction rule adopts your fix. A restrained DOF is unprovable when it lies in any family set of an incident member carrying a member load, pressure or state term, or in the axial family of an eigen-strained member. `b_proof_main.json` still reproduces, and the 48 committed proven counts are unchanged. The S-J negative controls cover it | both |
| R5-2 | `input_derived_dofs` is exactly the kernel's non-free DOFs, and hangers and constant-effort supports are free. The family rule is pinned with sources (PP:5169-5230, :5279, :10239, :10246). Both scripts follow it, and the counts are unchanged | both |
| R5-3 | S11's caller list is complete, including `product_equilibrium::evaluate` and `linear_supports::apply_linear_supports`. Cross-check it against I1's `ks_callers.txt` on the S11-K branch (at `14354efdb`) and against D1's newly found `StructuralAssembly::solve` caller (nonlinear lib.rs:1915) | D1 |
| R5-5 | Condition 3 includes the check-level rule-pack comparison: a decided check that becomes indeterminate stops that domain's retirement | both |
| N-1 | b = fl↑(2^-64·S*), which is an exact point only when S* = 0. A subnormal b is handled | both |
| N-2 | The zero-scale test is kernel-level | D1 |
| N-3 | The ledger firings at actual 0.80 are acknowledged | D1 |
| N-4 | EF's blindness to input representation is stated | D1 |
| N-5 | Benchmarks are enumerated and no outcome changes | D1 |
| N-6 | The phrase at the former D1:446 is removed | D1 |
| [align D1-B] | No alignment marks remain in D2 | D2 |

**R5-4, the four conditions of ROOT's pre-acceptance** (`R5_4_CURVED.md` and DESIGN §4.3.1):
1. EF's curved (and user-stiffness) re-formation in `Wide<2>` from binary64 inputs, built from the actual chord, is sound and rotation-consistent by construction. The Wide arctangent and the square-root-only sine and cosine are specified soundly (argument reduction, error, stop).
2. It misses none of the 4 curved Passed breaches on main. Reproduce them from `curved_ef.py`, preferably with your own independent re-formation.
3. It demotes none of the 12 realistic elbow case-modes (E1–E6).
4. Reusing the product's binary64 curved matrix would miss 2 of the 4, so re-formation is required.

Also confirm that the new K-D5 tests, mutations 31 and 32, and the separate linear entry `solve_with_formation_check` (called only from PP:3965, with the loop's calls pinned) are consistent with option (c) in S11-K.

**Three D2 5b choices to check (D2 flagged them):**
- **A new fail-closed reader rule:** a selected case whose invocation has a nonlinear support is refused as `INPUT_DOF_MISMATCH`, because W1 never selects one. Is it sound, is it consistent with D1's W1 coverage (nonlinear supports go to T5), and can it refuse a legitimate selected case?
- **R5-5 wording:** D2 counts withheld-row refusals as "undecided" (RULE_INPUTS_INCOMPLETE from RULE_RESULT_INDETERMINATE *or* from a withheld-row refusal). That is broader than D1's word "indeterminate". Is it consistent with D1 and with the row-level count, and does it make the gate stricter, never looser?
- **R5-2 family detail:** D2 states PP's family matching precisely. Hanger type is trimmed; family `"spring"` is compared untrimmed; `constant_effort_support`, `variable_spring_hanger` and `spring_hanger` contribute nothing; `boundary_motion` adds prescribed DOFs (PP:5169-5230, :5279, :10232-10250). Confirm it against the source, and check that D1's scripts and wording agree. If they don't, name which side should change.

**Rulings made since revision 5, to check for consistency only:**
- D-14: equivalent-static in F3, refused until then. The exact-from-inputs rule covers the solve-time seismic and wind generators only; self-weight is an ordinary stored input. Check the GeneratedUniform form.
- The exception pin: `GATE/S11_EXCEPTIONS.json`, 88 triples in 13 cases captured and 140 in 22 typed. Check that `s11_exceptions.py` reconciles 228/228.
- ROOT's §5 rulings (`SELECTION_PACKAGE.md` §5), as reflected in D2's §9.

## Basis

- D1 revision 5a at `16bcbf369`: DESIGN.md `7decddea`, S11_CONTAINMENT.md `82c1b072`, D5_TRIGGER.md `f6e24a69`, R5_4_CURVED.md `4843c4e9`.
- D2 revision 5b at `41f018355`: DESIGN_STANDING/DESIGN.md `1bc8e036` (its §0.5 table lists every change).
- Your own BACKCHECK_R5 (`0874bed35`) and its attachments.

## Running things

Standard-library Python is fine. Cargo is not needed; if you use it, run one heavy job at a time (check `pgrep -x cargo` first). Keep free disk above about 8 GB. Make no Git writes.

## Return

Write `T3/REVIEW/VERIFY_R5.md`, containing:
- a table with one row per item above: verdict (VERIFIED / NOT VERIFIED / PARTIAL), evidence, and site;
- R5-4's four conditions, each with your reproduction;
- any NOTE-level observations;
- what you ran;
- the overall verdict. It is **VERIFIED** only if every item is verified and all four R5-4 conditions hold.

Then send the manager a SendMessage summary with the verdict and the file's sha256.
