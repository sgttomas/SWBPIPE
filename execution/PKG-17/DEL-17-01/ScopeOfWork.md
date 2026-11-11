---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-17-01
package_id: PKG-17
decomposition_basis: execution/_Decomposition/SOFTWARE_DECOMP.md
project_scope_refs: [SOW-030, SOW-074, SOW-075]
package_objective_refs: [OBJ-009, OBJ-017, OBJ-018]
---

# Scope of Work — DEL-17-01 CAEPIPE and export-format source basis

## Purpose and Objective Traceability

This deliverable is the admitted source basis for the export and import formats: the register of the public documents and project references the format deliverables may rely on, what each one establishes and what it does not, the findings drawn from them, and the questions about target behaviour that no admitted source answers. The result it contributes is that every statement the product or its scopes make about a target format (CAEPIPE MBF, CAEPIPE batch execution and result CSV, PCF translation, glTF) traces to a named public source or is carried as an open question, so that no target behaviour is invented and no protected or proprietary material enters the repository.

It serves SOW-030, SOW-074 and SOW-075 under OBJ-009, OBJ-017 and OBJ-018 (`execution/_Decomposition/SOFTWARE_DECOMP.md`, PKG-17, row DEL-17-01, and open issue OI-017), with `docs/PRD.md` §8.9, §11.7 (FR-HAND-005), §16.2, §20.1 and §21.2. No exit criterion of `docs/RELEASE_TARGET_v1.0.md` names it; its register grounds the format facts that the MBF round trip (exit criterion 6) and the benchmark set (exit criterion 11) depend on.

- **OUT-001** — The source basis, two documents in this deliverable's folder: `Source_Basis_Register.md` (admitted sources, project references, findings, boundary constraints and open questions) and `CAEPIPE_Question_Dossier.md` (the questions for the vendor's developer or support team, each linked to the open question it would close).

## Deliverable Definition — Ontology

**Terms the source basis implements.**

- *Admitted source.* A public official vendor document, a public format specification or a project-owned document that the register lists with a source ID, a location (URL or path, and the section heading), the evidence it is admitted for, and its boundary: what it does not establish.
- *Source ID.* The stable name of an admitted source, such as `CAEPIPE-IMPORT-MBF`, `CAEPIPE-BATCH`, `CAEPIPE-PCF` or `GLTF-2.0`. Schemas, export profiles, fixtures and other scopes cite sources by this name.
- *Consultation boundary.* The subject an admitted source may be read for. Reading a source beyond its boundary establishes nothing, and a change of boundary is recorded in the register before anything relies on it.
- *Finding.* A statement drawn from admitted sources, with an identifier `F-17-01-nnn`, the source IDs and headings it rests on, and a label saying whether it is a documented fact or an inference.
- *Open question.* Target behaviour that no admitted source settles, with an identifier `TBD-17-01-nnn`, the deliverables it affects and the route by which it can close.
- *Vendor question.* A question in the dossier, `CQ-17-01-nnn`, linked to an open question, with the current public basis, the deliverables affected and what it gates.
- *Closure rule.* An open question closes only on a recorded answer: its source and date, whether it is public-citable or private to support correspondence, the questions and deliverables it affects, and the decision on whether it may be used in public artifacts. A recorded project decision that selects a bounded profile is also an answer. Asking a question closes nothing.

**Findings in force.**

- MBF is the first solver-specific target: it is a documented text model input and output path and is closer to the target's model data than PCF translation.
- External execution of CAEPIPE is supported only as an optional harness around the user's own executable and licence.
- CAEPIPE CSV or text results may be parsed as regression and handoff evidence, with parser coverage stated section by section.
- PCF serves broader interoperability; its translation has mapping and default behaviour that makes it unsuitable as the first deterministic exchange.
- GLB and glTF are review-geometry targets, not solver geometry.
- An export reports what it exported, omitted, approximated, delegated or could not support, and what is undecided.

**Open questions.**

