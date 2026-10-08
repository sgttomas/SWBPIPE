# RV115 ADDENDUM_01: execution record

Three read-only Python runs on 2026-10-07 UTC, with VENV's Python 3.13.14 and standard library only. Each ran once and exited with status 0. No repository file was written.

## 1. My own check

```
cd WT/scratch/rv115_rvk
TMPDIR=WT/scratch/rv115_rvk PYTHONDONTWRITEBYTECODE=1 VENV/bin/python rv115_addendum_checks.py > rv115_addendum_checks.out.json
```

**What it computes** for all eight oracle vectors:
- the represented-Z hull, hull(I_K/c, Ẑ);
- its 1024-bit outward tokens, from my own encoder;
- the axis-bit equality;
- that signed-corner division by the hull encloses both M/Ẑ and M·c/I_K.

It imports nothing from the repository.

## 2. The committed oracle generator, on scratch copies

FKT's `product_certificate_vectors.py` and `.rs` were copied into two scratch folders, `WT/scratch/rv115_rvk/k3_regen/orig` and `.../k3_2`.

```
cd WT/scratch/rv115_rvk/k3_regen/orig
TMPDIR=WT/scratch/rv115_rvk PYTHONDONTWRITEBYTECODE=1 VENV/bin/python product_certificate_vectors.py > verify.out.json
cd WT/scratch/rv115_rvk/k3_regen/k3_2
# the one change: drop " if mode else (F(),F())" from line 68
TMPDIR=WT/scratch/rv115_rvk PYTHONDONTWRITEBYTECODE=1 VENV/bin/python product_certificate_vectors.py --write > write.out.json
```

**The two runs:**
- **The first run** verifies the unchanged generator against the committed fixture.
- **The second run** writes `product_certificate_vectors.rs` beside the scratch copy only.

`k3_2_regeneration.txt` holds:
- both `diff` outputs (generator and fixture);
- the four sha256 values;
- both runs' printed summaries.

## 3. Files

| File | Purpose |
|---|---|
| `rv115_addendum_checks.py`, `rv115_addendum_checks.out.json` | Run 1 |
| `k3_2_regeneration.txt` | Runs 2 and 3: the exact exception (3 lines) and the new fixture sha256 `467f8811…` |

No cargo, native or solver job, no install and no Git write. Git reads used `GIT_OPTIONAL_LOCKS=0`. Nothing was written to the system temp directory.
