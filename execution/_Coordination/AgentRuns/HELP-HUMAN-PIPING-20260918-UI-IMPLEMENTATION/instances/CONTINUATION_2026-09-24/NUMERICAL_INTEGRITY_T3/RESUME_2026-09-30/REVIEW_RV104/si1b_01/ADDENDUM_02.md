# RV104 ADDENDUM_02: confirmation of SI1b's PR head (#1106)

TASK (Type 2) RV104, continued by ROOT. This addendum checks PR #1106's head against my review (`REVIEW.md`, `689c5e1c…`) and ADDENDUM_01 (`cd6ee1b0…`), both unchanged with their sum files still verifying. This round is sealed in `SHA256SUMS.addendum_02`. Placeholders are as before, and no machine paths are recorded.

## Verdict

**CONFIRMED.** One NOTE, with no action needed for the merge.

| Check asked | Result |
|---|---|
| 1. The head's scope | **Yes.** Exactly the 3 slice files plus the 4 package files, with no deletions. The 3 blobs equal `0730c87aef`'s and NUM `9e09bc2a35`'s. One commit on main, with no SI1b-branch or NUM commit in its ancestry. |
| 2. The package tells the truth | **Yes.** Every count, hash, claim, routed item and gate row checks out (§3). `SHA256SUMS` verifies 3/3 on the head's blobs. |
| 3. Pass B does not apply | **Yes.** No rules crate is in PP's closure, and the head touches nothing in it (§4). |
| 4. The two tools reproduce on the head | **Yes.** `source_equality.py`: 5/5 PASS, \|S\| = 3. `check_citations.py`: PASS, 1 resolved, 0 ambiguous, 0 unresolved. |

| ID | Severity | Path | Evidence | Remedy |
|---|---|---|---|---|
| A2-N1 | NOTE | `T/IMPLEMENTATION/SI1B/CHANGE_RECORD.md` §3, "(the rules crates; I61 PLAN §3)" | Four `PLAN.md` files sit under `R/I61/`: `step4_plan_01`, `u7_scoping_01`, `u8_plan_01` and `u9_plan_01`. The one meant is `R/I61/u8_plan_01/PLAN.md` §3, "S-I1's readiness" (sha256 `f274a6149051292c…`), which lists PP's dependency closure and concludes "no TEXT, no Pass B". The pointer is correct but not unique. The same phrase appears in RR. | Optional: name `u8_plan_01` in the merge record. The package is sealed, and the claim is true (§4). |

## 1. What I read

- **The head:** `WT/si1b-pr`, on branch `codex/piping-t3-si1b-pr-20261007` at **`b4f22e6ce7ccc705df919a4eb3f7bc26861dbb83`**. The worktree is clean, the remote ref is equal, and the parent is main `47a3bdfcf5a37e856465c45cf904383f10181498` (the remote `main`).
- **PR #1106 via `gh`:** open and draft; base `main`, head ref and OID as above; one commit; 7 files. The title equals the commit subject, and the body equals `PR_BODY.md` byte for byte.
- **The package:** `T/IMPLEMENTATION/SI1B/` (CHANGE_RECORD.md, PR_BODY.md, citations.json, SHA256SUMS). Its 4 blobs equal NUM `9e09bc2a35`'s.
- **The RR sections** "RV104 passes SI1b; a small test repair round; notes routed", "RV106 passes #1105; …" and "RV104 confirms SI1b's repair round; SI1b merged into NUM; PR #1106 cut".
- **`R/I79/si1b_01/REPAIR_01.md`** (`a44a274d…`) and `R/I61/u8_plan_01/PLAN.md` §3.

## 2. Check 1: scope and ancestry

Evidence: `evidence/addendum_02/scope_ancestry.txt`.

