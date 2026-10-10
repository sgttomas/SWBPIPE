# Piping CI

The shared selector in `tools/software_workflow/hosted_ci.py` reads the complete
PR diff and `tools/hosted-ci-routing.json`. Missing diff information selects full
coverage. Jobs check out the exact PR head. `Desktop E2E (source mode)` remains
the stable aggregate; it fails if selected work fails, is cancelled or never
runs. An irrelevant change is reported as not selected, not as a test pass.

Selected desktop changes run the production build, Vitest and one desktop
browser project for the core R2 journey and save/reopen checks. Selected
numerical inputs run the locked Cargo suite and, in parallel, Python product
schemas, qualification, preview and security checks. Python implementation and
test changes select that coverage too. Numerical resource selection also
covers fixtures loaded by Rust tests. Linux and WASM checks exercise portability
that local macOS checks cannot establish.

The full source browser matrix runs nightly or on demand, across both existing
viewport configurations. It does not block ordinary PRs. Dist/native release
checks remain available through the optional macOS release sweep. Failure logs
are CI artifacts; no collection manifests or per-test coverage packets are
committed. Required branch-protection changes are an owner act after the
replacement checks are demonstrated.
