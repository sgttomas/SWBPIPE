# Plan - Domain Engine Integration and OpenPipeStress Adoption

**Status:** Planning record grounded in authoritative source documents; not yet an implementation specification  
**Date:** 2026-05-04  
**Repository:** `chirality-app-dev`  
**Primary sources:** the three documents in `docs/thesis/bigger-picture/` listed below  
**Source authority:** These three source documents are authoritative for Domain Engine Integration intent, requirements, and roadmap unless and until the product owner declares otherwise.  
**Primary intent:** Capture, organize, and trace the product direction, architectural boundaries, implementation sequence, risks, and open decisions stated in the three authoritative source documents.

---

## 1. Purpose

This plan records the intended direction stated in the three authoritative documents in `docs/thesis/bigger-picture/`:

- `CHIRALITY_PRD_Amendment_Domain_Engine_Integration.md`
- `Chirality_OpenPipeStress_Integration_Plan.md`
- `Chirality_OpenPipeStress_Bigger_Picture_Development_Plan.md`

The documents collectively describe a major product extension: Chirality should support deterministic specialist domain engines through profiles, tool adapters, read boundaries, protected write paths, operation proposals, human gates, and auditable filesystem artifacts.

This plan is intended to preserve and operationalize that authoritative intent without silently editing the existing governed specification set. It should serve as the bridge between the source documents and later governed changes to `docs/PRD.md`, `docs/SPEC.md`, `docs/TYPES.md`, `docs/CONTRACT.md`, agent instructions, tools, frontend APIs, examples, or OpenPipeStress-side artifacts.

This distinction matters:

- the three source documents are authoritative for the intended Domain Engine Integration direction;
- this `plans/` document is a traceable planning record based on them;
- existing governed Chirality docs continue to describe the currently governed system until they are explicitly amended;
- any conflict between this plan and the source documents should be resolved in favor of the source documents unless the product owner declares otherwise.

---

## 2. Authoritative Source Basis

### 2.1 Source Document Aliases

This plan uses the following aliases for traceability:

| Alias | Source document | Primary role in this plan |
| --- | --- | --- |
| `PRD-AMEND` | `docs/thesis/bigger-picture/CHIRALITY_PRD_Amendment_Domain_Engine_Integration.md` | Product requirements, definitions, scope additions, API additions, profile shape, agent behavior rules, release strategy, adoption acceptance criteria. |
| `OPS-INT` | `docs/thesis/bigger-picture/Chirality_OpenPipeStress_Integration_Plan.md` | Concrete Chirality/OpenPipeStress architecture, integration levels, adapter contract, operation proposal schema, guardrails, backlog epics, tests, implementation sequence. |
| `ECOSYS-PLAN` | `docs/thesis/bigger-picture/Chirality_OpenPipeStress_Bigger_Picture_Development_Plan.md` | Larger ecosystem strategy, system roles, program phases, workstreams, release roadmap, MVP definition, technical decisions, risks, gates, long-term vision. |

Note on PRD filename: `PRD-AMEND` says it applies to `PRD_CHIRALITY_APP.md`. That filename is intentional source terminology: it is the product owner's name for this repo's PRD in a duplicate-filename working context. If this workspace also contains `docs/PRD.md`, later promotion work should reconcile the local file path with that intended PRD name instead of treating `PRD_CHIRALITY_APP.md` as an error.

### 2.2 Source Authority Rules

1. `PRD-AMEND`, `OPS-INT`, and `ECOSYS-PLAN` are authoritative for this planning topic until the product owner declares otherwise.
2. This plan must make extensive reference to those files and should not replace them as the source of record.
3. Where this plan summarizes a concept, the source files remain controlling.
4. Where the three source files differ in level of detail, the more specific source governs that topic:
   - product requirement semantics come primarily from `PRD-AMEND`;
   - OpenPipeStress integration mechanics come primarily from `OPS-INT`;
   - program sequencing and ecosystem framing come primarily from `ECOSYS-PLAN`.
5. Existing governed docs such as `docs/PRD.md`, `docs/SPEC.md`, `docs/TYPES.md`, and `docs/CONTRACT.md` should not be considered updated until a separate governed edit is made.
6. If this plan conflicts with the source documents, the source documents govern unless the product owner explicitly changes the intended direction.
7. If the source documents conflict with existing governed Chirality docs, the conflict should be treated as a planned amendment issue, not silently resolved.

### 2.3 Source Traceability Map

| Planning subject | Controlling source references |
| --- | --- |
| Domain Engine Integration product decision | `PRD-AMEND` Sections 1-3; `ECOSYS-PLAN` Sections 1-2; `OPS-INT` Section 1 |
| Core definitions | `PRD-AMEND` Section 4 |
| Product goals and non-goals | `PRD-AMEND` Sections 5-6; `ECOSYS-PLAN` Sections 3 and 8 |
| Product principles | `PRD-AMEND` Section 7; `ECOSYS-PLAN` Sections 2, 4, and 9 |
| Scope additions and exclusions | `PRD-AMEND` Section 8; `ECOSYS-PLAN` Section 8 |
| Functional requirements | `PRD-AMEND` Section 9 |
| Candidate APIs | `PRD-AMEND` Section 10 |
| Domain Engine Profile shape | `PRD-AMEND` Section 11; `OPS-INT` Sections 4-6; `ECOSYS-PLAN` Phase 1 and Workstream B |
| OpenPipeStress as first profile | `PRD-AMEND` Sections 12-13; `OPS-INT` Sections 2-6; `ECOSYS-PLAN` Sections 3, 5, 7, and 12 |
| Agent behavior rules | `PRD-AMEND` Section 14; `OPS-INT` Sections 8-9; `ECOSYS-PLAN` Sections 3, 9, and 11 |
| Validation and testing | `PRD-AMEND` Section 15; `OPS-INT` Section 12; `ECOSYS-PLAN` Section 11 |
| Risks and mitigations | `PRD-AMEND` Section 16; `ECOSYS-PLAN` Section 13 |
| Release and phase strategy | `PRD-AMEND` Section 17; `OPS-INT` Section 13; `ECOSYS-PLAN` Sections 5 and 7 |
| PRD text insertion candidates | `PRD-AMEND` Section 18 |
| Adoption acceptance criteria | `PRD-AMEND` Section 19; `ECOSYS-PLAN` Section 14 |
| OpenPipeStress adapter contract | `OPS-INT` Section 6 |
| Operation proposal schema | `OPS-INT` Section 7; `PRD-AMEND` Sections 4.8 and 14 |
| Chirality and OpenPipeStress backlog | `OPS-INT` Sections 10-11; `ECOSYS-PLAN` Section 10 |
| Workstreams | `ECOSYS-PLAN` Section 6 |
| MVP definition | `ECOSYS-PLAN` Section 8 |
| Technical architecture decisions | `ECOSYS-PLAN` Section 9 |
| Demonstration scenario | `ECOSYS-PLAN` Section 12 |
| Long-term product vision | `ECOSYS-PLAN` Sections 17-19 |

---

## 3. Executive Position

Source basis: `PRD-AMEND` Sections 1-3 and 7; `OPS-INT` Sections 1-3; `ECOSYS-PLAN` Sections 1-4 and 9.

The direction stated in the source documents is strong and should be pursued in a staged way.

The most important architectural decision is that Chirality should not become OpenPipeStress, and OpenPipeStress should not become Chirality. Chirality should remain the governed professional-work harness and agent operating layer. OpenPipeStress should remain the deterministic piping model, analysis, comparison, GUI, and handoff engine. The integration layer should be a general Domain Engine Framework that can support OpenPipeStress first and later support other specialist tools.

The right near-term posture is:

> Chirality governs work around deterministic domain engines. Domain engines own domain truth. Agents propose and organize. Deterministic tools compute and validate. Humans accept. Git-tracked files preserve the record.

This preserves Chirality's existing product principles:

- filesystem-native project state;
- git as event record;
- human authority at gates;
- no hidden memory as project truth;
- evidence over plausibility;
- explicit write scope;
- deterministic tooling wherever possible.

It also extends those principles into a more powerful product category: a local-first professional engineering workbench where agents can coordinate around specialist deterministic engines without silently taking control of engineering truth.

---

## 4. Recorded Intentions

Source basis: `PRD-AMEND` Sections 1-8 and 12-14; `OPS-INT` Sections 1-9; `ECOSYS-PLAN` Sections 1-4.

The three source documents indicate the following intentions.

### 4.1 Product Intention

Trace: `PRD-AMEND` Sections 1, 3, 5, and 7; `ECOSYS-PLAN` Sections 1-2.

Chirality should evolve from a deliverable-governance harness into a governed orchestration layer for professional work that may include deterministic domain engines.

