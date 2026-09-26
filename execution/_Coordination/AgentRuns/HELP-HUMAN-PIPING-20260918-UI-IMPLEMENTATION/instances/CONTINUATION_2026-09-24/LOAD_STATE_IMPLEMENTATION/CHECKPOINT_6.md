# Checkpoint 6 — T0R merge into T1 (S6 §6 obligations)

- **Author:** the session-3 T1 WORKING_ITEMS manager, parent HELP_HUMAN (ROOT).
- **Branch:** `codex/piping-load-states-20260925`. Merge commit `a9528b2e1`, parents `963f1ceb2` (the wave-1 head) and `82b43f9bd` (main after PR952, T0R `preview-physics-1`). A merge commit, no rebase, as ROOT directed.
- **Paths:** WORKING_ROOT-relative. Machine records are in `_run_records/session5/`.
- **Status:** completed execution, not acceptance. No M10, M16 or M29 finding closes here.

## Authority

- ROOT's instruction of 2026-09-26 (merge origin/main; the S6 §6 obligations; both standing test sets and the T0R 22-entry tamper vector must pass).
- T0R `DEFAULT_ROUTE_DESIGN/DESIGN.md` §10.2–10.3 and `DEFAULT_ROUTE_DESIGN/IMPLEMENTATION/S6_RECORD.md` §6 (T1 merge, R1 N-3).
- `DEFAULT_ROUTE_DESIGN/ROOT_SELECTION.md`: T1's early `needs_recompute` for `load-reference-source-1` is kept, and validation runs first.

## Resolutions

| Item | Resolution |
|---|---|
| `LoadCaseSolve` literals | `preview: None` in T1's two early returns (member section missing; prescribed-DOF blocking). |
| Standing, Rust `numerical_use_standing_with_context` | Order: `for_source` validation, then T0R's `standing_reason`, then T1's declared early `needs_recompute` for `load-reference-source-1`. |
| Standing, Python `numerical_use_standing` | The same order: `_source_contract` validation, then a contract outside `FRESH_CONTRACT_IDS` or with a `_standing_reason` gives `needs_recompute`, then the joined early return. |
| Static fresh set | `load-reference-1` and `load-reference-source-1` added in Rust (`FRESH_IDENTITIES`), Python (`FRESH_CONTRACT_IDS`) and TS (`FRESH_SEMANTIC_CONTRACT_IDS`; the two constants sit with the others in `numericalResultQuality.ts`). They are 0.4.0 exact-route identities only. T0R's three set pins, which anticipated this, now name the exact six identities, so they are stricter, not weaker. |
| Blocked 0.4.0 envelopes (S6 §6: "gets the preview id until T1 routes it") | Routed. A 0.4.0 document is exact-route only, so every blocked 0.4.0 envelope, including one without or with a legacy pressure contract, carries `load-reference-1`, its profile and the empty load/reference namespace. It never carries `preview-physics-1`. `formulation_basis_for_model` checks the load state first; `blocked_envelope` treats 0.4.0 as the exact namespace. New test `case_state::tests::blocked_load_state_envelopes_keep_the_load_reference_identity` (three blocked shapes, both modes). No solved path changes: a non-exact 0.4.0 document never solves. |
| Joint-element refusal | `refuse_unqualified_joint_elements` applies on every route, the 0.4.0 route included. This is conservative; the M07 repair belongs to T4. |
| `for_source`, the derivative union, the `package_v0_3.py` sets and `_source_contract` flags | Union of both sides. `physics-1` keeps T1's `forbid_load_reference_evidence` guard. |
| Carrier schemas (results, AnalysisRun, stress-neutral) | Merged structurally: main's order first, T1's appended. ResultEnvelope, `SemanticContract`, `AnalysisRun` and package `oneOf` order: precision, physics, source-blocks, physics-source, preview (4), load-reference-1 (5), load-reference-source-1 (6). |
| Schema pins | `test_source_block_schema_contract.py`, `test_load_reference_schema.py` and `test_load_reference_source_schema.py` now locate branches by identity and pin the relative order. The pinned content per branch is unchanged. |

## Checks (on `a9528b2e1`, shared target, `CARGO_INCREMENTAL=0`)

| Check | Result |
|---|---|
| product_physics, all targets | lib 331 passed, 1 ignored (the existing bounded resource measurement in `source_receipt/tests.rs`), and 13 integration targets all passing, including `preview_physics_runtime` 26, `load_reference_state_runtime` 20 and its extension 9 |
| result_export, all targets | 91 passed, including `preview_physics_contract` 19 with `a2_shared_tamper_vector` (22 variants), and the load-reference and joined contract suites |
| runner/headless | 69 passed, with the source-block, physics and precision artifact lanes enabled |
| operation_applier | 194 passed |
| Python (27 files: AnalysisRun, LR and joined readers and schemas, physics, physics-source, precision and preview consumer contracts, qualification adapters, results and viewer, source-block schema and validation, the three stress-neutral suites, model and persistence schemas) | 1231 passed, 109 subtests. The 11 source-block artifact gates ran on actual headless artifacts. Then the gated physics, precision and LR/LRS parity lanes: 4 passed. **Skips:** 2, `test_physics_consumer_contract.py` `PHYSICS_BLOCKED_OUTPUT_DIR`. No lane in the tree produces those artifacts, on main or here |
| T1 schema tests after the re-pin | 787 passed (the 11 artifact gates were skipped on that run, then run as above) |
| Desktop vitest, `src/features/results/` | 13 files, 334 passed, including `knownSemanticLimitations` and the TS A2 tamper vector (wasm engine built from this tree) |
| Standing tests | T0R's standing suites (Rust, Python, TS) and T1's joined early-return tests: all passing within the counts above |
| Producer raws | `cp4_regen_compare.py`: 34/34 regenerated raws match (`merge_regen_compare.log`) |
| VP-STATIC | Scratch rerun on the merge commit: both modes `all_required_assertions_matched`, 507/507, 14 `checks_passed`. All 42 runner stdin, stdout and stderr artifacts per mode are byte-identical to the recorded run. The only differences are run-specific: the ledger, selection, reader-basis bytes and reader snapshots (`merge_vp_static_rerun.txt`). The recorded run remains the one in `T1_VP_STATIC_RUN/`. |

**Not run here:** the full piping pytest sweep, full desktop vitest and build, Chromium e2e, the DEC-025 sweep, native checks and hosted CI. These belong to WP7.

## Next

WP2 (desktop types, TS load-reference readers and persistence), the WP3 native fields, WP4 (headless and CLI), then independent review and WP7. The briefs are in `TASK_BRIEFS/`.
