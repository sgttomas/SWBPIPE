# T3 D1 — numerical core: general accuracy, range and sparse scale

HELPS_HUMANS-style design record (TASK D1) for the T3 WORKING_ITEMS manager and ROOT, revision 2, 2026-09-26. It stays a proposal until ROOT selects it after V1's backcheck.

- **Basis.** Revision 1 was written at `e14f7fd13`; revision 2 was started at `12f2122cd` and completed at `4862a72a9`. Both sit on T3 branch `codex/piping-numerical-integrity-20260926`. The product basis is main `c61a540ea`. Line numbers are at `c61a540ea` and will drift.
- **Revision 1** is archived unchanged as `_run_records/DESIGN_revision1.md` (sha256 `7199390f…`). That is the text V1 reviewed.
- **Companion note.** `S11_CONTAINMENT.md` revision 2 (exact ledger and exact recovery, delivered first; revision 1 archived as `_run_records/S11_CONTAINMENT_revision1.md`) is part of this revision. `S11_CONTAINMENT.md` governs S11, except where ROOT's later decisions (`4862a72a9`) narrow it. Those are recorded here: the zero-witness boundary (§4.1.2) and K2a (§4.7, §6).
- **T1.** Candidate `f3270ea79`, read only with `git show` and `git diff 82b43f9bd f3270ea79`. T1 changes no file under `P/core/solver/**` or `P/validation/benchmarks/**`. In `PP` it does not touch the line that SUP-17 names.
- **Paths.** `P/` means `projects/chirality-piping/`. `PP` means `P/core/product_physics/src/lib.rs`. `FK` means `P/core/solver/frame_kernel/src/`. `SA` means `P/core/solver/nonlinear_integration/src/structural_adapter.rs`. `T3/` is this tranche's records folder.
- **Roles read.** Root `AGENTS.md` (sha256 `c8ce87ef…`), `agents/AGENT_TASK.md` (`1a13a5b0…`). I also consulted `agents/AGENT_HELPS_HUMANS.md` (`a0c9fb94…`) deliberately, for the design posture, as the brief asked.
- **What I did not do.** I changed no product source, test, fixture, reference or other record, and ran no Git write. I ran no cargo build, test or Rust probe; the host is held. I ran standard-library Python probes at low priority, and read public crates.io metadata (§10).

## Revision 2 — what changed and why

**Inputs:**
- `T3/REVIEW/RETURN.md` (V1, verdict BLOCKING) and its run records;
- `T3/ROOT_RULINGS_V1.md`, including the further rulings and the S11 pre-acceptance;
- `T3/MANAGER_NOTES/{V1_DISPOSITIONS, S11_MAP}.md`;
- `T3/REVIEW/S11_CHECK.md` (V1's check of S11 revision 1, BLOCKING, at `56b651282`) and ROOT's rulings on it, with ROOT's adopted no-interim text (`T3/ROOT_RULINGS_V1.md` at `2d07cad7f`);
- R1's references at `6c448d260` (`T3/REFERENCES/README.md` only);
- D2 revision 2 at `d566713e9` (§4.9);
- the manager's instructions of 2026-09-26.

| Finding | Change | Where |
|---|---|---|
| **V1-B1** (BLOCKING; ROOT ruling 1) | **Every multi-term sum outside the factorization is now one exact expansion, rounded once.** That covers per-DOF load contributions (the load ledger), stiffness entries, the reduced right-hand side with `K_fc u_c`, reactions, basic-deformation recovery sums, and combinations over retained states. **Combination outputs are now under the stop rule.** New probe evidence: V1's check L is exact at 128 bits with the ledger, but accepted with error 0.5 when loads are folded at p. The absorbed-term combination A + B − A2 is exact when combined exactly, but accepted with error 1.0 when folded. A combination that needs escalation rejects 128 and accepts 256. V1's cancellation case and two combination cases are added as negative controls (§7.3, 13–16). **The §3.1 claim "never accepted a failing candidate" is withdrawn** and restated as limited to precision-dependent error | §3.1, §3.2, §4.1.1, §4.1.2, §4.1.4-§4.1.6, §4.1.9, §7.3 |
| **V1-S11** (ROOT ruling 2), then **S11-V1 to S11-V7** and ROOT's seven additions | C3-full, redefined in `S11_CONTAINMENT.md` revision 2 as an exact per-case load ledger **plus exact recovery sums** (14 recovery-side load sums enumerated, S11 §2.2). The force vector is a type only the ledger can build, with an enumerated site test. Contribution granularity is defined per producer. One correctly rounded accumulator (`FK/exact_sum.rs`, from `pressure_sum::exact_sum`) replaces `Expansion::rounded()` and every fold. T1's three sites are named. The invariant "never newly silent" is stated and tested on probe A. S11-K is T1-disjoint and partly live (`SP`, `load_case_algebra`, the `FK` rounding sites); S11-F follows T1. A detected loss makes the case Sensitive with `LOAD_CONTRIBUTION_ABSORBED`. ROOT's pre-acceptance is suspended until V1's backcheck; ROOT's decisions on D-S11-1 to D-S11-4 are in the row "ROOT's S11 decisions" below | `S11_CONTAINMENT.md`; §4.1.2, §4.3, §6, §7, §9 |
| New finding N-S11-R | `Expansion::rounded()` (naive, not correctly rounded) feeds the published, byte-compared M03 intended-action `ResidualRow`; exact-zero residuals publish −0.0 (245 in committed fixtures) because the standard library's float `Sum` starts from −0.0. S11 proposes a zero witness so committed bytes stay unchanged (D-S11-1) | `S11_CONTAINMENT.md` §4.1.3, §7 |
| **V1-S8** (ROOT ruling 1) | 1e-9 cannot be proved for every published quantity: exact zeros and quantities far below the body scale have no relative guarantee. **The floor is stated and enforced.** Relative 1e-9 holds for \|q\| ≥ R·S\*, with R = 2^-64/1e-9 ≈ 5.42e-11. Below that the guarantee is absolute (2^-64·S\*). The receipt classifies every published quantity. Readers never present a below-floor quantity as relative-verified. VP-ROBUST requires every reference zero scale to be at least R·S\*. A weak-coupling control is added: its far-node quantities sit at 2e-21 to 6e-16 of S\*, yet are exact at 128 bits in the probe. Withholding below-floor quantities is offered to ROOT as an alternative, and not recommended because it would withhold structural zeros | §4.1.6, §4.10, §5, §9 D-12 |
| **V1-S2** (ruling 3, adopted) | **A shared retirement gate**, defined here and cited by D2 (S-F, S-G). Four conditions: coverage, budgets, three-language standing, and value agreement with the exact-block oracle. It is applied per identity family at F2 and at F3 | §4.4.1 |
| **V1-S1** (R-3(a), R-3(b), R-7 ruled) | Stated as ruled. `load-reference-source-1` stays fresh until F3 lands and retires at F3 under the gate. S-E1 is built with F3. Historical physics-source-1 stays eligible. Historical all-selected source-blocks-1 stays Current (R-7 (i)) | §4.4 |
| **V1-S3** | D2 owns the successor-identity readers (S-G, D2 §4.9). The receipt (§5) is their interface. **IF-1 is adopted:** `numerical_quality.cases[i]` keeps the ordinary attempt's outcome; the precision-p outcome lives only in the receipt (DD-11 does not arise) | §4.5, §5 |
| **V1-S4**, aligned with D2 (G1, G2, G5, IF-1, DD-11) | The receipt uses the checked profile `openpipestress_jcs_ijson_v1`. Every value that can exceed 2^53 − 1, be subnormal or be negative zero is a 16-hex binary64 bit string. Plain numbers are only exact integers within range. **Per-case hashing failure:** that case becomes `unavailable` (`receipt_encoding`) with ordinary standing. A publication-hash failure republishes the invocation under its base identity, with every attempt declined, as T1's SF-1 does. Never an `Err` | §5 |
| **V1-S5** (ruling 4) | Capture refuses \|x\| ≥ 2^53 on every route. D2 designs the capture fix (R-6). D1 adds capture-boundary cases to RF-RANGE and VP-ROBUST, and states W1 and W2's handling once capture admits such values | §4.7, §4.10 |
| **V1-S7** | `force_scale_exponent` stays out of `StructuralReport`, so the `Debug`-published report is byte-identical when b = 0. When b ≠ 0 it is published in its own evidence line. The b-selection rule is normative, including the subnormal cases. W2 publication applies the §5 representability outcomes | §4.7, §5 |
| **V1-S9** | Targeted hard-case classes for the arithmetic, a large seeded `Fraction` differential, and seeded rounding mutants | §4.11 |
| V1-N1 | §3.1 column (C) is relabelled "assembled binary64 matrix, solved exactly". V1's contribution-exact results are added beside it | §3.1 |
| V1-N2 | A negative-energy witness against represented binary64 entries now triggers the method for supported families. The method's check at p against the primitive model decides the outcome | §4.3 |
| V1-N4 and the manager | K5 is serialized after K2b. S11-K, K1, K2b and K5 all write `SA` and are serialized in that order | §6 |
| ROOT's S11 decisions (after `4663cdbb6`, recorded at `4862a72a9`) | D-S11-1: the zero witness is adopted **only for published diagnostic renderings** (the intended-action `ResidualRow` at `FK/structural.rs:554`); the ledger, the force vector, every recovery sum and every value that feeds a bit-equality check or a receipt use +0.0. D-S11-2: `Expansion::rounded()` replaced at all three `FK` sites, under the fixture stop rule. D-S11-3: S11-K is a full product slice with full gates, as its own PR to main. D-S11-4: no in-band marker. **Formation silent zero:** its reach is stated (only LEF-small among R1's RF-RANGE vectors; no realistic case), and checked formation is split out as **K2a, next after S11-K**, as its own slice | §4.1.2, §4.7, §6, §9 |
| V1-N5 | Components and releases go to T4 and T7 (proposed). Equivalent-static has no owning tranche in the graph, so it is recorded as open for ROOT | §4.2 |
| V1-N6 | `RLIMIT_AS` is enforced on Linux only. On macOS the runner uses an RSS watchdog | §4.8 |
| V1-N3 | No change; P1 checks the Passed boundary | — |
| R1's findings (`T3/REFERENCES/README.md` at `6c448d260`, relayed by the manager) | Finding 1: `RF-SKEW-A-CANT-AX-122-r1e-12` and `RF-FINITE-THIRTIETHS-O1e6` are compared on the represented basis. Finding 2: the k/a ≈ 1e-4 cases are continuity controls, and discrimination is claimed only from k/a ≤ 1e-8. Finding 3: LEF-small and LEF-large are solved by checked formation plus scaling at formation, never a silent zero coefficient. Finding 4: directional-spring cases run in the kernel lane only. Finding 5: RF-CANCEL is compared on R1's net-governed scale, and its relation to the S11 row scale is in `S11_CONTAINMENT.md` §5.2 | §4.7, §4.10, §7.1, §7.2 |

