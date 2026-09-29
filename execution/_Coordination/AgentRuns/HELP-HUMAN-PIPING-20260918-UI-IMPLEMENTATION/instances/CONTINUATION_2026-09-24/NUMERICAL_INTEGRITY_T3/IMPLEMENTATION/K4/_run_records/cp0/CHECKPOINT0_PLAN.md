# I12 / K4 — checkpoint 0: the plan (no product code)

- **Brief:** `TASK_BRIEFS/I12_K4_IMPLEMENTATION.md` at `ca5a489d3` (sha256 `55e5c5a2…4845848`), with its binding "ROOT rulings for this slice" (Q1–Q12) and `ROOT_RULINGS_V1.md` "K4: spawn and rulings (ROOT, 2026-09-28)" (stale-design items 1–10 as rulings).
- **Base:** `<wt>/k4`, branch `codex/piping-k4-20260928`, main `e7d930d49` (K1, K3, K2b merged). Nothing written in `<wt>/k4`.
- **Read:** Root `AGENTS.md`, `agents/AGENT_TASK.md`, `_COMMON.md`, `I8R_K1_RESUME.md` ("The Mac host", "Platform calibration"), D1 `DESIGN.md` rev 5a.2 (sha256 `fb62ef4a…`, verified; §1, §3.1–3.2, §4.1–4.4.1, §4.10, §4.11, §5, §6, §7.1, §7.3, §7.4), K3 `RETURN.md` (§3, §4, §5, §7, §8, §14, §15, addenda 1–2), K2b `RETURN.md` (§6, §9, §19), K-D5 `RETURN.md` (§3a, §3), R1 `README.md`, `references.json`/`references.py` (hashes `7b176dbb…`/`80d473a7…`, verified on the base and on `<wt>/numerics`), `numerical_integrity/{README.md, fixtures.json}`, D1's `probe_skew_precision.py`, `probe_rev2_b1.py`, `floor_kinds.py`/`floor_kinds.json`, and the code listed in the brief's Basis item 11.
- **Ran (read-only / scratch):** `references.py --model RF-MECH-LINE-IN-CHAIN1000` (1,007 nodes, 1,005 members; output `<scratch>/model_line_in_chain1000.json`); two standard-library scratch probes (`cp0_exact_formation_probe.py`, `cp0_by_kind.py`, §17). No cargo, no Git write.

Everything below is a proposal for ROOT. Items marked **[POSITION]** are my choices inside the brief; **[REFINE]** departs from or sharpens the brief's letter; **[OPEN]** needs a ruling; **[STOP?]** is a candidate stop item.

---

## 0. Re-located citations (brief drafted on `57617b0fb`; base `e7d930d49`)

K2b's hunks on these files are either after the cited lines or line-neutral (`structural.rs` `@@ -543,7 +543,7` and `@@ -2185 …` after `negative_pair_witness`; `sparse.rs` `@@ -36,8 +36,8` then `:379` onward; `load_ledger.rs` from `:320`; `lib.rs` from `:852`). **Every cited line is unchanged on the base:**

| Brief citation | On `e7d930d49` |
|---|---|
| `FK/structural.rs` `mod retained;` `:5` | `:5` |
| `gamma` `:521` | `:521` |
| `evaluate_original_residual` `:1383` | `:1383` (body `evaluate_original_residual_bound` `:1396`) |
| `screen_pivot` `:1494` | `:1494` |
| `estimate_rcond` `:1521` | `:1521` (body `ConditionMatrix::estimate_rcond` `:1537`) |
| `finish_checked_factor` `:1635` | `:1635` |
| `finish_structural` `:1883` | `:1883` |
| `factor_structural_profile` `:2027` | `:2027` (the skyline loop `ProfileFactor::factor_structural_profile` `:1962`) |
| `verify_negative_direction` `:2123` | `:2123` |
| `negative_pair_witness` `:2166` | `:2166` |
| `sparse.rs` `SparsePattern` `:48`, `from_positions` `:58`, `from_connectivity` `:82` | same; re-exported at `structural.rs:12` |
| `exact_sum.rs` magnitudes `:45-48`, `compare` `:103`, `subtract` `:113` | same (`LIMBS` `:19`, `QUANTUM_EXPONENT` `:21`) |
| `load_ledger.rs` `ForceTerm::accumulate` `:39`, `accumulate_dof` `:312` | same |
| `rigid_body.rs` `assess_rigid_body` `:33` | same |
| `sparse_direct/src/lib.rs:505` `reverse_cuthill_mckee` | same (`pseudo_peripheral_start` `:558`; tests `:934-990`; `order_sparse_structural` `structural.rs:56`) |
| `FK/lib.rs` frame algorithm (D1: `:511-525` at `c61a540ea`) | `FrameOrientation::from_x_axis_and_y_reference` `:525-539`; `FrameElement::local_x_axis` `:590-595`, `length` `:584-588`; `normalize` `:1864`, `dot` `:1885`, `cross` `:1897`; `local_stiffness` `:711` |
| K-D5 re-formation | `formation_check.rs`: `chord_axes` `:502`, `rotate` `:525`, `frame_matrix` `:580` — **all private to `formation_check`** (see §12.E) |
| M03 intended-action audit (denominator precedent) | `audit_intended_action` `:886` |
| M03 radix preparation (scale exponents) | `prepare_bound` `:1234` (`-binary_exponent(d).div_euclid(2)` `:1258`) |

---

## 1. Files and module layout

New files (all under `FK/src/structural/retained/`), declared `pub(crate) mod …;` in `retained/mod.rs` with a documentation paragraph:

| File | Content |
|---|---|
| `wide_sum.rs` | **the correctly rounded exact multi-term sum** (Q2(a); the name chosen at checkpoint 0) |
| `source.rs` | `PrimitiveSource` and its parts, validation, canonical encoding |
| `ledger.rs` | the load ledger at p (exact per-DOF nets, projection, encoding) |
| `assemble.rs` | frame, B, D, element matrices, pattern assembly, spring blocks, reduced rhs |
| `factor.rs` | geometry-first screen, RCM (Q7), radix equilibration, profile LDLᵀ at p, pivot screen, negative-energy pair check, Hager–Higham, triangular solves, residual at p + 64 and refinement |
| `recover.rs` | d_local, e, Q, end and station actions, spring actions, reactions, magnitudes; publication (`Binary64Outcome`, +0.0 rule); retained-state encoding |
| `combine.rs` | `RetainedCombination` (exact combination, own schedule) |
| `adaptive.rs` | the schedule, stop rule, S\* (2p and published), classification, evidence, budgets, factor reuse, the width dispatch |