This does not mean Chirality becomes a solver, model editor, CAD tool, code-compliance checker, professional validator, or issuer of relied-upon engineering conclusions. It means Chirality can discover, read, summarize, invoke, and coordinate around approved deterministic tools while preserving human authority and tool ownership.

### 4.2 First Concrete Domain

Trace: `PRD-AMEND` Sections 2, 3, and 12; `OPS-INT` Sections 1-3; `ECOSYS-PLAN` Sections 1, 3.3, and 12.

OpenPipeStress should be the first concrete Domain Engine Profile.

OpenPipeStress is a good proving ground because piping stress analysis has:

- a meaningful physical model;
- deterministic solver expectations;
- explicit state/run/comparison workflows;
- professional reliance boundaries;
- external validation/prover-tool realities;
- strong need for traceable handoff packages;
- clear distinction between model truth, analysis output, professional review, and project documentation.

### 4.3 Integration Intention

Trace: `OPS-INT` Sections 1-3 and 13; `ECOSYS-PLAN` Sections 4, 7, and 9.1-9.2.

The integration should be filesystem-native first, CLI/tool-adapter based second, and UI-deep-link based later.

Chirality should initially consume generated manifests, summaries, warnings, assumptions, run records, comparison records, proposals, and handoff records. It should not embed the OpenPipeStress GUI or directly manipulate canonical OpenPipeStress model files.

### 4.4 Governance Intention

Trace: `PRD-AMEND` Sections 6, 7, and 14; `OPS-INT` Section 9; `ECOSYS-PLAN` Sections 3.5, 9, and 11.4.

The integration must strengthen, not weaken, Chirality's authority model.

Domain integration must not create a back door around:

- human approval;
- agent write-scope constraints;
- dependency provenance;
- TBD handling;
- conflict surfacing;
- protected information boundaries;
- professional responsibility boundaries;
- outbound network restrictions;
- local filesystem auditability.

---

## 5. Planning Assumptions

Source basis: `PRD-AMEND` Sections 2, 6, 8, and 16; `OPS-INT` Sections 2, 9, and 13; `ECOSYS-PLAN` Sections 5, 8, 9, and 13.

This plan assumes:

1. The three bigger-picture documents are authoritative source documents for this planning topic until the product owner declares otherwise. Source: product owner instruction; reinforced by the source-authority rules in this plan.
2. `PRD_CHIRALITY_APP.md` in `PRD-AMEND` is intentional source terminology for this repo's PRD name in the owner's duplicate-filename working context. If the current checkout contains `docs/PRD.md`, that path should be reconciled with the intended PRD name during promotion rather than treated as a source error. Source: product owner clarification; `PRD-AMEND` header and Section 18.
3. The first implementation target should be read-only awareness and artifact scanning, not domain-tool mutation. Source: `PRD-AMEND` Section 17 Stage B; `OPS-INT` Level 1 and Phase 2; `ECOSYS-PLAN` Phase 4 and Program Release 1.
4. OpenPipeStress may live outside this repository, but the Chirality-side framework should be specified and testable locally using fixtures. Source: `OPS-INT` Sections 4, 12, and 13; `ECOSYS-PLAN` Workstreams D and F.
5. Any eventual OpenPipeStress CLI or adapter should be treated as an external deterministic tool with declared commands, schemas, outputs, write zones, and version metadata. Source: `OPS-INT` Section 6; `ECOSYS-PLAN` Sections 9.2 and 9.4.
6. The existing local-only and network policies remain in force unless explicitly changed by a future governed scope decision. Source: `PRD-AMEND` Sections 6 and 8; `ECOSYS-PLAN` Sections 8 and 13.
7. Professional engineering outputs remain drafts, review aids, or internal records until accepted by an accountable human. Source: `PRD-AMEND` Sections 7.2 and 14; `OPS-INT` Section 9.4; `ECOSYS-PLAN` Sections 3.5 and 9.

---

## 6. Target Product Model

Source basis: `PRD-AMEND` Sections 3-8 and 12-14; `OPS-INT` Sections 3, 8, and 9; `ECOSYS-PLAN` Sections 3-4.

### 6.1 Chirality Role

Trace: `PRD-AMEND` Sections 3, 5, 7, 8, 12, and 14; `OPS-INT` Sections 3, 8, and 9; `ECOSYS-PLAN` Sections 3.1, 4, and 9.

Chirality should own:

- project working-root selection;
- package and deliverable scaffolding;
- agent persona and task orchestration;
- document kits, working notes, TBDs, review notes, dependency registers, reconciliation records, checklists, and draft reports;
- domain-engine profile discovery and validation;
- domain artifact scanning;
- protected-path recognition;
- deterministic adapter invocation policy;
- result capture and provenance records;
- operation proposal authoring support;
- human review and approval workflow support;
- git-visible project memory.

Chirality should not own:

- OpenPipeStress canonical model truth;
- stress solver behavior;
- piping-specific model semantics;
- professional validation;
- code compliance conclusions;
- direct writes into accepted model states or solver outputs;
- external prover parsing unless separately scoped;
- professional acceptance, signing, sealing, issuing, or releasing.

### 6.2 Domain Engine Framework Role

Trace: `PRD-AMEND` Sections 4, 9, 10, and 11; `OPS-INT` Sections 4, 6, 10, and 13; `ECOSYS-PLAN` Sections 3.2, 6 Workstream B, and 9.

The Domain Engine Framework should own the generic integration concepts:

- Domain Engine Profile schema;
- profile registry and discovery;
- authoritative domain artifact declarations;
- Chirality-readable artifact declarations;
- protected write path declarations;
- agent-writable path declarations;
- deterministic tool declarations;
- invocation input/output schemas;
- domain result capture;
- operation proposal schema;
- human gate declarations;
- professional and IP boundary metadata.

The framework should remain domain-neutral. OpenPipeStress should be an example profile, not hard-coded into Chirality core.

### 6.3 OpenPipeStress Role

Trace: `PRD-AMEND` Sections 12 and 13; `OPS-INT` Sections 3, 4.2, 5, 6, 9, and 11; `ECOSYS-PLAN` Sections 3.3, 5 Phases 2-8, and Workstreams C-D.

OpenPipeStress should own:

- piping physical model;
- canonical project file;
- accepted model states;
- analysis runs;
- solver outputs;
- comparison records;
- engineering diagnostics;
- report fragments generated from domain state;
- handoff package generation;
- proposal validation;
- future proposal application through explicit human approval;
- GUI/domain editing experience.

OpenPipeStress should not own:

- Chirality's agent operating system;
- package/deliverable governance;
- cross-deliverable dependency management;
- general project reconciliation;
- professional acceptance records outside its domain handoff context.

### 6.4 Human Professional Role

Trace: `PRD-AMEND` Sections 7.2, 12, and 14; `OPS-INT` Sections 8-9; `ECOSYS-PLAN` Sections 3.5, 4.3, 8, and 9.

The accountable human should own:

- acceptance or rejection of model changes;
- external tool interpretation;
- professional reliance decisions;
- code compliance decisions;
- final report issuance;
- release of protected or proprietary data;
- decisions to promote planning documents into governed specs.

---

## 7. Non-Negotiable Boundaries

Source basis: `PRD-AMEND` Sections 6, 7, 8, 12, and 14; `OPS-INT` Sections 2, 6, 8, and 9; `ECOSYS-PLAN` Sections 3, 4, 8, 9, and 13.

These boundaries should be treated as design constraints.

### 7.1 Domain Truth Boundary

Trace: `PRD-AMEND` Sections 7.1, 7.4, 12.3, and 12.4; `OPS-INT` Sections 9.1-9.3; `ECOSYS-PLAN` Sections 3.1-3.3 and 9.3.

Domain engines own authoritative domain truth.

For OpenPipeStress, Chirality must not directly write:

- `OpenPipeStress/project.ops.yaml`;
- accepted model states;
- solver result files;
- comparison result files;
- handoff internals;
- acceptance records;
- private rule packs;
- private material libraries.

Chirality may write:

- proposals;
- review notes;
- TBD registers;
- draft report text;
- checklist items;
- dependency notes;
- change-management records;
- reconciliation records;
- agent commentary that cites generated summaries.

### 7.2 Tool Invocation Boundary

Trace: `PRD-AMEND` Sections 4.3, 9, 10, 15, and 17 Stage C; `OPS-INT` Section 6; `ECOSYS-PLAN` Sections 9.2 and Workstream D.

Domain tools must be invoked only through declared adapters.

Each tool command should declare:

- stable name;
- version;
- input schema;
- output schema;
- whether it is read-only or domain-controlled write;
- allowed working directories;
- allowed output paths;
- expected artifacts;
- timeout behavior;
- failure modes;
- whether a human gate is required before invocation;
- whether a human gate is required after invocation.

### 7.3 Proposal Boundary

Trace: `PRD-AMEND` Sections 4.8, 7.2, 12.4, 14, and 17 Stage D; `OPS-INT` Sections 7 and 9; `ECOSYS-PLAN` Sections 5 Phase 6 and 9.5.

Agents may propose domain changes, but proposals are not accepted changes.

Every proposal should state:

- proposal ID;
- source agent or human author;
- base domain state;
- intended operation;
- target entity or location;
- rationale;
- evidence references;
- assumptions;
- constraints considered;
- unresolved TBDs;
- boundary notice;
- validation status;
- human disposition.

No proposal should alter accepted domain truth until the domain engine validates it and the human accepts it.

### 7.4 Professional Boundary

Trace: `PRD-AMEND` Sections 6, 7.2, 12.1, and 14; `OPS-INT` Sections 2 Level 4, 8.4, and 9.4; `ECOSYS-PLAN` Sections 3.4, 3.5, and 8.

Neither Chirality nor OpenPipeStress should represent internal analysis as professional validation.

External professional tools may be used as validation/prover tools where industry practice requires them, but near-term integration should be manual and explicit. Chirality may summarize user-provided external results only as recorded artifacts. It must not claim that external validation occurred unless the evidence exists.

### 7.5 IP and Data Boundary

Trace: `PRD-AMEND` Sections 6 and 16; `OPS-INT` Section 11 EP-OPS-CHIR-006; `ECOSYS-PLAN` Workstream E and Risk Register.

Private rule packs, proprietary material libraries, protected client data, vendor models, and licensed external-tool exports require explicit handling.

The default posture should be:

- no automatic summarization of protected/private assets;
- no automatic inclusion in agent context;
- no network transmission except allowed provider paths;
- explicit release decisions by the human;
- boundary notices in handoff and report contexts.

---

## 8. Adoption Strategy

Source basis: `PRD-AMEND` Sections 17-19; `OPS-INT` Sections 13-15; `ECOSYS-PLAN` Sections 5, 7, 14, and 18.

The three authoritative source documents should not be merged wholesale into governed docs without an explicit adoption step. Their content should be promoted through staged, traceable amendments.

### 8.1 Stage A - Preserve Planning Record

Trace: `PRD-AMEND` Section 17 Stage A and Section 19; `ECOSYS-PLAN` Phase 0 and Section 18.

Status: this document.

Goal:

- Capture the strategic intent.
- Identify boundaries, risks, and sequencing.
- Keep the authoritative source documents distinct from already-governed requirements until formal amendments are made.

Exit criteria:

- Planning record exists in `plans/`.
- Source documents are listed.
- Near-term MVP and non-goals are explicit.
- Open questions are recorded.

### 8.2 Stage B - Governed Adoption Decision

Trace: `PRD-AMEND` Sections 3, 17, 18, and 19; `ECOSYS-PLAN` Phase 0 and Gate 1.

Goal:

- Decide whether Domain Engine Integration is accepted as a Chirality product direction.

Expected outputs:

- Human decision note, likely in a future change/adoption record.
- Determination of whether the PRD amendment text from `PRD-AMEND` Section 18 should be incorporated into the repo PRD file under the intended `PRD_CHIRALITY_APP.md` naming or reconciled with the existing local `docs/PRD.md` path.
- Decision on whether the new capability is current release scope, future release scope, or thesis-only for now.

Exit criteria:

- The product owner explicitly accepts, rejects, or defers Domain Engine Integration.
- Any accepted scope is tagged as MVP, later release, or research.

### 8.3 Stage C - Governed Specification Drafts

Trace: `PRD-AMEND` Sections 9-11 and 19; `OPS-INT` Sections 4.1 and 13 Phase 0; `ECOSYS-PLAN` Phase 1 and Documentation Plan.

Goal:

- Convert accepted concepts into stable local specs.

Candidate docs:

- `docs/DOMAIN_ENGINE_PROFILE_SPEC.md`
- `docs/DOMAIN_TOOL_ADAPTER_SPEC.md`
- `docs/DOMAIN_WRITE_BOUNDARY_POLICY.md`
- `docs/DOMAIN_OPERATION_PROPOSAL_SPEC.md`
- `docs/DOMAIN_ENGINE_INTEGRATION_PLAN.md`

Potential edits to existing docs:

- `docs/PRD.md` for product requirements;
- `docs/TYPES.md` for vocabulary;
- `docs/SPEC.md` for physical layout and file formats;
- `docs/CONTRACT.md` for invariants;
- `docs/PLAN.md` for roadmap positioning;
- `AGENTS.md` only if new agent roles or routing semantics are introduced;
- selected `agents/AGENT_*.md` files only after write-boundary rules are precise.

Exit criteria:

- The framework can be described without OpenPipeStress-specific logic in Chirality core.
- Profile, adapter, and proposal schemas are reviewable.
- Protected and agent-writable zones are specified in a testable way.

### 8.4 Stage D - Read-Only Implementation

Trace: `PRD-AMEND` Section 17 Stage B; `OPS-INT` Level 1 and Phase 2; `ECOSYS-PLAN` Phase 4 and Program Release 1.

Goal:

- Make Chirality aware of domain engines without allowing mutation of domain truth.

Expected outputs:

- profile validation;
- domain project discovery;
- artifact scanner;
- protected path matcher;
- domain registry APIs;
- fixture project;
- tests proving protected files are not written by Chirality-side operations.

Exit criteria:

- Chirality can detect an OpenPipeStress-like fixture project.
- Chirality can list readable artifacts.
- Chirality can identify protected paths.
- Agents can cite generated summaries without fabricating missing domain facts.

### 8.5 Stage E - Deterministic Tool Invocation

Trace: `PRD-AMEND` Section 17 Stage C; `OPS-INT` Level 1, Section 6, and Phase 3; `ECOSYS-PLAN` Phase 5 and Program Release 2.

Goal:

- Allow Chirality to invoke read-only deterministic domain tools through declared adapters.

Expected outputs:

- tool declaration schema;
- invocation endpoint;
- invocation logging;
- bounded stdout/stderr capture;
- artifact hash capture;
- timeout and failure behavior;
- no mutation outside declared output paths.

Exit criteria:

- A read-only adapter command can be invoked.
- Inputs and outputs validate.
- Results are captured as git-visible records.
- Failures become explicit tool results, not hidden runtime errors.

### 8.6 Stage F - Domain-Controlled Outputs

Trace: `OPS-INT` Level 2 and Phase 3; `ECOSYS-PLAN` Phase 5 and Program Releases 2-3; `PRD-AMEND` Section 17 Stage C.

Goal:

- Allow Chirality to request outputs that are created by the domain engine, not by agents.

Candidate outputs:

- analysis runs;
- state comparisons;
- run comparisons;
- handoff skeletons;
- report fragments generated from domain state.

Exit criteria:

- The domain engine creates the authoritative output.
- Chirality records the invocation and summarizes the generated artifact.
- Protected model truth remains controlled by the domain engine.

### 8.7 Stage G - Operation Proposal Workflow

Trace: `PRD-AMEND` Section 17 Stage D; `OPS-INT` Level 3, Sections 7 and 13 Phase 4; `ECOSYS-PLAN` Phase 6 and Program Releases 4-5.

Goal:

- Let agents produce structured proposals that domain engines validate and humans disposition.

Expected outputs:

- proposal schema;
- proposal authoring path;
- proposal validation command;
- validation summary;
- review note generation;
- human disposition field;
- future apply workflow gated by human approval.

Exit criteria:

- Agent-created proposals are clearly non-authoritative.
- Domain engine validation is deterministic.
- Accepted changes, if implemented later, create new domain states through the domain engine only.

### 8.8 Stage H - Platformization

Trace: `ECOSYS-PLAN` Phase 9 and Program Releases 6-7; `PRD-AMEND` Sections 3 and 16; `OPS-INT` Section 13 Phase 6 for future extensibility.

Goal:

- Generalize beyond OpenPipeStress after the pattern proves itself.

Candidate future domain engines:

- structural analysis;
- process simulation;
- cost/schedule tools;
- CAD/BIM extractors;
- safety/risk analysis;
- regulatory compliance document engines.

Exit criteria:

- At least one non-OpenPipeStress profile can be expressed without changing core framework semantics.
- Common abstractions remain stable.

---

## 9. Minimum Viable Scope

Source basis: `PRD-AMEND` Sections 8 and 17; `OPS-INT` Sections 2, 13, and 14; `ECOSYS-PLAN` Sections 7, 8, and 18.

