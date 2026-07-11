# WORKING_ITEMS Run Record - TP-E3-CONTAINER-001

**Date:** 2026-07-10
**Persona:** Type 2 bounded tranche worker
**Tranche:** `TP-E3-CONTAINER-001`
**Primary phase item:** R5 / Phase E / E3 - full report package container assembly
**Primary package:** `PKG-08` - Reporting, Audit, and Reproducibility
**Touched deliverables:** `DEL-08-01` (report generator surface; container assembly),
consuming `DEL-08-02` (audit manifest), `DEL-08-04` (result export),
`DEL-08-06` (state/comparison/handoff report sections) member contracts
**Decision basis:** `DEC-028` (D-09 Option C — multi-member archive per the
PKG-17 export-package contracts), `DEC-057` (D-06 O-A naming rider —
`.opsproj` / "OpenPipeStress Project Package"), `DEC-061` (D-10b O-A —
deterministic PDF emitter, landed by `TP-E3-PDFEMIT-001`)
(`execution/_Decomposition/SOFTWARE_DECOMP.md` §12)

## Objective

Assemble the remaining E3 "full report package" container (core-side only):
a deterministic report-package assembler producing the `DEC-028`
multi-member archive with `DEC-057` naming, whose members are the `DEC-061`
canonical rendered report HTML + hash-bound deterministic PDF, the
audit-manifest member, the result-export envelope member(s), and the
state/comparison/handoff report-section envelope member(s), with a
DEL-17-02-style package manifest recording per-member JCS-basis SHA-256
hashes (REQ-007/011/040-043). Hash-bound and deterministic throughout: no
timestamps, no generated identifiers; identical inputs produce identical
bytes for every member, the manifest, and the physical container.

## Inputs

- `core/reporting/pdf_emitter` (`DEC-061` rendered-report pair;
  `assemble_full_report_package` seam from `TP-E3-PDFEMIT-001`).
- `core/reporting/report_renderer` (canonical HTML pipeline, `DEC-021`).
- `core/reporting/audit_manifest`, `core/reporting/result_export`
  (member data models and validators).
- `core/serialization/canonical_json` (RFC 8785 JCS renderer, H5) for
  canonical-JSON member bytes.
- `execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-02_*`
  Specification REQ-007/011/040-043 (manifest/member contracts).
- `fixtures/reports/invented/calculation_report_fixture.json` — invented
  fixture data only.

## Changes

- `core/reporting/report_package` (NEW crate,
  `open_pipe_stress_report_package`), a sibling of `pdf_emitter` (which
  stays the rendered-report emission seam; the container sits above it in
  the dependency graph, avoiding the `report_generator` cycle noted in the
  `TP-E3-PDFEMIT-001` record):
  - `assemble_report_package_container`: assembles the `DEC-028`
    multi-member archive. Members, in fixed archive order:
    1. `package_manifest.json` — DEL-17-02-style manifest: member inventory
       with per-member SHA-256 over exact bytes (for canonical-JSON members
       the exact bytes ARE the RFC 8785/JCS rendering, so the recorded hash
       is the JCS-basis hash, REQ-007/011), source model reference, package
       ID, export profile ID, source-basis references (REQ-040),
       diagnostics + boundary notes (REQ-041), target-field status map
       (REQ-042), and sidecar identification (REQ-043 — none required;
       canonical members carry identity directly, and the rendered HTML/PDF
       identities are carried by the manifest). The manifest's own SHA-256
       is the package identity per `DEC-028` (evidence binds to canonical
       members and manifest per-member hashes, not raw container bytes).
    2. `calculation_report.html` + 3. `calculation_report.pdf` — the two
       `DEC-061` deterministic emissions of one assembled section model,
       hashes identical to the `pdf_emitter` outcome's recorded hashes.
    4. `audit_manifest.json` — the DEL-08-02 audit manifest as a
       canonical-JSON member (validated by `audit_manifest::validate_manifest`).
    5. `result_export_{envelope_id}.json` — one canonical-JSON member per
       DEL-08-04 result envelope (validated per envelope;
       `result_export::result_export_document`).
    6. `state_comparison_handoff_sections_{section_set_id}.json` — the
       DEL-08-06 report-section records as canonical-JSON members
       (caller-assembled; envelope shape and blocking diagnostics checked).
  - `DEC-057` naming: container file name `{sanitized package_id}.opsproj`,
    document kind "OpenPipeStress Project Package", recorded as crate
    constants and in the manifest.
  - Physical container: deterministic hand-rolled ZIP (zero new
    dependencies): stored/uncompressed members in fixed order, fixed ZIP
    epoch DOS timestamps (1980-01-01 00:00:00 constant, never a real time),
    no extra fields/comments, in-repo CRC-32; container SHA-256 recorded.
    Member bytes are exposed on the outcome so callers can also emit the
    `DEC-028` directory form.
  - Gating: rendered members carry every existing HTML/PDF gate; audit
    manifest and result envelopes are validated with their own crates'
    validators; DEL-08-06 records must carry the DEL-08-06 envelope; any
    blocking finding marks the package `export_blocked` (a blocked package
    still assembles with reasons recorded in the manifest for diagnosis).
  - Dependencies: five in-repo crates plus `serde`, `serde_json`, `sha2` —
    all already established project-wide. Zero new external dependencies
    (`DEC-023` posture).
