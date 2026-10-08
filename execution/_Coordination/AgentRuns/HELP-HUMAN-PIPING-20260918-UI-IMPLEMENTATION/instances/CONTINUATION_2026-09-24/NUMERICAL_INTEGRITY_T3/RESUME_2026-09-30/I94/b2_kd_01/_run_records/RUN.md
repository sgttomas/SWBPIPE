# I94 B2-KD: execution record

One standard-library Python check, run once, 2026-10-07 UTC, with VENV's Python 3.13.14. It reads and writes no repository file; it imports nothing outside the standard library.

```
cd WT/scratch/i94_b2_kd
TMPDIR=WT/scratch/i94_b2_kd PYTHONDONTWRITEBYTECODE=1 VENV/bin/python b2kd_checks.py > b2kd_checks.out.json
```

Exit status 0. The script and its output were then copied unchanged into this folder.

| File | Purpose |
|---|---|
| `b2kd_checks.py` | Exact combined nets and bit spans for DESIGN.md §6's specimens C1–C6; the cantilever tip coupling signs from an exact Fraction solve of a textbook frame element; C3's operand-row discriminator |
| `b2kd_checks.out.json` | Its output |

No cargo, vitest, native or solver job, no install and no Git write. Git reads used `GIT_OPTIONAL_LOCKS=0`. Nothing was written to the system temp directory.
