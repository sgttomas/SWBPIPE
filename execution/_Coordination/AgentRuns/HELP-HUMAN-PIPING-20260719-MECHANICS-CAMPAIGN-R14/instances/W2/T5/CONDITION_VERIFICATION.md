# T5 Condition Verification — DEL-05-04 PDU-037 Runtime Stale-Hash Acceptance-Reuse Negative

**Run:** `HELP-HUMAN-PIPING-20260719-MECHANICS-CAMPAIGN-R14` / W2 / T5
**Author:** WORKING_ITEMS (Agent 1, PKG-05 package manager)
**Date:** 2026-07-20
**Verified at:** branch `claude/piping-r14-pkg05-loads`, HEAD
`581a15b1c718fd444870f13e75fc7cd974518670` (post-wave-1 main, PR #292 merged)
**Disposition:** `CANNOT_ADVANCE — CONDITION_NOT_MET` (no execution, no
candidate brief; the Remaining row stays open unchanged)

## 1. The Conditional Row

DEL-05-04 `_STATUS.md ## Remaining` (sole item):

> Add the PDU-037 runtime stale-hash acceptance-reuse negative when an
> owning acceptance runtime exists; current const/schema checks do not
> demonstrate runtime invalidation or release-gate behavior.

The row is expressly conditional: the negative test may be added only
"when an owning acceptance runtime exists." The R14 campaign plan directs
this manager to FIRST verify that condition against the live tree and to
record a truthful advance/cannot-advance result.

## 2. What Was Opened (live tree)

- `execution/PKG-05_Loads, Load Cases, and Stress Recovery/1_Working/DEL-05-04_Analysis status semantics/_STATUS.md`
  (`IN_PROGRESS`; sole Remaining row quoted above) and `MEMORY.md`
  (2026-07-12 D-41 R5 T6 entry: "Declaration-level hash invalidation
  remains verified; a runtime stale-acceptance-reuse negative and release
  gates remain absent").
- `_run_records/WORKING_ITEMS_RUN_2026-07-12_D41-R5-T6-PDU037.md`
  ("Runtime stale-acceptance reuse and release gates remain absent").
- PDU-037 record:
  `execution/_Reconciliation/DeliverableConcordance/DELIVERABLE_CONCORDANCE_2026-07-11_1305/PROPOSED_DELIVERABLE_UPDATES.csv`
  row `PDU-037` (six-deliverable verification-refresh proposal) and the
  underlying claim row `DEL-05-04-REQ-014` in
  `.../WAVES/W2/CLAIM_CONCORDANCE_DEL-05-04.csv`, whose recorded evidence
  states: "no test exercises acceptance invalidation after a bound-content
  hash change; release gates do not yet exist" and "the requirement's
  subject gates do not yet exist."
- Cited implementation surfaces, opened and grepped in the live tree:
  `schemas/analysis_status.schema.yaml`,
  `schemas/project_persistence.schema.yaml`, `core/gui/pkg02_boundary.py`,
  `core/project_persistence/service.py`, `core/runner/headless/src/lib.rs`,
  `core/rules/rule_check_runner/src/lib.rs`,
  `apps/desktop/src-tauri/src/lib.rs`, desktop services,
  `tools/release/run_release_gate_records.py`, and repo-wide searches for
  `HUMAN_APPROVED_FOR_PROJECT`, `human_acceptance_records`,
  `human_acceptance`, `bound_hashes`, `acceptance_record`, `release_gate`.

## 3. Condition Findings (each independently checkable)

1. **No runtime stores, loads, matches, reuses, or invalidates a human
   acceptance record.** `schemas/analysis_status.schema.yaml` declares
   `human_acceptance_records` as "Optional external records only" with
   required `bound_hashes` — a schema shape, not a runtime.
   `core/project_persistence/service.py` (envelope builder,
   `build_project_persistence_envelope`) always emits a literal empty
   `"human_acceptance_refs": []` and has no read/reuse path. The desktop
   store (`apps/desktop/src-tauri/src/lib.rs`, `local_projects` /
   `local_rule_packs` / `local_libraries` sqlite tables) persists no
   acceptance record or acceptance reference column.
2. **Every live `HUMAN_APPROVED_FOR_PROJECT` code surface is a negative
   declaration, not an owner.** `core/gui/pkg02_boundary.py` publishes
   const policy flags (`human_acceptance_invalidates_on_hash_change:
   True`, `bound_hashes_required_for_human_acceptance: True`) and emits a
   blocking diagnostic when software tries to EMIT the acceptance status;
   `core/runner/headless`, `core/rules/rule_check_runner`, and the desktop
   command tests only assert the status is never software-emitted. None of
   these consume or invalidate a stored acceptance record.
3. **No release gate runtime exists to exercise.**
   `tools/release/run_release_gate_records.py` (DEL-09-05) is a
   gate-outcome record emitter over already-governed artifacts; its own
   docstring records that it is not a release/readiness claim, criterion
   outcomes never affect its exit code, and human-gated criteria are
   recorded `TBD`. It does not read acceptance records. The concordance
   claim row records "release gates do not yet exist."
4. **The declaration-level checks named by the row are exactly what
   exists, and nothing more.** `tests/test_analysis_status_schema.py`
   asserts `bound_hashes` membership; `tests/test_persistence_schema.py`
   asserts `HumanAcceptanceRef` requires `invalidates_on_hash_change`
   const true; `tests/test_model_state_schema.py` asserts
   `ImmutabilityPolicy.hash_invalidates_external_acceptance` const true —
   all const/schema declarations, matching the row's own
   characterization.
5. **Human acceptance workflow ownership/storage/UI remain recorded
   `TBD`** in DEL-05-04 `MEMORY.md` (2026-05-01 entry, carried forward
   2026-06-04), consistent with findings 1–3.

## 4. Disposition

The condition "an owning acceptance runtime exists" is **not met** in the
live tree. A runtime stale-hash acceptance-reuse negative test cannot be
written truthfully today: there is no runtime acceptance-reuse behavior to
drive to a negative outcome, and inventing a synthetic "runtime" solely to
fail it would fabricate the subject under test and misrepresent coverage.

- **T5 disposition:** `CANNOT_ADVANCE — CONDITION_NOT_MET`.
- The DEL-05-04 Remaining row, `_STATUS.md`, `MEMORY.md`, and run records
  are left byte-unchanged; no candidate brief was authored and no
  execution occurred.
- The row correctly self-arms: it becomes selectable if and when a future
  lawful selection lands an owning acceptance runtime (acceptance record
  storage/reuse in the persistence or desktop surface, or actual release
  gates consuming acceptance), at which point the negative test and any
  release-gate behavior check become writable against real behavior.

This record is verification evidence only. No lifecycle, review,
acceptance, release, threshold, or scope act occurred; no claim is made
beyond the opened artifacts and searches listed above.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