**Paths.** `git diff --name-status main head` lists 7 paths, none deleted:
- M `core/rules/expression_evaluator/src/lib.rs` (+430 −3);
- A `core/rules/rule_check_runner/tests/point_path_non_finite_run.rs` (+233);
- M `tests/test_rule_interval.py` (+5 −3);
- A `CHANGE_RECORD.md` (66), `PR_BODY.md` (32), `SHA256SUMS` (3) and `citations.json` (205), all under `T/IMPLEMENTATION/SI1B/`.

**Blobs.** For each of the 3 slice files the head, `0730c87aef` and NUM `9e09bc2a35` all carry the same blob:

| File | Blob | sha256 prefix |
|---|---|---|
| `lib.rs` | `2c72a54e9f…` | `07d9a7a525d6d151` |
| `point_path_non_finite_run.rs` | `4bb26a94df…` | `de59c4177135ac10` |
| `test_rule_interval.py` | `3bc6693373…` | `46ee5156d657039d` |

**Main has not drifted under the review.**
- Main's `lib.rs` and `test_rule_interval.py` blobs equal the review base `f8ed4f0551`'s.
- Main moved 4 commits since `f8ed4f0551`, but none touches `core/rules`, `core/units`, `core/analysis_runs`, the rule fixtures, `examples`, `tests/test_rule_interval.py` or `tests/conftest.py`.
- **So REVIEW.md's base-against-candidate results carry over to main against the head unchanged.**

**Ancestry.** One commit over main (`rev-list --count` = 1, with main as its only parent).
- None of `90d2c1ebf8`, `da0758064e`, `966113396e` or `0730c87aef` is an ancestor of the head.
- None of NUM's 778 commits not on main is an ancestor of the head.

**The commit message** is accurate: it carries the agent co-author line and states "Agent reviews, not personal review by the owner."

## 3. Check 2: the package against my records, I79's, RR and Git

**`SHA256SUMS`** verifies 3/3 on the head's blobs.

**CHANGE_RECORD.md:**
- **§1.** The +430 −3, the "8 tests" (6 + 2 in `lib.rs`), the 2 runner tests, "+5 / −3: comments and one docstring" and the three sha256 prefixes all match Git and my hashes. "NUM carries the same blobs" holds.
- **§2.**
  - The two sites and their exact messages are correct.
  - The panic was `.expect` on `EvaluationError::NonFiniteInput`.
  - The `stress_recovery` precedent exists ("{subject} recovery produced a non-finite value", `FindingCode::NonFiniteInput`).
  - The kept behaviours match my probe (REVIEW §4.4), as does the RR citation for the code ("RV103 passes #1103; …", item 1).
- **§3.**
  - The byte identity and interval invariance hold (REVIEW §4.2–4.3).
  - There is no schema, dependency, lock or product-caller change: the head touches nothing under `apps/`.
  - 94 cases and 193 pytest pass.
  - The boolean-over-non-finite item is T3-SI1c's.
  - "RV104's N-4 and N-5" matches RR's routing exactly.
  - The closure claim is true; see A2-N1 for its pointer.
- **§4.**
  - I79's 36,069 / 36,000 / 12,000 inputs with 503 base panics (18 + 276 + 209) match RETURN.md §5.
  - My 125,133 / 501,588 / 216,478, 5,877 / 2,609 and "PASS 0/0/6" match REVIEW.md.
  - The repair round, "I79's 13 and RV104's 11 all killed", "R2, R3 and R6 killed only by the repair round's two tests", and the byte-identical 626,721- and 216,478-line dumps match REPAIR_01 and ADDENDUM_01.
- **§5.**
  - The suite rows are 49 → 57, 33 → 35, 10 and 193. Main's blobs equal the review base, so "against main" is the same comparison I ran.
  - Every count change is an added test.
  - The other gate rows defer to the merge record, which is accurate for a cut head.
- **§6.** The carried obligations match RR.

**PR_BODY.md:**
- **"More than 600,000 evaluations"** is true. My two dumps alone hold 626,721 evaluator evaluations (point and interval) plus 216,478 runner lines, before I79's.
- **The figures:** "PASS (0 blocking, 0 should-fix)", "panicked 5,877 times in the evaluator" and "A repair round added assertions for its three surviving mutants" are correct.
- **"Agent reviews, not personal review by the owner"** is stated.
- **The Python line** says "comments" without the docstring. That is an acceptable summary: the change record states the docstring, and RR noted it (N-3).

