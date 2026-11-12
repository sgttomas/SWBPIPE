---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-13-03
package_id: PKG-13
decomposition_basis: execution/_Decomposition/SOFTWARE_DECOMP.md
project_scope_refs: [SOW-068]
package_objective_refs: [OBJ-014]
---

# Scope of Work — DEL-13-03 Constraint validation engine

## Purpose and Objective Traceability

The engine checks the design against what has been recorded about it and says what it finds. Given the physical model, the design knowledge and the constraints, it reports each place where a recorded constraint is not met: parts that should connect and do not, a route through a no-go volume, a clearance not kept, a support outside a supportable zone, a slope, drain or vent requirement not satisfied. It reports as plainly each constraint it could not check because a value it needs is missing. Every message names the entities concerned and the source of the constraint and of the data it used, and the same inputs always give the same messages. It checks only what the user recorded: it supplies no limit, infers no owner standard or code requirement, and accepts nothing.

It serves SOW-068 under OBJ-014 (`execution/_Decomposition/SOFTWARE_DECOMP.md`, PKG-13, row DEL-13-03), and PRD §13.2 and FR-KNOW-003, with §5.5 for missing data. Its result is the constraint validation an operation must pass before it is applied (PRD FR-AGENT-002; DEL-16-02). PKG-13 is outside v1.0, and no exit criterion of `docs/RELEASE_TARGET_v1.0.md` names this deliverable.

- **OUT-001** — The constraint validation engine, and the validation result it returns with that result's versioned schema.

## Deliverable Definition — Ontology

**Terms the engine implements.**

- *Validation.* The evaluation of a constraint set against the physical model and the design knowledge supplied with it. It reads; it changes none of the three.
- *Available data.* What the model, the design knowledge and the constraints state. A constraint is evaluated only from it.
- *Finding.* One validation message: a diagnostic with a stable code, a class, a severity, the entities it concerns, the sources it rests on, a message and a remediation. The classes are those of the constraint format (DEL-13-02).
- *Violation.* A finding that an evaluated constraint is not met, in the class of the constraint's category.
- *Missing-data finding.* A finding that a constraint could not be evaluated because a target, a design-knowledge record, a parameter, a unit or a dimension it needs is absent or unresolved. It names what is missing.
- *Source reference.* The provenance of the constraint and of each design-knowledge record a finding used, as supplied.
- *Per-value trace link.* A link a finding carries for one value it used: the path of that value in its source record paired with the path of the value in its target record. A path is never given without its pair.
- *Severity.* Blocking, warning or information.
- *Validation result.* The findings of one validation in a stable order, with a statement of whether any of them is blocking. Its schema is this deliverable's.

**Undecided.**

- How a geometric conflict is decided: the containment and distance rules and their tolerance. It waits on the coordinate frame and tolerance of design-knowledge geometry, which DEL-13-01 leaves undecided.
- Which violations block and which warn.
- Whether connectivity here still includes the model topology that DEL-04-06 checks before every solve (disconnected parts, malformed branches), or only the connectivity a recorded constraint or design knowledge states.
- The wire form of the validation result: its envelope, fields and version.
- Which record a per-value trace link of a validation finding takes as its target.

**Boundary.** The engine evaluates and reports. The constraint format and its finding classes are DEL-13-02's; the design-knowledge format is DEL-13-01's; the model document is DEL-02-01's. The topology checks of the solve are DEL-04-06's. The diagnostic record and the result envelope are DEL-00-06's. Showing findings in the desktop is DEL-07-08's, under DEL-07-04's presentation rules, and holding an operation back on a blocking finding is DEL-16-02's. Whether the desktop blocks anything else on a blocking finding is not decided. The engine states no criterion: every limit is a parameter of a constraint the user recorded.

## Completion and Reliance Basis — Epistemology

A finding is a statement about the recorded constraints and the data supplied. The absence of a finding establishes nothing about a condition nobody recorded, and no validation result is an engineering acceptance.

