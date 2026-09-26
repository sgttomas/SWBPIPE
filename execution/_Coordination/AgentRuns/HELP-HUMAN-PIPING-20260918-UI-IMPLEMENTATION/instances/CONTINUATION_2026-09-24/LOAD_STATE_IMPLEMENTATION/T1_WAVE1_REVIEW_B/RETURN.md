# RETURN — T1_WAVE1_REVIEW_B (independent review: operations, harness adapter and VP-STATIC admission)

- **Role:** TASK (Type 2), fresh-context, non-author reviewer. I wrote none of the reviewed bytes. I did not delegate.
- **Requested by:** the T1 WORKING_ITEMS manager; ROOT spawned me.
- **Checkout reviewed:** committed bytes of branch `codex/piping-load-states-20260925` at `6ded3e347` (the brief's candidate `31dc7ce08` plus the review-brief docs commit; no reviewed file differs). All runs used scratch copies. The shared worktree held uncommitted edits to `tools/validation/qualification_load_reference{,_helper}.py` during the review, so I ran nothing that counts toward this verdict on it. The Python runs used a shared, sparse clone checked out at `6ded3e347`.
- **Git:** no writes. I wrote only in this folder.
- **Not run:** the product comparisons, as the brief says.
- **Paths:** relative to WORKING_ROOT (`projects/chirality-piping/`). `LSI` is this folder's parent.

## Verdict: FINDINGS

Nothing I found is BLOCKING. The WP3 operations, the WP5 adapter and the WP6 admission mechanics do what their returns claim.

- The golden control file reproduces exactly from the base revision.
- The admission changes only the four fields named in the brief.
- R1–R3 match the freeze hashes.
- The re-pin is correct.

Three findings are Medium:

- **F1 and F2** show that operations elsewhere in the applier can still change `schema_version` on a 0.4.0 model, or orphan an `analysis_state` reference.
- **F3** is the output-limit defect the manager already reported. It fails closed, but it blocks the real run.

None of the three is silent: the product blocks each one at solve time, or the adapter records the case as `error`.

## Findings

| # | Severity | Location | Concrete failure scenario | Suggested repair |
|---|---|---|---|---|
| F1 | Medium | `core/model_operations/operation_applier/src/pressure_authoring.rs` `resolve`, the `Model/pressure_profile` branch (existing code, not WP3 bytes) | **Answers brief item 1b: yes, an operation can change `schema_version`.** On the `connected` 0.4.0 witness, `set_field Model/pressure_profile` with after-value `{"schema_version":"0.3.0","pressure_contract":{exact_straight_pressure_v2 2.0.0}}` reviews and applies with **no diagnostic**. The model becomes 0.3.0 but still carries `reference_configurations`, `expansion_laws` and `analysis_state`. What follows (probe P1):<br>• the product blocks the solve with `LOAD_STATE_CONTRACT_VERSION_MISMATCH`;<br>• the three typed load-state operations now refuse with `OP-LOAD-STATE-SCHEMA-VERSION-INVALID`, so the records can no longer be edited or removed through them;<br>• the operation-level inverse (back to 0.4.0) is refused with `OP-PRESSURE-PAYLOAD-INVALID`.<br>The same happens inside an atomic batch. This contradicts D3 and the WP3 rule "no operation sets or changes `schema_version`". | Refuse `Model/pressure_profile` when the current model is `schema_version` 0.4.0. Use a targeted code, for example by reusing `OP-LOAD-STATE-SCHEMA-VERSION-INVALID` or adding a pressure equivalent. Add a test on the `connected` witness. Also show that the 0.2.0 and 0.3.0 pressure-profile outcomes are unchanged (contract corpus and `exact_authoring` tests). |
| F2 | Medium | `src/rich_authoring.rs`, the `Material/temperature_points` path (existing code). This is a gap in the ruling §5 Q1 coverage | Ruling Q1 says: "An operation may not orphan an `analysis_state` or member reference." Replacing a material's `temperature_points` so that `point:cold` is removed, or renamed, **is admitted**; the only output is `OP-RICH-NOT-SOLVE-READY` warnings. Both cases' `exact_point` selections still name `point:cold`. The product then blocks with `LOAD_STATE_MATERIAL_POINT_UNRESOLVED` (probe P2). The Q1 list names configurations, laws, pipe/support/material/primitive deletes, but not this owner of `point_ref`. | In the `temperature_points` path, refuse with `OP-LOAD-STATE-INBOUND-REFERENCE` when a point ID named by an `exact_point` `point_ref` disappears. Count only elements whose `material_selection.material_ref` is the target material, and use WP3's "resolves before, not after" rule (`refuse_orphans`). Add a test (remove and rename) and a control showing that models without the records are unchanged. |
| F3 | Medium (known; reported by the manager during this review) | `tools/validation/qualification_load_reference.py`: `run_selection` admits `--output-limit-bytes` up to 64 MiB, but reads stdout with `read_captured_bytes(..., min(output_limit_bytes, LIMIT))`, where `LIMIT = gate.MAX_INPUT_BYTES` = 8 MiB. `consistency_observation` also caps the reader input at `LIMIT` | A case whose output is more than 8 MiB (the manager saw 14 MB and 22 MB) becomes `error` ("captured size outside admission limit"). The case keeps its denominator, so this fails closed, but those cases cannot be scored. The reader helper would refuse the same size too. | The repair is in progress with the WP5 author. On backcheck, confirm that:<br>• the admission, helper-input and helper-stdin limits agree with the process limit;<br>• no other limit silently truncates output;<br>• a test covers output above 8 MiB. |
| F4 | Low | The admitted package records: `*.criteria.candidate.json` rule `review`, `README.md`, `PROVENANCE.json`, and `LSI/T1_WP6_STATIC_CASES/_run_records/author_package.py` | After admission the package contradicts itself. No score changes: the adapter checks only that `review` is non-empty, and `MANIFEST.json` is correct.<br>• Every admitted rule still says `"review": "pending the independent WP6 freeze check; candidate only, not admitted"`.<br>• The README still says "Status: authoring only … Nothing here is admitted".<br>• In `PROVENANCE.json`:<br>&nbsp;&nbsp;– `status` is still `authoring_only_pending_independent_freeze`;<br>&nbsp;&nbsp;– `generator.sha256` still records `272864e8…`, but the generator has been `8764ed1b…` since `6824b6b6b`;<br>&nbsp;&nbsp;– all 28 `generator_outputs` hashes are the pre-admission ones.<br>• `author_package.py` line 1020 still writes the pre-R1 rule (`hot_thermal_strain`). The freeze (§3, R1) asked the manager either to edit it or to record it as superseded; neither is recorded. | Choose one:<br>(a) add a stale-record note to `ADMISSION.json` and the README: PROVENANCE and the authoring records describe the pre-admission state, and `author_package.py` is superseded for R1;<br>(b) refresh them: README status, PROVENANCE `status`/`generator`/`generator_outputs`, and optionally an admitted `review` string produced by the generator when `ADMITTED`.<br>Option (b) changes rule text, which the brief's four-field admission excludes, so it is the manager's decision. |
| F5 | Low | Test coverage of the adapter and applier suites | The predicates are present and read correctly, but no test protects them. Independent mutants (`_run_records/adapter_mutants.*`, `applier_mutants.*`):<br>• **Adapter, 8 of 12 survived:**<br>&nbsp;&nbsp;– X02: standing evidence codes must all be `NUMERICAL_INTEGRITY_CHECKS_PASSED`;<br>&nbsp;&nbsp;– X03: support-component `unit`;<br>&nbsp;&nbsp;– X04: applied-load dimension/unit;<br>&nbsp;&nbsp;– X05: criterion `result_family`;<br>&nbsp;&nbsp;– X07: `connector == []`;<br>&nbsp;&nbsp;– X08: wrapper `warning_count`;<br>&nbsp;&nbsp;– X10: exact JSON type of a `definition` value;<br>&nbsp;&nbsp;– X12: record `contract`/`profile`.<br>• **Applier, 3 of 6 survived:**<br>&nbsp;&nbsp;– R01: an unchanged payload still writes, so the no-op claim is untested;<br>&nbsp;&nbsp;– R02: `point_ref` resolved on any material, not only the selected one;<br>&nbsp;&nbsp;– R04: the support-state scan stops after the first case.<br>For example, deleting a support named only in the second case's `support_states` would go unguarded if R04's regression happened. | Add one targeted test per survivor. Every survivor needs only a small invented control. |
| F6 | Low | `tests/load_state_authoring.rs`: `displacement_mm`, `connected_middle_ux_mm` | `displacement_mm` solves in both modes but returns `values[0]` (sparse) only. The dense value is checked only for `MECHANICS_SOLVED`, so the RETURN's "both values are checked in both solver modes" overstates the test. (Dense and sparse agree to 1.1e-16 here; probe P4.) Also, `connected_middle_ux_mm` computes `l1` and `l2` separately but applies the equal-length formula `(u_r+u_f+ε₁L₁−ε₂L₂)/2`. That formula is valid only because the witness has L₁ = L₂ = 1 m. | Compare `values[1]` against the prediction as well. Either assert `l1 == l2`, or use the general form `u_m = (u_r/L₁ + u_f/L₂ + ε₁ − ε₂)/(1/L₁ + 1/L₂)`. |
| F7 | Info | `src/load_state_authoring.rs` `resolve` | The operation-level inverse cannot restore an explicit-null prior (`analysis_state: null`), because `null` is refused as an after-value (probe P3). The product blocks explicit null anyway, and desktop Undo is checkpoint-based, so the round-trip claim holds for every admissible prior. | Document it next to the ruling on `not_present` (Q2). |
| F8 | Info | `tools/validation/qualification_load_reference.py` line 57; the test `test_pinned_identities_match_start_bytes` | The comment still reads "Reader bytes at the WP5 start commit d8f0dc4f7". The pin is now the `bfef71b19` bytes. The test name says "start bytes", but its message is honest: it names `bfef71b19`. | Update the comment; optionally rename the test. |
| F9 | Info | `src/rich_authoring.rs` `validate_temperature_points` (existing code) | `exact_profile` is keyed on `schema_version == "0.3.0"`. A 0.4.0 exact-pressure model therefore expects `shear_modulus`, and its `poisson_ratio` points get spurious `OP-RICH-NOT-SOLVE-READY` warnings (seen in P2, even for points left unchanged). | Also treat 0.4.0 with the exact pressure contract as the exact profile. Belongs to the same follow-up as F2. |

## What I verified, and how

### 1. Operations (WP3, `5ced47dec`)

**Crate suite.** `cargo +1.97.1 test --locked --offline -j 2`: **187 passed**, 0 failed (140+2+2+6+8+13+10+1+5). Log: `_run_records/applier_cargo_test.log`. The file hashes equal WP3 Addendum 1: `lib.rs` `4f088d67…`, `load_state_authoring.rs` `63223cc2…`, golden `1da7ef35…`.

**Typed boundary.** The payloads are parsed with the product's own public DTOs (`case_state/input.rs`: `deny_unknown_fields`, tagged unions). So the product and the operation refuse the same unknown fields and discriminants. Explicit null is refused before parsing. Duplicate keys are refused, which is stricter than the product's `Value` path and was accepted by ruling Q3.
- Inner references are read from the same text after the typed parse: `material_ref`, `point_ref` (only on `exact_point`), `expansion_law_ref` (only on `free_length_state`). That covers every reference field in the DTOs except the `PredecessorValue` `case_ref`/`support_ref`, which ruling Q4 leaves to solve time.
- Duplicate configuration and law IDs are refused. Inbound orphans are refused for configurations, laws, and pipe, support and primitive deletes; material deletes are covered by the existing generic scan.

**`schema_version`.** The three load-state operations write exactly one key (`[path]`), never `schema_version`, and require 0.4.0.
- F1 is a pre-existing operation that does rewrite it.
- I searched the other writers: `pressure_authoring` is the only operation that writes `schema_version`. The `resolve_field` rules have no `schema_version` or `id` path.

**Undo/Redo.** The round trips cover value priors and absent priors, compared in both the canonical and the plain serialization. My reading of `refuse_orphans` agrees that it runs for `not_present` removals (mutant R03 killed). See F7 for explicit-null priors.

**Delete control provenance.**
- I overlaid the base revision's `lib.rs` (`d8f0dc4f7`, no `load_state_authoring`) on a scratch copy.
- I removed the WP3 module and its test, and ran `load_state_delete_control` with `LOAD_REFERENCE_DELETE_CONTROL_BLESS=1`.
- The regenerated golden is **byte-identical** to the committed one (`1da7ef35…`): 86 outcomes; 27 applied; `OP-ENTITY-DELETE-REFERENCED`, `OP-PIPE-DELETE-REFERENCED` and payload/stale refusals included.
- `product_physics`, `units` and `canonical_json` did not change between base and candidate, so this is the base behaviour.
- Records: `_run_records/golden_regeneration.{sh,log}`.

**Closed forms.**
- The expectations come from witness constants: node x, entered motions, fits, α, temperatures. They never come from producer output.
- I re-derived the hot case by hand: ε₁L₁ − ε₂L₂ = 0.001·λ_th m, so u_m = (0.5 + λ_th)/2 mm.
- The eigen_motion check uses u_y = u_y,root + θ·L = 3.1 mm.
- See F6 for the dense-mode and equal-length caveats.

**Probes.** `_run_records/review_b_probes.{rs,log}`: P1 (F1), P2 (F2), P3 (F7), P4 (F6).

**Mutants.** 6 independent mutants; R03, R05 and R06 killed; R01, R02 and R04 survived (F5).

### 2. Adapter (WP5, `31dc7ce08`)

**Suites.** Committed bytes: `test_qualification_load_reference` **43 OK**, `test_qualification_gate` **31 OK**, `test_qualification_physics*` **42 OK**. Log: `_run_records/python_suites.log`.

**HARNESS_CUT rules**, checked in the code:
- **Ledger first.** `predeclare` runs before any process. Placeholder rows keep `required_scalar_rows` when the selector inventory is refused.
- **Exactly once.** `resolve_row` requires both a unique row by ID and a unique signature match on that same row (mutant X09 killed). `_one` resolves records and items, with an exact-type `definition` check.
- **Missing is never zero.** A null field raises; `gate.finite` refuses non-numbers; a resolution error clears coverage (X01 killed).
- **Refusals.** Physics-1, source-1, raw 0.1, and the source-blocks and carrier namespaces are refused.
- **Custody.** `verify_retained_artifacts` and `verify_publication_custody` are used, and bound files are re-hashed after each case.
- **Standing.** Standing is reported beside the outcomes. `sensitive` or `insufficient` fails the `numerical_standing` obligation, so the case cannot pass.

**Denominator.** A required assertion cannot drop out silently:
- `gate.summarize` passes only when every predeclared assertion and all 7 structural checks per case are `matched`;
- `structural_checks` asserts that its denominator is exact;
- the selector, reference and criteria files are hash-locked by the manifest, which the run selection locks.

A case whose selector file is refused carries no negatives in its ledger, but it is `blocked`, so the run cannot pass.

**Negative uniqueness** (assertion ID, plus unique (quantity, wrong value)). The rule is sound for its purpose, which is to stop the same wrong value being counted twice:
- quantity identity is the row ID, or (basis, record, key, field) with `definition` excluded;
- binary64 equality, so -0.0 == 0.0 (X06 killed).

It does not prevent near-duplicates a few ulps apart, and it does not check that a wrong value is distinguishable from the positive expected value. Both gaps fail closed: a negative within tolerance of the true value fails whenever its positive passes. I report them as limits, not defects.

**Re-pin.**
- `MODULE_SHA256` = `eff1fb3b…` = `git show bfef71b19:core/analysis_runs/load_reference_evidence.py` = the candidate tree.
- The start-bytes test asserts exactly that, and its message names `bfef71b19`.
- The reader's top-level imports are inside the helper allowlist.

**Admitted package.** The adapter's own admission functions accept the admitted WP6 package: 14 cases, 467 positive and 40 negative, in both modes and both purposes. No process runs (`_run_records/adapter_admission_probe.*`).

**Mutants.** 12 independent mutants; 4 killed and 8 survived (F5).

### 3. Admission (`6824b6b6b`, `c1e130818`)

**Generator path.** `ADMISSION.json` has closed keys and `status: admitted`. The review path is WORKING_ROOT-relative and its sha256 is verified; it equals the committed freeze RETURN, `795b0421…`. The path changes only:
- `readiness`;
- `independent_review_ref` (`path#sha256=…`);
- `profile_status`;
- `scope`.

Checks:
- Without `ADMISSION.json`, the generator reproduces all 28 `6824b6b6b` files byte for byte. With it, it reproduces all 28 `c1e130818` files.
- A tampered review hash is refused (`_run_records/generator_without_admission.*`).
- `--check`: 28 files, 0 differences.

**Diff `6824b6b6b..c1e130818`.** The comparison was JSON-structural (`_run_records/admission_diff.*`). All 28 changed files differ only as follows:
- references: exactly `/readiness` (pending → ready) and `/independent_review_ref` (null → the freeze ref);
- criteria: exactly `/tolerance_profile/profile_status` (draft → reviewed) and `/tolerance_profile/scope`.

No value, wrong value, rule or selector changed.

**R1–R3.** All six R1 hashes at `6824b6b6b` equal the freeze RETURN §3 table: fixed/released selectors and criteria, `MANIFEST.json` `a70eed58…` and `PROVENANCE.json` `5f766002…`.
- The R1 rule is `…zero_scale:hot_total_eigenstrain`, with absolute value `5.9984e-13`, at assertions 18/62 (fixed) and 13/47 (released).
- The README diff is exactly the R2 and R3 text.

**Manifest re-hash.** All 84 bindings in the admitted `MANIFEST.json` (14 × 5 roles plus the analytical reference) match the admitted bytes. The manifest is otherwise identical to `6824b6b6b`; only the reference and criteria hashes differ.

**check_package (no-producer).** **4461 passed, 28 failed.** The failures are exactly the 14 × 2 invariants "draft" and "pending reference". Group totals A 241, B 140, C 29 and D 4051 + 28 equal the freeze's 4489/0, so that is the only effect (`_run_records/check_package_no_producer.log`). See F4 for the provenance records that admission left stale.

## Limits

- **Pre-integration adapter bytes.** WP5's pre-integration bytes (`009ca9e8…`, `64436b47…`) are not in history, so I could not diff them against the committed adapter and test. I reviewed the committed bytes directly.
- **The F3 repair.** I did not review it. It was uncommitted in the worktree; a backcheck is pending.
- **No product runs.** I ran no VP-STATIC product comparison, runner binary or producer-mode `check_package`.
- **What I did not recheck.** The WP6 content freeze, the rulings themselves, and the WP1 readers (REVIEW_A covers the readers).
- **Predecessor references.** `PredecessorValue` `case_ref`/`support_ref` are not scanned by `delete_load_case` or `delete_support`. Ruling Q4 places them at solve time, so this is not a finding.
- **Cleanup.** The scratch clone and cargo target were deleted after the records were copied here.
