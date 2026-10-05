# I61 RETURN: summary coverage in the producer's typed C3 seam

**Status: COMPLETE within the brief, uncommitted in WT/f2a-coverage for ROOT. All required tests pass; product_physics `--lib` also shows 7 failures that fail identically at base `652ad0cc1f`.** The proof-owned
coverage vector now reaches the private typed C3 seam as a borrowed slice. Its projection is
either null or the complete roster, and every entry's nine native flags are rederived from the
compact payload plus public facts and compared bit for bit. All six required mutants are killed,
and the NONE control passes. This is the typed seam only: no serializer, JSON, public receipt,
schema, reader, corpus, activation, allowance or M claim. No independent review, acceptance or
merge is claimed.

TASK Type 2, dispatched directly by ROOT (HELP_HUMAN, Agent 0) as a background subagent, with no
descendants. Host: M5 Max, with the memory guard running as PID 5387 throughout. Worktree:
WT/f2a-coverage on `codex/piping-f2a-coverage-20261003`, HEAD `652ad0cc1f` unchanged. No Git
writes or index operations were made; Git reads used `GIT_OPTIONAL_LOCKS=0`. First tool call
2026-10-03T19:39:19Z; source frozen 19:58Z; freeze runs ended 2026-10-03T20:10:53Z.

## Basis read (sha256)

NUM HEAD at read: `e02f02a93d`. Brief commit: `3fc80230c0`.

| File | sha256 |
|---|---|
| AGENTS.md | c8ce87ef342902cb081bc659b26fc9a4edda1b6dba513814e5cb1e14e0b1dffd |
| agents/AGENT_TASK.md | 1a13a5b00b3ce01ff8519efe6b46bcbe0cd6a5b7985e24282fa7efa2c57c8fb7 |
| P/AGENTS.md | d9481951912ceffdd5bc47dbb6549bf0044f5d92f9fe6a9b11ba5969bfebc792 |
| R/BRIEFS/I61_COVERAGE_PRODUCER.md | 34bfb85c418c59256fec0c64c860049c13e037df80e3fbe36d7ec61649e08259 |
| R/I57/summary_coverage_01/ADDENDUM.md (all) | 845d5258bf6ec61734242e8be958bbf544a13af3c6d6d8871d7f536f0b44ad59 |
| R/REVIEW_RV76/summary_coverage_01/REVIEW.md | 10341323b974d7a24155f23013d16b43e803f1d3cb6638df052a6ce2f51a61f3 |
| T3/ROOT_RULINGS_V1.md (the two named rulings, lines 7335–7415) | 4a249ad480abab54bd8ea27cdb5e327aa466c69d39e0ede94623241f1606a538 |

These source files were read and left unchanged, at CODE `652ad0cc1f`:

| File | sha256 |
|---|---|
| PP/retained_product.rs | d07383fc026e61e494a2b0a307271eaf2eb0329c333da95533f4b39c5d52af1a |
| FK/adaptive.rs | 6a2fc382bf8cae0502c41030da8ac9bc0cfe1f1b80b7aceac60ff7e40f345eda |
| FK/product_certificate.rs | 64e09224aacdc328440e9b1c9d602d894c9072873ae3ca8b0bffda6c484823ef |
| FK/origins.rs | c4bde5de1961f736c4b422d835ecc4bdecca847234337f7d6650410e1fbc991c |
| FK/recover.rs | e452e3467608a4680359a36d30b6a580d72a12b778d506ff8118646257b7d388 |
| FK/product_certificate/bridge.rs | 0cbc204925748ecb1bee480b762f7de4b425965c68ec972f5e8963e985c0a8df |
| P/core/solver/frame_kernel/src/structural.rs | d850b06a5d3d0ed67d2f261308ddaaf7855ba278c759ef4a314ae9b67726ce5a |
| P/core/solver/frame_kernel/tests/retained_k4/models.txt | 5aa07e539fa5c62a1650cd11dae9054f221171fe58e80655e19a97d99a76cd9e |

