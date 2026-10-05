# I61 RETURN: U5, the milestone's independent-reference comparison

**Status: PASS, with no stop.** In both modes, every published class claim of U1's pinned milestone successor agrees with I50's named oracle (exact rational, directed bounds):
- each mode has 25 `relative_verified`, 69 `absolute_verified` and 3 `input_derived` rows;
- the claims agree against **both** of the oracle's readouts, the source-annulus readout and the represented readout;
- the prepared route's maxima patches, overlay headlines and support rows pass the oracle's own observable checks;
- no row needed a reading;
- the negative controls are refused.

**Lane.** This is a records and test lane (PLAN §2 U5). It imports no product code, ran no Cargo job, and wrote nothing to maintained code. ROOT's grant is in the U3 grant 1b message (RR "U3 grant 1 verified; R-1, R-2 and R-3 ruled", NUM `efde9ca2d1`). Re-confirmation on the actual facade output waits for U3 grant 2.

**Run facts.**
- **Role and time:** TASK Type 2 under ROOT, with no descendants. 2026-10-04, about 06:33Z to 06:45Z, inside U3 grant 1b's session.
- **Host:** the memory guard (PID 5387) was running. Python only. The run was under a 900 s alarm while U3 grant 1b's Cargo jobs ran separately, one at a time.
- **Writes:** WT/scratch/i61_u5_reference_01/ and this folder.

## Inputs

All inputs are pinned in `_run_records/inputs_sha256.txt`.

**The successors.** These are U1's pinned files: sparse `ac6986b0…` (receipt `efc1a39b…`) and dense `6cd1d249…` (receipt `3e26499f…`).
- PP's committed `u3_permitted_path_publishes_the_pinned_successor` wrote them through the private driver.
- They are **byte-identical** to what the actual `run_linear_static_preview_value_with_retained_direct` publishes behind the disposable stub (`R/I61/u3_facade_02`, `u3_stub_dispatch_successor_*.json`; the same hashes).

**The oracle.** `R/I50/first_publishing_component_02/named_oracle.py`, sha256 `b1b58639…`. The script asserts this hash, which equals I50's SEAL entry.

**I50's captured records.** `WT/scratch/i50_first_publishing/runtime02/pp_debug_final.log`, sha256 `5ced66b5…`. The script asserts this hash, which equals I50's BULK_MANIFEST entry.
- These supply the oracle's inputs: the request, the source map and the represented section facts.
- The script asserts that each successor's own invocation request **equals** I50's named request, so the oracle's derivation applies to it.

