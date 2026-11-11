---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-04-06
package_id: PKG-04
decomposition_basis: execution/_Decomposition/SOFTWARE_DECOMP.md
project_scope_refs: [SOW-053,SOW-035]
package_objective_refs: [OBJ-003,OBJ-008,OBJ-012]
---

# Scope of Work — DEL-04-06 Solver diagnostics and singularity detection

## Purpose and Objective Traceability

Every solve and pre-solve check that cannot give a determinate, converged mechanics result says so with a deterministic diagnostic naming its cause, severity and affected entity, and the solver never reports more than a mechanics-only status. This deliverable makes the solver say what is wrong. When a model cannot be solved, or can be solved only with doubt, it gives the user a named, located statement: this model is unrestrained, this node has no support, this system is ill-conditioned, this case did not converge, these two nodes nearly coincide. Without it a bad model yields either a crash or numbers that look like results. With it the user can find and fix the cause, and a published result carries its warnings.

It serves SOW-053 and SOW-035 under OBJ-003, OBJ-008 and OBJ-012 (`execution/_Decomposition/SOFTWARE_DECOMP.md`, PKG-04, row DEL-04-06), and PRD §12.8, §12.2 (singularity and ill-conditioning detection) with FR-SOL-015, FR-KNOW-002 and FR-GUI-006, and the PRD §24 phase exit "singular model detection implemented". Under SOW-035 its diagnostics are reproducible: the same input gives the same records in the same order. No exit criterion of `docs/RELEASE_TARGET_v1.0.md` names it; it supports criterion 3, since every refusal a user meets is named through these records.

- **OUT-001** — The solver diagnostics: the diagnostic record, the detection and classification of singular, ill-conditioned, non-converged and invalid-model states, the model topology checks, and the on-demand model checks in the desktop.

## Deliverable Definition — Ontology

**Terms the diagnostics implement.**

- *Diagnostic.* A record with a machine-readable code, class, severity and source, a message, and, where they apply, the affected object, a reference to the model entity, a remediation, provenance and the units of any quantity it quotes. An affected reference names a degree of freedom (`dof:*`), a support, an element, or a load, including a load whose object is missing (`load:<missing-id>`).
- *Classes and severities.* A diagnostic is solve-blocking, an assumption warning, a nonlinear warning, or a statement that a policy is undecided; its severity is failure (the solve could not complete), blocking (the model is refused before solving), warning or information.
- *Solver status.* Of a solve, one of two mechanics states: the model is incomplete or the solve failed (`MODEL_INCOMPLETE`); or the mechanics were solved, possibly with warnings (`MECHANICS_SOLVED`). It is a solver result only: it carries no rule-check outcome and no human acceptance, and review by a person remains required.
- *Singular system.* A system the solver cannot solve because the model can move without resistance: an unrestrained model or part, or a mechanism. The diagnostic names the degree of freedom at which it was found.
- *Ill-conditioned system.* A reduced system whose condition estimate exceeds the warning level; beyond the failure level it is a conditioning failure.
- *Non-convergence.* An iterative solve whose residual is above its tolerance: a warning before the iteration limit, a failure at it.
- *Topology finding.* A warning about a model that can be formed but is unusual: parts not connected to each other, distinct nodes that coincide or nearly coincide, or a malformed branch.
- *Node-proximity tolerance.* The distance at or below which two distinct nodes not joined by an element are reported as near-duplicates: an absolute length, 5 mm unless the user sets another value for the model.
- *Model checks.* The checks the solve itself runs before forming a stiffness, run on the user's request without solving.

**Boundary.** Diagnostics report; they solve nothing and decide no tolerance. The frame kernel (DEL-04-01), supports (DEL-04-03, DEL-04-04) and loads (DEL-05-01) detect their own failures and findings, which this deliverable maps to records. This deliverable defines what a diagnostic says; DEL-04-07 raises the records in the product solve and carries them to the result unchanged. DEL-05-04 owns the analysis status semantics the solver status maps into. The results viewer and exports (PKG-07, PKG-08) present them. The model-document member that stores the node-proximity tolerance is DEL-02-01's.

