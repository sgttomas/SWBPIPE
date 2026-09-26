# ROOT's early rulings on the V1 review

HELP_HUMAN (ROOT), 2026-09-26, relayed to the T3 manager by SendMessage and recorded here by the manager. Inputs: [REVIEW/RETURN.md](REVIEW/RETURN.md) (verdict BLOCKING) and the manager's [proposed dispositions](MANAGER_NOTES/V1_DISPOSITIONS.md). These rulings let the revisions start; ROOT rules the rest in the full disposition package.

1. **V1-B1 is fixed in D1 revision 2, and stays BLOCKING until V1 backchecks it.**
   - Sum every source-level contribution exactly and round once: loads, and combinations over retained states.
   - Put combination outputs under the stop rule.
   - Add V1's cancellation case and a combination case as negative controls.
   - Withdraw the §3.1 claim.
   - V1-S8: either prove that 1e-9 holds for every published quantity, or state and enforce the floor, for example by withholding quantities below it.
2. **V1-S11 is a silent-wrong path live on main.** T3 owns it at top priority.
   - Map it now: its exact reach, the routes, and a realistic-magnitude example as well as the extreme one.
   - Propose containment that can land early as its own small slice, before any general-method work. For example, detect input cancellation beyond binary64 and mark the result Sensitive, or refuse it.
   - If the containment touches only files T1 does not change, it may land before T1 merges.
   - R1, or an addendum to it, needs a reference case.
3. **Interface.**
   - The package must carry a concrete recommendation for when `load-reference-source-1` stops being fresh; ROOT rules on it there.
   - Adopted now: V1's split of S-E into S-E1 (the re-derivation core, kept) and S-E2 (the joined standing wiring, conditional).
   - Adopted now: one shared retirement gate for D1's I-3 and D2's R-B, covering coverage, budgets, standing in all three languages, and value agreement (V1-S2).
   - Name an owner for the successor-identity readers (V1-S3).
4. **V1-S5, capture refusal of |x| ≥ 2^53.** It makes a 1e16 N/m spring unsolvable on every route. It is a refusal, not a silent wrong, but it is a real capability hole. T3 owns the capture step; T6 owns the export carriers. D2 corrects its PR-5 predictions.
5. **The remaining SHOULD-FIXes (S4, S6, S7, S9, S10) go into the revisions.** For S6, prefer a reader rule that cannot diverge across languages, such as refusing when a host-exp field is used, over a one-ulp tolerance that depends on the host.

## Manager's application

- **Successor-identity readers (V1-S3):** D2 owns them, as the manager proposed (dispositions R-5).
- **The shared retirement gate:** D1 defines it, and D2 cites it (dispositions R-4).
- **S11:** mapped in [MANAGER_NOTES/S11_MAP.md](MANAGER_NOTES/S11_MAP.md). D1 designs the containment slice as the first deliverable of revision 2. R1 gets the RF-CANCEL addendum ([TASK_BRIEFS/R1_ADDENDUM_CANCEL.md](TASK_BRIEFS/R1_ADDENDUM_CANCEL.md)).
- **Still open for ROOT in the package:** the fate of the joined identity (dispositions R-3 (a)); the eligibility of historical physics-source-1 (R-3 (b)); historical all-selected source-blocks-1 (R-7).
