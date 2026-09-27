# D1: design note for slice S11-G (the formation-noise guard)

This is a design TASK for D1 (resumed). `_COMMON.md` applies. Commissioned by ROOT on 2026-09-27 (`ROOT_RULINGS_V1.md`, "I4's F12 stop"). Its trigger is the no-interim ruling's reopen condition: formation-class Passed breaches above 1e-7 relative, and 1.2e57 rad published as Passed.

## Why

S11-F makes every load sum exact, but it cannot repair errors in the **formed terms** themselves. Seven frozen-reference triples remain Passed breaches after S11-F (`GATE/FORMATION_EXCEPTIONS.json`):

- **Load formation (3 triples).** RF-CANCEL-UDL-W1e80 and -W1e8, th.S1.RZ.
  - SP's per-load fixed-end coefficients are formed by different binary64 expressions at the i and j ends (0.5b² − 2/3·b³ + 0.25b⁴ against −b³/3 + 0.25b⁴). The intended-cancelling FEMs therefore differ: the ledger row at S1-RZ is (1e-8, −3.3333333333333325e79, +3.333333333333335e79), whose exact net is 2.63e64.
  - The published value is the exact net of the represented terms, and it is wrong by about 1e81× the criterion. In UDL-W1e8 the error is 48× the criterion, on both entries.
- **Formed recovery terms (4 triples).** F- and M-G1e80-GnG-INPLANE end moments. The end moment is the formed K_e·u, whose roundoff at the member's 50 N·m scale swamps a 1e-8 net: up to 1.2e-6 relative.

K-D5 does not catch these. Its EF uses the ledger's represented terms and checks the displacement solve, not formed recovery terms.

## What to design (narrow)

1. **The load-row guard (first).** For each ledger row, compute Σ over **formed** terms of |term|·(formation bound of that term). Demote the case to **Sensitive** where that exceeds 1e-9·max(|net|, scale). Use the exact net and the published row's scale class (D1's floor and scale rules).
   - **Formed terms against input terms (required).** A nodal load, or any exactly represented input, carries **zero** formation noise. Only terms produced by binary64 formation (FEM coefficients × w, thermal and thrust equivalents, generated loads before D-14's exact form, curved equivalents) carry a bound.
   - **Legitimate exact-input cancellation must never be demoted.** That includes all 221 triples S11-F repairs, and any nodal (G, n, −G).
   - Specify the per-term formation bounds (for example, a stated number of ulp of the formed value per formation expression), and how the ledger carries each term's formed/input tag and bound without changing `AssembledForce`'s exactness.
2. **The recovery-side guard (second, or a small follow-on).** For formed K_e·u end actions (and station actions if needed): Σ|products|·(bound) against 1e-9·max(|result|, scale), with demotion to Sensitive. State which recovery quantities it covers, and its cost.
3. **Fail-closed and no in-band change.** Demote to Sensitive, never refuse, never `Err`, **no new envelope field**. If a field would be needed, stop and tell the manager; it goes to ROOT.
4. **Coverage and forecast:**
   - all 7 formation triples caught, in every mode;
   - none of the 221 repaired triples demoted;
   - no RF-CANCEL, RF-SKEW or other frozen-reference row that passes today demoted, and no committed-fixture row demoted (forecast it by script over the committed fixtures and the frozen references, as your earlier withheld-row scripts did);
   - the committed-byte forecast: which committed bytes, if any, would change.
5. **Per row:** if any of the 7 cannot be caught without demoting a legitimate row, justify, per row, why it waits for F2/F3's exact formation. That goes to ROOT.
6. **Slice S11-G's write set and tests** (with RV1's lesson: every pin backed by a behavioural test that first shows the two paths differ). Include mutations, for example: dropping a formed term's bound; tagging a formed term as input; applying the guard to input terms.
7. **Conflict boundary.** S11-G is implemented on main after S11-F. **K-D5** may land before or after it: name the PP conflict boundary. K-D5 touches only the `solve_preview_reduced_system` call (PP:3965), FK `finish_checked_factor` and SA; S11-G touches the ledger, the Sensitive mapping and recovery.

## Basis

- `S11_CONTAINMENT.md` 5a.2 (§2.2 declared formation, §4, §5 guard and floor, §6, §10 items 2–3); `DESIGN.md` 5a.2 (the floor, S\*, classes, D-14);
- S11-K on main (the ledger and `ExactAccumulator`);
- I4's S11-F candidate on `codex/piping-s11f-20260927` (read-only), for the ledger tagging;
- `GATE/FORMATION_EXCEPTIONS.json` and `GATE/S11_EXCEPTIONS.json` (`db665f2cb`);
- the frozen references.

## Write set and return

- `T3/DESIGN_NUMERICS/S11G_GUARD.md`, with scripts and outputs in `_run_records/` and `SHA256SUMS` refreshed. No product code.
- Send the manager a SendMessage summary.

V1 then checks the note: all 7 caught; none of the 221 and no RF-CANCEL, RF-SKEW or committed-fixture row demoted; and the committed-byte forecast.

## Addendum (ROOT, 2026-09-27)

- **Required coverage:** S11-G's load-row guard must catch **all 4 UDL-W1e8 th.S1.RZ rows** (captured and typed, dense and sparse). After S11-F these rows publish the correctly rounded net of the represented terms, 0.4916666902601719, which is 47.99× the criterion. That is 3% worse than base's 46.47×, because base's fold happened to land closer. It is the same formation class as UDL-W1e80.
- Your forecast shows each of the 14 formation rows caught, using the **post-S11-F** published values: I4's `IMPLEMENTATION/S11F/_run_records/formation_rows/` on the S11-F branch, once committed.
