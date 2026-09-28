# D1 revision 5a.3 (proposed): the stop rule's resolution scale and floor

Design addendum drafted by DS1, a Type 2 TASK (numerical design drafter), for ROOT (HELP_HUMAN) on 2026-09-28. It answers ROOT's ruling "K4: A1 findings F-1 to F-3, the stop rule's blind spot" (`ROOT_RULINGS_V1.md`) and ROOT's A2 input. **It is a proposal.** ROOT selects it after an independent verifier has checked it. `DESIGN_NUMERICS/DESIGN.md` (revision 5a.2, sha256 `fb62ef4a…`) stays hash-pinned. The amended text is in §5, written as addendum blocks.

**Labels used throughout.** A claim is marked as one of:
- **Derived:** it follows step by step from the stated definitions, and a verifier can check it.
- **Measured:** it was observed in DS1's standard-library emulation (§10). A measurement is evidence, not proof.
- **Argued:** it is reasoned but not proved.
- **Conjecture:** it is believed, and it is not established.

## 0. Basis, inputs and what was not done

**Read.** Hashes are sha256, first 8 hex digits; `<wt>` is the T3 worktree root.

| Input | Where | Hash |
|---|---|---|
| Root `AGENTS.md`; `agents/AGENT_TASK.md` | numerics tree at `941af39bd` (was `2f48511b7` in the brief; the one commit between them adds only K5 rulings) | `c8ce87ef`; `1a13a5b0` |
| `T3/TASK_BRIEFS/_COMMON.md` | same | `6bb845bf` |
| D1 `DESIGN_NUMERICS/DESIGN.md` r5a.2, read in full: §4.1, §4.3, §4.4, §4.10, §5, §6 rows, §7 | same | `fb62ef4a` (the selected pin) |
| D2 `DESIGN_STANDING/DESIGN.md` r5b.2: §4.9 (G5a–G5c, §4.9.9, §4.9.10), §4.11 | same | `edc78f9c` (the selected pin) |
| `ROOT_SELECTION_DESIGNS.md`; `ROOT_RULINGS_V1.md`, the K4 sections | same | `2ef5325b`; `8d855aa8` |
| K4 brief `TASK_BRIEFS/I12_K4_IMPLEMENTATION.md` | same | `740c380a` |
| I12's plan `<wt>/scratch/i12/CHECKPOINT0_PLAN.md`, §4, §7, §8, §11 | scratch | `e1253faa` |
| I12's A1 probe `<wt>/scratch/i12/a1_probes/combine_tests_probes.rs` | scratch | `8653068e` |
| K4 `retained/{adaptive,recover,assemble,factor,combine}.rs` | branch `codex/piping-k4-20260928` at `cef218a10` (A1) and `3ed6c0e26` (A2), read with `git show` | A1 `adaptive.rs` `7d65bbc0`, A2 `b2187c88`; `recover.rs` `6a3645f5` at both |
| K4's generator `tests/retained_k4/gen_k4_vectors.py`, and the survey output `<wt>/scratch/i12/a1_k4_tests.txt` | A1 and A2 | — |
| K4's SD-tagged tests (`adaptive_tests.rs`, `combine_tests.rs`, `classification_tests.rs`, `references_tests.rs`), and I12's R1 lane run `<wt>/scratch/i12/r1_run1.txt` | A2 | — |
| R1 references `REFERENCES/references.json`; `references.py` (imported read-only, for its input parser and PI_Q) | numerics tree | `7b176dbb`; `80d473a7` |

**Where the stop rule lives in K4 (A1 `cef218a10`, `retained/adaptive.rs`).**
- `scales_at` `:1021`: S\* at 2p, with input-derived rows excluded (`:1033`).
- `stop_rule` `:1069`.
- `coupled_scales` `:292`, `body_extent` `:272`, `threshold` `:339`, `absolute_bound` `:345`, `classify` `:372`, `classify_rows` `:1372`.
- `residual_rows` `:799`, `run_schedule` `:1680`.

At A2 (`3ed6c0e26`) the bodies of these functions are byte-identical, at `:1093`, `:1141`, `:1430` and `:872`; the other lines moved. A2's commit message records that the prescribed values are now rounded once from their exact sums.

**Done.** I ran standard-library Python only (`fractions`, exact integer rounding), under `<wt>/scratch/ds1/` (§10).

**Not done.**
- No Git write.
- No cargo build or test, and no Rust run. Every K4 statement comes from reading its source and I12's recorded outputs.
- No product code.
- No file outside `<wt>/scratch/ds1/` was written.

**Emulator fidelity (measured).** DS1's emulator reproduces the selected precision of all 20 of K4's A1 survey models that it re-declares. Where K4 records a failure or a rejection, the emulator gives the same one:
- SKEW-K1E-28 fails at 128 on the condition estimate;
- SKEW6-K1E-12 is rejected at 128 on Rx at node 0;
- PIVOT fails a pivot at 128;
- REACTIONS-ONLY is rejected at 128 by R(0, Uy);
- RIGID-UNLOADED is rejected by R(0, Uy) at 128, 256 and 512;
- SKEW-K1E-60 fails a pivot at 128 and is rejected on Rx at 256.

The emulator's simplifications are listed in `emu.py`'s header: natural order instead of RCM, no geometry-first step, no support groups, and a reference at P = 2048.

## 1. Summary

**Recommendation, in five lines.**
1. Correct §4.1.9. The stop rule is blind to *saturated* rounding: a tail below 2p's rounding unit is lost identically at p and 2p (§2). A wrong value then passes by up to the verification's own first-order resolution error.
2. Define a per-body formation scale E for the force and moment kinds. E is the "Σ\|terms\|" of every recovery expansion, over a bounded operator, coupled through L_b as S\* is, and computed at 2p (§3). λ = 2^8 bounds the resolution error by a first-order rounding count.
3. Charge the verification's resolution to the candidate: accept p only if \|q_p − q_2p\| + V ≤ 2^-64·max(\|q_2p\|, S\*), with V = 2^(8−2p)·Ê. This fixes F-2, its S\* > 0 form and the prescribed tail, by escalation.
4. At the last candidate (p = 512) only, floor force and moment S\* at Φ = 2^-438·Ê. This makes F-3 and the unresolvable tails available at 512 with an honest absolute bound. It is option (iii).
5. Publish E per body in the receipt (`resolution_scale`). D2's G5b applies the floor only when the selected p is 512, and G5a gains a shape check and a zero rule. G5c is unchanged. No existing K4, D1 or R1 control changes its outcome.

**The options, one line each** (§4; the sweep is 1,966 seeded random frames, and the control set is K4's survey plus DS1's F-2/F-3 set).

