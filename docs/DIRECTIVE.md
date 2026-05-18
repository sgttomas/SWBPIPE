---
doc_id: OPS-DIRECTIVE
doc_kind: governance.directive
status: draft
created: 2026-04-30
refs:
  - rel: informs
    to: OPS-CONTRACT
  - rel: informs
    to: OPS-SPEC
  - rel: informs
    to: OPS-SOFTWARE-DECOMP
---

# DIRECTIVE — OpenPipeStress Founding Intent and Agentic Development Constraints

## 1. Founding intent

OpenPipeStress exists to make global piping flexibility and stress analysis more transparent, auditable, educational, and accessible without redistributing protected standards content or pretending that software can replace responsible engineering judgment.

The project shall implement open analytical mechanics and a modern software workflow. The responsible user supplies code-specific data, material allowables, proprietary component data, stress-intensification and flexibility factors, load combinations, owner requirements, and final professional judgment.

The technical lineage is the classical centerline/flexibility model of piping systems: connected pipe members, local component corrections, loads, restraints, moments, reactions, and stress recovery. Modern implementation should use robust numerical methods, but it should not convert ordinary global pipe stress analysis into routine 3D solid meshing.

## 2. Design philosophy

### 2.1 Ontology — what exists?

The software must make engineering objects explicit:

- projects, models, nodes, elements, components, materials, sections;
- supports, restraints, loads, load cases, combinations;
- solver results, stress results, reactions, warnings;
- rule packs, private libraries, source/provenance records, reports.

Each object must have stable identity, unit-aware fields, and provenance where engineering reliance may be affected.

### 2.2 Epistemology — what is warranted?

The software must distinguish:

- calculated mechanical results;
- user-entered code data;
- user-defined rule-pack checks;
- missing or unverified data;
- human professional approval.

A result can be mechanically solved without being ready for rule-checking. A rule-pack pass/fail can be computed without being a professional code-compliance certification. Missing information is a finding, not a problem to hide.

### 2.3 Praxiology — how is the work done?

Development work is decomposed into flat packages and bounded deliverables. Downstream agents execute only sealed, explicit deliverables. Solver changes, rule-engine changes, GUI workflows, and reports must be tested and reviewed before release.

### 2.4 Axiology — what values govern?

Public safety, professional responsibility, evidence, auditability, and respect for intellectual property govern the project. Convenience never justifies silent engineering defaults, hidden assumptions, protected data leakage, or misleading compliance claims.

## 3. Non-negotiable product principles

1. **Open mechanics, private code data.** The public repository ships solver mechanics, schemas, workflows, and invented examples; users supply code-specific and proprietary values.
2. **No silent engineering defaults.** Missing values required for solving or rule checking are visible and classified.
3. **Centerline global model first.** The primary solver is a 3D line-element/frame model. Local shell/solid FEA is a specialized handoff path.
4. **Unit safety.** All models, results, imports, exports, and formulas are unit-aware.
5. **Provenance by design.** Material, component, rule, allowable, SIF, flexibility, and report data carry source/provenance fields.
6. **Human authority.** The software may compute; it does not certify, seal, approve, or authenticate engineering work.
7. **Data boundary enforcement.** Protected standards tables, code text, copyrighted examples, and proprietary commercial data are not accepted into the public repository.
8. **Validation before reliance.** Solver features must have tests, benchmark cases, and release quality gates.

## 4. Scope

### 4.1 In scope

- Open-source global piping flexibility and stress analysis.
- 3D centerline/frame solver.
- Piping-specific component and support models.
- User-defined rule-pack system.
- Private material/component libraries.
- GUI modeler and results viewer.
- Auditable reports and reproducibility artifacts.
- Verification benchmarks and validation manual.
- Agentic development decomposition and governance.

### 4.2 Out of scope

- Redistribution of ASME or other protected standards content.
- Automatic legal/code interpretation.
- Claiming ASME approval, certification, endorsement, or official compliance.
- Replacing the engineer of record.
- Routine global 3D solid modeling as the default pipe stress method.
- Public distribution of vendor data without permission.
- Hidden cloud storage or telemetry of private project/rule data.

## 5. Authority and stop rules

Agents and software must stop and escalate to a human when:

- a required engineering value is missing and cannot be inferred from an authorized source;
- a source appears to contain protected standards or proprietary data intended for public contribution;
- two sources conflict on a code-relevant or safety-relevant value;
- a proposed feature would blur the boundary between computed rule-pack status and professional code compliance;
- a solver or rule-engine change cannot be verified with available tests;
- a user request would require copying protected code text, tables, figures, or examples into public artifacts;
- a claim would overstate what the available evidence supports.

## 6. Governance baseline

OpenPipeStress is intended to be a free and open-source project. The exact open-source license, contributor certification mechanism, release signing process, maintainer roster, and maintainer quorum remain `TBD` until the human project authority records those decisions.

Governance artifacts must preserve these boundaries:

- Maintainer approval is project governance only; it is not professional engineering approval of a piping calculation.
- Release labels describe software maturity and validation evidence only; they must not imply code compliance, certification, endorsement, sealing, or project-specific engineering acceptance.
- Public contribution review must check source, provenance, redistribution rights, protected-content risk, private-data risk, and test evidence before merge.
- Public releases must include known limitations, validation status, data-boundary notices, and professional-responsibility notices.
- Private rule packs, material data, component data, owner standards, and project models remain user-controlled unless the user intentionally exports or contributes them with documented rights.

The maintainer policy skeleton is `governance/MAINTAINERS.md`. It records active governance roles, decision surfaces, release gates, and unresolved `TBD` policy choices.

## 7. Reference basis

- `docs/INTENT.md` and `docs/PRD.md` define the product intent and product requirements.
- `docs/CONTRACT.md` and `docs/TYPES.md` define the current professional-accountability vocabulary and invariant boundaries; the dedicated professional-responsibility policy remains future `DEL-01-04` work.
- `agents/AGENT_SOFTWARE_DECOMP.md` defines the decomposition method: flat packages, small deliverables, stable IDs, context envelopes, and coverage telemetry.
- `governance/MAINTAINERS.md` defines the current maintainer-policy skeleton and records unresolved governance decisions as `TBD`.