Plus `FK/src/exact_sum.rs` (one additive accessor, Q3), `FK/tests/s11_site_table.rs` (additive rows, Q8), `FK/tests/retained_k4/**` (tests, generator, vectors, `SHA256SUMS`), and the records under `T3/IMPLEMENTATION/K4/`. **Nothing else.** Each K4 file ends with `#[cfg(test)] #[path = "../../../../tests/retained_k4/<file>_tests.rs"] mod tests;` (K3's pattern). No `tests/retained_k4/main.rs`.

Imports: `super::wide::{Wide, WideError, EXPONENT_LIMIT}`, `super::wide::multi::{WideContext, SupportedWidth, Binary64Outcome, AttemptWork, WidthWork, OpKind, limb_multiply_cost}`, `crate::exact_sum::ExactAccumulator`, `crate::rigid_body::{assess_rigid_body, ObjectiveFamily, RigidBodyStatus}`, `crate::structural::SparsePattern`, `crate::DOF_PER_NODE`. Generic code bounds on `where Wide<L>: SupportedWidth`.

---

## 2. Widths per precision, and how values cross widths

K3 ruling 8: 128, 192, 256 at L = 4; 320, 512 at L = 8; 576, 1024 at L = 16.

| Role | Precisions | Width |
|---|---|---|
| Candidate or verification solve (formation, assembly, factor, solves, recovery) | 128 / 256 / 512 / 1024 | 4 / 4 / 8 / 16 |
| Residual re-formation (p + 64) | 192 (p=128) / 320 (256) / 576 (512) | 4 / 8 / 16 |
| Ceiling's residual (Q4: no p + 64) | 1024 | 16 |
| Stop-rule comparison q_p vs q_2p | at W(2p) | 128→256 same width; 256→512 `widen::<8>`; 512→1024 `widen::<16>` |
| Ledger projection | target p | `from_integer` at the target context |
| Test-only | p = 53 at L = 4 (product-element check); K-D5 cross-check in `Wide<2>` at 128 | |

- The dispatch is const-generic: `run_solve::<L, R>(p, …)` with (L, R) ∈ {(4,4), (4,8), (8,16), (16,16)}; `widen::<R>()`'s compile-time R ≥ L holds for all four.
- **Values only ever widen** (exact). K4 never narrows a rounded value: every quantity needed at a lower precision is rounded **once, directly from its exact sum** into that context (for example the correction right-hand side is the exact residual sum rounded once to p, not the p + 64 residual narrowed). So there is no double rounding inside the method. `WideContext::round` is not used by the method.
- Publication: `to_binary64` of the p-bit value (one rounding), with the +0.0 rule (§4.4).

---

## 3. The correctly rounded exact multi-term sum (`wide_sum.rs`; Q2(a))

### 3.1 Algorithm (ROOT's recommended one)
A signed wide-integer accumulator in the manner of `ExactAccumulator`: separate **positive and negative magnitudes** in fixed stack buffers, a floating **anchor** (the exponent of bit 0), netted once at the end with one compare and one subtract, and rounded **once** by K3's `WideContext::from_integer(negative, &magnitude[..used], anchor)`, which is correctly rounded (nearest, ties to even, decided by every bit of the magnitude) for any length and gives +0 for a zero magnitude.

- **Terms** (each added exactly, never rounded):
  - `add_wide::<M>(&x, negate)`: any `Wide<M>` value (L = 4, 8, 16), read through `parts()`;
  - `add_binary64(x, negate)`: exact lift of a finite binary64 (NaN/∞ refused);
  - `add_product::<M>(ctx, &a, &b, negate)`: the exact product of two values of at most `ctx.precision()` bits, through K3's `two_product` (s + e = a·b exactly), both parts added; the context charges `TwoProduct`;
  - `add_integer(negative, magnitude, exponent)`: an exact ±integer·2^exponent (the ledger's netted value from the accessor, §5.2);
  - `add_scaled(&other, negate, factor: u64, pow2: i64)`: another accumulator's exact value times a small integer and a power of two (for the exact gate and screen decisions, §8.3–§8.5).
- **Placement:** each term is trimmed to its lowest and highest set bits. If its lowest bit is below the anchor, both magnitudes are shifted up (re-anchored) when the new span still fits; the carry headroom is kept.
- **Span limit (refuses, never truncates):** `SUM_LIMBS = 128` limbs per magnitude (8,192 bits, 2 KiB each), `CARRY_BITS = 64`, so a sum may span at most **8,128 bits** (highest possible bit − lowest set bit + 1). A term that would exceed it gives `SumRefusal::Span`. An anchor outside `i64`, or a `from_integer` result outside ±2^62, gives `SumRefusal::Exponent`. Neither ever drops a bit.
  - **Justification against the binary64 input range.** Every nonzero binary64 value and every exact product of two binary64 values has its leading bit in [2^-2148, 2^2048) (the `ExactAccumulator`'s range, which adds 64 carry bits). A K4 term has at most 2,048 significant bits (a product of two 1024-bit values). So any sum whose terms' leading bits lie in that range fits: 4,196 + 2,048 = 6,244 < 8,128 bits. Sums outside it arise only from extreme primitives (for example E·I/L³ far outside the binary64 range); they are refused.
  - **[POSITION] A span refusal is terminal, not escalated:** a term's span can only grow with p (terms keep their leading bit and gain low bits), so escalation cannot cure it. The case is unresolved with reason `exact_sum_span` and its attempts. The largest span used is recorded per attempt (§10), so ROOT sees the headroom.
- **No heap allocation** at all (per term or per sum): two `[u64; 128]` stack arrays; `clear()` zeroes only the used prefix so one accumulator is reused per row.
- **Work** (`SumWork`, deterministic, charged to the attempt, §10): limbs touched per added term (+1 carry), limbs shifted on re-anchoring, 2·used for the netting, and the magnitude length passed to `from_integer` (K3's N3; the context itself charges 2L as `Round`). `two_product` is charged by its context.
- **An exact zero is +0** (`from_integer`'s rule; D1 §4.1.2).

### 3.2 Signatures
```rust
pub(crate) const SUM_LIMBS: usize = 128;
pub(crate) const SPAN_LIMIT_BITS: i64 = 8128;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SumRefusal { Span, Exponent, NonFinite, Wide(WideError) }
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) struct SumWork { pub(crate) term_limbs: u64, pub(crate) shift_limbs: u64,
    pub(crate) net_limbs: u64, pub(crate) rounded_limbs: u64, pub(crate) max_span_bits: u64 }
impl SumWork { pub(crate) fn limb_multiply_equivalents(&self) -> u64; pub(crate) fn merge(&mut self, o: &Self); }
pub(crate) struct ExactWideSum { /* positive, negative: [u64; SUM_LIMBS], anchor: i128, used: usize, empty: bool, work: SumWork */ }
impl ExactWideSum {
    pub(crate) fn new() -> Self;
    pub(crate) fn clear(&mut self);
    pub(crate) fn add_wide<const M: usize>(&mut self, x: &Wide<M>, negate: bool) -> Result<(), SumRefusal> where Wide<M>: SupportedWidth;
    pub(crate) fn add_binary64(&mut self, x: f64, negate: bool) -> Result<(), SumRefusal>;
    pub(crate) fn add_product<const M: usize>(&mut self, ctx: &mut WideContext<M>, a: &Wide<M>, b: &Wide<M>, negate: bool) -> Result<(), SumRefusal> where Wide<M>: SupportedWidth;
    pub(crate) fn add_integer(&mut self, negative: bool, magnitude: &[u64], exponent: i64) -> Result<(), SumRefusal>;
    pub(crate) fn add_scaled(&mut self, other: &Self, negate: bool, factor: u64, pow2: i64) -> Result<(), SumRefusal>;
    pub(crate) fn signum(&self) -> i8;                      // exact
    pub(crate) fn is_zero(&self) -> bool;
    pub(crate) fn round<const L: usize>(&mut self, ctx: &mut WideContext<L>) -> Result<Wide<L>, SumRefusal> where Wide<L>: SupportedWidth; // once
    pub(crate) fn work(&self) -> SumWork;
}
```
### 3.3 Correctness argument (to be written out in RETURN)
(i) Each term is an exact dyadic integer placed exactly; (ii) the two magnitudes hold the exact sums of the positive and negative terms (unsigned adds with carry, headroom 2^64 terms); (iii) the netting is one exact subtraction of the smaller from the larger; (iv) `from_integer` rounds that exact integer once (K3, independently reviewed, tested on 68-limb magnitudes with far sticky bits and ties); (v) the only non-exact exits are explicit refusals. A tie decided by a far tail is decided correctly because no bit is ever discarded before the rounding.

---

## 4. Signatures of the other K4 files

### 4.1 `source.rs`
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum Component { Ux, Uy, Uz, Rx, Ry, Rz }
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct Dof { pub(crate) node: u32, pub(crate) component: Component }   // global index 6·node + component
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct StraightMember { pub(crate) id: u32, pub(crate) node_i: u32, pub(crate) node_j: u32,
    pub(crate) elastic_modulus: f64, pub(crate) shear_modulus: f64, pub(crate) area: f64,
    pub(crate) second_moment_y: f64, pub(crate) second_moment_z: f64, pub(crate) torsion_constant: f64,
    pub(crate) y_reference: [f64; 3] }
#[derive(Debug, Clone, Copy, PartialEq)] pub(crate) struct Spring { pub(crate) id: u32, pub(crate) dof: Dof, pub(crate) stiffness: f64 }
#[derive(Debug, Clone, Copy, PartialEq, Eq)] pub(crate) enum SpringKind { Translation, Rotation }
#[derive(Debug, Clone, Copy, PartialEq)] pub(crate) struct DirectionalSpring { pub(crate) id: u32, pub(crate) node: u32,
    pub(crate) kind: SpringKind, pub(crate) direction: [f64; 3], pub(crate) stiffness: f64 }      // Q6, kernel only
#[derive(Debug, Clone, Copy, PartialEq)] pub(crate) struct Constraint { pub(crate) dof: Dof, pub(crate) value: f64 } // 0: rigid; else prescribed
#[derive(Debug, Clone, PartialEq)] pub(crate) struct NodalLoad { pub(crate) dof: Dof, pub(crate) value: f64, pub(crate) source_id: String }
#[derive(Debug, Clone, Copy, PartialEq)] pub(crate) struct Station { pub(crate) id: u32, pub(crate) member: u32, pub(crate) fraction: f64 }
#[derive(Debug, Clone, PartialEq)] pub(crate) struct SupportGroup { pub(crate) id: u32, pub(crate) node: u32,
    pub(crate) restrained: [bool; 6], pub(crate) springs: Vec<u32>, pub(crate) directional_springs: Vec<u32> } // [OPEN] O2
#[derive(Debug, Clone, PartialEq)] pub(crate) enum SourceError { /* NoNodes, NonFiniteCoordinate{node}, NodeOutOfRange{…},
    DuplicateId{…}, NonPositiveProperty{member, property}, NonFiniteProperty{…}, SubnormalDerivedPrimitive{member, property},
    ZeroLength{member}, DegenerateAxis{member}, NonPositiveSpring{id}, ZeroDirection{id}, DuplicateConstraint{dof},
    NonFiniteValue{…}, StationOutOfRange{id}, UnknownMember{…}, SupportMismatch{…} */ }
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct PrimitiveSource { /* private: nodes: Vec<[f64; 3]>, members, springs, directional_springs,
    constraints, loads, stations, supports (each list sorted canonically at construction) */ }
impl PrimitiveSource {
    pub(crate) fn new(nodes: Vec<[f64; 3]>, members: Vec<StraightMember>, springs: Vec<Spring>,
        directional_springs: Vec<DirectionalSpring>, constraints: Vec<Constraint>, loads: Vec<NodalLoad>,
        stations: Vec<Station>, supports: Vec<SupportGroup>) -> Result<Self, SourceError>;
    pub(crate) fn encoding(&self) -> Vec<u8>;            // canonical (Q10, §9)
    pub(crate) fn stiffness_encoding(&self) -> Vec<u8>;  // the factor-reuse identity (§8.8)
    // read accessors for nodes(), members(), … (no &mut; immutable after `new`)
}
```
- **Validation** (D1 §4.1.1): finite coordinates; node indices in range; i ≠ j; E, G, A, Iy, Iz, J finite and > 0; **A, Iy, Iz, J not subnormal** (the derived primitives); length exactly nonzero (coincident nodes refused); **y_reference not parallel to the chord, decided exactly** (d × y_ref = 0 with exact sums; no tolerance, D1 "No axis tolerance is used at p"); springs k finite > 0; directional direction finite and exactly nonzero; each DOF constrained at most once; values finite; unique ids per list; stations 0 ≤ t ≤ 1 on an existing member; support groups consistent. The DOF partition is complete by construction (a DOF is free unless constrained), so "incomplete partition" cannot be constructed; duplicates are refused.
- **[REFINE] Ids.** Members, springs, directional springs, stations and support groups carry a `u32` id, and every list is sorted canonically at construction. Published per-entity quantities are keyed by id, not list position. This is what makes the brief's permutation test (E, L) hold for the retained-state encoding, and it gives F2a its entity mapping. Nodes stay indexed (the DOF numbering); relabelling nodes is not a permutation of E.
- **Type-level exclusion:** no other family exists in the type. K4 adds no refusal reason for nonlinear supports (brief §"What W1 is for").

### 4.2 `ledger.rs`
```rust
pub(crate) struct RetainedLedger { /* per loaded DOF, ascending: (Dof, netted exact value: sign, [u64; 68], -2148) */ }
impl RetainedLedger {
    pub(crate) fn from_source(source: &PrimitiveSource) -> Result<Self, LedgerRefusal>; // one ExactAccumulator per DOF, one `add` per NodalLoad (S11 granularity)
    pub(crate) fn combined(operands: &[(f64, &RetainedLedger)]) -> ...;                 // see §7: built from the operands' terms with add_product(c, v)
    pub(crate) fn add_to(&self, dof: Dof, sum: &mut ExactWideSum, negate: bool) -> Result<(), SumRefusal>; // enters another sum EXACTLY
    pub(crate) fn project<const L: usize>(&self, dof: Dof, ctx: &mut WideContext<L>) -> Result<Wide<L>, ...>; // rounded once
    pub(crate) fn encoding(&self) -> Vec<u8>;
}
```
- The projection is the accessor + `from_integer` (stale-design ruling 3). **Wherever the ledger joins another sum (rhs, residual, reaction, combination) it enters through `add_to` exactly**, never as its p-rounded projection (K4-M8's kill). The projection itself is used only where the design names "the load at p" standing alone (none in the solve path; it serves the tests and evidence).
- K4's ledger uses FK's `ExactAccumulator` directly (the same accumulator and granularity as S11's `LoadLedger`; W1a's nodal loads are all `ForceTermKind::Term` values — the only `Product` producer in PP is the curved thermal equivalent, `PP:9316`, which is not W1a).

### 4.3 `assemble.rs`
```rust
pub(crate) struct MemberFrame<const L: usize> { /* e_x, e_y, e_z, length, inv_length */ }
pub(crate) struct MemberMatrices<const L: usize> { /* B (6×12, global), D (EA/L, GJ/L, EIz/L, EIy/L), K_e upper triangle (78) */ }
pub(crate) struct Assembled<const L: usize> { /* pattern: SparsePattern (full, all DOFs), values: Vec<Wide<L>> per entry, precision */ }
pub(crate) fn form_members<const L: usize>(ctx: &mut WideContext<L>, source: &PrimitiveSource, sum: &mut ExactWideSum, meter: &mut Meter) -> Result<Vec<MemberMatrices<L>>, KernelStop>;
pub(crate) fn assemble<const L: usize>(ctx: &mut WideContext<L>, source: &PrimitiveSource, members: &[MemberMatrices<L>], pattern: &SparsePattern, sum: &mut ExactWideSum, meter: &mut Meter) -> Result<Assembled<L>, KernelStop>;
pub(crate) fn source_pattern(source: &PrimitiveSource) -> Result<SparsePattern, KernelStop>; // from_positions: member 12×12 blocks, spring diagonals, directional 3×3 blocks
pub(crate) fn reduced_rhs<const L: usize>(ctx: &mut WideContext<L>, k: &Assembled<L>, ledger: &RetainedLedger, prescribed: &[(usize, f64)], free: &[usize], sum: &mut ExactWideSum) -> Result<Vec<Wide<L>>, KernelStop>;
```
Formation at precision q (every operation rounded to q; every multi-term sum one exact expansion rounded once):
1. d_k = x_j,k − x_i,k (exact 2-term sum, rounded once); L = √(fl(Σ d_k²)) (exact dot, rounded once, then √); e_x = d/L.
2. proj = fl(Σ y_ref,k·e_x,k) (exact); y_c,k = fl(y_ref,k − proj·e_x,k) (exact); e_y = y_c/√(fl(Σ y_c²)); z_c,k = fl(e_x × e_y) per component (exact 2-product difference); e_z = z_c/√(fl(Σ z_c²)). This is the product's algorithm (`FK/lib.rs:525-539`) with every dot product exact. **[REFINE]** D1 says "every operation below is rounded to p"; forming the dot products exactly is strictly more accurate and keeps each step a single rounding. No axis tolerance.
3. invL = 1/L. D: EA/L = fl(fl(E·A)/L) (E·A is exact at q ≥ 106), GJ/L likewise, c_z = fl(fl(E·Iz)/L), c_y likewise; **4c and 2c are exact power-of-two scalings** (`mul_pow2`), so D's [[4,2],[2,4]] blocks carry no extra rounding.
4. B = B_local·T (global), each entry one rounded product or an axis component: row 0 (extension) ∓e_x on the translations; row 1 (twist) ∓e_x on the rotations; rows 2–3 (θz relative to the chord) ±fl(invL·e_y) on the translations, e_z on θ_i / θ_j; rows 4–5 (θy) ∓fl(invL·e_z) on the translations, e_y on θ_i / θ_j (the probe's `basic_B_local`, D1 §4.1.2 item 2).
5. DB_rs = one exact expansion of ≤ 2 products, rounded once; K_e[a][b] (a ≤ b) = one exact expansion of ≤ 6 products B[r][a]·DB[r][b], rounded once; K_e[b][a] := K_e[a][b] (symmetric by construction).
6. **Assembly** (D1 §4.1.2 item 4): each pattern entry = one exact expansion of its members' K_e contributions, the global-axis spring stiffnesses (binary64) and the directional spring entries, rounded once. Element matrices are stored per member (upper triangle, 78 values; at L = 16 about 11 MB per 1,000 members), and each entry gathers its contributions through a precomputed incidence list, so one accumulator is reused (not one per entry).
7. **Directional spring (Q6):** s = fl(nᵀn) (exact for binary64 n at q ≥ 108), m_ab = n_a·n_b (exact), k_ab = fl(fl(k·m_ab)/s). Formed from the binary64 direction, never from a binary64-normalized one (K4-M23).
8. **Reduced rhs** (D1 §4.1.2 item 6): rhs_i = one exact expansion of the ledger's exact net at i (`add_to`) and the exact products −K_ic·u_c (u_c binary64, lifted), rounded once to p. Neither K nor rhs is rounded to binary64.
- Pattern: `SparsePattern::from_positions` (member 12×12 blocks, spring diagonals, directional-spring 3×3 node blocks). For a source with no directional spring on an element-free node this equals `from_connectivity`'s pattern (a test asserts it).

### 4.4 `factor.rs`
```rust
pub(crate) enum GeometryVerdict { Proceed, Refused(Refusal) }
pub(crate) fn geometry_first(source: &PrimitiveSource) -> Result<GeometryVerdict, KernelStop>;
pub(crate) fn reverse_cuthill_mckee(adjacency: &[Vec<usize>]) -> Vec<usize>;         // Q7 port
pub(crate) struct Ordering { /* order, first (skyline), profile_entries */ }
pub(crate) fn order_free(pattern: &SparsePattern, free: &[usize]) -> Ordering;
pub(crate) struct RetainedFactor<const L: usize> { /* order, first, rows: Vec<Vec<Wide<L>>>, scale_exponents: Vec<i64>, pivots evidence */ }
pub(crate) enum FactorFailure { Pivot { global_dof: usize }, ZeroDiagonal { global_dof: usize }, NegativeEnergy { i: usize, j: usize }, Condition, Sum(SumRefusal), Wide(WideError), Budget(BudgetScope) }
pub(crate) fn factor<const L: usize>(ctx: &mut WideContext<L>, k: &Assembled<L>, free: &[usize], ordering: &Ordering, sum: &mut ExactWideSum, meter: &mut Meter) -> Result<RetainedFactor<L>, FactorFailure>;
impl<const L: usize> RetainedFactor<L> {
    pub(crate) fn solve(&self, ctx: &mut WideContext<L>, rhs: &[Wide<L>]) -> Result<Vec<Wide<L>>, WideError>;   // unscaled in, unscaled out
    pub(crate) fn condition(&self, ctx: &mut WideContext<L>, k: &Assembled<L>, free: &[usize], sum: &mut ExactWideSum) -> Result<Wide<L>, …>; // Hager–Higham, rcond
    pub(crate) fn pivot_margin_min(&self) -> Wide<L>;
}
pub(crate) struct Refined<const L: usize> { /* u_free, corrections: u8, worst_gate_ratio (exact, as sum parts), residual_basis: u32 */ }
pub(crate) fn solve_and_refine<const L: usize, const R: usize>(ctx: &mut WideContext<L>, ctx_r: &mut WideContext<R>, factor: &RetainedFactor<L>, k_r: &Assembled<R>, ledger: &RetainedLedger, prescribed: &[(usize, f64)], free: &[usize], rhs: &[Wide<L>], sum: &mut ExactWideSum, meter: &mut Meter) -> Result<Refined<L>, SolveFailure>;
```
Details in §8.

### 4.5 `recover.rs`
```rust
pub(crate) enum Kind { Translation, Rotation, Force, Moment }
pub(crate) enum End { I, J }
pub(crate) enum QuantityId { Displacement(Dof), DisplacementMagnitude(u32 /*node*/),
    EndAction { member: u32, end: End, component: Component },           // local, node-on-element
    StationAction { station: u32, component: Component },                // local, "j-end action of the sub-member i..x"
    SpringAction { spring: u32, component: Component },                  // global component(s)
    DirectionalSpringAction { spring: u32, component: Component },       // three global components
    Reaction(Dof), SupportForceMagnitude(u32), SupportMomentMagnitude(u32) }
pub(crate) struct RetainedState<const L: usize> { /* precision, u (all DOFs), per member Q[6], quantities: Vec<(QuantityId, Wide<L>)> */ }
pub(crate) fn recover<const L: usize>(ctx: &mut WideContext<L>, source: &PrimitiveSource, members: &[MemberMatrices<L>], k: &Assembled<L>, ledger: &RetainedLedger, u: &[Wide<L>], sum: &mut ExactWideSum, meter: &mut Meter) -> Result<RetainedState<L>, KernelStop>;
pub(crate) fn publish_value<const L: usize>(q: &Wide<L>) -> Binary64Outcome;   // exact zero → Normal(+0.0); else `to_binary64`
pub(crate) fn state_encoding<const L: usize>(state: &RetainedState<L>) -> Vec<u8>;
```
### 4.6 `combine.rs`
```rust
pub(crate) struct RetainedCombination { /* operands (factor, case index), selected p, states at p and 2p, evidence */ }
pub(crate) enum CombinationOutcome { Selected(RetainedCombination), Unresolved { reason: CombinationReason, attempts: Vec<AttemptRecord> } }
pub(crate) enum CombinationReason { CombinationUnresolved /* ceiling */, OperandUnresolved, OperandsDifferInStiffness, Budget(BudgetScope), ExactSumSpan }
impl RetainedCombination {
    pub(crate) fn form<const L: usize>(operands: &[(f64, &RetainedSolve)], p: u32, …) -> Result<RetainedState<L>, …>; // D1's `form(&[(factor, &RetainedSolve)], p)`
    pub(crate) fn solve(operands: &[(f64, &RetainedSolve)], case_limit: CaseLimit, meter: &mut InvocationMeter) -> CombinationOutcome; // the schedule
    pub(crate) fn publish(&self) -> Publication;
    pub(crate) fn evidence(&self) -> &RetainedEvidence;
}
```
### 4.7 `adaptive.rs` (outcome and evidence types)
```rust
pub(crate) const METHOD_TOKEN: &str = "contribution_preserving_multiprecision_v1"; // placeholder (D1 §4.1)
pub(crate) const POLICY: &str = "M03-INTEGRITY-MP-v1";                            // placeholder
pub(crate) const CANDIDATES: [u32; 3] = [128, 256, 512]; pub(crate) const CEILING: u32 = 1024;
pub(crate) const FLOOR_RATIO_BITS: u64 = 0x3DD0_0000_0000_0000;                   // R = 2^-34
#[derive(Debug, Clone, Copy, PartialEq, Eq)] pub(crate) struct CaseLimit(u64);   // required; no Default (Q5)
impl CaseLimit { pub(crate) fn new(limb_multiply_equivalents: u64) -> Self; }
pub(crate) struct InvocationMeter { /* limit, charged */ }                         // required; no Default
impl InvocationMeter { pub(crate) fn new(limit: u64) -> Self; pub(crate) fn charged(&self) -> u64; pub(crate) fn exhausted(&self) -> bool; }
pub(crate) enum BudgetScope { Case, Invocation }
pub(crate) enum AttemptRole { Candidate, Verification, VerificationThenCandidate }
pub(crate) enum AttemptReason { PivotScreen { global_dof: usize }, ConditionEstimate, ResidualGate { global_dof: usize },
    StopRule { body: u32, kind: Kind, quantity: QuantityId }, VerificationFailed, Budget(BudgetScope), ExactSumSpan, ExponentRange }
pub(crate) enum AttemptOutcome { Accepted, Verified, Rejected(AttemptReason), Failed(AttemptReason) }
pub(crate) struct StageWork { /* per stage: formation, assembly, rhs, factor, condition, solve, residual_formation, refinement, recovery, stop_rule; each u64 */ }
pub(crate) struct AttemptRecord { pub(crate) precision: u32, pub(crate) role: AttemptRole, pub(crate) outcome: AttemptOutcome,
    pub(crate) residual_basis: u32, pub(crate) corrections: u8, pub(crate) pivot_margin_min: Option<f64> /* fl↓ */,
    pub(crate) rcond: Option<f64> /* nearest; model information */, pub(crate) residual_worst: Option<f64> /* fl↑ */,
    pub(crate) work: AttemptWork /* K3, one record per context */, pub(crate) k4_work: SumWork, pub(crate) stages: StageWork,
    pub(crate) shared_work: Option<u64> /* §8.8 */, pub(crate) storage: StorageCounts }
pub(crate) struct StorageCounts { pub(crate) pattern_entries: usize, pub(crate) profile_entries: usize, pub(crate) limbs_per_entry: usize }
pub(crate) enum Refusal { InvalidSource(SourceError), MechanismWitnessed { body: u32, rigid_parameters: [f64; 6] },
    DirectionalSpringsDoNotSpan { node: u32, kind: SpringKind }, NegativeEnergy { i: usize, j: usize }, GeometryUnavailable(StructuralError) }
pub(crate) enum UnresolvedReason { Ceiling, Budget(BudgetScope), ExactSumSpan, ExponentRange }
pub(crate) enum CaseOutcome { Selected(RetainedSolve), Refused(Refusal), Unresolved { reason: UnresolvedReason, attempts: Vec<AttemptRecord> } }
pub(crate) struct RetainedSolve { /* source (owned), selected p, verification p, states at every solved p, evidence */ }
impl RetainedSolve { pub(crate) fn publish(&self) -> Publication; pub(crate) fn evidence(&self) -> &RetainedEvidence;
    pub(crate) fn source(&self) -> &PrimitiveSource; pub(crate) fn selected_precision(&self) -> u32; }
pub(crate) enum RowClass { RelativeVerified, AbsoluteVerified { bound_bits: u64 }, InputDerived, Unpublishable }
pub(crate) struct PublishedRow { pub(crate) id: QuantityId, pub(crate) kind: Kind, pub(crate) body: u32, pub(crate) value: Binary64Outcome, pub(crate) class: RowClass }
pub(crate) struct Publication { pub(crate) rows: Vec<PublishedRow> }
pub(crate) struct StopRuleSummary { pub(crate) worst: Vec<(u32 /*body*/, Kind, f64 /*fl↑*/)> }
pub(crate) struct RetainedEvidence { pub(crate) method: &'static str, pub(crate) policy: &'static str,
    pub(crate) attempts: Vec<AttemptRecord>, pub(crate) selected_precision: u32, pub(crate) verification_precision: u32,
    pub(crate) stop_rule: StopRuleSummary, pub(crate) floor_ratio_bits: u64, pub(crate) body_scales: Vec<(u32, Kind, u64)>,
    pub(crate) input_derived_dofs: Vec<Dof>, pub(crate) absolute_verified: Vec<(QuantityId, u64)>, pub(crate) not_covered: Vec<QuantityId>,
    pub(crate) unpublishable: Vec<(QuantityId, Binary64Outcome)>, pub(crate) pivot_margin_min: f64, pub(crate) rcond: f64,
    pub(crate) rcond_label: &'static str /* "sensitivity to matrix-entry perturbation, not to authored parameters" */,
    pub(crate) residual: ResidualSummary, pub(crate) source_encoding: Vec<u8>, pub(crate) ledger_encoding: Vec<u8>,
    pub(crate) retained_state_encoding: Vec<u8>, pub(crate) shared_group: Option<u32> }
pub(crate) fn solve_cases(sources: &[PrimitiveSource], case_limit: CaseLimit, meter: &mut InvocationMeter) -> Vec<CaseOutcome>;
pub(crate) fn solve_case(source: PrimitiveSource, case_limit: CaseLimit, meter: &mut InvocationMeter) -> CaseOutcome; // a group of one
// §4.1.6.1 functions for F2a's product rows (pure binary64, pinned):
pub(crate) struct ScaledRow { pub(crate) body: u32, pub(crate) kind: ScaleKind, pub(crate) value: f64 }
pub(crate) enum ScaleKind { Translation, Rotation, Force, Moment, Stress { member: u32, k_bits: u64 } }
pub(crate) fn body_extent(coords: &[[f64; 3]]) -> f64;                           // item 5
pub(crate) fn body_scales(rows: &[ScaledRow], extents: &[(u32, f64)]) -> Vec<(u32, [f64; 4])>; // items 4 and 6
pub(crate) fn stress_scale(fo: f64, mo: f64, area: f64, modulus: f64, k: f64) -> f64;         // item 7
pub(crate) fn intensified_k(i: f64) -> f64;                                      // fl↑(k√2·i)
pub(crate) fn threshold(s_star: f64) -> f64; pub(crate) fn absolute_bound(s_star: f64) -> f64; // fl(R·S*), fl↑(2^-64·S*)
pub(crate) fn classify(value: f64, s_star: f64) -> RowClass;
```
(Field lists are the checkpoint-0 target; minor shape changes at A will be reported.)

---

## 5. The `exact_sum.rs` accessor (Q3) and the ledger

### 5.1 The accessor (one additive function)
```rust
impl ExactAccumulator {
    /// K4 (T3 D1 §4.1.2 item 5; ROOT's K4 ruling Q3): the exact value netted once with
    /// `compare` and `subtract`, as (negative, magnitude, exponent of the magnitude's unit bit).
    /// The exponent is always -2148; an exact zero is (false, 0).
    pub(crate) fn net_parts(&self) -> (bool, [u64; LIMBS], i64) {
        match compare(&self.positive, &self.negative) {
            std::cmp::Ordering::Less => (true, subtract(&self.negative, &self.positive), QUANTUM_EXPONENT),
            _ => (false, subtract(&self.positive, &self.negative), QUANTUM_EXPONENT),
        }
    }
}
```
It changes nothing else in the file: no accumulation shape (the site table is unchanged for `exact_sum.rs`), `exact_sum`'s tests untouched. Its caller is `ledger.rs` (live through K4's allowed entry points, so no allowance of its own).

### 5.2 Ledger construction and use
Per loaded DOF (ascending), one `ExactAccumulator`, one `add(value)` per `NodalLoad` (any order; exact). The netted value is kept (68 limbs per loaded DOF). `add_to` passes it to an `ExactWideSum` through `add_integer` (trimmed). The combination ledger (§7) uses `add_product(c_i, v)` of the operands' own load terms, so it is exact too.

---

## 6. The published-quantity set, and where stations come from

Per case (all computed at p from the retained state, each **one exact expansion rounded once to p**, then rounded once to binary64 at publication):

| Quantity | Formation at p | Kind (for S\*) |
|---|---|---|
| u at every DOF (free: solved; constrained: the prescription lifted exactly) | solve / lift | translation or rotation; **constrained DOFs are `input_derived`** (rule 2a) |
| Displacement magnitude per node | fl(√(fl(u_x²+u_y²+u_z²))) (exact dot) | translation |
| d_local = T u (12 per member) | 3 products | (internal; not published) |
| e = B_local d_local (6) | ≤ 3 terms | (internal) |
| Q = D e (6 per member: N, T, M_zi, M_zj, M_yi, M_yj) | ≤ 2 products | (internal; in the retained-state digest) |
| End actions, local, node-on-element (12 per member) = B_localᵀQ: i: (−Q0, invL·Q2 + invL·Q3, −(invL·Q4 + invL·Q5), −Q1, Q4, Q2); j: (Q0, −(invL·Q2 + invL·Q3), invL·Q4 + invL·Q5, Q1, Q5, Q3), each shear one exact expansion of its two products, rounded once | ≤ 2 products | force (u, v, w), moment (θ) |
| Station actions at fraction t (6 per station): N = Q0, V_y = −(Q2+Q3)/L, V_z = (Q4+Q5)/L, T = Q1, M_y(t) = fl(t·Q5 + t·Q4 − Q4), M_z(t) = fl(t·Q3 + t·Q2 − Q2) | ≤ 3 products | force, moment |
| Global-axis spring action −k·u | 1 product | force or moment |
| Directional spring action, 3 global components −Σ_b k_ab u_b | 3 products | force or moment |
| Reaction at each constrained DOF: Σ_j K_cj u_j − f_c (ledger exact) | nnz products + ledger | force or moment |
| Support group force and moment magnitudes (see O2) | components by exact sums of the group's reactions and spring actions; then fl(√(fl(Σ c²))) | force, moment |

- **Stations come from the source** as `Station { id, member, fraction t }`, t a binary64 in [0, 1] **[POSITION, as the brief recommends]**. The station action is the "j-end action of the sub-member i→x": at t = 1 it equals the j-end action and at t = 0 minus the i-end action, and with nodal loads only the moments are linear (W1b adds the span-load term). The R1 adapter adds t = 0.5 for every member (R1's `Mb.mid`).
- Derived stress rows are not K4's (F2a, at the facade, from the once-rounded actions).
- Convention checks (tests): R1's N = Q0 (tension positive), T = Q1 with tw = (θ_j − θ_i)·e_ij, Mb.i = |(Q4, Q2)|, Mb.j = |(Q5, Q3)|; the test forms Mb as `sqrt(My*My + Mz*Mz)` in binary64, written out, never `hypot`.
- **[POSITION] The +0.0 rule:** `publish_value` returns `Normal(+0.0)` for any exact zero (a triangular solve can yield −0 in `Wide`; D1 §4.1.2 and Q11 give +0.0). Otherwise K3's `to_binary64` with its outcome. Underflow and overflow carry no value and are listed (`unpublishable`), never silent.

---

## 7. The combination (`combine.rs`)

- **Operands must share the stiffness identity** (members, springs, directional springs, the constrained-DOF set, nodes): recovery from the combined state uses one set of element operators. Otherwise `OperandsDifferInStiffness` **[POSITION; W1a]**. An unresolved or refused operand gives `OperandUnresolved`.
- **Forming at p** (D1 §4.1.1): u_c,j = one exact expansion of TwoProduct(lift(c_i), u_i,j) over the operands, rounded once to p (prescribed DOFs: the exact Σ c_i·v_i,j of the binary64 prescriptions, rounded once); **f_c is a combined exact ledger**: per DOF, `add_product(c_i, v)` over every load term of every operand (exact), so the ledger enters the reactions exactly (never Σ c_i·(p-rounded f_i)). Actions and reactions are recovered from the combined state at p with the shared member matrices at p.
- **[POSITION] The precision rule.** The combination runs its own schedule over candidates p ∈ {p_start, …, 512}, each verified at 2p, ceiling 1024, where **p_start = max over operands of their selected p**. Each candidate needs every operand's retained state at p and 2p. States already computed by an operand's own schedule are reused (a `RetainedSolve` keeps every state it solved). A missing state is solved on demand (for example an operand accepted at 128 has states at 128 and 256 only; a combination needing 512 solves it at 512). That extra solve's work is charged to the combination, and it never changes the operand's published values, outcome or evidence.
- **Stop rule and scale:** the combination's own published outputs at p and 2p, with S\* from the combination's own 2p rows and body extents (D1 §4.1.1, §4.1.6.1 item 8).
- **At the ceiling** → `CombinationUnresolved` (F2a maps to `RETAINED_PRECISION_UNAVAILABLE`, reason `combination_unresolved`); the operands keep their outcomes (K4-M24).
- Range combinations (envelopes) and T0R's gates stay at the facade.

---

## 8. The method: geometry, ordering, factor, solve, refinement, schedule

### 8.1 Geometry first (D1 §4.1.3; Q6)
1. Bodies: union–find over members; an element-free node is its own body.
2. Per body: local coordinates (ascending node index); grounds = constrained DOFs, global-axis spring DOFs, and, per node and kind, that node's **directional springs if their directions span R³, decided exactly** (a nonzero determinant of some triple, from exact triple products at p = 192, L = 4). If a node has directional springs of a kind that do not span R³ → `Refused(DirectionalSpringsDoNotSpan)` (Q6 as ruled). **[OPEN] O1** below.
3. `assess_rigid_body(coords, grounds, WeldedUnreleasedElasticFrames)`: `MechanismWitnessed` → `Refused(MechanismWitnessed)` with **no rows and no attempt**; `Restrained` or `NumericallyUnresolved` → proceed (an unwitnessed mechanism then fails the pivot screen at every p and ends unresolved, never "solved"); an `Err` → `Refused(GeometryUnavailable)`.
- **The libm note for RETURN:** `assess_rigid_body` uses binary64 `hypot` and a floating SVD to propose candidates; the witness itself is exact. A platform difference could only change whether a borderline body is witnessed (refused) or proceeds (then unresolved at the ceiling); it never contributes a published value.
- **RF-MECH-LINE-IN-CHAIN1000** (1,005 members): bodies and geometry only; the line body is witnessed and the case is refused before any pattern, factor or dense structure. Memory O(members).

### 8.2 Ordering (Q7) and equilibration
- Port of `sparse_direct::reverse_cuthill_mckee` (`lib.rs:505-600`): symmetrize, sort, dedup; neighbours by (degree, index); components by lowest index; pseudo-peripheral start by BFS level structure; reverse. Integers only.
- **[POSITION] Input graph:** the free–free sub-pattern of the **structural** `SparsePattern` (all coupled positions, independent of values), so the order is a pure function of connectivity and the free set. The binary64 sparse path orders the nonzero prepared values; V-K's cross-crate equality test must feed both the same adjacency (noted for V-K).
- Skyline `first[i]` from the ordered pattern; `profile_entries` recorded.
- **[POSITION] Radix equilibration as M03** (`prepare_bound` `:1258`): K̃ = S K S with s_i = −⌊e(K_ii)/2⌋, applied exactly (`mul_pow2`), so values are unchanged; it makes the Hager–Higham estimate the same scaled quantity M03 uses. A zero diagonal at a free DOF (exact) → `ZeroDiagonal`, terminal (precision cannot create stiffness; geometry normally witnesses it first). A negative diagonal cannot be formed by frames and positive springs.

### 8.3 Profile LDLᵀ at p and the pivot screen
- The loop of `ProfileFactor::factor_structural_profile` (`:1962`) in `Wide` at p (sequential rounded operations: the factorization is exempt from the exact-sum rule, D1 §4.1.2), with `work[k] = l_ik·d_k`, cancellation scale c_i = |ã_ii| + Σ|work_k·l_ik| and m_i = 2(i − first_i) + 2.
- **Screen decided exactly:** d_i > 64·γ_p(m_i)·c_i ⟺ d_i·(2^p − m_i) − 64·m_i·c_i > 0, one `ExactWideSum` of three exact products; no binary64 γ. A failure escalates (never read as a mechanism).
- The pivot margin d_i/(64γ_p(m_i)c_i), minimum over i, recorded as evidence, **rounded downward** to binary64 (§8.9).
- **Negative energy** (D1 §4.1.3; §4.3 "for pattern pairs only, at p, against the intended K"): on a pivot failure, for each free–free pattern pair (i, j), E = K_ii + K_jj − 2|K_ij| as an exact sum; a witness requires E < −64·γ_p(4)·(K_ii + K_jj + 2|K_ij|), decided exactly **[POSITION: the allowance keeps a rounding-level zero-energy pair from being read as negative]**. A witness → `Refused(NegativeEnergy)`, never escalated. No valid source reaches it (tested on a constructed pair through the internal function).

### 8.4 Condition estimate
Hager–Higham with the alternating-vector safeguard, mirroring `ConditionMatrix::estimate_rcond` (`:1537`) at p on K̃, with the p-factor's solves; the 1-norms and dot products are exact sums rounded once. rcond = 1/(‖K̃‖₁·est) at p. **`rcond ≤ 2^-(p−1)` (decided as ‖K̃‖₁·est ≥ 2^(p−1) on the rounded product) → escalate.** Published as model information with its label.

### 8.5 Solve and refinement (D1 §4.1.4)
1. rhs (§4.3 step 8), scaled by S; solve with the p-factor; unscale.
2. Residual against K re-formed at q = p + 64 **from the same primitives** (a fresh `form_members` and `assemble` at q, width R): r_i = one exact expansion of the ledger at i (exact, `add_to`) and −K^q_ij·u_j over the full row (prescribed columns with the binary64 prescriptions), **never rounded before the gate**.
3. Denominator d_i = |f_i| + Σ_j |K^q_ij·u_j| (exact; the coalesced-entry convention of M03's gate and of `audit_intended_action`), m_i = 2·count + 2.
4. **Gate decided exactly:** |r_i| ≤ 64·γ_p(m_i)·d_i ⟺ |r_i|·(2^p − m_i) ≤ 64·m_i·d_i (γ at p, not at q: D1 gates against 64·γ_p(m_i)), via `add_scaled` on the two exact accumulators; no binary64 guard terms are needed because nothing is rounded before the decision. The worst ratio |r|/(64γ_p d) is kept as evidence, rounded upward.
5. Correction: δ = factor.solve(r rounded **once, directly from its exact sum**, to p); u ← fl(u + δ) (a two-term sum rounded once, `ctx.add`). At most three corrections; stop early, as M03 does, when the worst ratio does not decrease; then escalate.
6. **The ceiling (Q4(a)):** at 1024 (verification-only), q = 1024 and K^q is the factor's own K (no re-formation), with the same exact residual, gate at 64γ_1024 and ≤ 3 corrections; the attempt records `residual_basis = 1024` (= p).

### 8.6 The schedule (D1 §4.1.6) — per case
```
solves at P = [128, 256, 512, 1024], lazily, in order; each solve = formation … recovery, or Failed(reason)
c = 0                                         // candidate index into [128, 256, 512]
loop c in 0..3:
    s_c = solve(P[c])            (already present if it was the previous verification)
    if s_c failed: record(P[c], Failed(reason)); continue
    v = solve(P[c+1])
    if v failed: record(P[c], Rejected(VerificationFailed)); record(P[c+1], Failed(reason)); continue
    if stop_rule(s_c, v) accepts: record(P[c], Accepted), record(P[c+1], Verified) → Selected(p = P[c], verification = P[c+1])
    else: record(P[c], Rejected(StopRule{worst body, kind, quantity})) and P[c+1] becomes the next candidate
→ Unresolved { Ceiling, attempts }
```
At most four solves (128, 256, 512, 1024). Budget, span and exponent-range exits are terminal at any point, with the partial attempt recorded (its charged work included).

### 8.7 The stop rule and S\* at 2p
- Every published quantity of §6 (input-derived rows included; they agree exactly) is compared: **accept iff for every q: |q_p − q_2p| ≤ 2^-64·max(|q_2p|, S\*_2p)**, decided exactly: A = q_p − q_2p (exact, at W(2p)), then 2^-64·M − sign(A)·A ≥ 0 with M = max(|q_2p|, S\*) by `cmp_value` (exact). ±0 compare equal. An all-zero body (S\* = 0) requires exact agreement (it falls out of the formula).
- **S\*_2p per (body, kind)** for kinds translation, rotation, force, moment (D1 §4.1.6; §4.1.6.1 items 4–6 applied at 2p): S(kind) = exact max of |q_2p| over the body's rows of that kind (**input-derived rows excluded**, as in the classification); L_b = §4.1.6.1 item 5's binary64 formula on the body's node coordinates (pinned, IEEE-exact); coupling in item 6's order with the products and quotients L_b·S and S/L_b **rounded once at 2p**; L_b = 0 omits the coupled terms.
- **K4-M14's |q_p| variant** differs from |q_2p| only inside a window of relative width 2^-64, so it is killed by exact boundary unit tests of the predicate (constructed q_p, q_2p, S\*), not by a model (§13).

### 8.8 Factor reuse (D1 §4.1.7) and how its work is attributed — [POSITION]
- `solve_cases` groups sources by `stiffness_encoding` (byte-equal; groups in first-appearance order). Per group and precision, formation at p and at p + 64, assembly, pattern, ordering, equilibration, factor, pivot screen and rcond are formed **once** and shared; each case runs its own ledger, rhs, solve, residual, refinement, recovery and stop rule. Each case's decisions depend only on its own solves, so **every outcome, state and evidence field is bit-identical to a separate solve** (tested).
- **Attribution:** the shared work is recorded once per group and precision (`shared_work`). Each case's per-case limit is checked against its own work **plus the full shared work of every precision it used**, so budget decisions per case are also identical to separate solves. The invocation meter is charged the shared work **once**. So reuse saves invocation budget only.

### 8.9 Directed roundings of evidence ratios (the summary ratios)
- The stop-rule summary per (body, kind): the worst |q_p − q_2p|/max(|q_2p|, S\*) **rounded upward to binary64**, so D2's G5a check "≤ 2^-64" stays sound (2^-64 is representable, and fl↑ is the least binary64 ≥ the exact ratio). Method: c₀ = to_binary64(the quotient at 64L bits); then step with `next_up`/`next_down` (integer bit operations) until c·M ≥ A and next_down(c)·M < A, each comparison an exact sum (`two_product` of c and M at the width of M, then minus A). M = 0 only when A = 0 (ratio 0).
- The residual summary: fl↑ of the worst |r|/(64γ_p d). The pivot margin minimum: fl↓. rcond: nearest (model information).

---

## 9. Canonical encodings (Q10)

All little-endian, versioned, order-independent (lists sorted canonically), no hash computed in FK:
- **Source** (`K4SRC\x01`): node count and coordinates (f64 bits verbatim); members by id (id, i, j, the six property bits, y_ref bits); springs by id (id, node, component, k bits); directional springs by id (id, node, kind, direction bits, k bits); constraints by DOF (DOF, value bits); loads by (DOF, source id bytes, value bits) with u32 length prefixes; stations by id; support groups by id. `stiffness_encoding` is the same minus loads and constraint values (constraint DOFs kept).
- **Ledger** (`K4LED\x01`): loaded DOFs ascending; each exact net in canonical form (sign, exponent of the lowest set bit, odd integer magnitude as u32 limb count + limbs); an exactly cancelled DOF is (0, 0, empty).
- **Retained state** (`K4RST\x01`): p, L; u for every DOF ascending, then every member's Q[0..6] by member id; each value as K3's `parts()` (sign byte, exponent i64, L limbs), with **a zero's sign normalized to +** so a −0 from a solve never changes the digest. D1 §4.1.8: "the canonical limbs of u_p and every member's Q".
- **Tests:** the generator builds the source and ledger encodings independently (hex vectors) and records their sha256 with `hashlib`; the Rust tests assert byte equality. Retained-state encodings: see O8.

---

## 10. Budgets and work (Q5)

- **API:** `CaseLimit::new(u64)` and `InvocationMeter::new(u64)` are required parameters of every entry point; neither implements `Default`; K4 ships **no numeric limit** (tests pass explicit values). ROOT's ruling stands: F2a does not merge without ROOT's limits.
- **Units:** limb-multiply equivalents: K3's `limb_multiply_cost` per `Wide` operation (counted by the contexts), plus K4's `SumWork` (1 per limb touched, shifted or netted, and the `from_integer` input length, K3's N3). Integer bookkeeping (pattern, RCM, geometry) is not charged; it is reported as storage counts. Binary64 classification is not charged.
- **Recording (K3's N2):** one context per (width, precision) per attempt, `AttemptWork::record`ed **once** at the attempt's end. Per-stage figures are differences of `ctx.work()` snapshots (read-only), so no context is recorded twice.
- **Exhaustion checks** at every stage boundary and inside the factor (per row), residual (per row) and recovery (per member) loops, so any overrun is bounded by one row or member. Exhaustion → the partial attempt is recorded with its work, the case is `Unresolved { Budget(scope) }`; once the invocation meter is exhausted every later case and combination returns it with no attempt. **Successful, failed and verification work are all charged.**
- **Deterministic tables committed at D** (per named case, by stage and precision): limb-multiply equivalents; pattern and profile entries; limbs per entry. Debug wall times are observations only.

---

## 11. The ceiling (Q4(a)): outline of the forward-error argument (to be written step by step in RETURN, including its reliance on the uncertified estimate)

1. **What the verification solve must deliver.** q_1024 is only a comparator for the 512 candidate; the stop rule's guarantee needs |q_1024 − q\*| ≪ 2^-64·max(|q\*|, S\*) (D1 §4.1.6, "What acceptance guarantees").
2. **Formation.** K_1024 is re-formed from the binary64 primitives at 1024 bits; each element entry carries a relative error of at most c_f·2^-1024 of its contributions' magnitudes (c_f = the number of roundings in §4.3, counted in RETURN; ≲ 20), and the assembled entry one more rounding. So K_1024 = K\* + ΔK_f with |ΔK_f| ≤ (c_f+1)·2^-1024·|K|_contrib componentwise; the ledger is exact, so f has no formation error.
3. **Backward error of the solve.** The exact residual gate at 1024 bounds |f − K_1024 u_1024|_i ≤ 64γ_1024(m_i)·(|f_i| + Σ_j|K_1024,ij||u_j|), a componentwise backward error ω ≤ 64γ_1024(m) ≈ 64·m·2^-1024 relative to K_1024 and f.
4. **Combined:** u_1024 solves (K\* + ΔK)u = f + Δf with |ΔK|, |Δf| ≤ ε·(|K|, |f|) componentwise, ε ≈ (64m + c_f + 1)·2^-1024.
5. **Perturbation bound** (Oettli–Prager / Skeel): ‖u_1024 − u\*‖/‖u\*‖ ≲ cond(K\*)·ε/(1 − cond·ε), in the equilibrated norm the factor works in (§8.2).
6. **The condition number — the uncertified step.** The 512 candidate passed rcond_512 > 2^-511, i.e. **est(κ(K̃)) < 2^511**, and its pivot screen and refinement. Hager–Higham is a lower-bound estimator and is not certified (as everywhere in M03); the argument **assumes the true κ exceeds the estimate by at most a factor F**. The 1024 solve's own screens (rcond_1024 > 2^-1023, pivots, refinement) are consistent but weaker.
7. **Result:** forward error ≲ F·2^511·(64m + c_f + 1)·2^-1024 = F·(64m + c_f + 1)·2^-513. For m ≤ 2^20 (far beyond any row count W1a admits) that is ≲ F·2^-486, which is below 2^-64·2^-50 (a 2^50 margin under the stop rule's threshold) for every **F < 2^372**. Hager–Higham underestimates by small factors in practice (typically < 10); the argument fails only if it is wrong by more than 2^372.
8. **From u to the published quantities:** recovery at 1024 is exact expansions rounded once (§6), so each quantity's error is its linear image of the u error plus 2^-1024 relative; the kind-coupled S\* (translation↔rotation via L_b, force↔moment via L_b) bounds those images body-wise. The mixed-unit step (equilibrated norm → per-kind body scales) is where the argument is an argument, not a proof; RETURN states it explicitly and the reviewer checks it.
9. **A second, estimate-light route (to be written beside steps 5–7).** Both the 512 and the 1024 solves are dominated, to first order, by perturbations proportional to their unit roundoffs (formation and backward error; the ledger and every sum outside the factor are exact, so there is no common-mode term, D1 §4.1.9). So err_1024 ≈ 2^-512·err_512 in the first-order regime, and acceptance (|q_512 − q_1024| ≤ 2^-64·S\*) then bounds err_1024 ≲ 2^-512·2^-64·S\*/(1 − 2^-512). The first-order regime is exactly κ·2^-512 ≪ 1, which rcond_512 > 2^-511 **suggests but does not certify**; near that boundary (est κ between about 2^507 and 2^511) the formation perturbation at 512 can hide a singular intended K, and then the 512 solve is wrong at O(1) and in a different direction from the 1024 solve, so the rule rejects. RETURN writes both routes, names the uncertified step in each, and the reviewer checks them.
10. **What it does not claim:** no certified bound; the stop rule can accept a wrong 512 candidate only if the 1024 solve is wrong **by nearly the same amount in the same direction**, which requires a precision-independent (common-mode) error, the class revision 2's exact sums remove (D1 §4.1.9).

---

## 12. Tests (brief's A–L), where they live, and the generator

### 12.0 Layout (`FK/tests/retained_k4/`)
- Test modules: `wide_sum_tests.rs`, `source_tests.rs`, `ledger_tests.rs`, `assemble_tests.rs`, `factor_tests.rs`, `recover_tests.rs`, `combine_tests.rs`, `adaptive_tests.rs`; shared parsing and builders in `support.rs` (`#[path = "support.rs"] mod support;` from each, relative to the test file's directory). No `main.rs`.
- **Generator** `gen_k4_vectors.py` (standard library; `--check` regenerates in memory and compares byte for byte). It imports read-only, with sha256 recorded in its header and in RETURN: K3's `gen_wide_k3_vectors.py` (`ca48d1bd…`: `W`, `rnd`, `SplitMix64`, `gen_operands`, `apply` for the N5 streams), R1's `references.py` (`80d473a7…`, for full models and the exact π; `references.json` `7b176dbb…` read and hash-checked), D1's `floor_kinds.json` (`83265615…`, §4.10's enumerated not-covered list), and reads model inputs from `kd5_models.rs` (`17a4680c…`, D5C-1 controls) and `k2b_models.rs` (`c23eedd2…`, spring-carried). D1's probes (`probe_skew_precision.py` `b36e2a64…`, `probe_rev2_b1.py` `0f1a47bd…`) supply the **definitions** of the k = 1e-28, six-member, B1 and S8-W models only; K4's generator computes its own exact expectations.
- **Vectors:** `sum_targeted.txt`, `sum_differential.txt` + `sum_differential_sample.txt`, `streams.txt` + `streams_sample.txt`, `ledger.txt`, `formation.txt`, `models.txt` (every model as binary64 bits with its exact expectations), `r1_cases.txt` (adapted R1 cases, expected values and scales, floor classification, negative-control values), `classification.txt` (J), `encodings.txt`; `SHA256SUMS` over all of them.
- **Exact expectations** for K4's own controls: exact rational direct stiffness from the binary64 inputs (prescribed values included), with circular sections in projector form (orientation-free); controls that assert individual local components use rational-axis geometries only (axis-aligned; (3,4,0) with y_ref (0,0,1); (2,3,6) with (6,2,−3); (1,2,2) with (2,1,−2)). Irrational results (bending magnitudes) at 70 digits.

### 12.1 The R1 adapter (and how `references.py` defines the represented inputs)
- `references.py`'s `Model(defn, Exact(), decode)`: `decode` is the identity (intended) or `rep(x) = Fr(Decimal.from_float(float(Fr(x))))` (represented), applied to **node coordinates, E, G (or ν), OD, ID, spring k, loads and each load contribution**; **spring directions are not decoded** (exact integer vectors); sections A = π·(OD²−ID²)/4, I = π·(OD⁴−ID⁴)/64, J = 2I in **exact rationals with π = PI_Q (190-digit Machin)**, from the decoded OD and ID.
- **The adapter:** every kernel input is binary64, so it is **fl() of the value `references.py` uses on the case's basis**: coordinates, E, G, k, load values = rep(x); A = fl(A_basis), Iy = Iz = fl(I_basis), J = fl(2·I_basis) = 2·fl(I_basis), where "basis" is intended or represented per the case's `basis` field. Directions are the integer vectors (exact in binary64). **At a node where any spring of a kind has a non-axis direction, every spring of that kind at that node becomes a `DirectionalSpring`** (so the 345 AX triads (3,4,0), (−4,3,0), (0,0,1) stay one spanning directional triple under Q6's rule as ruled; otherwise (0,0,1) would become a global-axis spring and the remaining pair could not span); every other spring becomes `Spring{dof, k}`. The physics is identical (a directional spring along a global axis equals the global-axis spring). y_reference = (0,0,1) unless the member is exactly parallel to Z, then (1,0,0) (R1's sections are circular, so R1's quantities do not depend on it). Rigid DOFs → `Constraint{value: 0}`. RF-CANCEL's `load_contributions_in_authored_order` → one `NodalLoad` each, authored order, source ids `c0, c1, …`. Every member gets a station at t = 0.5 (Mb.mid). **Properties used and stated:** E, G, OD, ID, A, I, J = 2I, π = PI_Q.
- The difference between R1's represented model (exact A_rep) and the kernel's (fl(A_rep)) is a relative 2^-53 in A, I, J; the generator computes the exact solution of the kernel's own binary64 model for the two represented-basis cases and reports its distance from R1's `expected_represented` (expected ≪ 1e-9; reported, not absorbed).
- **Comparisons:** R1's keys (`u`, `th`, `R`, `S.<node>.<i>.F*|M*`, `N`, `T`, `Mb.i|mid|j`, `tw`, `ext`), the unchanged predicate |obs − exp| ≤ 1e-9·max(|exp|, scale) with R1's class scales (RF-CANCEL: the binding net-governed column), expected values rounded once to binary64 by the generator. tw = fl(T/k_t), ext = fl(N/k_a) with k_t = fl(fl(G·J)/L), k_a = fl(fl(E·A)/L) and L the product's binary64 length (`FK/lib.rs:584-588`), never differenced.
- **The floor check:** S\* from the **reference values** with K4's §4.1.6.1 functions (one body per compared case; twist fl(mo/k_t), extension fl(fo/k_a) per member); a comparison with scale < R·S\* is `not_covered`, never a pass. Reported as three numbers (passes, absolute-range passes — none expected in K4's families — and not-covered). The set must equal `floor_kinds.json`'s variant-F lists restricted to K4's cases (RF-WEAK 46, RF-CANCEL 3 under `F_rec`, RF-SKEW 2). floor_kinds.py used 60-digit decimals and exact GJ/L; K4 uses the pinned binary64 formulas, so a borderline row could differ; any difference is reported (stop item).
- **The discrimination check:** each R1 negative control with `discriminates: true` for a K4 case must fail the same predicate on its published `values`; non-discriminating ones are listed.

### 12.2 The exact-block in-scope list (enumerated by structure; confirmed at A with `exact_boundary::Context::new`'s own block check)
In scope (every free block of order ≤ 2, axis-aligned, nodal loads, ≤ 256 DOFs): **N01; N05; N06; N08 (four torque cases); N09 (bending and torsion).** Out of scope: every R1 case in K4's families (chains, skew, weak, finite, cancel: blocks of order ≥ 3), the K-D5 controls (root rotational springs couple the root rotation with the tip, order 3), the probes, N02–N04 (mechanisms or order 4), and the spring-carried G = 1e-300 case (one free DOF, but its represented binary64 system cannot be formed: K2a refuses G·J, the trigger K2b ruling A kept). The test builds the represented system from FK's binary64 element formation and springs, runs `Context::new` + `solve`, and compares W1's values with `project_displacement`/`project_reaction` under the criterion with the body-level coupled scale stated in the test (D1 §4.4.1 item 4).

### 12.3 Test list

**A. Nothing existing moves:** K3a's and K3's directories byte-identical and both generators' `--check` OK; K3a/K3/K-D5/`exact_sum` tests unchanged and passing; FK full suite and CI's 39-manifest profile (`--no-fail-fast`) against ROOT's Mac baseline; warning-free non-test build; the site table with the declared Q8 extension; T9 (ROOT).

**B. Multi-term sum:** Fraction oracle at 128, 192, 256, 320, 512, 576, 1024 at their widths; RV12's counterexample (1 + 2^-127, 2^-128, −2^-400 → 1 + 2^-127 at 128); ties decided by a far tail, at p = 64L and 64L − 1; total cancellation → +0 (bits); mixed signs; gaps beyond the width; product, binary64 and 68-limb-net terms; span limit: 8,128 bits accepted, 8,129 refused (never truncated) and an exponent refusal; re-anchoring in both directions; seeded differential ≥ 10^5 sums per precision, 2–64 terms, K3's scheme (seed, chunk and stream digests; first 1,000 records committed).

**C. Ledger and accessor:** netting with (+,0) for zero and the −2148 quantum; accessor + `from_integer` at p = 53 equals `round()` **on normal results** (**[REFINE]**: in the subnormal range a p = 53 rounding followed by the binary64 conversion is a double rounding, so K3's own test restricted this check to normal results; I add the exact form: accessor + `from_integer` at a p ≥ the net's span, then `to_binary64`, equals `round()` everywhere, with overflow ↔ `NonRepresentable` and underflow ↔ +0.0 as the only mapped differences); Fraction at every K4 precision on seeded ledgers with cancellation and ties; RF-CANCEL authored-order contributions and check L (1e80, 1e-8, −1e80) exact; **a prescribed-coupled rhs row whose correct rounding is a tie decided by the ledger's tail** (kills K4-M8).

**D. N5 streams:** 10^6 operations of + − × ÷ √ at p = 128, 192 (L = 4), 320 (L = 8) and 576 (L = 16), K3's operand rules, new seeds (`K4_P0128`, `K4_P0192`, `K4_P0320`, `K4_P0576`), Fraction oracle via K3's generator functions; the Rust side regenerates operands (a test-only port of K3's SplitMix64 and operand rules, since K3's test helpers are private to `multi::tests`) and compares digests; full count, opt-level 0, no `#[ignore]`; debug wall times recorded as observations.

**E. Formation, assembly, reduction:** rigid modes: for the six rigid motions r of the binary64 geometry, |K_e·r| ≤ 64·2^-p·Σ|K_e||r| componentwise (exact), at every K4 precision, axis-aligned, (3,4,0) and (2,3,6); against the product's element: at p = 53 the local Bᵀ_local D B_local agrees with `local_stiffness` to about 1e-16 with the same zero pattern; **against K-D5 at 128: a test-only port of K-D5's `chord_axes`/`frame_matrix`/`rotate` sequence on K3a's `WideArith`** (they are private to `formation_check`; K3a's `Wide2`/`WideArith` are reachable), agreement within a derived bound (≤ 64 ulps of 128 bits of the element's largest entry; the derivation from the two rounding counts goes in RETURN); assembly entries equal the generator's Fraction emulation of §4.3 (p-rounded contributions, summed exactly, rounded once), bit for bit; order independence: permuting member, spring and load lists gives bit-identical K, rhs and encodings; §7.3-16's duplicate-operand control (two identical members meeting at a node, a third 2^-300 as stiff, listed in the order in which a sequential fold loses it); a prescribed-motion control (KREV-02 analogue) with an exact reference; `from_positions` pattern equals `from_connectivity`'s where both apply.

**F. Factor, screens, solve, refinement:** pivot failure at 128 that passes at 256 (a single member with k/a = 2^-120); rcond escalation on a constructed matrix whose pivots pass (a 70×70 LDLᵀ with unit bidiagonal factor of −2s: κ ≈ 4^n; rcond ≤ 2^-127 at 128, passes at 256) through the internal factor; negative energy on a constructed indefinite pair through the internal function; geometry first: RF-MECH's 8 refusal cases refused with no rows and no attempt, LINE345-RX-COMPANION solves; **RF-MECH-K0: the adapter passes k = 0 and source validation refuses it (`NonPositiveSpring`) [POSITION]**, plus an extra control with the zero spring omitted, which geometry refuses; the p + 64 residual control (see O5); at most three corrections; the ceiling solve as ruled (residual basis recorded as p).

**G. Schedule and stop rule:** k = 1e-28: 128 rejected, 256 accepted against 512, within 1e-9; the six-member skew run k = 1e-12: 128 rejected (the scratch indicates 2.5e-19 > 2^-64 with K4's formation), accepted p recorded; N05 and N06 accepted at 128; attempts list, verification reuse, ≤ 4 solves; every published kind in the comparison; a structural-zero kind accepted through the coupled S\*; an all-zero body agrees exactly; ceiling and budget exhaustion unresolved with attempts and reason; exact boundary tests of the predicate (±1 ulp around 2^-64·max(|q_2p|, S\*), and a constructed case where |q_p| ≠ |q_2p| decides); a reactions-only control (see O6).

**H. Recovery and publication:** end actions, stations (including an asymmetric t = 0.25 station with an exact reference), spring actions (global-axis and directional), reactions, magnitudes, each rounded once with its outcome; exact zero → +0.0 (bits); subnormal, underflow and overflow outcomes on constructed values; N06's torque and spring action nonzero and correct (§7.3-2).

**I. Combinations:** B1-C (A + B − A2, A = A2 = 1e80, B = 1e-8) exact; B1-E ((P, ε) − P, ε/P = 1e-45): 128 rejected, 256 accepted against 512; independent escalation; `CombinationUnresolved` at the ceiling (a constructed ceiling) with the operands' outcomes unchanged; S\* from the combination's own body; the precision rule (an operand accepted at 256; on-demand operand solve charged to the combination).

**J. S\* and classification:** bit for bit against the generator's binary64 reimplementation (seeded synthetic row sets): coupling order, L_b, S(kind), per-member stress scales with k₁, k√2 = `0x3FF6A09E667F3BCD`, k_{2√2} = `0x4006A09E667F3BCD`, k₄, k_i = fl↑(k√2·i); rows one ulp either side of t = fl(R·S\*) (§7.3-20); S\* < 2^-988 → all `absolute_verified`; b = fl↑(2^-64·S\*) including 0 < S\* < 2^-1011, b = 0 only at S\* = 0; `input_derived` for restrained and prescribed DOFs; S8-W's far-node quantities `absolute_verified` with R = 2^-34.

**K. References and routed cases:** N05, N06, NP-A on the intended basis, every quantity including internal torque and the spring action, and **a result equal to NP-A's represented solution is a failure**; N05 with a transverse tip force (T3's N05 item; S11-F's `s11f_tests.rs:1200` says the replay overflow occurs "whatever the load", so the invented force is F_y = 1 N at the tip beside N05's torque, exact reference by the generator) within 1e-9; R1 frozen at `c0f14201c` through the adapter: RF-CHAIN 30, RF-SKEW 36 (see O1), RF-WEAK 9, RF-FINITE 6 (THIRTIETHS-O1e6 represented), RF-MECH 9, RF-CANCEL 38 (UDL cases excluded, W1b), RF-SKEW-A-CANT-AX-122-r1e-12 represented; the floor check and the discrimination check (§12.1); the exact-block oracle (§12.2); routing targets: RF-SKEW-T-CANT-OFF-122-r1e-04 and K-D5's D5C-1 controls (PROBE_D, PROBE_C, BENDING_SOFT from `kd5_models.rs`) within 1e-9; the spring-carried G = 1e-300 case (outcome reported: I expect it selected at 128 with θ = 1.0 exactly and the member torque and root reaction ≈ 2^-1082 published as `Underflow`, explicit, see O9); B1-L exact with the ledger.

**L. Budgets, work, determinism:** golden work counts per case by stage and precision (kill K4-M17/M18/M19); `from_integer`'s length charged; failed and verification work charged; limits as parameters, exhaustion per case and per invocation; determinism across runs and permutations; factor reuse bit-identical to separate solves (outcomes, states, evidence, per-case budget decisions); **the source scan**: K4's files (lexed, comments and strings stripped) contain none of `powi`, `powf`, `exp`, `exp2`, `exp_m1`, `ln`, `ln_1p`, `log`, `log2`, `log10`, `sin`, `cos`, `tan`, `asin`, `acos`, `atan`, `atan2`, `sinh`, `cosh`, `tanh`, `asinh`, `acosh`, `atanh`, `hypot`, `cbrt`, `mul_add` (with a lexer self-control).

---

## 13. Mutants

Clean copy and target per mutant under `<wt>/k4-mut/<mutant>/`, NONE first, ≤ 3 at once at `-j 4`, targets deleted afterwards. Each must be killed at a behavioural assertion (named at C).

| # | Mutant | Planned kill |
|---|---|---|
| D1 | promote FK's binary64 assembled K to p | N05 wrong k; N06 nonpositive pivot → unavailable; NP-A "equal to represented" |
| D2 | round u to binary64 before recovery | N06 torque 0 (H) |
| D3 | drop K_fc·u_c / round the reduced rhs to binary64 | prescribed-motion control (E) |
| D4 | omit a contribution / flip an axis sign in B | rigid modes (E); N and R1 (K) |
| D5 | fix p = 128 | k = 1e-28 (G) |
| D6 | skip the 2p verification | k = 1e-28 (G) |
| D7 | disable geometry | RF-MECH: attempts non-empty, outcome not `Refused` (F, K) |
| D10 | order from list order | permutation determinism (E, L) |
| D13 | fold loads at p | B1-L (accepted with error 0.5); RF-CANCEL (K) |
| D14 | combine term by term at p | B1-C (I) |
| D15 | combination outside the stop rule | B1-E published from 128 (I) |
| D16 | sequential addition in assembly (and in recovery) | duplicate-operand control, assembly entry (E); the recovery half's control is designed at A (O6) |
| D17 | drop classification | S8-W (J) |
| D20 | classify on the p value / R = 10^9·2^-64 rounded / S\* not from published | one-ulp rows (J) |
| D25 | k = 1 everywhere / √2 and 2 for span statics | per-member scale bits (J) |
| K4-M1…M25 | as the brief's table | as the brief's table; M14's |q_p| variant by the exact predicate tests (G); M11 see O5; M16 see O6 |
| K4-M26 | directional span decided with a binary64 determinant | a triple with exact det ≠ 0 whose binary64 det is 0 (F) |
| K4-M27 | station interpolation swaps t and 1 − t | the t = 0.25 station (H) |
| K4-M28 | RCM tie-break by descending index | FK-local RCM tests on sparse_direct's graphs (F) |
| K4-M29 | classification with ≤ t instead of < t | the row exactly at t (J) |
| K4-M30 | combination starts at 128 regardless of operands | the combination's attempts on the 256-operand control (I) |
| K4-M31 | a binary64 load fold in a K4 file (`f += load.value`) | the S11 site table (Q8 condition 3) |
| K4-M32 | rcond on the unequilibrated K | attempts/rcond evidence on the k = 1e-28 case (F, L) |

---

## 14. Dead code

- **Per-item `#[allow(dead_code)] // F2a API` or `// V-K API`** on each entry point with no non-test caller: the source constructors and accessors, `solve_cases`/`solve_case`, `CaseLimit`, `InvocationMeter`, `RetainedSolve::{publish, evidence, source, selected_precision}`, `RetainedCombination::{solve, publish, evidence}`, the §4.1.6.1 functions (F2a's product rows), the encodings and the ordering accessor (V-K API). Private helpers stay live through them (K3's probe finding), so they carry none; `form` is live through `solve`.
- Test-only helpers are `#[cfg(test)]`. No module-wide or `cfg_attr` allowance. The non-test FK build has no warnings.
- Minimality shown as K3 did: with every allowance removed, the non-test build's warnings are exactly those entry points and the helpers they keep live (recorded at A).
- K3's `// K4 API` allowances in `multi.rs` stay (not edited; they become redundant but harmless).
- **Q9 export list for F2a/V-K** goes in RETURN's "F2a and V-K interface" section (the types and functions of §4 with those labels).

---

## 15. Q8: the S11 site table
K4's nine files join `SOURCES` (additive). Rows: every function with a scanned shape, with its count and disposition (integer loops and counters, and the new disposition **"p-bit exact expansion, rounded once"** as count-0 rows naming each K4 exact-sum site: `assemble`, `reduced_rhs`, the residual row, the recovery functions, reactions, the combination state and ledger, the ledger construction). The factor's and solves' sequential `ctx.add` sums are invisible to the scan (method calls), as the table's stated limit says, so D13–D16 and K4-M1 remain load-bearing. K1's five conditions: additive; a disposition for every match; K4-M31 killed by the table; declared in CHANGE_RECORD and RETURN; a site that fits no disposition stops the work.

---

## 16. Host plan
`RUSTUP_TOOLCHAIN=1.97.1 RUSTUP_AUTO_INSTALL=0 CARGO_INCREMENTAL=0 cargo … --offline --locked -j 8`, `RUST_TEST_THREADS=4`, target `<wt>/k4-target`, at most two cargo jobs of mine; no dense matrix above 1,000 members anywhere (the 1,005-member case never reaches the pattern); memory guard log checked on any SIGKILL. Python: `<VENV>`, generator runs under `nice`. `/usr/bin/grep` only.

---

## 17. Scratch evidence at checkpoint 0
`<scratch>/cp0_exact_formation_probe.py` and `cp0_by_kind.py` import D1's probe read-only and replace only its formation with §4.3's (exact dots, exact D doublings, DB and K_e one exact expansion, exact assembly and loads); dense LDL, no RCM, scaling or refinement, so **indicative only**:
- k = 1e-28: 128 error 2.4e-4 (rejected, disagreement 2.4e-4), pivot margin 3.38; 256 accepted against 512 (4.5e-43). **Matches G.**
- Six-member k = 1e-12: 128 disagreement 2.5e-19 > 2^-64 (rejected); by group: u 1.7e-19, T 6.9e-20, Mi 1.6e-19, Mj 1.1e-19, R 2.5e-19 — every group is above 2^-64, so this case does not isolate reactions (O6).
- Single member k = 1e-12 (N06 class): accepted at 128 (1.25e-20).

---

## 18. Positions on questions still unresolved (for ROOT)

- **O1 [OPEN; STOP? for the RF-SKEW count] Q6's span rule refuses RF-SKEW-T-PIN-AX (6 cases).** Each T-PIN-AX case has a **single** rotational directional spring along e at N0 with both nodes translation-pinned (verified in `references.json` by an exact determinant scan of every K4-family case: only RF-SKEW has non-axis springs); one direction cannot span R³, so the ruled screen refuses all six as unsupported (it cannot hand them to `assess_rigid_body`, which would falsely witness the axis rotation). So RF-SKEW is 30 compared + 6 refused, not 36. T-CANT-AX and A-CANT-AX run: the 122 triads span as they are, and the six 345 cases span because the adapter keeps their (0,0,1) spring directional (§12.1; without that, the scan finds only 2 non-axis directions there). A possible refinement, not assumed: let the span test count every ground direction of the kind at the node (directional springs, global-axis springs, rigid DOFs); it is equally safe and would not change T-PIN-AX. Options: **(a, recommended)** accept the refusal in K4 as ruled, and route T-PIN-AX's geometry to K5 (W4 generalizes the rigid-body assessment, §4.9) or V-K; (b) a K4-local exact screen for non-spanning directional grounds (exact rank of the rigid-motion constraint rows over rationals; new code outside the ruled rule, ~200 lines + tests); (c) skip geometry for such bodies and rely on the factor (a true mechanism would then be escalated, against "never escalated").
- **O2 [OPEN] Support grouping for the magnitude rows.** D1's table makes `support_reaction_force_magnitude_v2`, `reaction_resultant` and `support_reaction_moment_magnitude_v2` "formed at p" and stop-rule-checked, per product support (`PP` builds a support's vector from its restraints and springs). Only the kernel can form them at p, so the source needs the grouping. **Recommend:** the optional `SupportGroup` list of §4.1 (per-node displacement magnitudes need nothing). Alternative: defer magnitudes to a later K4 addendum before F2a.
- **O3 [REFINE] Member, spring, station and group ids** (§4.1), needed for the permutation-invariant encoding and F2a's mapping.
- **O4 [POSITION] Span refusal terminal** (§3.1), with the max span recorded.
- **O5 [OPEN] K4-M11 (residual at p instead of p + 64).** Derivation: with exact assembly the p + 64 residual differs from a p residual by K_p's formation error, (K_q − K_p)u. On rows whose |K||u| terms are large (every row that carries an amplified soft mode) that difference is inside 64γ_p(m)·d and triggers no correction, so both variants give the same attempts and published values. A difference appears only on rows whose assembled entry cancels its contributions while the row's other terms are small (for example two collinear spans of lengths 1 and 1 + 2^-40 between fixed ends, loaded by a moment at the shared node, row UY there): the p + 64 residual fails the gate, and iterative refinement with the more accurate residual converges in one or two corrections (κ·2^-p ≪ 1). The published binary64 values agree either way (the formation error's effect on u is ~2^-p of S\*). **So the only behavioural kill is on evidence: the attempt's `corrections` (≥ 1 vs 0) and the retained state's low bits (and golden work).** I will build that control at A; ROOT rules whether an evidence-level kill suffices (as for M31b), or accepts the derived equivalence at the published-value level.
- **O6 [POSITION] K4-M16 control.** The six-member case disagrees in every kind, so K4-M16 needs its own control. Planned: a **prescribed rigid translation** of a member's two end nodes plus a tiny load elsewhere: u agrees at 2^-p of S\*(translation), end actions are exactly zero (d_j − d_i cancels exactly), but the reactions K_cj·u_j carry the element's rigid-mode leakage O(2^-p·EA/L) against a tiny S\*(force), so only the reaction kind disagrees at 128. For spring actions a separate control is designed at A; if none is admissible, I derive why and report.
- **O7 [POSITION] Combination rules** (§7): shared stiffness identity required; p_start = max operand p; on-demand operand solves charged to the combination.
- **O8 [OPEN, recommendation] Retained-state sha256 pins.** Source and ledger encodings are pinned independently (generator bytes). For the retained state, **recommend:** the generator emulates §4.3–§8.5 bit for bit for three tiny cases (N05 at 128/256, N06 at 128/256, one skew member at 128) and pins their encodings' sha256 — an independent second implementation of the method's arithmetic order, which CI's Linux run then checks cross-platform. For every other named case the determinism tests (runs, permutations, reuse) apply without a pinned digest. Fallback if the emulation proves disproportionate at A: pin Rust-produced golden encodings (determinism evidence only), and say so.
- **O9 [POSITION] Rows without a binary64 value** (underflow/overflow) are listed as `unpublishable`, excluded from S\*(kind) and from both classification lists; F2a decides the case's standing. The spring-carried case will exercise it (its torque ≈ 2^-1082).
- **O10 [POSITION]** rcond on the radix-equilibrated K (§8.2); RCM on the structural pattern (§8.2); negative-energy allowance (§8.3); exact screen and gate decisions (§8.3, §8.5); frame dot products exact (§4.3); the station convention (§6).
- **O11 [OPEN, cost] Debug-suite time.** Estimate at opt-level 0: streams ~6 min (K3's §8 rates), the sum differential ~1–3 min, the R1 and control solves ~5–8 min (≈ 30k `Wide` operations per solve at 30–80 µs each, two to four solves per case, ~140 cases, parallel at 4 threads), so FK's debug suite may grow by roughly 10–15 minutes. Measured at A; hosted CI's numerical-job time goes in the merge record. If ROOT wants a cap, the R1 lane could be split across more test functions (parallelism) — never reduced or ignored.
- **O12 [NOTE] The residual gate's denominator** is the coalesced |K_ij||u_j| (M03's convention). O5's analysis shows refinement with the p + 64 residual repairs cancelled-entry formation error, so this convention causes no availability loss; recorded in case ROOT prefers a contribution-level denominator.

## 19. Stop items at checkpoint 0
- **None requiring an edit outside the write set.** Everything above uses K3's §14 API, `SparsePattern`, `assess_rigid_body`, `ExactAccumulator` plus the approved accessor, and (tests only) `exact_boundary`, K3a's `WideArith` and FK's binary64 element functions.
- **O1** changes a count the brief states (RF-SKEW 36 → 30 compared, 6 refused) as a direct consequence of the Q6 ruling; I do not treat it as a stop, but ROOT should confirm before A.
- **Design items not implementable as literally written:** none found. Two literal readings are refined and declared: C's p = 53 accessor check holds for normal results only (§12.3 C), and K-D5's formation is private, so E's cross-check uses a test-only port (§12.3 E).
