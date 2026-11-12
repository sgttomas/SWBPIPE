---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-17-05
package_id: PKG-17
decomposition_basis: execution/_Decomposition/SOFTWARE_DECOMP.md
project_scope_refs: [SOW-030, SOW-046, SOW-075]
package_objective_refs: [OBJ-007, OBJ-009, OBJ-017, OBJ-018]
---

# Scope of Work — DEL-17-05 CAEPIPE external run harness and CSV parser

## Purpose and Objective Traceability

This deliverable is the correlation loop with the external prover. An optional harness hands an MBF that SWBPIPE wrote to the user's own licensed CAEPIPE, collects the results CAEPIPE prints, parses them, binds them back to SWBPIPE's entities and compares them with SWBPIPE's own results; the benchmark set applies that loop across invented model families. The result it contributes is that the owner can see, case by case, where SWBPIPE's mechanics agree with CAEPIPE's and where they do not, each difference screened and classed, without the project bundling the tool, publishing what the tool produced, or reading agreement as professional acceptance.

It serves SOW-030, SOW-046 and SOW-075 under OBJ-007, OBJ-009, OBJ-017 and OBJ-018 (`execution/_Decomposition/SOFTWARE_DECOMP.md`, PKG-17, row DEL-17-05: the harness is optional and user-owned), with `docs/PRD.md` §8.9, §16.2 and §22.5, which makes correlation against an external professional solver the principal validation posture. It owns exit criterion 11 of `docs/RELEASE_TARGET_v1.0.md`: every benchmark case is screened against the benchmark method's tolerance profile, the findings are triaged, and the owner has reviewed the full record.

- **OUT-001** — The external-run harness and the result parser: the opt-in harness around the user's executable, the run record and run package (`schemas/caepipe_external_run.schema.json`), and the parser of the CSV that CAEPIPE prints.
- **OUT-002** — The correlation and the benchmark set: the public invented cases and benchmark models, the comparison of SWBPIPE's results with CAEPIPE's, the screening against the benchmark method's profile and the triage of findings.

## Deliverable Definition — Ontology

**Terms the product implements.**

- *User-owned executable.* The user's own licensed CAEPIPE, on a machine the user controls. The project holds no copy of it, does not look for it and does not install it.
- *Exchange folder.* A private folder outside every repository, named by the environment variable `SWBPIPE_CAEPIPE_EXCHANGE`. It holds the harness configuration, in which the user names the executable and acknowledges responsibility for the licence and the execution environment, and every file a run produces. Everything CAEPIPE writes, and everything parsed or derived from it, lives here; nothing of it leaves except benchmark agreement the owner publishes under the data boundary policy.
- *Run record.* The account of one external run of one model in one run state: the configuration checked, the state of the executable path, the acknowledgement, the source model and export identifiers, and, where the model declares runs, the run's series, each line of the written MBF the run substitutes and, for a screened run, the model document SWBPIPE solved, or, for a capture-only run, that no SWBPIPE solve or comparison was made, the MBF's hash and its identity file, the command shape actually used, the working directory and environment, the expected and the observed result file, the exit status, what output was captured, the parser's status and coverage, and the classification of the evidence.
- *Target CSV artifact.* The results file CAEPIPE prints for a run. "Expected" and "observed" are states of the same artifact.
- *Bound result.* A result file is compared only when its run record binds it to the current MBF: the hash of the MBF on the host and the hash the guest reported for the MBF it solved both equal the current MBF's hash.
- *Parser coverage.* The sections of the printed CSV the parser reads. A section is read only after its column-header and unit rows are verified exactly; a unit is never assumed; values are normalized by the unit catalog's exact definitions; an unknown section is skipped with a diagnostic.
- *Binding.* The attachment of a parsed row to a SWBPIPE entity through the MBF writer's identity file: nodes by the node map, element end forces by the member whose map entry names the same two nodes, supports by node. A row is never bound by its position.
- *Comparison record.* For one quantity of one case: the independent hand-calculation value where one exists, SWBPIPE's value and CAEPIPE's value, in SI base units, with each check and a class for every difference. It identifies the internal model state hash, the exported model hash, the target tool and its version where disclosed, the import and export assumptions, manual changes, the load-case and support mappings, the comparison basis, the tolerance basis and the disposition of each difference (`docs/PRD.md` §22.5). A cause is a labelled hypothesis until evidence supports it.
- *Screening profile.* The benchmark method's tolerances in the release target: 1e-4 relative when every input is held exactly; 5e-3 when a named inexact input remains; 2e-2 for nonlinear cases, with support status matching exactly; 5e-3 on modal frequencies with a modal assurance criterion of at least 0.99; plus a printed-resolution floor per result kind. A scalar passes when |S − C| ≤ a + r · max(|S|, |C|), with a the printed-resolution floor of its result kind and r the class tolerance. Values are only tightened.
- *Screening class.* `within_profile`, `finding` (outside the profile), `unmatched` (a value on one side only, a row that binds to nothing, a partial or non-finite sum, or a parser error) or `status_mismatch` (a case solved on one side and not printed on the other).
- *Inexact input.* A code-dependent input that changes mechanics and cannot be held equal on both sides. It is named on the model and carried by a one-parameter series.
- *Triage.* The examination of a finding in the benchmark method's order (mapping check, closed form, SWBPIPE's own invariants, convention series) ending in one of three classes: SWBPIPE's to fix, a documented convention difference, or unexplained.

