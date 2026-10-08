# RV118 (RV-C): execution record

Placeholders as in REVIEW.md: WT, NUM, P (NUM's `projects/chirality-piping`), R, VENV. `S` = my scratch folder `WT/scratch/rv118_rvc` (disposable). TMPDIR was `S/tmp`.

**Environment.** Python 3.13.14 from VENV; `jsonschema` 4.26.0 (VENV's). Every Python run had `PYTHONDONTWRITEBYTECODE=1` and `-B`. Git reads used `GIT_OPTIONAL_LOCKS=0`. No cargo, native, solver or test job, no install, no Git write. Nothing was written outside `S` and this record folder.

**NUM.** HEAD was `96e52b5e62` when I started and `670a5144dd` when I finished: ROOT committed records only in between (RV115's ADDENDUM_02, RR, the work graph). At both heads the tree outside `P/execution` equals the dispatch commit `6aa2878872`'s (`git diff --quiet`), and differs from main `54f1ba1f6d` only by SI1c's five rules-crate files, which this review does not read. B1 read-only: `b1` `603e238517`, `b1-r` `b5cb7faaeb`, `b1-p` `11cc14e3e6`, `b1-t` `7e47e51b5d`, all clean.

| Step | Command | Output | Runs |
|---|---|---|---|
| E1 | `shasum -a 256` of the brief, CONTRACT.md and the basis records; `shasum -a 256 -c` of I97's `SHA256SUMS` and I96's `SHA256SUMS.revision_01` | `checks.txt` §1 | 1 |
| E2 | Copy I96's `b3d_statics.py`, `revision_01/b3d_statics_r1.py` and `statics/r1/*` into `S/i96/` (same relative layout), and I97's three scripts into `S/i97/`; `cmp` each copy with its committed file | equal | 1 |
| E3 | `VENV -B S/i97/b2c_statics.py NUM/P S/i96 S/statics_X` for X = a, b | six files each; `diff -r` identical; hashes in `checks.txt` §2 | 2 |
| E4 | `VENV -B S/i97/b2c_checks.py NUM/P NUM/P/core/analysis_runs/retained_precision.py S/statics_X/retained_precision_mp_v2.schema.json S/checks_X.out.json` | `cmp`-equal to each other and to I97's `_run_records/b2c_checks.out.json` | 2 |
| E5 | `sh S/i97/b2c_collisions.sh NUM` | every line after the header equal to I97's `b2c_collisions.out.txt` (the header names NUM's HEAD) | 1 |
| E6 | `VENV -B evidence/rv118_hashes.py NUM/P <statics> S/i96/statics/r1/semantic_contract_v0_3_physics_retained_1.json <out>`, once with `<statics>` = I97's committed `statics/` and once with `S/statics_a` | `rv118_hashes.out.json`; the two outputs are byte-identical | 2 |
| E7 | `VENV -B evidence/rv118_instances.py NUM/P NUM/P/core/analysis_runs/retained_precision.py S/statics_X/retained_precision_mp_v2.schema.json <out>` for X = a, b | `rv118_instances.out.json`; the two outputs are byte-identical | 2 |
| E8 | `patch -s -o <out> <copy of main's SCHEMA> < <diff>` in `S/work/patch/` for I97's `SCHEMA_J1.diff` and `SCHEMA_B2.diff` and I96's r1 `SCHEMA_ENUM.diff`; then `patch --dry-run` of `SCHEMA_ENUM.diff` on the B2-only text | `checks.txt` §3 | 1 |
| E9 | Code reading with `grep -n`, `sed -n` and `git grep` at NUM and on the four B1 worktrees | the sites in `checks.txt` §4 | — |

**What the scripts import.** `rv118_hashes.py` is standard library only; its JCS canonicalizer is written from RFC 8785 for the value classes these statics contain and imports nothing from I96, I97 or production. `rv118_instances.py` imports `jsonschema` and loads NUM's committed PY reader file by path, only to call its G1 walker `_shape` with its schema loader replaced in memory, as I97's checks do; it writes nothing into the repository.

**A correction during the run.** My first `rv118_instances.py` run built two `LedgerError` instances with a bare string where SCHEMA's `SumError` is an object `{tag}`; both validators refused them, as they should. I corrected my instances and re-ran (E7's two runs are after the correction). A leftover no-op expression in the body-level check was also removed; the output was byte-identical before and after.

**Screen** (run on this folder before returning): the strict path pattern and the machine's host name over every file (no gzipped file here); `find <folder> -type l`; `git status --ignored` on the folder.
