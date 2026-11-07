//! The modal resource guard's combined preflight P0 (DEL-04-09 Design
//! §5.3.8, "Where each check sits"), and the budget form it is judged
//! against. **No weight, allotment or ceiling is adopted here** (Agent 0,
//! 2026-11-04): [`ModalResourceBudget`] has no `Default` and no constructor
//! with values; only tests build one, and it is labelled test-local.
//!
//! **What P0 is.** In `run_modal_dense`, after `prepare_modal` and before
//! the state case's solve and any n²-sized allocation, P0 forms a count c̄_t
//! for every cost term of §5.3.8's table from request, model and parameter
//! inputs only. **Every bounded c̄_t is an upper bound on the work it
//! charges, in its stated unit, derived from source for every admitted
//! input** (the derivations follow). A term for which no such bound is
//! derived is *unbounded*, and P0 refuses any request in which it occurs.
//! An allotment that a later stage would enforce (R_max at S2, L_A at S2b)
//! is never charged as if it were a bound: those stages are not built.
//! Then, in checked u128 arithmetic:
//! - W = Σ_t w_t·c̄_t, with w_t integer picoseconds per counted unit;
//! - the stage allotments A_s = Σ_{t∈s} w_t·c̄_t, for the stage s at which
//!   t's count becomes exact (S1 in the kernel; S2, S2b and the confirmation
//!   counter are not built, so their allotments are recorded and enforced by
//!   nothing yet);
//! - Σ_s A_s ≤ W_budget, inclusive.
//!
//! W is a calibrated weighted-work budget in nominal time units, not a
//! runtime guarantee. A count is in its own unit, never a calibrated flop
//! proxy; §5.3.8's w_b64 and w_screen units (19n³/3 + 6n·R, n³/3) are
//! leading-order calibration units, not upper bounds, and are not used.
//!
//! **Refusals**, each blocking and in the approved MODAL namespace:
//! - `MODAL_RESOURCE_PREFLIGHT_EXCEEDED`: a term is inadmissible by
//!   arithmetic (unbounded, or a nonzero count with an undetermined weight:
//!   the approved weight-or-refuse rule), naming every such term; or W
//!   exceeds W_budget, naming the dominant term, the largest w_t·c̄_t;
//! - `MODAL_RESOURCE_ARITHMETIC_OVERFLOW`: a count, a product or a sum
//!   overflows u128. Nothing wraps or saturates.
//!
//! **The capability boundary today.** T2's integrity work is unbounded and
//! occurs in every request (and T1's integrity and confirmation work in
//! every request with a state solve), so **P0 refuses every budgeted
//! request**, naming the unbounded terms. That is the approved
//! weight-or-refuse boundary, stated plainly: P0 admits nothing until those
//! terms are bounded. A request without a budget is unaffected.
//! `T3.filters`, `T7.assembly` and `T9.construction` are bounded (below):
//! their std binary searches, sorts and maps were replaced by
//! repository-owned operations (modal_eigen `support`) whose step counts
//! are derived from source.
//!
//! **Dimensions** (§5.3.8's symbols; each an upper bound from source):
//! - N, the nodes, and n_g = 6N.
//! - n̄_f, the pencil order. `reduce_assembled_sparse_system` (frame_kernel
//!   `structural/sparse.rs:851`) takes the free DOFs as exactly the
//!   complement in 0..n_g of the restrained set it is given, and refuses an
//!   out-of-range or repeated DOF. That set is the prepared linear one, or
//!   with nonlinear supports a superset of it (`state::linearize_modal_
//!   supports` only adds rigid states). So n_f ≤ n_g − |distinct linear
//!   restrained DOFs in 0..n_g|. With a restraint basis, n̄_f = n_g.
//! - n̄_c = n̄_f: the computed pairs are at most the condensed order.
//! - n̄_r = n̄_f: n_r ≤ n_c ≤ n_f. R_max (⌊R_max/6N⌋) is enforced only at S2,
//!   which is not built, so it is recorded and not used. n_r is never
//!   assumed ≤ `mode_count`: cluster completion can exceed it.
//! - p̄ = min(n̄_f, A_band): the attempted band pairs are among the condensed
//!   μ, and S1-band refuses more than A_band before any band work.
//! - R̄ = A_QL: S1-QL refuses before a sweep that would pass it.
//! - s = s_max: `sturm::run_candidates` takes at most `max_candidates`
//!   shifts in either placement, and one placement runs per request.
//! - L̄ = [`MODAL_RESOURCE_LIMB_BOUND`] limbs, derived below. L_A is enforced
//!   only at S2b, which is not built, so it is recorded and not used.
//! - D_K = Σ_K d_e², D_M = Σ_M d_e², E: from the re-formation's element
//!   sizes (`reform_element_stiffness`/`_mass`, modal_eigen
//!   `reformation/mod.rs:263`, `:357`): 12 × 12 for straights, arcs,
//!   connectors and tie rods; an arc's mass two 6 × 6 lumps; a body 6 × 6;
//!   an axis spring 1 × 1; a skewed spring 3 × 3. Over every built pipe,
//!   arc, connector, tie rod and spring, and one body per mass-bearing pipe
//!   segment (`mass::assemble_mass` keys bodies by pipe):
//!   D_K ≤ 144·(pipes + arcs + connectors + tie rods) + springs + 9·skewed;
//!   D_M ≤ 144·pipes + 72·arcs + 36·segments; E ≤ pipes + arcs + segments +
//!   connectors + tie rods + springs + skewed.
//! - μ, the exact-product calls per element entry: 1 without a restraint
//!   basis (one `add_dd_product`), 36 with one (`product_into` on a mixed
//!   element: ≤ 3 × 3 coordinate pairs × 2 dd words × 2 calls for a factor).
//!   A call is at most 8 parts (`add_product4_within`, four factors), so
//!   Q = 8μ parts per element entry; a re-formed position per element entry
//!   is at most μ.
//! - ν, the assembly's position visits per element entry, and θ, its
//!   traversal steps per element entry (modal_eigen `reformation`
//!   `for_each_position`): without a restraint basis every element is plain,
//!   at most d(d + 1)/2 ≤ d² visits and 1 + d + d(d + 1)/2 ≤ 3d² steps, so
//!   ν = 1 and θ = 3; with one an element may be mixed, at most 9d² visits
//!   (≤ 3 coordinates per DOF) and 1 + d + d² + 3d² + 9d² ≤ 15d² steps, so
//!   ν = 9 and θ = 15.
//! - F̄ = P = n̄(n̄ + 1)/2 entries in each formed list (`stiffness.rs`): K's
//!   list is one entry per stored column c ≤ r of each free row r of a
//!   compressed matrix whose rows are sorted and deduplicated (frame_kernel
//!   `structural/sparse.rs`), mapped by the injective free numbering; M's
//!   is one entry per stored (row ≤ col) DOF pair of the upper node blocks
//!   (keyed (i, j), i ≤ j), or with a restraint basis one per pair of the
//!   blocks' coordinates (each node pair once): each a distinct lower
//!   position, so at most P each, and L̄ = 2P together.
//!
//! **L̄, the limb span of every `ExactSum`** (modal_eigen `exact.rs`). An
//! add of a part p at scale σ touches the three limbs from ⌊(e(p) + σ −
//! 52)/32⌋ up (reserving a fourth), and `low` only decreases; the length never shrinks; a carry pass
//! pushes a limb only while the top limb is ≥ 2³¹ in magnitude. So for an
//! accumulator whose parts have representation bits in [B_lo, B_hi] and
//! fewer than 2^c adds, the length is at most
//! ⌊(B_hi + c)/32⌋ + 4 − ⌊B_lo/32⌋ + 1.
//! - *Parts.* A product of binary64 factors x_i (e_i ∈ [−1074, 1023]) is
//!   split into exact TwoProduct parts. Each part is a nonzero piece of the
//!   exact product, so its bits lie at or above the product's lowest bit,
//!   ≥ Σ(e_i − 52), and its representation lsb is at most 52 below that;
//!   the product is below 2^(Σe_i + k), so a part's top bit is ≤ Σe_i + k.
//! - *Level 1* (sums of products of binary64 values and dd words): at most
//!   five binary64 factors per product (`product_into` on a mixed element
//!   with a factor: q_a, q_b, the word, λ, x). B_lo = 5·(−1126) − 52 =
//!   −5682; B_hi = 5·1024 = 5120.
//! - *Level 2* (`add_terms_product`: the terms of a level-1 sum times at
//!   most two binary64 factors; `reformation/mod.rs:1417`,
//!   `orthogonality.rs:176`; the formation difference adds level-1 terms
//!   directly, `reformation/mod.rs:1197`): a term is an integer limb below
//!   2³² at 2^(32(low₁ + k)), so B_lo = 32·low₁ − 2·1126 − 52 and B_hi =
//!   32·(top₁ + 1) + 2·1024. No level-2 sum's terms are multiplied again.
//! - *Adds:* every add is a part of a call counted in a term here (≤ 8 parts
//!   a call), and P0 refuses a count ≥ 2¹²⁸, so fewer than 2¹³¹ < 2^c adds,
//!   c = 132.
//! - Hence L̄₁ = 168 + 178 + 1 = 347 and L̄ = L̄₂ = 241 + 250 + 1 = 492 limbs,
//!   for every accumulator; terms() of a sum is at most its length.
//! - *Limb operations:* one part's add moves at most 5L̄ + 16 limbs (a
//!   downward growth: reallocation, resize, shift and fill; or an upward
//!   resize; three chunk updates; the carry pass every 2²⁸ adds, amortized),
//!   and one read (`to_wide_with`, `terms_into`) at most 12L̄ + 32 (copy,
//!   two carry passes, negation, trim, scan and the term list).
//!
//! **The terms** (counted unit; derivation; stage). n = n̄_f throughout.
//! - T1, only with nonlinear supports (the state case's solve):
//!   S̄ = (I_as + 1)·(2 + d̄) + 1 solves. I_as is the product's active-set
//!   cap (`DEC_046_PRODUCT_PREVIEW_ACTIVE_SET_MAX_ITERATIONS`, lib.rs:197);
//!   dense mode solves once per iteration without derived friction and
//!   2 + d with it (nonlinear_integration `lib.rs:1639`–`:1795`); the
//!   all-active recovery trial of iteration 1 adds one more step; the +1 is
//!   the gate factorization. d̄ is the friction supports (`:2131`). Each
//!   solve factors through frame_kernel's `cholesky` (`structural.rs:2063`,
//!   via `solve_structural_dense_binary64`/`factor_structural_cholesky`).
//!   - `T1.factor` = S̄ × T2.factor's count; `T1.entries` = S̄ × T2.entries'
//!     (P0);
//!   - `T1.coupling` = (I_as + 1)·(⌈2d̄³/3⌉ + 2d̄² + d̄·n_g) flops and entry
//!     steps: `solve_dense`, the influence and coupling matrices and the
//!     unit-force vectors (P0);
//!   - `T1.integrity`: **unbounded**, each solve's M03 integrity work, as
//!     T2.integrity;
//!   - `T1.confirmation`: **unbounded**, the static confirmation's
//!     evaluations E_conf, whose A_conf counter is a held NI seam.
//! - T2, the static screen (`stiffness.rs`, the dense structural solve):
//!   - `T2.factor` = (n − 1)n(n + 1)/6 Cholesky inner iterations, exactly
//!     the k-loop of `cholesky` (`structural.rs:2089`–`:2095`), each one
//!     checked product, one checked subtraction and one checked absolute
//!     addition (P0);
//!   - `T2.entries` = 3n² entry steps: n(n + 1)/2 factor entries (each one
//!     read and a checked division, or a square root with its pivot
//!     screen), the n² finiteness scan, the n(n − 1)/2 symmetry scan and the
//!     n² factor allocation (P0);
//!   - `T2.integrity`: **unbounded.** The M03-INTEGRITY work around the
//!     factor: the `SparseAssemblyEvidence` construction, the dense copy and
//!     symmetry view, `prepare_bound`'s radix scaling, exact right-hand
//!     sides (`ExactAccumulator`) and skew audit, `audit_contributions`
//!     (`Expansion` sums and differences, each ≤ 2,099 components since its
//!     parts are nonoverlapping, zero-free binary64), the triangular solves,
//!     residual records, condition estimate and finish. Its pass counts are
//!     not derived in this slice.
//! - `T3.kernel`: kernel steps of `solve_dense_pencil_with_allotments`
//!   (modal_eigen `dense.rs:358`), item by item in
//!   [`modal_resource_kernel_steps`] (S1). Band recovery is T4; the
//!   massive-DOF complements are `T3.filters`.
//!   - **The unit.** A kernel step is at most 64 operations. Operations are
//!     counted literally: every read of a named value (a variable,
//!     parameter, field or element), every arithmetic operation (+, −, ×, ÷,
//!     √), absolute value, negation and logical `!`, every comparison or
//!     predicate (`is_finite`, `is_nan`, `is_normal`, `&&`, `||`, a match or
//!     `?` test; `max`/`min` count a comparison and a select), and every
//!     binding or assignment (`let`, `=`, compound assignment, each pattern
//!     element, each parameter bound at a call, each returned value, each
//!     constructed field or `push`). Excluded, and only these: loop
//!     control, index arithmetic (an index variable read only to subscript
//!     or slice), bounds checks and `debug_assert`s. Where a count was
//!     unclear it is rounded up. A body of k operations is charged ⌈k/64⌉
//!     steps.
//!   - **Premises (not repository source).** A core slice or `Vec` element
//!     primitive (`copy_within`, `fill`, `swap`/`swap_with_slice`,
//!     `resize`, iterator adapters, `push` with amortized growth) is one
//!     step per element, at the per-element counts below. Allocator calls
//!     are charged in no term. `f64::sqrt` is one operation (IEEE 754's
//!     correctly rounded square root).
//!   - **The ledger** (`dense.rs` unless stated; operations per line, total,
//!     steps). `get` (`matrix.rs:96`–`:98`): 3 parameters, :97 2 reads,
//!     `>=`, 2 reads, 2 binds, :98 a read and the return = 12.
//!     `max_propagating_nan` (`dd.rs:565`–`:570`): 2 parameters, :566 2
//!     reads, 2 predicates, `||`, :569 2 reads, `max` 2, return = 12.
//!
//!     | Body | Lines | Per line | Total | Steps |
//!     |---|---|---|---|---|
//!     | `hypot` (inlined) | :1143–:1150 | :1143 params 2; :1144 2 reads, 2 abs, 2 binds = 6; :1145 2 reads, `>=`, 2 reads, 2 binds = 7; :1146 read, `==` = 2; :1149 2 reads, ÷, bind = 4; :1150 3 reads, ×, +, √, ×, return = 8 | 29 | — |
//!     | QL iteration | :1305–:1333, :1357–:1360 | :1305 3; :1308 35 (4 reads, `m − l`, `charge_ql_rotations` 28, `?`, assign); :1309 3; :1310 3; :1312 2; :1313 3 reads, −, ×, ÷, bind = 7; :1314 read, `hypot` 29, bind = 31; :1315 2; :1316 3; :1318 6; :1319 6; :1320 2; :1321 4; :1325 4; :1327 2; :1328 1; :1329 2; :1330 2; :1331 2; :1332 1; :1333 1; :1357 6 reads, negation, 4 ×, ÷, assign = 13; :1358 4; :1359 4; :1360 3 reads, abs, ×, `<=` = 6 | 149 | 3 |
//!     | `charge_ql_rotations` (inlined) | :715–:732 | params 3; :720 2; :723 read, `try_from` 2, `map_err` 1, `?`, bind = 6; :724 2 reads, +, check, `ok_or` 2, `?`, bind = 8; :725–:731 read, match, bind, 2 reads, `>`, read, `Ok`, return = 9 | 28 | — |
//!     | QL shift element | :1323 | 2 reads, −, assign | 4 | 1 |
//!     | QL rotation scalar | :1335–:1350 | :1335 2; :1336 2; :1337 2; :1338 2 reads, ×, bind = 4; :1339 4; :1340 2 reads, `hypot` 29, assign = 32; :1341 4; :1342 4; :1343 4; :1344 4 reads, ×, ×, −, assign = 8; :1345 6 reads, 5 operations, store = 12; :1348 (debug) 3; :1350 3 reads, +, `row_pair` (:1136–:1140: params 4, read, split, 2 binds, 2 reads, 2 fields, return = 13), 2 binds = 19 | 100 | 2 |
//!     | QL row update | :1352–:1354 | :1352 2; :1353 4 reads, ×, ×, +, store = 8; :1354 8 | 18 | 1 |
//!     | QL per-l body | :1297–:1298, :1302–:1303, :1365–:1366 | :1297 3 reads, 2 abs, +, `max` 2, assign = 9; :1298 2; :1302 3; :1303 1; :1365 4; :1366 1 | 20 | 1 |
//!     | QL scan | :1299–:1300 | :1299 2 reads, −, `<`, 3 reads, abs, ×, `>`, `&&` = 11; :1300 3 | 14 | 1 |
//!     | QL e shift | :1284 | read, assign | 2 | 1 |
//!     | Householder d init / final | :1165; :1252–:1253 | 2; 2 + 1 | ≤ 3 | 1 |
//!     | Householder per-row block | :1168–:1169, :1173, :1185–:1192, :1207, :1212, :1226 | 1, 1, 2, 2, 3, 2, 3, 4, 6, 4, 1, 6, 2 | 37 | 1 |
//!     | Householder scale / zero / normalize | :1171; :1176–:1178; :1182–:1183 | 5; 2 + 1 + 1; 4 + 6 | ≤ 10 | 1 |
//!     | Householder e zero / w outer / w inner | :1194; :1198–:1200, :1205; :1202–:1203 | 1; 2 + 2 + 6 + 2; 6 + 6 | ≤ 12 | 1 |
//!     | Householder f / hh / update outer / inner | :1209–:1210; :1214; :1217–:1223; :1220 | 4 + 6; 6; 2 + 2 + 2 + 1; 5 reads, 2 ×, +, −, store = 10 | ≤ 10 | 1 |
//!     | Householder accumulation | :1230–:1233; :1235; :1238; :1240; :1243; :1248 | 2 + 1 + 2 + 2; 4; 1; 6; 6; 1 | ≤ 7 | 1 |
//!     | `cholesky_ordered` entry | :1011–:1043 | `k_ij(j)` (bind, read, 2 reads of `order`, `get` 12, return) and bind = 18; `i == j` 3; `!(s > 0)` 3; ratio: read, `k_ij(i)` 17, ÷, bind = 20; `<` 3; assign 2; √ store 3 (off-diagonal: 18 + 3 + 4) | 52 | 1 |
//!     | `cholesky_ordered` inner / row closure | `s -= r·r`; `k_ij` capture | 3 reads, ×, −, assign = 6; 2 | ≤ 6 | 1 |
//!     | census entry | :959–:1009 | `equilibrated(a, b)` (2 params, `==` with 2 reads, read, 2 reads of `kept`, `get` 12, 2 reads, 2 ÷, return) and bind = 26; `a == b` 3; pivot test 3 reads, 2 comparisons, `&&`, `!` = 7; √ store 3 | 39 | 1 |
//!     | census inner / root | `s -= r·r`; read, `get` 12, bind, `!(d > 0)` 3, √ push 3 | 6; 20 | ≤ 20 | 1 |
//!     | `forward_solve_row` / `back_solve_block` / `back_transform` row | :1045–:1086 | slice 2, read and bind 2, read, read, ÷, store 4 | 8 | 1 |
//!     | the same, inner | | 3 reads, ×, −, assign | 6 | 1 |
//!     | `check_dense` entry | `admission.rs:91`, :61–:72 | 4 reads, `?`, `get` 12, `check_entry` (4 params, read, `admitted`: param, 3 reads, `==`, 2 abs, `>=`, `<=`, `&&`, `||`, return = 12, test, `Ok`) = 19 | 36 | 1 |
//!     | massless entry / row | :943 | param, read, `get` 12, `==`, return = 16; param, test, push = 3 | ≤ 16 | 1 |
//!     | `check_finite` / validate elements | :915; :925, :929 | param, read, predicate, `!`, return = 5; 8; 5 | ≤ 8 | 1 |
//!     | reduce fill element / row | :1092–:1094 | param, read, `get` 12, return, push = 16; bind, extend = 2 | ≤ 16 | 1 |
//!     | forward-solve call | :1099, :1103 | 4 params, chunk bind | 5 | 1 |
//!     | transpose / symmetrize | :1121; :1113 | read, `swap` 4 = 5; 2 | ≤ 5 | 1 |
//!     | `swap_rows` call / element | :1126–:1134 | 4 params, 3, 2 reads, `<`, 4, read, split, 2 binds, call = 20; 4 | ≤ 20 | 1 |
//!     | sort outer / comparison | :1377–:1387; :1379–:1383 | 2 + 2 + 3 + 5 + `swap_rows` 20 = 32; 2 binds, 3, 2, 2 = 9 | ≤ 32 | 1 |
//!     | root / y map / map out | :462; :465; :468 | param, read, `get` 12, √, return, push = 17; 2 binds, 2 reads, ×, return, push = 7; 7 | ≤ 17 | 1 |
//!     | estimator elements | `norm_estimate.rs` | wrapper predicate 4; `norm1` 7; signs 6; argmax 9; `e` 1; `==` 3; alternative 11; `x` 1 | ≤ 11 | 1 |
//!     | mass-norm element / row | :474; :470–:476 | 2 binds, read, `get` 12, abs, 2 reads, 2 ÷, return, + and bind 2 = 23; 3 binds, bind, `max_propagating_nan` 12, return = 17 | ≤ 23 | 1 |
//!     | reorder elements | :508–:522 | count 6; reverse 4; swap 4; resize 1; `copy_within` call 5, element 2; allocation 1 | ≤ 6 | 1 |
//!     | back-transform row / scatter | :523–:530 | chunk bind, `fill` 2, `copy_from_slice` 2, 3 params = 8; 2 binds, read, store = 4; fill / copy elements ≤ 2 | ≤ 8 | 1 |
//!     | screen count element | :679–:685 | bind, 3 reads, `classify_mu` 10 (:656–:663: 3 params, 2 reads, `<=`, 2 reads, `<=`, return), match, 2 binds, 2 reads, `checked_count_add` 10, `?`, assign | 31 | 1 |
//!     | screen body (band path) | :557–:581 | :557 2; :558 3 reads, `classify_mu` 10, `==` = 14; :562 2; :563 15; :566 10; bind 1; :568–:570 35 (`screen_passes` :882–:889 23 inlined); :571 2; :572 2; :573 5; :575–:581 18 | 106 | 2 |
//!     | below-threshold filter / retained / sort | :588; :590–:595; :601–:611 | 5; 2 binds, 3, 2 = 7, copy element 2; outer 2 + 3 + 5 + 20 = 30, comparison 5, swap element 4 | ≤ 30 | 1 |
//!     | output elements | :618; :923–:934 | param, read, ÷, return, push = 5; ≤ 8 | ≤ 8 | 1 |
//!     | thresholds | :478–:507, :537–:538 | :478–:482 11; :486 6; :487–:488 9; :489–:492 19; :493–:496 17; :497–:507 50 (10 fields, 10 binds, 5 × 6 tests); :537 7; :538 10 | 129 | 3 |
//!     | other straight-line code | :364–:454, :460–:470, :508–:522, :532–:555, :588–:598, :601, :615–:650, and the helpers' prologues | per block ≈ 12, 16, 20, 40, 2, 6, 7, 1, 3, 13, 8, 20, 8, 6, 2, 40, 8, 2, 20, 9, 40, 3, 1, 8, 2, 30, 8, 1, 25, 51 | ≈ 412 | 12 (768) |
//!     | Hager scalar control and per-solve calls | `norm_estimate.rs:18`–`:93`, :464–:469 | start ≈ 12; ≤ 4 iterations × ≈ 40; close ≈ 21; 11 solves × (call ≈ 12, `solves += 1` 3, filter 2) ≈ 187 | ≈ 380 | 8 (512) |
//!
//!   - R enters as R̄(2n + 5). The iterations are at most the rotations (each
//!     sweep has at least one rotation). Each iteration is 3 steps plus its
//!     shift loop (≤ n), and each rotation 2 steps plus its row update (n).
//! - `T3.filters` = 2(n + 1) support steps (S1; 0 at n = 0, refused before
//!   the filters). The kernel's and the census's massive-DOF lists are
//!   `support::ascending_complement` of the structurally massless DOFs, n + 1
//!   steps each (one per index and the call; a body is at most 9
//!   operations), where they were std `slice::binary_search` filters. A
//!   *support step* is `T3.kernel`'s step (at most 64 operations, counted
//!   literally) under the same premises; an `ExactSum` call is one
//!   operation in it (its limb work is the `.limbs` terms).
//! - T4, band recovery (`dense.rs:759`–`:880`; S1): `T4.scaling` = 2n² + n
//!   WideDd entry steps when p̄ > 0; `T4.visits` = 4p̄n² exact-product calls
//!   (2n² in `recover_band_mode`, 2n² in `formed_pair_errors`);
//!   `T4.insertions` = 4p̄n² (each call may insert); `T4.rows` = p̄(n + 2)
//!   WideDd row steps; `T4.limbs` = p̄·(16n²·(5L̄+16) + (n + 2)·(12L̄+32))
//!   limb operations.
//! - T5 (S2): `T5.census` = 16·n̄_c steps of linear passes over the computed
//!   pairs (the eigenvalue copy, `cluster_ids`, frequencies,
//!   `reported_set`, estimates, the evidence rows, the cut-off
//!   disclosures); `T5.outcome` = n̄_c·p̄ + 2n̄_c² + n̄_c steps of the
//!   outcome's band flags and cluster ranges (`outcome.rs`).
//! - T6 (S2): `T6.visits` = n̄_r(n² + 3n) exact-product calls (one
//!   `mass_bilinear` and three `wide_dot`s per mode,
//!   `modal_quantities.rs:591`, `:625`); `T6.limbs` =
//!   n̄_r·(4(n² + 3n)·(5L̄+16) + 4·(12L̄+32)).
//! - T7 (S2): `T7.elements` = E element re-formations (fixed-size p128 work
//!   each); `T7.assembly` = [`modal_resource_assembly_steps`] support steps,
//!   every step of the re-formed pencil's assembly and of the formation
//!   difference other than the element re-formations and the limb work:
//!   the two matrices are each assembled twice (`ReformedPencil::new_*` and
//!   `formation_difference`). Per matrix X, with D_X its element squares,
//!   v_X ≤ ν·D_X position visits, u_X ≤ min(v_X, P) re-formed positions and
//!   F_X ≤ P formed entries (modal_eigen `reformation`, each count stated
//!   there):
//!   - each assembly, 3t + 12v + 4n + 3 with t ≤ θ·D_X traversal steps
//!     (three traversals, two stable counting sorts, the slots), where it
//!     was a `BTreeMap`;
//!   - the pencil's range check, 5 per position (`to_dd` with at most five
//!     scaling iterations each way, the level-1 span, and `check_entry`:
//!     at most 230 operations), and on a refusal the element search, at
//!     most 7·D_X;
//!   - `formed_lower`, 9F + 4n + 3 (a stable counting sort where it was
//!     `sort_unstable_by_key`);
//!   - `difference`, 27F + 50u + u·L̄ + 14n + 3: the accumulators, the two
//!     merge walks (where it binary-searched), one step per term added (at
//!     most L̄ terms per read), and `scaled_norm`'s WideDd arithmetic at 24
//!     steps per entry (literally about 1,253 operations: two `from_dd`, two
//!     `mul`, `abs` and two `add`) and 7 per row (`to_normal_or_zero` with
//!     at most six scaling iterations: about 265);
//!   - once, K's diagonals by a merge walk, n + F_K, and their scaling at 16
//!     steps each (`Dd::div` and `Dd::sqrt`: about 632 operations), with
//!     the scaling's fill n, and 4 for the calls.
//!
//!   So `T7.assembly` = Σ_X (6t_X + 24v_X + 7D_X + 36F_X + (55 + L̄)u_X +
//!   26n + 12) + 18n + F_K + 4, charged at t_X = θD_X, v_X = νD_X, u_X =
//!   min(νD_X, P) and F_X = P:
//!   (6θ + 24ν + 7)(D_K + D_M) + (55 + L̄)(min(νD_K, P) + min(νD_M, P)) +
//!   73P + 70n + 28. The operation figures above are literal counts under
//!   the step convention, rounded up to whole steps with a margin; they
//!   are a numerical-resource claim for independent review.
//!   `T7.limbs` follows those same consumers, not a single assembly:
//!   - `new_on_coordinates` assembles each matrix and reads each sum once
//!     for its range check;
//!   - `formation_difference` assembles each matrix again, reads each sum
//!     once for matching/absent entries and once for its re-formed norm;
//!   - every formed entry receives its value and is read for the difference
//!     norm; matching sums contribute at most L̄ parts each.
//!   Thus the insertions are bounded by 2Q(D_K + D_M) + n(n + 1) +
//!   L̄·μ(D_K + D_M), and reads by 3μ(D_K + D_M) + n(n + 1).
//!   Each insertion costs at most 5L̄+16 limb operations and each read
//!   12L̄+32. Element re-formation itself is still once per operand:
//!   the assembly traversals reuse already formed entries, so `T7.elements`
//!   does not acquire a second element-reformation charge.
//! - T8, residuals of every computed pair (S2b): `T8.insertions` =
//!   n̄_c·μ(D_K + 2D_M) calls (K̂x, λM̂x, exact M̂x); `T8.generalized_mass` =
//!   n̄_c·n·L̄ calls; `T8.rows` = n̄_c(2n + 1) WideDd row steps; `T8.limbs` =
//!   n̄_c·((5L̄+16)·(Q(D_K + 2D_M) + 4nL̄) + (12L̄+32)·(2n + 1)).
//! - T9, Sturm evidence at the dense profile, P = n(n + 1)/2 (S2):
//!   `T9.construction` = [`modal_resource_construction_steps`] support steps
//!   for `ShiftPencil::from_entries` (modal_eigen `sturm`), once per
//!   request: 13L + 9n + 5P + 12 for L ≤ L̄ = 2P input entries (the checks,
//!   three stable counting sorts where it was a std `sort_by_key`, the
//!   union pattern, the scaling and row sums) and the reverse Cuthill–McKee
//!   order (`support::reverse_cuthill_mckee_lower`, repository-owned where it
//!   was `sparse_direct`'s, whose two std sorts per adjacency list had no
//!   source constant): 15n + 8q + 3 + (n + 3)(5n + 2q) + 3n(n + 1) with q ≤
//!   P − n off-diagonal positions. The traversals are counted with their
//!   sizes: per component of m nodes and h half-edges, at most m + 2
//!   pseudo-peripheral traversals (reachability, the first candidate, at
//!   most m − 1 accepted since the eccentricity strictly grows below m, and
//!   one refused) and the final one, each at most 3m + h + 2 steps, and the
//!   least-node scans at most (m + 1)(m + 2); `T9.factorization` =
//!   s·(n(n − 1)(n − 2)/6 + n²) LDLᵀ steps (`factorize_ldlt`,
//!   `sparse_direct/src/lib.rs:408`: per row of width w ≤ i,
//!   w(w − 1)/2 + 2w + 1); `T9.passes` = s·(29P + 98n + 70) steps (the
//!   formed list, profile build, factor clone and observations, ≤ 11
//!   ordered solves of 2P + 8n, separations, gap positions);
//!   `T9.neighbours` = 2(P + 3n) calls and checks; `T9.limbs` =
//!   2·((5L̄+16)·8P + 2(12L̄+32)).
//! - T10, orthogonality of the published set (S2b): `T10.products` =
//!   n̄_r·μ·D_M calls; `T10.bilinear` = n̄_r(n̄_r + 1)/2·n·L̄ calls;
//!   `T10.limbs` = n̄_r((5L̄+16)·Q·D_M + (12L̄+32)·n) + n̄_r(n̄_r + 1)/2·
//!   ((5L̄+16)·4nL̄ + 12L̄+32).
//!
//! **Not charged here:** T11, the projection (it runs in the document path
//! after `run_modal_dense`, and its count needs B_doc, still proposed), and
//! work outside §5.3.8's table (K's formation, the mass assembly, the
//! pencil's n² fill, the outcome's per-mode O(n_g) publication, and
//! modal_eigen `ModalBasis::new`'s duplicate-DOF check, which is a std
//! `BTreeSet` and so has no source constant either). **Memory** is an open
//! item: the option-(c) ledger's rows still carry O(·) terms and
//! approximate constants, so no memory check is made. The support
//! operations' capacities are stated where they are defined (modal_eigen
//! `support`, `reformation`, `sturm`): each vector is allocated once at a
//! length or capacity counted before it and never grows, by std's
//! infallible allocation (an allocation failure aborts and a capacity
//! overflow panics, as before); none of them is retained past its call
//! except its result.
use super::integrity::{ModalIntegrityParameters, MODAL_RESOURCE_ARITHMETIC_OVERFLOW, MODAL_RESOURCE_PREFLIGHT_EXCEEDED};
use super::prepare::Prepared;
use crate::{diag, Diagnostic, DOF_PER_NODE};
use open_pipe_stress_modal_eigen::dense::DenseAllotments;
use open_pipe_stress_nonlinear_supports::NonlinearSupportBehavior;
use std::collections::BTreeSet;

