# U8: change record

**This PR brings U8 to main: the test-only witnesses that the F2a D1 milestone left open.**
- **Real-input fallbacks.** RV93 N-5's Candidate and Preparation fallbacks, and W-C1's Native fallback, are reached from real D1 inputs on the actual Direct entry, with no fault hook.
- **The L = 0 base.** A producer-solved successor with a memberless body is pinned, and copied into two fixtures under D-U6-5.
- **Corpus 07l.** The fixtures enter the shared corpus as producer-solved bases, with 8 mutations and 4 must-pass entries. Python, Rust and TypeScript read them identically.
- **Nothing else changes.** There is no production, reader `src`, schema or build-identity change, and no product caller.

U8 is an intermediate F2a PR. It goes to main on its own, as I61's plan decision 7 rules (RR "I61's U8 plan ruled…"). The reviews are agent reviews, not personal review by the owner.

**Status of this file.** I77 drafted it, records only, while the full suite and RV98 ran. ROOT completed it at the cut: the full suite and RV98 are in. The PR-head gates run before the merge, and are recorded in the merge record `IMPLEMENTATION/U8_MERGE/` on the integration branch.
- **The U8 head** is `bd6b4be2c3` on `codex/piping-f2a-u8-20261005`. It sits on NUM `b1e2d7741e`, whose maintained source equals main `c1bfc460fc`.
- **The PR branch** is cut by ROOT from main. It carries the 7 files from `bd6b4be2c3` and this package.
- **Main has moved since.** The local `origin/main` is `c1571f7feb` (#1098 and #1099). Its 209 changed files are all under `projects/chirality-app-v4/`: no piping path, and none of the 7 files. NUM absorbed #1098 at `10df4a37ea`.

**Notation:**
- **P** = `projects/chirality-piping`; **PP** = `P/core/product_physics`; **RE** = `P/core/reporting/result_export`; **DT** = `P/apps/desktop/src`.
- **RR** = T3's `ROOT_RULINGS_V1.md`, which is append-only; an RR "title" names a ruling heading.
- **R/…** paths are T3's `RESUME_2026-09-30` records on NUM.
- **Citations in the 7 files resolve through `citations.json`,** pinned at NUM `fd3990a710` (§6).

## 1. What the PR contains

**Maintained source: 7 files under P.** 2 are added and 5 modified, +29,026 / −14 lines, 6,372,777 B at the head. The 5 modified files have the same base blobs on main `c1bfc460fc` and on `b1e2d7741e`, and main's later commits through `c1571f7feb` touch none of the 7.

| File | Change | Purpose | sha256 at `bd6b4be2c3` |
|---|---|---|---|
| `PP/src/retained_facade_tests.rs` | +244 / −0, a new "U8 (I68)" section after the last test | Three tests: the real-input fallbacks, the L = 0 successor with its value controls, and D-U6-5 for the L = 0 fixtures. The inputs are built inline from the milestone request | `2a1229b5a860489ee3616eb180a8f65cd1e1c6fc1048461b55c22ccf4e3316c1` |
| `P/fixtures/results/retained_precision_l0_successor_sparse_interactive.json` | new, 226,746 B | The L = 0 successor document (id `u8_l0_isolated_node_sparse_interactive`), as PP's test writes it | `93c6c86548b9d263cba9d9869010043d23ed1c9f2f304dd9f82eb705eb350876` |
| `P/fixtures/results/retained_precision_l0_successor_dense_scrutiny.json` | new, 228,096 B | The same, dense | `dbb3d477364248fb9ae15f7b9cff44410c2bffe45f783dd02d96eca663b0ac88` |
| `P/fixtures/results/retained_precision_cases.json` | +16,139 / −1 (07k → 07l) | The shared corpus. The two producer-solved bases, 8 mutations and 4 must-pass entries are appended. The one deleted line is the old top-level claim | `5ac13296c69745ae837e41b93c42e4daa0c4100dc863229f8e6a4cdbe2692ccd` |
| `P/tests/test_retained_precision_contract.py` | +68 / −7 | Python's count pins and the appended slices; a new D-U6-5 test for the embedded bases | `729405c6352a2f6ac83ef04eddf0bbdcac9c91a10cd4e0b2e9c0aec8036bd794` |
| `RE/tests/retained_precision_contract.rs` | +23 / −6 | Rust's two count pins; a new slice, `snapshot_07l_mutation_outcomes`; stale comments | `7832a6024c8c460bd02d145ec490fbe5ee798e118ab14c6163fd919030d6d5d0` |
| `DT/features/results/retainedPrecision.test.ts` | +45 / −0 | TS's count pins and the appended entries, written in the test rather than read from the corpus | `59531ed1ecf9aff190d9501a0eb120113380e36a2c37a25effd88f007629c287` |

**The commits,** on the U8 branch:
- `d449097085`: U8-1, the PP tests and the fixtures (I68);
- `69a925bd68`: U8-2, corpus 07l and the Python test (I69);
- `de01e43bc4`: U8-3, the Rust test (I70);
- `bd6b4be2c3`: U8-3, the TS test (I71).

**This package** has 4 files in `…/NUMERICAL_INTEGRITY_T3/IMPLEMENTATION/U8/`:
- this record;
- `PR_BODY.md`;
- `citations.json`;
- `SHA256SUMS`.

It reuses main's `IMPLEMENTATION/F2A_D1/source_equality.py` and `check_citations.py` (#1082) unchanged, without copies. U8's records stay on NUM.

## 2. The witnesses, and what each proves

| Witness | Test | What it proves |
|---|---|---|
| **RV93 N-5's real-input fallbacks** | `u8_real_input_fallbacks_append_one_notice` (`first_load_only`, `tiny_spring`) | The Candidate and Preparation fallbacks are reached from real D1 inputs, not only from hooks.<br>• **`first_load_only`:** the milestone with its first load alone ends at Candidate. The product certificate refuses five torsional-shear rows (`SharperExact`).<br>• **`tiny_spring`:** support 1's stiffness is set to 1e-300. It ends at Preparation, because the ordinary run does not solve and preparation requires `MECHANICS_SOLVED`.<br>**In the registered build, both modes:** admitted inside D1; the expected cause; one ordinary run that reaches G-C once; the published bytes are the plain bytes plus exactly one N1 notice (the byte assertion comes before the count); no hook armed before or after.<br>**In any other build:** the plain bytes, from one run |
| **W-C1, Native with Ceiling** | the same test, input `u8_two_body_case_b` | Two-body case B, from RR "I68's probe verified…", reaches the Native fallback from a real input, with one notice. The kernel ends `Unresolved(Ceiling)`: p128 and p256 stop at the stop rule, p512 at the charge, and p1024's verification solves. The U8-0 probe records the reason. By decision 3 the test does not assert it |
| **The L = 0 producer-solved successor** | `u8_l0_isolated_node_publishes_pinned_successor` | The milestone plus node N2 at (3, 0, 0), which no member references, with `rigid:N2` restraining all six DOFs. Body 1 is a single node, extent 0. It is admitted inside D1 and publishes a successor in both modes through the Direct entry, in the registered build, with B′ holding.<br>**The pins:** document `93c6c865…` / `dbb3d477…`; receipt `c00cbe76…` / `0b4250c8…`; published bytes `9b425066…` / `5d84fce6…` (sparse / dense).<br>**The value controls:**<br>• **Body 1:** its 15 rows are exactly +0: 6 input-derived and 9 exact zeros. Its coverage is `has_data false, stop [F,F,F,F]`, and its scales are zero.<br>• **Body 0:** every milestone row is present and bit-identical, with the milestone's class claim, and within U5's unchanged per-class criterion.<br>This proves the producer's handling of an extent-0 body (PLAN §1.2) |
| **D-U6-5 for the L = 0 fixtures** | `u8_d_u6_5_l0_fixtures_are_the_live_successors` | Each fixture is byte-identical to the live successor document: from the Direct entry in the registered build, and from the private driver in any other build. Each carries the pinned hashes |
| **07l parity in three readers** | the Python, Rust and TS contract tests | Corpus 07k → 07l: (cases, mutations, must-pass) (15, 278, 24) → (17, 286, 28).<br>**The two bases** are case-level `producer_solved`. Their `source` and `invocation` equal the fixtures' JSON values exactly, which is D-U6-5 for an embedded base (RR "I69's corpus 07l committed; …", reading 1).<br>**The mutations,** all refused at G5a `RETAINED_PRECISION_SCALE_MISMATCH`, each in both modes:<br>• SNAPSHOT_05_PLAN §1.2's isolated rotation stop, `has_data` and coupled estimate;<br>• I69's translation-plus-rotation stop.<br>**The must-pass entries:** the uncoupled estimate and the translation stop.<br>**The claim** is decision 6's: "synthetic controls plus listed producer-solved bases; no native Current evidence". The top-level `kind` stays `synthetic_control`.<br>**Discrimination:** four reader mutants remove or couple the L = 0 rules: RM1 the A-exclusion, RM2 the feasibility rule's L = 0 branch, RM3 the uncoupled hats, RM4 the no-free-DOF rule.<br>• In Python and Rust, each is killed only by 07l entries.<br>• In TS, RM2–RM4 are killed only by 07l entries. RM1 is also killed by TS's existing `stopFeasible` unit test |

**Class counts on the bases,** in all three readers: 25 relative, 78 absolute, 9 input-derived and 1 non-quantity (sparse); 25 / 78 / 9 / 2 (dense).

## 3. What it does not do

- **No production, reader `src`, schema, build-identity or CI change.**
  - Pass B finds the registered entry byte-identical to `0c7827b6ad`'s, with M = 4,026,531,840 B.
  - The milestone still publishes its pinned successors (`ac6986b0…` / `6cd1d249…`).
- **No Ceiling receipt row.** That is W-C2, B1's acceptance witness together with D38's pin. U8 commits W-C2's two-body model only as case B's builder.
- **F-1 is routed, not pinned.** F-1 is the Rust precommit reader requiring a parity row that F1b legitimately omits at b ≠ 0.
  - B0 states the rule; B1 implements it in all three readers.
  - No U8 test runs two-body case A, and 07l adds no dense, range-scaled base.
- **No native Current evidence.**
  - No product caller publishes successors; activation stays with B8.
  - Hosted CI (the Stale build) exercises only the plain-route branches (RV97 N-2, accepted as planned).
  - The registered-build witnesses run on the owner's Mac: the full suite, DEC-025 and Pass B.

## 4. Reviews

- **RV97, round 1,** on `b1e2d7741e..d449097085` and the probe: **PASS,** 0 BLOCKING, 0 SHOULD-FIX, 5 NOTE (`R/REVIEW_RV97/u8_01/REVIEW.md`).
  - Every probe fact was re-derived two independent ways.
  - 29 of 29 mutants were killed.
  - Body 0 passes U5's oracle (STOPS `[]`).
  - **The rulings on the notes:** N-1 became erratum E-6; N-2 was accepted as planned; N-3 was recorded; N-4's U5 variant was accepted as the records-level replay; N-5 is met by the probe and committed by B1.
- **RV97, round 2,** on `d449097085..bd6b4be2c3`: **PASS,** 0 / 0 / 2 (`REVIEW_ROUND2.md`).
  - 348 documents went through the three readers' public entries, with 0 discrepancies.
  - D-U6-5 holds as ruled.
  - The provenance claims are true.
  - R2-N-1 extends E-6. R2-N-2 is routed to B1.
- **The two rounds together cover the complete diff** `b1e2d7741e..bd6b4be2c3`.
- **RV98** confirms I72's Pass B: **PASS** (0/0/2). It confirms the two L = 0 fixture rows as `not-d1`: test-only, embedded only by a `#[cfg(test)]` module.
- **RV97's confirmation** of the PR head's package and equality comes before the merge (the merge record).

## 5. Gates (RR "T3's gate set and Git rules, consolidated…"; decision 7)

| Gate | Result | Record |
|---|---|---|
| **Complete-diff review** | RV97 PASS in both rounds (§4) | `R/REVIEW_RV97/u8_01/` |
| **Pass B,** fresh and fail-closed (I72, on `bd6b4be2c3`) | `DELTAS TO READ`, exit 6: the same gate vector as F's. Every gate is 0 except `pp_outcomes`; against F, its only new outcomes are U8's three tests, all `ok`.<br>• **Unchanged:** the entry; maxima 0.8881 / 0.8929 M; TEXT D 14,734; the 11 reviewed entries; every witness and challenge line.<br>• **The delta:** 10 rows added over F′. 8 are main's documentation (`not-d1`). 2 are the L = 0 fixtures, which the tool labels `unreachable`; I72 reads them as test rows, since their embedder is a `#[cfg(test)]` module file | `R/I72/u8_passb_01/` |
| **Pass B's confirmation (RV98)** | **PASS** (0/0/2). The fixture rows are `not-d1` (ROOT's ruling, on RV98's recommendation) | `R/REVIEW_RV98/u8_passb_01/` |
| **G5–G8** (T9, both-entry, src-tauri, the controls) | Carried by ruling (decision 7). U8 changes no production byte | RR "I61's U8 plan ruled…" |
| **The full 40-manifest suite** before the freeze | **PASS** on `bd6b4be2c3`. 38 of 40 manifests are identical to F′'s. PP +3 and result_export +1 are the added tests, all ok; the known `t13` fails on both sides | `IMPLEMENTATION/U8_GATES/full_suite/` |
| **`source_equality.py`** (the PR head against NUM with U8 merged) | Run on the PR head before the merge (the merge record). I77's pre-check at the U8 head (PR = int = `bd6b4be2c3`, main `c1bfc460fc`): \|S\| = 7; checks 1–3 and 5 PASS; check 4 needs the package. #1082's `compatibility.py` rule is reported as not needed | `R/I77/u8_package_01/` |
| **`check_citations.py`** | **PASS** at `bd6b4be2c3` against `b1e2d7741e`, and against main `c1bfc460fc`: 10 resolved, 0 ambiguous, 0 unresolved (§6). Rerun on the PR head before the merge (the merge record) | `R/I77/u8_package_01/` |
| **GEN-8** on the exact head (E-4's method) | Before the merge | `IMPLEMENTATION/U8_MERGE/` |
| **Hosted CI** on the PR, and the full-SHA dispatch (`piping-desktop-e2e.yml`, `target_base` = main) | Before the merge | `IMPLEMENTATION/U8_MERGE/` |
| **The exact-head Mac DEC-025** against a fresh baseline of current main (`run_dec025.sh`, `compare_suites.py`, counted at `ALL-DONE`) | Before the merge | `IMPLEMENTATION/U8_MERGE/` |
| **Native witness** | Not applicable: U8 adds no product behaviour. G10 stays outstanding by the owner's decision | RR "Owner decision: G10 is redefined…" |

**Acceptance runs on the U8 head,** by the implementers and RV97. The Mac failures are the known ones.

| Suite | Result | Source |
|---|---|---|
| PP, registered, all targets | 708 passed, 1 failed (Mac `t13`), 10 ignored. Base: 705 / 1 / 10 | I68, RV97, I72 |
| PP, Stale | 708 / 1 / 10; the three U8 tests pass through their unregistered branches | I68 |
| result_export | 173 (base 172) | I70, RV97 |
| runner/headless | 85 passed, 2 failed (the known `load_reference` pair) | I68, I72 |
| Python | The retained suites: 480 (base 463). The 24-file sweep: 1,873 passed, 30 skipped (base 1,856) | I69, RV97 |
| TS | `retainedPrecision.test.ts`: 474. The desktop suite: 3,574 (base 3,552), with all 22 additions in that file. `tsc` clean | I71, RV97 |
| Mutants | PP: 6 / 6 (I68) and 29 / 29 (RV97). Reader mutants RM1–RM4 are killed by 07l entries in Python, Rust and TS, and by nothing older except TS's `stopFeasible` unit test for RM1. Corpus and test mutants are killed (I69 6 / 6; I70 TM-A; I71 TM1–TM4) | I68–I71, RV97 |

## 6. Citations

**`citations.json`** follows #1082's index format, pinned at NUM `fd3990a710`. It carries #1082's 23 document names unchanged as detection vocabulary, and adds PLAN and SNAPSHOT_05_PLAN. With #1082's names alone, the tool finds no document citation in U8's added lines.

**Checked by main's `check_citations.py`:**
- **Records and RR:** `R/I68/u8_probe_01` (twice), RR "I61's U8 plan ruled…" (PP and Python) and RR "I68's probe verified…".
- **Documents:** PLAN §1.2 (twice) and §1.3; SNAPSHOT_05_PLAN §1.2. Both documents resolve on main.
  - PLAN is a bare name with 16 candidate files in T3's records. It resolves by a context rule: in PP's U8 section it means `R/I61/u8_plan_01/PLAN.md`.
  - Without the rule, the tool lists the three citations as ambiguous.
- **One code line:** `zz_rv93.rs:292–315` is pinned as a code anchor. It is a record file, RV93's probe module, at the NUM pin, and the anchor records five lines' text.

**Outside the tool's classes,** and listed under `named_references`:
- the review findings RV93 N-5, RV94 N-3, C04 and RV90 S1, N1, N2 and N4;
- the decisions D-U6-5, decision 6, W-C1 and W-C2, D-U7-2, D11 and D37;
- the attributions to I68 and I69;
- the § anchor of `R/I68/u8_probe_01 §3`;
- the U5 criterion;
- the `u8_head` commit;
- one wrapped code-line citation.

Of these, RV94 N-3, D-U7-2, RV90's items, C04, D11 and D37 are carried in modified lines from main's text. I77's `verify_named.py` checks all 15 entries at the pin: 63 checks, 0 failed (`R/I77/u8_package_01/`).

**Negative controls fail as they should:**
- PLAN without its rule is listed as ambiguous;
- removing the code anchor, or changing one anchored line's text, gives UNRESOLVED;
- a wrong RR heading line gives FAILED;
- a missing RR entry gives UNRESOLVED;
- a pin without U8's records stops with an error.

**Commands, from a checkout that holds NUM's objects.** `--index` must be given, or the tool reads #1082's index.
```
python3 …/IMPLEMENTATION/F2A_D1/source_equality.py --repo . --pr <PR head> --int <NUM head with U8> --main <main> --work <scratch dir> --package …/IMPLEMENTATION/U8
python3 …/IMPLEMENTATION/F2A_D1/check_citations.py --repo . --base <main> --head <PR head> --index …/IMPLEMENTATION/U8/citations.json
```

## 7. Open obligations carried forward

1. **W-C2 and F-1 → B1.**
   - W-C2 is the receipt's Ceiling row, on the two-body pair as a two-case invocation, with D38's pin. It also commits pair case A as W-C1's control (RV97 N-5).
   - F-1: B0 states the parity-row rule, aligned to OQ5; B1 implements it in all three readers, with a shared corpus case and a mutation, re-qualified once together with D38's reader change.
2. **RV97 R2-N-2 → B1.** The Rust contract test's module doc, "These do not establish execution", understates, since the two bases were published by an actual execution. It is reworded at B1's alignment.
3. **Mutation 277's slice → B6.** `g7_not_required_quality_enum_invalid` (07k) is in no slice tally. It needs a one-entry slice `277..278` (I70's item 2).
4. **E-6.** `R/I68/u8_probe_01/PROBE.md` §7 step 9 and §8, and `R/I69/u8_corpus_07l_01/RETURN.md` §4, say that the TS reader does not use the copied wasm assets. It does, in G8's unit conversion and canonical hashing.
   - The records are sealed, and the erratum stands in RR.
   - The assets' build revision `e2b83da584` is inferred (I71; corroborated by RV97). B7 builds its own assets.
5. **From this package, for ROOT's ruling:**
   - the record-file code anchor;
   - the wrapped citation `retained_memory_witness_tests.rs :181–199`, which the tool does not see. It is accurate today and could name `w6_input()` at the file's next touch;
   - the `u8_head` commit, which resolves on the U8 branch and, after ROOT's merge, on NUM, but not on main.
