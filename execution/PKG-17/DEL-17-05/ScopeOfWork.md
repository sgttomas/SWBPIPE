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
- *Screening class.* `within_profile`, `finding` (outside the profile), `unmatched` (a value on one side only, a row that binds to nothing, a partial or non-finite sum, or a parser error), `status_mismatch` (a case solved on one side and not printed on the other) or `sensitive_side` (SWBPIPE itself publishes the case's numerical quality as other than checks passed; the scalar is compared with nothing and is never `within_profile`).
- *Inexact input.* A code-dependent input that changes mechanics and cannot be held equal on both sides. It is named on the model and carried by a one-parameter series.
- *Triage.* The examination of a finding in the benchmark method's order (mapping check, closed form, SWBPIPE's own invariants, convention series) ending in one of three classes: SWBPIPE's to fix, a documented convention difference, or unexplained.

**Boundary.** This deliverable owns the harness, the parser, the comparison, the cases and the benchmark models.

- The MBF writer, its identity file, the MBF importer and the authored MBF cases on which the importer is examined are DEL-17-04's. The harness runs the authored cases through the user's CAEPIPE as it runs its own cases, and this deliverable runs the importer's private regression on the MBFs its runs export; the importer's outcomes are DEL-17-04's.
- SWBPIPE's side of every comparison is the product's solve through the headless runner (DEL-10-05). For the benchmark set, the side that counts for exit criterion 11 uses the exact route (DEL-04-07). A benchmark model authored without the pressure contract is solved as its exact-route twin: the same mechanics restated in the form the exact route requires, under the shear model the benchmark records. The twin is a side of a stored result only where the writer writes its model file byte for byte as it writes the tracked model's. The pressure-free solve of the same model is kept as a second, labelled series. Whether a mechanics capability is correct is a criterion of the deliverable that owns it; a finding classed as SWBPIPE's to fix goes to that deliverable.
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
- **REQ-020** — Every result is screened with the screening profile. A result outside it, an unmatched row, and a status, refusal or mode-count mismatch are each a finding, and every finding is triaged. A case whose SWBPIPE side the solver publishes with a numerical quality status other than checks passed is not screened: each of its scalars carries the class `sensitive_side`, the case is recorded as one finding with the integrity status and the cause the solver names, and its comparison waits for the answer of the deliverable that owns that cause; while it is so published, exit criterion 11 is unmet for that case. Which cases these are is read from SWBPIPE's own result and from nothing else. A case is `sensitive_side` whenever the solver publishes it so, whatever its values, and while it is so published no step forms, records or shows a comparison of it with the prover's values.

### Acceptance criteria

