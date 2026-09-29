# D1 revision 5a.3 (proposed, revision 3): the stop rule's resolution term, verification estimate and ceiling floor

Design addendum by DS1, a Type 2 TASK (numerical design drafter), for ROOT (HELP_HUMAN), 2026-09-28. **This is revision 3 of the candidate.** It resolves V4's delta check at R2 (NOT VERIFIED: 1 BLOCKING, 1 SHOULD-FIX, 6 NOTEs) and applies ROOT's rulings on it.

- **The candidate and R2 stay unchanged:** the candidate `DESIGN_NUMERICS/REV_5A3_CANDIDATE/D1_REV_5A3_SSTAR_RESOLUTION.md` (sha256 `c265d7bb…`, numerics `a7015eea3`); R2 `D1_REV_5A3_SSTAR_RESOLUTION_R2.md` (sha256 `326a8d78…`, numerics `9e9e3056a`).
- **This file is a proposal.** V4 runs a delta check; ROOT selects only after VERIFIED.
- `DESIGN_NUMERICS/DESIGN.md` (revision 5a.2, sha256 `fb62ef4a…`) stays hash-pinned. The amended text is in §5, as addendum blocks.
- **§12 maps every finding of V4's delta check to its resolution** ("Changes from R2"). §11 keeps R2's map of V4's first verification.
- **What R3 changes, in one line each:** the estimate's residual is one exact sum over the contributions, on the final state (V4-R1, V4-R2); R2's claimed margin for the K^q term is withdrawn and the term restated (§5.5); the gate's bounded test applies to the best state (V4-R5); Lemma 2's proof, G5a item 4's units and V4's errata are corrected.

**Labels.** Every claim is marked as one of:
- **derived:** it follows step by step, and a verifier can check each step;
- **measured:** it was observed in DS1's standard-library emulation (§13). A measurement is evidence, not proof;
- **argued:** it is reasoned but not proved;
- **conjecture:** it is believed, and it is not established.

## 0. Basis, inputs and what was not done

**Read for this revision.** Hashes are sha256, first 8 hex digits. `<wt>` is the T3 worktree root.

| Input | Hash |
|---|---|
| V4's delta check at R2: `V4_VERIFICATION.md`, section "Delta check at R2", at numerics `e5f4ef3a8` | `c2f5539b` (file) |
| V4's delta records `_v4_records/delta_r2/` (read: `r2_lever2`, `r2_lever_fix`, `r2_probe_seed`, `v4emu_r2`) | — |
| `ROOT_RULINGS_V1.md`, section "D1 revision 5a.3: rulings on V4's delta check at R2", at `e5f4ef3a8` | `69c8f23b` (section) |
| V4's verification, `DESIGN_NUMERICS/REV_5A3_CANDIDATE/V4_VERIFICATION.md`, at numerics `3057fc22f` | `0222d0ec` |
| V4's records `_v4_records/` (read: `v4emu`, `cases`, `t_demo`, `small_checks`, `gate_probe_{nsec,loadonly}`) | — |
| `ROOT_RULINGS_V1.md`, section "D1 revision 5a.3: rulings on V4's verification", at `085638e58` | `9a36d688` |
| K4 at `5ad1b6174` (`codex/piping-k4-20260928`): the Rust probe `probe_v4_s3_y_reference_with_a_chord_component` (`adaptive_tests.rs`); RETURN §21; the combination's prescribed-row fix with control and mutant K4-M33 | — |
| K4 at `3ed6c0e26` (A2), for the functions below | — |

The candidate's §0 lists its own inputs, all still valid: D1 r5a.2 `fb62ef4a`; D2 r5b.2 `edc78f9c`; R1 references `7b176dbb`; and the rest.

**Where the stop rule, S\* and the gate live in K4 (A2 `3ed6c0e26`, `retained/adaptive.rs`):**
- `residual_rows` `:872`;
- `scales_at` `:1093–1136`;
- `stop_rule` `:1141`;
- `classify_rows` `:1430`.

Their bodies are byte-identical to A1's (the candidate's §0).

**Done.** Standard-library Python only, under `<wt>/scratch/ds1/`.
- `emu3.py` is R2's `emu2.py` with R3's three changes, each behind a switch whose default is R3: the estimate's residual at contribution level on the final state, the bounded test on the best state, and an evidence-only precision variant for the residual (§5.5).
- R3 re-ran, under emu3:
  - every control (44: R2's 40, EXACT-RIGID and V4's LEVER2 at three gains);
  - every mutant (M1–M16);
  - K4's Rust probe cases, rebuilt;
  - the 2,000-frame sweep;
  - the error-constant and estimate measurements;
  - K4's R1 lane (120 cases). R1's formation-scale ratios depend only on E, which R3 does not change, so R2's figures stand.
- The files are listed in §13.

**Not done.**
- No Git write, and no cargo or Rust run. Every K4 statement comes from reading its source and recorded outputs.
- No product code.
- No file written outside `<wt>/scratch/ds1/`, apart from temporary edit scripts and copies of outputs for the rerun comparisons, in the session's own temporary directory.

## 1. Summary

**The recommendation, in six lines.**
1. **Correct §4.1.9.** A tail below 2p's rounding unit is lost identically at p and at 2p ("saturated"; Lemma 2). A wrong value then passes by up to the verification's own first-order error.
2. **Define a per-body formation scale E** for the force and moment kinds (§4.1.6.2).
   - It is the Σ\|terms\| of every recovery expansion over a bounded operator, with each stage rounded once at 2p.
   - g is exact; E is coupled through L_b; it is published rounded upward.
   - V, Φ and G5b's item 6a all use one binary64 ê, bit for bit (S4).
