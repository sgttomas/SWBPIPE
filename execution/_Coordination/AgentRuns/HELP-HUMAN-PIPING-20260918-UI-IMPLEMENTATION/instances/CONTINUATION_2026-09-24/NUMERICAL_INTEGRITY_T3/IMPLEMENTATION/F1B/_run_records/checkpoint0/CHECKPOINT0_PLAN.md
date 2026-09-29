# I13 (F1b) checkpoint 0: the plan

- **TASK:** I13, a Chirality Type 2 TASK (implementer) for facade slice F1b of T3.
- **Delegation mechanism:** a Claude Code background subagent launched by ROOT (HELP_HUMAN) through the Agent tool. ROOT is the only return path. I13 delegates nothing.
- **Brief:** `T3/TASK_BRIEFS/I13_F1B_IMPLEMENTATION.md` at numerics `28bb8dfa8` (sha256 `57cce4f6…`, re-hashed on disk), with its "ROOT rulings for this slice (2026-09-28)" (Q1–Q14) and `ROOT_RULINGS_V1.md` "F1b: spawn and rulings".
- **Read:** Root `AGENTS.md`, `agents/AGENT_TASK.md`, `_COMMON.md`, `I8R_K1_RESUME.md` ("The Mac host", "Platform calibration"), DESIGN §1, §2.1, §4.6–§4.8, K1 RETURN §9, §11, §12, K2b RETURN §5, §6, §9–§10, §13.3, §14, §15 and addenda 1–2, `K2B_MERGE/RECORD.md` "For F1b", the RV11 rulings, "F1 split" and "F1a: D-5 evidence-line format", I7 (write set, tests, addendum 1), and G1's gate-baseline README.
- **Base:** `<wt>/f1b`, branch `codex/piping-f1b-20260928`, main `e7d930d49`, clean. No product code was written this turn. No Git write.
- **Scratch experiments this turn** (standard library, read-only, under `<wt>/scratch/i13/exp/`):
  - `lef_large_census.py`: the product-level census b for LEF-large;
  - `rf_range_base.py` → `rf_range_base.out`: every RF-RANGE run on G1's Mac base (`<wt>/scratch/gate_base_e7d930d49/runs.jsonl`);
  - `committed_model_sizes.out`: the drafter's `committed_model_sizes.py` on the base.

Abbreviations: `P/` = `projects/chirality-piping/`; PP = `P/core/product_physics/src/lib.rs`; FK = `frame_kernel/src/lib.rs`; `structural.rs` and `sparse.rs` are FK's; SA = `nonlinear_integration/src/structural_adapter.rs`; SP = `straight_pipe/src/lib.rs`; NI = `nonlinear_integration`.

---

## 0. Citations re-located on the base

Every file:line citation in the brief was re-located on `e7d930d49`. They are current, with these exact positions:

- PP `:1826` (first-basis assembly), `:1834-1838` (curved and springs), `:1846-1852` (`basis_solve_states`), `:1873` → `assemble_case_stiffness` `:2276-2291`, `:1957-1972`, `:2332`, `:2360`, `:2426-2442`, `:2481-2500`, `:2505-2509`, `:2514-2533`, `:2667`, `:2675-2681`, `:2726-2735`, `:2745`, `:2759-2771` (`OrdinaryAttempt`), `:2787-2791`, `:2780`, `:2868-2915` (the attempt's outcome match), `:2931-2946`, `:3119`, `:3150`, `:3297`, `:3373-3386`, `:4033`, `:4392-4492`, `:4418`, `:4441-4449`, `:4452-4468`, `:4490`, `:4553`, `:1078-1108`, `:1118`, `:12082-12095`, `:12198`.
- `source_recovery.rs` `:34`, `:466` (the n > 256 budget refusal), `:470-471` (the first read of `stiffness`: its dimensions), `:879` (`AssemblyEvidence::new` of the frames), `:892-905` (the fold comparison).
- `source_receipt.rs` `:309-319` (the replay's own dense K), `:657-660`, `:914-921` (the 12× reservation, at `:921`).
- `sparse.rs` `:48`, `:82`, `:218`, `:288`, `:299`, `:309`, `:325`, `:360`, `:403`, `:592`, `:729`, `:1287-1291`. FK `:24` (`AXIS_TOLERANCE`), `:525`, `:563`, `:582`, `:711`, `:866`, `:991`, `:1073`, `:1129`, `:1146-1153`, `:1217-1219`, `:1864-1867`. `structural.rs` `:2196`, `:2256`, `:2286`, `:2297`, `:2321`, `:2338`, `:2514`.
- SA `:451`, `:468` (`new`; the brief's `:467` is its doc line), `:573`, `:644`, `:1441`, `:1540`, `:1583-1601`, `:1627-1632`, `:1688-1696`, `:1722`. SP `:427`, `:441`, `:521`, `:701`, `:726`. NI `lib.rs:543`, `:581`. `formation_guard.rs:456-459`.
- `k2a_formation_range_runtime.rs`: `SPRING_CARRIED` is `:152`, and its test is `:165-166` (the brief's `:166`). `PARTIAL_UNDERFLOW` is `:188`, and its test is `:249-250` (the brief's `:250`).
- NI `s11k_tests.rs`: the K-D5 pin is `:1116`, with its product half at `:1186-1220`. The K2b pin is `:1584`, with its product half at `:1620-1630`.
- `canonical_json/src/lib.rs:94-116` (the unsafe-integral check is at `:106-107`); PP `:5620-5631`; headless `src/lib.rs:804`.

**Two facts on the base change the brief's write-set expectations** (refinements, not stops):
- **`assemble_case_stiffness` and `add_curved_bend_stiffness_contributions` keep a product caller** in `source_receipt.rs` (`:212` and `:315`, the n ≤ 256 captured replays that Q9 leaves unchanged), and test callers.
  - So they stay, and their rule-8 counts stay 1.
  - Only their calls on the ordinary route are removed.
  - `run_linear_static_preview_captured_once` goes from 4 to 2.
  - PP's `multiply_matrix_vector` (`:12198`) loses its only caller and is deleted (its row is removed).
- **`solve_load_case` is also called by `src/source_receipt/load_state_join_tests.rs:723` and `src/source_receipt/load_state_tests.rs:501`** (tests, with a dense `k` from `assemble_case_stiffness`). Its signature change needs call-site updates there (OQ9).

---

## 1. Split of the work between A1 and A2

- **A1 (the sparse wiring and the guard) is a pure refactor at b = 0.** It covers:
  - Scope 1 **without** the `NumericalRange` deferral: a formation error still blocks exactly as today;
  - Scope 2;
  - Scope 4's dense view (Q9);
  - F1a's N1.

  Nothing may change in published bytes. The whole of main's behaviour, C1 cases included, must be byte-identical at A1, so A1's T9, suites and targeted tests are a clean check of the wiring.
- **A2 (W2) covers:**
  - the deferral;
  - Scope 3;
  - Scope 4's scaled-evidence refusal;
  - the Q10 and Q11 edits;
  - the product-run table and the proposed C1 list.

  Scope 4's refusal lands at A2 because it names `ForceScale` in `source_recovery.rs`, which NI's K2b pin forbids until that pin is rewritten (§8). So A1 touches no NI pin.

---

## 2. The case loop's new structure (exact signatures)

### 2.1 Per-basis state

```rust
/// F1b: one modulus basis's (or one 0.4.0 resolved case's) global stiffness.
/// A1 uses `SparseStiffness` directly. This enum arrives at A2.
enum BasisStiffness {
    /// K1's sparse assembly at b = 0 (frames and users, then the realized
    /// curved bends as blocks, then the springs: the dense order, bit for bit).
    Formed(SparseStiffness),
    /// K2a's `FrameKernelError::NumericalRange` at this basis's assembly, on a
    /// linear invocation: deferred to each case of the basis as its step-1
    /// range trigger (D1 §4.7 step 1; Q2).
    RangeDeferred(FrameKernelError),
}

/// Replaces PP:1826-1838, :1957-1972 and the ordinary-route use of
/// `assemble_case_stiffness` (:1873). `Err` is every formation error that
/// blocks the invocation today (the caller keeps `solver_blocked`): any
/// non-range error, and a range error on an invocation with a nonlinear support.
fn assemble_basis_stiffness(
    built: &BuiltModel,
    springs: &[SpringEntry],
    linear: bool, // built.nonlinear_supports.is_empty(); A1 has no such parameter and returns SparseStiffness
) -> Result<BasisStiffness, FrameKernelError>;
// body: assemble_sparse_stiffness(built.nodes.len(), &built.frame_elements,
//   &built.user_stiffness_elements, &blocks /* StiffnessBlock{node_i,node_j,stiffness:global_stiffness}
//   per built.curved_bend_elements, in order */, &springs /* (node_dof.global_index(),
//   stiffness.value) per boundary spring, in order */, &SparseAssemblyOptions::new())
```

- **`basis_solve_states`** becomes `Vec<(Option<String>, Vec<MaterialInput>, BuiltModel, BasisStiffness, Option<String>)>`. The three sites call `assemble_basis_stiffness`, and `Err(e) => return solver_blocked(model, diagnostics, e)` as today.
- **Nothing dense is held across cases or bases.**
- **Errors:** K1 forms each element by the same call (`global_stiffness`) in the same order, so the first failing element and its error are today's. Blocks and springs are node-checked, which is unreachable in the product (their nodes are validated).
- **The invocation is linear** iff `built.nonlinear_supports.is_empty()`, the same for every basis because supports do not depend on materials. Every later basis and 0.4.0 case uses the same flag.

### 2.2 The dense-scrutiny guard: placement

- It runs **once per invocation, before the case loop**, right after basis 0's assembly outcome is known (Formed or RangeDeferred). Non-deferred formation errors keep today's block, first.
- It runs only when `solver_mode == DenseScrutiny`.
- On refusal: `diagnostics.push(dense_scrutiny_refusal_diagnostic(..)); return blocked_envelope(model, diagnostics);`.
- At this point nothing n² has been allocated. The ordinary route's first n² allocation is inside the first case's SA solve. Q9's dense view (n ≤ 256) and the replays are bounded by 256.
- The estimate depends only on the node count, which is the same for every basis. See §6.

### 2.3 `solve_load_case` (signature)

```rust
fn solve_load_case(
    model: &PreviewModel,
    built: &BuiltModel,
    materials: &[MaterialInput],
    stiffness: &BasisStiffness,          // was &[Vec<f64>]  (A1: &SparseStiffness)
    restrained_dofs: &[usize],
    spring_entries: &[SpringEntry],
    load_case: &PreviewLoadCase,
    modulus_basis_record: Option<&str>,
    solver_mode: PreviewSolverMode,
    capture: Option<&source_receipt::CapturedInvocation>,
    source_budget: &mut SourceRecoveryBudget,
    load_state: Option<&case_state::resolve::ResolvedCase>,
    diagnostics: &mut Vec<Diagnostic>,
) -> Result<LoadCaseSolve, FrameKernelError>
```

### 2.4 The case, in order (everything not named is unchanged)

1. **Unchanged up to the ledger:** loads, the ledger (`case_force_ledger`), `finish_case_ledger`, and S11-G's load-row finding.
2. **The finiteness scan** (`:2675-2681`):
   - `Formed(k)`: `require_finite_mechanics(k.values().iter().copied().chain(force.values().iter().copied()))`. The stored values are row-major, and absent entries are +0.0 (finite). So the first non-finite value is the same entry as in the dense row-major scan, with the same bits, and the error is identical.
   - `RangeDeferred`: the force values only.
3. **`prescribed` and the 0.4.0 prescribed-DOF check:** unchanged.
4. **The partition** (`:2726-2735`):
   - `Formed(k)`: `reduce_assembled_sparse_system(k, &force, restrained_dofs, load_state.is_some().then_some(&prescribed_values[..]))?`. Its errors are main's reductions' errors, in their order (K1). The ordinary route needs only `free_dofs` and `force`; `free_position` is not needed.
   - `RangeDeferred`: `free_dofs` is the ascending complement of `restrained_dofs`. This equals the sparse reduction's `free_dofs` (pinned by a test on Formed cases). There is no reduction, because there is no K.
5. **The observation force:** `legacy_observation_force(&force, k, restrained_dofs, &prescribed, load_state.is_some())`, which now reads `k.get(row, column)`. It is bit-identical, because `get` returns the stored value or +0.0, and the dense K holds +0.0 wherever nothing is stored. For `RangeDeferred` it is not formed.
6. **The ordinary attempt:**
   ```rust
   enum OrdinaryFailure {                       // A2
       Structural(StructuralError),
       Formation(FrameKernelError),              // only from BasisStiffness::RangeDeferred
   }
   let attempted_linear: Result<PreviewLinearSolve, OrdinaryFailure> = match stiffness {
       Formed(k) => solve_preview_reduced_system(solver_mode, k, reduced.force.values(), built,
           spring_entries, &force, &observation_force, &prescribed, load_case,
           &mut preliminary_diagnostics).map_err(OrdinaryFailure::Structural),
       RangeDeferred(e) => Err(OrdinaryFailure::Formation(e.clone())),
   };
   ```
7. **`report_sensitive`, `attempt_err`, `needs_source_recovery` and `source_eligible`:** unchanged. `OrdinaryAttempt` is no longer formed here; see step 11.
8. **Exact-block (Q2 step 2),** where it is eligible: unchanged. The differences:
   - The `Input.stiffness` is the dense view (Q9, §7).
   - The attempt call is `source_recovery::solve_ordinary(input, limits, ForceScale::UNSCALED)` (A2; §7).
   - For `OrdinaryFailure::Formation`, the attempt is replaced by a zero-work named decline, as proposed in OQ2.
9. **The outcome match** (replacing `:2868-2915`):
   ```rust
   let mut ordinary_error: Option<StructuralError> = None;   // for the receipt (step 11)
   let mut force_scaled: Option<ForceScaledPublication> = None;
   let linear_solve = match attempted_linear {
       Ok(solve) => { diagnostics.extend(preliminary_diagnostics); Some(solve) }
       Err(OrdinaryFailure::Structural(error)) if selected_source.is_some() => { /* today's info record */ ordinary_error = Some(error); None }
       Err(OrdinaryFailure::Structural(error)) if permits_contact_seed_trial(..) && eligible_contact_dofs(..).is_some() => { ordinary_error = Some(error); None }
       Err(failure) => match ordinary_range_trigger(&failure).filter(|_| built.nonlinear_supports.is_empty()) {
           // Q2 step 3: W2, only here: linear, range-triggered, exact-block did not recover.
           Some(trigger) => match force_scaling_attempt(/* §3.2 */) {
               Ok((solve, publication)) => { force_scaled = Some(publication); Some(solve) }
               Err(refusal) => { append_force_scaling_refusal(diagnostics, &load_case.id, &refusal, &trigger, model); return Ok(LoadCaseSolve::refused(&load_case.id)) }
           },
           None => match failure {
               OrdinaryFailure::Structural(error) => { append_integrity_failure(diagnostics, &load_case.id, &error, model); return Ok(/* today's empty solve */) }
               OrdinaryFailure::Formation(error) => return Err(error), // unreachable by construction: deferral only when linear
           },
       },
   };
   ```
   - `LoadCaseSolve::refused` is today's empty `LoadCaseSolve` literal, factored out, with the same fields. The five inline copies may stay; it is cosmetic.
   - The `Formation` + nonlinear arm reproduces today's `solver_blocked` through the caller. It is unreachable because `assemble_basis_stiffness` defers only when linear.
10. **Displacements** (`:2917-2930`):
    - They are filled from `linear_solve.solution` over `free_dofs` (step 4), then the prescribed values, as today.
    - The mode row (`append_linear_solver_mode_evidence`) runs as today when `selected_source.is_none()`. At b ≠ 0 its `PreviewLinearSolve` has every observation field `None` (OQ5).
    - **Dense scrutiny's protected lanes:** only when `solver_mode == DenseScrutiny && force_scaled.is_none()`:
      ```rust
      fn dense_observation_system(stiffness: &SparseStiffness, force: &AssembledForce,
          restrained_dofs: &[usize], prescribed_values: Option<&[f64]>)
          -> Result<ReducedAssembledSystem, FrameKernelError>
      // = reduce_assembled_system[_with_prescribed_displacements](&stiffness.to_dense(), ...), built AFTER the attempt
      ```
      Then today's `legacy_dense_observation(&dense_reduced)` and `append_sparse_live_path_evidence` run.
    - The dense reduction cannot fail when the sparse one succeeded, because the errors are the same and in the same order (K1). If it ever failed, the lane is skipped with `.ok()`. That is equivalent to main, which also skips on a failed LU.
11. **The receipt's ordinary attempt follows the published verdict (S11-G G-2).** It is formed after step 9, by one `OrdinaryAttempt::passed(` call (t10b's count stays 1):
    ```rust
    let ordinary_attempt = match (&linear_solve, &ordinary_error) {
        (Some(solve), _) => source_receipt::OrdinaryAttempt::passed(solver_mode, &solve.structural_report,
            integrity_diagnostic_id(&load_case.id), load_row_finding.is_some()),
        (None, Some(error)) => source_receipt::OrdinaryAttempt::rejected(solver_mode, error, integrity_diagnostic_id(&load_case.id)),
        (None, None) => source_receipt::OrdinaryAttempt::not_attempted(solver_mode),
    };
    ```
    - On main's paths the arguments are today's, so the value is byte-identical. In main, `ordinary_attempt` is only read at the receipt, at the end of the function.
    - For a W2-published case it records W2's report. That is the published verdict before R-b′. If R-b′ then demotes the case (§3.3), the receipt's `OrdinaryAttempt::wire` check refuses with "ordinary outcome changed", and the invocation's receipt fails closed (`SOURCE_BLOCK_RECOVERY_FINALIZATION_FAILED`, blocking). It never publishes an inconsistent receipt. Main behaves the same way for any R-b′-demoted case in a selecting invocation.
    - A W2 refusal is blocking, so its invocation has no receipt.
12. **Publication:** at b = 0 unchanged, except that reactions use `restrained_reactions(k, &u, &force)` (§2.6). At b ≠ 0, see §3.4.

### 2.5 `solve_preview_reduced_system`'s new signature

```rust
fn solve_preview_reduced_system(
    solver_mode: PreviewSolverMode,
    stiffness: &SparseStiffness,          // was original_stiffness: &[Vec<f64>]
    _reduced_force: &[f64],               // unused, as today
    built: &BuiltModel,
    spring_entries: &[SpringEntry],
    global_force: &AssembledForce,
    observation_force: &[f64],
    prescribed: &[(usize, f64)],
    _load_case: &PreviewLoadCase,         // unused, as today
    _diagnostics: &mut Vec<Diagnostic>,   // unused, as today
) -> Result<PreviewLinearSolve, StructuralError>
```

- **Body:** `SparseAssemblyEvidence::new(stiffness.pattern(), n_nodes, frames, users, &curved, &springs)?`, then `.solve_assembled_with_formation_check(stiffness, global_force, &free, prescribed, mode, &curved_sources, selected)?`. This is the pattern evidence in **both** modes (Q8(d)). In DenseScrutiny, SA materializes the dense view itself (SA:663-683).
- The DEC-050/053 sparse-entry lane is unchanged.
- **The unused parameters are kept** to minimize churn. They are already unused.
- **Fallback (Q8(d)):** if T9 or the gate shows any dense-mode byte difference, the DenseScrutiny branch returns to `AssemblyEvidence::new` plus `solve_assembled_with_formation_check(&stiffness.to_dense(), …)`, and I report it. The guard's constant holds for both representations (§6).

### 2.6 Reactions at b = 0

- `restrained_reactions` is kept as E12's named site (its rule-8 row is 0): `fn restrained_reactions(stiffness: &SparseStiffness, displacements: &[f64], force: &AssembledForce) -> Vec<f64>`, which delegates to `stiffness.reactions(displacements, force)`.
- A dimension mismatch (unreachable: both are 6·nodes) gives a NaN vector, which `require_finite_mechanics` refuses as today.
- PP's `multiply_matrix_vector` is deleted, because it has no caller.

---

## 3. W2 in the product (A2)

### 3.1 PP's range classifier and its pinned equality with SA's

```rust
/// D1 §4.7 step 1 as SA's `evaluate_force_scaled` classifies it (SA:1627-1632,
/// :1688-1690): K2a's NumericalRange at formation, or StructuralError::Range from
/// the evidence or the solve. Everything else is not a range trigger.
fn ordinary_range_trigger(failure: &OrdinaryFailure) -> Option<RangeTrigger> {
    match failure {
        OrdinaryFailure::Formation(e @ FrameKernelError::NumericalRange { .. }) => Some(RangeTrigger::Formation(e.clone())),
        OrdinaryFailure::Structural(e @ StructuralError::Range(_)) => Some(RangeTrigger::Evaluation(e.clone())),
        _ => None,
    }
}
```

- **SA's third arm** (`:1691-1696`: `Refused(ScaledEvaluation)` → range) cannot occur at b = 0. At `UNSCALED`, the force-scaled solves take `f` itself and never call `AssembledForce::force_scaled` (SA:770-777, :838-845). `new_force_scaled(UNSCALED)` is `new`. So no `Refused` value exists at b = 0 (derivation D1, §10).
- **The pin (test D-CLASS):** for every case in B and D, build the case's `ForceScalingCase` and call `solve_with_force_scaling`.
  - If PP classifies `None`, the orchestrator must return PP's own b = 0 result: `Debug`-equal solution, or an equal error.
  - If PP classifies `Some(t)`, the orchestrator must go to b ≠ 0: `Ok` with b ≠ 0, or `Refused { trigger: Some(t) }` with PP's `t`, or a b ≠ 0 non-range error.
  - This kills F1B-M16.

### 3.2 The W2 attempt, admission and publication

```rust
/// D1 §4.7 steps 2-5 through K2b's orchestrator (the only W2 entry), for a linear
/// case whose ordinary attempt range-triggered and that exact-block did not recover.
fn force_scaling_attempt(
    model: &PreviewModel,
    built: &BuiltModel,
    spring_entries: &[SpringEntry],
    load_case: &PreviewLoadCase,                 // the effective case (0.4.0: resolved)
    load_application: &LoadApplication,
    thermal_loads: &[ThermalElementLoad],
    pressure_thrust_loads: &[PressureThrustLoad],
    exact_pressure: Option<&pressure_runtime::ExactPressureCase>,
    force: &AssembledForce,                      // the case ledger, unscaled (RV11-N4)
    prescribed: &[(usize, f64)],
    solver_mode: PreviewSolverMode,
) -> Result<(PreviewLinearSolve, ForceScaledPublication), ForceScalingFailure>;
```

- **The `ForceScalingCase`** is exactly today's linear case at b = 0:
  - `node_count`, `built.frame_elements`, `built.user_stiffness_elements`;
  - `curved`, formed as in `solve_preview_reduced_system` (`CurvedBendStiffnessElement::from_macro_element`), and `curved_sources`;
  - `springs` as `(dof, k)`, `force`, `prescribed`;
  - `mode`;
  - `selected: true` (linear);
  - `representation: EvidenceRepresentation::Pattern`.
- **Order inside, fixing the precedence of the refusal reasons:**
  1. `solve_with_force_scaling(&case)`. Its `Err` is `ForceScalingFailure::Refused(r)` or `::Failed(e)`. An `Ok` with b = 0 is `::NotEngaged`, a defensive fail-closed case that is unreachable by D1.
  2. **Admission (Q3), checked on `Ok` at b ≠ 0:** `force_scaling_admission(..)`. So the orchestrator's own refusals (census, window, scaled evaluation) take precedence. That is what §6's PHYS-R4 prediction and the D test's "at b ≠ 0" wording require (OQ7).
  3. **Step-5 publication,** before any row is built: `force_scaled_publication(..)`. Any refusal refuses the case, and no row of it is ever built.
- **c1 (OQ1):** `fn census_force_scale(case: &ForceScalingCase<'_>) -> Option<i32>` recomputes b for the refusal message. It uses FK's public `ForceScaleCensus` (`frame`, `user`, `matrix`, `spring`, `load_term`, `force_scale()`) over the case's public fields, in SA's order (SA:1583-1601). No formula is replicated: the window, midpoint and parity are FK's `force_scale()`. The census scope is five loops. A test pins b equal to `outcome.solution.force_scale` on every success path in D, and equal to K2b's recorded b (312) on the limitation chain.

```rust
/// Q3: at b ≠ 0 only straight frames, ground springs, rigid restraints, prescribed
/// support motion and authored nodal loads.
fn force_scaling_admission(
    model: &PreviewModel, built: &BuiltModel, load_case: &PreviewLoadCase,
    load_application: &LoadApplication, thermal_loads: &[ThermalElementLoad],
    pressure_thrust_loads: &[PressureThrustLoad],
    exact_pressure: Option<&pressure_runtime::ExactPressureCase>, force: &AssembledForce,
) -> Result<(), ForceScalingFailure /* ::NotAdmitted { family, detail } */>;
```

The checks, in this order. The first failure is the refusal, and `<family>` is the token:

| # | Check | `<family>` |
|---|---|---|
| 1 | `built.user_stiffness_elements` non-empty | `user_stiffness_element` |
| 2 | `built.curved_bend_elements` non-empty | `curved_bend_macro_element` |
| 3 | `load_application.element_uniform_loads` non-empty | `uniform_element_load` |
| 4 | `thermal_loads` non-empty (thermal, and the 0.4.0 eigen loads) | `thermal_or_eigen_load` |
| 5 | `pressure_thrust_loads` non-empty | `pressure_thrust_load` |
| 6 | `exact_pressure` with a non-empty `assembled_operands` | `exact_pressure_operand` |
| 7 | any consumed constant-effort support in the model | `constant_effort_support` |
| 8 | any ledger term that is not a `Term` whose source is the id of one of the case's node-targeted primitive loads | `non_nodal_load_term` |
| 9 | any such nodal `Term` equal to exactly zero (OQ13) | `zero_nodal_load_term` |

- Check 8 is the catch-all behind 3–7.
- **Check 9's rationale:** a nodal magnitude is formed at b = 0 by unit normalization, or by the 0.4.0 factor (`case_state/resolve.rs:1074`). A term that underflowed to exactly zero is invisible to the census (K2b §14). A subnormal one is refused by the census (`SubnormalAtFormation`). A normal one is the value main forms.
- An exact-pressure-profile model with no pressure operand passes (needed for PHYS-R4 without pressure; OQ14).

```rust
/// Step 5 through K2b's checked helpers only (RV11-1, RV11D-1).
fn force_scaled_publication(
    built: &BuiltModel, spring_entries: &[SpringEntry], restrained_dofs: &[usize],
    outcome: &ForceScalingOutcome, force: &AssembledForce,
) -> Result<ForceScaledPublication, ForceScalingFailure /* ::Publication { quantity, reason } */>;

struct ForceScaledPublication {
    force_scale_exponent: i32,                    // b ≠ 0
    reactions: Vec<Option<PublishedValue>>,       // per global DOF; Some at restrained DOFs
    spring_actions: Vec<PublishedValue>,          // per spring_entries index
    end_actions: Vec<[PublishedValue; 12]>,       // per built.pipes index (no curved span at b ≠ 0)
    records: Vec<RecordOutcome>,                  // outcome.solution.records
}
enum ForceScalingFailure {
    Refused(ForceScalingRefusal),                             // steps 2-4
    Failed { error: ForceScaledError, b: Option<i32> },       // RV11-N2: non-range at the chosen b; census Formation
    NotAdmitted { family: &'static str, detail: String, b: i32 },
    Publication { quantity: String, reason: ForceScaleReason, b: i32 },
    NotEngaged,
}
```

- **Reactions:** `outcome.stiffness.force_scaled_reactions(&u, force, scale, restrained_dofs)`. The scale is rebuilt from `outcome.solution.force_scale`, which is carried as the `ForceScale` value. PP never constructs one from an integer.
- **Spring actions:** `force_scaled_spring_action((dof, k), &u, scale)` per spring entry.
- **End actions:** `pipe.frame_element()?.force_scaled_end_actions(&u, scale)` per pipe. With no curved span admitted, `built.pipes[i].frame_element()` is `built.frame_elements[i]` (PP:5633-5641).
- **The ledger is unscaled (RV11-N4), by construction and a tested check:**
  - The only public way to form a scaled `AssembledForce` is `AssembledForce::force_scaled`. The rewritten NI K2b pin (§8) keeps the token `force_scaled(` at zero occurrences in every PP module. So no scaled ledger can exist in PP, and `force` is the binding `finish_case_ledger` returned.
  - The behavioural check is the reach and LEF-large values: a twice-scaled force moves u by 2^b.
  - F1B-M13 is killed by the pin and by D's values.

### 3.3 R-b′ at b ≠ 0 (Q5(a))

- **Unchanged code.** `pipe.bending_formation_bound(&displacements)` runs at b = 0 (SP:701, `local_stiffness` at `:726`). It meets K2a's refusal for the member whose formation left the range, so the bound is unavailable, R-b′ fires (`formation_guard.rs:456-459`), and a Passed case is demoted to Sensitive.
- **Evaluation-trigger cases** (formation normal at b = 0, such as PHYS-R4 without pressure) may compute a bound. R-b′ then decides as today, on the published unscaled q and B.
- ruling C's "CONT Passed" holds at kernel level only.
- The scale-aware bound is on F2a's input list and the T3-close list (Q5(b)).

### 3.4 The publication path of every published quantity at b ≠ 0

Admission leaves: straight members, ground springs, rigid restraints, 0.4.0 prescribed motion and authored nodal loads.

| Published quantity | Site | Path at b ≠ 0 |
|---|---|---|
| Displacement components (mm rows) and magnitudes, `max_displacement` | PP `:3060-3109` (`append_node_displacement_component_results`, `displacement_magnitude`) | **unchanged**: u is never scaled (`unscale_structural_solution` leaves it) |
| Rigid reactions (support vector slots, `reaction_resultant`, exact-profile signed support rows, the preview support vectors) | PP `:3119`, `:3137-3143` | **K2b helper** `SparseStiffness::force_scaled_reactions`; each slot is the `PublishedValue`. The derived magnitude (`hypot`) and the rows are unchanged binary64 functions of the published values |
| Spring actions | PP `:3150` | **K2b helper** `force_scaled_spring_action`, replacing `-k*u` |
| Nonlinear support slots | PP `:3153-3162` | none (linear only) |
| Member end actions (`append_element_force_results`) | PP `:3297` then `:3310-3334` | **K2b helper** `FrameElement::force_scaled_end_actions`. Then today's `equivalent_nodal_load_terms_with_spans(&[], &[])` (empty: it only reads `length()`, SP:584-598) and `exact_straight_end_forces` with no terms (one exact term, rounded once: the value; a −0.0 becomes what main's path gives it). No fixed-end term is added at scale (A2.5) |
| Exact-profile wall membrane (`recover_wall_effective_membrane`) | PP `:3335-3368` | **unchanged**, from the published end actions. It is reached only for an exact-profile model with no pressure operand (OQ14) |
| Station and endpoint section resultants | `straight_section_resultants` (`:3427-3449`, `:3514-3533`) | **unchanged**, from the published actions; no loads are admitted |
| Stresses (endpoint and station), `STRESS_RECOVERY_LIMITED`, `open_formula_stress_summary`, `straight_summary_extrema`, exact-profile maxima, component stress multipliers, `max_stress` | PP `:3574-3860` | **unchanged** binary64 functions of the published values (OQ6) |
| R-b′ | PP `:3372-3386`, `:3862-3880` | unchanged; Sensitive (§3.3) |
| Constant-effort rows, curved and user rows, expansion-joint thrust rows | | none (excluded families) |
| Spring-hanger input echo, modulus-basis record | | unchanged (inputs) |
| Preview-physics rows (support attribution, certified maxima) | `preview_physics::render` | **unchanged**, from the case records built from the published values |
| Combinations | `append_combination_results` / `preview_physics::append_combination_results` | **unchanged**, from the published case rows. Combinations disable exact-block, not W2 |
| `linear_solver_mode_basis` | PP `:4494-4538` | published with the mode's primary `solution_basis`; every observation field `not_observed`; `dense_fallback=false`; mode code 1 or 2 (OQ5) |
| DEC-050/053 parity rows | PP `:2931-2946` | **not run** at b ≠ 0 (OQ5) |
| Integrity diagnostic | `append_integrity_report` | W2's unscaled report (`unscale_structural_solution`: records with outcomes; `scale_exponents` + b/2). K-D5's record is in displacement units. The `range_scaling:` line is appended (§4.1) |
| `LOAD_CONTRIBUTION_ABSORBED` | `:3008-3010` | from the unscaled report's `load_fidelity` (K2b recomputes the bits from the unscaled ledger) |
| Receipt, when the invocation selects another case | §2.4 step 11 | follows the published verdict, or fails closed |

- **Invocation level:** `require_finite_mechanics` over every row, as today. A non-finite derived row blocks the invocation with today's `SOLVER_SYSTEM_BLOCKED`. In a product run that is a finding to report, not a relabel (the brief's LEF-large note).
- The reactions check covers only the restrained DOFs, which are the only ones published. Main's `require_finite_mechanics(reactions)` also scanned the free-DOF residuals, which are not published.
- **RV11D2-N1 (informational):** the helpers form the scaled products (one rounding) and then unscale once. So a subnormal outcome carries two roundings, within its stated precision (2^-1075/|v| ≫ 2^-53).