/// The stage at which a term's count becomes exact (module docs).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum ModalResourceStage {
    /// Exact at P0 from its inputs.
    Preflight,
    /// The kernel's S1 (built): A_QL and A_band.
    Kernel,
    /// T1's confirmation counter (A_conf; not built, a held NI seam).
    Confirmation,
    /// S2, after the reported set (not built).
    ReportedSet,
    /// S2b, after the re-formation (not built).
    Reformed,
}

/// The stages in the order of [`ModalResourcePreflight::stage_allotments`].
pub(crate) const MODAL_RESOURCE_STAGES: [ModalResourceStage; 5] = [
    ModalResourceStage::Preflight,
    ModalResourceStage::Kernel,
    ModalResourceStage::Confirmation,
    ModalResourceStage::ReportedSet,
    ModalResourceStage::Reformed,
];

/// One cost term of P0 (module docs). The discriminant indexes
/// [`ModalResourceBudget::weights`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ModalResourceTerm {
    T1Factor,
    T1Entries,
    T1Coupling,
    T1Integrity,
    T1Confirmation,
    T2Factor,
    T2Entries,
    T2Integrity,
    T3Kernel,
    T3Filters,
    T4Scaling,
    T4Visits,
    T4Insertions,
    T4Rows,
    T4Limbs,
    T5Census,
    T5Outcome,
    T6Visits,
    T6Limbs,
    T7Elements,
    T7Assembly,
    T7Limbs,
    T8Insertions,
    T8GeneralizedMass,
    T8Rows,
    T8Limbs,
    T9Construction,
    T9Factorization,
    T9Passes,
    T9Neighbours,
    T9Limbs,
    T10Products,
    T10Bilinear,
    T10Limbs,
}

