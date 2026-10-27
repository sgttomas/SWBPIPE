# On-demand model checks

The desktop runs the model's existing checks when the user asks, without
solving, and lists their findings. Choosing a finding selects the entity it
names. Paths are relative to `projects/chirality-piping/`. Standard claim fence
applies (F-PIP-2; claims taxonomy per DEC-081).

This extends W-8 ([model-topology-checks.md](model-topology-checks.md)). Its
warnings, and the solve's other input checks, used to appear only after a
solve. The user can now see them while modelling. The checks serve PRD
FR-KNOW-002 and FR-GUI-006. No check rule is added, and no number, threshold
or refusal changes.

## The checks

| Group | What runs | Owner |
|---|---|---|
| Solve input, restraint and topology checks | The solve's own checks before it forms the first stiffness, in the solve's order and with its stops: request parse and closed records; input validation; applied self weight, shared sections, unit normalization and the base material pair; load-state resolution; the model build; constant-effort consumption; the W-8 topology checks; and the restraint pre-checks (no support, fewer than six ground constraints, invalid support input) | `core/product_physics/src/model_checks.rs` (`preview_model_checks_value`) |
| Model-data slot checks | The W-13 document validator | `core/model_operations/operation_applier/src/document_validator.rs`, through `modelDocumentDiagnostics` |
| Library references | The W-13 library-reference findings the store published for this model | `features/library/libraryRefFindings.ts` |

**One sequence, held to the solve's.** `preview_model_checks_value` takes the
desktop's solve request (`{"model": …, "materials": []}`). It calls the
functions that `run_linear_static_preview_observed_inner` calls, with the same
arguments, and stops where the solve stops at a blocking diagnostic. The only
copies are the diagnostics that the solve builds inline: document kind, schema
version and the three restraint pre-checks. The work after the first stiffness
formation does not run here: the dense-scrutiny size guard, assembly, the
solve itself and its numerical-integrity diagnostics. Nor does the
retained-memory observer.

The module is declared at the end of `core/product_physics/src/lib.rs`, so no
existing line moves, and nothing on the solve or retained route calls it.

The drift guard is `core/product_physics/tests/model_checks_runtime.rs`. It
solves each of 14 invented models in both solver modes, including the bundled
demo, the refused demo and models built to raise each check family. For each,
the checks' findings must equal the leading diagnostics of the solve's envelope
(id, code, severity, source and affected refs). A branch warning's message may
gain the applied-sides statement once rows are published. If a finding blocks,
the solve must publish exactly those findings. A request that the solve
refuses outright is refused with the same text.

## Desktop

- **Engine.** The wasm engine exports `model_checks_json`
  (`operation_applier/src/model_checks.rs`). It returns
  `{document_kind: "openpipestress.desktop.model_checks", checked: true, diagnostics}`,
  or `{checked: false, refusal}` with the solve's own refusal text. The same
  engine runs in the browser preview and the native app, so the checks need no
  native command and take no memory-pool admission. They read a JSON copy of
  the model.
- **Command.** Analyze → **Check Model**, with the keyboard equivalent ⇧⌘M
  (Shift+Ctrl+M off macOS):
  - in-app menu: `analyze.check-model`;
  - native menu: an item with accelerator `CmdOrCtrl+Shift+M`;
  - browser preview: the webview's shell key handler.

  The command runs `runModelChecks` (`features/diagnostics/modelChecks.ts`) on
  the session model and opens the Issues drawer.
- **List.** `features/diagnostics/ModelChecksPanel.tsx`, at the top of the
  Issues drawer:
  - findings are grouped by check, each with its own code, severity and
    message;
  - the summary is a status region, counting blocking or error findings,
    warnings and info, and a new run moves keyboard focus to it;
  - the findings are buttons in the tab order, and the arrow keys, Home and
    End move between them;
  - choosing one selects the first affected reference that names a node,
    pipe, support, component or load case (`resolveEntitySelection`, as the
    Diagnostics panel does). A finding that names none (for example
    `SUPPORT_INPUT_MISSING`, whose reference is `supports`) is
    `aria-disabled` and selects nothing.
  - **Run model checks again** in the panel reruns them.
- **Never a change.** The checks change neither the model nor the undo history.
  A run keeps the model value it read. After an edit, the panel says the list is
  out of date until the checks run again. When the solve cannot read the model,
  or the engine is not loaded, the panel says that the solve's checks did not
  run, and gives the reason.

## Evidence

- `core/product_physics/tests/model_checks_runtime.rs`: the drift guard above.
  It also checks that the request is unchanged, that a second run gives the
  same findings, and that the corpus raises each family: disconnected parts,
  coincident and near-duplicate nodes, an invalid tee, no support, fewer than
  six ground constraints, an unknown pipe end, a zero-length pipe, an unknown
  support family and an invalid tolerance.
- `core/model_operations/operation_applier/tests/model_checks.rs`: the engine
  report carries the solve's leading diagnostics, leaves the model alone, and
  reports an unreadable model unchecked with the solve's refusal.
- `apps/desktop/src/features/diagnostics/modelChecks.test.ts`, through the
  built wasm engine:
  - the topology warning, linked to its node;
  - the restraint pre-check, with no entity;
  - an unknown pipe end, linked to the pipe;
  - a refused model;
  - in each case, the model unchanged.
- `apps/desktop/src/features/diagnostics/ModelChecksPanel.test.tsx`:
  - grouping, codes and text;
  - selection, and no selection for a finding that names no entity;
  - arrow, Home and End navigation;
  - focus moving to a new run's outcome once;
  - the out-of-date note and the not-run statement.
- `apps/desktop/src/App.modelChecks.test.tsx`, in the full shell:
  - the menu item shows ⇧⌘M;
  - ⇧⌘M runs the checks and opens the Issues drawer with focus on the
    outcome;
  - the topology finding selects its node;
  - undo is unchanged;
  - the native menu command runs the checks too.
