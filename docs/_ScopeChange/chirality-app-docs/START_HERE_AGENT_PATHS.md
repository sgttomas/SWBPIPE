# Start Here: Learning Paths for Working with Agents

This document is the onboarding entry point for newcomers who want to learn how to work with and build with agents in this project.

You can start from one of three paths:

1. `docs/` only
2. `docs/` + decomposition file
3. `docs/` + decomposition file + semantically enriched deliverable folders

Each path gives a different view of agent behavior and a different level of operational depth.

1. Will teach you how to do a decomposition of a scope of work using SOFTWARE_DECOMP.
2. Will teach you how to use agents to scaffold and enrich a project directory using a tree structure, and what standard documentation is required. You will primarily interact with ORCHESTRATOR. You can also learn how to run subagents in parallel all at the same time.
3. Will teach you how to run agents over task loops and how to run agent teams along directed acyclic graphs. You will primarily interact with TASK for this.

Software is a great medium to teach about agents, and this `chirality-app-dev` repository is intentionally a software development project for learning Chirality agent workflows end-to-end.

Whichever path you choose, don't forget to purge the remaining documents so you're starting from the proper "clean slate" for your chosen path.

If you want them back later on they're always backed up at the original location on GitHub `https://www.github.com/sgttomas/chirality-app-dev`

---

## Project Objective (What Completion Produces)

If this project is taken to completion, the result is:

- A working Chirality desktop harness that runs agents against a user-selected working root (`projectRoot`) with filesystem-as-state.
- A governed, auditable agent operating system (agent instructions + governance docs + decomposition + lifecycle + dependency records).
- A complete in-scope deliverable set progressed through lifecycle to issuance, with immutable evidence under tool roots (reconciliation, closure, change, estimates/schedule where applicable).

In short: a deployable agent execution environment plus the full governance and execution artifacts needed to operate it responsibly.

---

## Choose a Start Path

| Path | Use When | What You Will Learn | Typical Agent Interaction Depth |
|---|---|---|---|
| Path 1: `docs/` only | You are brand new and want the mental model first | Governance, invariants, vocabulary, lifecycle, and role boundaries | Conceptual and policy-level interactions |
| Path 2: `docs/` + decomposition | You want to operationalize scope into working structure | How scope becomes packages/deliverables/objectives/open issues, then gets scaffolded and semantically enriched | Structure and sequencing interactions |
| Path 3: enriched deliverables from Path 2 | You want real execution behavior on prepared units | How agents perform local work, manage dependencies, run control loops, and produce evidence | Full execution interactions |

---

## Path 1: `docs/` Only

### Inputs

- `docs/DIRECTIVE.md`
- `docs/CONTRACT.md`
- `docs/SPEC.md`
- `docs/TYPES.md`
- `docs/PLAN.md`

### What to Do with Agents

Use persona agents to interpret and explain the framework:

- `HELP_HUMAN`: ask for a newcomer briefing, glossary, and workflow checklist.
- `HELPS_HUMANS`: ask why the instruction architecture is structured as Type 0/1/2.
- `SOFTWARE_DECOMP`: ask for a simulated decomposition walkthrough without execution writes.

### First Prompt Example

```text
Read docs/DIRECTIVE.md, docs/CONTRACT.md, docs/SPEC.md, docs/TYPES.md, and docs/PLAN.md.
Then explain the Chirality model to me as a newcomer:
1) what the system is optimizing for,
2) how Type 0/1/2 roles differ,
3) what a safe first hands-on exercise should be.
Use HELP_HUMAN as your persona.
```

### Expectations

- You will understand intent, constraints, and terminology.
- You will have insight into the categories used to take a complex scope and break it down into manageable tasks.
- You can reason about what should happen, but not yet verify what is happening in a concrete instance.

### Exit Criteria

You can explain:

- what “filesystem is the state” means,
- why humans remain final authority,
- why categories and types are essential for reliable agentic behaviour
- how Type 0/1/2 responsibilities differ.

### Expected Artifacts Checklist

- [ ] You have read `docs/DIRECTIVE.md`, `docs/CONTRACT.md`, `docs/SPEC.md`, `docs/TYPES.md`, and `docs/PLAN.md`.
- [ ] You have a HELP_HUMAN-produced newcomer briefing (in chat transcript).
- [ ] You have a SOFTWARE_DECOMP decomposition of the `docs/`.

---

## Path 2: `docs/` + Decomposition File

### Inputs

- All Path 1 inputs
- Active decomposition file under `execution/_Decomposition/`
  - Current project example: `execution/_Decomposition/ChiralityApp_SoftwareDecomposition_2026-02-21_G7-APPROVED.md`

### What to Do with Agents

Use persona agents to work from real scoped structure:

- `ORCHESTRATOR`: request tier/blocker-front summaries from decomposition + coordination policy.
- `SCOPE_CHANGE`: simulate or perform amendment flow (intake -> impact -> propagation).
- `PREPARATION`: scaffold package and deliverable folders with the minimum metadata files (`_CONTEXT.md`, `_STATUS.md`, `_REFERENCES.md`, `_DEPENDENCIES.md`, `_SEMANTIC.md`, `MEMORY.md`).
- `4_DOCUMENTS`: create the initial production document kit for each deliverable (Datasheet, Specification, Guidance, Procedure).
- `CHIRALITY_FRAMEWORK`: generate the semantic framework artifact (`_SEMANTIC.md`) and help move deliverables toward `SEMANTIC_READY`.
- `CHIRALITY_LENS`: run lensing analysis (`_SEMANTIC_LENSING.md`) to expose gaps, ambiguity, and decision pressure before implementation.
- `DEPENDENCIES`: extract and normalize deliverable-local dependency registers (`Dependencies.csv` + `_DEPENDENCIES.md`) from source documents.
- `AUDIT_DEP_CLOSURE`: run closure/health checks across dependency graphs (acyclicity, blockers, unresolved links, and topology quality).

