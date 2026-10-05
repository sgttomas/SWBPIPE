# RV82: independent review of U1 grant 2 (closed translations, the unavailable form, U2 on the failure path)

**Verdict: PASS.** 0 BLOCKING, 0 SHOULD-FIX, 9 NOTE.

**My grant-1 findings are repaired:**
- **S2 is complete.** Every same-mode foreign invocation I tried is refused, through both serializers. Only a canonically identical invocation passes.
- **S1 is fixed.** R06 is now killed.
- **N1 is closed for five of its six checks at their sites.** R08 is pinned only in its new helper; its call site is not (N1′).
- **N3, N6 and N7 are done.**

**What I verified:**
- **G-i is total.** Every match is exhaustive and has no wildcard arm. No Debug text and no panic appear on any production path.
- **My own spot derivation agrees on 62 of 62 entries,** across 20 tables, against C2 §2–§4, C3 §3, RR:7784 and the schema `$defs`.
- **All 260 encodable corpus values validate against their `$def`.** The six refused variants are justified.
- **U2 on the failure path is sound.** The FK anchor travels on every path, from the moment it is created to both outcomes.
- **The pre-anchor classification is right:** see §3. No genuine D1 proof failure can be misreported.
- **`serialize_unavailable` is unreachable from production.** Its D4d code and phase match the readers' table on all 8 receipts I produced, and the S-2 and S-3 branches fail closed.
- **No other behaviour changed:**
  - PP is 648 ok, which is grant 1's 634 plus the 14 added tests.
  - runner/headless and result_export are identical across base, grant 1 and the candidate.
  - The milestone's ordinary and successor bytes are unchanged.
  - The three readers pass again with parity 98/98 and 99/99.
- **Mutants:**
  - I61's grant-2 set: 46 of 48 killed, the same as I61. Its two survivors are confirmed unconstructible from PP.
  - My own: 13 of 19 killed. The survivors are explained below.

TASK Type 2, dispatched directly by ROOT (HELP_HUMAN), with no descendants.
- **Candidate:** `b54caba7ab`, on top of grant 1 `59a5de2032` (base `43a6368c21`).
- **Records:** NUM `1f63288a50`.
- **Time:** 2026-10-04T05:15Z to about 06:00Z.
- **Host:**
  - The memory guard (PID 5387) was checked before every Cargo job.
  - Default toolchain; `--locked --offline`; `CARGO_BUILD_JOBS=4`, `RUST_TEST_THREADS=2`; one Cargo job at a time.
  - No install, no new tooling, and no solver-at-scale, native or DEC-025 job.
- **Git:** reads only, with `GIT_OPTIONAL_LOCKS=0`.
- **Copies:** fresh `git archive` copies of `b54caba7ab`, `59a5de2032` and `43a6368c21` in WT/rv82. I deleted them afterwards.
- **One false start, disclosed:** the first suite pass used subtree archives. It failed at compile on missing `validation/` fixtures, so I re-ran every suite on whole-tree archives. The aborted logs are kept in scratch.
- **Writes:** WT/rv82/, WT/targets/rv82/, WT/scratch/rv82_u1_serializer_02/ and this folder.

**Placeholders:** P, PP, FK and R as in `u1_serializer_01`. `evidence/` is this folder's evidence subfolder.

## Findings

