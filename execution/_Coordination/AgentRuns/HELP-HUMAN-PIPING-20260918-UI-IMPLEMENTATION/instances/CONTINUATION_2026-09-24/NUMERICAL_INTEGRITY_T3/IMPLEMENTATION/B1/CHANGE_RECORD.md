# PR-B1: change record

**This PR brings B1 to main: up to three load cases (C = 3) in one retained-precision invocation, at I82's option S3, through the retained route.** It is B1's compact PR under PLAN_v2 §6 (`R/I84/b1_plan_01/PLAN_v2.md`, `c85786b7…`). The reviews are agent reviews, not personal review by the owner.

**Status of this file.** I108 (SK) drafted it, records only, from the records named here. The PR-head gates run before the merge and are recorded in the merge record on the integration branch.
- **The code commit** is `7f5f72912e` on `codex/piping-t3-pr-b1-20261008` (#1154), cut from main `953d8c9446`. It carries the 33 maintained files that NUM `31eed8497f` changes (RR "RV124 confirms B1's Pass B; the platform test fix committed on NUM").
- **NUM carries B1:** `b1` `ddc8eaaf54` (R6b's registration) merged as `fd62a11046`, then main `6c821d9ccf` as `a41eea7b5f`. Two later commits change maintained source:
  - `75cd6be76b` corrects the `threshold_bytes` comment, which now cites RR "R6b: RV124 passes SQ and confirms M". That is a two-line comment change; `retained_memory.rs` is not a reviewed input;
  - `31eed8497f` carries the platform test fix (§1.1).
- **Main:** NUM's main base is `6c821d9ccf`. The PR's main, `953d8c9446`, adds #1153 and #1155 (App v4 only: 209 paths, none under P), so it touches none of the 33.

**Notation:** P = `projects/chirality-piping`; PP = `P/core/product_physics`; RE = `P/core/reporting/result_export`; DT = `P/apps/desktop/src/features/results`; AR = `P/core/analysis_runs`; RR = T3's append-only `ROOT_RULINGS_V1.md` (an RR "title" names a heading); R = T3's `RESUME_2026-09-30` records on NUM; T = T3's folder on NUM. DESIGN = `R/I78/b0_contract_01/DESIGN_v2.md` (`5933b90b…`); QUAL = main's `IMPLEMENTATION/F2A_D1/copies/QUALIFICATION.md`.

## 1. What the PR contains

**33 files under P:** 3 added and 30 modified, +329,970 / −947 lines, 22,010,200 B at the head. sha256 from the Git objects at `7f5f72912e`.

| File | +/− | Change | sha256 |
|---|---|---|---|
| `DT/previewPhysicsEvidence.test.ts` | +33 / −0 | Tests of the extrema-number demand (I4 ruling 3) | `5bce45a90922e43ff0bdfef8f9a6cb87b5ae27a17a1756b7ea39825d4e4196b4` |
| `DT/previewPhysicsEvidence.ts` | +4 / −1 | TS's raw and transport reads demand number-valued `global_upper_bound_pa` and `certified_gap_pa` (I4 rulings 2, 3) | `fb5b6277c6292caee5e619b197be79ac88bd5f7e02ff6e312bbec69ef6d33993` |
| `DT/retainedPrecision.test.ts` | +440 / −5 | SR-TS's n-case tests, its repair pins, and 07n's TS harness pins | `e81f07774e2daff2f50edbcc1b138e5edc9984935dd5b30d0d29e67239d81092` |
| `DT/retainedPrecision.ts` | +90 / −18 | TS retained reader at n cases: D38 (4b), F-1 text B per case, the alignment set; RV108 N4, N6 | `40bfdd4e15a7cc4cb24ab27ff5ee302c6de523b5261147500d9c7b9b573ff515` |
| `DT/retainedPrecisionIntegration.test.tsx` | +1 / −1 | The carrier scope sentence's pin (RV108 N6(b)); outside T6S's panel block | `3b3ad7bc5411f44b935cfd10aad69db3ebfb8e691f8e90416cbf49398b61eccf` |
| `AR/compatibility.py` | +24 / −8 | `_source_contract`: string guards on the enum tests (RV108 N1, ruled in); Rust's header order on the retained transport step only (I4 ruling 1) | `d2eb8465b6f0715e9ea5e2f82e1e961a618e5c5688c514c5f04ad1e8f2c94c2c` |
| `AR/preview_physics_evidence.py` | +15 / −4 | On transport, withheld records compared as multisets (I4 ruling 2) | `0d92228583d0044749700b2e8c35dd0b005d0003ec1ff191fd9d4ce64133d566` |
| `AR/retained_precision.py` | +163 / −19 | PY retained reader at n cases: D38 (4b), F-1 text B, G8's invocation and material-basis checks, the alignment set, header at G2, C2's kernel row; RV108 N2 | `75e12637685816f0a7a44ae4ad2e18191a38147c89b3fd10cea5f7c58b85580c` |
| `PP/src/lib.rs` | +240 / −59 | `retained_w1`: T-4's classifier (decisions 1 and 21, `NoTriggeredCase`), the SA–SP seam, the n-case custody and notices; test hooks | `f77e2273860f946539dd2a8580902fd1188a9d99158bcadbf0d8ddfb1c885b09` |
| `PP/src/retained_facade_tests.rs` | +1,346 / −37 | ST's and SP's tests: classifier, re-basings, W-C2 and SF-2 pins, N-16, ordinal pin, fault tests; I77's two line citations as symbols; the glibc variants (§1.1) | `23ec8dea0d6daa36748d01d835215d51dbb0cda65110833b277188c5d1f53109` |
| `PP/src/retained_memory.rs` | +188 / −110 | Admission at S3 (SA); the parked-slot error text; the regenerated GENERATED PROFILE block; G-B's SF-1 bound; `threshold_bytes` = 11,274,289,152 | `bb9f4eb1cacc891f54f148ab97929e26db7976145ccb72b4574a0cbfe16e3bea` |
| `PP/src/retained_memory_law_tests.rs` | +609 / −47 | SA's gate tests; SQ's re-pins at M and new law tests; `CAP_MAXIMAL_RING` (§1.1) | `ea192e51da70624973a58c4a7b757d434616367d4f088c6a3158cf01f36886b8` |
| `PP/src/retained_memory_witness_tests.rs` | +385 / −89 | The S1 witnesses, one entry point per mode, with B1's inputs and DEF-O reports | `82cd49b20261bdeeeb046c14d22ca46463a834edecb9a1a9ef1a4920bd0be248` |
| `PP/src/retained_product.rs` | +963 / −273 | The n-case transaction, DESIGN T-2 to T-13 | `cac6e68d379fef76976491677a310a6dcd4d1e80a2f4bdd335025a37d782b24e` |
| `PP/src/retained_product_tests.rs` | +43 / −31 | Adapted to the per-case structures | `97e0f7fa4d0f11c2207af04f53bc2f23dc284383c7b34c620a94b67c06ce5a96` |
| `PP/src/retained_receipt.rs` | +3 / −3 | Reads the capture's native outcome as the case's own | `621cf9d17608c1f8fbc28d719ef318d8013e5227b31d9864a67bfcb632a37435` |
| `PP/src/retained_tests_hooks/grant2.rs` | +34 / −6 | Per-case fault hooks (`fail_preparation_of_case`) | `1f849c75b1c9cad0f124ae7465c063ef7039ae147b7e506e4a357dd53d76781a` |
| `PP/src/retained_wire.rs` | +388 / −61 | The n-case serializer and receipt; `constructor_ordinal` is the authored ordinal | `f24f258f40281ded54b537a3acd68057b9c1662aae39a6db1689f6055a3c4ac1` |
| `PP/src/retained_wire_tests.rs` | +6 / −6 | Reads `native_pair()` | `1f0dd77b68d4093eb14b1dd8772d839dbc79299ceb5b1c6cc0bb05881fc24e90` |
| `PP/tests/common/b1_sq_inputs.rs` | new, 381 lines | I86's inputs for the witnesses and the challenge, pinned by I86's hashes; the ring as bits (§1.1) | `acc37104f6aca3c76c775f78e78c90109dc4915e01835027ad70f8be67f54df5` |
| `PP/tests/retained_memory_challenge.rs` | +137 / −82 | The challenge at C = 3: `CAP_BYTES` 16 GiB; the bound by furthest phase (A1-S-1); per-input, per-mode entries | `d330eb15d05b5a190ce71e8d70510d62de156a5182de364037b2dbd2ca3cf72a` |
| `PP/tests/s11f_site_test.rs` | +4 / −0 | `retained_product.rs` joins rule 8, with its two integer sites (E-12) | `17ffdb42d66c4939041fe35129a16bc7c2810de536e453ae8cafd5d3a209ac19` |
| `RE/src/retained_precision.rs` | +481 / −37 | RS retained reader at n cases: D38 (4b), F-1 text B, G5 `not_required`, the alignment set, the transport metadata check | `e175d4149f926a06b78312a8bb26965aaf5f683ac678d48e73ab5f4c0aa207e8` |
| `RE/src/source_blocks.rs` | +126 / −0 | RV95 N-5's direct `#[cfg(test)]` test, appended; no production line | `e388416b2619a2ab1e51c10467bba06140c2c254f39bc8d03650ff860a76a13e` |
| `RE/tests/retained_precision_carriers.rs` | +2 / −2 | The scope sentence's pin (RV108 N6(b)) | `a7e88aca33ecbdc5ca74b36d049b6a01ed9078b69e7ad75994f4b5a58678bcb3` |
| `RE/tests/retained_precision_contract.rs` | +2,484 / −29 | SR-RS's n-case tests, repair pins, RV113's twelve rows, 07n's RS pins; the module doc reworded (RV97 R2-N-2) | `6b56505128528730def95611fa0f2a3b638dd4aa52784e0bdec84bd6e696f904` |
| `P/core/runner/headless/tests/retained_precision_admission.rs` | +16 / −6 | The out-of-domain oracle is C + 1 = 4 load cases (RV107 SF-2) | `73da3a859e04fd2859d28d808aaf10b3f0db4f66978215a0dc028a149f3afd88` |
| `P/fixtures/results/retained_precision_carrier_cases.json` | +1 / −1 | The transport scope sentence (RV108 N6(b)) | `16a73775a014beea4d647abc124cd6d2179ffd9d3a30c58a3842fd178eb5ec71` |
| `P/fixtures/results/retained_precision_cases.json` | +283,214 / −0 | Corpus 07n appended to 07m | `ea113e7b25b96cbabcbd3fe43a8604da38a67e681f8a5ab3ad490eb37313e283` |
| `P/fixtures/results/retained_precision_w_c2_successor_dense_scrutiny.json` | new, 729,615 B | W-C2's successor, dense, as PP's Direct test writes it | `f2800bd4f2b4c90217918a6e1287f98305a1b6b07893295b5790f387c075d3a3` |
| `P/fixtures/results/retained_precision_w_c2_successor_sparse_interactive.json` | new, 729,145 B | The same, sparse | `7922e3e5278d0d87dc5faf79dfbc1f2a384899e97df306cc742355cdacdb6269` |
| `P/tests/test_retained_precision_carriers.py` | +118 / −1 | SR-PY's carrier tests (RV108 N1, N2), header pins, the scope sentence's pin | `ff61a071510695fa1820e6ce9df5800366d0fefe73c4851e954e20c598355396` |
| `P/tests/test_retained_precision_contract.py` | +865 / −11 | SR-PY's n-case tests, repair pins, 07n's PY pins | `29e69391eef16ea97ffcb18a5bf3c6a9fa2341a4613a74fa1d868e0468be87a2` |

### 1.1 The platform test fix

#1154's hosted CI failed on Linux in 4 `product_physics` tests that pass on the Mac (RR "I107's Pass B (no stop); PR-B1's hosted CI fails on Linux: two platform defects in B1's tests, diagnosed and fixed"). Both causes are in B1's tests, not its production code. The fix is three test files, +110 / −9, which RV124 read.
- **W2b's input pins.** `cap_maximal`'s ring used `f64::cos` and `f64::sin`, which differ between glibc and macOS in the last bit. The ring is now spelled as binary64 bits (`CAP_MAXIMAL_RING`), with a test that each entry is within one ulp of the platform's trigonometry. On the Mac the table equals libSystem's values bit for bit, so every input and pin is unchanged.
- **W-C2's dense document and (C, B, A)'s dense pin.** On Linux, one value differs (case C's `rigid:N0` support force magnitude, formed by the ordinary route's `f64::hypot`), and with it the two hashes over it. Exact glibc variants now apply under `cfg(all(target_os = "linux", target_env = "gnu"))`. Each platform must produce its own bytes exactly, with no tolerance, and the committed fixtures are unchanged.
- **Pass B's gates are unchanged** by it: `law` and `pp_outcomes` each gain the added test, and `delta` gains test-class rows only (RV124's confirmation).

**The slice history** is on `b1` (`codex/piping-t3-b1-20261007`) through `ddc8eaaf54`: ST, SP, SA, SR-RS, SR-PY, SR-TS, SC and SQ, integrated at I1–I4′ and R6b. The PR carries none of it.

## 2. What it does

**C = 3 at S3, through the retained route.**
- **Admission (SA):** `LOAD_CASES` = 3 and `TOTAL_LOADS` = 384 at D1's model caps (n = m = g = 32, Σr = 192, l = 128). D1.4 admits 1 to C cases with no combination or component; D1.5 and D1.7 hold per case; G-B and G-C are priced at C = 3. G-B's byte bound is T11 less one case's late capture (RV112 SF-1, at SQ).
- **The producer (ST, SP):** DESIGN T-1 to T-13.
  - **T-4:** a case whose ordinary verdict is `checks_passed` is not attempted (decision 1). Decision 21's exclusion also keeps out of A a case whose seed records a mechanism, asymmetric or invalid-input structural failure with no W2 publication. With no attempted case, `NoTriggeredCase` publishes the exact ordinary bytes, with no W1 work.
  - The attempted cases are prepared in request order, solved in one native batch call, frozen per case, staged and serialized in request order. On a fallback, one N1 notice is published per attempted case (T-12).
  - c = 1 runs through the same n-case path. Every committed c = 1 successor pin is byte-identical (RV109 rounds 1 and 2).
- **T-11's headline staging, a clarification of DESIGN.** At c ≥ 2, each summary headline is recomputed as the governing row over the staged rows: greatest value, then the smaller case id, then location (RR "SP returned before I3; the headline rule accepted; …").
- **A latent defect of main, repaired.** `retained_wire.rs` wrote `constructor_ordinal` as the term's canonical position, while all three readers derive the authored ordinal. An in-domain case whose loads were not authored in canonical order fell back at precommit G8 with one notice. That was fail-safe, but it denied a certified successor in main's registered dev/test build. The producer now writes `t.original`. No committed byte changes, because every pin was authored in canonical order (RR "I98's B2-W verified; …", ruling 4; RV109 round 2 ADDENDUM_01).
- **The Direct entry, in the registered dev/test build only.** `threshold_bytes` = 11,274,289,152 B (M = 10.5 GiB; R6a, final at R6b).
  - In-build E_mov,max + R is 9,800,676,166 B sparse (0.8693 M) and 9,859,807,510 B dense (0.8745 M), W3 binding.
  - The text-error budget is 8.05 % sparse and 6.67 % dense.
  - Every other build is Stale and publishes the ordinary bytes (SG's Stale sweep is byte-identical to base).
- **The witnesses publish.** W-C2 (A selected, B `not_required`, C unavailable at Ceiling) publishes in both modes, pinned as the two new fixtures. So does the cap-maximal three-case input (|A| = 3, Σ l_i = 384; QUAL_B1 §4).

**The three readers' eligibility.** RS, PY and TS read an n-case successor and agree on every gate and code, apart from declared differences: the raw G7 codes, and RV120 A-N2's two Python-only transport differences (PY refuses an array over 16,384 items; PY admits an unsafe integral `span_index`, which RS refuses at G7 and TS at G1).
- **SR's alignment:** D38's (4b) relaxation; F-1 text B per case; G5's `not_required` rule; G8 per case.
- **The three-reader alignment set** (RR "RV113's three returns verified; …"): the (f) family at G3; the (g) model-scope members at G8; C2's cause table at G5; the transport header at G2 and the metadata at G7.
- **I4′** (RR "I4 made at `30f3d1b24a`; …", rulings 1–5): Rust's header order in PY, the extrema-number demand, the multiset rule, and C2's kernel row.
- **Four base-reader edits are ruled in:** PY's `_source_contract` guards; PY's header order (retained transport only); PY's multiset rule; TS's extrema-number demand. Each refuses only shapes the producer cannot emit; all 126 extrema records in the committed fixtures carry numbers (RV120 `rvr_i4p_01`).

**07n.** The corpus grows to 26 cases (+9), 534 mutations (+240) and 78 must-pass entries (+50); the 07m prefix is unchanged.

**Also:**
- RV95 N-5's direct unit test of `source_blocks::integer`'s 2^53−1 bound (I74 decision 9);
- RV97 R2-N-2's module doc;
- I77's two code-line citations as symbols (`zz_rv93_input_fallbacks`, `w6_input()`);
- E-12: rule 8 now scans `retained_product.rs`.

## 3. What it does not do

- **No product caller** of the retained entries (SG gate 4). The native commands are unchanged.
- **No public activation:** that is B8's, and successors are not public before it.
- **No supported-machine statement.** That is owner-held. R9's reading is in RR "SQ complete; R9 read; …", and RSS_TIME §5 states the non-claims: no 16 GB behaviour observed, no concurrency claim, and M is not an RSS bound.
- **Not B2, B3, S-I2, F2b or F3.** D1.4 still refuses a combination or a component, and D1.5 still refuses pressure regions (SG gates 1 and 3).
- **No FK, schema, `Cargo.lock`, reviewed-input or D1-visibility change.** None of the 33 files is FK, a schema, `Cargo.lock` or a reviewed static.
- **T6S is untouched:** one T6S test file gains one line outside T6S's block. The c = 1 successors are unchanged, so T6S-2's goldens stand (PLAN_v2 §3.12).
- **The disclosed limit (F-1 text B, with RV113 N-4's wording).** A dense case's parity row deleted, with the receipt resealed including its row indices, is admitted and eligible.
- **R-b′:** the whole invocation is abandoned on an unrecoverable fault. This is a known limit (DESIGN §7, RV105 N-5).
- **About 30 KB of B1's heap is unpriced** (RV124 Q-N1: `Vec<CaseAttempt>`, 28,200 B at |A| = 3, and small O(c) locals). That is against 287,052,726 B of dense margin, and M is unaffected. Pass B found no other unpriced owner (I107).

## 4. Review

| Review | Scope | Result | Repairs |
|---|---|---|---|
| RV107 (`R/REVIEW_RV107/b1_plan_01/`) | PLAN and PLAN_v2 | REVIEW 0/7/16; ADDENDUM_01 not confirmed as written, 0/2/11 | R1's seven amendments and A1's (RR "B1's PLAN_v2 accepted; …") |
| RV109 RV-P round 1 (`rvp_round1_01/`) | ST | PASS 0/1/5 (`707045b9…`) | SF-1 repaired (I85 REPAIR_01); ADDENDUM_01 CONFIRMED 0/0/1 (`6c7a9f87…`) |
| RV109 early read (`rvp_r3p_read_01/NOTES.md`) | SP's first part | Interim, 0/2/7 (`7d312cf0…`) | R3P-1 to R3P-9 folded into SP |
| RV109 RV-P round 2 (`rvp_round2_01/`) | SP | PASS 0/2/4 (`206fd360…`) | SF-1, SF-2 and the ordinal fix at I3; ADDENDUM_01 CONFIRMED 0/0/2 (`f065a1f6…`); every ST and SP hunk through `03f55e7178` is in RV-P's ledger |
| RV112 RV-Q round 1 (`R/REVIEW_RV112/rvq_round1_01/`) | SA and the parked-slot patch | PASS 0/1/8 (`3ac684d2…`) | SF-1 (G-B's bound) at SQ; RV124 confirmed it |
| RV113 RV-R (`R/REVIEW_RV113/rvr_sr_rs_01/`) | SR-RS | PASS 0/1/5 (`09140cb9…`) | Repair 01 CONFIRMED (`10ca3fe4…`); repair 02 CONFIRMED with S-1 (`c43317f8…`), whose twelve rows came in I4′ |
| RV113 (`rvr_sr_ts_01/`) | SR-TS | PASS 0/1/2 (`d44dec19…`) | Repair 01 CONFIRMED with S-1 (`5f86b3e7…`), repaired in I4′ |
| RV113 (`rvr_sr_py_01/`) | SR-PY | PASS 0/4/2, after repair 01 (`d8611e59…`) | Repair 02 and item 4 CONFIRMED with S-1 (`a61bbe1b…`), repaired in I4′ |
| RV120 RV-R (`R/REVIEW_RV120/rvr_i4p_01/`) | I4′'s three lanes | RS and TS CONFIRMED, 0/0/2 (`5397c4f3…`); PY CONFIRMED, 0/0/2 (`dec198e8…`) | none |
| RV120 (`sc_01/`) | SC and the readers' pins | PASS 0/0/5 (`cb681753…`) | none |
| RV124 RV-Q (`R/REVIEW_RV124/b1_sq_01/`) | SQ: G5, M, G6, the re-pins, the witnesses, the challenge, RSS_TIME | PASS 0/0/5; M confirmed (`9bb6f811…`) | Q-N1 and Q-N5 to SB; Q-N3 in §6 below |
| RV125 RV-X (`R/REVIEW_RV125/b1_x_01/`) | The whole PR, at `0752ae8b98` | PASS 0/0/7 (`3c2ea5d4…`) | Its two should-fixes on `8248921552` (the misquoted R6a title, the commit message's reviewers) were repaired by the recut. N-4, N-6 and N-7 are corrected in this package |
| RV124 RV-Q (`R/REVIEW_RV124/b1_passb_01/`) | I107's Pass B | CONFIRMED 0/0/2 (`c88e7a82…`) | none |

Counts are BLOCKING/SHOULD-FIX/NOTE.

## 5. Gates (PLAN_v2 §6's gate set)

| Gate | Result |
|---|---|
| Complete-diff review (RV-X) | PASS 0/0/7, RV125 (`R/REVIEW_RV125/b1_x_01/REVIEW.md`, `3c2ea5d4…`) |
| `source_equality.py`, `--int 31eed8497f --main 953d8c9446` | Checks 1–3 and 5 PASS on `7f5f72912e`: B = `6c821d9ccf`, \|S\| = 33, 33 equal, no three-way merge. Check 4 PASSES on a scratch commit of `7f5f72912e` with this package (`_draft_run_records/`). On the PR head: see the merge record |
| `check_citations.py`, `--base 953d8c9446 --head 7f5f72912e` | **PASS:** 98 resolved, 0 ambiguous, 0 unresolved (§6). On the PR head: see the merge record |
| The full 40-manifest suite and the src-tauri suite, before the freeze | PASS: 0 changed outcomes; src-tauri 116 = 116 (`T/IMPLEMENTATION/B1_PREFREEZE/RECORD.md`, `33daa960…`) |
| Pass B (SB, I107), with RV-Q's confirmation | PASS with no stop: I107 (`R/I107/b1_passb_01/RETURN.md`, `8449242f…`; its one delta, the challenge's `process_floor`, is environmental), and RV124 CONFIRMED (`R/REVIEW_RV124/b1_passb_01/CONFIRM.md`, `c88e7a82…`). The platform fix leaves its gates unchanged |
| Hosted CI and the full-SHA dispatch | See the merge record. CI failed on Linux at `0752ae8b98`; §1.1 is the fix |
| GEN-8 | See the merge record |
| The exact-head DEC-025 and the src-tauri suite on the head | See the merge record |
| T9 and both-entry part 1 | See the merge record |
| The Direct-entry gates (SG) | PASS, I106 (`R/I106/b1_sg_01/RETURN.md`, `3edc7b73…`): pressure refused at D1.5 with exact bytes; coexistence holds; Stale byte-identical to base; 8 registered rows, all D1.4's widening with unchanged bytes; no product caller |
| T6S consistency | The c = 1 successors are unchanged. T6S's suites run in DEC-025: see the merge record |

## 6. B1's evidence, by citation

| Evidence | Record |
|---|---|
| **QUAL_B1,** with M's ruling and the text-error budget | `copies/QUAL_B1.md` = `R/I104/b1_sq_01/QUAL_B1.md` (`eb6541fc…`), §3 and §6. M: RR "R6a: …" and "R6b: …"; RV124 §2 |
| **QUAL §7 for B1 (Q-N3): the identifier audit** | G5's TEXT enforces the audit on B1's code. 12 of 12 controls pass: the unmodified run is complete, and each of the 11 altered runs is incomplete with its own finding (`R/I104/b1_sq_01/_run_records/g5/g5_c3/audit_controls.out.txt`). RV124 §1.6: removing any of I104's five new `id_audit` entries, or the terminal-kind static rule, makes TEXT incomplete at that site. The 4 new non-candidates, read by type, carry no identifier alias (QUAL_B1 §11; RV124 §8) |
| **QUAL §9 for B1 (Q-N3): the controls** | The registration in a copy: PP 741 passed, 1 failed (Mac `t13`), 79 ignored (QUAL_B1 §8; RV124 §3). The unregistered and Stale fixture sweep moved to SG gate 3. The other crates' suites moved to the pre-freeze suite and DEC-025. The mutants: SQ's G04, and RV124's 21 (20 killed, M17 equivalent by data, Q-N4), with each slice's set in its return and review. Pass B's controls are SB's |
| **The generator and the profile tree** | Generator `R/I104/b1_sq_01/_run_records/chain/g5_profile.py` (`a8882aba…`, the same bytes as main's `F2A_D1/copies/g5_profile.py`). Tree `_run_records/g6/n5/g5_c3/profile_tree.json` (`46dd2546…`), with N-5's `g6/tools/sq_n5_chain.py`. RV124 §1.1 regenerated the committed block from it byte for byte |
| **`registration.diff`** | `copies/registration.diff` = `R/I104/b1_sq_01/registration.diff` (`d85101ea…`), one hunk, applied as `b1` `ddc8eaaf54`. It is SQ's historical diff: its comment's citation was corrected on NUM at `75cd6be76b` |
| **The witness logs** | `R/I104/b1_sq_01/_run_records/g6/witnesses/` (dev/test and release): 40 of 40 entry points pass in both builds (QUAL_B1 §4). RV124 §5 reran 51 dev/test entry points |
| **The challenge** | `PP/tests/retained_memory_challenge.rs`; every peak is within its A1-S-1 bound, the largest 113.3 MiB against E_mov,max (RSS_TIME §2; `_run_records/g6/rss/`; RV124 §6) |
| **RSS_TIME, with R9's reading** | `copies/RSS_TIME.md` = `R/I104/b1_sq_01/RSS_TIME.md` (`f230c7ec…`). R9: RR "SQ complete; R9 read; …". The largest RSS at the caps is 206.5 MiB; the priced worst case is 9.18 GiB; release wall time is at most 1.77 s |
| **07n's parity** | RV120 `sc_01/REVIEW.md`: 1,218 checks per reader, 0 misses; TS = PY on all 1,914 verdicts; RS differs only on 46 entries' declared raw G7 codes; 07m's 339 entries unchanged. Records: I100 `b1_sc_01/RETURN.md` (`a4bd24f9…`); I101 `b1_sc_pins_01/` (`6343e404…`, ADDENDUM_01 `ba4f17ab…`) |
| **The W-C2 pins** | I85 `R/I85/b1_sp_01/I3_01.md` §2 (`5084603d…`): document = fixture `7922e3e5…` / `f2800bd4…`; published bytes `c7a18593…` / `a77c010b…` (sparse / dense). RV109 round 2 ADDENDUM_01 rebuilt both byte for byte; SG gate 2 matched them |
| **RV95 N-5's direct test** | I90 `R/I90/b1_sr_rs_01/RETURN.md` §3.5 (`29eb10a3…`); RV113 SR-RS §7. It kills S1 and the off-by-one, and covers the 13 receipt fields, `failure.block_order` included. It calls `integer` directly, and a census binds every call site; it does not take the composite receipt's path. RV125 N-5: DESIGN §7's composite-receipt clause is met in substance, because the composite receipt reaches `integer` only through the call sites the census enumerates |
| **RV97 R2-N-2** | `a4eab1dd01`. The module doc of `RE/tests/retained_precision_contract.rs` now says the two L = 0 bases were published by the Direct entry in the registered dev/test build, and that reading the controls establishes no execution |

**`citations.json`** (#1082's format) is pinned at NUM `31eed8497f`. Its documents table is #1082's 23 names, unchanged.
- **The run** (`--base 953d8c9446 --head 7f5f72912e`, `_draft_run_records/outputs/check_pr.out`): 71 record and RR citations (28 distinct) and 27 document citations; 98 resolved, 0 ambiguous, 0 unresolved, 0 verification failures. **PASS.**
- **Three negative controls** each fail, as they should: a missing entry (UNRESOLVED), R6b's entry pointed at R6a's heading line, and a changed copy (each FAILED).

## 7. Routed notes

- **SQ2:** B1's +112 MB over I82's pricing uses part of B3-S's headroom. SQ2 prices B3 on B1's real profile and may select 10.75 GiB; B3b-A stays provisional (RR "R6a: …").
- **B2 (DEF-O):** any DEF-O revision covers the MPa ×1e6 projection beside NC-1's mm→SI (RV124 Q-N2; RR "R6b: …").
- **B2:** combination coverage comes at G3 with B2's contract, which also changes (g)'s `combinations` and `components` rule (RR "RV113's three returns verified; …", item 2; RR "I91's and I92's rounds verified; …", ruling 2).
- **B3:** B3a changes (g)'s `pressure_contract` rule (the same item). RV78-N1's preview-table bindings go to `physics-retained-1` (RR "I86's SW probe accepted; …", ruling 1).
- **B3 (`b2`):** the G8 sourced-case rule, a three-reader disagreement that predates B3, is aligned for B2/B3: PY on `b2-p`, RS and TS with I101. PR-B1's readers equal main's there (RR "The G8 sourced-case rule aligned across the three readers (I100's B3 addendum 01); …").
- **B2/B3 admission (lane A, lane P, SQ2):** the modulus-basis custody fields in `retained_product.rs` are per invocation. They must become per case before any lane admits material selectors at c ≥ 2 (RV125 N-2; RR "RV125 (RV-X) passes PR-B1 (0/0/7); …").
- **B2's reader round:** a negative-value refusal for `global_upper_bound_pa` and `certified_gap_pa`, with a shared shape (RV125 N-1, which carries RV120 N-2; the same ruling).
- **Outside B2, B3 and SQ2:**
  - F2b gets the per-term `constructor_ordinal` (RV109 round 2 ADDENDUM_01 N-1);
  - B7 gets one Rust source for the transport metadata check (RR "I90's SR-RS repair round 2 verified; …", ruling 1);
  - T3 product work after PR-B1 (I109): published magnitudes that do not depend on the platform's libm, such as a correctly rounded `hypot`. It would retire `t13`'s Mac failure and §1.1's glibc variants (RR "I107's Pass B (no stop); …");
  - the owner's machine-adaptive memory budget study comes after PR-B1 (RR "#1114 merged; …").
- **Not taken, optional:**
  - a Direct-entry variant with a fault armed (RV109 round 2 ADDENDUM_01 N-2; SQ item 10);
  - the per-call observation rule, about −150 MB (RR "R6a: …").

**Closed by Pass B and RV-X:** PLAN_v2 §2.3's phase-4 check (I107's item 10: 3 error-text owners per case, within 3m + 1, and 3c + 1 per invocation); RV109 round 2's N-1 (RV125: M11 is killed by five SP tests); and Q-N1 (no other unpriced owner).

**Erratum (RV125 N-7).** The first draft of this record attributed the `checks_passed` rule to decision 21. It is decision 1; §1 and §2 now read so.
