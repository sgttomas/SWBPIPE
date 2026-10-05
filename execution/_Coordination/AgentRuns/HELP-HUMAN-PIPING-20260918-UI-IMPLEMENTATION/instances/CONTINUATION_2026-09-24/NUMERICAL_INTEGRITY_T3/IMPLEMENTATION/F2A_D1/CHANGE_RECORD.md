# F2a D1 milestone: change record

**This PR brings T3's first retained-precision (F2a) code to main: the D1 milestone and reader eligibility.**
- **The milestone:** the synthetic skew cantilever RF-SKEW-T-CANT-OFF-122-r1e-04 publishes an M03-INTEGRITY-MP-v2 successor.
  - It publishes through the product-physics Direct entry, in both solver modes.
  - It does so only in the registered dev/test build.
  - The successor agrees with its independent reference.
- **The readers:** the Python, Rust and TypeScript readers can now treat such a statement as numerically eligible when it is supplied with its actual invocation.
- **Nothing publishes successors to users.** No product caller is added, every other build keeps the ordinary route, and public activation is a later, separate step (§4).

This is an intermediate F2a PR, not the completion of F2a. The obligations in §5 stay open by name. The review is agent review, not personal review by the owner.

**Status of this file.** Regenerated at the freeze (RV95 S-2), then updated by ROOT at the refreeze for the one defect DEC-025 and hosted CI found on the first freeze (§3, After the cut).
- **The PR's source head** is `6cfe50d368`.
- **Its integration head** is NUM `42009dba72` (`codex/piping-numerical-integrity-20260926`). Its maintained source equals the PR's, except the two files main also changed, which are recorded three-way merges (§2).
- **The cut** was `5a0461661f`, from U = `2e03d7cc25`. The six source changes since the cut are in §3, "After the cut".
- **This package's commit makes the frozen head F′.** The first freeze, F = `20dd3d929d`, failed one test crate in DEC-025 and hosted CI (§3). Gates on F′ are rerun or carried over by ruling, and recorded in the post-merge record (§7).

**Notation:**
- **P** = `projects/chirality-piping`.
- **RR** = T3's `ROOT_RULINGS_V1.md`, which is append-only, so its line numbers are stable. An RR "title" names a ruling heading.
- **R/…** paths are T3's `RESUME_2026-09-30` records on NUM.
- **Records resolve through `citations.json`:**
  - every record path or RR line that maintained source cites maps to a commit-pinned GitHub URL on NUM;
  - four small files are copied into this package (§6).

## 1. What the PR contains

| | |
|---|---|
| **Maintained source (S)** | 140 files under P: 59 added, 81 modified, 12,016,302 B; +204,707 / −908 lines against main `5fdc5ab601`. 138 are byte-identical to the integration head `42009dba72`. The other two, `compatibility.py` and `source_blocks.rs`, equal the recorded three-way merges (§2) |
| **This package** | 10 files, 194,107 B including `SHA256SUMS`: this record, the PR body, `source_equality.py`, `check_citations.py`, `citations.json`, four copied records, `SHA256SUMS` |
| **Not included** | The integration branch's 5,677 execution-record files (191.4 MB). Under the handoff packaging rule and U9 decision 5, they stay on NUM. Gate bulk stays in the T3 scratch area, with hashes in the post-merge record |

**By area:**

| Area | Paths | What |
|---|---|---|
| Kernel (frame_kernel) | `core/solver/frame_kernel/src/structural/retained/**`, `retained_resource.rs` | Retained-precision arithmetic for the product: the directed certificate, product certificate (bridge, final case, source residual), origins, checked work, resource constants |
| Product facade (product_physics) | `src/retained_*.rs`, `build.rs`, `build_identity.rs`, `lib.rs` (the Direct and Headless retained entries) | The prepared producer, the receipt serializer, facade capture behind a linear permit, admission law and memory profile, build identity |
| Readers | `core/analysis_runs/retained_precision.py`, `result_export/src/retained_precision.rs`, `apps/desktop/src/features/results/retainedPrecision*.ts` | Ordered G0–G8 checks, standing and eligibility |
| Carriers | AnalysisRun (Python and TS), derivative, stress-neutral export, semantic contract, the desktop panels | The successor travels with its receipt; the export panels refuse it explicitly |
| Schemas and fixtures | `schemas/retained_precision_mp_v2.schema.json`, `results.v0.3` successor branch, the corpus (snapshot 07k), the carrier case file (v4), the milestone request and successors | The contract and the shared test corpus |
| Runner, harnesses | `core/runner/headless`, `performance_harness`, `numerical_robustness` | The headless retained entry (refused at D1.0) and the observation harness updates |

