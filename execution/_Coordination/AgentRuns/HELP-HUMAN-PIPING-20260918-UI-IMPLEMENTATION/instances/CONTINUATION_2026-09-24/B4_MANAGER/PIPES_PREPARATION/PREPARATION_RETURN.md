# Pipes and remaining-family preparation return

Read-only TASK return to `/root/b4_manager` (WORKING_ITEMS), executed by delegated-harness-native child `/root/b4_manager/pipes_preparation`. No descendants. Accepted product basis: 42bd234f671508ba9ab157e1981334ba8610279f. Evidence-only write scope is this directory; actual host allows broader workspace writes, which were not used. No product edits, tests, builds, browser/native actions, application processes or network. Shell tools were used only for source inspection and this evidence artifact. No new family implementation activated.

The earlier next-preparation result is retained: finish the compact Model/narrow-Both fit slice, then Pipes is the next coherent family. The corresponding brief was found one directory above the supplied `_run_records` location; both were read. This return refines supported semantics rather than reopening Nodes, Materials, Sections or fit diagnosis.

## Exact Pipes migration boundary

All source citations below refer to 42bd234f671508ba9ab157e1981334ba8610279f; paths are relative to `projects/chirality-piping/`.

| Current column | Supported route and bounded migration |
|---|---|
| Label | `Element`, `set_field`, `label`, none/dimensionless. Engine Text; direct trimmed nonblank string, explicit TBD allowed. Review retains its existing queue contract. |
| Shared section | `section_ref`, readonly in grid. Keep dedicated SectionAssignment assign/detach route; never scalar `set_field`. |
| From / To | `from` / `to`, readonly in grid. Neither appears in Element scalar FieldRules. Do not invent endpoint reassignment. |
| Mill tol. | `Element`, `set_field`, `section.mill_tolerance.value`, length. OptionalQuantity NonNegative, an absolute thickness reduction, not a percentage. Explicit zero is meaningful. |
| Material | `Element`, `set_field`, `material`, none/dimensionless. Required EntityRef to an existing `materials` id; not arbitrary text, not a material-name match or implicit creation. |
| Provenance | `Element`, `set_field`, `provenance`, none/dimensionless, engine Text. Direct/review editing only when actual provenance is a string. Preserve structured values read-only; current guard only covers node/material/section and must not be copied as blanket permission for Pipes. |

Evidence: `apps/desktop/src/features/model-tree/ModelTree.tsx:1215–1231,1580–1624`; `core/model_operations/operation_applier/src/lib.rs:398–466,6790–6829`. Shared-section dedicated controls: `apps/desktop/src/features/toolkit/SectionAssignment.tsx:53–60,78–105`; engine contract `lib.rs:1380–1412` requires Element/section_ref/none/dimensionless, whole-pipe canonical before value, assign payload `{section_ref: id}` or detach payload `{source_section_ref: id}`. These are distinct from common cell scalar before values. Bound OD/wall continue through Sections or explicit detach, not new Pipes columns.

## Optional tolerance and reference requirements

The existing generic grid loses absence semantics: missing values project to empty string (`ModelTree.tsx:1670–1682`), while optional engine authoring checks before `TBD` and writes the complete value/unit object (`lib.rs:7295–7318`). The migration must project absent tolerance as TBD for its operation before, without treating absence as zero. Nonnegative finite replacement only; blank, NaN/Infinity, negative and TBD replacement are not numeric authoring. There is no clear/delete optional-quantity route in this FieldRule; do not present blank as deletion.

Preserve the actual entered unit for existing quantities, including explicit zero. For absent tolerance require an explicit visible unit choice/confirmation; do not synthesize an unshown `m`. The inspector currently exposes a unit-editable field and selects tolerance unit, then wall unit, then project length, then `m` (`PropertyInspector.tsx:1674–1684`); generic grid uses project length fallback and cannot edit units. That existing fallback is not evidence that absent units were entered by the user. A safe bounded option is to expose the proposed unit visibly and require explicit entry/choice before first authoring, or leave absence disabled with a pointer route to the inspector until the opt-in optional editor is implemented.

