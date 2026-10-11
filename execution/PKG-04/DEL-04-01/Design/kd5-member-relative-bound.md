# K-D5 member-relative bound and the unified R-b′ predicate (T4-KD5)

Revision 3. It implements HELP_HUMAN's ruling of 2026-10-11, made under the owner's delegation: **option (d), "repair"**, and HELP_HUMAN's later ruling of the same day on all-zero bodies. Revision 0 (68d78411af) derived the bound; revision 1 (5fec6bb9c0) made the unit publish what the bound certifies and folded in the load closure. Revision 2 applied T3's re-review of revision 1 (I1–I8, S6) and T4-RV34's check, and replaced two stand-ins with measurements taken on the product: ε_E (§2.1) and the remainder under the attempt's own matrix (§4.2). Revision 3 makes the summand-completeness invariant explicit, qualifies the all-zero explanation by the existing minimum-scale guard, corrects the spring-reaction sign, and source-qualifies the surviving combination producer. The numerical criterion is unchanged.

The record extends K-D5 ([curved-numerical-integrity.md](curved-numerical-integrity.md)) and R-b′/O-13 ([arc-recovery-guard.md](arc-recovery-guard.md)). It changes no product code. Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

**Result.**

- **One intended system.** K-D5's intended system is its p = 128 re-formation of the stiffness and of every formed load term from its binary64 primitives (§1). The residual, both corrections, the bound and the published member rows all use that one system. No ledger value of a formed load term enters any of them.
- **Repair.** A member row of a case that would publish Passed is published as its certified recovery: the intended recovery m_g at the corrected displacement u + w + w₂, rounded once to binary64 (§3). Its bound is the rounding, the evaluation radius and the third-order remainder.
- **Soundness.** On all 90,734 breaching groups of the RV30 and realistic sets and RV29's 190, B ≥ err (§4.1). With the attempt's own matrix and the measured ε_E, every published repaired group of every set also has B ≥ err (§4.2).
- **ε_E (T3's I4).** Measured as the implementation will form it, ε_E ≤ 4.9e-13·‖K̃_s‖₁ on every case-mode of every set, at least 30 times below the 2^-36·‖K̃_s‖₁ that would let the Neumann condition fail. The go/no-go threshold (2^-40·‖K̃_s‖₁) holds: **go**. No case-mode fails closed, including RV30's 7,209 short-bend case-modes (§2.1).
- **Residual demotions under (d), with the attempt's own matrix and the measured ε_E (T3's I5):**
  - RV30: 0 of 41,085 pre-guard-Passed case-modes;
  - realistic: 0 of 921;
  - RV29: 0 of 90;
  - W1 straight set: 0 of 2,561.

  The worst judged B/c is 0.032, so the margin is at least 31 (§4.2).
