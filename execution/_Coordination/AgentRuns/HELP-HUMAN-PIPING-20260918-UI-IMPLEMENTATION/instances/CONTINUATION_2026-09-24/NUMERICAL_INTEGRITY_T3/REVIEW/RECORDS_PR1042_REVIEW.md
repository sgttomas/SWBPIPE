# RV13: independent review of records PR #1042

**Verdict: PASS.** There are no BLOCKING findings. There are 3 SHOULD-FIX findings and 9 NOTEs.
- **Every merge fact I checked holds.** That covers the PR, merge, head and base SHAs of PRs #1038, #1041 and #1040; the three chains, commit by commit; the eight CI runs, their conclusions, heads and the three dispatches' `target_base`; the three DEC-025 summaries and the three stated original hashes; the suites and T9 claims; and the review verdicts.
- **The rulings are intact.** `ROOT_RULINGS_V1.md` keeps main's 898 lines verbatim and appends 13 dated sections. Within the PR, four later edits to the appended text are insert-only: three bracketed pointers and the Q7 "SUPERSEDED" marking. There is no silent rewrite.
- **The SHOULD-FIX findings are about currency and the completeness of the ruling record, not false records:**
  - S1: stale present-tense status in the work graph;
  - S2: ROOT's decisions at RV11's delta checks, including the reversal on `force_scaled_end_actions`, have no entry in `ROOT_RULINGS_V1.md`;
  - S3: K4's Q5 ruling makes F2a wait for V-P's measurements, but V-P runs after F2a in the selected order.

## Reviewer, brief and basis

