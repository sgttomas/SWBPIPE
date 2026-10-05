# RV82: independent review of U1 grant 1 with U2 (the private serializer and owner binding)

**Verdict: PASS.** 0 BLOCKING, 2 SHOULD-FIX, 9 NOTE.

The candidate does what grant 1 asked:
- Ordinary bytes are unchanged on every suite and route I ran.
- I derived the milestone receipt independently from the contract, the rulings and the native state. All 86 checks agree with the serializer's output in both modes.
- The output equals experiment 03's except for the listed members, and each difference traces to its stated cause.
- All three accepted readers pass G0–G8, with parity 98/98 and 99/99.
- U2's binding is structural and sound for one case.
- I61's 39 mutants reproduce exactly.

**F1's predicate holds exactly.** The capture sets `d5_diagnostic_ref` exactly when K-D5's `formation_check` line is written into that same integrity diagnostic. I established this from source and confirmed it over 90 captured seeds in both directions. One direction is not pinned by any committed test (S1).

**F2 is implemented as ROOT ruled** (T1 (a) composed with D39), and the readers accept its wire form.

The two SHOULD-FIX items are:
- a missing negative control for F1 (S1);
- an association gap: the serializer binds its `invocation` argument to the candidate only by mode and case id (S2).

Neither affects the milestone bytes. Both should be repaired in the grant-2 stream, before U3 wires the serializer.

TASK Type 2, dispatched directly by ROOT (HELP_HUMAN), with no descendants.
- **Candidate:** `59a5de2032` on `codex/piping-f2a-serializer-20261004`.
- **Base:** NUM `43a6368c21`.
- **Records basis:** NUM `af8bd1141b`.
- **Time:** 2026-10-04T04:16Z to about 05:05Z, inside the 3-hour box.
- **Host:**
  - The memory guard (`memguard.sh`, PID 5387) ran throughout, checked before every Cargo job.
  - Default toolchain; `--locked --offline`; `CARGO_BUILD_JOBS=4`, `RUST_TEST_THREADS=2`; one Cargo job at a time. I61 ran Cargo concurrently in its own targets.
  - No solver-at-scale, native or DEC-025 job, and no install.
- **Git:** reads only, with `GIT_OPTIONAL_LOCKS=0`: `log`, `diff`, `show`, `archive`, `rev-parse` and `merge-base`.
- **Copies:** `git archive` copies of the candidate (`WT/rv82/cand`, plus the core subtrees `WT/rv82/mut` and `WT/rv82/probe`) and of the base (`WT/rv82/base`). I deleted them at the end.
- **Writes:** WT/rv82/, WT/targets/rv82/, WT/scratch/rv82_u1_serializer_01/ and this folder.
- **Wider consultation:** none beyond the brief's reading list, the contract (C1, C2), D-4 §3 and the cited source.

**Placeholders:**
- P = projects/chirality-piping
- PP = P/core/product_physics
- FK = P/core/solver/frame_kernel/src/structural/retained
- R = the RESUME_2026-09-30 record root
- `evidence/` = this folder's evidence subfolder

## Findings

