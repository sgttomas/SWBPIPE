# R1 Method Notes — Implementation-Surfaces Index

**Run:** DELIVERABLE_CONCORDANCE_2026-07-11_1305 · R1 (plan §8 R1, read-only inventory)
**Source state:** frozen worktree `551f84ef6be656f1603ce0acfa5e3935aa9683c7` at
`/Users/ryan/ai-env/projects/chirality/.claude-worktrees/piping-frozen-551f84ef6`,
project root `projects/chirality-piping`. All reads from the frozen tree; both
output files written only into this run folder in the working tree. Frozen-tree
`git status --porcelain` verified clean after the run (recorded below by the
executor). No builds or re-executions were performed (enumeration and grep
only), so the R1_CONVENTIONS.md addendum-9 side-effect-free redirections were
not needed.

## Enumeration commands (all against the frozen tree)

- Crates: `find . -name Cargo.toml -not -path '*/target/*'` (37 manifests: 33
  under `core/**`, 3 under `validation/benchmarks/**`, 1 `apps/desktop/src-tauri`);
  name/description read from each manifest; `[[bin]]` targets from manifests
  plus `src/bin/*` listings.
- Python modules: `find core -name '*.py' -not -path '*__pycache__*'`; one
  surface per module package (engine/service/controls grain); purpose = first
  module docstring line.
- Schemas: `ls schemas/` (38 files) plus other public schema locations found by
  `find . -name '*schema*'`: `validation/witness/schemas/…` (1) and
  `tools/release/*_schema.json` (2). `NamedInSPEC` = literal filename match in
  `docs/SPEC.md`.
- Desktop app: `ls apps/desktop/src/features` (44 feature dirs + 1 shared
  helper file), `ls apps/desktop/src/services` (12 service modules + wasm
  engine loader), plus app shell (`App.tsx`) and the Tauri shell crate.
- Fixtures: `ls fixtures/` (24 families, family grain with file counts).
- Tools: `tools/**` per `tools/REGISTRY.md` plus directory listing (12 scripts
  + the registry itself).
- Evidence generators: crate `examples/` targets that emit committed evidence
  (`product_physics/examples/preview_result.rs`,
  `performance_harness/examples/sparse_default_promotion_observation.rs`) and
  `validation/witness` tools.
- Templates/packaging/API: `.github/ISSUE_TEMPLATE`, `docs/*TEMPLATE*`,
  `governance/*TEMPLATE*`, root and desktop `package.json`, wasm build script,
  `tauri.conf.json`, `vite.config.ts`, `requirements-dev.txt`,
  `provenance/build-artifacts` (pinned-lockfile family),
  `api/api_boundary_contract.yaml`.
- Attribution: for each surface, `rg -l -F <key>` over `execution/**` for each
  search key (path form, crate/module name, component name, and for five
  panels a verified DOM `data-testid` prefix). A DEL attribution requires the
  matching file to live under a `DEL-XX-YY_*` directory (four-document kit,
  `_STATUS.md`/`MEMORY.md`/registers, or deliverable-local `_run_records`).
  Matches only in package-level `_run_records`, `_Aggregation`,
  `_Coordination`, `_Decomposition`, etc. do not attribute; such rows record
  `NONE_FOUND` with a pointer to the nearest non-DEL match.

## Grain decisions

- Rust crates at crate grain (one row per `Cargo.toml`); `[[bin]]` targets get
  their own `BIN_TARGET` rows (3). The Tauri shell crate (which also has a bin)
  is a single `APP_BINDING` row.
- Python engines at module-package grain (one row per bounded module, not per
  file); single-file modules rowed at file grain.
- Desktop app at panel/feature-directory grain (44 dirs + shared
  `exportUnitDisclosure.ts`), plus service-module grain for
  `src/services` (12), one shell row, and two `APP_BINDING` rows (wasm engine
  loader; Tauri shell incl. `model_document_migration.rs`).
