# T3 D1 — numerical core: general accuracy, range and sparse scale

HELPS_HUMANS-style design record (TASK D1) for the T3 WORKING_ITEMS manager and ROOT, revision 1, 2026-09-26. It stays a proposal until ROOT selects it after the independent review (V1).

- **Basis.** T3 branch `codex/piping-numerical-integrity-20260926` at `e14f7fd13`. The product basis is main `c61a540ea`. Line numbers are at `c61a540ea` and will drift.
- **T1.** Candidate `f3270ea79`, read only with `git show` and `git diff 82b43f9bd f3270ea79`. T1 changes no file under `P/core/solver/**` or `P/validation/benchmarks/**`. In `PP` it does not touch the line that SUP-17 names.
- **Paths.** `P/` means `projects/chirality-piping/`. `PP` means `P/core/product_physics/src/lib.rs`. `FK` means `P/core/solver/frame_kernel/src/`. `SA` means `P/core/solver/nonlinear_integration/src/structural_adapter.rs`. `T3/` is this tranche's records folder.
- **Roles read.** Root `AGENTS.md` (sha256 `c8ce87ef…`), `agents/AGENT_TASK.md` (`1a13a5b0…`). I also consulted `agents/AGENT_HELPS_HUMANS.md` (`a0c9fb94…`) deliberately, for the design posture, as the brief asked.
- **What I did not do.** I changed no product source, test, fixture, reference or other record, and ran no Git write. I ran no cargo build, test or Rust probe; the host is held. I ran two standard-library Python probes at low priority, and read public crates.io metadata (§10).

## 1. Recommendation in brief

**W1, general accuracy.** Implement the selected contribution-preserving multiprecision method (`CORRECTNESS_DESIGN/CONTRIBUTION_PRECISION/CONTRACT.md`) as the one method for N05-class accuracy and for general retained-source recovery. Add four concrete choices to it:

1. **Formation.** Rebuild every element at precision p from its binary64 primitives, in basic-deformation form, `K_e = Bᵀ D B`. The probe (§3) shows why: the rounded binary64 element matrix breaks the element's rigid-body null space, even for an axis-aligned member.
2. **Solver.** Use a sparse profile LDLᵀ at precision p, on the same sparse structure W3 introduces.
3. **Stop rule.** A candidate at p is accepted only when a fresh solve at 2p agrees with it on every published quantity. The screen is `2^-64` of the connected body's scale for that kind of quantity (§4.1.6). ROOT registers this constant as method policy. It is not a comparison tolerance.
4. **Arithmetic.** Write a small, dependency-free fixed-limb binary float type in `frame_kernel`. No lockfile changes.

Two alternatives are rejected. Generalizing the merged exact-block method is exact on the wrong system: the probe finds scaled errors of 5e-6 to 1e-3 against the 1e-9 criterion, and finds a matrix that is not positive definite for N06-class skewed cases. A global relative-coordinate reformulation is deferred.

**Coverage.** Three phases.
- **W1a:** straight frames, global-axis linear springs, rigid restraints, nodal loads, and combinations through T0R's gates.
- **W1b:** element uniform loads, thermal eigen axial loads, pressure thrust on straight members, constant effort, prescribed support motion (0.4.0, after T1) and user matrices.
- **W1c:** curved bends, with T4.

Nonlinear and contact recovery stays with T5. Every excluded family is refused per case with a named reason; the case keeps its ordinary standing.

**Triggers and identities.**
- The method runs automatically, per case, on the ordinary route and on the exact route, whenever the ordinary M03 outcome is not `Passed`. Witnessed mechanisms, negative energy, asymmetry and invalid input never trigger it.
- It publishes under proposed successor identities that allow mixed envelopes (placeholders for ROOT, §4.4). No invocation-level failure path exists for them.
- I recommend retiring exact-block selection for fresh solves. The exact-block method stays as an independent oracle in tests.

**W2, range.** Scale the ordinary structural system exactly, by a power of two in force units, inside the kernel. The scale exponent is 0 whenever today's unscaled evaluation succeeds, so every current result stays bit-identical. The PHYS-R4 refusal is removed. The admitted range and its refusal reason are defined in §4.7. The W1 path has an internal 64-bit exponent.

**W3, sparse scale (M32).**
- One sparse pattern holds assembly, the M03 gate, reduction, reactions and the nonlinear linearized solves.
- Dense scrutiny stays as an explicitly selected mode, built from the same sparse values, with a resource guard.
- There is no automatic dense fallback. Main has none today either.
- Parity and memory protocols are in §4.8.

**W4, M03 residual items.**
- A constrained-body null-space witness covers bodies that contain user matrices, and curved elements that pass an objectivity screen.
- The SUP-17 text is reworded.

**W5, VP-ROBUST.** A new validation crate, `P/validation/benchmarks/numerical_robustness/`, with two lanes:
- a kernel lane, which can run before T1 merges;
- a product lane, through the public entry in both modes.

Seeded faults sit behind a kernel feature that product builds never enable. A scale runner measures peak memory in fresh processes.

**No owner decision is needed** if ROOT selects this. If ROOT prefers `rug` (GMP/MPFR), linking LGPL-3.0+ code into the MIT-licensed signed macOS app becomes a governance question (§9).

## 2. What main does today

### 2.1 The ordinary solve is dense from end to end

| Stage | Site | Shape |
|---|---|---|
| Global assembly | `FK/lib.rs:768` (`vec![vec![0.0; total_dofs]; total_dofs]`); called at `PP:1378`, and again per modulus basis at `PP:1452` | dense n×n |
| Evidence | `SA:35` `absolute_roundoff` and `operation_counts`, plus `n×n` temporaries (`SA:117-118`) | dense n×n ×2 |
| Reduction | `FK/lib.rs:867`; called at `PP:1855` | dense n_f×n_f |
| M03 gate input | `structural.rs:26` `stiffness: &[Vec<f64>]`; `validate` audits all n² pairs (`:243-263`); `prepare_structural` builds dense `a` (`:597`) | dense |
| Contribution audit | `contribution_sums` allocates `n×n` `Expansion`s (`:402`), and `contribution_differences` clones them (`:423`). `audit_intended_action` allocates them again (`:517`) | 32 bytes per entry, twice |
| "Sparse" factor | `sparse_direct/src/structural.rs:10,14` builds adjacency and profile from the dense prepared matrix | dense input |
| Residual | `evaluate_original_residual` walks full dense rows (`:717`) | O(n²) per evaluation |
| Negative witness | `negative_pair_witness` tries every pair (`:1335-1336`), each with an O(n²) check (`:1299-1300`) | O(n⁴) on the failure path |
| Reactions | `multiply_matrix_vector(stiffness, …)` (`PP:2135`) | dense |

Reading the code, a peak of at least about 100 bytes per n² entry follows: dense K, reduced K, two evidence arrays, the prepared matrix, and two Expansion arrays. This is an estimate from the source, not a measurement. For a straight chain of m members (n = 6(m+1)):

| Members | n | Dense-path lower bound |
|---:|---:|---:|
| 10 | 66 | 0.45 MB |
| 100 | 606 | 38 MB |
| 1,000 | 6,006 | 3.8 GB |
| 10,000 | 60,006 | 374 GB |

RF-LARGE at 1,000 and 10,000 members cannot run today in either mode. The DEC-053 observation set stops at a 48-member chain and a 7×8 grid (`P/validation/benchmarks/sparse_default_promotion_observation.dec053.json`).

**Fallback.** No dense fallback runs today. `dense_fallback_message` is only ever `None` (`PP:3418`). Mode code 3 is therefore never emitted, and `LinearSolveMode::solution_basis(true)` is never called (`nonlinear_integration/src/lib.rs:52-58`, called only with `false` at `:2085` and `:2148`).

### 2.2 Accuracy

- The ordinary gate is binary64 throughout: `PreparedSystem.matrix: Vec<Vec<f64>>`, the Cholesky and profile factors, and `StructuralSolution.displacements: Vec<f64>`.
- An absorbed positive diagonal contribution is rejected as unresolved (`structural.rs:410-414`).
- The condition boundary is `rcond ≤ EPSILON`, which is unresolved (`:868`); `rcond < √EPSILON` makes the result Sensitive (`:934`).
- Retained-source recovery runs only when the ordinary result is Sensitive or rejected, a capture exists, there are no nonlinear supports and no combinations (`PP:1879-1885`).

