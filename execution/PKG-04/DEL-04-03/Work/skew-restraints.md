# Skewed restraints (R-6, R-7): build design, shared interface and first-implementation brief

> **Status: candidate design, not independently reviewed.** Design and references only: no product
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
| Authored vector | Stored as authored. The canonical form is d̃ = 2^s·d with max\|d̃_i\| ∈ [1, 2). It is admitted iff every nonzero d̃_i is normal; otherwise `SUPPORT_DIRECTION_RANGE_UNSUPPORTED`, with no rounding and no tolerance. Power-of-two rescalings are bit for bit invariant. Positive rescalings keep the exact system. Negation keeps u and R and negates the along-direction scalar (§4). |
| Rank | Per node and exact. S-K1 needs no arithmetic: with axis rows A and one stop, the rank is \|A\| + [d̃_F ≠ 0] on the free axes F. S-K2's joint case decides rank by exact 2×2 and 3×3 minors. A dependent set is `SUPPORT_CONSTRAINT_REDUNDANT`. A near-dependent set is admitted when exactly independent, as an exact reduction (§5). |
| Reactions | Global support-on-pipe components, plus a new row `support_reaction_along_direction_v1` (N, positive along authored +d). Attribution comes from the unique multipliers, never from a split (§7). |
| Certificate | K-D5's existing machinery on the transformed system QᵀK_intQ, with exact splits for triple products and the existing `SPLIT_EXACT_MIN` premise. The binary64 factorization and correction are unchanged. The member bound is lifted to physical DOFs with one new per-DOF lift radius. No new certificate is introduced (§10). |
| Pricing | The retained route never admits `direction` (D1 premise), so every skew arm is zero on D1. Pinned identity-path bodies and some layouts change, so a D3 actual-source turn is needed (§12). |
| Exact obstacles | None that blocks S-K1. Three reserved choices are raised in §14: the meaning of imposed motion on a skewed record, the order of later slices, and whether an exact fallback should widen the range rule. |

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
| 5 | `IMPOSED_DISPLACEMENT_SKEW_SUPPORT_UNSUPPORTED` (new, blocking) | `imposed_motion::validate`. An `imposed_displacement` primitive whose target is a skewed record | S-K3, after the meaning choice (§14 C-1) |
| 6 | `SUPPORT_DIRECTION_NODE_UNCONNECTED` (new, blocking) | Boundary preparation. A skewed record on a node with no element: the node's 3×3 block is not in the pattern | Permanent |
| 7 | `SUPPORT_SKEW_STOPS_PER_NODE_UNSUPPORTED` (new, blocking) | Boundary preparation. A node with two or more skewed stops | S-K2 (§5.3) |
| 8 | `SUPPORT_CONSTRAINT_REDUNDANT` (adopted name, blocking) | Boundary preparation. §5's exact rank is below the node's row count | Permanent (attribution would not be unique) |
| 9 | `SUPPORT_SKEW_NODE_PRESCRIBED_MOTION_UNSUPPORTED` (new, blocking for the case) | Case solve, before reduction. A nonzero prescribed translation on an axis restraint at a skewed stop's node (W-16 primitive or load-state motion) | S-K3 (affine reduction, §8.3) |
| 10 | `SUPPORT_DIRECTION_SKEW_FORCE_SCALED_UNSUPPORTED` (new, blocking for the case) | Case solve. A skew model's ordinary formation leaves the binary64 range (`RangeDeferred`) or would need W2/K2b. The ordinary failure is published as today and W2 is not attempted | S-K2 |

Also unchanged:
- `SNUBBER_DYNAMIC_ONLY_UNSUPPORTED` refuses every static snubber; DEL-04-10 consumes the basis.
- `HANGER_SIZING_CANDIDATE_SKEW_UNSUPPORTED` stays.
- The retained-route decline (`FamilyFact::SupportDirection`) stays.
- The CAEPIPE SI writer and MBF importer refuse non-axis directions. That mapping is
  DEL-17-04's, and none is scheduled here.

**Order within a stage.** Skew admission runs in `judge` and gives one code per record, in
document order, the first that applies of 2, 3, 4. Boundary preparation is per node, in
ascending node order, the first that applies of 6, 7, 8, and reports every node. Case-level
codes 9 and 10 are per case. A record refused at one stage does not reach later stages, because
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

