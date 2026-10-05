# RV100 addendum 01: confirmation of #1088 at H2

**Reviewer:** RV100, the reviewer who wrote `REVIEW.md` in this folder. TASK (Type 2), dispatched by ROOT; ROOT is the return path. I wrote none of the repairs or rulings.

**Request:** ROOT's relay of 2026-10-05 asked for four confirmations on H2. It points to two new RR sections, both read in full at H2:
- "RV100 passes #1088 at H; S-1 repaired by a re-cut; errata E-3 and E-4" (RR:12134);
- "Owner direction: proportionate CI; records-only PRs drop DEC-025 and the dispatch" (RR:12179).

**Placeholders** are as in `REVIEW.md`. The commits are:
- **H** = `e2b83da584`, the reviewed head.
- **H2** = `020d25a3d61c0a8f63ad92b5e0c602328b1a3ddb`.
- **N2** = NUM `f1b3a82531`.
- **M** = main `f506f3e2de`, the PR's base.
- **M2** = main `a5ecca3b598ecd145a6b08d828c792fa8ae0c81e` (#1089).
- **C** = the tree of H2 combined with M2, `5033cd22fbcb…`.

## The candidate

- **The branch.** It is H → `af7510c3a3` → H2, three commits on M. The PR's head is H2, and origin's `refs/pull/1088/head` is H2.
- **The state at 14:58Z.** The PR is a draft, MERGEABLE, with merge state CLEAN and 60 changed files. Origin's main is M2.
- **Equalities with NUM.** H2's whole tree equals N2's (`d852c4d8ecd2…`). `af7510c3a3`'s execution tree equals NUM `11a61e275d`'s.

## Verdict: **PASS** for H2

| Severity | Count |
|---|---|
| BLOCKING | 0 |
| SHOULD-FIX | 0 |
| NOTE | 5 |

The four confirmations:
1. **The H→H2 delta is execution-only text.**
2. **S-1 and N-1 to N-6 are each resolved in RR.** N-3 has a small residual (A-3).
3. **Main's move is app-v4 only,** and GEN-8 passes on the combination.
4. **The screen is clean.**

The owner-direction gate set (GEN-8, the PR's automatic CI, independent review) is met at H2:
- **GEN-8** passes at H2 and on C.
- **All four automatic runs succeeded,** on GitHub's merge of H2 into M2, whose tree is exactly C.
- **This addendum** is the independent review.

The handoff and prompt now state the proportionate gates consistently at the points ROOT named. One residual ambiguity, about the portability policy, is for ROOT to rule on (A-1).

## Findings (this addendum)

| ID | Sev. | Path | Evidence | Remedy |
|---|---|---|---|---|
| A-1 | NOTE | RR:12186 vs RR:12189; handoff "Gates before a main merge" and "Send records to main"; prompt steps 5 and 8 | **(a) The portability policy has two homes.** RR:12186 defines a records-only PR as changing "only `projects/chirality-piping/execution/`, plus at most the portability policy", which gives it the small gate set. RR:12189, handoff "Gates before a main merge" and prompt step 5 keep "the full gate set" for any PR that changes "the portability policy". A records PR that registers policy entries, as #1084 did, therefore falls under both. The policy's only readers are `tools/practitioner_harness` (GEN-8, `surface_roles.py`) and `tools/validation/validate_path_anchors.py`, both in the automatic governance-harness run. No DEC-025 suite reads it. **(b) The E-4 method sits only on the source-PR side.** It is written in the source-PR gate list (handoff) and prompt step 5. The records-PR texts (handoff "Send records to main", prompt step 8) say only "run GEN-8". | ROOT rules on (a), with my recommendation: policy appends that travel with records keep the records-only gates. Make the three texts agree, and add one line to the records-PR texts on GEN-8's method: a Git checkout of the exact head, saved with SHA and command. Do this at the next handoff revision; it does not affect #1088, which leaves the policy unchanged. |
| A-2 | NOTE | RR:12176 (the re-cut premise) and handoff "Send records to main" ("T3's `REFERENCES/` and `DESIGN_NUMERICS/` are read by `gen_k4_vectors.py`") | **Two more piping scripts read T3 paths.** RR:12176 says "the only piping code that names T3 paths" is `gen_k4_vectors.py`. `validation/benchmarks/numerical_robustness/cases/gen_vk_cases.py` and `runner/run_harness_mutants.py` also read T3's `REFERENCES/references.{json,py}` and `DESIGN_NUMERICS/_run_records/floor_kinds.json`. The conclusion still holds for #1088. None of its 60 paths is under those folders. The other piping tests that mention `execution/` use synthetic temporary trees, other packages' fixtures (PKG-15, `_DAG`), or exclude `execution` from their walks (`retained_precision_carriers.rs:1071`). `tools/ci/e2e_plan.py` classes `execution/` as non-source. | Name all three readers, or state the rule by directory ("a change under T3's `REFERENCES/` or `DESIGN_NUMERICS/` runs the suites that read them"), at the next revision. |
| A-3 | NOTE | handoff:58 ("**T3's records reached main** in two records-only PRs"), handoff:81 ("then the follow-up records PR is merged"), and the work-graph T3 row ("reached main in … #1088 … Its merge record on NUM confirms the merge") | **N-3 is mostly resolved:** handoff:6, :26 and :57, ROOT_CURRENT and the prompt now defer to `RECORDS_MERGE_2026-10-05B/`. Three past-tense phrases remain. They are true on main only if #1088 merges, and until then they are false on NUM. The handoff's framing at line 6 and its "Check it first" instruction make this low-risk. | The 05B record confirms the merge. If #1088 does not merge, add labelled corrections on NUM. |
| A-4 | NOTE | RR:12134 ("S-1 repaired by a re-cut"), RR:12170 ("the PR branch gains one commit"), handoff:60 ("repaired by a re-cut") | **H2 is not a re-cut.** The branch gained **two** commits and is three commits on M. RV96's B-2(ii) used "re-cut" for a single commit on main, and my brief required "the branch is one commit on main". **The squash outcome is equivalent,** for three reasons. Squashing H2 onto M2 with `--match-head-commit` gives one commit whose tree is C. No commit on the branch carries any of the 13 originals (trees H, `af7510c3a3` and H2: 0/13). The withdrawn S-1 wording never reaches main's history. | Word it as "repaired by follow-up commits on the PR branch, squash-merged" at the next append. |
| A-5 | NOTE | PR #1088's description on GitHub | **The PR description is stale.** It still says NUM `656d0e274d`, "31 added and 10 modified files", and "Still pending: … hosted CI with the full-SHA dispatch, and DEC-025". The head is H2 (60 files), and the gates have changed by ruling. The prepared squash body (`WT/scratch/records_pr2/squash_body.txt`) is accurate for H2, so main's log will be right. | Update the PR description before merging. |

**Observations, no finding:**
- **`run_dec025.sh` checks the guard by name** (`pgrep -f memguard.sh`). Any process whose command line contains that string satisfies the check; the old wrappers checked PID 5387.
- **`ALL-DONE` means the run completed, not that it passed.** Pass or fail is in `surfaces.txt` and the per-test comparison. The handoff's wording ("complete only at `ALL-DONE`") is accurate.

## 1. The H→H2 delta: confirmed

`git diff --no-renames --raw H H2` gives **26 paths: 19 added, 7 modified, 0 deleted**, all under `P/execution/`, with 0 paths outside. The listing is in `_run_records/addendum_01/delta_H_H2_raw.txt`.

**The paths:**
- **This folder's 18 files** (`REVIEW.md`, `SHA256SUMS` and 16 in `_run_records/`). They are **blob-identical to what I wrote** (18/18 compared with `git hash-object`).
- **The new host tool:** `host_tools/run_dec025.sh.txt`, mode 100755 like the other host-tool `.sh.txt` copies. It is byte-identical to `WT/scratch/u9_dec025/run_dec025.sh`.
- **Modified:**
  - `HANDOFF_2026-10-05_PROMPT.md`;
  - `HANDOFF_2026-10-05_TO_NEXT_ROOT.md`;
  - `CLEANUP.md` (+8/−0, addendum 2);
  - the HANDOFF folder's `SHA256SUMS` (CLEANUP.md resealed, `run_dec025.sh.txt` added);
  - `ROOT_CURRENT.md`;
  - RR (+64/−0);
  - `WORK_GRAPH.md` (the T3 row).

**The second commit.** `af7510c3a3` → H2, the owner-direction commit, changes only the prompt, the handoff and RR.

**Against main.** M→H2 is **60 paths: 50 A, 10 M, 0 D**, with an empty non-execution diff.

**RR is append-only.**
- Main's RR (1,031,126 B) and H's (1,035,698 B) are both exact byte prefixes of H2's (1,042,052 B, sha256 `671d464d928e…`).
- H2 appends 6,354 B in 64 lines, the two named sections. No earlier line is changed.

**The premise of the superseded DEC-025 carry-over (RR:12174–12176),** checked as asked:
- H2's tree outside `P/execution/` equals H's.
- No DEC-025 suite reads a changed path (A-2 corrects the stated reason).

**DEC-025 on H**, now informational, is complete:
- `meta.txt` shows head H, `ALL-DONE` at 14:48:51Z, every surface exit 0, pytest "3540 passed, 32 skipped" and vitest "3552 passed".
- Per-manifest passed, failed and ignored totals equal R3's for all 41 suite logs (`_run_records/addendum_01/dec025_H_informational.txt`).

## 2. S-1 and N-1 to N-6, and the owner direction

| Item | Recorded in RR | In the living texts at H2 | Status |
|---|---|---|---|
| **S-1** | Withdrawn (RR:12134 ff.). The rule is "main has not moved", else refresh the gates or carry them over by ruling with a stated premise and the reviewer's confirmation, recorded in the merge record. One kind is established: main's move confined to other projects' directories (nothing in piping, `tools/`, `.github/`, the policy or root build files), with GEN-8 on the combination. | Handoff Git rules, step 3.8 and "Gates before a main merge"; prompt step 5. No "disjoint" wording remains in the handoff, prompt or ROOT_CURRENT. | **Resolved.** Wording nit: A-4. |
| **N-1** | Adopted. | Handoff "how to merge" and "Send records to main", prompt step 8: `gh pr merge --squash --match-head-commit`. | **Resolved.** |
| **N-2** | Erratum E-3 (the 41 = 31 + 10 count). | Handoff:59: "FAIL, FAIL, then PASS". | **Resolved.** |
| **N-3** | Adopted. | Handoff:6, :26, :57, ROOT_CURRENT, the prompt and the work graph defer to 05B. | **Resolved,** with the residual in A-3. |
| **N-4** | Erratum E-4. The ceiling method is withdrawn, replaced by a Git checkout of the exact head with SHA and command saved. | Handoff "Gates before a main merge", prompt step 5. ROOT's `gen8_H2.txt` and `gen8_H2_combined.txt` now record the head and the command. | **Resolved.** The records-PR texts don't restate the method (A-1(b)). |
| **N-5** | Adopted. | `CLEANUP.md` addendum 2: the 1,193 removed tree-copy files, the hash list's root and the i54 mapping, and the hand move. It agrees with my re-verification (154 OK, 0 bad once mapped). | **Resolved.** |
| **N-6** | Adopted: attempt 1 void, `run_dec025.sh`. | Handoff tooling row, the DEC-025 rule (`ALL-DONE`, `dec025-INCOMPLETE`, exit 3), a lesson, and prompt step 5. The wrapper creates its folder, refuses an existing run, and checks `ALL-DONE`. | **Resolved.** |

**The owner direction** (RR:12179 ff.): records-only PRs gate on GEN-8, the PR's automatic CI and an independent review, with no DEC-025 or full-SHA dispatch. It supersedes PR1068's precedent and the "records-only re-cut" carry-over.

These are now consistent at the points ROOT named:
- **Handoff "Send records to main"** gives the small gate set and `--match-head-commit`.
- **Handoff "Gates before a main merge"** is scoped to "a PR that changes source, tests, CI, tools or the portability policy", and points records PRs to the set above.
- **The handoff's carry-over rule** names one kind only; the re-cut kind is gone.
- **Prompt step 5** is scoped like the handoff's gate list. **Prompt step 8** says "No DEC-025 or full-SHA dispatch… squash-merge it with `--match-head-commit`".
- **Product PRs and S-I1** keep their full gates.

**No leftover contradiction remains,** except the policy edge (A-1(a)).

## 3. Main's move and the combination: confirmed

- **Main's move is app-v4 only.** `f506f3e2de..a5ecca3b59` is #1089 (4 commits) and touches **34 paths, all under `projects/chirality-app-v4/`**. That is 0 in piping, `tools/`, `.github/`, the policy or root files. This meets the established carry-over condition.
- **The combination tree.** Combining H2 with M2 locally gives tree **C = `5033cd22fbcb…`**. C equals M2 outside `P/execution/`, and C's `P/execution` equals H2's (`524da973ec25…`). **GitHub's merge ref for the PR runs, `c19a8ae574` (parents M2 and H2), has the same tree C** (`gh api`). The squash would therefore produce C, if main is still M2 at the merge.
- **GEN-8:**
  - **My run at H2.** In the PR checkout `WT/records-pr2`, a clean Git checkout of exact H2 (E-4's method), it gave `1 passed, 10 deselected` (29.93 s, 14:55:37–14:56:07Z). The tree stayed clean (`_run_records/addendum_01/gen8_pytest_H2.log`, which records the head and the command).
  - **The classification replay:** H2 has 6,114 candidates, **0 findings, 0 policy issues**; C has 6,114 candidates, **0 findings, 0 policy issues**.
  - **The policy** is the same blob, `9fb269d2…`, in M, M2 and H2.
  - **ROOT's files** `gen8_H2.txt` (head H2, PR worktree) and `gen8_H2_combined.txt` (H2 with M2, unpushed) both say `1 passed, 10 deselected`. They are consistent, and the hosted run below independently covers C.
- **The PR's automatic CI on H2.** All runs were created at 14:51:53Z, after main moved at 14:45:06Z, and **all completed with success** (`_run_records/addendum_01/ci_jobs_H2.txt`):

| Run | Workflow | Result |
|---|---|---|
| 37328163454 | governance-harness | **success**: checkout `c19a8ae57 Merge 020d25a3… into a5ecca3b…`, `CHIRALITY_REQUIRE_LIVE_TESTS: 1`, "1156 passed, 48 subtests passed" |
| 37328163446 | Piping Desktop E2E | **success**: Select source coverage and Desktop E2E (source mode); the numerical, source-remainder and accessibility jobs skipped by selection |
| 37328163411 | Harness Pre-merge Validation | **success** |
| 37328163403 | pec-tests | **success** |

- **Nothing is pending** on the branch. H's dispatch 37322486897 (success) and DEC-025 on H (complete) are informational under the ruling.

## 4. Publication screen, machine paths, redacted originals: PASS

- **Credentials and whole-host data in the delta.** I searched the 699 lines H→H2 adds for the patterns in `REVIEW.md` §2: credential prefixes and key material, `password`/`secret`/`token=`/`Authorization:`, application paths and names, session and resume identifiers, `user-data-dir`, system paths, `launchd`, host names, UUIDs, e-mail addresses, process-table rows, and `/Users/`, `/private/`, `/var/folders`, `/home/`, `/Volumes/`.
  - **The only hits** are my own `REVIEW.md` prose describing the searches, a `@pytest.fixture` line in my ceiling log, and my TSV row quoting RR:11998's `/Users/<user>` placeholder.
  - **There is no credential,** and no process line, app path, session ID or host name.
  - **Every delta file** is text, JSON, Python or shell text. The largest is RR at 1,042,052 B, with no file near 50 MB.
- **Machine paths in all 60 changed files at H2** (`_run_records/addendum_01/abs_paths_H2_all60.tsv`):
  - GEN-8's detector finds **0 in every file**.
  - The broader pattern hits only the two `/Users/<user>` placeholders above.
  - **The changed living documents have 0:** RR, ROOT_CURRENT, the work graph, the handoff and its prompt.
- **The 13 originals** (`_run_records/addendum_01/redactions_H2_check.txt`):
  - REDACTIONS.json is unchanged.
  - The 13 original blob IDs appear in **0/13** of H2's, C's and `af7510c3a3`'s trees.
  - The content hashes of the 60 changed blobs match no `original_sha256` (0/60).
  - None of `dfa5e2dc44`, `59b72619fe`, `3798d5eca7` or `29160bbc1c` is an ancestor of H2.

**Integrity at H2,** checked with `shasum -a 256 -c` in the PR checkout:

| Folder | SHA256SUMS (sha256 prefix) | Result |
|---|---|---|
| `RECORDS_MERGE_2026-10-05/` | `a69497bf467b` | 19/19 OK |
| `HANDOFF_2026-10-05/` | `bea859ffe1c9` | 18/18 OK |
| `REVIEW_RV96/records_01/` | `9a22601a44bb` | 33/33 OK |
| `REVIEW_RV100/records_01/` | `4336074de298` | 17/17 OK |

RR's citation of `REVIEW.md` (`3a83f40d…`, 17/17) is correct.

## Before the merge (ROOT)

- **Confirm main is still M2,** or that any further move again meets the carry-over kind; otherwise refresh the PR runs.
- **Merge with** `gh pr merge 1088 --squash --match-head-commit 020d25a3d61c0a8f63ad92b5e0c602328b1a3ddb`, using the explicit subject and `squash_body.txt`.
- **Record in `RECORDS_MERGE_2026-10-05B/`:**
  - main's move (#1089, app-v4 only);
  - the combination tree C;
  - the hosted run on `c19a8ae574`;
  - this confirmation.
- **This addendum and its SHA256SUMS update exist only on NUM.** They are not in H2, and travel with the next records PR.

## Host and method

- **Reads:**
  - NUM with `GIT_OPTIONAL_LOCKS=0`;
  - the PR checkout `WT/records-pr2` at H2, read-only and left clean;
  - `gh` for the PR, runs, logs and the merge-ref commit;
  - DEC-025's host logs.
- **What ran:** the single GEN-8 pytest (above) and the read-only replay and scans. DEC-025 had finished and no build was running (memory guard PID 5387 running).
- **Not run:** cargo, native jobs, other tests and installs. `TMPDIR` pointed to `WT/scratch/rv100_records_01/addendum_01/tmp`.
- **Disclosure: one object-store write.** To build the combination locally I ran `git merge-tree --write-tree a5ecca3b59 020d25a3d6`. It writes the resulting tree objects into the shared object store as unreferenced loose objects: no ref, index, working tree or commit changed. That is outside the letter of "no Git writes". The tree it wrote is the same one (`5033cd22…`) as GitHub's merge commit.

## What ROOT must rule on

1. **A-1(a):** whether a records PR that appends portability-policy entries keeps the records-only gate set (recommended) or takes the full set. Then align RR's next append, the handoff and the prompt.
2. **The rest are wording,** for the next append or revision: A-1(b), A-2, A-3 (unless 05B confirms the merge), A-4. **A-5** is the PR description, before merging.

## Evidence (`_run_records/addendum_01/`)

- `gen8_pytest_H2.log` and `gen8_replay_H2_combination_summary.txt`: GEN-8 at H2, and the replay for H2 and C.
- `delta_H_H2_raw.txt`: the delta.
- `main_move_and_combination.txt`: the 34 moved paths and the combination tree.
- `ci_runs_branch_at_addendum.jsonl`, `ci_jobs_H2.txt` and `ci_governance_harness_H2_excerpt.txt`: the CI evidence.
- `abs_paths_H2_all60.tsv`: machine paths.
- `redactions_H2_check.txt`: the originals.
- `dec025_H_informational.txt`: DEC-025 on H.