| # | Sev | Where (`b54caba7ab`) | Evidence | Remedy |
|---|---|---|---|---|
| N1′ | NOTE | retained_wire.rs:1105 (call site), :1114–1116 (helper) | **R08 is pinned only in `after_conserved`.** Replacing the call site `(after_conserved(…), "cases[].run.invocation_after")` with `(true, …)` (mutant R08b) survives the whole PP `--lib`. I61's "re-expressed" R08 mutates the helper body, which `test_after_conserved` covers. The other five checks (R09, R10, R13, R14, R22) are killed at their actual sites. RunWork cannot be constructed from PP, and native runs are self-consistent, so this stays a defensive check. | Optional: make `run_conservation` itself take the four exact amounts and test it, or accept the gap as defensive. |
| N2 | NOTE | retained_wire.rs:1206–1209 (`native{run_ref}`), :1625–1629 (the D4d `kernel_*` row) | **Mutants G04 and G07 survive.** G04 turns a native failure with a recorded Run into D38's `capture`; G07 fixes the `kernel_<terminal>` code. Nothing in PP produces a non-selected native Run, which I61 also states. My D4d spot check covered only the `source_unavailable/preparation` and `facade_certificate/facade` rows. The kernel row agrees with the readers' table by reading (PY:1049–1056), but is unexercised. Under T3 none of these is a publication. | For wider F2a: a seam-level unit test that builds the unavailable case from a synthetic non-selected outcome, pinning `native{run_ref}` and `kernel_unresolved`/`kernel_refused`. |
| N3′ | NOTE | FK final_case.rs:374–376; retained_receipt.rs:174–180 | **Mutant G05 survives:** it accepts proof work without an anchor. No test constructs pre-anchor proof work. The classification itself is right (§3). | Optional FK unit test: `ProductCertificateSpent::new(&[]).owner_matches(o) == false`. |
| N4 | NOTE | retained_receipt.rs:176–177 | **Mutant G09 is equivalent.** It changes the seam's error for a proof whose capture has no Selected outcome. A proof cannot start without a Selected outcome, and the serializer maps every seam error to the same `association`. | None. |
| N5 | NOTE | retained_wire.rs:692 (T27); :1604–1608 (V06) | **I61's survivors are confirmed unconstructible from PP.** For T27, `PreparationArithmeticCause` is a tuple struct with a private field and no public constructor (FK product_certificate.rs:639–640); its translation delegates to the pinned `numeric_error`. V06 is S-2, which needs a kernel fault seam. | Wider F2a, with S-2. |
| N6 | NOTE | retained_product.rs:273–286 (`ordinary_report`) | **R24 remains equivalent,** as in grant 1. | None. |
| N7 | NOTE | retained_wire.rs:394–400 | **`UnresolvedReason::WorkAccounting` emits `{fault}` only.** That matches the closed schema and ROOT's ruling RR:7784 ("`prior`: not carried on the wire … C3:261–263"). ROOT's grant-2 ruling cites "C2 §2's own wire form", but C2 §2 (I32 CONTRACT_DELTA §2) has no work_accounting row, and I34 API-02:317–318 had proposed `prior: null|Stop`. The binding basis is RR:7784. Mutant G01 (adding `prior`) is killed. | Citation only. |
| N8 | NOTE | retained_wire.rs:1313–1330; retained_wire_tests.rs (`u1_refusals_are_typed`) | **The G-j Scope branch for a two-case request has lost its only committed test.** S2's digest check now refuses the foreign two-case invocation first, which is stricter. A capture whose own invocation has two cases cannot be produced by the one-case driver, so the branch is unreachable in grant 2. | Wider F2a, when a multi-case capture exists. |
| N9 | NOTE | retained_product.rs:308 | **S2 binds the invocation the capture observed.** The digest is sha256 of the canonical JSON of `{request: <whole raw request>, solver_mode}` (source_receipt.rs:109–117), so it is complete. The typed request passed to the ordinary run beside it is tied to that capture only by `CapturedInvocation::parse` returning both. That is an existing custody. | U3: keep a single parse per invocation. |

## 1. My grant-1 findings on `b54caba7ab` (`evidence/g2_foreign_invocation.txt`, `evidence/mutants_rv82_g2_summary.txt`)

**S2, complete.**
- `ProductCapture::invocation` records `invocation_digest`.
- `one_case`, which both serializers share, refuses `association/invocation` unless the supplied invocation's digest equals the recorded one (retained_wire.rs:1320–1324).
- The digest covers the whole canonical raw request and the mode.

My probe results (through `serialize_selected` and `serialize_unavailable`):

| Invocation | Result |
|---|---|
| Label changed | refused |
| Project id changed | refused |
| Load magnitude changed | refused |
| Load provenance changed | refused |
| Unknown extra member on a node | refused |
| Same request, other mode | refused |
| Identical re-parse | emitted, correctly |
| Same request with its keys re-inserted in another order (canonically identical) | emitted, correctly |

**Mutants:**
- G10, which compares only the digest's presence, is killed.
- I61's S2 and S2b are killed.

**S1, fixed.** `u1_load_row_case_capture` now asserts `d5_diagnostic_ref == None`. R06 is killed by it on the whole PP `--lib`.

**N1.** R09, R10, R13, R14 and R22 are killed by I61's new negatives, at their sites. R08 is killed only in the helper (N1′).

