# T3 V1 — backcheck of S11 containment revision 3 (S11B items only)

Type 2 TASK V1, 2026-09-26, at the T3 manager's request.

- **Rules.** Read-only. Standard-library Python only. No cargo, no Git write. T1 was read only through `git show f3270ea79`.
- **Input.** `T3/DESIGN_NUMERICS/S11_CONTAINMENT.md` revision 3, committed at `d6575c25e` (sha256 `561c72003fb7dfe4…`, verified). Revision 2 is kept at `_run_records/S11_CONTAINMENT_revision2.md`.
- **Scope.** The S11B items only (§0 table), plus whether §11 reflects ROOT's S11B-1 ruling correctly.
- **Line numbers** refer to `c61a540ea` for main and `f3270ea79` for T1.

## 1. Verdict: **FINDINGS**

Both BLOCKING items are resolved:

- **S11B-1:** the exact prescribed-motion RHS at KS1 and KS2, plus KS3.
- **S11B-2:** E15 and E16.

S11B-3 to S11B-9 are resolved in substance. §11 reflects ROOT's S11B-1 ruling.

Three SHOULD-FIX findings remain:

- **R3-1.** The "bit-identical on main" claim no longer holds now that S11-K lands after T1.
- **R3-2.** Two mutants go unkilled as the tests are specified.
- **R3-3.** The site test is name-based, and it would not have caught E15 itself.

There are four NOTEs.

## 2. The S11B items

| Item | Status | Evidence and remarks |
|---|---|---|
| **S11B-1** | **Resolved** | KS1 (`prepare_structural`), KS2 (`reduce_system_for_boundary`) and KS3 (`evaluate_original_residual` on rows coupled to a nonzero prescribed value) are each one `ExactAccumulator` sum: ledger terms, plus `add_product(∓K, g/u)` over the full row, rounded once (§4.6). All three use **exact products**, so the RHS and the refinement residual are consistent. That matters: if one side used rounded products `fl(K·g)`, refinement would reintroduce errors of about ulp(K·g). KS3 is correctly motivated, because the correction is solved from those residual rows (`FK/structural.rs:966-973`). Tests: K11 is probe P in `FK` on the dense and profile paths, `reduce_system_with_prescribed_displacements`, a forced-refinement variant and the precondition; F9 covers the 0.4.0 route. Mutations M12–M14 are each killed by a named test. The claim that everything is bit-identical when every prescribed value is 0.0 is correct *for main as it stands*: `PP:3371-3374` builds prescribed as `(i, 0.0)`, and imposed displacements are refused at `PP:1236`. **See R3-1** |
| **S11B-2** | **Resolved** | E15 (`pressure_for_pipe`) and E16 (the joint thrust aggregate) are added as exact sums (§2.2, §4.4), tested by F10 with (P, 0.3 Pa, −P) at 1e8 and 1e80 and the precondition. **The §2.5 re-scan** did not list `preview_physics.rs`, `pressure_exact*`, `membrane_publication_range.rs`, `source_receipt/*` or `stress_recovery`. I scanned them and found no further published sum over more than one source (R3-N3) |
| **S11B-3** | **Resolved in the seams; the test is weaker than stated** | The ledger, `AssembledForce` and `ReducedForce` live in `FK`, and every case-force solve seam is typed (§4.3 table). The typed seams are added beside the old ones in S11-K and switched over in S11-F. The limits are stated: generic kernels, two protected observation lanes, T5 friction. **See R3-3 and R3-N1** |
| **S11B-4** | **Resolved** | Quantum 2^-2148, 68 limbs (4260 of 4352 bits used), rounding at the binary64 quantum including the subnormal binade (no copy-bits shortcut), and +0.0 for any zero including an underflowed nonzero net. That is a stated IEEE deviation, and the ledger records underflowed DOFs. So `round()` never returns −0.0. My probe-X cases are in K1, including −2^-1080 → +0.0 under the stated rule, and mutation M15 guards against the copy-bits shortcut. **See R3-N2** for one rounding-order detail |
| **S11B-5** | **Resolved, with gaps (R3-2)** | Required kill set G ∈ {1e8, 1e80}, with G = 1e7 informative only, and one mutant per E-site (M1a–M1p) |
| **S11B-6** | **Resolved** | The precondition rule applies to K4, K11, F4, F5, F6, F9 and F10. F5 is restated correctly. The producer folds nodal loads first, then per-member eigen pairs (`source_receipt.rs:218-219`; `source_recovery.rs:1270-1274`: member k adds −N at i and +N at j). So the interior node of two colinear hot members sees (1.3, +N, −N), which is the unfavourable order. **F8 is missing from the list** (R3-2) |
| **S11B-7** | **Resolved** | The witness applies only at `FK:554`, with my boundary wording adopted. The E3–E5 witness and K5 are withdrawn. I checked D1's corrected evidence independently (`probe_s11_r3.*`, check Z). There are **941** −0.0 result values in **29** files, **4** of them `support_reaction_component_v2` (spring actions), and **245** residual strings in **31** files. All **423** −0.0 member-action rows inside committed envelopes are **stations of ordinary cases**, none of them end forces. That matches the j-side negation after the station sum (`PP:7509-7519`): an exact sum gives +0.0 and the negation still gives −0.0. Published end forces are `local_forces[dof]` unnegated (`PP:8509-8531`) and never −0.0 today. So D1's claim that "E-site folds cannot yield −0.0" holds. The E12 caveat (`multiply_matrix_vector`, `Iterator::sum`) is conservative: a restrained row's diagonal times +0.0 contributes +0.0, so the sum is +0.0 |
| **S11B-8** | **Resolved** | The per-row m_i is the audit's own operation count (`FK/structural.rs:552`), recorded per row. The floor is scoped. "Acceptance does not depend on d_i" is limited to paths where every load-like term, prescribed products included, goes through the accumulator (§5.1, §5.2) |
| **S11B-9** | **Resolved** | Counts corrected, and reproduced by check Z. The formation item is referred to `R2_ERRATUM_F2.md` / K2a |
| **§11** | **Correct** | It records that S11B-1 does not block T1, the no-interim ruling extended to the 0.4.0 support-motion route with the settlement reopen trigger, and the exact RHS joining S11-K, which lands soon after T1 merges. That matches `ROOT_RULINGS_V1.md` at `45cfc92b1` |