| # | Sev | Where (candidate) | Evidence | Remedy |
|---|---|---|---|---|
| S1 | SHOULD-FIX | PP/src/lib.rs:4070 (the predicate); PP/src/retained_wire_tests.rs:399–430 | **F1 is pinned in one direction only.** Mutant R06 forces `d5_line = true`, so a `d5_diagnostic_ref` is emitted with no K-D5 line. It survives the **whole** PP `--lib` (472 passed; only t13 fails, as at base). M15 pins only the present→ref direction. `u1_load_row_case_capture` prints `d5` but never asserts it. Its case, RF-CANCEL-UDL-W1e8, has no K-D5 line; 88 of my 90 swept seeds have none. | Assert `seed.d5_diagnostic_ref == None` in `u1_load_row_case_capture`, or add an equivalent negative test, and re-run R06. |
| S2 | SHOULD-FIX | PP/src/retained_wire.rs:933, :958, :1058; PP/src/retained_product.rs:288–303 | **The supplied `CapturedInvocation` is bound to the candidate only by mode and case id.** `ProductCapture::invocation` records only `invocation_mode`. `serialize_selected` takes the invocation digest (`borrowed_digest()`) and the raw request from its argument. In my probe (`evidence/foreign_invocation.txt`) I passed the milestone candidate a same-mode invocation whose load-case **label** differs. The serializer emitted a receipt that binds the foreign digest, and the accepted Rust reader **passes** it against that foreign invocation (`invocation_bound=true`). A project-id or load-magnitude change is also emitted by the serializer, but the readers then catch it at G8. This is the same custody-versus-structure gap that RV77-N4/U2 closed for the proof. It is production-unreachable today. | Record the actual invocation identity in `ProductCapture::invocation`, for example its digest or a structural handle. Have `serialize_selected` refuse `receipt_failure{association, "invocation"}` on a mismatch. Add a test and a mutant. Repair in grant 2, before U3. **ROOT to rule on the placement.** |
| N1 | NOTE | retained_wire.rs:758, :1052, :427, :1016, :421, :446 | **These defensive checks survive the whole PP `--lib`:**<br>• R08: after = before + increment;<br>• R09: body `charged` = final after;<br>• R10: the G4 guard, that no legacy disclosure names the selected case;<br>• R13: the report outcome equals `numerical_quality`;<br>• R14: the omitted diagnostic must be `SOURCE_BLOCK_RECOVERY_UNAVAILABLE`;<br>• R22: the 2^53−1 boundary (`>` versus `>=`).<br>Each re-checks an invariant that the native route or the readers already hold. R10 and R14 guard T1 (a). | Optional cheap negatives through test seams: `successor_envelope` on a doctored envelope, and a legacy entry at exactly 2^53−1. |
| N2 | NOTE | retained_product.rs:268–281 (`ordinary_report`) | **Mutant R24 survives, and it is equivalent.** It lets a non-W2 report overwrite a captured initial failure. An initial failure followed by a published non-W2 report is unreachable: every `Err` path publishes W2, returns early, or takes the exact-selected or contact branch, none of which has a report capture. | None. |
| N3 | NOTE | retained_wire_tests.rs:56–70 | **Control B′ is not a committed test.** It is "the envelope returned on the permit path = the plain run", and no permit path exists until U3. A and B are committed, and the protected digests `9c7ec1a1…` and `21ca629c…` are pinned. The doc comment "(experiment 03 A, B, B')" overstates this. | U3 commits B′ when it installs the permit path. Adjust the comment. |
| N4 | NOTE | retained_product.rs:3491 | **`PreparedCandidateRefusal.certificate` is still `pub`, so crate-writable.** U2 is complete only on `PrivatePreparedCandidate`. ROOT accepted this as a grant-2 item. | Grant 2 makes it private. Its review checks that. |
| N5 | NOTE | retained_wire.rs:662–670, :746–764, :1047–1054 | **Producer-side conservation covers most of D-4 §3 item 5's list:** own = W+K, own stages = O, shared stages = S+V, D+Q ≤ O, the B/T run partition, after = before + increment, and charged = after. "Build flags" and "execution order" are left to the readers. That is acceptable under D-4 §3 item 6, since the Rust reader runs at U3's precommit, and execution order is trivially `[{case,0}]` in D1. | Record it in G4's conformance. |
| N6 | NOTE | retained_wire.rs:924 | **The doc comment "with `retained_precision` as its last member" is wrong.** PP's `serde_json::Map` is key-sorted (no `preserve_order`), so the member sits between `results` and `status`. This is harmless, because every hash is canonical. | Fix the comment. |
| N7 | NOTE | retained_wire.rs:101–176, :968–969, :1055 | **`Enc` is described as a first-failure encoder, but it is not strictly first.** An early-return refusal can follow an already-recorded failure: `legacy_source` Scope, `successor_envelope`, or `material_basis` Untranslated. The returned cause is still typed and inside C1:68's vocabulary. | Optional: call `finish()` before each early return, or reword the comment. |
| N8 | NOTE | FK/product_certificate/final_case.rs:1585–1587, :1853; FK/origins.rs:813–822; FK/adaptive.rs:5055 | **`owner_matches` is `Arc::ptr_eq` on the stamp's `Arc<CasePrep>`.** This is sound: the stamp holds a strong `Arc`, so the address cannot be reused, and `CasePrep::new` allocates once per case. The stamp's `run` is not compared. | Wider F2a: if a CasePrep allocation is ever shared between runs, bind the run as well. |
| N9 | NOTE | retained_wire.rs:599–608 | **G-d requires exactly one rigid owner per constrained DOF.** That is stricter than C2's "all original contributors", and fail-closed for D1, where `capture_supports` already refuses ambiguity. | Wider F2a needs the list form. |

