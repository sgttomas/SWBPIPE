# I61 U5, addendum 01: two corrections from RV86

RETURN.md stays as written; this addendum corrects two statements in it. Both are wording only:
- no result, verdict, count or file changes;
- no rerun is needed.

The basis is RV86's review (`R/REVIEW_RV86/u5_reference_01/REVIEW.md`, findings S-1 and N-2, with evidence in its `_run_records/`), as ROOT ruled in RR "RV86 on U5: PASS, with stated limits; the milestone's reference claim scoped" (NUM `7e4f5a51dd`).

## S-1. The cause of the seven represented-readout misses is J, through k_t, not A and Z

**Replaces** the RETURN's paragraph "Information (not a class claim)", specifically "These are exactly the J-dependent rows. The successor's echoed `section_terms` are the prepared route's annulus values, which differ from the ordinary represented A and Z by 2 ulps."

**The corrected statement:**
- **What the seven rows depend on.** The stop-rule-sharp bound misses against the represented readout on N1's y and z rotations (θ_root plus twist) and on the five torsional-shear stations (T·c/J). These rows depend on **J**, through the receipt's torsional stiffness k_t = G·J/L. They depend on neither A nor Z.
- **The size of the difference** (RV86's `c3_truths.py`):
  - I50's represented J is about **4.3 ulps above** the exact annulus J;
  - the receipt's k_t is the **correctly rounded** exact G·J/L, 4 ulps below a k_t formed from I50's J.
- **The A and Z differences are irrelevant to these rows.** The receipt's A and Z are also the correctly rounded exact annulus values, 2 ulps below I50's represented ones.
- **N1 rx is also J-dependent.** J contributes only about 1.0e-4 of its value, so it shows no miss. "Exactly the J-dependent rows" is therefore too strong. The correct statement: the seven misses are J-dependent rows, and the one remaining J-dependent row (N1 rx) is not sensitive enough to miss.
- **The conclusion is unchanged.** The successor converges to the source-annulus geometry, and every class claim passes against both readouts.

## N-2. The oracle does give value checks for the mode and parity rows

**Replaces** two statements in the RETURN:
- "Mapping notes", the "Ancillary rows" item, which said "The oracle gives no reference for them, so no value claim is checked";
- the method table's `non_quantity` row ("none").

**The corrected statement:** the oracle's `check` does state value checks for these rows (oracle lines 120–129):
- the mode row's bits are 1.0 (sparse) or 2.0 (dense), with a matching basis;
- the parity row is present only in dense, and is finite and ≥ 0.

U5's slice did not apply them. **RV86 applied them, and they pass in both modes.** The rows remain `non_quantity` under the reader, with no class claim.

## For U3 grant 2's rerun (ROOT; not done here)

The rerun will make RV86's **two-line pin change** (`R/REVIEW_RV86/u5_reference_01/_run_records/u5_compare_extract_variant.diff`), so that `u5_compare.py` reads RV86's committed **7,240-byte extract**:
- the extract is `i50_oracle_inputs_extract.log`, sha256 `a66a8a49422907556660cfdb4300d587845795160d2c5d0c76e07e682ef94fda`;
- `make_extract.py` keeps the hash assertion against I50's BULK_MANIFEST entry for the full log (`5ced66b5…`);
- this removes the local-only scratch dependency.

RV86 showed the extract reproduces `u5_report.json` and `u5_run.log` byte for byte. U5's script is otherwise unchanged.