- `TBD-17-01-001`: which CAEPIPE version and profile is the first supported target (DEL-17-04, DEL-17-05).
- `TBD-17-01-002`: which MBF record families and fields the first deterministic writer subset requires (DEL-17-04).
- `TBD-17-01-003`: whether an MBF field can carry a stable canonical identifier, or identity is sidecar-only (DEL-17-02, DEL-17-04).
- `TBD-17-01-004`: which CSV result sections are stable enough for automated parsing (DEL-17-05, DEL-17-06).
- `TBD-17-01-005`: which PCF subset avoids hidden translator defaults (DEL-17-07).
- `TBD-17-01-006`: which geometry fields and identity metadata suffice for glTF review without implying solver fidelity (DEL-17-08).

Whether the bounded pilot selections of DEL-17-04 and DEL-17-05 are project decisions that close `TBD-17-01-001`, `TBD-17-01-002` and `TBD-17-01-004`, and who makes such a decision, is not decided.

**Boundary.** This deliverable implements no exporter, importer, parser, harness or schema. Its own files are the register and the dossier.

- What a format deliverable does with a source fact, and which bounded profile it selects, belongs to that deliverable (DEL-17-02 to DEL-17-09).
- The rule for what public text may say about the external prover and what stays private is DEL-01-02's (`docs/IP_AND_DATA_BOUNDARY.md`). The register and dossier follow it: a fact the public manual documents is cited to it; something learned from running the tool is never a finding of the register. It may appear only as the context of an open question or vendor question, stated plainly as observed format behaviour and without evaluation; otherwise it stays in private notes.
- The dossier holds the questions. Whether, how and to whom a question is sent is the owner's decision.

## Completion and Reliance Basis — Epistemology

### Requirements

- **REQ-001** — Every source the register admits is a public official document, a public format specification or a project-owned document, and each has a source ID, a location, an admitted use and a boundary.
- **REQ-002** — A CAEPIPE-specific fact is grounded in official public CAEPIPE documentation or carried as an open question.
- **REQ-003** — A PCF fact distinguishes documented translator behaviour from what SWBPIPE's exporter intends.
- **REQ-004** — glTF and GLB facts are limited to review-geometry semantics and imply no equivalence with solver geometry.
- **REQ-005** — Every target-format fact that a PKG-17 scope, schema, export profile or source-basis reference relies on resolves to a source ID or finding of the register, or to an open question it lists.
- **REQ-006** — Every non-trivial claim in the register and the dossier names its source path and its section or heading, or states that the location has not been narrowed.
- **REQ-010** — The source basis contains no CAEPIPE binary, copied commercial example, proprietary model file, reverse-engineered binary behaviour or licence-bypass instruction.
- **REQ-011** — The source basis contains no protected standards text or table, proprietary formula, standards-derived example, material allowable, SIF or flexibility table, or owner criterion.
- **REQ-012** — A CAEPIPE run, import, export or parsed result is described only as regression or handoff evidence and never as professional acceptance.
- **REQ-013** — A target solver option is described as pass-through target configuration and never as SWBPIPE code-checking logic.
- **REQ-014** — Unknown or version-sensitive target behaviour stays an open question, and an open question gates the target claim that depends on it in each deliverable it names.
- **REQ-015** — No vendor question asks for proprietary solver algorithms or other proprietary internals, hidden translator or binary-format internals, reverse-engineered behaviour, protected standards content, commercial or licensed example files, licence-bypass procedures, or code-compliance calculations.

### Acceptance criteria

