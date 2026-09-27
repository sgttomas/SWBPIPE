# T3 V2: backcheck of R1 revision 2

V2's narrow backcheck of R1's revision 2, 2026-09-26. Revision 2 is commit `c0f14201c` on `codex/piping-numerical-integrity-20260926`. The manager asked for this check, and its scope follows `T3/ROOT_RULINGS_V2.md` §2: only findings F1, F3, F4, F5 and F9, plus a no-regression check. The independence rules are the same as in [RETURN.md](RETURN.md): no product source, standard-library Python only, and every long run under `nice -n 19`, single-threaded.

Inputs:

| File | sha256 |
|---|---|
| `T3/REFERENCES/references.py` | `80d473a7…` |
| `T3/REFERENCES/references.json` | `7b176dbb…` |
| `T3/REFERENCES/README.md` | `5a89bad9…` |

`_run_records/SHA256SUMS` verified OK for all 8 entries, including `SHA256SUMS.revision1`. The revision-1 `references.json` (`57e3254e…`) was taken from `6c448d260` with `git show`.

## Verdict: CONFIRMED

F1, F3, F4, F5 and F9 are resolved, and nothing has regressed.

- **Exact inputs (F1).** My independent direct-stiffness derivation now reproduces every published value from the inputs **as printed**, to within 5e-40 of `max(|exp|, scale)` (27,893 of 27,893).
- **No regression.** The structural diff against revision 1 changes nothing outside the stated scope.
- **Regeneration.** `references.py` regenerates `references.json` byte for byte (695 s, sha256 `7b176dbb…`).

Two wording remnants of F3 are left. Both are NOTEs and neither blocks (N1).

## 1. F1: exact model inputs. Resolved

`bc_diff.py` compares revision 2 with revision 1 in structure:

- 60 model-input strings changed. All of them are in exactly the 19 named cases:
  - RF-RANGE: CHAIN-LEF-large, CHAIN-SIM-a, CHAIN-SIM-b, CONT-LEF-large, CONT-SIM-b, SKEW-LEF-large, SKEW-SIM-a and SKEW-SIM-b;
  - RF-CANCEL: the ten G = 1e80 cases and UDL-W1e80.
- Each changed string moved by at most 2.7e-28 relative. That is the size of the old 28-digit rounding.
- G and w are now printed `1e+80`, which is exactly 10^80. My revision-1 reconstruction (`v2_trunc.py`) had found 10^80, not float(1e80), to be the value R1 used.
- All 33,595 numeric model-input strings in revision 2 parse. The 24 mantissas with 28 or more significant digits are exact long integers or terminating decimals (the SIM-b coordinates and OD/ID), not roundings.

The strongest evidence is the re-derivation. `v2_derive.py` reads only model inputs, and I ran it unchanged on the revision-2 JSON. `v2_compare.py` then gives:

| Family | Values | ≤ 5e-40 | Max normalized difference (revision 1 → revision 2) |
|---|---|---|---|
| RF-RANGE | 2720 | 2720 | 1.7e-27 → 4.68e-40 |
| RF-CANCEL | 1585 | 1585 | 3.6e-29 → 3.23e-40 |
| All ten families | 27893 | 27893 | ≤ 4.98e-40 |

The results are in `_run_records/backcheck/bc_compare.stdout.txt` and `bc_compare.json`. Everything else still agrees:

- every class scale;
- both represented-input bases (31/31 and 142/142 values);
- every RF-CANCEL recommended scale, gross scale and `governed_by` label (1585/1585 each);
- all 161 RF-CANCEL control flags;
- the 525 listed-value checks on discriminating controls.

## 2. F3: RF-CANCEL scale statements. Resolved, with the remnants in N1

- **JSON `criterion`.** It now reads: "scale = the published class scale of the value, except in RF-CANCEL, where the binding scale is the recommended (net-governed) column of each value row (ROOT ruling on V2 …); unchanged form".
- **All 41 `cancellation.recommended_scale` texts.** Each contains "BINDING scale for this case", "It never exceeds the class scale", "It can fall below |exp| (in mixed rows); there the comparison … is exactly relative" and "never below |exp|".
- **Numbers behind the text.** Recomputed from the revision-2 rows: the recommended scale is below |exp| in exactly 84 rows, all of them `mixed`, and above the class scale in 0 rows.
- **README.** §0, §6 (Scales) and §7 (RF-CANCEL scale choice) now carry the corrected claim and the binding ruling.

**N1 (NOTE).** Two general sentences still name only the class scale, with no RF-CANCEL exception:

- README line 9: "The `scale` is the published class scale of each value. See §4."
- JSON `conventions.classes`: "Every published value is compared with the scale of its class."

The JSON `criterion` header and every RF-CANCEL case text state the exception, so a reader of the package is not misled. A one-clause addition to each sentence would make them consistent. This does not block freezing.

## 3. F4: NC-WRONG-TRANSFORM in RF-WEAK-W-L. Resolved

