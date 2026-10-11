# K-D5 member-relative bound and the unified R-b′ predicate (T4-KD5)

This record gives the derivation and evidence for HELP_HUMAN's ruling of 2026-10-10. The ruling is a correctness fix for main: the integrity system must not publish `NUMERICAL_INTEGRITY_CHECKS_PASSED` for a case whose published member rows breach the 1e-9 row-relative criterion. The record extends K-D5 ([curved-numerical-integrity.md](curved-numerical-integrity.md)) and R-b′/O-13 ([arc-recovery-guard.md](arc-recovery-guard.md)). It changes no product code. The reserved choices at the end go to HELP_HUMAN before implementation. Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

**Result.**

- **The bound.** A certified member-relative bound follows from K-D5's p = 128 re-formation with one more exact residual and one more correction solve (§1). It is certified against K-D5's intended system under one conditioning assumption, which holds with a measured margin above 140.
- **Tightness.** With exact loads, the bound reproduces the true u-propagated row error to within 6e-6, relative.
- **Use.** The bound is applied in the *corrected-row* form (§2), which also measures the recovery's own rounding.
- **Catches.** It catches every Passed breach in RV29's set and in a realistic set of standard-elbow lines.
- **It does not catch every RV30 breach.** It misses 12 of 8,946 breaching RV30 case-modes. All 12 are t96 lines at UTM skew placements with formed thermal or pressure loads, on torsion rows at q/S* ≤ 3.5e-7, with err/c from 1.0 to 8.6. Eleven of the 12 are also missed today. The cause is load formation, which K-D5's residual takes as intended; §4.1 gives the evidence and §6 the closure.
- **Cost.** The cost is the true breach rate. At today's floor (2^-34·S*), 21.9 % of the 42,096 pre-guard-Passed case-modes evaluated publish a breaching member row. In the realistic set this includes:
  - L lines (12 %);
  - U lines (31 %);
  - Z lines (27 %);
  - 4-bend expansion loops (45 %);
  - committed qualification and reference requests.

  Whether to accept that loss or to change the floor is the first reserved choice.

## The defect

- **K-D5 is not member-relative.**
  - **The rule.** K-D5 estimates the forward error of u as w = K̃⁻¹ρ. It demotes a case when 2|w_i| > 1e-9·max(|u_i|, S*_kind) at a free DOF.
  - **The recovery.** Member rows are formed from K_m·Δ_m, with Δ_m the member's end-relative motion.
  - **The mechanism.** Take a row whose value is small against its member's |K_m||Δ_m|. A stiff short member, a near-zero torsion or a near-symmetric station shear are examples. The recovery cancels, and an error in u that passes K-D5 becomes a row error far above 1e-9·q.
- **R-b′ masks gross errors.** `rb_prime_fires` requires q > 2^10·B. RV29's determinate 10 × 1 m skew cantilever therefore publishes Passed with errors of 6.1e5–7.3e5 × RD(1e-9)·q.
- **O-13's short-bend limit is geometric.** The limit is Rφ/L < 1.6e-2. Inert collinear nodes and soft legs defeat it (RV29). The same amplification occurs in straight chord twins and in rows away from the bend (RV30).

## What K-D5 computes today

FK `structural/formation_check.rs` (`check`, `evaluate`) is called from `finish_checked_factor` in `structural.rs`. It runs only when the case would publish Passed (`!ordinary_sensitive`) and a `FormationSource` is attached.

1. **K_p.** Every stiffness contribution is re-formed in `Wide<2>` at p = 128 from its binary64 primitives: straight frames, realized curved bends, objective connectors and springs.
2. **ρ.** ρ = f − K_p·u, one `ExactAccumulator` per free row.
   - f uses the ledger's force terms.
   - Each re-formed coefficient enters through its exact binary64 split; prescribed columns are included.
   - A split truncated below 2^-1074 widens the row by 2^-1074·|u_j|.
3. **w.** w = D·F̃⁻¹(D·ρ·2^t)·2^-t.
   - D = diag(2^{e_r}) is the attempt's scaling.
   - F̃ is the attempt's own factor: dense Cholesky or sparse skyline LDL.
   - Rows more than 2^1022 below the largest are flushed to zero.
