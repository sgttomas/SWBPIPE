# N1 return — existing report-package producer reconnaissance

RUN_STATUS: SUCCESS

ControlSurface: FILE

TaskProfile: NONE

TaskSkill: software-repository-reconnaissance

ScopePath: `/Users/ryan/ai-env/projects/chirality/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260722-DEL0801-REPORT-PACKAGE-R16/instances/N1`

ResolvedSkillPath: `/Users/ryan/ai-env/projects/chirality/skills/software-repository-reconnaissance`

ResolvedSkillVersion: 1

ResolvedTaskProfileRequirement: NONE

CompanionFiles: `BRIEF_SCHEMA.md (found), TOOL_POLICY.md (found), QA_CHECKS.md (found)`

AllowedTools: `python3 tools/software_workflow/discover_repository.py:*`, `python3 tools/software_workflow/select_affected_checks.py:*`, and brief-authorized read-only file/search/Git commands

RuntimeOverrides: none

WriteAuthorization: EXPLICIT_BRIEF_TEXT — only this `RETURN.md`

## Executive result

The complete in-memory producer already exists at `core/reporting/report_package`: `assemble_report_package_container(&ReportPackageContainerInput) -> ReportPackageContainerOutcome`. It deterministically emits a `.opsproj` ZIP containing package manifest, canonical HTML, deterministic PDF, audit manifest, one or more result envelopes, and one or more DEL-08-06 state/comparison/handoff records. It intentionally writes no files, assembles bytes even when blocked, and requires its caller to refuse persistence when `export_blocked` is true (`core/reporting/report_package/src/lib.rs:1-38,98-164,170-195,323-355`).

The desktop is not bound to that producer. Its Tauri crate depends only on `report_renderer`, its only report command returns HTML plus a derived print view, and the File menus contain only project create/open/list/save actions (`apps/desktop/src-tauri/Cargo.toml:15-22`; `apps/desktop/src-tauri/src/lib.rs:1097-1110,3317-3348`; `apps/desktop/src/App.tsx:867-889,1500-1512`). No report-package command, package-specific TypeScript service, destination chooser, or atomic `.opsproj` writer exists.

## Observed component and dependency map

1. Desktop session adapter: `buildRenderableReportInput` maps `PreviewModel`, `MechanicsResult`, `AnalysisRunEnvelope`, and optional persistence summary into the renderer input, preserving explicit units, provenance, diagnostics, statuses, and TBDs (`apps/desktop/src/features/report/renderableReportInput.ts:260-283,285-364,366-455`).
2. Redaction gate: `controlReportRendererInput` routes the renderer input through `DREP-IPC-003`, `public_report`, with lossless materialization required (`apps/desktop/src/features/report/reportRedactionProjector.ts:13-18`). The panel refuses IPC when this result is blocked (`apps/desktop/src/features/report/RenderedReportPanel.tsx:54-75`).
3. Current desktop IPC: `renderCalculationReport` accepts only a controlled `DREP-IPC-003` result, invokes `render_calculation_report`, and returns a renderer outcome (`apps/desktop/src/services/reportRenderService.ts:38-71,74-88`).
4. Renderer: `RenderableReportInput` consists of `report_title`, `CalculationReport`, `ReportSections`, and caller-formatted `ResultRow[]`; `render_calculation_report` validates the report and sections, runs pre/post protected-content lint, emits deterministic HTML, and returns diagnostics/findings plus `export_blocked` (`core/reporting/report_renderer/src/lib.rs:61-115,397-405,844-965`).
5. PDF emitter: `assemble_full_report_package` renders HTML and deterministic PDF from the same input/model and returns both hashes and blocking reasons (`core/reporting/pdf_emitter/src/lib.rs:82-119,125-210`).
6. Package producer: `assemble_report_package_container` consumes the renderer input plus audit/result/section records, validates each upstream object, materializes members in fixed order, builds the manifest, and creates a stored deterministic ZIP (`core/reporting/report_package/src/lib.rs:98-121,170-355,624-700`).
7. Upstream Rust dependencies are `pdf_emitter`, `report_renderer`, `audit_manifest`, `result_export`, and `canonical_json`; test-only assembly also uses `report_generator` and `report_sections` (`core/reporting/report_package/Cargo.toml:11-23`). There is no runner dependency.

## Exact producer API and payload types

### Input

`ReportPackageContainerInput<'a>` requires (`core/reporting/report_package/src/lib.rs:98-121`):

- `package_id: String` — deterministic caller ID and sanitized filename stem;
- `export_profile_id: String`;
- `source_model_ref: PackageRef { ref_type, ref_id }`;
- `source_basis_refs: Vec<PackageRef>`;
- `report: &report_renderer::RenderableReportInput`;
- `audit_manifest: &audit_manifest::AuditManifest`;
- `result_envelopes: &[result_export::ResultEnvelope]`;
- `state_comparison_handoff_records: &[serde_json::Value]`.

