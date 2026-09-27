# RV6: independent complete-diff review of slice F1a

**Verdict: PASS**, for PR #1025 at head `b91201ee5fea5028da8511378f024effcf8de3d1`. There are no BLOCKING and no SHOULD-FIX findings, and there are 4 NOTEs.

**Reviewer.** RV6 is a Type 2 TASK. The brief is the T3 manager's spawn message: a complete-diff review of `5ae22926e..559ecb64a`, the merge check, and six review items.
- **Independence.** I did not design D-5, S11-G or SUP-17, check their designs, or implement K-D5, S11-G or F1a (I7). This is not owner review.
- **What I changed.** I fixed nothing and made no Git writes. I built only from a `git archive 559ecb64a` extraction in `<scratch>`, never in `<wt>/f1a`. I built only in the cargo slot the manager granted, with free disk at 12 GB throughout, and then removed the target (`<wt>/rv6-target`) and the extraction.

## 0. Revisions reviewed

| Item | Value |
|---|---|
| Candidate | PR #1025, head `b91201ee5` (tree `4b0c46cf…`) |
| Chain | `5ae22926e` (main, after K-D5) → `559ecb64a` (I7's F1a; tree `c6a06267…`) → `b91201ee5` (ROOT's merge of main `7e0125a7b`; parents `559ecb64a`, `7e0125a7b`) |
| Diff | `git diff 5ae22926e..559ecb64a`: 38 files, +7260 −8. Product and test code is 3 files, +728 −8: `product_physics/src/lib.rs`, the new `src/f1a_tests.rs`, and `tests/formation_check_runtime.rs` (comment only) |
| Merge check | `git diff --stat 559ecb64a b91201ee5 -- projects/chirality-piping tools .github ':!projects/chirality-piping/execution'` is empty (`_run_records/f1a/merge_check.txt`). The merge brings no piping product, tools or CI change |
| Tree built | `git archive 559ecb64a` of `projects/chirality-piping/{core,fixtures,schemas,validation}`; toolchain `rustc 1.97.1`, `RUSTUP_AUTO_INSTALL=0 CARGO_INCREMENTAL=0 --offline --locked` |
| Design basis | `DESIGN_NUMERICS/DESIGN.md` revision 5a.2 (sha256 `fb62ef4a…`): §4.3.1, D5C-3, §4.9, §5 item 5a |
| Rulings | `ROOT_RULINGS_V1.md`: "F1 split (ROOT)", "F1a: D-5 evidence-line format (ROOT)", and the S11-G and K-D5 sections |
| Brief basis | `TASK_BRIEFS/I7_F1_IMPLEMENTATION.md`, the F1a part |
| Records basis | `<wt>/numerics` read at `b92d1f2d0`. At `4b8f7e40e`, the head when this record was written, the three later commits touch only K2a; no F1a, D-5 or SUP-17 text changed |

## 1. The D-5 evidence line

PP is `projects/chirality-piping/core/product_physics/src/lib.rs` at the candidate.

- **Templates and field order.** PP:1115 and PP:1124 match the ruling's two templates exactly: `reason; row; doubled_correction; scale; trigger_ratio`, and `reason; detail`.
- **The `reason=` tokens.**
  - K-D5 defines no string form for `FormationCheckReason` (`frame_kernel/src/structural/formation_check.rs:80-86`), so the tokens are the snake_case of the variant names, as condition 1 requires.
  - `f1a_reason_tokens_are_the_enum_variants` pins each token against the variant name, read through the enum's Debug form, with an exhaustive match.
  - I7's M9a (a variant rename) and M9b (a hard-coded token) are killed.
- **f64 formatting.** The f64 values use `{:?}`. The zero-scale clause prints `trigger_ratio=inf`, pinned in `f1a_evidence_line_exact_text`.
- **The row label.** It is `integrity_dof_label` (PP:1046-1052), with its `global_dof=<i>` fallback, pinned by the `row=global_dof=12` case.
- **The `row=none` fallback (PP:1118).** It is unreachable:
  - an `Estimate` record always carries `global_dof: Some(i)` (`formation_check.rs:329-335`);
  - the unavailable template has no row field.

  It does not conflict with the ruling, which prescribes only `integrity_dof_label` and its fallback (N1).