**The published classes.** These come from the **accepted, unchanged** Python reader, `P/core/analysis_runs/retained_precision.py` at `bee3dc07ca` (sha `d77008e2…`, identical to U1's reader lane).
- The reader supplies each row's normalized bits, S\*, class and absolute bound.
- The public entry refuses at G0 until U7 sets `_IMPLEMENTATION_COMPLETE`, so the lane calls its ordered checks, `_validate_draft`, with eligibility off, as U1's lane did. The reader reports invocation bound, standing `needs_recompute` and not eligible in both modes.
- The checked-JSON and units authorities are I52's prebuilt binaries (hashes recorded), which experiment 03 also used.

## Method (`_run_records/u5_compare.py`)

**What it reuses from the oracle, by exact text slice of the pinned file:**
1. The oracle's module helpers, unchanged.
2. `check`'s whole input derivation, through the nested `truth(r, geometric)`. This covers:
   - the frame and orthonormality;
   - the loads, springs and supports;
   - the materials;
   - static equilibrium;
   - relative torsion;
   - the section bracket with rational π bounds.

   The slice returns `truth` instead of running I50's private-verdict comparison, so every one of the oracle's input assertions runs.
3. `check`'s closing observable checks, unchanged, run on the successor envelope:
   - the support-norm guards;
   - the extrema midpoint identity;
   - the headline identities.

**The per-row check.** For each published row:
- the normalized value must equal the reader's `normalized_bits`;
- every absolute bound must equal the receipt's `absolute_verified` entry.

Each class claim is then checked against each readout separately:

| Class | Claim checked | Allowance |
|---|---|---|
| `relative_verified` | relative 1e-9 on the published value (D1 §4.1.6, "Relative 1e-9 on a published q") | \|n − t\| ≤ \|n\|/10⁹ in SI, and the same in the published unit (the oracle's DecimalSi and DecimalRaw) |
| `absolute_verified` | the published bound | \|n − t\| ≤ b, with b = the receipt's bound bits |
| `input_derived` | exact | n = t |
| `non_quantity` | none | not a quantity: the mode row, and the dense parity row |

**For information only, not a class claim:** the stop rule's bound on the published S\* plus the publication rounding (the oracle's SharperExact).

**Verdicts** are pass, fail or unproved by the oracle's directed intervals. Any unproved verdict, any unmapped kind, any `not_covered` row or any non-pass would be a stop.

## Results (`_run_records/u5_report.json`, `u5_run.log`)

**Both modes**, each against the source-annulus readout and the represented readout:

| Mode | Rows | Class claims checked | Relative 1e-9 (SI and unit) | Absolute bound | Input exact | Unproved | Observable checks |
|---|---:|---:|---:|---:|---:|---:|---|
| sparse_interactive | 98 (1 non_quantity) | 97 | 25/25 pass | 69/69 pass | 3/3 pass | 0 | pass |
| dense_scrutiny | 99 (2 non_quantity) | 97 | 25/25 pass | 69/69 pass | 3/3 pass | 0 | pass |

**Which rows fall in each class:**
- **relative_verified (25):**
  - the N1 displacement magnitude;
  - N0's rotations (x, y, z);
  - N1's translations (y, z) and rotations (x, y, z);
  - the five torsional-moment stations and the five torsional-shear stations;
  - each spring's own moment component (Mx, My, Mz) and its moment magnitude.
- **absolute_verified (69):** the structural zeros. These are:
  - the axial, shear and bending actions and stresses;
  - N1's x translation;
  - N0's displacement magnitude;
  - the rigid support's six components and two magnitudes;
  - each spring's force components, its two off-axis moment components and its force magnitude;
  - the normal-stress maxima.
- **input_derived (3):** N0's prescribed translations.

**Information (not a class claim).** The stop-rule-sharp bound passes for all 25 relative rows against the **source-annulus** readout.
- Against the **represented** readout, it misses on 7 rows, the same in both modes:
  - N1's y and z rotations;
  - the five torsional-shear stations.
- These are exactly the J-dependent rows. The successor's echoed `section_terms` are the prepared route's annulus values, which differ from the ordinary represented A and Z by 2 ulps.
- So the successor converges to the source-annulus geometry, which is what D1's prepared formation states. I50's dual-readout obstruction, at the ordinary route's scales, is not a defect of the successor.
- This is reported, not used: the class claims pass against both readouts.

**Negative controls (non-vacuity), both modes:**
- `result:disp:N1` moved to 2e-9 relative is refused by the 1e-9 test;
- `result:disp:N0` moved past twice its bound is refused by the bound test.

## Mapping notes (no reading was needed)

- **Support rows.** The components and the force and moment magnitudes, rigid and the three springs, map through the oracle's own `truth`, by kind, entity and component. The oracle's norm guard also passes.
- **Maxima.**
  - The `pipe_elastic_normal_stress_maximum_v2` rows map through the oracle's `truth`; they are 0 under pure torsion. They are `absolute_verified` and within their bounds.
  - The overlay's `pipe_stress_extrema` patches pass the oracle's midpoint identity, in which each maxima row equals the midpoint of its published lower and upper values.
  - I did not assert that the extrema interval encloses the truth. The oracle does not state that check, and asserting it would be a reading.
- **Overlay headlines.** `summary.max_displacement` and `summary.max_open_formula_stress` pass the oracle's headline identities.
- **Ancillary rows.** The mode row and the dense parity row are `non_quantity` under the reader. The oracle gives no reference for them, so no value claim is checked.

## Still open

- **Re-confirm at U3 grant 2.** Run the same script on the bytes the committed facade entry publishes under the real permit.
- **The script is reusable unchanged.** It takes the successor files as arguments.

## Records

| File | Content |
|---|---|
| `_run_records/u5_compare.py` | the comparison script |
| `_run_records/run_u5.sh` | the invocation, with placeholders |
| `_run_records/inputs_sha256.txt` | input pins |
| `_run_records/u5_report.json` | every row: truths as exact fractions, error bounds, allowances and verdicts |
| `_run_records/u5_run.log` | the run log |
| `SHA256SUMS` | file hashes |

All paths are placeholders, with no machine paths.