3. **Change the stop rule.** Accept p only if \|q_p − q_2p\| + V ≤ 2^-64·max(\|q_2p\|, S\*), with V = 2^(8−2p)·ê, **and** the verification estimate satisfies W ≤ 2^(6−2p)·ê (§4.1.6.3; ROOT's S2 ruling, and its confirmation of V/4).
   - W's residual is **one exact sum over the contributions** (element entries and directional-block entries as formed at q, spring stiffnesses in binary64), **recomputed on the final state** (V4-R1, V4-R2; ROOT: adopted).
   - W then measures the 2p state's solve error to first order, every saturated assembly sum included: measured within a relative 2·10^-4 at P = 128, 5·10^-11 at 256 and 2·10^-15 at 512.
   - What it does not measure is the formation error of individual entries below q's resolution. R2's "2^72 margin" for it is withdrawn: it is bounded by the gain, not by a constant (§4.1.6.3).
4. **At p = 512 only,** floor the force and moment S\* at Φ = fl↑(2^-438·ê).
5. **Residual gate (§4.1.4; ROOT's S3 ruling, O12 reversed; the hybrid form confirmed).** The acceptance test uses the bounded operator at contribution level, \|f\| + ΣĀ\|u\|, on the best state evaluated (V4-R5). Refinement stays driven by M03's coalesced ratio, so every correction made today is kept. The gate's own residual stays over K^q's assembled entries; §5.6 derives why it needs no contribution-level form.
6. **Publish** `resolution_scale` (E per body) and the estimate's summary. G5a gains the shape check, the zero rule, the sanity bound and V4's published-data lower bound on ê. G5b applies item 6a at p = 512. G5c is unchanged.

**Evidence** (measured; the full tables are in §7 and §13):

| Set | Result under the revision | Result today |
|---|---|---|
| **2,000 random frames** | No false claim (the tightest claim ratio, 0.995, is the same frame and value as today); nothing unresolved; no relative row lost; 0 G5a failures; work +1.0 % (proxy, excluding the E pass and the estimate). W ≤ 0.0048·V, so the estimate never fired. Every frame's selected precision, classes and rejection reasons equal R2's | 1 false claim; 230 unresolved |
| **44 controls** (K4's survey, V4's DEMOTION2, ASSEMBLY-SAT, loaded cantilever, EXACT-RIGID and LEVER2 at gains 2^90, 2^100 and 2^110, DS1's F-2/F-3 set and the mutant controls) | Every selected case honest; the three LEVER2 cases are refused (Unresolved); max W/V 0.0021 on the rest; 0 G5a failures | 10 false claims (9 natural, including the three LEVER2; SEEDED-COMMON's is seeded). **R2 made 3:** the LEVER2 cases, claim ratios 1,005, 1.03e6 and 1.05e9 |
| **K4's Rust probe** (20 cases), **rebuilt in the emulator; emulation only** (V4-R3) | All 20 selected at 128, honest, 0 G5a failures | The 8 single-mode y_ref (3,4,5) cases are refused by the coalesced gate, in the emulator as in Rust. K4's Rust control for the hybrid gate is not yet built |
| **K4's R1 lane** (120 cases) | Identical selected precision and classes, today, R2 and R3 | — |

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

### 2.5 Instances (measured, `run_controls3.stdout.json`)

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

**Validity (first order):** 68g·2^-P ≪ 1. It holds for g < 2^(P−16). A member with g ≥ 2^(P−16) has an e_y formed mostly of noise at P: it fails the stop rule at p and is resolved at a higher p, or it is unresolved (argued).

**Measured:** max\|q_P − R\*(u_P)\|/(2^-P·E_q) ≤ 2.33, where R\* is the recovery with P = 2048 operators (`measure3_3.json`, LEVER2 included; V4: 0.69).

**The element's rigid-mode leakage** is the K_e row with u a rigid motion: ≤ (20g + 42.5)·2^-P·(B̄ᵀ\|D\|B̄\|r\|). So F-3's leakage is inside λ.

### 3.4 Translation and rotation (argued, as in r5a.2)

For these kinds E would be \|u\| ≤ S\*, so V ≤ 2^(8−2p)·S\* is negligible. Their propagated tails are bounded through the rcond screen by F·2^-(p+1)·\|u\|, where F is Hager–Higham's uncertified factor. V4 found no false displacement row in 400 full-schedule runs, and the 2,000-frame sweep found none either (measured).

## 4. The options, and the choice

The candidate's §4 derivations of (i-a), (i-b), (i-g), (ii) and (iii) stand. Their sweep figures come from the candidate's emulator, over 1,966 frames. Two corrections:

**S1: (iii)'s demotion claim, restated (derived; V4-S1).**
- At p = 512 a floored kind has S\* ≥ Φ = 2^-438·ê, so its threshold t = R·S\* ≥ 2^-472·ê. **A row is demoted only if the case is selected at 512 with Φ above the kind's computed scale, and the row has \|q\| < 2^-472·ê(body, kind).**
- Reaching 512 through V requires some row with ê/M > 2^(2p−72), which is 2^440 at the 256 candidate, or another rejection at 256.
- **Measured:** V4's DEMOTION2 (ê ≈ 2^482; 1 N rows) is demoted exactly as V4 found: relative rows 7 → 4 (V4: b = 1.67e-6 N on the force rows; emu2: b ≤ 4.7e-6 over both kinds). In the 2,000-frame sweep no relative row was lost.

The candidate's "never demotes" is withdrawn. The contrast with (i-g) is the threshold: (i-g) floors a kind whose published scale is below 2^(10−p)·ê (2^-118·ê at p = 128), and demotes its rows below 2^(40−p)·ê.

**The two additions are orthogonal to the options.** The verification estimate (§4.1.6.3) and the gate's bounded acceptance (§4.1.4) would serve any of them. With them, (iii)'s honesty rests on a narrower conjecture than the candidate's:
- W measures the verification's solve error to first order at every verification state (P = 256, 512 and 1024), every saturated sum of contributions included (R3, V4-R1);
- one term remains outside it: e_q, the formation error of individual entries below q's resolution. It can exceed b only by itself, and by at most 2^-66·(Γ_q/ê)·b at p = 128 and 256 and 2^-516·(Γ_q/ê)·b at p = 512, where Γ_q is the entry-to-row gain (derived, §4.1.6.3);
- that it is negligible is a conjecture: nothing in the design bounds Γ_q by a constant.

That reduces the robustness argument the candidate gave for (i-g) (margin 2^66 against λ at 2p). It does not remove it. (i-g)'s larger floor would bound e_q's excess by 2^-(p+4)·(Γ_q/ê)·b (derived: its b ≥ 2^-64·2^(10−p)·ê), against (iii)'s 2^-66·(Γ_q/ê)·b. The 3p + 64 option of §4.1.6.3 gives (iii) 2^-(66+p)·(Γ_q/ê)·b without (i-g)'s demotions. **(iii) remains the recommendation:**
- it changes no correct outcome except by escalation, and by the demotion class above;
- it demotes only below 2^-472·ê;
- the estimate costs one exact residual pass over the contributions, one substitution pair and one recovery pass per verification.

## 5. The exact amended text (addendum blocks)

### 5.1 §4.1.6, stop rule: replace the "Accept p" bullet; add the floor bullet

> - **Accept p** (revision 5a.3) when both hold, decided exactly:
>   - (a) every published quantity q satisfies `|q_p − q_2p| + V_q ≤ 2^-64 · max(|q_2p|, S*)`, where **V_q = 2^(8−2p)·ê(body, kind)** for q of kind force or moment and V_q = 0 for translation and rotation;
>   - (b) the verification estimate passes, `W_q ≤ 2^(6−2p)·ê(body, kind)`, for every force and moment row (§4.1.6.3).
>
>   ê is §4.1.6.1 item 6a's binary64 value, formed from the E bits the receipt publishes and lifted exactly. V_q, Φ and the estimate's threshold use that same ê, bit for bit.
> - **The ceiling floor (revision 5a.3).** At the last candidate, p = 512 (verified at 1024), `S*(kind) := max(S*(kind), Φ)`, with `Φ = fl↑(2^-438 · ê(body, kind))`, for force and moment. It applies in the stop rule and in the classification: item 6a, the same Φ bits. At p = 128 and 256 there is no floor.
> - **Why (revision 5a.3).**
>   - A rounding whose tail lies below both p's and 2p's rounding units is identical in both candidates (§4.1.9). (a) charges the verification's resolution; (b) measures its solve error.
>   - Without V, F-2 passed labelled exact against a truth of 2^-300.
>   - Without the floor, an unloaded body moved rigidly (F-3) could not be accepted at any precision.

### 5.2 §4.1.6 "What acceptance guarantees": replace its first bullet (V4-N6)

> - **What acceptance bounds (revision 5a.3).** If p is accepted, then for every q, |q_p − q\*| ≤ |q_p − q_2p| + |q_2p − q\*|, and |q_2p − q\*| ≤ (recovery resolution) + (the solve error of the 2p state).
>   - The recovery resolution is ≤ 68g·2^-2p·E_q (§4.1.6.2 item 5; derived to first order).
>   - The solve error is measured by W_q to first order, apart from e_q, the formation error of individual entries below q's resolution (§4.1.6.3).
>   - The rule admits p only when W_q ≤ V_q/4 and |q_p − q_2p| + V_q ≤ 2^-64·M_q.
>   - So |q_p − q\*| ≤ 2^-64·M_q − 124·2^-2p·ê + |e_q|, derived to first order. b = fl↑(2^-64·S\*) bounds |q_p − q\*| for an `absolute_verified` row whenever |e_q| ≤ 124·2^-2p·ê, and b is exceeded by at most |e_q| otherwise. For a kind floored at p = 512, b = fl↑(2^-64·Φ) ≈ 2^-502·ê.
>   - That e_q is negligible is a conjecture, stated in §4.1.6.3 with the gain it would need.
> - **Scope of b.** b bounds the candidate's value q_p, not the published binary64 value. Two small differences follow; both were present in r5a.2 and are stated, not widened:
>   - publication adds |q_pub − q_p| ≤ 2^-53·|q_p|, which for an absolute row (|q_p| < 2^-34·S\*) is < 2^-87·S\* ≤ 2^-23·b (V4 has corrected its earlier 2^-24·b to 2^-23·b, V4-R7);
>   - the published S\*_pub (from binary64 rows) may lie below the stop rule's S\*_2p by a relative (2^-64 + 2^-52).
>
>   So |q_pub − q\*| ≤ b·(1 + 2^-22). D2's interval binding [q_pub ± b] inherits this relative 2^-22 and the first-order qualification.

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
> 5. **Resolution bound (derived to first order):** |q_P − R\*(u_P)| + |a_q|ᵀ|δu_rep| ≤ (20g + 48)·2^-P·E_q/g ≤ 68·2^-P·E_q, valid while 68g·2^-P ≪ 1. **λ = 2^8.**
> 6. **Role.** E serves the verification. When a 2p state becomes the next candidate, the comparison against 4p uses the E of the 4p verification (V4-N11).

### 5.5 §4.1.6.3 (new): the verification estimate (S2; ROOT: adopted; the residual revised in R3 for V4-R1 and V4-R2)

> At the verification state (precision P = 2p), after its residual gate has passed:
> 1. **Residual (revision 5a.3, R3).** Take the **final state**: the state the gate selected (§4.1.4 step 3), whose values the stop rule compares and the case would publish. For every free row i form
>
>    `r_i = f_i − Σ_e Σ_j k(e)_ij·u_j − Σ_s k(s)·u_i − Σ_d Σ_j k(d)_ij·u_j`
>
>    as **one exact expansion over the contributions**, rounded nowhere before it ends, where:
>    - f_i is the exact ledger net;
>    - k(e)_ij are element e's entries **as formed at q**, the same q-formed entries whose assembly gives K^q (q = P + 64; at P = 1024, q = 1024 and they are the state's own, Q4);
>    - k(s) is each global-axis spring's binary64 stiffness at row i;
>    - k(d)_ij are each directional block's entries as formed at q;
>    - u_j runs over every DOF, prescribed values included as rounded at P.
>
>    No contribution is summed into an assembled entry first, and nothing is reused from the gate: the residual is recomputed here, on the final state.
> 2. **Correction.** δ = K_P⁻¹·r̂, where r̂_i is r_i rounded once to P, with the state's own factor: one scaled substitution pair, `factor.solve`. δ = 0 at constrained DOFs.
> 3. **Estimate.** W_q = |L_q(δ)| for every published force or moment q. L_q is q's recovery at P applied to δ **with the ledger omitted**: end, station, spring and directional actions through their recovery; reactions Σ_j K_P,cj·δ_j; support-group components as sums, and magnitudes bounded by the sum of their components' W. Each is one exact expansion rounded once at P.
> 4. **Test.** W_q ≤ 2^(6−2p)·ê(body, kind) (= V_q/4) for every force and moment row, with item 6a's ê. It is decided exactly, and only after (a) of the stop rule has accepted.
>
>    *Why V/4 and not V (ROOT confirmed V/4 at `e5f4ef3a8`).* The test is set at V/4 so that the bound below closes: resolution ≤ 68·2^-2p·ê plus estimate ≤ 64·2^-2p·ê stays inside V = 256·2^-2p·ê, leaving 124 units. With the test at V, |q_2p − q\*| could reach (68 + 256)·2^-2p·ê > V, and acceptance would no longer imply the claim. No emulated case other than LEVER2 comes near either threshold: the largest W/V is 0.0048 (sweep), 0.0054 (R1 lane) and 0.0021 (controls).
> 5. **Failure.** The candidate p is rejected with reason `verification_estimate`, and the worst row and its ratio are recorded. The schedule escalates exactly as after a stop-rule rejection: the 2p state becomes the next candidate, verified at 4p with its own estimate. At the ceiling (p = 512, verified at 1024) the case is Unresolved (Ceiling), with the reason recorded.
> 6. **Evidence.** The worst W_q/V_q per body and kind, rounded upward to binary64, is recorded in the attempt and in the receipt beside the stop-rule summary. G5a checks that it decodes to ≤ 2^-2.
> 7. **Work (charged to the verification attempt).**
>    - Per free row, one exact expansion over its contributions: at most 12 terms from each element at the row's node, one per spring and three per directional block at that node, plus the ledger; then one rounding.
>    - One substitution pair and one linear recovery pass at P.
>    - The q-formed element and block entries are those formed for K^q. K4 either retains them (144 per member and 9 per block, each a q-bit value) or re-forms each element's entries as its rows are expanded; the bits are the same.
>    - The estimate is not reused when the state becomes a candidate.
>
> **What W estimates (derived to first order).** Let K^c be the exact, unrounded sum of the q-formed contributions, so that r = f − K^c·u_P exactly, and let ũ solve K^c·ũ = f with the prescribed values.
> - The factor solves with relative error ≲ κ·2^-P, and κ·2^-P ≤ F·2^-(p+1) ≪ 1, because the candidate passed its rcond screen (κ < F·2^(p−1)). This step is argued, not derived: F is Hager–Higham's uncertified factor, as in r5a.2.
> - So W_q = |a_q·(ũ − u_P)|·(1 + O(κ·2^-P)). This includes the saturated right-hand-side tails, u's own representation, and **every saturated sum of contributions, because assembly adds no error to K^c**: LEVER2's lost spring, ASSEMBLY-SAT's entry.
> - **Measured** (`measure3_3.json`, rows whose solve error exceeds 2^-(P+20)·ê): |W_q/(R\*(u_P) − q\*)| lies in 1 ± 2·10^-4 at P = 128 (121 states), 1 ± 5·10^-11 at 256 (143) and 1 ± 2·10^-15 at 512 (145).
> - **On LEVER2** the 512 state's estimate is exact to the lost spring (ratio 1). At 256 it rejects the candidate with W/V = 2,681, 2.7·10^6 and 2.8·10^9 for the three gains (`lever3_ratio.stdout.json`; V4 measured 1.07·10^4 × V/4 for the first).
>
> Hence |q_2p − q\*| ≤ 68·2^-2p·E_q + W_q·(1 + tiny) + |e_q| ≤ (68 + 64)·2^-2p·ê + |e_q|, where **e_q is the one term W does not measure.**
>
> **e_q = a_q·(u\* − ũ): the formation error of individual entries at q, propagated.**
> - *Derived (first order).* An element entry's formation error is ≤ (20g + 42.5)·2^-q times its bounded chain (V4's count), so ≤ 62.5·2^-q·Ā_e, because Ā_e carries g ≥ 1. A directional-block entry's is ≤ 4·2^-q·\|k_ab\| (four roundings). Springs have none. So **\|e_q\| ≤ 62.5·2^-q·Γ_q, with Γ_q = \|a_q\|·\|K\*⁻¹\|·Ā·\|u\|**, the entry-to-row gain.
> - *Derived.* From the stop rule and item 4, \|q_p − q\*\| ≤ 2^-64·M_q − 124·2^-2p·ê + \|e_q\|. So b holds whenever \|e_q\| ≤ 124·2^-2p·ê, and is exceeded by at most \|e_q\| otherwise. Relative to b:
>   - at p = 128 and 256 (q = 2p + 64, and b ≥ V because V ≤ 2^-64·M): the excess is ≤ 2^-66·(Γ_q/ê)·b, and b holds whenever Γ_q ≤ 2^64·ê;
>   - at p = 512 (q = 1024, and b ≥ 2^-64·Φ ≈ 2^-502·ê): the excess is ≤ 2^-516·(Γ_q/ê)·b.
> - **R2's claim is withdrawn.** R2 stated that K^q's formation error reaches forces at ≤ c·2^-(P+64)·ê with c ≤ 256, a margin of 2^72 against V. Nothing in the design bounds Γ_q by a constant times ê. LEVER2 reaches Γ_q ≈ 2^90·ê inside the rcond screen at p = 256. The rcond screen bounds a diagonally scaled condition number, which does not bound Γ_q. R2's p = 512 margin of 2^522/c is restated above as a bound on the excess.
> - *Derived.* Every saturated sum of contributions is now inside W, which closes the LEVER2 and ASSEMBLY-SAT class. What e_q can still carry is formation error within single entries.
> - *Argued.* Such error enters the stop rule's Δ at its p-level size unless the entry's value is the same at p, 2p and q. That is a tail below 2^-q within one entry, which needs near-degenerate inputs in one element or block, for example a member direction whose components differ by more than 2^(p+32). V4 argued that in an exactly representable structure only assembly-level sums can saturate: an axis-aligned element's entries are single-source terms.
> - *Measured, a caution.* A formed element does **not** annihilate rigid translations exactly in K4's formation as the emulator mirrors it: the lower triangle is copied from the upper, whose products round differently. 405 of 1,044 translation checks are nonzero at the rounding level (`checks3.stdout.json`). So no derivation may assume that formed elements are translation-exact.
> - **Conjecture.** No case combines a saturated within-entry tail with Γ_q > 2^64·ê (p = 128, 256) or Γ_q > 2^494·ê (p = 512, for an excess above 2^-22·b) and a published row whose truth lies more than b from its value. None is known. V4's 200 adversarial models and DS1's 2,000 frames never fired the estimate, and LEVER2 is refused.
> - **An option ROOT may add; it is not in this text.** Form W's contributions at q_W = min(3p + 64, 1024) instead of q.
>   - Effect (derived): the bound on e_q shrinks by 2^p, so the excess becomes ≤ 2^-(66+p)·(Γ_q/ê)·b.
>   - Cost: one formation pass at 3p + 64 per verification (448 bits at p = 128, 832 at p = 256; unchanged at 512).
>   - Measured: all 44 controls keep their selected precision, classes, honesty and W/V, and LEVER2 is refused as at q (`controls_wq3.stdout.json`). The sweep and the R1 lane were not run under it.
>   - No known case needs it. DS1's view is that it is worth adopting if K4's formation pass at 3p + 64 is small beside the factorization, because it narrows the conjectured window by 2^p.
>
> What W does not cover, besides e_q:
> - **the recovery resolution,** which is covered by the count;
> - **a combination's rounded prescription** (the residual is consistent with the rounded prescription), which is covered by V through E's \|u\|;
> - **common-mode loss of exact inputs,** which the exact sums remove.

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
3. **Honesty does not rest on the gate.** Acceptance of a published value rests on the stop rule, V and the verification estimate, which measures exactly what the gate tolerates. The gate is a convergence screen. Its looser acceptance can only move states to the stop rule, and at the verification the estimate then applies.
4. **Refinement stays driven by d^c.** Corrections toward K^q's solution are made where they are made today:
   - K4-M11's evidence-level control, TWO-SPAN, keeps its one correction (measured);
   - today's correction counts and golden work are unchanged wherever today's gate passes.

   The pure form (d^b driving refinement too, mutant M13) drops TWO-SPAN's correction (measured: 1 correction under the coalesced and hybrid gates, 0 under the pure form; `mutants3.stdout.json`) and would break O5's kill of K4-M11.