### CLM-013 — Requirements

Requirement numbers 001 to 007 are those of the earlier `DEL-13-03-Rn` series.

- **REQ-001** — The engine validates available design knowledge for the categories of SOW-068: connectivity, route conflicts, clearance conflicts, support-zone conflicts, slope, drain and vent conflicts, and missing required data (PRD FR-KNOW-003).
- **REQ-002** — Its outputs are deterministic validation messages. A constraint failure is never left to report prose or agent text (SOW-068; PRD §13.2).
- **REQ-003** — Missing required data is an explicit finding and is never defaulted (PRD §5.5; `docs/CONTRACT.md` OPS-K-DATA-2).
- **REQ-004** — A message shows the source of the constraint and of the design knowledge it used, wherever the source or the availability of that knowledge affects reliance on the message (SOW-068).
- **REQ-005** — The engine infers no hidden owner standard, protected code requirement, protected or proprietary value, or engineering acceptance logic (`docs/CONTRACT.md` OPS-K-IP-1, OPS-K-DATA-1, OPS-K-AUTH-1).
- **REQ-006** — Its outputs follow the diagnostic record and the result-envelope contract (decomposition AB-00-06).
- **REQ-007** — The engine's behaviour in every category and at each boundary condition is examined on invented inputs.
- **REQ-009** — This deliverable owns the validation result as a production contract with one authoritative versioned schema, and validation is reached through an application service that returns it. The engine's in-memory result is not that contract. The result carries per-value trace: a finding that used a value of the model or the design knowledge carries a per-value trace link for it (also cited as DEL-13-03-SCA011-REQ-001, DEL-13-03:1 and DEL-13-03:2).

### Acceptance criteria

- **AC-001** — For each category of REQ-001, given a model, design knowledge and a constraint in force whose data is complete: a design that does not meet the constraint yields a violation of that category's class naming the constraint and the entities that violate it, and a design that meets it yields no violation.
- **AC-002** — The same model, design knowledge and constraints give the same findings, with the same codes, in the same order, on every run, and the three inputs are unchanged by validation.
- **AC-003** — A constraint that lacks a target, a design-knowledge reference, a parameter, a unit or a dimension, or that names a record not supplied, yields a missing-data finding that names what is missing. The constraint is not evaluated, and no value is substituted. A constraint or design-knowledge document that does not conform to its format yields a schema finding that names the fault, and nothing in it is evaluated as if it conformed.
- **AC-004** — Every finding names the entities it concerns and carries the source of its constraint and of each design-knowledge record it used, equal to the provenance supplied.
- **AC-005** — The engine holds no limit or criterion value of its own: with a constraint whose limit parameter is absent it reports missing data and neither a violation nor a pass. No finding text quotes a standard or states acceptance, approval, certification or code compliance.
- **AC-006** — Every finding has the fields of the diagnostic contract and a class of the constraint format. The result states whether any finding is blocking, and that statement is true exactly when a finding of blocking severity is present.
- **AC-007** — The result an application service returns for a validation validates against this deliverable's versioned schema and is carried in a result envelope with its outcome kind.
- **AC-008** — The inputs of every check of the engine are invented and contain no protected or private content.
- **AC-009** — Each finding that used a value of the model or the design knowledge carries a per-value trace link for that value. Its source path and its target path are both present and non-empty, and each resolves in the record it names. No link carries one path without the other.

## Production and Verification Method — Praxeology

**Method choices that define the engine.**

- The engine reads and reports. It never changes the model, the design knowledge or the constraints, and it repairs nothing.
- Every limit is a parameter of a constraint, with its unit. Quantities are compared only after conversion through the unit catalog (DEL-02-02); the engine has no conversion of its own.
- A constraint that cannot be evaluated is reported, never skipped.
- Findings carry stable codes and are returned in a stable order.

**How each criterion is examined.**