4. **D5C-1.** The case is demoted (`Estimate`) at the worst free row with 2|w_i| > 1e-9·max(|u_i|, S*_kind), where S* is the coupled body scale of u. Anything that cannot be re-formed or evaluated also demotes (`FormationCheckUnavailable`).

The gate before K-D5 supplies two quantities:

- r̂, the Hager/Higham estimate of 1/κ₁(K̃_s) (`estimate_rcond`, documented as not certified);
- a = (relative perturbation + load perturbation)/r̂, which must be below 1.

A Passed case has r̂ ≥ √ε.

## 1. The bound

### Premises inherited from K-D5

- **P1 (intended system).** The intended problem is K_p·u* = f, with K_p the p = 128 re-formation and f the ledger's force terms, as K-D5 takes it. Load formation before S11-F, recovery formation and input representation lie outside it (K-D5's "What EF does not see"). The bound is certified against this problem; §4.1 measures the gap to an exact reference.
- **P2 (exact sums).** The residual sums are exact (`ExactAccumulator`), with K-D5's truncation allowance.

### Computation

These steps extend `evaluate` after w is formed.

1. s = ρ − K_p·w, exactly, in the same accumulators, with the element matrices from K-D5's step 1.
2. w₂ = D·F̃⁻¹(D·s·2^{t₂})·2^{-t₂}, the same solve as for w.
3. s₂ = s − K_p·w₂, exactly.
4. Λ = 2^10 / ((1 − a)·r̂·‖K̃_s‖₁), rounded upward. ‖K̃_s‖₁ is the norm that `estimate_rcond` already forms; it is returned.
5. For each member m and each published row r of m, form the u-linear recovery map A_r. This is the row as a function of the member's end displacements:
   - for end rows, K_p,m in the row's frame;
   - for section cuts and stations, that map times the station transfer;
   - stresses are scalar multiples of their resultants.

   Evaluate A_r·w and A_r·w₂ as exact sums of the K_p,m splits times the binary64 components, followed by the transfer at p. Evaluate ‖A_r·D‖₁ = Σ_c |A_rc|·2^{e_c} over the member's free DOFs, rounded upward.

### The bound

Let e = u* − u. For any vectors w and w₂, however the solves produced them, this identity is exact:

  e = w + w₂ + K_p⁻¹·s₂ .

K_s = D·K_p·D is symmetric, so ‖·‖₁ = ‖·‖_∞. Hence

  |A_r·e| ≤ B_e,r := |A_r·w| + |A_r·w₂| + ‖A_r·D‖₁ · ‖K_s⁻¹‖_∞ · ‖D·s₂‖_∞ .

Call the last term R_r. For a group g of rows (|N|, hypot(V_y, V_z), |T| or hypot(M_y, M_z)):

  ‖(A·e)_g‖₂ ≤ ‖(A·w)_g‖₂ + ‖(A·w₂)_g‖₂ + ‖(R_r)_{r∈g}‖₂ .

What the bound assumes:

- **A1 (conditioning, the only estimate).** ‖K_s⁻¹‖_∞ ≤ Λ.
  - It holds when the Hager estimate underestimates ‖K̃_s⁻¹‖₁ by at most 2^10, and the formation perturbation from K̃_s to K_s stays within the gate's a (a Neumann bound).
  - The product's amplification gate already relies on both estimates.
  - Measured over the 42,096 evaluated case-modes, ‖K_s⁻¹‖·r̂·‖K̃_s‖₁ lies in [1, 7.17], a margin above 140.
  - Λ enters only through the third-order term. A larger error in the estimate changes only that term (§4.3).
- **The correction solve's own error.** No assumption is needed. s₂ is the exact residual of w + w₂, so the quality of the solves affects only how tight the bound is. Observed ‖w₂‖/‖w‖ ≤ 2.4e-9.
- **Rounding of the published rows** (recovery formation) is not in B_e. §2 measures it.
- **Dense and sparse.** Nothing depends on the factorization: each mode uses its own F̃, w, w₂ and r̂. The bound holds for the binary64 solve as computed.

### Preconditions, and the unavailable path (fail closed)

The bound is unavailable, and every group of the case fires, when any of these holds:

