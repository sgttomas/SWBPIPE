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
  - `MODEL-FILE-SCHEMA-REFUSED`, for the statuses that project open and save
    also refuse: `retired_schema` (any version below 0.7.0, including 0.1.0
    through 0.6.0, with the `MODEL_SCHEMA_VERSION_RETIRED` re-author
    diagnostic), `newer_than_supported` and `unsupported_schema` from the one
    version check, `evaluate_model_document`
    (`src-tauri/src/model_document_version.rs`; DEL-02-05 CLM-003). Model
    document 0.7.0 is the one current version; B7b retired 0.3.0 and 0.4.0,
    T4-U4b/U5b retired 0.5.0 and W-13 S1b retired 0.6.0. The version check reads
    the version only: a 0.7.0 document that declares a
    retired pressure contract passes it unchanged, and its solve publishes no
    rows, with the blocking diagnostic `PRESSURE_MODEL_REAUTHOR_REQUIRED`. This
    path adds no migration or compatibility code.
- **Shape check (webview).** `MODEL-FILE-SHAPE` with the JSON path of each
  missing or mistyped member that the desktop's `PreviewModel` requires, the
  desktop dereferences, and the desktop always writes:
  - `data_boundary`, `project.name`, `project.description`, `analysis_status.*`;
  - entity `id`/`label`/`provenance` and the other required strings;
  - `components`, `diagnostics`, load-case `status`;
  - node positions;
  - material `elastic_modulus`, section `properties` and pipe `section`
    quantities (`{value: number, unit: string}`);
  - support `restraints[]` strings;
  - combination `terms[]` (`load_case`, `factor`);
  - with W-6a (PR #1588), the members adoption and rendering dereference:
    every `project.units` value a string; a present `section_ref` a string; a
    present `components[].completeness` an array of `{diagnostic_code, status}`
    strings; a present `diagnostics[].id` a string; component `geometry`,
    `modifiers` and `mechanics_interface` members of their `PreviewComponent`
    types.

  The Rust save (`src-tauri/src/model_document_shape.rs`) applies the same check,
  kept equal by the shared corpus `fixtures/model_document/desktop_shape_cases.json`.
  `modelDocumentAdoption.test.tsx` retypes the demo's string members (433 cases)
  and requires each to be refused at admission or adopted and rendered without
  error.

  It lives in `src/features/model-file/modelDocumentShape.ts`, beside the type
  it mirrors. As a last guard, the session builds the model index for the
  candidate before adopting it. A document the index cannot build is also
  refused with `MODEL-FILE-SHAPE`, rather than unmounting the workspace.
- **Adoption.** The session adopts the document exactly as read. No default is
  filled in, no in-memory migration is applied, and nothing is normalized. The
  solve proof's `model_sha256` is therefore the RFC 8785 JCS SHA-256 of the
  file's document. The document starts unsaved, like the bundled demo. New
  Local Project saves it exactly as adopted: save applies the same version
  check and never rewrites the document. A save response whose model differs
  from the request is refused (`PROJECT-PERSISTENCE-RESPONSE-INTEGRITY`). A
  refusal, a cancelled chooser or a superseded request leaves the session as it
  was.
- **Project identity: never a silent replacement.** Local projects are keyed by
  `project.id`. HELP_HUMAN ruled on 2026-10-10: "the app never silently
  replaces a stored project". The bundled demo is no exception.
  - **Save in place.** HELP_HUMAN's confirmation on 2026-10-10: saving the
    project the session already is (it opened, created or last saved that
    `project.id`) "is an ordinary save in place and does not prompt". The
    condition is that "the stored project is still the revision this session
    last opened or saved". If the store has moved since (another window or
    process saved that id), "the save prompts like any other replacement".
  - **The store enforces both.** `create_local_project` and `save_local_project`
    refuse a write onto a stored id with `PROJECT-ID-EXISTS` unless one of these
    holds:
    - `confirmed_revision` equals the stored revision: the user chose Replace
      in a prompt that showed exactly that revision. A revision is the
      envelope hash, the last-saved time and a fingerprint of the stored row,
      so two different writes in one second differ. `find_local_project`
      returns the revision with the entry.
    - for Save, `expected_base` equals the stored envelope hash. This is the
      `project_envelope_hash` value of the revision the session last opened
      or wrote. The session records it wherever an open or a write lands for
      it, even when a later action superseded the request, so its own last
      save always counts as its base. The store already holds this hash, so
      no new revision field is needed. A row without an envelope hash (for
      example a legacy row) has no revision to compare, and a save onto it
      asks.

    A save in place whose base has moved, or a Replace of a revision that has
    since been saved over, is refused with `PROJECT-BASE-MOVED`. The app then
    asks again about the revision now stored, so a revision nobody was shown is
    never replaced.

    `create_project_in_store` and `save_project_in_store` (the Tauri commands
    are thin wrappers around them) each open one `BEGIN IMMEDIATE`
    transaction before preparing the write (the version check and the
    replacement check). They keep it through the check (`guarded_write_project_rows`)
    and the commit. The store's write lock
    is therefore held from the first read. A second writer, such as another
    window or process, waits on the lock (a 5 s busy timeout), then sees the
    first writer's row and is refused. If the wait times out, its save fails
    visibly and writes nothing. Two concurrent-writer tests drive those same
    functions. Both fail if the check runs outside the transaction, or if the
    commands use a deferred transaction. The browser preview's
    store mirrors the rule.
  - **The prompt.** On either refusal, the webview asks
    (`ReplaceProjectPrompt`, a modal alert dialog). It names the stored project
    and when it was last saved. For a moved base, it says the stored copy has
    been saved since this session last saw it.
    - **Replace "name"** resends the write, confirming the revision shown.
    - **Save as a new project** gives the document an id unused in the store:
      `<id>:copy-<timestamp>`, with `-N` appended if that is taken. Nothing else
      in the document changes. The session adopts the renamed document with a
      fresh undo history, so its model hash and the next solve proof's
      `model_sha256` change with the id. Editor intents, the proposal and the
      review target are carried over. The run solved under the old id is not.
    - **Cancel** is the default: it has focus, and Escape chooses it. The
      stored project is kept, and the status line says so.
  - **Modality.** While the prompt is open, the workspace is `inert`, Tab
    cycles the prompt's buttons, and menu commands and window shortcuts are
    ignored. When the prompt closes, focus returns to where it was.