## Completion and Reliance Basis — Epistemology

A diagnostic is a statement about the discrete model and its solve. The absence of a diagnostic establishes nothing about the physical system.

### Requirements

- **REQ-001** — Singular, ill-conditioned, non-converged, invalid-restraint, invalid-topology, invalid-numeric-input and undecided-policy states each produce a deterministic diagnostic (SOW-053; PRD FR-SOL-015).
- **REQ-002** — A diagnostic carries its code, class, severity, source and message and, where they apply, its affected reference, model-entity reference and remediation, with provenance and quantity units.
- **REQ-003** — Provenance, remediation, the model-entity reference and unit metadata remain available on the diagnostic wherever it is carried (`docs/CONTRACT.md` OPS-K-UNIT-1).
- **REQ-004** — The solver status maps mechanics states only: incomplete or failed to "model incomplete", solved or solved with warnings to "mechanics solved", as a solver result only, with human review required, no rule status and no human acceptance (`docs/CONTRACT.md` OPS-K-MECH-2, OPS-K-AUTH-1).
- **REQ-005** — A kernel failure maps to a blocking diagnostic: a singular system, an invalid restraint, invalid topology or orientation, invalid numeric input, or mismatched matrix and vector dimensions (SOW-053).
- **REQ-006** — Support errors and findings map to blocking model-validation diagnostics that name the support and, where the error concerns a degree of freedom, that degree of freedom (`docs/CONTRACT.md` OPS-K-DATA-2).
- **REQ-007** — Load errors and findings map to blocking model-validation diagnostics, including a load that names something missing (`docs/CONTRACT.md` OPS-K-DATA-2).
- **REQ-008** — An ill-conditioned reduced system is detected and reported: its condition estimate, classified against a warning level and a failure level, gives an ill-conditioning warning beyond the warning level and a blocking conditioning-failure diagnostic beyond the failure level, each naming the case, and a non-finite estimate is rejected (SOW-053; PRD FR-SOL-015, §12.2). DEL-04-07 raises the diagnostic in the product solve. The warning and failure levels for product solves are not yet decided.
- **REQ-009** — A residual and tolerance are classified as no diagnostic, a non-convergence warning or a non-convergence failure, by iteration count against the supplied limit (`docs/CONTRACT.md` OPS-K-SOLVER-2).
- **REQ-010** — A solve whose tolerance or convergence policy has no governed value says so in a named warning. A settled choice is never reported as undecided.
- **REQ-011** — No diagnostic states or implies certification, approval, professional reliance, release readiness or code compliance (PRD §21.2).
- **REQ-012** — *Topology checks.* Before solving, on both solve routes, and without refusing a solve or changing any published value, the product warns when the model's elements form two or more disconnected parts, naming each and whether it is supported; when distinct nodes that no element joins coincide or lie no farther apart than the node-proximity tolerance; and when a branch component's node does not join exactly three pipes or its header and branch name the same pipe. The branch warning states plainly which of the component's branch stress factors are still applied, naming each pipe, and names exactly the sides whose intensified results the solve publishes; a refused result names none. The user can change the tolerance for the model (PRD FR-KNOW-002, §12.8, §14.4).
- **REQ-013** — *On-demand model checks.* On the user's request, without solving and without changing the model, the desktop runs the solve's own checks before stiffness formation (input validation, the restraint pre-checks and the topology checks), with the model-data slot checks and the library-reference findings, and lists each finding with its own code, severity and text. Choosing a finding that names a model entity selects it, and the command has a keyboard equivalent. The requirement further fixes:
  - *Agreement with the solve.* Each solve finding matches the corresponding leading diagnostic of a solve in id, code, severity, source and affected references, and in text before any post-solve statement of applied branch stress factors is appended.
  - *Memory.* The checks run under the memory admission of a solve in the app (a capped worker within the per-invocation budget), and under a stated input size bound in the browser preview. Library-reference findings are read only after that call returns a report from an admitted or bounded check.
  - *Bounded report.* All three groups share one bounded allowance for serialized findings: findings beyond it are counted, not listed; long messages and model-read refusal text are shortened; library messages are kept whole when their findings fit; ids and references are kept whole.
  - *Cancellation.* A check in progress is cancelled, and its memory released, when the project session changes or a newer check is requested (PRD FR-KNOW-002, FR-GUI-006).

