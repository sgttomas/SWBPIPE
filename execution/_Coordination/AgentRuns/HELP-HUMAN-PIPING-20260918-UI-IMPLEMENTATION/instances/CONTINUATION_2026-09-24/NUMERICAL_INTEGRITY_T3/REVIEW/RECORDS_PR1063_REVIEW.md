# RV26: independent review of records PR #1063

- **Reviewer:** RV26, a Type 2 TASK (independent reviewer).
- **PR:** https://github.com/sgttomas/chirality/pull/1063, branch `codex/piping-t3-records-20260930b`.
- **Head reviewed:** `21e2285e30d47113a4d4dceae71c60696f1832ab`. Base main `490b75bd9` (#1062's merge), which is also the merge base.
- **Date:** 2026-09-30.
- **Verdict: PASS.** There are no BLOCKING findings. There are 2 SHOULD-FIX findings and 10 NOTEs.

## Summary

- **Scope is records only.** All 13 changed paths are under `projects/chirality-piping/execution/_Coordination/`: 8 added and 5 modified, all at mode 100644, with no binary.
- **Nothing on main is rewritten.**
  - `ROOT_RULINGS_V1.md`, `RECORDS_PR1062_REVIEW.md` and `records_pr1062_review/SHA256SUMS` begin with main's bytes and only append.
  - The handoff gains two inserted lines (the "Companion" note at :5, and a blank line), and the work graph's :62 gains one inserted clause. Both are insert-only.
- **Every hash holds.** RV25's folder verifies 34/34 with `shasum -a 256 -c`, run from the folder. Main's `REVIEW/_run_records/SHA256SUMS` verifies 76/76 from `REVIEW/`, as RV25-N5 describes.
- **No machine paths or host names leak.** GEN-8 passes at the head.
- **Most of the operating notes check out against the records.** These all match their sources:
  - every named finding;
  - the KF3 merge-order slip, RV24's 8-versus-4 count, and RV23's five rebuilt mutants;
  - both "whichever merges second" cases;
  - the test-count rule, which I recounted for five slices;
  - the DEC-025, suite and `gen --check` durations, and the memory guard.
- **Two claims are contradicted by the records:**
  - **S1:** KF2's B is said to have taken "a few hours". Its records span about 28 minutes.
  - **S2:** ROOT's "one process slip" leaves out at least three others that are recorded, one of them in the handoff item the same sentence cites.
- **The NOTEs** cover these points:
  - an incomplete list;
  - small range and wording points;
  - claims resting on session experience or on uncommitted owner statements;
  - the pointers between the two operating-notes files;
  - templates that do not yet carry the notes' own lessons;
  - one wording point in the new rulings.

## Reviewer, brief and basis

- **Reviewer:** RV26, dispatched directly by ROOT (HELP_HUMAN) through the host's background-subagent mechanism. ROOT is my only return path.
  - I wrote none of these records and delegated nothing.
  - I made no Git writes and no index operations. I used only `git show`/`diff`/`log`/`rev-list`/`merge-base`/`ls-tree`/`cat-file` and `gh pr`/`run` reads.
  - No cargo was run.
- **Brief:** ROOT's dispatch message. Its five priorities are §1–§8 below. RV25's review of #1062 (`REVIEW/RECORDS_PR1062_REVIEW.md`) is the format.
- **Read:**
  - Root `AGENTS.md` and `agents/AGENT_TASK.md`;
  - RV25's review;
  - `HANDOFF_2026-09-30_AUDIT_PAUSE.md` and `OPERATING_NOTES_FOR_LOCAL_ROOT.md`;
  - all 180 lines of `OPERATING_NOTES_2026-09-30.md`;
  - `ROOT_RULINGS_V1.md` from :1106 to the end, with the sections each note cites;
  - the briefs I19–I21;
  - the reviews RV20–RV24 (findings, confirmations and merge checks);
  - the nine Mac-era merge records' `dec025/meta.txt` and `suites_vs_baseline.txt`;
  - KF2's B run records, and the K4, KF1 and KF2 RETURNs where they give suite times;
  - `OWNER_DIRECTION.md` and `I8R_K1_RESUME.md`;
  - the PR body.
- **Paths:** `T3/` is `projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/`. `ON` is `T3/OPERATING_NOTES_2026-09-30.md` at the head. `V1` is `T3/ROOT_RULINGS_V1.md`. `WG` is the work graph. `<wt>` is the T3 worktrees root.
- **Scripts and outputs:** `T3/REVIEW/_run_records/records_pr1063_review/` (README.txt, SHA256SUMS). I reused three of RV25's committed scripts by path, unchanged.

## 1. Scope (PASS)

- **The diff:** `git diff --name-status 490b75bd9 21e2285e3` has 8 A and 5 M, with 820 insertions and 1 deletion (`WG:62`, replaced by the same line plus the pointer).
  - All 13 paths are under `_Coordination/`: 12 under `T3/`, plus `WG`.
  - There is no binary, and all are at mode 100644.
  - The files are exactly the PR body's list.
- **The commits:** `c8e13bb7d` (parent `490b75bd9`) and `21e2285e3` (parent `c8e13bb7d`).
  - #1062 merged at 07:10:41Z as `490b75bd9`, with first parent `7ad3a9adf` and second parent `7b59a1efc`, the head RV25 checked last.
  - `origin/main` is still `490b75bd9`.
- **Hosted CI on the head:** 7 passed and 6 were skipped (runs 36712612121, 36712612107, 36712612145 and 36712612111, all `pull_request`), the records-only selection.
- **Hygiene:** `git diff --check` flags two trailing spaces, at `delta_7b59a1efc/delta_checks.out.txt:18-19`. They are in RV25's raw output, which is hash-bound (RV25-N9's class), and need no action.

