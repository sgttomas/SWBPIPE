# D1 revision 5a.3 (proposed, revision 6): the stop rule's resolution term, verification estimate, formation charge and ceiling floor

Design addendum by DS1, a Type 2 TASK (numerical design drafter), for ROOT (HELP_HUMAN), 2026-09-29. **This is revision 6 of the candidate.** It resolves V4's delta check at R5 (NOT VERIFIED: 1 BLOCKING on availability, 1 SHOULD-FIX, 5 NOTEs) and applies ROOT's rulings on it.

- **The candidate and R2 to R5 stay unchanged:**
  - the candidate, `DESIGN_NUMERICS/REV_5A3_CANDIDATE/D1_REV_5A3_SSTAR_RESOLUTION.md` (sha256 `c265d7bb…`, numerics `a7015eea3`);
  - R2 (`326a8d78…`, numerics `9e9e3056a`);
  - R3 (`2cbb6582…`, numerics `c85dc1151`);
  - R4 (`4a52422d…`, numerics `c644b751d`);
  - R5 (`ef68ab38…`, numerics `ff5dc2b05`).
- **This file is a proposal.** V4 runs a delta check; ROOT selects only after VERIFIED.
- `DESIGN_NUMERICS/DESIGN.md` (revision 5a.2, sha256 `fb62ef4a…`) stays hash-pinned. The amended text is in §5, as addendum blocks.
- **§15 maps every finding of V4's delta check at R5 to its resolution** ("Changes from R5"). §11 to §14 keep the earlier maps.
- **What R6 changes:**
  - **A second certified bound, from one shifted factorization (V4-V1).** Factor K̃_P − σI with the same loop. If every pivot is positive, λ_min(K̃_P) > σ′ = σ − δ − γ_m·N′_L, so ‖K̃_P⁻¹‖₁ ≤ ⌈√n⌉/σ′ =: S (Lemma E, derived). **The certified bound used is B = min(Uc, S).** σ comes from the estimate (an availability use). A failed shift halves σ, at most three factorizations in all, then Uc stands alone. The shift runs only where Uc is missing or looser than the shift can be.
  - **Per block (V4-V2, V4-V5).** Uc, S and B are formed per connected block of the free-free pattern, from the same passes. θ, the bound's existence and the g check are tested only for blocks that carry data. A body's charge uses the largest B over its data-carrying blocks.
  - **R1's RF-LARGE frames at 10, 100 and 1,000 members, in K4's elimination order, are all selected at 128 under R6, honest,** as under today's rule and R4. R5 moved five of the six at 100 members to 256 or 512, and left five of the six at 1,000 members Unresolved (§6.1).
  - **The emulator now factors in K4's order** (a port of `factor.rs`'s reverse Cuthill–McKee), so its bound figures transfer to K4.
  - **Mutants:**
    - M17 loses its only kill and is kept for the derivation;
    - M20 is killed by a new control, THETA-STUB-COUPLED;
    - M27 (the shift bound without its backward-error term) is killed at low precision;
    - M28 (no shift) and M29 (R5's form) are killed by RF-LARGE.
  - NOTEs V3 to V7 are fixed; V4's R3 count erratum is recorded.
  - **No uncertified step remains in the honesty guarantee** (§9).

**Labels.** Every claim is marked as one of:
- **derived:** it follows step by step, and a verifier can check each step;
- **measured:** it was observed in DS1's standard-library emulation (§16). A measurement is evidence, not proof;
- **argued:** it is reasoned but not proved;
- **conjecture:** it is believed, and it is not established.

**The honesty guarantee (§5.5's Corollary) carries no "argued" or "conjecture" label, and it rests on no uncertified estimate.** The labels remain in this file only for availability statements and for history.

## 0. Basis, inputs and what was not done

**Read for this revision.** Hashes are sha256, first 8 hex digits. `<wt>` is the T3 worktree root.

| Input | Hash |
|---|---|
| V4's delta check at R5: `V4_VERIFICATION.md`, section "Delta check at R5", at numerics `6ca7a4338` | `fd381e3b` (file) |
| V4's R5 records `_v4_records/r5/` (read: `r5_large`, `r5_large_sched_rcm`, `r5_shift`, `r5_stress`, `r5_m17`; its `SHA256SUMS`) | `6bc70bd1` (`SHA256SUMS`) |
| `ROOT_RULINGS_V1.md`, section "D1 revision 5a.3: rulings on V4's delta check at R5", at `6ca7a4338` (heading to end of file) | `6ef6c730` (section) |
| V4's delta check at R4 (`8831325d0`) and ROOT's rulings on it | `7f405328` (file), `02948a14` (section) |
| V4's delta check at R3 (`c3f2cfc72`) and ROOT's rulings on it | `ed2031c3` (file), `9e53317f` (section) |
| V4's delta check at R2 (`e5f4ef3a8`) and ROOT's rulings on it | `c2f5539b` (file), `69c8f23b` (section) |
| V4's verification (`3057fc22f`) and ROOT's rulings on it (`085638e58`) | `0222d0ec`, `9a36d688` |
| K4 at `5ad1b6174`: the Rust probe, RETURN §21, K4-M33 | — |
| K4 at `3ed6c0e26` (A2), for the functions below | — |
| D1 r5a.2 §4.1.3 (the condition screen), §2 (W1a's scope) and §4.2 (kernel support motion) | `fb62ef4a` |
| K4's `retained/factor.rs` at `cef218a10`: the factor loop (Lemmas D and E) and the ordering (`reverse_cuthill_mckee`, `order_free`), ported for R6 | `81f81f24` (file) |
| R1's `REFERENCES/references.json` and `references.py` (RF-LARGE), through R2's `r1_adapter` | `7b176dbb` (as the candidate) |

The candidate's §0 lists its own inputs, all still valid: D1 r5a.2 `fb62ef4a`; D2 r5b.2 `edc78f9c`; R1 references `7b176dbb`; and the rest.

**Where the stop rule, S\* and the gate live in K4 (A2 `3ed6c0e26`, `retained/adaptive.rs`):**
- `residual_rows` `:872`;
- `scales_at` `:1093–1136`;
- `stop_rule` `:1141`;
- `classify_rows` `:1430`.

Their bodies are byte-identical to A1's (the candidate's §0). The factor, its condition estimate and its ordering mirror `factor.rs` (V4 checked the factor line by line; DS1 ported the ordering for R6).

**Done.** Standard-library Python only, under `<wt>/scratch/ds1/`, one process at a time under `nice -n 19`, as the host rule requires.
- `emu6.py` is R5's `emu5.py` with R6's changes, each behind a switch whose default is R6 (option `r6`):
  - K4's elimination order (`ORDER`; `k4order.py` ports `factor.rs`'s reverse Cuthill–McKee, and `k4order_check6` compares it with V4's port);
  - per block, Uc_c and the shifted-factorization bound S_c, B_c = min(Uc_c, S_c) (`BOUND`, `SHIFT_BE`, `SHIFT_TRIES`);
  - θ, the bound's existence and the g check per data-carrying block (`SCOPE`);
  - every row loop sparse over the structural pattern. **Regression:** with emu5's order and R5's or R4's switches, emu6 reproduces `run_controls5` exactly on all 58 controls (`reproduce6`).
- R6 ran, under emu6 in K4's order:
  - every control: 59 (R5's 58 and THETA-STUB-COUPLED), under today's rule, R4, R5 and R6;
  - every mutant: M1–M18 and M20–M29;
  - K4's probe cases, rebuilt;
  - Uc_c, S_c and B_c against the exact per-block norm on the 234 model states (`uc_check6`: R5's 231 and THETA-STUB-COUPLED's three), and Lemma E at low precision and on V4's F2 family (`stress6`);
  - R1's RF-LARGE references at 10, 100 and 1,000 members (`large6`);
  - the 2,000-frame sweep;
  - K4's R1 lane (120 cases).
- The files are listed in §16.

**Not done.**
- No Git write, and no cargo or Rust run. Every K4 statement comes from reading its source and recorded outputs.
- No product code.
- No file written outside `<wt>/scratch/ds1/`, apart from temporary edit scripts and copies of outputs for the rerun comparisons, in the session's own temporary directory.
- The exact ‖K̃⁻¹‖₁ at RF-LARGE's 1,000 members (6,000 DOFs): not computed. The 1,000-member lane itself ran in full (§6.1).

## 1. Summary

**The recommendation, in seven lines.**
1. **Correct §4.1.9.** A tail below 2p's rounding unit is lost identically at p and at 2p ("saturated"; Lemma 2). A wrong value then passes by up to the verification's own first-order error.
2. **Define a per-body formation scale E** for the force and moment kinds (§4.1.6.2).
   - It is the Σ\|terms\| of every recovery expansion over a bounded operator, with each stage rounded once at 2p.
   - g is exact; E is coupled through L_b; it is published rounded upward.
   - V, Φ and G5b's item 6a all use one binary64 ê, bit for bit (S4).
3. **Change the stop rule.** Accept p only if \|q_p − q_2p\| + V ≤ 2^-64·max(\|q_2p\|, S\*), with V = 2^(8−2p)·ê for force and moment rows, **and** the verification estimate satisfies W ≤ 2^(6−2p)·ê (§4.1.6.3; ROOT's S2 ruling and its confirmation of V/4).
   - W's residual is one exact sum over the contributions, formed at **q_W = min(3p + 64, 1024)**, with **exact prescribed values**, recomputed on the final state.
   - Translation and rotation rows take their measured bound W⁺ as V.
4. **Charge what W cannot measure,** with a certified bound on the inverse norm (R5; per block, with the shift bound, in R6).
   - Per block c of the free–free pattern, two bounds from the verification's own factor:
     - **Uc_c = U_c/(1 − U_c·γ_m·N_L,c)**, with U_c from ‖M(L)⁻ᵀD⁻¹M(L)⁻¹e‖ and N_L,c = ‖\|L\|D\|Lᵀ\|‖₁ over the block, every operation on nonnegative data rounded upward (Lemma D);
     - **S_c = ⌈√n_c⌉/σ′_c** from one shifted factorization of K̃_P − σ_c·I, with σ′_c = σ_c − δ_c − γ_m·N′_L,c rounded downward (Lemma E; V4-V1). It runs only where Uc_c is missing or looser than S_c can be; σ_c comes from the estimate, an availability use; a failed shift halves σ_c, at most three factorizations.
   - **B_c = min(Uc_c, S_c).** A body's B_b is the largest B_c over its blocks with data.
   - C_q = ‖ā_q S‖₁·(2^(7−q_W)·B_b·N_u + 70·2^-P·‖S⁻¹δ̂‖_∞ + 3·B_b·‖S·r₂‖_∞) must be ≤ 60·2^-2p·ê at p = 128 and 256, and ≤ 2^-22·2^-64·M_q at 512.
   - For every block with data: B_c exists, θ_c = B_c·2^(7−P)·‖SĀS‖_c ≤ 1/2, and every member with a nonzero prescribed DOF or a free DOF in the block has g ≤ 2^(P−16).
   - **The guarantee is then derived beyond first order, with no uncertified step** (§5.5's Theorem and Corollary).
5. **At p = 512 only,** floor the force and moment S\* at Φ = fl↑(2^-438·ê).
6. **Residual gate (§4.1.4; ROOT's S3 ruling, O12 reversed; the hybrid form confirmed).**
   - The acceptance test uses the bounded operator at contribution level, \|f\| + ΣĀ\|u\|, on the best state evaluated (V4-R5).
   - Refinement stays driven by M03's coalesced ratio, so every correction made today is kept.
   - The gate's own residual stays over K^q's assembled entries (§5.6).
7. **Publish** `resolution_scale` (E per body), the estimate's and the charge's summaries, and per body θ and B_b.
   - G5a gains the shape check, the zero rule, the sanity bound, V4's published-data lower bound on ê, and the summaries' bounds.
   - G5b applies item 6a at p = 512. G5c is unchanged.

**Evidence** (measured in emulation, in K4's elimination order; the full tables are in §6.1, §7 and §16):

| Set | Result under R6 | R5 (and today) |
|---|---|---|
| **R1's RF-LARGE, 10 and 100 members** (12 frames, 45 to 600 free DOFs) | **All 12 at 128, honest,** with today's classes and 0 G5a failures. S is within 2^3.3 to 2^5.6 of the norm; the shift runs where Uc is loose | R5: five of the six 100-member frames at 256 or 512 (`uc`, `charge`), as V4 found. Today and R4: all at 128 |
| **R1's RF-LARGE, 1,000 members** (6 frames, 4,500 to 6,000 free DOFs; full schedule, emulated) | **All 6 at 128, honest,** with today's classes and 0 G5a failures. S ≤ 2^7.3 times the norm (measured against est; the exact norm was not computed) | R5: five of six Unresolved (`uc` at every precision). Today and R4: all at 128 |
| **2,000 random frames** | **0 false claims; 0 unresolved;** 893 at 128, 720 at 256, 387 at 512. **Identical to R5 in the same order, frame by frame:** precision, classes, rejection reasons and G5a. Worst charge 1.5e-7 of its allowance; θ ≤ 5.7e-40; W/V ≤ 0.0082; 0 G5a failures; no `uc` rejection. The shift ran on 380 frames | R5 in emu5's natural order: 889/726/385; K4's order moves 24 frames' precision and 34 frames' classes or reasons, all honest. Today: 1 false claim (R982), 225 unresolved |
| **59 controls** (R5's 58 and THETA-STUB-COUPLED) | Every selected case honest; 0 G5a failures. **Two selections change from R5, each as intended:** THETA-STUB and HH-SLENDER-m40 return to 128 (block scope; per-block bound). THETA-STUB-COUPLED is at 256 (θ). The worst charge at a selected pair is 1.6e-3 of its allowance. LEVER2, TILT-LEVER and SEEDED-SOFT are refused | R5 in K4's order: every R5 control at R5's precision. Today: 11 false claims (9 natural, including the three LEVER2; SEEDED-COMMON's and SEEDED-SOFT's are seeded) |
| **The bounds against the exact norm** | Uc_c, S_c and B_c ≥ exact on all 267 blocks of the 234 model states; B_c/exact ≤ 14. Lemma E at low precision: S ≥ exact on all 1,457 factors where it exists; M27's unshifted-error bound falls below exact 27 times | R5 (natural order): Uc ≥ exact on 245 states |
| **K4's probe** (20 cases), rebuilt, **emulation only** | All 20 at 128, honest, 0 G5a failures, the same classes as R5. The best state moves in five single-mode cases. Worst charge 7.4e-49 of its allowance | Today: the 8 single-mode y_ref (3,4,5) cases are refused by the coalesced gate; the one-member in-plane case now fails at 256, as in Rust |
| **K4's R1 lane** (120 cases) | 120 of 120 at 128, with today's classes; 0 G5a failures; W/V ≤ 0.0029; worst charge 2.0e-41 of its allowance | R5, in either order: the same, case by case. Today: 120 at 128 |

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

### 2.5 Instances (measured, `run_controls4.stdout.json`; the same today-rule outcomes in `run_controls5.stdout.json`)

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

**Validity (a check since R4, scoped in R5 and R6; §4.1.6.3 item 10).** The count needs 68g·2^-P small.
- The check is g ≤ 2^(P−16) at the verification, so 68g·2^-P ≤ 2^-9.9.
- Each first-order count then holds in its exact form with a factor of at most 1 + 2^-8.8 (§4.1.6.3 Lemma B, derived). R4 stated 1 + 2^-7, which is looser and still true. V4 derived the factor as at most 1 + 2^-9.5 for K4's Rust stages (R5 said "measured"; corrected for V4-V4).
- **R5 scoped the check** to members with a free DOF or a nonzero prescribed DOF in a body that carries data (V4-U2). **R6 scopes it** to members with a nonzero prescribed DOF or a free DOF in a block that carries data (§4.1.6.3 item 7a). Any other member's entries multiply only zeros, so Lemma B is not needed for them.
- A case that fails the check is not selected at that p. G-PRESC-MEMBER is the check's availability kill of M23 (§7).

**Measured:** max\|q_P − R\*(u_P)\|/(2^-P·E_q) ≤ 2.33, where R\* is the recovery with P = 2048 operators (`measure3_3.json`, LEVER2 included; V4: 0.69).

**The element's rigid-mode leakage** is the K_e row with u a rigid motion: ≤ (20g + 42.5)·2^-P·(B̄ᵀ\|D\|B̄\|r\|). So F-3's leakage is inside λ.

### 3.4 Translation and rotation (derived in R5; derived modulo F in R4; argued in r5a.2 to R3)

For these kinds E would be \|u\| ≤ S\*, so a V of 2^(8−2p)·S\* would be negligible and none is charged. R3 and earlier bounded their propagated error only by argument, through the rcond screen.

**R4 measures and charges it.**
- A free displacement or rotation row's verification error is ≤ W⁺_i = \|δ̂_i\| + s_i·(t₁ + t₃) (§4.1.6.3 item 12 and its Theorem, derived; with Uc in R5, and with the per-block bound B in R6).
- The stop rule adds W⁺ to \|Δ\| for these rows.
- The claim for translation and rotation rows is therefore derived, like the force and moment claim.
- **W⁺ is load-bearing (V4-U3).** Mutant M22, which drops it, publishes a false claim on SEEDED-SOFT (§7). No unseeded control changes precision or class under M22 (`mutants5`, `mutants6`); the sweep was not run under the mutants.

## 4. The options, and the choice

The candidate's §4 derivations of (i-a), (i-b), (i-g), (ii) and (iii) stand. Their sweep figures come from the candidate's emulator, over 1,966 frames. Two corrections:

**S1: (iii)'s demotion claim, restated (derived; V4-S1).**
- At p = 512 a floored kind has S\* ≥ Φ = 2^-438·ê, so its threshold t = R·S\* ≥ 2^-472·ê. **A row is demoted only if the case is selected at 512 with Φ above the kind's computed scale, and the row has \|q\| < 2^-472·ê(body, kind).**
- Reaching 512 through V requires some row with ê/M > 2^(2p−72), which is 2^440 at the 256 candidate, or another rejection at 256.
- **Measured:** V4's DEMOTION2 (ê ≈ 2^482; 1 N rows) is demoted exactly as V4 found: relative rows 7 → 4 (V4: b = 1.67e-6 N on the force rows; emu2: b ≤ 4.7e-6 over both kinds). In the 2,000-frame sweep no relative row was lost.

The candidate's "never demotes" is withdrawn. The contrast with (i-g) is the threshold: (i-g) floors a kind whose published scale is below 2^(10−p)·ê (2^-118·ê at p = 128), and demotes its rows below 2^(40−p)·ê.

**The additions are orthogonal to the options.** The verification estimate and the formation charge (§4.1.6.3) and the gate's bounded acceptance (§4.1.4) would serve any of them.
- With them, (iii)'s honesty no longer rests on a conjecture. W measures the verification's solve error, and the charge bounds what W cannot measure.
- The whole guarantee is derived beyond first order, with no uncertified step (§4.1.6.3's Theorem and Corollary; the certified Uc since R5, and B = min(Uc, S) per block since R6).
- The candidate's robustness argument for (i-g) (margin 2^66 against λ at 2p) is therefore no longer needed.

**(iii) remains the recommendation:**
- it changes no correct outcome except by escalation, and by the demotion class above;
- it demotes only below 2^-472·ê;
- the estimate and the charge cost, per verification:
  - one formation pass at q_W;
  - two exact residual passes over the contributions;
  - two light passes: Ā's rows, and E's expansion with operands s;
  - one substitution pair and one recovery pass for W;
  - one substitution pair and one pass for Uc (R5);
  - up to three shifted factorizations, only where Uc is missing or looser than the shift can be (R6).

## 5. The exact amended text (addendum blocks)

### 5.1 §4.1.6, stop rule: replace the "Accept p" bullet; add the floor bullet

> - **Accept p** (revision 5a.3) when all hold, decided exactly:
>   - (a) every published quantity q satisfies `|q_p − q_2p| + V_q ≤ 2^-64 · max(|q_2p|, S*)`, where **V_q = 2^(8−2p)·ê(body, kind)** for q of kind force or moment, and **V_q = W⁺_q** (§4.1.6.3 item 12) for translation and rotation (R4; 0 before R4);
>   - (b) the verification estimate passes, `W_q ≤ 2^(6−2p)·ê(body, kind)`, for every force and moment row (§4.1.6.3 item 4);
>   - (c) at the verification, for every block of the free–free pattern that carries data (§4.1.6.3 item 7a): **a certified bound B_c exists** (item 7d), **θ_c ≤ 1/2** (item 9), and **g ≤ 2^(P−16) for every member with a nonzero prescribed DOF or a free DOF in such a block** (item 10) (R4; scoped per body in R5 and per block in R6);
>   - (d) **the charge passes** for every force and moment row: C_q ≤ 60·2^-2p·ê(body, kind) at p = 128 and 256, and C_q ≤ 2^-22·2^-64·M_q at p = 512 (§4.1.6.3 item 11; R4).
>
>   ê is §4.1.6.1 item 6a's binary64 value, formed from the E bits the receipt publishes and lifted exactly. V_q, Φ, the estimate's threshold and the charge's allowance use that same ê, bit for bit. A failure of (b), (c) or (d) rejects p like a failure of (a), with its own reason (`verification_estimate`, `uc`, `theta`, `g_validity`, `charge`).
> - **The ceiling floor (revision 5a.3).** At the last candidate, p = 512 (verified at 1024), `S*(kind) := max(S*(kind), Φ)`, with `Φ = fl↑(2^-438 · ê(body, kind))`, for force and moment. It applies in the stop rule and in the classification: item 6a, the same Φ bits. At p = 128 and 256 there is no floor.
> - **Why (revision 5a.3).**
>   - A rounding whose tail lies below both p's and 2p's rounding units is identical in both candidates (§4.1.9). (a) charges the verification's resolution; (b) measures its solve error; (c) and (d) bound, beyond first order, what (b) cannot measure: the formation error of individual entries below q_W's resolution, and the second-order remainder.
>   - Without V, F-2 passed labelled exact against a truth of 2^-300.
>   - Without the floor, an unloaded body moved rigidly (F-3) could not be accepted at any precision.

### 5.2 §4.1.6 "What acceptance guarantees": replace its first bullet (V4-N6)

> - **What acceptance bounds (revision 5a.3).** If p is accepted, then for every q, |q_p − q\*| ≤ |q_p − q_2p| + |q_2p − q\*|, and |q_2p − q\*| ≤ (recovery resolution) + (the solve error of the 2p state).
>   - The recovery resolution is ≤ 69·2^-2p·E_q (§4.1.6.2 item 5 in its exact form, §4.1.6.3 Lemma B; derived under the g check).
>   - The solve error is ≤ Ŵ_q·(1 + 2^-2p) + C_q for force and moment rows and ≤ W⁺_q for translation and rotation rows (§4.1.6.3 Theorem; derived).
>   - The rule admits p only when (a) to (d) hold.
>   - So |q_p − q\*| < 2^-64·M_q − 62·2^-2p·ê at p = 128 and 256, |q_p − q\*| ≤ 2^-64·M_q·(1 + 2^-22) at p = 512, and |q_p − q\*| ≤ 2^-64·M_q for translation and rotation rows (§4.1.6.3 Corollary). b = fl↑(2^-64·S\*) therefore bounds |q_p − q\*| for an `absolute_verified` row, with the factor 1 + 2^-22 at p = 512. For a kind floored at p = 512, b = fl↑(2^-64·Φ) ≈ 2^-502·ê.
>   - **No step is uncertified** (R5; R6). The inverse norm that every step uses is bounded, per block, by B_c = min(Uc_c, S_c), derived from the verification's own factor and, where needed, shifted factorizations of the same matrix (§4.1.6.3 Lemmas D and E). R4's "modulo F" is withdrawn.
> - **Scope of b.** b bounds the candidate's value q_p, not the published binary64 value. Two small differences follow; both were present in r5a.2 and are stated, not widened:
>   - publication adds |q_pub − q_p| ≤ 2^-53·|q_p|, which for an absolute row (|q_p| < 2^-34·S\*) is < 2^-87·S\* ≤ 2^-23·b (V4 has corrected its earlier 2^-24·b to 2^-23·b, V4-R7);
>   - the published S\*_pub (from binary64 rows) may lie below the stop rule's S\*_2p by a relative (2^-64 + 2^-52).
>
>   So |q_pub − q\*| ≤ b·(1 + 2^-22) at p = 128 and 256, and ≤ b·(1 + 2^-21) at p = 512, where the charge's 2^-22 adds to the publication gap. D2's interval binding [q_pub ± b] inherits this relative 2^-21. R3's "first-order qualification" is withdrawn, because no first-order step remains, and so is R4's "modulo F" (§4.1.6.3).

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

### 5.5 §4.1.6.3 (new): the verification estimate and the formation charge (S2; V4-R1, V4-R2; rigorous since R4 for V4-T1; certified in R5 for V4-U1; per block, with the shift bound, in R6 for V4-V1, V4-V2 and V4-V5)

> At the verification state (precision P = 2p), after its residual gate has passed. **Every norm below is taken over the free DOFs of the row's own body.** K is block-diagonal by body, because springs and directional blocks do not connect bodies (§4.1.6.1 item 1). So are the factor's L and D: elimination never fills an entry between two bodies.
>
> 1. **Residual.** Take the final state, the state the gate selected (§4.1.4 step 3). For every free row i form
>
>    `r_i = f_i − Σ_e Σ_j k(e)_ij·u⁰_j − Σ_s k(s)·u⁰_i − Σ_d Σ_j k(d)_ij·u⁰_j`
>
>    as **one exact expansion over the contributions**, where:
>    - f_i is the exact ledger net. W1a's ledger is exact: nodal forces and moments in binary64;
>    - k(e)_ij and k(d)_ij are element and directional-block entries **formed at q_W = min(3p + 64, 1024)** (R4; ROOT's ruling 1 at R3). That is 448 bits at p = 128 and 832 at p = 256. At p = 512 it is 1024, the state's own entries (Q4);
>    - k(s) is each global-axis spring's binary64 stiffness;
>    - u⁰ is the final state with every prescribed value at its **exact** value (R4): a single case's binary64 value, or a combination's exact expansion Σ_k λ_k·u_k,c. K4 already carries those terms to publish the prescribed row once rounded (K4-M33).
>
>    Nothing is summed into an assembled entry first, and nothing is reused from the gate.
> 2. **Correction.** δ̂ = K_P⁻¹·r̂, where r̂_i is r_i rounded once to P, with one scaled substitution pair using the state's factor. δ̂ = 0 at constrained DOFs.
> 3. **Estimate.** Ŵ_q = |L_q(δ̂)| for every published force or moment q. L_q is q's recovery at P applied to δ̂ with the ledger omitted, as in R3.
> 4. **Test.** Ŵ_q ≤ 2^(6−2p)·ê(body, kind) (= V_q/4), with item 6a's ê (ROOT confirmed V/4 at `e5f4ef3a8`).
> 5. **The correction's own residual (R4).** r₂_i = r_i − Σ_j K^c_ij·δ̂_j over the same contributions, as one exact expansion.
> 6. **Norms (R4; directions and the ∞-norm fixed in R5, V4-U8).** S = diag(s_i) is K4's radix equilibration of the free DOFs, the powers of two the factor already uses. Ā is E's assembled bounded operator at P (§4.1.6.2 item 3). ā_q is q's bounded recovery row: the coefficients of E_q's expansion, ledger omitted. Every norm is one exact expansion per row or column, **rounded upward**:
>    - ‖SĀS‖ = the larger of max_j s_j·Σ_i s_i·Ā_ij (the 1-norm) and max_i s_i·Σ_j Ā_ij·s_j (the ∞-norm). Ā's two triangles come from different rounding paths and can differ in the last place; Lemma C uses the 1-norm and the Theorem's step 5 the ∞-norm;
>    - ‖SĀ|u⁰|‖_∞ = max_i s_i·Σ_j Ā_ij·|u⁰_j|, over every column j. It is the pass that forms E's reaction rows (§4.1.6.2 item 4), extended to the free rows;
>    - ‖S·r‖_∞ and ‖S·r₂‖_∞;
>    - ‖S⁻¹δ̂‖_∞ = max_i |δ̂_i|/s_i;
>    - ‖ā_q S‖₁ = Σ_j ā_qj·s_j. This is E_q's expansion evaluated with s_j in place of |u_j| at free DOFs and 0 at constrained ones.
> 7. **The certified bound B (R5's Uc; per block, with the shifted-factorization bound, in R6).** K̃_P = S·K_P·S is the matrix the verification factored as L·D·Lᵀ, in K4's elimination order, with L unit lower triangular on the profile and D the positive pivots.
>
>    **7a. Blocks (R6; V4-V2, V4-V5).** A block is a connected component of the free–free structural pattern: K4's `structure.pattern`, from which the factor's profile is built. Blocks refine bodies.
>    - **L and D are block-diagonal (derived).** For i and j in different blocks, K̃_ij is an exact zero. By induction over the loop, every work entry, multiplier and cross term the loop forms between two blocks is an exact zero, so no rounding crosses a block.
>    - **A block carries data** if a nonzero ledger term is at one of its DOFs, if one of its DOFs is coupled by the pattern to a constrained DOF whose prescribed value is nonzero, or if its state is nonzero at one of its DOFs.
>    - **A block without data needs no bound (derived).** Its right-hand side is exactly zero for K\*, K^c and K_P alike, so u = ũ = u\* = 0 on it, and r, r₂, δ̂, x̃ and y vanish there. Its rows add nothing to any term below.
>
>    **7b. Uc_c (R5's bound, per block).** Every quantity below is nonnegative and formed at P bits, **rounded upward** (to nearest, then one ulp up when below the exact value, as fl↑ in item 6a):
>    - a = M(L)⁻¹e by forward substitution: a_i = 1 + Σ_{j<i} |l_ij|·a_j;
>    - b_i = a_i/d_i, then c = M(L)⁻ᵀb by back substitution: c_j = b_j + Σ_{i>j} |l_ij|·c_i;
>    - a′ = |Lᵀ|e (a′_j = 1 + Σ_{i>j} |l_ij|), b′_i = d_i·a′_i, c′ = |L|b′ (c′_i = b′_i + Σ_{j<i} |l_ij|·b′_j);
>    - per block c: **U_c = max_{i∈c} c_i** and **N_L,c = max_{i∈c} c′_i** = ‖\|L_c\|D_c\|L_cᵀ\|‖₁;
>    - γ_m = m·2^-P/(1 − m·2^-P), with m = 2n + 2 and n the number of free DOFs;
>    - t_c = U_c·γ_m·N_L,c. If t_c < 1, the denominator 1 − t_c is rounded **downward** and **Uc_c = U_c/(1 − t_c)** upward. Otherwise Uc_c does not exist.
>
>    M(L) is L's comparison matrix: its diagonal is 1 and its off-diagonal entries are −|l_ij|. Lemma D derives Uc_c ≥ ‖K̃_c⁻¹‖₁.
>
>    **7c. S_c, the shifted-factorization bound (R6; V4-V1, ROOT's ruling).** It runs only for a block that carries data and whose Uc_c is missing or exceeds 2·⌈√n_c⌉·est_c, the least value S_c can take. n_c is the block's number of free DOFs.
>    - **σ_c = 1/(2·est_c), rounded downward.** est_c is the largest ratio ‖(K̃⁻¹x)_c‖₁/‖x_c‖₁ over the condition screen's own solves with x_c ≠ 0; for a single block it is est itself. **This is an availability use of the estimate**: any σ_c > 0 leaves Lemma E true, and σ_c only decides whether S_c exists and how tight it is.
>    - **The shifted factorization.** Replace each diagonal entry of every such block by d̃_i = fl(K̃_ii − σ_c), rounded to nearest; other blocks are unshifted. Run the factor loop, with the pivot test replaced by d′_i > 0. A block with a pivot that is not positive fails at this σ_c; its later rows are not used, and no other block is affected (7a).
>    - **For a block whose pivots are all positive:**
>      - N′_L,c = max_{i∈c} c′_i from the shifted factor, by 7b's second pass (upward);
>      - δ_c = 2^(1−P)·max_{i∈c} |d̃_i|, exact (it bounds the rounding of the shifted diagonal: |x − fl(x)| ≤ 2^-P·|x| ≤ 2^(1−P)·|fl(x)|);
>      - σ′_c = σ_c − (γ_m·N′_L,c + δ_c), with the bracket upward and the difference downward;
>      - if σ′_c > 0, **S_c = ⌈√n_c⌉/σ′_c**, upward. ⌈√n_c⌉ is an exact integer.
>    - **A block that failed** has its σ_c halved (exactly) and is refactored together with every other block that failed. There are at most three shifted factorizations in all (σ_c, σ_c/2, σ_c/4). A block whose pivots passed but whose σ′_c ≤ 0 is not retried. Lemma E derives S_c ≥ ‖K̃_c⁻¹‖₁.
>
>    **7d. B_c = min(Uc_c, S_c)**, over those that exist. **If a block with data has neither, p is not selected (reason `uc`).** A body's bound B_b is the largest B_c over its blocks with data.
>
>    Either rounding direction may be implemented as round to nearest followed, unconditionally, by one ulp in the required direction. That is never on the wrong side of the exact value, and the extra ulp only loosens the bound.
>
>    **Work:**
>    - R5's substitution pair (a, then c) and pass (a′, b′, c′) over L's profile, unchanged; the maxima are taken per block.
>    - One pass per screen solve for est_c's per-block sums.
>    - **Only when 7c's condition holds for some block:** up to three factorizations of the shifted matrix, each with one pass for N′_L.
>
>    All are charged to the verification attempt. **Measured:** on the controls the shift runs on 3 of 59 (SKEW6-K1E-12, and the two HH-FOOL LOADED frames where it fails), and on RF-LARGE at every verification except CONT-AX, where Uc is tight.
> 8. **The estimate serves availability only (R5; extended in R6).** The verification's Hager–Higham estimate est stays where r5a.2 put it, in the condition screen (§4.1.3). There it escalates ill-conditioned candidates early, which is availability.
>    - **In R6 it also chooses σ_c** (7c), through est_c from the screen's own solves. That is availability too: a poor σ_c costs the shift bound, never honesty. **No step of the guarantee uses est** (§5.9).
>    - **F is withdrawn** (R5). R4 pinned F = 2^16 as an allowance in ‖K̃_P⁻¹‖₁ ≤ F·est. V4 showed that no pinned F is a bound: within the design's screens the estimator's miss is unbounded (V4-U1). Examples, in emu5's natural order:
>      - HH-FOOL-m, a 12-DOF frame, misses by 7.7e8, 8.5e20, 8.9e26 and 1.1e57 at m = 40, 80, 100 and 200 (V4, and emu5 in `uc_check5`);
>      - V4's matrix family F2 misses by 3.3e66 at p = 256.
>
>      In K4's order the HH-FOOL frames fool the estimate less (it misses by about 2^5.6; `uc_check6`), which changes nothing: the bound does not use it.
>    - **R4's statement that the alternating-sign vector is the estimator's safeguard is withdrawn** (R5). HH-FOOL's hidden direction w = (1, −1, −1, 1) is orthogonal to e and to the alternating vector by construction.
> 9. **θ per block with data (R4; per body in R5; per block in R6, V4-V5).** For each block c that carries data (7a), form θ_c = B_c·2^(7−P)·‖SĀS‖_c, rounded upward, with ‖SĀS‖_c over the block's free rows and columns (item 6). Test: θ_c ≤ 1/2.
>    - A block without data needs no Lemma C: every term there is exactly zero (7a).
>    - R5's per-body test also tested a data-carrying body's zero-state blocks (V4-V5: THETA-STUB).
> 10. **g (R4; scoped in R5, V4-U2; per block in R6).** Every member with a nonzero prescribed DOF, or with a free DOF in a block that carries data, satisfies g ≤ 2^(P−16) (g of §4.1.6.2 item 2, formed at P).
>     - Any other member's entries multiply only zero displacements, in K_P, K^c, K\* and every recovery, so Lemma B is not needed for it (derived).
> 11. **The charge (R4; certified in R5; per block in R6).** For every published force or moment row q of body b:
>
>     `C_q = ‖ā_q S‖₁·(t₁ + t₂ + t₃)`
>
>     - t₁ = 2^(7−q_W)·B_b·N_u, with N_u = ‖SĀ|u⁰|‖_∞ + 2·B_b·‖SĀS‖·‖S·r‖_∞;
>     - t₂ = 70·2^-P·‖S⁻¹δ̂‖_∞;
>     - t₃ = 3·B_b·‖S·r₂‖_∞;
>     - every norm is the largest over body b's blocks with data (7a).
>
>     **Test,** decided exactly (C_q formed as one exact expansion rounded upward):
>     - at p = 128 and 256: C_q ≤ 60·2^-2p·ê(body, kind);
>     - at p = 512: C_q ≤ 2^-22·2^-64·M_q, with M_q the stop rule's max(|q_2p|, S\*).
>
>     t₁ is V4's term 62.5·2^-q_W·‖a_q·S‖₁·‖K̃⁻¹‖₁·‖S·Ā·|u|‖_∞, with the certified B_b for the norm, the factor 2 from item 9's Neumann bound and the γ-margins of Lemma B. t₂ and t₃ are the second-order remainder, measured at runtime.
> 12. **Translation and rotation rows (R4).**
>     - For a free displacement or rotation row i of body b: W⁺_i = |δ̂_i| + s_i·(t₁ + t₃).
>     - For a displacement magnitude: the sum of its components' W⁺. A constrained component contributes its prescription's rounding, |u_P,c − u\*_c|, which is computed exactly.
>     - §4.1.6's rule (a) takes V_q = W⁺_q for these rows, plus 2^(1−2p)·|q_2p| for a magnitude (its own two roundings).
> 13. **Failure.** Items 4, 7d, 9, 10 and 11 reject p with the reasons `verification_estimate`, `uc`, `theta`, `g_validity` and `charge`. The schedule escalates exactly as after a stop-rule rejection. At the ceiling the case is Unresolved (Ceiling).
> 14. **Evidence.**
>     - Per body and kind: the worst Ŵ_q/V_q and the worst C_q/allowance, rounded upward.
>     - Per body: the largest θ_c and B_b, rounded upward.
>     - G5a checks that they decode to ≤ 2^-2, ≤ 1, ≤ 1/2, and a finite positive value (§6.3).
>     - **Encoding (R6; V4-V6).** A B_b that is not below 2^1024 is not encodable in binary64. The case then fails with `receipt_encoding`, as an unencodable E does. **It cannot happen at a selected p (derived):**
>       - B̄ ≥ |B| entrywise (the bounded row puts 1 or |1/L| where B has a direction cosine or its multiple), |D| ≥ D and g ≥ 1, so Ā_ii ≥ K_P,ii·(1 − 2^-(P−8)).
>       - The radix equilibration makes K̃_ii ≥ 1/2 whichever mantissa convention the exponent uses (emu6: K̃_ii ∈ [1, 4)), so ‖SĀS‖_c ≥ 1/4.
>       - θ_c ≤ 1/2 then gives B_c ≤ 2^(P−6) ≤ 2^1018.
> 15. **Work (charged to the verification attempt).**
>     - One formation pass of the element and directional-block entries at q_W. The gate's K^q at P + 64 stays.
>     - Two exact expansions over the contributions: r and r₂.
>     - One pass over Ā's rows: ‖SĀS‖ and ‖SĀ|u⁰|‖_∞.
>     - One E-like pass with operands s: ‖ā_q S‖₁ for every row.
>     - One substitution pair and one linear recovery pass for W.
>     - One substitution pair and one pass for Uc (R5).
>     - Up to three shifted factorizations, only when 7c's condition holds (R6).
>
>     S is the factor's own. est is the condition screen's, used for the screen and for σ_c only.
>
> **What W, C and W⁺ bound (derived; R4, V4-T1; certified in R5, V4-U1; per block with the shift bound in R6, V4-V1, V4-V2).**
>
> *Notation.* 𝓕 and 𝓒 are the free and constrained DOFs. K\* is the exact intended stiffness, f\* the exact ledger, and u\* the exact solution with the exact prescribed values u\*_𝓒 = u⁰_𝓒. K_P is the assembled stiffness at P. K^c is the exact, unrounded sum of the q_W-formed contributions. Δ = K^c − K\*, on free rows and every column. ũ solves K^c_𝓕𝓕·ũ_𝓕 = f\* − K^c_𝓕𝓒·u⁰_𝓒 with ũ_𝓒 = u⁰_𝓒. a\*_q is q's exact recovery row. Then, exactly, r = K^c_𝓕𝓕·(ũ_𝓕 − u_𝓕) and r₂ = K^c_𝓕𝓕·(ũ_𝓕 − u_𝓕 − δ̂).
>
> K\*_𝓕𝓕, K^c_𝓕𝓕 and K_P,𝓕𝓕 have the same structural pattern, so all three are block-diagonal over item 7a's blocks. For a block c, K̃_c is K̃_P's principal submatrix on the block, and (K̃_P⁻¹)_c = K̃_c⁻¹.
>
> **Lemma A (derived; V4's bound on Γ_q, V4-T1(b); per block in R6).** For symmetric invertible K̃ = S·K·S, block-diagonal over blocks c, and v vanishing on the blocks outside a set 𝒞:
> - |a·K⁻¹·v| ≤ ‖a·S‖₁·max_{c∈𝒞} ‖K̃_c⁻¹‖₁·max_{c∈𝒞} ‖S·v_c‖_∞;
> - |(K⁻¹·v)_i| ≤ s_i·‖K̃_c⁻¹‖₁·‖S·v_c‖_∞ for i in block c.
>
> *Proof.* K⁻¹ = S·K̃⁻¹·S, block by block. Apply Hölder's inequality on each block, and ‖K̃_c⁻¹‖_∞ = ‖K̃_c⁻¹‖₁ by symmetry. The same bound holds for \|a\|·\|K⁻¹\|·v with v ≥ 0. K\*, K^c and K_P are symmetric: K4 forms each element and directional block once per upper-triangle entry and mirrors it (V4 checked this).
>
> In the Theorem, v is r, r₂ or Δ·ũ, and all three vanish on every block without data (item 7a). So 𝒞 is the body's set of blocks with data, and the maximum of B_c over it is B_b.
>
> **Lemma B (derived: V4's count in its exact γ-form, under item 10).**
> - Each counted chain is a product of at most c factors (1 + δ) with |δ| ≤ 2^-P, relative to a nonnegative majorant. The exception is the Gram–Schmidt subtraction, whose absolute error, relative to |y_ref|, the count converts to relative error with g.
> - By Higham's Lemma 3.1, such a product lies within γ_c = c·2^-P/(1 − c·2^-P) of 1. The normalization by the cancelled vector adds one factor (1 − g·c·2^-P)⁻¹.
> - With 68·g·2^-P ≤ 2^-9.9 (item 10), each exact error is at most its first-order count times (1 − 2^-9.9)⁻² ≤ 1 + 2^-8.8. R4 stated 1 + 2^-7, which is looser and still true. **V4 derived** (R4 check 6; R5 said "measured", corrected in R6 for V4-V4) that the factor is at most 1 + 2^-9.5 for K4's Rust stages.
> - Members outside item 10's scope multiply only zeros (item 10), so they need no factor.
>
> Hence, entrywise:
> - (i) |K\*_ij − K_P,ij| ≤ 67·2^-P·Ā_ij. The count is 20g + 45.5 per assembled entry, so ≤ 65.5 relative to Ā, which carries g ≥ 1. The γ-factor and Ā's own formation at P (less than a relative 2^-(P−8)) raise 65.5 to at most 65.8;
> - (ii) |Δ_ij| ≤ 63.5·2^-q_W·Ā_ij, the count 62.5 at q_W in the same way;
> - (iii) |a\*_q| ≤ ā_q·(1 + 2^-(P−8)) entrywise. The recovery at P differs from the exact recovery of the same state with exact prescriptions by at most 69·2^-P·E_q; the count is 68, and E_q includes |u| at prescribed DOFs.
>
> **Lemma D (derived; V4-U1's bound, checked by DS1; per block in R6). Uc_c ≥ ‖K̃_c⁻¹‖₁.**
> 1. **The factor's backward error.** K4's factor (`retained/factor.rs` at `cef218a10`, which DS1 read; emu6 runs the same loop) forms each row i, in K4's elimination order, by the work-vector recurrence:
>    - work_j = fl(K̃_ij − Σ_k fl(work_k·l_jk));
>    - l_ij = fl(work_j/d_j);
>    - d_i = fl(K̃_ii − Σ_k fl(work_k·l_ik)), with work_k = l_ik·d_k·(1 + δ_k)⁻¹.
>
>    By Higham's Lemma 8.4, each computed entry satisfies K̃_ij = Σ_{k≤j} l_ik·d_k·l_jk·(1 + θ_k), with |θ_k| ≤ γ_{t+2}, where t ≤ n is the number of terms. Hence **K̃_P + ΔK = A := L·D·Lᵀ with |ΔK| ≤ γ_m·|L|·D·|Lᵀ|**, since m = 2n + 2 ≥ t + 2. This is the argument of Higham's Theorem 10.3, applied to this recurrence. V4 confirmed it against the loop as coded, and noted that γ_(t+1), even γ_n, already suffices; m = 2n + 2 is conservative. The step uses only the loop's roundings, so it holds for any symmetric input, the shifted one of Lemma E included.
> 2. **A's inverse, per block.** L, D and ΔK are block-diagonal (item 7a, and |ΔK| ≤ γ_m·|L|D|Lᵀ|). The pivot screen keeps every d_i > 0 (§4.1.3), so A_c = L_c·D_c·L_cᵀ is symmetric positive definite and A_c⁻¹ = L_c⁻ᵀ·D_c⁻¹·L_c⁻¹. For unit triangular L_c, |L_c⁻¹| ≤ M(L_c)⁻¹ (Higham's Theorem 8.12). So |A_c⁻¹| ≤ M(L_c)⁻ᵀ·D_c⁻¹·M(L_c)⁻¹ entrywise, a nonnegative matrix, and ‖A_c⁻¹‖₁ = ‖A_c⁻¹‖_∞ ≤ max_{i∈c} c_i ≤ U_c. c restricted to block c is exactly M(L_c)⁻ᵀD_c⁻¹M(L_c)⁻¹e_c, because the passes never cross a block, and every operation in 7b is monotone on nonnegative data and rounded upward.
> 3. **K̃_c's inverse.** K̃_c = A_c·(I − A_c⁻¹·ΔK_c), and ‖A_c⁻¹·ΔK_c‖₁ ≤ U_c·γ_m·N_L,c ≤ t_c < 1. By the Neumann series, ‖K̃_c⁻¹‖₁ ≤ U_c/(1 − t_c) ≤ Uc_c, with the denominator rounded downward and the quotient upward. ∎
>
> The elimination order is a symmetric permutation, which leaves each block's inverse norm unchanged. With one block, Uc_c is R5's Uc. No step uses an estimate.
>
> **Why Uc alone is not enough (V4-V1, measured).** |L⁻¹| ≤ M(L)⁻¹ discards every sign cancellation, and a frame's factor has several multipliers of order 1 per row. So M(L)⁻¹e grows geometrically along the elimination. On R1's RF-LARGE frames in K4's order, U/exact reaches 2^151 to 2^628 at 100 members (V4), and Uc does not exist at any precision for five of the six at 1,000 members (V4; §6.1).
>
> **Lemma E (derived; V4-V1's bound, checked by DS1). If 7c's shifted pivots of block c are all positive and σ′_c > 0, then λ_min(K̃_c) > σ′_c and S_c ≥ ‖K̃_c⁻¹‖₁.**
> 1. **The shifted input.** Let Ĝ be K̃_P with 7c's shifted diagonal: Ĝ_ii = d̃_i = fl(K̃_ii − σ_c) for i in block c. So Ĝ_c = K̃_c − σ_c·I + E_s, where E_s is diagonal with |E_s,ii| = |d̃_i − (K̃_ii − σ_c)| ≤ 2^-P·|K̃_ii − σ_c| ≤ 2^(1−P)·|d̃_i| ≤ δ_c.
> 2. **Its factor's backward error.** Lemma D's step 1, applied to the input Ĝ, gives Ĝ + ΔG = L′·D′·L′ᵀ with |ΔG| ≤ γ_m·|L′|·D′·|L′ᵀ|. Everything is block-diagonal (item 7a): a failed block's rows never reach block c.
> 3. **Positivity.** D′_c > 0, so A′_c = L′_c·D′_c·L′_cᵀ is symmetric positive definite: xᵀA′_c x > 0 for every x ≠ 0.
> 4. **Weyl, as a Rayleigh quotient.** K̃_c = A′_c + σ_c·I − E_s − ΔG_c. For a unit vector x:
>
>    xᵀK̃_c x > σ_c − ‖E_s‖₂ − ‖ΔG_c‖₂ ≥ σ_c − δ_c − γ_m·‖\|L′_c\|D′_c\|L′_cᵀ\|‖₂ ≥ σ_c − δ_c − γ_m·N′_L,c ≥ σ′_c.
>
>    - ‖E_s‖₂ is its largest diagonal entry, ≤ δ_c.
>    - For \|X\| ≤ Y entrywise with Y ≥ 0: ‖X‖₂ ≤ ‖\|X\|‖₂ ≤ ‖Y‖₂. The second step is monotonicity of the spectral radius of \|X\|ᵀ\|X\| ≤ YᵀY (Perron–Frobenius).
>    - For a symmetric nonnegative Y, ‖Y‖₂ = ρ(Y) ≤ ‖Y‖₁ = N′_L,c (N′_L,c is its largest column sum, rounded upward).
>    - σ′_c is formed with its subtracted terms upward and the difference downward, so it is at most the exact difference.
> 5. **The norm.** So λ_min(K̃_c) > σ′_c > 0, ‖K̃_c⁻¹‖₂ < 1/σ′_c, and ‖K̃_c⁻¹‖₁ ≤ √n_c·‖K̃_c⁻¹‖₂ < ⌈√n_c⌉/σ′_c ≤ S_c. The inequality ‖X‖₁ ≤ √n·‖X‖₂ holds column by column (‖x‖₁ ≤ √n·‖x‖₂). ∎
>
> Nothing in the proof depends on how σ_c was chosen, so est_c's use in 7c is availability only. The proof also shows, independently of the first factor, that K̃_c is positive definite.
>
> **Looseness (derived).**
> - est_c is a norm ratio of computed vectors, so it is at most ‖K̃_c⁻¹‖₁, up to rounding.
> - When the first shift succeeds, S_c ≈ ⌈√n_c⌉/σ_c = 2·⌈√n_c⌉·est_c ≤ 2·⌈√n_c⌉·‖K̃_c⁻¹‖₁, up to 1 + (γ_m·N′_L,c + δ_c)/σ′_c. Each halving doubles S_c.
> - The first shift succeeds whenever σ_c < λ_min(K̃_c) by more than the backward error. Since 1/λ_min = ‖K̃_c⁻¹‖₂ ≤ ‖K̃_c⁻¹‖₁, that holds when est_c ≥ ‖K̃_c⁻¹‖₁/2 with room to spare.
> - So S_c is polynomially loose, at most about 2√n_c times the exact norm (8√n_c after two halvings), where Uc_c can be exponentially loose.
>
> V4 gave the same derivation in its R5 check; DS1 checked each step.
>
> **Lemma C (derived, under items 7 and 9; per block in R6).** For every block c with data: ‖(S·K\*_c·S)⁻¹‖₁ ≤ 2·B_c and ‖(S·K^c_c·S)⁻¹‖₁ ≤ 2·B_c.
>
> *Proof.*
> - Write both matrices as K̃_c + E.
>   - At p = 128 and 256, Lemma B gives |E| ≤ (67·2^-P + 63.5·2^-q_W)·SĀS ≤ 2^(7−P)·SĀS entrywise.
>   - **At p = 512 (V4-U5),** q_W = P = 1024 and 67 + 63.5 > 128. But there K^c's contributions are the state's own entries, so K^c − K_P is the single assembly rounding: |K^c − K_P| ≤ 2^-P·Ā·(1 + 2^-(P−8)). And |K\* − K_P| ≤ 67·2^-P·Ā by Lemma B(i). Both are ≤ 2^(7−P)·SĀS in scaled form.
> - With ‖K̃_c⁻¹‖₁ ≤ B_c (Lemmas D and E): ‖K̃_c⁻¹E‖₁ ≤ B_c·2^(7−P)·‖SĀS‖_c = θ_c ≤ 1/2. Each factor is rounded upward, so the tested θ_c is not below the exact one.
> - So I + K̃_c⁻¹E is invertible with inverse norm ≤ 2 (Neumann series), and ‖(K̃_c + E)⁻¹‖₁ ≤ 2·B_c. ∎
>
> This is the second-order control ROOT named: ‖K⁻¹δK‖ ≤ 1/2, with a factor of 2.
>
> **Theorem (derived).** Under items 7, 9 and 10:
> - for every published force or moment row q: |q_2p − q\*| ≤ 69·2^-P·E_q + Ŵ_q·(1 + 2^-P) + C_q;
> - for every free translation or rotation row i: |u_2p,i − u\*_i| ≤ W⁺_i.
>
> *Proof.* As in R5, with B_b for Uc, and Lemma A applied over the body's blocks with data (the vectors vanish on the others).
> 1. q_2p − q\* = [R_P(u_P) − R\*(u⁰)] + a\*_q,𝓕·(u_𝓕 − u\*_𝓕). The bracket is ≤ 69·2^-P·E_q by Lemma B(iii).
> 2. u\*_𝓕 − u_𝓕 = x̃ + y, with x̃ = ũ_𝓕 − u_𝓕 = (K^c_𝓕𝓕)⁻¹·r and y = u\*_𝓕 − ũ_𝓕 = (K\*_𝓕𝓕)⁻¹·Δ·ũ. Both are exact: subtract the two systems. This step needs ũ_𝓒 = u\*_𝓒, hence item 1's exact prescribed values.
> 3. x̃ − δ̂ = (K^c_𝓕𝓕)⁻¹·r₂ exactly. By Lemmas A, B(iii) and C, |a\*_q·(x̃ − δ̂)| ≤ ‖a\*_q S‖₁·2·B_b·‖S·r₂‖_∞ ≤ ‖ā_q S‖₁·t₃.
> 4. |a\*_q·δ̂| ≤ |L_q(δ̂)| + 69·2^-P·ā_q·|δ̂| (Lemma B(iii) applied to δ̂) ≤ Ŵ_q·(1 + 2^-P) + ‖ā_q S‖₁·t₂ (Hölder).
> 5. By Lemmas A, B and C, |a\*_q·y| ≤ ‖a\*_q S‖₁·2·B_b·63.5·2^-q_W·‖S·Ā·|ũ|‖_∞, the norm over rows of blocks with data. Since |ũ_𝓕| ≤ |u_𝓕| + |x̃| and ‖S⁻¹x̃_c‖_∞ ≤ 2·B_c·‖S·r_c‖_∞, ‖S·Ā·|ũ|‖_∞ ≤ ‖S·Ā·|u⁰|‖_∞ + ‖SĀS‖_∞·‖S⁻¹x̃‖_∞ ≤ N_u. This is the ∞-norm, covered because item 6 takes ‖SĀS‖ as the larger of the two norms (V4-U8(b)). So |a\*_q·y| ≤ ‖ā_q S‖₁·t₁.
> 6. Adding steps 3 to 5 gives the force and moment bound.
> 7. For a free row i of block c, steps 3 and 5 without the recovery row give |u_2p,i − u\*_i| ≤ |δ̂_i| + s_i·2·B_c·‖S·r₂‖_∞ + s_i·127·2^-q_W·B_c·N_u ≤ W⁺_i. ∎
>
> The constants 70, 3 and 2^7 exceed 69, 2 and 127. That slack covers the upward rounding of every factor and each 1 + 2^-(P−8) factor.
>
> **Corollary: the guarantee (derived).**
> - **p = 128 and 256.** Acceptance gives |q_p − q_2p| ≤ 2^-64·M_q − V_q, Ŵ_q ≤ V_q/4 and C_q ≤ 60·2^-2p·ê. With E_q ≤ ê and V_q = 256·2^-2p·ê, |q_p − q\*| ≤ 2^-64·M_q − (256 − 69 − 64·(1 + 2^-2p) − 60)·2^-2p·ê < 2^-64·M_q − 62·2^-2p·ê.
> - **p = 512.** C_q ≤ 2^-86·M_q gives |q_p − q\*| ≤ 2^-64·M_q·(1 + 2^-22).
> - **Translation and rotation rows.** Rule (a) with V_q = W⁺_q gives |q_p − q\*| ≤ 2^-64·M_q.
> - **So b bounds |q_p − q\*| for every `absolute_verified` row,** with the factor 1 + 2^-22 at p = 512, and the relative claim holds for every relative row.
> - **No step is first order, and no step is uncertified.**
>   - The counts enter in their exact γ-form (Lemma B).
>   - The second-order remainder is bounded by Lemma C and measured by t₂ and t₃.
>   - The inverse norm is bounded by Lemmas D and E from the verification's own factor and, where needed, one to three shifted factorizations of the same matrix.
>
> **Measured** (emu6, in K4's order; `uc_check6`, `stress6`, `large6`, `run_controls6`, `gate_probe7`, `sweep6`, `r1_lane6`; §16):
> - **Per-block bounds against the exact norm: Uc_c, S_c and B_c ≥ exact on all 267 blocks** of the 234 model states (R5's 231 and THETA-STUB-COUPLED's three) (`uc_check6`, with the shift forced on every block).
>   - S_c exists on 246 blocks. It is missing on 21, all of them the H block of an HH-FOOL frame (or HH-SLENDER), where the shift failed three times: est_c misses the hidden mode, so σ_c stays above λ_min. There B_c = Uc_c, which is exact.
>   - B_c/exact ≤ 14 (SKEW6-K1E-12, where Uc_c/exact is 4,719 in K4's order; 96 in natural order). The smallest S_c/exact is 2.0.
> - **Lemma E at low precision** (`stress6`, 19,979 random factors, P = 10 to 32): S ≥ exact on all 1,457 factors where S exists, Uc ≥ exact on all 14,198 where Uc exists, and B ≥ exact on all. On V4's F2 family at P = 256 and 512 (56 shifts, σ on both sides of λ_min), none is below exact.
> - **RF-LARGE** (§6.1). At 10 members, S/exact is 2^3.3 to 2^4.0, and Uc/exact up to 2^47. At 100 members, S is 2^5.5 to 2^5.6 above the norm computed at 512 bits, where Uc is up to 2^191 above it or does not exist. At 1,000 members S is at most 2^7.3 times the norm (against est; the exact norm was not computed), and Uc does not exist at any precision on five of the six.
> - **Where the shift runs.**
>   - On the controls: on 3 of 59, all at every verification. On SKEW6-K1E-12 it runs once and tightens B from 2^90.6 to 2^82.2. On HH-FOOL-m40- and m100-LOADED it runs three times and fails each time; there Uc_c is exact.
>   - On RF-LARGE: at every verification except CONT-AX, where Uc is tight.
> - **At every selected pair of the controls** (`run_controls6`):
>   - the worst C_q/allowance is 1.6e-3, on CHARGE-SLENDER and HH-SLENDER-m40 at 128; the next worst is 5.8e-12;
>   - the largest θ is 5.1e-21;
>   - every selected control is honest, with no G5a failure.
> - **R5's measurements** (emu5, natural order) remain in R5 as history: Uc ≥ exact on 245 states; U/exact ≤ 96.4 there; the F2 factor where U alone fell below exact.
>
> **Withdrawn or replaced from R5.**
> - "Uc is one global bound" and "a per-body Uc is not proposed" (R5 §5.5 item 7, §8 item 4) are withdrawn. Every bound is per block (V4-V2).
> - R5's per-body θ and g scope is narrowed to blocks with data (V4-V5).
> - R5's "Uc … moves no sweep, probe or R1 outcome" held only because the R1 lane left out RF-LARGE (V4-V1). R6's lane includes it (§6.1).
> - R5's measurements of est and Uc were in emu5's natural order; K4 factors in RCM order, where U differs (V4). R6's figures are in K4's order.
>
> **Withdrawn or replaced from R4 (kept from R5).**
> - F, its value 2^16, and every "modulo F" are withdrawn. The certified bound replaces F·est (items 7 and 8; Lemmas D and E).
> - R4's "est is exact to rounding on these small matrices; F = 2^16 leaves 16 bits" stood as a claim about the estimator's reliability. It is withdrawn: the miss is unbounded within the screens (item 8).
> - The alternating vector is not a safeguard (item 8).
>
> **Withdrawn or replaced from R3 (kept from R4).**
> - "The rcond screen … does not bound Γ_q" was replaced by Lemma A with Lemma C (V4-T1(b)). The bound is now certified by Lemmas D and E.
> - R3's conjecture on e_q was withdrawn: e_q is charged (t₁) and bounded (Lemma C).
> - R3's citation of V4's statement that only assembly-level sums can saturate in an exactly representable structure was dropped (V4-T2). V4's erratum: TILT-LEVER's member is formed exactly at every P ≤ 576, yet its (uy,uy) entry carries an axial (t/L)² tail lost within the entry. DS1's own example stands (direction components differing by more than 2^(p+32)).
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

**Effect on every control** (measured in emulation: R3's `run_controls3`, `gate_probe4`, `r1_lane3` and `sweep3`; unchanged under R4 by `run_controls4`, `gate_probe5`, `r1_lane4` and `sweep4`, under R5 by `run_controls5`, `gate_probe6`, `r1_lane5` and `sweep5`, and under R6, in K4's order, by `run_controls6`, `gate_probe7`, `r1_lane6` and `sweep6`). Every probe result below is **emulation only** (V4-R3): K4's Rust probe runs only the coalesced gate, and K4's Rust control for the hybrid gate is not yet built.
- **K4's Rust probe cases,** rebuilt in emu3 to emu6: a (3,4,0) run of 1 or 3 N-section members, root fixed, five tip loads, y_ref (3,4,5) or (0,0,1).
  - Under the coalesced gate the emulator refuses the same 8 single-mode y_ref (3,4,5) cases as Rust (Unresolved), with gate ratios 1.7e15–2.2e16. The general load and every y_ref (0,0,1) case are selected at 128, as in Rust.
  - One difference, now closed: in the one-member in-plane case, Rust (RETURN §21) and V4's emulator fail the coalesced gate at 256, while emu2 to emu5, in natural order, pass that state and then fail its 512 verification. **In K4's order (emu6) the emulation fails the gate at 256 as Rust does** (`gate_probe7`). The outcome (Unresolved) is the same throughout.
  - Under the hybrid gate **all 20 are selected at 128, honest, with 0 G5a failures,** as V4 reproduced independently. The 8 single-mode cases pass the bounded test on their best state after the coalesced-driven corrections (three in emu5's order; in K4's order the gate stops after one or three, and the best state moves in five cases, with the same outcome); the other 12 pass the coalesced test with none.
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
>   The premise that the verification's error is small against the stop rule's tolerance is then derived, beyond first order, with every step certified: the inverse norm is bounded per block by B_c = min(Uc_c, S_c), from the verification's own factor and, where needed, shifted factorizations of the same matrix (§4.1.6.3 item 7 and Lemmas D and E). Revision 5a.3's R4 text, "modulo F", is withdrawn (R5).
> - **Negative controls:** F-2, F-2 with S\* > 0, the prescribed tail, F-2 at the ceiling, ASSEMBLY-SAT, F-3 (three forms), DEMOTION2, SEEDED-COMMON, SEEDED-SOFT (R5), LEVER2 (three gains), TILT-LEVER (three gains) and CHARGE-SLENDER join §7.3.
> - **In the factorization and triangular solves,** non-saturated error changes at 2p and the stop rule sees it. The refinement residual at p + 64 checks the solve against the exact-expansion right-hand side.
> - **The same exact-sum rule holds on the ordinary binary64 route** through S11-K and S11-F, as before.

### 5.8 §5 item 1 (the receipt): add to a `selected` entry

> - **`resolution_scale`** (revision 5a.3): per body, `force` and `moment` bit strings: E_fo and E_mo of §4.1.6.2, uncoupled, rounded upward, finite, +0.0 only when zero. Likewise in a combination's entry.
> - **The stop-rule summary** is the worst (|q_p − q_2p| + V_q)/M_q, rounded upward, and decodes to ≤ 2^-64.
> - **`verification_estimate`:** per body and kind, the worst W_q/V_q, rounded upward, decoding to ≤ 2^-2.
> - **`verification_charge`** (R4; R5; R6): per body and kind, the worst C_q divided by its allowance, rounded upward, decoding to ≤ 1. Also, per body, the largest θ_c over its blocks with data, rounded upward and decoding to ≤ 1/2, and **B_b**, the largest certified bound over those blocks, rounded upward, finite and positive (R6; R5 published one global Uc). A B_b not below 2^1024 would fail the receipt with `receipt_encoding`, as an unencodable E does; θ ≤ 1/2 makes that impossible at a selected p (§4.1.6.3 item 14, derived). R4's F bits are withdrawn with F.

### 5.9 §4.1.3 "Condition estimate": add one sentence (R4; replaced in R5; extended in R6)

> **The estimate's role (revision 5a.3, R5 and R6).** The estimate est serves availability only. It screens ill-conditioned candidates early (rcond ≤ 2^-(p−1), unchanged), and in R6 its solves choose the shift σ_c of §4.1.6.3 item 7c. It carries no part of the honesty argument.
> - **Its miss is unbounded within the screens.** On a 12-DOF frame whose soft mode is orthogonal to every vector the estimator applies K̃⁻¹ to, including the alternating-sign vector, est misses ‖K̃⁻¹‖₁ by 8.9e26 while both screens pass. On a 16-DOF matrix the miss is 3.3e66 at p = 256 (V4's measurements; DS1 reproduced the frame's in emu5's natural order; in K4's order the same frame misses by about 2^5.6).
> - **So a missed ill-conditioning may pass the screen.** On the retained-precision route it then meets the stop rule and W, which measure, and θ, the charge and W⁺, which bound. Those three use the certified bound B (§4.1.6.3 item 7), never est. A miss in σ_c only makes the shift fail, and then Uc_c stands alone.
> - **The alternating-sign vector is not a safeguard** against such a miss: it is orthogonal to the soft mode by construction.

## 6. Effects downstream

### 6.1 The discriminating controls (§7.3, §4.10)

Measured with R6's rule, in K4's elimination order: bounded-gate acceptance on the best state, the contribution-level estimate at q_W, and the charge with B_c = min(Uc_c, S_c), θ and the g check per block with data. It is compared against K4's rule and gate, against R4 and against R5, all four in K4's order (`run_controls6`). emu6 run with emu5's order and R5's or R4's switches reproduces `run_controls5` exactly on all 58 of R5's controls (`reproduce6`).

**Every selected case is honest.** Every control keeps R5's precision and classes except two, which move from 256 to 128 by design:
- THETA-STUB, because its stub's zero-state block is not tested (V4-V5);
- HH-SLENDER-m40, because the cantilever's bound is its own (V4-V2).

R6's new control, THETA-STUB-COUPLED, is at 256.

| Control | Kills | Outcome |
|---|---|---|
| k = 1e-28 | D5, D6; M18 (R4) | 128 fails the condition estimate; 256 — unchanged. With the charge at q = 2p + 64 (M18) it moves to 512 |
| Six-member k = 1e-12 | K4-M14; SD-G1; M18 | 128 rejected on Rx; 256 — unchanged |
| B1-L | D13 | 128 — unchanged; the loads act at free DOFs, which do not enter E. (R4's M18 kill here came from F's 2^16; §7) |
| S8-W (both) | D17, D20; SD-J2 | 128 — unchanged |
| PIVOT | M18 (R4) | 256 — unchanged; 512 under M18 |
| REACTIONS-ONLY | K4-M16; SD-G4 | 128 rejected by R(0, Uy); 256 — unchanged |
| TWO-SPAN | K4-M11 (O5) | 128 with one correction — unchanged |
| LEVER2 (gains 2^90, 2^100, 2^110; V4-R1) | M15 (M11 and M14 no longer: the charge also refuses LEVER2 at 256, and at q_W = 832 even assembled entries resolve its spring) | **Unresolved (safe):** 128 fails the pivot test, 256 is rejected by the estimate (and would be by the charge, at 4.1e79 or more of its allowance under R6's B; 1.6e79 under R5's Uc in natural order), 512 by the stop rule (1024 resolves the spring). Today and R2 select 256 with false claims |
| TILT-LEVER (gains 2^90, 2^100, 2^110; V4-T1(c)) | — | **Unresolved (safe)**, today and under R3 to R6: the stop rule rejects 256 and 512 (Δ ≠ 0 at the leak site). Under R4 to R6 the charge (R6: ≥ 1.0e30) and θ (R6: ≥ 6.0e10) would also refuse 256 |
| CHARGE-SLENDER (R4) | M3, M7, M18, M26 (R5, R6); M7, M17, M18 and M19 in R4 | **128 (R5, R6; 256 in R4).** est is exact here, so R4's F·est was 2^16 times the norm and refused 128 at 102 times its allowance. The bound equals the norm, and the charge is 1.6e-3 of its allowance. Under M26 (F·est) it returns to 256 |
| HH-SLENDER-m40 (R5; DS1) | M24, M26, M29 (each at 256); M3, M7, M18 | **128 (R6; 256 in R5)**: HH-FOOL-m40's unloaded frame plus CHARGE-SLENDER's loaded cantilever as a third body. With per-block bounds the cantilever's charge uses its own B (1.6e-3 of the allowance). R5's global Uc carried the hidden mode into it (M29). Honest either way. M17 no longer changes it |
| THETA-ZERO-BODY (V4-U2) | M25 | **128 (R5; 256 in R4).** N05 plus an unloaded cantilever with Iy = Iz = 2^-400. That body carries no data, so θ is not tested there (θ over the data bodies: 7.7e-63). M25 restores R4's unscoped θ and selects 256 |
| G-FIXED-MEMBER (V4-U2) | M25 | **128 (R5; 256 in R4).** N05 plus a fully fixed member with g = 2^301 that moves no DOF, so the g check does not apply to it. M25 selects 256 |
| THETA-STUB (R5; DS1) | M25, M29 (each at 256); M18 | **128 (R6; 256 in R5).** A unit member loaded with Fy = 1 and, from the fixed node 0, a stub (A = 2^50, Iy = Iz = 2^-204) free only in uz at its far node. Node 0's restraint makes uz(2) a block of its own, with no data, so θ is not tested there (V4-V5) |
| THETA-STUB-COUPLED (R6; DS1) | M20 (128), M2, M7, M18 | **256.** The stub hangs from node 3, whose rotations are free, so uz(2) is in the loaded block, and θ = 28 at the 128 verification refuses 128. The charge there is 5.6e-12 of its allowance. Without θ (M20) it is selected at 128, honest: an availability kill |
| G-PRESC-MEMBER (R5; DS1) | M23, M10 | **256.** A unit member loaded with Fy = 1, and in the same body a member with g = 2^245 whose far node is fixed with ux prescribed to 2^-300. The prescription is nonzero, so the g check applies, and it refuses 128 (g > 2^(256−16)). Without the check (M23) it is selected at 128, honest. An availability kill |
| SEEDED-SOFT (V4-U3; test-only seed) | M22 (false), M15 (false) | **Unresolved (safe)**, as in R4. The seed of 2^40 m on the soft node's ux moves the soft member's force by 2^-260 N, inside V/4, but corrupts its displacement. W⁺ refuses u(node 2, ux) at 128, 256 and 512. Without W⁺ (M22) it is selected at 128 with a false claim. Today's rule selects 128 with a false claim |
| HH-FOOL-m40, m100 and their LOADED forms (V4-U1) | M2 and M7 (both LOADED forms); M18 (m100-LOADED) | m40 and m40-LOADED at 128; m100 at 128; m100-LOADED at 256 (the stop rule rejects 128), in R4, R5 and R6 alike. In K4's order est_c misses the H block's norm by 2^4.4 to 2^5.6 (natural order: 7.7e8 and 8.9e26). Uc_c is exact there (U/exact = 1), and the shift, triggered on the LOADED forms, fails three times. The charge at the selected pair is at most 6.3e-46 of its allowance |
| N05, N06, SKEW-\*, AXIS, OBLIQUE, DUPLICATE, PRESCRIBED, ZERO-TORSION-345, ALL-ZERO-BODY, MIXED, SKEW-K1E-60 | — | Unchanged precision and classes |
| RF-CHAIN, RF-SKEW, RF-WEAK, RF-FINITE, RF-CANCEL (nodal), RF-MECH-LINE345-RX-COMPANION | Their negative controls | 120 of 120 unchanged precision (all 128) and classes, under R2 to R6; R5 and R6 in K4's order equal R5 in natural order case by case (`r1_lane6.stdout.json`); largest W/V 0.0029; worst charge 2.0e-41 of its allowance (R5: 7.4e-41); 0 G5a failures |

**R1's RF-LARGE lane (R6; V4-V1, ROOT's ruling;** `large6_10.json`, `large6_100.json`, `large6_1000.json`**).** R1's own RF-LARGE reference models, through `r1_adapter`, in K4's elimination order, under today's rule, R4 (F·est), R5 (one global Uc) and R6. Each is one block. Honesty is checked against emu6's P = 2048 reference. "Uc/norm" and "S/norm" are log₂ ratios at the P = 256 state:
- at 10 members, against the exact ‖K̃⁻¹‖₁ in rationals;
- at 100 members, against the norm computed at 512 bits, one solve per column. That reference is not exact, but its error is far below the margins shown.

| Model | Free DOFs | Today | R4 | R5 (its rejections) | R6 | log₂ Uc/norm | log₂ S/norm |
|---|---|---|---|---|---|---|---|
| CHAIN 10-AX | 60 | 128 | 128 | 128 | 128 | 19.4 | 4.0 |
| CHAIN 10-ROT | 60 | 128 | 128 | 128 | 128 | 47.1 | 4.0 |
| TREE 10-AX | 60 | 128 | 128 | 128 | 128 | 7.0 | 4.0 |
| TREE 10-ROT | 60 | 128 | 128 | 128 | 128 | 29.2 | 4.0 |
| CONT 10-AX | 45 | 128 | 128 | 128 | 128 | 0.0 | not run (Uc tight) |
| CONT 10-ROT | 45 | 128 | 128 | 128 | 128 | 17.9 | 3.3 |
| CHAIN 100-AX | 600 | 128 | 128 | 512 (`uc`, `charge`) | 128 | none (t ≥ 1) | 5.6 |
| CHAIN 100-ROT | 600 | 128 | 128 | 512 (`uc`, `uc`) | 128 | none (t ≥ 1) | 5.6 |
| TREE 100-AX | 600 | 128 | 128 | 256 (`charge`) | 128 | 151.2 | 5.6 |
| TREE 100-ROT | 600 | 128 | 128 | 512 (`uc`, `charge`) | 128 | none (t ≥ 1) | 5.5 |
| CONT 100-AX | 450 | 128 | 128 | 128 | 128 | 0.0 | not run (Uc tight) |
| CONT 100-ROT | 450 | 128 | 128 | 256 (`charge`) | 128 | 190.6 | 5.5 |
| CHAIN 1000-AX | 6000 | 128 | 128 | Unresolved (`uc` at 128, 256, 512) | 128 | none (t ≥ 1) | ≤ 7.3 (against est) |
| CHAIN 1000-ROT | 6000 | 128 | 128 | Unresolved (`uc` at 128, 256, 512) | 128 | none (t ≥ 1) | ≤ 7.3 (against est) |
| TREE 1000-AX | 6000 | 128 | 128 | Unresolved (`uc` at 128, 256, 512) | 128 | none (t ≥ 1) | ≤ 7.3 (against est) |
| TREE 1000-ROT | 6000 | 128 | 128 | Unresolved (`uc` at 128, 256, 512) | 128 | none (t ≥ 1) | ≤ 7.3 (against est) |
| CONT 1000-AX | 4500 | 128 | 128 | 128 | 128 | 0.0 (tight) | not run (Uc tight) |
| CONT 1000-ROT | 4500 | 128 | 128 | Unresolved (`uc` at 128, 256, 512) | 128 | none (t ≥ 1) | ≤ 7.1 (against est) |

- **At 10 and 100 members:**
  - every run of every model is honest, with 0 G5a failures;
  - R6's classes equal today's;
  - R5's selections reproduce V4's `r5_large_sched_rcm` exactly.
- **S/norm is 2·⌈√n⌉·est/norm on every shifted case:** 2^4.0 = 16 at 60 DOFs, and 2^5.6 = 50 at 600. The estimate equals the norm to rounding on these frames, so S is as tight as Lemma E allows with σ = 1/(2·est).
- **The shift ran once per verification on the ten loose cases, and never on CONT-AX,** where Uc equals the norm. R6's worst charge at the selected pair is 4.5e-44 of its allowance.
- **At 1,000 members (4,500 to 6,000 free DOFs), the full schedule ran in emulation for all four rules, with honesty against emu6's P = 2048 reference** (`large6_1000.json`; 2.5 to 9 minutes per model, 36 minutes in all).
  - **R6 selects all six at 128, honest,** with today's classes and 0 G5a failures. The worst charge at the selected pair is 1.4e-39 of its allowance. Today's rule and R4 also select all six at 128, honest.
  - **R5 leaves five of the six Unresolved (Ceiling)** with `uc` at 128, 256 and 512, as V4 predicted: Uc does not exist at P = 256, 512 or 1024. CONT-AX, where Uc is tight, stays at 128.
  - **What was not computed:** the exact ‖K̃⁻¹‖₁ at 6,000 DOFs, which would need 6,000 rational or high-precision column solves.
    - The looseness is therefore reported against est, a lower bound on the norm up to rounding: S/est = 2·⌈√n⌉, that is 2^7.3 at 6,000 DOFs and 2^7.1 at 4,500. So S/norm ≤ 2^7.3.
    - As a check that σ′ < λ_min, which Lemma E certifies: λ_min estimated by 12 steps of inverse iteration at 512 bits exceeds σ′ by 2^1.5 to 2^2.1 in every case. That estimate is not a certified bound.
  - The shift ran once per verification on each of the five loose frames, and its first σ succeeded every time.

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
- for these ratios, RF-LARGE with 1,000 or 10,000 members (the RF-LARGE lane above runs 1,000);
- the 7 generator cases (RF-LARGE n10000 and RF-MECH-LINE-IN-CHAIN1000);
- RF-RANGE-THIN-A and THIN-B (their section format is outside DS1's adapter).

**No discriminating control stops discriminating.** R1's not-covered set (RF-WEAK 46, RF-CANCEL 3, RF-SKEW 2) is unchanged.

### 6.2 K4's SD-tagged tests (ROOT's list)

| Tag | Under the revision |
|---|---|
| SD-G1, SD-G2, SD-G3, SD-G4, SD-G6, SD-I1, SD-I2, SD-J2, SD-K1 | **Unchanged** (measured on the emulated controls, or derived: SD-G3 and SD-I2 have E = 0, so V = 0, Φ = 0 and W = 0) |
| SD-G5 | **Translation vectors change only by W⁺** (R4: V_q = W⁺_q for translation and rotation rows; 0 when the state is exact). Add:<br>• force-kind boundary vectors at \|Δ\| = ε·M − V (accepted) and one 2p-ulp above (rejected);<br>• an estimate boundary at W = V/4 and one ulp above;<br>• the gate's bounded test at its boundary, on a best state that is not the last;<br>• an estimate residual whose assembled and contribution-level forms differ (LEVER2's shape);<br>• R4: charge boundaries at C_q equal to its allowance and one ulp above, at p = 256 and p = 512; θ_b = 1/2 and one ulp above; g = 2^(P−16) and 2^(P−15); a translation row at \|Δ\| + W⁺ = ε·M;<br>• R5: Uc's vectors: a factor with t = U·γ_m·N_L just below 1 (Uc exists) and at 1 (`uc`); the directed roundings; a factor on which U alone falls below the exact norm, so that the γ_m term is needed (F2-m100 at p = 128); θ and the g check on a body without data (not tested) and with one nonzero prescription (tested);<br>• R6: blocks: a body with a zero-state block (not tested) and a block coupled only to a nonzero prescription (tested); the shift: a block where it is not triggered (Uc_c ≤ 2⌈√n_c⌉·est_c), one where the first shift succeeds, one where it fails once and succeeds at σ/2, one where all three fail (Uc_c alone, or `uc`), and one where the pivots pass but σ′ ≤ 0; K4's elimination order on RF-LARGE-CHAIN-n00100-AX, where Uc does not exist at P = 256 and the shift bound selects 128 |
| SD-L1 | **Changed:** the E pass, the estimate and the charge are charged to the verification: the q_W formation pass, two residual passes over the contributions (r and r₂), one pass over Ā's rows, one E-like pass with operands s, one substitution pair and one recovery,, (R5) Uc's substitution pair and pass over L's profile, and (R6) the per-block sums in the screen's solves and, only when triggered, up to three shifted factorizations with one pass each. The gate adds no correction where today's gate passes, so other counts are unchanged. Re-pin, with the E pass, the estimate, Uc, the shift and the charge as their own stages |
| SD-J1 | **Values unchanged** for vectors with p ≠ 512 or E = 0. The format gains (selected p, E_fo, E_mo). Add item-6a vectors: Φ binding and not binding, the fl↑ boundary, ê < 2^-584 |

### 6.3 D2 (r5b.2)

**Fields.** `resolution_scale`, `verification_estimate` and, since R4, `verification_charge` and θ in the receipt; in R5, Uc in place of R4's F; in R6, the per-body B_b and θ in place of R5's one Uc (§5.8).

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
   - **Measured:** 0 failures over 59 controls, the 20 probe cases, 2,000 sweep frames, 120 R1 cases and 18 RF-LARGE frames, under R6 as under R5 and R4 (`run_controls6`, `gate_probe7`, `sweep6`, `r1_lane6`, `large6`). Mutants M5 and M7 fail it on several controls (`mutants6.stdout.json`). M2 fails the sanity bound (item 3) on TWO-SPAN. The emulator's rows are in SI units, so it does not exercise the conversion.
   - Item 4 cannot catch LEVER2's class, because E is correct there (V4); the estimate does.
   - It closes the under-report gap exactly where the floor binds. On RIGID-UNLOADED, LB = 0.4 against S\*_c ≈ 1e-132 (V4).
5. **Estimate summary:** each `verification_estimate` entry decodes to ≤ 2^-2.
6. **Charge summary (R4; R5; R6):** each `verification_charge` entry decodes to ≤ 1; each body's θ decodes to ≤ 1/2; each body's B_b decodes to a finite positive value (R6; R5 had one Uc; R4's F bits are withdrawn). A B_b that does not encode fails the receipt with `receipt_encoding` (§5.8). A reader checks the summaries' bounds, not the charge itself: C_q and B_b depend on the unpublished verification state, its factor and any shifted factorizations, and the Rust replay audit (§4.1.8) recomputes them.

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

R4's F = 2^16 (`0x40F0000000000000`) is withdrawn with F (R5).

**Order:** item 6 coupling → item 6a (p = 512 only) → item 7.

**Unchanged:** R = 2^-34, t = fl(R·S\*), the S\* < 2^-988 rule, and b.

### 6.5 F2a's contract

F2a (atomic with S-G1):
- maps K4's E, the estimate summary and, since R4, the charge summary and θ into the receipt, with the per-body B_b (R6) in place of R5's Uc and R4's F;
- applies item 6a in the producer classification;
- keeps product-derived rows on the updated fo and mo;
- maps an unencodable E to `receipt_encoding`;
- lands the G5a and G5b changes, D2's parity files and the shared case files in the same PR.

**F2a's merge gate is unchanged.** The combination-as-solve rule needs nothing more.

**K4's own fix, outside this addendum.** K4 fixed the combination's prescribed-row double rounding (V4-N7) at `5ad1b6174`: the row is published from its exact sum, rounded once, with control and mutant K4-M33. R4's exact prescriptions in W's residual (§4.1.6.3 item 1) use the same exact terms.

**F3 (W1b) owes one extension (R4; corrected in R5, V4-U4).** W1a's ledger is exact (nodal loads), so the Theorem needs nothing from it. W1b's formed equivalents (uniform, thermal, thrust, constant effort, generated equivalent-static loads) are formed values. Let c_f be the formation count F3 states for a formed term: at precision q, each term is within c_f·2^-q·\|term\| of its exact value, in the γ-form of Lemma B. Let T_i = Σ\|terms\| at DOF i.

F3 owes three things.
1. **Residual.** F3 enters the formed terms into W's residual as formed at q_W, as R4 said.
2. **Charge (derived; R4's form under-charged).**
   - With a formed ledger f_W, the Theorem's step 2 becomes y = (K\*_𝓕𝓕)⁻¹·[(f\* − f_W) − Δ·ũ].
   - By Lemmas A and C, the load part contributes at most ‖ā_q S‖₁·2·B_b·max_i s_i·c_f·2^-q_W·T_i to row q.
   - t₁ = 2^(7−q_W)·B_b·N_u covers that if N_u gains **max_i s_i·(c_f/63.5)·T_i**, since 2^7/63.5 > 2·(1 + 2^-(P−8)). Step 7 and W⁺ follow in the same way.
   - R4 added s_i·c_f·2^-q_W·T_i to N_u. That multiplied the load error by a second 2^-q_W: it under-charged by about 2^(q_W−6) (V4-U4(a)).
3. **Recovery (derived; missing from R4).**
   - Formed terms at a restrained DOF enter the reaction rows (R = K·u − f_c), and member-load terms enter end actions and stations. The recovery at P forms them, with error up to c_f·2^-P·T.
   - E's reaction stage uses the net \|f_c\|, which can cancel. So Lemma B(iii)'s 69·2^-P·E_q does not cover that error.
   - F3 must therefore add (c_f/69)·T to the E stage of every row a formed term enters, with its own count in Lemma B(iii). For formed terms this is the reverse of the net rule that M6 tests for W1a's exact ledger.

Until F3 does all three, W1b is outside the Theorem's scope. That is consistent with W1b being outside W1a.

### 6.6 K4's ceiling argument (route 1, step 10; route 2; V4-N5)

**Restated:**

> The stop rule can accept a wrong 512 candidate only if the 1024 state's error is common to 512 and 1024. That is either:
> - a common-mode loss of an exact input, which the exact sums remove; or
> - saturated rounding below 1024's resolution (Lemma 2).
>
> Under 5a.3, the 1024 state's recovery resolution is charged by V = 2^(8−1024)·ê (derived; exact form under the g check). Its solve error is measured by the verification estimate, whose residual is one exact sum over the 1024-formed contributions, so K_1024's assembly roundings are included (R3). The formation error of individual 1024-formed entries (Q4: no re-formation above 1024), and the second-order remainder, are charged: C_q ≤ 2^-22·2^-64·M_q (R4, §4.1.6.3 item 11), under θ ≤ 1/2.
>
> **Honesty:** b = fl↑(2^-64·Φ) bounds the accepted 512 value with the factor 1 + 2^-22, derived with every step certified (§4.1.6.3 Corollary; R4 had "modulo F"). R3's "excess ≤ 2^-516·(Γ_q/ê)·b" is superseded by the charge, which evaluates that excess at runtime and bounds it by 2^-22·b.
>
> **Availability:** the factor 4 between λ·2^-512·ê and 2^-64·Φ is the room for the 512 candidate's own resolution noise to pass the rule.

**Route 1's Oettli–Prager step.** It now uses the gate's backward error relative to |f| + Ā|u|, a larger majorant than |K||u| by a bounded factor (Ā's entries ≤ the element coefficients' sums). RETURN restates the constant.

**No uncertified step remains (R5; R6).** R4 named Hager–Higham's ‖K̃⁻¹‖₁ ≤ F·est as route 2's one uncertified step. R5 replaced it with Uc from the 1024 verification's own factor; R6 uses B_c = min(Uc_c, S_c) per block (§4.1.6.3 item 7, Lemmas D and E), so route 2's honesty rests on §4.1.6.3's Theorem alone. The estimate remains in the condition screen and in the choice of σ_c, for availability (§5.9).

### 6.7 Work (V4-N11, N12)

The sweep's work proxy, Σ(p/128)² over solves, excludes the E pass, the estimate, Uc, the shift and the charge:
- the E pass is one absolute recovery pass plus Ā's constrained rows;
- the estimate is one residual pass over the contributions (R3), one substitution pair and one linear recovery;
- the charge (R4) is one formation pass of element and directional-block entries at q_W, one more residual pass over the contributions (r₂), one pass over Ā's rows and one E-like pass with operands s;
- Uc (R5) is one substitution pair and one pass over L's profile, on nonnegative data. It costs about what the estimate's substitution pair costs: no more than one of Hager–Higham's iterations, of which the condition screen already runs up to eleven solves.
- **The shift (R6)** is up to three factorizations of the shifted matrix, each with one pass over its profile, only for a verification where some block with data has Uc_c missing or above 2⌈√n_c⌉·est_c. A factorization is the costliest step of an attempt, so the shift at most quadruples the verification's factorization work where it runs. It never runs where Uc is tight. **Measured:** it ran on 380 of the 2,000 sweep frames, on 3 of the 59 controls, and on the loose RF-LARGE frames (§6.1).

All are charged to the verification attempt.
- A residual pass has one term per contribution rather than one per assembled entry: for a member, 144 terms instead of its share of the assembled entries, about twice a gate residual.
- The q_W formation pass costs one formation of the model's elements at 448 or 832 bits, where the gate's is at 320 or 576.
- Together they are small beside a factorization for banded models of any size. For very small models, formation can be comparable to the factorization.
- The proxy is +1.1 % against today over 2,000 frames (52,480 against 51,888), identical to R5's in the same order: R6 changes no frame's solves. The shift's factorizations are outside the proxy: under R6 they ran on 380 of the 2,000 frames, 1,112 shifted factorizations against 5,494 solves. The emulator bounds every solved state, where K4 needs the bound only at the verification, so this overstates K4's count.

## 7. Tests K4 must add, and the mutants (measured in emu6, in K4's elimination order)

**Controls** (invented inputs; `models.py`, `models2.py`, `models3.py`, `lever3.py`, `models4.py`, `models5.py`, `models6.py`; the expectations are R6's, in K4's elimination order; `run_controls6.stdout.json`). Every R5 control keeps R5's expectation under R6, precision and classes alike, with two exceptions: THETA-STUB (256 → 128), whose stub block carries no data, and HH-SLENDER-m40 (256 → 128), whose cantilever now has its own bound. K4's order changes no control's precision under today's rule, R4 or R5. It moves one GS-ROT-y345 row between absolute and unpublishable, an emulation-specific GS outcome (V4-N8).

| Id | Model | Expected |
|---|---|---|
| **F-2** | I12's probe (EA/L = 512, ux(0) = 1, load 2^-300) | 128: rejected by V (Δ = 0, M = 0, V > 0). 256: rejected by Δ = 2^-300 > ε·M (N-4). **512**, with \|N\| = \|R\| = 2^-300 `relative_verified` |
| **F-2-SPOS** | Member 1 E = 2^200 with F-2's pattern and load 2^-60; member 2 unit section carrying 1 N | 512, all honest; today ratio 16 |
| **F-2-CEIL**, **PRESCRIBED-TAIL** (combination form, operands A = 1 and B = 2^-1000 with factors (1, 2^-100)), PRESCRIBED-TAIL-FREE | as the candidate | 512 via Φ, honest; today false at 128 |
| **ASSEMBLY-SAT** (V4-N2) | DEMOTION's layout with a unit member 2 | **512**, honest; today R = −1.5 N `relative_verified` against 0 |
| **DEMOTION2** (V4-S1) | E_big = 2^480; member 2's I = 2^400 | **512**, relative rows 7 → 4, b ≤ 4.7e-6 (V4: 1.67e-6 N on the force rows); documents the only demotion class |
| **EXACT-RIGID** (V4) | One unit member, both nodes fully prescribed with ux = 1 | 512 with b ≈ 1.1e-150 > 0 on the force rows (today 128 with b = 0); honest |
| **LEVER2-k90**, **-k100**, **-k110** (V4-R1; `lever3.py`) | V4's exactly representable lever (§2.5): gain 2^90, 2^100 or 2^110; prescribed rigid y-translation 1; tip spring 2^-580 of the tip's assembled diagonal; load P = 2^(⌊log2 ê⌋ − 438) on the out-of-plane branch. Every LDLᵀ pivot and multiplier of the spring-free matrix is dyadic in the natural order (checked exactly) | **Unresolved (Ceiling):** 128 fails the pivot test; 256 rejected by `verification_estimate` (W/V = 2,681, 2.7e6, 2.8e9), and the charge would refuse it too (4.1e79 to 4.8e97 of its allowance under R6; R5: 1.6e79 to 1.8e97; R4: 1.0e84 to 1.2e102); 512 rejected by the stop rule. Today and under R2's assembled residual: selected at 256, claim ratios 1,005, 1.03e6, 1.05e9. K4's Rust control uses the same model. Its expected outcome is Unresolved whatever the elimination order (argued): at 256 the estimate or Δ rejects, and at 512 Δ does, because 1024 resolves the spring. V4 built no case for K4's RCM order |
| **TILT-LEVER-k90**, **-k100**, **-k110** (V4-T1(c); `models4.py`, V4's `r3_tilt` ported) | LEVER2's lever with no spring; the arm A–C tilted to C = (2^k, −2^(k−290), 0), EA/L = 2^(s+6), so the arm's (uy,uy) entry carries an axial (t/L)² tail lost within the entry up to 576 bits; a rigid rotation ω = 2^-10 about the link node prescribed at A, G and D; P as for LEVER2 | **Unresolved (Ceiling):** 128 fails the pivot test; 256 and 512 rejected by the stop rule at the leak site (Δ ≠ 0), as V4 found. The charge would refuse 256 as well (R6: 1.0e30 to 1.1e36 of its allowance, θ 6.0e10 to 6.3e16; R5: 3.8e29 to 4.0e35) |
| **CHARGE-SLENDER** (R4; `models4.py`) | A unit cantilever along x, node 0 fixed; E = A = L = J = G = 1, Iy = Iz = 2^-180; tip load Fy = 1 N. The bounded operator sees the axial stiffness through the transverse displacement 2^180/3 | **128 (R5, R6):** honest; the charge is 1.6e-3 of its allowance, since the bound equals the norm here. R4's F·est was 2^16 times the norm and refused 128 at 102 times the allowance. Kills M26 (F·est), selected at 256, and M3, M7 and M18 |
| **RIGID-UNLOADED** (F-3), **F-3-FREE**, **F-3-ROT** | as the candidate | 512, honest; today unresolved |
| **LOADONLY-y345** and **K4's Rust probe set** (5 loads × 1 and 3 members × y_ref (3,4,5) and (0,0,1)) | K4 `5ad1b6174` | **All selected at 128**, honest (in emulation; K4 builds the hybrid-gate control in Rust). The 8 single-mode y_ref (3,4,5) cases are refused under the coalesced gate, as in Rust |
| **GS-TRANS-y345**, **GS-ROT-y345**, **GS-ROT-y345-LOADED** | — | 512, 512, 128 (V4-N8: the GS outcomes are emulation-specific; the LOADED and LOADONLY cases are the robust ones) |
| **SEEDED-COMMON** (S2's control; V4-R2) | N05 with a test-only hook that adds 2^-110 m to node 1 uy of the **final state**, after the gate and before recovery, at every precision: a common verification error the stop rule cannot see | Unresolved (Ceiling), `verification_estimate` at 128, 256 and 512, **through the specified form**: the estimate's residual is recomputed on the final state, so it contains the seed. Without the estimate's test (M11), or with the gate's residual reused (M15, R2's text), selected at 128 with claim ratio 23. The charge does not catch the seed: it is 2^-110 of the state, far inside the allowance |
| **SEEDED-SOFT** (R5; V4-U3; `models5.py`, test-only seed) | A stiff member carries 1 N. At its tip, a soft member (A = I = J = 2^-300) runs to node 2, loaded with 2^-240 N, which moves 2^60 m. The same hook adds 2^40 m to node 2's ux in the final state at every precision. The soft member's force moves by 2^-260 N, inside V/4; the displacement is corrupted by 2^-20 of itself | **Unresolved (Ceiling):** W⁺ rejects u(node 2, ux) at 128, 256 and 512 through the stop rule. **Kills M22 with a false claim:** without W⁺ it is selected at 128, claim ratio 954 (953.7; V4: 954). M15 is also false on it. Today's rule selects 128 with a false claim |
| **HH-FOOL-m40**, **-m100**, **-m40-LOADED**, **-m100-LOADED** (R5; V4-U1; `models5.py`) | V4's frame, 12 free DOFs in two bodies. Body H: four collinear nodes on t·(1,−1,0), free in ux, uy, uy, ux, with axial members that restrain every motion except w = (1,−1,−1,1), which only bending with I = 2^-m holds. w is orthogonal to e and to the alternating vector. Body V: a grounded axial chain. LOADED adds a unit x-load on each body | m40, m40-LOADED and m100 at 128; m100-LOADED at 256 (the stop rule rejects 128); honest. In emu5's natural order est misses ‖K̃⁻¹‖₁ by 7.7e8 and 8.9e26; in K4's order est_c misses H's block by 2^4.4. Uc_c equals the norm (U/exact = 1). On the LOADED forms the shift is triggered for H's block and fails three times (σ_c is too large), so B_c = Uc_c. The charge at the selected pair is ≤ 6.3e-46 of its allowance |
| **HH-SLENDER-m40** (R5; DS1; `models5.py`) | HH-FOOL-m40 unloaded, plus CHARGE-SLENDER's loaded cantilever as a third body (member 50) | **128 (R6; 256 in R5):** with per-block bounds, the cantilever's B is its own (V4-V2), so the charge is 1.6e-3 of its allowance, as on CHARGE-SLENDER. Honest. Kills M24 (the global estimate carries H's mode into the cantilever's charge), M26 and M29 (each at 256), and M3, M7 and M18. It no longer kills M17 |
| **THETA-ZERO-BODY** (R5; V4-U2) | N05 plus an unloaded cantilever with Iy = Iz = 2^-400 | **128:** the cantilever carries no data, so θ is not tested there. Kills M25 (unscoped θ), selected at 256 |
| **G-FIXED-MEMBER** (R5; V4-U2) | N05 plus a fully fixed member with y_ref = (1, 2^-300, 0), so g = 2^301 | **128:** the member moves no DOF, so the g check does not apply. Kills M25, selected at 256 |
| **THETA-STUB** (R5; DS1; `models5.py`) | Nodes (0,0,0), (1,0,0), (0,1,0). A unit member 0–1; a stub 0–2 with y_ref (0,0,1), A = 2^50, Iy = Iz = 2^-204. Node 0 fixed; node 2 fixed except uz; Fy = 1 N at node 1 | **128 (R6; 256 in R5):** node 0's restraint makes the stub's uz(2) a block of its own, with no data, so θ is not tested there (V4-V5). Kills M25 and M29 (each at 256) and M18 (512). It no longer kills M20 |
| **THETA-STUB-COUPLED** (R6; DS1; `models6.py`) | Nodes 0 (0,0,0), 1 (1,0,0), 3 (2,0,0), 2 (2,1,0). Unit members 0–1 and 1–3; a stub 3–2 with y_ref (0,0,1), A = 2^50, Iy = Iz = 2^-204. Node 0 fixed; node 3's translations fixed and rotations free; node 2 fixed except uz; Fy = 1 N at node 1 | **256:** the stub's uz(2) is in the loaded block (through node 3's rotations), so θ is tested there. θ = 28 at the 128 verification (the scaled bounded operator's uz(2) diagonal is about 2^252), so 128 is refused by `theta`. The charge there is 5.6e-12 of its allowance. **Kills M20** (selected at 128, honest): an availability kill. Also kills M2, M7 (the entrywise Ā removes the isotropic term, so θ passes) and M18. The frame is planar and loaded in-plane, so uz(2) and node 3's out-of-plane rotations carry an exactly zero state (§9) |
| **RF-LARGE-CHAIN-n00100-AX**, **RF-LARGE-TREE-n00100-AX** (R6; R1's references via `r1_adapter`) | R1's own 100-member frames (600 free DOFs), in K4's order | **128**, honest, as today and under R4 (§6.1). Kill M28 (no shift; 512 and 256) and M29 (R5's form; 512 and 256), and M2 (TREE unresolved) and M7 (G5a) |
| **G-PRESC-MEMBER** (R5; DS1; `models5.py`) | Nodes (0,0,0), (1,0,0), (0,0,1). A unit member 0–1; a member 0–2 with y_ref (2^-244, 0, 1), so g = 2^245. Node 0 fixed; node 2 fixed with ux prescribed to 2^-300; Fy = 1 N at node 1 | **256:** the member has a nonzero prescribed DOF, so the g check applies (R5: in a body with data; R6: always, since its entries meet a nonzero operand in the reactions) and refuses 128 (2^245 > 2^240). **Kills M23** (selected at 128, honest): an **availability kill**. Also kills M10 |
| **LEDGER-AT-RESTRAINT** | As the candidate | 128 |
| **M7-GS1**, **M10-G**, **R115-SEED3** | The mutant controls below | 512, 256, 512; honest |
| **E-UNIT** | Every control at every precision | E, g and Ā bits equal the generator's emulation of §4.1.6.2 |
| **E-HEADROOM** | Every control at every precision with a state, except the states the estimate or the charge rejects (LEVER2, TILT-LEVER) | \|q_P − q_2P\| ≤ 2^8·2^-P·ê. Observed \|q_P − q\*\| ≤ 1.77·2^-P·ê over 739 states (R3's `measure3_3.json`; R4 and R5 do not change the states), a margin of about 2^7. LEVER2's 512 state reaches 7.2e11·2^-P·ê: the lost spring, which W measures exactly (ratio 1) |
| **TWO-SPAN** | K4-M11 | Still one correction at 128 |
| **E-ESTIMATE** (test-only) | Every control, at P = 256 and 512 | W_q agrees with \|R\*(u_P) − q\*\| (a P = 2048 reference) within a relative 2^-8, on rows whose error exceeds 2^-(P+20)·ê. Observed within 5·10^-11 in R3, LEVER2 included |
| **E-CHARGE** (R4; R5; R6) | Every control at every verification | C_q, θ_c, W⁺ and every factor equal the generator's emulation of §4.1.6.3 items 5 to 12, with the directed roundings. The factors are: the blocks and their data flags; Uc_c with U_c, N_L,c and γ_m; S_c with est_c, σ_c, d̃, δ_c, N′_L,c and σ′_c, and the number of shifted factorizations; B_c and B_b; S; ‖SĀS‖ in both norms; ‖SĀ\|u⁰\|‖_∞; ‖S·r‖_∞; ‖S·r₂‖_∞; ‖S⁻¹δ̂‖_∞; ‖ā_q S‖₁. Emulated: the worst C_q/allowance at the selected pairs is 1.6e-3 (CHARGE-SLENDER and HH-SLENDER-m40 at 128) and otherwise ≤ 5.8e-12; 7.4e-49 (R5, natural order: 1.1e-48) on the probe set |
| **E-UC** (R5; R6 per block, with the shift) | Every control with at most 40 free DOFs, at every verification; RF-LARGE; V4's F2 family; the low-precision stress | Uc_c, S_c and B_c ≥ the exact ‖K̃_c⁻¹‖₁ (rationals), per block. Observed on all 267 blocks of the 234 model states with no failure (`uc_check6`), on RF-LARGE (§6.1), and in `stress6` (Lemma E at P = 10 to 32, and on F2 at 256 and 512). K4's control uses V4's HH-FOOL-m and F2 families, where est misses by up to 1.1e57 and 3.3e66 in natural order, and RF-LARGE-CHAIN-n00100-AX, where Uc does not exist at P = 256 |

**Mutants.** A kill is a change of selected precision, of class counts or of honesty, against R6 in K4's order (`mutants6.stdout.json`; M13's is a change of correction count, at evidence level). The control list is R5's, plus THETA-STUB-COUPLED and two of R1's RF-LARGE frames at 100 members (CHAIN-AX, TREE-AX). Every kill list below equals R5's on R5's controls, except where a row says otherwise.

| # | Mutant | Kill (emulated) |
|---|---|---|
| M1 | Drop V | F-2, F-2-SPOS, F-2-CEIL, PRESCRIBED-TAIL, ASSEMBLY-SAT: false claims at 128; M10-G and EXACT-RIGID at 128. (DEMOTION2 no longer: the charge refuses its 128 and 256 candidates) |
| M2 | ê uncoupled | **Behavioural (S6):** F-3-FREE, F-3-ROT, GS-ROT-y345 and **R115-SEED3** unresolved (the frame the candidate's emulator generates at seed 3, index 115, from `sweep.gen`; made a required control); G5a fails on TWO-SPAN. R5: also HH-FOOL-m40-LOADED and -m100-LOADED unresolved. R6: also RF-LARGE-TREE-n00100-AX unresolved, and a G5a failure on THETA-STUB-COUPLED. (In K4's order F-3-ROT is no longer among them) |
| M3 | Φ at every p | REACTIONS-ONLY 256 → 128 (SD-G4); F-2, F-2-SPOS, F-2-CEIL, PRESCRIBED-TAIL, the F-3 forms, ASSEMBLY-SAT, GS, M7-GS1, M10-ANISO, M10-G and EXACT-RIGID at 128; R115 at 256. (DEMOTION2 no longer, as under M1.) R5: also CHARGE-SLENDER, whose relative rows go 8 → 3 at 128 now that R5 selects it there. R6: also HH-SLENDER-m40, likewise at 128 |
| M4 | No Φ | F-2-CEIL, PRESCRIBED-TAIL, the F-3 forms, GS, M7-GS1, M10-ANISO, R115 and EXACT-RIGID unresolved; DEMOTION2 and ASSEMBLY-SAT relative rows 4 → 7 |
| M5 | E without the prescribed \|u\| | PRESCRIBED-TAIL and ASSEMBLY-SAT false claims at 128; RIGID-UNLOADED and M7-GS1 unresolved; M10-G and EXACT-RIGID at 128; DEMOTION2 relative rows 4 → 7; G5a failures on PRESCRIBED-TAIL, REACTIONS-ONLY, DEMOTION2, ASSEMBLY-SAT, M10-G and EXACT-RIGID |
| M6 | E from Σ\|ledger terms\| | LEDGER-AT-RESTRAINT at 256 |
| M7 | Entrywise operator in E and Ā | **Behavioural (S6):** LOADONLY-y345, GS-TRANS-y345, GS-ROT-y345 and M10-ANISO refused by the gate; G5a lower-bound failures on RIGID-UNLOADED, F-3-FREE, F-3-ROT, REACTIONS-ONLY, SKEW6-K1E-12, SKEW-K1E-28, B1-L, LEDGER-AT-RESTRAINT and R115; CHARGE-SLENDER with G5a failures. R5: also HH-SLENDER-m40 (128) and G5a failures on both HH-FOOL LOADED forms. R6: **THETA-STUB-COUPLED at 128**, because the entrywise Ā has no isotropic axial term at the stub's bending DOF, so θ passes; G5a failures on both RF-LARGE-100 controls. (M7-GS1 alone is not killed: the body-level ê masks it) |
| M8 | Φ = 2^-(2p−74)·ê | The F-3 forms, GS, M7-GS1, M10-ANISO and R115 unresolved; F-2-CEIL unresolved (R4: the charge's 512 allowance falls with the floor); DEMOTION2 and ASSEMBLY-SAT relative rows 4 → 7 |
| M9 | Floor at 256 | F-2, F-2-SPOS and all floored controls at 256, EXACT-RIGID included; DEMOTION2 stays at 512 with relative rows 4 → 7 |
| M10 | g = 1 | **Behavioural (S6):** **M10-G**, 256 → 128. M10-G uses DEMOTION2's layout with member 1's y_ref (1, 2^-12, 0) on its (2,0,0) chord, so g = 2^13 and the formation is exact; E_big = 2^173; member 2's I = 2^100. With g, ê/M = 2^186 > 2^184 rejects 128. R5: also G-PRESC-MEMBER at 128, since g = 1 disables the g check. **Why the anisotropic frame V4 suggested does not kill it (corrected in R5, V4-U7).** R2's reason, that non-orthogonality enters only at second order, was wrong. Gram–Schmidt leaves e_x·e_y ≈ 0.028·g·2^-P (V4, measured at g = 2^24 and 2^44), which is first order. For a rigid rotation ω the bending strain is, to first order, (e_x·e_y)(ω·e_x), so the exact rigid rotation about the member axis leaks about 0.007·g·2^-P·(B̄ᵀ\|D\|B̄\|u\|) = 0.007·2^-P·Ā\|u\| (derived by V4; DS1 checked the first-order expansion). **That leak lies inside V only because E carries g.** On M10-ANISO (g ≈ 2^12) it is about 29 units of 2^-P times the g-free majorant, under V's 256, so g = 1 still passes there (measured). By V4's coefficient, a member with g above about 2^15 would leak beyond V without g (estimated, not built). g is kept because the count needs it and because this leak needs it |
| M11 | Skip the estimate's test (b) | **SEEDED-COMMON** selected at 128 with claim ratio 23. (LEVER2 no longer: the charge refuses it) |
| M12 | Coalesced gate only (K4 at A2) | LOADONLY-y345, GS and M10-ANISO unresolved (with K4's Rust probe cases) |
| M13 | Bounded test drives refinement | TWO-SPAN: 0 corrections instead of 1 (the K4-M11 evidence, O5). No outcome changes, so the kill is at evidence level, as O5's |
| **M14** (R3) | W's residual over K^q's assembled entries (R2's quantity) | **Not killed in R4 or R5, and not needed for honesty (derived).** At q_W = 832, even assembled entries resolve LEVER2's 2^-580 spring. An assembled form adds one rounding per entry at q_W, so Lemma B(ii)'s count becomes 62.5 + 1, and in the γ-form (62.5 + 1)·(1 + 2^-8.8) < 63.7. t₁ then needs 2·63.7·(1 + 2^-(P−8)) < 128 = 2^7, which holds. So t₁ covers the assembled form. R4 wrote "63.5 → 64.5", and 2·64.5 = 129 > 2^7 (V4-U6); V4's corrected constant is 64 with 1 + 2^-7, and R5's is 63.7 with Lemma B's 1 + 2^-8.8. **The contribution-level form is kept for ROOT's ruling at R2:** it makes W measure every assembly sum exactly, rather than charge it |
| **M15** (R3) | W's residual reused from the gate's evaluation, before the hook (R2's text) | **SEEDED-COMMON** selected at 128, claim ratio 23. R5: also SEEDED-SOFT at 128, false |
| **M16** (R3) | The bounded test on the last state, not the best (R2) | **Not killed; acceptable** (V4-T3, ROOT). Availability only. A K4 unit test on the gate's choice (SD-G5's vector) is its kill, at evidence level |
| **M17** (R4) | Drop the charge (d) | **Not killed in R6; kept for the derivation** (V4-V2, ROOT's ruling), as M21 is. The charge is the Theorem's t₁ to t₃, and B is its norm. HH-SLENDER-m40 no longer kills it: with per-block bounds, the hidden mode's block no longer reaches the cantilever's charge, and the case is selected at 128 with or without the charge. **No single-block kill was built.** V4 found none in CHARGE-SLENDER's family (`r5_m17`: from Iy = 2^-186 the stop rule rejects first). A charge-alone refusal needs a soft mode inside the loaded block that the load does not excite. In K4's order the HH construction no longer hides its mode from the estimate (it misses by 2^4.4 to 2^5.6), and a numerically decoupled soft part of the block would show the structural block's conservatism (§9), not the charge's necessity |
| **M18** (R4) | W and the charge formed at q = 2p + 64 (R3's q) | **Detected:** SKEW-K1E-28, PIVOT, SKEW6-K1E-12, N06 and CHARGE-SLENDER move to 512; so do R5's HH-SLENDER-m40 and HH-FOOL-m100-LOADED, and in R6 THETA-STUB and THETA-STUB-COUPLED. At 2p + 64, t₁ is 2^p times its value at 3p + 64, and the charge exceeds its allowance on conditioned controls (V4: 4.4e19 on SKEW-K1E-28 with F = 1). **B1-L and LEDGER-AT-RESTRAINT no longer kill it (R5):** their charge at 128 is 7.5e-42 of the allowance, so t₁ at 2p + 64 reaches about 2^128 times that, 2.5e-3, under Uc, and about 166 with R4's extra 2^16 (derived; measured: M18 keeps both at 128 under R5 and R6) |
| ~~M19~~ (R4) | F omitted (F = 1) | **Retired with F** (R5). Its R5 analogues are M24 and M26 |
| **M20** (R4) | Drop the θ check (c) | **Killed by THETA-STUB-COUPLED** (R6), selected at 128 and honest: an **availability kill**. THETA-STUB no longer kills it: R6 does not test θ on its stub's zero-state block. In THETA-STUB-COUPLED the stub is in the loaded structural block, so θ is Lemma C's hypothesis there (θ = 28 at the 128 verification; the charge is 5.6e-12 of its allowance). The stub's DOF is numerically decoupled from the moving ones, with zero state, so the kill still shows the structural block's conservatism (§9). No honesty kill was built. The check is kept because Lemma C needs it |
| **M21** (R4) | Prescribed values rounded in W's residual (R3) | **Not killed. Vacuous for every case the W1a facade publishes, and kept for the derivation (R6 wording, V4-V3).**<br>• W1a's published cases have rigid restraints: the facade refuses nonzero support motion until W1b. So every published W1a prescription is 0 and exact, a combination's is Σλ_k·0 = 0, and M21 is the unmutated rule on every published W1a case.<br>• **DESIGN §4.2 admits nonzero prescribed motion in the kernel.** §7's own kernel controls use it (F-2, EXACT-RIGID, PRESCRIBED-TAIL, LEVER2, TILT-LEVER, G-PRESC-MEMBER), and none kills M21.<br>• A single case's binary64 prescription is exact at any P ≥ 53. Only a combination's exact expansion can be rounded by P.<br>• **No charge covers the term it would drop (derived):** with ũ_𝓒 ≠ u\*_𝓒, step 2 gains (K\*_𝓕𝓕)⁻¹·K\*_𝓕𝓒·(u\*_𝓒 − u_P,𝓒), bounded by ‖ā_qS‖₁·2·B_b·2^-P·‖SĀ_𝓕𝓒\|u_𝓒\|‖_∞, which is 2^(q_W−P−6) = 2^(p+58) times what t₁ charges for the same norm at p < 512. (R5 said "no other test implies it"; that overstated what the derivation shows.)<br>• V4 found that a translation-row kill would need a prescribed-to-free gain above 2^(P−64).<br>• The exact form is kept because step 2 needs ũ_𝓒 = u\*_𝓒 |
| **M22** (R4) | Translation and rotation rows without W⁺ (R3) | **Killed by SEEDED-SOFT with a false claim** (R5; V4-U3): selected at 128, claim ratio 954 (`m22_ratio5`). W⁺ is load-bearing |
| **M23** (R4) | Drop the g check (c) | **Killed by G-PRESC-MEMBER** (R5), selected at 128 and honest: an **availability kill**. R4's reason, that a large g makes V large, holds only on R4's controls: G-PRESC-MEMBER's g = 2^245 member carries a 2^-300 prescription, so V does not see it. No honesty kill was built. The check is kept because Lemma B needs it |
| **M24** (R5) | est in place of the bound | **Killed by HH-SLENDER-m40** (R6: 256 against 128), on availability, in the opposite direction from R5. est is one global estimate. In K4's order it carries HH-FOOL's hidden-mode norm (it misses that norm by only 2^5.6), which lies far above the cantilever's own block bound, so the cantilever's charge refuses 128. It is not a single-block kill, and it depends on the estimate's globality. **As in R5, no honesty kill was built.** Lemmas D and E, not a kill, are why the certified bound is required |
| **M25** (R5) | θ and the g check unscoped (R4's form) | **THETA-ZERO-BODY**, **G-FIXED-MEMBER** and (R6) **THETA-STUB** at 256 (R6: 128). Availability |
| **M26** (R5) | F·est in place of the bound (R4's form) | **CHARGE-SLENDER** and (R6) **HH-SLENDER-m40** at 256 (R6: 128). Availability |
| **M27** (R6) | The shift bound without its backward-error and rounding terms (σ′ = σ) | **Killed in the low-precision stress** (`stress6`): with σ above λ_min, the shifted pivots can pass by rounding, and then ⌈√n⌉/σ falls below the exact norm in 27 of 20,000 random factors (P = 10 to 32; smallest ratio 0.20), while the specified S_c never does. **Not killed on any control or RF-LARGE case (derived):** at the design's precisions the dropped term is below σ_c by a factor of about 2m·2^-P·κ ≤ m·2^-p (the condition screen bounds κ at the candidate). A selection or honesty kill would need σ_c inside that window just above λ_min, which the halving schedule does not aim for |
| **M28** (R6) | No shift: Uc_c alone | **RF-LARGE-CHAIN-n00100-AX at 512 and RF-LARGE-TREE-n00100-AX at 256** (R6: 128). Availability; the lane's other 100-member cases behave alike (§6.1) |
| **M29** (R6) | R5's form: one global Uc, per-body scope | **THETA-STUB** and **HH-SLENDER-m40** at 256, **RF-LARGE-CHAIN-n00100-AX** at 512, **RF-LARGE-TREE-n00100-AX** at 256 (R6: 128 for all four). Availability |

**The estimate's kills (V4-R2).** SEEDED-COMMON kills M11 and M15 through the specified form: the text says the residual is recomputed on the final state, and emu6 implements exactly that. SEEDED-SOFT kills M15 as well (R5). In R3, LEVER2 also killed M11 and M14; since R4 the charge refuses LEVER2 by itself, which is the charge doing its job. E-ESTIMATE is the estimate's positive test, E-CHARGE the charge's and E-UC the bound's.

**The derivation's guards (M20 to M23; R5, V4-U3).** Each guards one step of §4.1.6.3's derivation:
- **M20 (Lemma C):** killed by THETA-STUB-COUPLED, on availability;
- **M21 (step 2):** not killed; vacuous for every case the W1a facade publishes, and kept for the derivation; no charge covers the term (derived above);
- **M22 (the translation claim):** killed by SEEDED-SOFT, with a false claim;
- **M23 (Lemma B):** killed by G-PRESC-MEMBER, on availability.

R4's statement that no control isolates them is withdrawn. Each is kept because it is a hypothesis of a derivation step, so the guarantee is derived rather than conditional on the order in which tests reject. **Kept for the derivation, without a kill:** M17 (R6; the charge), M21 and M27 at the design's precisions. **Not behavioural:** M14 (derived: t₁ covers it; kept for ROOT's ruling) and M16 (availability only; SD-G5's unit test).

## 8. Where the design needs more than an S\* amendment

1. **The conditioned part: resolved, with every step certified (R5; R6).**
   - ROOT adopted the estimate, its contribution-level residual (at `e5f4ef3a8`), and, at `c3f2cfc72`, q_W = 3p + 64 and V4's runtime charge.
   - R4 bounds the second-order remainder under θ ≤ 1/2 and measures it (t₂, t₃).
   - R5 bounds the inverse norm by Uc from the verification's own factor (Lemma D). R4's one uncertified step, ‖K̃⁻¹‖₁ ≤ F·est, is gone.
   - R6 adds the shifted-factorization bound, which is polynomially loose where Uc is exponentially loose (Lemma E), and takes the smaller of the two per block.
2. **The residual gate: resolved** (§5.6), with refinement preserved, the hybrid form confirmed and the best state tested.
3. **The published value carries the 2^-22 relative gap** (2^-21 at p = 512, §5.2). This is pre-existing and stated. The bound itself is no longer first-order.
4. **A per-body schedule.** An F-3 body still sends its whole case to 1024. **Not proposed.** The bound, θ and the g check are per block since R6 (V4-V2, V4-V5), so one body's looseness no longer escalates another's charge.
5. **Route 1's constant under the bounded majorant:** for K4's RETURN (V4-R8; nothing missing in the design).
6. **W1b's formed ledger terms (F3):** residual at q_W, N_u's term max_i s_i·(c_f/63.5)·T_i, and the recovery side, E and Lemma B(iii)'s count (§6.5, corrected in R5). W1a needs nothing: its ledger is exact.

## 9. What I could not resolve

- **Does any uncertified step remain? No** (R5, R6; derived).
  - Every step of the guarantee is derived: Lemmas A to E, the Theorem and the Corollary (§4.1.6.3).
  - The inverse norm is bounded per block by B_c = min(Uc_c, S_c). Both are computed with directed rounding, from the verification's own factor and, where needed, shifted factorizations of the same matrix, whose backward errors Lemmas D and E charge.
  - est remains in the condition screen and in the choice of σ_c, for availability (§5.9). A miss there costs no honesty: the checks it could fool use B, and a poor σ_c only makes the shift fail.
- **What the guarantee still rests on, other than derivation.**
  - **Lemma B takes V4's count** as recounted stage by stage against K4's Rust. Its exact γ-form is derived from the count's structure. DS1 re-checked the axis and Gram–Schmidt stages, not every chain.
  - **Lemmas D and E take K4's factor loop as read** (`retained/factor.rs` at `cef218a10`). A change to that loop's operation order needs γ_m re-derived; m = 2n + 2 leaves room for any order with at most n terms per entry. The shifted factorization needs K4 to run the same loop with its pivot test replaced by d′_i > 0.
  - **Rounding directions.** K4 must round Uc's and S's operations as §4.1.6.3 item 7 says (upward, with the two subtractions downward) and form θ and the norms upward. The emulator does. K4 already rounds its binary64 summaries upward; its factor uses the wide context to nearest, so the directed wide operations may be new code (nearest plus one ulp in the required direction is allowed). E-UC and SD-G5 test them.
- **Emulation only.** λ, the estimate, Uc, the shift, the charge, θ, W⁺, the gate and G5a are checked against DS1's emulator and V4's, not against K4's Rust. K4's E-UNIT, E-HEADROOM, E-ESTIMATE, E-CHARGE, E-UC, LEVER2, TILT-LEVER, CHARGE-SLENDER, SEEDED-SOFT, THETA-STUB-COUPLED, the RF-LARGE lane and the hybrid-gate probe controls do that.
- **RF-LARGE, stated plainly (§6.1).**
  - At 10 and 100 members, in K4's order, R6 selects every case at 128, as today's rule and R4 do. All are honest, with the same classes. R5 had moved five of the six 100-member cases to 256 or 512.
  - The shift bound is within 2^3.3 to 2^4 of the exact norm at 10 members, and within 2^5.5 to 2^5.6 of the norm computed at 512 bits at 100 members. Where the shift ran, Uc was 2^7 to 2^191 above the norm, or did not exist.
  - **At 1,000 members** the full schedule ran for all four rules. R6 selects all six at 128, honest, with today's classes; R5 leaves five of the six Unresolved with `uc` at every precision. The exact norm was not computed at 6,000 DOFs: S is 2^7.1 to 2^7.3 above est, a lower bound on the norm, and σ′ lies 2^1.5 to 2^2.1 below an inverse-iteration estimate of λ_min, which is consistent with Lemma E but certifies nothing.
- **The mutants that are not killed with a false claim.**
  - **M17 is no longer killed** (V4-V2). With per-block bounds, HH-SLENDER-m40's hidden mode no longer reaches the cantilever's charge, and DS1 built no single-block case that the charge alone refuses (§7). **It is kept for the derivation**, as M21 is: the charge is t₁ to t₃ of the Theorem, and B is its norm.
  - **M24 is still killed, but only on availability and only through the estimate's globality.** HH-SLENDER-m40 goes to 256 under M24, because the global est carries H's mode into the cantilever's charge: the opposite direction from R5's kill (§7). No honesty kill was built; Lemmas D and E, not a kill, are why the certified bound is required.
  - **M20 and M23 are killed on availability only** (THETA-STUB-COUPLED, G-PRESC-MEMBER). No honesty kill was built.
  - **M21 is not killed.** It is vacuous for every case the W1a facade publishes: the facade refuses nonzero support motion until W1b, and W1a's restraints are rigid (V4-V3: DESIGN §4.2 admits nonzero prescribed motion in the kernel, and §7's kernel controls use it). No charge covers the term it would drop (§7). It is kept because the Theorem's step 2 needs exact prescriptions.
  - **M27 is killed only at low precision** (`stress6`). At the design's precisions the backward-error term is far below σ_c, so no selection or honesty kill exists there (derived, §7).
  - **M25, M26, M28 and M29 are killed on availability.** Each restores an earlier scope or bound (R4's, R5's, or Uc alone), which is certified but looser.
  - **M14 and M16 are not behavioural.** M14's assembled form is covered by t₁ (constant 63.7 < 64; V4-U6), and the contribution-level form stays for ROOT's ruling. M16 affects availability only (V4-T3; ROOT and V4 accept it).
- **θ's remaining conservatism (V4-V5).** R6 tests θ per structural block with data. It does not narrow further, to the numerically decoupled parts of a block: a K_P entry that rounds to zero need not be zero in K\* (TILT-LEVER's lost tail), so K_P's numerical pattern does not give K\*'s blocks, and Lemma C must hold for the structural block. THETA-STUB-COUPLED's stub is such a numerically decoupled, zero-state part of its block. With the isotropic bounded operator, θ can bind before the charge only where the rows with a large ‖SĀS‖ carry no state (argued, not derived).
- **The emulation of today's gate differed from Rust on one probe case** (the one-member in-plane load at 256; V4-R3). In K4's order it now agrees (`gate_probe7`). In K4's order the hybrid gate's best state also moves in five single-mode probe cases, with the same outcome.
- **W1b is outside the Theorem** until F3 meets §6.5's three obligations.
- **G5a item 4's unit conversion** is derived but not exercised: the emulator's rows are in SI units.
- **The R1 families listed in §6.1 as not measured.**
- **T1's fixtures.** Whether its support-motion fixtures hold rigidly moved, unloaded bodies (condition 3's R5-5 movements at F-3) was not surveyed.

## 10. NOTEs declined

None, in any of the five checks. V4's NOTEs at R5 (V4-V3 to V7) are each resolved (§15). V4's NOTEs at R4 (V4-U5 to U9) were resolved in R5 (§14). V4-R8 needs nothing in the design. V4-T3 to T5 are recorded as V4 states them (§13). R2's N7 is K4's fix (`5ad1b6174`, K4-M33), cited, not specified here.

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

V4's delta check at R3 (`V4_VERIFICATION.md` "Delta check at R3", file sha256 `ed2031c3…`, numerics `c3f2cfc72`; NOT VERIFIED: 1 BLOCKING, 4 NOTEs) and ROOT's rulings on it (`ROOT_RULINGS_V1.md`, section sha256 `9e53317f…`). These are R4's resolutions, kept as history. **R5 supersedes their F parts** (F pinned, "the one uncertified step is F", M19): see §14.

**Count corrected (RV16-D2).** R4's header said "5 NOTEs" for this check, following V4's verdict line. V4's findings table lists four, V4-T2 to T5, and V4-T1 is the BLOCKING finding.

| Finding | Resolution | Where |
|---|---|---|
| **V4-T1** (BLOCKING) e_q is only conjectured | **V4's closure adopted, both parts, as ROOT ruled, and made rigorous.**<br>• **3p + 64 specified:** W's contributions are formed at q_W = min(3p + 64, 1024); the extra formation pass is charged.<br>• **The runtime charge:** C_q = ‖ā_q S‖₁·(t₁ + t₂ + t₃) ≤ 60·2^-2p·ê at p = 128 and 256, and ≤ 2^-22·2^-64·M_q at 512. t₁ is V4's term 62.5·2^-q_W·F·est·‖a_q S‖₁·‖SĀ\|u\|‖_∞ with the Neumann factor 2 and the γ-margins. est is the verification's Hager–Higham estimate, S K4's radix equilibration, Ā E's bounded operator (from the pass that forms E's reaction rows, extended to the free rows) and ā_q the bounded recovery row. The work is charged.<br>• **F pinned:** F = 2^16, bits `0x40F0000000000000`, in §4.1.6.3 item 8 and §4.1.3 (§5.9). It is the allowance in ‖K̃⁻¹‖₁ ≤ F·est, which the condition screen and W's accuracy already used unnamed.<br>• **On failure,** p is not selected, and the next p is tried (reason `charge`).<br>• **"The rcond screen … does not bound Γ_q" is replaced** by V4's derivation (Lemma A with Lemma C), cited.<br>• **Beyond first order (ROOT's rigour requirement):**<br>&nbsp;&nbsp;– the second-order remainder is bounded under a θ ≤ 1/2 check (‖K̃_P⁻¹E‖ ≤ 1/2, factor 2; Lemma C);<br>&nbsp;&nbsp;– the remainder is measured by r₂ (t₃) and by the recovery of δ̂ (t₂);<br>&nbsp;&nbsp;– the counts enter in their exact γ-form under a g check (Lemma B);<br>&nbsp;&nbsp;– prescribed values enter W's residual exactly;<br>&nbsp;&nbsp;– translation and rotation rows carry their measured bound W⁺ in the stop rule.<br>No step is left argued; the one uncertified step is F.<br>• **Emulation (emu4):**<br>&nbsp;&nbsp;– LEVER2 (three gains) and TILT-LEVER (three gains) are refused;<br>&nbsp;&nbsp;– the 44 R3 controls and the 20 probe cases keep R3's precisions and classes, with worst charge margins of 4.7e-29 and 9.0e-45 of the allowance;<br>&nbsp;&nbsp;– M17 (no charge) and M19 (F = 1) are killed by CHARGE-SLENDER, a control the charge alone refuses;<br>&nbsp;&nbsp;– M18 (q = 2p + 64) is detected by seven controls;<br>&nbsp;&nbsp;– the sweep and the R1 lane are compared with R3 in §6.1 and §1. | §5.5, §5.1, §5.2, §5.7, §5.8, §5.9, §3.3, §3.4, §4, §6, §7, §8, §9 |
| **V4-T2** (NOTE) V4's R2 statement that only assembly-level sums can saturate is false | R3's citation of it is dropped. **V4's erratum recorded:** TILT-LEVER's member is formed exactly at every P ≤ 576, yet its (uy,uy) entry carries an axial (t/L)² tail lost within the entry. DS1's own example (components differing by more than 2^(p+32)) stands | §5.5 |
| **V4-T3** (NOTE) The gate's choice affects availability, never honesty; M16's survival is acceptable | Recorded as V4 states it. ROOT accepts M16's survival | §7, §9 |
| **V4-T4** (NOTE) In V4's emulator the best state is #2 of 4 in seven single-mode probe cases and #1 in one; all 20 are selected at 128 either way | Recorded as V4 states it: an emulation detail. DS1's emulators choose the second, third or fourth state; the outcome is the same | §5.6 |
| **V4-T5** (NOTE) TILT-LEVER demonstrates the within-entry case, which the stop rule refuses at the leak site | Recorded as V4 states it. R3's "Argued" paragraph is superseded: under R4 such a case is also bounded by the charge, whatever the stop rule sees | §5.5, §7 |
| DS1's own additions in R4 | Made while closing V4-T1; each needed by a step of the Theorem:<br>• exact prescribed values in W's residual (step 2);<br>• the r₂ residual and t₂ (steps 3 and 4);<br>• θ (Lemma C);<br>• the g check (Lemma B);<br>• per-body norms (ALL-ZERO-BODY showed a global norm would charge an all-zero body);<br>• W⁺ for translation and rotation rows (R3 §3.4 was argued).<br>Each is flagged as R4's in the text | §5.5, §3.4, §7 |

## 14. Changes from R4: V4's delta check at R4 and its resolutions

V4's delta check at R4 (`V4_VERIFICATION.md` "Delta check at R4", file sha256 `7f405328…`, numerics `8831325d0`; NOT VERIFIED: 0 BLOCKING, 4 SHOULD-FIX, 5 NOTEs) and ROOT's rulings on it (`ROOT_RULINGS_V1.md`, section sha256 `02948a14…`). These are R5's resolutions, kept as history. **R6 supersedes** Uc's global form, the per-body scope of θ and the g check, and M17's and M24's kills (§15).

| Finding | Resolution | Where |
|---|---|---|
| **V4-U1** (SHOULD-FIX) Within both screens, est misses ‖K̃_P⁻¹‖₁ without bound, so no pinned F is a bound | **Uc adopted, as ROOT ruled, and F withdrawn.**<br>• **Uc** = U/(1 − U·γ_m·‖\|L\|D\|Lᵀ\|‖₁), with U = ‖M(L)⁻ᵀD⁻¹M(L)⁻¹e‖_∞, from the verification's own factor. It replaces F·est in t₁, t₃, N_u, θ, W⁺ and so in W's own accuracy (item 7).<br>• **Directed rounding:** every operation on nonnegative data upward, the denominator downward. Nearest plus one ulp in the required direction is allowed. A `uc` rejection when t ≥ 1.<br>• **Work:** one substitution pair and one pass over L's profile, charged to the verification.<br>• **Lemma D derives Uc ≥ ‖K̃_P⁻¹‖₁:** factor backward error (Higham's Lemma 8.4, in the manner of Theorem 10.3, with m = 2n + 2), \|L⁻¹\| ≤ M(L)⁻¹, then a Neumann step. It cites V4's derivation. DS1 checked each step and read K4's `factor.rs` loop for step 1.<br>• **est** is kept only in the condition screen, for availability (§5.9). F is removed from every step; no availability heuristic uses it, so it is withdrawn entirely, with its receipt bits and constant.<br>• **§5.5, §5.9 and §9 rewritten:** the miss is unbounded within the screens (HH-FOOL-m, F2), and the alternating vector is not a safeguard. §9 states that no uncertified step remains.<br>• **Measured:** Uc ≥ exact on 245 states, with no failure. U/exact ≤ 96.4; 1.0 on every HH-FOOL frame; within 10^-9 of 1 on F2, where U alone falls below exact once, so the backward-error term is needed.<br>• **Selections:** CHARGE-SLENDER 256 → 128, since Uc = est = the norm there. Nothing else changes on R4's controls, probes, sweep or R1 lane.<br>• **Mutants:** M24 (est in place of Uc) is killed by DS1's HH-SLENDER-m40, on availability. It is not killed by HH-FOOL-m-LOADED, whose charge is 6.3e-46 of its allowance (§7). M26 (R4's F·est) is killed by CHARGE-SLENDER. M19 is retired | §1, §4, §5.1, §5.2, §5.5, §5.7–§5.9, §6, §7, §8, §9 |
| **V4-U2** (SHOULD-FIX) θ and the g check are over-broad | **Scoped as V4 proposed.**<br>• θ_b is tested per body, for bodies that carry data: a nonzero ledger term, prescription or state. A body without data has u = 0 exactly and needs no Lemma C (derived).<br>• The g check applies to members with a free DOF, or a nonzero prescribed DOF, in such a body. Other members multiply only zeros and need no Lemma B (derived).<br>• **V4's two controls return to 128** (measured). M25 restores R4's unscoped form and is killed by both.<br>• **M20 and M23 are still killed, on availability,** by DS1's THETA-STUB (θ = 48 in a loaded body) and G-PRESC-MEMBER (g = 2^245 with a 2^-300 prescription). No honesty kill could be built: both are honest under their mutants | §5.1, §5.5 items 9–10, §6.1, §7 |
| **V4-U3** (SHOULD-FIX) W⁺ is load-bearing; §7 and §9 said no control isolates the guards | **SEEDED-SOFT added** to §7 and to K4's controls. It kills M22 with a false claim, ratio 954 (V4: 954), and M15 too.<br>• **M21:** no kill built. **Derived:** it is vacuous in W1a, since every W1a prescription is 0, and no other test implies the guard: the uncharged term would be 2^(p+58) times t₁. **Kept** for W1b, because step 2 needs it.<br>• **M14:** derived to be covered by t₁ (constant 63.7), so it is not a guard for honesty. Kept for ROOT's ruling at R2.<br>• §7's guard paragraph and §9 are corrected | §7, §9 |
| **V4-U4** (SHOULD-FIX) F3's obligation under-charges W1b's loads and omits the recovery side | **Restated and derived** (§6.5).<br>• N_u gains max_i s_i·(c_f/63.5)·T_i, the true size. R4's term carried a second 2^-q_W.<br>• **Recovery side:** E's stage gains (c_f/69)·T for every row a formed term enters (reactions, end actions, stations), with its own count in Lemma B(iii). This is the reverse of M6's net rule, for formed terms.<br>• c_f is stated by F3 | §6.5, §8, §9 |
| **V4-U5** (NOTE) Lemma C's inequality fails at p = 512 | One sentence added: at q_W = P, K^c − K_P is the single assembly rounding, so the conclusion holds | §5.5 Lemma C |
| **V4-U6** (NOTE) M14's slack: 2·64.5 > 2^7 | Corrected: (62.5 + 1)(1 + 2^-8.8) < 63.7, and 2·63.7·(1 + 2^-(P−8)) < 128. V4's 64 with 1 + 2^-7 also holds | §7 M14, §9 |
| **V4-U7** (NOTE) M10's "second order" reason is wrong | Replaced. Non-orthogonality is first order (0.028·g·2^-P), and the rigid-rotation leak, 0.007·2^-P·Ā\|u\|, lies inside V only because E carries g. That is a second reason to keep g. On M10-ANISO the leak is about 29 units, under V's 256 | §7 M10 |
| **V4-U8** (NOTE) θ has no rounding direction; step 5 needs the ∞-norm | (a) θ_b and every norm are rounded upward (items 6 and 9). (b) ‖SĀS‖ is the larger of its 1- and ∞-norms; step 5 cites it | §5.5 items 6, 9; Theorem step 5 |
| **V4-U9** (NOTE) "This closure adds no new reliance" is wrong | Withdrawn. V4 is right that R4's reliance was new for the guarantee. R5 removes the reliance rather than rewording it: est only screens | §5.9, §9 |
| **RV16-D2** (ROOT) R4's header counted 5 NOTEs in V4's R3 check | Corrected to 4 (V4-T2 to T5) in §13. V4's verdict line at R3 also says 5, but its table lists four | §13 |
| **ROOT's emulation requests** | • Uc on all 195 `est_check4` states, the HH-FOOL-m frames (m = 40, 80, 100, 200) and V4's F2 family: Uc ≥ exact everywhere; the U/exact and Uc/est ratios are in §5.5 and `uc_check5`.<br>• Controls and probes: three selections change from R4 (CHARGE-SLENDER, THETA-ZERO-BODY, G-FIXED-MEMBER), each explained. The probe set is unchanged.<br>• M24 added.<br>• The sweep and R1 lane were re-run: identical to R4 frame by frame and case by case, with smaller charges | §1, §5.5, §6.1, §7 |
| DS1's own changes in R5 | • Lemma B's γ-factor is tightened to 1 + 2^-8.8 under the g check (R4's 1 + 2^-7 is still true; V4 measured 1 + 2^-9.5).<br>• Lemma D's step 1 was checked against K4's `factor.rs`, and the permutation invariance stated.<br>• M18's kill list loses B1-L and LEDGER-AT-RESTRAINT, whose R4 kill came from F's 2^16 (derived).<br>• M3 and M7 gain kills on the moved or new controls (§7).<br>• E-UC replaces E-EST; `uc` joins the rejection reasons | §5.5, §7 |

## 15. Changes from R5: V4's delta check at R5 and its resolutions

V4's delta check at R5 (`V4_VERIFICATION.md` "Delta check at R5", file sha256 `fd381e3b…`, numerics `6ca7a4338`; NOT VERIFIED: 1 BLOCKING on availability, 1 SHOULD-FIX, 5 NOTEs) and ROOT's rulings on it (`ROOT_RULINGS_V1.md`, section sha256 `6ef6c730…`).

| Finding | Resolution | Where |
|---|---|---|
| **V4-V1** (BLOCKING, availability) Uc's comparison matrix grows geometrically along a frame's elimination: on R1's RF-LARGE frames in K4's order, five of six leave 128 at 100 members, and Uc never exists at 1,000 | **V4's shifted-factorization bound adopted, as ROOT ruled; B = min(Uc, S).**<br>• **The bound (7c):** factor K̃_P − σI with the same loop, pivots required positive; σ′ = σ − δ − γ_m·N′_L, downward; S = ⌈√n⌉/σ′, upward.<br>• **Lemma E** derives S ≥ ‖K̃⁻¹‖₁ (Weyl as a Rayleigh quotient, Lemma D's step 1 applied to the shifted input, ‖Y‖₂ ≤ ‖Y‖₁ for symmetric nonnegative Y). It cites V4's derivation; DS1 checked each step. Its looseness is about 2√n, polynomial.<br>• **σ from the estimate** (est_c, from the screen's own solves), an availability use, stated as such. A failed shift halves σ, at most three factorizations, then Uc stands alone.<br>• **Work:** the extra factorizations run only for a data-carrying block whose Uc_c is missing or above 2⌈√n_c⌉·est_c.<br>• **RF-LARGE lane in K4's order** (§6.1): at 10 and 100 members R6 selects all twelve at 128, honest (R5: five of six moved at 100). S is within 2^3.3 to 2^5.6 of the norm. At 1,000 members R6 selects all six at 128, honest, where R5 leaves five Unresolved; S ≤ 2^7.3 times the norm (against est).<br>• **Mutants:** M27 (no backward-error term) killed at low precision; M28 (no shift) killed by RF-LARGE-100; M29 (R5's form) likewise | §1, §4, §5.1, §5.2, §5.5 items 7, 8, 15 and Lemmas D, E, §5.9, §6.1, §6.2, §6.7, §7, §8, §9 |
| **V4-V2** (SHOULD-FIX) Uc is global while θ is per body | **Per block** (finer than ROOT's per body): Uc_c, S_c and B_c from the same passes (the maxima per block of the free–free pattern); `uc` tested only for blocks with data; a body's charge uses the largest B_c over its data blocks.<br>• HH-SLENDER-m40 returns to 128.<br>• **M17 loses its kill** and is **kept for the derivation**, as M21 is; DS1 built no single-block case the charge alone refuses (§7).<br>• **M24 keeps a kill, on availability, in the other direction:** HH-SLENDER-m40 goes to 256 under M24, because the global est carries H's mode into the cantilever's charge | §5.5 items 7 and 11, §7, §8, §9 |
| **V4-V3** (NOTE) M21's "vacuous in W1a" overstates; "no other test implies it" overstates | Restated: vacuous for every case the W1a facade publishes (DESIGN §4.2 admits kernel support motion); "no charge covers the term" replaces "no other test implies it" | §7, §9 |
| **V4-V4** (NOTE) the 1 + 2^-9.5 factor is V4's derivation | "Derived" | §3.3, Lemma B |
| **V4-V5** (NOTE) THETA-STUB's θ comes from a decoupled zero-state block | **θ narrowed to blocks with data.** THETA-STUB returns to 128. **A coupled variant, THETA-STUB-COUPLED, kills M20** on availability: the stub is in the loaded structural block, where Lemma C is required. **Not narrowed further**, and why: a K_P entry that rounds to zero need not be zero in K\*, so numerical decoupling does not give K\*'s blocks (§9) | §5.5 items 7a, 9 and 10, §7, §9 |
| **V4-V6** (NOTE) Uc of 2^1024 or more is not encodable | Rule: `receipt_encoding`, as for E. Derived unreachable at a selected p: θ_c ≤ 1/2 with ‖SĀS‖_c ≥ 1/4 gives B_c ≤ 2^(P−6) ≤ 2^1018 | §5.5 item 14, §5.8, §6.3 |
| **V4-V7** (NOTE) V4's R3 verdict line said 5 NOTEs | Recorded. R5's §13 already carries the corrected count (RV16-D2) | §13 |
| **ROOT's emulation requests** | • emu6 factors in K4's order (DS1's port of `factor.rs`'s RCM, identical to V4's port on 97 models).<br>• The shift bound and min(Uc, S) ≥ exact on all 267 blocks of the 234 model states, Lemma E in 19,979 random low-precision factors (S ≥ exact in all 1,457 where S exists, Uc in all 14,198) and on F2 at P = 256 and 512 (28 shift bounds), and on RF-LARGE (exact at 10 members; the norm computed at 512 bits at 100; at 1,000, only σ′ below an inverse-iteration estimate of λ_min, since the exact norm was not computed).<br>• Controls, probes and RF-LARGE under today's rule, R4, R5 and R6; the sweep and the R1 lane under today's rule, R5 and R6, against R5's and R4's records (§6.1, §7).<br>• M27, "the shift bound without its backward-error term": killed in the stress, where ⌈√n⌉/σ fell below the exact norm in 27 factors (P = 10, 12 and 16) while S never did; not killed on any control (derived) | §5.5, §6.1, §7, §16 |
| DS1's own changes in R6 | • The emulator's row loops are sparse, and it reproduces R5 exactly on all 58 controls in emu5's order (`reproduce6`).<br>• σ_c comes from per-block ratios of the screen's own solves (est_c), so one block's softness does not loosen another's shift bound (with the global est, V's block in HH-FOOL-m200 had S/exact of 1.4e56; with est_c, S/exact is at most 14 on every block).<br>• In K4's order the HH-FOOL frames fool est by only about 2^5.6 (emu5's natural order: 7.7e8 to 1.1e57) | §5.5, §16 |

## 16. Evidence (scratch, standard-library Python 3.13)

The evidence of R2 to R5 is unchanged. ROOT committed it at `_run_records_r2/` to `_run_records_r5/`. R6's is new. Every Python experiment of DS1's ran one process at a time under `nice -n 19`, in K4's elimination order unless stated; other sessions' processes ran on the host meanwhile.

**One defect found and fixed during R6.** The first R1 lane run stopped with an error: inside `solve_at`, the directional-block loop reused the name of the block map, so the shift failed on a model with directional springs. The loop variable was renamed. Every output below was then regenerated on the final `emu6.py` by `chain6.sh`. The runs made before the fix (controls, probe, stress, mutants, RF-LARGE at 10 and 100 members) reproduce exactly, apart from timings; the bound check was rerun with THETA-STUB-COUPLED added. None of their models has a directional spring on which the shift ran, so the defect could only stop a run, never change a result.

| File | What it is | sha256 (first 16): script, output |
|---|---|---|
| `emu6.py` | R6's emulator: emu5 plus K4's order, the blocks and their data flags, per-block Uc_c, est_c, the shifted-factorization bound S_c and B_c, the block scope of θ and the g check, and sparse row loops (switches `ORDER`, `BOUND`, `SCOPE`, `SHIFT_BE`, `SHIFT_TRIES`, `SHIFT_ALWAYS`; option `r6`) | `767c9381822dd61b` |
| `k4order.py` | DS1's port of `factor.rs`'s `reverse_cuthill_mckee` and `order_free`, and the structural pattern | `e80ea80987841640` |
| `bounds6.py` | JSON summaries of the bounds (log₂ values) | `afd5f9fd3f42f706` |
| `models6.py` | models5's controls plus THETA-STUB-COUPLED | `ed77a19bae9f457c` |
| `reproduce6.py` → `reproduce6.stdout.json` | emu6 in emu5's order with R5's and R4's switches against `run_controls5`: 0 of 58 differ | `2265c10120637b4f`, `879ac630fd37cb38` |
| `k4order_check6.py` → `k4order_check6.stdout.json` | DS1's ordering port against V4's: identical on 97 models | `b10dc75e779328af`, `d19aafc68e07988a` |
| `run_controls6.py` → `run_controls6.stdout.json` | 59 controls × {today, R4, R5, R6}, with R6's charge, θ and bound summary at every verification | `7c4170342fc40029`, `1489ce00094efdcd` |
| `uc_check6.py` → `uc_check6.stdout.json` | Uc_c, S_c and B_c against the exact per-block norm: 234 model states, 267 blocks | `78667ef945729b65`, `8bd60514dcc66fcf` |
| `stress6.py` → `stress6.stdout.json` | Lemma E and M27 at low precision (19,979 factors), and F2 at 256 and 512 | `a30973e8c585772a`, `a80a1244f64b5487` |
| `mutants6.py` → `mutants6.stdout.json` | M1–M18, M20–M29 on 40 controls | `26df4f70b3b6054c`, `79bfc3e5d0ff21c8` |
| `gate_probe7.py` → `gate_probe7.stdout.json` | K4's probe set under R6 | `2131f7ad8bb676df`, `ef410238f4de06cd` |
| `sweep6.py`, `summarize6.py` → `sweep6_{11,12}.json`, `sweep6_summary.json` | 2,000 frames × {today, R5, R6} | `fb4106acc52e4333`, `8cd640a53950e3a7`; `941f444c7169eeae`, `9d79d9c31eb61b3f`, `a028df523f5348a3` |
| `r1_lane6.py` → `r1_lane6.stdout.json` | K4's R1 lane (120 cases) × {today, R5, R6} | `a69d21592cc9072b`, `ebd27429a829a06a` |
| `large6.py` → `large6_10.json`, `large6_100.json`, `large6_1000.json` | R1's RF-LARGE at 10, 100 and 1,000 members × {today, R4, R5, R6}, with the looseness | `02020c0aa874976b`; `a3594d066c6442b3`, `8901005534a9eadf`, `2d0cff3eefe5be25` |
| `chain6.sh` | The sequence that produced the outputs above | `9c51975a35caf00d` |

The scripts import, read-only and unchanged:
- the earlier emulators `emu.py` to `emu5.py` (the model builders take their helpers from emu2 and emu4);
- `models.py` to `models5.py`, `sweep.py`, `gate_probe3_cases.py`, `lever3.py` and `r1_adapter.py`.

All of them are in `_run_records/` to `_run_records_r5/`. `k4order_check6` also reads V4's `_v4_records/r5/r5_large.py.txt` and executes its `rcm` function, unchanged.

**To rerun** (from `<wt>/scratch/ds1/`, one at a time, each under `nice -n 19`, with `PYTHONDONTWRITEBYTECODE=1`):
- `python3 reproduce6.py`;
- `python3 k4order_check6.py <T3 path> <T3 path>/DESIGN_NUMERICS/REV_5A3_CANDIDATE/_v4_records/r5/r5_large.py.txt`;
- `python3 run_controls6.py`;
- `python3 uc_check6.py`;
- `python3 stress6.py 20000 6`;
- `python3 mutants6.py <T3 path>`;
- `python3 gate_probe7.py`;
- `python3 sweep6.py 11 1000`, then `python3 sweep6.py 12 1000`, then `python3 summarize6.py sweep6_11.json sweep6_12.json > sweep6_summary.json`;
- `python3 r1_lane6.py <T3 path> lane`;
- `python3 large6.py <T3 path> 10 honesty`, likewise with 100 and with 1000.

`chain6.sh <T3 path>` runs all of them in sequence, on the final `emu6.py` (the sweep and `large6` at 10 and 100 members after the 1,000-member lane).

**Delegation.** DS1 ran as a Claude Code background subagent launched by ROOT through the Agent tool. This revision was made on ROOT's follow-up message. ROOT is the only return path. DS1 dispatched nothing.