**Boundary.** This deliverable owns the harness, the parser, the comparison, the cases and the benchmark models.

- The MBF writer, its identity file, the MBF importer and the authored MBF cases on which the importer is examined are DEL-17-04's. The harness runs the authored cases through the user's CAEPIPE as it runs its own cases, and this deliverable runs the importer's private regression on the MBFs its runs export; the importer's outcomes are DEL-17-04's.
- SWBPIPE's side of every comparison is the product's solve through the headless runner (DEL-10-05). Whether a mechanics capability is correct is a criterion of the deliverable that owns it; a finding classed as SWBPIPE's to fix goes to that deliverable.
- The reference system and the standing mechanics benchmarks are DEL-09-01's. The stress-neutral package is DEL-17-06's. The desktop panel that reviews external-run evidence is DEL-07-12's; it invokes nothing.
- The rule for what public text may say about the prover is DEL-01-02's. Publishing benchmark agreement, and the review of the full record, are the owner's.
- Stresses and code checks are outside every comparison.

**Not decided.**

- Which documented pattern is the first invocation profile, and which print-to-file sections form the parser's coverage (REQ-003, REQ-009; DEL-17-01 `TBD-17-01-004`). The pilot uses one documented pattern and a set of sections as a bounded pilot selection, which governs the pilot only.
- The screening tolerance of the response-spectrum class. It is set before any comparison run, from the class's closed-form references and never from comparison results.
- Which deliverable commits that each published result row names, by its model identifier, the entity it describes (node, element end, support). AC-004 and AC-005 find SWBPIPE's row for a bound member by it; DEL-08-04 commits stable row identifiers and a deterministic order only.
- The pipe shear shape factor and the default gravitational acceleration the prover uses (DEL-17-01 `CQ-17-01-010`). The hand calculations' shear coefficient is an assumption until it is answered.
- The tolerance within which a hand-calculation or closed-form value counts as agreeing with SWBPIPE's solve. Until it is set, each such difference is recorded, a correlation case's difference is also classed (AC-005), and no agreement is claimed from it. Whether SWBPIPE's mechanics are correct remains the owning deliverable's criterion.

## Completion and Reliance Basis — Epistemology

### Requirements

