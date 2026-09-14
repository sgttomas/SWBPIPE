---
doc_id: PIP-SCA009-VOCAB-ANNEX
doc_kind: scope_change.vocabulary_annex
status: candidate_normative_text_gate3_not_approved
date: 2026-08-20
amendment_id: SCA-009
decomp_variant: SOFTWARE
current_basis_commit: 89758a32634ee6cedbd1dbadf35e3728fb48d2eb
decomposition_revision: "0.11"
---

# Piping SCA-009 Vocabulary Annex — ratified two-class interactive operation vocabulary

**State: `CANDIDATE NORMATIVE TEXT — GATE 3 NOT APPROVED`**

This annex is the durable transcription of the vocabulary the owner ratified
in the D3 ruling (2026-08-20; verbatim in `ACCEPTANCE_RECORD.md`). It is the
candidate coverage contract that `DEL-07-09` will own. It becomes
decomposition-bound normative truth only through SCA-009 Gates 3–5; nothing
here amends any live surface today.

Gate-2 modification (owner ruling, 2026-08-20: "Yes, add the landing column
and rule the envelope L."): every NORMATIVE-NOW and ROADMAP row now carries
an **"Implementation lands in"** column with the accepted landing mapping.
Landing principle: **`DEL-07-09` never dispatches implementation; its
coverage ledger routes each vocabulary item to the owning deliverable(s)**
named in that column. For Tier-1 (already-implemented) items the column
records the deliverable that owns the existing surface. No item content was
otherwise changed by this modification.

All evidence anchors below were spot-verified against the working tree at
`main@89758a32634ee6cedbd1dbadf35e3728fb48d2eb`. Paths are relative to
`projects/chirality-piping/`.

## 1. Normative constraints

1. **Taxonomy binding.** The vocabulary binds to the **implemented**
   operation taxonomy — the closed change-kind set accepted by
   `core/model_operations/operation_applier/src/lib.rs` `check_kinds`
   (lines 1606–1629; 19 executable change kinds plus
   `insert_component_symbol`, which is recognized but always blocked at
   line 1678) — **not** to the broader schema `OperationKind` enum
   (`schemas/model_operation.schema.json` `$defs.OperationKind`, line 407):
   the enum tokens `move`, `reconnect`, `constraint`, and
   `design_knowledge` have no resolver, and the change-kind tokens
   `move_geometry` and `reconnect` (schema lines 338–339) are unresolved.
   Coverage claims against the schema enum alone are non-conforming.
2. **Single mutation route.** Every vocabulary item, human- or
   agent-invoked, routes through the one PKG-16 structured-operation layer
   (SOW-069; `apps/desktop/src/services/operationService.ts` lines 9–12:
   one engine, `core/model_operations/operation_applier`, for all edits).
   No vocabulary item may introduce a second mutation route.
3. **Sequencing rule.** NORMATIVE-NOW wiring of **existing** backend
   capability (Tier 1 exposure gaps and Tier 2 authoring vocabulary) comes
   first; **net-new** backend capability (Tier 3 items) is implemented
   separately and afterwards. (Owner ruling block 2.)
4. **Hanger-selection framing.** Spring-hanger selection operates only over
   **user-imported** hanger libraries, consistent with DEC-049
   (`SOFTWARE_DECOMP.md` line 635: code-neutral, no catalog sizing, no
   bundled protected values) and the user-supplied library-import boundary
   (`core/library_import/library_import_document/src/lib.rs` lines 50–53:
   `LibraryKind` = Material | Section | Component, provenance-gated; a
   hanger library kind is a governed extension of that user-supplied
   model). No hanger catalog is bundled.
5. **Units framing.** Project-wide unit switching is a **display-system
   toggle** per DEC-018 (`SOFTWARE_DECOMP.md` line 604: SI-canonical
   internal set with dual display catalog): entered-units-preserved storage
   is unchanged, and no stored value is silently mutated by a unit-system
   switch.

## 2. Implemented operation taxonomy (binding target)

`operation_applier/src/lib.rs` `check_kinds` (lines 1606–1629) accepts
exactly: `set_field`, `update_load`, `update_support`, `create_node`,
`delete_node`, `connect_pipe_run`, `delete_pipe_run`, `create_section`,
`create_material`, `create_support`, `delete_support`, `create_load_case`,
`delete_load_case`, `create_primitive_load`, `delete_primitive_load`,
`create_combination`, `delete_combination`, `create_combination_term`,
`delete_combination_term`; plus `insert_component_symbol`, recognized but
always blocked (line 1678, completion-plan A3). Deferred inspector fields:
`Component.kind` and `Combination.terms` direct edit (lines 255–256). No
delete exists for Material/Section/Component; no move/copy/reconnect
resolver exists.

## 3. Class NORMATIVE-NOW

Tier 1 + Tier 2 + the accepted Tier-3 items. Every item carries its tier
provenance, a verified backend-evidence anchor, and its binding to the
implemented taxonomy.

### 3.1 Tier 1 — end-to-end today (exposure and polish gaps only)

| # | Item | Tier | Backend evidence anchor (verified) | Implemented-taxonomy binding | Implementation lands in |
|---|---|---|---|---|---|
| 1 | Pipe run definition (routing, continuation) | 1 | `PreviewPipe`, `core/product_physics/src/lib.rs:140` | `create_node`, `delete_node`, `connect_pipe_run`, `delete_pipe_run`, `set_field` | DEL-07-01 (viewport tools); DEL-16-01 (operation layer) — existing surfaces |
| 2 | Anchors and DOF-set restraints | 1 | `SupportFamily`, `core/solver/linear_supports/src/lib.rs:18-25` | `create_support`, `update_support`, `delete_support` | DEL-07-02 (inspector forms); DEL-04-03 (support models); DEL-16-01 (operation layer) — existing surfaces |
| 3 | Materials with temperature-dependent E/G/alpha (create + edit; no delete) | 1 | `PreviewModel.materials`, `core/product_physics/src/lib.rs:94-110` | `create_material`, `set_field` (no `delete` kind exists) | DEL-07-03 (material editor surface); DEL-16-01 (operation layer) — existing surfaces |
| 4 | Temperature / pressure load cases | 1 | `PreviewModel.load_cases`, `core/product_physics/src/lib.rs:94-110` | `create_load_case`, `delete_load_case`, `set_field` | DEL-05-01 (load case engine); DEL-07-03 (load-case editor surface, R-005 lineage) — existing surfaces |
| 5 | Force / moment / displacement loads | 1 | `core/loads/primitive_loads` crate; `PreviewModel` load inputs | `create_primitive_load`, `delete_primitive_load`, `update_load` | DEL-05-01 (engine); DEL-07-02 (inspector forms) — existing surfaces |
| 6 | Wind equivalent-static loading | 1 | `WindExposedSpanInput` / `exposed_spans`, `core/product_physics/src/lib.rs:463` | `create_primitive_load`, `update_load` (known gap: `exposed_spans` editing not exposed) | DEL-05-01 (engine); DEL-07-02 (inspector forms — `exposed_spans` edit gap) |
| 7 | Seismic static-g loading | 1 | `SeismicGenerationInput`, `core/product_physics/src/lib.rs:430`; `generate_seismic_equivalent_static_loads` (lib.rs:34) | `create_primitive_load`, `update_load` | DEL-05-01 (engine) — existing surface |
| 8 | Load-case combinations (3 algebra bases incl. range envelopes) | 1 | closed basis set `mechanics`, `result_state_subtraction`, `range_envelope`, `operation_applier/src/lib.rs:3786` | `create_combination`, `delete_combination`, `create_combination_term`, `delete_combination_term` | DEL-05-02 (algebra engine); DEL-07-03 (load-case editor surface, R-005 lineage) — existing surfaces |
| 9 | Typed selection identity | 1 | `FieldKind::EntityRef`, `operation_applier/src/lib.rs:190` | targeting contract for `set_field` / `update_*` operations | DEL-16-01 (EntityRef contract); DEL-07-01 (selection surface) — existing surfaces |
| 10 | Session-scoped undo/redo | 1 | `undoStack` / `redoStack`, `apps/desktop/src/App.tsx:359-360` (checkpoints to ~700) | checkpoint inversion over all applied change kinds | DEL-00-05 (GUI state/undo-redo architecture); DEL-07-01/DEL-07-02 (session surfaces) — existing |
| 11 | Agent-proposal route (full validate/preview/apply) | 1 | `OperationAuthorType` incl. `"agent"`, `schemas/model_operation.schema.json:306-310` | all implemented change kinds via the same applier route | DEL-16-01/DEL-16-02/DEL-16-03 (operation framework) — existing surfaces |

### 3.2 Tier 2 — backend-complete, authoring vocabulary missing

| # | Item | Tier | Backend evidence anchor (verified) | Implemented-taxonomy binding | Implementation lands in |
|---|---|---|---|---|---|
| 12 | Spring hangers (variable / constant-effort; installed/cold/hot loads, travel, limits) | 2 | `SpringHangerInput`, `core/product_physics/src/lib.rs:335` (model + solver complete; inspector shows bare restraint sets) | wire authoring onto `create_support` / `update_support` / `set_field`; no new kind required | DEL-16-01 (`hanger.*` field paths); DEL-07-02 as inspector forms unless a future design act creates a dedicated panel slice (R-006 conditional — a dedicated slice would be a new DEL-07-10 via its own decomposition act) |
| 13 | Nonlinear supports (one-way, lift-off, gap, friction incl. derived normal reaction) | 2 | `NonlinearSupportInput`, `core/product_physics/src/lib.rs:363` (solve-complete) | wire authoring onto `create_support` / `update_support` / `set_field` | DEL-16-01 (`nonlinear.*` field paths); DEL-07-02 as inspector forms unless a future design act creates a dedicated panel slice (R-006 conditional as item 12) |
| 14 | Expansion joints (4-axis stiffness, pressure-thrust refs) — editable, not creatable | 2 | expansion-joint fields, `core/product_physics/src/lib.rs:260-272` | `set_field` today; creation blocked behind `insert_component_symbol` (applier lib.rs:1678) | DEL-16-01 (operation wiring); DEL-07-02 (EJ creation inspector forms) |
| 15 | Component creation — bends, tees, reducers, valves, flanges (currently field-by-field edit only) | 2 | branch/component fields, `core/product_physics/src/lib.rs:222-238`; `insert_component_symbol` always blocked, `operation_applier/src/lib.rs:1678` | unblock/complete `insert_component_symbol` (recognized kind); `set_field` for edits | DEL-16-01 (`create_component` per completion plan A3); DEL-07-01 (viewport component insertion) |
| 16 | Named support families LineStop / VerticalSupport surfaced (not collapsed to Guide) | 2 | `SupportFamily` (`linear_supports/src/lib.rs:18-25`) vs preview collapse of partial DOF sets to `Guide` (`product_physics/src/lib.rs:3160-3174`) | `create_support` / `update_support` with family-preserving preview mapping | DEL-04-03 (family emission fix) |
| 17 | Sections: create exists; delete and by-ref assignment missing | 2 | `create_section` in `check_kinds` (`operation_applier/src/lib.rs:1606-1629`) | `create_section`, `set_field`; deletion requires a taxonomy extension | DEL-16-01 (deletion / by-ref operation wiring); DEL-07-03 (editor surface) |
| 18 | Material / section / component deletion | 2 | absent from the `check_kinds` closed set (`operation_applier/src/lib.rs:1606-1629`) | requires new `delete_*` change kinds in the implemented taxonomy | DEL-16-01 (deletion change kinds); DEL-07-02/DEL-07-03 (removal affordances) |

### 3.3 Tier 3 — accepted net-new items (ratified NORMATIVE-NOW; sequenced after existing-capability wiring)

| # | Item | Tier | Backend evidence anchor (verified) | Implemented-taxonomy binding | Implementation lands in |
|---|---|---|---|---|---|
| 19 | Element insert / split (break pipe, insert-in-run) | 3 (accepted) | no resolver exists; change-kind enum is closed (`operation_applier/src/lib.rs:1606-1629`) | new change kind(s) added to the implemented taxonomy; not satisfied by the schema `OperationKind` enum | DEL-02-01 + DEL-16-01 (semantics + resolvers); DEL-07-01 (viewport tools) |
| 20 | Copy / rotate / mirror of runs | 3 (accepted) | schema tokens exist with no resolver: `move_geometry` / `reconnect`, `schemas/model_operation.schema.json:338-339` | new resolver-backed change kinds; token existence alone is non-conforming | DEL-02-01 + DEL-16-01 (semantics + resolvers); DEL-07-01 (viewport geometry-transform tools) |
| 21 | Nozzle / equipment boundary conditions | 3 (accepted) | exports name it unsupported: `free_end_and_equipment_nozzle_semantics`, `apps/desktop/src/features/pcf-export/PcfExportPanel.tsx:391` | new boundary-condition authoring bound to `create_support`-class or new kinds | DEL-02-03 (analysis boundary model) + DEL-04-03 (linear support/restraint models); GUI in DEL-07-01/DEL-07-02 |
| 22 | Automatic self-weight case generation | 3 (accepted) | mass-per-length already computed: `compute_pipe_mass_per_length`, `core/product_physics/src/lib.rs:4730` | new generation path emitting standard `create_load_case` / `create_primitive_load` operations | DEL-05-01 (primitive load case engine); mass properties from DEL-03-08 |
| 23 | Spring-hanger selection from user-imported hanger libraries | 3 (accepted) | `LibraryKind` (user-supplied, provenance-gated), `core/library_import/library_import_document/src/lib.rs:50-53`; `SpringHangerInput`, `product_physics/src/lib.rs:335` | selection resolves to `create_support` / `update_support` payloads; DEC-049 framing (Section 1.4) is mandatory | DEL-03-02 (component library schema) + DEL-03-07 (provenance checker); library editor GUI in DEL-07-03 |
| 24 | Project-wide unit switching as a display-system toggle | 3 (accepted) | `core/units` (DEC-018, `SOFTWARE_DECOMP.md:604`); per-field entered units + dual display exist | display-layer only; no model mutation, hence no change kind; storage stays entered-units-preserved | DEL-02-02 (unit system contract); surface in DEL-07-02 |

## 4. Class ROADMAP (deferred)

| # | Item | Tier provenance | Evidence anchor | Deferral basis | Implementation lands in |
|---|---|---|---|---|---|
| R1 | Node renumbering | 3 (deferred) | no implementation or change kind exists (`check_kinds` closed set, `operation_applier/src/lib.rs:1606-1629`) | deferred in the presented Tier-3 triage the owner accepted | Unassigned — mapped by the promoting owner act |
| R2 | Snubbers | 3 (deferred) | zero repository hits across `core/`, `apps/`, `schemas/` (verified 2026-08-20) | deferred in the presented Tier-3 triage the owner accepted | Unassigned — mapped by the promoting owner act |
| R3 | Cold spring / cut-short | 3 (deferred) | no repository implementation | owner ruling block 2: "rarely used and I'm comfortable adding that later rather than now" | Unassigned — mapped by the promoting owner act |

ROADMAP items are not coverage obligations of `DEL-07-09` at acceptance.
Promoting any ROADMAP item into NORMATIVE-NOW is a future owner act
(scope-change or owner-ruled deliverable amendment), not an implementation
choice.

## 5. Consumption

On SCA-009 Gate-5 acceptance, this annex's content becomes the coverage
contract owned by `DEL-07-09` (single palette-surface owner per D4), with
the palette surface organizing the NORMATIVE-NOW command set over the
existing DEL-07-01/DEL-07-02 surfaces and routing exclusively through the
PKG-16 operation layer. Until then it is candidate text with no effect.