| Option | Result |
|---|---|
| (i-a) floor S\* at the verification's resolution, 2^(80−2p)·Ê | Honest F-2 at 128 (coarse bound); F-3 stays unresolved; changes nothing else. |
| (i-b) floor S\* at the candidate's resolution, 2^(74−p)·Ê, at every p | Honest and cheapest (−55 % work in the sweep). K4-M16's control stops discriminating, and 390 of 1,966 sweep models lose relative rows. |
| (i-g) the same floor, gated to kinds whose published scale is below 2^(10−p)·Ê | Honest; −44 % work; no K4 or R1 control changes (DS1's MIXED-2^-200 does). 230 sweep models lose relative rows, all in floored kinds, whose Ê/S\* > 2^118 at p = 128 by the gate's definition. |
| (ii) resolution-charged stop rule, V only | Honest; fixes F-2 by escalation; F-3 and the 1024-unresolvable tails stay unresolved; no D2 change. |
| **(iii) (ii) plus the floor at p = 512 only (recommended)** | Honest; 0 dishonest and 6 unresolved in the sweep (the current rule: 1 dishonest, 199 unresolved); no demotions; +1.4 % work; no control changes; the smallest D2 change that includes E. |

## 2. The failure class, derived

### 2.1 Notation

- q is a published force, moment, translation or rotation of a selected case.
- q\* is its exact value for the intended model (the binary64 primitives, exact arithmetic).
- q_P is the method's value at precision P, and e_P(q) = q_P − q\*.
- ε = 2^-64 and M_q = max(\|q_2p\|, S\*_2p(body(q), kind(q))).

The stop rule (§4.1.6) accepts p when \|q_p − q_2p\| ≤ ε·M_q for every q.

### 2.2 Lemma 1: what acceptance bounds (derived, exact)

If p is accepted, then for every q:
- \|e_p(q)\| ≤ ε·M_q + \|e_2p(q)\|;
- \|e_p(q)\| ≥ \|e_2p(q)\| − ε·M_q.

*Proof.* This is the triangle inequality on e_p = e_2p + (q_p − q_2p).

So the claim \|e_p(q)\| ≤ ε·M_q, which the classification publishes as b = fl↑(2^-64·S\*) for below-floor rows and as the relative 1e-9 for the others, holds only to the extent that \|e_2p(q)\| is negligible against ε·M_q. This is §4.1.6's own premise ("If q_2p is accurate well beyond 2^-64·S\*"). Nothing in r5a.2 enforces it.

### 2.3 Lemma 2: saturation (derived)

Let an intermediate x be formed exactly and rounded once, to nearest: at p bits in the candidate's computation and at 2p bits in the verification's. Write x = y + t, where y ≠ 0 is a p-bit number and 0 < \|t\| < ½·ulp_2p(y). Then fl_p(x) = fl_2p(x) = y.

*Proof.* y is representable at p and at 2p. Because ½·ulp_2p(y) ≤ ½·ulp_p(y), \|t\| lies strictly inside both half-ulps, so y is the unique nearest at both precisions.

Since ½·ulp_2p(y) ≤ 2^-2p·\|y\|, it suffices that \|t\| < 2^-2p·\|y\|/2. The rounding error is −t at both precisions: it is **identical**.

### 2.4 Theorem: the class, and by how much (derived to first order)

To first order in the unit roundoffs, e_P(q) = Σ_k g_k·δ_k^(P). Here the sum runs over the roundings k of the P-computation of q (formation, reduced right-hand side, factorization, substitution, refinement, recovery and publication), g_k = ∂q/∂x_k, and δ_k^(P) = fl_P(x_k) − x_k.

Split the roundings into the set S of those saturated in the sense of Lemma 2 and the rest.
- For k in S, δ_k^(p) = δ_k^(2p) = −t_k.
- So e_p − e_2p = Σ_{k∉S} g_k·(δ_k^(p) − δ_k^(2p)). **The stop rule observes only the non-saturated part.**
- The saturated part, E_sat(q) = Σ_{k∈S} g_k·δ_k, is common to both candidates, with \|E_sat(q)\| ≤ Δ_2p(q) := Σ_k \|g_k\|·½·ulp_2p(x_k) ≤ 2^-2p·Σ_k \|g_k\|·\|x_k\|.

With Lemma 1, an accepted candidate satisfies \|e_p(q)\| ≤ ε·M_q + Δ_2p(q) + O(2^-p)·(ε·M_q + Δ_2p(q)). The last term is the non-saturated error at 2p, about 2^-p times its value at p.

**Therefore:**
- the stop rule can accept a wrong value **only if** Δ_2p(q) is not negligible against ε·M_q, that is, M_q ≲ 2^(64−2p)·Σ_k\|g_k\|\|x_k\| (2^-192 of the formation scale at p = 128);
- **by how much:** the excess over the claimed bound is at most Δ_2p(q) (first order).

S\* in r5a.2 is formed from computed values only (`scales_at`, `classify_rows`), and nothing ties it to Σ\|g\|\|x\|. So S\* can be 0, or far below the formation scale, while the truth is not resolved at 2p.

### 2.5 Instances

The emulator's claim ratio is \|q_pub − q\*\| divided by the published claim: b for an absolute row, 1e-9·\|q\*\| for a relative row. A ratio above 1 is a false claim. The table below is measured with DS1's emulator; `run_controls.stdout.json` holds the records.

| Case | Mechanism | Current rule |
|---|---|---|
| **F-2** (I12). One member, E = 1024, EA/L = 512; ux(0) = 1 prescribed; load 2^-300 at node 1 | The reduced right-hand side 512 + 2^-300 and u_1 = 1 + 2^-309 are both saturated at 128 and 256. N\* = 2^-300 | Selected at 128; N = 0 labelled `absolute_verified` with b = 0; claim ratio ∞ |
| **F-2 with S\* > 0** (DS1). Member 1 has EA/L = 2^199 with F-2's pattern and a load of 2^-60 (u_1 = 1 + 2^-259); member 2 in the same body carries 1 N | S\*(force) = 1, so b = 2^-64, but N1\* = 2^-60 is lost at 128 and 256 | Selected at 128; claim ratio **16** (the error is 2^-60 against b = 2^-64) |
| **Prescribed tail** (ROOT's A2, point 1). A combined prescription 1 + 2^-1100 beside an exact 1 | The prescription rounded once at P loses its tail at every P ≤ 1024. N\* = 2^-1091 | Selected at 128 with b = 0; claim ratio ∞ (also its free-node variant) |
| **F-2 at the ceiling** (DS1). EA/L = 2^40; load 2^-1000 (u_1 = 1 + 2^-1040) | Saturated at every P ≤ 1024 | Selected at 128 with b = 0; ratio ∞ |
| **F-3** (I12, RIGID-UNLOADED). An unloaded body moved rigidly by prescribed values | Leakage of about 2^-P·E at every P; S\*_2p is the leakage at 2p | Unresolved at the ceiling (safe) |
| **F-3 with free nodes**: a (3,4,0) three-member run translated by a prescribed root; also a rigid rotation (F-3-FREE, F-3-ROT) | As F-3, through the solve | Unresolved |
| Random sweep, 1,966 frames (§10) | Rigid prescribed motions, tiny loads, stiffness contrasts | 1 false claim (R982, an F-2 pattern: a 2^-200 torque on a large prescribed translation); **199 unresolved** |

A sweep record flagged in an earlier emulator build was the reference's own noise at P = 2048 against an exact zero. The honesty check now allows the reference's resolution, 2^(8−2048)·Ê, as slack (`emu.honesty`).

**The combination class (ROOT's A2, point 2).** Since F-1's ruling a combination is its own solve, from the exact combined ledger and prescription. So it falls under exactly the same lemmas. A combination whose combined prescription is rigid (F-3), or whose combined prescription has a tail (the prescribed tail), behaves as the single cases above, whatever its operands' outcomes. The addendum applies to it unchanged, because its S\*, E and floor come from its own verification state (§4.1.6.1 item 8).

### 2.6 What was wrong in §4.1.9 (derived)

§4.1.9 says that error which depends on the precision is of order 2^-p at p and 2^-2p at 2p, so the stop rule sees it, and that only common-mode loss (identical at p and 2p) escapes. **The first half is false for saturated roundings.** Their error is precision-dependent (a larger precision resolves it) but identical at p and 2p (Lemma 2). Common-mode loss of an exact input, V1's check L, is the special case where t is an input tail.

Revision 2's exact sums remove the common-mode forms they targeted, and they remain necessary. They cannot remove saturation, which is inherent in a p-bit state: u, the reduced right-hand side, K's entries, the prescriptions of a combination and every recovery stage are rounded. The amended text is in §5.4.

## 3. The formation scale E (definition and derivation)

### 3.1 Definition

The **bounded recovery operator** replaces every operator entry by an upper bound of its magnitude that does not depend on the entry's computed value:
- each axis component \|e_a,c\| is replaced by 1;
- each \|B\| entry is replaced by 1 for axial, twist and rotation columns, and by 1/L for the translation columns of the bending rows;
- \|D\| and \|1/L\| are used as they are.

This matters for correctness. A local-axis component that is 0 in truth but formed as rounding noise (y_ref with a chord component) has an error as large as its own value, so entrywise absolute values do not bound its error. They measurably failed to on DS1's GS-y345 probes (§8.2).

For every published force or moment quantity q of a selected case (or of a combination as its own solve), **E_q** is the value of q's recovery expansion with every operator replaced by the bounded operator, every operand by its absolute value, and every sum by the sum of absolute terms. It is evaluated at the verification precision 2p, from the verification state:

1. **End actions** (per member, `recover.rs` at A1 `:239`), where a node's 1-norm is \|u_x\| + \|u_y\| + \|u_z\| for translations (and the same for rotations):
   - Ē_d = the node's translation or rotation 1-norm, for every local component;
   - Ē_e0 = Ē_d,j + Ē_d,i (axial), and likewise for twist;
   - Ē_e2 = Ē_θ,i + (1/L)(Ē_tr,i + Ē_tr,j) (and e3, e4, e5 alike);
   - Ē_Q = \|D\|·Ē_e (for example Ē_Q2 = 4·(EI_z/L)·Ē_e2 + 2·(EI_z/L)·Ē_e3);
   - Ē_V = (1/L)(Ē_Qa + Ē_Qb);
   - all of it multiplied by the member's Gram–Schmidt factor g_m = 2^⌈log2(\|y_ref\|/\|y_⊥\|)⌉ ≥ 1, which is exact to apply.
2. **Station actions:** \|t\|·Ē_Q,j + \|t\|·Ē_Q,i + Ē_Q,i for the moments; the end-j forces as in item 1.
3. **Spring actions:** \|k\|·\|u\|. Directional springs: Σ_b \|k_ab\|·\|u_b\|.
4. **Reactions** (`recover.rs` `:403`): Ē_R,c = \|f_c\| + Σ_j Ā_cj·\|u_j\|.
   - f_c is the **exact ledger net**, never Σ\|ledger terms\|. The ledger is exact, so its terms carry no resolution error. Σ\|terms\| would inflate E wherever cancelling contributions act at a restrained DOF: DS1's LEDGER-AT-RESTRAINT, (1e80, 1e-8, −1e80) at a restrained DOF, escalates from 128 to 256 under it (§7, mutant M6). Loads at free DOFs do not enter E at all, since E is a recovery scale.
   - Ā = Σ_e g_e·B̄_eᵀ\|D_e\|B̄_e is the assembled bounded operator, plus \|k\| of springs at c.
5. **Support-group magnitudes:** the sum of their components' Ē_q.
6. **u includes the prescribed values as rounded at 2p.** A single case's values are exact binary64; a combination's exact combined prescription is rounded once, and its tail is ROOT's A2 point 1.

Then:
- E(body, kind) = max over the body's rows of that kind that are not `input_derived` of Ē_q, rounded upward once to binary64 for publication.
- **Coupled through L_b as S\* is:** Ê_fo = max(E_fo, E_mo/L_b), Ê_mo = max(E_mo, L_b·E_fo). A single-node body (L_b = 0) is uncoupled.

### 3.2 Why the coupling is required (measured)

Without the coupling, the realized ratio \|q_P − q\*\|/(2^-P·E) reaches **1,217** (seed 3, frame R115 at P = 256) and 750 (seed 4) with the bounded E. With an earlier entrywise E and no coupling, it reached 1,570 on F-3-FREE's moments (`diag.py`) and 3,879 in the sweep. Either way, that is above λ.

The cause is the same one §4.1.6 gives for S\*'s coupling. Formation leakage in the stiff axial terms is a force field, and the structure carries it as moments through lever arms up to L_b.

With the coupled Ê, the realized ratio is ≤ **2.73** over K4's survey, DS1's controls and 550 random frames: **2,287 states**, at every P from 128 to 1,024 (`measure_lambda_{3,4}.json`). That ratio includes the conditioned (solve) part of the error.

### 3.3 λ = 2^8: the first-order rounding count (derived)

In units of u_P = 2^-P, relative to the bounded chain value, and first order, the stages give the following. Every sum is one exact expansion rounded once, as K4 forms it.

| Stage | Bound |
|---|---|
| e_x = d/L (d, dot, √, ÷) | 4.5 |
| e_y via y_⊥ = y − (y·e_x)e_x, normalized | ≤ 20g + 4 |
| e_z = e_x × e_y, normalized | ≤ 40g + 20 |
| 1/L | 3.5 |
| EA/L, GJ/L, EI/L | 4.5 each |
| B entries (1/L·e) | ≤ 40g + 24.5 |
| d = T·u, with the representation of u (1) | ≤ 40g + 22 |
| e | ≤ 40g + 26.5 |
| Q | ≤ 40g + 32 |
| V | ≤ 40g + 36.5 |
| **End actions total** | **≤ 77 at g = 1** |
| DB | ≤ 40g + 30 |
| ke | ≤ 80g + 55.5 |
| Assembly (+1), the representation of u (+1), the final rounding (+1) | |
| **Reactions total** | **≤ 80g + 59 ≤ 139g** |

Because g is folded into E, the count is ≤ 139 for every row, so **λ = 2^8 bounds the verification's resolution error: \|q_2p − R\*(u_2p)\| + \|a_q\|ᵀ\|δu_rep\| ≤ λ·2^-2p·E_q.**

- R\* is the exact recovery, and δu_rep is u's own rounding.
- "Derived" here means first order: γ_n = n·u/(1 − n·u) is taken as n·u, and every n here is ≤ 139 ≪ 2^120.

**The element's rigid-mode leakage** is the special case of the ke row with u a rigid motion r: \|K_e,P·r\|_a = \|Σ_b δK_ab·r_b\| ≤ (80g + 55.5)·2^-P·(B̄ᵀ\|D\|B̄\|r\|)_a. K4 measured it at ≤ 1.38·2^-P against the entrywise scale (A1 survey: M-236 at 512). F-3's leakage is therefore inside λ by the count.

**What λ does not cover (argued, measured).** The solve's backward error propagated through K^-1: the rounded right-hand side, K's formation, and the residual gate's tolerance 64·γ_p(m)·w. This is the conditioned part.
- It is argued to be covered by Ê for force and moment rows. A backward error is a nodal force (or moment) field of size ≤ c·2^-P·Ē, and forces respond to force perturbations through load paths whose gains are statics, and lever arms, which the coupling carries, not compliances.
- Measured, the whole error, conditioned part included, is ≤ 2.73·2^-P·Ê, against λ = 256.
- Soft modes do not break this: the soft-mode amplification a/k is cancelled by the soft stiffness k in the soft-path action. In SKEW-K1E-60 (k = 1e-60), the whole error at 256, 512 and 1024 is ≤ 0.005·2^-P·Ê (measured).
- **The claim that it holds generally is a conjecture** (§8.1).

**Translation and rotation need no V and no floor (argued).** For them E would be \|u\|, which is ≤ S\* by construction, so V ≤ 2^(8−2p)·S\* is negligible. Their conditioned saturation is bounded by S\* ≥ max\|u\| and the rcond screen, κ < F·2^(p−1): about 2^-(p+1)·F·S\*, which is ≪ ε·S\* for any estimator factor F < 2^60. No sweep model showed a false claim on a displacement row.

## 4. Options, each derived

Each option is stated as a rule. For each: the constant, the guarantee, F-2 and F-3, the cost and the D2 fields. The costs come from the 1,966-model sweep (`sweep_summary.json`, `sweep_quality.txt`). There, work is Σ(p/128)² over the solves actually performed, a crude limb-multiply proxy; the current rule does 5,395 solves and 50,950 work units.

### 4.1 (i-a): floor at the verification's resolution

**Rule.** S\*(fo/mo) := max(S\*_c, Φ), with Φ = 2^(80−2p)·Ê. The exponent is 64 + 8 (λ) + 8 (margin m).

**Guarantee (derived).** The verification's resolution error is ≤ 2^(8−2p)·Ê ≤ 2^-72·S\*, so \|e_p\| ≤ ε·M·(1 + 2^-8) plus the conditioned part.
- The relative 1e-9 claim holds: 2^-30·(1 + 2^-8) + 6u ≈ 9.35e-10.
- b = fl↑(2^-64·S\*) is short by up to 2^-8 relative. To be a bound, b would need a factor (1 + 2^-8), which changes G5c's pinned b.

**F-2:** honest at 128 with a coarse bound (F-2: b ≈ 2^(16−256)·Ê). **F-3:** unresolved; the candidate's leakage 2^-p·Ê ≫ ε·Φ. **Cost:** none. **Controls:** unchanged. **D2:** E published; G5b's floor at every p.

**Robustness.** Honesty rests on λ covering the conditioned part at 2p (measured margin 256/2.73 ≈ 94).

### 4.2 (i-b): floor at the candidate's resolution, at every p

**Rule.** S\*(fo/mo) := max(S\*_c, Φ_p), with Φ_p = 2^(74−p)·Ê.
- The constant needs ε·Φ_p ≥ \|q_p − q_2p\| + (verification error) ≤ λ·2^-p·Ê·(1 + 2^-p) + λ·2^-2p·Ê. With λ ≤ 2^8 that needs 2^(74−p) with a factor-4 margin, whose derivation is in §3.3. Against the measured 2.73 the margin is 375.
- This is the brief's "2^-(p−c)·E" with c = 74.

**Guarantee (derived).** e_2p ≤ 2^(8−2p)·Ê ≤ 2^(−66−p)·S\*, which is negligible.

**F-2, F-3:** selected at 128, honest. **Cost:** sweep 4,558 solves (−16 %) and 22,870 work (−55 %).

**But** it changes many outcomes:
- 650 of 1,966 models change p;
- 390 lose relative rows (4,457 rows);
- **REACTIONS-ONLY is accepted at 128, so K4-M16's control (SD-G4) stops discriminating** (log2(Ê/S\*) ≈ 76 > 128 − 74);
- MIXED-2^-200 and F-2-SPOS lose relative rows.

R1 would be unchanged (the largest log2(Ê/S\*) is 51.6 < 54), but by only 2.4 bits. **D2:** E published; the floor at every p.

### 4.3 (i-g): the candidate floor, gated

**Rule.** For force and moment, a kind is floored at candidate p iff its **published** coupled scale S\*_c,pub < t_p = 2^(10−p)·Ê. A floored kind takes S\* = 2^64·t_p = 2^(74−p)·Ê; any other kind keeps S\*_c. The gate is recomputable by readers from published rows, E and L_b.

**Guarantee (derived).**
- A non-floored kind has S\* ≥ 2^(10−p)·Ê, so the verification's resolution error ≤ 2^(8−2p)·Ê ≤ 2^(62−p)·ε·S\*, which is 2^-66 at p = 128. That tolerates an unproven conditioned amplification up to 2^(p−62) times λ.
- A floored kind is claimed only to b = 2^(10−p)·Ê, which is ≥ 4 times the candidate's own λ-bound.

**Robustness: the strongest of the options.**

**F-2, F-3:** selected at 128, honest. **Cost:** 4,781 solves (−11 %) and 28,310 work (−44 %). **Controls:** unchanged, except DS1's MIXED-2^-200 (512 → 128).

**Quality:** a kind whose real scale is below 2^(10−p)·Ê is floored at p instead of resolved by escalation.
- Sweep: 233 models are selected earlier than today, and 230 of them lose relative rows (2,583 rows).
- A demotion arises in a floored kind, which by the gate has Ê/S\*_pub > 2^(p−10) (2^118 at p = 128). R1's largest Ê/S\* is 2^51.6, so no R1 kind is floored.
- F-2-SPOS's real 1 N force becomes `absolute_verified` with b ≈ 3e25 N. That is honest and useless, because body-level E poisons it.

**D2:** E published; the gate and the floor in G5b at every p.

### 4.4 (ii): the resolution-charged stop rule

**Rule.** Accept p iff, for every q, \|q_p − q_2p\| + V_q ≤ ε·M_q. Here V_q = 2^(8−2p)·Ê(body, kind) for force and moment, and V_q = 0 for translation and rotation. S\* is unchanged.

**Guarantee (derived).** \|e_p\| ≤ \|q_p − q_2p\| + \|e_2p\| ≤ ε·M_q − V_q + \|e_2p\| ≤ ε·M_q whenever \|e_2p\| ≤ V_q, which holds for the resolution part by §3.3. So b and the relative claim hold exactly, with no (1 + 2^-m) slack, for the resolution part.

**F-2:** rejected at 128 and at 256 (\|0 − 2^-300\| > ε·2^-300), and **accepted at 512 with N = 2^-300 `relative_verified`** (six relative rows instead of three). F-2-SPOS is likewise exact at 512.

**F-3, and the tails 1024 cannot resolve:** unresolved (safe). **Sweep:** 12 cases change, 0 false claims, 209 unresolved. **D2:** no change to S\*, G5b or G5c. The summary ratio's meaning becomes (\|Δ\| + V)/M, and G5a still checks ≤ 2^-64.

**Robustness.** For kinds with ε·M close to V, honesty rests on λ covering the conditioned part (margin about 94).

### 4.5 (iii) = (ii) plus the floor at the last candidate (recommended)

**Rule.** (ii) at every candidate. **At p = 512 only,** S\*(fo/mo) := max(S\*_c, Φ), with Φ = fl↑(2^(74−512)·Ê) = fl↑(2^-438·Ê) (bits `0x2490000000000000`).

**Guarantee (derived).** As (ii). At 512 the floor makes acceptance possible when \|q_512 − q_1024\| + V ≤ ε·Φ = 2^(10−512)·Ê. That holds for leakage and tails because \|q_512 − q_1024\| ≤ λ·2^-512·Ê·(1 + 2^-512) ≤ 2^(8−512)·Ê·(1 + tiny) (derived), with a factor-4 margin and measured 2.73. The claim is b = fl↑(2^-64·Φ) = 2^-502·Ê.

**F-2, F-2-SPOS:** exact at 512. **Prescribed tail, F-2 at the ceiling:** selected at 512, honest (b ≥ truth). **F-3, F-3-FREE, F-3-ROT, RIGID-UNLOADED:** selected at 512, honest.

**Cost** (sweep; derived from the rule for the classes):
- 5,411 solves (+0.3 %) and 51,686 work (+1.4 %);
- 193 unresolved cases become selected at 512, with the same four solves they already spend failing;
- 11 cases are selected later than today:
  - 10 of them were honest at 128 or 256. Nine are unloaded axis-aligned members moved rigidly by prescribed values, whose forces the current rule computed exactly or within its claim; (iii) cannot tell them from F-2, so it escalates them to 512. The tenth (R953) has a 2^-200 load on a 1,000 m prescribed translation, which (iii) resolves at 256 instead of bounding at 128;
  - 1 is R982, which was dishonest;
- **no case is selected earlier, and no relative row is lost.**
- Realistic F-3 (a skew or sloped run translated or rotated rigidly) is unresolved today after four solves, and is selected after the same four solves under (iii). The only new cost falls on rigid motions that the current rule happens to accept, such as exactly computed zeros on axis-aligned members. They move from 2 solves to 4.

**Controls:** unchanged (§6.1). **D2:** E published. G5b adds the floor only when the selected p = 512; G5a gains a shape check and a zero rule (§6.3).

**Robustness:** as (ii). For p = 512, floored kinds carry the (i-g)-type margin.

### 4.6 Other combinations considered

**(iii-g).** V plus a gated floor at every p, with the gate decided on the verification's values (ε·S\*_2p < V_2p). It is cheaper than (iii) (−38 % work), but:
- the gate is not recomputable from published values, so it needs a published flag;
- body-level E poisons mixed bodies, and 76 sweep models lose relative rows.

**Not recommended.**

### 4.7 The comparison

| | False claims (sweep) | Unresolved (sweep) | Controls changed | Models losing relative rows | Work vs today | D2 change | Honesty rests on |
|---|---|---|---|---|---|---|---|
| Today | 1 (plus 5 in the F-2 set) | 199 | — | — | — | — | the unenforced premise |
| (i-a) | not swept (controls: 0) | not swept (F-3 unresolved, derived) | 0 | not swept | about 0 (derived) | floor at every p; b factor | λ at 2p |
| (i-b) | 0 | 5 | K4-M16, MIXED, F-2-SPOS | 390 | −55 % | floor at every p | margin 2^66 |
| (i-g) | 0 | 5 | MIXED | 230 (all Ê/S\* > 2^118) | −44 % | gate and floor at every p | margin 2^66 |
| (ii) | 0 | 209 | 0 | 0 | +1.4 % | none | λ at 2p |
| **(iii)** | **0** | **6** | **0** | **0** | **+1.4 %** | **floor at 512; E; G5a** | λ at 2p |

**Why (iii) over (i-g).**
- (iii) changes no outcome that is correct today, except to escalate.
- It never demotes.
- It fixes every wrong outcome in the control set and the sweep, and makes F-3 available.
- Its D2 change is the smallest that carries E.

(i-g) is the documented alternative if K6 and V-K show F-3-class cases are frequent and a 1024-bit verification is too costly. Its honesty margin is also larger. Moving from (iii) to (i-g) later changes only S\*'s formula in G5b and the stop rule's S\*, and the receipt's E field is the same.

## 5. Recommendation: the exact amended text (addendum blocks)

`DESIGN.md` stays hash-pinned. These blocks amend it when ROOT selects them. Section numbers are D1's.

### 5.1 §4.1.6 Stop rule: replace the "Accept p" bullet, and add two bullets after "The scale S\*"

> - **Accept p** (revision 5a.3) when every published quantity q satisfies `|q_p − q_2p| + V_q ≤ 2^-64 · max(|q_2p|, S*)`, decided exactly. **V_q = 2^(8−2p)·Ê(body, kind)** for q of kind force or moment (§4.1.6.2), lifted exactly from its published binary64 bits; V_q = 0 for translation and rotation. V_q is the verification's own first-order resolution bound (λ = 2^8, §4.1.6.2): the rule charges the 2p candidate's possible error to the p candidate.
> - **The ceiling floor (revision 5a.3).** At the last candidate, p = 512 (verified at 1024), the force and moment scales are floored: `S*(kind) := max(S*(kind), Φ)`, with `Φ = fl↑(2^-438 · Ê(body, kind))`, in the stop rule and in the classification (§4.1.6.1 item 6a). At p = 128 and 256 there is no floor, and a kind whose scale lies below its verification's resolution escalates.
> - **Revision 5a.3's reason.** A rounding whose discarded tail lies below both p's and 2p's rounding units is identical in both candidates (§4.1.9). Without V, such a loss passed with a false claim (F-2: an exact 0 labelled bound 0 against a truth of 2^-300). Without the floor, an unloaded body moved rigidly (F-3) could not be accepted at any precision.

### 5.2 §4.1.6 "What acceptance guarantees": replace its first bullet

> - **What acceptance bounds (revision 5a.3, derived).** If p is accepted, then for every q, |q_p − q\*| ≤ |q_p − q_2p| + |q_2p − q\*|. The rule's V_q bounds the second term's **resolution** part: the rounding of the 2p recovery chain, of its operators, and of u_2p itself, by §4.1.6.2's count (≤ 139 units of 2^-2p·Ê against λ = 2^8). So |q_p − q\*| ≤ 2^-64·max(|q_2p|, S\*) exactly for that part, and b = fl↑(2^-64·S\*) is a bound for it. The remaining part of |q_2p − q\*| is the solve's backward error propagated through K^-1. It is argued to lie within V_q as well (§4.1.9), and it was measured at ≤ 2.73·2^-2p·Ê in every emulated case. It is not proved. For a kind floored at p = 512, the claim is b = 2^-502·Ê.

The floor bullets that follow (R = 2^-34 and classification on the published value) are unchanged.

### 5.3 §4.1.6.1: add item 6a after item 6, and add §4.1.6.2

> 6a. **The ceiling floor (revision 5a.3).**
> - Per body, the receipt carries `resolution_scale`: E_fo and E_mo as bit strings, finite and ≥ +0 (§4.1.6.2).
> - In binary64, in this order: `ê_fo = max(E_fo, fl(E_mo/L_b))`; `ê_mo = max(E_mo, fl(L_b·E_fo))`. With L_b = 0, ê = E.
> - **Only if the case's selected precision is 512:** `fo := max(fo, fl↑(2^-438·ê_fo))` and `mo := max(mo, fl↑(2^-438·ê_mo))`. Here fl↑ is decided exactly as b is: r = ê·2^-438 to nearest, and next_up(r) when r·2^438 < ê. So Φ is exact whenever ê ≥ 2^-584, and Φ > 0 whenever ê > 0.
> - 2^-438 has bits `0x2490000000000000`.
> - Items 7 and 8 use the updated fo and mo, so the stress, twist and extension scales inherit the floor.
> - For p = 128 or 256, item 6a changes nothing.

> #### 4.1.6.2 The formation scale E (revision 5a.3)
>
> 1. **Definition.** For each published force or moment quantity q, E_q is q's recovery expansion evaluated with the **bounded operator** and absolute operands:
>    - axis components → 1;
>    - bending-row translation entries of B → 1/L;
>    - other B entries → 1;
>    - |D| and |1/L| as they are;
>    - operands |u| at every DOF of the member's nodes, prescribed values included as rounded at the precision;
>    - every sum → the sum of absolute terms.
>
>    Two further rules:
>    - member quantities are multiplied by g_m = 2^⌈log2(|y_ref|/|y_⊥|)⌉;
>    - reactions use |f_c| + Σ_j Ā_cj·|u_j|, with the **exact ledger net** f_c, never Σ|terms|, and Ā = Σ_e g_e·B̄ᵀ|D|B̄ plus the |k| of springs at c.
>
>    Spring actions: |k|·|u|. Directional: Σ|k_ab|·|u_b|. Support magnitudes: the sum of their components' E.
> 2. **Computation.** E is computed at the verification precision 2p, from the verification state. Each stage is one exact sum of exact products rounded once at 2p. **E(body, kind)** is the maximum over the body's non-`input_derived` rows of that kind, rounded upward once to binary64 for publication (`resolution_scale`), and that binary64 value is what V and Φ use. If it overflows binary64, the case's receipt entry cannot be encoded, and it is `unavailable` (`receipt_encoding`, §5 item 2), never flushed.
> 3. **Coupling.** `Ê_fo = max(E_fo, E_mo/L_b)`, `Ê_mo = max(E_mo, L_b·E_fo)`, as S\* is coupled (item 6a gives the binary64 form).
> 4. **The resolution bound (derived to first order).** |q_P − R\*(u_P)| + |a_q|ᵀ|δu_rep| ≤ 139·2^-P·E_q, by the count of roundings in formation (axes ≤ 40g + 20; B ≤ 40g + 24.5; D ≤ 4.5), assembly (1), recovery (≤ 40g + 36.5 for end actions; ≤ 80g + 59 for reactions) and the representation of u (1). The bounded operator is what makes the count hold when an axis component is formed as noise. **λ = 2^8.**
> 5. **Translation and rotation have no E.** Their S\* is at least max|u| of the body, which exceeds their own resolution by construction. Their conditioned part is covered by the rcond screen as before.
> 6. **Combinations** (their own solves since F-1) use their own verification state.

### 5.4 §4.1.9: replace the section

> #### 4.1.9 What the stop rule can and cannot see (revision 2; corrected in revision 5a.3)
> - **The stop rule observes only the part of the error that differs between p and 2p.**
> - **Saturation (revision 5a.3, derived).** A value x = y + t rounded once, with y a p-bit number and 0 < |t| < ½·ulp_2p(y) (so |t| < 2^-2p·|y|), rounds to y at p and at 2p. Its tail is lost identically in both candidates. The error is precision-dependent, since a larger precision resolves it, but the stop rule cannot see it. Every published q carries such a common component of at most Δ_2p(q) = Σ_k |∂q/∂x_k|·½·ulp_2p(x_k) (first order). An accepted candidate's error can exceed its claimed 2^-64·M_q by up to that amount. The sites are u itself, the reduced right-hand side, K's entries, a combination's rounded prescription, and every recovery stage.
> - **Revision 2's claim that only precision-independent (common-mode) error can pass is withdrawn.** Common-mode loss of an exact input (V1's check L) is the special case where t is an input tail. Revision 2's exact sums still remove the common-mode forms they targeted (loads, stiffness entries, the reduced right-hand side, recovery sums, combinations), and they stay required. They cannot remove saturation, which is inherent in a p-bit state.
> - **What revision 5a.3 adds.**
>   - The rule charges the verification's resolution bound V_q = 2^(8−2p)·Ê to every force and moment comparison (§4.1.6). A value lost below 2p's resolution then escalates until it is resolved, or, at p = 512, until the ceiling floor bounds it honestly.
>   - The negative controls F-2, F-2 with S\* > 0, the prescribed tail, F-2 at the ceiling, and F-3 (fully prescribed, free-node translation, free-node rotation) join §7.3.
> - **What remains an argument.** The solve's backward error (the rounded right-hand side, K's formation, the residual gate's tolerance), propagated through K^-1 into forces and moments.
>   - It is argued to lie within V_q: a force imbalance reaches forces through statics and lever arms, which the L_b coupling of Ê carries.
>   - It is measured at ≤ 2.73·2^-2p·Ê against λ = 2^8.
>   - It is not proved.
>   - Displacements stay covered by S\* ≥ max|u| and the rcond screen, as before.
> - **In the factorization and triangular solves,** non-saturated error changes at 2p and the stop rule sees it. The refinement residual at p + 64, formed against the exact-expansion right-hand side, checks the solve independently of the factor.
> - **This is an argument, not a proof.**
> - **The same exact-sum rule holds on the ordinary binary64 route** through S11-K and S11-F, as before.

### 5.5 §5 item 1 (the receipt): add to a `selected` entry

> - **`resolution_scale`** (revision 5a.3): per body, `force` and `moment` bit strings, E_fo and E_mo of §4.1.6.2. Uncoupled, rounded upward, finite, +0.0 only when zero.
> - **The stop-rule summary** is now the worst of (|q_p − q_2p| + V_q)/M_q, rounded upward, and it still decodes to ≤ 2^-64.

## 6. Effects downstream

### 6.1 The discriminating controls (§7.3, §4.10) under (iii)

(iii) changes a stop-rule decision only where \|Δ\| lies within V of ε·M (log2(V/(ε·M)) = 72 − 2p + log2(Ê/M)), or where p = 512 and Φ > S\*_c.

| Control | Kills | Under (iii) | Evidence |
|---|---|---|---|
| k = 1e-28 (SKEW-K1E-28): 128 fails the condition estimate, 256 accepted | D5, D6 | Unchanged | Emulated; at 256 the largest log2(V/(ε·M)) is −317 (`v_ratio.stdout.json`) |
| Six-member k = 1e-12 (SKEW6): 128 rejected on Rx, 256 accepted | K4-M14; SD-G1 | Unchanged | Emulated; V/(ε·M) ≤ 2^-108 |
| B1-L: exact with the ledger; the folded mutant accepted with error 0.5 | D13 | Unchanged. B1-L's loads act at a free DOF, which does not enter E, so the folded mutant is still accepted at 128 and wrong: a common-mode loss of an exact input, which the exact ledger removes and V is not meant to see | Emulated (128) |
| B1-C, B1-E: combinations as their own solves (F-1); nets 1e-8 and 1e-53 | D14, D15 as re-derived under F-1 | Unchanged; the nets are small ordinary loads | By construction |
| S8-W: far-node rows `absolute_verified` | D17, D20; SD-J2 | Unchanged (128; class counts identical) | Emulated |
| RF-CHAIN, RF-SKEW, RF-WEAK, RF-FINITE, RF-CANCEL | Their negative controls | Unchanged. All are selected at 128 in I12's run (`r1_run1.txt`). The largest log2(Ê/S\*) per family, from R1's own solutions, is 51.1, 51.6, 50.0, 13.6 and 10.1, so V/(ε·M) ≤ 2^(−184+51.6) = 2^-132 at 128, and Φ is never reached | `r1_ratios.stdout.json` |
| PIVOT: 128 fails a pivot, 256 accepted | D6 variants | Unchanged | Emulated |
| REACTIONS-ONLY: 128 rejected by a reaction | K4-M16; SD-G4 | Unchanged (V/(ε·M) ≤ 2^-108) | Emulated |
| ZERO-TORSION-345, ALL-ZERO-BODY, TWO-SPAN, DUPLICATE, PRESCRIBED, N05, N06, SKEW-K1E-4, AXIS, OBLIQUE, SKEW-K1E-12, SKEW-K1E-60 | Various | Unchanged (same p, same class counts; V/(ε·M) ≤ 2^-114 at every compared pair of these, and ≤ 2^-210 for SKEW-K1E-60) | Emulated |

**No control stops discriminating.** R1's not-covered set (RF-WEAK 46, RF-CANCEL 3, RF-SKEW 2) is unchanged, because no R1 case reaches 512 and V cannot flip a decision at 2^-132.

**One caution for K4:** E must use the ledger's **net**. With Σ\|ledger terms\|, cancelling contributions at a restrained DOF escalate a case that is exact at 128 (mutant M6 in §7). B1-L's and RF-CANCEL's loads act at free DOFs, which do not enter E, so they are unaffected either way.

### 6.2 K4's SD-tagged tests at A2 (ROOT's list), under (iii)

| Tag | Assertion | Under (iii) |
|---|---|---|
| SD-G1 | SKEW6 rejected at 128 by the stop rule; the 256 verification reused; SKEW-K1E-60's four attempts | **Unchanged** |
| SD-G2 | ZERO-TORSION-345 accepted at 128 through the coupled S\* | **Unchanged** (S\*(force) = S_mo/L_b = 1, so V/(ε·M) ≤ 2^-179) |
| SD-G3 | ALL-ZERO-BODY's unloaded body: exact +0.0, `AbsoluteVerified{bound 0}` | **Unchanged**: E = 0 for that body, so V = 0 and Φ = 0 |
| SD-G4 | REACTIONS-ONLY rejected at 128 by a reaction; selected 256 | **Unchanged** |
| SD-G5 | The predicate's exact boundary (a Translation quantity; q_2p = 1; 1 ± 2^-64 accepted, one ulp beyond rejected; K4-M14's \|q_p\| variant) | **Unchanged for these vectors**, since V = 0 for translation. `stop_rule`'s signature gains Ê per body and kind, so the test passes E = 0. **Add** force-kind boundary vectors: \|Δ\| = ε·M − V exactly accepted, and one 2p-ulp above rejected |
| SD-G6 | N06: perturbing any quantity by 2^-60 of M rejects at that quantity | **Unchanged** (V ≥ 0 only adds; the base acceptance holds with V/(ε·M) ≤ 2^-118) |
| SD-L1 | Golden work counts | **Changed:** the E pass at the verification precision, and V's exact additions, are charged. Re-pin, with the E pass as its own stage |
| SD-I1 | 2·SKEW6 − SKEW6 (its own solve) rejected at 128 | **Unchanged** (the same solve as SKEW6) |
| SD-I2 | SKEW6 − SKEW6: exact +0.0, selected at 128 | **Unchanged** (net ledger 0 and prescription 0, so u = 0, E = 0, V = 0) |
| SD-J1 | Seeded and targeted classification vectors: classes, bounds, body scales | **Values unchanged** for every vector whose selected p ≠ 512 or whose E is 0. The vector format gains (selected p, E_fo, E_mo). **Add** item-6a vectors at p = 512: Φ binding, Φ not binding, the fl↑ boundary, and ê < 2^-584 (subnormal Φ) |
| SD-J2 | S8-W's far-node rows `absolute_verified` | **Unchanged** |
| SD-K1 | The floor check's not-covered set equals §4.10's list | **Unchanged** (§6.1) |

### 6.3 D2 (r5b.2): published-row S\*, the classes and G5a–G5c

**Does E have to be published? Yes, per body, for force and moment.** Under (iii), G5b needs it only when the selected p = 512. It is published for every selected case so that the receipt shape is uniform, the replay audit can check it, and a later move to (i-g) needs no schema change.

**Fields.** `resolution_scale: [{body, force: "<16-hex>", moment: "<16-hex>"}]` in a `selected` entry, and likewise in a combination's own entry. The values are uncoupled E, rounded upward to binary64, finite, and +0.0 only for zero. The body order and ids are the same as the S\* list.

**Can readers recompute E? No.** E comes from the unpublished 2p state and the p-formed operators.
- It is producer-attested, covered by `receipt_sha256`, at the trust level the section terms have (§4.1.6.1 item 7), and checked by the Rust replay audit (§4.1.8), which reproduces u_2p and hence E.
- What readers can check:
  - the shape;
  - the **zero rule:** E_k = 0 ⇒ every published row of kind k in that body is +0.0. E_q = 0 forces q = 0 exactly, so this is derived;
  - a sanity bound, ê_k ≥ S\*_c,k·(1 − 2^-40), decided as fl(ê_k·(1 + 2^-40)) ≥ S\*_c,k. It holds for an honest producer because \|q\| ≤ E_q·(1 + λ·2^-P) (derived), and it catches an E under-reported to 0 or near it.

**The changes, by check.**

| Check | Change |
|---|---|
| **G5a** | `resolution_scale` present for every body with a force or moment row; the encoding rule (G2); the zero rule; the sanity bound. The summary check "≤ 2^-64" is unchanged; the summary's meaning now includes V. Mismatch code: `RETAINED_PRECISION_SCALE_MISMATCH` (existing) |
| **G5b** | Item 6a after item 6 and before item 7, only when the selected p is 512. Everything else is unchanged. The recomputed S\* must still equal the receipt's bits |
| **G5c** | Unchanged: it classifies against G5b's S\*. b = fl↑(2^-64·S\*) is unchanged in formula, and for floored kinds it equals 2^-502·ê |
| **§4.9.9 (the consumer rule), §4.11 (C)** | Unchanged in form. A floored kind's rows are `absolute_verified` with a tiny b, so under C they bind as [q − b, q + b]. **Note** that b is a first-order bound; see §8.3 |
| **§4.9.10 (the closed table)** | Unchanged |
| **Classification summary and the retirement gate (D1 §4.4.1)** | Row counts can change only for cases selected at 512 with a floored kind, or for a case unresolved today that becomes selected |

**Check-level condition 3 (R5-5).** A body moved rigidly whose forces are computed exactly as 0 is published today with b = 0 (a point) and under (iii) with b > 0 (an interval). A sign or equality check on such a row can move from decided to undecided, and R5-5 reports it.
- **Candidates:** T1's support-motion fixtures, if a body there is moved rigidly with no load. That is F3's gate for `load-reference-source-1`, where D2's C already applies.
- **Committed source-blocks-1 and physics-source-1 families:** none expected, since they carry nodal loads. The gate report shows it either way.

**VP-ROBUST (§4.10), the harness side.**
- For a case selected at 512, the harness applies item 6a, with the receipt's E, when it forms S\* for the zero-scale floor check and for twist and extension. Otherwise nothing changes.
- The enumerated not-covered list is unchanged (§6.1).
- The class correspondence check compares classes and is unaffected.

### 6.4 §4.1.6.1's pinned binary64 formulas

- Items 1–6 and 7–8 are unchanged.
- **Item 6a is added** (§5.3). Its constants are 2^-438 (`0x2490000000000000`) and, for G5a's sanity bound, 2^-40 (`0x3D70000000000000`).
- The order is: item 6 coupling → item 6a (only at p = 512) → item 7 stress scales.
- fl↑ is decided exactly, as b is, in all three languages: Rust `mul_add` sign or an exact scaling check; Python `Fraction`; TS BigInt.
- R = 2^-34, t = fl(R·S\*), the rule for S\* < 2^-988 and b are unchanged.

### 6.5 F2a's contract

F2a (atomic with D2's S-G1):
- maps K4's E into `resolution_scale` for each selected case and each combination entry;
- applies item 6a in the producer-side classification;
- keeps the product-derived rows (stresses, wall rows, span-statics rows) on fo and mo after item 6a, so their scales inherit the floor through item 7. E is formed from the kernel's quantities only; product rows derived from them add no cancellation (§4.1.6's R4-1 proof);
- maps an E that cannot be encoded to `unavailable` (`receipt_encoding`);
- updates the receipt schema, D2's G5a/G5b readers and parity files, and the shared case files, in the same atomic PR.

**F2a's merge gate** ("does not merge without this addendum implemented", ROOT) is unchanged.

The combination-as-solve rule (F-1) needs nothing further.

### 6.6 K4's ceiling argument (I12's plan §11: route 1, step 10; route 2)

Step 10 said: "the stop rule can accept a wrong 512 candidate only if the 1024 solve is wrong by nearly the same amount in the same direction, which requires a precision-independent (common-mode) error". **Restated:**

> which requires error identical at 512 and 1024: either a common-mode loss of an exact input (removed by the exact sums), or saturated rounding below 1024's resolution (Lemma 2). Under 5a.3 the second is charged by V = 2^(8−1024)·Ê: a kind whose ε·M exceeds \|Δ\| + V has verification resolution error ≤ V (derived). A kind that cannot meet it is floored by Φ = 2^-438·Ê, whose claim b = 2^-502·Ê bounds the 512 candidate's resolution error with a factor-4 margin.

Route 2's "no common-mode term" becomes: "no common-mode term outside saturation, and saturation is charged by V."

The uncertified step (the Hager–Higham estimate) is unchanged in both routes. The argued step is added: the conditioned backward error is within V, as §5.4 says.

### 6.7 Budgets and work

- The E pass costs about one recovery pass at 2p in absolute arithmetic, plus Ā's rows for reactions. It is charged to the verification attempt. The reuse of that verification as the next candidate reuses its E.
- V's addition is one exact add per force and moment row.
- The sweep's work proxy is +1.4 % for (iii).
- K4's golden work counts (SD-L1) are re-pinned.

## 7. Tests K4 must add (kernel; the generator's `Fraction` emulation as the oracle)

**Controls** (invented inputs, stated in DS1's `models.py`; expectations from the exact reference):

| Id | Model | Expected under (iii) |
|---|---|---|
| **F2** | I12's probe: (0,0,0)–(2,0,0), E = 1024, G = A = I = J = 1, y_ref (0,1,0); node 0 fully fixed with ux = 1; node 1 fixed in 1..5; load 2^-300 at node 1 ux | 128 rejected on a force row; 256 rejected; **512 accepted** vs 1024; \|N\| = \|R(0, Ux)\| = 2^-300, `relative_verified` |
| **F2-SPOS** | Member 1 E = 2^200 (unit section) with F-2's pattern and a load of 2^-60; member 2 (unit section, along y) to node 2, free in ux, uy and rz, loaded 1 N in uy | 512; N1 = 2^-60 `relative_verified`; member 2's 1 N `relative_verified`. **Today:** 128 with claim ratio 16 |
| **F2-CEIL** | F-2 with E = 2^41, load 2^-1000 | 512 via Φ; N = 0 `absolute_verified` with b = fl↑(2^-64·fl↑(2^-438·ê)) ≥ \|N\*\| |
| **PTAIL** | A combination whose combined prescription is 1 + 2^-1100 at ux of node 1 of F-2's member: operand A prescribes 1, operand B prescribes 2^-1000, with factors (1, 2^-100); both are normal binary64. Node 0 prescribed 1 | 512 via Φ; honest b; its free-node variant likewise |
| **F3** | RIGID-UNLOADED (K4's) | **512** (was unresolved). Body 0's force and moment kinds floored; reactions `absolute_verified` with b = 2^-502·ê; body 1 unaffected |
| **F3-FREE**, **F3-ROT** | A (3,4,0) three-member run whose root node is fully prescribed (ux = 1), or a two-member run whose root is prescribed rz = 1e-3; the rest free; unloaded | 512, honest |
| **LEDGER-AT-RESTRAINT** | B1-L's geometry (SKEW, k = 1e-4) with (1e80, 1e-8, −1e80) at node 0 Ux, which is restrained, and 2e-8 at node 1 RY | 128 (the E of mutant M6 moves it to 256) |
| **EXACT-RIGID** | An axis-aligned member with power-of-two length under a rigid translation (forces exactly 0 at every P) | 512 with b > 0. Documents the only new escalation |
| **E-UNIT** | Every survey model at 128–1024 | E's bits equal the generator's bounded-chain emulation; coupling; the g factor; prescribed \|u\| included; the ledger net |
| **E-HEADROOM** | Every survey model at every P with a state | \|q_P − q_2P\| ≤ 2^8·2^-P·Ê (the premise, observed ≤ 2.73), asserted as an observation with its margin |
| **SD-G5+** | Force-kind predicate boundary vectors with V | As §6.2 |
| **J-6a** | Classification vectors for item 6a at p = 512 | As §6.2 |

**Mutants** (each with its intended kill):

| # | Mutant | Kill |
|---|---|---|
| M1 | Drop V | F2, F2-CEIL and PTAIL accepted at 128 with b = 0 (the truth check fails); F2-SPOS at 128 with claim ratio 16 (emulated, `mutants.stdout.json`) |
| M2 | V and Φ from the uncoupled E | **Not killed by these controls with the bounded E** (emulated: F3-FREE's worst claim ratio rises from 0.001 to 0.62, still honest). Killed by E-UNIT's coupling bits. A behavioural control can be taken from the frame the emulator's seed 3 generates as R115, whose uncoupled ratio is 1,217, above λ |
| M3 | Φ at every p (option i-b) | REACTIONS-ONLY selected at 128 (SD-G4); F2 selected at 128 instead of 512 (emulated) |
| M4 | No Φ | F3, F3-FREE, F3-ROT, F2-CEIL and PTAIL unresolved (emulated) |
| M5 | E without the prescribed \|u\| | PTAIL accepted at 128 with b = 0 (the truth check fails); RIGID-UNLOADED unresolved (emulated) |
| M6 | E from Σ\|ledger terms\| | A new control, LEDGER-AT-RESTRAINT: B1-L's geometry with (1e80, 1e-8, −1e80) at a restrained DOF (node 0, Ux), exact at 128, moves to 256 (emulated). RF-CANCEL and B1-L do not kill it: their loads act at free DOFs |
| M7 | E from the entrywise \|T\|, \|B\| | **Not killed behaviourally** in DS1's emulation, because body-level Ê masked it. Killed by E-UNIT's bits |
| M8 | Φ with 2^(74−2p) (the verification resolution) | F3, F3-FREE and F3-ROT unresolved (emulated) |
| M9 | The floor applied at 256 (2^(74−256)) instead of 512 | F2 and F2-SPOS selected at 256 instead of 512, and F3 at 256 (emulated) |
| M10 | Omitting g | Killed only by E-UNIT. DS1 measured no g-dependence of the leakage for isotropic sections (§8.4) |

## 8. Where the design needs more than an S\* amendment

1. **The conditioned part is argued, not proved (conjecture).** "The solve's backward error, propagated through K^-1 into forces and moments, stays within λ·2^-2p·Ê" is a conjecture. Its support:
   - the statics argument;
   - measured ≤ 2.73 on 2,287 states;
   - my attempts to construct a counterexample. In each, the formation scale E tracked the amplification: a shallow two-bar geometry, where E scales with the same angle as the force gain; a soft mode, where a/k cancels against k; and a large prescribed coupling on a soft-mode row, which is impossible because that row is stiffly restrained.

   A design that makes it an estimate instead of an argument is cheap. At the verification precision, solve one more correction, δ = K̃⁻¹·fl(r_2p), from the exact residual that K4 already forms (`residual_rows`, `:799`), and add \|recover(δ)\| (the linear part) to V. That is a first-order a posteriori estimate of q_2p's error, including the saturated right-hand-side and formation tails, at the cost of one triangular-solve pair and one recovery pass. **Proposed, not emulated.** ROOT may prefer it to the conjecture, or choose (i-g), whose margin (2^66 at p = 128) makes the conjecture irrelevant for kinds that are not floored.
2. **K4's residual gate can refuse a rigid motion at every precision (measured; availability only).** The gate's denominator d_i = \|f_i\| + Σ\|K^q_ij·u_j\| (plan §8.5 step 3; D1 §4.1.4, "the componentwise guarded ratio") does not include the formation noise of entries that are zero in truth. DS1's emulator shows this with a y_ref that has a chord component, which gives a noise-formed axis component:
   - GS-ROT-y345 and GS-TRANS-y345 fail the gate at 128, 256 and 512;
   - so does **GS-ROT-y345-LOADED**, the same with a 1 N tip load, which the emulation leaves unresolved under K4's gate for that reason alone;
   - with the bounded operator Ā in the denominator (\|f\| + ΣĀ\|u\|), all four pass: the loaded one is selected at 128, and (iii) selects the unloaded ones at 512;
   - **no survey control changes** (`gate_probe.stdout.json`);
   - realistic sloped runs with y_ref (0,0,1) pass either way (`gate_probe2.stdout.json`).

   This is a residual-gate change (§4.1.4), outside this addendum. It is recorded as a finding for K4 and ROOT. It is **not checked against K4's Rust.**
3. **b is a first-order bound.** Every option's b rests on first-order resolution counts, and on the conditioned argument for the rest. D2's C (§4.11.4, "no straddling result can pass") therefore inherits first-order soundness, not a rigorous enclosure. This was already so in r5a.2 and is not introduced here. Revision 5a.3 makes it explicit (§5.2).
4. **g covers anisotropic sections only in theory (measured).** For isotropic sections, a Gram–Schmidt error rotates the local frame about the member axis and leaves the element objective. The leakage did not grow with g up to 2^20 (`gs_probe.stdout.json`). The factor is kept because the count needs it for Iy ≠ Iz.
5. **A per-body schedule would localize the cost.** Bodies are independent systems, yet the case escalates as a whole: an F-3 body forces a 1024-bit verification of every body in its case under (iii). Selecting p per body would need per-body receipt entries. **Not proposed now.**
6. **Input-derived rows of a combination (a side note, not verified).** A combination's prescribed displacement row is `input_derived`, and its published value should be the exact combination rounded once to binary64, not a p-rounded value rounded again (double rounding). I did not check A2's code for this.

## 9. What I could not resolve

- **The general bound for the conditioned part** (§8.1): a conjecture, with its evidence and a proposed estimator.
- **λ is a first-order count, not a verified constant.** It should be confirmed by K4's E-UNIT and E-HEADROOM tests on the Rust implementation. My emulator mirrors K4's roundings as I read them, and it is not K4.
- **The realistic frequency of F-3-class cases, and the cost of a 1024-bit verification on large models,** are K6's and V-K's to measure. The choice between (iii) and (i-g) may turn on them.
- **Whether committed T1 support-motion fixtures contain rigidly moved, unloaded bodies,** which would give condition 3's check-level movements at F3. Not surveyed.
- **The residual-gate finding** (§8.2) is emulation-only.

## 10. Evidence (scratch, standard-library Python 3.13; placeholders only)

Everything is under `<wt>/scratch/ds1/`.

| File | What it is | sha256 (first 16) |
|---|---|---|
| `emu.py` | The emulator; header lists its simplifications | `db5caacd2bdd80c1` |
| `models.py` | K4's survey models re-declared, plus the F-2/F-3/PTAIL set | `78e9ee8d1f981a97` |
| `run_controls.py` → `run_controls.stdout.json`, `controls_table.txt` | Controls × options, with the honesty check | `3d2ae195050aadd1` (script), `c1c63ac488094ed3` (json) |
| `diag.py` | Per-kind noise ratios, coupled and uncoupled | `1953108412c59ca3` |
| `measure_lambda.py` → `measure_lambda_{3,4}.json` | The realized \|q_P − q\*\|/(2^-P·Ê) | `99f0607030c51607`, `40d3cae714e39ab5` |
| `r1_ratios.py` → `r1_ratios.stdout.json` | R1's log2(Ê/S\*) per family | `5cc7cbc98f5df736` (script), `8718e60ff4fccfdb` (json) |
| `sweep.py`, `summarize.py` → `sweep_{11,12}.json`, `sweep_summary.json`, `sweep_quality.txt` | 1,966 seeded random frames | `3d144d740f6c0f66` (summary) |
| `gate_probe.py`, `gate_probe2.py` → `*.stdout.json` | §8.2 | `d3bb8ce3ae4d0d00`, `ddf5405e31e16d17` |
| `gs_probe.py` → `gs_probe.stdout.json` | §8.4 | `35e16f27d531329a` |
| `mutants.py` → `mutants.stdout.json` | §7's mutants M1–M6, M8, M9, emulated | `a310b59a0b40389d` |
| `v_ratio.py` → `v_ratio.stdout.json` | log2(V/(ε·M)) per control and pair (§6.1, §6.2) | `7fe5f27769bad500` |

**To rerun:**
- `python3 run_controls.py`;
- `python3 sweep.py 11 1000; python3 sweep.py 12 1000; python3 summarize.py sweep_11.json sweep_12.json`;
- `python3 measure_lambda.py 3 150; python3 measure_lambda.py 4 400`;
- `python3 mutants.py; python3 v_ratio.py; python3 gate_probe.py; python3 gate_probe2.py; python3 gs_probe.py`;
- `python3 r1_ratios.py <T3 path>`.

`emu.py` gained its mutant switches (`E_PRESCRIBED`, `E_LEDGER`) and an overflow guard in `honesty` after the sweep ran. Their defaults reproduce the sweep's code path.

**Constants used:** λ = 2^8, V = 2^(8−2p)·Ê, Φ_512 = 2^-438·Ê; for the alternatives, the gate 2^(10−p)·Ê and (i-a)'s 2^(80−2p)·Ê.

**Delegation.** DS1 ran as a Claude Code background subagent launched by ROOT through the Agent tool. ROOT is its only return path, and DS1 dispatched nothing.
