---
run-id: WORKING_ITEMS_RUN_2026-06-14_TP-C3-LIBREFPICKER-001
timestamp: 2026-06-14T13:30:00-0600
completed: 2026-06-14T14:55:00-0600
run-status: SUCCESS
control-surface: INLINE
scope-path: /Users/ryan/ai-env/projects/chirality/projects/chirality-piping
write-authorization: COORDINATION_LOOP_PREAPPROVED_TRANCHE
---

# WORKING_ITEMS_RUN TP-C3-LIBREFPICKER-001 — richer library/record/slot resolution-preview picker in the C4 run panel (C3 residual)

## Tranche and authority basis

- Tranche: the completion-plan **C3** residual **"a richer C4 run-panel
  library/record/slot picker (the panel surfaces the reference read-only)"** —
  the single consistently-named remaining Phase C residual, carried by all three
  prior library-reference slices (`TP-C3C4-LIBREF-001`,
  `TP-C3-LIBREFAUTHOR-001`, `TP-C3-LIBREFSECCOMP-001`). It is the earliest
  unblocked item on the current R3/Phase C dependency spine (C2→**C3**→C4), so it
  is pre-approved for execution under the Application Integration And Issuance
  Loop (`_COORDINATION.md` step 3.1).
- Problem: a rule-pack `private_library_value` required input authors a
  `library_value_ref` (`DeclarationsEditor`) that the desktop `run_rule_checks`
  wrapper resolves from the local private-library store at run time. The Run Rule
  Checks panel previously surfaced that authored reference **read-only** — a line
  of descriptive text — so a user could not tell from the panel whether the
  reference would actually resolve in their local store, nor discover the valid
  record/slot ids to author. The only resolution signal was post-run
  (`bound_inputs` showing `supplied`/`MISSING`).
- Blockers: none. The two decisions adjacent to this work are **ruled**: the
  additive `library_value_ref` schema member is **ratified** (`DEC-038`,
  2026-06-14) and D-02b is **ruled** (`DEC-037`, unrelated — it gates writable
  expression text syntax, not library references).

## Design decision (conservative; no human ruling required)

The picker is a **read-only-to-the-pack resolution preview**, not a run-time
override:

- It **never mutates the rule pack** and **never overrides** the authored
  reference at run time. The pack's authored `library_value_ref` remains the sole
  binding authority; the backend run still resolves from that reference +
  `projectId`. A run-time *override/selection* picker (choosing a different
  library/record/slot at run time) was deliberately **not** built — that would
  duplicate the `DeclarationsEditor` authoring surface and change the
  "reference authored in the pack" model, which is a human design ruling, not an
  agent call. Recorded here as a non-goal, not an open residual.
- It **never renders the private value** — only the structural ids
  (record/slot) and a resolution status. The private value continues to be read
  only at run time and never embedded in the pack or shown in the panel (IP
  boundary).
- It is **desktop-only**: the local private-library SQLite store is reachable
  only through the Tauri command path. In browser preview the picker reports the
  honest store-unavailable seam (the same posture as the run path and the library
  manager).

The frontend traversal that indexes records/slots **mirrors the desktop resolver
exactly** (`src-tauri` `extract_library_slot_value` / `find_library_slot_value`),
read verbatim — nothing invented:

| kind | records member / id key | slot array(s) / slot id key |
|---|---|---|
| material | `material_records` / `material_id` | `allowables` / `allowable_id` |
| section | `section_records` / `section_id` | `dimensions` / `dimension_id`, `properties` / `property_id` |
| component | `component_records` / `component_id` | `fields` / `field_id` |

## Changes (frontend only; no schema/runner/Rust change)

### `apps/desktop/src/services/libraryImportService.ts`
- New pure, unit-testable helpers (the per-kind dispatch table mirrors the Rust
  resolver):
  - `indexLibraryRecordsSlots(document, libraryKind) -> LibraryRecordIndex[]`:
    extracts record ids and, per record, the value-slot ids across every slot
    array for the kind (de-duplicated, first-seen order — exactly the membership
    a slot-existence check needs). Records/slots without a string id and
    non-object/non-array shapes are skipped (never invented).
  - `classifyLibraryReference(records, recordId, slotId) -> "resolves" |
    "record_missing" | "slot_missing"`.
- New exported types `LibraryRecordIndex` and `LibraryReferenceResolution`
  (`resolves | record_missing | slot_missing | library_missing`;
  `library_missing` is decided upstream from the store list / a null open).

