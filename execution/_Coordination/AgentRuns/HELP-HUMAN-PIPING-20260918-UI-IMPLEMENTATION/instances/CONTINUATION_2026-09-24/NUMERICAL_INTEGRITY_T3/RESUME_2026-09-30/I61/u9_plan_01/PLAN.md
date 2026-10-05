# I61: U9 plan, the standing gates and the compact main PR for the D1 milestone

**Summary.** U9 merges one compact PR from current main. It carries the 136 reviewed maintained files that F2a changes, plus a small evidence package of about 10 files. It does not carry the 5,677 execution-record files on NUM. The PR is the D1 milestone plus reader eligibility, and nothing more.

The gates run in two waves:
- **Content gates.** They run on the cut candidate while RV95 reviews: Pass B, the controls, T9, the both-entry gate, src-tauri and early hosted CI.
- **Exact-final-head gates.** They run on the frozen head after RV95 confirms: hosted CI with the full-SHA dispatch, GEN-8, Mac DEC-025 and the native witness. The merge follows immediately.

**Three facts found while planning change the work:**
1. Main has moved 87 commits past the F2a base. Two of them are the owner's cloud tasks, both already merged:
   - PR1078 changes `compatibility.py`, with one add/add conflict;
   - PR1080 changes the source-blocks validator, a production `.rs` change in `result_export`.

   So the full Pass B is needed, not the mechanical rerun.
2. Maintained source still says "no permit exists until U4 G5/G6" in six places. A line-neutral comment repair is needed before the freeze.
3. Maintained comments cite records that the compact PR will not carry. They need an index that resolves them.

**Estimate:** about 22–32 agent-hours, or 12–18 h wall. RV95 is the critical path.

**Method.** Read-only planning: no builds, no Cargo and no Git writes. Git reads used `GIT_OPTIONAL_LOCKS=0`. The only file operation outside the repository was a three-way `git merge-file -p` of `compatibility.py`, done on scratchpad copies. The memory guard (PID 5387) was running. The plan ran from 2026-10-04T19:34Z to about 20:10Z.

**Basis:**
- NUM `fd5032f6cb`;
- the U7 head `ffe65ef203` (RV94 is still reviewing it);
- `origin/main` = `5fdc5ab6012ddb50ecf286621a291b4da577405e`. That ref was last updated by a fetch at 19:31:21Z; this TASK did not fetch.