- **AC-001** — With no exchange folder configured, or with a configuration that lacks the responsibility acknowledgement, every live step skips with a stated reason, public CI passes, and the parser and comparison checks still run. The harness accepts an executable only as the path the user configured; it refuses an exchange folder inside a Git work tree or without an ignore-all rule, and a configured path that is not a plain absolute path, naming the field.
- **AC-002** — The run record of every run, skipped ones included, has each member of a run record as defined above, with a private path redacted where needed. A result file whose run record does not bind it to the current MBF is not compared. The run record names the MBF's profile and carries each open question that profile carries.
- **AC-003** — On the invented print fixture, the parser returns the fixture's authored values, in its output units, for every covered section. A section whose header or unit row differs, or whose unit labels are not of one system, is refused and reported; an unknown section is skipped with a diagnostic; a conflicting duplicate row is reported; and no value is read under an assumed unit.
- **AC-004** — Every compared value is bound through the writer's identity file. A row pair that names no member or several, a second pair for one member, and a member with no pair are each reported as unmatched, and a row pair in another order or position is still bound to its member.
- **AC-005** — For each public correlation case, the comparison record holds the hand-calculation value, SWBPIPE's value and the prover's value for every quantity the case names, with each check and a class for every difference, the difference between the hand calculation and SWBPIPE included; and the case with a deliberately injected input fault is reported as a difference and classed. No difference is left without a class, no cause is recorded as established without its evidence, and every comparison record identifies each item `docs/PRD.md` §22.5 lists.
- **AC-006** — Every model of the benchmark set without declared runs has a benchmark record, and every declared screened run has its own benchmark record, in which every scalar of every mapped case is screened with the profile of its class and carries one screening class. A capture-only run keeps its case's identity; the runner refuses it by name and collects nothing for it, so this refused attempt supplies no result to screen and it is not counted among the screened cases that exit criterion 11 asks for. No non-finite value and no partial sum is `within_profile`. Every finding has a triage class reached in the method's order, every comparison record identifies each item `docs/PRD.md` §22.5 lists, and the owner has reviewed the full record. On an invented SWBPIPE result whose numerical quality marks one case as other than checks passed, every scalar of that case carries `sensitive_side` and none is `within_profile` or `finding`, the record holds the status and the solver's named cause, the other cases of the model are screened as before, and the case is not counted among the screened cases.
- **AC-007** — The benchmark generator gives the same bytes for the same seed, and the tracked models equal a fresh generation. Every model is invented, names per case, and per run where it declares runs, the inputs not held exactly, and belongs to a family. Every run a model declares names its series and the MBF the target program runs, with that file's hash, and, for a screened run, the model document SWBPIPE solves. The set holds straight spans, nonlinear supports, hangers and dynamic models (REQ-018); and each closed-form value a model carries is recorded, in each screened run state where it holds, beside SWBPIPE's solve of that state with their difference.
- **AC-008** — No tracked file is an output of the prover, a value parsed or derived from one, or a comparison or run record of a real run, apart from benchmark agreement the owner has published under the data boundary policy, and none sits under an exchange-style path; a file of such a kind is tracked only with a sidecar declaring it invented. The pilot command's standard output carries operational status only. A run package that embeds a private, protected or commercial payload blocks, and a package's privacy defaults are private, local-only and telemetry off under any partial override.
- **AC-009** — A run package validates against its schema, names the DEL-17-04 MBF export package it rests on, and blocks an attempted execution that lacks either acknowledgement. A skipped package records the configuration checked, why the path was absent or invalid, the skip reason and that parser-only checks ran. No package field, record or output classifies a run or parsed output as professional acceptance, code compliance, formal validation or proof of compatibility, or presents a comparison as professional acceptance.
- **AC-010** — Where an exchange folder is configured, the private regression check re-parses each stored result file, solves the case again, repeats the comparison and fails when any quantity's outcome, cause or evidence status leaves its recorded classification. It prints no prover value, and it skips with its reason when no folder is configured.

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
- **VER-006** — Comparison with a named reference (CAEPIPE's printed results for each benchmark model, produced by the user's own installation) screened by the release target's profile, and judgment by the owner: reviews the full record of screening and triage. Executable check (observes the screening of an invented SWBPIPE result whose numerical quality marks one case as other than checks passed: the class of every scalar of that case, the absence of any prover value or difference for it in the record, the status and cause recorded, the other cases unchanged, and the counts).
- **VER-007** — Executable check (observes the generator's output for a fixed seed against the tracked models, each model's declarations and family, and each closed-form value against SWBPIPE's solve).
- **VER-008** — Executable check (observes tracked files for prover output, exchange-style paths and undeclared derived records; the pilot command's standard output; and run packages with seeded private payloads and partial privacy overrides) and judgment by a reviewer who did not write the text: examines public text added in this deliverable for content that evaluates the prover.
- **VER-009** — Executable check (observes run packages against the schema, with and without each acknowledgement, and the skipped package's fields) and judgment by a reviewer who did not write them: examines records and outputs for the classifications REQ-008 excludes.
- **VER-010** — Executable check (observes the private regression check on an invented exchange folder and with none configured); the check itself runs where a real exchange folder is configured.

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
