# RV16: independent review of records PR #1049

- **Reviewer:** RV16, a Type 2 TASK (independent reviewer).
- **PR:** https://github.com/sgttomas/chirality/pull/1049, branch `codex/piping-t3-records-20260928`.
- **Head reviewed:** `04553ad05e37d457f4521f473b9a1d147ed590d5`.
- **Date:** 2026-09-28.
- **Verdict: PASS.** There are no BLOCKING findings. There are 5 SHOULD-FIX findings and 11 NOTEs.

## Summary

- **Every hash holds.** All eight SHA256SUMS files that cover a changed path verify at the head. Each one lists exactly its folder's tracked files; `REV_5A3_CANDIDATE/` delegates `_v4_records/**` to V4's own nested sums.
- **The rulings file is intact.** `ROOT_RULINGS_V1.md` keeps the base's lines 1–1176 verbatim, apart from two insertions:
  - the RV13-D1 bracket at :1154, an in-line insert;
  - the RV13-D2 line at :1176.

  It appends 14 sections (:1179–:1537), each headed "(ROOT, 2026-09-28)". There is no silent rewrite.
- **ROOT's rulings match their evidence, with one exception.** They reproduce the verdicts and counts of V4 (three checks), RV14 (review and delta), RV15 and G1. The K6 spawn base is wrong (S1). The 5a.3 rulings describe nothing as VERIFIED or selected.
- **The SHOULD-FIX findings concern completeness and currency, not false verdicts:**
  - S1: the K6 spawn base in the rulings is not the branch's base;
  - S2: F1b's first gate part 1 is recorded as "`gate_check` PASS", but it failed the brief's gate condition;
  - S3: ROOT's rulings on RV14's first review are not in `ROOT_RULINGS_V1.md`;
  - S4: six superseded or amended rulings carry no pointer at the text they supersede;
  - S5: the 5a.3 R2 and R3 evidence named in the designs is only partly committed.
- **K6's host and memory rules are safe on the 128 GB Mac with no swap:**
  - no dense run at 10,000 members, enforced four ways;
  - an in-process heap cap under every run;
  - the 16 GiB ceiling run last and alone.

  N4 and N5 are refinements.
- **Merge readiness (N10).** Hosted CI refuses the head because it does not integrate its base `56dd72334`. Main must be merged in, and that merge needs a delta check.

## Reviewer, brief and basis

- **Reviewer:** RV16, dispatched directly by ROOT (the SWBPIPE HELP_HUMAN session) as a background subagent of ROOT's session on the owner's Mac. ROOT is my only return path.
  - I wrote none of these records and delegated nothing.
  - I made no Git or GitHub writes: no fetch, commit, checkout or push.
  - I ran no cargo. I ran at most one process at a time, with Python under `nice -n 19`.
- **Brief:** ROOT's dispatch message, which applies `TASK_BRIEFS/RV13_RECORDS_PR1042_REVIEW.md`'s method and RV13's review (`REVIEW/RECORDS_PR1042_REVIEW.md`) as the standard, to the delta `24dea2dae..04553ad05`.
- **Paths.** `T3/` is `projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/`. `RELAY/` is the sibling `APP_V4_RELAY_2026-09-28/`. `V1` is `T3/ROOT_RULINGS_V1.md`. `<wt>` is the T3 worktrees root.
- **Candidate:**
  - The head equals PR #1049's `headRefOid`. Base main is `56dd72334`; the merge base is `24dea2dae`.
  - The delta has 35 commits. It changes 374 files: 368 added and 6 modified (the brief said 375). All are under `projects/chirality-piping/execution/`: 373 under the continuation folder, plus the work graph.
  - Everything was read by commit (`git show`, `git archive` into my scratch), never from the working tree. The extraction has been deleted.
- **Read:**
  - Root `AGENTS.md`, and the Mac host rules (`TASK_BRIEFS/I8R_K1_RESUME.md:24-40`);
  - `DESIGN_NUMERICS/DESIGN.md` r5a.2 (sha256 `fb62ef4a…`, unchanged): §2.1, §4.1.7, §4.1.9, §4.8, §6's rows and order, and D-8;
  - `ROOT_SELECTION_DESIGNS.md` (Selected item 2, C4);
  - the three new briefs (I13, I14, I15), and I12's Q6;
  - every new or changed Markdown record in the delta;
  - the gate-baseline tables and JSON;
  - the 5a.3 evidence tables, against the committed files.

## 1. Scope and hygiene (PASS; N9)

- **Scope:** records only. There is no product code, test, fixture, schema or tool change. Scripts are committed as records (`.py.txt`).
- **Machine paths:** none in any of the 374 files at the head. That covers GEN-8's `MACHINE_ABS_PATH_RE` (`tools/practitioner_harness/surface_roles.py:22-26`), plus a broader scan for home, temp, tilde and user-name forms.
- **Model identifiers:** none. The records name the host as "a Claude Code background subagent", as RV14-N4 already accepted.
- **GEN-8:** the hosted `harness` job (governance-harness run 36498076004, `pull_request`) succeeded on `04553ad05`.
  - I did not run GEN-8 locally. The host rule allows one process at a time, and `<wt>/numerics` may hold untracked files, so a local run would not test the candidate exactly.
