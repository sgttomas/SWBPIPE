---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-17-04
package_id: PKG-17
decomposition_basis: execution/_Decomposition/SOFTWARE_DECOMP.md
project_scope_refs: [SOW-030, SOW-074, SOW-075]
package_objective_refs: [OBJ-009, OBJ-017, OBJ-018]
---

# Scope of Work — DEL-17-04 CAEPIPE MBF export profile and deterministic writer

## Purpose and Objective Traceability

This deliverable carries a SWBPIPE model to and from MBF, CAEPIPE's text model batch file. The writer turns a model into an MBF with an identity file and a loss report; the importer reads MBF model data back into a SWBPIPE model document. The result it contributes is that an engineer can iterate between SWBPIPE, where the design is developed, and CAEPIPE, where it is validated, without re-entering the model and without losing track of which entity is which: everything the MBF cannot carry is named, nothing is approximated silently, and a model that goes out and comes back is the same model with the same identities.

It serves SOW-030, SOW-074 and SOW-075 under OBJ-009, OBJ-017 and OBJ-018 (`execution/_Decomposition/SOFTWARE_DECOMP.md`, PKG-17, row DEL-17-04: the first solver-specific target, with bounded model import alongside export), with `docs/PRD.md` §8.4, §10.4, §11.7 (FR-HAND-001 to FR-HAND-005), §16.1, §16.2 and §18.4. With DEL-07-12 it owns exit criterion 6 of `docs/RELEASE_TARGET_v1.0.md`: the desktop saves the reference system as MBF with no family of the reference system omitted or unsupported in the loss report, and imports it back with the same entity identities.

- **OUT-001** — The MBF writer and its profile: the SI writer, its identity file, its loss report and diagnostics, and the export package contract `schemas/caepipe_mbf_export.schema.json`.
- **OUT-002** — The MBF model importer: the inverse of the writer profile, with its declared subset, named refusals, import loss report, identity restoration and data-boundary checks.
- **OUT-003** — The desktop MBF round trip: the engine the desktop runs, the native export and import commands, and the round-trip check of exit criterion 6.

## Deliverable Definition — Ontology

**Terms the product implements.**

