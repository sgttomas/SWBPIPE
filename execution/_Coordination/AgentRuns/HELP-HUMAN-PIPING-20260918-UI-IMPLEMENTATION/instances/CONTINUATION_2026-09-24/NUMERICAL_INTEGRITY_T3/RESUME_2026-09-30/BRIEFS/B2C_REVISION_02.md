# B2-C revision 02 (I97): option (ii), and RV118's addendum

Read your own `R/I97/b2_c_01/REVISION_01.md` (`6f6a583f…`) first. Then read:
- RV115's `R/REVIEW_RV115/b2_kd_01/ADDENDUM_03.md` (`4bd3e234…`): SA3-1, and NC-1 to NC-5;
- RV118's `R/REVIEW_RV118/b2_c_01/ADDENDUM_01.md` (`774b06bd…`): A-1 to A-6;
- RR "RV115 confirms S-4 (a)'s soundness with SA3-1; the combination's displacement magnitude becomes one exact rounding" and "RV118 confirms B2-C revision 01; the exact rounding is formed in the kernel's projection (B2-K); I97 writes revision 02". Their rulings are your specification.

## What to change

1. **Option (ii): a combination's `displacement_magnitude`** is RN64 of the exact 3-norm √(x² + y² + z²) of the same node's frozen raw `global_nodal_displacement_{x,y,z}` values (mm). It replaces r1's nested-hypot recipe.
   - **The recipe, exactly:** each square is formed exactly as a dyadic rational, and their sum exactly. The square root is then rounded once to binary64, **ties to even** (A-5: an exact midpoint is reachable; give RV118's example).
   - **Keep r1's clauses:** finite only, canonical +0, SI by the projection's mm rule, the certificate against the dual physical norm, and the unchanged 64ε guard.
   - **G7's guard holds by construction:** |p − r| ≤ 2.5 ulps against hypot(hypot) in any faithful library, a margin of about 25× (RV115; RV118 measured at most 3 ulps).
   - **State that the bits are the same on every platform,** and that availability returns to v0's, per RV115's study.
   - **Give a reference implementation** in your records script (integers or `fractions` and an integer square root, VENV's Python only). Check it on RV115's and RV118's triples, the midpoint example, subnormal, large and signed-zero inputs, and against `math.hypot` nested within the guard.
2. **A-1: the site is FK `final_case.rs` `ProductProofDraft::project`,** for combination owners only (lane K, B2-K).
   - `certify_final` accepts only the bits the projection froze (ROOT read `:1884–1894`); PP supplies bits for record rows only.
   - B2-P keeps the observables stage. REVISION_01 §8's "B2-K is unchanged" is corrected.
   - **B2-K's brief gains:** the branch in KD §5.7; a K-09 diagnose check that the published bits equal RN64 of the exact 3-norm; K-13 byte identity for case magnitudes; and RV-K's review. State which existing kernel arithmetic it reuses, if any.
   - **Cost it in B2-K.** If it exceeds 4 h of B2-K, return to ROOT before writing the rest.
3. **SA3-1's text:** REVISION_01 §1.1's coverage argument is an upper bound, not availability parity. Restate §1.1 for (ii).
4. **NC-2:** `stages.observables` states the guard's formula, 64ε·max(|p|, MIN_POSITIVE).
5. **A-2:** correct S-3's parenthesis. Only `scale_bits` is null; `normalized_bits` comes from the row's value.
6. **A-3:** mark W-CB1z's full-gate PASS and `b2_c1_range_mechanics` as predicted, not observed. Name where each is first observed, and what happens if it does not pass.
7. **A-4:** place m69's row away from G5c's class threshold |n| = 2⁻³⁴·S. m50's new `source_ref` names an existing CaseSource.
8. **A-6:** reconcile the in-domain shape count (18 against RV118's 19) by listing the shapes.
9. **NC-3:** SC2 adds a TS test of `consistentNorm` on adversarial and subnormal triples. Add it to §5.2's harness items.
10. **Regenerate DEF-C r2, its H and PTABLE r2** with a `b2c_statics_r2.py` that imports r1's and v0's sealed generators unchanged. Run it twice, byte-identical, with controls that reproduce r1's two statics. J1's reviewed inputs become SCHEMA `abf3225c…` (unchanged, if so), PTABLE r2 and DEF-C r2.
11. **Restate the estimates:** B2-K gains the formation, and B2-P loses none of its stage.

**Sealed files are never replaced in place.** Write `statics/r2/`, `_run_records/r2/`, `REVISION_02.md` and `SHA256SUMS.revision_02`. Leave CONTRACT.md, REVISION_01.md, their sums and `statics/`, `statics/r1/` untouched.

## Rules

As revision 01's brief (`R/BRIEFS/B2C_REVISION_01.md`):
- documents and code reading only, plus read-only Python with VENV (`PYTHONDONTWRITEBYTECODE=1`, `-B`, `TMPDIR` in scratch);
- no host `python3`, no cargo, no installs and no Git writes;
- absolute paths only;
- scratch in `WT/scratch/i97_b2_c/`;
- placeholder paths only, no symlink and no `build` folder;
- screen with the strict pattern, the host names (the machine's earlier host-name form named in RR, its network name, and the laptop-model form, case-insensitive) and `.local` (judged by what it names);
- run `git status --ignored`.

## Output

- `R/I97/b2_c_01/REVISION_02.md`, `statics/r2/`, `_run_records/r2/` and `SHA256SUMS.revision_02`.
- **Budget:** 2–4 h.
- **End your turn with:**
  - REVISION_02.md's sha256;
  - the regenerated hashes (DEF-C r2 raw and H; PTABLE r2; the J1 SCHEMA);
  - B2-K's cost for (ii);
  - one line per item;
  - anything for ROOT.