## 1. Recommendation in brief

**W1, general accuracy.** Implement the selected contribution-preserving multiprecision method (`CORRECTNESS_DESIGN/CONTRIBUTION_PRECISION/CONTRACT.md`) as the one method for N05-class accuracy and for general retained-source recovery. Add five concrete choices to it:

1. **Formation.** Rebuild every element at precision p from its binary64 primitives, in basic-deformation form, `K_e = Bᵀ D B`. The probe (§3) shows why: the rounded binary64 element matrix breaks the element's rigid-body null space, even for an axis-aligned member.
2. **Solver.** Use a sparse profile LDLᵀ at precision p, on the same sparse structure W3 introduces.
3. **Exact sums (revision 2, V1-B1).** Every multi-term sum outside the factorization is one exact expansion, rounded once. That covers loads per DOF, stiffness entries, the reduced right-hand side, reactions, recovery sums and combinations over retained states. This removes the common-mode loss V1 found: a contribution lost identically at p and at 2p.
4. **Stop rule.** A candidate at p is accepted only when a fresh solve at 2p agrees with it on every published quantity, combination outputs included. The screen is `2^-64` of the connected body's scale for that kind of quantity (§4.1.6). Relative 1e-9 is guaranteed only above the floor R·S\* ≈ 5.42e-11·S\*; below it the guarantee is absolute, and the receipt says so (V1-S8). ROOT registers the constant as method policy; it is the acceptance threshold for a selected case, not a comparison tolerance.
5. **Arithmetic.** Write a small, dependency-free fixed-limb binary float type in `frame_kernel`. No lockfile changes.

Two alternatives are rejected. Generalizing the merged exact-block method is exact on the wrong system: the probe finds scaled errors of 5e-6 to 1e-3 against the 1e-9 criterion, and finds a matrix that is not positive definite for N06-class skewed cases. A global relative-coordinate reformulation is deferred.

**Coverage.** Three phases.
- **W1a:** straight frames, global-axis linear springs, rigid restraints, nodal loads, and combinations through T0R's gates.
- **W1b:** element uniform loads, thermal eigen axial loads, pressure thrust on straight members, constant effort, prescribed support motion (0.4.0, after T1) and user matrices.
- **W1c:** curved bends, with T4.

Nonlinear and contact recovery stays with T5. Every excluded family is refused per case with a named reason; the case keeps its ordinary standing.

**Triggers and identities.**
- The method runs automatically, per case, on the ordinary route and on the exact route, whenever the ordinary M03 outcome is not `Passed`. Witnessed mechanisms, asymmetry and invalid input never trigger it. A negative-energy witness found only against the represented binary64 entries does trigger it for supported families (V1-N2).
- It publishes under proposed successor identities that allow mixed envelopes (placeholders for ROOT, §4.4). No invocation-level failure path exists for them.
- I recommend retiring exact-block selection for fresh solves. The exact-block method stays as an independent oracle in tests.
- **Retirement gate** (§4.4.1). Retirement happens only under one shared gate with four conditions: coverage, budgets, three-language standing, and value agreement. It applies at F2 for source-blocks-1 and physics-source-1, and at F3 for `load-reference-source-1`, as ROOT ruled in R-3(a).

**S11, cancelled load contributions (a live silent-wrong path on main).** C3-full; the design is `S11_CONTAINMENT.md` revision 2.
- **The rule.** Every force contribution goes, term by term, into one exact per-case ledger, rounded once, and the force vector can be built only from it. Every recovery-side load sum (end forces, stations, extrema, curved sections, reactions, combinations) is one exact sum of its individual terms, rounded once through the same accumulator. Retained source and both receipt replays use the same ledger.
- **S11-K** lands first when the host is released. It is T1-disjoint: the accumulator, the ledger types, the kernel audit entry point, and live exact recovery in `SP`, exact combinations in `load_case_algebra`, and the `FK` rounding sites.
- **S11-F** is the first facade slice after T1 merges: the ledger at every producer, the `PP` recovery composition and T1's three sites.
- **The guard.** The audit marks any remaining loss Sensitive, with `LOAD_CONTRIBUTION_ABSORBED`. It is defence in depth, with a stated floor.
- ROOT's pre-acceptance is suspended until V1 backchecks revision 2. ROOT has decided its four sub-decisions (`4862a72a9`): the zero witness only for published diagnostic renderings (§4.1.2), all three `FK` rounding sites, S11-K as a full-gate product slice, and no in-band marker. K2a (checked formation) follows S11-K as the next early slice. ROOT's adopted no-interim text governs: no interim containment before T1 merges.

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

**Column (C), relabelled in revision 2 (V1-N1).** My probe's column solves the *assembled* binary64 matrix exactly, after a + k has already been rounded in binary64. The merged exact-block method instead sums the represented contributions exactly. V1 reran the probe with that method (`REVIEW/_run_records/v1_stop_rule_probe.*`, check C); its results are in column (C′).

| Case | (C) Assembled binary64 matrix, solved exactly | (C′) Represented contributions summed exactly, solved exactly (V1) | Rounded matrix promoted to 128 bits | Ordinary binary64 | Primitive rebuild, p = 128 | Stop rule |
|---|---|---|---|---|---|---|
| Axis-aligned, k = 1e-4 (bending soft mode) | 1.05e-5 (8 of 22 fail) | 1.05e-5 (5 fail) | 1.05e-5 | 1.06e-5 | 7.5e-28 | 128 accepted (8.1e-28) |
| Skew (3,4,0), k = 100 (ordinary scale) | 4.7e-12 | 4.7e-12 | 4.7e-12 | 9.4e-12 | 1.7e-34 | 128 accepted |
| Skew (3,4,0), k = 1e-4 (N05 class) | 5.5e-6 (10 fail) | 4.7e-6 (10 fail) | 5.5e-6 | 8.7e-6 | 0 | 128 accepted (1.3e-28) |
| Skew (3,4,0), k = 1e-12 (N06 class) | nonpositive pivot | nonpositive pivot | nonpositive pivot | nonpositive pivot | 0 | 128 accepted (1.9e-20) |
| Oblique (2,3,6), k = 1e-4 | 1.07e-3 (19 fail) | 1.07e-3 (19 fail) | 1.07e-3 | 2.7e-3 | 2.6e-25 | 128 accepted |
| Skew, k = 1e-28 (arithmetic stress) | nonpositive pivot | nonpositive pivot | nonpositive pivot | nonpositive pivot | 1.8e-4 (10 fail); the pivot screen passes, margin 3.4 | 128 rejected (1.8e-4). p = 256 has error 5.0e-43 and is accepted against 512 (5.3e-43) |
| Six-member skew run, k = 1e-12 (39 free DOFs in one block) | nonpositive pivot | nonpositive pivot | nonpositive pivot | nonpositive pivot | 2.1e-19 | 128 rejected (2.1e-19 > 2^-64 ≈ 5.4e-20). p = 256 has error 0; its 512 verification was not run |
| k = 0, a genuine mechanism | — | — | — | — | nonpositive pivot at 128, 256 and 512 (screen margins −0.003 to −0.005) | never solved |

**What follows:**
- **(C) is refuted.** Solved exactly, the represented binary64 contributions still have scaled errors of 5e-6 to 1e-3 against the 1e-9 criterion; this holds in both (C) and (C′). For N06-class skewed cases, the represented matrix is not even positive definite in exact arithmetic. Promoting the rounded matrix changes nothing.
- **An axis-aligned transform is not enough.** In the first row the transform is a signed permutation. The error comes from the independently rounded bending coefficients, which break the element's rigid-rotation null space.
- **Primitive basic-deformation formation meets 1e-9** at 128 bits, with more than 15 orders of margin, for N05 and N06 classes, axis-aligned, skewed and oblique, and for a block of order 39.
- **The pivot screen alone is not enough.** At k = 1e-28 and 128 bits it passes (margin 3.4), yet the answer is 1.8e-4 wrong. The 2p agreement rule catches this. This is why the stop rule is required, not optional.
- **The stop rule, on these cases.** It escalated the six-member case, whose 128-bit answer already met 1e-9. On these cases it accepted no failing candidate.
  - Revision 1 went further and said it "never accepted a failing candidate". **That claim is withdrawn** (V1-B1). The stop rule detects only error that depends on the precision.
  - V1's check L shows the failure mode: a contribution lost identically at p and at 2p leaves the two candidates in agreement on a wrong answer.
  - Revision 2 removes every such common-mode class it knows of, by forming sums exactly (§4.1.2, §4.1.9). It does not claim the stop rule is a proof.
- **Precision does not regularize a mechanism.**

This is the evidence the refutation lacked ("a numerical analogue, not a deployed spatial-frame test"). It still is not Rust, MPFR, the product, or a performance result.

### 3.2 Revision 2 evidence: exact sums and combinations (V1-B1, V1-S8)

`_run_records/probe_rev2_b1.py` → `probe_rev2_b1.stdout.json`. It uses revision 1's emulation, imported unchanged, on D1's N05-class skew case. Loads of 1e80 are arithmetic stress only, outside any physical claim.

| Check | Rule | Stop rule | Error of the accepted candidate |
|---|---|---|---|
| B1-L: V1 check L, moment contributions (1e80, 1e-8, −1e80) on RX plus 2e-8 on RY | Loads folded at p (revision 1) | 128 vs 256 agree to 1.8e-28 → **128 accepted** | 0.5 of the body scale; strict relative 2.0 (7 quantities fail) |
| | **Load ledger: exact per-DOF sum, rounded once (revision 2)** | 128 accepted (1.3e-28) | **0**, no failures |
| B1-C: combination A + B − A2, with A = A2 = 1e80 and B = 1e-8 on RX | Combined at p term by term | 128 vs 256 agree exactly → **128 accepted** | 1.0 (8 quantities fail) |
| | **Combined as one exact expansion, rounded once, under the stop rule** | 128 accepted (5.4e-28) | 5.4e-28 of the body scale; no failures |
| B1-E: combination (P, ε) − P, ε/P = 1e-45 (the true value needs more than 128 bits) | Combined exactly, under the stop rule | **128 rejected** (1.0); 256 accepted against 512 (1.3e-21) | 1.3e-21 of the body scale; no failures |
| S8-W: a stiff member, then a soft member (E scaled by 1e-10 or 1e-14) to a node grounded by 1e12 springs | Revision 2 | 128 accepted (2.5e-36) | No failures. The far node's quantities sit at 2e-17 to 6e-16 (s = 1e-10) and 2e-21 to 6e-20 (s = 1e-14) of S\*, below the floor, yet exact at binary64 publication |