### First Prompt Example

```text
Read README.md, AGENTS.md, docs/, and execution/_Decomposition/ChiralityApp_SoftwareDecomposition_2026-02-21_G7-APPROVED.md.
Using ORCHESTRATOR, walk me through converting this decomposition into enriched deliverables.
Include:
1) scaffolding sequence (PREPARATION),
2) document kit sequence (4_DOCUMENTS),
3) semantic enrichment sequence (CHIRALITY_FRAMEWORK + CHIRALITY_LENS),
4) dependency extraction and closure audit sequence (DEPENDENCIES + AUDIT_DEP_CLOSURE).
Give me the exact order and expected outputs per step.
```

### Expectations

- You gain scope truth: stable IDs, package boundaries, deliverable responsibilities, objective mappings, open-issue traceability.
- You can understand sequencing intent and change-control behavior.
- Outcome: deliverable folders are scaffolded, documented, semantically enriched, dependency-mapped, and closure-audited. This enriched deliverable state is the starting point for Path 3.

### Exit Criteria

You can:

- trace any `SOW-*` to `PKG-*` and `DEL-*`,
- explain current scope IN/OUT decisions and their implications,
- describe how a scope change propagates without breaking ID stability.

### Expected Artifacts Checklist

- [ ] Active decomposition file exists under `execution/_Decomposition/`.
- [ ] Package and deliverable folders are scaffolded under `execution/PKG-*/1_Working/DEL-*`.
- [ ] Each targeted deliverable has minimum metadata files:
  `_CONTEXT.md`, `_STATUS.md`, `_REFERENCES.md`, `_DEPENDENCIES.md`, `_SEMANTIC.md`, `MEMORY.md`.
- [ ] Each targeted deliverable has document kit files:
  `Datasheet.md`, `Specification.md`, `Guidance.md`, `Procedure.md`.
- [ ] Semantic enrichment files are present where expected:
  `_SEMANTIC.md` and `_SEMANTIC_LENSING.md`.
- [ ] Dependency extraction outputs are present:
  `Dependencies.csv` + `_DEPENDENCIES.md` with populated run notes.
- [ ] At least one closure audit snapshot exists under `execution/_Reconciliation/DepClosure/` and is pointed to by `_LATEST.md`.

---

## Path 3: Enriched Deliverable Folders (from Path 2)

### Inputs

- Starting point: enriched deliverable folders produced at the end of Path 2.
- Deliverable folders under `execution/PKG-*/1_Working/DEL-*` including metadata and production docs
  - `_CONTEXT.md`, `_STATUS.md`, `_REFERENCES.md`, `_DEPENDENCIES.md`, `Dependencies.csv`, `MEMORY.md`
  - production docs (PROJECT/SOFTWARE: Datasheet/Specification/Guidance/Procedure)
  - optional semantic artifacts (`_SEMANTIC.md`, `_SEMANTIC_LENSING.md`)

### What to Do with Agents

Use full operational workflow:

- `WORKING_ITEMS` + `TASK`: execute deliverable-local work in bounded sessions.
- `RECONCILIATION` + `AUDIT_DEP_CLOSURE`: validate cross-deliverable graph health.
- `CHANGE`: publish coherent commits after verification.

### First Prompt Example

```text
Using WORKING_ITEMS as your persona for orchestration with TASK agent execution, run one tier control loop:
1) identify active-front deliverables from blocker policy,
2) execute bounded deliverable-local work,
3) refresh dependencies only on touched deliverables,
4) run interface reconciliation,
5) produce a handoff-ready state update.
Show exactly which files should change at each step.
```

### Expectations

- You see actual agent execution behavior: local writes, evidence-backed updates, reruns, lifecycle movement, and control-loop fan-in/fan-out.
- You can compare policy intent vs implementation state and detect drift.
- You can run real progression cycles from unblocked work through checking/issuance with auditability.

### Exit Criteria

You can:

- execute a tier control loop end-to-end,
- validate dependency and reconciliation outputs,
- produce a clean handoff state for the next session.

### Expected Artifacts Checklist

- [ ] At least one control-loop report is produced under `execution/_Coordination/` (for example `TIER*_CONTROL_LOOP_*.md`).
- [ ] At least one interface reconciliation report is produced under `execution/_Reconciliation/` (for example `TIER*_INTERFACE_RECON_*.md`).
- [ ] Touched deliverables show lifecycle and evidence updates (`_STATUS.md`, dependency run notes, and relevant production docs).
- [ ] If publication is part of the run, CHANGE output includes a committed git revision.
- [ ] `execution/_Coordination/NEXT_INSTANCE_STATE.md` is updated with current pointers, graph truth, and next actions.

---

## Recommended Learning Progression

1. Start in Path 1 to establish vocabulary and constraints.
2. Move to Path 2 to understand decomposition as operational authority.
3. Move to Path 3 to run real agent workflows and produce verifiable outputs.

---

## Safety and Responsibility Notes

- Treat agent output as draft or structured assistance until reviewed.
- Keep project truth on disk; do not rely on hidden memory.
- Respect write scopes and lifecycle authority (`_STATUS.md`).
- Use `MEMORY.md` for deliverable-local memory; do not create `_MEMORY.md` in this project profile.
