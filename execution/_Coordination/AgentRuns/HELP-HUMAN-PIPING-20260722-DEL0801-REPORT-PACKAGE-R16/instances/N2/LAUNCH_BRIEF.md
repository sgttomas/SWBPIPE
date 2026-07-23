# N2 launch brief — desktop menu and native atomic-save reconnaissance

- RequestedBy: `W-PKG08` WORKING_ITEMS.
- RunID: `HELP-HUMAN-PIPING-20260722-DEL0801-REPORT-PACKAGE-R16`.
- ParentInstanceID: `W-PKG08`.
- ChildInstanceID: `N2`.
- PackageID / DeliverableID: `PKG-08` / `DEL-08-01`.
- TaskSkill: `software-repository-reconnaissance` (`ApplyEdits: false`).
- AcceptedBasis: Git SHA `8698b0338ac82556fee583dd3f85bb62d0b74f85`,
  active DAG-008, current desktop/Tauri report save surfaces and tests.

## Objective

Map the existing desktop report UI/menu, Tauri command/service boundary, file
picker/native save APIs and capabilities, current HTML/JSON save flows, and the
minimum route for caller-owned atomic `.opsproj` save using same-directory temp
file, complete write/sync as supported, atomic rename/replacement, cleanup, and
structured failure diagnostics.

## Scope and exclusions

Read only the relevant `apps/desktop` report feature, menu/App wiring, Tauri
Rust commands/capabilities/config, tests and H4 evidence surfaces, plus the
workflow profile. Do not inspect or change headless runner behavior. Do not
implement, format, build, test, or change product/state files. Do not delegate.

## Allowed tools and write target

Read-only file/search/Git commands and registered deterministic discovery tools
only. The sole write target is this instance's `RETURN.md`; use the patch tool,
not shell redirection.

## Required return

Exact UI-to-native route map; current save semantics and gaps; atomic-save
platform considerations; capability/security constraints; likely affected
paths; source, Vitest, Rust, build, and H4 source+dist checks; negative/failure
tests; risks, unknowns, and recommended write fence. Cite file paths and line
numbers.