- **REQ-001** — The harness is optional and disabled unless the user supplies a local executable path and accepts responsibility for the licence and the execution environment.
- **REQ-002** — The project does not bundle, download, install or redistribute CAEPIPE or CAEPIPE 3D+, does not wrap it to bypass its licence, and implies no availability of it.
- **REQ-003** — The first executable invocation profile stays undecided until public documentation, a support clarification or the owner's recorded selection of one documented pattern settles it.
- **REQ-004** — The harness records the command shape actually used, the working directory, the input MBF path, the expected and the observed CSV path, the exit status and captured output where available, and diagnostics.
- **REQ-005** — The MBF a run consumes is an output of DEL-17-04's writer, or that output with a declared substitution of whole lines that changes no node, element or support. The lines that may be substituted are the data lines under a named options heading from a maintained short list (the pressure options record; the dynamics options record must be added to the list in the same change that first declares a run requiring its substitution). The run record names the writer's profile and carries each open question that profile carries; for a substituted file it also records each original and replacement line and the resulting file's hash, and results are bound through the writer's identity file.
- **REQ-006** — The run directory holds or references the export manifest, the identity map, the loss report, the MBF input, the CSV output when produced, and the run metadata.
- **REQ-007** — The harness verifies operational facts only: input present, invocation attempted, result file found, parser run, expected sections and counts where confirmed, and diagnostics.
- **REQ-008** — No run or parsed output is classified as professional acceptance, code compliance, formal validation or proof of compatibility. A comparison is validation evidence and is never presented as professional acceptance (`docs/PRD.md` §22.5).
- **REQ-009** — The parser reads only sections confirmed by a public source or by an invented fixture, and reports an unsupported, unrecognized, missing or unmapped section as a diagnostic.
- **REQ-010** — Results are correlated to canonical identifiers through the writer's identity file and manifest; a correlation by row order or name alone is marked weak.
- **REQ-011** — Parsed results are bound to the source model, the export and the run before they are used as evidence.
- **REQ-012** — A check that needs the executable skips, with its reason, when none is configured, and does not fail public CI.
- **REQ-013** — Public parser fixtures are invented or rights-cleared; no CAEPIPE example, private project output or standards-derived data is committed.
- **REQ-014** — User-provided CAEPIPE outputs are private, local and excluded from telemetry by default; a public classification needs documented provenance and redistribution rights.
- **REQ-015** — Diagnostics preserve unit, coordinate, identity-map, loss-report and parser-coverage uncertainty instead of normalizing it away.
- **REQ-016** — External-run evidence binds to a DEL-17-04 MBF export package reference, and an attempted execution blocks unless both the licence and the environment responsibility are acknowledged.
- **REQ-017** — The benchmark compares mechanics only: displacements, rotations, element forces and moments, support loads, frequencies, mode shapes and spectrum responses.
- **REQ-018** — Every benchmark model is invented and comes from a seeded generator that covers model families from straight spans to nonlinear supports, hangers and dynamics; a closed-form member gives an independent third value.
- **REQ-019** — Each code-dependent input that changes mechanics is held equal on both sides or named as an inexact input and carried by a one-parameter series. SWBPIPE is never tuned to match.
- **REQ-020** — Every result is screened with the screening profile. A result outside it, an unmatched row, and a status, refusal or mode-count mismatch are each a finding, and every finding is triaged.
- **REQ-022** — A live step or a screen changes nothing already stored in the exchange folder and removes nothing from it: every
  file it writes is
  new, created so that an existing name is never replaced. A live step puts what it collects in a new run folder of
  its own, and a screen puts its records in a new screen folder of its own; outside those folders a step creates
  only a selection, a series record, the guest script named by its content, the run lock's file where it is absent
  and a holder record of that lock; and, in a step refused because its work directory proves to lie inside the exchange folder, that directory and its probe. It may create absent container directories for these paths. Before a live step
  collects anything from the guest or publishes its selection, it establishes that step's guest exchange root by
  content. The guest works in a fresh work directory that the step creates exclusively for itself, whose
  resolved location the step has checked to be outside the exchange folder; no file of an earlier step is reused
  there, and every place the program is directed to write is checked to lie in that directory before the program
  is started. One live
  step runs at a time: a step holds the run by an operating-system lock on the lock file for as long as its process
  lives, having first shown that the file system honours such a lock, and a step that cannot take it is refused with
  the holder's record. The lock settles which host process may run a step and nothing else: a guest script of an ended run still
  running, an instance of the program that the step did not start, and saved settings that an ended run reached
  and is not recorded to have restored each stop a live step by name, and the last stays until a clearance for
  that run has been recorded by an explicit act that the harness never performs by itself. Before the first guest command that can start the program or change a saved setting, a live step that
  guards the saved settings writes their before-state to a new file in its run folder, holding the value only
  of the members it may write back, and neither that record nor any value or hash from it is copied into a
  report. Which run of a case is current is named by a selection file that is itself new and
  that always names its source: a run folder of the case, or the statement that its results are the ones already
  stored. A case with no selection, or whose selection states that its results are the ones already stored, is read
  where they are. A sealed run is never published as current. For external-program results, a screen reads the
  validated stored source resolved by the case resolver: the current selection, or the legacy location when no
  selection exists. It requires neither the guest, a live guest session, VM tools, guest configuration nor a check
  of the guest's exchange root.