Engine requires intent dimension length and an accepted unit symbol; explicit JSON `{value,unit}` must match intent unit. Existing numeric value without stored unit is blocked. Explicit value/unit payload can change stored unit without hidden conversion; bare numeric payload must retain stored unit (`lib.rs:7140–7318`). Do not reuse Sections' required-existing/positive admission: tolerance can be absent and zero is valid. Do not compare empty/TBD numerically (Number(empty) is zero), and include unit in edit capture, no-op and stale checks. Shared-section cache and effective-wall checks remain engine authority: bound OD/wall local edits reject; tolerance remains local but must leave positive effective wall and coherent bound geometry (`section_bindings.rs:76–99,214–237`; `lib.rs:1320–1347`). No frontend promise that finite/nonnegative alone guarantees application.

Material editor should be opt-in typed reference behavior: options show current model material labels with exact ids; stored/emitted value is the exact id. Unknown, blank, missing current field and removed options remain invalid; engine revalidates at application. Duplicate labels must never choose implicitly. Pointer-accessible list/choice plus keyboard completion should share direct/review renderer and preserve draft/focus across disclosure, family and generation changes. The existing table kind system has text/quantity/enum options, so avoid silently relabeling all text fields as references. Required EntityRef resolver trims replacement, checks current string/before and existing collection membership (`lib.rs:6790–6829`).

## Common grammar and proof boundary

Reuse EngineeringTable's direct/review rendering and typed structured operation path, not a parallel model. Current numeric grammar accepts signed decimal/scientific finite numbers (`tableState.ts:22–24`); it does not yet implement the adopted UX's dimensional suffix/US feet-inch vocabulary. Keep that limitation visible and scoped; do not claim full §3.1 grammar. Direct text must reject blank before the adapter's `trim() || TBD` fallback. Review queue behavior, visible changed-cell capture, retained unqueued drafts, per-operation history, stale-generation guard, rejection visibility and undo/redo must remain truthful. Quantity ordering must use the existing projection contract rather than lexical display strings. Apply/Cancel/Queue/Clear and reference choices require reachable pointer controls in finite compact bodies, both densities and unit modes. No optimistic accepted-model writes or queued-as-applied wording.

The handoff §3's one mutation route, canonical projections, result/history integrity, no fabricated controls, retained rendering foundation and separate typed/engine gaps all apply. UX §2.3 and §3.1 inform table fit, editing, reference resolution and pointer behavior; specification aspirations do not manufacture unsupported operations.

Recommended smallest coherent next executable slice, after compact fit review/native/integration closes: Pipes Label + string Provenance + Material typed-reference editor + optional Mill tol. editor, retain all three readonly reference columns. One integration writer owns ModelTree, modelTableAdapter, tableState/EngineeringTable only as needed for opt-in reference/optional quantity behavior, scoped styles and focused tests. No engine/schema/controller/VirtualList changes are justified by this preparation. If optional authoring cannot be completed safely in that slice, retain an explicitly disabled cell with inspector route rather than pretend family completeness. Required later proof: direct/review payload equivalence, zero/absent/existing tolerance, mixed actual units, missing unit, unknown/duplicate-label references, removed option while editing, effective-wall/binding rejection, unchanged model/history on rejection, stale before, retained drafts, sorting/filter self-removal, undo/redo and save/reopen, plus compact native pointer reach. These are proposed checks, not performed results.

## Remaining supported families

- Supports: `update_support` label/provenance, node EntityRef, restraints comma-separated normalized set; Family/Hanger readonly. Dependent nonlinear normal-reaction references can block node/restraint changes (`ModelTree.tsx:1233–1254`; `lib.rs:467–486,1348–1377,6831–6855`). A coherent next follow-on is Supports using proven reference behavior plus a dedicated restraint-set editor; not a scalar-only migration.
- Components: label/provenance and node reference, many optional geometry, mapping and modifier inputs, each with field-specific positivity/dimensions. Kind readonly; center_of_gravity remains unsupported (explicit source comment). Migrate by coherent component kind rather than all visible optional scalar-looking columns (`ModelTree.tsx:1289–1450`; `lib.rs:487–726`).
- Load cases: `update_load` text label/kind/status/provenance; optional modulus basis id and temperature, equivalent-static numeric inputs, wind direction enum and exposed-pipe reference list. Matching/bracketing/no-extrapolation remain solve-side contracts. A bounded basic text/identity slice is possible, but does not complete specialized load editing (`ModelTree.tsx:1452–1558`; `lib.rs:727–821`).
- Combinations: existing `set_field` label/basis/provenance text is a simple supported slice; terms/expression/stress-rule expansion is not established by those scalar routes (`ModelTree.tsx:1559–1563`; `lib.rs:822–835`).
- Materials/Sections are predecessors already migrated; library breadth, joined/expanded rows, common copy/CSV and session Checked remain separate B4 work. This inspection did not reopen or validate those implementations.

