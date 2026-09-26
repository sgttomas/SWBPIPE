# T3 D2 — result standing, source envelopes and transport (revision 3)

Design TASK D2 (Type 2, HELPS_HUMANS-style design posture) for the T3 WORKING_ITEMS manager, 2026-09-26. **This is a proposal.** It becomes a basis only if ROOT selects it after V1's backcheck. Revision 1 is kept byte-for-byte as `_run_records/DESIGN_revision1.md` (sha256 prefix `185e178efe3b9855`), and revision 2 as `_run_records/DESIGN_revision2.md` (`5c36da2cb8cf89e6`, also in `DESIGN_revision2.sha256`). Revision 3 is narrow: §0.1 lists every change.

- **Brief:** `T3/TASK_BRIEFS/D2_STANDING_DESIGN.md` with `_COMMON.md`. Revision inputs at `065c9ff60`: `T3/REVIEW/RETURN.md` (V1), `T3/ROOT_RULINGS_V1.md` (ROOT's early rulings, plus the further rulings on R-3(a), R-3(b) and R-7 relayed by the manager), and `T3/MANAGER_NOTES/V1_DISPOSITIONS.md`.
- **Abbreviations:**
  - `P/` is `projects/chirality-piping/`; `T3/` is this tranche's records folder; `LSI/` is T1's `LOAD_STATE_IMPLEMENTATION/`;
  - `PP` is `P/core/product_physics/src/lib.rs`;
  - `D1` is `T3/DESIGN_NUMERICS/DESIGN.md`. D1 revision 1 (sha256 prefix `7199390f39f2c46d`) is the version read; D1 revision 2 was not yet available.
- **Bases.** Main `c61a540ea`; T1 candidate `f3270ea79`, read only through `git show` and `git diff c61a540ea...f3270ea79`. Line numbers are at those commits and will drift.
- **Design against T1 as merged.** Places where T1's final merge could change this design are marked **[T1-n]** (§10).
- **Nothing was built or run against product code** (host hold). The only runs are read-only, standard-library corpus scans (§12). The revised probes and their predictions are in `_run_records/`.
- **Probe addendum.** After ROOT released the host, the prepared probes PR-1 to PR-7 were run unchanged against exported copies of both bases. The results are in §12.1.
- **Revision 3 inputs:** `T3/REVIEW/BACKCHECK_R2.md` at `3ea78add8` (V1's backcheck of D1 revision 2: S8-R, S2-R, S5-R and the IF-1 confirmation), and ROOT's rulings on it at the end of `T3/ROOT_RULINGS_V1.md` (`5111651ed`), with the manager's relay of ROOT's final wording for gate condition 3. D1 revision 2 (`3ff9c1fd873d9f28`) §4.1.6, §4.4.1 and §5 were read; D1 revision 3 was not yet available.

## 0.1 Revision 3 — what changed and why

| # | Input | What changed | Where |
|---|---|---|---|
| 1 | V1 BACKCHECK_R2 S8-R; ROOT's S8-R ruling | **The floor is enforced by the readers.** G5 gains G5b, which recomputes S\* from the published rows, and G5c, which recomputes every quantity's verified-accuracy class as `abs(q) < fl(R·S*)`, with R's bits pinned. A mismatch is `unsupported`. A new consumer rule (§4.9.9) applies to `absolute_verified` and not-covered quantities: they are withheld from reliance (rule binding refused with a named reason, never counted as Passed, the headline refused when it points at one), shown as uncovered with their absolute bound, and carried with their class in every canonical and exported form. No exemption is proposed. | §4.9.3, §4.9.9, §4.7, §6.2, DD-13 |
| 2 | V1 S2-R; ROOT's final wording for gate condition 3 | The gate's standing condition now reads, as ROOT worded it: "the successor identity's standing is no worse than the retiring identity's, case by case, in all three languages", with identical, fail-closed standing in each language. H-a log-law cases are `needs_recompute` under both the joined identity and its successor, so they pass. Cited where S-E2, S-F and S-G2 depend on the gate. | §4.2.2, §4.5.2, §4.5.3, §4.9.5 |
| 3 | V1 confirms that D1 adopted IF-1 | **DD-11 is closed.** `numerical_quality` keeps the ordinary attempt's outcome, and the precision-p outcome lives only in the receipt. | §4.9.2, §5, §9 |
| 4 | V1 S5-R (D1 states the `digest_ok()` dependency) | **I-7 is closed:** W1 attempts require S-H's `digest_ok()`; otherwise the case gets `RETAINED_PRECISION_UNAVAILABLE` (`invocation_not_representable`). G4 admits that reason. | §5, §4.9.3 |
| 5 | Host release | The probe addendum (§12.1) is folded in unchanged. Its findings F-P2, F-P3 and F-P7 are recorded, not designed. | §12.1 |

The rest of revision 2, including §12.1, stands unchanged.

## 0. Revision 2 — what changed and why

| # | Input | What changed | Where |
|---|---|---|---|
| 1 | V1-S5; ROOT ruling 4 | **Capture refusal.** Invocation capture hashes the raw request with the checked profile, so any request with a finite value above 2^53 − 1 in magnitude is refused before any solve, on every route, on main and on T1. New slice **S-H** separates custody from the digest: the solve no longer depends on the digest, and a request the checked profile refuses solves ordinarily with no retained-source attempt. §3.6 and §4.6.2 are corrected, and so are the PR-5 predictions: PR-5a/5b now predict the capture refusal, PR-5d/5e are the receipt-hash cases, and PR-5f covers the ordinary route. Two facts limit any "switch the profile" fix: the scientific profile refuses host integers above 2^53 − 1 and negative zero. | §3.6, §4.6.2, §4.8, §6.1 |
| 2 | V1-S3; ruling 3; dispositions R-5 | **Successor-identity readers.** D2 owns these. New §4.9 defines the reader contract for D1's successor identities: receipt validation, the standing basis (a verified, invocation-bound receipt, never `numerical_quality` alone), the Current-admission set, schema and table slices, and three-language parity without kernel replay. New slice **S-G** lands atomically with D1's F2. | §4.9, §5, §7 |
| 3 | V1-S1; ruling 3; further ruling R-3(a) | **S-E split.** S-E1 is the resolved-case re-derivation core, built alongside D1's F3 and reusable for D1's 0.4.0 successor. S-E2 is the joined standing wiring, route test and carriers, built only if F3 will not land within T3 (R-3(a)), and designed for that window. | §4.2, §4.3, §7 |
| 4 | V1-S2; ruling 3; dispositions R-4 | **Shared retirement gate.** R-B and S-F cite D1's gate (coverage, budgets, three-language standing, value agreement) in place of revision 1's preview-only I-3. S-F's producer gate is superseded by D1's F2; S-F keeps the reader, text and test parts. | §4.5, §5 |
| 5 | V1-S6; ruling 5 | **Host-rounded fields.** The one-ulp tolerance is withdrawn. Recommended: a case whose re-derivation needs a host-rounded operation (`exp`, `exp_m1`: the `logarithmic_per_current_length` law) is `needs_recompute` with a named reason, decided from the invocation, not from values. The exact, host-independent alternative is specified and costed. | §4.2.1, DD-4 |
| 6 | V1-S10 | **Slice order.** S-C is serialized after S-A, since both write `physics_source.rs` and `physics_source.py`. | §7 |
| 7 | V1-N7 | **Scan claim.** Reworded, with a second scan: T1's SF-1 fallback raws (receipt-less `load-reference-1`) carry the diagnostic, on not-joined cases, as designed. | §3.4, §12 |
| 8 | Further ruling R-7 | **DD-7 option (i) adopted.** Historical all-selected source-blocks-1 stays Current, with the notice and the summary rule-binding refusal. The costs of both options are kept. `N_SB` and `N_SB_MIXED` are rewritten for (i). | §4.5.3, §9 |
| 9 | Further ruling R-3(b) | **Historical physics-source-1 stays eligible.** The static "fresh" sets are redefined as Current-admission sets, independent of emission. | §4.7, §4.9.6 |
| 10 | Revision 1's DD-1, DD-2, DD-3, DD-5, DD-6, DD-8, DD-9 | Kept, with statuses updated (§9). | §9 |

Unchanged in substance: F1 for composite finalization (S-D), the S-A tightening, the S-B display rule, S-C, the binding-route analysis, the comparison policy, and the parity method.

## 1. Recommendation in brief

| # | Item | Recommendation |
|---|---|---|
| 1 | Composite finalization | **F1 (S-D): port T1's SF-1 republication to both pre-0.4 source identities.** When a selected join cannot finalize, the invocation reruns in the same ledger, every successful attempt is declined, and every case publishes its ordinary response with its ordinary standing. The `Err` becomes an unreachable guard. Residuals R-1a and R-1b stay open until D1's F2 retires exact-block selection, or until F2 (the partial receipt) is chosen. S-D's reach is the window before D1's F2. |
| 2 | Joined eligibility | **S-E1**, a re-derivation core (Rust, Python, TS) built alongside D1's F3: the reader re-derives the resolved 0.4.0 operands from the actual invocation. **S-E2**, the joined standing wiring, is built only if F3 misses T3 (R-3(a)). Until then joined results stay `needs_recompute` (T1's early return). |
| 3 | Binding route | No new route. Eligibility makes results flow through `result_envelope_binding`. If S-E2 is built, T1's "no document" test becomes a positive route test with route-generated carriers. The successor identities use the same route (§4.9.7). |
| 4 | Selected-UNAVAILABLE | **Tighten** physics-source-1 and source-blocks-1 to the joined S13 rule (S-A). The successor readers adopt it from the start. |
| 5 | source-blocks-1 re-homing | **R-B:** D1's F2 retires exact-block selection for fresh solves under the **shared retirement gate**. Historical source-blocks-1 stays readable and byte-unchanged. Mixed envelopes stay `needs_recompute`; all-selected envelopes stay Current with the notice and the summary refusal (R-7 (i)). The texts stop promising T3. |
| 6 | Transport and display | **Display (S-B):** refuse a conversion that shows a nonzero value as zero or subnormal. **Capture (S-H, T3):** make the solve independent of the checked-profile digest. **Receipts:** a hashing failure is a finalization failure (S-D). **Canonical carriers and persistence:** T6 adopts `openpipestress_jcs_binary64_v1`, with the integer-literal and negative-zero rules stated. **Comparison policy:** no protected predicate changes. |
| 7 | Standing across languages | One shared case file per slice in Rust, Python and TS. The static sets become Current-admission sets. Five standing changes are listed in §4.7. |
| 8 | Successor-identity readers | **S-G:** a closed reader contract for D1's successor identities (§4.9). Standing comes from an invocation-bound, verified per-case receipt plus model-derived re-derivation, the same trust basis as physics-source-1. There is no kernel replay in the readers; replay is a Rust validation-lane audit. **Revision 3:** the readers recompute S\* and every quantity's accuracy class. Below-floor and not-covered quantities are shown uncovered, refused for rule binding and never counted as Passed (§4.9.9). |

**No owner decision is needed.** Technical decisions for ROOT are in §9.

## 2. Distinctions this design depends on

- **Publication versus standing.** A result can be published (readable, verifiable) without being Current. Standing is whole-envelope on every route: `numerical_use_standing*` needs every requested case to qualify (`semantic_contract.rs:367-453`; `compatibility.py:284-335`; `numericalResultQuality.ts:68-112`).
- **The finding standard, read against that rule.** "No invocation loses a correct result because another case failed" can mean that correct values stay published. It can mean that a correct case stays usable only with case-scoped standing (option F3), which no instrument defines.
- **Unreachable versus refused.** A reader rule that refuses a fact no producer emits changes no honest output. A producer path that becomes unreachable is kept as an invariant.
- **Re-derivation versus replay.** A reader re-derives the model-derived operands that enter the solve, from the actual invocation. It does not assemble, solve or replay. This is physics-source-1's trust basis (`physics_source.rs:896-974`). The receipt and publication hashes are unkeyed. They prove consistency and binding, not producer origin.
- **Correctly rounded versus host-rounded arithmetic.** `+ − × ÷ √` and fused multiply-add are reproducible bit for bit in every language. Host `exp` and `exp_m1` (T1 `case_state/thermal.rs:9-11, 484, 666-674`) are not.
- **Custody versus digest.** Custody means retaining the actual request Value. The digest is a hash of it under a canonical profile. Today the solve depends on the digest (`source_receipt.rs:64-73`). Only receipts need it.
- **Emission versus Current admission.** Whether fresh solves still emit an identity is separate from whether its results may be Current (ROOT R-3(b), R-7).

## 3. Current state, with citations

### 3.1 Composite finalization

- **The `Err`.** The value route returns `Err("SOURCE_BLOCKS_FINALIZATION_FAILED")` when a source identity has no receipt (`PP:1249-1253`). The invocation receipt failure path is `PP:1668-1680`.
- **The blocked envelope.** A selected case whose own finalization fails pushes a blocking diagnostic (`PP:2914-2926`). The case loop then returns a blocked `MODEL_INCOMPLETE` envelope (`PP:1430-1432`, `:1497-1499`, `PP:10718`).
- **Composite qualification.** The composite receipt requires every case to qualify (`source_receipt/composite.rs:938-947`), and `P/schemas/physics_source_recovery.schema.json` is closed to `qualified`. The non-composite receipt admits `partial` (`source_receipt.rs:859, 904`).
- **Other failure triggers:** the publication reservations (`source_receipt.rs:677-684, 715-722`), the invocation ledger of 64M (a committed N05 case charges 3.2M), and the receipt's checked hash (`source_receipt.rs:28-35`, `:751`, `:904-905`). A published value above 2^53 − 1 fails that hash (§3.6).
- **The recorded trigger.** P12 and P3 in T1's CP3 review (`LSI/REVIEW_CHECKPOINT_3/_run_records/review3_probes.rs.txt:124-134, 533-543`; `probes_final.log:32, 41`).
- **T1's SF-1** (`LSI/CP4_WIRE_ADDENDUM.md` §1.2):
  - the wrapper is at `lib.rs:1433-1466`;
  - failure recording is gated to 0.4.0 at `:1988-2000` and `:3494-3503`;
  - the decline is at `source_recovery.rs:217-229` and `lib.rs:2404-2440`;
  - the republication continues the same ledger (ROOT N-2; `LSI/REVIEW_CHECKPOINT_4/BACKCHECK.md`).

  The pre-0.4 `Err` is pinned by `the_pre04_composite_finalization_failure_is_unchanged_by_this_fallback` (`source_receipt/load_state_fallback_tests.rs:293-312`). V1 confirmed that F1 is faithful to this code (V1 §4).

### 3.2 Joined eligibility

- **The T1 joined reader.** J0 identity, J1 the S1–S13 pre-pass, J2 the receipt, J3 `physics_source::validate(&projected, None)`. It never passes an invocation and returns `Ok(false)` (`load_reference_source.rs:29-31, 76-78`). Python and TS mirror it.
- **Standing.** The declared early `needs_recompute` is at `semantic_contract.rs:470-476` and `compatibility.py:335-339` (T1). ROOT §7 routes eligibility to T3.
- **The physics-source-1 analogue.** It re-derives E, ν and α from the pre-0.4 case modulus basis (`physics_source.rs:809-974, 1128-1141`; `physics_source.py:371-425`). That cannot serve 0.4.0: 0.4.0 selects materials per member and interpolates with fused `mul_add` (`case_state/material.rs:320` at T1).
- **What the records already carry.** The joined record carries the resolved intermediates (`load_reference.rs:36-150` at T1; `LSI/CP2_WIRE_ADDENDUM_1.md` §3), bound by `physical_evidence_sha256` (`load_reference_source.rs:166-180`). The readers do not check `[T_lower, T_upper]` containment (`CP4_WIRE_ADDENDUM.md` §2).
- **ROOT R-3(a).** Joined stays fresh until D1's F3 lands.

### 3.3 Binding route

- `build_result_export_document_with_evidence` gates on standing only (`result_envelope_binding.rs:236-270`, gate `:258-260`). V1 confirmed this.
- The actual-solve route mints `QualifiedPreviewEvidence` (`headless/src/lib.rs:731-756`).
- `derive_document` accepts the joined identity (T1 `derivative.rs:97-116, 326-334`).
- T1 pins the absence of a joined document (`load_reference_route_tests.rs:473-598`). The committed joined carriers come from the test-side `derive_document`.

### 3.4 Selected-UNAVAILABLE

- **Where the joined rule lives.** S13 `JOIN_SELECTED_UNAVAILABLE_DIAGNOSTIC` is at `load_reference.rs:596-618`, `load_reference_evidence.py:416` and `loadReferenceEvidence.ts:397` (all T1).
- **Where it does not.** physics-source-1 and source-blocks-1 readers check no such code in any language.
- **The single emitter.** It is the attempt's `Err` arm, where the case is never selected (`PP:1888-1897`; T1 `lib.rs:2425-2438`). V1 confirmed this.
- **Corpora** (revision 2 wording, V1-N7):
  - no committed receipt-bearing envelope carries the diagnostic (main 0 of 43, T1 0 of 83);
  - on main, no committed JSON outside the records carries it at all;
  - at T1, four committed JSON files carry it. Two are the SF-1 fallback raws `result_export/tests/fixtures/load_reference_fallback_uz-{dense_scrutiny,sparse_interactive}.raw.json`, which are receipt-less `load-reference-1` envelopes with the diagnostic on not-joined cases, exactly as SF-1 designs. The other two are the mutation case files `load_reference_mutations.json` and `load_reference_source_mutations.json`.
  - No committed envelope carries the diagnostic on a *selected* case.

### 3.5 source-blocks-1 carries (T0R R-1, R-2)

- **Selected plus ordinary.** The envelope reads `needs_recompute` with `SOURCE_BLOCKS_ORDINARY_CASE_LEGACY_SEMANTICS`:
  - Rust: `semantic_contract.rs:329-338`, `source_blocks.rs:657-667`;
  - Python: `compatibility.py:338-343`, `source_blocks.py:450`;
  - TS: `knownSemanticLimitations.ts:53-61`, `sourceBlockRecovery.ts:285-300`.
- **All-selected.** The envelope is Current, and its summary is refused for rule binding with `RULE_SOURCE_BLOCKS_SUMMARY_NOT_RELIABLE`:
  - Rust: `semantic_contract.rs:340-352`;
  - Python: `compatibility.py:364-380`;
  - TS: `knownSemanticLimitations.ts:63-68` via `ruleCheckService.ts:327`;
  - native: `src-tauri/src/lib.rs:3206-3216`;
  - `rule_check_runner`: `lib.rs:93`.
- **What the selected rows publish.** Selected cases already publish six signed `support_reaction_component_v2` rows. Their summary is the abs-sum.
- **User texts.** `N_SB` and `N_SB_MIXED` are at `knownSemanticLimitations.ts:20-21`, a file T1 edits. The Python docstring at `compatibility.py:365-369` also says "until T3".

### 3.6 Transport, capture and display

- **Display chain.** `QuantityReadout` (`features/display-units/index.tsx:95`) → `displayQuantityService.ts` → Tauri or WASM `convert_display_quantities` (`operation_applier/src/display_units.rs:7-33`) → `units::convert_for_dimension` (`units/src/lib.rs:917-950`).
  - Only a non-finite result is refused (`display_units.rs:27-28`).
  - `unitConversion.ts:68-85` prints "0" for an underflowed value.
  - None of these files is changed by T1.
- **The checked limit.** Every finite binary64 above 2^53 in magnitude is integral, and the checked guards refuse every |x| > 2^53 − 1:
  - `derivative::guard_json` (`derivative.rs:13-19`);
  - the checked profile (`canonical_json/src/lib.rs:94-108`);
  - Python (`adapter.py:40-46`);
  - TS (`resultExportAdapter.ts:25`);
  - the receipt hash (`source_receipt.rs:28-35`).
- **Capture (corrected in revision 2; V1-S5).** `run_linear_static_preview_value_with_mode` calls `CapturedInvocation::parse` before parsing the request (`PP:1239-1240`). That hashes the *raw request* with the checked profile (`source_receipt.rs:64-73`). Both production callers propagate the error:
  - desktop: `src-tauri/src/lib.rs:1557-1563`;
  - headless: `headless/src/lib.rs:737`, `:865`.

  T1 does the same (`lib.rs:1414-1415`). Consequences:
  - any request containing a finite value above 2^53 − 1 in magnitude (for example a 1e16 N/m spring) is refused before solving, on **every** route, ordinary included;
  - a desktop request carries such a value as an integer literal, because JS formats 1e16 as `10000000000000000`, which serde reads as a `u64`. The refusal is then `CHECKED-JSON-UNSAFE-INTEGER`. A float literal gives `CHECKED-JSON-UNSAFE-INTEGRAL-FLOAT`;
  - no value is silently altered. Every path is a refusal.
- **Ordinary results with such published values.** They keep their raw envelope, but the headless evidence digest fails (`headless/src/lib.rs:741-745`, `derivative::digest` → `guard_json`). They get no canonical document or AnalysisRun (`canonical_export_unavailability`).
- **The scientific profile** `openpipestress_jcs_binary64_v1` (`canonical_json/src/binary64.rs`) is standalone. Its value path refuses:
  - host integers (`i64`/`u64`) above 2^53 − 1 (`binary64.rs:196-203`, `UNSAFE-HOST-INTEGER`);
  - negative zero (`:207-209`, `NEGATIVE-ZERO`);
  - documents above 8 MiB or 262,144 nodes (`:8-10`). T1's VP-STATIC shakedown recorded runner outputs of 14.1 MB and 21.9 MB (`LSI/T1_WAVE1_RULINGS.md` §9).

  The checked profile admits negative zero and renders it `0`. Python reaches both profiles through the Rust CLIs (`adapter.py:94-104, 244-307`). TS has only the checked hash (`services/hashService.ts:111`).
- **Protected comparison.** The mechanics predicate is absolute 1e-9 (`validation/benchmarks/mechanics/src/lib.rs:1437-1441`). It stays unchanged (`STAGE1_PLAN.md` §8.4).

## 4. Options and the recommended design, per item

### 4.1 Composite finalization (S-D)

#### 4.1.1 Options

| | Option | Publishes on failure | Contract change | Standard |
|---|---|---|---|---|
| **F1** | **Port SF-1**, same ledger | physics-1 or preview-physics-1. Ordinary rows; `UNAVAILABLE` naming the cause; ordinary standing | None | Standing: yes. Publication: yes, except R-1a and R-1b |
| F2 | Partial receipt under the source identity | Selected rows kept; failed cases ordinary; `needs_recompute` | physics-source-1 needs a new receipt policy (its schema is closed to `qualified`), producer `failed_physics` cases, readers ×3, schema branches | Both, in the publication sense |
| F3 | F2 plus case-scoped standing | As F2; a qualified subset can be eligible | F2 plus subset standing everywhere and consumer adoption | Both senses. Product semantics that no instrument covers |

#### 4.1.2 Why F1

- **F1 and F2 have identical standing.** F2's gain is inspection of the recovered values, and publication in the N06 sub-case.
- **F2 would be retired early.** D1's F2 slice retires exact-block selection for fresh solves (D1 §4.4), so a new physics-source receipt policy would be built only to be retired.
- **F1 is never worse than today,** and it is byte-identical everywhere else (§6.3).
- **One rule governs all source identities:** "a selected join publishes only when its receipt finalizes; otherwise the invocation publishes ordinarily".

#### 4.1.3 Mechanism (`PP`, after T1's merge)

1. **Wrapper.** The clone condition becomes: capture digest available (S-H) && `load_state_join_withheld.is_none()` && no combinations && no nonlinear support record. The rest of the wrapper is T1's: at most two runs, same ledger. **[T1-1]**
2. **Decline.** The pre-0.4 arm (T1 `lib.rs:2404`, `None => Ok(recovery)`) declines when `load_state_join_withheld` is set. The replay-reservation screen stays 0.4.0-only, because porting it would change committed pre-0.4 selection.
3. **Failure recording.** Drop the 0.4.0 gates on `record_load_state_join_failure` (T1 `lib.rs:3494-3500`, `:1988-1994`).
4. **Diagnostic text.** The `UNAVAILABLE` suffix naming the withheld join applies to pre-0.4 too.
5. **The `Err` line** (`PP:1249-1253`) stays as an unreachable guard, with its text unchanged.
6. **What the republication publishes.** Exact 0.3.0 models publish physics-1; other pre-0.4 models publish preview-physics-1 (rendered, because `source_selected` is false, `PP:1504`). Rows are bit-identical to the typed route. Each declined or failed case carries one `UNAVAILABLE`.

#### 4.1.4 The T1 characterization test

It is replaced in place, not deleted **[T1-8]**. The new name is `the_pre04_composite_finalization_failure_republishes_every_case_ordinarily`, on the P12 input, in both modes. It asserts:
- `Ok` and identity physics-1;
- no receipt, `MECHANICS_SOLVED`, and no blocking diagnostic;
- rows bit-identical to the typed route;
- `UNAVAILABLE` on both cases, with "invocation join withheld" on `case`;
- quality `sensitive`;
- standing `needs_recompute` in Rust and Python, and not eligible in TS.

The S-D record carries the old name and assertion, the file hashes before and after, the reason, and the restoring mutant (which must fail).

Companion triggers:
- 21 N05 cases (PR-3);
- a private case limit (T1's pattern);
- a published value above 2^53 − 1 with a safe request (PR-5d, PR-5e; V1-S5).

#### 4.1.5 Residuals

- **R-1a.** A selected case's recovered values are replaced in the fallback by its ordinary sensitive values, which are not Current.
- **R-1b.** A selected case whose own ordinary attempt was rejected (N06) gives a blocked envelope that names the cause. Today that input returns `Err` (PR-2).

Both close when D1's F2 stops fresh exact-block selection, provided D1's method covers those cases (the shared gate's coverage condition includes N05 and N06). They also close with option F2.

### 4.2 Joined eligibility: S-E1 (core) and S-E2 (conditional wiring)

#### 4.2.1 S-E1: `RESOLVED-CASE-REDERIVATION-v1`

S-E1 lives in new files:
- `P/core/reporting/result_export/src/load_reference_rederive.rs`;
- `P/core/analysis_runs/load_reference_rederive.py`;
- `apps/desktop/src/features/results/loadReferenceRederive.ts`.

It is a library with one entry per language: `rederive_resolved_cases(invocation) -> ResolvedCases | Scope(reason)`, plus a comparison `compare_resolved(records, resolved) -> Ok | Mismatch(code)`. It is consumed by S-E2 (joined standing, if built) and by S-G for D1's 0.4.0 successor (§4.9.5). S-E1 alone changes no standing.

**Source discipline.**
- It is written from T1's wire records (`CP2_WIRE.md`, `CP2_WIRE_ADDENDUM_1.md` §3–5, `CP3_WIRE_ADDENDUM.md` §1–2, `CP2_WIRE_ADDENDUM_2.md` §5, and any later accepted addendum **[T1-3]**), not from `case_state/*`.
- Step RD-0 comes before any code. The implementer lists every re-derived field and its specified arithmetic. Silences in the records go to the manager as addendum requests.
- The independent reviewer checks this discipline.

**Arithmetic.**
- Correctly rounded operations and fused multiply-add are bit-exact:
  - Rust uses `f64::mul_add`;
  - Python 3.11 has no `math.fma`, so it uses an exact `fractions.Fraction` product-sum rounded by `float()`, which is correctly rounded in CPython;
  - TS uses an exact BigInt rational with round-half-even.
- Temperature identity uses exact rationals from the shortest round-trip decimal and the affine unit definition, with checked i128 arithmetic as the producer states (`case_state/temperature.rs:1-11` at T1). An overflow is a scope refusal.

**Host-rounded operations** (revision 2, V1-S6; DD-4). Recommended rule **H-a (refuse):**
- If any member of a case uses a definition whose value needs host `exp` or `exp_m1` (today only `logarithmic_per_current_length`, `CP3_WIRE_ADDENDUM.md` §2), the re-derivation returns `Scope("SOURCE_LOAD_REFERENCE_REDERIVATION_HOST_ROUNDED")` for that case.
- The decision is made from the invocation's law *definition*, not from any value, so every language and host decides identically.
- Consequence: those cases are `needs_recompute`. It is a coverage limit, not a wrong value.

The alternative, **H-b (exact bracketing)**, is host-independent:
- The argument `a` is re-derived bit-exactly. The recorded value `r` is accepted iff `pred(r) < exp(a) < succ(r)`, the faithful-rounding condition, decided with a rigorous rational enclosure of `exp(a)` (Taylor terms with a remainder bound, and range reduction by an exact power of two). The same test on `exp(a) − 1` handles `exp_m1`.
- `exp(a)` is irrational for rational `a ≠ 0` (Lindemann), so it never equals the rational endpoints, and refinement terminates.
- The implementation fixes a precision schedule (128, 256, 512, 1024 bits). If the enclosure is still undecided at 1024 bits, the case is a scope refusal, and that outcome is identical in every language because the algorithm is the same. For `a = 0`, `r` must be exactly 1 (or 0 for `exp_m1`).
- Every downstream value is then re-derived from the recorded `r`.
- **Cost:** a rational or big-integer implementation in each language. Python has `Fraction` and TS has BigInt, but `result_export` has no big-integer type, so Rust needs an in-crate one or a new dependency (a lockfile change).
- H-b is the path if ROOT wants logarithmic-law coverage. Its implementation can reuse D1's in-repo `wide.rs` only if it becomes a shared crate, which is not proposed here.

**Checks** (J4.2–J4.8, as in revision 1):

| Check | What it compares |
|---|---|
| J4.2 | 0.4.0 document; normalization to SI |
| J4.3 | Per-member selection, points, fraction, E, ν, G, the temperatures, the law identity, the consumed segments, the stretches, the strains, `total_eigenstrain`, the fit, `reference_length_m` |
| J4.4 | Admissibility (coverage, positivity) evaluated by the reader, plus the new `[T_lower, T_upper]` containment of every consumed and consulted segment |
| J4.5 | Support motions |
| J4.6 | The contribution ledger and excluded sources |
| J4.7 | `exact_cases[i].pipe_materials[j]` against J4.3 |
| J4.8 | The reference-geometry projection digest, if RD-0 finds its specification |

Numbers compare by binary64 value (T1's `same`, `load_reference.rs:278-290`).

**Scope.** The whole `resolved_straight_load_state_v1` profile, minus the H-a cases. Anything else the reader cannot re-derive gives `Scope(reason)`.

**Schedule.** Alongside D1's F3 (ROOT R-3(a)), in the same atomic PR as the S-G branch for D1's 0.4.0 successor (§4.9.5).

#### 4.2.2 S-E2: joined standing wiring (conditional)

- **Condition** (ROOT R-3(a)): S-E2 is built only if D1's F3 will not land within T3. The manager records this in the work graph. The decision point is ROOT's T3 closure review, or the scheduling of F3, whichever comes first.
- **Window.** If S-E2 is not built, joined results stay `needs_recompute` through T1's early return, both while joined is fresh (T1 merge to F3) and afterwards for historical joined envelopes. That is safe.
- **If built:**
  - it removes T1's early returns **[T1-2]**;
  - joined joins the source-identity branch with J4 = J4.1 (the invocation, through the projected physics-source-1 validation with `MaterialCheck::External` from S-C) plus S-E1;
  - standing:
    - `numerically_eligible` needs J0–J4 to pass, an invocation, the aggregate `qualified`, no combinations, `MECHANICS_SOLVED`, and requested refs equal to the receipt order;
    - `needs_recompute` applies with no invocation, with a `Scope` result (including H-a), or with differing requested refs;
    - `unsupported` applies on a mismatch;
  - TS gets a check-for-check port with a validated token (DD-3 TS-a).
- **After F3.** Once F3 lands and joined stops being fresh (R-3(a)), historical joined envelopes keep the standing S-E2 gave them. Joined has no known defect, so ROOT's R-3(b) logic for physics-source-1 applies by analogy. That is decision DD-10.
- **The gate at F3** (revision 3). The joined family retires under D1's shared gate (§4.5.2). Its standing condition is "the successor identity's standing is no worse than the retiring identity's, case by case, in all three languages". S-E2 does not change what that condition compares:
  - without S-E2, the retiring joined identity reads `needs_recompute` for every case, so any fail-closed successor standing passes;
  - with S-E2, a joined case is eligible exactly where J4 re-derivation succeeds, and the successor (S-G2, using the same S-E1 core) must be at least as good on each such case;
  - under H-a, log-law cases are `needs_recompute` under both identities, and pass.

#### 4.2.3 The TypeScript position

- **Recommended: TS-a,** a check-for-check port. This is T1's pattern, and parity with the native Rust rule gate requires it.
- **Alternative: TS-b,** a native-delegated verdict. It needs one implementation fewer, but a browser session gets `needs_recompute` and it adds an IPC contract (DD-3).
- Under H-a, the TS port needs no transcendental arithmetic.

### 4.3 Binding route

- **No code change** in `result_envelope_binding.rs`. Its gate (`:258`) and `derive_document` handle any eligible identity.
- **If S-E2 is built:**
  - `load_reference_route_tests.rs::joined_actual_solve_retains_invocation_bound_receipt_without_canonical_export_both_modes` is replaced in place by `joined_actual_solve_mints_bound_evidence_and_canonical_document_both_modes` **[T1-4]**, with revision 1's positive and negative controls;
  - route-generated carriers `fixtures/results/load_reference_source_route_*.document.json` are added, generated only by the actual headless route;
  - the committed test-origin carriers stay unchanged.
- **If S-E2 is not built:** T1's test stays as a characterization. Its "T3 work" comment is updated to cite ROOT R-3(a).
- **The successor identities** use the same route (§4.9.7).
- **The headless evidence digest** (`headless/src/lib.rs:741-745`) is export-side, so it stays with T6 (§4.6.2).

### 4.4 Selected-UNAVAILABLE alignment (S-A)

- **Meaning.** An `UNAVAILABLE` on a selected case contradicts the receipt and the `SELECTED` diagnostic. No producer emits the pair (§3.4).
- **Direction: tighten.**
  - physics-source-1 and source-blocks-1 refuse an `UNAVAILABLE` whose `affected_refs` contains a case with `selected_method == retained_source_blocks_exact_v1`, with the code `SOURCE_BLOCKS_SELECTED_UNAVAILABLE_DIAGNOSTIC`.
  - The check goes after `EXACT_SOURCE_REQUIRED` in each language (`source_blocks.rs::validate_in`, shared with the composite path; `source_blocks.py`; `sourceBlockRecovery.ts`; `physicsSourceRecovery.ts` if its path is separate).
- **Effect.**
  - No committed receipt-bearing envelope changes outcome.
  - T1's fallback raws are receipt-less `load-reference-1`, outside these readers.
  - A resealed contradictory envelope goes from eligible (with an invocation) to `unsupported`.
- **The alternative, relaxing joined S13,** would weaken a T1-qualified reader in three languages.
- The successor readers adopt the same rule from the start (§4.9.3).

### 4.5 source-blocks-1 re-homing

#### 4.5.1 Options

| | Option | Fresh solves | Carries | Status |
|---|---|---|---|---|
| **R-B** | **Retire exact-block selection for fresh solves under the shared gate** (D1's F2) | The successor identity where D1's method selects; preview-physics-1 otherwise | Moot for fresh results. Historical: mixed stays `needs_recompute`; all-selected stays Current with the notice and refusal (R-7 (i)) | Recommended; aligned with D1 D-4 option A |
| R-A | Successor identity `preview-physics-source-1` (exact-block under preview-physics-1 semantics) | Selected cases get the circular maximum and six components | Fixed for fresh results | Superseded by D1's successor identities; not recommended |
| R-0 | Keep T0R's containment | source-blocks-1 as today | Stay; texts reworded | Only if the shared gate is not met within T3 |

#### 4.5.2 The shared retirement gate (cited, not defined here)

D1 defines the gate (ROOT ruling 3; dispositions R-4). Its four conditions, as V1 stated them:

1. **Coverage:** every committed `fixtures/product_preview/{source_blocks,physics_source}` request, solved fresh in both modes, is selected by D1's method.
2. **Budgets:** the D-8 limits admit them.
3. **Standing** (ROOT's final wording, revision 3): "the successor identity's standing is no worse than the retiring identity's, case by case, in all three languages". On top of that, each language's standing must be identical and fail-closed across the whole family. It does not require every case to become eligible. A declared out-of-scope subset reads `needs_recompute` under both identities, and passes. The H-a log-law cases under the joined identity are the example. The successor standing comes from S-G (§4.9), so S-G is a precondition of the gate.
4. **Values:** the projections match exact-block within the unchanged 1e-9.

R-B, S-F and residuals R-1a and R-1b cite this gate. If it is not met within T3, ROOT chooses R-0 (DD-6). Under R-0, the texts are reworded to "until the general accuracy method is available", so that they never promise a tranche that will not deliver.

#### 4.5.3 S-F (after the gate is met; with or after D1's F2)

1. **Producer.** Superseded by D1's F2, which stops exact-block selection for fresh solves on both routes. S-F adds no producer gate. **[T1-1]** The family retires only when D1's shared gate passes (§4.5.2). Its standing condition (ROOT's wording) is "the successor identity's standing is no worse than the retiring identity's, case by case, in all three languages". On the committed source-blocks-1 requests, an all-selected envelope is Current today, so the `<preview-retained>` result for the same request must be Current too. A mixed envelope is `needs_recompute` today, so any fail-closed successor standing passes.
2. **Standing** (R-7 (i)):
   - mixed envelopes keep `SOURCE_BLOCKS_ORDINARY_CASE_LEGACY_SEMANTICS`;
   - all-selected envelopes stay Current, and `rule_binding_refusal` keeps refusing the summary and the headline's row;
   - source-blocks-1 **stays** in the Current-admission sets (§4.7);
   - no new standing reason is added.
3. **Why option (ii) is not taken** (ROOT R-7). The abs-sum summary is the only known-defective quantity, and rule binding already blocks reliance on it. The other quantities are correct. Demoting all-selected envelopes to historical-only would withhold correct results with no correctness gain.
   - Cost of (i): source-blocks-1 stays in three Current-admission sets, and the binding refusal stays at every binding site, including any future one (T6).
   - Cost of (ii): a new standing reason in three languages, removal from the sets, and migration of the source-blocks-1 standing tests.
4. **Bytes.** Historical bytes, schemas, tables and fixtures stay unchanged and readable. A tampered envelope stays `unsupported`.
5. **Texts** (`knownSemanticLimitations.ts:20-21`, a T1-overlap file **[T1-5]**; and the Python docstring `compatibility.py:365-369`):
   - **`N_SB`:** "Summary stress in this retained-source result is the sum of absolute axial and bending components, not the circular-section maximum. The admitted loads are nodal only, so it is conservative and at most √2 (about 1.414) times the maximum. Rule checks cannot bind to it. Solve again to obtain the circular-section maximum."
   - **`N_SB_MIXED`:** "This retained-source result contains an ordinary load case whose rows keep the retired precision-1 semantics (norm-only reactions and an absolute-sum stress summary). It is not Current, rule- or export-eligible. Solve again to obtain signed support reactions and the circular-section maximum."
   - **The Python docstring** becomes: "…so no rule may bind to it; fresh solves no longer emit source-blocks-1."
   - `RULE_SOURCE_BLOCKS_SUMMARY_NOT_RELIABLE` stays. No text names a tranche.
6. **Tests.** Producer-driven source-blocks-1 tests migrate with D1's F2 (§6.4). Reader and standing tests on the historical fixtures are unchanged.

### 4.6 Transport, capture and display

#### 4.6.1 Display representability (S-B; before T1's merge)

A conversion is `converted` only for one of:
- an exact zero (either sign; displayed as "0");
- the identity (the input bits are returned);
- all-normal input, canonical intermediate and output.

Otherwise it is `unavailable` with `DISPLAY_UNIT_RANGE: nonzero value is not representable as a normal binary64 in <unit>; shown in <stored unit>`.

- The rule sits in `display_units.rs` around `convert_for_dimension`; the units API is unchanged.
- `unitConversion.ts::convertForDisplay` returns `null` (entered-only) under the same rule.
- `QuantityReadout` already falls back with a notice.
- For normal operands the relative error is below 2.3e-16. Affine temperature conversions are not refused for relative loss.
- Tests are as in revision 1 (PR-6 items).

#### 4.6.2 The 2^53 − 1 limit: capture, receipts and carriers (corrected in revision 2)

| Boundary | Owner | Design |
|---|---|---|
| **Invocation capture** (`PP:1239-1240` → `source_receipt.rs:64-73`) | **T3** (ROOT ruling 4) | **S-H** (§4.8): custody without a required digest. The request is solved whether or not the checked profile admits it. Retained-source attempts need the digest. |
| **Source receipt hashing** (existing identities) | T3 | A receipt that cannot be hashed (checked-profile refusal of a published value, byte limits) is a finalization failure, which S-D republishes ordinarily. Receipts keep declaring `openpipestress_jcs_ijson_v1`; their bytes do not change. |
| **Successor receipts** (D1) | D1 defines, S-G reads | D1 revision 2 names the profile and the bit-string encoding (V1-S4). The reader contract (§4.9.3) requires that a successor receipt can always be hashed for any value the envelope can publish: large or out-of-profile numbers go into 16-hex bit strings (as `source_receipt.rs:36-38` does), and the per-case outcome of a hashing failure is stated. |
| **Canonical results documents, AnalysisRuns, stress-neutral packages, the headless evidence digest, persistence, desktop export** | **T6** (DD-8) | Adopt `openpipestress_jcs_binary64_v1` for result-value positions. Exact-integer positions keep integer checks. The per-schema inventory and a differential proving identical text wherever both profiles admit are required (PR-7 is the pre-check). T6 must also state three rules, which the scientific profile's value path refuses today (§3.6): the **integer-literal rule** (a result value serialized as a `u64` above 2^53 − 1 must be declared a binary64 real), the **negative-zero rule** (sign-normalize, or declare the producer never publishes −0), and the **size-limit rule** (the 8 MiB and 262,144-node limits against 14–22 MB outputs). |

**T3's range requirement** (M34): every finite binary64 input or result either survives capture, solve, raw publication, readers, display and transport bit for bit, or is refused with a specific reason. It is never silently altered.

**T3's evidence:**
- RF-RANGE cases (R1) extend to request values and published values in `[2^53, 1e21)` and above, and to subnormals;
- P1 records where each is carried or refused;
- the S-H tests show that a 1e16 N/m spring solves on every route.

#### 4.6.3 Comparison policy

- Numerical references use `|obs − exp| ≤ 1e-9 · max(|exp|, scale)`, with stated zero scales. That is ROOT §8.4.
- Transport and display cases compare bits or the representability class.
- Parity cases compare outcomes and codes exactly.
- No protected predicate changes.

### 4.7 Standing across languages

**The Current-admission sets** (revision 2).
- The constants `FRESH_IDENTITIES` (Rust), `FRESH_CONTRACT_IDS` (Python) and `FRESH_SEMANTIC_CONTRACT_IDS` (TS) keep their names, to avoid churn at every call site.
- Their doc comments and the T0R pins are redefined as "identities whose results may be Current. Membership is independent of whether fresh solves still emit them" (ROOT R-3(b), R-7).
- precision-1 stays out.

| Change | Rust (`semantic_contract.rs`) | Python (`compatibility.py`) | TS | Slice |
|---|---|---|---|---|
| S-1 successor identities | Add each successor id to the Current-admission set and to a source-like standing branch using the §4.9 validator with the invocation | same | `numericalResultStanding` → the successor standing token | S-G (atomic with D1's F2; the 0.4.0 successor with F3) |
| S-2 selected-UNAVAILABLE | Validator refusal → `unsupported`; no standing-function edit | same | same | S-A |
| S-3 joined (conditional) | Remove T1's early return; add the joined branch | same | same | S-E2, only if built |
| S-4 retirement | None: source-blocks-1 and physics-source-1 stay in the sets (R-3(b), R-7 (i)); doc comments only | same | same | S-F |
| S-5 composite fallback, capture | None (existing identities) | none | none | S-D, S-H |
| S-6 verified-accuracy classes (revision 3) | `rule_binding_refusal` returns `RULE_QUANTITY_BELOW_VERIFIED_FLOOR` or `RULE_QUANTITY_NOT_COVERED` for classified rows of successor envelopes. Envelope standing unchanged | `rule_binding_refusal` same | `ruleBindingRefusal` same; row labels and notices | S-G1, S-G2 |

**Parity.** Each slice has one shared JSON case file (envelope, invocation or null, requested refs, expected standing, expected reason or code) consumed by all three languages. The parity summary records exact agreements, declared language-specific strings, zero undeclared differences, and zero cases accepted by one language and refused by another. Each new branch has a mutant per language, killed by a named case.

### 4.8 Capture: custody without a required digest (S-H; new in revision 2)

**Today.** `CapturedInvocation::parse` computes `encoded_len` and the digest with the checked profile, and returns `Err` before the request is parsed (`source_receipt.rs:64-73`). Any request the checked profile refuses therefore cannot be solved.

**Options.**

| | Option | Effect | Problem |
|---|---|---|---|
| **H-1** | **Fallible digest.** `CapturedInvocation` keeps the raw Value and mode. `digest` becomes `Result<String, ReceiptError>`. `parse` fails only on a request that does not parse. | Every parsable request solves. Retained-source attempts, and the joined replay, require `digest.is_ok()`. Otherwise a case that would have attempted gets one `SOURCE_BLOCK_RECOVERY_UNAVAILABLE` info ("invocation not representable in `openpipestress_jcs_ijson_v1`: <reason>") and publishes ordinarily. For 0.4.0 its record reads `retained_source_attempt=unavailable` | None found. Existing receipt bytes are unchanged, because an admitted request is hashed exactly as today |
| H-2 | Hash capture with the scientific profile | Admits floats above 2^53 − 1 | Still refuses integer literals above 2^53 − 1, which is what the desktop sends; refuses negative zero; adds an 8 MiB and 262,144-node refusal. It changes the receipt's declared canonicalization, and so existing receipt bytes |
| H-3 | Normalize numbers before hashing (declare every generic number a binary64 real) | Admits everything finite | Hashes something other than the actual raw request, which is a custody change; changes receipt bytes |

**Recommended: H-1.** Its mechanics:
- **Where.** `source_receipt.rs` (`CapturedInvocation`, and `encoded_len` computed from the checked text when available). In `PP`: the attempt gate (`PP:1885`; T1 `lib.rs:2391`) adds `capture.digest_ok()`, and the S-D wrapper uses it. All of these are T1 files, so S-H comes after T1's merge, before or with S-D.
- **Readers.** The republished or ordinary envelope carries an `UNAVAILABLE` info on an unselected case, which every ordinary identity's readers already admit (V1 confirmed the ROOT_SELECTION F-1 reference classes). No reader changes.
- **Digest users.** A receipt is never built without a digest; finalization would refuse anyway, and S-D covers it. Desktop and headless callers are unchanged: they now receive `Ok` envelopes where they received `Err`.
- **The headless evidence digest still fails** for such requests, so canonical export is unavailable. That is T6's carrier (DD-8), and the unavailability reason says so.
- **Tests.** In both modes:
  - a request with a 1e16 N/m spring given as an integer literal solves on the preview route, the exact route and (after T1) the 0.4.0 route, with rows bit-identical to the typed route;
  - a sensitive case in such a request carries the capture `UNAVAILABLE`;
  - every committed request produces byte-identical output (the admitted-request invariance).
- **Successor identities.** D1's method uses the same capture. D1 decides whether its receipt needs H-1's digest. The reader contract requires only that a receipt names the profile its invocation digest used (§4.9.3).

### 4.9 Successor-identity readers (S-G; new in revision 2)

D2 owns these readers (ROOT ruling 3, dispositions R-5). The interface is D1 §5 (revision 1), with D1 revision 2's canonical profile and bit-string encoding (V1-S4). Where D1 revision 2 differs, S-G follows D1 and keeps the rules below.

#### 4.9.1 Identities (placeholders; ROOT reserves)

| Successor | Inherits rows and evidence of | Route | Slice |
|---|---|---|---|
| `<preview-retained>` (D1 suggests `…/preview-physics-retained-1`) | preview-physics-1 (table, `contract_evidence`, reader checks) | Ordinary pre-0.4 | S-G1, atomic with D1's F2 |
| `<physics-retained>` (`…/physics-retained-1`) | physics-1 (`contract_evidence.exact_cases`, `pressure`, `connector`) | 0.3.0 exact | S-G1, atomic with D1's F2 |
| `<load-reference-retained>` (0.4.0; name for ROOT) | load-reference-1 (`load_reference_states` records) | 0.4.0 | S-G2, atomic with D1's F3; uses S-E1 |

Each successor has its own semantic table. Following the source-blocks-1 precedent, the table declares `inherited_semantic_contract_sha256` for its base table, plus its own profile, a receipt policy (placeholder `M03-INTEGRITY-MP-1`) and a receipt schema.

#### 4.9.2 Envelope shape the readers require

- The base identity's envelope, plus one closed top-level member (placeholder `retained_precision`) with a `body` and a `receipt_sha256`.
  - The body holds: `receipt_version`, `policy`, `canonicalization` (the profile D1 names), the invocation digest (algorithm, profile, payload scope, value), `publication_sha256`, and `cases[]`.
  - There is exactly one entry per load case, in the order of the request and of `numerical_quality.cases`.
- Each case entry is one of:
  - **`selected`:** D1 §5.1's closed receipt (method token, policy, source identity digest, attempts list, selected p and verification p, the stop-rule summary per body and kind, pivot margin minimum, rcond at p, retained-residual summary, retained-state digest, and the ordinary-attempt reference);
  - **`unavailable`:** the attempt failed. It carries a reason, the attempts list and a reference to its `RETAINED_PRECISION_UNAVAILABLE` diagnostic;
  - **`not_required`:** the ordinary attempt passed.
- **Mixed envelopes are normal** (D1 §5.3). A successor identity is emitted only when at least one case is `selected`.
- **`numerical_quality.cases[i]` keeps the ordinary attempt's M03-INTEGRITY-v1 outcome** for every case, selected ones included. The precision-p outcome lives only in the receipt. Readers never derive a selected case's standing from `numerical_quality`. D1 adopted this as IF-1 (D1 revision 2 §4.5 and §5 item 3, confirmed by V1's backcheck), so DD-11 is closed. G5 still refuses a selected case whose `numerical_quality` claims `checks_passed` without a matching ordinary attempt.
- Every row of a selected case carries `recovery_method = contribution_preserving_multiprecision_v1` in its evidence (D1 §5.2).

#### 4.9.3 Reader checks (identical order and codes in Rust, Python and TS)

| Step | Check |
|---|---|
| G0 | Identity, profile and table sha256 |
| G1 | Receipt shape against the closed schema; `receipt_sha256` over the body, and `publication_sha256` over the envelope minus the receipt, **with the profile named in the body**. Readers support exactly the profiles ROOT registers for the policy: the checked profile needs nothing new; the scientific profile needs a TS canonicalizer that does not exist today (`hashService.ts` has only the checked one). An unknown profile gives `unsupported` |
| G2 | Encoding. Every receipt number that can exceed 2^53 − 1, be subnormal or be negative zero is a 16-hex bit string decoding to a finite binary64. Plain JSON numbers in the receipt are only exact integers (counts, precisions, work) within the profile's integer range |
| G3 | Case coverage and order against the request (when an invocation is supplied) and `numerical_quality`. Case ids unique |
| G4 | Diagnostics. Exactly one `RETAINED_PRECISION_SELECTED` per selected case and one `RETAINED_PRECISION_UNAVAILABLE` per unavailable case, each with `affected_refs == [case id]`. No `RETAINED_PRECISION_UNAVAILABLE` or `SOURCE_BLOCK_RECOVERY_UNAVAILABLE` names a selected case (the S13 rule from the start). No `SOURCE_BLOCK_RECOVERY_SELECTED` anywhere: a successor envelope never mixes methods (D1 option A) |
| G5 (G5a) | Per selected case: p ∈ {128, 256, 512}; verification p = 2p ≤ 1024; the attempts list ends with the accepted attempt, and every earlier attempt has a rejection reason; work within the registered limits (D-8); every stop-rule entry decodes to a value ≤ 2^-64 (exact binary64 comparison with the constant); pivot margin and rcond decode to finite positive values; the retained-state and load-ledger digests are 64 lowercase hex; the ordinary-attempt reference binds to `numerical_quality.cases[i]` and its diagnostic, as source-blocks `ordinary()` does. `RETAINED_PRECISION_UNAVAILABLE` reasons admitted in G4 include `receipt_encoding`, `publication_hash_range` and `invocation_not_representable` (D1 §5 item 2; S5-R) |
| G5b (revision 3, S8-R) | **S\* recomputation.** For each body and each kind D1 revision 3 defines (translation, rotation, force, moment, and the new twist and extension kinds), the reader recomputes S\* from the published rows of that case (or combination), exactly as D1 revision 3 specifies: the kind mapping of row kinds and units, the pinned unit factors, S(kind) as the largest abs value, the coupling with L_b in D1's stated operation order, and the twist and extension derivation D1 fixes. Inputs the rows cannot supply (body membership, L_b, section terms for twist and extension) are checked against the invocation model with D1's formula, and structurally when there is no invocation. Eligibility always has an invocation, so every eligible result has its S\* fully verified. The recomputed S\* must equal the receipt's S\* bits exactly. Otherwise `RETAINED_PRECISION_SCALE_MISMATCH` (`unsupported`) |
| G5c (revision 3, S8-R) | **Classification recomputation.** R is read from its pinned bits in the receipt, which must equal the registered constant's bits. For every published physical quantity row of a selected case (and every retained-state combination output), the reader computes `t = fl(R·S*)` for that row's body and kind, and its class: `absolute_verified` iff `abs(q) < t`, else `relative_verified`. A row whose kind is outside D1's kind mapping is `not_covered`. The receipt's `absolute_verified` list must equal the recomputed set exactly, each listed bound must equal `fl(2^-64·S*)` bit for bit, and any row the receipt marks `not_covered` must be one the reader also finds outside the mapping. Otherwise `RETAINED_PRECISION_CLASSIFICATION_MISMATCH` (`unsupported`). Rows of unselected cases are ordinary rows and carry no class |
| G6 | Rows. Every row of a selected case carries the method token. No row of an unselected case carries it |
| G7 | The base-identity validator on a projection. It removes only the receipt, the method-token evidence and the identity, profile and policy constants that G0–G6 bound. It then runs the unchanged preview-physics-1, physics-1 or load-reference-1 evidence validator (T1's projection pattern, `load_reference_source.rs:184-220`) |
| G8 | With an invocation: invocation shape and hash (with the named profile); model project id; case coverage; requested mode. Model-derived operands: `<physics-retained>` uses physics-source-1's `actual_materials` through S-C's parameter, over the physics-1 evidence; `<load-reference-retained>` uses S-E1 (J4.2–J4.8; `Scope` → `needs_recompute`); `<preview-retained>` compares the materials and sections its preview-physics-1 evidence publishes, and what it does not publish, the receipt's source identity digest covers (the physics-source-1 trust level, stated as a limit) |
| Transport | Canonical carriers without raw rows: G0–G2, plus the base identity's transport-metadata check on the projection. Never eligible |

#### 4.9.4 The standing basis

- **Chosen: a verified receipt plus invocation binding, without replay.** Python and TS cannot run the kernel (D1 §5.5), and physics-source-1 is Current today on this basis. Rust-only replay as a standing input would break parity or force delegation.
- **Replay** of every committed successor raw and every VP-ROBUST product-lane output, comparing retained-state digests, is a **Rust validation-lane audit** in `numerical_robustness` (D1's W5). It is not a standing input. A replay mismatch is a producer defect that reopens standing (as ROOT's R-3(b) wording provides).

| Standing | Condition |
|---|---|
| `numerically_eligible` | G0–G8 pass with an invocation; requested refs equal the case order; every case is `selected` (G5 passed) or `not_required` with the base identity's ordinary eligibility (`checks_passed`, `passive_model_basis`, `represented_equations_retained`, evidence refs resolve; the existing rules at `semantic_contract.rs:410-452`); `MECHANICS_SOLVED`; T0R's combination gates respected by the base validator |
| `needs_recompute` | No invocation; any `unavailable` case; any `not_required` case not ordinarily eligible; a `Scope` result; differing requested refs |
| `unsupported` | Any G-check fails |

#### 4.9.5 The 0.4.0 successor (S-G2)

G8 uses S-E1. The H-a rule (DD-4) means a D1 0.4.0 successor case that uses the logarithmic law is `needs_recompute`, unless ROOT chooses H-b. The same case is `needs_recompute` under the retiring joined identity, so ROOT's gate condition 3 ("no worse than the retiring identity's, case by case, in all three languages") passes for it (S2-R ruling). D1 records the H-a subset in the gate definition as a declared out-of-scope subset.

#### 4.9.6 Admission sets, schemas and tables

- **Sets.** Each successor id joins the three Current-admission sets in its atomic slice.
- **Tables.** New `P/fixtures/results/semantic_contract_v0_3_<successor>.json` files.
- **Schemas** (T1 files):
  - one new receipt schema file per policy;
  - a new branch in `results.v0.3.schema.yaml`, `analysis_run.v0.3.schema.json` and `stress_neutral_export.v0.3.schema.json`, appended after T1's branches (index 7 and on);
  - schema pins locate branches by identity, as T1's checkpoint 6 did.
- **The AnalysisRun** carries the receipt, like physics-source-1's `source_block_recovery`.
- **Stress-neutral packaging and desktop export** of successor identities are T6. The desktop refuses them with T1's shared refusal (T1 ruling §12) until T6.

#### 4.9.7 Binding

`result_envelope_binding` produces documents for eligible successor results unchanged. `derive_document` and `validate_document` gain the successor ids in their receipt-copy lists (`derivative.rs:97-116, 326-334` pattern).

#### 4.9.8 Parity and controls

- **Positive controls.** A shared case file `retained_precision_cases.json` (raw, table and transport cases) consumed by the three readers. Positive controls use producer outputs from D1's F2 and F3 (N05 and N06 on both routes, mixed envelopes, both modes).
- **Negative controls.** One mutation per G-check, including:
  - a stop-rule entry at 2^-64 plus one ulp;
  - a verification p that is not 2p;
  - a selected case with `numerical_quality` rewritten to `checks_passed` and the receipt removed;
  - an `UNAVAILABLE` on a selected case;
  - a plain JSON number above 2^53 − 1 in the receipt;
  - a wrong profile name;
  - an invocation whose material point differs.
- **Mutants.** One per new branch per language.

#### 4.9.9 Verified-accuracy classes: the consumer rule (revision 3, S8-R)

ROOT ruled that a quantity below the floor is withheld, or marked uncovered, and never counted as Passed. `absolute_verified` quantities are withheld from Current, or explicitly exempt with a stated reason. The rule below applies in Rust, Python and TS, and at every binding and export site.

| Class (from G5c) | What readers show | Rule binding and reliance | Canonical and exported forms |
|---|---|---|---|
| `relative_verified` | The value, as today | Bindable, subject to the envelope's standing | As today |
| `absolute_verified` | The value, marked "uncovered: verified only to an absolute bound of ±b (below the relative floor)", with b = `fl(2^-64·S*)` | **Withheld.** `rule_binding_refusal` returns `RULE_QUANTITY_BELOW_VERIFIED_FLOOR`. The rule runner reports the input as `RULE_INPUTS_INCOMPLETE` with that reason, never as a pass. A summary or headline whose `result_ref` names such a row is refused in the same way | The receipt travels with the document. `derive_document` copies it (the receipt-copy list of §4.9.7), and adds one `row_disclosures` entry per such row naming the class and bound. Every canonical-document consumer applies the same refusal |
| `not_covered` | The value, marked "uncovered: no verified accuracy for this quantity kind" | **Withheld.** `RULE_QUANTITY_NOT_COVERED`, as above | As above, with class `not_covered` |

**Envelope standing is not demoted by these rows.** Demoting it would withhold correct relative-verified quantities, for example every signed action in a torsion model whose axial force is a structural zero computed as noise (D1 §4.1.6 D-12 rationale). This mirrors T0R's R-2 and R-7 (i) pattern: an envelope stays Current, and a quantity with no reliable reading is refused wherever it could be relied on. So "withheld from Current" is enforced at quantity level:
- the quantity never binds;
- it never counts as Passed;
- it never appears unlabelled.

**Exemption.** None is proposed. An exact structural zero cannot be told apart from noise below the floor (D1 §4.1.6), so no class of below-floor quantity can be exempted on evidence. A future exemption needs a ROOT ruling that names the quantity class and its reason.

**Interval binding (option).** Binding an `absolute_verified` quantity as the interval q ± b, and passing a rule only if it passes at both ends, would restore use of these quantities. It needs interval semantics in `rule_check_runner` (a new input shape) and in its desktop mirror. It is not proposed in T3 (DD-13).

**The VP harness.** A below-floor or not-covered comparison never counts as a pass (ROOT's F2 ruling). That is D1's and R1's harness rule. On the reader side it means S-G's shared case files never treat a floor-classified row as verified-relative evidence.

**Where the rule lives:**
- `semantic_contract::rule_binding_refusal`, `compatibility.rule_binding_refusal` and `knownSemanticLimitations.ruleBindingRefusal` gain the two codes for successor identities, reading the validated receipt. The native binding sites and `rule_check_runner` already route through the helper (§3.5).
- `knownSemanticNotices` gains a text for each class.
- `derive_document` gains the disclosure.

All of these are S-G1 or S-G2 files.

**Notice texts:**
- **`N_RP_ABSOLUTE`:** "Uncovered quantity: verified only to an absolute bound of ±{b} {unit}, below the relative accuracy floor for this body. It is shown for inspection; rule checks cannot bind to it."
- **`N_RP_NOT_COVERED`:** "Uncovered quantity: no verified accuracy for this quantity kind. It is shown for inspection; rule checks cannot bind to it."

### 4.10 Interaction with S11 (context only)

V1-S11, an absorbed and cancelled nodal load published as Passed, is D1's containment slice (ROOT ruling 2; `MANAGER_NOTES/S11_MAP.md`). Its reader-side effect is none, provided the containment marks the case not Passed (Sensitive or refused): whole-envelope standing then withholds Current in every language, unchanged. If the containment adds a diagnostic code to an identity whose readers enumerate diagnostic codes, that code must be added to those readers in the same slice. D2 flags this as interface IF-2.

## 5. Interface assumptions and requirements on D1

- **I-1 (row semantics).** Successor rows have their base identity's semantics (D1 §4.4). S-G's G7 depends on this.
- **I-2 (standing basis).** A verified receipt plus invocation binding (§4.9.4). IF-1, that `numerical_quality` keeps the ordinary attempt's outcome, is adopted by D1 (DD-11 closed).
- **I-8 (floor classification, revision 3).** D1 revision 3 defines:
  - S\* on published rows for every kind, including twist and extension, with the kind mapping, unit factors, L_b formula and operation order pinned;
  - R's bits, in the receipt;
  - the classification `abs(q) < fl(R·S*)` on the published value;
  - which rows, if any, are `not_covered`.

  G5b and G5c mirror those definitions exactly. If D1 revision 3 differs in a detail, S-G follows D1 and keeps the structure (recompute, compare bits, refuse on mismatch).
- **I-3 (retirement).** Replaced by the shared retirement gate (§4.5.2), defined by D1. S-G is its standing condition.
- **I-4 (existing source identities).** Answered by ROOT: physics-source-1 is no longer emitted after F2 but stays eligible (R-3(b)); joined stays fresh until F3 (R-3(a)).
- **I-5 (range).** Unchanged: §4.6 applies to whatever D1 publishes.
- **I-6 (receipt encoding).** D1 revision 2 names the profile and the bit-string encoding. The reader requirements are G1 and G2. D1 states the per-case outcome of a hashing failure (V1-S4). Recommended to D1: the checked profile with bit strings, because readers then need no new canonicalizer in TS.
- **I-7 (capture): closed.** D1 states that W1 attempts require S-H's `digest_ok()`; otherwise `RETAINED_PRECISION_UNAVAILABLE` (`invocation_not_representable`) applies (V1 S5-R).
- **IF-2 (S11).** §4.10.

**Validity if D1 chooses differently.** Items 1, 3, 4, 6 and 7 hold. Item 8 (S-G) is written against D1 §5; a different receipt changes G5's fields, not the structure. Item 5 depends on the gate. Item 2 is scheduled by ROOT's R-3(a).

## 6. Verification plan

### 6.1 Probe reproductions

Sources are in `_run_records/d2_probe/`; the plan and predictions are in `_run_records/PROBE_PLAN.txt` (revision 2). They build from exported copies only.

| Probe | Input | Predicted main and T1 | After implementation |
|---|---|---|---|
| PR-1 | P12 | `Err` SOURCE_BLOCKS_FINALIZATION_FAILED | S-D: `Ok` physics-1, not Current |
| PR-2 | P12 with N06 selected | `Err` | S-D: blocked envelope naming the cause (R-1b) |
| PR-3 / 3b | 21 / 2 N05 copies | `Err` / `Ok` source-blocks-1 | S-D: `Ok` ordinary / unchanged |
| PR-4 | P3 (0.4.0) | main: fixture absent; T1: `Ok` load-reference-1 | unchanged |
| **PR-5a/5b** (revised) | N05 with a 1e16 N·m request torque | **`Err` CHECKED-JSON-UNSAFE-INTEGRAL-FLOAT (capture)** | S-H: `Ok` ordinary, capture `UNAVAILABLE` |
| PR-5c | Torque 1e7 | `Ok` source identity | unchanged |
| **PR-5d/5e** (new) | Torque 1e12 (request safe; published rotation about 1e16 rad) | `Err` SOURCE_BLOCKS_FINALIZATION_FAILED (receipt hash) | S-D: `Ok` ordinary, not Current |
| **PR-5f** (new) | PR-5a plus a combination (ordinary route) | `Err` CHECKED-JSON-UNSAFE-INTEGRAL-FLOAT | S-H: `Ok` preview-physics-1 |
| PR-6 | 8 display conversions | underflow reported `converted` | S-B: `unavailable` |
| PR-7 | Guards, both profiles, including 5e-324 and −0.0; 200,000-value differential | checked refuses \|x\| > 2^53 − 1 and admits −0; binary64 admits large floats and refuses −0; differ=0 | T6 pre-check |

### 6.2 Per-slice controls

| Slice | Positive | Negative, tamper and mutation |
|---|---|---|
| S-A | Every committed physics-source-1 and source-blocks-1 raw and carrier keeps its outcome | Four shared cases per family; the check mutant killed ×3 |
| S-B | Identity, zero and normal unchanged | Underflow clauses `unavailable`; clause mutants killed |
| S-C | physics-source-1 outcomes identical | `External` still running `actual_materials` is killed by S-G1 or S-E1 fixtures |
| S-D | PR-1, PR-3 and PR-5d/5e publish ordinarily, with rows bit-equal to the typed route and accepted ×3; committed witnesses still select | The gate-restoring mutant fails the replaced test; PR-2 is blocked with its cause; the guard is never reached; ledger bounded, with `attempts` counting both runs |
| S-H | A 1e16 N/m spring (integer literal) solves on each route, both modes; every committed request byte-identical | The mutant restoring the early `?` fails; a sensitive case in such a request carries the capture `UNAVAILABLE`, never a selection |
| S-E1 | Every committed joined and load-reference-1 witness re-derives to its published records, in 3 languages | One mutation per J4 row gives Mismatch; `Scope` cases (H-a law, i128 overflow) give identical outcomes ×3 |
| S-E2 (if built) | Five joined witnesses eligible with their invocation; route documents minted | Revision 1's J4 controls; no invocation → `needs_recompute` |
| S-G | §4.9.8 positives. Revision 3 adds: a weak-coupling case (D1's S8-W) whose below-floor rows are listed `absolute_verified`, shown uncovered and refused for binding identically in the three languages; a relative row in the same envelope still binds | §4.9.8 negatives. Revision 3 adds: an S\* entry off by one ulp; R's bits changed; a qualifying row missing from the `absolute_verified` list; an extra listed row; a bound off by one ulp; a row marked `not_covered` that is inside the mapping (each `unsupported`); a headline pointing at an `absolute_verified` row (binding refused). Rust replay audit in the validation lane |
| S-F | Historical raws keep their standing (all-selected Current, mixed `needs_recompute`); texts updated | A tampered historical raw is `unsupported` |

### 6.3 Pre-0.4 differential (S-D, S-H, and D1's F2)

Every committed pre-0.4 request, plus the probe inputs, runs through the base and the candidate in both modes. Outputs must be byte-identical except on the inputs the slice is meant to change, each listed with its reason.

### 6.4 Existing tests that change

None is deleted, and none is weakened.

| Test | Slice | Change and justification |
|---|---|---|
| `load_state_fallback_tests.rs::the_pre04_composite_finalization_failure_is_unchanged_by_this_fallback` | S-D | Renamed, and turned into a behavioural test (§4.1.4). It pinned the defect, labelled "not an endorsement" |
| Tests asserting a capture `Err` for out-of-profile requests (the S-H implementer searches for `CHECKED-JSON-` expectations on the value route) | S-H | Become `Ok` ordinary-route assertions. Each is listed in the S-H record |
| `load_reference_route_tests.rs::joined_actual_solve_retains_…_without_canonical_export_both_modes` | S-E2 only | Replaced by the positive route test. If S-E2 is not built, only its comment changes |
| Joined standing assertions ×3, and the doc comments in `load_reference_source.rs:29-31` and peers | S-E2 only | Eligible with an invocation; `needs_recompute` without |
| Producer-driven source-blocks-1 and physics-source-1 tests (`product_physics/tests/source_block_recovery.rs`; the headless `actual_source_blocks_bind_…`, `source_block_capture_retains_…`, `actual_value_stress_publication_range_refusal_…`) | D1's F2, with S-F | Identity assertions move to the successor identity. Physical value assertions against the independent references are kept. Capture-custody assertions move to a surviving capture path (S-H tests or the successor) |
| `knownSemanticLimitations.test.ts`, `KnownSemanticNotices.test.tsx` text pins | S-F | New texts |
| T0R's set pins ×3 | S-G | Name the successor ids as well; they stay exact pins |

## 7. Slices, order and T1 serialization

| Slice | Content | Write set (main paths) | T1 overlap | When |
|---|---|---|---|---|
| **S-A** | Selected-UNAVAILABLE tightening | `result_export/src/{source_blocks.rs, physics_source.rs}`; `analysis_runs/{source_blocks.py, physics_source.py}`; `sourceBlockRecovery.ts`, `physicsSourceRecovery.ts`; shared cases; tests | disjoint | Before T1 merges |
| **S-B** | Display representability | `operation_applier/src/display_units.rs`; `services/unitConversion.ts`; tests | disjoint | Before T1 merges; parallel with S-A |
| **S-C** | `physics_source` material-check parameter (outcome-neutral) | `physics_source.rs`, `physics_source.py`, and `physicsSourceRecovery.ts` if needed | disjoint | Before T1 merges, **after S-A** (V1-S10) |
| **S-H** | Capture without a required digest | `source_receipt.rs`; `PP` (attempt gate); tests | **T1** | After T1 merges and main is merged in; first facade slice of D2 |
| **S-D** | Pre-0.4 composite fallback | `PP`; `load_state_fallback_tests.rs`; new `pre04_fallback_tests.rs` | **T1** | After S-H. Worth landing only if D1's F2 is not imminent (its reach is the pre-F2 window) |
| **S-G1** | Readers for `<preview-retained>` and `<physics-retained>` | New `result_export/src/retained_precision.rs`, `analysis_runs/retained_precision.py`, `features/results/retainedPrecisionEvidence.ts`; `semantic_contract.rs`, `compatibility.py`, `numericalResultQuality.ts`, `knownSemanticLimitations.ts`; `derivative.rs`; tables, receipt schema, carrier schema branches; shared cases | **T1** | **One atomic PR with D1's F2**; after S-C; after ROOT's identity reservation and D-8 budgets |
| **S-E1** | Resolved-case re-derivation core | New `load_reference_rederive.{rs,py}`, `loadReferenceRederive.ts`; shared cases | New files; consumers are T1 files | With D1's F3 |
| **S-G2** | Reader for the 0.4.0 successor | As S-G1, the 0.4.0 branch | **T1** | One atomic PR with D1's F3 and S-E1 |
| **S-E2** | Joined standing wiring, route test, carriers, TS port | `load_reference_source.{rs,py}`, `loadReferenceSourceEvidence.ts`; standing files; `load_reference_route_tests.rs`; new route carriers | **T1** | **Conditional:** only if D1's F3 will not land within T3 (ROOT R-3(a)) |
| **S-F** | source-blocks-1 texts, doc comments and test migrations | `knownSemanticLimitations.ts`; `compatibility.py` docstring; set doc comments ×3; tests | **T1** | With or after D1's F2, once the shared gate is met |
| T6 handoff | binary64 adoption in carriers, headless digest, persistence, export | T6's own | n/a | Routed by ROOT (DD-8) |

- **Order.** S-A and S-B run in parallel, then S-C. After the T1 merge: S-H, then S-D (conditional on F2's timing), then S-G1 with F2, then S-E1 and S-G2 with F3; S-F with F2; S-E2 only if F3 misses T3.
- **Merge gate:**
  - complete-diff independent review;
  - hosted CI, including the dual-viewport dispatch whenever TS changes;
  - a clean DEC-025 sweep;
  - native witnesses on the owner's Mac, recorded as outstanding if not available. S-H: a 1e16 N/m spring solves in the native app. S-G1: a successor result is shown Current with its invocation.
- **Each slice is one atomic PR** across its languages.

## 8. What T3 completes and what remains

| Item | After this design, with D1 | Remains |
|---|---|---|
| Composite `SOURCE_BLOCKS_FINALIZATION_FAILED` | Closed: no `Err` (S-D), and no fresh exact-block selection after F2 | R-1a and R-1b in the pre-F2 window only |
| Capture refusal (V1-S5) | Closed for solving (S-H) | Canonical export of such results: T6 |
| Joined eligibility (T1 §7) | Scheduled by R-3(a): S-E1 with F3; S-E2 conditional | Historical joined standing after F3 (DD-10) |
| Binding route (T1 §11) | Automatic for eligible identities | Desktop export of the load-reference and successor identities: T6 |
| Selected-UNAVAILABLE | Closed (S-A); built into S-G | None |
| T0R carries R-1, R-2 | Fresh results: moot after F2. Historical: containment kept by ruling (R-7 (i)); texts no longer promise a tranche | None, unless a defect is found |
| Successor readers (V1-S3) | S-G1 and S-G2 designed, with the floor enforced (G5b, G5c, §4.9.9) | None in design; implementation with F2 and F3 |
| M34 display range | Closed (S-B) | None |
| M34 transport range | Requirement, references, capture fix and fallback | Carrier adoption and persistence: T6 |
| M34 comparison policy | Stated; unchanged predicates | None |
| Case-scoped Current (F3) | Not proposed | Only if requested |

Containment alone closes no group. Closure needs VP-ORACLES and VP-ROBUST on the merged candidate.

## 9. Decisions for ROOT

| ID | Decision | Options (recommended first) | Status |
|---|---|---|---|
| DD-1 | Composite finalization policy | **F1** / F2 / F3 | Open |
| DD-2 | Residuals R-1a and R-1b | **Accept as open until D1's F2** / close with F2 | Open |
| DD-3 | TS position for joined and successor eligibility | **TS-a check-for-check** / TS-b native-delegated verdict | Open |
| DD-4 | Host-rounded fields | **H-a: `needs_recompute` when a host-`exp` definition is used (decided from the invocation)** / H-b: exact bracketing with rational enclosures (Rust big-integer cost) / (withdrawn) one-ulp tolerance | Open; ROOT ruling 5 prefers H-a |
| DD-5 | Selected-UNAVAILABLE direction | **Tighten** / relax | Open |
| DD-6 | Re-homing | **R-B under the shared gate** / R-0 if the gate misses T3 | Open |
| DD-7 | Historical all-selected source-blocks-1 | **(i) stays Current with the notice and summary refusal** / (ii) historical-only | **Ruled (i)** (R-7) |
| DD-8 | Transport split | **T3: capture (S-H), requirement, references, fallback, display. T6: binary64 carriers, headless digest, persistence, export, with the integer-literal, negative-zero and size rules** / T3 adopts the carriers now | Open (ruling 4 assigns capture to T3) |
| DD-9 | Eligible load-reference, joined and successor results are Current and rule-eligible; export and report package refused until T6 | **Confirm** / keep them off rule checks until T6 | Open |
| DD-10 | Historical joined standing after F3, if S-E2 was built | **Keep eligibility (as R-3(b))** / `needs_recompute` | Open, only if S-E2 is built |
| DD-11 | Selected-case `numerical_quality` (IF-1) | **Ordinary attempt's outcome; precision-p outcome only in the receipt** / D1 writes `checks_passed` and S-G special-cases it | **Closed:** adopted by D1 (V1 backcheck) |
| DD-12 | Capture fix | **H-1 fallible digest** / H-2 scientific profile at capture / H-3 normalized hashing | Open |
| DD-13 | Consumer rule for `absolute_verified` and `not_covered` quantities (revision 3) | **Quantity-level withholding: shown uncovered with the bound, rule binding refused, never Passed, class carried in canonical forms; envelope standing unchanged; no exemption** / whole-envelope `needs_recompute` whenever a selected case has such a row / interval binding q ± b in the rule runner | Open. ROOT's S8-R ruling requires withholding or a stated exemption; the recommendation meets it |

**Owner-level question.** None. F3 would raise one (case-scoped Current), as in revision 1.

## 10. Places where T1's final merge could change this design

- **[T1-1]** SF-1's shape (wrapper, decline, ledger). S-D and S-H follow the merged code.
- **[T1-2]** The joined reader's steps, T1's standing early return, and the TS joined port.
- **[T1-3]** The joined record and wire records. S-E1 is specified from them; any later addendum is re-read in RD-0.
- **[T1-4]** `load_reference_route_tests.rs` names and assertions.
- **[T1-5]** `knownSemanticLimitations.ts` and the set constants.
- **[T1-6]** Any change to T1's S13 rule before merge.
- **[T1-7]** T1's desktop export refusal and its T6 routing (DD-9).
- **[T1-8]** The characterization test's name and location.
- **[T1-9]** The committed joined witnesses and carriers.
- **[T1-10]** T1's capture and replay use of `CapturedInvocation` (S-H).
- **[T1-11]** The carrier-schema branch order that S-G appends to.

## 11. Records consulted

- **Instructions.** Root `AGENTS.md`, `agents/AGENT_TASK.md`, and `agents/AGENT_HELPS_HUMANS.md`. The last was read deliberately for the design posture, recorded as the wider consultation the Root doctrine requires.
- **Revision 1 basis.** Unchanged; see `_run_records/DESIGN_revision1.md` §11 for the list and sha256 prefixes.
- **Revision 2 inputs** at `065c9ff60`:
  - `T3/REVIEW/RETURN.md` (V1, BLOCKING in D1 only);
  - `T3/ROOT_RULINGS_V1.md`, including the further rulings on R-3(a), R-3(b) and R-7 as relayed by the manager (the committed record may differ in wording);
  - `T3/MANAGER_NOTES/V1_DISPOSITIONS.md`;
  - `T3/MANAGER_NOTES/S11_MAP.md` (context);
  - `T3/DESIGN_NUMERICS/DESIGN.md` revision 1 (`7199390f39f2c46d`), §1, §4.1.6, §4.2–4.5, §5, §6, §9.
- **Product source read in revision 2,** at `c61a540ea`:
  - `source_receipt.rs:20-130` (capture, `checked`, `hash`, `bits`);
  - `canonical_json/src/binary64.rs:1-215`;
  - `canonical_json/adapter.py` (helper outline);
  - `derivative.rs:13-41`;
  - `src-tauri/src/lib.rs:1557-1563`;
  - `headless/src/lib.rs:731-745`.

  At `f3270ea79`: `git grep` for the diagnostic code in committed JSON.

## 12. Sources, probes and limits

**Run** (read-only, standard library or Git, no build):
- `_run_records/scan_unavailable.py.txt` over every committed JSON under `P/` at both bases (`scan_unavailable.log`): receipt-bearing envelope objects carrying the diagnostic, 0 of 43 (main) and 0 of 83 (T1).
- Revision 2: `git grep -l SOURCE_BLOCK_RECOVERY_UNAVAILABLE` over committed JSON (`_run_records/grep_unavailable_json.log`):
  - main: none outside the execution records;
  - T1: two SF-1 fallback raws, two mutation case files, and one T1 record.

**Prepared, not run (host hold):** the revised probe crate `_run_records/d2_probe/`, and `PROBE_PLAN.txt` revision 2 (PR-5 split; PR-7 extended). Every prediction in §6.1 is unverified until those probes run.

**Not done:** no build, test, probe or suite; no Git write; no product, test, fixture or schema edit; no read of R1's references.

**Limits:**
- The capture refusal is established by reading (as V1's was).
- The integer-literal form of desktop requests follows from JS number formatting and serde's parsing, not from a run.
- The successor reader contract was written against D1 revision 1's §5 and checked against D1 revision 2's §5 in V1's backcheck (G1, G2 and G5a consistent). G5b and G5c are written against D1 revision 3's announced definitions (I-8), which were not yet available.
- H-b's termination at 1024 bits is not proved for every binary64 argument. H-a has no such dependency.
- The ROOT rulings after `065c9ff60` were read from the manager's relay.

### 12.1 Probe results (addendum after the host release)

**How the probes were run.**
- The unchanged `_run_records/d2_probe` sources and `PROBE_PLAN.txt` revision 2.
- Built from `git archive` exports of `c61a540ea` and `f3270ea79` in `<scratch>`, never from T1's worktree.
- `CARGO_TARGET_DIR=<t3-target>`, `RUSTUP_TOOLCHAIN=1.97.1` (rustc 1.97.1), `CARGO_INCREMENTAL=0`, `cargo build --offline --release -j 2`. Not `--locked`: the probe crate is not in the copied lockfile.
- Each build waited until no other cargo process was running.
- Records are in `_run_records/probes/`: per-probe logs `PR-1.log` to `PR-7.log`, full logs, build logs, `ENVIRONMENT.txt` (toolchain and binary hashes), the runner script, and `SHA256SUMS`.
- The build output and exports were deleted afterwards.

**Main and T1 gave identical results on every probe except PR-4,** where main lacks the 0.4.0 fixture. Every probe gave the same outcome in both solver modes.

| Probe | Predicted | Observed (both bases, both modes) | Verdict |
|---|---|---|---|
| PR-1, P12 | `Err` SOURCE_BLOCKS_FINALIZATION_FAILED | `Err` SOURCE_BLOCKS_FINALIZATION_FAILED | Confirmed |
| PR-2, P12 with N06 selected | `Err` SOURCE_BLOCKS_FINALIZATION_FAILED | `Ok`, but a **blocked** physics-1 envelope: `MODEL_INCOMPLETE`, 0 rows, blocking `NUMERICAL_INTEGRITY_ASSEMBLY_UNRESOLVED` on `case:ordinary-pressure`. `case` (N06) carries `SOURCE_BLOCK_RECOVERY_SELECTED`; `case:ordinary-pressure` carries `SOURCE_BLOCK_RECOVERY_UNAVAILABLE` | **Prediction failed** (finding F-P2 below) |
| PR-3, 21 N05 cases | `Err` SOURCE_BLOCKS_FINALIZATION_FAILED | `Ok`, but a **blocked** preview-physics-1 envelope: `MODEL_INCOMPLETE`, 0 rows, blocking `SOURCE_BLOCK_RECOVERY_FINALIZATION_FAILED` at `case:18`. That is the per-case finalization arm (`PP:2914-2926`), not the invocation receipt | **Prediction failed in form** (finding F-P3) |
| PR-3b, 2 cases | `Ok` source-blocks-1 | `Ok` source-blocks-1, receipt `qualified`, both cases selected | Confirmed |
| PR-4, P3 | main: skipped; T1: `Ok` load-reference-1 | main: skipped. T1: `Ok` load-reference-1, `MECHANICS_SOLVED`, `sensitive`, no receipt; both cases `UNAVAILABLE` and `NOT_JOINED` | Confirmed |
| PR-5a/5b, request torque 1e16 | `Err` capture refusal | `Err CHECKED-JSON-UNSAFE-INTEGRAL-FLOAT: 10000000000000000` | **Confirmed (V1-S5).** The probe used a float literal; the integer-literal code (`CHECKED-JSON-UNSAFE-INTEGER`) was not exercised |
| PR-5c, torque 1e7 | `Ok` source identity | `Ok` source-blocks-1, qualified | Confirmed |
| PR-5d/5e, torque 1e12 | `Err` SOURCE_BLOCKS_FINALIZATION_FAILED (receipt hash) | `Err` SOURCE_BLOCKS_FINALIZATION_FAILED for source-blocks-1 and physics-source-1 | Confirmed. Attribution to the receipt hash is by elimination: capture passed, and the `Err` value carries no detail |
| PR-5f, ordinary route with a combination, request 1e16 | `Err` capture refusal | `Err CHECKED-JSON-UNSAFE-INTEGRAL-FLOAT: 10000000000000000` | Confirmed: the refusal applies to the ordinary route too |
| PR-6, display | Scaled underflow reported `converted`; overflow `unavailable` | 5e-324 Pa→MPa `converted` 0.0; 1e-303 Pa→MPa `converted` 1e-309 (subnormal); 2.2250738585072014e-308 N→kN `converted` 2.225e-311 (subnormal); 1e-320 Pa→MPa `converted` 0.0; identity keeps 5e-324; −0 Pa→MPa gives +0 (sign dropped); 1.7e308 MPa→Pa `unavailable` | Confirmed |
| PR-7, canonical | Checked refuses \|x\| > 2^53 − 1; binary64 admits; −0 differs; differ = 0 | `guard_json` and the checked profile refuse every value from 2^53 up (9007199254740992, 1e16, 1e20, 1e21, 1e300). binary64 admits them all. −0: checked admits it and renders `0`; binary64 refuses it (`BINARY64-JSON-NEGATIVE-ZERO`). Differential: 105,049 checked-admitted values, 0 differing | Confirmed, with one observation (F-P7) |

**Findings from the probes.** They are recorded, not designed. Neither this addendum nor revision 3 changes the design in response to them.

- **F-P2: a rejected, unrecoverable case blocks the whole invocation, before finalization.**
  - In PR-2, the second case's ordinary attempt is rejected (assembly unresolved), and retained source is unavailable to it (pressure region). The case loop therefore returns a blocked envelope as soon as that case solves (`PP:1497-1499`), and the N06 case's correct selected result is lost with it. S-D's fallback never runs.
  - This is the product's existing M03 rule on every route: a case with no publishable response blocks the invocation. It is not the finalization defect. Under the brief's standard it is still "an invocation loses a correct result because another case failed".
  - Only case-level publication (options F2 or F3, which needs a blocked-case form) or D1's method would change it.
  - Residual R-1b (N06 selected, with the *other* case sensitive rather than rejected) was not exercised by this input. A corrected probe PR-2b (N06 plus the sensitive P12 pressure case) is proposed for the implementation slice.
  - For ROOT, the question is whether this blocking rule stays as M03 policy or is taken up with F2/F3. I recommend recording it as open with D1's F2, and not changing it in T3's standing work.
- **F-P3: the invocation-ledger trigger surfaces at per-case finalization.** With 21 cases, case 18's finalization, not the invocation receipt, fails first, so the result is a blocked envelope rather than `Err`. S-D's mechanism covers this arm (§4.1.3 item 3), so the design is unaffected. The S-D companion test should assert the per-case arm with this input.
- **F-P7: binary64 canonical text is not readable by checked-profile consumers.** The scientific profile renders 1e16 and 1e20 as integral literals (`10000000000000000`, `100000000000000000000`), which the checked profile refuses on input. For T6: a carrier hashed or written under binary64 cannot be re-read by any checked-profile consumer. Adoption must move every consumer of that carrier at once. The same applies to the negative-zero rule already listed in §4.6.2.
- **Display sign of zero.** −0 displays as `0`, as §4.6.1 specifies. The sign is dropped in conversion, which is harmless for display.

`_run_records/SHA256SUMS` lists this folder's files; `_run_records/probes/SHA256SUMS` lists the probe records.