## 1. Ordinary bytes are untouched

**I read the diffs line by line.** The `lib.rs` and `final_case.rs` diffs (+50/−1 and +3/−0) are as ROOT describes:
- **The capture writes:** nine capture calls, each inside `if let Some(observer) = product.as_deref_mut()` (with `diagnostics.last()` or `&attempted_linear` paired in the same `if let`).
- **Two `mod` lines.**
- **A local `legacy_attempted` flag:** set only in the branch that calls `source_recovery::solve_ordinary`, and never branched on.
- **The `demoted` binding:** it binds `amend_integrity_report`'s existing return value.

Nothing else changes:
- no diagnostic push, message, debit or early return is added, moved or altered;
- the dispatch and `retained_memory.rs` are untouched;
- the recorders in `retained_product.rs` contain no `unwrap`, `expect` or unchecked index; the one `len() - 1` follows a push.

**Suites** (`evidence/suites.sh`, outcomes in `evidence/suite_{cand,base}_{pp,runner,rx}.outcomes`):

| Suite | Base | Candidate | Outcome-set diff |
|---|---|---|---|
| PP lib + 21 integration targets | 618 ok, 1 FAILED, 1 ignored | 634 ok, 1 FAILED, 1 ignored | exactly the 16 added tests, all ok |
| runner/headless | 85 ok, 2 FAILED | 85 ok, 2 FAILED | none |
| result_export | 149 ok | 149 ok | none |

The failures are exactly the expected Mac platform set, with identical messages at base and candidate:
- `s11g_tests::t13_committed_fallback_uz_is_byte_identical` ("SparseInteractive: committed bytes changed", s11g_tests.rs:1666);
- `load_reference_route_tests::load_reference_one_actual_solve_…` (:212);
- `cli_load_reference_one_both_modes_…` (load_reference_cli.rs:261).

**Production build warnings:** base 9, candidate 8. The only removed warning is "field `stage` is never read". Nothing is added (`evidence/build_{base,cand}.warnings`).

**Capture installed versus absent.** My probe (`evidence/rv82_probe_tests.rs::rv82_sweep_f1_and_gl`) ran `run_linear_static_preview_observed` with and without an installed `ProductCapture` on the same route. It covered 41 captured fixture requests × 2 modes: the milestone, source_blocks, physics_source, the DEC-092 request, all of S11-F's RF-CANCEL and all of S11-G's rb_controls. The bytes are identical in every case.
- 8 invocations take a different public dispatch, the physics_source exact models with a raised budget. That is a base property of the route; capture changes nothing there.
- 28 invocations (14 requests × 2 modes) refuse capture at parse.

**The committed controls:**
- A (the no-permit direct entry = plain) and B (the captured ordinary run = plain) are committed in `u1_ordinary_bytes_unchanged_under_capture`, with the milestone digests pinned: `9c7ec1a1…` (68250 B) and `21ca629c…` (69366 B). My probe reproduces both, and the plain bytes I captured carry those exact digests.
- B′ has no counterpart in grant 1 (N3).

The candidate envelope that the serializer starts from is *not* the plain bytes. It carries the prepared overlay: 71 row values, the summary maxima and the extrema. That is I51's accepted projection, not a U1 change.

## 2. Contract fidelity of the receipt (my derivation; not the emitter)

**The derivation.** `evidence/derive.py` derives the expected `retained_precision` members from five sources:
- the request;
- the plain ordinary envelope;
- the in-tree fixtures and schema;
- C1/C2/C3, decisions 1–2, T1 (a) and D39;
- native facts dumped by my probe (`evidence/facts_*.json`): the K4RST and K4LED bytes' SHA256, the K4RST magic, the certificate verdicts with their recipes, the kernel constraints, and the invocation's run/call/source counts.