- **Unsaved edits: never silently discarded (Agent 0, 2026-10-24).** A command
  that replaces the working model asks first while the model has edits no save
  holds (the toolbar's "Edited"). These are Open Local Project, opening a
  project from the index, Open Model Document…, New Blank and the blank
  load/reference model, and Open State (W-12). New Local Project saves the
  working model, so it does not ask.
  - **The prompt.** `UnsavedChangesPrompt` (`unsavedChangesGuard.ts`) is one
    modal alert dialog for every such command. It is modal as the replace
    prompt is, and sets its focus and key hold in a layout effect.
  - **The choices.**
    - **Cancel** is the default: it has focus as the prompt appears, and
      Escape chooses it. Nothing is replaced and the edits stay.
    - **Save Local Project, then …** runs Save Local Project's own flow,
      including its replace prompt. The command goes ahead only if that save
      lands.
    - **Discard edits and …** goes ahead without saving.
  - **Without edits** a command starts as before, with no prompt.
  - **One owner at a time (Codex r4).** None of these commands, nor Save
    Model State, starts a prompt or an operation while another project
    operation or prompt owns the session. The History panel's controls are
    disabled meanwhile, with a stated reason.
  - **Modal holds.** The replace prompt and the unsaved-changes prompt each
    take a hold on the shared modal flag (`modalHolds.ts`). Menu commands and
    shortcuts wait until the last hold is released, whichever prompt closes
    first.
  - **New Blank.** A New Blank whose time-stamped id is already stored takes a
    suffixed id, since a new blank has no identity of its own to keep.
  - **No normalization.** Save never rewrites a model document; B7a removed the
    0.1.0 → 0.2.0 normalization and its migration ledger records. The store's
    `model_migration_ledger_json` column (store v9) is no longer read or
    written; older rows keep their bytes.
  - **Browser preview store.** It keeps one snapshot per project id, as the
    native store does, so saving one project never discards another. This was
    HELP_HUMAN's direction of 2026-10-10. The multi-project store was chosen
    over a prompt before a discard, because a single-project preview was never
    a deliberate product limit. Open without an id opens the most recently
    written project. The native store orders by last-saved time at one-second
    resolution, then by id, so within one second native and browser can pick
    different projects.
  - **Known limit, recorded.** If `project.id` is edited during an in-flight
    save and then edited back, the session's recorded base can lag, and the
    next save asks as a moved base. This errs toward asking.
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
  solve without a model is refused (`PREVIEW-SOLVE-MODEL-REQUIRED`). So is the
  native sample-proposal command without the session's mechanics result. The
  webview's fixture-backed sample proposal for the browser preview is
  unchanged, and the app disables that action until a result exists.
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

Everything else is product code. The smoke build also turns off the webview's
background throttling, through Tauri's build-time `TAURI_CONFIG` merge, and
runs the app under `caffeinate`. macOS otherwise suspends a webview that is not
on screen, for example when the display sleeps, and the driver stalls. Product
builds and `tauri.conf.json` are unchanged.

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
values come from T3's G10 jsdom replay. The owner's ruling of 2026-10-24 set
the five row and N1 values to the native observations: milestone 113 rows in
both modes and N1 "…47199 mm" sparse, demo 745 rows in both modes. The causes
are #1461 (15 rows per member per case) and #1387 E14 (no dense parity row)
and E15 (correctly rounded norm3); see DEL-07-05 Design `owner-mac-check.md`.
W-7 (#1581) then added 12 global end rows per member per case, and the owner's
ruling of 2026-10-25 ("I approve W-7's native-smoke values") set the rows to
milestone 125 and demo 841 in both modes. The C-120 fix (#1620) then removed
the demo's malformed tee and its 4 intensified rows, and the owner's ruling of
2026-10-25 ("Both decisions I approve your recommended options.") set the demo
rows to 837 in both modes.
It also checks:

- the proof hash against its own JCS hash of each file;
- each saved model against the opened document;
- the replace prompt, as named observations. In phase 1:
  - a second New Local Project of the milestone prompts, and Cancel keeps the
    stored project;
  - Save as a new project stores the copy, which differs from the stored
    original in `project.id` alone, and its solve proof carries the copy's own
    hash;
  - the session's own save of the copy saves in place without a prompt;
  - after another writer saves the copy (a smoke-only command that uses the
    product's own persistence functions), the session's next save prompts as a
    moved base, and Cancel keeps the other writer's revision;
  - Replace writes only after confirmation.

  In phase 2, saving the bundled demo over the stored demo prompts too;
- that the exported file in Downloads is bound to the solved model's project,
  shows the row count the export panel showed, and accounts for every row of the
  solve;
- where `expected.json` says a case's export is not offered, the export panel's
  text, and that nothing was written. The milestone is Sensitive and so not
  Current, and offers none. Under K-D5 on the converged active set (DEL-04-01
  `kd5-nonlinear-active-set.md` §5) the demo is `checks_passed` and Current, and
  its result export is offered and written.

**From blank and the output gates (W-4).** Phase 1 then continues in the same
process, and phase 2 reopens what it saved. None of these checks reads a value
from `expected.json`; each is judged by identity, by a closed form or by a
refusal named in the product:

- **The demo's local-private report.** With "Include known private values in
  this local rendered report" selected, the render is unblocked in the
  `local_private` context, shows a canonical HTML SHA-256, passes its export
  gates, and offers the local-private save, with the warning and the
  document's LOCAL PRIVATE notice.
- **Rule checks.** The bundled demo pack runs on the demo and on the
  exact-route model. Both are refused as `RULE_INPUTS_INCOMPLETE`, naming
  `demo_actual_quantity` and `demo_limit_quantity` as missing.
- **Exact-route gates.** It opens
  `fixtures/model_operations/exact_pressure_authoring_model.json` and solves
  it. The proof must carry the file's JCS hash, and the standing must name
  `3.0.0/exact_pressure_v3`. Each refusal is read from the surface's own output
  gate:
  - In the rendered-report panel, the gate shows
    `PRESSURE-2-OUTPUT-NOT-YET-AVAILABLE` and no render control.
  - In the MBF panel, the gate shows the same code, with neither the package
    nor the MBF text download.
  - The result JSON export is offered. The driver waits for the asynchronous
    packet.
- **A model authored from blank.**
  - **Authoring.** New Blank Project must open the units dialog (SI, US
    customary, custom); the default SI is accepted. The driver then authors
    through the GUI a node, a material, a pipe to a new node, a fixed support,
    a load case and a 350 N tip force. It uses the viewport commands, the
    toolkit, the target pickers and Review changes, and applies each queued
    intent. The inputs are in `run.mjs` (`BLANK`).
  - **Solve.** It solves natively. Checked:
    - integrity `checks_passed`;
    - with the filter cleared, the results count every solved row;
    - the deformation overlay is available for both nodes. That is the
      overlay's state, not proof of drawing: no check here or in the browser
      specs inspects the drawn deformed pipe (`rotation-glyph` draws the
      rotation glyphs only);
    - the tip's displacement magnitude and its signed `uy` equal the
      Euler–Bernoulli PL³/3EI from the same inputs within 1e-9 relative
      (DEC-024/026 criterion), and `ux` and `uz` are zero.
  - **Save.** Save Local Project saves in place without a prompt. The stored
    model must hold:
    - the seven unit keys and the authored entities;
    - a support fixing all six DOF at the anchor;
    - the tip force as 350 N along `global_y` at the tip node.

    Its JCS hash must equal the solve proof's.
  - **Reopen.** Phase 2 reopens that project in a fresh process and solves it
    again. The proof must carry the stored model's hash, with the same rows,
    tip value and hash as before the save, and the same signed tip
    deflection.

A broken step stops the driver, the phase reports not completed, and every
later check fails for want of its observation.

Run it on a Mac with a GUI session:

```sh
node apps/desktop/native-smoke/run.mjs   # add --keep to retain the evidence folder
```

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
