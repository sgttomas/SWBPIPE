# CAEPIPE MBF model import (DEL-17-04)

**Basis:** branch `claude/piping-caepipe-pilot-20261026` at `71272e769c` (PR #1652, current
`main` merged), which adds the SI writer `core/handoff/caepipe_mbf/si_writer.py`. Model
document 0.7.0 (DEL-02-01 `Work/model-document-0.7.0.md`); 0.8.0 as proposed in open
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
modulus option of §20.2 (O-7), within its existing boundary. *Historical statement, kept and
marked (2026-11-02):* the modulus option, and the expansion case that O-7 and §20.4 also take
from that page, rely on `CAEPIPE-ANALYSIS` content outside its current registered boundary
("analysis-option semantics only: number of thermal loads, gravitational acceleration and
direction"), pending the G-2 proposal (DEL-17-01 `Source_Basis_Register.md`, **Proposed
boundary change**). Historical use does not establish admission (Agent 0, 2026-11-02;
paraphrase).

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
  infinity. A numeral is read without computing anything proportional to an untrusted length
  or exponent before bounding it, in the Rust engine's order (`parse_fraction`): each digit run
  beyond Python's default 4300-digit `int()` limit raises `int()`'s `ValueError` before it is
  converted, as `Fraction` raises it; a zero mantissa is 0 at any exponent; and a non-zero
  numeral whose decimal exponent (its exponent less its fraction digits) lies beyond ±10⁵, or
  whose magnitude lies beyond 10^±400, is far outside binary64 and is refused by the same code
  before its exact value is formed, so an exponent at the i64 limits such as
  `1e-9223372036854775808` never computes a power of ten. English decimal feet and the
  zero tests of numerals read the same digit-run checks first; a numeral's digit count is that
  of its value, so a Unicode zero digit leads like an ASCII one (as in the Rust engine), though
  the import refuses a non-ASCII numeral before reading it (`MBF-IMPORT-CHARACTER-UNSUPPORTED`). Every derived value is rounded through the same guard, which refuses by the same
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
does for entered units (DEL-02-02 `Work/project-unit-selection.md` §2, items 6 and 8).
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
| OPTEMP thermal cycles, solve-thermal flag, cold-modulus flag | `delegated`. The cold-modulus flag is field 5; the page codes it 0 Off and 1 On, gives no default, and says it is ignored for six listed piping codes (§20.4.9, G-1). The import treatment is unchanged |
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
  Under D-2 and the Route 1 ruling (§20.2), a bound import of a Route 1 file (sidecar format 5)
  restores a constant-α, temperature-dependent modulus exactly (§20.4.4; designed, not
  implemented). Unbound imports are unchanged.
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
   - acos, tan and the importer's other trigonometric functions (asin, atan2, sin), and the
     SI writer's, are correctly rounded to binary64 (`core/handoff/caepipe_mbf/correctly_rounded.py`),
     not taken from the platform C library, so the Python and the Rust engine produce the same
     bits on every platform (parity ruling (a)).
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
a bound import of a format-2, format-3 or format-4 sidecar has the writer regenerate the sidecar from the imported
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
unchanged: ids (format 4's section ids included), `reversed`, an authored `y_reference` or
`family`, a recorded `origin`, a thermal direction, the shear model of a contract, region ids,
and closure transfers with their provenance. Such a value regenerates as the sidecar gives it, so an edit of one imports
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
`MBF-SI-PRESSURE-REGION-ZERO` and `MBF-SI-SHEAR-MODEL-NOT-EXPORTED` (omitted). Sidecar format 4 (§15)
adds the writer's `MBF-SI-SECTION-BINDING-UNSUPPORTED` and `MBF-SI-SECTION-REFERENCE-STALE`.

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
`sidecar_format` = 4: format 3 plus `section_map` (**Format 4: section identities**, below).
Format 3 is format 2 with its self-digest (below). A format-2 sidecar
holds what a bound import restores and the MBF does
not carry, every field with its effective value; it never uses null, or another sentinel, as
a value a default then fills, and it does not repeat what the MBF determines (layout order,
record names, material and section numbers, bend geometry, CAEPIPE kinds and names; no
`section_map` before format 4). Its fields:

- `node_map`: id → CAEPIPE name;
- `element_map`: id → `from`, `to`, `reversed`, and for a straight its effective
  `y_reference` (the authored vector, or else CAEPIPE's documented local y, derived from the
  written MBF's own positions as an import derives it) with `y_reference_authored`, for a
  bend `bend` with its `component` id;
- `support_map`: id → `node`, `restrained`, the effective `family` (the authored one, or the
  solve's inference: anchor with six DOFs, else guide) and `family_authored`. Formats 3 and 4 also
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

The parser refuses a format-2, format-3 or format-4 sidecar missing any field, with null in any (but a
format-4 record's null library section), or with a field
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
sidecar is a format-2 sidecar plus one field, `sidecar_sha256`; format 4 carries it too. Its value is the lowercase
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
  of the `mbf_render.json` beside it, and that companion is parsed strictly too. A format-2,
  format-3 or format-4 sidecar always recorded its own `mbf_sha256` and never takes the
  companion's. Deleting it is a partial edit: formats 3 and 4 refuse it by the self-digest,
  and format 2 by the parser.
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
leaves a sidecar of another MBF unapplied. A sidecar claims a self-digest when
`sidecar_format` is the integer 3 or 4. Such a sidecar is refused, and nothing of it is
applied, in three cases:

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
Only formats 3 and 4 have `sidecar_sha256`: a format-2 or legacy sidecar that carries it is
refused by path. The writer's preflight (§9.4) re-imports the writer's own output with its
self-digest.

*What it proves and what it does not.* The self-digest detects accidental or partial edits:
a changed byte, a dropped map, or a sidecar paired with another sidecar's digest. It is not
a signature. A deliberate editor can recompute it, or remove it and claim format 2. Meaning
is therefore still governed by the regeneration identity and the agreement rule (§14),
over exactly the values they governed before. Authorship is still not claimed. Key order,
whitespace and the JSON spelling of a number are not content. The sidecar reaches the
importer as a parsed JSON object, and the canonical form sorts its members and writes the
value of each number.

**Format 4: section identities** (Q-2; A-3; desktop design §1.3 C6-5 and §4.4). Implemented
in both twins, the Python and the Rust engine crate.

*What format 4 records.* Format 4 is format 3 plus one map, `section_map`: PIPE record name
→ `{"swbpipe_section": <section id or null>}`. The id is the library section
(`sections[].id`) that every pipe written with that record references through
`section_ref`. Null states that those pipes have no `section_ref`; nothing fills it. Every
record the layout uses has exactly one entry, and there is no other entry. Element entries
are unchanged: a pipe's `section_ref` follows from its record (the layout's `P` carry) and
that record's entry, so it is not repeated per element. The map carries one identity per
record and the binding of pipes to it. It carries no section content.

*How the writer forms records.* The writer keys PIPE records by (`section_ref` or none, OD,
wall) instead of (OD, wall). Records are still numbered `P1`, `P2`, … in layout order.
Two library sections with equal OD and wall therefore get two records with equal values,
and so does a referenced section and an unreferenced pipe of the same OD and wall. One
record whose sidecar entry lists several sections, with a `section_ref` per element, was
rejected for these reasons:

- It is the writer's rule for materials already: one record per material id, and
  `material_map` maps the record name to `swbpipe_material`. `section_map` is then an
  identity map keyed by CAEPIPE name like the others, with no per-element copy that would
  need its own agreement check.
- A bend's pipe and its incoming straight are one layout row with one record, so they must
  already share a record (`MBF-SI-BEND-UNSUPPORTED`). Bend authoring already requires them
  to share `section_ref` (`OP-BEND-CORNER-SECTION-MISMATCH`). Keying by `section_ref` can
  newly refuse only a bend whose two pipes differ in `section_ref` alone, which authoring
  does not produce.
- The MBF itself keeps the sections apart, so an unbound import and the target program
  see as many sections as the library binds.
- The MBF text is unchanged for every model in which all pipes of one OD and wall share one
  `section_ref`, or none.

The writer reads `model.sections` as a list of records with string ids; another shape is
`MBF-SI-INPUT-INVALID`. A null `section_ref` is absent, as the solve reads it. A pipe with
`section_ref` must bind as the solve binds it (`resolve_shared_sections` in
product_physics). Otherwise the writer refuses, because the import could not restore it:

- `MBF-SI-REFERENCE-UNRESOLVED`: `section_ref` is blank, or does not name exactly one
  library section (the solve's `SECTION_REFERENCE_INVALID`);
- `MBF-SI-SECTION-BINDING-UNSUPPORTED`: the section's `section_type` is not `pipe`, or its
  `properties` are not exactly `outside_diameter` and `wall_thickness`, each a length
  quantity (`SECTION_BINDING_UNSUPPORTED`, `SECTION_REFERENCE_INVALID`);
- `MBF-SI-SECTION-REFERENCE-STALE`: the pipe's inline OD or wall differs from the section's
  in value or unit (`SECTION_REFERENCE_CACHE_STALE`).

All pipes of one section therefore have one OD and wall, and each section id is on exactly
one record.

*What a bound format-4 import restores.* For each entry with an id:

- one `sections[]` entry:
  - `id` from the sidecar;
  - `section_type` `pipe`, the only type a `section_ref` binds;
  - `properties` `outside_diameter` and `wall_thickness`, each the same quantity the import
    writes into the inline section of the record's pipes: the record's value in the
    import's length unit. The binding is therefore never stale;
  - `name` the PIPE record name, as every imported entity is labelled by its CAEPIPE name;
  - `provenance` the PIPE record's line (§16.1).
- `section_ref` set to that id on every pipe written with that record.

The `sections` entries follow record order. The member is written only when it is
non-empty, so an unbound import, a format-2 or format-3 import and a format-4 sidecar
whose entries are all null give exactly today's output. The output `id_map` gains
`section_map` (record name → id) only when an id is restored.

Inline sections are formed exactly as today (OD, wall, and §8's corrosion-allowance rule).
A library entry holds only OD and wall, the two properties a binding admits. It never
holds a value that the inline section lacks or contradicts, and the importer invents no
value. These are not restored and stay loss entries: a section's name, provenance, tags,
notes and `provenance_record`; library sections no pipe references; and the section fields
the PIPE record does not carry (below). The identity file carries none of them. A field
the MBF lacks is not moved into it.

*Rebind route.* Section ids are applied by PIPE record name, as materials are, and each is
reported `rebound_unconfirmed`. An entry naming a record that the file lacks is
`MBF-IMPORT-SIDECAR-ENTRY-ABSENT` (`omitted`). A record without an entry binds no library
section.

*Unchanged.* An unbound import forms no `sections` and no `section_ref`. Format-2, format-3
and legacy sidecars import exactly as before, with no library and no `section_ref`. A
format-2 or format-3 sidecar carrying `section_map` is refused by path, as today. A legacy
`section_map` keeps its legacy shape and checks.

*Refusals for a malformed format-4 `section_map`.* All come before assembly unless noted.

- `MBF-IMPORT-SIDECAR-INVALID`, by JSON path:
  - `section_map` missing;
  - `section_map` not an object, or an entry that is not an object, has a key other than
    `swbpipe_section` or lacks it;
  - `swbpipe_section` neither null nor a string that is non-empty after trimming;
  - an empty record name;
  - an id that two entries give, at the later entry's `swbpipe_section` (one-to-one, as
    `node_map` is).
- `MBF-IMPORT-SIDECAR-CONFLICT`:
  - bound, an entry for a record that the file lacks or that no element uses, or a record
    an element uses with no entry, at `section_map`, naming the records (as the other maps'
    completeness is refused);
  - a restored id equal to another restored or formed id, at `MBF section <name>`, or at the
    later entity that takes it.
- The self-digest, exactly as for format 3: missing or malformed is `-INVALID`, and a wrong
  digest is `MBF-IMPORT-SIDECAR-DIGEST-MISMATCH`.
- After assembly, `MBF-IMPORT-SIDECAR-NOT-REPRODUCED` for anything else the writer would
  not reproduce. For example, two null entries for records of equal OD and wall are
  refused, because the writer would write one record (the first differing path is the
  later record's entry, such as `section_map[P3]`).

*Regeneration identity and self-digest.* The writer records format 4. A bound import of a
format-2, format-3 or format-4 sidecar regenerates, and the regenerated sidecar is
compared in the given sidecar's format. For formats 2 and 3 it is compared without
`section_map`. Their imports restore no `section_ref`, so the writer keys the records by OD
and wall alone, as the writer that wrote them did. For format 4, `section_map` regenerates
from the restored `sections` and `section_ref`: each pipe has the same key, in the same
layout order, so each record has the same name and entry. Section ids pass from the model
to the sidecar unchanged, so §14's consistent edits gain section ids. A renamed id, or an
entry set to null where no record merge results, imports as stated. The self-digest is
unchanged: the same canonical serialisation of the whole sidecar without
`sidecar_sha256`, which now covers `section_map` too. A sidecar claims a self-digest when
`sidecar_format` is 3 or 4. The seeded mutation test gains a format-4 fixture with two
sections of equal OD and wall and an unreferenced pipe of the same OD and wall.

*Criterion 6.*

- C6-5: section ids are restored, and no section identity is reported unrecoverable.
- C6-6: §18.2's canonical form adds, when bound, each pipe's `section_ref`, and each
  referenced section's id with its OD and t (printed-directly bound). Its exclusions gain
  section names, section provenance and library order. Unreferenced library sections are
  an omission the writer lists.

*The writer's loss entries after the change.*

- `sections` is no longer a `MBF-SI-MEMBER-NOT-EXPORTED` member. Its handled fields are
  `id` (the sidecar), `section_type` (`pipe`, implied, otherwise refused) and `properties`
  (written as the record's OD and wall).
- `pipe_segments.section_ref` is handled: written as the pipe's record and that record's
  `section_map` entry. It gets no loss entry.
- Library sections no pipe references get one `MBF-SI-FIELD-NOT-EXPORTED` at
  `model.sections`, naming each id, as unused materials do.
- Through the sweep, `sections.name` gets `MBF-SI-FIELD-NOT-EXPORTED` ("the PIPE record has
  no name field; the import names the section by its record"). `provenance`, `tags`,
  `notes` and `provenance_record` are listed as for any entity.
- A bend's pipe and its incoming straight that differ in `section_ref` alone are refused as
  `MBF-SI-BEND-UNSUPPORTED`, whose text says so ("section (or its section_ref alone)").

*PipeSection fields in the PIPE record today* (unchanged here; widening this coverage needs
source admission and is a separate increment):

- Written: `outside_diameter` (field 2) and `wall_thickness` (field 3), in mm.
- Written as 0, refused if non-zero (`MBF-SI-SECTION-UNSUPPORTED`): `corrosion_allowance`
  (field 4; under the exact pressure contract an absent one is refused,
  `MBF-SI-CORROSION-ALLOWANCE-MISSING`), `mill_tolerance` (field 5) and
  `insulation_thickness` (field 7).
- Omitted (`MBF-SI-SECTION-FIELD-NOT-EXPORTED`): `material_density`, `contents_density`
  and `insulation_density`. Field 6 is written as 0, and weight travels only as weight
  primitives.
- Fields with no SWBPIPE counterpart: lining density and thickness (fields 8 and 9) are
  written as 0, and soil (field 10) is not written.

The reference system's sections (DEL-09-01 §4.2) state a non-zero mill tolerance,
corrosion allowance and insulation, so the writer still refuses them after this change.

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
  file (§9.4);
- format 4: library sections and each pipe's `section_ref`, from `section_map`
  (**Format 4: section identities**).

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
  `sidecar_sha256` (**The self-digest**), format 4 format 3 with `section_map` (record name →
  `swbpipe_section`, a non-empty id or null; **Format 4**); a legacy sidecar holds the seven
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
  also `family_authored`; formats 3 and 4 also `spring`, only on exactly one translation with
  `family_authored` true); the supports at each node
  together must pass the solve's support rules (§12). A bound import compares the file's rigid
  DOFs with the union of the non-spring entries at the node, and its spring DOFs with the spring
  entries, one each (`MBF-IMPORT-SIDECAR-CONFLICT`); a restored spring takes its stiffness from
  the file;
- `load_case_map`: id → label;
- `material_map`: name → `swbpipe_material` id, the four numbers the writer records,
  `temperatures_c`, `elements` and `origin` (one of `MATERIAL_ORIGINS`, format 2 also
  `"undeclared"`; legacy optionally absent);
- `section_map`: legacy, name → its two numbers and `elements`; format 4, name →
  `swbpipe_section`;
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

A seeded mutation test exercises the parser on the writer's format-4 sidecars of seven invented files
(pressurized, pressure-free, with a contract and no region, with an authored non-default
orientation, with two forces the MBF sums, with two springs, tranche 3, and with two library
sections of equal OD and wall and a pipe of the same OD and wall with none, format 4). Each field is swapped for every other JSON type, each container emptied, each
number made huge or non-finite, an unknown key added and each key removed, and each of the
nine maps deleted and emptied. Each import passes a closure declaration, so a bound import
that fell through to unbound formation would succeed and be caught. Every mutated import is
refused with a named `MBF-IMPORT-*` code and never raises. The exceptions are consistent
edits (§14), which import as the model they yield: removing one of the two summed entries,
and giving a record's null library section an id (format 4). An id set to null is refused in
these files, since its pipes would merge with a null record of the same OD and wall. Inside each entry reported as summed, the test also changes the category, the
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
still claims format 3 or 4 with a well-formed digest and canonical content, the self-digest
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
  supports by node with their DOF split and family, bends by TI number, materials by name,
  and format 4's library sections by PIPE record name. It restores no case or primitive id, orientation, origin or pressure state: those are
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
`nA`/`nB`), element, support, bend, material and load case, `section_map` (PIPE record →
section id) when a format-4 sidecar restores a section, and `region_map` (region id →
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
| `OPTIONS HGRA,SI`, or `HGRA,<code>,SI` with the caller's documented piping code (and `OPCODE 0,0,0`, B31J off, for an ASME B31 code; no code is stated by `MBF-SI-PIPING-CODE-NOT-WRITTEN`; DEL-17-05 Design §11.2); always Y vertical; X, Y, Z unchanged (§5.1) | SI mode; Y vertical, or Z rotated exactly (X, Y, Z) → (X, Z, −Y); HGRA, the piping code and OPCODE are `delegated` |
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
| `sections` (library) `id`, `section_type`, `properties`; `pipe_segments.section_ref` | Written / refused | A `PIPE` record per library section (or none) and OD and wall; the record's section id in the sidecar's `section_map` (format 4, §15); a binding the solve refuses is refused (`MBF-SI-REFERENCE-UNRESOLVED`, `MBF-SI-SECTION-BINDING-UNSUPPORTED`, `MBF-SI-SECTION-REFERENCE-STALE`) |
| `sections` that no pipe segment references | Omitted (`MBF-SI-FIELD-NOT-EXPORTED`, entity `model.sections`, naming each id) | The MBF writes only the sections its pipes reference |
| `sections` `name`, `provenance` (and `tags`, `notes`, `provenance_record`) | Omitted | The PIPE record has no name field; the import names the section by its record and its line |
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
| A support, rigid or spring, at a node no pipe segment reaches | Refused (`MBF-SI-TOPOLOGY-UNSUPPORTED`) | No layout row carries it; a spring alone there raised KeyError before |
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
   parser's spring vocabulary is refused by path outside formats 3 and 4.

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
  model is outside the subset. One exception: on a bound format-4 import, PIPE records
  that print equal values are distinct when their `section_map` entries differ (§15,
  format 4). The restored section identity keeps them apart, and re-export writes them
  as distinct records again. Two such records with equal entries, or two whose
  differing values collide only in print, are still outside the subset.

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
  `pressure_map`);
- section names, section provenance and library order, and library sections no pipe
  references (the writer lists them as omitted).

Each exclusion is listed in the test so that it stays explicit.

**Canonical form.** Both models are reduced to this subset, in SI base units:

- node positions;
- per pipe: endpoint pair, oriented only when the sidecar is bound; OD; t; E; ν, the
  writer's, given or derived; G; and α; when bound, its `section_ref` (format 4, §15);
- when bound, per referenced library section: its id with OD and t, each with the
  printed-directly bound of its pipes' PIPE record;
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
- **O-7 (ruled: Agent 0's decision under the owner's delegation, 2026-10-27, as recommended below; the ruling is stated in §7.1;
  narrowed by D-2, 2026-10-31, and the Route 1 ruling and its refinements, 2026-11-02, all below and in §20.4).** How should a MATERIAL whose temperature rows differ in E
  or α be mapped onto SWBPIPE's `temperature_points` and a case's modulus basis?
  - *What the public pages document.* Each row gives a temperature, E and α (*Format of .MBF*,
    MATERIAL). α is the mean coefficient, not the instantaneous one, and element temperatures
    must lie within the table (`CAEPIPE-MATERIAL`). The page gives no formula for the strain, no
    interpolation and no base temperature of the mean. An analysis option selects E at T1 for
    operating case 1 (hot modulus) or at the reference temperature (cold modulus), and the
    expansion case is operating minus sustained (`CAEPIPE-ANALYSIS`); the MBF carries the
    option as OPTEMP's modulus flag, read today as `delegated` (§6).
    *Historical statement, kept and marked (2026-11-02):* the modulus option and the expansion
    case in this sentence rely on `CAEPIPE-ANALYSIS` content outside its current registered
    boundary, pending the G-2 proposal (DEL-17-01 `Source_Basis_Register.md`, **Proposed
    boundary change**). Historical use does not establish admission (Agent 0, 2026-11-02;
    paraphrase). The flag's position, OPTEMP field 5, is admitted from `CAEPIPE-IMPORT-MBF`
    (§20.4.9, G-1 (a)).
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
  - *Wording of the entries below.* Every entry below paraphrases Agent 0; none quotes Agent 0,
    and none is labelled accepted wording. Text in quotation marks quotes the label or document
    text named beside it, not Agent 0's message. The one exception is the binding requirement for
    sidecar format 5, which §20.4.5 quotes as the lane's brief gives it. Decision 3 is a
    paraphrase too: its text was normalized from Agent 0's delivered message, so it is not
    literally verbatim. D-2's
    historical wording (2026-10-31) was reconstructed from Agent 0's thread; neither its date
    nor its text has been verified against the original messages, and this Design does not
    mark either as verified. Agent 0's confirmation of 2026-11-02, below, supplies the current
    decision basis for D-2's substance without standing in for its historical words. The Route 1
    ruling, the refinements and the decisions of 2026-11-02 are paraphrased from the lane's
    briefs.
  - *D-2 (attributed to Agent 0, 2026-10-31; paraphrase reconstructed from Agent 0's thread;
    date and text unverified).* Agent 0 accepted the narrower rule: a modulus that varies with
    temperature, with a constant expansion coefficient, is written and read back exactly. Two
    conditions apply:
    - materials whose expansion varies stay refused and are written up as a release limitation
      for the owner (criterion 3), whose acceptance is pending;
    - the rule is written for materials in general and tested on more than the reference
      system's material, so that criterion 6 cannot pass only because the fixture was tailored
      to it.
  - *Confirmation of D-2's design intent (Agent 0, 2026-11-02; paraphrase).* Agent 0 confirmed
    the substantive narrow design intent: a modulus that varies with temperature, with a constant
    expansion coefficient; a general mapping, not tailored to the fixture; mapping only where
    exact per-member thermal, modulus and source proof holds; varying α and the cold-modulus
    basis refused; acceptance pending. This is the current decision basis. It does not verify
    D-2's historical date or wording. It does not complete criterion 6, and it closes no
    dossier question.
  - *Route 1 (Agent 0, 2026-11-02, binding; paraphrase).* Design comes first, through the
    load/reference-state group, with a per-member faithful-equivalence condition rather than
    one inferred case T1. Only an explicitly designed subset is admitted:
    - one selected thermal case;
    - a constant-α basis;
    - each exported member's modulus-selection temperature equal to that member's mapped MBF
      T1. This is a per-member rule with one bounded exception, not an unconditional
      temperature rule: an `explicit_base_properties` selection on a material with no
      temperature points has no selection temperature, and it is eligible only under
      decision 3 below (§20.4.2 e).

    A selection that differs is refused by name with the member's id. The other terms of the
    ruling:
    - `unchanged_reference` is not equated with T_ref until a reference configuration
      establishes the mapped global reference, and the writer's 20 °C default never supplies an
      equivalence;
    - multiple reference datums or configurations, explicit interval strain, free-length
      states and the like are mapped only where equal strain and modulus semantics are proved,
      and refused otherwise;
    - Route 2 (a case-level `modulus_basis_temperature` with thermal primitives) stays refused;
    - Q-5 is not adopted;
    - existing formats and ordinary routes are unchanged.

    The design is §20.4 (designed, not implemented). Cold-modulus files stay refused as above.
  - *Refinements (Agent 0, 2026-11-02, binding; paraphrase).*
    - **Decision 3 (paraphrase, normalized from Agent 0's delivered message; not literally
      verbatim).** An `explicit_base_properties` selection on a material with no temperature
      points is eligible for the same-T1 condition only if the actual selected E basis is
      constant and exactly representable by the mapped MBF material semantics, with the other
      relevant material-property and thermal-law conditions satisfied separately. All
      properties are not to be inferred constant from missing E points; the explicit
      per-member basis and α restrictions remain. Counterexamples are to be added for
      conflicting selectors and record bases. It is the basis of the bounded exception above.
    - **Derived T1 rows** use SWBPIPE's exact existing interpolation semantics and units, with
      representable output precision, and are tested at endpoints, interior, a duplicate T1, a
      T1 colliding with an authored row, a shared material and the round trip. Their admission
      basis is Agent 0's decision below (option (a), exact). CQ-17-01-016 is not retired
      automatically. It may be marked inapplicable to this precisely bounded write route only
      after admitted format semantics prove that the target selects the exact supplied T1 row,
      with no other interpolation or modulus convention. Until then it stays open (G-4,
      §20.4.9).
    - **Sidecar format 5** binds the source and export bytes, the identities and the
      derivation provenance, so that an import strips only proven generated rows and restores
      the authored table's order and meaning. It never strips by guessing, and a forged or
      mismatched sidecar is refused. Carrying no independent numerical authority is not by
      itself a reversibility proof. Format 5 is eligible under the existing interoperability
      scope. Before implementation it needs independent persistence scrutiny and explicit
      backwards compatibility and refusals: formats 2–4 unchanged, and a stated outcome for a
      format-4-only importer that meets format 5. There is no silent product-schema bump.
      §20.4.5 states how a generated row is proved: against a source the user explicitly
      designates, never by the sidecar's self-consistency alone. It also states exactly what
      the refusal of a forged or mismatched sidecar covers (**What "no forged sidecar
      acceptance" covers**).
    - **Release limitation.** Varying α and cold-modulus files go to the criterion 3 list,
      with the owner's acceptance pending. No acceptance is inferred. The proposed entry is in
      §20.4.10, for the implementation PR.
    - **The operating case.** Designing the operating W+P+T mapping is approved as a separate
      design-first item toward criterion 6. Thermal-only Route 1 does not complete the
      reference system.
  - *Decisions (Agent 0, 2026-11-02, after reading the `ce093a4c7` diff; paraphrase).*
    - **Derived T1 rows: option (a), exact admission, is chosen.** A derived row's printed E,
      round-tripped through its printed unit, must equal the solver's actual selected binary64
      E on both the exported and the restored model, with the conversion and rounding
      semantics stated (§20.4.2 e). Otherwise the named member is refused. There is no §18.2
      precision relaxation. Option (b), print precision, is rejected (§20.4.10). A random
      sample's admission rate is sample evidence only, not a bound or a justification. Nothing
      is tuned to RS-M1.
    - **G-4 stays open.** The mathematical argument that a row at T1 makes interpolation
      irrelevant is not sufficient without selected target semantic evidence. Any source
      consultation for G-1 and G-4 stays strictly within each registered row's current boundary
      (DEL-17-01 `Source_Basis_Register.md`). Being on a registered page does not admit its
      other subsections. Target row-selection or point-value behaviour that the boundary does
      not explicitly support becomes a dossier question or a source-boundary proposal before
      broader use (§20.4.9).
    - **DEL-09-01 §10's note** is kept below the table, verbatim, with the fixture unchanged,
      and is labelled an "MBF consumer-coverage note". It is not a pending-family generator
      entry and not a completed admission. §11.1's wording ("planned for the restricted subset
      defined by") is correct.
    - **Wording authenticity.** D-2's historical attribution stays a paraphrase, unverified;
      decision 3 is a paraphrase; nothing implies that criterion 6 is complete or that a dossier
      question is closed (the wording note above).
  - *Source proof and source boundary (Agent 0, 2026-11-02, after the independent review at
    `0169db35e`; paraphrase).*
    - **A verified source for every bound format-5 import with temperature-point materials.**
      Option (a) of the review is chosen: such an import is refused unless a verified source is
      available, and the relabel-as-point residual is not accepted. A cross-machine import
      requires the user to supply that source; it is not impossible. A source the user supplies
      passes the same hash and content checks as any other (§20.4.4, §20.4.5).
    - **`CAEPIPE-ANALYSIS`'s boundary.** The source-boundary conflict that §20.4 exposed is folded
      into the planned G-2 register-change PR. §20.4 stays blocked on that evidence until the
      exact expansion passes independent IP review and the applicable adoption authority.
      Historical use does not establish that a statement was within the registered boundary.
  - *Source association (Agent 0, 2026-11-02, after reading `f3d08f015` and its independent
    re-review; paraphrase).*
    - **The digest verifies, never selects.** The rule is correct.
    - **Explicit user designation is approved as the initial bound-import route.** The user
      designates the source independently, before the digest and content checks. A mismatch,
      or no association, is refused. The relabel-residual withdrawal and the G-2 block stay.
    - **The trusted export ledger (store v14) is not adopted.** It is eligible only for a
      separate design review, and this Design grants no persistence, schema or retention
      authority. Turning unsaved text into a `handoff_basis` state is a new write, consent and
      storage effect, not automatically equivalent to Record Run's authority. The exact design
      and its old and new commitments go to independent review and Agent 0's adoption before
      any migration or command; there is no current shared-store grant. §20.4.5 lists what that
      design must establish.
    - **Retention (Open 1).** Indefinite retention or non-deletion is not adopted by analogy.
      The designation route keeps no export record. Current deletion behaviour is preserved,
      and restricting an existing authorized delete path needs exact review.
    - **Source-free exports (Open 2)** may remain offered if otherwise eligible. The complete
      review clearly notices that a later bound import requires independent source designation
      and access to the exact source, and that no automatic round trip is promised. The
      existing format and export authority are preserved, with no unstated source persistence.
    - **No digest-lookup fallback (Open 3).** Until a separate ledger design and implementation
      is adopted, imports with points require designation.
    - **Point-free materials (Open 4)** are not approved just because they have no points. The
      exact format-5 fields and what they restore are reviewed; if format 5 restores thermal,
      reference or selection metadata beyond format 4's consistency level, designation is
      required for that too, or the path stays refused. Points were the minimum mandatory
      guard, not a waiver for other metadata. §20.4.5 (**Point-free materials**) gives the
      analysis and the rule chosen.
  - *Point-free rule and format-5 contract (Agent 0, 2026-11-02, after the independent reviews
    at `147d71586` and `2dc8d26aa`; paraphrase).*
    - **Every bound format-5 import requires independent source designation,** followed by the
      hash and full-content checks, whether or not any material has temperature points. Without
      designation, only an already-supported unbound import is permitted, and it reports the
      lost metadata explicitly; no unbound route is added implicitly. Formats 2 to 4 are
      unchanged, and the stronger assurance is scoped to format 5 against the designated source.
    - **Confirmed:** `derivation.render_profile`, `derivation.export_inputs` and the code
      `MBF-IMPORT-RENDER-PROFILE-UNSUPPORTED` fit the source and export binding design.
    - **Correction:** a profile check catches a declared mismatch, not a forgotten version
      bump. Deterministic re-render equality is required, with tests that detect a rendering
      change made without the required profile update (§20.4.5, **Writer-build drift**;
      §20.4.7 item 5a).
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
6. temperature-dependent materials. **Narrowed by D-2 (attributed to Agent 0, 2026-10-31;
   wording unverified), Agent 0's confirmation of its design intent, and the Route 1 ruling, its
   refinements and decisions (Agent 0, 2026-11-02); designed, not implemented (§20.4). All are
   paraphrased in O-7, decision 3 included.**
   - **What Route 1 writes.** A temperature-dependent modulus with a constant expansion
     coefficient is written through the load/reference-state group: one thermal case, a
     constant-α basis, and the per-member rule. Each exported member's modulus-selection
     temperature equals its MBF T1, and the E the solver selects equals the E of the MBF row at
     T1 exactly (§20.4.2 e, g). A selection that differs is refused by name with the member's id
     (`MBF-SI-MODULUS-SELECTION-MISMATCH`). The one bounded exception is an
     `explicit_base_properties` selection on a material with no temperature points, eligible only
     under decision 3's conditions (§20.4.2 e). Derived MATERIAL rows at a member's T1 are
     admitted only exactly (option (a), Agent 0, 2026-11-02), and remain conditional on G-4.
   - **Imports.** Bound imports restore the group's inputs exactly through sidecar format 5.
     Every bound format-5 import needs a source the user explicitly designates, verified by hash
     and then by content, whether or not any material has temperature points; otherwise it is
     refused, and a derived row is stripped only when that source proves it generated (§20.4.4,
     §20.4.5). The sidecar's digest never selects the source. Unbound imports keep refusing a
     temperature-dependent MATERIAL (§7.1).
   - **What it waits on.** The implementation waits on G-1 (b), (c) and (e) (the value that
     selects the hot modulus, an omitted flag, a null piping code) and G-4 (proof that the target
     uses the exact supplied T1 row), §20.4.9. CQ-17-01-016 stays open. A
     sustained case beside a varying modulus waits on G-2.
   - **Still refused.** Route 2 (a case-level modulus basis with thermal primitives); varying α
     and cold-modulus files, which go to the criterion 3 list with the owner's acceptance
     pending.
   - **Not covered.** The operating case (W+P+T) is a separate design-first item toward criterion
     6. Thermal-only Route 1 does not complete the reference system.
   - **Ordinary route.** Identical rows, already read, remain its one exact reading.

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
  expansion case as its difference from the sustained one, as `CAEPIPE-ANALYSIS` documents
  (outside that row's current boundary, pending the G-2 proposal; §2).
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
carrying no self-digest. Sidecar format 4 (§15, **Format 4: section identities**) followed
for criterion 6 (Q-2): the writer records it, and formats 3, 2 and legacy import as before.

**From the authored cases (§18.3): implemented** (Agent 0's assignment under the owner's delegation,
2026-10-27). Two support spellings in exported MBF files (observed format behaviour) are read in §12:
`STIFF=Rigid` on a guide or limit stop as the rigid support, and a `None` limit of a limit stop as a
blank, one-way limit, refused by name.

Later, in proposed order: concentrated weight and uniform load; contents, insulation and
lining; rigid and valve joints; reducers (0.8.0); node SIF user values (the bend pair travels as
`BSIF`, §11); multiple thermal loads;
intermediate bend nodes; loops.

### 20.4 Load/reference-state thermal export (D-2 Route 1): designed, not implemented

**Status.** Design only. Nothing in this subsection is implemented, and no code, fixture,
schema or ScopeOfWork changes with it. Basis: `main` at `50cfc630b`. It applies the rulings
below (O-7, §20.2). Every ruling below is a paraphrase of Agent 0, decision 3 included; none
quotes Agent 0 (O-7's wording note). The one quotation is the binding requirement for sidecar
format 5 (§20.4.5).

- **D-2 (attributed to Agent 0, 2026-10-31; paraphrase; date and text unverified).** A modulus
  that varies with temperature, with a constant expansion coefficient, is written and read back
  exactly. Materials with varying expansion stay refused and go to the owner as a release
  limitation, whose acceptance is pending. The rule is written for materials in general and
  tested on more than the reference system's material.
- **Confirmation of D-2's design intent (Agent 0, 2026-11-02; paraphrase).** Varying E with
  constant α; a general mapping, not tailored to the fixture; mapping only where exact
  per-member thermal, modulus and source proof holds; varying α and the cold-modulus basis
  refused; acceptance pending. This is the current decision basis. It verifies no historical
  wording, does not complete criterion 6 and closes no dossier question.
- **Route 1 (Agent 0, 2026-11-02, binding; paraphrase).** Design comes first. The per-member
  modulus basis is carried through the load/reference-state group, and only an explicitly
  designed subset is admitted:
  - one selected thermal case;
  - a constant-α basis;
  - the per-member rule below.

  Route 2, a case-level `modulus_basis_temperature` with thermal primitives, stays refused.
  Q-5 is not adopted.
- **Refinements and decisions (Agent 0, 2026-11-02, binding; O-7).** Decision 3 bounds the
  undated exception below. Derived T1 rows are admitted only by option (a), exact admission,
  which Agent 0 chose; they remain conditional on G-4, which stays open, as does CQ-17-01-016.
  Sidecar format 5 must bind source and export bytes, identities and derivation provenance.
  Every bound format-5 import with temperature-point materials is admitted only against a
  source the user explicitly designates, verified by hash and then by content; the sidecar's
  source digest verifies that source and never selects it (§20.4.4, §20.4.5; Agent 0,
  2026-11-02, paraphrase). §20.4.5 extends the same requirement to point-free materials
  (**Point-free materials**). The trusted export ledger is proposed, not adopted. Varying α and
  cold-modulus files go to the criterion 3 list with the owner's acceptance pending (§20.4.10).
  Source consultation for G-1 and G-4 stays within each registered row's current boundary
  (§20.4.9).
- **The operating case.** Agent 0 approved designing the operating W+P+T mapping as a separate
  design-first item toward criterion 6 (2026-11-02; paraphrase). Route 1 is thermal only and
  does not complete the reference system (§20.4.8). The operating mapping is not designed here.

**The per-member rule.** Each exported member's modulus-selection temperature equals that
member's mapped MBF T1 under exact identity, and the E the solver selects for that member
equals exactly the E that the MBF row at T1 states (§20.4.2 e, g). A selection that differs is
refused by name with the member's id. The one bounded exception (decision 3): an
`explicit_base_properties` selection on a material with no temperature points has no selection
temperature. It is eligible only when the actual selected E basis is constant and exactly
representable by the mapped MBF material semantics, with the other material-property and
thermal-law conditions satisfied separately (§20.4.2 e). Nothing is inferred from missing
points.

**Sources.** SWBPIPE's side of each equivalence is cited from source: the schema
`schemas/load_reference_state.schema.json` and the solver's case-state resolution in
`core/product_physics/src/case_state/`. No DEL-04 Work document defines these semantics
(searched); the code is their statement. The MBF side uses only admitted rows (§2):

- `CAEPIPE-IMPORT-MBF`: the MATERIAL, LOADS, OPTEMP and LAYOUT records.
- `CAEPIPE-MATERIAL`: α is the mean coefficient, and element temperatures must lie within the
  table's range.
- `CAEPIPE-ANALYSIS`, as O-7 records it: the hot-modulus option selects E at T1 for operating
  case 1, the cold-modulus option selects E at the reference temperature, and the expansion
  case is operating minus sustained. The row's current registered boundary is narrower:
  "analysis-option semantics only: number of thermal loads, gravitational acceleration and
  direction" (DEL-17-01 `Source_Basis_Register.md`). The modulus-option and expansion-case
  semantics are therefore not admitted by that boundary as it stands, and their historical use
  in O-7 does not establish that they were within it. Agent 0 folded this conflict into the
  planned G-2 register-change PR (2026-11-02; paraphrase), which is the route for it. §20.4
  stays blocked on that evidence until the exact expansion of the row's boundary passes
  independent IP review and the applicable adoption authority (G-1, G-2, G-4). This Design does
  not widen the boundary.
- `CAEPIPE-LOAD-CASES`: case membership.

Statements about the MBF follow §0's labels. Where a needed fact has no admitted source, it is
listed as a gap (§20.4.9) and not filled.

#### 20.4.1 Where Route 1 applies, and what stays unchanged

**Entry condition.** A model takes Route 1 only when all three of these hold:

- it carries the exact pressure contract (`mod.rs:30-32`, `is_load_state`);
- it has a non-empty `reference_configurations`;
- every load case carries `analysis_state`.

The solver requires the same complete group: `resolve.rs:123-129` and `:215-218`. Any other
model that carries a group key keeps today's refusals by their existing names:

- `MBF-SI-REFERENCE-CONFIGURATION-UNSUPPORTED` (`si_writer.py:689-691`);
- `MBF-SI-TEMPERATURE-DEPENDENT-MATERIAL-UNSUPPORTED` (`:747-752`);
- `MBF-SI-LOAD-CASE-FEATURE-UNSUPPORTED` (`:1249-1251`).

**Unchanged.** The following are untouched:

- Today's thermal-primitive export: ΔT → LOADS T1 = T_ref + ΔT (`si_writer.py:1444-1463`,
  `:1683-1690`), with OPTEMP (`:1789`) and identical MATERIAL rows (`:1709-1715`,
  `:1792-1799`).
- Unbound and bound imports of every file outside Route 1.
- Sidecar formats 2, 3 and 4.
- Route 2. A material with `temperature_points` and a case-level `modulus_basis_ref` or
  `modulus_basis_temperature` outside the group stays refused, by the two existing codes
  above. Inside the group, the solver refuses a case-wide basis too (`resolve.rs:220-223`).
- The writer's 20 °C default, which still applies only to the ordinary route (§20.4.2 f).

A model outside Route 1 is written byte for byte as today, with a format-4 sidecar.

#### 20.4.2 The admitted subset

**(a) Reference configuration: one, with one source-bound reference temperature.**

- **Form.** Exactly one `reference_configurations[]` item (schema `ReferenceConfiguration`,
  :145). Its `geometry_ref` is `authored_model_geometry`, the only kind. Every member reference
  (:120) satisfies all of these:
  - its basis is `temperature_reference` (:49);
  - its fit is `none` (:82);
  - every member gives the same `installation_temperature` under exact identity (below).
- **What OPTEMP receives.** That common installation temperature is T_ref, and it is written
  as OPTEMP field 1.
- **Why.** The solver resolves each member's installation temperature from its own reference
  (`resolve.rs:782-798`). A common one is therefore the only form whose single MBF reference
  temperature states every member's reference, as `CAEPIPE-IMPORT-MBF` gives it (OPTEMP has
  one reference temperature).
- **The writer's argument.** Under Route 1, OPTEMP comes from the configuration and never from
  `reference_temperature_c` or `_f`. The signature's default becomes an internal "not given"
  sentinel. The ordinary route resolves it to 20.0 exactly as today, so no default
  manufactures an equivalence. An explicit argument that differs from T_ref is refused (§20.4.3).
- **Regeneration.** The importer's regeneration call passes OPTEMP's value
  (`importer.py:2368`). On a Route 1 file that value equals the restored configuration's, so
  regeneration sees no conflict.

**(b) Case roles.** Each case resolves through its own `analysis_state` (`resolve.rs:658-688`).
Route 1 classifies cases by role, from what each case's ledger and element states contain:

| Role | Condition | MBF case |
|---|---|---|
| thermal | `load_sources` empty; no `pressure_regions`; no `boundary_motion`; at least one member with T1 ≠ T_ref | the expansion label (`LOAD_CASE_LABELS["expansion"]`), as today |
| sustained | `load_sources` non-empty or pressure regions; every member at T1 = T_ref | the sustained label, as today |
| reference state | no source, no region, every member at T1 = T_ref | omitted (`MBF-SI-LOAD-CASE-EMPTY`, as today) |
| operating (mixed) | sources or regions together with a member at T1 ≠ T_ref | refused (`MBF-SI-LOAD-CASE-MAPPING-UNSUPPORTED`; with regions `MBF-SI-PRESSURE-CASE-UNSUPPORTED`), as today |

Further rules for the roles:

- Sustained membership carries no temperature (`CAEPIPE-LOAD-CASES`). In a sustained or
  reference-state case, each member's mapped temperature is therefore T_ref, and its thermal
  state must resolve to zero strain there: `unchanged_reference`, `constant_alpha_interval`
  with ΔT = 0, or `free_length_state` with operating temperature T_ref. The LOADS T1 is the
  thermal case's (c).
- At most one case of each role is exported, as today (`si_writer.py:1292-1296`).
- A sustained case's sources each have factor 1, and every stored primitive of the case is in
  its ledger. A source with another factor, or a stored primitive left out of the ledger, would
  change the effective case (`resolve.rs:398-399`) in a way the MBF cannot state.
- Support states are `active_model_device` only. The solver refuses the others
  (`resolve.rs:296-306`).

**(c) Element thermal states and their MBF T1.** For every member, its thermal state in the
thermal case gives one T1, written as that element's LOADS T1 through the layout row's `L=`
reference (§10.3). Members with equal (T1, P1) share one LOADS record, as today.

| Variant (schema :504) | Admitted when | Its MBF T1 | SWBPIPE strain (source) |
|---|---|---|---|
| `constant_alpha_interval` {α_c, ΔT, `engineering_interval`} (:559, :568) | α_c is the material's constant α (e); `operating_temperature`, if given, equals T_ref + ΔT | T_ref + ΔT | α_c·ΔT (`thermal.rs:531-540`) |
| `free_length_state` {law} (:584) | the law is admitted (d); the member basis is `temperature_reference` (`resolve.rs:873-877`); `operating_temperature` is given (`resolve.rs:878-882`) | `operating_temperature` | α·(T_op − T_inst)/(1 + α·(T_inst − T_datum)) (`thermal.rs:582-594`), equal to α·(T_op − T_inst) because the datum equals T_inst (d) |
| `unchanged_reference` (:516) | the member's reference is `temperature_reference` at the common T_ref, which establishes the global reference (a); `operating_temperature`, if given, equals T_ref | T_ref | 0 (`thermal.rs:526`) |
| `explicit_interval_strain` (:534) | never | — | refused (§20.4.3) |

**Strain equivalence.**

- *SWBPIPE side.* In every admitted row the resolved thermal strain is α·(T1 − T_ref), with α
  the material's constant α, evaluated in binary64 as the solver does.
- *MBF side.* MATERIAL gives α at each temperature row (`CAEPIPE-IMPORT-MBF`), and the
  tabulated α is the mean coefficient (`CAEPIPE-MATERIAL`). The admitted pages give no strain
  formula and no base temperature for the mean (O-7).
- *The reading.* With α the same in every row, Route 1 relies on exactly the reading §7.1 and
  §9.2 already apply to today's thermal-primitive export: the strain is α·(T1 − T_ref) whatever
  the base of the mean or the interpolation of α. Route 1 claims no strain semantics beyond
  today's route (G-3, §20.4.9).
- *Unchanged reference.* For `unchanged_reference`, T1 = T_ref is used only where (a) has
  established T_ref as that member's own installation temperature. Zero strain then equals
  α·0 under the same reading. Without that, the member is refused (§20.4.3), as the ruling
  requires.

**(d) Expansion laws** (schema `ExpansionLaw`, :307).

- **Admitted.** Every law on an exported material is `engineering_secant` with
  `data.kind: constant` (:227), coefficient equal to the material's constant α, and
  `datum_temperature` equal to T_ref under exact identity.
- **Why the datum must equal T_ref.** For a constant secant law, the solver divides by the
  installation datum stretch 1 + α·(T_inst − T_datum) (`thermal.rs:582-594`, `stretch` at
  :201-211). That stretch is exactly 1 only when T_datum = T_inst. With T_inst = T_ref, the
  product α·0 is exactly 0, so the division is by 1.
- **Refused.** Any other datum gives a strain that α·(T1 − T_ref) does not state (example 4).
  Secant tables, `engineering_dilation`, `differential_per_datum_length` and
  `logarithmic_per_current_length` are refused: none is a constant-α basis.

**(e) Material and material selection.**

*Material requirements.* Every exported material meets all of these:

- `constitutive_basis` `homogeneous_isotropic_E_nu_v1`, which the selector requires
  (`material.rs:179-185`).
- **Constant α.** One value, compared exactly after normalization to 1/°C (1/K and 1/degC ×1,
  1/degF ×9/5, §5.2). It is given by every present point α, the material-level α if present,
  every `constant_alpha_interval` coefficient of a member made of it, and every law coefficient
  on it.
- **One Poisson ratio** across the points and the base. The MBF's first line carries one
  (`CAEPIPE-IMPORT-MBF`, MATERIAL). The selector interpolates ν (`material.rs:319-326`), so with
  equal ν the result is that ν exactly.
- **Points.** Every temperature point is dated and the dates are distinct. The material's base
  E, ν and α equal one point's. The MBF has no base-property record, and the selector reads the
  base only for `explicit_base_properties` (`material.rs:199-227`), which (below) admits only a
  point-free material.

*MATERIAL rows.* A material with points is written with one row per point (temperature, the
point's E, α). It also gets one **derived row** at each distinct member T1 (exact identity) of
that material that is not a point temperature. Derived rows are admitted only under option
(a) below, and remain conditional on G-4 (§20.4.9). A point-free material keeps today's three
identical rows (`si_writer.py:1709-1715`), with ΔT_e = T1_e − T_ref. T_ref must lie within each
material's point range. The target needs element temperatures within the table (`CAEPIPE-MATERIAL`), and
the members' T1 already are, since the selector forbids extrapolation (`material.rs:307`).

*The E a row must state: the solver's selection, as the code performs it.* A derived row's E is
not a separately computed interpolation. It is the value the solver selects for the members the
row serves:

- **Temperatures.** The resolver replaces each absolute temperature a member compares by its
  exact-identity representative in kelvin. That representative is the first binary64 unit
  conversion registered for that exact value in the member's own identity scope, in this order:
  points, selection temperature, operating temperature, installation temperature, law
  temperatures (`resolve.rs:438-485`, `:716-737`; `temperature.rs:58-101`). The conversion is
  `value × factor + offset` in binary64 (`units/src/lib.rs:799-805`, `:1007-1040`).
- **E.** Each point's E is converted to Pa in binary64 the same way (`material.rs:78-120`).
- **Selection.** When the requested kelvin value equals a point's, the selector takes that point
  (`material.rs:300-305`). Otherwise it takes the strict adjacent bracket, forms the binary64
  fraction f = (T − T_lo)/(T_hi − T_lo), and takes E = `f.mul_add(E_hi − E_lo, E_lo)`. That is
  a fused multiply-add on the binary64 difference, rounded once (`material.rs:307-326`). ν is
  formed the same way.

It is not an exact rational rounded once, and it is not, in general, a 10-digit decimal.

*The equality basis: option (a), exact admission (Agent 0's decision, 2026-11-02; O-7).*

- **The printed value.** Let d be a row's printed `.10g` decimal in N/mm² (§18.2's format). For a
  derived row, d is formed from the exact rational E/10⁶ of the selected binary64 E, not from a
  binary64 MPa quotient. Since 1 N/mm² is exactly 10⁶ Pa, the row states exactly d × 10⁶ Pa.
- **On the exported model.** For every member the row serves, the solver's selected binary64 E
  (above) must equal d × 10⁶ Pa as a rational.
- **On the restored model.** The import restores each point's E as the binary64 nearest its
  printed decimal, in MPa, and each temperature as the binary64 nearest its printed decimal, in
  °C (§20.4.4). The solver converts them as it converts any input: E to Pa as fl(x × 10⁶), and a
  temperature to kelvin as fl(t + 273.15) (`material.rs:78-91`; `units/src/lib.rs:799-805`,
  `:1007-1040`). The derived row itself is stripped (§20.4.4), so each member's E is selected
  again from the restored points by the same selector. That selected binary64 E must again equal
  d × 10⁶ Pa as a rational.
- **Otherwise** the export is refused by the member's id (`MBF-SI-MODULUS-NOT-EXACT`), naming
  the material, T1, the selected E and d. The comparison is exact. No §18.2 printed-precision
  class, tolerance or relaxation applies to it.
- **Point rows and point-free materials.** The same equality applies to a point row's E and to a
  point-free material's E.

Counterexamples, computed with a reproduction of `material.rs:300-326` and the kelvin
representatives, with a fused multiply-add:

- **6a, off the printed grid** (review of `b74ac36e7`). Points 204 GPa at 25 °C and 196 GPa at
  100 °C, selected at 50 °C. The solver's E is 201333333333.33334 Pa. The row would print
  d = 201333.3333, which states 33.3… Pa less. The member is refused (§20.4.6, example 6).
- **6d, on the grid as a rational, off it in binary64.** Points 185 GPa at 0 °C and 150 GPa at
  50 °C, selected at 0.16 °C, close to the lower endpoint. The exact rational interpolation is
  184888 N/mm², and d prints as 184888. The solver's E is 184887999999.99997 Pa, one ulp below
  184888 × 10⁶. The member is refused: the criterion is the value the solver solves with, not
  the exact rational interpolation.
- **Boundary.** A T1 equal to a table endpoint, or to any point's temperature, takes that point
  (`material.rs:300-305`) and writes no derived row (below). It is admitted when the point's
  d × 10⁶ Pa equals the point's binary64 E.
- **Units.** 347 degF (5a) and 448.15 K (5c) on EX-A select 194000000000.0 Pa, which equals
  194000 × 10⁶: admitted. A modulus authored as 29e6 psi is 199947.9615018825… N/mm² and is
  refused (7c). Same-temperature spellings in different units are treated under one row per T1,
  below.

The admitted set is whatever meets that equality. A derived row is admitted only when the
solver's binary64 E lies exactly on the printed grid, which is 100 Pa for E from 10⁵ to 10⁶
N/mm². Enumerating every integer-°C T1 with the reproduction, every one inside the §20.4.6
tables, and inside RS-M1's table, meets it. That is an observed outcome, not a design input. No
value or rule is chosen to fit RS-M1, and no sampled admission rate is a bound or a justification
for the rule. Option (b), print precision, is rejected (§20.4.10).

- **One row per T1.** Members of one material whose T1 are exactly equal share one derived row.
  Each member has its own identity scope, so members that author the same temperature in
  different units can select E values one ulp apart. Points 189 GPa at 3 °C and 177.5 GPa at
  78 °C, selected at 7.4 degC and at 45.32 degF, give 188325333333.33334 Pa and
  188325333333.3333 Pa. Every member a row serves must select exactly d × 10⁶ Pa. A member that
  does not is refused by its id.
- **No derived row at a point temperature.** A T1 exactly equal to a point's temperature uses
  that point's row, and the selector then takes the point itself (`material.rs:300-305`). A
  derived row never duplicates or replaces an authored row.

*Admitted selections in the thermal case* (schema `MaterialSelection`, :431):

| Selection | Admitted when | Equivalence |
|---|---|---|
| `temperature_interpolation` at T_sel (:486) | T_sel = the member's T1 under exact identity, and the selected E equals the row's d × 10⁶ Pa exactly (above) | SWBPIPE takes E and ν at T_sel: the point itself when T_sel is a point temperature (`material.rs:300-305`), else the binary64 fused interpolation above (`:307-326`); G = E/(2(1+ν)) is derived (`resolve.rs:987-990`). The MBF has a row at T1 that states exactly that E. With the hot-modulus option, the target uses E at T1 (`CAEPIPE-ANALYSIS`, outside that row's current boundary, pending the G-2 proposal; §20.4, **Sources**), and a row states E at its own temperature (`CAEPIPE-IMPORT-MBF`, MATERIAL). That the target then uses the supplied row's E exactly, with no other interpolation or modulus convention, is not yet proved (G-4). CQ-17-01-016 stays open until it is |
| `exact_point` (:464) | the point is dated at the member's T1, and its E equals the row's d × 10⁶ Pa exactly | the same pair as interpolation at that temperature (`material.rs:228-273`), under the same G-4 condition |
| `explicit_base_properties` (:444) | decision 3's bounded exception only, when every condition below holds | the selector's basis is the material's single base E and ν (`material.rs:199-227`), written as today's identical rows, which state that E at every row temperature (§7.1) |

*The bounded exception for `explicit_base_properties`* (decision 3, O-7). Such a selection has no
selection temperature (`material.rs:199-227`, `selection_temperature_k: None`). It is eligible
for the same-T1 condition only when all of these hold. None is inferred from missing points:

1. **Constant selected basis.** The material has no `temperature_points`. The selector's actual
   basis is then the material's base E and ν, the same at every T1. On a material with points, an
   undated selection is refused (`MBF-SI-MODULUS-SELECTION-UNDATED`). That holds even when the
   base equals a point whose date is the member's T1: the selector does not read that point, and
   the table's E is not constant.
2. **Exactly representable.** The base E meets option (a)'s equality: d × 10⁶ Pa equals the
   selector's binary64 base E, on the exported and on the restored model
   (`MBF-SI-MODULUS-NOT-EXACT`). It also passes (g)'s round trip
   (`MBF-SI-REFERENCE-STATE-NOT-PRINTABLE`).
3. **Other properties and thermal-law conditions, checked separately.**
   - ν passes its own round trip (g).
   - α is constant under (e)'s list: the material-level α and every coefficient of every
     member's interval or law on the material. Each must be present where its route needs it,
     and none is taken from the absence of points.
   - Laws and their datum meet (d).
   - The member's reference basis is `temperature_reference` at T_ref (a).
4. **Per member.** Each member's selection is checked on its own record and case. Another member
   of the same material, or the same member in another case, establishes nothing for it. A
   selection naming a material other than the member's pipe material is refused
   (`MBF-SI-MODULUS-SELECTION-CONFLICT`); the solver refuses it too
   (`LOAD_STATE_MATERIAL_REFERENCE_MISMATCH`, `material.rs:172-178`).

Counterexamples are §20.4.6, example 7.

Two further consequences:

- The per-member rule therefore constrains the modulus only. Strain comes from the thermal
  state, never from the points: the load/reference-state route does not read point α (thermal
  inputs, `resolve.rs:800-917`).
- An `analysis_basis_override` can only accompany equal temperatures here. The selector needs
  it only when they differ (`material.rs:147-159`). It is carried as authored text (§20.4.5).

*Sustained case.* The admitted sources do not state which modulus the target uses for the
sustained case (G-2). Until G-2 is answered, a sustained case is admitted only when every
material it uses has a single E across its MBF rows. Its selections are restored at T_ref (or
as explicit base properties). Otherwise it is refused (§20.4.3). The thermal case's
equivalence then holds without G-2: with no sustained loads the sustained case is null, so the
documented expansion case (operating minus sustained) is the T1 response at E(T1).

**(f) OPTEMP, LOADS and layout.**

- **OPTEMP.** Field 1 is T_ref (a). The cold-modulus flag, OPTEMP field 5 (§6; G-1 (a),
  admitted), is written so that it selects the hot modulus. The value that does so, the effect
  of an omitted field and the modulus under a null piping code are G-1 (b), (c) and (e), and the
  implementation waits on them. A piping code for which the page says the flag is ignored is
  proposed for non-admission (§20.4.3; G-1 (d)).
- **LOADS.** T1 per element as in (c), P1 as today (§9.4).
- **Layout.** Layout rows carry `L=` as today. A bend pipe and its incoming straight still share
  one row, so they must share T1 (`MBF-SI-BEND-UNSUPPORTED`, as today).

**(g) Exact identity and printing.**

- **Exact comparison.** Every equality above is decided on exact rationals, never by binary64
  affine conversion. This is the solver's own temperature identity: the canonical shortest
  round-trip decimal of the authored binary64 under the unit's exact affine definition
  (`temperature.rs:1-16`, `:37-56`; DEL-02-01 `model-document-0.9.0.md` §1.5.1). Intervals use
  K and degC ×1 and degF ×5/9. Absolute degC, degF and K are admitted (degF by §5.2's exact
  definition, K by T − 273.15); other units take `MBF-SI-UNIT-UNSUPPORTED`.
- **What exact identity covers.** It decides equality of *inputs*. It does not make two
  binary64 evaluations from equal inputs identical. The resolver keeps evaluating in binary64
  from each member's representatives (`temperature.rs:12-13`), and representatives of one exact
  value can differ by an ulp between documents that spell it in different units. §20.4.4 states
  what is exact and what is compared numerically.
- **Canonical-decimal round trip, checked explicitly.** The writer checks every retained
  numerical value of a Route 1 export, and every interval the import reconstructs. No digit
  count is inferred. The values are:
  - T_ref, each T1, each point temperature and each `operating_temperature`;
  - each reconstructed interval ΔT = T1 − T_ref;
  - each point E, a point-free material's E, the derived rows' E (option (a), above);
  - ν, and the constant α, whether from points, from the material level of a point-free
    material, from an interval coefficient or from a law coefficient.

  For each value, two equalities must hold, or the export is refused by name
  (`MBF-SI-REFERENCE-STATE-NOT-PRINTABLE`, naming the value):
  1. **Exported.** The authored canonical decimal is converted exactly, as a rational, into the
     MBF unit by its unit's definition, and the result equals the printed `.10g` decimal p as a
     rational:
     - an absolute temperature by the unit's exact affine conversion to °C: degC as written,
       K as T − 273.15, degF as (T − 32) × 5/9 (for example 77 degF is exactly 25 °C, not
       77 × 5/9);
     - an interval, and every other quantity, by its exact multiplicative factor: intervals in
       K and degC ×1 and in degF ×5/9 into degC; E into N/mm²; α into 1/°C (1/K and 1/degC ×1,
       1/degF ×9/5); ν ×1.
  2. **Restored.** The value the import restores is the binary64 nearest p, in the unit it
     restores (§20.4.4). Its canonical decimal must be p again. For ΔT, the restored value is
     the binary64 nearest the exact difference of the two printed decimals, and its canonical
     decimal must equal that exact difference. Where the member authors an interval, that
     difference must also equal the interval's exact value.

  The second check replaces the earlier claim that such a difference has at most 15
  significant digits. Two 10-digit decimals of different magnitude can differ in more digits
  than binary64 can carry.

#### 20.4.3 Refusals

Any refusal refuses the whole file, as today (§17).

| Code | New or existing | Condition |
|---|---|---|
| `MBF-SI-MODULUS-SELECTION-MISMATCH` | new | a member's modulus-selection temperature (interpolation temperature, or exact point's date) differs from its mapped T1. Names the pipe, the case, both temperatures |
| `MBF-SI-MODULUS-SELECTION-UNDATED` | new | an undated selection outside decision 3's bounded exception: `explicit_base_properties` on a material with points, even where the base equals a point dated at T1 (§20.4.2 e) |
| `MBF-SI-MODULUS-NOT-EXACT` | new | a row's printed E, read as d × 10⁶ Pa, that differs from the solver's selected binary64 E for a member the row serves, on the exported or the restored model (option (a), §20.4.2 e). Names the member, the material, T1, the selected E and d |
| `MBF-SI-MODULUS-SELECTION-CONFLICT` | new | a selection that names a material other than the member's pipe material |
| `MBF-SI-UNCHANGED-REFERENCE-UNESTABLISHED` | new | `unchanged_reference` on a member whose reference does not establish T_ref (`direct_strain_reference`) |
| `MBF-SI-REFERENCE-BASIS-UNSUPPORTED` | new | any other member with `direct_strain_reference`. It supplies no absolute temperature (schema :65) |
| `MBF-SI-REFERENCE-CONFIGURATION-MULTIPLE` | new | more than one reference configuration |
| `MBF-SI-REFERENCE-DATUM-MULTIPLE` | new | member installation temperatures that differ |
| `MBF-SI-REFERENCE-FIT-UNSUPPORTED` | new | a fit other than `none` (`natural_length_change`, `fit_strain`; `resolve.rs:925-966`) |
| `MBF-SI-REFERENCE-TEMPERATURE-CONFLICT` | new | an explicit writer reference temperature that differs from T_ref |
| `MBF-SI-INTERVAL-STRAIN-UNSUPPORTED` | new | `explicit_interval_strain`. Its strain has no temperature, and T1 = T_ref + ε/α would be a temperature label inferred by assumption and a rounded division, not a proved equality |
| `MBF-SI-EXPANSION-NOT-CONSTANT` | new | α values of one material that differ (D-2: a release limitation) |
| `MBF-SI-EXPANSION-LAW-UNSUPPORTED` | new | a law that is not a constant `engineering_secant` |
| `MBF-SI-EXPANSION-DATUM-UNSUPPORTED` | new | a constant secant law whose datum differs from T_ref |
| `MBF-SI-OPERATING-TEMPERATURE-CONFLICT` | new | an `operating_temperature` that differs from the member's T1 under (c) |
| `MBF-SI-POISSON-NOT-CONSTANT` | new | ν values of one material that differ |
| `MBF-SI-MATERIAL-POINT-UNDATED` | new | a point without a temperature, or two at one temperature |
| `MBF-SI-BASE-PROPERTIES-UNREPRESENTED` | new | base E, ν, α equal to no point's |
| `MBF-SI-MATERIAL-TABLE-RANGE` | new | T_ref outside a material's point range |
| `MBF-SI-REFERENCE-STATE-NOT-PRINTABLE` | new | a retained value or reconstructed interval that fails §20.4.2 (g)'s explicit canonical-decimal round trip, exported or restored. Names the value |
| `MBF-SI-SUSTAINED-MODULUS-UNPROVED` | new | a sustained case beside a material whose rows differ in E, or a sustained selection not at T_ref (G-2) |
| `MBF-SI-LEDGER-FACTOR-UNSUPPORTED` | new | a load source factor other than 1 |
| `MBF-SI-LEDGER-EXCLUSION-UNSUPPORTED` | new | a stored primitive missing from its case's ledger (a stored legacy thermal primitive included; `resolve.rs:370-377`) |
| `MBF-SI-BOUNDARY-MOTION-UNSUPPORTED` | new | any `support_states[].boundary_motion` (§20.4.8) |
| `MBF-SI-LOAD-CASE-MAPPING-UNSUPPORTED` | existing | more than one thermal-role or sustained-role case; an operating (mixed) case |
| `MBF-SI-PRESSURE-CASE-UNSUPPORTED` | existing | pressure regions in a case with a member at T1 ≠ T_ref |
| `MBF-SI-LOAD-CASE-FEATURE-UNSUPPORTED`, `MBF-SI-TEMPERATURE-DEPENDENT-MATERIAL-UNSUPPORTED`, `MBF-SI-REFERENCE-CONFIGURATION-UNSUPPORTED` | existing | outside Route 1 (§20.4.1), including Route 2 |
| `MBF-SI-ROUTE1-PIPING-CODE-IGNORES-MODULUS-FLAG` | new (proposed design code; not adopted) | a Route 1 export whose piping-code argument is a token on §20.4.9's proposed list (G-1 (d)): the page says the cold-modulus flag is ignored for that code, so the mapped modulus selection (E at each member's T1) is not established for it. Names the token. A null code is not refused by it (G-1 (e)); exports outside Route 1 are unchanged |
| `MBF-SI-SHEAR-MODULUS-NOT-EXPORTED` | existing (omitted) | a point carrying `shear_modulus`, which the selector ignores (`resolve.rs:991`) and the MBF cannot carry |
| `MBF-IMPORT-COLD-MODULUS-UNSUPPORTED` | new (importer) | a Route 1 file whose OPTEMP modulus field does not select the hot modulus. The writer never writes the cold modulus. On the writer side, a selection at T_ref in a member whose T1 differs is a mismatch |
| `MBF-IMPORT-REFERENCE-STATE-UNBOUND` | new (importer) | a format-5 sidecar on the rebind route (`rebind_by_node_number`), where the regeneration identity does not run |
| `MBF-IMPORT-SOURCE-UNASSOCIATED` | new (importer; design code) | a bound format-5 import for which the caller explicitly designated no source, whether or not any material has temperature points (§20.4.5, **The designated source** and **Point-free materials**). The importer never finds a source by the sidecar's digest or any other sidecar field, and there is no fallback. Names the presented MBF and sidecar digests. Nothing is restored or stripped |
| `MBF-IMPORT-SOURCE-MISMATCH` | new (importer) | a designated source whose canonical SHA-256 differs from the sidecar's `source_model_sha256` (check 1); or one that the importer, rendering it independently with the recorded export inputs, either cannot render (the writer refuses it) or renders to MBF bytes whose SHA-256 differs from the presented MBF's (check 2 a); or one whose format-5 sidecar, formed for the *rendered* digest, differs from the presented sidecar (check 2 b; §20.4.4, **Source proof**). Names the first difference: the writer's refusal codes; for the bytes, the first differing record and field where practical; for the sidecar, the first differing field, and for a row the row and both classifications |
| `MBF-IMPORT-RENDER-PROFILE-UNSUPPORTED` | new (importer) | a format-5 sidecar whose `derivation.render_profile` is not the importing build's own writer profile, so check 2 cannot render the source as the export rendered it (§20.4.5, **Export inputs outside the source document**). Checked at check 2, before rendering. Names both profiles. Nothing is restored or stripped; the MBF alone still imports unbound |
| `MBF-IMPORT-DERIVED-ROW-UNPROVED` | new (importer) | a MATERIAL row that the format-5 sidecar marks as generated, but whose generation §20.4.4 does not prove: any of stripping conditions 2 to 7 fails after the source proof has passed. Names the material and row. The row is never stripped, and the import is refused |
| `MBF-IMPORT-SIDECAR-INVALID` | existing (importer) | a format-5 sidecar given to an importer that reads formats 2 to 4 only. Its parser refuses `sidecar_format` 5 before binding (`importer.py:1176-1179`, `:1661-1665`), so the import neither binds nor falls back to an unbound reading (§20.4.5) |
| `MBF-IMPORT-MATERIAL-TEMPERATURE-DEPENDENT` | existing (importer) | unbound: unchanged (§20.4.4) |

#### 20.4.4 The importer's inverse

**Bound (exact digest, sidecar format 5).** The import assembles the group from MBF values and
sidecar identities. It then runs the regeneration identity (§14): the writer regenerates the
format-5 sidecar from the imported model, and any difference is
`MBF-IMPORT-SIDECAR-NOT-REPRODUCED`.

What comes from the MBF (MBF-determined, checked):

- T_ref, from OPTEMP;
- each member's T1, from its LOADS record;
- E per row, the constant α and ν, from MATERIAL;
- the hot-modulus field, from OPTEMP.

**Source proof: required for every bound format-5 import.** A bound format-5 import is admitted
only against a source S that the user explicitly designates, independently of the sidecar and
before any check (§20.4.5, **The designated source**): a saved state the user chooses or a model
document the user supplies. Without a designation, the import is refused,
`MBF-IMPORT-SOURCE-UNASSOCIATED`. The sidecar's `source_model_sha256` only verifies S; it never
selects S, and the importer has no lookup by it. Agent 0's ruling requires this wherever a
material has temperature points; §20.4.5 (**Point-free materials**) shows why it is required for
point-free materials too. Before any identity is restored or any row stripped, S must pass both
of these checks, in this order:

1. **Hash.** S's canonical SHA-256, computed from S's own text, equals the sidecar's
   `source_model_sha256`. A source that differs is `MBF-IMPORT-SOURCE-MISMATCH`.
2. **Content: an independent rendering of S.** The importer first requires the sidecar's
   `render_profile` to be its own writer's (`MBF-IMPORT-RENDER-PROFILE-UNSUPPORTED` otherwise).
   It then renders S itself with the export inputs that the sidecar records (§20.4.5, **Export
   inputs outside the source document**), and computes the SHA-256 of the rendered MBF bytes.
   The rendering runs the writer as the export ran it, apart from the reimport preflight, which
   can only refuse an export and never alters its bytes (`si_writer.py:1936-1939`). So the
   rendering does not recurse into another import. Then, in this order:
   - **(a) Bytes.** The rendered digest equals the SHA-256 computed from the presented MBF file.
     The bound route has already required that digest to equal the sidecar's `mbf_sha256`. The
     rendered digest comes from S and the recorded export inputs only, and the presented digest
     is never injected into anything expected. If the writer refuses S under the recorded inputs,
     or the digests differ, the result is `MBF-IMPORT-SOURCE-MISMATCH`. It names the writer's
     codes, or, where practical, the first record and field in which the rendered and presented
     texts differ.
   - **(b) Sidecar.** The format-5 sidecar that the writer forms from S for the *rendered*
     digest equals the presented sidecar in every field. For every material with points, that
     covers the authored point identities and their order (`points`), every row's
     classification and source (a point row's point id, `alpha_authored` and provenance; a
     derived row's members and bracket), the checked copies of every row's values, `base_point`,
     and the whole of `reference_state_map` and `derivation`. A difference is
     `MBF-IMPORT-SOURCE-MISMATCH`, naming the first differing field; for a row, it names the
     row and both classifications.

   (a) covers every value the MBF carries, including those that no sidecar field repeats.
   Format 4's maps omit section dimensions, spring stiffness and every other MBF-determined
   value (§15; `si_writer.py:233-266`). Forming S's sidecar for the *presented* digest instead,
   as the regeneration route does (`regenerate_caepipe_sidecar`, `si_writer.py:444-464`, returns
   `result.sidecar(mbf_sha256)` with the caller's digest), would only check a supplied digest
   against itself. An MBF with an MBF-only value changed and both digests recomputed would then
   pass the sidecar comparison and the MATERIAL rows alike (§20.4.6, example 10).

Both checks run whether or not any row is marked generated. A sidecar that marks an authored row
as generated, relabels a generated row as a point, or changes an authored identity, order or
provenance therefore differs from S's own sidecar and is refused (§20.4.6, examples 8b and 8c).
So does a sidecar that changes a point-free export's reference-state metadata (example 9). An MBF
in which a value that no sidecar field repeats has been changed, with both digests recomputed,
renders differently from S and is refused at check 2 (a) (example 10). Example 8d shows why S is
designated independently and never found by the sidecar's digest. These are the checks that establish **source authenticity**, relative to the designated S.
Consistency between the sidecar, the MBF and the restored model (the stripping conditions and
the regeneration identity below) does not, and neither does the self-digest, which anyone who
edits a sidecar can recompute.

What the import restores:

- **Configuration.** The installation temperature is T_ref, the basis `temperature_reference`
  and the fit `none` on every member. Ids, label and provenance come from the sidecar.
- **Laws.** `engineering_secant`, constant, coefficient α, datum T_ref. The id and provenance
  come from the sidecar.
- **Points.** From the point rows: temperature, E, ν (first line), α where
  `alpha_authored`, with id and provenance from the sidecar. They are restored in the authored
  order the sidecar records (`points`, §20.4.5), not in MBF row order. The base E, ν and α come
  from the recorded `base_point`.
- **Generated rows: stripped only when proved.** A MATERIAL row is removed only when every one of
  these holds. Otherwise the import is refused (`MBF-IMPORT-DERIVED-ROW-UNPROVED`, naming the
  material and row), and nothing is stripped by guessing:
  1. **Source proof.** The source proof above has passed, so S's own sidecar classifies the row
     as derived, with the same members and bracket;
  2. the sidecar's `derivation` records the row as generated, with its members, its bracket
     point ids and its MBF row index;
  3. its temperature equals no restored point's temperature under exact identity;
  4. every recorded member is in the thermal case and restores to `temperature_interpolation` at
     a T1 exactly equal to the row's temperature. Every interpolating member at that T1 is
     recorded;
  5. the bracket points are the restored points adjacent to that temperature;
  6. the solver's selection on the restored model (§20.4.2 e) gives, for every recorded member,
     exactly d × 10⁶ Pa, where d is the row's printed E, and the row's α and ν equal the
     material's;
  7. every row the sidecar records as a point row is restored as a point, so the rows stripped
     and the points restored together account for every MATERIAL row of the material.

  Conditions 2 to 7 are checked against the MBF and the sidecar, and the regeneration identity
  then regenerates the whole format-5 sidecar, the `derivation` block included, from the
  restored model; any difference is `MBF-IMPORT-SIDECAR-NOT-REPRODUCED`. These establish
  **consistency** only. A sidecar that drops an authored point's identity and calls its row
  derived can satisfy conditions 2 to 7 and still reproduce (§20.4.6, example 8). Condition 1,
  the source proof, is what refuses it: only the designated source shows which rows were
  authored.

  **Consequence.** A bound import of a Route 1 model is refused wherever the user designates no
  source, whether or not any row is marked generated and whether or not any material has
  points. That includes an import on another machine, unless the user supplies the source there
  (§20.4.5, **Cross-machine imports**). Without its source, a Route 1 MBF whose materials all have
  a single E still imports unbound, with the group listed as lost (**Unbound**, below).
- **Thermal states,** by the recorded kind:
  - `constant_alpha_interval`: coefficient α, ΔT = T1 − T_ref;
  - `free_length_state`: the law id, `operating_temperature` T1;
  - `unchanged_reference`: T1 must equal T_ref, else `MBF-IMPORT-SIDECAR-CONFLICT`;
  - in a sustained case, every member's temperature is T_ref (ΔT = 0, operating temperature
    T_ref), as (b) maps it.
- **Selections.** `temperature_interpolation` at T1 (thermal case) or T_ref (sustained case),
  or `exact_point` with the recorded point id, whose date must equal T1, or
  `explicit_base_properties` with its recorded reference.
- **Ledger.** The ordinary ledger restores the case's primitives (from `load_map`), each with
  factor 1, in the recorded order. Support states are `active_model_device` for each support.
  History is `independent_equilibrium`. The contract string is the supported one.

**Units.** Temperatures are restored in °C (intervals in degC), E in MPa and α in 1/degC, the
file's units, as the existing importer does for E (§7.2). ΔT is formed as the nearest binary64
to the exact decimal difference of the two printed values. The writer's preflight (§14) has
already checked, explicitly, that this value's canonical decimal is that difference exactly and
equals the authored interval (§20.4.2 g). The imported model therefore meets the per-member rule
under exact identity. A value that fails the check is refused at export by name; no digit bound
is assumed.

**Result: what is exact, and what is compared numerically.**

- **Inputs: exact.** On the admitted subset, the exported and imported groups agree exactly as
  rationals in kelvin and SI. This covers T_ref, every T1, every ΔT and operating temperature,
  every selection temperature, every point E and α, and ν. Only the unit spelling changes (for
  example GPa → MPa, degF → degC, K → degC). Identities, kinds, order and provenance text are
  equal through the regeneration identity. Losses are the existing §18.2 exclusions only.
- **Selected E, ν and G: equal bit for bit.** The writer has checked option (a) on both models,
  so each member's selected binary64 E equals d × 10⁶ Pa on each side. ν is the material's
  single ν on both sides, since interpolating equal values returns that value exactly. G is
  derived from those two. The per-member rule therefore holds on both sides. The writer
  reproduces the solver's selection to check this, and the engine test (§20.4.7, item 7) checks
  the reproduction against the solver itself.
- **Thermal strain: numerical agreement, not identity.** Exactly equal inputs do not give
  identical binary64 strains. The solver forms each strain as α_b·(T_op,b − T_inst,b), rounded
  once (`thermal.rs:156-190`, `:531-540`, `:582-594`). Here T_op,b and T_inst,b are the member's
  kelvin representatives, and α_b is the binary64 coefficient in 1/K.
  - *Example.* T_ref 25 degC; operating and selection temperature 467.6 degF, exactly 242 °C;
    α 1.1e-5 1/K. The original's representative of 467.6 degF is 515.1500000000001 K. The
    solver's strain is 0.002387000000000001. Restored in °C, the representative is 515.15 K and
    the strain is 0.002387.
  - *Bound.* Each side is compared with the exact strain s* = α*·(T1* − T_ref*), formed from the
    printed MBF values as rationals. This is §18.2's ΔT class, δ(T1) + δ(T_ref) + 4ε|T1|. Here
    δ = 0, because (g) has proved both printed exactly. The ε term is taken over the solver's
    kelvin conversion instead of the importer's °C arithmetic:

    |s − s*| ≤ |α*|·4ε·(T1_K + T_ref,K + 766 K) + 8ε·|s*|

    It is an absolute bound only, as §18.2's ΔT bound is. ε = 2⁻⁵³.
  - *Derivation.* A representative is v_b·f_b + o_b in binary64. For °F, the canonical decimal's
    own rounding, f_b = fl(5/9), o_b = fl(459.67·5/9) (at most 766ε K) and two roundings give at
    most ε·(4T_K + 1532 K). That is the largest of °C, °F, degR and K, so each representative is
    within 4ε·(T_K + 383 K) of its exact value. The difference is rounded once. α_b is within 3ε
    relative (1/degF converts by fl(9/5)), and the product is rounded once. An interval in degF
    converts by fl(5/9) and is covered by the 8ε term.
  - *Check.* A reproduction of the solver's arithmetic over 200,000 random °C, °F and K
    temperatures and coefficients reached at most 0.23 of this bound.
  - Refusing such cases instead would refuse true equalities, such as example 5d's mixed units,
    over an ulp of conversion. The MBF prints no strain. The target forms its own from T1, T_ref
    and α, which (g) proves exact. That is why the bound, not a refusal, is written.

**What the bound does not cover.** It covers solver agreement only. The equivalence of SWBPIPE's
strain with the target's is today's constant-α reading (G-3), unchanged.

**Unbound.** Unchanged. MATERIAL rows that differ in E are refused,
`MBF-IMPORT-MATERIAL-TEMPERATURE-DEPENDENT` (§7.1), and the O-7 ruling still governs unbound
files. An unbound import never forms the group. A Route 1 file whose materials all have a single
E imports as an ordinary model: thermal primitives ΔT = T1 − T_ref (§9.2), with no reference
configuration, element states or selections. That is a loss of the group's structure, and the
report lists it as such. Forming a group from an unbound file would need its own profile
decision; none is proposed.

**Rebind** (`rebind_by_node_number`) with a format-5 sidecar is refused,
`MBF-IMPORT-REFERENCE-STATE-UNBOUND`. The group's meaning rests on the regeneration identity,
and rebind skips it (§15).

#### 20.4.5 Sidecar format 5 and the provenance carried

**The binding requirement (Agent 0, as quoted in the lane's brief of 2026-11-02).** "Sidecar 5
must bind source + export bytes/identities/derivation provenance so import strips ONLY proven
generated rows and restores authored table order/meaning; no guessed stripping or
forged/mismatched sidecar acceptance."

The writer records **format 5** only for a Route 1 export. It is format 4 plus three changes:

- **`material_map[<name>]` for a material with points.** The single `elastic_modulus_n_per_mm2`
  is replaced by three things. `points` lists the point ids in authored order. `rows` lists the
  rows in MBF order. Each row records `temperature_c`, `elastic_modulus_n_per_mm2` and
  `thermal_expansion_mm_per_mm_per_c` as checked copies (below), plus a `source`. A point row's
  source is `{"point": <id>, "alpha_authored": <bool>, "provenance": <text>}`. A generated row's
  source is `{"derived": {"members": [<pipe id>, …], "bracket": [<point id>, <point id>]}}`,
  with members in authored order. The entry also records `base_point` (identity) and the
  material-level `alpha_authored`. A point-free material's entry is format 4's.
- **`reference_state_map`**:
  - `configuration`: `id`, `label` (null when absent, as format 4's null section), `provenance`,
    and `members`, a map from pipe id to `provenance`;
  - `laws`: from material id and law id to `provenance`;
  - `cases`: from case id to:
    - `role`, `provenance`, and `ledger` (source ids in order);
    - `supports` (support ids in order);
    - `elements` (in authored order), each carrying `pipe`, `thermal` (the kind),
      `thermal_provenance` (null for `free_length_state`, which has none), `law` (null unless
      free length), `selection` (`{"kind": "temperature_interpolation"}`,
      `{"kind": "exact_point", "point": <id>}` or
      `{"kind": "explicit_base_properties", "applicability_reference": <text>}`),
      `operating_temperature_authored` (bool), and `override` (null or `{reason, provenance}`).
- **`derivation`**, which binds the bytes and names the method:
  - `source_model_sha256` is the SHA-256, as 64 lowercase hex digits, of the RFC 8785 (JCS)
    canonical text of the model document the writer read. That is the serialization the
    self-digest already uses (`sidecar_canonical_bytes`, `si_writer.py:281-290`), and the text
    the desktop hashes as a document's model hash, so a designated source can be checked
    against it whatever bytes it was read from. The digest verifies a source; it is never used
    to find one (below, **The designated source**).
  - `selection_semantics` is the fixed name `swbpipe_case_state_selection_binary64_v1`. It
    stands for the solver's selection as §20.4.2 (e) cites it. A later change to those semantics
    changes the name, and a sidecar naming other semantics is refused
    (`MBF-IMPORT-SIDECAR-INVALID`).
  - `render_profile` is the fixed name of the writer's byte rendering of Route 1 exports
    (below, **Export inputs outside the source document**).
  - `export_inputs` records the writer arguments that change the bytes and do not come from
    the model document: `title`, the printed title line, and `piping_code`, the OPTIONS
    piping-code token or null (below).
  - The export bytes stay bound by `mbf_sha256`, as in every format. Under format 5, the source
    proof also requires them to be the importer's own rendering of the designated source
    (§20.4.4, check 2 (a)).

**Numerical values in the sidecar.** The sidecar carries checked copies of MBF-determined
values, the row temperatures, E and α in `rows`. It has no independent numerical authority.
The import reads every value from the MBF. It then verifies that each copy equals the MBF value
it copies, and that the regenerated derived rows equal the MBF's rows (§20.4.4). A copy that
differs is `MBF-IMPORT-SIDECAR-CONFLICT`. The copies let the regeneration identity compare row
provenance with values, so that a relabelled row is refused. The absence of numerical authority
is not by itself the reversibility proof. The proof is the designated source's verification
below and in §20.4.4 (**Source proof**), the proven stripping of §20.4.4 and the regeneration
identity; the last establishes consistency only. The every-field-required rule, the self-digest and the
parser of §15 apply unchanged.

**Export inputs outside the source document.** Check 2 renders the designated source again, so
it needs every input that the export's bytes depend on. The writer's bytes are a function of the
model document and of its arguments (`render_caepipe_mbf_si`, `si_writer.py:418-442`; the
engine twin's `WriterSettings`, `core/handoff/caepipe_mbf_engine/src/writer/mod.rs:398-408`).
Each input outside S:

| Input | Effect on the bytes | Format 5 records | Check 2's rendering |
|---|---|---|---|
| Title | the title line only, printed as `_title(title)` (`si_writer.py:1785`, `:2136-2142`) | `export_inputs.title`: the printed title line, by value | uses it. `_title` returns its own output unchanged (printable ASCII without commas, stripped, at most 70 characters), so the printed line reproduces itself |
| Piping code | the OPTIONS line, and OPCODE for an ASME B31 code (`si_writer.py:1785-1787`) | `export_inputs.piping_code`: the token, or null | uses it. The desktop's export settings carry no piping code (`export_settings.rs:7-13`, `:96-103`; `WriterSettings::new` defaults it to null, `writer/mod.rs:411-418`), so a desktop export records null. Under Route 1 the code also decides admission (proposed, `MBF-SI-ROUTE1-PIPING-CODE-IGNORES-MODULUS-FLAG`, §20.4.3), and a null code leaves G-1 (e) open |
| Reference temperature | OPTEMP field 1 | not recorded | omits it. Under Route 1, OPTEMP comes from S's configuration and never from the argument, and an explicit argument that differs is refused (§20.4.2 a, `MBF-SI-REFERENCE-TEMPERATURE-CONFLICT`). An omitted argument and an accepted equal one must print the same field (§20.4.7, item 6) |
| Layout-row limit | none; it raises only a warning (`si_writer.py:1867-1871`). The desktop fixes it at none (`export_settings.rs:84-86`) | not recorded | none |
| Library store content | none. The desktop's export reads the store only to screen the document's dispositions (`apps/desktop/src-tauri/src/mbf_export.rs:1294-1302`). The admission returns the canonical text unchanged (`model_document_save.rs:279-283`), and the worker renders that text (`mbf_export.rs:1315`). The writer's section library is the document's own `sections` (`si_writer.py:797-803`) | not recorded | reads no store. A library changed or deleted since the export does not affect it |
| Unit system | SI only, the writer's one rendering; the document's own project units decide the conversions (`si_writer.py:612`) | covered by `render_profile` | n/a |
| Writer build | potentially every byte | `derivation.render_profile` | requires the importer's own profile; otherwise `MBF-IMPORT-RENDER-PROFILE-UNSUPPORTED` |

- **Recorded inputs are attacker-controlled, and why that is safe.** Anyone who edits the sidecar
  and recomputes its digests can change `export_inputs` and `render_profile`. They only
  parametrize the rendering of the source the user designated. They never select a source, and
  they never stand in for any of S's values: every other byte of the expected MBF comes from S,
  so check 2 (a) still requires every model record to be S's own. An edited input can change
  only these:
  - the title line, and with it the restored project name (`importer.py:3265-3267`). That name
    is the export's label, chosen at export; no format restores S's project name;
  - the OPTIONS piping-code token and, when the code changes between an ASME B31 code and
    another, the associated OPCODE record (`si_writer.py:1786-1788`). The importer keeps them
    only in the import result's `target_options`, listed as delegated, pass-through target
    configuration (`importer.py:1637-1643`, `:264`). They change no restored model value.
    Under the proposed Route 1 non-admission (§20.4.3), an edit to a listed token is refused at
    check 2, because the writer refuses to render S with it;
  - through any of these byte changes, the MBF digest, and with it the import-generated project
    id and digest description (`importer.py:3267-3268`).

  These are import-envelope fields, not restored source identities. Engineering values and
  source-bound entity identities remain determined by S.

  A different `render_profile` can only cause a refusal. Neither input changes the meaning of
  restored model data, so neither needs binding to the designation. A later writer argument
  that could change that meaning must be bound to S or refused; it may not be added to
  `export_inputs`.
- **Writer-build drift: an open requirement.** Today's code has no identifier for the writer's
  byte rendering. `MBF_SI_PROFILE_ID` (`si_writer.py:34`) appears only in `to_dict()` (`:216`),
  not in any sidecar, and `MBF_VERSION` (`:37`) is the target format's version, which the file
  prints. `render_profile` is new. Its rule: any writer change that alters the bytes of any
  admitted Route 1 export changes the name. The profile check catches only a declared
  mismatch; it does not catch a forgotten bump (Agent 0, 2026-11-02; paraphrase in O-7). Two
  requirements cover that case:
  - **Deterministic re-render equality.** Rendering the same source with the same export inputs
    gives the same bytes every time, in the Python and the Rust writer alike; the writer reads
    no clock, environment or randomness. Check 2 (a) then requires the importing writer's own
    rendering to equal the presented MBF byte for byte. A pair whose bytes that writer no longer
    reproduces is refused, whatever profile it declares; it is never accepted.
  - **Tests that detect an unbumped rendering change.** A pinned digest of the Route 1 writer
    vectors' rendered bytes, keyed by `render_profile`, fails when those bytes change and the
    name does not (§20.4.7 item 5a). It detects a change only for the inputs the vectors cover;
    beyond them, the rule rests on review, and the re-render equality above still refuses
    rather than accepts.

  An importer renders only with its own writer. Keeping older writers, to import older
  profiles, is not proposed. A refused pair's MBF still imports unbound. The Python and Rust
  writers must render the same bytes, as writer parity already requires.

**What binds what, and what is refused.**

- **Export bytes.** `mbf_sha256` decides the route, as today. A mismatch makes the import
  unbound, and an unbound import never forms the group (§20.4.4). On a bound format-5 import,
  the bytes must also equal the importer's own rendering of the designated source (§20.4.4,
  check 2 (a)); `mbf_sha256` alone does not tie them to the source.
- **Sidecar bytes.** The self-digest covers every field, `derivation` included. A mismatch is
  `MBF-IMPORT-SIDECAR-DIGEST-MISMATCH`, as today.
- **Identities and meaning: consistency.** The regeneration identity regenerates the whole
  format-5 sidecar from the restored model and the given MBF digest, and compares it. That
  includes `rows`, their sources and `derivation`, except `source_model_sha256` and
  `export_inputs`. It takes those two as recorded, as it takes the MBF digest, and check 2
  verifies them. A sidecar whose
  rows, sources, members or brackets do not reproduce is `MBF-IMPORT-SIDECAR-NOT-REPRODUCED` or
  `MBF-IMPORT-DERIVED-ROW-UNPROVED`. The self-digest and the regeneration identity establish
  that the sidecar, the MBF and the restored model are consistent. They do not establish that
  the sidecar describes the model that was exported.
- **Source bytes: authenticity.** The restored model is not byte-equal to the source, because
  units are respelled, so `source_model_sha256` cannot be regenerated. Authenticity comes only
  from a source the user explicitly designates (below). Its canonical SHA-256 must equal the
  recorded digest. The importer's independent rendering of it must reproduce the presented MBF
  bytes, and the sidecar formed for the rendered digest must equal the presented one (§20.4.4,
  **Source proof**); `MBF-IMPORT-SOURCE-MISMATCH` otherwise. The import then compares the
  restored model with the source under §20.4.4. A bound format-5 import with no designated
  source is refused, `MBF-IMPORT-SOURCE-UNASSOCIATED`, whatever the sidecar marks and whether
  or not any material has points (below, **Point-free materials**).

**The designated source: the only adopted association.** Agent 0 approved explicit user
designation as the initial bound-import route (2026-11-02; paraphrase in O-7). The source S is
associated with the export by the user's explicit designation, made independently of the
sidecar. The steps run in this order:

1. **Designation, before any check.** The caller designates S:
   - in the desktop, a saved state the user chooses from a project's state list, read and
     re-hashed by `read_state_for_in_store_with` (`model_states.rs:2377`), which refuses a
     snapshot whose stored text no longer matches its hash (`:2438-2443`); or a model document
     the user supplies, through the Open Model Document route's checks (desktop design §4.1);
   - in the headless importer, its source-document parameter, as the round-trip tests use it.

   The state list is shown as it is today. It is not filtered, ranked, preselected or annotated
   by `source_model_sha256` or by any other sidecar field. A document lying beside the MBF, or
   a copy carried inside the sidecar, is a source only when the user names it.
2. **Hash.** Check 1 of §20.4.4's source proof: S's canonical SHA-256, computed from S's own
   text, equals the sidecar's `source_model_sha256`.
3. **Content.** Check 2: the importer renders S itself with the recorded export inputs. The
   SHA-256 of the rendered MBF bytes equals that of the presented MBF (a), and the format-5
   sidecar formed from S for the rendered digest equals the presented sidecar in every field
   (b). The presented digest is never injected into what is expected.

A designated source that fails either check is `MBF-IMPORT-SOURCE-MISMATCH`. An import with no
designation is `MBF-IMPORT-SOURCE-UNASSOCIATED`. Either way nothing is restored or stripped, and
the import does not fall back to any other association. The importer never searches a store, a
project or a folder by the sidecar's digest: it has no digest lookup. The import review names the
designated source and how it was designated. Nothing else is a source: not the session's
restored document, not a state found by the sidecar's digest, not a file found beside the MBF
and not a copy carried inside the sidecar.

*Why the digest must not select.* One project holds saved state A, EX-A's three points, and saved
state B, the same table plus a redundant authored point at 175 °C (§20.4.6, example 8). Both write
the same MBF bytes. The user exports B. An attacker substitutes a sidecar regenerated from A,
which the writer forms deterministically from A even if A was never exported. A lookup by the
digest would find genuine A, every check would agree with the substituted sidecar and the MBF,
and B's authored point would be stripped. The association therefore comes from the user, never
from the sidecar (example 8d).

*What designation establishes, and what it does not.* With both checks passed, the presented MBF
is, byte for byte, the importer's own rendering of the designated source under the recorded
export inputs. Every field of the presented sidecar is the one the writer forms from that source
for those bytes. The import therefore restores that source's own values, MBF-only values such as
section dimensions and spring stiffness included. It restores that source's identities, order,
classifications, provenance and reference-state metadata, and strips only the rows that source's
own export generates. The recorded export inputs decide only import-envelope fields: the title
line and restored project name, the OPTIONS piping-code token and its OPCODE record, and, through
the bytes, the import-generated project id and digest description (above, **Export inputs
outside the source document**). Engineering values and source-bound entity identities remain
determined by S. Designation decides
*which* source; the checks verify only that the pair is that source's export content. They do
not show that the pair was exported from that source rather than from another with the same
canonical text, which is the same model. A pair that the writer forms from the designated source
is accepted whether or not that source was ever exported. In each case what is restored is
exactly the designated source.

- **Cross-machine imports.** A bound import of a Route 1 export on another machine requires the
  user to supply the exact S there: a model document of the exported text (for example, one
  saved with Save Model Document and carried with the MBF), or a saved state of that text in
  the receiving store. Without it, the bound import is refused
  (`MBF-IMPORT-SOURCE-UNASSOCIATED`). The MBF without its sidecar still imports unbound
  (§20.4.4, **Unbound**).

**The export review's notice (Agent 0's Open 2, 2026-11-02; paraphrase of the ruling).** Export
MBF keeps no record of its source (below, **Persistence**), so every Route 1 export is
source-free in that sense. It may remain offered when it is otherwise eligible. Its complete
review (desktop design §3.3) states clearly, before Save is offered, that:

- a later bound import of this pair requires the user to designate the exported source,
  independently of the files;
- the user must then have access to the exact source: a saved state or a model document whose
  canonical text is the exported text. Where the exported text is unsaved, keeping it is the
  user's choice, through the existing Save Model State or Save Model Document commands;
- no automatic round trip is promised.

The export's format and authority are unchanged: it writes the MBF and its sidecar and persists
nothing about its source. The notice is desktop work, among the export review's Route 1 entries.

**Point-free materials (Agent 0's Open 4, 2026-11-02; the ruling paraphrased, the analysis
here).**

- **What format 4 carries, at its existing assurance.** §15 lists the format-4 fields:
  `node_map`, `element_map`, `support_map`, `load_case_map`, `material_map` (a material's
  `swbpipe_material` id and `origin`), `section_map`, `load_map`, `pressure_map`, `mbf_sha256`
  and `sidecar_sha256`. A bound format-2, -3 or -4 import restores from them what the MBF does
  not carry (§15, **Bound**), under §15's checks against the MBF and the regeneration
  identity, and consults no source. §14 (**Consistent edits**) lists the values that pass from
  the model to the sidecar unchanged, so that a consistent, re-digested edit of one is accepted:
  ids (format 4's section ids included), `reversed`, an authored `y_reference` or `family`, a
  recorded `origin`, a thermal direction, the shear model of a contract, region ids, and
  closure transfers with their provenance. Those fields remain at format 4's adopted
  consistency-only assurance (O-6). This Design leaves that unchanged for every format-2, -3 and
  -4 import.
- **What format 5 adds, point-free materials included.** A point-free material's `material_map`
  entry is format 4's. But every format-5 sidecar carries `reference_state_map` and
  `derivation`, whatever its materials, and a bound format-5 import restores from
  `reference_state_map` meaning that no format-4 import restores (§20.4.4, **What the import
  restores**; a format-4 or unbound reading gives thermal primitives only, **Unbound**):
  - *reference metadata:* the configuration's id, label and provenance, each member reference's
    provenance, and with them the reference configuration itself (installation temperature T_ref,
    basis `temperature_reference`, fit `none`);
  - *thermal metadata:* each member's thermal-state kind and `thermal_provenance`, a
    `free_length_state`'s law, the laws' ids and provenance, and
    `operating_temperature_authored`;
  - *selection metadata:* each member's selection kind, an `explicit_base_properties`
    selection's `applicability_reference` text, and an `override`'s reason and provenance;
  - each case's `role`, `provenance`, `ledger` order and `supports` order.
- **Counterexample: consistency alone admits a mutation.** §20.4.6, example 9: a point-free
  export whose sidecar is edited to change a member's thermal kind, add a law, change an
  `applicability_reference` text and change the configuration's id and provenance, with the
  self-digest recomputed and the MBF unchanged. The restored model regenerates the edited
  sidecar exactly, and its strain agrees with the original's within §20.4.4's bound, so the
  regeneration identity and every numerical check pass. Point-free format 5 therefore restores
  thermal, reference and selection metadata beyond format 4's consistency level, and the
  distinction is resolved against an exemption.
- **The rule.** Every bound format-5 import requires a designated source that passes both
  checks, whether or not any material has temperature points. Points remain the minimum
  mandatory guard of Agent 0's earlier ruling; this rule waives no other metadata. Refusing
  point-free bound format-5 imports outright was the alternative; designation is chosen because
  check 2 verifies every MBF byte and every sidecar field. Without its source, a point-free Route 1 MBF still imports
  unbound as an ordinary model, the group listed as lost.
- **Why designation suffices.** The import restores only from the MBF and the sidecar. Check 2
  (a) makes the MBF the importer's own rendering of S, so every value the MBF carries is S's.
  Check 2 (b) makes the sidecar equal, field for field, the one the writer forms from S for
  those bytes. Every restored value, identity and item of metadata is therefore S's own, the
  format-4 maps inside a format-5 sidecar included. The only exceptions are the import-envelope
  fields that the export inputs decide: the title line and project name, the piping-code token
  and its OPCODE record, and the import-generated project id and digest description (above).
  None is a restored source identity or engineering value. The stripping
  conditions and the regeneration identity then run as consistency checks on top. The
  assurance is relative to S, and S is the user's designation (above).

**What "no forged sidecar acceptance" covers.** The binding requirement's "no ...
forged/mismatched sidecar acceptance" is met here in this exact sense: no bound format-5 import
accepts a pair unless all three of these hold. The source the user designated has a canonical
SHA-256 equal to the sidecar's `source_model_sha256`. The MBF is, byte for byte, the importer's
own rendering of that source under the recorded export inputs. The sidecar equals, in every
field, the one the writer forms from that source for those bytes. It does not cover:

- format-2, -3 and -4 sidecars, which keep format 4's consistency-only assurance (above);
- the choice of source: a pair the writer forms from the designated source is accepted
  whether or not that source was exported (above, *What designation establishes*);
- the export inputs, the title and the piping code. They are the export's choices, not the
  source's. Editing them changes only import-envelope fields: the title line and restored
  project name, the delegated piping-code token and its OPCODE record, and, through the bytes,
  the import-generated project id and digest description (above, **Export inputs outside the
  source document**);
- an MBF imported without its sidecar, which is unbound and forms no group.

**Persistence: the adopted design adds no store.** Designation reads an existing saved state
through an existing read, or a document the user supplies. It adds no table, column, migration,
state or file, and it changes no deletion or retention behaviour. Export MBF writes its two
files as the desktop design gives them (desktop design §3.6) and nothing else. This Design grants
no schema, persistence or retention authority. The export ledger at the end of this subsection
is a proposal outside the adopted design.

- **No residual acceptance.** Agent 0 ruled that every bound format-5 import containing
  temperature-point materials requires a verified source (2026-11-02; paraphrase), and approved
  explicit designation as the route. No sidecar is admitted on its own word: one that marks an
  authored row as generated, one that relabels a generated row as a point (example 8c), and one
  that changes authored identities, order or provenance are each refused, by the source proof
  when a source is designated and by `MBF-IMPORT-SOURCE-UNASSOCIATED` when none is. The earlier
  residual, an added point imported with the source digest reported as unverified, is
  withdrawn. Point-free materials need a designated source too (above, **Point-free
  materials**).

**Backwards compatibility and refusals (required before implementation).**

- **Formats 2, 3 and 4 are unchanged.** They parse and route as today. Every export outside Route
  1 is written byte for byte as today, with a format-4 sidecar. No option writes format 5 for a
  model outside Route 1. No option writes a Route 1 export with a format-4 sidecar, which would
  drop the group without saying so.
- **A format-4-only importer meeting format 5** (`main` today, and any build before format 5).
  Its self-digest check covers formats 3 and 4 only. Its strict parser then refuses
  `sidecar_format` 5 before any binding (`importer.py:812`, `:1176-1179`, `:1660-1665`), so the
  whole import is `MBF-IMPORT-SIDECAR-INVALID`, bound or not. It neither falls back to format 4
  nor reads the MBF unbound. Given the MBF without the sidecar, it imports unbound: varying rows
  are `MBF-IMPORT-MATERIAL-TEMPERATURE-DEPENDENT`, and single-E files import as an ordinary
  model with the group listed as lost. The format-4 behaviour is pinned by a test before format
  5 lands (§20.4.7, item 9).
- **A format-5 importer meeting a mismatched pair.** A format-5 sidecar beside an MBF with no
  Route 1 content, or a format-4 sidecar beside a Route 1 MBF whose rows differ in E, is refused:
  `MBF-IMPORT-SIDECAR-CONFLICT`, or `MBF-IMPORT-MATERIAL-TEMPERATURE-DEPENDENT` as today.
- **No silent product-schema bump.** Format 5 is a writer record. It changes no model-document
  schema or version, and `schemas/caepipe_mbf_export.schema.json` carries no sidecar format.
  Should the implementation need either to change, its PR states the change and versions it
  explicitly.
- **Scrutiny.** Format 5 changes persistence. Its implementation PR takes an independent
  persistence review of the exact format, these refusals and the compatibility tests, under
  the local loop's rules for persistence changes. The export ledger is not part of the adopted
  design; any ledger needs its own separate design review (below, **Proposed, not adopted,
  unimplemented**).

**Where provenance lives.** The **ledger** is in the MBF's case membership (`CAEPIPE-LOAD-CASES`)
and in `load_map` and `reference_state_map.cases[].ledger`. **Temperatures:** T_ref is in
OPTEMP field 1, each T1 in a LOADS record and its `L=`, and point dates in MATERIAL rows.
**Modulus basis:** the selection kind and point are in `reference_state_map`, the values in
MATERIAL rows, the option in OPTEMP (G-1). **Strain:** the kind is in `reference_state_map`;
α is in MATERIAL and T1 − T_ref in OPTEMP and LOADS. **Authored text** (provenance strings,
override reasons, labels) is in `reference_state_map`, as closure transfers already travel in
`pressure_map` (§15).

The writer's result gains a `reference_state` block in `to_dict()` (absent outside Route 1, so
existing outputs are unchanged). It holds one row per member and case: the kind, T_sel, T1,
selected E and its source (point or derived), α, and the strain α(T1 − T_ref). The import
report gains the same block. The export review (desktop design §3.3) can show it; that is
desktop work, not part of this subsection. The solver's own per-case evidence
(`resolve.rs:976-1000`) is unchanged.

##### Proposed, not adopted, unimplemented: a trusted export ledger (store v14)

**Status (Agent 0, 2026-11-02; paraphrase).** Not adopted, and not implemented. It is eligible
only for a separate design review. Nothing here is a requirement on the writer, the importer or
the desktop. This Design grants no schema, persistence or retention authority, and the adopted
design still adds no store (above, **Persistence**). The ledger would be substantive new
persistence: no part of it exists in today's code. Its exact design, and its old and new
commitments, go to independent review and Agent 0's adoption before any migration or command;
there is no current shared-store grant. Until such a design and its implementation are adopted,
every bound format-5 import requires designation, with no digest-lookup fallback.

**Motivation.** Designation asks the user to keep and name the exact source. A record written
by Export MBF could associate a pair with its source on the same machine without that step.

**What exists today** (facts a ledger would change). No record associates an MBF export with its
source. Export MBF is given the session's canonical text and its claimed hash, not a project or a
saved state (`MbfExportRequest`, `apps/desktop/src-tauri/src/mbf_export.rs:180-193`). It admits
that text as its own canonical form with that hash (`model_document_save.rs:259-282`), writes
two files under a file-level rollback, and opens the store only for library reads
(`mbf_export.rs:1266-1305`, `:1394-1410`). It writes nothing to the store, and its production
entry refuses every request as not connected (`mbf_export.rs:1415-1438`). The store's
migrations (`lib.rs:659-819`; v12 `model_states`, `model_states.rs:51-79`; v13 `analysis_runs`,
`analysis_runs.rs:60-89`) hold projects, libraries, states and runs, not exports. Report-package
and handoff saves write files only. The exported text may be unsaved: a state exists for it only
if the user saved one. Tests that use v14 as a newer store's version (for example
`model_states.rs:3411`) would move with any v14 migration.

**The candidate drafted at `f3d08f015` (recorded, not adopted).** A store v14 table with one
row per format-5 export with temperature-point materials, holding the project id, a saved-state id, `source_model_sha256`, the
MBF's and the sidecar's SHA-256, `created_at` and a record hash, append-only under v12's and
v13's trigger pattern. It would be written after the read-back of both files, in one
`BEGIN IMMEDIATE` transaction that also saved the exported text as a new `handoff_basis` state
when no state held it, with a new code `MBF-EXPORT-RECORD-FAILED` and the file rollback on
failure. At import, the desktop would look up records by its own digests of the presented bytes
and re-verify the named state on read. None of this is adopted. The candidate's claims are the
open requirements below, not properties.

**Open requirements a separate design must establish (Agent 0, 2026-11-02, with the independent
re-review at `f3d08f015`; paraphrase).**

1. **An association key** independent of the MBF and sidecar digests, which an attacker
   controls.
2. **Two legitimate exports with the same MBF from different states.** *Counterexample (re-review
   P1).* State A (example 1) and state B (example 8's source) write the same MBF bytes, and the
   user exports both, so the ledger holds both records. An attacker replaces B's sidecar with A's
   genuine recorded sidecar. The candidate's lookup matches A's record and restores A, and B's
   authored point `point:EX-A-175` is removed: a recorded sidecar selects between different
   authored sources. The candidate's reading of this as a faithful round trip does not hold. The
   re-review's suggested rule, recorded for that design and not adopted: "When records sharing an
   MBF digest identify different source-model digests, automatic source association is refused;
   caller must designate; repeated records of the same source do not create ambiguity."
3. **Record authenticity and provenance.**
4. **Matching of the full pair.**
5. **Project and state selection.** An import forms a new unsaved project (desktop design §4.1
   step 5, §4.4), so the candidate searched the whole store.
6. **Atomic filesystem-plus-database failure ordering.**
7. **Recovery, crash and rollback.** The candidate claimed that a crash between the read-back
   and the commit fails closed. That holds only where no earlier record matches the same pair.
   An earlier export of the same pair has a record, and the uncommitted export's files would
   import through it. The design must state the behaviour in both cases.
8. **Idempotence and duplicate exports.**
9. **Orphan-state cleanup.**
10. **Consent and storage for unsaved text.** Saving unsaved session text as a `handoff_basis`
    state is a new write, consent and storage effect. It is not automatically equivalent to
    Record Run's authority (`analysis_runs.rs:26-28`, `:1744-1760`).
11. **Retention: not decided (Agent 0's Open 1; paraphrase).** Indefinite retention or
    non-deletion is not adopted by analogy. The candidate would have blocked deleting a state
    with an export record, with no record-retirement action. That is stronger than the
    recorded-run precedent: a state's runs block its deletion (`analysis_runs.rs:66-69`;
    `model_states.rs:2672-2690`), but `delete_analysis_run` removes a run
    (`analysis_runs.rs:2094-2130`), after which the state can be deleted. A design must specify
    reference integrity, deletion and refusal behaviour, and its effect on existing user
    deletion or retirement. Current behaviour is preserved; restricting an existing authorized
    delete path needs exact review.

#### 20.4.6 Worked examples (invented values)

Common to every example:

- **Material `material:EX-A`.** E/ν basis, ν 0.29, α 1.1e-5 1/K throughout. Points
  `point:EX-A-025` at 25 °C with 204 GPa, `point:EX-A-150` at 150 °C with 196 GPa, and
  `point:EX-A-250` at 250 °C with 188 GPa. The base equals `point:EX-A-025`.
- **Configuration.** One configuration: every member `temperature_reference` at 25 degC, fit
  `none`.
- **Cases.** One thermal case, `case:EX-T`, with no sustained case, so G-2 does not arise.
- **MBF values below** are the records the writer would print.

**1. Admitted, interpolated and exact point.** The members:

- `pipe:EX-1` is `free_length_state`, with law `law:EX-A` (constant secant α 1.1e-5 1/K,
  datum 25 degC), `operating_temperature` 175 degC and `temperature_interpolation` 175 degC.
- `pipe:EX-2` is `constant_alpha_interval` {1.1e-5 1/K, 125 K}, with `exact_point`
  `point:EX-A-150`.

The MBF records:

- OPTEMP field 1 is `25`.
- MATERIAL rows: `25,204000,1.1e-05`; `150,196000,1.1e-05`; `175,194000,1.1e-05` (derived:
  196 + 0.25·(188 − 196) GPa); `250,188000,1.1e-05`.
- LOADS: `L1,175,0,0` for EX-1 and `L2,150,0,0` for EX-2.

The equivalence:

| Member | SWBPIPE E and strain | MBF E and strain |
|---|---|---|
| EX-1 | E = 194 GPa; strain 1.1e-5·150/(1 + 1.1e-5·0) = 1.65e-3 | E = 194000 MPa at 175; strain 1.1e-5·(175 − 25) |
| EX-2 | E = 196 GPa; strain 1.1e-5·125 = 1.375e-3 | E = 196000 MPa at 150; strain 1.1e-5·(150 − 25) |

The bound round trip, with the exported model designated as the source (§20.4.5), restores 25,
175, 150, 125, the three points and α exactly, drops the derived row, and the per-member rule
holds. Without a designated source, the import is refused, `MBF-IMPORT-SOURCE-UNASSOCIATED`,
before anything is restored or stripped. The comparison is equal with no tolerance used (only E
prints differently, as MPa).

**2. Mismatch.** As example 1, but EX-1 selects `exact_point` `point:EX-A-150`, with an
`analysis_basis_override` that the solver needs for 150 ≠ 175 (`material.rs:155-157`). The
solver admits it, with E = 196 GPa. In the MBF, the target would use E at T1 = 175, which is
194 GPa. The export is refused: `MBF-SI-MODULUS-SELECTION-MISMATCH` at `pipe:EX-1`, "selects
its modulus at 150 degC; its MBF T1 is 175 degC". No file is written.

**3. `unchanged_reference`.**

- **3a.** `pipe:EX-3` is `unchanged_reference`, selecting `temperature_interpolation` 175 degC
  (the pattern RS-P180 uses). Its T1 is T_ref = 25, because the configuration establishes 25 as
  its installation temperature. 175 ≠ 25, so `MBF-SI-MODULUS-SELECTION-MISMATCH` at `pipe:EX-3`.
- **3b.** EX-3 instead selects `exact_point` `point:EX-A-025`. Then T1 = 25 = the selection:
  admitted, with `L3,25,0,0`. The strain is 0 on both sides (α·0), and E is 204 GPa on both.
  The round trip restores `unchanged_reference` from the sidecar and checks T1 = T_ref.
- **3c.** As 3b, but EX-3's member reference is `direct_strain_reference`.
  `MBF-SI-UNCHANGED-REFERENCE-UNESTABLISHED` at `pipe:EX-3`. No reference establishes T_ref,
  and the writer's 20 °C default is not used to supply one.

**4. Reference datum.**

- **4a.** As example 1, but `law:EX-A` has datum 0 degC. The SWBPIPE strain is
  1.1e-5·150/(1 + 1.1e-5·25) = 22/13337 ≈ 1.649546e-3, 2.75e-4 relative below the 1.65e-3 that
  α·(T1 − T_ref) states. `MBF-SI-EXPANSION-DATUM-UNSUPPORTED` at `law:EX-A` (and at `pipe:EX-1`).
  Writing T1 ≈ 174.9588 to absorb the factor would manufacture an equivalence. It would also
  break the per-member rule, since the selection is at 175. It is not done.
- **4b.** EX-2's installation temperature is 30 degC, the others' 25 degC:
  `MBF-SI-REFERENCE-DATUM-MULTIPLE`.

**5. Unit boundaries.**

- **5a, °F.** The material's α is 6.1e-6 1/degF throughout (its points too). T_inst 77 degF;
  EX-2 `constant_alpha_interval` {6.1e-6 1/degF, 270 degF}; selection
  `temperature_interpolation` 347 degF. Exactly: 77 °F = 25 °C, 270 °F interval
  = 150 K, and 347 °F = 175 °C. 25 + 150 = 175, so the member is admitted. The MBF has OPTEMP
  `25`, T1 `175` and α `1.098e-05` (6.1e-6 × 9/5). The round trip returns 25 degC, 150 degC
  interval, 175 degC and 1.098e-5 1/degC: equal as rationals, with degF spelled as degC.
- **5b, °F not printable.** T_inst 70 degF = 21.1111… °C, which prints as 21.11111111.
  `MBF-SI-REFERENCE-STATE-NOT-PRINTABLE`, naming the value. A native English-mode writer (O-2)
  would remove this boundary.
- **5c, K.** T_inst 298.15 K, ΔT 150 K and selection 448.15 K: admitted, with OPTEMP `25` and
  T1 `175`.
- **5d, mixed units.** T_ref 25 degC; EX-2 `constant_alpha_interval` with ΔT 217 K; selection
  467.6 degF. Exactly, 467.6 °F
  = 242 °C = 25 + 217, so the member is admitted with T1 `242`. Binary64 conversion through
  Rankine gives (467.6 + 459.67)·5/9 = 515.1500000000001 K against 515.15 K. A writer comparing
  binary64 values would refuse a true equality. The exact identity (§20.4.2 g) does not.
- **5e, strain after °F restoration.** As 5d, but EX-2 is `free_length_state` with
  `operating_temperature` 467.6 degF. The solver's strain on the original is 0.002387000000000001
  and on the restored model 0.002387. Inputs are exact, and E is 188640 MPa on both sides. The
  strains agree within §20.4.4's bound, not bit for bit.

**6. Derived rows under option (a).** The values were computed with the solver's arithmetic: a
reproduction of `material.rs:300-326` and the kelvin representatives, with `fma`.

| Member | Selected E (binary64 Pa), original and restored | Row d (N/mm²) | (a) |
|---|---|---|---|
| EX-1, interpolated at 175 degC | 194000000000.0 | 194000 (derived) | admitted |
| EX-2, exact point at 150 degC | 196000000000.0 | 196000 (point) | admitted |
| EX-3 (3b), exact point at 25 degC | 204000000000.0 | 204000 (point) | admitted |
| 5a, 347 degF; 5c, 448.15 K | 194000000000.0 | 194000 (derived) | admitted |
| 5d and 5e, 467.6 degF | 188640000000.0 | 188640 (derived) | admitted |
| 6a: points 204 GPa at 25 degC and 196 GPa at 100 degC, selected at 50 degC | 201333333333.33334 | 201333.3333 | refused, `MBF-SI-MODULUS-NOT-EXACT` |
| 6d: points 185 GPa at 0 degC and 150 GPa at 50 degC, selected at 0.16 degC | 184887999999.99997 | 184888 (the exact rational interpolation) | refused, `MBF-SI-MODULUS-NOT-EXACT` |
| 6e: points 189 GPa at 3 degC and 177.5 GPa at 78 degC, selected at 7.4 degC; and at 45.32 degF | 188325333333.33334; 188325333333.3333 | 188325.3333 | each refused by its own id, `MBF-SI-MODULUS-NOT-EXACT` |

- **6b, a duplicate T1.** EX-1 and a second member EX-1b, both at 175 degC, share one derived
  row whose `derived.members` lists both. If EX-1b authors 347 degF, its own representative is
  used. Here it gives the same E, so one row serves both. Where two spellings give E one ulp
  apart (§20.4.2 e), the member that differs from d × 10⁶ is refused by its id.
- **6c, a T1 that collides with an authored row.** A member at 150 degC by interpolation takes
  `point:EX-A-150` itself. No derived row is written, and the row's source is the point.
- **6f, boundaries.** Members of EX-A at 25 degC and at 250 degC by interpolation take the
  endpoint points (204000 and 188000 N/mm²): no derived row, admitted.

**7. The bounded exception for undated selections (decision 3).**

- **7a, admitted.** Point-free material `material:EX-B`: E 200 GPa, ν 0.3, α 1.2e-5 1/K.
  `pipe:EX-5` is `constant_alpha_interval` {1.2e-5 1/K, 150 K} with `explicit_base_properties`.
  The writer writes three identical rows at 200000 N/mm². The solver's E is 200000000000.0 Pa on
  both models, and LOADS T1 is 175.
- **7b, a conflicting selector.** EX-4 is made of EX-A, which has points, is `unchanged_reference`
  at T1 = 25, and selects `explicit_base_properties`. Its base equals `point:EX-A-025`, so the
  selected 204 GPa equals the row at 25. It is still refused, `MBF-SI-MODULUS-SELECTION-UNDATED`
  at `pipe:EX-4`. The material's E is not constant, and the agreement at one temperature is not
  read as a dated selection. A selection whose `material_ref` names EX-B while EX-4's pipe is of
  EX-A is `MBF-SI-MODULUS-SELECTION-CONFLICT`.
- **7c, a record basis that differs.** As 7a, but EX-B's E is 29e6 psi. Its basis is constant,
  but the MBF record cannot state it. It is 199947.9615018825… N/mm², which prints as
  199947.9615. Refused: `MBF-SI-MODULUS-NOT-EXACT` and `MBF-SI-REFERENCE-STATE-NOT-PRINTABLE`.
  Separately, as 7a but with EX-5's member reference `direct_strain_reference`: refused,
  `MBF-SI-REFERENCE-BASIS-UNSUPPORTED`. The reference record supplies no temperature, so the
  absence of a selection temperature does not stand in for one.
- **7d, ν or α conditions that fail.** As 7a, but:
  - EX-5's interval coefficient is 1.25e-5 1/K, against the material's 1.2e-5:
    `MBF-SI-EXPANSION-NOT-CONSTANT`. Point-free does not mean constant α.
  - EX-B's ν is 0.29999999999, which prints as 0.3: `MBF-SI-REFERENCE-STATE-NOT-PRINTABLE`,
    naming ν.
  - EX-5 is `free_length_state` under a constant secant law on EX-B whose datum is 20 degC
    while T_ref is 25 degC: `MBF-SI-EXPANSION-DATUM-UNSUPPORTED`.

**8. A forged classification (the redundant-authored-point attack).**

- **The source.** EX-A plus an authored point `point:EX-A-175` at 175 °C with 194 GPa, which
  lies on the line between its neighbours, and `pipe:EX-1` interpolating at 175 degC. The
  selector takes the point itself. The writer writes four point rows and no derived row, and
  the genuine sidecar lists four points.
- **The forgery.** A sidecar that omits `point:EX-A-175` from `points`, records the 175 row as
  derived with bracket `point:EX-A-150` and `point:EX-A-250` and members [`pipe:EX-1`], and
  recomputes its self-digest. The MBF bytes are unchanged, so `mbf_sha256` still binds.
- **Why consistency does not catch it.** Conditions 2 to 7 of §20.4.4 hold: the restored
  three-point model selects exactly 194000000000.0 Pa at 175 degC, and it regenerates the same
  MBF and the same forged sidecar. Stripping would delete an authored point.
- **8a, no designated source.** Refused, `MBF-IMPORT-SOURCE-UNASSOCIATED`: the caller designated
  no source. Nothing is restored and no row is stripped.
- **8b, the genuine source designated.** The user designates the four-point source. Check 1
  passes only if the forger kept its digest; check 2 then finds S's own sidecar listing
  `point:EX-A-175` and classifying the 175 row as a point: `MBF-IMPORT-SOURCE-MISMATCH`, naming
  the row and both classifications. If the forger changed the digest, check 1 refuses the
  designated source, `MBF-IMPORT-SOURCE-MISMATCH`. A changed digest is not looked up, so it
  cannot reach some other stored state.
- **8c, the reverse relabelling.** The genuine example 1 sidecar with its 175 derived row
  relabelled as an authored point (a new point id added to `points`), self-digest recomputed.
  No derived row is marked, but the source is still required. With no designated source:
  `MBF-IMPORT-SOURCE-UNASSOCIATED`. With the example 1 model designated:
  `MBF-IMPORT-SOURCE-MISMATCH` at the 175 row, derived in S and a point in the sidecar. It is
  never imported with an added point.
- **8d, the two-saved-state substitution, refused under designation.** One project holds state
  A, EX-A's three points with `pipe:EX-1` interpolating at 175 degC (example 1), and state B,
  example 8's source with `point:EX-A-175`. Both write the same MBF bytes. The user exports B.
  The attacker replaces B's sidecar with A's, which the writer forms deterministically from A
  whether or not A was ever exported; it is genuine in every field and its
  `source_model_sha256` is A's real digest. A lookup by that digest would find A, and every
  check against A would pass while B's authored point is stripped. The import does no lookup.
  The user, meaning to restore B, designates B: check 1 refuses, since A's digest is not B's,
  `MBF-IMPORT-SOURCE-MISMATCH`, and nothing is restored or stripped. With no designation:
  `MBF-IMPORT-SOURCE-UNASSOCIATED`. If the user designates A instead, the import restores A
  exactly and its review names A: that is the user's choice of source, not a selection by the
  sidecar (§20.4.5, *What designation establishes*). Under the proposed ledger, the same
  substitution with both A and B exported is the re-review's counterexample, an open
  requirement of that proposal (§20.4.5, **Proposed, not adopted, unimplemented**, item 2).

**9. A consistent edit to a point-free export (Open 4's counterexample).**

- **The source.** Example 7a: point-free `material:EX-B`, and `pipe:EX-5`
  `constant_alpha_interval` {1.2e-5 1/K, 150 K} with `explicit_base_properties` whose
  `applicability_reference` is `REF-EX-B-1`. The MBF has OPTEMP `25`, T1 `175` and three
  identical MATERIAL rows at 200000 N/mm² and α 1.2e-05.
- **The edit.** A sidecar in which EX-5 is `free_length_state` under a new law `law:EX-B-F`
  (constant `engineering_secant`, α 1.2e-5 1/K, datum 25 degC), with `operating_temperature`
  175 degC authored; the `applicability_reference` reads `REF-EX-B-2`; and the configuration's
  id and provenance are changed. The self-digest is recomputed, and the MBF bytes are unchanged.
- **Why consistency does not catch it.** The restored model has a law the source never had, a
  different thermal-state kind and a different applicability reference. Writing it again gives
  the same MBF (T1 is 175 either way) and regenerates the edited sidecar exactly. Its strain,
  1.2e-5·(175 − 25)/(1 + 1.2e-5·0), and the source's 1.2e-5·150 are both 1.8e-3 as rationals
  and agree within §20.4.4's bound, so no numerical check refuses it. A format-4 import restores
  none of this metadata.
- **9a, no designated source.** `MBF-IMPORT-SOURCE-UNASSOCIATED`. Nothing is restored. The MBF
  alone imports unbound as an ordinary model, the group listed as lost.
- **9b, the 7a model designated.** Check 1 passes if the editor kept the digest; check 2 finds
  S's own sidecar recording EX-5 as `constant_alpha_interval`: `MBF-IMPORT-SOURCE-MISMATCH`,
  naming the first differing field.

**10. An MBF-only value changed, with both digests recomputed (the circular source proof).**

- **The source.** Example 1, with every pipe on one section of outside diameter 168.3 mm and
  wall 7.11 mm (invented). In 10b, the source also has a spring support at EX-2's far node, of
  linear stiffness 1750 N/mm in one translational direction (invented).
- **The edit.**
  - 10a: the MBF's PIPE record prints the wall as 7.5 mm.
  - 10b: the spring's stiffness is printed as 1800 N/mm.

  In both, the MBF's SHA-256 is recomputed and written as `mbf_sha256`, and the self-digest is
  recomputed. Nothing else in the sidecar changes, because no map records a wall or a stiffness:
  format 4's `section_map` records only the section's id (§15).
- **Under a check that forms S's sidecar for the presented digest.** The bound route holds,
  because the recorded digest is the presented file's. S's sidecar, formed for that digest,
  equals the presented sidecar in every field, and S's MATERIAL rows equal the MBF's. The
  regeneration identity reproduces the sidecar from the restored model, and stripping
  conditions 2 to 7 hold. The import would restore a 7.5 mm wall (10a) or a 1800 N/mm spring
  (10b), which S does not have.
- **Under check 2.** The importer renders S with the recorded export inputs. Its PIPE record
  prints 7.11 (10a), and its spring record prints 1750 (10b). The rendered digest therefore
  differs from the presented MBF's: `MBF-IMPORT-SOURCE-MISMATCH` at check 2 (a). It names the
  PIPE record's wall field (10a) or the spring record's stiffness field (10b) as the first
  difference. Nothing is restored or stripped. With no designation, the import is
  `MBF-IMPORT-SOURCE-UNASSOCIATED`.

#### 20.4.7 Planned tests (both twins; generated vectors only)

No model document is written by hand. A generator `reference_state_models()` is added beside
`coverage_inputs()` in `tools/fixtures/caepipe_mbf_writer_vectors.py`, and its import
counterpart in `caepipe_mbf_import_vectors.py`. It builds every model from parameters with the
existing builders. Its parameter set:

- thermal kind × selection kind;
- interpolated and exact-point T1;
- a point-free second material;
- °C, °F, K and mixed units;
- each refusal's minimal trigger.

The generated material tables are not RS-M1's (D-2: the rule is general). The vectors are
harvested into `writer_vectors.json` and `import_vectors.json` and replayed byte for byte by
the Python tests and the Rust engine (`core/handoff/caepipe_mbf_engine/tests/writer_parity.rs`,
`parity.rs`). The existing check that every writer code is emitted by a vector covers the new
codes.

1. **Admitted vectors** (§20.4.6 1, 3b, 5a, 5c, 5d, 5e, 6b, 6c, 7a, and a point-free mix):
   written; MBF records as stated; format-5 sidecar; bound import `imported`, with the exported
   model designated as the source in every vector, the point-free mix included; the
   regeneration identity holds; exact rational equality of every §20.4.4 input; ids and the
   authored point order equal.
2. **Refusal vectors.** One per §20.4.3 code, each naming its entity (2, 3a, 3c, 4a, 4b, 5b, 6a,
   6d, 6e, 7b, 7c, 7d, 8, 9 and 10 among them).
2a. **Derived rows (option (a)), each with the selected E checked bit for bit against d × 10⁶ Pa
   on the exported and the restored model:**
   - endpoints: T1 at the lowest and highest point temperatures, which take the points and
     write no derived row (6f);
   - interior: an admitted fraction (EX-1) and a refused one (6a);
   - a T1 whose exact rational interpolation lies on the printed grid while the solver's
     binary64 E does not: 184887999999.99997 Pa against d = 184888 (6d), refused;
   - units: °F and K spellings that select an E on the grid (5a, 5c), admitted; the same
     temperature in degC and degF selecting E one ulp apart, both off the grid (6e), each
     refused by its own id; a psi modulus that does not print exactly (7c), refused;
   - a duplicate T1 across members, in one unit and in two units (6b), with a vector where the
     spellings select E one ulp apart, so that the differing member is refused by id;
   - a T1 colliding with an authored row (6c);
   - a material shared by several members at several T1, so that one table carries several
     derived rows in MBF order;
   - the round trip: derived rows stripped only under §20.4.4's seven conditions, with the
     source designated by the caller, points restored in authored order, and the regenerated
     `derivation` equal; the same export imported with no designated source, refused
     `MBF-IMPORT-SOURCE-UNASSOCIATED`; an export with temperature-point materials but no
     derived row, refused the same way without a source; and a point-free export (7a), refused
     the same way without a source.
   No vector's table, temperature or tolerance is chosen to fit RS-M1.
2b. **Canonical-decimal round trip (§20.4.2 g).** A vector per retained value kind (temperature,
   ΔT, point E, point-free E, ν, α) fails each of the two checks once. ΔT includes a pair of
   10-digit printed temperatures whose difference has more than 15 significant digits, so its
   restored canonical decimal differs and the export is refused by name.
3. **Ordinary route unchanged.** Every existing vector replays byte-identical (text, format-4
   sidecar, `to_dict()`), and no model outside Route 1 gets format 5 or a `reference_state`
   block.
4. **Unbound.** Unbound import of a Route 1 file is `MBF-IMPORT-MATERIAL-TEMPERATURE-DEPENDENT`.
   With single-E materials it is an ordinary import plus the listed loss. A format-5 rebind is
   `MBF-IMPORT-REFERENCE-STATE-UNBOUND`.
5. **Mutation.** Each MBF-determined Route 1 field ×1.01 is refused. An edited derived-row E,
   with both digests recomputed, is `MBF-IMPORT-DERIVED-ROW-UNPROVED` or
   `MBF-IMPORT-SIDECAR-NOT-REPRODUCED`. A sidecar copy that differs from the MBF value it copies
   is `MBF-IMPORT-SIDECAR-CONFLICT`. A changed modulus field is
   `MBF-IMPORT-COLD-MODULUS-UNSUPPORTED`. A designated source whose canonical SHA-256 differs
   from `source_model_sha256` is `MBF-IMPORT-SOURCE-MISMATCH`, and so is one whose own sidecar
   differs from the presented one in any single field (a point id, the point order, a row's
   classification, a provenance text, a checked value), each mutated once. Forged
   classifications, each with the self-digest recomputed and the MBF bytes unchanged:
   - **the redundant-authored-point attack** (§20.4.6, example 8): an authored point at
     175 °C whose row the sidecar calls derived. With no designated source it is
     `MBF-IMPORT-SOURCE-UNASSOCIATED` (8a); with the genuine source designated it is
     `MBF-IMPORT-SOURCE-MISMATCH`, naming the row and both classifications (8b); with a
     `source_model_sha256` that names the forged three-point model and the genuine source
     designated, `MBF-IMPORT-SOURCE-MISMATCH` at check 1. The test asserts that no authored point
     is deleted in any variant;
   - **a forged-classification variant:** on EX-A alone, an authored interior point
     (`point:EX-A-150`, with a member interpolating at 150 degC) recorded as derived with bracket
     `point:EX-A-025` and `point:EX-A-250`. `MBF-IMPORT-SOURCE-UNASSOCIATED` without a source and
     `MBF-IMPORT-SOURCE-MISMATCH` with it. Here consistency would catch it as well
     (`MBF-IMPORT-DERIVED-ROW-UNPROVED`): the restored two-point table selects
     195111111111.1111 Pa at 150 degC, not 196000 × 10⁶ (condition 6). It is the contrast to
     example 8, which only the source check refuses;
   - **the reverse relabelling** (8c): `MBF-IMPORT-SOURCE-MISMATCH` with the source designated
     and `MBF-IMPORT-SOURCE-UNASSOCIATED` without one; the test asserts that no model with an
     added point is returned;
   - **the two-saved-state substitution** (8d): A's genuine sidecar beside the MBF B's export
     wrote. With B designated, refused at check 1, `MBF-IMPORT-SOURCE-MISMATCH`; with no
     designation, `MBF-IMPORT-SOURCE-UNASSOCIATED`; with A designated, A restored exactly and
     named as the source. The test also asserts that the importer takes no source it was not
     given (it has no lookup by `source_model_sha256` to call);
   - **a consistent edit to a point-free export** (9): the thermal kind with its new law, the
     `applicability_reference` text, and the configuration's id and provenance, each edited
     once with the self-digest recomputed. Each is `MBF-IMPORT-SOURCE-MISMATCH` with the 7a
     model designated and `MBF-IMPORT-SOURCE-UNASSOCIATED` without one; the test asserts that
     no edited model is returned;
   - **an MBF-only value** (10): a section's outside diameter and wall, and a spring's
     stiffness, each changed once in the MBF, with `mbf_sha256` and the self-digest
     recomputed. Each is `MBF-IMPORT-SOURCE-MISMATCH` at check 2 (a) with the source designated,
     naming the first differing record and field, and `MBF-IMPORT-SOURCE-UNASSOCIATED` without
     one. The test also asserts that S's sidecar formed for the *presented* digest
     (`regenerate_caepipe_sidecar`'s caller-digest form) equals the presented sidecar, so that
     the vector shows the gap check 2 (a) closes. It asserts that no mutated value is returned.
5a. **Export inputs (§20.4.5, Export inputs outside the source document):**
   - admitted vectors re-render to identical bytes with a non-default title (one that the writer
     sanitizes, and one longer than 70 characters) and with each piping-code kind (none; an
     ASME B31 code off §20.4.9's proposed list, such as `B311`, with OPCODE; another documented
     code off that list). The null-code and off-list vectors stay conditional on G-1 (b), (c)
     and (e);
   - each token on §20.4.9's proposed list, given as the piping-code argument for an otherwise
     admitted vector: `MBF-SI-ROUTE1-PIPING-CODE-IGNORES-MODULUS-FLAG`, naming the token; the
     same argument on the ordinary route writes today's bytes;
   - `export_inputs.title` or `piping_code` edited, with the digests recomputed and the MBF
     unchanged: `MBF-IMPORT-SOURCE-MISMATCH` at check 2 (a);
   - the same edit applied to the MBF's title or OPTIONS line (and OPCODE record, when the
     code changes between B31 and another) as well: admitted. The engineering values and
     source-bound entity identities restored are exactly S's; the edited title becomes the
     project name, the token and any OPCODE record are kept as delegated target configuration,
     and the import-generated project id and digest description follow the new MBF digest;
   - a `render_profile` other than the importer's: `MBF-IMPORT-RENDER-PROFILE-UNSUPPORTED`,
     before any rendering;
   - deterministic re-render equality: each admitted vector rendered twice, and by the Python
     and the Rust writer, gives identical bytes;
   - a pinned digest of the Route 1 writer vectors' bytes, keyed by `render_profile`, fails
     when those bytes change and the name does not, so a rendering change made without the
     required profile update is detected (the profile check alone catches only a declared
     mismatch);
   - the source proof's rendering reads no store, and runs no reimport preflight.
6. **Writer argument.** On Route 1, an omitted reference takes the configuration's, an equal
   explicit one is accepted, and a different one is `MBF-SI-REFERENCE-TEMPERATURE-CONFLICT`. An
   accepted equal reference, given in degC or in degF, prints the same OPTEMP field and the same
   bytes as an omitted one. On the ordinary route, the omitted reference is still 20 °C.
7. **Solver agreement (engine).** For each admitted vector, the solver's per-member evidence
   (`resolve.rs:976-1000`) is compared on the original and on the imported model:
   - `selected_E_pa`, `selected_nu` and `derived_G_pa` are equal bit for bit, and `selected_E_pa`
     equals the MBF row's d × 10⁶ Pa exactly. This also checks the writer's reproduction of the
     selection against the solver;
   - `thermal_strain` on each side is within §20.4.4's bound of the exact s* from the MBF values.
     It is not required to be bit-equal; 5e is the vector where it is not.

   This runs through the existing runner binary (`OPENPIPESTRESS_RUNNER_BIN`). No new solver code
   is needed.
8. **Reference system.** Exporting `fixtures/reference_system/reference_system.json` is refused,
   and the named set includes `MBF-SI-MODULUS-SELECTION-MISMATCH` at `pipe:RS-P180` and
   `MBF-SI-BOUNDARY-MOTION-UNSUPPORTED` at `load:RS-SAM` (§20.4.8). The fixture is read, not
   edited.
8a. **Designation and no store (desktop, with the import and export slices).** Rust tests:
   - a designated saved state passes the same two checks as a supplied document, and a saved
     state whose stored text no longer matches its hash is refused by its read
     (`read_state_for_in_store_with`) before either check;
   - the state list offered for designation is the same, in the same order, whatever sidecar is
     presented;
   - a Route 1 export writes its two files and nothing to the store, and its complete review
     carries §20.4.5's notice.

   There are no export-ledger tests. The ledger is proposed, not adopted (§20.4.5); a separate
   design would carry its own.
9. **Compatibility.** Before format 5 lands, a test pins today's importer refusing a format-5
   sidecar with `MBF-IMPORT-SIDECAR-INVALID` at `sidecar_format`, bound and unbound. After it
   lands: formats 2, 3 and 4 replay unchanged, and the mismatched pairs of §20.4.5 are refused
   by name.

#### 20.4.8 The reference system

Route 1 does not by itself export the reference system. Its records are:

- **RS-P180.** In RS-OPE, RS-P180 is `unchanged_reference` and selects
  `temperature_interpolation` at 210 °C, while its T1 is T_ref = 20 °C. While it is an exported
  member, it is refused by name (`MBF-SI-MODULUS-SELECTION-MISMATCH` at `pipe:RS-P180`). The
  fixture stays as authored. RS-P180 is the span the expansion joint
  RS-EJ1 replaces (DEL-09-01 §5.1). It leaves the export only when an expansion-joint tranche
  (RT-K4, refused today) proves that the joint's exported representation removes the replaced
  span from the MBF, or maps it with its own per-member equivalence. Joint support is not
  assumed to do either. DEL-09-01's document records the mismatch in an MBF consumer-coverage
  note below its §10 ledger and in §11.1; its fixture is unchanged.
- **RS-OPE.** RS-OPE is an operating case: W + P + T, with pressure regions. Under (b) it
  stays refused (`MBF-SI-PRESSURE-CASE-UNSUPPORTED`). Exporting it needs the operating W+P+T
  mapping, which Agent 0 approved as a separate design-first item toward criterion 6
  (2026-11-02). Route 1 does not provide it and does not design it. Its 25 other members are
  otherwise inside (c): `free_length_state` with a constant secant law whose datum equals T_inst
  (20 °C), and selection at their operating temperature (210 °C). Their selected E at 210 °C,
  between RS-M1's 189 GPa at 200 °C and 182 GPa at 300 °C, is 188300000000.0 Pa in the solver's
  binary64 arithmetic. That is exactly 188300 N/mm², so a derived row at 210 meets option (a).
  This is an observed outcome; nothing in the rule is chosen to produce it.
- **RS-SUS.** RS-SUS beside RS-M1's varying modulus is `MBF-SI-SUSTAINED-MODULUS-UNPROVED`
  until G-2. Its selections are at 20 °C = T_ref.
- **RS-SAM.** RS-SAM's `boundary_motion` on anchor RS-C is a separate coverage dependency
  (`MBF-SI-BOUNDARY-MOTION-UNSUPPORTED`). It is also a third load case, beyond the
  one-sustained-and-one-thermal mapping. The thermal slice does not admit it.

#### 20.4.9 Gaps (no admitted source states these; none is filled)

- **G-1. The hot-modulus field (partly admitted, 2026-11-02).** The admitted OPTEMP record
  lists a cold-modulus flag (§6; the importer reads OPTEMP fields 2, 4 and 5 as `delegated`,
  `importer.py:1769-1792`).
  - *Source and how it was read.* `CAEPIPE-IMPORT-MBF`, *Format of .MBF*, the OPTEMP
    description, at `https://docs.sstcae.com/users_manual/import_mbf.htm`, accessed 2026-11-02,
    within that row's registered boundary. The CAEPIPE lane read the page twice through WebFetch,
    and the reads agreed. WebFetch returned a model-generated digest, a secondary representation
    rather than the page's bytes, so on its own it gives limited support. The archived fallback
    was unreachable and was not retried. The same day this lane read the OPTEMP and MATERIAL
    descriptions, and nothing else on the page, from the raw HTML of the registered URL, fetched
    read-only with `curl` into a scratch directory that was then deleted. The phrases quoted in
    (a), (b) and (d) are taken from that raw text, not from the digest. Everything else is
    paraphrased.
  - *(a) Which field: admitted.* OPTEMP gives, in order, the reference temperature, the number of
    thermal cycles, the number of thermal loads, the solve-thermal flag and the cold-modulus flag
    ("Use Cold Modulus"). Fields 2 to 5 are optional. The flag is field 5, which matches the
    importer's delegated fields 2, 4 and 5.
  - *(b) Which value selects the hot modulus: partly answered.* The page codes the flag
    "0 = Off, 1 = ON". It does not say that 0 selects each element's modulus at its operating
    temperature T1 (the hot modulus). That reading is an inference and is not admitted.
  - *(c) An omitted field: open.* The page gives no default for an omitted OPTEMP field. Today
    the writer writes field 1 only.
  - *(d) Codes for which the flag is ignored: a new constraint.* The same field description says
    the flag is ignored for six codes: ASME B31.3, B31.4, B31.5, B31.8 (printed "31.8"; read here
    as B31.8), B31.12 and EN 13480. It does not say which modulus those codes use. An ignored
    flag could mean another selection policy, and none is inferred. Neither §6 nor this
    subsection recorded this before 2026-11-02.
  - *(e) A null piping code: open, and separate from (c).* With a null code argument the writer
    writes no code token, and every desktop export records null (`export_settings.rs:7-13`,
    `:96-103`; §20.4.5). No admitted source says which modulus applies to a file that names no
    code, or whether CAEPIPE then applies a default code. A null code is not assumed to select a
    default code, and an answer to (c) alone does not settle (e).
  - *What waits.* Route 1's implementation waits on G-1 (b), (c) and (e), because every Route 1
    equivalence uses the hot modulus, and on G-4 below. The questions are drafted under
    CQ-17-01-016 in the DEL-17-01 dossier, as drafts for the owner, not sent. The meaning of the
    hot/cold option itself is recorded from `CAEPIPE-ANALYSIS`, whose current boundary does not
    cover it (§20.4, **Sources**). The route for that is the G-2 proposal (DEL-17-01
    `Source_Basis_Register.md`, **Proposed boundary change**).
  - *Piping codes that ignore the flag: proposed non-admission, candidate Route 1 only.* This is
    proposed for numerical and contract review and Agent 0's decision, and is not adopted. It is
    scoped to the candidate Route 1 design. Existing routes and their output are unchanged,
    nothing is implemented, and no release limitation is created.
    - **Rule.** A Route 1 export whose piping-code argument is a token on the list below is
      refused as `MBF-SI-ROUTE1-PIPING-CODE-IGNORES-MODULUS-FLAG` (§20.4.3), naming the token.
    - **Rationale.** For these codes, the mapped modulus selection (E at each member's T1) is
      not established: the page says the flag is ignored and does not say what applies instead.
    - **Exact list and basis.** The tokens are drawn from the writer's and importer's documented
      OPTIONS tokens (`PIPING_CODE_OPTIONS`, `importer.py:145-151`). They are `B313`, `B314`,
      `B315`, `B318`, `B3112`, `B3112-PL` and `EUROPEAN`. The first five are matched to B31.3,
      B31.4, B31.5, B31.8 and B31.12 by their names. `B3112-PL` is included because its name
      also denotes B31.12. `EUROPEAN` is included conservatively: no reviewed source here maps a
      token to EN 13480, and it is the only listed token whose name could denote it (`EN13941`
      names another standard by number). This lane read only the OPTEMP and MATERIAL
      descriptions, so it did not check this name-to-token correspondence against the OPTIONS
      description. A reviewed mapping replaces the list.
    - **Not covered.** A token off the list, such as `B311`, still waits on (b) and (c). A null
      code is (e).
- **G-2. The sustained case's modulus.** `CAEPIPE-ANALYSIS`, as recorded, states the modulus for
  operating case 1 and the reference temperature for the cold option. It does not state the
  modulus of the sustained case under the hot option. So it also does not state whether the
  expansion case (operating minus sustained) equals the T1 response at E(T1) when sustained
  loads exist. Until it is answered, a sustained case is admitted only beside single-E
  materials. The question can join CQ-17-01-016's draft. The register change that would admit
  the statements G-2 relies on is proposed, not adopted (DEL-17-01 `Source_Basis_Register.md`,
  **Proposed boundary change**). Until it is adopted, they stay outside the boundary.
- **G-3. The strain formula.** The admitted pages give no strain formula or base of the mean
  (O-7). Route 1 uses only the constant-α reading that §7.1 and §9.2 already apply to today's
  export. It adds no claim, and it closes when CQ-17-01-016 does.
- **G-4. The target's use of the supplied T1 row (CQ-17-01-016 stays open).** Route 1 writes a
  row at every member's T1, so that no interpolation rule is needed *if* the target, under the
  hot-modulus option, uses the E of a row whose temperature equals the element's T1 exactly. It
  must do so with no other interpolation, smoothing or modulus convention. That the hot modulus
  is E at T1 is recorded from `CAEPIPE-ANALYSIS`, outside that row's current boundary (§20.4,
  **Sources**). The admitted MATERIAL description gives each temperature row's temperature, E
  and α, with units only. Re-read on 2026-11-02 (G-1, *Source and how it was read*), it says
  nothing about row selection, interpolation or the hot modulus. No admitted page says that the
  target takes the supplied row's value unchanged. Until admitted format
  semantics prove it, Route 1's varying-modulus equivalence is conditional, its implementation
  waits on G-4 as well as G-1, and CQ-17-01-016 stays open. Only after that proof may
  CQ-17-01-016 be marked inapplicable to this precisely bounded write route, and only to it. It
  still governs unbound imports and any interpolation between rows.
  - *G-4 stays open (Agent 0, 2026-11-02).* The mathematical argument above, that a row at T1
    makes interpolation irrelevant, is not sufficient without selected target semantic
    evidence.
  - *Source boundary.* Any consultation for G-1 and G-4 stays strictly within each registered
    row's current boundary in DEL-17-01 `Source_Basis_Register.md`. Being on a registered page
    does not admit its other subsections. Today `CAEPIPE-MATERIAL` is registered for
    material-property semantics only, `CAEPIPE-ANALYSIS` for the number of thermal loads and
    gravitational acceleration and direction only, and `CAEPIPE-IMPORT-MBF` for its listed
    sections and record descriptions.
  - *Where the boundary is silent.* Target row-selection or point-value behaviour that a row's
    current boundary does not explicitly support becomes a dossier question (beside G-1, with
    CQ-17-01-016) or a source-boundary proposal before broader use. The CAEPIPE lane does that
    work; this Design fills nothing.

#### 20.4.10 For Agent 0

- **Derived rows: option (a), exact admission, is Agent 0's decision (2026-11-02).** Derived
  rows remain conditional on G-4. The rule uses the solver's own selected binary64 E (§20.4.2 e),
  computed for the examples with a reproduction of the solver's arithmetic and checked against
  the solver by §20.4.7 item 7.
  - *(a) Exact (chosen).* A row is admitted only when d × 10⁶ Pa equals the selected E exactly,
    on the exported and the restored model; otherwise `MBF-SI-MODULUS-NOT-EXACT` by member id.
    - Admits: EX-1, EX-2, EX-3 (3b), 5a, 5c, 5d and 5e, 6b, 6c, 6f and 7a.
    - Refuses 6a (201333333333.33334 Pa against 201333.3333 N/mm²), 6d (184887999999.99997 Pa
      against 184888), 6e and 7c.
    - RS-M1 at 210 °C: E is 188300000000.0 Pa, so the derived row at 210 meets (a) for the 25
      `free_length_state` members as far as E goes. They stay refused with RS-OPE as an
      operating case, and RS-P180 stays refused by `MBF-SI-MODULUS-SELECTION-MISMATCH` (T1 20,
      selection 210).
  - *(b) Print precision (rejected).* It would have compared a derived row's E in §18.2's
    printed-directly class, |E − d × 10⁶| ≤ (δ(d) + 4ε|d|) × 10⁶ Pa, and admitted 6a. Agent 0
    rejected it: the target would solve with an E up to 5e-10 relative from SWBPIPE's, the
    per-member rule would no longer state E exactly, and §18.2's precision would be relaxed for
    interior members. No such relaxation applies.
  - *Sample evidence.* The random-sample admission rates given at `ce093a4c7` were sample
    evidence only, not a bound or a justification. They are dropped.
- **The undated exception** follows decision 3 (§20.4.2 e), with counterexamples 7b to 7d.
- **The operating case** (W+P+T) is recorded as Agent 0's separate design-first item. Route 1
  does not include it.
- **G-1, G-2 and G-4** go to the CAEPIPE lane and the dossier, within each registered row's
  current boundary (§20.4.9). G-4 stays open, and so does CQ-17-01-016. G-1 (a) is admitted
  (2026-11-02); (b), (c) and (e) stay open, and their draft questions are in CQ-17-01-016,
  not sent.
- **For Agent 0's decision: piping codes that ignore the flag (proposed).** The proposed
  candidate-Route-1 non-admission `MBF-SI-ROUTE1-PIPING-CODE-IGNORES-MODULUS-FLAG` and its
  exact token list (§20.4.9, G-1 (d)) go to numerical and contract review. The list's
  `B3112-PL` and `EUROPEAN` entries rest on token names, not on a reviewed mapping.
- **For Agent 0's adoption: the `CAEPIPE-ANALYSIS` boundary (proposed).** The exact widened text
  is in DEL-17-01 `Source_Basis_Register.md`, **Proposed boundary change**. It is named by
  function because a headings-only read could not place it. It awaits independent IP review.
- **`CAEPIPE-ANALYSIS`'s boundary (Agent 0, 2026-11-02; paraphrase).** O-7 (ruled 2026-10-27)
  and §20.4 record the hot/cold modulus option and the expansion case (operating minus
  sustained) from `CAEPIPE-ANALYSIS`, whose current boundary is the number of thermal loads and
  gravitational acceleration and direction only. Agent 0 folded the conflict into the planned
  G-2 register-change PR. §20.4 stays blocked on that evidence until the exact expansion passes
  independent IP review and the applicable adoption authority; historical use does not
  establish that the statements were within the registered boundary. This Design does not
  widen the boundary.
- **A verified source for every bound import with points (Agent 0, 2026-11-02; paraphrase).**
  Option (a) is chosen and applied (§20.4.4, **Source proof**; §20.4.5): the relabel-as-point
  residual is withdrawn, and such imports are refused wherever no source is designated
  (example 8c).
- **Explicit designation is the adopted route (Agent 0, 2026-11-02; paraphrase).** The user
  designates the source before the hash and content checks; the digest only verifies it, and
  there is no lookup or fallback (§20.4.5, **The designated source**; examples 8a to 8d).
- **The export ledger is not adopted (Agent 0, 2026-11-02; paraphrase).** §20.4.5 records it as
  proposed, not adopted and unimplemented, with the open requirements a separate design must
  establish, the re-review's counterexample among them. This Design grants no schema,
  persistence or retention authority, and no retention is decided (Open 1). Source-free exports
  carry the export review's notice (Open 2).
- **Point-free materials (Open 4; confirmed by Agent 0, 2026-11-02; paraphrase in O-7).** §20.4.5 (**Point-free
  materials**) and example 9 show that a consistent sidecar edit can change a point-free
  export's thermal, reference and selection metadata, which no format-4 import restores, while
  the regeneration identity and every numerical check pass. The rule chosen: every bound
  format-5 import requires designation, whether or not any material has points. Refusing
  point-free bound format-5 imports outright was the alternative. Format 2 to 4 imports keep
  their adopted consistency-only assurance, unchanged.
- **The source proof renders the source independently (independent review at `147d71586`).**
  Check 2 now renders the designated source itself and compares MBF digests before comparing
  the sidecar formed for the rendered digest (§20.4.4; example 10). The earlier form injected the
  presented digest, so it could not exclude a changed MBF-only value. This adds
  `derivation.render_profile`, `derivation.export_inputs` (title and piping code) and the code
  `MBF-IMPORT-RENDER-PROFILE-UNSUPPORTED` to format 5, which Agent 0 confirmed (2026-11-02;
  paraphrase in O-7). The writer-build identifier is still to be implemented; a profile check
  catches only a declared mismatch, so deterministic re-render equality and the unbumped-change
  tests are required (§20.4.5, **Writer-build drift**; §20.4.7 item 5a).
- **Paraphrase.** Every ruling in O-7 is a paraphrase of Agent 0, decision 3 included (its text
  was normalized from Agent 0's message). D-2's historical attribution (2026-10-31) is a
  paraphrase, unverified against the original messages; Agent 0's confirmation of 2026-11-02
  supplies the current basis for its substance.
- **DEL-09-01 §10.** The RS-P180 ledger row given to this lane is not in the §10 table. That
  table is checked row for row against the fixture generator's `LEDGER` and `authoring.json`
  (`tools/validation/test_reference_system_ledger.py`). A new row there fails that check unless
  fixture files change, and this lane leaves the fixture unchanged. The row's text stands
  verbatim below the table, labelled an "MBF consumer-coverage note" (Agent 0, 2026-11-02): not
  a pending-family generator entry and not a completed admission.
- **Release limitation: proposed, for the implementation PR.** The entry is not added to
  `docs/RELEASE_LIMITATIONS_v1.0.md` here. It is added in the PR that makes the refusal
  standing, as that file's entry form requires. Owner acceptance is pending and is not inferred.

  > **LIM-nnn — MBF exchange of materials whose expansion coefficient varies with temperature,
  > and of cold-modulus files** (proposed)
  >
  > - **Rows:** RT-R7, RT-R8, RT-G3.
  > - **Code:** `MBF-SI-EXPANSION-NOT-CONSTANT` (export); `MBF-IMPORT-MATERIAL-TEMPERATURE-DEPENDENT`
  >   (import of MATERIAL rows that differ in α, bound or unbound);
  >   `MBF-IMPORT-COLD-MODULUS-UNSUPPORTED` (a Route 1 file set to the cold modulus).
  > - **What is refused:** exporting to, or importing from, a CAEPIPE MBF file a material whose
  >   thermal expansion coefficient changes with temperature. Also importing a file whose
  >   analysis option uses the cold (reference-temperature) modulus. A modulus that varies with
  >   temperature, with a constant expansion coefficient, is exchanged under the bounded route of
  >   DEL-17-04 §20.4, once implemented.
  > - **Why:** the public MBF description tabulates a mean coefficient but states no base
  >   temperature for it and no interpolation rule, so a varying coefficient has no proved exact
  >   mapping (CQ-17-01-016, open). SWBPIPE selects E and α together for a case, so a cold-modulus
  >   file, which needs E at the reference temperature with α at the operating temperature, has
  >   no equivalent member.
  > - **What the user does instead:** analyse such materials in SWBPIPE, and exchange the
  >   geometry, supports and loads through MBF with the material entered in each program
  >   separately. Or, where the user's engineering basis supports it, exchange a constant
  >   coefficient. That is the user's decision, and SWBPIPE does not make it. For a cold-modulus
  >   file, re-export from the target with the hot-modulus option if the user's analysis basis
  >   allows, or enter the material in SWBPIPE.
  > - **Basis:** DEL-17-04 `Work/caepipe-mbf-import.md` §7.1, §20.2 (O-7, D-2) and §20.4; made
  >   standing by the Route 1 implementation PR.
  > - **Status:** owner's acceptance pending.

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
  (§5.2); the schema-enum version check (§19); `mbf_sha256` in `id_map.json` (§15); and
  sidecar format 4's `section_map` with PIPE records per library section (§15, both twins).
  Format 4 changes no ScopeOfWork meaning (REQ-016 covers it) and no `deliverable.yaml`
  check or path.
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
