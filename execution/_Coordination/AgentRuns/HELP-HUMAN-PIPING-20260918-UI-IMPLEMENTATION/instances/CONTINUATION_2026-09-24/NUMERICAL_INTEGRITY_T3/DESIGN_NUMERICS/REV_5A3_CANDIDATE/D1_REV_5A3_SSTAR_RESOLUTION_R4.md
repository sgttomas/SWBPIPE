# D1 revision 5a.3 (proposed, revision 4): the stop rule's resolution term, verification estimate, formation charge and ceiling floor

Design addendum by DS1, a Type 2 TASK (numerical design drafter), for ROOT (HELP_HUMAN), 2026-09-28. **This is revision 4 of the candidate.** It resolves V4's delta check at R3 (NOT VERIFIED: 1 BLOCKING, 5 NOTEs) and applies ROOT's rulings on it.

- **The candidate, R2 and R3 stay unchanged:**
  - the candidate, `DESIGN_NUMERICS/REV_5A3_CANDIDATE/D1_REV_5A3_SSTAR_RESOLUTION.md` (sha256 `c265d7bb…`, numerics `a7015eea3`);
  - R2 (`326a8d78…`, numerics `9e9e3056a`);
  - R3 (`2cbb6582…`, numerics `c85dc1151`).
- **This file is a proposal.** V4 runs a delta check; ROOT selects only after VERIFIED.
- `DESIGN_NUMERICS/DESIGN.md` (revision 5a.2, sha256 `fb62ef4a…`) stays hash-pinned. The amended text is in §5, as addendum blocks.
- **§13 maps every finding of V4's delta check at R3 to its resolution** ("Changes from R3"). §11 and §12 keep the earlier maps.
- **What R4 changes:**
  - W's contributions are formed at q_W = min(3p + 64, 1024), and prescribed values enter W's residual exactly (§5.5 item 1).
  - V4's runtime charge C_q is added, with a measured second-order remainder, a θ ≤ 1/2 check and a g check, and **e_q is now bounded, not conjectured** (§5.5 items 5 to 13).
  - The Hager–Higham allowance is pinned: **F = 2^16** (§5.5 item 8, §5.9).
  - Translation and rotation rows carry their measured bound in the stop rule (§5.5 item 12).
  - Every step of the guarantee is derived; the one exception is ‖K̃⁻¹‖₁ ≤ F·est, which the condition screen and W already rely on (§9).

**Labels.** Every claim is marked as one of:
- **derived:** it follows step by step, and a verifier can check each step;
- **measured:** it was observed in DS1's standard-library emulation (§14). A measurement is evidence, not proof;
- **argued:** it is reasoned but not proved;
- **conjecture:** it is believed, and it is not established.

