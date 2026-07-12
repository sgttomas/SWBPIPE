# WORKING_ITEMS Run: D-41 R5 T4 PDU-001/PDU-038

- Date: 2026-07-12
- Deliverable: `DEL-17-06`
- Authority: `DEC-074` O11/E7; R5 T4 bounded application/export seam
- Epistemic status: contract and fixture verification, not validation

Added conditional unresolved-assumption and reproducibility-reference inputs to
the stress-neutral builder. The references pass through unchanged to the
package and hash-bound manifest seed, with strict-schema properties and a
mutation-resistance regression.

References remain unresolved/reference-only. No comparison tolerance,
pass/fail semantic, target compatibility, solver validation, or professional
disposition was added.

Validation: focused cache-disabled export tests passed 35/35; full
cache-disabled project tests passed 493/493; `git diff --check` passed.
Lifecycle remains `IN_PROGRESS`; the exact D-41 bootstrap is preserved for T7.