**N3, N6 and N7.**
- **N3:** the control comment now names A and B, and leaves B′ to U3. The `ORDINARY_SHA256` doc comment still says "controls B and B′". That is historically true: both of experiment 03's controls equalled these digests.
- **N6:** the comment is corrected (:1455–1458).
- **N7:** the precedence is documented (:101–107).

## 2. G-i, the closed translations

**Totality.**
- Every translation function matches exhaustively over its native enum, with no `_` arm.
- The only wildcard arms in production are:
  - the non-range `RangeTrigger` (`encoding`; a W2 trigger is a range error by construction);
  - an unknown integrity code;
  - a non-failed, non-completed verification (`association`);
  - a non-complete summary coverage (null, unchanged from grant 1);
  - two non-enum matches.

**No Debug text and no panic.**
- The only `format!` calls on the projection path produce hex bits, sha hex and the two diagnostic ids, plus `kernel_{terminal}` built from a fixed `&str`.
- Typed Text payloads are static names or owned typed strings. `CaptureError` details are Display-built ids, never `{:?}`.
- The Debug-formatted `ProductCapture.work` string is never read by the serializer.
- There is no `unwrap`, `expect`, `panic!` or `assert` in production.
- `owned_ordinary()` replaces the panicking `ordinary()` on the unavailable path.

**Spot derivation** (`evidence/derive_g2.py`, `evidence/derive_g2_out.txt`). I wrote 62 expected wire values by hand. Each came from the contract text, RR:7784 and the schema `$defs`, not from the code. I compared them with the serializer's output for the same native variants, read through the test-only corpus hook. **62/62 agree.** They cover:

| Table | Entries |
|---|---|
| WideError | 6 |
| Stop | 11 |
| Quantity | 3 |
| attempt Reason | 5 |
| Outcome | 2 |
| Unresolved | 3, including work_accounting without `prior` |
| Refusal | 3, including the six `rigid_parameters` bit strings |
| BlockRefusal | 2, the `block_step` sentinel and a row |
| NumericError | 2 |
| ViewIssue | 2 |
| BridgeError | 3 |
| ProductError | 3 |
| SourceError | 4, including the directional association refusal |
| OriginError | 1 |
| CaptureError | 2 |
| OperationalError | 1 |
| G5aError | 2, including the out-of-range `quantity_kind` |
| SectionError | 2, including property index 5 |
| StructuralError | 2 |
| PublicError | 3, including D38 `capture` and `numeric{cause:null}` |

**Schema validation, my own jsonschema Draft 2020-12 run.** All 266 corpus entries over 20 `$def`s: **260 valid, 0 invalid**. The 6 refused are exactly the six wire-less variants:

| Variant | Refusal | Why it is justified |
|---|---|---|
| `WideError::CountRange` | encoding | The schema's WideError has no such tag (C1 §4: unknown variants fail encoding). |
| `WideError::WorkAccounting` | encoding | Same; I34 API-02's WideError `work_accounting` was not adopted into the schema. |
| `SourceError::ZeroDirection` | association | C2 §3: impossible from the declared empty directional list, so a typed internal association failure. |
| `SourceError::NonFiniteDirection` | association | Same. |
| G5a `quantity_kind` ∉ {0,1} | encoding | The schema enum is [0, 1]. |
| A section property index ≥ 5 | encoding | The schema enum has 5 names. |

In D1 a work fault abandons the successor under D-4, so the two WideError refusals change no reachable outcome.

**`UnresolvedReason::WorkAccounting`** emits `{space, tag, fault}`, which is the schema's form, with `prior` private (N7 on the citation).

## 3. U2 on the failure path

**FK.**
- `ProductCertificateSpent.anchor: Option<Arc<ProofAnchor>>` is set the moment the anchor is created (final_case.rs:1636).
- It is carried through:
  - `rebind` (:441);
  - the temporary `mem::replace` at :1655, restored by `bind.rebind(&[])`;
  - every stage move: `into_ready`, projection, `abandon`, `abandon_values` and `certify_final` (:1620–1621, :1793–1794, :1870–1875, :1878–1902).