The current fixture demonstrates the concrete construction and expected live categories: a renderable report, audit manifest with model/input hashes and solver/unit identity, result envelope with explicit unit/dimension/provenance/reproducibility/status data, and a `DEL-08-06` section-set record (`core/reporting/report_package/tests/container.rs:84-131,142-225,228-256`).

### Output

`ReportPackageContainerOutcome` returns (`core/reporting/report_package/src/lib.rs:142-164`):

- `container_file_name`, always sanitized stem plus `.opsproj`;
- `document_kind`, always `OpenPipeStress Project Package`;
- `package_identity_sha256_hex`, the canonical manifest-member hash;
- `container_bytes` and its separately recorded `container_sha256_hex`;
- all `ContainerMember` records, including role/name/media type/hash basis/hash/length/identity flag/bytes;
- `export_blocked` and `blocking_reasons`;
- the underlying HTML/PDF `FullReportPackageOutcome`.

### Desktop wire gap

`ReportPackageContainerInput` is a borrowed Rust assembly type and does not implement `Deserialize`; `AuditManifest` and `ResultEnvelope` likewise are not currently deserialize-ready wire DTOs. The desktop currently owns only the renderer dependency and renderer JSON command (`apps/desktop/src-tauri/Cargo.toml:15-22`; `apps/desktop/src-tauri/src/lib.rs:1097-1110`). A package command therefore needs an explicit owned desktop request DTO plus validated conversion into these producer types, or a deliberately bounded serde-enablement change. This is an implementation seam, not authority to weaken the core types.

## Manifest, schema, and artifact invariants

- Member order is fixed: manifest, HTML, PDF, audit manifest, result envelopes, then DEL-08-06 records (`core/reporting/report_package/src/lib.rs:275-307`; independently asserted at `core/reporting/report_package/tests/container.rs:410-447`).
- Canonical JSON members are exact JCS bytes; HTML/PDF use exact-byte hashes (`core/reporting/report_package/src/lib.rs:362-390`).
- The manifest records DEC-028/057/061 basis, source references, per-member hashes, field statuses, diagnostics, boundary notes, and the professional-boundary booleans (`core/reporting/report_package/src/lib.rs:530-615`). It lists non-manifest members; the hash of the manifest's own exact canonical bytes is returned as package identity (`core/reporting/report_package/src/lib.rs:565-577`; tests at `core/reporting/report_package/tests/container.rs:449-497`).
- ZIP members are stored, not compressed, in fixed order with fixed ZIP-epoch timestamps, no extra fields/comments, and deterministic CRCs (`core/reporting/report_package/src/lib.rs:624-700`; tests at `core/reporting/report_package/tests/container.rs:544-568`).
- Duplicate sanitized member names, missing result members, missing DEL-08-06 records, and upstream blocking findings block export (`core/reporting/report_package/src/lib.rs:209-273,309-329`).
- A blocked package still contains bytes for diagnosis. Saving those bytes is forbidden by the caller contract (`core/reporting/report_package/src/lib.rs:26-32`; tests at `core/reporting/report_package/tests/container.rs:606-668`).
- The report JSON schema governs calculation-report syntax only and explicitly excludes runtime, lint, redaction/export-control, and professional-claim implementation (`schemas/report_generator.schema.yaml:1-15`). No dedicated JSON schema for the `.opsproj` package manifest was found; its present executable contract is the Rust producer plus container tests.

## Redaction, diagnostics, provenance, units, claims, and boundary invariants