### `apps/desktop/src/features/rule-check/RuleCheckRunPanel.tsx`
- Each `private_library_value` input with a reference now renders a **"Preview
  resolution"** button. On click, `handlePreviewLibrary` calls the existing
  `listLocalLibraries(projectId)` (finds the referenced library by `(kind, id)`
  and gathers the available libraries of that kind for browsing) and, when found,
  `openLocalLibrary(...)`, then `indexLibraryRecordsSlots` +
  `classifyLibraryReference` to produce a status.
- A new read-only browse view (`rule-check-library-browse-<id>`) shows: a
  resolution badge (`data-status` ∈ resolves / library_missing / record_missing /
  slot_missing / unavailable / unsupported_kind / error) with a plain-language
  message, the available local libraries of the kind, and the referenced
  library's records → slot ids with the authored record/slot marked
  "(referenced)". Unknown `library_kind` (not material/section/component) is
  surfaced as a note ("cannot resolve at run time"); browser preview surfaces the
  desktop-only store seam.
- State `libraryPreviews` is reset on every new pack load (`resetForNewPack`), so
  a stale preview never carries across packs.

## Evidence

- Desktop Vitest: **357 pass** (+12 over the 345 baseline):
  - `libraryImportService.test.ts` (+7): `indexLibraryRecordsSlots` across
    material / section (both slot arrays, scan order) / component, plus the
    skip-malformed/de-dup/non-array/null cases; `classifyLibraryReference`
    resolves / record_missing / slot_missing.
  - `RuleCheckRunPanel.test.tsx` (+5): preview button present for a
    referenced library input; a resolving reference renders `data-status=resolves`
    and browses `mat:a (referenced)` / `allow:Sh (referenced)` / `allow:Sc` from
    a mocked store; library_missing lists the available libraries and
    short-circuits before `open_local_library`; record_missing vs slot_missing
    against the stored library; and the browser-preview desktop-only seam.
- `npx tsc -b` (desktop): clean. `npm run build` (desktop): clean (the
  >500 kB chunk note is the pre-existing advisory, not an error).
- Five-surface DEC-025 sweep: see the committed sweep summary (cargo crate sweep,
  pytest, desktop Vitest+wasm, Playwright e2e ×2, production build).

### UI evidence posture (H4 amendment)
This tranche adds a user-visible desktop surface, so the **browser-reachable**
slice is automated in Playwright (default per H4): the existing
`r2-smoke.spec.ts` rule-check test now pastes a `private_library_value` pack,
asserts the "Preview resolution" control renders, clicks it, and asserts the
honest desktop-only store seam (`LIBRARY-IMPORT-BACKEND-DESKTOP-ONLY`) — passing
on both viewports (10/10). The **desktop-store resolution/browse itself**
(library_missing / record_missing / slot_missing / resolves against the local
SQLite store) is desktop-only and not browser-reachable, so it is exercised by
the Vitest desktop-mode mocked-`invoke` suite — the documented exception, the
same posture `TP-C3C4-LIBREF-001` / `TP-C3-LIBREFSECCOMP-001` used for the
resolution they shipped.

## Residuals and hand-offs

- The C3 dependency-spine residual "richer C4 run-panel library/record/slot
  picker" is now **closed**: the panel browses library → record → slot and
  previews resolution, no longer just read-only text.
- **Non-goal (not a residual):** run-time *override* of the authored reference in
  the run panel. Deferred behind a human design ruling (it would change the
  "reference authored in the pack" model); not built here.
- Open C4 residuals (unchanged, separate spine item): driving `aggregate_status`
  into the solve envelope / `result_export`; the future additive
  `acceptability_relation` / solver-result-selector schema members.

## Boundary compliance

Frontend-only; local-only (Tauri store commands + invented test fixtures; no
network/daemon/telemetry). The picker is read-only with respect to the rule pack;
private library values are read only at run time and are never embedded in the
pack, committed, or rendered in the panel. Status-vocabulary-only; no
compliance/certification/sealing/authentication/approval/code-compliance or
professional-acceptance claim. Deliverables stay `CHECKING`. Git/test evidence is
source-control hygiene only.

## Open decisions awaiting human ruling

- **D-06** (release matrix / installer / signing) — `NOT_PREPARED`; gates Phase E
  packaging only; not on the R3/Phase C spine.
- **D-11** (issuance waves) and **D-12** (FR-024/025 disposition) — human-paced /
  R5-gate; not blocking current-stage work.
- No decision blocks the next Phase C item.
