# I97 B2-C revision 01: execution record

Placeholders are as in CONTRACT.md: WT, NUM, P (NUM's `projects/chirality-piping`), R and VENV. Two more:
- `OUT` = `R/I97/b2_c_01`;
- `S` = my scratch folder `WT/scratch/i97_b2_c/work/r1` (disposable).

**Environment:**
- Python 3.13.14 from VENV, with VENV's `jsonschema` 4.26.0.
- Every Python run used `-B`, `PYTHONDONTWRITEBYTECODE=1` and `TMPDIR=WT/scratch/i97_b2_c/tmp`, and Git reads used `GIT_OPTIONAL_LOCKS=0`.
- No `__pycache__` was written into OUT, I96's record folder or NUM's code (checked with `find`).
- NUM was at `ba7bbae589`. Its maintained tree (`P/fixtures`, `P/schemas`, `P/core`, `P/tests`, `P/apps`) equals `cebff253d6`'s, CONTRACT.md's basis (`git diff --quiet`).
- Nothing was written outside the scratch folder, `OUT/statics/r1/`, `OUT/_run_records/r1/`, `OUT/REVISION_01.md` and `OUT/SHA256SUMS.revision_01`.

| Step | Command (in `S`) | Output | Runs |
|---|---|---|---|
| S1r1 | `VENV -B b2c_statics_r1.py NUM/P R/I96/b3_d_01 OUT/_run_records/b2c_statics.py OUT/statics out_a` | `out_a/`: DEF-C r1, PTABLE r1, `b2c_statics_r1.out.json` | 2 (`out_a`, `out_b`): byte-identical (`diff -r`) |
| S2r1 | `VENV -B b2c_checks_r1.py NUM/P NUM/P/core/analysis_runs/retained_precision.py OUT/statics/retained_precision_mp_v2.schema.json out_a/semantic_contract_v0_3_preview_physics_retained_1.json OUT/_run_records/b2c_checks.py checks_a.out.json` | `checks_a.out.json` | 2 (`checks_a`, `checks_b`, the second with `out_b`'s PTABLE): byte-identical (`cmp`) |
| S3r1 | `sh b2c_collisions_r1.sh NUM` | `coll_a.out.txt` | 2: identical (`cmp`) |
| Control | `VENV -B OUT/_run_records/b2c_checks.py NUM/P NUM/P/core/analysis_runs/retained_precision.py OUT/statics/retained_precision_mp_v2.schema.json v0checks_rerun.out.json` | v0's checks at the current NUM | 1: equal to the sealed `OUT/_run_records/b2c_checks.out.json` (`cmp`) |

**What the scripts load:**
- **`b2c_statics_r1.py`** imports the sealed v0 generator `OUT/_run_records/b2c_statics.py`, and I96's `b3d_statics.py` and `b3d_statics_r1.py`, all unchanged and by path. Its controls require v0's functions to reproduce all five sealed v0 statics byte for byte before it applies revision 01's changes.
- **`b2c_checks_r1.py`** imports the sealed v0 `b2c_checks.py` (for its `Judge`) and NUM's committed PY reader (for `_shape`), by path. It writes only its output file.

**Placed in the record** (copies of the `_a` outputs, `cmp`-equal):
- `OUT/statics/r1/`: `retained_precision_prepared_combination_v1.json` and `semantic_contract_v0_3_preview_physics_retained_1.json`;
- here: the three scripts, `b2c_statics_r1.out.json` (S1r1), `b2c_checks_r1.out.json` (S2r1), `b2c_collisions_r1.out.txt` (S3r1) and this file.

**Screens** were run on OUT before returning:
- the brief's strict path pattern;
- this machine's host name;
- the dot-local word pattern, judged by what each hit names;
- `find -type l`;
- `git status --ignored`.

The only dot-local hits are SCHEMA's own `$id` line, in the sealed v0 `statics/` (the merged SCHEMA text and the two diffs). That `$id` is main's schema identifier, not a machine name. Revision 01 adds no hit.
