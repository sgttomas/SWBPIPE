---
doc_id: OPS-AGENTS
doc_kind: governance.agent_index
status: active
created: 2026-04-30
revised: 2026-09-19
---

# AGENTS — SWBPIPE Project Instructions

Root `AGENTS.md` and the selected `agents/AGENT_*.md` package govern agent roles
and delegation. This file holds Piping-specific constraints. The recurrent
development procedure is `loop/LOOP_INIT.md`; the init prompt enters it.
The owner's handoff prompt or current directions supply session steering.
An agent's plan, handoff or interpretation does not create owner authority.

## Paths and knowledge sources

Resolve `REPO_ROOT` with `git rev-parse --show-toplevel`. Set `WORKING_ROOT` to
`{REPO_ROOT}/projects/chirality-piping`. Use these anchors or repository-relative
paths in durable instructions and briefs, not machine-specific absolute paths.

The external engineering corpus `domains/piping-design/` has vetted prose,
concepts and design guidance, but its extracted equation artifacts are
unreviewed `pdf2md`/OCR extractions pending the maintainer's manual review.
Consumers may cite the prose for concepts, terminology and approach. Never
present an extracted equation as authoritative; report the artifact's review
status. Never use those equation artifacts as references for physics-model
builds, solver/kernel work or analytical verification. Use the maintainer's
vetted engineering sources. This preserves DEC-043.

## Project boundaries

The domain, privacy, claims, lifecycle and scope-gated integration boundaries
in F-PIP-1 through F-PIP-4 remain subject to their owning rulings. Their retained
definitions are in `loop/WORKPLAN_2026-07-18b_piping_loop.md`, under "Standing
constraints — fences"; later owner rulings govern explicit amendments. Replacing
the old loop procedure does not waive those boundaries. It supersedes F-PIP-5's
deliverable-only work-selection procedure with the session work graph described in `loop/LOOP_INIT.md`.
Do not edit historical fence definitions or infer release/issuance authority.

Write only inside the authorised scope, including required run evidence. Root
governance, agents, workflows, skills and tools need explicit authority for that
work. Do not alter other projects or stage, revert or repair unrelated state.
For authorised dependency-register work, use the canonical type system and the
approved snapshot named by `execution/_DAG/_LATEST.md`; preserve legacy labels
as provenance rather than re-emitting them as current enums.

## Execution attribution

Keep model allocation in the approved run strategy and actual execution records,
not these standing instructions (D-GOV-17 M1-D). A model or harness change must
not silently change role, authority or scope. Reuse transcript parsers, launch
scripts and attribution conventions only when they work in the current host.
Preserve their evidence purpose using available tools; attribute commits
truthfully. Do not copy another harness's model or co-author identity.

## Software checks

Software work uses `software-workflow.json` under the root
`docs/SOFTWARE_WORKFLOW_PROFILE.md` contract. The profile registers checks;
it does not grant authority or replace the review, evidence and owner-held
boundaries in `loop/LOOP_INIT.md`.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