The MVP should be deliberately narrow.

### 9.1 MVP Includes

Chirality-side (source: `PRD-AMEND` Sections 8.1, 9, 15, and 17 Stage B; `OPS-INT` Sections 10 and 13 Phase 2; `ECOSYS-PLAN` Section 8):

- Domain Engine Profile schema draft.
- OpenPipeStress example profile.
- profile validation.
- project discovery.
- artifact scanning.
- readable/protected/agent-writable path classification.
- read-only deterministic tool declaration support.
- read-only tool invocation support if a fixture adapter exists.
- agent access to bounded summaries/manifests.
- boundary notices in generated review notes.
- tests for protected path classification.

OpenPipeStress-side, if available (source: `OPS-INT` Sections 4.2, 6, 11, and 13 Phase 1; `ECOSYS-PLAN` Section 8 and Workstreams C-D):

- canonical project layout fixture.
- generated model manifest.
- generated warning/assumption summary.
- generated state list.
- generated run list.
- read-only CLI commands.

Shared (source: `OPS-INT` Section 12.3; `ECOSYS-PLAN` Workstreams E-F and Section 8):

- fixture project.
- professional boundary notice.
- no direct agent editing.
- no automatic external prover claims.
- no network expansion.

### 9.2 MVP Excludes

Trace: `PRD-AMEND` Section 8.2; `OPS-INT` Level 4 and Phase 6; `ECOSYS-PLAN` Section 8 exclusions.

The MVP should exclude:

- direct agent edits to OpenPipeStress model files;
- automatic application of proposals;
- embedded OpenPipeStress GUI;
- commercial external-tool parsers;
- automatic professional validation;
- code compliance conclusions;
- dynamic analysis;
- CAD/BIM import;
- route optimization;
- multi-domain platform marketplace behavior;
- cloud execution;
- new outbound network paths.

---

## 10. Proposed System Components

Source basis: `PRD-AMEND` Sections 4, 9, 10, 11, 14, and 15; `OPS-INT` Sections 4, 6, 7, 9, and 10; `ECOSYS-PLAN` Sections 6, 9, and 10.

### 10.1 Domain Profile Registry

Trace: `PRD-AMEND` Sections 8.1, 9 FR-DOM-001 through FR-DOM-006, 10, and 11; `OPS-INT` Section 10 EP-C-DOM-001 and EP-C-DOM-002.

Purpose:

- Locate available domain-engine profiles.
- Validate profile schema.
- Expose supported engines and tools to UI/API/agents.

Initial locations to consider:

- bundled profiles under the instruction root;
- project-local profiles under the working root;
- examples/fixtures for test profiles.

Open decision:

- Whether project-local profiles are trusted automatically, require human approval, or are treated as untrusted until validated.

### 10.2 Profile Validator

Trace: `PRD-AMEND` Sections 9 FR-DOM-002 and 15; `OPS-INT` Section 12.1; `ECOSYS-PLAN` Phase 1 acceptance criteria.

Purpose:

- Validate required fields.
- Validate path glob syntax.
- Validate tool declarations.
- Validate read/write zone separation.
- Reject profiles that allow protected paths to overlap agent-writable paths.
- Warn on overly broad patterns.

Expected behavior:

- fail closed on malformed profiles;
- produce human-readable diagnostics;
- produce machine-readable validation results.

### 10.3 Artifact Scanner

Trace: `PRD-AMEND` Sections 8.1, 9 FR-DOM-007 and FR-DOM-008, and 12.2; `OPS-INT` Section 10 EP-C-DOM-004; `ECOSYS-PLAN` Phase 4.

Purpose:

- Scan a working root or deliverable subtree for domain-engine artifacts.
- Classify artifacts as authoritative, readable, generated, proposal, review, protected, or unknown.
- Return bounded metadata to agents.

Expected metadata:

- relative path;
- artifact class;
- owning engine;
- profile version;
- modified time;
- size;
- optional content hash;
- whether content may be read by agents;
- whether content may be summarized;
- whether content may be written by Chirality;
- boundary notice, if applicable.

### 10.4 Protected Path Guard

Trace: `PRD-AMEND` Sections 7.4, 9 FR-DOM-003 and FR-DOM-004, 12.3, and 15; `OPS-INT` Section 10 EP-C-DOM-003; `ECOSYS-PLAN` Section 9.3.

Purpose:

- Prevent Chirality-side APIs, tools, or agents from writing to protected domain paths.

This must become runtime enforcement, not just instruction guidance.

Expected behavior:

- classify candidate writes before execution;
- reject writes into protected paths;
- log blocked write attempts;
- provide clear errors;
- support test fixtures;
- ensure agent-writable proposal paths do not overlap protected paths.

### 10.5 Tool Adapter Contract

Trace: `PRD-AMEND` Sections 4.3, 8.1, 9 FR-DOM-009 through FR-DOM-012, and 10; `OPS-INT` Section 6; `ECOSYS-PLAN` Sections 9.2 and Workstream D.

Purpose:

- Define how Chirality invokes deterministic domain tools.

Each adapter command should declare:

- command name;
- command version;
- engine name;
- engine version requirement;
- invocation type: read-only, domain-controlled write, proposal validation, proposal application;
- input schema;
- output schema;
- allowed input paths;
- allowed output paths;
- timeout;
- environmental requirements;
- whether network is forbidden;
- whether private assets may be read;
- whether human approval is required.

### 10.6 Invocation Result Capture

Trace: `PRD-AMEND` Section 9 FR-DOM-010 through FR-DOM-012 and Section 15; `OPS-INT` Sections 6 and 12; `ECOSYS-PLAN` Sections 9.4 and 11.

Purpose:

- Preserve deterministic tool invocations as auditable project records.

Each invocation should capture:

- invocation ID;
- timestamp;
- user/session context;
- engine profile ID and version;
- adapter command and version;
- input JSON/YAML;
- resolved working directory;
- declared read paths;
- declared write paths;
- output paths;
- exit code;
- stdout/stderr excerpts;
- validation result;
- content hashes where appropriate;
- boundary notices.

### 10.7 Operation Proposal Support

Trace: `PRD-AMEND` Sections 4.8, 8.1, 9 FR-DOM-013 through FR-DOM-015, 14, and 17 Stage D; `OPS-INT` Section 7 and Section 10 EP-C-DOM-005.

Purpose:

- Give agents a structured way to propose domain changes without mutating domain truth.

Proposal lifecycle:

```
drafted -> schema_validated -> domain_validated -> human_reviewed -> accepted/rejected/deferred -> applied_by_domain_engine
```

The final application step should not be part of the MVP unless explicitly scoped.

### 10.8 Boundary Notice System

Trace: `PRD-AMEND` Sections 8.1, 9 FR-DOM-016, 12.1, 14, and 18; `OPS-INT` Sections 8-9; `ECOSYS-PLAN` Workstream E.

Purpose:

- Prevent users from mistaking agent summaries or internal domain-tool results for professional acceptance.

Boundary notices should appear in:

- profile metadata;
- tool results;
- proposal validation outputs;
- generated review notes;
- handoff checklists;
- report drafts;
- UI surfaces that summarize domain artifacts.

---

## 11. Candidate APIs

Source basis: `PRD-AMEND` Section 10, with implementation detail informed by `OPS-INT` Sections 4, 6, and 7.

The PRD amendment proposes API shapes that should be refined during specification. The first six endpoints below are drawn directly from `PRD-AMEND` Section 10; the additional endpoints are implementation-support candidates derived from the registry, scan, invocation, and proposal flows in `OPS-INT` Sections 4, 6, and 7.

Initial candidate endpoints:

- `GET /api/domain/profiles/list`
- `POST /api/domain/profile/validate`
- `POST /api/domain/artifacts/scan`
- `POST /api/domain/tool/invoke`
- `GET /api/domain/proposals/list`
- `POST /api/domain/proposal/validate`

Additional endpoints to consider:

- `GET /api/domain/profile/:id`
- `POST /api/domain/path/classify`
- `POST /api/domain/write/check`
- `GET /api/domain/invocations/list`
- `GET /api/domain/invocations/:id`

API concerns:

- project root must be explicit or bound to an existing harness session;
- profile trust status must be included;
- path inputs must be normalized and constrained to the working root;
- protected path classification must happen before any write;
- tool invocation must not bypass existing network policy;
- tool execution must be bounded and logged.

---

## 12. Candidate Data Shapes

Source basis: `PRD-AMEND` Sections 4 and 11; `OPS-INT` Sections 6 and 7; `ECOSYS-PLAN` Sections 9 and 10.

### 12.1 Domain Engine Profile