**Does the gate's own residual need the contribution-level form of §4.1.6.3? No (derived).**
1. **Nothing published is decided by the gate's residual.**
   - The stop rule reads the candidate's and the verification's recovered values.
   - The estimate forms its own residual, at contribution level, on the final state (§4.1.6.3 item 1).
   - The gate decides only whether an attempt proceeds and which evaluated state is final.
2. **Whatever state the gate passes, W measures what the gate's residual misses.** If K^q's assembled entries lose a contribution, refinement converges toward K^q's solution. Then r_c = f − K^c·u_P = (K^q − K^c)·u_P + (f − K^q·u_P), so the lost contribution enters W's residual and W measures its effect. On LEVER2 the gate passes the 256 state with its residual 0, and W rejects it with W/V = 2,681 (measured).
3. **Availability is unchanged except within 2^-71 of the gate's boundary.**
   - Each assembled entry of K^q is its contributions' exact sum rounded once at q, so \|r_q − r_c\| ≤ 2^-q·Σ_j \|K^c_ij\|·\|u_j\|, which is ≤ 2^-q·d_i for either denominator.
   - The gate's tolerance is 64·m_i·2^-P·d_i, with m_i ≥ 2 and q = P + 64.
   - So the assembled form moves the test by at most a relative 2^-71 of its tolerance.
   - A lost contribution, such as LEVER2's spring, is itself below 2^-q of its entry and is inside that bound.