- **W1's N-5 abandonment (T3's M1).** (d) abandons none of the 2,844 invocations in W1's domain; R-b′ abandons 64 of them today (§4.4).
- **Published values change.** Member rows of a Passed case are certified recoveries, not binary64 recomputations from the published displacements (§3.4). Nonzero evaluation cases generally change last bits; zero-load and exact-result cases can retain identical member bytes. In 21.8 % of RV30 case-modes (23.0 % realistic) a row moves by more than its criterion: those are today's breaches, now repaired.
- **All-zero bodies.** The criterion and its floor stay as ruled. A body whose true rows are all zero is Sensitive when a certified row is nonzero and its coupled force or moment scale reaches the existing 2^-988 guard. A resulting demotion carries the notice only when every group of the case meets §3.3 condition 3; a loaded group or body suppresses it. The notice is `STRESS_FREE_WITHIN_NUMERICAL_PRECISION` (§3.3). Up to 82 committed case-modes of SP-1 set E are such bodies (§4.5).
- **The closure is necessary.** Without it, (d) would publish Passed breaches on 2,215 RV30 and 76 realistic case-modes (§4.3). The short-bend rule is removed only with the closure, in the same unit (T3's M3).
- **Fail closed.** The nonlinear-support route, a Passed case without the member-bound inputs, and K-D5 without a ledger each fail closed with a named reason (§2.3, §5). On the nonlinear route, 8 committed case-modes move to Sensitive.

## The rulings and what they supersede

**HELP_HUMAN, 2026-10-11 (under the owner's delegation), option (d).** Publish the certified member rows recovered at p (the intended recovery m_g at u + w + w₂, rounded to binary64), with T3's full load closure folded into D5C-1, so that K-D5 certifies one intended system. The 1e-9 criterion and its 2^-34·S* floor stay, with no interim criterion change. Every formed load term is covered, and B ≥ err is shown on all 90,734 breaching groups. "Certified" means against K-D5's intended system. The published values change: this is stated as a property, and the SP-1/SP-4 blast radius is declared before any re-pin (§3.4, §6). The residual demotion rate and M1's W1-abandonment rate are reported (§4). The short-bend rule is removed only after the closure lands, in the same unit (M3). The nonlinear-support route fails closed with a named, disclosed reason (§5). Re-registration is priced with `tools/retained_pricing/` (§6).

**HELP_HUMAN, 2026-10-11, all-zero bodies.** The floor stays. The reason given is that keeping the criterion carries less risk of false conclusions, while anchoring the floor to the load scale could certify unchecked small-but-meaningful rows. The unit adds a named notice with no criterion change (§3.3).

**Superseded by this unit.** HELP_HUMAN's ruling supersedes "my short-bend thresholds (ruling 5, plus the withdrawn (c) limit)" and "the t15b 1e-4 rad acceptance", and asks that they be named "in the DEL-04-01 Design as superseded by this unit". They are:

1. The Passed and Sensitive thresholds of "Short stiff bends" ([curved-numerical-integrity.md](curved-numerical-integrity.md)): Passed at 6.6e-3 and Sensitive from about 6e-3 down to 5e-5. They become outcomes of the solver's own gate (r̂ < √ε) and of the unified predicate. The pivot-screen refusal and `CURVED_BEND_SHORT_RELATIVE_TO_NEIGHBOURS` below 1e-4 stay.
2. The withdrawn option (c) limit: O-13's `SHORT_BEND_PASSED_RATIO` = 1.6e-2 with the short-bend rule (`short_bend_finding`, `ShortBend`) on `codex/piping-t4-o13-20261010`. It is removed after the closure, in this unit (§4.3).
3. The t15b acceptance at 1e-4 rad ("guard and K-D5 margin from φ = 1e-4 rad", pinned by `t15b_certified_arcs_pass_at_utm_and_small_angles`). A small arc's verdict is now the unified predicate's outcome on its certified rows. t15b is re-pinned to that outcome, predicted Passed.

Both Design files name these supersessions with the same quotations.

## The defect

- **K-D5 is not member-relative.** K-D5 bounds the forward error of u per DOF against max(|u_i|, S*_kind). Member rows come from K_m·Δ_m. A row small against its member's |K_m||Δ_m| cancels in recovery, so an error in u that passes K-D5 becomes a row error far above 1e-9·q.
- **R-b′ masks gross errors.** `rb_prime_fires` requires q > 2^10·B. RV29's skew cantilevers therefore publish Passed with errors of 6.1e5–7.3e5 × RD(1e-9)·q.
- **O-13's short-bend limit is geometric.** It is defeated by inert nodes, soft legs and chord twins (RV29, RV30).
- **K-D5's residual takes the ledger's formed loads as intended.** Load formation propagates into rows unseen (RV33 finding 2; T3's M2). §4.3 measures it.

## 1. The intended system and the load closure (T3's M2, S1, S2)

### Definitions

- **Certified** means certified against K-D5's intended system (S1): the linear problem K_int·u* = f_int.
  - K_int is K-D5's p = 128 re-formation of every stiffness contribution from its binary64 primitives, as today: straight frames, realized arcs, objective connectors and springs.
  - f_int is the p = 128 re-formation of every load term from its binary64 primitives.
  - The input representation is part of the intended input: binary64 coordinates, section dimensions, E, G, ν, α, ΔT, densities, g, p, k and loads as authored. The distance between K_int's arc and the real-number element stays K-D5's existing premise (2^-120·κ_tip, not certified).
- **One intended system (S2).** The residual ρ = f_int − K_int·u, the correction residuals s and s₂, and the recovery m_g of every published row all use the same re-formed matrices and load terms. No formed ledger value enters any of them.

### The closure rule (fail closed)

FK's `FormationSource` gains `loads: Vec<LoadPrimitive>`. Each primitive names the ledger term it contributes to. **One ledger term may carry many primitives**, of different kinds and pipes: `push_exact_pressure_operands` pushes one operand per DOF that sums terminal caps, bend terms, kink remainders and straight Poisson pairs over every pipe meeting the node. Every ledger term falls into exactly one class:

- **Input term.** A term with no S11-G formation record: a nodal force or moment, a constant effort, a prescribed displacement value. It enters as its exact binary64 value, which is its intended value.
- **Formed term.** A term with a formation record. It is replaced by the sum of its primitives' re-formations at p, each added to the row accumulators as the exact split of its Wide2 value. Enclosure radii are carried in ψ (§2).

A formed term whose primitives do not cover it, or a primitive whose re-formation fails (a `WideError` or a failed R-1 precondition), makes the check unavailable: `FormationCheckUnavailable { "load closure: <source> …" }`. Coverage is decided by the ledger's own formation records, not by a hand-kept list or by whether a term has any primitive.

**Complete summands (HELP_HUMAN's delegated technical ruling, 2026-10-11).** Each formed ledger term has a closure manifest of expected summand identities. The identity is the ledger term's index plus its producer's stable summand identity (family, member or support identity, and ordinal within that producer's authored or factor group). Each expected entry also binds the primitive to its immutable source operands: the family and source slot, sign and coefficient, geometry reference, and exact binary64 operand bits. Supplied operand bindings are compared exactly; a matching identity attached to the wrong pipe, direction or operands fails closed. The manifest is established by the producer's complete source/factor-group enumeration, before and independently of collecting the supplied `LoadPrimitive` entries. It is retained with the ledger formation record. The supplied primitives reference these identities; FK compares their identity multiset exactly with the manifest before any residual or recovery evaluation, on both the ordinary and W2 force-scaled routes. Missing, duplicate, extra, mismatched-term, wrong-operand-binding, and absent-manifest entries make the check unavailable with a named load-closure detail. Input terms have no formation record and may not carry a closure primitive.

The expected identities must never be derived from the supplied primitive vector: doing so would bless a dropped summand. For a grouped pressure term, the manifest enumerates every operand in its producer's factor group, including all pipes meeting the node; dropping one supplied operand leaves the original manifest intact and fails closed. Producer tests independently compare each complete family with its exact reference, and a mutant that drops one of several supplied primitives must fail the closure before publication. Both the manifest and the actual primitive storage are included in re-registration pricing (§6).

**K-D5 without a ledger fails closed.** When the force binding is `Legacy` or `Binary64` (`force_terms` is `None`), there is nothing to close: FK returns `FormationCheckUnavailable { "load closure: no ledger" }`. Today such a case runs K-D5 on the folded force.

### Every formed load term

Producers are in PP `lib.rs` unless named.

| Term | Producer | Owner | Primitive and re-formation at p | Radius |
|---|---|---|---|---|
| Nodal forces and moments, constant-effort support forces | `push_nodal_loads`, `add_constant_effort_support_loads` | DEL-05-05, DEL-04-07 | Input terms: no primitive | 0 |
| Prescribed displacements | `imposed_displacements`, K_fc·u_c | DEL-04-07 | Input u_c; K_fc·u_c is already in ρ (prescribed columns) | as today |
| Uniform loads on straight members, authored | `add_uniform_element_loads` (consistent vector) | DEL-05-05 (application), DEL-04-02 (consistent vector) | `UniformStraight{frame, w}`: wL/2 and wL²/12 in the chord frame from (x_i, x_j, y, w), rotated to global | 2^-120 level |
| Generated self-weight | `self_weight.rs` (`stable_mass_per_length` × g) → `add_uniform_element_loads` | DEL-05-01 | `SelfWeight{pipe, densities, OD, t, insulation, g}`: the intensity w = g·Σ ρ_k·A_k is re-formed at p from the authored inputs, then enters `UniformStraight` or `UniformCurved`. The ledger's γ₄ operand bound is therefore not needed | 2^-120 level |
| Uniform loads on realized arcs (R-1's certified arc loads) | `add_uniform_element_loads` → FK `certify_curved_uniform_load` | DEL-05-05 (with DEL-04-01's certificate) | `UniformCurved{arc, w}`: R-1's midpoint–radius enclosure, which is linear in w and is applied to each term of w's exact split | R-1's radius |
| Free strain on straight members (eigen pair P·x_a) | `add_thermal_equivalent_loads` | DEL-05-01 (ε), DEL-04-02 | `FreeStrainStraight{frame, ε}`: ±E·A·ε along the chord | 2^-120 level |
| Free strain on arcs, K_b·u_free(ε) | `add_curved_bend_thermal_equivalent_load` | DEL-05-01, DEL-04-07 | `FreeStrainCurved{arc, ε}`: K_int,arc·(ε·d) with the same arc matrix as ρ | 2^-120 level |
| T4-U2 pressure free strain on arcs, K_b·u_free(ε_p) | exact-pressure route | DEL-04-07 | `FreeStrainCurved{arc, ε_p}`: ε_p = (1 − 2ν)·p·A_i/(E·A) at p | 2^-120 level |
| T4-U2 pressure operand groups: terminal caps, bend terms −c_b, kink remainder, straight Poisson pairs (`poisson_eigen`) | `pressure_runtime` factor groups → `push_exact_pressure_operands` | DEL-04-07 | One `PressureOperand{kind, pipe, p, OD, t, coefficient, direction}` per summand: p·π·r_i²·c·d with r_i = OD/2 − t, π and the coefficient at p, and directions from the arc's (d, R, y) or the chord at p as K-D5 forms them. The ledger's γ₂₀ bound is not used | 2^-120 level |
| Connector reference loads (T4-U3 q_ref) | `add_connector_reference_loads` | DEL-04-07 | `ConnectorReference{connector, q_ref}`: `connector_matrix`·q_ref | 2^-120 level |
| W2 force-scaled terms | F1b/K2b | DEL-04-07 | The same primitives at 2^b, which is an exact scaling | as above |

**One ε.** ε is the resolved total eigenstrain of the load state: thermal α·ΔT plus fit and cold-spring strain on the 0.4.0 route. It is formed once at p from its primitives, and that one value feeds the straight, arc and pressure primitives of the case.

Under the effective-force pressure model the nodal pressure terms net to zero at bend ends in the intended system. Re-formed at p, they do net. Today their binary64 residue enters ρ as if intended.

**Dependency edges (T3's I3).** The integrator that attaches the primitives is DEL-04-07. Its `deliverable.yaml` carries the four load needs: DEL-04-01 (the `LoadPrimitive` type, the closure and the certified recovery), DEL-05-01 (ε and self-weight primitives), DEL-04-02 (straight-member primitives) and DEL-05-05 (distributed-load application, including R-1's arc loads). Each condition states what that owner must supply. DEL-04-01 keeps the `LoadPrimitive` type as its output and declares no need on its consumers, so the graph has no cycle.

**D5C-1 fold.** K-D5's per-DOF rule runs on the closed w. On every evaluated set no per-DOF verdict changes (§4.2).

## 2. The bound (RV33's corrections, T3's S6, RV34)

### Computation

These steps run in FK `formation_check.rs` `evaluate`. Element matrices are re-formed on the stack at each use, never cached on the heap (S5).

1. **ρ.** ρ = f_int − K_int·u, exactly, as one `ExactAccumulator` per free row.
   - Every coefficient enters through its exact split.
   - The radius vector ψ_ρ collects every split truncated below 2^-1074 (2^-1074·|u_j|) and the load enclosure radii, as a radius, not a shift (RV33 finding 4).
2. **w.** w = D·F̃⁻¹(D·ρ·2^t)·2^-t, with the attempt's own factor, as today. D5C-1 runs on w.
3. **s.** s = ρ − K_int·w, exactly. ψ_s = ψ_ρ plus the truncation allowances of K_int·w.
4. **w₂.** w₂ = D·F̃⁻¹(D·s·2^{t₂})·2^{-t₂}.
5. **s₂.** s₂ = s − K_int·w₂, exactly. ψ = ψ_s plus the allowances of K_int·w₂.
6. **σ.** σ := ‖D·(|s₂| + ψ)‖_∞, rounded upward.
7. **Λ (RV33 finding 1).**
   - `estimate_condition` returns the gate's own (r̂, est, ‖K̃_s‖₁), est = 1/(r̂·‖K̃_s‖₁). These are the gate's values. A recomputed norm differs: I41's dumped norm was 0.4 % off the gate's.
   - ε_E bounds ‖E‖ for E = K_s − K̃_s, where K̃_s is the prepared, scaled and symmetry-projected binary64 matrix the factor sees. It is the triangle bound, formed upward as the larger of the row-sum and column-sum norms of
     - Σ_e |D(K_int,e − K̃_e)D| over frames, connectors and arcs, with K̃_e the element's 144 contributions in assembly order, plus
     - the exact assembly rounding |D(Σ_e K̃_e − K̃)D|, plus
     - the projection change |K̃_s,assembled − K̃_s,projected|.
   - Λ := 2^10·est / (1 − 2^10·est·ε_E), rounded upward.
   - The gate's `a` is not used: it covers assembly rounding only (RV33).
8. **The recovery map.** For each published row, A_r is the row as a linear function of the member's end displacements, and ‖A_r·D‖₁ = Σ_c |A_rc|·2^{e_c}, upward.

### The bound

Let e = u* − u. For any w and w₂, exactly, e = w + w₂ + K_int⁻¹·s₂*, where |s₂* − s₂| ≤ ψ. K_s = D·K_int·D is not exactly symmetric (S6, RV33 note 5), so the bound uses the ∞-norm directly:

  |A_r·K_int⁻¹·s₂*| = |A_r·D·K_s⁻¹·D·s₂*| ≤ ‖A_r·D‖₁ · ‖K_s⁻¹‖_∞ · σ =: R_r .

K̃_s is symmetric, so its 1- and ∞-norm inverses agree, and ε_E bounds both norms of E.

- **A1, the one assumption (RV33 finding 1).** ‖K̃_s⁻¹‖_∞ ≤ 2^10·est. Hager's estimate is a lower bound, so this is an assumption.
  - With A1 and 2^10·est·ε_E < 1, ‖K_s⁻¹‖_∞ ≤ Λ (Neumann).
  - Measured: the Hager ratio for the prepared factor matrix K̃_s lies in [1, 7.17], a margin of at least 140. I44 checks that matrix's inverse; the bound on the intended K_s inverse follows from A1 and the Neumann argument, rather than an independent intended-matrix inverse measurement.
- **The correction solves need no assumption.** s₂ is the exact residual of w + w₂; the solves affect only tightness.

### 2.1 ε_E, measured (T3's I4)

**Method.** The I44 measurement uses ordinary summation with inflation; it measures the triangle-bound scale and cannot substitute for the implementation's rigorous upward accumulation and its tests. A scratch build of the product (t4-u2 at 98dba87714, which contains I41's probe base d92850097d) forms the three contributions to ε_E inside `evaluate` as step 7 states. It walks the 144-entry contribution blocks in assembly order, re-forms each element at p, and also evaluates the ‖E‖ norm directly for comparison. Sets: the realistic set (921 pre-guard Passed, including the loops), RV29 (tiny-angle and UTM), the W1 straight set, and all of RV30 (C/D families and the short-bend family). Evidence: `WT/scratch/t4_code/t4_i44/` (`wt/` probe hook, `tools/i44_eval.py`, `tools/analyze.py`, `analysis.txt`).

**Pass condition.** K-D5 runs only when r̂ ≥ √ε = 2^-26. So ε_E ≤ 2^-36·‖K̃_s‖₁ guarantees 2^10·est·ε_E ≤ 1 at every r̂ the check sees. **Go/no-go threshold:** go if every evaluated case-mode has ε_E ≤ 2^-40·‖K̃_s‖₁, which is 16 times under the pass condition and equal to revision 1's stand-in. No-go if any realistic or short-bend case-mode exceeds 2^-36·‖K̃_s‖₁.

| Set | K-D5 evaluations | ε_E/‖K̃_s‖₁: median | p99 | max | max 2^10·est·ε_E | Fail closed |
|---|---|---|---|---|---|---|
| Realistic | 934 | 2.0e-15 | 6.2e-15 | 6.5e-15 | 2.7e-4 | 0 |
| Realistic loops | 78 | 4.4e-15 | 6.1e-15 | 6.1e-15 | 2.7e-4 | 0 |
| RV29 (tiny-angle 36, UTM 8) | 100 | 1.2e-15 | 4.9e-13 | 4.9e-13 | 1.4e-4 | 0 |
| W1 straight set | 2,564 | 2.7e-16 | 8.2e-16 | 1.1e-15 | 1.9e-5 | 0 |
| RV30, all | 41,310 | 1.5e-15 | 1.8e-14 | 1.8e-13 | 0.010 | 0 |
| RV30 C and D | 11,148 | 1.6e-15 | 9.5e-15 | 1.8e-14 | 5.4e-4 | 0 |
| RV30 short-bend family | 7,209 | 1.6e-15 | 5.4e-15 | 1.6e-14 | 5.4e-4 | 0 |

**Verdict: go.**

- The largest value is 4.9e-13 (RV29 `k10000`, an arc beside a stiff spring): 0.54 of the threshold and 30 times under the pass condition.
- Arcs dominate ε_E. The worst arc entry error relative to the element's largest entry is 9.3e-14. P1's acceptance tolerance of 1e-11 is therefore not approached in practice.
- The triangle bound is at most 3.2 times the exact ‖E‖.
- Revision 1's stand-in was conservative on every case-mode, so its Λ was too.
- The fail-closed rule does not rest on this measurement. A future case above the pass condition is Sensitive (fail-safe), not unsound.

### 2.2 K2b scaling (S3, RV34)

W2 solves K' = 2^b·K_int and f' = 2^b·f_int, formed at 2^b.

- The displacements are unchanged at any b. w and w₂ are displacement corrections and need no unscaling.
- s, s₂, ψ, σ and every force-unit row are formed at 2^b.
- Λ′ is computed from the W2 attempt's own gate and contributions: its r̂′, ‖K̃′_s‖₁, scale exponents D′ and ε′_E. No relation to the unscaled attempt's Λ or D is assumed: D′ is not 2^{-b/2}·D for odd b.
- m_g and R_r are unscaled by 2^-b consistently: m_g by the publication's single rounding (`unscale_for_publication` of the exact value), R_r rounded upward.

### 2.3 Fail closed

The bound is unavailable, and the case is Sensitive with a named `FormationCheckUnavailable` detail, when any of these holds:

- 2^10·est·ε_E ≥ 1 ("Neumann").
- The observed contraction exceeds 2^-20 ("contraction"). The contraction is c₂ = ‖D⁻¹·w₂‖_∞ / ‖D⁻¹·w‖_∞ in the solve's own scaled variables. When ‖D⁻¹·w₂‖ = 0, c₂ := 0 (this covers ρ = 0, where w = w₂ = s₂ = 0 and R = 0). When ‖D⁻¹·w‖ = 0 < ‖D⁻¹·w₂‖, c₂ := +∞.
- s or s₂ leaves the exact-sum range, or the second solve fails or returns a non-finite value ("second solve").
- Λ·σ is not finite.
- The load closure fails, or there is no ledger (§1).

All of these are decided in FK, so the solve itself is Sensitive. That keeps them off R-b′'s recovery-demotion path and off W1's N-5 abandonment (§4.4).

**No Passed case without the member-bound inputs.** PP publishes `NUMERICAL_INTEGRITY_CHECKS_PASSED` on any route only when `StructuralSolution.member_bound` is present. A report that would be `CHECKS_PASSED` without it is published `NUMERICAL_INTEGRITY_SENSITIVE` with the sentence of §5. Today `gate_formation()` returning `None` skips K-D5 silently; after this unit no route passes on that.

## 3. Repair: the published rows and the decision

### 3.1 The certified row

For a group g of a pre-guard-Passed case:

- **m_g** is the intended recovery of g at the corrected displacement u + w + w₂: K_int,m·(u + w + w₂) as one exact sum of three products per coefficient split, minus the member's intended load terms (§1), with the station transfer and the stress factors at p. It is evaluated in ball arithmetic (T4-U1b's), with midpoint m_g and radius r_g.
  - **Straight members** use a new FK certificate. It takes over the role of `exact_straight_end_forces` (K_e·u minus the equivalent terms and the free-strain axial load): the same recipe, at p, with the closure's primitives.
  - **Arcs** use O-13's certificate structure (Δ through K·u = [−H·f_j; f_j], so rigid motion cancels exactly), with K_int,arc in place of the L7′ ball.
  - **Connectors** use `connector_matrix`.
- **The published value** is v_g = fl(m_g), each row rounded once. Magnitude rows are formed at p from their components, then rounded.
- **The bound** is B_g = ‖v_g − m_g‖ + ‖r_g‖ + ‖R_g‖, rounded upward.

Under A1, B_g bounds ‖v_g − g_int(u*)‖. r_g is computed by the ball arithmetic, not assumed (revision 1's emulation used a stand-in of 2^-100·‖A·D‖·‖D⁻¹u‖).

**Which rows publish.** Every pre-guard-Passed case publishes its certified rows, whether or not the unified predicate then demotes it. The demotion sentence quotes q and B of the published rows. A case that K-D5 demotes or cannot check, and every case that is not pre-guard Passed, keeps today's binary64 rows and today's bytes.

### 3.2 The groups (RV33 finding 3, RV34, T3's I2)

The groups are every published member row, for straight members, realized arcs and connectors:

- end rows, endpoint section cuts and the three interior stations;
- per location, four groups: |N|, hypot(V_y, V_z), |T| and hypot(M_y, M_z).

**Pipe-wall rows** form their own groups. The wall axial force is N_w = N_eff + p·A_i, formed at p from m_N and the input product p·A_i, and rounded once. Its group per location has q from the published wall value and B_w = ‖v_w − m_w‖ + r_w + R_N, where R_N is the effective axial row's remainder. The endpoint action rows (`pipe_wall_endpoint_action_v2`) follow the same rule. The membrane stress N_w/A_w is scaled at p and judged through N_w.

**Stress rows** are scaled at p from the certified resultants and judged through the resultant they scale. This covers the axial, bending and torsional components and the component stress-multiplier rows (`result:stress:…:user-multiplier`, the binary64 multiplier times the certified nominal stress).

**Everything derived from member actions derives from the repaired actions (T3's I2).** The governing maximum `pipe_elastic_normal_stress_maximum_v2` and its `pipe_stress_extrema` evidence (PP `lib.rs`, the exact-pressure branch through `exact_straight_summary_extrema`) are today formed from the binary64 `corrected_local_forces` or `exact_mechanical_local_forces`. After this unit they take the repaired end actions instead: fl of the certified m at the governing end, with the member's loads. The same holds for:

- the `result:stress:*` summaries (`straight_summary_extrema` and the station-stress summary);
- the located `max_stress` quantity.

So a published maximum cannot be formed from rows other than the ones published beside it. A test asserts value_upper_pa ≥ every published normal-stress station row of the member on the evaluation sets.

- **The scope string stays.** "supplied_binary64_polynomial_coefficients; solution and coefficient formation error are separate" (PP `retained_product.rs` :2786, RE `physics_evidence.rs` :608, and four other readers) remains true. The coefficients are still binary64 values supplied to the enclosure, now the repaired ones. Their error against the intended system is B, which is separate. Keeping the string leaves the six readers unchanged.
- **The W1 retained-source maximum** (basis `retained_source_endpoint_normal_max_v1`, `source_receipt::composite_member_maximum`) is unchanged. It is published only for cases routed to retained-source recovery: an ordinary report that is Sensitive, an attempt that errs, or a load-row finding (`needs_source_recovery`). Such a case never carries certified rows.

**Support reactions** are recovered at p as K_int,rf·(u + w + w₂) + K_int,rc·u_c − f_int,r at restrained rows, and as −k·(u + w + w₂) for springs, preserving the published support-on-pipe sign convention. They form two groups per support: **force** (F_x, F_y, F_z) and **moment** (M_x, M_y, M_z).

Displacement and rotation rows are published unchanged (u). K-D5's per-DOF rule judges them, as today. The bound covers them (the 286 breaching rotation and displacement groups in §4.1), but they are not repaired.

### 3.3 The decision (S7), the floor and the all-zero notice

One predicate replaces `rb_prime_fires`, O-13's `arc_group_fires` with its B_u, and the short-bend rule:

  fires(g) ⟺ ¬finite(B_g) ∨ [ exceeds(B_g, q_g) ∧ S*_g ≥ 2^-988 ∧ q̂_g ≥ 2^-34·S*_g ] .

- **q_g** is the 2-norm of the published values, and exceeds(B, q) is today's exact B > RD(1e-9)·q.
- **The floor operand (S7)** is q̂_g = max(q_g, ‖m_g‖ + ‖r_g‖ + ‖R_g‖), the certified upper bound of the row's true magnitude. A gross error cannot shrink a judged row below the floor and hide. On every evaluated set, the only rows that leave the floor under repair are rows whose true value is below it (RV30's 25 C-family symmetric midspan shear rows, §4.2).
- **S*_g** is the body's coupled scale over the published (repaired) rows. It is the moment scale for bending, torsion and support moment groups, and the force scale for axial, shear and support force groups (O-13's scale extension, which includes arcs).
- **No resolution conjunct.** R-2's behaviour now applies to every member.

**The consequence.** If any group fires, the case is demoted by R-b′'s mechanism:

- one sentence, naming at most six groups (member or support, location, group, q, B), each item no longer than today's per-end item;
- `CHECKS_PASSED` becomes `SENSITIVE`;
- the no-op rule holds, and the load-row sentence stands first (I5 ruling 1).

**All-zero bodies (HELP_HUMAN's ruling).** A body whose true rows are all zero is Sensitive whenever a certified row is nonzero and the corresponding coupled scale S* reaches 2^-988. This follows from A1, under which a true-zero row has ‖m_g‖ ≤ ‖r_g‖ + ‖R_g‖:

- B_g = ‖v_g − m_g‖ + ‖r_g‖ + ‖R_g‖ ≥ ‖v_g − m_g‖ + ‖m_g‖ ≥ q_g > RD(1e-9)·q_g whenever q_g > 0;
- the largest row sets S* and passes the relative floor; the separate S* ≥ 2^-988 guard must also hold.

A body whose every certified row is exactly zero (S* = 0) stays Passed, for example a zero-load case with u = 0. A body whose force and moment scales are both below 2^-988 can also stay Passed under the existing guard, even with nonzero noise; this unit does not change that criterion or emit the demotion-only notice there. The floor is not anchored to the load scale.

**The notice `STRESS_FREE_WITHIN_NUMERICAL_PRECISION`** (info severity, one per load case, emitted after that case's integrity diagnostic). It is an implementation item of this unit, with no criterion change.

- **Noise-level group.** Let a_g be the group's absolute evaluation scale: the 2-norm over its rows of Σ|t| over the terms t of the exact sum that forms m_g. These are the |K_int,m|·|u + w + w₂| products and the member's intended load terms. a_g is formed upward in the same ball evaluation that forms r_g. The group is noise-level, Z(g), when q̂_g ≤ 2^-34·a_g: the certified upper bound of its true magnitude lies below 2^-34 of the terms that cancel to form it.
- **Firing predicate.** The notice appears for load case c exactly when all three hold:
  1. c is pre-guard Passed with `member_bound` present: K-D5 ran and passed, there is no load-row finding, and no other demotion applies;
  2. the unified predicate demotes c, and every B_g of c is finite;
  3. Z(g) holds for every member and support group of c, judged or not, in every body.
- **"Solely because"** is decided by condition 3. Every group that fires is noise-level, and every S*_g is then at most max q̂ ≤ 2^-34·max a, so S* is noise-derived. A single group with q̂_g > 2^-34·a_g, anywhere in the case, suppresses the notice whatever the standing.
- **What it states.** X = max over the force groups (axial, shear, support force) and Y = max over the moment groups (torsion, bending, support moment) of ‖fl(m_g)‖ + B_g, rounded upward. Under A1 every force row's true magnitude is at most X and every moment row's at most Y. Draft text, to be run through the claims lint (GEN-13) at implementation:

  > "Load case {case}: every member and support force row is bounded by {X} N and every moment row by {Y} N·m at 128-bit precision; the case is stress-free to within numerical precision. The standing remains NUMERICAL_INTEGRITY_SENSITIVE."

- **Constraints.** It never changes the code: the case stays `NUMERICAL_INTEGRITY_SENSITIVE`. No standing logic, W1 path or integrity reader reads it. Its code does not begin with `NUMERICAL_INTEGRITY_`, so readers that select the integrity diagnostic by prefix are unaffected. It does not appear when any row is significant (condition 3).
- **Pins.**
  - Positive: a free-expansion L line under ΔT with one anchor. It is Sensitive, carries the notice, and X and Y are at least every published |row|.
  - Negative control: the same L line with one genuinely loaded member (a nodal force at the free end). There is no notice, whatever the standing.
  - "Solely because": a stress-free body beside a loaded body. The stress-free body makes the case Sensitive, and there is no notice.
  - A zero-load case (u = 0) stays Passed with no notice.

**Combination rows (RV33 note 10).** Combination rows are binary64 linear combinations of the certified case rows (`preview_physics::append_combination_results` in `PP/src/preview_physics.rs`). The 1e-9 criterion governs case rows. A combination row is not judged against its own q, which under cancellation can be much smaller than its terms. This unit does not change that; it is disclosed and listed under open items.

### 3.4 The published values change (property)

**Property.** Member rows (end actions, section actions, station rows, stresses, pipe-wall rows) and support reactions of a load case published `NUMERICAL_INTEGRITY_CHECKS_PASSED` on the linear route are certified recoveries. They are not binary64 recomputations from the published displacements.

- Last bits changed on every Passed case-mode in the nonzero evaluation sets; this is a measurement, not a universal property. Zero-load and exact-result cases can remain byte-identical.
- In 21.8 % of RV30 case-modes (23.0 % realistic, 21.3 % on W1's straight set), some row moves by more than its criterion. These are today's breaches, repaired. The attempt's own matrix changes the repaired values by at most B, which is far below c, so these rates are those of revision 1's emulation.
- The per-case largest change |v_d − v|/S*:

| Set | Median | p90 | p99 | Maximum |
|---|---|---|---|---|
| RV30 | 9.2e-13 | 3.6e-11 | 4.0e-10 | 6.3e-9 |
| Realistic | 6.3e-13 | 4.9e-11 | 3.7e-10 | 5.9e-10 |

- **Pins on mode differences (T3's I8).** Repaired rows are nearly mode-independent. Pins that hold mode-specific member-row values move:
  - W-C2's per-mode hashes;
  - the B2-P successor pins;
  - S11-G's per-mode q and B in t11–t13b and t20;
  - O-13's per-mode verdicts (T3's M3 case: 1.10 sparse, 1.75 dense today);
  - the per-mode committed outputs of §5 and §6.

  `curved_bend_macro_element_keeps_dense_sparse_parity` and the DEC-053 parity row (`sparse_live_path_dense_parity_relative_delta`, which compares displacements) stay. No test requires member rows to differ between modes.

**Limitation sentence (draft for the implementation).** This sentence goes in the linear profiles' `FormulationBasis.limitations`. It avoids the registered boundary vocabulary; run the claims lint on it.

> "Member actions, section actions, stresses and support reactions of a load case reported NUMERICAL_INTEGRITY_CHECKS_PASSED are recovered at 128-bit precision from the formation check's corrected displacement and rounded once; they are not binary64 recomputations from the published displacements, from which they may differ by about 1e-12 of the body's action scale (at most about 6e-9 in the evaluation sets)."

**Scope beside it (T3's I7), in the same `limitations` list:**

> "The recovery is checked against the formation check's re-formed system (stiffness and loads formed at 128-bit precision from the binary64 inputs). The published displacements are the solve's own, judged by the formation check's per-row rule; support reactions balance the re-formed loads, not the solve's binary64 load vector."

## 4. Evidence (emulation and product measurement)

**Method.**

- **Probe and reference.**
  - Revision 1 (I43) re-ran I41's probe binary (built at d92850097d, observe mode) on RV30's 22,560 models, both modes. It reused I41's realistic and RV29 dumps.
  - Revision 2 (I44) builds a scratch probe of t4-u2 at 98dba87714, which contains d92850097d. It dumps K-D5's u, the gate's r̂ and ‖K̃_s‖₁, the prepared matrix K̃_s and ε_E (§2.1), on the same sets plus W1's.
  - RV30's exact reference (80-digit Decimal; I41's extension for RV29) gives u* and the exact rows.
- **The emulation.** The intended system is the reference: loads exact from their primitives, so the closure is perfect except for input representation. Section-property rounding moves the truth by at most 3e-8·c (RV33). ρ_c = K·e is exact; s and s₂ are exact. Two variants:
  - **I43 (revision 1):** w and w₂ from a binary64 LU of the scaled reference stiffness; Λ with the stand-in ε_E = 2^-40·‖K̃_s‖₁.
  - **I44 (revision 2):** w and w₂ from a binary64 LU of the product's own K̃_s. The formation error E then enters the contraction as it does in the product. Λ uses the measured ε_E.

  Validation: the emulated contraction against the product's own (I41's w and w₂ on the ledger residual) has a median ratio of 0.75 (p1 0.03, p99 11). The product's largest contraction is 2.3e-9.
- **Judging** is RV30's grouping, with RD(1e-9) and the floor as in §3.3. r_g is the stand-in 2^-100·‖A·D‖·‖D⁻¹u‖.
- **Evidence:** `WT/scratch/t4_code/t4_i43/` (revision 1: `tools/i43_eval.py`, `analyze.py`, `extra.py`, `d5c1.py`, `noclosure.py`, `gen_w1.py`; `analysis_*.txt`, `eval/`, `w1/`) and `WT/scratch/t4_code/t4_i44/` (revision 2: `wt/` probe hook, `tools/i44_eval.py`, `tools/analyze.py`, `analysis.txt`, `logs/`).

### 4.1 Soundness: B ≥ err on every breaching group (T3's M2)

The closed bound is applied to today's published rows v (I43): B_c = ‖v − m_g‖ + ‖r‖ + ‖R‖, against err = ‖v − g(u*)‖. Since B_c − err ≥ ‖R‖ − ‖A(e − w − w₂)‖, the remainder ratio decides it.

| Set | Breaching groups | of which displacement or rotation | B_c < err | max ‖A(e − w − w₂)‖/c |
|---|---|---|---|---|
| RV30 | 87,375 | 286 | 0 | 1.2e-9 |
| Realistic | 3,359 | 0 | 0 | 6.9e-11 |
| RV29 | 190 | 0 | 0 | 2.5e-14 |
| W1 straight set | 16,609 | 0 | 0 | 9.0e-12 |

- There are no exceptions on the 90,734 groups. As a guard on today's rows, the closed bound fires on exactly the 8,942 RV30 case-modes with a breaching member or reaction group, with no miss and no false demotion. The other 4 of RV30's 8,946 breaching case-modes breach only through displacement or rotation groups (RV33 note 9).
- With the I44 variant, the published repaired rows have B ≥ err on every group of every set (0 violations).
- The certified remainder exceeds the true remainder ‖A(e − w − w₂)‖ on every group by a factor of at least 184 in any emulation so far: 184 in RV34's, 230 in I44's on RV30 (`CORNER|3.0-40.0|t7|90.0deg|w|o-xy`), and 353 on the W1 set. This is a measurement, not a property.

### 4.2 Under (d): residual demotions, the remainder with the attempt's matrix, D5C-1 (T3's I5)

| Set | Pre-guard Passed | Demoted under (d) | Worst judged B/c | R > c at today's q: I43 / I44 | still judged under (d) | Contraction max | Sensitive today → Passed (with a breach of the repaired rows) |
|---|---|---|---|---|---|---|---|
| RV30 | 41,085 | **0** | 0.032 | 26 / 25 | 0 | 1.7e-9 | 7,608 (0) |
| Realistic | 921 | **0** | 0.015 | 0 / 0 | 0 | 5.6e-10 | 48 (0) |
| RV29 | 90 | **0** | 5.1e-6 | 0 / 0 | 0 | 1.1e-9 | 34 (0) |
| W1 straight set | 2,561 | **0** | 0.013 | 0 / 0 | 0 | 3.4e-10 | 64 (0) |

- **The remainder with the attempt's matrix.** T3 scaled I43's R by the ledger s₂ ratio (I41's dump) and counted 40 runs with R > c. That ratio, up to 4.5e5, includes the ledger's load-formation gap in the ledger residual, which the closure removes. On the closed residual, the product's own K̃_s raises ‖D·s₂‖ over the stand-in by at most 2.0e4 (A family; 289 realistic). The judged R > c count at today's q is 25 RV30 case-modes, at most 100·c. All are C-family far-member L lines, rows of symmetric midspan shear whose true value is 1e-16 to 2e-15 of S*. Under (d) every one leaves the floor, so (d) judges none of them.
- **The Sensitive → Passed moves** are today's guard demotions.
  - RV30: short-bend rule 7,003, arc guard 399, both 206; 2,827 of them breach today.
  - Realistic: 36, 6 and 6.
  - RV29: 14 short-bend and 20 straight R-b′.
  - W1 set: 64 straight R-b′.

  Every one publishes repaired rows within its criterion.
- **Fail-closed conditions.** None fires on any set. The contraction is at most 1.7e-9 against 2^-20 = 9.5e-7. 2^10·est·ε_E is at most 0.010 (§2.1). The measured K̃_s inverse satisfies A1 on every case-mode; the intended K_s inverse bound is the conditional Neumann consequence.
- **D5C-1 fold.** With the closed w, the per-DOF rule demotes no case-mode that the ledger w did not, and stops demoting none. The largest trigger ratio is 0.998 (RV30), 0.994 (realistic) and 0.97 (RV29).
- **The load-row guard (T3's I6).** S11-G's load-row guard still demotes on formed-term ledger defects. Under (d) those defects reach no certified row: the closure keeps them out of ρ and the rows, and their effect on u is judged by D5C-1 and the bound.
  - Its role after this unit is a conservative standing demotion and the trigger for retained-source recovery (`needs_source_recovery`). It stays unchanged here, because retiring it changes W1's routing, which is T3's interface.
  - Availability effect: 6 RV30 case-modes (`B|L|0.5-20|t96|R1|rho0.012–0.017|k1|wTp|utm73-skew`, both modes) are Sensitive only through it. Their repaired rows are within criterion (B/c ≤ 1.0e-7). The effect is 0 in the realistic, RV29 and W1 sets. Retiring it is an open item.

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

**The mechanism.**

1. A straight-member recovery demotion on a W1 invocation sets `recovery_demoted` (`lib.rs` `observer.ordinary_recovery_demoted`).
2. T-4's `case_triggers` reads the published `solve_quality` after R-b′ and attempts the case.
3. `retained_wire.rs` records `ordinary_attempts[].formation.recovery_finding` as untranslated, and W1 declines the whole retained successor (`b1_sp_w_c2_transaction_faults_and_abandonment`).

**The set.** `tools/gen_w1.py` builds 1,422 invented straight-only models in `source_recovery`'s admitted family, beyond the B1 corpus:

- straight frames, end anchors (or one anchor), and concentrated nodal forces and moments only;
- chord twins of the L, U, Z and 4-bend loop lines at Rφ/L from 0.008 to 90°, sharp-corner twins, and skew multi-span runs and cantilevers;
- t7, t24 and t96 sections at origin, utm73-skew, utm5-skew and skew2 placements.

| | Invocations | Refused | Sensitive at the solve | Pre-guard Passed | Recovery demotions (abandonments) |
|---|---|---|---|---|---|
| Today (R-b′) | 2,844 | 108 | 175 | 2,561 | 64 |
| Revision 0, as a guard on today's rows | 2,844 | 108 | 175 | 2,561 | 533 |
| (d), I44 (the attempt's matrix, measured ε_E) | 2,844 | 108 | 175 | 2,561 | **0** |

- Every fail-closed condition is decided in FK as K-D5 unavailability, so it follows W1's K-D5 path (`d5_diagnostic_ref`, translated), not N-5.
- With nodal forces and moments as the only loads, a body whose rows are all zero has no load on its free rows, so u = 0 there. Its certified rows are then exactly zero, and it stays Passed without the notice.
- Every one of the 2,561 pre-guard-Passed case-modes carried a K-D5 evaluation, so the PP-side "no `member_bound`" rule (§2.3) does not reach N-5 on this set. A test pins it on a W1 invocation.
- A PP-side firing would still abandon W1; its predicted rate is 0. N-5 is left unchanged.

### 4.5 Committed requests (no exact reference; predicted)

- **Integrity codes that move to Sensitive:**
  - the nonlinear-route cases (§5);
  - all-zero bodies whose certified rows are not all exactly zero (§3.3), each with the notice. In SP-1 set E these are up to 82 committed case-modes: the free-length, coefficient-definition, constant-α, thermal-datum, released fit-state and free-tip prescribed-rotation qualification requests, in both modes and both request forms, plus the three mixed ordinary-pressure requests. O-13's `p8_a_stress_free_arc_stays_passed` is another. Revision 1's count of 16 undercounted these. The implementation counts them; a body whose certified rows all come out exactly zero stays Passed.
- **Predicted to stay Passed** with repaired rows:
  - I7's `U2-L-FREE-P-K2-SKEW-X5E6` and `-X7P3E6`;
  - `prescribed_rotation_all_fixed`;
  - the 1e-4 rad arc of t15b;
  - the zero-load and temperature-identity cases (u = 0, rows exactly zero).
- **Passed linear-route cases publish certified member rows** (§6); zero-load and exact-result cases can remain byte-identical.

## 5. The nonlinear-support route

**Fail closed.** The route has no K-D5: no `FormationCheckedSystem` is built when nonlinear supports exist (ROOT's ruling). With (d) the unified predicate has no B_g there. A converged nonlinear case whose report would be `CHECKS_PASSED` is published `NUMERICAL_INTEGRITY_SENSITIVE`, with the sentence:

> "the member-relative formation check is not available on the nonlinear-support route; member rows are binary64 recoveries"

The same rule, with "on this route", covers any other route that would publish Passed without `member_bound` (§2.3). Its rows stay today's binary64 rows. The `RECOVERY_BASIS_UNQUALIFIED` branches are unchanged.

**Committed cases moving to Sensitive.** Two models with nonlinear supports publish Passed today: `fixtures/product_preview/invented_demo_model.json` and `core/product_physics/tests/fixtures/preview_physics_invented_model.json`. Each has two cases (`load:L-100`, `load:L-200`), in both modes: 8 case-modes. They appear in 5 committed outputs:

- `invented_demo_result_preview_physics_1_{sparse,dense}.json`;
- `invented_demo_result_legacy_0_1.json`;
- `fixtures/results/preview_physics_invented_{sparse,dense}.json`.

That is 10 integrity diagnostics. The desktop demo's cases show Sensitive.

**The next T4 unit: K-D5 on the converged active set.** At convergence the selected state solves a linear system: K plus the active supports' stiffness or constraint rows, with the closed and gapped supports fixed.

- Build its `FormationCheckedSystem`, with active support springs as springs, active gaps as prescribed or eliminated rows, and friction as its linearized state.
- Run K-D5 and (d) unchanged on it.
- Add one condition: the active set is certified. Each inactive support's gap, and each active support's contact force, must have the right sign with margin above its own bound (a ball on the support reaction rows). Otherwise the case fails closed with a named reason.

The cost and the W1 relation (the route is outside D1) belong to that unit.

## 6. The declared blast radius, pins and pricing (SP-1, SP-4; T3's I1)

**Bytes that change.**

- **Member rows and support reactions** of every Passed linear-route case: 36 committed linear output fixtures carry Passed member rows (`fixtures/results/*`, `fixtures/product_preview/{load_reference,load_reference_source,physics_source}/*`, `core/reporting/result_export/tests/fixtures/source_blocks_statement_control.json`). Displacements and every case that is not pre-guard Passed are unchanged. Zero-load and exact-result cases may retain identical member bytes.
- **The retained successors' ordinary base.** W-C2 (B1) and the B2 combination successors each contain a `CHECKS_PASSED` case (`not_required`) whose ordinary base rows change, so their publication and receipt hashes change. The exact successors have no Passed case and do not move. Each item below was located by grep at this revision:
  - PP `core/product_physics/src/retained_facade_tests.rs`:
    - `W_C2_PINNED` (:1882). Its registered branch in `b1_sp_w_c2_fixtures_are_the_live_successors` (:2194) needs the re-registered profile.
    - `b2p_witness_successors_are_pinned_in_both_modes` (:3947) and `b2p_w_cb1_successors_are_pinned_in_both_modes` (:3952).
  - `fixtures/results/retained_precision_w_c2_successor_{sparse_interactive,dense_scrutiny}.json`.
  - RE `core/reporting/result_export/tests/retained_precision_contract.rs`; headless `core/runner/headless/tests/reader_cli.rs`; `fixtures/results/retained_precision_reader_outcomes.json`.
  - Python `tests/test_retained_precision_contract.py`, `tests/test_retained_precision_b2.py`, `tests/retained_precision_corpus.py`.
  - Desktop `apps/desktop/src/test-support/retainedPrecisionCorpus.ts`, `apps/desktop/src/services/analysisRunCompatibility.test.ts`.
  - Corpora `fixtures/results/retained_precision_cases.json` (40 `CHECKS_PASSED` integrity codes), `retained_precision_cases_07o.json` (15), `retained_precision_b2_parity_probes.json` and `retained_precision_rv120_b3_inputs.json`.
- **SP-1 corpus bases.** Sets E, B1, B2, Z, F, V and D are re-based. The moves are predicted from the typed entry at this branch's head:
  - E: 76 of 96 runs carry a Passed case; their member rows change, and up to 82 case-modes take the notice (§4.5).
  - B1: 44 of 64 runs carry a Passed case; their member rows change.
  - Z: 6 runs, Passed with u = 0 and rows exactly zero; predicted unchanged (signed zeros to be checked).
  - **D: 0 of 6 runs move.** All are refused (`PRESSURE_REGION_PRESSURE_NEGATIVE`).
  - **V: 0 of 192 runs move.** All are refused (`EXACT_PRESSURE_COMPOSITION_UNSUPPORTED`, or `EXACT_PRESSURE_COMBINATION_UNSUPPORTED` with connector re-authoring).
- **Integrity codes** change per §4.5 and §5.
- **TEXT:**
  - the unified sentence;
  - the nonlinear and no-member-bound fail-closed sentences;
  - the new K-D5 unavailable details (closure, no ledger, contraction, Neumann, second solve);
  - the notice `STRESS_FREE_WITHIN_NUMERICAL_PRECISION`;
  - the limitation and scope sentences in the linear `FormulationBasis`.

**Sequence.**

1. This unit re-registers the profile first (pricing below).
2. T4 then re-pins and regenerates the corpora on this branch.
3. T3 reviews the re-pins.

T3's R2 (Rust-only base readers) commits native outcomes for a shared base corpus. Whichever of R2 and this unit merges second regenerates them.

**Other pins.**

- S11-G: `rb_prime_clauses_at_their_boundaries`, `t11`, `t12`, `t13`, `t13b`, `t20` and `ruling1_both_guards_fire_and_the_load_row_sentence_stands`.
- O-13's `arc_recovery` P1–P8 and G1–G6. The short-bend tests are deleted.
- `t15b`, re-pinned to its outcome.
- F1b's W2 and K2a reach tests that read R-b′'s sentence.
- The S11-F site table (`s11f_site_test` rule 6 and FK's `s11_site_table`): every new function that forms a load at p, and the recovery at p, is a force site.
- straight_pipe's `t17` retires with `bending_formation_bound`'s guard use.
- FK `formation_check_tests` (`kd5_*`), for the closure, ε_E and the second correction.
- `retained_wire_tests` and the retained façade's N-5 tests are unchanged (N-5 stays). Their pinned hashes move as listed above.

**Pricing (registered profile; `tools/retained_pricing`, `price.py run <head>`).** Re-registration is pre-approved within M = 10.5 GiB (11,274,289,152 B) and the 5 % rule. Each change below is small at the D1 caps (n = m = g = 32, F = N = 192), against the 8.1 % (dense) and 9.6 % (sparse) of TAV_W that the 0.9 M rule leaves for text error. The tool's run on the implementing head gives the numbers.

| Row (`base/ordinary_caps.py` and its tree) | Change |
|---|---|
| "Ordinary formation check (outer vectors)" | + 2·Xc(ExactAccumulator, F) for s and s₂; + B(8·5F) for the radius vector ψ, w₂, the second right-hand side and the ε_E row and column sums; the per-row allowance replaces the `Vec<f64>` lists; + V(LoadPrimitive, l) and the complete closure manifests with their expected summand identities |
| "H_formation128 (T06: heap 0)" | Stays 0 only if every re-formation is on the stack: K_int per element for ρ, s, s₂, ε_E and recovery, and load primitives at p, with no `Vec<Element>` cache (S5). A test pins it |
| StructuralSolution (now priced, S5) | + B(16·N) for w and w₂, + s(MemberBoundInputs), live from the completion through the recovery stage |
| W2 "evaluate" | It is ×2 over core_total, so it counts the larger formation-check row twice (T3's double count). The tail gains the unscaled member-bound inputs once: + B(16·N) |
| "Straight member recovery" | V(RecoveryRecord, m) becomes V(RecoveryRecord, m + a + c + g) (a = c = 0 under D1). s(RecoveryRecord) grows to 7 locations × 4 groups for members and 2 groups for supports. The certified recovery uses the stack only |
| "Formation-guard late maps and RecoveryFinding" | Coupled force and moment scales per body (O-13's ActionScales) |
| Text(recovery_finding), Text(formation_detail), D, D_env | Re-run the TEXT chain: new sentence items (no longer than today's), new detail strings, the notice and the limitation sentences |
| New FK loops (load primitives; the s and s₂ passes; ε_E over contributions; the recovery locations) | O-10 loop rules (`O10_LOOPS_LAST`), with their arguments in DEL-05-02's `retained-pricing-rules.md` |

The run is `run <head>`, then `compare` against the registered tree, then `generate`. The `profile_` law tests are re-pinned.

**Runtime (RV34).** Each element is re-formed at p in five passes instead of one: ρ, s, s₂, ε_E and recovery. One extra pass with the ε_E sums, measured in the scratch probe on RV30's D chunk (400 models, both modes), costs about 0.5 ms per case-mode. That is about 15 % of the probe's 3.5 ms per case-mode. Four extra passes therefore add at most about 2 ms per case-mode on these small models (about +60 %), linear in the element count. The memory pricing does not cover time. The implementation measures it on the realistic set and at the D1 caps and reports it with the PR.

## 7. Implementation plan

**FK `structural/formation_check.rs`:**

- `LoadPrimitive` enum (§1), many per ledger term, and `FormationSource::loads`; ledger-held closure manifests with independently enumerated expected summand identities.
- In `evaluate`:
  - exact closure-manifest/primitive identity-multiset equality over the ledger's formation records, and the no-ledger failure;
  - the radius vector ψ;
  - the s, w₂ and s₂ passes, each element re-formed on the stack;
  - ε_E by the triangle bound over the contribution blocks (§2 step 7);
  - the contraction (with its 0/0 rule) and Neumann checks;
  - D5C-1 on the closed w.
- Return `MemberBoundInputs { w, w2, exponents, lambda_sigma }`, or `FormationCheckUnavailable` with a named detail.

**FK `structural.rs`:**

- `estimate_rcond` keeps its signature. `ConditionMatrix::estimate_condition` returns the gate's (r̂, est, ‖K̃_s‖₁).
- `finish_checked_factor` passes est, the norm and the contributions to the check.
- The sparse `formation_check` view passes the contributions (today it passes `None`).
- `StructuralSolution.member_bound: Option<MemberBoundInputs>`, never in `StructuralReport`.
- The W2 entry computes Λ′ from its own attempt, passes w and w₂ through, and unscales Λ′·σ′ upward (§2.2).

**FK certificates (new `structural/certified_recovery.rs`):**

- `certify_straight_recovery(frame, loads, u, w, w2)` for end rows, endpoint section cuts and the three stations, as balls, with each group's a_g.
- `certify_curved_recovery`, O-13's, gains the correction terms and uses K_int,arc.
- Connectors and support reactions as §3.2.

**NI `structural_adapter.rs`** (`formation_source`, `solve_assembled_with_formation_check`) attaches the load primitives that the PP producers hand over with their ledger terms.

**PP `lib.rs`:**

- The producers of §1's table push their `LoadPrimitive` beside each formed term. The exact-pressure factor groups push one primitive per summand.
- The recovery loop publishes fl(m_g) for every member row (straight, arc and connector branches), the pipe-wall rows, the stresses scaled at p, and the reactions, whenever `member_bound` is present. It forms one `RecoveryRecord` per member and per support.
- The governing maximum, its extrema and the stress summaries are formed from the repaired actions (§3.2).
- No `CHECKS_PASSED` without `member_bound`; the nonlinear route fails closed (§5).
- The notice `STRESS_FREE_WITHIN_NUMERICAL_PRECISION` (§3.3).
- The short-bend rule is removed. `bend_neighbour_ratios` stays for the refusal.
- The linear `FormulationBasis` gains the limitation and scope sentences.

**PP `formation_guard.rs`:**

- `member_group_fires(q, q_floor, B, S*)` replaces `rb_prime_fires` and `arc_group_fires`.
- One `RecoveryRecord` shape: member or support, body, and per location and group (q, q̂, B, a), or the unavailable reason.
- One finding sentence, and the notice predicate.
- `short_bend_finding`, `ShortBend`, `SHORT_BEND_PASSED_RATIO` and `RESOLUTION_FACTOR` are removed.

**Readers.** No field moves; values, codes and one info diagnostic change. The extrema scope string is kept (§3.2), so no reader changes.

**Computed by the implementation, not assumed (RV34).** The emulations used stand-ins or measurements for these; the implementation computes each one and reports the result with the PR:

- ε_E by the triangle bound (§2.1 measured it on the product);
- est and ‖K̃_s‖₁ from the gate;
- r_g and a_g by ball arithmetic;
- the ψ radii, including the load enclosure radii, and the self-weight re-formation that replaces the γ₄ operand bound;
- Λ′ for W2 from that attempt;
- a closure test per primitive family against an exact reference, including grouped pressure operands and fit strain;
- the count of all-zero-body notices on committed outputs;
- the fail-closed rate on RV30's short bends (measured 0 of 7,209);
- the runtime of the extra passes.

**Tests and mutants that must be killed:**

- **Each B term:**
  - drop ‖A·w₂‖ (in m_g, evaluate at u + w);
  - drop R, or use s in place of s₂;
  - drop ψ;
  - drop the 2^10;
  - use 1/(1 − a) in place of the ε_E Neumann factor, or drop any of ε_E's three parts;
  - drop D;
  - use ‖·‖₁ in place of ‖·‖_∞ on a non-symmetric K_int fixture;
  - for W2, reuse the unscaled attempt's Λ.
- **The closure:**
  - a formed term without a primitive must fail closed;
  - a ledger term carrying several primitives, with one dropped while its independently enumerated ledger manifest remains intact, must fail closed; duplicates, extras, mismatched term identities and wrong operand bindings must also fail closed, in ordinary and W2 routes;
  - K-D5 with no ledger must fail closed;
  - a ledger value substituted for a primitive (the realistic STD-U wT utm-skew line kills it at 3.8e3·c);
  - each primitive family has a test against an exact reference.
- **Repair:**
  - publishing v instead of fl(m_g);
  - m_g at u;
  - rounding magnitudes after their components;
  - the governing maximum formed from v instead of the repaired actions;
  - the K2b unscaling of w (must not happen) and of R (must).
- **The decision:**
  - `≥` in place of `>`;
  - a rounded comparison;
  - the floor conjunct, and q in place of q̂;
  - the 2^-988 guard;
  - the resolution conjunct reintroduced (RV29's cantilever kills it);
  - group norm replaced by a maximum;
  - the force and moment reaction groups merged;
  - the spring support-on-pipe sign reversed; signed components and equilibrium are pinned on ordinary and W2 routes;
  - the pipe-wall group merged into N.
- **The notice:**
  - the positive, the negative control and the "solely because" pin of §3.3;
  - a mutant that lets the notice change the code;
  - a mutant that ignores unjudged groups in condition 3.
- **Fail closed:** the contraction limit and its 0/0 rule, the Neumann limit, the second solve, the closure, no ledger, no `member_bound`, and the nonlinear route each demote, with the named detail.
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

- **Combination rows** are not judged against their own q (§3.3).
- **The load-row guard** no longer guards any certified row (§4.2). Retiring it changes W1's routing into retained-source recovery and needs T3's agreement. Its availability cost is 6 RV30 case-modes and 0 realistic.
- **Runtime** of the five p passes (§6) is measured at implementation and reported.
- **The emulation's intended loads** are RV30's reference definitions. The implementation's primitives must match them, including the effective-force pressure model and the resolved total eigenstrain, or the closure test against the exact reference fails.
