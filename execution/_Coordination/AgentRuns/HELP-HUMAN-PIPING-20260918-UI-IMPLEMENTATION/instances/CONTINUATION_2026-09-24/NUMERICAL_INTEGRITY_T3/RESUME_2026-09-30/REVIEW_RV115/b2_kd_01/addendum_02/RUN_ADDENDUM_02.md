# RV115 ADDENDUM_02: execution record

One standard-library Python script of my own, run once on 2026-10-07 UTC with VENV's Python 3.13.14. It exited with status 0.

```
cd WT/scratch/rv115_rvk
TMPDIR=WT/scratch/rv115_rvk PYTHONDONTWRITEBYTECODE=1 VENV/bin/python rv115_addendum02_checks.py \
  P/fixtures/results/retained_precision_prepared_ordinary_v1.json \
  R/I97/b2_c_01/statics/retained_precision_prepared_combination_v1.json \
  P/fixtures/results/preview_physics_unicode_ids_sparse.json > rv115_addendum02_checks.out.json
```

It reads only its three arguments. It imports nothing from the repository: the canonical form is re-implemented, and the checked-JSON executable is not called. It writes only to stdout. The script and its output were copied unchanged into `addendum_02/` (`cmp`, identical).

| Output key | Check |
|---|---|
| `1_hash` | H for DEF-O (the control, `a7ed7ca0…`), for DEF-C (`9adf5178…`) and for the alternative domain (`562cbe14…`); raw bytes equal their canonical form; the operand definition's sha256 equals H(DEF-O) |
| `2_members` | Every DEF-C leaf path changed against DEF-O; every proof-relevant member byte-identical; the excludes difference |
| `3_R7` | The fixture's combination recounted, with n, m and g from entity references, and its families, sites and unique keys; the cases' row totals for comparison |

No cargo, native job, install or Git write. Nothing was written to the system temp directory.
