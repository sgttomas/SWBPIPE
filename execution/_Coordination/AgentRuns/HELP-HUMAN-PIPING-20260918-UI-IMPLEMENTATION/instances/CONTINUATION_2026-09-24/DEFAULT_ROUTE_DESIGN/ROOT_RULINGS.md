# ROOT rulings on the first T0R design draft

HELP_HUMAN (ROOT), 2026-09-26, under the owner's delegated correctness authority. The inputs are:

- the first draft [DESIGN.md](DESIGN.md);
- the [independent design review](REVIEW/RETURN.md), verdict BLOCKING;
- the [S0 reference check](REFERENCE_CHECK/RETURN.md), which found discrepancies of definition and coverage but no arithmetic error;
- the owner's [SIF decision](../OWNER_SIF_DECISION_2026-09-26.md);
- the owner's report-package answer, below.

The design lead revises DESIGN.md and `references.py` to these rulings. The revision then gets an independent backcheck before ROOT selects it.

## Scope

T0R keeps option (d): repair what can be computed correctly on the ordinary route, and withhold the rest with stated reasons, under one new result identity. The review found two combination defects on main, and the design must now contain both: nonlinear superposition, and constant effort counted Σfactors times. Both publish silently wrong Current results today, which is exactly what T0R exists to stop. Containment still closes no group.

## Rulings

1. **B-1, nonlinear combinations.** When any operand's model has nonlinear supports, withhold every `mechanics` combination row with `NONLINEAR_COMBINATION_REQUIRES_SOLVE`, and keep the case states. Keep `result_state_subtraction` only as a labelled signed difference. Remove the nonlinear kinds from the `mechanics` allowlist. Add the lift-off counterexample as a reference and a mutation control. This contains part of M15.
2. **B-2, source-blocks-1 interaction.**
   - A source-blocks-1 envelope that contains any ordinary or failed case is published as it is today, with precision-1-shaped rows. It is not Current-, rule- or export-eligible until T3, and it carries a diagnostic that names why.
   - An all-selected source-blocks-1 envelope keeps its present standing and carries a notice for its abs-sum summary.
   - Correct §2.1(4) and decision 4, and add mixed-envelope tests (selected with ordinary, and selected with failed).
   - The design lead may propose a better rule, with evidence. The standard is that no known-defective quantity stays Current.
3. **SF-1, constant effort in combinations.** When a consuming constant-effort support exists and Σfactors ≠ 1, withhold `mechanics` combinations with a diagnostic. Subtraction may stay. Add the reference. This contains part of M11.
4. **SF-2, mixed moduli.** When operand modulus bases differ, withhold `mechanics` combinations with a diagnostic, and add a test.
5. **SF-3, merge constraint.** Land the producer, the readers and the supersession gates in one atomic PR.
6. **SF-4.** Use a static identity rule (precision-1 is never a fresh identity) or an envelope-consistency check, not a route predicate copied into three languages. List the helper as a T1 obligation.
7. **SF-5.** Add every missed consumer to §6 and to the slice write sets, and give the derived fixtures an owner. The missed consumers are:
   - the packaged self-test and native tests in `src-tauri`;
   - the `physics_audit_regression` benchmark. Its `pure_moment_does_not_become_force` asserts the M05 defect as correct, and that assertion is corrected, not deleted: the new expectation is the signed support moment;
   - the self-weight tests;
   - `analysisRunCompatibility.ts`, which gains a preview-evidence check;
   - `ResultsPanel.tsx` and `ComparisonPanel.tsx`;
   - the TS fixtures and `nativeMechanicsReplay.ts`.
8. **SF-6, verification.**
   - Replace the detection run with an adapter that compares main's legacy quantities with the frozen values.
   - Add the missing references: withheld attribution, non-consuming constant effort, the mixed source-blocks envelope, nonlinear combination withholding and constant-effort Σfactors.
   - Accept either REF-M08-L tie location within tolerance.
   - Include the per-case displacement tie site.
