---
doc_id: OPS-INIT
doc_kind: governance.bootstrap
status: current_bootstrap
created: 2026-04-30
---

# INIT — OpenPipeStress Agent Bootstrap

OpenPipeStress is a free and open-source, code-neutral piping stress analysis project. The software shall implement open analytical mechanics for global piping flexibility/stress analysis while requiring users to supply code-specific and proprietary engineering data.

This bootstrap is maintained only at the repository root. The former `docs/INIT.md` copy is retired to avoid split bootstrap authority.

All agentic development work must preserve four boundaries:

1. **Open mechanics vs. protected standards data** — public code may implement mechanics and workflows; protected code tables, text, examples, and proprietary values are not public project content.
2. **Mechanics solve vs. user rule check** — a solved model is not the same thing as a code check.
3. **User rule check vs. professional approval** — software output is decision support until a competent human accepts it for project use.
4. **Global centerline analysis vs. local FEA** — routine pipe stress analysis uses a 3D line-element/frame model; shell/solid FEA is a specialized local handoff.

## Required reading order

1. `docs/DIRECTIVE.md` — founding intent and stop rules.
2. `docs/CONTRACT.md` — invariant catalog.
3. `docs/TYPES.md` — vocabulary, identifiers, statuses, and domain terms.
4. `docs/SPEC.md` — technical architecture and implementation mechanics.
5. `docs/IP_AND_DATA_BOUNDARY.md` — public/private data boundary.
6. `docs/VALIDATION_STRATEGY.md` — verification and release-quality expectations.
7. `docs/AGENTIC_DEVELOPMENT_WORKFLOW.md` — how agents execute deliverables.
8. `execution/_Decomposition/SOFTWARE_DECOMP.md` — current package/deliverable working surface.
9. `docs/_Registers/*.csv` — machine-readable scope, deliverable, and context-budget registers.

## Agent rule

Unknown engineering values become `TBD`. Suspected protected data is quarantined. Conflicts are surfaced. No agent may claim certification, code compliance, approval, sealing, or professional reliance.

## Next step

After reading these files, use the accepted `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.5 basis. Revision 0.5 includes the SCA-001 `PKG-00 — Software Architecture Runway` architecture gate and the accepted SCA-002 physical-model, design-knowledge, operation, state/run/comparison, handoff, external-prover-boundary, and GUI comparison scope. Downstream coordination surfaces remain stale until refreshed through `plans/SCA-002_DOWNSTREAM_REFRESH_PLAN.md`. Type 2 execution begins only from refreshed sealed `DEL-XX-YY` context, explicit write scope, applicable invariants, and acceptance criteria.
