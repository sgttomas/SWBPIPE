# Notice — hosted CI burden reduction

Tranche `ROOT-CI-BURDEN-REDUCTION-20260925` reduces hosted runner time for the
**Piping Desktop E2E** workflow. The stable check name, aggregate gate, barrier
stage, four-partition full mode and manual full dispatch are unchanged.

What changed for Piping:

- **Routing.** Root directories other than the Piping workflow and setup action
  (root `execution/` records, root `tools/`, other workflows, exports) are
  non-inputs for both the desktop and numerical suites. Root-level build files
  and a future root `.cargo/` remain conservative inputs. Piping `validation/`, Python `tests/` and non-CI
  `tools/` now select only the numerical suite; the browser selector reports
  not applicable for them. Desktop, core, fixtures, schemas, examples,
  workspace manifests and `tools/ci/` keep their existing selection.
- **Compact profile scope on pull requests.** Pull-request selection runs every
  `chromium-desktop` identity and runs `chromium-compact` only for B3
  accessibility, workspace layout, C3 viewport visibility, B4 table editing and
  Sections, linear authoring, R2 smoke, and the named UI-foundation layout
  titles. Omitted compact identities are recorded in the collection artifact
  with that reason. A pull-request full run is therefore not a DEC-093
  surface-4 CI binding (`run_evidence_sweep.py` requires both viewport
  projects). Manual `workflow_dispatch` full runs, `playwright.config.ts`,
  local runs and DEC-025 evidence sweeps keep both profiles in full, so the
  dual-viewport CI binding path is unchanged: dispatch a full run for it.
- **Checkout.** Piping jobs no longer check out `execution/` run evidence
  (about 2.8 GB of the 3 GB tree). No Piping selector, build or test reads it.

Observed motivation (2026-09-24 to 2026-09-25 hosted runs): Piping Desktop E2E
was about 77% of runner time. Coordination notices under root `execution/` and
root `tools/` changes had selected full browser and numerical coverage for
unrelated PRs. Each Piping job spent about 60 seconds checking out evidence.

The Piping loop decides whether to adopt, amend or revert the pull-request
compact scope. Restoring it is a one-line change to `hosted_profile` in
`tools/ci/e2e_plan.py`. This notice does not repin an authority corpus or
change local check profiles, native witnesses, release gates or
DEC-025/DEC093 evidence obligations.
