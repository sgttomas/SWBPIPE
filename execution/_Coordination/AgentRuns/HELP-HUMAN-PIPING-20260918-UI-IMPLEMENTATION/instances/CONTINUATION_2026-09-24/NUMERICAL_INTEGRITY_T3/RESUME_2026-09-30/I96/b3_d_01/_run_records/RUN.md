# I96 B3-D: execution record

Placeholders: WT, NUM, P (= NUM's `projects/chirality-piping`), R, VENV as in the dispatch. Scratch: `WT/scratch/i96_b3_d/` (disposable). Python 3.13 from VENV, standard library only, except step 2b (VENV's installed `jsonschema`). Environment for every Python run: `PYTHONDONTWRITEBYTECODE=1`, `TMPDIR=WT/scratch/i96_b3_d`. Nothing was written outside the scratch folder and this record folder.

| Step | Command (run in `WT/scratch/i96_b3_d`) | Output | Runs |
|---|---|---|---|
| 1 | `VENV/bin/python b3d_numeric_checks.py NUM/P b3d_numeric_checks.out.json` | `b3d_numeric_checks.out.json` (Ĝ check: 250,011 pairs; section bits: 18 published sections and 2,000 random ones) | 1 |
| 2 | `VENV/bin/python b3d_statics.py NUM/P out` | `out/retained_precision_prepared_exact_v1.json`, `out/semantic_contract_v0_3_physics_retained_1.json`, `out/retained_precision_mp_v2.schema.b3b.json`, `out/SCHEMA_ENUM.diff`, `out/b3d_statics.out.json` | 2, byte-identical outputs |
| 3 | `sh b3d_collisions.sh NUM > b3d_collisions.out.raw.txt` | the raw log; copied here as `b3d_collisions.out.txt` with `projects/chirality-piping/` written as `P/` (`sed 's#projects/chirality-piping/#P/#g'`) | 1 |
| 2b | `VENV/bin/python -c "import json, jsonschema; jsonschema.Draft202012Validator.check_schema(json.load(open('out/retained_precision_mp_v2.schema.b3b.json')))"` | prints nothing on success (the draft SCHEMA is a valid 2020-12 schema) | 1 |
| 4 | `GIT_OPTIONAL_LOCKS=0 git -C NUM show codex/piping-t3-b1-20261007:P/core/product_physics/src/lib.rs > b1_lib.rs` | a read copy of `b1`'s `lib.rs` (not copied to the record) | 1 |

**Copied into the record:**
- `statics/`: the definition draft, the table draft, `SCHEMA_ENUM.diff` (all from step 2's `out/`);
- `_run_records/`: the three scripts and their outputs.

The scratch-only `retained_precision_mp_v2.schema.b3b.json` is the SCHEMA with B3b's change alone; its sha256 is in `b3d_statics.out.json`.

**Git reads** used `GIT_OPTIONAL_LOCKS=0`: `status`, `log`, `rev-parse`, `diff --quiet`, `show`, `grep`. No Git write. No cargo, native, solver or test job; no install.

**Reproduce:** rerun steps 1–3 on a tree whose `P/fixtures`, `P/schemas` and `P/core` equal main `2007709549`. Steps 1 and 2 are deterministic (seeded random draws). Step 3's counts depend on the HEAD read; the maintained-tree counts depend only on main's tree.
