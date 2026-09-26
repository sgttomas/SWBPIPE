# ROOT's rulings on the V2 reference refutation

HELP_HUMAN (ROOT), 2026-09-26, relayed to the T3 manager by SendMessage and recorded here by the manager. The input is [REFERENCE_CHECK/RETURN.md](REFERENCE_CHECK/RETURN.md): REFERENCES CONFIRMED, with nothing blocking.

1. **RF-CANCEL scale.** The net-governed "recommended" column is the binding comparison scale, under the unchanged criterion `|obs − exp| ≤ 1e-9 · max(|exp|, scale)`. The package states this explicitly. R1 corrects its README claim (F3): the net scale can fall below |exp|, and where it does the comparison is exactly relative.
2. **Before freezing, R1 makes a narrow revision.** V2 backchecks only these items:
   - F1: print every model input exactly, including the 8 RF-RANGE cases and the 11 G=1e80 RF-CANCEL cases;
   - F3: correct the README;
   - F4: relabel the two NC-WRONG-TRANSFORM controls truthfully;
   - F5: correct the README's NC-ZEROED text;
   - F9: correct the RESTRAINT-COUNT wording.

   F6 to F8 are recorded as notes. **F8:** the harness treats expected values below the binary64 range as absolute comparisons, and reports them as such.
3. **F2 (SHOULD-FIX for D1 and the harness).** D1 gives twist and extension their own scale kinds, so the stop-rule guarantee covers them. Any comparison still below the floor is flagged by the harness as **not covered by the guarantee**. That includes the 3 G=1e80 RF-CANCEL rows and the RF-WEAK far and coupling regions. Neither a below-floor comparison nor a not-covered flag counts as a pass. D1 folds this into its revision, and V1's BACKCHECK_R2 checks it.

After V2's narrow backcheck, ROOT freezes the references with the selection package. The host stays held while T1 runs its full local e2e and then VP-STATIC.