**Undecided.** Whether the numerical-integrity diagnostic that the solve integration and the retained-precision method raise for a sensitive case already meets REQ-008's obligation.

### Acceptance criteria

- **AC-001** — Models singular by construction are each detected and named: a model with no support, a part with no support, and a mechanism give a blocking diagnostic that locates the unrestrained degree of freedom, and no result is published for them. The same holds for a zero-length element and an invalid orientation.
- **AC-002** — Each kernel, support and load failure maps to the diagnostic this deliverable defines, with its code, class, severity, source and affected reference: a kernel failure to a blocking diagnostic, a support error or finding to a blocking model-validation diagnostic that names the support and, where the error concerns a degree of freedom, that degree of freedom, and a load error or finding to one that names the load, including a missing one.
- **AC-003** — Every diagnostic of every kind carries the fields of REQ-002, and the same input gives the same diagnostics in the same order.
- **AC-004** — Against supplied warning and failure levels, a condition estimate beyond the warning level gives the ill-conditioning warning and beyond the failure level the blocking conditioning-failure diagnostic, each naming the case; an estimate within the warning level gives neither, and a non-finite estimate is rejected.
- **AC-009** — A nonlinear solve whose residual is above its tolerance carries a non-convergence warning before the iteration limit and a failure-level non-convergence diagnostic at it; a residual within tolerance gives none.
- **AC-005** — The solver status of an incomplete or failed solve is "model incomplete" and of a solved one "mechanics solved", and no status or diagnostic carries a rule outcome, an acceptance or a compliance statement.
- **AC-006** — A case solved without a governed tolerance carries the named warning, and no diagnostic describes the sparse solver strategy as undecided.
- **AC-007** — On both routes: a connected model and an isolated anchored node give no topology finding; two supported parts warn and solve to the values each part gives alone; an unsupported part is named and still refused by the solve's own checks; nodes that coincide, or lie at or within the tolerance, warn in any length unit, and nodes just beyond it do not; a user-set tolerance replaces 5 mm; a branch on two pipes, or whose header is its branch, warns and a valid one does not; a malformed branch's warning names exactly the sides whose intensified results are published, under every combination of header and branch factor, and a refused result names none. No topology finding changes a published value.
- **AC-008** — For every model, the on-demand findings equal the solve's leading diagnostics as REQ-013 states, and when a finding is blocking the solve publishes exactly those findings. The model is unchanged by a check; a finding that names an entity selects it; the report stays within its bound whatever the model, with omitted findings counted; a check over the memory budget is refused by name; and a superseded check is cancelled.

## Production and Verification Method — Praxeology

**Method choices that define the diagnostics.**

- Detection belongs to the code that meets the condition; this deliverable owns the vocabulary and the mapping, so that one condition has one name wherever it is met.
- Thresholds and tolerances are inputs to classification. The diagnostics hold none.
- A topology finding is a warning, never a refusal: a model that can be formed but is unusual may be intended. Topology that prevents the mechanics from being formed is refused by the solve's own checks.
- The node-proximity tolerance is an absolute length, so the finding does not depend on the model's overall size.
- The on-demand checks call the solve's own check code and stop where the solve would begin forming a stiffness; they are not a second implementation.

**How each criterion is examined.**