The loss V1 showed is a property of how sums are formed, not of the stop rule. Once the sums are exact, the same stop rule accepts correct candidates and rejects the one that needs escalation. S8-W shows that below-floor quantities can be exact in practice, but the stop rule does not guarantee them. The guarantee is stated in §4.1.6.

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
- **The opaque result.** `RetainedSolve` is bound to its source and precision. `RetainedSolve::publish()` rounds each quantity once.
  - `RetainedCombination::form(&[(factor, &RetainedSolve)], p)` is revised for V1-B1. It forms `Σ cᵢ·uᵢ` and `Σ cᵢ·fᵢ` as one exact expansion per component: TwoProduct of the binary64 factor and the p-bit state, then TwoSum accumulation. Each is rounded once to p, and actions and reactions are recovered from the combined state at p.
  - It is formed at p and at 2p from the two precisions' retained states. **Its published outputs go through the stop rule** exactly as case outputs do, with S\* taken from the combination's own body scale.
  - A combination whose candidates disagree escalates independently of its operand cases. At the ceiling, it is withheld with `RETAINED_PRECISION_UNAVAILABLE` (reason `combination_unresolved`), and its operand cases keep their standing.

No caller can supply a matrix, factor, closure or label. This mirrors the KREV trust boundary of `finish_structural` (`structural.rs:1086-1107`).

#### 4.1.2 Formation, assembly and reduction at precision p

Every operation below is rounded to p bits, and every input is a binary64 value lifted exactly.

1. **Frame.** `d = x_j − x_i`, `L = √(d·d)`, `e_x = d/L`. Gram-Schmidt of `y_reference` gives `e_y`, normalized, and `e_z = e_x × e_y`, normalized. This is the product's own algorithm (`FK/lib.rs:511-525`), so the axes converge to the exact axes of the binary64 geometry. No axis tolerance is used at p.
2. **Basic deformations.** `B_local` (6×12) holds the axial extension, the twist, and the end rotations relative to the chord in both local planes, using `1/L`. `B = B_local·T`.
3. **Constitutive operator.** `D` (6×6) is `diag(EA/L, GJ/L) ⊕ (EI_z/L)[[4,2],[2,4]] ⊕ (EI_y/L)[[4,2],[2,4]]`. `Bᵀ D B` reproduces the product's local matrix; the probe agrees to 1.3e-16 in binary64, with the same zero pattern.
4. **Assembly (exact, revision 2).** Each pattern entry of K is formed as one exact expansion of its p-bit element contributions and binary64 spring stiffnesses, using TwoSum in `Wide`. It is rounded once to p.
5. **Loads: the load ledger (revision 2).** Each DOF's load is the exact sum of its identified contributions, rounded once to p. Nodal forces and moments are exact binary64 values. W1b's equivalents (uniform, thermal, thrust, constant effort) are p-bit values formed from binary64 inputs. It is the same ledger S11-F introduces for the ordinary route (`S11_CONTAINMENT.md` §4.2–§4.3), with the same contribution granularity, carried at precision p: `FK/exact_sum.rs`'s `ExactAccumulator` holds the exact sum and gains one further projection, to `Wide<L>` at p, beside its binary64 rounding.
6. **Reduction (exact, revision 2).** Free and prescribed maps come from the source. Each `rhs_i` is one exact expansion: the ledger terms plus the exact products `−K_ic·u_c` of the p-bit coefficients and binary64 prescribed values, rounded once to p. Neither K nor rhs is ever rounded back to binary64.

Because `B·r = 0` holds exactly for every rigid motion r of the exact geometry, the artificial rigid-mode stiffness is O(2^-p·a), not O(2^-53·a).

**The exact-sum rule (V1-B1).** Every multi-term sum outside the factorization and the triangular solves is formed exactly and rounded once. That is: loads, stiffness entries, the reduced right-hand side, reactions (§4.1.5), basic-deformation recovery sums (§4.1.5), combinations (§4.1.1) and the residuals (§4.1.4). §4.1.9 explains why this is the class the stop rule cannot see, and what remains. Every such sum uses one accumulation discipline: exact accumulation of its binary64 or p-bit terms, then one correct rounding (to nearest-even), shared with the binary64 route's `FK/exact_sum.rs`. An exact zero is +0.0. **Zero-witness boundary (ROOT, D-S11-1).** The only exception is a published diagnostic rendering whose replaced expression publishes −0.0 for an exact zero today: the intended-action `ResidualRow` fields `residual` and `normalized_residual` produced at `FK/structural.rs:554`. There the exact zero keeps −0.0, written as a literal, so committed evidence bytes are unchanged. Those two fields are consumed only by the `Debug` rendering in diagnostics (`PP:883`, `:2046`, `:2057`) and by the gate ratio through `r.abs()`, where the sign has no effect. `ResidualRow` has no `Serialize`, and `intended_residual_rows` is read elsewhere only for size accounting (`SA:765`). The witness never applies to the ledger, the force vector, a recovery sum, a published result row (rows are bound byte for byte by receipts, `source_receipt.rs:811`), a `source_recovery` or `exact_boundary` comparison, or any value at precision p. All of those use +0.0. This narrows `S11_CONTAINMENT.md` §4.1.3, which had also proposed the witness at the recovery sums E3, E4 and E5. Any committed −0.0 that passed through one of those sums as its zero therefore becomes +0.0, and S11-K's fixture stop rule reports it before any regeneration.

#### 4.1.3 Factor, screens and mechanism handling

- **Geometry first.** The geometric rigid-body assessment runs before any factor (W4 generalizes it). A witnessed mechanism is refused and never escalated.
- **Factor.** RCM ordering from the pattern. The ordering is integer data, shared with the binary64 sparse path. Profile LDLᵀ at p.
- **Pivot screen** at precision p: `d_i > 64·γ_p(m_i)·c_i`, where `γ_p(m) = m·2^-p/(1 − m·2^-p)`. This is the M03 structure with `u_p = 2^-p`. A failed pivot at p escalates to the next p; at the ceiling the result is unresolved. It is never read as a mechanism.
- **Negative energy.** Checked for pattern pairs only, at p, against the intended K.
- **Condition estimate.** Hager–Higham with the p-factor. `rcond ≤ 2^-(p-1)` counts as unresolved at p and escalates. The estimate is published as model information, with the label "sensitivity to matrix-entry perturbation, not to authored parameters".

#### 4.1.4 Solve and refinement

1. Solve with the p-factor.
2. Evaluate `r = f − K u` against the intended system re-formed at p + 64, from the same primitives. The load term is the same ledger rounded once to p + 64, not a fold. Each `r_i` is one exact expansion of the p-bit products and the ledger terms (revision 2).
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

**Revision 2.** Each component of `d_local`, `e`, `Q` and the end actions is one exact expansion of its (at most five) product terms, rounded once. Each reaction is one exact expansion of `K_cj·u_j` products and the ledger terms, rounded once.

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
- **What it is not.** The stop rule is operational evidence of numerical convergence, not a forward-error enclosure. It changes no comparison criterion. It is the acceptance threshold for a selected case (V1-S8).
- **Scope (revision 2).** Every published quantity of a selected case, and every published output of a combination formed over retained states (§4.1.1).

**What acceptance guarantees (V1-S8, revision 2).**
- If q_2p is accurate well beyond 2^-64·S\*, acceptance bounds the candidate's error by `2^-64·S*`.
- **Relative 1e-9 on q therefore follows only for `|q| ≥ R·S*`, with R = 2^-64/1e-9 ≈ 5.42e-11 (the floor).** Below the floor the guarantee is the absolute bound 2^-64·S\*. For exact zeros, relative accuracy is undefined.
- **A proof for every quantity is not available.** No agreement rule can separate a structural zero computed as noise from a legitimately tiny value, without a floor. So revision 2 states the floor and enforces it:
  1. **Classification.** The receipt carries S\* per body and kind, as bit strings. It lists every published quantity with `|q_p| < R·S*` as `absolute_verified`, with its bound; every other quantity is `relative_verified`. R and 2^-64 are exact binary64 constants; the classification is one binary64 comparison per quantity, reproducible by every reader.
  2. **Readers** (D2's S-G) verify the classification against the published rows. They never present an `absolute_verified` quantity as relative-accurate. A rule binding or qualified export of such a quantity carries its absolute bound, and D2 decides how rule checks use it.
  3. **VP-ROBUST** requires every reference quantity's comparison scale to be at least R·S\* of its case (a check on R1's zero scales, passed to R1 and V2). So acceptance implies the unchanged reference predicate for every frozen comparison. A reference that violates this is reported, not loosened.
  4. **The weak-coupling control S8-W** (§3.2) stays in VP-ROBUST as a positive control: below-floor quantities are published, labelled `absolute_verified`, and exact in the probe.
- **The alternative for ROOT (D-12):** withhold every nonzero quantity below the floor. It is not recommended: it would withhold structural zeros computed as noise, such as the axial force of a torsion-only member, and correct weak-coupling responses.

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
- **Replay is not a standing input.** In D2's revision 2 (§4.9.4) it is a Rust validation-lane audit in `numerical_robustness`, so all three languages decide standing on the same basis.

#### 4.1.9 What the stop rule can and cannot see (revision 2, V1-B1)

- **The stop rule detects error that depends on the precision.** Such error is of order 2^-p at p and 2^-2p at 2p, so the two candidates differ by about the error itself.
- **It cannot detect common-mode loss:** a contribution lost identically at p and at 2p. That needs a sum whose exact value is smaller than the rounding of an intermediate partial sum at both precisions. There are two known forms:
  - **Mixed-sign sums of exact source values.** V1's check L: binary64 loads are exact inputs, so their fold loses the same bits at every precision below the span of the sum.
  - **Sums in which bit-identical computed operands cancel.** V1's combination A + B − A2: A and A2 are the same computation, identical at each precision. So are two identical members meeting at a node, or a rigid translation shared by both ends of a member.
- **Revision 2 removes both forms.** Every such sum is formed as an exact expansion and rounded once (§4.1.2, §4.1.4, §4.1.5, §4.1.1). Only the final rounding remains, relative to the sum itself.
- **What remains inside the factorization and triangular solves.** Every operand there carries formation error of order 2^-p, which changes at 2p. Error that grows from it is precision-dependent, and the refinement residual at p + 64, formed against the exact-expansion right-hand side, checks the solve independently of the factor.
- **This is an argument, not a proof.** The negative controls (§7.3, items 13–16) make each removed form a test that must fail when reintroduced.
- **The same rule holds on the ordinary binary64 route through S11-K and S11-F:** one exact load ledger, rounded once, and exact recovery sums through the same accumulator, with an audit against the exact per-DOF loads (`S11_CONTAINMENT.md` §4). The F-slices supersede that route's recovery with recovery at p.

