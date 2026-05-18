---
doc_id: TP-RUN-01
doc_kind: implementation.plan
status: implemented
created: 2026-05-10
closed: 2026-05-10
---

# TP-RUN-01 Preview Runtime Spine Plan

## Purpose

Unify desktop and headless preview runtime execution around
`core/product_physics` so both surfaces can consume live computed preview
results from supplied invented or cleared preview models.

TP-MAC-05 is the closed mechanics baseline for this tranche. TP-RUN-01 should
connect runtime surfaces to that baseline without reopening mechanics scope.

## Scope

- Add optional preview model input to the desktop Tauri preview mechanics
  command while preserving the existing fixture fallback when no model is
  supplied.
- Update the React preview service so the Tauri solve path passes the currently
  loaded model into the runtime when Tauri is available.
- Preserve browser and Vitest fallback behavior through the existing generated
  invented mechanics fixture.
- Extend the headless runner as an in-memory preview runner bridge around
  `core/product_physics`, accepting structured runner metadata and a
  `LinearStaticPreviewRequest`.
- Return computed mechanics output through the existing result-envelope path,
  including TP-MAC-05 endpoint stress component rows.
- Add runner metadata, deterministic result references, audit-manifest
  references, and checksums/hashes sufficient for run-context and report-packet
  audit trails.
- Keep runner privacy and professional-boundary fields explicit and local-first.

## Boundaries

- No final CLI binary, command syntax, package scripts, or public command
  contract.
- No persistence layer, project container, migrations, or stored run history.
- No external execution, public transport, adapter format, local FEA invocation,
  prover invocation, external result ingestion, or network integration.
- No new mechanics: intermediate stations, shear recovery, pressure-to-frame
  load conversion, equivalent stress, and principal stress remain deferred.
- No governed calculation-report rendering beyond the existing read-only packet
  context.
- No compliance, certification, sealing, professional approval,
  release-readiness, production-readiness, or engineering acceptance claims.
- Use invented or explicitly cleared data only.

## Public Interfaces

- The Tauri preview mechanics command may accept an optional preview model
  payload; the existing zero-argument path remains valid for fallback use.
- The React preview service passes the current loaded model to Tauri when the
  desktop runtime is available.
- The headless runner exposes library-level structured preview execution, not a
  final user-facing CLI.
- Runner results include deterministic refs and hashes for the mechanics result
  envelope and audit context.

## Acceptance Criteria

- Desktop Tauri solve computes from a supplied current preview model payload.
- Browser and Vitest fallback paths continue to work from the generated
  invented mechanics fixture.
- Headless preview execution validates runner metadata and preview request
  shape, returns blocking diagnostics for invalid requests, and returns a
  mechanics envelope plus runner result metadata for valid invented requests.
- Result refs and hashes include TP-MAC-05 endpoint stress component rows
  through the existing result-envelope path.
- Existing TP-MAC-05 desktop smoke behavior remains intact.
- Coordination handoff files point the next instance at TP-RUN-01 as the active
  implementation tranche and keep TP-MAC-05 as the closed mechanics baseline.

## Verification

- `cargo test --manifest-path core/product_physics/Cargo.toml`
- `cargo test --manifest-path core/runner/headless/Cargo.toml`
- `python3 -m pytest tests/product_preview/test_product_preview_service.py tests/test_analysis_run_records.py tests/test_results_schema.py`
- `python3 tests/test_headless_runner_contract.py`
- `npm test --workspace apps/desktop`
- `npm run build --workspace apps/desktop`
- Browser smoke using `apps/desktop/SMOKE.md`
- `git diff --check`

## Assumptions

- The headless runner remains a library-level structured I/O bridge in this
  tranche.
- The invented preview model remains the only default public runtime input.
- Existing fixture fallback remains for non-Tauri browser and test
  environments.
- TP-MAC-05 mechanics behavior and result IDs are preserved unless a later
  governed plan explicitly changes them.

## Closeout

TP-RUN-01 is implemented for the current product-preview workflow.

Delivered:

- optional preview model payload input for the Tauri preview mechanics command,
  preserving the zero-argument invented-fixture fallback;
- React preview service routing of the loaded preview model into the desktop
  runtime solve path when Tauri is available;
- preserved browser and Vitest fixture fallback behavior;
- in-memory headless runner bridge around `core/product_physics` accepting
  structured runner metadata plus `LinearStaticPreviewRequest`;
- runner output containing the mechanics envelope, result-envelope ref,
  deterministic result refs including TP-MAC-05 endpoint stress rows,
  audit-manifest ref, privacy/professional-boundary fields, and SHA-256
  checksums;
- blocking diagnostics for invalid runner metadata before mechanics execution.

Final verification was run on 2026-05-10:

- `cargo test --manifest-path core/product_physics/Cargo.toml`
- `cargo test --manifest-path core/runner/headless/Cargo.toml`
- `python3 -m pytest tests/product_preview/test_product_preview_service.py tests/test_analysis_run_records.py tests/test_results_schema.py`
- `python3 tests/test_headless_runner_contract.py`
- `cargo check --manifest-path apps/desktop/src-tauri/Cargo.toml`
- `npm test --workspace apps/desktop`
- `npm run build --workspace apps/desktop`
- browser smoke against `http://127.0.0.1:5173/` using `apps/desktop/SMOKE.md`
- `git diff --check`

The desktop build retained the existing Vite chunk-size warning. Final CLI
syntax, persistence, project containers, external execution, new mechanics,
release/professional claims, and professional acceptance workflows remain
deferred.
