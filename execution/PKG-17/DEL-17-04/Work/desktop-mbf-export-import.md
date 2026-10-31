# Desktop MBF export and import (RT-R7, RT-R8)

**Basis:** `main` at `153485db8c`. The SI writer `core/handoff/caepipe_mbf/si_writer.py`
(#1652, profile `caepipe-mbf-si-pilot-1`), the importer `importer.py`, tranches 1–3 (#1670,
#1685, #1697), the authored cases (#1702), and the import Design `caepipe-mbf-import.md`
(cited here as "the import Design"). Release target `docs/RELEASE_TARGET_v1.0.md`: rows RT-R7
and RT-R8 (Must) and exit criterion 6. File names follow DEL-07-01 `Work/export-file-names.md`
(#1711).

This is a design before implementation, drafted for Agent 0 and independent review. Risk class:
persistence, file writes and numerical meaning. Standard claim fence applies (claims taxonomy
per DEC-081). Nothing here records or implies acceptance, issuance or release, which remain
the owner's acts (F-PIP-3).

## 0. Locality and rulings

This design sits beside the import Design in DEL-17-04, for two reasons:

- the release target names DEL-17-04 as the owner of both rows;
- every decision here rests on the contracts of the writer and the importer.

Two current texts place the desktop part elsewhere:

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

### 1.4 Coverage against the reference system

Criterion 1 lists the reference system's contents. On `main` today:

| Family (criterion 1) | Row | Writer | Importer |
|---|---|---|---|
| Straight pipe; temperature-independent material; section | RT-G1, RT-G3 | written | read |
| Temperature-dependent modulus | RT-G3 | refused (`MBF-SI-TEMPERATURE-DEPENDENT-MATERIAL-UNSUPPORTED`) | refused (O-7 ruling) |
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
   keyboard.
2. **The dialog.** It captures the session's canonical model text and hash. It runs the wasm
   writer with these settings:
   - **Title:** defaults to the project name. The writer trims it to 70 printable ASCII
     characters.
   - **Reference temperature:** 20 °C by default, or 68 °F in a degF project, passed as
     `reference_temperature_f` (exact).
   - **Row limit:** none, passed as `max_layout_rows=None` (Q-4).
3. **Review.** The dialog shows the review of §3.3. An edit to the model changes the hash, and
   the review runs again.
4. **Save….** It is enabled only when the status is `written`. It calls `save_mbf_export`
   (§3.6), which opens the native save chooser. Cancel writes nothing.
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
  the identity file of the same stem is replaced with it. The dialog says so before Save.

### 3.3 Review before writing

- **Profile.** The profile id and the MBF version written. The carried open items
  TBD-17-01-001 to -003 are shown as open.
- **Not written (blocks).** Every `unsupported` entry, with its entity, code and reason,
  grouped by family. While any exists, Save is disabled.
- **Omitted (warns).** Every `omitted` entry, listed in full. Save remains available.
- **Assumptions and conversions.** The writer's `assumptions`, including the normalization
  factors.
- **Warnings.** For example `MBF-SI-SHEAR-MODULUS-NOT-EXPORTED`.
- **Materials.** Each material whose sidecar origin is `undeclared` is named: "importing this
  file asks where these values came from".
- **Identity.** "Identities travel in the identity file beside the MBF. Without it, an import
  forms ids from node numbers and refuses records whose meaning is still open."
- **Boundary.** "SWBPIPE writes the subset listed here. It does not check how another program
  reads or analyses the file."

| Condition | Effect |
|---|---|
| Writer `status: refused` (any `unsupported` entry, an out-of-table unit, a retired model version) | Blocks |
| Route screening blocked, or a protected disposition (§3.5) | Blocks |
| Review stale (model hash or settings changed) | Blocks until the review runs again |
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
  conversion and its conversion witnesses (`exportLengthConversion`, MBF-LENGTH-*) are retired
  with the TypeScript builder (A-6). The dialog only displays the writer's list.
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

`save_mbf_export` takes these arguments: `{canonicalText, claimedModelHash, settings: {title,
referenceTemperature: {value, unit}, maxLayoutRows: null}, suggestedName, reviewed:
{mbfSha256, sidecarSha256}, screening, localFirst}`. In order:

1. **Route evidence.** Check `DOTH-MBF-012`, bound to the SHA-256 of `canonicalText`.
2. **Text.** Apply the 64 MiB cap, canonicalize again and check the hash against the claim,
   as `model_document_save.rs` does.
3. **Document checks.** Run `check_model_document` and the desktop shape check, so an MBF is
   only ever written from a document the desktop opens.
4. **Disposition screen.** As in §3.5.
5. **Writer.** Run the Rust writer. If it refuses, return its diagnostics without opening
   the chooser.
6. **Staleness.** Compute `mbf_sha256` and the sidecar. If either digest differs from
   `reviewed`, refuse with `MBF-EXPORT-REVIEW-STALE`.
7. **Chooser.** Open the native save chooser (`.mbf`, the documents folder). The suggested
   name is checked against `swbpipe-model[-…].mbf`. The path comes only from the chooser.
8. **Writes.** Write the identity file first, then the MBF, each through the extension-neutral
   atomic writer under its own format (`MBF_FORMAT` and `MBF_IDS_FORMAT`).
   - If the MBF write fails, the command removes the identity file it wrote, after an
     identity check.
   - Any pair left behind is self-describing: an identity file whose `mbf_sha256` does not
     match the MBF beside it makes a later import unbound, and the import says so.
9. **Read-back.** Read both files back in fixed blocks and compare their SHA-256 values with
   the written digests.
   - On a mismatch, refuse with `MBF-EXPORT-READBACK-MISMATCH` and remove the files the
     command wrote, after an identity check.
10. **Receipt.** Both file names, byte counts and digests, and the atomic writer's durability
    statement.

The webview's wasm text is used for the review only and is then discarded. The written bytes
are always the native writer's output for the hash-checked text.

### 3.7 Memory

The command follows the DEL-04-07 ingress rules for model-sized interactive commands:

- **Admission.** `command_ingress::admit(…, ArgsDecoding::Moved)` admits envelope + M + 4 KiB,
  held until the command returns.
- **Work within the lease.** `PoolReadMemory` is seeded with the lease. Each of these is
  reserved before it is made:
  - the parse and the canonical copy;
  - the writer's working set, a fixed multiple k·M;
  - the MBF and sidecar bytes;
  - a 64 KiB read-back block;
  - the response.
- **Refusal.** A reservation the pool refuses refuses the save with
  `MBF-EXPORT-MEMORY-UNAVAILABLE`.
- **Measurement.** The slice fixes k from a counting-allocator measurement on its largest test
  model and pins it. The test asserts that the peak stays within what the command held.

### 3.8 Reach-scan re-pin

- **The pin.** Adding `save_mbf_export` (and `open_mbf_file`, §4.7) changes the text of
  `app_invoke_handler`.
  - The PR that adds each command re-pins its SHA-256 in `user_release_reach_scan.rs`
    `MACRO_DEFINITIONS`.
  - It extends the doc comment with the entries, each reviewed: no `mod`, no
    `$parameter ::`, no scanned name added.
- **Unsafe code.** The new `src-tauri` module (`mbf_file.rs`) carries no
  `#[allow(unsafe_code)]` and is not added to `SHELL_UNSAFE_MODULES`.
- **Reach closure.** The engine crate depends on no reporting crate, so the renderer's reach
  closure is unchanged (DEL-08-01 `Work/report-user-release.md` §2.8).

## 4. Import (RT-R8)

### 4.1 Flow

1. **Entry point.** File > Import MBF… (`file.import-mbf`), placed after Open Model Document….
   - It is a `ModelDocumentSource` (`action: "Importing a model batch file"`, `operation:
     "import_mbf"`) passed to `handleOpenModelDocument`.
   - It therefore takes the same busy check, the same unsaved-changes guard (Cancel keeps the
     edits) and the same adoption.
2. **Reading.** `open_mbf_file` (§4.7) opens the native open chooser (`.mbf`). It reads the
   MBF and, when present, `<stem>.swbpipe-ids.json` beside it, and returns both as bytes with
   their SHA-256 values.
3. **Import.** The wasm importer runs with `source_label` set to the chosen file's name.
4. **Review.** The import review (§4.3) is shown before anything is adopted.
   - When `MBF-IMPORT-MATERIAL-PROVENANCE-REQUIRED` is returned, the review asks per material
     (or for all of them at once): "project-invented example data" or "my own source:
     [name]".
   - Vendor-library and unknown origins cannot be declared, so such a material keeps the
     import refused.
   - The importer runs again in wasm with the declaration. The files are not read again.
5. **Adopt.** The document goes through `checkNativeOutcome`: the desktop shape check and the
   document validator through the wasm engine. It then goes through `buildModelIndex`,
   exactly as Open does.
   - It starts unsaved, with no stored base. New Local Project saves it.
   - The first native save checks it again.
6. **Afterwards.** The import report and id map stay viewable in the review until it closes.
   They are not persisted (import Design §16.4).

Running the importer in wasm keeps declaration re-runs local, with no second chooser. Native
code does the file-level reading. This meets REQ-017's "only through the existing document
open path": the open path's webview half is the same validator the native half runs, and
nothing is written before a native save.

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

### 4.3 Import review

- **Status.** Bound or unbound, and why: no identity file; an identity file for other bytes
  (`MBF-IMPORT-SIDECAR-NOT-APPLIED`); or a refused identity file, such as one with a failed
  self-digest (`MBF-IMPORT-SIDECAR-DIGEST-MISMATCH`).
- **Refusals (block).** Each with its code, line and name, and no model values (the report
  carries none). Adopt is disabled.
- **Loss entries (do not block).** By category: `omitted`, `approximated`, `delegated`,
  `unsupported` and `tbd`.
- **Target options.** "Recorded, not applied" (REQ-015).
- **Assumptions.** For example the G derivation, the exact conversions and the Z rotation.
- **Identity.** Restored counts per class and the unrecoverable list. An unbound import says
  "ids formed from node numbers".
- **Units.** The file's unit mode and angles, and the project units the document receives
  (import Design §5.2).
- **Boundary.** "A successful import means the listed subset of this file was read. It is not
  evidence that two programs analyse the model alike."

### 4.4 Entity identity

- **What a bound import restores.** Ids from `node_map`, `element_map` (including bend
  components), `support_map`, `load_case_map`, `material_map`, `load_map` and the
  `pressure_map` regions.
- **What it does not restore:**
  - **Sections.** The writer names PIPE records `P1`, `P2`, … by distinct OD and wall, and
    format 3 has no `section_map`. Section ids are therefore formed again, and two sections
    with equal OD and wall merge. Sections are in the identity set (A-3), so sidecar format 4,
    which adds `section_map` ids, is required for C6-5 (Q-2). Formats 3, 2 and the legacy
    format keep their current handling, and the self-digest is unchanged.
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

### 4.6 Refusals

- **Native, before the importer runs** (`MBF-OPEN-*`): the chosen path or the identity file
  is not a regular file or is a symlink; a file exceeds its cap; a read fails; or the memory
  pool refuses a reservation.
- **Importer.** Every importer refusal is shown with its own code (import Design §4 to §15).
- **Adoption.** The refusals of `checkNativeOutcome` and `buildModelIndex`, as Open shows
  them.
- **Effect.** A refusal leaves the session unchanged.

### 4.7 Native command and memory

`open_mbf_file` takes no model-sized arguments.

- **Admission.** It is admitted within 4 KiB.
- **Reading.** Each file's load is reserved before it is read: the MBF up to 16 MiB and the
  identity file up to `MAX_MODEL_DOCUMENT_BYTES`.
- **Response.** The response (base64 bytes and digests) is reserved and sent through
  `respond_metered`, as `read_project_autosave` does.
- **The webview.** The wasm importer's working set is webview memory. The dialog drops the
  bytes when it closes.
- **The pin.** The command changes `app_invoke_handler` and is re-pinned as in §3.8.

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
- **Native test** (`src-tauri`). In a temporary directory, `save_mbf_export` writes the pair
  and the files read from disk import bound with the same ids (C6-3, C6-7).
- **Desktop test** (Vitest). The export dialog drives a mocked native layer with the wasm
  writer's bytes. Import MBF adopts them, and the session's id sets equal the exported ones.
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
| 4 | Export: `DOTH-MBF-012` (local_private, explicit intent, lossless) in the three catalogues, with parity tests in all three; `save_mbf_export`; the atomic formats; reach-scan re-pin; menu item and dialog; retire the TypeScript builder (A-6); the #1711 row | Rust: admitted under its lease, with the counting-allocator peak within it; named arguments; hash mismatch; stale review; writer refusal opens no chooser; protected disposition refused; Cancel writes nothing; identity file before MBF; an injected failure of the second write removes the first; a read-back mismatch removes both; bytes equal the engine's. Route-catalogue parity; reach-scan pin. Vitest: refusals block Save, omissions warn, degF reference temperature, stale review runs again, suggested names, keyboard reach | Independent review (persistence, file writes) and an independent permissions review of the new governed route |
| 5 | Import: `open_mbf_file`; the import source and review; material declaration; reach-scan re-pin | Rust: caps, symlink and non-regular-file refusals, identity file absent, present and unreadable, lease. Vitest: the guard's Cancel keeps edits; refusals block Adopt; a bound import restores ids; an unbound import states its consequence; the declaration re-run; the adopted document passes `checkNativeOutcome`; unsaved with no stored base | Independent review (session replacement; provenance declarations) |
| 6 | The criterion-6 check of §5 and its CI routing | As in §5 | Focused review |
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
    `canonical_bytes`, `dumps_sorted` and the `to_dict`, bytes and sidecar methods of both
    result types, whose fields a caller may set. A deeper value fails closed as `Unsupported`
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

**Ruled by Agent 0, 2026-10-28.** The table keeps the proposals; where a ruling differs, the
ruling governs.

- **A-1:** the locality is accepted. The §8.3 texts take one independent review on their exact
  Old/New, then Agent 0's confirmation. Slices 4 and 5 wait for them; slice 2 does not.
- **A-2:** accepted. Slices 2 and 3 take numerical review. Their parity bar is the whole
  committed MBF corpus, byte for byte both ways, plus the authored cases. The writer is ported
  after the SIF fix lands.
- **A-3:** this reading is the criterion's own, so it narrows nothing.
  - "Must family" means every family the reference system contains under criterion 1, plus
    the cases and combinations it holds; each traces to its Must row. The RT-row list in the
    table is a cross-check, not the definition.
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
  handling, and the self-digest is unchanged.
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
