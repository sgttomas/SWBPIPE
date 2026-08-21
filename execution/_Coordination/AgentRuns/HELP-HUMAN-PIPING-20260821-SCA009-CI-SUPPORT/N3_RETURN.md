# N3 return — CI fragility preventive hardening

- Run: `HELP-HUMAN-PIPING-20260821-SCA009-CI-SUPPORT`
- Node: `N3`
- Executor: `WORKING-ITEMS-CI-HARDENING` (`WORKING_ITEMS`, Agent 1)
- Basis: `b1876a5e0f0083e10c0c18255cd92ed0079b63a2`
- Verdict: `PASS`
- Blockers: none

## Design decisions

1. Selected Playwright config disablement rather than merge-base prefetch.
   `playwright.config.ts` now sets
   `captureGitInfo: { commit: false, diff: false }`. This removes the implicit
   network path at its source instead of adding fetch behavior to the existing
   depth-1 checkout. Commit/run identity is already carried by the evidence
   sweep's workflow/run/head-SHA binding, so Playwright metadata was redundant.
2. The dist config is unchanged. `.github/workflows/piping-desktop-e2e.yml`
   invokes the source config only; it does not execute
   `playwright.dist.config.ts`.
3. Candidate whitespace uses a step-level `timeout-minutes: 3`. This leaves
   margin above ordinary object access while bounding the observed 15-minute
   lazy-fetch hang well below the job cap. A failure-only following step emits
   a legible GitHub annotation that distinguishes validator-reported
   whitespace defects from a log that stopped during lazy base-blob fetching.
4. One G4 manifest covers the single owner-directed tranche. It records the
   root workflow and coupled policy-test surfaces, names the project-local
   Playwright config as an implementation path, and defers regeneration of the
   public Chirality App derivative export until an accepted post-merge
   instruction-surface snapshot exists.

## Files written

- `.github/workflows/governance-harness.yml`
- `projects/chirality-piping/apps/desktop/playwright.config.ts`
- `tools/validation/test_validate_instruction_tranche_manifest.py`
- `docs/governance_harness/tranche_manifests/PIPING-CI-SLOW-OBJECT-HARDENING-20260821.yaml`
- This node-local return, status, and `N3_CHECKS.json`

Intentionally unchanged:

- `.github/workflows/piping-desktop-e2e.yml`
- `projects/chirality-piping/apps/desktop/playwright.dist.config.ts`

## Validation

- Focused G4 policy tests: `PASS`, 47 tests.
- Whole-corpus G4 validation: `PASS`, 40 schema-valid manifests.
- YAML parse of both relevant workflows and the new manifest: `PASS`.
- TypeScript check of `playwright.config.ts`: `PASS`.
- CI-shaped Playwright config/list check with
  `DEBUG_GIT_COMMIT_INFO=1`: `PASS`, 22 tests discovered across the two
  registered viewport projects, with no GitCommitInfo git/fetch activity.
- Scoped Candidate whitespace validation: `PASS`, with zero skipped untracked
  binary or symlink paths.
- Scoped `git diff --check`: `PASS`.

Exact commands and the two corrected command-invocation mistakes are recorded
in `N3_CHECKS.json`. No slow-serving reproduction, browser run, Actions rerun,
unsigned-artifact lane, commit, push, or PR action was performed.

## Fan-in disposition

The node is ready for HELP_HUMAN fan-in. The G4 manifest's derivative-export
deferral remains explicit; no routed M6 notice is required because no agent,
skill, policy prose, tool CLI, or downstream authority-corpus surface changed.
