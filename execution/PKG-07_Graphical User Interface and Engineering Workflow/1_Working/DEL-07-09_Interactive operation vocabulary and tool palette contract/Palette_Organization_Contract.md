# Palette organization contract — current bounded implementation

DEL-07-09 / PKG-07 / SOW-077 / OBJ-006, OBJ-015. Producer A2_COVERAGE_FINAL under N1_WI_PKG07. All paths are project-relative.

Accepted scope truth is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.12 / DEC-094 and `execution/_ScopeChange/SCA-009_2026-08-20_0000/ACCEPTANCE_RECORD.md` with its `Vocabulary_Annex.md`. Approved derivative graph: `execution/_DAG/DAG-010/`. The annex's historical candidate header is unchanged; the acceptance record and DEL-07-09 `_CONTEXT.md` establish current consumption.

Accepted implementation inputs under `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260905-UI-TOOLKIT-PARITY/instances/N1_WI_PKG07/`: `snapshots/PHASE_A_ACCEPTED_001/MANIFEST.json`, `snapshots/M_EXISTING_ACCEPTED_001/MANIFEST.json`, `B2_ACCEPTED_MODULE_SNAPSHOT.json`, `T1_ACCEPTED_SNAPSHOT.json`, `T2_ACCEPTED_SNAPSHOT_V2.json`, `T3_ACCEPTED_SNAPSHOT_V2.json`, `T4_ACCEPTED_SNAPSHOT_V2.json`, and `T4_RENDERER_ACCEPTED_SNAPSHOT_V3.json`. Backend/native input: `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260905-UI-TOOLKIT-PARITY/N2_WI_PKG16/PACKAGE_HANDOFF_V2.md` and its B1/B2/B3/C/native accepted chain. Imported-library input: `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260905-UI-TOOLKIT-PARITY/instances/N4_WI_PKG03/PKG03_ACCEPTED_SNAPSHOT_C2.json`. Self-weight input: `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260905-UI-TOOLKIT-PARITY/instances/N5_WI_PKG05/SELF_WEIGHT_ACCEPTED_SNAPSHOT_V1.json`.

These are derivative implementation and check evidence, not decomposition truth. Current App integration is frozen at `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260905-UI-TOOLKIT-PARITY/instances/N1_WI_PKG07/children/B0_UI_INTEGRATION/UI_FREEZE_V9.json` (source hashes in sibling `SOURCE_HASHES_V9.json`) and was inspected for this reconciliation; final root N7 full source/docs review and integrated acceptance are **pending**. The accepted module snapshots do not assert that later App integration or subsequent module amendments passed N7. No lifecycle, release, professional-reliance, D58 or roadmap promotion occurs here. Source/interface changes require a fresh accepted snapshot and affected coverage recheck.

## Organization and ownership

DEL-07-09 owns coverage and organization; implementation remains in the SCA-009 annex's named deliverables. This does not create a dedicated support deliverable or change accepted implementation landings. OPS-K-DATA-1/2/3, UNIT-1, IP-1/2/3, PRIV-1, AUTH-1/2 and AGENT-1/2/3/4 continue to apply. Engineering inputs remain user-authored or lawfully imported with provenance; missing values remain missing and produce actionable diagnostics.

The searchable `ToolkitPalette` uses `toolkitCapabilities` from `apps/desktop/src/features/toolkit/capabilityCatalog.ts`. Commands navigate existing surfaces via `App.tsx::handleToolkitCommand`. Readiness means a form can open; final application still requires backend validation and explicit review.

| Group | Implemented entry surface | Vocabulary |
|---|---|---|
| Build | Node/pipe drafts and explicit continuation; typed component forms; geometry split form | 1, 14, 15, 19 |
| Supports | Rich family/restraint/hanger/nonlinear configuration; equipment/nozzle DOF form | 2, 12, 13, 16, 21 |
| Properties | Material creation/temperature table; Section grid and assignment/detach; imported libraries/hanger selection | 3, 17, 23 |
| Loads | Case/primitive/combination forms; wind/seismic fields and detailed exposure; self-weight plan | 4–8, 22 |
| Edit | Guarded selected-entity removal; copy/rotate/mirror forms; session undo/redo | 10, 18, 20 |
| Select and View | Typed tree/viewport selection/search; Entered/SI/US readout preference | 9, 24 |
| Review | Shared pending changes and batch review; offline attributed proposal intake/reference | 11 and all mutations |

Exact command IDs, focus targets and semantic operation linkages for all 24 rows and R1–R3 are in `Palette_Operation_Routing.md` and `Capability_Comparison.csv`. Roadmap help lists node renumbering, snubbers and cold spring; these remain non-executable and require an owner promotion act.

