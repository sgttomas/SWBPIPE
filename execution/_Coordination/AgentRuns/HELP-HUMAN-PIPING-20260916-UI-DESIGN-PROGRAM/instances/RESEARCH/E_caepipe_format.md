# E — CAEPIPE exchange format: what the public documentation describes

Date: 2026-09-18 (UTC). Agent: RESEARCH-E, TASK child of HELP-HUMAN-PIPING-20260916-UI-DESIGN-PROGRAM.
Brief: [`../../briefs/RESEARCH-E_caepipe_format.md`](../../briefs/RESEARCH-E_caepipe_format.md).
Status: bounded research return from public documentation only. Not an acceptance, not a compatibility claim.

**No compatibility claim is made anywhere in this document.** It records what SST Systems' published
documentation says the format contains. Whether an export written from the SWB tables would be read
correctly by any CAEPIPE build is a matter for later evidence, not for this return.

Sourcing legend, following D's convention:
**[S: n]** stated by the cited public document; **[I]** my inference or comparison across two cited
documents, marked as such; **[PK]** prior knowledge, not from a source read here. **No [PK] marks
appear in this document** — nothing below is asserted from memory of the product. Quotations from the
manual are kept under 15 words each; field names, keywords and code letters are reproduced as the
factual identifiers they are, and the manual's explanatory prose is paraphrased rather than copied.

Method: public web pages only, read over HTTPS without authentication. No software was downloaded,
installed or run; no binary was inspected; no sample file of unknown licence was used.

Relation to D: D recorded that CAEPIPE presents a text Layout window beside an OpenGL graphics
window [D §1, S: 35, 36]. This return does not repeat that. It characterises the *file* formats.

---

## 1. Sources

All CAEPIPE manual pages below are the **CAEPIPE User's Manual**, published by **SST Systems, Inc.
and SST India Pvt. Ltd.** (copyright line "(c) SST Systems, Inc. and SST India Pvt. Ltd., 2026"),
served at `docs.sstcae.com`. The older host path `www.sstusa.com/docs/users_manual/<page>` issues a
301 to `docs.sstcae.com/users_manual/<page>`; search engines still surface the old host, so both
appear in the wild and only the new one is cited here.

Retrieval: every page was first read in this session between **2026-09-18T05:15Z and 05:28Z**, then
re-requested in a verification pass; the time in each row is the verified request, which returned
HTTP 200. The manual is a Dr.Explain-generated site whose table of contents is script-rendered, so
navigation was followed through in-page links rather than a contents tree.

### 1.1 Reached

| # | Page / title | URL | Retrieved (UTC) | What it covers |
|---|---|---|---|---|
| 1 | Import (Main window File menu) | https://docs.sstcae.com/users_manual/import.htm | 2026-09-18T05:28:24Z | The three neutral files CAEPIPE can import |
| 2 | Export data from CAEPIPE | https://docs.sstcae.com/users_manual/export_data_from_caepipe.htm | 2026-09-18T05:28:24Z | The nine documented output paths |
| 3 | File Menu (Layout window) | https://docs.sstcae.com/users_manual/file_menu_2.htm | 2026-09-18T05:28:25Z | Export commands; QA Block; Revision Record |
| 4 | Export to .mbf | https://docs.sstcae.com/users_manual/export_to__mbf.htm | 2026-09-18T05:28:26Z | Names .mbf the neutral format; points to Appendix A |
| 5 | Export to 3D Plant Design | https://docs.sstcae.com/users_manual/export_to_3d_plant_design_1.htm | 2026-09-18T05:28:26Z | PDMS `.mac`, CADMATIC `.3dd`, graphics only |
| 6 | Appendix A — Import / Export | https://docs.sstcae.com/users_manual/appendix_a___import___export.htm | 2026-09-18T05:28:27Z | Appendix index; `EXPORT_INCHES` variable |
| 7 | **IMPORT MBF** | https://docs.sstcae.com/users_manual/import_mbf.htm | 2026-09-18T05:28:28Z | **The .mbf specification: sections, key letters, comment keys** |
| 8 | EXPORT MBF | https://docs.sstcae.com/users_manual/export_mbf.htm | 2026-09-18T05:28:29Z | Export command; current and 6.xx dialects |
| 9 | Example MBF Format | https://docs.sstcae.com/users_manual/example_mbf_format.htm | 2026-09-18T05:28:29Z | A complete worked .mbf; the hanger-type number table |
| 10 | MBF Generation in Excel with Absolute Coordinates | https://docs.sstcae.com/users_manual/mbf_generation_in_excel_with_absolute_coordinates.htm | 2026-09-18T05:28:30Z | MBF as CSV; absolute-coordinate node form |
| 11 | Running CAEPIPE/CAEPIPE 3D+ in Batch Mode | https://docs.sstcae.com/users_manual/running_caepipe_caepipe_3d__in_batch_mode.htm | 2026-09-18T05:28:31Z | Command line, arguments, CSV results |
| 12 | PCF EXPORT | https://docs.sstcae.com/users_manual/pcf_export.htm | 2026-09-18T05:28:31Z | PCF export index |
| 13 | Element types from CAEPIPE (PCF) | https://docs.sstcae.com/users_manual/element_types_from_caepipe.htm | 2026-09-18T05:28:32Z | Element-by-element PCF attribute mapping |
| 14 | Data Types from CAEPIPE (PCF) | https://docs.sstcae.com/users_manual/data_types_from_caepipe.htm | 2026-09-18T05:28:33Z | Support/nozzle PCF attribute mapping |
| 15 | Limitations (PCF) | https://docs.sstcae.com/users_manual/limitations.htm | 2026-09-18T05:28:34Z | Index of what PCF export drops |
| 16 | Element Types (not transferred to PCF) | https://docs.sstcae.com/users_manual/element_types_2.htm | 2026-09-18T05:28:34Z | Seven element types PCF drops |
| 17 | Data Types (not transferred to PCF) | https://docs.sstcae.com/users_manual/data_types_2.htm | 2026-09-18T05:28:35Z | Nine data types PCF drops |
| 18 | Units (PCF) | https://docs.sstcae.com/users_manual/units.htm | 2026-09-18T05:28:36Z | English/SI selection rule for PCF |
| 19 | Vertical Axis (PCF) | https://docs.sstcae.com/users_manual/vertical_axis.htm | 2026-09-18T05:28:37Z | Y-up to Z-up translation on PCF export |
| 20 | IMPORT MATERIAL LIBRARY | https://docs.sstcae.com/users_manual/import_material_library.htm | 2026-09-18T05:28:38Z | The `.mlb` material-library batch format |
| 21 | EXPORT MATERIAL LIBRARY | https://docs.sstcae.com/users_manual/export_material_library.htm | 2026-09-18T05:28:38Z | `.mlb` export, editable in a text editor or Excel |
| 22 | LAYOUT WINDOW | https://docs.sstcae.com/users_manual/layout_window.htm | 2026-09-18T05:28:39Z | The ten Layout-window columns |
| 23 | Node | https://docs.sstcae.com/users_manual/node.htm | 2026-09-18T05:28:39Z | Node numbering rules; generated suffixes; `*` form |
| 24 | Type | https://docs.sstcae.com/users_manual/type.htm | 2026-09-18T05:28:40Z | The element-type column |
| 25 | DX, DY, DZ | https://docs.sstcae.com/users_manual/dx__dy__dz.htm | 2026-09-18T05:28:41Z | Offsets, From-row absolute coordinates, feet-inch entry |
| 26 | Matl (Material) | https://docs.sstcae.com/users_manual/matl__material_.htm | 2026-09-18T05:28:41Z | Material propagation down the rows |
| 27 | Sect (Section) | https://docs.sstcae.com/users_manual/sect__section_.htm | 2026-09-18T05:28:42Z | Section propagation down the rows |
| 28 | Load | https://docs.sstcae.com/users_manual/load.htm | 2026-09-18T05:28:42Z | Load-name propagation down the rows |
| 29 | Data | https://docs.sstcae.com/users_manual/data.htm | 2026-09-18T05:28:43Z | The single data-item column |
| 30 | Comment | https://docs.sstcae.com/users_manual/comment.htm | 2026-09-18T05:28:44Z | Comment rows; hydrostatic test load as a comment row |
| 31 | Element types | https://docs.sstcae.com/users_manual/element_types_1.htm | 2026-09-18T05:28:44Z | How an element is entered |
| 32 | Data types | https://docs.sstcae.com/users_manual/data_types_1.htm | 2026-09-18T05:28:45Z | How a data item is entered |
| 33 | Shortcut keys for Element types | https://docs.sstcae.com/users_manual/shortcut_keys_for_element_types.htm | 2026-09-18T05:28:46Z | **The full element-type enumeration** |
| 34 | Shortcut keys for Data types | https://docs.sstcae.com/users_manual/shortcut_keys_for_data_types.htm | 2026-09-18T05:28:46Z | **The full data-type enumeration** |
| 35 | Load cases | https://docs.sstcae.com/users_manual/load_cases.htm | 2026-09-18T05:28:47Z | The load cases and how they are selected |
| 36 | Load Combinations | https://docs.sstcae.com/users_manual/load_combinations.htm | 2026-09-18T05:28:47Z | Combination count |
| 37 | Units (Ctrl+U) | https://docs.sstcae.com/users_manual/units__ctrl_u_.htm | 2026-09-18T05:28:48Z | Per-item unit selection stored with the model |
| 38 | Hydrotest Load | https://docs.sstcae.com/users_manual/hydrotest_load.htm | 2026-09-18T05:28:49Z | Hydrotest as a specialised comment |
| 39 | Layout Window Menus | https://docs.sstcae.com/users_manual/layout_window_menus.htm | 2026-09-18T05:28:49Z | Menu structure |
| 40 | Loads Menu | https://docs.sstcae.com/users_manual/loads_menu.htm | 2026-09-18T05:28:50Z | The load kinds the program carries |
| 41 | Options Menu | https://docs.sstcae.com/users_manual/options_menu_1.htm | 2026-09-18T05:28:52Z | Analysis options, units, node increment |
| 42 | Misc Menu | https://docs.sstcae.com/users_manual/misc_menu.htm | 2026-09-18T05:28:53Z | Lists, checks, spectrums, soils, allowables |
| 43 | CAEPIPE User's Manual (root) | https://docs.sstcae.com/users_manual/index.htm | 2026-09-18T05:28:23Z | Manual root; contents tree is script-rendered |
| 44 | SST Systems — CAEPIPE Translators | https://www.sstcae.com/translator | 2026-09-18T05:28:54Z | Translator products in and out of CAEPIPE |
| 45 | SST Systems — PDMS/E3D to CAEPIPE (PDMS2KP) | https://www.sstcae.com/translator/pdmse3d-to-caepipe | 2026-09-18T05:28:55Z | Output file is `.mbf`; what the translator carries |
| 46 | CADMATIC — CAEPIPE export (third party) | https://docs.cadmatic.com/plant/Content/Plant%20Modeller/Model/caepipe_export.htm | 2026-09-18T05:28:56Z | An independent generator of `.mbf`, with a worked file |