- **Reviewer:** RV13, a Type 2 TASK. ROOT (HELP_HUMAN) dispatched me directly and is my return path.
  - **Delegation mechanism:** a background subagent launched by ROOT's session on the owner's Mac (the host's agent tool); I did not delegate.
  - I wrote none of these records, made no Git writes, and ran no cargo, npm or test suite. The only test run was GEN-8.
- **Brief:** `TASK_BRIEFS/RV13_RECORDS_PR1042_REVIEW.md` (untracked; sha256 `b8e73dd7…`).
- **Read:** Root `AGENTS.md`, `agents/AGENT_TASK.md`, `TASK_BRIEFS/_COMMON.md`, "The Mac host" in `I8R_K1_RESUME.md`, RV9's `REVIEW/RECORDS_PR1035_REVIEW.md` (for format), `DESIGN_NUMERICS/DESIGN.md` revision 5a.2 (sha256 `fb62ef4a…`, unchanged by this PR) §4.1, §4.4.1, §6 and §9, `ROOT_SELECTION_DESIGNS.md`, `.agents/skills/chirality-change/SKILL.md`, the piping `AGENTS.md` (DEC-025) and the GEN-8 file filter in `tools/practitioner_harness/cmd_self_check.py`.
- **Candidate:**
  - `origin/codex/piping-numerical-integrity-20260926` = `a8895ee8457037371136de49575f8927da597814` after a plain `git fetch origin`. It is PR #1042's `headRefOid` and `<wt>/numerics` HEAD.
  - Base: main `e7d930d49`, which is also the merge base. Main did not move during the review.
  - Reviewed: the complete diff `git diff origin/main...a8895ee84`, 331 files (320 added, 11 modified), 88,372 added lines.
  - The merge of main `08e4b05fe` adds exactly main's delta: `git diff ca5a489d3 08e4b05fe` and `git diff 5a2b9d112 e7d930d49` have the same sha256, and `--remerge-diff` is empty.
  - The branch continues from PR #1035's merge `5a2b9d112` (`a2c3421d2`'s parent), whose second parent is `59660791a`. No history was rewritten.
- **PR #1042's own checks:** 7 success and 6 skipped, including `harness` (hosted GEN-8).

## 1. Scope: records only (PASS)

- **All 331 paths are under `projects/chirality-piping/execution/`:** 330 under `T3/`, plus the work graph.
- **File types:** 118 `.log`, 90 `.txt`, 30 `.json`, 21 `.md`, 11 `SHA256SUMS`, and code committed as records (`.py.txt` 27, `.sh.txt` 18, `.rs.txt` 8, `.patch.txt` 3, `.toml.txt` 1, `.inc.txt` 1), plus 3 `.stdout.txt`.
- **No product code, test, fixture, schema or tool is changed.**

## 2. Hygiene (PASS)

- **GEN-8:** `<VENV>/bin/python -m pytest tools/practitioner_harness/test_live_baseline.py -k gen8 -q`, from `<wt>/numerics` at the candidate: **1 passed, 10 deselected**. It passed again with this review's files present (`_run_records/records_pr1042/gen8.txt`).
- **Machine paths** (`/usr/bin/grep`, and `rv13_checks.py.txt` §2 over the full content of all 331 files): none.
  - The only pattern hits are in RV10's run records: a scratch-path regex built from parts (`build_records.py.txt:18`) and RV10's own description of its grep pattern (`checks/records_hygiene.txt:9-10`). Neither names a path.
  - The only root-anchored strings in added lines are the shebangs of committed scripts and `/usr/bin/grep`, a standard system tool named by the briefs.
- **Model identifiers:** none in any added or changed file.
- **SHA256SUMS:** every SHA256SUMS covering a changed file verifies and lists exactly its folder's tracked files, with none missing and none extra.

  | Folder | Entries |
  |---|---|
  | `IMPLEMENTATION/K1_MERGE/` | 10/10 |
  | `IMPLEMENTATION/K2A_MERGE/` | 4/4 |
  | `IMPLEMENTATION/K2B_MERGE/` | 13/13 |
  | `IMPLEMENTATION/K3_MERGE/` | 14/14 |
  | `IMPLEMENTATION/M03_SKEW_PIN_MERGE/` | 12/12 |
  | `PLATFORM_CALIBRATION_MAC/` | 22/22 |
  | `REVIEW/_run_records/k2b_review/` | 105/105 |
  | `REVIEW/_run_records/k3_review/` (including its `delta_2511f5a3c/`) | 108/108 |
  | `REVIEW/_run_records/k3_review/delta_2511f5a3c/` | 27/27 |
  | `REVIEW/_run_records/m03_skew_pin_review/` | 47/47 |
  | `REVIEW/_run_records/records_pr1035/` | 4/4 |

  - The changed files directly in `T3/`, `REVIEW/` and `TASK_BRIEFS/`, and the work graph, are outside any maintained SHA256SUMS, as in earlier records PRs. `REVIEW/_run_records/SHA256SUMS` lists only the design-stage check files (76/76 verify, unchanged here); no slice review has been added to it.
- **`git diff --check`** over the whole diff: clean.

## 3. Rulings integrity (PASS)

- **`ROOT_RULINGS_V1.md`:** in the net diff, lines 1–898 are main's, verbatim. Lines 899–1150 are 13 appended sections, each headed "(ROOT, 2026-09-28)".
- **Commit by commit** (`rv13_checks.py.txt` §4), four edits change earlier appended text. Each keeps the old text verbatim and only inserts:
  - `a95adb540`, :924 and :957: bracketed pointers to "K2b: rulings on I10's checkpoint-A stop";
  - `ffc9ea275`, :1050: the heading gains "— SUPERSEDED", and a paragraph names the superseding section and says the text is kept;
  - `4ec82a9b3`, :1084: the bracketed RV11-3 correction of ROOT's own `97000ab9f` sentence.
- **The other modified files:**
  - `HANDOFF` :82, `OPERATING_NOTES` :64 and `K2A_MERGE/RECORD.md` :54: one bracketed insert each, main's text kept;
  - `K1_MERGE/RECORD.md`, `PLATFORM_CALIBRATION_MAC/RECORD.md` and `OWNER_DIRECTION.md`: appended sections only, each dated;
  - `REVIEW/M03_SKEW_PIN_REVIEW.md` :3: RV10's own delta-check sentence appended to its verdict line (`bea34e7f0`, which cites the new hash `8db9ec52…`);
  - `ROOT_RULINGS_V2.md`: unchanged.
- **Two in-place rewrites, both legitimate:**
  - The work graph's T3 row and next-safe-action bullet are rewritten as execution state. The K1-spawn text is kept with a "[Superseded …]" pointer, and the figures were replaced by citations, per RV9's N2. Currency is S1.
  - `TASK_BRIEFS/I12_K4_IMPLEMENTATION.md` was brought to its spawn state in `ca5a489d3`, which says so. The write-set statuses change from "ROOT to rule" to the rulings, and the K2b status to "merged". The questions and options are kept "for the record".
- **RV9's findings on PR #1035** are dispositioned as `a2c3421d2` says: S1, S2 and S3; N1–N6 and N8. N7 (optional) was not taken, and N9 needed no action.

## 4. The merge records (PASS on every fact; N1, N2, N3)

The facts are in `_run_records/records_pr1042/github.txt`.

### M03_SKEW_PIN_MERGE/RECORD.md

- **PR #1038:** MERGED 2026-09-28T07:53:23Z as `e8b416e433533fe4…`, with parents `6e18505e3` and `5dd6dfdd8`. The head is `5dd6dfdd8fe1e49c…` and the base is `6e18505e3`. The `--match-head-commit` flag itself is not observable; the second parent equals the stated head.
- **Chain:** `885f065e5` (parent `eb52114e9`; `m03_skew_scope.rs` +947, `k1_tests.rs` +150), `39dfd69c7` (61 record files), `1d105d633` (a merge whose second parent is `6e18505e3` and whose message says `5a2b9d112`, as disclosed, also in the PR body), `5dd6dfdd8` (4 record files).
  - Main's changes `eb52114e9..6e18505e3` are App v4 files, T3 records, the work graph and one governance tranche manifest. None is a piping product path.
- **CI:** 36391476996 (pull_request, `5dd6dfdd8`), 36387730033 (pull_request, `1d105d633`) and 36387729465 (workflow_dispatch, `1d105d633`, `target_base` `6e18505e38520ef3…`): all success. The rollup is 12 success and 4 skipped.
- **DEC-025:** the JSON is commit `1d105d633`, `working_tree_dirty: false`, only `sandboxed`, cargo exit 101, later surfaces `not_run`.
  - The original in `<wt>/scratch/sweep_skewpin/` hashes `f96a9253…`, as stated, and differs from the committed copy only in the `<VENV>` path.
  - `suites.log`: 39 manifests. Against K1's combined-tree Mac run (the `eb52114e9` product tree), only FK 179 → 184 and NI 101 → 102 change. The failures are the three platform tests.
  - pytest 3023 passed and 32 skipped; vitest 134 files and 2822/2822; the build passed.
- **T9:** I9's base and candidate lists both equal the Mac calibration's native main list, 112/112.
- **Review:** RV10's verdicts, counts, the 864 reproduced runs and its six mutants (five killed, DEEP-REFUSE-1019 surviving as N2) are as stated.

### K3_MERGE/RECORD.md

- **PR #1041:** MERGED 2026-09-28T11:22:51Z as `57617b0fbfa6e59a…`, with parents `98b1723b1` and `2511f5a3c`. The head is `2511f5a3c73fd95f…`.
- **Chain:** the nine commits match GitHub's list and their stated content.
  - FK tests: 42 at `74add6078`, 43 with the guard, 45 at the head.
  - `FK/Cargo.toml` at `e83e22356` equals `98b1723b1`'s (empty diff).
  - `e62837f7e` touches only `tests/retained_wide_k3/`.
- **CI:** 36413698754 (pull_request, `2511f5a3c`; Numerical cargo suite 11:07:15Z–11:19:09Z, 11.9 minutes), 36404664521 (pull_request, `b7e93650e`) and 36404663005 (dispatch, `b7e93650e`, `target_base` `98b1723b1b263cf3…`): all success. The rollup is 12 success and 4 skipped.
- **DEC-025:** commit `b7e93650e`, clean, cargo exit 101. The original hashes `b3eeb351…`, as stated.
  - Against M03's run (the `98b1723b1` product tree), only FK 184 → 227 changes.
  - vitest failed 1 of 2822: `App.deadControls.test.tsx`, an `AssertionError` from a click-observation timeout. The re-run passed 2822/2822 (N3).
- **T9:** I11's lists and RV12's head list equal the calibration list, 112/112.
- **Review:** "PASS at `b7e93650e`: 0 BLOCKING, 2 SHOULD-FIX, 7 NOTE", then the delta PASS with D1, match `K3_REVIEW.md` (`b477547f7`, `e63325363`).

### K2B_MERGE/RECORD.md

- **PR #1040:** MERGED 2026-09-28T12:05:31Z as `e7d930d493bf5b2f…`, with parents `57617b0fb` and `33e33c723`. The head is `33e33c723a974a86…`.
- **Chain:** the 13 commits match GitHub's list and their stated content. The base is `eb52114e9`, and `087b3a088` and `33e33c723` merge main `98b1723b1` and `57617b0fb`.
- **CI:** 36416556943 (pull_request, `33e33c723`; Numerical cargo suite 11:35:55Z–11:46:59Z, 11.1 minutes) and 36416551310 (dispatch, `33e33c723`, `target_base` `57617b0fbfa6e59a…`): both success. The rollup is 12 success and 4 skipped.
- **DEC-025 on the final head:** commit `33e33c723`, clean, cargo exit 101. The original hashes `350a3d83…`, as stated.
  - Only FK 227 → 249 and NI 102 → 120 change, and the failures are the three platform tests. The baseline's description is N1.
  - pytest, vitest (2822/2822) and the build pass.
  - The earlier `087b3a088` vitest failure excerpt is `App.test.tsx`, "Test timed out in 30000ms", as disclosed.
- **T9:** I10's checkpoint-B and RV11-fix lists all equal the calibration list, 112/112. T9 was not re-run after the RV11D fixes (disclosed in K2B RETURN A2.7: new functions only).
- **Review:** FAIL at `087b3a088` (1 BLOCKING); PASS at `f385a8bc8` (146,602 values, 0 wrong); PASS at `112c1729d` (537 solves); confirmed at `33e33c723` (439/439). These match `K2B_REVIEW.md` at `833d69b96`, `90ab6f1f9`, `255fce346` and `cf44386ae`.

### Timing of the gates

Each PR merged after its last review commit, its last CI run and its DEC-025 evidence:
- M03 at 07:53:23Z, after the review (07:29), CI (07:35) and the sweep (07:09);
- K3 at 11:22:51Z, after the review (11:19:02), CI (11:19:17) and the vitest re-run (ended 11:22:00);
- K2b at 12:05:31Z, after the confirmation (11:43:10), CI (11:47:15) and the sweep (ended 12:04:27).

## 5. The reviews as filed (PASS; N5)

- **Every verdict and count** stated in the merge records, the rulings, the work graph and the PR body matches the review files at the cited commits.
- **The review files grew by appends only:** K3 by one delta section; K2b by two delta sections and a merge confirmation; M03 by §9 plus RV10's one-sentence addition to line 3. Their sha256 at each commit are in `github.txt`.
- **The hashes cited in commit messages hold:** `dc609ce7…` (M03 review at `d2df479f3`), `8db9ec52…` (at `bea34e7f0` and the head) and `769090f5…` (RV9's review).
- **The run records** of all four reviews verify (§2).

## 6. The rulings' history (PASS on honesty; S2)

- **Q7 and its reversal:** honest.
  - The original ruling (`973438aa7`) is kept and marked SUPERSEDED. The reversal (`ffc9ea275`) names ROOT's false premise, the `powi` constant-folding mechanism and the test that caught it.
  - `K3/RETURN.md:549` records the failing figure, 1.1364e-13 against 1.1378e-13.
  - The CI times it cites hold: 3.7 and 5.4 minutes on 36380608240 and 36391476996.
- **The b-rule "third attempt" framing:** honest and consistent. V1 :1069, K2B `RETURN.md:5` and `:587`, the merge record and the PR body all say ROOT asked for "equivalent by construction" and was wrong.
- **RV11-1 and RV11-3:** V1's RV11 section matches the review, 1 BLOCKING, 3 SHOULD-FIX and 7 NOTE. The premise correction is an insert at :1084 that keeps ROOT's wrong sentence. RV11's delta check confirms the correction (`K2B_REVIEW.md`, RV11-3 "resolved").
- **The reversal on `force_scaled_end_actions`:** honest where it is recorded, but not recorded in the rulings (S2).
  - "ROOT's decision 2" is `K2B_REVIEW.md:225`, `:237`, `:241-250`.
  - "ROOT reversed its decision" is K2B `RETURN.md:1192`.
- **`d2df479f3`:** disclosed in `bea34e7f0`'s message (with `dc609ce7…`), in the M03 merge record and in the PR body. `d2df479f3`'s own message omits it, as the disclosure says.
- **`1d105d633`:** the wrong main SHA in its message is disclosed in the merge record and in PR #1038's body. The parent is correct.

## 7. K4's brief and rulings (PASS with S3; N9)

- **The pins hold.** The brief pins `DESIGN.md` at `fb62ef4a…`, which is its hash at the head.
- **Q4, no p + 64 residual at the ceiling:** consistent, as an explicit, recorded departure.
  - §4.1.6 makes 128, 256 and 512 the only candidates, so the 1024 solve is only ever the 512 candidate's verification (q_2p).
  - §4.1.4 step 2's p + 64 residual needs 1088 bits there. §4.1.1's own widths stop at L = 16 (1024 bits).
  - The ruling (brief :131-133; V1 :1145) keeps an exact-expansion residual at 1024, the 64·γ_1024 gate and three corrections, and records the residual basis as p.
  - The argument checks:
    - the 512 candidate passed its §4.1.3 screen, so rcond > 2^-511;
    - a backward error gated at about m·2^-1018 gives a forward error of about m·2^-507;
    - the formation error of K at 1024 adds about cond·c·2^-1024, also far below 2^-64·S\*.
  - What is lost is §4.1.9's independence of the residual from formation error. The forward-error bound covers that loss, and the ruling requires the argument written out, "including its reliance on the uncertified rcond estimate", and checked.
- **Q5, the reading of §4.1.7:** recorded explicitly as ROOT's reading, but its sequencing contradicts the selected order (S3).
- **The other rulings** each match the design, or are explicit, recorded departures:
  - Q1 and Q12 match the §6 K4 row;
  - Q2 (a new eighth `retained/` file) and Q3 (the `exact_sum.rs` accessor, which §4.1.2 item 5 anticipates) are declared write-set extensions;
  - Q6 (`DirectionalSpring`, against §4.1.1's springs), Q7 (the RCM port, against §4.1.3) and Q9 (MOD-D) are in V1's list;
  - Q10 moves the digest itself downstream of §4.1.1's "identity digest". It is ruled in the brief and summarized in V1 as "canonical encodings, hashed downstream";
  - Q11 matches §5 item 7 through list item 8.
- **The stale-design list against main `e7d930d49`:**
  - `retained/` holds `wide.rs` and `wide/multi.rs` with L = 4, 8 and 16 (items 1 and 5);
  - `ExactAccumulator` takes binary64 terms and keeps separate positive and negative magnitudes at `exact_sum.rs:46-47`, with `compare` :103 and `subtract` :113 (items 2 and 3);
  - `reverse_cuthill_mckee` is at `sparse_direct/src/lib.rs:505`, and `sparse_direct` depends on FK, not the reverse (item 4);
  - `Representability` and `PublishedValue` are at `structural.rs:2286` and `:2297`, and `Binary64Outcome` is at `multi.rs:666` (item 8);
  - `mod retained;` at `structural.rs:5` is private (item 9);
  - `AXIS_TOLERANCE = 1.0e-12` gives `DegenerateAxis` (item 10);
  - every line cited in the brief's basis item 11 is current, including `finish_structural` :1883 and the public `factor_structural_profile` :2027.
  - The list is accurate. Its heading is N9.

## 8. Currency (S1; N6, N7, N8)

- **The work graph** carries stale present-tense status (S1) and omits PR #1035 (N6).
- **The handoff and the operating notes** are dated cloud-era snapshots with present-tense status (N7). The handoff's §2 defers to the work graph.
- **"K4 is in implementation" is current.** `<wt>/k4` is on `codex/piping-k4-20260928` at `e7d930d49`.

## Findings

| ID | Severity | Site | Evidence | Resolution |
|---|---|---|---|---|
| S1 | SHOULD-FIX | `WORK_GRAPH.md` :62 (next safe action) and :35 (T3 row) | **Stale status presented as current.** (a) :62 still says "next on the kernel path: K2b, then F1b; the skew M03 pin is open (K5, or a tests-only follow-up)", then later that both merged. It then says "Next on the kernel path: K5 (after K2b), and K4 (after K1 and K3; brief drafted, with K3's handovers)". At the head K4 is in implementation (I12, from `e7d930d49`), as :35 says ("Next on the kernel path: K4 …, then K5 and K6"). The "Active now" clause names no active slice. (b) :35 still labels the skew pin "**open:** not taken by K1 …" inside the same bold span that says it was "done … merged as PR1038". (c) :35 still says "Also run a better-conditioned skew model (G comparable to E) on main, to establish main's downstream standing, which is not established beyond RV7's probe." V1 :1041, "K2a product reach: main's skew standing is now established", rules it established by I9's runs, as reproduced by RV10. | Update :62 in place: K2b and the skew pin merged; K4 in implementation (I12, `e7d930d49`), then K5 and K6; F1b unblocked; active now K4. Remove "open:", or mark it superseded. Mark the "Also run …" sentence done, with a pointer to V1 :1041. |
| S2 | SHOULD-FIX | `ROOT_RULINGS_V1.md` (no entry); `K2B_MERGE/RECORD.md:53` ("Rulings and findings, as recorded in ROOT_RULINGS_V1") | **ROOT's decisions at RV11's delta checks, and one reversal, are recorded only in K2b's RETURN and RV11's review.** (1) "ROOT's two decisions stated at resume" (`K2B_REVIEW.md:225`, `:241-250`): the reaction check applies at every b, b = 0 included (decision 1, which I10 left as "ROOT's call", K2B `RETURN.md:965`); and `force_scaled_end_actions` stays off the pin list, "covered indirectly" (decision 2). (2) ROOT reversed decision 2 after RV11D-N1 (`RETURN.md:1192`, "ROOT reversed its decision"). (3) ROOT's message rulings on RV11D-1, RV11D-2, N1, N3 and N2 (`RETURN.md:1129-1134`). V1's last K2b section is "rulings on RV11's review", items 1–5. Yet the merge record's section heading says "as recorded in ROOT_RULINGS_V1" and lists RV11D-1 and RV11D2-N1. Its F1b list omits RV11D-N2 ("as ROOT ruled": the b = 0 refusal is stricter than E12, and F1b's gate measures it). The history is honest, but the ruling record is incomplete. It is the same class as RV9's S3. Q7, by contrast, has both its ruling and its reversal in V1. | Add a dated V1 section, "K2b: ROOT's decisions at RV11's delta checks": decision 1; decision 2 and its reversal (RV11D-N1); and the RV11D-1, RV11D-2, N1–N3 rulings. Put a bracketed pointer at K2B_MERGE :53, and add RV11D-N2 to its "For F1b" list. |
| S3 | SHOULD-FIX | `TASK_BRIEFS/I12_K4_IMPLEMENTATION.md:134-137` (Q5 ruling); `ROOT_RULINGS_V1.md:1132` and `:1146` (item 6) | **The Q5 ruling makes F2a wait for V-P, which runs after F2a.** The ruling says "F2a does not merge without ROOT's limits, which are set from the K6 and V-P measurements (C4)". Item 6 says §4.1.7 "binds the product-wiring slice (F2a), per C4's measurement order". But V-P comes after F2a everywhere it is ordered: DESIGN §6's V-P row is "after F1 and F2a" (:1031); the order is "… F3 … Then V-P and the join" (:1042); and `ROOT_SELECTION_DESIGNS.md`, Selected item 2, has "F2a … → F3 … → V-P → join". D-8 (:1270) is "Select after the K6 and V-P measurements". As written, F2a cannot merge. The ruling moves §4.1.7's tension from K4 to F2a without resolving it, and records no reorder. §4.1.7 itself says "W3/W5 measurements", which would admit V-K's kernel lane after K4. K4 is unaffected: it ships the mechanism only. | Amend the Q5 ruling before F2a's brief, and record which of these holds: (a) F2a's limits come from K6 and V-K, and V-P confirms or revises them later; (b) V-P's product-lane measurement runs on F2a's candidate before merge, recorded as a departure from the selected order; or (c) F2a merges with W1 unselectable until the limits exist. Correct item 6 to match. |
| N1 | NOTE | `K2B_MERGE/RECORD.md:44`; the three `dec025/` folders | (a) "Against the Mac run of main's tree, only frame_kernel (227 → 249) and nonlinear_integration (102 → 120) change, by K2b's tests." The baseline is the K3 candidate `b7e93650e` ("main 57617b0fb less 2 K3 tests", `dec025/suites_vs_baseline.txt`), and 2 of FK's 22 added tests are K3's RV12 follow-ups (`e62837f7e`). By `#[test]` counts, K2b adds 20 FK and 18 NI tests from `57617b0fb` to `33e33c723`. The failure claim holds. (b) The driver `dec025_mac.sh.txt` is committed only in `M03_SKEW_PIN_MERGE/dec025/`. The K3 and K2b folders neither carry nor cite it, though their `meta.txt` format matches it. | Optional wording fix: "against the K3 candidate's Mac run (main less K3's 2 follow-up tests): K2b adds 20 FK and 18 NI tests". Cite the driver from the K3 and K2b records. |
| N2 | NOTE | `K3_MERGE/RECORD.md:12`, `:21`, `:36` | (a) "RV12 checked the final head's outputs": RV12's T9 ran at `b7e93650e` (`K3_REVIEW.md` §3.1). The final head `2511f5a3c` changes only FK tests, so the claim holds in substance. (b) "its piping tree equals `eb52114e9`'s" holds outside `execution/` (`core`, `fixtures`, `validation`, `schemas`), not for the whole piping tree. (c) The `e83e22356` row omits the guard test's four-line doc-comment rewording (`K3_REVIEW.md` §3.8). | Optional wording fixes. |
| N3 | NOTE | `K3_MERGE/RECORD.md:43-44`; `dec025/vitest_rerun/meta.txt`; V1 :1123 | **The vitest disclosures are true on the pass, but loose on the host.** (a) The re-run is described as "on a quiet host … load average about 3 at the start". Its committed `meta.txt` ends with load 11.91 / 9.54 / 7.32 at 11:22:00Z, higher than the load "above 8" blamed for the first failure. (b) "while two reviewers and pytest ran": the sweep's own `surfaces.txt` shows its pytest ending at 10:10:57Z, before vitest started at 10:11:50Z. The "above 8" loads here and at K2b's `087b3a088` are not in committed evidence. (c) The re-run's `meta.txt` records no head. The sweep worktree's reflog (uncommitted) shows `2511f5a3c` checked out at 11:19:33Z, one second before the start, so the claim holds. The pass (134 files, 2822/2822, no timeout raised) holds. | Optional: add the end load, and "head per the worktree's reflog". Record `git rev-parse HEAD` and the load in future re-run metadata. |
| N4 | NOTE | `K1_MERGE/RECORD.md:47`; the M03, K3 and K2b merge records; the work graph | `OWNER_DIRECTION.md` now separates the owner's words ("Run the sandboxed sweep on the Mac for K1") from ROOT's extension to later slices, which it marks "not the owner's wording". But K1_MERGE's heading still reads "the owner's decision for Mac-run slices", with no pointer; RV9's S3 resolution asked K1_MERGE to cite the entry. The later records and the graph say "under the owner's Mac decision". | Optional: a bracketed pointer at K1_MERGE :47, and "under ROOT's application of the owner's K1 decision (`OWNER_DIRECTION.md`)" in later records. |
| N5 | NOTE | `REVIEW/M03_SKEW_PIN_REVIEW.md` §5 and §9, `_run_records/m03_skew_pin_review/checks/gen8.txt`; `IMPLEMENTATION/K2A/RETURN_ADDENDUM_1.md:71` | **Two items raised in reviews have no recorded disposition.** (a) RV11-N7 found GEN-8 unreliable on a `git archive` nested in the ignored outer worktree. RV10's GEN-8 runs were exactly that: `<wt>/scratch/rv10/full`, which lies inside the outer worktree and is ignored by its `.gitignore` (line 6), while its record says "outside a Git work tree the self-check walks every file". By `cmd_self_check.py:750-759`, only untracked AgentRuns files of an active role are then scanned, so the run was partial. No merge record relies on it. Hosted `harness` passed on both PR #1038 heads, and GEN-8 passes here. (b) RV10's S2(a) and N7 flag K2a addendum :71, "6EI/L² against the floor", as imprecise: the skew threshold lies up to about 1.6 binades above the floor. It is marked "for ROOT", with no pointer or ruling. | Optional: add RV11-N7's method lesson to the operating notes (GEN-8 only in a git working tree of the candidate). Put a bracketed pointer at K2a addendum :71 to RV10 S2(a). |
| N6 | NOTE | `WORK_GRAPH.md:35` | The T3 row records every earlier records PR (#991, #1001, #1004, #1019, #1029), but not PR #1035: the records through `59660791a`, merged at `5a2b9d112` on 2026-09-28T06:09:22Z after RV9's PASS. | Add one sentence, as for PR #1029. |
| N7 | NOTE | `HANDOFF_2026-09-28_TO_LOCAL.md` §1, §2, §7; `OPERATING_NOTES_FOR_LOCAL_ROOT.md` §5 | Both are dated cloud-era snapshots, but they speak in the present tense. The handoff says K2a "is in its gate now", "The two-part gate is running", K1 is "A WIP commit only", and "Open agents in the cloud session …". The notes count "four rulings" withdrawn; this PR adds Q7, the third-attempt framing, the RV11-3 premise and the pin-list decision. §2 of the handoff defers to the work graph. | Optional: one bracketed banner per file, "status as of the handoff; for current status see the work graph's T3 row". |
| N8 | NOTE | GitHub descriptions of PRs #1041 and #1040 (outside this diff) | Both merged bodies still end "To come: the independent review, and DEC-025 under the owner's Mac decision". #1040's never mentions RV11's FAIL or the fixes. The merge records carry the facts. | Optional: edit the merged descriptions to point to the merge records. |
| N9 | NOTE | `ROOT_RULINGS_V1.md:1140` | The heading "Design text made stale by K1, K2b and K3" also covers design-internal gaps rather than staleness. Item 5's 1088 bits already exceed §4.1.1's own L = 16. Item 6 is §4.1.7 against C4, and item 7 is §4.1.1 against §4.10. Each is still an explicit ruling, so nothing is unrecorded. | Optional: "made stale or found inconsistent". |

## What I did not do

- **No builds or runs** of product code, suites, gates or T9. §4's suites and T9 facts are re-derived from the committed files.
- **Not re-reviewed:** the K3, K2b and skew-pin code, and the reviewers' mutations. The merge records' review claims were checked against the review files at the cited commits.
- **Uncommitted evidence I read:**
  - the three original sweep JSONs in `<wt>/scratch/sweep_*`;
  - the sweep worktree's reflog;
  - `<wt>/k4`'s branch and HEAD.
  - I did not verify the host memory guard, the Mac env caps beyond the committed driver, or the "above 8" loads (N3).
- **Not reviewed:** the untracked RV13 brief, which is not in the candidate. The briefs I9–I12 and RV11–RV12 were read for their rulings, pins and cited lines, not line by line.
- **Side effect:** my first GEN-8 run refreshed the ignored `.pytest_cache/v/cache/nodeids` in `<wt>/numerics`. The second ran with `-p no:cacheprovider` and `PYTHONDONTWRITEBYTECODE=1`. `git status` shows only the untracked brief and my write set.
- **No Git writes.** My only writes are this file and `_run_records/records_pr1042/`, all uncommitted.

## Evidence

`REVIEW/_run_records/records_pr1042/`, with its own `SHA256SUMS`:
- `rv13_checks.py.txt`: the read-only check script (standard-library Python, run from the repository root with `a8895ee84 origin/main`);
- `checks.stdout.txt`: its output (scope, hygiene scans, SHA256SUMS, in-place edits, suites, T9);
- `github.txt`: the GitHub and Git facts for §4 to §6, the original sweep hashes and the reflog lines;
- `gen8.txt`: both GEN-8 runs.

## Delta check at 7df3acefe

**Delta verdict: PASS.** All three SHOULD-FIX findings are resolved, and every NOTE is resolved or has only an optional remainder. The check adds 3 new NOTEs (D1–D3) and no BLOCKING or SHOULD-FIX finding. The review's verdict stands: PASS.

**Scope.** ROOT resumed me for this check (mechanism unchanged: a background subagent of ROOT's session; no delegation, no Git writes).
- **Head:** PR #1042's head is `7df3acefe7d04e5520403d8c7464b40c7e615d77`, verified after a plain `git fetch origin`; main is still `e7d930d49`.
- **The delta `a8895ee84..7df3acefe` has two commits:**
  - `4c82ea736` adds my seven files verbatim: the review hashes `ca4d715a…`, and the run records and brief match what I wrote.
  - `7df3acefe` is the fix: 13 files, +50/−17.
- **`<wt>/numerics` moved during the check.** ROOT added two local commits on top of the PR head: `bacf939a8` (K4 checkpoint-0 rulings) and `28bb8dfa8` (the F1b brief I13). They only append to `ROOT_RULINGS_V1.md` (+84 lines) and add one brief. They are not in the PR, and I did not review them. Every check below reads `7df3acefe` by commit, except the local GEN-8 run (see Hygiene).
- **Evidence:** `_run_records/records_pr1042/delta/`.

### Findings: resolution

| ID | Status at `7df3acefe` | Evidence |
|---|---|---|
| S1 | **resolved** | :62 has three changes. The stale clause is kept, with "[Superseded: the skew pin and K2b merged; see below.]". The kernel sentence is rewritten to "K4 (W1a), **in implementation** (I12, from `e7d930d49`, 2026-09-28), then K5 and K6", which agrees with :35. "Active now: K4 (I12), and F1b's brief" is added; F1b's draft brief exists, uncommitted, in `<wt>/scratch/briefs/`. At :35, "open:" becomes "[closed]:", as proposed ("Remove 'open:'"), and the "Also run …" sentence gains "[Done: established by I9's runs … `ROOT_RULINGS_V1.md`, 'K2a product reach: main's skew standing is now established'.]". |
| S2 | **resolved** (D1) | The new V1 section "K2b: ROOT's decisions at RV11's delta checks (ROOT, 2026-09-28; recorded late, RV13-S2)" is at :1152. Checked against K2B `RETURN.md` (:963–965, A2.4 at :1192, addendum 2's rulings at :1129–1134), `REVIEW/K2B_REVIEW.md` (:225, :237–238, :241–250) and my review: decision 1 and its F1b consequence, RV11D-N2, decision 2 and its reversal after RV11D-N1 (RV11D-PIN-ACTIONS-EVASION), both helpers in `FORCE_SCALED_ENTRY_POINTS`, the five addendum-2 rulings, and RV11D-N3 for F1b all match. K2B_MERGE :53 has its bracketed pointer, and RV11D-N2 is added to the "For F1b" list. |
| S3 | **resolved** (D2) | The new section "K4: Q5 amended (ROOT, 2026-09-28; RV13-S3)" is at :1169, with bracketed pointers at V1 :1132, :1146 (item 6) and I12 brief :137. It adopts option (a): limits from K6 and V-K plus K4's deterministic counts, with V-P confirming or revising after F2a, and a revision is its own ruling. It states the error accurately against DESIGN §6's V-P row ("after F1 and F2a") and the selected order. |
| N1 | resolved; optional remainder | (a) The bracketed correction at K2B_MERGE :44 is exact: the baseline `b7e93650e`, the 2 K3 tests, and K2b's 20 FK and 18 NI tests. It also cites the driver. (b) K3_MERGE still does not cite `dec025_mac.sh.txt` (optional). |
| N2 | resolved; optional remainder | (a) and (b) have bracketed inserts at K3_MERGE :36 and :12. (c) The `e83e22356` row's doc-comment omission is not taken (optional). |
| N3 | resolved; optional remainder | The bracket at K3_MERGE :44 adds the end load (11.91 at 11:22:00Z), the head by reflog rather than `meta.txt`, and that the "above 8" loads are uncommitted. "while two reviewers and pytest ran" (N3(b)) stays unchanged (optional). |
| N4 | resolved; optional remainder | The K1_MERGE :47 heading gains the pointer to `OWNER_DIRECTION.md`. The later records still say "under the owner's Mac decision" (optional). PR #1041's edited description now says "ROOT's application of the owner's Mac decision". |
| N5 | resolved (D3) | (a) is a new first bullet in the operating notes' §6, matching `cmd_self_check.py:750-759`. (b) is a bracketed pointer at K2a `RETURN_ADDENDUM_1.md:71`, citing RV10's S2(a) and N7 and the scoped M03 RETURN §3.4. |
| N6 | resolved | The PR1035 sentence is added to :35, and its facts hold: `59660791a`, `5a2b9d112`, RV9's PASS, and the fixes in `a2c3421d2`. |
| N7 | resolved | Identical dated banners are inserted at the top of the handoff and of the operating notes. |
| N8 | resolved | GitHub descriptions of PRs #1041 (updated 13:07:12Z) and #1040 (13:07:14Z) no longer contain "To come". Each now has a "Done since this description was written (edited after merge, 2026-09-28)" line naming its merge record, which exists at `7df3acefe`. #1040's names RV11's FAIL at `087b3a088` and the fixes. |
| N9 | resolved | V1 :1140 gains "[or found inconsistent within the design (RV13-N9)]". Inserting it moved one comma, from inside the bold span to after the bracket; every word is kept. This is text this PR added, not main's. |

### In-place edits (`delta/delta_checks.stdout.txt`)

- **Against main**, main's text survives verbatim, character for character, in every main file the fix touches except the work graph: the handoff, the operating notes, `K1_MERGE/RECORD.md`, K2a's `RETURN_ADDENDUM_1.md` and `ROOT_RULINGS_V1.md`. V1 is still a pure append after main's line 898: one hunk, +278 lines.
- **Against `4c82ea736`,** every edit is insert-only except two:
  - the work graph's rewrites, which are execution state and exactly S1's proposed resolution ("open" → "[closed]"; :62's kernel and F1b sentences);
  - the comma at V1 :1140 (N9).
- The two new V1 sections are appended, and each is dated.
- `K2B_MERGE`, `K3_MERGE` and the I12 brief are this PR's own files, and their edits are bracketed inserts.

### Q5's amended sequencing and the selected order

**Consistent: it adds a dependency, and contradicts nothing.**
- The selected order has two chains. The kernel chain runs "…, then K4, then K6 and V-K"; the facade chain runs "S11-F → F1 → F2a (atomic with S-G1) → … → F3 → V-P → join" (`ROOT_SELECTION_DESIGNS.md`, Selected item 2).
- DESIGN §6's rows put K6 "after K1", V-K "after R1's references are frozen and K4", and F2a "after F1 and ROOT's identity reservation". F2a wires K4's method, so it follows K4 in any case.
- Nothing orders V-K or K6 against F2a. Making F2a's merge wait for K6's and V-K's runs is therefore a new cross-chain dependency that the order allows. Only V-P must follow F2a, and the amendment keeps V-P after F2a, as a confirmation.
- **Two wording points** are D2:
  - "both of which precede F2a" states as fact a dependency that this ruling creates;
  - the amendment departs from C4 and D-8 without naming them.
- **Q9 stays workable.** Q9 says "V-K or F2a, whichever runs first" adds the export: F2a may still be developed first, but cannot merge before V-K's runs.

### Other checks

- **The four refreshed SHA256SUMS verify:** K1_MERGE 10/10, K2A 170/170, K2B_MERGE 13/13 and K3_MERGE 14/14. Each changes exactly the one line of its edited file, and each lists exactly its folder's tracked files. Every other SHA256SUMS covering a changed path also verifies, including `records_pr1042/` 4/4 at `7df3acefe` (`delta/checks_7df3acefe.stdout.txt` §3).
- **Machine paths and model identifiers:** none in the 340 changed files at `7df3acefe`. The only pattern hits are RV10's two pattern-describing records (as before). The new banners, brackets and sections name only repository-relative paths, and so do the two edited PR descriptions.
- **Scope:** still records only; all 340 paths are under `projects/chirality-piping/execution/`.
- **GEN-8** (`delta/gen8_delta.txt`), `-p no:cacheprovider`:
  - **Hosted:** governance-harness run 36426286583 is a `pull_request` run on `7df3acefe`: success.
  - **Local:** it passes in `<wt>/numerics` at `28bb8dfa8`, with this delta's files present. That tree is the candidate plus two append-only local commits. GEN-8's lint is per-line, so a pass there covers `7df3acefe`'s files.
  - **An archive run is not possible.** I extracted a `git archive` of `7df3acefe` outside any Git work tree, and the live self-check stopped with `HarnessOperationalError`: its brief-adoption check needs `git ls-files`. So a GEN-8 run needs a Git working tree of the candidate. This also means RV10's recorded premise, "outside a Git work tree the self-check walks every file", does not hold at all (N5(a)). I deleted the extraction.

### Delta findings

| ID | Severity | Site | Evidence | Resolution |
|---|---|---|---|---|
| D1 | NOTE | `ROOT_RULINGS_V1.md:1154` | "These decisions were stated in ROOT's resume messages to I10." The committed evidence puts decisions 1 and 2 at RV11's resume for the delta check ("ROOT's two decisions stated at resume", `K2B_REVIEW.md:225`). Only item 3's rulings are recorded as a message to I10 (`RETURN.md:1129`). A2.4's "ROOT reversed its decision" implies that I10 knew of decision 2. Nothing records that I10 was told decision 1. | Optional: "stated in ROOT's messages: decisions 1 and 2 at RV11's delta-check resume, and item 3's rulings to I10". |
| D2 | NOTE | `ROOT_RULINGS_V1.md:1169-1176` | (a) "K6's measurements and V-K's kernel-lane runs, both of which precede F2a": the selected order does not order either against F2a, so this is the dependency the amendment creates, not an existing fact. (b) It departs from C4 ("selected from the K6 and V-P measurements", `ROOT_SELECTION_DESIGNS.md`) and D-8 (DESIGN :1270) without naming them. The substance is sound. | Optional: "which F2a's merge now waits for", and "amends C4 and D-8: the limits come from K6 and V-K, and V-P confirms or revises them". |
| D3 | NOTE | `IMPLEMENTATION/K2A/RETURN_ADDENDUM_1.md:71` and `K2A/SHA256SUMS` | The N5(b) pointer, which I proposed, changes a hash-bound, merged K2a record: sha256 `b696e806…` becomes `cdafd957…`. Two committed records cite the old hash and now fail a re-check: `IMPLEMENTATION/M03_SKEW_PIN/RETURN.md:24` (I9's basis) and `REVIEW/_run_records/k2a/delta_aad23e82d.txt:26` (RV7). Both stay true of the bytes read then. The text is kept verbatim, but the bracket names RV13-N5 and not the prior hash. | Optional: record the prior hash `b696e806…` (the bytes on main `e7d930d49`) beside the pointer, or in `K2A_MERGE/RECORD.md`. For the future, prefer pointers from rulings or merge records over edits to hash-bound RETURNs. |

### What I did not do in the delta

- I did not review `bacf939a8` or `28bb8dfa8` (local, not in the PR), or I13's draft beyond confirming that it exists.
- I did not re-run the original review's GitHub facts for PRs #1038, #1041 and #1040, beyond their edited descriptions.
- No Git writes. My writes are this appended section, `_run_records/records_pr1042/delta/`, and the updated `_run_records/records_pr1042/SHA256SUMS`. All are uncommitted.