## Interaction and draft contract

Palette availability is context-sensitive: selected Material for temperature points, configured wind case for exposure, selected pipe for Section assignment, valid support context for rich support forms, and supported entity type for removal. Unavailable controls explain missing selection/input or held/native-only capability. Stable entity IDs survive focus/filter changes; queued targets snapshot the actual model basis.

Pipe continuation starts from an explicitly selected existing endpoint after the prior pipe is applied; material, section, endpoint and units stay explicit. Pointer work creates visible editable drafts. Cancelling clears transient state, not model content. Rich support configuration preserves absence versus zero and explicit removal; imported hanger replacement discloses clearing prior top-level stiffness/nonlinear settings. Material point rows have stable IDs. Wind exposure references actual pipes and explicit spans.

Forms queue intents or batches, then shared review validates and previews before explicit Apply. Errors leave the model unchanged. IDs, hashes and operation tokens belong in optional technical details. Ordinary controls use engineering labels, keyboard-accessible inputs and visible disabled reasons. Session undo/redo restores up to 25 checkpoints; a successful batch yields one checkpoint and invalidates stale computed results. This is not durable acceptance history.

## Scope limits that must remain visible

- Split/transform support bounded straight unreferenced runs. Attachments and unknown engineering references block application; finite component insert-in-run and attached engineering transforms remain residuals. Rotate/mirror requires an explicit valid local frame. Component-symbol creation remains separately implemented and historically closed for rows 14–15.
- Equipment/nozzle authoring uses explicit global free/rigid/spring DOFs with association metadata on generated supports. It does not validate external equipment, invent stiffness or persist an all-free boundary as a fictitious support.
- Self-weight uses selected pipe mass only, original-model hash, explicit signed-axis gravity and source evidence. Missing density/gravity is not fabricated; absent optional contents/insulation contributions remain disclosed. Components/support masses are outside this generator.
- Hanger import and storage are native-only; browser library import is unavailable. Selection is manual from a validated user-imported record. No protected catalog is bundled and no computed hanger sizing is claimed.
- D58 remains held. Offline file attribution is unverified identity; explicit human review/application does not establish a live provider or professional acceptance.

## Display quantities and source preservation

`DisplayUnitsProvider` wraps the App and `view.units` focuses `DisplayUnitSelector`. `useDisplayQuantity` selects catalog symbols in `features/display-units/targets.ts` and delegates numeric conversion to `services/displayQuantityService.ts` → real native/Wasm Rust units. Tree/inspector readouts, viewport scale, results, comparison, knowledge, diagnostics and rule-check quantities consume common readout components.

Entered drafts, original units, stored model quantities, hashes and source evidence remain unchanged. Raw operation diff/audit/provenance values remain source evidence, not silently converted records. Unsupported dimensions or missing catalog targets (including US targets absent for rotational stiffness/acceleration and other entries in `DISPLAY_TARGETS`), non-finite values or unavailable converter responses retain the entered value/unit with an explanation. This is a display preference with bounded conversion coverage, not a promise to convert every dimension or persist a new unit-system setting on disk. Absolute temperature and temperature interval retain distinct conversion semantics.

## Offline reference and review-context retention

The offline intake's **Offline authoring reference** downloads machine-readable capabilities and operation schema directly from the actual catalog/schema supplied by App. See `Palette_Operation_Routing.md` for source anchors; no duplicate standing register is introduced.