### 1.2 Unreachable

| Source | URL | Outcome | Time (UTC) |
|---|---|---|---|
| KP2CII — CAEPIPE-to-CAESAR II user's manual (PDF) | https://www.sstusa.com/pdfs/KP2CII.pdf | 301 to `https://www.sstcae.com/`; the PDF is no longer served | 2026-09-18T05:28:57Z |
| CPTOPS — CAEPIPE-to-PIPESTRESS user's manual (PDF) | https://www.sstusa.com/pdfs/CPTOPS.pdf | 301 to the home page | 2026-09-18T05:28:59Z |
| Release Announcement V14.00 (PDF) | https://www.sstusa.com/pdfs/ReleaseAnnouncement_V1400.pdf | 301 to the home page | 2026-09-18T05:29:00Z |
| SST Systems — What is new / release notes | https://www.sstcae.com/what-is-new | Served, but release-note bodies are script-rendered; no format statements were in the served text | 2026-09-18T05:29:02Z |
| CAEPIPE Technical Reference Manual | (search results point only to third-party re-hosts: scribd, pdfcoffee, idoc.pub, wayplant) | Not read: re-hosted copies of unknown licence and unverified fidelity were deliberately not used | — |

These four gaps matter for one thing only: the per-version history of MBF changes. Version applicability
is nonetheless recorded inside the MBF specification itself, field by field [S: 7], so nothing in §3–§5
depends on the unreachable release notes.

---

## 2. Formats CAEPIPE reads and writes

### 2.1 Reads (import)

The Main window File > Import command accepts three neutral files: the CAEPIPE neutral file `.mbf`,
an Algor (Autodesk) PipePak neutral file `.pnf`, and a CII neutral file `.cii` [S: 1].

| Format | Direction | Documented well enough to write to? |
|---|---|---|
| `.mbf` model batch file | import and export | **Yes.** Appendix A gives sections, key letters, comment keys, units and a complete worked example [S: 7, 9] |
| `.pnf` (Algor/Autodesk PipePak) | import only | **No.** The manual directs the reader to contact SST for details of `.pnf` and `.cii` import [S: 7] |
| `.cii` (CAESAR II neutral) | import only | **No**, same statement [S: 7] |

### 2.2 Writes (export)

The manual enumerates nine documented outputs [S: 2]:

| # | Output | Form | Documented enough to write to? |
|---|---|---|---|
| 1 | Model and results to CSV and TEXT, via File > Print > Print to file | text | Output only; no round-trip is claimed |
| 2 | Model and results to PDF, via a PDF print driver | PDF | Output only |
| 3 | **Model to MBF**, via File > Export to MBF | text/CSV | **Yes** — Appendix A [S: 7] |
| 4 | Model to PCF (Piping Component File), via Layout Frame > File > Export to PCF | text | Element-by-element and data-type-by-data-type attribute mapping is published [S: 13, 14], with an explicit list of what is dropped [S: 16, 17] |
| 5 | Model to PDMS, E3D and CADMATIC | `.mac` (PDMS) / `.3dd` (CADMATIC) | Described as structural components, "only dumb graphics without attributes" [S: 5] |
| 6 | Material library to an ASCII file | `.mlb` | **Yes** — a second documented text batch format [S: 20, 21] |
| 7 | Time History results to CSV | CSV | Results output |
| 8 | Hanger report to LICAD via an ASCII `.cli` file | `.cli` | Named only; no field spec found |
| 9 | Deformed geometry to PDMS, E3D, CADMATIC | as #5 | Graphics only |

### 2.3 The native file and the batch file

Importing an `.mbf` creates the corresponding binary model file `.mod`, which is then shown in the
Layout window [S: 7]. Exporting from the Layout window writes an `.mbf` that may be edited in a text
editor and read back, with the warning that the existing `.mod` is overwritten, so the edited text
file must be renamed before re-import [S: 8]. Export offers two dialects: the current version's
format, and "Model batch file 6.xx" for backward compatibility with CAEPIPE 6.81 through 10.00 [S: 8].

