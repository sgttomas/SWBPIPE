# WORKING_ITEMS Run Record - TP-E3-PDFEMIT-001

**Date:** 2026-07-10
**Persona:** Type 2 bounded tranche worker
**Tranche:** `TP-E3-PDFEMIT-001`
**Primary phase item:** R5 / Phase E / E3 - full report package + PDF emitter
**Primary package:** `PKG-08` - Reporting, Audit, and Reproducibility
**Touched deliverables:** `DEL-08-01` (calculation report generator/renderer surface)
**Decision basis:** `DEC-061` / D-10b Option O-A
(`execution/_Coordination/_DECISIONS/D-10b_deterministic_pdf_emitter.md` §5.1;
`execution/_Decomposition/SOFTWARE_DECOMP.md` §12 `DEC-061`)

## Objective

Implement the ruled hash-bound deterministic PDF emitter: a pure in-repo
minimal deterministic PDF writer (zero new external dependencies) that
consumes the SAME assembled section model the deterministic HTML renderer
emits from, so HTML and PDF are two deterministic emissions of one assembled
section model, each SHA-256-recorded per the existing convention.

## Inputs

- `core/reporting/report_renderer` (canonical HTML pipeline, DEC-021) — the
  single assembled-section source (`assemble_sections`, previously private).
- `core/reporting/report_generator`, `report_sections`,
  `protected_content_linter` — existing validation and lint gates.
- `fixtures/reports/invented/calculation_report_fixture.json` — invented
  fixture data only.

## Changes