- **Whitespace** (`git diff --check`): every hit is in a raw run record. Only one hit is disclosed (N9).
- **SHA256SUMS at the head** (entries verified / tracked files):

  | Folder | Result |
  |---|---|
  | `RELAY/` | 4/4 |
  | `T3/DESIGN_NUMERICS/REV_5A3_CANDIDATE/` | 73/73 (every file outside `_v4_records/`) |
  | `…/REV_5A3_CANDIDATE/_v4_records/` | 83/83 (including `delta_r2/` and `r3/`) |
  | `…/_v4_records/r3/` | 16/16 |
  | `T3/GATE_BASELINE_MAC_E7D930D49/` | 29/29 (including `full_envelope/`) |
  | `T3/IMPLEMENTATION/K2A_MERGE/` | 4/4 |
  | `T3/REVIEW/_run_records/k5_review/` | 163/163 |
  | `T3/REVIEW/_run_records/records_pr1042/` | 9/9 |

  - At `e5f4ef3a8`, the candidate SUMS still held `V4_VERIFICATION.md`'s pre-delta hash. `c85dc1151` repaired it and says so in its message.
  - Main's `REVIEW/_run_records/SHA256SUMS` is unchanged.

## 2. `ROOT_RULINGS_V1.md`: each new section against its evidence

### Integrity

- The two in-place changes against the base are insert-only (difflib opcodes):
  - :1154, the "[Correction (RV13-D1): …]" bracket;
  - :1176, the new "[Added (RV13-D2): …]" line.

  Both resolve RV13's delta NOTEs as RV13 proposed. D3 is resolved in `IMPLEMENTATION/K2A_MERGE/RECORD.md`'s dated addendum. Its hashes hold: `RETURN_ADDENDUM_1.md` is `b696e806…` at `aad23e82d`, `f12e06876` and `e7d930d49`, and `cdafd957…` at `24dea2dae` and at the head.
- **The appended sections match their commits one to one.** Every section is dated. Commit order equals section order.

### K4: checkpoint 0 (:1179) and A1 (:1351)

- **The plan exists as cited:** `<wt>/scratch/i12/CHECKPOINT0_PLAN.md`, sha256 `e1253faa…`, 591 lines.
- **O1 says it amends Q6, and why.** I12's brief keeps the old text, with no pointer (S4).
- **A1:**
  - `cef218a10` is "K4 checkpoint A1", and `<wt>/scratch/i12/a1_probes/` exists.
  - F-2's arithmetic holds: 2^-300 / 512 = 2^-309.
  - The refuted claim is DESIGN §4.1.9's first bullet, quoted correctly.
  - The ruling creates one gate, "K4 does not reach checkpoint B until the addendum is selected and implemented". It is within ROOT's authority, and later sections keep it.
- **One gap:** O12, approved at :1202, is reversed at :1432 with no pointer at :1202 (S4).

### F1b: spawn (:1223), checkpoint 0 (:1263), the A2 stop (:1338) and A2 (:1395)

- **Spawn:** F1b's first commit, `94e543a24`, has parent `e7d930d49`. The spawn summary matches I13's Q1–Q14 item for item.
- **Citations:** the stale-design line numbers I spot-checked on `e7d930d49` are exact: `PP:1826`, `:1957`, `:2276`, `:2726`, `:3119`, `:4392` and `:4490`.
- **Checkpoint 0:**
  - The plan hash `ba90de63…` and its 856 lines hold.
  - "8,192 DOFs, that is 1,364 chain members" holds: 96·8192² = 6 GiB exactly. `PP:2878-2882` on `130445db2` carries the same constants.
- **The A2 stop:** the test exists at `PP:22217` on `130445db2` and `:21267` on `e7d930d49`, with its 1e308 load.
- **A2:**
  - C1 = 14 + 6 + 8 = 28, and its parts add up.
  - The full-envelope variant is recorded as the method for every later gate.
  - The RV11D-N2 figures and "16 selections, none after a Range" rest on F1b's uncommitted A2 evidence. I did not verify them.
  - :1409 restates the PASS condition without the brief's C2 clause (S2).

### K5: spawn (:1300), checkpoint 0 (:1373) and RV14's delta (:1486)

- **Spawn:** K5's first commit, `0c732061b`, has parent `24dea2dae`. Items 1–17 restate the brief's list faithfully.
- **Checkpoint 0:**
  - The plan hash `7f50c788…` holds. It has 942 lines, not 943 (N1).
  - S1 reverses Q7 to (a) and says why. I14's brief and V1's :1310 carry no pointer (S4).