/// The number of P0 terms.
pub(crate) const MODAL_RESOURCE_TERM_COUNT: usize = 34;

/// Every term, in index order.
pub(crate) const MODAL_RESOURCE_TERMS: [ModalResourceTerm; MODAL_RESOURCE_TERM_COUNT] = {
    use ModalResourceTerm as T;
    [
        T::T1Factor,
        T::T1Entries,
        T::T1Coupling,
        T::T1Integrity,
        T::T1Confirmation,
        T::T2Factor,
        T::T2Entries,
        T::T2Integrity,
        T::T3Kernel,
        T::T3Filters,
        T::T4Scaling,
        T::T4Visits,
        T::T4Insertions,
        T::T4Rows,
        T::T4Limbs,
        T::T5Census,
        T::T5Outcome,
        T::T6Visits,
        T::T6Limbs,
        T::T7Elements,
        T::T7Assembly,
        T::T7Limbs,
        T::T8Insertions,
        T::T8GeneralizedMass,
        T::T8Rows,
        T::T8Limbs,
        T::T9Construction,
        T::T9Factorization,
        T::T9Passes,
        T::T9Neighbours,
        T::T9Limbs,
        T::T10Products,
        T::T10Bilinear,
        T::T10Limbs,
    ]
};

impl ModalResourceTerm {
    /// The term's name in refusals and records (module docs).
    pub(crate) fn modal_resource_id(self) -> &'static str {
        use ModalResourceTerm as T;
        match self {
            T::T1Factor => "T1.factor",
            T::T1Entries => "T1.entries",
            T::T1Coupling => "T1.coupling",
            T::T1Integrity => "T1.integrity",
            T::T1Confirmation => "T1.confirmation",
            T::T2Factor => "T2.factor",
            T::T2Entries => "T2.entries",
            T::T2Integrity => "T2.integrity",
            T::T3Kernel => "T3.kernel",
            T::T3Filters => "T3.filters",
            T::T4Scaling => "T4.scaling",
            T::T4Visits => "T4.visits",
            T::T4Insertions => "T4.insertions",
            T::T4Rows => "T4.rows",
            T::T4Limbs => "T4.limbs",
            T::T5Census => "T5.census",
            T::T5Outcome => "T5.outcome",
            T::T6Visits => "T6.visits",
            T::T6Limbs => "T6.limbs",
            T::T7Elements => "T7.elements",
            T::T7Assembly => "T7.assembly",
            T::T7Limbs => "T7.limbs",
            T::T8Insertions => "T8.insertions",
            T::T8GeneralizedMass => "T8.generalized_mass",
            T::T8Rows => "T8.rows",
            T::T8Limbs => "T8.limbs",
            T::T9Construction => "T9.construction",
            T::T9Factorization => "T9.factorization",
            T::T9Passes => "T9.passes",
            T::T9Neighbours => "T9.neighbours",
            T::T9Limbs => "T9.limbs",
            T::T10Products => "T10.products",
            T::T10Bilinear => "T10.bilinear",
            T::T10Limbs => "T10.limbs",
        }
    }

    /// Why the term has no bound at P0 (module docs); `None` for a bounded
    /// term.
    pub(crate) fn modal_resource_unbounded_reason(self) -> Option<&'static str> {
        use ModalResourceTerm as T;
        match self {
            T::T1Integrity | T::T2Integrity => {
                Some("the static dense path's M03 integrity work (preparation, audits, residual records, condition estimate) is not bounded from source in this slice")
            }
            T::T1Confirmation => Some("the confirmation evaluations have no source bound, and their A_conf counter is a held NI seam"),
            _ => None,
        }
    }

    /// The stage at which the term's count becomes exact (module docs).
    pub(crate) fn modal_resource_stage(self) -> ModalResourceStage {
        use ModalResourceTerm as T;
        match self {
            T::T1Factor | T::T1Entries | T::T1Coupling | T::T1Integrity | T::T2Factor | T::T2Entries | T::T2Integrity => {
                ModalResourceStage::Preflight
            }
            T::T1Confirmation => ModalResourceStage::Confirmation,
            T::T3Kernel | T::T3Filters | T::T4Scaling | T::T4Visits | T::T4Insertions | T::T4Rows | T::T4Limbs => ModalResourceStage::Kernel,
            T::T5Census
            | T::T5Outcome
            | T::T6Visits
            | T::T6Limbs
            | T::T7Elements
            | T::T7Assembly
            | T::T7Limbs
            | T::T9Construction
            | T::T9Factorization
            | T::T9Passes
            | T::T9Neighbours
            | T::T9Limbs => ModalResourceStage::ReportedSet,
            T::T8Insertions | T::T8GeneralizedMass | T::T8Rows | T::T8Limbs | T::T10Products | T::T10Bilinear | T::T10Limbs => {
                ModalResourceStage::Reformed
            }
        }
    }
}

