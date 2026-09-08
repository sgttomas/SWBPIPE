# Native provenance and diagnostic correction — 2026-09-08

This derivative triage corrects two inferences from the first native solve attempt. It changes no source, test, governed state, or acceptance status.

## Confirmed facts

- The active node Add/Apply path preserves entered provenance. `PipeViewport.tsx:225-230,572-580` calls `buildNodeCreationSubmission`; `routeDraft.ts:91-105,426-440` serializes `draft.provenance.trim()` into the create-node intent; `App.tsx:1958-1959` connects that path to the application callbacks. Native persisted-store evidence independently confirmed that both nodes and the other authored records retained the entered value `synthetic_ui_acceptance_input`.
- `PipeViewport.tsx:1963-1980` contains a legacy `buildExplicitNodeIntent` helper with `user_entered_local_preview`, but a repository search found only its definition and zero call sites. It did not produce the native model. Therefore there is no confirmed node-provenance forwarding defect and no basis here to reopen frozen U7 R3.
- The native `mechanics_result_json` contains exactly seven fresh backend `PROVENANCE_INPUT_MISSING` blockers: two nodes, one pipe, one support, one material, one load case, and one primitive load. It contains no `BLANK_PROJECT_AUTHORING_TARGET` diagnostic.
- The separately displayed `BLANK_PROJECT_AUTHORING_TARGET` item is retained model metadata created at `projectService.ts:477-500`. The Issues UI concatenates model and result diagnostics (`DiagnosticsPanel.tsx:22-26`; `App.tsx:2950-2965`), which explains why the item appeared beside backend findings.
- A native run submits the current model without using stored diagnostic readiness (`App.tsx:629-665`; the only pre-submit model gate is model absence at 645-648). Tauri deserializes the payload and invokes product physics (`src-tauri/src/lib.rs:1449-1462`). Product physics creates a fresh diagnostics vector and validates the current model (`core/product_physics/src/lib.rs:830-859`; `validation.rs:12-25`). Its input `analysis_status` is deserialized but not consulted by production solve logic, and its Rust input model has no model-diagnostics field. Fresh blocking diagnostics yield `MODEL_INCOMPLETE` (`lib.rs:9266-9290,9578-9579`); otherwise the result is `MECHANICS_SOLVED` (`lib.rs:1151-1159`). The retained blank-project display note therefore does not block backend execution.

## Rerun boundary

Changing the synthetic native-input provenance to an explicit invented/cleared label addresses the seven observed backend provenance blockers. The rerun must still establish the resulting status and may reveal other current-model validation findings; this record does not promise a solved result or alter the native acceptance gate.

## Inspected source bindings

- `PipeViewport.tsx` — `ac3789a6b4e8c452fe398d062183b25d884f55f5763af877ac6f1b3d4cb9f2ad`
- `routeDraft.ts` — `cb258db1ab0750b891a023c0723cfbfdb55a898bb4930d4aaf4137cc6992c7a4`
- `App.tsx` — `a9217c959bfc2979e21cde5d5baf865f75258f03b902ecdd2c0fd545e5bfbde9`
- `projectService.ts` — `da38305ab02c69be95cdae4e79c9546568038716d597cc72385883938cc1363d`
- `DiagnosticsPanel.tsx` — `c752935edf4268f05adedc29a8f1047241c0cb498f8dd44144eeec657f637c1b`
- `product_physics/src/lib.rs` — `e757b8a51e2c4ae68ac4d6c37620663bf4d698ff03b8d6349b40b484bb591903`
- `product_physics/src/validation.rs` — `cc98b205467c9a965adc03179e91c43f174ba4b9bafb0f7aff47ed5ad6e936a7`
- `src-tauri/src/lib.rs` — `d9f8cfc11a5763a22da42eb2c2fbb43cb299f6205d1d481c6b3a8cb55cc9de81`
