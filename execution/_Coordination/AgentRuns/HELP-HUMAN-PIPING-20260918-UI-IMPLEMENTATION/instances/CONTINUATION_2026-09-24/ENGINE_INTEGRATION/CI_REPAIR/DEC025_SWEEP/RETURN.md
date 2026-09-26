# DEC-025 sweep for PR905

HELP_HUMAN (ROOT) ran the registered DEC-025 five-surface sweep on a clean checkout of PR905 head `6d9c0915f1c46563bc123fc7196109f835d6708a`. Overall status: **pass**. [Summary](_run_records/SWEEP_20260925T213252Z_6d9c0915f1c4.json), [binding](_run_records/surface4-ci-binding.json), [invocation](_run_records/invocation.txt), [log (gzip)](_run_records/sweep.log.gz).

| Surface | Result |
|---|---|
| cargo_crate_sweep | pass (39 crate manifests) |
| python_pytest | pass: 1,750 passed, 15 skipped |
| desktop_vitest | pass: 118 files, 2,059 tests |
| desktop_playwright_e2e | pass: CI-bound to full dual-viewport dispatch run 36190876950 on the same head |
| desktop_production_build | pass |

The records commit that follows this sweep changes only files under `projects/chirality-piping/execution/`, which are not build, test or selection inputs. The sweep therefore covers the product, test and fixture bytes of the merging revision; hosted CI must still pass on that final head. This is development evidence, not a release, acceptance or certification claim.
