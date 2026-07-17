# DEL-10-04 — CI Browser Provisioning And Phase Mapping

**Date:** 2026-07-16

**Brief:** `CB-2026-07-15-DEL-10-04-CIBROWSER-001` (owner-adopted 2026-07-16)

**Execution:** single agent; no delegated run

**Lifecycle:** `IN_PROGRESS` (unchanged)

## Work And Evidence

The adopted documentation tranche compared `docs/BUILD_AND_RELEASE.md` §7
against the root and desktop npm scripts, both Playwright configs, the
installed Playwright CLI, and the `DEC-025` five-surface sweep contract. The
guide now records:

- lockfile-bound dependency installation and Playwright-managed Chromium
  provisioning for a future authorized workflow;
- CI server-reuse and worker posture without depending on the macOS Chrome
  fallback;
- both `npm run test:e2e:desktop` and
  `npm run test:e2e:dist:desktop`; and
- a provider-neutral phase table that preserves the sequential `DEC-025`
  surface order and retains an explicit final production-build confirmation.

No executable surface, hosted workflow, package manifest/lock, Playwright
config, public export, release artifact, signing, or publication surface was
changed.

## Validation

- Installed Playwright CLI help confirms `install --with-deps` and the
  `chromium` browser target.
- Focused adopted-brief tests: 44 passed.
- Full project Python suite: 498 passed.
- Practitioner-harness suite: 266 passed.
- Repo-wide self-check: exit 0; pre-existing cross-project finding inventory
  unchanged.
- Receipt validator and `git diff --check`: pass at closeout.
- Documentation boundary review: only the adopted guide and governed
  DEL-10-04/coordination/loop records changed; no executable file changed, so
  the `DEC-025` code-touching sweep was not required.

## Knowledge Classification And Handoff

- **Authority:** the owner-adopted brief authorizes this bounded work; the
  underlying commands and behavior remain governed by their live executable
  sources and existing rulings.
- **Status:** `docs/BUILD_AND_RELEASE.md` and the deliverable records are draft
  derivative engineering documentation, not a live CI workflow or release
  acceptance record.
- **Provenance:** `SOW-032`, `OBJ-008`/`OBJ-009`, AB-00-08, `DEC-020`,
  `DEC-025`, `DEC-057`, `DEC-059`, package scripts, Playwright configs, and
  the installed Playwright CLI.
- **Supersession:** the expanded §7 supersedes its prior shorter phase map;
  it does not supersede `DEC-025` or any executable configuration.
- **Residual uncertainty/gates:** hosted/public CI remains `D-05b`-gated;
  signing/notarization or its explicit PRD deviation remains `D-06b`-gated;
  provider, thresholds, publication, and release acceptance remain unresolved
  or owner-controlled.
- **Rerun trigger:** changes to package scripts, either Playwright config,
  Playwright provisioning syntax, or `DEC-025` ordering.

The two documentation residuals are removed from `_STATUS.md ## Remaining`.
The public-CI and signing/notarization residuals remain unchanged.