**citations.json:**
- The only citation SI1b adds to maintained source is "T3 D2 §4.11.2", in `point_path_non_finite_run.rs`'s module doc. The other new comment labels (I73's `gen_400_5`; I79's `t_259_1`, `t_2856_3`) are evidence labels, as the index says.
- `num_commit` is `97f31c0f03` ("I79's SI1b repair round 01 (records only)", an ancestor of NUM).
- `source_base` is `bfb26596bf` (#1104's merge, an ancestor of main). Main's only later commit, `47a3bdfcf5` (#1105), is records only, so the maintained diff is the same.

## 4. Check 3: Pass B

**PP's lock** (`core/product_physics/Cargo.lock`) lists 15 workspace packages:
- canonical_json, curved_bend, frame_kernel, linear_supports, load_case_algebra;
- nonlinear_integration, nonlinear_supports, primitive_loads, product_physics;
- result_export, solver_diagnostics, sparse_direct, straight_pipe, stress_recovery, units.

**None is a rules crate.** There are 0 matches for `expression_evaluator`, `rule_check_runner`, `rule_pack_document` or `completeness_checker`.

**Nothing in PP points at the change:**
- PP's `Cargo.toml`, `build.rs` and `src/` contain no reference to `core/rules` or `test_rule_interval`.
- The head changes no path under `core/product_physics`, `core/serialization`, `core/solver`, `core/loads`, `core/units` or `core/reporting`.

This matches `R/I61/u8_plan_01/PLAN.md` §3. **Pass B does not apply.**

## 5. Check 4: the tools, reproduced

These are main's tools from `T/IMPLEMENTATION/F2A_D1/`; the head does not change them. Their sha256: `source_equality.py` `18786526ef328375…`, `check_citations.py` `0ccfaecb82e6f3f5…`. Each ran read-only with `GIT_OPTIONAL_LOCKS=0`, writing only into `WT/scratch/rv104_si1b_01/a2/`.

**`source_equality.py --repo WT/si1b-pr --pr b4f22e6ce7 --int 9e09bc2a35 --main 47a3bdfcf5 --package T/IMPLEMENTATION/SI1B --work …`: RESULT PASS.**
- B = `47a3bdfcf5` (NUM has absorbed main), and |S| = 3.
- Check 1 passes.
- Check 2 passes: 3 paths are identical in blob and mode.
- Check 3 passes: no merge rule is needed.
- Check 4 passes: 4 execution files, all inside the package, with no sha256 mismatch.
- Check 5 passes: 3 rows, 3 equal.

**`check_citations.py --repo WT/si1b-pr --base 47a3bdfcf5 --head b4f22e6ce7 --index <package>/citations.json --package <package>`: RESULT PASS.**
- Resolved 1, ambiguous 0, unresolved 0, with 0 verification failures.
- `D2 §4.11.2` resolves on main to `DESIGN_STANDING/DESIGN.md`.

Both agree with ROOT's head gates. I did not re-run GEN-8, which runs tests and was outside this round's host limits.

## 6. Host

- **Allowed operations only:** Git reads with `GIT_OPTIONAL_LOCKS=0`, plus one `ls-remote`; one `gh pr view` read; the two Python tools.
- **No cargo and no tests,** since DEC-025 holds the lock. No Git writes.
- **Scratch:** `WT/scratch/rv104_si1b_01/a2/`, with `TMPDIR` there. Nothing went to the system temp directory.
- **No copies or targets** were made this round.

## 7. Evidence

`evidence/addendum_02/` holds:
- `scope_ancestry.txt`;
- `source_equality.txt` and `.json`;
- `check_citations.txt` and `citations_resolved.md`;
- `pr1106.json` (the `gh` read).
