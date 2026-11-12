---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-14-02
package_id: PKG-14
decomposition_basis: execution/_Decomposition/SOFTWARE_DECOMP.md
project_scope_refs: [SOW-072]
package_objective_refs: [OBJ-016]
---

# Scope of Work — DEL-14-02 Analysis run records

## Purpose and Objective Traceability

Every result the product shows can be traced to exactly what produced it: the model state that was solved, the solver build and settings, the units, the load cases and combinations, the diagnostics, and the rule packs and libraries referred to. An analysis run record is that binding. A recorded run stays as it was after the model moves on, so results belong to a run and never only to whatever the current model happens to be, and anyone holding a run can recompute the hashes that tie it to its inputs and results.

It serves SOW-072 under OBJ-016 (`execution/_Decomposition/SOFTWARE_DECOMP.md`, PKG-14, row DEL-14-02), and `docs/PRD.md` §5.7 (every result set binds to its model state, solver version, settings, load cases, rule-pack references and library versions), §8.7, §15.2, FR-CMP-002 (Must) and the run level of FR-MOD-006 (Must: assumptions and open issues recorded at run level). No exit criterion of `docs/RELEASE_TARGET_v1.0.md` names it. It supports criterion 4, whose report states the run it reports, through DEL-08-01 and DEL-08-02.

- **OUT-001** — The analysis run record: its schema in one current version, the building and validation of a record from a solved result, the building of the input manifest it names, and the recording, listing and reading of runs in the project store, with their labels.

## Deliverable Definition — Ontology

**Terms the product implements.**

- *Analysis run.* One solver execution on one model: a static solve on either analysis route, or a response-spectrum analysis, whose run kind and the request it records are DEL-04-10's.
- *Run record.* The schema-governed record of a run (`schemas/analysis_run.schema.json`): its identity, name, kind and time of creation; the model state it is bound to; the solver's name, version and build reference; references to its settings and unit system; its load cases and combinations; diagnostics; result references; rule-pack references; library references; hashes; analysis statuses; reproducibility data; immutability policy; professional boundary and provenance.
- *Received result.* The mechanics result as its producer published it. A record binds it and never rewrites it.
- *Result reference.* What a record says of one result row: the row's identifier and position, its family and the dimension its producer stated, the semantic contract under which it was produced (identifier and digest), how the row was interpreted, and its hash.
- *Input manifest.* The canonical statement of what a solve was given, listing what DEL-08-02/REQ-012 requires. This deliverable builds it for each solve, keeps it with a recorded run and binds it to the record, which names it by hash.
- *Transient record.* The record built when a solve completes, for the session's current run. It names the session's model, not a saved state.
- *Recorded run.* An immutable revision of a run's record, made when the user records the run. It is bound to a saved model state, carries its own time of recording, and is kept in the project store with the received result and the input manifest of the solve.
- *Record revision.* A record is identified by its own SHA-256. A change makes a new revision and never edits one.
- *Checked binding.* A recorded run is bound to its state by a check and not by assertion: the model payload in the solve's input manifest hashes to the state's model hash, the manifest hashes to the value the record names, and the manifest is the one the solve that produced the result was given.
- *Rule-pack reference.* The identity, version, checksum and source notice of a pack that the model refers to and that was resolved for the solve. Resolution records identity only: it applies no pack and makes no check pass.
- *Run labels.* A recorded run's name, tags, notes and run-level assumptions. They are editable and outside the record's hash, as DEL-14-01 defines for a state's labels.

**One version.** There is one current run-record version, named in the schema. A record of any other version is refused by name and the user is told to re-run; nothing is migrated, and the bytes of an older record are never relabelled or re-hashed.

**Undecided.**