## 3. Findings

**R3-1 (SHOULD-FIX): KS1–KS3 are not bit-identical on the main S11-K will land on.**

- §2.4, §4.6, §8.1 and §8.3 say KS1–KS3 are bit-identical on main, and the fixture diff is "expected unchanged".
- But ROOT now has S11-K landing *after* T1 merges. By then main carries T1's committed support-motion fixtures with nonzero prescribed values. For example, `fixtures/product_preview/load_reference/connected.request.json` has UX 0.5 mm and RZ 0.001 rad boundary motions.
- The affected files are `load_reference/connected-*.raw.json`, `load_reference_source/eigen_motion-*.raw.json`, and the derived `fixtures/results/load_reference_*connected*` and `*eigen_motion*` documents, stress-neutral files and AnalysisRuns.
- Their raws publish original-residual rows in the `Debug` text (for example `residual: -4.3886270779714704e-10`). KS3's exact numerator will almost surely change those rows' low bits. KS1 may also change displacement bits.
- **Required change.** State that KS1–KS3 are bit-identical only where every prescribed value is zero. Pre-register T1's support-motion fixtures as expected diffs for the stop rule, with the reason, so the stop is anticipated and goes to ROOT as planned.

**R3-2 (SHOULD-FIX): two mutant groups are not killed by the tests as specified.**

- **F8 uses the exact order.** It specifies "two uniform loads (G, −G) plus 0.3". In that order, (G, −G, 0.3), the binary64 fold is exact by Sterbenz: check O gives 0.3 = 0.3 at both G = 1e8 and G = 1e80. So M1h–M1k (E8–E11, the curved recovery sums) would survive. F8 is also missing from the precondition list.
  - **Required change.** Use (G, 0.3, −G) or (0.3, G, −G), put F8 under the precondition rule, and give its thermal case cancelling thermal strains so E8's pre-summed strain is exercised.
- **K4 has no axial-effect loads.** It is probe A: uniform local-y loads only. That cannot kill M1b (E2, `equivalent_local_axial_effect_loads`) or the axial half of E3.
  - **Required change.** Add a K4 case with three axial-effect loads (thermal or thrust, (G, n, −G)) on one member, with the precondition.

**R3-3 (SHOULD-FIX): the site test's name rule would not have caught E15.**

- Rule 3 flags compound assignment and `iter_mut` only on bindings whose names contain `force`, `rhs` or `load`, or that are bound from expressions that mention them.
- E15 itself is `let mut pressure = 0.0; … pressure += load.magnitude.value` (`PP:10786-10798`): the name and the initializer match neither. So a future E15-class published sum in `PP` is not caught.
- Rule 6's completeness check covers only `FK`, `SA` and `nonlinear_integration`, and the claim in §4.3 limit 4 ("the site test stops a published sum being formed … outside the E-list") is overstated.
- **Required change.** Turn §2.5's table into the test's constant. Every function in `PP`, `pressure_runtime.rs`, `self_weight.rs`, `SP`, `CB` and `load_case_algebra` that contains a floating-point compound assignment or `.sum`/`fold` must appear in a named list (E-site, declared formation, integer, max, or allow-listed observation). A new one is then a visible list edit. Otherwise, state that E-list completeness is a review obligation beyond the test.

**NOTEs**

- **R3-N1.** Rules 3 and 4 will also trip on the nonlinear influence solves' unit-force vectors (`nonlinear_integration/src/lib.rs:1333`, `unit_force[…] += 1.0`, solved through the generic kernels). Add them to the allow-list as T5's.
- **R3-N2.** KS1 and KS3 round the exact sum and then radix-normalize. Where the unscaled value would be subnormal, that double-rounds. The accumulator can apply the exact power-of-two scale before its single rounding. W2 makes this unlikely, but say so or scale first.
- **R3-N3.** My scan of the files the §2.5 re-scan omitted found nothing further. `preview_physics.rs:777-778` sums combination factors only for a gate, with a Σ|c|-scaled tolerance, and publishes no value from them. No sums outside tests in `pressure_exact*`, `membrane_publication_range.rs`, `source_receipt/{composite, endpoint_maximum}.rs` (a max fold only) or `stress_recovery`.
- **R3-N4.** "M1a–M1p" is 16 labels for 15 E-sites (E1–E13, E15, E16). Relabel.

## 4. Not checked

- No product build or run, and no fixture diff.
- DESIGN revision 3 and D2 revision 3 are a separate item.
- Items outside S11B.

## 5. Run records (`T3/REVIEW/_run_records/s11_backcheck_r3/`)

- `probe_s11_r3.py.txt` → `probe_s11_r3.stdout.json`: check Z (the −0.0 counts) and check O (fold orders). Run it from `P/` with argument `.`.
- The classification of the 423 member-action −0.0 rows (ordinary-case stations, none at end forces) came from a one-off standard-library walk run inline. Its result is quoted in §2 and it can be reproduced with check Z's walk plus each row's `metadata.location` and case selection.
- Python 3.11.15, `nice 19`. Hashes are in `T3/REVIEW/_run_records/SHA256SUMS`.