It then compares the result with the serializer's output. **86/86 checks pass** (`evidence/derive_out.txt`).
- **T1 (a):**
  - The successor's diagnostics are the plain list minus `diagnostic:source-recovery:case` (`SOURCE_BLOCK_RECOVERY_UNAVAILABLE`, naming the case), plus one appended selected diagnostic.
  - No legacy-unavailable diagnostic names the case.
  - `legacy_source = {unavailable, null, 0}`.
  - `legacy_source_work[0] = {0, "source closure", "source_closure", 46628, 0, 4000000, "booked"}`. I derived these values from the published legacy Debug text as an oracle only, and `booked` from the Err arm's `debit`, which never refunds (lib.rs:895–901).
- **D39:**
  - The disposition comes from the request and the plain report. Eligible means captured, with no nonlinear support and no combination. Needs recovery means the report is Sensitive. `charged > 0` means an actual attempt ran, so the disposition is `unavailable`.
  - The typed `source_eligible` and `needs_source_recovery` are separate inputs to `ordinary_legacy_route` (retained_product.rs; lib.rs:3784), and they are not merged.
  - My sweep exercised **all five producer branches** and matched each against the envelope's own disclosure:
    - `not_eligible` ×6;
    - `not_required` ×64;
    - `declined_without_attempt` ×2 (RF-CANCEL-UDL-W1e8, WorkReport 0/0/0);
    - `unavailable` ×2 (the milestone);
    - `exact_selected` ×16 (with the `…:selected` diagnostic).
- **Decision 2 (A2/D6a):** `diagnostic_refs` = the three `LOAD_CATEGORY_PREVIEW_MAPPED` ids plus `diagnostic:numerical-integrity:case`, in envelope order. The invocation-level `RULE_CHECK_INPUTS_MISSING`, which has no `affected_refs`, is excluded, and so are the omitted legacy diagnostic and `RETAINED_PRECISION_SELECTED`.
- **Decision 1 (A1):** `retained_state_sha256` = sha256 of the 760 K4RST bytes, whose magic is `4b3452535401` ("K4RST\x01"). Its value is `f89baaf9…f302` in both modes, and `ledger_sha256` = sha256 of the K4LED bytes.
- **G-a:**
  - `producer.semantic_contract_id` and `formulation_basis.profile_id` come from the semantic table.
  - `recovery_method` = `contribution_preserving_multiprecision_v1` on every row of the case (98 and 99 rows); the token is in the schema.
  - One `RETAINED_PRECISION_SELECTED` diagnostic: id `diagnostic:retained-precision:case:selected`, severity `info`, source `core/product_physics`, `affected_refs` `[case]`, with the fixed message and no state or Debug text.
- **G-b:**
  - `initial = {report, diagnostic:numerical-integrity:case, sensitive}`, which equals `numerical_quality`.
  - `w2 = not_triggered`: there is no `range_scaling:` line and no W2 diagnostic.
  - `formation = {load_row_finding: null, d5_diagnostic_ref: diagnostic:numerical-integrity:case}`.
- **G-d:** each of the three constraints, N0 UX/UY/UZ, has `support_indices = [0]`, derived from the request's rigid support `rigid:N0`. The three spring supports at N0 are not constraint owners.
- **G-e:** `not_covered = []`, derived from the verdicts: every quantity row has a class, and the class-less rows are `NonQuantity` (plus `DenseParityObservation` in dense). `absolute_verified` has 69 rows, which equal the verdicts' `AbsoluteVerified` bound bits.
- **G-j:**
  - case, run, source and product-attempt indices are 0;
  - `quality_binding` is present at the case's index;
  - one call (owner `[{case,0}]`, sources `[0]`, runs `[0]`);
  - `execution_order = [{case,0}]`;
  - limits are 20e9 and 60e9 (the meter's own limit);
  - `charged` = the run's `invocation_after`.
- **The fixture hashes:**
  - **The definition:** H("retained_precision_formation_v1", the definition fixture) = `a7ed7ca0…0349`. I computed it with my own RFC 8785 for integer-only payloads and also through the accepted canonicalizer, and it equals the table's `product_formation_definitions` entry.
  - **Policies:** they equal the table's.
  - **Recomputed hashes:** `sources[].preparation.sha256`, `source_identity_sha256` and `receipt_sha256` (by my own JCS and by the accepted canonicalizer), and `publication_sha256` (accepted canonicalizer, since the envelope carries floats). All match.

**F1's predicate (ROOT's addition).** The question was whether `linear.formation_check.is_some()` at the report capture site holds exactly when K-D5's line is written into that same integrity diagnostic. It does:
- `append_integrity_report` (lib.rs:1103–1142) always pushes exactly one diagnostic: the integrity record `diagnostic:numerical-integrity:{case}`.
- `demote` and the two evidence appenders mutate only `diagnostics.last_mut()`, which is that record.
- The `formation_check_evidence_line(…)` line (lib.rs:1145–1163, non-empty for both reasons) is appended iff its `formation_check` argument is `Some`. That argument is `linear.formation_check.as_ref()` (lib.rs:4060), the same field the capture tests (lib.rs:4070).
- The capture's `diagnostics.last()` is read immediately after the call, so it is the same record.
- No other site writes a `formation_check:` line. The nonlinear call (lib.rs:4106) passes `None` and has no report capture.
- So the predicate is exact by construction.

