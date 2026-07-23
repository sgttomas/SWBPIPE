# N1 launch brief — existing report-package producer reconnaissance

- RequestedBy: `W-PKG08` WORKING_ITEMS.
- RunID: `HELP-HUMAN-PIPING-20260722-DEL0801-REPORT-PACKAGE-R16`.
- ParentInstanceID: `W-PKG08`.
- ChildInstanceID: `N1`.
- PackageID / DeliverableID: `PKG-08` / `DEL-08-01`.
- TaskSkill: `software-repository-reconnaissance` (`ApplyEdits: false`).
- AcceptedBasis: Git SHA `8698b0338ac82556fee583dd3f85bb62d0b74f85`,
  active DAG-008, DEL-08-01 live files, DEC-028/DEC-061 and the current
  report-package/redaction contracts.

## Objective

Map the existing report-package producer, its schema/manifest/redaction gates,
callable Rust/Tauri/TypeScript seams, tests, diagnostics, professional-boundary
notices, and precise inputs/outputs needed for a desktop caller. Distinguish
observed facts, supported inference, unknowns, and scope risks.

## Scope and exclusions

Read only the relevant report generator/renderer/PDF/package/redaction source,
manifests, fixtures, schemas, tests, DEL-08-01 records, and registered workflow
profile. Do not inspect or change runner implementations except to verify the
explicit no-runner-change boundary. Do not implement, format, build, test, or
change product/state files. Do not delegate.

## Allowed tools and write target

Read-only file/search/Git commands and registered deterministic discovery tools
only. The sole write target is this instance's `RETURN.md`; use the patch tool,
not shell redirection.

## Required return

Component/dependency map; exact producer entry points and payload types; schema,
redaction, manifest, diagnostics, provenance/units/claim-state/boundary
invariants; likely affected paths; focused and registered checks; risks,
unknowns, and recommended write fence. Cite file paths and line numbers.

