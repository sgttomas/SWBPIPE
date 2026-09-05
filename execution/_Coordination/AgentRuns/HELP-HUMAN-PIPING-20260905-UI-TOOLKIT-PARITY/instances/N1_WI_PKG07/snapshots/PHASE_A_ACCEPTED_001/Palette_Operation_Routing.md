# Palette-to-operation routing — Phase A draft

Identity: DEL-07-09 / PKG-07 / SOW-077 / OBJ-006, OBJ-015. Producer A2_MAP; parent N1_WI_PKG07; run HELP-HUMAN-PIPING-20260905-UI-TOOLKIT-PARITY.

Derivative basis: accepted SOFTWARE_DECOMP revision 0.12, DEC-094, SCA-009 accepted snapshot and approved DAG-010. Baseline code: `740569598f9d00440636b8ea25264127f418e4ec`. Companion `Capability_Comparison.csv` is the complete 27-row baseline; `Palette_Organization_Contract.md` defines interaction behavior. This draft creates no lifecycle acceptance and does not replace immutable upstream truth. All paths below are relative to the project root unless explicitly marked repository-root.

## Common executable route

Human draft → shared `EditorOperationIntent` → `validateModelOperation` → Rust `validate_operation` / deterministic diff → explicit Apply → `applyModelOperation` → Rust `apply_operation` → returned session-model update and receipt. `apps/desktop/src/services/operationService.ts` routes native Tauri commands or Wasm calls to `core/model_operations/operation_applier`; it does not implement validation/apply semantics itself.

At baseline, `EditorOperationIntent` (`apps/desktop/src/types.ts`) carries operation ID/kind/proposed status, `author_type: user`, typed target, change ID/kind/path/before/after/unit/dimension/source note, validation status, audit boundary and professional-boundary flags. Intent edits are snapshots of their model basis, not live links to a later selection. `App.tsx::handleApplyIntent` stores the returned outcome and commits only a successful applied model.

The baseline executable change-kind set is `set_field`, `update_load`, `update_support`, `create_node`, `delete_node`, `connect_pipe_run`, `delete_pipe_run`, `create_section`, `create_material`, `create_support`, `delete_support`, `create_load_case`, `delete_load_case`, `create_primitive_load`, `delete_primitive_load`, `create_combination`, `delete_combination`, `create_combination_term`, `delete_combination_term`, and `insert_component_symbol` (`operation_applier/src/lib.rs::check_kinds`). Component creation uses the last token, not a new `create_component` token. Broader schema `OperationKind` tokens do not add executable semantics.

## Operation dependency contracts

This table separates the baseline from manager-reported in-flight work. The latter is a coordination input, not a claim that a resolver is implemented or tested. Parent managers freeze the actual returned contract before UI work relies on it; type declarations belong to the UI integration owner.

| Contract | Rows | Required structured meaning | State and owner |
|---|---|---|---|
| Existing editor exposure | 1, 2, 4, 5, 7, 8, 14, 15 | Existing target refs/change kinds; reuse actual builders and shared validators | Baseline executable. GUI owner per annex |
| Section fields | 17 | `set_field` on Section name, section_type, properties.outside_diameter.value, properties.wall_thickness.value and provenance; units and geometry constraints | PKG-16 implementing exact current grid fields; baseline rejects Section via `collection_for` |
| Guarded removals | 18 | `delete_material`, `delete_section`, `delete_component`, target ID plus current basis; reference blockers, no cascade | PKG-16 implementing; consume tested taxonomy and payload contract before enabling controls |
| Support configuration | 2, 12, 13, 16 | `update_support` field `configuration`, whole object `{family, restraints, stiffness?, hanger?, nonlinear?, provenance}`; `create_support` rich payload uses same support fields | Parent-reported B2 agreement; PKG-16 implementation pending at contract production |
| Material temperature table | 3 | `set_field` field `temperature_points`, whole stable-ID point array; typed temperature/E/G/alpha, provenance and explicit missing/removal semantics | Parent-reported B2 agreement; PKG-16 implementation pending |
| Detailed wind exposure | 6 | `update_load` field `equivalent_static.wind.exposure`, whole `{exposed_pipe_refs, exposed_spans}`; referenced pipes and span validation | Parent-reported B2 agreement; PKG-16 implementation pending |
| Whole-record encoding | 3, 6, 12, 13, 16 | Canonical JSON `before` and JSON payload `after`; nested quantities carry dimensions/units; envelope unit `none`, dimension `dimensionless` | Parent-reported B2 transport convention; use backend canonicalization contract rather than ad-hoc ordering |
| Section assignment | 17 | Explicit stable section reference and physical-model consumption/snapshot policy | Held pending PKG-02 shared-ref contract; do not pretend current inline `Element.section` fields establish it |
| Split / insert | 19 | Explicit source element, split location/unit, resulting stable IDs and reference disposition; atomic all-or-nothing validation/application | DEL-02-01 semantics → DEL-16-01 resolver → DEL-07-01 UI |
| Copy / rotate / mirror | 20 | Selected source refs, new-ID mapping, pivot/axis or plane and units/angle; explicit attached entity/reference treatment | DEL-02-01 semantics → DEL-16-01 resolver → DEL-07-01 UI |
| Nozzle/equipment boundary | 21 | Owner-defined boundary model and DOFs/coordinate/reference semantics; reuse support route only when physically equivalent | DEL-02-03 + DEL-04-03 → operation contract → DEL-07-01/02 UI |
| Self-weight generation | 22 | Explicit gravity, direction, selected pipes and mass/provenance inputs; ordered standard create_load_case + distributed_force primitive operations; no defaults | Parent-reported PKG-05 generator direction; mass inputs DEL-03-08. Ordered list alone does not establish transaction semantics; GUI must resolve partial-failure handling |
| Imported hanger selection | 23 | User chooses imported record; preserve library/record provenance identity and map to support payload; no sizing calculation or catalog data | Parent-reported PKG-03 manual selection direction; DEL-03-02/07 import contract → DEL-07-03 UI → tested support payload |
| Display preference | 24 | Presentation-only system selector shared by quantity renderers; no stored-value/unit or model-hash changes | Parent-reported N2 converter contract: convert_display_quantities_json batch {id,value,from_unit,to_unit,dimension_id} returns per-item converted/unavailable/value/unit/message. SI/US/entered preference remains storage/hash/draft-neutral; no frontend conversion constants; unsupported values remain in entered units without a full-conversion claim. DEL-02-02 → DEL-07-02 UI |
| Future agent origin | 11 | Same explicit parameters, hashes, constraints, deterministic diff and acceptance/audit boundary as human | D-58 held; no provider connection or agent application authorized by this derivative contract |
| Roadmap | R1–R3 | No executable payload proposed here | Deferred; future owner promotion required |

