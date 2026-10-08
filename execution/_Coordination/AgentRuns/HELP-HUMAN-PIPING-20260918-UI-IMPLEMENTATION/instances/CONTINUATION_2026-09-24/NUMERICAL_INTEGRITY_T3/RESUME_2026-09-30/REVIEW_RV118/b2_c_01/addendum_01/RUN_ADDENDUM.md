# RV118 (RV-C) ADDENDUM_01: execution record

**Placeholders** are as in REVIEW.md: WT, NUM, P (NUM's `projects/chirality-piping`), R, VENV.
- `S` is my scratch folder `WT/scratch/rv118_b2_c` (disposable).
- `O` is `R/I97/b2_c_01`.
- TMPDIR was `S/tmp`.

**Environment.**
- **Python:** 3.13.14 from VENV. Every VENV run had `PYTHONDONTWRITEBYTECODE=1` and `-B`.
- **Git** reads used `GIT_OPTIONAL_LOCKS=0`.
- **Not run:** no cargo, native, solver or test job, no install, no Git write.
- **Writes:** nothing outside `S` and this record folder.

**Heads.**
- **NUM:** `d1d6517455` at the start and `827dc41c2e` at the end. ROOT committed records only in between: RV115's ADDENDUM_03 and the (ii) ruling. P's `core`, `fixtures`, `schemas`, `apps` and `tests` trees are equal at both heads (`git diff --quiet`).
- **B1, read-only:** `b1` `03f55e7178`, `b1-r` `6e3e4fe219`, `b1-p` `2843a59a16` and `b1-t` `6fa6a64658`, all clean.

| Step | Command | Output | Runs |
|---|---|---|---|
| A1 | `shasum -a 256` of the brief, REVISION_01.md and RV115's ADDENDUM_03. `shasum -a 256 -c` of O's `SHA256SUMS.revision_01` and `SHA256SUMS`, my `SHA256SUMS` and RV115's `SHA256SUMS.addendum_03` | `checks_addendum.txt` §1 | 1 |
| A2 | Copy I96's `b3d_statics.py`, `revision_01/b3d_statics_r1.py` and `statics/r1/*` into `S/i96/`, and O's `_run_records/*`, `_run_records/r1/*` scripts and `statics/*` into `S/i97/`, keeping the relative layout. Then `cmp` each copy with its committed file | equal | 1 |
| A3 | `VENV -B S/i97/_run_records/r1/b2c_statics_r1.py P S/i96 S/i97/_run_records/b2c_statics.py S/i97/statics S/work/r1_X`, for X = a, b | `diff -r` identical. The DEF-C r1, PTABLE r1 and `b2c_statics_r1.out.json` files are `cmp`-equal to O's | 2 |
| A4 | `VENV -B S/i97/_run_records/r1/b2c_checks_r1.py P P/core/analysis_runs/retained_precision.py S/i97/statics/retained_precision_mp_v2.schema.json S/work/r1_X/semantic_contract_v0_3_preview_physics_retained_1.json S/i97/_run_records/b2c_checks.py S/work/checks_r1_X.out.json`, for X = a, b | `cmp`-equal to each other and to O's `_run_records/r1/b2c_checks_r1.out.json` | 2 |
| A5 | `sh S/i97/_run_records/r1/b2c_collisions_r1.sh NUM`, run in `S/work` | Every line after the header equals O's `b2c_collisions_r1.out.txt`. The header names NUM's HEAD, `d1d6517455` | 1 |
| A6 | `VENV -B rv118_r1_checks.py P O/statics/retained_precision_prepared_combination_v1.json O/statics/r1 O/statics/semantic_contract_v0_3_preview_physics_retained_1.json S/i96/statics/r1/semantic_contract_v0_3_physics_retained_1.json <out>`, for two outputs | `rv118_r1_checks.out.json`. The two outputs are byte-identical | 2 |
| A7 | `VENV -B rv118_a01_option_ii.py <out>`, for two outputs | `rv118_a01_option_ii.out.json`. The two outputs are byte-identical | 2 |
| A8 | Code reading with `grep -n` and `sed -n` at NUM and on the four B1 worktrees | The sites in `checks_addendum.txt` §3 | — |

**What the scripts import.**
- **`rv118_r1_checks.py`** is standard library only.
  - Its JCS canonicalizer is the one in my sealed `evidence/rv118_hashes.py`, written from RFC 8785.
  - It imports nothing from I96, I97 or production.
  - **Arguments:** DEF-O and main's PTABLE are read under `P/fixtures/results`. XTABLE is I96's r1 static, from my scratch copy, which `cmp` equals the record.
- **`rv118_a01_option_ii.py`** is standard library only and takes no inputs.

**Host slip.** I made one `python3 -c` call with the host interpreter, not VENV. It loaded `S/work/rv118_a01_option_ii_a.out.json` and printed the zero-triple summary, and it wrote nothing.

**Screen,** run on this record folder before returning:
- the strict path pattern, the machine's host name and `.local`, judged by what each names (there is no gzipped file here);
- `find <folder> -type l`;
- `git status --ignored` on the folder;
- `find NUM -name __pycache__` limited to R.
