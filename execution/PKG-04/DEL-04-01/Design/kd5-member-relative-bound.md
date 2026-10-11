# K-D5 member-relative bound and the unified R-b′ predicate (T4-KD5)

Revision 1. It implements HELP_HUMAN's ruling of 2026-10-11, made under the owner's delegation: **option (d), "repair"**. Revision 0 (68d78411af) derived the bound and measured the cost of applying it as a guard. Revision 1 makes the unit publish what the bound certifies. It also folds T3's load closure into D5C-1, and applies RV33's corrections and T3's interface review (M1–M3, S1–S7).

The record extends K-D5 ([curved-numerical-integrity.md](curved-numerical-integrity.md)) and R-b′/O-13 ([arc-recovery-guard.md](arc-recovery-guard.md)). It changes no product code. Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

**Result.**

- **One intended system.** K-D5's intended system is its p = 128 re-formation of the stiffness and, new in revision 1, of every formed load term from its binary64 primitives (§1). The residual, both corrections, the bound and the published member rows all use that one system. No ledger value of a formed load term enters any of them.
- **Repair.** A member row of a case that would publish Passed is published as its certified recovery: the intended recovery m_g evaluated at the corrected displacement u + w + w₂, rounded once to binary64 (§3). Its bound is the rounding, the evaluation radius and the third-order remainder.
- **The bound holds on every breaching group.** On all 90,734 breaching groups of the RV30 and realistic sets (87,375 + 3,359) and RV29's 190, B_c ≥ err. The certified remainder exceeds the true remainder ‖A(e − w − w₂)‖ by at least 2,763 times on every group. There are no exceptions (§4.1).
- **Residual demotions (predicted by emulation).**
  - RV30: 0 of 41,085 pre-guard-Passed case-modes.
  - Realistic set: 0 of 921.
  - RV29: 0 of 90.

  The worst judged ratio B/c is 0.023, so the margin is at least 43 (§4.2).