## 2. Append-only (PASS)

I ran RV25's committed `append_only.py.txt` unchanged, with base `490b75bd9` and head `21e2285e3` (`append_only.out.txt`).

| File (main → head lines) | Result |
|---|---|
| `V1` (3,035 → 3,058) | Head starts with main's bytes. 23 appended lines: "Records PR #1062 merged; T3 paused for the audit" and "Operating notes for the pause; a second records PR". |
| `REVIEW/RECORDS_PR1062_REVIEW.md` (336 → 363) | Head starts with main's bytes. 27 appended lines: RV25's "Delta check at 7b59a1efc". |
| `records_pr1062_review/SHA256SUMS` (27 → 34) | 0 removed, 7 added, 0 changed. The added entries are the seven `delta_7b59a1efc/` files. |
| `HANDOFF_2026-09-30_AUDIT_PAUSE.md` (244 → 246) | **Insert-only:** two lines after main's :3, a blank line and the "Companion" note (head :5), under the opening note. No other change. |
| `WG` (416 → 416) | **:62 is insert-only.** The old line is a character subsequence of the new one, and the one insertion is "; with `T3/OPERATING_NOTES_2026-09-30.md`: how ROOT ran the work, and what it learned". |

## 3. Hashes (PASS)

- **`sums_coverage.out.txt`** runs RV25's `sums_coverage.py.txt` against the head's blobs.
  - `records_pr1062_review/`: 34 entries, all verified. The folder has no unlisted tracked file.
  - The "CHECK" line for `REVIEW/_run_records/` is main's unchanged sums file, whose paths are relative to `REVIEW/` (RV25-N5).
- **`shasum -a 256 -c`** (`checks.out.txt` §2): 34 OK from `records_pr1062_review/`, and 76 OK for `REVIEW/_run_records/SHA256SUMS` run from `REVIEW/`.
- **Paths no SHA256SUMS lists:** the new notes, the handoff, `V1`, `WG` and RV25's review. That matches main's precedent (RV25 §3).
- **At the head:** `RECORDS_PR1062_REVIEW.md` is sha256 `0ab9812e3e450300…`, and its folder's SHA256SUMS is `0a1e3082f59850c2…` (N10).

## 4. Machine paths, host names, GEN-8 (PASS)

- **`leak_scan.out.txt`** runs RV25's `leak_scan.py.txt` over all 13 files; for modified files it reads only the added lines. It covers:
  - GEN-8's regex;
  - the broad path forms;
  - model identifiers;
  - the host's computer name, local host name and user name. These were supplied at run time from my session scratch and are never printed.

  It finds **0 hits in every class.** A raw grep of the added lines finds one mention of `<home>/.local`, a placeholder in RV25's section.
- **The notes' host facts** (18 cores, 128 GiB) are not identifiers. The notes' only paths are placeholders (`<wt>/guard/memguard.sh`, `<wt>/rv<n>*/`).
- **GEN-8 passes at the head** (`gen8.out.txt`): 1 passed and 10 deselected, run from `<wt>/numerics`. It was run twice:
  - before any RV26 file existed, with `git status` empty;
  - again with this review and its records present.

## 5. The operating notes against the records (S1, S2; N1–N6)

### 5.1 Named findings and events: all match

