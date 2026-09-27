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

## Confirmation (ROOT, crossing message)

- The RF-CANCEL ruling (1) is confirmed.
- **P1 may start on the current references** once the host is released. It takes exact inputs from `references.py`, not from the printed JSON, and records the references commit it ran against. After R1's F1 revision, P1's mismatches are re-evaluated against the frozen references before any finding is final.
- **F2:** D1's erratum or revision 3 goes to V1's BACKCHECK_R2. Twist and extension get their own kinds, and anything left below the floor is flagged not covered and never counts as a pass.

## P1 early report on RF-CANCEL (ROOT, 2026-09-26)

- **Confirmed: no reopen; the S11 no-interim ruling stands.** P1 finds Passed, Current-eligible breaches on main in RF-CANCEL and S11 probe-A cases at gross/net ≥ about 3e7 on a single DOF: ratios 2.48 at G = 1e7, and 9.93 to 22.4 at 1e8. The G = 1e5 and 1e6 cases, and the cancel-first order, pass. These are exactly the predicted S11 class, on R1's synthetic ratios, and every error is within the adopted bound of about one rounding step of the gross term.
- **Recorded as confirmation** that main publishes Passed and Current-eligible breaches at those ratios. S11-F's urgency is on record.
- **Note for S11-F's reviewer.** In RF-CANCEL-UDL-W1e8, main's shared-node rotation misses by a ratio of 46.5, worse than both of R1's modelled fold controls (1.0 and 2.5). So main folds element-load equivalents in a different order from R1's models. S11-F's tests compare against the exact net, so the repair does not depend on the fold order.
- P1's final numbers are re-evaluated against the frozen revision-2 references.