4. **The assembled form is the cheaper one.** The gate forms up to four residuals per state, and the assembled form has one term per nonzero entry rather than one per contribution. Ā^q is only a denominator, so its assembly rounding (relative 2^-q) is immaterial.

**Effect on every control (measured in emulation: `run_controls3`, `gate_probe4`, `r1_lane3`, `sweep3`).** Every probe result below is **emulation only** (V4-R3): K4's Rust probe runs only the coalesced gate, and K4's Rust control for the hybrid gate is not yet built.
- **K4's Rust probe cases,** rebuilt in emu3: a (3,4,0) run of 1 or 3 N-section members, root fixed, five tip loads, y_ref (3,4,5) or (0,0,1).
  - Under the coalesced gate the emulator refuses the same 8 single-mode y_ref (3,4,5) cases as Rust (Unresolved), with gate ratios 1.7e15–2.2e16. The general load and every y_ref (0,0,1) case are selected at 128, as in Rust.
  - One difference: in the one-member in-plane case, Rust (RETURN §21) and V4's emulator fail the coalesced gate at 256, while emu2 and emu3 pass that state and then fail its 512 verification. The outcome (Unresolved) is the same; the difference is in the emulation of today's gate only.
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
>   - The ceiling floor bounds what 1024 cannot resolve (§4.1.6.1 item 6a).
>
>   The premise is derived to first order, apart from e_q: the formation error of individual entries below q's resolution, which can exceed b by at most itself, ≤ 2^-66·(Γ_q/ê)·b at p = 128 and 256 and ≤ 2^-516·(Γ_q/ê)·b at p = 512. That it is negligible is a conjecture (§4.1.6.3).
> - **Negative controls:** F-2, F-2 with S\* > 0, the prescribed tail, F-2 at the ceiling, ASSEMBLY-SAT, F-3 (three forms), DEMOTION2, SEEDED-COMMON and LEVER2 (three gains) join §7.3.
> - **In the factorization and triangular solves,** non-saturated error changes at 2p and the stop rule sees it. The refinement residual at p + 64 checks the solve against the exact-expansion right-hand side.
> - **The same exact-sum rule holds on the ordinary binary64 route** through S11-K and S11-F, as before.