Trace: profile fields and path categories are drawn primarily from `PRD-AMEND` Section 11, with artifact categories and adapter needs cross-checked against `OPS-INT` Sections 4-6 and `ECOSYS-PLAN` Phase 1.

The source documents suggest a YAML profile shaped roughly as follows:

```yaml
schema_version: "0.1"
engine:
  id: "openpipestress"
  name: "OpenPipeStress"
  version_range: ">=0.2.0"

discovery:
  root_markers:
    - "OpenPipeStress/project.ops.yaml"

authoritative_artifacts:
  - "OpenPipeStress/project.ops.yaml"
  - "OpenPipeStress/states/**"
  - "OpenPipeStress/runs/**"
  - "OpenPipeStress/comparisons/**"
  - "OpenPipeStress/handoff/**"

chirality_readable_artifacts:
  - "OpenPipeStress/manifests/**"
  - "OpenPipeStress/summaries/**"
  - "OpenPipeStress/warnings/**"
  - "OpenPipeStress/assumptions/**"
  - "OpenPipeStress/proposals/**"

protected_write_paths:
  - "OpenPipeStress/project.ops.yaml"
  - "OpenPipeStress/states/**"
  - "OpenPipeStress/runs/**"
  - "OpenPipeStress/comparisons/**"
  - "OpenPipeStress/handoff/**"

agent_writable_paths:
  - "OpenPipeStress/proposals/**"
  - "OpenPipeStress/review_notes/**"
  - "OpenPipeStress/tbd/**"

deterministic_tools:
  - name: "ops.validate_model"
    mode: "read_only"
  - name: "ops.summarize_model"
    mode: "read_only"

professional_boundary:
  notice: "Internal domain-engine output is not professional acceptance."
  human_acceptance_required: true
```

Specification work should turn this into a precise schema with required fields, path pattern semantics, and validation rules.

### 12.2 Tool Invocation Result

Trace: result capture fields are derived from `PRD-AMEND` Sections 9 and 15, `OPS-INT` Section 6 command contracts, and `ECOSYS-PLAN` Sections 9.2 and 9.4.

Candidate shape:

```yaml
schema_version: "0.1"
invocation_id: "DOM-INV-0001"
engine_id: "openpipestress"
profile_version: "0.1"
tool_name: "ops.summarize_model"
tool_version: "0.2.0"
mode: "read_only"
started_at: "TBD"
completed_at: "TBD"
working_root: "TBD"
inputs:
  model_path: "TBD"
outputs:
  result_path: "TBD"
  summary_path: "TBD"
exit_code: 0
status: "succeeded"
warnings: []
boundary_notices:
  - "This result is generated by a domain engine and is not professional acceptance."
hashes:
  input_hashes: []
  output_hashes: []
```

### 12.3 Operation Proposal

Trace: proposal fields are drawn primarily from `OPS-INT` Section 7, with boundary and agent-behavior constraints from `PRD-AMEND` Sections 4.8 and 14 and lifecycle framing from `ECOSYS-PLAN` Phase 6.

Candidate shape:

```yaml
schema_version: "0.1"
proposal_id: "PROP-0001"
title: "Add guide support near node TBD"
created_by: "AGENT_TASK"
created_at: "TBD"
engine_id: "openpipestress"
base_model_state: "STATE-0001"
operation:
  type: "add_support"
  target: "TBD"
  proposed_parameters: {}
rationale:
  summary: "TBD"
  evidence:
    - path: "TBD"
      source_ref: "TBD"
assumptions:
  - "TBD"
unresolved_tbd:
  - "TBD"
boundary:
  status: "proposal_only"
  notice: "This proposal is not an accepted engineering change."
validation:
  schema_status: "TBD"
  domain_status: "TBD"
human_disposition:
  status: "not_reviewed"
  reviewer: "TBD"
  reviewed_at: "TBD"
```

---

## 13. Required Governance Updates

Source basis: `PRD-AMEND` Sections 5, 6, 7, 8, 18, and 19; `OPS-INT` Sections 9 and 10; `ECOSYS-PLAN` Sections 5, 6, and 15.

If adopted, the framework should update the governance layer deliberately.

### 13.1 PRD Updates

Trace: `PRD-AMEND` Sections 5, 6, 8, 9, 16, 18, and 19.

The repo PRD, identified by the source material as `PRD_CHIRALITY_APP.md` and currently represented in this checkout by `docs/PRD.md` unless renamed, should gain:

- Domain Engine Integration product capability;
- domain-engine user journeys;
- in-scope and out-of-scope boundaries;
- functional requirements for profile validation, artifact scanning, tool invocation, protected write paths, operation proposals, and boundary notices;
- non-goals around solver ownership, professional validation, direct model writes, external prover automation, and network expansion;
- known gaps and risks.

### 13.2 TYPES Updates

Trace: `PRD-AMEND` Section 4; `OPS-INT` Sections 6-7; `ECOSYS-PLAN` Sections 3 and 9.

`docs/TYPES.md` should define:

- Domain Engine;
- Domain Engine Profile;
- Domain Tool Adapter;
- Authoritative Domain Artifact;
- Chirality-Readable Artifact;
- Protected Write Path;
- Agent-Writable Domain Artifact;
- Operation Proposal;
- Domain-Controlled Write;
- External Prover Artifact;
- Professional Acceptance Record.

### 13.3 SPEC Updates

Trace: `PRD-AMEND` Sections 11 and 13; `OPS-INT` Sections 4-7; `ECOSYS-PLAN` Documentation Plan.

`docs/SPEC.md` should define:

- optional domain-engine directory conventions inside deliverables;
- profile file location conventions;
- proposal file conventions;
- invocation result record conventions;
- path pattern matching rules;
- protected path behavior;
- generated summary/manifest expectations.

### 13.4 CONTRACT Updates

Trace: `PRD-AMEND` Sections 7, 12, 14, and 15; `OPS-INT` Section 9; `ECOSYS-PLAN` Sections 9 and 11.4.

`docs/CONTRACT.md` should add invariants only after enforcement is clear.

Candidate invariants:

- domain engines own authoritative domain truth;
- agents must not write protected domain paths;
- domain tool invocations must be declared and logged;
- operation proposals are non-authoritative until human accepted and domain-applied;
- professional validation claims require explicit evidence;
- protected/private domain assets must not be read or summarized unless released.

### 13.5 Agent Instruction Updates

Trace: `PRD-AMEND` Section 14; `OPS-INT` Sections 8-9; `ECOSYS-PLAN` Workstream A and Governance Tests.

Agent instruction updates should wait until the write zones and proposal semantics are precise.

Likely affected agents:

- ORCHESTRATOR;
- WORKING_ITEMS;
- RECONCILIATION;
- CHANGE;
- TASK;
- DEPENDENCIES;
- AUDIT_GOVERNANCE;
- AUDIT_SCOPE_CLOSURE;
- EVALUATION_REPORT.

Updates should be minimal and should avoid piping-specific instructions unless the task is explicitly OpenPipeStress-scoped.

---

## 14. Implementation Roadmap

Source basis: `PRD-AMEND` Section 17; `OPS-INT` Section 13; `ECOSYS-PLAN` Sections 5, 7, and 18.

### Phase 0 - Adoption and Alignment

Trace: `PRD-AMEND` Sections 17 Stage A, 18, and 19; `OPS-INT` Section 13 Phase 0; `ECOSYS-PLAN` Phase 0 and Gate 1.

Objective:

- Decide whether Domain Engine Integration is an accepted product direction.

Tasks:

- Record this plan.
- Review source documents for contradictions.
- Preserve the intentional `PRD_CHIRALITY_APP.md` source name from `PRD-AMEND` while reconciling it with the current checkout's PRD path.
- Decide whether the PRD amendment should be incorporated into the current repo PRD file or promoted as a separately named governed PRD artifact.
- Decide whether OpenPipeStress is an example, a committed first integration, or a future demonstration.

Exit criteria:

- Human adoption decision exists.
- Scope is labeled as current, next, future, or thesis-only.

### Phase 1 - Framework Specifications

Trace: `PRD-AMEND` Sections 9-11 and 19; `OPS-INT` Section 13 Phase 0; `ECOSYS-PLAN` Phase 1 and Workstream B.

Objective:

- Define the generic Domain Engine Framework before implementing OpenPipeStress-specific behavior.

Tasks:

- Draft `DOMAIN_ENGINE_PROFILE_SPEC.md`.
- Draft `DOMAIN_TOOL_ADAPTER_SPEC.md`.
- Draft `DOMAIN_WRITE_BOUNDARY_POLICY.md`.
- Draft `DOMAIN_OPERATION_PROPOSAL_SPEC.md`.
- Draft OpenPipeStress example profile.
- Define fixture project layout.
- Define profile trust model.
- Define protected path matching semantics.