- Redaction is not performed by `report_package`; its direct gates are renderer/report-section validation and lint, audit-manifest validation, result-envelope validation, and DEL-08-06 record checks (`core/reporting/report_package/src/lib.rs:192-273`). The desktop must pass a controlled payload and preserve the redaction decisions/findings/summary before assembly or exposure.
- Current renderer control is specifically `public_report` and lossless (`apps/desktop/src/features/report/reportRedactionProjector.ts:13-18`). Actual user-local report input is private/pending, and the existing panel test proves it is blocked before IPC, preview, save, or print (`apps/desktop/src/features/report/renderedReport.test.tsx:424-453`). A package-save route cannot silently relabel that input public.
- The redaction contract requires explicit local/private intent to retain private values in `local_private`; otherwise public/shared/downstream contexts redact or block according to metadata (`docs/security/redaction_export_controls.md:57-65`). The current `ControlledExportLink` has no report-package binding; it binds canonical HTML save to `DREP-HTML-SAVE-005/public_report` and operates on data URLs, not atomic filesystem writes (`apps/desktop/src/features/redaction-controls/ControlledExportLink.tsx:14-32,53-94`).
- Report validation preserves the six-state vocabulary, requires `HUMAN_REVIEW_REQUIRED`, complete provenance, explicit unresolved TBD review, safe rule-pack references, and no compliance/certification/sealing/approval/authentication claims (`core/reporting/report_generator/src/lib.rs:484-500,526-648`).
- The renderer maps unknown review status to pending and carries provenance privacy/redistribution into lint (`core/reporting/report_renderer/src/lib.rs:280-305`). It reports structured validation diagnostics and pre/post findings (`core/reporting/report_renderer/src/lib.rs:82-115,938-964`).
- The existing desktop adapter preserves entered model units, explicit result units, and `conversion_performed: false`; display strings become deterministic hashed input (`apps/desktop/src/features/report/renderableReportInput.ts:53-70`; `core/reporting/report_renderer/src/lib.rs:61-80`).
- The package manifest states artifact-not-issuance, competent human review, no generated timestamps/IDs, and no professional claims (`core/reporting/report_package/src/lib.rs:598-613`). These notices must remain byte-visible and must not be replaced by a UI-only disclaimer.
- Governing invariants include protected-content/provenance controls, explicit missing data, no software authority claims, unit-aware exports, safe rule-pack handling, report disclosures, and private-data controls (`docs/CONTRACT.md:23-29,33,36-42`).

## Desktop integration seams and material blockers

### Observed seams

- Add the report-package crate as a desktop Rust dependency beside the renderer (`apps/desktop/src-tauri/Cargo.toml:15-22`).
- Add a Tauri command beside `render_calculation_report`, and register it in the existing handler list (`apps/desktop/src-tauri/src/lib.rs:1097-1110,3317-3348`).
- Add a TypeScript package service beside `reportRenderService.ts`; avoid returning `Vec<u8>` as a large JSON number array if the Rust caller can assemble, gate, and atomically persist in one command.
- Extend `RenderedReportPanel` or a tightly scoped sibling in the Report workspace; this panel already owns session inputs, redaction evidence, gate display, and report actions (`apps/desktop/src/features/report/RenderedReportPanel.tsx:24-43,54-79,121-193`).
- Add the same File command ID to both the DOM File menu and native Tauri File submenu. Native events already converge on the single `runMenuCommand` sink (`apps/desktop/src/App.tsx:867-944,946-955`; `apps/desktop/src-tauri/src/lib.rs:3250-3261,3310-3316`).

### Material blockers/unknowns to resolve in the frozen brief

1. **Input ownership:** the frontend builds a renderer input, but the existing result-export packet builder is private to `ResultExportPanel` and its JSON shape is not the Rust `ResultEnvelope` wire shape (`apps/desktop/src/features/result-export/ResultExportPanel.tsx:94-184`). The integration owner must define one canonical owned request mapping, not copy divergent ad-hoc packet builders.
2. **Audit-manifest source:** no existing desktop command produces the exact Rust `AuditManifest` required by the package. Mapping must preserve actual model/input hashes, solver build, unit-system ref, assets/rule packs, and boundary flags; invented placeholders or silent defaults would violate the package gate.
3. **DEL-08-06 member source:** the producer accepts generic JSON and validates only deliverable ID, nonempty section-set ID, and blocking diagnostics. The caller needs a frozen, tested mapping from current state/comparison/handoff records rather than an empty synthetic record (`core/reporting/report_package/src/lib.rs:231-273`).
4. **Redaction evidence carrier:** the producer input/manifest has no field/member for redaction decisions/findings/summary. The accepted implementation requirement to preserve redaction output therefore needs an explicit manifest/member design, or a citable reason why the controlled payload and returned evidence remain outside the package. Do not drop the evidence silently.
5. **Local-private intent:** current report rendering uses `public_report` and normally blocks user-local data. If `.opsproj` is a local-private save, the UI must collect explicit local-private intent and route it through the existing contract; if it is public/shared, destructive redaction plus lossless-package expectations can withhold materialization. This is a product-policy choice that must be frozen before coding.
6. **Destination selection:** neither Cargo manifest contains a Tauri dialog plugin, and no report-package destination chooser exists. A new dependency or OS dialog mechanism is not implied by this reconnaissance. The brief must define caller-supplied destination semantics or authorize a chooser explicitly.
7. **Atomic replacement details:** D-09 records write-temp/flush/atomic-rename as the implementation discipline (`execution/_Coordination/_DECISIONS/D-09_native_package_container.md:70,120`). The command should create the temp in the destination directory, write all bytes, flush/sync, rename only after all gates pass, remove only its own failed temp, and never delete/replace the prior file before successful rename. Cross-platform overwrite behavior and destination collision policy require tests.
8. **No runner expansion:** `report_package` has no runner dependency and the requested producer/desktop tranche can be completed without reading or changing runner implementation. Preserve this boundary; do not add or alter a runner verb.