| Note | Claim | Source |
|---|---|---|
| ON:108 | RV19-1, a false publication | `V1:2011-2015`; `WG:62` |
| ON:109 | RV22-1, a relaxation applied to completed builds | `K6B_REVIEW.md:46`: "The code relaxes by outcome, not by build" |
| ON:110 | RV23-1, refusals dropped on a stop | `KF3_REVIEW.md:25`: "A refusal is dropped from the evidence when a budget stop follows it" |
| ON:111 | RV24-1, three regressions surviving the committed tests | `KF2_REVIEW.md:32` (M1, M5, M4b) |
| ON:112, :137 | RV25-S1 and S2, ROOT's figures, fixed before merge | `V1:3016-3020`; RV25's delta check |
| ON:74 | RV24 counted 8 where there were 4; I20 caught it; RV24 confirmed | `KF2_REVIEW.md:35` ("eight"); `V1:2964` (I20: 4 of 16); `KF2_REVIEW.md:271`: "I20's correction is right, and my 'eight' was wrong" |
| ON:79 | RV23 rebuilt five of I19's mutants from descriptions | `KF3_REVIEW.md:19`: "five of I19's (re-implemented from `CHECKPOINT_A.md` §4, since their diffs are not recorded)" |
| ON:127 | The S1 correction understated the finding (RV25-D1) | `RECORDS_PR1062_REVIEW.md:326`; `V1:2806` |
| ON:137 | RV20-1, RV21-1/2, RV22-1/2/3, RV23-1 and RV24-1: SHOULD-FIX, fixed before merge | `V1:2425`, `:2600-2602`, `:2626-2633`, `:2899`, `:2934`; each confirmation's ruling (N1) |
| ON:140 | RV23-N1's precedence test, "cheap" and protecting a ruled decision | `V1:2904`, in those words; M4b killed (`KF3_REVIEW.md:281`) |
| ON:142 | RV23-1 fixed because F2a will publish `bound_refusals` | RV23-1 states the reason ("F2a will publish this field"). The ruling (`V1:2899-2903`) orders the fix without restating it: consistent. |
| ON:144 | The export (K6b and V-K) | `V1:2172`: "Whichever PR merges first carries the export". V-K merged first; K6b merged main, resolving `adaptive.rs` and `verify.rs` to main's side (`V1:2679`). |
| ON:144 | The parity check (K6b, tightened by KF3) | `V1:2570`: "whichever of KF3 and K6b merges second". K6b merged at 01:08:38Z and KF3 at 05:28:58Z; `V1:2720`: "KF3 merges second, so KF3 updates K6b's checks". |
| ON:145 | K6b's backstop stop, K6B-S3's parity stop, V-K's THIN | `V1:2279`, `:2469`, `:2175` |
| ON:146 | KF3-B2 is K6b's E_max omission, routed to K6c with a reason | `V1:2805-2818` |
| ON:122 | KF3 merged without a main check | `V1:2989`; RV25 §5 verified it |
| ON:36, :41 | KF2's equality argument checked at checkpoint 0; KF2's guard read cell for cell at A | `V1:2770-2776`, `:2848-2849` |
| ON:98 | A sweep started while its reviewer was confirming the same head | KF3's DEC-025 started 04:52:24Z. "KF3: RV23 confirms aa83f6796" was committed at 04:58:33Z and says "DEC-025 (running now)". |
| ON:180 | #1062 carried 1,031 files | RV25 §1 |

### 5.2 Durations and counts

`checks.out.txt` §3–§5, `ci_job_times.out.txt` and `test_counts.out.txt`.

| Note | Claim | Records | Result |
|---|---|---|---|
| ON:71 | The per-crate DEC-025 change equals the added `#[test]`s, "every time (KF3 +15 and +1; KF2 +16, 1 ignored)" | Recounted with the note's own command on macOS `/usr/bin/grep`, which handles `\s`. Against the main each head carries: KF3 frame_kernel +15 and harness +1; KF2 +16 with one `#[ignore]` (417→432, 1 ignored); K6b +20 (54→74); KF1 +8 (394→402); V-K +47 (VR new). | match |
| ON:87 | DEC-025 about 36–37 min (K6b, KF3, KF2) | 36.7, 36.0 and 36.6 min (`meta.txt` start → ALL-DONE). The others are 29–41 min. | match |
| ON:88 | FK's full debug suite at 2 threads, 11–13 min | The lib alone takes 683–710 s at 2 threads (KF2 RETURN :282, KF1 :491, K4 :946, K4_REVIEW :338) | consistent |
| ON:89 | `gen_k4_vectors.py --check` about 15 min | 14 min 43 s, and 16 min 56 s with FK's suite beside it (K4 RETURN :949, :1009) | match |
| ON:90 | Hosted CI's numerical cargo job 18–24 min | "Numerical cargo suite" on every E2E run of the nine slice PRs: 16.9–24.0 min over the 38 runs from K4 on, with one outlier of 10.7 min (KF1 `66adfede4`, pull_request); 6.9–14.1 min before K4 | about right (N2) |
| ON:91 | A slice review about 50–80 min | From the "to review" ruling commit to the "rulings on the review" commit, which includes ROOT's own ruling time: RV19 59, RV20 45, RV21 51, RV22 45, RV23 80, RV24 52 min | range starts nearer 40 (N2) |
| ON:92 | A records review about 40 min | #1062 opened 06:12:44Z; the ruling on RV25 was committed 06:53:57Z | match |
| ON:93 | A confirmation or delta check 5–25 min | RV23's confirmation 23 min; RV25's two delta checks about 9 and 6 min (fix commit to last output file) | match |
| **ON:95** | **KF2's B "took a few hours of mostly machine time"** | **About 28 min.** The grant ruling was committed 02:44:12Z. B's records run from 02:48:38Z (T9 build) to 03:16:24Z (src-tauri): T9 and probe builds of 29–34 s each; part 1 in 384 s (base) and 382 s (candidate); part 2 from 03:06:53Z to 03:14:24Z, including a 180 s wait for load; then src-tauri. The acceptance was committed 03:18:15Z. `<wt>/scratch/i20/b/` was created and last modified in the same window. | **contradicted (S1)** |
| ON:101 | Memory guard: SIGKILL below 35%; no kill this week; KF3's 10,000-member peaks about 3 GB | `memguard.sh` uses `FLOOR=35` on `kern.memorystatus_level` and kills processes naming `<wt>` or the gate probes. `memguard.log` has 2 start lines and no KILLED line, as KF2's B INDEX also records. TREE peaks were 2,889.9/2,891.5 MiB (`V1:2757`), about 3.0 GB. | match |
| ON:81 | The host: 18 cores, 128 GiB, no swap | `hw.ncpu` 18 and `hw.memsize` 128 GiB match. `vm.swapusage` reports a 1 GiB dynamic swap file with about 0.2 GiB in use. | N3 |
| ON:102 | 0.3–0.8 M tokens per subagent run; 4–5 implementer runs and 2–3 reviewer runs per slice | Token counts are not recorded. The run counts are consistent with the checkpoint and confirmation trail, for example KF2: 0, A, B, D and the fix; RV24: review, confirmation and merge check. | session (N5) |
| ON:122 | "at least seven figure or claim errors in rulings (handoff §7.2 item 1), and one process slip" | Seven matches §7.2 item 1. "One process slip" does not (S2). | **contradicted (S2)** |