### 5.8 §5 item 1 (the receipt): add to a `selected` entry

> - **`resolution_scale`** (revision 5a.3): per body, `force` and `moment` bit strings: E_fo and E_mo of §4.1.6.2, uncoupled, rounded upward, finite, +0.0 only when zero. Likewise in a combination's entry.
> - **The stop-rule summary** is the worst (|q_p − q_2p| + V_q)/M_q, rounded upward, and decodes to ≤ 2^-64.
> - **`verification_estimate`:** per body and kind, the worst W_q/V_q, rounded upward, decoding to ≤ 2^-2.

## 6. Effects downstream

### 6.1 The discriminating controls (§7.3, §4.10)

Measured with R3's rule (bounded-gate acceptance on the best state, the contribution-level estimate), against K4's rule and gate. Every selected case is honest.

| Control | Kills | Outcome |
|---|---|---|
| k = 1e-28 | D5, D6 | 128 fails the condition estimate; 256 — unchanged |
| Six-member k = 1e-12 | K4-M14; SD-G1 | 128 rejected on Rx; 256 — unchanged |
| B1-L | D13 | 128 — unchanged; the loads act at free DOFs, which do not enter E |
| S8-W (both) | D17, D20; SD-J2 | 128 — unchanged |
| PIVOT | — | 256 — unchanged |
| REACTIONS-ONLY | K4-M16; SD-G4 | 128 rejected by R(0, Uy); 256 — unchanged |
| TWO-SPAN | K4-M11 (O5) | 128 with one correction — unchanged |
| LEVER2 (gains 2^90, 2^100, 2^110; V4-R1) | M11, M14, M15 | **Unresolved (safe):** 128 fails the pivot test, 256 is rejected by the estimate, 512 by the stop rule (1024 resolves the spring). Today and R2 select 256 with false claims |
| N05, N06, SKEW-\*, AXIS, OBLIQUE, DUPLICATE, PRESCRIBED, ZERO-TORSION-345, ALL-ZERO-BODY, MIXED, SKEW-K1E-60 | — | Unchanged precision and classes |
| RF-CHAIN, RF-SKEW, RF-WEAK, RF-FINITE, RF-CANCEL (nodal), RF-MECH-LINE345-RX-COMPANION | Their negative controls | 120 of 120 unchanged precision (all 128) and classes, under R2 and R3 (`r1_lane3.stdout.json`); largest W/V 0.0054; 0 G5a failures |

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
| SD-G5 | **Unchanged for the Translation vectors** (V = 0; they pass E = 0). Add force-kind boundary vectors at \|Δ\| = ε·M − V (accepted) and one 2p-ulp above (rejected); an estimate boundary at W = V/4 and one ulp above; the gate's bounded test at its boundary, on a best state that is not the last; and an estimate residual whose assembled and contribution-level forms differ (LEVER2's shape) |
| SD-L1 | **Changed:** the E pass and the estimate (its residual pass over the contributions, one substitution pair and one recovery) are charged to the verification. The gate adds no correction where today's gate passes, so other counts are unchanged. Re-pin, with the E pass and the estimate as their own stages |
| SD-J1 | **Values unchanged** for vectors with p ≠ 512 or E = 0. The format gains (selected p, E_fo, E_mo). Add item-6a vectors: Φ binding and not binding, the fl↑ boundary, ê < 2^-584 |

