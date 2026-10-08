# RV115 (RV-K): execution record

One standard-library Python script of my own, run once on 2026-10-07 UTC with VENV's Python 3.13.14. It imports nothing outside the standard library and nothing from the repository. It reads one file: I94's run output, given as its argument, for a value-by-value comparison of the combined nets. It writes only to stdout.

```
cd WT/scratch/rv115_rvk
TMPDIR=WT/scratch/rv115_rvk PYTHONDONTWRITEBYTECODE=1 VENV/bin/python rv115_checks.py R/I94/b2_kd_01/_run_records/b2kd_checks.out.json > rv115_checks.out.json
```

The script exited with status 0. The script and its output were then copied unchanged into this folder (`cmp`, identical).

| File | Purpose |
|---|---|
| `rv115_checks.py` | The checks below (its docstring lists them) |
| `rv115_checks.out.json` | Its output, keyed by check number |

| Key | Check |
|---|---|
| `1_nets` | C1–C6's exact combined nets from my own operand definitions, K4LED canonical forms, limb spans, individual-product flags, and K4LED bytes and sha256 (tip node 1). These are expectations for RV-K's code round |
| `2_E5` | My own directed 1024-bit rounding. C6's endpoints and width (2^-423). A seeded random sweep of 4,010 nets of binary64 products, with 0 violations of δ ≤ 2^(⌊log2\|N\|⌋−1023) and δ = 0 exactly when the net fits 1024 bits |
| `3_products` | Every product in C1–C6 is exact in binary64. An alternative (0.1·3, 54 bits) is not |
| `4_cantilever` | Tip flexibility by curvature integration, at L = 1 and L = 2, with the coupling signs |
| `5_C3` | The operand-row sum is exactly 0 in binary64, while the truth is 2^-60/EA |
| `6_rows` | 52 native rows, 73 case final rows and 72 combination final rows for the specimen; stress slots 0–19 and maximum slot 20 |
| `7_capacity` | The `for_invocation` counterexample (SF-1), and that the sufficient check is dormant under `for_calls` |
| `8_C6_mutation` | The omitted 2^-600 against the K-lane and G-lane widths (SF-4) |
| `9_constrained_loads` | No specimen term lies at a constrained DOF (SF-2) |
| `compare_I94` | My nets and spans equal I94's on all six combinations |

No cargo, native or solver job, no install and no Git write. Git reads used `GIT_OPTIONAL_LOCKS=0`. Nothing was written to the system temp directory.