### 5.3 Claims that rest on session experience (N5, N6)

These are not contradicted by any record. They cannot be verified from committed records:
- ON:12: "up to four subagents at once";
- ON:13: the compactions;
- ON:34: the spawn prompt's wording;
- ON:36: KF3's A2 argument "checked line by line". The ruling states the argument (`V1:2556`) but records no check;
- ON:51: the PR bar;
- ON:77: the completion-notice wording;
- ON:78: I20's B grant resent. B's first record is 4 min after the grant ruling commit, and there is no record of a resend;
- ON:102: token usage;
- ON:155: the `rm` safety check;
- ON:156: `.output` files.

The owner statements (ON:16, :166, :167) are N6.

### 5.4 Technical checks of §8

- **The quirks that are right:**
  - zsh's unquoted `--include=*.rs` fails with "no matches found";
  - macOS `/bin/cat -A` is illegal, while `-e` works;
  - BSD `sed -i 's/x/y/'` without `''` fails;
  - `gh run list --commit` exists (gh 2.96.0).
- **`echo ====` does fail,** but through zsh's `=command` expansion ("zsh:1: === not found"), not as a glob (N4).

## 6. Consistency with the handoff, the 2026-09-28 notes and the rulings (N7)

- **The handoff:**
  - ON agrees with handoff §5 on host concurrency, DEC-025 (35–40 min), GEN-8 in a clean working tree, the merge gates, reviewer merge checks and TASKs making no Git writes.
  - It agrees with §7.1 items 1–4 and 6, and with §7.2 items 1, 6 and 7.
  - It disagrees with §7.2 item 2 (S2).
- **The rulings:** no note contradicts a ruling. ON:3 and `V1:3058` both say the rulings govern.
  - ON:137's practice matches `V1:2425`: "Fix before merge, as with earlier slices' SHOULD-FIX findings".
  - ON:132's bracket rule matches the RV25-D2 ruling (`V1:3018`).
- **The 2026-09-28 notes:**
  - ON's return-path, stop-rule, never-rewrite and records-PR practice restates them.
  - Two points diverge without saying so (N7):
    - their §1 manager and nested-delegation shape and §3's manager-held cargo token were not the Mac practice;
    - their §1 lets an earlier DEC-025 sweep stand after a main merge that touches no piping, `tools/` or `.github/` path, while ON:59 runs DEC-025 "on that exact head", which is what every Mac-era record did (RV25 §5).
  - Handoff :148's "(see also OPERATING_NOTES §2)" is now ambiguous.

## 7. Usefulness (N8, N9)

- **The templates do not yet carry the notes' own lessons.**
  - ON:22 names I21 as a template, and ON:79 and :178 ask every brief to name the file that holds mutant diffs. I21 does not ask for mutant diffs.
  - Reviewer prompts have not been committed since RV13, so ON §5 has no committed example (N8).
- **Reviewer folders:** ON:113 says they are deleted afterwards. That has been the practice since RV20, but older ones remain, and handoff §9's inventory misses several folders (N9).

## 8. The new rulings (N10)

- **"Records PR #1062 merged" (`V1:3037-3051`) matches GitHub and RV25:**
  - the head `7b59a1efc`, the merge `490b75bd9` at 07:10:41Z, and first parent `7ad3a9adf`;
  - the three verdicts;
  - 7 passed and 6 skipped;
  - GEN-8 at each head (RV25 §4 and both delta sections);
  - "as for RV16": `RECORDS_PR1049_REVIEW.md` was appended to on numerics after #1049 merged (RV25 §2).
- **"Operating notes for the pause" (`V1:3053-3058`)** matches the PR's content: the pointers are insert-only, and it states that the rulings govern.
- **One wording point, and one hash not cited:** N10.

## Findings