The MBF is a line-oriented text file and the manual states it is "essentially a CSV ... file format"
[S: 10]. It can be produced from a spreadsheet: the manual publishes a sample generated in Excel using
absolute coordinates [S: 10]. An independent vendor, CADMATIC, publishes its own generated `.mbf` and
describes it as ASCII [S: 46], and SST's PDMS/E3D translator names its output file as a CAEPIPE model
file `.mbf` [S: 45]. So three independent producers of the format are publicly documented.

### 2.4 Batch execution

An `.mbf` can be given to the executable on the command line, which analyses it and writes results as
CSV beside the input file; the documented second argument is `batchmode=yes` with no spaces, the two
arguments separated by a semicolon [S: 11]. Dragging an `.mbf` onto the executable does the same
[S: 7]. Maximum file path length was raised from 80 to 256 characters at Version 15.00 [S: 11].

*Design note, not a compatibility claim:* this is the mechanism by which a table-authored file could be
analysed without a human driving the GUI. Whether it works for files this product writes is later evidence.

### 2.5 Out of CAEPIPE, by separate products

Separate paid translators convert `.mod` files to CAESAR II neutral `.cii` and to PIPESTRESS `.fre`;
translators into CAEPIPE exist from PDMS/E3D, CADMATIC, Tribon, CATIA, PDS, PCF and AutoPLANT [S: 44].
These are products, not published formats, and their manuals were not reachable (§1.2).

---

## 3. Layout row fields — the `.mbf` LAYOUT section

### 3.1 File structure

Input is given in a fixed order of sections. Except for the heading, each section begins with a
keyword on its own line, followed by that section's data; **only the first three characters of the
keyword are significant** [S: 7]. Any number of heading lines may precede; only the first becomes the
model title [S: 7]. The documented sections, in order [S: 7]:

`Heading`, `VERSION`, `OPTIONS`, `FCURVE`, `FCYCLE`, `MATERIAL`, `SOILS`, `PIPE`, `LOADS`,
`BMATERIALS`, `BSECTIONS`, `BLOADS`, `SPCMS`, `SPLVL`, `LAYOUT`, `PUMPS`, `COMPRESSORS`, `TURBINES`,
`ACHXS`, `FIHTR`, `SEISMIC`, `ASCES`, `WIND`, `ASCEW`, `1991W`, `WIND1`–`WIND4`, `ALLOWABLES`,
`PSDLS`, `PSDLD`, `SNOWICE`, `WAVE1`–`WAVE4`.

`VERSION` carries the MBF version, e.g. `11.00` [S: 7]; the published examples show `13.00` and `14`
[S: 9, 10].

### 3.2 The LAYOUT record

A layout line is a sequence of **(key letter, data) pairs separated by commas, which may be given in
any order**, with one exception: comments must come last. A line containing only continued comments
is acceptable [S: 7]. A trailing comma on the last comment continues it on the next line [S: 7], and
the published examples use that continuation heavily [S: 9].

**Key letters** [S: 7]:

| Key | Meaning |
|---|---|
| `*` | Model comment line; up to 70 characters |
| `F` | **From node.** Starts a new branch; its `X`,`Y`,`Z` are absolute coordinates, not offsets |
| `T` | **To node.** From the previous `F` or previous `T` — but not from a previous Location node |
| `L` | **Location node.** Additional data at a node that already carries a data item |
| `H` | Hydro test load |
| `K` | Code for the node (see 3.3) |
| `M` | Material name; retained until changed |
| `P` | Pipe (section property) name; retained until changed |
| `J` | Joint code (see 3.4) |
| `X`,`Y`,`Z` | Offsets (or, on an `F` row, coordinates) |
| `B` | Bend radius; only meaningful where a tangent intersection has been specified; default is long radius |
| `C` | Start of the comment section; comments separated by commas |

So the file's own grammar is: **one line per node, carrying what arrives at that node, with the
From-ness of the line stated by its prefix letter.** A branch is a line beginning `F<node>` naming an
existing node, after which `T` lines continue from it. The manual states this directly: when a new
branch is started, the first node of the branch is given as a From node [S: 7]. Both worked examples
show it — SST's own file has bare `F100`, `F240`, `F260`, `F680`, `F940` lines re-anchoring to earlier
nodes, one preceded by the comment `*Branch Pipe` [S: 9]; CADMATIC's generated file has `F110` [S: 46].
On a From row for an already-defined node, coordinates need not be repeated because the program already
knows them [S: 25].

### 3.3 `K` — node codes

| Code | Meaning | Note |
|---|---|---|
| `A` | Anchor | |
| `B` | Branch connection | |
| `G` | Generic Support | Version 10.30 or later |
| `H` | Hinge | To node only |
| `M` | Miter bend | To node only |
| `I` | Tangent intersection | To node only; this is how a bend is expressed |
| `T` | Welding Tee | |
| `S` | Sweepolet | |
| `W` | Weldolet | |
| `F` | Fabricated Tee | |
| `E` | Extruded Tee | |
| `R` | Radiused branch | |
| `P` | Branch on thickened pipe | |

### 3.4 `J` — joint codes

| Code | Meaning | Note |
|---|---|---|
| `B` | Ball joint | |
| `C` | Cut pipe | Cold spring |
| `D` | Reducer | |
| `E` | Expansion | |
| `I` | Jacket bend | |
| `L` | Elastic element | |
| `M` | Beam | |
| `P` | Jacket pipe | |
| `R` | Rigid | |
| `S` | Slip joint | |
| `T` | Tierod | |
| `V` | Valve | |

A plain pipe carries no joint code; in the Layout window the Pipe type is likewise shown as a blank
[S: 33]. The manual instructs that the weight of a rigid joint or valve is given by a `WGT` comment,
expansion-joint stiffnesses by `ES`, and pressure thrust area by `TA` [S: 7].

### 3.5 Geometry, offsets and coordinates

`X`, `Y`, `Z` are offsets from the previous node, except on an `F` row where they are absolute
coordinates [S: 7, 25]. In English units they may be written as combinations of feet, inches and
fractions; the manual's documented entry forms [S: 7]:

| Entry | Value |
|---|---|
| `-10` | −10 ft |
| `10'8` or `10-8` | 10 ft 8 in |
| `0'8` or `0-8` | 8 in |
| `10.5` | 10 ft 6 in |
| `1'6-3/8` or `1-6-3/8` | 1 ft 6.375 in |

In SI units, offsets are mm [S: 7]. A node number suffixed with `*` makes that row's `X`,`Y`,`Z`
absolute coordinates rather than offsets; the Excel-generated sample uses `T20*`, `T30*` and so on
throughout [S: 10], and the Layout window documents the same `*` convention [S: 25].

Angles are degrees by default; the `RAD` option in `OPTIONS` switches to radians [S: 7]. The vertical
axis is Y by default; the `Z` option in `OPTIONS` makes Z vertical [S: 7]. Units are English by
default; the `SI` option switches to SI [S: 7].

Nodes are numeric. The Layout window states node numbers should be numeric except for
internally-generated nodes with suffixes `A`, `B`, `C`, `D` and `J`, and absolute-coordinate nodes;
`A`/`B` are the near and far ends of a bend's curve, `C`/`D` the same for a jacketed bend, and `J` the
end node of a jacket pipe [S: 23]. Within the MBF, intermediate bend nodes are constrained to
`>1` and `<99999` [S: 7]. SST's example file uses `L200A` and `L530B`, i.e. Location rows at
generated bend-end nodes [S: 9].