- `core/reporting/report_package/tests/container.rs` (NEW): 12-test suite —
  double independent-build byte-identity of container/manifest/members;
  fixed member inventory and order; manifest-inventory/hash agreement
  recomputed from member bytes; JCS idempotence of every JSON member;
  HTML/PDF member hashes bound to the `DEC-061` emission hashes; a
  test-side independent ZIP reader (separate table-driven CRC-32)
  round-tripping every member byte-identically and asserting stored
  entries + fixed DOS timestamps; input-change hash propagation
  (member -> package identity -> container); gate blocking for invalid
  audit manifest, missing named members, non-DEL-08-06 records, and a
  blocked report input; `.opsproj` naming/sanitization. Runs under the
  standard cargo sweep (DEC-025 `cargo_crate_sweep` discovers all crate
  manifests).

## Evidence

- `cargo test --manifest-path core/reporting/report_package/Cargo.toml` —
  12 passed, 0 failed.
- Sibling reporting crates re-run, all passed: `pdf_emitter` 8,
  `report_renderer` 8, `report_generator` 10, `report_sections` 13,
  `protected_content_linter` 4, `audit_manifest` 13, `result_export` 12.
- `cargo fmt --check` and `cargo clippy --all-targets` clean on the new
  crate.
- `PYTHONDONTWRITEBYTECODE=1 python3 -m pytest -q` from
  `projects/chirality-piping` — 387 passed.
- `PYTHONDONTWRITEBYTECODE=1 python3 tools/practitioner_harness/harness.py
  self-check` from repo root — exit 0 (pre-existing findings only; none
  introduced by this tranche).
- External parse witness (local, not committed): the fixture container
  (`invented-package-001.opsproj`, SHA-256
  `d22cf491422afd878a585af98604a7cc2207e788cd63244fa59f082d9d1cd3e0`,
  package identity / manifest SHA-256
  `d916b02de61f2a01142998ea54cad48f6aa832231ef68e0dc19003bac34c1990`)
  opens in Python `zipfile`: `testzip()` reports all CRCs OK, all six
  members extract byte-identically, every archive timestamp is the fixed
  1980-01-01 00:00:00 constant, and every manifest member hash matches a
  recomputed SHA-256 of the extracted member bytes.
- DEC-025 five-surface sweep at the committed HEAD: see the sweep summary
  committed with this branch under `validation/evidence/sweeps/`.

## Runner binding status (mandate condition evaluated)

Not wired; recorded as a residual. The `DEC-065` verb set (`solve`,
`validate-input`, `export-results`, `run-benchmark`, `run-regression`) has
no natural existing home for report-package assembly: there is no report
verb; `solve` output is the runner-result + mechanics-envelope contract;
and `export-results` is an explicit `DEC-065`-stable stub whose downstream
payload binding is already named as its own later bounded tranche
(`HEADLESS_RUNNER_OPERATION_STUB_REQUIRES_DOWNSTREAM_PAYLOAD`). Adding a
new verb is ruled vocabulary and out of scope. The library surface + tests
land here; the runner/app binding of the package seam remains the named E3
residual.

## Boundary Review

- No new third-party dependencies in any `Cargo.toml`; the ZIP writer and
  CRC-32 are in-repo code.
- No network, cloud, or telemetry surface; the assembler writes no files.
- Invented fixture data only; no protected standards content; all existing
  report lint gates carried forward unchanged through the rendered members.
- The container is an artifact format, not an issuance act (F-PIP-2/3):
  no release-readiness, lifecycle, issuance, professional approval,
  certification, sealing, authentication, or code-compliance claim is made
  by this tranche; no `_STATUS.md` lifecycle transition occurred. The R5
  package question remains the human R5 exit review's (`D-10b` §7).

## Residuals

- App/runner binding of the package seam: neither the desktop app nor the
  headless runner calls `assemble_report_package_container` yet (see
  "Runner binding status" above); future bounded work (the desktop menu
  binding is a named follow-on tranche).
- On-disk save of the container (atomic write-temp/rename posture named by
  the D-09 packet) is caller-side work that lands with the app/runner
  binding; this crate deliberately writes no files.
- The DEL-08-06 state/comparison/handoff records are caller-supplied JSON
  (their assembler is the Python DEL-08-06 engine); a Rust-side builder or
  cross-language bridge, if ever wanted, is separate work.
- Compatibility window / versioning policy for the container form beyond
  `schema_version` "1.0.0" remains open `DEC-028` bounded-tranche territory.
