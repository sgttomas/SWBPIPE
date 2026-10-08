# RV77: independent review of I61's producer coverage seam

**Verdict: PASS. BLOCKING 0, SHOULD-FIX 1, NOTE 4.**

The candidate is commit `c618675e84` (base CODE `652ad0cc1f`). It does what I57 §1–§3
require of the typed C3 seam:
- the coverage is the proof's own borrowed vector, never the adapter copy;
- empty maps to null, and a complete vector covers every body in order;
- the stage rules match the actual failure seams;
- the rederivation is bit-for-bit equal to native `summary_coverage_data` on every
  genuine case RV77 enumerated, and never refuses one;
- no failure is added to any affected suite.

The single SHOULD-FIX is a test gap, not a code defect. Three rules the code enforces
correctly are not pinned by the candidate's own tests. RV77's reviewer tests pin all three.

This is source review with test and mutant evidence. It is not acceptance, merge, F2a
completion, a memory or M claim, or release.

## Identity, host and basis

- **Reviewer:** RV77, TASK Type 2, dispatched directly by ROOT (HELP_HUMAN, Agent 0) as a
  background subagent. It has no descendants and did not write the code.
- **Host:** <host> (arm64), cargo/rustc 1.97.1. The memory guard ran
  throughout (PID 5387).
- **Time:** first tool call 2026-10-03T20:16:35Z; the runs ended by 20:37:03Z.
- **Path abbreviations:** WT is the t3 worktree root, P is `projects/chirality-piping`,
  FK is `P/core/solver/frame_kernel/src/structural/retained`, PP is
  `P/core/product_physics/src`, and R is the RESUME_2026-09-30 record root.
- **Source copy:** RV77's own `git archive c618675e84` in WT/rv77. Its four changed
  files hash exactly as in I61's RETURN:
  - final_case.rs: 3986919726e9…
  - retained_receipt.rs: 55dc8cb50526…
  - retained_product_tests.rs: be2579221d45…
  - product_final_case_tests.rs: 9e0ca9219254…
- **Base and repair trees:** extracted separately from `652ad0cc1f` and `e0fc33b4f7`
  (P/core and P/fixtures only).
- **Git:** no Git writes, index operations or installs. Git reads used `GIT_OPTIONAL_LOCKS=0`.

Basis read (sha256):