- K-D5 itself is unavailable or demoting. The case is then already Sensitive, so this is a no-op.
- s or s₂ leaves the exact-sum range, or the second solve fails or returns a non-finite value.
- r̂ ≤ 0, a ≥ 1, or Λ·‖D·s₂‖ is non-finite.
- A member's recovery map cannot be formed at p: an explicit curved matrix, or an uncertified load term.
- The published u is not the u that K-D5 checked.
  - A W2 force-scaled publication must carry its own scaled check, with w unscaled by 2^-b.
  - A source-recovery selection publishes retained values and is outside this guard.

If ρ = 0 exactly, then w = w₂ = s₂ = 0 and B_e = 0.

## 2. The integrity decision: the corrected-row form

Take a published member group g with published values v_g. Define m_g and r_g as follows:

- **m_g** is the intended recovery of g evaluated at the corrected displacement u + w + w₂. It uses the row's intended function (K_p,m, the intended load and free-strain terms, the station transfer) at p = 128, with K_p,m·(u + w + w₂) taken as one exact sum of the three products.
- **r_g** is the ball radius of that evaluation (at the 2^-120 level) plus ‖(R_r)_g‖.

Under P1, P2 and A1:

  ‖v_g − g_int(u*)‖ ≤ B_g := ‖v_g − m_g‖ + r_g .

This is I34's a-posteriori form B_f = |v − m| + r (O-13's arc certificate), with u replaced by u + w + w₂. One certificate measures the recovery's own rounding and the solve's error together. They are not added in absolute value, so cancellation between them is not charged.

The triangle form B_f + B_e is a valid but looser alternative. It needs an a-posteriori B_f at every group; with today's a-priori straight bound (γ₁₆Σ|K||T||u|) it is not usable (§4.2, variant U2).

**The decision.** For a case that would publish Passed, a group g is *unresolved-or-breaching* when B_g is unavailable or non-finite, or when

  B_g > RD(1e-9)·q_g (decided exactly) ∧ S*_g ≥ 2^-988 ∧ q_g ≥ 2^-34·S*_g .

- q_g is the 2-norm of the group's published values.
- S*_g is the body's coupled scale over the case's published rows, as O-13 extended it, guarded arcs included: the moment scale for bending and torsion, the force scale for axial and shear.

**The consequence.** If any group fires, the case is demoted by R-b′'s mechanism.

- One sentence is appended to the case's existing integrity diagnostic, naming at most six groups (member, location, group, q, B), and the code moves from `CHECKS_PASSED` to `SENSITIVE`.
- The no-op rule holds: a case already Sensitive or weaker is left byte for byte.
- Nothing is refused, no value changes, and no envelope field or code is added.

**The groups** are every published member row, for straight frames, realized arcs and objective connectors:

- end rows (chord frame);
- endpoint section cuts;
- the three interior stations.

The stress, pipe-wall and membrane rows are judged through the resultant they scale. Support reactions (K_rf·u − f at restrained rows) form one group per support, with A = K_rf.

Displacement and rotation rows stay with K-D5's per-DOF rule. Rotation rows also breach row-relatively (128 group-cases in this evidence). That is K-D5's own question and is not changed here.

## 3. R-b′ unified for all members

One predicate replaces `rb_prime_fires` (straight), `arc_group_fires` (O-13 with R-2), O-13's B_u and the short-bend rule:

  fires(g) ⟺ ¬finite(B_g) ∨ [ exceeds(B_g, q_g) ∧ S*_g ≥ 2^-988 ∧ q_g ≥ 2^-34·S*_g ] ,

where `exceeds` is today's exact test B > RD(1e-9)·q.

- **No resolution conjunct.** A row whose error is gross relative to q fires. This is R-2's behaviour, now for every member and group.
- **Composition with §1–§2.** B_g is the corrected-row bound.
- **K-D5's per-DOF rule** is unchanged and runs first.
- **The load-row guard** is unchanged. When it fires on the same case, the load-row sentence stands first (I5 ruling 1).

**RV29's straight masking is closed.** The 10 m and 20 m skew cantilevers at η = 1e-9 (q = 1.94e-6 N·m) publish Passed today. Under the predicate, B_g/(RD(1e-9)·q) at the end groups M7.j and M8.i matches the true error, 8.2e5 to 8.3e6. The error is u-driven (Eu ≈ err), and the groups fire in both modes. The same holds for η = 3e-9 to 1e-6, where today's R-b′ also fires.