### Acceptance criteria

- **AC-001** — With no exchange folder configured, or with a configuration that lacks the responsibility acknowledgement, every live step skips with a stated reason, public CI passes, and the parser and comparison checks still run. The harness accepts an executable only as the path the user configured; it refuses an exchange folder inside a Git work tree or without an ignore-all rule, and a configured path that is not a plain absolute path, naming the field.
- **AC-002** — The run record of every run, skipped ones included, has each member of a run record as defined above, with a private path redacted where needed. A result file whose run record does not bind it to the current MBF is not compared. The run record names the MBF's profile and carries each open question that profile carries.
- **AC-003** — On the invented print fixture, the parser returns the fixture's authored values, in its output units, for every covered section. A section whose header or unit row differs, or whose unit labels are not of one system, is refused and reported; an unknown section is skipped with a diagnostic; a conflicting duplicate row is reported; and no value is read under an assumed unit.
- **AC-004** — Every compared value is bound through the writer's identity file. A row pair that names no member or several, a second pair for one member, and a member with no pair are each reported as unmatched, and a row pair in another order or position is still bound to its member.
- **AC-005** — For each public correlation case, the comparison record holds the hand-calculation value, SWBPIPE's value and the prover's value for every quantity the case names, with each check and a class for every difference, the difference between the hand calculation and SWBPIPE included; and the case with a deliberately injected input fault is reported as a difference and classed. No difference is left without a class, no cause is recorded as established without its evidence, and every comparison record identifies each item `docs/PRD.md` §22.5 lists.
- **AC-006** — Every model of the benchmark set without declared runs has a benchmark record, and every declared screened run has its own benchmark record, in which every scalar of every mapped case is screened with the profile of its class and carries one screening class. A capture-only run keeps its case's identity; the runner refuses it by name and collects nothing for it, so this refused attempt supplies no result to screen and it is not counted among the screened cases that exit criterion 11 asks for. No non-finite value and no partial sum is `within_profile`. Every finding has a triage class reached in the method's order, every comparison record identifies each item `docs/PRD.md` §22.5 lists, and the owner has reviewed the full record.
- **AC-007** — The benchmark generator gives the same bytes for the same seed, and the tracked models equal a fresh generation. Every model is invented, names per case, and per run where it declares runs, the inputs not held exactly, and belongs to a family. Every run a model declares names its series and the MBF the target program runs, with that file's hash, and, for a screened run, the model document SWBPIPE solves. The set holds straight spans, nonlinear supports, hangers and dynamic models (REQ-018); and each closed-form value a model carries is recorded, in each screened run state where it holds, beside SWBPIPE's solve of that state with their difference.
- **AC-008** — No tracked file is an output of the prover, a value parsed or derived from one, or a comparison or run record of a real run, apart from benchmark agreement the owner has published under the data boundary policy, and none sits under an exchange-style path; a file of such a kind is tracked only with a sidecar declaring it invented. The pilot command's standard output carries operational status only. A run package that embeds a private, protected or commercial payload blocks, and a package's privacy defaults are private, local-only and telemetry off under any partial override.
- **AC-009** — A run package validates against its schema, names the DEL-17-04 MBF export package it rests on, and blocks an attempted execution that lacks either acknowledgement. A skipped package records the configuration checked, why the path was absent or invalid, the skip reason and that parser-only checks ran. No package field, record or output classifies a run or parsed output as professional acceptance, code compliance, formal validation or proof of compatibility, or presents a comparison as professional acceptance.
- **AC-010** — Where an exchange folder is configured, the private regression check re-parses each stored result file, solves the case again, repeats the comparison and fails when any quantity's outcome, cause or evidence status leaves its recorded classification. It prints no prover value, and it skips with its reason when no folder is configured.
- **AC-012** — For every live path and every screen, entered through its command, every path that existed before
  the step still exists with its original content afterwards, on success, on every refusal, on error and on a crash
  at every write boundary, and no path is removed. This holds with stored files linked, by hard or symbolic link,
  to names a step uses; with a name appearing at a collection destination after the step's own check, the guest
  overwriting as the real guest does; and with a work
  directory, a child path of it or an output destination of the program that resolves into the exchange folder,
  and with a work file linked to stored content, each of which is refused before any guest command that deletes,
  copies or starts the program; a step's work directory is new and holds nothing of an earlier step. Where the resolved path looks separate and the content probe proves otherwise, the step is refused, nothing stored is changed, and the two created items are named in the refusal. A conflicting, unanswered or failed candidate in the
  drive check refuses the step. A selection is created exclusively and never rewritten, and the highest sequence
  number is current. It is published only after its run's record names that case and that run folder, the run
  ended in an outcome eligible for selection and its verified files hash as recorded. A selection that is corrupt,
  conflicting, without a stated source or naming a sealed run stops the reader by name, and the case is read from
  nowhere else. A live step started while another holds the run is refused with the holder's record and issues no
  guest command; after the holder's process has been killed the lock no longer refuses the next step, and no file has been
  removed; on a
  file system that does not honour the lock the step is refused. A live step is refused by its own name for each of: a guest script of an ended run still running; an instance
  of the program it did not start; and an ended run that reached the program's saved settings and has no record
  of their restore and no recorded clearance. Every step that reserved a run folder leaves a complete record of
  its outcome, under another new name where the first is occupied. Every screen command completes a screen, its
  records produced, with the guest command absent from the host and no guest configuration present. No reader
  opens the content of a sealed result: each reads the sealed flag first, and the private regression copies only
  what the selection names.