**Runtime confirmation:** over 90 captured seeds, `d5_diagnostic_ref.is_some()` ⇔ the integrity message contains ` formation_check: reason=`. The ref is the integrity id. The milestone is present in both modes; the other 88 are absent. The negative direction is not committed (S1).

**F2 against T1 (a) and D39.** `DeclinedWithoutAttempt` maps to `{declined_without_attempt, null, work_ref → its WorkReport}` and omits the disclosure by its captured id (retained_wire.rs:461–464).
- **T1 (a)** omits the legacy diagnostic on a selected case. The decline pushes the same `SOURCE_BLOCK_RECOVERY_UNAVAILABLE` code (lib.rs:3770), so T1 (a) applies.
- **D39** gives the disposition.
- **G4** forbids any legacy-unavailable diagnostic naming a selected case. So the only alternative to F2's omission would be to refuse.
- **The WorkReport is 0/0/0**, and the settlement is truthfully `booked` (a zero debit).
- **The readers:** I rewrote the milestone's legacy route to F2's shape (`evidence/f2_variant.py`, receipt hash recomputed), and all three readers pass it in both modes (`evidence/f2_variant_{py,rs,ts}_results.txt`). This is a reader-side check only; the milestone does not reach the branch.

## 3. Byte identity with experiment 03

**My copies of experiment 03's outputs** verify: `bca4e9ca…2e59` (sparse) and `06449153…f310` (dense).

**The structural diff** (`evidence/diff_exp03_out.txt`) has exactly six leaves per mode:
- the sidecar `id` label;
- **G-a:** the selected diagnostic's `id` (`…:case` → `…:case:selected`) and its `message`;
- **G-b/F1:** `ordinary_attempts[0].formation.d5_diagnostic_ref`, from null to the integrity id;
- `publication_sha256`;
- `receipt_sha256`.

**Reconstruction** (`evidence/explain_exp03.py`): applying only those three member changes to experiment 03's file and recomputing the two hashes gives a value equal to U1's file in both modes. Every other member is byte-equal, including G-d (`[0]`), G-e (`[]`), A1 and every class and work counter.

**U1's files** are `ac6986b0…59dc` / `6cd1d249…c9b5`, with receipts `efc1a39b…7494` / `3e26499f…ac4a`, equal to the committed pins.

The F1 difference lies outside the brief's literal list (G-d, G-e, A1, G-a). It is the G-b change that ROOT accepted as an application of C2:160.

## 4. The three accepted readers

**The readers.** Their bytes are identical at the candidate and NUM `af8bd1141b` (an empty `git diff` over analysis_runs, result_export, the desktop source, schemas, fixtures and serialization). The last reader commit is `85905e95e9`.

**Invocations** (my own; eligibility off):
- **Python** (`evidence/run_py_reader.py`): `_validate_draft`, plus jsonschema. The public `validate_retained_precision` refuses G0 by design.
- **Rust** (`evidence/rv82_probe_tests.rs::rv82_rust_reader`): `retained_precision::validate`, through the dev-dependency.
- **TypeScript** (`evidence/rv82Reader.test.ts`): `validateRetainedPrecision`, under vitest 4.1.10.

