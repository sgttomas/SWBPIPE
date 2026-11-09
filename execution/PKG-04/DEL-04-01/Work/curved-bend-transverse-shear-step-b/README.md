# Curved-bend transverse shear, Step B drafts

These are drafts of Step B, product admission of `timoshenko_annulus_energy_curved_v2`,
for [curved-bend-transverse-shear.md](../curved-bend-transverse-shear.md) §5. Nothing here is
applied, built or adopted. They are kept so the work can resume. At write time, each hunk is
re-derived on the actual base, not applied blindly.

| File | Base | Status |
|---|---|---|
| `STEP_B_DRAFT.md`, `step-b.diff` | `f18834a6b` (Step A, before pressure elongation) | The first whole-Step-B survey: 60-row surface inventory, 86 hunks, findings and open choices. It is superseded for B1 by `b1/`. It remains the reference for B2 (exact route on `pressure-3`). |
| `b1/B1.md`, `b1/b1.diff` | `3c9db2295`, a local merge of `origin/claude/pressure-elongation@55bb15bf7`, `origin/main@fcd8bb12c` and Step A `f18834a6b` | The B1 exact diff (61 files, 140 hunks), with Agent 0's decisions (a)–(h) and its three answers applied. `git apply --check` passed on that base. |
| `b1/make_b1.py` | the same base | The draft's own generator, a record of how `b1.diff` was produced. It is not maintained tooling and is not run by any check. It reads the local base path named in it. |

Agent 0's decisions, as recorded in `b1/B1.md`:
- amend the 3.0.0 vocabulary in place;
- semantic IDs `preview-physics-curved-shear-1` and `pressure-curved-shear-1` (on `pressure-3`);
- profiles `product_preview_mechanics_curved_shear_v1` and `exact_pressure_v3_curved_shear_v1`;
- B1 and B2 as separate PRs, with one pricing turn for both;
- the exact route refuses v2 in B1 with `EXACT_PRESSURE_SHEAR_MODEL_NOT_ADMITTED`;
- the exact-route applier texts stay at Euler–Bernoulli/v1 until B2.
