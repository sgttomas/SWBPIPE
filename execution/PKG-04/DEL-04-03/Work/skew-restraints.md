# Skewed restraints (R-6, R-7): build design, shared interface and first-implementation brief

> **Status: candidate design, revision 3.**
> - Revision 1 (`24f343662`) was reviewed independently (gpt-6-astra, medium) with no P0.
> - Revision 2 (`fd9710a72`) applied that review. A focused confirmation closed most of it and
>   left four items.
> - Revision 3 fixes those four:
>   - the attribution numerator and quotient are range-safe up to the published values (§7.2,
>     §10.5);
>   - S-K3's radius is taken about τ_b (§8.3);
>   - the restricted rescale w is used for the pivot and for Q on F (§5.1, §8.3);
>   - the Cholesky factor of D K′ D is D L D (§4.3).
>
> A narrow independent confirmation of revision 3 closed every finding; only the signed-range
> wording (absolute values) was corrected after it. Design and references only: no product
> or kernel code is changed by it. It builds on the adopted kernel design (DEL-05-02 Work
> `operating-case.md` §0 rows R-6, R-7 and "Skew kernel scheduled"; §2.1–§2.6; §6.2) and
> does not reopen it. It changes numerical meaning when built, so it takes targeted references
> (`validation/references/skew_restraints/`) and independent review before any implementation.
> Nothing here records or implies acceptance, issuance or release (F-PIP-3).

**Basis.** Code at main `e6579eef5`. Line numbers refer to that revision. Short names are as in
`operating-case.md`: PP `core/product_physics/src`, FK `core/solver/frame_kernel/src`, LSUP
`core/solver/linear_supports/src`, NS `core/solver/nonlinear_supports/src`, NI
`core/solver/nonlinear_integration/src`, RE `core/reporting/result_export/src`.

