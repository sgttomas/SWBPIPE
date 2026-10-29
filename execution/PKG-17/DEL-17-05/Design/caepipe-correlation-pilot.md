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

The batch print's extension follows the print-file extension saved in the tool's preferences,
so the same print may be written as `<model>.txt`. The harness collects either and records the
name collected (`print_file`). `<model>.csv` is taken as named; `<model>.txt` is accepted only
when the print-CSV parser (§6) recognises its header, parses at least one section and reports no
error, and is otherwise refused as `print_file_unrecognised` and not read as results. When both
files are written, neither is taken (`print_file_ambiguous`) and no fallback runs. The saved
extension itself is never changed.

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
`run_pilot --authored --authored-dir <path>` takes the authored cases from another directory with
the same `manifest.json` layout instead of the tracked one, so authored files that are themselves
private are run without entering the repository. The directory must lie inside the exchange folder
with no symbolic link on the way (else the run stops before any case), and its case ids must not
name a tracked writer or authored case, whose case folders the run would otherwise replace. Such a run
wants only the solve and its printed results, so it never runs the export step; `--no-export` leaves
the export out of a tracked authored run too.

**Saved settings.** The tool keeps saved settings in the guest registry, among them a recent-files
list that a solve may update. The configuration may name that registry key (`settings_guard`: `key`,
an `HKCU\…` path; `recent_files_subkey`, the list's subkey; `restore_recent_files`, `false` unless
set, which needs the subkey; `restore_scope`, `"recent_files"` unless set to `"key"`, which enables
restoring by itself and widens it to the whole key; `call_timeout_s`, default 20, and `section_budget_s`, default 120, both in
seconds); `--authored-dir` runs require it. Every guard call (registry reads and writes, process
queries) runs with `call_timeout_s`; the solve keeps its own timeout. Guard reads (process lists,
registry readings, the restore's read-back) have no side effect, so a transport failure of the prlctl
client (exit 255) is retried up to six attempts in all, 2 s apart, every attempt within the call's
`call_timeout_s` (and, in the restore, the section budget); no attempt starts that would pass either
limit, and a read that still fails stops the run as before. Writes keep the single settled retry
described under **Restore**.

- **Precondition.** Before the first case and again before each round trip, the guest's process
  list (`tasklist`) must be readable and show no instance of the tool: none is the run's own yet, so
  any one is another user's instance (`foreign_instance_running`), and an unreadable or malformed list
  is `precondition_unknown`. Either way nothing starts. The check starts, closes and signals nothing.
- **Before.** The whole key, every subkey included, is read (`reg query /s`). The reading keeps every
  key it prints, empty ones included, and every value, and counts only when every line of it is
  understood and every key it prints is the configured key or lies under it; when it fails nothing runs
  (`settings_check_failed`). `reg query /s` prints no header for the queried key while that key holds no
  values of its own (observed on the guest); its printed subkeys show that it exists, so it is recorded
  as present. An output that prints no key at all is no reading.
- **During.** The guest script appends the id of each process it starts to `owned_pids.txt` in the
  case's work folder (and lists them in its report), the GUI print instance included. An
  `--authored-dir` run falls back to printing through the GUI only with `restore_scope` `"key"`, whose
  restore covers the settings that route writes; otherwise a batch run that leaves no CSV ends
  `batch_no_csv`.
- **After**, whatever happened in between (a timeout of the host's call or an error while collecting
  included), in checks guarded separately so that a failure in one skips neither the other nor the
  result:
  - the record of ids must be complete and valid (readable, at least one id, one decimal id per line,
    equal to the report's list when there is one), and the process list valid `tasklist` CSV. A
    process of ours still listed is `owned_process_running`; another instance of the tool, the
    precondition checked again, is `foreign_instance_running`; a record or list that falls short is
    `owned_process_state_unknown`;
  - the whole key is read again. When the two readings are identical (keys and values) the result is
    `unchanged`.
- **Restore.** Only when the configuration enables it, the process checks are clean, and every
  difference lies in the restore scope, the harness writes the scope back to the first reading in one
  routine. With `"recent_files"` the scope is the values of the recent-files subkey (no key added or
  removed, nothing else changed); with `"key"` it is everything strictly inside the configured key: its
  values, and its subkeys at any depth, which may be created or deleted. The configured key itself, its
  parent and anything else are never written; the plan checks every difference against the scope and
  refuses the whole restore, before any write, when one lies outside it (with `"recent_files"` such a
  difference is `settings_changed`). It plans the writes (subkeys that were removed are created,
  shallowest first; values that changed or were removed are set to their earlier type and data, in the
  first reading's order; values that appeared are deleted, except inside a subkey that appeared;
  subkeys that appeared are deleted with their contents, deepest first; a key or value name outside
  the allowed characters, or a value that cannot be written back exactly, stops it before any write)
  and keeps a write log: each value or subkey, what it restores and what it replaces,
  its attempts and its outcome. Each write counts as completed only on exit 0 with the command's success
  message, and is never repeated blindly (the retry the harness applies to reads does not apply to
  writes). A reported failure is logged `failed` and other output `unknown`; either stops the restore
  at that write. A value set, value delete or subkey delete whose outcome a transport failure (the
  client's exit 255, or a timeout) left unknown is settled first: the process list is checked, and the
  value or subkey is read back (a subkey with every value in it). If it is already as intended (the
  value at its restored data; the value or subkey absent, as reg reports a missing key or value), the
  write is completed; if it is still as the run left it, the write is issued once more (attempt 2), at
  most once; any other read-back (other data or content, an unreadable read-back) or another instance
  stops the restore with the write `unknown`. Every attempt and the read-back are logged. Creating a
  subkey is not retried. It checks
  the process list before each write, after the last write,
  and once more before declaring success, reads the key a third time and compares it exactly with the
  first reading. The result is `recent_files_restored` (`settings_restored` with `"key"`) only when every
  step completed and the
  comparison found no difference. Any other outcome, a check that finds another instance or cannot
  tell, or an exception or cancellation at any step, the comparison and the failure path itself
  included, takes one failure path: the key is read once more (best effort), the result is
  `settings_restore_failed` with the write log, so the record shows exactly which writes happened, the
  run record is written, and the first exception, if any, is raised afterwards unchanged with the
  result attached. The first exception is kept from the moment it occurs, before any diagnostic or
  logging; the failure path's bookkeeping, its reading, its comparison, the attachment and the record
  each run under their own guard, so a later exception or cancellation is recorded as secondary and
  neither replaces the first nor skips the reading. Once the restore has been entered, the final
  reading is attempted whether or not anything was logged. A record never says nothing was written
  when its log shows writes. A difference anywhere else is `settings_changed`, and nothing is written.
- **Recorded restore.** With `restore_scope` `"key"`, value differences that an earlier round trip
  recorded but did not restore can be written back to their recorded `before` values through the same
  plan and routine (`restore_recorded_settings`), outside any round trip: only after the precondition is
  clear, the key reads whole, and every recorded value still has its recorded `after` state
  (`changed_since_record` otherwise, nothing written); the third reading is compared with the intended
  one, and each recorded value's status is listed.

**Interruption (threat model).** For the whole guarded round trip, SIGTERM (and SIGBREAK where it
exists) is made catchable on the main thread: it raises a cancellation (`TerminationRequested`, a kind
of `KeyboardInterrupt`) as SIGINT does, instead of ending the process unrecorded; the previous handler
is restored when the round trip ends. Signals are deferred only across the restore's
registry-mutation section: the check before each write, the writes, the check after them and the
verification reading. There SIGINT, SIGTERM and SIGBREAK only record themselves, so no signal can cut
between a check and its write or leave a write half-logged; the previous handlers are restored when
the section closes, and the signals it deferred are reported then (`deferred_signals`) and stop the
run as a `KeyboardInterrupt` raised after the restore's outcome is recorded in memory, or are kept as
a secondary error beside a primary error that is raised instead. **Bound:** the section has a total
budget, `section_budget_s` (120 s by default). Each call in it is given at most the guard call timeout
and never more than the budget still remaining, and no retry starts after the budget has run out; when
it runs out the restore stops as `settings_restore_failed` with its write log and the stop is raised
(`SectionBudgetExceeded`). The worst case of the deferred section is therefore the section budget.
Everything else (the post-run checks, the recovery, the attachment and the record I/O) runs in
normal signal context: a signal there interrupts as usual. The post-run checks (the process query,
the precondition, the readings) are each guarded against any exception, so a cancellation raised by
one is kept (raised as the first error, or kept beside it) and the settings reading still runs; an
ordinary error there leaves the state unknown and stops the run. The guard result, with its write
log, is attached to the error before any record I/O begins and is then written on its own to
`caepipe_out/settings_guard.json` before the run record. That write stays interruptible; a cancellation
during it is retried once and is then raised. So a signal or failure during the record (the first such
failure included, recorded as `run_record_error`) cannot lose the log. The reporting and persistence
steps between the checks and the end of the round trip are covered by an outer recovery as well: an
exception leaving the round trip before its records are done first has the guard result attached (the
round trip's own, else the restore's, else one built from the write log), and that result is then
written to `settings_guard.json` once more, best effort and still interruptible, naming the exception
(`round_trip_error`). Each `TerminationRequested` also carries the guard result as it stands when the
signal arrives, so a second termination request, during that write or anywhere else, still leaves with
the write log. Exceptions raised by callees take
the deferred path above, with the first one raised. An exception raised by one of the guard's own
bookkeeping statements would be a defect in the harness; beyond the tests that inject failures at each
callee and recovery step, that case is not designed for.

**Known limits.**
- A failure inside the recovery's own callbacks (its bookkeeping, final reading, comparison or reason)
  can leave out the diagnostics that step would have added; the failed status and the write log are
  kept.
- If every attachment of the guard result to the raised error fails (the signal handler's, the round
  trip's and the outer recovery's), the result is only in
  `settings_guard.json` and the run record, not on the error.
- Signal deferral, and the catchable SIGTERM, are available only on the main thread; elsewhere the
  record says deferral is unavailable (`signal_deferral`) and signals act as they normally do.
- Whether `reg query /s` prints a subkey that holds no values has not been observed on the guest (every
  subkey read there has values). If it does not, such a subkey added or removed during a round trip is
  not seen; every value change is. With `restore_scope` `"key"` this includes an empty parent left
  behind when a subkey that appeared is deleted.
- Each process check, the session precondition before the first case included, is a guard read: all
  its attempts together take at most `call_timeout_s` (20 s by default). It writes nothing and stays
  interruptible.

The restore is safe only because no other instance of the tool runs during the round trip: the
precondition admits none before it, and the checks inside the restore look again before each write
and before success. A check and the write after it are not atomic: another instance starting in
between would not be seen until the next check, and excluding other writers outright would need a
change to the guest environment, which the harness does not make. That residual window is accepted
because, under the operating agreement, no other instance runs during these sessions. A breach of it
is detected after the fact by the following check or by the comparison, and is recorded as
`settings_restore_failed` with the write log; it is never reported as a successful restore.

The run stops at the first case whose result is anything but `unchanged`, `recent_files_restored` or
`settings_restored`, whose round trip raised (the guard result travels with the exception), or that ended
`batch_no_csv` where the GUI fallback is refused. The guard
result is checked before any other test of the record, so a rejected input hash does not pass over it.

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

  Unit labels: each unit-bearing column must carry a label the layout prints for its
  quantity: length `mm` or `inch`, rotation `deg` or `rad`, force `N` or `lb`, moment
  `Nm` or `ft-lb`, stress (checked, not read) `MPa` or `psi`. A file prints one system
  (SI or English) and one angle unit; a section that mixes them, or departs from the
  sections read before it, is refused (`CAEPIPE-PRINT-UNITS-MIXED`), and any other
  label is refused by name (`CAEPIPE-PRINT-COLUMN-HEADER-MISMATCH`). English and radian
  values are normalized to the output units (mm, deg, N, N·m) with the unit catalog's
  exact definitions (`core/units`: inch 0.0254 m, foot 0.3048 m, pound-force
  4.4482216152605 N, degree π/180 rad), each factor formed exactly and rounded once;
  SI values are read unchanged. A value that is not finite after conversion refuses its
  row (`CAEPIPE-PRINT-VALUE-UNREPRESENTABLE`). Each parsed section records the label every value was
  printed in (`source_units`), and the result records the file's system and angle unit.

  The section titles, header and unit rows (the English and radian labels included),
  the blank-as-zero rule and the row order are observed format behaviour of the CSV
  CAEPIPE writes; the public documentation does not list them. The public test fixture
  and the English and radian test texts are invented and labelled.
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
- The pilot's own bend case uses k = 1. Non-unit factors are exercised only by the benchmark's G2 set (§11.1).
- CAEPIPE's pipe shear shape factor and default gravitational acceleration
  (CQ-17-01-010); κ = 0.5 stays an `ASSUMPTION` until answered.
- Two-way iteration: the export step (§3) writes `caepipe_out/exported.mbf` for
  DEL-17-04's private import regression (DEL-17-04 `Design/caepipe-mbf-import.md` §18.3),
  for the writer cases and for the authored cases (§3, §5), whose outcomes are recorded
  privately in `imported/`. The authored set grows with each importer tranche: each new
  mapping or named refusal gets an authored file.
- Benchmark (§11): tranche 1 has run under the settings guard. Its screening and §3 triage are held
  privately (§8), for the owner's review of the full record. Still open:
  - further VM batches and the later tranches (T2 at scale, then T3–T6 as their constructs land). Each
    starts only on Agent 0's go-ahead;
  - closed forms for G2 (anchored L with k) and G3 (elastic centre);
  - a clean one-parameter R/D series for bend transverse shear;
  - the pressure family;
  - parser sections for frequencies and spectra;
  - declare rigid stiffness as an inexact input (§11.3) for models with a rigid element or line stop,
    so their screening class is `named_inexact`;
  - the VM reruns that the private tranche-1 triage lists: G4-05 and G6-03, as authored and with every
    modulus divided by 100;
  - a possible convention option in SWBPIPE: straight-pipe shear with shape factor 2 (κ = 0.5, the
    assumption above) and bend transverse shear with the same factor. This is a scope call for Agent 0
    or the owner, not started;
  - DEL-17-04 §18.3's private importer round trip on the MBF that benchmark runs export
    (`--export-mbf`).
- Source rows for the remaining component and support families (Q-3, for W-17's coverage criterion).
  One row per family, citing CAEPIPE's public manual page by URL, in our own words, in reference-system
  order. The families are a tee with user SIF modifiers, an eccentric reducer, valves, flanges, an
  expansion joint with tie rods, a gap, lift-off, friction, a variable spring and a constant-effort
  support. A family with no public page becomes a limitation candidate.
- DEL-17-04 importer: the revised O-7 (Agent 0), to be recorded in DEL-17-04 Design §20.2 in place of
  the hot basis. CQ-17-01-016 and CQ-17-01-017 were resolved by experiment, not sent. The change:
  - the modulus basis follows the MBF's cold-modulus flag, absent meaning cold, except under the codes
    whose flag is ignored. For those, map that code's basis and cite it;
  - map the per-case modulus basis explicitly to DEC-092, never by default;
  - take thermal expansion as the mean from the `OPTEMP` reference temperature;
  - map rotational stiffness units per degree, with English-unit moments in inch-pounds;
  - refuse by name an element temperature outside the material table;
  - pin the mapping to version 15.10;
  - close TBD-17-04-010.
- DEL-17-04 writer, `OPPRES`: Agent 0's decision (b). Write the pressure-elongation option for the
  sustained case only when the pressure contract applies `transfers_to_wall` closure, otherwise off. The
  export report names the remaining modelling differences, with tests.
- Publication policy and checker for benchmark agreement (D15). Branch
  `claude/piping-caepipe-publication-20261028` at `bf7a2b9e53` holds the gated design: a structured ruling
  record and a fixed summary rendering. Still open:
  - reconcile `IP_AND_DATA_BOUNDARY.md` §6.2 with D15;
  - fix review r3's findings. A threshold or unit misrepresented, or a partial quote, blocks; the other
    findings are listed known limits;
  - cite licence clause numbers only.

  One owner-reserved choice stays open: AC-001's "implying compatibility" could read "implying general
  compatibility (benchmark agreement may be published only under Design §8.1)". If that wording is not
  approved, publication relies on reading agreement on stated cases as no compatibility claim.

## 11. Benchmark set, tranche 1 (RT-V2)

This section records the method for the benchmark set against CAEPIPE that RT-V2 of
`docs/RELEASE_TARGET_v1.0.md` requires (§2; method in §3). It compares mechanics only:
displacements, rotations, element forces and moments, and support loads. Stresses and code
checks are left out. Tranche 1 covers families G1–G8: linear, Y vertical, temperature-independent
materials, no pressure. The models, the settings, the comparison rule and the code are public.
Every run, result, comparison and classification stays in the exchange folder (§8). Nothing in
this section claims agreement, and nothing in it records, implies or derives acceptance (F-PIP-3).

### 11.1 Models

`tools/caepipe_correlation/benchmark_models.py` writes the invented models to
`validation/caepipe_benchmark/models/` and their `manifest.json` (seed, generator version, family,
sampling, parameters, series, model hash and file hash). The generator draws only `random()` from
`random.Random(seed)`, and its JSON is canonical, so the same seed gives the same bytes. `--check`
compares the tracked files with a fresh generation. The default seed is 20261028.

| Family | Models | Content |
|---|---|---|
| G1 straight spans | 10 | cantilevers (the shear D/t series and two opposite corners), guided cantilever, two- and three-span continuous, fixed–fixed |
| G2 single bends | 16 | anchored L; R/D 1, 1.5, 3, 5 × 45°, 90° × k = 1 and a sampled k of 3–15 |
| G3 Z and U loops | 12 | six of each: two opposite corners and four Latin hypercube points |
| G4 multi-plane | 6 | X leg, riser, Z leg: two opposite corners and four Latin hypercube points |
| G5 branch tees | 6 | the tee-ratio series (0.3, 0.5, 0.75, 1.0) and two vertical branches |
| G6, G7 support mixes | 3 + 3 | risers with guides, a spring and a line stop; runs with restraints, guides, line stops and springs |
| G8 anchor movement | 4 | fixed–fixed translations and a rotation; an L with a general movement |

G1 to G4 take their corner or series points plus a Latin hypercube. G5 to G8 enumerate their
configurations (and G5 its tee series) and sample the rest by the hypercube. Run sections span
OD 60–610 mm and D/t 10–100 (log-uniform); a tee branch is the run OD times its ratio. The three
invented materials are temperature-independent (E 200, 195 and 70 GPa class). Each model has a sustained case (weight
and forces) and an expansion case (a uniform thermal change, or the anchor movement for G8). Every
bend carries a user flexibility factor. Closed-form Euler–Bernoulli values are attached to the
straight members where one exists (G1 cantilevers and fixed–fixed; G8). They are the third,
independent value, and each agrees with SWBPIPE's solve to binary64 round-off (a focused test).
Concentrated forces act only at nodes that carry no support.

Each model names, per case, the inputs that cannot be held exactly equal (§11.3). A case with none
is in the exact class.

### 11.2 Writer settings

The benchmark writes an explicit piping code, which the writer then honours (DEL-17-04):
`render_caepipe_mbf_si(..., piping_code=...)` takes one documented OPTIONS piping-code token
(*Format of .MBF*, OPTIONS) and writes `OPTIONS HGRA,<code>,SI`. For an ASME B31 code it also
writes `OPCODE 0,0,0`: B31J is off (field 3), and fields 1 and 2 are code-stress settings only.
An undocumented token is refused (`MBF-SI-PIPING-CODE-UNSUPPORTED`). The current B31.1 token
`B311` is written with the warning `MBF-SI-PIPING-CODE-B31J-LOCKED`, because the user's manual
states that B31.1 (2020 or later) computes SIFs and flexibility factors by B31J (*Analysis*).

Without a code, the text is unchanged (no code token), byte for byte. The info diagnostic
`MBF-SI-PIPING-CODE-NOT-WRITTEN` then states that the target program's default code applies.

The generator chooses `B311-80` for every model. This is an ASME B31.1 edition that predates
B31J, so OPCODE's B31J off stands. It is not `NONE`, under which a single static case applies every
load together (*Analysis*), and the benchmark never relies on the code the target program would
otherwise take from its saved settings. Pressure correction does not arise, because tranche 1
has no pressure. Gravity is the earth default, with Y vertical, as the writer always writes. A
user-entered k replaces the code's bend flexibility factor (*Bend*). With temperature-independent
materials the code's hot or cold modulus choice has no effect.

### 11.3 Inputs not held exactly

| Input | Where | Carried by |
|---|---|---|
| `shear_deformation`: SWBPIPE's frame route is Euler–Bernoulli, and CAEPIPE's pipe element has a shear term (§7.1) | every case with bending | the `shear-dt` series (G1) and the span series (§7) |
| `bend_flexibility_application`: whether k also applies to torsion is unpublished | bends | the `bend-k-*` pairs (G2) |
| `bend_transverse_shear`: SWBPIPE's curved bend has none | bends | R/D 1–5 across G2 |

Branch flexibility is held equal (B31J off, and SWBPIPE assembles none). The `tee-ratio` series
checks that.

### 11.4 Binding

Results are bound through the writer's id map and never by position:

- **Nodes:** through `node_map`.
- **Element forces:** `bind_element_rows` binds each element-force row pair to the member whose
  `element_map` entry names the same two node labels. Each row belongs to the member end at its
  node, taking `reversed` into account. Pairs in another order or position are bound and noted.
  A pair that names no member or several members, a second pair for a member, and a member with
  no pair are each listed as unmatched.
- **Supports:** through `support_map` by node. SWBPIPE's reactions at one node are summed,
  because the printed table has one row per node. A restraint row prints forces only, so moments
  are compared at anchors alone.

The global element-force rows are the end action on the element at its node, which is the same
sign convention as SWBPIPE's `element_global_end_*_v1` rows (observed format behaviour). The
printed support loads are the pipe's load on the support, so they are sign-flipped (§6).

### 11.5 Two-way comparison and screening

`core/handoff/caepipe_external/benchmark.py` compares SWBPIPE's own results, from the headless
runner, with CAEPIPE's printed results. No analytic value is needed. The compared cases are each
mapped primitive case and the operating case. For the operating case, SWBPIPE's side is the sum
of its primitive cases (superposition, linear models) and CAEPIPE's side is its
`Operating (W+P1+T1)` case.

Each scalar is screened with the RT-V2 §3 **screening profile**. Under the owner's ruling,
Agent 0 screens with it and the owner's review of the full record decides. The values are only
tightened:

`|S − C| ≤ a_k + r · max(|S|, |C|, 10⁻³ · Q_k)`

- r = 1e-4 when every input is held exactly, and 5e-3 when a named inexact input remains;
- a_k = one printed unit: 1e-6 m, 0.0001° (1.745e-6 rad), 1 N, 1 N·m;
- Q_k = the case maximum of kind k.

Each result is classed `within_profile`, `finding` (outside the profile), `unmatched` or
`status_mismatch` (a case solved on one side but not printed on the other). `unmatched` covers:
- a value on one side only;
- a printed row that binds to nothing;
- a SWBPIPE value with a contribution that is missing from one of the summed cases, is not a
  finite number, or sums to a non-finite value. It is never screened on a partial sum;
- each parser error, such as a refused section or a conflicting duplicate row, as one row
  under case `parser`.

No non-finite value is ever `within_profile` (record version 2). The parser skips the fx..mz
local element-force layout only when its unit labels are valid and of one system; otherwise it
refuses the section (parser `caepipe-print-csv-4`). A closed-form value is kept as the third value with two checks:
against SWBPIPE (rel 1e-6, binary64 scale) and against CAEPIPE (the profile). A closed form that
disagrees with SWBPIPE makes the result a finding.

The nonlinear and modal classes of RT-V2 §3 wait for their tranches. The triage order is RT-V2's.

### 11.6 Running

`run_pilot --benchmark <manifest>` checks each model file against the manifest's hashes, then
runs the selected models one at a time (`--case`, and serial batches by `--batch-size` and
`--batch`). For each model it:

1. solves the model in SWBPIPE (headless runner);
2. writes the MBF with the model's piping code;
3. runs the guarded round trip and collects the batch print (`model.csv` or `model.txt`);
4. parses the print and builds the two-way record.

It writes `comparison/benchmark_comparison.json` and the case manifest, which is the model's run
record (outcome, route and attempt), to the model's case folder in the exchange folder.

A live run needs `settings_guard`, `--freeze-file` and `--exclusive-file`. It uses the GUI print
fallback only with `restore_scope` `"key"`, whose restore covers the settings that route writes
(§5). Models are paced `--pace-s` apart, 60 s by default. Before each model, the run:

- stops if the freeze file exists;
- holds, polling every 30 s for up to 30 min, while the exclusive file names a running process;
- asks the guest for `cmd /c ver`, and asks once more 2 min later before it stops;
- checks that no instance of the tool is running (§5).

A model whose round trip raised, or left no CSV, with a clean settings check is retried once,
2 min later. A model that again leaves no CSV is recorded as such and the run goes on. The run
stops at the first settings check that is not clean, at a check before a model that fails, and at
a round trip that raised twice.

`--offline` rebuilds the records from the stored, bound prints, with no wait. Standard output
carries operational status only.