### 3.6 Element-specific data, by element

Element geometry beyond the offsets is carried in the comment section. The documented keys by element
[S: 7]:

| Element | Expressed as | Its own fields |
|---|---|---|
| Pipe | no `J`/`K` code | section and material references only |
| Bend | `K=I` plus `B=<radius>` | `BTHK` bend thickness, `BMAT` bend material, `FF` flexibility factor, `BSIF(in,out)`, `IN1(node,angle)`, `IN2(node,angle)` |
| Miter bend | `K=M` | `WS` widely spaced; bend keys above |
| Jacketed bend | `J=I` | `JTHK`, `JR` jacket radius, `JMAT`, `JSEC`, `JLOAD`, `IN1`, `IN2` |
| Jacketed pipe | `J=P` | `JMAT`, `JSEC`, `JLOAD` |
| Valve | `J=V` | `VWGT` weight, `THKF` thickness factor, `INSF` insulation factor, `AWGT` additional weight, `OFFSET(x,y,z)` |
| Reducer | `J=D` | `OD1`, `OD2`, `THK1`, `THK2`, `CONE` cone angle |
| Rigid element | `J=R` | `WGT` |
| Bellows / expansion joint | `J=E` | `ES(axial,lateral,torsional)`, `TA` thrust area, `BK` bending stiffness, `WGT` |
| Slip joint | `J=S` | `FFOR` friction force, `FTOR` friction torque, `TA`, `WGT` |
| Ball joint | `J=B` | `FRCT(bending,torsional)`, `ROTL(bending,torsion)`, `RK(bending,torsion)`, `WGT` |
| Hinge joint | `K=H` | `RLIM` rotation limit, `DV(x,y,z)` direction vector |
| Tie rod | `J=T` | `KTIE(tension,compression)`, `GAP(tension,compression)` |
| Elastic element | `J=L` | `TRAK(kx,ky,kz)`, `ROTK(kxx,kyy,kzz)`, `LXAX(x,y,z)`, `LYAX(x,y,z)` |
| Beam | `J=M` | `MAT` beam material, `SEC` beam section, `LOAD` beam load, `BETA` |
| Cut pipe (cold spring) | `J=C` | `LONG`, `SHORT` |
| Hydrostatic test load | `H` row prefix | `HSG` specific gravity, `HPRES` pressure |
| Comment | `*` row prefix | free text, up to 70 characters |

Temperature and pressure may also be written **on a layout row** with the comment keys `T`/`TEMP`
(deg F or C) and `P`/`PRES` (psig or bar); the manual's own LAYOUT illustration shows
`F10,KA,M5,P12,CT=650,P=500` [S: 7]. `AMB` sets the ambient/reference temperature, default 70 F
[S: 7], `SG` the specific gravity, `U`/`UNIF` a uniform load [S: 7].

### 3.7 Records the layout row refers to

**`PIPE` (section properties)**, one line per section [S: 7]. Name is up to 5 characters; OD must be
the actual OD, not the nominal size, and thickness an actual value, not a schedule:

`Name, OD, Thk, Corrosion allowance, Mill tolerance (%), Insulation density, Insulation Thk, Lining density, Lining Thk, Soil name`

Units: inch / lbf per ft3 in English, mm / kgf per m3 in SI [S: 7].

**`MATERIAL`** [S: 7]. Name up to 5 characters. First line:

`Name, Density, Poisson's ratio, [Long. joint factor], [Circ. joint factor], [Type], [Description], [Yield], [Tensile], [Slope M], [Constant], [Endurance], [Fatigue Curve Name]`

Then one line per temperature point: `Name, Temp, E, alfa, [allowable stress], [yield stress], [rupture stress]`.
For FRP types (`Type` = `FR`, `F1`, `F2`) the temperature lines instead carry
`Name, Temp, E, hoop modulus, shear modulus, alfa, [hoop allowable], [torsional/shear allowable], [axial allowable]`.
Density is **weight** density (lbf/in3, kgf/m3), not mass density; the manual is explicit that CAEPIPE
takes weights, not masses, and derives mass internally as weight divided by g [S: 7, 37].

**`LOADS`** — this is where temperatures and pressures live as named records [S: 7]. Name up to 5 characters:

`Name, T1, P1, Specific Gravity, [T2, P2, T3, P3, Additional weight, Peak Temperature, Peak Pressure, T4, P4, T5, P5, T6, P6, T7, P7, T8, P8, T9, P9, T10, P10, Wind1, Wind2, Wind3, Wind4, Snow, Ice, Wave, Wave2, Wave3, Wave4]`

Units: F / psi / lbf per ft in English; C / bar / kgf per m in SI [S: 7]. `T4`–`T10` and the peak pair
require Version 10.10 or later; the wind flags 10.20/10.40; snow, ice and the wave flags 14.00 [S: 7].
The trailing flags are include/exclude switches (1 or 0). If the peak pair is absent, the maxima of
the operating temperature and pressure are used as peak [S: 7].

**`SOILS`** for buried pipe, keyed by a 3-character soil name referenced from the `PIPE` record; the
field list differs by code (general, EN 13941-1, ISO 14692-3) and by cohesive/cohesionless type [S: 7].

**Beam records**: `BMATERIALS` (`Name, E, [Poisson, Density, Alfa]`), `BSECTIONS`
(`Name, Axial area, Major I, Minor I, [Torsional I, Major shear area, Minor shear area, Depth, Width]`),
`BLOADS` (`Name, T1, [T2, T3, Additional weight, T4…T10, Wind1…Wind4]`) [S: 7].

**Equipment**: `PUMPS`, `COMPRESSORS`, `TURBINES`, `ACHXS` (air-cooled heat exchangers, 13.20 or
later) and `FIHTR` (fired heaters, 13.20 or later), each a description, an axis vector and the node
numbers of the connections [S: 7].

### 3.8 Units conventions, summarised

- The MBF carries a **single binary unit choice** for the whole file: English by default, SI via the
  `SI` keyword in `OPTIONS` [S: 7]. The GUI, by contrast, lets the engineer set any combination of
  English, SI and Metric per item, saved with the model [S: 37].
- Lengths: inch/feet-inch-fraction (English) or mm (SI); pressure psig or bar; temperature F or C;
  force lb or N; moment ft-lb or Nm, except where a key states in-lb or Nm/deg; density is weight
  density; stiffnesses lb/in or N/mm translational and in-lb/deg or Nm/deg rotational [S: 7].
- An environment variable `EXPORT_INCHES=YES`, with the `.mod` in inches, exports the MBF with length
  in inches and moments in lb-in, stated to avoid rounding loss across export and import [S: 6].
- A default of four decimals on a length input can be raised to seven with `INCREASE_DIGITS=YES`,
  though such data "can be printed other than to TXT and CSV formats" [S: 25].

---

## 4. Attachment and auxiliary data

### 4.1 How a node carries an attachment

In the Layout window, **`Data` is a single column and a row carries one data item**; where a node
needs more than one item, a **Location** row for the same node carries the next [S: 7, 22, 29]. The
MBF expresses the same thing: `L` is the Location key letter, "used to input additional data at a node
when the node has more than one data item such as a hanger/force" [S: 7]. SST's example shows
`L10,CFLANGE=WN,WGT=220` and `L370,CFLANGE=WN,WGT=58` doing exactly this [S: 9].

