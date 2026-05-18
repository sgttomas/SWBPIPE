# Procedure: DEL-15-02 Target mapping and unsupported-behavior contract

## Purpose

Define and check the target mapping and unsupported-behavior contract for handoff exports without inventing target-specific behavior, copying private/protected data, or implying professional approval.

## Prerequisites

| Prerequisite | Source | Status |
|---|---|---|
| Current deliverable context and accepted decomposition basis. | `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md` | Available |
| Approved DAG-002 mirror/evidence surface. | `Dependencies.csv`; `_DEPENDENCIES.md` | Available; preserve rows unchanged in this run. |
| Canonical handoff package schema and manifest predecessor. | `Dependencies.csv` DAG-002-E0805 | ACTIVE mirror row |
| Adapter framework predecessor. | `Dependencies.csv` DAG-002-E0806 | ACTIVE mirror row |
| Local FEA handoff data contract predecessor. | `Dependencies.csv` DAG-002-E0807 | ACTIVE mirror row |
| Private data redaction and export controls predecessor. | `Dependencies.csv` DAG-002-E0808 | ACTIVE mirror row |
| Physical-to-analytical transformation predecessor. | `Dependencies.csv` DAG-002-E0809 | ACTIVE mirror row |
| Comparison mapping, tolerance, and export contracts predecessor. | `Dependencies.csv` DAG-002-E0810 | ACTIVE mirror row |
| Exact target list, canonical package container, and target-specific mapping strategy. | `execution/_Decomposition/SOFTWARE_DECOMP.md` OI-015 | TBD |

## Steps

1. Confirm the deliverable identity:
   - Use DEL-15-02, PKG-15, API_CONTRACT, SOW-074, and OBJ-017 from `_CONTEXT.md`.

2. Confirm governing boundaries:
   - Apply unit awareness, provenance, no silent defaults, professional-boundary, protected-content, and private-data constraints from `docs/CONTRACT.md`, `docs/DIRECTIVE.md`, `docs/SPEC.md`, `docs/TYPES.md`, and `docs/IP_AND_DATA_BOUNDARY.md`.

3. Define the contract scope:
   - Include target mapping metadata.
   - Include unsupported-target flags.
   - Include approximate behavior disclosure as an explicit ASSUMPTION from the decomposition wording.
   - Keep exact target names, schema file paths, property names, and taxonomy values as TBD unless accepted source material resolves them.

4. Map upstream handoff context:
   - Treat the canonical handoff package schema and manifest as a required predecessor.
   - Preserve model hash, units manifest, entity IDs, library/rule references, unresolved assumptions, and warnings as handoff context where the final contract supports them.

5. Define unsupported behavior handling:
   - Record unsupported or approximate behavior as explicit flags, findings, diagnostics, or TBD slots.
   - Do not coerce unsupported behavior into defaults.
   - Do not claim target-tool equivalence, validation, certification, approval, or code compliance.

6. Define privacy and protected-content handling:
   - Prefer stable references, checksums, source notes, review state, and redaction status.
   - Exclude private project data, protected standards text, proprietary formulas, proprietary values, and private rule-pack payloads from public artifacts by default.

7. Check dependencies:
   - Read the deliverable-local `Dependencies.csv` as an approved DAG-002 mirror/evidence surface.
   - Preserve all approved DAG-002 rows as ACTIVE for this workflow.
   - Record any conflict with dependency-extract normalization rules rather than rewriting the mirror.

8. Record unresolved decisions:
   - Target list: TBD.
   - Canonical package container: TBD.
   - Target-specific mapping strategy: TBD.
   - Exact unsupported behavior taxonomy values: TBD.
   - Exact schema path and property names: TBD.

## Verification

| Check | Expected result |
|---|---|
| Four-document consistency | Datasheet, Specification, Guidance, and Procedure use the same deliverable ID, package ID, scope item, objective, and TBD boundaries. |
| Source grounding | Non-trivial claims cite `_CONTEXT.md`, decomposition/registers, approved dependency mirror, or listed governing docs. |
| Unsupported behavior | Unsupported and approximate behavior are explicit; no silent defaults are introduced. |
| Privacy/IP boundary | Public artifacts do not copy private/protected payloads. |
| Professional boundary | No field or procedure creates software-generated approval, certification, sealing, authentication, or code-compliance status. |
| Dependency mirror preservation | All existing approved DAG-002 rows in `Dependencies.csv` remain ACTIVE and unmodified. |
| Schema validation | `python3 tools/validation/validate_dependencies_schema.py <deliverable>/Dependencies.csv` passes if `Dependencies.csv` exists. |

## Records

- `Datasheet.md`
- `Specification.md`
- `Guidance.md`
- `Procedure.md`
- `_SEMANTIC.md`
- `_SEMANTIC_LENSING.md`
- `Dependencies.csv`
- `_DEPENDENCIES.md`
- final workflow report from this setup run
