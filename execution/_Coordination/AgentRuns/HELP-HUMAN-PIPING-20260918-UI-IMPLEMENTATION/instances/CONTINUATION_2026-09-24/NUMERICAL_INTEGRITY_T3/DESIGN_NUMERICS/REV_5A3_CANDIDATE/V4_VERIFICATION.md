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
