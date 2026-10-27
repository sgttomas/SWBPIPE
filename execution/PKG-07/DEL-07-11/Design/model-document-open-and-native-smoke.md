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
    - **When the blank becomes the session.** Create closes the units dialog
      at once. The blank becomes the session only when its native create and
      the hashes of its response return (`createBlankProjectFrom` in
      `workspaceSession.ts`). Until then the previous project stays on screen
      and stays editable.
    - **What supersedes it (W-17, #1640).** Only a change to the model or
      project on screen supersedes the pending blank: a commit through
      `commitModel` (an applied operation, or undo/redo with history) or
      another project operation. The create checks its own model-commit
      epoch for this, not the request epoch. Selecting an entity or typing
      into an unapplied form advances the request epoch but not the
      model-commit epoch, so the blank still installs. Before #1640 either
      one cancelled it silently.
    - **A superseded blank is named (Agent 0, #1640).** The edit wins and is
      never overwritten. The blank is stored but not opened. The status line
      gives its name and id and the way back:
      `NEW-BLANK-SUPERSEDED: the new blank project "<name>" (<id>) was created
      but not opened, because the model changed while it was being created.
      This session keeps the changed model. The blank is stored locally: open
      it from List Local Projects.` The last project operation is recorded as
      `create_blank_superseded`. The stored blank is not deleted, and the
      desktop has no command to delete a stored project. A blank superseded
      by another project operation gets no message: that operation owns the
      status line.
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
  - In the report panel, the report package shows the same code, with no
    package controls (W-18 (b2); owner ruling 10A).
- **The demo's report package (W-18 (b2)).** In phase 1, right after the
  demo's result JSON export and in the same session, the driver selects
  "Include known private values in this local package" (unchecked until then)
  and runs File > Save Report Package…, answering the save chooser outside
  Downloads. A save the native memory pool refuses only for the Mac's
  momentary free memory (nothing runs or is written) is asked again. Checked, with no value pinned: the receipt is `REPORT-PACKAGE-SAVED`;
  the file's SHA-256 and length are the receipt's; the product's reader
  (`read_report_package_container`, through a smoke-only command) re-reads an
  unblocked 2.0.0 desktop package whose identity is the receipt's; every
  member's recomputed SHA-256 and length match the manifest inventory; and the
  result member is the RFC 8785 text of the result JSON the same session
  exported.
- **A model authored from blank.**
  - **Authoring.** New Blank Project must open the units dialog (SI, US
    customary, custom); the default SI is accepted. The blank becomes the
    session only when its native create returns (see New Blank above), and
    until then the tree shows the previous project. In run 2 of the #1634
    smoke at `4847f14b13` the driver read that row as the blank's id and
    authored at once. Its typing into the node form, or its first applied
    edit, superseded the pending create; the authoring and the solve then ran
    on the previous project. Since #1640 only an applied edit supersedes it.
    So before authoring, the driver waits for the create's own status line
    and for a tree whose only project is a new id; that id is the blank's. A
    `Blank create failed` line stops it at once. The driver then authors
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

**Showing a section.** Choosing the page that is already open closes it (View
menu). So the driver sends a View section command only when that section is not
shown, and then waits until it shows. The driver used to send the command
regardless. Two steps then chose Report while it was open: the demo's
local-private render after its package save, and the exact-route render. That
second choice closed the page. Its wait passed only when its first sample ran
before the close was drawn, which depends on whether the menu command's reply or
the page's re-render runs first in the webview. In run 1 of the #1668 smoke at
`594e437a09`, at the exact-route render, the re-render ran first. The wait timed
out after 120 s, the blank was never authored, and phase 2 failed for want of
it. When the wait did pass, the render ran on a closed page, and on the exact
route the package save chose Report a third time to reopen it. `native-smoke/driver.test.mjs` covers both cases, an open
section and a hidden one. Timeouts are unchanged.

The other show helpers follow the same rule, and every show first yields one
task before it samples. React draws a click's update in a microtask, after the
driver's own synchronous code, so a sample taken straight after a click sees
the page as it was.
- Review changes (`showOperations`). Queuing an intent opens Review changes
  itself. In run 1 of the #1679 smoke at `15c1996957`, the from-blank load
  step sampled the Load Cases stage before the queue's navigation was drawn.
  That stage has no Review changes tab, so the driver chose View > Review
  changes, which closed the page that had just opened, and the wait timed out
  after 120 s. The rerun passed because its first sample ran before the
  toggle landed.
  - The driver now sends nothing when the page shows.
  - Otherwise it uses the Model stage's tab, which only opens the page, and
    the menu only where that tab is not shown.
- Closing a page (`closeSection`) waits until its Close control is gone,
  instead of a fixed 200 ms.
- The Inspector toggle (`ensureInspector`) is pressed at most once and waited
  for until open, instead of a fixed 300 ms.

`driver.test.mjs` covers each. No timeout was raised.

A step the window reports as failed stops the driver at once with that
report, rather than waiting out its timeout. One example is a solve whose job
reaches a terminal state other than `completed`: `refused`, `failed` or
`cancelled` (a job still `cancelling` has not ended). A solve that is refused,
fails, is cancelled or does not finish in time is reported with:
- its job's summary, and any queue reason, resource refusal or error the Solve
  panel shows;
- the typed reason (code, required bytes, headroom bytes, pressure);
- the native memory pool's current headroom and pressure, as
  `get_memory_pool_status` reads them.
A memory refusal or queue wait is then told apart from a slow solve. The
details go to the driver log, the phase error and `summary.json`
(`solve_job_diagnostics`). Timeouts and expected values are unchanged.
`native-smoke/driver.test.mjs` tests this behaviour, and the New Blank wait
above, with a fake window (`node --test`); no CI job runs it.

The smoke checks each WebAssembly engine against a content stamp before it
runs. `build:wasm` writes `source-stamp.json` into each engine folder: a
SHA-256 over the engine crate's closure (normal and build path dependencies:
manifests, build scripts, `src/`), its lock file, the build script, the compile
reads cargo's dep-info lists (schemas and fixtures included or digested at
build time, by the path cargo reported, with every symbolic link on that path
and its target), the bytes of the cargo configuration files, `rustc -vV` of the
rustc cargo runs, and the compiler, linker, rustflag and profile environment
(`RUSTC`, `RUSTFLAGS`, `CARGO_BUILD_*`, `CARGO_TARGET_<triple>_*`,
`CARGO_PROFILE_*` and similar). The cargo configuration files' keys are
decoded as TOML (bare, basic-string with every escape, literal-string, dotted,
in table headers and inline tables). Refused by name: a configuration file with
a `rustc`, `rustc-wrapper` or `rustc-workspace-wrapper` key anywhere, a
`build.build-dir` or `build.target-dir` key, or an `include` key; one the
scanner cannot tokenize with confidence; a compile read whose path has a
`..` that cancels a symbolic link; and, before any cargo runs (the engine build
and stamp, the manifest reads and the smoke's app build and clean), a non-empty
`RUSTC_WRAPPER`, `RUSTC_WORKSPACE_WRAPPER`, `CARGO_BUILD_RUSTC_WRAPPER` or
`CARGO_BUILD_RUSTC_WORKSPACE_WRAPPER` in the environment. Our wrappers never
set one; an inherited wrapper would run unseen for every compile, and one that
calls `t3_slot.sh` would take a host slot under the worktree lock. The app
build's cargo configuration is checked the same way, from `src-tauri`. A `[target.<triple>]` `linker`, `runner` or
`rustflags` key is not refused: the file's bytes are in the stamp. Each engine
builds in its own target directory, `apps/desktop/target/wasm-engines/<engine>`:
every cargo that builds, cleans or reports the target directory (the engine
build and its `cargo metadata`, the app build and the app's `cargo clean -p`;
the manifest-reading `cargo metadata --no-deps` writes into neither) has `CARGO_TARGET_DIR`,
`CARGO_BUILD_TARGET_DIR` and `CARGO_BUILD_BUILD_DIR` set to its dedicated
directory, so an inherited build directory can neither keep intermediate
artifacts outside it nor be cleaned. That directory, or one cargo descends into
beneath it (each entry one and two levels down, such as `debug`, `debug/deps`
or `wasm32-unknown-unknown/release`, and the `deps`, `build`, `.fingerprint`,
`incremental` and `examples` of each `<triple>/<profile>`), being a symbolic
link or resolving outside it is refused by name before each cargo run and
before the directory is deleted.
An engine whose published stamp equals its current inputs is not built at
all: nothing is compiled and nothing is republished. Any build, for whatever
reason (a missing or different stamp, inputs that moved during an attempt),
first deletes that directory, so every build is clean: neither an edit that
kept a file's modification time, nor a link retargeted to an older file, nor
an ordinary cargo build run in that directory since (of other sources, later
restored with their old times) leaves cargo's mtime-based freshness check an
artifact to publish under the new stamp. A clean build of both engines costs
about a minute (58 s measured for both together on the development host). The
stamp is written only when
the complete input set, digested before cargo runs and again after, held
still; a newly revealed compile read or a changed input means another clean
build, and after three the build fails naming the inputs. `run.mjs` recomputes
each stamp and runs `build:wasm` when a folder or stamp is missing or the hash
differs, also under `--skip-build`. It logs `engine rebuilt: <engine>
(<reason>; stamp <hash>)` or `engine current, stamp <hash>`.

One per-worktree build lock, `apps/desktop/target/build.lock`
(`scripts/worktreeBuildLock.mjs`), is held for the whole engine check, build,
glue and publication of `build:wasm` (a direct `npm run build:wasm` takes it)
and by `run.mjs` from its engine check through the app build, binding and
both app launches: the smoke releases it only after its last launch ends, on
every exit path; `build:wasm` under the smoke inherits it through the token
the lock records. So two builds or smokes in one worktree never interleave, and
the `.tmp-`/`.old-` cleanup never removes another active run's directories.
One build or smoke per worktree at a time is the supported use; a second is
refused, not queued. The lock is created
atomically (a complete record hard-linked into place) with the holder's pid,
the start time `ps` reports for it (in UTC and the C locale), its purpose, a
token and when it was acquired. It fails closed: no lock is ever broken
automatically, since neither a pid's absence nor its start time shows that no
build process of the holder survives. A taker that finds the lock held refuses
at once, naming the holder's pid, purpose, recorded start and acquired times,
with a liveness hint from `ps` (informational; it removes nothing) and the
exact removal step, so that a crashed run costs a minute, not a diagnosis:
`TZ=UTC ps -p <pid> -o pid,lstart,command` for the holder and
`pgrep -fl -- '<project directory>'` for any cargo, rustc, wasm-bindgen,
build-script or app process whose command line names this worktree's project
directory (rustc carries it in `--out-dir`, and every cargo build the tooling
runs names a manifest in it), then `rm -f '<lock path>'`, each with the
absolute paths quoted for zsh and bash. Lock order: the host slot first
(`t3_slot.sh`, `t3_cargo.sh` or `t3_exclusive.sh`, taken by the wrapper that
launches the build or smoke), then the worktree lock; code holding the
worktree lock never acquires a host slot, and a command that resolves to one of
those wrappers, or to a script naming them, is refused by name, as is an
inherited compiler wrapper.

Interruption. The holder installs its exit and stop-signal handlers before
it publishes the lock, so from the moment the lock exists an exit or a stop
signal releases it; an acquisition that fails stops listening without losing
a signal already received. While it holds its own lock, the holder runs every
build command, and the smoke each app launch, through `runUnderLock`:
asynchronously and as the leader of its own process group (`detached`), so
cargo's rustc, build-script and wasm-bindgen descendants, or caffeinate and the
app, are in that group. A command under an inherited lock (`build:wasm` under
the smoke) stays in its parent's group, which the holder signals. Each
command's group is settled before the command returns and the next one starts:
once its leader exits, the group gets up to 2 s to empty, then SIGTERM and up
to 2 s, then SIGKILL and up to 2 s. A group seen empty is forgotten at once,
and only a group still tracked and seen alive just before is ever signalled,
so no signal reaches a group id the system has since reused. A group that
cannot be confirmed empty stops the run (exit 1), keeping the lock with the
removal steps. On SIGINT, SIGTERM or SIGHUP the holder sends that signal to
each of its groups and waits up to 5 s for every group to be empty, sends
SIGKILL to any group that is not and waits up to 2 s more, and only when every
group is gone releases its lock; it then exits 128 + the signal number (130,
143, 129), the smoke removing its private app copy as it exits. Once it is
handling a stop signal the handler alone escalates: a command's own settlement
sends nothing more, so a descendant still cleaning up keeps the full 5 s. A group still
not empty after SIGKILL (on macOS a group left with only an unreaped zombie
counts) keeps the lock and the copy, and the holder prints the same
confirmation and removal steps as the refusal. That text is written
synchronously, its first line saying the lock is kept, and retried while the
pipe is full for at most 2 s: delivery is best effort, and a reader that does
not drain standard error within 2 s gets only the head of the text, or
nothing, while the lock is kept regardless. A signal that arrives during
synchronous work (the probes `ps`, `rustc -vV` and `cargo metadata`, or the
work after a command returns) is delivered in the event loop's next poll
phase. The tooling waits for one before deciding an exit or removing its
handlers (`afterPendingSignals`: a timer, which fires in a later loop
iteration, then an immediate, which runs after that iteration's poll phase).
`exitAfterSignals`, which the build and the smoke exit through, waits so before
it exits; a release keeps the handlers until then; and a successful
`build:wasm` waits so after its release. A signal received before that wait is
then honoured: it exits 128 + the signal number. An ordinary or nonzero exit releases the lock.
An uncaught error releases it too; were a group still running at that point,
it is sent SIGKILL and the lock is kept. A signal pending at an uncaught error
is not honoured: the exit status is the error's. Limits: SIGKILL of the
holder, or a power loss, cannot be handled. They leave the lock, any surviving build
process, and for the smoke its private app copy; recovery is the printed steps
below. A descendant that leaves its process group is not followed; the printed
`pgrep` step is what finds it.

The app embeds `dist/` when cargo builds it. `run.mjs` builds it with
the target and build directories set to `src-tauri/target` and
`--message-format=json-render-diagnostics`, and binds and launches only the
executable cargo's `compiler-artifact` message reports, which must lie in that
directory; no report, or one elsewhere, stops the run by name before anything is
bound or launched. After a build whose inputs held still it writes a binding
record, `apps/desktop/target/native-smoke/app-binding.json`: the engine stamps;
a hash of the frontend inputs (`src/`, the app's `index.html`, `package.json`,
Vite, TS, PostCSS and `.env` files, `public/` other than the engine folders, the
npm workspace's `package.json` and `package-lock.json`, and each file outside
`src/` a source names in a relative import, such as the project fixtures); a
digest of `dist/`; a hash, with the stamp's toolchain context, of the app's Rust
inputs (the `src-tauri` crate closure by the same cargo-metadata method, every
file at the top of `src-tauri/` including `tauri.conf.json` and `build.rs`,
`icons/`, `capabilities/`, and the dep-info compile reads, which are recorded);
the build command and smoke configuration; and the executable's path, size and
SHA-256. Before every app build the smoke performs, `cargo clean -p` removes the
closure's own packages from `src-tauri/target`, so the build is clean for them:
neither a valid record nor matching Rust inputs say what the directory holds,
since an ordinary cargo build after the binding (of other sources, later
restored with their old times) leaves artifacts cargo would take as fresh. `dist/` changing while cargo builds, or
the frontend inputs changing while Vite builds, stops the run by name with no
binding and no launch; a changed or newly revealed Rust input means another
build, up to three. Under the lock, the bound executable is copied into a
private per-run directory, `apps/desktop/target/native-smoke/run-<token>/`
(created exclusively, about 115 MB), and the copy's SHA-256 must equal the
bound binary's; each launch runs the copy, its identity checked again first,
and the copy is removed when the run ends, normally or by SIGINT, SIGTERM or
SIGHUP. No other build in the worktree runs while the smoke launches, and none
could change what it launches. A hard-killed run (SIGKILL, power loss) leaves
that directory, an accepted cost. The refusal the next run prints names it by
the lock's token, after the lock's removal step: once the lock is removed and
nothing runs from that directory (the printed `pgrep` shows no process from
it), `rm -rf '<that directory>'` removes it. Recovery removes only that named,
inactive directory, never every `run-*`. The app needs nothing beside its
executable: `dist/` is embedded, it links only system libraries, and its solve
workers start the same copy (`current_exe`). Under `--skip-build` the frontend and app are rebuilt when an engine was
rebuilt, a `dist/` engine copy's stamp differs, the record is missing or
unreadable, or the frontend hash or `dist/` digest differs; the app alone when
the engine stamps, the Rust inputs, the build or the executable differ. The log
names which applies.

What this guards against. In scope, and to be closed: accidental staleness or
mis-binding in our own environments: ordinary edits (including ones that keep a
file's modification time), branch switches, reused worktrees, the environment
our wrappers set and the smoke inherits (`CARGO_TARGET_DIR`,
`CARGO_BUILD_BUILD_DIR` and the like) and shared or linked target directories.
Out of scope, documented limits: local cargo configuration or filesystem tricks
deliberately constructed to defeat the check. The refusal of compiler and
output keys in any TOML spelling, and of files the scanner cannot tokenize,
also stops such constructions where that was cheap; these limits remain: the
contents of a linker or runner program (only its name; compiler wrappers are
refused); a file swapped between
the check and cargo's read, or a link created during a build; and cargo
configuration read from places other than the working directory's ancestry and
`CARGO_HOME` (command-line `--config`, which the smoke does not pass).

Not covered: files outside the project (registry crates and npm packages beyond
their lock files, the Rust toolchain beyond `rustc -vV`, system SDKs). The
frontend binding's textual import scan sees every import in production sources;
the only computed URLs there load the engine glue from `public/`, which the
engine stamps cover. It does not see test-only inputs: the `import.meta.glob`
in `src/features/solve/LoadReferenceStatesBlock.test.tsx`,
`src/features/workspace/resultsSessionState.test.ts`,
`src/features/results/pressureSourceIntegration.test.tsx` and
`src/features/results/PhysicsSourceEvidencePanel.test.tsx`, and the computed
generator path in `src/design/tokens.test.ts`. Those are not in `dist/` and are
not bound. `native-smoke/engine-stamp.test.mjs`,
`native-smoke/app-binding.test.mjs` and `scripts/build-wasm-engine.test.mjs`
test the stamp, the clean and stable-build rules and these decisions.

Run it on a Mac with a GUI session:

```sh
node apps/desktop/native-smoke/run.mjs   # add --keep to retain the evidence folder
```

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
