# I96 B3-D revision 01: execution record

Placeholders: WT, NUM, P (= NUM's `projects/chirality-piping`), R, VENV as in the dispatch. Scratch: `WT/scratch/i96_b3_d/` (disposable). Python 3.13 from VENV; `PYTHONDONTWRITEBYTECODE=1`, `TMPDIR=WT/scratch/i96_b3_d`. Nothing was written outside the scratch folder and this record folder.

| Step | Command (in `WT/scratch/i96_b3_d`, beside the unchanged `b3d_statics.py`) | Output | Runs |
|---|---|---|---|
| R1 | `VENV/bin/python b3d_statics_r1.py NUM/P out_r1` | `out_r1/`: the definition, the table, `SCHEMA_ENUM.diff`, `CARRIER_PROFILE_ENUMS.diff`, the scratch-only SCHEMA copy, `b3d_statics_r1.out.json` | 2 (`out_r1`, `out_r1b`), all six outputs byte-identical |
| R1′ | `VENV/bin/python b3d_statics_r1.py NUM/P R/I96/b3_d_01/statics/r1`: the same generator, its output pointed at the record | the four statics byte-identical to R1's. The two non-static outputs, the scratch-only SCHEMA copy and `b3d_statics_r1.out.json`, were byte-identical to R1's and moved back to scratch, so `statics/r1/` holds only the four statics. This folder's out file is R1's | 1 |
| R2 | `VENV/bin/python -c "import json, jsonschema; jsonschema.Draft202012Validator.check_schema(json.load(open('out_r1/retained_precision_mp_v2.schema.b3b.json')))"` | none on success (the revised draft SCHEMA is a valid 2020-12 schema) | 1 |
| R3 | `GIT_OPTIONAL_LOCKS=0 git -C NUM grep -F -l -e <name> HEAD -- P ':!P/execution'` for `nonempty_pressure_regions` and the three new hash prefixes | 0 maintained hits each (`nonempty_pressure_regions`: 2 hits in `P/execution`, RV116's review) | 1 |
| R4 | `GIT_OPTIONAL_LOCKS=0 git -C NUM diff --quiet 2007709549 HEAD -- P/core P/fixtures P/schemas P/apps P/tests` | exit 0 (equal) at NUM `d867caa1ee` | 1 |

**Placed in the record:**
- the revised definition, table, `SCHEMA_ENUM.diff` and `CARRIER_PROFILE_ENUMS.diff` in `statics/r1/` (R1′);
- the v0 drafts unchanged at their original `statics/` paths;
- `b3d_statics_r1.py` and `b3d_statics_r1.out.json` here.

**Reproduce:** put `b3d_statics.py` (`_run_records/`, unchanged) and `b3d_statics_r1.py` (here) in one folder and run R1 on a tree whose `P/fixtures` and `P/schemas` equal main `2007709549`. The run is deterministic. Its `controls` member re-derives DEF-O's pinned H and the v0 draft's raw sha256 and H.

No cargo, native, solver or test job; no install; no Git write.