- **W1's N-5 abandonment (T3's M1).** On 2,844 straight-only invocations in W1's retained-source domain, (d) demotes none, so W1 abandons none. Today R-b′ makes W1 abandon 64 of them. Revision 0's guard-only form would have made it abandon 533 (§4.4).
- **Published values change.** Every member row of a Passed case is recomputed at p, so last bits change on every such case. In 21.8 % of RV30 case-modes (23.0 % realistic) a row moves by more than its criterion: those are today's breaches, now repaired. The per-case largest change is about 1e-12·S* at the median, 4e-10·S* at p99 and at most 6.3e-9·S* (§3.4).
- **The closure is necessary.** Without it, (d) would publish Passed breaches on 2,215 RV30 and 76 realistic case-modes, up to 3.9e4 times the criterion (§4.3). The short-bend rule is therefore removed only with the closure, in the same unit (T3's M3). With the closure, no case moves from Sensitive to Passed with a breach.
- **Nonlinear-support route.** It has no K-D5, so it fails closed with a named reason. 8 committed case-modes move to Sensitive: the demo and the invented preview-physics model, two cases each, both modes (§5). Extending K-D5 to the converged active set is the next T4 unit.

## The ruling and what it supersedes

**HELP_HUMAN, 2026-10-11 (under the owner's delegation).** The unit's target is option (d): publish the certified member rows recovered at p (the intended recovery m_g at u + w + w₂, rounded to binary64), with T3's full load closure folded into D5C-1, so that K-D5 certifies one intended system. The 1e-9 criterion and its 2^-34·S* floor stay unchanged, with no interim criterion change. The conditions are T3's M1–M3 and S1–S7 and RV33's corrections; this record meets them in the sections named below.

**Superseded when this unit lands:**

1. **HELP_HUMAN's ruling 5 on short stiff bends** ([curved-numerical-integrity.md](curved-numerical-integrity.md), "Short stiff bends"), in respect of its Passed and Sensitive thresholds.
   - The thresholds were Passed at 6.6e-3, Sensitive from about 6e-3 down to 5e-5, and O-13's derived Passed limit.
   - They become outcomes of the solver's own gate (r̂ < √ε) and of the unified predicate, not rules.
   - The pivot-screen refusal and `CURVED_BEND_SHORT_RELATIVE_TO_NEIGHBOURS` below 1e-4 stay.
2. **The withdrawn option (c) limit of 1.6e-2.** This is O-13's `SHORT_BEND_PASSED_RATIO` with the short-bend rule (`short_bend_finding`, `ShortBend`), on `codex/piping-t4-o13-20261010`. It is removed in this unit, after the closure (§4.3).
3. **The t15b acceptance at 1e-4 rad.** This is the requirement that "guard and K-D5 margin from φ = 1e-4 rad" holds, with the test `t15b_certified_arcs_pass_at_utm_and_small_angles` pinning Passed.
   - A small arc's verdict is now the unified predicate's outcome on its certified rows.
   - t15b is re-pinned to that outcome, which is predicted Passed because its rows are repaired. It is no longer an acceptance requirement.

Notes recording this are added to both Design files.

## The defect (unchanged from revision 0)

- **K-D5 is not member-relative.** K-D5 bounds the forward error of u per DOF against max(|u_i|, S*_kind). Member rows come from K_m·Δ_m. A row that is small against its member's |K_m||Δ_m| cancels in recovery, so an error in u that passes K-D5 becomes a row error far above 1e-9·q.
- **R-b′ masks gross errors.** `rb_prime_fires` requires q > 2^10·B. RV29's skew cantilevers therefore publish Passed with errors of 6.1e5–7.3e5 × RD(1e-9)·q.
- **O-13's short-bend limit is geometric.** It is defeated by inert nodes, soft legs and chord twins (RV29, RV30).
- **K-D5's residual takes the ledger's formed loads as intended.** Load formation propagates into rows unseen (RV33 finding 2; T3's M2). This is new in revision 1's analysis; §4.3 measures it.

## 1. The intended system and the load closure (T3's M2, S1, S2)

### Definitions

- **Certified** means certified against K-D5's intended system (S1). That system is the linear problem K_int·u* = f_int.
  - K_int is K-D5's p = 128 re-formation of every stiffness contribution from its binary64 primitives, as today: straight frames, realized arcs, objective connectors and springs.
  - f_int is the p = 128 re-formation of every load term from its binary64 primitives.
  - The input representation is part of the intended input: binary64 coordinates, section properties, E, G, α, ΔT, p, k and loads as authored. The distance between K_int's arc and the real-number element stays K-D5's existing premise (2^-120·κ_tip, not certified).
- **One intended system (S2).** These all use the same re-formed matrices and the same re-formed load terms:
  - the residual ρ = f_int − K_int·u;
  - the residuals s and s₂ of the two corrections;
  - the recovery m_g of every published row.

  No formed ledger value enters any of them. There is no ledger-and-p hybrid.

### The closure rule (fail closed)

FK's `FormationSource` gains `loads: Vec<LoadPrimitive>`, one entry per formed load contribution, each naming the ledger terms it replaces. Every ledger term falls into exactly one class:

- **Input term.** A term with no S11-G formation record, such as a nodal force or moment, a constant effort, or a prescribed displacement value. It enters as its exact binary64 value, which is its intended value.
- **Formed term.** A term with a formation record. It is replaced by its primitive's re-formation at p, added to the row accumulators as the exact split of the Wide2 value. Any enclosure radius is carried (§2).

A formed term that no primitive covers, or a primitive whose re-formation fails (a `WideError` or a failed R-1 precondition), makes the check unavailable: `FormationCheckUnavailable { "load closure: <source> …" }`. The case is then Sensitive at the solve, never Passed on a ledger value. Coverage is therefore decided by the ledger's own formation records, not by a list kept by hand.

### Every formed load term

All producers are in PP `lib.rs` unless named. The primitive and its re-formation at p are given per term.

| Term | Producer | Owner | Primitive and re-formation at p | Radius |
|---|---|---|---|---|
| Nodal forces and moments, constant-effort support forces | `push_nodal_loads`, `add_constant_effort_support_loads` | DEL-05-05, DEL-04-07 | Input terms: no primitive | 0 |
| Prescribed displacements | `imposed_displacements`, K_fc·u_c | DEL-04-07 | Input u_c; K_fc·u_c is already in ρ (prescribed columns) | as today |
| Self-weight and uniform loads on straight members | `self_weight.rs` → `add_uniform_element_loads` (consistent vector) | DEL-05-01 (self-weight), DEL-05-05 (application), DEL-04-02 (consistent vector) | `UniformStraight{frame, w}`: wL/2 and wL²/12 in the chord frame from (x_i, x_j, y, w), rotated to global | 2^-120 level |
| Uniform loads on realized arcs (R-1's certified arc loads) | `add_uniform_element_loads` → FK `certify_curved_uniform_load` | DEL-05-05 (with DEL-04-01's certificate) | `UniformCurved{arc, w}`: R-1's midpoint–radius enclosure of the consistent vector | R-1's radius |
| Thermal and free strain on straight members (eigen pair P·x_a) | `add_thermal_equivalent_loads` | DEL-05-01 (α, ΔT, ε), DEL-04-02 | `FreeStrainStraight{frame, ε}`: ±E·A·ε along the chord, with ε = α·ΔT formed at p | 2^-120 level |
| Thermal free strain on arcs, K_b·u_free(ε) | `add_curved_bend_thermal_equivalent_load` | DEL-05-01, DEL-04-07 | `FreeStrainCurved{arc, ε}`: K_int,arc·(ε·d) with the same arc matrix as ρ | 2^-120 level |
| T4-U2 pressure free strain on arcs, K_b·u_free(ε_p) | exact-pressure route | DEL-04-07 | `FreeStrainCurved{arc, ε_p}`: ε_p = (1 − 2ν)·p·A_i/(E·A), formed at p | 2^-120 level |
| T4-U2 pressure operands: terminal caps, bend terms −c_b, kink remainder | `push_exact_pressure_operands` | DEL-04-07 | `PressureOperand{kind, p, A_i, tangents}`: tangents from the arc's (d, R, y) at p, as K-D5 forms the arc | 2^-120 level |
| Straight-route pressure end forces | `exact_straight_end_forces` | DEL-04-02 | `PressureStraight{frame, p, section}` | 2^-120 level |
| Connector reference loads (T4-U3 q_ref) | `add_connector_reference_loads` | DEL-04-07 | `ConnectorReference{connector, q_ref}`: `connector_matrix`·q_ref | 2^-120 level |
| W2 force-scaled terms | F1b/K2b | DEL-04-07 | The same primitives at 2^b, which is an exact scaling | as above |

Under the effective-force pressure model the nodal pressure terms (caps, −c_b and kink remainder) net to zero at bend ends in the intended system. Re-formed at p, they do net. Today their binary64 residue enters ρ as if intended.

**Dependency edges.** DEL-04-01's `deliverable.yaml` gains gating needs on the load owners, DEL-05-01, DEL-04-02, DEL-04-07 and DEL-05-05 (R-1's arc loads are applied by DEL-05-05). Each need states the condition the owner must meet: an intended-load definition K-D5 can re-form at p, made of the binary64 primitives and the formula, attached to the formed ledger term as a `LoadPrimitive`.

**D5C-1 fold.** K-D5's per-DOF rule runs on the closed w. On every evaluated set no per-DOF verdict changes (§4.2): the load-formation part of w is far below 1e-9·S*.

## 2. The bound (RV33's corrections and T3's S6)

### Computation

These steps run in FK `formation_check.rs` `evaluate`. The element matrices are re-formed on the stack at each use, never cached on the heap (S5).

1. **ρ.** ρ = f_int − K_int·u, exactly, as one `ExactAccumulator` per free row.
   - The loads are as in §1.
   - Every coefficient enters through its exact split.
   - The radius vector ψ_ρ collects every split truncated below 2^-1074 (2^-1074·|u_j|) and the load enclosure radii.
   - The allowances are carried as a radius, not added into the accumulator as a shift (RV33 finding 4).
2. **w.** w = D·F̃⁻¹(D·ρ·2^t)·2^-t, with the attempt's own factor, as today. D5C-1 then runs on w.
3. **s.** s = ρ − K_int·w, exactly, in the same accumulators. ψ_s = ψ_ρ plus the truncation allowances of K_int·w.
4. **w₂.** w₂ = D·F̃⁻¹(D·s·2^{t₂})·2^{-t₂}.
5. **s₂.** s₂ = s − K_int·w₂, exactly. ψ = ψ_s plus the allowances of K_int·w₂.
6. **The radius on ‖D·s₂‖.** σ := ‖D·(|s₂| + ψ)‖_∞, rounded upward.
7. **Λ (RV33 finding 1).** `estimate_rcond` exposes the Hager estimate: est ≈ ‖K̃_s⁻¹‖₁ = 1/(r̂·‖K̃_s‖₁) and ‖K̃_s‖₁.
   - ε_E bounds ‖E‖ for E = K_s − K̃_s. It is formed directly, upward, as the larger of the row-sum and column-sum norms. It is the triangle bound over the element contributions: Σ_e |D(K_int,e − K̃_e)D|, with K̃_e the binary64 contribution the assembly used, plus the exact assembly rounding |D(Σ_e K̃_e − K̃)D| that the contribution audit already forms.
   - Λ := 2^10·est / (1 − 2^10·est·ε_E), rounded upward.
   - The gate's `a` is not used for this. It covers assembly rounding only, not element formation (RV33).
8. **The recovery map.** For each published row, A_r is the row as a linear function of the member's end displacements, and ‖A_r·D‖₁ = Σ_c |A_rc|·2^{e_c}, upward.

### The bound

Let e = u* − u. For any w and w₂, exactly, e = w + w₂ + K_int⁻¹·s₂*, where s₂* is s₂ and |s₂* − s₂| ≤ ψ. K_s = D·K_int·D is not exactly symmetric (S6, RV33 note 5): `rotate` and the arc's H·K_t·Hᵀ round independently. The bound therefore uses the ∞-norm directly:

  |A_r·K_int⁻¹·s₂*| = |A_r·D·K_s⁻¹·D·s₂*| ≤ ‖A_r·D‖₁ · ‖K_s⁻¹‖_∞ · σ =: R_r .

K̃_s (the prepared binary64 matrix) is symmetric, so its 1- and ∞-norm inverses agree. The ∞-norm Neumann step needs ‖E‖_∞, and ε_E bounds both norms.

- **A1, the one assumption (RV33 finding 1).** ‖K̃_s⁻¹‖_∞ ≤ 2^10·est. Hager's estimate is a lower bound, so A1 is an assumption on ‖K̃_s⁻¹‖ and is stated as one.
  - With A1 and 2^10·est·ε_E < 1, ‖K_s⁻¹‖_∞ ≤ Λ (Neumann).
  - Measured: true ‖K_s⁻¹‖ ≤ Λ on every evaluated case-mode, and the Hager ratio lies in [1, 7.17] (revision 0), a margin of at least 140.
- **The correction solves need no assumption.** s₂ is the exact residual of w + w₂. The quality of the solves affects only how tight the bound is.

### Fail closed

The bound is unavailable, and the case is Sensitive, when any of these holds:

- 2^10·est·ε_E ≥ 1.
- The observed contraction is above 2^-20: ‖D⁻¹·w₂‖_∞ / ‖D⁻¹·w‖_∞ > 2^-20. This is RV33's "‖D·w₂‖/‖D·w‖", written in the solve's own scaled variables y = D⁻¹w. It is where K_int differs from K̃ enough to matter.
- s or s₂ leaves the exact-sum range, or the second solve fails or returns a non-finite value.
- Λ·σ is not finite.
- The load closure fails (§1).

All of these are decided in FK and reported as K-D5's `FormationCheckUnavailable` with a named detail, so the solve itself is Sensitive. That keeps them off R-b′'s recovery-demotion path, and so off W1's N-5 abandonment (§4.4).

ρ = 0 exactly gives w = w₂ = s₂ = 0 and R = 0.

**K2b scaling (S3).** W2 solves K' = 2^b·K_int, f' = 2^b·f_int.

- The displacements are unchanged at any b. w and w₂ are displacement corrections and **need no unscaling** (revision 0's "w unscaled by 2^-b" was wrong).
- s, s₂, ψ, σ and every force-unit row are formed at 2^b. D' = 2^{-b/2}·D, Λ' = Λ and R' = 2^b·R.
- m_g and R_r are unscaled by 2^-b consistently: m_g by the publication's single rounding (`unscale_for_publication` of the exact value) and R_r rounded upward. The ratio R/c does not depend on b.

## 3. Repair: the published rows and the decision

### 3.1 The certified row

For a group g of a case that would publish Passed:

- **m_g** is the intended recovery of g at the corrected displacement u + w + w₂: K_int,m·(u + w + w₂) as one exact sum of three products per coefficient split, the member's intended load terms (§1), the station transfer at p, and the stress factors at p. It is evaluated in ball arithmetic (T4-U1b's), with midpoint m_g and radius r_g.
  - **Straight members** use a new FK certificate.
  - **Arcs** use O-13's certificate structure (Δ through K·u = [−H·f_j; f_j], so rigid motion cancels exactly), with K_int,arc in place of the L7′ ball. The rows then belong to the same system as ρ.
  - **Connectors** use `connector_matrix`.
- **The published value** is v_g = fl(m_g), each row rounded once. Magnitude rows are formed at p from their components, then rounded.
- **The bound** is B_g = ‖v_g − m_g‖ + ‖r_g‖ + ‖R_g‖, rounded upward.

Under A1, B_g bounds ‖v_g − g_int(u*)‖. Its terms are the rounding to binary64 (½ ulp), the evaluation radius (about 2^-120 of |A||u|) and the third-order remainder.

### 3.2 The groups (RV33 finding 3)

The groups are every published member row, for straight members, realized arcs and connectors:

- end rows, endpoint section cuts and the three interior stations;
- per location, four groups: |N|, hypot(V_y, V_z), |T| and hypot(M_y, M_z);
- stress and pipe-wall rows, judged through the resultant they scale.

**Support reactions** are recovered at p as K_int,rf·(u + w + w₂) + K_int,rc·u_c − f_int,r at restrained rows, and k·(u + w + w₂) for springs. They form two groups per support: **force** (F_x, F_y, F_z) and **moment** (M_x, M_y, M_z). Revision 0 mixed N and N·m in one group, which made the decision depend on the units.

Displacement and rotation rows are published unchanged (u). K-D5's per-DOF rule judges them, as today. The bound covers them (the 286 breaching rotation and displacement groups in §4.1), but they are not repaired.

### 3.3 The decision (S7)

One predicate replaces `rb_prime_fires`, O-13's `arc_group_fires` with its B_u, and the short-bend rule:

  fires(g) ⟺ ¬finite(B_g) ∨ [ exceeds(B_g, q_g) ∧ S*_g ≥ 2^-988 ∧ q̂_g ≥ 2^-34·S*_g ] .

- **q_g** is the 2-norm of the published values, and exceeds(B, q) is today's exact B > RD(1e-9)·q.
- **The floor operand (S7)** is q̂_g = max(q_g, ‖m_g‖ + ‖r_g‖ + ‖R_g‖), the certified upper bound of the row's true magnitude. A gross error cannot shrink a judged row below the floor and hide.
  - On every evaluated set, the only rows that leave the floor under repair are rows whose true value is below it. Example: RV30's 26 C-family symmetric midspan shear rows, at about 1e-16·S* in truth, published today as noise at about 1e-10·S*.
- **S*_g** is the body's coupled scale over the published (repaired) rows: the moment scale for bending, torsion and support moment groups, and the force scale for axial, shear and support force groups. This is O-13's scale extension, which includes arcs.
- **No resolution conjunct.** R-2's behaviour now applies to every member.

**The consequence.** If any group fires, the case is demoted by R-b′'s mechanism:

- one sentence, naming at most six groups (member or support, location, group, q, B), each item no longer than today's per-end item;
- `CHECKS_PASSED` becomes `SENSITIVE`;
- the no-op rule holds, and the load-row sentence stands first (I5 ruling 1).

A case that K-D5 demotes or cannot check keeps today's binary64 rows and today's bytes. A case that is not pre-guard Passed publishes no certified rows.

**Combination rows (RV33 note 10).** Combination rows are binary64 linear combinations of the certified case rows (`append_combination_results`). The 1e-9 criterion governs case rows. A combination row is not judged against its own q, which under cancellation can be much smaller than its terms. This unit does not change that. It is disclosed, and listed under open items.

### 3.4 The published values change (property)

**Property.** Member rows (end actions, section actions, station rows, stresses) and support reactions of a load case published `NUMERICAL_INTEGRITY_CHECKS_PASSED` on the linear route are certified recoveries. They are not binary64 recomputations from the published displacements.

- Last bits change on every such case: 100 % of the evaluated Passed case-modes.
- In 21.8 % of RV30 case-modes (23.0 % realistic, 21.3 % on W1's straight set), some row moves by more than its criterion. These are today's breaches, repaired.
- The per-case largest change |v_d − v|/S*:

| Set | Median | p90 | p99 | Maximum |
|---|---|---|---|---|
| RV30 | 9.2e-13 | 3.6e-11 | 4.0e-10 | 6.3e-9 |
| Realistic | 6.3e-13 | 4.9e-11 | 3.7e-10 | 5.9e-10 |

A reader recomputing K_e·u from the published displacements in binary64 will therefore differ from the published rows by about 1e-12·S*, and by up to about 6e-9·S* on stiff short members.

**Limitation sentence (draft for the implementation).** This sentence goes in the linear profiles' `FormulationBasis.limitations`. It avoids the registered boundary vocabulary; run the claims lint on it.

> "Member actions, section actions, stresses and support reactions of a load case reported NUMERICAL_INTEGRITY_CHECKS_PASSED are recovered at 128-bit precision from the formation check's corrected displacement and rounded once; they are not binary64 recomputations from the published displacements, from which they may differ by about 1e-12 of the body's action scale (at most about 6e-9 in the evaluation sets)."

## 4. Evidence (emulation; the implementation confirms)

**Method.**

- **Probe and reference.**
  - I41's probe binary (built at d92850097d, observe mode) was re-run on RV30's 22,560 models, both modes. It dumps K-D5's u, w, w₂, ρ, s, s₂, the scale exponents, r̂ and ‖K̃_s‖₁.
  - I41's realistic and RV29 dumps were reused.
  - RV30's exact reference (80-digit Decimal; I41's extension for RV29) gives u* and the exact rows.
- **The emulation** (`tools/i43_eval.py`):
  - **The closure.** The intended system is the reference: loads exact from their primitives, so the closure is perfect except for input representation. Section-property rounding moves the truth by at most 3e-8·c (RV33).
  - **The closed corrections.** ρ_c = K·e is exact. w_c and w₂_c come from a binary64 LU of the scaled reference stiffness, which stands in for the attempt's factor and affects tightness only. s_c and s₂_c are exact.
  - **Λ** is formed from the product's est with the stand-in ε_E := 2^-40·‖K̃_s‖₁, about 4,096 ε relative and above binary64 element formation.
  - **r_g** := 2^-100·‖A·D‖·‖D⁻¹u‖, conservative against 2^-120.
  - **Judging** is RV30's grouping, with RD(1e-9) and the floor as in §3.3.
- **Sets:**
  - RV30, all families: 41,085 pre-guard-Passed case-modes;
  - realistic: 921;
  - RV29: 90;
  - a new W1-domain straight set: 1,422 models (2,844 case-modes), described in §4.4.
- **Evidence:** `WT/scratch/t4_code/t4_i43/`:
  - `tools/`: `i43_eval.py`, `analyze.py`, `extra.py`, `d5c1.py`, `noclosure.py`, `gen_w1.py`, `driver.sh`;
  - `analysis_*.txt`, `extra_*.txt`, `d5c1.txt`, `noclosure.txt`;
  - `eval/`, `w1/`.

### 4.1 Soundness: B_c ≥ err on every breaching group (T3's M2)

The closed bound is applied to today's published rows v: B_c = ‖v − m_g‖ + ‖r‖ + ‖R‖, against err = ‖v − g(u*)‖. Since B_c − err ≥ ‖R‖ − ‖A(e − w − w₂)‖, the remainder ratio decides it.

| Set | Breaching groups | of which displacement or rotation | B_c < err | min R / ‖A(e − w − w₂)‖ | max ‖A(e − w − w₂)‖/c |
|---|---|---|---|---|---|
| RV30 | 87,375 | 286 | 0 | 4,756 | 1.2e-9 |
| Realistic | 3,359 | 0 | 0 | 2,763 | 6.9e-11 |
| RV29 | 190 | 0 | 0 | 2.1e8 | 2.5e-14 |
| W1 straight set | 16,609 | 0 | 0 | 22,262 | 9.0e-12 |

There are no exceptions on the 90,734 groups (RV30 and realistic). The same closed bound applied as a guard to today's rows would fire on exactly the 8,942 RV30 case-modes with a breaching member or reaction group, with no miss and no false demotion. Revision 0, without the closure, had 12 misses and 16 non-breach demotions. The other 4 of RV30's 8,946 breaching case-modes breach only through displacement or rotation groups (RV33 note 9).

### 4.2 Under (d): residual demotions, verdict moves and D5C-1

| Set | Pre-guard Passed | Demoted under (d) | Worst judged B/c | Passed with a breach | Sensitive today → Passed (with a breach of the repaired rows) |
|---|---|---|---|---|---|
| RV30 | 41,085 | **0** | 0.023 | 0 | 7,608 (0) |
| Realistic | 921 | **0** | 0.012 | 0 | 48 (0) |
| RV29 | 90 | **0** | 1.2e-5 | 0 | 34 (0) |
| W1 straight set | 2,561 | **0** | 0.0062 | 0 | 64 (0) |

- **The Sensitive → Passed moves** are today's guard demotions.
  - RV30: short-bend rule 7,003, arc guard 399, both 206; 2,827 of them breach today.
  - Realistic: 36, 6 and 6.
  - RV29: 14 short-bend and 20 straight R-b′.
  - W1 set: 64 straight R-b′.

  Every one publishes repaired rows within its criterion.
- **T3 estimated about 0.09 % on RV30** by counting R > c at today's q. In this evaluation that count is 26 case-modes, all of them the C-family symmetric-midspan rows of §3.3. Repaired, they leave the floor because their true value does, so (d) judges none of them.
- **Fail-closed conditions.**
  - Contraction is at most 1.7e-9 against the 2^-20 = 9.5e-7 limit.
  - 2^10·est·ε_E is at most 0.062 with the stand-in ε_E, at r̂ = 1.5e-8.
  - Λ ≥ the true ‖K_s⁻¹‖_∞ on every case-mode.
- **D5C-1 fold.** With the closed w, the per-DOF rule demotes no case-mode that the ledger w did not, and stops demoting none. The largest trigger ratio is 0.998 against 0.998 (RV30), 0.994 against 0.994 (realistic) and 0.97 (RV29).

### 4.3 Why the closure is required, and M3

**(d) without the closure.** This variant uses the ledger's w, w₂ and s₂ in m_g. It publishes repaired rows that keep the load-formation gap, and its bound cannot see the gap.

| Set | Passed with a breach | of which Sensitive today | Worst err/c |
|---|---|---|---|
| RV30 | 2,215 | 713 | 3.9e4 (A-family thermal T at utm5-skew, torsion) |
| Realistic | 76 | 10 | 3.8e3 (STD-U t24 wT utm-skew, section-cut torsion) |
| RV29, W1 set | 0 | 0 | (nodal loads are exact) |

**T3's case** `B|L|1-4|t96|R1|rho0.016|k5|wTp|utm73-skew`:

- Today it is Sensitive by the short-bend rule, with breaches of 1.10 (sparse) and 1.75 (dense) × c.
- Under (d) with the closure it publishes Passed, with repaired rows at err/c = 1.03e-7 and B/c = 1.03e-7 in both modes.

So the short-bend rule is removed in the same unit as the closure and after it. On every evaluated set, no case moves from Sensitive to Passed with a breach.

### 4.4 W1's N-5 abandonment (T3's M1)

**The mechanism.** A straight-member recovery demotion on a W1 invocation sets `recovery_demoted` (`lib.rs` `observer.ordinary_recovery_demoted`). T-4's `case_triggers` reads the published `solve_quality` after R-b′ and attempts the case. Then `retained_wire.rs` records `ordinary_attempts[].formation.recovery_finding` as untranslated, and W1 declines the whole retained successor (`b1_sp_w_c2_transaction_faults_and_abandonment`).

**The set.** `tools/gen_w1.py` builds 1,422 invented straight-only models in `source_recovery`'s admitted family, beyond the B1 corpus:

- straight frames, end anchors (or one anchor), concentrated nodal forces and moments only;
- chord twins of the L, U, Z and 4-bend loop lines at Rφ/L from 0.008 to 90°, sharp-corner twins, and skew multi-span runs and cantilevers;
- t7, t24 and t96 sections; origin, utm73-skew, utm5-skew and skew2 placements.

| | Invocations | Refused | Sensitive at the solve | Pre-guard Passed | Recovery demotions (abandonments) |
|---|---|---|---|---|---|
| Today (R-b′) | 2,844 | 108 | 175 | 2,561 | 64 |
| Revision 0, as a guard on today's rows | 2,844 | 108 | 175 | 2,561 | 533 |
| Revision 1 (d) | 2,844 | 108 | 175 | 2,561 | **0** |

Every fail-closed condition is decided in FK as K-D5 unavailability, so it follows W1's K-D5 path (`d5_diagnostic_ref`, translated), not N-5. On W1's domain only straight frames occur, so every member's certified recovery is formable. A PP-side firing would still abandon W1; its predicted rate is 0. N-5 is left unchanged.

### 4.5 Committed requests (no exact reference; predicted)

- **Integrity codes that move to Sensitive:**
  - the nonlinear-route cases (§5);
  - bodies whose every row is zero in the intended system: SP-1 set E's 16 load-reference qualification case-modes (free expansion, rows today about 1e-10 N against 0) and O-13's `p8_a_stress_free_arc_stays_passed`.

  Under (d) those rows become third-order values against an exact zero, and the unchanged criterion fires on them unless they come out exactly zero. The implementation confirms which. This is the consequence of keeping the criterion and floor, as ruled.
- **Predicted to stay Passed** with repaired rows (they demote under revision 0):
  - I7's `U2-L-FREE-P-K2-SKEW-X5E6` and `-X7P3E6`;
  - `prescribed_rotation_all_fixed`;
  - the 1e-4 rad arc of t15b.
- **Every Passed linear-route case changes member-row bytes** (§6).

## 5. The nonlinear-support route

**Fail closed.** The route has no K-D5: no `FormationCheckedSystem` is built when nonlinear supports exist (ROOT's ruling). With (d) the unified predicate has no B_g there. A converged nonlinear case whose report would be `CHECKS_PASSED` is published `NUMERICAL_INTEGRITY_SENSITIVE`, with the sentence:

> "the member-relative formation check is not available on the nonlinear-support route; member rows are binary64 recoveries"

Its rows stay today's binary64 rows. The `RECOVERY_BASIS_UNQUALIFIED` branches are unchanged.

**Committed cases moving to Sensitive.** These are the two models with nonlinear supports that publish Passed today, `fixtures/product_preview/invented_demo_model.json` and `core/product_physics/tests/fixtures/preview_physics_invented_model.json`. Each has two cases (`load:L-100`, `load:L-200`), in both modes: 8 case-modes. They appear in 5 committed outputs:

- `invented_demo_result_preview_physics_1_{sparse,dense}.json`;
- `invented_demo_result_legacy_0_1.json`;
- `fixtures/results/preview_physics_invented_{sparse,dense}.json`.

That is 10 integrity diagnostics. The desktop demo's cases show Sensitive.

**The next T4 unit: K-D5 on the converged active set.** At convergence the selected state solves a linear system: K plus the active supports' stiffness or constraint rows, with the closed and gapped supports fixed.

- Build its `FormationCheckedSystem`, with active support springs as springs, active gaps as prescribed or eliminated rows, and friction as its linearized state.
- Run K-D5 and (d) unchanged on it.
- Add one condition: the active set is certified. Each inactive support's gap, and each active support's contact force, has the right sign with margin above its own bound (a ball on the support reaction rows). Otherwise the case fails closed with a named reason.

The cost and the W1 relation (the route is outside D1) are that unit's.

## 6. Bytes, pins and pricing (SP-4 forecast)

**Bytes.**

- **Member rows and support reactions** change bits on every Passed linear-route case. 36 committed linear output fixtures carry Passed member rows (`fixtures/results/*`, `fixtures/product_preview/{load_reference,load_reference_source,physics_source}/*`, `core/reporting/result_export/tests/fixtures/source_blocks_statement_control.json`). The ordinary base of the retained successors changes too.
  - Displacements and every Sensitive case are unchanged.
  - Each regenerated output is declared under SP-1, and the SP-1 corpus bases (sets E, B1, Z, F, V, D) are re-based.
  - None of the registered profile's `reviewed_inputs` fixtures carries a Passed integrity code (checked), so the profile identity line does not move for this reason.
- **Integrity codes** change per §4.5 and §5.
- **TEXT:**
  - the unified sentence;
  - the nonlinear fail-closed sentence;
  - the new K-D5 unavailable details (closure, contraction, Neumann, second solve);
  - the limitation sentence in the linear `FormulationBasis`.

**Pins.**

- S11-G: `rb_prime_clauses_at_their_boundaries`, `t11`, `t12`, `t13`, `t13b`, `t20` and `ruling1_both_guards_fire_and_the_load_row_sentence_stands`.
- O-13's `arc_recovery` P1–P8 and G1–G6. The short-bend tests are deleted.
- `t15b`, re-pinned to its outcome.
- F1b's W2 and K2a reach tests that read R-b′'s sentence.
- The S11-F site table (`s11f_site_test` rule 6, and FK's `s11_site_table`): every new function that forms a load at p, and the recovery at p, is a force site.
- straight_pipe's `t17` retires with `bending_formation_bound`'s guard use.
- FK `formation_check_tests` (`kd5_*`), for the closure and the second correction.
- `retained_wire_tests` and the retained façade tests are unchanged (N-5 stays).

**Pricing (registered profile; T3's repo tool `tools/retained_pricing`, `price.py run <head>`).** Re-registration is pre-approved within M = 10.5 GiB (11,274,289,152 B) and the 5 % rule. The rows that change are forecast below. Each is small at the D1 caps (n = m = g = 32, F = N = 192), against the 8.1 % (dense) and 9.6 % (sparse) of TAV_W that the 0.9 M rule leaves for text error. The tool's run on the implementing head gives the numbers.

| Row (`base/ordinary_caps.py` and its tree) | Change |
|---|---|
| "Ordinary formation check (outer vectors)" | + 2·Xc(ExactAccumulator, F) for s and s₂; + B(8·5F) for the radius vector ψ, w₂, the second right-hand side and the ε_E row and column sums; the per-row allowance replaces the `Vec<f64>` lists; + V(LoadPrimitive, l) |
| "H_formation128 (T06: heap 0)" | Stays 0 only if every re-formation is on the stack: K_int per element for ρ, s, s₂, ε_E and recovery, and load primitives at p, with no `Vec<Element>` cache (S5). A test pins it |
| StructuralSolution (now priced, S5) | + B(16·N) for w and w₂, + s(MemberBoundInputs), live from the completion through the recovery stage |
| W2 "evaluate" | It is ×2 over core_total, so it counts the larger formation-check row twice (T3's double count). The tail gains the unscaled member-bound inputs once: + B(16·N) |
| "Straight member recovery" | V(RecoveryRecord, m) becomes V(RecoveryRecord, m + a + c + g) (a = c = 0 under D1). s(RecoveryRecord) grows by size_of to 7 locations × 4 groups for members and 2 groups for supports. The certified recovery uses the stack only |
| "Formation-guard late maps and RecoveryFinding" | Coupled force and moment scales per body (O-13's ActionScales) |
| Text(recovery_finding), Text(formation_detail), D, D_env | Re-run the TEXT chain: new sentence items (kept no longer than today's), new detail strings and the limitation sentence |
| New FK loops (load primitives; the s and s₂ passes; ε_E over contributions; the recovery locations) | O-10 loop rules (`O10_LOOPS_LAST`), with their arguments in DEL-05-02's `retained-pricing-rules.md` |

The run is `run <head>`, then `compare` against the registered tree, then `generate`. The `profile_` law tests are re-pinned.

## 7. Implementation plan

**FK `structural/formation_check.rs`:**

- `LoadPrimitive` enum (§1) and `FormationSource::loads`.
- In `evaluate`:
  - the closure rule over the ledger's formation records;
  - the radius vector ψ;
  - the s, w₂ and s₂ passes, with each element re-formed on the stack;
  - ε_E;
  - the contraction and Neumann checks;
  - D5C-1 on the closed w.
- Return `MemberBoundInputs { w, w2, exponents, lambda_sigma }`, or `FormationCheckUnavailable` with a named detail.

**FK `structural.rs`:**

- `estimate_rcond` keeps its signature. `ConditionMatrix::estimate_condition` returns (r̂, est, ‖K̃_s‖₁).
- `finish_checked_factor` passes est and the contribution list to the check.
- `StructuralSolution.member_bound: Option<MemberBoundInputs>`, never in `StructuralReport`.
- The W2 entry passes w and w₂ through and unscales Λ·σ upward (S3).

**FK certificates (new `structural/certified_recovery.rs`):**

- `certify_straight_recovery(frame, loads, u, w, w2)` for end rows, endpoint section cuts and the three stations, as balls.
- `certify_curved_recovery`, O-13's, gains the correction terms and uses K_int,arc.
- Connectors and support reactions as §3.2.

**NI `structural_adapter.rs`** (`formation_source`, `solve_assembled_with_formation_check`) attaches the load primitives that the PP producers hand over with their ledger terms.

**PP `lib.rs`:**

- The producers of §1's table push their `LoadPrimitive` beside each formed term.
- The recovery loop publishes fl(m_g) for every member row (straight, arc and connector branches), the stresses scaled at p, and the reactions, when `member_bound` is present and the case is not Sensitive. It forms one `RecoveryRecord` per member and per support.
- The nonlinear route fails closed (§5).
- The short-bend rule is removed. `bend_neighbour_ratios` stays for the refusal.
- The linear `FormulationBasis` gains the limitation sentence.

**PP `formation_guard.rs`:**

- `member_group_fires(q, q_floor, B, S*)` replaces `rb_prime_fires` and `arc_group_fires`.
- One `RecoveryRecord` shape: member or support, body, and per location and group (q, q̂, B), or the unavailable reason.
- One finding sentence.
- `short_bend_finding`, `ShortBend`, `SHORT_BEND_PASSED_RATIO` and `RESOLUTION_FACTOR` are removed.

**Readers.** No field moves; only values and codes change, so no reader changes.

**Tests and mutants that must be killed:**

- **Each B term:**
  - drop ‖A·w₂‖ (in m_g, evaluate at u + w);
  - drop R, or use s in place of s₂;
  - drop ψ;
  - drop the 2^10;
  - use 1/(1 − a) in place of the ε_E Neumann factor;
  - drop D;
  - use ‖·‖₁ in place of ‖·‖_∞ on a non-symmetric K_int fixture.
- **The closure:**
  - a formed term without a primitive must fail closed;
  - a ledger value substituted for a primitive (the realistic STD-U wT utm-skew line kills it at 3.8e3·c);
  - each primitive family has a test against an exact reference.
- **Repair:**
  - publishing v instead of fl(m_g);
  - m_g at u;
  - rounding magnitudes after their components;
  - the K2b unscaling of w (must not happen) and of R (must).
- **The decision:**
  - `≥` in place of `>`;
  - a rounded comparison;
  - the floor conjunct, and q in place of q̂;
  - the 2^-988 guard;
  - the resolution conjunct reintroduced (RV29's cantilever kills it);
  - group norm replaced by a maximum;
  - the force and moment reaction groups merged.
- **Fail closed:** the contraction limit, the Neumann limit, the second solve, the closure and the nonlinear route each demote, with the named detail.
- **The no-op rule**, and the load-row and recovery sentences together.
- **Source pins** tying the call sites to the tested predicate and to the stack-only re-formation (T06).
- **Natural positives:**
  - RV29's 10 m cantilever, which is repaired and stays Passed, with a test asserting its rows are now within c;
  - a no-closure kill: the A-family thermal L line at utm5-skew.
- **Natural negatives** (Passed with repaired rows):
  - T3's M3 case;
  - the realistic STD-L, STD-U and loop lines.
- **The W1 relation:** a W1-domain straight invocation that R-b′ demotes today publishes Passed, with no N-5 abandonment.

## Open items

- **All-zero bodies.** Free expansions and stress-free arcs stay Sensitive under the unchanged criterion (§4.5) unless their certified rows are exactly zero. This is a consequence of the ruling, recorded here. It is not a new choice.
- **Combination rows** are not judged against their own q (§3.3).
- **ε_E on curved elements at UTM** is measured by the implementation. The emulation used a stand-in. Failure would show as a fail-closed rate above 0 on the realistic set.
- **The emulation's intended loads** are RV30's reference definitions. The implementation's primitives must match them, including the effective-force pressure model, or the closure test against the exact reference fails.
