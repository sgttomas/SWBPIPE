# Guidance: DEL-11-05 Contributor tutorial and onboarding

## Purpose

The contributor tutorial should make the first hour of contribution work predictable: read the governing documents, locate the active deliverable, understand the data and authority boundaries, work only inside the assigned scope, produce evidence, and hand off for review. The tutorial is a path through the existing governance system, not a replacement for `CONTRIBUTING`, `docs/AGENTIC_DEVELOPMENT_WORKFLOW.md`, or human review.

## Principles

1. Start with boundaries before mechanics. Contributors need the OpenPipeStress rule that public code may implement open mechanics while protected standards and proprietary engineering data remain outside public defaults.
2. Treat the decomposition as the work identity. A contributor works on one `DEL-XX-YY` inside one `PKG-XX` unless a human changes scope.
3. Keep authority labels clear. Type 2 outputs are drafts/proposals until accepted; deterministic tools provide evidence, not judgment; humans resolve scope changes and acceptance gates.
4. Prefer `TBD` over invention. Missing policy choices, engineering values, source citations, or legal conclusions are surfaced rather than guessed.
5. Keep examples clean. Public examples are invented, original, public-domain, or permissively licensed with provenance. They do not copy protected standards examples or commercial software cases.

## Considerations

### Contributor Path

A contributor-facing tutorial should guide the reader through this sequence:

1. Read `INIT.md`, `AGENTS.md`, `docs/CONTRACT.md`, `docs/TYPES.md`, `docs/SPEC.md`, and the active decomposition/register rows relevant to the assigned deliverable.
2. Confirm the sealed deliverable identity, package, scope items, objectives, invariants, acceptance criteria, and explicit write scope.
3. Inspect `_CONTEXT.md`, `_REFERENCES.md`, `_DEPENDENCIES.md`, and `_STATUS.md` before changing production artifacts.
4. Make only scoped edits and record evidence through validation commands, warnings, and open issues.
5. Return a handoff with changed paths, validation results, data-boundary notes, and review readiness.

### Protected-Data Boundary

Contributor onboarding should be explicit that the public repository may contain schemas, mechanics, workflows, blank templates, and invented or permissively licensed examples, but not protected standards text/tables/examples, copied code formulas, proprietary vendor catalogs without rights, private owner standards, private rule packs, or company design bases. If a source appears protected, the tutorial should instruct the contributor to stop, mark the concern, and escalate through the project review path rather than paraphrase it.

### Professional Responsibility Boundary

The tutorial should avoid language that sounds like certification. A maintainer can accept a development artifact, but that is not a professional engineering approval of a piping calculation. A user rule-check result is not a professional code-compliance declaration. Software output remains decision support until a competent human accepts it for a particular project.

### Architecture-Basis Handling

SCA-001 allows `PKG-00` `SEMANTIC_READY` content to be used as an architecture-basis candidate for sealed brief injection. Contributors should use applicable basis IDs as context constraints, especially the no-bypass API/adapter baseline, without treating PKG-00 as `ISSUED` or copying full PKG-00 prose into new artifacts.

## Trade-offs

| Trade-off | Guidance |
|---|---|
| Helpful tutorial vs. over-specific process | Provide a concrete path through existing files, but keep repo-level publication and policy decisions separate from this deliverable. |
| Contributor confidence vs. certification language | Explain what maintainers review and what tools check, but avoid any statement that software or agents approve engineering work. |
| Rich examples vs. IP risk | Use invented examples and provenance notes rather than real protected code examples or commercial software files. |
| Automation vs. human authority | Use scripts for evidence and repeatability; preserve human gates for scope, acceptance, legal/professional questions, and ambiguous protected content. |

## Examples

### Safe Onboarding Step

> Confirm `DeliverableID`, `PackageID`, `Scope Coverage`, `Objective Support`, write scope, and applicable invariants from `_CONTEXT.md` before editing.

This is a process example only. It does not introduce engineering values or standards material.

### Unsafe Onboarding Step

> Copy a code example from a standard or commercial report and adapt it into public docs.

This violates the protected-content boundary unless documented redistribution rights and human/legal review explicitly allow it.

## Conflict Table (for human ruling)

No source conflicts were detected during setup. If future repo-level onboarding asks this deliverable to edit `CONTRIBUTING` or `docs/AGENTIC_DEVELOPMENT_WORKFLOW.md` directly, that is a scope change from this setup session and must be routed for human approval.