The design's anchors hold at this source:
- `ProductCertificateSpent::summary_coverage()` is at final_case.rs:301 and `CertifiedProductProof::summary_coverage()` at :1758.
- The atomic assignment `spent.coverage = summary_coverage_data(..)?` is at :1195. The local vector is returned only after every body completes.
- The adapter's fallible copy is at PP:3362–3366.
- No contradiction with I57 §2/§3 was found.

## Changed files (in WT/f2a-coverage)

| File | Base sha256 / lines | New sha256 / lines | +/− |
|---|---|---|---|
| FK/product_certificate/final_case.rs | 3bc84bf1…5634 / 1800 | 3986919726e962b52eaa730eb4cc76311fb79ff6dee6e075aa75b4a4d639def7 / 1890 | +91 −1 |
| PP/retained_receipt.rs | 84810876…eace / 138 | 55dc8cb50526104186c944f9265f54395c3dc206e70fa7c43be2fc2982dee517 / 191 | +55 −2 |
| PP/retained_product_tests.rs | 9bb2ac03…962a / 2916 | be2579221d4530bcfa1cd02bd34742a96ca6c9c71d7b9608a9ad59946a4bb491 / 3131 | +215 −0 |
| P/core/solver/frame_kernel/tests/retained_k4/product_final_case_tests.rs | c5895e36…39a6 / 499 | 9e0ca9219254de392445954cbf5b5aaed8aa160484c60acf09a94d89a1340cfa / 634 | +135 −0 |

Nothing else changed. In particular, `PP/retained_product.rs`, `FK/product_certificate.rs`,
`structural.rs` and `s11_site_table.rs` are untouched. No re-export was needed and no integer
site was declared.

### What the source now does

- **FK: the borrowed view.** `ProductProofTrace` gains `summary_coverage: &'a [ProductSummaryCoverage]`. `typed_trace` sets it to `&self.coverage`, which is the proof's own vector, so the view is a borrow, not a copy. It is never sourced from `ProductCapture.summary_coverage`. No solve, residual, nonzero scan or coverage computation was added.
- **FK: the typed nine-flag function.** `rederive_coverage(CoverageBody{body,stop[4],has_data}, &CoverageFacts) -> ProductSummaryCoverage` does the following:
  - estimate is presence AND E>0, coupled across the force/moment pair only when L≠0;
  - p512 charge is `[stop[force], stop[moment]]`;
  - p128/p256 charge is estimate;
  - proof precision 1024 is not an input.
  
  It also enforces the necessary public consistency that a genuine vector always satisfies:
  - p must be 128, 256 or 512;
  - a floor exists iff p is 512 (as adaptive.rs:3591 requires);
  - an absent kind has no stop;
  - a positive floor forces the stop of its present force/moment kind.
- **FK: the public facts, `coverage_facts`.** These come from the owner:
  - per-kind presence from the bound layout `prep.layout`, which is `recover::layout` of the bound source. Any input-derived force or moment row is refused.
  - L, recomputed by `adaptive::body_extent` over the body's nodes in ascending (native) order, using a fallibly reserved buffer. It must equal `prep.extents[b]` bit for bit.
  - E from `evidence().resolution_scale`.
  - the floor from `evidence().floor`.
  - native p.
