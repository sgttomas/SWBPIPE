# N2 return — desktop menu and native atomic-save reconnaissance

RUN_STATUS: SUCCESS

ControlSurface: FILE

TaskProfile: NONE

TaskSkill: `software-repository-reconnaissance`

ScopePath: `/Users/ryan/ai-env/projects/chirality/projects/chirality-piping/apps/desktop`

ResolvedSkillPath: `/Users/ryan/ai-env/projects/chirality/skills/software-repository-reconnaissance`

ResolvedSkillVersion: `1`

ResolvedTaskProfileRequirement: `NONE`

CompanionFiles: `BRIEF_SCHEMA.md (found), TOOL_POLICY.md (found), QA_CHECKS.md (found)`

AllowedTools: `python3 tools/software_workflow/discover_repository.py:*`, `python3 tools/software_workflow/select_affected_checks.py:*`; the sealed brief additionally permitted read-only file/search/Git commands.

RuntimeOverrides: none

ToolsUsed:

- `python3 tools/software_workflow/discover_repository.py`
- Read-only `git`, `rg`, `find`, `sed`, and `nl` inspection commands under the sealed brief.

ToolPolicyCompliance: PASS — repository discovery ran before targeted source inspection; no build, test, generator, installer, formatter, or network command ran. `select_affected_checks.py` was not run because no accepted changed-path set exists yet.

WriteAuthorization: EXPLICIT_BRIEF_TEXT — only this managed `RETURN.md`.

## Executive result

The desktop has a reusable two-menu/one-sink command architecture, but no report-package save command. The in-DOM and native menus currently expose only report *navigation* (`View > Report`). The two report save surfaces are controlled browser `data:` downloads: sanitized preview JSON and canonical HTML. The Tauri backend renders HTML, but it does not receive a destination path, open a native picker, package `.opsproj`, or write any report artifact.

The minimum coherent binding is a new shared command id such as `file.export-report-package`, emitted by both menu implementations and handled by the existing React command sink. The handler must obtain only the accepted, redaction-cleared report-package payload, ask the user for a `.opsproj` destination in the Tauri runtime, and invoke one desktop-owned Rust atomic-save command. The report-package producer must remain payload/schema owner; the desktop command is solely the filesystem integration owner. Browser mode must return an explicit unavailable/cancelled route and must not substitute an anchor download.

## Exact UI-to-native route map

### Existing route

1. `App.tsx` defines `view.section.report`, but no file/export report command (`apps/desktop/src/App.tsx:147-184`).
2. The in-DOM `View` menu derives its report item from `WORKSPACE_SECTIONS` (`apps/desktop/src/App.tsx:223-227`, `1523-1535`).
3. The native Tauri `View` menu separately declares the same id `view.section.report` (`apps/desktop/src-tauri/src/lib.rs:3263-3276`).
4. Native clicks emit `native-menu-command` (`apps/desktop/src-tauri/src/lib.rs:3310-3316`); `nativeMenu.ts` listens and passes only the string payload (`apps/desktop/src/services/nativeMenu.ts:10-31`).
5. Both menu surfaces reach `runMenuCommand`; its default branch toggles `view.section.*` (`apps/desktop/src/App.tsx:867-944`).
6. The Report dock mounts `RenderedReportPanel`, `ReportPanel`, and `ReportLintPanel` (`apps/desktop/src/App.tsx:1294-1319`).

### Existing report generation and save surfaces

- Canonical HTML: `RenderedReportPanel` builds input, applies `controlReportRendererInput`, and calls `renderCalculationReport` only when not blocked (`apps/desktop/src/features/report/RenderedReportPanel.tsx:54-78`). The service invokes `render_calculation_report` only in Tauri (`apps/desktop/src/services/reportRenderService.ts:43-71`). Rust delegates to `open_pipe_stress_report_renderer`, adds a derived print view, and returns JSON (`apps/desktop/src-tauri/src/lib.rs:1097-1110`). If unblocked, the panel exposes HTML through a `data:text/html` download anchor (`apps/desktop/src/features/report/RenderedReportPanel.tsx:133-180`).
- Preview JSON: `ReportPanel` constructs a preview packet, redaction-controls it, and exposes it through a `data:application/json` anchor (`apps/desktop/src/features/report/ReportPanel.tsx:67-118`, `631-671`, `1081-1086`). Its declared scope is `local_browser_download_preview`, not `.opsproj` package persistence (`ReportPanel.tsx:668-672`).
- Both anchors pass through `ControlledExportLink`: JSON uses route `DREP-JSON-002`; HTML uses `DREP-HTML-SAVE-005` (`apps/desktop/src/features/redaction-controls/ControlledExportLink.tsx:14-18`). That component decodes, controls, and re-encodes a browser data URL (`ControlledExportLink.tsx:59-95`, `129-161`).

### Proposed minimum route

`native File menu or in-DOM File menu` -> `file.export-report-package` -> existing `native-menu-command` bridge -> `runMenuCommand` -> one new TypeScript report-package save service -> native save-picker route -> `invoke("save_report_package_atomically", { request })` -> Rust validation and atomic writer -> structured receipt/diagnostic -> user-visible status.