---

## 4. The evidence line and the refusal template (Q6, Q7)

### 4.1 The `range_scaling:` evidence line (Q7), proposed exactly

```
range_scaling: force_scale_exponent=<b>; basis=exact power-of-two[; <entry>]{0..6}[; more=<k>]
<entry> :=
    record=<record>@<dof>:subnormal(relative_precision=<p>)
  | record=<record>@<dof>:underflow
  | record=<record>@<dof>:overflow
  | subnormal=reaction@<dof>:relative_precision=<p>
  | subnormal=spring_action@<dof>:relative_precision=<p>
  | subnormal=end_action@<member>.<i|j>:<Fx|Fy|Fz|Mx|My|Mz>:relative_precision=<p>
```

- `<b>` is the `i32` b ≠ 0.
- `<record>` is `RecordOutcome.record` verbatim (for example `intended_residual_rows.residual`).
- `<dof>` is `integrity_dof_label(model, dof)`, with its `global_dof=<i>` fallback.
- `<p>` is the f64 `{:?}`, as F1a's line uses it.
- `<member>` is the pipe's `element_id`. Local actions 0–5 are end `i`, and 6–11 are end `j`, in the listed component order.
- **Order ("DOF order", as ruled):**
  1. Every entry with a global DOF (records, reactions, spring actions), sorted by (global DOF, then record before reaction before spring action, then K2b's record order);
  2. then end actions by (pipe index, local index).
- **Length bound:** at most S11-G's `NAMED` (6) entries, then `; more=<total − 6>` when there are more. `NAMED` is private in `formation_guard.rs:58` (OQ8).
- **Placement:** appended to the integrity diagnostic's message after F1a's `formation_check:` line (or after S11-G's load-row sentence, or the base), separated by one space.
  - It goes through a new last parameter of `append_integrity_report`: `range_scaling: Option<&ForceScaledPublication>`. The rendering is `fn range_scaling_evidence_line(model: &PreviewModel, publication: &ForceScaledPublication) -> String`.
  - It is unconditional at b ≠ 0: it is method evidence, not a demotion.
  - R-b′'s later sentence (`amend_integrity_report`) follows it under S11-G's unchanged no-op rule.
  - The full composition: `<base>[ <S11-G load-row>][ <formation_check>] <range_scaling>[ <R-b′>]`.
  - At b = 0 the parameter is `None`, and A's oracle (a verbatim copy of main's `append_integrity_report`) proves byte identity.
- **Example** (the partial-underflow records, b = 898 at kernel level):
  `range_scaling: force_scale_exponent=898; basis=exact power-of-two; record=residual_rows.evaluation_allowance@N1:UY:subnormal(relative_precision=0.0111…); record=intended_residual_rows.residual@N1:UY:underflow; record=intended_residual_rows.evaluation_allowance@N1:UY:subnormal(relative_precision=0.0037…)`

### 4.2 The W2 refusal (Q6), proposed exactly

- **It is modelled on `append_integrity_failure` (PP `:1265-1285`):**
  - id `integrity_diagnostic_id(case_id)`;
  - severity `blocking`;
  - `affected_refs` `[case_id]`;
  - its message suffix is kept.
- **Code:** `NUMERICAL_INTEGRITY_UNRESOLVED`, except for `Failed { Structural(e) }` (OQ4).

```
Load case <id>: <reason>; range_scaling: attempted; step1_trigger=<trigger:?>[; force_scale_exponent=<b>; basis=exact power-of-two]; global_dof_map=<map:?>; no structural rejection is bypassed by generic LU or output quantization
```

| Failure | `<reason>` | b field |
|---|---|---|
| `Refused(r)`: steps 2–3 | `r`'s `Display` (`range: subnormal stiffness or load at formation`; `range: exponent span [<e_min>, <e_max>] exceeds …`) | `force_scale_exponent=none` (no b exists) |
| `Refused(r)`: step 4 | `range: scaled evaluation outside normal range` | c1: `<b>` from `census_force_scale`. c2: the field is omitted |
| `Failed { error }` (RV11-N2) | `error`'s `Display` | c1: `<b>`. c2: omitted. A census `Formation` error has no b: `none` |
| `NotAdmitted` | `range: family not admitted under force scaling: <family>` | `<b>` (the orchestrator's) |
| `Publication` | `range: publication outside binary64; at=<quantity>`, with `<quantity>` in §4.1's `kind@where` form | `<b>` |
| `NotEngaged` | `range: force scaling did not engage after a range trigger` | omitted |

- `<trigger:?>` is **PP's** step-1 `RangeTrigger`, for example `Formation(NumericalRange { name: "GJ/L: G*J" })` or `Evaluation(Range("arithmetic outside normal range"))`. It equals `r.trigger` wherever the orchestrator recorded one (pinned in D-CLASS).
- **PHYS-R4's existing assertion** `contains("Range(\"arithmetic outside normal range\")")` stays true, and so do its code, severity, affected-refs and count assertions. So `pressure_membrane_range.rs` needs **no edit**. The restatement is asserted in a new F1b test.
- **K2a's name survives** (ruling 7), inside the trigger's `Debug`. The K2a runtime file's linear variants assert the new template (approved Q10 edit). Their nonlinear `open_gap` variants are unchanged.
- **The b-rule is not callable as one public function.** The window formula is (`ForceScaleCensus::force_scale()`, FK), but the census scope is SA's private `force_scale_census` (SA:1583-1601). OQ1 carries my c1/c2 position.

---

## 5. Admission and publication refusals: where they go

Every W2 failure is published by `append_force_scaling_refusal(diagnostics, case_id, &failure, &trigger, model)`, in place of `append_integrity_failure`, and the case returns empty. The invocation then blocks (`has_blocking`), exactly as main blocks it. So the text reaches a receipt only in a captured invocation that main blocks entirely.

---

## 6. The resource guard (Q8)

### 6.1 The per-entry constant, counted in the code

The dense scrutiny path's peak is inside SA's DenseScrutiny branch (SA:663-683, and :784-799, :857-876 for the force-scaled siblings), in FK's `prepare_bound` → `audit_contributions`. Six n²-sized buffers are alive together (n = 6·nodes; n_f ≤ n):

| # | Buffer | Site | Bytes/entry |
|---|---|---|---:|
| 1 | dense K view `k.to_dense()` | SA:664 (`sparse.rs:299`) | 8 |
| 2 | symmetry allowances `dense_symmetry_view().0` | SA:575 | 8 |
| 3 | symmetry operation counts `dense_symmetry_view().1` (`usize`) | SA:576 | 8 |
| 4 | prepared matrix `a` (n_f²) | `structural.rs:1260` | 8 |
| 5 | contribution sums `vec![vec![Expansion::default(); n]; n]` (`Vec<f64>` 24 + `usize` 8) | `structural.rs:775` | 32 |
| 6 | contribution differences `sums.to_vec()` | `structural.rs:796` | 32 |
| | **total** | | **96** |

- **The later phases are smaller:**
  - the factor adds `l` (`structural.rs:1822`, 8) after 5 and 6 are freed;
  - `audit_intended_action`'s second sums array (`:890`, 32) gives 8+8+8+8+8+32 = 72.
- **PP's own dense-mode buffers** (dense view, dense reduction and `solve_dense`'s copy, FK:1641) come after the attempt: 24.
- **Not counted:** the row headers (O(n)); the `Expansion` heap terms of stored entries (O(nnz)); `validate`, rcond and the residual (O(n)); the formation check (O(nnz)); the negative-pair witness (O(n) at a time).
- **The dense `AssemblyEvidence` fallback** reaches the same 96: its constructor holds 4 arrays (32 B) transiently, and at the solve holds K 8 + evidence 16 + `a` 8 + 64 = 96.
- W2 in dense mode runs two such evaluations in sequence, never together.
- This agrees with §2.1's "at least about 100" (main also holds its per-basis dense K and dense reduced K, which F1b does not).

### 6.2 Formula, constants and ceiling

```rust
/// 96 = 8 (dense K view) + 8 + 8 (dense symmetry views) + 8 (prepared matrix)
///      + 32 + 32 (contribution sums and differences), the n^2 buffers alive at the
///      dense scrutiny path's peak (I13 checkpoint 0 §6.1). An estimate, not a measurement.
const DENSE_SCRUTINY_BYTES_PER_ENTRY: u128 = 96;
/// Provisional (ROOT Q8(a), 2026-09-28): the gate's 6 GiB heap cap on the owner's
/// Mac; revisited from K6's and V-P's measurements.
const DENSE_SCRUTINY_CEILING_BYTES: u128 = 6 * 1024 * 1024 * 1024; // 6_442_450_944
fn dense_scrutiny_estimate_bytes(dense_entries: u128) -> u128;      // 96 * dense_entries (saturating)
struct DenseScrutinyRefusal { dimension: usize, dense_entries: u128, estimated_bytes: u128, ceiling_bytes: u128 }
fn dense_scrutiny_guard(dimension: usize, ceiling_bytes: u128) -> Result<(), DenseScrutinyRefusal>; // refuses iff estimate > ceiling
fn dense_scrutiny_ceiling_bytes() -> u128;  // the constant; under cfg(test) a thread-local override (the lowered-ceiling test)
fn dense_scrutiny_refusal_diagnostic(refusal: &DenseScrutinyRefusal) -> Diagnostic;
```

- **The input:** `dense_entries = (dimension as u128)²` with dimension = `built.nodes.len() * DOF_PER_NODE`. That is `SparseStorageCounts::dense_entries` by definition (`sparse.rs:315`). A test pins equality with `storage_counts().dense_entries` on assembled models. Using the dimension lets the guard run when basis 0 is `RangeDeferred`, which has no `SparseStiffness` (OQ12).
- **The ceiling:**
  - dense_entries ≤ 6·2^30/96 = 2^26 = 67,108,864, so n ≤ 8,192;
  - nodes ≤ 1,365 (n = 8,190) in any topology, which is chain members ≤ 1,364;
  - a 1,000-member chain (n = 6,006) is 3,462,915,456 B (3.23 GiB), under the ceiling;
  - 1,365 members (n = 8,196) is 6,448,743,936 B, refused;
  - 10,000 members is 345,669,123,456 B, refused.
- **It applies to every DenseScrutiny invocation, linear or not.** The nonlinear loop's own dense base assembly (both modes; NI `lib.rs:581`) is not covered by this constant. It is disclosed as T5's M32 remainder (Q1(a)), and there is no sparse ceiling in F1b (Q8(a)).
- **The diagnostic:**
  - id `diagnostic:physics:dense-scrutiny-resource-guard`;
  - code `SOLVER_SYSTEM_BLOCKED`, `blocking`;
  - `affected_refs` `["model"]` (as `solver_blocked`);
  - message:
    ```
    dense scrutiny resource guard: estimated dense-path peak <estimated_bytes> bytes (96 bytes x <dense_entries> dense entries, <dimension> global DOFs squared) exceeds the provisional ceiling <ceiling_bytes> bytes; the model is refused before any n^2 allocation. The estimate is a stated formula, not a measurement; sparse_interactive does not use it, and no automatic dense fallback exists
    ```
- **Unchanged:** mode code 3 stays unused, `dense_fallback_message` stays `None`, and `dense_scrutiny_available=true` stays in the mode row.
- **The new refusal class** (dense models above about 1,365 nodes) is ROOT's provisional product decision, reported to the owner. On the gate corpus it touches only the 12 dense n10000 runs (C2).

---

## 7. The dense view for source recovery, and the scaled-evidence refusal (Q9; Scope 4)

- **The shared limit:** `pub(super) const DENSE_SOURCE_DOF_LIMIT: usize = 256;` in `source_recovery.rs` replaces the literal at `:466`, with no behaviour change. PP uses the same constant.
- **The dense view:** in `solve_load_case`, `let recovery_stiffness: Vec<Vec<f64>> = if source_eligible && n <= DENSE_SOURCE_DOF_LIMIT { k.to_dense() } else { Vec::new() };`. `Input.stiffness` keeps its type.
  - For n ≤ 256 the view equals main's dense K bit for bit (K1), so every exact-block attempt, selection, replay and receipt is main's.
  - For n > 256 the attempt reaches `charge(64)` and then the Budget refusal at `:466`, before the first read of `stiffness` (`:470-471`). So the charge and the bytes are main's.
  - `recovery_input()` is otherwise read only when a case was selected, and that requires n ≤ 256.
- **`source_receipt.rs` is unchanged** (its replays assemble their own n ≤ 256 dense K).
- **The scaled-evidence refusal (A2; OQ3):**
  ```rust
  /// F1b: the only product entry into retained-source recovery. An ordinary attempt
  /// formed at 2^b ≠ 1 is refused before any work is charged (D1 §4.7 with Q2(a):
  /// exact-block runs before W2, so this is a defensive guard).
  pub(super) fn solve_ordinary(input: Input<'_>, limits: exact::Limits, attempt_scale: ForceScale)
      -> Result<SelectedSourceRecovery, RecoveryFailure>;
  // attempt_scale != UNSCALED => Err(RecoveryFailure { stage: "source closure",
  //   helper_stage: AttemptStage::SourceClosure,
  //   error: RecoveryError::Unsupported("force-scaled ordinary evidence (b != 0) is not a retained-source input"),
  //   work: WorkReport { charged: 0, rejected: 0, limit: limits.operations } });  else solve(input, limits)
  ```
  - PP's single call passes `ForceScale::UNSCALED`, a constant.
  - `source_recovery::solve(` then has no non-test PP caller outside `source_recovery.rs`, which a lexer test pins.
  - t10b's anchor `source_recovery::solve(` becomes `source_recovery::solve_ordinary(`, a one-line amendment approved under Q11. Its ordering assertions are unchanged.
- **The formation-trigger decline (A2; OQ2, recommended option B):** `pub(super) fn range_formation_decline_without_attempt() -> RecoveryFailure`. It is zero work, with `Unsupported("the ordinary stiffness was not formed (range at formation); no retained-source attempt")`, and is modelled on `formation_decline_without_attempt()` (`:77`). It is reported as `SOURCE_BLOCK_RECOVERY_UNAVAILABLE`, as usual. `source_budget.attempts` is not incremented.

---

## 8. The pin and site-test edits (Q11)

### 8.1 PP `tests/s11f_site_test.rs`

| Change | Site removed or added | When |
|---|---|---|
| `run_linear_static_preview_captured_once` 4 → 2 | the two spring-diagonal `+=` (PP `:1836`, `:1970`) | A1 |
| row `("PP/lib.rs", "multiply_matrix_vector", 1, …)` removed | the function is deleted (no caller) | A1 |
| `assemble_case_stiffness` 1 and `add_curved_bend_stiffness_contributions` 1 | **unchanged** (they keep `source_receipt.rs:212`, `:315`; §0) | — |
| `legacy_observation_force` 1 | unchanged count (`*value -= stiffness.get(row, column) * displacement`) | A1 |
| `restrained_reactions` 0 (E12) | unchanged row; the body delegates to `SparseStiffness::reactions` | A1 |
| new rows | none expected: the new functions have no `+=`, `-=`, `.sum(` or `fold(` (formatted with `format!`/`join`). Any accumulation added later is an unlisted row, and fails | A1/A2 |
| `t10b` | the anchor `source_recovery::solve(` → `source_recovery::solve_ordinary(` | A2 |
| a new lexer assertion (in t10b, or `f1b_*` tests) | `source_recovery::solve(` has no non-test caller in PP outside `source_recovery.rs` | A2 |

- **No count rises.** Rules 1–7 and t8 are untouched. There is no forbidden call, and `solve_dense(` and `solve_symmetric_system_from_entries(` stay in the allow-listed observation lanes.
- **The binary64-fold mutant** in each new PP function is killed by rule 8, as an unlisted accumulation. It is run per new function at C.

### 8.2 NI `s11k_tests.rs` (tests only)

- **The K-D5 pin (`:1116`): no edit is needed.** PP still calls `.solve_assembled_with_formation_check(` exactly once, inside `solve_preview_reduced_system` (now the pattern sibling). W2 reaches the formation check only inside SA's orchestrator, and PP names none of the other `FORMATION_ENTRY_POINTS`. OQ10 asks ROOT to confirm leaving it byte-unchanged.
- **The K2b pin's product half (`:1620-1630`)**, "no PP mention before F1b", becomes **"exactly the declared PP sites"**:
  - For each PP non-test module and each `FORCE_SCALED_ENTRY_POINTS` token, the occurrences (with `token_indices`' prefix boundary: `ForceScale` also matches `ForceScaleCensus`, `ForceScaleReason`, `ForceScaledError` and `ForceScaledSolution`) are grouped by their enclosing top-level item: a `fn`, `struct`, `enum` or the `use` block.
  - Each (module, item, token) count must equal a declared table exactly.
  - The planned declaration: in `lib.rs`, the `use` block; `force_scaling_attempt` (`ForceScalingCase`, `solve_with_force_scaling`); `census_force_scale` (`ForceScalingCase`, `ForceScaleCensus`); `force_scaled_publication` (`force_scaled_reactions`, `force_scaled_end_actions`, `force_scaled_spring_action`, `ForceScale…`); `ForceScalingFailure`; `append_force_scaling_refusal` (`ForceScaleReason`/`ForceScaledError` renderings). In `source_recovery.rs`, `use` and `solve_ordinary` (`ForceScale`).
  - `force_scaled(`, `new_force_scaled`, `solve_force_scaled` and `with_force_scale` stay at zero in PP.
  - The exact table is fixed at A2 from the code, and it is F2a's to extend.
- **The loop halves are unchanged.**
- **The NI pin mutants** (a second PP orchestrator call; a PP scaled helper named outside its declared item; the loop reaching a scaled or pattern entry) must be killed there (F).

### 8.3 The existing unit tests: call-site updates only

- `s11f_tests.rs`: `:1910`, `:1919`; `assemble_case_stiffness` at `:820`, `:1892` is unchanged.
- `f1a_tests.rs:406`, `:641`, `:651`: `None` for the new parameter.
- `s11g_tests.rs`: no change is expected (it calls `source_recovery::solve`, which is kept).
- `source_receipt/load_state_join_tests.rs:722-723` and `source_receipt/load_state_tests.rs:95`/`:501`: `solve_load_case` takes `assemble_basis_stiffness(..)` instead of `assemble_case_stiffness(..)` (OQ9).
- `source_recovery.rs` tests: none.
- **Any assertion change stops the work.**

---

## 9. F1a's follow-ups

- **N1:** a one-line comment at PP `:1118`: "`row=none` is unreachable: K-D5 sets `global_dof` for every `Estimate` record; kept as the formatter's total fallback".
- **N2:** disclosed in CHANGE_RECORD. `source_receipt.rs:914-921` reserves 12× the bytes of every envelope diagnostic. F1b's new text reaches a receipt only in a captured invocation that selects another case and publishes a W2 case, where the integrity message grows by the `range_scaling:` line (at most about 600 bytes with `NAMED` = 6). W2 refusals always block, so they never reach one. The guard refusal is a blocked envelope, with no receipt.

---

## 10. The derivations (outlined; written in full in RETURN, each independently checkable)

- **D1: PP's range classification equals the orchestrator's step 1.**
  - (a) PP's ordinary attempt is the orchestrator's b = 0 evaluation, operation for operation. The orchestrator forms `assemble_sparse_stiffness(…, SparseAssemblyOptions::new().with_force_scale(UNSCALED))`, which is `new()` (`sparse.rs:601-612`), on the same frames, users, blocks and springs. `SparseAssemblyEvidence::new_force_scaled(UNSCALED)` is `new` (SA:519-521). `solve_force_scaled_with_formation_check` at `UNSCALED` takes `f` unscaled (SA:838-845). It runs `check_pattern`, `geometry` and the same formation source (SA:1446-1448) and the same solve. `force_scaled_outcome` at `UNSCALED` is the identity (`structural.rs:2514-2522`, `:2593-2595`). PP's formation step (`assemble_basis_stiffness`) is the same assembly call.
  - (b) SA's classification arms: `:1627-1632` and `:1688-1690` equal PP's two arms. `:1691-1696` is unreachable at b = 0, by (a).
  - (c) PP's other failures before the attempt are non-range `FrameKernelError`s (finiteness, ledger, reduction). They are raised identically at b = 0 (the orchestrator would raise the same error, or never see the case), and they block today.
- **D2: exact-block cannot select a formation-range case.**
  - `prepare_sources` (`source_recovery.rs:879`) calls `AssemblyEvidence::new(built.nodes.len(), &built.frame_elements, …)`.
  - `EvidenceParts::new` calls each frame's `local_stiffness()` (SA:1071-1073). The error maps to `InvalidInput("frame local stiffness")`, and then to `RecoveryError::Exact(Arithmetic)`.
  - K2a's `NumericalRange` at the basis assembly can only come from a frame, because users and blocks are not K2a-checked and blocks arrive formed. It is the same deterministic `local_stiffness` of the same `built.frame_elements`.
  - `:879` is on every success path (`solve` → `prepare_sources`). So no formation-range case is ever selected.
  - Under OQ2's option B the attempt is replaced by a zero-work decline, and D2 shows that nothing is lost.
- **D3: the scaled-evidence refusal is unreachable at product level.**
  - `solve_ordinary` has one product caller, inside exact-block's branch of `solve_load_case`, with the constant `ForceScale::UNSCALED` (lexer pin).
  - Exact-block's branch precedes the outcome match in which W2 runs (Q2(a)), and no `ForceScalingOutcome` exists before it.
- **D4: byte identity by construction wherever main publishes.**
  - The only changed control-flow arm is `Err(failure)` with a range trigger on a linear invocation and no selection. There, main pushes `append_integrity_failure` (blocking), and the invocation publishes nothing.
  - The only deferred error is K2a's `NumericalRange` on a linear invocation. There, main returns `solver_blocked` at the assembly.
  - Every other arm runs main's code on bit-identical inputs: K1 parity of K, partition, RHS, reactions and `StructuralSolution`, and Q9's view. `OrdinaryAttempt`'s arguments are main's (§2.4 step 11).
  - So every invocation main publishes, on either entry and in either mode, receipts included, is unchanged.
  - **Exclusion:** the guard's new refusal class (DenseScrutiny above 6 GiB estimated), which on the gate corpus is only the n10000 runs.
  - This is checked independently at the gate (C3) and by T9.
- **D5: the guard's constant** (§6.1), and **its placement** before every n² allocation: the per-basis assembly is O(nnz); the case's first n² allocation is SA's; Q9's view is ≤ 256² entries.
- **D6: the partition equality for `RangeDeferred`** (the complement equals `reduce_assembled_sparse_system`'s `free_dofs`, `sparse.rs:798`), and **the finiteness scan's first-entry equality** (§2.4 step 2).
- **D7: admission soundness** (§3.2). At b ≠ 0 every published force-unit value is either:
  - a K2b checked helper's `PublishedValue`; or
  - an unchanged binary64 function of published values, the same function main applies at b = 0.

  Every load term is an exact input: a normal authored value, with a zero term refused.
- **D8: LEF-large's product b and its power-of-two relation.** The experiment gives b = −702, with the census span [336, 1124] set by e(G) = 336 and (12·E)·I = 1124, for all three cases. Everything below these results depends on that span:
  - Loads and springs lie strictly inside it, so b does not depend on them.
  - Binary64 arithmetic commutes with exact powers of two when no range event occurs. So every published displacement component is the base's times 2^(pf−pm−pl) (translations) or 2^(pf−pm−2pl) (rotations), and forces are 2^pf and moments 2^(pf+pl) times the base's, bit for bit. Otherwise a helper refuses by name.
  - The magnitude rows (`powi`/`sqrt`, `hypot`) go through libm and are compared only within 1e-9 (the brief's rule).
- **D9: mutant equivalences,** if any survive: derived, never assumed.

---

## 11. Tests

The predicate everywhere is the unchanged `|obs − exp| ≤ 1e-9·max(|exp|, scale)`. There is no powi, powf, exp, ln, trigonometric function, `hypot` or `cbrt` on constants in expected values. Exact references come from `Fraction`, K2b's generated bits, or R1's references.

- **A:**
  - T9 (Mac-only, from `git archive` copies of base and candidate), expected 112/112;
  - the 39 manifests against `K2B_MERGE/dec025/suites.log` (only the three known Mac failures, with identical blocks);
  - K1, K2b, K-D5, S11-F, S11-G and F1a tests unchanged, except Q10 and Q11;
  - **A-ORACLE** (`f1b_tests.rs`): `append_integrity_report(…, None)` equals a verbatim copy of main's on the F1a cases (the `f1a_no_record_is_byte_identical_to_main` pattern);
  - the gate (§13).
- **B:**
  - **B1–B3** at unit level (`f1b_tests.rs`, private functions). On a declared subset (K-D5's curved models; a user element; two modulus bases; a 0.4.0 prescribed case), `assemble_basis_stiffness` equals `assemble_case_stiffness` entry for entry, bit for bit (stored entries, and +0.0 elsewhere). `reduce_assembled_sparse_system` equals `reduce_assembled_system*` (free DOFs and force bits). `restrained_reactions` (sparse) equals a verbatim copy of main's dense E12, including the sign of a zero.
  - **B1–B3 corpus-wide:** a scratch observation build (§13.3.1) over every T9 request and every gate request of at most 1,000 members.
  - **B4:** outcome-class parity between modes (N01–N09, R01–R07, NP-B, NP-D, the T0R references, the R1 families; both entries), candidate against Mac main, from T9 and the gate. The parity rows must be byte-identical.
  - **B5:** a two-modulus-basis invocation, each case on its own values (mutation 11's control).
  - **B6:** relabelling and permutation. The answers agree within 1e-9 across numberings (PP test), and each numbering is byte-identical to Mac main (scratch run on both trees).
  - **B7:** gap, one-way and friction models byte-identical to Mac main (T9, gate).
- **C:**
  - **C-DECISION** (unit): `dense_scrutiny_guard` at dense_entries 2^26 − 1 (below), 2^26 (exactly 6 GiB: allowed) and 2^26 + 1 (refused, with estimate 6,442,451,040), and at dimension 60,006 (no overflow); `storage_counts().dense_entries == dimension²` on assembled models.
  - **C-LOWERED** (unit, cfg(test) hook): a 2-node model (n = 12, 144 entries) with the ceiling lowered to 96·143 B. It gives `SOLVER_SYSTEM_BLOCKED` with the estimated bytes 13,824 and no mechanics rows, on both entries. At 96·144 it solves. This kills F1B-M9 (a removed call, or an inverted comparison).
  - **C-SPARSE** (own binary `tests/f1b_sparse_pattern_memory.rs`, counting global allocator with current, peak and cap):
    - an axis-aligned invented chain of **1,000 members** (n = 6,006; the RF-LARGE-CHAIN-n01000-AX shape), sparse mode, the typed and then the captured entry, in one `#[test]` with the peak reset between them;
    - asserts the peak heap increment over the call is **< 64 MiB**. The dense view alone is 8·6,006² = 288,576,288 B = 4.30 × 64 MiB;
    - the **cap is 2 GiB**. A dense-view regression (F1B-M10: 289 MB, or 577 MB with a dense reduction) reaches the assertion (a counted kill). A full dense-path regression (3.46 GB) aborts at the cap, safely (not a counted kill);
    - **contingency:** if A1's measured peak exceeds 64 MiB, I report it and propose 2,000 members (n = 12,006; dense view 1.153 GB), a 256 MiB bound and a 4 GiB cap. I change nothing without ROOT.
  - **C-CEILING** (own binary `tests/f1b_guard_ceiling.rs`, capped counting allocator):
    - a chain of **1,365 members** (n = 8,196; estimate 6,448,743,936 B > 6 GiB), DenseScrutiny, both entries;
    - the refusal with that number, and no rows;
    - peak heap increment **< 64 MiB** (under 1.1 % of the estimate);
    - **cap 512 MiB**: a removed guard would reach the 537 MB dense view and abort safely.
    - It never reaches 10,000 members.
  - **C-NOFALLBACK:** a sparse integrity refusal (N02-class mechanism; N06 unresolved) publishes its refusal with no dense-mode row. Across every F1b test envelope, no `linear_solver_mode_basis` has value 3, and `dense_fallback=true` never appears.
  - **The debug wall time** of each C test is recorded. The 1,000-member sparse product solve at opt-level 0 may approach or exceed a minute. If it does, I report it and do not `#[ignore]` it.
- **D (W2)** (`f1b_tests.rs` and `tests/f1b_w2_runtime.rs`; both entries where reachable, both modes):
  - **Every Scope §6 row,** as required or as ruled at A2.
  - **D-CLASS** (§3.1).
  - **Engagement:** a nonlinear (`open_gap`) variant never engages, and is byte-identical to Mac main (the scratch run on both trees). F1B-M1 is killed there.
  - **Coexistence (Q2):** the construction found by A2's search (§13.2). It is selected on Mac main, and the candidate's envelope and receipt are byte-identical. If none exists, I derive why and report it.
  - **The mixed captured invocation:** its outcome (finalizes, or fails closed with `ordinary outcome changed`) is pinned.
  - **Admission:** each of the 9 checks is a named refusal. At least the six named families (thermal, uniform, exact-pressure operand, constant effort, curved bend, user element) are constructed as range cases, and their b = 0 twins are unchanged.
  - **Publication analogues at product level:** F-A2 (reactions and actions refused, never a wrong Normal); F-S (spring action refused); an in-range reaction equal to 2^-b times the scaled exact sum rounded once; a `Subnormal` value with its precision in the line; the record outcomes rendered (partial underflow); displacements never scaled.
  - **RV11-N4:** the pin assertion, plus the value test.
  - **The evidence line:** the exact template; absent at b = 0; exactly one per case at b ≠ 0; composition with S11-G and F1a, and R-b′ after it; the no-op rule; `NAMED` and `more=` (a renderer test with 8 synthetic outcomes).
  - **Refusals:** the exact template for every reachable `ForceScaleReason`, admission family and publication quantity, each with the step-1 trigger. Spring-carried keeps `GJ/L: G*J`. The non-range failure at the chosen b carries the trigger and b (OQ1, OQ4).
  - **A subnormal derived section value on the range path** is refused by name (`SubnormalAtFormation`); the same value at b = 0 is unchanged.
  - **The limitation chain** (product analogue) is refused by name, b = 312 under c1, and not fixed.
  - **The `k2a_formation_range_runtime.rs` linear variants** are restated (Q10). The nonlinear variants are untouched.
- **E:**
  - An `Input` with `attempt_scale ≠ UNSCALED` gives `Unsupported`, named, with zero work charged (kills F1B-M11).
  - With n = 258 (43 nodes) and an empty `stiffness`, the Budget refusal carries the same `RecoveryFailure` (stage, error, work) as with a full dense K. This is the order pin.
  - A captured Sensitive case with n > 256 gets main's `Budget` refusal byte for byte, and one with n ≤ 256 gets main's recovery byte for byte (the scratch run on both trees).
  - Option B's decline, if ruled: zero work, named, reported.
- **F:**
  - The K2b pin's new product half, and its three new mutants.
  - The K-D5 pin unchanged.
  - The original pins' mutants (K1's and K2b's tables) re-run and killed at the same sites.
  - PP's site test: the reduced counts, the removed row, and `t10b`.
- **G:** the gate (§13).

---

## 12. Mutants (C; clean `git archive` copy and target per mutant under `<wt>/f1b-mut/<m>/`, NONE first, at most 3 at `-j 4`)

| # | Mutant | Intended kill |
|---|---|---|
| 8 | a spring omitted from the pattern assembly, in sparse mode only | B1 (unit); B4; T9 |
| 9 | W2 publishes without unscaling by 2^-b | D (reach and LEF-large values; the powers of two) |
| 10 | the assembly order depends on labels | B6 byte identity |
| 11 | one modulus basis reused for every case | B5 |
| — | b ≠ 0 leaking into b = 0: the line at b = 0, or W2 without a range trigger | A-ORACLE; T9; D |
| F1B-M1 | W2 engaged on a nonlinear invocation | D (`open_gap` byte identity) |
| F1B-M2 | W2 before exact-block | D (coexistence) |
| F1B-M3 | the line omitted at b ≠ 0 | D (line tests) |
| F1B-M4 | a `Subnormal` rendered without its precision | D (subnormal reaction or action) |
| F1B-M5 | record outcomes not rendered | D (partial underflow; renderer test) |
| F1B-M6 | reactions at b ≠ 0 through `SparseStiffness::reactions` on K′ | D (F-A2 analogue) |
| F1B-M7 | member actions at b ≠ 0 through SP's b = 0 recovery | D (reach: `ELEMENT_FORCE_RECOVERY_FAILED` instead of published) |
| F1B-M8 | spring action at b ≠ 0 as unchecked `-k·u` | D (F-S analogue) |
| F1B-M9 | the guard call removed, or `>` inverted | C-LOWERED; C-DECISION |
| F1B-M10 | sparse mode materializes the dense view | C-SPARSE (assertion; the cap only as a safety net) |
| F1B-M11 | the scaled-evidence refusal removed | E (unit) |
| F1B-M12 | admission dropped | D (admission) |
| F1B-M13 | a pre-scaled ledger passed to the orchestrator | F (pin: `force_scaled(` in PP); D (values) |
| F1B-M14 | the step-1 trigger dropped from a refusal | D (spring-carried `GJ/L: G*J`) |
| F1B-M15 | per-basis `NumericalRange` still blocks linear invocations | D (reach) |
| F1B-M16 | the classifier admits a non-range error | D-CLASS |
| F1B-M17 (own) | `DENSE_SCRUTINY_BYTES_PER_ENTRY` 96 → 32 | C-DECISION (the estimate value and the 2^26 + 1 refusal) |
| F1B-M18 (own) | `>` → `>=` in the guard (refuses exactly at the ceiling) | C-DECISION ("at" case); C-LOWERED (96·144 solves) |
| F1B-M19 (own) | admission check 9 (zero nodal term) removed | D (a W2 case with an authored 0 N nodal load: refused) |
| F1B-M20 (own) | the `NAMED` limit ignored in the line | D (renderer test, 8 outcomes) |
| F1B-M21 (own) | `OrdinaryAttempt` formed from the b = 0 attempt for a W2-published case | D (mixed invocation's pinned outcome) |
| — | NI pin mutants: a second PP orchestrator call; a scaled helper named outside its declared PP item; the loop reaching a scaled or pattern entry | F |
| — | a binary64 `+=` fold in each new PP function | PP rule 8 |
| — | K1's and K2b's original pin mutants | the same sites as their tables |

A stack-overflow or allocator abort is never counted as a kill. A survivor is reported, never hidden.

---

## 13. Product runs planned for A2, the RV11D-N2 method (Q4), and the gate

### 13.1 Tools

- **Builds:** `git archive` trees of the candidate and of Mac main `e7d930d49`, under `<wt>/scratch/i13/`, with their own targets (deleted afterwards). At most two cargo jobs, `-j 8`, `RUST_TEST_THREADS=4`.
- **(a) The P1 probe** with the 6 GiB heap cap, built from the candidate tree (G1 already holds Mac main's binary and `runs.jsonl`). It is driven by P1's `run_one`.
- **(b) A scratch harness crate** depending on PP by path into each tree. It sends the reach, PHYS-R4, limitation-chain, coexistence, mixed and admission requests through both public entries and both modes. It writes JSONL rows: case, entry, mode, envelope sha256, outcome, integrity code, the parsed b, the refusal reason and trigger, and published components with their bits.
- **(c) A scratch observation build:** a third `git archive` copy of the candidate with an instrumentation module appended to PP. It is never committed, and it is not product code. It is used for B1–B3 corpus-wide, RV11D-N2 and the forced-even-b probe.

### 13.2 The product-run table (A2)

The columns are: case, entry, mode, main's outcome (G1's `runs.jsonl`, or a harness run on main), the candidate's outcome, b, standing (integrity code), and the 1e-9 comparison against the reference.

1. **All 32 RF-RANGE cases × 2 entries × 2 modes (128 runs).**
   - G1's Mac base: 62 `refused_blocked` over 20 cases and 38 `refused_capture` (`exp/rf_range_base.out`).
   - **Proposed C1 list, from Mac data** (28 runs; fixed at A2 by the product runs):
     - 22 `NUMERICAL_INTEGRITY_UNRESOLVED` Range runs:
       - CHAIN-E-1000 ×4;
       - CONT-E-1000 ×4;
       - SKEW-E-1000 ×4;
       - SKEW-F-960 ×4;
       - THIN-B ×4;
       - SKEW-E+960 typed ×2;
     - 6 K2a `SOLVER_SYSTEM_BLOCKED` runs: {CHAIN, SKEW, CONT}-LEF-large typed ×2.
   - **Not C1:**
     - SKEW-F+960 and CONT-F+960 typed (4 runs; `computed mechanics must be finite, got inf`: a non-range `NonFiniteInput`, which W2 never engages) are unchanged;
     - the 30 `PIPE_ELEMENT_INPUT_INVALID` runs (LEF-small, L−240, SIM-a, …) are unchanged;
     - the 38 capture refusals are unchanged.
2. **LEF-large typed:**
   - b = −702 (D8);
   - each displacement component compared bit for bit against its RF base case's (RF-CHAIN-T-n05-r1e-08 and siblings, from G1's base envelopes) times the stated power of two;
   - forces ×2^600 and moments ×2^800, or refused by name;
   - standing Sensitive (Q5(a)).
3. **The reach set** through the harness, candidate and main, both entries and both modes:
   - reach_zero (`EXACT_ZERO`), reach_lef (`LEAST_SUBNORMAL`) and partial underflow: published within 1e-9 of K2b's exact references;
   - spring-carried: `ScaledEvaluation` with `GJ/L: G*J`, b = 548 under c1;
   - the `open_gap` variants: byte-identical to main.
   - PP's partial-underflow model is compared with K2b's kernel case (load 1e-307 N, b = 898) and I state which differs.
4. **PHYS-R4:**
   - With pressure (fixed ends; typed and captured): predicted `range: subnormal stiffness or load at formation`, trigger `Evaluation(Range("arithmetic outside normal range"))`. The run must show the census refusal, which confirms that the end-cap operand is subnormal (Q10's condition).
   - Without pressure (a cantilever, `pressurized = false`, tip Fy = tip Mx = f64 bits `0x0031fa182c40c60d` ≈ 1e-307): predicted published, b = 536, within 1e-9 of K2b's exact values for UY, RZ and RX (UX = 0; K2b's 1e-156 axial load is not in the product fixture helper), standing per Q5.
5. **The limitation chain:**
   - a product analogue of K2b's E = 2^440 two-bar chain (sections from OD and wall; loads 2^-1010 N and 1 N), typed;
   - predicted `ScaledEvaluation`, with b stated;
   - captured refuses 2^440 at capture;
   - the forced-even-b probe shows the in-window b that solves.
6. **The coexistence search on Mac main first** (captured, axis-aligned cantilevers in exact-block scope): subnormal nodal loads (1e-310 N), loads near 2^-1022 with stiff springs, and a two-basis variant. The first construction that main's exact-block selects after a range trigger becomes D's coexistence test.
7. **The mixed captured invocation:** case A (selected by exact-block) plus case B (W2; its own formation-range modulus basis). Its outcome is pinned.
8. **Admission:** the six families on range models, and their b = 0 twins.
9. **The F-A2 and F-S product analogues.**
10. **Every `ScaledEvaluation` refusal** found in 1–9 and on the gate gets the forced-even-b probe (Q12): every even b in the window, through the public `assemble_sparse_stiffness(…with_force_scale)`, `SparseAssemblyEvidence::new_force_scaled` and `solve_force_scaled_with_formation_check`, in the observation build. The probe reports which b solve, and within 1e-9.

### 13.3 RV11D-N2's availability cost (Q4), proposed method

- **The instrumentation:** in the scratch observation build (13.1(c)), after each case publishes at b = 0 on the ordinary route (a `Formed` linear case with `linear_solve` `Some` and no selected source), the module evaluates, on the case's own `SparseStiffness`, unscaled ledger, published u, frames and springs:
  - `force_scaled_reactions(&u, &force, UNSCALED, restrained)`;
  - `force_scaled_end_actions(&u, UNSCALED)` per straight pipe that is not curved and not recovered;
  - `force_scaled_spring_action((dof, k), &u, UNSCALED)` per spring.
- For each quantity it logs **refused** or **published**. For published ones it checks bit equality with the value the product published (E12, SP's `local_forces`, `-k·u`). K2b claims equality wherever they publish, so a mismatch is a finding.
- **Corpus:** every gate request, typed entry (the captured entry's ordinary u is the same), both modes. The n10000 runs are sparse only (dense is guard-refused), and the four dense 1,000-member timeouts are excluded. T9's 112 committed fixtures are added. At most 4 workers, and the 10,000-member runs one or two at a time.
- **Reported:** the runs with at least one refusal, the refused quantities by kind and family, the reason (a subnormal product, or a partial sum), and 0 bit mismatches expected. It is an observation, not a claim, and it is not product code.

### 13.3.1 B1–B3 corpus-wide

- In the same observation build, for requests of ≤ 1,000 members only, each basis's `assemble_basis_stiffness` is compared with main's dense assembly (`assemble_case_stiffness`, still present) entry for entry. The partition and RHS, and the reactions (against a verbatim copy of main's dense E12), are compared too.
- The dense side is built only at these sizes, one run at a time for the 1,000-member ones.

### 13.4 The gate (B)

As the brief states:
- K-D5's two-part method on this Mac. The P1 probe plus the heap cap is built twice from `git archive` trees, and `gen.py`, `run.py`, `compare.py` and `gate_check.py` are unchanged. The inputs are hash-checked.
- **Part 1:** 884 runs of the candidate against G1's base (`<wt>/scratch/gate_base_e7d930d49/`, PASS, 764 gate rows, 0 breaches). The 12 sparse n10000 runs are run one or two at a time.
- **Part 2:** the four dense 1,000-member timeouts, interleaved base, candidate, base, candidate, at 1800 s, on a quiet host. ROOT serializes this against K4.
- **PASS requires** 0 trusted breaches; C3 byte identity; C1 ⊆ the ruled list; no C2 sparse heap-cap abort.
- `<wt>/guard/memguard.log` is checked after every heavy phase.

---

## 14. Positions on the open questions (each with a recommendation)

- **OQ1 (Q6 c1/c2).**
  - Fact: the b-rule's formula is public (`ForceScaleCensus::force_scale()`), but its census scope is SA's private `force_scale_census` (SA:1583-1601). No single public function yields b.
  - **Recommendation: c1-composed.** PP runs FK's public census methods over the `ForceScalingCase`'s public fields in SA's order. That is five loops, and no formula. It is pinned equal to the orchestrator's b on every D success path and to 312 on the limitation chain.
  - If ROOT reads the census scope as part of the "formula", use c2: b only where the outcome carries it, and "range_scaling: attempted" otherwise. Both templates are specified in §4.2.
- **OQ2 (exact-block for a formation trigger).**
  - There is no assembled K, so "run the attempt anyway" has no truthful stiffness input. An empty slice gives `SourceMismatch("actual model/source dimensions")` for n ≤ 256, a misleading reason (option A). A placeholder n×n zero K reaches `:879` and fails there truthfully, charging the precharge (option C, the brief's letter).
  - **Recommendation: B.** A zero-work named decline, `range_formation_decline_without_attempt()`, with the precedent of S11-G's D22-1. D2 shows nothing is lost.
- **OQ3 (encoding the scaled-evidence refusal).**
  - **Recommendation:** the wrapper `solve_ordinary(input, limits, attempt_scale)` in `source_recovery.rs`, a one-line t10b amendment and a lexer pin.
  - Alternative: a new parameter on `solve`, which keeps t10b but adds `UNSCALED` at about 10 test call sites, 8 of them in `src/source_receipt/*_tests.rs`.
- **OQ4 (the code of a non-range failure at the chosen b, RV11-N2).**
  - **Recommendation:** `Structural(e)` gets `append_integrity_failure`'s mapping of the unscaled error (for example, a Mechanism stays `NUMERICAL_INTEGRITY_PHYSICAL_MECHANISM`), under the W2 template with the trigger and b. That is K2b's "today's outcomes". `Formation(e)` and a census error are `NUMERICAL_INTEGRITY_UNRESOLVED` per case.
  - Alternative: every W2 failure `NUMERICAL_INTEGRITY_UNRESOLVED`.
- **OQ5 (the DEC-050/053 lanes at b ≠ 0).**
  - **Recommendation:** not run. They observe the unscaled binary64 system that left the range, and they never select. The mode row is published with every observation field `not_observed`.
  - Alternative: run the sparse-entry lane as today; it fails formation for formation triggers anyway.
- **OQ6 (derived rows at b ≠ 0: stations, stresses, magnitudes, preview, combinations).**
  - **Recommendation:** unchanged binary64 from the published values, as main computes them for every case. A non-finite result keeps today's refusal, and the limitation is disclosed (§4.7 step 5 covers actions, reactions and records).
  - Alternative: range-check each derived row at b ≠ 0 and refuse if it is subnormal. That is stricter than b = 0, and it is C1-only.
- **OQ7 (the admission order): after the orchestrator.** This is needed for §6's PHYS-R4 reason and for D's "at b ≠ 0". **Confirm.**
- **OQ8 (`NAMED` is private in `formation_guard.rs`,** which is outside the write set).
  - **Recommendation:** approve a one-token visibility change (`const NAMED` → `pub(crate) const NAMED`), declared.
  - Alternative: a PP constant of 6 plus a source-text pin against `formation_guard.rs:58`.
  - Without one of these, the line cannot share S11-G's constant.
- **OQ9:** confirm that call-site-only updates in `src/source_receipt/load_state_join_tests.rs` and `load_state_tests.rs` (tests calling `solve_load_case`) are within the "existing PP unit tests" row. `source_receipt.rs` itself is unchanged.
- **OQ10:** leave NI's K-D5 pin byte-unchanged (§8.2). **Recommendation: yes.**
- **OQ11:** the refined site-table expectations of §0 and §8.1. **Recommendation: approve.**
- **OQ12:** the guard input is dimension², pinned equal to `storage_counts().dense_entries`, evaluated after basis 0's outcome, for every DenseScrutiny invocation. **Recommendation: approve.**
- **OQ13:** admission check 9 (an exactly-zero nodal term is refused at b ≠ 0), because unit-normalized and 0.4.0-factored magnitudes are formed at b = 0. **Recommendation: approve.** It regresses nothing, because main refuses every such case.
- **OQ14:** exact-profile models without pressure operands are admitted, so that PHYS-R4 without pressure solves. **Recommendation: approve.**
- **OQ15:** the receipt follows W2's verdict (§2.4 step 11), and it fails closed when R-b′ demotes after. **Recommendation: approve**, and pin whatever the mixed run shows.

## 15. Stop items

- **None blocks A1.** No kernel edit is needed: K1 and K2b supply every entry.
- **Conditional:**
  - OQ8's shared-constant option needs a one-token edit in `formation_guard.rs`, which is outside the declared write set. I will not make it without ROOT's approval.
  - OQ9's test files are under `src/source_receipt/`. I will not touch them without confirmation.
  - If ROOT rules OQ2 option C (the brief's letter), it needs no new write set.
- **Stops declared in advance for A1 and A2:** any T9 or gate byte difference outside C1 and C2; a Scope §6 prediction contradicted (above all PHYS-R4's operand subnormality, LEF-large's b = −702 and power-of-two relation, and spring-carried's `ScaledEvaluation`); a C2 sparse heap-cap abort; a memory-guard SIGKILL.

## 16. Host

- `RUSTUP_TOOLCHAIN=1.97.1`, `RUSTUP_AUTO_INSTALL=0`, `CARGO_INCREMENTAL=0`, `--offline --locked`, `-j 8`, `RUST_TEST_THREADS=4`, and at most two cargo jobs of mine at a time.
- Target `<wt>/f1b-target`; scratch trees and targets under `<wt>/scratch/i13/`, deleted afterwards.
- No dense matrix is formed at ≥ 10,000 members. Dense parity stops at 1,000 members. The C-SPARSE and C-CEILING binaries carry caps.
- I check `<wt>/guard/memguard.log` after every heavy phase. K4 (I12) builds in `<wt>/k4`, and the write sets are disjoint.
- **Platform:** `aarch64-apple-darwin`, rustc 1.97.1. T9 and the gate are Mac-only comparisons against Mac main.