/// The level-1 limb span bound L̄₁ (module docs): ⌊(5120 + 132)/32⌋ + 4
/// − ⌊−5682/32⌋ + 1.
pub(crate) const MODAL_RESOURCE_LEVEL1_LIMBS: u128 = 347;
/// L̄, every `ExactSum`'s limb span bound (module docs): the level-2 span,
/// ⌊(B_hi + 132)/32⌋ + 4 − ⌊B_lo/32⌋ + 1 with B_lo = 32·(−178) − 2304 and
/// B_hi = 32·169 + 2048.
pub(crate) const MODAL_RESOURCE_LIMB_BOUND: u128 = 492;

/// The derivation of [`MODAL_RESOURCE_LEVEL1_LIMBS`] and
/// [`MODAL_RESOURCE_LIMB_BOUND`] (module docs), as arithmetic.
pub(crate) fn modal_resource_limb_spans() -> (i64, i64) {
    const BITS: i64 = 32;
    const CARRY: i64 = 132;
    let factor_low = -1074 - 52;
    let factor_high = 1024;
    let span = |low: i64, high: i64| (high + CARRY).div_euclid(BITS) + 4 - low.div_euclid(BITS) + 1;
    let (low1, high1) = (5 * factor_low - 52, 5 * factor_high);
    let level1 = span(low1, high1);
    let (low_limb1, top_limb1) = (low1.div_euclid(BITS), (high1 + CARRY).div_euclid(BITS) + 4);
    let (low2, high2) = (BITS * low_limb1 + 2 * factor_low - 52, BITS * (top_limb1 + 1) + 2 * factor_high);
    (level1, span(low2, high2))
}