- `core/reporting/report_renderer/src/lib.rs`:
  - The previously private HTML-shaped section assembly is refactored into a
    public emission-neutral block model: `SectionBlock`
    (Subheading / Paragraph / Table / BoundaryBox), `AssembledSection`, and
    `assemble_report_sections`, plus `section_body_html` for the HTML
    emission of the blocks. Verified byte-identical: the fixture render hash
    before and after the refactor was
    `498a8d71d33d2724cb14ae40d82a4dbb5cf65f5c9061d3bd6c1c4585317c596f`
    (probe test, removed after verification).
  - Exposed emission-neutral frame text helpers shared by both emissions:
    `report_meta_text`, `status_banner_text`, `export_blocked_banner_text`,
    and made `lint_provenance` public so the PDF emitter gates its text model
    with the identical provenance mapping. HTML bytes unchanged by these
    (escaping is character-wise, verified by the same probe hash).
  - One deliberate content change to the shared audit-manifest paragraph:
    "The SHA-256 of this rendered HTML document ..." is now emission-neutral
    ("this rendered calculation-report document ... The HTML and PDF
    emissions of the same assembled report sections are each hash-recorded
    this way ..."), because the paragraph is now part of the shared section
    model both documents carry. This changes the canonical HTML bytes; the
    fixture render hash after the change is
    `fde75537c97aa43f7d0d8cfce32b29bf9f1476263ae149370000c3af7ea2b626`.
    No golden hash is pinned anywhere in-repo (checked; determinism tests
    assert re-render equality, not a stored digest).
  - The document-frame trailing note and `derived_print_view` labeling are
    untouched: the derived webview print view keeps its ruled
    "not hash-bound evidence" role (`DEC-021`).
- `core/reporting/pdf_emitter` (NEW crate,
  `open_pipe_stress_pdf_emitter`): pure in-repo minimal deterministic PDF
  writer at the deliberately plain `D-10b` §5.1 scope:
  - text, tables, and banner boxes only; base-14 standard fonts
    (Courier / Courier-Bold, WinAnsiEncoding — no font embedding or
    subsetting); uncompressed content streams; fixed document metadata
    (constant `/Title` and `/Producer`, no `/CreationDate`, no `/ModDate`,
    no trailer `/ID`, nothing varying);
  - deterministic layout: fixed Letter geometry, monospace metrics
    (Courier 0.6 em advance makes wrap widths exact), greedy word wrap,
    deterministic pagination with repeated table header rows and
    deterministic row splitting across pages;
  - `emit_calculation_report_pdf` / `pdf_from_render`: the HTML render gates
    (report + section validation, pre-render section lint, post-render
    document lint) are evaluated first and the PDF is never less gated than
    the HTML emission; a blocked input carries a visible EXPORT BLOCKED
    banner in the PDF; the PDF document text model is linted post-emission
    (the analogue of the renderer's post-render gate iii), on the shared
    pre-render text, per the packet §2 "Unresolved" third bullet (no binary
    re-extraction exists; gating is text-model-side by design);
  - `assemble_full_report_package`: emits HTML + PDF from one assembled
    section model and records both as `PackageMember` rows
    (role / file name / media type / SHA-256 / byte length), the PDF hash
    recorded alongside the canonical HTML hash per the existing
    `lib.rs` audit-manifest pattern.
  - Dependencies: four in-repo reporting crates plus `serde` and `sha2`,
    both already used project-wide. Zero new external dependencies
    (`DEC-023` posture).
- `core/reporting/pdf_emitter/tests/emit.rs` (NEW): byte-golden determinism
  suite mirroring `report_renderer/tests/render.rs:122-158` — byte-identical
  re-emission, 64-hex lowercase hash, `%PDF-1.4` header / `%%EOF` trailer,
  no timestamps, no document ID, no external references, gate blocking
  (prohibited professional claim, missing report id), package member hash
  binding, single-sourced section titles in both emissions, and
  deterministic non-WinAnsi degradation. Runs under the standard cargo test
  sweep (DEC-025 `cargo_crate_sweep` discovers all crate manifests).

## Evidence

- `cargo test --manifest-path core/reporting/pdf_emitter/Cargo.toml` — 8
  passed, 0 failed.
- `cargo test --manifest-path core/reporting/report_renderer/Cargo.toml` — 8
  passed, 0 failed (existing suite untouched).
- Sibling reporting crates (`report_generator` 10, `report_sections` 13,
  `protected_content_linter` 4, `audit_manifest` 13, `result_export` 12) —
  all passed.
- `cargo fmt --check` and `cargo clippy --all-targets` clean on both touched
  crates.
- `PYTHONDONTWRITEBYTECODE=1 python3 -m pytest -q` from
  `projects/chirality-piping` — 377 passed.
- `python3 tools/practitioner_harness/harness.py self-check` from repo root —
  exit 0 (pre-existing findings only; none introduced by this tranche).
- External parse witness (local, not committed): the fixture PDF
  (SHA-256 `0adb2939d72f3b00b8ed4f31d2ecd07096e3359c173e7f739bd4713f0e7b986a`,
  3 pages) parses in pdftotext and PyMuPDF with correct xref offsets, the
  fixed metadata above, and all section content classes rendering (tables,
  banners, boundary box, sign-off grid).
- DEC-025 five-surface sweep at the committed HEAD: see the sweep summary
  committed with this branch under `validation/evidence/sweeps/`.

## Boundary Review

- No new third-party dependencies in any `Cargo.toml`.
- No network, cloud, or telemetry surface; the emitter writes no files.
- Invented fixture data only; no protected standards content; lint gates
  carried forward unchanged.
- The derived webview print view keeps its "not hash-bound evidence"
  labeling; nothing was removed or relabeled.
- No release-readiness, lifecycle, issuance, professional approval,
  certification, sealing, authentication, or code-compliance claim is made
  by this tranche; no `_STATUS.md` lifecycle transition occurred. The R5
  package question remains the human R5 exit review's (`D-10b` §7).

## Residuals

- E3 remainder: the §22.6 "full report package" as a broader container —
  audit-manifest / result-export / state-comparison-handoff envelope members
  and any on-disk packaging — is NOT assembled by this tranche; the
  `assemble_full_report_package` seam currently records the two canonical
  rendered-report members (HTML + PDF) with their hashes. Container/naming
  decisions stay with `D-06` per `DEC-028`.
- App/runner binding: the desktop app and headless runner do not yet call
  `assemble_full_report_package` or surface the PDF hash; future bounded
  work.
- Architecture note (scope delta, recorded): the package-assembly entry
  point lives in the new `pdf_emitter` crate, not in `report_generator`,
  because `report_generator` sits below `report_renderer` in the crate
  dependency graph and cannot call either emitter without a cycle;
  `report_generator` remains the report data-model crate.
- Visual fidelity is deliberately plain (monospace evidence document), per
  the ruled §5.1 scope.
