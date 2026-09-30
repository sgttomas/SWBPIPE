# T3 handoff for the audit pause (2026-09-30)

> **A dated snapshot.** Its present tense is as of the pause. For current status, read the T3 row of the work graph and the latest sections of `ROOT_RULINGS_V1.md`. This file builds on `OPERATING_NOTES_FOR_LOCAL_ROOT.md` and `HANDOFF_2026-09-28_TO_LOCAL.md`, which remain valid except where §7 below corrects them; read those two first.

## 0. Why this pause, and who reads this

- **The pause:** ROOT (HELP_HUMAN, the Mac session) paused T3 on 2026-09-30, for a session usage limit.
- **While ROOT is paused,** the owner runs a separate agent to audit the work to date and run repair cycles. ROOT then resumes, more or less from here, with the audit's findings and any repairs.
- **The aim** is to find issues early and fix them. Nobody is being caught out.
- **Readers:**
  - **the auditor:** §8, then §1–§6 for context;
  - **whoever resumes T3** (ROOT or another agent): all of it, and §7 before acting.
- **Placeholders:**
  - `<repo>` is the owner's `chirality` checkout.
  - `<wt>` is `<repo>/.claude/worktrees/swbpipe-control-layer-8a41be/.claude/t3`, which holds every T3 worktree, target and scratch folder. The T3 worktrees are `git worktree`s of `<repo>`.
  - `<VENV>` is `<repo>/.claude/worktrees/swbpipe-control-layer-8a41be/projects/chirality-piping/.venv`.
  - `T3/` is this folder; `P/` is `projects/chirality-piping/`.

## 1. State at the pause

### 1.1 Merged to main since the 2026-09-28 handoff

Each slice has `IMPLEMENTATION/<SLICE>_MERGE/RECORD.md`, with its gates and a sanitized DEC-025.

| Slice | What | PR | Merge |
|---|---|---|---|
| K2a | checked formation | #1032 | `f12e06876` |
| K1 | kernel sparse representation | #1034 | `eb52114e9` |
| M03 skew pin | tests and records: M03's scope on skew members (from RV7's B1) | #1038 | `e8b416e43` |
| K2b | W2's exact force-radix scaling, the kernel half of formation-time scaling | #1040 | `e7d930d49` |
| K3 | the rest of W1's arithmetic (`Wide<L>`, conversions, K4's primitives) | #1041 | `57617b0fb` |
| K5 | W4, the constrained-body witness and the curved rule | #1044 | `1cdeae2c1` |
| F1b | facade sparse wiring, W2 at formation, dense-scrutiny and lane guards (provisional 6 GiB) | #1052 | `59cb20073` |
| K6 | harness observations | #1053 | `7ac7b1c37` |
| K4 | W1a, the retained-precision kernel method under D1 revision 5a.3 | #1054 | `ab02ee3a6` |
| KF1 | K4's stop-rule trackers bounded (memory independent of the data) | #1056 | `0f5d8c7b4` |
| V-K | the VP-ROBUST kernel lane (`numerical_robustness`, VR), FK's `retained_api` export | #1057 | `f8400d290` |
| K6b | W1 observations on the K6 harness | #1058 | `78f55f927` |
| KF3 | W1a at scale: amendment A2 (an unformable certified bound is unavailable, not a stop); partial stage work and per-block refusals on every path | #1059 | `dd61120ff` |
| KF2 | K6's N10: the dense negative-pair witness from O(n⁴) to O(n²), every result bit-identical | #1060 | `7ad3a9adf` |

### 1.2 Open at the pause

- **No slice PR is open.** KF3 and KF2 both merged on 2026-09-30.
- **The records PR** (numerics → main, branch `codex/piping-t3-records-20260930`) carries this file and every T3 record since PR #1049. Its review, merge and outcome are recorded in a numerics ruling after this file ("Records PR … merged", or its open state if the pause came first). That ruling and anything after it stay on numerics until the next records PR.

### 1.3 Briefed, not spawned