Add the command to both menu declarations; otherwise browser/H4 and packaged-native behavior diverge. Because the native menu has no React-driven enabled-state synchronization today, the sink itself must always recheck readiness/redaction and fail closed even if a native item is clickable.

## Current save semantics and gaps

Observed facts:

- The Tauri invoke handler registers report rendering but no package or filesystem-save command (`apps/desktop/src-tauri/src/lib.rs:3317-3348`).
- Desktop JS dependencies contain `@tauri-apps/api` but no dialog or filesystem plugin (`apps/desktop/package.json:16-37`). Rust dependencies contain core `tauri`, renderer, SQLite, and project crates but no Tauri dialog/filesystem plugin and no report-package crate (`apps/desktop/src-tauri/Cargo.toml:15-26`).
- No `src-tauri/capabilities/` or `src-tauri/permissions/` files exist. `tauri.conf.json` declares window/build/bundle settings only (`apps/desktop/src-tauri/tauri.conf.json:1-30`).
- Existing `Save Local Project` persists an internal SQLite snapshot, not a user-selected file (`apps/desktop/src/App.tsx:875-889`; `apps/desktop/src/services/projectService.ts:571-611`; `apps/desktop/src-tauri/src/lib.rs:1832-1868`). It is not an atomic `.opsproj` precedent.
- Current report tests prove redaction/render routing and suppression, not filesystem persistence (`apps/desktop/src/features/report/renderedReport.test.tsx:196-236`, `409-522`; `apps/desktop/src-tauri/src/lib.rs:3430-3533`).

Gaps:

- No `.opsproj` destination selection or extension enforcement.
- No desktop dependency on the accepted report-package producer.
- No caller-owned byte serialization boundary or expected digest/size receipt.
- No same-directory temporary-file/write/sync/rename/cleanup implementation.
- No structured cancellation, stage-specific I/O diagnostics, overwrite semantics, or user-visible save result.
- No capability declaration for a plugin-based picker. If the implementation adds a Tauri plugin, its least-privilege capability must be explicit; do not grant broad frontend filesystem access when the custom Rust command can own the write.
- No current H4 coverage of native filesystem behavior; browser Playwright cannot prove the packaged Tauri picker or rename operation.

## Atomic-save contract and platform considerations

Recommended Rust boundary:

- Input: accepted package bytes (or one exact serialized package string), destination selected by explicit user action, and optional expected payload SHA-256 supplied by the producer contract. Do not reconstruct or reinterpret package content in the writer.
- Validate before opening: nonempty payload, `.opsproj` suffix, existing parent directory, target not a directory, overwrite intent, and package/manifest preconditions already reported as satisfied. Reject any redaction-blocked or incomplete payload before picker/invoke.
- Create an unpredictable temp file with `create_new(true)` in the destination's parent directory. Same-directory placement is required for same-filesystem rename semantics.
- `write_all`, flush, then `sync_all` (or the explicitly chosen supported sync primitive); close the handle before rename where platform behavior requires it.
- Rename/replace atomically only under a documented platform policy. On Unix/macOS, same-filesystem rename can atomically replace an existing file; Windows replacement/open-handle behavior differs. Do not claim cross-platform atomic replacement until separately implemented/tested. Current product metadata calls this a macOS technical preview (`apps/desktop/src-tauri/Cargo.toml:2-5`), but the diagnostic contract should still name unsupported replacement behavior rather than silently delete-then-rename.
- After rename, sync the parent directory where supported to make the directory entry durable. Report an honest durability state if directory sync is unsupported or fails after the rename.
- On every pre-rename failure, attempt temp cleanup without touching the existing destination. Preserve both the primary failure and any cleanup failure. Never implement replacement by deleting the destination first.
- Return a typed receipt (`saved|cancelled`) or typed error with stable code, stage (`validate|create_temp|write|file_sync|replace|directory_sync|cleanup`), OS error kind/raw code, bytes/digest, replacement flag, and durability status. Avoid leaking full private paths into telemetry or public report content.
- Explicitly decide and test symlink policy. A string-path check alone is race-prone; at minimum reject a symlink destination/parent when that is the accepted policy and surface the limitation.

Unknown requiring manager/implementation resolution:

- Whether the adopted producer returns bytes, a JSON value, or a container receipt; N2 did not inspect or alter the producer package. Freeze this at fan-in before implementation.
- Picker mechanism: adding `@tauri-apps/plugin-dialog`/Rust plugin plus a narrow capability is the conventional separation, but the dependency/capability choice is not present in the accepted tree and must be frozen before edits.
- Accepted overwrite policy and whether macOS-only atomic replacement is sufficient for this tranche.

## Tests and checks

### Focused source/Vitest