## 4. Evidence

**Method.**

- **Probe.** A scratch probe ran in a detached worktree of `codex/piping-t4-u2-20261010` (d92850097d, with O-13 and U3 merged). For each case-mode it dumped K-D5's u, w, s, w₂ and s₂, the gate's r̂ and a, and Λ·‖D·s₂‖.
- **Prototype.** A scratch PP hook evaluated B_e and the predicate variants with the product's own recovery code.
- **Reference.** RV30's exact reference (`ref.py`, 80-digit Decimal) gave, per group:
  - the published error err;
  - its exact split into the u-part Eu = ‖A(u* − u)‖ and the formation part Ef;
  - the emulated bounds, including the corrected-row bound B_c = ‖(v − R(u)) − A(w + w₂)‖ + R.

  RV29's specs were converted to it, with per-member sections and nodal loads added.
- **Judging** follows RV30's `compare.py`: groups at or above 2^-34·S*, with the ratio err/(RD(1e-9)·q).
- **Sets with an exact reference:**
  - all 22,560 RV30 models (families A–D, chord twins, corner, cert, k1–k3 and smoke), typed entry, both modes;
  - RV29's 53 specs (inert-node, soft-leg, near-π, large-k, tiny-angle and straight cantilevers);
  - a realistic set of 492 lines:
    - I34's L-line set;
    - standard elbows at R = 1.5D and R = 0.2286 m, with t7, t24 and t96 sections, as L, U and Z lines and 4-bend expansion loops;
    - loads w, wT and wTp, at origin-xy and UTM-skew placements.
- **Sets without a reference:**
  - T4-I19's 182 SP-1 committed requests;
  - T4-I7's U2 reference cases, built as I34 built them;
  - the demo.
- **Evidence location:** `WT/scratch/t4_code/t4_i41/`: `tools/`, `analysis_*.txt`, `floor.txt`, `moves_*.txt`, `avail_extra.txt`, `real/`, `rv29/`, `eval/`.

### 4.1 Acceptance

The table counts pre-guard-Passed case-modes: solver Passed, K-D5 not demoting, load-row guard silent. Today's short-bend rule counts as a guard here, because it is replaced.

| Set | Pre-guard Passed | Breaching | Missed today | B_e alone misses | Corrected-row misses (×1) | Corrected-row misses (×2) |
|---|---|---|---|---|---|---|
| RV30, all families | 41,085 | 8,946 | 6,119 | 203 | **12** | 1 |
| RV29 | 90 | 42 | 17 | 2 | **0** | 0 |
| Realistic | 921 | 212 | 176 | 3 | **0** | 0 |

**Soundness of B_e.**

- With exact loads (RV29), Eu ≤ 1.000006·B_e on all 3,076 groups with Eu > 0.1·c.
- With formed distributed, thermal and pressure loads, Eu exceeds B_e where load formation, unseen by K-D5, propagates into the rows.
  - Eu > 1.5·B_e on 723 of 508,877 RV30 groups with Eu > 0.1·c, and on 35 of 17,930 realistic ones.
  - The largest ratios are 206 (on a row with Eu < 0.6·c) and 69 (B-family wTp at UTM-skew).

**The corrected-row misses are those load-formation rows at the threshold.**