/// The resource budget P0 and S1 are judged against (module docs). It has
/// no `Default` and no constructor with values: every field is the
/// caller's, and none is adopted. Only tests build one.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ModalResourceBudget {
    /// w_t, integer picoseconds per counted unit, indexed by
    /// [`ModalResourceTerm`]. `None` is an undetermined weight: under the
    /// weight-or-refuse rule a term with a nonzero count and no weight
    /// refuses.
    pub(crate) weights: [Option<u64>; MODAL_RESOURCE_TERM_COUNT],
    /// W_budget, picoseconds: Σ_s A_s must not exceed it.
    pub(crate) work_budget: u128,
    /// A_QL: the most implicit QL rotations (S1-QL, enforced).
    pub(crate) ql_rotation_allotment: u64,
    /// A_band: the most attempted band-recovery pairs (S1-band, enforced).
    pub(crate) band_pair_allotment: u64,
    /// L_A: a recorded allotment for S2b. **Not used by P0**: S2b is not
    /// built, so L is charged at the derived L̄.
    pub(crate) limb_allotment: u64,
    /// R_max: a recorded row allotment for S2. **Not used by P0**: S2 is not
    /// built, so n_r is charged at n̄_f.
    pub(crate) reported_row_allotment: Option<u64>,
}

impl ModalResourceBudget {
    /// The kernel's S1 allotments from this budget: A_QL and A_band.
    pub(crate) fn modal_dense_allotments(&self) -> DenseAllotments {
        DenseAllotments { ql_rotations: Some(self.ql_rotation_allotment), band_pairs: Some(self.band_pair_allotment) }
    }
}

/// The state solve's dimensions of T1 (module docs).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ModalStateDimensions {
    /// S̄, structural solves.
    pub(crate) solves: u128,
    /// d̄, derived-friction candidates per iteration.
    pub(crate) friction: u128,
    /// I_as + 1, the iterations with the recovery trial.
    pub(crate) iterations: u128,
}

/// The quantities the term counts are formed from (module docs). P0 fills
/// them with bounds; a test can fill them with a run's actual values and
/// evaluate the same formulas.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ModalResourceDimensions {
    /// n_g = 6N.
    pub(crate) global: u128,
    /// n_f.
    pub(crate) free: u128,
    /// n_c.
    pub(crate) computed: u128,
    /// n_r.
    pub(crate) reported: u128,
    /// p, attempted band pairs.
    pub(crate) band_pairs: u128,
    /// R, QL rotations.
    pub(crate) ql_rotations: u128,
    /// s, Sturm candidates.
    pub(crate) sturm_candidates: u128,
    /// L, limbs.
    pub(crate) limbs: u128,
    /// D_K = Σ_K d_e².
    pub(crate) stiffness_squares: u128,
    /// D_M = Σ_M d_e².
    pub(crate) mass_squares: u128,
    /// E, re-formation operands.
    pub(crate) element_operands: u128,
    /// μ, exact-product calls per element entry (1, or 36 with a
    /// restraint basis).
    pub(crate) coordinate_multiplier: u128,
    /// ν, the assembly's position visits per element entry (1, or 9 with a
    /// restraint basis).
    pub(crate) assembly_visits_per_square: u128,
    /// θ, the assembly's traversal steps per element entry (3, or 15 with a
    /// restraint basis).
    pub(crate) assembly_steps_per_square: u128,
    /// T1's dimensions; `None` without nonlinear supports (no state solve).
    pub(crate) state: Option<ModalStateDimensions>,
}