- Whether a hanger-sizing result is recorded as an analysis run. DEL-04-11 leaves the same question open for its result.
- How result payloads that are not JSON are partitioned for hashing.
- Which members a project's recorded runs occupy in the portable package, and which deliverable's package writer and reader carry them there (REQ-018). DEL-14-01 states the same for saved states.
- Where a record takes the solver's version and build reference from (REQ-003). DEL-04-07's published result does not state them, and DEL-08-02/REQ-012 does not list them in the input manifest.

**Boundary.**

- **CLM-001** — What a result means belongs to its producers: the solve and the published result to DEL-04-07, the stress rows to DEL-05-03, the ordinary result reader and its base semantic tables to DEL-05-02, verified bounds and the reader of retained-precision and retained-source statements to DEL-04-08. This deliverable records a result's identity, standing and classification as the reader gives them and re-derives none of them.
- **CLM-002** — Saved model states are DEL-14-01's. The canonical JSON form, the form of a hash record, the audit manifest and what an input manifest must list are DEL-08-02's. The project store is DEL-02-05's, the status vocabulary DEL-05-04's, the diagnostic record DEL-00-06's, and the local keeping and checksum of rule packs DEL-06-04's.
- **CLM-003** — Comparing two runs is DEL-14-04's. Holding a result in the native process and showing it are DEL-07-07's and DEL-07-05's; recording a run from the History panel is DEL-07-08's interaction. The result export document is DEL-08-04's and the run sections of a report DEL-08-06's.
- **CLM-004** — Outside the product: comprehensive ingestion of a commercial prover's outputs, external validation, and any record of professional acceptance.

## Completion and Reliance Basis — Epistemology

A run record establishes which inputs and which build produced a result, and that neither has changed since. It does not establish that the result is right: that is examined where the solve is examined. A row recorded as unverifiable and a missing value recorded as a finding keep a gap visible; neither supplies the unit, dimension or value the producer did not state.

### Requirements

