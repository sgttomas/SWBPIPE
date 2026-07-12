# Procedure: DEL-07-02 Model tree and property inspector

## Purpose

Define the bounded procedure for maintaining and extending model-tree/property-inspector behavior within the GUI state/interaction architecture owned by `DEL-00-05`, without transferring architecture authority or expanding functionality.

## Prerequisites

- Confirm the sealed brief names `DEL-07-02` and the approved implementation write scope before any GUI source or test file is edited.
- Confirm the accepted `DEC-074` O2 delegation: `DEL-00-05` owns GUI state/interaction architecture and `DEL-07-02` owns model-tree/property-inspector behavior implementation within it.
- Confirm upstream architecture basis constraints from `AB-00-03`, `AB-00-05`, `AB-00-06`, `AB-00-07`, and `AB-00-08`.
- Confirm accepted domain/schema contracts for entity identity, unit-bearing values, provenance, diagnostics, rule-pack references, and private-library references.
- Confirm accepted application-service command/query contracts for inspector edits and tree/selection reads.
- Confirm all UI fixtures, screenshots, and examples are synthetic, public-domain, or otherwise cleared for repository use.

## Steps

1. Re-read `_CONTEXT.md`, `Specification.md`, `_DEPENDENCIES.md`, and any accepted upstream schema/service contracts.
2. Identify the entity types and field groups that `DEL-07-02` owns for tree navigation and selected-entity inspection without redefining `DEL-00-05` state/interaction architecture.
3. Define tree-to-viewport-to-inspector selection behavior using transient GUI state and stable model identities.
4. Define property inspector read-only/editable states and command-backed mutation paths.
5. Preserve unit display/edit hooks, dimensional validation, provenance, private/public status, and rule-pack checksum/source status in the inspector where relevant.
6. Add visible findings for missing solve-required inputs, missing rule-check inputs, provenance warnings, assumptions, and IP-boundary warnings without creating defaults.
7. Require a bounded implementation brief for any new behavior. Preserve the open residuals for model-level `modulus_basis_records` entry, broader unit entry/pickers, test/factoring hardening, and broader canvas/tree/editor UX until separately executed.
8. Maintain UI tests for tree navigation, selection synchronization, inspector field groups, missing-data visibility, provenance/private status, and command/query boundary behavior.

## Verification

| Check | Expected evidence |
|---|---|
| Scope boundary | Changes are limited to the approved implementation scope for DEL-07-02. |
| Tree/selection behavior | UI tests confirm model tree selection updates the inspector and stays aligned with stable model identity. |
| Inspector behavior | UI tests confirm selected entity type controls visible field groups and command-backed edit behavior. |
| Missing-data behavior | UI tests confirm missing solve-required and rule-check-required values are explicit findings. |
| Unit/provenance behavior | Tests or review evidence confirm unit, provenance, checksum, and private/public status are not dropped. |
| IP/privacy boundary | Fixtures and UI text contain no protected standards content, proprietary values, private project data, or compliance claims. |

## Records

- Implementation notes or pull request summary when code work is authorized.
- UI test results.
- Command/query boundary review evidence.
- Fixture and screenshot provenance notes.
- Protected-content review evidence where applicable.
- Missing-data, provenance, and private-status UI evidence.
- Any human rulings for `TBD` items.
- `DEC-074` O2 ownership-delegation evidence and the corresponding deliverable-local R5 run record.