| File | sha256 |
|---|---|
| AGENTS.md | c8ce87ef342902cb081bc659b26fc9a4edda1b6dba513814e5cb1e14e0b1dffd |
| agents/AGENT_TASK.md | 1a13a5b00b3ce01ff8519efe6b46bcbe0cd6a5b7985e24282fa7efa2c57c8fb7 |
| P/AGENTS.md | d9481951912ceffdd5bc47dbb6549bf0044f5d92f9fe6a9b11ba5969bfebc792 |
| R/BRIEFS/RV77_COVERAGE_PRODUCER_REVIEW.md (NUM `5fce0abd48`) | 57fd3486c42822940062f9c2094bc6150f9a9eb9c3e2a8ab6d3a2e43c0a0dc61 |
| R/I57/summary_coverage_01/ADDENDUM.md | 845d5258bf6ec61734242e8be958bbf544a13af3c6d6d8871d7f536f0b44ad59 |
| R/REVIEW_RV76/summary_coverage_01/REVIEW.md | 10341323b974d7a24155f23013d16b43e803f1d3cb6638df052a6ce2f51a61f3 |
| R/I61/coverage_producer_01/RETURN.md (read after RV77's own source reading) | 8821c28ef238b73b684c4ef99e555cb4c5ca4d06949e274ef89627cbc718b423 |
| T3/ROOT_RULINGS_V1.md, the two named rulings (lines 7335–7356, 7479–7510) | 9dff2a26…, e9f87268… |

## Findings

| # | Sev | Where (at `c618675e84`) | Evidence | Remedy |
|---|---|---|---|---|
| S1 | SHOULD-FIX | FK/product_certificate/final_case.rs:269–273; PP/retained_receipt.rs:113, :121 | Three implemented rules have no test in the candidate's suite: the positive-floor-forces-stop refusal at L = 0; the refusal of null on a passed G5a; and the requirement that non-null coverage has `capture.source`. Mutants R1, R6 and R10 survive all `i61_` tests and the broader `fk:product_certificate` and `pp:retained` suites. RV77's tests kill them: `rv77_full_payload_domain…` kills R1, and `rv77_stage_rules…` kills R6 and R10. | Add equivalent tests before acceptance. `tests/rv77_enum.rs` and `tests/rv77_pp.rs` here can be adopted as they are. No source change is needed. |
| N1 | NOTE | final_case.rs:291, :301–302, :288; retained_receipt.rs:119–120; native :1492–1499 | Five mutants survive every test, RV77's included: R1n, R2, R5, R8 and R9. Each is equivalent or unreachable at this source (MUTANTS.md). In particular, the extent bit-equality compares two values from the same `CasePrep` (`prep.source` and `prep.extents`). It is a regression guard against a change to `CasePrep::with`, not a binding of the proof to the owner. | None required. Describe the extent check accurately in later records. Pinning R5 would need an FK-side lane-tamper hook. |
| N2 | NOTE | final_case.rs:294–300; retained_receipt.rs:108; I61 RETURN "Accounting" | Sizes are verified by run: ProductProofTrace 1328 B (+16, one slice; I51 recorded 1312), PreparedAttemptView 1648, CoverageBody 12, CoverageFacts 56, `[f64;3]` 24, ProductSummaryCoverage 16. All are charged by `size_of`, so the copy accounting is honest. The RETURN's account is not fully accurate, in two respects: (1) it calls the extent arithmetic "reads", but per body there are two passes over all nodes (count, then copy) and 9 binary64 operations, unmetered by any owner (TraceCopyWork counts copies only); (2) the 16 B `SummaryCoverage` is charged in local costs although it is already inside the PreparedAttemptView charged at :152, a conservative double count. No allowance is claimed. | Correct the wording in the record. Decide whether projection compute is metered when the receipt transaction is designed. |
| N3 | NOTE | PP/retained_product_tests.rs:3126–3130 | The comment says a proof checked against a different owner "refuses as an association failure". The run prints `I61_FOREIGN_OWNER same_public_facts_result_ok=true`, and nothing is asserted. | Correct the comment and assert `is_ok()`, so the documented custody limit is pinned and a later structural binding has to flip it deliberately. |
| N4 | NOTE | PP/retained_product.rs:3339, :3342 | The owner binding is by custody only, as ROOT accepted. Every production caller passes the proof's own owner (§4 below). But `certificate` is a crate-visible `pub` field on both `PreparedCandidateRefusal` and `PrivatePreparedCandidate`, so code in the crate could pair another attempt's proof with this capture, and the seam would not notice (N3). No code does so today. | Make the fields private with accessors, or bind the anchor in the receipt transaction (ROOT's stated deferral). |

## 1. Custody and sourcing (I57 §3)

- **Source of the vector.** `typed_trace` sets `summary_coverage:&self.coverage`
  (final_case.rs:378), a borrow of the proof's own vector. I61's tests assert pointer
  equality with the proof slice. `retained_receipt::summary_coverage` reads only
  `proof.summary_coverage` (:109). Neither `ProductCapture.summary_coverage` nor
  `prepared_verdict_copy` is referenced. M1 (adapter source) is killed.
- **Ready path.** `CertifiedProductProof::summary_coverage` is `&self.work.coverage`
  (:1848), and `PrivatePreparedCandidate::typed_trace` passes `self.certificate.work()`.
  It is the same vector.
- **No new computation in projection.** The projection adds no solve, residual,
  `verification_nonzero`/state read, nonzero scan or coverage computation. It reads
  public facts and runs Boolean rederivation and binary64 extent arithmetic over public
  coordinates, both of which I57 §2 requires.
- **Null and complete.** `summary_coverage` returns None iff the proof is None. An empty
  vector becomes Null. A non-empty vector must have exactly `body_count` entries with
  `body == index` (:316–320), or it is refused as an association failure.
- **Assignment.** In the prepared path, coverage is assigned only at check_intervals
  :1285, reached only from `certify_final` :1878. The other call, at :1099, is the
  non-prepared `run_case` and never reaches this seam. `begin_prepared_product`,
  `project`, `abandon` and `abandon_values` never assign it. The atomic `?` at :1285
  leaves the draft's empty vector on any helper failure.

The §3 table, row by row against the actual seams (PP/retained_product.rs:3456–3570
and 3604–3634):

| I57 §3 row | Actual seam | Projection |
|---|---|---|
| Preparation/native failure; no proof | `PreparedCaseFailure::typed_trace` and `native_refusal_trace` pass proof None | no coverage object (actual: I61 PP) |
| Proof-start/lane failure, projection failure, values/aliases abandonment | `begin_prepared_product` or `project` Err, `abandon_values` or `abandon`; the vector stays empty | Null; certificate NotEntered, so no refusal |
| Certificate entered, fails before summary | certify_final owner/shape/row checks before :1878 | Null with certificate Failed (actual FK proof, synthetic stage record) |
| Summary completed, later certificate failure | the check_intervals tail, the predicate check, or the accounting fault after :1285 | Complete with certificate Failed (actual); complete never sets the CheckRef |
| Certificate succeeded, later copy/observable/G5a/commit refusal | `certificate=Some(certified)` before each such return; `typed_trace` falls back to `self.certificate.work()` | Complete (actual: the bisected MapWrite prefix) |
| Ready | `PrivatePreparedCandidate::typed_trace` | Complete (actual, both modes) |

## 2. Stage rules

These are at retained_receipt.rs:111–127.

- **Null is refused** on Ready, on `checks[0]==Passed`, and on `checks[2]==Passed`.
  - `checks[0]==Passed` is equivalent to a completed certificate stage, because both
    are set only by `PreparedTrace::checked`.
  - Ready already implies every check passed (:163–164).
  - RV77's `rv77_stage_rules…` additionally checks a null with G5a passed under a failed
    certificate, and it is refused.
- **Non-null coverage requires** each of the following, and each is checked:
  - both lanes Some, AdmittedK then AnnularSource, both Ok;
  - `source_ready` and `capture.source`;
  - Preparation, Native, ProofStart, Projection, Maxima, Values and Aliases all Completed.
    RV77 tested each of the seven set to Failed, and each is refused;
  - the certificate stage and its check entered. RV77 tested each independently, and
    both are refused;
  - `capture.native` Selected.
  
  This is stronger than I57 in requiring Preparation and Native, which every genuine
  trace satisfies.
- **The same source and Run** are established by custody, not compared (§4).
- **A complete vector never implies certificate success.** The certificate CheckRef is
  derived from `trace.checks[0]` alone (:165–166). The actual numeric-predicate failure
  projects Complete with `CheckRef::Failed`.

## 3. `rederive_coverage` against native `summary_coverage_data`

DERIVATION.md gives RV77's own derivation. In short:
- estimate is the identical expression on identical inputs. The coupling test is the
  same because the extent bits are required equal.
- p128/p256 charge equals estimate.
- At p512, native charge is present ∧ positive (after the coupling and the floor OR).
  The rederived charge is stop[force/moment] = present ∧ (positive ∨ A ∨ D). These are
  equal because recover::layout makes D false for force/moment, and A is already in
  positive.

The extent recomputation is sound and exact. It uses the same `body_nodes` filter and
order, the same `source`, and the same `adaptive::body_extent` as `CasePrep::with`.

Every added refusal holds for every genuine vector (DERIVATION.md §4):
- **floor iff p512:** `RetainedSolve` is built only after `certify_publication` enforces it;
- **no stop on an absent kind:** stop is ORed only inside the row loop;
- **a positive floor forces its stop:** the floor is ORed into positive before stop is formed;
- **no input-derived force/moment row:** recover::layout.

Executed evidence (ENUMERATION.txt), from RV77's own row-level transcription of the
native function run against the real `rederive_coverage`:
- **Genuine domain:** 589,824 cases, with 0 refusals and 0 mismatches. 393,216 of them
  are p512, and 157,696 of those have estimate ≠ charge. Every single estimate/charge
  flip is detected.
- **Payload domain:** 73,728 cases, including invalid (p, floor) pairs, all matching
  RV77's independent statement of the contract (9,288 accepted, 64,440 refused).
- **Input-derived force/moment rows:** 336 patterns would break the p512 identity, so
  the coverage_facts refusal is necessary.

## 4. Owner binding

Callers of `check_summary_coverage`:
- production: only `retained_receipt::summary_coverage` (:128), which passes the
  Selected owner from `capture.native`;
- tests: FK `i61_check` and `i61_actual_native…`.

Callers of `retained_receipt::project`:
- production: the four typed_trace methods at PP/retained_product.rs:3606–3634, two of
  which pass no proof;
- tests: I61's and RV77's PP tests (synthetic pairings, labelled).

In `project_candidate`, three things keep the proof tied to its owner:
- The owner is taken from `self.capture.native`, and every proof work (draft, failure
  or certified) descends from `begin_prepared_product(case.run, owner, ..)` on it.
- `self.capture.native=native` is restored (:3553) before the refusal or candidate wraps
  the same `PreparedCase`.
- The owner cannot be replaced afterwards:
  - `solve_native` refuses after the Native stage or with `native.is_some()`;
  - `prepared` is a private field;
  - `test_capture_mut` is `cfg(test)`;
  - re-entry is refused (`proof_attempted`).

No non-test code calls `project_candidate` or `typed_trace` yet.

So no reachable production path pairs a proof with a different owner. The seam itself
cannot detect such a pairing when the public facts are identical (N3, N4). This is
ROOT's accepted custody limit.

## 5. Mutants

MUTANTS.md has the full results.
- **I61's six and NONE,** re-run from the clean archive with I61's patches verbatim:
  NONE passes (FK 3, PP 4). All six are killed by the same tests as in I61's table.
- **RV77's eleven:**
  - killed by the candidate's tests: R3 (has_data from stop outcomes), R4
    (certificate-entered not required), R7 (absent-kind refusal dropped);
  - survivors of the candidate's tests: R1, R1n, R2, R5, R6, R8, R9 and R10. RV77's tests
    kill R1, R6 and R10 (S1);
  - survivors of all tests: R1n, R2, R5, R8 and R9, each argued equivalent or
    unreachable (N1).

## 6. No other behaviour change

| Run (candidate unless named) | Result |
|---|---|
| frame_kernel `--lib` | 477 passed, 0 failed, 1 ignored (583 s) |
| frame_kernel `--test s11_site_table` | 3 passed |
| product_physics `--lib` | 449 passed, 7 failed, 1 ignored |
| product_physics `--lib` at base `652ad0cc1f` | 445 passed, 7 failed, 1 ignored |

- The candidate's PP failure set is identical to base: six `f1a_tests::*`, each panicking
  at f1a_tests.rs:72, and `s11g_tests::t13_committed_fallback_uz_is_byte_identical`
  ("SparseInteractive: committed bytes changed"). The +4 passes are I61's new tests.
- The compiler warnings are identical to base (the same five pre-existing PP dead-code
  warnings, and none in FK).
- The diff only adds to the existing test files (+215/−0, +135/−0), and the existing
  `typed_trace` and `project` callers' tests still pass.
- The only behaviour change is the new refusals and the new field in the typed seam,
  which has test-only callers.
- The accounting is covered in N2.

## F1a repair commit (for ROOT's end question)

Commit `e0fc33b4f7`, parent `c618675e84`, changes exactly one line of
`P/core/product_physics/tests/formation_check_runtime.rs`.
- Its blob sha256 is faaf940d8973…, which is byte-identical to `main` (local and
  `origin/main`) and to `8104a4fedd~1`.
- The restored inline literal is byte-identical to the fixture
  `P/fixtures/product_preview/rf_skew_t_cant_off_122_r1e-04.request.json` (both 2aa51bee…).
- The fixture remains, and its four code users are unchanged.
- product_physics `--lib` at `e0fc33b4f7`: 455 passed, 1 failed (`s11g t13`, the known
  Mac test), 1 ignored. The six F1a tests pass.

**Confirmed.**

## Evidence here (see SHA256SUMS)

- **REVIEW.md:** this report.
- **DERIVATION.md:** RV77's derivation.
- **ENUMERATION.txt:** the enumeration and stage-rule run output.
- **MUTANTS.md:** the mutant results.
- **mutants/R*.diff:** RV77's mutation diffs.
- **tests/rv77_enum.rs and tests/rv77_pp.rs:** the reviewer tests. They were included
  only in RV77's copy, through a `#[path]` module line appended to final_case.rs and
  retained_product_tests.rs.
- **Bulk (not committed):** logs, scripts and driver are in
  WT/scratch/rv77_coverage_producer_01/. RUNS.txt lists their sha256.

## For ROOT

- **S1:** decide whether to require the three missing tests before acceptance (RV77's
  files can be adopted), or to accept the candidate with RV77's evidence as the pin.
- **N4/N3:** record with the deferred structural owner binding when the receipt
  transaction is designed.
- Nothing here needs an owner ruling.

## Confirmation at fa225abded

**Verdict: CONFIRMED.** S1 and N3 are fixed, and the N2 correction is accurate. The
new head adds no failure. The PASS stands, with no open BLOCKING or SHOULD-FIX finding.
N1 is recorded, and N4 is carried to the receipt-transaction design, as ROOT ruled.

This confirms source and tests only. It is not acceptance, merge or release.

**How this was checked**
- ROOT asked for this in its message and in the ruling "RV77's review of the producer
  coverage seam: PASS; the missing tests are added before acceptance" (NUM `059e876617`).
- RV77 used its own `git archive fa225abded` in WT/rv77c, deleted afterwards, and the
  existing targets WT/targets/rv77/{frame_kernel,product_physics}.
- The memory guard ran (PID 5387), and only one cargo job ran at a time, with
  `--locked --offline`, CARGO_BUILD_JOBS=4 and RUST_TEST_THREADS=2.
- There were no Git writes. Git reads used `GIT_OPTIONAL_LOCKS=0`.
- **Disclosed host deviation (ROOT interim ruling):** the Xcode licence on this Mac is
  unaccepted, so every cargo command ran with
  `DEVELOPER_DIR=/Library/Developer/CommandLineTools` (Apple clang 21.0.0) in its own
  environment. No system setting was changed.
- The runs took place from 2026-10-03T20:58Z to 21:10:39Z. Records are in
  `confirm_fa225abded/`:
  - RESULTS.txt;
  - RUNS.txt, the bulk sha256;
  - run.sh, the driver.

| Check | Result |
|---|---|
| Delta `e0fc33b4f7..fa225abded` | One commit, `fa225abded` (parent `e0fc33b4f7`). It changes only `PP/retained_product_tests.rs` (+61 −1) and `P/core/solver/frame_kernel/tests/retained_k4/product_final_case_tests.rs` (+149 −0). |
| Source unchanged | At `fa225abded`, final_case.rs = 3986919726e962b5… and retained_receipt.rs = 55dc8cb505261041…, byte-identical to `c618675e84`. formation_check_runtime.rs = faaf940d8973… (the F1a repair, as confirmed above). |
| Tests verbatim | The body of `mod rv77_enum { … }` (product_final_case_tests.rs:640–782) has sha256 7cc8d74bcc652bfe…, and the body of `mod rv77_pp { … }` (retained_product_tests.rs:3140–3190) has 24290692e4fd9c18…. Both are byte-identical to tests/rv77_enum.rs and tests/rv77_pp.rs here. The only additions are the module wrapper and a three-line provenance comment each. rv77_enum now sits inside `product_final_case_tests`, and its `use super::*` still resolves: it compiles and passes. |
| R1, R6, R10 killed by the committed suite | NONE passes (`fk:product_certificate` 34 passed; `pp:retained` 48 passed). R1 is killed by `rv77_enum::rv77_full_payload_domain…` ("missing refusal p=512 floor=Some([3.0, 0.0]) present=[false,false,true,false] stop=[false;4]"). R6 is killed by `rv77_pp::rv77_stage_rules…` ("null with passed G5a"), and R10 by the same test ("missing capture.source"). RV77's committed mutation diffs applied with `patch -p1`; each restore was sha256-verified. |
| N3 | Lines 3126–3133 now say a proof projected against a different selected owner with identical public facts is *not* refused, and that owner identity rests on producer custody. The test asserts `view.is_ok()`. The run prints `I61_FOREIGN_OWNER same_public_facts_result_ok=true`, so the comment is accurate. |
| frame_kernel `--lib` | 480 passed, 0 failed, 1 ignored (593 s): 477 plus the 3 rv77_enum tests. Warnings: 0. |
| frame_kernel `--test s11_site_table` | 3 passed. |
| product_physics `--lib` | 456 passed, 1 failed, 1 ignored. The only failure is `s11g_tests::t13_committed_fallback_uz_is_byte_identical` ("SparseInteractive: committed bytes changed"), the known Mac test that also fails at base. The count is 455 at `e0fc33b4f7` plus rv77_pp. Warnings: the same five pre-existing ones. **No failure is added.** |
| N2 correction (I61 ADDENDUM_RV77.md, "Corrections…") | **Accurate.** It now lists, as unmetered compute (TraceCopyWork counts copies only): one layout pass, two node passes, two linear lookup scans (resolution_scale and floor) and the 9 binary64 extent operations plus min/max, giving O(bodies × (layout rows + nodes)) overall. The lookup scans add O(bodies²), which that bound covers because there are at least as many layout rows as bodies. It names the 16 B `SummaryCoverage` as a conservative double count, restates the extent bit-check as a regression guard (N1), and claims no allowance. The sizes and `size_of` charges are unchanged. |

One small point in I61's addendum is stale but harmless: its status line still says
"uncommitted … (branch head `e0fc33b4f7`)", although the work is now committed as
`fa225abded`. It needs no action.