- **REQ-001** — The run record is a schema-first contract in JSON Schema 2020-12, `schemas/analysis_run.schema.json` (also cited as DEL-14-02-R001). Source: SOW-072; decomposition AB-00-03.
- **REQ-002** — Every recorded run is bound to one exact saved model state, by the checked binding (also cited as DEL-14-02-R002). Source: SOW-072; PRD FR-CMP-002.
- **REQ-003** — A run record preserves the solver version and build reference, the settings, the units, the load cases and combinations, the diagnostics and warnings, the results, the rule-pack references, the library references, the result hashes and the time of the run (also cited as DEL-14-02-R003). Source: SOW-072; PRD §15.2.
- **REQ-004** — Results attach to immutable run records and not only to the mutable current model. A recorded run, its received result and its input manifest are read-only, and a change makes a new revision (also cited as DEL-14-02-R004). Source: SOW-072.
- **REQ-005** — A unit-bearing value a record includes or refers to carries its unit, unless its field is explicitly classified as dimensionless, a ratio, a percentage or a coefficient. The received rows are kept unchanged; interpretation is recorded beside them and never supplies a dimension or other metadata the producer did not state. A row without them is recorded as unverifiable (also cited as DEL-14-02-R005 and DEL-14-02:3). Source: PRD §5.6.
- **REQ-006** — A missing solve-required or rule-check-required value is an explicit finding in the record, with its diagnostic and provenance, and never a silent default (also cited as DEL-14-02-R006 and DEL-14-02:2). Source: PRD §5.5.
- **REQ-007** — Each hash record names its payload scope, the record revision and its serialization profile. Computed hashes are SHA-256 over the RFC 8785 canonical form, on the product's one canonical implementation. Checksums received from a producer are kept apart from computed ones and unchanged (also cited as DEL-14-02-R007). Source: SOW-072; decomposition AB-00-04.
- **REQ-008** — The record keeps the software's authority boundary: results and diagnostics are software outputs, human acceptance is a separate record bound by hash, and no status or label of a run states approval, certification, sealing, authentication or code compliance (also cited as DEL-14-02-R008). Source: PRD §21.2, §21.3.
- **REQ-009** — Rule-pack and library references give identity, version, checksum and provenance. No record, public fixture or example embeds protected standards text or tables, proprietary values, private formulas or a private pack's payload (also cited as DEL-14-02-R009). Source: SOW-072; PRD §20.1.
- **REQ-010** — A run is reproducible as a record: the same solved result and inputs give the same record and the same hashes in every implementation that builds one (also cited as DEL-14-02-R010). Source: SOW-072; PRD §22.4.
- **REQ-011** — Building, recording, listing and reading runs are commands and queries that return result envelopes with their diagnostics (also cited as DEL-14-02-R011). Source: decomposition AB-00-03.
- **REQ-012** — The desktop builds a record for every completed solve of an analysis whose results are recorded as runs (AC-001); a modal result is not one (DEL-04-09/REQ-002). It records runs against saved model states with the checked binding. A recorded run's rule-pack references are the model's references resolved against the local rule-pack store at solve time. A reference that is unresolved, or whose stored pack has a different checksum, is named as a diagnostic of the record and is not among its rule-pack references (also cited as DEL-14-02:1). Source: PRD FR-CMP-002, §15.2.
- **REQ-013** — A record is built only from a result the product reader accepts. A refused result, and a result of a retired identity, is never recorded. The record carries the standing and row classification the reader gave, and recording changes neither. Source: DEL-05-02/REQ-019 and DEL-04-08/REQ-005 (one product reader of result statements).
- **REQ-014** — The one-version rule holds for every reader of a run record.
- **REQ-015** — A stored run whose record, received result, input manifest or state binding no longer agrees with its recorded hashes is refused by name on every read. Source: SOW-072 (result hashes).
- **REQ-016** — A recorded run's labels, run-level assumptions among them, can be edited without changing its record, its hashes or its binding. Source: PRD FR-MOD-006.
- **REQ-017** — A recorded run keeps its own format. It is listed, read and compared by its project's identity, and no model-document version check is applied to it, so the runs of a project whose model document is of a retired version remain reachable.
- **REQ-018** — A recorded run's domain truth is its canonical JSON record with the received result and input manifest it binds, and its labels, and the project store holds them as a projection that can be rebuilt from them. A project's recorded runs travel with the project in its portable package and come back from it with the same bytes, labels, binding and hashes. Source: SOW-072; decomposition AB-00-04.

### Acceptance criteria

