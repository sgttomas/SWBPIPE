# CAEPIPE correlation pilot

This Design governs the first external-prover correlation loop (PRD §22.5): SWBPIPE
models are written as CAEPIPE input, solved by the owner's CAEPIPE installation, and
CAEPIPE's results are kept as private reference data and compared with SWBPIPE and an
independent hand calculation. It records how DEL-17-05's open placeholders and its
invocation `TBD` are settled for the pilot, and what stays open.

## 1. Activation and purpose

PRD §22.5 makes activation "owner-gated on lawful access to the external tool". The
owner activated it on 2026-10-26:

<!-- owner-words -->
- Request: "Could we start making validation cases using a CAEPIPE solver in my
  Parallels Windows virtual desktop and have validation cases made as input files that
  can be given to CAEPIPE and solved and then take the outputs as the validation data
  for SWBPIPE to later use?"
- Rulings on Agent 0's questions: "1. Start that 5th lane now 2. Yes the license I just
  read allows such use and I take full responsibility. 3. Automate as much as possible,
  even the first time if that could be done right. 4. Make the shared folder in a place
  you recommend so that it's going to work well for our needs." And: "I have just
  installed an evaluation version of CAEPIPE and it is currently running through
  Parallels."
- Purpose of the pilot: "The pilot using the trial version of CAEPIPE is just to prove
  the basic concept for how I want to do validation will work. Once that's assured, I
  will pay for the full license allowing us to do more extensive validation work."
<!-- /owner-words -->

The pilot is a proof of concept supporting that licence decision. It exercises each
part of the loop once on a small case set; breadth of coverage and governed
tolerances wait for the full licence. CAEPIPE results are a reference, not ground
truth: the hand-calculation witnesses remain the primary verification anchor, and
neither CAEPIPE agreement nor a completed run is professional acceptance, validation
of engineering use, or compatibility proof (DEL-17-05-REQ-008, F-PIP-3).

## 2. Target tool and environment

| Item | Pilot value |
|---|---|
| Target | CAEPIPE 15.10 (PRD §22.5 names the target tool and version), run under the owner's licence; MBF `VERSION 15.10` per the public import page |
| Environment | The owner's Windows virtual machine under Parallels on the development Mac; commands run as the logged-in Windows user (`prlctl exec <vm> --current-user`) |

The pilot cases are small. The writer reports the layout row count against a
configurable threshold and warns above it, but does not refuse.

## 3. Invocation profile (settles DEL-17-05-REQ-003 for the pilot)

The project selects the documented batch pattern (`CAEPIPE-BATCH`, located in DEL-17-01 `Source_Basis_Register.md`):

```text
<CAEPIPE executable> "<model>.mbf";batchmode=yes
```

It imports the MBF, writes `<model>.mod` and `<model>.res`, and is documented to write
`<model>.csv` beside the MBF. The harness always runs this route first and uses its CSV
whenever one appears.

**Fallback when the batch route yields no result file.** The guest script then stops
the process it started, opens the solved results in a fresh CAEPIPE instance that it
starts itself, and prints the results to CSV through the documented print-to-file
command (`CAEPIPE-PRINT-TO-FILE`), finding menu items and buttons by their visible
captions. It reads no binary result file and stops only processes it started. The run
record states which route produced the CSV (`csv_from_batch` or `csv_from_gui_print`).