All three W-L controls now carry:

- `defect_form`: "flexibility (constitutive frame of M2 in tree integration); rigid-body kinematics exact";
- `other_forms`, which cites V2's direct-stiffness form: "fails by about 1e9 at th.N2.RX. Not computed by this script".

The description says the `discriminates` flag refers to the flexibility form only. The flags are unchanged: r1e-04 true (9.99e3), r1e-08 and r1e-12 false.

I recomputed my direct-stiffness form against revision 2 (`bc_nc.json`). It fails by 1.0e9 in all three cases, and `th.N2.RX` fails by 1.0e9 in each; at r1e-08 the largest violation is at `tw.M1`, also 1.0e9. The relabelling is therefore truthful. Of the 232 recomputed controls, the only disagreements are these two form-specific labels, now documented.

## 4. F5 and F9. Resolved

**F5, the README's RF-ZERO NC-ZEROED text.** It now says the control reports `false`, correctly, because N.M1 and T.M1 (1e-3) would be caught by a naive absolute floor. It says the naive-floor point is made by the displacement values alone, and that no displacement-only control is included.

- Checked: a zero for u_x = 1.675e-12 m passes |0 − exp| ≤ 1e-9 absolute, but fails 1e-9·max(|exp|, 1.675e-12) = 1.7e-21. The same holds for θ_x.
- The text matches the JSON (`naive_absolute_1e-9_floor_accepts_this_defect: false`).

**F9, NC-RESTRAINT-COUNT.** The wording is now case-specific:

- **DISC-CHAIN100:** "only the global form … a per-body count finds no restraint on the sub-assembly".
- **DISC-CHAIN100-SPRING:** "… finds one restraint (the spring)".
- **LINE122 ×3, LINE345-RZ, K0 and LINE-IN-CHAIN1000:** "both forms of the count are defeated". Checked against the models: the per-body restraint counts are 33, 44, 6 (5 rigid plus the k = 0 spring) and 18 on the line sub-body, all ≥ 6.
- One sub-nuance for K0: a count that ignores zero-stiffness springs sees 5 and refuses. That is a correct refusal either way, and it needs no change.

## 5. No regression

`bc_diff.py` walks both JSON trees. There are 141 differences, and every one falls in the scope R1 stated (0 outside it):

| Path class | Count |
|---|---|
| model input strings | 60 |
| `purpose` | 11 |
| `cancellation.gross_scale` texts (these quote G) | 10 |
| `cancellation.recommended_scale` texts | 41 |
| NC `description` (3 W-L, 8 RF-MECH) | 11 |
| `defect_form` and `other_forms` (added) | 3 + 3 |
| `criterion` | 1 |
| `status` | 1 |

No value changed under `expected`, `expected_represented`, `scales`, `counts`, `basis`, `finite_input`, `regions`, `range`, `null_motions`, `nullity`, or any negative-control result field (`discriminates*`, `violating_values`, `max_violation_ratio`, `at`, `values`, `net_used`).

**Regeneration.** `references.py` (`80d473a7…`) was copied unchanged to scratch and run with `nice -n 19 python3 references.py`: 695 s, exit 0, empty stderr. The output has sha256 `7b176dbbf2296be02d8bca19c698ee56d0d5753751150a9175e0d3ad4cf89cc9`. `cmp` finds it identical to the committed file, and its stdout is identical to `_run_records/references.stdout.txt`. The record is in `_run_records/backcheck/regeneration.txt`.

## 6. Not checked

- F6 to F8 and F2. They are out of scope per ROOT_RULINGS_V2 §2–3; F2 goes to V1's BACKCHECK_R2.
- R1's revised self-check. I did not read it.
- I read no product source, ran no cargo, npm or product tests, and made no Git write.
- I wrote only `REFERENCE_CHECK/BACKCHECK.md`, `REFERENCE_CHECK/_run_records/backcheck/**` and `REFERENCE_CHECK/SHA256SUMS`.

## 7. Record

**Read:**

- `T3/ROOT_RULINGS_V2.md`
- the revision-2 README diff against `6c448d260`
- `references.json` (revision 2), and revision 1 from git
- the revision-2 `references.py`, run only; I did not read its source

**Ran:**

- `_run_records/backcheck/bc_diff.py`: the structural diff and the F1, F3, F4 and F9 texts.
- The existing `v2_derive.py`, `v2_compare.py`, `v2_cancel.py` and `v2_nc.py`, unchanged, on the revision-2 JSON: derivation 83 s, then about 20 s for the rest.
- The regeneration.

**Outputs** are in `_run_records/backcheck/`: `bc_diff.json` and `.stdout.txt`, `bc_derive.stdout.txt`, `bc_compare.json` and `.stdout.txt`, `bc_cancel.json` and `.stdout.txt`, `bc_nc.json` and `.stdout.txt`, and `regeneration.txt`. The hashes are in `REFERENCE_CHECK/SHA256SUMS`.
