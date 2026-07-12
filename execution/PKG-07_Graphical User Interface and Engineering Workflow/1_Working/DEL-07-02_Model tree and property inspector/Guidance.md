# Guidance: DEL-07-02 Model tree and property inspector

## Purpose

This deliverable owns the bounded model-tree/property-inspector behavior implementation within the GUI state/interaction architecture owned by `DEL-00-05`. The implemented and residual work helps users see model structure, selected-entity data, missing inputs, assumptions, provenance, and private/rule-pack status early in the workflow.

## Accepted Ownership Boundary

- `DEL-00-05` retains GUI state/interaction architecture ownership.
- `DEL-07-02` owns model-tree/property-inspector behavior implementation within that architecture.
- `DEC-074` option O2 resolves `PDU-009`; it does not transfer architecture or broaden functionality.
- `DEL-07-03` and `DEL-07-04` retain their specialized-editor and missing-data warning/blocking scopes respectively.

## Principles

- Treat the model tree as navigation and visibility, not a second durable model store.
- Route edits through application-service commands and keep selection, expansion, filtering, and panel focus as transient GUI state.
- Show missing solve-required and rule-check-required values as explicit findings instead of silently filling defaults.
- Preserve unit and provenance context for editable engineering fields.
- Keep rule-pack checks, private-library values, and professional acceptance distinct from the open mechanics model.
- Keep implementation choices that remain unresolved, including component/state libraries and exact command/query names, as `TBD`.

## Considerations

The bounded tree and inspector implementation consumes upstream domain schema, persistence, command/query/job envelope, diagnostics, rule-pack, material/component/library, unit, and GUI state contracts. Current evidence settles bounded behavior only; unresolved contracts and broader UX remain governed residuals rather than authority for this deliverable to redefine `DEL-00-05` architecture.

The inspector should support workflows that make engineering gaps visible before solve execution or rule checking. It should not turn missing code-specific, material, component, SIF/flexibility, allowable, or rule-pack values into public defaults. Private project and library data should remain local/user-controlled unless a later export or contribution path is explicitly authorized with documented rights.

## Trade-offs

| Topic | Guidance |
|---|---|
| Tree richness vs scope control | Include the navigation and inspection concepts needed by DEL-07-02, but leave specialized editors to DEL-07-03 and missing-data blocking UX to DEL-07-04 unless an accepted brief expands scope. |
| Convenience vs data boundary | Do not prefill protected/code-specific values for convenience; use visible `TBD`, diagnostics, or incomplete-state UI. |
| Selection state vs durable data | Keep current selection, tree expansion, filters, and panel focus out of durable project state unless a persistence contract later authorizes a view-state record. |
| Editable inspector vs command boundary | Inspector edits should be command-backed and validation-aware; direct mutation of durable model objects from UI state should be avoided. |
| Rule status vs professional judgment | Show user-rule/check readiness or diagnostics only as software findings; do not present them as professional code compliance. |

## Boundary Rationale

The inspector is a convenience surface over governed model and service contracts. Command-backed edits keep validation, unit checks, diagnostics, provenance handling, undo/redo scope, and result-envelope behavior in the application-service boundary instead of allowing UI state to become an alternate authority.

The split with adjacent GUI deliverables remains explicit: DEL-07-02 owns navigation and selected-entity inspection setup, DEL-07-03 owns specialized material/component/rule-pack editor implementation, and DEL-07-04 owns missing-data warning and blocking UX unless a later sealed brief or human ruling changes that boundary.

## Examples

Current focused tests and run evidence use governed fixtures for bounded behavior. Any future screenshots, examples, or additional fixtures must use synthetic, public-domain, or otherwise cleared model data and must not reproduce protected standards content, proprietary project data, or protected code examples.

## Conflict Table (for human ruling)

| Conflict ID | Issue | Contenders | Human ruling |
|---|---|---|---|
| D41-PDU-009 | Selection/inspector architecture versus behavior ownership. | `DEL-00-05`; `DEL-07-02` | `DEC-074` O2: `DEL-00-05` owns GUI state/interaction architecture; `DEL-07-02` owns model-tree/property-inspector behavior implementation within it. |