- **AC-001** — A record built from an invented solved model, for each analysis whose results are recorded as runs (the static analyses on each analysis route, and response spectrum), validates against the schema, and each category of REQ-003 is present and equal to the corresponding value of the received result and the input manifest.
- **AC-002** — Recording a run against a saved state succeeds only when the model payload of the solve's input manifest hashes to the state's model hash and the manifest hashes to the value the record names. A changed manifest, another state and a run that holds no manifest are each refused by name, and nothing is stored. A result offered with the input manifest of another solve is refused by name, and nothing is recorded.
- **AC-003** — After a run is recorded, editing the model, solving again and saving the project each leave the recorded record, received result and input manifest byte-identical. An attempt to change a recorded run, through any route the product offers, is refused by name. Two recordings of identical solves are two distinct runs.
- **AC-004** — Each computed hash of a record is reproduced, from the payload it names, by an implementation of RFC 8785 and SHA-256 that imports no product code. Each hash record states its scope and profile, a received checksum appears apart from the computed ones and as received, and a change to any covered value changes the hash that covers it.
- **AC-005** — A record of any version other than the current one is refused by name by every reader, with the instruction to re-run, and its stored bytes are unchanged. A recorded run of the current record version whose input manifest holds a model document of a retired version is still listed, read and compared.
- **AC-006** — For one committed set of solved results and inputs, every implementation that builds a record gives byte-identical records in the stored form, equal hashes, and the same refusal for each input that is refused.
- **AC-007** — A result row for which the producer stated no unit or dimension is recorded as unverifiable, with a finding and no inferred unit. A missing solve-required or rule-check-required value appears as a diagnostic with its provenance and no substituted value. The received rows in the record equal the producer's, row for row.
- **AC-008** — A model reference to a pack the local store holds with the referenced checksum appears among the record's rule-pack references with its identity, version, checksum and source notice. A reference to an absent pack, and one whose stored pack has another checksum, each appear as a named diagnostic and not among the references. No pack content appears in the record.
- **AC-009** — A result the product reader refuses, and a result of a retired identity, each yield a named refusal and no record. For an accepted result the record's standing and row classes equal the reader's.
- **AC-010** — A stored run altered outside the product, in its record, received result, input manifest or state reference, is refused by name when it is read or compared. Editing a recorded run's labels leaves its record and hashes unchanged.
- **AC-011** — No status, field or label of a run record states approval, certification, sealing, authentication or code compliance; its analysis statuses are those of DEL-05-04's vocabulary; and every public fixture and example of a run is invented and holds no protected or private content.
- **AC-012** — Building a record, recording a run, listing runs, reading a run and editing a run's labels each return a result envelope carrying the outcome and its diagnostics, on success and on each refusal of AC-002, AC-005, AC-009 and AC-010. None reports its outcome by an error text alone.
- **AC-013** — A project with recorded runs, written to its portable package and read into an empty project store with its saved states, lists the same runs, and each returns its record, received result, input manifest and labels byte-identical to those written, bound to the same state, with hashes that verify as AC-004 requires.

## Production and Verification Method — Praxeology

**Method choices that define the product.**

- The received result is stored and bound as received. Interpretation sits beside it, so that what the producer said and what the record concluded can always be told apart.
- A binding is checked from content, never taken from a caller's claim.
- A change is a new revision. Nothing edits a record, and no reader repairs or migrates one.
- Every reader of a result takes the product reader's verdict; a second implementation exists only as a test verifier.
- Where more than one implementation builds a record, they are held to byte equality on a committed set of cases.

**How each criterion is examined.**