**Owning deliverable.** DEL-04-03, by `tools.deliverables`: it owns LSUP, `imposed_motion.rs`
and the commitment DEL-04-03:2 (skewed translational restraints and springs). The design also
changes code that other deliverables own: DEL-04-01 (FK), DEL-04-07 (PP's solve and NI's
adapter), DEL-04-09 (modal), DEL-02-01 (the applier's authoring of `direction`) and DEL-04-08
(readers and pricing). §11 lists those seams.

---

## 0. Summary

| Item | Settled here |
|---|---|
| First build slice (S-K1) | PF route, models with no nonlinear support, both solver modes, K-D5 included. Admits skewed `spring` records (any number per node) and at most one skewed rigid `line_stop` per node. The stop may share its node with axis rigid restraints held at zero, axis springs and rotational restraints. Everything else is refused by an exact named code (§2). |
| Shared interface | `open_pipe_stress_frame_kernel::restraint_basis::RestraintBasis`, a new FK module, built by LSUP. One constructor, one congruence-term enumeration, one lift and one attribution. Static, K-D5, modal and (later) NI all call them (§3). |
| Authored vector | Stored as authored. The canonical form is d̃ = 2^s·d with max\|d̃_i\| ∈ [1, 2). It is admitted iff every nonzero d̃_i is normal; otherwise `SUPPORT_DIRECTION_RANGE_UNSUPPORTED`, with no rounding and no tolerance. Power-of-two rescalings are bit for bit invariant. Positive rescalings keep the exact system. Negation keeps the exact u and R (bit for bit only under the solve's sign-symmetry premise P-SYM, which a test checks) and negates the along-direction scalar (§4). |
| Rank | Per node and exact. S-K1 needs no arithmetic: with axis rows A and one stop, the rank is \|A\| + [d̃_F ≠ 0] on the free axes F. S-K2's joint case decides rank by exact 2×2 and 3×3 minors. A dependent set is `SUPPORT_CONSTRAINT_REDUNDANT`. A near-dependent set is admitted when exactly independent, as an exact reduction (§5). |
| Reactions | Global support-on-pipe components, plus a new row `support_reaction_along_direction_v1` (N, positive along authored +d). Attribution comes from the unique multipliers, never from a split. It is range-safe: the restricted direction is rescaled exactly before its norm is formed. Physical reaction rows include every skewed spring block (§7, §10.5). |
| Certificate | K-D5's existing machinery on the transformed system QᵀK_intQ, with exact splits for triple products and the existing `SPLIT_EXACT_MIN` premise. The binary64 factorization and correction are unchanged. The member bound is lifted to physical DOFs with one new per-DOF lift radius. No new certificate is introduced (§10). |
| Pricing | The retained route never admits `direction` (D1 premise), so every skew arm is zero on D1. Pinned identity-path bodies and some layouts change, so a D3 actual-source turn is needed (§12). |
| Exact obstacles | None that blocks S-K1. The reserved choices of §14 are decided (Agent 0, 2026-11-04): C-1 = meaning B (ground motion along d/|d|), C-2 = S-K2 → S-K3 → S-K5 → S-K4 after S-K1, C-3 = keep the range refusal. |

---

## 1. Today (verified at `e6579eef5`)

- **Document rules.** `document_members.rs:333-372` (validator, both applier and solve):
  `SUPPORT_DIRECTION_INVALID`, `…_ZERO`, `…_RECORD_UNSUPPORTED` (another family, an inferred
  family, more than one DOF, a `nonlinear` member, a snubber with more than one restraint),
  `…_ROTATIONAL_UNSUPPORTED`.
- **Interim refusal.** `document_members.rs:745-767` emits `SUPPORT_DIRECTION_SKEW_UNSUPPORTED`
  (blocking) for two or more nonzero components. It is reached from
  `operating_conditions::judge` (`operating_conditions.rs:408`), which both the static solve and
  modal's `prepare_modal` (`modal/prepare.rs:150`, through `lower`) run. So lifting it lifts it
  for modal too.
- **Axis path.** `lower_axis_aligned_directions` (`document_members.rs:800`) removes an exactly
  axis-aligned `direction` and rewrites the record's DOF token. The record then takes today's
  path, byte for byte.
- **Applier.** `create_support` (`operation_applier/src/lib.rs:5360-5373`) refuses a skew
  `direction` with `SUPPORT_DIRECTION_SKEW_UNSUPPORTED` and an axis-aligned one with
  `OP-CREATE-SUPPORT-PAYLOAD-INVALID`. No other operation writes `direction`.
- **Assembly and boundary.** PP builds `LinearSupport` records (`lib.rs:8911-8962`), and
  `prepare_boundary` (LSUP `lib.rs:384`) partitions DOF indices.
  `assemble_sparse_stiffness` (FK `structural/sparse.rs:599`) adds springs on the diagonal
  (:743). `reduce_assembled_sparse_system` (:798) partitions. PP's case solve
  (`solve_prepared_load_case`, `lib.rs:5267`) reduces at :5508. It solves through NI's
  `SparseAssemblyEvidence::solve_assembled_with_formation_check` (`structural_adapter.rs:900`,
  from `solve_preview_reduced_system`, `lib.rs:7864`), forms reactions as
  `restrained_reactions` (:5203) and publishes support rows at :6116-6230, with K-D5 rows from
  `certified_publication::support` (`certified_publication.rs:312`).
- **K-D5.** `FormationSource` (FK `structural/formation_check.rs:335`), `subtract_action`
  (:729), `epsilon_e` (:815), `evaluate` (:916); `MemberBoundInputs` (:323);
  `evaluate_row_inner` (`certified_recovery.rs:770`), `certify_support_reaction` (:1454) and
  `certify_spring_reaction` (:1732).
- **Ground screening.** The PP count (`lib.rs:3159-3176`, and its modal twin) and
  `assess_rigid_body` (FK `rigid_body.rs:33`) through NI's `BodyEvidence::geometry`
  (`structural_adapter.rs:1504`). Both are fed DOF indices.
- **Modal.** `form_modal_pencil` (`modal/stiffness.rs:130`) takes the static
  `form_basis_stiffness` and partitions with `reduce_assembled_sparse_system` (:158). It reads
  M_ff from node blocks.
- **Retained route.** Admission refuses any `direction` (`retained_memory.rs:1040`,
  `FamilyFact::SupportDirection`), so D1 never holds a skewed record.

---

## 2. The first build slice (S-K1) and its refusals

### 2.1 Admitted

On the pressure-free route (PF, preview-physics-2), in a model with **no nonlinear support**,
in both `sparse_interactive` and `dense_scrutiny`, with the K-D5 formation check and certified
recovery, the following are admitted:

- **Skewed springs.** An explicit `spring` record, single translational DOF, positive stiffness,
  and a `direction` with two or more nonzero components that passes §4.2. Any number may sit at
  one node. They add to axis springs and rigid restraints there.
- **Skewed rigid stops.** An explicit `line_stop` record, single translational DOF, with such a
  `direction`. At most one per node. At its node it may be combined with:
  - axis rigid restraints on translations, which must be held at zero in every case (§2.2
    item 8);
  - rotational rigid restraints, with any prescribed values;
  - axis springs and skewed springs.
- **Everything else in an admitted model is as today.** That includes W-16 imposed motion and R-c
  spring-base motion at any node other than a skew record's own ground, PF combinations, and
  modal (DEL-04-09) on the same models.

Both families are translational and linear; the stop is an exact constraint (R-6), never a
penalty spring.

### 2.2 Refused, in gate order (§6.2 conventions: earliest stage first, one code per object)

| # | Code | Object and stage | Remover |
|---|---|---|---|
| 1 | `SUPPORT_DIRECTION_INVALID`, `…_ZERO`, `…_RECORD_UNSUPPORTED`, `…_ROTATIONAL_UNSUPPORTED` | Unchanged document rules (validator: applier and solve) | Rotational skew: not scheduled (same algebra on R-DOFs). Skewed `nonlinear` record: not scheduled (DEL-04-04 directional contact) |
| 2 | `SUPPORT_DIRECTION_SKEW_EXACT_UNSUPPORTED` (new, blocking) | Skew admission (solve; applier). A skewed record on a document with a `pressure_contract` | S-K5 (EX) |
| 3 | `SUPPORT_DIRECTION_SKEW_NONLINEAR_MODEL_UNSUPPORTED` (new, blocking) | Skew admission (solve). A skewed record in a model with any nonlinear support. This covers a friction-normal source and modal's state case | S-K4 (NI) |
| 4 | `SUPPORT_DIRECTION_RANGE_UNSUPPORTED` (adopted name, blocking) | Skew admission (solve; applier). §4.2's range rule | Permanent (intrinsic to the direction) |
| 5 | `IMPOSED_DISPLACEMENT_SKEW_SUPPORT_UNSUPPORTED` (new, blocking) | `imposed_motion::validate`. An `imposed_displacement` primitive whose target is a skewed record | S-K3 (meaning B, §14 C-1) |
| 6 | `SUPPORT_DIRECTION_NODE_UNCONNECTED` (new, blocking) | Boundary preparation. A skewed record on a node with no element: the node's 3×3 block is not in the pattern | Permanent |
| 7 | `SUPPORT_SKEW_STOPS_PER_NODE_UNSUPPORTED` (new, blocking) | Boundary preparation. A node with two or more skewed stops | S-K2 (§5.3) |
| 8 | `SUPPORT_CONSTRAINT_REDUNDANT` (adopted name, blocking) | Boundary preparation. §5's exact rank is below the node's row count | Permanent (attribution would not be unique) |
| 9 | `SUPPORT_SKEW_NODE_PRESCRIBED_MOTION_UNSUPPORTED` (new, blocking for the case) | Case solve, before reduction. A nonzero prescribed translation on an axis restraint at a skewed stop's node (W-16 primitive or load-state motion) | S-K3 (affine reduction, §8.3) |
| 10 | `SUPPORT_DIRECTION_SKEW_FORCE_SCALED_UNSUPPORTED` (new, blocking for the case) | Case solve. A skew model's ordinary formation leaves the binary64 range (`RangeDeferred`) or would need W2/K2b. The ordinary failure is published as today and W2 is not attempted | S-K2 |
| 11 | `SUPPORT_SKEW_REACTION_RANGE_UNSUPPORTED` (new, blocking for the case) | Case publication. A **published** value (a stop or axis component, or the along-direction value) is outside binary64 (§7.2). An intermediate such as N_w never triggers it | Permanent (a genuine out-of-range result) |

Also unchanged:
- `SNUBBER_DYNAMIC_ONLY_UNSUPPORTED` refuses every static snubber; DEL-04-10 consumes the basis.
- `HANGER_SIZING_CANDIDATE_SKEW_UNSUPPORTED` stays.
- The retained-route decline (`FamilyFact::SupportDirection`) stays.
- The CAEPIPE SI writer and MBF importer refuse non-axis directions. That mapping is
  DEL-17-04's, and none is scheduled here.

**The coordinate transform's range** is not a §2.2 code. A product of the coordinate transform
below the exact-split range, or a non-finite rounding, refuses the case on the ordinary
`NumericallyUnresolved` path, named in §6.2. It is not a K-D5 demotion (§10.7).

**Order within a stage.** Skew admission runs in `judge` and gives one code per record, in
document order, the first that applies of 2, 3, 4. Boundary preparation is per node, in
ascending node order, the first that applies of 6, 7, 8, and reports every node. Case-level
codes 9 to 11 are per case. A record refused at one stage does not reach later stages, because
the stage's blocking return stops the solve.

`SUPPORT_DIRECTION_SKEW_UNSUPPORTED` is no longer emitted by S-K1. Its constant is kept for the
history of its refusals, as 1b kept its interim constant.

### 2.3 Later slices (designed here; built in the order Agent 0 sets)

| Slice | Content | Design |
|---|---|---|
| S-K2 | Joint construction: two or three stops per node, exact minors and expansions; W2/K2b with skew (blocks × 2^b, T unchanged) | §5.3, §10.6 |
| S-K3 | Imposed motion at or along skewed records (affine reduction u = u₀ + Q v) | §8.3 |
| S-K4 | Skew in nonlinear models (NI's P loop, `complete_iteration`, confirmation, R-3's G2, modal's state case) | §11 seams N-1 to N-4 |
| S-K5 | The EX route (W1 exact boundary, the pressure-2 contract) | §11 seam E-1 |

---

## 3. The shared interface

### 3.1 Location and ownership

- **The type and its algebra:** FK, new module `core/solver/frame_kernel/src/restraint_basis.rs`
  (`pub mod restraint_basis;` in FK `lib.rs`). It must sit in FK because K-D5
  (`formation_check.rs`, `certified_recovery.rs`), the sparse reduction and `rigid_body.rs` all
  consume it, and LSUP depends on FK, not the reverse. Owner: DEL-04-01's code, under this
  design.
- **Its construction from support records:** LSUP, a new `prepare_skew_boundary` beside
  `prepare_boundary`, which is unchanged. LSUP owns the support models (DEL-04-03).
- **Consumers:** PP's static case solve (DEL-04-07), K-D5 (DEL-04-01), modal (DEL-04-09), and
  later NI (S-K4), DEL-04-10 and EX.

### 3.2 Signature (S-K1; S-K2 extends the entry type, not the API)

```rust
// core/solver/frame_kernel/src/restraint_basis.rs

/// R-7: an authored direction with two or more nonzero components, its canonical exact
/// power-of-two scaling and its pivot (§4). Never normalized.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SkewDirection { authored: [f64; 3], scaled: [f64; 3], scale_exponent: i32, pivot: usize }
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DirectionError { NonFinite, Zero, AxisAligned { axis: usize }, RangeUnsupported { component: usize } }
impl SkewDirection {
    pub fn new(authored: [f64; 3]) -> Result<Self, DirectionError>;
    pub fn authored(&self) -> [f64; 3];
    pub fn scaled(&self) -> [f64; 3];       // d̃ = 2^s·d, max|d̃_i| ∈ [1, 2)
    pub fn scale_exponent(&self) -> i32;    // s
    pub fn pivot(&self) -> usize;           // argmax |d̃_i|, ties to the lowest index
}

pub struct SkewStop   { pub support_id: String, pub node: usize, pub direction: SkewDirection }
pub struct SkewSpring { pub support_id: String, pub node: usize, pub direction: SkewDirection, pub stiffness: f64 }

/// One constrained node: Q (row-major, exact binary64 entries; +0.0 for a structural zero),
/// its pivot m, its free axes F and its stop.
pub struct NodeBasis { pub node: usize, pub stop: usize, pub free_axes: [bool; 3], pub pivot: usize, pub q: [[f64; 3]; 3] }

pub enum BasisFinding { StopsPerNode { node: usize, supports: Vec<String> }, Redundant { node: usize, supports: Vec<String> } }
pub enum BasisError { SplitRange { dof: usize }, NonFinite { dof: usize }, Shape(&'static str), Sum(SumError) }

pub struct RestraintBasis { /* node_count, axis_restrained, restrained_coordinates, nodes, stops, springs */ }
impl RestraintBasis {
    /// `axis_restrained` is today's `BoundaryPreparation::restrained_dofs` (ascending).
    pub fn build(node_count: usize, axis_restrained: &[usize], stops: Vec<SkewStop>, springs: Vec<SkewSpring>)
        -> Result<Self, Vec<BasisFinding>>;
    pub fn has_constraint_transform(&self) -> bool;          // !nodes.is_empty()
    pub fn axis_restrained(&self) -> &[usize];               // physical, as today
    pub fn restrained_coordinates(&self) -> &[usize];        // axis ∪ pivots, ascending: the partition
    pub fn nodes(&self) -> &[NodeBasis];
    pub fn stops(&self) -> &[SkewStop];
    pub fn springs(&self) -> &[SkewSpring];
    pub fn node_basis(&self, node: usize) -> Option<&NodeBasis>;
    /// The binary64 3×3 block of one skewed spring (§6.1).
    pub fn spring_block(&self, spring: usize) -> [[f64; 3]; 3];
    /// (Qᵀ X Q)_{rc} = Σ q_ir · X_ij · q_jc: the physical (i, j) pairs and the exact weights
    /// (q_ir, q_jc) of one coordinate entry. Identity rows give the single pair (r, c) with
    /// weights (1, 1). The one enumeration that K', M', the allowance map and K-D5 use.
    pub fn congruence_terms(&self, row: usize, col: usize) -> CongruenceTerms;
    /// K' = Qᵀ K Q on K's pattern, each touched entry one exact sum rounded once (§6.2).
    pub fn transform_stiffness(&self, k: &SparseStiffness) -> Result<SparseStiffness, BasisError>;
    /// The coordinate partition and reduced right-hand side
    /// f'_r − Σ_c K'_rc g_c, with f' = Qᵀ f formed term by term from the physical ledger
    /// (§6.3), each free row one exact sum rounded once.
    pub fn reduce(&self, k_prime: &SparseStiffness, force: &AssembledForce, values: Option<&[f64]>)
        -> Result<SparseReducedSystem, StructuralError>;
    /// u = Q v, each physical entry one exact sum rounded once; v_pivot must be 0.
    pub fn lift(&self, v: &[f64]) -> Result<Vec<f64>, BasisError>;
    /// Adds (Q v)_dof exactly to `acc` (for certificates and the exact constraint check).
    pub fn lift_exact(&self, v: &[f64], dof: usize, acc: &mut ExactAccumulator) -> Result<(), BasisError>;
    /// K-D5's coordinate bound inputs as physical ones (§10.4).
    pub fn lift_member_bound(&self, v: &[f64], bound: &MemberBoundInputs) -> Result<MemberBoundInputs, BasisError>;
    /// Binary64 attribution at one constrained node from its physical rows r = (K u − f) (§7.2).
    pub fn attribute(&self, node: usize, r: [f64; 3]) -> Result<NodeAttribution, BasisError>;
    /// Ground rows for rigid-body screening: axis DOFs as today, plus one direction row per stop
    /// and per positive skewed spring (§9).
    pub fn ground_rows(&self) -> Vec<GroundRow>;
}
pub struct NodeAttribution { pub stop: StopReaction, pub axis: Vec<(usize /*dof*/, f64)> }
pub struct StopReaction { pub global: [f64; 3], pub along: f64 }
pub enum GroundRow { Axis(usize), Direction { node: usize, d: [f64; 3] } }
```

LSUP:

```rust
// core/solver/linear_supports/src/lib.rs
pub struct SkewSupport { pub support_id: String, pub family: SkewFamily, pub node_index: usize,
                         pub direction: [f64; 3], pub stiffness: Option<SupportQuantity> }
pub enum SkewFamily { LineStop, Spring }
/// New `FindingCode`s: `SkewDirectionRange`, `SkewStopsPerNode`, `ConstraintRedundant`.
pub fn prepare_skew_boundary(node_count: usize, prepared: &BoundaryPreparation, skew: &[SkewSupport])
    -> Result<RestraintBasis, Vec<SupportFinding>>;
```

`LinearSupport`, `BoundaryPreparation` and `prepare_boundary` are unchanged. A skewed record
never enters `LinearSupport`, so its DOF token is never restrained or sprung (§6.4).

### 3.3 What it returns

- **The basis T.** Per constrained node, Q is node-local, with columns w_m e_j − w_j e_m
  (j ∈ F, j ≠ m), e_m for the pivot coordinate, and e_a for each axis-restrained a.
  - w = 2^ŝ·d̃ is §5.1's exact restricted rescale, with max_{i∈F}|w_i| ∈ [1, 2). With no axis
    row at the node, ŝ = 0 and w = d̃.
  - Every entry is an exact binary64 of magnitude below 2 (an authored component times a power
    of two), 1.0 or +0.0.
- **The partition.** `restrained_coordinates()`.
- **The reduced system.** `transform_stiffness` + `reduce`.
- **The lift.** `lift`, `lift_exact`, `lift_member_bound`.
- **Reaction attribution.** `attribute`, and its certified twin in FK `certified_recovery.rs`
  (§10.5).
- **Ground rows.** `ground_rows`.

### 3.4 How each path consumes it

| Path | Consumption |
|---|---|
| Static PF linear (PP) | `form_basis_stiffness` gains the skewed spring blocks. When `has_constraint_transform()`, the case solve forms K′, reduces by `basis.reduce`, solves on the coordinates through NI's adapter with `with_restraint_basis`, lifts, and attributes. With no skew records the basis is `None` and today's code runs unchanged (§6.5). |
| K-D5 | `FormationSource.restraint: Option<&RestraintBasis>` drives the transformed residual, the allowance and D5C-1, and `lift_member_bound` the member bound (§10). |
| Modal (DEL-04-09) | `form_modal_pencil` takes the basis. K′_ff is from `transform_stiffness`, M′_ff from `congruence_terms` over its double-double node blocks, the excitation is Qᵀ(M R_k), and shapes are lifted by `lift`. The same constructor and enumeration give the same subspace. |
| NI (S-K4) | `base_restrained_dofs` becomes the basis. Each iteration's active axis DOFs rebuild it with `RestraintBasis::build(…, axis ∪ active, …)`, so the rank policy applies per state (§11 N-1). |
| DEL-04-10 | Snubbers in the dynamic stiffness use the same `SkewStop` and `SkewSpring` types. |

---

## 4. The authored vector and its exactness policy

### 4.1 Storage

`direction` is three finite binary64 global components, stored as authored. The applier writes
the entered numbers (shortest round-trip JSON) and never a normalized copy (0.9.0 §2.1). Only
`SkewDirection` reads it, and it reads only `authored`. Every computation reads only `scaled`
and `pivot`.

### 4.2 Canonical scaling and admission (exact; no tolerance)

1. **Invalid.** A non-finite component gives `SUPPORT_DIRECTION_INVALID`. Every component ±0
   gives `…_ZERO`. These are the validator's, so they never reach the kernel.
2. **Axis-aligned.** Exactly one nonzero component takes today's path through
   `lower_axis_aligned_directions`, byte for byte. That covers ±e_a, any positive or negative
   magnitude and any signed zeros elsewhere. No along-direction row is published for it (§7.1).
3. **Skewed.** Otherwise let e be the binary exponent with max|d_i| ∈ [2^(e−1), 2^e) (C `frexp`).
   Then s = 1 − e and d̃ = 2^s·d, so max|d̃_i| ∈ [1, 2). The direction is admitted iff every
   nonzero d̃_i is normal (|d̃_i| ≥ 2^−1022). Otherwise it is
   `SUPPORT_DIRECTION_RANGE_UNSUPPORTED`, naming the components. For an admitted direction
   every d̃_i is exact: scaling up never loses bits, and scaling down to a normal value is
   exact.
4. **Pivot.** m = argmax|d̃_i|, ties to the lowest index.

The second admission premise of the adopted design ("any exact product the certificate needs
falls below the exact-split range") depends on K and u, not on the record alone. It is
therefore not an admission rule. It is a fail-closed certificate condition (§10.2 P1), which
demotes the case by name and never rounds.

### 4.3 Which rescalings are allowed, and what is invariant (proofs)

Let d be admitted and c ≠ 0 with d′ = c·d exactly representable (each c·d_i exactly binary64).

- **(I1) Power of two, c = 2^k: every output is bit for bit identical.** max|d′_i| =
  2^k·max|d_i|, so e′ = e + k and s′ = s − k. Hence d̃′ = 2^(s−k)·2^k·d = d̃ exactly, and m′ = m.
  The kernel reads only (d̃, m), so every formed value, solution, reaction and row is the same
  bits. The admission verdict is also identical, since it depends only on d̃. This includes
  2^±600 and the subnormal-pair case (2^−1074, 2^−1074, 0) → (1, 1, 0).
- **(I2) Positive factor c > 0: the exact system and every exact result are identical; binary64
  outputs may differ by rounding.**
  - The constraint set {u : d′ᵀu = 0} = {u : dᵀu = 0}.
  - The spring block k·d′d′ᵀ/(d′ᵀd′) = k·ddᵀ/(dᵀd).
  - The column space of Q′ on the node equals Q's: both span d^⊥ on F, plus e_m.
  - The unique solution (u*, R*) of the constrained equilibrium is a function of the
    constraint set and K only. The along-direction scalar R*ᵀd′/|d′| = R*ᵀd/|d| for c > 0.

  K-D5 bounds, when established, contain the same exact values. Examples: d = (3,4,0) with
  (9,12,0) and (2.25,3,0); (1,2,0) with (0.1, 0.2, 0), whose binary64 components are exactly
  in ratio 1 : 2.
- **(I3) Negation, c = −1.**
  - **Guaranteed.** d̃′ = −d̃ and m′ = m, so the constraint set and spring blocks are identical,
    and so are the exact u* and R*. The exact along-direction scalar is exactly negated:
    R*·d̃′/|d̃′| = −s*. Both cases' established K-D5 bounds contain these same exact values, and
    their binary64 outputs agree within those bounds.
  - **Bit for bit, only under premise P-SYM.** Q′ negates the columns j ≠ m and keeps e_m. So
    K′′ = Q′ᵀKQ′ = D K′ D and f′′ = D f′ with D = diag(±1), and negation of a binary64 value is
    exact. Bitwise equality v′ = D v (hence u′ = Q′v′ = Q v, R′ = R, s′ = −s) holds only if every
    step from K′ and f′ to the published values commutes with D. **P-SYM:**
    - each operation is odd in its sign-flipped inputs: products and sums of binary64 values
      under round-to-nearest-even, Cholesky on D K′ D (with K′ = L Lᵀ and the
      conventional positive diagonal, its factor is D L D: (D L D)(D L D)ᵀ = D L Lᵀ D, and
      diag(D L D) = diag(L); the pivots are the same);
    - no branch depends on a value's sign. Magnitude-only choices such as equilibration
      exponents and pivot screens are sign-free; a comparison against a signed value would not
      be;
    - the reduction, the K-D5 correction and the lift form the same exact sums up to sign, and
      `ExactAccumulator::round` is symmetric except that it never returns −0.0.

    The premise is not proved here for every product path (sparse and dense, M03, W-paths), so
    it is a stated premise with a maintained test: `restraint_basis_tests.rs` and
    `skew_restraints_references.rs` assert bitwise u and R (up to the sign of a zero) and the
    exactly negated scalar for the `invariance.negation` pairs, in both solver modes. If an
    implementation cannot meet the test, As built records the guaranteed claim only, and the
    code is not changed merely to pass it.
- **(I4) Non-representable factors** are not rescalings of an authored binary64 vector; the
  user authors whatever binary64 vector they mean.

The references hold groups for I1 to I3 (`invariance`, `direction_policy`).

### 4.4 The exact fallback when scaling would be inexact

An inexact canonical scaling means the components span more than the normal range: the ratio
max/min is at least 2^1023. No exact power-of-two scaling then puts the largest component in
[1, 2) and keeps every component normal. Every other power-of-two scaling has the same span.
The fallback is the refusal by name (`SUPPORT_DIRECTION_RANGE_UNSUPPORTED`), as the adopted
design states and as Agent 0 decided (C-3, 2026-11-04: keep the refusal). Its message names the
subnormal component and states that an axis-aligned direction is the authored form if that axis
is meant. Scaling so the smallest component is normal was considered and is not adopted.

---

## 5. Constraint rank policy (per node, exact)

### 5.1 S-K1: axis rows plus one stop

At node N, let A be the node's axis-restrained translations (held at zero) and F the other
translations. The constraint rows are e_a (a ∈ A) and d̃.
- On u_A = 0 the stop row reduces to d̃_Fᵀu_F = 0.
- **Rank** = |A| + [d̃_F ≠ 0], decided by inspecting signs of zero; no arithmetic.
- d̃_F = 0 gives `SUPPORT_CONSTRAINT_REDUNDANT`. The stop restrains nothing beyond A, and its
  multiplier would not be unique.
- **Pivot on F.** m = argmax_{i∈F}|d̃_i| (ties lowest).
- **The restricted rescale (revision 3).** d̃_m is not in [1, 2) in general. It is the largest
  component on F only, and can be as small as 2⁻¹⁰²². For example, d = (1, 2⁻⁶⁰⁰, 0) with UX
  held gives d̃_m = 2⁻⁶⁰⁰.
  - So the node uses w = 2^ŝ·d̃, with ŝ ≥ 0 the exponent that puts max_{i∈F}|w_i| = |w_m| in
    [1, 2). Then ŝ ≤ 1022, and w is exact (scaling up never loses bits). On F,
    w_Fᵀu_F = 0 ⇔ d̃_Fᵀu_F = 0, so the constraint is unchanged.
  - With A = ∅, ŝ = 0 and w = d̃, so revision 1's nodes and the `direction_policy` bits are
    unchanged.
- **Q.** Columns w_m e_j − w_j e_m for j ∈ F∖{m}; e_m (the pivot coordinate, restrained at 0);
  e_a for a ∈ A (restrained at 0). Every entry of Q on F has magnitude below 2, as §10.4's
  exponent sum and §10.2's split products assume.
  - Without the rescale, a column such as 2⁻⁶⁰⁰ e_z would scale K′ by about 2⁻¹²⁰⁰ there and
    push its triple products under the split range.
  - The references list each single-stop node's columns (`attribution.Q_on_F_columns_from_w`).
    `R4_near_dependent_axis` gets e_z, not 2⁻⁵⁰⁰ e_z.

| \|F\| | d̃_F | Result |
|---|---|---|
| 3 | ≠ 0 | two free coordinates (§2.2 of the adopted design) |
| 2 | ≠ 0 | one free coordinate. If d̃_F has one nonzero entry, the column is w_m e_j: the stop fixes u_m, and still carries the reaction |
| 1 | ≠ 0 | none: the node's translations are fixed; attribution is unique |
| any | = 0 | redundant |

`R4_full_fix_one_free_axis`, `R4_redundant_axes` and `R4_near_dependent_axis` (d = (1, 2^−500, 0)
with UX: exact rank 2, u_y = 0 exactly) cover this table.

### 5.2 Interaction with base restraints and springs on the same DOFs

- **Axis rigid restraints at the node** are the rows A above. They must be held at zero (§2.2
  item 9). The existing rule that no DOF is restrained twice still applies to them; the stop
  restrains no DOF index.
- **Rotational restraints** are separate DOFs, with Q the identity there, and any prescribed
  value is admitted.
- **Axis and skewed springs** at the node add to K before the transform. They impose no
  constraint, so rank is unaffected.
- **A skewed spring at a node whose translations the stop and axes fix** contributes no free
  stiffness. It still publishes its force −k d̃(d̃ᵀu)/(d̃ᵀd̃), which is exactly zero because u is
  zero there.

### 5.3 S-K2: joint construction (designed here, built later)

Stack C (r × 3) from the axis rows e_a and stops d̃_k at the node. Restrict to F after the axis
rows, as in §5.1.

- **Rank, exactly.** Each 2×2 minor c₁ᵢc₂ⱼ − c₁ⱼc₂ᵢ is a difference of two binary64 products.
  It is exact in one `ExactAccumulator` (`add_product` twice); its sign is exact.
- **The 3×3 determinant** needs triple products. Split c₁ᵢ·c₂ⱼ exactly (TwoProduct, valid when
  the product is zero or at least `SPLIT_EXACT_MIN` = 2^−969 in magnitude), then
  `add_product` each part with c₃ₖ. Outside that range the node is `SUPPORT_DIRECTION_RANGE_UNSUPPORTED`.
- **Rank below r** gives `SUPPORT_CONSTRAINT_REDUNDANT`. The redundant cases are
  `K2_parallel`, `K2_antiparallel_scaled` and `K2_axis_plus_two_stops_redundant`.
- **Near-dependent sets are not refused.** If a minor is nonzero exactly, the rank is full and
  the reduction is exact. The multipliers may be very large (`K2_near_parallel`: d₂ = (3, 4 +
  2^−48, 0), λ = ∓2^48 P per unit d). That is the stated model's answer, and K-D5 shows its
  standing. There is no tolerance.
- **The basis.**
  - Rank 2 on F with |F| = 3 gives one column t = c₁ × c₂.
  - Its entries are the 2×2 minors: exact values, generally not binary64 (the adopted design's
    example needs 200 bits).
  - `NodeBasis.q` becomes an entry type `QEntry = Binary64(f64) | Expansion(Vec<(f64, f64)>)`,
    the exact sum of products.
  - Assembly uses fl(t), formed once from the exact value. K-D5 evaluates the intended
    QᵀK_intQ with the exact expansion (four-factor splits), and charges the difference between
    the assembled fl(t)-system and the intended one in ε_E, as every formation difference is
    charged (§10.3).
  - Rank 3 means the node's translations are fixed and T is empty.
- **Attribution.** With r independent rows, λ is unique. Solve Cᵀλ = r on the r × r minor of
  largest exact |det|, ties to the lowest row set, by exact Cramer numerators and one certified
  division per multiplier. The remaining rows are a consistency check within bounds. Examples:
  `K2_orthogonal_pair` (+4P/5, +3P/5, against the axis pair's 0 and P), `K2_oblique_pair`
  (+20P/7, −15P/7) and `K2_three_stops_fixed`.

---

## 6. Assembly and the reduced system

### 6.1 The skewed spring block (S-K1)

- With n = d̃ᵀd̃ formed exactly and rounded once, its entries are K_s,ij = fl(k·fl(fl(d̃_i·d̃_j)/n)).
  q_ij = q_ji, so the block is symmetric bit for bit.
- It is added by `assemble_sparse_stiffness` through a new `SparseAssemblyOptions` member
  `node_blocks: Vec<(usize, [[f64; 3]; 3])>`. Blocks are added after every axis spring, in
  `basis.springs()` order, row-major, to the node's existing diagonal 6×6 pattern block (§2.2
  item 6 guarantees one).
- With no blocks the assembly is unchanged, bit for bit.
- K-D5's intended block is k·d_i d_j/(dᵀd) at p = 128 from the authored components (WideArith;
  scale-free, so the exponent range is not at issue). The formation difference is charged in ε_E.
- A block entry that rounds to zero or subnormal is a formation rounding and charged; it is never
  a silent drop, because the intended value is used.

### 6.2 K′ = QᵀKQ (S-K1)

- **Pattern.** Q is node-local, and a connected node's three translation rows share one column
  set (node blocks), so every entry of K′ lies in K's pattern.
- **Untouched entries.** Rows and columns away from constrained nodes are K's bits.
- **Each touched entry** is one exact sum Σ q_ir·K_ij·q_jc rounded once:
  - split K_ij·q_jc by TwoProduct into (h, l) (exact when the product is zero or at least
    2^−969 in magnitude);
  - `add_product(q_ir, h)` and `add_product(q_ir, l)`.

  The exact value is symmetric, so K′ is symmetric bit for bit.
- **Range.** A product below the split range, or a non-finite rounding, is
  `BasisError::SplitRange` or `NonFinite`. The case is then refused by the existing ordinary
  failure path with the error named (S-K1 never rounds such a product).
  - **As built (RT-A3 condition 1).** NI's linear skew entry maps the two errors to
    `StructuralError::NumericallyUnresolved` with the reasons
    `restraint_basis: split range in the coordinate transform` and
    `restraint_basis: non-finite rounding in the coordinate transform`, and `global_dof:
    None`, because a coordinate is not a physical DOF.
  - PP publishes it on its existing `NumericallyUnresolved` arm, as a blocking
    `NUMERICAL_INTEGRITY_UNRESOLVED` for the case. It never takes the pivot-screen special
    case (`PIVOT_SCREEN_REASON`) or the `InvalidInput` backstop.
  - It is a refusal of the case. It is not K-D5's §10.7 demotion to Sensitive, which applies
    to a case that solved.
  - **As built (Astra B4, RT-A3's re-check criteria).** The transformed load's and the
    reduced coordinate load's range or non-finite failures map the same way, split on the
    typed variant: `BasisError::Ledger(LedgerError::Sum { error: NonRepresentable |
    NonFinite })` gives `restraint_basis: the transformed load is outside binary64`, and
    `BasisError::Reduction(FrameKernelError::NonFiniteInput)` (in the skew reduction, only
    the adjusted force's rounding) gives `restraint_basis: the reduced coordinate load is
    outside binary64`.
  - A shape or sum error stays the backstop, because it is outside the entry's contract:
    the ledger's `EmptySource`, `DofOutOfRange` and `Sum { AccumulatorOverflow }`, and the
    reduction's length and DOF errors.
  - Regressions: a UZ guide and a stop (1, 2⁻⁶⁰⁰, 0), whose k·q·q falls below 2⁻⁹⁶⁹; and a stop
    (1.5, 1, 0) under the finite force (−1e308, 1e308, 0), whose free-coordinate force is
    2.5e308. Each is tested in both modes, at NI and through PP's arm, with no rows for the
    case.
- **When.** Formed per case in the skew branch, from the basis's formed K. This is O(entries in
  constrained rows) work plus one copy of the values. Caching per modulus basis is an
  optimization left to the implementation.

### 6.3 The right-hand side and reduction

- f′ = Qᵀf is formed from the physical ledger, term by term, on each touched coordinate row r.
  For each term on a physical DOF i with q_ir ≠ 0:
  - `ForceTermKind::Term(v)` gives `add_product(q_ir, v)`;
  - `Product(a, b)` splits q_ir·a (TwoProduct) and adds each part × b.
- The reduced free row is that exact sum minus Σ_c K′_rc g_c (exact products), rounded once.
- Restrained coordinates are the axis DOFs (value g as today; zero at a constrained node) and
  the pivots (value 0).
- The ledger itself stays physical: load closure and K-D5 bind physical terms (§10).

### 6.4 The record's DOF token

A skewed record's single DOF token (its restraint, or `stiffness.dof`) states only that the
record is translational (0.9.0 §2.1). It is never restrained, never a spring DOF and never a
reaction slot. d̃ alone governs.

### 6.5 Byte identity of everything else

The basis is built only when the lowered model holds a skewed record. PP holds it as
`Option<RestraintBasis>`, which is `None` otherwise. Every new call sits under that `Some`, or
under `has_constraint_transform()`. With `None`, PP, FK, NI and modal execute today's code, and
`pressure_free_byte_identity` and every existing fixture are unchanged.

---

## 7. Loads, reactions and their publication

### 7.1 Rows

- **Each skewed record:**
  - six `support_reaction_component_v2` rows as today: Fx, Fy, Fz are the global
    support-on-pipe force R; Mx, My, Mz are +0.0;
  - the magnitude rows as today, where |R| = |s|;
  - one new row per case:
    - `kind`: `support_reaction_along_direction_v1`, value s, unit `N`, `entity_ref` the
      support id;
    - id `result:support-along-direction-v1:{exact_source_identity([case, support])}`;
    - `metadata`: `basis` the record's component-row basis; `component` `along_direction`;
      `coordinate_system` `support_direction`; `location` `node`; `sign_convention`
      `support_on_pipe_positive_along_authored_direction`.
- **An axis record at a constrained node** publishes its attributed component (§7.2), not the
  raw row residual.
- **An axis-aligned `direction`** is lowered and publishes no along-direction row. This keeps
  byte identity (0.9.0 §2.1: for −e_x the along-d value would be the exact negation of today's UX
  component). The ±axis twins in the references (`P_AX_*`) pin it.
- The new kind is a linear state kind. It is listed with `support_reaction_component_v2` in PF
  `preview_physics.rs`'s row kinds and in PF combinations (`mechanics`, `absolute_sum`, `srss`,
  subtraction), so a combination's along-direction value is the combination of the case
  values.
  *As implemented (R-1 desktop review, Agent 0's correction 2):* the agreed row id
  `result:support-along-direction-v1:` + the exact source identity of (case, record) names the
  case, as the `_v2` kinds' ids do, although the kind ends in `_v1`. The producer, RE and the
  Python verifier therefore treat it as those kinds (`skew::case_named_id`): every case's row
  keeps the agreed id unqualified, and a combination forms it in its support's group, beside
  Fx–Mz, as `result:combination:<id>:support-action:<n>:<record>:along_direction`, with the
  group's derivation and sign convention. Before this, a second case's row was qualified
  (`result:loadcase:…`, refused by the reader as `ALONG_DIRECTION_ROW_ID`) and no combination
  over two or more cases published one; the single-case fixture hid it. Pinned by
  `fixtures/results/skew_along_direction_combinations_preview.json` (two cases; signed,
  absolute-sum, SRSS, envelope and subtraction combinations).

### 7.2 Attribution (binary64 path; K-D5's certified twin in §10.5)

At a constrained node, let r = (K u − f) on its three translations, where u is the lifted
displacement and each row is one exact sum rounded once, as `restrained_reactions`.
- **Stop: range-safe multiplier (review P1-1).** n_F = Σ_{i∈F} d̃_i² is not formed directly. It
  can underflow in an admitted, well-defined case: d = (1, 2⁻⁶⁰⁰, 0) with UX and UZ held gives
  n_F = 2⁻¹²⁰⁰, which rounds to 0, although the rank is exactly 3 and the true multiplier is
  2⁶⁰⁰ per unit d̃. Instead the restricted direction is rescaled exactly:
  - ŝ and w = 2^ŝ·d̃ are §5.1's restricted rescale: exact, ŝ ≤ 1022, |w_a| < 2^1023 for
    a ∈ A, and max_{i∈F}|w_i| ∈ [1, 2).
  - n_w = Σ_{i∈F} w_i² ∈ [1, 12), formed exactly and rounded once, so it never underflows.
  - **The numerator stays exact (revision 3).** N_w = Σ_{i∈F} w_i r_i is held exactly in an
    `ExactAccumulator`, never rounded on its own. An unrepresentable N_w does not mean an
    unrepresentable reaction. Example: d = (1,1,0), r = (2¹⁰²³, 2¹⁰²³, 0) gives N_w = 2¹⁰²⁴,
    while μ, both components and s = √2·2¹⁰²³ are finite.
  - **The quotient is range-safe.** Let k = 0 when N_w rounds to a finite binary64, otherwise
    k = 4.
    - N′ = `round_scaled(−k)` of N_w. It is finite in both cases, since |N_w| < 6·2¹⁰²⁴.
    - μ′ = fl(N′/n_w), and μ = μ′·2^k. With k = 4, |N′| ≥ 2¹⁰²⁰ is normal, so μ is exactly what an
      unbounded-exponent rounding would give.
    - μ is the multiplier per unit w; per unit d̃ it is 2^ŝ·μ. In exact arithmetic
      r_F = μ w_F, so μ is the unique multiplier, not a split.
  - **Stop components:** R_i = fl(μ·w_i) for every i. They are formed from the finite μ; if μ′·2^k
    itself overflows, see Range.
  - **Along-direction value:** s = round(2^ŝ·μ·ν), with ν = `correct_norm::norm3(d̃)`
    (correctly rounded, in [1, 2√3)). The exact product μ·ν is formed in an `ExactAccumulator`
    and rounded once by `round_scaled(ŝ)`, so no intermediate overflows.
  - **Range (revision 3).** `SUPPORT_SKEW_REACTION_RANGE_UNSUPPORTED` fires only when a
    **published** value is outside binary64: a stop component R_i, an axis component, or s.
    - μ is not published.
    - If μ′·2^k overflows, the stop component at the restricted pivot (|w_m| ≥ 1) has
      |R_m| ≥ |μ|. That published component is then out of range, and it is the value named.
    - Nothing is flushed or published as infinite.
  - **Regressions** (`attribution_range` in the references, inputs r given directly to
    `attribute`):
    - `N_w_overflow_published_finite`: k = 4, μ = 2¹⁰²³, R = (2¹⁰²³, 2¹⁰²³, 0), s =
      0x1.6a09e667f3bcdp+1023; **no refusal**.
    - `published_along_overflow`: r = 1.5·2¹⁰²³(1,1,0). μ and both components are
      1.5·2¹⁰²³ (finite); s = 1.5·√2·2¹⁰²³ is not; **refused**, on the along-direction value
      only.
    - `tiny_restricted_direction`: d = (1, 2⁻⁶⁰⁰, 0) with UX and UZ held, r_y = 1.
      n_F = 2⁻¹²⁰⁰ rounds to 0; ŝ = 600, n_w = 1, μ = 1. Stop (2⁶⁰⁰, 1, 0), UX −2⁶⁰⁰,
      s = 2⁶⁰⁰.
    - The model-level twins are `R4_tiny_restricted_direction` (kernel and product).
- **Axis record a ∈ A.** fl(r_a − μ·w_a), as one exact sum (r_a plus the exact product −μ·w_a,
  with μ the finite binary64 above) rounded once. If that sum is outside binary64, the case is
  refused on that published component.
- **Skewed spring.**
  - t = d̃ᵀ(u − 0) is an exact dot, rounded once. The ground is fixed in S-K1; S-K3 adds g.
  - Both values branch from the one product κ = fl(k·t); there is no chained division.
  - Components: R_i = fl(−fl(κ/n)·d̃_i), where n = d̃ᵀd̃ is formed exactly and rounded once.
    This is −k t d̃_i/n.
  - Along-direction value: s = −fl(κ/ν), with ν = `correct_norm::norm3(d̃)`. This is −k t/|d̃|.
  - For `S1_spring_isotropic`: R = (6/25, 8/25, 0) P and s = +2/5 P.
- Units are N, as for the force components. Rotational skew is refused.

### 7.3 Transformed loads

Loads stay physical everywhere they are published or certified. Only the solve's right-hand
side is Qᵀf (§6.3). Moved spring bases (R-c) on axis springs at any node, including a
constrained node, are physical ledger terms (k·g) and transform like any load.

---

## 8. Imposed motion

### 8.1 Admitted in S-K1

- Every W-16 motion at nodes with no constrained translation.
- Rotational prescribed values at a constrained node.
- R-c base motion of axis springs anywhere.

The partition carries them unchanged: their DOFs are axis coordinates.

### 8.2 Refused in S-K1

- §2.2 item 5: a primitive targeting a skewed record.
- §2.2 item 9: a nonzero prescribed translation at a stop's node.

### 8.3 S-K3 (designed; item 5 takes meaning B, decided in §14 C-1)

**The constraint right-hand side first, then the particular solution (review P2-5).** A
motion primitive on a skewed record does not supply a particular solution. It supplies the stop
row's right-hand side h, so the node's constraints are C u = (g_A, h). Here g_A are the axis
rows' prescribed values, from their own W-16 primitives or load-state motion. In scaled form the
stop row reads d̃ᵀu = h̃ with h̃ = 2^s·h. Only after that is the complete system C u₀ = (g_A, h̃)
solved for u₀:

- **The particular solution (one stop, axis rows A, pivot m ∈ F).** u₀ = Σ_{a∈A} g_a e_a + τ e_m,
  with τ = (h̃ − Σ_{a∈A} d̃_a g_a)/d̃_m = (2^ŝ h̃ − Σ_{a∈A} w_a g_a)/w_m.
  - It satisfies every row exactly: e_aᵀu₀ = g_a and d̃ᵀu₀ = h̃.
  - **The range (revision 3).** |d̃_m| alone lies in [2⁻¹⁰²², 2), not [1, 2) (§5.1), so τ is
    formed with §5.1's restricted rescale: the divisor satisfies |w_m| ∈ [1, 2).
    - The numerator 2^ŝ h̃ − Σ w_a g_a is formed exactly for A, or as a ball for B, in
      `WideArith` (i64 exponent), so no intermediate leaves its range.
    - τ itself can be large when d̃_F is small, because the constraint plane's offset along e_m
      is then large.
    - τ_b = round(mid([τ])), rounded once. If τ_b, or the formed term K e_m τ_b, is outside
      binary64, the case is refused by name, `SUPPORT_SKEW_MOTION_RANGE_UNSUPPORTED`. That is
      an S-K3 code, designed here and not emitted by S-K1.
  - τ is an exact datum, but generally not a binary64 value: it is a quotient by w_m, and under
    meaning B also irrational.
  - **The radius about τ_b (revision 3).** [τ] is a certified `WideArith` ball (midpoint
    mid([τ]), radius rad([τ])) containing τ, formed from the exact numerator, the exact w_m and,
    for B, the certified square root.
    - A tight ball may contain no binary64 value at all, so rad([τ]) is not an error bound about
      τ_b.
    - The radius used about τ_b is ρ_τ = |τ_b − mid([τ])| + rad([τ]), evaluated with upward
      rounding. Then |τ − τ_b| ≤ ρ_τ.
    - Example: in `K3_B_plane_13`, |τ − τ_b| ≈ 6.2106×10⁻²⁰. The references record each case's
      exact τ − τ_b and its upward binary64 bound (`tau.error_abs_upper_binary64`), which ρ_τ
      must not undercut.
- **The affine reduction.** u = u₀ + Q v with Q unchanged. The right-hand side becomes
  Qᵀ(f − K u₀).
  - K-D5 evaluates the intended right-hand side with u₀'s exact part and τ about τ_b. The term
    K_int e_m τ_b enters ρ exactly, and |K_int e_m|·ρ_τ enters ψ, as load re-formation radii do.
  - The lift publishes fl(u₀_b + Q v). Its lift radius also carries ρ_τ on DOF m, about τ_b and
    never rad([τ]) alone.
  - The intended u* = u₀ + Q v* satisfies C u* = (g_A, h̃) exactly.
  - S-K2's joint nodes solve C u₀ = g on §5.3's attribution minor by exact Cramer numerators,
    with one certified division per component.
- **A moving axis restraint alone** is h = 0 with g_A ≠ 0. Example: `K3_moving_guide_plus_stop`:
  UZ at h = 2⁻¹⁰ and stop (3,4,5) give 3u_x + 4u_y = −5h.

**The two candidate meanings of a primitive on a skewed record (§14 C-1; Agent 0 decided B, 2026-11-04).**
Each defines h only; the particular solution then follows from the complete system above.

- **Meaning B (the adopted §2.4 reading; the reviewer prefers it).**
  - A primitive moves the record's ground by g along +d/|d|. So h = |d|·g, equivalently
    h̃ = |d̃|·g, which is scale-free. It is generally irrational, carried as a certified ball;
    no rounded |d| defines the constraint.
  - A skewed spring under B acts with −k d̃(d̃ᵀu − |d̃| g)/n, with formed load k g d̃/|d̃|.
  - References with irrational norms, as exact values a + b√q:
    - `K3_B_plane_13` (d = (1,3,0), |d| = √10);
    - `K3_B_guide_plus_stop_345` (UZ held, d = (3,4,5), |d| = 5√2);
    - `K3_B_moving_guide_plus_stop_345` (UZ moved and the ground moved, both nonzero);
    - the rational control `K3_stop_ground_meaning_B` (d = (3,4,0)).
- **Meaning A (valid only if the owner deliberately chooses global-axis ground motion).**
  - A primitive with target DOF b moves the record's ground by the vector Δ = g e_b. So
    h = dᵀΔ = d_b·g, exact and division-free in h.
  - Δ is the ground's displacement, **not** a particular solution. With UX held and a stop
    (3,4,0) whose ground moves along X by g, Δ = g e_x would violate UX. The complete system
    gives u_x = 0 and u_y = 3g/4 (`K3_A_ground_x_with_ux_held`).
  - A skewed spring under A acts with −k d̃ d̃ᵀ(u − g e_b)/n, with formed load k g d̃ d̃_b/n.
  - References: `K3_stop_ground_meaning_A`, `K3_A_ground_x_with_ux_held`,
    `K3_A_guide_plus_stop_ground_y` and the product twin `P_K3_A_ground_x_with_ux_held`.

The meanings differ in what the authored primitive says. B keeps the adopted directional-motion
meaning; A reinterprets the target DOF as a global ground axis. The choice is the owner's, and
this design does not recommend A over B.

---

## 9. Ground screening

- **The PP count** (`lib.rs:3159-3176`, and modal's twin in `prepare.rs`) stays a necessary
  lower bound. It becomes |axis ground DOFs ∪ positive axis-spring DOFs ∪ contact DOFs| + the
  number of stops + the number of positive skewed springs. That is an upper bound on the rank of
  the ground rows, so the check never refuses a model whose rank is at least 6, and it is still
  not a stability claim. The message gains "skewed constraints: n".
- **`assess_rigid_body`** gains a row-based sibling, `assess_rigid_body_rows(coordinates, &[GroundRow], family)`.
  - An axis row is today's `motion_rows` line, by the existing code, bit for bit.
  - A direction row is Σ_a d̃_a·motion_rows(node)[a].
  - The existing function delegates with axis rows only.
  - Premise: a direction row's norm is in [1, 2√3). Scaling a row by a bounded positive factor
    cannot create an exact zero singular value or remove one, and the screen's existing
    thresholds are unchanged. No new threshold is introduced.
- **NI's `BodyEvidence::geometry`** takes the basis's ground rows when one is attached
  (`with_restraint_basis`), instead of deriving rows from the coordinate partition, whose pivot
  DOF is not a physical restraint.

---

## 10. Numerical certificate and standing (K-D5)

No new certificate. The existing K-D5 evaluation (D1 §4.3.1) is applied to the transformed
intended system. Its premises are stated below; each failure demotes the case by a named
`FormationCheckUnavailable` detail and never rounds.

### 10.1 The intended system

- K_int is today's p-formed elements and axis springs, plus the p-formed skewed spring blocks
  (§6.1).
- The intended reduced system is Qᵀ K_int Q v = Qᵀ(f_int − K_int u₀), with Q exact (S-K1:
  binary64 entries) and u₀ = 0 in S-K1.
- For every v, u = Q v satisfies dᵀu = 0 exactly, in exact arithmetic, so the constraint is the
  authored one (R-6). The rounded-division mutant is killed by `mutant`.
- **What is exact, what is published, and what the certificate covers (stated as the review
  requires).**
  - **Exact.** The **unrounded** lift u = Q v, for the binary64 coordinates v the solve returns,
    satisfies dᵀu = 0 exactly. Tests check this with an `ExactAccumulator`, and the mutant fails
    it.
  - **Published.** Displacement rows are, as for every PF case, the ordinary binary64 solution:
    here u_p = fl(Q v), each component rounded once. u_p need **not** satisfy dᵀu_p = 0 exactly;
    |d̃ᵀu_p| ≤ Σ_i |d̃_i|·½ulp(u_p,i) (× 2^−s for the authored d), and tests check that bound.
  - **Covered by the certificate.** K-D5's established groups (members, stations, reactions and
    along-direction values) enclose values of the intended exact solution u* = Q v*, where v*
    solves the intended reduced system and so lies on the constraint plane. They use
    u_p + w_p + w₂,p, the lift radius and the remainder (§10.4). The certificate makes no claim
    that the published u_p lies on the plane, and none that displacement rows are corrected.

### 10.2 The residual rows ρ (`evaluate`, `subtract_action`)

On free coordinate rows:
- the load side is each ledger term or re-formed primitive ball times q_ir (§6.3's products,
  radii × |q_ir| upward);
- the action side is −Σ q_ir·k_ic·q_cj·v_j for element, spring and block entries touching a
  constrained node. Split q_ir·q_cj, then each part × v_j (TwoProduct), and
  `add_product_to(row, ·)` with the Wide2 entry; at most four binary64 parts per term. Entries
  away from constrained nodes use today's code.

Premises:
- **P1 (split range).** Every TwoProduct used is of operands whose exact product is 0 or ≥
  2^−969 in magnitude (FK `connector.rs` `SPLIT_EXACT_MIN`). Otherwise the detail is
  `restraint_basis: split range at DOF {dof}`.
- **P2.** `ExactAccumulator` holds products of two binary64 values exactly (quantum 2^−2148).
  Triple and quadruple products are admitted only through P1's splits.

### 10.3 Formation and assembly allowance (`epsilon_e`)

- **Contributions.** The physical contributions are bound by identity and order as today. Skewed
  spring blocks are appended after axis springs as nine entries each, row-major, with binding
  value bits fl as §6.1 and identity (node, row, col).
- **Formation charge.** Each physical contribution's formation bound ε_ij (today's exact
  difference) is charged to coordinate entries (r, c) with weight |q_ir|·|q_jc|, upward.
- **Transform and assembly charge.** It is exact(Σ_ij q_ir (Σ_e contrib_e,ij) q_jc) − K′_stored,rc
  per touched stored entry, plus the existing projection term.
- The row and column sums and the maximum are unchanged. Premise: the triangle inequality on
  E = (K′_stored − QᵀK_asmQ) + Qᵀ(K_asm − K_int)Q, which is the existing argument with Q
  inserted.

The same mapping gives NI `SparseAssemblyEvidence`'s per-entry absolute roundoff on touched
entries, used by the M03 gate and the dense-scrutiny contribution audit: Σ|q_ir|·ar_ij·|q_jc| +
½ulp(K′_rc), upward. Premise: |K′_stored − QᵀK_exactQ| ≤ that bound by the triangle inequality.

### 10.4 Correction, D5C-1 and the member bound

- **Correction.** `correction` and the second correction w₂ are unchanged on coordinates. So
  are the binary64 factorization of K′_ff, the contraction and the Neumann bound λ_σ.
- **D5C-1** is judged physically at constrained nodes. For physical translation i there,
  |w_phys,i| ≤ Σ_j |q_ij||w_j| (upward), the scale is max(|u_i|, S*_trans) with u lifted, and the
  reported DOF is physical. Other rows are as today. `body_scales` reads the lifted u.
- **`lift_member_bound`** returns physical `MemberBoundInputs`:
  - u_p = fl(Q v) (published);
  - w_p = fl(exact(Q(v + w)) − u_p);
  - w₂,p = fl(exact(Q(v + w + w₂)) − u_p − w_p);
  - **`lift_radius`** (new field, `Vec<f64>`, empty meaning zero everywhere) is
    |exact(Q(v + w + w₂)) − u_p − w_p − w₂,p| rounded up;
  - exponent E_i is the least integer with 2^E_i ≥ Σ_j |q_ij|·2^(exp_j), over free coordinates j
    (exact dyadic sum). **When that sum is zero, E_i = `i32::MIN`** (review P2-4): no free
    coordinate maps to DOF i, so u_i is exactly prescribed and has no remainder. This covers
    every axis-restrained DOF and the pivot of a stop-fixed node (|F| = 1, e.g.
    `R4_full_fix_one_free_axis`, where u_m = 0 exactly). Its lift values and lift radius are
    then exactly 0.
- **`evaluate_row_inner`** adds ku·lift_radius[i] to the radius (no change when empty).
- Premise **P3**: the coordinate remainder bound |e_v,j| ≤ 2^(exp_j)·λ_σ maps to
  |Q e_v|_i ≤ 2^(E_i)·λ_σ, and lift_radius encloses the lift's rounding. So every existing
  recovery (members, stations, arcs, connectors) is unchanged in form.

### 10.5 Certified reactions (FK `certified_recovery.rs`)

- **Physical reaction rows include every skewed spring block (review P1-2).**
  `certify_support_reaction` today accumulates element rows and axis springs only. When
  `source.restraint` is `Some`, it also accumulates, after the axis springs, each skewed spring
  block touching the row's DOF. The block's intended entries are the p-formed k·d_i d_j/(dᵀd)
  of §6.1, as balls. They are applied to u + w + w₂ with the lifted bound: physical exponents
  E_i and the lift radius.
  - This applies to the three translation rows at a stop's node, and to every axis reaction at
    a node carrying a skewed spring, with or without a stop.
  - Omitting the block is a defect, not a tolerance matter. In `RS_stop_and_spring` (stop
    (3,4,0), spring (0,3,4)), dropping the block leaves r_z = 54/553 P against a stop direction
    with no z component, so attribution's consistency check fails on a valid case. The
    references record the omitted-block row (`regressions.reaction_rows_include_skew_springs`).
  - With `restraint: None` the function is unchanged.
- **`certify_node_attribution(source, force, basis, node, u, bound) -> CertifiedNodeAttribution`.**
  - r_i is the extended `certify_support_reaction` at each of the node's three translations. At
    the pivot DOF the exponent from §10.4 includes the remainder, unless the node is fixed
    (`i32::MIN`).
  - **The multiplier is range-safe, as in §7.2.** w = 2^ŝ d̃ and n_w = Σ_{i∈F} w_i² ∈ [1, 12)
    are exact. The numerator Σ_{i∈F} w_i r_i, the quotient μ, the components and s are all
    formed as `WideArith` balls (i64 exponent), so no intermediate leaves its range.
    - Each published value is the ball's midpoint, rounded once.
    - `SUPPORT_SKEW_REACTION_RANGE_UNSUPPORTED` fires only when that published rounding is
      outside binary64 (revision 3).
  - Consistency: each |r_i − μ w_i| for i ∈ F must be within their enclosures (radii and
    remainders). This is a necessary consistency check, not a proof of validity: validity rests
    on the §10 derivation and the references. Otherwise
    `RecoveryFailure` "restraint_basis: reaction not along the constraint", which leaves the
    record unavailable, as any recovery failure does.
  - Stop rows: μ·w_i. Axis rows: r_a − μ·w_a. The along-direction row is μ·2^ŝ·|d̃|, with a
    certified square root as in `certified_magnitude` and an exact `mul_pow2`.
  - A published value outside binary64 is §2.2 item 11.
- **`certify_skew_spring_reaction(spring, u, bound)` (review P1-3).** It mirrors
  `certify_spring_reaction`. t = d̃ᵀ(u + w + w₂) is formed by exact products per term, with the
  lift radius and remainder, and κ = −k·t is a ball. **Both outputs branch from κ:**
  - components: κ·d̃_i/n (n = d̃ᵀd̃ exact), i.e. −k t d̃_i/n;
  - along-direction: κ/|d̃| (certified square root), i.e. −k t/|d̃|.

  No output divides by n and then by |d̃|. `S1_spring_isotropic` pins +2/5 P, not 32/125 P.
- `certified_publication::support_record` gains the group `along_direction` (one row) for a
  skewed record. Readers re-check s·|d| = dᵀR within the published bounds (§11 R-1).

### 10.6 Force scaling (S-K2)

At 2^b, skewed blocks are formed at b = 0 times 2^b (exact), as connectors are. Q is
unchanged. K2b's step-5 reaction and range checks apply to the physical rows. S-K1 refuses
instead (§2.2 item 10).

### 10.7 Standing

A skewed case publishes K-D5's verdict as any PF case does. When a premise fails, the case is
demoted to Sensitive with the named detail. That is a truthful standing, never a refusal and
never a rounded certificate. The references' product cases are admitted within bounds that must
contain their exact values (§13).

---

## 11. Seams, their owners, and the coordination before implementation

| Seam | Where | Owner (lane) | What must be agreed before writing |
|---|---|---|---|
| F-1 FK assembly | `structural/sparse.rs` (`SparseAssemblyOptions.node_blocks`, `assemble_sparse_stiffness`), new `restraint_basis.rs` | DEL-04-01 (frame kernel). #122 also edits `sparse.rs` | The options member, its order after springs, and merge order with #122 |
| F-2 K-D5 | `structural/formation_check.rs` (`FormationSource.restraint`, `evaluate`, `subtract_action`, `epsilon_e`, D5C-1), `certified_recovery.rs` (`lift_radius`, two new certify fns), `MemberBoundInputs` | DEL-04-01 / DEL-04-08 (K-D5 owners) | §10's premises P1 to P3, and the new field on a pinned struct |
| F-3 ground screening | `rigid_body.rs` (`assess_rigid_body_rows`) | DEL-04-01 / DEL-04-06 | No new threshold (§9) |
| A-1 NI adapter | NI `structural_adapter.rs`: `SparseAssemblyEvidence::with_restraint_basis`, the allowance map, `BodyEvidence::geometry`, the formation source | RT-A3 P's lane holds NI; #122 edits this file | File-level merge order with #122; the adapter's linear entry only, never the loop |
| P-1 the P loop and its `complete_iteration` tail | NI `lib.rs` (#122 stacks `complete_iteration`) | RT-A3 P | S-K1 touches neither; S-K4 replaces `base_restrained_dofs` by the basis and rebuilds it per active set inside the loop and its tail (N-1) |
| N-1 confirmation | NI `confirmation.rs` (K-D5 nonlinear) | DEL-04-01 kd5-nonlinear / RT-A3 | S-K4: the confirmation evaluates on coordinates with the per-state basis |
| N-2 R-3's G2 (gap certification) | future G2 design (DEL-05-02 R-3) | the load-cases lane (G2's designer) | G2's certificate must be stated on the reduced coordinates when a stop shares a gap's node (S-K4) |
| N-3 the #97 nonlinear-state seam | `modal/state.rs` (typed state case) | dynamics lane (DEL-04-09) | S-K4: modal's linearized active DOFs rebuild the basis with the same `build` |
| N-4 friction-normal source | PP derived friction normals | DEL-04-04 / RT-A3 | A stop as a normal source stays refused at S-K4 until designed |
| D-1 dynamics' nullspace | `modal/stiffness.rs`, `modal/prepare.rs` | dynamics lane (DEL-04-09; #113 open) | Modal consumes K′, M′ by `congruence_terms`, Qᵀ M R_k and the lift **in the same PR** as the static lift (the refusal is shared through `judge`, §1) |
| D-2 snubbers and spectrum | DEL-04-10 | dynamics lane | Its snubbers use the same `SkewStop`/`SkewSpring`. REQ-011's influence vectors from support motion need S-K3's meaning |
| P-2 PP solve | `lib.rs` (build supports, the ground count, `form_basis_stiffness`, `solve_prepared_load_case`'s skew branch, reactions and publication), `certified_publication.rs`, `imposed_motion.rs` | DEL-04-07 / DEL-04-03. #122 edits `lib.rs` and `preview_physics.rs` | Merge order with #122 |
| M-1 document rules and lowering | `document_members.rs`, `operating_conditions.rs` | DEL-02-01 (S1's owner). #122 edits `document_members.rs` | The skew admission replaces the interim arm; codes per §2.2 |
| AP-1 applier | `operation_applier/src/lib.rs` `create_support` | authoring lane (DEL-02-01 / DEL-07-03), through the manager | §9's contract (below) |
| R-1 readers and contract | RE `preview_physics_evidence.rs`, `tests/verifiers/preview_physics_evidence.py`, `tools/serialization/semantic_contract_tables.py`, desktop known kinds | DEL-04-08 / DEL-05-02 readers; desktop lane | The new row kind's contract text, digests (1b's procedure, `operating-case.md` §6.6) and reader re-check |
| E-1 EX | `pressure_runtime`, W1 `exact_boundary.rs` | RT-A3 / R-4 lanes | S-K5 |
| PR-1 pricing | DEL-04-08 D3 | pricing registration owner | §12 |
| H-1 hangers | DEL-04-11 derived solve | hanger lane | No change. A non-candidate skewed record elsewhere takes the product solve. Candidates keep their refusal |
| I-1 import and SI writer | DEL-17-04 | MBF lane | No change; a later mapping |

### The model, applier and authoring contract for S-K1 (to the authoring lane through the manager)

- **`create_support` writes `direction`** exactly as entered (shortest round-trip JSON, never
  normalized) when:
  - the explicit `family` is `line_stop` or `spring`;
  - the record acts on exactly one translational DOF;
  - it has no `nonlinear` member;
  - the vector is finite and nonzero.

  The document rules (§2.2 item 1) give these codes. Both skewed and exactly axis-aligned
  vectors are written. The axis-aligned one lowers to today's record at the solve.
- **Refused at authoring by name:**
  - a skewed `direction` on a document with a pressure contract
    (`SUPPORT_DIRECTION_SKEW_EXACT_UNSUPPORTED`);
  - a skewed direction failing §4.2 (`SUPPORT_DIRECTION_RANGE_UNSUPPORTED`).

  `OP-CREATE-SUPPORT-PAYLOAD-INVALID` for an axis-aligned direction is removed.
- **Solve-time refusals not checked at authoring** (they depend on the rest of the model):
  the nonlinear-model code, a second stop at the node, redundancy and the node's motion.
  Precedent: 1b authors `difference_basis` and gates at the solve.
- **Not in S-K1:** editing or removing `direction` on an existing record (`update_support`,
  `set_field`) remains an unsupported field path until the authoring lane designs it. Desktop
  entry is the authoring lane's.

### 11.1 Seam agreements for S-K1 (2026-11-04)

Agent 0 released S-K1, stacked on #122's exact reviewed source. Agent 0 authorizes S-K1's
boundaries; the owners coordinate integration. Each owner below agreed its seam with these
conditions, and they bind the §15 implementation. Exact hunks go to each owner before the
write. Nothing in a shared file is written before agreement.

- **RT-A3 (P-1, A-1, and P's part of F-1).**
  - `pub mod restraint_basis;` in FK `lib.rs` is agreed; a full-file pin restates at D3.
  - **F-1.** With `node_blocks` empty, every P path stays byte-identical: K6 FK/SD binary64
    parity, P's `p_*` tests, and the shared-factor bit-identity refusal. Add one test in which
    a factor is shared across right-hand sides with node blocks present. S-K1 stacks on P's
    final `sparse.rs`; RT-A3 sends the SHA.
  - **F-2, technical agreement.**
    - `FormationSource.restraint` is carried on **both** the dense `ConfirmationAttempt` and
      P's sparse `SparseConfirmationAttempt` / formation-checked path, with a dense = sparse
      test on a skewed case.
    - `MemberBoundInputs.lift_radius` is empty when it means zero. Every existing
      construction stays bit-identical.
    - P2's justification: the smallest product of two binary64 values is 2⁻²¹⁴⁸, so the
      accumulator quantum holds every two-factor product exactly. Triples and quadruples are
      split by P1 first.
    - P3: |e_v,j| ≤ 2^exp_j·λ_σ holds for every free j at once, as the componentwise
      remainder. By the triangle inequality, |Q e_v|_i ≤ Σ_j|q_ij|2^exp_j λ_σ ≤ 2^E_i λ_σ.
      `i32::MIN` requires exactly-zero lift values.
    - §10.4 D5C-1 uses the published u_p = fl(Qv), compared upward.
    - **F-2's decider is SWBPIPE Agent 0, as DEL-04-01's decision holder** (2026-11-04; an Agent
      0 development decision, not the owner's personal acceptance).
      - **Authorized boundary:** `formation_check.rs`, `certified_recovery.rs`, `rigid_body.rs`
        and the new pinned `MemberBoundInputs.lift_radius`, as this reviewed design states.
      - **Before merge:**
        - RT-A3's technical review of the hunks;
        - an independent Astra MEDIUM numerical proof;
        - the actual D3 premises for frame, heap and source.
      - **Not assumed valid:** no field value. The radius's units, provenance, range and
        outward bound, and the old axis identity, are reviewed and tested.
      - **Preserved:** the FK, K-D5 and mathematical claims, with no hidden new normalization or
        parameter.
  - **F-3.** No new threshold.
  - **A-1.** The linear entry only.
    - `with_restraint_basis` is structurally unreachable from the loop: the nonlinear + skew
      refusal is structural, with a test that the loop path never carries a basis.
    - The loop, `complete_iteration` and `confirmation.rs` are never touched.
    - Add S11-K and R2 site-table rows for new adapter functions. R2's confirmation-entry
      rule is not widened.
- **RT-U4 (DEL-04-08 pricing; R-1 C1 mirror).**
  - **D1 memory.** `lift_radius` stays **empty on D1**: an empty `Vec` of capacity 0 when the
    model holds no skewed record, with a source check or capacity row (the D1.3 empty-owner
    pattern). KD5O4 then stays B(20N) and KD5O5 is unchanged. A dense field would price as
    B(28N) everywhere, which is not chosen.
  - **The struct grows.** `s(MemberBoundInputs)` goes from 80 B to 104 B with a fourth Vec.
    The pricing switch on `pub struct MemberBoundInputs` (`g3lib.py:245` fallback 80,
    `ordinary_caps.py:29`, `text_lexicon.py:174`) needs a new-revision switch keyed on the
    field's presence, so that older registered revisions keep their formula. That is D3 work.
  - **Debug text.** `MemberBoundInputs` reaches no Debug or format site; the S-K1 pricing
    notes state this, or the unified text model gains the field.
  - **Transform workspace.** Every rotation or transform allocation in
    `solve_prepared_load_case`, K-D5 `evaluate`/`evaluate_row_inner` and the NI adapter sits
    **inside** the direction branch. Anything that must be allocated up front gets a
    capacity-0 guard on D1. New owned layouts beyond the four are listed for the layout pins.
  - **C1 mirror.** C1's frozen candidate dd458d1d3 is not changed before its run.
    `support_reaction_along_direction_v1`'s exact strings and digests go to RT-U4 once
    implemented, and RT-U4 adds the mirror after the run. `recorded_tables.rs` re-pins to the
    merged bytes by whoever merges second.
- **Dynamics (D-1).** Agreed in principle. The eigenvalues of the restricted pencil do not
  depend on the basis, so a non-orthonormal division-free Q is sound. Conditions:
  - **(a) Re-formation.** `reform_modal_pencil` (`modal/outcome.rs`) and `modal_eigen`'s
    `ReformedPencil::new` / `formation_difference` apply the same `congruence_terms`, as exact
    three-factor products q_ir·k·q_jc into `ExactSum`, so that K̂′ = QᵀK̂Q and M̂′ = QᵀM̂Q.
    - Check 1 compares formed and re-formed reduced pencils. Checks 2 and 3 (residuals and
      orthogonality on M̂′) run on the reduced re-formed pencil. Sturm stays on formed K′, M′.
    - This touches `modal_eigen/src/reformation/*`, `modal/outcome.rs` and `integrity.rs`,
      and keeps #110's reset and capacity discipline.
    - *As implemented (the modal review of `ae1e47b60`):* a skewed spring's re-formed
      rank-one block k·d̃d̃ᵀ/(d̃ᵀd̃) is formed from the canonical d̃ = 2^s·d (`SkewDirection::scaled`,
      §5), the vector the formed K uses, never from the authored d, whose squares leave the
      double-double range at 2^±600. The block is the same exactly, and §4.3 I1 holds on the
      modal result: d·2^±600, d·2^−1020 and the subnormal pair give their twin's result bit for
      bit (`modal/tests_skew.rs`). The static product path, whose intended blocks use the wide
      exponent, is pinned the same way (`tests/skew_restraints_references.rs`).
  - **(b)** The weight-basis identity check (`modal/weight_basis.rs`) stays on the global M.
    The excitation uses the global M and R_k before Qᵀ.
  - **(c)** The census and structural masslessness are judged on the reduced M′, and the text
    says so.
  - **(d)** `modal_result_document` (modal-1 0.1.0) refuses skew models by name, in its
    caller and before `modal_doc_solved`, whose text §8.52 pins. Publishing the global M_ff
    with lifted shapes would be a modal-1 revision.
  - **(e)** One `build` per model, shared by static and modal.
  - **(f)** The congruence and re-formation terms stay countable for the later O-4 guard
    (≤ 9 terms per entry for a 3×3 node block).
  - The TASK writes the modal hunks; dynamics reviews them. One independent dynamics review
    of the modal hunks happens before merge.
  - #113 is open, with no dynamics hunks planned in `modal/stiffness.rs`, `prepare.rs` or
    `judge`.
- **Authoring lane (AP-1).** The `create_support` contract of §9 is agreed. Conditions:
  - **(a) Update paths.** `update_support configuration` replaces only its CONFIG members, so
    a kept `direction` survives an unrelated edit. A configuration change that makes a kept
    `direction` inadmissible is **refused by name** by the document rule, never written and
    never dropped, with a regression for each case:
    - family → `guide` or another non-`line_stop`/`spring` family;
    - multi-DOF restraints;
    - adding `nonlinear`.
  - **(b) Changing the single translational DOF token** (UX → UY) on a skewed record is
    well-defined. By §6.4 the token states only that the record is translational, and d̃ alone
    governs, so the solve is unchanged. A change to a rotational token makes `direction`
    inadmissible and is refused as in (a).
  - **(c) Agent authorship: Agent 0 decided (a), 2026-11-04.**
    - A restraint direction is geometry, taken from the person's explicit input or authorized
      geometric intent. The agent's catalog does not make it user-only.
    - The applier is authoritative for S-K1's admission scope and refusals. Authored non-zero
      vectors are preserved as given, with no silently inferred restraints, no normalization
      and no default orientation.
    - LIM-001 / O5 is unchanged: k, SIFs, stiffness, stiffness scaling and their source remain
      the person's to enter.
    - Catalog v4's free-JSON `after` does not admit every model member; the applier's checks
      remain the authority.
    - The next catalog revision Agent 0 assigns may add S-K1's implemented admission scope and
      its named refusals to `create_support`'s guidance. That is a catalog-byte change, re-pinned
      with Agent 0 and App v4. Until then the catalog is unchanged, and the admission above takes
      effect only when S-K1 is implemented.
  - Editing or removing `direction` stays unsupported until the authoring lane designs it.
    The desktop Direction field is the authoring lane's, after S-K1.

### 11.2 Phase-A hunk review: conditions for phase B (2026-11-04)

Phase A (head `83c154ae6`) wrote only FK `restraint_basis.rs`, its tests and the one agreed
`pub mod` line. The owners reviewed the hunk batch (`/tmp/sk1a-hunks-202611042132/`) and set
these conditions on phase B.

- **RT-A3, owner review (not an independent review).**
  - **Hunks 01–05 OK.** These cover F-1 node blocks, F-2 plumbing, the A-1/P-2 literal sites
    and F-3 rows.
  - **`FrameKernelError::InvalidInput(&'static str)` OK.**
    - The DEL-04-06 diagnostics mapping classifies it as a **named input refusal**: never
      numerical range, never range-deferred, never a solver failure that demotes a case.
    - It is a guard in depth: §2.2's earlier gates (item 6, item 10) refuse first in PP. A PP
      regression proves the named §2.2 code wins.
  - **F-3.** The `64·γ(n)·max` screen's n counts **ground rows, direction rows included**. A
    one-stop test pins n.
  - **P's `sparse.rs`** is final at `78580da7e`, which is already in this base.
  - **Spec 06** (the F-2 numerical bodies, A-1, P-2) is accepted as phase B's basis. The actual
    code hunks go to RT-A3 before merge, and F-2's to the Astra numerical proof. RT-A3's cert
    lines (−3466, −3684, −7229/+7252) are untouched.
  - **DEL-04-06 and DEL-03-06 are not RT-A3's.** Their signer is asked of Agent 0. Any shared
    `SPLIT_EXACT_MIN` is bit-identical to today's, leaves existing connector uses unchanged,
    and has a test pinning 2⁻⁹⁶⁹.
- **Authoring lane: AP-1 approved, with one should-fix.**
  - The `create_support` admission check must use the document validator's own item-1
    predicate, through one shared function, or run the skew gates only after item 1 passes.
    It must not re-implement that predicate. Today the re-implementation upper-cases and trims
    the DOF token, and treats `nonlinear: null` as absent.
  - Regressions, on an exact document: a skewed `line_stop` with `restraints: ["uy"]`, and one
    with `nonlinear: null`, each get the validator's item-1 code, not the skew code.
  - Optionally add `anchor` and `vertical_support` to (a)'s family regression.
  - One source for the code strings (M-1: PP `document_members` constants) replaces FK's
    `SkewCode` strings.
- **RT-U4, pricing at S-K1's D3.**
  - **B(20N) stands only with a registered premise.**
    - An `O10_EDGE_ZERO` or `O10_SITE_SIZE` 0 row for `RestraintBasis` methods, with its
      source check (D1 declines `direction`, `retained_memory.rs:1040`).
    - The D1 capacity-0 fact for `lift_radius`: `Vec::new()` at the one product
      construction.
  - **The 80 → 104 B switch** is keyed on `pub lift_radius: Vec<f64>` in `g3lib.py:244-245`,
    `ordinary_caps.py:29` and `text_lexicon.py:174`. Older revisions keep 80.
  - **Debug** is settled by the pricing's own format-site and call-graph inventory, not by
    grep. If `StructuralSolution`'s Debug is reached only from `#[cfg(test)]` sites, that is a
    stated premise.
  - **`assess_rows`' iterator chain collects nothing.**
  - **Layouts.** `SparseAssemblyOptions` (32 → 56 B) and `FormationSource` (+8 B) on a priced
    stack frame mean the AC-004 stack witnesses re-run at S-K1's D3, in an Agent-0-cleared
    exclusive window. That window is planned into the D3 order. New owned types are listed as
    zero on D1.
  - **B7a.** The PP `lib.rs` B7a restatement is #122's (minuend_final) D3 debt, not S-K1's.
    S-K1's D3 starts from the frozen predecessor after #122's re-pin. If the two are ever
    combined, both basis changes are named.
  - **C1 mirror.** It follows once phase B implements the row and its digests exist.

### 11.3 Agent 0's phase-B scope decision and dynamics' modal corrections (2026-11-04)

**Agent 0** decided the following. These are development decisions, not personal owner
acceptance. Agent 0 signs for this bounded cross-deliverable change, and the load-cases manager
owns implementation and integration in the S-K1 PR. General ownership of DEL-03-06 and DEL-04-06
does not transfer, and no S6 work is assigned.
- **DEL-04-06 diagnostics.**
  - `SkewDirectionRange`, `SkewStopsPerNode` and `ConstraintRedundant` join the existing
    `InvalidRestraint` mapping.
  - `FrameKernelError::InvalidInput(&'static str)` maps to a blocking ModelValidation input
    diagnostic. It is a backstop only, never NumericalRange, deferred-range, or a
    factorization or convergence failure.
  - The earlier named PP and applier refusals stay authoritative. Regressions show that the
    unconnected-node and force-scale refusals occur before the backstop on product paths.
  - Code, severity, source and detail are tested for the new mappings, and existing mappings
    are preserved.
- **DEL-03-06 split constant.** `connector.rs` owns the crate-visible `SPLIT_EXACT_MIN`;
  `tie_rod.rs` and `restraint_basis.rs` import it.
  - Every local duplicate of this threshold is removed.
  - The value stays `f64::from_bits(54 << 52)` = 2⁻⁹⁶⁹, with its bits pinned.
  - No change to the numerical predicates, existing connector and tie-rod uses, the public
    API, or the boundary and range reference behaviour.
- **F-2.** The 02 plumbing and the phase-B numerical scope remain authorized under this
  reviewed design, §11.1 and RT-A3's seam review.
  - The actual F-2, A-1 and P-2 code goes to RT-A3 and to the independent Astra MEDIUM
    numerical and persistence review. Compiled plumbing does not establish the numerical
    bodies.
  - The rigid-body screen's n counts all ground rows, with the single-stop test, and no new
    threshold is added.
- **Desktop routing correction.** `reactionOverlay.ts:15` (`REACTION_COMPONENT_KIND` =
  `support_reaction_component_v2`) is an arrow renderer's kind constant. Its loop at :123 consumes
  global Fx/Fy/Fz, Mx/My/Mz and magnitude rows. It is not a reader registry.
  - It is not replaced, and `along_direction` is not admitted as a component.
  - The new scalar (`coordinate_system: support_direction`) must not double-count a reaction,
    invent a global direction, or alter the standing or frame gates. The existing arrows go on
    consuming the emitted global component rows.
  - The actual reader, table and kind dispatch sites are updated through the maintained
    generators. A real-result consumer regression shows that adding the scalar leaves the
    component arrow unchanged.
- **Authoring's should-fix** is part of phase B (§11.2).
- **Debug and pricing.**
  - `MemberBoundInputs` reaches `StructuralSolution`'s derived Debug. At S-K1's D3 the
    actual-source text model, layout atoms, frame, call and heap owners and pinned strings are
    updated, including `FormationSource`'s added field.
  - An empty Vec is not assumed free of layout or text cost. Layout statements stay candidates
    until native layout and registration evidence exists, and historical revisions' pricing
    identities stay intact.
- **Reconciliation.** DEL-04-03, DEL-04-01, DEL-04-06 and DEL-03-06, and any changed consumer
  or dependency meaning, are reconciled in the same PR. Unchanged declarations are left alone.
- **Out of scope.** No native app run, no exclusive measurement, no public admission outside
  S-K1, and no release authority. Retained pricing and witness order stay P+cert → #122 → S-K1.

**Dynamics (D-1): the spec is OK, with these corrections.** Dynamics reviews the actual code before
merge, followed by one independent dynamics review.
- **C1. Structural singularity evidence.** `form_modal_pencil`'s zero-force structural solve
  (about :176–188) and its partition (`reduce_assembled_sparse_system`) use the same
  skew-reduced K′ on `restrained_coordinates` that the static path solves; otherwise the model
  is refused. The hunk names the static-path routine that supplies the evidence.
- **C2. Formed K′ and M′.**
  - Each entry is at most 9 three-factor terms over the **full symmetric** global blocks, using
    both M_ij and M_jiᵀ, since ModalMass stores only i ≤ j.
  - Entries are accumulated exactly and rounded once. The lower triangle is formed and
    mirrored, so the result is exactly symmetric.
  - `stiffness_asymmetric_entries` stays measured on the global K, before the transform.
- **C3. Check 1.** Its criterion covers formed K′ and M′ against the re-formed exact Q̃ᵀK̂Q̃ and
  Q̃ᵀM̂Q̃: at most one extra rounding per entry, plus |Q̃|ᵀ|E||Q̃|.
  - The difference measured on the AC-006 oblique case is reported.
  - Q̃ is the same rounded basis on both sides and in the lift. No orthogonality of Q̃ is
    assumed.
- **C4. `free_dof_mass`.** With a basis it is aₖᵀM′aₖ, where aₖ = Q̃_fᵀrₖ, accumulated in Dd and
  rounded once. For skew models it is labelled "kept-subspace mass". Unchanged nodes keep
  today's value bit for bit.
- **C5. Labels.** Rotated coordinates get their own labels (node plus skew coordinate index or
  basis id), and non-skew coordinates keep today's. No diagnostic prints a global DOF name for
  a mixed coordinate.
- **C6. Reduced versus lifted.**
  - On the reduced system: checks 1–3, Sturm, the τ_m census and structural masslessness on
    M′. A massless coordinate means an exactly zero row and column of M′.
  - On the lifted φ = Q̃φ′: τ_t (M1b), participation L = φ′ᵀQ̃ᵀ(M rₖ) with Q̃ᵀ(M rₖ) rounded
    once from Dd, member and node reporting, and the dᵀφ = 0 test.
  - Published shapes are lifted in binary64. Check 3's M̂′-orthogonality is the governing
    measure and is not re-measured on global M.
  - **As implemented, with dynamics' review B1.** The normalization case, τ_t and the pivot
    are chosen on the raw lifted vector φ = Q̃φ′. The published shape is
    fl(φ_d / pivot) over the global DOFs. Today's below-normal rule applies per global DOF:
    |φ_d| < |pivot|·2⁻¹⁰²², compared in WideDd, publishes 0 and flags d. So the reference
    entry is exactly 1, and every entry of the normalizing class (translations under
    UnitMaxTranslation, rotations under UnitMaxRotation) has magnitude ≤ 1; an entry of the
    other class can exceed 1, as on the axis path. The generalized
    mass, participation and effective masses are the reduced-coordinate quantities, on
    φ′/pivot with M′ and Q̃ᵀ(M r_k). They are consistent with the published lifted shape to
    rounding. Checks 2 and 3 read the reduced φ′/pivot. The coordinate re-formation has a
    limb-allotment variant (`new_with_coordinates_within_limb_allotment`, review N1) for
    O-4's staged guard.
- **C7. Resources.** `dense_scrutiny_guard` stays on the global 6N, before any allocation.
  - The kernel and its allotments see the reduced n′.
  - Congruence formation and re-formation are countable as terms per entry (≤ 9, ≤ 4 in S-K1)
    × entries.
  - Per-node scratch goes through #110's reserve and limb accounting.
- **C8.** `MODAL_DOCUMENT_SKEW_RESTRAINT_UNSUPPORTED` is raised in `modal_result_document`'s
  caller, before `modal_doc_solved`, so §8.52's pin is untouched.
  - It is registered wherever MODAL refusal codes are catalogued and tested.
  - A refused model produces no document.
- **C9.** Skew modal tests run under `test_local_parameters()` only. The production parameter
  instance on `claude/rt-dynamics-instance` is not involved.

---

## 12. Pricing outlook (no tool run, no profile)

- **Files and crates that change.**
  - FK: new `restraint_basis.rs`; `structural/sparse.rs`, `formation_check.rs`,
    `certified_recovery.rs`, `rigid_body.rs`, `lib.rs` exports.
  - LSUP `lib.rs`.
  - NI `structural_adapter.rs`.
  - PP: `lib.rs`, `certified_publication.rs`, `document_members.rs`, `operating_conditions.rs`,
    `imposed_motion.rs`, `preview_physics.rs`, `modal/prepare.rs`, `modal/stiffness.rs`.
  - Applier `lib.rs`; RE `preview_physics_evidence.rs`; the serialization tables.
- **Is the retained route reached (D1)?**
  - **The skew arms: no.** Retained admission refuses any `direction`
    (`FamilyFact::SupportDirection`, `retained_memory.rs:1040`), so on D1 the basis is `None`,
    `FormationSource.restraint` is `None` and `lift_radius` is empty. A premise rule
    ("SKW-D1: D1.6's support clause refuses `direction`") zeroes every new loop and site behind
    them.
  - **The identity path: yes.** The D1 graph reaches `solve_prepared_load_case`,
    `form_basis_stiffness`, `assemble_sparse_stiffness`, `evaluate`, `evaluate_row_inner`,
    `certify_support_reaction` and the adapter. Their pinned bodies change (the `Option`
    branches, one predicate scan of `model.supports`).
  - **Layouts change:** `FormationSource` (+ one `Option<&_>`), `MemberBoundInputs` (+ one
    `Vec`), `SparseAssemblyOptions` (+ one `Vec`), and `SparseAssemblyEvidence` (+ one
    `Option`).
- **Consequence.** One D3 actual-source turn with independent review (DEL-04-08
  `retained-pricing-rules.md`, AC-004's rules) before S-K1 merges. Expected movement: struct-size
  deltas and the one predicate loop; the registration decides. AC-004 stack witnesses run with no
  concurrent builds.
- **Minimizing rule for the implementation.**
  - New work lives in new functions called only under `Some(basis)` or
    `has_constraint_transform()`.
  - Do not add fields to `LinearSupport` or `BoundaryPreparation`.
  - Do not allocate on the identity path.

---

## 13. Verification

**References.** `validation/references/skew_restraints/` (README, generator, `references.json`,
`SHA256SUMS`). It is standard-library Python in exact rationals, reads no product code, and uses
two independent routes (KKT and RREF nullspace). Contents:
- **`direction_policy`:** 30 authored vectors with the expected class, scale, canonical vector,
  pivot, T and Q bits, or refusal code. The canonical T and Q are checked bit for bit.
- **`invariance`:** I1 to I3 groups.
- **`kernel`:**
  - §2.6's S-1, S-2, R-1 to R-4;
  - the S-K1 reductions, redundant and near-dependent cases, and the range regression
    `R4_tiny_restricted_direction` (n_F rounds to 0; n_w = 1);
  - each single-stop case's Q columns on F from w, and τ with its nearest binary64 τ_b, its exact
    error τ − τ_b and that error's upward binary64 bound;
  - per single-stop case an `attribution` block: F, the pivot on F, ŝ, w, n_w, μ, each physical
    translation's free coordinates or `i32::MIN`, and the particular solution u₀;
  - the S-K2 joint, orthogonal, oblique, near-parallel, parallel, antiparallel and three-stop
    cases;
  - S-K3: the moving guide; meaning A alone and with axis rows (`K3_A_ground_x_with_ux_held`,
    `K3_A_guide_plus_stop_ground_y`); meaning B as a rational control and with irrational norms
    (`K3_B_plane_13`, `K3_B_guide_plus_stop_345`, `K3_B_moving_guide_plus_stop_345`), given
    exactly as a + b√q with correctly rounded binary64 values.
- **`mutant`:** fl(1/3), with residual 2^−54 per unit coordinate.
- **`product`:** the guided cantilever (anchor, rotations held at B); exact u_B and every
  record's reaction for the S-K1 springs and stops (scaled, ×3, negated, plane (1,3,0), joint
  with UZ, full fix, redundant, stop + spring), the axis twins, S-K2 and S-K3.
  - The ±2⁶⁰⁰-scaled directions (`P_R1_stop_scaled_2p600`) are reachable only through the
    typed entry. The value entry's checked JSON refuses an integral float above 2⁵³
    (`CHECKED-JSON-UNSAFE-INTEGRAL-FLOAT`), as for the CE2 factor. The product test solves
    that case through the typed entry and asserts the value entry's refusal.
- **`modal_kernel`:** a point mass with a stop or a spring, exact ω².
- **`regressions`:** the review's findings on revision 1, each asserted exactly by the generator:
  - `spring_along_direction`: −k t d̃_i/n and −k t/|d̃|, against the rejected chain (2/5 against
    32/125 P);
  - `reaction_rows_include_skew_springs`: r = λd with the block; r_z = 54/553 P without it;
  - `attribution_range`;
  - `lifted_remainder_exponent_minimum`;
  - `particular_solution_not_ground_displacement`.
- **`attribution_range`:** direct `attribute` inputs.
  - `N_w_overflow_published_finite`: no refusal.
  - `published_along_overflow`: refused on s only.
  - `tiny_restricted_direction`.

  Each gives exact values and §7.2's binary64 values: k, N′, μ′, μ, components, s and axis.
- **`negatives`:** every code of §2.2 with its stage.

**Acceptance checks for S-K1** (each test named in §15):
- Product cases in both solver modes:
  - u_B and each reaction within 1e-9 relative of the exact value (an exact zero against 1e-9 of
    the case's largest magnitude), as W-16 does;
  - every established K-D5 group's bound contains the exact value;
  - the exact constraint check on the unrounded lift is exactly zero, and the published u_p's
    residual |d̃ᵀu_p| is within §10.1's stated bound;
  - `R4_tiny_restricted_direction` and its product twin publish finite values matching the exact
    ones (no division by zero), and its attribution block's ŝ, w and n_w bits match.
- I1 cases bit for bit equal.
- I3 cases equal within the stated bounds (guaranteed), with the along-direction value negated.
  Bit for bit in u and R up to the sign of a zero is the P-SYM test (§4.3), in both modes.
- Axis twins byte-identical documents.
- `pressure_free_byte_identity` and every existing fixture unchanged.
- Each negative's code and named object.
- Modal: the modal kernel ω² and a product-level oblique case (the dynamics lane's AC-006).

---

## 14. Reserved choices and obstacles

- **C-1: decided (Agent 0, 2026-11-04): meaning B**, the adopted §2.4 reading. A primitive on a
  skewed record moves its ground along d/|d|. The authored raw finite vector is preserved, and the
  certified enclosure and particular solution carry every ground term. A (global-axis
  reinterpretation) is not adopted merely to avoid an irrational representation. The reviewed B
  centre and range treatment (§8.3) applies; S-K3 ships only once implemented and reviewed. Each
  meaning defines only the stop row's right-hand side h. The particular solution then comes from
  the complete C u₀ = (g_A, h) (§8.3, review P2-5).
  - **B** (the adopted §2.4 reading; the reviewer prefers it): g along +d/|d|, h = |d| g, an
    irrational datum carried as a certified ball.
  - **A**: the ground moves by g e_b, h = d_b g, exact in h. It is valid only if the owner
    deliberately chooses global-axis ground motion.

  A is kept here only as the rejected alternative.
- **C-2: decided (Agent 0, 2026-11-04): after S-K1, S-K2 (joint stops) → S-K3 (imposed motion)
  → S-K5 (EX) → S-K4 (nonlinear).** This closes ordinary linear EX parity before further
  nonlinear PF expansion. If a reviewed design proves that S-K4 must precede S-K5, the exact
  dependency goes to Agent 0 with a minimal reorder; S-K1 is not blocked. There is no permanent
  PF-only release exception, and the MBF importer does not yet map skewed records (DEL-17-04);
  nothing here claims it does.
- **C-3: decided** (Agent 0, 2026-11-04): keep the range refusal (§4.4).
- **No exact obstacle to S-K1 was found.** One design limit is recorded: a skewed record on an
  unconnected node is refused (item 6), not supported.
- **Thresholds, eligibility, acceptance, P's step, the #97 seam, modal internals and
  registration timing** are not decided here.

---

## 15. First-implementation brief (S-K1)

An implementation TASK runs these steps in order on one branch (`claude/rt-skew-s-k1` or as the
manager names it). It commits per step. Before step 1 the manager has cleared the seams marked
"before".

**General rules.**
- Use cargo only through `/Users/ryan/dev/chirality-t3/tools/t3_cargo.sh`, with at most two
  slots and never exclusive.
- Use pytest only through `pytest_env.sh`, with explicit file lists.
- Select checks with `python3 tools/software_workflow/select_affected_checks.py
  software-workflow.json <paths>`.
- Every test that reads references uses
  `include_str!("…/validation/references/skew_restraints/references.json")` and parses the
  `exact` fields as rationals (num/den in i128 or a small big-rational helper in the test). It
  never regenerates them.

**Step 0. Seams (before).** The manager confirms with:
- the FK/K-D5 owners: F-1 to F-3, the new field and §10's premises;
- RT-A3 P: A-1 and merge order with #122; P-1 is untouched;
- the dynamics lane: D-1, modal consumption in the same PR (they write step 8 or review it);
- the authoring lane: AP-1;
- the readers' owner: R-1;
- DEL-04-08: PR-1.

Output: a note in the PR description of who agreed what.

**Step 1. `SkewDirection` and `RestraintBasis` (FK).**
- **Files:** new `core/solver/frame_kernel/src/restraint_basis.rs` and
  `restraint_basis_tests.rs`; `lib.rs` (`pub mod restraint_basis;`).
- **Functions:** all of §3.2 except `lift_member_bound` (step 4), using `ExactAccumulator`, a
  TwoProduct helper with the `SPLIT_EXACT_MIN` guard (reuse `connector.rs`'s rule; do not copy a
  new constant without a shared definition) and `correct_norm::norm3`.
- **Tests** (`restraint_basis_tests.rs`):
  - every `direction_policy` entry: class, scale, scaled bits, pivot, T and Q bits, codes;
  - I1/I3 groups;
  - each S-K1 `kernel` case on a single-node `SparseStiffness::from_dense` (6×6, translations =
    diag tip, rotations = 1, rotations restrained): K′ symmetric bit for bit, the lifted u within
    1e-12 relative, the exact constraint on the unrounded lift (ExactAccumulator of dᵀ(Qv) == 0),
    `attribute` within 1e-12 relative;
  - attribution's range safety:
    - `R4_tiny_restricted_direction` gives finite values equal to the exact ones, and ŝ, w and
      n_w match the `attribution` bits (the regression for n_F = 2⁻¹²⁰⁰, which rounds to 0);
    - every `attribution_range` case gives §7.2's binary64 bits;
    - `published_along_overflow` is the only one refused, and the refusal names the
      along-direction value;
  - Q on F equals `attribution.Q_on_F_columns_from_w` bit for bit in every single-stop case;
  - the published-lift bound: |d̃ᵀu_p| ≤ Σ|d̃_i|·½ulp(u_p,i), next to the exact zero of the
    unrounded lift;
  - I3 under P-SYM: bitwise u and R (up to zero signs) for the negation pairs. A failure is
    reported, not silenced (§4.3);
  - the `mutant` kill: a basis built with c = (fl(1/3), 1) gives a nonzero exact residual;
  - `K2_*` give `StopsPerNode`; `R4_redundant_axes` gives `Redundant`.
- **References:** `direction_policy`, `invariance`, `kernel`, `mutant`.
- **Seams cleared:** F-1 (module only).
- **Review point RV-1** (independent, numerical): the policy, Q, the transform's exactness and
  attribution.

**Step 2. Skewed spring blocks in assembly (FK).**
- **Files:** `structural/sparse.rs` (`SparseAssemblyOptions.node_blocks`; addition after
  springs; a node-block check; `force_scaled_inputs` returns
  `FrameKernelError::InvalidInput("skew node block at force scale")` when blocks are present);
  `structural/sparse/tests.rs`.
- **Tests:** with no blocks, existing tests unchanged; a block equals its dense addition; a
  missing pattern block is refused.
- **Seams cleared:** F-1.

**Step 3. Row-based rigid-body screen (FK).**
- **Files:** `rigid_body.rs` (`GroundRow`, `assess_rigid_body_rows`; `assess_rigid_body`
  delegates); tests in-module.
- **Tests:** axis-only rows give bit-identical assessments (existing tests); a single stop on
  an otherwise free body leaves five witnessed modes; six independent stops restrain.
- **Seams cleared:** F-3.

**Step 4. K-D5 on the transformed system (FK).**
- **Files:** `structural/formation_check.rs`:
  - `FormationSource.restraint: Option<&'a RestraintBasis>`; update every struct literal;
  - `skew_spring_matrix` at p;
  - the Q-weighted load rows in `evaluate`;
  - `subtract_action` and `epsilon_e` per §10.2 and §10.3;
  - D5C-1 per §10.4;
  - named `FormationCheckUnavailable` details.
- **More files:**
  - `MemberBoundInputs.lift_radius` (update the about 20 literal sites with `lift_radius:
    Vec::new()`), and `evaluate_row_inner`'s radius;
  - `restraint_basis.rs` `lift_member_bound`;
  - `certified_recovery.rs` `certify_node_attribution` (range-safe w and n_w, §10.5) and
    `certify_skew_spring_reaction` (both outputs from κ = −k t: components κ d̃_i/n,
    along-direction κ/|d̃|);
  - `lift_member_bound`'s exponent rule: `i32::MIN` whenever no free coordinate maps to the DOF
    (§10.4).
- **Tests:** new `structural/skew_kd5_tests.rs`:
  - `regressions.spring_along_direction`: the certified ball contains 2/5 P for `S1`, not
    32/125 P;
  - `regressions.lifted_remainder_exponent_minimum`: every physical translation of
    `R4_full_fix_one_free_axis` gets `i32::MIN`, and its lift radius is 0;
  - `R4_tiny_restricted_direction`: the certified attribution's balls contain the exact values;
  - the residual of the exact intended solution is zero, so the check passes;
  - a seeded displacement perturbation is detected;
  - every product S-K1 case's certified groups contain the exact values (a test-local frame
    model mirroring `product.spec`);
  - P1 failure gives the named detail;
  - with `restraint: None`, every existing K-D5 test is unchanged.
- **References:** `kernel`, `product`.
- **Seams cleared:** F-2.
- **Review point RV-2** (independent numerical scrutiny, required): P1 to P3, ε_E's mapping,
  the lift radius and attribution bounds.

**Step 4b. Physical reaction recovery includes every skewed spring block (FK; review P1-2).**
- **Files:** `structural/certified_recovery.rs` `certify_support_reaction`. When
  `source.restraint` is `Some`, after the axis springs, accumulate each skewed spring block
  touching the reaction DOF. The block is p-formed per §6.1 as balls and applied to
  u + w + w₂ with the lifted bound (exponents E_i, lift radius). With `None` it is unchanged.
  `certify_node_attribution` and every axis reaction at a node carrying a skewed spring use it.
- **Binary64 path:** the physical `restrained_reactions` already reads the assembled K, which
  holds the blocks (step 2). Its test asserts that.
- **Tests** (in `skew_kd5_tests.rs`):
  - `regressions.reaction_rows_include_skew_springs`: with the block the certified rows contain
    λ(3,4,0), and the attribution's consistency check passes;
  - a mutant that skips the block reproduces r_z = 54/553 P and fails the check;
  - an axis UZ guide at a node with a skewed spring and no stop: its certified row contains the
    exact reaction;
  - `P_RS_stop_and_spring` (product) in both modes.
- **References:** `regressions`, `kernel` `RS_stop_and_spring`, `product`.
- **Seams cleared:** F-2.
- **Review point:** part of RV-2.

**Step 5. LSUP construction.**
- **Files:** `core/solver/linear_supports/src/lib.rs` (`SkewSupport`, `SkewFamily`,
  `prepare_skew_boundary`, three `FindingCode`s); tests in-crate.
- **Tests:** from support records, the basis equals step 1's for the S-K1 kernel cases; codes
  for range, stops per node and redundant.
- **Seams cleared:** the DEL-04-03 code path.

**Step 6. NI adapter.**
- **Files:** `core/solver/nonlinear_integration/src/structural_adapter.rs`:
  - `SparseAssemblyEvidence::with_restraint_basis(&RestraintBasis)`: skewed spring
    contributions (nine entries each, after axis springs), the allowance map of §10.3, ground
    rows for `geometry` and `constrained_geometry`, and the formation source's `restraint`;
  - `solve_assembled_with_formation_check` uses them when attached.
- **Tests:** new `structural_adapter/skew_tests.rs`. Sparse and dense K1 identity on K′ solves;
  the M03 gate passes on the product case; geometry with a stop.
- **Seams cleared:** A-1 (merge order with #122 agreed in step 0).

**Step 7. Admission, lowering and imposed-motion refusals (PP).**
- **Files:**
  - `document_members.rs`: replace the skew arm of `interim_support_refusals` with
    `skew_admission` (codes 2, 3, 4 in order, one per record; messages state the object and the
    reason); `skew_message` is kept for history;
  - `imposed_motion.rs`: `validate` gives code 5;
  - new code constants.
- **Tests:**
  - new `core/product_physics/tests/skew_restraints_refusals.rs`: each §2.2 code and its named
    object, and the gate order with two failing conditions;
  - update the tests that asserted the interim code:
    - `tests/model_document_0_9_0.rs:343` and `tests/model_document_0_9_0_s2.rs:250`: the PF
      linear skew solves, so assert admission there, and add the EX and nonlinear twins for the
      new codes;
    - the applier's `lib.rs:16115-16117` and `tests/condition_set_authoring.rs:543, 678, 749-754`
      (in step 10).
- **Seams cleared:** M-1.

**Step 8. The static PF solve and modal (PP).**
- **Files and functions, static:**
  - in `lib.rs`, build skewed records into `SkewSupport` from the lowered model (beside
    `lib.rs:8911`; a skewed record is not a `LinearSupport`), with code 6;
    `prepare_skew_boundary` when any exists (findings mapped to codes 7 and 8); the ground count
    (§9);
  - `form_basis_stiffness(built, springs, basis, linear)` passes node blocks;
  - in `solve_prepared_load_case`, the skew branch: code 9 after `case_prescribed`; code 10 for
    `RangeDeferred` or an ordinary range failure; `transform_stiffness`, `basis.reduce`, and
    `solve_preview_reduced_system`'s skew sibling with `with_restraint_basis`; `lift`; physical
    `restrained_reactions`; `attribute` (range-safe, §7.2); code 11 for an out-of-range
    attributed value; rows of §7.1;
  - in `certified_publication.rs`: `source` sets `restraint`, `support` uses
    `certify_node_attribution` and `certify_skew_spring_reaction`, and `support_record` adds
    `along_direction`; `lift_member_bound` before any member recovery;
  - `preview_physics.rs`: the row kind in the linear kinds and combinations.
- **Modal** (the dynamics lane writes or reviews; same PR):
  - `modal/stiffness.rs` `form_modal_pencil(…, basis)`: K′_ff, M′_ff via `congruence_terms` in
    `Dd`, excitation Qᵀ(M R_k), shapes by `lift`;
  - `modal/prepare.rs`: the basis and the ground count.
- **Tests:**
  - new `core/product_physics/tests/skew_restraints_references.rs`: every `product` S-K1 case
    and twin in both solver modes, as in §13, including `P_R4_tiny_restricted_direction`, and
    the P-SYM negation test;
  - `pressure_free_byte_identity` and `formation_check_runtime` unchanged;
  - new `core/product_physics/src/modal/tests_skew.rs`: `modal_kernel` ω², shapes orthogonal to
    d, and a product oblique case.
- **References:** `product`, `modal_kernel`, `invariance`.
- **Seams cleared:** P-2, D-1.
- **Review point RV-3:** targeted regression checks on the product path (both modes) and an
  independent read of the integration.

**Step 9. Readers and contract.**
- **Files:**
  - RE `preview_physics_evidence.rs` and `tests/verifiers/preview_physics_evidence.py`: accept
    the kind; re-check s·|d| = dᵀR within the row's bound and the sign convention; check
    `along_direction` groups;
  - `tools/serialization/semantic_contract_tables.py`: the contract text for the kind and the
    group, then digests regenerated by 1b's procedure (`operating-case.md` §6.6:
    `derive`, `apply-u4b`, `repin --from-rev HEAD`, schemas manifest);
  - desktop known kinds (desktop lane's review).
- **Tests:** the existing reader corpus is unchanged; a new reader test fails when tampered (a
  wrong sign, a scaled s, a missing row). The independent reader must not import product code.
- **Seams cleared:** R-1.

**Step 10. Applier.**
- **Files:** `operation_applier/src/lib.rs` `create_support` (§11's contract);
  `tests/skew_direction_authoring.rs` (new); and the updated tests named in step 7.
- **Tests:**
  - written as entered (bits of the JSON numbers);
  - the axis-aligned is written;
  - EX and range refusals by name;
  - the document re-opens and solves on PF.
- **Seams cleared:** AP-1.

**Step 11. Reconciliation and pricing.**
- **Text:**
  - DEL-04-03 SOW: replace "Support coordinate policy beyond explicit node DOFs remains TBD"
    (CLM-005 DOF mapping, :115, R10, :219, :224, :293) with the built policy (axis DOFs, or one
    authored global direction on a single translational DOF of a `line_stop` or `spring`; S-K1's
    limits), and DEL-04-03:2's status (as built, with S-K2 to S-K5 remaining).
  - DEL-04-03 `deliverable.yaml`: `code_paths` gain `core/solver/frame_kernel/src/restraint_basis.rs`;
    `checks` gain the new tests.
  - DEL-02-01 0.9.0 §3.8's row for `SUPPORT_DIRECTION_SKEW_UNSUPPORTED`: "removed by S-K1 on PF
    linear models; the remaining refusals per DEL-04-03 Work `skew-restraints.md` §2.2".
  - DEL-04-09 and DEL-04-10 texts only if their conditions change (they should not).
  - This design's §16 "As built".
- **Pricing:** the D3 actual-source turn (§12) with independent review. Re-pricing tools only as
  DEL-04-08's rules say.
- **Review point RV-4:** the D3 registration review.
- **PR:** quote each old → new text, the checks on the exact head, RV-1 to RV-4, and the open
  items S-K2 to S-K5 and C-1 to C-3.

---

## 16. As built

S-K1, on `claude/rt-skew-sk1` (with `claude/rt-skew-sk1-r1`). This records what was built and
where. It claims no integration, acceptance, release or D3 registration, which stay with their
owners.

**Built (§11.1 seams, §15 steps 1-10):**
- **Basis (FK, RV-1):** `restraint_basis.rs` (`SkewDirection`, `RestraintBasis`,
  `congruence_terms`, transform, reduce, lift, attribution) on `connector.rs`'s shared
  `SPLIT_EXACT_MIN`; tested in `restraint_basis_tests.rs` against `direction_policy`,
  `invariance`, `kernel` and `mutant`.
- **Assembly and screens (FK):** `SparseAssemblyOptions.node_blocks`; `rigid_body` ground rows
  (F-3, n counts every ground row); the formation check's restraint source on the dense and
  sparse attempts (F-2); K-D5 on the transformed system with `lift_radius` and the certified
  reaction at skewed nodes (§10.5).
- **NI and LSUP:** NI's linear skew entry (A-1; the loop never carries a basis). LSUP
  `prepare_skew_boundary`, with the three codes in DEL-04-06's `InvalidRestraint` mapping and
  the `InvalidInput` backstop.
- **Product (PP):** `skew.rs` and `document_members` (§2.2 gates in order, one source of the
  code strings, M-1); `support_reaction_along_direction_v1`, one row per skewed record and case
  under the agreed id. A combination forms it in its support's group (§7.1 note, R-1).
- **Readers (R-1):** RE `along_direction_evidence` and the Python verifier re-check it, with
  the RE and Python combination mirrors. The tables went through the generator (`apply-sk1`),
  with the S-K1 base's tables kept as the `recorded_pre_sk1` variants in the Rust, Python and
  webview registries.
- **Modal (D-1):** the reduced pencil (C1-C9); shapes published on the lifted vector (B1).
  Spring re-formation is on the canonical d̃ (§11.3 note). The modal result document and the
  test-only phase replay refuse a skew model by name (C8 (a), (b)).
- **Applier (AP-1):** `create_support` authors `direction` under the one item-1 predicate and
  the skew gates. A present `pressure_contract: null` is refused by the shared validator
  (Agent 0, 2026-11-05).
- **Desktop:** the webview's handling of the kind: the support summary's per-case list with
  each row's own derivation and sign convention, the known kind, notices, CSV, and the
  `recorded_pre_sk1` registry. The reaction overlay is unchanged.

**Tests:** the product cases in both static modes, the ±axis byte-identity twins, the
power-of-two twins (static and modal, bit for bit), the refusals by name, the reader tamper
cases and the two-case combinations fixture.

**Open:**
- S-K2 to S-K5 (§2.3, in the order C-2 sets), with C-1 = meaning B and C-3 = keep the range
  refusal.
- The D3 actual-source turn (§12; RV-4). The retained profile reports Stale while its reviewed
  inputs, the preview-physics-2, retained-1 and timoshenko-1 tables, carry R-1's row; the
  MODAL pins (C8 (c)) are on that list.
