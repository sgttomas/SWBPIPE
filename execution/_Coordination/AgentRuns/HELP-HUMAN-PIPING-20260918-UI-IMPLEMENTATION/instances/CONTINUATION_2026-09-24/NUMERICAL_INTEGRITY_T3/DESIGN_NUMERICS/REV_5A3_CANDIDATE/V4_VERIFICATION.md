# V4: independent verification of D1 revision 5a.3 (option iii)

V4 is a Type 2 TASK acting as an independent design verifier for ROOT (HELP_HUMAN), 2026-09-28. V4 did not write the design. DS1 did. V4 was asked to find defects, not to confirm. V4 fixed nothing and made no Git write.

**Candidate:** `DESIGN_NUMERICS/REV_5A3_CANDIDATE/D1_REV_5A3_SSTAR_RESOLUTION.md`, sha256 `c265d7bbbf4720d9f56a507ed26f5df68b1b9f47037afed17f95e9d084698b32`, at numerics `a7015eea3`.

**Claim labels.** Every claim below is either:
- **derived** (it follows step by step, and the steps are given); or
- **demonstrated** (it was run in V4's own standard-library emulator and exact-rational oracle; the file is named).

"Emulated" means V4's emulator, never DS1's.

## Verdict

**NOT VERIFIED.** There are 0 BLOCKING, 7 SHOULD-FIX and 13 NOTE findings.

**What holds (demonstrated and derived).**
- Option (iii)'s mechanism does what the candidate says:
  - F-2, F-2-SPOS, F-2-CEIL, the prescribed tail (both forms), RIGID-UNLOADED, F-3-FREE, F-3-ROT and EXACT-RIGID all reproduce in V4's emulator exactly as §2.5 and §7 state.
  - On 200 adversarial models built to saturate:
    - today's rule made 38 false claims;
    - (iii) made none (`honesty_sweep_3.json`).
- **λ = 2^8 is sound for the resolution part.** V4's own recount against K4's Rust gives at most 20g + 48 units; DS1 gives 139g.
- **The conjecture on the conditioned part was not refuted.**
  - Among V4's own controls, the worst realized total error is 0.79·2^-P·Ê.
  - Across 550 random adversarial frames the worst is 1.21. Across deliberately built levers and spines, with every tiny load set just below the saturation threshold, the worst is 0.21.

**Why not verified.** The text has errors that must be corrected before selection.
- A general claim is false: (iii) *does* demote, as DEMOTION2 shows (V4-S1).
- The honesty premise is still unproven, and the design's own residual gate tolerates more than λ (V4-S2).
- The routed residual-gate finding is wider than §8.2 says. K4's gate refuses an ordinary loaded model at every precision, so (iii)'s F-3 availability claim does not hold for such frames (V4-S3).
- Three parts of the specification are incomplete:
  - the precision of Φ and V in the stop rule (V4-S4);
  - the computation of g, and the edges of E (V4-S5);
  - behavioural kills for the mutants M2, M7 and M10 (V4-S6).
- The D2 reader loses recomputability at p = 512, and a cheap published-data check that closes most of the gap is available (V4-S7).

None of these makes (iii) unsound as a rule. A delta check of the corrected text should suffice.

## Findings

| ID | Severity | Site | Evidence | Resolution |
|---|---|---|---|---|
| V4-S1 | SHOULD-FIX | §4.7 "It never demotes"; §1 item 5 and §4.5 "no demotions", "no relative row is lost" | **Demonstrated** with DEMOTION2 (`t_demo.log`, `run_controls.json`):<br>• member 1 has E = 2^480 and is translated rigidly; its forces are exact zeros at every P;<br>• member 2 is a unit section carrying 1 N, and its sections are chosen so that no assembled entry saturates.<br>Today it is selected at 128 with N = ±1 `relative_verified`, exact. Under (iii), V rejects 128 and 256; at 512, Φ = 2^-438·Ê ≈ 3.1e13 N, and the ±1 N rows become `absolute_verified` with b = 1.67e-6 N (withheld before S-I).<br>**Derived:** V escalates once Ê/M > 2^(2p−72), and the floor demotes a row once \|q\| < 2^-472·Ê. Body-level Ê "poisons" a mixed body, which is the criticism §4.3 makes of (i-g), at a far higher threshold | Restate the claim: (iii) demotes only rows with \|q\| < 2^-472·Ê(body), after escalating at Ê/M > 2^(2p−72). Carry that into the (iii)/(i-g) comparison. Add a DEMOTION-type control beside EXACT-RIGID |
| V4-S2 | SHOULD-FIX | §3.3 "What λ does not cover", §5.2, §5.4, §8.1: the conjecture | **Demonstrated, not refuted** (worst 0.79 on V4's controls; 1.21 in the adversarial search; 0.21 on saturated levers and spines).<br>**Derived, supporting (single site):** take one coupling member of stiffness k_ic at the DOF, the rest of K positive semidefinite, and the member's energy bound D_q·(B_q·w)² ≤ wᵀKw. A saturated RHS tail t_i at a DOF coupled to prescribed motion obeys \|t_i\| ≤ 2^-2p·k_ic·\|a\|. Its force gain obeys G_qi ≤ √(D_q·(K⁻¹)_ii) ≤ √(D_q/k_ic), so the error is ≤ 2^-2p·√(D_q·k_ic)·\|a\| ≤ 2^-2p·Ê.<br>**Derived, against a proof:** the bound grows with the number of coherent saturated sites (the energy bound gives √n), and K4's gate admits \|r_i\| ≤ 64·γ(m_i)·d_i, that is at least 64·m ≥ 768 units against λ = 256. So λ cannot be derived from the gate. The premise rests on solves being far better than the gate requires | Before selection, ROOT either adopts §8.1's a-posteriori estimator as selected text or records the conjecture as an accepted named risk.<br>**Derived:** the exact residual sees F-2's lost load exactly (r = 2^-300 at 128 against 192 bits), so the estimator covers the RHS and solve sites. It does not cover a combination's rounded prescription, which V already covers |
| V4-S3 | SHOULD-FIX | §8.2 (routed gate finding); §4.5 "Realistic F-3 … is selected after the same four solves" | **Demonstrated** (`gate_probe_loadonly.*`): an ordinary loaded cantilever has no prescribed motion and is refused by K4's gate denominator \|f\| + Σ\|K^q u\| at 128, 256 and 512. Its setup is:<br>• a (3,4,0) three-member run with the root fully fixed and the N section;<br>• 1 N at the tip, y_ref (3,4,5), so e_y's x and y components are formation noise.<br>Worst gate ratios are 6.2e15, 6.5e15 and 7.1e15.<br>With y_ref (0,0,1), or with the bounded operator in the denominator, it is selected at 128 and honest. The ratio does not depend on p: noise at p against a denominator with no term of that size.<br>GS-TRANS-y345 and GS-ROT-y345-LOADED (N section) are also refused at all three precisions, so under K4's gate (iii) leaves them unresolved (`gate_probe_nsec.*`) | Correct §8.2's scope: the refusal is not only rigid motion, and "availability only" understates it for loaded models. Qualify §4.5's F-3 availability sentence. Route to ROOT and K4 as a gate defect with a Rust control before checkpoint B. This is emulation-only: V4 did not run K4 |
| V4-S4 | SHOULD-FIX | §5.1 (V_q, Φ), §4.1.6.2 item 3, §5.3 item 6a | **Derived:** V and Φ must be formed exactly as G5b forms them: ê from the published E bits and the binary64 coupling fl(E_mo/L_b) and fl(L_b·E_fo), then Φ = fl↑(2^-438·ê). K4's `scales_at` couples S\* at 2p with 2p roundings (`adaptive.rs:1093–1136`); if Φ is coupled the same way, Φ_stop ≠ Φ_pub. Then the published b = fl↑(2^-64·Φ_pub) can be below the stop rule's ε·Φ_stop, by up to about 2^-52 relative | State in §5.1 that the stop rule's V and Φ use item 6a's binary64 ê, bit for bit |
| V4-S5 | SHOULD-FIX | §3.1 item 1 and §4.1.6.2 item 1 (g); items 2 and 4 of §3.1 | **Derived:** g = 2^⌈log2(\|y_ref\|/\|y_⊥\|)⌉ is not computable as written. The text does not say:<br>• which \|y_⊥\| is meant (the exact value, or the 2p Gram–Schmidt residual `yc`, whose norm `form_member` discards at `assemble.rs:186`);<br>• how ⌈log2⌉ of a ratio of rounded values is decided.<br>E-UNIT pins E's bits across Rust and the generator, so the definition must be exact. (λ tolerates g low by a factor of 2: 80·2g + 59 ≤ 219g.)<br>Also open:<br>• Ā for reactions says "the \|k\| of springs at c", which is ambiguous for a `DirectionalSpring` block's off-diagonal terms;<br>• whether `Unpublishable` rows (O9) enter E's max;<br>• the rounding direction of E's intermediate 2p roundings | Define g exactly (for example from the 2p `yc` and \|y_ref\|², decided by exact comparison of squares, rounded up). Name directional blocks in Ā. State that unpublishable rows are included in E's max (or not). State E's rounding |
| V4-S6 | SHOULD-FIX | §7 mutants M2, M7, M10 | **Derived:** E-UNIT compares the Rust with a re-implementation of the same definition, so a definitional error passes both. D1 §7.3's own rule is: "A mutation that no discriminating comparison kills is reported, and its case set is extended before implementation continues" | Make DS1's R115 frame a required behavioural control for M2. Construct an anisotropic (Iy ≠ Iz) noise-frame control for M10. Add a per-row control for M7 (§3.1 says entrywise E fails per row on GS-y345) |
| V4-S7 | SHOULD-FIX | §6.3 G5a and G5b; D2 G5b "recomputed S\* must equal the receipt's bits" | **Derived:** at p = 512, S\*, the classification and b depend on producer-attested E, which readers cannot recompute. G5a's sanity bound only catches E below S\*_c, which is useless exactly where the floor binds: in F-3, S\*_c is the leakage.<br>**Derived:** E_N ≥ g·(EA/L)·(Nt_i + Nt_j) ≥ k_a·(Nt_i + Nt_j), from published displacement rows (`input_derived` included) and the receipt's k_a; likewise k_t·(Nr_i + Nr_j) for moments.<br>**Demonstrated** (`small_checks.json`): an honest E satisfies this lower bound on every V4 control. On RIGID-UNLOADED the bound is 0.4, against S\*_c = 1.1e-132 | Add to G5a: ê_fo ≥ max_m k_a,m·(Nt_i + Nt_j) and ê_mo ≥ max_m k_t,m·(Nr_i + Nr_j). Use a derived margin for p-versus-2p displacements (subtract 6·2^-63·S\*_tr from Nt) and for binary64 k_a (2^-40) |
| V4-N1 | NOTE | §3.3 table | **Derived** recount against K4's Rust (§2 below): end actions ≤ 10g + 30; reactions ≤ 20g + 45.5; the largest row ≤ 20g + 48 (support-group magnitudes). DS1's constants are about 2× larger on the frame terms, so both counts are within 2^8.<br>The table omits:<br>• stations (≤ 40g + 33 by DS1's constants);<br>• support-group components and magnitudes (+2.5);<br>• directional blocks and actions (≤ 6).<br>The first-order validity condition must include g: 139g·2^-2p ≪ 1, not "n ≤ 139 ≪ 2^120" | Complete the table and fold g into the validity condition |
| V4-N2 | NOTE | §2.5 instances; §7 controls | **Demonstrated** (ASSEMBLY-SAT, `t_demo.log`): today's rule publishes R(node 1, ux) = −1.5 N as **`relative_verified`**, against a truth of 0. The cause is the saturated assembled entry fl(2^479 + 1.5) = 2^479 at 128 and 256. (iii) escalates the case to 512, where it is honest. This is a K-entry saturation site with no control in §7 | Add a saturated-assembly control |
| V4-N3 | NOTE | §2.3 Lemma 2 | **Demonstrated** (`small_checks.json`): the hypothesis \|t\| < ½·ulp_2p(y) fails at a binade boundary. With y = 1 and t = −0.6·2^-2p, fl_p(x) = 1 but fl_2p(x) = 1 − 2^-2p. The stated sufficient condition \|t\| < 2^-2p·\|y\|/2 is correct | State the lemma with the sufficient condition, or with the half-spacing below y |
| V4-N4 | NOTE | §4.4 (ii) on F-2 | **Derived:** at 128 and 256, Δ = 0. The rule rejects because V > ε·M = 0, not "(\|0 − 2^-300\| > ε·2^-300)", which is the truth error | Correct the parenthesis |
| V4-N5 | NOTE | §6.6 restatement | **Derived:** "whose claim b = 2^-502·Ê bounds the 512 candidate's resolution error with a factor-4 margin" conflates availability with honesty. The factor 4 lies between the expected Δ ≤ λ·2^-512·Ê and the acceptance threshold ε·Φ. The honesty of b rests on \|e_1024\| ≤ V, with no margin beyond V | Reword |
| V4-N6 | NOTE | §4.4 "hold exactly, with no (1 + 2^-m) slack"; §5.2 (pre-existing since r5a N-1) | **Derived:** b bounds \|q_p − q\*\|, not \|q_pub − q\*\|. Two things push the published value past b:<br>• for an absolute row, the binary64 publication adds up to 2^-54·\|q_pub\| < 2^-88·S\*, which is up to 2^-24·b;<br>• S\*_pub can be below the stop rule's S\*_2p by up to (2^-64 + 2^-53)·S\*.<br>D2's C binds [q_pub ± b] | Either state b's scope, or widen b by a pinned factor |
| V4-N7 | NOTE | §8.6 (DS1 unverified); K4 A2 | **Derived** from K4's code, which rounds twice:<br>• `CasePrep::prescribed_at` rounds Σc·v once at p (`adaptive.rs:646–666`);<br>• `recover` copies u into the published values (`recover.rs:258`);<br>• `publish_value` rounds to binary64 (`recover.rs:459–468`).<br>**Demonstrated** (`small_checks.json`): 1 + 2^-53 + 2^-150 publishes as 1.0 at p = 128, where the correctly rounded value is 1 + 2^-52. At p ≥ 256 there is no difference | For K4, outside 5a.3: publish a combination's `input_derived` row as its exact sum rounded once to binary64 |
| V4-N8 | NOTE | §8.2's per-precision list | **Demonstrated:** the unloaded GS outcomes depend on the emulation and on the input bits.<br>• Reproduced: V4 reproduces DS1 for GS-TRANS-y345 and GS-ROT-y345-LOADED (N section, refused at 128, 256 and 512).<br>• Not reproduced: for GS-ROT-y345 (N section), the 128 candidate passes the gate and the 256 verification is refused, and (iii) selects it at 512 (`gate_probe_nsec.*`).<br>• With unit sections the unloaded cases are rejected by the stop rule, not the gate (`gate_probe.*`).<br>The sections differ in π's rounding (DS1: float `math.pi`; V4: `Fraction(math.pi)`; K4: `PI_Q`) | Present §8.2's list as emulation-specific; keep the LOADED and LOADONLY cases as the robust controls |
| V4-N9 | NOTE | §6.1 "No control stops discriminating"; R1 | **Derived** from `r1_ratios` and K4's RETURN §12.2: the R1 claim covers five of K4's six R1-lane families. RF-MECH's companions were not measured, and neither were VP-ROBUST's V-K families RF-LARGE, RF-INVARIANCE, RF-RANGE and RF-ZERO | Scope the claim, or measure those families |
| V4-N10 | NOTE | §6.3 and §6.4 sanity bound | The pinned constant is 2^-40 (`0x3D70000000000000`), but the check multiplies by 1 + 2^-40 (`0x3FF0000000001000`) | Pin the exact binary64 expression for all three languages |
| V4-N11 | NOTE | §6.7 "reuses its E" | **Derived:** E serves the verification role. When the 2p state becomes the next candidate, V uses the E of the new verification at 4p | Clarify |
| V4-N12 | NOTE | §4.5 and §4.7 "+1.4 % work" | **Derived:** the work proxy Σ(p/128)² counts solves only; the E pass (about one recovery pass plus Ā rows at 2p, §6.7) is not in it | State that the proxy excludes the E pass |
| V4-N13 | NOTE | D2 §4.11.2 ("the basis of b"); §4.9.9 notices | **Derived:** neither text mentions that at p = 512 a floored kind's b = 2^-502·ê comes from the producer-attested E | Update the D2 text with the addendum |

## 1. The failure class and the correction of §4.1.9

**The derivation is correct at first order** (derived):
- Lemma 1 (§2.2) is the triangle inequality.
- Theorem §2.4: saturated roundings are identical at p and 2p, so the stop rule sees only the rest. The excess over the claim is at most Δ_2p(q) ≤ 2^-2p·Σ\|g_k\|·\|x_k\|.
- The lemma's own statement is imprecise at binade boundaries (V4-N3). Its sufficient condition is right.

**§4.1.9's first half is refuted as claimed** ("error which depends on the precision is of order 2^-p at p and 2^-2p at 2p, so the stop rule sees it"). V4 demonstrates it independently on F-2, the prescribed tail and ASSEMBLY-SAT.

**Other ways a wrong value passes today** (demonstrated or derived; all are covered by (iii) unless stated):

| Site | Status |
|---|---|
| u's representation (F-2, F-2-SPOS) | Covered by λ (the rep term) |
| Reduced RHS (F-2) | Covered by the conditioned argument (V4-S2) |
| K's assembled entries: ASSEMBLY-SAT, a **false relative claim** on a reaction | Covered by λ for reactions. For the solve, it rests on the conjecture |
| A combination's rounded prescription (the prescribed tail) | Covered by λ (the rep term; E includes \|u_c\|) |
| Recovery stages, stations, spring actions, directional-spring actions, support-group components and magnitudes | Covered by λ (V4-N1) |
| Factorization and triangular solves (saturated intermediates in exactly representable structures) | The conjecture (V4-S2) |
| Binary64 publication rounding; S\*_pub below S\*_2p | **Not covered.** Pre-existing: relative rows have margin; b has none (V4-N6) |
| A combination's `input_derived` prescribed rows (double rounding) | **Not covered.** One binary64 ulp (V4-N7) |
| Common-mode loss of exact inputs (the fold mutant) | Removed by the exact ledger, as §6.1 says. V does not see it |

**Translations and rotations** (item 1).
- They are subject to saturation: u's representation, and RHS tails propagated through K⁻¹.
- **Derived:** the representation error is ≤ 2^-2p·\|u\| ≤ 2^-2p·S\*.
- **Derived:** the propagated tail is ≤ κ·2^-2p·‖u‖. The rcond screen at p gives κ < F·2^(p−1), so the tail is ≤ F·2^-(p+1)·‖u‖ (equilibrated norm).
- This depends on Hager–Higham's uncertified factor F, as before.
- **Demonstrated:** no translation or rotation row was dishonest in any V4 run, which includes 400 full-schedule runs in the honesty sweep.
- So "no V for translation and rotation" is sound to the same standard as r5a.2.

## 2. Attempts to break (iii)

### 2.1 λ re-counted against K4's Rust (first order, in units of u = 2^-P, relative to the bounded chain)

| Stage | K4 site | V4 | DS1 |
|---|---|---|---|
| e_x = d/L (exact sums, √, ÷) | `assemble.rs:166–172`, `normalize :130–148` | 4.5 | 4.5 |
| proj; yc = y_ref − proj·e_x | `:178`, `:179–185` | ≤ 10·\|y_ref\| + 1·\|yc\| | — |
| e_y | `:186` | ≤ 10g + 3.5 (2-norm) | 20g + 4 |
| e_z (cross product, normalized) | `:187–194` | ≤ 10g + 13.5 (the same as I12's K4 RETURN §9.1) | 40g + 20 |
| 1/L; EA/L and the other section terms | `:195`; `:198–213` | 3.5; 4.5 | 3.5; 4.5 |
| B bending translation entries fl(inv·e) | `:217–218` | ≤ 10g + 18 | 40g + 24.5 |
| d = T·u, with u's representation | `recover.rs:269–278` | ≤ 10g + 15.5 | 40g + 22 |
| e | `:282–301` | ≤ 10g + 20 | 40g + 26.5 |
| Q | `:302–323` | ≤ 10g + 25.5 | 40g + 32 |
| V (shear) | `:325–337` | ≤ 10g + 30 | 40g + 36.5 |
| Stations M(t) | `:360–379` | ≤ 10g + 26.5 | (≤ 40g + 33) |
| DB | `assemble.rs:247–256` | ≤ 10g + 23.5 | 40g + 30 |
| K_e | `:257–266` | ≤ 20g + 42.5 | 80g + 55.5 |
| Assembled K, then reactions (u's representation, final rounding) | `:490–509`; `recover.rs:404–416` | ≤ 20g + 45.5 | 80g + 59 |
| Support-group components and magnitudes | `recover.rs:420–451`, `:221–235` | ≤ 20g + 48 | (not listed) |
| Spring; directional-spring action | `:381–402`; `assemble.rs:300–337` | 2; 6 | (not listed) |

**V4's count is ≤ 20g + 48 ≤ 68g. DS1's is ≤ 139g.** Both are below λ = 2^8 = 256. λ is sound for the resolution part.

**Demonstrated** (`measure.json`, 19 models, every precision with a state): the realized resolution ratio max\|q_P − R\*(u_P)\|/(2^-P·E_q) is **0.69**. R\* is the exact recovery with exact operators.

### 2.2 The conjecture: the conditioned part within V

**Demonstrated** (`measure.json`, `adversary_{1,2}.json`, `lever2.json`, `spine2.json`), as max\|q_P − q\*\|/(2^-P·Ê) over force and moment rows:

| Model set | Worst |
|---|---|
| V4's controls and K4's survey models | 0.79 |
| 550 random axis-aligned dyadic frames (exact formation) with rigid prescribed translations, tiny loads next to the prescribed nodes, and stiffness spans of up to 2^70 | 1.21 |
| Beam levers with gain a/b up to 2^20 | 0.093 |
| Spines of up to 16 sites | 0.21 |

In the lever and spine models, every tiny load is 0.99 of half an ulp (at 256) of its DOF's coupling. That is the largest loss Lemma 2 permits.

**Derived:** a single-site bound (V4-S2) explains the lever and spine results. The √n growth in coherent sites and the gate's 64·m tolerance explain why the argument is not a proof.

**Not refuted. Not proved.**

### 2.3 Φ = 2^-438·Ê

**Derived:**
- ε·Φ = 2^-502·Ê.
- The expected Δ at 512 is ≤ λ·2^-512·Ê = 2^-504·Ê, a factor-4 availability margin.
- b = ε·Φ bounds \|q_512 − q\*\| whenever \|e_1024\| ≤ V.
- Φ is exact iff ê ≥ 2^-584.

**Demonstrated** (`small_checks.json`): the bits `0x2490000000000000` and `0x3D70000000000000` are right. At p = 512, every V4 control floored by Φ is honest: F-2-CEIL, both prescribed-tail forms, RIGID-UNLOADED, F-3-FREE, F-3-ROT and EXACT-RIGID. The worst claim ratio is 5.4e-4 of b.

### 2.4 Discriminating controls

**Demonstrated** (`fidelity.json`): K4's recorded outcomes, which V4's emulator matches, are unchanged under (iii):

| Control | Outcome |
|---|---|
| SKEW-K1E-28 | fails the condition screen at 128; 256 |
| SKEW6-K1E-12 | rejected at 128 on node 0 Rx; 256 |
| PIVOT | fails a pivot at 128; 256 |
| REACTIONS-ONLY | rejected at 128 by R(0, Uy); 256 |
| ZERO-TORSION-345 | 128 |
| ALL-ZERO-BODY | 128 |

V4 did not rerun B1-L, S8-W, R1 or the mutants M1–M10. No discriminating control that V4 ran stopped discriminating.

## 3. Ê's definition (item 3)

- **Well defined:** yes, except for g and the edges listed in V4-S5.
- **Computable at 2p in integers only, and deterministic:** yes, given V4-S5's specifications.
- **Encodable:** yes: fl↑ to binary64, overflow maps to `receipt_encoding`, and +0 only for 0.
- **The bounded operator for skew members is sound** (derived). Axis components are replaced by 1 ≥ \|e_c\|. Each component's absolute error is ≤ (10g + 13.5)·u. So noise-formed components, whose error is as large as their value, are bounded. The entrywise \|T\| and \|B\| are not bounded this way, as §3.1 says. For skew members the bounded chain exceeds the entrywise one by at most a factor of about √3 per 1-norm: conservative.
- **The coupling is required** (DS1's measured 1,217 uncoupled). This makes the conjecture depend on the L_b coupling, so M2 needs a behavioural kill (V4-S6).

## 4. Downstream effects (item 4)

- **G5a.** The shape check, zero rule and sanity bound are consistent.
  - The zero rule is derived correctly: E_q = 0 forces q = 0.
  - The sanity bound's pinned expression is incomplete (V4-N10) and does not guard the floor (V4-S7).
- **G5b.** Item 6a at p = 512 only is consistent with G5c and §4.9.10. It requires V4-S4, so that the stop rule and the reader use one Φ. Readers cannot recompute E (V4-S7).
- **G5c.** The formula is unchanged. For floored kinds, b = 2^-502·ê.
- **The receipt field `resolution_scale`.** It is complete for selected cases and combination entries. V4-S5 leaves its value ambiguous.
- **F2a.** Consistent. Product rows inherit fo and mo after item 6a.
- **VP-ROBUST.** Consistent. The R1 claims are scoped (V4-N9).
- **The retirement gate.** Condition 3 is affected:
  - rigid-motion zeros become intervals, as §6.3 says;
  - demotions at Ê/\|q\| > 2^472 can raise withheld counts before S-I (V4-S1).
- **Can a D2 checker recompute everything from published data?** No: E and the stop-rule summary are producer-attested. V4-S7 gives a published-data lower bound that closes the under-reporting gap where the floor matters.

## 5. The alternatives (item 5)

DS1's comparison is fair on the whole. Two points need correcting.
- **(iii)'s "never demotes" is false** (V4-S1). The real contrast with (i-g) is the threshold: 2^472 against (i-g)'s 2^118. That still favours (iii) strongly.
- **(ii) and (iii) share the unproven premise** for non-floored kinds with ε·M near V. (i-g)'s margin, 2^(p−62), makes that premise irrelevant for its non-floored kinds. DS1 states this.

**V4's view:**
- (iii) is preferable to (i-a), (i-b) and (i-g) on outcome preservation.
- Against (ii), it buys F-3 availability for a D2 change and a producer-attested E. With V4-S3 unresolved in K4, that availability does not reach noise-formed frames.
- The §8.1 estimator would remove the premise from both (ii) and (iii) at the cost of one solve pair and one recovery at 2p.

## 6. DS1's "found beyond S\*" items, reproduced (item 6)

1. **The residual-gate denominator.** Reproduced with a caveat, and extended (V4-S3, V4-N8):
   - The mechanism is derived: after refinement with the p-factor, r ≈ (K_p − K^q)·u. In a row whose true entries multiply small or zero displacements, d contains no term of that size. So \|r\|/d is O(1), and the gate ratio is independent of p.
   - The bounded-operator denominator passes every case V4 tried.
2. **A combination's prescribed rows rounded twice.** Confirmed from K4's code and demonstrated numerically (V4-N7).

## 7. Basis, method and records

**Read** (sha256, first 8 hex digits):

| Input | Hash |
|---|---|
| Candidate | `c265d7bb` |
| DESIGN.md r5a.2 (§4.1 in full, §4.3, §4.4, §4.10, §5, §7) | `fb62ef4a` |
| D2 r5b.2 (§4.9.3 G5a–G5c, §4.9.9, §4.9.10, §4.11) | `edc78f9c` |
| `ROOT_SELECTION_DESIGNS.md` | `2ef5325b` |
| `ROOT_RULINGS_V1.md` (K4 sections), now at `582e2a38` (DS1 read `8d855aa8`; later rulings were appended) | `582e2a38` |
| K4 at `3668ee8a4`, read with `git show`: `adaptive.rs` | `b2187c88` |
| `recover.rs` | `6a3645f5` |
| `assemble.rs` | `bae6c64f` |
| `factor.rs` | `81f81f24` |
| `combine.rs` | `ff220459` |
| K4 `RETURN.md` draft | `4b365be6` |
| K4's generator (model inputs) | — |
| DS1's `emu.py`, `models.py` and `gate_probe.py`: read only to locate the GS discrepancy (`ds1cmp_cmp_gs.py.txt` compares model inputs; V4's oracle never uses DS1's code) | — |

**The emulator** (`v4emu.py.txt`) was written by V4 from K4's Rust. It covers formation, assembly, the RHS, the LDLᵀ loops with the exact pivot screen, the solve, the residual gate at p + 64 with three corrections, recovery, the stop rule (current, ii, iii), E, and publication and classification. Its simplifications, stated in its header:
- ascending order, not RCM;
- an exact rcond (never stricter than K4's);
- no geometry-first step, directional springs or support groups.

The oracle is the same pipeline in exact rationals, on rational geometries: axis-aligned and 3-4-5.

**Fidelity** is demonstrated on six of K4's recorded survey outcomes (§2.4).

**Records** are in `_v4_records/`, with `SHA256SUMS`. Scripts are stored as `.py.txt`. The directory holds `run_controls`, `t_demo`, `fidelity`, `gate_probe`, `gate_probe_nsec`, `gate_probe_loadonly`, `measure`, `adversary` (seeds 1 and 2), `lever`, `lever2`, `spine2`, `honesty_sweep` (seed 3), `small_checks`, and each one's `.json` and `.log`. The scripts ran under `<wt>/scratch/v4/` with standard-library Python.
- Some logs repeat the imported `lever` and `fidelity` output, because those modules print when imported.
- To rerun: `python3 run_controls.py`; `python3 measure.py`; `python3 adversary.py 1 150 900`; `python3 honesty_sweep.py 3 200 1500`; and the other scripts with no arguments.

**Not done:**
- no Git write, no cargo build or test, and no Rust run;
- every K4 statement comes from reading its source;
- no file outside the write set, apart from `<wt>/scratch/v4/`.

**Delegation.** V4 ran as a Claude Code background subagent that ROOT launched through the Agent tool. ROOT is its only return path. V4 dispatched nothing.

## Delta check at R2

This section replaces the paused interim note. It covers R2, `D1_REV_5A3_SSTAR_RESOLUTION_R2.md` (sha256 `326a8d78…`, numerics `9e9e3056a`), against ROOT's rulings on this verification (`ROOT_RULINGS_V1.md` at `085638e58`, section sha256 `9a36d688…`) and K4 at `5ad1b6174`, read with `git show`: the probe `probe_v4_s3_y_reference_with_a_chord_component`, `run_345`, RETURN §21, and `publish_prescribed` / `exact_publication`.

**Method.** V4 implemented R2 in its own emulator (`delta_r2/v4emu_r2.py.txt`, written from R2's text), on V4's K4 emulator. It does not use DS1's emu2. It implements:
- g, exact from the formation's `yc`;
- the stage-rounded Ā, including springs;
- E, stage-rounded;
- V, Φ and the threshold from item 6a's binary64 ê;
- the estimate W, with its residual either reused from the gate (R2's text) or recomputed on the final state;
- the hybrid, coalesced and pure gates;
- G5a items 1–5.

Python only; no cargo. Every claim below is derived or demonstrated, as before.

### Verdict at R2

**NOT VERIFIED. 1 BLOCKING, 1 SHOULD-FIX, 6 NOTE.**

The revision resolves S1 and S3–S7 and every NOTE. It does not fully resolve S2: the adopted estimate misses a saturated assembled stiffness entry (V4-R1), and its specified form does not kill mutant M11 through the control R2 names (V4-R2). The fix for V4-R1 is small and demonstrated. With it, a delta check of the changed text should suffice.

| ID | Severity | Site | Evidence | Resolution |
|---|---|---|---|---|
| V4-R1 | **BLOCKING** | §5.5 (§4.1.6.3) "What W estimates", its named term "K^q's own formation error … 2^-72·V for c ≤ 256"; §5.2; §5.7 "a margin of 2^72 against V"; §4 "no longer rests on the conjecture"; §8 item 1; §9 | **Demonstrated** (`delta_r2/r2_lever2.*`): R2 publishes a false claim on LEVER2, an exactly representable dyadic lever:<br>• pivot at the origin, link at distance 1, tip at 2^90, with the node numbering chosen so that every LDLᵀ pivot and multiplier is dyadic (checked exactly);<br>• a rigid y-translation of 1 prescribed;<br>• a global spring at the tip's uy of 2^-580 times the tip's assembled diagonal;<br>• a load P = 2^-438·ê on an out-of-plane axial branch, which sets M so that V ≤ ε·M at 256.<br>Selected at 256, with W/V = 0 and every G5a item passing. The link force and two reactions are published as 0.0 `absolute_verified` with b = 2.69e-138 against a truth of 2.70e-135: a **claim ratio of 1,005**. Arm 2^100 gives 1.03e6; arm 2^110 gives 1.05e9.<br>**Derived:** the spring is lost in the assembled entry at every P ≤ 576, K^q included. So Δ = 0, r = f − K^q·u = 0 and W = 0. The error G·k·a is bounded only by the rcond screen (G ≲ √κ ≤ 2^127 at p = 256), not by c ≤ 256: here c ≈ 2^83. p = 512 is safe (a gain above 2^522 is needed). p = 128 is safe unless Hager–Higham underestimates κ by more than about 2^19.<br>Realism: absurd scales, an exact dyadic structure, and an elimination order that suits it. V4 did not construct a case for K4's RCM order | Form W's residual as **one exact expansion over the q-formed element entries and the binary64 spring and directional-block contributions** (contribution level), not over K^q's rounded assembled entries.<br>**Demonstrated** (`delta_r2/r2_lever_fix.*`): LEVER2 is then rejected by `verification_estimate` at 256 (W at 1.07e4 times V/4) and ends unresolved (safe). The 22 controls keep every selected precision and W/V.<br>**Derived:** in an exactly representable structure only assembly-level sums can saturate (an axis-aligned element entry has single-source terms). The remaining unmeasured term is element-entry formation error, which in a non-exact structure the stop rule sees.<br>Restate the named term and its margins to match |
| V4-R2 | SHOULD-FIX | §5.5 item 1 "reused, not recomputed" against §7 SEEDED-COMMON, the kill of M11 | **Demonstrated** (`delta_r2/r2_probe_seed.*`): R2's SEEDED-COMMON adds the seed to the solved state after the gate. With W's residual reused, as the text specifies, W is blind to the seed: the case is selected at 128 with claim ratio 23, identical to mutant M11. With the residual recomputed on the final state, the estimate fires at 128, 256 and 512 and the case is Unresolved. That matches DS1's result, because emu2 recomputes (`residual(u)` after the seed) | Recompute W's residual on the final state (V4-R1's fix does so necessarily), or place the hook where the reused residual sees it |
| V4-R3 | NOTE | §1's evidence table, "K4's Rust probe (20 cases) … All 20 selected at 128" | **Derived** from `5ad1b6174`: the Rust probe runs only the coalesced gate. The bounded-gate result is emulation (DS1's and V4's). V4 reproduces it independently: all 20 selected at 128, honest, 0 G5a failures.<br>For the one-member in-plane case, RETURN §21 (Rust) shows the coalesced gate failing at 256; V4's emulator agrees; DS1 says emu2 passes that state | Label the row as emulation, pending K4's Rust control |
| V4-R4 | NOTE | §2.3 Lemma 2's proof | **Derived:** the statement is correct. The step "since \|y\|/2 ≥ 2^(e−1), \|t\| < 2^-2p·\|y\|/2 is strictly inside the half-spacing" is a non sequitur: it gives a lower bound on the threshold. A two-case argument closes it:<br>• away from a power of two, the half-spacing is 2^(e−2p) > 2^-2p·\|y\|/2, because \|y\| < 2^(e+1);<br>• at y = ±2^e on the smaller side, the half-spacing is 2^(e−2p−1) = 2^-2p·\|y\|/2 | Replace the step |
| V4-R5 | NOTE | §5.6, the hybrid gate | **Derived:** the bounded test is applied to the last state. K4's loop leaves after a correction whose coalesced ratio did not decrease, so that state may be the worse one. Availability only | Apply the bounded test to the best state seen |
| V4-R6 | NOTE | §6.3 G5a item 4 | **Derived:** the published u and S\* must be taken after item 3's unit conversion (mm, kN). The conversion adds at most 2^-52 relative, inside the derivation's 2^-40 headroom | State it |
| V4-R7 | NOTE | V4's own errata, which R2 corrects | **Derived:** V4-N4 was wrong at 256: there Δ = 2^-300 and the rejection is by Δ (R2 §7 is right). V4-N6's 2^-24·b should be 2^-23·b: the unit roundoff is 2^-53 (R2 §5.2 is right) | None |
| V4-R8 | NOTE | §5.6, K4's RETURN §6.6, route 1 | **Derived:** the bounded majorant changes route 1's Oettli–Prager constant. R2 routes this to K4's RETURN; nothing is missing in the design | None |

### The checks ROOT asked for

1. **S1–S7 and the NOTEs.**
   - S1: resolved, with the threshold 2^-472·ê and DEMOTION2 as a control. **Demonstrated:** V4's DEMOTION2 goes 7 → 4 relative rows at 512.
   - S2: **not fully resolved** (V4-R1, V4-R2).
   - S3: resolved in emulation; K4 has not yet built the Rust control.
   - S4: resolved. **Derived:** V, Φ and the threshold come from one binary64 ê; V4's implementation uses exactly that, and G5b matches.
   - S5: resolved. g is exact; directional blocks are entrywise; unpublishable rows are included; stage rounding is to nearest; publication rounds upward.
   - S6: resolved. The M11 kill depends on V4-R2.
   - S7: resolved, with the note V4-R6.
   - N1–N13: resolved (§11). K4's fix for N7 is confirmed in `5ad1b6174`: `publish_prescribed` goes through `exact_publication`, rounding once.
2. **The estimate (§4.1.6.3).**
   - **Specified exactly:** yes, apart from reuse against recompute (V4-R2).
   - **Sound to first order:** for the RHS, the solve and u's representation. **Demonstrated:** the worst W/V is 0.0025 over the controls and 0.0035 over 200 adversarial models, where it never fired. SEEDED-COMMON is caught when the residual is recomputed.
   - **It misses** saturated assembled entries, and R2's c ≤ 256 is not true of them (V4-R1).
3. **The hybrid gate (departure 1). Sound.**
   - **Derived:** acceptance of values rests on the stop rule, V and W, computed on the final state, so neither the coalesced-driven refinement nor the bounded acceptance can make a published claim wrong.
   - **Derived:** the bounded test admits every state the coalesced test admits, since Ā ≥ \|K\| entrywise at first order and g ≥ 1.
   - A state that passes only the bounded test is judged by Δ (candidate) and by W (verification).
   - **Demonstrated:**
     - all 20 probe cases are selected at 128 and honest;
     - TWO-SPAN keeps its correction: 1 under the coalesced and hybrid gates, 0 under the pure form, as R2's M13 states;
     - V4's LOADONLY-y345 and GS-ROT-y345-LOADED are selected at 128.
   - **V4 supports ROOT confirming the hybrid form, with V4-R5.**
4. **V/4 (departure 2). Derived correctly.**
   - The resolution term is ≤ 68·2^-2p·ê. W ≤ 64·2^-2p·ê. The sum, 132, leaves 124 units of V for the unmeasured terms.
   - At V the sum would be 324 > 256.
   - u's representation is counted in both terms, which is conservative.
   - **V4 supports the V/4 form.**
5. **No control changes except those stated.** **Demonstrated** (`delta_r2/r2_controls.*`, 22 controls under today's rule and R2):
   - the survey controls are unchanged: N05, SKEW-K1E-28, SKEW6, PIVOT, REACTIONS-ONLY, ZERO-TORSION-345, ALL-ZERO-BODY;
   - the F-2, F-3, prescribed-tail, EXACT-RIGID, DEMOTION2 and ASSEMBLY-SAT sets go to 512 as stated;
   - LOADONLY-y345 and GS-ROT-y345-LOADED go to 128, as stated;
   - every R2 outcome is honest, with 0 G5a failures.
6. **G5a item 4 holds, and a D2 reader can check it** (derived). Its inputs are all published: displacement and rotation rows, `input_derived` included; the receipt's k_a, k_t, `resolution_scale`, S\*_tr and S\*_ro; and L_b. **Demonstrated:** 0 failures over the 22 controls, the 20 probe cases, LEVER2 and 200 sweep models.
   - Caveat: item 4 cannot catch V4-R1, because E itself is correct there.
7. **"Could not resolve" (§9).** Honest, except that the named K^q term's "large margins" are refuted (V4-R1). K4's Rust confirmations (E-UNIT, E-HEADROOM, the probe control) remain open, as stated.

### Records

The records are in `_v4_records/delta_r2/`; `_v4_records/SHA256SUMS` is regenerated to cover the directory (67 files).

| File | What it is |
|---|---|
| `v4emu_r2.py.txt` | V4's R2 implementation |
| `v4emu.py.txt` | V4's emulator, now exposing `yc` from formation |
| `r2_controls` | 22 controls |
| `r2_probe_seed` | K4's probe set; SEEDED-COMMON reused and recomputed; TWO-SPAN under each gate |
| `r2_lever`, `r2_lever_search` | The first, inexact attempts |
| `r2_lever2` | LEVER2, the break |
| `r2_lever_fix` | The contribution-level residual on LEVER2 and the controls |
| `r2_sweep` | 200 adversarial models under R2 |

Each script is stored as `.py.txt`, beside its `.json` and `.log`. Some logs repeat imported modules' output.

To rerun (from `<wt>/scratch/v4/`): `python3 delta_r2/r2_controls.py`, `python3 delta_r2/r2_probe_seed.py`, `python3 delta_r2/r2_lever2.py`, `python3 delta_r2/r2_lever_fix.py`, and `python3 delta_r2/r2_sweep.py 3 200 1500`.

Uncommitted.

## Delta check at R3

This covers R3, `D1_REV_5A3_SSTAR_RESOLUTION_R3.md` (sha256 `2cbb6582…`, 732 lines, numerics `c85dc1151`), against:
- ROOT's rulings on the R2 delta (`ROOT_RULINGS_V1.md`, section "D1 revision 5a.3: rulings on V4's delta check at R2");
- R3's §9 ("could not resolve") and §12 (the map of V4's R2 findings to their resolutions).

**Method.**
- V4's own emulator is extended to R3's specification: W's residual is one exact sum over the q-formed contributions on the final state, and the hybrid gate takes the best state. It is V4's code, not DS1's emu3.
- Python only, at most one process at a time, under `nice -n 19`. No cargo.
- The records are in `_v4_records/r3/`, with their own `SHA256SUMS`; `_v4_records/SHA256SUMS` is regenerated.

### Verdict at R3

**NOT VERIFIED. 1 BLOCKING, 0 SHOULD-FIX, 5 NOTE.**

R3 implements V4's R2 fix faithfully, and every R2 finding is resolved. **One gap remains, by ROOT's standard: e_q.** It is the formation error of individual q-formed entries, and R3 states its negligibility as a conjecture.

V4 could not show that e_q is unreachable. It could not build an adversary either: its element-level attempt is refused, with a derived reason. V4 shows that e_q can be **charged at runtime** from quantities the design already forms. With the 3p + 64 option, the charge costs nothing on every model tested.

That is the closure V4 recommends. A delta check of the added text and the charge's specification should then suffice.

| ID | Severity | Site | Evidence | Resolution |
|---|---|---|---|---|
| V4-T1 | **BLOCKING** | §5.5 (§4.1.6.3) e_q; §5.2; §5.7; §4; §8 item 1; §9 | **(a) DS1's bound is correct** (derived, first order). u\* − ũ = K\*⁻¹(K^c − K\*)ũ, and \|K^c − K\*\| ≤ 62.5·2^-q·Ā entrywise (V4's count, with Ā carrying g ≥ 1). So \|e_q\| ≤ 62.5·2^-q·Γ_q. The excess over b is ≤ 2^-66·(Γ_q/ê)·b at p = 128 and 256 (b ≥ V) and ≤ 2^-516·(Γ_q/ê)·b at 512 (b ≥ 2^-502·ê); b holds whenever Γ_q ≲ 2^65·ê.<br>**(b) Γ_q can be bounded at runtime** (derived), contrary to §5.5's "the rcond screen … does not bound Γ_q":<br>Γ_q ≤ ‖a_q·S‖₁ · ‖K̃⁻¹‖₁ · ‖S·Ā·\|u\|‖_∞,<br>where K̃ = S·K·S is K4's radix equilibration, ‖K̃⁻¹‖₁ is the verification's own Hager–Higham estimate times its uncertified factor F, Ā·\|u\| comes from E's reaction pass, and a_q is the recovery row.<br>**Demonstrated** (`r3/r3_charge.*`, exact ‖K̃⁻¹‖₁):<br>• at R3's q = 2p + 64, this charge exceeds e_q's allowance on the conditioned controls: SKEW-K1E-28 by 4.4e19, PIVOT by 1.4e18, SKEW6-K1E-12 by 2.7e6. Charging at q would move them from 256 to 512;<br>• at q_W = 3p + 64, every non-LEVER model (22 controls, 10 probe cases) passes with a margin of at least 2^62 (worst 1.3e-19 of the allowance);<br>• LEVER2 would also be charged out at 256.<br>**(c) Adversary.** TILT-LEVER (`r3/r3_tilt.*`) puts a within-entry loss in the lever arm: a tilt of 2^-290 whose axial (t/L)² term is below half an ulp at 576 bits, under a prescribed rigid rotation about the link node. It is refused at every gain (2^90, 2^100, 2^110). **Derived why:** a member is rotation-invariant in truth, so the lost tail's error lives in the computed state. It must be representable both at the leak site and at the output member, which are a lever arm apart, and that forces κ past the rcond screen. The other cases:<br>• a directional block's saturated tail is proportional to n nᵀ, so it acts only through the real stretch;<br>• springs enter exactly;<br>• tails acting on deformation are proportional to the element's real forces, and need a gain above 2^(2p).<br>This is a case analysis, not a proof: mixed cases, the constant relating \|K_e\|\|w\| to the element's forces, noise-formed elements and multi-element coherence are not closed.<br>**(d)** By ROOT's standard this is a gap, and BLOCKING until closed | **Close it by a runtime charge.** Add to §4.1.6.3 the test<br>C_q = 62.5·2^-q_W·F·est·‖a_q·S‖₁·‖S·Ā·\|u\|‖_∞ ≤ 60·2^-2p·ê(body, kind),<br>inside the 124 units V leaves. At p = 512, test it against 2^-22·b, or against the same remainder. Here est is the verification's Hager–Higham estimate and F a pinned allowance.<br>**Adopt the 3p + 64 option with it**, so the charge does not bind in practice (demonstrated).<br>This closes e_q by derivation to first order, **modulo F**: the same uncertified step as the rcond screen and W's own accuracy. The 3p + 64 option alone only shrinks the term by 2^p. The charge alone at q = 2p + 64 closes it too, at the cost of three conditioned controls |
| V4-T2 | NOTE | §5.5 "*Argued.* … V4 argued that in an exactly representable structure only assembly-level sums can saturate" | **V4 erratum, demonstrated** (TILT-LEVER's formation): a dyadic-tilted member is formed exactly at every P ≤ 576, yet its (uy,uy) entry carries an axial (t/L)² tail lost within the entry. So V4's R2 statement is false. DS1's own example (direction components differing by more than 2^(p+32)) is right | Drop the citation of V4's claim; keep DS1's sentence |
| V4-T3 | NOTE | §5.6 best state; §7 M16 survives | **Derived:** the gate's choice selects which state is final, and the stop rule, V and W then judge that same state. So the choice affects availability, never honesty. **M16 surviving is acceptable** | None |
| V4-T4 | NOTE | §5.6 "the chosen state is the second, third or fourth of four" | **Demonstrated** (`r3/r3_repro.*`): in V4's emulator the best state is #2 of 4 in seven of the eight single-mode probe cases and #1 in one. All 20 are selected at 128 and honest either way. This is an emulation detail | None |
| V4-T5 | NOTE | §5.5 "*Argued.* Such error enters the stop rule's Δ at its p-level size unless …" | **Demonstrated** (TILT-LEVER): that near-degenerate within-entry case exists, and the stop rule refuses it at the leak site (Δ ≠ 0 at 256 and at 512). This supports the argument | None |

### The checks ROOT asked for

1. **V4's fix, as specified. Faithful.**
   - §5.5 item 1 forms one exact expansion over the q-formed element and directional-block entries plus the binary64 spring stiffnesses, over every DOF, on the final state, recomputed.
   - Its work is charged in item 7.
   - **Demonstrated** (`r3/r3_repro.*`): **LEVER2 is refused at 2^90, 2^100 and 2^110**, by the pivot test at 128, the estimate at 256 and the stop rule at 512. V4's W/(V/4) values of 1.07e4, 1.10e7 and 1.12e10 equal DS1's W/V of 2,681, 2.7e6 and 2.8e9.
   - SEEDED-COMMON is refused by the estimate at 128, 256 and 512.
2. **e_q:** V4-T1, and (a) to (d) in its row.
3. **V4-R2 to R8, as resolved. All verified.**
   - R2: the residual is recomputed on the final state; SEEDED-COMMON is caught.
   - R3: labelled emulation.
   - R4: the two-case proof is correct.
   - R5: best state; M16's survival is acceptable (V4-T3).
   - R6: the conversion is absorbed. **Derived:** fl(ê·c) ≥ E·(1 + 2^-41) > E·(1 + 2^-47) ≥ LB.
   - R7: the errata are recorded.
   - R8: nothing is needed in the design.
4. **The false rigid-cancellation assumption. No step of R3 relies on it** (derived).
   - Every use is an upper bound: the λ count's leakage, the gate's point 2, and F-3's availability.
   - LEVER2 and ASSEMBLY-SAT rest on axis-aligned dyadic elements, whose products are exact, so the mirrored triangle negates exactly.
   - The only related error was V4's own (V4-T2).
5. **R2's p = 512 reframing, and R3's derivations. Correct** (derived).
   - At 512 the excess is ≤ 62.5·2^-1024·Γ_q ÷ (2^-502·ê) ≈ 2^-516·(Γ_q/ê)·b.
   - (i-g)'s 2^-(p+4)·(Γ_q/ê)·b holds for its non-floored kinds.
   - The gate's 2^-71 availability bound holds.
   - §5.2's inequality holds, and so do G5a item 4's constants.
   - **New errors:** V4 found none beyond V4-T1's "does not bound Γ_q" and V4-T2's citation.

**Also demonstrated.**
- R3 on V4's 22 controls and 20 probe cases gives the same selected precisions and classes as R2. All are honest, with 0 G5a failures.
- On V4's 200 adversarial models (`r3/r3_sweep_3.*`): 13 selected at 256, 187 at 512, 0 false claims, 0 G5a failures, the estimate never fired, worst W/V 0.0035.

**Records** (`_v4_records/r3/`, stored as `.py.txt`, each beside its `.json` and `.log`):

| File | What it is |
|---|---|
| `v4emu.py.txt`, `v4emu_r2.py.txt` | Now carrying R3's switches |
| `r2_controls.py.txt`, `r2_lever2.py.txt` | Made import-safe |
| `r3_repro` | The reproduction |
| `r3_charge` | The runtime bound |
| `r3_tilt` | The element-level adversary |
| `r3_sweep` | The adversarial sweep |

To rerun, from `<wt>/scratch/v4/`: `nice -n 19 python3 r3/<script>.py`; for the sweep, `nice -n 19 python3 r3/r3_sweep.py 3 200 1500`.

Uncommitted.

## Delta check at R4

This covers R4, `D1_REV_5A3_SSTAR_RESOLUTION_R4.md` (sha256 `4a52422d…`, 905 lines, numerics `c644b751d`), against:
- ROOT's rulings on the R3 delta (`ROOT_RULINGS_V1.md`, section "D1 revision 5a.3: rulings on V4's delta check at R3");
- R4's §9 ("could not resolve"), §13 (the map of V4's R3 findings) and §14 (the evidence in `_run_records_r4/`).

**Method.**
- V4 read DS1's `emu4.py` against §4.1.6.3 items 1 to 14, line by line, and re-executed it read-only (sha256 `cc2df0b8…`, identical to `_run_records_r4/emu4.py.txt`) for the R4 machinery. V4 did not port the charge into its own emulator this round.
- The Hager–Higham estimator (K4's `condition`, mirrored line by line from `factor.rs`), the comparison bound, the matrix families, the new controls and every derivation below are V4's own.
- Python only, at most one process at a time, under `nice -n 19`. No cargo.
- The records are in `_v4_records/r4/`, with their own `SHA256SUMS`; `_v4_records/SHA256SUMS` is regenerated.

### Verdict at R4

**NOT VERIFIED. 0 BLOCKING, 4 SHOULD-FIX, 5 NOTE.**

**V4-T1 is closed as ROOT ruled.** The charge is specified and emulated as ruled, with the Neumann factor, the γ-margins and per-body norms. The Theorem holds step by step, with two text defects (V4-U5, V4-U8). The Rust count behind Lemma B is confirmed stage by stage.

**The one remaining reliance does not hold as a bound.** Within both screens, K4's Hager–Higham estimate misses ‖K̃_P⁻¹‖₁ by as much as the pivot screen allows: 8.9e26 on a 12-DOF frame in DS1's own emulator, and 3.3e66 on a 16-DOF matrix at p = 256. No pinned F closes that.

V4 built no false claim from it: the charge's margin absorbed every violation it built. A cheap certified bound from the verification's own factor replaces F·est. On all 195 of DS1's states it stays below F·est, so no availability is lost (V4-U1).

The other findings:
- θ and the g check are over-broad: they escalate cases because of bodies or members whose rows are exactly zero (V4-U2).
- W⁺ is load-bearing: a seeded control kills M22 with a false claim (V4-U3).
- F3's stated obligation under-charges W1b's loads and omits the recovery side (V4-U4).

| ID | Severity | Site | Evidence | Resolution |
|---|---|---|---|---|
| V4-U1 | SHOULD-FIX | §5.5 items 7–8 and "Measured" (est); §5.9; §6.6; §8 item 1; §9, first bullet | **Demonstrated: within both screens, the estimate misses ‖K̃_P⁻¹‖₁ by as much as the pivot screen allows** (`r4/r4_hh.*`, `r4/r4_hhframe.*`, `r4/r4_hh_p256.*`).<br>• **Frame HH-FOOL-m** (DS1's emu4, 12 free DOFs, two bodies). Body H is four collinear nodes on t·(1,−1,0) with free DOFs ux, uy, uy, ux. Its axial members restrain every motion except w = (1,−1,−1,1), which only the bending, I = 2^-m, holds. Its DOFs sit at free positions 0, 2, 4 and 6, so w ⊥ e and w ⊥ the alternating vector. Body V is a grounded axial chain.<br>• exact/est = 7.7e8, 8.5e20, 8.9e26 and 1.1e57 at m = 40, 80, 100 and 200, at every P the model solves (for m = 200, the 128 candidate fails the pivot screen). The true norm exceeds F·est by 1.2e4 to 1.7e52. est stays 74.7 at every m: the run never sees body H's mode.<br>• **Matrix family F2** (16 DOFs, radix-equilibrated SPD): 2.5e30 at p = 128 and 3.3e66 at p = 256. The candidate's factor and the verification's both pass the pivot and condition screens.<br>• **Why (derived):** suppose every vector the run applies K̃⁻¹ to is orthogonal to a soft eigendirection w. Then the run (at most 11 solves, each fixed by the previous responses) is the same with or without the softness, by induction, while ‖K̃⁻¹‖₁ grows as 1/λ_w. Here e is an eigenvector of H's stiff part, so every sign is +1 on H. The iterations go to V's largest column, where w = 0, and stop on j = previous. The alternating vector is ⊥ w.<br>• **Safeguards measured:** 20 iterations give the same estimate. Power iteration from e is fooled (2.8e30 on F2), and so is power iteration from any pinned vector that is ⊥ w (2.2e18 on F3, built that way).<br>• K4's strict stopping test does defeat the textbook single-direction example (F1): rounding noise sends it to a unit vector, which finds the column (exact/est = 1.0 at every m). The miss needs F2's structure.<br>• **No false claim followed.** HH-FOOL-m40-LOADED is selected at 128 and honest with the true norm at 1.2e4·F·est, because the charge at the selected pair is 5.4e-50 of its allowance and grows at most quadratically in the norm. V4 built no case where a formation tail shared by p, 2p and q_W acts through a hidden mode | **The disclosure is too weak, and a certified replacement is at hand.**<br>• §5.5, §5.9 and §9 should say that, within the screens, the estimator's miss is unbounded (demonstrated), so no pinned F is a bound. The alternating vector is not the safeguard §5.5 implies.<br>• **Replace F·est by the comparison-matrix bound** Uc = U/(1 − U·γ_m·‖\|L\|D\|Lᵀ\|‖₁), where U = ‖M(L)⁻ᵀD⁻¹M(L)⁻¹e‖_∞ comes from the verification's own LDLᵀ factor. Here M(L) is the comparison matrix, with \|L⁻¹\| ≤ M(L)⁻¹ for triangular L, and γ_m is the factor's backward error. It is computed upward on nonnegative data, at the cost of one substitution pair and one pass.<br>• **Derived:** Uc ≥ ‖K̃_P⁻¹‖₁.<br>• **Measured** (`r4/r4_ucontrols.*`): Uc ≥ exact on all 195 of DS1's `est_check4` states. U/exact ≤ 96.4 there (worst SKEW6-K1E-12), and U/exact = 1.0 on every family above. Uc ≤ 0.0015·F·est on all 195, so θ, the charge and W⁺ would lose no availability on them. Uc can be loose, since a comparison matrix can grow like 2^n, but that costs only availability, never honesty.<br>• Then no step of the guarantee is uncertified. If Uc is not adopted, ROOT re-rules the limitation on this basis |
| V4-U2 | SHOULD-FIX | §5.5 items 9 and 10; §5.1 (c); §7 M20 and M23 | **θ takes the maximum of ‖SĀS‖₁ over every body, and the g check covers every member.** Both include bodies and members whose rows are exactly zero, which need neither Lemma B nor Lemma C.<br>**Demonstrated** (`r4/r4_kills.*`, emu4):<br>• **THETA-ZERO-BODY** is N05 plus an unloaded cantilever with Iy = Iz = 2^-400. θ = 2.0e60 at the 128 verification. R4 selects 256 (128 rejected: `theta`); M20 selects 128, honest.<br>• **G-FIXED-MEMBER** is N05 plus a fully fixed member with y_ref = (1, 2^-300, 0), so g = 2^301. R4 selects 256 (`g_validity`); M23 selects 128, honest.<br>• This contradicts R4's own reason for per-body norms (ALL-ZERO-BODY).<br>**Derived:** for every force or moment row q of body b at p = 128 and 256,<br>C_q/allowance ≥ θ_b·2^(p−64)·ρ_q/60, where ρ_q = ‖ā_qS‖₁·‖SĀ\|u⁰\|‖_∞/(‖SĀS‖₁·ê).<br>So θ_b fails while every charge passes only where ρ_q < 2^(71−p) for every row, that is, in near-idle bodies | Test θ_b = F·est·2^(7−P)·‖SĀS‖₁(b) **per body**, only for bodies with a nonzero load, prescription or state. Apply the g check only to members that have a free DOF or a nonzero prescribed DOF in such a body. Add both controls, each expected at 128 |
| V4-U3 | SHOULD-FIX | §7 M22; §7 "The guards that no control isolates"; §9 "The guards M20 to M23 are not killed by any control" | **Demonstrated** (`r4/r4_kills.*`), control SEEDED-SOFT:<br>• A stiff member carries 1 N. At its tip, a soft member (A = I = J = 2^-300) runs to a node loaded with 2^-240 N, which moves 2^60 m.<br>• A test-only seed of 2^40 m is added to that node's ux in the final state at every precision, with SEEDED-COMMON's hook.<br>• The seed moves the soft member's force by 2^-260 N, inside V/4. But it is 2^-20 of the displacement it corrupts.<br>• **R4:** Unresolved. The stop rule rejects u(node 2, ux) through W⁺ at 128, 256 and 512.<br>• **M22:** selected at 128, **false**, claim ratio 954.<br>So W⁺ is load-bearing, at the evidence level SEEDED-COMMON gives M11 and M15.<br>M20 and M23 are killed by V4-U2's controls, as availability kills. M14 and M21 are not killed (checks 5) | Add SEEDED-SOFT to §7 and to K4's controls (kills M22), and V4-U2's two controls. Correct §7 and §9 |
| V4-U4 | SHOULD-FIX | §6.5 "F3 (W1b) owes one extension"; §8 item 6; §9 | **(a) Mis-normalized (derived).**<br>• t₁ multiplies N_u by 2^(7−q_W)·F·est. So adding s_i·c_f·2^-q_W·Σ\|terms\| to N_u charges the formed-load error at 2^-2q_W. That under-charges by about 2^(q_W−6).<br>• By Lemmas A and C, the load error contributes ‖ā_qS‖₁·2F·est·max_i s_i·c_f·2^-q_W·Σ_i\|terms\|. t₁ covers that only if N_u gains max_i s_i·(c_f/63.5)·Σ_i\|terms\|.<br>**(b) The recovery side is missing.**<br>• Formed terms at a restrained DOF enter the reaction rows (R = Ku − f_c).<br>• E uses the net \|f_c\|, which can cancel.<br>• So E_q and Lemma B(iii)'s count must carry c_f·2^-P·Σ\|terms\|: for formed terms, the reverse of the rule M6 tests for W1a.<br>• The same holds for any member-load terms that enter end actions or stations | Restate F3's obligation:<br>• N_u gains s_i·(c_f/63.5)·Σ_i\|terms\|;<br>• E and the recovery count carry the formed terms' error;<br>• c_f is stated.<br>The scope itself is right (check 7) |
| V4-U5 | NOTE | §5.5 Lemma C, proof, first bullet | **Derived.** At p = 512, q_W = P = 1024, so 67·2^-P + 63.5·2^-q_W = 130.5·2^-P > 2^(7−P). The inequality is false there.<br>The conclusion still holds. At q_W = P the contributions are the state's own, so K^c − K_P is the single assembly rounding: \|K^c − K_P\| ≤ 2^-P·Ā·(1 + 2^-(P−8)) | Add one sentence for p = 512 |
| V4-U6 | NOTE | §7 M14 "63.5 → 64.5, within the charge's slack" | **Derived.** 2 × 64.5 = 129 > 2^7. In the γ-form the constant is (62.5 + 1)(1 + 2^-7) < 64, and 2·64·(1 + 2^-(P−8)) ≤ 128 holds, just. M14 stays non-behavioural and harmless | Use 64 |
| V4-U7 | NOTE | §7 M10: "Non-orthogonality enters only at second order, g²·2^-2P. So rigid motions leak no g-amplified force." The text dates from R2; V4 missed it at R2 and R3 | **Demonstrated** (`r4/r4_checks.*`, emu4 at P = 256, skew chord (3,4,12), y_ref 2^-k off the chord):<br>• e_x·e_y = 0.028·g·2^-P at g = 2^24 and 2^44. That is first order.<br>• The exact rigid rotation about the member axis leaks 0.007·g·2^-P·(B̄ᵀ\|D\|B̄\|u\|), which is 0.007·2^-P·Ā\|u\|.<br>**Derived:** for a rigid rotation ω the bending strain is, to first order, e₄ = (e_x·e_y)(ω·e_x).<br>The leak lies inside V only because E carries g, which is the reason to keep g | Replace the paragraph's stated reason. The M10-ANISO outcome is DS1's measurement |
| V4-U8 | NOTE | §5.5 items 6 and 9; Theorem step 5 | **(a) θ has no rounding direction** (derived). θ and the norms are not given one; C_q is "rounded upward". Lemma C needs θ ≤ 1/2 exactly, or a factor 1/(1 − θ). A θ rounded to nearest is low by at most a relative 2^-(P−8), which the slack in t₁ and t₃ absorbs.<br>**(b) Step 5 needs ‖SĀS‖_∞, while N_u carries ‖SĀS‖₁.** The two triangles of Ā are formed along different rounding paths, since DB̄ is rounded before B̄ᵀ·DB̄, so they can differ in the last place. **Measured:** exactly symmetric on every control and on an Iy ≠ Iz member. The 128/127 slack covers the difference | Say "rounded upward" for θ and the norms, and note the ∞-norm |
| V4-U9 | NOTE | §9 "This closure adds no new reliance"; §5.9 | **Derived.** Before R4 the estimate only screened: a missed ill-conditioning surfaced as a rejection by the stop rule or by W. R3's premise that used it was V4-T1's gap. Under R4 the published bound itself rests on ‖K̃_P⁻¹‖₁ ≤ F·est. The reliance is new for the guarantee, and V4-U1 shows it can fail | Reword. The point is moot if V4-U1's replacement is adopted |

### The checks ROOT asked for

1. **V4-T1 is closed as ruled, modulo V4-U1.**
   - **emu4 implements §4.1.6.3 items 1 to 14 as written** (read line by line):
     - q_W = min(3p + 64, 1024), giving 448, 832 and 1024;
     - W's residual and r₂ are single exact expansions over the q_W-formed contributions, with exact prescriptions;
     - S is the factor's radix scaling;
     - ‖SĀ\|u⁰\|‖_∞ runs over every column, and ‖SĀS‖₁ is a column sum over the body's free DOFs;
     - N_u carries the Neumann factor 2;
     - t₁ = 2^(7−q_W)·F·est·N_u, t₂ = 70·2^-P·‖S⁻¹δ̂‖_∞ and t₃ = 3·F·est·‖S·r₂‖_∞;
     - C_q = ‖ā_qS‖₁·(t₁ + t₂ + t₃), with ‖ā_qS‖₁ taken from the E pass with operand s;
     - the tests are 60·2^-2p·ê and 2^-22·2^-64·M_q;
     - W⁺ covers translation and rotation rows, magnitudes included.
   - **The margins hold:** 128 ≥ 2·63.5·(1 + 2^-(P−8)), 70 ≥ 69·(1 + 2^-(P−8)) and 3 ≥ 2·(1 + 2^-(P−8)).
   - **The per-body norms are sound.** K is block-diagonal by body, and the estimate of the whole matrix bounds each block. θ's maximum over bodies is the exception (V4-U2).
   - **The ruling's tests reproduce** (`r4/r4_repro.*`, emu4 re-executed), each equal to DS1's `run_controls4` record:
     - CHARGE-SLENDER is selected at 256, with 128 rejected by `charge`;
     - M17 and M19 select it at 128, honest, so their kill is a change of precision;
     - LEVER2 and TILT-LEVER are Unresolved at all three gains;
     - SEEDED-COMMON is Unresolved;
     - N05 is selected at 128.
   - **At p = 512 the allowance uses M_q rather than S\*,** so relative rows are allowed 2^-86·\|q_2p\|. §5.2's factor 1 + 2^-21 covers this.
2. **The Theorem holds step by step** (derived).
   - **Lemma A is correct.** Its symmetry holds: K4 forms element and directional blocks once per upper-triangle entry and mirrors them, so K\*, K^c and K_P are symmetric.
   - **Lemma B holds** (check 6).
   - **Lemma C holds** for K\*, and for K^c at p = 128 and 256. At 512 its inequality fails but the conclusion holds (V4-U5).
   - **The Theorem's identities hold:** r = K^c_𝓕𝓕(ũ − u), r₂ = K^c_𝓕𝓕(ũ − u − δ̂) and y = (K\*_𝓕𝓕)⁻¹·Δ·ũ, with Δ over every column. Each follows from subtracting the two systems.
   - **Steps 3 to 7 hold with their constants.** Step 5's norm is covered by V4-U8(b).
   - **The Corollary holds:**
     - 256 − 69 − 64·(1 + 2^-P) − 60 = 63 − 2^(6−P) > 62;
     - the p = 512 bound and the translation rows follow.
   - **The runtime checks use computed quantities.**
     - g is exact (exact dot products and an exact comparison). Lemma B's factor (1 − g·c·2^-P)⁻¹ covers its use of the P-formed y_c.
     - θ is formed from the computed est and Ā, but no rounding direction is stated (V4-U8(a)).
   - **Nothing is argued except F** (V4-U1).
3. **F = 2^16: it cannot be defended as a bound** (V4-U1).
   - **Worst ratio within the screens:**
     - 2.5e30 at p = 128 and 3.3e66 at p = 256 (F2);
     - 8.9e26 on the emu4 frame with its 128 candidate passing, and 1.1e57 at m = 200.
   - **est is exact to rounding** on DS1's 195 states (DS1), and on V4's F1 and F3, where K4's strict test defeats the classic tie.
   - **Safeguards:**
     - more iterations: no;
     - a second estimate by power iteration: no;
     - the comparison bound Uc: yes. It is certified, costs one substitution pair and one pass, and was measured at or below 0.0015·F·est on every state.
4. **The five additions.**

   | Addition | Correct | Necessary | Changes a selected outcome |
   |---|---|---|---|
   | Exact prescriptions in W's residual | Yes: step 2 needs ũ_𝓒 = u\*_𝓒 | Yes, for step 2 as written; otherwise (K\*_𝓕𝓕)⁻¹K\*_𝓕𝓒(u\*_𝓒 − u_P,𝓒) is uncharged | No, on DS1's controls; V4 could not build a kill |
   | r₂ pass | Yes: x̃ − δ̂ = (K^c_𝓕𝓕)⁻¹r₂ exactly | Yes: it bounds δ̂'s own error rather than estimating it | No |
   | θ and g checks | Yes: they are Lemma C's and Lemma B's hypotheses | Yes, where a body carries data | **Yes, spuriously**: two constructed controls go from 128 to 256 (V4-U2) |
   | Per-body norms | Yes | Yes (ALL-ZERO-BODY) | Only as intended |
   | W⁺ on translation and rotation rows | Yes (step 7) | **Load-bearing** (SEEDED-SOFT, V4-U3) | No, on DS1's controls |
5. **The surviving mutants.**

   | Mutant | Result | Class |
   |---|---|---|
   | M20 | Killed by THETA-ZERO-BODY (128 against 256). Derived: θ binds before the charge only in near-idle bodies (V4-U2) | Availability kill; the guard as scoped is over-broad |
   | M21 | Not killed. V4's attempt: a lost prescription tail, under 2^-P of its value, cannot exceed b on a translation row, because S\*_tr includes the prescribed node's magnitude row (emu4's layout). It would need a prescribed-to-free gain above 2^(P−64) | Kept hypothesis, no kill found |
   | M22 | Killed by SEEDED-SOFT: a false claim, ratio 954 (V4-U3) | Load-bearing |
   | M23 | Killed by G-FIXED-MEMBER (128 against 256) | Availability kill; the guard as scoped is over-broad |
   | M14 | Not behavioural; the constant is 64, not 64.5 (V4-U6) | Harmless; the contribution level stays per ROOT's ruling |
6. **Lemma B's count, rechecked against the Rust stages DS1 did not re-check.**
   - The sources are the exported `assemble.rs` (`form_member`, `form_directional`, `assemble`) and `recover.rs` (`recover`).

   | Stage | Count |
   |---|---|
   | B | 10g + 18: iz = fl(inv·e_z,k) gives 3.5 + (10g + 13.5) + 1 |
   | DB | 10g + 23.5: the section term 4.5, plus B, plus 1. The 4c and 2c factors are exact scalings |
   | K_e | 20g + 42.5: B, plus DB, plus 1. It is formed on the upper triangle and mirrored |
   | Assembled entry | 20g + 43.5: one exact sum, rounded once and mirrored |
   | d = T·u | 10g + 15.5: the axes, plus u's representation (1), plus 1 |
   | e | 10g + 20 |
   | Q | 10g + 25.5 |
   | V | 10g + 30 |
   | Stations | 10g + 26.5 |
   | Spring actions | 2 |
   | Directional blocks and actions | 4 and 6 |
   | Reactions | 20g + 45.5 |
   | Support groups | 20g + 46.5 |
   | Magnitudes | 20g + 48 |

   - Every stage agrees with §3.3. So do Lemma B's constants: (i) 65.5 → 67, (ii) 62.5 → 63.5 and (iii) 68 → 69, with (iii)'s 68 already including the prescribed value's representation through d.
   - Under the g check, the γ-form factor is at most 1 + 2^-9.5, against the 1 + 2^-7 used.
7. **The scope of W1b is acceptable.**
   - W1a's ledger is exact, so f\* = f and the Theorem needs no load term.
   - W1b's formed terms are outside W1a, and R4 excludes them until F3. That is consistent.
   - F3's stated obligation must be corrected, though (V4-U4).
8. **New errors:** V4-U5, U6, U8 and U9 in R4's text, and V4-U7 in older text.
   - §7's claims that θ binds only where another test already rejects, and that a large g "makes V large", hold only on DS1's controls (V4-U2).

**Records** (`_v4_records/r4/`, scripts stored as `.py.txt`, each beside its `.json` and `.log`). The dependency hashes are in `dependencies.sha256`: V4's `v4emu.py`, DS1's `emu4.py` and the model builders it imports, which are read-only.

| File | What it is |
|---|---|
| `r4_hh` | K4's estimator on the matrix families F1, F2 and F3, with the safeguards and U |
| `r4_hhframe` | HH-FOOL in emu4, with R4's schedule on the loaded variants |
| `r4_hh_p256` | F2 at p = 256, and HH-FOOL at m = 200 |
| `r4_ucontrols` | est, exact, U and Uc on DS1's 195 states |
| `r4_kills` | THETA-ZERO-BODY, G-FIXED-MEMBER and SEEDED-SOFT, each under R4 and under its mutant |
| `r4_checks` | Ā's symmetry, and the Gram–Schmidt non-orthogonality and leak |
| `r4_repro` | The ruling's controls, re-executed against DS1's records |

To rerun, from `<wt>/scratch/v4/r4/`, one at a time: `PYTHONDONTWRITEBYTECODE=1 nice -n 19 python3 <script>.py`. The scripts import DS1's `emu4` from `<wt>/scratch/ds1/` read-only, and V4's `v4emu` from `<wt>/scratch/v4/`.

**Delegation.** V4 ran as a Claude Code background subagent that ROOT launched through the Agent tool, and continued on ROOT's follow-up message. ROOT is the only return path. V4 dispatched nothing.

Uncommitted.

## Delta check at R5

This covers R5, `D1_REV_5A3_SSTAR_RESOLUTION_R5.md` (sha256 `ef68ab38…`, 1,020 lines, numerics `ff5dc2b05`), against:
- ROOT's rulings on the R4 delta (`ROOT_RULINGS_V1.md`, section "D1 revision 5a.3: rulings on V4's delta check at R4");
- R5's §9 ("could not resolve"), §14 (the map of V4's R4 findings), §15 (the evidence in `_run_records_r5/`) and §0 (the host-rule disclosure).

**Method.**
- Lemma D is checked against K4's `retained/factor.rs` in the k4 worktree. Its sha256 is `81f81f24…`, the same file R5 cites at `cef218a10`. The `Wide` value model is checked in `retained/wide.rs`.
- **V4's own code:**
  - an independent Uc with directed rounding, stress-tested at low precision;
  - a line-by-line port of K4's reverse Cuthill–McKee order;
  - Uc's looseness measured on R1's own RF-LARGE reference models, up to 1,000 members.
- **DS1's emu5 re-executed read-only:** its schedule, in its own natural order and in K4's order, on those models, and a scan of CHARGE-SLENDER for M17.
- Python only, one process at a time, under `nice -n 19`. No cargo.
- The records are in `_v4_records/r5/`, with their own `SHA256SUMS`; `_v4_records/SHA256SUMS` is regenerated.

### Verdict at R5

**NOT VERIFIED. 1 BLOCKING, 1 SHOULD-FIX, 5 NOTE.**

**Honesty: R5 holds.**
- Lemma D is correct as K4 codes the factor. The constant m = 2n + 2 is conservative, the Neumann hypothesis is tested at runtime and the directed rounding is valid for every operation.
- Tracing every honesty-bearing quantity in the Theorem and Corollary finds no step that rests on an estimate, a measurement or an argument.
- No case of Uc < exact was found: none in 46,315 low-precision factors with t up to 1 − 8·10^-8, and none anywhere else.

**Availability: R5 fails on R1's own large family (V4-V1).**
- The comparison-matrix bound grows geometrically along the elimination of a 3-D frame.
- **In K4's order, on R1's RF-LARGE references:**
  - at 100 members, U/exact reaches 2^151 to 2^628;
  - DS1's emulator, run in K4's order, moves five of the six from 128 to 256 or 512;
  - at 1,000 members, Uc does not exist at any precision (t ≥ 1 at P = 256, 512 and 1024) for five of the six. They would be Unresolved (Ceiling) with reason `uc`, whatever their conditioning.
- **A certified bound with polynomial looseness exists:** one shifted factorization. It is within 2^5.1 of exact at 100 members, where U is off by up to 2^628.

**Uc is global where θ is per body** (V4-V2): one body's looseness escalates or rejects every body. The NOTEs are wording and bookkeeping.

| ID | Severity | Site | Evidence | Resolution |
|---|---|---|---|---|
| V4-V1 | **BLOCKING** (on availability; honesty is intact) | §5.5 item 7 (U); §1 ("moves no sweep, probe or R1 outcome"); §9 ("Uc's availability on large models is not measured") | **Demonstrated on R1's own RF-LARGE reference models** (via DS1's `r1_adapter`, read-only; `r5/r5_large.*`, `r5/r5_large_sched*.*`).<br>• Each model is assembled by emu5 at P = 256 and radix-equilibrated. It is factored in K4's reverse Cuthill–McKee order (a line-by-line port of `factor.rs`) and in natural order.<br>• **10 members** (45–60 DOFs): U/exact is up to 2^47 in K4's order.<br>• **100 members** (450–600 DOFs), in K4's order: U/exact = 2^316 (CHAIN-AX), 2^628 (CHAIN-ROT), 2^151 (TREE-AX), 2^314 (TREE-ROT), 2^191 (CONT-ROT) and 2^0 (CONT-AX). t = U·γ_m·N_L ≥ 1 at P = 256 for CHAIN-AX, CHAIN-ROT and TREE-ROT, and at P = 512 too for CHAIN-ROT.<br>• **DS1's emu5 with its factor in K4's order** (`case.free` set to that order; everything else in emu5 is order-invariant): five of the six leave 128.<br>&nbsp;&nbsp;– CHAIN-AX goes to 512 (`uc`, then `charge`);<br>&nbsp;&nbsp;– CHAIN-ROT goes to 512 (`uc` twice);<br>&nbsp;&nbsp;– TREE-AX goes to 256 (`charge`);<br>&nbsp;&nbsp;– TREE-ROT goes to 512 (`uc`, then `charge`);<br>&nbsp;&nbsp;– CONT-ROT goes to 256 (`charge`).<br>&nbsp;&nbsp;With R4's F·est in place of Uc, all six are selected at 128. **In emu5's own natural order,** two already move: TREE-AX to 256 and TREE-ROT to 512.<br>• **1,000 members** (4,500–6,000 DOFs), in K4's order: log2 U = 1,733 to 6,604, against exact norms of at most 2^25 to 2^54 (the shift bound below, which is an upper bound; binary64 illustration). t ≥ 1 at P = 256, 512 and 1024 for five of the six (all but CONT-AX). Uc never exists, so they are Unresolved (Ceiling) with `uc` at every precision. In natural order, two of the six (TREE-AX and TREE-ROT) still fail at P = 1024.<br>• **Cause (derived):** \|L⁻¹\| ≤ M(L)⁻¹ discards every sign cancellation. A frame factor has several multipliers of order 1 per row, so M(L)⁻¹e grows geometrically along the elimination: about 3.4 bits per member for CHAIN-AX in K4's order, and 6.6 for CHAIN-ROT. CONT-AX's U equals the exact norm (measured) | **Keep Uc, and add a certified bound whose looseness is polynomial. Take the smaller of the two.**<br>**The shifted factorization (derived):**<br>• Factor K̃_P − σI with the same loop. Lemma D's step 1 gives its backward error, γ_m·\|L′\|D′\|L′ᵀ\|, plus the rounding of the shifted diagonal.<br>• If every pivot is positive, then by Weyl λ_min(K̃_P) > σ − γ_m·N_L′ − (the shift's rounding) =: σ′, using that a symmetric nonnegative matrix's 2-norm is at most its 1-norm.<br>• So ‖K̃_P⁻¹‖₁ ≤ √n/σ′, formed with directed rounding.<br>• σ can be chosen from est, which is an availability use of est. A failed shift halves σ or falls back to Uc.<br>**Illustrated in binary64** (`r5/r5_shift.*`, K4's order, σ = half an inverse-iteration estimate of λ_min):<br>• looseness 2^2.8 to 2^3.5 at 10 members and 2^4.3 to 2^5.1 at 100 (at most 2√n);<br>• finite at 1,000 members (2^25 to 2^54), where U is 2^1,733 to 2^6,604.<br>**Cost:** one extra factorization per verification, needed only when Uc is loose.<br>**Otherwise,** ROOT accepts the loss on this evidence, and R5's §1 and §9 must state it |
| V4-V2 | SHOULD-FIX | §5.5 item 7 ("Uc is one global bound"); §8 item 4 ("a per-body Uc is not proposed") | **R5 scopes θ per data-carrying body (V4-U2) but keeps Uc global.** So one body's looseness raises every body's charge, θ and W⁺, and one body's t ≥ 1 rejects the whole case with `uc`, whether or not that body carries data.<br>**Demonstrated by R5's own HH-SLENDER-m40:** an unloaded body's hidden mode lifts the global Uc 7.7e8 above est and pushes the loaded cantilever from 128 to 256.<br>**Derived:** L and D are block-diagonal by body (R5 item 7). So U_b = max of c_i over body b's rows, and N_L,b and t_b likewise, come from the same two passes at no extra cost.<br>**Consequence:** M17's and M24's only kill is HH-SLENDER-m40, and it would disappear. **Measured** (`r5/r5_m17.*`): CHARGE-SLENDER at Iy = Iz = 2^-180, 2^-184 and 2^-186 to 2^-200 gives no single-body case the charge alone refuses. The stop rule rejects first from 2^-186. So M17 and M24 would then be kept for the derivation, as M21 is | Use Uc_b per body, tested (`uc`) only for bodies with data, as θ_b is. Record M17 and M24 as kept for the derivation if no single-body kill exists |
| V4-V3 | NOTE | §7 M21; §9 ("M21 … vacuous in W1a, since every W1a prescription is 0") | **DESIGN §4.2 lists nonzero prescribed support motion as "kernel yes" in W1a;** only the facade refuses it until W1b. §7's own kernel controls use nonzero prescriptions: F-2, EXACT-RIGID, PRESCRIBED-TAIL (a combination with a tail), LEVER2, TILT-LEVER and G-PRESC-MEMBER. So "vacuous in W1a" holds for published W1a cases, not for the W1a kernel.<br>**"No other test implies the guard (derived)"** overstates. The derivation shows that t₁'s bound does not cover the term; it does not show that no test catches it | Say "vacuous for every case the W1a facade publishes", and "no charge covers the term" |
| V4-V4 | NOTE | §3.3, Lemma B, §14: "V4 measured the factor as at most 1 + 2^-9.5" | V4 derived that factor (R4 check 6); it did not measure it | Say "derived" |
| V4-V5 | NOTE | §7 THETA-STUB and M20 | **THETA-STUB's θ = 48 comes from the stub's block of K_𝓕𝓕.** Node 0's restraint decouples that block from the loaded member, and its state is exactly zero. So the kill still shows over-breadth, one level below V4-U2: Lemma C needs θ only per connected block of K_𝓕𝓕 that carries data.<br>A stub coupled to the moving part, with ρ_q < 2^(71−p) (V4-U2's inequality), would show θ binding where Lemma C is actually at issue. Under ROOT's ruling the availability kill is acceptable | Say what the kill shows, or build the coupled variant |
| V4-V6 | NOTE | §5.8, §6.3 G5a item 6: "Uc, rounded upward, finite and positive" | Uc can reach 2^1024 or more, outside binary64's range: for example at P = 1024, where Uc exists for U up to nearly 2^1024/(m·N_L), and Uc = U/(1 − t) exceeds U as t nears 1. The encoding failure is not specified; E has `receipt_encoding` | Specify it: `receipt_encoding`, or a log-scale field |
| V4-V7 | NOTE | **V4 erratum:** "Delta check at R3", verdict line | V4's R3 verdict said "5 NOTE"; its table lists four (V4-T2 to T5). R5 §13 records the correction (RV16-D2) | None |

### The checks ROOT asked for

1. **Lemma D, step by step. Correct.**
   - **γ_m with m = 2n + 2 is valid, and conservative, for K4's loop as coded** (derived from `factor.rs` `factor`):
     - each work_j and each pivot d_i is K̃_ij minus a sequentially rounded sum of t ≤ n − 1 rounded products;
     - each l_ij = fl(work_j/d_j) is one rounded division;
     - so by Higham's Lemma 8.4, K̃_ij = Σ_k l_ik·d_k·l_jk·(1 + θ_k), with |θ_k| ≤ γ_(t+1). R5 states γ_(t+2); γ_n already suffices;
     - entries outside the profile are exactly zero;
     - `Wide` has an i64 exponent, no subnormals, and each operation rounded once to nearest (`wide.rs`). So (1 + δ) with |δ| ≤ 2^-P holds exactly.
   - **The Neumann step is checked at runtime.** t is formed upward from U, γ_m and N_L, all upward; γ_m's denominator 2^P − m is exact. The test is t < 1; 1 − t is rounded downward and Uc upward.
   - **The directed rounding is sufficient.** "Nearest, then one ulp in the required direction" never lands on the wrong side for +, × and ÷ on positive operands, nor for 1 − t, including at binade boundaries.
   - **Stress-tested** (`r5/r5_stress.*`), with V4's own ru/rd and K4's loop:
     - 20,000 random radix-equilibrated SPD matrices (Gram, Laplacian, mixed-sign lever and near-singular kinds; n = 3 to 10) at P = 10, 12, 16, 20, 24 and 32;
     - of 95,363 factors, Uc existed on 46,315, with **0 cases of Uc < exact**; t reached 1 − 8·10^-8;
     - U alone was below exact on 27,781 factors, so the backward-error term is needed.
     - With m = n + 1, and even with m = 1, no violation appeared either. Random tests cannot certify the constant; the derivation does.
2. **"No uncertified step remains." Confirmed** (derived).
   - Traced:
     - the stop rule (a) to (d), decided exactly;
     - V and ê (E_q ≤ E ≤ ê: item 6a takes the max with E itself);
     - Lemma B's counts, V4's recount against the Rust;
     - Lemma A (symmetry, from the mirrored blocks);
     - Lemma C (θ_b upward; the p = 512 case repaired);
     - Lemma D;
     - Theorem steps 1 to 7 and their constants;
     - the Corollary's arithmetic;
     - the publication gap.
   - None rests on an estimate, a measurement or an argument. est appears only in the condition screen.
   - **Standing premises, which are not uncertified steps:**
     - K4 conforms to the specified stages (E-UNIT, E-CHARGE, E-UC);
     - the intended model is nonsingular per body, so u\* exists. Lemma C establishes this for bodies with data; data-less bodies rely on it, and have zero rows.
3. **Trying to break Uc.**
   - **(a) t near 1:** t reached 1 − 8·10^-8 in the stress test, with Uc ≥ exact. On R1's CHAIN-AX in K4's order, t at P = 256 crosses 1 between 10 members (2^-212) and 100 (2^102).
   - **(b) Uc < exact through rounding:** none. Not in 46,315 low-precision factors, nor on DS1's 245 states (DS1), nor on RF-LARGE, where Uc is far above exact.
   - **(c) Looseness on large models:** V4-V1.
4. **U2 and U3 as resolved.**
   - **The availability kills of M20, M23 and M24 are acceptable under ROOT's ruling,** with the observations below.
     - THETA-STUB still demonstrates over-breadth, at block level (V4-V5).
     - M24's kill, like M17's, exists only through Uc's cross-body coupling (V4-V2).
     - Neither V4 nor DS1 built an honesty case for M24. On V4's HH-FOOL frames the charge's margin absorbs est's miss (R5 §7).
   - **M21:** the vacuity holds for published W1a cases. Deferring its kill to W1b, and keeping it for the derivation, is consistent with ROOT's ruling. The wording needs V4-V3.
   - **SEEDED-SOFT's kill of M22** reproduces (R5: ratio 953.7).
5. **U4 to U9 as resolved. All correct** (derived).
   - **U4:**
     - 2^7/63.5 = 2.016 ≥ 2·(1 + 2^-(P−8)), so N_u's term max_i s_i·(c_f/63.5)·T_i charges the load error at its true size;
     - the recovery term (c_f/69)·T gives exactly c_f·2^-P·T under Lemma B(iii)'s 69.
   - **U5:** at q_W = P, K^c − K_P is the single assembly rounding.
   - **U6:** (62.5 + 1)·(1 + 2^-8.8) = 63.64 < 63.7, and 2·63.7·(1 + 2^-(P−8)) < 128.
   - **U7:** the reason is replaced. The M10-ANISO figure (about 29 units) is DS1's measurement.
   - **U8:** θ_b and every norm are rounded upward, and ‖SĀS‖ is the larger of the two norms. emu5 implements both.
   - **U9:** withdrawn.
6. **New errors:** V4-V2 to V4-V6.
   - §1 and §9's claim that Uc "moves no sweep, probe or R1 outcome" holds only because the R1 lane excludes RF-LARGE (V4-V1).

**Also.**
- **§0's host-rule disclosure** is recorded as DS1 states it. It concerns process, not the design.
- **DS1's emu5 factors in natural order, while K4 uses RCM.** The two give different U (V4-V1), so emu5's `uc_check5` availability figures do not transfer to K4.

**Records** (`_v4_records/r5/`, scripts stored as `.py.txt`, each beside its `.json` and `.log`). The dependency hashes are in `dependencies.sha256`: DS1's `emu5.py`, the builders it imports, `r1_adapter.py`, and R1's `references.py` and `references.json`.

| File | What it is |
|---|---|
| `r5_large` | U, N_L, t and exact on RF-LARGE at 10, 100 and 1,000 members, in K4's order and in natural order |
| `r5_large_sched` | emu5's schedule on the 12 models with 10 and 100 members, natural order, Uc against F·est |
| `r5_large_sched_rcm` | The same, with emu5's factor in K4's order |
| `r5_shift` | The shifted-factorization bound, illustrated in binary64 |
| `r5_stress` | Lemma D at low precision, plus the m-sensitivity runs |
| `r5_m17` | The CHARGE-SLENDER scan for a single-body M17 kill |

To rerun, from `<wt>/scratch/v4/r5/`, one at a time:
- `PYTHONDONTWRITEBYTECODE=1 nice -n 19 python3 <script>.py`, with arguments as the files record;
- `r5_large.py 10 700`, `r5_large.py 100 700` and `r5_large.py 1000 0`;
- `r5_shift.py 10,100 700` and `r5_shift.py 1000 0`;
- `r5_stress.py 20000`, then with `n+1` and with `1`;
- `r5_m17.py -180,-184,-186,-188,-190,-192,-194,-196,-200`;
- the two schedule scripts take the case ids in their logs.

**Delegation.** V4 ran as a Claude Code background subagent that ROOT launched through the Agent tool, and continued on ROOT's follow-up messages. ROOT is the only return path. V4 dispatched nothing.

Uncommitted.

## Delta check at R6

This covers R6, `D1_REV_5A3_SSTAR_RESOLUTION_R6.md` (sha256 `7cdb91c9…`, 1,198 lines, numerics `e47f853ba`), against:
- ROOT's rulings on the R5 delta (`ROOT_RULINGS_V1.md`, section "D1 revision 5a.3: rulings on V4's delta check at R5");
- R6's §9, §15 (the map of V4's R5 findings), §16 (the evidence in `_run_records_r6/`), and DS1's `emu6.py` (sha256 `767c9381…`).

**Method.**
- Lemma E and the per-block decomposition are checked step by step, against K4's `retained/factor.rs` (`81f81f24…`, as R6 cites) and `wide.rs`.
- **V4's own code:**
  - a low-precision stress of Lemma E;
  - the certified bounds on R1's RF-LARGE references, with its own factor loop, estimator port and directed rounding;
  - the norm at 1,000 members;
  - a check of emu6's estimator against K4's indexing.
- **DS1's emu6 re-executed read-only:** the R6 schedule on RF-LARGE at 100 members and on the 1,000-member CHAIN-ROT, and a copy of it re-indexed as K4 indexes (below).
- Python only, one process at a time, under `nice -n 19`. No cargo.
- The records are in `_v4_records/r6/`, with their own `SHA256SUMS`; `_v4_records/SHA256SUMS` is regenerated.

### Verdict at R6

**NOT VERIFIED. 0 BLOCKING, 1 SHOULD-FIX, 2 NOTE.**

**V4-V1 is resolved.**
- **Lemma E is correct step by step,** and the shift bound is certified.
- **In V4's own stress,** S never fell below the exact norm. That includes 6,448 factors where σ ≥ λ_min but every shifted pivot passed by rounding; in all of them σ′ < λ_min held.
- **V4 reproduces DS1's bounds on RF-LARGE** to every printed digit, at 10 and 100 members.
- **V4 computed the norm at 1,000 members** (binary64): S/norm is 2^7.09 to 2^7.29, and S ≥ the norm in every case.
- **emu6's R6 schedule, re-executed,** selects all six 100-member frames and the 1,000-member CHAIN-ROT at 128, honest.

**The honesty guarantee has no uncertified step.** est only screens and chooses σ, and any σ leaves Lemma E true. The per-block decomposition is sound for every norm the Theorem uses.

**One emulation defect (V4-W1).**
- **The defect:** emu6 indexes the Hager–Higham estimate's vectors by RCM rank; K4 indexes them by free position.
- **What stands:** honesty is unaffected. RF-LARGE's estimates, and all 59 controls' R6 selections, are unchanged under K4's indexing.
- **What does not stand:**
  - R6's statement that HH-FOOL fools the estimate by only about 2^5.6 "in K4's order" is wrong;
  - M24's kill by HH-SLENDER-m40 disappears.

| ID | Severity | Site | Evidence | Resolution |
|---|---|---|---|---|
| V4-W1 | SHOULD-FIX | emu6's condition estimate; §5.5 item 8 ("In K4's order the HH-FOOL frames fool the estimate less (it misses by about 2^5.6)"); §5.9; §7 HH-FOOL, HH-SLENDER-m40 and M24 rows; §9 (M24); §15 | **K4's `condition` indexes its vectors over free positions** (ascending global DOF): x, the sign vector, the argmax (last index on ties) and the alternating vector. They are mapped into the RCM order only inside `solve_scaled` (`factor.rs`). emu6 instead sets `case.free` to the RCM order and runs the same loops over that list, so its alternating vector and its tie-break are indexed by RCM rank.<br>**Demonstrated** (`r6/r6_k4hh.*`): emu6_k4hh is a copy of emu6.py in which only those two loops are re-indexed as K4's are.<br>• On HH-FOOL-m40 and -m100 at P = 256, H's est_c is 2^-2.1 against a norm of 2^35.7 and 2^95.7, a miss of 2^37.8 and 2^97.8. The global est is 2^6.2, a miss of 2^29.5 and 2^89.5. emu6 gives 2^30.1 and 2^90.1, the "about 2^5.6".<br>• **M24 on HH-SLENDER-m40:** emu6 selects 256 (the kill), emu6_k4hh selects 128, the same as R6. So the kill is an artifact.<br>**What is unaffected:**<br>• all five HH frames' R6 selections;<br>• **all 59 controls:** identical R6 selections and attempts under both indexings (`r6/r6_controls_k4hh.*`);<br>• **RF-LARGE:** on all 12 frames at 10 and 100 members, the K4-indexed est equals the RCM-indexed est to every digit, so σ, S and the selections stand (`r6/r6_rflarge_k4hh.*`);<br>• honesty, because no step of the guarantee uses est | Index emu6's estimate over free positions, as K4 does, and rerun the est-dependent evidence: the mutants, and the sweep (not rerun by V4). Correct the HH-FOOL statements.<br>Record M24 as kept for the derivation, like M17, unless another control kills it. K4's M24 test must not expect a kill from HH-SLENDER-m40 |
| V4-W2 | NOTE | §7 M27: "Not killed on any control or RF-LARGE case (derived): … the condition screen bounds κ at the candidate" | The condition screen's κ bound is est·‖K̃‖₁ < 2^(p−1), and est's miss is unbounded within the screens (V4-U1). In K4's indexing it misses by 2^97.8 on a screened frame (V4-W1). So "no selection or honesty kill exists at the design's precisions" is argued, not derived.<br>**Also demonstrated** (`r6/r6_lemmaE.*`): M27's term is load-bearing at low precision. ⌈√n⌉/σ fell below the exact norm in 566 of V4's factors (smallest ratio 0.45), and λ_min ≤ σ in 6,448 | Label it argued. The low-precision kill is acceptable evidence that the term is needed |
| V4-W3 | NOTE | §5.5 item 7c and "Looseness"; §6.7; §9 | **(a) S's polynomial looseness depends on est_c** (derived). At least one of the three shifts succeeds only if 8·est_c exceeds ‖K̃_c⁻¹‖₂ (σ/4 < λ_min) by more than the backward error. Where est_c is fooled (V4-W1), B falls back to Uc: on HH-FOOL that is exact, but a block that is both fooled and Uc-loose would be rejected with `uc`. §9's "a poor σ_c costs the shift bound" covers this; say that availability then rests on Uc.<br>**(b) Work budget:** the up to three extra factorizations count against the verification's work budget (D1 §4.1.7). Their effect on large models' budgets is not measured | State both |

### The checks ROOT asked for

1. **Lemma E, step by step. Correct** (derived).
   - **The Rayleigh-quotient Weyl step is valid with the rounded shifted diagonal.**
     - The loop factors Ĝ = K̃_c − σI + E_s, where E_s is the diagonal rounding.
     - So K̃_c = A′ + σI − E_s − ΔG, and for a unit x, xᵀK̃_c x > σ − ‖E_s‖₂ − ‖ΔG‖₂, because xᵀA′x > 0.
   - **δ is exact and bounds the shift's rounding.** It is a power-of-two multiple of a P-bit number. For round to nearest, |x − fl(x)| ≤ 2^-P·|fl(x)|, including when fl(x) is a power of two, so δ carries a factor of 2 to spare.
   - **γ_m's backward error is applied to the right input:** Ĝ, the matrix the loop actually factors. Lemma D's step 1 uses only the loop's roundings (K4's recurrence gives γ_(t+1)), and E_s carries the difference from K̃ − σI.
   - **‖Y‖₂ ≤ ‖Y‖₁ is used only where it holds:** for Y = γ_m·\|L′\|D′\|L′ᵀ\|, which is symmetric and nonnegative since D′ > 0. \|ΔG\| ≤ Y gives ‖ΔG‖₂ ≤ ‖Y‖₂ by Perron–Frobenius monotonicity.
   - **d′ > 0 suffices:** A′ = L′D′L′ᵀ is exactly positive definite. Weyl, not positivity, carries the backward error.
   - **The directed rounding is complete:**
     - σ may be any positive P-bit value, and the same value enters d̃ and σ′;
     - d̃ is rounded to nearest and covered by δ;
     - N′_L and γ_m are rounded upward, and γ_m's denominator 2^P − m is exact;
     - the bracket is rounded upward and the difference downward;
     - ⌈√n⌉ is exact, and the quotient is rounded upward;
     - σ′ ≤ 0 gives no S.
   - **Stressed with V4's own implementation** (`r6/r6_lemmaE.*`):
     - 1,500 random radix-equilibrated SPD matrices at P = 8 to 24, with σ swept from 0.25 to 3 times λ_min, plus σ′ ≈ 0;
     - 95,142 shifted factorizations; 31,773 had every pivot positive, and 14,186 gave S;
     - **0 cases with λ_min(K̃) ≤ σ′** (tested exactly: K̃ − σ′I positive definite by rational LDLᵀ), and **0 with S < exact**. This includes the 6,448 factors where σ ≥ λ_min and the pivots still passed, and 2,691 cases with σ′ < σ/16;
     - the smallest S/exact was 1.40.
2. **"No uncertified step remains." Confirmed** (derived).
   - Re-traced with S and per-block bounds: Lemmas A to E, the Theorem, the Corollary and item 14's encoding bound.
     - The encoding bound: θ_c ≤ 1/2 with ‖SĀS‖_c ≥ 1/4 gives B_c ≤ 2^(P−6), because K̃_ii ∈ [1, 4) and Ā_ii ≥ K_P,ii·(1 − 2^-(P−8)).
   - **est enters only three places, all availability:** the condition screen, the trigger (Uc_c > 2⌈√n_c⌉·est_c) and σ_c. Lemma E holds for any σ > 0, so a bad σ only fails the shift. Demonstrated: σ up to 3·λ_min never gave S < exact.
   - The standing premises are unchanged: K4 conforms, and K\* is nonsingular per body.
3. **Trying to break S: no break.**
   - **(a) d′ > 0 with λ_min < σ′:** none, in 14,186 cases.
   - **(b) σ′ ≈ 0:** none, in 2,691 cases.
   - **(c) Missed coupling:** impossible (derived).
     - Blocks come from the structural pattern, a superset of every nonzero of K\*, K^c and K_P: all 144 member positions, the spring diagonals, and the directional 3×3 blocks.
     - Cross-block entries of L, D and the work vector are exact zeros.
     - Coupling through a constrained DOF is not coupling in K_𝓕𝓕.
4. **Availability. Confirmed for R6, and for K4's indexing.**
   - **V4's own bounds at P = 256** (`r6/r6_bounds.*`) equal DS1's per-block log₂ U, Uc, est and S on all 12 frames at 10 and 100 members.
     - At 10 members, B ≥ the rational norm on all six. B/norm is 2^3.28 to 2^4.00 where the shift ran, and CONT-AX's B is Uc, equal to the norm.
     - At 100 members, B/norm = 2^5.46 to 2^5.64 (norm in binary64). CONT-AX's B is Uc, equal to the norm.
   - **The norm at 1,000 members was computed** (`r6/r6_exact1000.*`: binary64, one solve per column, 9 to 24 s per model):
     - 2^41.25 (CHAIN-AX), 2^47.40 (CHAIN-ROT), 2^43.39 (TREE-AX), 2^44.21 (TREE-ROT), 2^18.47 (CONT-AX) and 2^20.63 (CONT-ROT);
     - DS1's B/norm is 2^7.09 to 2^7.29, and B ≥ the norm on all six;
     - est is at most 2^0.19 below the norm (TREE-ROT), and equals it to binary64 accuracy on the rest.
     - Rational solves at 6,000 DOFs are infeasible in pure Python, and binary64 is ample against a 2^7 margin.
   - **emu6's R6 schedule, re-executed** (`r6/r6_sched*`): the six 100-member frames and CHAIN-ROT at 1,000 members are all selected at 128. They are honest against emu6's P = 2048 reference, with 0 G5a failures, as DS1 records.
5. **The per-block decomposition. Sound** (derived).
   - **Lemmas A, C, D and E are per block.**
   - **Every vector the Theorem bounds vanishes on data-less blocks.** r, r₂, δ̂, x̃ and Δ·ũ all do: a block coupled to a nonzero prescription carries data by definition. So Hölder's inequality block by block gives the body's maximum over its data blocks.
   - **N_u's ∞-norm over a data block's rows** involves only that block's columns and constrained DOFs, because Ā shares K's pattern.
   - **The charge's per-body maximum** is therefore an upper bound for every row of the body.
6. **The mutants.**
   - **The availability-only kills are acceptable under ROOT's ruling:** M20 (THETA-STUB-COUPLED), M23, M25, M26, M28 and M29.
   - **M24's kill does not hold in K4's indexing** (V4-W1).
   - **M27's low-precision-only kill is acceptable.** V4's stress also shows the term is load-bearing mathematically. DS1's "no kill at design precisions" is argued, not derived (V4-W2).
7. **V3 to V7: resolved as ruled.**
   - V3: scope restated, and "no charge covers the term".
   - V4: "derived".
   - V5: θ per data block, with THETA-STUB-COUPLED.
   - V6: `receipt_encoding`, derived unreachable.
   - V7: recorded.
   - **New errors:** V4-W1, V4-W2 and V4-W3.

**Records** (`_v4_records/r6/`, scripts stored as `.py.txt`, each beside its `.json` and `.log`). The dependency hashes are in `dependencies.sha256`.

| File | What it is |
|---|---|
| `r6_lemmaE` | Lemma E and M27 at low precision |
| `r6_bounds` | V4's own Uc, S and B on RF-LARGE at 10 and 100 members |
| `r6_exact1000` | The norm at 1,000 members (binary64) against DS1's S |
| `r6_sched` | emu6's R6 schedule re-executed on RF-LARGE-100 and on the 1,000-member CHAIN-ROT |
| `r6_k4hh` | emu6_k4hh (re-indexed as K4), HH-FOOL and M24 on HH-SLENDER-m40 |
| `emu6_k4hh.py.txt` | The patched copy of DS1's emu6.py; the two loops are the only change |
| `r6_rflarge_k4hh` | RF-LARGE est and S under K4's indexing |
| `r6_controls_k4hh` | R6's selections on the 59 controls under both indexings |

To rerun, from `<wt>/scratch/v4/r6/`, one at a time: `PYTHONDONTWRITEBYTECODE=1 nice -n 19 python3 <script>.py`, with arguments as the logs record. `r6_k4hh.py` regenerates `emu6_k4hh.py` from DS1's `emu6.py`.

**Delegation.** V4 ran as a Claude Code background subagent that ROOT launched through the Agent tool, and continued on ROOT's follow-up messages. ROOT is the only return path. V4 dispatched nothing.

Uncommitted.

## Delta check at R7

This covers R7, `D1_REV_5A3_SSTAR_RESOLUTION_R7.md` (sha256 `5502aef9…`, 1,219 lines, numerics `69737e699`), against:
- ROOT's rulings on the R6 delta (`ROOT_RULINGS_V1.md`, section "D1 revision 5a.3: rulings on V4's delta check at R6");
- R7's §0 (the emulator correction), §16 (changes from R6), §17 (the evidence in `_run_records_r7/`), and DS1's `emu7.py` (sha256 `cbf9780e…`, identical to `_run_records_r7/emu7.py.txt`).

**Method.**
- `emu7.py` is diffed against `emu6.py` and against V4's own re-indexed copy (`_v4_records/r6/emu6_k4hh.py.txt`), and read against K4's `condition` and `solve_scaled` (`retained/factor.rs`, `81f81f24…`).
- R6 is diffed against R7.
- DS1's claims are spot-reproduced with V4's own copy, not with emu7.
- Python only, one process at a time, under `nice -n 19`. No cargo.
- The records are in `_v4_records/r7/`, with their own `SHA256SUMS`; `_v4_records/SHA256SUMS` is regenerated.

### Verdict at R7

**VERIFIED. 0 BLOCKING, 0 SHOULD-FIX, 0 NOTE.**
- V4-W1 is resolved exactly as ruled, and V4-W2 and W3 are worded as asked.
- R7 changes evidence and wording only; the design is R6's, which V4 confirmed at R6.
- Every DS1 claim V4 spot-reproduced holds.
- No finding is open.

### The checks ROOT asked for

1. **The re-indexing matches K4's `condition` exactly.**
   - **In K4** (`factor.rs`), `condition` holds its vectors in free-position order (ascending global DOF): the start vector, the sign vector, the argmax over \|z\| (j starts at 0, and `>=` gives the last index on ties) and the alternating vector (n−1+i)/(n−1) with sign (−1)^i. Only `solve_scaled` maps them into the elimination order and back.
   - **emu7 changes exactly two things:** the argmax scan runs over free positions, starting at the first and taking the last on ties, and the alternating vector is built by free position (`r7/r7_emu_diff.txt`). This is logically identical to V4's `emu6_k4hh`; the differences are comments, the docstring and the `EST_INDEX` switch (`r7/r7_emu7_vs_v4copy.txt`).
   - **Nothing else in the estimator depends on the indexing** (derived):
     - the start vector e/n is uniform;
     - the sign vector is elementwise, with zero to +1 in both;
     - the dot product z·x and the 1-norm of y are exact sums, rounded once;
     - `j == previous` compares DOF identities;
     - the per-block ratios are per-block sums;
     - the column-sum norm is order-free.
2. **DS1's claims spot-reproduced with V4's own copy** (`r7/r7_spot.*`; V4's `emu6_k4hh`, not emu7):
   - **The 59 controls** under today's rule, R4, R5 and R6: every selection and every attempt list equals `run_controls7` (0 of 236 differ).
   - **The mutants:** R6 and M17, M20, M22, M23, M24, M26, M28 and M29 on ten controls (HH-SLENDER-m40, both HH-FOOL LOADED frames, CHARGE-SLENDER, THETA-STUB-COUPLED, SEEDED-SOFT, G-PRESC-MEMBER, SKEW6-K1E-12, and RF-LARGE-CHAIN-n00100-AX and TREE-n00100-AX), with honesty against P = 2048.
     - Selected precision, honesty, relative count, G5a and corrections all equal `mutants7` (0 of 90 differ).
     - M24 changes nothing on any of the ten, so its kill list is empty, as R7 says.
   - **HH-FOOL's estimate** at P = 256: H's est_c is 2^-2.08 against norms of 2^35.75, 2^75.75, 2^95.75 and 2^195.75 at m = 40, 80, 100 and 200.
     - Those are misses of 2^37.83, 2^77.83, 2^97.83 and 2^197.83.
     - The global est is 2^6.22, a miss of 2^29.5 to 2^189.5.
     - These equal R7's figures.
   - **Earlier V4 checks that still apply:** RF-LARGE's estimates at 10 and 100 members are identical under both indexings (V4's `r6_rflarge_k4hh`), and the R6 selections of the 59 controls are identical (`r6_controls_k4hh`).
3. **V4-W2 and V4-W3 are worded as asked.**
   - **W2:** §7's M27 row and §9 label "no kill at the design's precisions" as argued. They give the reason: the condition screen bounds est·‖K̃‖₁, and the estimate's miss is unbounded, 2^97.8 on HH-FOOL-m100. The low-precision kill stands, citing V4's 566 factors.
   - **W3(a):** §5.5's "Looseness", item 8 and §9 state three things:
     - S is polynomially loose only when the estimate is within about 8× of the norm, since one of σ, σ/2 and σ/4 must fall below λ_min;
     - otherwise availability rests on Uc_c;
     - a block both fooled and Uc-loose is rejected with `uc`, costing availability only.
   - **W3(b):** items 7 and 15, §6.7 and §9 state that the up to three shifted factorizations count against the verification's work budget (D1 §4.1.7), and that their cost on large models is for K6b and V-K to measure.
4. **Nothing else changed** (`r7/r7_design_diff.txt`, all 57 hunks read).
   - **The hunks are:**
     - the header, and §0's correction note and inputs;
     - §1's evidence rows;
     - the mutants7 citation in §3.4;
     - in §5.5: the work-budget sentences in items 7 and 15, the fallback in item 8, the HH-FOOL figures, the 8× condition under "Looseness", and "Measured";
     - §5.9's HH-FOOL figure;
     - §6.1's rerun notes and V4's 1,000-member norms;
     - §6.7's work budget and the 381 frames;
     - §7's HH-FOOL, HH-SLENDER-m40, M17, M24 and M27 rows;
     - §9 and §10;
     - a history note in §15;
     - the new §16 and §17.
   - **No specification sentence changes, except the work-budget statement ROOT ruled.** It makes explicit that R6's "charged to the verification attempt" means D1 §4.1.7's budget. Lemmas A to E, the Theorem, the Corollary and every test and threshold are R6's text.
5. **Open items. None blocks verification.** They are recorded plainly, for ROOT's selection record.
   - **Kept for the derivation, with no kill:**
     - M17 (the charge);
     - M21 (exact prescriptions; vacuous for published W1a cases);
     - M24 (est in place of the certified bound);
     - M27 at the design's precisions (killed only at low precision).
   - **Killed on availability only:** M20, M23, M25, M26, M28 and M29. No honesty kill exists for them.
   - **Argued, not derived** (none is a step of the honesty guarantee):
     - that M27 has no kill at the design's precisions;
     - that θ binds before the charge only where the rows with a large ‖SĀS‖ carry no state (THETA-STUB-COUPLED);
     - LEVER2's Unresolved outcome under every elimination order.
   - **Measured, not proved:**
     - that no emulated block is both est-fooled and Uc-loose;
     - the availability figures (sweep, controls, RF-LARGE, R1 lane);
     - the 1,000-member norm (V4's binary64, a 2^7 margin).
   - **Emulation only:** everything is checked against DS1's and V4's emulators, not K4's Rust. K4 still has to build these, with its E-UNIT, E-CHARGE, E-UC and SD-G5 controls:
     - directed wide rounding, for Uc, S, θ and the norms;
     - a shifted-pivot variant of its factor loop (d′ > 0);
     - the per-block bounds.
   - **Standing premises:**
     - Lemma B's count as V4 recounted it against K4's Rust;
     - Lemmas D and E tied to K4's factor loop as read at `81f81f24`; a change to its operation order needs γ_m re-derived;
     - K\* nonsingular per body;
     - K4 conforming to the specification.
   - **Out of scope until later work:**
     - W1b, until F3 meets §6.5's three obligations;
     - the shift's time and work on large models (K6b, V-K).
   - **Carried from earlier revisions:**
     - G5a item 4's unit conversion is not exercised;
     - the R1 families §6.1 lists as not measured;
     - T1's fixtures are not surveyed;
     - M14 and M16 are not behavioural;
     - the 1,000-member lane was rerun under R6's rule only. RF-LARGE's estimates are identical under both indexings, so today's rule, R4 and R5 would not change there.

**Records** (`_v4_records/r7/`):

| File | What it is |
|---|---|
| `r7_spot.py.txt`, `.json`, `.log` | The spot reproduction |
| `r7_design_diff.txt` | R6 → R7 |
| `r7_emu_diff.txt` | emu6 → emu7 |
| `r7_emu7_vs_v4copy.txt` | emu7 against V4's `emu6_k4hh` |
| `dependencies.sha256` | The dependency hashes |

To rerun, from `<wt>/scratch/v4/r7/`: `PYTHONDONTWRITEBYTECODE=1 nice -n 19 python3 r7_spot.py`. It imports V4's `r6/emu6_k4hh.py`, regenerated by `r6/r6_k4hh.py` from DS1's `emu6.py`.

**Delegation.** V4 ran as a Claude Code background subagent that ROOT launched through the Agent tool, and continued on ROOT's follow-up messages. ROOT is the only return path. V4 dispatched nothing.

Uncommitted.