**Lane disclosures:**
- **TypeScript:** the `node_modules` link is to REPO_ROOT's P/node_modules. `apps/desktop/public` is **copied, not built**, from WT/f2a-readers (HEAD `85905e95e9`), the same bytes I61 used; hashes are in `evidence/ts_lane_public_sha256.txt`.
- **Python:** it needs the checked-JSON and units authorities, so I built `openpipestress_jcs_ijson` and `openpipestress_units` from the candidate tree's own unchanged crates with `--locked --offline --release` into WT/targets/rv82. That was a repo build, not new tooling.

**Results:**

| Mode | Python | Rust | TypeScript | Parity |
|---|---|---|---|---|
| sparse_interactive | PASS G0–G8, needs_recompute, eligible False, bound, 98, 0 schema violations | PASS, bound, eligible false, 98 | PASS, needs_recompute, eligible false, bound, 98 | py = rs = ts on (id, normalized, scale, class); = certificate verdicts: **98/98** |
| dense_scrutiny | PASS, 99, 0 violations | PASS, 99 | PASS, 99 | **99/99** |

The classes are 69 absolute_verified, 25 relative_verified, 3 input_derived and 1 non_quantity (2 in dense). See `evidence/parity.txt`.

## 5. U2, the owner binding

**The holders:**
- `PrivatePreparedCandidate.certificate` is private, with a read-only accessor (retained_product.rs:3493–3498). The only mutation is `#[cfg(test)] test_swap_certificate`.
- `PreparedCandidateRefusal.certificate` remains `pub` (N4; grant 2).

**`owner_matches`** is `ProofAnchor::matches_owner`, that is `Arc::ptr_eq(&stamp.prep, &owner.prep)`:
- The stamp holds a strong `Arc<CasePrep>`, created only by `product_owner_stamp`, which checks the selected run, prep, source identity and cache origins. So the check is allocation identity, not public facts.
- A foreign owner with identical public facts has a different per-case `CasePrep::new` allocation and is refused. `u2_foreign_owner_refused` shows this, and my mutant R04 confirms the test discriminates. R04 compares only the public K4 source identity and is killed by that test.

**Every path from the proof to the projection.** `serialize_selected` is the only projection, and nothing reads the certificate before the check at :947–950. After the check it reads:
- `passed()` (:951);
- `typed_trace()`, which reads `certificate.work()` (retained_product.rs:3782), at :963;
- `verdicts()`, for the classes and parity, at :1007.

A mismatch returns `ReceiptFailure{Association, "cases[].selection.owner"}`, which goes on the wire as `association`. The failure-path seam is grant 2's.

## 6. Failure discipline and checked work custody

**No panic on the projection path** (retained_wire.rs:1–1074, production part):
- There is no `unwrap`, `expect`, `panic!` or `assert`. The three `unwrap_or*` calls cannot panic: one has a failure closure (:165), one is the Option default for an absent verification (:681), and one is a sentinel that fails the sum (:748).
- **Indexing:**
  - `inv.runs()[case.run]` is guarded at :954;
  - `run.records[i]` is guarded at :983 (a fixed `[_;4]`);
  - `out[last]` and `parts[last]` use `checked_sub` and equal lengths;
  - the component table indexes `0..6`;
  - `scales[…][kind as usize]` indexes a `[_;4]` over the 4-variant `Kind`, in declaration order T, R, F, M.
- **`serde_json` `IndexMut`** writes only into objects (`formulation_basis` is non-optional).
- **Arithmetic** is checked: `sum` uses `checked_add` and `diff` uses `checked_sub`.

**No Debug text reaches the wire.** The only `format!` calls are bit hex, sha hex and the selected id.

**Every untranslated variant refuses typed** (`Untranslated` → `encoding`):
- Outcome reasons, BlockRefusal, group or build reasons, CheckRef failures and lane errors;
- SectionError arithmetic or out-of-range property, a directional quantity, and non-quarter stations;
- `formation_failure`, `not_attempted`, W2 failed, an R-b′ demotion (F3), a missing legacy route;
- a named or temperature basis, derived E/ν, and Unpublishable.