### 2.3 The exact-block method's scope

- Every free connected block has order ≤ 2 (`FK/structural/exact_boundary.rs:419-420`).
- Systems have at most 256 DOFs (`:292`).
- The arithmetic is exact expansions on the represented binary64 contributions.
- Transforms must be signed permutations (`source_recovery.rs:303`).
- Nodal loads only (`:470`), straight frames only (`:435`).

Contributions are rounded global element entries. For axis-aligned members they equal the local entries. The local bending coefficients `12EI/L³`, `6EI/L²`, `4EI/L` and `2EI/L` are each rounded independently (`FK/lib.rs:719-726`). A bending soft mode needs at least three coupled DOFs, so I believe the order ≤ 2 limit is what keeps these out of scope today. V1 should check that reasoning.

### 2.4 Range (PHYS-R4)

`structural::transform_roundoff` (`structural.rs:1248-1283`) rejects any subnormal result through `checked_value`. For the PHYS-R4 inputs (E = 1 Pa, OD 4e-77 m, wall 1e-77 m, L = 1 m), every stiffness entry is normal, but the allowance at `[UY,UY]` is 7.53e-321 (`ENGINE_INTEGRATION/RETURN.md`). By my hand arithmetic: EA/L ≈ 9.4e-154, 12EI/L³ ≈ 1.4e-306 and GJ/L ≈ 1.1e-307 N·m. The allowance is about γ(24) ≈ 2.7e-15 times the latter entries. The refusal is correct inside a normal-range error model. Storing the allowance as a subnormal would lose bits and could round it down.

### 2.5 M03 residual items

- **Rigid-null witness.** `AssemblyEvidence::geometry` skips any body that contains a user or curved edge (`SA:208-217`). Its basis text says so (`SA:264`).
- **User elements.** Their energy is `Σ k_d (Δ_d)²` over six relative local DOFs, and all four stiffnesses must be positive (`FK/lib.rs:635-638`, `1021-1043`). The zero-energy set is therefore "both nodes share translation and rotation". That is not the rigid-motion set when the element has length.
- **Curved elements.** They are built from an inverted tip flexibility with a rigid chord transfer (`curved_bend/src/lib.rs:241-251`). That construction has the rigid-motion null space in exact arithmetic. The represented matrix is not screened for it.
- **SUP-17.** `PP:1362` names "missing global rigid-body DOF classes". This precheck counts directly restrained DOF classes (`PP:10813-10835`). One test pins the old text (`PP:19995`).

### 2.6 Dependencies and CI

- `frame_kernel` has no dependencies at all. Its lockfile has one package. `sparse_direct` has two and `nonlinear_integration` eight, all of them path crates.
- 23 product lockfiles contain `open_pipe_stress_frame_kernel`. The list is in §4.11.
- CI's numerical job (`.github/workflows/piping-desktop-e2e.yml:172-204`, `P/tools/ci/numerical_ci.py:27-47`) runs `cargo fetch --locked` for every manifest found under `core/` and `validation/benchmarks/`. It then runs `cargo test --offline --locked`. A new crate under `validation/benchmarks/` is picked up automatically and needs its own `Cargo.lock`.
- Releases target macOS arm64 (`desktop-release-template.yml:30,297`). The project licence is MIT (`docs/CONTRACT.md` OPS-K-GOV-1).

## 3. Options for the general accuracy method (W1)

| | (A) Contribution-preserving multiprecision (selected basis) | (B) Global relative-coordinate or basic-deformation reformulation | (C) Generalized exact-block |
|---|---|---|---|
| Model solved | The intended binary64-input model, re-formed at p | The intended model, in transformed coordinates | The represented binary64 contributions, exactly |
| Skewed members | Yes | Yes | No: exact on a model whose rigid null space is broken by O(u·a) |
| Order > 2, weak coupling | Yes, with a sparse factor | Needs cycle compatibility, supports and multi-point constraints in the basis | Exact elimination on expansions; expansion growth and a 256-DOF, 256-term budget |
| Scale | Sparse profile at p. Scalars cost more, but the method runs only on triggered cases | Transformation fill and conditioning unknown | Not feasible beyond small blocks |
| Range | Internal 64-bit exponent | binary64 | Checked binary64 range |
| Reuse | `Expansion` audit, M03 structure, sparse ordering | Little | All of `exact_boundary` |
| Evidence | Refutation and prototype (2-coordinate); this probe (3D, §3.1) | Scalar oracle only | This probe refutes it |

### 3.1 Probe evidence (standard-library Python, `_run_records/probe_skew_precision.py`)

**Cases.** One or six members of the N-series section, so every coordinate is an exact integer and every frame axis is an exact rational. The root node has its translations fixed and its rotations held only by three global rotational springs k. A moment acts at the tip. The reference is exact rational arithmetic from the binary64-decoded section and load inputs.

**Error measure.** Each cell is the worst `|obs − exp| / max(|exp|, S)` at binary64 publication. S is the body-level coupled scale of §4.1.6, used here as a zero scale for convenience only; R1 owns the real zero scales. "Fail" means above 1e-9.

| Case | Exact solve of represented binary64 contributions (C) | Rounded matrix promoted to 128 bits | Ordinary binary64 | Primitive rebuild, p = 128 | Stop rule |
|---|---|---|---|---|---|
| Axis-aligned, k = 1e-4 (bending soft mode) | 1.05e-5 (8 of 22 fail) | 1.05e-5 | 1.06e-5 | 7.5e-28 | 128 accepted (8.1e-28) |
| Skew (3,4,0), k = 100 (ordinary scale) | 4.7e-12 | 4.7e-12 | 9.4e-12 | 1.7e-34 | 128 accepted |
| Skew (3,4,0), k = 1e-4 (N05 class) | 5.5e-6 (10 fail) | 5.5e-6 | 8.7e-6 | 0 | 128 accepted (1.3e-28) |
| Skew (3,4,0), k = 1e-12 (N06 class) | nonpositive pivot | nonpositive pivot | nonpositive pivot | 0 | 128 accepted (1.9e-20) |
| Oblique (2,3,6), k = 1e-4 | 1.07e-3 (19 fail) | 1.07e-3 | 2.7e-3 | 2.6e-25 | 128 accepted |
| Skew, k = 1e-28 (arithmetic stress) | nonpositive pivot | nonpositive pivot | nonpositive pivot | 1.8e-4 (10 fail); the pivot screen passes, margin 3.4 | 128 rejected (1.8e-4). p = 256 has error 5.0e-43 and is accepted against 512 (5.3e-43) |
| Six-member skew run, k = 1e-12 (39 free DOFs in one block) | nonpositive pivot | nonpositive pivot | nonpositive pivot | 2.1e-19 | 128 rejected (2.1e-19 > 2^-64 ≈ 5.4e-20). p = 256 has error 0; its 512 verification was not run |
| k = 0, a genuine mechanism | — | — | — | nonpositive pivot at 128, 256 and 512 (screen margins −0.003 to −0.005) | never solved |

**What follows:**
- **(C) is refuted.** The exact solution of the represented binary64 contributions has scaled errors of 5e-6 to 1e-3 against the 1e-9 criterion. For N06-class skewed cases, the represented matrix is not even positive definite in exact arithmetic. Promoting the rounded matrix changes nothing.
- **An axis-aligned transform is not enough.** In the first row the transform is a signed permutation. The error comes from the independently rounded bending coefficients, which break the element's rigid-rotation null space.
- **Primitive basic-deformation formation meets 1e-9** at 128 bits, with more than 15 orders of margin, for N05 and N06 classes, axis-aligned, skewed and oblique, and for a block of order 39.
- **The pivot screen alone is not enough.** At k = 1e-28 and 128 bits it passes (margin 3.4), yet the answer is 1.8e-4 wrong. The 2p agreement rule catches this. This is why the stop rule is required, not optional.
- **The stop rule errs on the safe side.** It escalated the six-member case, whose 128-bit answer already met 1e-9. It never accepted a failing candidate. It costs time, not correctness.
- **Precision does not regularize a mechanism.**

