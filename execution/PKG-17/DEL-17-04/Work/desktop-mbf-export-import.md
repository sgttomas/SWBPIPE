# Desktop MBF export and import (RT-R7, RT-R8)

**Current verification limit.** RT-R8's "connected" status is source/test
evidence. No current-source real App walkthrough has been performed in this
work. The next selected integration check must exercise that actual App path;
this note supplies neither a reference-system round trip nor criterion-6
qualification. The remaining refusals in the import Design still stand.

**Basis:** `main` at `153485db8c`. The SI writer `core/handoff/caepipe_mbf/si_writer.py`
(#1652, profile `caepipe-mbf-si-pilot-1`), the importer `importer.py`, tranches 1–3 (#1670,
#1685, #1697), the authored cases (#1702), and the import Design `caepipe-mbf-import.md`
(cited here as "the import Design"). Release target `docs/RELEASE_TARGET_v1.0.md`: rows RT-R7
and RT-R8 (Must) and exit criterion 6. File names follow DEL-07-01 `Work/export-file-names.md`
(#1711).
Slice 5's design (§4, §4.8, 2026-11-07) reads source on `main` at `b06e1da32`. Its implementation (§4.9) is on the
slice-5 PR, from `main` at `394f4c623`.

This design was drafted before implementation for Agent 0 and independent review. The implemented desktop export and import scope is recorded in the ScopeOfWork and §4.9; the remaining coverage limits are recorded in §1.4 and §5. Risk class:
persistence, file writes and numerical meaning. Standard claim fence applies (claims taxonomy
per DEC-081). Nothing here records or implies acceptance, issuance or release, which remain
the owner's acts (F-PIP-3).

## 0. Locality and rulings

This design sits beside the import Design in DEL-17-04, for two reasons:

- the release target names DEL-17-04 as the owner of both rows;
- every decision here rests on the contracts of the writer and the importer.

At the time of drafting, two texts placed the desktop part elsewhere:

- **DEL-17-04.** CLM-008 says the deliverable "shall not implement … GUI behavior". CLM-012
  says "A desktop MBF import action would belong to DEL-07-12 and is not part of this
  deliverable."
- **DEL-07-12.** It owns `apps/desktop/src/features/caepipe-mbf/**`. Its finite inventory
  lists ".mbf model export" but no import. It has no `Work/` locality.

This design therefore keeps the panel and dialog code under DEL-07-12's paths. The engine,
the native MBF file logic and the criterion-6 check stay with DEL-17-04. The ScopeOfWork text
is settled by A-1 (§8.1), with the exact Old/New proposals in §8.3.

**Rulings applied.**

- **Owner, 2026-10-26 (both ways):** <!-- owner-words -->"I want to expand to go both ways.
  That would make any kind of iteration between the two much more effective."<!-- /owner-words -->
- **Owner, 2026-10-26 (public text):** <!-- owner-words -->"Don't state facts about CAEPIPE in
  public text."<!-- /owner-words --> The scope is "New facts only". Product text stays
  generic, as the import Design §0 applies it.
- **Owner, 2026-10-26 (units):** <!-- owner-words -->"units conversions that are routinely done
  in engineering applications are permissible as a solution."<!-- /owner-words --> The writer
  and importer use only exact definitional conversions (import Design §5).
- **Owner, 2026-10-28:** the release target ("Accepted."), including RT-R7, RT-R8 and
  criterion 6.
- **Agent 0, 2026-10-28:** one file-name convention (#1711). The MBF export uses kind `model`.
- **Agent 0, 2026-10-28 (§8.1 decisions):** A-1 to A-6 and O-1 are decided as §8.1 records.
  The §8.3 texts await one independent review and Agent 0's confirmation; slices 4 and 5
  wait for that, slices 2 and 3 do not.
- **CAEPIPE lane, 2026-10-28 (§8.2 questions):** Q-1 to Q-6 are confirmed as recommended, with
  the details §8.2 records. Its answers reach this design through Agent 0.

## 1. Scope and the gap

### 1.1 What the headless code does

- **Writer.** `render_caepipe_mbf_si(model, *, title, reference_temperature_c=20.0,
  max_layout_rows=20, reference_temperature_f=None)` returns an `MbfSiResult`.
  - The result holds `status` (`written` or `refused`), `text`, `diagnostics`, `loss_report`,
    `assumptions` and the id maps. `sidecar(mbf_sha256)` returns the sidecar, format 3 with
    its self-digest.
  - Every refusal is an `unsupported` loss entry, and any refusal refuses the whole file, so
    `text` is `None`. `omitted` entries (combinations, `model_settings`, empty cases) are
    warnings.
  - The writer runs the importer over its own text as a preflight
    (`MBF-SI-PREFLIGHT-REFUSED`).
  - Units are SI, scaled by powers of ten. US-customary units are normalized exactly
    (`MBF-SI-UNITS-NORMALIZED`). Every other unit is refused by name.
- **Importer.** `import_caepipe_mbf(data, *, sidecar, material_provenance, pressure_closure,
  rebind_by_node_number, source_label)` returns a model document of the current version, an
  import report (`swbpipe.caepipe_mbf_import_report`) and an id map.
  - An import is **bound** when the sidecar's `mbf_sha256` equals the hash of the bytes. Every
    other import is unbound. A sidecar for other bytes is not applied
    (`MBF-IMPORT-SIDECAR-NOT-APPLIED`, info).
  - An unbound import of writer output refuses `L=` with `MBF-IMPORT-TBD-17-04-007`, except
    where profile decision PD-1 admits it.
  - Material values need affirmative provenance (import Design §7.4).
- **Checks.** `tests/test_caepipe_mbf_si_writer.py`, `test_caepipe_mbf_importer.py`,
  `test_caepipe_mbf_round_trip.py` (reversible and byte-reproducible subsets, import Design
  §18.2), the private regression `test_caepipe_private_mbf_import.py`, and the data-boundary
  guard.

### 1.2 What the desktop lacks

- **RT-R7.** The Exports section's `CaepipeMbfExportPanel.tsx` builds its own package and
  `.mbf` text in TypeScript (`openpipestress.technical_preview.caepipe_mbf_export_package`)
  and does not call the writer.
  - It offers anchor links on route `DOTH-FORMAT-003` (`downstream_tool`).
  - The native `save_local_result_json` admits only the stress-neutral JSON name on that
    route, so the desktop withholds the MBF text.
  - Its suggested names carry the project id (`openpipestress-preview-<project id>.mbf`),
    which #1711 retires.
  - It writes no sidecar, so nothing it wrote could come back with its identities.
- **RT-R8.** There is no import action, native command or chooser.
- **The engine.** The writer and importer are Python, and the desktop runs no Python.
  - Native code is Rust. The webview's engine is Rust compiled to wasm
    (`apps/desktop/scripts/build-wasm-engine.mjs`).
  - Neither function can be reached from the desktop today (§2).
- **Coverage.** The writer's pilot subset leaves most of the reference system's families
  refused or omitted (§1.4). The desktop surface alone cannot meet criterion 6.

### 1.3 Criterion 6 as checks

The criterion's two clauses are "no Must family omitted or unsupported in the loss report"
and "imports it back with the same entity identities". They map to these checks:

| Check | Pass condition |
|---|---|
| C6-1 | The engine's writer returns `status: written` for the model, with the desktop's export settings (§3.1). |
| C6-2 | No `loss_report` entry with category `omitted` or `unsupported` names an entity of a Must family in the A-3 table. `approximated` cannot occur (REQ-003). |
| C6-3 | The two files on disk are the engine's bytes: their read-back SHA-256 values equal the reviewed `mbf_sha256` and the sidecar's digest (§3.6). |
| C6-4 | Importing the saved pair is bound, verifies the sidecar self-digest, returns `status: imported` and has no refusal. |
| C6-5 | For every entity class in A-3's identity set, the imported ids equal the exported ids, and the report lists no unrecoverable identity (import Design §18.1(b)). |
| C6-6 | The exported and imported models agree under the canonical comparison within the import Design §18.2 tolerances. |
| C6-7 | C6-1 to C6-6 run through the desktop's own engine: the native command and the wasm build, not the Python harness. |

*Note on C6-7 (Agent 0's 2026-11-02 decisions, §3.9.5):* for export, the desktop's own engine is
the native commands only; the webview runs no writer. For import (slice 5's design, §4.8), the
engine is worker task 5 in the native capped worker, and the webview's wasm build runs only the
document validator at Adopt (`checkNativeOutcome`). C6-7's text is unchanged.

### 1.4 Coverage against the reference system

Criterion 1 lists the reference system's contents. On `main` today:

| Family (criterion 1) | Row | Writer | Importer |
|---|---|---|---|
| Straight pipe; temperature-independent material; section | RT-G1, RT-G3 | written | read |
| Temperature-dependent modulus | RT-G3 | refused (`MBF-SI-TEMPERATURE-DEPENDENT-MATERIAL-UNSUPPORTED`). D-2 Route 1 is designed, not implemented (import Design §20.4), with constant α and one thermal case. It applies the per-member rule: each member's modulus selection is at its MBF T1, and the row there states the selected E exactly. The one bounded exception is `explicit_base_properties` on a point-free material, under decision 3's conditions. Derived T1 rows are admitted only exactly (option (a), Agent 0, 2026-11-02). Route 1 waits on G-1 and G-4, and CQ-17-01-016 stays open | refused (O-7 ruling). Once Route 1 is implemented, bound imports with sidecar format 5 restore its inputs exactly. Every bound format-5 import, point-free materials included, needs a source the user explicitly designates before the hash and content checks: a saved state the user chooses or a model document the user supplies. The sidecar's source digest only verifies it and never selects it. Otherwise such an import is refused (`MBF-IMPORT-SOURCE-UNASSOCIATED`), and a derived row is stripped only when that source proves it generated. A trusted export ledger is proposed, not adopted. Unbound imports stay refused |
| Realized bend with user k | RT-K1 | written | read |
| User SIFs | RT-K1 | a realized bend's directional pair is written as the bend comment `BSIF` (#1730); the scalar `sif_user_value` is not written | read: `BSIF` on a bend row restores the pair |
| Tee with user modifiers | RT-K2 | refused (`MBF-SI-COMPONENT-UNSUPPORTED`) | not read |
| Eccentric reducer, valves, flanges, rigid items, weight at CoG | RT-K3 | refused | later (§20.3) |
| Expansion joint with tie rods | RT-K4 | refused | not read; not in §20.3 |
| Pressure (exact route) | RT-L1 | written | read |
| Anchor, guide, line stop, vertical support; single-translation spring | RT-S1 | written | read |
| Gap, lift-off, friction | RT-S2 | refused | refused; needs the operating case (§20.3 ruling) |
| Variable spring | RT-S3 | as a user-stiffness spring only; a `hanger` member is refused | spring read; hangers refused |
| Constant-effort support | RT-S3 | refused | refused |
| Weight, one thermal case, forces, moments, anchor displacement | RT-L1, RT-A10 | written: at most one sustained-type and one thermal-type case | read |
| Further thermal and operating cases | RT-L1, RT-G5 | refused (`MBF-SI-LOAD-CASE-MAPPING-UNSUPPORTED`) | not read |
| Wind; three-axis static seismic | RT-A9 | refused (`equivalent_static` with `MBF-SI-LOAD-CASE-FEATURE-UNSUPPORTED`; other load categories with `MBF-SI-LOAD-UNSUPPORTED`) | not read |
| Hydrotest | RT-A6 | refused (`MBF-SI-PRESSURE-UNSUPPORTED`) | not read |
| Combinations, including SRSS and absolute sum | RT-A4 | omitted (`MBF-SI-COMBINATION-NOT-EXPORTED`) | none created |
| Modal, response spectrum, hanger sizing | RT-A7, RT-A8, RT-A11 | no mapping; `model_settings` is omitted | not read |

**MBF pending ledger: temperature-dependent modulus (D-2 Route 1, import Design §20.4).**
Route 1 alone does not export the reference system. It is thermal only. Each dependency below is
recorded so that no tranche is assumed to close another. The rulings are Agent 0's: D-2,
attributed to 2026-10-31, and Route 1 with its refinements and decisions of 2026-11-02. All are
paraphrased in the import Design's O-7, decision 3 included. D-2's historical date and wording
are unverified against the original messages; Agent 0's confirmation of its design intent
(2026-11-02) is the current basis. Nothing here completes criterion 6 or closes a dossier
question.

- **The per-member rule.** Every exported member selects its modulus at its mapped MBF T1, and
  the MBF row at T1 states exactly the E the solver selects. The one bounded exception is an
  `explicit_base_properties` selection on a material with no temperature points. It is eligible
  only when the selected E basis is constant and exactly representable in the MBF, with ν, α
  and the law checked separately (decision 3). It is not an unconditional temperature rule.
- **The import's designated source.** Every bound format-5 import, point-free materials
  included, needs a source S that the user explicitly designates, independently of the sidecar
  and before any check: a saved state the user chooses, or a model document the user supplies
  through the Open Model Document route's checks. S is then verified by hash and then by
  content before anything is restored or stripped (import Design §20.4.4 and §20.4.5). The
  sidecar's `source_model_sha256` only verifies S; the import never searches by it, and the
  state list is not filtered, ranked or preselected by it. A cross-machine import needs the
  exact source to be supplied. The import review names the source used. Without one, such an
  import is refused (`MBF-IMPORT-SOURCE-UNASSOCIATED`).
  - **Export review notice.** Export MBF keeps no record of its source. The complete review of
    a Route 1 export states that a later bound import requires the user to designate the
    exported source, that the user must have access to that exact source, and that no
    automatic round trip is promised (import Design §20.4.5, **The export review's notice**).
  - **No store.** The adopted design adds no store and changes no deletion or retention
    behaviour. A trusted export ledger (store v14) is proposed, not adopted and unimplemented;
    it is eligible only for a separate design review (Agent 0, 2026-11-02; paraphrase), and
    nothing in slice 4 or 5 depends on it.

- **RS-P180.** It stays as authored: `unchanged_reference`, selecting its modulus at 210 °C. It
  is therefore refused by name (`MBF-SI-MODULUS-SELECTION-MISMATCH` at `pipe:RS-P180`) while it
  is an exported member.
  - It leaves the export only when the expansion-joint tranche (RT-K4, refused today) proves
    that the joint's exported representation removes the replaced span from the MBF, or maps it
    with its own per-member equivalence. Later joint support is not assumed to do either.
  - DEL-09-01's fixture is unchanged. Its document records the mismatch: an MBF consumer-coverage
    note below the §10 table, and §11.1.
- **RS-OPE.** It is an operating case (W + P + T), which stays refused
  (`MBF-SI-PRESSURE-CASE-UNSUPPORTED`). It needs the operating W+P+T mapping. Agent 0 approved
  designing that as a separate design-first item toward criterion 6 (2026-11-02). Route 1 does
  not provide it. As far as E goes, its 25 free-length members' derived row at 210 °C is exact
  (188300 N/mm²).
- **RS-SUS.** Beside RS-M1's varying modulus, it waits on G-2, the sustained case's modulus
  (`MBF-SI-SUSTAINED-MODULUS-UNPROVED`).
- **RS-SAM.** Its `boundary_motion` is a separate reference-system coverage dependency
  (`MBF-SI-BOUNDARY-MOTION-UNSUPPORTED`). The thermal slice does not admit it.
- **Varying expansion and cold-modulus files.** Under D-2 they stay refused
  (`MBF-SI-EXPANSION-NOT-CONSTANT`; `MBF-IMPORT-COLD-MODULUS-UNSUPPORTED`). They go to the
  criterion 3 limitations list with the owner's acceptance pending, and none is inferred. The
  proposed entry, for the implementation PR, is in the import Design §20.4.10.

Closing criterion 6 for the reference system therefore takes **coverage tranches** of the
writer and importer. These are the CAEPIPE lane's work, not desktop work (Q-3):

- Each tranche is source-gated through DEL-17-01, as the import Design requires.
- Some tranches wait for SWBPIPE members first: the operating case with nonlinear supports,
  and a case scope on hanger loads.
- A family that no admitted public source documents as an MBF record stays refused. Under
  criterion 3 it then goes to the release limitations, which the owner accepts (A-3).

## 2. Where the writer and importer run

| Option | What it means | Assessment |
|---|---|---|
| **R. Rust engine crate** | Port both functions to one Rust crate, used natively and in the wasm engine; the Python stays as the twin, checked by parity vectors | Recommended (A-2) |
| T. TypeScript port | The webview writes and imports; native only saves bytes | Native code cannot check that the bytes it writes are writer(model). A twin is still needed |
| P. Bundle Python | Ship an interpreter in the app | Refused: it enlarges the signed and notarized build (RT-V6) and adds a native-execution surface |
| H. Python headless service | The desktop launches a Python process | Refused for the same reasons, and the app does not depend on a Python installation |

**R in detail.**

- **The crate.** `core/handoff/caepipe_mbf_engine` (`open_pipe_stress_caepipe_mbf`).
  - It carries `#![forbid(unsafe_code)]`, has no file I/O and depends on no reporting crate.
  - It builds for both the native target and wasm.
  - It exposes `render_si(model, settings)`, `sidecar(result, mbf_sha256)` and
    `import(bytes, options)`. These use the Python codes, messages, maps and JSON outputs
    unchanged. As built (slice 2, §6), `sidecar` is the result's method
    `WriterResult::sidecar(mbf_sha256)`, as in the Python's `MbfSiResult`.
- **Parity.** Committed vectors from every public writer and import test model, the pilot
  cases, the invented import fixtures and the authored cases (#1702).
  - The Rust and Python outputs must be byte-identical: MBF text, sidecar bytes, each result's
    `to_dict()`, `model.json`, the import report and the id map.
  - The writer prints 10 significant digits. Rust must reproduce Python's float formatting
    exactly, so the vectors include rounding boundaries and fail closed, as `model_states.rs`
    does for `repr(float)`.
- **Numerical meaning.** The port carries unit factors, bend reconstruction and tolerance
  arithmetic. It therefore takes targeted regressions and independent scrutiny.
- **Two implementations.** Until the CAEPIPE lane decides otherwise (Q-1), the Python remains
  the validation lane's implementation. Each coverage tranche lands in both, and the parity
  test fails when either side moves alone.

## 3. Export (RT-R7)

### 3.1 Flow

1. **Entry points.** File > Export MBF… (`file.export-mbf`), placed after Save Handoff…. The
   Model batch file panel in the Exports section opens the same dialog. Both are reachable by
   keyboard. Neither has a result-output gate: the export carries no result data, so whatever
   result the session holds (none, a PF, exact or retained one, or a held one), its route,
   standing or hold neither disables nor refuses it (§6, "Output gate removed"). The export's own
   checks apply (§3.3 to §3.9). An Export MBF… chosen before the session's model hash is shown is
   refused by name (`MODEL_DOC_SAVE_HASH_PENDING`) and can be chosen again (slice 4b.2b).
2. **The dialog.** It captures the session's canonical model text and hash. It never runs the
   wasm writer: it asks the native `review_mbf_export` (§3.9.5) for a review with these
   settings:
   - **Title:** defaults to the project name. The writer trims it to 70 printable ASCII
     characters.
   - **Reference temperature:** 20 °C by default, or 68 °F in a degF project, passed as
     `reference_temperature_f` (exact).
   - **Row limit:** none, passed as `max_layout_rows=None` (Q-4).
3. **Review.** The dialog shows the review of §3.3. An edit to the model changes the hash, and
   the review runs again.
4. **Save….** It is enabled only when the status is `written` and the review is complete, current
   and within the output ceiling (§3.3). It calls `save_mbf_export`
   (§3.6), which opens the native save chooser. Cancel writes nothing. A review is not an
   authorization: the save rechecks everything itself (§3.9.5).
5. **Status line.** It names both written files and the count of omitted entries.

### 3.2 File names

- **The MBF.** The suggested name is `swbpipe-model.mbf`, or `swbpipe-model-<label slug>.mbf`
  when the user gives a label (#1711, kind `model`). The user may type any name. Native code
  requires the `.mbf` extension.
- **The sidecar (identity file).** Native code derives its name from the chosen path as
  `<chosen stem>.swbpipe-ids.json` in the same folder (A-5). The webview never names it. It
  is a derived name, not a suggested one, so the token rule does not apply. The export slice
  adds the row to #1711's table.
- **Replacing a file.** When the user confirms replacing an existing `.mbf` in the chooser,
  the identity file of the same stem is replaced with it. The dialog says so before Save. An
  identity file of the chosen stem whose `.mbf` is absent is not replaced: it may belong to an MBF
  that was moved, and the save is refused (`MBF-EXPORT-IDENTITY-FILE-EXISTS`, §3.6 step 8).

### 3.3 Review before writing

- **Profile.** The neutral label "SI model batch file, pilot subset", not the writer's
  internal profile id. The carried open items TBD-17-01-001 to -003 are shown as open.
- **Not written (blocks).** Every `unsupported` entry, and every error diagnostic that has no
  `unsupported` loss entry of the same code and entity (an invalid or out-of-range value, an unresolved
  reference, a schema version: the engine raises these without a loss entry), each with its
  code, its entity and a product-side description, grouped by family. While any exists, Save is
  disabled. A refusal is never silent: when the writer refuses and reports nothing that
  explains it, the dialog still states that SWBPIPE refused to write the file.
- **Omitted (warns).** Each `omitted` entry the review carries, with its code, its entity and
  a product-side description. "You can still save" is shown only when the review is complete,
  the writer's output is otherwise eligible, and every other current condition (intent, route
  standing, staleness, resources) permits the save; not merely when the writer produced bytes.
- **Bounded review.** The dialog shows the review that `review_mbf_export` returns, which is at
  most 1 MiB serialized (§3.9.5). Its lists (not written, omitted, warnings) are therefore listed
  within that bound. The dialog states whether the review is complete or truncated, the total
  count of entries not shown and how many of them block. The status and the blocking state are
  computed before truncation and always shown.
- **Complete review required.** The export is offered only after the writer's complete loss
  report has been shown (AC-003; §8.2 Q-6; REQ-004). **Any truncated review blocks the save**
  with `MBF-REVIEW-TRUNCATED` ("The loss report is too large to show in full, so the export is
  not offered."), even when none of the entries not shown would block on its own. Truncation is
  itself a blocking condition. The dialog claims nothing about the entries it does not show
  beyond their counts. The save path enforces this itself (§3.9.5): it never relies on a
  disabled control or on a flag sent by the webview. There is no second, unbounded view, report
  or download of the loss report; the writer's complete report exists only inside the worker,
  under its budget.
- **Usability limitation.** A model whose review exceeds 1 MiB serialized cannot be exported.
  The boundary is the size of the review: its entries (one per loss entry and unpaired
  diagnostic, each carrying a code, an entity and a category) and its fixed fields, after
  escaping and the per-field bound on long strings. No source argument excludes legitimate
  complex models from it: a model with many unexported members or fields produces one entry
  per member or field. This is a stated route limitation; the owner has not accepted it as a
  release limitation.
- **Unit conversions.** Each unit the writer normalized, with its exact definition and, where
  it has one, its exact factor (the writer's normalization data, not its assumption
  sentences).
- **Assumptions.** Not displayed, and the dialog does not point to a report: there is no local
  report artifact, and the writer report has no RT-R5 release standing. The dialog shows "The
  writer's assumptions are not displayed here."
- **Warnings.** Every other warning, for example `MBF-SI-SHEAR-MODULUS-NOT-EXPORTED`, by its
  code, entity and product-side description.
- **Materials.** Each material whose sidecar origin is `undeclared` is named: "importing this
  file asks where these values came from".
- **Identity.** "Identities travel in the identity file beside the MBF. Without it, an import
  forms ids from node numbers and refuses records whose meaning is still open."
- **Boundary.** "SWBPIPE writes the subset listed here. It does not check how another program
  reads or analyses the file."
- **Product text** (owner's ruling: "Don't state facts about CAEPIPE in public text"). The
  dialog does not show the engine's message texts: several state the target program's
  behaviour. Each code the writer can emit maps to a short description of what SWBPIPE did or
  did not write, kept in the dialog's module (`MBF_LOSS_DESCRIPTIONS` in `mbfExport.ts`) and
  keyed by code; a test holds that every code the engine's writer emits (`MBF-SI-*` and
  `MODEL_SCHEMA_VERSION_RETIRED`) has one. A blocking code's description (a code the writer
  raises by `refuse` or as an error, `MBF_BLOCKING_CODES`) ends "so SWBPIPE does not write the
  file."; the test derives the blocking set from the engine source and holds the descriptions
  to it. A writer refusal the native save returns is shown the same way. The engine's and the Python's texts are
  unchanged, so the parity bytes are preserved.
- **The export's own refusals** (slice 4b.2b). The refusals that are not about the model are shown
  by code with a product text (`MBF_EXPORT_REFUSAL_TEXT` in `mbfExport.ts`) that says what
  happened and what the user can do, never the native wording: `MBF-EXPORT-CANCELLED` (start
  again), `MBF-EXPORT-RETIREMENT-UNCONFIRMED` and `MBF-EXPORT-EPOCH-INVALID` (try again, then
  reload the window or restart), `MBF-EXPORT-RETIREMENT-ID-INVALID` (try again, then restart),
  `MBF-EXPORT-EPOCH-EXHAUSTED` (export is unavailable until SWBPIPE restarts, which recovers it),
  and `INGRESS-RAW-BODY-REQUIRED`, which a start meets once the window's transport has fallen
  back. Its text depends on the start: a review writes nothing ("Export isn't available in this
  window. Nothing was written. Reload the window or restart SWBPIPE to export again."), but a
  save's outcome is unknown, because the transport's resend can be refused after the original
  save committed and wrote (§3.7, "Source basis and limits"), and the page cannot tell that case
  from a save first sent after the fallback. A save refused so, in the open dialog or after it
  closed, says "SWBPIPE could not confirm whether this export wrote its files, because this
  window can no longer send exports. If you chose a folder for it, check that folder for the
  model batch file and its identity file. Reload the window or restart SWBPIPE to export again."
  (`MBF_SAVE_REFUSAL_TEXT`, `mbfSaveRefusalText` in `mbfExport.ts`). After such a save, open or
  closed, the page sends no further review or save until the window is reloaded or SWBPIPE
  restarts: Save stays disabled, and a review or save the dialog would start (after a settings
  change, a new review, or a close and reopen) is refused before anything is sent with
  `MBF-EXPORT-UNAVAILABLE`, "Export isn't available in this window: SWBPIPE could not confirm
  whether an earlier export wrote its files. This export did not start. If you chose a folder for
  the earlier export, check that folder for the model batch file and its identity file. Reload the
  window or restart SWBPIPE to export again." The page's retirements are still sent (§6, "Export
  unavailable after an unknown save"). A close that reaches a save past its commit does not stop it, and the save
  may still finish, or fail and undo itself: the status line first says only that the export
  "was already writing its files when it was closed; it is finishing", then the save's own
  outcome (`closedSaveStatusLine`): both files named when it saved, or its codes with their text
  when it failed or was undone, or that it "has no confirmed result" with the save's unknown-outcome text when it was
  refused after the fallback; a closed save's outcome is shown so whatever the close's receipt said, unless the
  outcome itself shows that nothing was written (§6, slice 4b.2b). Each line names the export's project, and "from an earlier
  project session" once the session has changed. No text names the mechanism (epochs, ids or tickets), and none suggests that anything grants a
  save.

| Condition | Effect |
|---|---|
| Writer `status: refused` (any `unsupported` entry or other error diagnostic, an out-of-table unit, a retired model version) | Blocks |
| Route screening blocked, or a protected disposition (§3.5) | Blocks |
| Review stale (model hash or settings changed) | Blocks until the review runs again |
| Review declined (`MBF-REVIEW-NATIVE-ONLY`, `MBF-REVIEW-MEMORY-UNAVAILABLE`) | Blocks |
| Review truncated (`MBF-REVIEW-TRUNCATED`), whatever the entries not shown | Blocks |
| Export result above the output ceiling (`MBF-EXPORT-OUTPUT-TOO-LARGE`, from the review's ceiling result; the save enforces it again through `result_limit`) | Blocks |
| `omitted` entries, warnings, undeclared material origin | Warn |

The release target's "Must" is not product vocabulary. The Must-family table lives in the
criterion-6 test (§5), not in the dialog.

### 3.4 Units

- **What is written.** The writer writes SI MBF only: mm, N, N·m, MPa, °C and bar.
  - It admits project units m, mm or in for length, N or lbf for force, and degC or degF for
    temperature.
  - US-customary units are normalized exactly, and each factor is listed.
  - Anything else is refused by name (`MBF-SI-PROJECT-UNIT-UNSUPPORTED`,
    `MBF-SI-UNIT-UNSUPPORTED`).
  - There is no English writer mode (import Design O-2).
- **Who owns the arithmetic.** Only the engine does unit arithmetic. The panel's own length
  conversion and its conversion witnesses (its use of `exportLengthConversion`, MBF-LENGTH-*,
  MBF-PROJECT-UNIT-MISSING) are retired with the TypeScript builder (A-6, slice 4b.2b);
  `exportLengthConversion` stays for the PCF export. The dialog only displays the writer's list.
- **Reference temperature.** It is not a model member, so the dialog asks for it. An import
  reads OPTEMP, and the session offers the imported value as the next export's default (Q-5).
- **English imports.** An imported English model re-exports through the documented
  normalization (REQ-014), and that is reported.

### 3.5 Screening, redaction and private content

- **Route.** A new governed route, `DOTH-MBF-012`: `local_private`, explicit user intent,
  lossless (A-4). It is added to the three catalogues:
  - `core/security/local_first_storage/route_control.py`;
  - `redactionExportControls.ts`;
  - the Rust admission.

  In the Python and TypeScript catalogues the route is local-private only
  (`LOCAL_PRIVATE_ONLY_ROUTE_IDS`, with the CSV route): any other context is refused
  (`LOCAL_PRIVATE_ROUTE_REQUIRED`). The native admission remains authoritative.
- **No redaction.** Redaction would change the bytes and break the round trip. A screening
  that would alter the payload blocks the save instead (`exact_payload_match`).
- **Disposition screen.** Native code screens the exact admitted model as Save Model Document
  does (`export_disposition_screen`).
  - A `provenance_record` that is `protected_suspected`, `quarantined` or `rejected` refuses
    the save with `PROTECTED_CONTENT_BLOCKED`.
  - An unresolved library reference warns.
- **Content.** The MBF and the identity file hold the user's model, so they are private, like
  the model document.
  - The file names carry no project identity (#1711). The title is in the file's content, and
    the user can edit it.
  - Sharing goes through the user-released report (RT-R5). Nothing is sent anywhere (F-PIP-1).
- **Product text.** It states no fact about the target program beyond the format name the
  product already uses ("Model batch file (.mbf)").

### 3.6 Native command, atomic write and read-back

The native command is connected (slice 4b.2a, §3.7, §6): `save_mbf_export` runs the steps below
in production, with step 5's writer in the capped worker process. `MBF-EXPORT-NOT-CONNECTED`,
which slices 4a and 4b.1 returned, is retired. Slice 4b.2b connects the frontend: the menu item
and the Exports panel action open the mounted dialog, which calls the command through the
barrier (§3.7).

**The library store.** Steps 2 to 4 read the libraries a document references from the local
store, which is opened lazily, only if the document references one, with the desktop's normal
open (`open_project_store`, as Save Model Document and the report package save open it). That
open may create the store, and it applies store schema v14 (the retirement of the unused FTS
sidecar, #146) atomically, or, if that step fails, rolls it back, opens at v13 with the step
reported pending, and retries it on a later open. The export writes no project, library or run
rows, and a read or open error refuses. The read-only open (`open_store_read_only`) is not used: it
refuses a store below the target version, including one whose deferred migration step is
pending, which the normal open keeps openable, so the export would refuse where Save Model
Document succeeds (RT-U4, 2026-11-05).

`save_mbf_export` takes these arguments: `{canonicalText, claimedModelHash, settingsJson,
suggestedName, reviewed: {mbfSha256, sidecarSha256}, screening, localFirstEvidence}`.
`settingsJson` is the settings' JSON text, `{title, referenceTemperature: {value, unit},
maxLayoutRows: null}`: the text the review ran with. In order:

1. **Route evidence.** Check `DOTH-MBF-012`, bound to the SHA-256 of `canonicalText`. Then read
   the settings text by the engine's one mapping (`export_settings_from_json`, which
   `review_mbf_export` also calls on the same text), before any copy of the model text: a text
   over its bound or a title over 4 KiB of UTF-8 is refused with
   `MBF-EXPORT-SETTINGS-INVALID`.
2. **Text.** Apply the 64 MiB cap, canonicalize again and check the hash against the claim,
   as `model_document_save.rs` does.
3. **Document checks.** Run `check_model_document` and the desktop shape check, so an MBF is
   only ever written from a document the desktop opens.
4. **Disposition screen.** As in §3.5.
5. **Writer.** (The design 4b connects.) Run the Rust writer in the capped worker process,
   worker task 3 in `save` mode (§3.7, §3.9.5). If it refuses, return the bounded review
   (`export_review`, §3.9.5) without opening the chooser; raw diagnostics and message texts never
   leave the worker.
   - **Complete review.** The `save`-mode run forms the bounded review itself and refuses with
     `MBF-REVIEW-TRUNCATED` when it is truncated, whatever the request claims. The MBF and
     identity-file bytes do not leave the worker, the chooser is not opened and nothing is
     written (§3.9.5).
   - **Output ceiling.** The whole serialized save result (MBF, identity file, bounded review and
     framing) is at most 64 MiB (§3.7). Above it the save is refused with
     `MBF-EXPORT-OUTPUT-TOO-LARGE` before the host allocates any of the result. The chooser is
     not opened and nothing is written.
6. **Staleness.** Compute `mbf_sha256` and the sidecar. If either digest differs from
   `reviewed`, refuse with `MBF-EXPORT-REVIEW-STALE`.
7. **Chooser.** Open the native save chooser (`.mbf`, the documents folder). The suggested
   name is checked against `swbpipe-model[-…].mbf`. The path comes only from the chooser.
   - **Cancel and commit.** A save retired after its worker ended (a newer save, or
     `cancel_mbf_export`) opens no chooser. Once the chooser returns a
     path, the save commits under its run's lock before the first write. A cancel that takes
     the lock first refuses the save `MBF-EXPORT-CANCELLED`, with nothing written. A cancel
     after the commit is too late: its receipt says so (`too_late`), and steps 8 to 10 complete,
     or undo themselves, and report their own outcome. A save is never half-aborted. A
     committed save stays tracked until it completes, superseded or not, so every cancel
     during its write reports it too late.
   - **Retirement by the dispatch ticket.** A save whose request carries a retirement epoch
     (`expectedEpoch`) that a retirement (`cancel_mbf_export`) has since
     passed is cancelled, whenever its task runs: at its entry, before the model is decoded, or
     when it registers its run. It replies `MBF-EXPORT-CANCELLED`, with no worker, no chooser
     and no write (§3.7, "Dispatch ticket"). Once the epoch is exhausted (§3.7) every save is
     refused `MBF-EXPORT-EPOCH-EXHAUSTED` at its entry, or cancelled when it registers, the same
     way.
8. **Writes.** No failure loses a file the user already had. The output ceiling is decided at
   step 5, before any write, so it never truncates valid file bytes and never leaves a partial
   pair.
   - **Before the first write.** Both destinations pass the atomic writer's own destination
     checks (extension, an existing non-symlink parent, no symlink or directory destination,
     and off macOS no replacement), so a destination the writer would refuse is refused before
     anything is written. An identity file of the chosen stem whose `.mbf` is absent is
     refused (`MBF-EXPORT-IDENTITY-FILE-EXISTS`): it may belong to an MBF that was moved.
   - **Backups.** Each existing regular file at either destination is backed up by a hard link
     to a hidden name in the same folder (`.swbpipe-mbf-<32 hex>.bak`). If a needed link
     cannot be made (FAT, exFAT, some network volumes), the save is refused before any write
     (`MBF-EXPORT-REPLACE-BACKUP-UNAVAILABLE`), and a backup already made is removed. There is
     never an unbacked replacement.
   - **Writes.** The identity file first, then the MBF, each through the extension-neutral
     atomic writer under its own format (`MBF_IDS_FORMAT` and `MBF_FORMAT`).
   - **Rollback.** On any later failure (either write or the read-back) the save is undone at
     each destination. A file this save wrote is removed, or the backup is renamed back over it
     in one rename, each only after the identity check: never remove or rename over a file the
     command did not create or back up. If the original is still in place, its backup is
     removed.
   - **Rollback that cannot restore.** If a backup cannot be renamed back, or the identity
     check refuses to touch the file now at the name, the save is not reported as success and
     the backup is not deleted. The refusal is `MBF-EXPORT-ROLLBACK-INCOMPLETE`, naming each
     kept backup and the file it holds in full (a file name is at most 255 bytes, within the
     2048 bytes a diagnostic's text keeps), so the user can recover it by hand. The response's
     reservation covers this refusal, so the names always reach the user.
   - **Success.** The backups are removed. A backup that cannot be removed is a warning in the
     receipt (`MBF-EXPORT-BACKUP-CLEANUP-FAILED`), not a failure.
   - **File identity.** Device and inode on Unix. Elsewhere, the size and both times, each
     present and equal; a missing time is not identical.
9. **Read-back.** Read both files back in fixed blocks and compare their SHA-256 values with
   the written digests.
   - On a mismatch, refuse with `MBF-EXPORT-READBACK-MISMATCH` and undo the save as in step 8.
10. **Receipt.** Both file names, byte counts and digests, and the atomic writer's durability
    statement.

The webview never runs the writer (§3.9.5). The written bytes are always the native writer's
output for the hash-checked text, from the save's own worker run. The review's digests serve
only the staleness check of step 6.

### 3.7 Memory

- **The in-process pricing was unsound.** Slice 4a first priced the in-process writer's
  working set as k·M plus a fixed 2 MiB, with k = 640 from a per-entity argument and measured
  shapes. Review found that bound unsound: the writer's peak grows with the square of its
  input, so no factor of the model's bytes bounds it. It has two quadratic terms:
  - long ids multiplied by the copies of an id each record makes (Agent 0's ruling,
    2026-11-01). Each primitive load holds about 40 copies of its load case's id. The test
    model `long_model(480)` (484 thermal loads) with a 20,000-byte case id, 315,911 bytes of
    model text, peaked at 644 MB priced, where k·M would have reserved about 204 MB; with the
    load count fixed, the peak grows linearly in the id's length, and with the id fixed,
    linearly in the load count (counting-allocator measurements, PR #70's review rounds);
  - long ids multiplied by unknown fields. The counting allocator measures the export fixture
    with one node renamed to a long id carrying unknown members. A 4 KiB id with 256 unknown
    fields (20.0 KB of model text) peaked at 7.7 MB priced, 385 bytes per model byte. A
    16 KiB id with 1024 unknown fields (63.8 KB) peaked at 105.6 MB, 1655 bytes per model
    byte. On a pipe, a 16 KiB id with 1024 unknown fields (31.0 KB) peaked at 141.9 MB,
    4570 bytes per model byte, before the writer refused the model.

  The k·M reservation, its constant and its counting-allocator pin are removed.
- **Governance.** Native writer memory is governed by running the writer in the capped
  worker process (owner decision 2026-10-12, "(c) Capped worker process"; Agent 0's ruling,
  2026-11-01) as worker task 3 (slice 4b.2a, `mbf_worker.rs`). The host keeps its own
  reservations for the text, the outputs, the read-back and the reply, below. Slice 4b.2b
  connects the live export (the menu item, the Exports panel and the mounted dialog).
- **Connected (4b.2a).** The production entry points (`save_mbf_export_admitted`,
  `review_mbf_export_admitted`) run task 3 through `mbf_worker::CappedWorkerRun`, the only
  production writer run; `MBF-EXPORT-NOT-CONNECTED` is retired. The in-process run
  (`InProcessWriterRun`) and its seam are compiled only for tests, and a source scan holds that
  in production code task 3's workload arm is the only caller of `worker_render` and
  `CappedWorkerRun` the only run.
- **Admission.** Both commands take C1's form (DEL-04-05 `c1-native-custody-plan.md` §8,
  "One running total per command"): the raw `args-json-v1` body is copied under a capped
  interactive lease, which seeds the command's `memory_pool::CommandLedger` (through
  `stored_read::NativeLeaseMemory`); the arguments' bound (envelope + M + 4 KiB,
  `ArgsDecoding::Moved`) grows it before they are parsed (`command_ingress::admit_raw`). The
  budget is captured then: min(the budget the copy's lease was admitted under, 12 GiB). Every
  later lease of the command is reserved in that ledger before the pool admits it, and the
  leases are held until the command's response is built (DEL-14-01 r4).
- **The worker's leases** (4b.2a). Before the worker starts, one ledger bound
  (`memory_pool::LedgerBound`) reserves the parent's copy of the request, the worker's grant up
  to its maximum and the largest result lease (the resident envelope of the 64 MiB output
  ceiling); the maximum grant is what the captured budget leaves after the command's other live
  leases and those two, and a starting grant above it is refused `ingress_exceeds_budget` before
  any worker starts. The grant is admitted under that maximum by the solve's queue wait
  (`wait_acquire`, at most 2 s; cancellation or a newer review ends it), grows on demand up to it,
  and is released only after the worker is reaped. The request copy is a capped interactive
  lease. Once the worker is reaped the bound shrinks to the result lease, which is held until the
  response is built. A result lease larger than the bound's share for it (the resident envelope
  of the 64 MiB ceiling, which bounds every admitted length's) is refused
  `MBF-EXPORT-WORKER-PROTOCOL` in every build, so the ledger never under-counts it.
- **The bound takes the command's whole remaining budget.** While the worker runs, the bound
  holds everything the captured budget leaves after the command's other live leases. A later
  stage inside the same command (a reservation after the run) gets those bytes back only after
  the bound shrinks to the result lease (`LedgerBound::shrink_to`) or drops. Dropping it,
  including when the run fails, is cancelled or unwinds, returns the ledger exactly to its total
  before the bound; a bound above the budget is refused with the ledger and the pool's
  reservations unchanged.
- **Cancellation** (`mbf_worker::MbfRuns`). A newer review or save cancels the older ones of
  its kind. `cancel_mbf_export` is the only *retirement* (the performance session reset,
  `dispose_all_preview_mechanics_results`, does not touch the MBF export; its contract is
  performance's, unchanged by this slice). A retirement cancels every
  run in progress (a waiting run stops waiting, a running worker is killed and reaped before its
  grant is released, and a save before its commit writes nothing, §3.6 step 7) and moves a
  monotone retirement epoch on, by a checked advance that never wraps. Every registered run stays tracked until its command ends,
  committed saves included; a cancel reports each run it stopped (`cancelled`) and each
  committed save it reached (`too_late`), and a run an earlier cancel stopped in neither.
- **Dispatch ticket** (`expectedEpoch`). A run belongs to the retirement epoch its request
  carries, not to when its task happens to run. Tauri 2 extracts an async command's arguments
  inside the task it spawns, so a request sent before a retirement may begin after it.
  - **The reply.** `cancel_mbf_export` takes `{retirementId}` (below, "Idempotent retirement")
    as Tauri's JSON arguments, the form the frontend sends, or as an `args-json-v1` raw body
    (below, "Wire form and bounds"). It replies
    `{cancelled, too_late, epoch, retirement_id}`, where `epoch` is the retirement epoch after its
    bump (`MbfRuns::retire_keyed`) and `retirement_id` echoes the id it answers.
  - **The request.** Every `review_mbf_export` and `save_mbf_export` request carries the
    frontend barrier's latest acknowledged epoch as `expectedEpoch`, a member of its admitted
    `args-json-v1` arguments.
  - **Native check.** The command reads the ticket from the admitted argument map
    (`MbfRuns::enter`). This happens after admission and before the model or any other member
    is decoded.
    - A ticket below the current epoch was issued before a retirement since. It refuses
      `MBF-EXPORT-CANCELLED`.
    - A missing, `null` or malformed ticket (anything but a non-negative integer), or one above
      the current epoch, is a protocol error: `MBF-EXPORT-EPOCH-INVALID`. It goes through the
      normal ingress and argument-lease route.
    - Either refusal registers no run, starts no worker, opens no chooser and writes nothing.
  - **Later checkpoints.** An accepted ticket is compared again when the run registers, so a
    retirement in between cancels the run from the start. After that, the registered run's own
    cancellation applies: the wait, the worker, the pre-chooser check and the commit, §3.6
    step 7.
  - **Page mount.** The workspace sends a keyed retirement when it mounts (page load included),
    whether or not a model check is running or the export was ever opened, so whatever an old
    page left queued or running is retired then, not at a later first use. Every start waits for
    an acknowledged retirement of its own page (the mount's, or else its own handshake).
  - **Authority.** The ticket only binds a request to a retirement. It authorizes nothing: the
    route evidence, the reviewed digests, the review's truncation and the commit are checked
    as before.
  - **Exhaustion.** The export enters a terminal, fail-closed *exhausted* state in either of
    two cases. Only a restart of the app's process leaves it.
    - *Epoch overflow.* A retirement finds the epoch at `u64::MAX` and cannot move it on. The
      export becomes exhausted rather than wrapping or saturating (saturating would leave the
      tickets at `u64::MAX` current), and the epoch stays at `u64::MAX`. Each applied
      retirement moves the epoch by one, so this takes 2^64 of them in one process; it exists
      so the epoch's meaning cannot fail silently.
    - *Token capacity.* A retirement comes from a new page token once the host keeps
      `RETIREMENT_PAGES_MAX` = 64 tokens (below, "No applied token is forgotten"). The token is
      not recorded and the epoch is not moved on: it stays at its current value, far below
      `u64::MAX`. Each page load makes one token, so in practice **the 65th distinct page load
      in one process** (a webview reload counts) exhausts the MBF export until the app
      restarts. This is an availability limit, chosen over evicting a token whose delayed
      duplicate would then be applied again.
    - In both, the retirement that exhausts the export cancels every run in progress as any
      other does, and its reply adds `exhausted: true` with the epoch as it is (the member is
      absent otherwise). From then on every request is refused `MBF-EXPORT-EPOCH-EXHAUSTED` at
      its entry, whatever its ticket, a command that entered before is cancelled when it
      registers, and later retirements stay exhausted; the epoch moves no more. The frontend
      treats an exhausted reply as unacknowledged, so its starts refuse
      `MBF-EXPORT-RETIREMENT-UNCONFIRMED`. The dialog shows each in product text (§3.3, "The
      export's own refusals"): an exhausted export is unavailable until SWBPIPE restarts.
- **Idempotent retirement** (`retirementId`, `mbf_worker::RetirementId`). Each logical
  retirement carries `{page, sequence}`: a random token of 32 lowercase hex digits made once per
  page load (`crypto.getRandomValues`), and a sequence from 1 that rises by one per logical
  retirement in that page. A re-delivery of the same logical retirement (the barrier's, or the
  transport's) reuses it.
  - **Native rule** (`MbfRuns::retire_keyed`). For every page token it has applied, the host
    keeps the last sequence applied, and applies a retirement only when its sequence is above
    it. A duplicate or older delivery is a no-op: it cancels nothing and moves nothing on, and
    its reply says so truthfully (`duplicate: true`, `cancelled: 0`, `too_late: 0`, the
    current epoch, the id echoed). A missing, `null` or malformed id (not an object of exactly
    `page` and an integer `sequence` from 1 to 2^53 - 1) is refused
    `MBF-EXPORT-RETIREMENT-ID-INVALID` and retires nothing.
  - **No applied token is forgotten.** The host keeps at most 64 page tokens (one per page load
    in the process) and never evicts one, since a forgotten token's delayed retry would be
    applied again. A retirement from a new token beyond them is not recorded: it makes the
    export exhausted (fail closed), cancelling every run in progress, with the epoch unmoved
    (above, "Exhaustion": token capacity, recovered by a restart).
  - **Ids are not authenticated.** A page token is random but not a secret, and the host does
    not check who sends a retirement. Code running in the webview can send `cancel_mbf_export`
    with any id: it could cancel MBF runs, or use up the 64 tokens and exhaust the export until
    restart (a denial of service). It gains no save authority: a retirement only cancels and
    refuses, and neither a token nor a ticket authorizes anything. This is not a security
    boundary; the webview's code is trusted as for every other command.
  - **Wire form and bounds** (`mbf_worker::cancel_command`; repair 6). The frontend sends the
    retirement as Tauri's JSON arguments, `{"retirementId": {page, sequence}}` with no ingress
    header, always, not only after a detected fallback: the page cannot see the fallback (the
    flag is private to Tauri's transport script, and the invoke whose `fetch` failed is itself
    resent through `postMessage`), and the JSON arguments arrive unchanged on both transports
    (the custom protocol posts them as `application/json`; the fallback `JSON.stringify`s them
    inside its message). The host admits this form for `cancel_mbf_export` only, beside the raw
    form (an `args-json-v1` raw body with the ingress header, admitted as before); a JSON body
    under the ingress header is still refused `INGRESS-RAW-BODY-REQUIRED`. Both forms read the
    id from the same member by the same rule and retire by the one keyed path, so validation,
    once-only application (a delivery in one form and a re-delivery in the other are one
    logical retirement), receipts and exhaustion are identical. The starts
    (`review_mbf_export`, `save_mbf_export`) stay raw-body only.
    - *What the 4 KiB bound limits.* For the raw form, the raw bytes before the prescan and
      before any lease (`INGRESS-ARGS-TOO-LARGE`), as for every bounded C1 command. For the JSON
      form, Tauri parses the body into a `serde_json::Value` before the command runs, unpriced,
      and neither transport limits a request's size first: the custom protocol reads the whole
      body and parses it as JSON, and the fallback parses the whole message string. No check
      of ours bounds Tauri's parse or its allocations, exactly as for every JSON-argument
      command (`cancel_preview_mechanics_job`). The command's check bounds what follows: the
      decoded arguments' canonical serialization is measured without allocating a serialized buffer (a
      rejection allocates only serde_json's small boxed I/O error), and refused
      `INGRESS-ARGS-TOO-LARGE` above 4 KiB before anything is read from them; anything but an
      object is refused `INGRESS-ARGS-NOT-OBJECT`; the id is then copied out of at most that
      many bytes. Like the JSON-argument commands, the form takes no ingress lease: the value a
      lease would price already exists.
  - **Frontend.** A receipt acknowledges a delivery only when it echoes that delivery's id; a
    `duplicate` receipt for its own id acknowledges the current epoch. A page that cannot make
    an id sends nothing, and the delivery is unacknowledged.
  - **Source basis and limits** (Tauri 2.11.1 `scripts/ipc-protocol.js`; RT-U4's assessment).
    A rejected `fetch`, or a failed read of its reply, makes the transport resend the command
    through `postMessage` without cancelling or deduplicating the original delivery, which may
    still reach the host. A settled invoke is therefore not every delivery of it. The same
    failure also sets the page's `customProtocolIpcFailed` for good, and the fallback sends
    through `JSON.stringify`, which turns a raw `Uint8Array` body into a JSON number array: the
    raw-body ingress then refuses `review_mbf_export` and `save_mbf_export`
    (`INGRESS-RAW-BODY-REQUIRED`). The keyed `cancel_mbf_export` is sent as JSON arguments
    (above, "Wire form and bounds"), so after the flip the page's retirements are still
    admitted and applied: a run admitted before the flip, a save waiting at its chooser
    included, is stopped by the page's next retirement (a session retirement, or the dialog's
    close) before its commit, and writes nothing. The page can no longer start a run: each
    start, sent raw, is refused at ingress and fails closed. So the schedule "fallback
    acknowledged, fresh run admitted, delayed original cancels it" is not a demonstrated
    same-page product path, and the tests that pin it are injected-fault tests of the rule.
    The ingress refusal proves only that the delivery it answers wrote nothing: when the
    fallback was triggered by a failed read of a save's reply, the original raw save may
    already have committed, written or undone itself, and its resend is the one refused. The
    page cannot see the flip, so a save refused `INGRESS-RAW-BODY-REQUIRED` has an unknown
    outcome and is reported so (§3.3); a review refused so wrote nothing. Such a save shows the
    page that its transport has fallen back, so from then on the page sends no review or save
    at all (`MBF-EXPORT-UNAVAILABLE`, §3.3) and keeps sending its retirements.
    Duplicate deliveries of an id the host has applied remain no-ops whatever their route or
    form. Its frontend result is discarded by its generation, and the save's pre-commit checks
    are unchanged. *Across a navigation* the order is not established:
    a delayed first delivery from an old page (an id the host never applied) that arrives after
    the new page's start is applied as a new retirement and cancels that start. Distinct page
    tokens with their sequences do not order such a delivery in time, so this design does not
    claim to prevent it. It is conditional, not established on WKWebView. Its effect is
    fail-safe (the new start is cancelled, nothing is written, a committed save completes), and
    RT-U4 documents C1's conditional accounting for it separately.
- **The frontend barrier** (`features/caepipe-mbf/mbfExportCancel.ts`). Every MBF start
  (`reviewMbfExport`, `saveMbfExport`, raw-body ingress commands) and every retirement
  (`retireMbfExportSession`) goes through it. The workspace retires the export whenever it
  retires a project session (each new session generation, a project replaced by one with the
  same id included, React Strict Mode's re-run) and when it unmounts, independently of any
  model check, and sends a keyed retirement when it mounts (§3.7, "Page mount"); the dialog
  (slice 4b.2b) retires it when it closes or unmounts, and a new project session closes the
  dialog. **Serialized retirements:**
  each `cancel_mbf_export` delivery is sent only after the one before it has settled
  (acknowledged or not); the barrier neither waits for nor sends the performance reset. Tauri
  sends each invoke as an independent request, so send order is no barrier: without the chain,
  an earlier retirement could be applied natively after a later one was acknowledged and a
  fresh start sent, and cancel that start. The chain orders deliveries as the barrier sends
  them; it cannot order a second delivery the transport makes of the same invoke, which the
  native rule ("Idempotent retirement") makes a no-op. A start captures the
  retirement generation as it is called, waits for the latest delivery (by the chain, every
  delivery the barrier sent before it has then settled), and checks
  immediately before every send (a headroom re-send included) that no retirement has been
  requested since and that the caller's own session check (`stillCurrent`) holds; a stale
  start is never sent and resolves `MBF-EXPORT-CANCELLED`. **Ticket:** the barrier records the
  largest epoch any settled `cancel_mbf_export` delivery acknowledged; a reply without an
  epoch, or an exhausted one, is not an acknowledgement. A start takes that epoch once, when its wait is over and it is still
  current, and sends it as `expectedEpoch`, the same bytes in every re-send. A stale start
  never refreshes its ticket; only a new, current start takes a newer acknowledgement. Until a
  retirement has been acknowledged in the page, the first start delivers one as its handshake,
  and concurrent first starts share it. Each result carries its generation
  and `current()`, so the caller discards a completion that arrives after its session was
  retired. **Semantics:** a retirement cancels every MBF run that started before it completed;
  a start issued during a retirement reaches native only after it is acknowledged and is not
  cancelled by it. **Fail-safe rule:** a start never treats an unacknowledged retirement (the
  `cancel_mbf_export` invoke rejected or threw) as done. When the latest delivery is
  unacknowledged it re-delivers it once, with the same id (shared by the starts waiting on it),
  so a retirement the host applied whose reply was lost is a no-op there; if that is
  unacknowledged too, the start is not sent and refuses `MBF-EXPORT-RETIREMENT-UNCONFIRMED`,
  and the next start tries again. An earlier unacknowledged delivery followed by an
  acknowledged one needs no re-delivery: the later one cancels every run started before it,
  and the earlier one adds no epoch. The chain moves on once a delivery settles, so a failed
  one never holds a start. No start has been sent since the retirement was requested, so a
  re-delivery cancels only retired runs.
- **Host-side reservations.** The command's ledger (`NativeLeaseMemory`) is seeded with the
  lease. Each of these is reserved before it is made:
  - the settings' parse (1 MiB, for a settings text of at most 25 KiB);
  - the parse and the canonical copy;
  - the worker's save result, at most the output ceiling below (the worker's result lease, held
    through the response), and each host copy of it: any decoded copy, any buffer the atomic
    writer takes, and the receipt or refusal reply are each leased;
  - a 64 KiB read-back block;
  - the response, bounded for a receipt, for the largest refusal after a write, and for a
    refusal before the chooser, which carries at most R of review (§3.9.5).
- **Output ceiling** (the design 4b connects; Agent 0, 2026-11-02). This is a new, explicitly
  bounded MBF product-route limit. The whole serialized save result (MBF, identity file,
  diagnostics and framing) is at most 64 MiB (67,108,864 bytes). It applies in addition to the
  existing rule that a result fit within the worker's grant.
  - **Why.** The identity file grows super-linearly with the model (Q1, §3.9.2), so the result
    cannot be priced from M, and the host cannot reserve its copies without a stated bound.
  - **Where.** The worker's `result_limit` hook (`SupervisorHooks::result_limit`,
    `solve_worker.rs`) checks the length the worker announces before any result lease is
    admitted and before the host allocates any of the result, as for solve results (DEL-04-07
    `Work/memory-budget-decision.md`). The child's construction of the result stays counted
    against its grant.
  - **Refusal.** `MBF-EXPORT-OUTPUT-TOO-LARGE`, distinct from a budget or worker refusal
    (`MBF-EXPORT-MEMORY-UNAVAILABLE`), a writer refusal, a protocol failure and a cancel. Valid
    file bytes are never truncated, nothing is written (§3.6 step 5), and §3.6's transaction and
    rollback are unchanged.
  - **Limitation.** A model whose export result exceeds 64 MiB cannot be exported from the
    desktop. This is a usability limitation of the route. The owner has not accepted it as a
    release limitation.
- **Refusal.** A reservation the pool refuses refuses the save with
  `MBF-EXPORT-MEMORY-UNAVAILABLE`, including a refusal raised inside the admission's screen.
- **Webview preview.** As built in 4a, the dialog's review ran the wasm writer in webview
  memory, ungoverned, and the dialog was not mounted. §3.9 (adopted design, Agent 0, 2026-11-02; its
  native part implemented in 4b.1 and 4b.2a, the dialog mounted in 4b.2b) finds five super-linear terms, none bounded by an existing limit. The shipped
  dialog therefore never runs the wasm writer: `review_mbf_export` runs the 4b worker in
  `review` mode and returns at most 1 MiB of review. The wasm writer remains only as an
  unconnected or test resource, never a fallback.

### 3.8 Reach-scan re-pin

- **The pin.** Adding `save_mbf_export`, `review_mbf_export` and `cancel_mbf_export` (and the
  import's `open_mbf_import`, `review_mbf_import`, `adopt_mbf_import` and `cancel_mbf_import`,
  §4.7) changes the text of `app_invoke_handler`.
  - The PR that adds each command re-pins its SHA-256 in `user_release_reach_scan.rs`
    `MACRO_DEFINITIONS`.
  - It extends the doc comment with the entries, each reviewed: no `mod`, no
    `$parameter ::`, no scanned name added.
- **Unsafe code.** The new `src-tauri` modules (the export's `mbf_export.rs` and `mbf_worker.rs`,
  and slice 5's `mbf_import.rs`) carry no `#[allow(unsafe_code)]` and are not added to
  `SHELL_UNSAFE_MODULES`.
- **Reach closure.** The engine crate depends on no reporting crate, so the renderer's reach
  closure is unchanged (DEL-08-01 `Work/report-user-release.md` §2.8).

### 3.9 Webview preview admission (adopted design)

**Status: adopted design (Agent 0, 2026-11-02); its native part is implemented (slices 4b.1 and
4b.2a, §6), and slice 4b.2b mounts the dialog on `review_mbf_export`.** Agent 0 chose (iii) with the
bounded review, the task-specific worker modes and the output ceiling recorded in §3.9.5,
§3.9.7 and §3.7. This answers Agent 0's ruling of 2026-11-01. The ruling
asks for every size dimension that makes the preview's allocations super-linear, set against
the model's existing limits, and for the smallest sound admission. The analysis is from reading
source only, because a host hold was in force: each count below is a count from source, and
§3.9.6 plans the tests that confirm it. Paths are relative to
`core/handoff/caepipe_mbf_engine/src/` unless they are named otherwise.

#### 3.9.1 What the preview runs

- **The call.** `MbfExportDialog.tsx:134-135` canonicalizes the session model and calls
  `renderMbfExport` (`services/mbfEngine.ts:193-213`). That function calls the wasm
  `render_mbf_json` synchronously, on the webview's main thread.
- **The wrapper** (`caepipe_mbf_wasm/src/lib.rs:123-186`) does four things:
  - it parses the text (`parse_sidecar_json`);
  - it runs `render_si`: the reader, the layout and the render;
  - it runs the writer's preflight (`writer/render.rs:864`, `875-932`). The preflight is a bound
    re-import of the written text, and that import's regeneration runs the writer a second time,
    over the imported model (`importer/build.rs:404-462`);
  - it builds the outcome from `to_dict_bytes` and `sidecar_bytes`.
- **The webview's copies.** The webview parses the outcome, then parses `report_json` a second
  time (`mbfEngine.ts:199-212`).
- **Memory stays held.** Wasm memory only grows, and the engine is a cached singleton
  (`mbfEngine.ts:97`, `170-178`). A preview's peak therefore stays held until the page reloads.

#### 3.9.2 The size dimensions

**When a term is linear.** A copy of text the model writes out at that place is linear. An id
named by each reference, such as a pipe's `from` or a load's `target`, is paid for in the text
once per reference. A term is super-linear when one occurrence in the text is copied once for
each member of another collection.

**Two multipliers apply throughout.**

- An `OMap` key is held twice (`value.rs:11-15`).
- A copy inside JSON text can be up to 3.5 times its UTF-8 length: a 4-byte character becomes a
  12-byte `ensure_ascii` surrogate pair, and the outcome escapes the backslashes again.

| # | Dimension | Copy pattern | Source | Live copies (by count) | Existing limit |
|---|---|---|---|---|---|
| Q1 | A case id's length × the primitive loads and pressure regions in that case | Per load: the entity `load:<case>/<load>`, the record's `load_case` and the `load_records` key; `load_map`'s entry and key; the sidecar's `load_map` key and `load_case`, formed twice; the preflight's id-map entry, its `ids_seen` texts, the regenerated sidecar and both canonical sidecars it compares. Per region: the place `load_case:<case>/region:<id>`. In the sweep: a label per load. On the refusal path: each refused load's entity and message, each held in a diagnostic and a loss entry | `writer/read.rs:937-942`, `1064`, `1155`, `1174`, `1318`, `1417-1426`; `writer/render.rs:769-791`, `855`, `878`; `writer/mod.rs:599`; `importer/assemble.rs:1331`, `1383-1401`, `1442`; `importer/build.rs:416-454`; `sidecar.rs:879`, `984-1001` | About 30 to 40 at the preflight's deepest point. About 16 on the refusal path, where a refused load can be 15 bytes (`{"category":0},`) | None |
| Q2 | An entity's length × the present unhandled fields of its record and the record's swept parts. For a load, the entity is `load:<case>/<load>` | `found[path]` gets the entity once per field. Each path's message joins those entities, and the message is held in a diagnostic and a loss entry | `writer/read.rs:1367-1374`, `1405-1428`, `1432-1446`; `writer/mod.rs:847-855` | 3 in the writer, about 8 at the wrapper and about 6 more in the webview's heap | None in force (see below) |
| Q3 | A material id's length × the refused nonzero thermal loads on pipes of that material, when the material has no expansion coefficient | Each refusal's message names the pipe's material, which the load's own text does not name | `writer/read.rs:1259-1267` | 2 in the writer, about 8 at the wrapper | None |
| Q4 | The text of a bend's component id × the weight and thermal loads on the bend's pipe | `load_map` copies the target pipe's whole `element_map` entry into each such load, and for a bend that entry holds `bend.component` | `writer/render.rs:549-550`, `562`, `776-778` | About 5 | None |
| Q5 | The id of the first case of a load-case kind × the later cases of that kind | Each later case's refusal names `load_case:<first>` | `writer/read.rs:975-982` | 2 in the writer, about 8 at the wrapper | None |

**A check against §3.7.** On `long_model(480)`, §3.7's measured peak gives 644 MB / (20,000 ×
484) = 66.5 priced bytes per case-id byte per load. That agrees with Q1's count once each string
built by `format!` is priced at up to twice its length (it grows by doubling).

**Candidates rejected as growth factors.**

- **Node, pipe, material, section and support ids × their references.** Each reference repeats
  the id in the text, so each copy is a constant number of copies of text. This covers:
  - the readers (`writer/read.rs:401-408`, `413`, `324`, `532-533`, `641-642`, `1065-1070`,
    `1183-1188`, `1213-1218`);
  - the layout (`writer/render.rs:238-245`, `284-353`);
  - the node and attribute maps (`writer/render.rs:381-451`).

  A bend copies its incoming straight's id at most twice (`writer/render.rs:500-506`).
- **String fields (labels, provenance, notes).** The sweep stores field names with the entity,
  never the field's value (`writer/read.rs:1371-1372`). A region's and its terminals'
  provenance is copied once (`writer/render.rs:806-828`). Project units are copied once
  (`writer/read.rs:1388`).
- **`repr` and `str` in messages.** Each is of the value at one place in the text, taken a fixed
  number of times. Examples: `writer/mod.rs:880-886`; `writer/read.rs:404`, `513-518`, `1283`.
- **Nesting.** The webview's parse refuses more than 64 levels (`MAX_SIDECAR_NESTING`, re-exported
  at `lib.rs:48`), and the writer's screen refuses more than 256 (`value.rs:286`,
  `writer/mod.rs:620-622`). No walk multiplies copies by depth. The importer's path strings
  (`importer/build.rs:349-372`) are transient. A path is held only in a refusal message, and a
  load's path names its case, which Q1 counts.
- **The MBF text.** It is bounded per row: the title is at most 70 characters
  (`writer/mod.rs:61`), a description at most 30 (`writer/render.rs:593`), and the rest is
  names and numbers. So the text, its re-parse (`writer/render.rs:696`) and its re-import
  (`writer/render.rs:893`) are linear in rows.
- **The sidecar.** It is linear in the model, except that it carries Q1: its `load_map` holds
  the case id twice per load (`writer/mod.rs:599`). The identity file's size is therefore
  super-linear in the model text (§3.9.7).
- **Record counts and diagnostics.** These are linear, but the constant per byte is large on the
  refusal path. A 15-byte refused load creates two four-member dictionaries
  (`writer/mod.rs:829-855`), and they are copied again by `refused()`, by `to_dict` and into the
  outcome. 4a's per-entity argument (412 bytes per model byte, removed with k) counted only
  written units.
- **CPU time (not memory).** These terms grow super-linearly in time but hold no extra memory.
  In the webview they block its main thread:
  - the layout sorts the remaining nodes once per tree (`writer/render.rs:275-283`);
  - a pipe with an unresolved material scans the diagnostics (`writer/read.rs:417`);
  - section binding (`writer/read.rs:330`, `340`);
  - the bend scan of loaded nodes (`writer/render.rs:91`);
  - the scan of unconnected nodes (`writer/render.rs:252-254`);
  - combination operands (`writer/read.rs:1471-1473`);
  - the importer's `incomplete` (`importer/mod.rs:336-337`).

#### 3.9.3 Against the existing limits

| Limit | What it bounds | Q1–Q5 |
|---|---|---|
| Model schema `Id` (`schemas/model.schema.yaml:1238-1242`): a string matching `\S`, with no `maxLength` | Nothing about length | Not bounded |
| The schema's closed records (`additionalProperties: false` on every record the sweep visits) | Would hold Q2's F at 29 at most per part (ComponentGeometry's 32 properties, less the 3 handled) | Not in force. The document validator refuses unknown members only on its own slots (`document_validator.rs:30-31`), and the shape check (`modelDocumentShape.ts`) checks types only. An admitted model may carry unknown members on a node, pipe, support, component or load |
| Shape check: entity ids are strings (`modelDocumentShape.ts:23-33`) | The type only. Load and region ids are not checked at all | Not bounded |
| 64 MiB (`model_document_file.rs:27`, `MAX_MODEL_DOCUMENT_BYTES`) | M only | Each term reaches M²/c. For example, Q1 with half the text one case id and half refused loads is (32 MiB) × (32 MiB / 15) × 16, of order 10¹⁵ bytes |
| Nesting (64 at the parse, 256 at the writer) | The stack | Not a growth factor |
| Ingress prescan (`ingress.rs:550`) | Native serde_json materialization | Native only. It does not price the engine |
| DEL-04-06's browser bound (4 MiB, `modelChecks.ts:61`) | That command's browser-only path | A different command, with no proof for this writer |

No term is bounded by an existing limit.

#### 3.9.4 Options

- **(i) Webview admission limits.** These would be checks before `render_si`, over the parsed
  model, of M and of the five products Q1–Q5. They are not chosen, for four reasons:
  - **The products need the writer's logic.** Q5 needs the writer's case-kind classification,
    and Q3 and Q4 need its pipe, material and bend relations, so the TypeScript would
    re-implement the writer's branches. A route the walk misses is unsound, as the k pricing
    was.
  - **The linear coefficient is not established.** It needs a new per-unit argument that covers
    refusal-path units. Two rough counts put it at several hundred bytes per model byte,
    across wasm memory and the JavaScript heap.
  - **The webview has no stated memory ceiling.** It would need one, measured (§3.9.5).
  - **Time and residency stay.** The main-thread time terms remain, and the peak stays held for
    the session (§3.9.1).

  A sound (i) would still decline models that the native export writes.
- **(ii) A restricted review path.** It is not chosen, because it is not smaller. The status
  depends on the layout, the bends and the preflight's re-import (`MBF-SI-PREFLIGHT-REFUSED`),
  so a path that computes the loss entries and the status is the writer under another name, and
  outside the parity vectors.
- **(iii) Review through the 4b worker. Chosen (Agent 0, 2026-11-02).** It reuses 4b's
  governance proof instead of proving webview bounds. DEL-04-06 takes the same route for its model-sized work, which runs
  in the app in a capped worker (`modelChecks.ts:152`).

#### 3.9.5 Design (iii)

- **Route.** The shipped dialog never runs the wasm writer. It calls a native
  `review_mbf_export` command with `{canonicalText, claimedModelHash, settingsJson}`.
  - **Browser-only route** (no native host). It has no save. The dialog declines the review
    there with `MBF-REVIEW-NATIVE-ONLY` ("The model batch file review runs in the desktop app.")
    and runs no writer.
  - **The wasm export entry point** (`caepipe_mbf_wasm` `render_mbf_json`) stays only as an
    explicitly unconnected or test resource. It is never a fallback. The import does not run in wasm.
    Slice 5's design assessed that route, which also runs the writer through regeneration
    (`importer/build.rs:404-405`), and did not adopt it (§4.8.1). The import runs as task 5 in
    the capped worker (§4.8.3).
- **Admission.** The review uses the same model, settings, admission and binding as
  `save_mbf_export`: the same ingress admission, the settings text read by
  `export_settings_from_json`, and §3.6 steps 2 and 3 (the 64 MiB cap, canonicalization, the
  hash bound to the claim, and the document checks). The review therefore runs on exactly what
  a save would write. It holds no route evidence.
- **Review touches nothing.** Review mode makes no mutation and touches no file, store,
  destination or chooser. The same holds for the command: `review_mbf_export` and its worker run
  open no store, file, destination or chooser, and write nothing.
- **Permissions and reach.** `review_mbf_export` joins 4b's permissions review, reach-scan
  re-pin (§3.8) and ingress review exactly as `save_mbf_export` does.
- **Worker task and modes.** The command runs 4b's MBF worker task, task 3 in the 4b proposal.
  Task 3 carries a strict `mode` discriminator, `review` or `save`, with these invariants:
  - the child refuses any other `mode` value, any field that does not belong to the request's
    mode, and a missing field the mode requires;
  - each mode's result shape is fixed. `review` returns only the bounded review below. `save`
    returns the MBF bytes, the identity-file bytes and the bounded review, within the output
    ceiling (§3.7); on a refusal it returns only the bounded review;
  - the parent refuses a result whose shape does not match the requested mode (for example, a
    `review` result carrying MBF bytes) as a protocol failure;
  - there is no generic or arbitrary-worker dispatch. Task 3 and its two modes are the only MBF
    work the worker accepts;
  - the envelope's byte layout is to be agreed with the desktop performance work (#59). These
    invariants hold whatever layout is agreed.

  The envelope, the starting grant, growth up to the admitted budget, the 2 s wait for a slot
  and the worker refusals are 4b's. A worker refusal declines the review with
  `MBF-REVIEW-MEMORY-UNAVAILABLE`, which names the worker's refusal. At most one review is in
  progress. A newer review, closing the dialog or a change of project session kills the running
  worker and releases its grant after the reap, as for model checks.
- **A review is not an authorization.** The save rechecks the current model, settings, binding,
  route standing and staleness itself (§3.6 steps 1 to 6) in its own `save`-mode worker run.
  `reviewed` carries the review's digests, and step 6 compares them with that run.
  - **Completeness enforced on the save path.** The `save`-mode worker run forms the same
    bounded review (`export_review`, below) over the same model and settings, inside its
    budget, and refuses with `MBF-REVIEW-TRUNCATED` when it is truncated, before the MBF and
    identity-file bytes leave the worker, and before any chooser or write. The host does not accept a completeness flag from the webview. Files are
  created only on the user's explicit intent in the chooser (step 7).
- **The bounded review.** One engine function, `export_review(result, allowance)`, beside
  `export_settings.rs`, forms the review inside the worker.
  - **Ceiling.** The total serialized review is at most R = 1 MiB (1,048,576 bytes), counted
    after escaping. R covers everything the review carries: headers, status, digests, counters,
    codes, entities and conversions, not only the entries. R is an explicitly selected MBF
    review ceiling under the engineering delegation (Agent 0, 2026-11-02). It is not inherited
    from DEL-04-06's report semantics.
  - **Status first.** The status, the blocking state and any global refusal are computed over
    the full result before any bound is applied, and are always carried. Overflow can never turn
    a blocked export into a savable one or hide a global refusal.
  - **Classification.** It applies §3.3's classification, which moves here from `mbfExport.ts`,
    before any bound is applied, so a diagnostic is never separated from its loss entry.
  - **Contents.** It returns:
    - the status and blocking state, and `mbfSha256` and `sidecarSha256`, computed in the
      worker over the full text and sidecar bytes;
    - whether the save result would be within the output ceiling (§3.7), computed in the worker
      from the full text and sidecar lengths;
    - the unit conversions;
    - the undeclared materials;
    - the entries, each with list, code, entity and category: the not-written entries first,
      then the omitted, then the warnings, each in the writer's order;
    - whether the review is `complete` or `truncated`, the total count of entries not shown and
      how many of them block, and the same two counts per list.
  - **Overflow.** Space for the status, digests and counts is reserved first. Entries are added
    in order while they fit within R; the first that does not, and every later entry, is counted,
    not listed. A diagnostic and its loss entry are listed together or counted together.
  - **Long strings.** An id, code, entity or unit string longer than a fixed per-field bound
    (set in 4b, well below R) is carried as its first bytes up to that bound, cut at a character
    boundary, with its full byte length and SHA-256. The representation is formed from the
    string in place: no unbounded intermediate, such as an escaped copy or a joined message, is
    allocated only to be cut.
  - **What stays in the worker.** Message texts are not sent: no target-program message reaches
    product chrome, and §3.3's conversion and loss-text boundaries are unchanged. In review mode
    the MBF text and the sidecar bytes never leave the worker.
  - **The result.** It comes back under a result lease of R plus the worker's frame header. A
    longer announced result ends at `result_limit` before any lease (`ResultTooLarge`) and is
    shown as `MBF-REVIEW-MEMORY-UNAVAILABLE`; a malformed frame is a protocol failure. The codes
    for a protocol failure and a cancellation are 4b's, named with the worker envelope.
- **Text.** §3.3 lists entries within the bounded review and counts the rest with how many
  block.

**What (iii) bounds.**

- No model-sized MBF work runs in webview memory. The webview holds the canonical text, which it
  already has, and at most R of review.
- Q1–Q5, the linear terms and the time terms all run in the capped worker. Its cap enforces the
  admitted per-invocation budget (DEL-04-07), and cancellation ends the time terms.
- A model beyond the budget is declined by name, and its save would be declined by the same
  admission. Model validity is unchanged: such a model is still valid, and nothing new is
  refused at open.
- The save's result is bounded by the 64 MiB output ceiling (§3.7). A model whose export result
  exceeds it cannot be exported (`MBF-EXPORT-OUTPUT-TOO-LARGE`).

**What (iii) does not bound or claim.**

- It does not predict the worker's peak. The cap enforces the budget; nothing estimates the peak
  in advance.
- It sets no wall-clock limit beyond cancellation.
- It covers nothing in the browser-only route, where nothing runs.
- It does not make the 4a wasm preview governed. That code stays ungoverned and is not connected
  to the shipped dialog.

**Preview memory allowance.** (iii) needs no webview memory allowance: the worker runs under the
existing per-invocation budget. R and the output ceiling are new MBF product-route limits on
payload size; they set no webview allowance and do not change the per-invocation budget. (i) is not chosen, so no webview allowance is set.

#### 3.9.6 Planned tests (not run)

Slice 4b.2a runs the Rust worker tests below that need the capped worker, in a real worker
process (`mbf_worker_tests.rs`, §6): review mode against the recorded reviews and digests, the
truncated review blocking the save, the output ceiling refused before any result lease, a newer
review killing the running worker, and a budget raised while a review waits. The Vitest cases
below run (slices 4b.1 and 4b.2b). The dimension-table tests (Q1–Q5 in a worker) and the at-limit
save of a 64 MiB result remain planned.

- **Confirming the dimension table.** These run in the engine crate, natively, with the counting
  allocator. Each doubles one factor with the others held, and expects the excess over the base
  shape to double:
  - Q1 on `long_model(480)` with a 20,000-byte case id, and on 1 MiB of refused loads
    (`{"category":0}`) under a 16 KiB case id;
  - Q2 on a node with a 16 KiB id and 1,024 unknown fields, and on a pipe with the same;
  - Q3: a material with a 16 KiB id and no expansion coefficient, and 1,000 thermal loads on its
    pipes;
  - Q4: a bend with a 16 KiB component id, and 1,000 thermal loads on its pipe;
  - Q5: a first sustained case with a 16 KiB id, followed by 1,000 sustained cases.

  A control doubles every id's length on a shape whose ids are named only at each reference. Its
  peak must grow at most linearly in M. These tests confirm the analysis; under (iii) they set no
  limit.
- **Rust, the worker.**
  - Review mode returns no MBF text and no sidecar bytes, and opens, creates or changes no file.
    Its digests equal save mode's and the recorded `export_digests.json` cases.
  - The child refuses an unknown `mode` and a field of the other mode; each mode's result has its
    fixed shape.
  - For each Q shape above, the worker either completes within its budget, with a review of at
    most R in total and the overflow counted, or ends with a named worker refusal that becomes
    `MBF-REVIEW-MEMORY-UNAVAILABLE`. In both cases the host's counted peak stays within the
    ingress lease plus R plus the frame header.
  - A truncated review of a blocked export still reports the blocked status and its global
    refusal, and its total and blocking counts are exact.
  - A truncated review blocks the save even when its blocking count is 0: the `save`-mode run
    refuses `MBF-REVIEW-TRUNCATED` before any result lease, chooser or write, whatever the
    request claims about the review.
  - A string longer than the per-field bound is carried as its prefix, byte length and SHA-256,
    and the worker's counted peak shows no copy of the whole escaped string.
  - A diagnostic and its loss entry are listed together or counted together.
  - A save result above 64 MiB is refused with `MBF-EXPORT-OUTPUT-TOO-LARGE` before any result
    lease is admitted, opens no chooser and writes nothing; one at the limit is saved. The test
    runs with a grant above the ceiling: `result_limit` is checked after the grant and footprint
    checks (`solve_worker.rs:1201-1226`), so a smaller grant fails first as memory unavailable.
  - A newer review kills the running worker, and its grant is released after the reap.
  - A budget changed while a review waits does not extend it.
- **Vitest.**
  - The dialog calls `review_mbf_export` and never `renderMbfJson` (a spy on the engine).
  - In the browser-only route the review declines with `MBF-REVIEW-NATIVE-ONLY` and Save stays
    disabled.
  - Complete or truncated, the counts not shown and the blocking counts are displayed.
  - A truncated review disables Save and shows `MBF-REVIEW-TRUNCATED`, also when its blocking
    count is 0; "You can still save" appears only for a complete, eligible review.
  - A memory refusal and the output-ceiling refusal are shown by name.
  - A stale review after `MBF-EXPORT-REVIEW-STALE` runs again.

#### 3.9.7 Decided (Agent 0, 2026-11-02)

- **(iii)** is chosen. The browser-only route declines with `MBF-REVIEW-NATIVE-ONLY`, and the wasm
  writer stays only as an unconnected or test resource.
- **§3.3** lists entries within the 1 MiB bounded review and counts the rest with how many block.
- **The 4b worker proposal** gains task 3's strict `review`/`save` modes and the review result.
  Its envelope's byte layout is to be agreed with performance (#59).
- **The save's outputs** are bounded by the 64 MiB output ceiling (§3.7), because the identity
  file carries Q1 and so cannot be priced from M.
- **The import slice.** The import (§4.7) runs the same regeneration. It needs its own design
  before connection (§6, slice 5); this section does not analyse it. That design is §4.8, and
  none of this section's proofs is carried over to it.

## 4. Import (RT-R8)

**Status of this section.** §4.1, §4.3 and §4.5 to §4.8 are slice 5's source, memory and transaction design
(2026-11-07, drafted on `main` at `b06e1da32`). They replace the earlier wasm-route text of §4.1 and §4.7, which is
not adopted (§4.8.1). The design takes independent review (§4.8.10) and is merged before any production connection.
§4.9 records the implementation (slice 5's PR): what was built, the refinements made within this design, and which
§4.8.9 checks were implemented, how, and which were not. §4.2 and §4.4 are unchanged.

### 4.1 Flow

1. **Entry point.** File > Import MBF… (`file.import-mbf`) follows Open Model Document… in the native File menu
   (`lib.rs` `NATIVE_FILE_MENU`, disabled while a project operation is busy, like its neighbours) and in the in-window
   File menu (`App.tsx`). Both dispatch through `runMenuCommand` to `requestImportMbf` (`workspaceSession.ts`). It does
   nothing while `projectSessionBusy()` holds. In the browser-only route it is refused with `MBF-IMPORT-NATIVE-ONLY`
   ("Importing a model batch file needs the desktop app.") before anything is sent.
2. **Choose and read.** `requestImportMbf` mounts the import dialog (`MbfImportDialog.tsx`, mounted by `App.tsx`). The
   dialog is modal like the export dialog: `mbfImportDialogOpenRef` joins every list that holds project commands
   (`projectSessionBusy`, the menu, accelerator and close-request holds). The dialog calls `open_mbf_import` (§4.7)
   through the import barrier (§4.8.6). Native code opens the read-only chooser (`.mbf`). It opens the chosen MBF
   and, when one is present, its identity file `<stem>.swbpipe-ids.json` beside it, checks and reads both, and
   records the source (§4.8.2). It then runs the first review in worker task 5 (§4.8.3). If the user dismisses the
   chooser, the dialog closes and nothing changes.
3. **Review** (§4.3). The dialog shows the bounded review the worker formed. No file bytes, base64 copy, model,
   import report or id map reaches the webview during review.
4. **Declarations.** When the review lists materials that need a provenance declaration (`declaration_needed`), the
   dialog asks for each listed material, or for all listed materials at once: "project-invented example data", or
   "my own source: [name]" with a non-empty name. Vendor-library and unknown origins cannot be declared, so such a
   material keeps the import refused. "All listed materials" sends one entry per listed name. It never sends the
   engine's `*` key, which would conflict with an origin a bound identity file records (import Design §7.4, "A
   recorded origin is the material's"). Each change sends `review_mbf_import` with the declaration. Native code
   re-reads the chosen files through their pinned handles, refuses any change, and runs task 5 again. Adopt is
   disabled while a review is pending, refused or stale.
5. **Adopt.** Adopt is enabled only when the review shown is current, `complete` and `adoptable`, and was formed for
   the declaration the form now holds. Adopt releases the dialog's modal hold. If another project operation has
   started meanwhile, it refuses with "Import not adopted: another project operation is in progress." Otherwise it
   hands the request's one retirement to the import flow, so closing the dialog sends none (§4.8.6). It then
   calls `handleOpenModelDocument(mbfImportSource(...))`, a `ModelDocumentSource` (`action: "Importing a model batch
   file"`, `operation: "import_mbf"`). That function then applies its own steps:
   - the busy check;
   - the unsaved-changes guard, whose Cancel keeps the edits (`open_model_kept_edits`);
   - the project-operation owner;
   - `source.read()`, which is `adopt_mbf_import` (§4.8.7) followed by `checkNativeOutcome` (the desktop shape check
     and the document validator through the loaded wasm engine);
   - `stillCurrent()`;
   - `buildModelIndex`;
   - the synchronous commit.

   The model starts unsaved, with no stored base. New Local Project saves it, and the first native save checks it
   again.
6. **Afterwards.** Once `handleOpenModelDocument` returns, the import flow retires the source with a keyed
   `cancel_mbf_import`, whatever the outcome. The status line after a landed import names the file, the binding and
   the restored identity counts from the review the user adopted. The import report and id map are not shown or
   persisted (import Design §16.4); the review carries their bounded summary (§4.3).

REQ-017's "only through the existing document open path" holds. The document enters through `handleOpenModelDocument`
and `checkNativeOutcome`, exactly as Open's does. Before that, the worker runs the native half of the open path's
checks (`check_model_document`: kind, version and the W-13 validator) over the same bytes (§4.8.3). Nothing is written
to disk before a native save.

**What changed from the earlier text, and why.**

- *Old:* "The importer runs again in wasm with the declaration. The files are not read again." The earlier text also
  had `open_mbf_file` return both files to the webview as base64 bytes, and ran the importer and its writer-backed
  regeneration in webview memory.
- *New:* the importer runs only in the capped worker. Each review and each Adopt re-reads the chosen files through
  their pinned handles, and refuses a change.
- *Why:* §4.8.1 has the full assessment. In short, the wasm route puts the importer's working set, including the
  writer that regeneration runs (§3.9.2's super-linear terms), into ungoverned, never-shrinking webview memory, and
  it puts up to 80 MiB of file bytes into the webview several times over. Retaining the bytes natively between
  commands would need a long-lived pool custody that no command ledger owns. Re-reading keeps no model-sized custody
  between commands, and SHA-256 proves that every run imports exactly the bytes the user chose.
- *Guard placement:* the unsaved-changes guard now runs at Adopt, not before the chooser. A review touches nothing in
  the session, and the guard means what it meant before: nothing is replaced without the user's yes, and Cancel keeps
  the edits.

### 4.2 Coverage

- **Writer output.** The importer reads exactly the writer's output subset and the import
  Design §14 extension (table in §1.4).
- **Files edited or written elsewhere.** It reads only the documented records and refuses
  every other one by name.
- **Not yet in the desktop.** These options stay unavailable until a later slice (§6, slice
  7):
  - Unbound pressure files need a closure declaration, so they are refused with
    `MBF-IMPORT-PRESSURE-CLOSURE-REQUIRED`.
  - Rebinding by node name (`rebind_by_node_number`) is not offered.

### 4.3 Import review (bounded)

One engine function, `import_review(result, source, declaration_sha256, document_check, allowance)`, forms the review
inside the worker. It lives in `core/handoff/caepipe_mbf_engine/src/import_review.rs`, beside `export_review.rs`, and
uses the same mechanics as the export review (§3.9.5).

- **Format.** ASCII JSON, `swbpipe.mbf_import_review`, format 1. The whole review is at most R = 1 MiB after escaping
  (`MBF_REVIEW_MAX_BYTES`, reused, not raised). A string longer than the per-field bound (`REVIEW_FIELD_MAX_BYTES`,
  512 bytes) is carried as `{prefix, bytes, sha256}`, formed in place.
- **Fixed leading members**, in this order and byte form, so the host can check them without parsing:
  `{"document_kind":"swbpipe.mbf_import_review","format":1,"status":"<imported|refused>","adoptable":<true|false>,
  "model_sha256":<"hex"|null>,"model_bytes":<n>,`.
- **Status first.** The importer's status, `adoptable` and the blocking state are computed over the full result
  before any bound applies. Overflow never makes a refused import adoptable.
- **`adoptable`** holds when the importer's status is `imported`, the worker's document check passes (§4.8.3), the
  review is `complete`, and the adopt result is within the transfer limit (§4.8.4).
- **Source.** The file name (the `source_label`), the MBF's SHA-256 and byte count, and the identity file: `absent`,
  or present with its SHA-256 and byte count.
- **Binding.** Bound, or unbound and why: no identity file; an identity file for other bytes
  (`MBF-IMPORT-SIDECAR-NOT-APPLIED`); or a refused identity file (for example `MBF-IMPORT-SIDECAR-DIGEST-MISMATCH`,
  `MBF-IMPORT-SIDECAR-INVALID`, a sidecar format the engine does not read included).
- **Declaration.** `declaration_sha256`, the SHA-256 of the declaration bytes the worker was given, or null. The list
  `declaration_needed` holds the MBF material names refused `MBF-IMPORT-MATERIAL-PROVENANCE-REQUIRED`. The count of
  materials whose origin a bound identity file recorded is carried as well.
- **Refusals** (block). Each with its code, MBF line and entity, and no message text and no model values.
- **Loss entries** (do not block). Each with its code, entity and category (`omitted`, `approximated`, `delegated`,
  `unsupported`, `tbd`).
- **Target options.** Their names, marked "Recorded, not applied" (REQ-015).
- **Identity.** Per class of A-3's identity set: the restored count, the count formed from MBF names, and the
  unrecoverable list, all computed from the id map. An unbound import says "ids formed from node numbers". The
  implementation names the id-map members it reads and pins them in a test with one bound and one unbound fixture
  (§4.8.9).
- **Units.** The file's unit mode and angle option, and the project units the document receives, as the import
  report records them (import Design §5.2). A member the report does not carry is stated as not available, never
  recomputed.
- **Model.** `model_sha256` and `model_bytes` (the canonical model text), the document check's outcome (`passed`, or
  its codes), the transfer limit (`within` or `beyond`) and `schema_status`.
- **Completeness.** `complete` or `truncated`, with the counts not shown, how many of them block, and the same counts
  per list.
- **Mapping assumptions (REQ-015).** An `assumptions` list carries every applied mapping assumption as a stable
  identifier and structured parameters, rendered with source-safe product descriptions. These facts are recorded
  alongside the existing assumption emissions, under the same conditions in `importer/finish.rs` and
  `importer/pressure.rs`; they are not guessed from diagnostic codes or parsed back from English sentences.
  The existing full-report assumption strings and serialized Python/Rust parity outputs are unchanged.
  - Coverage includes the input units and angle mode; Y axes or the exact Z-to-Y rotation; exact offset summation
    and one binary64 rounding per node; English-to-SI re-export normalization; bare-metal-only weight and its unit
    conversion; G derivation and temperature-independent material handling; thermal differences and zero-field
    handling; applied pressure-region, closure and shear choices; support/spring mappings; earth-weight direction;
    binding/profile conventions actually applied; and the absence of created combinations. Conditional parameters
    preserve the source meaning rather than replace it with a generic success statement.
  - The worker bounds these entries, parameters and descriptions within R, using the same per-field and aggregate
    completeness accounting as the other lists. An assumption counted but not fully represented makes the review
    truncated and blocks Adopt. A new or unmapped assumption identifier refuses adoption with
    `MBF-IMPORT-ASSUMPTION-UNAVAILABLE`; it cannot silently disappear from a complete review.
  - Maintained tests exercise each emission branch, require every emitted full-report assumption to have its
    structured counterpart, compare the represented choices with expected mappings, and hold the existing parity
    bytes unchanged. This metadata is desktop-review support, not another numerical interpretation or importer.
- **Not carried.** Raw engine message texts and raw assumption sentences. Several state facts about the target
  program. The dialog instead shows the codes and structured assumption identifiers with product descriptions from
  `MBF_IMPORT_DESCRIPTIONS` and `MBF_IMPORT_ASSUMPTIONS` in `mbfImport.ts`. Their tests cover every emitted diagnostic
  code and assumption identifier, including the meaningful conditional parameters above. Product descriptions state
  SWBPIPE's mapping behavior and preserve the applied assumptions without introducing new target-program claims.
- **Deterministic.** The review contains no token, epoch, time or path. The same bytes, file name and declaration
  give byte-identical reviews (REQ-017), so `review_sha256` binds the source digests, the declaration, the model
  digest and everything the user saw.
- **A truncated review blocks Adopt** with `MBF-IMPORT-REVIEW-TRUNCATED`. Every refusal and loss entry must have been
  shown before adoption (REQ-013, REQ-015). This is a stated usability limit of the route, which the owner has not
  accepted as a release limitation.
- **Boundary.** "A successful import means the listed subset of this file was read. It is not evidence that two
  programs analyse the model alike."

### 4.4 Entity identity

- **What a bound import restores.** Ids from `node_map`, `element_map` (including bend
  components), `support_map`, `load_case_map`, `material_map`, `load_map` and the
  `pressure_map` regions, and with sidecar format 4 from `section_map`: the library sections
  the pipes reference, and each pipe's `section_ref`.
  - **Sections (format 4, implemented; Q-2).** The writer writes a PIPE record per library
    section (or none) and OD and wall, so two sections with equal OD and wall stay apart, and
    records each record's section id, or null, in `section_map`. A restored section has the
    record's OD and wall, type `pipe`, the record name as its name and the record's line as
    its provenance; its original name and provenance are the writer's omitted loss entries.
    Sections no pipe references are omitted. Formats 3, 2 and the legacy format keep their
    handling (no library is restored), and the self-digest is unchanged. Design: import
    Design §15 **Format 4: section identities**.
- **What it does not restore:**
  - **The project.** The import forms `project:mbf-<12 hex>`, so an import is a new unsaved
    document, not the original project. The project id is outside the identity set because it
    identifies the store's project, not a model entity, and an import must not collide with
    it (A-3).
  - **Unread members.** Lines, constraints, equipment interfaces, rule-pack references,
    unresolved assumptions and `analysis_status` are not written. The writer states this
    only as one assumption line (Q-6).

### 4.5 Identity file binding (#1685)

- **Bound.** Binding is by digest only (import Design §15). The identity file's `mbf_sha256`
  must equal the MBF's SHA-256, and its `sidecar_sha256` must verify before anything reads it.
- **Never assumed.** The desktop never treats an identity file as bound because of its name or
  folder.
- **Refused, not set aside.** An identity file that is present but fails the file-level checks
  (§4.6) refuses the import. It is not silently set aside, so an import is never unbound
  without the user seeing why.
- **Moved MBF.** An MBF moved without its identity file imports unbound, and the review says
  so.
- **Supported engine subset only.** The desktop passes the engine's `ImportOptions` with a sidecar, a material
  provenance declaration and the source label only. It passes no pressure closure, no rebinding (slice 7), no shear
  model and no source document. A sidecar format the engine does not read, format 5 included (import Design §20.4),
  is the engine's `MBF-IMPORT-SIDECAR-INVALID`. Nothing here adds a format-5 or designated-source route, a Route 1
  input, an unsupported-family exemption or a value change. Supplier Route 1 stays held.

### 4.6 Refusals

Every refusal leaves the session unchanged. The dialog shows each code with a product text.

| Where | Codes |
|---|---|
| Before the chooser, at entry | `MBF-IMPORT-NATIVE-ONLY` (webview); `MBF-IMPORT-EPOCH-INVALID`, `MBF-IMPORT-EPOCH-EXHAUSTED`, `MBF-IMPORT-CANCELLED` (the ticket, §4.8.6); `MBF-IMPORT-ARGS-INVALID` (an unknown or missing member, a path member included); `MBF-IMPORT-RETIREMENT-ID-INVALID` and, in the webview, `MBF-IMPORT-RETIREMENT-UNCONFIRMED` (retirement, §4.8.6); the ingress refusals (`INGRESS-*`) |
| At the chosen files (`path` `mbf` or `identity_file`) | `MBF-OPEN-NOT-A-FILE` (a symlink, directory or other non-regular file); `MBF-OPEN-TOO-LARGE` (MBF above 16 MiB, identity file above 64 MiB); `MBF-OPEN-UNREADABLE` (open, stat or read failed); `MBF-OPEN-NAME-INVALID` (a file name that is not UTF-8 or is longer than 1 KiB) |
| At a re-read | `MBF-IMPORT-SOURCE-RETIRED` (the source id is not the current source); `MBF-IMPORT-SOURCE-CHANGED` (§4.8.2) |
| Declaration | `MBF-IMPORT-DECLARATION-INVALID` (not an object of MBF material names to `{origin, source_name}` with `origin` `project_invented` or `user_supplied` and a non-empty `source_name` of at most 512 bytes; or above the argument bound) |
| Worker and memory | `MBF-IMPORT-MEMORY-UNAVAILABLE` (a refused bound, grant, growth, footprint, request copy, wait or reservation); `MBF-IMPORT-WORKER-PROTOCOL`; `MBF-IMPORT-WORKER-REQUEST-INVALID`; `MBF-IMPORT-ENGINE-FAILED` (the engine returned no result) |
| Importer | Every importer refusal by its own code (import Design §4 to §15), carried in the review |
| Adopt | `MBF-IMPORT-REVIEW-STALE` (the reviewed digest differs); `MBF-IMPORT-REVIEW-TRUNCATED`; `MBF-IMPORT-ASSUMPTION-UNAVAILABLE` (an applied assumption lacks its bounded source-safe representation); `MBF-IMPORT-MODEL-TOO-LARGE` (the transfer limit); `MBF-IMPORT-NOT-ADOPTABLE` (the review's blocking codes, listed); then `checkNativeOutcome`'s and `buildModelIndex`'s refusals, as Open shows them |
| Transport | `INGRESS-RAW-BODY-REQUIRED`: "Import isn't available in this window. Nothing was imported and your model is unchanged. Reload the window or restart SWBPIPE to import again." Import writes nothing, and the session changes only on a reply the page received, so this outcome is known (§4.8.6) |

### 4.7 Native commands

Four commands are registered in `app_invoke_handler!`. None takes a path or file bytes.

| Command | Arguments | Reply |
|---|---|---|
| `open_mbf_import` | `{expectedEpoch, requestSequence}`, an `args-json-v1` raw body of at most 4 KiB | `{outcome: "cancelled"}`; `{outcome: "reviewed", sourceId, fileName, reviewSha256, review}`; or `{outcome: "refused", fileName, diagnostics}` |
| `review_mbf_import` | `{expectedEpoch, requestSequence, sourceId, declaration}`, raw body of at most 1 MiB | `reviewed` or `refused`, as above |
| `adopt_mbf_import` | `{expectedEpoch, requestSequence, sourceId, declaration, reviewSha256}`, raw body of at most 1 MiB | `{outcome: "opened", file_name, byte_count, schema_status, document, warnings: [], reviewSha256, modelSha256}`, the `NativeModelDocumentOutcome` shape `checkNativeOutcome` takes; or `{outcome: "refused", file_name, diagnostics}` |
| `cancel_mbf_import` | `{retirementId: {page, sequence}}`, as Tauri's JSON arguments or as a raw body (§4.8.6) | `{cancelled, epoch, retirement_id}`, with `duplicate` or `exhausted` only when true |

- **Arguments.** The starts (`open`, `review`, `adopt`) are raw-body only, admitted in C1's form as the export's are
  (§3.7, "Admission"). An unknown member is refused `MBF-IMPORT-ARGS-INVALID` before anything else is read from the
  arguments. `declaration` is `null` or the object of §4.6. `sourceId` is a positive integer.
  `requestSequence` is a positive safe integer: the host rejects duplicates/older dispatches within the current
  retirement epoch and rechecks the accepted sequence at registration (§4.9.2).
- **Replies.** The `review` member is the worker's review bytes, embedded verbatim (at most R). A refusal carries at
  most 8 diagnostics, each `{code, path, message}` with a message of at most 2 KiB (at most 32 KiB in all). Native
  message texts are not shown: the dialog maps each code to its product text.
- **The pin.** The commands change `app_invoke_handler` and are re-pinned as in §3.8.

### 4.8 Source, memory and transaction design (slice 5)

Paths are relative to `apps/desktop/src-tauri/src/` unless named otherwise. The source reading behind this section is
on `main` at `b06e1da32`. Counts and bounds are from reading source. §4.8.9 plans the tests that confirm them;
§4.9.3 records which were implemented.

#### 4.8.1 Alternatives assessed

| Option | What actually happens | Verdict |
|---|---|---|
| **(a) §4's wasm route.** The MBF and identity file are sent to the webview as base64; `caepipe_mbf_wasm::import_mbf_json` runs per declaration | The webview holds each file as base64, as a decoded `Uint8Array` and as a copy in wasm memory: up to (16 + 64) MiB × about 3.3. `import_mbf_outcome` builds the whole `ImportResult` (model, diagnostics, loss report, id map) and returns `model_json`, `report_json` and `id_map_json` inside one JSON string, which the webview parses again. A bound import's `regeneration_identity` (`importer/build.rs:404`) calls `SiWriter`, whose `regenerate_caepipe_sidecar` (`writer/mod.rs:640`) runs `Writer::run` over the imported model and clones the sidecar for comparison. The writer's super-linear terms (§3.9.2) therefore run in wasm memory, which never shrinks (§3.9.1), and §3.7 has already disproved a k·input bound. The export's governance proof covers the native export worker, not this route | Not adopted. It is ungoverned. The wasm `import_mbf_json` stays only as an unconnected test resource, as `render_mbf_json` does, and no product source imports it (a source scan holds this) |
| **(b) Native worker; host retains the file bytes between commands** | Up to 80 MiB plus its envelope would be held from Open until retirement, across commands and across page reloads until a keyed retirement reaches the host. Each `CommandLedger` belongs to one command (DEL-14-01 r4), so this needs a new long-lived pool custody that no ledger owns, whose release depends on IPC delivery | Not chosen. It is a new memory-custody class outside the adopted pool policy (DEL-04-07), and changing that policy is not authorized here |
| **(c) Native worker; host retains the imported model and report** | Their size follows the importer's working set, not the input, so it cannot be priced from M | Rejected for the reason of (b), and worse |
| **(d) The webview keeps the file bytes and re-sends them** | Raw file copies in the webview, re-sent at every declaration change, and model-sized raw bodies that Tauri's postMessage fallback cannot carry | Rejected |
| **(e) Low custody: pinned handles, re-read under each command's leases** | The host keeps only a fixed-size source record (§4.8.2). Every model-sized byte lives inside one command's ledger and is released before its reply is returned. SHA-256 proves that each re-read is the selected bytes | **Chosen** |

#### 4.8.2 Source state

- **Managed state** (`mbf_import.rs`, the new MBF import module): `MbfImportState { epoch, latest_sequence, exhausted, applied, run,
  source, next_source_id }`.
  - `applied` holds the retirement page tokens, at most `RETIREMENT_PAGES_MAX` = 64, as for the export.
  - `run` holds at most one import run in progress.
  - `source` holds at most one source.

  It is separate from the export's `MbfRuns`. An import retirement never cancels an export run, and an export
  retirement never cancels an import run. Neither touches the performance reset.
- **The source record.** One `Source { id, entry_epoch, file_name, mbf: Pinned, identity: IdentityFile }`.
  - `Pinned { path, handle, identity, size, sha256 }`. `path` is the chooser's path and is never sent to the
    webview. `handle` is the open file. `identity` is the device and inode on Unix, or elsewhere the size and both
    times, each present (§3.6 step 8's rule). `size` and `sha256` are as read at selection.
  - `IdentityFile` is `Absent { expected_path }` or `Present(Pinned)`, where `expected_path` is the derived
    `<stem>.swbpipe-ids.json` (A-5).
  - Its size is fixed: two paths of the platform's length, two file descriptors, two digests and a few integers. It
    is not charged to the pool, as the export's 64 page tokens are not. There is at most one source per process, so
    the native registry is bounded.
- **Source id.** A `u64` taken from `next_source_id`, starting at 1 and advanced by `checked_add`. Overflow makes
  the import exhausted, as epoch overflow does. The id is opaque and authorizes nothing (§4.8.6, "Not
  authenticated").
- **Selection** (`open_mbf_import`, after the chooser).
  - `lstat` the chosen path. It must be a regular file and not a symlink. It is then opened read-only with no
    following of a final symlink and with nonblocking acquisition (`O_RDONLY | O_NOFOLLOW | O_NONBLOCK` on Unix,
    through safe `OpenOptions` flags). The acquired handle's `fstat` must itself report a regular file, with
    identity equal to the `lstat` identity and size at most 16 MiB, before any read is attempted. Replacing the
    checked regular file with a FIFO cannot make the open wait for a writer: the acquired non-regular handle is
    refused and dropped. Cancellation is checked before acquisition and immediately after it, before the handle
    can be installed or read. This removes that FIFO wait; it is not a wall-clock guarantee for arbitrary
    filesystem/kernel latency.
  - `lstat` the derived identity-file path. If it is not found, the identity file is `Absent`. If it exists, it is
    checked and opened the same way with a 64 MiB cap (`MAX_MODEL_DOCUMENT_BYTES`). A failure refuses the import by
    name (§4.5, "Refused, not set aside").
  - Each file is read into the run's request buffer (§4.8.4) and hashed, and the digests are recorded.
- **Install.** The source is installed under the state lock only if the run is not cancelled and `entry_epoch` is
  still the current epoch. Installing replaces the previous source and drops its handles. Otherwise the new handles
  are dropped and the reply is `MBF-IMPORT-CANCELLED`.
- **Re-read** (each `review_mbf_import` and `adopt_mbf_import`).
  1. Under the lock, the request's `sourceId` must equal the current source's id, and the source's `entry_epoch`
     must equal the current epoch. Otherwise the run refuses `MBF-IMPORT-SOURCE-RETIRED`. The run duplicates the
     handles (`try_clone`) and copies the record, then leaves the lock. It reads by position (`read_at` on Unix,
     `seek_read` on Windows), so a superseded run still reading can never move another run's offset.
  2. For each pinned file, the path's `lstat` must still give a regular non-symlink file with the recorded identity
     and size. The run reads exactly `size` bytes in 64 KiB blocks into the request buffer, checking cancellation
     between blocks. A one-byte read at `size` must return 0. The SHA-256 of the bytes read must equal the recorded
     digest.
  3. If the identity file was `Absent`, `lstat(expected_path)` must still report not found. An identity file that
     appeared since is a change.
  4. Any difference refuses `MBF-IMPORT-SOURCE-CHANGED`: a file modified in place, replaced, renamed away, removed,
     grown or shrunk, an identity file that appeared, or a symlink or other non-regular file at a name. The refusal
     says "The chosen file or its identity file changed since it was chosen. Choose it again." The source stays
     installed, so the change is refused again until a new Open or a retirement.
- **What the checks prove.** The worker imports exactly the bytes in the request buffer, and their digests equal the
  bytes selected, so what is imported never depends on a race. The name checks are there to be truthful to the user.
  A rename that races the checks can at worst refuse; it cannot change what is imported.
- **Release.** The source is dropped, closing its descriptors, by: a retirement (§4.8.6); a new Open that installs
  a source; exhaustion; and process exit. Nothing else holds it.

#### 4.8.3 Worker task 5 (`WorkerTask::MbfImport`)

- **Code.** `solve_worker.rs` gains `WorkerTask::MbfImport`, request code 5, in `task_code` and in the request's
  decode. Codes 0 to 4 (sparse and dense solves, model checks, task 3 `MbfRender`, task 4 `HangerSizing`) are
  unchanged, and 6 and above are refused.
- **The `workload` arm** returns `(mbf_import::worker_import(&raw)?, None)`. Its `ATST` frame has status 0, followed
  by `RSLT`. Before arm, only `resident_envelope(raw_len) + reserve` and the raw read run; parsing happens after
  arm, as for task 3. `supervise` is unchanged.
- **Request** (little-endian): `u32 mode ‖ u32 label_len ‖ label ‖ u32 declaration_len ‖ declaration ‖ u32 mbf_len ‖
  MBF bytes ‖ u32 identity_len ‖ identity bytes`.
  - Mode 1 is `review` and mode 2 is `adopt`.
  - `label_len` is at most 1 KiB of UTF-8.
  - `declaration_len` is 0 for none, or at most 1 MiB of JSON.
  - `mbf_len` is at most 16 MiB.
  - `identity_len` is `0xFFFFFFFF` for absent, or at most 64 MiB.
  - Anything else is refused `MBF-IMPORT-WORKER-REQUEST-INVALID`.
- **What the worker runs.**
  1. It parses the declaration with the engine's reader (`parse_sidecar_json`) and builds `ImportOptions::new(label)`
     with `sidecar: Some(SidecarInput::Json(bytes))` when the identity file is present and `material_provenance`.
     Every other option stays at its default (§4.5).
  2. It calls `mbf::import`. A bound import's regeneration runs the writer here, inside the grant.
  3. For an `imported` result it forms `model_bytes()` and reads them with `serde_json`. It runs
     `model_document_file::check_model_document` (kind, version, W-13 validator) on them; a blocking finding makes
     the review not adoptable, with its codes.
  4. It forms `import_review` and disposes of the `ImportResult` without recursion (`value::dispose`).
  5. It forms the envelope.
- **Envelope** (ASCII): `MBFI1\n<mode>\n<import_status>\n<adoptable>\n<model_sha256 or ->\n<review_sha256>\n<model_bytes>\n<model_len>\n<review_len>\n`,
  then the model bytes, then the review bytes.
  - `import_status` is `imported` or `refused`; `adoptable` is `0` or `1`, and can be `1` only for `imported`.
  - `model_bytes` is the candidate canonical model length, distinct from the carried `model_len`. With no
    candidate it is 0 and the model digest is `-`.
  - `review` mode always has `model_len` 0, even when a candidate model has been digested in the worker.
  - `adopt` mode carries the model only when `adoptable` is `1`, with `model_len == model_bytes`; otherwise it
    carries the review only and `model_len` is 0.
  - The worker decides the transfer limit from the computed lengths before it builds the result (§4.8.4).
  - A failure is `<code>: <message>` of at most 64 KiB, with no engine text.
- **Decode** (`decode_import_result(mode, text)`, host). It reads the envelope as views and checks:
  - the header as borrowed slices, bounded before any derived storage: fixed magic, one-byte mode/flag,
    at most eight bytes for import status, at most 20 digits per canonical decimal length (checked `u64` parse),
    and exactly 64 lowercase hexadecimal bytes for each SHA-256. Only the model-digest field permits the single
    `-` sentinel, and only when there is no candidate (`model_bytes == 0`). The review digest never permits it.
    An overlong or malformed field is a protocol refusal before any field is copied;
  - canonical decimal lengths that exactly cover the bodies;
  - the mode, the import status and adoptable flag, and the mode's fixed payload shape;
  - a review of at most R;
  - the review's fixed leading members, byte for byte, by streamed comparison or a fixed 1 KiB stack buffer using
    only the validated fields (`status` from `import_status`, the adoptable flag mapped to JSON `false` or `true`,
    `model_sha256`, `model_bytes`). No model/header-sized `String`, `Vec` or formatted prefix is allocated outside
    H6/H7. The fixed prefix and numeric/hash scratch are covered by the separately admitted H8 control term;
  - the SHA-256 of the review view against `review_sha256`;
  - only in `adopt` mode with `adoptable == 1`, a carried model view that is all ASCII (the canonical bytes are
    `ensure_ascii`) and whose SHA-256 equals `model_sha256`;
  - in `review` mode, an empty model payload and valid candidate metadata. A positive `model_bytes` and its
    digest describe the candidate computed inside the worker; they are not a claim that the model is carried.
    The empty payload is never hashed as though it were that candidate. Non-adoptable `adopt` results likewise
    carry no model, as specified above.

  Anything else is `MBF-IMPORT-WORKER-PROTOCOL`. The host never parses the model or the review.

#### 4.8.4 The command's ledger and its copies (host)

Each start takes C1's form (§3.7, "Admission"):

- the raw body is copied under a capped interactive lease (`mbf_worker::reserve`'s pattern), which seeds the
  command's `CommandLedger` through `NativeLeaseMemory`;
- the budget is captured then, as min(the lease's budget, 12 GiB);
- `command_ingress::admit_raw` grows the ledger for the arguments' parse;
- every later lease is reserved in that ledger before the pool admits it (DEL-14-01 r4).

The model-sized copies, all within that one ledger:

| # | Copy | Bound | Charged as |
|---|---|---|---|
| H1 | The raw arguments | 4 KiB (open) or 1 MiB | The seeding lease |
| H2 | The arguments' parse | `admit_raw`'s bound for H1 | The ledger's growth |
| H3 | The declaration, re-serialized for the worker after its shape check | At most H1 | A named lease |
| H4 | The request buffer. The files are read straight into it, with no other read buffer and constant hashing state | `allocation_envelope(raw_len)`, where `raw_len` is the request's exact length from the recorded sizes, at most 16 MiB + 64 MiB + 1 MiB + 1 KiB + 20 | The bound's request share |
| H5 | The worker's grant | Up to the maximum grant: the captured budget less the ledger's other live leases, H4's share and the result share | The bound's grant share; released only after the reap |
| H6 | The worker's result | `review`: R + 4 KiB (`result_limit`). `adopt`: 64 MiB (`result_limit`, `MBF_IMPORT_RESULT_MAX_BYTES`, the same number as the export's output ceiling, not raised) | The bound's result share, `resident_envelope(limit)`; after the reap the bound shrinks to the result lease |
| H7 | The reply text | Its exact serialized length J, counted before allocation, including JSON escaping of the file name and all wrapper members: the review reply at most R + 8 KiB; a refusal at most 32 KiB; the adopt reply at most 64 MiB | A named lease of `envelope(J)`, reserved before the buffer is allocated (`serialize_metered`'s rule) |
| H8 | Decoder control scratch | Fixed 4 KiB, including the at-most-1-KiB prefix buffer, decimal formatting and hash state; no header-sized dynamic storage | A named lease before decode, held through response construction; a size/layout check holds the fixed scratch within it |

- **The bound.** One `LedgerBound` reserves H4, H5's maximum and H6's share before any of them is admitted. Above
  the budget it is refused `MBF-IMPORT-MEMORY-UNAVAILABLE` with the ledger unchanged. A starting grant
  (`initial_grant(reserve, resident_envelope(raw_len))`) above the maximum grant is refused before any worker starts.
- **Order.**
  1. The bound.
  2. The grant's queue wait: `wait_acquire`, at most 2 s (`MBF_START_WAIT`'s value), ended by cancellation or a
     newer run.
  3. H4's admission.
  4. The selection or re-read into H4 (§4.8.2).
  5. `supervise` with this run's hooks. The worker is reaped inside `supervise`, and only then is the grant
     released.
  6. Result custody: the bound shrinks to the result lease. A result lease larger than the bound's share is refused
     `MBF-IMPORT-WORKER-PROTOCOL` in every build.
  7. Admit H8, then decode. A refused H8 admission returns a memory refusal with no derived header allocation.
  8. The exact H7 length is counted over borrowed views, including the file name's escapes; an adopt reply above
     64 MiB is refused before any response buffer allocation. H7 is then reserved and written: the reply's fixed
     members, with the review or model view copied in once.
  9. The response takes ownership of H7's buffer; no application-side handoff clone is made. The result lease and
     the bound are held through response construction, then released, then H7's lease, then the command's other
     leases. An implementation that copies at handoff must admit that additional copy before making it.
- **Lifetimes.** H4 is handed to the supervisor, which drops it once it is sent. H5 ends at the reap. H6 and H7 are
  held together only from the reply's construction until the response exists. Between commands the import holds no
  pool or ledger charge, only the fixed source record (§4.8.2).
- **Policy boundary.** H7 and every application-owned handoff copy are admitted above. Once the response is handed
  to Tauri, its transport lifetime and internal IPC copies are the existing `respond_metered` residual in
  DEL-04-07 `memory-budget-decision.md`; this is not a bound on their heap. No import-only application buffer may
  be hidden in that residual. §4.8.5 compares the receiving consumer with Open.
- **Transfer limit.** Both the worker's adopt envelope (header + model + review) and the actual serialized native
  adopt reply (wrapper + model, with no review body) must fit 64 MiB. The worker computes both lengths, with checked
  arithmetic and actual JSON escaping, before forming its result. If either exceeds the limit the review reports
  `beyond`, so Adopt is disabled (`MBF-IMPORT-MODEL-TOO-LARGE`). `result_limit` enforces the worker-envelope limit
  again before any result lease, and step 8 enforces the final reply limit before allocation or delivery. The
  model's own exact L-byte canonical text must also fit `MAX_MODEL_DOCUMENT_BYTES` (64 MiB); no subtraction of the
  maximum R is substituted for these actual lengths. A model whose import exceeds a limit cannot be imported from
  the desktop.
  This is a stated route limitation; the owner has not accepted it as a release limitation.
- **What the worker's cap does and does not bound.** The cap enforces the admitted per-invocation budget on the
  importer, the regeneration's writer, the canonical model, the document check and the review. Nothing predicts
  their peak in advance (§3.9.5, "What (iii) does not bound"). A model beyond the budget is refused by name, and is
  still a valid file.
- **No budget, ceiling or pricing changes.** Every number here is an existing limit (64 MiB document cap, R, 512
  bytes, 2 s, 64 MiB result value) or a new import-route bound on a new command's payload: the 16 MiB MBF cap, the
  1 MiB argument bound and the 4 KiB open argument bound. No pool budget, worker ceiling or retained-route pricing
  moves, and no re-pricing registration is needed: the import touches no retained route (RT-A3 alone owns released
  Rust pricing).

#### 4.8.5 Webview copies and the existing Open-consumer boundary

**Policy application (Agent 0, 2026-11-07).** DEL-04-07 `memory-budget-decision.md` states that custody counts
delivered bytes, not webview heap, and records the common command-response and webview-heap residuals. Existing
Open is not a heap proof. The lack of such a proof is equally not a new slice-5-only gate. The new native reading,
importer, writer-backed regeneration, review, reply and application-owned handoff copies need their own admission
(§4.8.4). The delivered document must fit the existing Open domain and use exactly its maintained consumer, with
no additional unmetered import stage. This applies existing policy; it grants no allowance, multiplier, budget
change or risk acceptance. Whole-App/model-state heap qualification remains unresolved under RT-U4, and historical
v0.1 residuals do not establish owner acceptance for v1.0.

**During review.** Each serialized review reply is at most R + 8 KiB. The page keeps the current bounded review
and declaration form, and replaces rather than accumulates previous reviews. A superseded reply is discarded by
generation. This is additional bounded import-control data, not an inherited model consumer: its wire bytes are
admitted as H7, while its decoded graph is not represented as a proven JavaScript heap bound. No MBF bytes,
identity-file bytes, full import report, id map or candidate model reaches the webview. At Adopt the flow captures
only the source id, declaration, review/model digests, file name and bounded status summary; it releases the review
body and dialog's references before the model reply is requested. The flow does not close over the review object.

**At Adopt.** L is the exact canonical model byte count and J the exact serialized native reply byte count, both
checked natively against their limits before delivery (§4.8.4). These byte counts do not price WebKit objects. The
following inventory describes overlaps and ownership, not a measured or proven heap multiplier:

| # | Copy or work | Byte account, overlap and lifetime |
|---|---|---|
| W1 | Tauri's reply body and decoded reply string | J ≤ 64 MiB, admitted as H7 through native response construction. Transport-internal copies and the decoded string can overlap parsing. JavaScript code-unit lengths and UTF-8 byte counts differ; no claim of a J-byte heap follows. The import adapter retains no raw reply text after Tauri decodes it. |
| W2 | Tauri's one reply parse, including its document object | One candidate document graph, the same graph passed by reference through the source, validator and session. There is no import-side `JSON.parse(model_json)`, `structuredClone`, retained candidate cache or second full-model object. The old session model/history remain live until adoption, as for Open. |
| W3 | `checkNativeOutcome`'s `modelDocumentShapeDiagnostics` | The maintained shape walk over W2, with its diagnostics; no additional import shape walk. A refusal ends the candidate flow without session replacement. On success the returned envelope is shallow: its document is still W2. |
| W4 | `checkNativeOutcome`'s `modelDocumentSlotValidation` | Exactly the existing one wasm-validator call (`modelDocumentShape.ts`): `JSON.stringify(document)`, wasm input/parse, validator report string/parse and findings. These temporaries overlap W2; wasm linear-memory high-water capacity can remain afterwards. Stringified numbers/escapes can differ in length from L. No import validator call is added before or after it in the webview. |
| W5 | The maintained model-index work | `handleOpenModelDocument`'s uncached `buildModelIndex(document, 0, 0)` guard precedes session mutation; publication/rendering later use `modelIndexFor` and its existing cache. Their records refer to W2. The guard's temporary index is not retained by the import flow. Existing indexes/history can overlap the new graph until the shared consumer releases its references. |
| W6 | `commitModel` and ordinary model publication | `prepareModelPublication` calls `uiModelIdentityHash`, which stringifies the document synchronously before publication; that text is temporary. The model assigned to current-model refs and React state is W2, not a clone. Original session/history/result references are cleared by the existing open reset, and garbage collection timing remains the common residual. |
| W7 | The existing post-adoption consumer | `handleOpenModelDocument` runs `libraryReferenceWarnings` and its saved-basis `computeModelHash`; the existing `[model]` layout effect also calls `computeModelHash`. Hash calls can overlap while loading the wasm engine. `hashService.ts` stringifies and invokes the existing wasm canonical hash, with its input/parse/hash temporaries. The imported source adds no hash or library pass, and keeps no candidate text for them. |

**Source/call-path comparison.** Ordinary Open's `modelDocumentFile.ts::openModelDocumentFile` invokes the native
reader and calls `checkNativeOutcome`. The import's `mbfImportSource.read` invokes native Adopt, checks only its
bounded captured identity fields, then calls that same `checkNativeOutcome` once. Both return into the unchanged
`handleOpenModelDocument` transaction: currentness, index guard, publication and reset, then the existing library
warnings/hash work. The worker applies `check_model_document` before delivery, the same native kind/version/slot
checks Open applies, and the source preserves the engine model bytes and semantics. A model refused by the common
consumer is refused by import; the adapter does not default, normalize or bypass it.

**Remaining condition.** Independent review must establish this comparison against committed source, and the
implementation must hold it with connected tests (§4.8.9): the same document object enters the common consumer,
the same validation/index/adoption and failure behavior occurs, and no importer, report, file-byte work, additional
validator pass or retained extra model is introduced in the webview. Every genuinely additional import-only
allocation/lifetime needs an explicit account; it is not inherited merely because Open also handles documents.
An unaccounted additional stage is repaired within current policy or returned with the exact required policy
change. No measurement campaign or new webview allowance is selected, and this comparison makes no claim about a
whole-WebKit or whole-App heap ceiling.

#### 4.8.6 Cancellation, retirement and duplicate deliveries

- **Runs.** At most one import run is in progress. A newer start of any kind cancels the older one: its wait ends,
  or its worker is killed and reaped before its grant is released. The run's control is the export's pattern (a
  `cancelled` flag and kill access to the worker while it is unreaped), without a commit, because import writes
  nothing. The run's control is checked:
  - after the chooser returns (an Open retired while its chooser was open installs no source);
  - between re-read blocks;
  - at the supervisor;
  - before the source is installed.

  A cancelled run replies `MBF-IMPORT-CANCELLED`.
- **The ticket** (`expectedEpoch`). Each start carries the import barrier's latest acknowledged import epoch. It is
  read from the admitted arguments before any other member (`MbfImportState::enter`):
  - below the current epoch: `MBF-IMPORT-CANCELLED`;
  - missing, malformed or future: `MBF-IMPORT-EPOCH-INVALID`;
  - exhausted: `MBF-IMPORT-EPOCH-EXHAUSTED`.

  It is compared again when the run registers. It authorizes nothing.
- **Retirement** (`cancel_mbf_import`). It is the export's keyed retirement (§3.7, "Idempotent retirement" and
  "Wire form and bounds"), applied to the import's own state:
  - `{page, sequence}` ids, each applied once; a duplicate or older delivery is a truthful no-op;
  - at most 64 page tokens, never evicted, and a new token beyond them exhausts the import until restart;
  - a checked epoch advance;
  - the JSON form, which survives Tauri's postMessage fallback, admitted beside the raw form through one keyed path,
    with the same 4 KiB measurement.

  An applied retirement cancels the run in progress and drops the source. The reply has no `too_late`, because
  import has no commit.
- **The id reader.** The id rule is the export's (`RetirementId`'s members and limits). `mbf_import.rs` reads it with
  its own reader, which refuses `MBF-IMPORT-RETIREMENT-ID-INVALID` with import wording, so `mbf_worker.rs` is
  unchanged.
- **Not authenticated.** As with the export, code in the webview could retire or exhaust the import. It gains no
  authority: a retirement only cancels and drops.
- **Retirement triggers** (the import barrier, `features/caepipe-mbf/mbfImportCancel.ts`). It has the export
  barrier's rules (§3.7, "The frontend barrier"):
  - serialized retirements;
  - one ticket per start for all its sends;
  - a generation check before every send;
  - a re-delivery of an unacknowledged retirement with the same id, once, then
    `MBF-IMPORT-RETIREMENT-UNCONFIRMED`;
  - a handshake retirement before the page's first start;
  - its own page token, made once per page load.

  It retires the import:
  - at workspace mount, page load included, so an old page's source is dropped;
  - at each project-session retirement;
  - at the dialog's Close, Escape or unmount, except after Adopt, which hands the request's one retirement to the
    import flow;
  - after Adopt's `handleOpenModelDocument` returns;
  - when a new Open supersedes the dialog's request.
- **Performance reset.** `dispose_all_preview_mechanics_results` does not touch the import. Its comment and meaning
  (#149: it carries no page identity) are unchanged. The export's retirement, barrier, tickets and states are
  unchanged.
- **Duplicate and stale deliveries.**
  - The starts are raw-body only. After Tauri's fallback a resent start arrives as JSON and is refused
    `INGRESS-RAW-BODY-REQUIRED`. Import has no external effect, so that outcome is known (§4.6):
    - a duplicated Open may have registered a source, which the next retirement drops;
    - a duplicated review changed nothing;
    - a duplicated adopt changed nothing natively, and the session changes only if the page received its reply
      (§4.8.7).
  - No "unavailable" page state is needed, unlike the export's unknown save (§6, "Export unavailable after an unknown
    save"). Every later raw start is refused the same way and says so.
  - A reply whose start is no longer current (its generation was retired, or a newer review replaced it) is
    discarded.
  - An adopt reply is consumed by one `await` in `handleOpenModelDocument` and by nothing else.

#### 4.8.7 Adopt linearization and session atomicity

1. **The webview's capture.** At Adopt the dialog captures `{sourceId, declaration, reviewSha256, modelSha256}` from
   the current review, whose request generation is current and whose declaration equals the form's.
2. **The native adopt.** It admits the arguments, checks the ticket, takes the source under the lock (otherwise
   `MBF-IMPORT-SOURCE-RETIRED`), re-serializes the declaration, and re-reads into H4 with the checks of §4.8.2. It
   runs task 5 in `adopt` mode and decodes the result (§4.8.3).
   - It requires the recomputed `review_sha256` to equal the request's `reviewSha256`. Otherwise it refuses
     `MBF-IMPORT-REVIEW-STALE`, and no model is returned.
   - It requires the adoptable flag to be `1`. Otherwise it refuses with the review's blocking codes:
     `MBF-IMPORT-NOT-ADOPTABLE`, `MBF-IMPORT-REVIEW-TRUNCATED` or `MBF-IMPORT-MODEL-TOO-LARGE`.

   Equal review digests bind the source digests, the declaration digest and the model digest the user saw, because
   they are all inside the review. The host has recomputed the model's digest over the bytes it returns.
3. **Native has no commit point.** The adopt writes nothing and changes no native state except its own run. A
   cancellation before the reply is built replies `MBF-IMPORT-CANCELLED` with no model. After the reply is built
   there is nothing native left to cancel.
4. **`read()` in the webview** (`mbfImportSource`).
   - If the barrier reports the reply not current, it returns `{outcome: "cancelled"}`. This can only come from a
     retirement that also ends the session's currency.
   - A refusal, an invoke rejection or `INGRESS-RAW-BODY-REQUIRED` becomes `{outcome: "refused", diagnostics}`,
     shown in product text.
   - Otherwise the reply's `reviewSha256` and `modelSha256` must equal the captured ones. If not, it refuses
     `MBF-IMPORT-REVIEW-STALE`.
   - It then returns `checkNativeOutcome(reply)`.
5. **The session's linearization point.** This is `handleOpenModelDocument`'s synchronous block after `await
   source.read()`: `stillCurrent()` (`projectRequest`, `requestEpochRef`), then `buildModelIndex`, then
   `advanceProjectSession` and `commitModel` with the rest of the reset (`storedBaseRef.current = null`, the saved
   basis as `loaded-source`, the cleared history and results). There is no `await` inside it.
   - Any refused, cancelled, stale, changed, retired or not-current outcome returns before this block. It leaves the
     original session's model, history, stored base, saved basis and results exactly as they were.
   - The only awaits after it (`libraryReferenceWarnings`, `computeModelHash`) are guarded by generation, as for
     Open.
   - While the project-operation owner is held, every session-replacing command is held. Only an unmount can
     intervene, and it leaves nothing to commit into.
6. **After it.** The import flow sends the keyed retirement, which releases the native source. If that delivery goes
   unacknowledged, the barrier's next start re-delivers it, and the next workspace mount drops the source in any
   case.
7. **Duplicate outcomes.** Two deliveries of one adopt produce at most one received reply that is consumed once. A
   lost reply means no session change. An Adopt pressed twice is impossible, because the button is disabled when
   the modal hold is released.

#### 4.8.8 Permissions and reach

- **Inputs.** Native code reads only files the user chose in the native open chooser, and the identity file of the
  derived name beside the chosen MBF (A-5, the same name the export writes). There is no caller path and no other
  file access: the arguments carry no path, and an extra member is refused. The files are opened read-only, with no
  final-symlink follow and regular files only, and nothing is created, written, renamed or removed. The import opens
  no store and adds no store schema, retention or deletion behaviour. No entitlement or capability file changes.
- **Commands.** The four commands (§4.7) are registered in `app_invoke_handler!`. That changes its text, so its
  SHA-256 in `user_release_reach_scan.rs` `MACRO_DEFINITIONS` is re-pinned as in §3.8:
  - the previous digest is reproduced first;
  - the new value is taken from the scan's own report, not typed;
  - the doc comment lists the entries, each reviewed: no `mod`, no `$parameter ::` and no scanned name added.

  `mbf_import.rs` carries no `#[allow(unsafe_code)]` and is not added to `SHELL_UNSAFE_MODULES`.
- **Routing.** A new `.rs` file in a crate with runtime-read declarations refreshes its `source_trees` digest and the
  `scan.files` inventories in `tools/hosted-ci-routing.json`. They are refreshed with the value
  `tools/ci/numerical_ci.py` (`source_read_dependencies`) computes, never typed by hand, and checked by
  `tests/test_ci_numerical.py` and `tools/software_workflow/test_hosted_ci.py` (`docs/CI_STRATEGY.md`).
- **Ingress lists.** `open_mbf_import`, `review_mbf_import` and `adopt_mbf_import` join `NATIVE_INGRESS_COMMANDS` and
  its test twin, sent only through the import barrier. `cancel_mbf_import` does not, like `cancel_mbf_export`.
- **Route.** The import writes no file, so it takes no governed storage route.
- **Data.** Imported content is private, like an opened document. Material provenance is affirmative (REQ-019). An
  imported document carries the MBF hash in its provenance (import Design §7.4).

#### 4.8.9 Planned tests (as planned; §4.9.3 records the implemented checks)

Every test must fail without its wiring. Only invented, project-written inputs are used: the engine writer's output
for the invented corpus and `fixtures/caepipe_mbf/invented/desktop_export/` cases.

- **Engine (`tests/import_review.rs`).**
  - The status, `adoptable` and blocking state are computed before truncation, with exact counts.
  - The per-field bound is applied in place, and the counting allocator shows no whole escaped copy.
  - The fixed leading members are byte-exact.
  - Identical inputs give byte-identical reviews. A changed declaration, file name or identity file changes
    `review_sha256`.
  - A bound fixture and an unbound fixture each pin the per-class identity counts.
  - The writer and import parity vectors are unchanged.
  - Every applied assumption emission has its structured source-safe counterpart, including conditional Z-axis
    rotation, offset rounding, G derivation, bare-metal weight, pressure and support mappings. Unknown identifiers
    and a missing counterpart block Adopt; overflow counts assumptions exactly and makes the review truncated.
- **Native (`mbf_import_tests.rs`, with the real worker; `solve_worker.rs`'s task-code round trip).**
  - Codes 0 to 5 round-trip, and 6 is refused.
  - Selection: a symlinked MBF; a directory; a FIFO; a file of 16 MiB + 1; an identity file that is a symlink, a
    directory, above 64 MiB or unreadable (each refused, never set aside); absent (unbound review); present (bound
    review).
  - Acquisition races for both MBF and identity file: the seam replaces a checked regular file with a FIFO before
    `open`. Nonblocking acquisition promptly refuses the non-regular handle without waiting for a writer. A
    cancellation at the same seam installs no source, drops every acquired descriptor and returns every lease.
    Removing `O_NONBLOCK` must fail the bounded-deadline race test; the test owns and cleans up its FIFO.
  - Re-read, after each of these, gives `MBF-IMPORT-SOURCE-CHANGED`: an in-place edit with the same inode; a replace
    or rename at the name; removal; growth during the read (the EOF check); an identity file appearing beside an
    originally absent one; an identity file removed or edited. An unchanged re-read gives a review byte-identical to
    the first.
  - No caller path: a `path` or other extra member is refused `MBF-IMPORT-ARGS-INVALID`, and no file is opened (a
    chooser and file seam records none).
  - Ledger:
    - one byte above the budget is refused by name before any worker, and at the budget the worker starts;
    - the bound holds the whole remainder while the worker runs, and the result lease alone after the reap;
    - the result lease is held until the response;
    - the pool's reservations are the same before Open and after each reply (no charge between commands);
    - a budget raised while a run waits does not extend it.
  - `result_limit`: a review result above R + 4 KiB, and an adopt result above 64 MiB, are refused before any result
    lease. The test grants above the ceiling, as §3.9.6's does.
  - Cancellation:
    - in the queue wait;
    - with the worker running (killed, reaped and its grant released);
    - while the chooser is held open (no source installed);
    - between re-read blocks;
    - a newer review superseding an older one.
  - Ticket and retirement:
    - stale, missing, null, malformed and future tickets;
    - retirement before admission and before registration;
    - the JSON and raw forms (one id delivered both ways is applied once);
    - a duplicate as a no-op;
    - 64 tokens, then exhaustion;
    - epoch and source-id overflow;
    - the performance reset leaving the import's run and source alone;
    - an import retirement leaving an export review running, and the reverse.
  - Adopt:
    - a changed declaration, or a review digest from another source, gives `MBF-IMPORT-REVIEW-STALE` with no model;
    - a retired source gives `MBF-IMPORT-SOURCE-RETIRED`;
    - not adoptable, truncated and too large each refuse;
    - for every invented corpus case, the adopted document's bytes equal `mbf::import(...).model_bytes()` and the
      import parity vectors' `model.json` (parity).
  - Tampered results give `MBF-IMPORT-WORKER-PROTOCOL`: a model in review mode, the other mode's envelope, a review
    prefix mismatch, a review or model digest mismatch, a non-ASCII model, an attested result, an exit or a panic.
  - Successful `review` mode has an adoptable candidate digest/length and zero model payload; successful `adopt`
    mode carries and verifies that same candidate. Applying body hashing to review mode must fail its positive
    test; omitting body verification in adopt mode must fail a tampered-model test.
  - Oversized digest/status/decimal header tokens, uppercase or nonhex digests, a review-digest sentinel and an
    inconsistent model sentinel all refuse before prefix construction. The bounded-prefix/streamed comparison
    uses no token-sized allocation; a counting-allocator negative control that formats an unchecked token must
    fail its scratch account. H8 is admitted before this stage and its fixed scratch stays within 4 KiB.
  - A `syn` source scan holds that in production code task 5's arm is the only caller of `worker_import`, and that no
    product source references `import_mbf_json`, with negative controls.
- **Vitest.**
  - The import never imports the wasm MBF engine, and no invoke's arguments or reply carries MBF or identity-file
    bytes or base64 (spies on `invoke`).
  - The review is shown with its completeness and counts.
  - Applied mapping assumptions are displayed with their source-safe descriptions and parameters. An unavailable
    assumption and a truncated assumptions list each disable Adopt; raw assumption sentences are never displayed.
  - A refusal, a truncated review, a stale review and `MBF-IMPORT-MODEL-TOO-LARGE` each disable Adopt.
  - Declarations:
    - each declaration change sends a new review and disables Adopt until it returns;
    - vendor and unknown origins are not offered;
    - "all listed" sends per-name entries and never `*`.
  - Cancel in the guard keeps the edits (`open_model_kept_edits`), and the source is retired once.
  - Session preservation: after each refused, cancelled, stale, changed, retired or not-current outcome, the session's
    model, undo stack, stored base and saved basis are deep-equal to before.
  - A bound import restores the fixture's exported ids in the session. An unbound import's status line says ids were
    formed from node numbers.
  - The adopted model starts unsaved with no stored base.
  - `INGRESS-RAW-BODY-REQUIRED` shows the import's product text.
  - The barrier: mount retirement; serialized retirements; JSON-form cancel; re-delivery with the same id; a
    `MBF-IMPORT-RETIREMENT-UNCONFIRMED` refusal.
  - App level (`App.mbfImportDialog.test.tsx`):
    - the native menu event and the in-window item open the dialog;
    - a busy session does not open it;
    - Close, Escape and an App unmount each send one retirement;
    - a reply arriving after close is discarded;
    - the export dialog and its tests are unchanged.
- **Mutations to check** (each must fail a test):
  - no hash comparison on re-read;
  - no check for an identity file that appeared;
  - the source kept after a retirement;
  - adopt without the reviewed digest;
  - the result lease released before the response;
  - no generation check before a send;
  - the commit before `stillCurrent`;
  - a declaration change that does not invalidate Adopt;
  - `*` sent for "all listed".
  - model-body hashing applied to a successful review-mode reply;
  - blocking file acquisition after the path check;
  - an applied mapping assumption dropped from the review;
  - an unchecked header token copied into a dynamic prefix.
- **Open-consumer comparison.** Feed the same maintained native-produced document through ordinary Open and the
  import source. Assert one call to the shared webview validator in each flow, the same document reference at its
  entry and at adoption, the same index/refusal and unsaved-adoption behavior, and no import-only model/report/file
  bytes or raw model text retained by the dialog or source. Keep the existing publication/hash/library call path;
  do not add import-specific calls to make the comparison pass. Negative controls add a second validator call or
  retain a second model and must fail the corresponding maintained check. Native tests cover the actual escaped
  final reply lengths and H6/H7 overlap through response construction, including an application-side handoff copy
  if the implementation makes one. This is a source/connected-behavior comparison, not a heap-ceiling measurement.
- **Not claimed.** These tests do not show criterion 6. Slice 6's connected supported-subset round trip
  (`save_mbf_export` → `open_mbf_import` → `adopt_mbf_import`, bound and unbound, in the desktop) is the next finish
  dependency (§5, §6). Criterion 6 stays open for the reference system's families (§1.4).

#### 4.8.10 Review gate and implementation boundary

- **Before any production connection,** this design takes independent review of:
  - numerical meaning and parity: the adopted bytes are the engine's import, unchanged;
  - persistence: session replacement atomicity, source custody and release, and the absence of any write;
  - permissions: chooser-owned read-only inputs, no caller path, the four commands, the reach re-pin and the ingress
    lists.

  It is then merged, and Agent 0 confirms it.
- **Policy/application condition.** Review must establish §4.8.5's existing-consumer comparison and the independent
  admission of every new import stage; implementation must preserve both with the specified connected checks.
  A genuine additional unaccounted import allocation blocks connection. The unresolved common-consumer heap
  qualification is tracked under RT-U4, not converted to a new slice-5-only proof gate or risk acceptance.
- **The implementation PR** carries the review records above and the generated reach and routing artifacts from
  their writers (§4.8.8).
- **The implementation's intended touch set,** which the manager confirms in the draft PR before editing:
  - DEL-17-04: `core/handoff/caepipe_mbf_engine/src/import_review.rs` and its test; `apps/desktop/src-tauri/src/
    mbf_import.rs` and `mbf_import_tests.rs`.
  - Strictly necessary importer support in `core/handoff/caepipe_mbf_engine/src/lib.rs` and
    `src/importer/{mod,finish,pressure}.rs`: structured assumption facts paired with existing emissions. The
    report/model/id-map bytes and numerical branches stay unchanged and are held by parity and mapping tests.
  - Slice 5's shared hunks, by the owner's assignment: `lib.rs` (the module, the four commands, the managed state,
    the menu item and its tests); `solve_worker.rs` (task 5's code, decode and arm only); `user_release_reach_scan.rs`
    (the pin); `tools/hosted-ci-routing.json` (by its writer).
  - DEL-07-12's paths: `features/caepipe-mbf/MbfImportDialog.tsx`, `mbfImport.ts` and `mbfImportCancel.ts`, with
    their tests.
  - The mount and wiring: `App.tsx` (the dialog's mount and the in-window item); `workspaceSession.ts`
    (`requestImportMbf`, `mbfImportSource`, the modal hold and the retirements);
    `services/nativeIngress.ts` and its test twin (the lists).
  - Nothing in `mbf_export.rs`, `mbf_worker.rs`, the export dialog, `mbfExportCancel.ts`, tasks 3 and 4, the
    performance reset, `services/modelDocumentFile.ts`, `native-smoke/expected.json`, store schemas or retained
    pricing changes.
- **Reconciliation in the implementation PR.**
  - DEL-17-04's `ScopeOfWork.md` gains a CLM-003 paragraph for the desktop import and two CLM-010 trace rows: the
    mount, and the memory and source custody.
  - Its `deliverable.yaml` `code_paths` and `checks` gain the new files.
  - DEL-07-12's finite inventory already names the import (".mbf model export and import"), and its `code_paths`
    already cover `features/caepipe-mbf/**`. Its `checks` gain the dialog tests.

#### 4.8.11 Open matters

- **Existing consumer and additional work** (§4.8.5). Review and connected implementation checks must distinguish
  the admitted new native import/handoff stages from the unchanged Open-consumer heap residual. No common heap
  bound or v1.0 risk acceptance is inferred; whole-App qualification remains under RT-U4.
- **Usability limits, not accepted as release limitations:** a truncated review blocks Adopt; an import result above
  64 MiB cannot be adopted; a material whose name the review cannot carry in full (more than 512 bytes) cannot be
  declared, and keeps the import refused.
- **Exhaustion.** The 65th distinct page load in one process exhausts the import until restart, as for the export
  (§3.7).

### 4.9 Implementation (slice 5)

The slice-5 PR implements §4.1, §4.3 and §4.5 to §4.8 on `main` at `394f4c623`. Independent reviews requested repairs;
their checked revisions, dispositions and integration conditions are recorded in the PR. The reviews cover
numerical meaning and parity, persistence and session replacement, permissions and the reach re-pin, and provenance
declarations. Nothing here records acceptance or shows criterion 6.

#### 4.9.1 What was built

- **Engine.** `import_review.rs` forms the review of §4.3 (`swbpipe.mbf_import_review`, format 1). After the fixed leading
  members come `completeness`, `transfer_limit` and a `blocks` object (`import_refused`, `document_check`,
  `truncated`, `model_too_large`, `assumption_unavailable`), then `source`, `binding` (`state`, `reason`),
  `declaration` (`sha256`, `recorded_origins`), `units`, `model` (`document_check`, `schema_status`), `identity`
  (per class of `IDENTITY_MAPS`: `count`, `restored`, `formed_from_mbf_names`; and `unrecoverable`) and `counts`. The
  lists follow, in this order: `refusals`, `document_check`, `assumptions`, `declaration_needed`, `loss_entries`,
  `target_options`. The serializer helpers are the export review's, shared by visibility only. `ImportResult` gains
  `assumption_facts` and `recorded_material_origins`, neither part of `to_dict()`. The origin count is recorded
  when `assemble_materials` actually selects a bound-sidecar origin for an emitted material; caller provenance
  wording cannot supply that decision. `importer/finish.rs` and `importer/pressure.rs` record one
  `AssumptionFact` beside each existing assumption emission, under the same condition and in the same order, with
  16 identifiers (`ASSUMPTION_IDS`). The report, model and id-map bytes are unchanged; the parity replay holds them.
- **Native.** `mbf_import.rs` holds the source record, the selection and re-read, task 5 (`worker_import`), its
  envelope and decode (`decode_import_result`), the ledger-bounded run (`Command::run_worker`), the four commands'
  flows and the keyed retirement on its own `MbfImports` state. `solve_worker.rs` gains task code 5 and its workload
  arm only. `lib.rs` registers the module, the four commands, the managed state and File > Import MBF…
  (`file.import-mbf`, after Open Model Document…, disabled while busy).
- **Frontend.** `mbfImport.ts` holds the types and product descriptions: `MBF_IMPORT_DESCRIPTIONS` for every importer and
  native code, and `MBF_IMPORT_ASSUMPTIONS` for every structured identifier, formed from its parameters.
  `mbfImportCancel.ts` is the barrier, and `MbfImportDialog.tsx` the dialog. `workspaceSession.ts` gains
  `requestImportMbf`, `mbfImportSource`, the modal hold in every list that holds project commands, and the
  retirements. `App.tsx` gains the mount and the in-window item. `menuCommands.ts` gains the id, and `nativeIngress.ts`
  and its test twin gain the three starts.
  `mbfImportSession.ts` owns generation-specific retirement without retaining model/review/DOM data.

#### 4.9.2 Refinements within this design

- **Dispatch and declaration currency.** Each start carries a positive safe-integer `requestSequence`, retained on
  every resend. Native entry keeps only the highest sequence in the current retirement epoch, rejects duplicates
  and older starts, and checks the sequence again at registration. An applied retirement resets that sequence
  space; a duplicate retirement does not. This prevents an older queued command from superseding a newer one even
  when executor/admission timing reorders them. The frontend also checks the latest start and the dialog's current
  declaration before replies or retries; reverting to the displayed declaration invalidates pending work too.
- **Response handoff.** H7 admission can wait. Whether admitted or refused, cancellation and final response
  preparation are ordered under the run control's lock; a cancellation that won during admission returns no model. A prepared
  response is marked handed off before invoking the response callback, so a later retirement does not falsely
  count that command as stopped. This is not a filesystem commit or session adoption; frontend currency and the
  maintained Open transaction still decide whether a received model enters the session.
- **Generation-owned retirement.** `mbfImportSession.ts` keeps bounded ownership metadata and a retired-generation
  watermark. Completion and passive cleanup call the same idempotent retirement, in either order; an older
  generation cannot retire or close a newer dialog. No review, model or DOM node is retained by this owner.
- **Raw protocol fixture and public provenance** (Agent 0 ruling, 2026-11-07). `desktop_import/import_replies.json`
  is a raw native transport capture: its five reply strings retain the response callbacks' exact bytes. It is test
  evidence of transport behavior, not a public engineering-model dataset. Its companion supplies all seven public
  provenance fields and explicitly covers the entire invented transcript, the identified source model, exact export
  settings, actual saved MBF/identity pair and material declaration, including every nested model and record. The
  native producer uses actual export review/save task 3 and its file transaction before task-5 import; it binds
  the source and pair bytes, review/declaration/model digests and every captured model to those invented inputs.
  The App test setup only JSON-parses the strings unchanged. Inner runtime provenance stays exactly as the importer
  emits it and can be partial; string encoding alone supplies no provenance disposition. The generic public-model
  scan is unchanged, and other committed engineering-model datasets still need seven fields per provenance record.
  An unidentified captured byte stops the artifact. Focused independent scrutiny checks this data-boundary
  application; this distinction grants no runtime behavior, acceptance or release authority.
- **Retirements in the browser-only route.** The import's mount and session-end retirements are sent only in the
  Tauri runtime. The browser route has no native import: File > Import MBF… is refused `MBF-IMPORT-NATIVE-ONLY` before
  anything is sent, so no source can exist to drop. The export's browser-route behaviour is unchanged.
- **Schema status.** Task 5 makes a candidate whose native document check opens with a schema status other than
  `current` not adoptable (`MBF-IMPORT-SCHEMA-NOT-CURRENT` at `schema_version`). The adopt reply's `schema_status` is
  therefore `current`, and its `byte_count` is the canonical model's length L.
- **Conventions.** A binding convention's structured form maps the importer's own three `what` texts exactly
  (`importer/mod.rs` `convention_key`). Any other text records the `unavailable` marker, which blocks adoption
  (`MBF-IMPORT-ASSUMPTION-UNAVAILABLE`). Nothing is guessed from diagnostic codes or sentences.
- **Recorded origins.** The count follows the importer's actual bound-sidecar origin-selection decision for emitted
  materials, not caller-controlled source wording. It is informational and is not used for adoption.
- **Declaration shape (host).** Besides §4.6, the host refuses the `*` key, a material name above 512 bytes and an
  entry with any member other than `origin` and `source_name`. The worker receives the declaration re-serialized
  canonically, so equal declarations give equal `declaration_sha256`.
- **Adopt's blocking codes.** The host reads them from the review's fixed members after the leading ones
  (`completeness`, `transfer_limit`, `blocks`), within a 1 KiB window. It never parses the review.
- **Selection race.** A regular file replaced by another regular file between the `lstat` and the open (a changed
  identity) refuses `MBF-OPEN-UNREADABLE`. A non-regular one refuses `MBF-OPEN-NOT-A-FILE`.
- **Reach-scan rules.** `mbf_import.rs` uses no `debug_assert_eq!`, which is not on the scan's invocation list. The
  envelope's counted length is checked as a named `MBF-IMPORT-ENGINE-FAILED`.
- **Windows.** The open uses `FILE_FLAG_OPEN_REPARSE_POINT` and `seek_read`. The import's native tests are Unix-only,
  as the worker harness is.

#### 4.9.3 The checks implemented

Every check below uses maintained invented inputs: the desktop export fixture and its recorded `degc-default`
digests; the native connecting tests and App reply producer call `review_mbf_export` and `save_mbf_export`'s command
pipeline (task 3 and its file transaction), then open and adopt those actual saved files through task 5, both with
the identity file and after deliberately removing it. Other file/refusal cases use engine-written bytes. Inputs also
include `units_si.mbf` and
`units_english.mbf`; and `fixtures/caepipe_mbf/invented/desktop_import/import_replies.json`. That last fixture holds
verbatim native reply strings; the native test compares its complete bytes to the actual callback strings and
records the source-file, saved-pair and exact declaration bindings. The seven-field companion explicitly covers
every input and reply; the producer checks complete coverage and public disposition. It is regenerated only on
request. No imported model or parity value is edited to make it public fixture data.

- **Engine** (`tests/import_review.rs`; `tests/parity.rs`): the fixed leading members byte for byte; status, totals and
  blocking before truncation, at the smallest allowance and one byte below the full review; determinism and what
  changes `review_sha256`; per-class identity counts for one bound and one unbound import; the declaration-needed
  list; a missing, unknown or `unavailable` assumption blocks; the English branch's facts paired with its strings;
  the transfer decision sees the adoptable review's exact length; the per-field bound. Over every parity vector,
  each result's facts pair with its assumption strings, and every identifier is recorded at least once. The parity
  bytes are unchanged.
- **Identity/source review repairs.** Identity expectations use an independent fixture inventory. A bound summed
  primitive has a generated id, not either original id; only support lists expand their entries, while each
  pressure-region map key counts one region regardless of its member count. Named maintained parity inputs cover
  the summed load and two-/three-member regions. An unbound caller supplying the recorded-origin marker as a source
  name still has zero recorded-sidecar origins. Guide facts preserve preceding-element selection with a following
  fallback, and formed pressure-region facts preserve explicit node closure overrides. Material declarations use
  own dictionary entries for names such as `__proto__`; transport tests retain those entries through JSON.
- **Native** (`mbf_import_tests.rs`, task 5 in a real worker): bound and unbound connected imports of the saved pair,
  with ids restored or formed and the adopted document equal to the engine's import; each change after selection
  (in-place edit, replacement, removal, symlink, identity file removed, edited or appeared, growth during the read)
  is refused, and an unchanged re-read is byte-identical; selection refusals; the FIFO acquisition race for both
  files, and a cancel at that seam; a retirement while the chooser is open and between read blocks; unknown members,
  every ticket form and every declaration shape; keyed retirement in both forms, duplicates, 64 tokens, epoch and
  source-id exhaustion, and the import and export retirements apart; the performance reset does not name the
  import; Adopt's staleness and retired source; review mode with no model and adopt mode with the verified
  candidate; every tampered or malformed envelope; the escaped adopt-reply length; queue-wait and supersession
  cancellation, with workers reaped and leases released; the review `result_limit`; the budget boundary one byte
  below and at it; the source scan that task 5 is the only production caller of `worker_import`.
  `solve_worker.rs`'s round trip covers codes 0 to 5, refuses 6, and checks that task 5's workload is
  `worker_import`. `lib.rs` covers the menu item.
  Additional regressions stop an already-cancelled registration before its chooser, reject a late older dispatch,
  cancel specifically while H7 admission is held, and prove H6 and H7 separately remain held through response
  construction (H1 alone cannot satisfy those assertions). Forced H7 refusal also hands off before the response
  callback: retirement there counts zero stopped commands; cancellation before preparation retains its cancelled outcome.
- **Vitest.** `mbfImport.test.ts`: every engine and native code and every structured identifier has its product
  description; the conditional parameters; Adopt blocked by an unknown or cut assumption, truncation or the
  transfer limit; no wildcard; and no product source uses the wasm importer, with negative controls.
  `mbfImportCancel.test.ts`: the handshake, tickets, the JSON-form cancel, re-delivery with the same id, then
  unconfirmed; a retired or superseded start is never re-sent. `mbfImportSession.test.ts` covers completion before
  cleanup, cleanup before completion, reopening in one generation and older cleanup preserving a newer request.
  `App.mbfImportDialog.test.tsx`, against replies produced using the UI's actual invented-provenance declaration:
  - the native menu and the in-window item open the dialog, and a busy session starts nothing;
  - a bound Adopt goes through the Open transaction, with one validator call given the reply's own document object,
    and the import is the same as ordinary Open for the same document;
  - unbound per-name declarations, with Adopt held until the declared review returns;
  - declaration reversion invalidates a pending reply and a later declaration can review/adopt again; prototype-like
    material names remain own declaration entries through the dialog/transport;
  - Close, Escape and unmount each send one retirement, and a reply after close is discarded;
  - a native refusal, a stale digest, `INGRESS-RAW-BODY-REQUIRED` and a shape refusal each leave the session as it
    was;
  - with unsaved edits, the guard's Cancel keeps them, sends no adopt and retires once;
  - the browser route refuses by name before any send.
- **Generated artifacts.** `app_invoke_handler` is re-pinned from the scan's span rule; the previous digest is
  reproduced first, and the reach scan passes. `tools/hosted-ci-routing.json` is refreshed with `numerical_ci`'s own
  values, and `tests/test_ci_numerical.py` and `tools/software_workflow/test_hosted_ci.py` pass.

**Not implemented or not run** (recorded, not claimed):

- the counting-allocator checks of the in-place per-field bound and of H8's scratch;
- the mutation list, as a mutation run (its main controls are the behavioural negative controls above);
- an attested, exiting or panicking task 5;
- a budget raised during the wait;
- session preservation as a deep comparison of internal state (the App tests compare what the session shows: the
  project, its edited state and the tree).

The FIFO acquisition race runs in a bounded test subprocess for each file. The parent owns the sandbox, kills and
reaps a blocked probe at its deadline, and fails the regression, so removing nonblocking acquisition cannot hang
the suite. The probe is an ignored helper invoked explicitly by that parent, not an unexecuted regression.
Initial local Vitest used the main checkout's installed packages and wasm build of 2026-11-05. Seven
`App.shell` tests fail locally in the wasm result reader (`SOURCE_PREVIEW_PHYSICS_EVIDENCE_SHAPE`), unrelated to the
import; hosted CI rebuilds the wasm.

Criterion 6 is not claimed: reference-family coverage, C6-2 and Route 1 stay open and held (§1.4, §5).

## 5. The round-trip check

**In CI, with invented data only.**

- **Fixture.** `fixtures/caepipe_mbf/invented/coverage/coverage_model.json`, with its
  invented-provenance sidecar. It holds one instance of every family the writer covers, and
  grows with each coverage tranche.
- **Engine test** (Rust, and the Python twin through parity).
  - It carries the A-3 Must-family table. Each family is marked `covered` or `refused`, with
    the expected code.
  - For the covered set it asserts C6-1, C6-2, C6-4, C6-5 and C6-6.
  - For each family marked `refused`, it asserts that the named refusal still occurs, so the
    table cannot drift.
  - Criterion 6's fixture-level check holds when the table has no `refused` Must family.
- **Native test** (`src-tauri`). In a temporary directory, `save_mbf_export` writes the pair,
  and the files read from disk import bound with the same ids through `open_mbf_import` and
  `adopt_mbf_import` (§4.7) (C6-3, C6-7).
- **Desktop test** (Vitest). The export dialog drives mocked `review_mbf_export` and
  `save_mbf_export` (the bytes may come from the wasm writer, used as a test resource). Import MBF adopts them through mocked `open_mbf_import`,
  `review_mbf_import` and `adopt_mbf_import`, whose replies are the native import's of those
  bytes, and the session's id sets equal the exported ones.
- **CI routing.** The tests are routed in `tools/hosted-ci-routing.json`.

**What only the owner's reference system shows.**

- The reference system authored in the actual product build, saved through the native chooser
  and imported back, with C6-1 to C6-6 on it.
- Criterion 6 for the families that the reference system actually contains.
- Whether the target program reads the file belongs to the validation lane's private evidence
  (RT-V2, DEL-17-05), not to criterion 6.

**The reference system (O-1).** Criterion 1 calls it "an invented reference system". Agent 0
placed it in DEL-09-01 (2026-10-31, replacing this section's earlier assignment to W-17): it
is specified in DEL-09-01 `Work/reference-system.md` and generated as
`fixtures/reference_system/reference_system.json`, with its run sheet (criteria 1 and 7),
starting from the families that exist now and adding the rest as their rows land. That
specification's §11.1 lists the families criterion 6 expects in the loss report. Families without an admitted public MBF
source (today valves, reducers, flanges, rigid elements and nozzles; Q-3) are authored in the
fixture and expected in the loss report until their sources exist. The fixture is named here
and in RT-R7/R8's status column; the release target's criterion text is the owner's and is
not edited.

## 6. Slices

Each slice is one PR. Every test must fail without its wiring.

| Slice | Content | Tests | Gate |
|---|---|---|---|
| 1 | This design | GEN-13; deliverables check | Agent 0 on §8 |
| 2 | Engine crate: the writer port and `sidecar`, the wasm build, parity vectors. It starts from the CAEPIPE lane's SIF fix and Q-6 omitted-entries sweep (its named head), so the port does not copy the drop | Byte parity with Python over the whole committed MBF corpus and the authored cases, both ways, plus every writer test model and pilot case (text, sidecar, `to_dict()`); every `MBF-SI-*` code; normalization factors exact; float formatting at rounding boundaries; `forbid(unsafe_code)` | Independent numerical review (numerical meaning, file bytes) and targeted regressions |
| 3 | Engine crate: the importer port | Byte parity over the whole committed MBF corpus and the authored cases, both ways, plus every import fixture and the outputs of `test_caepipe_mbf_round_trip.py`; every named refusal; bound and unbound imports; self-digest; the reversible and byte-reproducible sets in Rust | Independent numerical review |
| 4 | Export: `DOTH-MBF-012` (local_private, explicit intent, lossless) in the three catalogues, with parity tests in all three; `save_mbf_export` and `review_mbf_export` (worker task 3 in `save` and `review` modes, §3.9.5); the atomic formats; reach-scan re-pin; menu item and dialog; retire the TypeScript builder (A-6); the #1711 row | Rust: admitted under its lease, with the counting-allocator peak within it; named arguments; hash mismatch; stale review; writer refusal opens no chooser; protected disposition refused; Cancel writes nothing; identity file before MBF; an injected failure of the second write removes the first; a read-back mismatch removes both; bytes equal the engine's; review mode touches no file or chooser; unknown modes refused; the review is at most 1 MiB in total, with its status and blocking counts computed before truncation; a save result over 64 MiB refused with `MBF-EXPORT-OUTPUT-TOO-LARGE` before allocation, writing nothing. Route-catalogue parity; reach-scan pin. Vitest: the dialog never runs the wasm writer; `MBF-REVIEW-NATIVE-ONLY` in the browser-only route; refusals block Save, omissions warn, degF reference temperature, stale review runs again, suggested names, keyboard reach | Independent review (persistence, file writes) and an independent permissions review of the new governed route |
| 5 | Import: the four commands of §4.7 and worker task 5 (§4.8.3); the bounded import review (§4.3); the low-custody source (§4.8.2); material declaration; the dialog, the menu item and the import barrier; reach-scan re-pin and routing refresh by their writers. Its source, memory and transaction design is §4.8 (2026-11-07). That design is independently reviewed and merged before any production connection: the export's proof does not cover the import, whose regeneration runs the writer again | §4.8.9; implemented as §4.9.3 records, on the slice-5 PR. It includes the earlier list: Rust caps, symlink and non-regular-file refusals, identity file absent, present and unreadable, the leases; Vitest: the guard's Cancel keeps edits; refusals block Adopt; a bound import restores ids; an unbound import states its consequence; the declaration re-run; the adopted document passes `checkNativeOutcome`; unsaved with no stored base | First, independent review of the §4.8 design: numerical meaning and parity, persistence (session replacement, source custody), permissions (chooser-owned read-only inputs, the commands, the reach re-pin). Then independent review of the implementation (session replacement; provenance declarations) |
| 6 | The connected supported-subset round trip in the desktop, bound and unbound (save, then import and adopt), which is the next finish dependency after slice 5; the criterion-6 check of §5 and its CI routing | As in §5, and for the reference system: its review is complete and its export result is below the 64 MiB output ceiling | Focused review |
| 7 | Unbound pressure closure declaration; rebinding by node name | Vitest and engine tests for both | Independent review |
| 8+ | Coverage tranches (CAEPIPE lane, each designed in the import Design §20.3), in Python and Rust together | Parity; each new family moves from `refused` to `covered` in §5's table | That Design's gates |

A bound format 2 or 3 import in slice 3 needs slice 2's writer, which it takes through the
`SidecarRegenerator` trait. Slice 4 needs slice 2, slice 5 needs slice 3, and slice 6
needs slices 4 and 5.

**Slice 2, as implemented (2026-10-30).** `core/handoff/caepipe_mbf_engine/src/writer/` ports
`si_writer.py` byte for byte.

- **API.** `render_si(model, &WriterSettings)` returns `WriterResult`, the `MbfSiResult` port,
  with its maps, loss report and assumptions, `to_dict()` and its canonical bytes, and
  `sidecar(mbf_sha256)` (format 3 with its self-digest) and its file bytes.
  `regenerate_caepipe_sidecar` is ported.
  - `WriterSettings` also carries the Python's `piping_code` argument.
  - The title, reference temperatures and piping code are `Value`s, because the Python takes
    any value and refuses or `str()`s it.
  - `max_layout_rows` is `Option`, so the product's `None` (Q-4) is expressible.
- **Regeneration and preflight.** The crate's writer (`SiWriter`) is the importer's regenerator
  when `ImportOptions::regenerator` is None. `ImportError::RegeneratorRequired` is removed.
  The `SidecarRegenerator` trait stays public. The writer's preflight
  (`MBF-SI-PREFLIGHT-REFUSED`) runs the Rust importer.
- **Errors.** A Python exception the writer raises is `ImportError::Python` with the same type
  and message. An input the port cannot reproduce fails closed as `ImportError::Unsupported`:
  a NaN load category, whose set membership the Python decides by object identity, and the
  sidecar parser's internal sentinel.
  - **Nesting bound** (reviews r1 and r2 of #32). No value may sit inside more than 256 lists and
    dicts (`MAX_VALUE_NESTING`, counted as the sidecar parser counts its own bound of 64). Every
    public operation that walks a caller's `Value` checks this first, without recursion. That covers the
    writer's model and settings, the regeneration, `import`'s parsed options, `sidecar_self_digest`,
    `canonical_bytes`, `dumps_sorted`, the `to_dict`, bytes and sidecar methods of both
    result types, whose fields a caller may set, and a caller's `SidecarRegenerator` output.
    A refused value the crate owns is dropped without recursion. A deeper value fails closed as `Unsupported`
    (those methods now return `ImportError`, which keeps the Python's exceptions as
    `ImportError::Python`). Equality (`py_eq`) does not recurse. A test runs every walk at the
    bound on a 1 MiB stack, wasm's default, in debug and release builds. The Python may still
    accept a deeper value, up to its own recursion limit.
- **Parity.** `tools/fixtures/caepipe_mbf_writer_vectors.py` records the inputs of every writer
  call the suites make, the corpus the import vectors write back, the correlation cases and
  generated coverage inputs. Each input is replayed through the unpatched Python for its
  expected outputs.
  - `tests/writer_parity.rs` and `tests/test_caepipe_mbf_writer_parity.py` replay the vectors.
    Every `MBF-SI-*` code is emitted.
  - `tests/parity.rs` now checks each recorded regeneration against the Rust writer's own.
- **Open.** One Python defect is recorded and ported unchanged. A spring support at a node no
  pipe segment reaches raises `KeyError` instead of a named refusal. This lane fixes it in both
  twins in a later slice, with its vectors regenerated.

**Slice 4a, as implemented (2026-11-01).** Slice 4 is split. 4a is an explicitly unconnected foundation: its
production entry point refuses with `MBF-EXPORT-NOT-CONNECTED` (§3.7), and native writer memory governance and the
live export are unfinished until 4b's worker path is verified. 4b connects it after the desktop performance work (#59)
integrates.

- **Webview writer.** `core/handoff/caepipe_mbf_wasm` wraps the engine for wasm (`render_mbf_json`,
  `import_mbf_json`), built by `build-wasm-engine.mjs` into `public/mbf-engine/` and loaded by
  `src/services/mbfEngine.ts`. The model text is read as the engine reads JSON (`parse_sidecar_json`). Under §3.9's
  adopted design the wasm export entry point stays only as an unconnected or test resource; the shipped dialog never runs it, and the import's wasm route is not adopted (slice 5's design, §4.8.1).
- **Settings.** `{title, referenceTemperature: {value, unit}, maxLayoutRows: null}`: `degC` is passed as
  `reference_temperature_c`, `degF` as `reference_temperature_f`, with no row limit, and the title is at most 4 KiB of
  UTF-8. One engine function maps them, `export_settings_from_json` in `core/handoff/caepipe_mbf_engine`
  (`src/export_settings.rs`), which reads the settings' JSON text with the engine's own reader. The wrapper and the
  native module both call it on the same text: the dialog sends the text it reviewed with as `settingsJson`. The
  recorded cases `fixtures/caepipe_mbf/invented/desktop_export/export_digests.json`, with each case's settings text,
  are written by the wrapper's test and checked by the native test and the dialog test. They include integers between
  2⁶³ and 2⁶⁴ and between 2⁶⁴ and 10²¹, which both sides read as the same ints. The writer refuses those temperatures
  alike (`MBF-SI-PREFLIGHT-REFUSED`).
- **Reviewed digests.** `reviewed.sidecarSha256` is the SHA-256 of the identity file's bytes, not its internal
  self-digest. The route evidence keeps the sibling saves' member name `localFirstEvidence`.
- **Route.** `DOTH-MBF-012` is in `route_control.py` and `redactionExportControls.ts`, each with parity tests. Unlike
  `DOTH-MODEL-DOC-007` and `DOTH-HANDOFF-PACKAGE-008`, it is in both catalogues' local-private-only set with the CSV
  route (Agent 0, 2026-11-01): a public context is refused. The native admission, the shared local-private check
  called from the new module, remains authoritative.
- **Native save.** `apps/desktop/src-tauri/src/mbf_export.rs` runs §3.6 steps 1 to 10 as plain functions behind a
  chooser and file-operation seam, reachable only through the test seam `InProcessWriterTestSeam`; its production
  entry point `save_mbf_export_admitted` refuses with `MBF-EXPORT-NOT-CONNECTED` before any parse, writer run or file
  access, and a test holds that it writes nothing. Steps 2 to 4 are Save Model Document's admission
  (`admit_model_document_text_screened`). The formats are `MBF_FORMAT` and `MBF_IDS_FORMAT`. Step 8 checks both
  destinations with the atomic writer's own checks before any write (`check_destination_as`, a small public helper in
  `atomic_report_package_save.rs`). It backs up each file to be replaced by a hard link and undoes the save on any
  later failure; a rollback that cannot restore keeps and names its backups. A cleanup or restore touches a file only
  while it is still the file the save wrote or backed up: the same device and inode, or off Unix the same size and
  both times.
- **Memory.** The in-process k·M pricing was found unsound and is removed with its constant and its pin (§3.7). The
  writer's memory is to be governed by the capped worker process in 4b. The host-side reservations (settings, text,
  outputs, read-back, response) stay, and the memory refusal raised inside the admission's screen is returned as
  `MBF-EXPORT-MEMORY-UNAVAILABLE`.
- **Dialog.** `apps/desktop/src/features/caepipe-mbf/MbfExportDialog.tsx` (with `mbfExport.ts`) shows the §3.3
  review and the settings, re-runs the review on any change and after `MBF-EXPORT-REVIEW-STALE`, and calls an
  injected `saveMbfExport`. It is not mounted. The review follows §3.3's product-text rule: a neutral profile label,
  each loss entry, and each error diagnostic without one, by code, entity and a description from
  `MBF_LOSS_DESCRIPTIONS` (a blocking code's says the file is not written), and unit conversions from the
  wrapper's `unit_conversions` (the writer's normalization data, carried in `WriterResult::unit_normalizations`,
  outside `to_dict()`). The writer's assumptions are not shown, and neither is the MBF version number.
- **Shared pins.** One line in `user_release_reach_scan.rs` `SHELL_DEPENDENCIES` admits the engine dependency
  (Agent 0, 2026-11-01). The engine adds `num-bigint`, `num-integer` and `num-traits` to the shell's normal
  dependency graph. `ryu`, `sha2` and the rest of its tree were already there.
- **4b connects:** the writer in the capped worker process and the verification of that path (§3.7), as worker
  task 3 with its strict `review` and `save` modes (§3.9.5); the 64 MiB output ceiling through the worker's
  `result_limit` hook, refusing with `MBF-EXPORT-OUTPUT-TOO-LARGE` (§3.7); command registration and ingress admission
  for `save_mbf_export` (`save_mbf_export_args`, `save_mbf_export_admitted`) and for `review_mbf_export`, which joins
  the same permissions, reach and ingress review; the `app_invoke_handler` re-pin; the menu item, the Exports panel
  action, mounting the dialog on `review_mbf_export` (§3.9), and retiring the TypeScript builder (A-6).

**Slice 4b.1, as implemented (2026-11-01).** 4b is split around the desktop performance work (#59): 4b.1 builds the
bounded review, worker task 3 and its envelope in MBF-owned files only, and connects nothing. Both production entry
points (`save_mbf_export_admitted`, `review_mbf_export_admitted`) refuse with `MBF-EXPORT-NOT-CONNECTED`; no command is
registered and no worker task is added.

- **Engine.** `export_review(result, files, allowance, output_fits)` in `core/handoff/caepipe_mbf_engine`
  (`src/export_review.rs`) holds §3.3's classification, moved from `mbfExport.ts`: `not_written` (each `unsupported`
  loss entry, then each error diagnostic without an `unsupported` loss entry of the same code and entity), `omitted`, `warnings`
  (each warning without an `omitted` entry of the same code and entity), then the undeclared materials. A loss entry
  and the diagnostic raised with it are one entry, so they are listed or counted together. Info diagnostics and every
  engine message, reason and assumption are not carried. The review is ASCII JSON (`swbpipe.mbf_export_review`, format
  1), at most R after escaping: status, `blocking` and `refused_unexplained` are computed over the full result; the
  fixed fields are reserved at their widest; entries are listed in order while they fit and the rest counted
  (`completeness`, `not_shown`, `not_shown_blocking`, and per list `total`, `listed`, `not_shown`,
  `not_shown_blocking`). The per-field bound is 512 bytes (`REVIEW_FIELD_MAX_BYTES`): a longer string is carried as
  `{prefix, bytes, sha256}`, formed in place, each entry sized before it is written. `output_ceiling` (`within` or
  `beyond`, one width) is the caller's decision on the review's exact length. The unit conversions (each normalized
  unit, its definition and exact factor) and the digests and file lengths are carried. Tests:
  `tests/export_review.rs`; the writer and import vectors are unchanged.
- **Worker task 3** (`apps/desktop/src-tauri/src/mbf_export.rs`). `worker_render(raw)` reads `u32 mode ‖ u32
  settings_len ‖ settings JSON (≤ 25 KiB) ‖ model canonical text` (little-endian; mode 1 `review`, 2 `save`; a bad
  mode, length or field is refused `MBF-WORKER-REQUEST-INVALID`), runs `render_si`, forms the review and returns the
  ASCII envelope `MBF1\n<mode>\n<status>\n<mbf_len>\n<ids_len>\n<review_len>\n` and the bodies. `review` mode
  returns `reviewed` with the review only. `save` mode returns `written` with the MBF, the identity file and the
  review, or `writer-refused`, `review-truncated` (whatever the blocking count) or `output-too-large` (decided from
  the computed lengths before the result is built) with the review only. `Err` is a named failure of at most 64 KiB
  with no engine text. `decode_worker_result(mode, text)` reads the envelope as views: header, canonical decimal
  lengths exactly covering the bodies on character boundaries, the review within R and the result within 64 MiB, the
  mode, a status of that mode with its fixed shape, and a review that supports the status; for `written` it
  recomputes both digests and requires the review to name them and the lengths. Anything else is
  `MBF-EXPORT-WORKER-PROTOCOL`.
- **Pipelines.** Step 5 runs task 3 through a writer-run seam (`MbfWriterRun`); only `InProcessWriterRun`, which needs
  `InProcessWriterTestSeam`, implements it. The written bytes and the step-6 digests come from the decoded envelope.
  A refusal before the chooser replies with its code (`MBF-EXPORT-WRITER-REFUSED`, `MBF-REVIEW-TRUNCATED`,
  `MBF-EXPORT-OUTPUT-TOO-LARGE`) and the bounded review only. `review_mbf_export_outcome` takes the settings mapping and
  §3.6 steps 2 and 3 (`admit_model_document_text_unscreened` in `model_document_save.rs`), then task 3 in `review`
  mode, and opens no store, file, destination or chooser. The seam's failures map to named codes (memory, the output
  ceiling, protocol, `MBF-EXPORT-CANCELLED`). 4a's transaction, backups, lone-identity refusal, settings bounds, route
  and full names are unchanged.
- **Dialog.** `MbfExportDialog.tsx` takes an injected `reviewMbfExport` (`{canonicalText, claimedModelHash,
  settingsJson}`), or null in the browser-only route, which declines with `MBF-REVIEW-NATIVE-ONLY`. It shows complete or
  truncated, the counts not shown and how many block, per list, and the ceiling. `MBF-REVIEW-TRUNCATED` and
  `MBF-EXPORT-OUTPUT-TOO-LARGE` block Save; "You can still save" needs a complete, eligible review and every other
  condition, the explicit choice included. A save refusal's review replaces the one shown. The dialog does not import
  the wasm engine; `services/mbfEngine.ts` is a test resource. Vitest reads review payloads generated by the worker
  (`fixtures/caepipe_mbf/invented/desktop_export/review_cases.json`, regenerated by `mbf_export.rs`
  `the_dialogs_review_cases_are_the_workers` with `MBF_REVIEW_CASES_REGENERATE=1`; the truncated and over-ceiling
  variants use a test bound).
- **Review fixes (#74's P3 findings).** An error is folded only into an `unsupported` loss entry of its code and
  entity; one matching an `omitted` entry is its own blocking `not_written` entry. Task 3 checks the formed review
  against R in every build and refuses one above it `MBF-EXPORT-WORKER-PROTOCOL`. An empty settings field
  (`settings_len = 0`) is `MBF-WORKER-REQUEST-INVALID`. The decoder's agreement rule already refuses `written` and
  `review-truncated` with a blocking review; both are now tested. The review fixtures are unchanged.
- **4b.2 wires:** task 3 `MbfRender` in `solve_worker.rs` (an `ATST` status-0 frame, then `RSLT`; the workload returns
  `(worker_render(&raw), None)`; pre-arm only `resident_envelope(raw_len) + reserve` and the raw read, parsing after
  arm); `result_limit` at the 64 MiB ceiling; a `MbfWriterRun` over the capped worker; command registration and
  ingress for both commands; the `app_invoke_handler` re-pin; the menu item, the Exports panel and mounting the dialog;
  retiring the TypeScript builder (A-6); and the cancel, newer-review and budget tests of §3.9.6.

**Slice 4b.2a, as implemented (2026-11-05).** 4b.2 is split again: 4b.2a connects the native path, 4b.2b
the frontend. The production entry points run task 3 in the capped worker; `MBF-EXPORT-NOT-CONNECTED` is retired.

- **Worker task 3** (`solve_worker.rs`, a shared file, minimal hunks; Agent 0: MBF owns code 3, hanger sizing
  takes 4). `WorkerTask::MbfRender` is request code 3 in `task_code` and in the request's decode (codes 0 to 2
  unchanged; 4 and above refused). Its `workload` arm returns `(mbf_export::worker_render(&raw)?, None)`: an `ATST`
  status-0 frame, then `RSLT`, or the named failure. The pre-arm check stays `resident_envelope(raw_len) +
  reserve` and the raw read; parsing happens after arm. `supervise` is unchanged.
- **Registry getters** (`solve_jobs.rs`, shared; Agent 0's form): `pool()` loses its `cfg(test)`;
  `worker_launcher()` borrows the registry's own launcher and `process_reserve()` returns its reserve. Nothing
  else there changes; `run_model_checks` is untouched.
- **Ledger bound** (`memory_pool.rs`, shared): `CommandLedger::bound` reserves a `LedgerBound` for leases the pool
  admits for the command outside the ledger (the worker's grant, the request copy, the result lease), as one
  delta before the first of them; `LedgerBound::shrink_to` releases it down exactly.
- **The run** (`mbf_worker.rs`, MBF-owned). `CappedWorkerRun` (§3.7, "The worker's leases"): the bound, the
  bounded queue wait (`MBF_START_WAIT`, 2 s, the model checks' window), the request copy, `supervise` with hooks
  whose `result_limit` is the 64 MiB ceiling, and result custody (`WorkerResultHold`, held in the pipeline's
  `Held` until the response is built). `WorkerOutcome` maps to the named codes: refused grant, growth, footprint,
  request copy or wait (`MBF-EXPORT-MEMORY-UNAVAILABLE`, the review's `MBF-REVIEW-MEMORY-UNAVAILABLE`),
  `ResultTooLarge` (`MBF-EXPORT-OUTPUT-TOO-LARGE`; the review's memory refusal), task 3's own named failure by its
  code, any other failure, an attested result or a stopped worker (`MBF-EXPORT-WORKER-PROTOCOL`), and
  cancellation (`MBF-EXPORT-CANCELLED`). `MbfRuns` (managed app state) holds at most one review and one save in
  progress: a newer review cancels the older (its wait ends, or its worker is killed and reaped before its grant is
  released). `MbfRuns::cancel_all` cancels both (as repaired below).
- **Commands** (`lib.rs`): `review_mbf_export` and `save_mbf_export` are registered in `app_invoke_handler!` and
  take C1's form (§3.7, "Admission"); the save's chooser offers `.mbf` and its library store opens only when the
  document references a library. The save keeps §3.6's order: the `DOTH-MBF-012` route evidence, the settings,
  the admission and screen, task 3 in `save` mode (a review is no authorization: the save's own run refuses a
  truncated review `MBF-REVIEW-TRUNCATED`), the staleness check, then the chooser and 4a's transactional writes,
  backups and refusals, unchanged. The `app_invoke_handler` pin is re-pinned (§3.8; the previous digest
  reproduced first; the macro's only change is the two entries). The pipelines take the command's memory as
  `ReadMemory`; the in-process run and its seam are compiled only for tests.
- **DEL-14-01 r4.** A command's live leases (its argument lease and its growth, every named reservation, the
  ledger bound covering the worker's grant, the request copy and the result lease) are counted in its one ledger
  before the pool admits them, within the budget captured with the argument lease; a budget changed meanwhile
  applies to later commands.
- **Tests** (`mbf_worker_tests.rs`, with the real worker this test binary runs). Connected path: the review
  command replies each production-limit review of `review_cases.json` byte for byte, with the recorded digests, its
  result lease held at the response; the save command writes files whose SHA-256 are `export_digests.json`'s and
  whose bytes are the engine's. These replace the NOT-CONNECTED tests. Then: the ingress refusals; the ledger
  holding the whole budget while the worker runs and the result lease alone after it; the exact boundary (one byte
  above the budget refused by name before any worker, at it the worker starts); a budget raised while a review
  waits; cancellation in the queue wait and the wait's deadline; a newer review killing the running worker, and a
  save's worker cancelled before any chooser; a worker above its grant; a result announced above the ceiling
  refused before any result lease; tampered output (not an envelope, the other mode's envelope, an attested result,
  the supervisor's protocol violations, an exit, a panic, a foreign failure name); and the save's own refusals
  (truncated, over the ceiling, writer refusal, stale review, other route evidence). A `syn` scan holds the
  production references to `worker_render`, task 3 and the run seam, with negative controls; `solve_worker.rs` adds
  the task-code round trip. Mutations checked: no `result_limit`, no ledger bound and the result lease released
  before the response each fail a test.
- **Repair after the review at 9c5785a6d** (2026-11-05):
  - *A superseded save could still write* (P2). The save checks its cancellation before the chooser and commits
    under its run's lock after it (§3.6 step 7): a save superseded while its chooser was open refuses
    `MBF-EXPORT-CANCELLED` and writes nothing, and a cancel after the commit is reported too late while the save
    completes. 4a's transactional save, backups and refusal order are unchanged.
  - *No production cancellation route* (P2). `cancel_mbf_export` (`lib.rs`, a no-argument command like
    `cancel_model_checks`; reply `{cancelled, too_late}`) is registered in `app_invoke_handler!` and re-pinned
    (§3.8). The native session reset (`dispose_all_preview_mechanics_results`, through `reset_native_session`)
    cancels every MBF run before `dispose_all_results`. The frontend's project-session retirement calls it on its
    own (`features/caepipe-mbf/mbfExportCancel.ts`, from `workspaceSession.ts`): the model-check effect cancels
    natively only while a check is in flight, so it would not reach an MBF-only session. `solve_jobs.rs` keeps
    its three getters only. Four Vitest files that list the exact native commands of another flow
    (`App.test.tsx`, `modelStateHistory.session.test.tsx`, `modelDocumentSave.test.tsx`,
    `storedDocumentExport.test.tsx`) now leave `cancel_mbf_export` out of those lists: it writes nothing.
  - *Source scan* (P3). A file declared both as a test-only module and by a production `mod` is scanned:
    production reachability is resolved over every declaration first, with dual-declaration negative controls.
  - *Result lease* (RT-U4). The lease-size check is a hard refusal in every build (§3.7), and a test pins
    `resident_envelope(length) <= RESULT_LEASE_BYTES` for every admissible length, with the envelope's monotonicity
    checked piece by piece.
  - *Tests added:* a save superseded while its chooser is held open writes nothing, and the later save completes;
    a cancel after the commit is too late and the files are written; the session reset cancels a running review and
    a queued save with no model check and releases everything; `cancel_mbf_export` stops the current run and is a
    no-op with nothing running; the pipeline's two boundary checks with an in-process run; `LedgerBound`'s exact
    shrink, drop and unwind and its over-budget refusal; and a Vitest of the session retirement (same-id replacement,
    unmount, no model check). Mutations checked: no commit check, no pre-chooser check, no reset cancel, a cancel
    that ignores the commit, a commit that ignores a cancel, the scan's old exclusion and no frontend retirement each
    fail a test, none by hanging.
- **Repair after the re-check at 6ccc68aff** (2026-11-05):
  - *Retirement missed commands not yet registered* (P2). `MbfRuns` keeps a monotone retirement epoch; each
    command captures it at entry (`lib.rs`, before its reservation, scheduling and admission) and a command
    retired since registers cancelled from the start (§3.7, "Retirement epoch"). The page-load reset is a
    retirement too.
  - *An old session's cleanup could cancel a new session's run* (P2), with Agent 0's added boundary (an obsolete
    start waiting on the retirement must not be sent after it). The frontend barrier (§3.7) serializes starts
    behind retirements, re-checks the caller's generation immediately before every send, tags each result with
    its generation, and states its fail-safe rule for an unacknowledged retirement. `review_mbf_export` and
    `save_mbf_export` join `NATIVE_INGRESS_COMMANDS` and its test twin, sent only through the barrier.
    `cancel_mbf_export` keeps no arguments.
  - *A too-late cancel stopped tracking a committed save* (P3). Every run stays tracked until its command ends;
    committed saves, superseded ones included, are reported too late by every cancel during their write.
  - *Scan path spellings* (P3). The scan normalizes resolved module paths lexically (`.` dropped, `..` taken back)
    before comparing declarations, with aliasing negative controls.
  - *Tests added:* Rust, a review and a save retired before admission and before registration, by
    `cancel_mbf_export` and by the page-load reset (no chooser, no write); a command entering after a retirement
    is unaffected; a committed save stays too late through repeated cancels and supersession. Vitest: a start
    issued during a delayed retirement is sent only after it, and is not cancelled; old starts waiting on a
    retirement are never sent after a same-id replacement, a failed caller check, unmount, or Strict Mode's double
    effect; a delayed old completion is tagged not current; an unacknowledged retirement (rejected, or thrown)
    refuses `MBF-EXPORT-RETIREMENT-UNCONFIRMED` until a re-delivery is acknowledged; a start retired during a
    headroom re-send is not sent again. Mutations checked: no entry-epoch check, committed runs dropped at a
    cancel, a committed predecessor dropped at supersession, unnormalized scan paths, no frontend await, no
    generation re-check before the send, a failed retirement treated as acknowledged and no session retirement
    each fail a test, none by hanging.
- **Repair: the dispatch ticket** (2026-11-05, the last repair's open matter 1). Native "entry" was the first
  statement of the command body, which Tauri 2 runs in a spawned task. A start sent before a retirement whose
  task first ran after it took the new epoch and escaped the retirement. Agent 0 confirmed the ticket design with
  refinements, so the epoch now travels in the request (§3.7, "Dispatch ticket"): it is captured once per logical
  start, kept through every re-send, validated before any model decode, and authorizes nothing. `lib.rs` no longer
  captures an epoch. `cancel_mbf_export` replies the epoch. The MBF output bytes are unchanged.
  - *Tests added:* Rust:
    - a review and a save built with the old ticket, retired by `cancel_mbf_export` and by the page-load reset, and
      only then run: `MBF-EXPORT-CANCELLED`, no worker launched, no chooser, no write, nothing tracked;
    - a missing, `null`, string, negative, fractional, oversized or future ticket refuses
      `MBF-EXPORT-EPOCH-INVALID` for both commands, before a model member of the wrong type is reached;
    - the earlier tests now run with current tickets: a fresh start after each retirement reviews and saves the
      recorded files, and a stale review or wrong route evidence with a current ticket is still refused.

    Vitest:
    - the first start in a page waits for one shared handshake and carries its epoch;
    - a start issued while a retirement is on its way carries that retirement's epoch;
    - an old start in a headroom re-send loop across a retirement keeps its old ticket, is never re-sent, and
      resolves cancelled, while a new start carries the new epoch;
    - a reply without an epoch is unacknowledged;
    - the session tests check each start's ticket.
  - *Mutations checked,* each failing a test, none by hanging: native code that ignores the ticket; a frontend
    ticket captured when the start is called; one epoch behind; no handshake; no ticket sent; no re-check before a
    re-send.
- **Repair: serialized retirements and a non-wrapping epoch** (2026-11-05, the independent re-check at
  f77172935).
  - *Overlapping retirements could cancel a fresh run* (P2). Each delivery replaced the barrier's latest, and a
    start waited only for that one; Tauri's invokes are independent requests, so retirement A sent before B could
    be applied natively after B was acknowledged and a fresh start sent. Retirement deliveries are now a chain
    (§3.7, "Serialized retirements"), headed by the page-load reset.
  - *The epoch could wrap* (P3). `MbfRuns::retire` advances it with `checked_add`; on overflow the export is
    exhausted (§3.7, "Exhaustion"), with the new code `MBF-EXPORT-EPOCH-EXHAUSTED`.
  - *Tests added:* Rust: the epoch seeded at `u64::MAX - 1` and retired twice, with a review running at
    `u64::MAX`: no wrap, the review cancelled and its worker reaped, the reply `exhausted`, every ticket refused
    `MBF-EXPORT-EPOCH-EXHAUSTED` for both commands with no worker and no chooser, an earlier entry cancelled at
    registration, later retirements still exhausted. Vitest: retirements A then B and a fresh start, answered
    newest first by a host that cancels runs below each applied epoch: B is sent only after A settles, the start
    after B, and it is not cancelled; a failed A then an acknowledged B: the start is sent with B's epoch, without
    a re-delivery or a hang; no retirement, and so no start, is sent before the page-load reset settles; an
    exhausted reply is unacknowledged. Mutations checked: no chain (the newest-first test fails), and an
    unchecked advance (the boundary test fails).
- **Repair 5: idempotent retirement** (2026-11-05, the independent re-check at 8cecfc1ce; Agent 0's
  disposition).
  - *A settled invoke is not every delivery* (P2). Tauri's transport may deliver one invoke twice (§3.7,
    "Idempotent retirement", with its source basis and limits). `cancel_mbf_export` now takes `{retirementId}`
    through the raw-body ingress and applies each id once; a duplicate or older id is a truthful no-op; applied
    page tokens are never forgotten, and a new token beyond 64 exhausts the export. The barrier gives each
    logical retirement its id, reuses it on re-delivery, and accepts a receipt only for its own id.
  - *The reset* (option (a)). The performance reset's unkeyed MBF retirement is removed:
    `dispose_all_preview_mechanics_results` is again exactly its pre-4b.2a body, and `reset_native_session` is
    gone. The workspace sends a keyed retirement at mount (page load included), and the barrier no longer waits
    for the reset. The ticket, the generation guards and the pre-commit rules are unchanged.
  - *Tests added:* Rust: the reset leaves a running review and the epoch alone, and the new page's keyed mount
    retirement stops it; an injected-fault schedule (the resend applied, a fresh review running, the delayed
    original and an older id of the same page each a no-op with a truthful receipt, a new page token stopping
    the review); 64 tokens applied, a fresh review running, a delayed duplicate of the oldest a no-op, then a new
    token exhausting the export (the review stopped, the epoch unmoved, no token evicted, starts refused); a
    missing, `null` or malformed id refused `MBF-EXPORT-RETIREMENT-ID-INVALID` with nothing cancelled or
    recorded, and a request without the ingress header or above the argument bound refused at ingress; the
    earlier retirement tests run with the keyed page-mount retirement in place of the reset. Vitest: ids (one
    page token, rising sequences, a new token per page); the injected-fault schedule against a host mock that
    applies the keyed rule, with an in-test control showing a host that ignores the sequence cancels the fresh
    start; a re-delivery reuses the id and the duplicate receipt acknowledges; a headroom re-send reuses the
    body; a receipt for another id is no acknowledgement; the workspace mount sends a keyed retirement with no
    model check or export open, and the first start waits for it; the barrier neither waits for nor sends the
    reset.
- **Repair 6: retirement after the transport's fallback** (2026-11-05, the independent re-check at
  d4517a4e5; Agent 0's option (ii)).
  - *P2.* After Tauri's `postMessage` fallback a raw-body cancel arrived as a JSON number array and was refused,
    so an admitted save could pass its pre-chooser check and its commit and write despite the retirement. The
    frontend now always sends `cancel_mbf_export` as Tauri's JSON arguments, and the host admits that one form
    for this command beside the raw form, through the one keyed path (§3.7, "Wire form and bounds", with what
    the 4 KiB bound does and does not limit). The starts stay raw-body only; no other command changes, and the
    handler list is unchanged. `cancel_mbf_export` leaves the frontend's ingress command lists.
  - *P3.* The code and this Design no longer say the page-load reset bumps the epoch (the keyed mount retirement
    does), and distinguish the two exhaustion states, their recovery by restart, the 65th-page-load limit and
    the ids' lack of authentication (§3.7).
  - *Tests added:* Rust, with the bodies Tauri 2.11.1's transports deliver (the custom protocol's JSON body; the
    fallback's message parsed as Tauri does, its `payload` the body and its `options.headers` the headers): an
    admitted save waiting at its chooser, then the raw form after the fallback refused `INGRESS-RAW-BODY-REQUIRED`
    with nothing stopped, then the JSON form applied, the save replying `MBF-EXPORT-CANCELLED` with nothing
    written and its ledger and grant released; one id delivered raw and as JSON (each order) applied once, the
    second a `duplicate` no-op; every malformed id refused `MBF-EXPORT-RETIREMENT-ID-INVALID` in the JSON form
    too, a JSON body that is not an object or is over 4 KiB refused at ingress. Vitest: an emulation of the
    transport (the IPC replacer, the fallback's message) and of the host's accepted forms; an admitted save,
    then the fallback, then a retirement whose reply is lost: the save is cancelled, and the next start
    re-delivers the same id in the JSON form (a duplicate, acknowledged) and is itself refused raw; the cancel
    is sent as JSON with no ingress header, and the raw body does not survive the fallback. The headroom re-send
    test of the cancel is removed: the JSON form takes no lease. Mutations checked: a host that refuses the JSON
    form fails the regression, the duplicate test and the malformed test; a frontend that sends the cancel raw
    fails both fallback tests.

**Slice 4b.2b, as implemented (2026-11-06).** The frontend: the menu item, the Exports panel action and the mounted
dialog on 4b.2a's native path, and the TypeScript builder's retirement (A-6). No native command, worker, format or
identity file changes; `app_invoke_handler` is unchanged, so the reach-scan pin is too.

- **Entry points** (§3.1). `file.export-mbf` ("Export MBF…") follows Save Handoff… in the native File menu
  (`lib.rs` `NATIVE_FILE_MENU`, disabled while a project operation is busy like its neighbours) and in the in-window
  File menu (`App.tsx`), and dispatches through the one command sink (`workspaceSession.ts` `runMenuCommand`). The
  Model batch file panel's one action (`CaepipeMbfExportPanel.tsx`, "Export MBF…") calls the same
  `requestExportMbf`. Both apply the panel's output gate (`outputGateRefusal`, exported from
  `LoadReferenceOutputGate.tsx`; `OUTPUT_POLICY` is unchanged), and both refuse by name while the session's model hash
  is not yet shown.
- **The request.** `requestExportMbf` takes the session's model and its owned hash (`modelDocumentSaveBasisRef`, as
  Save Model Document does), resolves the model's library references, and opens the dialog unless the session moved
  on meanwhile. It supplies the barrier's `reviewMbfExport` (or null off the native runtime, so the browser-only route
  declines with `MBF-REVIEW-NATIVE-ONLY` before anything reaches the barrier), its `saveMbfExport`, and `retire`
  (the session's one retirement for this request, `mbfExportSession.ts`). The dialog is modal like Save Model Document: the workspace is inert and menu commands,
  accelerators, close requests and project commands are held while it is open (`mbfExportDialogOpenRef`).
- **Close and stale results.** Close, Escape, an App unmount and a session replacement each send one logical
  retirement through the barrier (`mbfExportSession.ts`): an open dialog request is the session's retirement until the
  user closes it and its retirement has been sent, so the dialog's cleanup and the session's end call one retire, and
  only the first call sends; with no request held, the session's end retires on its own (4b.2a's). The dialog's
  services take its own `stillCurrent` (a review its newer review or close superseded; a save whose dialog closed),
  which the barrier checks before every send, and the dialog discards any result whose `current()` is false or whose
  dialog has closed. Each new request starts with Save available (`saving` and the rest of its state are reset), so a
  save of a closed dialog never holds the next one. A native `too_late` means only that the close crossed the save's
  commit: `write_pair` still runs and can fail or undo itself. So a save that had not settled when its request was
  retired is the session's to report: a `too_late` receipt reports it as finishing, and its own eventual outcome is
  reported, saved or refused, whatever the receipt said. The receipt is no proof that a save wrote nothing: a save
  that completed natively before the cancel arrived (a write failure or an undone save included) is no longer
  registered, so it is counted in neither total, and an unacknowledged receipt says nothing. Only an outcome that
  itself establishes that nothing was written is not reported: `MBF-EXPORT-CANCELLED` (refused before the commit),
  a dismissed chooser, a refusal before the chooser that carries the review, and the stale or not-sent refusals
  (`MBF-EXPORT-REVIEW-STALE`, `MBF-EXPORT-RETIREMENT-UNCONFIRMED`, `MBF-EXPORT-EPOCH-INVALID`,
  `MBF-EXPORT-EPOCH-EXHAUSTED`), each only when every code of the refusal is one of them.
  `INGRESS-RAW-BODY-REQUIRED` is not among them (repair round 3): Tauri 2.11.1 resends an invoke through its fallback
  when its `fetch` fails or when reading its reply fails, so the original save may have committed, written or undone
  itself before the resend was refused, and the refusal proves only that the resend wrote nothing. A closed save
  refused with it, alone or with other codes, is reported as unknown (`kind: "unknown"`, "has no confirmed result",
  status operation `export_mbf_outcome_unknown_after_close`). The outcome's line says the export was closed before its result came back. The report keeps the request's project
  and session: after a new project session it is named as an earlier session's export, never as the current project's.
  A new project session closes the dialog. The barrier, its ticket, its ids and its guards are mounted as they are;
  nothing re-implements them.
- **Product text** (§3.3, "The export's own refusals"). `MBF_EXPORT_REFUSAL_TEXT`, `mbfRefusalText`,
  `closedSaveStatusLine` and `mbfFailureDiagnostic` in `mbfExport.ts`: the dialog shows each code with its product text in place of the native wording, and a start
  refused `INGRESS-RAW-BODY-REQUIRED` declines the review with "Export isn't available in this window", while a save
  refused so, open or closed, shows `mbfSaveRefusalText`'s unknown-outcome text (§3.3), never "Nothing was written".
  The barrier cannot refuse a start for a known fallback before sending it (the flag is private to Tauri's transport
  script), so every save refused with this code is treated as unknown.
- **A-6.** `CaepipeMbfExportPanel.tsx` no longer builds a package or MBF text: `buildCaePipeMbfExportPacket`, its
  package and MBF text links (with their `ControlledExportLink` route entries), its project-id file names, its
  length conversion and witnesses and its `MBF-LENGTH-*` and `MBF-PROJECT-UNIT-MISSING` codes are removed, with the
  tests of them (`exportLengthConversion.test.tsx`, `exportProjectUnitRefusal.test.tsx` keep their PCF and local FEA
  cases). `services/mbfEngine.ts` stays as the test resource 4b.1 left it; no product source imports it.
  `apps/desktop/native-smoke/expected.json` is unchanged; the smoke driver's MBF control probe and the Playwright
  smoke's panel assertion follow the new action (neither was run). The live records that described the preview
  package are reconciled with the native export: `ExportReviewPanel.tsx`'s `caepipe_mbf_export` record and
  `ExportAdapterSdkPanel.tsx`'s target now state the native writer, the local-private route `DOTH-MBF-012` with its
  explicit choice, a model-only payload with no result data, the identity file, and (in the review record) that the
  files hold private project data (`private_payload_included: true`, so the manifest's private-payload count is 1).
  Their `export_id`/`target_id`, `document_kind` and `target_family` keep their historical spelling. The preview's
  public report template and its unit and conversion-witness surface in `ReportLintPanel.tsx` are retired, not moved
  onto the local-private model files (unit-policy targets 44 to 43, conversion-witness targets 2 to 1, lint targets 46
  to 45; the export review's lint record and the Playwright smoke follow, unrun). `redactionExportControls.ts` keeps
  its handlers for the preview package's document kind, for historical payloads.
- **Tests.** Vitest: `App.mbfExportDialog.test.tsx` runs the real App against a mocked native host: the native menu's
  event and the in-window File menu item open the dialog, the Exports panel's action opens it and focus returns
  there, review then save through the native commands (the canonical text and its hash, the reviewed digests, the
  route evidence, the same ticket for both), Cancel and Escape each send one retirement, the browser preview declines
  before any review is sent, a review answering after its dialog closed is discarded, a save writing at the close is
  reported written, the exhausted and fallen-back states' product text, and unmounting the App with the dialog open
  retires the export. `MbfExportDialog.test.tsx` adds: close and unmount call `retire`; non-current review and save
  results are discarded; the dialog's own check reaches the barrier; each code's product text; the texts name no
  mechanism and grant nothing; and a source scan that no product source keeps the builder, its links or the wasm
  engine. `App.test.tsx`'s MBF block now opens the mounted dialog from the panel (browser route) and closes it, in
  place of the retired summary and profile. Rust: `native_file_menu_offers_export_mbf_after_save_handoff` and the
  busy-state test's list. Repair round: `mbfExportSession.test.ts` holds the retirement counts in every order
  (close, unmount, session replacement with the dialog open, no dialog, a request never mounted) and the closed-save
  reports (each of a saved, write-failed, undone and thrown outcome under a receipt of zero, an unacknowledged one and
  a too-late one; the no-write outcomes silent under each); `App.mbfExportDialog.test.tsx` holds a save closed after its commit that then saves, fails to write, or is
  undone, one that completes after a new project session (named as the earlier session's), one that had completed
  natively before the close (a `too_late` of 0) and was undone, reopening during a pending
  save before and after it settles, and exact retirement counts for Close, Escape and an App unmount. Repair round 3
  (P1, the fallback's resend): a save whose original committed and whose resend was refused
  `INGRESS-RAW-BODY-REQUIRED` is reported as unknown under each receipt (`mbfExportSession.test.ts`, with a mixed
  refusal), shows the unknown-outcome text in the open dialog (`MbfExportDialog.test.tsx`,
  `App.mbfExportDialog.test.tsx`) and in the status line once the dialog has closed (`App.mbfExportDialog.test.tsx`);
  a review refused with the same code still says nothing was written; the other no-write codes stay silent. A session
  replacement with the dialog open cannot be reached through the App (every session-replacing command is held while
  it is open), so its count is held at the owner.
- **Open.** Criterion 6 is not claimed: the reference system still needs the coverage tranches and the round-trip
  check (slices 5 and 6). The panel and menu keep the `caepipe-mbf` output gate although the export now carries no
  result data; its removal is a separate follow-up (done: "Output gate removed" below). No App launch, Playwright,
  native smoke or model run was made.
- **DEL-17-05's external-run panel** (repair round 2, by Agent 0's ruling that no misleading live consumer is left for a
  later owner). DEL-17-05-REQ-016 requires external-run evidence to bind to *a* DEL-17-04 MBF export package
  reference, and its schema requires only a non-empty `mbf_package_ref` and `input_mbf_path`; neither requires the
  retired preview's identifier. The panel (`CaepipeExternalHarnessPanel.tsx`) and the export review's
  `caepipe_external_run_evidence` record keep that reference (`caepipe-mbf:desktop-preview-del-17-04`) and the
  preview's input name (`download:openpipestress-preview-mbf-package.json`) as historical identifiers, and state that
  the input is not available: `mbf_package_status`/`mbf_package_availability` say
  `historical_retired_preview_package_not_available`, `run_directory.input_mbf_path` is `not_available`, the
  `mbf_package_bound_to_del_17_04` check reports `historical_reference_not_available` (never `pass`), and a warning
  diagnostic (`CAEPIPE-EXTERNAL-MBF-INPUT-NOT-AVAILABLE`) names it. The panel's state-binding line marks the reference
  "(historical; not available)". The Python run package (`core/handoff/caepipe_external/run.py`), its fixtures and the
  schema are unchanged.

**Output gate removed (follow-up to 4b.2b, 2026-11-06).** Agent 0's ruling: the same canonical model and settings must
not become unexportable because an unrelated result is held; the gate protected no result data in these files. Only
the result-output gate changes; criterion 6 is not claimed.

- **Old policy.** `caepipe-mbf` was one of `OUTPUT_SURFACES`' 21 entries, refused by every gated route's table
  (`pressure`, `pressure_source`, `retained_preview_physics`, `retained_physics`). The panel wrapped its Export MBF…
  action in `LoadReferenceOutputGate` on the Current result (a refusal in place of the action), and `requestExportMbf`
  refused the File menu's item with `Export MBF… is not available: <reason>` (`export_mbf_refused`) through
  `outputGateRefusal(currentSolvedResultRef.current, "caepipe-mbf")`. The ungated routes (`preview_physics`,
  `unsupported`) and no result added no refusal.
- **New policy.** `caepipe-mbf` is not an output surface: `OUTPUT_SURFACES` has 20 entries (seventeen in T1's group
  a), and no route's table names it. The panel takes no result and renders its action whenever the session can open
  the export; `requestExportMbf` reads no result. Both entry points stay as enabled as before for the model: the
  native item while no project operation is busy (`lib.rs`, unchanged), the in-window item while a model is present
  and no project operation is busy (`modelDocumentSaveReady = model !== null && !projectBusy`, `App.tsx`,
  unchanged), and either invocation still refuses by name while the model's hash is pending (`requestExportMbf`). Every other surface's decision, for every route, is unchanged; nothing else is admitted.
- **Kept.** The model's admission and screening, the local-private explicit-intent route `DOTH-MBF-012`, the reviewed
  digests, the loss, completeness, size and budget checks, cancellation (the barrier, its ticket and its retirements)
  and the file-transaction guards. No native command, worker, format, identity file or reach-scan pin changes: the
  native commands take only `{canonicalText, claimedModelHash, settingsJson}` (review) and those plus `suggestedName`,
  `reviewed`, `screening` and `localFirstEvidence` (save) (`mbf_export.rs` `review_mbf_export_args`,
  `save_mbf_export_args`); `mbf_worker::reserve` uses the registry only for the memory lease, so no result can reach
  them.
- **Tests.** `App.mbfExportUngated.test.tsx` runs the real App against a mocked native host whose solves replay
  committed producer bytes. For each context it exports from the File menu and from the panel with no result, then
  again beside the result, and asserts the four `review_mbf_export` and `save_mbf_export` requests are equal (canonical
  text, hash, settings, suggested name, reviewed digests, route evidence, the ticket's type), and that the export sent
  only its own commands, with zero of every result read (the take, the full copy and its release, dispose, the native
  result pages, verdict, summary and record preview, and the restored and custody full reads). Contexts: a PF
  (preview-physics-2) result; an exact (pressure-2) result; a retained route (the exact result read as
  `retained_physics` through a `sourceContract` double: no product caller delivers a successor and the native replay
  records no reply for its bytes); and a held result (the exact result held natively with its full copy also held in
  the webview, C2). Against the old gate the exact, retained and held cases fail with the refusal by name; the PF case
  passes there too (its route was ungated). The panel tests (`loadReferenceOutputRefusal.test.tsx`,
  `retainedPrecisionOutputRefusal.test.tsx`) move the panel out of the gated lists and assert the action is offered
  beside each load/reference-state identity, a retained successor, a PF result and no result; `outputPolicy.test.ts`
  drops the cell and asserts no route names it. The native smoke's exact-route check now expects no refusal, the
  action and no download link (`driver.js`, `run.mjs`; not run); `expected.json` does not pin MBF and is unchanged.
  The Playwright smoke had no gate assertion. A result held back by its numerical verdict or by an RT-A5 held-status
  gate could not be delivered into an App session here (the replay records no reply for the session's delivery of
  those bytes); the export reads no result at all, so its route, standing or hold cannot reach the request.

**Export unavailable after an unknown save (follow-up to 4b.2, 2026-11-06).** Agent 0's inspection: an unknown save
outcome left Save enabled. The dialog's `canSave` did not read the unknown diagnostic, and its `finally` cleared
`saving`, so the user could start another save the transport could not carry. Only the frontend changes. No native
command, IPC form, protocol or backend changes.

- **Old behaviour.** A save refused `INGRESS-RAW-BODY-REQUIRED`, as an outcome or as a rejection, showed the
  unknown-outcome text (§3.3), and then Save was offered again with the same review and choice. A settings change, a
  new review or a reopened dialog sent `review_mbf_export` again, and the ingress refused it.
- **New behaviour.** The page state lives in `mbfExportCancel.ts`, beside the page token, as "Export unavailable in
  this page". The barrier's `saveMbfExport` judges every save it sends, whether its dialog is open, closed or
  replaced and whether or not the result is current. A save whose outcome is unknown (`mbfSaveOutcomeUnknown`, either
  form) marks the page before its outcome is returned. The dialog marks it too on the same rule, so its Save holds
  whatever service it is given.
  - From then on every review and save start is refused before any send, with `MBF-EXPORT-UNAVAILABLE`. That covers a
    start that was waiting on a retirement when the page was marked, and a headroom re-send, which resolves
    cancelled. Save is disabled (`useSyncExternalStore` on the page state), and the dialog starts no review: it
    declines with the code and its text (§3.3).
  - Settings edits, a new review, a close and reopen and a new project session leave the state as it is. Only a new
    page clears it: a reload, or a restart. Tests reset it through `resetMbfExportUnavailableForTests`, from
    `src/test/setup.ts`, as a fresh page.
  - Retirements are not held. `cancel_mbf_export` is sent in the JSON form, which still reaches the host after the
    fallback (§3.7), so an admitted save that has not committed can still be stopped.
  - The unknown save's own outcome is shown as before: unknown, never saved, failed or nothing written. Nothing is
    inferred from the transport, a retirement, a too-late receipt or a lost reply. `MBF-EXPORT-UNAVAILABLE` is one of
    the closed-save codes that establish nothing was written, because such a start is never sent.
  - Every other refusal keeps its retry behaviour: cancelled, stale review, the retirement and epoch codes, a write
    failure, and a review refused `INGRESS-RAW-BODY-REQUIRED`. That review wrote nothing, and its own text already
    says to reload or restart. The rule follows the unknown save only.
  - §3.7's "the barrier cannot refuse a start for a known fallback" still holds for the first start after the flip,
    since the flag stays private to Tauri. The page refuses only after a save's unknown outcome has shown it.
- **Tests.** `mbfExportCancel.test.ts` covers the following. Either form marks the page. A save its caller retired
  while it ran marks it too. No further start is sent and each is refused by name. A retirement is still sent and
  acknowledged. A subscriber is told once. A start that was waiting on a retirement is not sent. A headroom re-send
  stops. Other refusals and failures leave the page available. A fresh module (a reload) exports again.
  `MbfExportDialog.test.tsx` covers the following, for both forms. Save is disabled. Settings, a new review and a
  close and reopen start nothing and show the text, and the dialog's retirement is still called. A closed dialog's
  unknown save marks the page. A fresh page saves. Five other refusals can be tried again. `App.mbfExportDialog.test.tsx`
  runs the real App and barrier for both forms. The guidance is shown and Save is disabled. A settings change, a close
  (one `cancel_mbf_export`) and a reopen send no further `review_mbf_export` or `save_mbf_export`. After a fresh page
  the review is sent again. A closed dialog's committed save whose reply was lost leaves the reopened dialog
  declining with no review sent. With `markMbfExportUnavailable` made a no-op, twelve of the new tests fail. The
  retry, text and fresh-page tests pass, as they should.

**Slice 5 design candidate (2026-11-07; for independent review, nothing connected).** §4.8 records the source,
memory and transaction design that slice 5's row requires before connection. In summary:

- **Engine route.** The import runs in a new worker task 5 (`WorkerTask::MbfImport`; tasks 0 to 4 are
  unchanged) in `review` and `adopt` modes.
- **Commands.** `open_mbf_import`, `review_mbf_import`, `adopt_mbf_import` and the keyed
  `cancel_mbf_import`, with the import's own epoch and tokens.
- **Low custody.** The host keeps one source record of pinned read-only handles and digests, and no
  model-sized custody between commands. Each review and Adopt re-reads the files under that command's
  ledger and refuses any change.
- **Review.** The webview sees only a bounded review (at most 1 MiB) until Adopt.
- **Adopt.** Adopt goes through `handleOpenModelDocument` and `checkNativeOutcome`, after the existing
  unsaved-changes guard, with the reviewed digest checked natively and in the webview.
- **What it replaces.** §4's earlier wasm route and its "files are not read again" rule (§4.1, "What
  changed from the earlier text, and why").
- **Policy application and open matters** (§4.8.5, §4.8.11). New import stages and handoff bytes are independently
  admitted; the final document uses the existing Open consumer. Exact independent review and connected comparison
  checks must establish that boundary. Common-consumer heap qualification remains under RT-U4; no new allowance or
  risk acceptance is inferred.

Criterion 6 is not claimed.

## 7. Boundaries

- **No compliance or acceptance claims.** SWBPIPE makes no code-compliance claim. Piping-code,
  hanger and analysis options pass through as target configuration only (REQ-006, REQ-015). A
  successful export or import is not compatibility or validation evidence. CAEPIPE is the
  validation reference (RT-V2), and that work is separate.
- **No protected content.** This design describes the MBF format only as far as the merged
  code and the import Design's admitted public sources do. It copies no vendor documentation
  and adds no fact about the target program.
  - Product text stays generic (the owner's public-text ruling).
  - Material values need affirmative provenance. Allowable, strength, fatigue and code data
    never enter the model (REQ-019).
- **Private data.** An MBF written by CAEPIPE, and everything derived from it, stays private
  and is never committed (CLM-006). Public tests use only invented MBFs that the project wrote
  itself.
- **Owner's acts.** Exit criteria are evidence for the owner's release decision, not the
  decision. Standard claim fence applies (claims taxonomy per DEC-081).

## 8. Open decisions and questions

### 8.1 For Agent 0

**Ruled by Agent 0, 2026-10-28; A-3 amended by the owner, 2026-11-08.** The table keeps the
original proposals; where a ruling or the owner's later amendment differs, it governs.

- **A-1:** the locality is accepted. The §8.3 texts take one independent review on their exact
  Old/New, then Agent 0's confirmation. Slices 4 and 5 wait for them; slice 2 does not.
- **A-2:** accepted. Slices 2 and 3 take numerical review. Their parity bar is the whole
  committed MBF corpus, byte for byte both ways, plus the authored cases. The writer is ported
  after the SIF fix lands.
- **A-3 (amended by the owner, 2026-11-08; option 3, "Family list amended", of import
  Design §20.5.13):**
  - "Must family" means every family the reference system contains under criterion 1, plus
    the cases it holds; each traces to its Must row. Combinations, which combine solved
    results, are outside criterion 6's families; RT-A4 coverage is judged elsewhere.
    The RT-row list in the original proposal table is a cross-check, not the definition.
    The other refusals in import Design §20.5.10 still stand, so criterion 6 remains unmet.
    No reference-system round-trip claim is made.
  - The identity set is as proposed, sections included, so Q-2's format 4 is a requirement.
  - The project id is excluded because it identifies the store's project, not a model entity,
    and an import must not collide with it (§4.4).
  - A family with no MBF record goes to the criterion 3 limitations list, which the owner
    accepts at release. Nothing about that is decided here.
- **A-4:** a new `DOTH-MBF-012` (local_private, explicit intent, lossless), with parity tests in
  all three catalogues and an independent permissions review.
- **A-5:** the sibling `<stem>.swbpipe-ids.json`. An unbound import says why it is unbound.
- **A-6:** retired when slice 4 lands. The stop rule on `apps/desktop/native-smoke/expected.json`
  stands.
- **O-1:** the reference system is an invented fixture with its run sheet (§5), owned by
  DEL-09-01 (`Work/reference-system.md`) since Agent 0's placement of 2026-10-31.

| Id | Decision | Recommendation | Consequence | Blocks |
|---|---|---|---|---|
| A-1 | Locality of the desktop actions: DEL-17-04 CLM-008 excludes GUI behavior, and CLM-012 gives desktop import to DEL-07-12 | Keep the GUI code in DEL-07-12's paths and the engine, native file logic and criterion-6 check in DEL-17-04, under this design; apply §8.3 | The ScopeOfWork text changes, which is a commitment change and takes independent review. Without it, slices 4 and 5 build outside both deliverables' stated scope | 4, 5 |
| A-2 | Engine route (§2) | R: one Rust crate, native and wasm, with the Python as twin | About 6,000 lines ported. Each tranche lands twice until Q-1 retires one side. Numerical review for slices 2 and 3 | 2–6 |
| A-3 | The meaning of "Must family" and "same entity identities" in criterion 6 | Must families: the model-input families of Must rows RT-G1, RT-G3, RT-K1 to RT-K4, RT-S1 to RT-S3 and RT-L1, plus the Must analysis families RT-A4, RT-A6 and RT-A9 that the model holds as cases or combinations. Identity set: nodes, pipes, components, supports, load cases, primitive loads, materials, pressure regions and sections. The project id is excluded | §1.4 shows most of these refused or omitted today. Meeting the criterion as written needs the coverage programme (Q-3). Any family with no documented MBF record can only be listed under criterion 3 for owner acceptance. Because the exit criteria are owner-accepted, a reading that narrows them goes to the owner | 6 |
| A-4 | Governed route for the save | A new `DOTH-MBF-012` (`local_private`, explicit intent, lossless), not `DOTH-FORMAT-003` (`downstream_tool`) | A new storage route in three catalogues, with parity tests and independent review. A downstream-tool route that redacts would break the round trip | 4 |
| A-5 | Where the identity file lives | A sibling `<stem>.swbpipe-ids.json`, named by native code. Not chosen: a copy in the local store keyed by MBF hash (lost on another machine and invisible to the user); a package folder (the target program reads a plain `.mbf`) | The user moves two files together. Moving one alone gives an unbound import that says why | 4, 5 |
| A-6 | The current TypeScript builder and its links | Retire `buildCaePipeMbfExportPacket`, its anchor links and project-id names when slice 4 lands, and repoint DEL-17-04's desktop checks | Tests that pin the preview change. If `apps/desktop/native-smoke/expected.json` (owner-reserved) would change, the slice stops and reports | 4 |
| O-1 | The reference system is not yet a named file | Author it as an invented fixture with its run sheet (criteria 1 and 7), name it in the release target, and add it to §5's models | Until it exists, criterion 6 is shown only on the coverage fixture | none (the owner's evidence) |

### 8.2 For the CAEPIPE lane

**Confirmed by the CAEPIPE lane as recommended, 2026-10-28, through Agent 0:**

- **Q-1:** byte-parity port accepted.
- **Q-2:** sidecar format 4 adds `section_map` ids. Formats 3, 2 and legacy keep their current
  handling, and the self-digest is unchanged. Implemented (import Design §15, **Format 4: section
  identities**).
- **Q-3:** admitted public sources exist today for layout, bends, anchors, guides, limit stops,
  skewed restraints, hangers (semantics only), materials, force loads, load cases, OPTEMP and
  OPPRES. None exist yet for valves, reducers, flanges, rigid elements or nozzles. The lane is
  adding source rows for the reference system's families; until they exist, those families are
  authored in the fixture and listed in the loss report.
- **Q-4:** `max_layout_rows=None` in the product.
- **Q-5:** OPTEMP goes in a typed model member. The lane first checks for an existing temperature
  member with the same meaning. A new member would be a model-document change through Agent 0.
- **Q-6:** the SIF fix and the omitted-entries sweep are the lane's first code item after its
  current batch, on a branch from main whose head it names. Slice 2 starts from that head.

| Id | Question | Recommendation | Consequence | Blocks |
|---|---|---|---|---|
| Q-1 | Does the lane accept the Rust port with byte parity? Later, should the harness call the Rust build so that one implementation remains? | Accept parity now, and decide on retiring one side after the coverage tranches settle | Two implementations to keep in step until then | 2, 3 |
| Q-2 | Should a sidecar format 4 record section ids (and later the ids of non-bend components)? | Yes, as format 3 plus `section_map` id entries, with the same every-field-required rule | Without it, C6-5 excludes sections and A-3's identity set shrinks | 6 |
| Q-3 | What is the coverage order for §1.4's refused families, and which have admitted public sources? | Order by the reference system: tees with user SIFs, reducers, valves, flanges and concentrated weight, further thermal cases, hydrotest, wind and static seismic, combinations. The nonlinear supports and hangers follow the solver lane's operating case | It sets when criterion 6 can close, and which families become release limitations | 8+ |
| Q-4 | Should the product write with `max_layout_rows=None`, and does the row-limit warning belong in product text? | `None` in the product, with the warning kept for the harness only | No evaluation-host text in the product dialog | 4 |
| Q-5 | The reference temperature is a writer argument, not a model member. Should the importer record OPTEMP in the document? | Record it in a model member or `project.notes`, so that re-export reproduces the bytes without the session's memory | Without it, byte reproduction after a reopened project depends on the user re-entering the value | 4 |
| Q-6 | Should everything the writer does not write be listed as `omitted` entries: non-empty members (lines, constraints, equipment interfaces, rule-pack references) and fields it does not read, such as a bend's user SIF modifiers? | Yes, one entry per member or field, so the export review is complete and REQ-004 holds. The SIF case is a silent drop today, so the writer's fix comes with a regression test | Today the user sees one assumption sentence for members, and nothing for SIFs | 4 |

### 8.3 Proposed ScopeOfWork text (A-1)

No ScopeOfWork is edited by this design. These are the exact texts after one independent review
(2026-10-28), which found that each Old occurs exactly once when unwrapped and replaced AC-003's
coverage sentence so that it does not understate A-3. Each Old and New is one line in its file;
each sits inside the existing paragraph, keeping its prefix.

**DEL-17-04 CLM-008** (inside its `>` paragraph).

- *Old:* "It shall not implement a CAEPIPE result parser, external run harness, public API, GUI behavior, release claims, CAEPIPE compatibility claims, code-compliance claims, professional-acceptance claims, or full MBF coverage claims."
- *New:* "It shall not implement a CAEPIPE result parser, external run harness, public API, release claims, CAEPIPE compatibility claims, code-compliance claims, professional-acceptance claims, or full MBF coverage claims. Its desktop part is limited to the engine, native file commands and round-trip check of Design `desktop-mbf-export-import.md` (RT-R7, RT-R8); the GUI code of those actions is DEL-07-12's."

**DEL-17-04 CLM-012** (inside its `>` paragraph).

- *Old:* "A desktop MBF import action would belong to DEL-07-12 and is not part of this deliverable."
- *New:* "The desktop MBF export and import actions use this deliverable's engine and native file commands under Design `desktop-mbf-export-import.md`; their GUI code is DEL-07-12's."

**DEL-17-04, a new acceptance criterion after AC-002** (an unquoted list item, in AC-002's format).

- *New:* "- **AC-003** — The desktop saves a model in the writer's declared subset as an MBF with its identity file through the native chooser after showing the writer's loss report, and imports the pair back bound with the identities Design `desktop-mbf-export-import.md` §1.3 defines. A CI check on an invented fixture exercises C6-1 through C6-7 as applicable and tracks coverage and named refusals for every family the criterion-1 reference system contains, plus its cases and combinations. Criterion 6 is met only when the reference system passes C6-1 through C6-6 through the actual desktop build, with no such family omitted or unsupported in the loss report."

**DEL-07-12, Governing Values** (an ordinary paragraph).

- *Old:* "Finite inventory: native JSON package; .mbf model export; licensed external-run evidence review;"
- *New:* "Finite inventory: native JSON package; .mbf model export and import (DEL-17-04 Design `desktop-mbf-export-import.md`); licensed external-run evidence review;"