/// μ with a restraint basis (module docs).
const MODAL_RESOURCE_SKEW_PRODUCTS: u128 = 36;
/// ν and θ without and with a restraint basis (module docs).
const MODAL_RESOURCE_PLAIN_ASSEMBLY: (u128, u128) = (1, 3);
const MODAL_RESOURCE_SKEW_ASSEMBLY: (u128, u128) = (9, 15);

/// A resource overflow: the quantity that overflowed u128.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ModalResourceOverflow(pub(crate) &'static str);

fn modal_resource_mul(a: u128, b: u128, quantity: &'static str) -> Result<u128, ModalResourceOverflow> {
    a.checked_mul(b).ok_or(ModalResourceOverflow(quantity))
}

fn modal_resource_add(a: u128, b: u128, quantity: &'static str) -> Result<u128, ModalResourceOverflow> {
    a.checked_add(b).ok_or(ModalResourceOverflow(quantity))
}

/// The checked product of `factors`.
fn modal_resource_product(factors: &[u128], quantity: &'static str) -> Result<u128, ModalResourceOverflow> {
    factors.iter().try_fold(1u128, |p, &f| modal_resource_mul(p, f, quantity))
}

/// The checked sum of `parts`.
fn modal_resource_total(parts: &[u128], quantity: &'static str) -> Result<u128, ModalResourceOverflow> {
    parts.iter().try_fold(0u128, |s, &p| modal_resource_add(s, p, quantity))
}

/// ⌈log₂ x⌉, 0 for x ≤ 1.
fn modal_resource_ceil_log2(x: u128) -> u128 {
    if x <= 1 {
        0
    } else {
        u128::from(128 - (x - 1).leading_zeros())
    }
}

/// T3's kernel steps at order n with R rotations: each body of the module
/// docs' ledger, times its iterations (n_m, n_0, the positive and the kept
/// rows each ≤ n), times its steps. Every item is monotone in n. The
/// massive-DOF complements (`dense.rs:424`, `:969`) are `T3.filters` and
/// are not counted here. The total is
/// 5·n(n+1)/2 + 2·Σ_i Σ_{j≤i} j + 37·n(n−1)/2 + 3n·n(n−1)/2
/// + 3·Σ_{t<n} t² + 21n² + 148n + 23 + R(2n + 5).
pub(crate) fn modal_resource_kernel_steps(n: u128, rotations: u128) -> Result<u128, ModalResourceOverflow> {
    if n == 0 {
        return Ok(0);
    }
    let q = "T3.kernel";
    let p = |f: &[u128]| modal_resource_product(f, q);
    let n2 = p(&[n, n])?;
    let tri = p(&[n, n - 1])? / 2; // n(n − 1)/2
    let pairs = p(&[n, n + 1])? / 2; // n(n + 1)/2
    let cube6 = p(&[n - 1, n, n + 1])? / 6; // Σ_i Σ_{j≤i} j
    let squares = p(&[n - 1, n, 2 * n - 1])? / 6; // Σ_{t<n} t²
    let row_tri = p(&[n, tri])?;
    modal_resource_total(
        &[
            // check_dense on K and M: entries.
            p(&[2, pairs])?,
            // the massless scans (kernel, census): entries and rows.
            p(&[2, n2])?,
            p(&[2, n])?,
            // the order.
            n,
            // cholesky_ordered: allocation, row closures, inner, entries; check_finite(r).
            n2,
            n,
            cube6,
            pairs,
            n2,
            // reduce_mass: fill elements and rows; two forward passes: calls, row
            // bodies, inner; transpose; symmetrize; check_finite(work).
            n2,
            n,
            p(&[2, n])?,
            p(&[2, n2])?,
            p(&[2, row_tri])?,
            tri,
            tri,
            n2,
            // d, e allocation.
            p(&[2, n])?,
            // householder_tridiagonalize: d init and final, Σ_{i<n} (i² + 7i + 1) and
            // Σ_{t<n} (2t² + 3t + 1) = 3·Σt² + 10·Σt + 2(n − 1); the closing stores.
            p(&[4, n])?,
            p(&[3, squares])?,
            p(&[10, tri])?,
            // the transpose.
            tri,
            // implicit_ql: e shift, per-l bodies, check_finite(d); scans; per iteration
            // (≤ R) 3 steps and a shift loop ≤ n; per rotation 2 steps and a row of n.
            p(&[3, n])?,
            pairs,
            p(&[rotations, 2 * n + 5])?,
            // sort_ascending: outer bodies, comparisons, row-swap elements.
            n,
            tri,
            n2,
            // roots; ≤ 11 Hager solves of 2·tri + 10n element and body steps; the
            // once-only x and alternative vectors; the mass norm's elements and rows.
            n,
            p(&[11, p(&[2, tri])? + 10 * n])?,
            p(&[2, n])?,
            n2,
            n,
            // Hager's scalar control (8), the thresholds (3), the straight-line code (12).
            23,
            // reorder: count, reverse, swap calls, copy calls, allocation; swap, resize
            // and copy elements.
            p(&[5, n])?,
            p(&[3, n2])?,
            // back-transform: per row its call, fill and copy (n), row bodies (n),
            // scatter (n), inner (tri); check_finite(work).
            n,
            p(&[3, n2])?,
            row_tri,
            n2,
            // screen: counts (n), bodies at 2 steps (2n), two filters (2n), retained
            // bodies (n), sort outer bodies (n); retained copies and sort swaps; sort
            // comparisons.
            p(&[7, n])?,
            p(&[2, n2])?,
            tri,
            // census: roots, allocation, inner, entries.
            n,
            n2,
            cube6,
            pairs,
            // outputs: eigenvalues and the μ and λ checks; the vectors' check.
            p(&[3, n])?,
            n2,
        ],
        q,
    )
}

/// `T3.filters` (module docs): the two massive-DOF complements, n + 1
/// support steps each; 0 at n = 0, which the kernel refuses first.
pub(crate) fn modal_resource_filter_steps(n: u128) -> Result<u128, ModalResourceOverflow> {
    if n == 0 {
        return Ok(0);
    }
    modal_resource_mul(2, modal_resource_add(n, 1, "T3.filters")?, "T3.filters")
}

/// The sizes `T7.assembly` is counted from (module docs), per matrix [K, M]:
/// P0 fills them with bounds, a test with a run's actual sizes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ModalAssemblySizes {
    /// n, the pencil order.
    pub(crate) order: u128,
    /// D_X, the element squares.
    pub(crate) squares: [u128; 2],
    /// t_X, the traversal steps of one assembly.
    pub(crate) traversal: [u128; 2],
    /// v_X, the position visits of one assembly.
    pub(crate) visits: [u128; 2],
    /// u_X, the distinct re-formed positions.
    pub(crate) positions: [u128; 2],
    /// F_X, the formed entries.
    pub(crate) formed: [u128; 2],
    /// L̄, the most terms a read gives.
    pub(crate) limbs: u128,
}

/// `T7.assembly` in support steps (module docs):
/// Σ_X (6t_X + 24v_X + 7D_X + 36F_X + (55 + L̄)u_X + 26n + 12) + 18n + F_K + 4.
pub(crate) fn modal_resource_assembly_steps(sizes: &ModalAssemblySizes) -> Result<u128, ModalResourceOverflow> {
    let q = "T7.assembly";
    let p = |f: &[u128]| modal_resource_product(f, q);
    let n = sizes.order;
    let per_read = modal_resource_add(55, sizes.limbs, q)?;
    let matrix = |x: usize| -> Result<u128, ModalResourceOverflow> {
        modal_resource_total(
            &[
                p(&[6, sizes.traversal[x]])?,
                p(&[24, sizes.visits[x]])?,
                p(&[7, sizes.squares[x]])?,
                p(&[36, sizes.formed[x]])?,
                p(&[per_read, sizes.positions[x]])?,
                p(&[26, n])?,
                12,
            ],
            q,
        )
    };
    modal_resource_total(&[matrix(0)?, matrix(1)?, p(&[18, n])?, sizes.formed[0], 4], q)
}

/// `T7.assembly`'s sizes at P0's bounds (module docs): t = θD, v = νD,
/// u = min(νD, P), F = P.
pub(crate) fn modal_resource_assembly_bounds(dims: &ModalResourceDimensions) -> Result<ModalAssemblySizes, ModalResourceOverflow> {
    let q = "T7.assembly";
    let n = dims.free;
    let positions = modal_resource_mul(n, modal_resource_add(n, 1, q)?, q)? / 2;
    let squares = [dims.stiffness_squares, dims.mass_squares];
    let mut traversal = [0; 2];
    let mut visits = [0; 2];
    for x in 0..2 {
        traversal[x] = modal_resource_mul(dims.assembly_steps_per_square, squares[x], q)?;
        visits[x] = modal_resource_mul(dims.assembly_visits_per_square, squares[x], q)?;
    }
    Ok(ModalAssemblySizes {
        order: n,
        squares,
        traversal,
        visits,
        // min(νD, P), without a `min` call (§8.58's MODAL-FB name link).
        positions: visits.map(|v| if v < positions { v } else { positions }),
        formed: [positions; 2],
        limbs: dims.limbs,
    })
}

