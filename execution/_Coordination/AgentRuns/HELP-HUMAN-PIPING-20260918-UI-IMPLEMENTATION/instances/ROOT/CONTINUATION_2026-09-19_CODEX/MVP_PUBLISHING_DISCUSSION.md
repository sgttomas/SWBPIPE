# MVP publishing target — discussion draft

Status: ROOT recommendation for owner discussion, 2026-09-20. The owner corrected the optional-agent reply: agent control of SWBPIPE is required to enable planned CAEPIPE validation (OWNER_MVP_AGENT_CORRECTION_2026-09-20.md). Other recommendations remain proposed. This does not amend the PRD, initiate a new phase, reconcile deliverables, authorize Runtime integration, lift qualification holds or publish a product. Current B3B/C3/CI work continues under existing authority.

## Owner's current steering

> Very good.  Carry on as HELP_HUMAN in your Agent 0 role down that path.  What do you think is a good target for the "MVP" product that we can focus on for publishing?  We're still building out basic functionality at this point.  But we should discuss what "finished" looks like.  You will already have guidance in this regard, but we should probably update it and sharpen it.

> Keep in mind that the PRD is a basis, however our understanding can evolve through the implementation.  If a better or clearer idea emerges, and that's a preferable path to follow, we can adopt that, with amendments to the PRD.  However, it is a good basis.

ROOT interpretation: use the adopted PRD as a starting basis, evaluate requirements against observed implementation and user value, and propose explicit amendments when a better product boundary emerges. This is permission to develop the proposal; no particular changed requirement has yet been adopted.

## Recommended product promise

**A piping designer can develop a small route/support concept, understand the mechanical consequences of a change, and hand another person a reproducible model and result package.**

Recommend a first public v0.1 design preview aimed at qualified piping designers and engineers performing early design iteration. Define MVP by this complete job and a declared supported model envelope. The exact public maturity label remains proposed; it must match the evidence and final owner release ruling.

The reference journey is:

1. Install and launch the packaged application; open an invented example or start a blank local project.
2. Define a small supported pipe route, materials/sections, restraints and loads through the interface, with clear units and actionable missing-input feedback.
3. Solve and inspect deformation, forces/moments, reactions, supported stress outputs and diagnostics; identify the model basis and standing of the displayed result.
4. Change a route or support, review/apply the change and use Undo/Redo. Preserve a baseline and compare it with the changed design and its run.
5. Save, close and reopen without losing model data or confusing results from another basis.
6. Export a canonical package and readable result/report record that another person can reopen or reproduce from its recorded inputs.

The acceptance story should include an invalid input, an interrupted/failed operation and a stale result or proposal. These exercise ordinary recovery, not only an uninterrupted demonstration.

## Proposed first-release boundary

| Area | Recommended initial commitment | Evidence or proposed simplification |
|---|---|---|
| Model and mechanics | A declared linear-static 3D subset, initially straight/rigid elements and linear restraints with supported weight, thermal and imposed-displacement inputs, pressure metadata and fundamental result recovery. | PRD §23. Select representative models and exact supported combinations before claiming coverage; existing broader code alone does not qualify them. |
| Authoring and review | A user can complete the journey through the native UI without raw-file editing or developer intervention. All editing routes preserve the same validation, history and result invalidation. | PRD §24 R2; current loop connected-journey requirement. |
| Design iteration | Save an immutable baseline, make one alternative and compare relevant model and result changes. | Keeps §23's basic state/run comparison. Recommend this focused experience before a general mapping editor or extensive comparison-management UI; any incompatible requirement gets an explicit amendment. |
| Private data/rules | Safe local/private data boundaries, required inputs, private rule-pack schema and an invented non-code example. | PRD §23. Full library/catalogue breadth and a sophisticated rule authoring environment need not expand the initial user promise. |
| Handoff | Canonical schema-compliant package plus a readable report with reproducible basis and limitations. | PRD §23. A direct adapter for a named commercial tool is a separate promise and qualification task. |
| Agents | Establish reliable agent control through the shared operation/controller seam before planned CAEPIPE validation; the agent uses Computer Use for CAEPIPE. | Owner corrected the earlier optional-agent answer on2026-09-20. Runtime/Piping leads must agree concrete interfaces, disjoint writes and order before integration writes; broad public agent UX is not yet specified. |
| Platform | Start with the adopted Apple Silicon macOS target; settle a usable installation/signing experience before publication. | DEC-057 and later DEC-089. No cross-platform promise without its own evidence. |

