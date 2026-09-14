# Context: DEL-07-09

**Deliverable ID:** DEL-07-09
**Name:** Interactive operation vocabulary and tool palette contract
**Package ID:** PKG-07
**Package Name:** Graphical User Interface and Engineering Workflow
**Type:** UX_UI_SLICE

## Description

Own the ratified two-class interactive operation vocabulary coverage contract and the single tool-palette surface organization over the viewport and tree/inspector surfaces, routed through the structured model operation layer.

## Anticipated Artifacts

- vocabulary coverage ledger
- palette organization contract
- palette-to-operation routing map

## Scope Coverage

- SOW-077

## Scope Detail

- SOW-077: The GUI shall provide a ratified interactive model-building operation vocabulary and a governing tool-palette contract that organizes the human command surface for building and modifying the piping model, with every palette command routed through structured model operations.

## Objective Support

- OBJ-006
- OBJ-015

## Context Envelope

- **Envelope:** L
- **Risk:** WATCH
- **Recommended Action:** Confirm scope and split if it expands.
- **Notes:** Contract/coverage slice only. Implementation lands in the deliverables named in the SCA-009 Vocabulary Annex; editor implementation does not land here.

## Package Reference

- **Package:** PKG-07 Graphical User Interface and Engineering Workflow
- **Package Scope:** Implements the interactive modeler, editors, warning UX, solve-execution UX, and results views.
- **Package Assigned Scope Items:** SOW-020, SOW-021, SOW-022, SOW-023, SOW-036, SOW-055, SOW-076, SOW-077
- **Package Exclusions:** Does not silently supply missing code data.

## Accepted Authority

- **Decomposition:** `execution/_Decomposition/SOFTWARE_DECOMP.md`
- **Accepted Revision:** 0.12
- **Scope Change:** `execution/_ScopeChange/SCA-009_2026-08-20_0000/`
- **Decision:** DEC-094
- **Vocabulary Contract:** `execution/_ScopeChange/SCA-009_2026-08-20_0000/Vocabulary_Annex.md`
- **Boundary:** DEL-07-09 owns the coverage contract and palette organization only; it never dispatches or receives the implementation routed by the annex.

## Register References

- **Deliverables Register:** `docs/_Registers/Deliverables.csv` row DEL-07-09
- **Scope Ledger:** `docs/_Registers/ScopeLedger.csv` row SOW-077
- **Context Budget QA:** `docs/_Registers/ContextBudgetQA.csv` row DEL-07-09

## PREPARATION Notes

- Structural scaffold and source-grounded metadata only.
- Lifecycle is OPEN; no production document or implementation artifact is created by PREPARATION.