/// The reverse Cuthill–McKee order's support steps (module docs) on n nodes
/// with q off-diagonal positions: 15n + 8q + 3 + (n + 3)(5n + 2q) +
/// 3n(n + 1).
pub(crate) fn modal_resource_rcm_steps(n: u128, q: u128) -> Result<u128, ModalResourceOverflow> {
    let t = "T9.construction";
    let p = |f: &[u128]| modal_resource_product(f, t);
    let traversals = p(&[modal_resource_add(n, 3, t)?, modal_resource_add(p(&[5, n])?, p(&[2, q])?, t)?])?;
    let scans = p(&[3, n, modal_resource_add(n, 1, t)?])?;
    modal_resource_total(&[p(&[15, n])?, p(&[8, q])?, 3, traversals, scans], t)
}

/// `T9.construction` in support steps (module docs): `from_entries` on L
/// input entries with at most P union positions before and after its
/// retain (P ≥ n, the diagonals), 13L + 9n + 5P + 12, and the order on
/// q = P − n off-diagonal positions.
pub(crate) fn modal_resource_construction_steps(n: u128, entries: u128, positions: u128) -> Result<u128, ModalResourceOverflow> {
    let t = "T9.construction";
    let p = |f: &[u128]| modal_resource_product(f, t);
    let off_diagonal = positions.checked_sub(n).ok_or(ModalResourceOverflow(t))?;
    modal_resource_total(&[p(&[13, entries])?, p(&[9, n])?, p(&[5, positions])?, 12, modal_resource_rcm_steps(n, off_diagonal)?], t)
}

/// The count c_t of every term (module docs) from `dims`; `None` for a term
/// with no bound.
pub(crate) fn modal_resource_term_counts(
    dims: &ModalResourceDimensions,
) -> Result<[Option<u128>; MODAL_RESOURCE_TERM_COUNT], ModalResourceOverflow> {
    use ModalResourceTerm as T;
    let p = modal_resource_product;
    let t = modal_resource_total;
    let n = dims.free;
    let n2 = p(&[n, n], "n_f²")?;
    let positions = p(&[n, n + 1], "n_f(n_f + 1)")? / 2;
    let screen = p(&[n.saturating_sub(1), n, n + 1], "T2.factor")? / 6;
    let entries = p(&[3, n2], "T2.entries")?;
    let (l, mu) = (dims.limbs, dims.coordinate_multiplier);
    let part = t(&[p(&[5, l], "5L")?, 16], "limbs per part")?;
    let read = t(&[p(&[12, l], "12L")?, 32], "limbs per read")?;
    let q8 = p(&[8, mu], "Q")?;
    let (dk, dm) = (dims.stiffness_squares, dims.mass_squares);
    let pair_entries = t(&[dk, p(&[2, dm], "2·D_M")?], "D_K + 2·D_M")?;
    let element_entries = t(&[dk, dm], "D_K + D_M")?;
    let (c, r, s, b) = (dims.computed, dims.reported, dims.sturm_candidates, dims.band_pairs);
    let forms = p(&[r, r + 1], "n_r(n_r + 1)")? / 2;
    let mut counts = [Some(0u128); MODAL_RESOURCE_TERM_COUNT];
    let mut set = |term: T, value: u128| counts[term as usize] = Some(value);

    if let Some(state) = dims.state {
        set(T::T1Factor, p(&[state.solves, screen], "T1.factor")?);
        set(T::T1Entries, p(&[state.solves, entries], "T1.entries")?);
        let d = state.friction;
        let coupling = t(&[p(&[2, d, d, d], "2d³")?.div_ceil(3), p(&[2, d, d], "2d²")?, p(&[d, dims.global], "d·n_g")?], "T1.coupling")?;
        set(T::T1Coupling, p(&[state.iterations, coupling], "T1.coupling")?);
    }
    set(T::T2Factor, screen);
    set(T::T2Entries, entries);
    set(T::T3Kernel, modal_resource_kernel_steps(n, dims.ql_rotations)?);
    set(T::T3Filters, modal_resource_filter_steps(n)?);
    set(T::T4Scaling, if b > 0 { t(&[p(&[2, n2], "T4.scaling")?, n], "T4.scaling")? } else { 0 });
    set(T::T4Visits, p(&[4, b, n2], "T4.visits")?);
    set(T::T4Insertions, p(&[4, b, n2], "T4.insertions")?);
    set(T::T4Rows, p(&[b, n + 2], "T4.rows")?);
    let band_limbs = t(&[p(&[16, n2, part], "T4.limbs")?, p(&[n + 2, read], "T4.limbs")?], "T4.limbs")?;
    set(T::T4Limbs, p(&[b, band_limbs], "T4.limbs")?);
    set(T::T5Census, p(&[16, c], "T5.census")?);
    set(T::T5Outcome, t(&[p(&[c, b], "n_c·p")?, p(&[2, c, c], "2n_c²")?, c], "T5.outcome")?);
    let mode_calls = t(&[n2, p(&[3, n], "3n")?], "n² + 3n")?;
    set(T::T6Visits, p(&[r, mode_calls], "T6.visits")?);
    let mode_limbs = t(&[p(&[4, mode_calls, part], "T6.limbs")?, p(&[4, read], "T6.limbs")?], "T6.limbs")?;
    set(T::T6Limbs, p(&[r, mode_limbs], "T6.limbs")?);
    set(T::T7Elements, dims.element_operands);
    set(T::T7Assembly, modal_resource_assembly_steps(&modal_resource_assembly_bounds(dims)?)?);
    let formed = p(&[n, n + 1], "n(n + 1)")?;
    let reformed_positions = p(&[mu, element_entries], "μ(D_K + D_M)")?;
    let assembly_parts = t(
        &[p(&[2, q8, element_entries], "2Q(D_K + D_M)")?, formed, p(&[l, reformed_positions], "L·μ(D_K + D_M)")?],
        "T7.limbs",
    )?;
    let assembly_reads = t(&[p(&[3, reformed_positions], "3μ(D_K + D_M)")?, formed], "T7.limbs")?;
    set(T::T7Limbs, t(&[p(&[part, assembly_parts], "T7.limbs")?, p(&[read, assembly_reads], "T7.limbs")?], "T7.limbs")?);
    set(T::T8Insertions, p(&[c, mu, pair_entries], "T8.insertions")?);
    set(T::T8GeneralizedMass, p(&[c, n, l], "T8.generalized_mass")?);
    let pair_rows = t(&[p(&[2, n], "2n")?, 1], "2n + 1")?;
    set(T::T8Rows, p(&[c, pair_rows], "T8.rows")?);
    let pair_parts = t(&[p(&[q8, pair_entries], "Q(D_K + 2D_M)")?, p(&[4, n, l], "4nL")?], "T8.limbs")?;
    let pair_limbs = t(&[p(&[part, pair_parts], "T8.limbs")?, p(&[read, pair_rows], "T8.limbs")?], "T8.limbs")?;
    set(T::T8Limbs, p(&[c, pair_limbs], "T8.limbs")?);
    set(T::T9Construction, modal_resource_construction_steps(n, p(&[2, positions], "L")?, positions)?);
    set(T::T9Factorization, p(&[s, t(&[p(&[n, n.saturating_sub(1), n.saturating_sub(2)], "T9.factorization")? / 6, n2], "T9.factorization")?], "T9.factorization")?);
    set(T::T9Passes, p(&[s, t(&[p(&[29, positions], "29P")?, p(&[98, n], "98n")?, 70], "T9.passes")?], "T9.passes")?);
    set(T::T9Neighbours, p(&[2, t(&[positions, p(&[3, n], "3n")?], "T9.neighbours")?], "T9.neighbours")?);
    set(T::T9Limbs, p(&[2, t(&[p(&[part, 8, positions], "T9.limbs")?, p(&[2, read], "T9.limbs")?], "T9.limbs")?], "T9.limbs")?);
    set(T::T10Products, p(&[r, mu, dm], "T10.products")?);
    set(T::T10Bilinear, p(&[forms, n, l], "T10.bilinear")?);
    let shape_limbs = t(&[p(&[part, q8, dm], "T10.limbs")?, p(&[read, n], "T10.limbs")?], "T10.limbs")?;
    let form_limbs = t(&[p(&[part, 4, n, l], "T10.limbs")?, read], "T10.limbs")?;
    set(T::T10Limbs, t(&[p(&[r, shape_limbs], "T10.limbs")?, p(&[forms, form_limbs], "T10.limbs")?], "T10.limbs")?);

    for term in MODAL_RESOURCE_TERMS {
        let occurs = match term {
            T::T1Integrity | T::T1Confirmation => dims.state.is_some(),
            _ => true,
        };
        if occurs && term.modal_resource_unbounded_reason().is_some() {
            counts[term as usize] = None;
        }
    }
    Ok(counts)
}