## Production and Verification Method — Praxeology

**Method choices that define the product.**

- The harness runs one case at a time, in the user's environment, and stops no process it did not start.
- A result is bound by hash to the MBF that produced it before any value is compared.
- The comparison works in SI base units, and the printed resolution of the prover's output sets the floor of every tolerance.
- A difference is carried by a named hypothesis and a one-parameter series, never absorbed by a wider tolerance.
- The cases, models, settings, comparison rule and code are public; runs, results, comparisons and classifications are private, except benchmark agreement that the owner publishes under the data boundary policy.

**How each criterion is examined.**

- **VER-001** — Executable check (observes the harness with no configuration, with an unacknowledged configuration, with an exchange folder inside a work tree and with an invalid path): each skips or refuses with its stated reason, and no process is started.
- **VER-002** — Executable check (observes the run record written for a run with a substituted runner, and the comparison's refusal of an unbound result file).
- **VER-003** — Comparison with a named reference (the invented print fixture, whose values were authored by hand) and executable check (observes the parser on altered headers, mixed unit systems, unknown sections and duplicate rows).
- **VER-004** — Executable check (observes binding on invented tables with reordered, missing, duplicated and unbindable rows).
- **VER-005** — Comparison with a named reference (closed-form hand calculations that import no product code) and executable check (observes the comparison record of each public case on invented prover values, the fault-injected case included).
- **VER-006** — Comparison with a named reference (CAEPIPE's printed results for each benchmark model, produced by the user's own installation) screened by the release target's profile, and judgment by the owner: reviews the full record of screening and triage.
- **VER-007** — Executable check (observes the generator's output for a fixed seed against the tracked models, each model's declarations and family, and each closed-form value against SWBPIPE's solve).
- **VER-008** — Executable check (observes tracked files for prover output, exchange-style paths and undeclared derived records; the pilot command's standard output; and run packages with seeded private payloads and partial privacy overrides) and judgment by a reviewer who did not write the text: examines public text added in this deliverable for content that evaluates the prover.
- **VER-009** — Executable check (observes run packages against the schema, with and without each acknowledgement, and the skipped package's fields) and judgment by a reviewer who did not write them: examines records and outputs for the classifications REQ-008 excludes.
- **VER-010** — Executable check (observes the private regression check on an invented exchange folder and with none configured); the check itself runs where a real exchange folder is configured.
- **VER-012** — Executable check (observes, through each command with the guest substituted by one that overwrites
  as the real guest does, a digest of every pre-existing path before and after: each live path and each screen on
  success, on each refusal, on an error and on a crash at each write boundary; planted links, and names appearing
  after the check at the real copy destinations; an aliased work directory, child path and output destination, and a linked work file; conflicting drive answers; a selection of
  each untrusted kind; the lock held, its holder killed, and a file system that does not honour it; an ended run's guest script still running, a foreign program instance, and an ended run with settings reached and neither restore nor clearance recorded; early and partial record failures; the screen commands with neither guest command
  nor guest configuration; and sealed results with every read of content recorded outside any helper that catches
  an exception) and judgment by a reviewer who did not write the code: makes an inventory of its own of every site,
  host and guest, that creates, writes or removes a path in the exchange folder. The property is claimed against
  every action of the harness and its guest script, the program the script operates included for every file the
  program writes at the script's direction, on every path and outcome, crashes and retries included; against any
  state of the exchange folder present when a step starts, links included; against another harness process at the
  same time; and against a guest whose letters, mappings or paths differ from the configuration by accident, a work
  folder that resolves into the exchange folder included. It is not claimed against another actor that writes into
  the exchange folder while a step runs, nor against a guest that answers falsely by design.

## Governing Values and Decisions — Axiology

- **AX-001** — *Breadth of comparison.* Better is more benchmark families, cases and result kinds screened, and fewer unmatched rows. Breadth is never bought by loosening a tolerance, by tuning SWBPIPE to match, or by binding a row by its position.
- **AX-002** — *Explained differences.* Better is a larger share of findings triaged to a cause that a closed form or a series supports. A cause is never recorded without that evidence; the finding stays unexplained.
- **AX-003** — *The user's tool.* No convenience justifies finding, bundling or installing the executable, or running it without the user's configuration and acknowledgement.
- **AX-004** — *Evaluate privately.* Nothing that evaluates the prover leaves the exchange folder except what the data boundary policy lets the owner publish, and narrow confirmed parser coverage is preferred to a guess at an unconfirmed section.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-030 SOW-075 OBJ-009 OBJ-018 | REQ-001 REQ-002 REQ-012 | AC-001 | VER-001 | The decomposition row and `docs/PRD.md` §22.5 make the harness optional and user-owned; the seeded configurations state what must skip or refuse |
| OUT-001 | SOW-075 OBJ-017 | REQ-004 REQ-005 REQ-006 REQ-007 REQ-011 | AC-002 | VER-002 | REQ-004 and REQ-006 carry the statement of what a run records; the substituted runner's inputs, which the harness did not write, state what the record must hold |
| OUT-001 | SOW-046 OBJ-007 | REQ-003 REQ-009 REQ-013 REQ-015 | AC-003 | VER-003 | Outside source: the invented print fixture's hand-authored values and the vendor's public print-to-file page admitted in the DEL-17-01 register |
| OUT-001 | SOW-046 SOW-075 OBJ-007 OBJ-017 | REQ-010 REQ-011 | AC-004 | VER-004 | The identities compared are those of the invented model and its identity file, which the parser did not write |
| OUT-002 | SOW-046 SOW-075 OBJ-007 OBJ-017 | REQ-008 REQ-019 | AC-005 | VER-005 | Outside source: closed-form Euler–Bernoulli and Timoshenko hand calculations that import no product code; `docs/PRD.md` §22.5 lists what a comparison identifies. Without a set agreement tolerance the record shows each difference, not that SWBPIPE is correct |
| OUT-002 | SOW-046 SOW-075 OBJ-007 OBJ-017 | REQ-017 REQ-019 REQ-020 | AC-006 | VER-006 | Outside source: CAEPIPE's own printed results, screened by the release target's benchmark-method profile; `docs/PRD.md` §22.5 lists what a comparison identifies, and the owner's review of the full record decides |
| OUT-002 | SOW-046 OBJ-007 | REQ-018 REQ-019 | AC-007 | VER-007 | Outside source: closed-form member solutions for the third value; equality with a fresh generation is the product's own earlier output and guards against change only |
| OUT-001 | SOW-075 OBJ-018 | REQ-013 REQ-014 | AC-008 | VER-008 | `docs/IP_AND_DATA_BOUNDARY.md` (interoperate openly, evaluate privately); the seeded payloads and invented exchange folder state what must stay private |
| OUT-001 | SOW-030 SOW-075 OBJ-009 OBJ-018 | REQ-008 REQ-012 REQ-016 | AC-009 | VER-009 | `docs/PRD.md` §16.3 and §21.2; the schema is examined with a JSON Schema 2020-12 validator |
| OUT-002 | SOW-046 OBJ-007 | REQ-014 REQ-020 | AC-010 | VER-010 | The product's own earlier classification, which guards against change only |
| OUT-001 | SOW-075 OBJ-017 OBJ-018 | REQ-022 | AC-012 | VER-012 | The owner's condition on the private exchange folder (reading and adding, never changing what is stored); the substituted guest and the planted states say what must stay unchanged |