**Checked work custody (D-4 §3 items 1–6):**
- `record_work` reads `checked_lme` twice, then `checked_own_work`, `checked_shared_work`, `checked_stop_rule_work`, `checked_verification_work`, `checked_verification_shared_work`, `checked_case_charge` and `checked_invocation_increment`.
- Run amounts go through `RunWork` accessors, and Call and Build amounts are `WorkTotal` through `exact`.
- The meter goes through `checked_charged`.
- Trace Counts go through `exact` with the range check.
- A stage slot is emitted only after `StageWork::checked_total().exact()` succeeds (`stages`).
- No legacy field is named. The source guard holds, and C01–C09 are reproduced.
- The legacy `charged` and `limit` must be ≤ 2^53−1; `rejected` beyond that gives `saturation_not_excluded`.
- `ReceiptCheck::wire` is total over C1:68's tokens (D-4b; D01–D03).
- Conservation coverage is in N5. No reader was changed.

## 7. Unreachability

- `mod retained_wire` is private, and `serialize_selected` is `pub(super)`.
- Across P/core, its only uses are in `retained_wire_tests.rs`.
- `ProductCapture` is installed only by `prepare_observed`, which has only test callers.
- The dispatch and `retained_memory.rs` are unchanged.
- "permit" occurs in the diff only in comments and in control A's no-permit assertion. There is no permit or test permit in maintained code (decision 7).

## 8. Mutants (`evidence/mutants_rv82.py`, `evidence/mutants_rv82_summary.txt`)

**The setup.** The mutants ran on a clean `git archive` of `59a5de2032` (WT/rv82/mut), with target WT/targets/rv82/mut. Afterwards I verified that each mutated file is byte-equal to the commit.

**The NONE control:**
- with I61's filter (`--lib -- u1_ u2_`): 16/16 ok;
- with the whole `--lib`: 472 ok, with only t13 failing.

**I61's 39 mutants**, run from its list verbatim with its filter: **39/39 killed**, with no compile-error kill. The verdicts and killing-test sets are identical to I61's summary.

**RV82's own 18 mutants** ran against the whole PP `--lib`, with t13 excluded. **10 were killed:**

| Mutant | Killed by |
|---|---|
| R01: a G-b member from the wrong report (`diagnostics.first()`) | 5 tests |
| R02a: D39 `not_required` folded into `not_eligible` at the capture | `u1_d39_route_capture` |
| R02b: D39 `not_required` folded into `not_eligible` on the wire | `u1_d39_legacy_dispositions` |
| R03: the D6a filter admitting `RETAINED_PRECISION_SELECTED` | the pinned bytes |
| R04: `owner_matches` on public facts only | `u2_foreign_owner_refused` |
| R05: a hash over non-canonical text | the pinned bytes |
| R07: A1 over the wrong bytes | the pinned bytes |
| R17: `legacy_attempted` always true | `u1_load_row_case_capture` |
| R18: G-d over any support at the node | 3 tests |
| R23: the selected diagnostic's severity | the pinned bytes |

**8 survived:**
- **R06** is S1.
- **R08, R09, R10, R13, R14 and R22** are N1: defensive or boundary checks with no committed negative.
- **R24** is N2: an equivalent mutant.

## 9. Nothing weakened, and the fence

**The diff from base touches 9 files:**
- `Cargo.toml`: the `[dev-dependencies]` `open_pipe_stress_result_export` path entry only.
- `Cargo.lock`: one path-package entry and one dependency line. Its dependencies were already locked, so no external crate is added.
- `lib.rs` and `retained_product.rs`.
- `retained_wire.rs` and `retained_wire_tests.rs` (new).
- `retained_product_tests.rs`: `.certificate` → `.certificate()` at 9 sites and one comment. The `I61_FOREIGN_OWNER` assertion is unchanged.
- `s11g_tests.rs`: one added test, accepted by ROOT.
- `final_case.rs`: one method.

**Unchanged:** `retained_receipt.rs` is in the fence but untouched. No reader, schema, fixture or existing check changes; the empty diff stat covers analysis_runs, reporting, apps, schemas and fixtures.

**The changed-file hashes** match I61's RETURN table, for the four files I mutated and restored.

## For ROOT

1. **S2:** the placement of the invocation binding. My recommendation is grant 2, alongside the refusal owner binding, before U3 wires the serializer.
2. **S1:** a one-line negative control in the grant-2 stream.
3. **N3, N4 and N5** carry to U3, grant 2 and G4 respectively.
