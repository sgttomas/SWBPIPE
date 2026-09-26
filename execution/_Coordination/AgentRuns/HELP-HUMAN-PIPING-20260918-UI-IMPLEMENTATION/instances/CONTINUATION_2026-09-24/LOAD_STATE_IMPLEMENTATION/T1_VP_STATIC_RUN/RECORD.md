# T1 VP-STATIC development comparison: load/reference states (recorded run)

- **Run by:** the T1 WORKING_ITEMS manager (integration lane), after `T1_WAVE1_REVIEW_A` and its backcheck, once ROOT released the host hold.
- **Purpose:** `development_comparison`, one run per solver mode.
- **Scope of the claim:** this records observed outcomes of the admitted VP-STATIC package against the product. It is not app Current, engineering acceptance, physical validation or release qualification, and on its own it closes no M10, M16 or M29 finding.
- **Paths:** WORKING_ROOT-relative. Machine records are in `_run_records/`, with absolute paths replaced by `<WORKING_ROOT>`, `<SCRATCH>` and similar placeholders.

## Identities

| Item | Value |
|---|---|
| Candidate commit (clean tree) | `d58baad57959f8fda282746708e1c996b145069b` |
| Runner executable | `openpipestress-runner`, dev build from the candidate, sha256 `6bedade8f54923956a2301195d3d8ed46b6c42be9756ca3b0b3ad697b045d3d4` |
| Adapter | `tools/validation/qualification_load_reference.py`, transport `load_reference_1_cli_1.0_raw0.2` |
| Case manifest | `validation/qualification/fixtures/load_reference/MANIFEST.json` (admitted package; hash in the selection records) |
| Reader binding | `openpipestress.load_reference_consistency_binding/1`, status `reviewed_candidate`. `review_basis`: `T1_WAVE1_REVIEW_A/RETURN.md`, `T1_WAVE1_REVIEW_A/BACKCHECK.md`, `REVIEW_CHECKPOINT_3/RETURN.md`, `REVIEW_CHECKPOINT_4/RETURN.md`. Reader module `core/analysis_runs/load_reference_evidence.py` sha256 `14e1750e…` |
| Process limits | 300 s per case; output limit 64 MiB, the adapter's maximum. Two cases emit about 14 MB and 22 MB of stdout |
| Environment | `_run_records/environment.json` (Linux x86_64, Python 3.11.15, Rust 1.97.1) |

**Command:** `LSI/_run_records/session4/t1_vp_static_run.py`, run as:

`python <script> <WORKING_ROOT> <runner> <scratch> <review paths…>`

## Result

| Mode | Outcome | Cases | Assertions matched / required | Positive | Negative | Structural checks | Numerical standing |
|---|---|---|---|---|---|---|---|
| sparse_interactive | `all_required_assertions_matched` | 14 | 507 / 507 | 467 / 467 | 40 / 40 | 98 / 98 | 14 `checks_passed` |
| dense_scrutiny | `all_required_assertions_matched` | 14 | 507 / 507 | 467 / 467 | 40 / 40 | 98 / 98 | 14 `checks_passed` |

- Nothing failed, was blocked, errored or was not run.
- The families are support motion (M10), reference temperatures (M16) and cold spring (M29), taken from 12 independent analytical references (`core/product_physics/tests/fixtures/load_reference_states/reference_cases.json`).
- The 40 negative assertions confirm that the product does **not** reproduce the references' wrong-result discriminators.
- The adapter's owned-reader consistency check agreed on every case.

**Records:**

- `_run_records/ledger-<mode>.json` and `summary-<mode>.md`: the full ledgers, sanitized;
- `selection-<mode>.json` and `reader_binding.json`: the inputs;
- `artifacts-<mode>.sha256`: the sha256 of every retained artifact, including raw runner stdout, stdin and reader snapshots;
- `run_result.json`: the script output.

**Raw outputs** (about 148 MB) are not committed. They reproduce deterministically: rerun the command on the candidate commit with the same executable, and compare against `artifacts-<mode>.sha256`.

## What this does and does not establish

**Established.** The ordinary `load-reference-1` route, on the candidate, reproduces the admitted independent references in both modes, within the admitted per-rule criteria. Those criteria are relative 1e-9 for nonzero references and explicit zero-scale absolutes, under the existing symmetric PKG-14 classifier. The route covers:

- prescribed support translations and rotations (two-bar, all-fixed, free-tip);
- per-member thermal and reference states (datum ratios, coefficient definitions, constant α intervals, the serial shared-material companion, multi-segment free length, temperature-unit identity);
- signed cold-spring fit states (fixed and released, cold/hot/return, persistent source once);

every case with `checks_passed` standing.

**Not established:**

- The joined `load-reference-source-1` route. It has no VP-STATIC case, and its results are `needs_recompute` (T3).
- The excluded or unauthored reference items recorded in the package README: `shared_material_parallel`, the signed-fit baselines and the unscored gaps.
- Native application behaviour: the owner's Mac witnesses are outstanding.
- The WP7 PR qualification.