- **The basis T.** Per constrained node, Q is node-local, with columns d̃_m e_j − d̃_j e_m
  (j ∈ F, j ≠ m), e_m for the pivot coordinate, and e_a for each axis-restrained a. Every entry
  is an authored scaled component, 1.0 or +0.0.
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
- **(I3) Negation, c = −1: u and the global reactions are bit for bit identical up to the sign
  of a zero, and the along-direction scalar is exactly negated.**
  - d̃′ = −d̃ and m′ = m. Q′ negates the columns j ≠ m and keeps e_m.
  - K′′ = Q′ᵀKQ′ equals K′ except that each entry in exactly one negated index changes sign
    (exactly, since negation is exact). So the reduced system is DK′_ffD v′ = D f′_f with
    D = diag(±1). The binary64 solves are sign-symmetric under round-to-nearest-even, so
    v′ = D v bit for bit, and u′ = Q′v′ = Q v.
  - R = K u − f is unchanged. The scalar s′ = R·d̃′/|d̃′| = −s.

  Signed zeros may differ. A review point for the implementation is to normalize published
  zeros as today.
- **(I4) Non-representable factors** are not rescalings of an authored binary64 vector; the
  user authors whatever binary64 vector they mean.

The references hold groups for I1 to I3 (`invariance`, `direction_policy`).

### 4.4 The exact fallback when scaling would be inexact

An inexact canonical scaling means the components span more than the normal range: the ratio
max/min is at least 2^1023. No exact power-of-two scaling then puts the largest component in
[1, 2) and keeps every component normal. Every other power-of-two scaling has the same span.
The adopted design's fallback is the refusal by name
(`SUPPORT_DIRECTION_RANGE_UNSUPPORTED`), and it is kept. Its message names the subnormal
component and states that an axis-aligned direction is the authored form if that axis is meant.

A wider exact fallback exists (§14 C-3). It scales so the *smallest* nonzero component is
normal, and lets formation's own finite checks refuse a too-large entry. It needs no threshold,
but it widens formation and certificate ranges, so it is raised, not adopted.

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
- **Q.** Columns d̃_m e_j − d̃_j e_m for j ∈ F∖{m}; e_m (the pivot coordinate, restrained at 0);
  e_a for a ∈ A (restrained at 0).

| \|F\| | d̃_F | Result |
|---|---|---|
| 3 | ≠ 0 | two free coordinates (§2.2 of the adopted design) |
| 2 | ≠ 0 | one free coordinate. If d̃_F has one nonzero entry, the column is d̃_m e_j: the stop fixes u_m, and still carries the reaction |
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

### 7.2 Attribution (binary64 path; K-D5's certified twin in §10.5)

At a constrained node, let r = (K u − f) on its three translations, where u is the lifted
displacement and each row is one exact sum rounded once, as `restrained_reactions`.
- **Stop.** λ̃ = (Σ_{i∈F} d̃_i r_i) / n_F, with the numerator an exact dot rounded once and
  n_F = Σ_{i∈F} d̃_i² exact rounded once. In exact arithmetic r_F = λ̃ d̃_F, so this is the unique
  multiplier, not a split. Then R_i = fl(λ̃·d̃_i) and s = fl(λ̃·|d̃|), with |d̃| from
  `correct_norm::norm3`.
- **Axis record a ∈ A.** fl(r_a − λ̃·d̃_a), as an exact sum rounded once.
- **Skewed spring.**
  - t = d̃ᵀ(u − 0) is an exact dot, rounded once. The ground is fixed in S-K1; S-K3 adds g.
  - μ = fl(fl(k·t)/n).
  - R_i = fl(−μ·d̃_i) and s = −fl(fl(k·t)/|d̃|).
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

### 8.3 S-K3 (designed; the meaning of item 5 is reserved, §14 C-1)

- **Affine reduction.** u = u₀ + Q v with C u₀ = g. Choose u₀ = Σ_a g_a e_a + Δ, where:
  - the axis-restrained values g_a sit at their DOFs;
  - Δ is the stop ground's particular solution;
  - Q is unchanged.

  The right-hand side becomes Qᵀ(f − K u₀). K-D5's residual and the lift carry u₀ exactly.
  Example: `K3_moving_guide_plus_stop`, with UZ at h and stop (3,4,5), gives 3u_x + 4u_y = −5h.
- **Meaning A (exact, division-free; recommended).** A primitive on a skewed record with target
  DOF a moves the stop's ground by g along the global axis a: Δ = g e_a, so dᵀu = d_a g. For a
  skewed spring, the force is −k d̃ d̃ᵀ(u − g e_a)/n and the formed load is k g d̃ d̃_a/n
  (certified division, as the block). Reference: `K3_stop_ground_meaning_A`.
- **Meaning B (the adopted §2.4 reading).** g along +d/|d|, so dᵀu = |d| g, with |d| generally
  irrational. u₀ = (g/|d|)·d is carried as a p-ball, and its radius enters the residual's ψ, as
  load re-formation radii do. Reference: `K3_stop_ground_meaning_B` (d = (3,4,0), |d| = 5).