- **Placement.** The line follows S11-G's step (PP:1098-1100 is the S11-G step, PP:1101-1107 the line) and is separated by one space. S11-G's no-op rule is untouched.
- **Byte identity with no record.** The proof is non-vacuous:
  - `f1a_no_record_is_byte_identical_to_main` compares serialized output against a copy of main's `append_integrity_report`, across both qualities and three finding states.
  - The copy is verbatim: I diffed it against `git show 5ae22926e`, and only the function name differs.
  - At product level, T9's base and candidate hash lists are identical (112 of 112), and the base list equals K-D5's combined candidate list.

## 2. The private `Option<FormationCheck>` field

- **No serialized path.** `PreviewLinearSolve` (PP:4366) derives only `Debug, Clone`. It is never formatted, and it is held in no serialized type. `FormationCheck` and `StructuralSolution` have no serde derive.
- **No other reader.** The field is read only at the linear `append_integrity_report` call (PP:3006).
- **The nonlinear call's `None` (PP:3048) drops nothing.** Nonlinear solves cannot produce a record:
  - `StructuralSolution` has one constructor (FK `structural.rs:1480`).
  - `formation` is `Some` only through `prepare_formation_checked_structural` (`:987-991`).
  - That is reached only through the adapter's `solve_assembled_with_formation_check` (`nonlinear_integration/src/structural_adapter.rs:375`).
  - That entry's only production caller is PP:4441. It passes `selected = built.nonlinear_supports.is_empty()`, and an unselected call falls back to plain `solve_assembled` (`:385-386`).
  - The nonlinear crate's `lib.rs` never references `formation_check`.

## 3. SUP-17

- **Wording.** PP:1810 matches §4.9 byte for byte, with the ruled "…" being `support_contribution_summary(&model)`. `tests::under_restrained_model_reports_solver_diagnostic` (PP:21313-21325) pins the whole message and asserts that the old phrase is absent.
- **Nothing live reads the old text.** Outside `execution/`, the old phrases occur only in PP. No TS, Python, tools or other-crate code matches "support contributions", "restrained global DOF" or either old phrase.
- **`PHYSICS_AUDIT_2026-09-05/V1/children/V1-C/TEST_ASSERTIONS.json`** is a static snapshot of the old test body. Nothing loads it:
  - It is named only in hash manifests: V1-C `MANIFEST.json`, `V1/FINAL_MANIFEST.json`, `R1/EVIDENCE_INTEGRITY.json`, and the S1 and transport records.
  - V1-C's `mutation_probe.py` and `write_coverage.py` do not read it.
  - No test, tool or CI file outside `execution/` names `TEST_ASSERTIONS` or `PHYSICS_AUDIT_2026-09-05`, apart from `.md` files.
  - The file is unchanged, so its manifests still verify.

## 4. The composition claims (RETURN §3.1), traced in code

- **A record implies a Sensitive report.** FK `structural.rs:1465-1492` runs the check only when `!ordinary_sensitive`, and sets `quality: Sensitive` if `ordinary_sensitive || formation_check.is_some()`.
- **The report and the record stay paired.** PP takes both from the same solution (PP:4469-4473). Nothing reassigns `.quality`: a grep for `.quality =` in product_physics finds nothing.
- **So the code starts as SENSITIVE (PP:1089)**, and S11-G's `demote` returns at once (`formation_guard.rs:481-484`).
- **R-b′ is a no-op too.** It runs after the recovery loop (PP:3853, `amend_integrity_report` at `formation_guard.rs:493-502`) and uses the same `demote`.
- **Hence** "demoted by both" gives the line alone, and the reverse (a sentence plus a line on one record) is unreachable.
- **No reader parses the integrity message.** `result_export/src/source_blocks.rs:263-270` and `analysis_runs/source_blocks.py:115` check `code` only.

## 5. Tests: non-vacuity and evasion mutants

