# I14: implement slice K5 (W4: the constrained-body witness and the curved objectivity screen)

> This is an implementation TASK. Read Root `AGENTS.md`, `agents/AGENT_TASK.md` and `_COMMON.md` first. The Mac host rules in `I8R_K1_RESUME.md` ("The Mac host" and "Platform calibration") override `_COMMON.md`'s host section, and apply to you in full, with K5's paths below in place of K1's.

## Roles

- ROOT (HELP_HUMAN) dispatches you directly, as a background subagent, and is your return path. There is no separate T3 manager on the Mac.
- Make no Git writes and no index operations. ROOT commits.
- Record the delegation mechanism in RETURN.

## Purpose

K5 is the §6 row "K5: W4 witness and curved screen | after K2b (V1-N4; shares `SA`) | `FK/rigid_body.rs`, `SA` (geometry, curved screen) | A user-element internal mechanism (NP-C-like), a stabilized companion, near-collinear ties, a curved screen positive and a seeded negative".

- **Why now.** K2b (PR #1040, main `e7d930d49`) was K5's predecessor in the selected kernel order "S11-K → K3a → K-D5 → K2a → K1 → K2b → K5" (`ROOT_SELECTION_DESIGNS.md`, Selected item 2; §6 and D-10). K4 (I12) and F1b (I13) are in implementation in parallel. Neither edits `SA` or `FK/rigid_body.rs` (Coordination).
- **What W4 is for** (§1, §2.5, §4.9).
  - Today `SA`'s geometric screen assesses only bodies made entirely of straight frames. A body that contains a user-stiffness element or a curved bend is skipped (`BodyEvidence::geometry`, `SA:1241-1250`), and only the matrix gate judges it.
  - The skip is disclosed in the published basis text: "physical rigid-null witness unqualified for bodies containing user/curved elements; matrix positivity remains mandatory" (`symmetry_basis`, `SA:1291-1298`, rendered through `StructuralReport.symmetry_basis`, `FK/structural.rs:313`).
  - W4 adds a constrained-body null-space witness that covers user-element ties and curved elements. A witnessed mechanism is then refused as a physical mechanism, with its direction, instead of being left to the matrix gate's pivot screen.
- **Geometry first is shared with W1.** "The geometric rigid-body assessment runs before any factor (W4 generalizes it)" (§4.1.3). W1a (K4) is frames only and calls `assess_rigid_body`. W1b's user matrices (F3) and W1c's curved elements (with T4) will need K5's function.

**Product reach (by reading; to be confirmed by product runs at checkpoint 0 and B).**
- **User elements are realized in no product solve.** FK's `UserStiffnessElement::new` refuses a zero lateral stiffness (`validate_positive_finite`, `FK/lib.rs:649-652`, `:1832-1840`). PP builds joints only through `new` (`PP:5864`), and before that refuses every joint with a nonzero lateral stiffness on every route (`PP:1740` → `preview_physics.rs:110-148`, `JOINT_ELEMENT_EQUILIBRIUM_UNQUALIFIED`). The one exception is PP's `#[cfg(test)]` historical scope (`historical_pressure_reference::with_scope`), which suspends the joint refusal for named in-crate tests (for example `PP:13139-13240`, `:16744-16760`).
- **Curved bends are realized** when a component sets `solver_consumption = "curved_bend_macro_element"` (`PP:140`, `build_curved_bend_macro_elements` `:5937`). The desktop cannot set it (R5-4). No committed request or model that T9 runs realizes one. Product tests and fixtures that do include `preview_physics_runtime.rs` (`arc_model` `:834`, and `:918`), `formation_check_runtime.rs:225`, `s11f_tests.rs:1591`, `s11g_tests.rs:1750`, PP `lib.rs` tests, and headless `result_envelope_binding.rs:453-500`, which solves `fixtures/results/invented/result_export_v0_2.json`'s `curved-tip-weight-full` and `curved-pressure-full` producer cases.

### What K5 builds on (merged; every line on `24dea2dae`, re-locate on your base)

- **FK's rigid-body screen** (`FK/rigid_body.rs`): `assess_rigid_body` `:33-192`, with its one-sided Jacobi SVD `:88-131`, the rank screen `τ_B = 64·γ(max(m, 6))·σ_max` `:134`, the candidate list `:157-177`, and the exact witness `original_rigid_witness` `:193-248`. The witness verifies each candidate with `Expansion` sums of the original coordinates and publishes node motion only if it is exactly representable (`:234-246`). The tests are `:250-403`, including KREV-01's coarsened-witness and radix-unit controls.
- **`Expansion`** (`FK/structural.rs:675-745`, `pub(crate)`, so reachable from `rigid_body.rs`): exact nonoverlapping sums, `add_product` (TwoProduct via `mul_add`), `is_zero` and `exact_scalar`.
- **K1: the shared body evidence.** `BodyEvidence` (`SA:1203-1285`) serves both `AssemblyEvidence::geometry` (`SA:132-139`) and `SparseAssemblyEvidence::geometry` (`SA:540-547`). `EvidenceParts::new` (`SA:1038-1159`) records frame and user coordinates (`node`, `:1160-1172`), edges with an objective flag (frames `true`, users and curved `false`, `:1081-1129`) and positive-spring grounds (`:1131-1144`).
- **K-D5: the formation primitives and the selection flag.**
  - `FormationPrimitives` (`SA:43-49`) holds the frames, users, curved slots (`CurvedSlot`, `:52-60`, with `explicit` for a slot with no traced formation) and springs.
  - The curved slot is matched to its macro element by node indices and bitwise-equal global stiffness (`formation_source`, `SA:1302-1352`).
  - The four formation-checked entries take `curved_sources` and `selected`, and `selected` is false for any invocation with a nonlinear support (`PP:4441-4449`, `built.nonlinear_supports.is_empty()`).
- **K2b: the force-scaled siblings.** `solve_force_scaled_with_formation_check` (dense `SA:390-435`, sparse `:824-…`) matches curved slots at 2^b (`force_scaled_formation_source`, `SA:1441-1507`). The W2 orchestrator `solve_with_force_scaling` (`SA:1722-1746`) calls them with the case's `selected` (`ForceScalingCase`, `:1540-1559`; `evaluate_force_scaled`, `:1606-1699`).
- **R5-4 (adopted in revision 5a)** (`DESIGN_NUMERICS/R5_4_CURVED.md` §2): the product's curved element is K = [[H K_t Hᵀ, −H K_t], [−K_t Hᵀ, K_t]] with H from the chord c = (R(cos φ − 1), R sin φ, 0) (`curved_bend/src/lib.rs:242-252`), not from the nodes. So it "is not objective on binary64 inputs". R5-4's intended element builds H from the actual chord, and "is the unique objective element with the product's flexibility … whose null space is exactly the six rigid modes of the actual nodes, for any K_t".
- **The Mac calibration** (`PLATFORM_CALIBRATION_MAC/RECORD.md` §1): macOS `hypot` is 1 ulp from correctly rounded on 16 of 334 distinct T9 arguments. **6 of those 16 have second argument exactly 1.0**, the shape of the Jacobi rotation `zeta.hypot(1.0)` at `rigid_body.rs:108` (77 distinct such calls in all; `t9/libm_calls_distinct_*.txt`). No published status differed.

### What K5 adds (§4.9; the names are the design's, the signatures are fixed at checkpoint 0)

1. **`assess_constrained_bodies(sub_bodies, ties, grounds)`, beside `assess_rigid_body`, in `FK/rigid_body.rs`.**
   - **Objective sub-bodies**: node sets joined by straight frames, and by curved elements qualified under Q2. Each has six rigid parameters (t, θ); node motion is `u = t + θ×(x − o)`, rotation θ.
   - **Ties**: each user element imposes `u_a = u_b` and `θ_a = θ_b`. "Its energy is zero only for equal nodal motion, because every stiffness is positive" (subject to Q5).
   - **Grounds**: the restrained and prescribed DOFs and the positive springs, as today (and, under Q6, directional grounds).
   - **Rank**: "The null space of the stacked map uses the existing SVD rank screen and τ_B form. It returns Restrained, MechanismWitnessed (with the direction mapped to nodes), or NumericallyUnresolved" (subject to Q4).
2. **The curved objectivity screen** (§4.9; subject to Q2): "For the six rigid vectors r_k at the element's nodes, `|K_e r_k|` must lie within the formation allowance from `curved_formation` (`SA:291-402`)" (now `SA:1781-1892`). "If it passes, the element joins the objective sub-body. If it fails, the body stays unqualified for a witness, with a reason, and the matrix gate still runs."
3. **SA's wiring** (subject to Q1): the bodies that `BodyEvidence::geometry` skips today are assessed with (1), and a witnessed mechanism is `StructuralError::Mechanism { direction }` over the global DOFs, as for frames (`SA:1268-1274`).
4. **The derivation** (§4.9's proof sketch, written out in RETURN and checked by the reviewer): "Each family's energy is non-negative, and its zero set is the stated linear space: rigid motions for frames and screened curved elements, equal motion for user elements. So total energy is zero exactly on the intersection with the grounds. This extends the existing welded-frame argument."

**The reduction the stacked map admits (recommended; derive it at checkpoint 0).** The ties force one θ on every sub-body they connect. Choose a spanning forest of the tie graph over the sub-bodies. A tree tie (a ∈ A, b ∈ B) gives `t_B = t_A + θ×(x_a − x_b)`. So every node x of sub-body B moves as `t_0 + θ×(x − o + s_B)`, where `s_B` is the exact sum of the tie offsets `x_a − x_b` along the tree path from the root. A non-tree tie adds three rows `θ × c = 0`, with c its exact cycle offset (for a tie inside one sub-body, `c = x_a − x_b`). Each connected body therefore reduces to **six unknowns**: ground rows of `assess_rigid_body`'s form at the "virtual positions" `x − o + s_B`, plus the cycle rows. **This is exact algebra, never a dense 6S×6S map** (Constraints, memory), and it makes clear why a tie is not a rigid link (§7.3 item 12): a rigid link would use the actual positions x, with no shift.

### What K5 does not do

- **No change to `assess_rigid_body` or `original_rigid_witness`** (subject to Q4(c)). K4 calls `assess_rigid_body` (Coordination). Frame-only bodies keep today's code path, node order, ground order and published direction bits.
- **No change to the edges' objective flag, `qualified_passive_family` or exact-block eligibility.** `source_recovery.rs:887` refuses exact-block for an unqualified family, `permits_contact_seed_trial` (`SA:1898-1912`) reads the flag in the loop (`NI lib.rs:649`), `scrutinize_gaps` reads it (`SA:2328`), and PP passes its own flag at `PP:2884`. W4's qualification is carried separately.
- **No field on `StructuralReport` or `StructuralSolution`, and no edit to `FK/structural.rs`.** The report is `Debug`-published (D5C-3). "With a reason" (§4.9) is carried by K5's own return types, available to tests and F2a.
- **No change to the nonlinear loop or to any invocation with a nonlinear support** (Q1; revision 5a.1 and ROOT's "D2 revision 5b choices" item 2: "T3 must not remove a result the product publishes today").
- **No SUP-17 text.** §4.9's SUP-17 paragraph was delivered by F1a (PR #1025).
- **No formation-range work on curved elements** (the K2a T3-close item; Q10), no curved formulation repair (T4/W1c), and no joint repair (T4's M07).
- **No new `StructuralError` or `FrameKernelError` variant** (K2b ruling 7). A new `NumericallyUnresolved` reason string is proposed at checkpoint 0, if any.
- No dependency or lockfile change. No timing or memory-growth claim (K6 owns them).
- **Routed items K5 does not take:**
  - **The skew M03 pin** was routed "to K1's pattern-path M03 tests (K5 fallback)". It was closed by a tests-only slice, PR #1038 at `e8b416e43`. M03's general skew scope is on the T3-close list. K5 adds no M03 decision, so it does not lean on M03's skew scope, and the M03_SKEW_PIN review's carry-forward (N2: torsion, near-threshold and Iy ≠ Iz rows "if K2b, K5 or F1b lean on M03's skew scope") is not triggered. The reviewer confirms this.
  - **The gate corpus's missing curved bends** (work graph, T3 row) stays a T3-close decision. K5's product-run table is evidence for W4 only.

## ROOT rulings for this slice (2026-09-28)

A TASK drafted this brief. ROOT reviewed it and rules on its open questions as follows. The questions, with their options, remain at the end for the record. The stale-design list is recorded as rulings in `ROOT_RULINGS_V1.md`, "K5: spawn and rulings (ROOT)".

1. **Q1: (b), with the PP tests-only file.** W4 applies only in the four `selected` branches of the formation-checked entries: dense and sparse, unscaled and force-scaled.
   - Every other `geometry()` call keeps today's frame-only geometry byte for byte, pinned by a behavioural test and a text test.
   - No invocation with a nonlinear support changes, and the loop is untouched.
   - RETURN derives that the contact-seed guard stays closed.
2. **Q2: (b). A curved slot matched to its macro source joins its sub-body by construction** (R5-4's intended element is objective, with a null space exactly the rigid modes of its actual nodes). The screen's ratio is computed exactly and recorded as evidence only. Explicit and unmatched slots stay unqualified.
   - **Condition.** RETURN derives step by step that a witness under (b) is an exact zero-energy motion of the intended element. It also derives that the product's element carries energy along it only through its chord mismatch, the non-objectivity R5-4 and the T4 row record. **So a K5-C1 refusal never refuses a model that the intended physics restrains.** The reviewer checks this independently.
   - Every K5-C1 and K5-C2 occurrence in the corpus is reported from product runs on Mac main and the candidate. ROOT rules on the table before B.
3. **Q3: (a), T4's confirmation is not a prerequisite,** as R5-4 established for K-D5. RETURN derives the claim for the intended element. T4's item stays open for the product's element.
4. **Q4: (b), a libm-free screen in the new function,** with the same τ_B rule, `sqrt` forms in place of `hypot`, and the witness verified exactly. `assess_rigid_body` stays byte-identical, and its `hypot` dependence goes on the T3-close list.
5. **Q5: (a),** ties for today's element:
   - every stiffness must be finite and positive, or the element is unqualified with a reason;
   - a T4 tripwire test is added;
   - RETURN carries a notice to T4.
6. **Q6: (a).** K5's function accepts directional ground rows, tested at FK level. Wiring K4's `geometry_first` to it is a follow-up after both merge; K5 does not write K4's files. This resolves the route of K4's O1 amendment.
7. **Q7: (b).** A new basis text for mixed families on the four selected branches only. The exact string is proposed at checkpoint 0 and fixed by ROOT.
8. **Q8: (b), gate part 1 only, as a regression net.** On this Mac, part 1 takes about 5 minutes per tree (G1, `GATE_BASELINE_MAC_E7D930D49/`).
   - It is compared per run against G1's baseline, if K5's base has the same product tree as `e7d930d49`; verify that. Otherwise, against a fresh base run.
   - PASS requires every run byte-identical in P1's envelope, outcome and standing.
   - ROOT serializes the run against F1b's gate.
9. **Q9: (a).** K5's logic lives in `rigid_body.rs`, and SA only plumbs data, so neither site table changes. An unavoidable SA row is a declared, additive extension of FK's table under K1's conditions, coordinated with K4.
10. **Q10: (a), not K5's.** The item stays on the T3-close list. K5 records any zero or subnormal curved entry its corpus meets, as an observation.
11. **Q11: one slice,** with checkpoint A split into A1 (the FK function) and A2 (the SA wiring and the curved rule).

## Scope

### 1. Kernel or product (Q1)

- **As designed, K5 is not kernel only.** The row writes `SA` "(geometry, curved screen)", and every solve entry of both evidences calls `geometry()`: 11 call sites (dense `SA:168`, `:227`, `:268`, `:316`, `:361`, `:412`; sparse `:606`, `:659`, `:714`, `:780`, `:848`). Among them is the nonlinear loop's `solve_binary64` (`SA:268`, called at `NI lib.rs:1990`).
- **Recommended (Q1(b)): W4 applies only in the four `selected` branches** of the formation-checked entries: dense `SA:227` and `:412`, sparse `:659` and `:848`. Every other call keeps today's frame-only geometry byte for byte.
  - These branches are reached only by linear invocations with no nonlinear support. So no invocation with a nonlinear support changes, and the loop is untouched.
  - **Why the loop matters here:** `permits_contact_seed_trial` admits any `Mechanism` unconditionally (`SA:1900`). Under Q1(a), a W4 witness on a curved body would open the contact-seed path at `PP:2884` and in the loop (`NI lib.rs:649`), which is T5's domain.
  - Under Q1(b) the PP guard stays closed: `eligible_contact_dofs` returns `None` for an empty support list (`NI lib.rs:439-441`). Derive this in RETURN.
- **They are also the entries F1b moves PP onto** (the sparse formation-checked sibling, and W2's orchestrator, which calls the force-scaled formation-checked siblings with the case's `selected`). So PP reaches W4 on either side of F1b's merge.
- **W4 rides the existing arguments.** It uses `selected` and `curved_sources`, which those entries already take, so no public SA signature changes. A signature change would force a PP edit, which is outside K5's write set.

### 2. Frame-only bodies are byte-identical by construction

- `assess_rigid_body` and `original_rigid_witness` are unchanged. A frame-only body is enumerated, ordered and grounded as today (the seed-order breadth-first search at `SA:1215-1240`, origin at the first node, prescribed then spring grounds, `:1255-1260`).
- **The first failing body still ends the screen, in seed order.** A model whose first failing body is mixed may now report that body's outcome. That is in the curved change class below.
- RETURN derives this claim step by step, and the reviewer checks it independently.

### 3. What may change in published bytes (the fixture stop rule)

- **The committed fixtures are not expected to move.**
  - No committed request or model that T9 runs realizes a curved bend.
  - The committed models with a realized-joint flag (for example `fixtures/product_preview/invented_preview_model.json`, joint C-150, lateral 9e5) are refused at `PP:1740` before any element is built.
  - Frame-only paths are unchanged (item 2).
  - Re-check all three on your base with a scan (checkpoint 0).
  - **So T9's expected result is 112 of 112 byte-identical (Mac-only). Any committed-byte change stops the work,** and nothing is regenerated without ROOT's approval.
- **The pre-registered change classes on the product** (under Q1(b)). These are the only published changes allowed, each checked by product runs on Mac main and the candidate:
  - **K5-C1 (curved mechanism).** A linear invocation, with no nonlinear support, whose body containing a qualified curved element is a geometric mechanism. Main leaves it to the matrix gate, which refuses it (typically `NUMERICAL_INTEGRITY_UNRESOLVED` or `NUMERICAL_INTEGRITY_NEGATIVE_ENERGY`) or publishes a value along the mechanism, held only by the element's formation error (Sensitive through K-D5, or Passed). The candidate refuses it as `NUMERICAL_INTEGRITY_PHYSICAL_MECHANISM` with the witness direction (`PP:1272`; the message renders the direction through `{self:?}`, `FK/structural.rs:244-248`).
  - **K5-C2 (ambiguous curved geometry).** The same invocations, where the body's geometric rank is in the τ_B band and no exact witness exists: `NumericallyUnresolved` ("rigid-restraint rank unresolved", or the reason ruled at checkpoint 0), where main's matrix gate may publish. Report every occurrence in the corpus.
  - **K5-C3 (basis text; only if Q7(b) is ruled).** The `symmetry_basis` text of mixed families, on the four selected branches only.
  - **K5-C4.** Nothing else. In particular, every frame-only case and every invocation with a nonlinear support keep identical envelope bytes.
- **User elements** reach none of these classes in the product. In PP's `#[cfg(test)]` historical scope they can reach C1 to C3, so those in-crate tests are part of the corpus.
- **The both-entry gate (Q8).** The gate's 222 requests realize no curved bend and no user element (work graph, T3 row, the coverage finding from RV5's K-D5 review). Under Q1(b) every gate run is therefore byte-identical by construction, and the gate cannot show K5's change. **Recommended: not run**, with the scan and the derivation as evidence, and the curved product-run table (Required tests F) in its place.

## Write set (re-locate every line on your base)

| File | Change | Status |
|---|---|---|
| `FK/src/rigid_body.rs` | additive: `assess_constrained_bodies` and its types; the libm-free screen (Q4); the curved screen or qualification helper if it lives in FK (Q2); directional grounds (Q6). `assess_rigid_body` and `original_rigid_witness` unchanged | in the K5 row |
| `SA` (`nonlinear_integration/src/structural_adapter.rs`) | the W4 wiring in the four selected branches (Q1); per-body sub-bodies, ties and curved qualification from `FormationPrimitives`, `coordinates` and `curved_sources`; the `symmetry_basis` text (Q7); a `#[cfg(test)] mod k5_tests;` line | in the K5 row |
| new `SA/structural_adapter/k5_tests.rs` | SA-level tests, the loop pin (behavioural and text) and the wiring pins (as K1's `k1_tests.rs`, K-D5's `kd5_tests.rs` and K2b's `k2b_tests.rs`) | certain (the SA test-module precedent) |
| new `FK/tests/k5_constrained_bodies.rs`, and `FK/tests/k5_constrained/` (a standard-library generator with `--check`, vectors, its own `SHA256SUMS`) | FK-level tests of the public function | certain |
| new `FK/tests/k5_scale.rs` | the 10,000-member reduction test with a capped counting global allocator, **in its own test binary** | certain (Constraints, memory) |
| new `P/core/product_physics/tests/k5_*.rs` | product-level pins through both entries and both modes: a curved mechanism refused with its witness (K5-C1), and the same model with a nonlinear support unchanged | **proposed** (Q1; a PP tests-only extension; no PP source) |
| `FK/tests/s11_site_table.rs` | rows for any new accumulation shape in `SA`, additively, under K1's five conditions | **only if needed** (Q9; K4 also edits this file) |
| `T3/IMPLEMENTATION/K5/**` in `<wt>/k5` | records | certain |

**Not in scope. Stop and ask before touching any of these:**
- `FK/src/structural.rs`, `FK/src/lib.rs`, `FK/src/structural/**` (including K4's `retained/**`), `exact_sum.rs`, `load_ledger.rs` and `formation_check.rs`.
- `curved_bend` (`CB`), `straight_pipe`, and `nonlinear_integration/src/lib.rs` (including `CurvedBendStiffnessElement`, `NI lib.rs:170-237`).
- `PP` source, `source_recovery.rs` and `source_receipt.rs` (F1b's).
- `nonlinear_integration/src/s11k_tests.rs` and `PP/tests/s11f_site_test.rs` (F1b's, Q11 there). The loop pin goes in `k5_tests.rs` instead.
- K4's files: `FK/src/structural/retained/**`, `FK/tests/retained_k4/**` (and `s11_site_table.rs`, except under Q9).
- **The frozen references and fixtures, which are never edited:** N01–N09, R01–R07 and NP-A–NP-D (`P/validation/benchmarks/numerical_integrity/`); R1's `REFERENCES/**`; the T0R references; `GATE/*.json`. A mismatch goes to ROOT.
- The committed fixtures and hash pins; dependencies and lockfiles; `performance_harness` (K6) and `numerical_robustness` (V-K, V-P).

**Constraints on the implementation:**
- **No function of unspecified precision in K5's code:** no `hypot`, `powi`, `powf`, `exp*`, `ln*`, `log*`, trigonometric or hyperbolic function, or `cbrt`.
  - Only IEEE-754 +, −, ×, ÷, `sqrt` and `mul_add` (each correctly rounded, so identical on every platform) and exact expansions.
  - Build powers of two from bits, never with `powi`.
  - `assess_rigid_body`'s existing `hypot` (`:52`, `:108`) is untouched and recorded in RETURN (Q4).
- **Exactness.** A witness is published only after exact verification against the original coordinates (the virtual positions and cycle offsets formed exactly), and only when every node motion is exactly representable. A rounded recovery is never presented as a witness (`rigid_body.rs:234-235`).
- **Determinism.** The published direction is a deterministic function of the exact inputs:
  - canonical row order (ascending local node, then DOF, then cycles in canonical tie order);
  - a canonical candidate order;
  - a canonical representative for the witness.
  - Input list order (sub-bodies, ties, grounds) must not change the status or the direction bits.
- **Tests follow ROOT's standing lessons:**
  - **no test depends on compile-time evaluation of a function of unspecified precision;**
  - **no expected value passes through the platform libm.**
    - Coordinates and stiffnesses come from the generator as binary64 bits; expectations come from exact rational arithmetic.
    - A curved element formed through the product's `sin`, `cos` and `atan2` (`curved_bend/src/lib.rs:166`, `:246-250`) is asserted only by outcome, with a stated margin (a screen ratio ≤ 1/4 or ≥ 4 under Q2(a)), never by bits.
    - `assess_rigid_body`'s direction bits (candidate from a `hypot`-dependent SVD) are compared base against candidate on the Mac, never pinned as constants.
- **Every general claim is derived and independently checkable** (ROOT's standing lesson; ROOT has been wrong twice on underived claims). This covers:
  - the energy zero sets (§4.9's proof sketch);
  - the tree reduction's equivalence with the stacked map;
  - item 2's frame-only byte identity;
  - Q1's "no invocation with a nonlinear support changes";
  - the gate's byte identity (Q8);
  - the screen's platform independence (Q4);
  - every mutant equivalence and every "unreachable" path.
- **Memory (I8R).**
  - **Never materialize a dense matrix for a model with 10,000 or more members**, including a dense 6S×6S stacked map over S sub-bodies. The reduction keeps memory O(nodes + grounds + ties) per body.
  - **RF-MECH-LINE-IN-CHAIN1000** (1,007 nodes, 1,005 members) runs at FK level (coordinates and grounds) or through `SparseAssemblyEvidence` on its pattern. Never pass it through `AssemblyEvidence::new`, which holds four dense n×n arrays (`SA:76-81`): about 1.2 GB at n = 6,042.
  - SA-level tests on dense evidence stay at a few hundred members or fewer: the four dense arrays are about 13 MB at DISC-CHAIN100's 103 members, and about 1.2 GB at 1,005.
- **No public SA signature changes** (PP and F1b call them). W4 rides `selected` and `curved_sources` (Scope 1).
- **Pre-existing cost, recorded:** `BodyEvidence::geometry`'s body search scans every edge for every node (`SA:1222-1240`), so it is O(nodes × edges). K5 keeps it for frame-only order (item 2). It is not a memory issue, and K6 owns timing.
- **`dead_code`.** Items with no non-test caller get a per-item `#[allow(dead_code)] // <consumer>` ("F3 API", "W1c API", "V-K API"). The non-test build has no warnings.
- **Callers.** Enumerate every caller of every changed function by lexer scan (`_run_records/callers.txt`), as K-D5 and I7 did.

## Basis (read in this order)

T3's records are read at `<wt>/numerics`, which carries ROOT's latest rulings. Code is read on your base.

1. Root `AGENTS.md`, `agents/AGENT_TASK.md`, `_COMMON.md`, and `I8R_K1_RESUME.md` "The Mac host" and "Platform calibration".
2. `T3/DESIGN_NUMERICS/DESIGN.md` revision 5a.2 (sha256 `fb62ef4a…`; hash-pinned, don't edit):
   - §1 (W4) and §2.5 (the rigid-null witness, user elements, curved elements);
   - **§4.9, all of it;**
   - §4.1.3 (geometry first, as W1 uses it);
   - §4.2 (the user-matrix W1b and curved W1c rows);
   - §4.3 (`Mechanism` never triggers W1; the nonlinear-support rule) and §4.3.1 (the curved and user re-formation; D5C-2);
   - §4.10's kernel lane and RF-MECH ("must be refused with a witness or an unresolved status, and no rows");
   - §6: the K5 row, the order and the serialization note, and D-9 and D-10 (§9);
   - §7.1 (NP-C), §7.2's RF-MECH row, §7.3 items 7 and 12, §7.5 and §7.6.
3. `DESIGN_NUMERICS/R5_4_CURVED.md` §2 and §5.
4. `T3/ROOT_SELECTION_DESIGNS.md` (the order, the fixture stop rule, the nonlinear-loop constraint).
5. `T3/ROOT_RULINGS_V1.md`:
   - "D2 revision 5b choices" item 2 (the nonlinear-support rule);
   - "R5-4: curved bends under K-D5";
   - "K2a product reach: correction 2", item 4 (the curved-formation T3-close item) and item 5 (the lesson);
   - "K1: the S11 site table for sparse.rs and formation_check.rs" (the five conditions);
   - "K1: extending the K-D5 and option-(c) source pins …" (the pin method);
   - "The skew M03 pin: a tests-only follow-up before K2b";
   - "K3: Q7 reversed" (the lesson on functions of unspecified precision);
   - "K2b: the b-rule's window misses …" (the lesson on derived claims);
   - "K4: spawn and rulings", and "K4: rulings on I12's checkpoint-0 plan", **O1** (the Q6 amendment);
   - "F1b: spawn and rulings".
6. The Mac calibration: `PLATFORM_CALIBRATION_MAC/RECORD.md` §1, and `t9/libm_calls_distinct_mac_native.txt` against `t9/libm_calls_distinct_correctly_rounded.txt` (the Jacobi `hypot(ζ, 1)` rows).
7. **The references:**
   - `T3/REFERENCES/README.md` (RF-MECH, §6), `references.json` (sha256 `7b176dbb…`) and `references.py` (`80d473a7…`). `references.py --model RF-MECH-LINE-IN-CHAIN1000` prints its full model.
   - `P/validation/benchmarks/numerical_integrity/{README.md, COVERAGE.md, fixtures.json}`, NP-C: "Full rigid rank with internal slip; stabilized companion; exact near-collinear rows" (`near_collinear_exact_rank: 2`).
8. **Merged records** (on your base): `IMPLEMENTATION/KD5/RETURN.md` (the curved and user re-formation, slot matching); `IMPLEMENTATION/K1/RETURN.md` §12; `IMPLEMENTATION/K2B/RETURN.md` §15; `REVIEW/M03_SKEW_PIN_REVIEW.md` N2; `IMPLEMENTATION/K1_MERGE/RECORD.md`, "A routed item K1 did not take".
9. **The code on your base:**
   - `FK/rigid_body.rs` (all);
   - `FK/structural.rs`: `StructuralError` `:223-248`, `StructuralReport` `:294-314`, `StructuralSolution` `:316-325`, `gamma` `:521`, `Expansion` `:675-745`, `transform_roundoff` `:2084-2119`;
   - `FK/lib.rs`: `UserStiffnessElement` `:622-696`, `user_stiffness_local_matrix` `:1739-1761`;
   - `SA`: `:1-160`, `:213-283`, `:390-435`, `:540-547`, `:644-662`, `:824-850`, `:1028-1352`, `:1441-1507`, `:1528-1746`, `:1781-1912`;
   - `NI lib.rs`: `:170-237`, `:433-441`, `:595-660`, `:1975-2016`;
   - `curved_bend/src/lib.rs`: `:100-259`, `:998-1052`;
   - PP: `:1265-1285`, `:1613`, `:1725-1760`, `:2870-2895`, `:4392-4460`, `:5789-5883`, `:5937`; `preview_physics.rs:110-148`; `source_recovery.rs:879-889`;
   - headless `src/lib.rs:804` and `src/result_envelope_binding.rs:445-500`;
   - `FK/tests/s11_site_table.rs` (SOURCES `:39-90`, the SA rows `:150-153`) and `PP/tests/s11f_site_test.rs` (KERNEL `:84-…`);
   - `SA/structural_adapter/{k1_tests, kd5_tests, k2b_tests}.rs` (`kd5_tests.rs:591-601` builds a lateral = 0 joint by struct literal).
10. **The T3 row of `WORK_GRAPH.md`** (the T3-close items that touch K5, listed under Purpose and Q10) and **the T4 row** (the M07 joint repair; "confirm the curved-bend construction keeps the rigid-motion null space (D1 W4 and W1c)"; the curved non-objectivity at UTM coordinates).
11. `T3/OWNER_DIRECTION.md`, "Owner decision (2026-09-28): DEC-025 on the Mac".
12. `TASK_BRIEFS/I12_K4_IMPLEMENTATION.md` and `I13_F1B_IMPLEMENTATION.md`, for coordination.
13. **The brief's probe:** `<wt>/scratch/briefs/k5_scratch/curved_screen_probe.py` (sha256 `490c509e…`) and its two stdout files. It is an emulation of the product's curved element and of `curved_formation`, on the drafting host's libm, not a product run (Q2).

## Base, branch and paths

- **Branch:** `codex/piping-k5-20260928`, from current main. ROOT creates it in `<wt>/k5` and records the SHA at spawn.
  - **At spawn, the base is main `24dea2dae`,** whose product tree (`core`, `fixtures`, `validation`, `schemas`, `apps`, `tools`) equals `e7d930d49`'s.
- **Target:** `<wt>/k5-target`.
- **Scratch:** `<wt>/scratch/i14`.
- **Mutants:** one clean `git archive` copy and one clean target per mutant, under `<wt>/k5-mut/<mutant>/`. Delete each target afterwards.
- **Python:** `<VENV>`, standard library only (`fractions`, `hashlib`) for the generator, scans and product-run drivers.
- **Baselines.** The base's product tree equals `e7d930d49`'s, so two already exist:
  - the 39-manifest Mac suites: `IMPLEMENTATION/K2B_MERGE/dec025/suites.log` (with `suites_vs_baseline.txt`);
  - T9's base hashes: `PLATFORM_CALIBRATION_MAC/t9/output_sha256_main_mac_native.txt`. They are a cross-check for your own base build, not a substitute for it.

## Coordination

**K4 (I12, in implementation in `<wt>/k4`).**
- Its write set: `FK/src/structural/retained/**`, one accessor in `FK/src/exact_sum.rs`, `FK/tests/s11_site_table.rs` (Q8 there), `FK/tests/retained_k4/`, and its records.
- **Write-set overlap: none in K5's certain rows.** The one conditional overlap is `FK/tests/s11_site_table.rs`, only if K5's SA edits add an accumulation shape (Q9).
- **K4 reads `assess_rigid_body`** (I12's approved plan, `<wt>/scratch/i12/CHECKPOINT0_PLAN.md` §8.1, bound for K4's `_run_records/`: per body; `MechanismWitnessed` is refused before any attempt, while `Restrained` and `NumericallyUnresolved` proceed). K5 leaves it byte-identical, so K4 need not re-run its RF-MECH tests.
- **K4's O1 amendment** ("K4: rulings on I12's checkpoint-0 plan"): a body with a non-spanning directional ground "is not assessed geometrically … It proceeds as `NumericallyUnresolved` does", and "a full geometric treatment of partial directional grounds belongs to W4/K5's generalized assessment (§4.9)". **So this is a W4 gap routed to K5 (Q6).** K5 cannot write K4's files. Wiring K4's `geometry_first` to K5's function is a follow-up after both merge.

**F1b (I13, in implementation in `<wt>/f1b`).**
- Its write set: PP, `source_recovery.rs`, NI `s11k_tests.rs` (tests only), PP tests (`s11f_site_test.rs`, `k2a_formation_range_runtime.rs`, `pressure_membrane_range.rs`, new `f1b_*`). **F1b makes no SA edit** (its Q6(c) was refused).
- **Write-set overlap: none,** provided K5 edits neither NI's `s11k_tests.rs` nor PP's `s11f_site_test.rs`.
  - PP's site test scans SA with force-token rules only (its rule 8's accumulation counts cover SP, CB and `load_case_algebra`). K5's geometry touches no force token. Confirm at A2. A failure there is a stop, not an edit.
- **Semantic touchpoints:**
  - F1b moves PP onto `SparseAssemblyEvidence::solve_assembled_with_formation_check` (`SA:644`) and W2's orchestrator. Q1(b) wires W4 into those branches too, so PP's route sees W4 on either side of F1b's merge.
  - F1b refuses a realized curved bend at b ≠ 0 by name (its Q3). So in the product, W4's force-scaled branches run at b = 0 only; SA-level tests still cover b ≠ 0.
  - F1b's brief: "K5 changes `geometry()` outcomes for user and curved bodies. Whichever slice merges second merges main and re-runs its suites, T9 and gate part 1."
  - If K5 merges second, it re-runs its suites, T9 and the curved product-run table on the combined tree.
  - If F1b merges second, its gate part 1 is unaffected (no curved bend in the corpus), but its curved-model tests see W4.

**T4, T5, F2a/F3 and W1c** (no edits; notices in RETURN):
- **T4:** the user-tie rule is for today's element. T4's M07 repair changes its zero-energy set, and the tripwire test (Required tests C) fails when it does. The curved qualification rule (Q2, Q3) is stated for T4's null-space item.
- **T5:** the loop's geometry is unchanged (Q1(b)). RETURN states how the loop could adopt W4, and what `permits_contact_seed_trial` would then admit.
- **F2a, F3 and W1c:** the interface section (Return).

**Merge order.** Any slice may merge first. The second merges main, then re-runs its suites and T9 (and the affected product-run table or gate part) before its PR merges.

**Host.** Implementers share the Mac under I8R's caps: at most two cargo jobs of your own, `-j 8`, `RUST_TEST_THREADS=4`, and at most three mutants at once at `-j 4`. **ROOT serializes the heavy phases** (T9 builds, product-run batches, mutation batches) against K4's and F1b's, including F1b's gate. Check `<wt>/guard/memguard.log` after every heavy phase.

## Required tests

The predicate, wherever a value is compared, is the unchanged `|obs − exp| ≤ 1e-9·max(|exp|, scale)`. No new tolerance is introduced anywhere. Geometric outcomes are compared exactly.

**A. Nothing existing moves**
- `assess_rigid_body` and `original_rigid_witness` are byte-identical in source (a diff in RETURN), and their tests pass unchanged.
- **RF-MECH's 9 cases** (R1, frozen at `c0f14201c`), through SA's geometry on frame-only models (dense and sparse evidence, DISC-CHAIN100 and the smaller cases) and at FK level or on sparse evidence (LINE-IN-CHAIN1000). Required:
  - the same statuses as Mac main (expected: the 8 mechanisms witnessed and LINE345-RX-COMPANION `Restrained`; establish main's at checkpoint 0);
  - **the published `Mechanism` direction bits identical, base against candidate, on the Mac** (a harness built from `git archive` copies). Directions are not pinned as constants (Constraints, libm).
- `qualified_passive_family` and `symmetry_basis` are unchanged for every frame-only evidence. `qualified_passive_family` is unchanged for mixed families (a pin). Exact-block eligibility is unchanged (`source_recovery.rs:887`: a curved or user case stays "unqualified passive family").
- K-D5's, K1's, K2b's and S11's tests pass unchanged, including NI's `s11k_tests.rs` pins and both site tables (unless Q9's extension is ruled).
- FK's, NI's, PP's and headless's full suites, and CI's 39-manifest profile with `--no-fail-fast`, against ROOT's Mac baseline. The only failures allowed are the three known Mac platform tests, with byte-identical failure blocks.
- The non-test build has no warnings.
- T9 (see Gates).

**B. `assess_constrained_bodies` (FK; constructed cases, with exact expectations from the generator)**
1. **The reduction against the full stacked map.** The generator forms the unreduced stacked map (6 unknowns per sub-body, 6 rows per tie, every ground) with `Fraction` and decides its exact rank. That is independent of K5's reduction. On a seeded set of at least 10^3 bodies (1–8 sub-bodies, 0–10 ties including cycles and ties inside one sub-body, random grounds), with cases outside the τ_B band only (σ_min/σ_max ≥ 1e-6, or exactly deficient), the status must agree:
   - full rank → `Restrained`;
   - exactly deficient → `MechanismWitnessed` with an exact null motion, or `NumericallyUnresolved` when no candidate verifies, and never `Restrained`. Every seeded case built from a small-integer null motion must be witnessed;
   - a witness on a full-rank case is a failure.
   - Record the seed, the count of each outcome and a digest; commit the first 1,000 records.
2. **The row's five cases** (§6):
   - **An NP-C-like internal mechanism:** a body whose union, welded, would be `Restrained` (`assess_rigid_body` on all its nodes says so), but whose tie motion is free. The expectation is `MechanismWitnessed`, with an exact direction equal to the generator's.
   - **Its stabilized companion:** one more ground removes the tie mechanism, giving `Restrained`.
   - **Near-collinear ties:** the virtual pin positions `x + s_B` are collinear although the actual pins are not.
     - Exactly collinear: `MechanismWitnessed` (rotation about the virtual line).
     - One ulp off the line, in the τ_B band with no exact null vector: `NumericallyUnresolved`.
     - Well separated: `Restrained`.
     - NP-C's `near_collinear_rows` (exact rank 2) are one row pair of this set.
   - **A curved screen positive and a seeded negative:** as ruled under Q2 (see D).
3. **§7.3 item 12, both ways:** a tie case that the rigid-link model calls `Restrained` (a missed mechanism), and a converse that the rigid-link model calls witnessed (a false physical mechanism). Each is decided exactly by the generator.
4. **Cycles:**
   - a non-tree tie whose rows `θ × c = 0` remove the only free mode (`Restrained`; the mutant that ignores cycles witnesses a mode with positive energy);
   - a cycle with c = 0 exactly (no constraint);
   - a tie inside one sub-body (c = `x_a − x_b`).
5. **Exactness** (KREV-01 analogues for ties):
   - offsets at 1e16 and 1e200 scale, where a rounded candidate looks null but the exact check refutes it (no witness, and `node_motion` is `None`);
   - valid witnesses that survive origin shifts and radix units 0.5, 1 and 2;
   - an exact mechanism whose canonical direction is not representable ends `NumericallyUnresolved`, never with a rounded direction.
6. **Order independence:** permuting the sub-bodies, the ties, the tie ends and the grounds gives the same status and the same direction bits.
7. **Power-of-two invariance:** coordinates times 2^k give the same statuses.
8. **Validation:** out-of-range indices, a node in two sub-bodies or in none, non-finite coordinates, and an empty body, with the same error kinds `assess_rigid_body` uses.
9. **Directional grounds (Q6(a)):**
   - a spanning triad per node and kind is equivalent to its three DOF grounds;
   - an axial rotational spring on a translation-pinned line (the RF-SKEW-T-PIN-AX class of K4's O1) gives the status the generator decides;
   - a non-spanning set with a real mechanism is witnessed exactly.
10. **libm independence:** a source scan pins that K5's new functions call none of the functions named under Constraints. The scan excludes `assess_rigid_body` and `original_rigid_witness`, and says so.

**C. User elements** (FK and SA)
- **A tie only when all four stiffnesses are finite and positive** and the orientation is valid. Otherwise the body is unqualified, with a reason, and the matrix gate runs as today. Cover:
  - lateral = 0 by struct literal, as `kd5_tests.rs:591-601` builds it;
  - a negative value;
  - NaN.
  - Explain in RETURN why lateral = 0 cannot be a tie: the lateral relative translation is then free.
- **The T4 tripwire.** An exact rational check that, for a represented element with positive stiffnesses, the tie space (`u_a = u_b`, `θ_a = θ_b`) is exactly the null space of `user_stiffness_local_matrix` (`FK/lib.rs:1739-1753`) under an invertible transform. The test fails if the element's structure changes (T4's M07 repair), and its doc comment names T4.
- **PP's `#[cfg(test)]` historical tests that realize joint C-150** (for example `current_composite_derived_normal_friction_and_reversal`, and the review-row test at `PP:16744-16760`) pass unchanged. State which of them are selected (no nonlinear support), and what W4 decided for each.

**D. Curved elements (SA; as ruled under Q2)**
- **Qualification:**
  - Under Q2(a): the screen formed and compared exactly (`K_e r_k` and `Σ_j bound_ij·|r_kj|` as exact expansions).
  - Under Q2(b): a slot matched to its macro source.
  - Either way, an explicit slot (`CurvedBendStiffnessElement::new`, no traced formation) and an unmatched slot are unqualified, with a reason. So is a slot whose macro node coordinates differ from the frame coordinates recorded for the same node.
- **A curved-only body:** a bend between two translation-pinned nodes, with no frame. Rotation about the chord is witnessed, with the coordinates taken from the macro source (`SA` records none for curved nodes; `EvidenceParts::node` is called for frames and users only).
- **2^b:** the force-scaled selected branch gives the same W4 outcome at b ≠ 0 as at b = 0, with slots matched as `force_scaled_formation_source` matches them (SA-level).
- **The screen's ratio** (both options) is recorded per element on the curved corpus as an observation. **Tests assert product-formed elements by outcome only, with margin** (Constraints). Boundary tests use constructed matrices with exact bits.

**E. SA wiring (Q1)**
- **W4 runs only in the four selected branches.** Pin it with:
  - a text pin in `k5_tests.rs`: the W4 entry token occurs exactly in those four bodies, on identifier boundaries, as K1's scoped pins do;
  - a behavioural pin: a mixed-body mechanism through `solve`, `solve_binary64`, `solve_assembled`, `solve_force_scaled` and each unselected branch gives the matrix gate's outcome (the body skipped, as today; never W4's `Mechanism`), bit for bit equal on dense and sparse evidence and to Mac main's in the product-run table. The selected branches give the W4 outcome.
- **The loop:** a nonlinear-loop model with a curved mechanism publishes as on Mac main (NI-level, in `k5_tests.rs`).
- **Dense and sparse evidences** give identical W4 outcomes and direction bits.
- **Order:** the first failing body in seed order decides the error, as today.
- **The basis text:** as ruled under Q7.
- **The mapping to global DOFs,** including curved-only nodes.

**F. Product runs (records at checkpoint B, not tests)**
- **The corpus:** every product test and fixture that realizes a curved bend (Purpose); R5-4's E1–E6-class elbows; K-D5's skew-plane elbow cantilever at k_X = 8.5; K-D5's review UTM elbows (`REVIEW/_run_records/kd5_review/m31b/product_large_coordinates/`); and constructed curved mechanisms at about 1 m, 1 km and 5e6 m, each also with a nonlinear support added.
- **The runs:** both entries (captured `run_linear_static_preview_value_with_mode`, `PP:1613`; typed through headless `run_preview_in_memory_mode`, `:804`) and both modes, on `git archive` builds of Mac main and the candidate.
- **The record per run:** main's outcome, the candidate's, the code, the standing, the envelope sha256, and the change class.
  - **Any change outside K5-C1 to C3 stops the work.**
  - **Any change in an invocation with a nonlinear support stops the work.**
- **Under Q1:** the PP test file pins a curved mechanism refused as a physical mechanism, with its direction's exactness (not its bits, if the direction came through libm), on both entries and in both modes, and the nonlinear-support variant unchanged.

**G. Memory and scale**
- **A 10,000-sub-body tie chain** (about 20,000 nodes) at FK level, under a capped counting global allocator in its own test binary. Peak heap stays under a bound proposed at checkpoint 0, linear in the input. The cap makes a regression abort that binary instead of exhausting the host; such an abort is not a counted kill. No dense 6S×6S map exists.
- LINE-IN-CHAIN1000 at FK level or on sparse evidence only, never through `AssemblyEvidence::new` (Constraints).
- Record the debug wall times of B and G. If one exceeds about a minute, report it to ROOT; nothing is ignored or reduced.

## Mutants

Run from clean copies, with a NONE control first. Each mutant must be killed at a behavioural or pin assertion; name the killing test. A stack-overflow or allocator abort is not a kill. A survivor is a defect to report; never weaken a test to kill it. Where no admissible control exists, derive the equivalence and report it; ROOT rules.

**The design's §7.3 items that touch W4:**

| # | Mutant | Intended kill |
|---|---|---|
| 12 | Treat a user-element tie as a rigid link (no virtual shift) | B3 both ways; B2's NP-C-like case |
| 7 (W4 analogue) | Disable W4 (mixed bodies skipped as today) | B2's NP-C-like case at SA level; D's curved-only mechanism |

**K5's own:**

| # | Mutant | Intended kill |
|---|---|---|
| K5-M1 | Non-tree ties (cycle rows) ignored | B4 (a false witness where the cycle restrains the mode) |
| K5-M2 | The virtual shift's sign flipped (`x_b − x_a`) | B2 (NP-C-like); B3 |
| K5-M3 | A user element with a zero, negative or NaN stiffness accepted as a tie | C (the lateral = 0 case) |
| K5-M4 | An explicit or unmatched curved slot qualified | D |
| K5-M5 | (Q2(a)) The screen formed from the rounded binary64 product `K_e r_k`, or against one scalar allowance instead of per row | D (constructed boundary matrices) |
| K5-M6 | A witness published from the rounded candidate without the exact check | B5 (large offsets) |
| K5-M7 | The τ_B band dropped (`Restrained` whenever no exact witness exists) | B2 (the one-ulp near-collinear case) |
| K5-M8 | W4 reached from an unselected entry or from `solve_binary64` | E (the behavioural and text pins; the loop test) |
| K5-M9 | Frame-only bodies routed through the new function, or their node or ground order changed | A (RF-MECH statuses; direction bits base against candidate); T9 |
| K5-M10 | The edges' objective flag set for qualified curved or tied bodies | A (the `qualified_passive_family` pin; exact-block eligibility) |
| K5-M11 | `hypot` (or another function of unspecified precision) in the new screen | B10 (the source scan) |
| K5-M12 | Curved node coordinates taken from a frame when the macro source disagrees | D (the inconsistent-coordinates case) |
| K5-M13 | The force-scaled branch matches curved slots on b = 0 bits | D (2^b) |
| K5-M14 | Input order leaks into the rows or the candidates | B6 |
| — | Your own, at least two | — |

## Gates (ROOT runs the PR)

- **Suites.** FK's, NI's, PP's and headless's full suites, and CI's 39-manifest profile with `--no-fail-fast`, against ROOT's Mac baseline.
- **T9 (Mac-only).** The committed-fixture diff, built on this Mac from `git archive` copies of base and candidate and compared with each other.
  - 112 of 112 byte-identical is expected.
  - **Any committed-byte change stops the work.**
- **The both-entry gate:** as ruled under Q8 (recommended: not run, with the scan and the derivation in RETURN).
- **The product-run table (F),** base against candidate on the Mac, with every change classified.
- **An independent complete-diff review,** with oracles independent of K5's generator:
  - an exact rational null-space computation of the unreduced stacked map for every constructed case;
  - an independent re-derivation of the tree reduction, the energy zero sets and the frame-only byte identity;
  - the curved qualification rule as ruled, checked on the corpus;
  - a re-run of the loop pin's and K1's pins' original mutants.
- **Hosted CI** green on the candidate head. Record the numerical job's time.
- **DEC-025** under the owner's Mac decision (`OWNER_DIRECTION.md`, 2026-09-28):
  - the Mac sweep, whose only cargo failures are the three known platform tests, identical to Mac main;
  - pytest, vitest and the build pass;
  - hosted Linux CI's numerical cargo job supplies the clean Linux cargo run;
  - the deviation is recorded in the merge record.
- **No native witness is possible:** the desktop cannot realize a curved bend (R5-4), and no build realizes a joint. Record this.

## Checkpoints

End your turn at each one with a status for ROOT: the changed files, the results, and any stop. ROOT verifies, commits and resumes you.

- **0: a plan, before any product code.** It covers:
  - exact signatures: the FK function and its types, the SA helper, and the per-body record that carries the reason for an unqualified body;
  - **the reduction:** the spanning forest, the virtual positions, the cycle rows, the origin, and its equivalence with the stacked map, derived;
  - **the screen:**
    - row construction and normalization (a power-of-two characteristic length is recommended, so normalization is exact);
    - the Jacobi rotation without `hypot` (`sqrt(ζ² + 1)` with an overflow guard);
    - τ_B and its m (grounds plus 3 per cycle);
    - the candidate list and its canonical order;
    - the exact verification with `Expansion`;
    - the representability rule and the canonical representative;
    - the unresolved reason string;
  - the curved rule under Q2, where coordinates come from, and the matching (dense, sparse, 2^b);
  - the user validation and the T4 tripwire (Q5);
  - the wiring (Q1), with the list of entries, and the pin plan;
  - the basis text (Q7), with its exact proposed string;
  - the site-table plan (Q9): SA's accumulation counts before and after;
  - **the scans:** the gate's 222 requests and T9's inputs for a realized curved bend or user element (Q8), and the committed models with a joint flag;
  - **the product-run plan (F):** inputs, entries, modes and the change classes. **One Mac-main product run of a constructed curved mechanism** at the origin and at 5e6 m, to establish main's outcome before any code (ROOT's lesson: a claim about product behaviour needs a product run);
  - the generator, the vectors and their hashes;
  - the test list, the mutant list and the list of derivations;
  - your position on every open question still unresolved.
- **A1: the FK function.** A clean compile; B, C's FK part, G; the scans; a warning-free non-test build.
- **A2: the SA wiring.** C's SA part, D and E; the site tables; K-D5's, K1's and K2b's tests unchanged.
- **B:** the suites against the Mac baseline, T9, and the product-run table (F), with the change classes. ROOT rules on the table before C.
- **C:** the mutation table, with the NONE control first, and the original pins' mutants with their kill sites.
- **D:** CHANGE_RECORD (following `.agents/skills/chirality-change/SKILL.md`) and RETURN, with `_run_records/` and SHA256SUMS.

**Stop and report** (end your turn) on any of these:
- a committed-byte change in T9;
- any change on a frame-only body (status, direction bits or reason), or in an invocation with a nonlinear support;
- a product outcome outside K5-C1 to C3, or a prediction in this brief that a product run contradicts;
- a change to `qualified_passive_family`, exact-block eligibility or `permits_contact_seed_trial`'s behaviour;
- a needed edit outside the write set, above all `FK/structural.rs`, `FK/lib.rs`, `CB`, `NI lib.rs`, NI's `s11k_tests.rs`, PP source or PP's `s11f_site_test.rs`, or K4's or F1b's files;
- a site that fits no disposition, if Q9's extension is needed;
- a witness that cannot be verified exactly (never publish it);
- a reference mismatch (never edit a reference);
- a surviving mutant, or an original pin mutant no longer killed;
- a SIGKILL from the memory guard. Check `<wt>/guard/memguard.log`, and do not retry blindly.

## Return

- **Files:** `T3/IMPLEMENTATION/K5/` on the K5 branch: CHANGE_RECORD, RETURN, `_run_records/` and SHA256SUMS.
  - Use placeholders only (`<wt>`, `<scratch>`, `<VENV>`, `<home>`), with no machine paths and no model identifiers.
  - State the platform (`aarch64-apple-darwin`, rustc 1.97.1), and that T9 and the product runs are Mac-only comparisons against Mac main.
- **RETURN covers:**
  - the files and their line counts;
  - each item, with the design's words and the rulings;
  - the checkpoint-0 positions as ruled;
  - **the derivations:** the energy zero sets (frames, qualified curved elements, ties, grounds), the tree reduction, frame-only byte identity, "no invocation with a nonlinear support changes", the gate's byte identity (if Q8(a)), the screen's platform independence, and the soundness of the curved rule as ruled (Q2, Q3);
  - the libm note: `assess_rigid_body`'s `hypot` remains, it decides only a refusal and contributes no published value except a witness direction, which is compared base against candidate;
  - the product-run table with the change classes;
  - the screen ratios on the curved corpus (observations);
  - the mutation table;
  - T9 and the per-crate counts against the baseline;
  - the callers;
  - the toolchain and host;
  - the delegation mechanism;
  - what was not done.
- **RETURN has an "Interface for SA, F2a, F3, W1c, K4, V-K, T4 and T5" section with exact signatures,** as K1's §12 and K3's §14 did:
  - `assess_constrained_bodies` and its types; the SA helper and its per-body record;
  - the directional-ground API (Q6) and what K4's `geometry_first` must pass to use it;
  - the curved qualification rule and the tie rule, stated for T4, with the tripwire test's name;
  - for T5: what the loop would change if it adopted W4, and what `permits_contact_seed_trial` would then admit;
  - for F2a: the published outcomes (the `Mechanism` direction, the basis text) and how W1's geometry-first uses the function for W1b's user matrices (F3) and W1c's curved elements;
  - any `pub use` a consumer outside FK needs (FK's `rigid_body` is already `pub mod`).

## Open questions for ROOT (each with options and a recommendation)

**Q1. Where W4 applies (scope and wiring).** `geometry()` has 11 call sites, including the loop's `solve_binary64` (Scope 1).
- **(a) Every call site** (the design's "W4 generalizes it").
  - The loop and every invocation with a nonlinear support then change.
  - `permits_contact_seed_trial` admits any `Mechanism` unconditionally (`SA:1900`), so a witness opens the contact-seed path at `PP:2884` and `NI lib.rs:649`. That is T5's domain.
- **(b) Only the four `selected` branches** of the formation-checked entries (dense and sparse, unscaled and force-scaled). Every other call keeps today's frame-only geometry, pinned by a behavioural and a text test.
  - The product changes only for linear invocations with no nonlinear support that realize a curved bend (K5-C1 to C3).
  - Add a PP tests-only file for the product pins (write set, "proposed").
- **(c) Kernel only.** K5 adds the FK function and an SA helper with no product caller. The wiring moves to F2a or F3. The row's "SA (geometry …)" then waits.
- **Recommendation: (b), with the PP test file.**
  - It mirrors revision 5a.1 (a nonlinear case "keeps its ordinary result and standing exactly as today") and the selection's loop constraint.
  - It covers PP's route on both sides of F1b's merge.
  - It delivers the design's product benefit for curved mechanisms now.

**Q2. The curved qualification rule.** §4.9's screen tests the represented `K_e` against `curved_formation`'s allowance. But R5-4 (adopted after §4.9 was written) says that allowance "bounds only the symmetrization stages … It covers neither the trigonometric chord, the flexibility nor the inverse, and the chord mismatch is the dominant defect". The brief's emulation (Basis 13; not a product run) of the screen as written, on 90°, 45°, 10° and 2° elbows at R = 0.3 m:
- **within about 10 m of the origin it passes at φ ≥ 10°** (worst ratio 0.008 to 0.77). At φ = 2° it fails even at the origin (2.37);
- **from about 100 m it fails** (2.9 to 13); at 1 km, 17 to 810 except one 90° case (0.02); at 1e5 m, 273 to 6.4e4; at 2e6 to 5e6 m, 1.2e4 to 2.5e6;
- a ratio near 1 (0.77 at 10 m, 10°) can move with the platform's `sin` and `cos`, since the chord comes from them.

Options:
- **(a) As written.** Screen the represented matrix against the allowance, formed and compared exactly. Pass joins; fail leaves the body unqualified (today's behaviour).
  - It is safe, but it qualifies few real elbows.
  - It leaves unwitnessed exactly the class where a witness matters most: a mechanism held only by the element's formation error at large coordinates. The element's non-objectivity there is the T4 row's UTM finding. Whether main's matrix gate then passes such a mechanism is to be established by the checkpoint-0 product run.
- **(b) Qualify by construction.** A slot matched to its macro source joins its sub-body, because R5-4's intended element (H from the actual chord) is objective by construction, as frames are treated by their intended element today. The screen's ratio is computed exactly and recorded as evidence only. Explicit and unmatched slots stay unqualified.
- **(c) Both.** Matched and screened.
- **Recommendation: (b).**
  - It follows R5-4's adopted analysis, and makes the witness catch the mechanisms that matter.
  - Its product change is K5-C1: refusals of true mechanisms, some of which main publishes today along the mechanism.
  - That class is pre-registered, and confirmed or refuted by product runs at B.
  - If ROOT prefers the letter's conservatism, (c) keeps (a)'s coverage with the source condition.

**Q3. T4's confirmation of the curved null-space claim** (§4.9 "Coordination"; D-9 "T4 confirms the curved construction"; the T4 row's item is still PLANNED).
- **(a) Not a prerequisite,** as R5-4 established for K-D5: "T4's null-space confirmation is not a prerequisite: the intended element is objective by construction". R5-4 §5 adds that for the product's own element "the null space holds exactly only for arc-consistent geometry, which binary64 centres do not give". RETURN derives the claim for the intended element step by step, and the reviewer checks it. T4's item stays open for the product's element.
- **(b) A prerequisite.** K5 ships ties only; curved qualification waits for T4 (a later K5b).
- **Recommendation: (a).**

**Q4. The rank screen's platform dependence.** The existing screen uses `hypot` (`rigid_body.rs:52`, `:108`). On T9's inputs, 6 of the 16 macOS `hypot` arguments that differ from correctly rounded have the Jacobi shape `hypot(ζ, 1)`. No status changed, but a knife-edge τ_B decision or a candidate's bits could.
- **(a) Reuse the existing screen** for mixed bodies.
- **(b) A libm-free screen in the new function:** the same τ_B rule and Jacobi, with a power-of-two characteristic length and `sqrt` forms in place of `hypot`, and the witness verified exactly. `assess_rigid_body` stays byte-identical, and its `hypot` goes on the T3-close list.
- **(c) Make `assess_rigid_body` libm-free too.** That changes frame-only internals and possibly witness bits, so it needs the both-entry gate and K4's RF-MECH re-run.
- **(d) Exact rank with no τ_B band.**
  - It needs exact multiplication of multi-term values, which FK has only inside the private `retained` module.
  - It departs from the accepted policy that "a deficient/ambiguous numerical rank blocks ordinary solved publication pending either a witnessed rigid mode or stronger scrutiny" (`CONTINUATION_2026-09-24/CORRECTNESS_DESIGN/NUMERICAL_POLICY_REVIEW/RETURN.md` §3, a sibling of `T3/`; the τ_B row of `CORRECTNESS_DESIGN/NUMERICAL_REFERENCE.md`).
- **Recommendation: (b).**

**Q5. User ties.** No product solve realizes a user element (Purpose). Only PP's `#[cfg(test)]` historical scope does (joint C-150, lateral 9e5). A lateral = 0 element exists only by struct literal (K-D5's tests), and its zero-energy set is larger than a tie. T4's M07 repair will change the element.
- **(a) Implement per §4.9 for today's element,** with every stiffness required finite and positive (otherwise unqualified, with a reason), a T4 tripwire test, and a notice to T4 in RETURN.
- **(b) Defer ties to T4's repaired element.** K5 is then curved only, and §7.3 item 12 waits.
- **(c) Derive each tie from the element's actual null space** (a partial tie for lateral = 0). It covers an element the product cannot build.
- **Recommendation: (a).**

**Q6. Partial directional grounds** (K4's O1 amendment routes them to W4/K5; §4.9's grounds are DOF-indexed only).
- **(a) K5's function accepts directional ground rows** (node, kind, binary64 direction; the row `nᵀ[I, −skew(r)]`), tested at FK level (B9). Wiring K4's `geometry_first` to it is a follow-up after both merge (V-K, or a small K4 follow-up). K5 does not write K4's files.
- **(b) As (a), and K5 also wires K4's `factor.rs`** if K4 merges first, as a declared write-set extension.
- **(c) Not in K5.** It stays routed (T3-close list).
- **Recommendation: (a).** The API is cheap and exact; the wiring belongs with K4's files.

**Q7. The published basis text for mixed families.** `symmetry_basis` (`SA:1291-1298`) is rendered into every solved case's integrity text (`StructuralReport.symmetry_basis`). After K5, "physical rigid-null witness unqualified for bodies containing user/curved elements" is false on the selected branches.
- **(a) Leave it unchanged everywhere.** It is byte-identical, but it states something false; the inaccuracy is recorded.
- **(b) A new text for mixed families on the four selected branches only,** describing W4's qualification. The frame-only text, and every other entry, are unchanged. This is K5-C3, in non-committed outputs only (T9 is unaffected). The exact string is fixed at checkpoint 0.
- **Recommendation: (b).** Published evidence should be true, and the change is confined to cases no committed fixture holds.

**Q8. The both-entry gate.**
- **(a) Not run.** The corpus realizes no curved bend or user element (re-verified by scan), so under Q1(b) every run is byte-identical by construction. The evidence is T9, the RF-MECH base-against-candidate check and the product-run table (F).
- **(b) Part 1 only,** as a regression net (about 50 minutes or more per tree on the Mac, serialized against F1b's gate).
- **(c) The full gate.**
- **Recommendation: (a).** K1, K2b and K4 ran no gate because they changed no published byte. K5 does change published bytes (K5-C1 to C3), but only on paths the corpus does not reach, so the gate would show nothing. (b) if ROOT wants the net.

**Q9. The S11 site tables.** SA is in FK's `s11_site_table.rs` SOURCES (rows `:150-153`; rule 7 counts every `+=`, `-=`, `.sum(`, `fold(` and self-assignment fold per function name) and in PP's `s11f_site_test.rs` KERNEL list. Both files are now in other slices' write sets (K4's Q8, F1b's Q11).
- **(a) Keep K5's logic in `rigid_body.rs`** (not scanned; it carries no load, force or right-hand-side sum), with SA only plumbing data, so neither table changes. Any unavoidable SA row is an additive, declared extension of FK's table under K1's five conditions, coordinated with K4 (the second to merge rebases).
- **(b) Also add `rigid_body.rs` to SOURCES,** with geometry exemptions (K1's precedent for files carrying sums).
- **Recommendation: (a).** (b) can ride a later tests slice after K4 merges, if ROOT wants the coverage.

**Q10. The K2a T3-close item routed "to K5 or its own slice":** whether curved-element formation (`curved_bend`, `arc_model`) or another stiffness-forming path can produce K2a's unbounded zero or wrong rounding from 1/L²- or 1/L³-scaled terms.
- **(a) Not K5.** It stays on the T3-close list for its own investigation. `CB` is outside K5's row, and W4 is geometry.
- **(b) K5 takes an investigation-only part:** a caller scan and probes on its curved corpus, no code, reported in RETURN.
- **Recommendation: (a),** with K5 recording, as an observation, any zero or subnormal curved entry its corpus meets.

**Q11. One slice, or two.**
- **Recommendation: one slice,** with checkpoint A split into A1 (the FK function) and A2 (the SA wiring and the curved rule). The PR is a few hundred lines of product code plus tests. Two PRs would add a review and a T9 cycle for little attribution gain.

## Design text made stale or inconsistent by merged slices (for ROOT; `DESIGN.md` stays hash-pinned)

None of these is yet recorded as a ruling.

1. **§4.9's and §2.5's citations have drifted** (at `24dea2dae`):
   - the family skip `SA:208-217` is now `BodyEvidence::geometry` `SA:1212-1284` (skip at `:1241-1250`);
   - the basis text `SA:264` is now `symmetry_basis` `:1291-1298`;
   - `curved_formation` `SA:291-402` is now `:1781-1892`;
   - `FK/lib.rs:635-638, 1021-1043` are now `:649-652` and `user_stiffness_local_matrix` `:1739-1761`;
   - `curved_bend/src/lib.rs:241-251` is now `:242-252`;
   - `assess_rigid_body` is unchanged at `FK/rigid_body.rs:33`.
2. **§4.9's SUP-17 paragraph** (`PP:1604`) was delivered by F1a (PR #1025). It is no longer W4's.
3. **§4.9 does not say which entries W4 applies to.** Since K1, `BodyEvidence` serves both evidences, and `geometry()` is called at 11 sites, including the loop's `solve_binary64`. A `Mechanism` admits the contact-seed trial unconditionally (`SA:1900`). See Q1.
4. **§4.9's curved screen against R5-4 (§4.3.1, revision 5a):** `curved_formation` does not bound the chord mismatch, which dominates. The screen as written qualifies elbows only near the model origin (the brief's emulation). See Q2.
5. **§4.9 "Coordination" and D-9 ("T4 confirms the curved construction")** against R5-4 §5 ("the null space holds exactly only for arc-consistent geometry, which binary64 centres do not give") and §4.3.1 ("T4's null-space confirmation is not a prerequisite"). T4's item is still PLANNED. See Q3.
6. **§4.9's ties ("every stiffness is positive") against §4.3.1 and D5C-2's "user-stiffness elements (lateral zero) … the only realized form on the ordinary route".** FK's `UserStiffnessElement::new` refuses lateral = 0 (`FK/lib.rs:650`), and PP builds joints only through `new` (`PP:5864`), after refusing lateral ≠ 0 (`PP:1740`). **So the ordinary route realizes no user element,** and a lateral = 0 element exists only by struct literal. `kd5_tests.rs:591-592` repeats the stale claim. See Q5.
7. **T4's pending M07 repair** (the joint's lateral springs "without the rigid-body moment coupling") will change the user element's zero-energy set. §4.9's tie rule is for today's element. See Q5 (the tripwire).
8. **§4.9's "the existing SVD rank screen and τ_B form" is platform-dependent** through `hypot` (the Mac calibration's Jacobi rows). See Q4.
9. **§4.9 gives no data path for curved node coordinates.** `CurvedBendStiffnessElement` carries none (`NI lib.rs:170-177`), and SA records coordinates from frames and users only. They are available only from `curved_sources`, which only the four formation-checked entries receive. Q1(b) and Q2 use that.
10. **§4.9's "with a reason" has no carrier.** `StructuralReport` is `Debug`-published (D5C-3), so a field there would change every published report. The reason lives in K5's own return types.
11. **The K5 row's write set omits the site tests that scan SA:** FK's `s11_site_table.rs` and PP's `s11f_site_test.rs`, both now in other slices' write sets. See Q9.
12. **§6's "S11-K, K-D5, K1, K2b and K5 share `SA` and `FK/structural.rs`, so they stay serialized"** (and D-10): K2b has merged, K5 needs no `FK/structural.rs` edit, and K4 and F1b make no SA edit. So K5 can run in parallel with both.
13. **§4.1.3's "W4 generalizes [geometry first]":** K4's W1a calls `assess_rigid_body` (frames). W1b's user matrices (F3) and W1c's curved elements will need K5's function. See the Return interface.
14. **K4's O1 amendment routes non-spanning directional grounds to W4,** but §4.9's grounds are DOF-indexed only. See Q6.
15. **The work graph routes the curved-formation T3-close item "to K5 or its own slice",** but the K5 row has no formation scope. See Q10.
16. **§7.2's and §4.10's RF-MECH** have no user or curved mechanism (R1 is frames and springs), so W4 has no frozen reference. Its tests are constructed (the row's list) with an exact generator, and NP-C's `near_collinear_rows` are data, not a model.
17. **The K5 row's "a curved screen positive and a seeded negative"** presumes the screen gates. Under Q2(b), the seeded negative becomes an explicit or unmatched slot, and the screen is recorded evidence.

**Recommendation:** ROOT records items 1–17 as rulings when it rules on this brief, as it did for K4's and F1b's lists. `DESIGN.md` is not edited.