This is the evidence the refutation lacked ("a numerical analogue, not a deployed spatial-frame test"). It still is not Rust, MPFR, the product, or a performance result.

**Recommendation.** (A), with basic-deformation element formation. Keep (C) as a kernel test oracle within its merged scope. Defer (B)'s global transformation. Its useful insight, keeping each element's deformation coordinates, is already taken up by (A)'s formation.

## 4. Recommended design

### 4.1 W1 — the retained-precision structural method

Proposed names, all placeholders for ROOT:
- method token `contribution_preserving_multiprecision_v1`;
- policy `M03-INTEGRITY-MP-v1`, a precision-aware reading of M03-INTEGRITY-v1;
- diagnostics `RETAINED_PRECISION_SELECTED` and `RETAINED_PRECISION_UNAVAILABLE`.

#### 4.1.1 Kernel home and types (new files under `FK/structural/retained/`)

- **`wide.rs`: the arithmetic (§4.11).**
  - `Wide<const L: usize>`: sign, `i64` exponent and `[u64; L]` significand, with L = 2, 4, 8, 16 for 128, 256, 512 and 1024 bits.
  - `+ − × ÷ √`, each rounded to nearest, ties to even, at a runtime precision p ≤ 64L.
  - Exact conversion from `f64`.
  - Correctly rounded conversion to `f64`, with an explicit outcome: normal, subnormal (with its relative precision bound), underflow to zero, or overflow.
  - A work counter.
  - Integer-only arithmetic, so results are bitwise reproducible across platforms.
- **`source.rs`: `PrimitiveSource`,** an immutable per-case declaration with an identity digest.
  - Node coordinates (binary64).
  - `StraightMember {node_i, node_j, E, G, A, Iy, Iz, J, y_reference}`, all binary64 operands.
  - Springs `(dof, k > 0)`.
  - Constraints `(dof, value)`.
  - Nodal loads `(dof, value, source_id)`.
  - W1b adds member uniform loads, eigen axial forces, end thrusts, constant-effort forces and user relative-DOF springs.
  - Only supported families can be constructed, so exclusion is a type-level fact.
  - Validation rejects nonfinite or nonpositive properties and incomplete partitions. It also rejects any derived primitive that is subnormal, because its bits were lost before this boundary.
- **`assemble.rs`, `factor.rs`, `recover.rs`, `adaptive.rs`:** formation, assembly and reduction; the profile LDLᵀ and screens; recovery; the schedule, stop rule and evidence.
- **The opaque result.** `RetainedSolve` is bound to its source and precision. `RetainedSolve::publish()` rounds each quantity once. `RetainedSolve::combine(&[(factor, &RetainedSolve)])` forms combinations at p on the same operator.

No caller can supply a matrix, factor, closure or label. This mirrors the KREV trust boundary of `finish_structural` (`structural.rs:1086-1107`).

#### 4.1.2 Formation, assembly and reduction at precision p

Every operation below is rounded to p bits, and every input is a binary64 value lifted exactly.

1. **Frame.** `d = x_j − x_i`, `L = √(d·d)`, `e_x = d/L`. Gram-Schmidt of `y_reference` gives `e_y`, normalized, and `e_z = e_x × e_y`, normalized. This is the product's own algorithm (`FK/lib.rs:511-525`), so the axes converge to the exact axes of the binary64 geometry. No axis tolerance is used at p.
2. **Basic deformations.** `B_local` (6×12) holds the axial extension, the twist, and the end rotations relative to the chord in both local planes, using `1/L`. `B = B_local·T`.
3. **Constitutive operator.** `D` (6×6) is `diag(EA/L, GJ/L) ⊕ (EI_z/L)[[4,2],[2,4]] ⊕ (EI_y/L)[[4,2],[2,4]]`. `Bᵀ D B` reproduces the product's local matrix; the probe agrees to 1.3e-16 in binary64, with the same zero pattern.
4. **Assembly.** `K_e = Bᵀ D B` is added into the sparse pattern (W3) in a fixed element order. Springs are added at p.
5. **Reduction.** Free and prescribed maps come from the source. `rhs = f_f − K_fc u_c` at p. Neither K nor rhs is ever rounded back to binary64.

Because `B·r = 0` holds exactly for every rigid motion r of the exact geometry, the artificial rigid-mode stiffness is O(2^-p·a), not O(2^-53·a).

#### 4.1.3 Factor, screens and mechanism handling

- **Geometry first.** The geometric rigid-body assessment runs before any factor (W4 generalizes it). A witnessed mechanism is refused and never escalated.
- **Factor.** RCM ordering from the pattern. The ordering is integer data, shared with the binary64 sparse path. Profile LDLᵀ at p.
- **Pivot screen** at precision p: `d_i > 64·γ_p(m_i)·c_i`, where `γ_p(m) = m·2^-p/(1 − m·2^-p)`. This is the M03 structure with `u_p = 2^-p`. A failed pivot at p escalates to the next p; at the ceiling the result is unresolved. It is never read as a mechanism.
- **Negative energy.** Checked for pattern pairs only, at p, against the intended K.
- **Condition estimate.** Hager–Higham with the p-factor. `rcond ≤ 2^-(p-1)` counts as unresolved at p and escalates. The estimate is published as model information, with the label "sensitivity to matrix-entry perturbation, not to authored parameters".

#### 4.1.4 Solve and refinement

1. Solve with the p-factor.
2. Evaluate `r = f − K u` against the intended system re-formed at p + 64, from the same primitives.
3. Gate each free row with the componentwise guarded ratio against `64·γ_p(m_i)`.
4. Allow at most three corrections with the p-factor, as today (`structural.rs:960`).
5. If it still fails, escalate.

#### 4.1.5 Recovery before rounding

For each member, at precision p:
- `d_local = T u`;
- `e = B_local d_local`;
- `Q = D e`;
- end actions (node-on-element, local) = `B_localᵀ Q`;
- station actions, which are linear for nodal loads (W1b adds the load term at p).

Also at p: spring actions `−k u`, and reactions `(K u − f)` on the constrained rows. Only then is each published quantity rounded once, with its representability outcome.

**Derived stresses** keep the existing binary64 publication methods: the circular maximum `exact_straight_summary_extrema`, and stations. They are applied to the once-rounded actions. This is a second rounding, but of a well-conditioned function: a sum of non-negative terms. It is not a recovery from rounded displacements. The row provenance says so.

#### 4.1.6 Adaptive schedule and stop rule

**Schedule.** Candidates at p = 128, 256 and 512, each verified at 2p. The ceiling is 1024 bits, as CONTRACT requires. The verification solve at 2p repeats formation from the binary64 operands; it does not reuse p quantities. Solves are reused: a rejected 128 candidate's 256 verification becomes the next candidate. So at most four solves run: 128, 256, 512 and 1024.

**Stop rule** (proposed; ROOT registers it as method policy):
- **Accept p** when every published quantity q satisfies `|q_p − q_2p| ≤ 2^-64 · max(|q_2p|, S*)`.
- **The scale S\*** is built per connected body and per kind, where the kinds are translation, rotation, force and moment. Let S(kind) be the largest |q| of that kind in the body at 2p, and L_b the body's node extent. Then:
  - translation: `S* = max(S(translation), L_b·S(rotation))`;
  - rotation: `S* = max(S(rotation), S(translation)/L_b)`;
  - force (member and reaction forces): `S* = max(S(force), S(moment)/L_b)`;
  - moment: `S* = max(S(moment), L_b·S(force))`.
- **A body with all scales zero** is unloaded and unmoving, and must agree exactly.
- **Rationale.** `2^-64` is the binary64 significand plus 11 guard bits. The coupling gives a structurally zero kind a physical scale, so noise can be accepted. The probe (§3.1) shows that per-group scales without this coupling never accept the noise in structural zeros.
- **Stricter option for ROOT.** Per-member scales with a body-level floor. It verifies small actions against their own member, but forces 256 bits on many skewed soft models.
- **What it is not.** The stop rule is operational evidence of numerical convergence, not a forward-error enclosure. It changes no comparison criterion.

