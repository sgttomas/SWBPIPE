# CANDIDATE Brief - DEL-10-04 CI Browser Provisioning And Phase Mapping

**Brief ID:** CB-2026-07-15-DEL-10-04-CIBROWSER-001  
**Status:** CANDIDATE - AWAITING OWNER ADOPTION  
**Prepared:** 2026-07-15  
**Deliverable:** DEL-10-04 - Build, packaging, and CI/CD pipeline  
**Package:** PKG-10  
**Target stage:** PRD R5 / Phase E  
**Execution posture:** one bounded agent; no delegation required

## Purpose And Accepted Basis

Close the two ungated documentation residuals in DEL-10-04 `_STATUS.md`:

1. record the CI browser-provisioning policy for Playwright surfaces; and
2. map prospective provider workflow jobs to the provider-neutral phases in
   `docs/BUILD_AND_RELEASE.md` §7.

The brief is grounded in SOW-032 / OBJ-008 / OBJ-009, AB-00-08,
`DEC-020`, `DEC-025`, `DEC-057`, `DEC-059`, the existing root npm scripts,
the two Playwright configs, and the DEC-025 sweep. DAG-007 has seven active
execution-upstream rows for DEL-10-04 and all seven are `SATISFIED`.

This brief does not select or activate a hosted CI provider. D-05b/publication
prerequisites, the D-20 owner scan/sign-off, D-06b signing/notarization, public
repository creation, and release authority remain gated.

## Objective

Make the existing provider-neutral build guide sufficiently explicit that a
future authorized CI workflow can provision and execute the current browser
evidence lanes without relying on a maintainer's installed Google Chrome or
silently omitting the production-dist lane.

## Work Decomposition

### T1 - Browser provisioning policy

Document, from the installed Playwright CLI and live configs:

- dependency installation remains lockfile-bound (`npm ci` in the project
  workspace when a future exported/public workflow is authorized);
- browser provisioning uses the Playwright-managed Chromium matching the
  installed package (`npx playwright install --with-deps chromium` on a
  supported Linux CI image; equivalent dependency preparation must be
  declared on other images);
- CI must not depend on the macOS Google Chrome fallback path;
- `CI=true` and the config's one-worker posture apply unless an explicit
  evidence-backed `PLAYWRIGHT_WORKERS` value is supplied;
- both `npm run test:e2e:desktop` and
  `npm run test:e2e:dist:desktop` execute, with wasm rebuilt through the
  existing scripts; retained failure traces are diagnostic artifacts, not
  release evidence by themselves.

### T2 - Provider-neutral phase mapping

Extend `docs/BUILD_AND_RELEASE.md` §7 with a provider-neutral job table that
maps repository sanity, Python/security, all-crate Rust, wasm build + Vitest,
Playwright dev-server lane, Playwright production-dist lane, and production
build to the existing commands. Preserve the DEC-025 sequential/F-4-safe
ordering and point to the commit-bound sweep as the authoritative local
aggregate command.

### T3 - Deliverable closeout

Update DEL-10-04 `ScopeOfWork.md`, `_STATUS.md`, `MEMORY.md`, and a new
deliverable-local run record. Remove only the two landed documentation
residuals. Preserve the public-CI and D-06b gated items unchanged.

## Write Fence

Allowed writes:

- `projects/chirality-piping/docs/BUILD_AND_RELEASE.md`
- DEL-10-04 `ScopeOfWork.md`, `_STATUS.md`, `MEMORY.md`, and `_run_records/**`
- loop receipt and DEC-025 sweep summary required by closeout

Read-only evidence:

- `projects/chirality-piping/package.json`
- `projects/chirality-piping/apps/desktop/package.json`
- `projects/chirality-piping/apps/desktop/playwright.config.ts`
- `projects/chirality-piping/apps/desktop/playwright.dist.config.ts`
- `projects/chirality-piping/tools/release/run_evidence_sweep.py`
- root `.github/workflows/**`

Forbidden writes include `.github/**`, package manifests/locks, Playwright
configs, source code, public-export contents, release artifacts, signing or
publication surfaces, and every path outside `projects/chirality-piping/**`.

## Validation Criteria

- The documented commands exactly match live package scripts and installed
  Playwright CLI capability.
- Both source-mode and production-dist browser lanes are explicit.
- The mapping preserves the DEC-025 five-surface order and does not represent
  any root monorepo workflow as an activated piping CI workflow.
- Provider, public-repository, scan/sign-off, signing, publication, coverage,
  performance, and release acceptance gates remain explicit.
- Focused checks pass:
  `python3 -m pytest -q tests/test_evidence_sweep.py tests/test_release_readiness_script.py tests/test_export_public_openpipestress.py`.
- Full project Python suite, repo-wide self-check, practitioner-harness pytest,
  receipt validator, `git diff --check`, and documentation path/boundary review
  pass.
- Because the adopted implementation would be documentation-only and would
  not change browser behavior, the DEC-025 code-touching sweep is not required
  by this brief. If execution changes any executable surface, the sweep becomes
  mandatory before push.

## Acceptance And Handoff

Owner adoption authorizes only this bounded documentation tranche. It does not
authorize hosted CI, public export, publication, signing, an R5 exit claim, or
any lifecycle transition. On completion, the resulting run record and receipt
must identify the source authority, derivative status, residual gates, and
rerun trigger (changes to package scripts, Playwright configs, or DEC-025
ordering).