## Recommended implementation write fence

One serialized implementation owner should own the overlapping product files. Recommended maximum product fence:

- `core/reporting/report_package/src/lib.rs` and `core/reporting/report_package/tests/container.rs` only if the frozen design adds redaction-evidence/manifest fields; otherwise leave the proven producer unchanged;
- `apps/desktop/src-tauri/Cargo.toml`, its lockfile if dependency resolution changes, and `apps/desktop/src-tauri/src/lib.rs`;
- one new package service under `apps/desktop/src/services/`;
- `apps/desktop/src/features/report/RenderedReportPanel.tsx`, `renderableReportInput.ts` only if canonical package inputs require an extracted shared builder, and `renderedReport.test.tsx`;
- `apps/desktop/src/App.tsx` and focused `App.test.tsx` menu coverage;
- redaction projector/control tests only for an explicitly frozen new route ID/context;
- tranche-local run evidence and later DEL-08-01-only closeout surfaces under the manager's separate authority.

Avoid touching `core/runner/**`, unrelated export panels, public schemas, root governance, release surfaces, or lifecycle records. If serde support in `audit_manifest`/`result_export` or a new dialog dependency is selected, add those paths explicitly to the candidate brief before implementation rather than treating them as incidental compilation edits.

## Focused and registered checks (discovered, not executed)

Focused checks:

- `cargo test --manifest-path core/reporting/report_package/Cargo.toml`
- `cargo test --manifest-path core/reporting/pdf_emitter/Cargo.toml`
- `cargo test --manifest-path core/reporting/report_renderer/Cargo.toml`
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
- focused Vitest for `apps/desktop/src/features/report/renderedReport.test.tsx` and relevant `App.test.tsx` menu cases
- focused H4/Playwright evidence for native/desktop report-package request, blocked non-exposure, explicit intent, successful atomic save, and failure preservation of the prior file
- `git diff --check` and an explicit write-fence/status audit

`select_affected_checks.py` selected the registered union for the likely fence:

- `desktop-test`
- `desktop-build`
- `piping-pytest`
- `evidence-sweep`
- `harness-self-check`

The profile commands are defined at `software-workflow.json:5-17`; this run discovered them but did not execute them.

## Recommended Agent 2 boundaries

- Implementation owner: one serialized writer for producer-to-Tauri-to-menu/save integration, because Cargo, Tauri command registration, report UI, route evidence, and tests are one overlapping contract.
- Fresh verifier: read-only/diff-first verification of gate ordering, redaction evidence, blocked non-persistence, byte identity/hash invariants, atomic failure recovery, menu reachability, no runner changes, and the complete check union.

## Epistemic classification

- OBSERVED: all API, manifest, gate, menu, dependency, test, and missing-binding facts above are cited to the frozen SHA `8698b0338ac82556fee583dd3f85bb62d0b74f85`.
- SUPPORTED INFERENCE: keeping package bytes inside the Rust command avoids inefficient JSON-array IPC and makes the Tauri layer the caller that can honor `export_blocked` immediately before atomic persistence.
- UNKNOWN: exact destination-picker mechanism, collision/overwrite UX, canonical desktop mappings for audit/result/DEL-08-06 inputs, and whether redaction evidence becomes a package member or manifest field.
- PROPOSAL: freeze those four unknowns before implementation and keep one serialized owner over the integration fence.

## Tools Used

- `python3 tools/software_workflow/discover_repository.py`
- `python3 tools/software_workflow/select_affected_checks.py`
- brief-authorized read-only `git`, `rg`, `sed`, and `nl`

## Tool Policy Compliance

PASS — deterministic discovery ran before targeted exploration; affected-check selection used the accepted project profile and declared likely paths; no build, test, generator, installer, network, formatting, or product-write command ran.

## Outputs Produced

- This bounded producer reconnaissance with cited component map, exact API/types, invariants, seams, blockers, checks, and recommended write fence.

## Missing

- none for reconnaissance; the four product-design unknowns are explicitly preserved above.

## Needs Human Ruling

- none at N1 level; the WORKING_ITEMS manager should freeze the product choices in the candidate brief and escalate only if they expand authority (for example, a new dialog dependency or changed package-manifest meaning).

## Dependency Notes

- No cycle identified in this bounded producer/desktop map.
- Package assembly depends on accepted DEL-08-02, DEL-08-04, and DEL-08-06 contracts; the implementation must consume them without changing their meaning.

## Applied Changes

- Added only this authorized `N1/RETURN.md`; no product, state, runner, or lifecycle file changed.

## Proposed Changes

- As bounded above; no proposal is adopted by this return.