The data item itself is written in the comment section of the row. Anchors, branch SIFs and generic
supports are the exception: they are written as `K` codes in the line body [S: 7], although the Layout
window classes them as Data types, not element types [S: 34]. **[I]** — that is a comparison of two
cited lists, not a statement the manual makes.

### 4.2 Field tables by attachment

Restraints and supports [S: 7]:

| Item | Declared by | Fields |
|---|---|---|
| Anchor | `K=A` | `TRAK(kx,ky,kz)`, `ROTK(kxx,kyy,kzz)`, `FREE` / `FREEY` (free during hanger design), `LA` local or global anchor (10.10+), `LD` local or global displacement, `TAG` (≤14 chars, 10.10+) |
| Restraint, 2-way | `F`/`FIXD(x,y,z)` translational, `FIXR(x,y,z)` rotational; also `FIXDX=1`, `FX`, `FIXRY` forms | `TAG` |
| Guide | `G` | `GGAP` gap, `STIFF` stiffness, `MU` friction coefficient, `CNOD` connected node, `TAG` |
| Limit stop | `LS(M1,M2)`, M1 negative-direction allowance, M2 positive | `DV(x,y,z)`, `STIFF`, `MU`, `CNOD`, `AXIAL` / `SHEARY` / `SHEARZ` local-axis selection (10.50+), `TAG` |
| Skewed restraint | `K=<translational stiffness>` with `DV(...)`; `KR=` rotational with `DV(...)` | `MU`, `CNOD`, `AXIAL`/`SHEARY`/`SHEARZ`, `TAG` |
| Snubber | *not separately documented in the key list* (see §7) | `STIFF`, `DV(...)`, `CNOD`, `AXIAL`/`SHEARY`/`SHEARZ` are all stated to apply to snubbers |
| Generic support | `K=G` (10.30+) | `GRA1`, `GRA2` translational stiffness groups; `GRB1`–`GRB3` coupling; `GRC1`, `GRC2` rotational; displacements from 12.00 |
| Nozzle | `NOZZLE=650` (API 650), `297` (WRC 297) or `5500` (10.10+) | `NOD`, `NTHK`, `VOD` vessel OD or sphere radius, `VTHK`, `E`, `L1`, `L2`, `RPAD`, `DV(...)`, `TAG` |

Hangers and springs [S: 7]:

| Item | Declared by | Fields |
|---|---|---|
| Variable spring hanger (to be designed) | `VS` or `VS=n` | `HTYP` hanger type number, `MLV` maximum load variation % (default 25), `SR` short range, `CNOD`, `TAG`, `LTAG` |
| User hanger | `US(no. of hangers, spring rate, hot load)`; `US(1,0,2300)` is a constant support | `CLD` cold load, `CNOD`, `TAG` |
| Constant support | `CS=n` | `CNOD` |
| Rod hanger | *not separately documented in the key list* (see §7) | `CNOD` and `TAG` are stated to apply to it |
| Hanger design standard | `OPTIONS` line: `HGRA` Grinnell (default), `HGRB` Bergen-Paterson, `HGRC` Fee and Mason, `HGRD` Basic Engineers, `HGRE` Power Piping, `HGRF` Nordon, `HGRG` Carpenter and Paterson, `HGRH` NPS, `HGRI` PTP, `HGRJ` Corner and Lada, `HGRK` Elcen | |
| `HTYP` values | 35 numbered manufacturers, published as a table: ABB-PBS 1 … Gradior 35 | [S: 9] |

Loads and masses at a node [S: 7]:

| Item | Declared by | Fields |
|---|---|---|
| Concentrated weight | `CWGT` (lbf or kgf) | `OFFSET(x,y,z)` from the node |
| Force | `FOR(x,y,z)` or `FORX=`, `FORZ=` | `MOM(x,y,z)` or `MOMX=`; `FLC` selects the case the force is added to: 1 Sustained, 2–11 T1–T10, 12 static seismic (10.30+) |
| Harmonic load | `FOR(...)` | `FRE` frequency (Hz), `PH` phase |
| Uniform load | `U` or `UNIF` (lbf/ft or kgf/m) | |
| Specified displacement | `D`/`DIS(x,y,z)`, `DIS2`, `DIS3`, `DIS4`–`DIS10`, `DISD` (peak) | Anchor, Nozzle (10.10+), Generic Support (12.00+). **A zero entry is ignored and is not treated as a specified displacement** |
| Specified rotation | `ROT(...)`, `ROT2`, `ROT3`, `ROT4`–`ROT10`, `ROTD` | Same zero rule |
| Seismic displacement / rotation | `SEIS(x,y,z)` / `SERO(rx,ry,rz)`; `SEIS2`, `SERO2`, `SEIS3`, `SERO3` (12.00+) | |
| Settlement | `SETT(x,y,z)` / `STRO(rx,ry,rz)` | |
| Wind displacement / rotation | `WIND1`–`WIND4` / `WINR1`–`WINR4` (12.00+) | |

Fittings, SIFs and node attributes [S: 7]:

| Item | Declared by | Fields |
|---|---|---|
| Flange | `FLANGE=<type>`: `WN` weld neck, `SO` single welded slip on, `DW` double welded slip on, `SW` socket welded, `FW` fillet welded, `LJ` lap joint, `TH` threaded | `WGT`, `GDIA` gasket diameter, `BDIA` bolt circle diameter (12.00+, EN 13480-3 flange equivalent pressure) |
| Branch SIF | `K` codes `T`, `S`, `W`, `F`, `E`, `R`, `P`, `B` (§3.3) | `PAD` reinforcement thickness (fabricated tee), `CRTCH` crotch radius (extruded tee), `R` fillet radius (radiused branch, branch connection), `THK` (radiused branch, branch on thickened pipe), `LEN` (branch on thickened pipe) |
| User SIF | `SIF=v` or `SIF(in-plane,out-plane)` | `SIFA` axial, `SIFT` torsional (10.10+); `SIA`, `SII`, `SIO`, `SIT` sustained indices and `SIFB` branch flag (13.20+, B31.3 2024 or later) |
| IGEM SCFs | `IPS`, `IBS`, `ITS`, `IQS`, `ISS` sustained; `IPC`, `IBC`, `ITC`, `IQC`, `ISC` cyclic | IGEM code only |
| Weld | `WTYPE=` 1 butt, 2 fillet, 3 concave fillet, 4 tapered transition | `MM` mismatch |
| Threaded joint | `TJOINT` | none |
| Jacket end cap | `JCAP` | none |
| Spider | `SPIDER` | none |
| Support tag | `TAG` (≤14 chars) | applies to anchor, guide, hanger, limit stop, nozzle, restraint, rod hanger, skewed restraint, user hanger |
| Spectrum level tag | `LTAG` (10.50+) | on supports, when spectrums and levels are defined |

Allowable loads on equipment and anchors, `ALLOWABLES` section [S: 7]:

`Node, FX/P, FY/VL, FZ/VC, MX/MT, MY/MC, MZ/ML, [FR, MR]` — for a nozzle these are radial, y shear, z
shear, torque, circumferential and longitudinal moment; for an anchor they are global forces and
moments; resultant force and moment are optional and require Version 13.00 or later.

---

## 5. Load cases and analysis options

### 5.1 What the format carries

Analysis options, as optional sections valid from CAEPIPE 10.10 unless noted [S: 7]:

