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

The seven rows from `CAEPIPE-LAYOUT-OFFSETS` to `CAEPIPE-FORCE` are registered in DEL-17-01
`Source_Basis_Register.md`. Each has the live URL, an archived fallback where the Internet
Archive holds one (it holds none for `CAEPIPE-MBF-ABSOLUTE`), and a boundary:

- "record syntax only" for the four user's-manual layout pages;
- "case-membership semantics only" for `CAEPIPE-LOAD-CASES` and `CAEPIPE-FORCE`;
- "bend end-node naming and section assignment only" for `CAEPIPE-BEND`.

`CAEPIPE-LOCAL-AXES` was registered later with the boundary "pipe local-axis convention
only", for the straight-pipe orientation of §10.5.

## 3. Where it lives and its interface

- **Module:** `core/handoff/caepipe_mbf/importer.py`, beside the writer and sharing its
  constants (`MBF_VERSION`, `LOAD_CASE_LABELS`, `STANDARD_GRAVITY`). The writer keeps its two
  case labels in one mapping, `LOAD_CASE_LABELS["sustained"]` and
  `LOAD_CASE_LABELS["expansion"]`; this Design's "sustained label" and "expansion label"
  name those entries.
- **Function:**
  `import_caepipe_mbf(data: bytes, *, sidecar: Mapping | None = None, material_provenance: Mapping | None = None, rebind_by_node_number: bool = False, source_label: str) -> MbfImportResult`.
  - Input is bytes, not text, so that the importer owns the decoding (§4.1).
  - `material_provenance` is the affirmative material provenance declaration (§7.4).
  - `rebind_by_node_number` is the explicit request to apply an unbound sidecar (§15).
- **Result:** `status` (`imported` | `refused`), `model` (the document, or `None` when
  refused), `diagnostics`, `loss_report`, `assumptions`, `target_options` (§6), `id_map`
  (§15) and `source` (`mbf_sha256`, MBF version, unit mode, line count).
- **Report document:** `to_dict()` emits `document_kind:
  "swbpipe.caepipe_mbf_import_report"` with `profile_id: "caepipe-mbf-import-1"`. The
  report carries no model values from refused or omitted records, only their location and
  name.