| ID | Severity | Site | Evidence | Resolution |
|---|---|---|---|---|
| S1 | SHOULD-FIX | `ON:95` | **KF2's B is said to have "took a few hours of mostly machine time". Its records show about 28 minutes.**<br>• The grant ruling was committed 02:44:12Z ("KF2: checkpoint A accepted; B granted").<br>• `KF2/_run_records/b/` runs from 02:48:38Z (T9 build) to 03:16:24Z (src-tauri log):<br>  – builds of 29–34 s each, on warm targets;<br>  – part 1 "done 884 runs in 384 s" (base) and "382 s" (candidate), with 4 workers;<br>  – part 2 from 03:06:53Z to 03:14:24Z, including a 180 s load wait.<br>• The acceptance was committed 03:18:15Z.<br>• `<wt>/scratch/i20/b/` has the same window.<br>• This is the class ON §6 warns against: a figure written from memory. It sits in a "plan around these" section, and it would overstate a gate's cost six-fold. | Replace it with the recorded figure, for example: "KF2's B (T9, gate part 1 at 884 × 2 with 4 workers, part 2 and src-tauri) took about 30 minutes on warm targets (KF2 B's records, 02:48–03:16Z)." |
| S2 | SHOULD-FIX | `ON:122`; also `ON:127` | **"…and one process slip: merging without checking a main move." The records show at least three more this week, one of them in the handoff item the sentence cites:**<br>• **Rulings given only in messages** and recorded late, after a records reviewer found the gap: `V1:1152` ("recorded late, RV13-S2"), `V1:1539` ("recorded late, RV16-S3"), and `V1:1894` ("the C ruling was given in session"). This is the lesson ON:16 states.<br>• **ROOT asked implementers to prepare merges,** an index operation. It was withdrawn before any merge ran (`V1:2457`; handoff §7.2 item 2).<br>• **The work graph lagged the rulings** (handoff §7.2 item 7; ON §10 item 3).<br>• Separately, ON:127's "The corrections themselves overshot once" omits RV25-D2: the S1 ruling called two rewordings brackets, as did `18b625268`'s message (`V1:3018`).<br>The notes exist so an auditor can find issues early. Their own account of ROOT's errors should not undercount what the records already show. | Reword ON:122, for example: "…and at least four process slips: rulings left in messages until a records review found them (RV13-S2, RV16-S3, K6's C ruling); asking TASKs to prepare merges (an index operation, withdrawn before any ran; handoff §7.2 item 2); merging KF3 without checking a main move (item 6); and a lagging work graph (item 7)."<br>At ON:127: "overshot twice (RV25-D1, D2)". |
| N1 | NOTE | `ON:137` | **The SHOULD-FIX list reads as exhaustive but covers only RV20 onward.** These were also fixed before merge this week:<br>• RV11-2/3/4 (`V1:1115-1117`);<br>• RV13's three (`WG:62`);<br>• RV14-1 to 4 and RV14-D1 (`V1:1544`, `:1489`);<br>• RV16's five (`V1:1585`);<br>• RV17-1 to 4 (`V1:1708`);<br>• RV18-1 to 4 (`V1:1910`);<br>• RV19-2 to 5 and RV19-D4 (`V1:2025-2033`, `:2059`).<br>So the practice claim holds more strongly than the list shows. | "Every one this week was, for example: …", or the full list. |
| N2 | NOTE | `ON:90-91` | **Two planning ranges are slightly narrower than the records show.**<br>• The numerical cargo job ran 16.9–24.0 min over 38 runs from K4 on, with one 10.7 min outlier (`ci_job_times.out.txt`).<br>• Slice-review windows ran 45–80 min, and those windows include ROOT's ruling time (RV20 45, RV22 45).<br>The other rows match. | Optional: "17–24 min"; "about 40–80 min". |
| N3 | NOTE | `ON:81` (also handoff :108 and `I8R_K1_RESUME.md:26`, main's text) | **"No swap" is not literally true now.** `vm.swapusage` reports a 1 GiB dynamic swap file dated 2026-09-28, about 0.2 GiB in use. macOS creates swap on demand. The practical point stands: swap cannot absorb a runaway at this scale, and the guard is still needed. | Optional: "effectively no swap (macOS dynamic swap, about 1 GiB)". |
| N4 | NOTE | `ON:46`, `:72`, `:107-112`, `:143`, `:152`, `:161` | **Cross-references and wording:**<br>(a) :46's "(§4)" should be §3; the start-evidence advice is at :78.<br>(b) :72 cites KF2's `522167ac6` for merge resolutions. It was a clean merge: its remerge diff is empty (RV25 §5), and `V1:2991` says "clean; `structural.rs` is main's plus exactly KF2's delta". The check applies, but to a two-sided file, not a resolved one.<br>(c) :112 lists RV25-S1/S2, ROOT's figures, under "issues that the implementer's tests missed".<br>(d) :143 says the dense screen "became its own slice with an owner-facing note". It is a routed candidate, not scheduled, and its owner-facing note is still to be written (`V1:2783`; handoff §2 item 4).<br>(e) :152: `echo ====` fails through zsh's `=command` expansion, not globbing.<br>(f) :161: `--json jobs` gives start and end times, from which durations are computed. | Optional wording fixes. |
| N5 | NOTE | `ON:12`, `:13`, `:34`, `:36`, `:51`, `:77-78`, `:102`, `:155-156` | **These claims rest on session experience,** and no committed record shows them (§5.3). None is contradicted.<br>I20's resend (:78) is the one an auditor may look for. The grant ruling was committed at 02:44:12Z, and B's first record is at 02:48:38Z. | Optional: tag these "(session observation)", or record the resend in a KF2 ruling bracket. |
| N6 | NOTE | `ON:16`, `:166`, `:167` | **Owner statements are quoted or relied on without a committed record:**<br>• "(the owner's instruction)" on committing rulings where agents can read them;<br>• "not in a rush" and the preference for the cleanest path;<br>• hearing intentions before acting at closure points.<br>`OWNER_DIRECTION.md` records owner decisions verbatim, separate from ROOT's application; none of these is there. The related records are RV13-S2 and RV16-S3. | Record the owner's words, with the date, in `OWNER_DIRECTION.md`, or mark them as ROOT's account. |
| N7 | NOTE | `ON:5`; handoff :148 | **The pointers between the two operating-notes files:**<br>(a) Handoff :148 says "(see also OPERATING_NOTES §2)". With two files this is ambiguous. It means the 2026-09-28 file's §2 (messaging); the new file's §2 is the slice rhythm, and its §3 covers the return path.<br>(b) ON:5 says the 2026-09-28 notes "stay valid", without the handoff's "except where §7 corrects them". Two of their points were not the Mac practice:<br>  – §1's T3 manager and nested delegation, and §3's manager-held cargo token;<br>  – §1's rule that an earlier DEC-025 sweep can stand after a main merge that touches no piping path, where ON:59 runs DEC-025 on the exact head. | Insert-only bracket at handoff :148: "[the 2026-09-28 notes; see also `OPERATING_NOTES_2026-09-30.md` §3]".<br>At ON:5: "stays valid except where this file or the handoff's §7 differs (no T3 manager on the Mac; DEC-025 on the exact head)". |
| N8 | NOTE | `TASK_BRIEFS/I21_K6C_IMPLEMENTATION.md:65-73`; `ON:15`, `:22`, `:79`, `:106`, `:178` | **The templates the notes point to do not yet carry the notes' lessons.**<br>• I21 is named a template (:22) and is K6c's brief. It does not ask for mutant diffs (`checks.out.txt` §6: 0 matches), though :79 and :178 ask every brief to.<br>• Reviewer prompts after RV13 exist only as dispatch messages. `TASK_BRIEFS/` holds RV briefs only up to RV13, so §5's prompt advice has no committed example.<br>• :15's "take the next free numbers from `TASK_BRIEFS/` and `REVIEW/`" does not work for reviewers: RV numbers are not in either folder's file names after RV13. The next free numbers are I22 and RV27. | Before K6c spawns, add to I21's Required tests: "record each mutant's patch at `_run_records/**/mutants/<id>.diff`".<br>Commit one reviewer prompt as a template, for example RV24's or this one.<br>At :15, name I22 and RV27. |
| N9 | NOTE | `ON:113`; handoff §9 (main's text) | **"…and delete them afterwards" has been the practice since RV20, but earlier reviewers' folders remain,** and handoff §9's prune inventory is incomplete. `<wt>` still holds:<br>• the reviewer folders `k4-rv19`, `k4-rv19d`, `rv7-target`, `rv11-target` and `scratch/rv{7,9,10,11,12,14,17}`;<br>• targets that §9 does not list: `gate-base-target`, `gate-base-full-target`, `gate-cand-full-target`, `gate-cand2-full-target`, `k2b-target`, `k4-target`, `k5-target`, `k5-gate-target`, `k6-target`, `kf1-target`, `vk-target`, `skewpin-target` and `root-verify-target`;<br>• the mutant folders `{f1b,k1,k2b,k4,k6b,skewpin,vk}-mut`.<br>§9 lists only `rv7-target`, `rv11-target` and `scratch/rv11` of the reviewer folders. | ON:113: "(earlier reviewers' folders remain; see handoff §9)".<br>At the next records PR, a complete inventory as an insert-only addition to §9. |
| N10 | NOTE | `V1:3042`; `V1:3037-3051` | **(a) D3 was not fixed in `7b59a1efc`.** "NOTEs D1–D4, fixed in `7b59a1efc`" is off for D3, which was fixed in PR #1062's body. `7b59a1efc`'s subject names "D1, D2, D4", and RV25's check (`RECORDS_PR1062_REVIEW.md:350`) says so.<br>**(b) The final review's sha256 is not cited.** Unlike earlier rulings (`V1:3007`), this section does not cite the sha256 of the review as finally committed. At the head it is `0ab9812e3e450300…`, and the folder's SHA256SUMS is `0a1e3082f59850c2…`. | Optional bracket: "[D1, D2 and D4 in `7b59a1efc`; D3 in the PR body. RV25's review as merged-plus-delta: sha256 `0ab9812e…`.]" |

## What I did not do

- **No builds, suites, gates or mutants.** I ran GEN-8 twice, at the head only.
- **Not re-checked:** RV25's findings on #1062's earlier content, the merge records, and DEC-025's suite chains, which this PR does not touch. I re-derived only what the notes restate.
- **Not verifiable from committed records:** the claims in §5.3 (N5) and the owner statements (N6). For those I checked that no record contradicts them.
- **Host state read, read-only:**
  - `<wt>/guard/memguard.sh` and `memguard.log`;
  - `<wt>/scratch/i20/` folder times;
  - the `<wt>` top-level listing;
  - `sysctl` `hw.ncpu`, `hw.memsize` and `vm.swapusage`;
  - `zsh`, `/bin/cat`, `/usr/bin/grep` and BSD `sed` behaviour, tested on scratch files in my session scratch only.
- **Writes:** this file and `T3/REVIEW/_run_records/records_pr1063_review/`, all uncommitted:
  - my scripts, `checks.sh.txt`, `ci_job_times.sh.txt` and `test_counts.sh.txt`;
  - their outputs;
  - the outputs of RV25's three scripts;
  - `gen8.out.txt`, `README.txt` and `SHA256SUMS`.

  The host-name patterns for the leak scan lived only in my session scratch.

## Delta check at 95bb2e700

**Delta verdict: PASS, with one new SHOULD-FIX (D1) to fix before merge, and 3 NOTEs.** S1 and S2 are fixed, and both now agree with the records. N1–N10 are handled as ROOT's ruling says. Main's text is only inserted into, and every SHA256SUMS verifies. GEN-8 passes. The review's verdict stands: PASS.

D1 is my own error carried forward. My N7 said DEC-025 on the exact head "is what every Mac-era record did (RV25 §5)", but RV25 §5 covered only the nine merge records from K5 on. ROOT's fix turned that into a stronger claim, and two records of 2026-09-28 contradict it. My N7 wording (§6 and N7 above) is corrected here, not rewritten.

- **Mechanism:** ROOT resumed me for this check. I am still a background subagent of ROOT's session, with no delegation, no Git writes and no index operations.
- **The head:** `95bb2e7002a6a519199172615c1e0a00821a3287`, PR #1063's `headRefOid`. It has one parent, `21e2285e3`, the head I reviewed. `origin/main` is still `490b75bd9`.
- **The commit changes 17 paths, all under `_Coordination/`:**
  - 13 added: my review and its 12 run-record files;
  - 4 modified: the notes, the handoff, `V1` and I21.
- **My files are byte-identical to what I returned:** the review is `60813369e84f3a2b…`, and SHA256SUMS is `5ab47a89346287f1…`, 11/11 OK when run from the folder. All are at mode 100644.
- **Records:** `_run_records/records_pr1063_review/delta_95bb2e700/`. That covers `delta_checks.sh.txt` and its output, RV25's three scripts re-run (append-only on the delta and against main, SUMS coverage, leak scan), and `gen8_delta.out.txt`.

### Findings: resolution

| ID | Status at `95bb2e700` | Evidence |
|---|---|---|
| S1 | **resolved** | ON:97: "its recorded runs span about 28 minutes (02:48–03:16Z). Gate part 1 took about 6.4 min per side, and part 2 about 7.5 min". The bracket flags the first draft's "a few hours" as a ROOT figure error (RV26-S1).<br>Against the records: 384 s and 382 s are 6.4 min; part 2 ran 03:06:53Z–03:14:24Z, which is 7.5 min including the 180 s load wait. |
| S2 | **resolved** | ON:124-128 lists the four process slips, each with a site that exists:<br>• `V1` :1152, :1539 and :1894 (rulings left in messages);<br>• :2457 and handoff §7.2 item 2 (the merge request);<br>• §7.2 item 7 (the work graph);<br>• §7.2 item 6 (the main move).<br>ON:122 adds RV26-S1 to the error count. ON:133 now reads "went wrong twice", with RV25-D2. |
| N1 | resolved, with D2 | ON:143 adds RV26-S1/S2 and names RV11, RV13, RV14, RV16, RV17, RV18 and RV19. |
| N2 | resolved | ON:92: 17–24 min. ON:93: about 40–80 min. |
| N3 | resolved, with D3 | ON:83: "no swap partition; macOS showed about 1 GiB of dynamic swap in use". |
| N4 | resolved | (a) ON:48 now reads §3.<br>(b) ON:74: "a clean auto-merge checked the same way".<br>(c) ON:109: "the last is ROOT's own".<br>(d) ON:149: "routed to a separate slice, not yet scheduled".<br>(e) ON:158: zsh's `=` expansion. This session's own `echo =====` in zsh failed with "==== not found".<br>(f) ON:167: start and end times. |
| N5, N6 | resolved as ruled | ON:7 adds a "Sources" note. ON:18 and ON:172-173 mark the owner's words as said in this session and not otherwise recorded. The ruling gives its reason for not adding them to `OWNER_DIRECTION.md` (`V1:3069`). Both remedies I offered are acceptable. See D4 for the Sources list. |
| N7 | resolved, but see **D1** | Handoff :148 now reads "OPERATING_NOTES_FOR_LOCAL_ROOT §2", an insert-only edit. ON:5 now states the two divergences, and one of them is wrong (D1). |
| N8 | I21 part resolved | I21:67 is an inserted Required-tests line: "Record every mutant's patch as `_run_records/**/mutants/<id>.diff`". The other two parts are not mentioned (D4). |
| N9 | handoff part resolved | Handoff :236 is an insert-only line saying the inventory is partial, and naming the missing targets, `k4-rv19`, `k4-rv19d` and the `*-mut` folders. See D4 for what it still omits. |
| N10 | resolved | `V1:3042` gains an insert-only bracket: D3 was fixed in the PR body, and the final review is `0ab9812e…`. |

### Main's text is only inserted into

`append_only_vs_main.out.txt` checks the head against main `490b75bd9`:
- `V1` keeps main's 3,035 lines and appends 40. The :3042 bracket is in text this PR added.
- `RECORDS_PR1062_REVIEW.md` and RV25's SHA256SUMS still only append.
- The handoff's three changes are insert-only: :4-5, the `_FOR_LOCAL_ROOT` insertion at :148, and the new line at :236.
- `WG:62` is insert-only.
- I21 gains one inserted line (:67).

`append_only_delta.out.txt` shows the rewordings are confined to `OPERATING_NOTES_2026-09-30.md`, which is new in this PR.

### Hashes, leaks, GEN-8 and CI

- **SHA256SUMS** (`sums_coverage_95bb2e700.out.txt` and `delta_checks.out.txt` §2):
  - mine, 11/11;
  - RV25's, 34/34;
  - main's `REVIEW/_run_records/`, 76/76 from `REVIEW/`.

  The 7 changed paths that no sums file lists are the notes, the handoff, `V1`, `WG`, I21 and the two reviews, as on main.
- **Leak scan over the 17 changed paths:** 0 hits in every class, including the run-time host patterns.
- **GEN-8 passes at `95bb2e700`** (`gen8_delta.out.txt`): 1 passed and 10 deselected. It was run twice:
  - on a clean tree, with `git status` empty;
  - again with this section and `delta_95bb2e700/` present.
- **Hosted CI on `95bb2e700`:** 7 passed and 6 were skipped, the records-only selection. The runs are 36715479547, 36715479472, 36715479324 and 36715479441, all `pull_request` on the head.

### Delta findings

| ID | Severity | Site | Evidence | Resolution |
|---|---|---|---|---|
| D1 | SHOULD-FIX | `ON:5` ("And this week DEC-025 was always run on each slice's exact final head, never carried over from an earlier head"); my own N7 (§6 and the N7 row above: "which is what every Mac-era record did (RV25 §5)") | **Two merges this week carried DEC-025 over from an earlier head.**<br>• **M03 skew pin (#1038, merged 2026-09-28):** swept on `1d105d633`; the final head is `5dd6dfdd8`. The record says "The evidence stands for `5dd6dfdd8`, since that head changes only records" (`M03_SKEW_PIN_MERGE/RECORD.md:31`, `:40`).<br>• **K3 (#1041, merged 2026-09-28):** swept on `b7e93650e`; the final head is `2511f5a3c`. Two commits followed the sweep head, `e62837f7e` ("tests only", FK) and `2511f5a3c` (records) (`K3_MERGE/RECORD.md:37`).<br>• **Every other record from K1 to KF2 swept its exact final head** (`delta_checks.out.txt` §4).<br>**Where the error came from:** my N7 generalized RV25 §5, which covered the nine records from K5 on, to "every Mac-era record". The claim matters because it describes when the 2026-09-28 carry-over rule was used: it was used twice. | Reword ON:5, for example: "From K5 (2026-09-29) on, DEC-025 was run on each slice's exact final head. On 2026-09-28, M03's and K3's sweeps were carried to a later head (records only; tests and records), as their merge records disclose."<br>My N7 wording is corrected by this row. |
| D2 | NOTE | `ON:143` | "This week that covered RV20-1 … Earlier, RV11, RV13, RV14, RV16, RV17, RV18 and RV19's …". RV11 to RV19 were also this week (2026-09-28 and 29), so "Earlier" reads as before this week. | Optional: "This week: RV11, RV13, RV14, RV16, RV17, RV18 and RV19's findings, then RV20-1, …". |
| D3 | NOTE | `ON:83` | "about 1 GiB of dynamic swap in use". `vm.swapusage` gives a total of 1024 M, of which 228.75 M is used, so about 1 GiB is allocated and about 0.2 GiB is in use. | Optional: "about 1 GiB of dynamic swap allocated, about 0.2 GiB in use". |
| D4 | NOTE | `ON:7`; `ON:15`; `ON:115`; handoff :236 | **Parts of the NOTEs are left, and the ruling does not mention them. All are optional.**<br>• **ON:7's Sources list reads as complete.** Other claims also rest on session experience: the compactions (:15), "line by line" (:38), the PR bar, the completion-notice wording, the `rm` check and `.output` files.<br>• **N8's other two parts:** no reviewer-prompt template is committed, and ON:15 still says to take the next numbers from `TASK_BRIEFS/` and `REVIEW/` (next: I22, RV27).<br>• **N9's note at ON:115** (formerly :113) is unchanged.<br>• **Handoff :236** still omits `k2b-target` and the scratch reviewer folders (`scratch/rv{7,9,10,12,14,17}`). `du -sh <wt>/*/` does not list scratch's subfolders. | Optional: "for example" in ON:7, and a line in the next rulings section recording what was left. |

### What I did not do in the delta

- **Not re-checked:** the first review's checks outside the delta. The merge records, the durations and the named findings are unchanged by `95bb2e700`.
- **Runs:** GEN-8 twice at the head; read-only `git`, `gh`, `shasum` and `sysctl`.
- **Writes:** this appended section and `delta_95bb2e700/` (7 files). SHA256SUMS is extended by appending their entries; its 11 existing lines are unchanged. All are uncommitted.
