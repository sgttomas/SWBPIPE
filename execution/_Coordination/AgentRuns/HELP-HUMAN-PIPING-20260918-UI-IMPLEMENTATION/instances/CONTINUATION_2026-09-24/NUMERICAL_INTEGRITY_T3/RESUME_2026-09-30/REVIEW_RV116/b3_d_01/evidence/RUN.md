# RV116: execution record

Placeholders: WT, NUM, P (= NUM's `projects/chirality-piping`), R and VENV as in the dispatch. Scratch: `WT/scratch/rv116_rvd/` (disposable). Python 3.13.14 from VENV, standard library only. Every Python run used `PYTHONDONTWRITEBYTECODE=1` and `TMPDIR=WT/scratch/rv116_rvd/tmp`; the tmp folder stayed empty. Nothing was written outside the scratch folder and this record folder.

| Step | Command (run in `WT/scratch/rv116_rvd`) | Output | Runs |
|---|---|---|---|
| 1 | `cp R/I96/b3_d_01/_run_records/b3d_statics.py .` then `VENV/bin/python b3d_statics.py NUM/P regen` | `regen/` (the three drafts, the scratch SCHEMA, `b3d_statics.out.json`); compared in `regeneration.txt` | 1 |
| 2 | `VENV/bin/python rv116_checks.py NUM/P regen/retained_precision_prepared_exact_v1.json rv116_checks.out.json` | `rv116_checks.out.json` (H controls and DEF-E's H; the 18 published sections; the correctly rounded and preview sections; 3,000 random sections; 250,017 Ĝ cases) | 1 |
| 3 | `VENV/bin/python rv116_s2_illustration.py NUM/P regen rv116_s2_illustration.out.json` | `rv116_s2_illustration.out.json` (the hashes S-2's proposed wording would give; illustrative only) | 1 |
| 4 | `GIT_OPTIONAL_LOCKS=0 git -C NUM grep -F [-w] -l <name> HEAD -- P ':!P/execution'` per name, and the same over the whole repository | `collisions.txt` | 1 |
| 5 | `GIT_OPTIONAL_LOCKS=0 git -C NUM show codex/piping-t3-b1-20261007:P/core/product_physics/src/{lib.rs,retained_memory.rs}` | read copies in scratch only (`permitted_run`, `ordinary_dispatch`, `family_clauses`, `FamilyFact`) | 1 |

`rv116_checks.py` was written by RV116 and imports nothing from I96's scripts. `rv116_s2_illustration.py` imports the unchanged scratch copy of I96's `b3d_statics.py` (for `physics_retained_table`) and RV116's own canonicalizer, and asserts the two canonical forms agree.

**Git reads** used `GIT_OPTIONAL_LOCKS=0`: `rev-parse`, `log`, `diff --quiet`, `diff --stat`, `show`, `grep`, `branch`. NUM's HEAD moved from `73b5d4e2a6` (dispatch) to `d674763934` (RV115's addendum, records only) during the review; at both, `P/core`, `P/fixtures`, `P/schemas`, `P/apps` and `P/tests` equal main `2007709549` (`git diff --quiet`). No Git write, no cargo, native or solver job, no install.

**Reproduce:** rerun steps 1–4 on a tree whose `P/core`, `P/fixtures` and `P/schemas` equal main `2007709549`. Steps 1–3 are deterministic (seeded draws: 116 and 1160).