### 4.2 Coverage and refusals

| Family | W1a | W1b | W1c | Refusal reason while excluded |
|---|---|---|---|---|
| Straight frame members | yes | | | — |
| Global-axis linear springs (k > 0) | yes | | | — |
| Rigid restraints, zero value | yes | | | — |
| Nonzero prescribed support motion (0.4.0 `ResolvedCase`) | kernel yes | facade, after T1 | | `prescribed motion unsupported` |
| Nodal forces and moments | yes | | | — |
| Combinations (T0R-admitted `mechanics`, subtraction, range) | at the facade: exact expansion over the retained states, rounded once, under the stop rule (§4.1.1); T0R gates unchanged | | | `combination withheld by gate` (existing codes); `RETAINED_PRECISION_UNAVAILABLE` (`combination_unresolved`) at the ceiling |
| Element uniform loads, including weight | | yes | | `element load producer unsupported` |
| Thermal eigen axial load (0.4.0 resolved) | | yes | | `thermal producer unsupported` |
| Pressure thrust on straight members; exact-route pressure regions | | thrust yes; regions in W1c | yes | `pressure producer unsupported` |
| Constant-effort support forces | | yes | | `constant-effort producer unsupported` |
| User stiffness elements | | yes (relative-DOF B form) | | `user-matrix element unsupported` |
| Curved bend macro elements | | | with T4 | `curved element unsupported` |
| Components, releases | | | proposed owners T4 (joints M07, bends M02) and T7 (specialized components M17, M20), each when its element model is qualified (V1-N5) | `component or release unsupported` |
| Equivalent static | | | **open**: no owning tranche in the work graph; recorded for ROOT to assign (V1-N5) | `equivalent-static unsupported` |
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
- **Never triggered by** `Mechanism`, `Asymmetric` or `InvalidInput`.
- **`NegativeEnergy` (revision 2, V1-N2).**
  - The ordinary witness is verified against the stored binary64 matrix (`FK/structural.rs:1287-1327`, `:580-587`). For skewed or bending-soft models that matrix can be indefinite while the primitive model is positive definite (§3.1).
  - For a case in the supported family, a `NegativeEnergy` outcome therefore triggers the method, and the ordinary witness is kept as evidence.
  - The method's own result decides: selected if it passes at p, unresolved otherwise. `NUMERICAL_INTEGRITY_NEGATIVE_ENERGY` stays only when a negative direction is also verified at p against the primitive model. Frames and positive springs have non-negative energy by construction, so no such direction exists in the supported family.
  - Outside the family, the ordinary `NegativeEnergy` outcome stands.
- **The S11 load audit (S11-F).** A case marked Sensitive because a load contribution was absorbed is an ordinary Sensitive case and triggers the method in the same way. The method's ledger is exact, so the loss does not recur.
- **Not run by default on Passed cases.** The contract keeps the ordinary path "for cases it actually resolves under the existing policy". A Passed binary64 result can still carry a normwise estimate up to about κ·u < 1e-8 (with κ < 1/√EPSILON). If the detection run P1 finds any R1 case that is Passed yet misses 1e-9, ROOT should consider moving the trigger (§9, D-5).
- **The ordinary attempt always runs first.** Its M03 report is kept as evidence, in the same place `OrdinaryAttempt` sits today (`PP:1869-1872`).

### 4.4 Identities and what happens to existing ones

Proposals only; ROOT reserves names and versions.

| Identity | After D1 |
|---|---|
| `preview-physics-1` | Byte-unchanged for every envelope in which no case selects the new method |
| `<preview-retained>`, suggested `openpipestress.result_semantics/0.3.0/preview-physics-retained-1` | Ordinary route, emitted when at least one case selects the method. It inherits the preview-physics-1 table and adds a closed per-case receipt (§5). Other cases are rendered under preview-physics-1 semantics, with their ordinary standing |
| `<physics-retained>`, suggested `.../physics-retained-1` | The exact-route equivalent, inheriting physics-1 semantics |
| `source-blocks-1`, `physics-source-1` | Readers, fixtures and hashes unchanged. **Recommended (D-4 option A):** no longer emitted for fresh solves, **from F2, and only once the retirement gate (§4.4.1) passes for the family.** The exact-block method stays a kernel test oracle: in its scope, the new method's rows must match its projections within the unchanged criterion |
| `load-reference-source-1` (T1) | **As ruled (R-3(a)).** It stays fresh until F3 (W1b, including the 0.4.0 load states) lands, and it stops being fresh at F3, when the gate passes for it. Joined results stay `needs_recompute` until then. D2's S-E1 is built with F3, and S-E2 only if F3 will not land within T3. The 0.4.0 successor is `<load-reference-retained>` (D2 §4.9.1, S-G2) |
| Historical physics-source-1 | **As ruled (R-3(b)):** stays eligible after fresh retirement, through its existing reader. A defect found later reopens this |
| Historical all-selected source-blocks-1 | **As ruled (R-7 (i)):** stays Current, with the notice and the summary rule-binding refusal. Retirement for fresh solves does not change it |

Under option A, a fresh invocation never mixes two selected methods. The ordinary cases in an extended envelope carry repaired preview-physics-1 semantics, not precision-1 semantics. That removes, for fresh solves, the reason behind T0R's `SOURCE_BLOCKS_ORDINARY_CASE_LEGACY_SEMANTICS`. D2 owns that reader decision.

**Option B.** Keep exact-block first and the new method for the rest. It needs a mixed-method identity and two receipt families in one envelope. I do not recommend it.

#### 4.4.1 The shared retirement gate (revision 2, V1-S2; adopted by ROOT, defined here, cited by D2)

**Where it applies.** An identity family's exact-block selection is retired for fresh solves only when all four conditions hold on the actual candidate. The families and their slices:
- source-blocks-1 at F2, with D2's S-F;
- physics-source-1 at F2;
- load-reference-source-1 at F3.

