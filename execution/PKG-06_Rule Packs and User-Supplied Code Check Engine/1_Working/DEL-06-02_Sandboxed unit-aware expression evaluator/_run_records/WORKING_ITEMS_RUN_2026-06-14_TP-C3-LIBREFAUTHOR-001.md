---
run-id: WORKING_ITEMS_RUN_2026-06-14_TP-C3-LIBREFAUTHOR-001
timestamp: 2026-06-14T01:00:00-0600
completed: 2026-06-14T01:10:00-0600
run-status: SUCCESS
control-surface: INLINE
scope-path: /Users/ryan/ai-env/projects/chirality/projects/chirality-piping
write-authorization: COORDINATION_LOOP_PREAPPROVED_TRANCHE
---

# WORKING_ITEMS_RUN TP-C3-LIBREFAUTHOR-001 — author `library_value_ref` in the rule-pack declarations form-builder (C3 residual, authoring slice)

## Tranche and authority basis

- Tranche: the completion-plan **C3** residual named first in its row and in the
  prior tranche's hand-offs — "C2 authoring of `library_value_ref` in the
  declarations form-builder." It is the **authoring half** of the rule-pack ↔
  private-library round-trip whose **resolution half** landed in
  `TP-C3C4-LIBREF-001` (TP-MAC-157): the backend already resolves a
  `private_library_value` required input from the local private-library store at
  run time, but the reference could only be set by hand-editing raw document
  JSON. This slice replaces that hand-editing with structured form controls.
- Selection: earliest unblocked item on the Phase C / R3 dependency spine
  (`DEC-035` target stage). Unblocked — the optional `library_value_ref` schema
  member and its run-time resolution already landed (`TP-C3C4-LIBREF-001`); the
  Value-based `validate_rule_pack_document` tolerates the member (read by JSON
  pointer, no `deny_unknown_fields`); **D-02b** (AWAITING_RULING) gates only
  writable expression *text* syntax, not structured form authoring — this is
  form-builder work exactly like the rest of `DeclarationsEditor`.
- Scope: **frontend-only**, one component + its two test surfaces. No schema,
  Rust, or Python change. Write scope: `DeclarationsEditor.tsx`,
  `DeclarationsEditor.test.tsx`, `e2e/r2-smoke.spec.ts`, `apps/desktop/SMOKE.md`,
  this run record, and the completion-plan C3 row.

## Design decisions

- **Seed a complete reference on entry.** `LibraryValueRef` requires all four
  members (`library_kind` ∈ {material, section, component}; `library_id`,
  `record_id`, `slot_id`). When the user sets `source_kind` to
  `private_library_value` and no reference exists, the editor seeds a complete
  reference (`library_kind` → the first/only-resolved kind `material`; the three
  ids → the visible uppercase `"TBD"` placeholder the `Id` pattern accepts), so
  the document is always schema-valid and an unfilled reference resolves to
  nothing and the input **blocks** — never a partial shape, never a silent pass
  (CONTRACT no-silent-defaults). `library_kind` has no `"TBD"` member in the
  schema, so it necessarily defaults to a concrete kind; this is visible in the
  select, not hidden.
- **First-edit completion guard.** `updateInputLibraryRef` merges into the
  existing reference *or a fresh default* when none exists, so even an opened
  pack with `source_kind: private_library_value` but no reference (authored
  before this slice) yields a complete reference on the first field edit.
- **Never silently hidden; explicitly removable.** The sub-form shows when
  `source_kind === private_library_value` **or** a reference is present, so a
  reference left after the source_kind is changed away stays visible. A
  "Remove library reference" control drops the member (the only member-removing
  edit the editor exposes); every other member of the input round-trips
  verbatim (lossless).
- **IP boundary (decisive).** The form authors the *reference only*; the private
  value is read at run time from the local store and never embedded in the
  rule-pack document (PRD §13/§17.3, IP_AND_DATA_BOUNDARY). An honest note on
  the sub-form states this.

## Changes

### `apps/desktop/src/features/rule-packs/DeclarationsEditor.tsx`
- New exported `LIBRARY_KINDS = ["material","section","component"]` (verbatim
  schema vocabulary) and `defaultLibraryValueRef()` helper.
- `changeInputSourceKind` (seeds the reference on switch to
  `private_library_value`), `updateInputLibraryRef` (lossless merge over the
  existing ref or a fresh default), `removeInputLibraryRef` (drops the member).
- Required-input row: a `library_value_ref` sub-form (`library_kind` select +
  `library_id`/`record_id`/`slot_id` text fields + IP-boundary note + remove
  control) gated on `source_kind === private_library_value || ref present`.
- Component doc-comment updated to record the slice and the boundary posture.

### `apps/desktop/src/features/rule-packs/DeclarationsEditor.test.tsx` (+5)
- Vocab assertion `LIBRARY_KINDS` set-equality vs the schema's three kinds.
- `defaultLibraryValueRef()` exact-shape pin (concrete kind + `"TBD"` ids).
- Component: seed-on-switch-to-`private_library_value` (complete ref + sub-form +
  note); lossless field edits keeping the rest of the input verbatim;
  first-edit completion when an opened input lacks a ref; stale reference
  visible + removable, removal preserving siblings and hiding the sub-form.

### `apps/desktop/e2e/r2-smoke.spec.ts`
- Extended the existing rule-pack manager draft flow: set the appended input's
  `source_kind` to `private_library_value`, assert the sub-form + boundary note,
  fill the four controls, and assert a complete `library_value_ref` in the
  canonical draft JSON. Real-browser default UI evidence per the H4 amendment.

## Evidence

- `npm test --workspace apps/desktop` (Vitest): **345 pass** (+5; was 340).
- `npm run build --workspace apps/desktop`: clean (pre-existing chunk-size
  advisory only).
- `npx playwright test`: **10/10** (two viewports; rule-pack manager test
  carries the new authoring assertions).
- Five-surface DEC-025 sweep: see the committed sweep summary.
- No Rust/Python/schema surfaces touched (frontend-only slice); the optional
  member and its resolution were proven end-to-end in `TP-C3C4-LIBREF-001`.

## Residuals and hand-offs

- **Richer C4 run-panel picker** — the run panel still surfaces the reference
  read-only; a library/record/slot picker there is a follow-up.
- **Section/component slot resolution** — the runner resolves material allowable
  slots only (carried from `TP-C3C4-LIBREF-001`); this form authors all three
  kinds, but section/component resolution is still a backend follow-up.
- **Schema ratification** — `library_value_ref` remains a PROPOSAL additive
  member awaiting a human `DEC` ratification (companion to `DEC-031`); this slice
  does not change its status, only adds GUI authoring over it.

## Boundary compliance

Local-only (no network/daemon/telemetry); the reference is authored into the
local draft and the private value is never embedded or committed.
Status-vocabulary-only; no compliance/certification/sealing/authentication/
approval/code-compliance or professional-acceptance claim. Deliverables stay
`CHECKING`. Git/test evidence is source-control hygiene only.

## Open decisions awaiting human ruling

- **Schema ratification** of the additive `library_value_ref` member (PROPOSAL,
  companion to `DEC-031`).
- **D-02b** — writable rule-expression text syntax (`AWAITING_RULING`);
  unrelated to this slice (structured form authoring only).