**MBF export of the solved model (two-way iteration, DEL-17-04 Design §18.3).** A
separate guest step writes the solved model back to MBF. The guest script, run with
`-ExportMod`, opens a guest copy of `caepipe_out/model.mod` in a CAEPIPE instance it
starts. A fresh instance starts either with the layout window of a start-up model or
with a frame window without one; both offer File > Open.... The script waits for the
layout window whose title names the opened model, then runs File > Export to MBF.... In
the "Export Model to Batch File" dialog it selects the type "Model batch file (*.mbf)",
sets the file name (control 1152) and commits (control 1). It waits for the file to
stop growing, then stops the instance it started. It reports the SHA-256 of the model it
opened and of the MBF it wrote. The step uses the stored solve rather than a fresh round
trip: the run record already binds `model.mod` to the current MBF, and the export only
reads that model. `run_pilot --export-mbf` runs the step after each case's live round
trip. `--export-only` runs it alone from each case's stored solve and needs no SWBPIPE
runner. It writes the case's `caepipe_out/` files (the export, its guest logs and the
run record's export entry) and the harness copy of the guest script and run lock in
`_harness/`, and creates missing layer folders. When the stored solve is no longer bound,
any earlier export is removed and the run record's export entry states why. The step's route is `mbf_exported` or the
reason none was written: `not_bound`, `prepare_failed`, `timeout`, `no_guest_json`,
`input_hash_mismatch`, `export_failed`, `mbf_not_written`, `mbf_not_collected` or
`export_hash_mismatch`.

**Authored cases (DEL-17-04 Design §18.3).** `run_pilot --authored [--case ID …]` runs the
authored cases of `validation/caepipe_correlation/authored/` instead of the writer cases: invented,
hand-written MBF files listed in its `manifest.json`. Per case the live round trip above takes the
authored file as written in place of the writer's render, and the export step follows it.
`--authored --export-only` runs only the export step from the stored solve. No SWBPIPE solve, CSV
comparison or series step runs, and no runner is needed. A round trip whose guest did not report
importing the authored file (both hashes) stops the case before the export. No new harness or guest
mode is needed: the harness already takes the MBF text it is given, and the export reads the model
the batch route wrote.

## 4. Configuration surface (DEL-17-05-PH-001)

The harness is disabled unless the user configures it (REQ-001, REQ-016):

- `SWBPIPE_CAEPIPE_EXCHANGE` names the private exchange folder;
- that folder holds `harness_config.json` with the virtual machine name, the guest path
  of the user's CAEPIPE executable, the guest path of the shared exchange folder, a
  guest-local work folder, and `license_and_environment_responsibility_acknowledged:
  true`.

Without both, every live step skips with a stated reason, and public CI never has
either. The harness refuses an exchange folder that lies inside a Git work tree or
lacks a `.gitignore` of `*`. Because the guest paths enter guest shell commands, each
must be an absolute Windows path (`X:\…`) of letters, digits, space and `\ . - _ ( )`
only, without `.` or `..` segments; any other value skips the harness naming the
field. No executable is discovered, bundled or installed.

## 5. Exchange folder and run directory (DEL-17-05-PH-002)

The exchange folder is outside every repository, mode 700, with a `.gitignore` of `*`.
The owner's VM sees it as a shared folder. Per case:

```text
cases/<case_id>/
  case_manifest.json         case id, model and MBF hashes, tool and version, trial first-launch date, route used, per-layer privacy
  swbpipe/                   solve request and SWBPIPE result envelope
  caepipe_in/                model.mbf, id_map.json, mbf_render.json (diagnostics, loss report, mapping assumptions)
  caepipe_out/               CSV, guest run log, .mod/.res as CAEPIPE wrote them        PRIVATE
  caepipe_out/exported.mbf   CAEPIPE's MBF export of the solved model (DEL-17-04)        PRIVATE
  imported/                  import outcome of exported.mbf, derived from CAEPIPE output PRIVATE
  parsed/                    normalized CAEPIPE results                                  PRIVATE
  comparison/                three-way comparison record                                 PRIVATE
series/<hypothesis_id>.json  hypothesis series evaluation (§7)                          PRIVATE
```

An authored case (§3; DEL-17-04 Design §18.3) uses the same folder under its own id
(`cases/cpa-nn-…/`). Its `caepipe_in/model.mbf` is the tracked authored file, written as is; it
has no `id_map.json` or `mbf_render.json`, and its `swbpipe/`, `parsed/` and `comparison/` layers
stay empty. Its `caepipe_out/` and `imported/` layers are as above and equally private.