Current parent-supplied allocation: generic table and Supports DEL-07-02; specialized Materials/Sections/Components/library/load-case+combination DEL-07-03; shell DEL-07-11; review DEL-07-08; execution evidence DEL-16. No allocation/scope amendment is made. Pipes recommendation depends on compact fit completing, not C4/B5/CLI decisions; this artifact grants no activation, acceptance or release.

## Consulted identity manifest

SHA-256 hashes below are exact bytes read (product files from fixed Git revision; guidance/design/prior records from workspace). No full alternate role or selected skill/workflow was loaded.

- `42bd234f671508ba9ab157e1981334ba8610279f:projects/chirality-piping/apps/desktop/src/features/model-tree/ModelTree.tsx` — `c0bc4b0abda319728cd16f117875dfdc758b7a1383f9c89e952bce7e9dc38459`
- `42bd234f671508ba9ab157e1981334ba8610279f:projects/chirality-piping/apps/desktop/src/features/model-tree/PropertyInspector.tsx` — `3120f29cb431fe649189a41048b95923487ff6140878dcb22bbad6b23a2d3f89`
- `42bd234f671508ba9ab157e1981334ba8610279f:projects/chirality-piping/apps/desktop/src/features/workspace/table/modelTableAdapter.ts` — `663f0edf7945edb39e9d2c9cc60c85a75b30f05266c8a8d875135c162394622b`
- `42bd234f671508ba9ab157e1981334ba8610279f:projects/chirality-piping/apps/desktop/src/features/workspace/table/tableState.ts` — `8f8a159741f7f9389dddee17378d35466877353bab441be6fe9291b4817ba403`
- `42bd234f671508ba9ab157e1981334ba8610279f:projects/chirality-piping/core/model_operations/operation_applier/src/lib.rs` — `22062ea42fd135a9dbefa0f7e09cf3a2feccc5193ff6028818fd1c6221a3f355`
- `42bd234f671508ba9ab157e1981334ba8610279f:projects/chirality-piping/core/model_operations/operation_applier/src/section_bindings.rs` — `424cba3f1e6292d75233bd403b60c922e2e56365c7c7b15b463c6efe58d9fd72`
- `AGENTS.md` — `c8ce87ef342902cb081bc659b26fc9a4edda1b6dba513814e5cb1e14e0b1dffd`
- `agents/AGENT_TASK.md` — `1a13a5b00b3ce01ff8519efe6b46bcbe0cd6a5b7985e24282fa7efa2c57c8fb7`
- `projects/chirality-piping/AGENTS.md` — `d9481951912ceffdd5bc47dbb6549bf0044f5d92f9fe6a9b11ba5969bfebc792`
- `projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/ROOT/CONTINUATION_2026-09-19_CODEX/B4_NEXT_PREPARATION_BRIEF.md` — `0bce9d60a09fb54ba2cac4e80a64b71ee4bcabd9ed458097be2d9c7626c04699`
- `projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/ROOT/CONTINUATION_2026-09-19_CODEX/_run_records/B4_NEXT_PREPARATION_RETURN.md` — `c6d8ece0b35b36db2d9fd71a7e35a084bef15df0e3663544883d926f401e26f8`
- `projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/ROOT/CONTINUATION_2026-09-19_CODEX/_run_records/B4_REMAINDER_PREPARATION_RETURN.md` — `065aeb1bc0bcb74752612fbc7f27913aa7f7ae104a75071867822dc2ddbde4ef`
- `projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260916-UI-DESIGN-PROGRAM/instances/ROOT/IMPLEMENTATION_HANDOFF_2026-09-18.md` — `0711b0e35f8d02f404dcae9a2b440bcf4506d5f0f9b76dc748195952bf7ab9a7`
- `projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260916-UI-DESIGN-PROGRAM/instances/UX-SPEC/UX_SPEC_V1.md` — `2141c1e844109d4869287048c695e79677f423acae965d5dd0436b7f6aeff682`
- `42bd234f671508ba9ab157e1981334ba8610279f:projects/chirality-piping/apps/desktop/src/features/toolkit/SectionAssignment.tsx` — `26e27ecdb47a5c3eac396fc71113c27ff083d3f50f6d1d7a8ebce63e5a7bac0a`