- *Writer profile.* The declared set of model members the writer writes, the units it writes them in and the conventions it applies. The one profile is `caepipe-mbf-si-pilot-1`: SI units, the default vertical axis Y.
- *Identity file.* The writer's sidecar, saved beside the MBF as `<stem>.swbpipe-ids.json`. It records the SHA-256 of the MBF it belongs to, the maps from MBF node numbers and record names to SWBPIPE identifiers (nodes, elements, supports, materials, sections, load cases, pressure regions), what the MBF cannot carry and a bound import needs, and its own digest. MBF identity is sidecar-only.
- *Bound and unbound.* An import is bound when an identity file is supplied whose recorded MBF hash equals the hash of the input bytes, and the file equals what the writer regenerates from the imported model. The MBF is then exactly what SWBPIPE wrote, and the writer's conventions describe it. Every other import is unbound, including any MBF that CAEPIPE wrote or rewrote and any file a person edited. An unbound import uses only meanings an admitted public source settles, with the one exception `TBD-17-04-007` names.
- *Declared subset.* The MBF sections, records, key letters, comment keywords, fields and values the importer reads. Each is cited to an admitted source of the DEL-17-01 register, or is a selected interpretation named as one.
- *Refusal and loss entry.* An item outside the declared subset is either refused by name, which blocks the import, or listed in the import loss report, which does not. A refusal has a code, a location and an effect. A record whose meaning rests on an open question is refused in an unbound import with a code naming that question, unless a retained bounded profile decision admits it (`TBD-17-04-007`).
- *Pass-through option.* A piping-code, hanger-design or analysis option of the target. It is recorded as target configuration and is never read by any SWBPIPE rule, check or acceptance logic.
- *Reversible subset.* The models for which writing and then importing bound returns the model. A model is in it when the writer writes it without refusal and without a shear-modulus omission, every compared value is in the writer's unit tables, every value as printed passes the importer's own checks, and distinct material, section and load records stay distinct as printed.
- *Byte-reproducible subset.* The part of the reversible subset that, imported bound and written again with identical writer settings, gives the same MBF byte for byte. A value that re-prints differently is a formatting collision; it is reported, and the model is excluded from byte reproduction by name.
- *Canonical comparison.* The comparison of a model with its import, both reduced to SI base units: node positions; each pipe's ends, diameter, wall, moduli, Poisson's ratio and expansion coefficient; restrained degrees of freedom and spring stiffnesses; each bend's radius, flexibility factor, plane and node; per case, the summed loads per target; and the pressure contract and regions. Sums are formed as exact rationals and rounded once, independently of the writer. Listed representation losses are excluded by name.
- *Round-trip bound.* The largest difference the comparison admits for a scalar. The writer prints ten significant digits, so a printed value p stands for any number within 0.5 × 10^(⌊log10|p|⌋ − 9) of it; the bound is that half-unit, the binary64 rounding of the conversions involved, and for bends the writer's admitted tangent-length, intersection and orientation residuals. A difference beyond its bound is a defect, not a tolerance matter.
- *Must family.* A family the reference system contains under exit criterion 1, or a load case it holds. Combinations, which combine solved results, are outside exit criterion 6's families; their coverage is judged under exit criterion 1.
- *Identity set.* The nine entity classes whose identifiers a round trip must return: nodes, pipes, components, supports, load cases, primitive loads, materials, pressure regions and sections. The project identifier is not in it.
- *Round-trip checks.* Exit criterion 6 as seven checks:
  - C6-1: the engine's writer returns a written MBF for the model under the desktop's export settings;
  - C6-2: no loss entry of category omitted or unsupported names an entity of a Must family;
  - C6-3: the two files on disk are the engine's bytes, their read-back hashes equal to the reviewed ones;
  - C6-4: importing the saved pair is bound, verifies the identity file's own digest and has no refusal;
  - C6-5: for every class of the identity set the imported identifiers equal the exported ones, and none is reported unrecoverable;
  - C6-6: the exported and imported models agree under the canonical comparison within the round-trip bounds;
  - C6-7: C6-1 to C6-6 run through the engine the desktop itself runs.
- *Designated source.* A model document the user names as the one a saved pair was written from. The import review states the identity set as established only when that document's hash is the identity file's recorded source hash, writing it again gives the same pair byte for byte, and the import has the same identifiers in each of the nine classes. This establishes the identity set, not equivalence of the whole model.

**Boundary.** This deliverable owns the writer, the importer, their engine, the native export and import commands, and the round-trip check.

- The export and import dialogs and the Exports panel are DEL-07-12's GUI code on this deliverable's engine and commands.
- Reading CAEPIPE analysis results, running CAEPIPE, and the private exchange folder are DEL-17-05's. The importer reads model input only; ingestion of commercial-tool results stays deferred.
- The package, profile, identity-map and loss-report definitions are DEL-17-02's; the admitted sources and open questions are DEL-17-01's.
- The reference system fixture is DEL-09-01's. The model document and its validator are DEL-02-01's; the document open path is DEL-02-05's, and the guard on replacing a session's unsaved model is DEL-07-11's. The governed save route and the screening of what is saved are DEL-12-02's. The capped worker and the memory budget the desktop runs engine work under are DEL-07-07's and DEL-04-07's.
- A family with no MBF record is a candidate release limitation under exit criterion 3, which only the owner accepts.
- An MBF written by CAEPIPE, even of a SWBPIPE model, is CAEPIPE output. It and everything derived from it stay in the user's private exchange folder.

**Not decided.**

