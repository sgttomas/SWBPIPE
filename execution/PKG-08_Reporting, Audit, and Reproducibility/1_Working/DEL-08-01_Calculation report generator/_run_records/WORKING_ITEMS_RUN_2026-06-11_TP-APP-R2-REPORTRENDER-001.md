# WORKING_ITEMS RUN — 2026-06-11 — A7 report rendering: deterministic hash-bound HTML renderer (TP-APP-R2-REPORTRENDER-001)

- **Tranche:** `TP-APP-R2-REPORTRENDER-001` — completion-plan Phase A item A7,
  implementing the D-10 ruling (`DEC-021` in
  `execution/_Decomposition/SOFTWARE_DECOMP.md` §12, selecting Option B of
  `execution/_Coordination/_DECISIONS/D-10_report_rendering_target.md`):
  deterministic, self-contained, scriptless single-file HTML from a Rust
  renderer crate as the canonical hash-bound calculation-report artifact;
  three-point linter gating with the post-render gate blocking export;
  webview print-to-PDF as a derived, non-hash-bound view naming the
  canonical hash.
- **Run by:** WORKING_ITEMS (Type 1 persona) inline; parallel TASK workers
  ran disjoint crate scopes (C1 grammar in `core/rules`, D-03 solver in
  `core/solver`) under separate run records.
- **Authority basis:** `DEC-021`; DEL-08-01/DEL-08-03/DEL-08-05 contracts as
  mature design authority (`CHECKING`); no lifecycle change.

## What landed

1. **New crate `core/reporting/report_renderer`** consuming the existing
   `CalculationReport` (report_generator) + `ReportSections`
   (report_sections) structs plus caller-formatted result rows
   (`RenderableReportInput`), emitting a deterministic single-file HTML
   document: fixed section order mirroring the eight required section kinds,
   inline CSS with a system font stack, no scripts, no external references,
   all dynamic text HTML-escaped. `RenderOutcome` carries the document, its
   SHA-256 (the canonical hash-bound evidence value), gate findings, and
   `export_blocked`. `derived_print_view` produces the labeled derived view
   naming the canonical hash for print/PDF use. The signoff block renders as
   empty human fields; the six automatic analysis statuses render verbatim.
2. **Three-point linter gating** via the existing
   `protected_content_linter`: (i) the bundled template surface
   (`public_report_template`) linted as `PublicReportTemplate` in tests;
   (ii) assembled section text linted pre-render; (iii) the final document
   text linted post-render. Any blocking finding — including
   `ProhibitedProfessionalClaim` — or blocking validation diagnostic from
   `validate_report`/`validate_report_sections` marks the outcome
   export-blocked, and the document itself renders a visible
   `EXPORT BLOCKED` banner naming the reasons.
3. **Feature-gated serde derives** (`serde` feature, default-off) added to
   `report_generator` and `report_sections` so the renderer/desktop seam
   deserializes the canonical JSON shapes; spellings pinned to the schema
   contract (SCREAMING_SNAKE_CASE analysis statuses and diagnostic classes,
   snake_case otherwise, `"TBD"` uppercase, envelope field `ref`), proven by
   deserializing `fixtures/reports/invented/calculation_report_fixture.json`
   in the renderer tests.
4. **Desktop command seam:** new Tauri command `render_calculation_report`
   (15th command) deserializing the input, rendering, and returning the
   outcome plus `derived_print_html`; registered in the invoke handler;
   src-tauri depends on the renderer crate.
5. **Frontend:** `services/reportRenderService.ts` (Tauri-only route; the
   browser preview returns an explicit
   `REPORT-RENDERER-DESKTOP-ONLY` unavailable route — no fallback renderer);
   `features/report/renderableReportInput.ts` adapter composing the input
   from session envelopes (run-record/result-envelope hashes, load cases,
   diagnostics, reproducibility TBDs; locally computed canonical-JSON
   SHA-256 for the model and sections payloads; explicit `TBD` markers, e.g.
   persistence ref before the project is saved);
   `features/report/RenderedReportPanel.tsx` (render button, canonical-hash
   display, gate state, save-canonical-HTML and derived-print actions that
   are refused while export is blocked, sandboxed preview iframe); wired
   into `App.tsx` beside the existing JSON report packet panel (kept
   distinct per the ruling).

## Evidence (focused, pre-sweep)

- `cargo test --manifest-path core/reporting/report_renderer/Cargo.toml` —
  8/8 (determinism, fixture deserialization, unblocked fixture render,
  template lint gate, prohibited-claim blocking through both lint points,
  incomplete-report blocking, derived-view labeling, escaping, JSON
  round-trip).
- `cargo test` report_generator 10/10, report_sections 13/13 (with and
  without the new serde feature).
- src-tauri `render_calculation_report` command test 1/1 (fixture render
  unblocked; invalid input rejected with `RENDER-INPUT-INVALID`).
- `npm test --workspace apps/desktop` — 148/148 (8 new in
  `renderedReport.test.tsx`: service routing both modes, adapter mapping
  with TBD markers and persisted-project ref, panel hash/gate display,
  blocked-state refusal of save/print, explicit desktop-only diagnostic).
- `npm run build --workspace apps/desktop` — green (standing chunk-size
  warning only).
- `python3 -m pytest -q tests -k report` — 22/22 (fixture/schema contracts
  unaffected).
- Full five-surface sweep evidence at the tranche push is recorded in the
  commit-bound sweep summary under `validation/evidence/sweeps/` and in
  `apps/desktop/SMOKE.md` TP-MAC-113.

## Boundary review

Local-only rendering (inline CSS, system fonts, no network fetch, no
scripts); invented fixture and session data only; the rendered document and
outcome carry no protected standards content and no private payloads beyond
the same reference/hash metadata already in the session envelopes. The
professional-boundary notice, empty human signoff block, and
`ProhibitedProfessionalClaim` blocking enforce PRD §15.2; nothing in this
tranche creates a release-readiness, professional approval, certification,
sealing, authentication, or code-compliance claim.

## Residuals / handoffs

- Browser-mode rendering seam (wasm-engine render export) intentionally not
  built; browser preview reports the explicit desktop-only route. Decide at
  A8 whether end-to-end report automation needs a wasm render export or a
  packaged-Tauri harness.
- Rule-pack refs render as an empty table until Phase C wires rule packs
  into the desktop solve path (renderer + section contract already accept
  them).
- Adapter maps app diagnostic severities to section diagnostic classes
  (`blocking|error → SOLVE_BLOCKING`, else `ASSUMPTION_WARNING`) with a
  fixed human-review remediation line — vocabulary mapping to revisit when
  rule-check diagnostics arrive (Phase C).
- Session provenance is recorded with the app's existing invented-preview
  vocabulary; provenance vocabulary for user-authored local models is a
  Phase B/C follow-up.
- The rendered-document hash is displayed and embedded in the derived view;
  persisting it into the local project store (alongside the run-record
  hashes) is a small follow-up when report persistence lands.
- D-10b (hash-bound deterministic PDF emitter) remains deferred to the R5
  lead-up per the ruling.