- **VER-001** — AC-001, AC-003, AC-004, AC-006 and AC-009, executable check: observes, for models singular or invalid by construction, the diagnostic raised, its location and the absence of a result; and, for each classification, the diagnostic at, below and above its supplied limit.
- **VER-002** — AC-002 and AC-005, executable check: observes each failure kind mapped to its record, field by field, and the status of a failed and of a solved case.
- **VER-003** — AC-007, executable check: observes the topology findings on constructed models in both solve modes and on both routes, with the published values compared against the same model without the unusual feature.
- **VER-004** — AC-008, executable check: observes the on-demand findings against the solve's leading diagnostics across a set of invented models, the unchanged model, the bounded report, the memory refusal and the cancellation.
- **VER-005** — AC-005, judgment by a reviewer who did not write the code: examines diagnostic texts against `docs/claims_registry.md`.

## Governing Values and Decisions — Axiology

- **AX-001** — *Name the cause where the user can act on it.* Better is a diagnostic that names the cause and the affected entity earlier and more specifically, so that fewer refused or warned solves leave the user without a cause. A more helpful message never guesses: it states only what the check established, and it never costs determinism or changes a published value.
- **AX-002** — *Never a quiet failure.* Better is fewer faults that reach a user as a crash, a generic error or a plausible number. A diagnostic is never downgraded or dropped to let a result through, and a refused or unconverged model is never reported as solved.
- **AX-003** — *Warn, do not obstruct.* A model the mechanics can form is solved, with its warnings. Usefulness of a warning never justifies refusing a model the user may intend.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-053 OBJ-003 OBJ-008 OBJ-012 | REQ-001 REQ-005 | AC-001 | VER-001 | Outside source: closed-form rank deficiency of rigid-body and mechanism models, which are singular by construction |
| OUT-001 | SOW-053 OBJ-003 OBJ-008 OBJ-012 | REQ-005 REQ-006 REQ-007 | AC-002 | VER-002 | The criterion states the expected record for each failure kind (`docs/CONTRACT.md` OPS-K-DATA-2) |
| OUT-001 | SOW-053 SOW-035 OBJ-003 OBJ-008 OBJ-012 | REQ-001 REQ-002 REQ-003 | AC-003 | VER-001 | The criterion states the expected fields; identical ordering on repeat runs is the product's own earlier output and only guards against change |
| OUT-001 | SOW-053 SOW-035 OBJ-003 OBJ-008 OBJ-012 | REQ-008 | AC-004 | VER-001 | Outside source: matrices of known condition number built by construction, classified against supplied levels. The levels a product solve uses are not decided, so this does not establish that a product solve warns at an adequate level |
| OUT-001 | SOW-053 OBJ-003 OBJ-008 OBJ-012 | REQ-004 REQ-011 | AC-005 | VER-002 VER-005 | PRD §21.2, `docs/CONTRACT.md` OPS-K-MECH-2 and `docs/claims_registry.md`, with the reviewer's reading of the texts |
| OUT-001 | SOW-053 OBJ-003 OBJ-008 OBJ-012 | REQ-010 | AC-006 | VER-001 | The settled solver strategy (DEC-023, DEC-053) and the tolerance policy (DEC-026, DEC-046) state what is and is not decided |
| OUT-001 | SOW-053 OBJ-003 OBJ-008 OBJ-012 | REQ-012 | AC-007 | VER-003 | The geometry is constructed, so the criterion states the expected findings; the 5 mm default and its inclusive boundary, the warning-only reading of PRD §14.4 and the applied-sides statement are requirements the check confirms, not evidence of their adequacy. Values unchanged by a finding compare the product with its own output and only guard against change |
| OUT-001 | SOW-053 OBJ-003 OBJ-008 OBJ-012 | REQ-013 | AC-008 | VER-004 | A consistency check between the on-demand check and the solve, both the product's own output: it only guards against change, and no numerical correctness is claimed |
| OUT-001 | SOW-053 OBJ-003 OBJ-008 OBJ-012 | REQ-009 | AC-009 | VER-001 | `docs/CONTRACT.md` OPS-K-SOLVER-2 and DEC-046; the residual, tolerance and limit are constructed inputs |