- All 12 RV30 misses are t96 lines at UTM-skew placements with thermal or pressure loads.
- They are torsion rows (end, section cut or midspan) at q/S* ≤ 3.5e-7, with err/c from 1.0 to 8.6.
- The worst, B|L|0.5-20|t96|R1|ρ0.02|wTp at end_j, has err/c = 8.6 and B/c = 0.61.
- Eleven of the 12 are also missed today.
- A factor of 2 on B_g (K-D5's `FORMATION_FACTOR` convention) leaves one miss: a Z line, wT, utm5-skew, err/c = 1.11, B/c = 0.22, with Eu/B_e = 3.3. Its cost is in §4.2.
- The sound closure is to re-form the load terms at p (§6).

**B_e alone** misses 203 RV30 case-modes. These breaches are formation-dominated.

- 182 of them are the arc's endpoint section-cut torsion (`s_torsion end_j`), with err up to 5.3·c and Ef ≈ err. O-13's B_f covers only the chord-frame end groups.
- The corrected-row form measures formation at every group. It closes these, and it closes RV29's tiny-angle station breach (φ = 1e-6, R = 3e5 m): torsion at the quarter points and midspan, err = 2.9e4·c, q/S* ≈ 1e-8, Ef ≈ err.

### 4.2 Availability

**Case-modes demoted by the corrected-row form at the 2^-34·S* floor:**

| Set | Demoted | Share of pre-guard Passed |
|---|---|---|
| All sets | 9,204 of 42,096 | 21.9 % |
| RV30 family A | 1,279 | 8 % |
| RV30 family B | 2,841 | 20 % |
| RV30 family C | 743 | 78 % |
| RV30 family D | 3,969 | 40 % |
| Realistic: standard L lines | 45 of 376 | 12.0 % |
| Realistic: U lines | 45 of 144 | 31.2 % |
| Realistic: Z lines | 76 of 281 | 27.0 % |
| Realistic: 4-bend loops | 35 of 78 | 44.9 % |
| Realistic: I34's L lines | 9 of 32 | 28.1 % |

**Verdict moves against today:**

| Set | Passed → Sensitive | of which correct (no breach) | Sensitive → Passed | of which breaching |
|---|---|---|---|---|
| RV30 | 6,121 | 13 | 4,779 (short-bend false demotions removed) | 1 |
| Realistic | 176 | 0 | 12 | 0 |

**The floor (`floor.txt`, all sets).** τ is a hypothetical floor: rows are judged only when q/S* ≥ τ.

| τ | Breaching case-modes | Demoted |
|---|---|---|
| 2^-34 (today) | 9,200 | 9,204 (21.9 %) |
| 1e-6 | 5,889 | 5,893 (14.0 %) |
| 1e-3 | 2,879 | 2,879 (6.8 %) |
| 1e-2 | 1,776 | 1,776 (4.2 %) |
| 1e-1 | 630 | 630 (1.5 %) |

At τ = 1e-3 the realistic families are still demoted: U lines 20.8 %, loops 33.3 %, Z lines 9.3 %, L lines 4.8 %.

**Why.**

- **The demotions are true breaches.** The corrected-row form demotes 16 non-breaching RV30 case-modes and none in RV29 or the realistic set.
- **Where the breaching rows are.** They are mostly rows whose value is small against their member's |K||Δ|:
  - torsion and shear in near-planar systems (straight-member torsion is the largest single class);
  - interior-station shear at near-symmetric points;
  - axial force where a free expansion is unrestrained.
- **Realistic loops are short-stiff-bend systems.** Expansion loops with 0.25 m elbows and 20 m runs have Rφ/L ≈ 0.02.

**Committed requests** (no exact reference; prototype predicate):

- **SP-1 set E: 16–18 case-modes move to Sensitive.**
  - **Load-reference qualification fixtures** (`coefficient_definition`, `constant_alpha_interval`, `multi_segment_free_length`, as preview requests and runner inputs; DenseScrutiny): 16 case-modes.
    - The axial force of a freely expanding member is published as −1.16e-10 N, where the truth is 0.
    - Every force row of the body is noise, so S* is the noise and q/S* = 1; B_e ≈ 1.4·q.
  - **`prescribed_rotation_all_fixed`**, with the triangle variant only: R-b′'s a-priori straight bound is 63 × q there. It is masked today by the resolution conjunct.
- **I7: 8 case-modes.** These are `U2-L-FREE-P-K2-SKEW-X5E6` and `-X7P3E6`, W and W+T, both modes. The arc's end section-cut torsion fires at q/S* = 3.4e-10, with B/c from 5e3 to 2e4.
- **SP-1 sets B1 (the retained-precision corpus requests) and Z:** no change.
- **Not evaluable on this head:** set D, set V and the demo are blocked on U2 (legacy pressure and connector re-authoring).

**Committed PP tests.** With the predicate applied in the prototype, 15 PP tests fail:

- in `s11g`: `p1`, `p4`, `p5`, `p6` and `p8` of `arc_recovery`, the two short-bend tests, `t11`, `t15b` and `t20`;
- `f1b_w2_runtime` (2 tests);
- `k2a_formation_range_runtime` (2 tests);
- `s11f_site_test` rule 6.

Among them are two controls designed to stay Passed:

- `p8_a_stress_free_arc_stays_passed`: every row is noise of about 1e-12 N, S* is that noise, and B_e ≈ 100·q.
- `t15b_certified_arcs_pass_at_utm_and_small_angles`: a 1e-4 rad arc whose quarter-station bending at q/S* = 8e-9 gives B/c = 2.2.

The W2 and K2a failures are prototype artifacts. Those tests rely on R-b′'s sentence on routes where the prototype's bound was not attached. Under §1's unavailable rule those cases stay Sensitive.

**Variants rejected on this evidence:**

- **U2.** The a-priori γ₁₆Σ|A||u| bound for every group demotes 12,979 RV30 and 225 realistic case-modes that do not breach.
- **U1.** B_e with today's B_f and no resolution conjunct misses the formation-dominated section-cut and station breaches: 332 RV30 and 10 realistic case-modes.
- **B_e with a factor of 2 plus today's guards** demotes 2,294 non-breaching RV30 case-modes and 69 realistic ones.

### 4.3 Numerical checks

- **Hager ratio.** ‖K_s⁻¹‖·r̂·‖K̃_s‖₁ ∈ [1, 7.17] (A1's margin ≥ 140).
- **The third-order remainder** ‖A·D‖·Λ·‖D·s₂‖ is at most 0.01·c on the realistic set and 4e-6·c on RV29.
  - In the RV30 C family it reaches 39·c to 106·c on 73 case-modes with r̂ ≈ 2e-8. Those cases are Sensitive or breaching anyway; this is where the 2^10 in Λ costs.
  - A first-order remainder ‖A·D‖·Λ·‖D·s‖ reaches 2e3·c on RV29's L4 lines. The second correction is what makes the bound tight.
- **Probe consistency.** The probe's u matched the published displacement rows bit for bit on every RV30 run. The product prototype and the Python emulation agree on B_e: for example 0.0953·c on the smoke L line at ρ = 0.017, in both.

## 5. Bytes and blast radius (SP-4 forecast)

- **T3's retained route does not use K-D5.**
  - `source_recovery::solve_ordinary` and the retained product solve through the exact boundary path, and curved spans stay outside D1 (`source_recovery.rs`, `retained_product.rs`).
  - Every invocation's *ordinary attempt* does run K-D5, and the new solve and residuals add cost to the ordinary route.
  - Like R-b′, the demotion is applied after recovery. It never changes `needs_source_recovery` or D1 admission.
- **Registered profile.**
  - `s(RecoveryRecord)` is priced by `size_of`, and it grows to per-location, per-group records. That needs re-registration under D3, pre-approved within M = 10.5 GiB and the 5 % text budget.
  - `Text(recovery_finding)` is a priced text atom. Keep each named group item no longer than today's per-end item, with at most six named, or re-run the TEXT chain.
  - `Text(formation_detail)` changes only if a new unavailable detail is longer than today's longest.
  - `StructuralSolution`, which gains the bound's inputs, is not priced.
- **Pins:**
  - `formation_guard.rs` and its S11-G pins: `rb_prime_clauses_at_their_boundaries`, `t11`, `t12`, `t13`, `t13b`, `t20` and `ruling1_both_guards_fire_and_the_load_row_sentence_stands`;
  - O-13's arc tests `p1`–`p8`;
  - the short-bend tests, deleted with the rule. The refusal `CURVED_BEND_SHORT_RELATIVE_TO_NEIGHBOURS` below 1e-4 stays;
  - `t15b`;
  - the F1b W2 and K2a reach tests that read R-b′'s sentence;
  - the S11-F site table (`s11f_site_test` rule 6, which lists every function that touches the force).

  straight_pipe's `t17` is unchanged unless the straight bound is replaced.
- **Corpus and committed outputs.**
  - Integrity codes move from `CHECKS_PASSED` to `SENSITIVE` for the committed requests listed in §4.2. Each regenerated output is declared under SP-1.
  - No retained-precision corpus request moved in the prototype.
  - TEXT entries change only through the sentence (above). No value, field or code changes.

## 6. Implementation plan

- **FK `structural/formation_check.rs`.**
  - Keep each element's K_p from the ρ pass, or re-form it.
  - After w, form s, w₂ and s₂ as exact sums.
  - Return `MemberBoundInputs { w, w2, scale: D, lambda_ds2 }`, or the unavailable reason.
  - **Load-formation closure** (needed for zero RV30 misses): form the thermal and pressure free-strain load terms of f from their primitives at p, as K_p·(ε·d), instead of taking the binary64 ledger values. The 12 misses all come from such terms at UTM coordinates.
- **FK `structural.rs`.**
  - `estimate_rcond` returns ‖K̃_s‖₁.
  - `finish_checked_factor` passes r̂ and a to the check.
  - `StructuralSolution` carries the inputs. `StructuralReport` does not, so reports that are not demoted keep their bytes.
  - The W2 force-scaled entry carries its scaled inputs and unscales them once.
- **Certificates.**
  - A straight-member recovery certificate at p: end actions from K_p,m and the intended load and free-strain terms, endpoint section cuts and stations. It sits beside O-13's arc certificate.
  - Both certificates take the correction w + w₂ as an extra exact-sum term, and both cover section cuts and stations.
  - Connectors use K-D5's connector matrix.
- **PP `formation_guard.rs`.**
  - `member_group_fires(q, B, S*)` replaces `rb_prime_fires` and `arc_group_fires`.
  - One `RecoveryRecord` shape: member, body, and for each location the four groups' q and B, or the unavailable reason.
  - One finding.
  - `short_bend_finding`, `ShortBend` and `SHORT_BEND_PASSED_RATIO` are removed.
- **PP `lib.rs`.**
  - Records are made for every published member group and every support group.
  - `bend_neighbour_ratios` stays only for the refusal diagnostic.
- **Tests and mutants that must be killed:**
  - **each B_e term:** drop |A·w₂|; drop the remainder; use s instead of s₂; drop (1 − a); drop the 2^10 factor; drop D;
  - **the corrected-row form:** evaluate m at u instead of u + w + w₂; drop r;
  - **the decision:**
    - `≥` in place of `>`;
    - a rounded comparison in place of the exact one;
    - the floor conjunct;
    - the 2^-988 guard;
    - the resolution conjunct reintroduced (RV29's cantilever kills it);
    - the group norm replaced by a maximum;
  - **the unavailable path:** a failed second solve, a non-finite Λ, an unformable member and a mismatched u each fire;
  - **the no-op rule**, and the load-row and recovery sentences together;
  - **a source pin** tying the call site to the tested predicate;
  - **natural positives:**
    - RV29's 10 m cantilever at η = 1e-9 (straight);
    - the inert-node L line at 4.5° (arc shear);
    - a B-family arc endpoint section cut (`s_torsion end_j`, formation-dominated);
    - RV30's far-member L line at 32.7×;
  - **natural negatives:** the 90° L line and I34's realistic Passed lines.
- **O-13's empirical limit.** `SHORT_BEND_PASSED_RATIO` = 1.6e-2 and its record in arc-recovery-guard.md are removed when this lands. The Passed and Sensitive thresholds in the "Short stiff bends" ruling become outcomes of the predicate, not rules.

## Reserved choices for HELP_HUMAN

1. **The cost of the criterion as written.**
   - At the 2^-34·S* floor, the fix demotes 21.9 % of the pre-guard-Passed case-modes evaluated: realistic L lines (12 %), U lines (31 %), Z lines (27 %) and loops (45 %), and the committed requests in §4.2. These are true breaches.
   - The options:
     - accept the demotions;
     - change which rows the 1e-9 row-relative criterion protects, with a higher member-row floor τ (§4.2);
     - add an absolute noise floor for bodies whose rows are all noise (p8, the free-expansion load-reference fixtures).

   Each option other than accepting the demotions changes an acceptance criterion.
2. **Load formation.**
   - K-D5 does not see it, so the corrected-row form misses 12 RV30 case-modes; 11 of these are also missed today.
   - The options:
     - re-form the free-strain load terms at p (§6), which is sound;
     - apply a factor of 2 on B_g, which leaves 1 miss and demotes 2,366 non-breaching RV30 and 70 realistic case-modes;
     - disclose the residual.
3. **Routes without K-D5.**
   - The nonlinear-support route is never selected for K-D5 (ROOT's ruling).
   - Failing closed would demote every Passed case on that route.
   - Keeping today's formation-only R-b′ there leaves the same breach mechanism unguarded, and must be disclosed.