- **VER-001** — AC-001, comparison with a named reference: constructed cases whose outcome follows in closed form from their geometry and stated limits (a point inside or outside a box, the distance from a point to a segment, the gradient between two elevations, the connected parts of a graph), worked without product code.
- **VER-002** — AC-002, AC-003, AC-004 and AC-006, executable check: observes the findings for constructed inputs across repeated runs, the inputs before and after, each missing-data case, the source references against the provenance supplied, and the blocking statement against the severities present.
- **VER-003** — AC-005 and AC-008, judgment by a reviewer who did not write the engine, with the protected-content scan: examines the engine's source for criterion values, the text of every finding, and the inputs of the checks; with an executable check that observes the missing-limit case.
- **VER-004** — AC-007, executable check: observes the result an application service returns under a JSON Schema 2020-12 validator that imports no product code.
- **VER-005** — AC-009, executable check: observes, for constructed inputs, the trace links of each finding that used a value, each path resolved in the record it names, and that no link carries an unpaired path.

## Governing Values and Decisions — Axiology

- **AX-001** — *Check what is recorded, and only that.* Better is more of the recorded constraints actually evaluated from the data supplied. A limit is never supplied, and an owner's or a code's requirement never inferred, to evaluate one more.
- **AX-002** — *Missing data is a finding.* Better is fewer constraints that pass through validation with neither a result nor a named reason; the target is none. A constraint is never treated as met because it could not be checked.
- **AX-003** — *Reproducible and traceable.* Better is a result a reviewer can reproduce and trace: the same findings on every run, each leading to its constraint and its sources. Findings are never merged, reordered by chance or summarized away to shorten a result.
- **AX-004** — *A message, not a verdict.* A finding supports review. Clarity for the user never justifies quoting a protected standard or a private criterion, and no result reads as acceptance.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-068 OBJ-014 | REQ-001 REQ-007 | AC-001 | VER-001 | Outside source: closed-form geometry and graph connectivity of the constructed cases, worked without product code; each limit is stated in the constructed constraint |
| OUT-001 | SOW-068 OBJ-014 | REQ-002 | AC-002 | VER-002 | The criterion fixes the expected relation: equal findings on every run and unchanged inputs. It compares the product with itself and guards reproducibility only |
| OUT-001 | SOW-068 OBJ-014 | REQ-003 REQ-006 | AC-003 | VER-002 | The inputs are constructed, so the criterion states the expected finding (PRD §5.5: nothing is defaulted); conformance is judged against the schemas of DEL-13-01 and DEL-13-02 |
| OUT-001 | SOW-068 OBJ-014 | REQ-004 | AC-004 | VER-002 | The criterion fixes the expected result: the source references equal the provenance in the constructed inputs |
| OUT-001 | SOW-068 OBJ-014 | REQ-005 | AC-005 | VER-003 | The package excludes owner standards, code data and acceptance logic (decomposition PKG-13), and PRD §21.2 lists the prohibited claims; the reviewer compares against sources the product did not write |
| OUT-001 | SOW-068 OBJ-014 | REQ-006 | AC-006 | VER-002 | The diagnostic fields are those of the architecture basis AB-00-06 in the decomposition; the classes are DEL-13-02's; the blocking statement is fixed by the criterion |
| OUT-001 | SOW-068 OBJ-014 | REQ-009 | AC-007 | VER-004 | Outside source: the JSON Schema 2020-12 specification, applied by a validator that imports no product code, against the schema this deliverable publishes. The schema's fields await the undecided wire form |
| OUT-001 | SOW-068 OBJ-014 | REQ-005 REQ-007 | AC-008 | VER-003 | PRD §20.1 and `docs/IP_AND_DATA_BOUNDARY.md` state what a public artifact may contain; the reviewer compares against sources the product did not write |
| OUT-001 | SOW-068 OBJ-014 | REQ-009 | AC-009 | VER-005 | The inputs are constructed, so the values each finding used and their paths are known from the inputs and not from the product's output; the pairing rule is fixed by the criterion |
