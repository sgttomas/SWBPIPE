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
| Solve input, restraint and topology checks | The solve's own checks before it forms the first stiffness, in the solve's order and with its stops: request parse and closed records; input validation; applied self weight, shared sections, unit normalization and the base material pair; load-state resolution; the model build; constant-effort consumption; the W-8 topology checks; and the restraint pre-checks (no support, fewer than six ground constraints, invalid support input) | `core/product_physics/src/model_settings/model_checks.rs` (`model_settings::model_checks::preview_model_checks_value`) |
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

The module is declared from `model_settings.rs`, a public module of the crate
that is not a retained-pricing owner source, so `lib.rs` and every other owner
source stay byte-identical; it reaches the crate root's private checks as any
module of the crate does. Nothing on the solve or retained route calls it. The
retained-pricing tool prices the head with exit 0, and its tree compares
`a_le_b` and identical to main's but for the basis line.

The drift guard is `core/product_physics/tests/model_checks_runtime.rs`. It
solves each of 15 invented models in both solver modes, including the bundled
demo, the refused demo and models built to raise each check family. For each,
the checks' findings must match the leading diagnostics of the solve's envelope
in id, code, severity, source and affected references. Each finding's message
is the solve diagnostic's message before any post-solve statement: on a
solved envelope the branch warning (`BRANCH_TOPOLOGY_INVALID`) gains the
statement of the branch stress factors still applied, which the checks, run
before any rows exist, never carry; so each check message is a prefix of the
solve's. If a finding blocks, the solve must publish exactly those findings. A
request that the solve refuses outright is refused with the same text. One
case (Codex review r1 of #1665) combines an unknown support family, which
validation refuses, with a stale generated self-weight load, which the next
stage would refuse: the checks, like the solve, list the first and not
`SELF_WEIGHT_INPUTS_STALE`, and removing that stop fails the guard.

**After the 0.8.0 ripple (#1649).** When #1649 merges, its
`append_eccentric_offset_refusals` call joins the solve's sequence; the checks
add it at the same place in `pre_formation_checks`, and the corpus's
hard-coded model-version fixtures follow the 0.8.0 documents.

## Desktop

- **Report.** `operation_applier/src/model_checks.rs` returns
  `{document_kind: "openpipestress.desktop.model_checks", checked: true, diagnostics, diagnostics_not_shown}`,
  or `{checked: false, refusal}` with the solve's own refusal text. The report
  is bounded whatever the model: at most 1 MiB of serialized findings in the
  checks' order, each text cut at 2 KiB on a character boundary (the finding
  is then marked `truncated`), at most 16 references per finding, and a count
  of the findings left out; a refusal's text is cut the same way. The panel
  states how many were left out and marks shortened text.
- **Memory (DEL-04-07).** Model-sized work follows the memory-budget decision
  (`memory-budget-decision.md`, "On-demand model checks"):
  - in the app, the checks run in the native host as `check_preview_model`, a
    raw `args-json-v1` command admitted and leased by the memory pool before
    it parses the model, as every model-sized interactive command is. The
    lease grows by 8 M for the check's working memory before it runs; a
    refused admission or growth leaves the solve's checks not run, and the
    panel names the refusal;
  - in the browser preview, which has no pool, the wasm engine's
    `model_checks_json` checks a model text of at most 4 MiB and refuses a
    longer one `MODEL-CHECKS-INPUT-TOO-LARGE` before parsing it; the desktop
    refuses it before copying it into the engine.
- **Command.** Analyze → **Check Model**, with the keyboard equivalent ⇧⌘M
  (Shift+Ctrl+M off macOS):
  - in-app menu: `analyze.check-model`;
  - native menu: an item with accelerator `CmdOrCtrl+Shift+M`;
  - browser preview: the webview's shell key handler.

  The command opens the Issues drawer and runs `runModelChecks`
  (`features/diagnostics/modelChecks.ts`) on the session model; while the
  native check runs, the panel's button reads **Running model checks** and is
  `aria-disabled`. A later run or a new project session supersedes a run still
  in progress.
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
  support family and an invalid tolerance. `the_checks_stop_where_the_solve_stops`
  holds the intermediate stop.
- `core/model_operations/operation_applier/tests/model_checks.rs`: the report
  carries the solve's leading diagnostics, leaves the model alone, and reports
  an unreadable model unchecked with the solve's refusal.
  `the_report_is_bounded_whatever_the_model`: 4,000 long-id supports give
  11,296,671 bytes of findings unbounded and a 1,046,747-byte report (371
  shown, 3,629 counted); a 1.2-million-character string refusal is cut to
  2 KiB; the browser bound refuses one byte over.
- `apps/desktop/src-tauri/src/lib.rs`: `model_checks_stay_within_their_lease`
  (the whole command's priced peak within its grown lease for four invented
  models; figures in `memory-budget-decision.md`; the native report equals
  the applier's) and
  `model_checks_refuse_by_type_when_their_working_memory_is_not_admitted`.
- `apps/desktop/src/features/diagnostics/modelChecks.test.ts`, through the
  built wasm engine:
  - the topology warning, linked to its node;
  - the restraint pre-check, with no entity;
  - an unknown pipe end, linked to the pipe;
  - a refused model;
  - in each case, the model unchanged;
  - in the app, the native command with the raw ingress body and never the
    wasm engine, and a memory-pool refusal named;
  - in the browser, a model text over the bound refused by name without
    reaching the engine.
- `apps/desktop/src/features/diagnostics/ModelChecksPanel.test.tsx`:
  - grouping, codes and text;
  - selection, and no selection for a finding that names no entity;
  - arrow, Home and End navigation;
  - focus moving to a new run's outcome once;
  - the out-of-date note and the not-run statement;
  - the left-out count, shortened text and a run in progress.
- `apps/desktop/src/App.modelChecks.test.tsx`, in the full shell:
  - the menu item shows ⇧⌘M;
  - ⇧⌘M runs the checks and opens the Issues drawer with focus on the
    outcome;
  - the topology finding selects its node;
  - undo is unchanged;
  - the native menu command runs the checks too.