- **Command line:** `python3 -m core.handoff.caepipe_mbf.importer <in.mbf> --out <dir>
  [--sidecar <id_map.json>] [--material-provenance <declaration.json>]
  [--rebind-by-node-number]` writes `model.json`, `import_report.json` and `id_map.json`.
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
  (`MBF-IMPORT-VALUE-OUT-OF-RANGE`).
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
- The vertical-axis option `Z` is refused in tranche 1 (`MBF-IMPORT-OPTION-UNSUPPORTED`).
  The writer only writes Y-vertical. Supporting Z-vertical needs both directions to change
  together; it is listed for tranche 2.
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
| OPTIONS `Z` | Refused (§5.1) |
| OPTIONS unknown token | `MBF-IMPORT-OPTION-UNSUPPORTED` |
| OPTEMP field 1, reference temperature | Read (§9.2). Required whenever any LOADS temperature is read; a missing reference temperature is refused (`MBF-IMPORT-REFERENCE-TEMPERATURE-MISSING`), never taken from a default |
| OPTEMP thermal cycles, solve-thermal flag, cold-modulus flag | `delegated` |
| OPTEMP number of thermal loads | Absent or 1 is accepted. More than 1 is refused (`MBF-IMPORT-THERMAL-LOADS-UNSUPPORTED`; multiple thermal cases are tranche 2). When given, it decides which LOADS slots are active (§9.1) |
| OPCODE, OPPRES | `delegated`. They affect code stresses and pressure options only; pressure is refused anyway (§9.1) |
| OPMISC fields 1–8 (dynamics and hanger options) | `delegated` |
| OPMISC gravitational acceleration and direction (15.10 and later) | Two layouts are read. **Documented** (*Format of .MBF*, 10 fields): field 9 is the acceleration; a non-empty field 10, a direction whose one-field encoding no source states, is refused (`MBF-IMPORT-TBD-17-04-005`). **The export's** (observed format behaviour, 13 fields): field 9 has no public description and is read only as the `0` every export writes, else `MBF-IMPORT-FIELD-UNDOCUMENTED`; field 10 is the acceleration; fields 11–13 are the direction as three components. Only earth gravity is read, by layout (independent review VER-002 r5). **Export layout:** only the setting the export writes for the default environment, acceleration `386.4` with direction (0, −1, 0); the export writes `386.4` in SI files too, so this reading is confined to the 13-field layout. **Documented layout:** the acceleration is in the file's documented unit (m/s² in SI, in/s² in English, *Format of .MBF*, OPMISC) and is read only as standard gravity, 9.80665 m/s² or 9.80665/0.0254 = 386.0886 in/s², printed correctly rounded to its own digits (|p − g₀| ≤ half a unit in p's last digit) with at least three significant digits; so `386.4` in an SI file (39.4 g) is refused. Earth gravity is listed as `delegated` and recorded in `assumptions`: SWBPIPE has no gravity setting and carries weight as explicit primitives along −Y (§7.3), which matches earth gravity (`CAEPIPE-ANALYSIS`: input is given for earth gravity). Any other acceleration or direction scales or turns the weights and is refused (`MBF-IMPORT-GRAVITY-UNSUPPORTED`) |
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
  has `temperature_points`, but mapping them needs two choices the MBF does not state:
  which modulus each case solves with (SWBPIPE's explicit `modulus_basis`), and whether the
  tabulated α is a mean coefficient from a reference temperature. Both are tranche 2 and
  need a ruling.
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
  - **No origin, no import.** A material whose entry has no `origin` needs the caller's
    explicit declaration. That covers a material inserted from a library (`library_ref`
    present), a `private_only`, `public_permissive` or `unknown` status without such a
    statement, and a bare provenance string.
- **Refused** (`MBF-IMPORT-MATERIAL-PROVENANCE-REQUIRED`): a material with no
  declaration; a declaration naming vendor-library or unknown origin; and any material
  §7.1 marks as suspected protected.
- **Recorded.** The imported material's `provenance_record` holds the declared source and
  its disposition: `private_only` for `user_supplied`, and the declared disposition for
  `project_invented`. It also holds `source_location` `sha256:<mbf>` and the G derivation
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
- **Refused:** a non-zero value of any of these, or a soil name
  (`MBF-IMPORT-SECTION-FIELD-UNSUPPORTED`). Insulation and lining weights, corrosion and
  buried pipe are tranche 2.

## 9. Loads (LOADS) and load cases

### 9.1 LOADS records

A LOADS record is name, T1, P1, specific gravity, then optional fields: T2, P2, T3, P3,
additional weight, peak temperature and pressure, T4 to T10 with P4 to P10, and the
environmental-load flags (*Format of .MBF*, LOADS).

| Field | Tranche 1 |
|---|---|
| T1 | Read (§9.2) |
| P1 | Must be 0. Otherwise `MBF-IMPORT-PRESSURE-UNSUPPORTED`; pressure, in bar or psi, is tranche 2 with SWBPIPE pressure regions |
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
- A case with no primitives is not created.
- No `combinations` are created. CAEPIPE forms its own (*Load cases*), and that is listed
  as `delegated`.
- With a bound sidecar (§15), the original case ids are restored from `load_case_map`.

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
  - anchor at `n` → `support:n`; restraint at `n` → `support:n-restraint`;
  - material `M` → `material:M`;
  - primitive loads → `load:<category>:<target id>[:<axis>]`.
- Node labels are the CAEPIPE names. Pipe labels are `a-b`. Support labels are the node's
  `TAG`, else `anchor at n` or `restraint at n`; bend components are labelled `bend n`; load
  cases carry the writer's case labels (§9.3).

### 10.5 Straight-pipe orientation

The MBF carries no element orientation. A straight pipe still needs a `y_reference`, which
the solve requires, so the importer assigns one deterministically:

- **Bound imports** restore it from the writer's sidecar: `element_map[pipe].y_reference`,
  which the writer records for every straight with an authored `y_reference`. Only a finite,
  non-zero vector is restored; anything else takes the documented local y below.
- **Otherwise** the straight takes the local y axis CAEPIPE documents for pipe elements
  (documented: `CAEPIPE-LOCAL-AXES`, technical reference *Local Coordinate System*). With
  global Y vertical, a non-vertical element's local y lies in the plane of its local x and
  global Y, toward +Y, so `y_reference` = Y − (Y·x)x normalized (computed as
  (−x_y·x_x, x_x² + x_z², −x_y·x_z) to keep precision near vertical). A vertical element's
  local y is global −X.
- **Selected interpretation.** The manual gives no angle for "vertical". This profile treats
  a straight as vertical when the horizontal part of its unit direction is at most 1e-6
  (`VERTICAL_SINE`).
- Z-vertical models are refused in tranche 1 (§5.1); the manual's Z-vertical convention
  (local z toward +Z in the plane of local x and Z; vertical elements y = global Y) joins
  the Z-vertical tranche.
- Each assignment is reported as `MBF-IMPORT-ORIENTATION-DEFAULTED` (info, by element).
  A circular section's stiffness does not depend on the orientation; only local-axis results
  (local forces, moments and stresses) depend on the convention.

## 11. Bends

The writer places a curved bend at its tangent intersection (TI) and writes
`T<n>,KI,B<R>` with the factor as comment `FF=<k>`. CAEPIPE generates the near and far end
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
   intermediate nodes `IN1` and `IN2`, `BTHK`, `BMAT`, `BSIF`, the miter code `KM` and
   `WS`, and jacketed bends (`JI`). `BSIF` could later map to SWBPIPE's in-plane and
   out-of-plane SIF user values; that is tranche 2.

## 12. Supports

Support data are comments on the node's row or on a Location row. They follow *Format of
.MBF*, LAYOUT, key letter K code A and the comment list.

| MBF | Tranche 1 |
|---|---|
| `KA`, anchor in global axes (`LA` absent or 0) | `support:n`, `family: anchor`, restraints UX…RZ |
| `FIXD(a,b,c)`, `F(a,b,c)`, `FIXDX[=1]`, `FX`, … | Translational restraints. Each value must be 0 or 1 (`MBF-IMPORT-COMMENT-INVALID`). They go on `support:n-restraint` with family absent (the solver's own inference, as for any family-less support) |
| `FIXR(…)`, `FIXRX[=1]`, … | Rotational restraints, on the same restraint support. SWBPIPE represents them, so they are imported. The writer refuses partial rotational restraints because the target tool did not accept them in an imported MBF, per private notes |
| FIXD and FIXR covering all six DOF | Kept as written. The writer writes such a node as `KA` |
| `KA` with `LA=1` (local axes), `TRAK`, `ROTK`, `FREE` | Refused (`MBF-IMPORT-SUPPORT-UNSUPPORTED`). Flexible anchors are tranche 2: one spring support per DOF, with N·m/deg → N·m/rad by multiplication by 180/π |
| Guide `G` (`GGAP`, `MU`, `STIFF`) | Refused. Tranche 2, rigid and axis-aligned first |
| Limit stop `LS` (`DV`, `STIFF`, `MU`, `AXIAL`, `SHEARY`, `SHEARZ`) | Refused. Tranche 2 |
| Hangers `VS`, `CS`, `US`, `HTYP`, `SR`, `MLV`, `CLD`; snubbers; skewed restraints `K`/`KR`; generic support `KG`, `GR*`; nozzles `NOZZLE` and its fields; `CNOD` | Refused (`MBF-IMPORT-SUPPORT-UNSUPPORTED`, naming the keyword) |
| `TAG=<text>` | Becomes the `label` of the node's support. On a node without a support it is listed as `omitted` |
| `LTAG` (spectrum level) | Empty is accepted (§4.4); non-empty is refused |

With a bound sidecar, the writer's `support_map` restores the original supports, their ids
and DOF sets. That map preserves each split explicitly: the writer may have merged several
supports into one anchor, and the sidecar splits them back. The writer records each
support's `family` in `support_map` (implemented), and bound imports restore it. A sidecar
entry without `family` (an older sidecar) is reported as unrecoverable (§15).

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
- `MBF-IMPORT-SIDECAR-INVALID`: a malformed sidecar, or rebinding requested without one (§15);
- `MBF-IMPORT-VALUE-UNREPRESENTABLE`: a read or derived value outside the binary64 range, or a
  non-zero value that would become 0 (§4.3).

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
- `MBF-IMPORT-SIDECAR-NOT-APPLIED` (info: the hashes differ and no rebinding was requested)
  and `MBF-IMPORT-ORIENTATION-DEFAULTED` (info, §10.5);
- `MBF-IMPORT-LOAD-SLOT-INACTIVE` (info, by record and slot, §9.1).

Further refusal: `MBF-IMPORT-GRAVITY-UNSUPPORTED`, an OPMISC gravity setting other than the
default earth setting (§6).

## 15. Ids and the sidecar map

There is no id carrier inside the MBF (TBD-17-01-003, TBD-17-04-003). Identity therefore
travels in a sidecar.

**Input.** The writer's maps (`node_map`, `element_map`, `support_map`,
`load_case_map`, `material_map`, `section_map`, `load_map`), as `run_pilot.py` already
writes them to `caepipe_in/id_map.json`, with `mbf_sha256` (implemented; older runs hold
it in `mbf_render.json`, and the importer accepts either file pair).

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
- straight-pipe orientation, from `element_map[...].y_reference` (§10.5).

**What a bound sidecar cannot restore.** Many-to-one mappings and identities the sidecar
does not preserve are reported, never guessed. Each is listed in the loss report as
`approximated` with code `MBF-IMPORT-ID-UNRECOVERABLE`, naming the sidecar entries
involved.

- **Primitive loads.** `load_map` records each primitive's case, category, target and
  direction (implemented), but not its magnitude. The writer sums several primitives on one
  target, so a primitive id is restored only when the sidecar names exactly one primitive
  for that (case, category, target, axis) and the import produces exactly one primitive for
  it. Every other primitive gets a §10.4 id and is reported.
- **Support families** when the sidecar entry has no `family` (§12).
- **Bend component nodes** (§11 item 5).
- **Members the writer does not export** (§17's last rows).

**Unbound** (no sidecar, or hashes differ: a CAEPIPE re-export, or an edit). The MBF
governs every value, and ids are formed by §10.4. A sidecar is not applied unless the
caller explicitly asks for `rebind_by_node_number`.

- **What rebinding does.** It applies the sidecar by CAEPIPE name only: nodes by number,
  pipes by from/to names, supports by node, bends by TI number, materials by name.
- **Unconfirmed identity.** Whether CAEPIPE keeps names is open (TBD-17-04-008). Every
  rebound id is therefore marked `rebound_unconfirmed` in the report and the output id map,
  and is never presented as a confirmed identity.
- **Entities and entries without a match.** Entities with no sidecar entry keep §10.4 ids.
  Sidecar entries with no MBF counterpart are listed as `omitted` ("absent from this MBF").
- **Conflicts.** A sidecar entry naming the same SWBPIPE id for two CAEPIPE names, or
  colliding with a formed id, is refused (`MBF-IMPORT-SIDECAR-CONFLICT`).

**Output.** `id_map.json` holds CAEPIPE name → SWBPIPE id for every node (including
`nA`/`nB`), element, support, bend, material and load case. DEL-17-05's CSV parser can
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
- **Absent members.** No combinations, lines, constraints, rule-pack references, model
  settings or pressure contract are written.

### 16.2 Determinism

Identical MBF bytes, sidecar, material-provenance declaration, rebinding option, source label and importer/profile version produce byte-identical outputs. Members are written in the
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
| `OPTIONS HGRA,SI`; Y vertical; X, Y, Z unchanged | SI mode; Y vertical; coordinates unchanged; HGRA is `delegated` |
| Lengths in mm, scaled by powers of ten from m or mm | Lengths kept in `mm`; project length `mm` |
| E in N/mm², from Pa, kPa, MPa or GPa | E in `MPa` |
| ν as given, or E/(2G) − 1 | ν as given; G = E/(2(1+ν)) with basis `homogeneous_isotropic_E_nu_v1` |
| Material without α → 0 | α = 0 explicit |
| Temperature-independent material at three identical temperatures | Identical lines → temperature-independent; differing lines refused |
| Weight per length (−Y) → weight density ρ = \|w\|/(A·g₀) | ρ → w = −ρ·g₀·A along `global_y`, in the mechanical case |
| Weight only along −Y | Weight along −Y only (Z vertical refused) |
| LOADS: T1 = T_ref + ΔT, P1 = 0, SG = 0 | ΔT = T1 − T_ref; P1 and SG must be 0 |
| `OPTEMP` = reference temperature | T_ref |
| Mechanical case → sustained label; thermal and displacement → expansion label (`LOAD_CASE_LABELS`) | `case:mbf-sustained` and `case:mbf-expansion-t1` (ids restored from the sidecar) |
| Combinations not exported | None created |
| Force and moment comments `FOR`, `MOM` per node, summed, without `FLC` | Primitives per non-zero component, in the mechanical case. Without `FLC`: bound imports only; unbound imports refuse it, except where an explicitly retained bounded profile decision admits it |
| Anchor displacement `D`, rotation `ROT` (rad → deg) | Imposed displacement and rotation on the anchor, in the thermal case, in `mm` and `deg` |
| Full six-DOF restraint (one or several supports) → `KA` | Anchor; the sidecar splits it back |
| Translational partial restraint → `FIXD` | Restraint support; family restored from `support_map` in bound imports (§12) |
| Partial rotational restraint refused | `FIXR` imported (wider than the writer) |
| Curved bend → `T<n>,KI,B<R>,…,CFF=<k>` at the TI; ends `nA`/`nB` | Tangent nodes from TI, R and the adjacent legs; bend pipe, y_reference and component reconstructed |
| Straight-pipe `y_reference` not in the MBF; recorded in `element_map` | Restored in bound imports; otherwise the documented local y (§10.5) |
| US-customary model units → exact normalization to SI (`MBF-SI-UNITS-NORMALIZED`, §5.2) | SI import of the normalized values |
| `M`, `P`, `L=` written only when changed | M and P carried until changed (documented). `L=` carried the same way in bound imports; unbound imports refuse it, except where an explicitly retained bounded profile decision admits it |
| Node numbers 10, 20, … in layout order; sidecar maps | Ids from the sidecar, or formed from numbers |
| Unread members (`analysis_status`, `lines`, `unresolved_assumptions`, `constraints`, `equipment_interfaces`, `rule_pack_refs`, …) not exported | Not recoverable from the MBF; the importer writes only §16.1's desktop-profile members |
| CRLF line ends; printable ASCII; 10 significant digits | CRLF or LF; UTF-8 with the listed marks; any precision |

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
   - **Models.** The six pilot cases (`validation/caepipe_correlation/cases/cpc-01…06`)
     and the writer test's model builders, in m and mm projects, each within the
     reversible subset of §18.2. The test file lists which of them are in the
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
- labels, provenance and the members of §17's last rows;
- several primitives on one target, which are compared as their sum;
- zero-valued primitives, which are dropped;
- absent versus zero α, and absent versus zero section fields;
- combinations, which are not exported.

Each exclusion is listed in the test so that it stays explicit.

**Canonical form.** Both models are reduced to this subset, in SI base units:

- node positions;
- per pipe: endpoint pair, oriented only when the sidecar is bound; OD; t; E; ν, the
  writer's, given or derived; G; and α;
- per node: the union of restrained DOFs, and per support when the sidecar is bound;
- per bend: R, k, the bend-plane direction and the component node. The bend-plane
  direction is compared as the writer checks it: the normalized projection of
  `y_reference` perpendicular to the unit chord between the two tangent nodes. For
  example, an invented fixture with `y_reference` = (2, 0, 0) reconstructs as
  (1, 0, −1)/√2. The raw vectors are 45° apart, but the projected directions agree.
- per case kind: summed force and moment vectors per node, summed weight per length per
  pipe, summed ΔT per pipe, and summed anchor displacements and rotations.

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
dossier as CQ-17-01-011 to -015.

| TBD | Question | Effect until closed |
|---|---|---|
| TBD-17-04-005 | Encoding and default of OPMISC's gravitational-direction field (15.10 and later) | **Finding** (observed format behaviour, recorded here; the TBD's status is Agent 0's): CAEPIPE's export writes OPMISC with 13 fields, the direction as three components after the acceleration, and for the default earth environment acceleration `386.4` (in SI files too) with direction (0, −1, 0). The importer reads that default setting only (§6) and refuses any other (`MBF-IMPORT-GRAVITY-UNSUPPORTED`) and a one-field direction (`MBF-IMPORT-TBD-17-04-005`). Still open: the public field list's one-field encoding, the meaning of the export's field 9, and the acceleration's unit in SI files. The writer never writes these fields |
| TBD-17-04-006 | Text encoding, foot and inch mark characters, and numeric precision that CAEPIPE's MBF export writes | The importer reads only strict UTF-8 and the marks the public pages show. Anything else is refused by name. Values are read as written, with no precision assumed; private-regression outcomes record differences, and no tolerance is widened for them |
| TBD-17-04-007 | Whether `L=<load name>` is the documented load reference, carried like M and P, and which load case a `FOR` or `MOM` without `FLC` joins; and whether these are stable across versions | Bound imports apply the writer's conventions. Unbound imports refuse both with `MBF-IMPORT-TBD-17-04-007`. Unbound imports may admit both where an explicitly retained bounded profile decision does so |
| TBD-17-04-008 | Whether CAEPIPE's MBF export keeps node numbers, row order and TI bend rows as imported, and whether it writes offsets or absolute coordinates | An unbound sidecar is applied only on explicit request, and every rebound id is marked unconfirmed (§15). Geometry is read only in its documented forms |
| TBD-17-04-009 | Whether the absolute-coordinate marker `*` precedes or follows the node number in an MBF layout row. The *Node* page and the *DX, DY, DZ* page and MBF examples disagree | The trailing marker shown in MBF examples is a selected interpretation (§10.2). Any other placement is refused by name (`MBF-IMPORT-TBD-17-04-009`) |

### 20.2 Owner choices

- **O-1.** Whether DEC-016's wording ("commercial stress software output parsers") should
  say explicitly that model import is outside it. This Design relies on the 2026-10-26
  ruling.
- **O-2.** English-mode iteration. Agent 0 decided exact normalization to the SI profile
  in the writer (§5.2, implemented). A native English writer mode remains an owner choice.
- **O-3.** The tranche-1 cut (§11, §12, §14): temperature-dependent materials, supports
  beyond anchors and restraints, pressure and Z-vertical are refused.
- **O-4.** How CAEPIPE-exported MBFs are produced for §18.3: by hand by the owner, or by a
  guest automation step.

### 20.3 Next tranches

The first-tranche cut is confirmed as a tranche plan (Agent 0, 2026-10-26). The next
tranche, in priority order, each with its writer counterpart where one is missing:

1. pressure (bar or psi) with SWBPIPE pressure regions, which most real models carry;
2. Z-vertical, a routine axis rotation;
3. rigid, axis-aligned guides and limit stops;
4. flexible anchors and springs;
5. hangers;
6. temperature-dependent materials, after a ruling on modulus basis and mean α.

Later, in proposed order: concentrated weight and uniform load; contents, insulation and
lining; rigid and valve joints; reducers (0.8.0); SIF user values; multiple thermal loads;
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