Original proposed member author/source/rationale/flags survive storage. On reload, retained context is visibly **acceptance UNKNOWN**, separate from executable pending batches; explicit requeue and fresh validation are required. Durable accepted/rejected chronology, decisions, grouping, receipts and acceptance timestamps remain the owning [DEL-16-03 Remaining](../../../PKG-16_Model%20Operation%20and%20Agent%20Proposal%20Framework/1_Working/DEL-16-03_User%20acceptance%20and%20operation%20audit%20trail/_STATUS.md#remaining) residual under [N2 PACKAGE_HANDOFF_V2](../../../_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260905-UI-TOOLKIT-PARITY/N2_WI_PKG16/PACKAGE_HANDOFF_V2.md).

## Evidence disposition

`Vocabulary_Coverage.csv` preserves original historical cells for rows 14–16 while adding separately labelled current evidence for all 24 normative rows and three deferred rows. Those historical closures do not imply current whole-toolkit closure. `Capability_Comparison.csv` preserves named baseline fields and records observed current implementation, exact routing and residuals.

Accepted upstream module checks and current App test definitions support bounded implementation claims. Final N7 integrated source/docs rereview remains pending; parent postrepair tests passed as recorded below; this contract claims neither final acceptance nor lifecycle promotion. A2_COVERAGE_FINAL freezes this docs slice with hashes and row completeness/history checks. Later findings require an explicit reviewed amendment; scope changes route to their owners. The immutable Phase A and historical annex artifacts remain unchanged.

## N7 F1/F2/F3 repair amendment — final rereview pending

The original [N7 REVIEW_RETURN](../../../_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260905-UI-TOOLKIT-PARITY/instances/N7_FINAL_REVIEW/REVIEW_RETURN.md) remains immutable CHANGES_REQUIRED evidence. This current derivative amendment records its bounded repairs; root N7 final rereview remains pending.

- **F3 / row 16 and support consumers:** F3 repaired: exactly nine canonical family tokens (anchor, guide, line_stop, vertical_support, spring, variable_spring_hanger, spring_hanger, constant_effort_support, nonlinear); readable UI labels emit these wire values. Explicit blank/padded/PascalCase/unknown family rejected by shared operation and physics boundaries; omitted/null legacy inference retained; explicit guide remains Guide. No new family/payload exclusivity matrix claimed. Source: `apps/desktop/src/features/support-configuration/SupportConfigurationForm.tsx`, `core/model_operations/operation_applier/src/rich_authoring.rs`, and `core/product_physics/src/lib.rs`.
- **F1 / hanger and support configuration:** F1 repaired: duplicate top-level/hanger stiffness must have the same accepted DOF and exactly equal Rust-normalized quantity; conflicts reject create/configuration and batch without publication. Existing lbf/in and lbf/ft compatibility preserved; no new tolerance or aliases. Owning accepted contract/source: `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260905-UI-TOOLKIT-PARITY/N2_WI_PKG16/F_REPAIR_ACCEPTED_SNAPSHOT_V1.json`; actual rebuilt-Wasm conflict/US backcheck: `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260905-UI-TOOLKIT-PARITY/N2_WI_PKG16/F_DUAL_BUILD_ACCEPTED_SNAPSHOT_V1.json`.
- **F2 / row 24 and boundary/support readouts:** F2 repaired: chosen complete top-level stiffness, otherwise hanger stiffness, keeps its DOF and quantity together. UX/Ux/ux through UZ/Uz/uz are linear; RX/Rx/rx through RZ/Rz/rz are rotational. SI rotational readout converts to N*m/rad; absent US target or unknown DOF retains entered source with explanation. Legacy properties.linear_stiffness remains linear; model/hash/drafts unchanged. Source: `apps/desktop/src/features/model-workspace/modelView.ts`; real converter regressions: `apps/desktop/src/features/toolkit/DisplayIntegration.test.tsx`.

Bounded UI source review: `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260905-UI-TOOLKIT-PARITY/instances/N1_WI_PKG07/N7_UI_REPAIR_SOURCE_ACCEPTED_V1.json`. F2 records 52 focused checks (51 display + one App inspector); F3 `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260905-UI-TOOLKIT-PARITY/instances/N1_WI_PKG07/children/B2_RICH_FORMS/F3_ACTUAL_ENGINE_GATE.json` records 14 non-mocked operation-Wasm tests passing. Separate owning physics evidence `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260905-UI-TOOLKIT-PARITY/instances/N6_WI_PKG04/F3_ACCEPTED_SNAPSHOT_V1.json` reports 124 native physics tests; the GUI operation tests do not execute physics and do not establish a new payload-compatibility matrix.

Current aggregate source freeze: `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260905-UI-TOOLKIT-PARITY/instances/N1_WI_PKG07/children/B0_UI_INTEGRATION/UI_FREEZE_V9.json` with `SOURCE_HASHES_V9.json` SHA256 `bbe4540585ed5e1b5bbad0732b7ce35d8b9afd440e0e31e34f905e02aa61baf7`. `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260905-UI-TOOLKIT-PARITY/instances/N1_WI_PKG07/children/B0_UI_INTEGRATION/CHECKS_V9_TERMINAL.json` records postrepair desktop build PASS, 43 test files / 732 tests PASS, and actual host built-dist Chromium regression 1 PASS (5.7 s). `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260905-UI-TOOLKIT-PARITY/instances/N1_WI_PKG07/children/B0_UI_INTEGRATION/V9_VERIFICATION_AFTER.json` records unchanged 66 source and 16 generated-artifact hashes. The docs child verified references/hashes; it did not rerun these product checks. Root final integrated rereview and parent fan-in remain pending, with lifecycle, D58, SOW-070, R1–R3 and bounded toolkit residuals unchanged.