- Fixtures at family (directory) grain, 24 rows.
- Excluded per instruction: build outputs, `target/`, caches, local databases,
  `.archive`. Also excluded as non-implementation surfaces (noted, not rowed):
  `tests/**` and `apps/desktop/e2e`/playwright configs (test harness, not
  product surface), `docs/**` prose (except the two consumed templates),
  `loop/**`, `init/**`, `_harness/adapter.yaml` (method-harness metadata),
  `governance/**` prose (except the certification template),
  `validation/evidence/**` and `validation/witness/generated|inputs|fixtures`
  data (evidence records/outputs — their generators are rowed),
  `validation/benchmarks/*.json` policy/observation records (DEC-050/DEC-053
  evidence records, not generators), `examples/` rowed at family grain,
  `package-lock.json` (covered by the `provenance/build-artifacts` row),
  `docs/MANIFEST.json` (docs register).

## Count summary by Kind (total 231)

| Kind | Count |
|------|-------|
| APP_BINDING | 2 |
| BIN_TARGET | 3 |
| BUILD_CONFIG | 6 |
| DESKTOP_FEATURE | 45 |
| DESKTOP_SERVICE | 12 |
| DESKTOP_SHELL | 1 |
| EVIDENCE_GENERATOR | 4 |
| EXAMPLE | 2 |
| FIXTURE_FAMILY | 24 |
| PACKAGING | 1 |
| PUBLIC_API | 1 |
| PUBLIC_REPO_TEMPLATE | 1 |
| PYTHON_MODULE | 36 |
| RUST_CRATE | 36 |
| SCHEMA | 41 |
| TEMPLATE | 3 |
| TOOL | 13 |

## NONE_FOUND attribution — unmapped-implementation shortlist (8)

Recorded only; not adjudicated (wave input per R1 scope).

- `SURF-011` (DESKTOP_FEATURE) `apps/desktop/src/features/build-readiness` — no execution-tree mention found
- `SURF-021` (DESKTOP_FEATURE) `apps/desktop/src/features/exportUnitDisclosure.ts` — no execution-tree mention found
- `SURF-050` (DESKTOP_FEATURE) `apps/desktop/src/features/telemetry` — no execution-tree mention found
- `SURF-104` (PYTHON_MODULE) `core/product_preview` — no execution-tree mention found
- `SURF-170` (BUILD_CONFIG) `package.json` — no execution-tree mention found
- `SURF-211` (TOOL) `tools/REGISTRY.md` — non-DEL execution matches only, e.g. _Reconciliation/DependencySemanticRefresh/SEMANTIC_REFRESH_2026-06-16/Evidence/git_status_short.txt
- `SURF-212` (TOOL) `tools/coordination/dependency_semantic_refresh_fanin.py` — no execution-tree mention found
- `SURF-213` (TOOL) `tools/coordination/dependency_type_rectification.py` — non-DEL execution matches only, e.g. _Reconciliation/DependencySemanticRefresh/SEMANTIC_REFRESH_2026-06-16/Evidence/git_status_short.txt

## Anomalies / observations

- There is no workspace-level `Cargo.toml`: every crate is a standalone
  manifest (37 total incl. the Tauri shell); pinned lockfiles are mirrored in
  `provenance/build-artifacts/` (26 lockfiles + `package-lock.json`).
- `core/**` is dual-language: 33 Rust crates plus 36 bounded Python modules
  (contract/reference engines). `core/library_import` exists in both forms —
  the Python `provenance_checker.py` (DEL-03-07 contract reference) and its
  runtime Rust port `library_import_document` (per that crate's manifest
  description).
- 18 of 38 `schemas/**` files are named canonically in `docs/SPEC.md`; the
  other 20 (mostly `.schema.json` export/comparison/operation contracts) are
  not, though most are named in DEL kits.
- Five desktop panels are referenced in the execution tree only via DOM
  `data-testid` prefixes (`caepipe-external-`, `caepipe-mbf-`,
  `review-geometry-`, `secret-private-library-`, `stress-neutral-`), not by
  file path or component name.
- `tools/coordination/dependency_semantic_refresh_fanin.py` appears in DEL
  records only via the tranche label `dependency_semantic_refresh` (65 DELs),
  not by script name — treated as NONE_FOUND (tranche label names the
  campaign, not the tool).
- `fixtures/product_preview/invented_mechanics_result.json` is a committed
  generated artifact (regenerated by the root npm script
  `generate:product-preview-mechanics` via the `preview_result` example).
- `validation/evidence/**` contains timestamped generated evidence records
  (sweeps, coverage, gates, release artifacts), including two `-dirty` sweep
  records from 2026-06-12.
