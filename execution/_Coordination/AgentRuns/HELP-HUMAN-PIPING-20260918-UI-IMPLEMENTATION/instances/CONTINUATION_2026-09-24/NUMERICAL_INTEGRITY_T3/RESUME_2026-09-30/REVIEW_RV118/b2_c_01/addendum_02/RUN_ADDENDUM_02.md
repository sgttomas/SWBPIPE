# RV118 (RV-C) ADDENDUM_02: execution record

**Placeholders** are as in REVIEW.md: WT, NUM, P (NUM's `projects/chirality-piping`), R, VENV.
- `S` is my scratch folder `WT/scratch/rv118_b2_c` (disposable).
- `O` is `R/I97/b2_c_01`.
- `A01` is `R/REVIEW_RV118/b2_c_01/addendum_01`.
- TMPDIR was `S/tmp`.

**Environment.**
- **Python:** VENV's 3.13.14 only, with `PYTHONDONTWRITEBYTECODE=1` and `-B` on every run. No host `python3` was used.
- **Git** reads used `GIT_OPTIONAL_LOCKS=0`.
- **Not run:** no cargo, native, solver or test job, no install, no Git write.
- **Writes:** nothing outside `S` and this record folder. `find` shows no `__pycache__` in `S`, O or this folder.

**Heads.** NUM was `c8e54918cd` at the start and `7955efb86f` at the end (erratum E-16, records only). P's `core`, `fixtures`, `schemas`, `apps` and `tests` trees are equal at both heads (`git diff --quiet`).

| Step | Command | Output | Runs |
|---|---|---|---|
| B1 | `shasum -a 256` of the brief and REVISION_02.md. `shasum -a 256 -c` of O's `SHA256SUMS.revision_02`, `.revision_01` and `SHA256SUMS`, and of my `SHA256SUMS` and `SHA256SUMS.addendum_01` | `checks_addendum_02.txt` §1 | 1 |
| B2 | Copy O's `_run_records/r2/*.py`, `_run_records/r1/*` scripts and `statics/r1/*` into `S/i97/`, keeping the relative layout (v0's scripts, `statics/` and I96's copies already sat in `S` from ADDENDUM_01). Copy A01's two scripts into `S/w2/a01/`. Then `cmp` every copy with its committed file | equal | 1 |
| B3 | `VENV -B S/i97/_run_records/r2/b2c_statics_r2.py P S/i96 S/i97/_run_records/b2c_statics.py S/i97/_run_records/r1/b2c_statics_r1.py S/i97/statics S/i97/statics/r1 S/w2/st_X`, for X = a, b | `diff -r` identical. DEF-C r2, PTABLE r2 and `b2c_statics_r2.out.json` are `cmp`-equal to O's | 2 |
| B4 | `VENV -B S/i97/_run_records/r2/b2c_checks_r2.py S/i97/_run_records/r1/b2c_checks_r1.py S/w2/a01/rv118_a01_option_ii.py S/w2/checks_X.out.json S/w2/vectors_X.json`, for X = a, b | Both outputs are identical, and `cmp`-equal to O's `b2c_checks_r2.out.json` and `exact_norm_vectors.json` | 2 |
| B5 | `VENV -B rv118_a02_checks.py P S/w2/a01 O S/i96/statics/r1/semantic_contract_v0_3_physics_retained_1.json <out>`, for two outputs | `rv118_a02_checks.out.json`. The two outputs are byte-identical | 2 |
| B6 | Code reading with `grep -n`, `sed -n` and `awk` at NUM; `git grep -w displacement_norm` over P's maintained tree | The sites in `checks_addendum_02.txt` §3 | — |

**What the script imports.** `rv118_a02_checks.py` is standard library only. It loads A01's sealed `rv118_r1_checks.py` (JCS, H, leaf diff) and `rv118_a01_option_ii.py` (`rn64_sqrt`, `exact_square_sum`) by path from `cmp`-equal scratch copies. It imports nothing from I96, I97 or production code.

**Screen,** run on this record folder before returning:
- the strict path pattern;
- the host names: the machine's host and network names, and any laptop-model form the brief names, case-insensitive;
- the dot-local word pattern, judged by what each hit names (there is no gzipped file here);
- `find <folder> -type l`;
- `git status --ignored` on the folder.