- Menu parity: both File menus expose the same command id/label; the browser item dispatches the shared sink.
- Readiness: absent result/package, incomplete manifest, redaction block, renderer/export block, or busy state never opens the picker and never invokes the writer.
- Runtime routing: browser returns `REPORT-PACKAGE-SAVE-DESKTOP-ONLY`; picker cancellation returns `cancelled`; Tauri success forwards exact bytes/path and renders receipt; structured failures render stable stage/code without a false success.
- Reentrancy: double click cannot launch two writes; retry after failure is allowed.
- Preserve existing JSON/HTML redaction routes and current no-bypass tests.

Likely Vitest files: `apps/desktop/src/App.test.tsx`, a new `apps/desktop/src/services/reportPackageSaveService.test.ts`, and only if the action is placed in an existing panel, `apps/desktop/src/features/report/renderedReport.test.tsx`.

### Rust

- New destination success; exact bytes/digest/size; temp and final are in the same parent.
- Existing-target atomic replacement preserves old bytes on every pre-rename failure.
- Invalid extension, missing/non-directory parent, destination directory, symlink-policy refusal, create collision, short/write failure, file-sync failure, rename/replace failure, directory-sync failure, and cleanup failure each produce the expected structured stage/code.
- Successful and failed runs leave no temp file; cancellation performs no write.
- Command serialization tests prove stable TypeScript-facing receipt/error fields.
- Retain existing renderer unblocked/invalid-input tests (`apps/desktop/src-tauri/src/lib.rs:3430-3533`).

Suggested focused command (discovered, not run): `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml report_package_save` (final test filter depends on implementation names).

### Registered and H4

- `apps/desktop/**` selects registered `desktop-test` and `desktop-build`; `harness-self-check` is always selected (`software-workflow.json:5-16`). Commands are `npm run test:desktop`, `npm run build:desktop`, and the registered harness self-check.
- Source H4: `npm run test:e2e:desktop` (`package.json:8-14`). Existing source smoke reaches Report and asserts the JSON data URL (`apps/desktop/e2e/r2-smoke.spec.ts:537-559`); extend it to exercise the File command's browser fail-closed/cancelled route without inventing a browser download.
- Dist H4: `npm run test:e2e:dist:desktop` (`package.json:8-14`). Current dist config runs only `*-dist.spec.ts` (`apps/desktop/playwright.dist.config.ts:33-53`), and the sole dist spec covers WASM/New Blank, not reports (`apps/desktop/e2e/wasm-engine-dist.spec.ts:1-26`). Add a bounded `*-dist.spec.ts` menu/route test so production-dist binding is not inferred from source H4.
- Limitation: both Playwright lanes are browser servers, so neither proves the real native picker, file sync, or rename. Rust command tests plus a packaged-macOS manual/H4-native evidence step are required for that claim.

## Recommended serialized write fence

Required/likely product ownership:

- `apps/desktop/src/App.tsx`
- `apps/desktop/src/App.test.tsx`
- `apps/desktop/src/services/reportPackageSaveService.ts` (new)
- `apps/desktop/src/services/reportPackageSaveService.test.ts` (new)
- `apps/desktop/src-tauri/src/lib.rs` (or a new module wired only from it)
- `apps/desktop/src-tauri/Cargo.toml`
- `apps/desktop/src-tauri/Cargo.lock`
- `apps/desktop/package.json` and repository lockfile only if a JS picker plugin is adopted
- `apps/desktop/src-tauri/capabilities/<bounded-capability>.json` only if required by the adopted picker design
- `apps/desktop/e2e/r2-smoke.spec.ts`
- one new `apps/desktop/e2e/*-dist.spec.ts`

Conditional only: `RenderedReportPanel.tsx`/test if the menu handler must consume its state rather than an App-owned accepted package state; `nativeMenu.ts` should require no change because it already forwards arbitrary command strings.

Explicit exclusion: no `core/runner/headless/**`, no runner verb or command, no unrelated report JSON/HTML semantics, and no broad frontend filesystem permission.

## Epistemic classification

- OBSERVED: route, current downloads, dependency/config/capability absence, registered checks, and existing test coverage cited above.
- SUPPORTED INFERENCE: a shared File-menu command plus one desktop-owned atomic writer is the minimum integration that preserves the current one-sink architecture.
- UNKNOWN: producer payload representation, adopted picker dependency, overwrite/platform policy, and packaged-native H4 mechanism.
- PROPOSED: command names, typed diagnostic shape, atomic algorithm, tests, and write fence.

Outputs:

- Exact desktop UI/native route map, save/capability gaps, atomic-save constraints, checks, negative tests, risks, unknowns, and recommended write fence in this return.

MISSING:

- None for the bounded reconnaissance. Producer payload details intentionally remain for manager fan-in with the producer reconnaissance.

NEEDS_HUMAN_RULING:

- None at N2. The manager should freeze picker/capability and overwrite/platform choices in the candidate brief before implementation.

DEPENDENCY_NOTES:

- Implementation depends on the accepted report-package producer payload contract and redaction/manifest readiness result. Desktop persistence must not become an alternate producer or bypass.

AppliedChanges:

- Added only this `RETURN.md`; product and governed state remained read-only.