- **AC-001** — Every row of the register's source tables has a source ID, an admitted use, a boundary and a location that resolves: a URL that answers or a path that exists. A source that no longer resolves is marked stale and nothing new relies on it. Every admitted source is of a kind REQ-001 allows.
- **AC-002** — Every finding cites at least one source ID present in the register, with its heading, and is labelled as documented fact or inference. Read against the cited heading, each fact says no more than the source does and stays within the source's consultation boundary.
- **AC-003** — Every source ID named in a PKG-17 scope, in `schemas/`, in `core/handoff/` or in the export fixtures is a source ID of the register.
- **AC-004** — Every open question names the deliverables it affects and its closure route, and every vendor question names its open question, its public basis and what it gates. A question recorded as closed carries the closure rule's record, and none is closed without it. This carries commitment DEL-17-01:1.
- **AC-005** — The register and the dossier contain nothing REQ-010 or REQ-011 names, describe external runs and parsed results only as REQ-012 allows, and describe target options only as REQ-013 allows. No statement in either claims compatibility with, validation by or acceptance through the target tool, and no vendor question asks for what REQ-015 excludes.

## Production and Verification Method — Praxeology

**Method choices that define the source basis.**

- A source is admitted for a stated subject, not as a whole document.
- Vendor documentation is paraphrased and cited by heading rather than copied in long excerpts; no vendor example is copied, and an interface fact quotes only the keywords, field names and short labels it needs.
- An inference is labelled as one and is never the sole ground of a target claim.

**How each criterion is examined.**

- **VER-001** — Executable check (observes the register's and dossier's tables and the source IDs used elsewhere): every row has its fields, every path exists, every finding's and question's identifiers resolve, and every source ID in the PKG-17 scopes, `schemas/`, `core/handoff/` and the export fixtures is a register ID; a seeded unknown ID and a seeded row without a boundary each fail it.
- **VER-002** — Judgment by a reviewer who did not write the register: examines each finding against the cited public page at the cited heading, each source's use against its consultation boundary, the reachability of each URL, and each closed question against the closure rule.
- **VER-003** — Executable check (the project's protected-content scan over this folder) and judgment by a reviewer who did not write the text: examines the register and the dossier for the content REQ-010 and REQ-011 exclude and for claims REQ-012 and REQ-013 exclude, and each vendor question for what REQ-015 excludes.

## Governing Values and Decisions — Axiology

- **AX-001** — *Sourced, not inferred.* Better is a larger share of the target-format statements across PKG-17 that cite an admitted source at a located heading, and fewer open questions. This is never bought by reading a source beyond its boundary, by inferring behaviour from the target's name or common practice, or by drawing a public statement from the tool's private outputs.
- **AX-002** — *Narrow admission.* Better is a smaller and more exactly bounded admitted use for each source. No proprietary, protected or uncleared material is admitted to close a question sooner.
- **AX-003** — *Questions stay visible.* Better is fewer places where a PKG-17 deliverable relies on target behaviour without naming the open question that governs it. A question is never softened into support language, and it is never closed by being asked.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-030 SOW-075 OBJ-009 OBJ-018 | REQ-001 REQ-006 | AC-001 | VER-001 VER-002 | Outside source: the public pages and project files the rows name; the comparison is against those pages and paths, not against the register |
| OUT-001 | SOW-030 SOW-074 OBJ-009 OBJ-017 | REQ-002 REQ-003 REQ-004 REQ-006 | AC-002 | VER-002 | Outside source: the cited vendor manual pages, the PCF translator document and the Khronos glTF 2.0 specification, read at the cited headings |
| OUT-001 | SOW-030 SOW-074 OBJ-009 OBJ-017 | REQ-005 | AC-003 | VER-001 | Owner ruling: the decomposition makes this register the source basis for PKG-17; the comparison is against schemas, code and scopes this deliverable did not write |
| OUT-001 | SOW-075 OBJ-017 OBJ-018 | REQ-014 | AC-004 | VER-001 VER-002 | Owner ruling: decomposition OI-017 gates target claims on gathered answers; a closure is compared with the answer's own source |
| OUT-001 | SOW-075 OBJ-018 | REQ-010 REQ-011 REQ-012 REQ-013 REQ-015 | AC-005 | VER-003 | Owner ruling: `docs/IP_AND_DATA_BOUNDARY.md` and `docs/PRD.md` §20.1 and §21.2 state what is excluded, and REQ-015 states what a vendor question may not ask; the scan's markers are synthetic |