## 2. Main movement absorbed

Main moved past the base `381be775ae` to `5fdc5ab601`. Two files in S are files main also changed, and each equals its recorded three-way merge of (integration head, base, main). `source_equality.py` check 3 re-derives both.

- **`P/core/analysis_runs/compatibility.py`: one conflict, resolved by rule (U9 decision 3).**
  - Main's PR1078 adds `_same_canonical`, with a body byte-identical to this PR's, and switches two copied-evidence comparisons to it.
  - The merge keeps one helper, the integration side's, and takes main's two call-site lines (blob `767da34027c5`).
  - The check requires main's side of the conflict to appear byte for byte inside the integration side.
- **`P/core/reporting/result_export/src/source_blocks.rs`: a clean merge.**
  - **Main's PR1080** walks source-blocks rows in receipt order.
  - **This PR's 32-bit bound** applies the `9_007_199_254_740_991` check to the u64 before `usize::try_from` (§3, After the cut).
  - The file is in S because of the 32-bit bound; at the cut it was main's version unchanged.
  - **PR1080 cannot run on D1:**
    - it is reached only via `for_source` branches that D1 never takes (I65's full Pass B; RV89, with 0 entries counted on the milestone and across the 71-input sweep);
    - and it changes no published byte on the F2a routes (G8's 324-output sweep is byte-identical, registered and Stale).
  - **The bound is identical on 64-bit by construction,** and is likewise unreachable on D1 (RV89).

## 3. What changed and why, unit by unit

Commits are on NUM and the U7 branch. Reviews are fresh and independent, and each repair was confirmed by the same reviewer.

### Before step 4: the accepted components (components of S at the cut)

- **The prepared private producer** (`922db9dce3`) passes the named case in both modes. Accepted after RV68.
- **Caller separation and the nonallocating census** (`24af17c470`). Accepted after RV72.
- **Typed preparation and proof evidence on failure prefixes** (`fc23cff95f`). Accepted after RV74.
- **The selected contracts:** corrected C3/F1 and wire completions 06–08 (RV69).
- **The kernel and certificate components** come from the reviewed component branch `codex/piping-f2a-work-exactness-20261002`.
- **The summary-coverage seam** (`c618675e84`, `fa225abded`). RV77: PASS, with its tests adopted. Merged at `f044127b1c`.
- **The three readers, the schema and corpus 07f.** Accepted after RV78–RV81 and six confirmation rounds (D1–D37). Fanned in at `c15e64b756`.
- **RV78's carried-artefact review** (the semantic table fixture and the YAML successor branch): PASS.

### U1 and U2: the receipt serializer, with the owner binding

- **Grant 1,** with U2's structural owner binding (RV77-N4): `59a5de2032`. **RV82: PASS.**
- **Grant 2,** closed translations, the unavailable form and U2's failure path: `b54caba7ab`. **RV82: PASS.**
- **Why:** the producer must emit the exact receipt the accepted readers validate, with custody bound to its owner.

### U3: facade capture behind the permit

| Grant | What | Commit | Review |
|---|---|---|---|
| 1 | The permit branch, unreachable | `bee3dc07ca` | **RV85: PASS** |
| 1b | Public carrier `RetainedPublication`, R-2's single unavailable notice, the linear permit | `4b31bbf23a` | **RV85: PASS** |
| 1c | Admission report kept, gate order, typed staging fallback | `886bef131a` | **RV85: PASS** |
| 1d | Test-only | `8abb5274a9` | — |
| 2 | The permitted path on the actual Direct entry | `664f8df7b7` | **RV93: PASS** |
| D-U6-5 | The carrier fixtures are the live successors | `f71478696b` | **RV93: PASS** (addendum, on `7f07a2f7b4`) |

**Why:** capture happens inside the one ordinary run, so the certified receipt describes what was actually published. Every fallback keeps the ordinary bytes.

### U4: memory profile, admission and registration (G2–G7)

| Step | What | Commit | Review |
|---|---|---|---|
| G2–G4 | Design | (records) | RV83: FAIL, repairs routed and confirmed. RV84: PASS and confirmed. RV87: PASS |
| G5 part 1 | Admission law, the D1 predicate, fail-closed build identity | `1e323058f3` | **RV89: PASS** |
| G5 part 2 | In-build profile, gate caps, budgets, stack witnesses | `cba3e9fda7` | **RV89: PASS** |
| G6 | Estimates closed, the identifier-class audit, the G-C solve fact | `2bb81ec1ea` | **RV89: PASS.** RV87: NOT CONFIRMED, then the class closed |
| G6 pre-registration repair | | `b43378d90a` | **RV89: PASS; RV87: CONFIRMED** |
| Registration | The dev/test identity: aarch64-apple-darwin, rustc 1.97.1 `8bab26f4f68e`, debug, opt-level 0, debug assertions, panic=unwind, no RUSTFLAGS. **M = 4,026,531,840 B, selected under D-7** | `0c7827b6ad` | (applied after the G6 reviews) |
| G7 | Re-qualification on the integrated basis; the T17_V4 line | `7f07a2f7b4` | **RV89: PASS** (Pass A and its addendum). RV87: PASS on the TEXT tool |
| Pass B | On the U7 head `cfda60403f` | — | **RV89: PASS** |

**Why:** no permit may rest on symbolic or partially priced terms.
- The admission law prices the invocation's maximum in the build: 0.8881 M sparse and 0.8929 M dense, within the 0.9 M margin.
- Every identity other than the registered one is Stale and keeps the ordinary route.

### U5: the independent reference

- **U5 is records only.** Against I50's exact-rational oracle, the milestone's successor agrees on 97 of 97 class claims in each mode. **RV86: PASS, with stated limits.**
- After U7, only the eligibility fields differ. The post-U7 report is the new baseline (RR "U7 slice L committed…").

### U6: carriers and standing

| Slice | What | Commit | Review |
|---|---|---|---|
| U6a | Rust dispatch, standing, derivative, Python entry | `844448112f` | **RV88: PASS** |
| U6b | Python carriers | `c89a7a986c` | **RV88: PASS** |
| U6c | Schemas | `cb03315779` | **RV88: PASS** |
| U6d | TypeScript carriers | `9555b6ffc2` | **RV88: PASS; RV91: PASS** |
| U6e | Reader round, snapshot 07g | `5e1e2625ac` | **RV90: PASS** |

**The repair rounds:**
- `cc4dd61d67` (07h; RV90 CONFIRMED);
- `968adb44fe` (RV91 CONFIRMED);
- `da274dd961` (RV88 confirmed every item but one, which was routed on);
- `76477534f6` (RV91 CONFIRMED);
- `6383e8e70e` and `b10ee5cf08` (post-U6f);
- `5f8d6291b8` (N-9).

**The whole of U6: RV92: PASS,** with its addendum PASS.

**Why:** the successor must travel with its receipt, and standing must come only from the verified receipt.

### U7: the reader eligibility switch

| Slice | What | Commit |
|---|---|---|
| T | TS standing bound to the live native capture; the explicit panel gates | `0ca5449c87` |
| P | Docs on `into_parts()` and `successor()` | `12a849a7bd` |
| F | The three completeness flags switched together | `cfda60403f` |
| Fix | TS summary counts Current only with eligible standing | `e5e1693ceb` |
| L | Live reruns in all three languages; corpus 07j closes the `not_required` gap | `ffe65ef203` |

**The repair round after RV94:**
- `fc575c7e56`: the stale "no permit" comments (U9 decision 6);
- `8c84e7ae14`: Python and Rust summaries count Current only with eligible standing (RV94 S-1);
- `e543c3d8f3`: D-U7-4's summary declared; N-2's docstring; N-3's G7 scope clause. This makes the corpus 07k (278 mutations).
- `2e03d7cc25`: code-line citations in comments and docstrings name symbols instead of stale line numbers (comment-only, line-neutral; `R/I61/u9_citations_01`).

**Reviews:**
- **RV94: PASS** on `ffe65ef203`, and **PASS** on its confirmation of the round (`R/REVIEW_RV94/u7_01/ADDENDUM_01.md`, 0/0/2). The citation rewording (`2e03d7cc25`) is covered by RV95.
- **RV89: PASS** on Pass B at `cfda60403f`.

### After the cut: six source changes (NUM, then carried to the PR)

| What | NUM | PR | Why |
|---|---|---|---|
| **The CI policy** | `c7bc3fd54e` | `fd3cbebb42` | `tools/ci/e2e_plan.py` gains `NUMERICAL_APP_INPUTS` = {`apps/desktop/src-tauri/src/lib.rs`}. PP's caller-separation guard `include_str!`s that file, so a change to it must select the numerical crate suite. Pinned in `tests/test_ci_e2e_plan.py`. Found by the first hosted run |
| **The 32-bit bound** | `7ff569a55c` | `92a5a9da1c` | `source_blocks::integer` checks the 2^53−1 bound on the u64 before `usize::try_from`. It is identical on 64-bit; on 32-bit an oversized value gets the same `SOURCE_BLOCKS_INTEGER` refusal. PP's runtime dependency on `result_export` now compiles it into the desktop wasm32 engines, where the old literal did not compile. Found by the second hosted run |
| **The U1 pin, scoped to the registered target** | `d069ab3ccf` | `6d8f8a82b2` | `retained_wire_tests::u1_ordinary_bytes_unchanged_under_capture` asserts the protected ordinary-bytes digest only where `ORDINARY_PINNED_TARGET` (aarch64 and macOS) holds. Dense ordinary binary64 bytes are target-dependent (Linux x86_64 differs). On every target it still asserts its claims: the Direct entry's ordinary envelope, and the captured run, equal the plain bytes. The successor pins are platform-stable and stay ungated. Found by the third hosted run |
| **RV95 N-3** | `c5adc16384` | `35d8ae59a7` | The policy pin asserts that `NUMERICAL_APP_INPUTS` is exactly the desktop source, so emptying the set fails |
| **RV95 S-1** | `bb3d766379` | `35d8ae59a7` | Six more stale pre-registration texts corrected, comment- and doc-only and line-neutral: `lib.rs`, `retained_memory.rs`, `retained_wire.rs` (two places), and the law, witness and challenge test headers (I65, `R/I65/u9_repair_01`) |
| **The K-D5/K2b module walker** | `42009dba72` | `6cfe50d368` | nonlinear_integration's source pins walk PP's module tree. The walker resolved `mod grant2;`, declared inside PP's test-only inline `retained_tests_hooks` module, as `src/grant2/mod.rs` and panicked; it now follows enclosing inline modules as rustc does. That exposed K2b's exact-site pin: the F2a capture split (`52842022cc`) moved PP's one `ForceScale::UNSCALED` from `solve_load_case` into `solve_load_case_observed`, and the pin now names that item, with the same token and count. Test-only, in a `#[cfg(test)]` module that PP's build never compiles. Found by DEC-025 on F and F's hosted run |

**Reviews of these commits:**
- **RV95** reviewed the first three as part of its complete review.
- **RV95's same-reviewer confirmation** of S-1 and S-2 ran on F; its confirmation of the walker repair runs on F′. Both are recorded in the post-merge record.
- **No Pass B rerun is needed for the CI-policy commit** (`not-d1`, RV89). Every other commit is unreachable on D1 or is comment- or test-only, and the frozen-head Pass B on F carries their reviewed entries (§7). The walker repair is a test file outside PP's compiled inputs, so Pass B's tree and delta inventory are rerun on F′ without a build, and RV89 confirms them.

**What eligibility means.** A supplied successor statement with its actual invocation stands `numerically_eligible`.
- **In TS,** it is eligible only while the live native capture holds (D-U7-4, a declared difference).
- **No producer-origin claim** follows from eligibility (D-U7-6).

## 4. Scope (U9 decision 1, D-U7-1)

**Public in this PR:**
- **The Direct retained entry** (`run_linear_static_preview_value_with_retained_direct`).
  - It publishes the successor only for D1 requests: one load case, no combinations, the preview family, no pressure, and counts within D1's caps (RV95 N-7).
  - Only in the registered dev/test build, under M = 4,026,531,840 B.
  - Within D1, a request whose W1 work falls back keeps the ordinary bytes, plus one notice once W1 work has run. Every request outside D1 keeps the ordinary bytes exactly.
- **The Headless entry** is refused at D1.0.
- **The three readers' eligibility** and the carriers, as in §3 (U7).

**Closed, and stated as closed:**

| Item | Detail |
|---|---|
| **Public activation** | No product caller (desktop, native app, headless CLI) publishes successors. The desktop calls only the ordinary wrapper (D-U7-1) |
| **Stale builds** | Every other build identity, including release builds and hosted Linux CI, keeps the ordinary route. Hosted CI therefore exercises only that route. The registered path is evidenced on the owner's Mac only |
| **A supported-machine statement of M** | Owner-held |
| **Wider F2a, U8, S-I, F2b, F3** | See §5 |

**The public-activation checklist** (RR "RV94 on U7…"). Activation requires at least:
1. **RV94 N-1:** review and tests for the Tauri rule-check backend accepting a successor with its invocation (`src-tauri/src/lib.rs` `qualify_rule_mechanics_with_context`).
2. **RV92 N-7:** memoize the binding before native activation.
3. **Native Current for successors.**
4. **T6's successor outputs,** replacing the explicit N-5 panel refusal deliberately.
5. **Confirmation that RV94 N-4 is resolved** (by the S-1 alignment).
6. **RV95 N-4: a 32-bit review** before any 32-bit consumer of the reader or of PP's retained path. The Rust reader has about 30 `as usize` index casts that would truncate on 32-bit. No 32-bit target reaches them today; the wasm32 engines import only PP's self-weight helpers, types and constants.
7. **A fresh review** of the activation itself.

## 5. Open obligations

1. **U8, the deferred witnesses:** the native Ceiling row, the L = 0 base, and RV93 N-5 (a real-input Candidate test).
2. **Wider F2a:**
   - the producer receipt and freeze transaction beyond D1;
   - resource and caller qualification for any new Direct caller, and re-qualification for any change to the D1 call graph (RR 10407, 10474; QUALIFICATION §11);
   - **RV95 N-6, re-qualification on any further identity.** Retained product values use the platform `hypot`, and dense ordinary bytes are target-dependent. Registering any further build identity (release, or another target) must therefore re-establish the milestone's bytes and verdicts on that identity;
   - multi-case invocations and combinations; preparation-only and mixed invocations; the promised exact routes;
   - the complete invocation's exact-block no-attempt rule;
   - registration of the release identity;
   - D38's pin, RV78-N1, and alignment of the N-3 G7 codes.
3. **S-I, F2b per domain, and F3.**
4. **Owner-held:** dense and lane ceilings; PHYS-R4 refusal and availability; observation framing; the KF3 lambda split; the KF2 dense screen; any supported-machine statement of M.
5. **T6:**
   - the successor outputs replacing the panel refusal;
   - the `results.schema.yaml` v0.3 dispatcher, which knows only precision-1;
   - RV95 N-5, a test of the 2^53−1 bound in `source_blocks::integer` (main's pre-existing gap).

## 6. Package and verification

| File | Purpose |
|---|---|
| `source_equality.py` | The five equality checks (U9 decision 2). Check 3 requires every S file main also changed to equal the recorded three-way merge; a conflict needs a recorded rule.<br>**At `6cfe50d368` against `42009dba72`: 5/5 PASS.** 138 paths identical; `compatibility.py` (one conflict, the rule holds) and `source_blocks.rs` (clean) equal their merges; the execution files are exactly this package.<br>**Negative controls fail as they should:** a stale PR head, a conflict without a rule, and a PR without the merge |
| `check_citations.py`, `citations.json` | Every record, design-document and code-line citation the PR adds resolves (U9 decision 5).<br>**At `6cfe50d368` against main** (unchanged from the cut). The NUM pin `cfcdb5997a` is on the pushed integration branch, and no cited record moved:<br>• **Totals:** 368 resolved, 0 ambiguous, 0 unresolved.<br>• **Records and RR:** 65 occurrences.<br>• **Design documents:** 289, against a table of 23 names. Anchors are verified in the pinned version; D1 resolves on main. `API.md` and `COMP` are decided by recorded context rules; COMP is ROOT's ruling.<br>• **Code lines:** 14, each pinned with its anchor text: 11 generated in the GENERATED PROFILE block and 3 in the hash-pinned corpus data. Every other bare `FILE:line` code citation is forbidden.<br>The 93 previously in comments now name their symbol. At `e543c3d8f3` itself the check fails with those 93 |
| `copies/g5_profile.py`, `copies/profile_tree.json` | The generator and input that regenerate `retained_memory.rs`'s GENERATED PROFILE block byte for byte, at the cut and again at `35d8ae59a7`: `python3 g5_profile.py profile_tree.json retained_memory.rs` |
| `copies/QUALIFICATION.md` | G6's qualification, the basis of M and the registered identity |
| `copies/G2_AMENDMENTS.md` | D-6's identity encoding (`build_identity.rs`) |

**Commands, run from a checkout:**
```
python3 source_equality.py --repo . --pr <PR head> --int <reviewed head> --main <main> --work <scratch dir> --package <package path>
python3 check_citations.py --repo . --base <main> --head <PR head>
```

## 7. Gates (U9 plan §1)

**Done before the freeze.** G5 and G6 transfer to `35d8ae59a7` by ROOT's carry-over ruling: since C2 the source changed only by comment- and doc-only, line-neutral PP edits and test files.

| Gate | Result | Record |
|---|---|---|
| **G1, RV95**, complete review of the PR diff, with the ledger | **PASS**: 0 BLOCKING, 2 SHOULD-FIX, 7 NOTE. All 99 source commits B..U map to a review. S-1 repaired (`35d8ae59a7`). S-2 is this regeneration | `R/REVIEW_RV95/u9_01/` |
| **G4, GEN-8** at the cut | 1 passed | RR "U9 cut…" |
| **G5, product T9** (M against C `6b9bb19a5f`, then C2 `92a5a9da1c`) | 112/112 identical. Exactly 2 added outputs, the milestone request in both modes, each equal to the Stale value-route bytes. Extra corpus 16/16. C2 reproduces C | `R/I61/u9_g5g6_01/t9/` |
| **G6, both-entry, part 1** (M against C and C2; 884 runs per side) | gate_check PASS on every side: 764 evaluated, 332 trusted, 0 trusted breaches. 884/884 identical; 0 heap-cap aborts; 0 timeouts | `R/I61/u9_g5g6_01/gate/part1/` |
| **G6, both-entry, part 2** (M against C2) | All 4 dense runs ended in 45–53 s (limit 1,800 s), with identical envelopes. A clean execution on a quiet host ran 45.8–46.2 s | `R/I61/u9_g5g6_01/gate/` |
| **G7, src-tauri suite** (ROOT) | M 116, C 116, with outcomes identical | RR "U9 gates G7 and G8 pass…" |
| **G8, pressure, coexistence, the sweep and callers** (C) | Pressure refused at D1.5 with exact bytes; the milestone publishes its successor. n05/n06 give `Coexistence` with exact bytes. The 324-output sweep is byte-identical (registered `9a74ff16…`, Stale `0e2db8b8…`). No product caller | `R/I61/u9_g8_01/` |
| **G9a, full Pass B** (I65, on `6b9bb19a5f`) | Exit 6, on the six added tests only. Entry byte-identical to `0c7827b6ad`'s; maxima 0.8881 / 0.8929 M; law 42/0. **RV89: PASS** (0/0/1), including the classification of PR1080 and the 32-bit bound as unreachable on D1 | `R/I65/u4_g7_05/`, `R/REVIEW_RV89/u4_g7_03/` |
| **Source equality and citations** (`35d8ae59a7`, then `6cfe50d368`) | 5/5 PASS; 368/0/0 PASS on each | `R/I61/u9_freeze_01/`; the post-merge record |
| **DEC-025 and hosted CI on the first freeze F** (`20dd3d929d`) | nonlinear_integration: 132 passed, 2 failed on the Mac, against main's 134; hosted CI stopped at the same crate. Every other manifest equals main's or adds named tests. Repaired after the freeze (§3) | RR "DEC-025 on F finds…" |
| **Hosted CI before the freeze** | Three runs found three defects, all fixed after the cut (§3) | RR "PR #1082's first/second/third hosted run…" |

**Run on the frozen head, and recorded in the post-merge record `IMPLEMENTATION/F2A_D1_MERGE/` on NUM:**
- hosted CI on F′ and the full-SHA dispatch (`target_base` = main);
- GEN-8 on F′;
- the Mac baseline on main (main is unmoved, so it stands) and DEC-025 on F′;
- the native witness (the ordinary route and the panel gates);
- the frozen-head Pass B on F (I65, with its prepared reviewed entries), confirmed by RV89, and its tree and delta inventory on F′;
- RV95's same-reviewer confirmation of S-1, S-2, the walker repair and the gate evidence.

**Acceptance runs on the reviewed source:**

| Suite | Result | Source |
|---|---|---|
| PP, registered and Stale | 705 passed, 1 failed (Mac `t13`), 10 ignored | RV95 |
| result_export | 172 | RV95 |
| runner/headless | 85 passed, 2 failed (known) | RV95 |
| vitest | 3,552; tsc clean | RV95 |
| Python, 24-file sweep plus retained suites | 1,848 | U7 repair round |
| Python retained | 463 | U7 repair round |
| PP fixture sweeps | Byte-identical: registered `9a74ff16…`, Stale `0e2db8b8…` | RV94, G8 |
| The switch's allocations on D1 | Identical in all 12 pairs | RV89 |