Exit criteria:

- Profile spec can represent OpenPipeStress.
- Profile spec does not require piping-specific fields in Chirality core.
- Protected and agent-writable path sets are disjoint.

### Phase 2 - Read-Only Domain Awareness

Trace: `PRD-AMEND` Section 17 Stage B; `OPS-INT` Section 13 Phase 2; `ECOSYS-PLAN` Phase 4 and Program Release 1.

Objective:

- Let Chirality discover and classify domain-engine artifacts without invoking external tools.

Tasks:

- Add profile validator.
- Add domain project discovery.
- Add artifact scanner.
- Add path classifier.
- Add protected path guard checks for Chirality-side writes.
- Add fixture tests.
- Add UI/API read-only listing if appropriate.

Exit criteria:

- Chirality can detect an OpenPipeStress fixture.
- Chirality can classify readable and protected paths.
- Chirality rejects writes to protected paths.
- Agents can be given bounded readable metadata.

### Phase 3 - Read-Only Deterministic Tool Invocation

Trace: `PRD-AMEND` Sections 9 FR-DOM-009 through FR-DOM-012 and 17 Stage C; `OPS-INT` Sections 6.1-6.4 and 13 Phase 3; `ECOSYS-PLAN` Phase 5.

Objective:

- Allow Chirality to call declared read-only domain tools.

Tasks:

- Implement tool invocation endpoint.
- Validate tool input schemas.
- Capture invocation records.
- Capture output metadata and hashes.
- Bound stdout/stderr capture.
- Add timeouts.
- Fail closed on undeclared tools.
- Add a mock or fixture adapter if real OpenPipeStress is unavailable.

Candidate commands:

- `ops.validate_model`
- `ops.summarize_model`
- `ops.list_states`
- `ops.list_runs`
- `ops.list_comparisons`

Exit criteria:

- Read-only tool calls are reproducible and logged.
- Undeclared tool calls are rejected.
- Tool results can be cited by agents.
- No protected domain truth is modified.

### Phase 4 - Domain-Controlled Output Generation

Trace: `OPS-INT` Sections 6.5-6.9 and 13 Phase 3; `PRD-AMEND` Section 17 Stage C; `ECOSYS-PLAN` Phase 5 and Program Releases 2-3.

Objective:

- Allow Chirality to request domain-generated outputs while preserving domain ownership.

Tasks:

- Extend adapter declaration with domain-controlled write modes.
- Add human gate requirements for write-producing tool calls.
- Verify output paths against profile declarations.
- Capture run/comparison/handoff records.
- Add report fragment handling.

Candidate commands:

- `ops.run_analysis`
- `ops.compare_states`
- `ops.compare_runs`
- `ops.generate_handoff`
- `ops.generate_report_fragment`

Exit criteria:

- The domain engine writes the output.
- Chirality records and summarizes the output.
- Agents do not create or edit authoritative results directly.

### Phase 5 - Operation Proposal Workflow

Trace: `PRD-AMEND` Section 17 Stage D; `OPS-INT` Sections 6.10-6.11, 7, and 13 Phase 4; `ECOSYS-PLAN` Phase 6 and Program Releases 4-5.

Objective:

- Let agents draft structured proposals and let the domain engine validate them.

Tasks:

- Implement proposal schema validation.
- Add proposal list/read APIs.
- Add proposal authoring conventions.
- Add `ops.validate_operation_proposal`.
- Add proposal review note generation.
- Add human disposition fields.

Future-only command:

- `ops.apply_operation_proposal`

Exit criteria:

- A proposal can be drafted, validated, reviewed, accepted/rejected/deferred, and recorded.
- Proposal application is either out of scope or gated through explicit human approval and domain-engine state creation.

### Phase 6 - Handoff and External Prover Workflow

Trace: `OPS-INT` Sections 2 Level 4, 8.4, 8.5, and 13 Phase 6; `ECOSYS-PLAN` Phase 8; `PRD-AMEND` Sections 6 and 12.1.

Objective:

- Support manual professional validation workflows without pretending to automate them.

Tasks:

- Define handoff manifest format.
- Define external result-state artifact semantics.
- Define boundary notices for external results.
- Allow manual import/reference of external review comments.
- Support comparison summaries that cite evidence.

Exit criteria:

- Chirality can organize review work around external results.
- It cannot declare external validation unless the evidence artifact exists.
- Human interpretation and acceptance remain explicit.

### Phase 7 - Platformization Beyond OpenPipeStress

Trace: `ECOSYS-PLAN` Phase 9 and Program Releases 6-7; `PRD-AMEND` Sections 3 and 16; `OPS-INT` Section 13 Phase 6.

Objective:

- Generalize the framework after one successful domain-engine integration.

Tasks:

- Create a generic domain-engine profile template.
- Add additional fixture profiles.
- Identify common adapter conventions.
- Remove OpenPipeStress assumptions from core code.
- Document the profile authoring process.

Exit criteria:

- A second domain engine can be described without new core abstractions.
- The framework remains useful outside piping stress analysis.

---

## 15. Backlog Candidates

Source basis: `OPS-INT` Sections 10-11; `ECOSYS-PLAN` Section 10; `PRD-AMEND` Sections 9 and 15.

### 15.1 Chirality Framework Backlog

| ID | Title | Priority | Notes | Source |
| --- | --- | --- | --- | --- |
| CH-DOM-001 | Domain Engine Profile schema | P0 | Define generic profile format. | `PRD-AMEND` FR-DOM-001; `OPS-INT` EP-C-DOM-001; `ECOSYS-PLAN` CH-DOM-001 |
| CH-DOM-002 | Profile validator | P0 | Fail closed on malformed or unsafe profiles. | `PRD-AMEND` FR-DOM-002 and Section 15; `OPS-INT` Section 12.1 |
| CH-DOM-003 | OpenPipeStress example profile | P0 | First concrete profile, not hard-coded core behavior. | `PRD-AMEND` Sections 11-12; `OPS-INT` Phase 0; `ECOSYS-PLAN` Phase 1 |
| CH-DOM-004 | Domain artifact scanner | P0 | Classify artifacts by profile declarations. | `PRD-AMEND` FR-DOM-007 and FR-DOM-008; `OPS-INT` EP-C-DOM-004 |
| CH-DOM-005 | Protected path classifier | P0 | Runtime support for write checks. | `PRD-AMEND` FR-DOM-003 and FR-DOM-004; `ECOSYS-PLAN` Section 9.3 |
| CH-DOM-006 | Protected write guard | P0 | Enforce no direct writes to protected paths. | `PRD-AMEND` Sections 7.4 and 15; `OPS-INT` EP-C-DOM-003 |
| CH-DOM-007 | Domain registry APIs | P1 | List profiles, engines, tools, artifacts. | `PRD-AMEND` Section 10; `OPS-INT` EP-C-DOM-002 |
| CH-DOM-008 | Read-only tool invocation | P1 | Declared deterministic commands only. | `PRD-AMEND` FR-DOM-009 through FR-DOM-012; `OPS-INT` Section 6 |
| CH-DOM-009 | Invocation result records | P1 | Git-visible audit trail for tool calls. | `PRD-AMEND` FR-DOM-010 through FR-DOM-012; `ECOSYS-PLAN` Section 9.4 |
| CH-DOM-010 | Operation proposal schema | P1 | Agent proposal format. | `PRD-AMEND` FR-DOM-013 through FR-DOM-015; `OPS-INT` Section 7 |
| CH-DOM-011 | Proposal validation API | P2 | Local schema validation plus domain adapter validation. | `OPS-INT` Section 6.10; `PRD-AMEND` Section 17 Stage D |
| CH-DOM-012 | Boundary notice rendering | P1 | UI/API/report surfaces. | `PRD-AMEND` FR-DOM-016; `OPS-INT` EP-C-DOM-006; `ECOSYS-PLAN` Workstream E |
| CH-DOM-013 | Agent instruction updates | P2 | Add domain rules after specs stabilize. | `PRD-AMEND` Section 14; `OPS-INT` Sections 8-9 |
| CH-DOM-014 | Domain fixture tests | P0 | Prevent regressions in path classification. | `OPS-INT` Section 12.3; `ECOSYS-PLAN` Workstream F |
| CH-DOM-015 | UI domain artifact panel | P3 | Later convenience, not MVP-critical. | `PRD-AMEND` Section 17 Stage E; `OPS-INT` Phase 5 |

### 15.2 OpenPipeStress Adapter Backlog