Creation/update of optional rich support data must distinguish absence, explicit zero where meaningful, and user-requested removal. Canonical `before` snapshots and nested quantity validation are required for whole-record updates. Backend contract tests must cover unknown keys, invalid enums, incompatible dimensions, dangling references, duplicate IDs and stale before/basis data before forms advertise support.

For node/pipe/component tools, selection remains a typed entity reference through focus changes. Use exact existing `componentIntent.ts` builders for current component payloads; no generic arbitrary JSON form should become the normal human workflow. Array editing needs stable row keys/IDs where the owning model defines IDs; it must not reinterpret a reordered row as the old target. Full-record replacement is acceptable only under the above guarded backend contract.

## Source ownership and execution graph

The initial reconnaissance found material, section, support and component creation plus field editing colocated in `features/model-tree/PropertyInspector.tsx`. Parallel children must not independently edit that file. `App.tsx`, `types.ts` and shared styles are integration hotspots as well.

Recommended bounded nodes, assigned by owning manager only after Phase A is validated:

| Node | Exclusive source scope | May run alongside | Prerequisite / return |
|---|---|---|---|
| UI integration contract/extraction | `types.ts`, `App.tsx`, `PropertyInspector.tsx`; newly extracted shared intent/draft contracts | Backend owners in separate packages; independent contract review | First freeze props and operation payload interfaces, then stop editing child-owned extracted files while children work |
| Palette and viewport | `features/viewport/PipeViewport.tsx`, new palette catalog/component and focused tests | Standalone support/editor modules after interfaces freeze | Uses agreed callbacks; returns command availability and entry-route evidence; no independent App/type edits |
| Rich support form | New support configuration form module and tests only | Palette and material/wind modules | Tested B2 support payload and frozen form props; returns intents, never model writes |
| Material/section/wind forms | New extracted editor modules and focused tests only | Support and palette | Tested B2 field contracts and exact Section/deletion contracts; manager can split further only into disjoint files |
| Tree/grid repair | `features/model-tree/ModelTree.tsx` and focused tests | Standalone forms | Section resolver result; otherwise render unsupported fields read-only with reason. Ref deletion remains blocked until returned tested contract |
| Imported library UI | `features/library/LibraryManagerPanel.tsx` and new selection module/tests | Other standalone forms | PKG-03 import and support payload contract |
| Load generator UI | `features/load-cases/LoadCaseManagerPanel.tsx` and tests | Other standalone forms | PKG-05 generator and list application/failure contract |
| Display-unit UI | Shared display module and explicitly enumerated renderer files | Only work with disjoint renderer scopes | DEL-02-02 contract; broad App/inspector/result changes serialize through integration owner |
| Final UI integration | App/types/inspector/shared styles, after child returns | Fresh read-only reviewers after diff freeze | Wire modules, update typed outcomes/selection for added kinds, targeted checks and full-diff independent review |

No proposed node changes accepted implementation landing or grants scope. DEL-07-09 does not dispatch implementation. N1_WI_PKG07 owns package execution and escalates cross-package contracts to HELP_HUMAN. Shared-file changes serialize; independent modules/backend packages may run concurrently only with explicit write boundaries. Tier-1/2 existing-capability wiring precedes Tier-3 implementation per SCA-009. A generator or geometry operation that depends on new semantics cannot be presented as mere UI wiring.

There is a potential objective-relative dependency cycle if the frontend waits for backend payloads while the backend waits for final form design. Resolve it by the recorded decomposition move: freeze the minimum operation and component-prop contracts first, then independently implement producers/consumers, then integrate and verify. This is a proposed decomposition of the work, not a cut or merge of accepted scope or a new gating edge in DAG-010. Parent work graph must record the adopted move; unresolved cycle-participating assumptions remain non-gating.

## Closure and refresh

Phase A child verdict: contract artifacts produced for review; toolkit implementation remains open. Parent must freeze the validated phase output before dependent use, preserving the accepted upstream snapshots and derivative status. Recompare all 24 normative rows against the final frozen implementation and attach actual validation/review evidence; do not overwrite the historical `Vocabulary_Coverage.csv` merely to make the baseline look current.

Remaining blockers are operation/reference contracts in the table, D-58 live-agent integration, and the full code-review/testing obligations. R1–R3 remain deferred rather than blockers of normative-now coverage. This child neither runs broad tests nor changes product source, lifecycle/register/pointers or Git state.