| Section | Carries |
|---|---|
| `OPTIONS` | Hanger design standard, piping code (a documented list of 42 code identifiers from `B311` to `ISO14692`), `SI`, `RAD`, `Z` |
| `OPCODE` | Axial force in stress, liberal allowable, B31J, service level, B31.8 design factor, Z183 location factor, class location, sour service, EN 13480 seismic factor. The manual warns the selected code may force some of these regardless of file content |
| `OPTEMP` | Reference temperature, number of thermal cycles, number of thermal loads (1, 2, 3 or 10), solve thermal case, use cold modulus |
| `OPPRES` | Pressure stress option, Bourdon effect, pressure correction for bends, peak pressure factor, Bourdon-to-case assignment (13.00+) |
| `OPMISC` | Cutoff frequency, number of modes, missing mass, friction in dynamics, hanger stiffness, spectrum mode/direction/group sums, gravitational acceleration and direction (15.10+) |

Loading data [S: 7]: `LOADS` and `BLOADS` carry the temperature and pressure sets T1–T10 / P1–P10 plus
peak, the specific gravity, additional weight and the wind/snow/ice/wave include flags;
`SEISMIC` or `ASCES` (ASCE/SEI 7-16) carry static seismic; `WIND`, `ASCEW`, `1991W` and `WIND1`–`WIND4`
carry wind profiles; `SNOWICE` (13.00+, ASCE/SEI 7-22) snow and ice; `WAVE1`–`WAVE4` (14.00+) wave and
current; `SPCMS`/`SPLVL` (10.50+) spectrums and levels; `PSDLS`/`PSDLD` (12.20+) random vibration;
`FCURVE`/`FCYCLE` (13.00+) fatigue curves and the actual number of cycles per expansion case.

`FCYCLE` is the only place where a **load case is named in the file**, and only as a label for a cycle
count: the documented examples are `Expansion (T1)`, `Expansion (T2)`, `Expansion (T1-T2)` and so on,
with a cycle count on each line [S: 7].

### 5.2 What the format does not appear to carry

The load cases themselves are selected in the GUI, under Loads > Load cases, mostly as checkboxes;
by default Sustained (W+P), Empty Weight (W), Expansion (T1) and Operating (W+P1+T1) are already
selected, and some cases appear in the dialog only after their data exists — response spectrum after
spectrums are input, cold spring cases after a cut-pipe element is entered, multiple expansion and
operating cases after multiple thermal loads are set [S: 35]. Load combinations are a separate command,
and the manual states the program can analyse "a total of up to 115+ load combinations" including 55
thermal ranges [S: 36].

**No section in the documented MBF keyword list selects load cases or defines load combinations**
[S: 7]. Nor is there a documented section for a time history load, a force spectrum, time functions, a
relief-valve load, the simplified/detailed fatigue evaluation settings beyond `FCURVE`/`FCYCLE`, the
per-item unit selection, or the QA Block and Revision Record that the File menu offers [S: 3, 40, 37].
Whether the importer supplies defaults, and which, is not stated. See §7.

---

## 6. Mapping — the chosen grammar onto the documented format

The grammar is the one fixed in `DIRECTION_DECISION_2026-09-17.md` §2 item 1 and elaborated in
`DESIGN_BRIEF_V1.md` §4: one row per node carrying the element that arrives at it, From on every row
defaulting to the previous row's node, a branch being a row whose From names an earlier node, element
type as a column, then DX/DY/DZ or absolute coordinates as a switchable group, section, material,
temperatures, pressures, and a mark per attachment class that opens a joined row.

Marks: **D** direct, **T** transformable (how is stated), **M-us** missing on our side,
**M-them** missing in the documented format.

### 6.1 Layout table