- The first supported target version and record-family subset (REQ-001, REQ-002; DEL-17-01 `TBD-17-01-001` and `TBD-17-01-002`). The profile writes MBF version 15.10 and a declared subset as a bounded pilot selection. That selection governs the pilot only; the two questions stay open until they close under DEL-17-01's closure rule.
- `TBD-17-04-005`: how the gravitational direction field is encoded and what both gravity fields default to. A present value is refused, and the writer does not write the field.
- `TBD-17-04-006`: the text encoding, foot and inch mark characters and numeric precision of CAEPIPE's own MBF export. The importer reads strict UTF-8 and the marks the public pages show, and no bound is widened for export precision.
- `TBD-17-04-007`: whether the layout comment `L=` is the documented load reference and which case a force or moment without `FLC` joins. Bound imports apply the writer's conventions. Unbound imports refuse both, except that a retained bounded profile decision admits both for the MBF version the writer emits, 15.10.
- `TBD-17-04-008`: whether CAEPIPE's MBF export keeps node numbers, row order and bend rows as imported. An unbound identity file is applied only on explicit request.
- `TBD-17-04-009`: whether the absolute-coordinate marker precedes or follows the node number; the public sources disagree. The profile selects the trailing marker and refuses any other placement.
- `TBD-17-04-010`: the unit of the rotational stiffnesses `ROTK` and `KR`, and three further support-record meanings the public pages leave open. These records are refused; a zero `ROTK` is read; the writer writes no rotational spring.
- The solve-equality threshold of the round trip (AC-008) is provisional and is not derived from the printed precision.
- An English-unit writer. An English import cannot be written back by the SI writer; whether iteration in English units is served by a writer extension or by documented normalization to SI is not selected.

## Completion and Reliance Basis — Epistemology

### Requirements

- **REQ-001** — The MBF profile declares its target version basis as undecided until `TBD-17-01-001` closes under DEL-17-01's closure rule.
- **REQ-002** — The first MBF record-family subset stays undecided until `TBD-17-01-002` closes under DEL-17-01's closure rule.
- **REQ-003** — The writer approximates no physical or analytical model entity silently.
- **REQ-004** — Every export has a loss report covering omitted, approximated, delegated, unsupported and undecided behaviour.
- **REQ-005** — Canonical identifiers are not claimed to be carried inside the MBF; they are mapped in the identity file.
- **REQ-006** — Pass-through target options stay target configuration and never become SWBPIPE code-checking logic.
- **REQ-007** — Every public fixture and test MBF is invented and written by the project.
- **REQ-008** — The export package uses sidecar identity mapping while DEL-17-01 `TBD-17-01-003` is open.
- **REQ-009** — The export package blocks when its subset inputs, its identity map or its loss report is missing.
- **REQ-010** — The export package blocks when the target-version basis, the record-subset basis or a carried open question is missing, blank or weakened without an admitted closure.
- **REQ-011** — The export package blocks when its source-basis references omit DEL-17-01, DEL-17-02, `CAEPIPE-IMPORT-MBF` or `CAEPIPE-EXPORT-MBF`, or name DEL-17-03 as a source for the target.
- **REQ-012** — The importer reads MBF model input into a SWBPIPE model document of the current version. It reads no CAEPIPE analysis result, and a successful import is not presented as compatibility, validation or acceptance evidence.
- **REQ-013** — The importer reads only its declared subset. Every other item is refused by name or listed in the import loss report; nothing is dropped, defaulted or inferred.
- **REQ-014** — The importer reads English and SI unit modes and the `RAD` option. It keeps each quantity in its MBF unit where the unit catalog has that unit, and otherwise applies only two exact definitional conversions: feet and inches to inches, and kilogram-force to newton. Any other unit, notation or value is refused by name.
- **REQ-015** — Each import returns diagnostics, an import loss report in the common contract's categories, and the mapping assumptions it applied. Piping-code, hanger-design and analysis options are recorded as pass-through options only.
- **REQ-016** — MBF node numbers and record names map to SWBPIPE identifiers deterministically. A bound identity file restores the identities and splits it preserves; many-to-one mappings and unrecoverable identities are reported; an unbound identity file is applied only on explicit request, every restored identity then reported as unconfirmed; and each import emits its own identity map.
- **REQ-017** — Identical MBF bytes, identity file, material-provenance declaration, rebinding option, source label and importer version give byte-identical outputs. The imported document validates against the model document's desktop profile and enters the product only through the document open path.
- **REQ-018** — A model in the reversible subset, written and imported bound, is reproduced under the canonical comparison within the round-trip bounds. A model in the byte-reproducible subset is written again byte for byte. Every exclusion, representation loss and formatting collision is explicit, and an unbound import of writer output gives its named refusals, except where a retained bounded profile decision admits the records (`TBD-17-04-007`).
- **REQ-019** — MBF files written by CAEPIPE and every import result derived from them are handled as private data, and imported entities carry provenance naming the source MBF's hash. Material mechanics data requires an affirmative declaration that it is project-invented or the user's own supplied source; vendor-library, unknown and suspected-protected origins are refused, and a redistribution disposition alone does not suffice. Allowable, strength, fatigue and code data never enter the model.
- **REQ-020** — A private regression check re-imports the CAEPIPE-exported MBFs in the user's exchange folder and fails when an import outcome leaves its recorded classification. Every CAEPIPE-derived value, difference, classification and detailed diagnostic stays in the exchange folder; standard output carries operational status only; and the check skips, saying why, when no exchange folder is configured.
- **REQ-021** — The export package states no compatibility, validation, code-compliance or professional-acceptance claim, and blocks when model or profile metadata carries wording of one (`docs/PRD.md` §21.2).
- **REQ-022** — The importer is examined on invented, hand-written MBF cases that this deliverable provides, each naming the construct it exercises and the outcome the importer gives on it. The correlation harness (DEL-17-05) also runs them through the user's CAEPIPE, and the private regression of REQ-020 reads what CAEPIPE exported for them.