**Failure.**
- At the ceiling, or when the budget runs out, the case is unresolved: `RETAINED_PRECISION_UNAVAILABLE` with the attempted precisions and the reason.
- The case keeps its ordinary outcome and standing.
- Nothing is relabelled as solved.

#### 4.1.7 Budgets and reuse

- **Work units.** Counted in limb-multiply equivalents per attempt. Successful, failed and verification work is all charged.
- **Limits.** ROOT selects the per-case and per-invocation limits from the W3/W5 measurements, as `COMPOSITE_ENGINE/RESOURCE_POLICY.md` did for physics-source-1. No implementing slice ships without them.
- **Factor reuse.** Linear cases that share a modulus basis and state share one p-factor per precision, with multiple right-hand sides.

#### 4.1.8 Determinism and replay

- The arithmetic is integer-only and the ordering is fixed. The same `PrimitiveSource` therefore gives a bit-identical retained state on every platform.
- A reader in Rust can replay a captured invocation and compare a retained-state digest. That digest is sha256 over the canonical limbs of u_p and every member's Q.
- The full p-state is not persisted by default; replay reproduces it (§5).

### 4.2 Coverage and refusals

| Family | W1a | W1b | W1c | Refusal reason while excluded |
|---|---|---|---|---|
| Straight frame members | yes | | | — |
| Global-axis linear springs (k > 0) | yes | | | — |
| Rigid restraints, zero value | yes | | | — |
| Nonzero prescribed support motion (0.4.0 `ResolvedCase`) | kernel yes | facade, after T1 | | `prescribed motion unsupported` |
| Nodal forces and moments | yes | | | — |
| Combinations (T0R-admitted `mechanics`, subtraction, range) | at the facade, from retained states at p; T0R gates unchanged | | | `combination withheld by gate` (existing codes) |
| Element uniform loads, including weight | | yes | | `element load producer unsupported` |
| Thermal eigen axial load (0.4.0 resolved) | | yes | | `thermal producer unsupported` |
| Pressure thrust on straight members; exact-route pressure regions | | thrust yes; regions in W1c | yes | `pressure producer unsupported` |
| Constant-effort support forces | | yes | | `constant-effort producer unsupported` |
| User stiffness elements | | yes (relative-DOF B form) | | `user-matrix element unsupported` |
| Curved bend macro elements | | | with T4 | `curved element unsupported` |
| Components, releases | | | later | `component or release unsupported` |
| Equivalent static | | | later | `equivalent-static unsupported` |
| Nonlinear or contact supports | T5 | T5 | T5 | `nonlinear support family (T5)` |
| Legacy `imposed_displacement` | refused upstream (T0R SF-E) | | | — |

**How a refusal works.** One info diagnostic per case, `RETAINED_PRECISION_UNAVAILABLE`, naming the family, the model entity and the case. The case keeps its ordinary rows and standing, so a Sensitive case is withheld from Current as today.

**The effect of this phasing.** Real piping models almost always carry weight and elbows. W1a therefore repairs the N05 class and the frozen references, not most real models. W1b and W1c are required before "general accuracy" can be claimed for typical models (§8).

**References.** R1's brief covers nodal loads only. W1b needs an R1 addendum (RF-ELOAD) with independent references for uniform, thermal and thrust loads before it is implemented (§9, D-11).

### 4.3 When the method runs

- **Routes.** The ordinary route (models 0.1.0 and 0.2.0, and 0.3.0 `legacy_pressure_v1` with zero pressure) and the exact route (0.3.0 exact; W1a requires an explicitly empty pressure-region list, as physics-source-1 does). The 0.4.0 load-state route follows after T1 merges (W1b).
- **Trigger**, per case: a captured invocation exists, and the ordinary attempt ended in one of:
  - `SolveQuality::Sensitive`;
  - `NumericallyUnresolved` for an absorbed contribution, assembly amplification, a condition estimate at the working boundary, a failed intended action or original residual, or an unresolved pivot;
  - a `Range` error that W2 scaling could not resolve.
- **Never triggered by** `Mechanism`, `NegativeEnergy`, `Asymmetric` or `InvalidInput`.
- **Not run by default on Passed cases.** The contract keeps the ordinary path "for cases it actually resolves under the existing policy". A Passed binary64 result can still carry a normwise estimate up to about κ·u < 1e-8 (with κ < 1/√EPSILON). If the detection run P1 finds any R1 case that is Passed yet misses 1e-9, ROOT should consider moving the trigger (§9, D-5).
- **The ordinary attempt always runs first.** Its M03 report is kept as evidence, in the same place `OrdinaryAttempt` sits today (`PP:1869-1872`).

### 4.4 Identities and what happens to existing ones

Proposals only; ROOT reserves names and versions.

| Identity | After D1 |
|---|---|
| `preview-physics-1` | Byte-unchanged for every envelope in which no case selects the new method |
| `<preview-retained>`, suggested `openpipestress.result_semantics/0.3.0/preview-physics-retained-1` | Ordinary route, emitted when at least one case selects the method. It inherits the preview-physics-1 table and adds a closed per-case receipt (§5). Other cases are rendered under preview-physics-1 semantics, with their ordinary standing |
| `<physics-retained>`, suggested `.../physics-retained-1` | The exact-route equivalent, inheriting physics-1 semantics |
| `source-blocks-1`, `physics-source-1` | Readers, fixtures and hashes unchanged. **Recommended (D-4 option A):** no longer emitted for fresh solves. The exact-block method stays a kernel test oracle: in its scope, the new method's rows must match its projections within the unchanged criterion |
| `load-reference-source-1` (T1) | Unchanged by D1. Switching it waits for T1's merge, W1b, and D2's joined-eligibility design |

Under option A, a fresh invocation never mixes two selected methods. The ordinary cases in an extended envelope carry repaired preview-physics-1 semantics, not precision-1 semantics. That removes, for fresh solves, the reason behind T0R's `SOURCE_BLOCKS_ORDINARY_CASE_LEGACY_SEMANTICS`. D2 owns that reader decision.

**Option B.** Keep exact-block first and the new method for the rest. It needs a mixed-method identity and two receipt families in one envelope. I do not recommend it.

### 4.5 Interaction with M03-INTEGRITY-v1 and M03-PRODUCT-PREVIEW-EQUILIBRIUM-v1

- **The ordinary attempt** is judged by M03-INTEGRITY-v1, unchanged, in binary64. Its report stays as evidence.
- **The selected extended state** is judged by the proposed `M03-INTEGRITY-MP-v1`: the same structure with `u_p = 2^-p`, the precision-specific pivot and condition screens, residuals at p + 64, and the stop rule. A binary64 rcond screen is never applied to a p-bit factor, and no factor is relabelled.
- **Selected-state equilibrium.** M03-PRODUCT-PREVIEW-EQUILIBRIUM-v1's componentwise measurement is evaluated on the retained state against the intended system, with `γ_p`, and published with the basis label `retained_precision_p`. ROOT either registers it as that policy's precision-aware reading or gives it a successor id; that is within its existing authority (`CONTRIBUTION_PRECISION/SOURCE_QUALIFICATION.md`, "Reliance boundary").
- **Public projection residual.** The residual of the published binary64 displacements on the represented binary64 K is published as a separately labelled observation, not a gate. It can legitimately be worse. For N05 the represented K has the wrong k.
- **Claims.** No certified inertia, forward-error bound or physical accuracy is claimed. The numerical quality is "checks passed at precision p; 2p agreement verified". Historical DEC-046, DEC-050 and DEC-053 records, and the 64γ binary64 policy, are unchanged.

### 4.6 Boundary with T5