| Our column | MBF field | Mark | Note |
|---|---|---|---|
| Node | the node number after the row-prefix key letter | D | |
| Node, alphanumeric IDs | — | **M-them** | Node numbers are documented as numeric apart from the generated `A`/`B`/`C`/`D`/`J` suffixes and the `*` coordinate form [S: 23]. **Gap 1** |
| From (explicit on every row) | `F` prefix, else implicit continuation from the previous row | **T** | The format states From only where it changes; our always-visible From projects to `F` when it differs from the previous row's node and to `T` otherwise. Order-independence is ours, not the file's: the file is positional in that a `T` row means "from the previous row" [S: 7] |
| Branch (From names an earlier node) | a bare `F<node>` line re-anchoring to an existing node | **D** | Demonstrated in both published examples [S: 9, 46] |
| Element type = pipe | blank | D | |
| Element type = bend | `K=I` + `B=<radius>` | **T** | The file names the *node kind* (tangent intersection), not the element. Radius must accompany it |
| Element type = valve, reducer, rigid, bellows, slip joint, ball joint, tie rod, elastic, beam, cut pipe, jacketed pipe, jacketed bend | `J=` code (§3.4) | **T** | A published table from element-type names to `J`/`K` codes does not exist; the correspondence in §3.6 is assembled from two documented lists [I]. **Gap 2** |
| Element type = miter bend, hinge | `K=M`, `K=H` | T | Both "To node only" |
| Element type = **tee** | — | **M-them** | A tee is not an element type in CAEPIPE; it is a branch-SIF data item written as a `K` code at the node [S: 33, 34, 7]. Our column must either carry the branch-SIF kinds as element types and project them to `K`, or move them to a node-data table. **Gap 3, and a decision for the design** |
| DX, DY, DZ | `X`,`Y`,`Z` on a `T` row | D | |
| Absolute coordinates (switchable) | `X`,`Y`,`Z` on an `F` row, or node number suffixed `*` | **D** | The `*` form is documented for both the Layout window and MBF, and the Excel sample uses it for every row [S: 10, 25]. This is the switch the brief asks for, already present in the file |
| Section | `P=<name>` | **T** | Ours is a value per row; the file writes it only on change and it is retained until altered [S: 7, 27]. Export must suppress unchanged repeats or accept redundancy; the manual says another section "should be entered only when there is a change", not that repetition is an error. **Open, see §7** |
| Material | `M=<name>` | T | Same as section [S: 7, 26] |
| Temperature(s) | `C…T=` / `TEMP=` on the row, **or** the `LOADS` record named by the row's load | **T** | Two documented routes. A named load record carries T1–T10 and P1–P10; the row comment carries a single T. Our per-row temperature columns are closest to the comment route; our multi-thermal work is closest to the `LOADS` route. **Gap 4** |
| Pressure(s) | `C…P=` / `PRES=`, or the `LOADS` record | T | As above |
| Load name (the row's load reference) | `C…L=<name>` in both published examples | **M-them (documentation)** | `CL=54I`, `CL=1` appear throughout SST's own sample and in CADMATIC's generated file, referencing `LOADS` names; **the comment-key list in the specification contains no `L` key** [S: 7, 9, 46]. This is the single most consequential documentation gap for an exporter. **Gap 5** |
| Specific gravity | `SG` on the row, or field 4 of the `LOADS` record | T | |
| Bend radius | `B` | D | |
| Comment / note on a row | `C` comments; `*` for a model comment line (≤70 chars) | T | Our free text has no length rule; the file's model comment does |
| Row order | line order | D | |
| Attachment marks (restraint, load, node data) | no field | **M-them** | The marks are ours; on export they become the presence of the corresponding comment keys. Nothing is lost, but nothing round-trips the mark itself |
| Origin per row and per cell (entered, agent proposal accepted, propagated, generated, imported) | no field | **M-them** | **Gap 6.** Governance provenance has no place in the file. It stays in our store; an export loses it |
| Checked tag | no field | **M-them** | **Gap 7.** Same as origin. It is a human tag on our row, not a model fact |

### 6.2 Attachment tables

| Our table and columns | MBF | Mark | Note |
|---|---|---|---|
| Restraints: type | `FIXD`/`FIXR`, `G`, `LS`, `K=`+`DV`, `K=A`, `K=G`, `VS`, `US`, `CS`, `NOZZLE=` | T | Our single "type" enumerates across what the file splits between `K` codes and comment keys |
| Restraints: direction | `FIXD(x,y,z)`, `FIXR(x,y,z)`, `DV(x,y,z)`, `AXIAL`/`SHEARY`/`SHEARZ` | D | |
| Restraints: gap | `GGAP` (guide), `GAP(t,c)` (tie rod), `LS(M1,M2)` (limit stop) | T | The file has no single "gap"; it is per support kind |
| Restraints: friction | `MU`; `FFOR`/`FTOR` for slip joints; `FRCT` for ball joints | T | |
| Restraints: stiffness | `STIFF`, `K=`, `KR=`, `KTIE`, `TRAK`, `ROTK`, `GRA*`/`GRB*`/`GRC*` | T | Per kind, not one field |
| Restraints: connecting node | `CNOD` | D | Documented for guide, hanger, limit stop, rod hanger, skewed restraint, user hanger, snubber, constant support |
| Restraints: tag | `TAG` (≤14 chars) | D | Our tag must be length-limited on export |
| Loads: kind | `FOR`, `MOM`, `U`/`UNIF`, `CWGT`, `DIS*`, `ROT*`, `SEIS*`, `SETT`, `WIND*`, `HPRES`/`HSG` | T | |
| Loads: direction | the `(x,y,z)` triple on each key | D | |
| Loads: value with unit | the scalar in the key; units from the file-level English/SI switch | **T** | The file has one unit system per file; our per-value units must be converted on export. **Gap 8** |
| Loads: which case a force acts in | `FLC` (1 sustained, 2–11 T1–T10, 12 static seismic) | D | 10.30 or later |
| Node data: flanges | `FLANGE=`, `WGT`, `GDIA`, `BDIA` | D | |
| Node data: SIFs | `SIF`, `SIFA`, `SIFT`, `SIA`/`SII`/`SIO`/`SIT`, `SIFB` | D | |
| Node data: welds | `WTYPE`, `MM` | D | |
| Node data: concentrated mass | `CWGT` + `OFFSET`; the file takes **weight**, not mass | **T** | The manual is explicit that weight is the input and mass is derived internally [S: 7, 37]. If our column is named mass, the export is a division by g and the name is wrong. **Gap 9** |
| More than one attachment on one node | a `L<node>` Location row per extra item | **T** | Our joined tables allow N rows per node freely; export must serialise the second and later items onto Location rows [S: 7, 9, 22] |
| Snubber | *no declaration key documented* | **M-them (documentation)** | `STIFF`, `DV`, `CNOD`, `AXIAL`/`SHEARY`/`SHEARZ` are each said to apply to snubbers, but no key declares one. **Gap 10** |
| Rod hanger | *no declaration key documented* | **M-them (documentation)** | `CNOD` and `TAG` are said to apply to it. **Gap 11** |
| Time varying load | *no key or section documented* | **M-them** | A Layout-window data type [S: 34] with no MBF counterpart in the keyword list. **Gap 12** |

### 6.3 Load cases table

| Our table | MBF | Mark | Note |
|---|---|---|---|
| Load cases: the case rows (Sustained, Operating, Expansion, …) | no section | **M-them** | Selected in the GUI [S: 35]. **Gap 13** |
| Load case combinations | no section | **M-them** | A separate GUI command [S: 36]. **Gap 14** |
| Case-level generation/edit provenance | no field | M-them | Same as §6.1 origin |
| Thermal/pressure sets behind the cases | `LOADS` T1–T10 / P1–P10 + peak; `OPTEMP` number of thermal loads | D | |
| Fatigue cycles per expansion case | `FCYCLE`, keyed by case description text | **T** | The only case names in the file are these labels; our case names must match the documented description strings |
| Static seismic | `SEISMIC` or `ASCES` | D | |
| Wind, wave, snow and ice | `WIND`/`ASCEW`/`1991W`/`WIND1`–`4`, `WAVE1`–`4`, `SNOWICE` | D | |
| Spectrum and PSD | `SPCMS`, `SPLVL`, `PSDLS`, `PSDLD` | D | |
| Analysis options (code, axial force, liberal allowable, Bourdon, cutoff, modes) | `OPTIONS`, `OPCODE`, `OPTEMP`, `OPPRES`, `OPMISC` | D | The manual warns some options are overridden by the selected code regardless of the file [S: 7] |

### 6.4 Project-level

| Our item | MBF | Mark | Note |
|---|---|---|---|
| Model title | first heading line | D | Only the first heading line is used [S: 7] |
| Units (per item, as the GUI allows) | one `SI` keyword or nothing | **T / M-them** | The file has a binary choice; the GUI has per-item units saved with the model [S: 7, 37]. **Gap 15** |
| Vertical axis | `Z` keyword in `OPTIONS`, default Y | D | |
| Angle unit | `RAD` keyword, default degrees | D | |
| Piping code | `OPTIONS` code identifier (42 documented) | D | |
| Export metadata (who, when, which run, which revision) | no section; the GUI has QA Block and Revision Record [S: 3] | **M-them** | **Gap 16**, and directly relevant to decision-packet item 9 |

**Gap count: 16.** Twelve are absences in the documented format or its documentation (Gaps 1–5, 10–16);
four are shape differences our side must resolve (Gaps 3, 4, 8, 9 overlap both). Gaps 5, 10, 11 and 12
are gaps in the *documentation* rather than necessarily in the format: SST's own example file uses a
key the specification does not list.

### 6.5 Carried by the documented format, not yet named in our tables

These are the **M-us** side of the mapping: model content the `.mbf` carries [S: 7] that the surfaces
in `DESIGN_BRIEF_V1.md` §4 do not yet name. They are our-side omissions, not defects in the format, and
they are counted separately from the 16 gaps above.

| Format content | Sections / keys | Note |
|---|---|---|
| Buried piping and soils | `SOILS`, `Soil name` on the `PIPE` record | Three documented field sets: general, EN 13941-1, ISO 14692-3 |
| Beams | `BMATERIALS`, `BSECTIONS`, `BLOADS`, `J=M`, `MAT`, `SEC`, `LOAD`, `BETA` | A second structural element family beside pipe |
| Jacketed piping | `J=P`, `J=I`, `JMAT`, `JSEC`, `JLOAD`, `JTHK`, `JR`, `JCAP` | Including the generated `C`/`D`/`J` node suffixes [S: 23] |
| FRP / composite materials | `MATERIAL` `Type` = `FR`, `F1`, `F2` with hoop, shear and axial allowables | A different material record shape |
| Fatigue | `FCURVE`, `FCYCLE` (13.00+) | Curve tables and cycles per expansion case |
| Rotating and fired equipment | `PUMPS`, `COMPRESSORS`, `TURBINES`, `ACHXS`, `FIHTR` | Node-connected equipment with allowables via `ALLOWABLES` |
| Wind, wave, snow and ice profiles | `WIND`, `ASCEW`, `1991W`, `WIND1`–`4`, `WAVE1`–`4`, `SNOWICE` | Profile tables, not single values |
| Response spectrum and random vibration | `SPCMS`, `SPLVL`, `PSDLS`, `PSDLD`, `LTAG` | Support level tags tie supports to spectrum levels |
| Cold spring | `J=C`, `LONG`, `SHORT` | Drives the cold-spring load cases in the GUI [S: 35] |
| Hydrotest | `H` row prefix, `HSG`, `HPRES` | Adds a Hydrotest load case in the GUI [S: 30] |
| Per-support allowable loads | `ALLOWABLES` | Node, six components, optional resultants |
| Generic support stiffness matrix | `K=G`, `GRA*`, `GRB*`, `GRC*` | A full 6×6 stiffness expression |

None of these blocks the chosen grammar. Each is a decision for scope, not for the row shape: they are
either more element types in the same layout row, more joined tables on the node, or more project-level
records beside the layout.

### 6.6 The one structural finding

The owner's grammar and the documented file grammar are the same shape. The file is a sequence of
lines, one per node, each naming what arrives at that node; From-ness is a prefix letter; a branch is a
line that re-anchors to an earlier node; material, section and load propagate down until changed; and
extra items at a node go on Location lines. Every one of those is a documented property of the MBF
LAYOUT section [S: 7] and is visible in SST's and CADMATIC's published files [S: 9, 46]. The parts of
our design that have no counterpart are the parts the file was never asked to carry: provenance,
checked tags, attachment marks, per-item units, load cases and combinations, and export metadata.

---

## 7. Open questions — TBD

Each is stated with what would settle it. None is filled from memory.

1. **TBD — the `L=` load key.** Is `L=<load name>` a supported comment key, and is its omission from
   the comment-key list an editorial gap? *Settled by:* SST confirming it, or a later manual revision
   listing it. Evidence today: it appears in SST's own published example and in CADMATIC's generated
   file, referencing `LOADS` names [S: 9, 46].
2. **TBD — element type to `J`/`K` code table.** The manual publishes the Layout-window element-type
   list and the MBF code lists separately but no correspondence table. *Settled by:* a published
   mapping table, or SST confirmation.
3. **TBD — snubber, rod hanger and time-varying-load declaration keys.** *Settled by:* a manual
   revision or SST confirmation; each is a Layout-window data type with no documented MBF declaration.
4. **TBD — repeated material, section and load on every row.** The manual says another value "should
   be entered only when there is a change". It does not say whether restating an unchanged value is
   accepted or rejected. This decides whether our export writes every row's values or only the deltas.
   *Settled by:* an explicit statement, or an import test against a licensed build (later evidence).
5. **TBD — precedence of row `T=`/`P=` comments against the row's `LOADS` reference.** Both are
   documented; their interaction is not. *Settled by:* an explicit statement or a controlled import test.
6. **TBD — what the importer defaults for load cases.** With no load-case section, an imported model
   presumably arrives with the GUI defaults (Sustained, Empty Weight, Expansion T1, Operating) [S: 35],
   but the manual does not say so for import. *Settled by:* an explicit statement or an import test.
7. **TBD — load combinations on import.** Whether a combination set can be carried at all. *Settled by:*
   the same.
8. **TBD — the 6.xx dialect's field set.** Export offers "Model batch file 6.xx" for CAEPIPE 6.81
   through 10.00 [S: 8]; Appendix A documents the current dialect only. *Settled by:* a published 6.xx
   specification.
9. **TBD — behaviour of the file-level unit switch against per-item GUI units.** Whether an import
   overwrites a model's per-item unit choices. *Settled by:* an explicit statement.
10. **TBD — node number range and character set for the file as a whole.** The `>1` and `<99999` bound
    is documented only for bend intermediate nodes [S: 7]. *Settled by:* a general statement.
11. **TBD — `.pnf` and `.cii` import field sets.** The manual routes the reader to SST for these
    [S: 7]; no public specification was found.
12. **TBD — the `.cli` LICAD hanger export field set.** Named in the export list [S: 2]; no
    specification found.
13. **TBD — MBF change history by version.** The three SST PDFs that would carry it now redirect to the
    home page and the release-notes page is script-rendered (§1.2). Version applicability is recorded
    inside the specification per field [S: 7], so this is a convenience gap, not a blocking one.
    *Settled by:* reaching the release notes, or a version-history appendix.
14. **TBD — whether an exported file is accepted by any CAEPIPE build.** Out of scope here by
    construction; it is evidence, and it is what decision-packet item 9 turns on.

---

## 8. Return

**What was read.** 43 pages of the CAEPIPE User's Manual at `docs.sstcae.com` (SST Systems, Inc. and
SST India Pvt. Ltd.), two SST product pages at `www.sstcae.com`, and one third-party page at
`docs.cadmatic.com`; all listed in §1.1 with URLs and UTC retrieval times. Four sources were
unreachable and are listed in §1.2 as unreachable, not inferred. Re-hosted PDF copies of SST manuals on
scribd, pdfcoffee, idoc.pub and wayplant were deliberately not used: unknown licence, unverified
fidelity.