- So both `CertifiedProductProof.work` and `ProductProofFailure.work` keep it.
- The two new `owner_matches` are false before an anchor exists.
- The extra `Arc` clone allocates nothing. `prepared_capacities[4]` already prices the anchor, and the `CasePrep` it pins is already held by the capture's selected owner.
- No product_certificate code uses `strong_count` or `try_unwrap`.

**The C3 seam** (retained_receipt.rs:174–180). Any proof work, certified or refused, must belong to the capture's Selected owner. Otherwise the seam returns `WorkAssociation`, and it does so before any projection reads the work.

**The serializer** (retained_wire.rs:1573–1581) checks the refused proof's failure, and any surviving certificate, before projecting.

**The deliberate `I61_FOREIGN_OWNER` flip** (retained_product_tests.rs:3126–3136):
- The old custody assertion `view.is_ok()` becomes `Err(WorkAssociation)`.
- It adds `late.owner_matches(own) && !late.owner_matches(other)`.
- The late-failure roster just above it still projects against its own owner.

**Mutants:**
- **Killed:** U01–U04, and my G06, which makes `ProductProofFailure::owner_matches` always true; the seam and `u2_failure_path_foreign_owner_refused` both catch it.
- **Equivalent:** G09 (N4).

**The classification ROOT asked about: proof work without an anchor.** `begin_prepared_product` (final_case.rs:1624–1636) can fail before the anchor exists in only four ways:
1. **Either of the first two `work.visit()` calls.** Each is a `Cause::Accounting` on a fresh `ProductCertificateSpent::new`, at visit count 1 or 2, so it cannot overflow.
2. **`product_owner_stamp(run, owner)` refused**, giving `bad("prepared recorded owner")`, which is `Cause::Association`. The stamp requires:
   - the run's recorded selected entry;
   - `Arc::ptr_eq` on the prep;
   - a Case owner, equal source identity, and cache origins (origins.rs:795–800).
3. **The `size_of::<ProofAnchor>()` layout count**, a compile-time constant.

The only reachable cause is (2), and it is an owner association failure. Reporting it as `WorkAssociation` is the faithful classification: the proof's own cause says the same thing, as `Association`.
- **(1) and (3) are unreachable**, so no genuine D1 proof failure can be misreported.
- **The seam does drop the proof's own detail** ("prepared recorded owner"). That is acceptable: the path fails closed and is never published.
- **Keep `WorkAssociation`.** The pre-anchor path itself is unpinned (G05, N3′).

## 4. `serialize_unavailable` (`evidence/check_unavailable_out.txt`)

**Unreachable from production.**
- Its only callers are in `retained_wire_tests.rs`.
- The three `Refused` owners come only from `prepare_case` and `prepare_observed`, and neither has a non-test caller.
- `ProductCapture` is installed only by `prepare_observed`.
- lib.rs, the dispatch and `retained_memory.rs` are unchanged in grant 2.

**The D4d spot check.** I produced 8 receipts myself, through the actual refusal owners (`evidence/rv82_probe_tests.rs::rv82g2_unavailable_dump`):
- preparation refusal ×2 modes;
- D38 ×2;
- the candidate refusals `abandoned` and `values` after a selected Run, ×2 each.

For each, I derived the expected (code, phase) from the readers' accepted table (PY:1049–1056; RS:1988; TS:752), using the receipt's own error kind and Run, and compared:
- **8/8 match:** `source_unavailable/preparation` for preparation and D38, and `facade_certificate/facade` for both refusal kinds.
- **0 schema violations.**
- **D38's shape holds:** `run`, `run_ref` and `source_ref` are null; `execution_order` and `calls` are empty; `charged` is 0; `sources` is empty.
- **One `RETAINED_PRECISION_UNAVAILABLE` diagnostic,** referenced by the case.
- **The legacy disclosure is kept and referenced** (C2:160), and there is no method token.
- **Python, Rust and TypeScript all fail first at G3 `COVERAGE_MISMATCH`** on all 8, as T3 requires (`evidence/unavailable_*_results.txt`).

**From source,** `solve_native` sets `capture.native` only after the kernel returns, and errs after that only for a non-selected outcome (retained_product.rs:3432–3450). So `native{run_ref}` occurs only with a non-selected Run, which is exactly the reader's "native" row. That row is untested (N2).