- **RV14's delta (:1486):** it matches `REVIEW/K5_REVIEW.md`'s delta section:
  - the PASS verdict and 0 BLOCKING;
  - the four SHOULD-FIX findings, and N1, N2 and N5, resolved;
  - M1–M4 each killed by a new test;
  - RV14-D1's construction, taken verbatim as a `P` case;
  - RV14-D2 as a conservative limitation, the resolution RV14 offered.

  The review file was append-only from `3a17799e4` (`f468696e…`, the hash RV14's delta cites) to `f3e50948b`, and its 114 original SUMS entries are unchanged.
- **The one gap:** ROOT's rulings on RV14's *first* review are nowhere in V1 (S3).

### D1 5a.3: V4's first check (:1420), R2 (:1466) and R3 (:1497)

- **First check:**
  - The sha256 `0222d0ec…` is `V4_VERIFICATION.md` at `3057fc22f`. The counts (0 / 7 / 13) match V4 :15.
  - Every "what held" figure is V4's: 38 against 0 false claims on 200 models; ≤ 20g + 48 ≤ 68g against DS1's 139g; Φ.
  - The S2 decision (DS1 §8.1's estimator, `D1_REV_5A3_SSTAR_RESOLUTION.md:556`) and the S3 reversal (V4 :46, gate ratios 6.2e15–7.1e15) match their sources.
  - The NOTE list matches V4-N2, N7, N6 and N3.
- **R2:**
  - `c2f5539b…` is `V4_VERIFICATION.md` at `e5f4ef3a8`, and the counts 1 / 1 / 6 match V4 :263.
  - The LEVER2 description matches V4-R1: gain 2^90, spring 2^-580, claim ratio 1,005.
  - The confirmed departures and the V/4 arithmetic (68 + 64 = 132; 324 at V) match V4's checks 3 and 4.
  - The NOTE list covers R3–R7; R8 needs nothing.
  - The fix summary omits the directional blocks (N3).
- **R3:**
  - The verdict, the V4-T1 closure, the charge formula, "124 units", 2^-22·b, min(3p + 64, 1024) (R3 :349) and "44 controls" (R3 :79) all match their sources.
  - "5 NOTEs" copies V4's own miscount (N1).
  - One test requirement may lack a feasible control (N2).
  - The "rigour required in R4" item raises the bar above V4's first-order closure, and says so. Nothing in it exceeds ROOT's authority.
- **Nothing is described as VERIFIED or selected.** R1, R2 and R3 each call themselves "a proposal" and say "ROOT selects only after VERIFIED". Every commit message says NOT VERIFIED, and the pause note and the work graph keep K4 blocked.

### Pause and resume (:1447), and the heap-cap ruling

- **The pause note's heads are exact, and all four are on origin:** K5 `b379e5b27`, F1b `948e0bb99`, K4 `5ad1b6174`, and 5a.3 at numerics `9e9e3056a`. So are its K5 dispatch run (36459966791, `workflow_dispatch`, success, head `b379e5b27`) and V4's first-check counts.
- **The second pause commit (`7a6231dfa`) only appends** the agents' reports.
- **"Main is unchanged at `24dea2dae`" was true when committed.** `9fa4d1b59` is at 21:36:10Z, and PR #1043 merged 22 s later.
- **The heap-cap facts hold:**
  - 675,179,982 × 8 bytes = 5.4 GB;
  - C2 = 24 = 12 dense + 12 sparse, the 24 `n10000` runs that are `memory_refused` in G1's base;
  - 884 = 832 + 28 + 24.
- **The guard's conditions are proportionate.** Condition 1 is a derivation to be checked, not an assumption. The part-1 verdict is S2.

### K6 spawn (:1518)

- **The Q1–Q13 summary and items 1–12 match the brief.** Item 7 ("read as amended") matches "K4: Q5 amended".
- **The spawn base is wrong** (S1). The order departure is not named (N6).

## 3. The briefs I14 and I15 (PASS with S4; N4–N6)

- **I14 (K5):**
  - Its rulings Q1–Q11 match V1 :1303-1314.
  - The design departures are each recorded as rulings: Q1(b) item 3, Q2(b) item 4, Q3 item 5, Q4 item 8 and parallel running item 12.
  - Q7 (:84) and K5-C3 (:122) are superseded by V1 :1388, with no pointer (S4).
- **I15 (K6) against DESIGN:**
  - the K6 row (:1021) "after K1";
  - §4.8's watchdog (:826), Linux-peak rule (:827) and homes (:833-834);
  - D-8 (:1270) as amended.

  Each departure is a ruled item: 3, 4, 5, 6 and 7. The arithmetic holds:
  - 96 × 3,600,720,036 = 345,669,123,456 bytes;
  - 96 × 8,190² = 6,439,305,600 < 6,442,450,944;
  - 24 × 675,179,982 ≈ 16.2 GB;
  - P1's 3,617 MiB over 36,072,036 entries ≈ 105 bytes per entry.
- **I15 against the 128 GB, no-swap host:**
  - **No dense run at 10,000 members or more.** Four things enforce it: the binary's by-name refusal before any n² allocation (:161, test G :339-341, K6-M12); `--plan`'s refusal (test H :349); the stop list (:438); and the host rule (`I8R_K1_RESUME.md:31`).
  - **The admission rule** (:193): known Linux peak, or estimate × measured ratio (× 2 when none is measured), at most C/2 = 4 GiB. The ascent is size by size.
  - **A deterministic heap cap at C − 512 MiB** backs every run (Q3), so a mis-estimate aborts at allocation, and the host's memory is never exhausted.
  - **The identity-order lane's CONT at 10,000 members is never run** (Q12).
  - **The 16 GiB run** (Q4, :197): one process, last, alone, in a separately granted slot, with a 15.5 GiB heap cap. That is about an eighth of the host, well inside the guard's 35% floor. It still passes the binary's half-cap refusal and the admission rule.
  - **Tests** stay at 100 members or fewer. The watchdog test child is capped at 128 MiB and stops itself at 512 MiB.
  - **Safe as written.** N4 (the lane mode's by-name refusal) and N5 (the admission ratio's basis) are refinements.

## 4. The D1 5a.3 records (PASS on hashes; S5)

- **SHA256SUMS:** see §1. `V4_VERIFICATION.md` at the head is `ed2031c3…`, listed in the candidate SUMS.
- **The hashes the revisions cite hold:**
  - R1 `c265d7bb…`, R2 `326a8d78…`, R3 `2cbb6582…`;
  - every 16-hex prefix in R1 §10, R2 §12 and R3 §13 matches the committed file it names.
- **Missing files:**
  - R1's evidence is complete.
  - R2's and R3's tables name scripts and raw sweeps that are not committed (S5).
  - V4's records are complete against V4's own tables: first check, `delta_r2/` (the "67 files" at `e5f4ef3a8` holds) and `r3/` (including the two emulator copies its table lists).
- **Nothing is VERIFIED or selected** (§2). V4's delta sections end "Uncommitted.", which was true when written and is now stale. That is V4's text, not ROOT's.

## 5. The gate baseline (PASS)

- **The two tables have 884 rows each.** Each has 794 `ok` runs with an envelope hash and 794 full-envelope hashes; the key sets are equal. There are 0 differences between the first run and the full re-run in outcome, ok, exit code, summary hash or error hash.
- **The results.**
  - Both result JSONs give 764 evaluated, 328 trusted, 0 breaches and PASS.
  - Their `runs` arrays are equal as multisets. Only the order differs: the runs executed in parallel. "Equal row by row" in `RECORD.md` :42 is right in substance.
- **The RECORD's counts hold:**
  - RF-RANGE: 32 cases and 128 runs, split 62 / 38 / 28;
  - all 24 `n10000` runs are `memory_refused`;
  - the calibration comparison shows 6 message-only differences, all in RF-RANGE;
  - load peaked at 9.28, with memory at 95–96% (`host_samples.tsv`).
- **The uncommitted digests match the files in ROOT's scratch:**
  - `runs.jsonl` `8525c06d…` and `9139140c…`;
  - the digest lists `dd4338f3…` (2,813 lines) and `14d052ca…` (3,612 lines, 794 under `full/`).
- **What F1b's rulings rely on holds:**
  - C1 = 28, C3 = 832 with 0 differences, and C2 = 24;
  - I13's uncommitted `<wt>/scratch/i13/gate/compare_part1.json` and `gate2/compare_part1.json` give exactly those counts.
  - They also give `RESULT: FAIL` at `948e0bb99` (4 C2 heap-cap aborts) and `PASS` at `130445db2` (S2).

## 6. The K5 review records (PASS)

- **SHA256SUMS** is 163/163.
- **ROOT's D1 and D2 rulings match** RV14's delta rows :328-329 (§2).
- **RV14's merge analysis holds:** `28517eaaa`'s parents are `95c7501a7` and `65e2d6c2a`. The commit's own message names `df6d59e3c` (N11).

## 7. The App v4 relay records (PASS; N8)

- **The copies equal main `56dd72334`'s DEL-09-06 `Design/` files:**
  - `RELAY_ANSWERS_SWBPIPE.md` `afb6e063…`;
  - `FACTS_SQ01_SQ32.md` `733fb88a…`.
- **`DELIVERY_RECORD.md`'s table matches GitHub** (`gh pr view 1047`, `1048`; read-only):

  | Fact | GitHub |
  |---|---|
  | #1047 head | `c322826ea` |
  | #1047 merge | `41aeb2a02`, 22:49:33Z, `mergedBy` sgttomas |
  | #1047 commits | `e27794262` (parent `65e2d6c2a`), `73519d1d2` (merge of `d1cc97ce4`), `a2323bc96`, `c322826ea` |
  | #1047 files | 2 added |
  | #1048 head | `22a77811c` (parents `a999f4ba1`, `41aeb2a02`) |
  | #1048 merge | `56dd72334`, 23:09:58Z, `mergedBy` sgttomas |
  | #1048 diff | equal to `c322826ea..a999f4ba1` (both diffs hash `c261e4af…`) |
  | #1048 CI | 9 success, 4 skipped |

- **RV15's verdicts match `REVIEW_PR1047.md`:** 4 SHOULD-FIX and 10 NOTE at `a2323bc96`; D-1 SHOULD-FIX with D-2 and D-3 NOTEs at `c322826ea`; nothing new at `a999f4ba1`.
- **"No existing App file was changed" holds** for the two PRs. `RELAY_QUESTIONS_SWBPIPE.md` changed on main only through PRs other than these two.

## 8. The work graph's changed line (PASS with S1's counterpart; N7)

- **`WORK_GRAPH.md:62` is accurate at the head:**
  - PR #1044 is OPEN at `28517eaaa`;
  - F1b's re-run is on `130445db2` (`gate2` part 1 PASS);
  - DS1 is drafting R4, which exists in its scratch;
  - `<wt>/k6` is on `codex/piping-k6-20260928` at `56dd72334`, created 23:25:14Z;
  - PRs #1047 and #1048 are merged.
- **It states K6's base correctly** where V1 does not (S1). The "until VERIFIED" wording is N7.

## Findings

| ID | Severity | Site | Evidence | Resolution |
|---|---|---|---|---|
| S1 | SHOULD-FIX | `V1:1520` | **The K6 spawn base is misstated.** V1 says "spawned as I15 from main `41aeb2a02`".<br>• `<wt>/k6`'s branch `codex/piping-k6-20260928` was created at `56dd72334` at 23:25:14Z (its reflog), 10 s after V1's commit `195f15ab5` (23:25:04Z).<br>• The same PR's work graph (:62) says "from `56dd72334`", and I15 :290 says "from current main … records the SHA at spawn". So V1 is the SHA-at-spawn record, and it is wrong.<br>• No reliance is affected: `git diff 24dea2dae 56dd72334 -- projects/chirality-piping/` is empty, so "its piping tree equals `24dea2dae`'s" holds for both. | Add a bracket at :1520: "[Correction: the branch was created from `56dd72334` (PR #1048's merge), after main moved; the piping tree is unchanged.]" In future, record the base after creating the branch. |
| S2 | SHOULD-FIX | `V1:1451-1453`; `V1:1409` | **F1b's first gate part 1 failed its own gate condition, but V1 leads with "`gate_check` PASS".**<br>• I13's brief makes "no C2 sparse run aborting at the heap cap" a PASS condition (:499) and a stop (:556).<br>• I13's comparison at `948e0bb99` (`<wt>/scratch/i13/gate/compare_part1.json`) reads `RESULT: FAIL`, with `c2_sparse_heap_cap_aborts: 4`; the pause note (:25) says the stop condition was met.<br>• V1 lists the 4 aborts but never says that part 1 FAILED. "`gate_check` PASS" (trusted breaches only) can be read as the gate's verdict.<br>• Separately, :1409 restates "the gate's PASS requires" three conditions and omits the C2 clause. | Bracket at :1452: "part 1 FAILED on I13 brief :499 (4 C2 heap-cap aborts; a stop); `gate_check` (trusted breaches) passed." Bracket at :1409: "and no C2 sparse run aborting at the heap cap (I13 :499)". |
| S3 | SHOULD-FIX | `V1` (no entry); `REVIEW/K5_REVIEW.md:341` ("N3, N4: No action, as ruled") | **ROOT's rulings on RV14's first review are recorded nowhere in V1.** This is RV13-S2's class.<br>• RV14 found 4 SHOULD-FIX and 5 NOTE at `b379e5b27`. ROOT evidently ruled: fix RV14-1 to RV14-4, fix N1, N2 and N5, and take no action on N3 and N4 ("as ruled").<br>• For RV14-4, RV14 offered "otherwise give `None`, or publish [t, θ]" (:61). The fix instead refuses the witness, so 349 corpus witnesses change W → U (:307). That is a choice between outcome classes, and it has no ruling in V1.<br>• V1's only K5 review section is the delta (:1486). | Add a dated section, "K5: rulings on RV14's review (recorded late, RV16-S3)", with the four fixes, the chosen RV14-4 behaviour and its reason, and N1–N5. Cite K5's RETURN §16 on the K5 branch. |
| S4 | SHOULD-FIX | `TASK_BRIEFS/I12_K4_IMPLEMENTATION.md:140`; `V1:1202`; `V1:1310`; `TASK_BRIEFS/I14_K5_IMPLEMENTATION.md:84`, `:122`; `V1:1429`, `:1432`; `V1:1444` | **Six earlier rulings are superseded without a pointer at the superseded text.** Each later ruling does say what it amends; the older text stays unmarked, unlike Q5 (I12 :137, V1 :1132, :1146) and K3's Q7 ("SUPERSEDED").<br>(a) I12 Q6: "Otherwise the case is refused as unsupported" is amended by O1 (V1 :1203: "not refused"). K4 is still in progress under that brief.<br>(b) O12, "approved" at :1202, is reversed at :1432.<br>(c) K5's Q7(b), at V1 :1310 and I14 :84, is reversed to (a) at :1388. I14 :122 still offers K5-C3, which :1392 retires.<br>(d) "A case whose estimate exceeds V escalates" (:1429) and "the denominator becomes the bounded operator" (:1432) are modified by the confirmed W ≤ V/4 and the hybrid gate (:1475-1477).<br>(e) :1444's "2^-24·b" is 2^-23·b per V4-R7, which :1483 records as corrected. | Insert bracketed pointers at each site, for example "[Amended: 'K4: rulings on I12's checkpoint-0 plan', O1.]" and "[Reversed: see :1388 (S1).]". Keep the text. |
| S5 | SHOULD-FIX | `DESIGN_NUMERICS/REV_5A3_CANDIDATE/D1_REV_5A3_SSTAR_RESOLUTION_R2.md:623-650`; `…_R3.md:705`, `:717`, `:730`; `_run_records_r2/`, `_run_records_r3/` | **The 5a.3 revisions' evidence is only partly committed, and R3 says it was committed.**<br>• R2 §12 names `run_controls2.py`, `mutants2.py`, `gate_probe3.py`, `measure2.py`, `sweep2.py`, `summarize2.py`, `sweep2_{11,12}.json`, `r1_adapter.py` and `r1_lane2.py`. None is in `_run_records_r2/`, which holds 12 files.<br>• R3 :705 says "R2's evidence … ROOT committed it at `_run_records_r2/`".<br>• R3's `sweep3_{11,12}.json` are missing too.<br>• The committed `_run_records_r3/r1_lane3.py.txt:13` imports the uncommitted `r1_adapter`.<br>• Every hashed output is present and matches, so no hash is broken. But neither revision's "To rerun" works from the records. All the missing files (1.9–3.9 KB scripts, 0.6–0.9 MB sweeps) are in `<wt>/scratch/ds1/`. | Commit the ten R2 files and R3's two sweeps as `.py.txt` / `.json`, verifying each against R2's and R3's stated prefixes, and refresh the candidate SUMS. Or add a bracketed note to R3 :705 listing what was not committed. Prefer the first. |
| N1 | NOTE | `V1:1499`; `V1:1375` | (a) "NOT VERIFIED, with 1 BLOCKING (V4-T1) and 5 NOTEs": V4's table (:358-364) has 4 NOTEs (T2–T5), as V1's own list (:1512-1514) and `c3f2cfc72`'s message show. The "5" copies V4's verdict line (:350).<br>(b) "sha256 `7f50c788…`, 943 lines": the file is 942 lines, with a final newline. The hash matches. | Optional brackets: "[4 NOTEs; V4 :350 says 5]" and "[942 lines]". |
| N2 | NOTE | `V1:1510` | "A mutant that drops the charge is killed by a control that the charge alone refuses." V4's evidence shows no such control at q_W = 3p + 64:<br>• every non-LEVER model passes with a margin of at least 2^62 (`_v4_records/r3/r3_charge.*`);<br>• LEVER2 is refused by other tests as well (V4 :371).<br>R4 may have to construct one. | Say what happens if no admissible control exists at q_W, as I15 :362 does for mutants: "derive the equivalence and report it; ROOT rules". An emulator-only lowered q_W is another option. |
| N3 | NOTE | `V1:1473` | "one exact sum over the element contributions and spring stiffnesses" omits the directional-block contributions. Both V4's fix (:269) and R3 §5.5 item 1 include them. | Optional: add "and directional-block entries". |
| N4 | NOTE | `TASK_BRIEFS/I15_K6_IMPLEMENTATION.md:161`, `:339-341`, `:381` | The binary's by-name refusal at 10,000 members, test G and K6-M12 cover `dense` mode only. Q12's dense-LU lane mode also allocates n² (`FK/lib.rs:1631`). At 10,000 members it is stopped only by the estimate refusal and the heap cap. That is safe, but it is not the belt-and-braces the host rule (`I8R_K1_RESUME.md:31`) asks for. | Extend the by-name refusal, test G and K6-M12 to every mode that allocates n². |
| N5 | NOTE | `I15:193`, `:197` | The admission ratio is "the largest actual-to-estimate ratio measured at smaller sizes". At 10 members, fixed overhead can dominate the requested heap, so the largest ratio may make the Q4 run (which needs a ratio ≤ 1.334) or the 10,000-member sparse runs inadmissible. This is conservative, and affects availability only. | Optional: base the ratio on the largest measured size, or on the fit's n² coefficient, as ROOT rules at A2. |
| N6 | NOTE | `V1:1518-1523` | K6 runs before K4. The selected order is "…, then K4, then K6 and V-K" (`ROOT_SELECTION_DESIGNS.md:19`; DESIGN :1038). The K6/K6b split and "K6's row needs only K1" record the substance, but the departure is not named (RV13-D2's class). | Optional: "departs from the selected kernel order: K6's binary64 half runs before K4; K6b keeps the W1 part after K4." |
| N7 | NOTE | `WORK_GRAPH.md:62` ("K4 is **blocked** until D1 revision 5a.3 is VERIFIED") | V1 :1369 and :1516 block K4 until 5a.3 is selected, which follows VERIFIED, and implemented. | Optional: "until 5a.3 is VERIFIED and selected". |
| N8 | NOTE | `RELAY/DELIVERY_RECORD.md:9`, `:10`, `:13` | (a) :9 "base `65e2d6c2a`" is the branch point; when #1047 was opened (22:25:11Z), main was already `d1cc97ce4`.<br>(b) :10 lists `a2323bc96` under "main merged into the branch"; it is a documentation commit, and `73519d1d2` is the merge.<br>(c) :13 "the `sgttomas` account, not ROOT": GitHub shows the same account for #1048, which ROOT merged, so "not ROOT" rests on ROOT's attestation. The table shows, but does not say, that #1047 merged before any review covered its merged head `c322826ea` (the merge policy's "review … covering the actual candidate revision"). #1048 closed that gap. | Optional wording: "branch point `65e2d6c2a`"; "then `a2323bc96`, a follow-up"; "merged outside ROOT's session, before RV15's delta check of its head; #1048 carries the reviewed fixes". |
| N9 | NOTE | `REVIEW/_run_records/records_pr1042/delta/checks_7df3acefe.stdout.txt:61-81` (10 lines); `REVIEW/_run_records/k5_review/probes/probe_comparisons.txt:6`; `GATE_BASELINE_MAC_E7D930D49/full_envelope/variant_diff.txt:6`, `:50`, `:61` | Trailing whitespace appears only in raw run records. Only `variant_diff.txt`'s is disclosed, and only in `9fde956b2`'s commit message.<br>`git diff --check` also reports blank lines at EOF in `_run_records_r2/gate_probe3_cases.py.txt`, `k5_review/mutations/MUTANTS.txt` and `k5_review/mutations/logs/K5-M9.saprobe2/saprobe2_results_excerpt.txt`.<br>57 raw JSON outputs have no final newline. | Optional: one disclosure line per folder README or record, as RV14 did for K5's. |
| N10 | NOTE | PR #1049 checks (outside the diff) | "Select source coverage" and "Desktop E2E (source mode)" fail in run 36498076005 with "Update the PR base: event target base is missing, unavailable or not integrated into head". The head does not contain main `56dd72334`; the merge base is `24dea2dae`. Main's delta is 56 files, all under `projects/chirality-app-v4/`, and none overlaps this PR. | Merge main into the branch with a merge commit. Then run a delta check that the merge adds exactly `git diff 24dea2dae 56dd72334` and that `--remerge-diff` is empty, as RV13 did for `08e4b05fe`. |
| N11 | NOTE | K5 branch commit `28517eaaa` (outside the diff) | Its message says "Merge main `df6d59e3c` into K5", but its second parent is `65e2d6c2a`. RV14's delta (`K5_REVIEW.md:396`) states the parents correctly. | K5's merge record should disclose it, as K1_MERGE did for `1d105d633`. |

## What I did not do

- **No builds or runs** of product code, suites, gates or T9. No local GEN-8 (see §1).
- **Not verified against committed evidence:** the claims that rest on the F1b, K4 and K5 branches' uncommitted RETURNs. That covers:
  - F1b A2's RV11D-N2 figures and its "16 selections";
  - K4's F-1 to F-3 probe outcomes;
  - K5's Mac-main product run;
  - the dispatch's `target_base` (not in the run API or log).
- **Uncommitted evidence I read,** read-only: `<wt>/scratch/{i12,i13,i14}/CHECKPOINT0_PLAN.md`; `<wt>/scratch/i13/gate{,2}/compare_part1.json`; `<wt>/scratch/gate_base_e7d930d49{,_full}/`; `<wt>/scratch/ds1/` (the listing and file sizes); `<wt>/k6`'s branch and reflog.
- **Not re-derived:** the 5a.3 mathematics. I checked ROOT's rulings against V4's and DS1's stated results, not the results themselves.
- **Writes:** this file only, uncommitted. My check scripts and the extracted archive lived in my session scratch, and have been deleted.

## Delta check at 720924cbc

**Delta verdict: PASS.** Every SHOULD-FIX finding (S1–S5) is resolved. N1–N4, N6–N8, N10 and N11 are resolved; N5 is deferred to K6's A2, and N9 is disclosed. The check adds 3 new NOTEs (D1–D3) and no BLOCKING or SHOULD-FIX finding. The review's verdict stands: PASS.

**Scope.** ROOT resumed me for this check. The mechanism is unchanged: a background subagent of ROOT's session, with no delegation and no Git or GitHub writes.
- **Head:** `720924cbcb140a34a32cbc42c01dbeae0d32e971`. It is PR #1049's `headRefOid`, and `<wt>/numerics` HEAD; base main is `56dd72334`. Everything was read by commit, and I extracted `git archive 720924cbc` into my scratch.
- **The delta `04553ad05..720924cbc` has three commits:**
  - `c644b751d`: DS1's R4 and 15 of its run records, plus the candidate SHA256SUMS;
  - `adf43e1c5`: this review, committed verbatim (sha256 `1608735d…`, equal to what I returned), and ROOT's fixes (the commit ROOT first cited as "1a0…");
  - `720924cbc`: the merge of main `56dd72334`.

### Findings: resolution

| ID | Status at `720924cbc` | Evidence |
|---|---|---|
| S1 | **resolved** | V1 :1520 gains "[correction: the K6 branch was created at main `56dd72334`, which changes only `projects/chirality-app-v4/**`; the piping tree is the same (RV16-S1)]". It agrees with the work graph (:62) and with `<wt>/k6`. |
| S2 | **resolved** | V1 :1452 now opens "[Part 1 FAILED its own gate condition: 4 C2 sparse runs aborted at the heap cap … I13's comparison reads `RESULT: FAIL`. `gate_check`, which checks trusted breaches only, passed (RV16-S2).]". V1 :1409 gains the missing C2 clause, citing I13's PASS conditions. Both match `<wt>/scratch/i13/gate/compare_part1.json` and I13 :499. |
| S3 | **resolved** | The new section at V1 :1539, "K5: rulings on RV14's review (ROOT, 2026-09-28; recorded late, RV16-S3)", matches K5's `RETURN.md` §16 at `95c7501a7` and RV14's delta:<br>• the verdict and counts (RETURN :836);<br>• ROOT's direction to fix all four (:838);<br>• the four fixes and their pins (:842-845);<br>• I14's choice of refusal over "ROOT's alternative of publishing the parameters in a form that is always exact", with its reason (:893);<br>• the 349 W → U, every other result byte-identical (:931-932; `K5_REVIEW.md:307`, `:372`);<br>• N1 and N2 fixed, N3 and N4 no action, N5 by the merge (:846-849; `K5_REVIEW.md:339-342`). |
| S4 | **resolved** | Bracketed pointers now stand at all six sites: I12 :140 (and at :593, which repeats it), V1 :1202, V1 :1310, I14 :84 and :122, V1 :1429 and :1432, and V1 :1444 (2^-23·b). Each names the superseding section. |
| S5 | **resolved** | `adf43e1c5` adds 16 files: 10 to `_run_records_r2/`, 3 to `_r3/` and 3 to `_r4/`. Each is byte-identical to its file in `<wt>/scratch/ds1/`, and each scratch mtime predates its revision's commit: R2 11:57–12:15 against `9e9e3056a` at 12:38; R3's sweeps 16:27 against `c85dc1151` at 16:36; R4's sweeps 17:44 and 17:53 against `c644b751d` at 17:58 (local time, −0600). **The raw sweeps reproduce the committed summaries byte for byte:** running each `summarize{2,3,4}.py.txt` on its two raw sweeps under `nice -n 19` gives `b82092cf…`, `375087ef…` and `3bddcba8…`, exactly the committed `sweep{2,3,4}_summary.json`. Every script's imports now resolve within the committed records, `r1_adapter` included. |
| N1 | resolved | V1 :1499 "[4 NOTEs, T2–T5; V4's verdict line says 5 (RV16-N1)]"; V1 :1375 "[942 lines with a final newline (RV16-N1)]". |
| N2 | resolved | V1 :1510 gains the no-admissible-control rule. Its "R4 built one: CHARGE-SLENDER" matches R4 :573 and :777: the charge alone refuses it at 128 (102 times the allowance), and M17 is killed. |
| N3 | resolved | V1 :1473 gains "[and the directional-block entries (RV16-N3)]". |
| N4 | resolved | I15 :88, under ROOT's rulings, where ":68 … the condition binds": no run at 10,000 members or more in any mode that materializes an n² matrix, the dense-LU lane included. By-name refusal, test G and K6-M12 cover every such mode. |
| N5 | deferred | ROOT rules at K6's A2, as proposed. |
| N6 | resolved | V1 :1520 names the departure from the selected kernel order. It is an unbracketed insertion, tagged "(RV16-N6)"; the original words are kept. |
| N7 | resolved | `WORK_GRAPH.md:62`: "until D1 revision 5a.3 is VERIFIED and selected, and then implemented in K4". But see D1. |
| N8 | resolved | `DELIVERY_RECORD.md` :9, :10 and :13 are reworded as proposed: the branch point, "merge `73519d1d2`; then `a2323bc96`, the answers' note …", the merge before RV15's delta returned, and "rests on ROOT's attestation". These are rewrites, not brackets, of a record this PR added and main does not yet carry; that is acceptable. `RELAY/SHA256SUMS` is refreshed and verifies 4/4. |
| N9 | disclosed | The PR body's Checks section names the three trailing-whitespace files and "blank lines at EOF in some `_run_records` script copies (RV16-N9)". See D3. |
| N10 | **resolved** | See "The merge" below. Hosted CI on `720924cbc`: 7 success and 6 skipped, including `harness` (hosted GEN-8), "Select source coverage" and "Desktop E2E (source mode)", which had failed on `04553ad05`. |
| N11 | resolved | V1 :1556 records the erratum. K5's merge record is to disclose it. |

### The merge `720924cbc`

- **Parents** `adf43e1c5` and `56dd72334`; merge base `24dea2dae`.
- **It adds exactly main's delta:** `git diff adf43e1c5 720924cbc` and `git diff 24dea2dae 56dd72334` have the same sha256 (`d6483d63…`). All 56 files are under `projects/chirality-app-v4/`.
- **It keeps exactly the PR's delta:** `git diff 56dd72334 720924cbc` and `git diff 24dea2dae adf43e1c5` have the same sha256 (`89eaced6…`).
- **`git show --remerge-diff 720924cbc` is empty.** No path is touched by both sides.
- **The message** names the local branch, `codex/piping-numerical-integrity-20260926`. The commit is on both it and the PR branch `codex/piping-t3-records-20260928`; nothing relies on the name.

### In-place edits (`adf43e1c5` against `04553ad05`)

- **`ROOT_RULINGS_V1.md`:** 12 changed lines, and each old line is a subsequence of its new line (insert-only). At :1444, :1499 and :1520 the insert falls just before the line's final period. There is one appended section of 19 lines, dated.
- **I12, I14 and the work graph:** insert-only. **I15:** one added line.
- **`DELIVERY_RECORD.md`:** two rewrites (:9, :10) and one insert-only line (:13), all under N8.

### R4's records

- **SHA256SUMS:** the candidate `SHA256SUMS` lists 105 entries: the 73 at `04553ad05`, plus R4's 16 from `c644b751d`, plus the 16 from `adf43e1c5`. All 105 verify, and they are exactly the tracked files outside `_v4_records/`, which is unchanged. `RELAY/` is 4/4.
- **R4 is `4a52422d…`, 905 lines,** as its commit says.
- **Every §14 prefix matches its committed file:** `cc2df0b8`, `52113c96`, `a892a84a`, `48307994`, `ec78f507`, `1a6610a1`, `3bddcba8` and `ac336c5d`. So does §13's `ed2031c3` (`V4_VERIFICATION.md` at `c3f2cfc72`). R4's §14 statement that "ROOT committed" R2's and R3's evidence is true at this head.
- **Nothing calls 5a.3 VERIFIED or selected.** R4 :10 reads "This file is a proposal. V4 runs a delta check; ROOT selects only after VERIFIED". Every "selected" in the delta concerns a case's selected precision. V1 has no new 5a.3 ruling, and the work graph keeps K4 blocked.

### Hygiene

- **Machine paths and model identifiers:** none in the 41 files `c644b751d` and `adf43e1c5` change, by GEN-8's regex and the broader scan.
- **`git diff --check 04553ad05 adf43e1c5`** reports only a blank line at EOF in the three identical `r1_adapter.py.txt` copies (verbatim script copies, covered by the PR body's disclosure).
- **11 raw JSON outputs** have no final newline, as before.

### Delta findings

| ID | Severity | Site | Evidence | Resolution |
|---|---|---|---|---|
| D1 | NOTE | `WORK_GRAPH.md:62` ("DS1 is writing R4") | **Stale at the head.** R4 was committed at `c644b751d` (17:58) "for V4's delta check", before the fix commit (18:02) that edited the same sentence group. | "R4 is committed (`c644b751d`) and awaits V4's delta check." |
| D2 | NOTE | `REV_5A3_CANDIDATE/D1_REV_5A3_SSTAR_RESOLUTION_R4.md:3` | "NOT VERIFIED: 1 BLOCKING, 5 NOTEs" repeats V4's miscount of its R3 check, which V1 :1499 now corrects to 4. | Optional; DS1's text. V4's delta check at R4 may record it. |
| D3 | NOTE | PR #1049's description, Checks section (outside the diff) | "blank lines at EOF in some `_run_records` script copies" also has to cover two items that are not script copies: `REVIEW/_run_records/k5_review/mutations/MUTANTS.txt` and `…/logs/K5-M9.saprobe2/saprobe2_results_excerpt.txt`. | Optional: "in some raw run records". |

### What I did not do in the delta

- **The 5a.3 R4 mathematics:** not re-derived. R4 is DS1's proposal, for V4's delta check.
- **R4's §12 and §13 section hashes of `ROOT_RULINGS_V1.md`** (`69c8f23b…`, `9e53317f…`): not verified, because the section boundaries they hash are not stated.
- **Not re-checked:** facts from the first review outside the delta.
- **I15:** I did not verify that it received the N4 ruling.
- **Runs:** only the three `summarize*.py.txt` scripts, standard-library Python over committed JSON, one at a time under `nice -n 19`.
- **Writes:** only this appended section, uncommitted. My scratch extraction and scripts have been deleted.