### 6.3 D2 (r5b.2)

**Fields.** `resolution_scale` and `verification_estimate` in the receipt (§5.8).

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
   - **Measured:** 0 failures over 44 controls, the 20 probe cases, 2,000 sweep frames and 120 R1 cases. Mutants M5 and M7 fail it on several controls (`mutants3.stdout.json`). M2 fails the sanity bound (item 3) on TWO-SPAN. The emulator's rows are in SI units, so it does not exercise the conversion.
   - Item 4 cannot catch LEVER2's class, because E is correct there (V4); the estimate does.
   - It closes the under-report gap exactly where the floor binds. On RIGID-UNLOADED, LB = 0.4 against S\*_c ≈ 1e-132 (V4).
5. **Estimate summary:** each `verification_estimate` entry decodes to ≤ 2^-2.

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

**Order:** item 6 coupling → item 6a (p = 512 only) → item 7.

**Unchanged:** R = 2^-34, t = fl(R·S\*), the S\* < 2^-988 rule, and b.

### 6.5 F2a's contract

F2a (atomic with S-G1):
- maps K4's E and the estimate summary into the receipt;
- applies item 6a in the producer classification;
- keeps product-derived rows on the updated fo and mo;
- maps an unencodable E to `receipt_encoding`;
- lands the G5a and G5b changes, D2's parity files and the shared case files in the same PR.

**F2a's merge gate is unchanged.** The combination-as-solve rule needs nothing more.

**K4's own fix, outside this addendum.** K4 fixed the combination's prescribed-row double rounding (V4-N7) at `5ad1b6174`: the row is published from its exact sum, rounded once, with control and mutant K4-M33.

### 6.6 K4's ceiling argument (route 1, step 10; route 2; V4-N5)

**Restated:**

> The stop rule can accept a wrong 512 candidate only if the 1024 state's error is common to 512 and 1024. That is either:
> - a common-mode loss of an exact input, which the exact sums remove; or
> - saturated rounding below 1024's resolution (Lemma 2).
>
> Under 5a.3, the 1024 state's recovery resolution is charged by V = 2^(8−1024)·ê (derived). Its solve error is measured by the verification estimate, whose residual is one exact sum over the 1024-formed contributions, so K_1024's assembly roundings are included (R3). What remains is e_q, the formation error of individual 1024-formed entries (Q4: no re-formation above 1024), with \|e_q\| ≤ 62.5·2^-1024·Γ_q.
>
> **Honesty:** b = fl↑(2^-64·Φ) bounds the accepted 512 value to first order, except that e_q can exceed it by at most itself. At p = 512 every S\* ≥ Φ, so b ≥ 2^-502·ê and the excess is ≤ 2^-516·(Γ_q/ê)·b. R2's "c near 2^522" is restated this way: the tolerance can be used by \|Δ\|, so what matters is the excess over b, not a margin against 2^-64·M.
>
> **Availability:** the factor 4 between λ·2^-512·ê and 2^-64·Φ is the room for the 512 candidate's own resolution noise to pass the rule.

**Route 1's Oettli–Prager step.** It now uses the gate's backward error relative to |f| + Ā|u|, a larger majorant than |K||u| by a bounded factor (Ā's entries ≤ the element coefficients' sums). RETURN restates the constant.