**The fail-closed branches:**
- **S-3:** a `CaptureError::Source` preparation failure is refused as `Untranslated cases[].source_decline`. Source errors arise only at preparation or at the earlier capture, so both reach this branch, and V05 is killed.
- **S-2:** a prepared source with no native call is refused as `Untranslated sources[].kernel_source_sha256`. It is unreachable from PP (V06).

## 5. No other behaviour change (`evidence/suite_*`, `evidence/build_cand.warnings`)

| Suite | Base `43a6368c21` | Grant 1 `59a5de2032` | Candidate `b54caba7ab` |
|---|---|---|---|
| PP lib + 21 integration targets | 618 ok, 1 F, 1 ign | 634 ok, 1 F, 1 ign | 648 ok, 1 F, 1 ign |
| runner/headless | 85 ok, 2 F | 85 ok, 2 F | 85 ok, 2 F |
| result_export | 149 ok | 149 ok | 149 ok |

- **The diffs:** grant 1 → candidate is exactly the 14 added grant-2 tests, all ok. Base → grant 1 is 16 added, 0 removed.
- **The failures** are only t13 and the two load_reference tests.
- **Production build warnings** are identical to grant 1 (8).

**The milestone.** My probe's successor files are byte-identical to grant 1's: `ac6986b0…59dc` and `6cd1d249…c9b5`. `u1_ordinary_bytes_unchanged_under_capture` passes.

**The readers on those bytes, with my own invocations:**
- Python, Rust and TypeScript all PASS G0–G8 with eligibility off.
- py = rs = ts, and equal to the certificate's verdicts: 98/98 and 99/99. The verdict classes come from my grant-1 probe facts, whose native run produces these identical bytes.
- **TypeScript lane:** as before, the `node_modules` link is to REPO_ROOT, and `public/` is copied, not built, from WT/f2a-readers. The hashes are in `evidence/ts_lane_public_sha256.txt`.

**The diff from grant 1 touches 6 files,** with no reader, schema, fixture, dispatch, lib.rs or `retained_memory.rs` change:
- retained_wire.rs
- retained_wire_tests.rs
- retained_product.rs
- retained_product_tests.rs
- retained_receipt.rs
- FK final_case.rs

**The test edits:**
- The removed lines are the B′ doc comment and the two-case Scope expectation, which became Association/invocation (stricter; N8).
- `retained_product_tests.rs` changes only the `.certificate()` accessor and the deliberate flip.

## 6. Mutants (`evidence/mutants_rv82_g2.py`, `evidence/mutants_rv82_g2_summary.txt`)

**The setup.** The mutants ran on a clean `git archive` of `b54caba7ab` (core, fixtures and schemas), with target WT/targets/rv82/mut. Afterwards I verified that every mutated file is byte-equal to the commit.

**NONE controls:**
- the whole `--lib`: 486 ok, with only t13 failing;
- I61's filter: 31/31 ok.

**I61's grant-2 set,** 48 mutants run verbatim with its filter: **46 killed**, with no compile-error kill. The survivors are T27 and V06, as I61 reported (N5).

**My grant-1 survivors, re-run on the whole PP `--lib`:**
- **Killed:** R06, R08 (helper), R09, R10, R13, R14 and R22.
- **Survived:** R08b (the call site; N1′) and R24 (equivalent; N6).

**My own grant-2 mutants, on the whole PP `--lib`.** 6 of 10 killed:

| Mutant | Result |
|---|---|
| G01: Unresolved `work_accounting` carries `prior` | killed |
| G02: BlockRefusal bound swapped | killed |
| G03: SourceError spring id mapped as member id | killed |
| G06: `ProductProofFailure::owner_matches` always true | killed |
| G08: G5a `quantity_kind` admits 2 | killed |
| G10: S2 checks only the digest's presence | killed |
| G04: native with a Run treated as `capture` | survived (N2) |
| G07: the kernel code fixed | survived (N2) |
| G05: pre-anchor work accepted | survived (N3′) |
| G09: the seam's non-selected arm | survived, equivalent (N4) |

## For ROOT

There is nothing that must be ruled before U1 merges into NUM. Optional items:
- **N1′:** whether to pin R08 at its call site.
- **N2 and N3′:** carry to wider F2a or to an FK unit test.
- **N7:** cite RR:7784 rather than C2 §2 for the `prior` omission.
- **N9:** for the U3 brief, keep the single `CapturedInvocation::parse` custody.