**The honesty guarantee (§5.5's Corollary) now carries no "argued" or "conjecture" label.** The labels remain in this file only for availability statements and for history.

## 0. Basis, inputs and what was not done

**Read for this revision.** Hashes are sha256, first 8 hex digits. `<wt>` is the T3 worktree root.

| Input | Hash |
|---|---|
| V4's delta check at R3: `V4_VERIFICATION.md`, section "Delta check at R3", at numerics `c3f2cfc72` | `ed2031c3` (file) |
| V4's R3 records `_v4_records/r3/` (read: `r3_charge`, `r3_tilt`, `r3_repro`, `r3_sweep`) | — |
| `ROOT_RULINGS_V1.md`, section "D1 revision 5a.3: rulings on V4's delta check at R3", at `c3f2cfc72` | `9e53317f` (section) |
| V4's delta check at R2: `V4_VERIFICATION.md`, section "Delta check at R2", at numerics `e5f4ef3a8` | `c2f5539b` (file) |
| V4's delta records `_v4_records/delta_r2/` (read: `r2_lever2`, `r2_lever_fix`, `r2_probe_seed`, `v4emu_r2`) | — |
| `ROOT_RULINGS_V1.md`, section "D1 revision 5a.3: rulings on V4's delta check at R2", at `e5f4ef3a8` | `69c8f23b` (section) |
| V4's verification, `DESIGN_NUMERICS/REV_5A3_CANDIDATE/V4_VERIFICATION.md`, at numerics `3057fc22f` | `0222d0ec` |
| V4's records `_v4_records/` (read: `v4emu`, `cases`, `t_demo`, `small_checks`, `gate_probe_{nsec,loadonly}`) | — |
| `ROOT_RULINGS_V1.md`, section "D1 revision 5a.3: rulings on V4's verification", at `085638e58` | `9a36d688` |
| K4 at `5ad1b6174` (`codex/piping-k4-20260928`): the Rust probe `probe_v4_s3_y_reference_with_a_chord_component` (`adaptive_tests.rs`); RETURN §21; the combination's prescribed-row fix with control and mutant K4-M33 | — |
| K4 at `3ed6c0e26` (A2), for the functions below | — |
| D1 r5a.2 §4.1.3 (the condition screen, which has no numeric F) and §2 (W1a's scope: nodal loads only) | `fb62ef4a` |

The candidate's §0 lists its own inputs, all still valid: D1 r5a.2 `fb62ef4a`; D2 r5b.2 `edc78f9c`; R1 references `7b176dbb`; and the rest.

**Where the stop rule, S\* and the gate live in K4 (A2 `3ed6c0e26`, `retained/adaptive.rs`):**
- `residual_rows` `:872`;
- `scales_at` `:1093–1136`;
- `stop_rule` `:1141`;
- `classify_rows` `:1430`.

Their bodies are byte-identical to A1's (the candidate's §0).

**Done.** Standard-library Python only, under `<wt>/scratch/ds1/`, one process at a time under `nice -n 19`, as the host rule requires.
- `emu4.py` is R3's `emu3.py` with R4's changes, each behind a switch whose default is R4 (option `r4`):
  - q_W = 3p + 64 and exact prescriptions in W's residual;
  - r₂, the norms, est, F, θ, the g check and the charge;
  - the measured bound for translation and rotation rows.
- R4 re-ran, under emu4:
  - every control: 48, which are R3's 44 plus CHARGE-SLENDER and V4's TILT-LEVER at three gains;
  - every mutant: M1–M23;
  - K4's probe cases, rebuilt;
  - Hager–Higham's est against the exact ‖K̃_P⁻¹‖₁ on the controls and probes;
  - the 2,000-frame sweep;
  - K4's R1 lane (120 cases).

  R1's formation-scale ratios depend only on E, which R4 does not change, so R2's figures stand.
- The files are listed in §14.

**Not done.**
- No Git write, and no cargo or Rust run. Every K4 statement comes from reading its source and recorded outputs.
- No product code.
- No file written outside `<wt>/scratch/ds1/`, apart from temporary edit scripts and copies of outputs for the rerun comparisons, in the session's own temporary directory.

## 1. Summary

**The recommendation, in seven lines.**
1. **Correct §4.1.9.** A tail below 2p's rounding unit is lost identically at p and at 2p ("saturated"; Lemma 2). A wrong value then passes by up to the verification's own first-order error.
2. **Define a per-body formation scale E** for the force and moment kinds (§4.1.6.2).
   - It is the Σ\|terms\| of every recovery expansion over a bounded operator, with each stage rounded once at 2p.
   - g is exact; E is coupled through L_b; it is published rounded upward.
   - V, Φ and G5b's item 6a all use one binary64 ê, bit for bit (S4).
3. **Change the stop rule.** Accept p only if \|q_p − q_2p\| + V ≤ 2^-64·max(\|q_2p\|, S\*), with V = 2^(8−2p)·ê for force and moment rows, **and** the verification estimate satisfies W ≤ 2^(6−2p)·ê (§4.1.6.3; ROOT's S2 ruling and its confirmation of V/4).
   - W's residual is one exact sum over the contributions, formed at **q_W = min(3p + 64, 1024)**, with **exact prescribed values**, recomputed on the final state (V4-R1, V4-R2; ROOT's ruling 1 at R3).
   - Translation and rotation rows take as V their measured bound W⁺ (R4).
4. **Charge what W cannot measure (R4; V4-T1, ROOT's ruling 2 at R3).**
   - C_q = ‖ā_q S‖₁·(2^(7−q_W)·F·est·N_u + 70·2^-P·‖S⁻¹δ̂‖_∞ + 3F·est·‖S·r₂‖_∞) must be ≤ 60·2^-2p·ê at p = 128 and 256, and ≤ 2^-22·2^-64·M_q at 512.
   - θ = F·est·2^(7−P)·‖SĀS‖₁ must be ≤ 1/2, and every member's g ≤ 2^(P−16).
   - F = 2^16, pinned. est is the verification's own Hager–Higham estimate.
   - The guarantee is then derived beyond first order, modulo F (§5.5's Theorem and Corollary).
5. **At p = 512 only,** floor the force and moment S\* at Φ = fl↑(2^-438·ê).
6. **Residual gate (§4.1.4; ROOT's S3 ruling, O12 reversed; the hybrid form confirmed).**
   - The acceptance test uses the bounded operator at contribution level, \|f\| + ΣĀ\|u\|, on the best state evaluated (V4-R5).
   - Refinement stays driven by M03's coalesced ratio, so every correction made today is kept.
   - The gate's own residual stays over K^q's assembled entries (§5.6).
7. **Publish** `resolution_scale` (E per body), the estimate's and the charge's summaries, θ and F.
   - G5a gains the shape check, the zero rule, the sanity bound, V4's published-data lower bound on ê, and the summaries' bounds.
   - G5b applies item 6a at p = 512. G5c is unchanged.

**Evidence** (measured in emulation; the full tables are in §7 and §14):

| Set | Result under R4 | Result today |
|---|---|---|
| **2,000 random frames** | No false claim (the tightest claim ratio, 0.995, is the same frame and value as today); nothing unresolved; 0 G5a failures. **Every frame's selected precision, classes and rejection reasons equal R3's** (`sweep4_summary.json`). The estimate and the charge never fired: W ≤ 0.0048·V, θ ≤ 3.0e-33, and the worst charge at a selected pair is 0.0093 of its allowance (R790 at 128: moduli spanning about 2^200, est ≈ 2^106). Work +1.0 % (proxy, excluding the E pass, the estimate and the charge), as in R3 | 1 false claim; 230 unresolved |
| **48 controls** (R3's 44; CHARGE-SLENDER; TILT-LEVER at gains 2^90, 2^100 and 2^110) | Every selected case honest; 0 G5a failures. **The 44 R3 controls keep R3's precision and classes exactly.** The worst charge at a selected pair is 4.7e-29 of its allowance, and θ ≤ 1.5e-46. LEVER2 (three gains) and TILT-LEVER (three gains) are refused. CHARGE-SLENDER is refused at 128 by the charge alone and selected at 256 | 10 false claims (9 natural, including the three LEVER2; SEEDED-COMMON's is seeded). TILT-LEVER is refused today too |
| **K4's probe** (20 cases), rebuilt, **emulation only** | All 20 selected at 128, honest, 0 G5a failures, as in R3. Worst charge 9.0e-45 of its allowance | The 8 single-mode y_ref (3,4,5) cases are refused by the coalesced gate |
| **K4's R1 lane** (120 cases) | Identical selected precision (all 128) and classes under today's rule, R2, R3 and R4; worst charge 9.9e-37 of its allowance; 0 G5a failures | — |

## 2. The failure class, derived

### 2.1 Notation

- q is a published force, moment, translation or rotation of a selected case (or of a combination solved as its own case).
- q\* is its exact value for the intended model, and q_P its value at precision P, with e_P(q) = q_P − q\*.
- ε = 2^-64 and M_q = max(\|q_2p\|, S\*_2p(body, kind)).

The stop rule accepts p when \|q_p − q_2p\| ≤ ε·M_q for every q (the r5a.2 rule, before this revision).

### 2.2 Lemma 1: what acceptance bounds (derived, exact)

If p is accepted, then \|e_p(q)\| ≤ ε·M_q + \|e_2p(q)\| and \|e_p(q)\| ≥ \|e_2p(q)\| − ε·M_q. This is the triangle inequality.

So the published claims (b = fl↑(2^-64·S\*) for rows below the floor, and relative 1e-9 above it) hold only to the extent that \|e_2p(q)\| is negligible against ε·M_q. That is §4.1.6's own premise, and r5a.2 does not enforce it.

### 2.3 Lemma 2: saturation (derived; statement corrected at the binade boundary, V4-N3; proof repaired, V4-R4)

Let x be formed exactly and rounded once, to nearest, at p bits in the candidate and at 2p bits in the verification. Write x = y + t, where y ≠ 0 is a p-bit number and **0 < \|t\| < 2^-2p·\|y\|/2**. Then fl_p(x) = fl_2p(x) = y.

*Proof.* Let 2^e ≤ \|y\| < 2^(e+1). y is representable at p bits, hence at 2p bits. At 2p bits there are two cases, according to the neighbour of y on t's side.
- **Unless y = ±2^e and t points toward zero,** that neighbour is at distance 2^(e+1−2p), so the half-spacing on t's side is 2^(e−2p). Since \|y\| < 2^(e+1), 2^-2p·\|y\|/2 < 2^(e−2p), so \|t\| < 2^(e−2p).
- **If y = ±2^e and t points toward zero,** the neighbour below is at distance 2^(e−2p), so the half-spacing is 2^(e−2p−1) = 2^-2p·\|y\|/2 > \|t\|.

In both cases \|t\| is strictly less than the half-spacing on its side, so y is the unique nearest 2p-bit number to x. At p bits every spacing is 2^p times larger, so the same holds a fortiori. Hence fl_p(x) = fl_2p(x) = y. ∎

R2's proof inferred the strict inequality from a lower bound on the threshold (V4-R4); the statement was right. The candidate's hypothesis, \|t\| < ½·ulp_2p(y), is not sufficient at a binade boundary. V4's example: y = 1, t = −0.6·2^-2p.

### 2.4 Theorem: the class, and by how much (derived to first order)

To first order, e_P(q) = Σ_k g_k·δ_k^(P), over the roundings k of the P computation, where g_k = ∂q/∂x_k and δ_k^(P) = fl_P(x_k) − x_k.
- For a rounding saturated in Lemma 2's sense, δ_k^(p) = δ_k^(2p) = −t_k. So the stop rule sees only the non-saturated part.
- The saturated part is common to both candidates and bounded by Δ_2p(q) ≤ 2^-2p·Σ_k \|g_k\|·\|x_k\|.
- An accepted candidate therefore satisfies \|e_p(q)\| ≤ ε·M_q + Δ_2p(q) + O(2^-p)(ε·M_q + Δ_2p(q)).

A false claim therefore requires M_q ≲ 2^(64−2p)·Σ\|g\|\|x\|, and its size is at most Δ_2p(q). S\* is formed from computed values only, so nothing prevented this.

### 2.5 Instances (measured, `run_controls4.stdout.json`)

The claim ratio is \|q_pub − q\*\| divided by the claim: b for an absolute row, 1e-9·\|q\*\| for a relative one. A ratio above 1 is a false claim.

| Case | Site | Today (K4's rule and gate) |
|---|---|---|
| **F-2** (I12) | u and the reduced right-hand side saturated at 128 and 256 | Selected at 128; N = 0 with b = 0; ratio ∞ |
| **F-2 with S\* > 0** (DS1) | u saturated; a real 1 N force elsewhere in the body | Selected at 128; ratio 16 |
| **Prescribed tail** (ROOT's A2 point 1): combined prescription 1 + 2^-1100 | The prescription rounded at every P ≤ 1024 | Selected at 128; b = 0; ratio ∞ (both forms) |
| **F-2 at the ceiling** (DS1) | Saturated at every P | Selected at 128; ratio ∞ |
| **ASSEMBLY-SAT** (V4) | The assembled entry fl(2^479 + 1.5) = 2^479 at 128 and 256 | R = −1.5 N `relative_verified` against a truth of 0; ratio ∞ |
| **LEVER2** (V4-R1): an exactly representable lever of gain 2^90, 2^100 or 2^110, a prescribed rigid translation, and a tip spring 2^-580 of the tip's assembled diagonal | The spring, in every assembled entry up to 576 bits: at 256, at 512 and in K^q at 576 | Selected at 256; the link force and two reactions published 0.0 `absolute_verified`; claim ratios 1,005, 1.03e6 and 1.05e9. **R2 did the same:** W, formed over K^q's assembled entries, is 0 |
| **F-3** (RIGID-UNLOADED, F-3-FREE, F-3-ROT) | Rigid-mode leakage at every P | Unresolved (safe) |
| **TILT-LEVER** (V4-T1(c)): LEVER2's lever with no spring, its arm tilted by t/L = 2^-290 so that the arm's axial (t/L)² term in its (uy,uy) entry is lost within the entry up to 576 bits; a prescribed rigid rotation about the link node | Within one element entry, at 256, at 512 and in R3's q = 576 | Refused (safe) at every gain, today and under R3 and R4: the stop rule sees the leak site (Δ ≠ 0 at 256 and 512). Under R4 the charge would also refuse it at 256 (2.5e34 to 2.6e40 of its allowance) |
| **2,000 random frames** | — | 1 false claim (R982, an F-2 pattern); 230 unresolved |

**Combinations (ROOT's A2 point 2).** A combination is its own solve (F-1), so it falls under the same lemmas and the same rule, with its own E and estimate.

### 2.6 What was wrong in §4.1.9 (derived)

"Precision-dependent error is 2^-p at p and 2^-2p at 2p, so the stop rule sees it" is false for saturated roundings. Their error is precision-dependent, but identical at p and 2p. Common-mode loss of an exact input (V1's check L) is the special case where t is an input tail. Revision 2's exact sums remain necessary, and they cannot remove saturation.

## 3. The formation scale E (definition and derivation)

### 3.1 Definition (computable; V4-S5)

**The bounded operator B̄** replaces each axis component by 1, each bending-row translation entry of B by the 1/L of the same formation, and every other B entry by 1. |D| and 1/L are used as formed. This bounds a component that is formed as noise, whose error is as large as its own value.
- **Derived (V4 §3):** each axis component's absolute error is ≤ (10g + 13.5)·2^-P, and \|e_c\| ≤ 1.
- **Measured:** the entrywise form fails the G5a lower bound on 8 controls, and the gate built on it refuses the y_ref (3,4,5) cantilevers (§7, mutant M7).

**g (exact).** g_m = 2^k, where k is the least integer ≥ 0 with 4^k·(y_c·y_c) ≥ (y_ref·y_ref).
- y_c is the Gram–Schmidt residual vector `yc` of the same formation (`assemble.rs`, before normalization), and y_ref is the source's binary64 vector.
- Both dot products are exact sums of exact products (`ExactWideSum`), and the comparison is exact. So g ≥ \|y_ref\|/\|y_c\|, and g is a power of two, which makes multiplying by it exact.

**Stage rounding.** E_q is evaluated from the verification state at P = 2p. Every stage is one exact sum of exact two-factor products of that stage's P-bit operands, rounded once to nearest at P. The stages are:
- d̄: the node's 1-norm, \|u_x\| + \|u_y\| + \|u_z\| (and the same for rotations);
- ē;
- Q̄;
- V̄;
- stations;
- the rows of Ā;
- the reaction sums.

The products are those of the absolute operands with 1/L, the |D| coefficients and the Ā entries. They are:
- **End actions:** ē0 = d̄_j + d̄_i (axial, twist); ē2 = r̄_i + (1/L)(d̄_i + d̄_j) (and ē3–ē5 alike); Q̄ = \|D\|·ē; V̄ = (1/L)(Q̄_a + Q̄_b); all multiplied by g_m.
- **Station moments:** \|t\|·Q̄_j + \|t\|·Q̄_i + Q̄_i (× g_m). Forces are as at end j.
- **Spring actions:** \|k\|·\|u\|.
- **Directional spring actions:** Σ_b \|k_ab\|·\|u_b\|, with k_ab the block entries as formed.
- **Reactions:** \|f_c\| + Σ_j Ā_cj·\|u_j\|, where f_c is the **exact ledger net**, never Σ\|terms\|, and Ā is **the assembled bounded operator**:
  - Ā = one exact sum per entry, rounded once, of the element blocks g_e·B̄ᵀ\|D\|B̄;
  - each element block is itself stage-rounded: \|D\|B̄ rounded once, then B̄ᵀ(\|D\|B̄) rounded once;
  - plus \|k\| of global-axis springs on the diagonal;
  - plus **each directional block's formed entries in absolute value, entrywise** (\|k_ab\| for every a, b, off-diagonals included).
- **Support-group components and magnitudes:** the sum of their components' E_q.
- **Operands \|u\|:** the state's values at every DOF, prescribed values included as rounded at P. A combination's rounded prescription is ROOT's A2 point 1.

**E(body, kind)** is the maximum of E_q over the body's force (or moment) rows, **unpublishable rows included** (V4-S5, O9). E is a property of the formation, not of publication. Excluding a row whose value underflows would let its formation scale escape both V and G5a's lower bound (§6.3). A row whose value overflows forces E to overflow, and then:
- E is rounded upward once to binary64 for publication;
- an E that overflows makes the receipt entry unencodable, so the case is `unavailable` (`receipt_encoding`, §5 item 2).

**ê (item 6a), coupled in binary64 from the published bits:** ê_fo = max(E_fo, fl(E_mo/L_b)), ê_mo = max(E_mo, fl(L_b·E_fo)). With L_b = 0, ê = E.

**The single source (S4).** V, Φ and the estimate's threshold are formed from this binary64 ê and lifted exactly into Wide. The stop rule's own S\* stays coupled at 2p as K4 forms it (`scales_at`), because S\* is recomputed from rows. Φ, V and the threshold are not. So the stop rule and G5b use the same Φ bit for bit.

### 3.2 Why the coupling (measured)

Without the L_b coupling:
- the realized ratio \|q_P − q\*\|/(2^-P·E) reaches 1,217 (the candidate's R115 frame);
- **mutant M2 leaves F-3-FREE, F-3-ROT, GS-ROT-y345 and R115 unresolved**, and fails G5a on TWO-SPAN.

With the coupling, the total ratio is ≤ 1.77 over 739 states in emu3 (`measure3_3.json`; LEVER2's states excluded, since their error is the lost spring that the estimate rejects), as over 735 states in emu2, and ≤ 2.73 over 2,287 states in the candidate's emulator. V4 measured ≤ 1.21 independently.

### 3.3 λ = 2^8: the resolution count (derived; V4-N1)

This is V4's recount against K4's Rust (V4 §2.1), in units of 2^-P, first order, relative to the bounded chain. I checked the axis and Gram–Schmidt stages.

| Stage | Count |
|---|---|
| e_x | 4.5 |
| e_y | ≤ 10g + 3.5 |
| e_z | ≤ 10g + 13.5 |
| 1/L; the section terms | 3.5; 4.5 |
| B | ≤ 10g + 18 |
| d = T·u, with the representation of u | ≤ 10g + 15.5 |
| e | ≤ 10g + 20 |
| Q | ≤ 10g + 25.5 |
| V | ≤ 10g + 30 |
| Stations | ≤ 10g + 26.5 |
| DB | ≤ 10g + 23.5 |
| K_e | ≤ 20g + 42.5 |
| Assembled K, reactions | ≤ 20g + 45.5 |
| Support-group components and magnitudes | ≤ 20g + 48 |
| Springs | 2 |
| Directional blocks and actions | ≤ 6 |

**The largest row is ≤ 20g + 48 ≤ 68g.** With g folded into E, the resolution part is ≤ 68·2^-P·E_q. The candidate's looser count, ≤ 139g, also fits.

**Validity (R4: a check, §4.1.6.3 item 10).** The count needs 68g·2^-P small. R4 checks g ≤ 2^(P−16) at the verification, so 68g·2^-P ≤ 2^-9.9, and each first-order count then holds in its exact form with a factor of at most 1 + 2^-7 (§4.1.6.3, Lemma B, derived). A case that fails the check is not selected at that p. R3's argued remark about such members is replaced by the check; in practice V rejects them first (M23 survives, §7).

**Measured:** max\|q_P − R\*(u_P)\|/(2^-P·E_q) ≤ 2.33, where R\* is the recovery with P = 2048 operators (`measure3_3.json`, LEVER2 included; V4: 0.69).

**The element's rigid-mode leakage** is the K_e row with u a rigid motion: ≤ (20g + 42.5)·2^-P·(B̄ᵀ\|D\|B̄\|r\|). So F-3's leakage is inside λ.

### 3.4 Translation and rotation (derived modulo F in R4; argued in r5a.2 to R3)

For these kinds E would be \|u\| ≤ S\*, so a V of 2^(8−2p)·S\* would be negligible and none is charged. R3 and earlier bounded their propagated error only by argument, through the rcond screen.

**R4 measures and charges it.**
- A free displacement or rotation row's verification error is ≤ W⁺_i = \|δ̂_i\| + s_i·(t₁ + t₃) (§4.1.6.3 item 12 and its Theorem, derived modulo F).
- The stop rule adds W⁺ to \|Δ\| for these rows.
- The claim for translation and rotation rows is therefore derived modulo F, like the force and moment claim.
- Measured: no control and no sweep frame changes precision or class (§7, §6.1), and mutant M22, which drops the measured bound, survives.

## 4. The options, and the choice

The candidate's §4 derivations of (i-a), (i-b), (i-g), (ii) and (iii) stand. Their sweep figures come from the candidate's emulator, over 1,966 frames. Two corrections:

**S1: (iii)'s demotion claim, restated (derived; V4-S1).**
- At p = 512 a floored kind has S\* ≥ Φ = 2^-438·ê, so its threshold t = R·S\* ≥ 2^-472·ê. **A row is demoted only if the case is selected at 512 with Φ above the kind's computed scale, and the row has \|q\| < 2^-472·ê(body, kind).**
- Reaching 512 through V requires some row with ê/M > 2^(2p−72), which is 2^440 at the 256 candidate, or another rejection at 256.
- **Measured:** V4's DEMOTION2 (ê ≈ 2^482; 1 N rows) is demoted exactly as V4 found: relative rows 7 → 4 (V4: b = 1.67e-6 N on the force rows; emu2: b ≤ 4.7e-6 over both kinds). In the 2,000-frame sweep no relative row was lost.

The candidate's "never demotes" is withdrawn. The contrast with (i-g) is the threshold: (i-g) floors a kind whose published scale is below 2^(10−p)·ê (2^-118·ê at p = 128), and demotes its rows below 2^(40−p)·ê.

**The additions are orthogonal to the options.** The verification estimate and the formation charge (§4.1.6.3) and the gate's bounded acceptance (§4.1.4) would serve any of them.
- With them, (iii)'s honesty no longer rests on a conjecture. W measures the verification's solve error, and the charge bounds what W cannot measure.
- The whole guarantee is derived beyond first order, modulo F (§4.1.6.3's Theorem and Corollary).
- The candidate's robustness argument for (i-g) (margin 2^66 against λ at 2p) is therefore no longer needed.

**(iii) remains the recommendation:**
- it changes no correct outcome except by escalation, and by the demotion class above;
- it demotes only below 2^-472·ê;
- the estimate and the charge cost, per verification, one formation pass at q_W, two exact residual passes over the contributions, two light passes (Ā's rows, and E's expansion with operands s), one substitution pair and one recovery pass.

## 5. The exact amended text (addendum blocks)

### 5.1 §4.1.6, stop rule: replace the "Accept p" bullet; add the floor bullet

> - **Accept p** (revision 5a.3) when all hold, decided exactly:
>   - (a) every published quantity q satisfies `|q_p − q_2p| + V_q ≤ 2^-64 · max(|q_2p|, S*)`, where **V_q = 2^(8−2p)·ê(body, kind)** for q of kind force or moment, and **V_q = W⁺_q** (§4.1.6.3 item 12) for translation and rotation (R4; 0 before R4);
>   - (b) the verification estimate passes, `W_q ≤ 2^(6−2p)·ê(body, kind)`, for every force and moment row (§4.1.6.3 item 4);
>   - (c) **θ ≤ 1/2** and **every member's g ≤ 2^(P−16)** at the verification (§4.1.6.3 items 9 and 10; R4);
>   - (d) **the charge passes** for every force and moment row: C_q ≤ 60·2^-2p·ê(body, kind) at p = 128 and 256, and C_q ≤ 2^-22·2^-64·M_q at p = 512 (§4.1.6.3 item 11; R4).
>
>   ê is §4.1.6.1 item 6a's binary64 value, formed from the E bits the receipt publishes and lifted exactly. V_q, Φ, the estimate's threshold and the charge's allowance use that same ê, bit for bit. A failure of (b), (c) or (d) rejects p like a failure of (a), with its own reason.
> - **The ceiling floor (revision 5a.3).** At the last candidate, p = 512 (verified at 1024), `S*(kind) := max(S*(kind), Φ)`, with `Φ = fl↑(2^-438 · ê(body, kind))`, for force and moment. It applies in the stop rule and in the classification: item 6a, the same Φ bits. At p = 128 and 256 there is no floor.
> - **Why (revision 5a.3).**
>   - A rounding whose tail lies below both p's and 2p's rounding units is identical in both candidates (§4.1.9). (a) charges the verification's resolution; (b) measures its solve error; (c) and (d) bound, beyond first order, what (b) cannot measure: the formation error of individual entries below q_W's resolution, and the second-order remainder.
>   - Without V, F-2 passed labelled exact against a truth of 2^-300.
>   - Without the floor, an unloaded body moved rigidly (F-3) could not be accepted at any precision.

### 5.2 §4.1.6 "What acceptance guarantees": replace its first bullet (V4-N6)

> - **What acceptance bounds (revision 5a.3).** If p is accepted, then for every q, |q_p − q\*| ≤ |q_p − q_2p| + |q_2p − q\*|, and |q_2p − q\*| ≤ (recovery resolution) + (the solve error of the 2p state).
>   - The recovery resolution is ≤ 69·2^-2p·E_q (§4.1.6.2 item 5 in its exact form, §4.1.6.3 Lemma B; derived under the g check).
>   - The solve error is ≤ Ŵ_q·(1 + 2^-2p) + C_q for force and moment rows and ≤ W⁺_q for translation and rotation rows (§4.1.6.3 Theorem; derived modulo F).
>   - The rule admits p only when (a) to (d) hold.
>   - So |q_p − q\*| < 2^-64·M_q − 62·2^-2p·ê at p = 128 and 256, |q_p − q\*| ≤ 2^-64·M_q·(1 + 2^-22) at p = 512, and |q_p − q\*| ≤ 2^-64·M_q for translation and rotation rows (§4.1.6.3 Corollary). b = fl↑(2^-64·S\*) therefore bounds |q_p − q\*| for an `absolute_verified` row, with the factor 1 + 2^-22 at p = 512. For a kind floored at p = 512, b = fl↑(2^-64·Φ) ≈ 2^-502·ê.
>   - **Modulo F:** the one step not derived is ‖K̃_P⁻¹‖₁ ≤ F·est with F = 2^16 (§4.1.6.3 item 8).
> - **Scope of b.** b bounds the candidate's value q_p, not the published binary64 value. Two small differences follow; both were present in r5a.2 and are stated, not widened:
>   - publication adds |q_pub − q_p| ≤ 2^-53·|q_p|, which for an absolute row (|q_p| < 2^-34·S\*) is < 2^-87·S\* ≤ 2^-23·b (V4 has corrected its earlier 2^-24·b to 2^-23·b, V4-R7);
>   - the published S\*_pub (from binary64 rows) may lie below the stop rule's S\*_2p by a relative (2^-64 + 2^-52).
>
>   So |q_pub − q\*| ≤ b·(1 + 2^-22) at p = 128 and 256, and ≤ b·(1 + 2^-21) at p = 512, where the charge's 2^-22 adds to the publication gap. D2's interval binding [q_pub ± b] inherits this relative 2^-21 and the qualification "modulo F". R3's "first-order qualification" is withdrawn: no first-order step remains (§4.1.6.3).

### 5.3 §4.1.6.1: add item 6a after item 6

> 6a. **The ceiling floor (revision 5a.3).**
> - Per body, the receipt carries `resolution_scale`: E_fo and E_mo as 16-hex binary64 bit strings, finite and ≥ +0 (§4.1.6.2).
> - In binary64 (round to nearest), in this order: `ê_fo = max(E_fo, fl(E_mo/L_b))`; `ê_mo = max(E_mo, fl(L_b·E_fo))`. With L_b = 0, ê = E.
> - **Only if the case's selected precision is 512:** `fo := max(fo, fl↑(2^-438·ê_fo))` and `mo := max(mo, fl↑(2^-438·ê_mo))`. fl↑ is decided exactly as b is: r = ê·2^-438 to nearest, then next_up(r) when r·2^438 < ê.
> - The constant 2^-438 has bits `0x2490000000000000`. Φ is exact when ê ≥ 2^-584, because the product is then normal.
> - Items 7 and 8 then use the updated fo and mo. For p = 128 and 256, item 6a changes nothing.

### 5.4 §4.1.6.2 (new): the formation scale E

> 1. **Bounded operator.** Axis components → 1; bending-row translation entries of B → 1/L (as formed); other B entries → 1; |D| and 1/L as formed.
> 2. **g_m = 2^k,** where k is the least integer ≥ 0 with 4^k·(y_c·y_c) ≥ (y_ref·y_ref). y_c is the Gram–Schmidt residual of the same formation, y_ref the source's; both dot products are exact, and the comparison is exact.
> 3. **Ā** is the assembled bounded operator: per entry, one exact sum, rounded once, of:
>    - element blocks g_e·B̄ᵀ(|D|B̄), each factor product rounded once;
>    - |k| of global-axis springs;
>    - directional blocks' formed entries in absolute value, entrywise.
> 4. **E_q** for every force or moment quantity is its recovery expansion evaluated with B̄, Ā and absolute operands. Each stage is one exact sum of exact two-factor products, rounded once to nearest at the state's precision P, as §3.1 of the addendum lists:
>    - end actions × g_m; stations × g_m; spring actions |k||u|;
>    - directional actions Σ|k_ab||u_b|;
>    - reactions |f_c| + ΣĀ_cj|u_j|, with the exact ledger net f_c;
>    - support groups: the sum of their components' E.
>
>    The operands are |u| at every DOF, prescribed values included as rounded at P. **E(body, kind)** is the maximum over the body's force (moment) rows, unpublishable rows included. It is formed from the **verification** state and rounded upward once to binary64 for the receipt. An E that overflows makes the entry `unavailable` (`receipt_encoding`).
> 5. **Resolution bound** (derived to first order; exact form ≤ 69·2^-P·E_q under §4.1.6.3 item 10's g check, by §4.1.6.3 Lemma B): |q_P − R\*(u_P)| + |a_q|ᵀ|δu_rep| ≤ (20g + 48)·2^-P·E_q/g ≤ 68·2^-P·E_q. **λ = 2^8.**
> 6. **Role.** E serves the verification. When a 2p state becomes the next candidate, the comparison against 4p uses the E of the 4p verification (V4-N11).

### 5.5 §4.1.6.3 (new): the verification estimate and the formation charge (S2; V4-R1, V4-R2; made rigorous in R4 for V4-T1)

> At the verification state (precision P = 2p), after its residual gate has passed. **Every norm below is taken over the free DOFs of the row's own body.** K is block-diagonal by body, because springs and directional blocks do not connect bodies (§4.1.6.1 item 1).
>
> 1. **Residual.** Take the final state, the state the gate selected (§4.1.4 step 3). For every free row i form
>
>    `r_i = f_i − Σ_e Σ_j k(e)_ij·u⁰_j − Σ_s k(s)·u⁰_i − Σ_d Σ_j k(d)_ij·u⁰_j`
>
>    as **one exact expansion over the contributions**, where:
>    - f_i is the exact ledger net. W1a's ledger is exact: nodal forces and moments in binary64;
>    - k(e)_ij and k(d)_ij are element and directional-block entries **formed at q_W = min(3p + 64, 1024)** (R4; ROOT's ruling 1). That is 448 bits at p = 128 and 832 at p = 256. At p = 512 it is 1024, the state's own entries (Q4);
>    - k(s) is each global-axis spring's binary64 stiffness;
>    - u⁰ is the final state with every prescribed value at its **exact** value (R4): a single case's binary64 value, or a combination's exact expansion Σ_k λ_k·u_k,c. K4 already carries those terms to publish the prescribed row once rounded (K4-M33).
>
>    Nothing is summed into an assembled entry first, and nothing is reused from the gate.
> 2. **Correction.** δ̂ = K_P⁻¹·r̂, where r̂_i is r_i rounded once to P, with one scaled substitution pair using the state's factor. δ̂ = 0 at constrained DOFs.
> 3. **Estimate.** Ŵ_q = |L_q(δ̂)| for every published force or moment q. L_q is q's recovery at P applied to δ̂ with the ledger omitted, as in R3.
> 4. **Test.** Ŵ_q ≤ 2^(6−2p)·ê(body, kind) (= V_q/4), with item 6a's ê (ROOT confirmed V/4 at `e5f4ef3a8`).
> 5. **The correction's own residual (R4).** r₂_i = r_i − Σ_j K^c_ij·δ̂_j over the same contributions, as one exact expansion.
> 6. **Norms (R4).** S = diag(s_i) is K4's radix equilibration of the free DOFs, the powers of two the factor already uses. Ā is E's assembled bounded operator at P (§4.1.6.2 item 3). ā_q is q's bounded recovery row: the coefficients of E_q's expansion, ledger omitted. Then:
>    - ‖SĀS‖₁ = max_j s_j·Σ_i s_i·Ā_ij;
>    - ‖SĀ|u⁰|‖_∞ = max_i s_i·Σ_j Ā_ij·|u⁰_j|, over every column j: the pass that forms E's reaction rows (§4.1.6.2 item 4), extended to the free rows;
>    - ‖S·r‖_∞ and ‖S·r₂‖_∞;
>    - ‖S⁻¹δ̂‖_∞ = max_i |δ̂_i|/s_i;
>    - ‖ā_q S‖₁ = Σ_j ā_qj·s_j, which is E_q's expansion evaluated with s_j in place of |u_j| at free DOFs and 0 at constrained ones.
> 7. **est.** The verification's Hager–Higham estimate of ‖K̃_P⁻¹‖₁, where K̃_P = S·K_P·S, exactly as its condition screen forms it (§4.1.3). No new solve is needed. est estimates the whole matrix's norm, which bounds each body's block, so one est serves every body.
> 8. **F (pinned; R4).** **F = 2^16**, bits `0x40F0000000000000`. It is the allowance in the one uncertified relation this design uses:
>
>    **‖K̃_P⁻¹‖₁ ≤ F·est.**
>
>    It is pinned here and named in §4.1.3 (§5.9). r5a.2 had no numeric F. The candidate, R2 and R3 used this relation with the allowance unnamed: the condition screen (§4.1.3) compares est with 2^(p−1), and W's accuracy argument read the screen as κ < F·2^(p−1). R4 gives that allowance a value and uses it in items 9 to 12. The screen's threshold is unchanged.
> 9. **θ (R4).** θ = F·est·2^(7−P)·max over bodies of ‖SĀS‖₁. Test: θ ≤ 1/2.
> 10. **g (R4).** Every member's g (§4.1.6.2 item 2, formed at P) satisfies g ≤ 2^(P−16). This makes §3.3's validity condition a check.
> 11. **The charge (R4; V4's closure, ROOT's ruling 2).** For every published force or moment row q:
>
>     `C_q = ‖ā_q S‖₁·(t₁ + t₂ + t₃)`
>
>     - t₁ = 2^(7−q_W)·F·est·N_u, with N_u = ‖SĀ|u⁰|‖_∞ + 2F·est·‖SĀS‖₁·‖S·r‖_∞;
>     - t₂ = 70·2^-P·‖S⁻¹δ̂‖_∞;
>     - t₃ = 3·F·est·‖S·r₂‖_∞.
>
>     **Test,** decided exactly (C_q formed as one exact expansion rounded upward):
>     - at p = 128 and 256: C_q ≤ 60·2^-2p·ê(body, kind);
>     - at p = 512: C_q ≤ 2^-22·2^-64·M_q, with M_q the stop rule's max(|q_2p|, S\*).
>
>     t₁ is V4's term 62.5·2^-q_W·F·est·‖a_q·S‖₁·‖S·Ā·|u|‖_∞, with the factor 2 from item 9's Neumann bound and the γ-margins of Lemma B below. t₂ and t₃ are the second-order remainder, measured at runtime.
> 12. **Translation and rotation rows (R4).**
>     - For a free displacement or rotation row i: W⁺_i = |δ̂_i| + s_i·(t₁ + t₃).
>     - For a displacement magnitude: the sum of its components' W⁺. A constrained component contributes its prescription's rounding, |u_P,c − u\*_c|, which is computed exactly.
>     - §4.1.6's rule (a) takes V_q = W⁺_q for these rows, plus 2^(1−2p)·|q_2p| for a magnitude (its own two roundings).
> 13. **Failure.** Items 4, 9, 10 and 11 reject p with the reasons `verification_estimate`, `theta`, `g_validity` and `charge`. The schedule escalates exactly as after a stop-rule rejection. At the ceiling the case is Unresolved (Ceiling).
> 14. **Evidence.** Per body and kind, the worst Ŵ_q/V_q and the worst C_q/allowance, rounded upward; θ; and F's bits. G5a checks that they decode to ≤ 2^-2, ≤ 1, ≤ 1/2 and exactly 2^16 (§6.3).
> 15. **Work (charged to the verification attempt).**
>     - One formation pass of the element and directional-block entries at q_W. The gate's K^q at P + 64 stays.
>     - Two exact expansions over the contributions: r and r₂.
>     - One pass over Ā's rows: ‖SĀS‖₁ and ‖SĀ|u⁰|‖_∞.
>     - One E-like pass with operands s: ‖ā_q S‖₁ for every row.
>     - One substitution pair and one linear recovery pass.
>
>     est and S are the condition screen's and the factor's, already formed.
>
> **What W, C and W⁺ bound (derived modulo F; R4, V4-T1).**
>
> *Notation.* 𝓕 and 𝓒 are the free and constrained DOFs. K\* is the exact intended stiffness, f\* the exact ledger, and u\* the exact solution with the exact prescribed values u\*_𝓒 = u⁰_𝓒. K_P is the assembled stiffness at P. K^c is the exact, unrounded sum of the q_W-formed contributions. Δ = K^c − K\*, on free rows and every column. ũ solves K^c_𝓕𝓕·ũ_𝓕 = f\* − K^c_𝓕𝓒·u⁰_𝓒 with ũ_𝓒 = u⁰_𝓒. a\*_q is q's exact recovery row. Then, exactly, r = K^c_𝓕𝓕·(ũ_𝓕 − u_𝓕) and r₂ = K^c_𝓕𝓕·(ũ_𝓕 − u_𝓕 − δ̂).
>
> **Lemma A (derived; V4's bound on Γ_q, V4-T1(b)).** For symmetric invertible K̃ = S·K·S:
> - |a·K⁻¹·v| ≤ ‖a·S‖₁·‖K̃⁻¹‖₁·‖S·v‖_∞;
> - |(K⁻¹·v)_i| ≤ s_i·‖K̃⁻¹‖₁·‖S·v‖_∞.
>
> *Proof.* K⁻¹ = S·K̃⁻¹·S. Apply Hölder's inequality, and ‖K̃⁻¹‖_∞ = ‖K̃⁻¹‖₁ by symmetry. The same bound holds for \|a\|·\|K⁻¹\|·v with v ≥ 0, since \|K⁻¹\| = S·\|K̃⁻¹\|·S.
>
> With a = a_q and v = Ā·|u| this is V4's Γ_q ≤ ‖a_q S‖₁·‖K̃⁻¹‖₁·‖S·Ā·|u|‖_∞. **It replaces R3's statement that the rcond screen does not bound Γ_q.** The screen's estimate does bound it, through ‖K̃⁻¹‖₁ ≤ 2F·est (Lemma C).
>
> **Lemma B (derived: V4's count in its exact γ-form, under item 10).**
> - Each counted chain is a product of at most c factors (1 + δ) with |δ| ≤ 2^-P, relative to a nonnegative majorant. The exception is the Gram–Schmidt subtraction, whose absolute error, relative to |y_ref|, the count converts to relative error with g.
> - By Higham's Lemma 3.1, such a product lies within γ_c = c·2^-P/(1 − c·2^-P) of 1. The normalization by the cancelled vector adds one factor (1 − g·c·2^-P)⁻¹.
> - With 68·g·2^-P ≤ 2^-9 (item 10), each exact error is at most its first-order count times 1 + 2^-7.
>
> Hence, entrywise:
> - (i) |K\*_ij − K_P,ij| ≤ 67·2^-P·Ā_ij. The count is 20g + 45.5 per assembled entry, so ≤ 65.5 relative to Ā, which carries g ≥ 1. The γ-factor and Ā's own formation at P (less than a relative 2^-(P−8)) raise 65.5 to at most 66.1;
> - (ii) |Δ_ij| ≤ 63.5·2^-q_W·Ā_ij, the count 62.5 at q_W in the same way;
> - (iii) |a\*_q| ≤ ā_q·(1 + 2^-(P−8)) entrywise. The recovery at P differs from the exact recovery of the same state with exact prescriptions by at most 69·2^-P·E_q; the count is 68, and E_q includes |u| at prescribed DOFs.
>
> **Lemma C (derived modulo F, under item 9).** ‖(S·K\*_𝓕𝓕·S)⁻¹‖₁ ≤ 2F·est and ‖(S·K^c_𝓕𝓕·S)⁻¹‖₁ ≤ 2F·est.
>
> *Proof.*
> - Write both matrices as K̃_P + E. By Lemma B, |E| ≤ (67·2^-P + 63.5·2^-q_W)·SĀS ≤ 2^(7−P)·SĀS entrywise.
> - With ‖K̃_P⁻¹‖₁ ≤ F·est, ‖K̃_P⁻¹E‖₁ ≤ θ ≤ 1/2.
> - So I + K̃_P⁻¹E is invertible with inverse norm ≤ 2 (Neumann series), and ‖(K̃_P + E)⁻¹‖₁ ≤ 2F·est. ∎
>
> This is the second-order control ROOT named: ‖K⁻¹δK‖ ≤ 1/2, with a factor of 2.
>
> **Theorem (derived modulo F).** Under items 9 and 10:
> - for every published force or moment row q: |q_2p − q\*| ≤ 69·2^-P·E_q + Ŵ_q·(1 + 2^-P) + C_q;
> - for every free translation or rotation row i: |u_2p,i − u\*_i| ≤ W⁺_i.
>
> *Proof.*
> 1. q_2p − q\* = [R_P(u_P) − R\*(u⁰)] + a\*_q,𝓕·(u_𝓕 − u\*_𝓕). The bracket is ≤ 69·2^-P·E_q by Lemma B(iii).
> 2. u\*_𝓕 − u_𝓕 = x̃ + y, with x̃ = ũ_𝓕 − u_𝓕 = (K^c_𝓕𝓕)⁻¹·r and y = u\*_𝓕 − ũ_𝓕 = (K\*_𝓕𝓕)⁻¹·Δ·ũ. Both are exact: subtract the two systems.
> 3. x̃ − δ̂ = (K^c_𝓕𝓕)⁻¹·r₂ exactly. By Lemmas A, B(iii) and C, |a\*_q·(x̃ − δ̂)| ≤ ‖a\*_q S‖₁·2F·est·‖S·r₂‖_∞ ≤ ‖ā_q S‖₁·t₃.
> 4. |a\*_q·δ̂| ≤ |L_q(δ̂)| + 69·2^-P·ā_q·|δ̂| (Lemma B(iii) applied to δ̂) ≤ Ŵ_q·(1 + 2^-P) + ‖ā_q S‖₁·t₂ (Hölder).
> 5. By Lemmas A, B and C, |a\*_q·y| ≤ ‖a\*_q S‖₁·2F·est·63.5·2^-q_W·‖S·Ā·|ũ|‖_∞. Since |ũ_𝓕| ≤ |u_𝓕| + |x̃| and ‖S⁻¹x̃‖_∞ ≤ 2F·est·‖S·r‖_∞, we have ‖S·Ā·|ũ|‖_∞ ≤ N_u. So |a\*_q·y| ≤ ‖ā_q S‖₁·t₁.
> 6. Adding steps 3 to 5 gives the force and moment bound.
> 7. For a free row i, steps 3 and 5 without the recovery row give |u_2p,i − u\*_i| ≤ |δ̂_i| + s_i·2F·est·‖S·r₂‖_∞ + s_i·127·2^-q_W·F·est·N_u ≤ W⁺_i. ∎
>
> The constants 70, 3 and 2^7 exceed 69, 2 and 127. That slack covers the stage rounding of ‖ā_q S‖₁ and each 1 + 2^-(P−8) factor.
>
> **Corollary: the guarantee (derived modulo F).**
> - **p = 128 and 256.** Acceptance gives |q_p − q_2p| ≤ 2^-64·M_q − V_q, Ŵ_q ≤ V_q/4 and C_q ≤ 60·2^-2p·ê. With E_q ≤ ê and V_q = 256·2^-2p·ê, |q_p − q\*| ≤ 2^-64·M_q − (256 − 69 − 64·(1 + 2^-2p) − 60)·2^-2p·ê < 2^-64·M_q − 62·2^-2p·ê.
> - **p = 512.** C_q ≤ 2^-86·M_q gives |q_p − q\*| ≤ 2^-64·M_q·(1 + 2^-22).
> - **Translation and rotation rows.** Rule (a) with V_q = W⁺_q gives |q_p − q\*| ≤ 2^-64·M_q.
> - **So b bounds |q_p − q\*| for every `absolute_verified` row,** with the factor 1 + 2^-22 at p = 512, and the relative claim holds for every relative row. This is **modulo F**: the one step not derived is ‖K̃_P⁻¹‖₁ ≤ F·est. No step is first order: the counts enter in their exact γ-form (Lemma B), and the second-order remainder is bounded by Lemma C and measured by t₂ and t₃.
>
> **Measured** (emu4; `run_controls4`, `gate_probe5`, `est_check4`, §14):
> - **At every selected pair of the 44 R3 controls,** the worst C_q/allowance is 4.7e-29 (SKEW-K1E-12 at 128), a margin of about 2^94, and θ ≤ 1.5e-46. On the 20 probe cases the worst is 9.0e-45 and θ ≤ 1.8e-62.
> - **CHARGE-SLENDER** (§7): C_q/allowance = 102 at 128, so it is refused there; at 256 it is 3.0e-37, and it is selected at 256. It is honest at 128 as well: the charge is conservative, not a detector of a false claim.
> - **The 2,000-frame sweep:** the worst charge at a selected pair is 0.0093 of its allowance, in R790 at 128, whose moduli span about 2^200 (est ≈ 2^106 at P = 256). It passes with a margin of 2^6.7. θ ≤ 3.0e-33. No frame changes precision, class or rejection reason against R3 (`sweep4_summary.json`).
> - **K4's R1 lane:** the worst charge is 9.9e-37 of its allowance, and the 120 cases are unchanged.
> - **LEVER2 at 256:** C_q/allowance of 1.0e84, 1.1e90 and 1.2e102, so the charge alone would also refuse it (W refuses it first).
> - **TILT-LEVER at 256:** C_q/allowance of 2.5e34 to 2.6e40 and θ of 1.5e15 to 1.5e21; the stop rule refuses it first.
> - **est against the exact ‖K̃_P⁻¹‖₁:** over 195 verification states of the controls and the probe set (every model with at most 40 free DOFs; the three with none are skipped), exact/est lies between 1 − 8·10^-9 and 1.0 (`est_check4.stdout.json`). The estimate is exact to rounding on these small matrices. F = 2^16 leaves 16 bits for what emulation cannot show. The estimator has no proved bound: matrices are known on which the basic algorithm underestimates badly, which is why the alternating-sign vector, used here as in K4, was added. F is therefore a pinned allowance, not a proved constant, and it is the one reliance §9 states.
>
> **Withdrawn or replaced from R3.**
> - "The rcond screen … does not bound Γ_q" is replaced by Lemma A with Lemma C (V4-T1(b)).
> - R3's conjecture on e_q is withdrawn: e_q is now charged (t₁) and bounded (Lemma C).
> - R3's citation of V4's statement that only assembly-level sums can saturate in an exactly representable structure is dropped (V4-T2). V4's erratum: TILT-LEVER's member is formed exactly at every P ≤ 576, yet its (uy,uy) entry carries an axial (t/L)² tail lost within the entry. DS1's own example stands (direction components differing by more than 2^(p+32)).
> - The 3p + 64 option is now specified text (item 1).
>
> **Neither W nor C covers:**
> - the common-mode loss of exact inputs, which the exact sums remove;
> - W1b's formed ledger terms (F3; §6.5).

### 5.6 §4.1.4: amend step 3 (S3; ROOT: O12 reversed, the hybrid form confirmed; best state per V4-R5)

> 3. **Gate** (revision 5a.3). For each free row, with the exact r_i at q:
>    - the **coalesced** denominator d_i^c = |f_i| + Σ_j |K^q_ij·u_j| (M03's, as today);
>    - the **bounded** denominator d_i^b = |f_i| + Σ_j Ā^q_ij·|u_j|, where Ā^q is §4.1.6.2 item 3 formed at q, at contribution level;
>    - m_i as today.
>
>    Each is one exact expansion, not rounded before the test.
>    - **Refinement is driven as today by d^c.** If every row passes |r_i|(2^p − m_i) ≤ 64·m_i·d_i^c, the state passes. Otherwise it is corrected, at most three times, stopping early when the worst coalesced ratio does not decrease.
>    - **When refinement ends without the coalesced test passing,** the gate takes the **best state evaluated** (R3, V4-R5): among the states whose residuals it formed (the first solve and each correction), the one with the smallest worst bounded ratio max_i \|r_i\|(2^p − m_i)/(64·m_i·d_i^b), compared exactly, the earliest on a tie. That state passes if every row passes the same test with d_i^b, and it becomes the final state. Otherwise the attempt fails (ResidualGate) and escalates.
>    - At the ceiling (q = 1024, K^q = K_1024) the same rule applies.
>    - The gate's residual and both denominators stay over K^q's and Ā^q's assembled entries (see below).

**The form.** ROOT ruled that the gate's denominator becomes the bounded operator, and at `e5f4ef3a8` confirmed this hybrid form, on V4's derivations: the acceptance test uses the bounded denominator, and refinement's driver stays coalesced, because the pure form (mutant M13) removes TWO-SPAN's one correction, the evidence of O5's kill of K4-M11 (point 4 below).

**Why the best state (V4-R5; availability only).** Refinement stops after a correction whose coalesced ratio did not decrease, so the last state can be the worse one. Every evaluated state's rows already carry d^b, so choosing the best costs nothing. Measured: on the 8 single-mode probe cases the chosen state is the second, third or fourth of four (`gate_probe4.stdout.json`). The last state also passes there, so no control distinguishes the two rules (mutant M16 survives; §7).

**Why it is sound (derived; the measurements are named).**
1. **Ā^q ≥ \|K^q\| entrywise,** up to a relative ≤ 2^-(q−8). Each assembled entry is Σ_e fl(Σ_r B_ra·DB_rb) with \|B\| ≤ B̄ entrywise (axis components ≤ 1, \|1/L·e\| ≤ 1/L) and g ≥ 1. So the bounded test admits every state the coalesced one admits.
2. **The residual of K_p's formation is inside the bounded test.** After a solve with the p-factor, r ≈ (K^q − K_p)·u. That is K_p's formation error. By V4's count it is ≤ (20g + 45.5)·2^-p·(B̄ᵀ\|D\|B̄ summed)·\|u\| ≤ 65.5·2^-p·Ā\|u\|, because Ā carries the factor g ≥ 1. That is within 64·m_i·2^-p·d_i^b whenever m_i ≥ 2, and m_i = 2·(terms) + 2 ≥ 2 always (first order). The coalesced d^c has no term of that size in a row whose true entries multiply zero or noise-formed components: V4-S3, whose ratio of about 6e15 does not depend on p. The bounded d^b contains it by construction, so the gate no longer refuses states whose only residual is formation noise.
3. **Honesty does not rest on the gate.** Acceptance of a published value rests on the stop rule, V, the verification estimate and the charge (§4.1.6.3). The estimate and the charge judge the final state, whatever the gate tolerated. The gate is a convergence screen. Its looser acceptance can only move states to the stop rule, and at the verification the estimate and the charge then apply.
4. **Refinement stays driven by d^c.** Corrections toward K^q's solution are made where they are made today:
   - K4-M11's evidence-level control, TWO-SPAN, keeps its one correction (measured);
   - today's correction counts and golden work are unchanged wherever today's gate passes.

   The pure form (d^b driving refinement too, mutant M13) drops TWO-SPAN's correction (measured: 1 correction under the coalesced and hybrid gates, 0 under the pure form; `mutants3.stdout.json`, and again in `mutants4.stdout.json`) and would break O5's kill of K4-M11.

**Does the gate's own residual need the contribution-level form of §4.1.6.3? No (derived).**
1. **Nothing published is decided by the gate's residual.**
   - The stop rule reads the candidate's and the verification's recovered values.
   - The estimate and the charge form their own residuals, at contribution level on the final state, at q_W (§4.1.6.3 items 1 and 5).
   - The gate decides only whether an attempt proceeds and which evaluated state is final.
2. **Whatever state the gate passes, W measures what the gate's residual misses.** If K^q's assembled entries lose a contribution, refinement converges toward K^q's solution. Then r_c = f − K^c·u_P = (K^q − K^c)·u_P + (f − K^q·u_P), so the lost contribution enters W's residual and W measures its effect. On LEVER2 the gate passes the 256 state with its residual 0, and W rejects it with W/V = 2,681 (measured).
3. **Availability is unchanged except within 2^-71 of the gate's boundary.**
   - Each assembled entry of K^q is its contributions' exact sum rounded once at q, so \|r_q − r_c\| ≤ 2^-q·Σ_j \|K^c_ij\|·\|u_j\|, which is ≤ 2^-q·d_i for either denominator.
   - The gate's tolerance is 64·m_i·2^-P·d_i, with m_i ≥ 2 and q = P + 64.
   - So the assembled form moves the test by at most a relative 2^-71 of its tolerance.
   - A lost contribution, such as LEVER2's spring, is itself below 2^-q of its entry and is inside that bound.
4. **The assembled form is the cheaper one.** The gate forms up to four residuals per state, and the assembled form has one term per nonzero entry rather than one per contribution. Ā^q is only a denominator, so its assembly rounding (relative 2^-q) is immaterial.

**Effect on every control** (measured in emulation: R3's `run_controls3`, `gate_probe4`, `r1_lane3` and `sweep3`; unchanged under R4 by `run_controls4`, `gate_probe5`, `r1_lane4` and `sweep4`). Every probe result below is **emulation only** (V4-R3): K4's Rust probe runs only the coalesced gate, and K4's Rust control for the hybrid gate is not yet built.
- **K4's Rust probe cases,** rebuilt in emu3 and emu4: a (3,4,0) run of 1 or 3 N-section members, root fixed, five tip loads, y_ref (3,4,5) or (0,0,1).
  - Under the coalesced gate the emulator refuses the same 8 single-mode y_ref (3,4,5) cases as Rust (Unresolved), with gate ratios 1.7e15–2.2e16. The general load and every y_ref (0,0,1) case are selected at 128, as in Rust.
  - One difference: in the one-member in-plane case, Rust (RETURN §21) and V4's emulator fail the coalesced gate at 256, while emu2, emu3 and emu4 pass that state and then fail its 512 verification. The outcome (Unresolved) is the same; the difference is in the emulation of today's gate only.
  - Under the hybrid gate **all 20 are selected at 128, honest, with 0 G5a failures,** as V4 reproduced independently. The 8 single-mode cases pass the bounded test on their best state after the three coalesced-driven corrections; the other 12 pass the coalesced test with none.
- **V4's cantilever:** LOADONLY-y345 is selected at 128.
- **The GS cases:** GS-TRANS-y345 and GS-ROT-y345 are selected at 512, and GS-ROT-y345-LOADED at 128. **M10-ANISO:** 512.
- **No other control changes.** All 29 of the candidate's controls and V4's two keep their precision and classes, under R2 and R3 alike.
- **K4's R1 lane:** 120 of 120 cases identical, under today's rule, R2 and R3.
- **Sweep** (R2's figures, against the floor with today's gate): 35 frames that it leaves unresolved are selected (20 at 128, 8 at 256, 7 at 512), 7 are selected at a lower p, no frame changes class at the same p, and none loses a relative row. **R3 changes none of them:** every frame's selected precision, classes and rejection reasons equal R2's. W/V changes in 186 frames, mostly at the 1024 verification, where W's residual now includes K_1024's assembly roundings, which Q4 had left unmeasured. The largest W/V stays 0.0048 (`sweep3_summary.json`).

### 5.7 §4.1.9: replace the section

> #### 4.1.9 What the stop rule can and cannot see (revision 2; corrected in revision 5a.3)
> - **The stop rule observes only the part of the error that differs between p and 2p.**
> - **Saturation (derived).** A value x = y + t rounded once, with y a p-bit number and 0 < |t| < 2^-2p·|y|/2, rounds to y at p and at 2p (Lemma 2). Its tail is lost identically in both candidates. The error is precision-dependent (a larger precision resolves it), yet invisible. Its effect on q is at most Δ_2p(q) = Σ_k |∂q/∂x_k|·2^-2p·|x_k|/2, to first order. The sites are u itself, the reduced right-hand side, K's assembled entries, a combination's rounded prescription, and every recovery stage.
> - **Revision 2's claim that only precision-independent (common-mode) error can pass is withdrawn.** The common-mode loss of an exact input is the special case where t is an input tail. Revision 2's exact sums still remove it, and they stay required.
> - **What revision 5a.3 adds.**
>   - V charges the verification's recovery resolution (§4.1.6.2).
>   - The verification estimate measures its solve error, saturated tails and saturated sums of contributions included (§4.1.6.3).
>   - The formation charge bounds what the estimate cannot measure, and θ bounds the second-order remainder (§4.1.6.3, R4).
>   - The ceiling floor bounds what 1024 cannot resolve (§4.1.6.1 item 6a).
>
>   The premise that the verification's error is small against the stop rule's tolerance is then derived, beyond first order, **modulo F**: the one uncertified step is ‖K̃_P⁻¹‖₁ ≤ F·est, with F = 2^16 (§4.1.6.3 Corollary).
> - **Negative controls:** F-2, F-2 with S\* > 0, the prescribed tail, F-2 at the ceiling, ASSEMBLY-SAT, F-3 (three forms), DEMOTION2, SEEDED-COMMON, LEVER2 (three gains), TILT-LEVER (three gains) and CHARGE-SLENDER join §7.3.
> - **In the factorization and triangular solves,** non-saturated error changes at 2p and the stop rule sees it. The refinement residual at p + 64 checks the solve against the exact-expansion right-hand side.
> - **The same exact-sum rule holds on the ordinary binary64 route** through S11-K and S11-F, as before.

### 5.8 §5 item 1 (the receipt): add to a `selected` entry

> - **`resolution_scale`** (revision 5a.3): per body, `force` and `moment` bit strings: E_fo and E_mo of §4.1.6.2, uncoupled, rounded upward, finite, +0.0 only when zero. Likewise in a combination's entry.
> - **The stop-rule summary** is the worst (|q_p − q_2p| + V_q)/M_q, rounded upward, and decodes to ≤ 2^-64.
> - **`verification_estimate`:** per body and kind, the worst W_q/V_q, rounded upward, decoding to ≤ 2^-2.
> - **`verification_charge`** (R4): per body and kind, the worst C_q divided by its allowance, rounded upward, decoding to ≤ 1. Also θ, rounded upward and decoding to ≤ 1/2, and F's bits, `0x40F0000000000000`.

### 5.9 §4.1.3 "Condition estimate": add one sentence (R4)

> **The allowance F (revision 5a.3, R4).** Every use of the estimate in the honesty argument relies on ‖K̃_p⁻¹‖₁ ≤ F·est with **F = 2^16**, where K̃_p = S·K_p·S is the radix-equilibrated assembled matrix whose factor the screen uses: the retained-precision route's θ check, formation charge and translation bound (§4.1.6.3 items 8 to 12). This is the design's one uncertified step. The screen's threshold, rcond ≤ 2^-(p−1), is unchanged.

## 6. Effects downstream

### 6.1 The discriminating controls (§7.3, §4.10)

Measured with R4's rule (bounded-gate acceptance on the best state, the contribution-level estimate at q_W, the charge, θ and the g check), against K4's rule and gate. Every selected case is honest. **Every control keeps R3's precision and classes**, except CHARGE-SLENDER, which exists to be charged out at 128.

| Control | Kills | Outcome |
|---|---|---|
| k = 1e-28 | D5, D6; M18 (R4) | 128 fails the condition estimate; 256 — unchanged. With the charge at q = 2p + 64 (M18) it moves to 512 |
| Six-member k = 1e-12 | K4-M14; SD-G1 | 128 rejected on Rx; 256 — unchanged |
| B1-L | D13 | 128 — unchanged; the loads act at free DOFs, which do not enter E |
| S8-W (both) | D17, D20; SD-J2 | 128 — unchanged |
| PIVOT | M18 (R4) | 256 — unchanged; 512 under M18 |
| REACTIONS-ONLY | K4-M16; SD-G4 | 128 rejected by R(0, Uy); 256 — unchanged |
| TWO-SPAN | K4-M11 (O5) | 128 with one correction — unchanged |
| LEVER2 (gains 2^90, 2^100, 2^110; V4-R1) | M15 (M11 and M14 no longer: the charge also refuses LEVER2 at 256, and at q_W = 832 even assembled entries resolve its spring) | **Unresolved (safe):** 128 fails the pivot test, 256 is rejected by the estimate (and would be by the charge, at 1e84 or more of its allowance), 512 by the stop rule (1024 resolves the spring). Today and R2 select 256 with false claims |
| TILT-LEVER (gains 2^90, 2^100, 2^110; V4-T1(c)) | — | **Unresolved (safe)**, today and under R3 and R4: the stop rule rejects 256 and 512 (Δ ≠ 0 at the leak site). Under R4 the charge and θ would also refuse 256 |
| CHARGE-SLENDER (R4) | M17, M19 | **256**: the charge alone refuses 128 (102 times its allowance), the only control whose precision R4 changes. Without the charge, or with F = 1, it is selected at 128. It is honest at 128 as well: the charge is conservative |
| N05, N06, SKEW-\*, AXIS, OBLIQUE, DUPLICATE, PRESCRIBED, ZERO-TORSION-345, ALL-ZERO-BODY, MIXED, SKEW-K1E-60 | — | Unchanged precision and classes |
| RF-CHAIN, RF-SKEW, RF-WEAK, RF-FINITE, RF-CANCEL (nodal), RF-MECH-LINE345-RX-COMPANION | Their negative controls | 120 of 120 unchanged precision (all 128) and classes, under R2, R3 and R4 (`r1_lane4.stdout.json`); largest W/V 0.0054; worst charge 9.9e-37 of its allowance; 0 G5a failures |

**R1 formation ratios** (N9; `r1_ratios2.stdout.json`): the largest log2(ê/S\*_c) from R1's own displacements, per family.

| Family | Max log2(ê/S\*_c) |
|---|---|
| RF-CHAIN | 51.1 |
| RF-SKEW | 51.6 |
| RF-WEAK | 50.0 |
| RF-LARGE (≤ 200 members) | 32.2 |
| RF-INVARIANCE | 32.6 |
| RF-RANGE | 37.7 |
| RF-ZERO | 10.8 |
| RF-FINITE | 13.6 |
| RF-MECH (the non-generator cases) | 4.6 |
| RF-CANCEL | 10.1 |

So V/(ε·M) ≤ 2^(51.6−184) at 128, and no R1 case can reach the floor.

**Not measured:**
- RF-LARGE with 1,000 or 10,000 members;
- the 7 generator cases (RF-LARGE n10000 and RF-MECH-LINE-IN-CHAIN1000);
- RF-RANGE-THIN-A and THIN-B (their section format is outside DS1's adapter).

**No discriminating control stops discriminating.** R1's not-covered set (RF-WEAK 46, RF-CANCEL 3, RF-SKEW 2) is unchanged.

### 6.2 K4's SD-tagged tests (ROOT's list)

| Tag | Under the revision |
|---|---|
| SD-G1, SD-G2, SD-G3, SD-G4, SD-G6, SD-I1, SD-I2, SD-J2, SD-K1 | **Unchanged** (measured on the emulated controls, or derived: SD-G3 and SD-I2 have E = 0, so V = 0, Φ = 0 and W = 0) |
| SD-G5 | **Translation vectors change only by W⁺** (R4: V_q = W⁺_q for translation and rotation rows; 0 when the state is exact). Add:<br>• force-kind boundary vectors at \|Δ\| = ε·M − V (accepted) and one 2p-ulp above (rejected);<br>• an estimate boundary at W = V/4 and one ulp above;<br>• the gate's bounded test at its boundary, on a best state that is not the last;<br>• an estimate residual whose assembled and contribution-level forms differ (LEVER2's shape);<br>• R4: charge boundaries at C_q equal to its allowance and one ulp above, at p = 256 and p = 512; θ = 1/2 and one ulp above; g = 2^(P−16) and 2^(P−15); a translation row at \|Δ\| + W⁺ = ε·M |
| SD-L1 | **Changed:** the E pass, the estimate and the charge are charged to the verification: the q_W formation pass, two residual passes over the contributions (r and r₂), one pass over Ā's rows, one E-like pass with operands s, one substitution pair and one recovery. The gate adds no correction where today's gate passes, so other counts are unchanged. Re-pin, with the E pass, the estimate and the charge as their own stages |
| SD-J1 | **Values unchanged** for vectors with p ≠ 512 or E = 0. The format gains (selected p, E_fo, E_mo). Add item-6a vectors: Φ binding and not binding, the fl↑ boundary, ê < 2^-584 |

### 6.3 D2 (r5b.2)

**Fields.** `resolution_scale`, `verification_estimate` and, in R4, `verification_charge`, θ and F in the receipt (§5.8).

**E is producer-attested.** It comes from the unpublished 2p state. It is:
- covered by `receipt_sha256`;
- recomputed by the Rust replay audit (§4.1.8);
- checked by readers as follows.

**G5a (revision 5a.3).** In binary64, pinned, and identical in the three languages. A mismatch is `RETAINED_PRECISION_SCALE_MISMATCH`.
1. **Shape:** `resolution_scale` is present for every body with a force or moment row; the bits decode to finite values ≥ +0.
2. **Zero rule (derived):** E_k = 0 ⇒ every published row of kind k in that body is +0.0, because E_q = 0 forces q = 0.
3. **Sanity bound (N10):** fl(ê_k·c) ≥ S\*_c,k, with c = 1 + 2^-40 (bits `0x3FF0000000001000`) and S\*_c the item-6 coupled published scale.
4. **Lower bound (S7; V4's, with a derived margin).** Every input is taken **after §4.1.6.1 item 3's unit conversion** (V4-R6): the published u and S\*_tr, S\*_ro in m, rad, N and N·m, each conversion one binary64 operation; k_a and k_t in N/m and N·m/rad as the receipt carries them; E in N and N·m. For each member m of the body:
   - Nt(n) = fl(fl(|u_x| + |u_y|) + |u_z|) over the published translation rows at node n, `input_derived` included; Nr(n) likewise over rotations;
   - N_t = fl(Nt(i) + Nt(j)) and N_r = fl(Nr(i) + Nr(j));
   - LB_fo(m) = 0 if N_t ≤ 2^-59·S\*_tr, else fl(k_a·fl(N_t − 2^-60·S\*_tr)); LB_mo likewise with k_t, N_r and S\*_ro.
   - Check fl(ê_fo·c) ≥ max_m LB_fo(m) and fl(ê_mo·c) ≥ max_m LB_mo(m).

   *Derivation* (an honest E passes):
   - E_fo ≥ the axial end row's E_q = g·(EA/L)_P·(N̄_t,i + N̄_t,j) ≥ (EA/L)_P·(Nt_2p,i + Nt_2p,j)·(1 − 3·2^-P).
   - (EA/L)_P ≥ k_a·(1 − 2^-50).
   - Each converted published |u| is within 2^-52·|u| + 2^-64·max(|u_2p|, S\*_tr,2p) of |u_2p|: one publication rounding and one conversion rounding (≤ 2^-53 each), and the stop rule on that row. So Nt_2p ≥ Nt_pub·(1 − 2^-50) − 6.01·2^-64·S\*_tr,2p. The converted S\*_tr is within a relative 2^-50 of S\*_tr,2p, so this is > Nt_pub·(1 − 2^-50) − 2^-60·S\*_tr.
   - When N_t > 2^-59·S\*_tr, N_t − 2^-60·S\*_tr ≥ N_t/2, so LB's own roundings are relatively ≤ 2^-50.
   - Together, LB ≤ E·(1 + 2^-47) < fl(ê·c), since fl(ê·c) ≥ E·(1 + 2^-41). The conversion used 2^-47 of the 2^-40 headroom, where R2 used 2^-48.
   - The torque row gives the moment bound.
   - **Measured:** 0 failures over 48 controls, the 20 probe cases, 2,000 sweep frames and 120 R1 cases, under R4 as under R3. Mutants M5 and M7 fail it on several controls (`mutants4.stdout.json`). M2 fails the sanity bound (item 3) on TWO-SPAN. The emulator's rows are in SI units, so it does not exercise the conversion.
   - Item 4 cannot catch LEVER2's class, because E is correct there (V4); the estimate does.
   - It closes the under-report gap exactly where the floor binds. On RIGID-UNLOADED, LB = 0.4 against S\*_c ≈ 1e-132 (V4).
5. **Estimate summary:** each `verification_estimate` entry decodes to ≤ 2^-2.
6. **Charge summary (R4):** each `verification_charge` entry decodes to ≤ 1; θ decodes to ≤ 1/2; F's bits are `0x40F0000000000000`. A reader checks the summaries' bounds, not the charge itself: C_q depends on the unpublished verification state, and the Rust replay audit (§4.1.8) recomputes it.

**G5b.** Item 6a, after item 6 and before item 7, only when the selected p = 512. S\* is otherwise unchanged. Φ is the stop rule's, bit for bit (S4).

**G5c.** Unchanged. For floored kinds b = fl↑(2^-64·Φ) ≈ 2^-502·ê.

**D2 text updates (N13).**
- **§4.11.2 ("the basis of b")** gains: "For a case selected at 512, a floored kind's b is fl↑(2^-64·fl↑(2^-438·ê)), from the producer-attested E that G5a bounds below."
- **§4.9.9's N_RP_ABSOLUTE notices** gain the same basis sentence.
- **§5.2's scope of b** (the 2^-22 relative publication gap) is stated with C.

**The retirement gate.**
- **Condition 3:**
  - rigid-motion exact zeros become intervals, and R5-5 reports any check that moves;
  - demotions arise only below 2^-472·ê, with none in the committed families expected;
  - availability rises where today's gate refuses.
- **The gate report shows each case.**

### 6.4 §4.1.6.1's pinned binary64 formulas

| Item | Status |
|---|---|
| Items 1–6, 7–8 | Unchanged |
| Item 6a | Added (§5.3) |

**Constants:**

| Constant | Bits |
|---|---|
| 2^-438 | `0x2490000000000000` |
| 1 + 2^-40 | `0x3FF0000000001000` |
| 2^-59, 2^-60 | exact powers of two |
| F = 2^16 (R4) | `0x40F0000000000000` |

**Order:** item 6 coupling → item 6a (p = 512 only) → item 7.

**Unchanged:** R = 2^-34, t = fl(R·S\*), the S\* < 2^-988 rule, and b.

### 6.5 F2a's contract

F2a (atomic with S-G1):
- maps K4's E, the estimate summary and, in R4, the charge summary, θ and F into the receipt;
- applies item 6a in the producer classification;
- keeps product-derived rows on the updated fo and mo;
- maps an unencodable E to `receipt_encoding`;
- lands the G5a and G5b changes, D2's parity files and the shared case files in the same PR.

**F2a's merge gate is unchanged.** The combination-as-solve rule needs nothing more.

**K4's own fix, outside this addendum.** K4 fixed the combination's prescribed-row double rounding (V4-N7) at `5ad1b6174`: the row is published from its exact sum, rounded once, with control and mutant K4-M33. R4's exact prescriptions in W's residual (§4.1.6.3 item 1) use the same exact terms.

**F3 (W1b) owes one extension (R4).** W1a's ledger is exact (nodal loads), so the Theorem needs nothing from it. W1b's formed equivalents (uniform, thermal, thrust, constant effort, generated equivalent-static loads) are formed values.
- F3 must enter them into W's residual as formed at q_W.
- F3 must add their formation error to N_u, with their own count: for each free row i, s_i times c_f·2^-q_W·Σ\|terms\|.
- F3 must state c_f.

Until F3 does, W1b is outside the Theorem's scope. That is consistent with W1b being outside W1a.

### 6.6 K4's ceiling argument (route 1, step 10; route 2; V4-N5)

**Restated:**

> The stop rule can accept a wrong 512 candidate only if the 1024 state's error is common to 512 and 1024. That is either:
> - a common-mode loss of an exact input, which the exact sums remove; or
> - saturated rounding below 1024's resolution (Lemma 2).
>
> Under 5a.3, the 1024 state's recovery resolution is charged by V = 2^(8−1024)·ê (derived; exact form under the g check). Its solve error is measured by the verification estimate, whose residual is one exact sum over the 1024-formed contributions, so K_1024's assembly roundings are included (R3). The formation error of individual 1024-formed entries (Q4: no re-formation above 1024), and the second-order remainder, are charged: C_q ≤ 2^-22·2^-64·M_q (R4, §4.1.6.3 item 11), under θ ≤ 1/2.
>
> **Honesty:** b = fl↑(2^-64·Φ) bounds the accepted 512 value with the factor 1 + 2^-22, derived modulo F (§4.1.6.3 Corollary). R3's "excess ≤ 2^-516·(Γ_q/ê)·b" is superseded by the charge, which evaluates that excess at runtime and bounds it by 2^-22·b.
>
> **Availability:** the factor 4 between λ·2^-512·ê and 2^-64·Φ is the room for the 512 candidate's own resolution noise to pass the rule.

**Route 1's Oettli–Prager step.** It now uses the gate's backward error relative to |f| + Ā|u|, a larger majorant than |K||u| by a bounded factor (Ā's entries ≤ the element coefficients' sums). RETURN restates the constant.

**The uncertified step** is Hager–Higham's ‖K̃⁻¹‖₁ ≤ F·est. R4 pins F = 2^16 (§5.9). Route 2's honesty now rests on it through §4.1.6.3's Theorem rather than through an argued premise.

### 6.7 Work (V4-N11, N12)

The sweep's work proxy, Σ(p/128)² over solves, excludes the E pass, the estimate and the charge:
- the E pass is one absolute recovery pass plus Ā's constrained rows;
- the estimate is one residual pass over the contributions (R3), one substitution pair and one linear recovery;
- the charge (R4) is one formation pass of element and directional-block entries at q_W, one more residual pass over the contributions (r₂), one pass over Ā's rows and one E-like pass with operands s.

All are charged to the verification attempt.
- A residual pass has one term per contribution rather than one per assembled entry: for a member, 144 terms instead of its share of the assembled entries, about twice a gate residual.
- The q_W formation pass costs one formation of the model's elements at 448 or 832 bits, where the gate's is at 320 or 576.
- Together they are small beside a factorization for banded models of any size. For very small models, formation can be comparable to the factorization.
- The proxy is +1.0 %, the same as R3's against today over 2,000 frames.

## 7. Tests K4 must add, and the mutants (measured in emu4)

**Controls** (invented inputs; `models.py`, `models2.py`, `models3.py`, `lever3.py`, `models4.py`; the expectations are R4's; `run_controls4.stdout.json`). Every R3 control keeps R3's expectation under R4, precision and classes alike.

| Id | Model | Expected |
|---|---|---|
| **F-2** | I12's probe (EA/L = 512, ux(0) = 1, load 2^-300) | 128: rejected by V (Δ = 0, M = 0, V > 0). 256: rejected by Δ = 2^-300 > ε·M (N-4). **512**, with \|N\| = \|R\| = 2^-300 `relative_verified` |
| **F-2-SPOS** | Member 1 E = 2^200 with F-2's pattern and load 2^-60; member 2 unit section carrying 1 N | 512, all honest; today ratio 16 |
| **F-2-CEIL**, **PRESCRIBED-TAIL** (combination form, operands A = 1 and B = 2^-1000 with factors (1, 2^-100)), PRESCRIBED-TAIL-FREE | as the candidate | 512 via Φ, honest; today false at 128 |
| **ASSEMBLY-SAT** (V4-N2) | DEMOTION's layout with a unit member 2 | **512**, honest; today R = −1.5 N `relative_verified` against 0 |
| **DEMOTION2** (V4-S1) | E_big = 2^480; member 2's I = 2^400 | **512**, relative rows 7 → 4, b ≤ 4.7e-6 (V4: 1.67e-6 N on the force rows); documents the only demotion class |
| **EXACT-RIGID** (V4) | One unit member, both nodes fully prescribed with ux = 1 | 512 with b ≈ 1.1e-150 > 0 on the force rows (today 128 with b = 0); honest |
| **LEVER2-k90**, **-k100**, **-k110** (V4-R1; `lever3.py`) | V4's exactly representable lever (§2.5): gain 2^90, 2^100 or 2^110; prescribed rigid y-translation 1; tip spring 2^-580 of the tip's assembled diagonal; load P = 2^(⌊log2 ê⌋ − 438) on the out-of-plane branch. Every LDLᵀ pivot and multiplier of the spring-free matrix is dyadic in the natural order (checked exactly) | **Unresolved (Ceiling):** 128 fails the pivot test; 256 rejected by `verification_estimate` (W/V = 2,681, 2.7e6, 2.8e9), and the charge would refuse it too (1.0e84 to 1.2e102 of its allowance); 512 rejected by the stop rule. Today and under R2's assembled residual: selected at 256, claim ratios 1,005, 1.03e6, 1.05e9. K4's Rust control uses the same model. Its expected outcome is Unresolved whatever the elimination order (argued): at 256 the estimate or Δ rejects, and at 512 Δ does, because 1024 resolves the spring. V4 built no case for K4's RCM order |
| **TILT-LEVER-k90**, **-k100**, **-k110** (V4-T1(c); `models4.py`, V4's `r3_tilt` ported) | LEVER2's lever with no spring; the arm A–C tilted to C = (2^k, −2^(k−290), 0), EA/L = 2^(s+6), so the arm's (uy,uy) entry carries an axial (t/L)² tail lost within the entry up to 576 bits; a rigid rotation ω = 2^-10 about the link node prescribed at A, G and D; P as for LEVER2 | **Unresolved (Ceiling):** 128 fails the pivot test; 256 and 512 rejected by the stop rule at the leak site (Δ ≠ 0), as V4 found. The charge would refuse 256 as well (2.5e34 to 2.6e40 of its allowance; θ 1.5e15 to 1.5e21) |
| **CHARGE-SLENDER** (R4; `models4.py`) | A unit cantilever along x, node 0 fixed; E = A = L = J = G = 1, Iy = Iz = 2^-180; tip load Fy = 1 N. The bounded operator sees the axial stiffness through the transverse displacement 2^180/3, so the charge's t₁ is 102 times its allowance at 128 | **256:** 128 refused by `charge` alone; honest at both. Kills M17 (no charge) and M19 (F = 1), both selected at 128 |
| **RIGID-UNLOADED** (F-3), **F-3-FREE**, **F-3-ROT** | as the candidate | 512, honest; today unresolved |
| **LOADONLY-y345** and **K4's Rust probe set** (5 loads × 1 and 3 members × y_ref (3,4,5) and (0,0,1)) | K4 `5ad1b6174` | **All selected at 128**, honest (in emulation; K4 builds the hybrid-gate control in Rust). The 8 single-mode y_ref (3,4,5) cases are refused under the coalesced gate, as in Rust |
| **GS-TRANS-y345**, **GS-ROT-y345**, **GS-ROT-y345-LOADED** | — | 512, 512, 128 (V4-N8: the GS outcomes are emulation-specific; the LOADED and LOADONLY cases are the robust ones) |
| **SEEDED-COMMON** (S2's control; V4-R2) | N05 with a test-only hook that adds 2^-110 m to node 1 uy of the **final state**, after the gate and before recovery, at every precision: a common verification error the stop rule cannot see | Unresolved (Ceiling), `verification_estimate` at 128, 256 and 512, **through the specified form**: the estimate's residual is recomputed on the final state, so it contains the seed. Without the estimate's test (M11), or with the gate's residual reused (M15, R2's text), selected at 128 with claim ratio 23. The charge does not catch the seed: it is 2^-110 of the state, far inside the allowance |
| **LEDGER-AT-RESTRAINT** | As the candidate | 128 |
| **M7-GS1**, **M10-G**, **R115-SEED3** | The mutant controls below | 512, 256, 512; honest |
| **E-UNIT** | Every control at every precision | E, g and Ā bits equal the generator's emulation of §4.1.6.2 |
| **E-HEADROOM** | Every control at every precision with a state, except the states the estimate or the charge rejects (LEVER2, TILT-LEVER) | \|q_P − q_2P\| ≤ 2^8·2^-P·ê. Observed \|q_P − q\*\| ≤ 1.77·2^-P·ê over 739 states (R3's `measure3_3.json`; R4 does not change the states), a margin of about 2^7. LEVER2's 512 state reaches 7.2e11·2^-P·ê: the lost spring, which W measures exactly (ratio 1) |
| **TWO-SPAN** | K4-M11 | Still one correction at 128 |
| **E-ESTIMATE** (test-only) | Every control, at P = 256 and 512 | W_q agrees with \|R\*(u_P) − q\*\| (a P = 2048 reference) within a relative 2^-8, on rows whose error exceeds 2^-(P+20)·ê. Observed within 5·10^-11 in R3, LEVER2 included |
| **E-CHARGE** (R4) | Every control at every verification | C_q, θ, W⁺ and every factor (est, S, ‖SĀS‖₁, ‖SĀ\|u⁰\|‖_∞, ‖S·r‖_∞, ‖S·r₂‖_∞, ‖S⁻¹δ̂‖_∞, ‖ā_q S‖₁) equal the generator's emulation of §4.1.6.3 items 5 to 12. Emulated: worst C_q/allowance 4.7e-29 at the selected pairs of the R3 controls, 9.0e-45 on the probe set |
| **E-EST** (test-only) | Every control with at most 40 free DOFs | est against the exact ‖K̃_P⁻¹‖₁: exact/est ≤ F. Observed ≤ 1.0 over 195 states |

**Mutants.** A kill is a change of selected precision, of class counts or of honesty, against R4 (`mutants4.stdout.json`; M13's is a change of correction count, at evidence level).

| # | Mutant | Kill (emulated) |
|---|---|---|
| M1 | Drop V | F-2, F-2-SPOS, F-2-CEIL, PRESCRIBED-TAIL, ASSEMBLY-SAT: false claims at 128; M10-G and EXACT-RIGID at 128. (DEMOTION2 no longer: the charge refuses its 128 and 256 candidates) |
| M2 | ê uncoupled | **Behavioural (S6):** F-3-FREE, F-3-ROT, GS-ROT-y345 and **R115-SEED3** unresolved (the frame the candidate's emulator generates at seed 3, index 115, from `sweep.gen`; made a required control); G5a fails on TWO-SPAN |
| M3 | Φ at every p | REACTIONS-ONLY 256 → 128 (SD-G4); F-2, F-2-SPOS, F-2-CEIL, PRESCRIBED-TAIL, the F-3 forms, ASSEMBLY-SAT, GS, M7-GS1, M10-ANISO, M10-G and EXACT-RIGID at 128; R115 at 256. (DEMOTION2 no longer, as under M1) |
| M4 | No Φ | F-2-CEIL, PRESCRIBED-TAIL, the F-3 forms, GS, M7-GS1, M10-ANISO, R115 and EXACT-RIGID unresolved; DEMOTION2 and ASSEMBLY-SAT relative rows 4 → 7 |
| M5 | E without the prescribed \|u\| | PRESCRIBED-TAIL and ASSEMBLY-SAT false claims at 128; RIGID-UNLOADED and M7-GS1 unresolved; M10-G and EXACT-RIGID at 128; DEMOTION2 relative rows 4 → 7; G5a failures on PRESCRIBED-TAIL, REACTIONS-ONLY, DEMOTION2, ASSEMBLY-SAT, M10-G and EXACT-RIGID |
| M6 | E from Σ\|ledger terms\| | LEDGER-AT-RESTRAINT at 256 |
| M7 | Entrywise operator in E and Ā | **Behavioural (S6):** LOADONLY-y345, GS-TRANS-y345, GS-ROT-y345 and M10-ANISO refused by the gate; G5a lower-bound failures on RIGID-UNLOADED, F-3-FREE, F-3-ROT, REACTIONS-ONLY, SKEW6-K1E-12, SKEW-K1E-28, B1-L, LEDGER-AT-RESTRAINT and R115; CHARGE-SLENDER at 128 (its entrywise charge is smaller) with G5a failures. (M7-GS1 alone is not killed: the body-level ê masks it) |
| M8 | Φ = 2^-(2p−74)·ê | The F-3 forms, GS, M7-GS1, M10-ANISO and R115 unresolved; F-2-CEIL unresolved (R4: the charge's 512 allowance falls with the floor); DEMOTION2 and ASSEMBLY-SAT relative rows 4 → 7 |
| M9 | Floor at 256 | F-2, F-2-SPOS and all floored controls at 256, EXACT-RIGID included; DEMOTION2 stays at 512 with relative rows 4 → 7 |
| M10 | g = 1 | **Behavioural (S6):** **M10-G**, 256 → 128. M10-G uses DEMOTION2's layout with member 1's y_ref (1, 2^-12, 0) on its (2,0,0) chord, so g = 2^13 and the formation is exact; E_big = 2^173; member 2's I = 2^100. With g, ê/M = 2^186 > 2^184 rejects 128. **Why the anisotropic noise frame V4 suggested does not kill it** (derived; M10-ANISO was measured not to): a Gram–Schmidt error rotates an orthonormal frame about the member axis. Non-orthogonality enters only at second order, g²·2^-2P. So rigid motions leak no g-amplified force, and the g-amplified error is proportional to deformation (anisotropy × g·2^-P × the deformation force), which ε·M dominates. g is kept because the count needs it |
| M11 | Skip the estimate's test (b) | **SEEDED-COMMON** selected at 128 with claim ratio 23. (LEVER2 no longer: the charge refuses it) |
| M12 | Coalesced gate only (K4 at A2) | LOADONLY-y345, GS and M10-ANISO unresolved (with K4's Rust probe cases) |
| M13 | Bounded test drives refinement | TWO-SPAN: 0 corrections instead of 1 (the K4-M11 evidence, O5). No outcome changes, so the kill is at evidence level, as O5's |
| **M14** (R3) | W's residual over K^q's assembled entries (R2's quantity) | **Not killed in R4** (R3: LEVER2 false at 256). Two reasons, both derived. At q_W = 832, even assembled entries resolve LEVER2's 2^-580 spring. An assembled form would be covered by the Theorem with Lemma B(ii)'s constant raised by one rounding (63.5 → 64.5), within the charge's slack. The contribution-level form stays specified (ROOT's ruling at R2): it makes W measure every assembly sum exactly, rather than charge it |
| **M15** (R3) | W's residual reused from the gate's evaluation, before the hook (R2's text) | **SEEDED-COMMON** selected at 128, claim ratio 23 |
| **M16** (R3) | The bounded test on the last state, not the best (R2) | **Not killed; acceptable** (V4-T3, ROOT). Availability only. A K4 unit test on the gate's choice (SD-G5's vector) is its kill, at evidence level |
| **M17** (R4) | Drop the charge (d) | **CHARGE-SLENDER** selected at 128 (R4: 256). This is ROOT's required kill: a control the charge alone refuses |
| **M18** (R4) | W and the charge formed at q = 2p + 64 (R3's q) | **Detected:** SKEW-K1E-28, PIVOT, SKEW6-K1E-12, N06, B1-L, LEDGER-AT-RESTRAINT and CHARGE-SLENDER move to 512. The charge at 2p + 64 exceeds its allowance on conditioned controls, as V4 showed (4.4e19 on SKEW-K1E-28 with F = 1) |
| **M19** (R4) | F omitted (F = 1) | **CHARGE-SLENDER** selected at 128 (its charge is 102 times the allowance with F = 2^16, 2^-9.3 with F = 1) |
| **M20** (R4) | Drop the θ check (c) | **Not killed.** On the controls, θ exceeds 1/2 only where V, the stop rule or the charge already rejects: DEMOTION2's 128 candidate (θ = 1.1e51) and TILT-LEVER's 256 (1.5e15). The check is kept because Lemma C needs it |
| **M21** (R4) | Prescribed values rounded in W's residual (R3) | **Not killed.** No control has a gained, saturated prescription tail. The exact form is kept because the Theorem's step 2 needs ũ_𝓒 = u\*_𝓒 |
| **M22** (R4) | Translation and rotation rows without W⁺ (R3) | **Not killed.** Every control's and sweep frame's W⁺ is far inside 2^-64·M. The measured bound is kept because the translation claim needs it (§3.4) |
| **M23** (R4) | Drop the g check (c) | **Not killed.** A member with g > 2^(P−16) makes V large, since E carries g, and in every emulated case V rejects it first. The check is kept because Lemma B needs it |

**The estimate's kills (V4-R2).** SEEDED-COMMON kills M11 and M15 through the specified form: the text says the residual is recomputed on the final state, and emu4 implements exactly that. In R3, LEVER2 also killed M11 and M14; in R4 the charge refuses LEVER2 by itself, which is the charge doing its job. E-ESTIMATE is the estimate's positive test, and E-CHARGE the charge's.

**The guards that no control isolates (M20 to M23).** Each guards one step of §4.1.6.3's derivation:
- M20: Lemma C;
- M21: step 2;
- M22: the translation claim;
- M23: Lemma B.

Each costs nothing on every emulated case. Their mutants survive because no emulated case reaches them before another test rejects it (measured). They are kept because each is a hypothesis of a derivation step, so the guarantee is derived rather than conditional on that ordering.

## 8. Where the design needs more than an S\* amendment

1. **The conditioned part: resolved, modulo F.**
   - ROOT adopted the estimate, its contribution-level residual (at `e5f4ef3a8`), and, at `c3f2cfc72`, q_W = 3p + 64 and V4's runtime charge.
   - R4 bounds the second-order remainder under θ ≤ 1/2 and measures it (t₂, t₃).
   - Every step of the guarantee is derived except ‖K̃⁻¹‖₁ ≤ F·est (§4.1.6.3).
2. **The residual gate: resolved** (§5.6), with refinement preserved, the hybrid form confirmed and the best state tested.
3. **The published value carries the 2^-22 relative gap** (2^-21 at p = 512, §5.2). This is pre-existing and stated. The bound itself is no longer first-order.
4. **A per-body schedule.** An F-3 body still sends its whole case to 1024. **Not proposed.**
5. **Route 1's constant under the bounded majorant:** for K4's RETURN (V4-R8; nothing missing in the design).
6. **W1b's formed ledger terms (F3):** they must enter W's residual at q_W and the charge's N_u with their own count (§6.5). W1a needs nothing: its ledger is exact.

## 9. What I could not resolve

- **The remaining limitation, stated plainly.**
  - The honesty guarantee rests, to the factor F, on Hager–Higham's uncertified estimate: the design relies on ‖K̃_P⁻¹‖₁ ≤ F·est with F = 2^16.
  - The condition screen already relies on the same estimate (§4.1.3), and W already relied on it: R3's accuracy argument read the screen as κ < F·2^(p−1).
  - This closure adds no new reliance. It names F, pins its value, and uses it in θ, the charge and W⁺.
  - Every other step of the guarantee is derived (§4.1.6.3's Lemmas A to C, Theorem and Corollary). No step is argued or first-order.
  - Measured: est is exact to rounding on all 195 emulated states, so F = 2^16 has 16 bits of room there. Emulation cannot establish the estimator's worst case, and nothing proves it.
- **The guards M20 to M23 are not killed by any control.** The θ check, the exact prescriptions, the translation bound and the g check are each a hypothesis of a derivation step. No emulated case reaches them before another test rejects it (§7).
- **M16 is not killed; ROOT and V4 accept this** (V4-T3). The gate's choice of state affects availability only.
- **M14 is no longer a behavioural mutant** (§7). The contribution-level form stays specified by ROOT's ruling; the Theorem would also cover an assembled form, with one more rounding in its constant.
- **The emulation of today's gate differs from Rust on one probe case** (the one-member in-plane load at 256; V4-R3). The outcome is the same.
- **Emulation only.** λ, the estimate, the charge, θ, W⁺, the gate and G5a are checked against DS1's emulator and V4's, not against K4's Rust. K4's E-UNIT, E-HEADROOM, E-ESTIMATE, E-CHARGE, LEVER2, TILT-LEVER, CHARGE-SLENDER and hybrid-gate probe controls do that. est's accuracy in K4's Rust (E-EST) is not yet measured.
- **Lemma B takes V4's count as recounted against K4's Rust.** Its exact γ-form is derived from the count's structure. DS1 re-checked the axis and Gram–Schmidt stages, not every chain.
- **W1b is outside the Theorem until F3 extends N_u** (§6.5).
- **G5a item 4's unit conversion** is derived but not exercised: the emulator's rows are in SI units.
- **The R1 families listed in §6.1 as not measured.**
- **T1's fixtures.** Whether its support-motion fixtures hold rigidly moved, unloaded bodies (condition 3's R5-5 movements at F-3) was not surveyed.

## 10. NOTEs declined

None, in any of the three checks. V4-R8 needs nothing in the design. V4-T3 to T5 are recorded as V4 states them (§13). R2's N7 is K4's fix (`5ad1b6174`, K4-M33), cited, not specified here.

## 11. Changes from the candidate: V4's first verification and its resolutions

This is R2's map, with R3's changes to its rows marked.

| Finding | Resolution | Where |
|---|---|---|
| **V4-S1** "never demotes" false | Restated: demotion only below 2^-472·ê at 512; V-driven escalation at ê/M > 2^(2p−72). DEMOTION2 is a control | §4, §7 |
| **V4-S2** the premise is unproven | The verification estimate is specified: the quantity, the test W ≤ V/4, escalation, the work, the evidence, M11, SEEDED-COMMON, E-ESTIMATE. **R3:** its residual is one exact sum over the contributions, recomputed on the final state, and the unmeasured term e_q is restated (§12) | §4.1.6.3 (§5.5), §5.1, §5.2, §5.7 |
| **V4-S3** gate refusal | Bounded acceptance at contribution level; coalesced-driven refinement kept. **Confirmed by ROOT at `e5f4ef3a8`. R3:** the bounded test applies to the best state. The probe results are emulation only | §4.1.4 (§5.6) |
| **V4-S4** Φ and V precision | V, Φ, the threshold and item 6a from one binary64 ê, bit for bit | §3.1, §5.1, §5.3 |
| **V4-S5** g, directional blocks, unpublishable rows, rounding | g exact; directional blocks entrywise in Ā; unpublishable rows included; stage rounding to nearest, published fl↑ | §3.1, §5.4 |
| **V4-S6** M2, M7, M10 kills | R115-SEED3 (M2); the gate and G5a (M7); M10-G (M10); M10's anisotropic non-kill derived | §7 |
| **V4-S7** reader lower bound | G5a item 4, pinned, with its derivation; 0 honest failures. **R3:** taken after unit conversion | §6.3 |
| N1 count | V4's recount adopted, completed; validity includes g | §3.3 |
| N2 ASSEMBLY-SAT | Control added | §2.5, §7 |
| N3 Lemma 2 | Corrected: sufficient condition \|t\| < 2^-2p·\|y\|/2. **R3:** proof repaired | §2.3 |
| N4 F-2 reason | Corrected (128 by V; 256 by Δ). **R3:** V4 records its N4 as an erratum (V4-R7) | §7 |
| N5 §6.6 wording | Availability and honesty separated. **R3:** the 512 term restated as an excess over b | §6.6 |
| N6 publication gap | b's scope and the 2^-22 relative gap stated. **R3:** V4 corrects its 2^-24·b to 2^-23·b (V4-R7) | §5.2 |
| N7 double rounding | K4's fix `5ad1b6174`, K4-M33, cited | §6.5 |
| N8 GS emulation-specific | Stated; LOADED and LOADONLY and K4's probe are the robust controls | §7 |
| N9 R1 scope | Every non-generator family measured, except RF-LARGE > 200 members and the THIN cases; the K4 lane in full | §6.1 |
| N10 1 + 2^-40 bits | Pinned `0x3FF0000000001000` | §6.3, §6.4 |
| N11 E's role | Stated | §5.4 item 6 |
| N12 work proxy | States what it excludes; the E pass and the estimate, now with its residual pass, are costed | §6.7 |
| N13 D2 text | §4.11.2 and §4.9.9 sentences given | §6.3 |

## 12. Changes from R2: V4's delta check and its resolutions

V4's delta check at R2 (`V4_VERIFICATION.md` "Delta check at R2", sha256 `c2f5539b…`, numerics `e5f4ef3a8`) and ROOT's rulings on it (`ROOT_RULINGS_V1.md`, section sha256 `69c8f23b…`). These are R3's resolutions, kept as history. **R4 supersedes their statements on e_q** (§13): e_q is now charged and bounded, not conjectured.

| Finding | Resolution | Where |
|---|---|---|
| **V4-R1** (BLOCKING) The estimate misses a saturated assembled entry: LEVER2 is selected at 256 with W/V = 0 and a claim ratio of 1,005; R2's c ≤ 256 is refuted | **V4's fix adopted, as ROOT ruled.**<br>• W's residual is one exact expansion over the q-formed element entries, the binary64 spring stiffnesses and the q-formed directional-block entries, on the final state; its work is charged.<br>• LEVER2 is built at gains 2^90, 2^100 and 2^110. All three are refused and end Unresolved: 256 by the estimate (W/V = 2,681, 2.7e6, 2.8e9), 512 by the stop rule. Under R2's form, the claim ratios 1,005, 1.03e6 and 1.05e9 are reproduced.<br>• R2's "c ≤ 256, a 2^72 margin" is withdrawn. The remaining term e_q is bounded by 62.5·2^-q·Γ_q, not by a constant; its excess over b is ≤ 2^-66·(Γ_q/ê)·b at p = 128 and 256 and ≤ 2^-516·(Γ_q/ê)·b at 512; its smallness is a conjecture.<br>• The gate's own residual needs no contribution-level form, derived: nothing published depends on it, W measures what it misses, and it moves the gate's test by at most 2^-71 of its tolerance.<br>• No other control, R1 case or sweep frame changes precision or class. | §5.5, §5.6, §5.2, §5.7, §4, §6.6, §7, §8, §9 |
| **V4-R2** (SHOULD-FIX) The residual is reused, so SEEDED-COMMON is caught only by the emulator's recomputation | The residual is recomputed on the final state (item 1). SEEDED-COMMON's hook acts on the final state. M11 and M15 (the reuse form) are killed by SEEDED-COMMON through the specified form, and M11, M14 and M15 by LEVER2 | §5.5 item 1, §7 |
| **V4-R3** (NOTE) The probe's "20 selected at 128" is emulation | Labelled emulation only in §1 and §5.6. The one-member in-plane difference in the emulation of today's gate is recorded | §1, §5.6, §9 |
| **V4-R4** (NOTE) Lemma 2's proof step is a non sequitur | Replaced by V4's two-case argument | §2.3 |
| **V4-R5** (NOTE) The bounded test applies to the last state | It applies to the best evaluated state (smallest worst bounded ratio, the earliest on a tie). M16 is added, and it survives (availability only) | §5.6, §7 |
| **V4-R6** (NOTE) G5a item 4's units | Item 4 takes values after item 3's conversion. Its derivation now absorbs the conversion: LB ≤ E·(1 + 2^-47) < fl(ê·c) | §6.3 |
| **V4-R7** (NOTE) V4's errata | Recorded as corrected: V4-N4 was wrong at 256 (R2 §7 was right), and V4-N6's 2^-24·b is 2^-23·b (R2 §5.2 was right) | §5.2, §11 |
| **V4-R8** (NOTE) Route 1's constant | Nothing missing; for K4's RETURN, as before | §6.6, §8 |
| ROOT: the hybrid gate and V/4 confirmed | R2's "for ROOT to confirm" notes are removed; the derivations are kept | §5.5 item 4, §5.6, §9 |
| DS1's own correction found in R3 | A formed element does not annihilate rigid translations exactly in K4's formation (the mirrored triangle), so no step may assume it. None did, in R2 or R3 | §5.5 |

## 13. Changes from R3: V4's delta check at R3 and its resolutions

V4's delta check at R3 (`V4_VERIFICATION.md` "Delta check at R3", file sha256 `ed2031c3…`, numerics `c3f2cfc72`) and ROOT's rulings on it (`ROOT_RULINGS_V1.md`, section sha256 `9e53317f…`).

| Finding | Resolution | Where |
|---|---|---|
| **V4-T1** (BLOCKING) e_q is only conjectured | **V4's closure adopted, both parts, as ROOT ruled, and made rigorous.**<br>• **3p + 64 specified:** W's contributions are formed at q_W = min(3p + 64, 1024); the extra formation pass is charged.<br>• **The runtime charge:** C_q = ‖ā_q S‖₁·(t₁ + t₂ + t₃) ≤ 60·2^-2p·ê at p = 128 and 256, and ≤ 2^-22·2^-64·M_q at 512. t₁ is V4's term 62.5·2^-q_W·F·est·‖a_q S‖₁·‖SĀ\|u\|‖_∞ with the Neumann factor 2 and the γ-margins. est is the verification's Hager–Higham estimate, S K4's radix equilibration, Ā E's bounded operator (from the pass that forms E's reaction rows, extended to the free rows) and ā_q the bounded recovery row. The work is charged.<br>• **F pinned:** F = 2^16, bits `0x40F0000000000000`, in §4.1.6.3 item 8 and §4.1.3 (§5.9). It is the allowance in ‖K̃⁻¹‖₁ ≤ F·est, which the condition screen and W's accuracy already used unnamed.<br>• **On failure,** p is not selected, and the next p is tried (reason `charge`).<br>• **"The rcond screen … does not bound Γ_q" is replaced** by V4's derivation (Lemma A with Lemma C), cited.<br>• **Beyond first order (ROOT's rigour requirement):**<br>&nbsp;&nbsp;– the second-order remainder is bounded under a θ ≤ 1/2 check (‖K̃_P⁻¹E‖ ≤ 1/2, factor 2; Lemma C);<br>&nbsp;&nbsp;– the remainder is measured by r₂ (t₃) and by the recovery of δ̂ (t₂);<br>&nbsp;&nbsp;– the counts enter in their exact γ-form under a g check (Lemma B);<br>&nbsp;&nbsp;– prescribed values enter W's residual exactly;<br>&nbsp;&nbsp;– translation and rotation rows carry their measured bound W⁺ in the stop rule.<br>No step is left argued; the one uncertified step is F.<br>• **Emulation (emu4):**<br>&nbsp;&nbsp;– LEVER2 (three gains) and TILT-LEVER (three gains) are refused;<br>&nbsp;&nbsp;– the 44 R3 controls and the 20 probe cases keep R3's precisions and classes, with worst charge margins of 4.7e-29 and 9.0e-45 of the allowance;<br>&nbsp;&nbsp;– M17 (no charge) and M19 (F = 1) are killed by CHARGE-SLENDER, a control the charge alone refuses;<br>&nbsp;&nbsp;– M18 (q = 2p + 64) is detected by seven controls;<br>&nbsp;&nbsp;– the sweep and the R1 lane are compared with R3 in §6.1 and §1. | §5.5, §5.1, §5.2, §5.7, §5.8, §5.9, §3.3, §3.4, §4, §6, §7, §8, §9 |
| **V4-T2** (NOTE) V4's R2 statement that only assembly-level sums can saturate is false | R3's citation of it is dropped. **V4's erratum recorded:** TILT-LEVER's member is formed exactly at every P ≤ 576, yet its (uy,uy) entry carries an axial (t/L)² tail lost within the entry. DS1's own example (components differing by more than 2^(p+32)) stands | §5.5 |
| **V4-T3** (NOTE) The gate's choice affects availability, never honesty; M16's survival is acceptable | Recorded as V4 states it. ROOT accepts M16's survival | §7, §9 |
| **V4-T4** (NOTE) In V4's emulator the best state is #2 of 4 in seven single-mode probe cases and #1 in one; all 20 are selected at 128 either way | Recorded as V4 states it: an emulation detail. DS1's emulators choose the second, third or fourth state; the outcome is the same | §5.6 |
| **V4-T5** (NOTE) TILT-LEVER demonstrates the within-entry case, which the stop rule refuses at the leak site | Recorded as V4 states it. R3's "Argued" paragraph is superseded: under R4 such a case is also bounded by the charge, whatever the stop rule sees | §5.5, §7 |
| DS1's own additions in R4 | Made while closing V4-T1; each needed by a step of the Theorem:<br>• exact prescribed values in W's residual (step 2);<br>• the r₂ residual and t₂ (steps 3 and 4);<br>• θ (Lemma C);<br>• the g check (Lemma B);<br>• per-body norms (ALL-ZERO-BODY showed a global norm would charge an all-zero body);<br>• W⁺ for translation and rotation rows (R3 §3.4 was argued).<br>Each is flagged as R4's in the text | §5.5, §3.4, §7 |

## 14. Evidence (scratch, standard-library Python 3.13)

R2's and R3's evidence are unchanged. ROOT committed them at `_run_records_r2/` and `_run_records_r3/`. R4's is new, run one process at a time under `nice -n 19`.

| File | What it is | sha256 (first 16) |
|---|---|---|
| `emu4.py` | R4's emulator: emu3 plus q_W = 3p + 64, exact prescriptions in W's residual, r₂, the norms, est, F, θ, the g check, the charge and W⁺ (switches `W_Q`, `W_PRESC`, `F_ALLOW`, `CHARGE`, `THETA`, `G_CHECK`, `TRRO`, `W_TEST`; option `r4`) | `cc2df0b8cb00b21f` |
| `models4.py` | R3's 44 controls, CHARGE-SLENDER and TILT-LEVER ×3 (V4's `r3_tilt` construction) | `52113c96d0201c16` |
| `run_controls4.py` → `run_controls4.stdout.json` | 48 controls × {today, R3, R4}, with the charge and θ at every verification | `a892a84aa11d9a5a` |
| `gate_probe5.py` → `gate_probe5.stdout.json` | K4's probe set under R4 | `48307994bdec8e6c` |
| `mutants4.py` → `mutants4.stdout.json` | M1–M23 | `ec78f50775356574` |
| `est_check4.py` → `est_check4.stdout.json` | est against the exact ‖K̃_P⁻¹‖₁, 195 states | `1a6610a16f7f1061` |
| `sweep4.py`, `summarize4.py` → `sweep4_{11,12}.json`, `sweep4_summary.json` | 2,000 frames | `3bddcba82314c7ab` (summary) |
| `r1_lane4.py` → `r1_lane4.stdout.json` | K4's R1 lane (120 cases) | `ac336c5dabd630cd` |

**To rerun** (from `<wt>/scratch/ds1/`, one at a time, each under `nice -n 19`):
- `python3 run_controls4.py`;
- `python3 gate_probe5.py`;
- `python3 mutants4.py`;
- `python3 est_check4.py`;
- `python3 sweep4.py 11 1000`, then `python3 sweep4.py 12 1000`, then `python3 summarize4.py sweep4_11.json sweep4_12.json > sweep4_summary.json`;
- `python3 r1_lane4.py <T3 path> lane`.

**Delegation.** DS1 ran as a Claude Code background subagent launched by ROOT through the Agent tool. This revision was made on ROOT's follow-up message. ROOT is the only return path. DS1 dispatched nothing.