Both are exact designs. The choice changes the authoring contract's meaning, so it is not this
design's.

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
    (exact dyadic sum). It stays `i32::MIN` at axis-restrained DOFs.
- **`evaluate_row_inner`** adds ku·lift_radius[i] to the radius (no change when empty).
- Premise **P3**: the coordinate remainder bound |e_v,j| ≤ 2^(exp_j)·λ_σ maps to
  |Q e_v|_i ≤ 2^(E_i)·λ_σ, and lift_radius encloses the lift's rounding. So every existing
  recovery (members, stations, arcs, connectors) is unchanged in form.

### 10.5 Certified reactions (FK `certified_recovery.rs`)

- **`certify_node_attribution(source, force, basis, node, u, bound) -> CertifiedNodeAttribution`.**
  - r_i = `certify_support_reaction` at each of the node's three translations. At the pivot DOF,
    the exponent from §10.4 includes the remainder; free coordinates have no prescribed columns.
  - λ̃ = (Σ_{i∈F} d̃_i r_i)/n_F in ball arithmetic: exact d̃, an exact n_F, and the certified
    division of `WideArith`.
  - Consistency: each |r_i − λ̃ d̃_i| for i ∈ F must be within the balls' radii. Otherwise
    `RecoveryFailure` "restraint_basis: reaction not along the constraint", which leaves the
    record unavailable, as any recovery failure does.
  - Stop rows: λ̃ d̃_i. Axis rows: r_a − λ̃ d̃_a. The along-direction row: λ̃·|d̃|, with a
    certified square root as in `certified_magnitude`.
- **`certify_skew_spring_reaction(spring, u, bound)`.** Mirrors `certify_spring_reaction`:
  d̃ᵀ(u + w + w₂) by exact products per term, with the lift radius and remainder, then
  × −k ÷ n, components × d̃_i, and the along-direction ÷ |d̃|.
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
- **`kernel`:** §2.6's S-1, S-2, R-1 to R-4; the S-K1 reductions, redundant and near-dependent
  cases; the S-K2 joint, orthogonal, oblique, near-parallel, parallel, antiparallel and
  three-stop cases; the S-K3 moving guide and both ground-motion meanings.
- **`mutant`:** fl(1/3), with residual 2^−54 per unit coordinate.
- **`product`:** the guided cantilever (anchor, rotations held at B); exact u_B and every
  record's reaction for the S-K1 springs and stops (scaled, ×3, negated, plane (1,3,0), joint
  with UZ, full fix, redundant, stop + spring), the axis twins, S-K2 and S-K3.
- **`modal_kernel`:** a point mass with a stop or a spring, exact ω².
- **`negatives`:** every code of §2.2 with its stage.

**Acceptance checks for S-K1** (each test named in §15):
- Product cases in both solver modes:
  - u_B and each reaction within 1e-9 relative of the exact value (an exact zero against 1e-9 of
    the case's largest magnitude), as W-16 does;
  - every established K-D5 group's bound contains the exact value;
  - the exact constraint check on the unrounded lift is exactly zero.
- I1 cases bit for bit equal. I3 cases bit for bit in u and R (up to the sign of a zero), and the
  along-direction value negated.
- Axis twins byte-identical documents.
- `pressure_free_byte_identity` and every existing fixture unchanged.
- Each negative's code and named object.
- Modal: the modal kernel ω² and a product-level oblique case (the dynamics lane's AC-006).

---

## 14. Reserved choices and obstacles (raised, not decided)

- **C-1 (Agent 0 / authoring lane): the meaning of an imposed-motion primitive on a skewed
  record.** Option A is a global-axis ground displacement: exact and division-free, with the
  existing primitive shape, but it reads the target DOF as a ground axis, not "the record's DOF".
  Option B is g along +d/|d|, the adopted §2.4 reading: an irrational datum carried as a p-ball.
  S-K3 is blocked on it; S-K1 is not.
- **C-2 (Agent 0): the order of S-K2 to S-K5.** EX (S-K5) matters for pressurized imports, but
  the MBF importer does not map skewed records yet (DEL-17-04).
- **C-3 (Agent 0): widening §4.4's fallback** from refusal to "scale the smallest component to
  normal". It is exact and needs no threshold, but it widens formation ranges. Recommendation:
  keep the refusal.
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
  - `certified_recovery.rs` `certify_node_attribution` and `certify_skew_spring_reaction`.
- **Tests:** new `structural/skew_kd5_tests.rs`:
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
    `restrained_reactions`; `attribute`; rows of §7.1;
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
    and twin in both solver modes, as in §13;
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

Not built. This section is filled by the S-K1 implementation.
