# RV116 addendum 01: execution record

Placeholders: WT, NUM, P (= NUM's `projects/chirality-piping`), R and VENV as in the dispatch. Scratch: `WT/scratch/rv116_rvd_01/` (disposable). Python 3.13.14 from VENV, standard library only; every run used `PYTHONDONTWRITEBYTECODE=1` and `TMPDIR=WT/scratch/rv116_rvd_01/tmp`, which stayed empty. Nothing was written outside the scratch folder and this record folder.

| Step | Command (in `WT/scratch/rv116_rvd_01`) | Output | Runs |
|---|---|---|---|
| A1 | `cp R/I96/b3_d_01/_run_records/b3d_statics.py R/I96/b3_d_01/_run_records/revision_01/b3d_statics_r1.py .`, then `VENV/bin/python b3d_statics.py NUM/P v0`, `VENV/bin/python b3d_statics_r1.py NUM/P r1` and the same into `r1b` | `regeneration_r1.txt` | 3 |
| A2 | `cp WT/scratch/rv116_rvd/rv116_checks.py .` (RV116's own canonicalizer, sha256 `5224be42…`, as sealed in `evidence/`), then `VENV/bin/python rv116_r1_checks.py NUM/P R/I96/b3_d_01 rv116_r1_checks.out.json` | `rv116_r1_checks.out.json` | 1 (a first attempt stopped on a parser bug in my own diff applier, fixed in scratch and rerun; no output was written by the failed attempt) |
| A3 | `GIT_OPTIONAL_LOCKS=0 git -C NUM grep -F -l <name> HEAD -- P ':!P/execution'` and over the repository, for `nonempty_pressure_regions` and the three new hash prefixes | 0 hits each (stated in ADDENDUM_01.md §2) | 1 |
| A4 | `shasum -a 256 -c` on I96's `SHA256SUMS` and `SHA256SUMS.revision_01`, and on this folder's sealed `SHA256SUMS` | 11/11, 20/20, 8/8 OK | 1 |

`regeneration_r1.txt`'s last line (the generator's `controls` member) was printed by a one-line read of the scratch out file with the host's `python3`, not VENV's; it only parsed JSON.

**Git reads** used `GIT_OPTIONAL_LOCKS=0`: `rev-parse`, `log`, `diff --quiet`, `grep`, `ls-files`. NUM's HEAD is `173f8778c9`; its `P/core`, `P/fixtures`, `P/schemas`, `P/apps` and `P/tests` equal main `2007709549`. No Git write, no cargo, native or solver job, no install, no lock taken (no heavy job).
