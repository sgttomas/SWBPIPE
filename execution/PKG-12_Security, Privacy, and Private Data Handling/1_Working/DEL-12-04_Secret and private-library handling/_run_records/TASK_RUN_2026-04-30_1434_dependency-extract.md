# TASK Run Record - dependency-extract

run-status: SUCCESS
date: 2026-04-30
package-id: PKG-12
deliverable-id: DEL-12-04
skill: dependency-extract
schema-version: v3.1
scope-path: execution/PKG-12_Security, Privacy, and Private Data Handling/1_Working/DEL-12-04_Secret and private-library handling

## Inputs Read

- `_CONTEXT.md`
- `_REFERENCES.md`
- `_DEPENDENCIES.md`
- `Datasheet.md`
- `Specification.md`
- `Guidance.md`
- `Procedure.md`
- `docs/_Decomposition/SOFTWARE_DECOMP.md`

## Writes

- Overwrote `Dependencies.csv`
- Overwrote `_DEPENDENCIES.md`

## Extracted Rows

- ACTIVE rows: 7
- ANCHOR rows: 3
- EXECUTION rows: 4

## Notes

- Human-owned dependency sections were preserved in `_DEPENDENCIES.md`.
- Best-effort execution edges to `DEL-12-01` and `DEL-12-02` are marked as proposals in notes.
- Architecture-basis document constraints for AB-00-04 and AB-00-07 were extracted from the sealed `_CONTEXT.md` injection.