- **D1 supplies** a passive linear kernel API: for a given selected active state (a fixed restraint set and springs), solve at p and recover.
- **D1 does not** classify contact, decide gap signs at precision, iterate active sets, handle friction, or qualify the mixed recovery basis. `NUMERICAL_INTEGRITY_RECOVERY_BASIS_UNQUALIFIED` and the strict-gap law stay with T5 (M06).
- **At the facade,** D1 refuses every model with a nonlinear support record.
- **The nonlinear loop.** W3 converts the loop's base assembly and linearized solves to the sparse representation, without changing their semantics. The friction influence solves are forced to dense scrutiny today (`nonlinear_integration/src/lib.rs:1336-1342`); changing that is T5's (SUP-16).

### 4.7 W2 — range

**Mechanism: exact force-radix scaling of the ordinary system, in the kernel.**
- `AssemblyEvidence` gains a private `force_scale_exponent: i32`. It is chosen at construction, before any allowance is evaluated.
- Local stiffness is scaled by 2^b before `transform_roundoff`. `solve()` scales K and f by 2^b.
- Every scaling is exact because it maps normal numbers to normal numbers. Displacements are unchanged. Actions, reactions and residual records are unscaled exactly for publication.
- Every M03 screen is componentwise-relative, so the scaled evidence is equivalent.

**Choosing b.**
1. First try b = 0. If today's unscaled evaluation succeeds, b = 0, and results are bit-identical.
2. Otherwise, let e_min and e_max be the binary exponents of the smallest and largest nonzero stiffness entry, spring and load. Choose b so that `e_min + b − 64 ≥ −1022` (allowance headroom: γ(48) ≈ 2^-47 plus operation growth) and `e_max + b + 8 ≤ 1023`, centring the window.
3. Formation happens in SI binary64 in the facade. Any nonzero stiffness entry that is already subnormal there is refused, because the bits are gone.

**Admitted range (ordinary path).** A feasible b exists when every nonzero stiffness entry, spring stiffness and load component is normal binary64 in SI units, and their joint exponent span is at most about 1,980 bits. The displacements must also be normal or exactly zero, since force scaling does not change them.

**Refusal.** The case stays `NUMERICAL_INTEGRITY_UNRESOLVED` with a precise reason, for example: "range: exponent span of stiffness [e_min, e_max] and loads [·,·] exceeds the binary64 normal window after exact power-of-two scaling". The global DOF map is included where it applies. No new diagnostic code is added, so readers need not change.

**PHYS-R4.** By my arithmetic, b ≈ 500 puts EA/L near 3e-3, GJ/L near 3.5e-157, and the `[UY,UY]` allowance near 2.5e-170. All are normal. The public fixture then passes the evidence stage, and the free-DOF set is empty. The published stress path was already repaired (`membrane_publication_range.rs`). The detection run P1 records the actual end-to-end result.

**The extended path** has an internal 64-bit exponent, so range stops mattering inside it. Only publication can refuse: a nonzero quantity that underflows or overflows binary64 is explicit, and a subnormal result carries its reduced relative precision. See the D2 note (§5).

**What W2 does not cover:** length-unit scaling, which would need facade formation in scaled units, and cross-unit display (D2).

### 4.8 W3 — sparse assembly, reduction and reactions (M32)

**One representation.** A kernel `SparsePattern` built from element connectivity (12×12 blocks), springs, and user and curved blocks. Values are accumulated in the same element order as today's dense assembly, so every coalesced entry is bit-identical to the dense entry. That makes parity of the represented equations exact.

**The same gate in both modes, rewritten over the pattern.**
- `validate`: symmetry over pattern pairs.
- `prepare`: scaled values and `K_fc u_c` from sparse columns.
- Contribution audit: Expansions per pattern entry, so memory is O(nnz).
- Residual and intended action: sparse rows.
- Pivot screens: the existing profile factor, now built from entries with `SymmetricProfileMatrix::from_entries_with_order` and ordered by `adjacency_from_symmetric_entries`.
- `rcond`: norm from the pattern.
- `negative_pair_witness` and `verify_negative_direction`: pattern pairs only, O(nnz).

The dense `StructuralSystem` API stays, unchanged, for auxiliary callers and for dense scrutiny.

**Modes.**
- **Sparse interactive**, the default, uses only the pattern.
- **Dense scrutiny** materializes a dense view from the same values and keeps today's dense Cholesky, its labels (`dense_structural_integrity_primary`), and the protected DEC-050/053 legacy LU observation (`PP:1959-1974`).
- **Resource guard.** Dense scrutiny refuses above a declared ceiling, with a blocking `SOLVER_SYSTEM_BLOCKED` naming the estimated bytes. Sparse refuses above a profile ceiling. ROOT picks both from measurement.
- **No automatic fallback.** A sparse integrity outcome is never rescued (NP-07). A resource refusal is explicit, and dense needs more memory anyway. Mode code 3 stays reserved and unused, as today.

**Facade after T1 merges.**
- `PP:1378` and `:1452`: sparse assembly per modulus basis.
- `PP:1855`: partition maps.
- `PP:2135`: reactions from sparse rows.
- `solve_preview_reduced_system` takes the pattern.
- `source_recovery` gets a dense view for n ≤ 256 only, if option B were chosen.
- The nonlinear loop moves to sparse (§4.6).

**Parity protocol.** No new tolerance.
1. Bitwise equality of the coalesced K between pattern and dense assembly, for every fixture.
2. M03 outcome class parity between modes on N01–N09, R01–R07, NP-B, NP-D, the T0R references and the R1 families. A divergence near a screen boundary is recorded, never tuned.
3. Published quantities agree within the existing DEC-053 parity basis: 1e-9 relative, scaled by the dense magnitude (`performance_harness/README.md`).
4. A model with two modulus bases, each case on its own pattern.
5. Nonlinear gap, one-way and friction models: the same final active state, and quantities within the DEC-053 basis.
6. Relabelling and permutation give the same answers against the references.

**Memory and runtime protocol.**
- **Sealed models.** R1's RF-LARGE families (10, 100, 1,000 and 10,000 members; determinate and indeterminate; axis-aligned and rotated) and the nine DEC-053 observations. Each is generated deterministically into a product request, and the request's sha256 is committed.
- **One fresh process per model and mode**, run by a standard-library Python runner. It records peak RSS from `/usr/bin/time -v` (Linux) or `-l` (macOS) and applies an address-space cap so the host is protected. Stage timings (assembly, audit, factor, rcond, solve, residual, recovery) are printed by the binary as JSONL: five repeats, median and minimum.
- **Deterministic storage counts:** pattern nnz, profile entries, contributions, and for W1, limbs per entry.
- **Hardware metadata, toolchain and release profile** are recorded.
- **Observations only**, as DEC-053 did. The single claim is the before-and-after growth: peak memory in sparse mode grows about linearly with n on chains, against about n² before. It is stated as an observed fit, not a threshold.

**Homes.**
- `P/core/solver/performance_harness/`, disjoint: kernel-level sparse and dense observations.
- `P/validation/benchmarks/numerical_robustness/`, new: product-level runs through the public entry.

### 4.9 W4 — M03 residual items

**The rigid-null witness for user and curved bodies** (`SA:208-217`, `SA:264`; `FK/rigid_body.rs`).
- **New function.** `assess_constrained_bodies(sub_bodies, ties, grounds)` sits beside `assess_rigid_body`.
  - **Objective sub-bodies** are node sets joined by straight frames, and by curved elements that pass the screen below. Each has six rigid parameters (t, θ); node motion is `u = t + θ×(x − o)`, rotation θ.
  - **Ties.** Each user element imposes `u_a = u_b` and `θ_a = θ_b`. Its energy is zero only for equal nodal motion, because every stiffness is positive.
  - **Grounds** are the restrained DOFs and positive springs, as today.
  - **Rank.** The null space of the stacked map uses the existing SVD rank screen and τ_B form. It returns Restrained, MechanismWitnessed (with the direction mapped to nodes), or NumericallyUnresolved.
- **Proof sketch.** Each family's energy is non-negative, and its zero set is the stated linear space: rigid motions for frames and screened curved elements, equal motion for user elements. So total energy is zero exactly on the intersection with the grounds. This extends the existing welded-frame argument.
- **Curved objectivity screen.** For the six rigid vectors r_k at the element's nodes, `|K_e r_k|` must lie within the formation allowance from `curved_formation` (`SA:291-402`). If it passes, the element joins the objective sub-body. If it fails, the body stays unqualified for a witness, with a reason, and the matrix gate still runs.
- **Coordination.** T4 should confirm the curved construction's null-space claim.

