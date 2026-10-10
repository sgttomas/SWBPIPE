# Model document open path and native smoke

The desktop's ordinary walking skeleton (owner request 2026-10-10: "Want to make
a 'walking skeleton' of the app to allow for such testing? … You may do so."):

open a model document file → check it (named refusals) → solve sparse and dense
→ standing, results, diagnostics and solve proof → save as a local project →
reopen → export. Only the open step is new. Solve, persistence (DEL-02-05) and
export use their existing routes unchanged.

## Open path

- **Entry.** File > Open Model Document… (native menu and in-DOM menu id
  `file.open-model`; project page button `open-model-document`) →
  `handleOpenModelDocument` (`src/features/workspace/workspaceSession.ts`) →
  `openModelDocumentFile` (`src/services/modelDocumentFile.ts`) → native command
  `open_model_document_file` (`src-tauri/src/model_document_file.rs`).
- **File checks (native).** These run on the file the user chose in the native
  chooser:
  - `MODEL-FILE-UNREADABLE`;
  - `MODEL-FILE-TOO-LARGE` (64 MiB);
  - `MODEL-FILE-NOT-JSON`;
  - `MODEL-FILE-NOT-A-DOCUMENT`;
  - `MODEL-FILE-DOCUMENT-KIND` (`openpipestress.product_preview.model` only, so a
    solve request `{model, materials}` is refused);
  - `MODEL-FILE-SCHEMA-REFUSED`, for the statuses that project open also refuses:
    `newer_than_supported`, `unsupported_schema` and `failed` from the existing
    `evaluate_model_document`.

  When the one-current-version rule lands (B7), older versions are refused here
  through that same evaluation. This path adds no migration or compatibility
  code.
- **Shape check (webview).** `MODEL-FILE-SHAPE` with the JSON path of each
  missing or mistyped member that the desktop's `PreviewModel` requires and that
  the desktop always writes:
  - `data_boundary`, `project.name`, `project.description`, `analysis_status.*`;
  - entity `id`/`label`/`provenance` and the other required strings;
  - `components`, `diagnostics`, load-case `status`.

  It lives in `src/features/model-file/modelDocumentShape.ts`, beside the type
  it mirrors.
- **Adoption.** The session adopts the document exactly as read. No default is
  filled in, no in-memory migration is applied, and nothing is normalized. The
  solve proof's `model_sha256` is therefore the RFC 8785 JCS SHA-256 of the
  file's document. The document starts unsaved, like the bundled demo. New
  Local Project saves it, and the existing persistence normalization still
  applies on save (for example 0.1.0 → 0.2.0 with its ledger record). A refusal,
  a cancelled chooser or a superseded request leaves the session as it was.
- **Display fields are part of the desktop document contract.** A document
  without them is refused by name rather than given defaults (label = id,
  empty lists). Defaults would make the session's document, and its hash,
  differ from the file the user opened. Solver-side documents such as
  `rf_skew_t_cant_off_122_r1e-04.request.json`'s `model` are solve inputs, not
  desktop documents. `rf_skew_t_cant_off_122_r1e-04.desktop_model.json` is the
  milestone in desktop shape: the same model plus display fields.
- **No implicit model.** The working-directory fixture lookup is gone. The
  bundled invented demo and its design knowledge are compiled into the binary
  (`include_str!`), so a packaged app starts the same from any directory. A
  solve without a model is refused (`PREVIEW-SOLVE-MODEL-REQUIRED`).
- **Browser preview.** The browser preview cannot open files and refuses with
  `MODEL-FILE-NATIVE-ONLY`.
- **Scope fence.** This is the ordinary route only. The retained/Direct entry,
  its activation and the successor-panel witness belong to B8, and B8's
  activation is owner-held. B8's desktop caller qualification uses this open
  path and adds no second one.

## Native smoke (VER-003 evidence, not the owner's witness)

`apps/desktop/native-smoke/run.mjs` builds the frontend and the app with the
`native-smoke` cargo feature. No product build enables that feature. It then
runs the real macOS app twice under a throwaway `HOME`, so the project store and
Downloads are isolated. The harness (`src-tauri/src/native_smoke.rs`,
`native-smoke/driver.js`) substitutes only two things for a person:

- it names the file the open chooser returns;
- it sends native menu commands through the same dispatch a menu click uses.

Everything else is product code.

**Phase 1** first checks three refusals by name:

- a newer schema;
- a solve request's bare model (shape);
- a file that is not JSON.

Then, for the milestone and the demo, it:

- opens the document;
- solves sparse and dense;
- reads the standing line, the row count (solve proof and result filter), the
  `result:disp:N1` "Entered" value, the solve proof's `model_sha256` and any
  `RETAINED_PRECISION_*` codes;
- exports the result JSON where the product offers it;
- saves the document as a local project.

**Phase 2** is a new process. It reopens each project and solves again.

`run.mjs` checks the observations against `native-smoke/expected.json`. Those
values come from T3's G10 jsdom replay and are not re-pinned from a native run.
It also checks:

- the proof hash against its own JCS hash of each file;
- each saved model against the opened document;
- the exported file in Downloads.

Run it on a Mac with a GUI session:

```sh
node apps/desktop/native-smoke/run.mjs   # add --keep to retain the evidence folder
```

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
