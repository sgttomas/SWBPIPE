# Desktop E2E pull-request trigger narrowed — 2026-09-17

The hosted Desktop E2E workflow, `.github/workflows/piping-desktop-e2e.yml`,
no longer runs for every change under `projects/chirality-piping/`. Its
pull-request trigger now lists the job's actual inputs: the workflow file
itself, the piping `package.json` and `package-lock.json`, and everything
under `apps/`, `core/`, `fixtures/` and `schemas/`. Changes confined to
`execution/`, `plans/`, `docs/`, `loop/`, `validation/` and notice files no
longer start the suite. Manual dispatch, the concurrency rule and every job
step are unchanged. Root tranche manifest:
`docs/governance_harness/tranche_manifests/ROOT-PIPING-E2E-TRIGGER-PATHS-20260917.yaml`.

For this loop: a change that can affect product behaviour but lives outside
the listed paths must extend the allowlist in the same pull request rather
than rely on the suite having run. Host-executed Playwright surfaces required
by DEC-025 remain required evidence an agent runs itself; hosted CI remains
additional evidence. Branch protection on `main` requires only the `harness`
status, so the narrower trigger does not block merges.

This is an application notice, not an adoption request. No acceptance,
lifecycle or release state changes. Standard claim fence applies (F-PIP-2;
claims taxonomy per DEC-081).