**Baseline** on the archive (`_run_records/f1a/logs/BASE_*.log`):
- `--lib -- f1a_ under_restrained_model_reports_solver_diagnostic`: 8 passed;
- `--test formation_check_runtime`: 5 passed.

**RV6's mutants** (`rv6_mutate.py.txt`, run by `rv6_run.sh.txt`, one at a time on a freshly restored `lib.rs`) are all killed at behavioural assertions, with no compile errors:

| Mutant | Change | Killed by |
|---|---|---|
| E1 | the record is dropped in sparse mode (in the `PreviewLinearSolve` constructor) | `f1a_kd5_demoted_case_carries_exactly_one_d5_line`, `f1a_case_demoted_by_both_kd5_and_s11g_has_the_d5_line_alone` |
| E2 | fields reordered (`trigger_ratio` before `scale`, values under their keys) | `f1a_evidence_line_exact_text`, kd5_demoted, both_demoted |
| E3 | a line is rendered when there is no record | `f1a_non_demoted_cases_carry_no_d5_line`, `f1a_composition_with_s11g_layout_and_no_op_rule`, `f1a_no_record_is_byte_identical_to_main` |
| E4 | `{}` instead of `{:?}` for `doubled_correction` | exact_text only |
| E5 | the line is dropped whenever an S11-G finding exists | both_demoted, composition, no_record_byte_identical |
| E6 | the raw DOF index instead of `integrity_dof_label` | exact_text, kd5_demoted |

- **The first attempt** of `rv6_run.sh` stopped before any cargo command, on a typo in the disk-report line (an unbound shell variable). It was fixed and re-run. The logs are from the re-run.
- **I7's mutation logs** (M1–M8, M9a, M9b) agree with their RETURN §7 table.
- **Disk** stayed at 12 GB free throughout (`logs/disk.txt`).

## 6. Records

- **Checksums.** `sha256sum -c SHA256SUMS` in `IMPLEMENTATION/F1A/` passes 34 of 34. Coverage is exact: 34 files, 34 entries.
- **The committed product files** match RETURN §2's hashes: `lib.rs` `4768930b…`, `f1a_tests.rs` `fa31c436…`, `formation_check_runtime.rs` `faaf940d…`.
- **No machine paths.** The only `/usr` is a `#!/usr/bin/env` shebang, and everything else uses the `<wt>`, `<scratch>` and `<home>` placeholders.
- **No model identifiers** appear in the records. The commit message carries only the required co-author attribution trailer.
- **Claims.** The file:line claims in CHANGE_RECORD and RETURN match the code. RETURN §8's PP:2999 and PP:3041 are the call openings of the lines §3 cites as PP:3006 and PP:3048.

## 7. Findings

| ID | Severity | Site | Evidence | Resolution |
|---|---|---|---|---|
| RV6-N1 | NOTE | PP:1116-1118 | The `row=none` token is outside the ruled template, but unreachable (§1), and no test pins it: a changed token would survive. The reason is recorded only in RETURN §3 | Optional: a one-line code comment stating the unreachability |
| RV6-N2 | NOTE | `source_receipt.rs:914-921` | The receipt's publication reservation charges 12 × the diagnostic message bytes. On a K-D5-demoted case in a receipted captured invocation, the line adds about 150–170 bytes, roughly 2k work units against the 64e6 invocation limit (PP:824). This is negligible and fails closed (a refusal, never a wrong value), the same class as S11-G's sentences (RV4-N3). It is not disclosed | Optional disclosure |
| RV6-N3 | NOTE | PP:1791 | The SUP-17 code comment ("fewer than six independent ground DOFs …") is unchanged, although ROOT's ruling listed "a comment" in SUP-17's write set. RETURN §3(ii) discloses this, and the comment is an accurate statement of the check | None |
| RV6-N4 | NOTE | `f1a_tests.rs`, `f1a_no_record_is_byte_identical_to_main` | The unit oracle covers only `equilibrium = None`. The added branch does not depend on `equilibrium`, the nonlinear call passes no record, and T9 covers the product level | None |
