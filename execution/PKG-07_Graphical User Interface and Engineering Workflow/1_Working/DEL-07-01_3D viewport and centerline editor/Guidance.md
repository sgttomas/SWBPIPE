# Guidance: DEL-07-01 3D viewport and centerline editor

## Purpose

This setup deliverable gives future GUI implementation work a bounded viewport/editor contract. The intent is to make centerline model creation visible and reviewable while preserving the project's code-neutral, unit-aware, provenance-aware, and human-review boundaries.

The viewport is a user-facing editor surface, not an authority for engineering values. It can show missing data, provenance gaps, assumptions, and diagnostics, but it must not hide those states or convert them into silent defaults.

## Principles

| Principle | Guidance |
|---|---|
| Centerline first | Treat the default global model as a 3D line-element centerline representation. Local shell/solid FEA remains a specialized handoff path. |
| Command-mediated edits | Future edit gestures should produce application-service commands so durable model state, undo/redo, diagnostics, and persistence remain coherent. |
| Transient state separation | Camera, hover, selection, drag handles, snapping previews, and job progress are transient GUI state, not durable project data. |
| Unit-safe editing | Coordinates and editable values need unit-bearing model contracts. Avoid viewport-only numeric assumptions. |
| Visible uncertainty | Missing solve data, missing rule-check data, weak provenance, user assumptions, and nonlinear uncertainty should be visible diagnostics. |
| No protected defaults | Component symbols and edit tools must not embed protected code tables, proprietary catalog values, or copied commercial examples. |
| No professional claim | A rendered model, visual status, or rule-check color is decision support only until a competent human accepts it for project use. |

## Considerations

| Topic | Consideration |
|---|---|
| Initial slice size | `DEL-07-01` has an L context envelope. Keep the future implementation bounded to viewport/editor basics and split if it starts absorbing model tree, property inspector, results viewer, or solve-execution work. |
| Component visualization | Simple glyphs can support recognition and selection. Detailed component properties, private library data, and editor behavior belong to PKG-03 and adjacent PKG-07 deliverables. |
| State library choice | The exact state-management library remains `TBD`. Future work should preserve the durable/transient split regardless of library choice. |
| Viewport library baseline | Three.js is the accepted viewport baseline. Exact versions and wrappers remain implementation-level decisions. |
| Accessibility | The viewport should not be the only path to edit or inspect model data. Keyboard navigation, tooltips, high-contrast options, and structured panels are part of the wider GUI baseline. |
| Diagnostics | Warnings should be attached to affected model objects where possible so users can navigate from the viewport to the underlying issue. |
| Testing | Future tests should verify both visual presence and behavior: entity creation, selection, command dispatch, undo/redo eligibility, missing-data rendering, and no hidden defaults. |

## Trade-offs

| Trade-off | Setup position |
|---|---|
| Rich interactive modeling vs. broad GUI scope | Favor a narrow, testable editor slice over absorbing tree, property, solve, or results workflows. |
| Visual convenience vs. engineering warrant | Favor explicit `TBD` and diagnostics over convenience defaults for code-specific or proprietary data. |
| Direct canvas mutation vs. service commands | Favor service-command mutation so validation, diagnostics, undo/redo, persistence, and audit records remain coherent. |
| Detailed component graphics vs. protected data boundary | Favor simple symbolic visualization unless component geometry and metadata have lawful provenance. |
| Visual status colors vs. compliance language | Favor statuses that distinguish model completeness, mechanics, rule checks, and human review without implying compliance or approval. |

## Examples

Acceptable future examples for this slice are limited to invented, non-code, non-proprietary interaction examples, such as:

- creating a node with unit-aware coordinates supplied by the user;
- connecting two nodes with a pipe-run element that references `TBD` section/material data until supplied;
- rendering a bend arc as a symbolic component with user-supplied geometry placeholders;
- surfacing a diagnostic that a component's source/provenance is missing.

Do not use copied standards examples, protected tables, commercial software benchmark models, proprietary component catalogs, code-specific allowables, or real project data in public setup or future test fixtures unless explicit redistribution rights and review disposition exist.

## Conflict Table (for human ruling)

| Conflict ID | Conflict | Source A | Source B | Impacted sections | Proposed authority (PROPOSAL) | Human ruling |
|---|---|---|---|---|---|---|
| None | No source conflict identified during setup drafting. | N/A | N/A | N/A | N/A | N/A |

## Review Notes

- This setup document intentionally does not choose unresolved component/state libraries.
- This setup document intentionally does not implement product UI.
- Future work should escalate if the viewport/editor slice needs to exceed `DEL-07-01` scope or introduce protected/private data.
