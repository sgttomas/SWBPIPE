# Condition use and the difference template in the load manager (S3b)

Model document 0.9.0, slice S3b (DEL-02-01 Work `model-document-0.9.0.md` §1.2, §1.3,
§2.2, §2.3, §4.2, §11), for DEL-07-03:5. It serves RT-G5, RT-G2 and RT-G3. The set, pipe
and run-sheet surfaces are in DEL-07-02 Work `condition-set-authoring.md`. This note
describes the implementation; it records no backcheck or acceptance.

## 1. The Conditions control, per case

`features/condition-sets/ConditionUseControl.tsx` is mounted in the load manager under
the load/reference-state inputs, for the selected case.

- **Pressure-free documents.**
  - The inputs are a temperature key, chosen from the keys present in the model's sets
    ("not used" when none is chosen), and two marks: `added_weight: true` and
    `wind_exposure: true`.
  - An unticked mark is absent, never `false`, and nothing is defaulted.
  - The whole `ConditionUse` object is the `after` of `set_load_case_condition_use`. "Clear"
    is `clear_load_case_condition_use` with `after` `not_present`.
- **Uses refused on pressure-free documents.** Pressure and contents are listed as
  refused, with their codes, and offer no input:
  - `LOAD_CASE_CONDITION_PRESSURE_WITHOUT_PRESSURE_CONTRACT`;
  - `LOAD_CASE_CONDITION_CONTENTS_UNSUPPORTED`.
- **Members the control does not edit.** A use edit starts from the stored use and
  changes only the members the person changed. A stored member the control does not edit,
  or a stored value the engine refuses (for example `added_weight: false`), is shown as
  "Retained as authored" and written back unchanged, so the engine judges it by name
  (`LOAD_CASE_CONDITION_USE_EMPTY`).
- **Documents with `pressure_contract`.** The control is shown refused,
  `LOAD_CASE_CONDITION_USE_EXACT_UNSUPPORTED`, and offers no input. A stored use is shown
  as it is.
- **The engine's outcome in place.** Each edit is checked validate-only by the operation
  engine, then queued for Review/Apply (`engineCheckedQueue.tsx`). The findings are
  listed by code, message and references:
  - blocking: refused, and nothing is queued;
  - warning: the solve's staged findings;
  - info: permitted carried refusals.
- **The engine's resolution.** "Engine resolution (stored model)" calls
  `services/operatingConditionPreviewService.ts`. That service wraps the wasm export
  `preview_operating_conditions_json` (the wasm engine the desktop already loads, beside
  `operationService`; no native command). It lists the case's generated records: pipe,
  category, magnitude, direction and dimension. These include the thermal ΔT, the added
  weight and the wind-exposed pipes, together with the engine's findings for the case.
  The desktop computes no ΔT. The panel states that resolution is not solvability.

## 2. Kind suggestion

`operating` is among the kind suggestions: the metadata value options and the new case's
kind list. It is vocabulary only. `kind` stays a free string, and nothing composes W+P+T
or reads any other meaning into it (§2.3).

## 3. Operating minus sustained difference (template)

- **Where.** It is offered in the Create combination editor only when the basis is
  `result_state_subtraction` (`load-cases/operatingDifferenceTemplate.ts`).
- **Choosing it.** Choosing it clears the minuend and subtrahend, which the editor
  otherwise fills. The person then chooses the operating case (minuend) and the sustained
  case (subtrahend).
- **What it writes.** `create_combination` writes exactly those two and
  `difference_basis: "operating_minus_sustained"`. It never writes
  `subtrahend_support_state`, and never offers `minuend_final`.
- **Naming.** It is never called "expansion"; that naming and any code range belong to
  the user's rule pack.
- **The solve's gates.** A one-line note says the solve checks containment and the
  elastic basis (G-1, G-2) and may withhold the combination with a warning. The desktop
  does not reimplement them.
- **Documents with a pressure contract.** The template is shown disabled and refused,
  `LOAD_COMBINATION_DIFFERENCE_BASIS_EXACT_UNSUPPORTED`.
- **Stored combinations.** A stored combination's `difference_basis` is shown on its row.
  A stored `subtrahend_support_state` is shown as "retained as authored; not written by
  this editor".
- **Create only.** The combination update route (`update_load` on `Combination`) edits
  only label, basis and provenance, so the template does not edit an existing
  combination. Changing one is a delete and create.

## 4. Out of scope

These have no UI here:

- the `absolute_sum` and `srss` bases;
- combination operands;
- `test_condition`;
- support `direction`;
- contents and hydrotest;
- EX resolution.

## 5. Evidence

- `apps/desktop/src/features/condition-sets/conditionSets.engine.test.tsx`:
  - the end-to-end path, including a case's use with staged warnings shown and then
    applied;
  - the exact-route refusal with no inputs;
  - an unticked mark written as absence;
  - the engine's per-pipe 130 K records shown.
- `conditionSets.session.test.tsx`: a case's use through the session and its Undo; a
  reopened model's use edited, applied, saved again and reopened with the applied use.
- `conditionSets.engine.test.tsx`: a stored refused use member kept through an edit and
  named.
- `conditionSetSurfaces.engine.test.tsx`:
  - the `operating` suggestion;
  - the template's payload, no prefill and the engine's stored record;
  - the exact-route refusal of the template;
  - the retained `subtrahend_support_state`.
- `LoadCaseManagerPanel.test.tsx`: the kind options now include `operating`.

## 6. Open

- Native verification and the timed walkthrough.
- Editing an existing combination's `difference_basis` would need an update operation
  that the applier does not offer today.