**SUP-17** (`PP:1362`). Proposed text: "fewer than six independent ground constraints including positive springs: the six rigid-body modes of a connected structure cannot all be removed; directly restrained global DOF classes: {restrained}; global DOF classes with no direct restraint: {missing} (not a rigid-body mode analysis; separated restraints can resist rotations); support contributions: …". Update the test at `PP:19995` and record the change. The line sits outside T1's hunks, but in T1's file, so it lands after the merge.

### 4.10 W5 — the VP-ROBUST harness

**Crate** (new): `P/validation/benchmarks/numerical_robustness/`, with its own `Cargo.lock`, discovered by CI automatically.
- Dependencies: `product_physics`, `frame_kernel`, `sparse_direct`, and `serde_json` with `float_roundtrip`, the same set `numerical_integrity` uses.
- `cases/` holds adapters from R1's `references.json` to kernel `PrimitiveSource` values and to product requests. The adapter is reviewed code; R1's values are never edited.
- Examples, not tests, for the scale runs.

**Kernel lane (before T1 merges).**
- R1 families RF-CHAIN, RF-SKEW, RF-WEAK, RF-LARGE, RF-INVARIANCE, RF-RANGE, RF-ZERO, RF-FINITE and RF-MECH, through the kernel method, and through the binary64 sparse gate for the RF-MECH and RF-LARGE parity checks.
- **Limit.** Springs can lie along any direction at kernel level. The product supports only global-axis springs and restraints (`linear_supports/src/lib.rs:217-221`), so R1 cases with springs along a skewed axis run in the kernel lane only.

**Product lane (after T1 merges and the facade slices land).**
- Every authorable R1 case, through `run_linear_static_preview_value_with_mode` in both modes. Each variant runs as its own request.

**What is compared.**
- Global nodal displacements and rotations.
- The six signed reaction components per support (T0R v2 rows).
- Member invariants: axial force, torque and `hypot(My, Mz)` at the ends and stations.
- Method evidence: the selected method, the precisions, the M03 class.
- RF-MECH must be refused with a witness or an unresolved status, and no rows. Any recovered answer fails.

**Predicate.** Every comparison uses `|obs − exp| ≤ 1e-9·max(|exp|, scale)` with R1's stated zero scales. No new tolerance.

**Discrimination check.** Each of R1's negative-control values must fail the same predicate. A control that does not is reported as non-discriminating, not dropped.

**Seeded faults.**
- They sit behind `frame_kernel/mutation-controls`, `#[cfg(any(test, feature = …))]`, enabled only by this crate's mutation run. CI checks that no product manifest enables the feature.
- The kill matrix is recorded. Each fault must fail at least one comparison (§7.3).

**Memory and runtime.** The runner and records of §4.8, committed as JSON with hashes under `numerical_robustness/observations/`.

### 4.11 Build feasibility (facts)

| Backend | Latest (index, 2026-09-26) | Licence (crates.io) | Resolved non-optional dependencies | Declared `rust-version` | Other build needs |
|---|---|---|---|---|---|
| In-repo `wide.rs` | — | MIT (project) | none | — | none |
| `dashu-float` 0.6.1 | 0.6.1 | MIT OR Apache-2.0 | 6: `dashu-base`, `dashu-int`, `num-modular`, `num-order`, `static_assertions`, `cfg-if` | 1.68 | pure Rust; heap-allocated values |
| `rug` 1.30.0 | 1.30.0 | LGPL-3.0+ | 4: `gmp-mpfr-sys` 1.7.1 (LGPL-3.0+, `links = "gmp"`), `az`, `libc`, `libm` | 1.85 | C build of GMP/MPFR from bundled sources, per the crate's documentation; not verified on this host |
| `malachite-float` 0.12.0 | 0.12.0 | LGPL-3.0-only | 12 | 1.90.0 | — |
| `astro-float` 0.9.6 | 0.9.6 | MIT | 9, including proc macros | not declared | — |
| `twofloat` 0.8.4 | 0.8.4 | BSD-3-Clause | 9 | not declared | double-double only: fixed ~106 bits; the refutation's two-word prototype lost the whole twist and action in the k = 1e-28 case (`INDEPENDENT_REFUTATION.md` §3) |

The dependency sets are from the crates.io sparse index, taking the latest non-yanked version that satisfies each requirement. That approximates what `cargo` would resolve; nothing was fetched or built.

**Lockfile effect of an external backend in `frame_kernel`.** Each of these 23 lockfiles would change:
- `P/apps/desktop/src-tauri`;
- `P/core/loads/{load_case_algebra, primitive_loads, self_weight_wasm, stress_recovery, user_loads}`;
- `P/core/model_operations/operation_applier`;
- `P/core/product_physics`;
- `P/core/runner/headless`;
- `P/core/solver/{curved_bend, diagnostics, frame_kernel, linear_supports, nonlinear_integration, nonlinear_supports, performance_harness, sparse_direct, straight_pipe}`;
- `P/validation/benchmarks/{mechanics, nonlinear, numerical_integrity, physics_audit_regression, stress}`.

The kernel would stop being dependency-free. CI would fetch the crate at its `cargo fetch --locked` step, and the owner's Mac would need network access once.

**Recommendation: in-repo.**
- No lockfile changes, no C toolchain, no licence question.
- Stack-allocated limbs mean no allocation per operation.
- Bitwise determinism across platforms.
- A small trust base that can be tested thoroughly:
  - at p = 53 against hardware binary64, bitwise, over random normal-range operands;
  - at p = 64L against exact-rational test vectors, generated by a standard-library Python script checked in beside the tests.
- Estimated at about 800 to 1,200 lines with its tests. This is an estimate.
- `dashu-float` is the fallback if review rejects the in-repo arithmetic.
- `rug` is not recommended: LGPL-3.0+ static linking into the MIT macOS bundle, a C build, and `links = "gmp"`.

## 5. Interface note for D2

What D1 publishes and what readers would need to verify. D2 designs the readers.

1. **The per-case receipt** (closed shape) for each case that selects the method:
   - method token and policy id;
   - source identity digest;
   - the attempts list: p, outcome, reason, work;
   - selected p and verification p;
   - stop-rule summary: the worst normalized disagreement per body and kind;
   - pivot margin minimum, rcond estimate at p, retained-residual summary;
   - retained-state digest;
   - a reference to the ordinary attempt (the existing `OrdinaryAttempt`).

   Cases without the method carry no receipt. A case where the method was attempted and failed carries `RETAINED_PRECISION_UNAVAILABLE` and ordinary standing. A selected case never carries its own unavailable diagnostic.
2. **Row provenance.** Every row of an extended case carries `recovery_method = contribution_preserving_multiprecision_v1`, in the field `PP:2906` already uses. Derived stress rows say "from once-rounded retained actions".
3. **Identity and emission.** §4.4. The envelopes are mixed per case by design, so no invocation-level `Err` exists for them. That serves D2's composite standard: no correct case lost, and no failed case promoted.
4. **Retiring exact-block for fresh solves** (if D-4 option A). Fresh source-blocks-1 and physics-source-1 envelopes stop. Historical readers stay unchanged. D2's source-blocks-1 re-homing "retire" option then follows naturally.
5. **Replay.** Rust can replay bit-identically from the capture, at about the cost of the solve. Python and TS cannot run the kernel. D2 decides whether structural checks suffice for their standing, and how parity across languages is kept, following physics-source-1's precedent.
6. **Representability per published quantity:** normal, subnormal (reduced precision stated), underflow refused, or overflow refused. Transport and display of subnormals belong to D2.
7. **W2.** The integrity report gains `force_scale_exponent`. Physical residual records are unscaled, and may be subnormal as descriptive values, as `physical_residual_record` already allows. Any reader or test that parses `Debug`-formatted report text will see the new field.
8. **SUP-17.** A message text change only.
9. **Selected-UNAVAILABLE alignment.** Under option A, fresh solves no longer emit `SOURCE_BLOCK_RECOVERY_UNAVAILABLE` beside a selected case. D2's alignment then concerns historical and joined envelopes only.
10. **If ROOT chooses option B instead,** D2's composite and alignment designs must also cover mixed exact-block and multiprecision envelopes.