- **K6c** (`TASK_BRIEFS/I21_K6C_IMPLEMENTATION.md`). It is spawned after KF3 merges, because its base must carry KF3's kernel. It covers:
  - W1's E_max re-derived phase by phase against the post-KF3 kernel and corrected (finding KF3-B2);
  - VR's stale E_max port deduplicated, with its admissions re-checked;
  - W1-T4 re-run post-KF3, as K6b's reserved RETURN addendum 1, in `IMPLEMENTATION/K6C/`.

### 1.4 No agent is running at the pause

Every TASK dispatched in this session has finished (on 2026-09-29 and 30 these included I12 to I20, RV17 to RV25, V4 and DS1). ROOT dispatched every TASK directly as a background subagent (host-native, D-GOV-35), and no T3 manager (WORKING_ITEMS) was used on the Mac: ROOT verified and committed every TASK's work. A resumed session has none of these subagents; start fresh ones from their briefs and the records.

## 2. Remaining T3 order

The work graph's T3 row governs where it differs.
1. **Rule on the audit's findings** (§10).
2. **K6c.** Then **ROOT's W1 limits** (per-case work and memory), from K6, K6b, K6c, V-K and K4's work counts. **They must be set before F2a merges.**
3. **The facade:** F2a (with D2's S-G1), then S-I, then F2b per domain, then F3. F2a is the first slice that makes W1a's results reach users.
4. **Candidate slices, routed but not scheduled:**
   - **The dense pivot screen** ("KF2: rulings on I20's checkpoint-0 plan", Q3). A profile-based operation count is honest (I20's proof, KF2 plan §7). It changes dense report bytes and dense classes, so it needs ROOT's ruling and an owner-facing note.
   - **D1 design question KF3-B1:** estimate (b) rejects R1's large trees at every precision (the ratio is independent of P). This is slack at about 3,000–4,000 members, and a design limit of the λ split at about 6,000 and above ("KF3: D accepted; KF3-B1 and KF3-B2 routed").
5. **The T3-close list:**
   - `HANDOFF_2026-09-28_TO_LOCAL.md` §3;
   - the work graph's T3 row;
   - the composite `SOURCE_BLOCKS_FINALIZATION_FAILED` item, which must close before T3 closes (ROOT_RULINGS_V1 :584).
   - Nothing in this list has been re-verified for this handoff.

## 3. Owner decisions pending (none is needed yet)

- **The provisional 6 GiB dense and observation-lane ceilings,** after V-P's product-level measurements (F1b; K6's B3).
- **The PHYS-R4 named refusal,** and the availability list it now carries:
  - **THIN-A and THIN-B:** W1a's design limit, κ ≈ 2^507 ("V-K: THIN confirmed by GEN");
  - **R1's large trees:** no retained-precision result at about 3,000 members and above (KF3-B1). The ordinary binary64 route still publishes them with its ordinary class.
- **The observation-lane framing** (F1b).
- **Noted for T6 and T9, not T3's:** a cancelled dense-scrutiny job keeps its thread and memory until the solve returns (`P/apps/desktop/src-tauri/src/lib.rs:1688-1692` and `:1711-1717`). After KF2 that is minutes (the dense factor), not weeks.

## 4. Navigating `ROOT_RULINGS_V1.md` (about 3,000 lines)

Sections are appended in time order, each headed "(ROOT, date)". A later section supersedes an earlier one only where it says so; corrections are in-place brackets `[Correction …]`. For the Mac-era slices, search these headings:

| Topic | Headings (search the text) |
|---|---|
| D1 revision 5a.3 | "D1 revision 5a.3: rulings on V4's delta check at R4 / R5 / R6"; "D1 revision 5a.3 SELECTED" |
| **Amendment A1** (S* below 2^-988) | "K4: rulings on RV19's review" (RV19-6) |
| **Amendment A2** (an unformable bound is unavailable) | "KF3: rulings on I19's diagnosis and plan; D1 revision 5a.3 amendment A2" |
| K4 | "K4: rulings on I12's A3-0 plan …" through "K4 merged" |
| K6, K6b | "K6: …" sections; "K6b and V-K: spawn" through "K6b merged" |
| V-K | "V-K: rulings on I17's checkpoint-0 plan" through "V-K merged" (THIN: "V-K: rulings on I17's A1 stop") |
| KF1 | "KF1: spawn" through "KF1 merged" |
| KF3 | "KF3: rulings on I19's diagnosis and plan …" through "KF3 merged" |
| KF2 | "KF2: spawn" through "KF2 merged" |
| K6c, V3 | "K6c briefed; V-K's V3 addendum satisfied by KF3's B" |
| Coupled merges | "Main merged into K6b and KF3"; "KF3: main merged; K6b's parity restored in KF3" |

## 5. How things are actually done on the Mac

The rules are in `_COMMON.md`, `I8R_K1_RESUME.md:24-50` (the Mac host) and `OWNER_DIRECTION.md`; this section is the mechanics.

### 5.1 Host

- **No swap.** The memory guard (`<wt>/guard/memguard.sh`, floor 35%, log `<wt>/guard/memguard.log`) has run since 2026-09-27. Check `pgrep -fl memguard` before heavy work.
- **Cargo:** `RUSTUP_TOOLCHAIN=1.97.1`, `RUSTUP_AUTO_INSTALL=0`, `CARGO_INCREMENTAL=0`, `--offline --locked`. Use one cargo job per slice at `-j 4`, `RUST_TEST_THREADS=2`, with the slice's own target `<wt>/<slice>-target`.
- **Concurrency:** two or three slices building at once was normal and safe. **Timed slots** (a slice's measurement runs) are granted by ROOT, and nobody else builds during one.
- **Reviewers and mutants** build from `git archive <exact commit>` into their own folders (`<wt>/rv<N>/`, `<wt>/<slice>-mut/`), never in an implementer's worktree.

### 5.2 DEC-025 on the Mac (owner decision, `OWNER_DIRECTION.md`, 2026-09-28)

1. `cd <wt>/sweep-skewpin && git checkout --detach <candidate full SHA>`. It must be a clean tree: remove any leftover `P/validation/evidence/sweeps/SWEEP_*.json` from an earlier sweep first. A leftover made K4's first sweep dirty.
2. `rm -rf <wt>/sweep-skewpin-target`.
3. `<wt>/scratch/sweep_skewpin/dec025_mac.sh <wt>/sweep-skewpin/projects/chirality-piping <logdir> <VENV>/bin/python <wt>`. The committed copy of the driver is `IMPLEMENTATION/M03_SKEW_PIN_MERGE/dec025/dec025_mac.sh.txt`. It takes about 35–40 minutes.
4. **The expected result:** the sweep invocation fails at the cargo surface on product_physics's `t13` (fail-fast), a platform test. The driver then runs all manifests `--no-fail-fast`, then pytest, the wasm build, vitest and the production build.
5. **Compare the suites with the previous Mac run keyed by manifest path:** `HANDOFF_2026-09-30_TOOLS/cmp_suites.py.txt`. The expected failures are exactly three platform tests:
   - product_physics `s11g_tests::t13_committed_fallback_uz_is_byte_identical`;
   - runner_headless `cli_load_reference_one_both_modes_is_controlled_and_equals_the_library_route`;
   - runner_headless `load_reference_route_tests::load_reference_one_actual_solve_mints_bound_evidence_and_canonical_document_both_modes`.

   Any other difference must be explained by the slice's own tests. Check a pytest count change too: K6b's +8 were its runner tests.
6. Copy the `SWEEP_*.json` into the log folder, then delete it from the sweep worktree. `rm -rf <wt>/sweep-skewpin-target` again.
7. **Records:** copy the evidence into `<SLICE>_MERGE/dec025/`, sanitized:
   - strip ANSI codes;
   - replace the venv path with `<VENV>` and the T3 root with `<wt>`;
   - record the unsanitized JSON's sha256 in `sweep_json_original_sha256.txt`.
   - The latest Mac baseline is `<wt>/scratch/sweep_kf2/`, whose piping source equals main `7ad3a9adf`'s (after KF2).
8. **vitest's `App.test.tsx` workspace render can time out at 30 s** under a load above about 8. A timing-only failure is re-run on a quieter host, never waved through (K2b precedent).

### 5.3 GEN-8, CI and the merge

- **GEN-8:** in a clean Git working tree of the candidate (never a `git archive` copy): `set -o pipefail; <VENV>/bin/python -m pytest -q tools/practitioner_harness/test_live_baseline.py -k gen8`. Expect 1 passed.
- **The full-SHA dispatch:** first merge main into the branch (a merge commit, never a rebase). Then run `gh workflow run piping-desktop-e2e.yml --repo sgttomas/chirality --ref <branch> -f target_base=$(git rev-parse origin/main)`, and record the run ID and target_base.
- **The merge:** `gh pr merge <N> --repo sgttomas/chirality --merge --match-head-commit <full head SHA>`, once review PASS (SHOULD-FIX closed by practice), green CI with the dispatch, DEC-025 and GEN-8 all cover that exact head.
  - Then write `<SLICE>_MERGE/RECORD.md` (dec025/ and SHA256SUMS), and add a "<slice> merged" ruling on numerics.
- **After ROOT merges main into a reviewed branch:** ask the reviewer for a merge check, as RV22 did for K6b. It compares path by path against main, gives the remerge diff of the resolved conflicts, and runs the suites on a clean archive. It is cheap, and it closes the gap between the reviewed head and the merged head.
- **Records PRs** (numerics → main): merge main into numerics, run GEN-8, open the PR, then an independent records reviewer (the RV13 and RV16 precedent). Hosted CI selects the records-only checks. Hold further numerics commits while it is under review.

### 5.4 Agents

- **TASKs make no Git writes and no index operations.** ROOT commits.
  - When a TASK's branch needs main, **ROOT runs the merge.** The TASK resolves conflict markers by editing only, and ROOT stages and commits.
  - **Verify the resolution mechanically:** for each resolved file, the diff from the base to main's side and the diff from the branch head to the resolution should differ only by the ruled delta. This was done for KF3's merge, `c0473301e`.
- **Briefs work best with:** the exact paths, the base, a write set with "anything else is a stop", explicit stop conditions, the checkpoint list (0 is plan-only; A is the code; B the measurements; D the records), and the host rules.
- **The subagent return path is noisy** (see also OPERATING_NOTES §2):
  - a `SendMessage` to a subagent that has just ended its turn may not be seen. After granting work, check that it started: a process, a scratch folder, or a file changing. If not, resend, saying it is a resend;
  - hand-backs can arrive twice (I20's checkpoint A did). Check the branch state before acting on one.
- **Reviewers** are fresh per slice, and are told to build independent oracles: exact rationals in Python (`fractions`), GEN, and probes of their own. Their verdict names the exact head, and later commits get a confirmation or delta check.

## 6. Records conventions (additions since 2026-09-28)

- **Hash-bound records are never edited.** A later addendum goes in a new folder (for example K6c's RETURN is K6b's addendum 1, in `IMPLEMENTATION/K6C/`).
- **`shasum -a 256 -c SHA256SUMS` must run from the folder that holds it.** Run from elsewhere, it reports false failures. The one exception on main is `REVIEW/_run_records/SHA256SUMS`, whose paths are relative to `REVIEW/`, so it is run from there (RV25-N5).
- **Machine-path scans** (`/Users/`, `/private/`, `/var/folders`) can match a record that describes the patterns themselves. Read the hit before treating it as a leak.
- **Rulings cite RETURN sections for figures.** Where ROOT restates a figure, compute it from the record in the same turn (§7.2).

## 7. Lessons from the Mac era: what helped, and what hurt

### 7.1 Patterns worth keeping

1. **Checkpoint 0 (the plan before code), with ROOT ruling on numbered questions.** It caught the scope of KF3's amendment A2, and split KF2's screen change out before any code.
2. **Stop conditions produce the best findings:**
   - K6b's backstop stop, which fixed a runner/binary disagreement;
   - K6b's K6B-S3 parity stop, which exposed K4's missing partial-stage accounting (fixed in KF3);
   - V-K's THIN stop, a design limit;
   - K6b's A1 memory finding, which became KF1.
3. **Scale runs reach paths nothing else reaches.**
   - KF3's scale run made the shifted 1024 factorization reachable, and exposed an omission in K6b's E_max (KF3-B2).
   - **Rule:** when a change makes a code path reachable, re-derive every estimate or check that models that path.
4. **"Whichever merges second" for coupled slices.**
   - FK's `retained_api` export was carried by both K6b and V-K.
   - K6b's parity check was tightened by KF3.
   - Both slices proceed in parallel; the second to merge adapts.
5. **The mechanical merge-resolution check** (§5.4) and **reviewer merge checks** (§5.3).
6. **Independent oracles for honesty claims.** Reviewers' exact-rational checks and GEN, rather than the implementer's tests.

### 7.2 Patterns that hurt, and the fix

1. **ROOT's restated numbers or claims were wrong at least seven times in two days.** Each now carries a bracketed correction in `ROOT_RULINGS_V1.md`:
   - KF1's work increase: "+15% of the stop rule" should be +21–159%;
   - E_adm: MiB divided by 1,000;
   - the E_max change at 1,000 members;
   - on 2026-09-30, a claim about admissions ROOT had not checked. It was replaced in place without a bracket at first; the bracket was added after RV25-N2;
   - V-K B's heap and E_max in GB that were MiB/1,000, the same slip again (RV25-S2);
   - KF3-B2 stated as a measured excess of 19.5 MB, when the like-for-like comparison is within the bound. It spread into K6c's brief and the work graph before RV25 caught it (RV25-S1);
   - the dense factor's time given as "65–188 s at 1,000 members", when 188 s is the 1,364-member ceiling (RV25-N3).

   Most came from compressing a RETURN's table into one sentence, which drops its qualifiers: units, the basis of a comparison, which size a range belongs to.

   **Fix:** don't restate figures; cite the RETURN section. When a figure must appear, compute it from the record in the same turn. Never state a "nothing relied on it" claim without checking.
2. **ROOT told agents to run `git merge --no-commit`,** an index operation their rules forbid. It was caught and corrected. ROOT does merges.
3. **Porting code across crates goes stale.** VR's copy of E_max fell 153 MB behind K6b's. **Fix:** share rather than port (K6c's dedup). If a copy is unavoidable, pin it to the original with a test.
4. **V-K's scale runner stops a tier at the first "not selected".** Re-run with `--run <tier>`, which skips recorded runs (I19's B note).
5. **The rulings file is long.** Use §4's index.
6. **ROOT merged KF3 without first checking that main had moved** (to PR #1061). It was harmless: #1061 touched only `chirality-app-v4`, and the merged piping, `tools/` and `.github/` trees equal the gated head's (`KF3_MERGE/RECORD.md`). But it was luck, not process. **Fix:** immediately before `gh pr merge`, run `git fetch origin main`, then `git diff --name-only <the main the head carries> origin/main`. If any path is under `projects/chirality-piping`, `tools/` or `.github/`, merge main into the branch and re-gate the affected checks.
7. **The work graph lagged the rulings.** Update the T3 row at every merge, not in batches.
8. **Shell detail:** in zsh, an unquoted `--include=*.rs` fails with "no matches found". Quote globs passed to grep.

## 8. For the auditor

The auditor is a colleague looking for issues early. Findings are welcome, and so are repairs.

### 8.1 Where to look, highest value first

1. **D1 revision 5a.3's amendments A1 and A2.** ROOT made both by ruling (§4). The base design (R7) had an independent design verifier (V4); A1 and A2 were checked only by slice reviewers (RV19 for A1, RV23 for A2). A design-level check of each honesty argument against R7's §5 text is the most valuable audit item. A2's claim is that B_c enters the guarantee only through B_c ≥ ‖K̃_c⁻¹‖₁, so a minimum over the formed, certified bounds preserves every step.
2. **ROOT's rulings' figures** (§7.2 item 1): sample the figures in the 2026-09-29 and 2026-09-30 sections against their source records (RETURNs, `_run_records/`, reviews).
3. **Merge records against reality,** for K4, KF1, V-K, K6b, KF3 and KF2 (and KF3's disclosed merge-order slip): heads, CI run IDs, target_base, DEC-025 comparisons, GEN-8 and the review verdicts.
4. **E_max:** by code derivation its 1024 shift term under-counts by a net 10.8 MB at 10,000 members (KF3-B2; K6c's job). Like for like, the measured peaks were still within it by 7.0–7.5 MB (RV25-S1), so it is not yet *shown* to bound that phase. Check whether anything besides the harness's own admission relied on it.
5. **The open design questions,** which the audit may investigate but not decide:
   - KF3-B1 (estimate (b), the λ split);
   - the dense-screen proposal (KF2 plan §7).
6. **Process:** that no TASK made Git writes, and that every committed record folder's SHA256SUMS verifies.

### 8.2 Conventions, so ROOT can pick the work up cleanly

- **Branch and files:** work on its own branch (for example `codex/piping-t3-audit-20260930`). Findings go in `T3/AUDIT/`, each with an ID, a severity (BLOCKING, SHOULD-FIX or NOTE), file:line, evidence and a suggested remedy, as the reviews in `T3/REVIEW/` do.
- **Leave `ROOT_RULINGS_V1.md` to ROOT.** ROOT rules on the findings when it resumes.
- **Repairs** go as their own PRs through the normal gates (§5.3), under the owner's standing Git authorization.
- **Say explicitly** if a repair touches:
  - KF2's or KF3's branch;
  - `FK/src/structural/retained/**`;
  - H's `k6/w1/counts.rs`;
  - `numerical_robustness`.
- **Mac rules** (§5.1): the memory guard; `-j 4`; no builds during a timed slot. Don't delete other slices' worktrees or targets.
- **The handback:** a short summary in `T3/AUDIT/` listing the findings, the repairs merged (PR, merge SHA), anything changed on branches ROOT owns, and anything left open.

## 9. Disk and prune candidates (nothing was deleted for this handoff)

`<wt>` holds about 130 GB. The records keep every hash, so the items below are regenerable. Prune only with the owner's word:
- **worktrees of merged slices:** `f1b`, `k1`, `k2b`, `k3`, `k4`, `k5`, `k6`, `k6b`, `kf1`, `kf2`, `kf3`, `vk`, `skewpin` and `sweep-k1`, about 2.2 GB each (sweep-k1 5 GB);
- **targets:** `k1-target`, `k4-b-target`, `k4-b`, `f1b-target`, `tauri-f1b-target`, `rv7-target`, `rv11-target`, `k6b-target`, `kf3-target` and `kf2-target`, about 24 GB. `root-target` (6.4 GB) is of unrecorded origin; check before removing;
- **scratch** (about 70 GB): the largest are `i13` (27 GB), `rv11` (12 GB), `i20` (about 12 GB, including KF2's uncommitted gate `runs.jsonl` files, whose sha256 is recorded), `calib`, the two `gate_base_e7d930d49*` folders, `i14` and `tauri_f1b`. The merged slices' sweep folders (`sweep_k4` to `sweep_kf3`) are small. `sweep_kf2` is kept as the current Mac baseline, which overrides the prune list in `KF2_MERGE/RECORD.md` (RV25-N7).
- **Keep:**
  - `numerics` and `sweep-skewpin` (the DEC-025 worktree);
  - `scratch/sweep_skewpin` (the driver), `scratch/sweep_kf2` (the latest Mac baseline; its piping source equals main `7ad3a9adf`'s) and `scratch/calib` (the platform calibration);
  - `guard/`.

## 10. Resuming

1. Read this file, §7 first, then the T3 row of the work graph, then the last sections of `ROOT_RULINGS_V1.md`.
2. Read the audit's handback (`T3/AUDIT/`), and rule on its findings in `ROOT_RULINGS_V1.md`.
3. Merge main into numerics (§5.4), checking what the audit's repairs changed.
4. Continue from §2: K6c (spawn I21 from its brief, on a base that carries KF3), then the W1 limits, then F2a.