`caepipe_in/` holds only SWBPIPE-derived content or the invented authored file. Every layer from `caepipe_out/` on
is CAEPIPE output or derived from it and stays private (§8). Every directory and
file the pilot creates, writes or deletes must lie inside the resolved exchange
folder with no symbolic link on the way; otherwise the case stops as
`containment_violation`. Live runs solve on a guest-local copy of the case folder
and copy the outputs back. A failed preparation step (the guest copy of the MBF)
stops the run as `prepare_failed` before solving. The guest script reports the
SHA-256 of the MBF it solved; the run record holds it beside the host MBF's hash, and
when they differ the run is `input_hash_mismatch` and nothing is collected. A CSV is
compared only when its run record binds it to the current MBF (both hashes equal);
an offline re-run reports `stale` when the solved MBF differs from the current one
and `unbound_output` when no valid run record binds the CSV. The run record also holds
the SHA-256 of each collected output.

The export step (§3) runs only when the run record binds the solve to the current MBF
(both hashes) and lists `model.mod`, with its recorded hash equal to the file's where
the record has one; otherwise it is `not_bound` and nothing runs. It collects
`caepipe_out/exported.mbf` with the same containment and link refusal as the CSV, and
only when the guest reports opening a model whose hash equals the stored one. It keeps
the file only when its hash equals the one the guest reported. The run record's
`mbf_export` entry records the route, the solved-MBF, model and exported-MBF hashes,
the steps and the guest log. The step's guest output goes to `export_guest_log.json`,
`export_guest_stdout.txt` and `export_guest_stderr.txt`. Re-running the step replaces
these files and the entry. A new solve deletes them, because the export no longer
describes the solved model. Each guest step is
idempotent and is retried when the `prlctl` client itself fails (exit 255 with a
`PrlJob_` message), which happens intermittently on this host. Each prepare step brackets its
folder check, `(if not exist <work> mkdir <work>) & del … & copy …`: in `cmd` an unbracketed `if`
takes the rest of the line as its body, so a re-run whose work folder already existed would delete
and copy nothing and leave the previous guest copy to be solved or exported. The hash checks above
refuse such a run (`input_hash_mismatch`); the bracket makes the re-run replace the copy.

## 6. Writer, parser and comparison

