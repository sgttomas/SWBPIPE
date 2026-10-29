# CAEPIPE MBF model import (DEL-17-04)

**Basis:** branch `claude/piping-caepipe-pilot-20261026` at `71272e769c` (PR #1652, current
`main` merged), which adds the SI writer `core/handoff/caepipe_mbf/si_writer.py`. Model
document 0.7.0 (DEL-02-01 `Design/model-document-0.7.0.md`); 0.8.0 as proposed in open
PR #1649 (head `4291a7ace8`). Design before implementation; draft for independent review.
Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081). Nothing here records,
implies or derives acceptance, issuance or lifecycle standing (F-PIP-3).

## 0. Rulings this design applies

- **Owner, 2026-10-26 (scope):**
  <!-- owner-words -->"I want to expand to go both ways. That would make any kind of
  iteration between the two much more effective. For the validation it isn't strictly
  necessary, since the proof case lives in CAEPIPE."<!-- /owner-words -->
- **Agent 0's framing of that ruling:** model import only, CAEPIPE MBF → SWBPIPE model
  document. It is not the commercial result ingestion that DEC-016 defers. Public importer
  tests use MBF files the project writes itself. MBF files written by CAEPIPE, even of
  SWBPIPE's own models, are CAEPIPE output. They stay private, as regression data in the
  exchange folder.
- **Owner, 2026-10-26 (units):** <!-- owner-words -->"units conversions that are
  routinely done in engineering applications are permissible as a
  solution."<!-- /owner-words --> The ruling authorizes routine conversions.
  It is this Design's choice, not an owner limit, to use only the two exact definitional
  conversions the tranche needs (§5.3) and to refuse any other unit by name.
- **Owner, 2026-10-26 (judgment):** Agent 0, as HELP_HUMAN, resolves matters of precision
  or detail within normative engineering practice. This Design records each such resolution
  with its reason: §5.3 (exact conversion factors), §5.4 (rational offset arithmetic) and
  §18.2 (round-trip tolerances derived from the writer's number formatting and its
  admitted bend residuals).
- **Base rule for the writer's load conventions (TBD-17-04-007).** Bound imports apply
  the writer's conventions. Unbound imports refuse `L=` and FLC-less FOR/MOM with
  `MBF-IMPORT-TBD-17-04-007` pending documentation, except where an explicitly retained
  bounded profile decision admits them. A delimited extension at the end of this Design
  may carry such a decision. Removing that extension leaves every other part of this
  Design unchanged.
- **Owner, 2026-10-26 (public text):** <!-- owner-words -->"Don't state facts about
  CAEPIPE in public text. Have a machine redaction of any mentions and replace with
  something generic, maybe?"<!-- /owner-words --> The owner chose the scope
  <!-- owner-words -->"New facts only"<!-- /owner-words -->.
  - **Agent 0's application.** Anything learned from running the external prover (the
    target tool), from its outputs or from its screens is stated generically in public
    text, with no product name, vendor or version attached to the fact. The named detail
    lives in private notes, and public text points only to "private notes".
  - **What may still be cited.** Facts documented by the public manuals are cited with
    attribution. Existing names, paths and source IDs are not renamed.
- **Wording convention.** Each statement about the MBF format or the target tool is
  labelled as one of:
  - documented, citing an admitted public source;
  - a selected interpretation, which is the profile's choice and not a documented target
    fact: §10.2's marker placement, and any explicitly retained bounded profile decision;
  - a generic statement about the target tool's behaviour, per private notes.
- **Agent 0, 2026-10-27 (tranche 2), under the owner's delegation:** "the importer's next
  tranche: pressure and Z-vertical first". Implemented in §9.4 (pressure) and §5.1 and
  §10.5 (Z vertical), each with its writer counterpart.
- **Agent 0, 2026-10-27 (tranche 3), under the owner's delegation:** §20.3 items 3 to 6, mapped onto
  existing SWBPIPE members only; an element or option with no member is refused by name, and each gap
  that matters for parity is proposed in §20.3 for Agent 0. Item 6 waits for the ruling question of §20.2
  (O-7). Implemented in §12 (guides, limit stops, springs and flexible anchors; hangers stay refused).
- **Data rule (owner):** CAEPIPE's outputs and anything derived from them are private.
  Manual text, tables and examples are not copied. There is no reverse engineering and no
  claim of compatibility with CAEPIPE.

## 1. Purpose and boundary