## 6. Slices, order and T1 serialization

**Owners.** Kernel slices are T3 TASKs with one writer per file at a time. Facade slices go to the single `core/product_physics` integration owner named in the graph, after T1 merges and main is merged into the T3 branch. ROOT serializes that owner against other tranches.

| Slice | When | Write set | Tests |
|---|---|---|---|
| K1: W3 kernel sparse representation and sparse M03 gate | before T1 | `FK/structural.rs`, new `FK/structural/sparse.rs`; `P/core/solver/sparse_direct/src/{lib.rs, structural.rs}`; `SA` (sparse `AssemblyEvidence`) | Existing frame_kernel, sparse_direct and nonlinear_integration suites unchanged and passing. New: K bitwise parity, KREV-01 to KREV-05 on sparse, O(nnz) negative witness, dense and sparse outcome parity on N and R and NP-B and NP-D |
| K2: W2 force-radix scaling | before T1, after K1 (shares `SA`) | `SA`, `FK/structural.rs` helpers | b = 0 bit-identity on all existing tests; a synthetic PHYS-R4 element; RF-RANGE kernel cases; a refusal-reason test |
| K3: W1 arithmetic | before T1, in parallel with K1 | new `FK/structural/retained/wide.rs`, test vectors and generator under `P/core/solver/frame_kernel/tests/` | p = 53 against binary64 bitwise; vectors at 128 to 1024; subnormal and overflow publication outcomes |
| K4: W1a kernel method | before T1, after K1 and K3 | new `FK/structural/retained/{source, assemble, factor, recover, adaptive}.rs` | N05, N06, NP-A (intended), RF-CHAIN, RF-SKEW, RF-WEAK, RF-FINITE, RF-MECH, the k = 1e-28 stress case, the exact-block oracle in scope, mutation controls |
| K5: W4 witness and curved screen | before T1 | `FK/rigid_body.rs`, `SA` (geometry, curved screen) | A user-element internal mechanism (NP-C-like), a stabilized companion, near-collinear ties, a curved screen positive and a seeded negative |
| K6: harness observations | before T1 | `P/core/solver/performance_harness/**` | Kernel sparse and dense observations on RF-LARGE, runner dry run |
| V-K: VP-ROBUST kernel lane | before T1, after R1 is frozen and K4 | new `P/validation/benchmarks/numerical_robustness/**` | §4.10 kernel lane |
| F1: facade sparse wiring, W2 call site, SUP-17 | after T1 merges | `PP` (assembly `:1378`, `:1452`; reduction `:1855`; reactions `:2135`; `solve_preview_reduced_system`), `source_recovery.rs` (refuse scaled evidence), nonlinear loop in `nonlinear_integration/src/lib.rs` (with T5) | Full product suites, PHYS-R4 public fixture, parity protocol, dense-scrutiny guard |
| F2: facade W1a wiring and identities | after F1 and ROOT's identity reservation | `PP` (case loop, receipt, rows, combinations), new `P/core/product_physics/src/retained_publication.rs`, tables in `P/fixtures/results/`, schemas (T1 files) | Product lane of VP-ROBUST, N05 and N06 through the public entry, standing controls, the characterization tests for retired exact-block selection replaced and recorded |
| F3: W1b families | after F2 and the R1 addendum | `PP` load builders to `PrimitiveSource`, kernel element-load primitives | RF-ELOAD, 0.4.0 prescribed and eigen cases |
| R: readers | per D2 | D2's write sets | D2's plan |
| V-P: product lane and scale runs | after F1 and F2 | `numerical_robustness/**` | §4.10, §4.8 |
| Join | last | ROOT and the manager | §7.5 |