- **MBF writer (DEL-17-04).** `core/handoff/caepipe_mbf/si_writer.py` writes SI MBF
  (`OPTIONS SI`, lengths in mm, E in N/mm², weight density in kgf/m³) from the headless
  runner's preview model. A project or quantity in another unit system is refused by
  name, not converted. Unsupported constructs are refused by name and listed in the
  loss report. Rotational partial restraints are refused because CAEPIPE's MBF
  import does not accept `FIXR` records (observed format behaviour); translational
  `FIXD` is written. A
  curved bend is written at its tangent intersection (`KI`, radius `B`, the user
  flexibility factor as `FF`); CAEPIPE generates the bend's near and far end nodes
  with `A` and `B` suffixes on the bend node (technical reference, "Bend",
  <https://docs.sstcae.com/tech_manual/bend.htm>), which the writer's node map uses for SWBPIPE's
  tangent-point nodes. A bend without a user flexibility factor is refused. SWBPIPE's
  curved bend includes arc bending and axial flexibility but no transverse shear.
  Weight per length maps to a weight density using standard gravity
  (9.80665 m/s²). The user's manual documents a configurable gravitational
  acceleration but no default value, so no difference is declared for it; the
  default is asked of SST (CQ-17-01-010). Poisson's ratio is taken as given or
  derived as E/(2G) − 1.
- **Parser (PH-003).** `core/handoff/caepipe_external/print_csv.py` reads the CSV
  written by the documented print-to-file command (`CAEPIPE-PRINT-TO-FILE`).

  Its coverage register lists each recognized section (global
  displacements, anchor loads in global and local axes, restraint loads, element
  forces in local and global axes), checks each section's header and unit rows, treats
  a blank numeric cell as zero, skips Min/Max blocks and unknown sections with a
  diagnostic, and pairs element-force rows in from/to order (a weak, row-order
  correlation per REQ-010, bound back to SWBPIPE ids through the writer's id map).

  The section titles, header and unit rows, the blank-as-zero rule and the row order
  are observed format behaviour of the CSV CAEPIPE writes; the public documentation
  does not list them. The public test fixture is invented and labelled.
- **Comparison record.** Each case produces one record naming what PRD §22.5 lists:
  internal model hash, exported MBF hash, target tool and version, import/export
  assumptions, manual changes (none), load-case mapping, support mapping, comparison
  basis, tolerance basis and discrepancy disposition. Each compared quantity carries
  three values (analytic, SWBPIPE, CAEPIPE) and three pairwise checks. A quantity
  covered by a declared difference that gives an independent alternative analytic
  (§7.1) also carries that value and a fourth check, `alternative_analytic_vs_caepipe`,
  at the print-resolution tolerance (§7). The record lists the status of each
  hypothesis series its declarations name.
  - Outcome: `matched_within_tolerance`, `delta_exceeds_tolerance`,
    `row_unmatched_internal_only`, `row_unmatched_external_only`, `tolerance_tbd`.
  - Cause hypothesis for a non-matching outcome (an `ASSUMPTION` until evidenced):
    `solver_defect_candidate`, `export_mapping_issue`, `parser_mapping_issue`,
    `unit_or_axis_mismatch`, `support_semantics_mismatch`, `load_case_mismatch`,
    `missing_model_parameter`, `documented_behavior_difference`,
    `caepipe_profile_unknown`. It is derived from which pairs disagree: if the analytic
    and SWBPIPE values agree and CAEPIPE differs, the CAEPIPE side (export mapping,
    profile or a documented behaviour difference the case declares) is suspected; if
    SWBPIPE alone differs, `solver_defect_candidate`; if the analytic value alone
    differs, the case inputs are suspected; if all three disagree,
    `solver_defect_candidate` with the alternatives listed.
  - Declared behaviour differences. A case may declare a difference for named
    quantities: its hypothesis id, an independent alternative analytic value per
    quantity (computed in `hand_calcs.py`, the one source of the values) and a
    `source_basis` list of public references. A covered quantity's cause is
    `documented_behavior_difference` with `evidence_status: "established"` and
    disposition `assumption_recorded` only when (a) the analytic and SWBPIPE values
    agree, (b) SWBPIPE and the external prover disagree, (c) `alternative_analytic_vs_caepipe`
    matched, (d) the declaration's `source_basis` is not empty and (e) its
    hypothesis series is `consistent`. When (a) and (b) hold and any of (c)–(e)
    does not, the cause keeps the label with `evidence_status: "hypothesis"`, a
    `missing` list naming each failed or unevaluated condition, and disposition
    `requires_professional_review`. When the analytic check fails or has no
    verdict, the triangle rule names the cause and the declaration is listed only
    as an alternative; a deliberate fault injection likewise keeps its cause. No
    axis is silently defaulted.
  - Disposition: `no_action`, `assumption_recorded`, `bug_report`,
    `dossier_question`, `proposed_code_fix`, `requires_professional_review`.

The technical reference documents that sustained-category forces and moments also act
in the operating cases ("Force", <https://docs.sstcae.com/tech_manual/force.htm>).

Sign and unit mappings are applied before comparison. The target tool's support loads
and SWBPIPE's reactions differ in sign, rotations arrive in degrees, and the mechanical
case appears in more than one of CAEPIPE's load cases (observed format behaviour). The
pilot therefore compares the target tool's sustained case, with empty pipes and zero
pressure.

## 7. Cases and tolerances

The public case definitions live in `validation/caepipe_correlation/cases/`: invented
inputs, the SWBPIPE model, the hand-calculation formulas and values, the quantities
compared, and per-case tolerances labelled `ASSUMPTION`. The governed external-oracle
tolerance class stays `TBD` until the owner rules values; tolerances are never
loosened, nor SWBPIPE tuned, to match CAEPIPE. The pilot cases cover, once each, a
force load, thermal expansion, self-weight, a bend, a branch and an imposed
displacement on a support, plus one case with a deliberately injected input fault to
show that a real difference is caught and classified, and two span variants of the
cantilever for the slenderness series.

Each case carries three tolerances, all `ASSUMPTION`:

| Tolerance | Used by | Value |
|---|---|---|
| `analytic_vs_swbpipe` | analytic–SWBPIPE | rel 1e-6; abs 1e-9 m, 1e-9 rad, 1e-6 N, 1e-6 N·m (binary64 round-off scale) |
| `external` | SWBPIPE–prover, analytic–prover | rel 0.5 %; abs 2e-6 m, 2e-6 rad, 1 N, 1 N·m |
| `caepipe_print_resolution` | alternative analytic–prover | rel 1e-6; abs half a printed unit: 0.5 N, 0.5 N·m, 5e-7 m, 8.7266e-7 rad |

The printed resolution of CAEPIPE's CSV (observed format behaviour) sets the
`caepipe_print_resolution` values: half a printed unit is the tightest tolerance the
printed values can resolve.

**Hypothesis series.** A series in `validation/caepipe_correlation/series/` tests the
scaling a hypothesis predicts across member cases that differ in one parameter.
`shear-deformation.json` uses the tip deflection `n1_uy` of the cantilever at spans
3 m (`cpc-07-cantilever-span-3m`), 6 m (`cpc-01-cantilever-force`) and 12 m
(`cpc-08-cantilever-span-12m`). Shear deformation predicts a relative gap
(C − S)/S = 3EI/(κGAL²), positive and ∝ (r/L)² for any shape factor, so the log-log
slope of the gap against span between the shortest and longest member must be −2
within ±0.05 (`ASSUMPTION`). That is resolvable at print resolution: with the
shape-factor-2 gaps of 0.043 mm at 3 m and 0.173 mm at 12 m, rounding of the printed
external-prover value moves the slope by at most about 0.011 (about 0.021 at shape factor 1).
A member counts only when its record is bound to its current MBF, its
analytic–SWBPIPE check matched and its gap exceeds the print resolution; otherwise
the series is `not_evaluated` with the reasons. With every member counted it is
`consistent` when every relative gap is positive and the slope is within tolerance,
else `inconsistent`. After the selected cases, the pilot command evaluates every
series from the stored member records, writes the result privately to
`series/<hypothesis_id>.json` in the exchange folder, and rebuilds offline, from the
stored CSVs, each case whose declarations name that hypothesis and whose record
carries a different series status. Standard output names only the files written.

### 7.1 Shear deformation evidence

SWBPIPE's frame preview is Euler–Bernoulli. CAEPIPE's technical reference, page
"Fiber Reinforced Plastic Piping (FRP)", section "Stiffness matrix"
(<https://docs.sstcae.com/tech_manual/fiber_reinforced_plastic_piping__frp_.htm>),
describes a pipe stiffness matrix formed from axial (L/EA), transverse shear (a
shape factor times L/GA), bending (L/EI) and torsion (L/2GI) flexibility terms. It
publishes no shape-factor value, and the page sits in the FRP chapter, so its use
for the steel pipe element is an inference. The alternative analytic for the
`shear-deformation` hypothesis is the Timoshenko value: the Euler–Bernoulli witness
plus that shear term, with shape factor 2 (κ = 0.5, the thin-wall circular tube
value), an `ASSUMPTION`. The shape factor CAEPIPE uses is a question for SST
(DEL-17-01 CQ-17-01-010). The user's manual page "Analysis", section "Gravitational
Acceleration and Direction" (<https://docs.sstcae.com/users_manual/analysis.htm>),
documents a configurable gravitational acceleration introduced in 15.10, with
weights entered in earth-gravity units, but gives no default; the same question
asks SST for it. The difference is classified as documented only through the
rule of §6: the alternative analytic must match the external prover at print resolution and the
series must be consistent.

The private run records the residual of every alternative-analytic check. Where a
residual exceeds print resolution although SWBPIPE and the external prover match within
the external tolerance, no cause is assigned, the residual stays in the private record,
and the shape factor is not refitted to absorb it.

A difference indicating a wrong SWBPIPE output goes to Agent 0 with its evidence and
is not fixed in this deliverable.

## 8. Data boundary

<!-- policy-text -->
The owner's rule (2026-10-26): <!-- owner-words -->"I want: **interoperate openly, evaluate privately**".<!-- /owner-words -->
Public text (the repository, PRs and commits) may name CAEPIPE and state the file-format
and interface facts the code needs to interoperate: record syntax, CSV section layout,
labels, sign and unit conventions, and how the importer and exporter map records. A fact
the public manual covers is cited to it (DEL-17-01 `Source_Basis_Register.md`); any other
is stated plainly as observed format behaviour. Code may contain whatever it needs.
Anything that evaluates CAEPIPE stays in the exchange folder: result values, differences,
agreement counts and classifications, tolerances measured from results; its defects and
crashes, the trial's limits, the evaluation stamp, its performance; screenshots and
survey notes. The lane-local check `tools/caepipe_correlation/check_public_observations.py`
screens added public text for those kinds of content. A lexical check is only a backstop, so
every pull request in this lane that adds public text also gets an independent content pass for
evaluation material on its final diff (Agent 0, 2026-10-27); that review is the control.
<!-- /policy-text -->

CAEPIPE's output files and the values parsed or derived from them (results,
differences, agreement counts and classifications, tolerances measured from them) stay in
the exchange folder and are never committed, pushed, or written into PR, issue or other
GitHub text. The format and interface facts the code needs, including those learned
from CAEPIPE's outputs, are public as stated above. Public material is otherwise limited
to invented models, SWBPIPE-generated MBF, code, this Design, and invented parser
fixtures. Vendor files in the CAEPIPE install folder are read locally
only and never copied. No public text claims agreement with CAEPIPE. The pilot
command's standard output carries operational status only (case, route, the record's
path in the exchange folder or the reason none was written); outcome and cause counts
stay in the private record. A guard test checks that no tracked file sits under an
exchange-style path, carries the CAEPIPE CSV header of a real run, or is a parsed,
comparison or run-record artefact (by document kind or private-layer file name)
unless a provenance sidecar declares it invented, and that the harness refuses an
exchange folder inside a Git work tree.

## 9. Regression check

`tests/test_caepipe_private_correlation.py` reads the private corpus through
`SWBPIPE_CAEPIPE_EXCHANGE`. When it is unset or empty the test skips with that reason;
otherwise it re-parses each stored CAEPIPE CSV, re-solves the case in SWBPIPE,
re-runs the comparison and the series step (§7) in a temporary copy, failing if any
quantity's outcome, cause or cause evidence status moves out of its recorded
classification. It never prints external-prover values outside the local run.

## 10. Remaining items

- A governed tolerance class for external-oracle comparisons (owner ruling).
- Writer breadth: springs, guides, hangers, nonlinear supports, pressure, rigid and
  valve components, temperature-dependent materials, imperial-unit projects
  (conversion), multiple thermal cases.
- The desktop `.mbf` export (`CaepipeMbfExportPanel.tsx`) still writes the placeholder
  smoke text; it should use the SI writer's record subset.
- A wider correlation corpus (larger models, more load types) under the full licence.
- Non-unit bend flexibility factors are untested: the bend case uses k = 1.
- CAEPIPE's pipe shear shape factor and default gravitational acceleration
  (CQ-17-01-010); κ = 0.5 stays an `ASSUMPTION` until answered.
- Two-way iteration: the export step (§3) writes `caepipe_out/exported.mbf` for
  DEL-17-04's private import regression (DEL-17-04 `Design/caepipe-mbf-import.md` §18.3),
  for the writer cases and for the authored cases (§3, §5), whose outcomes are recorded
  privately in `imported/`. The authored set grows with each importer tranche: each new
  mapping or named refusal gets an authored file.