The importer reads an MBF (CAEPIPE's text model batch file) and produces a SWBPIPE model
document of the current version, an import report and an id map. It is the inverse of the
SI writer's profile `caepipe-mbf-si-pilot-1`. It also reads a small, explicitly listed
extension that SWBPIPE can represent exactly (§14), so that a model edited in CAEPIPE and
exported again can come back.

What it is not:

- **Not a result parser.** It reads model input sections only. CAEPIPE result files and
  CSV output stay with DEL-17-05's bounded parser. DEC-016's deferral of commercial result
  ingestion is unchanged.
- **Not a coverage or compatibility claim.** A successful import means the declared subset
  of this file was read faithfully. It is not evidence that CAEPIPE and SWBPIPE analyse the
  model alike.
- **Not a guesser.** Every section, record, key letter, comment keyword, field or value
  outside the declared subset is either refused by name, which blocks the import, or listed
  in the import loss report, which does not block it. Nothing is silently dropped,
  defaulted or inferred. §14 gives the rule that decides between refusal and loss entry.
- **Not a CAEPIPE-semantics oracle.** Where the public manual leaves a meaning open, the
  importer refuses and the question goes to the DEL-17-01 dossier (§20).

**Bound and unbound imports.** An import is **bound** when a writer sidecar is supplied
and its `mbf_sha256` equals the SHA-256 of the input bytes. The file is then exactly what
SWBPIPE's writer produced, and the writer's own conventions describe its meaning. Every
other import is **unbound**, including any MBF that CAEPIPE has written or rewritten, and
any file a person has edited. Two rules follow:

- **Bound imports.** The importer may apply the writer's conventions.
- **Unbound imports.** The importer uses only meanings that admitted public evidence
  settles, plus §10.2's selected marker placement. A record whose meaning rests on an
  open TBD is refused with a diagnostic that names that TBD (`MBF-IMPORT-TBD-17-04-00n`).
  It stays refused until admitted evidence or an explicit, bounded profile decision
  resolves it.
  - In particular, `L=` and FLC-less FOR/MOM are refused (§0, TBD-17-04-007), except
    where an explicitly retained bounded profile decision admits them.

## 2. Sources

Primary sources are the live public manual pages at `docs.sstcae.com`. Each is cited by
page and section. Their text, tables and examples are not reproduced.

| Source ID (DEL-17-01) | Page | Used for |
|---|---|---|
| `CAEPIPE-IMPORT-MBF` | `users_manual/import_mbf.htm`, section *Format of .MBF* (keyword list; OPTIONS, OPTEMP, OPCODE, OPPRES, OPMISC, MATERIAL, PIPE, LOADS, LAYOUT key letters and comment list) | Section order and keywords, record fields and units per unit mode, layout grammar, comment keywords |
| `CAEPIPE-EXPORT-MBF` | `users_manual/export_mbf.htm` | CAEPIPE exports current-version MBF and a 6.xx variant |
| `CAEPIPE-MBF-EXAMPLE` | `users_manual/example_mbf_format.htm` | Record syntax only, such as how a load name is referenced from a layout comment. Never copied into fixtures. |
| `CAEPIPE-LAYOUT-OFFSETS` | `users_manual/dx__dy__dz.htm` (*DX, DY, DZ*) | From-row coordinates vs offsets, the origin rule for the first From node, the absolute-coordinate node marker (described as following the node number), feet-inch input, and the default input precision for lengths |
| `CAEPIPE-NODE` | `users_manual/node.htm` (*Node*) | Generated node suffixes (A/B bend ends; C/D/J jacket) and the absolute-coordinate marker (described as a prefix; see §10.2) |
| `CAEPIPE-MBF-ABSOLUTE` | `users_manual/mbf_generation_in_excel_with_absolute_coordinates.htm` | That a node marked with `*` carries absolute coordinates |
| `CAEPIPE-UNITS` | `users_manual/units__ctrl_u_.htm` (*Units*) | Weight (not mass) inputs and weight densities |
| `CAEPIPE-LOAD-CASES` | `users_manual/load_cases.htm` (*Load cases*) | Case-membership semantics only: which loads each CAEPIPE case includes. Used to state the writer's case mapping and its inverse |
| `CAEPIPE-BEND` | `tech_manual/bend.htm` (technical reference, *Bend*) | CAEPIPE generates the near and far end nodes of a bend's curved portion, named with A and B suffixes on the bend node. The section given on the bend row applies to the curved portion and to the straight from the preceding node to the A node |
| `CAEPIPE-FORCE` | `tech_manual/force.htm` (technical reference, *Force*) | The Force technical reference documents that sustained-category forces and moments also participate in operating cases. It does not state which category an MBF force or moment without `FLC` takes |
| `CAEPIPE-ANALYSIS` | `users_manual/analysis.htm` (*Analysis*: *Number of Thermal Loads*; *Gravitational Acceleration and Direction*) | The number of thermal loads is the count of operating temperatures (and their pressures) in use; with `CAEPIPE-LOAD-CASES`, multiple operating and expansion cases exist only when it is above 1 (§9.1). From 15.10 a gravitational acceleration and direction scale the analysis, with all input given for earth gravity (§6) |
| `CAEPIPE-LOCAL-AXES` | `tech_manual/local_coordinate_system.htm` (technical reference, *Local Coordinate System*) | The pipe element local-axis convention: with global Y vertical, a non-vertical straight's local y lies in the plane of its local x and global Y, toward +Y; a vertical straight's local y is global −X (§10.5) |
| `CAEPIPE-TECH-REF` | `tech_manual/index.htm` | The technical reference's index; the pages used are registered separately above |
| `CAEPIPE-GUIDE` | `tech_manual/guide.htm` (technical reference, *Guide*) | A guide restrains the translations in the local y and z of the element it is attached to, its local x taken from the preceding element, else the following one; rotations are free; the default stiffness is rigid; a gap is symmetric about the guide axis; a friction coefficient makes the analysis nonlinear (§12) |
| `CAEPIPE-LIMIT-STOP` | `tech_manual/limit_stop.htm` (technical reference, *Limit Stop*) | A limit stop acts along one direction given by global components; a zero limit stops the node at its undeflected position, a blank limit leaves that side free, a non-zero limit is a gap; the default stiffness is rigid; limit stops need a nonlinear solution (§12) |
| `CAEPIPE-ANCHOR` | `tech_manual/anchor.htm` (technical reference, *Anchor*) | An anchor is rigid in all six DOFs by default; a flexible anchor has a stiffness per DOF, and a zero stiffness leaves that DOF free; displacements at a flexible anchor act through its stiffness (§12) |
| `CAEPIPE-SKEWED-RESTRAINT` | `tech_manual/skewed_restraint.htm` (technical reference, *Skewed Restraint*) | A two-way support along, or about, a direction given by global components, with a stiffness the user supplies (§12) |
| `CAEPIPE-HANGER` | `tech_manual/hanger.htm` (technical reference, *Hanger*) | Hangers act vertically; the number of hangers multiplies stiffness and load; hanger loads (hot or cold, by option) act as forces in the sustained and operating cases, and spring rates enter the stiffness only with an option (§12) |
| `CAEPIPE-MATERIAL` | `tech_manual/material.htm` (technical reference, *Material*) | The tabulated expansion coefficient is the mean coefficient, not the instantaneous one; element temperatures must lie within the table's range (§20.2, O-7) |

The seven rows from `CAEPIPE-LAYOUT-OFFSETS` to `CAEPIPE-FORCE` are registered in DEL-17-01
`Source_Basis_Register.md`. Each has the live URL, an archived fallback where the Internet
Archive holds one (it holds none for `CAEPIPE-MBF-ABSOLUTE`), and a boundary:

- "record syntax only" for the four user's-manual layout pages;
- "case-membership semantics only" for `CAEPIPE-LOAD-CASES` and `CAEPIPE-FORCE`;
- "bend end-node naming and section assignment only" for `CAEPIPE-BEND`.

`CAEPIPE-LOCAL-AXES` was registered later with the boundary "pipe local-axis convention
only", for the straight-pipe orientation of §10.5. The six tranche-3 rows from `CAEPIPE-GUIDE` to
`CAEPIPE-MATERIAL` are registered with the boundary "support-type semantics only" (the four support pages
and *Hanger*) or "material-property semantics only" (*Material*). `CAEPIPE-ANALYSIS` also gives the
modulus option of §20.2 (O-7), within its existing boundary.

## 3. Where it lives and its interface

- **Module:** `core/handoff/caepipe_mbf/importer.py`, beside the writer and sharing its
  constants (`MBF_VERSION`, `LOAD_CASE_LABELS`, `STANDARD_GRAVITY`). The writer keeps its two
  case labels in one mapping, `LOAD_CASE_LABELS["sustained"]` and
  `LOAD_CASE_LABELS["expansion"]`; this Design's "sustained label" and "expansion label"
  name those entries.
- **Function:**
  `import_caepipe_mbf(data: bytes, *, sidecar: Mapping | None = None, material_provenance: Mapping | None = None, pressure_closure: Mapping | None = None, rebind_by_node_number: bool = False, source_label: str) -> MbfImportResult`.
  - Input is bytes, not text, so that the importer owns the decoding (§4.1).
  - `material_provenance` is the affirmative material provenance declaration (§7.4).
  - `pressure_closure` is the closure-transfer declaration for pressure-region terminals
    (§9.4).
  - `rebind_by_node_number` is the explicit request to apply a sidecar by CAEPIPE name,
    whatever its digest, every restored identity unconfirmed (§15).
- **Result:** `status` (`imported` | `refused`), `model` (the document, or `None` when
  refused), `diagnostics`, `loss_report`, `assumptions`, `target_options` (§6), `id_map`
  (§15) and `source` (`mbf_sha256`, MBF version, unit mode, angles, vertical axis, line
  count).
- **Report document:** `to_dict()` emits `document_kind:
  "swbpipe.caepipe_mbf_import_report"` with `profile_id: "caepipe-mbf-import-1"`. The
  report carries no model values from refused or omitted records, only their location and
  name.
- **Command line:** `python3 -m core.handoff.caepipe_mbf.importer <in.mbf> --out <dir>
  [--sidecar <id_map.json>] [--material-provenance <declaration.json>]
  [--pressure-closure <declaration.json>] [--shear-model <model>] [--rebind-by-node-number]` writes `model.json`, `import_report.json` and `id_map.json`.
  It refuses an output folder inside a Git work tree unless the input MBF is a project-owned
  invented fixture (§16.4).
- **No product write path.** The importer writes a model document only. The document
  enters the product through the existing open path: the desktop shape check, the version
  gate and schema validation. It never writes into a project store. This is DEL-10-01's
  no-bypass boundary. A desktop "Import MBF" action belongs to DEL-07-12 and is not part
  of this tranche.

## 4. Reading the file

### 4.1 Bytes, lines and encoding

- The bytes are decoded as strict UTF-8. ASCII, which the writer emits, is a subset of it.
  Any other encoding, including a single non-UTF-8 byte, is refused:
  `MBF-IMPORT-ENCODING-UNSUPPORTED`, naming the byte offset.
- The manual's own examples write foot and inch marks both as ASCII `'` `"` and as the
  typographic `’` `”` (U+2019, U+201D). Both are accepted as foot and inch marks, and only
  in lengths. Any other non-ASCII character outside free text (headings, descriptions,
  model comments) is refused (`MBF-IMPORT-CHARACTER-UNSUPPORTED`). A leading byte-order mark
  is refused as `MBF-IMPORT-ENCODING-UNSUPPORTED`. Which encoding and marks CAEPIPE's MBF
  export writes is TBD-17-04-006.
- Line ends: CRLF or LF. A lone CR is refused (`MBF-IMPORT-LINE-END-UNSUPPORTED`). Trailing spaces are trimmed. Spaces around
  commas are not significant: the manual's examples show both forms.
- A blank line is ignored, except right after a section keyword where the manual gives the
  blank line a meaning (OPPRES, OPMISC). There it is recorded as "section present with
  CAEPIPE defaults" (§6).

### 4.2 Heading and sections

- Every line before the first section keyword is a heading line. The first becomes
  `project.name`. The rest go to `project.notes`, each prefixed with its line number.
- **Keyword recognition.** A section starts at a line that is exactly one documented
  section keyword: uppercase, no comma, nothing else on the line. The manual says only the
  first three characters are significant, so a three-character prefix is also accepted,
  but only when it names exactly one documented keyword. `OPT` names both OPTIONS and
  OPTEMP, so the bare prefix is refused (`MBF-IMPORT-SECTION-AMBIGUOUS`); the full words are
  accepted. An unknown single-token line inside a section is a data line of that section
  and must parse as one. A keyword-like line (an uppercase token of three or more letters
  and digits) that is not a record of its section is refused as
  `MBF-IMPORT-SECTION-UNKNOWN`.
- MATERIAL, PIPE and LAYOUT are required (`MBF-IMPORT-SECTION-MISSING`).
- `VERSION` is required. A file without it is the 6.xx variant and is refused by name
  (`MBF-IMPORT-VERSION-MISSING`). The accepted versions form an explicit list, initially
  `15.10`, the writer's version. Any other version is refused by name
  (`MBF-IMPORT-VERSION-UNSUPPORTED`), because the manual makes field layouts depend on
  version (TBD-17-01-001). Adding a version is a reviewed edit with its own tests.
- A section that appears twice is refused (`MBF-IMPORT-SECTION-REPEATED`).

### 4.3 Fields and numbers

- Fields are comma-separated, except that commas inside parentheses belong to the field,
  as in `FIXD(1,0,1)`.
- A real is an optional sign, digits with an optional decimal point, and an optional
  exponent. Anything else in a numeric field is `MBF-IMPORT-NUMBER-INVALID`, naming line and
  field. An empty optional field means "not given". An empty required field is refused
  (`MBF-IMPORT-FIELD-MISSING`). A MATERIAL, PIPE or LOADS name defined twice is refused
  (`MBF-IMPORT-RECORD-REPEATED`).
- A record with more fields than its documented field list, for the declared MBF version,
  is refused by name (`MBF-IMPORT-FIELD-UNDOCUMENTED`, naming the record and field
  position). Trailing fields with no public description are not given a meaning.
- Non-finite values are refused. Range checks match the writer's: E > 0; 0 < ν < 0.5;
  0 < t < OD/2; R > 0; k > 0; non-negative material weight densities; negative values are refused. They are applied before any
  derived value is formed, so an out-of-range input is refused by name, never clamped
  (`MBF-IMPORT-VALUE-OUT-OF-RANGE`). 0 < t < OD/2 is checked on the decimals and again, by
  the same code, on the binary64 values read to metres as the build compares them
  (`derive_pipe_section`, `validate_wall`, `SourceAnnulus::from_od_wall`): OD 504 mm with
  wall 251.99999999999997 mm passes in mm but reads as 2t = OD in m, and a wall of
  0.49999999999999999 reads as 0.5 (independent review VER-002 r6).
- **Binary64 range.** A value whose binary64 reading overflows, or a non-zero value that
  reads as 0, is refused (`MBF-IMPORT-VALUE-UNREPRESENTABLE`); it never imports as zero or
  infinity. Every derived value is rounded through the same guard, which refuses by the same
  code: E in Pa, G, node coordinates, the temperature change, summed loads, tangent lengths
  and nodes, and the weight per length of a non-zero weight density. Norms and unit
  directions use the scaled `hypot`, so neither tiny nor huge lengths overflow or underflow
  in their squares (independent review VER-002 r1).
- **One rounding per derived coordinate** (independent review VER-002 r3). Node positions
  (offsets summed along a run, absolute coordinates and feet-inch fractions) are exact
  rationals until they are rounded. Each tangent-node coordinate, TI ∓ t·d, is formed exactly
  from its binary64 operands and rounded once, so a non-zero coordinate below the binary64
  range is refused rather than rounded to 0 by a product. The bend-plane direction is formed
  from the exact TI and chord middle, scaled exactly by a power of two, then normalized; its
  components, being a direction, are rounded relative to its length. E in Pa stays the
  binary64 product the units catalog forms (§7.2); its factor exceeds 1, so it cannot
  underflow.

### 4.4 Layout rows and comments

A layout data line is a row, a continuation line or a model comment.

- **Model comment.** A line starting with `*` is a model comment. It goes to
  `project.notes` with its line number.
- **Row.** A row starts with its row key and node, `F<n>`, `T<n>` or `L<n>`, optionally
  followed by the absolute-coordinate marker `*` (§10.2). Key-letter fields follow in any
  order: `K`, `M`, `P`, `J`, `X`, `Y`, `Z`, `B`, `H`. A key letter may appear at most once
  per row (`MBF-IMPORT-ROW-KEY-REPEATED`).
- **Comments.** The first field that begins with `C` starts the comments. That one `C` is
  removed and the rest of the field is the first comment. For example `CL=S1` is the
  comment `L=S1`, and `CWGT=…` is the comment `WGT=…`, not `CWGT`. Every later field is a
  comment.
- **Continuation.** A row whose last comment ends with a comma continues on the next line,
  which carries comments only. Its fields are comments as written: no leading `C` is removed
  from a continuation line. A row that ends with a comma before any comment, a continuation
  with no following line, an empty key-letter field, unbalanced parentheses, or a To row
  without a preceding From row is refused (`MBF-IMPORT-ROW-SYNTAX-INVALID`). A layout with
  no element is refused (`MBF-IMPORT-TOPOLOGY-EMPTY`).
- **Comment form.** Each comment is `NAME`, `NAME=value` or `NAME(v1,…)`. The name is
  matched exactly against the documented comment list, and the longest documented name
  wins. For example `FIXDX` is not read as `F` followed by `IXDX`. An undocumented name is
  `MBF-IMPORT-COMMENT-UNKNOWN`.
- **Empty values.** A documented comment with an empty value, such as `TAG=` or `LTAG=`,
  carries no data. It is counted in the report summary and has no further effect.

## 5. Units

### 5.1 Modes

- `OPTIONS` without `SI` means English units, and `SI` means SI units. `RAD` switches
  angles from degrees to radians. This follows *Format of .MBF*, OPTIONS: Units.
- **Vertical axis Z** (documented: *Format of .MBF*, OPTIONS, `Z`: the vertical axis is Z;
  the default is Y). SWBPIPE has one vertical convention, global Y, with weight along −Y
  (§7.3). A Z-vertical file is turned into it by the exact rotation
  (X, Y, Z) → (X, Z, −Y), the rotation by −90° about X that takes the file's +Z to +y.
  - It permutes axes and changes one sign, so it adds no rounding. It applies, before any
    arithmetic, to every printed coordinate and offset and to every force, moment,
    displacement and rotation vector with its sign (a proper rotation maps moment and
    rotation vectors as it maps forces). Restraint flags (`FIXD`, `FIXR`) move with their
    axes: file Y → z, file Z → y.
  - Positions are then summed exactly and rounded once (§10.2), and bends (§11), weight
    (§7.3) and orientation (§10.5) are formed in SWBPIPE's axes. Messages name the file's
    axes. `source.vertical_axis` records `Z`, and `assumptions` state the rotation.
  - **Gravity.** Weight acts along the declared vertical: −Z in the file, −y after the
    rotation. OPMISC's gravity direction is read accordingly (§6).
- **The writer always writes Y vertical.** A SWBPIPE model document records no vertical
  axis: weight is an explicit primitive along −Y, and the writer accepts weight only along
  −Y. Writing `Z` would need a vertical-axis member the document does not have, or a guess
  from the loads. A Z-vertical import therefore re-exports Y-vertical with the rotated
  coordinates: the same model in the target's default convention. The writer's
  assumptions say so, and the importer's assumptions say it for a Z-vertical import.
- Each record's field units come from the manual's per-mode record descriptions. They are
  not uniform: in English mode an offset is in feet (feet-inch notation) but a bend radius
  is in inches, and in SI mode pressure is in bar.

### 5.2 Units are kept, not converted

SWBPIPE quantities carry their own unit, and the units catalog (`core/units`) has every
SI and US-customary unit the tranche-1 records use. The importer therefore writes each
quantity in its MBF unit and leaves normalization to the existing readers, as the desktop
does for entered units (DEL-02-02 `Design/project-unit-selection.md` §2, items 6 and 8).
The project units follow the mode.

**English imports and re-export (O-2).** Agent 0 decided this on 2026-10-26, under the
owner's engineering-judgment and units delegation ("units conversions that are routinely
done in engineering applications are permissible as a solution"):
- The first tranche re-exports an English-unit import through the SI writer by documented,
  exact normalization to the SI profile, using the definitional factors 25.4 mm/in,
  0.45359237 kg/lb, the standard-gravity kgf and lbf definitions, and °F intervals × 5/9.
- The import result states that normalization was applied, with its factors.
- A native English writer mode is an owner choice, carried in the survey proposal.
- **As implemented.** `si_writer.py` accepts the project units `in`, `lbf` and `degF` and the
  quantity units `in`, `lbf`, `psi`, `lbf*in`, `lbf*ft`, `lbf/in`, `degF` (interval) and
  `1/degF`. Each is multiplied by its exact rational factor and rounded once to binary64
  (a result that overflows, or a non-zero result that rounds to 0, is refused as
  `MBF-SI-VALUE-OUT-OF-RANGE`):
  1 in = 25.4 mm; 1 lbf = 0.45359237 kg × 9.80665 m/s² = 4.4482216152605 N; psi, lbf·in,
  lbf·ft and lbf/in from those two; °F intervals × 5/9; 1/°F × 9/5. An absolute reference
  temperature in °F, passed as `reference_temperature_f`, becomes (T − 32) × 5/9 °C. The
  writer's result carries the info diagnostic `MBF-SI-UNITS-NORMALIZED`, listing each
  factor applied, and its assumptions say so. Every other unit stays refused. The importer
  itself still converts only as §5.3 says; its English result states that the writer
  re-exports it by this normalization.

| Quantity | English MBF | Written as | SI MBF | Written as |
|---|---|---|---|---|
| Node position, offset | feet-inch notation (§5.4) | `in` (`project.units.length`) | mm | `mm` |
| OD, wall thickness, bend radius | in | `in` | mm | `mm` |
| Elastic modulus | psi | `psi` | N/mm² | `MPa` (the same unit) |
| Expansion coefficient | in/in/°F | `1/degF` | mm/mm/°C | `1/degC` |
| Temperature (to ΔT) | °F | ΔT in `degF` (interval) | °C | ΔT in `degC` (interval) |
| Weight density (to weight per length, §7.3) | lbf/in³ | `lbf/in` | kgf/m³ | `N/m` |
| Force | lbf | `lbf` | N | `N` |
| Moment | ft·lbf | `lbf*ft` | N·m | `N*m` |
| Imposed displacement | in | `in` | mm | `mm` |
| Imposed rotation | deg, or rad with `RAD` | `deg` / `rad` | deg / rad | `deg` / `rad` |

- **Project units.** English mode writes `in, lbf, lbf*in, psi, psi, degF` and `deg` (or
  `rad`); that is the US-customary preset. SI mode writes `mm, N, N*m, MPa, Pa, degC` and
  `deg` (or `rad`), keyed in the desktop's order length, force, moment, pressure, stress,
  temperature, angle: pressure `MPa` and stress `Pa`, as written above.
- **Moments.** The project moment key follows the preset. Each moment quantity carries its
  own `lbf*ft`.
- **Consequence for iteration.** An English-mode import opens and solves in SWBPIPE, and
  re-exports through the SI writer by the normalization above (O-2). The re-exported MBF is
  SI; a native English writer mode remains an owner choice (§20).

### 5.3 The two conversions the importer does make (HELP_HUMAN, routine and exact)

- **Feet and inches to inches**, by 1 ft = 12 in. This is the notation of §5.4.
- **Kilogram-force to newton**, by 1 kgf = 9.80665 N. This is the definition of kgf, and
  the same constant as the writer's `STANDARD_GRAVITY`. The catalog has no `kgf`, so SI
  weight densities are written as N/m weight per length (§7.3).

Both are exact by definition. The arithmetic rounding they add is at most one binary64
rounding per operation. The owner's units ruling would also permit other routine
conversions. This Design makes no others because the tranche does not need them. A unit,
notation or field the importer cannot carry, or convert by these two definitions, is
refused by name (`MBF-IMPORT-UNIT-UNSUPPORTED`).

### 5.4 Length notation (English mode)

The manual documents several English length forms (*Format of .MBF*, LAYOUT, X, Y and Z;
*DX, DY, DZ*). The importer accepts exactly this grammar. The examples below are invented
values.

```text
length   := [sign] ( feet_only | feet SEP inches [ "-" fraction ] [INCH] )
feet_only:= decimal                        e.g. 7, -2, 4.25 (decimal feet)
feet     := digits
SEP      := "'" | "’" | "-"                e.g. 7'3, 7-3
inches   := digits                         0..11
fraction := digits "/" digits              numerator < denominator, denominator > 0
INCH     := '"' | "”"
                                           e.g. 7'3-1/4", 7-3-1/4, 0'0-5/16", -2'0
```

- The sign applies to the whole length, so `-1'6` is −18 in.
- A bare number is feet, `4.25` being 4 ft 3 in. That is the manual's convention, not a
  guess.
- **Refused by name** (`MBF-IMPORT-LENGTH-NOTATION-INVALID`): inches of 12 or more,
  decimal inches next to feet, decimal feet with an inch part, a fraction without whole
  inches, a second sign, a missing inch part after a separator, and anything else the
  manual does not document.
- **Arithmetic.** The importer evaluates every length as an exact rational in inches
  (Python `fractions.Fraction`). It sums offsets along a run in rationals and converts each
  node position to binary64 once (HELP_HUMAN: this removes accumulated rounding, so a node
  position is the correctly rounded value of the decimal or fractional input).
- **SI mode.** SI lengths are plain decimals in mm and are summed the same way.
- **Bend radius.** `B` in English mode is inches, a plain decimal. A feet-inch form there
  is refused.
- **Precision.** The manual's *DX, DY, DZ* page gives a default input precision of four
  decimals for lengths. The precision CAEPIPE's MBF export writes is TBD-17-04-006. The importer reads
  whatever precision the file has and makes no rounding of its own.

## 6. Options and analysis settings

These records configure CAEPIPE's own analysis and code checks. They have no meaning in
the SWBPIPE model. The importer copies them into `target_options` in the report, as
pass-through target configuration (DEL-17-04-REQ-006). They never become SWBPIPE rule,
code-checking or acceptance logic, and their presence is listed in the loss report as
`delegated`.

| Record | Treatment |
|---|---|
| OPTIONS hanger-design method and piping code | `delegated`; the code name is kept as a string only |
| OPTIONS `SI`, `RAD` | Read (§5.1) |
| OPTIONS `Z` | Read: the exact rotation of §5.1 |
| OPTIONS unknown token | `MBF-IMPORT-OPTION-UNSUPPORTED` |
| OPTEMP field 1, reference temperature | Read (§9.2). Required whenever any LOADS temperature is read; a missing reference temperature is refused (`MBF-IMPORT-REFERENCE-TEMPERATURE-MISSING`), never taken from a default |
| OPTEMP thermal cycles, solve-thermal flag, cold-modulus flag | `delegated` |
| OPTEMP number of thermal loads | Absent or 1 is accepted. More than 1 is refused (`MBF-IMPORT-THERMAL-LOADS-UNSUPPORTED`; multiple thermal cases are tranche 2). When given, it decides which LOADS slots are active (§9.1) |
| OPCODE | `delegated`. It affects code stresses only |
| OPPRES | `delegated` without pressure. With pressure, fields 2 and 3 (Bourdon effect, pressure correction for bends) are read for the loss report (§9.4); the other fields stay `delegated` |
| OPMISC fields 1–8 (dynamics and hanger options) | `delegated` |
| OPMISC gravitational acceleration and direction (15.10 and later) | Two layouts are read. **Documented** (*Format of .MBF*, 10 fields): field 9 is the acceleration; a non-empty field 10, a direction whose one-field encoding no source states, is refused (`MBF-IMPORT-TBD-17-04-005`). **The export's** (observed format behaviour, 13 fields): field 9 has no public description and is read only as the `0` every export writes, else `MBF-IMPORT-FIELD-UNDOCUMENTED`; field 10 is the acceleration; fields 11–13 are the direction as three components. Only earth gravity is read, by layout (independent review VER-002 r5). **Export layout:** only the setting the export writes for the default environment, acceleration `386.4` with direction (0, −1, 0); the export writes `386.4` in SI files too, so this reading is confined to the 13-field layout. **Documented layout:** the acceleration is in the file's documented unit (m/s² in SI, in/s² in English, *Format of .MBF*, OPMISC) and is read only as standard gravity, 9.80665 m/s² or 9.80665/0.0254 = 386.0886 in/s², printed correctly rounded to its own digits (|p − g₀| ≤ half a unit in p's last digit) with at least three significant digits; so `386.4` in an SI file (39.4 g) is refused. Earth gravity is listed as `delegated` and recorded in `assumptions`: SWBPIPE has no gravity setting and carries weight as explicit primitives along −Y (§7.3), which matches earth gravity (`CAEPIPE-ANALYSIS`: input is given for earth gravity). Any other acceleration or direction scales or turns the weights and is refused (`MBF-IMPORT-GRAVITY-UNSUPPORTED`). **Z-vertical files (§5.1):** the export-layout setting is read only with direction (0, 0, −1), along the declared vertical (selected interpretation; no Z-vertical export has been observed), and (0, −1, 0) is refused, since in a Z-vertical file it would turn the weights horizontal. The documented layout gives no direction; its standard gravity acts along the declared vertical |
| OPMISC field counts other than 0–10 and 13 | Refused (`MBF-IMPORT-FIELD-UNDOCUMENTED`, §4.3) |

## 7. Materials (MATERIAL)

### 7.1 Records

- **First line.** Name, weight density, Poisson ratio, then optional joint factors, type,
  description, strengths and fatigue fields. Following lines give one temperature each,
  with E and α, then optional allowable, yield and rupture stresses (*Format of .MBF*,
  MATERIAL, non-FRP layout).
- **FRP types.** `FR`, `F1` and `F2` change the line layout. They are refused
  (`MBF-IMPORT-MATERIAL-FRP-UNSUPPORTED`).
- **Temperature-independent only.** A material is imported when every temperature line
  carries the same E and the same α, compared as parsed binary64. A single line counts.
  This is the exact inverse of the writer, which writes identical lines at three
  temperatures.
- **Varying values are refused** (`MBF-IMPORT-MATERIAL-TEMPERATURE-DEPENDENT`). SWBPIPE
  has `temperature_points`, but mapping them needs choices the MBF does not state: which
  modulus each case solves with (SWBPIPE's explicit `modulus_basis`), and how the tabulated α,
  which the public *Material* page documents as a mean coefficient, enters each case
  (O-7, §20.2). Identical rows are the one exact reading, since a constant E and a constant α
  give the same E and the same α(T₁ − T_ref) under every modulus basis and every reference
  temperature of the mean.
- **Ruling (O-7; Agent 0's decision under the owner's delegation, 2026-10-27).** A temperature-dependent MATERIAL stays refused until the reference
  temperature from which the tabulated mean α is measured, and the interpolation between rows
  for E and for α, are confirmed (DEL-17-01 CQ-17-01-016). It then takes the hot basis (§20.2,
  option A), for files whose OPTEMP modulus flag selects the hot modulus; cold-modulus files stay
  refused, since SWBPIPE's case basis selects E and α together.
- **Joint factors and type.** These are code-check settings. They are listed as
  `delegated` and are not imported.
- **Strength and code data.** These are the yield and tensile strengths,
  slope/constant/endurance, the fatigue curve name, and the allowable, yield and rupture
  columns. They never enter the model. A non-empty, non-zero value in any of them marks
  the material as suspected library or protected data, because the writer leaves them
  empty and allowables may be standards-derived (`docs/IP_AND_DATA_BOUNDARY.md` §3). The
  whole material is then refused (`MBF-IMPORT-MATERIAL-PROTECTED-SUSPECTED`), naming the
  column. No value is copied into the report.
- **Description** becomes the material `label`. Without one, the MBF name is the label.

### 7.2 Elastic constants (E, ν → G)

The MBF carries E and ν. The importer writes E (`MPa` or `psi`), ν (unit `1`), G and
`constitutive_basis: homogeneous_isotropic_E_nu_v1`. That is the only form the material
validators accept for a material carrying ν (DEL-02-01 0.7.0 Design §3.5, E/ν rule; W-17
§2.3a).

- G = E/(2(1+ν)) is derived as that rule states: E normalized to Pa, then the formula.
  `provenance_record.verification_note` says that G was derived from E and ν (isotropic)
  and names the values.
- **Inverse of the writer.** When the SWBPIPE source gave G and not ν, the writer derived
  ν = E/(2G) − 1 and the import re-derives G from it. The round-trip comparison (§18.2)
  compares G within its tolerance. When the source gave both and they disagreed, the writer
  warned `MBF-SI-SHEAR-MODULUS-NOT-EXPORTED`. Such models are outside the round-trip set.
- α = 0 is imported as an explicit coefficient of 0. The writer writes 0 for a material
  without a coefficient as well; §18.2 treats the two as equal.

### 7.3 Weight density → weight per length

SWBPIPE takes weight from `weight` primitives, and the writer turns those into a material
weight density over the bare metal area. The inverse, per element e with section area
A_e = π/4 (OD² − (OD − 2t)²):

- **English mode:** w_e = −ρ_w · A_e in `lbf/in`, with ρ_w in lbf/in³ and A_e in in². No
  conversion.
- **SI mode:** w_e = −ρ_w · 9.80665 · A_e in `N/m`, with ρ_w in kgf/m³ and A_e in m²
  (mm² × 10⁻⁶).

The result is one `weight` primitive per element with ρ_w ≠ 0: direction `global_y`,
dimension `force_per_length`, in the mechanical case (§9.3). ρ_w = 0 gives no primitive.
This is the writer's inverse.

- **Model-content assumption, recorded:** the weight acts on bare metal only. That holds
  because tranche 1 refuses non-zero contents, insulation, lining and additional weight
  (§8, §9.1).
### 7.4 Material provenance (required, affirmative)

An MBF cannot show where its material values came from. They may come from CAEPIPE's
material library, which is vendor data that may be standards-derived. A non-public
redistribution label does not prevent importing such data. The importer therefore
imports a material's mechanics data (E, ν, α, weight density) only with affirmative
provenance.

- **Declaration.** Every imported material needs an entry in the caller's
  `material_provenance` declaration, keyed by MBF material name (or `*` for all). The entry
  states either `project_invented` or `user_supplied`, and gives a `source_name`. A
  `user_supplied` entry states the user's own source; vendor-library data does not qualify.
- **Bound imports.** A bound sidecar qualifies only when its provenance affirmatively
  identifies project-invented data or the user's own supplied source; vendor-library,
  unknown and suspected protected origins remain refused. Redistribution disposition
  alone is insufficient.
  - **Mechanism (implemented).** The writer copies into each `material_map` entry an
    explicit `origin` member. It sets `project_invented` only when the source material's
    `provenance_record` states `contributor_certification: "invented non-engineering
    example"` with `redistribution_status: public_permissive` and carries no `library_ref`.
    It sets `user_supplied` only when the record's `contributor_certification` is
    `"user-supplied own source"` and the record carries no `library_ref`. Otherwise it sets
    no origin. Both are values the model schema admits: its `redistribution_status` enum has
    no invented value, so the certification string, the project's existing vocabulary for
    invented data, is the affirmative marker, and a disposition alone never qualifies.
  - **A recorded origin is the material's** (VER-002 r13). On a bound import the sidecar's
    recorded `origin` qualifies the material; a caller's declaration of another origin for it
    is refused (`MBF-IMPORT-SIDECAR-CONFLICT` at `material_map[<name>].origin`), neither being
    preferred, and a declaration of the same origin changes nothing.
  - **No origin, no import.** A material whose entry has no `origin` (format 2:
    `"undeclared"`) needs the caller's explicit declaration. That covers a material inserted from a library (`library_ref`
    present), a `private_only`, `public_permissive` or `unknown` status without such a
    statement, and a bare provenance string.
- **Refused** (`MBF-IMPORT-MATERIAL-PROVENANCE-REQUIRED`): a material with no
  declaration; a declaration naming vendor-library or unknown origin; and any material
  §7.1 marks as suspected protected.
- **Recorded.** The imported material's `provenance_record` holds the source and its
  disposition. A recorded origin keeps the certification the writer read it from
  (`"invented non-engineering example"` with `public_permissive`, or `"user-supplied own
  source"` with `private_only`), so the writer records the same origin again (§14). A declared
  origin holds the caller's source, the origin token as its certification, which the writer
  does not read as one (a re-export asks for the declaration again), and its disposition:
  `private_only` for `user_supplied`, and the declared disposition for `project_invented`. It also holds `source_location` `sha256:<mbf>` and the G derivation
  note (§7.2). PRD §18.4's provenance warnings then apply as usual.
- **Public tests.** Invented fixtures declare `project_invented` through their provenance
  sidecar.

## 8. Sections (PIPE)

- OD and wall thickness become each pipe segment's inline `section`, as the writer reads
  it: `outside_diameter` and `wall_thickness` in `in` or `mm`.
- **Zeros are absent.** Corrosion allowance, mill tolerance (a percentage in the MBF),
  insulation density and thickness, and lining density and thickness must be zero or
  empty, and are then written as absent. The MBF has no way to say "absent" and the writer
  writes 0 for absent. This is recorded in `assumptions`.
- **Under pressure** (§9.4) the exact pressure contract requires an explicit allowance on
  every section. A section whose PIPE record states field 4 (as 0, the only value read)
  gets `corrosion_allowance` 0 in the length unit. The import keeps whether the field is
  present: the manual's PIPE record documents no default for an omitted field, so a section
  used by an element that omits it, or leaves it empty, is refused under pressure
  (`MBF-IMPORT-CORROSION-ALLOWANCE-MISSING`, independent review VER-002 r1), never given an
  authored 0. Without pressure an omitted allowance stays absent. The other zero fields
  stay absent.
- **Refused:** a non-zero value of any of these, or a soil name
  (`MBF-IMPORT-SECTION-FIELD-UNSUPPORTED`). Insulation and lining weights, a non-zero
  corrosion allowance and buried pipe are later tranches.

## 9. Loads (LOADS) and load cases

### 9.1 LOADS records

A LOADS record is name, T1, P1, specific gravity, then optional fields: T2, P2, T3, P3,
additional weight, peak temperature and pressure, T4 to T10 with P4 to P10, and the
environmental-load flags (*Format of .MBF*, LOADS).

| Field | Tranche 1 |
|---|---|
| T1 | Read (§9.2) |
| P1 | Read (§9.4). A negative value is refused (`MBF-IMPORT-PRESSURE-UNSUPPORTED`): the manual gives no sign convention for external pressure |
| Specific gravity | Must be 0. Otherwise `MBF-IMPORT-CONTENTS-UNSUPPORTED` |
| T2…T10, P2…P10 | **Active slots.** When OPTEMP gives the number of thermal loads (N = 1 in tranche 1), slots above N are inactive: no load case uses them (`CAEPIPE-ANALYSIS`, *Number of Thermal Loads*; `CAEPIPE-LOAD-CASES`; OPTEMP field order from `CAEPIPE-IMPORT-MBF`). Each inactive slot is dropped and reported by record and slot as `MBF-IMPORT-LOAD-SLOT-INACTIVE` (info), whatever its values; they must still be numbers. CAEPIPE's export fills every slot, with a default temperature in unused temperature slots and 0 in unused pressures (observed format behaviour). **Without a count**, the conservative rule applies: a slot is inactive only when it is empty, or T equals the OPTEMP reference with P = 0 (reported the same way); anything else is ambiguous and refused (`MBF-IMPORT-THERMAL-LOADS-UNSUPPORTED`) |
| Additional weight | 0 or empty. Otherwise `MBF-IMPORT-LOAD-FIELD-UNSUPPORTED` |
| Peak temperature, peak pressure | `delegated`. They only feed CAEPIPE's peak case |
| Wind, snow, ice and wave flags | 0 or empty. Otherwise `MBF-IMPORT-LOAD-FIELD-UNSUPPORTED` |

### 9.2 Temperatures

- Each element's load reference (§10.3) gives T1. Its thermal change is ΔT = T1 − T_ref,
  with T_ref from OPTEMP.
- ΔT ≠ 0 gives one `thermal` primitive on the element: dimension `temperature_interval`,
  unit `degC` or `degF`, in the thermal case. It uses the same direction token as the
  pilot cases.
- ΔT = 0 gives no primitive. This is the writer's inverse: the writer writes T1 = T_ref + ΔT.

### 9.3 Load cases

The writer maps mechanical cases to the sustained label `LOAD_CASE_LABELS["sustained"]`.
It maps thermal and imposed-displacement cases to the first expansion label
`LOAD_CASE_LABELS["expansion"]` (writer assumptions; DEL-17-05 Design §6). The
importer forms at most two primitive load cases:

- `case:mbf-sustained`, labelled with the sustained label, `kind: sustained`, holds weight, concentrated
  forces and concentrated moments. Forces and moments are placed there per §13's case
  rule.
- `case:mbf-expansion-t1`, labelled with the expansion label, `kind: thermal`, holds thermal changes and
  anchor displacements and rotations.

Further rules:

- Both cases carry `status: draft`, the desktop's status for a new case.
- A case with no primitives is not created, except the sustained case when it carries
  pressure regions (§9.4).
- No `combinations` are created. CAEPIPE forms its own (*Load cases*), and that is listed
  as `delegated`.
- With a bound sidecar (§15), the original case ids are restored from `load_case_map`.

### 9.4 Pressure

**Source.** LOADS Pressure 1 is in psi in English files and in bar in SI files (documented:
*Format of .MBF*, LOADS). The importer keeps that unit, as it keeps every other unit
(§5.2), and the units catalog reads it to Pa as value × factor in binary64 (`core/units`).
For bar the factor is 100000.0, exact, so the reading is one rounding of the exact product.
For psi the factor `PSI_IN_PASCALS` is the binary64 quotient of the binary64 constants
4.4482216152605 and 0.00064516, which is 6894.757293168361, one unit in the last place below
6894.757293168362, the correctly rounded value of the definition 0.45359237 × 9.80665 /
0.0254² (independent review VER-002 r1). A psi pressure therefore carries that constant's
error, about 1.3e-16 relative, plus one rounding; psi moduli read the same way. The writer
converts with exact rational factors and rounds once (below).

**Route.** A model document with pressure takes the exact pressure contract; a document
without `pressure_contract` is pressure-free, and pressure primitives are retired (DEL-04-07
Design `pressure-contract.md`, "Current pressure boundary"; `schemas/model.schema.yaml`
`PressureContract`, `PressureRegion`). The importer therefore writes, when any element has
Pressure 1 ≠ 0:

- `pressure_contract` = `{version: 3.0.0, mode: exact_pressure_v3, shear_model}`. The MBF
  states no shear model. An unbound import takes the exact contract's default,
  `timoshenko_annulus_energy_v1`, Timoshenko with the energy-matched annular shear factor
  (owner D-6), or the caller's optional `shear_model` argument, which admits exactly the
  contract's two values (`euler_bernoulli`, `timoshenko_annulus_energy_v1`); any other value
  is refused (`MBF-IMPORT-SHEAR-MODEL-INVALID`). A bound import keeps the sidecar's shear
  model, and a different `shear_model` argument with it is refused
  (`MBF-IMPORT-SIDECAR-CONFLICT`). Realized curved bends stay Euler–Bernoulli under either
  model, as the route discloses. The assumptions state the model and its basis (Agent 0's
  decision under the owner's delegation, 2026-10-27).
  **Disclosure.** The public technical-reference page "Fiber Reinforced Plastic Piping
  (FRP)", section "Stiffness matrix", forms pipe flexibility with a transverse-shear term
  (shape factor × L/GA). Its use for steel pipe is an inference, and no shape-factor value
  is published; the question is open with SST (CQ-17-01-010). SWBPIPE's energy-matched
  annular factor may therefore differ from the external tool's unpublished factor;
- the explicit corrosion allowance on every section, which each PIPE record must state
  (§8);
- `pressure_regions` on every case: the regions on the sustained case, and an explicit
  empty list on the expansion case. Pressure 1 acts in CAEPIPE's sustained and operating
  cases and not in the expansion case (`CAEPIPE-LOAD-CASES`, case membership), and the
  writer maps the sustained case to the sustained label (§9.3);
- each region with basis `internal_differential_zero_external_v1`, the file's pressure and
  unit, its member pipes in chain order, two terminals and provenance naming the LOADS
  record. Materials are already on the E/ν basis the route needs (§7.2).

**Regions (unbound).** A region must be one acyclic chain of equal bore whose internal
nodes join no other pipe and which changes direction only at a realized bend
(`pressure_runtime.rs`: `PRESSURE_REGION_TOPOLOGY_INVALID`, `_INTERNAL_BRANCH`,
`_BORE_MISMATCH`, `_NONCOLLINEAR`). The route defines no boundaries itself: it admits or
refuses the regions a document states. The importer forms regions only at boundaries the
model has, checks each straight run whole with the route's own check, and never cuts a
straight run that the check admits (the lane manager's ruling after independent review VER-002 r2:
a cut adds closure terminals the file never had):

- **Chains.** A chain continues through a node only when the node joins exactly two
  elements with the same Pressure 1 and the same bore OD − 2t, both compared exactly. It
  therefore ends at a free end, a tee, a change of pressure or bore, or an element with
  Pressure 1 = 0, which belongs to no region. A bend continues its chain, and its end
  junctions are checked as the route checks them (**Bends**, below).
- **The route's check.** The route checks each straight run of a region, between its
  terminals or between bends, as a whole (`collinear_run`): from the run's first node, every
  later node's offset divided by the span must lie within 64ε = 64 × 2⁻⁵² of the span
  direction (correctly rounded norm), and advance without passing the span's end. The
  importer ports it operation for operation, on node positions read to metres as the
  catalog reads them (value × 0.001 or × 0.0254 + 0). Its correctly rounded norm rounds a
  subnormal result once on the subnormal lattice (VER-002 r2). Admission is not monotonic in
  the run's extent, so neither adjacent pairs nor prefixes decide it: sixteen 1000 mm
  straights whose Y offsets grow by 1e-11 mm per row turn by about 1e-14 rad at each node,
  yet the run's residual is about 2e-14 (refused); two straights meeting 3e-11 mm off a
  2000 mm line fail alone (1.5e-14), yet with an 8000 mm straight after them the whole run
  passes (3e-15).
- **Corners.** A node between two straights is a corner when their direction change exceeds
  the route's mitre threshold α_tan = 1e-3 rad (`curved_bend` `TANGENCY_TOLERANCE_RAD`),
  measured as the route's `kink`, 2·atan2(|a − t|, |a + t|) of the unit directions. Only a
  run that fails the check whole is examined for corners, so a drift the check admits is
  never a corner, whatever its local angles.
- **Bores.** The route compares each member's pressure bore with the first member's, in
  traversal order: r_i,r = OD/2 − ((t − c_m) − c_a) in metres, which must agree within 64ε of
  the larger radius (`pressure_runtime.rs`, `PRESSURE_REGION_BORE_MISMATCH`;
  `ExactSectionBases::derive`). The importer ports it with the same metre reading, mill
  tolerance absent and the stated allowance 0. Equal decimal bores do not settle it:
  128.2 − 2 × 63.6 and 128.3 − 2 × 63.65 are both 1 mm, but in metres the radii differ by
  about 5.55e-14 relative (independent review VER-002 r3). The chain still needs exactly
  equal decimal bores, so it may end where the route would admit one region; such a junction
  is a real, if tiny, bore change.
- **Bends.** The route forms each realized bend's arc from its tangent nodes, radius and
  y_reference in metres (`curved_bend` `objective_arc`). At each junction beside a bend it
  requires θ = kink(t_in, t_out) ≤ α_tan between the bend's end tangent and the adjacent
  member's direction (a straight's frame axis or the next bend's tangent), and refuses a
  larger θ as a mitre (`PRESSURE_REGION_MITRE_UNSUPPORTED`). A tangent, angle or member
  orientation it cannot form is `PRESSURE_REGION_GEOMETRY_INVALID`. §11 builds the tangent
  nodes in the file's unit, and the metre reading need not keep them tangent: near
  1e15 mm the binary64 spacing in m is 2⁻¹³, so a 50 mm bend's chord moves by about 1e-3
  relative and its start tangent leaves its straight by 1.46e-3 rad (independent review
  VER-002 r6). The importer ports these checks operation for operation on the values read
  to metres: product_physics's curved-bend screen, `objective_arc` with its exact sums, the
  frame orientation (§10.5) and `kink` with the runtime's arc angle 2·atan2, evaluated to 90
  digits and rounded once (it reproduces the runtime's frozen table bit for bit). kink is
  the same in either traversal order, so a refused junction refuses both orders.
- **The rule.** Each straight run of a chain is checked whole, in either traversal order. If
  it passes, it stays in one region. If it fails both ways, the region ends at each of its
  corners. Each region is then checked whole as the route checks it, its straight runs and
  its bores, first in file order and, if refused, reversed (independent review VER-002 r3).
  The first order the checks admit is emitted. A region that neither order admits is refused
  by name with its elements, and never cut: `MBF-IMPORT-PRESSURE-RUN-NONCOLLINEAR` for a
  straight run refused both ways (a run without a corner included),
  `MBF-IMPORT-PRESSURE-BORE-MISMATCH` for bores refused both ways,
  `MBF-IMPORT-PRESSURE-MITRE` or `MBF-IMPORT-PRESSURE-GEOMETRY-INVALID` at a bend, naming
  the node or element, and the first code with each order's failing check named when the
  two orders fail different checks. So a kink of
  1e-4 or 5e-4 rad in a run is refused, one of 2e-3 rad is a corner, and the review's run from
  (0, 0) through (2, 3.0000000000924) to (2000, 3000) mm, refused traversed from its first
  node, is emitted from its last.

- **Order and ids.** Members are in chain order. In file order the first terminal is the
  chain end whose element comes first in layout order; a region the route admits only
  reversed is emitted from its other end. Reversal changes nothing else: the members and
  pressure are the same, each terminal keeps its own node's closure transfer, and the route
  reads a bend traversed backward by its end tangents. The id is `region:mbf-<first
  terminal>-<second terminal>` by CAEPIPE node name. `id_map.region_map` lists each
  region's elements.
- **Corners** (Agent 0's decision under the owner's delegation, 2026-10-27). Two regions meet
  at a sharp straight-to-straight corner, each with a terminal there. Both corner terminals
  take `transfers_to_wall`, whatever the caller's global declaration: the pipe is continuous
  there, so its wall carries the thrust, and the end forces pAᵢ along each straight sum to
  pAᵢ(t_in − t_out) at the node, the resultant pressure force at a sharp corner. This is
  deduced from geometry, not inferred from the file, and each corner terminal's provenance
  and the assumptions say so. A `pressure_closure.nodes` entry naming the corner node still
  governs it. The exact route admits no mitre inside a region
  (`PRESSURE_REGION_MITRE_UNSUPPORTED`), and this partition does not place one there.

**Closure transfer is declared, never inferred.** Each terminal needs `transfers_to_wall` or
`separately_supported_or_compensated` (`pressure_runtime.rs`: "closure transfer is never
inferred"). The MBF does not state it. An unbound import with pressure therefore needs the
caller's declaration `pressure_closure` = `{closure_transfer, provenance, nodes}`:
`closure_transfer` applies to every real end (a free end, a tee, a change of pressure or
bore), `nodes` (optional) maps CAEPIPE node names, corner nodes included, to another closure
transfer, and each such terminal's provenance is "closure transfer declared at import:
<provenance>". A corner terminal no `nodes` entry names takes `transfers_to_wall`
(**Corners**, above). Without it the import is refused
(`MBF-IMPORT-PRESSURE-CLOSURE-REQUIRED`). A malformed declaration, or a `nodes` name that is
not a terminal, is refused (`MBF-IMPORT-PRESSURE-CLOSURE-INVALID`). A file without pressure
needs no declaration.

**Bound imports.** The writer's sidecar `pressure_map` restores the contract, the region
ids, members in order, and the terminals with their closure transfer and provenance, as the
sidecar parser types them (§15). It is checked against the file: every member must carry a
non-zero Pressure 1, equal across the region, and belong to one region only; every element
with pressure must belong to one; each terminal must name a model node. Otherwise the import
is refused (`MBF-IMPORT-SIDECAR-CONFLICT`). The pressure value always comes from the
file. A format-2 sidecar always records the effective pressure state (§15): `contract` (the
contract or `"none"`), the effective `shear_model` (Euler–Bernoulli without a contract) and
the regions; one missing, null or incomplete is refused by path
(`MBF-IMPORT-SIDECAR-INVALID`, VER-002 r10, r11). A legacy sidecar (main's writer) records
none and binds only a pressure-free file. A bound import of a file with any Pressure 1 ≠ 0
whose sidecar records no contract (`"none"`, or legacy) is refused
(`MBF-IMPORT-SIDECAR-CONFLICT`, `pressure_map`) before any region is formed or the shear
model defaulted: a bound import never falls
through to the unbound route (independent review VER-002 r9). On a file without pressure, a
pressure state without a contract restores the file as pressure-free, and the writer's
contract-only state (the contract, its shear model, no regions) restores the contract with
empty region lists, with or
without a LOADS section; a region the file does not carry is refused
(`MBF-IMPORT-SIDECAR-CONFLICT`).

A restored region is then checked whole as the route checks it, with the same ported checks
as an unbound region (its straight runs, its bores and its bend junctions, on the MBF's
values read to metres),
in its declared traversal order: from its first terminal, through its members, to its
second. It is never cut or reordered. A region that is not one chain between its terminals
is refused as `MBF-IMPORT-SIDECAR-CONFLICT`, as is a repeated region id (an empty one the
parser refuses, §15); one the checks refuse is refused as
`MBF-IMPORT-PRESSURE-RUN-NONCOLLINEAR`, `MBF-IMPORT-PRESSURE-BORE-MISMATCH`,
`MBF-IMPORT-PRESSURE-MITRE`, `MBF-IMPORT-PRESSURE-GEOMETRY-INVALID` or, where another pipe
joins an internal node, `MBF-IMPORT-PRESSURE-INTERNAL-BRANCH`, located at `pressure_map
region <id>` (independent review VER-002 r4, r5, r6). A byte-bound file can fail these
checks although its source model passed them, because the writer prints 10 significant
digits: OD 168.30000004 and wall 7.11000002 give the same bore radius in metres as 170.3
and 8.11, but print as 168.3 and 7.11000002; and two offsets collinear as authored need not
be collinear as printed. The writer's preflight below refuses such an export, so a bound
import fails these checks only if the file was not written by the current writer.

**Guard map (VER-002 r5 to r8).** Every refusal `pressure_runtime.rs` raises for a model,
region or member (47 codes, with the two admission-seam codes it raises), and what keeps an
import from reaching it. U is the unbound route, B the bound (restored) route; the writer's
preflight re-imports bound, so B covers exports too. A bound import restores sidecar
content, so no B claim rests on what the importer itself writes: each restored field is
checked against what the import admits (§15), and a row that holds "by construction" names
the restored fields it depends on (independent review VER-002 r7). The sidecar parser (§15)
is the guard of record for every structural refusal on B: a restored value of the wrong
type, outside its vocabulary, empty where an id is required, or outside binary64, and any
unknown key, is refused by JSON path (`MBF-IMPORT-SIDECAR-INVALID`) before anything reads
the sidecar (r8).

| Runtime code | Importer check | Route |
|---|---|---|
| `PRESSURE_REGION_NONCOLLINEAR` | ported `collinear_run` per straight run (`MBF-IMPORT-PRESSURE-RUN-NONCOLLINEAR`) | U, B |
| `PRESSURE_REGION_BORE_MISMATCH` | ported bore guard (`MBF-IMPORT-PRESSURE-BORE-MISMATCH`) | U, B |
| `PRESSURE_REGION_INTERNAL_BRANCH` | U: a chain continues only through a node joining exactly two pipes; B: ported (`MBF-IMPORT-PRESSURE-INTERNAL-BRANCH`, new in r5) | U, B |
| `_TOPOLOGY_INVALID`, `PRESSURE_TERMINALS_INVALID`, `PRESSURE_TERMINAL_NODE_UNKNOWN`, `_MEMBER_UNKNOWN`, `_MEMBERS_INVALID`, `_PIPE_OVERLAP` | U: chains of existing pipes, each in one region, with their two ends as terminals; B: membership, ownership and one chain from the first terminal to the second, (`MBF-IMPORT-SIDECAR-CONFLICT`), and every restored id non-empty after Rust's `trim` (the parser, `MBF-IMPORT-SIDECAR-INVALID`, r6, r8) | U, B |
| `_INPUT_MISSING`, `_ID_DUPLICATE` | U: formed ids, provenance, pressure and members; B: as restored, with ids and provenance non-empty after Rust's `trim` (the parser) and region ids unique (r5, r6, r8) | U, B |
| `PRESSURE_TERMINAL_INPUT_MISSING`, `PRESSURE_TERMINAL_CLOSURE_INVALID` | U: the closure declaration (`MBF-IMPORT-PRESSURE-CLOSURE-REQUIRED`, `-INVALID`); B: restored node, closure and provenance checked | U, B |
| `_PRESSURE_INVALID` | the file's value read finite (§4.3), and finite in Pa as the route reads it, value × 10⁵ or × `PSI_IN_PASCALS` + 0 (`MBF-IMPORT-VALUE-UNREPRESENTABLE`, new in r6) | U, B |
| `EXACT_PRESSURE_CORROSION_ALLOWANCE_REQUIRED` | `MBF-IMPORT-CORROSION-ALLOWANCE-MISSING`; the writer's `MBF-SI-CORROSION-ALLOWANCE-MISSING` | U, B |
| `EXACT_PRESSURE_RESULT_ID_COLLISION` | ported per kind, ':' read as '-' (`MBF-IMPORT-PRESSURE-ID-COLLISION`, new in r5) | U, B |
| `_GEOMETRY_INVALID` | ends coinciding in metres refused (`MBF-IMPORT-GEOMETRY-ZERO-LENGTH`); each straight's y_reference passes the frame kernel's complete orientation check on the positions in metres (§10.5, r6): a restored one that fails is refused by name with the element, nothing substituted, and so is a documented one that fails (`MBF-IMPORT-ORIENTATION-UNUSABLE`); bend tangents, junction angles and member orientations are formed as the route forms them, in metres (**Bends**; `MBF-IMPORT-PRESSURE-GEOMETRY-INVALID`, new in r6). A straight no longer than the frame kernel's 1e-12 m is left to the solve's named refusal (§10.1: no minimum length) | U, B |
| `EXACT_PRESSURE_MATERIAL_INVALID` | E > 0 and 0 < ν < 0.5 (`MBF-IMPORT-VALUE-OUT-OF-RANGE`) | U, B |
| `PRESSURE_REGION_MITRE_UNSUPPORTED` | ported tangency check at each junction beside a bend, on the arc formed in metres (**Bends**; `MBF-IMPORT-PRESSURE-MITRE`, new in r6). r5 listed it as unable to arise, reasoning in the file's unit; the metre reading breaks that (VER-002 r6) | U, B |
| `PRESSURE_MODEL_REAUTHOR_REQUIRED`, `PRESSURE_CONTRACT_UNSUPPORTED`, `EXACT_PRESSURE_SHEAR_MODEL_REQUIRED`, `PREVIEW_SCHEMA_VERSION_UNSUPPORTED`, `PREVIEW_CONTRACT_VERSION_MISMATCH`, `_BASIS_UNSUPPORTED`, `CORROSION_ALLOWANCE_WITHOUT_PRESSURE_CONTRACT`, `EXACT_PRESSURE_REQUIRES_REGION`, `EXACT_PRESSURE_REGIONS_REQUIRED` | by construction: the importer writes only the current schema version, the one contract, the one basis, the allowance only under the contract, no pressure primitive, and a region list on every case. B restores only the contract's shear model and the regions; a restored contract other than 3.0.0/exact_pressure_v3 with a shear model from `SHEAR_MODELS` is refused, no longer replaced by euler_bernoulli (`MBF-IMPORT-SIDECAR-INVALID`, new in r7) | U, B |
| `EXACT_PRESSURE_FAMILY_NOT_ADMITTED` | U: only anchors and rigid restraints, without family or as `anchor`, `guide` or `line_stop`, linear springs (`family: spring`, which `exact_admission` `support_family` admits; tranche 3, §12), and realized bends; no hanger, nonlinear support, connector, combination or equivalent-static load. B: the parser refuses a restored support family other than none, `anchor`, `guide`, `line_stop` or `vertical_support` (the rigid families the writer exports) and, in format 3 only, `spring` on one translation with `family_authored` true, an `anchor` restraining fewer than six DOFs, a restrained list that is not distinct DOFs, and supports the solve's family, DOF and overlap rules refuse (§12), by support id (`MBF-IMPORT-SIDECAR-INVALID`, r7, r8; r5 and r6 wrongly listed it as unable to arise). Components restore only their id | U, B |
| `JOINT_PRESSURE_INTERFACE_UNRESOLVED` | by construction: raised only for a joint a region lists as a connector member; the importer writes no connector and no region connector list, and on B the parser refuses any region field other than `id`, `members`, `terminals`, `provenance` and `pressure_bar` (new in r8; omitted from the r5 to r7 inventory) | U, B |
| `OBJECTIVE_CONNECTOR_VERSION_UNSUPPORTED`, `OBJECTIVE_CONNECTOR_NOT_IMPLEMENTED`, `EXACT_PRESSURE_REDUCER_IN_REGION` | by construction: the importer writes no connector and no reducer; B restores no component kind, interface or connector field, only the bend component's id (a string, else `MBF-IMPORT-SIDECAR-INVALID`) | U, B |
| `EXACT_PRESSURE_MATERIAL_AMBIGUOUS`, `_MATERIAL_MISSING`, `_MATERIAL_BASIS_REQUIRED`, `_POISSON_RATIO_REQUIRED`, `THERMAL_EXPANSION_INPUT_MISSING`, `LOAD_STATE_MEMBER_MATERIAL_MISSING`, `_SECTION_MISSING`, `_NOT_NORMALIZED`, `EXACT_PRESSURE_REGION_MEMBER_NOT_STRAIGHT` | by construction: each pipe names one imported E/ν material with explicit ν and α (B restores only material ids, merged only where E, ν and α are identical, and an origin from `MATERIAL_ORIGINS`); no load-state case (B restores case ids only); every pipe has a section, whose 0 < t < OD/2 is checked again on the values in m (§4.3, new in r6); bar and psi normalize to Pa in the solve's own seam; the last is an unreachable guard by its source comment | U, B |
| `EXACT_PRESSURE_OUTPUT_UNREPRESENTABLE`, `PRESSURE_LEDGER_SUM_UNREPRESENTABLE`, `PRESSURE_ASSEMBLY_COEFFICIENT_UNREPRESENTABLE`, `PRESSURE_ASSEMBLY_SOURCE_UNREPRESENTABLE`, `PRESSURE_ASSEMBLY_CANCELLATION_UNRESOLVED`, `NUMERICAL_INTEGRITY_ASSEMBLY_UNRESOLVED` | not an import guard: solve-time arithmetic of the assembled loads and stiffness, refused by name by the solve; the importer keeps every input finite. The last depends on the assembly arithmetic over the whole model: a spring stiffness negligible against the assembled stiffness (1e-300 N/mm at a cantilever's tip) imports as written and the solve refuses it by this code (tranche 3, VER-002 r2; regression in the round-trip tests) | — |
| `CHECKED-JSON-UNSAFE-INTEGRAL-FLOAT`, `CHECKED-JSON-UNSAFE-INTEGER`, `CHECKED-JSON-NON-FINITE` (the checked-JSON reader of the solve's input, canonical_json `validate_checked_value`, profile `openpipestress_jcs_ijson_v1`; not pressure codes, listed with them because every route reaches them) | ported: every number of the emitted document, whatever its field (coordinates, section, material, loads, pressures, temperatures, displacements, stiffnesses), must be an integer within ±(2⁵³ − 1), finite, and not an integral float beyond 2⁵³ − 1; else `MBF-IMPORT-VALUE-UNREPRESENTABLE` at its JSON path, nothing changed or dropped (tranche 3, VER-002 r2). The writer's preflight inherits the refusal as `MBF-SI-PREFLIGHT-REFUSED` | U, B |

Related: the exact route's `EXACT_IMPOSED_DISPLACEMENT_PRIMITIVE_UNSUPPORTED` is kept out by
`MBF-IMPORT-PRESSURE-DISPLACEMENT-UNSUPPORTED` (above).

**CAEPIPE pressure effects SWBPIPE does not model.** OPPRES fields 2 and 3 switch CAEPIPE's
Bourdon effect and pressure correction for bends, and a blank OPPRES line turns both on
(documented: *Format of .MBF*, OPPRES). SWBPIPE's exact route models neither (DEL-04-07
D-3). With pressure, each effect that is on, by value or by the blank-line default, or left
unstated (a missing field, or no OPPRES section), is listed as `omitted`
(`MBF-IMPORT-PRESSURE-EFFECTS-OMITTED`). A value other than 0 or 1 is refused
(`MBF-IMPORT-FIELD-UNDOCUMENTED`).

**Anchor displacements with pressure** are refused (`MBF-IMPORT-PRESSURE-DISPLACEMENT-UNSUPPORTED`).
The exact route takes boundary motion only in load-state form and refuses a support-targeted
primitive (`EXACT_IMPOSED_DISPLACEMENT_PRIMITIVE_UNSUPPORTED`, DEL-04-07 Design), and this
importer writes no load-state cases.

**Writer counterpart.** Under the same contract (any shear model), each pressure region of
the sustained-mapped case becomes Pressure 1 on its member elements, in bar: Pa × 10⁻⁵,
kPa × 10⁻², MPa × 10, bar × 1, and psi × 0.45359237 × 9.80665 / 0.0254² / 10⁵, each the
exact rational product rounded once (psi is listed in `MBF-SI-UNITS-NORMALIZED`). Elements
outside every region get 0, and LOADS records are keyed by (T1, P1). With any pressure the
writer adds `OPPRES` `0,0,0,1,0` after OPTEMP: PD/4t, Bourdon effect off, pressure
correction for bends off, peak pressure factor 1. That asks the target for the same
exclusions as SWBPIPE's model. The contract, region ids, members, terminals, closure
transfer and provenance are not in the MBF. The sidecar's `pressure_map` carries them
(`{contract, load_case, regions: [{id, members, terminals, provenance, pressure_bar}]}`),
and the shear model is listed as omitted (`MBF-SI-SHEAR-MODEL-NOT-EXPORTED`). The writer
refuses:

- a region in a case with thermal or displacement primitives
  (`MBF-SI-PRESSURE-CASE-UNSUPPORTED`: Pressure 1 does not act in the expansion case);
- a negative pressure, regions without the exact contract, another contract, and regions
  with connectors (`MBF-SI-PRESSURE-UNSUPPORTED`);
- a bend whose pressure differs from its incoming straight's (`MBF-SI-BEND-UNSUPPORTED`:
  one layout row carries both);
- a section without an explicit `corrosion_allowance` under the exact contract, whether or
  not it carries pressure (`MBF-SI-CORROSION-ALLOWANCE-MISSING`, VER-002 r2). The solve
  refuses such a model (`EXACT_PRESSURE_CORROSION_ALLOWANCE_REQUIRED`), and the MBF's
  written 0 would author an allowance on import;
- any export whose own bound re-import does not succeed completely (VER-002 r4, r5). The
  writer's preflight imports its own text, bound to its own sidecar, so the printed values
  are read exactly as an import reads them (positions summed from the printed offsets, bend
  tangent nodes rebuilt, OD and wall as printed). The restored regions then take the same
  whole-region checks in their declared order, and the re-import must restore every written
  region. A region the model's values admit but the printed values would not, by any region
  check (bend junctions included), is refused as `MBF-SI-PRESSURE-REGION-FORMAT`, naming the
  region. A recorded y_reference the frame kernel cannot use on the printed positions is
  refused by the re-import (`MBF-IMPORT-ORIENTATION-UNUSABLE`, §10.5), so the export is
  refused as `MBF-SI-PREFLIGHT-REFUSED`, naming the pipe and carrying that code. Any other re-import refusal, at any
  stage, is refused as `MBF-SI-PREFLIGHT-REFUSED`, carrying the importer's code, location
  and line: for example OD 100.00000004 with wall 50 prints as 100, which the import
  refuses (`MBF-IMPORT-VALUE-OUT-OF-RANGE`), and a region through a tee
  (`MBF-IMPORT-PRESSURE-INTERNAL-BRANCH`). The preflight runs for every export (it is total,
  §14). The writer refuses rather than printing more digits. The
  10-significant-digit print is the profile's established form, and the §18.2 bounds and
  byte reproduction rest on it. More digits would not settle the geometry either: an offset
  is a binary64 difference of two positions, so its exact decimal does not, summed, give the
  authored position back. The region is never cut or reordered.

A zero-pressure region writes Pressure 1 = 0 and is listed as omitted
(`MBF-SI-PRESSURE-REGION-ZERO`), since it imports as no region.

## 10. Layout geometry

### 10.1 Rows

Row meanings follow *Format of .MBF*, LAYOUT, and *DX, DY, DZ*:

- **From row `F<n>`.**
  - **Unknown node:** a new run begins there. X, Y, Z are absolute coordinates. Only the
    first From row may omit them, meaning the origin (*DX, DY, DZ*). Any other unknown From
    node without coordinates is refused (`MBF-IMPORT-NODE-POSITION-MISSING`). On a From row
    that gives some coordinates, a missing axis is 0: CAEPIPE's export writes only the non-zero
    absolute coordinates of a From node (observed format behaviour; no public rule states it).
    It is safe to read so because the only alternative readings, refusing the row or taking
    the axis from another node, have no basis either; the documented rules nearest it also
    give an absent value as 0 (the first From row at the origin, an absent To-row offset); the
    row still states a position, as a From row with no coordinates at all stays refused; and a
    misread position would surface in the private regression's canonical comparison (§18.3).
  - **Known node:** a branch starts there. Coordinates may be omitted. If given, they must
    equal the node's position within the half-unit bounds of their printed digits (§18.2's
    δ, summed over the node's path); otherwise
    `MBF-IMPORT-FROM-COORDINATES-CONFLICT`.
- **To row `T<n>`.** A new node, reached by a pipe from the previous From or To node. A
  Location row in between does not move the start. X, Y and Z are offsets. A missing axis
  is a zero offset. A zero-length element is refused (`MBF-IMPORT-GEOMETRY-ZERO-LENGTH`):
  the manual gives zero length to joint elements (*DX, DY, DZ*), and those are refused.
  The check is repeated on the rounded binary64 positions: distinct exact positions that
  round to one point are refused the same way.
- **No minimum length.** Any representable positive length imports; the importer sets no
  minimum element length (a 1e-200 mm straight is read). If the solve cannot form a very
  short element, the solve's own refusal is the report (independent review VER-002 r2).
- **To row naming an existing node** would close a loop. It is refused
  (`MBF-IMPORT-TOPOLOGY-LOOP`), as the writer refuses loops. Loops are tranche 2.
- **Location row `L<n>`.** Adds comment data at an existing node
  (`MBF-IMPORT-NODE-UNKNOWN` otherwise). It carries no offsets and no M, P, K or J
  (`MBF-IMPORT-ROW-KEY-UNSUPPORTED`). It does not move the run's current node.

### 10.2 Absolute coordinates

An absolute-coordinate marker `*` makes a row's X, Y and Z absolute coordinates instead
of offsets. The element runs from the previous node to that position.

- **The sources disagree on marker placement.** The *Node* page describes the marker as a
  prefix to the node number. *DX, DY, DZ* describes it as following the number, and the
  *MBF Generation in Excel with Absolute Coordinates* example shows it trailing (as in
  `T40*`).
- **Selected interpretation.** This profile selects the trailing marker shown in MBF
  examples. The alternative, a `*` anywhere else in the row key, remains refused pending
  clarification (`MBF-IMPORT-TBD-17-04-009`; TBD-17-04-009, CQ-17-01-015). Missing axes on such a row are refused rather than read as
zero, because the manual gives no rule for them.

### 10.3 Carried properties

- **Material and section.** `M<name>` and `P<name>` stay in effect until the next M or P
  (*Format of .MBF*, LAYOUT M and P).
- **Load reference.** The comment `L=<name>` assigns a LOADS record.
  - **Bound imports:** the writer's convention applies. The reference is carried until
    changed, as M and P are, and that is recorded in `assumptions`.
  - **Unbound imports:** an `L=` comment is refused (`MBF-IMPORT-TBD-17-04-007`),
    except where an explicitly retained bounded profile decision admits it.
  - **Why it is open.** The public example MBF shows the load name as a layout comment
    and omits it on the rows that follow. The manual's comment list does not include it.
    SST's confirmation is TBD-17-04-007 (CQ-17-01-013).
- **Missing property.** An element row with no material or section in effect is refused
  (`MBF-IMPORT-PROPERTY-MISSING`).
- **No load reference.** An element row with no load reference in effect is refused when
  the file has a LOADS section. When the file has no LOADS section, the element is
  imported with no temperature, pressure or contents load. The loss report lists that as
  `tbd` (`MBF-IMPORT-LOADS-ABSENT`, warning), because CAEPIPE's treatment of an element
  without a load record is not documented.
- **Unresolved name.** A name that no MATERIAL, PIPE or LOADS record defines is refused
  (`MBF-IMPORT-REFERENCE-UNRESOLVED`).
- The writer sets M, P and L on the first From row. A property key on a From row applies
  to the elements that follow.

### 10.4 Node numbers and ids

- A node number is a positive integer below 99999. Other forms are
  `MBF-IMPORT-NODE-NUMBER-INVALID`. Suffixes A, B, C, D and J are CAEPIPE-generated
  (*Node*) and never appear on input rows; a suffixed input node is refused.
- Without a sidecar, SWBPIPE ids are formed deterministically from CAEPIPE names:
  - node `n` → `node:n`;
  - the bend ends of TI row `n` → `node:nA` and `node:nB`, the end-node names the public technical reference
    gives (`CAEPIPE-BEND`), which the writer's node map already uses;
  - element from `a` to `b` → `pipe:a-b`;
  - bend → `component:bend-n`;
  - anchor at `n` → `support:n`; restraint at `n` → `support:n-restraint`; guide at `n` →
    `support:n-guide`; limit stop along axis `a` → `support:n-stop-a` (`x`, `y`, `z`, SWBPIPE's
    axes); spring on DOF `d` → `support:n-spring-d` (`ux`, `uy`, `uz`);
  - material `M` → `material:M`;
  - primitive loads → `load:<category>:<target id>[:<axis>]`.
- Node labels are the CAEPIPE names. Pipe labels are `a-b`. Support labels are the node's
  `TAG`, else `anchor at n`, `restraint at n`, `guide at n`, `limit stop at n` or `spring at n`; bend components are labelled `bend n`; load
  cases carry the writer's case labels (§9.3).

### 10.5 Straight-pipe orientation

The MBF carries no element orientation. A straight pipe still needs a `y_reference`, which
the solve requires, so the importer assigns one deterministically:

- **Bound imports** restore it from the writer's sidecar: `element_map[pipe].y_reference`,
  which a format-2 sidecar records for every straight (its effective one, with
  `y_reference_authored`; §15) and a legacy one for every straight with an authored
  `y_reference`. It is restored
  only if it is a finite vector from which the frame kernel forms an orientation on the MBF's
  positions in metres, checked as the kernel checks it (`FrameElement::orientation`, ported
  operation for operation): the element axis and the y_reference's part perpendicular to it
  each above 1e-12, then unit, mutually orthogonal and right-handed axes, each within 1e-12.
  A vector nearly parallel to its straight can fail only the last checks: along (1, 2, 0) m,
  (1, 2.00000001, 0) leaves y about 4e-8 from orthogonal to x (independent review VER-002
  r6). Any other recorded y_reference is refused by name with the element
  (`MBF-IMPORT-ORIENTATION-UNUSABLE`), and nothing is substituted (the lane manager's
  decision after independent review VER-002 r6). The
  writer's preflight refuses an export whose own bound re-import refuses so (§9.4), so a
  sidecar the writer wrote is always restored as written.
- **Legacy bound imports without a recorded y_reference**, and imports without a sidecar,
  take the documented local y below.
- **Otherwise** the straight takes the local y axis CAEPIPE documents for pipe elements
  (documented: `CAEPIPE-LOCAL-AXES`, technical reference *Local Coordinate System*). With
  global Y vertical, a non-vertical element's local y lies in the plane of its local x and
  global Y, toward +Y, so `y_reference` = Y − (Y·x)x normalized (computed as
  (−x_y·x_x, x_x² + x_z², −x_y·x_z) to keep precision near vertical). A vertical element's
  local y is global −X.
- **Selected interpretation.** The manual gives no angle for "vertical". This profile treats
  a straight as vertical when the horizontal part of its unit direction is at most 1e-6
  (`VERTICAL_SINE`).
- **Z vertical** (documented, same page). A non-vertical element's local z lies in the
  plane of its local x and global Z, toward +Z, and y = z × x. A vertical element's local y
  is global Y. After §5.1's rotation, the file's +Z is +y, so local z is the projected +y
  and y = z × x = (x_z, 0, −x_x) / |(x_x, x_z)|, a horizontal vector. A vertical element's
  local y, the file's global Y, is SWBPIPE's −z. The same `VERTICAL_SINE` applies to the
  rotated direction.
- For a vertical element the manual fixes y; z follows from x × y, so a downward element's
  z is the opposite of the one the manual names. This holds for both conventions.
- Each assignment is reported as `MBF-IMPORT-ORIENTATION-DEFAULTED` (info, by element).
- The documented y is checked the same way, on the positions in metres, whose direction can
  differ from the file's own values at extreme coordinates. One the kernel cannot use is
  refused (`MBF-IMPORT-ORIENTATION-UNUSABLE`): near 8.96e15 mm a 1 mm X step vanishes in m
  (spacing 2⁻⁹ m there), so a straight with a 1.08e-4 mm Y step has its axis in m nearly
  along the documented y. A straight no longer than 1e-12 m, which no
  orientation fits, is left to the solve's named refusal (§10.1).
  A circular section's stiffness does not depend on the orientation; only local-axis results
  (local forces, moments and stresses) depend on the convention.

## 11. Bends

The writer places a curved bend at its tangent intersection (TI) and writes
`T<n>,KI,B<R>` with the factor as comment `FF=<k>`, and a directional SIF pair as comment
`BSIF(<in-plane>,<out-of-plane>)`. CAEPIPE generates the near and far end
nodes of the bend's curved portion as `nA` and `nB` (documented: `CAEPIPE-BEND`, technical
reference *Bend*; *Node*). The inverse:

1. **The TI row.** A To row with key `KI` and a radius `B` is a TI node. Its offsets lead
   from the previous node P to the TI.
   - `B` without `KI` is refused.
   - `KI` without `B` is refused (`MBF-IMPORT-BEND-RADIUS-MISSING`). The manual's default
     is the long radius, which depends on nominal size, and that would need dimensional
     tables.
2. **The outgoing leg.** The next To row in the same run (node Q) gives the outgoing
   direction. The bend is refused (`MBF-IMPORT-BEND-UNSUPPORTED`, naming the reason) when:
   - the run ends at the TI;
   - a branch starts at the TI;
   - a Location row or any support or load comment is attached to the TI;
   - a support or load is attached to either tangent node, as the writer also refuses.
3. **Geometry.**
   - d_in = unit(TI − P) and d_out = unit(Q − TI); θ = acos(d_in·d_out).
   - Tangent length t = R·tan(θ/2). Near end N = TI − t·d_in; far end F = TI + t·d_out.
   - **Refused** (`MBF-IMPORT-BEND-GEOMETRY-INVALID`): θ within 1e-9 of 0 or of π;
     |TI − P| − t ≤ 1e-9·|TI − P|; or |Q − TI| − t ≤ 1e-9·|Q − TI|, unless Q is itself a TI
     whose own tangent length still leaves a positive straight between the two bends.
     These are the writer's relative tolerances (`_BEND_RELATIVE_TOLERANCE`).
   - **After rounding** (independent review VER-002 r2): R > 0 and θ > 0, so a tangent length
     that rounds to 0 is an underflow and is refused (`MBF-IMPORT-VALUE-UNREPRESENTABLE`).
     The generated geometry is checked on its binary64 values: the tangent nodes must differ
     from the TI and from each other, the TI must differ from the chord middle (the bend-plane
     direction), and the straights beside the bend must keep a non-zero length; otherwise
     `MBF-IMPORT-BEND-GEOMETRY-INVALID`.
   - **In metres.** Within a pressure region the bend is also formed as the solve forms it,
     from its tangent nodes and radius read to metres, and its end tangents must meet its
     neighbours within α_tan (§9.4, **Bends**).
4. **Members.**
   - Straight `pipe:P-nA`, from P to N.
   - Bend pipe `pipe:nA-nB`, from N to F, with `y_reference` = unit(TI − (N+F)/2). The
     writer requires exactly that: it points from the chord toward the TI in the bend
     plane.
   - Outgoing straight `pipe:nB-Q`.
   - The TI row's M, P and L apply to the incoming straight and the bend; the next row's
     apply to the outgoing straight. For the section this is documented (`CAEPIPE-BEND`).
     For M and L it is the same rule. The writer refuses differing properties within one
     row, so this is its inverse.
5. **Component** `component:bend-n`:
   - `kind: bend`, `node: node:nA`. The sidecar does not record which tangent node the
     source component named, so a bound import also uses the near tangent node and reports
     the component node as `MBF-IMPORT-ID-UNRECOVERABLE`; a source model whose component
     names the far node is excluded from canonical equality by name (§18.1);
   - `geometry.bend_pipe_ref`, `geometry.bend_radius` {R, `in`|`mm`} and
     `bend_geometry_source_reference` (MBF hash and line);
   - `mechanics_interface.solver_consumption: curved_bend_macro_element`;
   - `modifiers.flexibility_factor_user_value` from `FF`, with the dimensionless token the
     pilot cases use (`none`), and `modifiers.source_reference` naming the MBF line;
   - `modifiers.sif_in_plane_user_value` and `modifiers.sif_out_of_plane_user_value` from
     `BSIF`, read on a `KI` row only. The public format gives `BSIF` as the bend's SIFs, two
     values in parentheses, in-plane first (*Format of .MBF*, comment `BSIF`). Both values are
     required and must be positive. Otherwise the import refuses (`MBF-IMPORT-COMMENT-INVALID`,
     `MBF-IMPORT-VALUE-OUT-OF-RANGE`). The source reference then names comments `FF` and `BSIF`;
   - no `bend_angle`: geometry governs, and the writer only checks it when it is present.
6. **Bend without `FF`.** The `FF` comment is optional in the public format (*Format of
   .MBF*, comment `FF`). SWBPIPE never supplies a factor (the ComponentModifiers rule), so:
   - the bend is imported without `flexibility_factor_user_value`;
   - a `modifiers.source_reference` states that the MBF supplied none;
   - an `unresolved_assumptions` record (`kind: open_issue`, `affected_refs` the component)
     asks the user to enter a factor;
   - the loss report lists `MBF-IMPORT-BEND-FLEXIBILITY-MISSING` (warning, `omitted`);
   - the existing bend-modifier validation then reports the bend as incomplete.
7. **Refused bend data** (`MBF-IMPORT-BEND-UNSUPPORTED`, naming the keyword):
   intermediate nodes `IN1` and `IN2`, `BTHK`, `BMAT`, the miter code `KM` and `WS`, and
   jacketed bends (`JI`).

## 12. Supports

Support data are comments on the node's row or on a Location row. They follow *Format of
.MBF*, LAYOUT, key letter K code A and the comment list.

| MBF | Treatment |
|---|---|
| `KA`, anchor in global axes (`LA` absent or 0) | `support:n`, `family: anchor`, restraints UX…RZ |
| `FIXD(a,b,c)`, `F(a,b,c)`, `FIXDX[=1]`, `FX`, … | Translational restraints. Each value must be 0 or 1 (`MBF-IMPORT-COMMENT-INVALID`). They go on `support:n-restraint` with family absent (the solver's own inference, as for any family-less support) |
| `FIXR(…)`, `FIXRX[=1]`, … | Rotational restraints, on the same restraint support. A restraint with some but not all rotations and no anchor cannot be represented by the solve's families: a family-less support with fewer than six DOFs is a guide, which restrains translations only. It is refused (`MBF-IMPORT-SUPPORT-UNSUPPORTED`; independent review VER-002 r8, the lane manager's sign-off) |
| FIXD and FIXR covering all six DOF | Kept as written. The writer writes such a node as `KA` |
| `KA` with `TRAK(kx,ky,kz)` and `ROTK(0,0,0)`, a flexible anchor (tranche 3) | One `family: spring` support per translation with k > 0, `support:n-spring-ux` …, restraints [the DOF], `stiffness` {the DOF, k in `N/mm` or `lbf/in`}. A zero component leaves its DOF free (documented: `CAEPIPE-ANCHOR`); so does `ROTK(0,0,0)`, whatever its unit. TRAK and ROTK move with their axes under §5.1 (unsigned, as restraint flags) |
| Flexible-anchor refusals | `TRAK` or `ROTK` without `KA`, or a flexible anchor with another support at its node (`MBF-IMPORT-SUPPORT-UNSUPPORTED`); `KA` with only one of `TRAK` and `ROTK`, whose absent DOFs no source says are rigid or free, and a non-zero `ROTK` component, whose unit the public pages give two ways (`MBF-IMPORT-TBD-17-04-010`); a negative stiffness (`MBF-IMPORT-VALUE-OUT-OF-RANGE`); all six zero (restrains nothing, `MBF-IMPORT-SUPPORT-UNSUPPORTED`); `D`/`ROT` at a flexible anchor, which would move the springs' ground end (`MBF-IMPORT-DISPLACEMENT-UNSUPPORTED`); a repeated `TRAK` or `ROTK` (`MBF-IMPORT-COMMENT-INVALID`) |
| `KA` with `LA=1` (local axes), `FREE` | Refused (`MBF-IMPORT-SUPPORT-UNSUPPORTED`) |
| Guide `G`, rigid (tranche 3) | `support:n-guide`, `family: guide`, restraining the two global translations lateral to the guide's element: its local y and z (documented: `CAEPIPE-GUIDE`), read only when the element lies exactly along a global axis (its exact rational offsets have one non-zero component, after §5.1). The element is the preceding one (the element ending at `n`), else the one following element; a first node with several following elements is refused. `GGAP` and `MU` are read only as 0 or empty |
| Guide with `STIFF=k` | A spring on each lateral translation, k in `N/mm` or `lbf/in` (*Format of .MBF*, `STIFF`) |
| `STIFF=Rigid` on a guide or limit stop | Read exactly as the same row without `STIFF`: a rigid guide or line stop. The technical reference documents rigid as the default stiffness of both (`CAEPIPE-GUIDE`: "The default stiffness is rigid which is input by typing “r” or “Rigid”"; `CAEPIPE-LIMIT-STOP`: "The default is set to Rigid stiffness"). *Format of .MBF* shows only numeric `STIFF` values; the spelling `STIFF=Rigid` in an MBF file is observed format behaviour. Only `Rigid` is read; any other non-numeric value is `MBF-IMPORT-NUMBER-INVALID`, and the data-comment rules below apply unchanged |
| A `None` limit of `LS`, as in `LS(0.000,None)` | A blank limit: the technical reference shows a blank limit as `None` and documents that with no limit on one side the node "can move freely on that side" (`CAEPIPE-LIMIT-STOP`), so the stop is one-way. It is refused by name as a one-way restraint (`MBF-IMPORT-SUPPORT-UNSUPPORTED`, below), as an empty limit is. *Format of .MBF* documents only numeric limits; the spelling `None` in an MBF file is observed format behaviour. Only `None` is read so; any other non-numeric limit is `MBF-IMPORT-NUMBER-INVALID` |
| Limit stop `LS(0,0)` with `DV` along one global axis (tranche 3) | `support:n-stop-a`, `family: line_stop`, restraining that translation: zero limits stop the node at its undeflected position on both sides (documented: `CAEPIPE-LIMIT-STOP`), a two-way rigid restraint. `DV`'s sign and length do not matter; it rotates under §5.1. With `STIFF=k`, a spring on that DOF. `MU` is read only as 0 or empty |
| Skewed restraint `K=k` with `DV` along one global axis (tranche 3) | A spring on that translation (documented: `CAEPIPE-SKEWED-RESTRAINT`, a two-way support along its direction; *Format of .MBF*: `K` in lb/in or N/mm) |
| Guide, limit-stop and skewed-restraint refusals | A non-zero `GGAP` (a symmetric clearance) or `MU` (friction), and an `LS` limit that is non-zero (a gap) or blank (a one-way restraint): each nonlinear, `MBF-IMPORT-SUPPORT-UNSUPPORTED`; a guide whose element is not along a global axis, and a `DV` off the global axes (skewed directions), `MBF-IMPORT-SUPPORT-UNSUPPORTED`; `LS` without `DV`, and `AXIAL`, `SHEARY`, `SHEARZ` (which element's local axes a limit stop takes is not documented) and `KR` (unit given two ways), `MBF-IMPORT-TBD-17-04-010`; `K` without `DV`, `CNOD` (connected node), more than one of `G`, `LS`, `K` on one row, data comments (`GGAP`, `MU`, `STIFF`, `DV`) without their support type, a second guide or limit stop on one axis at a node, two springs on one DOF, and a spring on a rigidly restrained DOF, `MBF-IMPORT-SUPPORT-UNSUPPORTED`; `STIFF` or `K` not positive, `MU` negative, `MBF-IMPORT-VALUE-OUT-OF-RANGE`; `LS` without two limits, `G` with a value, `DV(0,0,0)`, `K(…)`, a repeated comment on one row, `MBF-IMPORT-COMMENT-INVALID`; a stiffness whose reading in N/m overflows, `MBF-IMPORT-VALUE-UNREPRESENTABLE` |
| Hangers `VS`, `CS`, `US`, `HTYP`, `SR`, `MLV`, `CLD` (tranche 3: kept refused) | Refused (`MBF-IMPORT-SUPPORT-UNSUPPORTED`, naming the keyword). `VS` and `CS` are designed by the target's hanger design. A hanger's load acts as a force in the sustained and operating cases, hot or cold load by option, and its spring rate enters the stiffness only by option (documented: `CAEPIPE-HANGER`). SWBPIPE's constant-effort support applies its load in every solved case, the thermal case included, and its spring hanger's loads are not consumed. No exact mapping exists (§20.3, gaps) |
| Snubbers; generic support `KG`, `GR*`; nozzles `NOZZLE` and its fields; `CNOD` | Refused (`MBF-IMPORT-SUPPORT-UNSUPPORTED`, naming the keyword) |
| `TAG=<text>` | Becomes the `label` of the node's support. On a node without a support it is listed as `omitted` |
| `LTAG` (spectrum level) | Empty is accepted (§4.4); non-empty is refused |

**The solve's support rules** (product_physics `rigid_linear_support_from_preview`,
linear_supports `prepare_rigid_support`), ported and applied to every imported model, bound
or not (independent review VER-002 r8): a support without family is an anchor with six DOFs
and a guide otherwise; an anchor restrains any DOF, a guide or line stop translations only,
a vertical support UZ only; a support restrains at least one DOF; and no DOF is restrained
twice at a node. A spring (`family: spring`, `prepare_spring`) restrains nothing and acts on
the one DOF its stiffness names; the solve takes that DOF and needs a positive, finite
stiffness in N/m (`SupportQuantity::positive`, else `SUPPORT_INPUT_INVALID`). The importer gives
each spring exactly that DOF as its restraint list, a positive stiffness, and refuses one whose
value × the catalog factor (N/mm 1000; lbf/in 4.4482216152605 / 0.0254, in binary64) is not
finite. The exact pressure route admits linear restraints and springs (`exact_admission`
`support_family`), so a spring needs no pressure guard. A stiffness negligible against the
assembled stiffness is refused by the solve (`NUMERICAL_INTEGRITY_ASSEMBLY_UNRESOLVED`, §9.4 guard
map), not by the import; a stiffness outside the solve's checked-JSON domain (1e20 N/mm, an integral
float beyond 2⁵³ − 1) is refused by the import, as every such number is (§9.4). A support that breaks one is refused by its id
(`MBF-IMPORT-SUPPORT-UNSUPPORTED`), as the solve refuses it (`SUPPORT_INPUT_INVALID`): for
example `KA` with `FIXD` at one node restrains a translation twice.

With a bound sidecar, the writer's `support_map` restores the original supports, their ids
and DOF sets. That map preserves each split explicitly: the writer may have merged several
supports into one anchor, and the sidecar splits them back where the solve admits each part
(a part with rotations and fewer than six DOFs it does not; the sidecar parser refuses it).
The writer records each support's `family` in `support_map` (implemented), and bound imports
restore it. A sidecar entry without `family` (an older sidecar) is reported as unrecoverable
(§15).

**Writer counterpart (tranche 3).** SWBPIPE's `guide` and `line_stop` families are already
written as `FIXD` with the family in `support_map`, which an import reads exactly; the writer
does not write `G` or `LS`. A `family: spring` support is written as a flexible anchor on the
node's row, `KA,TRAK(kx,ky,kz),ROTK(0,0,0)`, in N/mm: N/m × 10⁻³, kN/m × 1, kN/mm × 10³,
lbf/in × 4.4482216152605 / 25.4, each the exact rational product rounded once (listed in
`MBF-SI-UNITS-NORMALIZED`); a DOF without a spring writes 0. The writer refuses
(`MBF-SI-SUPPORT-UNSUPPORTED`) a rotational spring (ROTK's unit is open, TBD-17-04-010), a spring
whose restraints and stiffness do not name the same single DOF, a second spring on one DOF, and
a node with springs and rigid restraints (the importer reads a flexible anchor alone); a
stiffness that is not positive is `MBF-SI-VALUE-OUT-OF-RANGE`, an imposed displacement on a
spring `MBF-SI-IMPOSED-DISPLACEMENT-UNSUPPORTED`. `support_map` records each spring with its
DOF, `family: spring` and `family_authored: true`. The export preflight of §9.4, the writer's
own bound re-import, runs for every export (§14), so a stiffness whose printed value
the import refuses (1e306 N/mm, which reads as 1e309 N/m) is refused as
`MBF-SI-PREFLIGHT-REFUSED`.

## 13. Point loads and displacements

| MBF comment | Tranche 1 |
|---|---|
| `FOR(fx,fy,fz)`, `FORX=` … | One `concentrated_force` per non-zero component (`global_x/y/z`, `N` or `lbf`) in the mechanical case. `FLC=1` places it in the sustained case (documented, *Format of .MBF*, comment `FLC`). Without `FLC`: bound imports apply the writer's convention (sustained case); unbound imports refuse it (`MBF-IMPORT-TBD-17-04-007`), except where an explicitly retained bounded profile decision admits it. Any other explicit `FLC` is refused (`MBF-IMPORT-FORCE-CASE-UNSUPPORTED`) |
| `MOM(mx,my,mz)`, `MOMX=` … | One `concentrated_moment` per non-zero component (`N*m` or `lbf*ft`), with the same case rule as `FOR` |
| `D(…)`, `DIS(…)`, `DX=`, `DISX=` … at an anchor | One `imposed_displacement` per non-zero component on the anchor support (`target.type: support`, `dof` UX/UY/UZ, `mm` or `in`) in the thermal case. The manual says a zero entry is not a specified displacement, so zeros give nothing. This is the writer's inverse and the W-16 slice a support target |
| `ROT(…)`, `ROTX=` … at an anchor | The same for RX/RY/RZ, in `deg`, or `rad` under `RAD` |
| `D`/`ROT` at a non-anchor; `LD=1`; `DIS2…DIS10`, `ROT2…ROT10`, `DISD`, `ROTD`, `SETT`, `STRO`, `SEIS*`, `SERO*`, `WIND1-4`, `WINR1-4` | Refused (`MBF-IMPORT-DISPLACEMENT-UNSUPPORTED`) |
| `CWGT`, `U`/`UNIF`, `T`/`TEMP`, `P`/`PRES`, `SG`, `AMB`, `HSG`, `HPRES`, `FRE`, `PH` (harmonic) | Refused (`MBF-IMPORT-LOAD-UNSUPPORTED`). Concentrated weight and uniform load are tranche 2 |

Forces and moments on one node are summed per component, as the writer sums them.

## 14. Everything else

**The sidecar trust model** (Agent 0's decision under the owner's delegation, after
independent review VER-002 r13), stated as it is enforced. On an exact-digest bound import,
the guard verifies that the sidecar is the writer's record of the model it yields. The
sidecar is used only when it equals what the writer regenerates from the imported model,
after canonicalisation. Any value the writer would not reproduce counts as a difference and
is refused by name at its JSON path. The guard proves consistency, not authorship. An edit
that the writer would reproduce for the model it yields cannot be told apart from writer
output, and imports as such. Such consistent edits import as stated, with nothing defaulted
or misread. Authorship is not claimed. The sidecar's self-digest (§15) adds a check in
front of this guard. It catches an accidental or partial edit. It does not replace the guard
or bear on authorship.

**The regeneration identity** is the exact-digest route's guard of record. After assembly,
a bound import of a format-2 or format-3 sidecar has the writer regenerate the sidecar from the imported
model (`regenerate_caepipe_sidecar`). No MBF is written and the writer's export preflight
is not run.

**The writer's preflight is total** (tranche 3, independent review VER-002 r3). Every export
other than the regeneration route's, whose caller is the import, is checked by its own bound
re-import (§9.4), whatever it holds: pressure or none, springs or none. Any refusal of that
re-import is a named export refusal carrying the importer's code and location
(`MBF-SI-PREFLIGHT-REFUSED`, or `MBF-SI-PRESSURE-REGION-FORMAT` for a region check). It adds a
check only: an export it admits is the text it would have written without it. Printing and unit
normalization can each break a value that the model holds validly. For example, a force of
2⁵³ − 1 N prints to 10 significant digits as 9.007199255e+15, and 1e14 kN is written as 1e17 N.
Each is an integral float beyond 2⁵³ − 1, which the solve's checked-JSON reader refuses (§9.4
guard map), so the export is refused. The one input beyond the model is the import's record of which straights'
`y_reference` it derived (`id_map` `y_reference_derived`), so a derived orientation
regenerates as derived. The two sidecars are compared without the digests (`mbf_sha256`
and the self-digest `sidecar_sha256`), in the given sidecar's format, with key order
immaterial, every number as a binary64 (1 and 1.0 are one value) and each load direction
as the axis it names (`global_y`, `GLOBAL_Y`, `UY` and `uy` alike). A difference is refused
as `MBF-IMPORT-SIDECAR-NOT-REPRODUCED`, naming the first differing JSON path in sorted key
order with both values. A model the writer does not write is refused the same way at
`sidecar`, naming the writer's codes. Only one thing is set aside: the identity of a
primitive the MBF sums, which no regeneration can reproduce (REQ-016; reported by name as
`MBF-IMPORT-ID-UNRECOVERABLE`, §15) (VER-002 r14). Each sidecar entry this import summed
into the primitive (`id_map` `load_map[<formed id>].summed_entries`) is compared, field
by field, with the primitive's regenerated entry under its formed id. These fields are
the case, category, target and its kind, and direction; only the keys differ. A summed
entry that disagrees on any of them is refused by name. Restored and formed primitive ids
must be unique across cases before the model is assembled, or the import is refused by
name (`MBF-IMPORT-SIDECAR-CONFLICT` at `load_map[<case>/<id>]`), so the solve's
`DUPLICATE_ID` is unreachable. A test holds each set-aside identity to its report, and each
model to no other difference. The checks that run first are supporting checks and
give the narrower code: the strict parser, completeness, the ambiguity check (§15), the
no-defaults rule (every effective value comes from an explicit format-2 record or a
documented legacy omission, never from a default filling a missing or null field) and the
agreement rule (a recorded value the MBF or a derivation also determines must agree with
it; neither side is silently preferred, VER-002 r12). The seeded mutation test (§15) shows
that each mutation it makes is refused, and would be refused by the identity alone, except
the consistent edits below. A legacy
sidecar, which the writer no longer writes, is closed instead by the agreement of every
MBF-determined field it retains (§15).

**Consistent edits.** The digest binds only the MBF, and the MBF has no carrier for the
sidecar (TBD-17-01-003, TBD-17-04-003; O-6). The self-digest binds the sidecar only to
itself, and an editor can recompute it (§15). Some values pass from the model to the sidecar
unchanged: ids, `reversed`, an authored `y_reference` or `family`, a recorded `origin`, a
thermal direction, the shear model of a contract, region ids, and closure transfers with
their provenance. Such a value regenerates as the sidecar gives it, so an edit of one imports
as stated. Removing one of two primitives the MBF sums is such an edit too: the other then
names the file's primitive alone. A user who edited a
sidecar's identities, or whose sidecar the exact-digest route refuses, asks for the rebind
route instead (`rebind_by_node_number`, REQ-016, §15 **Unbound**). It is taken only on that
explicit request, whatever the digest. It applies the sidecar by CAEPIPE name, reports every
restored identity as unconfirmed (`rebound_unconfirmed`), and keeps its explicit checks; it
does not run the identity.

**The rule.** A record that changes the mechanics model or its loads, and that SWBPIPE
cannot represent exactly, is **refused** by name and blocks the import. A record that only
configures CAEPIPE's own checks, reports or dynamic analyses, with no counterpart in the
SWBPIPE model, is **listed** as `delegated` or `omitted` (warning) and does not block. When
the manual does not make clear which kind a record is, it is refused.

| Category | Disposition |
|---|---|
| Joint codes `J…` (ball, cut pipe, reducer, expansion joint, jacket, elastic element, beam, rigid, slip, tie rod, valve) and their comments (`WGT`, `VWGT`, `AWGT`, `ES`, `TA`, `OD1/OD2`, `THK1/THK2`, `CONE`, `FLANGE`, …) | Refused (`MBF-IMPORT-ROW-KEY-UNSUPPORTED`, naming the code). Rigid and valve (DEC-045 rigid components) and reducers (0.8.0, §19) are tranche 2 |
| K codes other than `A` and `I` (branch connections, tees, olets, hinge, miter, generic support) | Refused. Branch codes set SIFs and may set branch flexibility, so they are not mechanically inert |
| Row key `H` (hydrotest) | Refused |
| SIF and stress-index comments (`SIF`, `SIFA`, `SIFT`, `SIA`, `SII`, `SIO`, `SIT`, `SIFB`, IGEM SCF set, `R`, `PAD`, `CRTCH`, `THK`, `LEN`, `MM`, `WTYPE`, `TJOINT`) | Refused in tranche 1. SWBPIPE SIF user values are tranche 2 |
| SOILS, BMATERIALS, BSECTIONS, BLOADS, PUMPS, COMPRESSORS, TURBINES, ACHXS, FIHTR | Refused (`MBF-IMPORT-SECTION-UNSUPPORTED`) |
| SEISMIC, WIND (and ASCES, ASCEW, 1991W, WIND1–4, SNOWICE, WAVE1–4) | Accepted only when every load value is zero, listed as `omitted`; otherwise refused. Static equivalents are tranche 2 |
| FCURVE, FCYCLE, ALLOWABLES, SPCMS, SPLVL, PSDDT/PSDLS, PSDLD | `delegated` (fatigue, allowable loads, spectra and random-vibration definitions). The section name and record count are listed; values are not copied |
| Unknown section keyword or unknown comment | Refused (`MBF-IMPORT-SECTION-UNKNOWN`, `MBF-IMPORT-COMMENT-UNKNOWN`) |

**Codes named in the implementation beyond those above.** Refusals (block the import):

- `MBF-IMPORT-LINE-END-UNSUPPORTED`, `MBF-IMPORT-CHARACTER-UNSUPPORTED` (§4.1);
- `MBF-IMPORT-SECTION-MISSING` (§4.2);
- `MBF-IMPORT-FIELD-MISSING`, `MBF-IMPORT-RECORD-REPEATED`, `MBF-IMPORT-VALUE-OUT-OF-RANGE` (§4.3);
- `MBF-IMPORT-ROW-SYNTAX-INVALID`, `MBF-IMPORT-TOPOLOGY-EMPTY` (§4.4);
- `MBF-IMPORT-COMMENT-UNSUPPORTED`: a documented comment refused without a narrower code,
  such as the SIF and stress-index comments above;
- `MBF-IMPORT-SIDECAR-NOT-REPRODUCED`: a format-2 bound sidecar that is not what the writer
  regenerates from the imported model, at the first differing JSON path (above);
- `MBF-IMPORT-SIDECAR-AMBIGUOUS`: a sidecar whose reverse mapping is not a function,
  refused before assembly (§15);
- `MBF-IMPORT-SIDECAR-INVALID`: a sidecar the parser refuses, by JSON path, or rebinding
  requested without one (§15). The sidecar format decides what is complete: a format-2
  sidecar (`sidecar_format` 2) missing any field is refused, and a legacy sidecar (none, from
  main's tranche-1 writer) carrying a field that writer never wrote, such as `pressure_map`;
- `MBF-IMPORT-SHEAR-MODEL-INVALID`: a `shear_model` argument outside the contract's two
  values (§9.4);
- `MBF-IMPORT-VALUE-UNREPRESENTABLE`: a read or derived value outside the binary64 range, or a
  non-zero value that would become 0 (§4.3), a spring stiffness that is not finite in N/m (§12),
  and any emitted number outside the solve's checked-JSON domain, at its JSON path (§9.4);
- tranche 3: `MBF-IMPORT-TBD-17-04-010` (§12, §20.1): a limit stop without `DV`, `AXIAL`, `SHEARY`,
  `SHEARZ`, `KR`, a non-zero `ROTK`, and a flexible anchor with only one of `TRAK` and `ROTK`.
  `MBF-IMPORT-SUPPORT-UNSUPPORTED` no longer refuses `G`, `LS`, `K`, `TRAK`, `ROTK`, `DV`, `STIFF`,
  `GGAP` and `MU` as such; it refuses the cases §12 lists;
- tranche 2: `MBF-IMPORT-PRESSURE-CLOSURE-REQUIRED`, `MBF-IMPORT-PRESSURE-CLOSURE-INVALID`,
  `MBF-IMPORT-PRESSURE-DISPLACEMENT-UNSUPPORTED`, `MBF-IMPORT-PRESSURE-RUN-NONCOLLINEAR`,
  `MBF-IMPORT-PRESSURE-BORE-MISMATCH`, `MBF-IMPORT-PRESSURE-INTERNAL-BRANCH`,
  `MBF-IMPORT-PRESSURE-ID-COLLISION`, `MBF-IMPORT-PRESSURE-MITRE`,
  `MBF-IMPORT-PRESSURE-GEOMETRY-INVALID` (§9.4), `MBF-IMPORT-ORIENTATION-UNUSABLE` (§10.5)
  and `MBF-IMPORT-CORROSION-ALLOWANCE-MISSING` (§8). `MBF-IMPORT-GEOMETRY-ZERO-LENGTH` also
  refuses an element whose ends coincide in metres, the unit the solve reads (VER-002 r2). `MBF-IMPORT-PRESSURE-UNSUPPORTED`
  now refuses only a negative Pressure 1.

**Removed in tranche 2.** OPTIONS `Z` no longer refuses (`MBF-IMPORT-OPTION-UNSUPPORTED`
stays for unknown tokens; §5.1), a positive Pressure 1 no longer refuses (§9.4), and the
writer no longer refuses the exact pressure contract or its regions
(`MBF-SI-PRESSURE-UNSUPPORTED` stays for the cases §9.4 lists and for pressure primitives).
The writer's new codes are `MBF-SI-PRESSURE-CASE-UNSUPPORTED`, `MBF-SI-CORROSION-ALLOWANCE-MISSING`,
`MBF-SI-PRESSURE-REGION-FORMAT` and `MBF-SI-PREFLIGHT-REFUSED` (refusals),
`MBF-SI-PRESSURE-REGION-ZERO` and `MBF-SI-SHEAR-MODEL-NOT-EXPORTED` (omitted).

The writer refuses the corresponding cases of its own conversions as
`MBF-SI-VALUE-OUT-OF-RANGE` (§5.2, §18.2), and the same code for every value it derives
before printing that is not finite: the material temperature table and each operating
temperature (reference plus thermal change), layout offsets, the tangent intersection and
its tangent lengths, and the bare-metal area behind a weight density. Its norms are scaled
(`hypot`) as the importer's are.

Loss entries and information (do not block):

- `MBF-IMPORT-TARGET-OPTION-DELEGATED` and `MBF-IMPORT-SECTION-DELEGATED` (`delegated`, §6, §14);
- `MBF-IMPORT-SECTION-ZERO-OMITTED`,
  `MBF-IMPORT-RECORD-UNUSED` (a record no element references), `MBF-IMPORT-TAG-WITHOUT-SUPPORT`
  and `MBF-IMPORT-SIDECAR-ENTRY-ABSENT` (`omitted`);
- `MBF-IMPORT-DIRECTION-UNRECOVERABLE` (`approximated`, §15: a legacy thermal entry
  without a direction);
- `MBF-IMPORT-SIDECAR-NOT-APPLIED` (info: the hashes differ and no rebinding was requested)
  and `MBF-IMPORT-ORIENTATION-DEFAULTED` (info, §10.5);
- `MBF-IMPORT-LOAD-SLOT-INACTIVE` (info, by record and slot, §9.1);
- `MBF-IMPORT-PRESSURE-EFFECTS-OMITTED` (`omitted`, §9.4).

Further refusal: `MBF-IMPORT-GRAVITY-UNSUPPORTED`, an OPMISC gravity setting other than the
default earth setting (§6).

## 15. Ids and the sidecar map

There is no id carrier inside the MBF (TBD-17-01-003, TBD-17-04-003). Identity therefore
travels in a sidecar.

**Input.** The writer's sidecar (`MbfSiResult.sidecar`): `sidecar_format`, the maps
(`node_map`, `element_map`, `support_map`, `load_case_map`, `material_map`, `section_map`,
`load_map`, `pressure_map`), `mbf_sha256` and the self-digest `sidecar_sha256`, as
`run_pilot.py` writes it to `caepipe_in/id_map.json`. Main's earliest pilot runs kept the
digest in `mbf_render.json` beside a legacy sidecar. For that layout only, the file loader
takes the companion's digest (*Loading*, below).

**Sidecar format** (independent review VER-002 r10 to r12). The current writer records
`sidecar_format` = 3, which is format 2 with its self-digest (below). A format-2 sidecar
holds what a bound import restores and the MBF does
not carry, every field with its effective value; it never uses null, or another sentinel, as
a value a default then fills, and it does not repeat what the MBF determines (layout order,
record names, material and section numbers, bend geometry, CAEPIPE kinds and names; no
`section_map`). Its fields:

- `node_map`: id → CAEPIPE name;
- `element_map`: id → `from`, `to`, `reversed`, and for a straight its effective
  `y_reference` (the authored vector, or else CAEPIPE's documented local y, derived from the
  written MBF's own positions as an import derives it) with `y_reference_authored`, for a
  bend `bend` with its `component` id;
- `support_map`: id → `node`, `restrained`, the effective `family` (the authored one, or the
  solve's inference: anchor with six DOFs, else guide) and `family_authored`. Format 3 also
  records a spring (tranche 3, §12): one translation, `family: spring`, `family_authored` true.
  Its stiffness is in the MBF (`TRAK`) and is not repeated; no new field is added;
- `load_case_map`: id → label; `material_map`: name → `swbpipe_material` id and `origin`
  (one of `MATERIAL_ORIGINS`, or `"undeclared"`, when the import needs the caller's
  declaration);
- `load_map`: `<case>/<load id>` → `load_case`, `category`, `target` and the effective
  `direction` (the authored one, or the token the import forms: `global_x` thermal,
  `global_y` weight);
- `pressure_map`: `contract` (the exact contract's version and mode, or `"none"`), the
  effective `shear_model` (the contract's, or the route's `euler_bernoulli` without a
  contract) and `regions` (each `id`, `members`, two `terminals`; none without a contract).

The parser refuses a format-2 or format-3 sidecar missing any field, with null in any, or with a field
outside this list, by JSON path (`MBF-IMPORT-SIDECAR-INVALID`). Each recorded value the MBF
or a derivation also determines must agree with it, or the import is refused
(`MBF-IMPORT-SIDECAR-CONFLICT`, or `-INVALID` for the parser's own checks): the CAEPIPE
names and nodes of elements and supports, each node's restrained DOFs, the cases' labels,
each primitive's category, target and direction (a weight's is the gravity axis, −Y in model
axes; a force's or moment's the comment's axis; a displacement's the moved DOF; a thermal
direction, which the MBF does not carry, is authored), the regions' members and terminal
nodes, the file's pressure (it needs a contract; a pressure state without a contract is
Euler–Bernoulli with no region), an unauthored `y_reference` (exactly the documented local y
on the MBF's positions, after Z rotation) and an unauthored `family` (the solve's inference).
`*_authored` false means "derived, and equal to the derivation". The values the writer
copies from the model unchanged are restored as recorded, and regenerate as recorded (§14):
`reversed`, an authored `y_reference` or `family`, the ids, `origin`, a thermal direction,
the shear model of a contract, closure transfers and their provenance. A bound import restores
each value (an unauthored family as absent, which the solve infers to the same family). The
entry's variant must be the element's: a bend record for a bend, an orientation for a
straight (`MBF-IMPORT-SIDECAR-CONFLICT` at `element_map[<id>].bend`, on both routes; VER-002
r13).

**The self-digest** (Agent 0, under the owner's delegation, after tranche 2). A format-3
sidecar is a format-2 sidecar plus one field, `sidecar_sha256`. Its value is the lowercase
hex SHA-256 of the sidecar's canonical serialisation, taken without that field.

*Why a new format and not an optional field in format 2.* With a new format number, each
format's parser rule stays a fixed list in which every field is required. An optional digest
inside format 2 could not tell a sidecar written before the digest from one whose digest was
removed. The downgrade refusal below would then be impossible. Every format-2 sidecar stays
readable, unchanged.

*Canonical serialisation* (`si_writer.sidecar_canonical_bytes`). The serialisation is RFC 8785
(JSON Canonicalization Scheme), applied to the sidecar object without `sidecar_sha256`:

- **Objects.** Members are sorted by key, with keys compared as sequences of UTF-16 code units.
  There is no whitespace anywhere. A `,` separates members and elements, and a `:` separates
  a key from its value.
- **Strings.** `"` and `\` are escaped as `\"` and `\\`. U+0008, U+0009, U+000A, U+000C
  and U+000D become `\b`, `\t`, `\n`, `\f` and `\r`. Every other code point below U+0020
  becomes `\u00xx` in lowercase hex. All other characters are written literally, U+007F and
  non-ASCII included.
- **Numbers.** Each number is the binary64 value it denotes, so 1 and 1.0 are one value and
  −0 is 0. It is written as ECMAScript's Number::toString writes it: the fewest significant
  digits that read back as the value. The form is positional for values from 10⁻⁶ up to,
  but not including, 10²¹ (`0.000001`, `150000000000000000000`), and exponential otherwise
  (`1e-7`, `1e+21`).
- **Literals.** `true`, `false` and `null` are written as they are.
- **Encoding.** UTF-8, with no byte-order mark and no trailing newline.

Some values have no such form: a non-finite number, an integer that is not exactly a
binary64, a lone surrogate, a key that is not a string, and anything that is not a JSON
value. Each is refused by JSON path (`MBF-IMPORT-SIDECAR-INVALID`).

The self-digest does not reuse an existing canonical form, because neither is a byte-exact
definition that another implementation could reproduce. The regeneration identity's
canonicalisation (§14) compares parsed values; it is not a byte serialisation. The
importer's `canonical_bytes` is Python's `json.dumps`, which has its own number text (`1.0`
against `1`, `1e-07`). The tests carry an independent implementation and a digest derived
by hand from a small invented sidecar.

*Loading* (independent review VER-002, self-digest r1). The sidecar is verified exactly as
supplied. Nothing is added, merged or normalised before the check.

- **Parsing.** Where the importer parses JSON itself, it parses strictly
  (`parse_sidecar_json`), in every format. This covers the file loader (`load_sidecar`, the
  command line, the private regression of §18.3) and sidecar text or bytes passed to
  `import_caepipe_mbf`. Bytes must be strict UTF-8, and the text must be RFC 8259 JSON:
  NaN, Infinity and -Infinity are refused. A member name repeated within one object is
  refused at its JSON path (`MBF-IMPORT-SIDECAR-INVALID`, for example `node_map[N0]`).
  Python's `json.loads` would otherwise keep the last value silently and so hide the
  conflict. RFC 8785 requires unique names. A caller that passes `import_caepipe_mbf` an
  already-parsed object has resolved its own parse, and the importer reads that object as
  given.
- **Companion digest.** `load_sidecar` returns the file's bytes unchanged, for the importer
  to parse. There is one exception, which applies only to legacy sidecars, the format that
  relied on it: a legacy sidecar (no `sidecar_format`) without `mbf_sha256` takes the digest
  of the `mbf_render.json` beside it, and that companion is parsed strictly too. A format-2
  or format-3 sidecar always recorded its own `mbf_sha256` and never takes the companion's.
  Deleting it is a partial edit: format 3 refuses it by its self-digest, and format 2 by the
  parser.
- **Sweep.** These steps changed content before the check, and each is now closed:
  - the companion backfill, which applied to any sidecar without `mbf_sha256`;
  - `json.loads` keeping the last of repeated members;
  - `json.loads` admitting NaN and Infinity;
  - the private regression's own lenient `json.loads` of `id_map.json`, which now uses
    `load_sidecar`.

  `import_caepipe_mbf` neither copies nor changes a sidecar before the check. A number's
  spelling becomes its binary64 value, which is the canonical form's own value model, not a
  change of content.

*Import.* The self-digest is checked before the parser or any route reads the sidecar. The
check runs on every route: the exact-digest route, the explicit rebind, and the route that
leaves a sidecar of another MBF unapplied. A sidecar claims format 3 when `sidecar_format`
is the integer 3. Such a sidecar is refused, and nothing of it is applied, in three cases:

- **Digest missing.** Refused as `MBF-IMPORT-SIDECAR-INVALID` at `sidecar_sha256`. A removed
  digest is not read as format 2.
- **Digest malformed.** A value that is not a lowercase SHA-256 hex digest is refused at
  `sidecar_sha256` (`-INVALID`).
- **Digest wrong.** A digest that is not the content's is refused as
  `MBF-IMPORT-SIDECAR-DIGEST-MISMATCH` at `sidecar_sha256`.

A verified import reports `source.sidecar_self_digest` = `verified`. Two kinds of sidecar
carry no self-digest: a format-2 sidecar, written before the self-digest existed, and a
legacy sidecar. Each imports exactly as before, and the import says so by name
(`MBF-IMPORT-SIDECAR-NO-SELF-DIGEST`, info; `source.sidecar_self_digest` = `absent`).
Only format 3 has `sidecar_sha256`: a format-2 or legacy sidecar that carries it is refused
by path. The writer's preflight (§9.4) re-imports the writer's own output with its
self-digest.

*What it proves and what it does not.* The self-digest detects accidental or partial edits:
a changed byte, a dropped map, or a sidecar paired with another sidecar's digest. It is not
a signature. A deliberate editor can recompute it, or remove it and claim format 2. Meaning
is therefore still governed by the regeneration identity and the agreement rule (§14),
over exactly the values they governed before. Authorship is still not claimed. Key order,
whitespace and the JSON spelling of a number are not content. The sidecar reaches the
importer as a parsed JSON object, and the canonical form sorts its members and writes the
value of each number.

**Legacy sidecars.** A sidecar without `sidecar_format` is legacy: written by main's
tranche-1 writer (`si_writer.py` up to 42a86df92f) through `run_pilot.py`. That writer
recorded the seven maps other than `pressure_map` and the digest. Its element entries carry
`from`, `to`, `layout_order`, `layout_row`, `reversed`, `material`, `section`, `load`, a
`y_reference` only for a straight with an authored one, and a `bend` record; its supports
`node`, `caepipe_kind`, `restrained` and `family`; its materials the writer's numbers,
`temperatures_c`, `elements` and `origin` only where declared; its primitives `load_case`,
`caepipe_load_case`, `category`, `direction`, `target`, `mbf` and the CAEPIPE node or
element. Its earlier versions omitted `family`, `origin`, `direction` and `y_reference`, and
kept the digest in `mbf_render.json`. Every other field that writer always recorded is
required of a legacy sidecar: the seven maps, both section dimensions and `elements`, each
element's layout and record names, each primitive's `mbf`, and the rest above. The legacy
behaviour applies only to such sidecars and only to what that writer could produce:
it exported no pressure, so a legacy bound import is pressure-free (Euler–Bernoulli, as
every pressure-free document) and a legacy sidecar of a file with Pressure 1 is refused; a
missing `y_reference` takes the documented local y, and a missing `family` is reported
unrecoverable. A legacy sidecar carrying a field main's writer never wrote, such as
`pressure_map` or a null `y_reference`, is refused by path. A sidecar produced by main's
writer at 42a86df92f is kept as an invented fixture in the round-trip tests and imports
bound.

**A legacy sidecar is closed like format 2** (independent review VER-002 r15). The
regeneration identity does not apply to it: the writer no longer writes it. Instead, every
value it retains that the MBF also determines must agree with the MBF, or the import is
refused by field and JSON path (`MBF-IMPORT-SIDECAR-CONFLICT`, or `-INVALID` where the
parser compares one repetition with another). Main's writer wrote only SI, Y-vertical
files, so a legacy sidecar of an English or Z-vertical file is refused. Numbers are
compared within the writer's printing bound. The writer prints ten significant digits,
correctly rounded, with trailing zeros dropped (`_num`, `.10g`; "4000" stands for
4000.000000). A recorded value therefore lies within half a unit in the tenth significant
digit of its printed value (`tenth_digit`). A derived value is allowed the bound its
printed inputs give it. The TI is allowed the sum of those bounds over the offsets that
place it, from its run's From row, and the bend angle the angle that those position bounds
subtend on each leg (asin(e/L)), plus binary64 slack. A mutation test sets every
MBF-determined field of the fixture and of a weighted-cantilever legacy sidecar to a
different valid value: each number just outside its bound (×1.01; 10⁻⁶ relative for a
derived one) is refused, and just inside (×0.99; four binary64 steps) imports the same
model. The fixture is the positive control. Every field main's writer recorded:

| Map | MBF-determined (checked) | Identity (restored) | Authored (restored) |
|---|---|---|---|
| `node_map` | the CAEPIPE name | the id | |
| `element_map` | `from`, `to`, `layout_order`, `layout_row`, `material`, `section`, `load`; for a bend `radius_mm`, `angle_deg`, `flexibility_factor`, `tangent_intersection_mm`, `caepipe_row_node` | the id; `bend.component` | `reversed`, `y_reference` (frame-checked, §10.5) |
| `support_map` | `node`, `caepipe_kind`, `restrained` (their union per node; the split is restored) | the id | `family` (held to the solve's rules) |
| `load_case_map` | the label (the set is the file's cases) | the id | |
| `material_map` | the name, `weight_density_kgf_per_m3`, `poisson_ratio`, `elastic_modulus_n_per_mm2` and `thermal_expansion_mm_per_mm_per_c` (against every temperature row), `temperatures_c` (in order), `elements` (in layout order) | `swbpipe_material` | `origin` |
| `section_map` | the name, `outside_diameter_mm`, `wall_thickness_mm`, `elements` | | |
| `load_map` | `caepipe_load_case`, `category`, `target`, `direction` (but a thermal one), `mbf`, `caepipe_node`, `caepipe_element` | the load id; `load_case` | a thermal `direction` |

That is 35 MBF-determined fields, 8 identities and 5 authored values, with `direction`
counted by category, and `mbf_sha256`, which decides the route.

**Bound** (the sidecar's `mbf_sha256` equals the input's). A byte-bound sidecar restores
the identities, and the splits, that it explicitly preserves:

- nodes, from `node_map` inverted, which must be one-to-one;
- pipes and their from/to orientation, from `element_map` and its `reversed` flag;
- supports, their ids and DOF sets, and the split of a merged anchor, from `support_map`;
- bend components, from `element_map[...].bend.component`;
- materials, from `material_map[...].swbpipe_material`. MBF materials that map to one
  SWBPIPE material are merged after checking that E, ν and α are identical; otherwise
  `MBF-IMPORT-SIDECAR-CONFLICT`;
- load cases, from `load_case_map`;
- support families, from `support_map[...].family`;
- straight-pipe orientation, from `element_map[...].y_reference` (§10.5);
- the pressure contract and pressure regions, from `pressure_map`, checked against the
  file (§9.4).

**The sidecar parser** (independent review VER-002 r7, r8). The whole sidecar is parsed into
typed values before anything reads it, and the import reads only the parsed values. Each key
has an exact JSON type (a bool is not a number; null is a value only where the writer writes
null), an allowed vocabulary, a non-empty value where an id or name is required (empty after
Rust's `trim`, which removes only Unicode White_Space: U+3000 is, U+001C to U+001F are not,
although Python's `strip` removes them), and finite binary64 range for numbers. Unknown keys
and wrong shapes are refused by JSON path, for example `element_map[S1].reversed`
(`MBF-IMPORT-SIDECAR-INVALID`); nothing is defaulted or substituted. The vocabulary is the
union of what every SWBPIPE writer version records, so a sidecar from an older run, without
`family`, `origin`, `direction` or `y_reference`, still parses:

- top level: format 2 as listed under **Sidecar format**, format 3 the same with
  `sidecar_sha256` (**The self-digest**); a legacy sidecar holds the seven
  maps below (no `pressure_map`) and `mbf_sha256`, a lowercase SHA-256 or null. The entries
  below are a legacy sidecar's ("optionally": its documented omissions; format 2 keeps only
  the fields listed above);
- `node_map`: id → CAEPIPE name, one-to-one;
- `element_map`: id → `from`, `to`, `reversed` (bool), and optionally `layout_order`,
  `layout_row` (non-negative integers), `material`, `section`, `load`, and for a straight
  `y_reference` (`x`, `y`, `z` numbers; §10.5; format 2 also `y_reference_authored`), for a
  bend `bend` (`component` id, `radius_mm`, `angle_deg`, `flexibility_factor`,
  `tangent_intersection_mm` of three numbers, `caepipe_row_node`);
- `support_map`: id → `node`, `restrained` (a non-empty list of distinct DOFs), and
  `caepipe_kind` (`anchor`, `restraint`) and `family` (`anchor`, `guide`, `line_stop`,
  `vertical_support`; `anchor` only with all six DOFs; legacy also null or absent; format 2
  also `family_authored`; format 3 also `spring`, only on exactly one translation with
  `family_authored` true); the supports at each node
  together must pass the solve's support rules (§12). A bound import compares the file's rigid
  DOFs with the union of the non-spring entries at the node, and its spring DOFs with the spring
  entries, one each (`MBF-IMPORT-SIDECAR-CONFLICT`); a restored spring takes its stiffness from
  the file;
- `load_case_map`: id → label;
- `material_map`: name → `swbpipe_material` id, the four numbers the writer records,
  `temperatures_c`, `elements` and `origin` (one of `MATERIAL_ORIGINS`, format 2 also
  `"undeclared"`; legacy optionally absent);
- `section_map`: name → its two numbers and `elements`;
- `load_map`: `<case>/<load id>` → `load_case`, `category` (the writer's five), `target`
  (exactly a `node`, a `pipe`, or a `support` with its `dof`), `caepipe_load_case`, `mbf`,
  `caepipe_node` or `caepipe_element` by target, and `direction` (a token the solve parses;
  legacy also null or absent); load ids are non-empty and unique (`DUPLICATE_ID`). Each
  category names one kind of target (a force or moment a node, a weight or thermal load a
  pipe, a displacement a support), and a force a translation and a moment a rotation
  direction (VER-002 r13). A legacy primitive's repetitions must be what they repeat: its
  `caepipe_load_case` its case's label, its `caepipe_node` its target's CAEPIPE node, its
  `caepipe_element` the pipe's `element_map` entry, and its `mbf` the record main's writer
  wrote (`FOR`, `MOM`, `D`, `ROT`, `LOADS <L> Temperature 1`, `MATERIAL <M> weight
  density`). On a bound import a legacy element's `material`, `section` and `load` must be
  the file's record names;
- `pressure_map` (formats 2 and 3 only): as under **Sidecar format**; each terminal has
  `node_ref`, `closure_transfer` and provenance;
- integers (layout order and row) are non-negative and at most 2⁵³, exact in binary64.

**A bound sidecar is complete.** It was written for exactly this file, so each node,
element, support node, material, case and primitive of the file has its entry, a bend its
`bend` record, each section its entry, a file with Pressure 1 its `pressure_map` (§9.4), and
no entry names what the file lacks; otherwise `MBF-IMPORT-SIDECAR-CONFLICT`, naming the map.
A bound import takes no unbound default for anything the sidecar records: ids, families,
bends, materials, cases, primitives, regions and the shear model come from it. Only an
absence a legacy sidecar records is formed: a missing `y_reference` or `family` (above).

**An ambiguous sidecar is refused before assembly** (`MBF-IMPORT-SIDECAR-AMBIGUOUS`, by JSON
path; VER-002 r13). A sidecar is refused where its reverse mapping, from the file to the
sidecar, is not a function: two element entries between one pair of CAEPIPE nodes, or two
cases with one label, on both routes. Several primitives with one case, category, target
and axis are different: the MBF sums them into one record, a many-to-one mapping that
REQ-016 reports (below), and the import proceeds. The axis reads a direction through its
aliases. The check against the file of what the parser
admits stays where it was: ownership and pressure of regions (§9.4), supports' DOF sets per
node (§12), the frame check of `y_reference` (§10.5).

A seeded mutation test exercises the parser on the writer's format-3 sidecars of six invented files
(pressurized, pressure-free, with a contract and no region, with an authored non-default
orientation, with two forces the MBF sums, and with two springs, tranche 3). Each field is swapped for every other JSON type, each container emptied, each
number made huge or non-finite, an unknown key added and each key removed, and each of the
eight maps deleted and emptied. Each import passes a closure declaration, so a bound import
that fell through to unbound formation would succeed and be caught. Every mutated import is
refused with a named `MBF-IMPORT-*` code and never raises. The one exception is removing
one of the two summed entries, a consistent edit (§14), which imports as the model it
yields. Inside each entry reported as summed, the test also changes the category, the
target, the direction and the case, and adds an entry under the summed primitive's formed
id (a duplicate primitive id). Each is refused, and the identity alone, which sets aside
only the summed identity, refuses each too. The regeneration identity alone,
given the model the file and the unmutated sidecar determine, refuses each mutation too (a
mutated digest is the parser's: it decides the route; a mutated self-digest is the
self-digest's). The writer's own sidecar regenerates
exactly, as a positive control, and so does that of every model in the reversible subset
(§18.2). The writer's preflight, given the same maps, refuses with a named
`MBF-SI-*` code exactly where the import refuses. Each field is also set to a different
well-formed value; a single-DOF support entry also reads as the other kind (a spring as a line stop,
a rigid entry as a spring), which the MBF's spring and rigid DOFs refuse. One the MBF or a derivation determines is refused, and the identity alone
refuses it. One the writer copies from the model unchanged imports and regenerates as given
(§14). Each mutation and replacement is imported as a deliberate editor would leave it:
its self-digest is recomputed, so it reaches the guards above. Each is imported a second
time with its self-digest left stale, as an accidental edit leaves it. Where the sidecar
still claims format 3 with a well-formed digest and canonical content, the self-digest
alone refuses it (`MBF-IMPORT-SIDECAR-DIGEST-MISMATCH`). Otherwise the parser refuses it
by path.

**What a bound sidecar cannot restore.** Many-to-one mappings and identities the sidecar
does not preserve are reported, never guessed. Each is listed in the loss report as
`approximated` with code `MBF-IMPORT-ID-UNRECOVERABLE`, naming the sidecar entries
involved.

- **Primitive loads.** `load_map` records each primitive's case, category, target and
  direction (implemented), but not its magnitude. The writer sums several primitives on one
  target, so a primitive id is restored only when the sidecar names exactly one primitive
  for that (case, category, target, axis). A sum keeps its §10.4 id. It is reported at
  `load_map[<case>/<formed id>]`, and each sidecar entry it sums at `load_map[<key>]`. Its
  identity is all the regeneration identity sets aside (§14); a formed id that a restored
  id already has is refused by name (DUPLICATE_ID).
- **Thermal direction.** The MBF carries none, so a restored thermal primitive keeps the
  direction its sidecar entries record. When the MBF sums several, they must name one axis
  (aliases agree), and it is kept; entries naming different axes are refused by name
  (`MBF-IMPORT-SIDECAR-CONFLICT` at `load_map[<key>].direction`), since a sum across
  directions is not meaningful. The effective direction is resolved before anything is
  reported. A legacy entry without a direction records none. In a set where the other
  entries agree on one, the sum keeps it. Where no entry records one, the import's
  `global_x` stands. Either way each entry without a direction is reported by name
  (`MBF-IMPORT-DIRECTION-UNRECOVERABLE`), stating the direction the primitive takes and why
  (independent review VER-002 r15).
- **Support families** when the sidecar entry has no `family` (§12).
- **Bend component nodes** (§11 item 5).
- **Members the writer does not export** (§17.1).

**Unbound** (no sidecar, or hashes differ: a CAEPIPE re-export, or an edit). The MBF
governs every value, and ids are formed by §10.4. A sidecar is not applied unless the
caller explicitly asks for `rebind_by_node_number`.

- **REQ-016** (ScopeOfWork): "A sidecar that is not byte-bound shall be applied only on
  explicit request, with every restored identity reported as unconfirmed."
  The explicit request is honoured whatever the digest. A byte-bound sidecar with the request
  is applied the same way, rebound and unconfirmed (VER-002 r13; Agent 0 accepted it). The
  result never claims a bound or confirmed import: its `source` and `id_map` binding is
  `rebound_unconfirmed`, every restored id is listed there, and the model's data boundary
  says the ids are rebound and unconfirmed.
- **What rebinding does.** It applies the sidecar
  by CAEPIPE name only: nodes by number, pipes by from/to names with their `reversed` flag,
  supports by node with their DOF split and family, bends by TI number, materials by name.
  It restores no case or primitive id, orientation, origin or pressure state: those are
  formed or declared as on the unbound route. It keeps the parser, the element variant and
  ambiguity checks (§15 above) and does not run the regeneration identity (§14).
- **Unconfirmed identity.** Whether CAEPIPE keeps names is open (TBD-17-04-008). Every
  rebound id is therefore marked `rebound_unconfirmed` in the report and the output id map,
  and is never presented as a confirmed identity.
- **Entities and entries without a match.** Entities with no sidecar entry keep §10.4 ids.
  Sidecar entries with no MBF counterpart are listed as `omitted` ("absent from this MBF").
- **Conflicts.** A sidecar entry naming the same SWBPIPE id for two CAEPIPE names, or
  colliding with a formed id, is refused (`MBF-IMPORT-SIDECAR-CONFLICT`).

**Output.** `id_map.json` holds CAEPIPE name → SWBPIPE id for every node (including
`nA`/`nB`), element, support, bend, material and load case, and `region_map` (region id →
CAEPIPE elements) when the file has pressure. It also lists the straights whose
`y_reference` the import derived (`y_reference_derived`), the regeneration identity's one
input beyond the model (§14), and for each summed primitive the sidecar entries it sums
(`load_map[<id>].summed_entries`). DEL-17-05's CSV parser can
bind CAEPIPE results of an imported model through it.

## 16. The output document

### 16.1 Shape

- **Version.** `schema_version` is taken from the single enum value of
  `schemas/model.schema.yaml`. The importer fails if there is not exactly one. No literal
  appears in the importer.
- **Kind.** `document_kind: openpipestress.product_preview.model`.
- **Desktop profile.** The document satisfies the `DesktopDocument` profile, so the desktop
  can open it:
  - `project` with `id` (`project:mbf-<first 12 hex of the MBF hash>`), `name`,
    `description` ("Imported from MBF sha256:…") and `units` (§5.2);
  - `data_boundary` with the four policy strings the desktop writes on a blank document,
    with `private_data_policy` stating the import source and whether the import was bound;
  - `analysis_status` with the labels the pilot cases carry;
  - empty `diagnostics`;
  - labels and provenance on every entity.
- **Provenance strings.** "imported from MBF sha256:<hash>, line <n>" on every entity.
  Materials also carry `provenance_record` (§7.4).
- **Notes and open issues.** Heading lines and model comments go to `project.notes`.
  Bends without a factor become open issues (§11.6).
- **Absent members.** No combinations, lines, constraints, rule-pack references or model
  settings are written. The pressure contract is written only for a file with pressure, or
  restored by a bound sidecar (§9.4).

### 16.2 Determinism

Identical MBF bytes, sidecar, material-provenance and closure declarations, rebinding option, source label and importer/profile version produce byte-identical outputs. Members are written in the
writer's member order, entities in MBF order, and JSON is formatted canonically.

### 16.3 PRD §18.4 import warnings

- **Missing required fields and missing or inconsistent units:** refusals (§4, §5).
- **Missing provenance:** set on every entity (§16.1).
- **Unclear redistribution:** a material without affirmative provenance is refused (§7.4).
- **Suspected protected values:** the material is refused (§7.1, §7.4).
- **Unreasonable values:** refused by the range checks (§4.3).

### 16.4 Privacy of outputs

The importer cannot know who wrote an MBF. Its outputs are therefore treated as private
unless the input is a committed invented fixture, identified by its provenance sidecar
(§18.1). The command line refuses to write outputs inside a Git work tree for any other
input.

## 17. The writer's mappings and their inverses

| Writer (`si_writer.py` and its `assumptions`) | Importer |
|---|---|
| `OPTIONS HGRA,SI`; always Y vertical; X, Y, Z unchanged (§5.1) | SI mode; Y vertical, or Z rotated exactly (X, Y, Z) → (X, Z, −Y); HGRA is `delegated` |
| Lengths in mm, scaled by powers of ten from m or mm | Lengths kept in `mm`; project length `mm` |
| E in N/mm², from Pa, kPa, MPa or GPa | E in `MPa` |
| ν as given, or E/(2G) − 1 | ν as given; G = E/(2(1+ν)) with basis `homogeneous_isotropic_E_nu_v1` |
| Material without α → 0 | α = 0 explicit |
| Temperature-independent material at three identical temperatures | Identical lines → temperature-independent; differing lines refused |
| Weight per length (−Y) → weight density ρ = \|w\|/(A·g₀) | ρ → w = −ρ·g₀·A along `global_y`, in the mechanical case |
| Weight only along −Y | Weight along −Y (after the rotation, for a Z-vertical file) |
| LOADS: T1 = T_ref + ΔT, P1 = region pressure in bar (0 outside every region), SG = 0 | ΔT = T1 − T_ref; P1 > 0 → pressure regions in the file's unit (§9.4); SG must be 0 |
| With pressure, `OPPRES 0,0,0,1,0`; contract, regions, terminals and closure in `pressure_map` | Exact contract; regions restored from `pressure_map` when bound, else formed with the declared closure (§9.4) |
| `OPTEMP` = reference temperature | T_ref |
| Mechanical case → sustained label; thermal and displacement → expansion label (`LOAD_CASE_LABELS`) | `case:mbf-sustained` and `case:mbf-expansion-t1` (ids restored from the sidecar) |
| Combinations not exported | None created |
| Force and moment comments `FOR`, `MOM` per node, summed, without `FLC` | Primitives per non-zero component, in the mechanical case. Without `FLC`: bound imports only; unbound imports refuse it, except where an explicitly retained bounded profile decision admits it |
| Anchor displacement `D`, rotation `ROT` (rad → deg) | Imposed displacement and rotation on the anchor, in the thermal case, in `mm` and `deg` |
| Full six-DOF restraint (one or several supports) → `KA` | Anchor; the sidecar splits it back |
| Translational partial restraint → `FIXD` | Restraint support; family restored from `support_map` in bound imports (§12) |
| Partial rotational restraint refused | Refused as well: the solve admits no partial rotational restraint without a family (§12) |
| `guide`, `line_stop` families → `FIXD`, family in `support_map` | Restraint with the family restored; unbound `G` and `LS(0,0)` along an axis import as `guide` and `line_stop` (§12) |
| Spring (`family: spring`, one translation) → `KA,TRAK(kx,ky,kz),ROTK(0,0,0)` in N/mm; rotational springs refused | Flexible anchor → one spring per non-zero TRAK component, ids restored from `support_map` |
| Hangers, nonlinear supports refused | Hangers refused (§12) |
| Curved bend → `T<n>,KI,B<R>,…,CFF=<k>` at the TI; ends `nA`/`nB` | Tangent nodes from TI, R and the adjacent legs; bend pipe, y_reference and component reconstructed |
| Bend SIF pair → `BSIF(<in-plane>,<out-of-plane>)` after `FF`; one of the pair alone refused (`MBF-SI-BEND-SIF-INCOMPLETE`) | The pair, unit `none` (§11 item 5) |
| Straight-pipe `y_reference` not in the MBF; recorded in `element_map` | Restored in bound imports; otherwise the documented local y (§10.5) |
| US-customary model units → exact normalization to SI (`MBF-SI-UNITS-NORMALIZED`, §5.2) | SI import of the normalized values |
| `M`, `P`, `L=` written only when changed | M and P carried until changed (documented). `L=` carried the same way in bound imports; unbound imports refuse it, except where an explicitly retained bounded profile decision admits it |
| Node numbers 10, 20, … in layout order; sidecar maps | Ids from the sidecar, or formed from numbers |
| Members and fields the MBF does not keep: one `omitted` loss entry each (§17.1) | Not recoverable from the MBF; the importer writes only §16.1's desktop-profile members |
| CRLF line ends; printable ASCII; 10 significant digits | CRLF or LF; UTF-8 with the listed marks; any precision |

### 17.1 What the writer writes, omits and refuses

Every model member and field the writer reads or could receive is written, refused by name
or listed as omitted (REQ-003, REQ-004). An omitted field gets one `omitted` loss entry.
Its code is `MBF-SI-FIELD-NOT-EXPORTED`, its entity is `model.<member>.<field>`, and its
reason names every entity that has the field. A whole member gets `MBF-SI-MEMBER-NOT-EXPORTED`
with entity `model.<member>`. A field is listed only when it holds a value: an empty
string, list or object is not listed. The sweep is generic. A field not named below, such as
a newer schema's, is listed as omitted, never dropped. "Sidecar" means it travels in the
writer sidecar for a bound import (§15).

| Member or field | Writer | Reason |
|---|---|---|
| `schema_version` | Written (implied) / refused | Only the current version is read; the import writes it |
| `document_kind` | Written (implied) | A schema constant the import writes |
| `pressure_contract` | Written / refused | `OPPRES` and Pressure 1 (§9.4); contract in the sidecar; `shear_model` omitted (`MBF-SI-SHEAR-MODEL-NOT-EXPORTED`) |
| `shear_model`, `reference_configurations` | Refused | Not exported |
| `project.units` length, force, temperature | Written / refused | Values converted into the MBF's SI set (§5.2) |
| `project.units` any unit other than the import's (mm, N, N*m, MPa, Pa, degC, deg) | Omitted | A display choice the MBF does not keep |
| `project` other fields (`id`, `name`, `description`, `tags`, `notes`) | Omitted | No MBF field; the title is the caller's |
| `data_boundary`, `analysis_status` | Omitted | The import writes its own |
| `sections` (library), `pipe_segments.section_ref` | Omitted | Each pipe's own section is written as a `PIPE` record |
| `diagnostics` | Omitted | The import's validation forms its own |
| `lines`, `unresolved_assumptions`, `rule_pack_refs`, `constraints`, `equipment_interfaces` | Omitted | No MBF record |
| `combinations` | Omitted (`MBF-SI-COMBINATION-NOT-EXPORTED`) | The target program forms its own |
| `model_settings` | Omitted (`MBF-SI-MODEL-SETTINGS-NOT-EXPORTED`) | Input validation settings |
| `materials` `id`, `elastic_modulus`, `poisson_ratio`, `thermal_expansion_coefficient` | Written | `MATERIAL` (§7) for each material a pipe segment uses; id as description and in the sidecar |
| `materials` `shear_modulus` | Written as ν = E/(2G) − 1 when no `poisson_ratio` is given; omitted (`MBF-SI-SHEAR-MODULUS-NOT-EXPORTED`) when both are given and E/(2(1+ν)) differs from G | The MBF carries ν; the import derives G from E and ν |
| `materials` that no pipe segment uses | Omitted (`MBF-SI-FIELD-NOT-EXPORTED`, entity `model.materials`, naming each id) | The MBF writes only the materials its pipes use |
| `materials` `temperature_points`, `expansion_laws` | Refused | Temperature-dependent properties |
| `materials` `constitutive_basis` | Omitted | The import forms its own basis from E and ν |
| `materials` `provenance_record` | Omitted | Only its origin classification travels, in the sidecar (§7.4) |
| `nodes` `id`, `position` | Written | Layout coordinates; ids in the sidecar |
| `pipe_segments` `id`, `from`, `to`, `material`, `section` OD and wall | Written | Layout, `M`, `P` |
| `pipe_segments` `y_reference` | Omitted (`MBF-SI-ORIENTATION-NOT-EXPORTED`); sidecar | Straights only; a bend's fixes its plane |
| `section` `corrosion_allowance`, `mill_tolerance`, `insulation_thickness` | Written as 0 / refused if non-zero | Bare pipe only |
| `section` densities | Omitted (`MBF-SI-SECTION-FIELD-NOT-EXPORTED`) | Weight only from weight primitives |
| `supports` `id`, `node`, `restraints`, `family`, `stiffness` | Written / refused | `KA`, `FIXD`, `TRAK` (§12); family in the sidecar |
| `supports` `hanger`, `nonlinear` | Refused | Not exported |
| `supports` `properties` | Omitted | Display properties the solve ignores |
| `supports` `boundary_association` | Omitted | No MBF field |
| `components` other than a realized curved bend | Refused | §11 |
| Bend `geometry` `bend_pipe_ref`, `bend_radius`; `mechanics_interface.solver_consumption` | Written | `KI`, `B` at the TI |
| Bend `geometry.bend_angle` | Written (implied) | Checked against the straights; geometry governs |
| Bend `modifiers.flexibility_factor_user_value` | Written | `FF` |
| Bend `modifiers.sif_in_plane_user_value`, `sif_out_of_plane_user_value` | Written / refused if one alone | `BSIF` |
| Bend `modifiers.sif_user_value` | Omitted | A different SWBPIPE field from the pair; `BSIF(s,s)` would import as the pair |
| Bend `modifiers.source_reference` | Omitted | The import names the MBF line |
| Bend `completeness`, `geometry.bend_plane_orientation`, `geometry.bend_geometry_source_reference`, `mechanics_interface.rule_check_consumption` | Omitted | No MBF field, or derived |
| `load_cases` `id`, `primitive_loads`, `pressure_regions` | Written | §9; ids in the sidecar |
| `load_cases` `equivalent_static`, `modulus_basis_ref`, `modulus_basis_temperature`, `analysis_state` | Refused | Not exported |
| `load_cases` `label`, `kind`, `status` | Omitted | The import forms them from the target program's case |
| `primitive_loads` `id`, `category`, `target`, `direction`, `magnitude`, `dimension` | Written / refused | §9, §13; summed per target |
| Pressure region fields other than `provenance` | Written (sidecar) / refused | `member_connector_ids` refused |
| Pressure region `provenance` | Omitted | The import names the MBF line as the region's provenance |
| `label`, `provenance`, `tags`, `notes`, `provenance_record` of any entity not named above | Omitted | No MBF field, and the sidecar does not carry it |

## 18. How the importer is exercised

### 18.1 Public tests

Inputs are invented SWBPIPE models and MBF the project writes. Hand-written MBF fixtures
live in `fixtures/caepipe_mbf/invented/import/`, each with a provenance sidecar that
declares it invented. The boundary guard (§18.4) requires the sidecar.

1. **Grammar** (`tests/test_caepipe_mbf_importer.py`):
   - every length form of §5.4, with invented values, including signs and typographic
     marks, and every refused form;
   - numbers, comment splitting with parentheses, continuation lines, the `C` prefix rule
     and longest-name matching;
   - keyword recognition, including the ambiguous `OPT`;
   - encoding and line-end refusals.
2. **Refusals.** One test per named refusal and per loss category, each asserting the code,
   the line and that no model is produced (or, for loss entries, that the model is produced
   and the entry is listed).
3. **Round trip on the reversible subset** (`tests/test_caepipe_mbf_round_trip.py`).
   - **Models.** The pilot cases (`validation/caepipe_correlation/cases/`) and the writer
     test's model builders, in m and mm projects, each within the reversible subset of
     §18.2. Tranche 2 adds pressure models in Pa (an L through a bend), bar (a tee with a
     separately supported end) and psi (US-customary), and the imports of two invented
     Z-vertical files with pressure, one SI and one English, the SI one with a skew
     straight, a corner and a bend. The test file lists which of them are in the
     byte-reproducible subset.
   - For each model A with W = writer(A) and I = import(W):
   - (a) **Canonical equality, bound.** With the bound sidecar, A and I agree within
     §18.2's per-scalar bounds. Round-trip equality is required for bound imports.
   - (a′) **Refusal, unbound.** Without a sidecar, import(W) is unbound. The writer always
     emits `L=`, so the import is refused with `MBF-IMPORT-TBD-17-04-007`, and the test
     checks that refusal. It also checks the refusal for a writer MBF that carries a force
     or moment without `FLC`. Where an explicitly retained bounded profile decision admits
     these records, that decision states which unbound equality checks replace the
     refusal checks.
   - (b) **Ids.** With the bound sidecar, the identities and splits the sidecar preserves
     are restored, and every §15 unrecoverable identity is reported.
   - (c) **Byte reproduction**, for models in the byte-reproducible subset only (§18.2).
     - writer(I) equals W byte for byte.
     - Required conditions: a bound sidecar, and identical writer settings. The settings are
       the same `title` (the imported `project.name`), `reference_temperature_c` (the
       imported OPTEMP value, because the writer takes it as an argument, not from the
       model) and `max_layout_rows`.
     - For every model, the test compares W and writer(I) field by field and reports each
       formatting collision by record and field.
     - A collision on a model listed as byte-reproducible fails the test. A mismatch is
       investigated, never absorbed by widening a tolerance.
   - (d) **Solve equality.** Headless solves of A and I give global displacements and
     reactions that agree within τ_solve = 10³ · r_in of each family's largest magnitude.
     r_in is the model's largest relative input bound under §18.2. This is a
     numerical-meaning check (Root `AGENTS.md`). I is solved as imported: its straights carry
     the restored or documented orientation (§10.5); only a source model the writer tests
     author without orientation or provenance gets them added before its own solve.
     - Models with r_in > 1e-8 are excluded from this check and listed by name; an example
       is a ΔT that is tiny next to its temperatures.
     - **ASSUMPTION: 10³ is a provisional test threshold, not a derived bound.** It is
       meant to allow for amplification of input rounding through the stiffness solution
       on these small, well-conditioned invented models. Before it is relied on, a
       condition-number-based bound or measured sensitivity should replace it. A breach is
       investigated, not absorbed.
   - (e) **Determinism.** Importing twice gives identical bytes.
4. **Units check, independent of the writer.**
   - An invented English-mode MBF and an invented SI-mode MBF of the same geometry and
     loads. The test converts the SI values by exact definitions using `Fraction`, not the
     importer's code.
   - Both files are unbound and use only documented records: no LOADS section, forces
     and moments with `FLC=1`, weight densities, and anchor displacements and rotations.
     They therefore import under the base rule.
   - The two imports solve to equal results within 1e-9 relative.
   - The English import's positions equal the test's own rational evaluation of the
     feet-inch fields.
5. **Bend boundary case** (invented). A 90° bend with R = 999 mm, tangent lengths of
   999.0000008991 mm and adjacent straights of 0.5 mm. The writer writes it without
   diagnostics. Reconstruction shifts a tangent node by about 9e-7 mm, beyond the
   formatting-only bound. The test checks that the residual-inclusive bound of §18.2
   covers it, and that without the residual term the comparison would fail.
6. **Schema.** Every imported document validates against `schemas/model.schema.yaml`
   (`DesktopDocument` profile) and passes the desktop shape-case corpus checks that apply
   to opened documents.
7. **Tranche 2, hand-derived.** Pressure: each unit's conversion to bar, the region
   partition (tee, corner, collinear run, pressure and bore changes, an equal bore from
   another OD and wall, an unpressurized element), the closure declaration and its
   refusals, the OPPRES effects, and the bound pressure map, its shear model and tampered
   sidecars. Z vertical: a skew element's and a vertical element's local y, the rotation of
   coordinates, forces, moments, displacements, rotations and restraint axes, a bend, the
   gravity direction, a Z-vertical file against its hand-rotated Y-vertical twin (equal bit
   for bit, apart from straight orientation), and positions against the test's own exact
   sum of the printed offsets, rotated and rounded once. Independent review VER-002 r1 adds:
   the ported straight-run check on hand values; a drifting run the route refuses as one
   region, which the import refuses by name (VER-002 r2: never cut); a zigzag run and a run
   whose first two straights fail alone, which the route admits whole and which stay one
   region; corners at 2e-3 rad and refused kinks at 1e-4 and 5e-4 rad; a drifting run after a
   bend; the headless solve of the admitted imports and of a corner, while the drifting run
   as one region is refused by the route (`PRESSURE_REGION_NONCOLLINEAR`); a section with and
   without a stated corrosion allowance, and the writer's refusal of a section without one
   under the contract (the review's round trip); an element collapsing in metres; and a
   subnormal norm rounded once. VER-002 r3 adds: equal decimal bores the route's guard
   refuses in metres (refused by name; the route refuses them as one region too), and a run
   the route admits only reversed (emitted reversed, solved; refused by the route in file
   order). VER-002 r4 adds the review's two round trips (bores, geometry): each imports and
   solves, the writer refuses it as `MBF-SI-PRESSURE-REGION-FORMAT`, and without that
   preflight the bound import of its text refuses the restored region by name. VER-002 r5
   adds: a restored region through a tee (refused by the import, the writer and the route),
   the preflight's refusal of an OD that prints as twice the wall, ids equal once ':' reads
   as '-', empty and repeated restored region ids, and a restored orientation along the
   element axis.

8. **Tranche 3, hand-derived** (§12). Rigid guides and limit stops on invented layouts: the
   lateral DOFs of an X, a Y and a Z element, a run's first node taking its following element,
   two limit stops at one node, and the Z-vertical rotation of `DV`, a guide's element and `TRAK`.
   Springs from a flexible anchor, a flexible guide, a flexible limit stop and a skewed restraint,
   in N/mm and lbf/in. One refusal test per refused case of §12, hangers included. Solve-backed:
   an imported rigid guide and an imported limit stop each prop a 3 m cantilever loaded at
   midspan and carry 5P/16; a skewed-restraint spring and a flexible anchor at a cantilever's
   tip each carry k/(3EI/L³ + k) of the tip load (Euler–Bernoulli, within 1e-9). The writer's
   spring export: the hand-derived TRAK line, each refusal, and the preflight's refusal of a
   stiffness its printed value breaks (1e306 and 1e20 N/mm). VER-002 r2: the checked-JSON domain on
   a stiffness, a force and a coordinate, with 2⁵³ − 1 admitted and 2⁵³ refused; the runner's own
   `CHECKED-JSON-UNSAFE-INTEGRAL-FLOAT` for the same file with the guard removed; and a 1e-300 N/mm
   spring that imports as written and the runner refuses as `NUMERICAL_INTEGRITY_ASSEMBLY_UNRESOLVED`.
   The round trip adds `writer-springs` (the cantilever with
   a 250000 N/m and a 1.5 lbf/in spring) to the reversible and byte-reproducible subsets; the
   parser's spring vocabulary is refused by path outside format 3.

The weight-density conversion, the G derivation, the ΔT mapping and the rational
summation change numerical meaning. They therefore get targeted regression tests (items
3d and 4) and independent scrutiny by a separate agent before integration.

### 18.2 Reversible subset, canonical comparison and tolerances

**The reversible subset** is explicitly defined. A model is in it when all of the
following hold:

- the writer writes it with `status: written`;
- no `MBF-SI-SHEAR-MODULUS-NOT-EXPORTED` warning is raised;
- every value the comparison reads is in the writer's units tables.
- **Printed-domain validity.** import(W) completes without refusal, so every value *as
  printed* passes the importer's range, geometry and bend checks (§4.3, §10, §11). For
  example, ν = 0.49999999999 is printed as 0.5, which §4.3 refuses, so that model is
  outside the subset.
- **Distinct record keys preserved.** Every pair of distinct MATERIAL, PIPE or LOADS
  records in W differs in at least one printed field the importer reads. For example, ΔT
  values of 0.1 and 0.100000001 at T_ref = 20 give two LOADS records that both print
  T1 = 20.1. The import cannot tell their roles apart, and re-export merges them, so that
  model is outside the subset.

**The byte-reproducible subset** is the part of the reversible subset for which, in
addition:

- the sidecar is bound and the writer settings are identical (§18.1(c));
- no formatting collision occurs.

A formatting collision is any field whose value, re-derived from the parsed MBF and
re-printed by the writer, prints differently from W. Examples are a LOADS key merge, a
re-derived weight density, offset or TI that crosses a tenth-digit rounding boundary, and
a material temperature row (the writer's lowest and highest temperatures) re-derived from
ΔT. Collisions are reported explicitly, and the model is then excluded from byte
reproduction by name.

**Representation losses the comparison excludes:**

- `y_reference` on straights, which the MBF does not carry (bound imports restore it from
  the sidecar; unbound imports take §10.5's documented local y);
- support families, which `support_map` does not carry (§12);
- labels, provenance and the other members and fields §17.1 lists as omitted;
- several primitives on one target, which are compared as their sum;
- zero-valued primitives, which are dropped;
- absent versus zero α, and absent versus zero section fields;
- combinations, which are not exported;
- pressure-region ids and region provenance (bound imports restore the ids from
  `pressure_map`).

Each exclusion is listed in the test so that it stays explicit.

**Canonical form.** Both models are reduced to this subset, in SI base units:

- node positions;
- per pipe: endpoint pair, oriented only when the sidecar is bound; OD; t; E; ν, the
  writer's, given or derived; G; and α;
- per node: the union of restrained DOFs of its rigid supports, and per support when the sidecar
  is bound; per node and DOF, the spring stiffness in N/m, bounded as printed directly from its
  `TRAK` component in N/mm (× 10³);
- per bend: R, k, the bend-plane direction and the component node. The bend-plane
  direction is compared as the writer checks it: the normalized projection of
  `y_reference` perpendicular to the unit chord between the two tangent nodes. For
  example, an invented fixture with `y_reference` = (2, 0, 0) reconstructs as
  (1, 0, −1)/√2. The raw vectors are 45° apart, but the projected directions coincide.
- per case kind: summed force and moment vectors per node, summed weight per length per
  pipe, summed ΔT per pipe, and summed anchor displacements and rotations;
- the pressure contract, and per case kind each pressure region by its member pipes: its
  basis, its terminals (CAEPIPE name and closure transfer, ordered when bound) and its
  pressure in Pa, the exact product of value and unit factor rounded once.

Node positions of a Z-vertical import are compared in SWBPIPE's axes. The test also checks
the rotation itself exactly: its own rational sum of the printed offsets, rotated
(X, Y, Z) → (X, Z, −Y) and rounded once, equals every imported position bit for bit.

**Aggregation (independent review VER-002 r1).** Sums of primitives on one target are
formed exactly, then rounded once:

- **Writer.** It adds the primitives as exact rationals (`fractions.Fraction`) in the MBF
  unit, each the exact product of the authored value and its definitional factor, and
  rounds the sum once to binary64. Radians to degrees, the one non-rational factor, is
  rounded per primitive before the sum. A binary64 running sum lost the remainder of
  cancelling loads: 1e9 + 0.001 − 1e9 lbf wrote 0.004447937012 N instead of 0.004448221615 N.
- **Canonical comparison.** It sums A's primitives independently of the writer, as exact
  rationals in SI base units with its own exact factors (1 in = 0.0254 m, 1 lbf =
  4.4482216152605 N, °F intervals × 5/9; degrees by binary64 π), and rounds once. A
  comparison that summed in binary64 shared the writer's defect and concealed it.
- **Importer.** Its sums of FOR, MOM, D and ROT components on one node are exact rationals
  of the printed values, rounded once.

The bounds below are unchanged: a summed load is printed once, so its bound is the
printed-directly bound of the sum. The canonical form's bend-plane projection and the bend
residuals use scaled norms (`hypot`), so a bend of R = 1e-200 mm is compared, not divided
by zero (independent review VER-002 r2).

**Tolerances (HELP_HUMAN), derived from the writer's number formatting and, for bends,
its admitted residuals.** The writer
prints every real with `format(value, ".10g")`, and prints zero as `0`, exactly. A printed
value p ≠ 0 therefore stands for any number within

  δ(p) = 0.5 × 10^(⌊log10|p|⌋ − 9)

of it, where p is the printed decimal (δ(0) = 0). Let ε = 2⁻⁵³. The test computes, for
each compared scalar, an absolute bound from the printed values in W that the scalar
depends on:

- **Printed directly** (OD, t, R, k, E, ν, α, force and moment components, displacements,
  the reference temperature): δ(p) + 4ε|p|, converted to SI base units with the same
  factor as the value. A rotation printed in degrees and compared in radians uses
  δ(p)·π/180. For ν, A's value is the writer's ν before printing: the authored ν, or
  E/(2G) − 1 when only G was authored.
- **Pressure:** printed directly in bar, (δ(p) + 4ε|p|) × 10⁵ Pa. For example, 150 psi
  prints as 10.34213594 bar, so the bound is 5e-4 Pa plus the ε term.
- **ΔT = T1 − T_ref:** δ(T1) + δ(T_ref) + 4ε|T1|. This is an absolute bound only, and
  relative error is not used.

  For example, a ΔT of 0.123456789 °C at a 20 °C reference is printed as T1 = 20.12345679,
  so δ(T1) = 5e-9 °C. The import returns ΔT with an error of about 1e-9 °C: about 8e-9
  relative, but within the absolute bound.
- **Node coordinates:** per axis, the sum over the printed start coordinate and every
  printed offset on the node's path from its run start of
  δ(p) + 2ε × (the model's largest coordinate magnitude). The ε term covers the writer
  forming each offset as a binary64 difference of positions. A further 4ε × that magnitude
  covers the import.
- **Bend tangent nodes:** the TI's coordinate bound, plus
  δ(R)·tan(θ/2) + R·sec²(θ/2)·δθ/2, where δθ ≤ 2·(the leg end-point bounds)/(the shorter
  leg length).
- **Bend residuals.** Bend bounds include the writer's admitted tangent-length,
  intersection and orientation residuals in addition to formatting error; otherwise the
  affected models are explicitly excluded.
  - **Why.** The writer accepts a bend whose geometry departs from an exact corner bend
    within its relative tolerance 1e-9 (`_BEND_RELATIVE_TOLERANCE`): a tangent-length
    mismatch |tᵢ − R·tan(θ/2)|, an intersection gap of the adjacent straights, and an
    orientation misalignment. The misalignment is the sine of the angle between the
    normalized projection of `y_reference` perpendicular to the unit chord and the
    direction from the chord midpoint toward the TI, which the writer bounds by the same
    1e-9. The import rebuilds an exact corner bend, so those residuals are lost.
  - **How.** The test computes A's actual residuals with the writer's own formulas and
    adds them:
    - each tangent node's bound gains |tᵢ − R·tan(θ/2)| plus the intersection gap;
    - the bend-plane direction bound gains A's orientation residual, computed with that
      same projection, in radians.
  - **Exclusion.** A model whose residuals cannot be computed this way is excluded from
    canonical equality by name.
- **Weight per length:** relative bound δ(ρ)/|ρ| + (|∂A/∂OD|·δ(OD) + |∂A/∂t|·δ(t))/A + 8ε.
- **G.** The import derives G' = E′/(2(1+ν′)) from the printed E′ and ν′.
  - Relative bound δ(E)/E + δ(ν)/(1+ν) + 8ε, when A authored ν (A's G being
    E/(2(1+ν))) or authored only G.
  - When A authored both ν and G, the writer accepts them without warning if they agree
    within 1e-9 relative (`si_writer.py`, `MBF-SI-SHEAR-MODULUS-NOT-EXPORTED` threshold),
    and carries only ν. The bound then adds that 1e-9: 1e-9 + δ(E)/E + δ(ν)/(1+ν) + 8ε.
- **E, α:** printed directly (above).
- **Weight per length (note):** the bound above holds because the writer computes ρ from the
  authored OD and t, and the import recomputes A from the printed OD and t. The ∂A terms
  cover that difference.

A difference beyond its bound is an importer or writer defect, not a tolerance matter.
The bounds include formatting and arithmetic error plus the explicitly stated
writer-admitted residuals. r_in
in §18.1(d) is the largest of these bounds expressed relative to its scalar, over scalars
that are not zero.

### 18.3 Private regression against CAEPIPE-exported MBFs

The test is `tests/test_caepipe_private_mbf_import.py`.

- **Data.** It reads the exchange folder named by `SWBPIPE_CAEPIPE_EXCHANGE`. It skips
  with that reason when the variable is unset or the folder is empty, and when a case has
  no exported MBF.
- **Inputs per case.** `cases/<id>/caepipe_out/exported.mbf` is the file CAEPIPE's MBF
  export wrote after importing the case's writer MBF. It is CAEPIPE output, private.
  DEL-17-05's harness writes it with its export step (`run_pilot --export-mbf` or
  `--export-only`, DEL-17-05 Design §3, §5), bound to the case's solve by hash. The
  owner can also export it by hand (O-4).
- **Run.** The test imports the exported file as unbound. It asks for `rebind_by_node_number`, so every rebound id is marked unconfirmed
  (§15). It supplies the case's `project_invented` material declaration, then compares the
  canonical forms against the case's SWBPIPE model.
  - Under the base rule, an exported MBF that carries `L=` or a force or moment without
    `FLC` is recorded as `refused` (`MBF-IMPORT-TBD-17-04-007`). That is the expected
    outcome until TBD-17-04-007 closes, except where an explicitly retained bounded
    profile decision admits these records.
- **Outcomes.** It records one of `equal_within_writer_tolerance`, `differs` or
  `refused`, with the field names and refusal codes. The outcome goes to
  `cases/<id>/imported/import_outcome.json`, a private layer, on an explicit `--record`
  run.
- **Failure.** The test fails when a case's outcome moves out of its recorded
  classification.
- **Privacy.** All CAEPIPE-derived values, differences, classifications and detailed
  diagnostics remain in the private exchange folder. Standard output, including pytest
  output and assertion messages, contains operational status only: the case id, whether
  the case was checked or skipped and why, and the path of the private record. This
  matches DEL-17-05 Design §8. A failing test names the case and the private record path,
  never the outcome, field or code.
- **What it shows.** Which CAEPIPE-exported constructs the importer reads back, recorded
  privately.
  - **Expected first refusals.** The target tool's own re-export of a writer MBF differs
    from the writer's file in its sections, fields and rows, per private notes.
  - Under the bound/unbound rule (§1) and §4.3, undocumented parts and TBD-dependent
    records are refused by name. Each refusal becomes a dossier answer or a recorded owner or HELP_HUMAN resolution, never a silent parser
  adjustment derived from private files.

DEL-17-05 Design §5's run-directory table gains two rows: `caepipe_out/exported.mbf`
(PRIVATE) and `imported/` (PRIVATE, derived from CAEPIPE output).

**Authored cases** (Agent 0, under the owner's delegation, 2026-10-27). The writer cases cover only
what the writer writes: Y vertical and no refused input. A second case kind covers the rest of the
importer's mappings and named refusals.

- **Fixtures.** `validation/caepipe_correlation/authored/` holds invented, hand-written MBF files, each
  with an invented-provenance sidecar (§18.4), and `manifest.json`. Each manifest entry names the case
  (`cpa-nn-…`), its file, the construct it exercises, the importer's own outcome on the file
  (`importer_expectation`: imported, or refused with a named code) and, for a file with pressure, the
  closure declaration both imports take (§9.4). The set covers pressure (a straight run and a run
  through a bend), Z vertical (with a riser, a force, a bend with an anchor displacement, and supports),
  each mapped support kind of §12 (rigid guides, guides and limit stops with `STIFF`, two-way limit
  stops, axis-aligned `K`, flexible anchors), and each refusal: `GGAP`, `MU`, limit-stop gaps, a one-way
  stop, a skewed `DV`, `CNOD`, the hangers `VS`, `CS` and `US`, a non-zero `ROTK`, `KR` and a
  temperature-dependent material. `KR` is written alone on its row with its `DV`: the target imports
  one data item per row and takes further items from Location rows (observed format behaviour).
- **Round trip.** DEL-17-05's pilot command (`run_pilot --authored`, DEL-17-05 Design §3) has the target
  tool import the authored file as written, through the same batch route as a writer case, which also
  solves it; the export step then writes the model back to `caepipe_out/exported.mbf`. The authored
  file is the case's `caepipe_in/model.mbf`, and the run record binds the export to it by hash.
- **Comparison.** The test imports the authored file and the exported file, both unbound, without a
  sidecar, with the case's `project_invented` declaration and its closure declaration. For a mapped
  case the authored import is the reference: the SI writer renders it, and the comparison uses the
  same canonical forms and writer-derived tolerances as the writer cases (§18.2). The outcome is
  `equal_within_writer_tolerance`, `differs`, or `refused` when the export's import is refused. For a
  refused case it compares refusals: `refused_alike` when the export's import is refused with the
  manifest's named code, `refused_differently` when it is refused otherwise, and `differs` when it
  imports. An authored file that no longer meets its manifest expectation fails the test by case id.
- **Binding.** An authored case is checked only when its run record's export entry binds the export
  to the solve of exactly the tracked authored file (both hashes); otherwise it is skipped with that
  reason.
- **Outcomes and privacy** are as above: the record goes to `cases/<id>/imported/import_outcome.json`
  with `case_kind: authored`, and standard output carries operational status only.
- **Public tests** check each fixture against its manifest expectation with the importer alone, the
  manifest's coverage of the constructs above, and the classification and binding on invented
  exchange folders whose "export" is the authored file or an invented edit of it. They assert no
  outcome of a target-tool run.

### 18.4 Boundary guard

`tests/test_caepipe_data_boundary.py` is extended. It fails if any tracked `.mbf` lacks an invented-provenance sidecar, regardless of
directory. It also
fails if a tracked file has `document_kind: swbpipe.caepipe_mbf_import_report` without
one, or sits under an exchange-style `imported/` path.

## 19. Model document 0.8.0 (#1649)

0.8.0 adds one optional member, `geometry.eccentric_offset`, on reducers, and refuses
0.7.0 by name on open (owner, 2026-10-26: <!-- owner-words -->"R-c1 A, 0.8.0
a2"<!-- /owner-words -->).

- **Version.** The importer takes its version from the schema enum (§16.1). It therefore
  writes 0.8.0 as soon as the ripple lands, with no edit. Its round-trip and schema tests
  move with the ripple's regeneration.
- **Members.** Tranche 1 refuses reducers (`JD`). The importer never writes
  `eccentric_offset`, and nothing else in 0.8.0 touches the importer's members.
- **Writer version.** The writer's `check_document` reads the schema's single enum value
  (implemented), so it follows the 0.8.0 ripple without an edit.
- **Later reducer tranche.** A CAEPIPE reducer row carries both end ODs and thicknesses
  and its own offsets. A transverse offset component would map to `eccentric_offset` under
  R12's convention (global axes, from the centreline at the component's node to the
  centreline at the body's other end), and the node geometry would govern as 0.8.0
  requires. That waits for 0.8.0 and a reducer tranche.

## 20. Open items

### 20.1 TBDs

TBD-17-04-005 to -009 are added to DEL-17-04 CLM-022. The questions go to the DEL-17-01
dossier as CQ-17-01-011 to -015. TBD-17-04-010 is added to the DEL-17-04 ScopeOfWork TBD table
(Agent 0, 2026-10-27); its unit question is drafted in the dossier as CQ-17-01-017, and O-7's
as CQ-17-01-016. Both are drafts for the owner to send.

| TBD | Question | Effect until closed |
|---|---|---|
| TBD-17-04-005 | Encoding and default of OPMISC's gravitational-direction field (15.10 and later) | **Finding** (observed format behaviour, recorded here; the TBD's status is Agent 0's): CAEPIPE's export writes OPMISC with 13 fields, the direction as three components after the acceleration, and for the default earth environment acceleration `386.4` (in SI files too) with direction (0, −1, 0). The importer reads that default setting only (§6) and refuses any other (`MBF-IMPORT-GRAVITY-UNSUPPORTED`) and a one-field direction (`MBF-IMPORT-TBD-17-04-005`). Still open: the public field list's one-field encoding, the meaning of the export's field 9, and the acceleration's unit in SI files. The writer never writes these fields |
| TBD-17-04-006 | Text encoding, foot and inch mark characters, and numeric precision that CAEPIPE's MBF export writes | The importer reads only strict UTF-8 and the marks the public pages show. Anything else is refused by name. Values are read as written, with no precision assumed; private-regression outcomes record differences, and no tolerance is widened for them |
| TBD-17-04-007 | Whether `L=<load name>` is the documented load reference, carried like M and P, and which load case a `FOR` or `MOM` without `FLC` joins; and whether these are stable across versions | Bound imports apply the writer's conventions. Unbound imports refuse both with `MBF-IMPORT-TBD-17-04-007`. Unbound imports may admit both where an explicitly retained bounded profile decision does so |
| TBD-17-04-008 | Whether CAEPIPE's MBF export keeps node numbers, row order and TI bend rows as imported, and whether it writes offsets or absolute coordinates | A sidecar is applied by CAEPIPE name only on explicit request, whatever its digest, and every rebound id is marked unconfirmed (§15). Geometry is read only in its documented forms |
| TBD-17-04-009 | Whether the absolute-coordinate marker `*` precedes or follows the node number in an MBF layout row. The *Node* page and the *DX, DY, DZ* page and MBF examples disagree | The trailing marker shown in MBF examples is a selected interpretation (§10.2). Any other placement is refused by name (`MBF-IMPORT-TBD-17-04-009`) |
| TBD-17-04-010 | Support records whose meaning the public pages leave open: (a) the unit of `ROTK` and `KR`, in-lb/deg or N-m/deg on *Format of .MBF* but inch-lb/radian on the *Anchor* page; (b) whether a `KA` with only one of `TRAK` and `ROTK` leaves the other's DOFs rigid or free; (c) which element's local axes `AXIAL`, `SHEARY` and `SHEARZ` name for a limit stop (the *Guide* page states the preceding element for guides only); (d) the direction of an `LS` without `DV` | Refused by name (`MBF-IMPORT-TBD-17-04-010`). `ROTK(0,0,0)` is read, a zero being free in any unit. The writer writes no rotational spring. Part (a) is drafted as DEL-17-01 CQ-17-01-017 |

### 20.2 Owner choices

- **O-1.** Whether DEC-016's wording ("commercial stress software output parsers") should
  say explicitly that model import is outside it. This Design relies on the 2026-10-26
  ruling.
- **O-2.** English-mode iteration. Agent 0 decided exact normalization to the SI profile
  in the writer (§5.2, implemented). A native English writer mode remains an owner choice.
- **O-3.** The tranche-1 cut (§11, §12, §14): temperature-dependent materials and supports
  beyond anchors and restraints are refused. Pressure and Z vertical are read since
  tranche 2 (§20.3).
- The two §9.4 rulings, corner terminals and the unbound shear model, are decided (Agent 0
  under the owner's delegation, 2026-10-27) and stated in §9.4. Once the pressure-free
  Timoshenko unit (#1663) lands, pressure-free imports follow the same default through
  `model_settings.shear_model`; that is a later slice, not this one.
- **O-7 (ruled: Agent 0's decision under the owner's delegation, 2026-10-27, as recommended below; the ruling is stated in §7.1).** How should a MATERIAL whose temperature rows differ in E
  or α be mapped onto SWBPIPE's `temperature_points` and a case's modulus basis?
  - *What the public pages document.* Each row gives a temperature, E and α (*Format of .MBF*,
    MATERIAL). α is the mean coefficient, not the instantaneous one, and element temperatures
    must lie within the table (`CAEPIPE-MATERIAL`). The page gives no formula for the strain, no
    interpolation and no base temperature of the mean. An analysis option selects E at T1 for
    operating case 1 (hot modulus) or at the reference temperature (cold modulus), and the
    expansion case is operating minus sustained (`CAEPIPE-ANALYSIS`); the MBF carries the
    option as OPTEMP's modulus flag, read today as `delegated` (§6).
  - *What SWBPIPE holds.* A material's `temperature_points` (temperature, E, G or ν, α), and on
    each load case `modulus_basis_ref` (one point) or `modulus_basis_temperature` (linear
    interpolation, no extrapolation). The basis selects E, G and α together, and the thermal
    strain is α(basis) × ΔT.
  - *Options and their numerical consequences* (invented table: E 29.5e6 psi at 70 °F and
    25.5e6 psi at 500 °F; α 6.0e-6 and 7.0e-6 /°F; T_ref 70 °F, T1 500 °F):
    - **A. Hot basis.** The thermal case solves at T1 by interpolation: E 25.5e6, strain
      7.0e-6 × 430 = 3.01e-3. This equals the mean-α strain α(T1)(T1 − T_ref) only when the
      table's mean is taken from T_ref; from another base T₀ the strain is α(T1)(T1 − T₀) −
      α(T_ref)(T_ref − T₀) (from T₀ = 32 °F: 7.0e-6 × 468 − 6.0e-6 × 38 = 3.048e-3, 1.3 %
      higher). For one strain, thermal reactions scale with E: 25.5/29.5 = 0.864, 13.6 % below a
      cold-modulus solve's. Exact for files whose
      modulus flag selects the hot modulus, if the base of the mean is T_ref and CAEPIPE
      interpolates linearly.
    - **B. Cold basis.** The thermal case solves at T_ref: E 29.5e6, but SWBPIPE then takes
      α(T_ref) = 6.0e-6 too, strain 2.58e-3, 14.3 % below the mean-α strain. Not exact for any
      file: SWBPIPE would need a separate α basis (a gap, §20.3).
    - **C. Keep refusing.** Temperature-dependent materials stay out; parity models must be
      written with identical rows.
  - *Recommendation.* C now. Then A, for files whose OPTEMP flag selects the hot modulus, once
    SST confirms (as dossier questions) that the mean is taken from the OPTEMP reference
    temperature and that rows are interpolated linearly. Cold-modulus files stay refused until
    SWBPIPE has a separate α basis. The confirmation is drafted as DEL-17-01 CQ-17-01-016.
- **O-4.** How CAEPIPE-exported MBFs are produced for §18.3: by hand by the owner, or by a
  guest automation step.
- **O-6 (open, not adopted).** Binding the sidecar's digest into the MBF, so that a
  consistent edit (§14) would be refused too. Agent 0 did not adopt it: it costs more than
  it buys, and without signing it still would not stop a deliberate editor. It is revisited
  only if signing is ever wanted.

### 20.3 Next tranches

The first-tranche cut is confirmed as a tranche plan (Agent 0, 2026-10-26). The next
tranche, in priority order, each with its writer counterpart where one is missing:

1. pressure (bar or psi) with SWBPIPE pressure regions, which most real models carry.
   **Implemented** (§9.4): Pressure 1 kept in its unit as exact pressure regions in the
   sustained case under `3.0.0/exact_pressure_v3`, maximal chains with declared closure, and
   the writer's bar output with OPPRES and the sidecar's `pressure_map`;
2. Z-vertical, a routine axis rotation. **Implemented** (§5.1, §10.5): the exact rotation
   (X, Y, Z) → (X, Z, −Y) and the manual's Z-vertical local axes; the writer always writes
   Y vertical;
3. rigid, axis-aligned guides and limit stops. **Implemented** (tranche 3, §12): `G` on an element
   along a global axis → `guide` on its two lateral translations; `LS(0,0)` with an axis-aligned
   `DV` → `line_stop`. Gaps, friction, one-way stops and skewed directions are refused. The
   writer's counterpart already exists (`FIXD` with the family in `support_map`);
4. flexible anchors and springs. **Implemented** (§12): `KA,TRAK(…),ROTK(0,0,0)`, `G` or `LS`
   with `STIFF`, and `K` with an axis-aligned `DV` → `family: spring` supports in N/mm or lbf/in;
   the writer writes springs as flexible anchors, with its preflight, and sidecar format 3 records
   them. Rotational stiffness is refused (TBD-17-04-010);
5. hangers. **Refused** (§12): no exact mapping onto SWBPIPE's hanger members (gaps below);
6. temperature-dependent materials. **Refused under the O-7 ruling** (§7.1, §20.2) until
   CQ-17-01-016 is answered; then the hot basis. Identical rows, already read, are the one exact
   reading.

**Gaps for Agent 0** (tranche 3; each refused by name today). Each states the element, what SWBPIPE
lacks, the smallest member that would carry it, and the ruling (Agent 0's decision under the owner's delegation, 2026-10-27). Each ruled member is a
SWBPIPE model or analysis addition that its owning lane builds, not this lane. The importer keeps
refusing each item by name until its member exists, and then maps onto it.

- **Skewed directions** (a guide on a skewed element; `DV` off the axes; `K` along a skewed
  direction; `LA=1`). SWBPIPE's restraints and springs act on global DOFs only. Smallest member: an
  optional `direction` (a unit vector) on a single-DOF `line_stop` or `spring`, restraining
  translation along it. *Ruling: adopted in principle, second priority.* W-17 scopes it into the
  model-document 0.9.0 step with C1, so that there is one version step, not two; the solver lane
  covers the kernel side.
- **Guide clearance** (`GGAP` > 0). SWBPIPE's `nonlinear.gap` acts on one DOF; a guide's clearance
  is symmetric about its axis in the lateral plane. Smallest member: a two-sided gap on one DOF
  (`gap` with lower and upper limits) would carry an axis-aligned element's clearance per lateral
  DOF only approximately (a square, not a circle); an exact one needs a radial gap over two DOFs.
  *Ruling: stays refused.* A circular clearance is not approximated by a square one; revisited after
  the operating case below, as an exact radial gap.
- **Limit-stop gaps and one-way stops** (`LS(a,b)` with a non-zero or blank limit). SWBPIPE has
  per-DOF `nonlinear` `gap` and `one_way`, but a nonlinear state depends on the combined operating
  load, and the import forms only primitive cases (sustained; thermal). Smallest member: an
  operating case (sustained plus thermal) in which the nonlinear supports are solved, with the
  expansion case as its difference from the sustained one, as `CAEPIPE-ANALYSIS` documents.
- **Friction** (`MU` > 0). Same gap as above; SWBPIPE's `friction` behaviour exists per DOF.
- *Ruling on these two: the operating case with nonlinear supports, expansion as the difference, is
  adopted in principle as the first priority, design first.* It goes to the solver lane, which first
  reconciles it with the W-16 work in flight (#1654, imposed displacement with nonlinear supports;
  EX combinations). It changes the analysis basis, so it needs a design, independent numerical
  scrutiny and targeted regressions before any build.
- **Displacements at a flexible anchor**. SWBPIPE's imposed displacement is a rigid prescribed
  DOF. Smallest member: an imposed displacement of a spring's ground end (`target.type: support`
  on a spring support, moving its ground point). *Ruling: a later item.*
- **Hangers** (`VS`, `CS`, `US`). SWBPIPE's `constant_effort_support` applies its load in every
  solved case, the thermal case included, and its spring hanger's loads are review-only; CAEPIPE
  applies hanger loads in the sustained and operating cases only (`CAEPIPE-HANGER`). Smallest
  member: a case scope on a hanger's load (sustained and operating only), with the spring rate
  added to the stiffness as a spring. `VS` and `CS` loads come from the target's hanger design and
  have no input value to carry. *Ruling: after the operating case.*
- **Rotational stiffness** (`ROTK`, `KR`). Not a member gap: SWBPIPE's rotational springs exist; the
  unit is open (TBD-17-04-010).
- **Temperature-dependent materials, cold modulus**. SWBPIPE's case basis selects E and α together;
  a cold-modulus file needs E at T_ref with the mean α at T1. Smallest member: a separate α basis
  on a load case (O-7, option B). Under the O-7 ruling, cold-modulus files stay refused; no member
  is adopted.

After tranche 2 (Agent 0, under the owner's delegation), the sidecar self-digest is
**implemented** (§15). The writer records format 3: format 2 plus `sidecar_sha256`, the
SHA-256 of the RFC 8785 serialisation of the rest. The importer verifies it before anything
reads the sidecar, on every route. A mismatch, and a format-3 sidecar without the digest,
are refused by name. Format-2 and legacy sidecars import as before and are reported as
carrying no self-digest.

**From the authored cases (§18.3): implemented** (Agent 0's assignment under the owner's delegation,
2026-10-27). Two support spellings in exported MBF files (observed format behaviour) are read in §12:
`STIFF=Rigid` on a guide or limit stop as the rigid support, and a `None` limit of a limit stop as a
blank, one-way limit, refused by name.

Later, in proposed order: concentrated weight and uniform load; contents, insulation and
lining; rigid and valve joints; reducers (0.8.0); node SIF user values (the bend pair travels as
`BSIF`, §11); multiple thermal loads;
intermediate bend nodes; loops.

## 21. Reconciliation with this Design

- **DEL-17-04 `ScopeOfWork.md`:** REQ-012 to REQ-020, verification, acceptance, OUT-002,
  AC-002 and VER-002, TBD-17-04-005 to -009, and the two edited "parser" sentences, as
  applied to that file.
- **DEL-17-04 `deliverable.yaml`:** checks gain `tests/test_caepipe_mbf_importer.py`,
  `tests/test_caepipe_mbf_round_trip.py` and `tests/test_caepipe_private_mbf_import.py`.
  Code paths are unchanged.
- **DEL-17-01:** the seven source rows (§2) and `CAEPIPE-LOCAL-AXES` in `Source_Basis_Register.md`, and
  CQ-17-01-011 to -015 in `CAEPIPE_Question_Dossier.md`.
- **DEL-17-05 Design §5:** the two private run-directory rows. In §10, "two-way
  iteration" joins the remaining items.
- **Writer and pilot tool (implemented):** `family` in `support_map` (§12); primitive
  direction in `load_map` (§15); an explicit material `origin` in `material_map` (§7.4);
  straight-pipe `y_reference` in `element_map` (§10.5); exact US-customary normalization
  (§5.2); the schema-enum version check (§19); and `mbf_sha256` in `id_map.json` (§15).
- **Decomposition:** DEL-17-04's description in `execution/_Decomposition/SOFTWARE_DECOMP.md`
  should include bounded MBF model import alongside export. Its ID, package home and
  supporting scope mappings stay unchanged.

<!-- BEGIN EXTENSION PD-1. Delimited, retained extension; deleting from here to END PD-1 leaves the rest of this Design unchanged. -->

## Appendix PD-1. Extension: two load conventions for unbound `VERSION 15.10` imports

**Status.** Profile decision PD-1 is an ordinary, explicit and bounded design decision of
the implementation manager (2026-10-26), not an owner ruling. It is retained under the
owner's 2026-10-26 public-text ruling (§0). It is an explicitly retained bounded profile
decision in the sense of §0 and §1.

**What it admits.** Two selected interpretations, for unbound imports of the MBF format
version the writer emits: `VERSION 15.10`, as the public import page defines the
`VERSION` field. Bound imports already apply the writer's conventions under the base
rule.

1. **The load reference `L=<load name>`.** It assigns a LOADS record and is carried
   forward until changed, as M and P are.
   - Documented part: the public example MBF (`CAEPIPE-MBF-EXAMPLE`) gives the load name
     as a layout comment on rows that reference LOADS names, and omits it on the rows that
     follow.
   - Corroboration: the target tool assigned a load named only on the first layout row of
     a project-written file to every following element, per private notes.
2. **A `FOR` or `MOM` without `FLC`** belongs to the sustained case.
   - Documented part: `FLC=1` places a force in the sustained case (*Format of .MBF*).
     The Force technical reference documents that sustained-category forces and moments
     also participate in operating cases (`CAEPIPE-FORCE`).
   - Behaviour basis: the target tool treated a project-written force without `FLC`, and
     likewise a project-written moment without `FLC`, as sustained loads, per private
     notes. No public source states which case an MBF force or moment without `FLC`
     takes.
   - Any other explicit `FLC` value stays refused.

**What stays refused.** For any other MBF version, both conventions are refused by name
(`MBF-IMPORT-TBD-17-04-007`). TBD-17-04-007 remains open, asking SST (CQ-17-01-013) to
confirm that the conventions are documented and stable across versions.

**Effect on the rest of this Design:**

- §1, §10.3, §13 and §17: unbound `VERSION 15.10` imports admit both conventions. Each use
  is recorded in `assumptions` with PD-1 as its basis.
- §18.1(a′): the two refusal checks are replaced by canonical-equality checks of
  import(W) without a sidecar, using the writer's `node_map` for correspondence. The
  refusal checks remain for any other version.
- §18.3: exported `VERSION 15.10` MBFs carrying these records are imported, not recorded
  as refused.
- §20.1, TBD-17-04-007: its effect column reads "admitted for unbound `VERSION 15.10`
  imports under PD-1".

**Reconciliation belonging to PD-1.** The named detail of the two behaviour facts above
stays in private notes. DEL-17-05 Design records nothing about them in public text. This
appendix cites only "private notes".

<!-- END PD-1 -->