- **VER-001** — Executable check (observes records built from invented solved models for each such analysis against the schema, under a JSON Schema 2020-12 validator that imports no product code, and against the received result and input manifest they were built from).
- **VER-002** — Executable check (observes Record Run with a matching manifest and state, a changed manifest, another state, no manifest and the manifest of another solve, and the store after each).
- **VER-003** — Executable check (observes the stored record, result and manifest before and after model edits, a further solve and a project save; attempted changes through each service; and two recordings of identical solves).
- **VER-004** — Comparison with a named reference (RFC 8785 and SHA-256, computed by an implementation that imports no product code) with an executable check (observes that each covered value changes its hash).
- **VER-005** — Executable check (observes each reader given records of older, newer and unrecognised versions, and the stored bytes afterwards; and the listing, read and comparison of recorded runs whose manifests hold model documents of retired versions).
- **VER-006** — Executable check (observes each record-building implementation on one committed set of cases and compares records, hashes and refusals between them; it shows that the implementations agree and nothing more).
- **VER-007** — Executable check (observes records built from constructed results with a row lacking a unit or dimension, a missing solve-required value and a missing rule-check-required value, and compares the recorded rows with the producer's).
- **VER-008** — Executable check (observes a solve of a model whose rule-pack references are held, absent and held with another checksum, and the record's references and diagnostics).
- **VER-009** — Comparison with a named reference (the product reader's verdict for each committed result, which this deliverable does not compute).
- **VER-010** — Executable check (observes reads of recorded runs whose stored record, result, manifest and state reference were each altered in turn, and label edits).
- **VER-011** — Judgment by a reviewer who did not write the record, who examines the schema's vocabularies and the public fixtures against PRD §20.1 and §21.2 with the protected-content scan.
- **VER-012** — Executable check (observes the envelope each operation of AC-012 returns on success and on each refusal the criterion names).
- **VER-013** — Executable check (observes the recorded runs of a project written to its portable package and read into an empty project store, against the runs written).

## Governing Values and Decisions — Axiology

- **AX-001** — *A result is never without its basis.* Better is fewer results shown, exported or reported without a record that binds them to their state and inputs; the target is none. A binding is never asserted to make a run recordable.
- **AX-002** — *What the producer said is kept as said.* Better is more of a result's meaning carried from its producer into the record. It may never be gained by inferring a unit, a dimension or a standing the producer did not state.
- **AX-003** — *Refuse by name over reading leniently.* An older record, a refused result and a run that does not verify are each named and refused. Convenience of reopening old work never justifies migrating, relabelling or re-hashing it.
- **AX-004** — *The same record everywhere.* Better is fewer cases on which two implementations differ; the target is none, and agreement between them is never offered as evidence that the result is right.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-072 OBJ-016 | REQ-001 REQ-003 REQ-012 | AC-001 | VER-001 | Outside source for the record's form: the JSON Schema 2020-12 specification. The categories are those of PRD §15.2, and each is compared with the received result and input manifest, which the record builder did not write |
| OUT-001 | SOW-072 OBJ-016 | REQ-002 REQ-012 | AC-002 | VER-002 | The criterion fixes the expected result. The manifests and states are constructed to match or not, the pair of a result with another solve's manifest is constructed too, so the criterion states each refusal, and the hashes are recomputed with RFC 8785 and SHA-256 |
| OUT-001 | SOW-072 OBJ-016 | REQ-004 | AC-003 | VER-003 | The stored bytes are compared with a copy taken before each operation |
| OUT-001 | SOW-072 OBJ-016 | REQ-007 | AC-004 | VER-004 | Outside source: RFC 8785 and SHA-256 |
| OUT-001 | SOW-072 OBJ-016 | REQ-014 REQ-017 | AC-005 | VER-005 | The records are constructed, so the criterion states which are refused and which are read |
| OUT-001 | SOW-072 OBJ-016 | REQ-010 | AC-006 | VER-006 | The product's own output on committed cases, compared between its implementations. This guards against the implementations drifting apart and is not grounds for correctness; AC-001 and AC-004 carry those |
| OUT-001 | SOW-072 OBJ-016 | REQ-005 REQ-006 | AC-007 | VER-007 | The results are constructed with a known defect each, so the criterion states the expected finding; recorded rows are compared with the producer's rows |
| OUT-001 | SOW-072 OBJ-016 | REQ-009 REQ-012 | AC-008 | VER-008 | The packs and references are constructed to resolve or not, so the criterion states the expected references and diagnostics; pack checksums are recomputed as DEL-06-04 defines them |
| OUT-001 | SOW-072 OBJ-016 | REQ-013 CLM-001 | AC-009 | VER-009 | The expected verdicts are the readers', which DEL-05-02 and DEL-04-08 own and examine against their own references |
| OUT-001 | SOW-072 OBJ-016 | REQ-015 REQ-016 | AC-010 | VER-010 | Each alteration is made deliberately to a known run, so the criterion states that it is refused; hashes after a label edit are compared with those recorded before it |
| OUT-001 | SOW-072 OBJ-016 | REQ-008 REQ-009 CLM-004 | AC-011 | VER-011 | Judged by a reviewer against PRD §20.1, §21.2 and §21.3 |
| OUT-001 | SOW-072 OBJ-016 | REQ-011 | AC-012 | VER-012 | The criterion fixes the expected result: DEL-00-03's result-envelope contract, which this deliverable does not define, compared with the envelope each operation returns |
| OUT-001 | SOW-072 OBJ-016 | REQ-018 | AC-013 | VER-013 | The criterion fixes the expected result: the runs read back from the package are the runs written, compared byte for byte, with their state bindings checked and their hashes recomputed as AC-004 requires |
