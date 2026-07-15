<!-- chirality-architecture-basis/v1 -->
# DEL-00-05 — GUI state and interaction architecture — Architecture Basis

Classification: `ARCHITECTURE_BASIS_REFERENCE`
Consolidates: `Datasheet.md`, `Specification.md`, `Procedure.md`, `Guidance.md` (four-document kit, consolidated 2026-07-15 per piping decision D-43; source bytes hash-bound in the package `CONSOLIDATION_MANIFEST.md` and preserved in git history)
Authority basis: `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.9, architecture-basis row AB-00-05
Dependency direction: one-way — project packages depend on this member; this member depends on no package deliverable (HUMAN-STEER-PKG00-EXCLUSION-001)

## Purpose

Defines the cross-cutting GUI state, editing, selection, undo/redo, viewport-integration, and interaction architecture that GUI-adjacent packages consume as architecture-basis context. It is reference architecture, not product implementation: screens, components, styling, viewport rendering, and user-interface code are owned by the implementing packages. The GUI state boundary now has command-backed selection and operation flows in the implemented working-tree slice (D-41 R5 T7 PDU-054/PDU-055 current declarations); this member supplies the constraints those flows and future GUI work must preserve.

Identity: Deliverable DEL-00-05, PKG-00 Software Architecture Runway, type UX_UI_SLICE, scope item SOW-060, objective OBJ-013.

## Normative requirements

| ID | Current normative text | Basis |
|---|---|---|
| REQ-05-01 | Separate durable project state from transient session, viewport, selection, and job-progress state. | Kit Specification; AB-00-05 row, SOFTWARE_DECOMP rev 0.9 |
| REQ-05-02 | Route model-changing GUI actions through application-service commands rather than direct domain mutation. | Kit Specification; AB-00-05 row |
| REQ-05-03 | Undo/redo records are scoped to reversible model edits and preserve diagnostics when an edit changes solve readiness. | Kit Specification; AB-00-05 row |
| REQ-05-04 | Define framework-agnostic selection and interaction architecture. Ownership boundary (accepted, current): `DEL-00-05` owns the cross-cutting GUI state and interaction architecture; `DEL-07-02` is the accepted delegated owner of model-tree and property-inspector behavior and consumes this architecture basis. Behavior evidence belongs with `DEL-07-02`; the delegation moves no implementation into this member and expands neither scope. | D-41 `DEC-074` option O2 (ruling 2026-07-12, `D-41_R4_RULING_2026-07-12.md`; PDU-009 repair folded into the kit 2026-07-12) |
| REQ-05-05 | GUI warnings preserve the solve-blocking, rule-check-blocking, provenance, assumption, nonlinear, and IP-boundary classes (see DEL-00-06 for the class contract and its accepted per-boundary mapping refinement). | Kit Specification; DEL-00-06 contract |

Cross-cutting invariants carried unchanged from the kit: `OPS-K-IP-1` (no protected standards text, tables, formulas, allowables, or proprietary data in public artifacts), `OPS-K-DATA-2` (missing solve/rule-check-required values remain explicit findings, never silent defaults), `OPS-K-AUTH-1` (no certification/sealing/approval/code-compliance claims), `OPS-K-MECH-1` (global analysis is a 3D centerline/frame model; local FEA is a handoff path), `OPS-K-AGENT-1` (unknown facts become `TBD`), `OPS-K-AGENT-3` (Type 2 execution stays within sealed scope).

Boundary rules retained as current: GUI state cannot bypass application services, diagnostics, unit validation, or data-boundary checks; architecture language stays concrete enough for implementation but avoids premature stack decisions beyond accepted rulings; the distinction between mechanical calculation, user rule checking, and professional approval is preserved. Downstream consumers use this member as AB-00-05 architecture-basis context through sealed briefs and governed dispatch surfaces; that consumption does not make PKG-00 ISSUED, and it does not authorize PKG-01 through PKG-17 work from within this member.

## Resolved decisions (former TBD and human-ruling queue)

Register: `execution/_Coordination/_DECISIONS/_REGISTER.md`; codification in `execution/_Decomposition/SOFTWARE_DECOMP.md` §8.4 (~lines 585–640).

| Former open item | Resolution | Record |
|---|---|---|
| GUI framework / viewport engine (setup-era TBD) | Rust core/application services; Tauri 2 desktop shell; TypeScript/React/Vite GUI; Three.js viewport. Accepted 2026-04-30 and implemented (`apps/desktop/package.json`). | `DEC-009` (SOFTWARE_DECOMP rev 0.9 §8.4; SCA-001 Gate 3) |
| Model-tree / property-inspector ownership (D-41 OD-002/CU-002) | Option O2: DEL-00-05 retains cross-cutting GUI state/interaction architecture; DEL-07-02 is delegated owner of model-tree and property-inspector behavior. Folded into REQ-05-04 above. | D-41 `DEC-074` (R4 ruling 2026-07-12); PDU-009 |
| Setup-era "no implementation exists" framing | Superseded as current declaration by the implemented working-tree slice and its evidence; earlier wording is historical setup context only. | D-41 R5 T7 PDU-054/PDU-055 (claims `DEL-00-05-DECL-002`, `DEL-00-05-DECL-003`) |

## Realized artifacts

| Anticipated by the kit | Realized state | Owner |
|---|---|---|
| `docs/architecture/gui_state_model.md` | Never created under that name. The architecture is realized in practice by the implemented GUI slice: `apps/desktop/src/features/` (feature modules including model-tree, model-workspace, diagnostics, design-workspace) and `core/gui/`. | Implementation surfaces owned by their packages; this member remains architecture basis |
| Interaction architecture notes | Carried by this document and the constraints above | DEL-00-05 |
| Model-tree / property-inspector behavior | Implemented under the DEC-074 O2 delegation | DEL-07-02 |

## Open holds and routed questions

- OPEN: GUI component/state-management-library choice. `DEC-009` fixed the runtime/UI stack but left "exact dependency versions, component/state-management libraries, platform release matrix" implementation-level TBD (`execution/_Decomposition/SOFTWARE_DECOMP.md` §8.2). Routed to human architecture ruling; not silently selectable.
- OPEN: undo/redo storage mechanism and accessibility target details (kit TBDs never ruled; remain visible pending human authority).
- Deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining` (currently empty). Nothing in this document implies lifecycle, review, validation, release, professional-reliance, or code-compliance closure.

## Currency and provenance

Consolidated 2026-07-15 per piping decision D-43 from the four-document kit (`Datasheet.md`, `Specification.md`, `Procedure.md`, `Guidance.md`) as reconciled by D-41 R5 T7 (PDU-054/PDU-055 currentness declarations, 2026-07-12). Prior wording is preserved in git history; `MEMORY.md` is retained unchanged as the dated decision/evidence trail. Current upstream authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.9 with approved `execution/_DAG/DAG-007/` graph context.