### Acceptance criteria

- **AC-001** — For each invented model of the writer's test set, the writer either refuses with a named code and location, or returns the MBF, the identity file and a loss report in which every model member and field that holds a value is written, or listed with its entity as omitted or as approximated with the approximation described; nothing is approximated without its entry. The same model and settings give byte-identical output. Every canonical identifier of the model is in the identity file or in a loss entry.
- **AC-002** — Every item of the declared subset is read, and each has a source citation or is named as a selected interpretation. Every named refusal blocks with its code, location and effect, every loss entry and pass-through option is reported, and no option changes any SWBPIPE rule or check outcome. An unbound import of a record that depends on an open question is refused with the code naming that question; the one exception is the pair of records the retained profile decision admits (`TBD-17-04-007`). A CAEPIPE result file is refused.
- **AC-003** — The desktop saves a model in the writer's subset as an MBF with its identity file through the native chooser, after showing the writer's loss report, and imports the pair back bound with the identity set unchanged. On an invented coverage fixture, C6-1 to C6-7 hold as applicable, every Must family is marked covered or carries its named refusal, every combination the reference system holds is likewise marked covered or carries its named refusal, though combinations are outside exit criterion 6's families, and each named refusal still occurs. Exit criterion 6 is met only when the reference system passes C6-1 to C6-6 through the actual desktop build with no Must family omitted or unsupported in the loss report.
- **AC-004** — An export package built on the package contract validates against its schema and uses sidecar identity mapping. It blocks, with a diagnostic naming the cause, on each of: a missing subset input, identity map or loss report; a blank or weakened target-version basis, record-subset basis or carried open question; a source-basis reference set that omits a required source or names DEL-17-03; wording of a compatibility, validation, code-compliance or professional-acceptance claim in model or profile metadata; and unsupported behaviour that has no loss entry, a malformed reference or severity `info`. Unsupported behaviour with an explicit entry is a warning unless the entry marks it blocking.
- **AC-005** — English and SI imports of one invented model agree. Each quantity keeps its MBF unit where the catalog has it; the two exact conversions give the values formed from the units' public definitions; every documented English length form and the `RAD` option are read; every other unit, notation or value is refused by name; and writing an English import through the SI writer is refused with that reason stated.
- **AC-006** — With a bound identity file, an import returns the identities and splits the file preserves and reports each one it cannot recover and each many-to-one mapping. With an unbound file applied on request, every restored identity is marked unconfirmed, and without the request the file is not applied. Every import emits its own identity map.
- **AC-007** — Identical inputs give byte-identical import outputs. Every imported document validates against the model document's desktop profile, and no route admits an imported model into a session except the document open path, with its unsaved-changes guard.
- **AC-008** — Every invented model of the reversible subset, written and imported bound, equals itself under the canonical comparison within the round-trip bounds, the bend boundary case included, and the solves of the model and of its import agree in global displacements and reactions within the solve-equality threshold. Every model listed as byte-reproducible is written again byte for byte. Each exclusion, representation loss and formatting collision is reported by name, and an unbound import of writer output gives its named refusals, except where the retained profile decision admits the records (`TBD-17-04-007`).
- **AC-009** — A material without an affirmative project-invented or user-supplied declaration is refused, as are vendor-library, unknown and suspected-protected origins. No allowable, strength, fatigue or code value is read into the model. Every imported entity names the source MBF's hash. A tracked MBF or import report without an invented-provenance sidecar fails the boundary guard, and the private regression check reads only the configured exchange folder, prints operational status only, and skips with its reason when none is configured.
- **AC-010** — In the desktop, a save writes exactly the bytes whose review the user saw and refuses when they differ; it is made only on its governed local-private route with the user's explicit intent, and the browser-only build declines it. Closing or cancelling before the write leaves no file, and a result that is no longer current is discarded. An import is adopted only when the review the user saw is current and complete and the chosen files are unchanged since it; every refused, stale, changed or cancelled outcome leaves the session as it was, and an adopted model starts unsaved. The writer and the importer run only in the desktop's capped worker within its memory budget: a request priced over the per-invocation memory budget (DEL-04-07, DEL-07-07), or a result over the 64 MiB output ceiling of the worker's writer and importer tasks, is refused by name.
- **AC-011** — Every authored MBF case is invented and written by hand, names the construct it exercises and the importer outcome it records, and importing its file gives that outcome.