“Small” is not yet a numeric capacity claim. The reference-model envelope and second performance profile need the existing owner freeze and qualification process. No tolerance, performance limit, accessibility criterion or usability hold is relaxed by this proposal.

## Observable meaning of finished

- **Useful:** the reference journey works from the installed package using supported, representative models, with a short getting-started guide.
- **Dependable:** edits, review, cancellation, recovery, save/reopen, history and result standing behave correctly; no known blocking loss/corruption or misleading-success defect remains.
- **Credible:** required mechanics and workflow evidence binds to the release candidate, supported behavior is documented, and unsupported cases fail clearly. Internal verification and external-prover correlation retain their distinct meanings.
- **Usable:** native controls, keyboard use, supported window sizes and touched accessibility criteria work; the applicable independently conducted usability/qualification work remains subject to its existing owner holds.
- **Reproducible:** the exported package identifies the model/run basis and another person can reproduce the declared example through the documented procedure.
- **Publishable:** the actual package, release notes, scan/provenance records, public-export boundary and installation evidence are ready for the owner's release decision.

This is a proposed product finish line, not a new checklist of already-passed gates.

## What the current graph contributes

The current UI graph is the route to a coherent workbench: persistence/visibility now; tables and labels next; inspector/results/review and HUD/camera thereafter; then compatible view-state persistence, closing appearance and the applicable qualification.

It does not yet prove complete user-facing state/run comparison, private-rule workflows, onboarding, packaging or public-export readiness. Those are questions for a focused capability-to-journey inventory, not assumptions of missing code. The next planning artifact should list each reference-journey step, actual existing capability, remaining gap and smallest bounded change. This can be prepared without rebuilding the project DAG or beginning deliverable reconciliation.

## Changes to sharpen after owner discussion

Recommend one concise release-target addendum that names the audience, reference journey, supported envelope, required versus optional capabilities and observable exit criteria. Amend PRD §23/§24 only where the adopted target changes their meaning. Correct later-rule drift in release guidance separately: BUILD_AND_RELEASE still carries old CI/signing prose, and RELEASE_QUALITY_GATES' engineering-beta wording is not aligned with the newer VALIDATION_STRATEGY wording.

The owner corrected the initial agent-dependency answer: agent control of SWBPIPE enables the planned CAEPIPE validation and belongs before that milestone. The bounded first integration target remains inspect → propose → preview → accept → Undo → stale rejection, followed by the supported solve/export/read-result operations needed for a validation case. The Runtime and Piping leads agreed a Piping-owned stdio MCP facade and private live bridge, with peer read-only review. Exact interfaces and bounded activation are being prepared. The owner selected this Mac with a Windows environment and Codex as first controller, with embedding later. The specific guest/provider and live tool connection still need actual verification. This requirement does not imply that a broad general-purpose agent interface must be complete before the first publication.

## Evidence basis and limits

- Adopted docs/PRD.md §§1, 3, 9, 22.1/22.5, 23 and 24; source SHA256 cd6297af2ed9588f6da24c0665d32c74ee33d3e7f9115e733b47244fd75b4611.
- docs/RELEASE_QUALITY_GATES.md §§3–8; docs/VALIDATION_STRATEGY.md §4.
- DEC-057/058/059, DEC-089 signing ruling, D-72 ruling and addendum, and current run WORK_GRAPH.json.
- Bounded read-only inventory: MVP_BASIS_INVENTORY_BRIEF.md, SHA256 d870e751f67179d41118e9894f65b497951ce71aa939d600b2d05544a6c9d5b3; retained return _run_records/MVP_BASIS_INVENTORY_RETURN.md.
- Inventory used committed ROOT basis e8f91e086a0ce65ca706bd63e0a3c6e4ef21f15a. Current UI combined source is separately under review at 52142dbdaec190f3a7fdb47a4f937c3e23720824; its passing development checks do not establish release readiness.

The owner's latest agent-control-for-CAEPIPE-validation correction supersedes the initial optional-agent reply. Other product recommendations and held decisions remain open. Standard claim fence applies (F-PIP-2; DEC-081).