**The conditions.**
1. **Coverage.** Take every committed request under `P/fixtures/product_preview/source_blocks/` and `P/fixtures/product_preview/physics_source/`. For F3, also take every committed joined `load-reference-source-1` request (T1's `load_reference_states` carriers and fixtures). Each is solved fresh in both modes, and every case that exact-block selects today must be selected by the new method. A case outside W1's coverage fails the gate, and the family is not retired. D2's H-a limit applies here (D2 §4.9.5): a 0.4.0 case using the logarithmic law is `needs_recompute` under S-G2. That limit is recorded against this condition, not silently excluded.
2. **Budgets.** Those cases complete within the D-8 limits ROOT selects from measurement. The per-case and per-invocation charges are recorded.
3. **Standing.** The fresh results are Current, or `numerically_eligible`, in Rust, Python and TS through D2's S-G readers. They have the same outcome in all three languages, using the shared case files (D2 §4.7).
4. **Values.** Every published quantity that the exact-block projection also publishes agrees with it within the unchanged `|obs − exp| ≤ 1e-9·max(|exp|, scale)`. The scale is the case's R1-style zero scale, or, for these fixtures, the body-level coupled scale stated in the test. Signed six-component reactions and circular maxima, which exact-block's source-blocks-1 rows do not carry, are checked against T0R's preview-physics-1 rules instead.

**Evidence.** The gate runs as one committed test per family in `numerical_robustness` (product lane), plus D2's S-G parity files. Its record lists every request, mode, case, method, charge and standing. A gate failure blocks retirement for that family only; the other families and W1's selection are unaffected.

### 4.5 Interaction with M03-INTEGRITY-v1 and M03-PRODUCT-PREVIEW-EQUILIBRIUM-v1

- **The ordinary attempt** is judged by M03-INTEGRITY-v1, unchanged, in binary64. Its report stays as evidence.
- **The selected extended state** is judged by the proposed `M03-INTEGRITY-MP-v1`: the same structure with `u_p = 2^-p`, the precision-specific pivot and condition screens, residuals at p + 64, and the stop rule. A binary64 rcond screen is never applied to a p-bit factor, and no factor is relabelled.
- **Selected-state equilibrium.** M03-PRODUCT-PREVIEW-EQUILIBRIUM-v1's componentwise measurement is evaluated on the retained state against the intended system, with `γ_p`, and published with the basis label `retained_precision_p`. ROOT either registers it as that policy's precision-aware reading or gives it a successor id; that is within its existing authority (`CONTRIBUTION_PRECISION/SOURCE_QUALIFICATION.md`, "Reliance boundary").
- **Public projection residual.** The residual of the published binary64 displacements on the represented binary64 K is published as a separately labelled observation, not a gate. It can legitimately be worse. For N05 the represented K has the wrong k.
- **Claims.** No certified inertia, forward-error bound or physical accuracy is claimed. Historical DEC-046, DEC-050 and DEC-053 records, and the 64γ binary64 policy, are unchanged.
- **Where each outcome is recorded (revision 2, D2's IF-1).**
  - `numerical_quality.cases[i]` keeps the ordinary attempt's M03-INTEGRITY-v1 outcome for every case, selected ones included.
  - The precision-p outcome ("checks passed at precision p; 2p agreement verified") is recorded only in the receipt.
  - The producer never writes `checks_passed` into `numerical_quality` for a selected case, so D2's DD-11 does not arise. Readers derive a selected case's standing only from the verified receipt (D2 §4.9.4).

### 4.6 Boundary with T5

- **D1 supplies** a passive linear kernel API: for a given selected active state (a fixed restraint set and springs), solve at p and recover.
- **D1 does not** classify contact, decide gap signs at precision, iterate active sets, handle friction, or qualify the mixed recovery basis. `NUMERICAL_INTEGRITY_RECOVERY_BASIS_UNQUALIFIED` and the strict-gap law stay with T5 (M06).
- **At the facade,** D1 refuses every model with a nonlinear support record.
- **The nonlinear loop.** W3 converts the loop's base assembly and linearized solves to the sparse representation, without changing their semantics. The friction influence solves are forced to dense scrutiny today (`nonlinear_integration/src/lib.rs:1336-1342`); changing that is T5's (SUP-16).

### 4.7 W2 — range

**Mechanism: exact force-radix scaling of the ordinary system, in the kernel.**
- `AssemblyEvidence` gains a private `force_scale_exponent: i32`.
- When b ≠ 0, local stiffness is scaled by 2^b before `transform_roundoff`, and `solve()` scales K and f by 2^b.
- Every scaling is exact because it maps normal numbers to normal numbers. Displacements are unchanged. Actions, reactions and residual records are unscaled exactly for publication, with the outcomes defined below.
- Every M03 screen is componentwise-relative, so the scaled evidence is equivalent.
- **Revision 2 (V1-S7):** b is **not** a field of `StructuralReport`. The `Debug`-published report is therefore byte-identical for every current result. When b ≠ 0, the facade publishes one extra evidence line in the integrity diagnostic: `range_scaling: force_scale_exponent=<b>; basis=exact power-of-two`. It appears only in cases that are refused on main today, so no committed byte changes.

**Choosing b (normative, revision 2).**
1. **Evaluate with b = 0**, exactly as today. If the evaluation completes without a `StructuralError::Range`, then b = 0 and nothing else in this section applies.
2. **Only if step 1 fails with `Range`**, collect the nonzero values: every local element stiffness entry, every spring stiffness and every load component.
   - If any of them is subnormal, refuse with the reason "range: subnormal stiffness or load at formation". Its bits were lost before the kernel.
   - Otherwise let `e_min` and `e_max` be the least and greatest binary exponents (`binary_exponent`, `FK/structural.rs:181-189`).
3. **Feasible window.**
   - Let `b_lo = −1022 + 64 − e_min`, the allowance headroom: γ(48) ≈ 2^-47 plus operation growth.
   - Let `b_hi = 1023 − 8 − e_max`.
   - If `b_lo > b_hi`, refuse with the reason "range: exponent span [e_min, e_max] exceeds the binary64 normal window after exact power-of-two scaling".
   - Otherwise `b = ⌊(b_lo + b_hi)/2⌋`, with floor division toward −∞.
4. **Evaluate once with that b.** If this second evaluation also fails with `Range`, refuse with the reason "range: scaled evaluation outside normal range". There is no third attempt.
5. **Unscaling for publication.** Each published action, reaction and residual record is multiplied by 2^-b in bounded exact steps.
   - A normal result is exact.
   - A subnormal result is published with the representability outcome `subnormal`, with its relative precision stated in the evidence line (§5 item 6).
   - A nonzero result that would underflow to zero, or overflow, makes the case `NUMERICAL_INTEGRITY_UNRESOLVED` with the reason "range: publication outside binary64". It is never flushed.
   - Displacements are never scaled.

**Admitted range (ordinary path).** A feasible b exists when every nonzero stiffness entry, spring stiffness and load component is normal binary64 in SI units, and their joint exponent span is at most about 1,980 bits. The displacements must also be normal or exactly zero, since force scaling does not change them.

**Refusal.** The case stays `NUMERICAL_INTEGRITY_UNRESOLVED` with a precise reason, for example: "range: exponent span of stiffness [e_min, e_max] and loads [·,·] exceeds the binary64 normal window after exact power-of-two scaling". The global DOF map is included where it applies. No new diagnostic code is added, so readers need not change.

**PHYS-R4.** By my arithmetic, b ≈ 500 puts EA/L near 3e-3, GJ/L near 3.5e-157, and the `[UY,UY]` allowance near 2.5e-170. All are normal. The public fixture then passes the evidence stage, and the free-DOF set is empty. The published stress path was already repaired (`membrane_publication_range.rs`). The detection run P1 records the actual end-to-end result.

**The extended path** has an internal 64-bit exponent, so range stops mattering inside it. Only publication can refuse: a nonzero quantity that underflows or overflows binary64 is explicit, and a subnormal result carries its reduced relative precision. See the D2 note (§5).

**Formation range (revision 2, from R1's finding 3: `RF-RANGE-…-LEF-small` and `…-LEF-large`).**
- In these cases every input and output is normal binary64, but the product GJ (or EI) is not: it is 2^-1078 and 2^1122 respectively.
- On main, `FK/lib.rs:717-726` forms each coefficient in binary64 (`g * j / length`, `12.0 * e * iy / length3`).
  - An overflow becomes ∞. `validate_named_finite_slice` then fails, and the whole envelope is blocked with a non-finite error.
  - **An underflow becomes an exact 0.** That passes the finiteness check and silently removes that member's torsion or bending stiffness.
  - Downstream that surfaces as a zero diagonal (unresolved), a spurious mechanism witness, or, when a parallel path carries the load, a silently wrong answer. It needs moduli near 1e-300 Pa, so its realistic reach is nil. It is still a silent path, and I record it as a finding for the manager.
- **Revision 2 changes formation in two steps.**
  1. **Checked formation (K2a, `FK/lib.rs`, T1-disjoint, live; its own slice, next after S11-K).** `local_stiffness` forms each coefficient with checked operations. A product or quotient of nonzero finite operands that is zero, subnormal or non-finite is a new `FrameKernelError::NumericalRange { name }`, never a zero coefficient. The one exhaustive match outside `FK`, in `P/core/solver/diagnostics/src/lib.rs:285-362`, gains its mapping in the same slice. On main this turns the silent zero into a refusal; the case is blocked as any formation error is today.
  2. **Scaling at formation (with F1, kernel-owned sparse assembly).** When checked formation raises `NumericalRange`, the b-selection above runs on the predicted exponent of each coefficient: the sum of its operand exponents, which is exact to within ±1 and needs no product to be formed. The kernel then forms the elements with E, G, spring stiffnesses and load contributions scaled by 2^b. Every scaled operand and coefficient is normal, and each scaling is exact.
- **Result for R1's cases.** LEF-small and LEF-large are then **solved** on the ordinary path, and W1 solves them anyway for supported families, because its exponent is 64-bit. **A named range refusal** ("range: exponent span … exceeds the binary64 normal window …") remains only when no single b fits every coefficient and load together. That is a refusal of a representable problem, and VP-ROBUST records it as a failure of that case, not a pass.
- Until F1 lands, checked formation refuses these two cases with its named reason. P1 records main's behaviour.
- **Reach of the silent zero on main (ROOT's question).**
  - **R1's RF-RANGE vectors.** Only the vector −(200, 300, 600) (LEF-small) is affected. There E·I and G·J are about 2^-1078, below the smallest subnormal. So `12.0 * e * iy / length3`, `g * j / length` and their siblings round to exactly 0, and the member loses its torsion and bending stiffness. This is not a 1e-9 question. The published result is an unresolved zero diagonal, a spurious mechanism witness, or, when another path carries the load, a value wrong by far more than 1e-9 (P1 records which). The mirror vector +(200, 300, 600) (LEF-large) overflows to a blocked envelope instead, which is loud, not silent. Every other RF-RANGE vector keeps every coefficient normal: for example (0, −1000, 0) gives E·I about 2^-977 and G·J about 2^-978, and ±(−120, 500, 260) gives E·I of 2^42 and 2^2.
  - **Subnormal coefficients.** These are not zero but lose relative precision (a coefficient near 2^-1050 keeps about 24 bits, so about 6e-8). They exceed 1e-9 in the same way, and checked formation refuses them as well.
  - **Realistic models.** No case exists. A zero or subnormal coefficient needs a section stiffness product E·I, G·J or E·A (divided by at most L³) below about 2.2e-308 in SI units, about 300 orders of magnitude below any physical pipe section. Realistic coefficients lie between about 1e-3 and 1e15.
  - **Conclusion.** The path is silent but unreachable in realistic models. In R1's LEF-small it is a total loss of stiffness, not a small error.

**What W2 does not cover:** length-unit scaling, which would need facade formation in scaled units, and cross-unit display (D2).

**The capture boundary (revision 2, V1-S5, ROOT ruling 4).**
- Today, any request containing a finite `|x| ≥ 2^53` is refused at invocation capture, before W2 or W1 can act, on every route and on T1 as well. An example is a 1e16 N/m "rigid" spring.
- D2 designs the capture fix (R-6). T3 owns it as part of M34 range.
- Once capture admits such values, W2 handles them without change: a 1e16 N/m spring beside N-series stiffness spans about 40 binary exponents.
- W1 handles them exactly: the spring is an exact binary64 input.
- VP-ROBUST adds capture-boundary cases to RF-RANGE (§4.10). A request value of exactly 2^53 − 1, one of 2^53, and one of 1e16 must each give the outcome of the capture design selected at the time: a refusal code today, a solve after the fix.

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
- **One fresh process per model and mode**, run by a standard-library Python runner. It records peak RSS from `/usr/bin/time -v` (Linux) or `-l` (macOS).
  - **Host protection (revision 2, V1-N6).** On Linux the runner applies `RLIMIT_AS` through `resource.setrlimit`. **macOS does not enforce `RLIMIT_AS`.** There the runner polls the child's RSS every 100 ms with `ps -o rss= -p <pid>` and kills it above the cap, recording `killed_by_rss_watchdog` with the last reading.
  - The watchdog is coarser than a hard limit. Runs on the owner's Mac therefore start at sizes whose Linux peak is known to fit the cap with a factor of two to spare. Stage timings (assembly, audit, factor, rcond, solve, residual, recovery) are printed by the binary as JSONL: five repeats, median and minimum.
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

**R1's package (candidate at `6c448d260`, not yet frozen; revision 2).**
- **Represented basis.** Two cases are compared on R1's `expected_represented` values, not on the intended ones: `RF-SKEW-A-CANT-AX-122-r1e-12` and `RF-FINITE-THIRTIETHS-O1e6`. Their intended-input comparison fails through input rounding alone, which no solver can recover. So does every case whose `finite_input` field marks the represented basis.
- **Discriminating controls.** Mutation and stop-rule controls use only the negative controls R1 marks `discriminates`. For the soft families that means k/a from about 1e-8 down; the k/a ≈ 1e-4 cases are continuity controls.
- **Directional springs.** Cases flagged `needs_directional_spring` run in the kernel lane only. The product authors global-axis springs only (`linear_supports/src/lib.rs:217-221`).
- **RF-CANCEL scale.** RF-CANCEL is compared with R1's recommended net-governed scale. That is a scale choice, not a new tolerance, and it is what detects a lost net. See `S11_CONTAINMENT.md` §5.2 for how it relates to the S11 row scale: under C3-full the acceptance does not depend on the row scale, which matters only on the guard path.
- **RF-RANGE.** LEF-small and LEF-large must be solved (§4.7). A named range refusal is recorded as a failure.
- **Capture-boundary cases (V1-S5)**: 2^53 − 1, 2^53 and 1e16 in a request (§4.7).

**The zero-scale floor check (V1-S8).** For every reference quantity, the comparison scale max(|exp|, scale) must be at least R·S\* of the case, with R = 2^-64/1e-9, where S\* is computed from the reference values themselves. A quantity that fails this check is reported to R1 and V2, and the case is flagged as not covered by the stop-rule guarantee for that quantity. It is never loosened.

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
- A small trust base that can be tested thoroughly (expanded in revision 2, V1-S9):
  - at p = 53 against hardware binary64, bitwise, over random normal-range operands;
  - at p = 64L against exact-rational test vectors, generated by a standard-library Python script checked in beside the tests;
  - **targeted hard classes** at every p ∈ {53, 128, 256, 512, 1024} and for every operation:
    - exact ties to even at each limb boundary (bits 63/64, 127/128, …);
    - carry-out and renormalization on addition and multiplication;
    - massive cancellation (operands equal to within one ulp, and to within 2^-p');
    - exact and near-exact division and square root (perfect squares, and values one ulp from them);
    - sticky-bit paths (a nonzero discarded tail beyond the round bit);
    - TwoSum and TwoProduct error-free transformations, the basis of the exact-sum rule (§4.1.2);
    - conversion to binary64 across the subnormal boundary, including results that round up into the normal range, without double rounding;
    - overflow and underflow of the binary64 conversion, reported as representability outcomes;
    - `i64` exponent extremes, which must be refused, never wrapped;
  - **a large seeded differential** against the Python `Fraction` oracle: at least 10^6 operations per precision, generated with a fixed recorded seed, with the vectors committed and their hash recorded;
  - **seeded rounding mutants** (round-toward-zero, a dropped sticky bit, ties away from zero, an off-by-one limb shift). Each must be killed by the suite.
- Estimated at about 800 to 1,200 lines with its tests. This is an estimate.
- `dashu-float` is the fallback if review rejects the in-repo arithmetic.
- `rug` is not recommended: LGPL-3.0+ static linking into the MIT macOS bundle, a C build, and `links = "gmp"`.

## 5. Interface note for D2

What D1 publishes and what readers would need to verify. **D2 owns the successor-identity readers** (S-G, D2 revision 2 §4.9; ROOT's application of V1-S3). This section is their interface, and it follows D2's G1–G8.

1. **The receipt** is the top-level closed member (D2's placeholder `retained_precision`), with a `body` and a `receipt_sha256`. There is one entry per load case, in request order: `selected`, `unavailable` or `not_required`. A `selected` entry holds:
   - method token and policy id;
   - source identity digest;
   - the attempts list (p, outcome, reason, work), ending with the accepted attempt;
   - selected p and verification p;
   - stop-rule summary: the worst normalized disagreement per body and kind, and, for combinations, per combination;
   - S\* per body and kind;
   - the verified-accuracy classification (§4.1.6): the list of `absolute_verified` result ids with their bound. Every other published quantity is `relative_verified`;
   - pivot margin minimum, rcond estimate at p, and the retained-residual summary;
   - the load-ledger digest (the exact per-DOF expansions, hashed);
   - retained-state digest;
   - a reference to the ordinary attempt, bound to `numerical_quality.cases[i]` and its diagnostic.

   An `unavailable` entry carries its reason, its attempts and a reference to its `RETAINED_PRECISION_UNAVAILABLE` diagnostic. A selected case never carries its own unavailable diagnostic.
2. **Canonical profile and encoding (V1-S4, aligned with D2 G1, G2 and G5).**
   - **The profile is the checked profile `openpipestress_jcs_ijson_v1`,** the one today's source receipts use (`source_receipt.rs:28-35`). Every reader already has it. The scientific profile would need a TS canonicalizer that does not exist.
   - **Encoding.** Every receipt value that can exceed 2^53 − 1 in magnitude, be subnormal or be negative zero is a 16-hex binary64 bit string, as `source_receipt.rs:36-38` `bits()` does. That covers the stop-rule ratios, the pivot margin (typically 1e16 to 1e140), rcond, S\*, R and every bound. Plain JSON numbers appear only for exact integers within ±(2^53 − 1): counts, precisions and work units.
   - **Per-case hashing failure.** If a selected case's entry cannot be encoded, the case becomes `unavailable` with reason `receipt_encoding` and its ordinary standing. With the rule above this should be unreachable; it is kept as the defined outcome.
   - **Publication-hash failure.** `publication_sha256` covers the envelope minus the receipt. It fails when a published row value is ≥ 2^53 in magnitude, which is today's capture and carrier range limit (V1-S5, R-6). In that case the invocation is republished under its base identity (preview-physics-1, physics-1 or load-reference-1), with every attempt declined as `RETAINED_PRECISION_UNAVAILABLE` (reason `publication_hash_range`), each case keeping its ordinary standing. This follows T1's SF-1 pattern, continuing the same work ledger. **Never an `Err`, and never a blocked envelope.**
3. **`numerical_quality` (IF-1).** It keeps the ordinary attempt's outcome for every case (§4.5). Standing for a selected case comes only from the verified receipt.
4. **Row provenance.** Every row of a selected case carries `recovery_method = contribution_preserving_multiprecision_v1`, in the field `PP:2906` already uses. Derived stress rows say "from once-rounded retained actions". Combination rows formed over retained states carry the same token.
5. **Identity and emission.** See §4.4. The envelopes are mixed per case by design, so no invocation-level `Err` exists for them. Retirement of exact-block selection is gated per family (§4.4.1).
6. **Replay** is a Rust validation-lane audit, not a standing input (D2 §4.9.4). Rust replays bit-identically from the capture.
7. **Representability per published quantity:** normal, subnormal (reduced precision stated), underflow refused, or overflow refused. Transport and display of subnormals belong to D2.
8. **W2.** `force_scale_exponent` is **not** in `StructuralReport` (V1-S7). When b ≠ 0 there is one added evidence line in the integrity diagnostic. Envelopes with b = 0 are byte-identical to today.
9. **S11.** `LOAD_CONTRIBUTION_ABSORBED` (warning), with `affected_refs` = case id plus contributing load ids. Under C3-full it should not occur for correctly assembled cases (`S11_CONTAINMENT.md` §6).
10. **SUP-17.** A message text change only.
11. **Selected-UNAVAILABLE alignment.** Under option A, fresh solves no longer emit `SOURCE_BLOCK_RECOVERY_UNAVAILABLE` beside a selected case. D2's alignment then concerns historical and joined envelopes only.
12. **If ROOT chooses option B instead,** D2's composite and alignment designs must also cover mixed exact-block and multiprecision envelopes.

## 6. Slices, order and T1 serialization

**Owners.** Kernel slices are T3 TASKs with one writer per file at a time. Facade slices go to the single `core/product_physics` integration owner named in the graph, after T1 merges and main is merged into the T3 branch. ROOT serializes that owner against other tranches.

| Slice | When | Write set | Tests |
|---|---|---|---|
| **S11-K**: S11 kernel and library half (revised with `S11_CONTAINMENT.md` revision 2; subject to V1) | first, as soon as the host is released | `FK/exact_sum.rs` (new: `ExactAccumulator`, `exact_rounded_sum`, `exact_rounded_dot`) and `FK/lib.rs`; `FK/structural.rs` (`Expansion::rounded()` replaced at `:369-377`, `:406-414`, `:554` with the zero witness; the audit with exact per-DOF force terms through a new entry point); `FK/structural/exact_boundary.rs:361`, `:387`; `SA` (`AssemblyEvidence::with_force_terms`); `P/core/loads/primitive_loads/src/lib.rs` (`LoadLedger`, `AssembledForce`, `ForceTerm`); `P/core/solver/straight_pipe/src/lib.rs` (exact recovery sums E1–E4, E6; per-load equivalent terms); `P/core/solver/curved_bend/src/lib.rs` (`arc_section_resultant_terms`); `P/core/loads/load_case_algebra/{src/lib.rs, Cargo.toml}` (exact combination); `P/core/product_physics/src/pressure_sum.rs` (wrapper) | `S11_CONTAINMENT.md` §9 S11-K tests 1–10; mutations M1, M6, M7, M10, M11; every existing suite passing; the committed-fixture diff with its stop rule (S11 §8.3). Live: `SP` recovery, combinations, the `FK` rounding sites |
| K1: W3 kernel sparse representation and sparse M03 gate | after S11-K (shares `FK/structural.rs` and `SA`) | `FK/structural.rs`, new `FK/structural/sparse.rs`; `P/core/solver/sparse_direct/src/{lib.rs, structural.rs}`; `SA` (sparse `AssemblyEvidence`, carrying S11-K's force terms) | Existing frame_kernel, sparse_direct and nonlinear_integration suites unchanged and passing. New: K bitwise parity, KREV-01 to KREV-05 on sparse, O(nnz) negative witness, dense and sparse outcome parity on N and R and NP-B and NP-D, and S11-K's audit in both representations |
| **K2a**: checked formation (ROOT: early, next after S11-K) | **next after S11-K**, as its own PR to main with full gates | `FK/lib.rs` (checked `local_stiffness`, `FrameKernelError::NumericalRange`); `P/core/solver/diagnostics/src/lib.rs:285-362` (its mapping) | Every existing suite passing and byte-identical; LEF-small's underflow and LEF-large's overflow refused with the named reason (no zero coefficient, no ∞); a subnormal-coefficient control refused; mutation 18; the committed-fixture diff (expected unchanged: no committed coefficient is zero or subnormal) |
| K2b: W2 force-radix scaling | after K1 (shares `SA`, `FK/structural.rs`) | `SA`; `FK/structural.rs` helpers | b = 0 bit-identity on all existing tests and on every `Debug`-published report; a synthetic PHYS-R4 element; RF-RANGE kernel cases; the normative b-rule's branches (§4.7) |
| K3: W1 arithmetic | in parallel with S11-K and K1 (new file) | new `FK/structural/retained/wide.rs`; test vectors and generator under `P/core/solver/frame_kernel/tests/` | §4.11 classes, the differential and the mutants |
| K4: W1a kernel method | after K1 and K3 | new `FK/structural/retained/{source, ledger, assemble, factor, recover, combine, adaptive}.rs` | N05, N06, NP-A (intended), R1's discriminating RF-CHAIN, RF-SKEW, RF-WEAK and RF-FINITE (represented basis where marked), RF-MECH, RF-CANCEL, the k = 1e-28 case, the B1 and S8 controls (§7.3), the exact-block oracle in scope, and mutation controls |
| K5: W4 witness and curved screen | after K2b (V1-N4; shares `SA`) | `FK/rigid_body.rs`, `SA` (geometry, curved screen) | A user-element internal mechanism (NP-C-like), a stabilized companion, near-collinear ties, a curved screen positive and a seeded negative |
| K6: harness observations | after K1 | `P/core/solver/performance_harness/**` | Kernel sparse and dense observations on RF-LARGE; runner dry run, including the macOS watchdog path |
| V-K: VP-ROBUST kernel lane | after R1's references are frozen and K4 | new `P/validation/benchmarks/numerical_robustness/**` | §4.10 kernel lane, the zero-scale floor check |
| **S11-F**: the S11 facade half | **the first facade slice after T1 merges** | `PP` (the ledger at every producer of `S11_CONTAINMENT.md` §4.2, with pushed terms, including T1's eigen sites; `AssembledForce` seams; recovery sums E5, E7–E12; the Sensitive mapping and `LOAD_CONTRIBUTION_ABSORBED`; the enumerated site test); `P/core/product_physics/src/pressure_runtime.rs` (push group operands); T1's `source_recovery.rs:609-667` including `:1270-1274`, `source_receipt.rs:218-219` and `:320` | `S11_CONTAINMENT.md` §9 S11-F tests 1–8, including the invariant test and V1's 0.4.0 test; mutations M2–M5, M8, M9; the committed-fixture diff with disclosure (S11 §8.3) |
| F1: facade sparse wiring, W2 at formation, SUP-17 | after S11-F | `PP` (assembly `:1378`, `:1452`, now the kernel's sparse assembly with formation-time scaling; reduction `:1855`; reactions `:2135`; `solve_preview_reduced_system`); `source_recovery.rs` (refuse scaled evidence); the nonlinear loop in `nonlinear_integration/src/lib.rs` (with T5) | Full product suites, the PHYS-R4 public fixture, LEF-small and LEF-large solved, the parity protocol, the dense-scrutiny guard |
| F2: facade W1a wiring, identities, gate | after F1 and ROOT's identity reservation; **atomic with D2's S-G1** | `PP` (case loop, receipt, rows, exact combinations), new `P/core/product_physics/src/retained_publication.rs`, tables in `P/fixtures/results/`, schemas (T1 files) | Product lane of VP-ROBUST; N05 and N06 through the public entry; **the retirement gate (§4.4.1) for source-blocks-1 and physics-source-1**; the characterization tests for retired exact-block selection replaced and recorded |
| F3: W1b families and the 0.4.0 successor | after F2 and the RF-ELOAD addendum; **atomic with D2's S-G2 and S-E1** | `PP` load builders to `PrimitiveSource` and the ledger; kernel element-load primitives | RF-ELOAD; 0.4.0 prescribed and eigen cases; **the gate for `load-reference-source-1`**, which then stops being fresh (R-3(a)) |
| R: readers | per D2 | D2's write sets (S-G and the rest) | D2's plan |
| V-P: product lane and scale runs | after F1 and F2 | `numerical_robustness/**` | §4.10, §4.8 |
| Join | last | ROOT and the manager | §7.5 |

**Order.**
- **Before T1 merges.** S11-K comes first, when the host is released. Its live parts are repairs in T1-disjoint files (`SP`, `load_case_algebra`, the `FK` rounding sites), not interim containment (D-S11-3). S11-K is a full product slice and lands as its own PR to main with full gates (§7.6), including the fixture stop rule (D-S11-3). **K2a (checked formation) comes next**, as its own PR with full gates. Then K1, K2b and K5 in that order, all serialized because they share `SA` and `FK/structural.rs`.
- **Why K2a is separate from S11-K and not bundled.** The two share only `FK/lib.rs`, and only trivially: S11-K adds the `pub mod exact_sum;` declaration there, while K2a changes `local_stiffness`. Otherwise K2a writes only the diagnostics crate, and S11-K writes `FK/exact_sum.rs`, `FK/structural.rs`, `SA`, the load and solver crates and `pressure_sum.rs`. Merging K2a after S11-K resolves that one shared file. Their reviews differ too: an exact-summation repair with a fixture diff, against a new refusal of a formation range. Kept apart, each fixture diff is attributable to one change. K2a can be prepared in parallel with S11-K and merges right after it, so the silent path closes early. K3 runs in parallel from the start. K4 follows K1 and K3, then K6 and V-K.
- **After T1 merges.** S11-F comes first, then F1, then F2 (atomic with S-G1), then F3 (atomic with S-G2 and S-E1), then V-P and the join. W1c follows in T4's window.
- **Timing.** ROOT's S11 no-interim ruling reopens if T1's merge slips materially or P1 finds a Passed breach in a realistic model (ROOT's adopted text).

**Atomic PRs.** F2 with D2's S-G1, and F3 with S-G2 and S-E1: producer, readers and gates merge together, as T0R did.

**T1 overlap.**
- S11-K, K1 to K6 and V-K touch no T1 file.
- S11-F and F1 to F3 edit `PP`, `source_recovery.rs`, `source_receipt.rs`, the schemas and the fixtures, all of which T1 changes.
- `nonlinear_integration` is disjoint from T1 but is T5's future area, so ROOT serializes it.
- The `diagnostics` crate (K2a) is T1-disjoint.

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
- **R1 families (revision 2)**, once V2 refutes them and ROOT freezes them, with R1's zero scales. Specifically:
  - the represented basis for `RF-SKEW-A-CANT-AX-122-r1e-12` and `RF-FINITE-THIRTIETHS-O1e6`, and for every case whose `finite_input` marks it;
  - the discriminating negative controls as mutation tests;
  - RF-CANCEL on the net-governed scale;
  - LEF-small and LEF-large solved;
  - the zero-scale floor check (§4.10).
- **The DEC-053 nine observations,** under their unchanged predicates.

### 7.2 Detection run P1 (on main, before implementation)

Expected, recorded per quantity:

| Family | Expected on main |
|---|---|
| RF-CHAIN | k/a ≈ 1e-4 (and 1e-6): passes. These are continuity controls (R1 finding 2). **From about 1e-8 down**: Sensitive rows that miss 1e-9, or unresolved (absorbed contribution). R1's lost-soft, stored-assembly and subtract-rounded controls discriminate from 1e-8 down. Order > 2 means exact-block is unavailable |
| RF-SKEW, RF-FINITE (soft) | Misses of 1e-6 to 1e-3 from k/a ≈ 1e-8 down, or unresolved; exact-block unavailable (not a signed permutation). The two represented-basis cases are compared on represented inputs |
| Axis-aligned bending soft modes | Misses of about 1e-5 (§3.1), or unresolved |
| RF-WEAK | Likely passes, or unresolved through an absorbed contribution; to be observed |
| RF-LARGE | 1,000 and 10,000 members: refused, or out of memory under the runner's cap (§2.1). Long cantilever chains may also be unresolved on conditioning. That is my expectation, not a measurement |
| RF-RANGE, PHYS-R4 | `NUMERICAL_INTEGRITY_UNRESOLVED` (range) at the extremes. **LEF-large:** a blocked envelope with a non-finite formation error. **LEF-small:** the underflowed GJ becomes an exact 0, so an unresolved zero diagonal, a spurious mechanism, or a wrong value; P1 records which |
| RF-CANCEL | (G, n, −G) and (n, G, −G) Passed and Current with the fold's answer: within 1e-9 at G = 1e5 and 1e6, failing at 1e7 and 1e8, and the net lost at 1e80 (S11). (G, −G, n) exact |
| S11 recovery (V1's probe A as a product model: one member with uniform loads (G, 0.3, −G)) | Passed with the recovery fold's member actions: root shear and midspan moment off by more than 1e-9 of their own magnitude at G ≥ 1e7 (probe: 6.8e-9 at 1e7), and the solve's force also folded. A Passed breach with realistic magnitudes is reported to the manager at once (ROOT's reopen trigger) |
| RF-ZERO, RF-INVARIANCE (well-conditioned) | Pass |
| RF-MECH | Refused with a witness for qualified bodies. Large ones: memory, as for RF-LARGE |

Every P1 run protects the host: `RLIMIT_AS` on Linux, the RSS watchdog on macOS.

### 7.3 Mutation controls (each must fail at least one comparison)

Mutations run against R1's discriminating controls. A mutation that no discriminating comparison kills is reported, and its case set is extended before implementation continues.

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
13. **(revision 2, V1-B1)** Fold loads at p instead of the ledger. V1's check L is then accepted with error 0.5 (§3.2 B1-L), and RF-CANCEL fails.
14. **(revision 2)** Combine retained states term by term at p. The combination A + B − A2 is accepted with error 1.0 (§3.2 B1-C).
15. **(revision 2)** Take combination outputs out of the stop rule. The combination (P, ε) − P is published from 128 bits with error 1.0 (§3.2 B1-E).
16. **(revision 2)** Assemble stiffness entries or recovery sums by sequential addition at p. A duplicate-operand cancellation control fails: two identical members meeting at a node, plus a third member 2^-300 as stiff.
17. **(revision 2, V1-S8)** Drop the verified-accuracy classification. S8-W's far-node quantities are then labelled `relative_verified`, and the reader check fails.
18. **(revision 2, K2a)** Revert to unchecked formation. LEF-small's torsion coefficient becomes 0, and the underflow control fails.
19. **(S11)** Mutations M1–M11 of `S11_CONTAINMENT.md` §9, among them the restored recovery fold (killed at G ≥ 1e7 in probe A), the restored ledger fold, a producer outside the ledger, a pre-summed curved thermal, a naive rounding, a −0.0 ledger zero, and each missed T1 site.

### 7.4 Arithmetic (V1-S9)

§4.11's targeted classes, the seeded `Fraction` differential and the seeded rounding mutants run in `frame_kernel`'s own suite, and so in hosted CI.

### 7.5 Native witnesses (owner's Mac)

- N05 and N06 through the desktop, with the new method's token selected.
- An RF-SKEW soft case (k/a ≤ 1e-8).
- A ten-member RF-CHAIN.
- A 1,000-member model: time and peak memory recorded.
- The PHYS-R4 public fixture.
- An RF-CANCEL case and the realistic thermal case (a 4.1e7 N thermal pair after a 1.3 N co-axial load; `S11_CONTAINMENT.md` §9 F3), after S11-F.
- Save and reopen, and the export surfaces.
- A model outside coverage, such as an elbow, showing `RETAINED_PRECISION_UNAVAILABLE` with ordinary standing.

Record the candidate, the model hashes and the case ids. If a witness is not available, record it as outstanding.

### 7.6 Gates

- A complete-diff independent review.
- Hosted CI: the numerical cargo suite, including the new crate, and the full dual-viewport dispatch for surface 4.
- A clean DEC-025 sweep.
- The native witnesses.
- The retirement gate (§4.4.1), before any family is retired.
- VP-ORACLES and VP-ROBUST passing on the merged candidate. This is the graph's closure rule.
- **Early live slices (ROOT, D-S11-3).** S11-K and K2a each land as their own PR to main, with a complete-diff independent review, hosted CI including the surface-4 dual-viewport dispatch, a clean DEC-025 sweep, and the committed-fixture diff with its stop rule (`S11_CONTAINMENT.md` §8.3). Their change records carry the disclosure of §8.3 there (no in-band marker, D-S11-4).

## 8. What T3 completes and what remains, per group

| Group | D1 completes (when merged and verified) | Remains |
|---|---|---|
| M03 | General accuracy for the W1a and W1b families; **the S11 load-cancellation repair on every route, force and recovery side (S11-K and S11-F)**; the PHYS-R4 and formation range (W2, checked formation); the rigid-null witness for user and screened curved bodies; the SUP-17 wording; VP-ROBUST | Curved bends in the method (W1c, with T4); components and releases (proposed T4 and T7); equivalent-static (open, no owner); nonlinear mixed recovery and gap classification (T5); the S11 audit inside the nonlinear loop's linearized solves, and friction terms added in binary64 (`nonlinear_integration/src/lib.rs:1642-1659`), open with T5 (S11-V5) |
| M32 | Sparse assembly, reduction, reactions and the gate in both modes; the parity and memory protocol; dense scrutiny kept, with a guard | Friction influence solves (T5, SUP-16); resource ceilings chosen by ROOT from measurement |
| M34 | Solve-side range (W2 at formation); publication representability outcomes; the capture-boundary cases in VP-ROBUST | The capture-hash fix (D2, R-6), display and transport range, the scientific carrier, comparison policy (D2 and T6) |
| N05 ordinary accuracy | Repaired on the ordinary and exact routes for W1a models | — |
| General retained-source recovery | Order > 2, skewed and weakly coupled systems, and larger systems up to the measured budgets; W1b load producers; exact combinations over retained states | Curved, pressure regions, components (W1c and later) |
| Standing (with D2) | The retirement gate (§4.4.1) and the receipt interface; `load-reference-source-1` retires at F3 under the gate | Case-scoped standing, including S11's envelope-level Sensitive side effect (T6, as ROOT recorded) |

Real piping models almost always contain weight and elbows. Until W1b and W1c land, most of them get ordinary standing, or the unavailable diagnostic, when Sensitive. S11's repair of load cancellation (recovery sums in `SP` and combinations from S11-K; the ledger and the rest from S11-F) applies to every route and every producer from the first slice after T1 merges.

## 9. Decisions for ROOT

| ID | Decision | Options | Recommendation |
|---|---|---|---|
| D-1 | General method | (A) multiprecision with basic-deformation formation; (B) global reformulation; (C) generalized exact-block | (A). The probe refutes (C) (§3.1) |
| D-2 | Arithmetic backend | In-repo `wide.rs`; `dashu-float`; `rug` | In-repo, with §4.11's test plan (V1 agrees in V1-S9); `dashu-float` as the fallback |
| D-3 | Method policy (revised) | `M03-INTEGRITY-MP-v1`: the exact-sum rule (§4.1.2); the stop rule `2^-64·max(\|q_2p\|, S*)` with body-level coupled scales, applied to case and combination outputs; the equilibrium basis `retained_precision_p`; the 128/256/512 schedule with a 1024 ceiling | Register as proposed. The stricter per-member variant is the alternative |
| D-4 | Method order and identities | (A) retire exact-block selection for fresh solves, exact-block kept as oracle; (B) exact-block first plus the new method, in a mixed identity | (A), under the retirement gate (§4.4.1). Reserve three successor identities (names are placeholders): `<preview-retained>` and `<physics-retained>` at F2, `<load-reference-retained>` at F3 |
| D-5 | Trigger | Not Passed (the current Sensitive boundary), plus represented-only negative energy (V1-N2) and S11-Sensitive; or a stricter boundary | Not Passed as stated. Revisit if P1 finds a Passed case that misses 1e-9 |
| D-6 | W2 | Force-radix scaling at formation, with the normative b-rule and checked formation; the admitted range; no new diagnostic code (one new `FrameKernelError` variant) | As proposed |
| D-7 | W3 fallback and guards | No automatic fallback, dense scrutiny explicit and guarded; or automatic dense fallback on sparse representation errors | No automatic fallback. Ceilings from measurement |
| D-8 | Budgets | Per-case and per-invocation work limits for the method | Select after the K6 and V-P measurements, as RESOURCE_POLICY did. The gate's coverage condition must fit inside them |
| D-9 | W4 | Constrained-body witness; curved objectivity screen | As proposed; T4 confirms the curved construction |
| D-10 | Serialization | S11-K → K2a → K1 → K2b → K5 (S11-K, K1, K2b and K5 on `SA`; K2a on `FK/lib.rs`); `nonlinear_integration` with T5; curved screen with T4; facade after T1, with S11-F first | As in §6 |
| D-11 | R1 addendum | RF-ELOAD references (uniform, thermal, thrust, constant effort, prescribed motion) before W1b | Commission it after V2 |
| D-12 (new) | Below-floor quantities (V1-S8) | (a) publish them, classified `absolute_verified` in the receipt, with readers enforcing the label; (b) withhold every nonzero quantity below R·S\* | (a). (b) would withhold structural zeros and correct weak-coupling responses |
| D-13 (new) | S11 containment | C3-full as S11-K and S11-F (ledger plus exact recovery, `S11_CONTAINMENT.md` revision 2); C3-detect as the fallback; no interim containment (ROOT's adopted text); sub-decisions D-S11-1 (zero witness), D-S11-2 (all three `FK` rounding sites), D-S11-3 (live T1-disjoint repairs in S11-K), D-S11-4 (no in-band marker) in S11 §11 | ROOT decided D-S11-1 (witness for published diagnostic renderings only), D-S11-2, D-S11-3 (full gates, own PR) and D-S11-4 at `4862a72a9`. The pre-acceptance of C3-full awaits V1's backcheck of S11 revision 2 |
| D-14 (new) | Equivalent-static owner (V1-N5) | Assign a tranche, or record it as an open remainder | ROOT assigns |

**Owner-level.** None, if D-2 is in-repo or `dashu-float`. If ROOT prefers `rug`, then statically linking LGPL-3.0+ GMP and MPFR into the MIT-licensed, signed macOS bundle raises relinking obligations. That is a licence and governance matter for the owner. The options are: avoid it (recommended); ship with LGPL compliance measures; or dynamic linking, which is not how gmp-mpfr-sys builds by default. No protected comparison predicate changes; STAGE0 §5.3 stays as ROOT ruled. R1's net-governed scale for RF-CANCEL is a scale choice within the unchanged predicate, not a predicate change.

## 10. Sources, probes and limits

**Read for revision 1.**
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

**Read for revision 2.**
- `T3/REVIEW/RETURN.md` and `T3/REVIEW/_run_records/v1_stop_rule_probe.*` (check C values);
- `T3/ROOT_RULINGS_V1.md` (at `12f2122cd`, and again at `2d07cad7f` and `4862a72a9` for the S11 rulings and decisions);
- `T3/REVIEW/S11_CHECK.md` (at `56b651282`);
- `T3/MANAGER_NOTES/{V1_DISPOSITIONS, S11_MAP, D2_ON_D1_RETIREMENT}.md`;
- `T3/TASK_BRIEFS/R1_ADDENDUM_CANCEL.md`;
- D2 revision 2 at `d566713e9` (§3.1, §4.9, §4.6.2 interface rows);
- R1's `README.md` at `6c448d260` (not `references.json`).
- Source at `c61a540ea`:
  - `primitive_loads/src/lib.rs:1389-1410`;
  - `source_recovery.rs:520-600`, `:1285-1310`;
  - `source_receipt.rs:130-175`, `:570-667`;
  - `source_receipt/composite.rs:936-948`;
  - `exact_boundary.rs:355-400`;
  - `PP:868-960`, `:1424-1500`, `:1660-1700`, `:1752-1842`, `:2880-2935`, and the force-accumulation sites;
  - `result_export/src/semantic_contract.rs:360-440`, `source_blocks.rs:1225-1245`;
  - `FK/lib.rs:317-366`;
  - `diagnostics/src/lib.rs:285-362`;
  - for S11 revision 2: `SP` (recovery sums), `CB` (`arc_section_resultants_with_radial_pressure`), `load_case_algebra/src/lib.rs:291-350`, `pressure_sum.rs`, `self_weight.rs:470-490`, `FK/structural.rs:60-130`, `:320-420`, `:500-570`, `:940-980`, `FK/rigid_body.rs:225-250`, `exact_boundary.rs:195-240`, `:720-800`, `:1012-1032`, `nonlinear_integration/src/lib.rs:1636-1662`, `PP:2118-2145`, `:2320-2345`, `:7485-7600`, `:7650-7680`, `:7730-7800`, `:7960-8356`; and at T1, `source_recovery.rs:600-670`, `:1262-1280`, `source_receipt.rs:200-225`, `:305-325` (with `git show`).
- Not read: `NUMERICAL_REFERENCE.md` in full, and `DEFAULT_ROUTE_DESIGN/ROOT_RULINGS.md` beyond what `ROOT_SELECTION` and DESIGN quote.

**Ran** (all standard-library Python 3.11.15, `nice 19`, each under 1 s).
- `probe_skew_precision.py` → `.stdout.json`: revision 1's evidence, unchanged. Its emulation rounds every operation to p bits with an unbounded exponent. It is not Rust or MPFR, and not a product run. Its scales are a probe convenience, not references.
- `probe_rev2_b1.py` → `.stdout.json`: §3.2 (B1-L, B1-C, B1-E, S8-W). It imports the revision-1 emulation unchanged.
- `probe_s11_audit.py` and `scan_load_fold.py` → `.stdout.json`: `S11_CONTAINMENT.md` (revision 1 evidence, kept).
- `probe_s11_rev2.py` and `scan_element_loads.py` → `.stdout.json`: `S11_CONTAINMENT.md` revision 2 (invariant, mutation, orders, combinations, guard amplification, zero witness; element loads per member).
- `index_probe.py` → `.stdout.json`, and `crates_api_probe.txt`: crates.io metadata. Nothing was downloaded or built.
- Python versions: `_run_records/python_version.txt`. Hashes: `_run_records/SHA256SUMS`, with paths relative to this folder (`DESIGN_NUMERICS/`).

**Limits.**
- **Estimates.** The dense-memory figures (§2.1), the PHYS-R4 scale arithmetic (§4.7), the in-repo arithmetic size and every cost statement are estimates from source and arithmetic, not measurements.
- **Expectations.** The P1 table (§7.2) states expectations only.
- **Probe scope.** One body, at most six members, nodal loads only, dense LDL in emulation. The exact expansions are emulated with `Fraction` sums rounded once, not with TwoSum in `Wide`. No sparse profile at p, Rust arithmetic, budgets or performance is tested.
- **Not a proof.** §4.1.9's argument that the remaining factorization error is precision-dependent is an argument, not a proof.
- **Unverified here.** The `rug` C-build fact comes from crate documentation, not from this host.
- **To confirm.** T4 should confirm the curved null-space claim. R1's references are candidates until V2 and ROOT's selection.
- **Drift.** Line numbers drift with T1.
- **Not run.** Existing suites, CI, the native app and a DEC-025 sweep.