## Production and Verification Method — Praxeology

**Method choices that define the product.**

- The target is a narrow profile: what no admitted source confirms is refused or reported, never inferred from the target's name or common practice.
- A bound import may use the writer's conventions; an unbound import may not, except as `TBD-17-04-007` states.
- The identity file is trusted for consistency, not authorship: it is used only when it equals what the writer regenerates from the imported model.
- Every export is checked by its own bound re-import before it is returned, and a refusal of that re-import is an export refusal.
- Sums of loads on one target are exact rationals rounded once, in the writer, the importer and the comparison alike.
- The engine exists as a Python reference and a Rust port that the desktop runs. A criterion is examined on the engine the desktop runs, directly or through byte parity with the reference over committed vectors.
- Public text about the target states only what its public manual documents, cited; anything learned from running it is stated generically or kept private.

**How each criterion is examined.**

- **VER-001** — Executable check (observes the writer's output for each invented test model): the loss report's coverage of every valued member and field, each named refusal, byte equality of repeated writes, and the identity file's coverage of the model's identifiers.
- **VER-002** — Executable check (observes the importer on invented MBF text for every item of the declared subset and every named refusal and loss entry) and judgment by a reviewer who did not write the importer: examines each subset item's citation against the admitted public page and each selected interpretation against its open question.
- **VER-003** — Executable check (observes the desktop's engine, native commands and dialogs on the invented coverage fixture, and the reference system through the built desktop): C6-1 to C6-7, the family table against the refusals that occur, and the identity set before and after.
- **VER-004** — Executable check (observes the export package builder's response to each seeded omission, blank basis, weakened reference set, authority wording in metadata and unsupported-behaviour fault, and the package against its schema).
- **VER-005** — Comparison with a named reference (an evaluation of each unit conversion and length form written independently of the importer, from the units' public definitions) and executable check (observes English and SI imports of one invented model).
- **VER-006** — Executable check (observes imports with a bound identity file, with an unbound file with and without the explicit request, and with none).
- **VER-007** — Executable check (observes repeated imports byte for byte, each imported document against the model document's desktop profile, and the product's sources for any route into a session other than the document open path).
- **VER-008** — Comparison with a named reference (each authored invented model, compared with its own import under the canonical comparison, with bounds derived from the printed precision) and executable check (observes byte reproduction, the reported collisions and exclusions, and the two solves).
- **VER-009** — Executable check (observes the material-provenance refusals, the boundary guard on tracked files, and the private regression check's skip and standard output on an invented exchange folder); the check itself runs where an exchange folder is configured.
- **VER-010** — Executable check (observes the native save and import commands under a changed file, a stale review, a cancellation at each stage, a missing intent and an over-limit request, and the files and session afterwards).
- **VER-011** — Executable check (observes the Rust engine's output against the committed parity vectors of the Python reference, writer and importer, byte for byte).
- **VER-012** — Comparison with a named reference (the outcome each authored case records): imports each authored case's file and compares the outcome; and executable check (observes that each case names its construct and carries its invented-provenance record).

## Governing Values and Decisions — Axiology

- **AX-001** — *Coverage of the reference system.* Better is more Must families written and read back, measured by the count of covered rows in the family table. A family is never counted covered by approximating it, by dropping it unlisted, or by reading an MBF record whose meaning no admitted source settles.
- **AX-002** — *Identity returned.* Better is more of the identity set recovered on a bound round trip and fewer identities reported unrecoverable. An identity is never restored from an unbound file without the user's request and its unconfirmed mark.
- **AX-003** — *Refuse by name.* Where the choice is between refusing and guessing, refuse. Better is a refusal that names the record, the location and what the user can do; breadth is never bought with a default.
- **AX-004** — *Private stays private.* No gain in coverage or convenience justifies committing, quoting or deriving public text from a file the target tool wrote.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-030 SOW-074 OBJ-009 OBJ-017 | REQ-003 REQ-004 REQ-005 REQ-007 | AC-001 | VER-001 VER-011 | `docs/PRD.md` FR-HAND-004 and the common contract; the comparison is against the invented model's own members and identifiers. Parity with the Python reference is the product's own earlier output and guards against change only |
| OUT-002 | SOW-030 SOW-075 OBJ-009 OBJ-017 OBJ-018 | REQ-006 REQ-012 REQ-013 REQ-015 | AC-002 | VER-002 VER-011 | Outside source: the vendor's public manual pages admitted in the DEL-17-01 register, read by a reviewer who did not write the importer; the invented MBF text states what must be read and refused |
| OUT-003 | SOW-030 SOW-074 OBJ-009 OBJ-017 | REQ-003 REQ-004 REQ-016 REQ-018 | AC-003 | VER-003 | Exit criterion 6 and its family list; the expected identities and model are the fixture's and the reference system's own, which the engine did not write |
| OUT-001 | SOW-074 SOW-075 OBJ-017 OBJ-018 | REQ-001 REQ-002 REQ-008 REQ-009 REQ-010 REQ-011 REQ-021 | AC-004 | VER-004 | The DEL-17-01 register is the reference for the required sources and open questions, and `docs/PRD.md` §21.2 names the claims no package may state, found by the DEL-15-04 screen; the seeded faults state what must block |
| OUT-002 | SOW-030 OBJ-009 | REQ-014 | AC-005 | VER-005 | Outside source: the public definitions of the units (the international inch and foot, standard gravity for kilogram-force), evaluated without importer code |
| OUT-002 | SOW-074 OBJ-017 | REQ-016 | AC-006 | VER-006 | The identities compared are those of the invented model that was written, which the importer did not write |
| OUT-002 | SOW-030 OBJ-009 | REQ-017 | AC-007 | VER-007 | Outside source: the model document schema, DEL-02-01's; byte equality of repeated imports guards against change only |
| OUT-002 | SOW-030 SOW-074 OBJ-009 OBJ-017 | REQ-018 | AC-008 | VER-008 | Outside source: the authored invented model is the expected result, and each bound is a closed form of the ten-digit printed precision and binary64 rounding. The solve-equality threshold is provisional. Agreement of the two solves shows the models are solved alike, not that either result is correct |
| OUT-002 | SOW-075 OBJ-018 | REQ-007 REQ-019 REQ-020 | AC-009 | VER-009 | `docs/IP_AND_DATA_BOUNDARY.md` (interoperate openly, evaluate privately) and `docs/PRD.md` §20.2; the invented exchange folder states what must stay private |
| OUT-003 | SOW-030 SOW-075 OBJ-009 OBJ-018 | REQ-017 REQ-019 | AC-010 | VER-010 | `docs/PRD.md` §5.3 and §20.3 and the export controls of DEL-12-02; the seeded changes and cancellations state what must be refused or left unchanged |
| OUT-002 | SOW-030 SOW-075 OBJ-009 OBJ-017 | REQ-007 REQ-013 REQ-022 | AC-011 | VER-012 | The recorded outcomes are the importer's own earlier output and guard against change only; what each construct means rests on the citations of AC-002 |
