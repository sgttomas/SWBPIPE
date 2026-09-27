# ROOT's selection of the T3 designs

HELP_HUMAN (ROOT), 2026-09-26, under the owner's delegated correctness authority. ROOT relayed it to the T3 manager by SendMessage, and the manager records it here.

**Basis:**
- [SELECTION_PACKAGE.md](SELECTION_PACKAGE.md), FINAL, at `91192a1f1` (sha256 `7ee4efba…`);
- V1's targeted verification [REVIEW/VERIFY_R5.md](REVIEW/VERIFY_R5.md), **VERIFIED**, at `57842c192` (sha256 `8097e13b…`);
- the review trail listed in package §2.

ROOT checked the hashes before selecting: DESIGN `fb62ef4a`, S11_CONTAINMENT `e6507587`, D5_TRIGGER `f6e24a69`, R5_4_CURVED `2c9fae78`, D2 `edc78f9c`, and `GATE/S11_EXCEPTIONS.json` `8f3687f4`. The `DESIGN_NUMERICS`, `DESIGN_STANDING` and `GATE` SHA256SUMS all verify.

## Selected

1. **Designs.**
   - D1 revision 5a.2 at `932698d7a`: [DESIGN_NUMERICS/DESIGN.md](DESIGN_NUMERICS/DESIGN.md), [S11_CONTAINMENT.md](DESIGN_NUMERICS/S11_CONTAINMENT.md), [D5_TRIGGER.md](DESIGN_NUMERICS/D5_TRIGGER.md) (revision 2a) and [R5_4_CURVED.md](DESIGN_NUMERICS/R5_4_CURVED.md).
   - D2 revision 5b.2 at `8b2bca91b`: [DESIGN_STANDING/DESIGN.md](DESIGN_STANDING/DESIGN.md).
   - [ROOT_RULINGS_V1.md](ROOT_RULINGS_V1.md) and [ROOT_RULINGS_V2.md](ROOT_RULINGS_V2.md) are binding amendments. Where a ruling differs from a design, the ruling wins.
2. **The slice plan and every hard constraint in package §4, exactly as written:**
   - **Kernel:** S11-K → K3a → K-D5 → K2a → K1 → K2b → K5. The rest of K3 runs in parallel after K3a, then K4, then K6 and V-K.
   - **Facade:** S11-F (S-H in the same PR or after) → F1 → F2a (atomic with S-G1) → S-I1 and S-I2 → F2b per domain → F3 (atomic with S-G2 and S-E1) → V-P → join.
   - S-J comes after S-I. W1c comes with T4.
   - S-H never lands before S11-F.
   - No retirement where rows or checks regress. Coexistence holds.
   - The no-Passed-breach gate runs on both entries, with exactly the triples in [GATE/S11_EXCEPTIONS.json](GATE/S11_EXCEPTIONS.json): 88 triples in 13 cases captured, 140 in 22 cases typed. The list is empty after S11-F.
   - The nonlinear loop stays on the legacy binary64 variants until T5.
   - The fixture stop rule, and the per-slice gates.
3. **The §5 decisions:** final as tabled.
4. **R5-4: final acceptance**, resting on V1's two-way verification of all four conditions.

## Conditions

- **C1, the K3a arctangent.** The bound is measured, not proved (at most 2.69 ulp at p = 128, over 13 angles).
  - K3a's test vectors carry a few-ulp tolerance and include angles beyond V1's 13, across 0 < φ < π and near both ends.
  - K3a either proves a bound or records the measured one as its specified tolerance.
- **C2, NOTE 8.** `withheld_rows.py` and `b_proof.py` read `hanger.stiffness`, either in the K-D5 slice or in the first slice that relies on them. Any change to their outputs is reported to ROOT.
- **C3, the S11-K basis.** S11-K was selected on S11_CONTAINMENT revision 3 plus R3; the selected text is now revision 5a.2 (`e6507587`).
  - RV1 verifies S11-K against `e6507587` (see the addendum in `TASK_BRIEFS/RV1_S11K_REVIEW.md`).
  - Any 5a.x change that affects S11-K code is applied before the S11-K PR merges, as [ROOT_SELECTION_S11.md](ROOT_SELECTION_S11.md) already requires.
- **C4, deferred decisions.**
  - RF-ELOAD is selected separately after V3's refutation, before F3.
  - The D-4 identity names are approved at F2a.
  - The D-8 budgets and the M32 ceilings are selected from the K6 and V-P measurements.
- **C5, slice briefs.**
  - The manager prepares the K3a brief now, then K-D5.
  - K3a may be developed on a base that includes the S11-K head, but it lands after S11-K merges, with its own full-gate PR.
  - One heavy cargo job at a time on the host. The implementation spawns are held until S11-K is at RV1, and ROOT decides the spawn timing when it receives each brief.

**This selection closes no group.** A group closes only when it is fixed, verified and merged, with its paired evidence.

The graph-row updates (T3 per package §7, and T4's curved-element finding, already merged in PR970) go in ROOT's next records PR.
