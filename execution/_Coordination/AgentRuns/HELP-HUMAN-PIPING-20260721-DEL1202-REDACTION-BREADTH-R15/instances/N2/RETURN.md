# N2 Return — Desktop report preview/export and bug-report inventory

**Status:** `SUCCESS`; read-only; frozen SHA verified.

The report section has seven active exposure routes: raw DOM preview, JSON
download, pre-render Tauri IPC, HTML iframe preview, HTML save, print/PDF, and
lint JSON. None calls the existing TypeScript redaction mirror. The report
renderer and protected-content linter are complementary gates, not private-
data redaction. The HTML iframe is populated even when the renderer records
`export_blocked`.

A material projection defect was found: restored user-local project data can
be labelled with an `invented_public_example` session posture before any
redaction decision. A whole-object redaction call is unsafe because the walk
stops at metadata-bearing containers; the later implementation needs a
report-specific metadata projection and schema-compatible materializer before
DOM preview, IPC, hashing, data-URI creation, or print-frame creation.

No bug-report product egress exists: no UI command, Tauri command, writer,
bundle, or transport was found. Threat-model references and symbolic storage
roots are declarative/TBD only. A future route must use the same control, but
this tranche must not create it.

Browser Playwright can cover report DOM and JSON behavior. Pre-IPC native
blocking/sanitization requires mocked-Tauri Vitest plus existing Rust command
tests; the absence of a real native-GUI automation lane must be recorded
rather than represented as Playwright coverage.