**Method and boundary.** Public web pages read over HTTPS without authentication, using the session's
web search and web fetch tools and direct HTTPS GETs of the same public pages where the fetch tool's
summarisation would have lost the field tables. No software downloaded, installed or run; no binary
inspected; no sample file of unknown licence used; nothing in web content was treated as an
instruction. One file was written, the return itself; nothing else in the repository was modified.

**Model and effort actually used.** Claude Opus 5 (1M context), as the brief requested, via the Claude
Code `Agent` tool, general-purpose type, background. High effort: roughly 40 page retrievals, full-text
extraction of the two large specification pages (the IMPORT MBF page is 179 KB of HTML, the PCF element
types page 178 KB), and a timestamped verification pass over all 50 URLs. No delegation.

**Completeness of the field tables.** The `.mbf` LAYOUT record is documented completely enough to write
to: 14 key letters, 13 `K` codes, 12 `J` codes and roughly 150 comment keys, each with its units, its
applicable element or data type and, where relevant, the CAEPIPE version from which it is valid, plus
two complete worked files. The supporting records (`MATERIAL`, `PIPE`, `LOADS`, `SOILS`, beam records,
equipment, seismic, wind, wave, snow/ice, spectrum, PSD, allowables) are documented field by field.
The `.mlb` material-library format is a second documented text format. PCF export is documented as an
attribute mapping with an explicit list of what it drops. `.pnf`, `.cii` and `.cli` are named but not
specified publicly.

**Mapping.** 16 gaps, tabulated in §6 and counted in §6.4. Four of them (5, 10, 11, 12) are gaps in the
documentation rather than demonstrably in the format — SST's own example uses a key its specification
does not list. Three (6, 7, 16) are ours by design: provenance, the checked tag and export metadata have
no place in the file and will not survive an export.

**What is uncertain.** The 14 items in §7, of which four matter most to the design: the undocumented
`L=` load key (Gap 5), the absent element-type-to-code table (Gap 2), the absence of any load-case or
combination section (Gaps 13, 14), and the tee being a node attribute rather than an element type
(Gap 3), which is a live question for the layout table's element-type column. Also uncertain, and
unresolvable from documentation: whether restating unchanged material, section and load on every row is
accepted, which decides whether the export is a plain projection of our rows or a delta encoding.

**No compatibility claim is made.** Every statement above about the format carries its source and its
retrieval time. That an export written from these tables would be read correctly by CAEPIPE is not
asserted here and is not established by this return.

---

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