/// The bounds P0 uses (module docs), from the prepared request, the
/// parameters and the budget's enforced allotments only (A_QL, A_band).
pub(crate) fn modal_resource_bound_dimensions(
    prepared: &Prepared,
    params: &ModalIntegrityParameters,
    budget: &ModalResourceBudget,
) -> Result<ModalResourceDimensions, ModalResourceOverflow> {
    let built = &prepared.built;
    let wide = |value: usize| value as u128;
    let nodes = wide(built.nodes.len());
    let global = modal_resource_mul(nodes, wide(DOF_PER_NODE), "n_g")?;
    let free = if built.skew.is_some() {
        global
    } else {
        let restrained: BTreeSet<usize> = prepared.boundary.restrained_dofs.iter().copied().filter(|&dof| wide(dof) < global).collect();
        global - wide(restrained.len())
    };
    let pipes = wide(built.pipes.len());
    let arcs = wide(built.curved_bend_elements.len());
    let connectors = wide(built.connectors.len());
    let tie_rods = built.connectors.iter().try_fold(0u128, |sum, c| modal_resource_add(sum, wide(c.tie_rods().len()), "tie_rods"))?;
    let springs = wide(prepared.boundary.springs.len());
    let skewed = wide(built.skew.as_ref().map_or(0, |basis| basis.springs().len()));
    let segments = wide(prepared.model.pipe_segments.len());
    let twelve = modal_resource_total(&[pipes, arcs, connectors, tie_rods], "elements")?;
    let stiffness_squares = modal_resource_total(&[modal_resource_mul(144, twelve, "D_K")?, springs, modal_resource_mul(9, skewed, "D_K")?], "D_K")?;
    let mass_squares = modal_resource_total(
        &[modal_resource_mul(144, pipes, "D_M")?, modal_resource_mul(72, arcs, "D_M")?, modal_resource_mul(36, segments, "D_M")?],
        "D_M",
    )?;
    let element_operands = modal_resource_total(&[twelve, segments, springs, skewed], "E")?;
    let state = if built.nonlinear_supports.is_empty() {
        None
    } else {
        let friction = wide(built.nonlinear_supports.iter().filter(|s| matches!(s.behavior, NonlinearSupportBehavior::Friction)).count());
        let iterations = modal_resource_add(wide(crate::DEC_046_PRODUCT_PREVIEW_ACTIVE_SET_MAX_ITERATIONS), 1, "I_as + 1")?;
        let per = modal_resource_add(friction, 2, "2 + d")?;
        let solves = modal_resource_add(modal_resource_mul(iterations, per, "S_T1")?, 1, "S_T1")?;
        Some(ModalStateDimensions { solves, friction, iterations })
    };
    let assembly = if built.skew.is_some() { MODAL_RESOURCE_SKEW_ASSEMBLY } else { MODAL_RESOURCE_PLAIN_ASSEMBLY };
    Ok(ModalResourceDimensions {
        global,
        free,
        computed: free,
        reported: free,
        band_pairs: free.min(u128::from(budget.band_pair_allotment)),
        ql_rotations: u128::from(budget.ql_rotation_allotment),
        sturm_candidates: wide(params.sturm_max_candidates),
        limbs: MODAL_RESOURCE_LIMB_BOUND,
        stiffness_squares,
        mass_squares,
        element_operands,
        coordinate_multiplier: if built.skew.is_some() { MODAL_RESOURCE_SKEW_PRODUCTS } else { 1 },
        assembly_visits_per_square: assembly.0,
        assembly_steps_per_square: assembly.1,
        state,
    })
}

/// What P0 admitted: the bounds, the counts, each term's charge, the stage
/// allotments A_s and W.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ModalResourcePreflight {
    pub(crate) dimensions: ModalResourceDimensions,
    pub(crate) counts: [u128; MODAL_RESOURCE_TERM_COUNT],
    /// w_t·c̄_t, ps (0 for a zero count with no weight).
    pub(crate) charges: [u128; MODAL_RESOURCE_TERM_COUNT],
    /// A_s, ps, in [`MODAL_RESOURCE_STAGES`] order.
    pub(crate) stage_allotments: [u128; 5],
    /// W = Σ_s A_s, ps.
    pub(crate) total: u128,
}

fn modal_resource_overflow_refusal(overflow: ModalResourceOverflow) -> Vec<Diagnostic> {
    vec![diag(
        "diagnostic:modal:resource:overflow",
        MODAL_RESOURCE_ARITHMETIC_OVERFLOW,
        "blocking",
        format!(
            "the modal resource preflight's checked arithmetic overflowed at `{}`; no count, charge or sum is wrapped or saturated, so the request is refused before any state solve or n² allocation",
            overflow.0
        ),
        vec!["model".into(), overflow.0.to_string()],
    )]
}

fn modal_preflight_refusal(message: String, terms: Vec<String>) -> Vec<Diagnostic> {
    vec![diag("diagnostic:modal:resource:preflight", MODAL_RESOURCE_PREFLIGHT_EXCEEDED, "blocking", message, terms)]
}

/// P0 (module docs): the bounds, the counts and the budget check, or the
/// refusal by name.
pub(crate) fn modal_resource_preflight(
    prepared: &Prepared,
    params: &ModalIntegrityParameters,
    budget: &ModalResourceBudget,
) -> Result<ModalResourcePreflight, Vec<Diagnostic>> {
    let dimensions = modal_resource_bound_dimensions(prepared, params, budget).map_err(modal_resource_overflow_refusal)?;
    let bounded = modal_resource_term_counts(&dimensions).map_err(modal_resource_overflow_refusal)?;
    modal_resource_judge(dimensions, &bounded, budget)
}

/// The budget check on P0's counts (module docs): weight-or-refuse, the
/// checked charges and stage allotments, and Σ_s A_s ≤ W_budget.
pub(crate) fn modal_resource_judge(
    dimensions: ModalResourceDimensions,
    bounded: &[Option<u128>; MODAL_RESOURCE_TERM_COUNT],
    budget: &ModalResourceBudget,
) -> Result<ModalResourcePreflight, Vec<Diagnostic>> {
    // A term that arithmetic cannot admit: no bound, or a nonzero count with
    // an undetermined weight. The list is bounded by the fixed term count.
    let mut inadmissible = Vec::new();
    for term in MODAL_RESOURCE_TERMS {
        let id = term.modal_resource_id();
        match (bounded[term as usize], budget.weights[term as usize]) {
            (None, _) => inadmissible.push((id, format!("unbounded: {}", term.modal_resource_unbounded_reason().unwrap_or("no bound at P0")))),
            (Some(count), None) if count > 0 => inadmissible.push((id, format!("count {count}, weight undetermined"))),
            _ => {}
        }
    }
    if !inadmissible.is_empty() {
        let terms = inadmissible.iter().map(|(id, _)| id.to_string()).collect();
        let stated: Vec<String> = inadmissible.iter().map(|(id, why)| format!("{id} ({why})")).collect();
        return Err(modal_preflight_refusal(
            format!(
                "the modal resource preflight cannot admit {} by arithmetic: {}. A term whose weight is undetermined, or whose count has no bound, refuses rather than being dropped (weight-or-refuse), before any state solve or n² allocation",
                if inadmissible.len() == 1 { "a term" } else { "these terms" },
                stated.join("; ")
            ),
            terms,
        ));
    }
    let mut counts = [0u128; MODAL_RESOURCE_TERM_COUNT];
    let mut charges = [0u128; MODAL_RESOURCE_TERM_COUNT];
    let mut stage_allotments = [0u128; 5];
    for term in MODAL_RESOURCE_TERMS {
        let index = term as usize;
        let count = bounded[index].unwrap_or_default();
        let charge = modal_resource_mul(u128::from(budget.weights[index].unwrap_or_default()), count, term.modal_resource_id())
            .map_err(modal_resource_overflow_refusal)?;
        let stage = MODAL_RESOURCE_STAGES.iter().position(|&s| s == term.modal_resource_stage()).unwrap_or_default();
        stage_allotments[stage] = modal_resource_add(stage_allotments[stage], charge, "A_s").map_err(modal_resource_overflow_refusal)?;
        counts[index] = count;
        charges[index] = charge;
    }
    let total = modal_resource_total(&stage_allotments, "W").map_err(modal_resource_overflow_refusal)?;
    if total > budget.work_budget {
        // The dominant term: the largest charge, the first on a tie.
        let dominant = MODAL_RESOURCE_TERMS.iter().copied().fold(MODAL_RESOURCE_TERMS[0], |best, t| if charges[t as usize] > charges[best as usize] { t } else { best });
        let id = dominant.modal_resource_id();
        return Err(modal_preflight_refusal(
            format!(
                "the modal resource preflight's weighted work W = {total} ps exceeds the budget W_budget = {} ps; the dominant term is {id}, count {} at {} ps per unit = {} ps. Refused before any state solve or n² allocation; W is a weighted-work budget, not a runtime",
                budget.work_budget,
                counts[dominant as usize],
                budget.weights[dominant as usize].unwrap_or_default(),
                charges[dominant as usize]
            ),
            vec![id.to_string()],
        ));
    }
    Ok(ModalResourcePreflight { dimensions, counts, charges, stage_allotments, total })
}