9. **SF-7, report package.** The owner accepted the outage (answer below). The legacy report package refuses the new identity until T6, and the design states this for all fresh results.
10. **SF-8, intensified stress.** Following the owner's decision, T0R may publish a per-case equal-factor intensified measure `i·hypot(My, Mz)/Z`, where `i` is the user's own scalar SIF, at the ends of members adjacent to markers and branches.
    - It is labelled with its formula and the user factors it consumed, uses no default factor, and never enters combination algebra.
    - It needs hand-statics references and negative controls.
    - Make it the last producer slice. If it threatens T0R's schedule or correctness, it moves to T4 without holding up the rest.
11. **S0 reference fixes.**
    - Freeze REF-CE.
    - Freeze B1's signed arc-frame station values, the chord-frame endpoint values or the endpoint stress rows, and its `y_reference`.
    - Add a two-device-node SPRING case and the COMB moment envelope.
    - Add the translated-R1 moment-origin case, X1's partial-restraint rows, and the withheld-attribution and stress-coverage cases.
    - Record S1's length, X1's restraint sets and every zero-valued expectation's scale.
12. **Notes.**
    - **N-1:** adopt the narrower attribution rule; only two or more residual-determined devices on one DOF are ambiguous.
    - **N-2:** discriminate nominal arc rows in the table, and label the chord-frame endpoint rows truthfully.
    - **N-3:** specify the empty `preview_cases` shape for blocked envelopes.
    - **N-4:** define the modifier count and multiplier diagnostic under the new identity.
    - **N-6:** add an indeterminate arc reference.
    - **N-7:** native witnesses run on the owner's Mac.
    - **N-8:** probe with capture, as the desktop path does.
13. **Design lead's decision list.**
    - The identity name `preview-physics-1` is accepted.
    - Bends: an arc publishes no maximum. A model with an arc publishes no stress headline, and a straight-member-scoped headline (N-5) is not adopted in T0R.
    - Constant-effort support actions are published.
    - Re-homing source-blocks-1 goes to T3.
    - Add the kinked-arc `y_reference` warning as a diagnostic. It does not close M02.
    - User rule packs bound to retired result IDs will report `RULE_INPUTS_INCOMPLETE`. The owner is told.

## Owner answer on the report package

ROOT asked whether to accept that no fresh result would be report-package eligible until T6, or to add report support in T0R. The owner chose: "Accept outage to T6 (Recommended)."

## Rulings on revision 2 (design §12, R-1 to R-6)

- **R-1: accepted.** Readers derive the source-blocks standing reason from the receipt: only `aggregate == "qualified"` is admitted, and a selected-plus-ordinary envelope reads as needing recompute until T3. The producer adds no diagnostic, and source-blocks-1 bytes stay unchanged. This replaces the "or failed" clause of ruling 2.
- **R-2: rule binding is blocked.** An all-selected source-blocks-1 envelope keeps its Current standing for its other quantities, with a notice. Rule checks may not bind to its abs-sum summary until T3, which keeps ruling 2's standard that no known-defective quantity is relied on as if correct. The design lead's bound (at most √2 high, and conservative, because the admitted loads are nodal only) goes into the notice.
- **R-3: accepted; ROOT's SF-5 wording was wrong.** `pure_moment_does_not_become_force` asserts correct physics: the force norm is 0 under a pure moment. Keep that assertion and add the signed support moment expectation (Mz = −100 N·m).
- **R-4 to R-6: accepted as clarifications.** The nonlinear test applies to the model, including nonlinear supports that are inactive in every case. Σfactors = 1 uses the stated 64ε representation guard. The intensified measure uses the member's own Z, excludes arc ends, and gives an unreferenced header continuation a coverage diagnostic.
- **Imposed displacement.** The default route refuses the legacy imposed displacement; it is not silently dropped (probe cases E and E′). Add the optional targeted diagnostic for M10 in place of the raw serde error, as a small S2a item.

ROOT selects the design after the independent backcheck of revision 2 and the S0′ check of the `revision_2` references.
