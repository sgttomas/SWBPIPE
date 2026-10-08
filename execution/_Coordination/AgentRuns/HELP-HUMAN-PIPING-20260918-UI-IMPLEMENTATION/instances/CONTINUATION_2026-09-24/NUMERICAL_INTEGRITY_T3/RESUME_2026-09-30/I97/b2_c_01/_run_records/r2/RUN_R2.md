# I97 B2-C revision 02: execution record

Placeholders are as in CONTRACT.md: WT, NUM, P (NUM's `projects/chirality-piping`), R and VENV. Two more:
- `OUT` = `R/I97/b2_c_01`;
- `S` = my scratch folder `WT/scratch/i97_b2_c/work/r2` (disposable).

**Environment:**
- VENV's Python 3.13.14 only. The host's `python3` was not used.
- Every run used `-B`, `PYTHONDONTWRITEBYTECODE=1` and `TMPDIR=WT/scratch/i97_b2_c/tmp`, and Git reads used `GIT_OPTIONAL_LOCKS=0`.
- No `__pycache__` was written into OUT, I96's record folder or RV118's (checked with `find`).
- NUM was at `592f487abe`. Its maintained tree (`P/fixtures`, `P/schemas`, `P/core`, `P/tests`, `P/apps`) equals `cebff253d6`'s, CONTRACT.md's basis (`git diff --quiet`).
- Nothing was written outside the scratch folder, `OUT/statics/r2/`, `OUT/_run_records/r2/`, `OUT/REVISION_02.md` and `OUT/SHA256SUMS.revision_02`.

| Step | Command (in `S`) | Output | Runs |
|---|---|---|---|
| S1r2 | `VENV -B b2c_statics_r2.py NUM/P R/I96/b3_d_01 OUT/_run_records/b2c_statics.py OUT/_run_records/r1/b2c_statics_r1.py OUT/statics OUT/statics/r1 out_a` | `out_a/`: DEF-C r2, PTABLE r2, `b2c_statics_r2.out.json` | 2 (`out_a`, `out_b`): byte-identical (`diff -r`) |
| S2r2 | `VENV -B b2c_checks_r2.py OUT/_run_records/r1/b2c_checks_r1.py R/REVIEW_RV118/b2_c_01/addendum_01/rv118_a01_option_ii.py checks_a.out.json vectors_a.json` | `checks_a.out.json`, `vectors_a.json` | 2 (`_a`, `_b`): both files byte-identical (`cmp`) |

**What the scripts load, unchanged and by path:**
- **`b2c_statics_r2.py`:** the sealed v0 `OUT/_run_records/b2c_statics.py`, the sealed r1 `OUT/_run_records/r1/b2c_statics_r1.py`, and I96's `b3d_statics.py` and `b3d_statics_r1.py`. Its controls require DEF-C r1, PTABLE r1 (`OUT/statics/r1/`) and the v0 J1 SCHEMA (`OUT/statics/`) to be rebuilt byte for byte before anything is written.
- **`b2c_checks_r2.py`:** my sealed `OUT/_run_records/r1/b2c_checks_r1.py` (the layout model) and RV118's sealed `rv118_a01_option_ii.py` (its `rn64_sqrt`, `exact_square_sum` and `tie_example`, for the cross-check).
  - It replays RV115's ADDENDUM_03 generator from its seed: the code is copied with attribution, not imported, because that script runs at import.
  - It writes only its two output files.

**Placed in the record** (copies of the `_a` outputs, `cmp`-equal):
- `OUT/statics/r2/`: `retained_precision_prepared_combination_v1.json` and `semantic_contract_v0_3_preview_physics_retained_1.json`;
- here: the two scripts, `b2c_statics_r2.out.json` (S1r2), `b2c_checks_r2.out.json` (S2r2), `exact_norm_vectors.json` (S2r2's vectors file) and this file.

**Screens** were run on OUT before returning:
- the brief's strict path pattern;
- the brief's host-name screen: the machine's host and network names, and any laptop-model form the brief names, case-insensitive;
- the dot-local word pattern, judged by what each hit names;
- `find -type l`;
- `git status --ignored`.

The only dot-local hits are SCHEMA's own `$id` line, in the sealed v0 `statics/`. Revision 02 adds no hit.
