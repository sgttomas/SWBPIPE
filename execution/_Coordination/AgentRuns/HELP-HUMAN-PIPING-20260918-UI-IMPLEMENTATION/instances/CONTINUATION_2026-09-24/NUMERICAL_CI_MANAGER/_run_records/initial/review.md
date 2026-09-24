# Independent review — M27 numerical CI

No actionable findings. The reviewed implementation is suitable for manager fan-in. This is not a merge-readiness, hosted-execution, engineering-acceptance, or release determination.

The review covers the complete working candidate over `f7e8b467cb2db244f11fe49cede636140031b387` in `/private/tmp/piping-numerical-ci-20260924`: five maintained workflow/helper/test files, the canonical tranche manifest, all sixteen declared new Cargo.lock files, and all seventeen manager evidence files present at review completion (39 files total). The lockfiles are ignored by crate-local rules and therefore were explicitly enumerated and read rather than inferred from ordinary Git diff/status. Instruction origins/hashes, actual native delegated-harness parentage and Astra/xhigh attribution, and every reviewed candidate file SHA-256 are in adjacent `review-basis.json`. Workflow, selector, runner, lockfile, and tranche bytes remained unchanged during review. The requested maintained resource-regression tests and final documentary updates were independently backchecked and hash-bound.

Reviewed candidate content-map SHA-256: `6e460b3440254236e9cb2627d36282bab5dce1712fd85e29d0c37e282627ccb0`.

## Behavior and scope

- The numerical requirement is computed independently from the complete PR diff; manual coverage remains required. Removed and renamed input paths remain covered. Unrelated project and ordinary prose edits can remain exempt without allowing a UI-only change to hide a numerical input elsewhere in the PR.
- Candidate/event validation recomputes the plan, binds the hosted event's base/head/PR identity, checks the integrated target, and checks checkout HEAD before the runner discovers or launches commands. The runner requires an explicit Boolean numerical requirement.
- Real readiness discovery finds 38 crates and produces 38 locked fetches followed by 38 complete offline locked crate test commands. Missing/empty manifests or locks fail before execution. No workspace restructuring, manifest-version edits, existing-lock changes, browser provisioning in the numerical job, or exporter change is present.
- Dependency tracing covered compile-time resource includes, runtime fixture reads, the Python parity consumer, Cargo path dependencies, and benchmark provenance sources. All 36 literal Rust resource includes and all 68 benchmark provenance resource references select numerical coverage; the retained frozen mechanics JSON is correctly exempted from the general historical-evidence omission. Rust string line continuation was normalized during the static include audit.
- The existing accessibility barrier, four remainder shards, browser source selection, and stable aggregate check remain connected. Required numerical failure, cancellation, missing/unknown state, or unexpected skip makes the aggregate fail. Browser skip is permitted only under its existing selected mode rules.
- Scope validation using `tools/software_workflow/validate_change_scope.py` passed for all 39 files, with explicit lockfile paths included. The canonical manifest declares the owned instruction surfaces and retains pending ROOT notice/integration disposition without claiming hosted success.

## Verification

The reviewer independently ran `PYTHONDONTWRITEBYTECODE=1 /private/tmp/chirality-piping-dec025-venv/bin/python -m unittest discover -s projects/chirality-piping/tests -p 'test_ci_*.py' -v`: **44 tests passed** (4.298 seconds). `git diff --check` passed for the tracked diff.

The two added maintained tests discover actual Rust includes and current runtime resource paths, handle the existing escaped multiline input, and check selector coverage. Independent temporary negative controls confirmed that a newly referenced excluded evidence fixture fails each guard. These guards cover literal resources; future dynamic resource mechanisms still need explicit dependency review.

All sixteen new lockfiles were read and parsed; root package identity, lock schema, dependency-name closure, and registry checksum shape were checked. Their manifest/lock hashes match the recorded successful offline generation. The disclosed new cfg-if, syn, and unicode-ident resolutions were not mistaken for normalization of existing graphs.

The reviewer read and hash-verified the manager's real negative CLI control: the unchanged production runner bytes validated a temporary candidate, fetched its dependency-free lock, executed the deliberate `4 != 5` assertion, returned Cargo exit 101, and produced aggregate exit 1 under controlled browser-status inputs. The raw logs and rerun script support that bounded failure-propagation claim. The reviewer did not execute Cargo, npm, a browser, or hosted jobs.

## Remaining integration limits

ROOT must force-add exactly the sixteen declared ignored locks and verify they are actually in the staged/committed candidate. This is already recorded in the manager packet. Reconcile lock/manifests with any solver-branch integration, run the final candidate's required product checks, committed-range G4 and hosted workflow, and ensure review coverage remains current after any substantive change. The full 38-crate product suite, fresh hosted dependency availability, Linux execution and the 45-minute job envelope were not established by this local policy review or the temporary failure control.

Later copies of this review/basis and documentary status updates were not part of the 39-file snapshot. They need a bounded documentary backcheck; they do not invalidate the unchanged source/lock review.