**The uncertified step** (Hager–Higham's F) is unchanged.

### 6.7 Work (V4-N11, N12)

The sweep's work proxy, Σ(p/128)² over solves, excludes the E pass and the estimate:
- the E pass is one absolute recovery pass plus Ā's constrained rows;
- the estimate is one residual pass over the contributions (R3), one substitution pair and one linear recovery.

All are charged to the verification attempt. The residual pass has one term per contribution rather than one per assembled entry: for a member, 144 terms instead of its share of the assembled entries, about twice a gate residual. Together they cost about one refinement step, which is small beside the factorization. The proxy is +1.0 % against today over 2,000 frames, unchanged from R2.

## 7. Tests K4 must add, and the mutants (measured in emu3)

**Controls** (invented inputs; `models.py`, `models2.py`, `models3.py`, `lever3.py`; the expectations are R3's; `run_controls3.stdout.json`):

| Id | Model | Expected |
|---|---|---|
| **F-2** | I12's probe (EA/L = 512, ux(0) = 1, load 2^-300) | 128: rejected by V (Δ = 0, M = 0, V > 0). 256: rejected by Δ = 2^-300 > ε·M (N-4). **512**, with \|N\| = \|R\| = 2^-300 `relative_verified` |
| **F-2-SPOS** | Member 1 E = 2^200 with F-2's pattern and load 2^-60; member 2 unit section carrying 1 N | 512, all honest; today ratio 16 |
| **F-2-CEIL**, **PRESCRIBED-TAIL** (combination form, operands A = 1 and B = 2^-1000 with factors (1, 2^-100)), PRESCRIBED-TAIL-FREE | as the candidate | 512 via Φ, honest; today false at 128 |
| **ASSEMBLY-SAT** (V4-N2) | DEMOTION's layout with a unit member 2 | **512**, honest; today R = −1.5 N `relative_verified` against 0 |
| **DEMOTION2** (V4-S1) | E_big = 2^480; member 2's I = 2^400 | **512**, relative rows 7 → 4, b ≤ 4.7e-6 (V4: 1.67e-6 N on the force rows); documents the only demotion class |
| **EXACT-RIGID** (V4) | One unit member, both nodes fully prescribed with ux = 1 | 512 with b ≈ 1.1e-150 > 0 on the force rows (today 128 with b = 0); honest |
| **LEVER2-k90**, **-k100**, **-k110** (V4-R1; `lever3.py`) | V4's exactly representable lever (§2.5): gain 2^90, 2^100 or 2^110; prescribed rigid y-translation 1; tip spring 2^-580 of the tip's assembled diagonal; load P = 2^(⌊log2 ê⌋ − 438) on the out-of-plane branch. Every LDLᵀ pivot and multiplier of the spring-free matrix is dyadic in the natural order (checked exactly) | **Unresolved (Ceiling):** 128 fails the pivot test; 256 rejected by `verification_estimate` (W/V = 2,681, 2.7e6, 2.8e9); 512 rejected by the stop rule. Today and under R2's assembled residual: selected at 256, claim ratios 1,005, 1.03e6, 1.05e9. K4's Rust control uses the same model. Its expected outcome is Unresolved whatever the elimination order (argued): at 256 the estimate or Δ rejects, and at 512 Δ does, because 1024 resolves the spring. V4 built no case for K4's RCM order |
| **RIGID-UNLOADED** (F-3), **F-3-FREE**, **F-3-ROT** | as the candidate | 512, honest; today unresolved |
| **LOADONLY-y345** and **K4's Rust probe set** (5 loads × 1 and 3 members × y_ref (3,4,5) and (0,0,1)) | K4 `5ad1b6174` | **All selected at 128**, honest (in emulation; K4 builds the hybrid-gate control in Rust). The 8 single-mode y_ref (3,4,5) cases are refused under the coalesced gate, as in Rust |
| **GS-TRANS-y345**, **GS-ROT-y345**, **GS-ROT-y345-LOADED** | — | 512, 512, 128 (V4-N8: the GS outcomes are emulation-specific; the LOADED and LOADONLY cases are the robust ones) |
| **SEEDED-COMMON** (S2's control; V4-R2) | N05 with a test-only hook that adds 2^-110 m to node 1 uy of the **final state**, after the gate and before recovery, at every precision: a common verification error the stop rule cannot see | Unresolved (Ceiling), `verification_estimate` at 128, 256 and 512, **through the specified form**: the estimate's residual is recomputed on the final state, so it contains the seed. Without the estimate (M11), or with the gate's residual reused (M15, R2's text), selected at 128 with claim ratio 23 |
| **LEDGER-AT-RESTRAINT** | As the candidate | 128 |
| **M7-GS1**, **M10-G**, **R115-SEED3** | The mutant controls below | 512, 256, 512; honest |
| **E-UNIT** | Every control at every precision | E, g and Ā bits equal the generator's emulation of §4.1.6.2 |
| **E-HEADROOM** | Every control at every precision with a state, except the states the estimate rejects (LEVER2) | \|q_P − q_2P\| ≤ 2^8·2^-P·ê. Observed \|q_P − q\*\| ≤ 1.77·2^-P·ê over 739 states (`measure3_3.json`), a margin of about 2^7. LEVER2's 512 state reaches 7.2e11·2^-P·ê: the lost spring, which W measures exactly (ratio 1) |
| **TWO-SPAN** | K4-M11 | Still one correction at 128 |
| **E-ESTIMATE** (test-only) | Every control, at P = 256 and 512 | W_q agrees with \|R\*(u_P) − q\*\| (a P = 2048 reference) within a relative 2^-8, on rows whose error exceeds 2^-(P+20)·ê. Observed within 5·10^-11, LEVER2 included |

**Mutants.** A kill is a change of selected precision, of class counts or of honesty, against R3 (`mutants3.stdout.json`; M13's is a change of correction count, at evidence level).

| # | Mutant | Kill (emulated) |
|---|---|---|
| M1 | Drop V | F-2, F-2-SPOS, F-2-CEIL, PRESCRIBED-TAIL, ASSEMBLY-SAT: false claims at 128; DEMOTION2, M10-G and EXACT-RIGID at 128 |
| M2 | ê uncoupled | **Behavioural (S6):** F-3-FREE, F-3-ROT, GS-ROT-y345 and **R115-SEED3** unresolved (the frame the candidate's emulator generates at seed 3, index 115, from `sweep.gen`; made a required control); G5a fails on TWO-SPAN |
| M3 | Φ at every p | REACTIONS-ONLY 256 → 128 (SD-G4); F-2, F-2-SPOS, F-2-CEIL, PRESCRIBED-TAIL, the F-3 forms, DEMOTION2, ASSEMBLY-SAT, GS, M7-GS1, M10-ANISO, M10-G and EXACT-RIGID at 128; R115 at 256 |
| M4 | No Φ | F-2-CEIL, PRESCRIBED-TAIL, the F-3 forms, GS, M7-GS1, M10-ANISO, R115 and EXACT-RIGID unresolved; DEMOTION2 and ASSEMBLY-SAT relative rows 4 → 7 |
| M5 | E without the prescribed \|u\| | PRESCRIBED-TAIL and ASSEMBLY-SAT false claims at 128; RIGID-UNLOADED and M7-GS1 unresolved; DEMOTION2 at 256, M10-G and EXACT-RIGID at 128; G5a failures on PRESCRIBED-TAIL, REACTIONS-ONLY, DEMOTION2, ASSEMBLY-SAT, M10-G and EXACT-RIGID |
| M6 | E from Σ\|ledger terms\| | LEDGER-AT-RESTRAINT at 256 |
| M7 | Entrywise operator in E and Ā | **Behavioural (S6):** LOADONLY-y345, GS-TRANS-y345, GS-ROT-y345 and M10-ANISO refused by the gate; G5a lower-bound failures on RIGID-UNLOADED, F-3-FREE, F-3-ROT, REACTIONS-ONLY, SKEW6-K1E-12, B1-L, LEDGER-AT-RESTRAINT and R115. (M7-GS1 alone is not killed: the body-level ê masks it) |
| M8 | Φ = 2^-(2p−74)·ê | The F-3 forms, GS, M7-GS1, M10-ANISO and R115 unresolved; DEMOTION2 and ASSEMBLY-SAT relative rows 4 → 7 |
| M9 | Floor at 256 | F-2, F-2-SPOS and all floored controls at 256, EXACT-RIGID included |
| M10 | g = 1 | **Behavioural (S6):** **M10-G**, 256 → 128. M10-G uses DEMOTION2's layout with member 1's y_ref (1, 2^-12, 0) on its (2,0,0) chord, so g = 2^13 and the formation is exact; E_big = 2^173; member 2's I = 2^100. With g, ê/M = 2^186 > 2^184 rejects 128. **Why the anisotropic noise frame V4 suggested does not kill it** (derived; M10-ANISO was measured not to): a Gram–Schmidt error rotates an orthonormal frame about the member axis. Non-orthogonality enters only at second order, g²·2^-2P. So rigid motions leak no g-amplified force, and the g-amplified error is proportional to deformation (anisotropy × g·2^-P × the deformation force), which ε·M dominates. g is kept because the count needs it |
| M11 | Skip the estimate | **SEEDED-COMMON** selected at 128 with claim ratio 23; **LEVER2-k90** selected at 256, false |
| M12 | Coalesced gate only (K4 at A2) | LOADONLY-y345, GS and M10-ANISO unresolved (with K4's Rust probe cases) |
| M13 | Bounded test drives refinement | TWO-SPAN: 0 corrections instead of 1 (the K4-M11 evidence, O5). No outcome changes, so the kill is at evidence level, as O5's |
| **M14** (R3) | W's residual over K^q's assembled entries (R2's quantity) | **LEVER2-k90** selected at 256, false (claim ratio 1,005) |
| **M15** (R3) | W's residual reused from the gate's evaluation, before the hook (R2's text) | **SEEDED-COMMON** selected at 128, claim ratio 23; **LEVER2-k90** selected at 256, false |
| **M16** (R3) | The bounded test on the last state, not the best (R2) | **Not killed.** Availability only: on the probe cases the chosen state is not the last, but the last also passes. A K4 unit test on the gate's choice (SD-G5's new vector) is its kill, at evidence level |

**The estimate's kills (V4-R2).** M11 is killed by LEVER2, a natural control, and by SEEDED-COMMON. SEEDED-COMMON kills M11 and M15 through the specified form: the text now says the residual is recomputed on the final state, and emu3 implements exactly that. In R2 the text said "reused", and only the emulator's recomputation caught the seed. E-ESTIMATE is the estimate's positive test.

## 8. Where the design needs more than an S\* amendment

1. **The conditioned part: resolved at runtime, except e_q.** ROOT adopted the estimate and, at `e5f4ef3a8`, its contribution-level residual. What remains is e_q, the formation error of individual entries below q's resolution (§4.1.6.3). It is bounded by the gain Γ_q, not by a constant, and its smallness is a conjecture. The option of forming W's contributions at 3p + 64 narrows that window by 2^p (§4.1.6.3).
2. **The residual gate: resolved** (§5.6), with refinement preserved, the hybrid form confirmed and the best state tested.
3. **b is first-order, and the published value carries the 2^-22 relative gap** (§5.2). This is pre-existing and now stated.
4. **A per-body schedule.** An F-3 body still sends its whole case to 1024. **Not proposed.**
5. **Route 1's constant under the bounded majorant:** for K4's RETURN (V4-R8; nothing missing in the design).

## 9. What I could not resolve

- **e_q.** Nothing in the design bounds Γ_q by a constant times ê, so the claim that e_q is negligible remains a conjecture. What would break it is stated (§4.1.6.3), and no case is known. The 3p + 64 option is measured only on the controls.
- **M16 is not killed by any control.** V4-R5's change affects availability only; the probe cases choose an earlier state, but the last state also passes there.
- **The emulation of today's gate differs from Rust on one probe case** (the one-member in-plane load at 256; V4-R3). The outcome is the same.
- **Emulation only.** λ, the estimate, its contribution-level residual, the gate and G5a are checked against DS1's emulator and V4's, not against K4's Rust. K4's E-UNIT, E-HEADROOM, E-ESTIMATE, LEVER2 and hybrid-gate probe controls do that.
- **G5a item 4's unit conversion** is derived but not exercised: the emulator's rows are in SI units.
- **The R1 families listed in §6.1 as not measured.**
- **T1's fixtures.** Whether its support-motion fixtures hold rigidly moved, unloaded bodies (condition 3's R5-5 movements at F-3) was not surveyed.

## 10. NOTEs declined

None, in either check. V4-R8 needs nothing in the design. R2's N7 is K4's fix (`5ad1b6174`, K4-M33), cited, not specified here.

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

V4's delta check at R2 (`V4_VERIFICATION.md` "Delta check at R2", sha256 `c2f5539b…`, numerics `e5f4ef3a8`) and ROOT's rulings on it (`ROOT_RULINGS_V1.md`, section sha256 `69c8f23b…`).

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

## 13. Evidence (scratch, standard-library Python 3.13)

R2's evidence (`emu2.py` and the rest) is unchanged, and ROOT committed it at `_run_records_r2/`. R3's is new.

| File | What it is | sha256 (first 16) |
|---|---|---|
| `emu3.py` | R3's emulator: emu2 plus the contribution-level residual on the final state (`W_RES`), the best-state bounded test (`BEST_STATE`), the 3p + 64 option (`W_Q`), and the gate's chosen state recorded | `5b819ac2d1b32d7f` |
| `lever3.py` → `lever3.stdout.json` | LEVER2 at three gains under R3, M14, M15, today and the 3p + 64 option | `4ad85f5170dbf400` |
| `lever3_ratio.py` → `lever3_ratio.stdout.json` | LEVER2's W/V at the 256 rejection | `ffdf391d887fb5f2` |
| `models3.py` | R2's 40 controls, EXACT-RIGID and LEVER2 ×3 | `d11bc3612f10dd48` |
| `run_controls3.py` → `run_controls3.stdout.json` | 44 controls × {today, R2 as emulated, R3} | `4db84ccaffe6a0b0` |
| `mutants3.py` → `mutants3.stdout.json` | M1–M16 | `d595486b8d69bad1` |
| `gate_probe4.py` → `gate_probe4.stdout.json` | K4's probe set, rebuilt, with the gate's chosen state | `4c8a699c8784c9af` |
| `measure3.py` → `measure3_3.json` | Total, resolution, solve and estimate ratios | `24a5db452c842ac0` |
| `sweep3.py`, `summarize3.py` → `sweep3_{11,12}.json`, `sweep3_summary.json` | 2,000 frames | `375087efd50bdf6a` (summary) |
| `r1_lane3.py` → `r1_lane3.stdout.json` | K4's R1 lane (120 cases) | `8b86d34898ecc42f` |
| `checks3.py` → `checks3.stdout.json` | Formed elements against rigid translations and rotations | `71ddb1e8da502f84` |
| `controls_wq3.py` → `controls_wq3.stdout.json` | The 3p + 64 option on every control | `1b38a40acd9694aa` |

**To rerun** (from `<wt>/scratch/ds1/`):
- `python3 lever3.py` and `python3 lever3_ratio.py`;
- `python3 run_controls3.py`;
- `python3 mutants3.py`;
- `python3 gate_probe4.py`;
- `python3 checks3.py` and `python3 controls_wq3.py`;
- `python3 measure3.py 3 150`;
- `python3 sweep3.py 11 1000; python3 sweep3.py 12 1000; python3 summarize3.py sweep3_11.json sweep3_12.json > sweep3_summary.json`;
- `python3 r1_lane3.py <T3 path> lane`.

**Delegation.** DS1 ran as a Claude Code background subagent launched by ROOT through the Agent tool. This revision was made on ROOT's follow-up messages. ROOT is the only return path. DS1 dispatched nothing.