- **FK: `ProductProofTrace::check_summary_coverage(owner, copies)`.** This requires exactly one entry per native body. For each entry it checks that `body == index`, projects the compact payload, derives the facts, rederives the entry and compares it with `==` against the actual `ProductSummaryCoverage` (body, stop, estimate, charge and has_data). A mismatch is an `association` ProductFailure. The vector is never altered.
- **PP: the typed C3 value.** In `retained_receipt.rs`, `PreparedAttemptView.summary_coverage` is `Option<SummaryCoverage>`. It is None iff the proof is None, so no proof means no coverage object. `SummaryCoverage` is either `Null` or `Complete(CompleteCoverage)`. `CompleteCoverage` holds the proof's slice privately and exposes only `len()` and the compact `bodies()` iterator in ascending body id. Only `project` constructs it.
- **PP: the stage rules in `summary_coverage()`, run after the existing LostTrace check:**
  - an empty proof vector maps to Null;
  - Null is refused (StageConsistency) on Ready, on a passed certificate or on a passed G5a.
  
  Non-null coverage requires all of the following, otherwise StageConsistency:
  - both lanes present with AdmittedK then AnnularSource, both Ok;
  - `source_ready` and `capture.source`;
  - Preparation, Native, ProofStart, Projection, Maxima, Values and Aliases all Completed;
  - the certificate stage and check entered (Completed or Failed);
  - `capture.native` Selected.
  
  It then runs the FK cross-check:
  - an association failure maps to the existing `TraceProjectionError::WorkAssociation`;
  - an accounting or storage failure in the check maps to `LostTrace`.
  
  No new error variant was added. Complete coverage does not imply that the certificate succeeded; the certificate CheckRef is unchanged.

## Accounting (existing trace-cost owners only)

The cost of each part is recorded under an owner that already exists:

- **The borrowed view** is a 16-byte slice inside `ProductProofTrace`. The existing `copies.record::<ProductProofTrace>()` charges it, and the size is now 1328 B (I51's recorded 1312 B + 16).
- **The per-body projection** is charged in the kernel `TraceCopyWork` (`ProjectionWork.kernel`), per body:
  - one `CoverageBody` (12 B);
  - one `CoverageFacts` (56 B);
  - one `[f64;3]` (24 B) per body node copied into the extent buffer;
  - one rederived `ProductSummaryCoverage` (16 B).
  
  The function refuses if this work becomes inexact.
- **The typed value** charges one `SummaryCoverage` (16 B) per proof in the local `TraceCosts`. `project` refuses with LostTrace if it is lost. `PreparedAttemptView` is now 1648 B; I51 recorded 1608 B.

Layout, node, E and floor reads and the binary64 extent arithmetic are reads, not copies, so they are
not counted as events. No new allowance, LME, byte permit or M is claimed. The compact `bodies()`
iterator for the future encoder is lazy; its encoding cost belongs to the later receipt transaction.

## Commands and results

All runs used `--locked --offline`, `CARGO_BUILD_JOBS=4`, `RUST_TEST_THREADS=2`, an explicit
`--manifest-path` and the target dir `WT/targets/i61-coverage/{frame_kernel,product_physics}`.
Each had a 1,200 s wall (perl `alarm`), and only one cargo job ran at a time. The target dirs
were seeded by an APFS clone of `WT/targets/rv65-named-support`. Logs are in WT/scratch/i61_coverage_producer_01/.

| # | Command (crate, filter) | Result |
|---|---|---|
| 00 | frame_kernel `--lib --no-run` (warm) | ok, 17 s |
| 01 | frame_kernel `--lib i61_` | 3 passed |
| 02 | product_physics `--lib i61_` | 4 passed |
| 03 | frame_kernel `--lib product_certificate` | 31 passed, 0 failed |
| 04 | product_physics `--lib retained_product_tests` | 38 passed, 0 failed |
| 05 | frame_kernel `--test s11_site_table` | 3 passed |
| NONE | frame_kernel / product_physics `--lib i61_` on the frozen source | 3 / 4 passed |
| 10 freeze | frame_kernel `--lib` | 477 passed, 0 failed, 1 ignored (605 s) |
| 11 freeze | frame_kernel `--test s11_site_table` | 3 passed |
| 12 freeze | product_physics `--lib` | 449 passed, **7 failed**, 1 ignored. All 7 failures already exist at base CODE `652ad0cc1f`; see the note below |

Before each freeze run, the frozen sha256 of all four changed files was rechecked (`shasum -c`, OK).

**The 7 product_physics `--lib` failures already exist at base and are unrelated to I61.** They are:
- six `f1a_tests::*` tests, each panicking at f1a_tests.rs:72 with "RF_SKEW_T_CANT_OFF_122_R1E_04 in formation_check_runtime.rs" (a constant the test expects in its included source);
- `s11g_tests::t13_committed_fallback_uz_is_byte_identical`, which fails with "SparseInteractive: committed bytes changed".

To check this, the four changed files were temporarily overwritten in the working tree with their `652ad0cc1f` bytes (`git show`, with no index or Git write). The same 7 tests then failed identically (`13_base_pp_f1a_s11g.log`). The frozen bytes were restored afterwards and verified by sha256. These tests are not investigated here: they are outside the fence and the coverage scope.
product_physics shows five pre-existing dead-code warnings in unrelated items. No new warning appeared.

## Mutants (`_run_records/mutants/<id>.diff`)

Each patch was applied to the frozen source and run with `--lib i61_` on its crate. The file was then
restored, and the restore was verified by sha256 (final_case `3986919726e962b5…`, retained_receipt `55dc8cb505261041…`).

| Id | Mutation | Result | Killing test(s) |
|---|---|---|---|
| NONE | none | pass (3 FK, 4 PP) | — |
| M1_adapter_copy_source | coverage from `capture.summary_coverage` | KILLED | `i61_failure_prefixes_null_complete_and_adapter_prefix` (actual partial adapter copy: StageConsistency); `i61_certificate_prefixes_and_stage_rules_with_actual_fk_proofs` |
| M2_empty_as_all_false | empty proof vector → all-false complete roster | KILLED | the same two tests (null expected at maxima/values abandonment and at certificate-before-summary) |
| M3_null_allowed_on_ready | the Ready result is exempt from the null refusal | KILLED | `i61_certificate_prefixes_and_stage_rules_with_actual_fk_proofs` (Ready with an empty-coverage proof) |
| M4_drop_one_body | the complete roster drops its last body | KILLED | all four PP i61 tests |
| M5_p512_charge_is_estimate | p512 charge := estimate | KILLED | `i61_synthetic_rederivation_precision_floor_extent_and_absent_kinds` (synthetic) |
| M6_accept_flag_mismatch | the rederived entry is not compared | KILLED | `i61_rederivation_refuses_partial_reordered_and_mismatched_vectors` |

Two notes on these results:
- **M3.** The existing `project` already requires every check Passed for Ready, so Ready implies a passed certificate. The mutant therefore exempts Ready itself, not just the `Ready` disjunct, which on its own would be an equivalent mutant.
- **M5.** In the only actual p512 run (SKEW-K1E-60), every stop, estimate and charge is true, so that run cannot distinguish M5. The mutant is killed by the synthetic control only.

## I57 §5 controls

| Control | Test | Kind |
|---|---|---|
| Zero/no-data body keeps its entry | FK `i61_actual_native…`: ALL-ZERO-BODY body 1 has has_data=false and stop all false, in a 2-body source. PP `i61_specimen…`: the zero specimen through the prepared producer to Ready, with has_data=false, stop all false and estimate false | actual (FK native solve + proof summary function; PP full producer) |
| Cancelled ±x loads → has_data=true | PP `i61_specimen…`: specimen(false) plus +1/−1 UX terms (the PP:1442–1500 witness pattern) through the prepared producer to Ready; the complete roster has has_data=true with stop all false | actual |
| Absent body/kind; L=0 vs L≠0 | FK `i61_synthetic…` | synthetic. No fixture has a single-node body or an absent kind. |
| p512 with zero floor and with positive floor → charge = stop | FK actual SKEW-K1E-60: p512 with a positive floor, charge == [stop2, stop3], and the cross-check passes. Zero floor is synthetic. | actual (positive floor) + synthetic (zero floor and discrimination) |
| p128/p256 under fixed-1024 proof → charge = estimate | PP `i61_ready…`: both modes are p128, with no floor, 88 projection conversions at 1024, and charge == estimate. FK actual: TWO-SPAN, PRESCRIBED, ZERO-TORSION-345 (p128) and SKEW-K1E-28, SKEW6-K1E-12, REACTIONS-ONLY (p256) | actual |
| No proof → no trace/coverage object | PP: AfterPrelude preparation refusal and native refusal | actual |
| Lane failure → null | PP: invalid proposed material, so the actual K lane is retained as Err(NonpositiveSource), with null coverage | actual |
| Proof-start failure → null | PP: tampered radius fact; refused before any lane | actual |
| Projection failure → null | PP: synthetic stage record (Projection failed) over actual FK proof work | synthetic stage record |
| Values/aliases abandonment → null | PP: TraceFault::Maxima (abandon) and ValuesCompletion | actual (existing test hooks) |
| Certificate entered, fails before summary → null | PP: the actual FK certify_final refusal on a frozen value (empty proof vector), projected with a synthetic stage record | actual proof work, synthetic stage record |
| Summary completed, later certificate failure → complete | PP: old-source prepared run, a numeric_predicate failure, a complete roster and a Failed certificate check. Also the FK variant-4 proof under a synthetic stage record. | actual |
| Adapter copy fails partway → proof-owned vector | PP: the actual run with a bisected MapWrite permit (403 of 512) refuses the first coverage write. The adapter prefix is 0 of 1, the projection is the complete proof roster, and the certificate is passed. | actual |
| Ready → complete | PP `i61_ready…`, both modes | actual |
| Partial, reordered, foreign-body or flag-tampered vector refused | FK `i61_rederivation_refuses…` on actual native vectors | synthetic tamper of actual vectors |

## Deviations and interpretations (for ROOT)

1. **Placement.** The typed rederivation lives in FK and is reached through a method on the
   already-exported `ProductProofTrace`. `product_certificate` is private in `retained/mod.rs`, and
   every public export runs through `origins.rs`, which is outside the fence, so new FK types are
   not nameable from PP. The PP C3 value and the stage rules are in `retained_receipt.rs`.
2. **Extra consistency checks.** These are the ones listed under the typed nine-flag function above.
   They are necessary for every genuine vector and stronger than the bare comparison.
3. **Error mapping.** The brief's "existing ordinary transaction error" is read, in the private
   seam, as the existing `TraceProjectionError`. Association becomes `WorkAssociation`, and stage
   inconsistency becomes `StageConsistency`. There is no new variant.
4. **Same source/Run.** The private seam checks the presence of the prepared source and the
   selected native Run, and checks every entry against that owner's public facts. It cannot
   compare the proof's anchor with the owner, because `ProofAnchor::matches_owner` is `pub(super)`
   and `ProductCertificateSpent` holds no anchor. A proof projected against a different selected
   owner with identical public facts passes (`I61_FOREIGN_OWNER … ok=true`). This is the I57 §5
   limit: producer custody is by construction (`project_candidate` passes `capture.native`'s owner
   to `begin_prepared_product` and restores it before projection). Binding the anchor would need a
   new FK field or export.

## Open items

- The receipt transaction (serializer/JSON/schema/readers/corpus) is later work, as briefed.
- The proof-anchor binding above is ROOT's call. It is needed only if the typed seam must
  authenticate the owner identity itself rather than rely on producer custody.
- Some controls had no actual fixture. Each is covered synthetically instead, as labelled in the
  §5 table:
  - no p512 with a zero floor;
  - no L=0 body or absent kind;
  - no multi-body source through the prepared producer (the PP fixtures are single-body; the multi-body zero body is witnessed natively in FK).
- Fresh independent review of the frozen diff has not been done.
- The 7 product_physics failures that already exist at CODE `652ad0cc1f` (f1a_tests ×6 and
  s11g t13) block a clean product_physics `--lib` run independently of I61. They need their own
  owner and diagnosis.

## Bulk (WT/scratch/i61_coverage_producer_01/)

| sha256 | bytes | path |
|---|---|---|
| 5596b3cf947e0f89feec066a9c5847127c707e4ffac422d37a745abaa41b334c | 402 | WT/scratch/i61_coverage_producer_01/00_fk_warm_build.log |
| c2df1fc56ef7708ad427df2f9abcfc3cb8812b7bf2b582cad2a57a5ef0887f35 | 3100 | WT/scratch/i61_coverage_producer_01/01_fk_i61_focus.log |
| a794a381ee584eeb6001ab32c95e096fc74120fb69183bbd6c3547f4b77401db | 1334063 | WT/scratch/i61_coverage_producer_01/02_pp_i61_focus.log |
| 68afdc2b4316d1c685a4c9dd8f669e43d025206a800944383270457620492d85 | 4842 | WT/scratch/i61_coverage_producer_01/03_fk_product_certificate.log |
| f9db85dcad900e820da6142e3875b24a2631e9d23ec078ec8f620aea7f388ded | 7386 | WT/scratch/i61_coverage_producer_01/04_pp_retained_product_tests.log |
| d206323ba36daa51efbdf0ab4c47f5fa471965b947d84d1202987e3ad2651771 | 750 | WT/scratch/i61_coverage_producer_01/05_fk_s11_site_table.log |
| 4367b918c2f407875388a3254a82e696fe1a38f516540fe2cbd46535f10b0cf1 | 14 | WT/scratch/i61_coverage_producer_01/10_freeze_fk_lib.exit |
| 7760a54fad2d271aacb23fbd49eb889b42611096171225e6dbd07f1f02279555 | 51686 | WT/scratch/i61_coverage_producer_01/10_freeze_fk_lib.log |
| 7631669328e8fc94cbc625d7cc1a7323575f407a3ab0cc4b6b47b1c4acf02450 | 21 | WT/scratch/i61_coverage_producer_01/10_freeze_start.txt |
| 689a1fa42f7d7549b1fb6cf8a3b03bbd8704f45e7caa076dd2739424107cabb7 | 11 | WT/scratch/i61_coverage_producer_01/11_freeze_fk_s11.exit |
| 3dc77a31d964091ace3ed1735d394215b98e419b64e0d03d7a7d6ac03cd61f28 | 750 | WT/scratch/i61_coverage_producer_01/11_freeze_fk_s11.log |
| f315ae4de8e817fa5c7da064b2f735a9a095ba53e54868c3cc00a57f7fff4757 | 21 | WT/scratch/i61_coverage_producer_01/12_freeze_end.txt |
| 00de022735de3408c074e7fbb4be44208cdf5f765d17f02ddd06d7254b294716 | 16 | WT/scratch/i61_coverage_producer_01/12_freeze_pp_lib.exit |
| 0a7aa1e58f5d490a14e9fdc06e666ca0db807b5756b568871abf79d4a0d9e024 | 49247 | WT/scratch/i61_coverage_producer_01/12_freeze_pp_lib.log |
| 7310d3387d786a4e00cd284eb91fb77ffb6f06ca2966e4dad6ecab857e02b103 | 8782 | WT/scratch/i61_coverage_producer_01/13_base_pp_f1a_s11g.log |
| 3986919726e962b52eaa730eb4cc76311fb79ff6dee6e075aa75b4a4d639def7 | 82835 | WT/scratch/i61_coverage_producer_01/frozen/final_case.rs |
| 9e0ca9219254de392445954cbf5b5aaed8aa160484c60acf09a94d89a1340cfa | 35954 | WT/scratch/i61_coverage_producer_01/frozen/product_final_case_tests.rs |
| be2579221d4530bcfa1cd02bd34742a96ca6c9c71d7b9608a9ad59946a4bb491 | 158632 | WT/scratch/i61_coverage_producer_01/frozen/retained_product_tests.rs |
| 55dc8cb50526104186c944f9265f54395c3dc206e70fa7c43be2fc2982dee517 | 12999 | WT/scratch/i61_coverage_producer_01/frozen/retained_receipt.rs |
| 50262a5bbbc8836d0804123827b1ec9066c91375c35dc786aaf40d64f5342d7b | 618 | WT/scratch/i61_coverage_producer_01/frozen_source.sha256 |
| 15f1743c708351130311e080710e6653d75385c1c9b6532a3c838ec1ad27f45e | 43261 | WT/scratch/i61_coverage_producer_01/i61_source.diff |
| 68a1885a982d521c5bf25cafd3f5b6ffc23588d2494acd7a361e387f83851619 | 1334823 | WT/scratch/i61_coverage_producer_01/mutants/M1_adapter_copy_source.log |
| 55dc8cb50526104186c944f9265f54395c3dc206e70fa7c43be2fc2982dee517 | 12999 | WT/scratch/i61_coverage_producer_01/mutants/M1_adapter_copy_source.orig |
| e24f0fdaafa54f3cc3861f97b407c68c7ccb5631ed6dafb5a7675016dc6d317d | 260611 | WT/scratch/i61_coverage_producer_01/mutants/M2_empty_as_all_false.log |
| 55dc8cb50526104186c944f9265f54395c3dc206e70fa7c43be2fc2982dee517 | 12999 | WT/scratch/i61_coverage_producer_01/mutants/M2_empty_as_all_false.orig |
| 5361596169fb9379241ad9bbc1bf38502c897868f2f0914c79926441fc51c03a | 1334743 | WT/scratch/i61_coverage_producer_01/mutants/M3_null_allowed_on_ready.log |
| 55dc8cb50526104186c944f9265f54395c3dc206e70fa7c43be2fc2982dee517 | 12999 | WT/scratch/i61_coverage_producer_01/mutants/M3_null_allowed_on_ready.orig |
| 8735b1a9420fe5cf0da646416f480ae052c1d6d4336d1c868b1a2c4a641b348e | 342153 | WT/scratch/i61_coverage_producer_01/mutants/M4_drop_one_body.log |
| 55dc8cb50526104186c944f9265f54395c3dc206e70fa7c43be2fc2982dee517 | 12999 | WT/scratch/i61_coverage_producer_01/mutants/M4_drop_one_body.orig |
| 0b84141f89708a211fba69da7ae845ae464765e79ca3fa8732ff998ff47726e2 | 3876 | WT/scratch/i61_coverage_producer_01/mutants/M5_p512_charge_is_estimate.log |
| 3986919726e962b52eaa730eb4cc76311fb79ff6dee6e075aa75b4a4d639def7 | 82835 | WT/scratch/i61_coverage_producer_01/mutants/M5_p512_charge_is_estimate.orig |
| 0a0d00a6b9c97126197cd271b63b22a14dce84f80e3bfb27ab67fe313f5c3c18 | 3849 | WT/scratch/i61_coverage_producer_01/mutants/M6_accept_flag_mismatch.log |
| 3986919726e962b52eaa730eb4cc76311fb79ff6dee6e075aa75b4a4d639def7 | 82835 | WT/scratch/i61_coverage_producer_01/mutants/M6_accept_flag_mismatch.orig |
| 0add5fbbc66904e67ca8e4d9f594daacc2c56b33427d3005af38ede66d6d2704 | 3207 | WT/scratch/i61_coverage_producer_01/mutants/NONE_fk.log |
| 84256c92a27a6c430d1d5752cb50bd6f3c6ce523e65a482e8d3e1d0f45bd6e49 | 1334211 | WT/scratch/i61_coverage_producer_01/mutants/NONE_pp.log |
| 3986919726e962b52eaa730eb4cc76311fb79ff6dee6e075aa75b4a4d639def7 | 82835 | WT/scratch/i61_coverage_producer_01/orig/final_case.rs |
| 55dc8cb50526104186c944f9265f54395c3dc206e70fa7c43be2fc2982dee517 | 12999 | WT/scratch/i61_coverage_producer_01/orig/retained_receipt.rs |
| 4c739c93c26ef748933f11a54da3fc67d46d03ffb61428fc34dabadfb4c5a3bf | 1408 | WT/scratch/i61_coverage_producer_01/run_mutant.sh |

`orig/`, `frozen/` and `mutants/*.orig` are byte copies of the frozen sources used for restore. `i61_source.diff` is the complete frozen change against `652ad0cc1f`. `run_mutant.sh` is the run-local mutant driver (apply literal patch, diff, run, restore).