**Order.** K1 and K3 run in parallel, then K2, K4 and K5, then K6 and V-K. After T1 merges: F1, then F2 (with D2's reader slices), then F3, then V-P and the join. W1c follows in T4's window.

**One atomic PR** for F2 together with D2's readers, as T0R did: producer, readers and gates merge together.

**T1 overlap.** K1 to K6 and V-K touch no T1 file. F1 to F3 edit `PP`, `source_recovery.rs`, the schemas and the fixtures, all of which T1 changes. `nonlinear_integration` is disjoint from T1 but is T5's future area, so ROOT serializes it.

## 7. Verification plan

### 7.1 Frozen references (none edited)

- **N05, N06, NP-A.** The new method meets the unchanged 1e-9 intended-answer criterion on every quantity, including internal torque and the spring action.
  - NP-A's represented-matrix expectations stay the oracle for the ordinary route.
  - A new-method result equal to the represented solution is a failure: it would mean the rounded matrix was promoted.
- **N01 to N09, R01 to R07, NP-B, NP-D.**
  - Both modes, with outcome classes unchanged.
  - R01 to R05-type mutations are killed by the retained residual gate as well.
  - NP-C (connector) is refused as an unsupported family.
- **T0R references.** Kernel-lane "force extended" runs reproduce them. This is a test-only switch showing continuity at ordinary scale.
- **R1 families** (after V2 refutes them), with R1's zero scales.
- **The DEC-053 nine observations,** under their unchanged predicates.

### 7.2 Detection run P1 (on main, before implementation)

Expected, recorded per quantity:

| Family | Expected on main |
|---|---|
| RF-CHAIN | k/a ≳ 1e-6: likely passes. Smaller k/a: Sensitive rows that miss 1e-9, or unresolved (absorbed contribution). Order > 2 means exact-block is unavailable |
| RF-SKEW, RF-FINITE (soft) | Misses of 1e-6 to 1e-3, or unresolved; exact-block unavailable (not a signed permutation) |
| Axis-aligned bending soft modes | Misses of about 1e-5 (§3.1), or unresolved |
| RF-WEAK | Likely passes, or unresolved through an absorbed contribution; to be observed |
| RF-LARGE | 1,000 and 10,000 members: refused, or out of memory under the runner's cap (§2.1). Long cantilever chains may also be unresolved on conditioning. That is my expectation, not a measurement |
| RF-RANGE, PHYS-R4 | `NUMERICAL_INTEGRITY_UNRESOLVED` (range) at the extremes |
| RF-ZERO, RF-INVARIANCE (well-conditioned) | Pass |
| RF-MECH | Refused with a witness for qualified bodies. Large ones: memory, as for RF-LARGE |

Every P1 run applies an address-space limit so it cannot disturb the host.

### 7.3 Mutation controls (each must fail at least one comparison)

1. Promote the rounded binary64 K to p. N05 then has the wrong k and N06 a nonpositive pivot.
2. Round u to binary64 before recovery. N06's torque becomes 0.
3. Drop `K_fc u_c`, or round the reduced RHS to binary64 (the KREV-02 control, and 0.4.0 prescribed cases).
4. Omit one contribution, or flip an axis sign in `B`.
5. Fix the precision at 128 with no escalation. The k = 1e-28 case fails.
6. Skip the 2p verification, accepting on the pivot screen alone. The k = 1e-28 case fails, as in the probe.
7. Disable the geometric mechanism check. RF-MECH is then "recovered".
8. Omit a pattern entry, such as a spring, in sparse mode only (the R05 analogue).
9. Publish without unscaling by 2^b (W2).
10. Make the order depend on labels (relabelling fails).
11. Reuse one modulus basis for every case.
12. Treat a user-element tie as a rigid link (W4).

### 7.4 Native witnesses (owner's Mac)

- N05 and N06 through the desktop, with the new method's token selected.
- An RF-SKEW soft case.
- A ten-member RF-CHAIN.
- A 1,000-member model: time and peak memory recorded.
- The PHYS-R4 public fixture.
- Save and reopen, and the export surfaces.
- A model outside coverage, such as an elbow, showing `RETAINED_PRECISION_UNAVAILABLE` with ordinary standing.

Record the candidate, the model hashes and the case ids. If a witness is not available, record it as outstanding.

### 7.5 Gates

- A complete-diff independent review.
- Hosted CI: the numerical cargo suite, including the new crate, and the full dual-viewport dispatch for surface 4.
- A clean DEC-025 sweep.
- The native witnesses.
- VP-ORACLES and VP-ROBUST passing on the merged candidate. This is the graph's closure rule.

## 8. What T3 completes and what remains, per group

| Group | D1 completes (when merged and verified) | Remains |
|---|---|---|
| M03 | General accuracy for the W1a and W1b families; the PHYS-R4 range (W2); the rigid-null witness for user and screened curved bodies; the SUP-17 wording; VP-ROBUST | Curved bends in the method (W1c, with T4); components and releases; nonlinear mixed recovery and gap classification (T5) |
| M32 | Sparse assembly, reduction, reactions and the gate in both modes; the parity and memory protocol; dense scrutiny kept, with a guard | Friction influence solves (T5, SUP-16); resource ceilings chosen by ROOT from measurement |
| M34 | Solve-side range (W2); publication representability outcomes | Display and transport range, the scientific carrier, comparison policy (D2 and T6) |
| N05 ordinary accuracy | Repaired on the ordinary and exact routes for W1a models | — |
| General retained-source recovery | Order > 2, skewed and weakly coupled systems, and larger systems up to the measured budgets; W1b load producers | Curved, pressure regions, components (W1c and later) |

Real piping models almost always contain weight and elbows. Until W1b and W1c land, most of them get ordinary standing, or the unavailable diagnostic, when Sensitive.

## 9. Decisions for ROOT

| ID | Decision | Options | Recommendation |
|---|---|---|---|
| D-1 | General method | (A) multiprecision with basic-deformation formation; (B) global reformulation; (C) generalized exact-block | (A). The probe refutes (C) (§3.1) |
| D-2 | Arithmetic backend | In-repo `wide.rs`; `dashu-float`; `rug` | In-repo; `dashu-float` as the fallback |
| D-3 | Method policy | `M03-INTEGRITY-MP-v1` with the stop rule `2^-64·max(|q_2p|, S*)` and body-level coupled scales; the equilibrium basis `retained_precision_p`; the 128/256/512 schedule with a 1024 ceiling | Register as proposed. The stricter per-member variant is the alternative |
| D-4 | Method order and identities | (A) retire exact-block selection for fresh solves, exact-block kept as oracle; (B) exact-block first plus the new method, in a mixed identity | (A). Reserve the two successor identities (names are placeholders) |
| D-5 | Trigger | Not Passed (the current Sensitive boundary); or a stricter boundary | Not Passed. Revisit if P1 finds a Passed case that misses 1e-9 |
| D-6 | W2 | Force-radix scaling; admitted range; no new diagnostic code | As proposed |
| D-7 | W3 fallback and guards | No automatic fallback, dense scrutiny explicit and guarded; or automatic dense fallback on sparse representation errors | No automatic fallback. Ceilings from measurement |
| D-8 | Budgets | Per-case and per-invocation work limits for the method | Select after the K6 and V-P measurements, as RESOURCE_POLICY did |
| D-9 | W4 | Constrained-body witness; curved objectivity screen | As proposed; T4 confirms the curved construction |
| D-10 | Serialization | `nonlinear_integration` with T5; curved screen with T4; facade after T1 | As in §6 |
| D-11 | R1 addendum | RF-ELOAD references (uniform, thermal, thrust, constant effort, prescribed motion) before W1b | Commission it after V2 |

**Owner-level.** None, if D-2 is in-repo or `dashu-float`. If ROOT prefers `rug`, then statically linking LGPL-3.0+ GMP and MPFR into the MIT-licensed, signed macOS bundle raises relinking obligations. That is a licence and governance matter for the owner. The options are: avoid it (recommended); ship with LGPL compliance measures; or dynamic linking, which is not how gmp-mpfr-sys builds by default. No protected comparison predicate changes; STAGE0 §5.3 stays as ROOT ruled.

## 10. Sources, probes and limits

**Read.**
- `T3/{STAGE0_MAP, STAGE1_PLAN, OWNER_DIRECTION}.md`; `T3/TASK_BRIEFS/{_COMMON, D1_NUMERICS_DESIGN, D2_STANDING_DESIGN, R1_REFERENCES}.md`.
- `CORRECTNESS_DESIGN/{NUMERICAL_IMPLEMENTATION, M03_RESIDUAL_SUCCESSOR_ADOPTION}.md`, `CONTRIBUTION_PRECISION/{CONTRACT, INDEPENDENT_REFUTATION, RETURN, SOURCE_QUALIFICATION}.md`, `NUMERICAL_POLICY_REVIEW/RETURN.md`, `COMPOSITE_ENGINE/{SELECTION, RESOURCE_POLICY}.md`.
- `DEFAULT_ROUTE_DESIGN/{DESIGN, ROOT_SELECTION}.md`; `ENGINE_INTEGRATION/RETURN.md` (PHYS-R4, conversion scope).
- SUP-17 in `SOLVER_FINDINGS_ASSESSMENT/_run_records/{SUPPLIED_FINDINGS, supports/RETURN}.md` and `T0_REASSESSMENT/{RETURN, INDEPENDENT_CHECK}.md`.
- `P/validation/benchmarks/numerical_integrity/{README, COVERAGE}.md`.
- Source at `c61a540ea`:
  - `FK/{lib.rs, structural.rs, rigid_body.rs}` and the scope, API and limits of `FK/structural/exact_boundary.rs`;
  - `P/core/solver/sparse_direct/src/{lib.rs outline, structural.rs}`;
  - `SA`; `nonlinear_integration/src/{lib.rs sites, product_equilibrium.rs}`;
  - `performance_harness/{README.md, Cargo.toml}`;
  - `curved_bend/src/lib.rs:241-268`; `linear_supports/src/lib.rs:18-25, 217-228`;
  - `PP` at the sites cited, and `source_recovery.rs` (scope and signed permutation);
  - the CI workflow and `numerical_ci.py`, `desktop-release-template.yml`, and the lockfile inventory.
- From T1: `git diff 82b43f9bd f3270ea79` for `P/core/product_physics`, `P/core/solver` and `P/validation` (stat), plus the `source_recovery.rs` hunks.
- I did not read `NUMERICAL_REFERENCE.md` (48 kB) in full, nor `DEFAULT_ROUTE_DESIGN/ROOT_RULINGS.md`, beyond what `ROOT_SELECTION` and DESIGN quote.

**Ran.**
- **`_run_records/probe_skew_precision.py`** (Python 3.11 standard library, `nice 19`, under 1 s) produced `probe_skew_precision.stdout.json`.
  - Its emulation rounds every operation to p bits with an unbounded exponent. It is not Rust or MPFR, and not a product run.
  - Its scales are a probe convenience, not references.
- **`_run_records/index_probe.py`** read the crates.io sparse index and produced `index_probe.stdout.json`. It approximates dependency resolution; nothing was downloaded or built.
- **`_run_records/crates_api_probe.txt`** holds licences read from the crates.io API.
- Python versions are in `_run_records/python_version.txt`.
- Hashes are in `_run_records/SHA256SUMS`, with paths relative to this folder (`DESIGN_NUMERICS/`).

**Limits.**
- **Estimates.** The dense-memory figures (§2.1), the PHYS-R4 scale arithmetic (§4.7), the in-repo arithmetic size and every cost statement are estimates from source and arithmetic, not measurements.
- **Expectations.** The P1 table (§7.2) states expectations only.
- **Probe scope.** It covers one body and at most six members, nodal moments only, and dense LDL in emulation. It does not test the sparse profile at p, the Rust arithmetic, budgets or performance.
- **Unverified here.** The `rug` C-build fact comes from crate documentation, not from this host.
- **To confirm.**
  - T4 should confirm the curved null-space claim.
  - V1 should check the order ≤ 2 reasoning about bending soft modes (§2.3).
- **Drift.** Line numbers drift with T1.
- **Not run.** Existing suites, CI, the native app and a DEC-025 sweep.
