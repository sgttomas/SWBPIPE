# Guidance: DEL-11-03 Theory Notes - Classical to Modern Centerline Analysis

## Purpose

This deliverable prepares a bounded theory-note work surface for explaining classical-to-modern centerline analysis in OpenPipeStress. The useful center of the note is educational clarity: help users and developers understand why the project treats routine pipe stress as a global 3D centerline/frame problem while keeping code-specific checks and professional judgment outside solver authority.

## Principles

- Explain mechanics concepts in plain engineering language, but keep unsupported historical claims and source-specific details as `TBD`.
- Prefer public/permissive mechanics and numerical-analysis sources for future citations.
- Treat protected standards as boundaries, not source text for public reproduction.
- Use centerline/frame language consistently: nodes, line elements, local/global frames, supports, loads, displacements, element forces, and result interpretation.
- Distinguish practical global flexibility analysis from local shell/solid FEA; neither mode substitutes for professional acceptance.
- Keep formulas out of the setup draft unless later public/permissive sources and project-specific approval justify them.

## Considerations

The final note should be readable by both users and contributors. Users need enough context to understand model assumptions, limits, and warnings. Developers need enough structure to see how theory maps to solver architecture without treating the documentation as a code specification.

Protected-data risk is the dominant risk for this deliverable. Public prose may discuss open mechanics and general structural-analysis concepts, but it must not recreate code-body tables, code examples, code stress equations, SIF/flexibility tables, proprietary component data, or commercial-software benchmark examples.

The phrase "classical flexibility lineage" should be handled conservatively. Until public/permissive historical sources are selected, the note can state the intended coverage but should not make detailed historical claims about named methods, committees, or code evolution.

Terminology mapping for future prose:

| Term | Use in final theory note | Boundary |
|---|---|---|
| Classical flexibility analysis | Use for source-supported lineage and conceptual background. | Do not imply adoption of any protected code method unless public evidence and review support it. |
| Centerline model | Use for the geometry abstraction: a pipe run represented by a connected line with nodes, elements, supports, and loads. | Do not treat this as a complete local stress/FEA model. |
| 3D frame model | Use for the modern computational implementation idea: line elements with translational and rotational degrees of freedom in global and local frames. | Do not introduce implementation equations or library choices in this documentation deliverable. |
| Rule check | Use only for user-supplied acceptability evaluation after mechanics results. | Do not present the theory note as a code-compliance guide. |

## Trade-offs

| Trade-off | Preferred setup stance |
|---|---|
| Educational clarity vs. formula detail | Prefer conceptual explanation now; formula detail remains `TBD` pending public/permissive sources and protected-content review. |
| Classical terminology vs. modern implementation terms | Use both, but normalize them so "flexibility analysis", "centerline model", and "3D frame model" are not treated as conflicting concepts. |
| Solver explanation vs. implementation specification | Explain the model idea without prescribing code-level equations or numerical-library choices. |
| Public examples vs. protected examples | Use invented, non-code examples only if future work needs examples. |
| Mechanics output vs. compliance output | Mechanics output is solver evidence; acceptability and professional reliance remain user/human-governed. |

## Examples

No numerical examples are introduced in this setup run.

Future examples, if used, must be invented educational examples with non-code values, no protected standard tables, and no compliance claims.

## Conflict Table (for human ruling)

| Conflict ID | Conflict | Source A | Source B | Impacted sections | Proposed authority (PROPOSAL) | Human ruling |
|---|---|---|---|---|---|---|
| None | No current source conflict identified. | N/A | N/A | N/A | N/A | N/A |