| ID | Title | Priority | Notes | Source |
| --- | --- | --- | --- | --- |
| OPS-ADAPT-001 | Read-only CLI contract | P0 | Required before safe invocation. | `OPS-INT` EP-OPS-CHIR-001 and Phase 1; `ECOSYS-PLAN` Workstream D |
| OPS-ADAPT-002 | Model manifest generator | P0 | Bounded agent-readable summary. | `OPS-INT` EP-OPS-CHIR-003; `ECOSYS-PLAN` Section 9.4 |
| OPS-ADAPT-003 | State/run/comparison list commands | P0 | Read-only discovery. | `OPS-INT` Sections 6.3-6.4; `ECOSYS-PLAN` Phases 3-4 |
| OPS-ADAPT-004 | Validation summary command | P1 | Deterministic diagnostics. | `OPS-INT` Section 6.1; `PRD-AMEND` Section 15 |
| OPS-ADAPT-005 | Analysis run command | P2 | Domain-controlled write. | `OPS-INT` Section 6.5; `ECOSYS-PLAN` Phase 5 |
| OPS-ADAPT-006 | Comparison command | P2 | Domain-controlled write. | `OPS-INT` Sections 6.6-6.7; `ECOSYS-PLAN` Phase 5 |
| OPS-ADAPT-007 | Handoff generator | P2 | Domain-generated handoff package. | `OPS-INT` Section 6.8; `ECOSYS-PLAN` Program Release 3 |
| OPS-ADAPT-008 | Proposal validator | P2 | Validate proposed model operations. | `OPS-INT` Section 6.10; `ECOSYS-PLAN` Phase 6 |
| OPS-ADAPT-009 | Proposal apply workflow | P3 | Human-gated future capability. | `OPS-INT` Section 6.11; `ECOSYS-PLAN` Program Release 5 |

### 15.3 Shared Backlog

| ID | Title | Priority | Notes | Source |
| --- | --- | --- | --- | --- |
| SHARED-DOM-001 | Fixture project layout | P0 | Needed for testable adoption. | `OPS-INT` Section 12.3; `ECOSYS-PLAN` Workstream F |
| SHARED-DOM-002 | Professional boundary notice text | P0 | Reused in profile, UI, reports. | `PRD-AMEND` Sections 12.1 and 14; `OPS-INT` Section 9.4 |
| SHARED-DOM-003 | IP/data classification defaults | P1 | Avoid accidental exposure. | `OPS-INT` EP-OPS-CHIR-006; `ECOSYS-PLAN` Workstream E |
| SHARED-DOM-004 | Handoff manifest schema | P2 | Needed before external review workflow. | `OPS-INT` Sections 4.3 and 6.8; `ECOSYS-PLAN` Phase 8 |
| SHARED-DOM-005 | External result-state semantics | P3 | Future, manual first. | `OPS-INT` Level 4 and Phase 6; `ECOSYS-PLAN` Phase 8 |

---

## 16. Testing and Verification Strategy

Source basis: `PRD-AMEND` Section 15; `OPS-INT` Section 12; `ECOSYS-PLAN` Section 11.

### 16.1 Profile Tests

Trace: `PRD-AMEND` Section 15; `OPS-INT` Section 12.1; `ECOSYS-PLAN` Phase 1 acceptance criteria.

Tests should verify:

- valid profiles pass;
- missing required fields fail;
- malformed path patterns fail;
- protected and agent-writable path overlap fails;
- unknown tool modes fail;
- overly broad path patterns warn or fail depending on risk;
- profile version is captured.

### 16.2 Path Guard Tests

Trace: `PRD-AMEND` Sections 7.4 and 15; `OPS-INT` Section 12.1; `ECOSYS-PLAN` Section 9.3 and Governance Tests.

Tests should verify:

- writes to protected paths are blocked;
- writes to proposal paths are allowed when declared;
- path traversal is blocked;
- symlink behavior is defined and tested;
- absolute paths outside the working root are blocked;
- profile root markers do not allow escape from the working root.

### 16.3 Artifact Scanner Tests

Trace: `PRD-AMEND` Sections 12.2 and 15; `OPS-INT` Section 12.1; `ECOSYS-PLAN` Phase 4 acceptance criteria.

Tests should verify:

- artifact classes are correctly assigned;
- unreadable/protected/private artifacts are not read into agent context;
- missing expected artifacts produce diagnostics, not fabricated summaries;
- hashes and metadata are stable enough for audit use.

### 16.4 Tool Invocation Tests

Trace: `PRD-AMEND` Section 15; `OPS-INT` Sections 6 and 12.1; `ECOSYS-PLAN` Workstream D.

Tests should verify:

- undeclared tools cannot run;
- declared tools receive validated inputs;
- output paths are constrained;
- timeouts work;
- non-zero exits are captured;
- stdout/stderr capture is bounded;
- invocation records are written;
- read-only tool invocations do not modify fixture domain truth.

### 16.5 Proposal Tests

Trace: `PRD-AMEND` Sections 4.8, 14, and 15; `OPS-INT` Sections 7 and 12.3; `ECOSYS-PLAN` Phase 6.

Tests should verify:

- valid proposal schema passes;
- proposal with no base state fails;
- proposal with no evidence is marked incomplete;
- proposal cannot be mistaken for accepted change;
- domain validation status is recorded separately from human disposition.

### 16.6 Governance Tests

Trace: `PRD-AMEND` Sections 6, 7, 14, and 15; `OPS-INT` Section 9; `ECOSYS-PLAN` Section 11.4.

Tests or review checklists should verify:

- no generated text claims professional approval;
- no external prover result is fabricated;
- no agent claims code compliance without evidence and human acceptance;
- boundary notices appear in summaries and handoff contexts;
- protected/private assets remain excluded unless explicitly released.

---

## 17. Risk Register

Source basis: `PRD-AMEND` Section 16; `ECOSYS-PLAN` Section 13; additional tool-invocation risks derived from `OPS-INT` Sections 6 and 9.

| Risk | Severity | Mitigation | Source |
| --- | --- | --- | --- |
| Chirality becomes too domain-specific | High | Keep framework generic; use OpenPipeStress only as first profile. | `PRD-AMEND` Section 16; `ECOSYS-PLAN` Section 13 |
| Agents mutate domain truth | High | Runtime protected path guard; proposal-only agent writes. | `PRD-AMEND` Sections 7.4 and 16; `OPS-INT` Section 9 |
| Users mistake internal solver output for professional validation | High | Boundary notices, external prover semantics, human acceptance gates. | `PRD-AMEND` Sections 6, 14, and 16; `ECOSYS-PLAN` Section 13 |
| Tool invocation becomes an unsafe execution back door | High | Declared adapters only, constrained paths, no network expansion, logging, timeouts. | `OPS-INT` Sections 6 and 9; `PRD-AMEND` Section 15 |
| Scope expands before MVP is useful | High | Limit MVP to read-only awareness and bounded summaries. | `ECOSYS-PLAN` Section 8 and Risk Register |
| OpenPipeStress integration delays Chirality core hardening | Medium | Treat domain framework as staged future scope unless explicitly adopted. | `ECOSYS-PLAN` Workstream A and Section 13 |
| OpenPipeStress becomes a general agent OS | Medium | Keep OpenPipeStress focused on model, solver, states, GUI, handoff. | `PRD-AMEND` Section 2; `OPS-INT` Section 1; `ECOSYS-PLAN` Section 13 |
| Profile trust model is unclear | Medium | Require explicit trust/approval semantics before project-local profiles are executable. | Derived from `PRD-AMEND` Sections 9-11 and `OPS-INT` Section 10 |
| Protected data leaks into agent context | High | Default no-read/no-summarize for private assets; explicit release gates. | `OPS-INT` EP-OPS-CHIR-006; `ECOSYS-PLAN` Workstream E |
| External prover integration is overbuilt too early | Medium | Keep external validation manual in early releases. | `OPS-INT` Level 4 and Phase 6; `ECOSYS-PLAN` Phase 8 |
| Profile path glob behavior is ambiguous | Medium | Specify matching semantics and test edge cases. | `PRD-AMEND` Sections 11 and 15; `OPS-INT` Section 12 |
| Tool results are not reproducible | Medium | Capture versions, inputs, outputs, hashes, and fixture tests. | `OPS-INT` Section 6; `ECOSYS-PLAN` Sections 9.4 and 11 |

---

## 18. Decision Gates

Source basis: `PRD-AMEND` Section 19; `ECOSYS-PLAN` Section 14; `OPS-INT` Section 15.

### Gate 1 - Strategic Adoption

Trace: `PRD-AMEND` Sections 3 and 19; `ECOSYS-PLAN` Gate 1.

Question:

- Do we accept Domain Engine Integration as a Chirality product direction?

Evidence:

- planning record;
- PRD amendment review;
- architectural boundary review;
- risk acceptance.

Decision states:

- accepted;
- accepted for future release;
- deferred;
- rejected.

### Gate 2 - Specification Readiness

Trace: `PRD-AMEND` Section 19 items 2-4; `ECOSYS-PLAN` Gate 1 and Phase 1 exit gate; `OPS-INT` Phase 0.

Question:

- Are the domain profile, adapter, write-boundary, and proposal specs precise enough to implement?

Evidence:

- draft specs;
- OpenPipeStress example profile;
- fixture layout;
- validation rules.

### Gate 3 - Read-Only Safety

Trace: `PRD-AMEND` Section 17 Stage B; `OPS-INT` Phase 2; `ECOSYS-PLAN` Gate 3.

Question:

- Can Chirality safely discover and read domain-engine artifacts without mutation risk?

Evidence:

- profile validator tests;
- artifact scanner tests;
- protected path guard tests;
- fixture run.

### Gate 4 - Tool Invocation Safety

Trace: `PRD-AMEND` Section 17 Stage C; `OPS-INT` Phase 3; `ECOSYS-PLAN` Gate 4.

Question:

- Can Chirality safely invoke deterministic domain tools without bypassing governance?

Evidence:

- adapter contract;
- invocation logs;
- path constraints;
- no protected-path writes;
- failure-mode tests.

### Gate 5 - Proposal Workflow Safety

Trace: `PRD-AMEND` Section 17 Stage D; `OPS-INT` Phase 4; `ECOSYS-PLAN` Gate 5.

Question:

- Can agents propose domain changes without those proposals being mistaken for accepted domain truth?

Evidence:

- proposal schema;
- validation workflow;
- human disposition workflow;
- boundary notices;
- no direct model mutation.

### Gate 6 - Platformization Readiness

Trace: `ECOSYS-PLAN` Gate 6 and Program Release 6; `PRD-AMEND` Section 16; `OPS-INT` future Level 4 framing.

Question:

- Has the OpenPipeStress pattern proven general enough to support additional domain engines?

Evidence:

- stable profile schema;
- second fixture profile;
- limited OpenPipeStress-specific core code;
- reusable documentation.

---

## 19. Open Clarifying Questions

Source basis: open implementation choices surfaced by `PRD-AMEND` Sections 18-19, `OPS-INT` Sections 6, 9, and 13, and `ECOSYS-PLAN` Sections 5, 7, 13, and 18.

These questions do not block the planning record, but they should be answered before governed implementation.

1. Should the PRD amendment be directly incorporated into the current repo PRD file under the intended `PRD_CHIRALITY_APP.md` naming, reconciled with the current `docs/PRD.md` path, or first become a separate governed `DOMAIN_ENGINE_INTEGRATION_PRD.md`? Source: product owner clarification; `PRD-AMEND` header and Section 18.
2. Should OpenPipeStress be treated as a committed first integration, or as an example profile until its own repository/tooling is ready? Source: `PRD-AMEND` Section 12; `ECOSYS-PLAN` Phases 1-3.
3. Where should trusted bundled domain profiles live? Source: `PRD-AMEND` Sections 9 and 11; `OPS-INT` Section 10 EP-C-DOM-001.
4. Are project-local profiles allowed, and if so, what human approval step makes them trusted? Source: `PRD-AMEND` Sections 9-11; `OPS-INT` Section 9.
5. Should Chirality initially support only read-only domain tools, or should domain-controlled output generation be in the first release? Source: `OPS-INT` Levels 1-2; `ECOSYS-PLAN` Program Releases 1-2.
6. Should the protected path guard be implemented at API/tool-call boundaries only, or also in agent instruction/runtime wrappers? Source: `PRD-AMEND` Sections 7.4 and 15; `OPS-INT` EP-C-DOM-003.
7. What is the exact current network policy for domain tools that may internally call external license servers or vendor APIs? Source: `PRD-AMEND` Sections 6 and 8; `ECOSYS-PLAN` Risk Register.
8. What level of external prover result support is desired in the first OpenPipeStress demonstration: manual notes only, structured result states, or no external results? Source: `OPS-INT` Level 4 and Phase 6; `ECOSYS-PLAN` Phase 8.
9. Should private rule packs and material libraries be completely invisible to agents by default? Source: `OPS-INT` EP-OPS-CHIR-006; `ECOSYS-PLAN` Workstream E.
10. What human approval artifact should bind proposal application to a specific git SHA or domain state hash? Source: `PRD-AMEND` Sections 7.2 and 14; `OPS-INT` Section 6.11; `ECOSYS-PLAN` Program Release 5.

---

## 20. Immediate Recommended Next Steps

Source basis: `OPS-INT` Section 14; `ECOSYS-PLAN` Section 18; `PRD-AMEND` Section 19.

1. Keep the three bigger-picture documents in `docs/thesis/bigger-picture/` as authoritative source material until the product owner declares otherwise. Source: product owner instruction; `PRD-AMEND`, `OPS-INT`, `ECOSYS-PLAN`.
2. Use this plan as the current adoption record in `plans/`. Source: this planning request; `ECOSYS-PLAN` Phase 0.
3. Decide whether Domain Engine Integration is accepted as a future product direction. Source: `PRD-AMEND` Section 19; `ECOSYS-PLAN` Gate 1.
4. If accepted, draft `docs/DOMAIN_ENGINE_PROFILE_SPEC.md` first. Source: `OPS-INT` Section 14; `ECOSYS-PLAN` Section 18.
5. Draft an OpenPipeStress example profile against that spec. Source: `PRD-AMEND` Sections 11-12; `OPS-INT` Phase 0.
6. Define protected path matching rules and write guard behavior before any tool invocation work. Source: `PRD-AMEND` Sections 7.4 and 15; `OPS-INT` EP-C-DOM-003.
7. Create a minimal fixture project with generated summaries and protected canonical files. Source: `OPS-INT` Section 12.3; `ECOSYS-PLAN` Workstream F.
8. Implement read-only artifact scanning and path classification. Source: `PRD-AMEND` Stage B; `OPS-INT` Phase 2; `ECOSYS-PLAN` Program Release 1.
9. Add tests proving Chirality does not write protected domain paths. Source: `PRD-AMEND` Section 15; `OPS-INT` Section 12.1.
10. Only after read-only safety is proven, add deterministic read-only tool invocation. Source: `OPS-INT` Levels 1-2 and Phase 3; `ECOSYS-PLAN` Gates 3-4.
11. Defer proposal application, external prover automation, GUI embedding, and platformization until later gates. Source: `PRD-AMEND` Section 8.2; `OPS-INT` Level 4; `ECOSYS-PLAN` Releases 5-7.

---

## 21. Success Definition

Source basis: `OPS-INT` Section 15; `ECOSYS-PLAN` Sections 12 and 16; `PRD-AMEND` Section 19.

The first successful release of this direction should demonstrate:

1. Chirality can detect a domain-engine project inside a working root.
2. Chirality can validate a domain-engine profile.
3. Chirality can distinguish authoritative, readable, protected, and agent-writable artifacts.
4. Chirality can give agents bounded summaries without exposing protected/private domain truth.
5. Chirality can block direct writes to protected domain paths.
6. Chirality can invoke a declared read-only deterministic domain command, if the adapter exists.
7. Chirality records invocation evidence in git-visible files.
8. Agents can draft review notes, TBDs, checklists, and report text from generated domain summaries.
9. Agents cannot fabricate solver results, external validation, code compliance, or professional acceptance.
10. The pattern remains general enough that OpenPipeStress is the first profile, not a one-off hard-coded integration.

Trace: items 1-5 are drawn from `PRD-AMEND` Section 19 and `ECOSYS-PLAN` Gate 3; items 6-8 are drawn from `OPS-INT` Section 15 and `ECOSYS-PLAN` Program Releases 1-3; items 9-10 are drawn from `PRD-AMEND` Sections 14 and 16 and `ECOSYS-PLAN` Section 16.

---

## 22. Bottom Line

Source basis: guiding statements in `PRD-AMEND` Section 20, `OPS-INT` Section 16, and `ECOSYS-PLAN` Section 19.

The bigger-picture documents point in the right direction. The architecture is strongest when it stays federated:

- Chirality governs work.
- Domain engines compute and preserve model truth.
- Agents propose, summarize, and reconcile.
- External professional tools validate where required.
- Humans accept and issue.
- Files and git preserve the record.

The next move should be disciplined adoption: write the generic profile spec, prove read-only safety with a fixture, and resist implementing mutation or professional-validation workflows until the boundaries are enforceable.