**Abbreviations:**
- P = `projects/chirality-piping`
- PP = P/core/product_physics
- T3 = P/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3
- R = T3/RESUME_2026-09-30
- RR = T3/ROOT_RULINGS_V1.md at NUM `fd5032f6cb`
- H = T3/HANDOFF_2026-10-03_TO_NEXT_ROOT.md
- WG = P/execution/_Coordination/WorkGraphs/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/WORK_GRAPH.md (at NUM)
- WT = the worktree root (scratch at WT/scratch, targets at WT/targets)
- B = `381be775ae` (the merge base of NUM, U7 and main)
- M = main at the cut
- U = the final reviewed source head (today `ffe65ef203`; after RV94's repairs, whatever ROOT commits)
- C = the compact candidate
- F = C's frozen final head

## 1. The gates

Every gate below is required by H:183–186, RR:6220–6223 or R/FIRST_PUBLICATION_PATH.md:75–79, unless the row says otherwise. Nothing here relaxes a criterion. Where a precedent ran a gate, its method is reused unchanged.

| # | Gate and definition | What runs | Who | Duration | Evidence |
|---|---|---|---|---|---|
| G1 | **A fresh independent complete review, and same-reviewer confirmation of any repair.** H:183–184; P/AGENTS.md:114–119; AGENTS.md:160–162 and docs/PRD_ROOT.md:507–514 (main) | **RV95**, fresh, on the **complete C diff against M**: the 136 maintained files plus the evidence package. See §4 for the method. RV95 also confirms any repair, and checks that F differs from the reviewed C only in execution paths | RV95 (TASK, Type 2), dispatched by ROOT | 8–12 h, plus 1–2 h for confirmation | R/REVIEW_RV95/u9_complete_01/ (REVIEW.md, SHA256SUMS); its verdict is copied into the PR package |
| G2 | **Hosted CI with the full-SHA dispatch.** H:184; RR:6221–6222; `.github/workflows/piping-desktop-e2e.yml:11–16` (the `target_base` input) and `:171` (the numerical cargo job) | The pull_request runs: Piping Desktop E2E, Harness Pre-merge, governance-harness and pec-tests. Then `gh workflow run piping-desktop-e2e.yml --ref <PR branch> -f target_base=<M, all 40 hex characters>`. ROOT verifies each run's `headSha` = F and its conclusion | ROOT pushes, opens the PR and dispatches; GitHub runs | About 35–45 min. The numerical job took 21–24 min at KF2 | Run IDs and conclusions in the merge record (as KF2_MERGE/RECORD.md "Hosted CI") |
| G3 | **The exact-final-head Mac DEC-025.** P/docs/BUILD_AND_RELEASE.md:132–177; T3/OWNER_DIRECTION.md:23–34 (Mac sweep plus Linux CI; the 3 known platform failures) | The recorded driver T3/IMPLEMENTATION/M03_SKEW_PIN_MERGE/dec025/dec025_mac.sh.txt, unchanged, in a clean sweep worktree of F, with a fresh `CARGO_TARGET_DIR`:<br>(1) the fail-fast sweep invocation, expected to stop at PP `t13`;<br>(2) all 40 manifests, `--no-fail-fast`, compared by manifest path against a Mac baseline of M;<br>(3) pytest, `build:wasm:desktop`, `test:desktop` and `build:desktop`.<br>**Expected:** the failing tests are exactly `t13` and runner/headless's two `load_reference` tests. Every count change is a test that F2a or main added, listed by name. The evidence also records PP's build status, from `retained_precision_admission`'s Registered assertion, because the cargo surface on this Mac is the registered identity (RR:10225) | **ROOT only.** No TASK runs DEC-025 | About 40 min for F (K6c: 35 min). A baseline NFF of M takes about 15–20 min, unless M's piping tree equals a tree already swept | NUM: T3/IMPLEMENTATION/F2A_D1_MERGE/dec025/ (README, SUMMARY.json, sanitized logs, SHA256SUMS), as K6C_MERGE/dec025 |
| G4 | **GEN-8.** H:185; tools/practitioner_harness/README.md:422–445; `test_live_baseline.py:241` | `python3 -m pytest -q tools/practitioner_harness/test_live_baseline.py -k gen8`, with `set -o pipefail`, from the repository root in a clean worktree of F. Also run once on C, early, because the evidence package is new and GEN-8 has failed on record placement before (RR:3206–3212) | ROOT | About 1 min | The log in the merge record (K6c: "1 passed, 10 deselected") |
| G5 | **Product T9.** RR:2737 (112 of 112), RR:677; method in T3/IMPLEMENTATION/KF2/_run_records/b/t9/ | S11-K's `fixdiff_main.rs` (sha256 `ec089c1d…`), built `--release --offline` in each of two `git archive` trees (M and C; `P` only, `execution/` excluded), runs over `core`, `fixtures` and `validation`, plus F1b's extra corpus. The script is the precedent `run_t9.sh`.<br>**Expected:** the 112 common outputs are byte-identical, and the extra corpus is 16 of 16. **2 outputs are added:** C adds `P/fixtures/product_preview/rf_skew_t_cant_off_122_r1e-04.request.json`, which the harness picks up in both modes. They are the ordinary value-route bytes, because a release build is Stale. Each must equal the Stale sweep's value-route row for that request, or the difference must be explained | ROOT-scheduled. A TASK may execute it under an explicit ROOT grant naming the host slot, as I20 did for KF2 | About 20–30 min | WT/scratch/u9_t9/ (bulk); hash lists and the comparison go to the merge record |
| G6 | **The both-entry gate, parts 1 and 2.** RR:677–685, RR:2737; method in T3/GATE_BASELINE_MAC_E7D930D49/README.md and KF2/_run_records/b/gate/ | **Build:** the full-envelope `t3_p1_probe` variant used at KF2 B (`main.rs` `cd1052f7…`, derived from `T3/DETECTION/probe/main.rs.txt`, with the calibration's 6 GiB heap cap), built `--release --offline` from the M and C trees. Its `numerical_use_standing_with_context` call exercises C's changed `result_export` on ordinary envelopes.<br>**Inputs:** `gen.py` regenerates the 223 request files, which must match the calibration's hashes.<br>**Part 1:** 884 runs per side, with G1's `gate_run_base_full.py` and `run.py`/`compare.py` unchanged, then K-D5's `gate_check.py` with the empty `S11_EXCEPTIONS.json` and `FORMATION_EXCEPTIONS.json`. **Expected:** 884 of 884 identical (outcome, exit, summary and full-envelope sha256, standing); gate_check PASS on both sides; 0 trusted breach triples; 0 heap-cap aborts.<br>**Part 2:** the 4 dense 1,000-member runs, base and candidate interleaved, one at a time, on a quiet host. Each must end within 1,800 s; KF2 measured 67–68 s.<br>The probe calls only the typed and value entries; the Direct entry is covered by G8 | **ROOT.** Phase A runs 10,000-member models and part 2 is dense at scale, so this is a solver-at-scale job: ROOT runs it or grants it explicitly | Part 1 about 40–50 min, including both builds. Part 2 about 10–15 min | WT/scratch/u9_gate/ (`runs.jsonl` is about 600 MB per side, uncommitted, with sha256 and size); SUMMARY, the indexes and gate_check logs go to the merge record |
| G7 | **The src-tauri suite.** RR:2737; WG:41 (it runs neither in hosted CI nor in DEC-025). C changes `apps/desktop/src-tauri/Cargo.lock` | In `apps/desktop/src-tauri`: `cargo test --offline --locked`, `CARGO_BUILD_JOBS=4`, `RUST_TEST_THREADS=2`, own target, on a `git archive` of C. **Expected:** the same outcome as on M (116 passed at KF2) | A TASK or ROOT | About 10 min | Log and meta in the merge record |
| G8 | **Pressure refusal with no-pressure success, and the exact-block coexistence controls.** RR:6232–6234; R/I48/product_availability_01/RETURN.md:34; R/FIRST_PUBLICATION_PATH.md:38–40, :76; RR:189, :229 | Each registered run uses the default `cargo test`, which is the registered identity. Each Stale run uses its own target with a non-empty RUSTFLAGS.<br>(a) PP's named tests in the registered and Stale builds: `tests/f1b_w2_runtime.rs:543` (PHYS-R4 refused) and `:565` (no-pressure published at b536); `tests/pressure_membrane_range.rs:90ff`; `retained_product_tests.rs:2616` (exact pressure selection suppresses all prepared work); `retained_facade_tests.rs:213` and `:648` (no-permit and no-W1 refusals keep exact bytes).<br>(b) The grant-2 fixture sweep (R/I61/u3_grant2_02/_run_records/run_sweeps.sh; 324 TSV lines per build over the P/fixtures requests and five routes, `retained_direct` among them), on C, registered and Stale. **Expected:** byte-identical to I66's `9a74ff16…` and `0e2db8b8…`.<br>(c) A records-side addition in the same scratch harness: the PHYS-R4 pair through the **Direct** entry in the registered build. The pressure request must give the ordinary refusal bytes with no notice, since D1 excludes pressure. The no-pressure companion must still publish and meet its 1e-9 / b536 expectation: as a successor; as exact ordinary bytes when W1 declines; or as ordinary bytes plus the single N1 notice after a fallback (RR:9252–9258) | A TASK (small models; one cargo job at a time) | About 2–3 h | NUM R/I61/u9_controls_01/ (RETURN, the sweep TSVs, SHA256SUMS) |
| G9 | **Resource evidence.** RR:11125–11133 (Pass B conditions); RR:10474 (a D1 call-graph change re-runs TEXT, with RV87's non-candidate review); RR:10407 (any new Direct caller requires re-qualification); RR:10547–10557 | (a) **The full Pass B on C** with I65's u4_g7_03 tools, as at R/I65/u4_g7_04. It is needed because condition (d) fails: main's `result_export/src/source_blocks.rs` is a production `.rs` change in the crate PP's precommit links. `semantic_contract::for_source` reaches `source_blocks::validate` statically (semantic_contract.rs:464), though not on the milestone's preview-physics branch. Pass B enters and classifies that hunk. If TEXT reports any D1 call-graph change, RV87's non-candidate review is repeated. **Expected:** the entry is byte-identical to `0c7827b6ad`'s, M included; the maxima stay at 0.8881 / 0.8929 M; law 42/0; the witnesses 9 of 9.<br>(b) A Direct-caller scan of C: callers of `run_linear_static_preview_value_with_retained_direct` / `_headless` outside PP and runner tests. Today the only ones are PP `lib.rs:2259`/`:2266`, `retained_memory.rs:2988` and runner `lib.rs:740–774`.<br>(c) The guard log is unchanged (no KILLED line) across G3, G5 and G6, with host samples | (a) I65, then RV89 confirms; (b) a TASK; (c) ROOT | (a) about 2–3 h plus 1 h for RV89; (b) 15 min | R/I65/u4_g7_05/ and R/REVIEW_RV89/u4_g7_03/; the merge record cites them |
| G10 | **Native Current.** R/FIRST_PUBLICATION_PATH.md:77; RR:6662; WG:44 ("native witness where the tranche touches native paths"); D-U7-1 (RR:10882–10883); D-U7-4 (RR:10889) | **The scope proposed in decision 7.** On F, in the native desktop app built from F on this Mac:<br>(i) the milestone model and the PHYS-R4 pair solve on the ordinary route with unchanged standing and Current display;<br>(ii) the result-export and stress-neutral panels keep their explicit successor gate;<br>(iii) TS's D-U7-4 binding is covered by vitest through the mocked IPC (G3 surface 3). A successor without live capture reads `needs_recompute` with `RETAINED_PRECISION_NATIVE_CAPTURE_REQUIRED`.<br>The app calls only the ordinary wrapper (RR:9261), so no successor can reach it. **Native Current for successors belongs to public activation** | **ROOT only.** The owner's Mac; no TASK runs native | 30–45 min | Screenshots and logs under WT/scratch/u9_native/; the summary goes to the merge record |

**Host serialization.** One Cargo job at a time on the host. ROOT orders the runs that build or run solvers: G9a, G8, G5, G6 and G7 run in sequence, and G3 last on F. RV95's own builds take slots that ROOT grants. Hosted CI (G2) and GEN-8 can overlap anything.

**Applicability.** RV89's mechanical-rerun allowance (RR:11127) does not apply once main is absorbed, by condition (d). T9 and the both-entry gate do apply: C reaches product callers through PP and `result_export`, unlike the H/validation-only K6c (RR:4888).

## 2. PR packaging

### 2.1 The source set S (measured)

S is `git diff --name-only B U -- . ':!P/execution' ':!execution'`.
- **At the U7 head:** 136 paths, all under P. 59 are added and 77 modified; none is deleted. Their current files total 11,791,905 B.
- **By kind:**

  | Kind | Files | Lines added | Lines removed |
  |---|---|---|---|
  | Rust production | 43 | 23,819 | 686 |
  | Rust tests and vectors | 41 | 16,520 | 135 |
  | TS production | 14 | 1,800 | 37 |
  | TS tests | 5 | 2,482 | 2 |
  | Python production | 3 | 1,904 | 6 |
  | Python tests | 11 | 2,409 | 11 |
  | JSON/YAML fixtures and schemas | 11 | 155,342 | 19 |
  | Cargo files | 8 | 61 | 0 |

  The largest file is `fixtures/results/retained_precision_cases.json` at 4.9 MB.
- **NUM's own non-execution set** at `fd5032f6cb` is the same paths minus 2. Since `3f5fca3010`, NUM's commits touch only execution paths, and U7 touches no execution path. So merging U into NUM is clean, and S is the same whether it is taken from U or from NUM after that merge. **Take S from U after RV94's repairs and the comment slice (§3.3).**
- The records branch (5,677 execution files against B) is **not** brought.

### 2.2 The cut

ROOT makes the Git writes; nothing is rebased or force-pushed.
1. **Branch** from `origin/main` = M: `git switch -c codex/piping-f2a-d1-<date> <M>`.
2. **Take S** with `git checkout U -- <S minus compatibility.py>`. Every path is A or M, so no deletion handling is needed.
3. **`compatibility.py`, the one foreseen conflict.**
   - **Main's PR1078 (`1a0bfe2b71`, RV92 S-1 applied on main)** adds `_same_canonical`. Its body is byte-identical to the helper U already has. PR1078 also switches the two `source_block_recovery` and `contract_evidence` comparisons to it.
   - **A three-way `merge-file` (U, B, M) gives one add/add conflict:** the duplicate helper. Every other hunk merges cleanly.
   - **Resolution:** keep U's single helper and take main's two call-site lines.
   - **The result differs from U's file at exactly 2 lines** (:694 and :699 at U's numbering), and those lines are main's.
   - Main's 86 new lines in `tests/test_analysis_run_compatibility.py` then run against it.
4. **Commit 1, the source snapshot.** The message lists U, B, M, the source commit chain and the reviewing RVs, with truthful agent attribution.
5. **Commit 2, the evidence package** (§2.4).

The PR is a two-commit compact branch whose parent is M. Main is an ancestor, so a `--merge` produces exactly C's tree.

### 2.3 Showing maintained-source equality

A TASK writes `SOURCE_EQUALITY.json` and ROOT re-runs the checks. Each must print nothing or equal its stated expectation:
1. `git diff --name-only M C -- . ':!P/execution'` equals S exactly, as sorted lists.
2. `git diff --exit-code U C -- <S minus compatibility.py>`, modes included (`git ls-tree` on both).
3. `git diff U C -- P/core/analysis_runs/compatibility.py` is exactly main's two call-site hunks. Re-deriving the file with `git merge-file -p` from U, B and M, with the single resolution above, gives C's blob.
4. `git diff --name-only M C -- P/execution` equals the evidence package list.
5. For RV95: each S path's blob at U and at C, with the commit that last changed it on U.

**Foreseen main movement.** Main was 87 commits ahead of B at planning time. Of those changes, only 4 non-execution files are under P:
- `compatibility.py` and its test (PR1078);
- `result_export/src/source_blocks.rs` and its test (PR1080, `5b4f31766c`: receipt-order row walk).

The rest are App v4 records, governance-harness and workflow files, and Root `AGENTS.md` (D-GOV-52). None of these overlaps S apart from `compatibility.py`. **The two owner cloud tasks ROOT mentioned have already merged** as PR1078 and PR1080.

PR1080 overlaps no S file. Its effect on F2a is semantic only: a deterministic first refusal among several failing source-blocks rows. G1, G3 surface 2, G8 and G9 cover it.

**If main moves again before the cut,** re-run steps 1–3. Any new overlap with S, or any change to one of PP's 14 `REVIEWED_INPUTS` (`build_identity.rs:148–163`), is a stop to ROOT. A changed reviewed input would make the build Stale and change the registration.

### 2.4 What evidence goes in, and what stays outside

The rules are RR:6245–6250 and RR:6919–6945: concise records, decisive outputs, small inputs and manifests; the PR body states the record-file count and bytes with a reason; references must resolve.

**In the PR:** T3/IMPLEMENTATION/F2A_D1/, about 10 files and 0.2–0.3 MB. GEN-8-safe names, placeholders only:
- `CHANGE_RECORD.md`: the scope text (§3); the source chain from B to U, with commits, TASKs and RVs and their seals; the equality summary; and the planned gates;
- `SOURCE_EQUALITY.json` (§2.3);
- `REFERENCE_INDEX.json`. Maintained lines added in S cite records that main does not carry:
  - 7 `R/I6x/...` paths (for example `retained_memory.rs:944`, "R/I65/u4_g6_01/QUALIFICATION.md");
  - 2 `REVIEW_RV77/...` paths;
  - the generated-profile marker "part2/_run_records/g5_profile.py";
  - about 20 RR citations. Main's RR has 4,894 lines, NUM's 11,177.
  
  Each citation maps to the NUM commit SHA, path, sha256 and bytes. Comments are not edited to remove citations;
- `_run_records/` copies of the few decisive small files those comments depend on:
  - the final `g5_profile.py` (33,191 B) and its `profile_tree.json` (29,665 B), which regenerate a maintained block;
  - `QUALIFICATION.md` (38,177 B), the basis of M and the identity;
  - `G2_AMENDMENTS.md` (10,154 B);
- `EXTERNAL_MANIFEST.json`:
  - NUM's head SHA and execution census (file count and bytes);
  - the gate and sweep bulk under WT/scratch, with sha256, bytes and location;
- `review/`: RV95's final REVIEW and confirmation, added in the freezing commit (as A1's head carried RV31's packet);
- `SHA256SUMS`.

**Outside the PR:** the gate bulk (G3 raw logs, G5 outputs, G6 `runs.jsonl` and envelopes, G8 TSVs, G9 TEXT chains) stays under WT/scratch with manifests. Post-merge records (F2A_D1_MERGE/, rulings, graph) go on NUM. Any later records PR is cut from main with only execution paths and is not part of U9 (H:176–177; RR:6212–6218).

## 3. Scope truthfulness

### 3.1 What becomes public on main

- **The PP Direct retained entry,** `run_linear_static_preview_value_with_retained_direct` (PP `lib.rs:2259`). It publishes the M03-INTEGRITY-MP-v2 successor only for D1 requests:
  - one load case, no combinations, the preview family, no pressure, capped counts (RR:8825; D1.0–D1.11);
  - in the registered dev/test build identity (aarch64-apple-darwin, rustc 1.97.1 `8bab26f4f68e`, debug, opt-level 0, debug assertions on, panic=unwind, no RUSTFLAGS);
  - with M = 4,026,531,840 B (RR:10225, :10547; `retained_memory.rs:944–960`).
- **The Headless entry is refused at D1.0.** The milestone RF-SKEW-T-CANT-OFF-122-r1e-04 publishes through the Direct entry in both modes and agrees with its independent reference (U5; the post-U7 report is the baseline, RR:11152).
- **The three readers' eligibility** (Python, Rust and TS; U7). A supplied successor statement validated with its actual invocation reads `numerically_eligible`. In TS it reads eligible (`integrity_checked`) only while the live native capture holds (D-U7-4).
- **Eligibility describes a statement and its invocation.** It makes no producer-origin claim (D-U7-6).
- **The carriers transport the successor** (U6). The desktop export panels refuse it by an explicit gate (the T6 notice, WG:38).

### 3.2 What stays closed, stated in the PR

| Closed | Basis |
|---|---|
| **Public activation.** No product caller (desktop, native app, headless CLI) publishes a successor; the desktop calls only the ordinary wrapper | D-U7-1 (RR:10882–10883); RR:9261 |
| **Stale builds.** Every other identity keeps exact ordinary bytes: release and product builds, hosted Linux CI, other hosts and toolchains. Hosted CI therefore exercises only the ordinary path; the registered path is covered on this Mac only (G3, G8, G9) | RR:10225, :10552 |
| **Any supported-machine statement of M** | Owner-held, decision 9 (RR:8828, :10884) |
| **Wider F2a:**<ul><li>multi-case invocations and combinations;</li><li>preparation-only and mixed invocations, and the promised exact routes beyond D1;</li><li>complete invocation charging;</li><li>U8's deferred witnesses (the native Ceiling row, the L = 0 base, RV93 N-5);</li><li>registration of the release identity;</li><li>re-qualification of any future Direct caller</li></ul> | H:55–57; R/I61/step4_plan_01/PLAN.md:158–160; RR:11067, :10407 |
| **The complete invocation's exact-block no-attempt rule.** D1 is single-case, so G8 shows single-case coexistence only; "a successful single-case test cannot establish it" | R/FIRST_PUBLICATION_PATH.md:39–40 |
| **S-I, F2b per domain, F3** | H:58 |
| **The owner-held items:** dense and lane ceilings, PHYS-R4 refusal and availability, observation framing, the KF3 lambda split, the KF2 dense screen | H:194–196 |

### 3.3 Where it must be said

1. **The PR body:** §3.1 and §3.2 as written; the record count and bytes with their reason; the equality summary; the gate table with run IDs; and the sentence "registered-path evidence is Mac-only; hosted CI is Stale".
2. **`CHANGE_RECORD.md`** in the PR, with the same text.
3. **Maintained text: a pre-freeze, comment-only, line-neutral repair.** These six places still describe the pre-registration state:
   - PP `lib.rs:2185` ("Without a permit (until U4 G6) it is always `Ordinary`");
   - `lib.rs:2286` ("No permit exists until U4 G5");
   - `lib.rs:2919` ("Unreachable until U4 G5 adds a registered profile");
   - `retained_memory.rs:5–8` ("No production profile is registered … empty until G6 … every call refuses");
   - `retained_memory.rs:2847` ("with none registered (until G6), never");
   - `retained_facade_tests.rs:2`.
   
   Each must state the registered dev/test identity truthfully. G9's Pass B classifies them as comment-only. RV94 may already route them under its item 5; if not, ROOT grants a 1 h slice. No maintained doc under P/docs describes F2a, so none needs a change.
4. **After the merge, in the same pass:**
   - the merge record;
   - an RR ruling;
   - WG's T3 row (the milestone merged; the open list above);
   - a T6 notice: main's readers now grant eligibility, and successor outputs stay refused until T6 replaces the gate;
   - R/ROOT_CURRENT.md;
   - an appended status note in R/FIRST_PUBLICATION_PATH.md.

### 3.4 The handoff's "No scope omission is authorized" (H:55–58)

This PR does not complete F2a and must not say it does. It omits no obligation: everything in §3.2 stays open, by name, in the PR, in CHANGE_RECORD and in the work graph.

It is consistent with:
- **FIRST_PUBLICATION_PATH:10–11** (no producer-only merge): producer, readers and carriers ship together, as §3 there requires;
- **the accepted step-4 sequence:** U7 → U9, with U8 after the milestone (RR:8807–8817).

**The tension is H:161–162.** That step order puts "the final product gates/PR" after "complete wider F2a obligations", and H:177 speaks of "the final F2a PR". This plan reads the U9 PR as the **D1 milestone PR**, an intermediate F2a PR. The final F2a PR remains for complete F2a. ROOT should rule this explicitly (decision 1).

## 4. Order, owners and review

| Phase | Work | Owner | Wall |
|---|---|---|---|
| 0 (now) | RV94 finishes; ROOT rules on §5 | ROOT | — |
| 1 | RV94 repairs on U, if any; the comment slice (§3.3 item 3); U merged into NUM | Implementers, ROOT | 1–3 h |
| 2 | The cut (§2.2); the equality, index and manifests (§2.3–2.4); GEN-8 on C; push and open a **draft** PR so the pull_request CI runs early. This is the first time hosted Linux CI sees F2a code, and it is Stale there | ROOT, plus a TASK for the manifests | 1.5–2 h |
| 3 (parallel) | **RV95** on the complete C diff. **G9a** Pass B, with RV89 confirming. **G8** controls. **G5, G6 and G7** in ROOT's host slots | RV95; I65 and RV89; a TASK; ROOT | 8–12 h (RV95 is the critical path) |
| 4 | Repairs, if any, then RV95's same-reviewer confirmation. Reassess every gate a repair touches: a source change re-runs G5, G6, G8 and G9 as affected | Implementers, RV95, ROOT | 0–4 h |
| 5 | Freeze F (add RV95's return); re-confirm that F differs from C only in execution paths; G2 (dispatch with target_base = M), G4 and G3 on F; G10 native on F; RV95 confirms the gate evidence read-only | ROOT; RV95 | 1.5–2 h |
| 6 | Immediately before merging: fetch, verify `origin/main` = M and the PR head = F, then `gh pr merge --merge --match-head-commit F`. Verify both merge parents. In the same pass: the merge record, ruling, graph, T6 notice and ROOT_CURRENT; then merge main into NUM by a merge commit | ROOT | 1 h |

**RV95 (fresh; it wrote none of S and is not RV94).** RV95 reads the complete maintained diff against M. Fixtures are checked mechanically: hashes against the corpus generators, and every must-pass entry exercised by tests. Its brief, in priority order:
1. **Equality and coverage.** Rerun the §2.3 checks. Build a ledger from each S path to the accepted review that covered its final blob (RV77–RV94). Review every blob or hunk no accepted review covered in depth. That includes `compatibility.py`'s resolution, the comment slice and any RV94 repair.
2. **The integration seams across components:**
   - PP → result_export precommit;
   - permit construction, linear use and D1 admission;
   - registration and the reviewed inputs;
   - reader eligibility in three languages against the TS live-capture rule;
   - the carriers and panel gates;
   - main's PR1078 and PR1080 semantics against F2a's corpus.
3. **Scope truthfulness** (§3), the stale-text sweep, and `#[cfg(test)]` containment (no hook symbol in non-test builds).
4. **Packaging:** the index resolves; no machine paths; the record count and bytes match the body.
5. **Its own adversarial checks:**
   - at least 6 mutants across the seams;
   - a Linux-like Stale build (non-empty RUSTFLAGS) of PP, runner and result_export;
   - the milestone in the registered build, with the reader verdict.

**Estimate:**

| Item | Hours |
|---|---|
| RV95 | 8–12 |
| Its confirmations | 2–3 |
| Pass B with RV89 | 3–4 |
| G8 and G9b | 2.5–3.5 |
| ROOT's gates (G3, G5, G6, G7, G10 and GEN-8) | 3–4 (about 2.5 of it machine time) |
| The cut, manifests and equality | 2–3 |
| Repairs | 0–4 |
| The merge and records | 1 |
| **Total** | **about 22–32 agent-hours, about 12–18 h wall** (3–4 working sessions) |

This is above step 4's 6–10 h figure. That figure predates main's movement, the full Pass B, the comment slice and the reference index.

## 5. Decisions for ROOT (and the owner where marked)

1. **The PR's identity (ROOT; owner notice recommended).** Proposed: the U9 PR is the **D1 milestone PR**, with the scope in §3. The H:177 "final F2a PR" remains for complete F2a. U8 stays out (RR:8817, :11067).
   - Basis: the accepted step-4 sequence; FIRST_PUBLICATION_PATH §3's one package.
   - The alternative is to hold until all of wider F2a is done. It means one later and larger PR, while main keeps moving (87 commits in two days).
   - It is the first F2a code on main, and it turns reader eligibility on in main's readers. Proposed: ROOT sends the owner a notice, not an approval request (H:196–197).
2. **The packaging mechanism.** Proposed: a two-commit compact branch from M (§2.2): a source snapshot by path checkout, then the evidence package. No merge of NUM or U history. Ancestry to the reviewed commits is shown by SOURCE_EQUALITY and the commit message.
3. **The `compatibility.py` resolution.** Proposed: one `_same_canonical`, plus main's two call-site lines (§2.2 step 3). RV95 reviews it. Python suites and main's new tests run on C.
4. **Pass B.** Proposed: the **full** Pass B on C, not the mechanical rerun, because RR:11127 condition (d) fails on main's `result_export` change. If TEXT moves, RR:10474's TEXT and non-candidate review follow.
5. **References in maintained comments.** Proposed: resolve them through `REFERENCE_INDEX.json` to immutable NUM revisions. Copy only the 4 decisive small files (about 111 KB). Do not edit source comments to remove the citations. Keep the NUM branch as the immutable store; any tag is optional and ROOT's choice.
6. **The stale "until G5/G6" text.** Proposed: a comment-only, line-neutral repair before the freeze (§3.3 item 3), unless RV94 already routes it. Pass B classifies it.
7. **What native Current means at U9 (ROOT).** Proposed: G10 as scoped. That is the ordinary-route native witness, the panel gates and D-U7-4's TS binding. Native Current for successors moves explicitly to public activation, where a native caller exists (D-U7-1). The alternative, demonstrating native Current for a successor now, needs a product caller, which D-U7-1 excludes.
8. **The merge window and main movement (owner, via ROOT).** Proposed: ask the owner to hold merges to main for about 2 h covering phase 5–6. Without a hold:
   - if main moves in any path, ROOT merges the new main into the PR branch by a merge commit (never a rebase);
   - ROOT then re-runs G2 (with the new target_base), G4 and G3 on the new head;
   - G5, G6, G8 and G9 carry over only if `git diff --quiet F F' -- P ':!P/execution'` holds (product trees byte-identical); otherwise they re-run.
9. **The DEC-025 driver's parallelism.** It records `CARGO_BUILD_JOBS=8` and `RUST_TEST_THREADS=4`, which the current TASK host rule would not allow. Proposed: ROOT runs the recorded driver unchanged on a quiet host, with the guard on, for comparability with K6c and KF2. The alternative, 4 and 2, changes the driver's hash but no criterion.
10. **The probe and T9 harness builds.** They cannot use `--locked`, because cargo adds the harness package to the copied lock. Proposed: build `--offline` as in the precedents. After the build, diff the lock to show only the harness package was added.
11. **The Mac baseline for G3.** Proposed: a fresh NFF baseline on M. Reusing K6c's baseline is not proposed, because M's piping tree has changed since (PR1078, PR1080).
12. **RV95 also confirms the gates.** Proposed: RV95 confirms the gate evidence read-only, as RV31 did for A1, so no further reviewer is needed. The alternative is a separate fresh gate confirmer.
