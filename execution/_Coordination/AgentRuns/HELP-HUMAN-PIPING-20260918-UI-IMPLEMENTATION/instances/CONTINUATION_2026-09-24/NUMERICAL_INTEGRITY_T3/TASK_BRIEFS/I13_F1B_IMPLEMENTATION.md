# I13: implement facade slice F1b (the product's sparse wiring, the dense-scrutiny guard, and W2 at formation in the product)

This is an implementation TASK. Read Root `AGENTS.md`, `agents/AGENT_TASK.md` and `_COMMON.md` first. The Mac host rules in `I8R_K1_RESUME.md` ("The Mac host" and "Platform calibration") override `_COMMON.md`'s host section, and apply to you in full, with F1b's paths below in place of K1's. F1b is "the F1b part" of `TASK_BRIEFS/I7_F1_IMPLEMENTATION.md` (ROOT's "F1 split"). This brief replaces that part, and restates its LEF expectation (I7 addendum 1).

## Roles

- ROOT (HELP_HUMAN) dispatches you directly, as a background subagent, and is your return path. There is no separate T3 manager on the Mac.
- Make no Git writes and no index operations. ROOT commits.
- Record the delegation mechanism in RETURN.

## Purpose

F1b is the §6 row "F1: facade sparse wiring, W2 at formation, SUP-17 | after S11-F | `PP` (assembly `:1620`, `:1751`, now the kernel's sparse assembly with K2b's formation-time scaling; reduction `:2330-2342`; reactions `:2697`; `solve_preview_reduced_system`); `source_recovery.rs` (refuse scaled evidence); the nonlinear loop in `nonlinear_integration/src/lib.rs` (with T5) | Full product suites, the PHYS-R4 public fixture, LEF-small and LEF-large solved, the parity protocol, the dense-scrutiny guard", less what F1a already merged (the D-5 evidence line and SUP-17).

- **Why now.** ROOT split F1 because it consumes K1 (the kernel sparse assembly) and K2b (formation-time scaling). Both are on main at `e7d930d49`. The facade path is S11-F ✓ → S11-G ✓ → F1a ✓ → **F1b** → F2a (with D2's S-G1) → S-I → F2b per domain → F3.
- **What changes for users.** Today the product solves densely end to end in both modes (`FK/lib.rs` `assemble_global_stiffness_with_user_elements` at `PP:1826`, `:1957` and `:2280`). A 10,000-member model cannot run in either mode (the gate records `memory_refused`). A model whose formation or M03 evaluation leaves the binary64 normal range is refused (K2a's `NumericalRange`, or M03's `Range`).
- **After F1b:**
  - sparse interactive mode is pattern-only;
  - dense scrutiny is a dense view of the same values, behind a resource guard;
  - a linear case that main refuses for range is scaled by 2^b (W2) and either published, unscaled exactly, or refused by name.
  - Every case that main publishes is published byte for byte as main publishes it.
- **The both-entry gate runs here.** K1's and K2b's rulings deferred it to F1b ("K1: spawn timing and no both-entry gate"; "K2b: kernel only, and the LEF expectation restated").

### What F1b builds on (merged)

All line numbers below are on `e7d930d49`. Re-locate every one on your base.

- **K1 (PR #1034): the kernel sparse representation.** The F1b interface is `IMPLEMENTATION/K1/RETURN.md` §12. Its items live in `FK/src/structural/sparse.rs`:
  - `SparsePattern` `:48` (`from_connectivity` `:82`) and `SparseStiffness` `:218`, with `get` `:288`, `to_dense` `:299` (n²), `storage_counts` `:309`, `multiply` `:325` and `reactions` `:360`;
  - `assemble_sparse_stiffness` `:592`, whose coalesced values are bit-identical to the dense assembly in the product's order: frames and users, then blocks, then springs;
  - `reduce_assembled_sparse_system` `:729`, whose force is bit-identical to `reduce_assembled_system*`'s;
  - SA's `SparseAssemblyEvidence` (`SA:451`, `new` `:467`), its `solve_assembled_with_formation_check` (`:644`), and `dense_symmetry_view` (`:573`).
  - **K1's parity claim, which F1b now has to show at product level:** the pattern path's `StructuralSolution` is byte-identical in `Debug` to today's path, in both modes (K1 RETURN §9).
  - **The pattern path has no binary64 (option-(c)) binding** (`sparse.rs:1287-1291`: "the nonlinear loop stays on the dense `_binary64` path until F1b").
- **K2a (PR #1032): checked formation.** `local_stiffness` (`FK/lib.rs:711`) refuses a zero, subnormal or non-finite intermediate as `FrameKernelError::NumericalRange { name }`. On the product route it surfaces at the per-basis assembly (`PP:1826-1833`, `:1957-1964`) as an invocation-level `SOLVER_SYSTEM_BLOCKED` (`solver_blocked`, `PP:12082-12095`). Its product reach is the derivation section of `IMPLEMENTATION/K2A/RETURN.md`, as scoped by `RETURN_ADDENDUM_1` (correction 3, ruling 4). Cite it; do not restate its figures.
- **K2b (PR #1040): W2 in the kernel.** The F1b interface is `IMPLEMENTATION/K2B/RETURN.md` §15, as corrected by addenda 1 and 2:
  - `ForceScale` (`FK/lib.rs:866`; even b only);
  - `FrameElement::force_scaled_end_actions` (`:991`) and `force_scaled_spring_action` (`:1073`), which are checked and refuse rather than flush;
  - `ForceScaleCensus` (`:1129`);
  - `SparseStiffness::force_scaled_reactions` (`sparse.rs:403`);
  - in `FK/structural.rs`: `ForceScaleReason` `:2196`, `ForceScaledError` `:2256`, `Representability` and `PublishedValue` `:2286-2297`, `RecordOutcome` `:2321`, `ForceScaledSolution` `:2338` and `unscale_structural_solution` `:2514`;
  - the SA orchestrator `solve_with_force_scaling` (`SA:1722`) with `ForceScalingCase` (`:1540`). Its census (`SA:1583-1601`) covers every frame, user, curved slot, spring and **every load term, including terms at restrained DOFs**.
  - **K2b's rulings bind F1b:** even b (ruling 1); the census scope (2); descriptive residual records with explicit outcomes (ruling B); LEF-large restated (C); spring-carried stays a named refusal (A); the b-rule's documented limitation (`97000ab9f`); RV11-1 and RV11D-1 (publish through the checked helpers, never a wrong `Normal`).
  - **K2b's open items for F1b:**
    - RETURN §14: loads formed at b = 0 from out-of-range products;
    - the b-rule refinement;
    - RV11-N2, N3 and N4;
    - RV11D-N2 (F1b's gate measures the checked helpers' availability cost);
    - RV11D2-N1 (the double rounding of a subnormal outcome at b ≠ 0, within the stated precision; informational);
    - A2.5 (no load-term variant of the end-action helper).
    - All are in `K2B_MERGE/RECORD.md`, "For F1b".
- **K-D5 (PR #1017):** the formation check. PP calls it through `solve_assembled_with_formation_check` with `selected = built.nonlinear_supports.is_empty()` (`PP:4441-4449`). Under b it runs inside the force-scaled siblings (`force_scaled_formation_source`, `SA:1441`).
- **S11-F and S11-G (PRs #1000 and #1003):**
  - the ledger (`case_force_ledger`, `PP:2360`);
  - E12 reactions (`restrained_reactions`, `PP:2514-2533`);
  - the load-row guard (`PP:2667`);
  - the recovery guard R-b′, whose formation bound is `StraightPipeElement::bending_formation_bound` at b = 0 (`SP:701`, `local_stiffness` at `:726`; used at `PP:3373-3386`).
  - An unavailable bound makes R-b′ fire (`formation_guard.rs:456-459`).
- **F1a (PR #1025):** `append_integrity_report` (`PP:1078-1108`) renders K-D5's `formation_check:` line after S11-G's sentence, under its no-op rule. **Its follow-ups ride F1b:**
  - N1: the unreachable `row=none` branch at `PP:1118` needs a one-line comment;
  - N2: disclose that `source_receipt.rs:914-921` reserves 12× the diagnostic message bytes. F1b's new text adds to that.

### Where F1b acts on `e7d930d49`

| What | Site | Today |
|---|---|---|
| Per-basis assembly | `PP:1826-1838` (first basis), `:1957-1972` (per modulus basis), `:1873` → `assemble_case_stiffness` `:2276-2291` (each 0.4.0 resolved case) | dense n×n; formation errors block the invocation (`solver_blocked`) |
| Held stiffness | `basis_solve_states` `PP:1846-1852` (`Vec<Vec<f64>>`) | dense |
| Finiteness scan | `PP:2675-2681` (`stiffness.iter().flatten()`) | n² reads |
| Reduction and partition | `PP:2726-2735` | dense `reduce_assembled_system*`, n_f² |
| Legacy observation force | `legacy_observation_force` `PP:2481-2500` (`stiffness[row][column]`) | dense reads |
| Linear attempt | `solve_preview_reduced_system` `PP:4392-4492` (called at `:2745`): `AssemblyEvidence::new` `:4418` (n×n evidence), `solve_assembled_with_formation_check` `:4441` | dense evidence in both modes |
| DEC-050/053 observation lanes | the sparse-entry lane `PP:4452-4468` (already entries); dense only: `legacy_dense_observation` `:2505-2509` (LU on the dense reduced K) and `append_sparse_live_path_evidence` `:4553` (called at `:2931-2946`) | protected; they never select |
| Reactions | `restrained_reactions` `PP:2514-2533` → `multiply_matrix_vector` `:12198` (called at `:3119`) | dense K·u |
| Spring actions | `PP:3150` (`-k * u`) | binary64 |
| Member end actions | `pipe.recover_local_forces_from_global_model` `PP:3297` (SP:521, K2a's `local_stiffness` at b = 0) | binary64 at b = 0 |
| Retained-source input | `source_recovery::Input.stiffness: &[Vec<f64>]` (`source_recovery.rs:34`), read at `:466-471` and `:892-905`, only after the `n > 256` budget refusal at `:466`; the captured replay assembles its own dense K (`source_receipt.rs:309-319`, passed at `:660`) | dense |
| Nonlinear loop | `append_nonlinear_support_loop_results` `PP:4033` → NI `solve_active_set_frame_with_mode_and_springs_assembled` (NI `lib.rs:543`, dense assembly `:581`; binary64 solves `:1970-2013`) | dense, binary64 (option (c)) |

### What F1b does not do

- No kernel change. That means no edit to `FK`, `SA`, `SD`, `SP` or `CB`. K1, K2a and K2b supply every entry F1b needs. A needed kernel edit stops the work (see the stop list).
- **No nonlinear-loop change** (subject to Q1).
- No W1, no receipt identity, and no D-5 routing (F2a).
- No SUP-17 or D-5 line (F1a, merged).
- No capture-boundary change (D2's R-6).
- No input-validation fix: the missing lower magnitude bound, and derived A, I and J that are subnormal but pass K2a, stay routed out of T3 (K2a's ruling 3; the K2a merge record's N1).
- No timing or memory-growth claim (K6 owns them). Deterministic allocation counts in a test are allowed; they are not a measurement claim.

## ROOT rulings for this slice (2026-09-28)

A TASK drafted this brief. ROOT reviewed it and rules on its open questions as follows. The questions, with their options, remain at the end for the record. The stale-design list is recorded as rulings in `ROOT_RULINGS_V1.md`, "F1b: spawn and rulings (ROOT)".

1. **Q1: (a), the nonlinear loop is out of F1b.**
   - It stays dense and binary64, byte-identical. That is the selection's hard constraint: "The nonlinear loop stays on the legacy binary64 variants until T5".
   - Moving it to the pattern goes to T5, with a kernel slice for a pattern-path binary64 binding if T5 wants one.
   - F1b discloses the loop's dense base assembly as an M32 remainder owned by T5.
2. **Q2: (a), W2 engages after the ordinary b = 0 attempt and after exact-block,** on linear invocations only. The orchestrator stays the only W2 entry.
   - PP's range classifier is pinned equal to SA's by a test.
   - The byte-identity-by-construction claim is **derived** in RETURN and checked independently.
   - D's coexistence test is required. If no admissible construction exists, derive why and report it.
3. **Q3: (a). W2 admits at b ≠ 0 only** straight frames, ground springs, rigid restraints, prescribed support motion and authored nodal loads. Every other family is refused by name at b ≠ 0.
   - This departs from §4.7's letter, which assumes every load component is an exact input, and is recorded as a ruling. It regresses no availability, because main refuses every excluded case.
   - Full coverage (loads formed under b, and scaled recovery for curved, user and effort elements) is its own later slice, if a real case needs it.
4. **Q4: (a), today's publication functions at b = 0,** and K2b's checked helpers only at b ≠ 0.
   - RV11D-N2's availability cost is measured on the gate corpus by a scratch probe: how often the checked helpers would refuse at b = 0.
   - The method is proposed at checkpoint 0. It is an observation, not product code.
5. **Q5: (a), accept R-b′'s fail-closed demotion at b ≠ 0.** Range cases publish Sensitive at product level, and ruling C's "CONT Passed" holds at kernel level only.
   - No wrong value is published.
   - A scale-aware formation bound (option (b), an SP entry) goes on F2a's input list and the T3-close list, not into F1b.
6. **Q6: (a), with b per (c1) only where it needs no replicated formula.**
   - A W2 refusal is published per case as `NUMERICAL_INTEGRITY_UNRESOLVED`, blocking.
   - For K2a-triggered models this changes the code from `SOLVER_SYSTEM_BLOCKED`. That is class C1, cases main refuses, and no committed byte changes.
   - **b on the non-range path:**
     - If the b-rule is callable from PP as a public FK or SA function, PP calls it (c1), and a test pins its b equal to the orchestrator's on the success path.
     - If it is not, the message names the trigger and "range scaling attempted" without b (c2).
     - No b-rule formula is replicated in PP, and no SA edit is made (c3 is refused).
   - **The exact template** is proposed at checkpoint 0, modelled on PP's existing `NUMERICAL_INTEGRITY_UNRESOLVED` messages. It carries `r`'s `Display` and the step-1 trigger as `{:?}`, so `pressure_membrane_range.rs`'s `contains` assertion stays true if at all possible. ROOT fixes it at checkpoint 0.
7. **Q7: the proposed `range_scaling:` form is approved, with a length bound.**
   - The fields are `force_scale_exponent=<b>; basis=exact power-of-two`, then the non-normal outcomes in DOF order, as `subnormal=…` and `record=…` entries.
   - The number of named entries is limited by S11-G's `NAMED` limit, followed by `; more=<count>`.
   - The line is appended after F1a's `formation_check:` line, separated by one space.
   - The exact template is proposed at checkpoint 0 and fixed by ROOT.
8. **Q8: (a) and (d). A provisional dense-scrutiny ceiling now, and no sparse ceiling in F1b.**
   - **The estimate's per-entry constant** is derived at checkpoint 0 by counting, in the code, the n²-sized buffers that the dense scrutiny path holds at its peak. It is not taken from §2.1's "at least about 100 bytes".
   - **The ceiling is provisional:** 6 GiB of estimated dense-path bytes, the gate's heap cap on this Mac. It is a named constant with this provenance, and is revisited from K6's and V-P's measurements. The 1,000-member dense gate cases must stay under it.
   - **(d):** dense scrutiny uses the pattern evidence (`LinearSolveMode::DenseScrutiny`). It falls back to dense `AssemblyEvidence` if T9 or the gate shows any dense-mode byte difference, and reports that.
   - A refusal above the ceiling is a new refusal class for very large dense-scrutiny models. It is recorded as ROOT's provisional product decision and reported to the owner.
9. **Q9: (a).** `source_recovery::Input.stiffness` keeps its type. PP builds the dense view only when n ≤ 256, and a test pins that the budget refusal comes before every read. `source_receipt.rs` is unchanged.
10. **Q10: (a) for both. Scope §6 is approved as the expected outcomes;** its predictions become results at A2, where ROOT rules on the product-run table.
    - **PHYS-R4:** the design's "the public fixture then passes the evidence stage" is **restated** as a named refusal, provided A2's product run confirms that the fixture's exact-pressure end-cap operand is subnormal at formation. A no-pressure variant must solve.
    - The unmet design expectation, a scaled exact-pressure operand formation (kernel scope), goes on the T3-close list for a decision.
    - `k2a_formation_range_runtime.rs`'s linear variants and, only if Q6's template requires it, `pressure_membrane_range.rs` are declared edits.
11. **Q11: approved, under K1's conditions as adapted in the question:**
    - every change is listed with the site it removes or adds;
    - no count rises except on a new, disposed row;
    - a binary64-fold mutant in each new PP function is killed by the table;
    - the original pins' mutants are re-run and killed at the same sites;
    - the edits are declared in CHANGE_RECORD and RETURN.

    A site that fits no disposition stops the work.
12. **Q12: (a), the b-rule is wired as merged.** Every `ScaledEvaluation` refusal on the gate corpus and in D is reported with a forced-even-b probe. Any refinement is its own kernel slice.
13. **Q13: (a). ROOT supplies Mac main's gate baseline for the base.**
    - **Part 1** is run now by a separate gate-baseline TASK, which keeps `runs.jsonl` and every envelope, hashed.
    - **Part 2** (the four dense 1,000-member timeouts) is run interleaved, base and candidate, at the gate itself.
    - **The other baselines already exist,** because the base's tree `2b44cdf06…` equals K2b's final head `33e33c723`'s:
      - the 39-manifest Mac suites: `IMPLEMENTATION/K2B_MERGE/dec025/suites.log`;
      - T9's base hashes: the Mac calibration hashes (`PLATFORM_CALIBRATION_MAC/t9/`), which K2b reproduced at 112 of 112.
14. **Q14: one slice,** with checkpoint A split into A1 (the sparse wiring and the guard) and A2 (W2), and one gate.

## Scope

### 1. The sparse wiring (W3 at the facade; §4.8 "Facade after T1 merges")

- **Per-basis assembly** at `PP:1826`, `:1957` and `:2276` becomes `assemble_sparse_stiffness(…, &SparseAssemblyOptions::new())`, with the curved bends as `StiffnessBlock`s and the springs in today's order.
  - A formation error other than `NumericalRange` blocks the invocation exactly as today: the same error, `solver_blocked`, and the same diagnostics.
  - A `NumericalRange` on a **linear** invocation is deferred to the case loop (item 3). On an invocation with any nonlinear support it blocks exactly as today (Q2).
- **The partition** uses `reduce_assembled_sparse_system` and `free_position`.
- **The legacy observation force** reads `SparseStiffness::get`.
- **The finiteness scan** reads the stored values. Absent entries are +0.0. The first non-finite value in row-major order is the same stored entry.
- **The linear attempt** runs on `SparseAssemblyEvidence` in both modes (Q8(d)). In dense scrutiny, SA materializes the dense view behind the guard (item 2).
- **Reactions at b = 0** use `SparseStiffness::reactions`, which K1 states is bit-identical to `restrained_reactions`.
- **Dense views exist only:**
  - in dense scrutiny, for the protected legacy LU observation and the sparse-live-path parity rows, both behind the guard;
  - for retained-source recovery, only when n ≤ 256 (Q9).
- **The loop is untouched** (Q1).
- **No automatic dense fallback.** Mode code 3 stays reserved and unused, and `dense_fallback_message` stays `None` (`PP:4490`).

### 2. The resource guard (§4.8 "Resource guard"; Q8)

- **Dense scrutiny** refuses, before any n² allocation, a model whose dense-path estimate exceeds the ceiling. The refusal is a blocking `SOLVER_SYSTEM_BLOCKED` naming the estimated bytes.
  - The estimate is a stated formula over `SparseStorageCounts::dense_entries`, not a measurement.
  - It is checked once per invocation, before the case loop.
- **Sparse:** a profile ceiling, as ruled under Q8.
- The ceilings are named constants with their provenance, set as ROOT rules.

### 3. W2 at formation in the product (§4.7; K2b §15's recipe, as corrected)

1. **Engagement (Q2).** For each case of a linear invocation, in this order:
   1. **The ordinary attempt at b = 0,** as today: step 1. A range trigger is K2a's `NumericalRange` at the basis assembly, or `StructuralError::Range` from the evidence or the solve. PP's classification must be the orchestrator's (`SA:1627-1632`, `:1688-1696`); a test pins them equal.
   2. **Exact-block recovery,** exactly as today where it is eligible (`source_eligible`, `PP:2787-2791`; `needs_source_recovery`, `:2780`).
      - For a formation trigger, main never reaches this step: it blocks the invocation at the assembly first. F1b runs the eligible attempt here anyway, and its failed attempt is charged and reported as usual.
      - The claim that exact-block cannot select such a case (its `prepare_sources` forms the same frames, `source_recovery.rs:879`) is a derivation for RETURN (Constraints).
   3. **Only if the case is still unrecovered,** `solve_with_force_scaling` (steps 2–5).
   - Invocations with any nonlinear support never engage W2. They publish byte for byte as main.
2. **Admission at b ≠ 0 (Q3).** W2 publishes only the families F1b can publish exactly at scale: straight frames, ground springs, rigid restraints, prescribed support motion and authored nodal loads. Any other family in a case at b ≠ 0 is refused by name: an element, thermal, eigen, pressure or constant-effort load; a realized curved bend; a user-stiffness element.
3. **Publication at b ≠ 0:**
   - displacements unchanged (never scaled);
   - reactions through `outcome.stiffness.force_scaled_reactions(&u, force, b, &rigid)`;
   - straight-member elastic end actions through `FrameElement::force_scaled_end_actions`. Only nodal loads are admitted, so no fixed-end load term is added at scale (A2.5);
   - spring actions through `force_scaled_spring_action`;
   - the report through `unscale_structural_solution`. Each non-normal residual-record field in `ForceScaledSolution::records` is rendered with its field (RV11-N3), in the evidence line;
   - a `Subnormal` published value's relative precision is rendered in the same line (§4.7 step 5; §5 item 6);
   - a refused reaction, action or spring action is the step-5 refusal of the case (never flushed, never a wrong `Normal`).
   - At b = 0 nothing changes (Q4).
4. **The ledger passed to the orchestrator is the case's unscaled `AssembledForce`** (RV11-N4). F1b makes that an invariant, with a test that a pre-scaled ledger cannot reach it.
5. **The evidence line** (Q7): `range_scaling: force_scale_exponent=<b>; basis=exact power-of-two`, plus the outcomes as ruled. It appears in the integrity diagnostic, only when b ≠ 0, composed with S11-G's sentence and F1a's `formation_check:` line.
6. **Refusals (Q6):** the orchestrator's `Refused(r)`, and any F1b admission or publication refusal. Each is published per case under one ruled template carrying `r`'s `Display` and the step-1 trigger (K2a's name survives, ruling 7). On the path where the evaluation at the chosen b fails with a non-range error, F1b still carries the trigger, and b as ruled (RV11-N2; Q6(c)).
7. **The R-b′ guard at b ≠ 0 (Q5):** the formation bound at b = 0 is unavailable for a range case (`SP:726` meets K2a's refusal), so R-b′ fires. F1b either accepts that fail-closed demotion or computes the bound at scale, as ruled.
8. **The b-rule** is wired as merged (§4.7 step 4: one scaled evaluation, no third attempt; Q12).

### 4. `source_recovery.rs`

- **Refuse scaled evidence.** An `Input` that carries a force-scaled ordinary attempt (b ≠ 0) is refused as `Unsupported`, with a named reason, before any work is charged.
  - Under Q2(a), W2 runs only after exact-block has failed, so this refusal is a defensive guard.
  - Its unreachability at product level must be **derived** in RETURN and pinned at unit level.
- **The dense view** (Q9).

### 5. F1a's follow-ups

- N1: a one-line comment at `PP:1118`.
- N2: disclosed in CHANGE_RECORD, together with the receipt-reservation effect of F1b's own new text.

### 6. The LEF, reach and PHYS-R4 expectations at product level (restated; ruling C and I7 addendum 1)

These restate §6's F1 row, §4.10 and §7.1 ("LEF-small and LEF-large solved") at the level where they apply. **Each line is a required outcome, or a prediction to be confirmed by a product run at checkpoint A.** A prediction that the run contradicts is a stop, not a relabel. The lesson is ROOT's: a claim about product behaviour needs a product run.

| Case | Entry | Required outcome | Why (code on `e7d930d49`) |
|---|---|---|---|
| RF-RANGE-{CHAIN,SKEW,CONT}-LEF-small | both | **Unchanged: refused at model build,** `PIPE_ELEMENT_INPUT_INVALID` (blocking), byte-identical to Mac main. W2 never engages, because nothing is formed | `build_model_for_members` `PP:5620-5631` → `StraightPipeElement::new` (`SP:427`, calls `frame_element()` at `:441`) → `FrameElement::new` (`FK:563`, orientation at `:582`) → `from_x_axis_and_y_reference` (`:525`) → `normalize` (`:1864-1867`): \|x_j − x_i\| ≤ `AXIS_TOLERANCE` = 1e-12 m (`FK:24`) → `DegenerateAxis`. The gate records `refused_blocked` on both entries (K2a gate `final_result.json`) |
| LEF-large | captured | **Unchanged:** `Err`, no envelope | `CapturedInvocation::parse` (`PP:1620`) → `validate_checked_value` (`canonical_json/src/lib.rs:94-116`): an integral float above 2^53 − 1 is `CHECKED-JSON-UNSAFE-INTEGRAL-FLOAT` (`:106-107`). The capture fix is D2's R-6 |
| LEF-large | typed | **W2 engaged** (not source-eligible). Required: published, with every displacement component equal bit for bit to its RF base case's published component times the exact power of two (translations 2^(pf−pm−pl), rotations 2^(pf−pm−2pl); pl = 200, pm = 300, pf = 600; K2b RETURN §9 C), in both modes. Published forces are 2^pf and moments 2^(pf+pl) times the base's, or refused by name. **Standing:** never above the base's. Under Q5(a) it is Sensitive (R-b′ fail-closed); under Q5(b) it is the base's (CONT Passed; CHAIN and SKEW Sensitive). The b is the census's; derive it at checkpoint 0 (K2b's kernel b is −702) | Today K2a refuses it at the basis assembly (`PP:1826`) → `SOLVER_SYSTEM_BLOCKED` (gate: `refused_blocked`, typed). A derived stress or magnitude that leaves binary64 (`require_finite_mechanics`, `PP:2332`) would block it again: that is a finding, reported, not relabelled |
| reach_zero, reach_lef (PP `tests/k2a_formation_range_runtime.rs`, linear variants) | both | **W2 engaged.** Published; every displacement component within 1e-9 of an exact reference computed in the test; standing as ruled (Q5). K2b solved both at kernel level (b = 734) | Today: `SOLVER_SYSTEM_BLOCKED` (K2a, `12EIy/L^3: (12*E)*Iy`) |
| spring-carried (G = 1e-300 Pa; same file, `:166`) | both | **A named refusal:** `ScaledEvaluation` with the trigger `GJ/L: G*J`, in both modes, under Q6's template. "Force scaling cannot restore it" (ruling A) | M03's contribution audit refuses at every b. The ratio is scale-free |
| partial underflow (same file, `:250`, linear variant) | both | Established by product run at A: published within 1e-9, with ruling B's record outcomes rendered, or a named refusal. The PP model may differ from K2b's kernel case (load 1e-307 N, b = 898). Compare them and state which | Today: `SOLVER_SYSTEM_BLOCKED` (K2a) |
| the same file's `open_gap` (nonlinear) variants | both | **Unchanged,** byte-identical to Mac main (Q2) | The loop forms at b = 0 (NI `lib.rs:581`) and is pinned to reach no scaled entry (K2b) |
| PHYS-R4 public fixture (`tests/pressure_membrane_range.rs:90-122`) | typed (F1b adds captured) | **Predicted: a named refusal,** "range: subnormal stiffness or load at formation", with the step-1 trigger `Range("arithmetic outside normal range")`. It is **not solved** (Q10) | The stiffness coefficients are all normal (EA/L 9.4e-154, 12EI/L³ 1.41e-306, GJ/L 1.07e-307). But the fixture's own exact-pressure end-cap operand p·π·r_i² is about 2.99 quanta of 2^-1074: subnormal at formation (`<wt>/scratch/briefs/f1b_scratch/phys_r4_operands.py`). It is formed by `pressure_group_value` (`pressure_exact/source_geometry.rs:116-134`; `scaled_output`, `pressure_exact.rs:433-440`, accepts any finite output). It is pushed by `push_exact_pressure_operands` (`PP:2426-2442`) as a plain `Term` (`push_formed`, `FK/load_ledger.rs:152-163`), at restrained DOFs. The orchestrator's census reads every term (`SA:1597-1599`), and `ForceScaleCensus::load_term` flags a subnormal `Term` (`FK/lib.rs:1217-1219`, `value` at `:1146-1153`; K2b ruling 2) |
| PHYS-R4 without pressure (same section, a nodal tip load) | both | Published, within 1e-9 of an exact reference (K2b's synthetic element: b = 536). Standing as ruled (Q5) | A new product test |
| K2b's documented-limitation chain (E = 2^440; K2b RETURN §13.3) | typed (captured refuses 2^440) | **A named refusal** (`ScaledEvaluation`), pinned as a documented limitation, not "fixed" | §4.7 step 4, as ruled |

### 7. What may change in published bytes (the fixture stop rule)

- **The committed fixtures are not expected to move.**
  - No committed JSON under `P/{fixtures,validation,core}` carries a range refusal (`outside normal range`, `NumericalRange`, `range: `) or `SOLVER_SYSTEM_BLOCKED`.
  - The largest committed product model has 101 nodes (`core/product_physics/tests/fixtures/s11g/rb_controls.json`), far below any guard ceiling.
  - Re-check both with `<wt>/scratch/briefs/f1b_scratch/committed_model_sizes.py` on your base.
  - So T9's expected result is 112 of 112 byte-identical (Mac-only). **Any committed-byte change stops the work,** and nothing is regenerated without ROOT's approval.
- **The receipts.** Committed source-block, physics-source and load-reference receipts hash their envelopes and charge 12× the diagnostic bytes (`source_receipt.rs:914-921`). They move only if an envelope's text moves.
  - At b = 0 nothing may move.
  - The `range_scaling:` line and the W2 refusal text occur only in cases main refuses. So they reach a receipt only in a captured invocation that main blocks entirely (see D's mixed-invocation test).
- **The pre-registered change classes on the product.** These are the only published changes allowed, checked at the gate against a Mac run of main:
  - **C1 (range).** Runs whose Mac-main outcome is a range refusal on a linear invocation that exact-block did not recover: K2a's `SOLVER_SYSTEM_BLOCKED`, or M03's `NUMERICAL_INTEGRITY_UNRESOLVED` `Range`. On the gate corpus they are among the RF-RANGE family's 62 `refused_blocked` runs (32 cases, 128 runs; the K2a gate's Linux record; re-derive them on Mac main). Each may become published, or refused with F1b's text. **The exact list is fixed at checkpoint A and ruled before the gate.**
  - **C2 (memory).** The 24 RF-LARGE `n10000` runs (`memory_refused` on main).
    - Sparse mode may now complete, time out, or meet a named sparse ceiling. **A heap-cap abort in sparse mode is a finding:** an n² allocation remains on the pattern path.
    - Dense mode becomes the guard's refusal, before any n² allocation.
  - **C3.** Nothing else. Every other run keeps identical outcome, quality, standing and envelope bytes.

## Write set (re-locate every line on your base)

| File | Change | Status |
|---|---|---|
| `P/core/product_physics/src/lib.rs` | items 1–3 and 5 | in the F1 row |
| `P/core/product_physics/src/source_recovery.rs` | item 4: refuse scaled evidence; the dense view (Q9) | in the F1 row |
| new `P/core/product_physics/src/f1b_tests.rs` (`#[cfg(test)] mod f1b_tests;` from `lib.rs`, as F1a did) | unit tests | certain (F1a's precedent) |
| new `P/core/product_physics/tests/f1b_*.rs` | product-level tests through both entries. The allocation-counting guard test goes in **its own** test binary (it installs a counting global allocator) | certain |
| `P/core/product_physics/tests/pressure_membrane_range.rs` | PHYS-R4's restated expectation, if Q6's template does not keep today's `contains` assertion true | **approved** (Q10) |
| `P/core/product_physics/tests/k2a_formation_range_runtime.rs` | the linear variants' expectations restated (published, or a W2 refusal). The nonlinear variants are unchanged | **approved** (Q10; K2a's refusal was "K2a's intended interim") |
| `P/core/product_physics/tests/s11f_site_test.rs` | rule-8 `TABLE` counts reduced where F1b deletes a dense accumulation (`run_linear_static_preview_captured_once` 4 → 2, `assemble_case_stiffness` 1 → 0, `add_curved_bend_stiffness_contributions` and `multiply_matrix_vector` if they lose their callers); new rows for new PP functions; `t10b` if the routing site's shape changes (its `OrdinaryAttempt::passed(` count is 1) | **approved** (Q11; this is not additive) |
| `P/core/solver/nonlinear_integration/src/s11k_tests.rs` (tests only) | the product halves of `kd5_nonlinear_sources_name_no_formation_check_entry_point` (`:1116`; product half `:1186-1220`: exactly one PP call, inside `solve_preview_reduced_system`) and `k2b_force_scaled_entries_are_reached_by_neither_the_loop_nor_the_product` (`:1584`; product half `:1620-1630`: no PP mention "before F1b") become "exactly the declared PP call sites". **The loop halves are unchanged** | **approved** (Q11) |
| existing PP unit tests calling changed private functions (`lib.rs` tests, `s11f_tests.rs`, `s11g_tests.rs`, `source_recovery.rs` tests) | call-site updates only | declared; **any assertion change stops the work** |
| `P/core/product_physics/src/source_receipt.rs` | nothing under Q9(a) | **nothing** (Q9(a) ruled) |
| `nonlinear_integration/src/lib.rs` | nothing | **nothing** (Q1(a) ruled) |
| `T3/IMPLEMENTATION/F1B/**` in `<wt>/f1b` | records | certain |

**Not in scope. Stop and ask before touching any of these:**
- **The kernel:** `FK` (all of `frame_kernel/src/**`, including `sparse.rs`, `structural.rs`, `load_ledger.rs`, `formation_check.rs`, `rigid_body.rs`, `exact_sum.rs` and `retained/**`), `SA`, `SD`, `SP` (`straight_pipe`), `CB` (`curved_bend`), `diagnostics`, `load_case_algebra` and `pressure_exact`/`pressure_runtime.rs`.
- **FK's tests:** `FK/tests/s11_site_table.rs`, which is in K4's write set.
- **The frozen references and fixtures:** N01–N09, R01–R07 and NP-A–NP-D; R1's `REFERENCES/**`; the T0R references; `GATE/*.json` (both lists are empty and stay empty).
- **The committed fixtures and hash pins.**
- **Dependencies and lockfiles.**
- `performance_harness` (K6) and `numerical_robustness` (V-K and V-P).

**Constraints:**
- **Byte identity where main publishes.** Every case main publishes, on the ordinary route or through exact-block, is published byte for byte as main publishes it, on both entries and in both modes. This includes the integrity diagnostic's `Debug` report, the `linear_solver_mode_basis` row, the DEC-050/053 observation rows and every receipt.
- **No new diagnostic code** (§4.7). The guard uses `SOLVER_SYSTEM_BLOCKED`, and W2 refusals use the code ruled under Q6.
- **No new variant** of `StructuralError` or `FrameKernelError` (K2b ruling 7). PP-private error types are fine.
- **The loop stays on its binary64 path** (the selection's hard constraint; Q1).
- **Memory (I8R).**
  - Never materialize a dense matrix for a model with 10,000 or more members.
  - Dense and sparse parity stops at 1,000 members.
  - A test that could reach a dense path above the ceiling must be protected by the guard and by a capped counting allocator in its own test binary, so that a regression aborts that binary instead of exhausting the host.
- **Tests do not depend on the compiler's evaluation of any function of unspecified precision** (ROOT's Q7-reversed lesson).
  - No `powi`, `powf`, `exp*`, `ln*`, `log*`, trigonometric function, `hypot` or `cbrt` on constants in an expected value.
  - Exact references come from exact rational arithmetic or from binary64 bits.
  - A magnitude used in a comparison is written as `sqrt(a*a + b*b)`, not `hypot`.
  - **No test pins a published value that passes through the platform libm** (`hypot`- and `expm1`-derived rows differ between this Mac and Linux: `PLATFORM_CALIBRATION_MAC/RECORD.md` §1). Compare components, or use the 1e-9 criterion.
- **Every general claim is derived and independently checkable.** This covers:
  - the equivalence of PP's range classification with the orchestrator's;
  - "exact-block cannot select a formation-range case";
  - the unreachability of the scaled-evidence refusal at product level;
  - any mutant equivalence.

  ROOT has been wrong twice on underived claims.
- **Callers.** Enumerate every caller of every changed function by lexer scan (`_run_records/callers.txt`), as I7 required.

## Basis (read in this order)

T3's records are read at `<wt>/numerics`, which carries ROOT's latest rulings. Code is read on your base.

1. Root `AGENTS.md`, `agents/AGENT_TASK.md`, `_COMMON.md`, and `I8R_K1_RESUME.md` ("The Mac host" and "Platform calibration").
2. `TASK_BRIEFS/I7_F1_IMPLEMENTATION.md`, all of it, including addendum 1.
3. `T3/DESIGN_NUMERICS/DESIGN.md` revision 5a.2 (sha256 `fb62ef4a…`; hash-pinned, do not edit):
   - §1 (W2 and W3);
   - §2.1 (the dense pipeline and its estimate);
   - §4.3 and §4.3.1 with D5C-3 (context: the D-5 line and its composition);
   - §4.4's coexistence rule (context for Q2);
   - §4.6;
   - **§4.7 in full;**
   - **§4.8 in full;**
   - §4.10's "No Passed breach" (both entries);
   - §5 items 5a–8;
   - §6's F1, F2a and V-P rows and the order;
   - §7.1, §7.3 items 8–11 and 18, §7.5 and §7.6;
   - §9 D-6, D-7 and D-10.
4. `T3/ROOT_SELECTION_DESIGNS.md`: Selected item 2 ("The nonlinear loop stays on the legacy binary64 variants until T5"; the fixture stop rule), and C4 (the M32 ceilings from the K6 and V-P measurements).
5. `T3/ROOT_RULINGS_V1.md`:
   - "F1 split" and "F1a: D-5 evidence-line format";
   - "K2a: product reach …" and corrections 1–3, and "K2a product reach: main's skew standing is now established";
   - every K1 section;
   - every K2b section, including "the b-rule's window misses …" and "rulings on RV11's review";
   - "K-D5: the combined-tree gate after S11-G" (how the gate is run and recorded);
   - "K3: Q7 reversed" (the lesson);
   - "K4: spawn and rulings" (items 8 and 10).
6. **The merged records:**
   - `IMPLEMENTATION/K1/RETURN.md` §3, §9 and **§12**;
   - `IMPLEMENTATION/K2B/RETURN.md` §3–§6, §9, §13.3, **§14, §15**, and addenda 1–2;
   - `K2B_MERGE/RECORD.md`, "For F1b";
   - `REVIEW/K2B_REVIEW.md` N2–N4, RV11D-N2 and RV11D2-N1;
   - `F1A_MERGE/RECORD.md` and `IMPLEMENTATION/F1A/RETURN.md` §3–§3.1 and §6;
   - `KD5_MERGE/RECORD.md` and `K2A_MERGE/RECORD.md` (gates), with `IMPLEMENTATION/K2A/_run_records/gate/README.txt`;
   - `PLATFORM_CALIBRATION_MAC/RECORD.md` with its addendum, and `gate/` (the heap cap and the calibration driver).
7. **The T3 row of `WORK_GRAPH.md`:** the LEF item, the input-validation routing, and the performance-claims method.
8. **The code on your base:** every site in the table under Purpose; PP's `tests/{s11f_site_test, k2a_formation_range_runtime, pressure_membrane_range, formation_check_runtime}.rs`; NI's `s11k_tests.rs`; SA `:1355-1746`; FK `sparse.rs:200-760`; `FK/lib.rs:855-1300`; `FK/structural.rs:2188-2620`; `SP:427-560` and `:695-750`; headless `src/lib.rs:804` (the typed entry the gate reaches).
9. `T3/OWNER_DIRECTION.md`, "Owner decision (2026-09-28): DEC-025 on the Mac".
10. `TASK_BRIEFS/I12_K4_IMPLEMENTATION.md`, for coordination.

## Base, branch and paths

- **Branch:** `codex/piping-f1b-20260928`, from current main. ROOT creates it in `<wt>/f1b`. **At spawn, the base is main `e7d930d49`** (tree `2b44cdf06…`).
- **Target:** `<wt>/f1b-target`.
- **Scratch:** `<wt>/scratch/i13`.
- **Mutants:** one clean `git archive` copy and one clean target per mutant, under `<wt>/f1b-mut/<mutant>/`. Delete each target afterwards.
- **Python:** `<VENV>`, standard library only, for generators, scans and the gate scripts.
- **Baselines** (Q13 as ruled). The base's tree equals K2b's final head `33e33c723`'s, so two already exist:
  - the 39-manifest Mac suites: `IMPLEMENTATION/K2B_MERGE/dec025/suites.log` (with `suites_vs_baseline.txt`);
  - T9's base hashes: the Mac calibration hashes in `PLATFORM_CALIBRATION_MAC/t9/output_sha256_main_mac_native.txt`, which K2b reproduced at 112 of 112;
  - **the Mac gate baseline, part 1,** from a separate gate-baseline TASK that ROOT runs now. Its `runs.jsonl` and envelopes are kept and hashed under `<wt>/scratch/gate_base_e7d930d49/`; ROOT tells you when it is ready. Part 2 runs interleaved, base and candidate, at the gate.

## Coordination

**K4 (I12, in implementation in `<wt>/k4`).**
- Its write set: `FK/src/structural/retained/**`, one accessor in `FK/src/exact_sum.rs`, `FK/tests/s11_site_table.rs`, `FK/tests/retained_k4/`, and its records.
- **Write-set overlap: none.** F1b writes no FK file. Checked: PP's `s11f_site_test.rs` `KERNEL` list does not include `exact_sum.rs` or `retained/**`, so K4's changes do not reach F1b's site test.
- **The one conditional shared file:** `FK/tests/s11_site_table.rs`, only if F1b were allowed an FK addition (for example a load-term end-action variant, or a scale-aware formation bound). F1b does not touch it. A need for it is a stop.
- **Semantic touchpoints:**
  - F1b publishes with K2b's `Representability`/`PublishedValue`; K4 uses K3's `Binary64Outcome`. F2a unifies them (K4 ruling item 8).
  - K4 has no product caller.

**K5 (W4: `FK/rigid_body.rs`, `SA`), if it runs in parallel.**
- **Write-set overlap: none,** provided F1b makes no SA change (Q6(c) would be the one exception, which is why it is not recommended).
- **Semantic:** K5 changes `geometry()` outcomes for user and curved bodies. Whichever slice merges second merges main and re-runs its suites, T9 and gate part 1.

**K6 and V-P:** the guard ceilings (Q8). **F2a:** consumes F1b's case-loop structure (Return, "F2a interface").

**T5:** owns the nonlinear loop (Q1). F1b sends no change notice unless Q1(b) is ruled.

**Merge order.** Any slice may merge first. The second merges main, then re-runs its suites, T9 and the affected gate part before its PR merges.

**Host.**
- Implementers share the Mac under I8R's caps: at most two cargo jobs of your own, `-j 8`, `RUST_TEST_THREADS=4`, and at most three mutants at once at `-j 4`.
- **The gate and T9 are heavy phases. ROOT serializes them against K4's.**
- The 10,000-member sparse runs hold memory for their whole run, which they did not do on main (they aborted at once). So they follow Q13's worker rule, and the memory guard stays on. Check `<wt>/guard/memguard.log` after every heavy phase.

## Required tests

The predicate everywhere is the unchanged `|obs − exp| ≤ 1e-9·max(|exp|, scale)`. No new tolerance is introduced anywhere.

**A. Nothing main publishes moves**
- T9: 112 of 112 byte-identical, Mac-only, built from `git archive` copies of base and candidate on this Mac.
- The 39-manifest suites `--no-fail-fast` against ROOT's Mac baseline of the base. The only failures allowed are the three known Mac platform tests, with byte-identical failure blocks.
- K1's, K2b's, K-D5's, S11-F's, S11-G's and F1a's tests pass unchanged, except the declared Q10 and Q11 edits.
- **A b = 0 byte-identity oracle:** `append_integrity_report` with no range record is byte-identical to a verbatim copy of main's (F1a's `f1a_no_record_is_byte_identical_to_main` pattern).
- **The gate:** every run outside C1 and C2 has identical envelope bytes to Mac main (Gates).

**B. The sparse wiring (the §4.8 parity protocol)**
1. **Bitwise K:**
   - For every committed request that T9 runs, and every gate request of at most 1,000 members, `assemble_sparse_stiffness` equals the dense assembly entry for entry, bit for bit. This includes realized curved blocks and user elements in the product's order, and every modulus basis and 0.4.0 resolved case.
   - A scratch probe covers all of them, with its records. A PP test covers a declared subset: K-D5's curved models, a user element, two bases, and a 0.4.0 prescribed case.
   - The dense side is built only for these sizes.
2. **The partition and the right-hand side:** `free_dofs` and the reduced force bits equal `reduce_assembled_system*`'s, including 0.4.0 prescribed motion (KS2).
3. **Reactions:** `SparseStiffness::reactions` equals `restrained_reactions` bit for bit, including the sign of a zero, on the same corpus.
4. **Outcome-class parity between modes** on N01–N09, R01–R07, NP-B, NP-D, the T0R references and the R1 families, through both entries: the candidate's dense-against-sparse class for each (case, entry) equals Mac main's. A divergence near a screen boundary is recorded, never tuned. **Published values between modes stay within the DEC-053 basis** (the candidate reproduces main's parity rows byte for byte).
5. **Two modulus bases** in one invocation, each case on its own values; mutation 11's control.
6. **Relabelling and permutation:** the same answers, and byte identity with main for each numbering (K1 found the 1e-9 relabel check blind to a label-dependent order; byte identity catches it).
7. **The nonlinear gap, one-way and friction models** publish byte-identically to Mac main (the loop is unchanged).

**C. Memory and the guard**
- **Sparse mode is pattern-only.** In its own test binary with a counting global allocator, a sparse-mode product solve of a model sized so that the dense view alone needs at least four times the asserted peak shows peak heap below that bound. Size and bound are proposed at checkpoint 0. The count is deterministic and is not a memory-growth claim.
- **The guard.**
  - A unit test of the decision function on synthetic `SparseStorageCounts`, just below, at and just above the ceiling.
  - **A unit-level product test with the ceiling lowered through a `#[cfg(test)]` hook** (K2b's forced-b precedent). A small model above the lowered ceiling must give `SOLVER_SYSTEM_BLOCKED` with the estimated bytes and no mechanics rows, in both entries. This is the test that kills a removed guard call at an assertion.
  - An integration test at the real ceiling, with a model just above it (never 10,000 members): the same refusal, and peak heap far below the dense estimate, under a **capped** counting allocator in its own test binary.
    - The cap is a safety net: a regression aborts that binary instead of exhausting the host.
    - Such an abort is not a counted kill.
- **No fallback:** a sparse-mode refusal is never rescued by dense, and mode code 3 is never emitted.
- Record the debug wall time of the C tests. They run at full size in the default suite at opt-level 0, with no `#[ignore]`. If one exceeds about a minute, report it to ROOT.

**D. W2 in the product** (both entries where reachable, both modes)
- Every row of Scope §6, as required or as ruled after checkpoint A.
- **Engagement:**
  - A linear case whose b = 0 attempt range-triggers publishes through W2.
  - An invocation with a nonlinear support never engages W2, and stays byte-identical to main.
  - **PP's range classification equals the orchestrator's** on every range and non-range case in B and D.
- **Coexistence with exact-block (Q2):**
  - A captured, source-eligible case whose b = 0 attempt range-triggers and which **main's exact-block recovers** publishes byte-identically to Mac main, receipt included. A candidate construction is a subnormal nodal load (for example 1e-310 N) on an axis-aligned cantilever in exact-block's scope.
  - Show by a product run on Mac main that main selects it. **If no admissible construction exists, derive why, and report it** (ROOT rules, as for M31b).
- **The mixed captured invocation:** one case exact-block-selected, another W2-published. It finalizes with a consistent receipt, or fails closed. It never publishes an inconsistent receipt. Pin the outcome.
- **Admission (Q3):** a range case at b ≠ 0 carrying each excluded family (a thermal load, a uniform load, an exact-pressure operand, a constant-effort support, a realized curved bend, a user element) is refused by name. The same models at b = 0 are unchanged.
- **Publication at b ≠ 0,** at product level, as the analogues of RV11's probes:
  - F-A2: reactions and member actions that leave the normal range at 2^b are **refused, never a wrong `Normal`**;
  - F-S: a spring action likewise;
  - an in-range reaction publishes 2^-b times the scaled exact sum, rounded once;
  - a `Subnormal` published value carries its precision in the evidence line;
  - residual-record outcomes are rendered with their fields (RV11-N3);
  - displacements are never scaled.
- **The unscaled-ledger invariant** (RV11-N4): a ledger already at 2^b cannot reach the orchestrator, by type or by a tested check.
- **The evidence line:**
  - the exact template, as ruled (Q7);
  - absent at b = 0;
  - exactly one per case at b ≠ 0;
  - its composition with S11-G's sentence and F1a's `formation_check:` line;
  - the no-op rule.
- **Refusals:** the exact template (Q6) for every `ForceScaleReason` the product can reach, each with the step-1 trigger. K2a's names survive (spring-carried keeps `GJ/L: G*J`). The non-range failure at the chosen b carries the trigger, and b as ruled (RV11-N2).
- **The census on a subnormal derived section value** on the range path is a refusal by name. The same value at b = 0 is unchanged. This documents the routed input-validation finding; it is not a fix.
- **The b-rule:** the documented-limitation chain is refused by name (typed), and is not fixed.

**E. `source_recovery.rs`**
- A unit test: an `Input` carrying a force-scaled attempt is refused (`Unsupported`, named reason) with zero work charged. Its product-level unreachability under Q2(a) is derived in RETURN.
- **The dense view (Q9):** a captured Sensitive case with n > 256 gets main's `Budget` refusal byte for byte. A case with n ≤ 256 gets main's recovery byte for byte (the n > 256 check precedes every read of `stiffness`, `source_recovery.rs:466-471`).

**F. Pins**
- The NI pin extensions (Q11): the product halves name exactly F1b's declared PP sites; the loop halves are unchanged.
- The original pins' mutants (K1's and K2b's tables) are re-run and killed at the same sites.
- PP's site test (Q11): the new rows, the reduced counts, and `t10b`.

**G. The gate** (see Gates).

## Mutants

Run from clean copies, with a NONE control first. Each mutant must be killed at a behavioural or pin assertion. Name the killing test. A stack-overflow or allocator abort is not a kill (K2b's K2B-LEAK-OPTIONS ruling). A survivor is a defect to report. Never weaken a test to kill one. Where no admissible control exists, derive the equivalence and report it; ROOT rules.

**The design's §7.3 items that touch F1b:**

| # | Mutant | Intended kill |
|---|---|---|
| 8 | Omit a pattern entry (a spring) in sparse mode only | B1 bitwise K; B4; T9 |
| 9 | Publish without unscaling by 2^b | D (reach_zero or reach_lef values; LEF-large's powers of two) |
| 10 | The order depends on labels | B6 byte identity |
| 11 | One modulus basis reused for every case | B5 |
| — | b ≠ 0 leaking into b = 0 (the line printed at b = 0, or W2 engaged without a range trigger) | A's oracle; T9; D |

**F1b's own:**

| # | Mutant | Intended kill |
|---|---|---|
| F1B-M1 | W2 engaged for an invocation with a nonlinear support | the nonlinear reach variants (byte identity) |
| F1B-M2 | W2 run before exact-block, so the case never reaches exact-block | D's coexistence test |
| F1B-M3 | The `range_scaling:` line omitted at b ≠ 0 | D (line tests) |
| F1B-M4 | A `Subnormal` outcome rendered without its precision | D |
| F1B-M5 | Residual-record outcomes not rendered | D (partial underflow, or a constructed record) |
| F1B-M6 | Reactions at b ≠ 0 through `SparseStiffness::reactions` on the scaled K | D (the F-A2 analogue) |
| F1B-M7 | Member actions at b ≠ 0 through the b = 0 SP recovery | D (reach) |
| F1B-M8 | A spring action at b ≠ 0 as unchecked `-k·u` | D (the F-S analogue) |
| F1B-M9 | The guard's call removed, or its comparison inverted | C (the lowered-ceiling unit test; the decision-function test) |
| F1B-M10 | Sparse mode materializes the dense view | C (the peak-heap assertion; the capped allocator makes it fail safe) |
| F1B-M11 | The scaled-evidence refusal removed from `source_recovery.rs` | E (unit) |
| F1B-M12 | The admission rule dropped (an excluded family published at b ≠ 0) | D (admission) |
| F1B-M13 | A pre-scaled ledger passed to the orchestrator | D (RV11-N4) |
| F1B-M14 | The step-1 trigger dropped from a refusal | D (spring-carried's `GJ/L: G*J`) |
| F1B-M15 | The per-basis `NumericalRange` still blocks the linear invocation | D (reach) |
| F1B-M16 | PP's range classifier admits a non-range error | D (the classification pin) |
| — | The NI pin mutants: a second PP call site of the orchestrator; a PP scaled helper named outside the declared function; the loop reaching a scaled or pattern entry | F |
| — | Your own, at least two | — |

## Gates (ROOT runs the PR)

- **Suites and T9,** as in A.
- **The both-entry gate** (K-D5's two-part method, on this Mac, against a Mac run of main; Q13):
  - **Tools:** P1's probe (`DETECTION/probe/main.rs.txt`, `8dc727f4…`) with the calibration's 6 GiB heap cap appended (`PLATFORM_CALIBRATION_MAC/gate/heap_cap_appended_to_p1_probe.rs.txt`), built `--release --offline` twice, from `git archive` trees of base and candidate. Then P1's `gen.py`, `run.py` (`run_one`) and `compare.py`, and K-D5's `gate_check.py` (`8ad89fca…`), all unchanged.
  - **Inputs:**
    - references at `c0f14201c` (`references.json` `7b176dbb…`);
    - the requests re-generated and hash-checked against `PLATFORM_CALIBRATION_MAC/gate/gen_out_sha256.txt` (222);
    - the empty `GATE/S11_EXCEPTIONS.json` and `GATE/FORMATION_EXCEPTIONS.json`.
  - **Part 1:** 884 runs. **Part 2:** the 4 known dense timeouts (RF-LARGE-CHAIN-n01000-ROT and RF-LARGE-TREE-n01000-AX, dense, both entries), on a quiet host with no other cargo. Run them interleaved base, candidate, base, candidate, one at a time, each at 1800 s. The verdict is the union of both parts.
  - **PASS requires:**
    - 0 trusted breach triples;
    - every run outside C1 and C2 byte-identical to Mac main (envelope sha256 per (case, mode, entry));
    - every C1 run in the list ROOT ruled at checkpoint A, and published within the criterion or refused by name;
    - no C2 sparse run aborting at the heap cap.
  - **Record:** both `runs.jsonl` files with their sha256, the standing and byte comparisons, and the C1 and C2 tables.
  - **RV11D-N2's measurement,** as Q4 rules.
  - **No timing is compared.**
- **An independent complete-diff review,** with oracles independent of your tests:
  - an exact-rational solve of a subset of the C1 runs;
  - an independent derivation of the engagement order's byte-identity claim;
  - a re-run of the NI pins' original mutants.
- **Hosted CI** green on the candidate head, with the full-SHA dispatch. Record the numerical job's time.
- **DEC-025** under the owner's Mac decision (`OWNER_DIRECTION.md`, 2026-09-28):
  - the Mac sweep, whose only cargo failures are the three known platform tests, identical to Mac main;
  - pytest, vitest and the build pass;
  - hosted Linux CI's numerical cargo job supplies the clean Linux cargo run;
  - the deviation is recorded in the merge record.
- **The src-tauri suite** is run, as I7 and I3 did. **Native witnesses:** §7.5's PHYS-R4 and 1,000-member witnesses are join items, unless ROOT rules otherwise. F1b changes the solve path the desktop reaches, so ROOT may want one witness here.

## Checkpoints

End your turn at each one with a status for ROOT: the changed files, the results, and any stop. ROOT verifies, commits and resumes you.

- **0: a plan, before any product code.** It covers:
  - the case loop's new structure, with exact signatures:
    - the per-basis state (the `SparseStiffness` and the deferred `NumericalRange`);
    - the ordinary attempt, the exact-block attempt and the W2 attempt, in order;
    - how `OrdinaryAttempt` and the receipt follow the published verdict (S11-G G-2);
    - `solve_preview_reduced_system`'s new signature;
  - PP's range classifier, and its pinned equality with the orchestrator's;
  - the admission rule's implementation (Q3), and the per-family refusal texts;
  - the publication path of every published quantity at b ≠ 0: kernel helper, unchanged, or refused. This includes stresses, magnitudes, preview-physics rows, constant-effort rows and combinations;
  - R-b′ at b ≠ 0 (Q5);
  - the guard's estimate formula, constants and messages (Q8);
  - the evidence-line and refusal templates (Q6, Q7);
  - the dense view for source recovery (Q9);
  - the NI and PP pin edits (Q11);
  - **the derivations** listed under Constraints;
  - the test list, with the C tests' sizes and bounds, and the mutant list;
  - the product runs planned for A;
  - your position on every open question still unresolved.
- **A1: the sparse wiring.** It includes:
  - a clean compile and a warning-free non-test build;
  - B1–B7 targeted;
  - C;
  - T9 against the base (Mac-only).

  A committed-byte change here stops the work.
- **A2: W2.** It includes:
  - D and E targeted;
  - **the product-run table:** every RF-RANGE run (32 cases, 128 runs), LEF-small, LEF-large, the reach cases, PHYS-R4 with and without pressure, and the limitation chain, through both entries and both modes. For each run it gives main's outcome, the candidate's, b, the standing, and the 1e-9 comparison against the reference;
  - **the proposed C1 list.** ROOT rules on the table and the list before B.
- **B:** the suites against the Mac baseline, T9, and the gate's parts 1 and 2 with the comparisons.
- **C:** the mutation table, with the NONE control first, and the original pins' mutants with their kill sites.
- **D:** CHANGE_RECORD (following `.agents/skills/chirality-change/SKILL.md`) and RETURN, with `_run_records/` and SHA256SUMS.

**Stop and report** (end your turn) on any of these:
- a committed-byte change in T9;
- a gate run outside C1 and C2 whose bytes, outcome or standing differ from Mac main;
- a trusted breach;
- a C2 sparse run aborting at the heap cap (an n² allocation remains);
- a needed edit outside the write set, above all in FK, SA, SD, SP, CB or the NI loop;
- a product outcome that contradicts a Scope §6 prediction;
- a reference mismatch (never edit a reference);
- an original pin mutant that is no longer killed;
- a surviving mutant;
- a SIGKILL from the memory guard (check `<wt>/guard/memguard.log`; do not retry blindly).

## Return

- **Files:** `T3/IMPLEMENTATION/F1B/` on the F1b branch: CHANGE_RECORD, RETURN, `_run_records/` and SHA256SUMS.
  - Use placeholders only (`<wt>`, `<scratch>`, `<VENV>`, `<home>`), with no machine paths and no model identifiers.
  - State the platform (`aarch64-apple-darwin`, rustc 1.97.1), and that T9 and the gate are Mac-only comparisons against Mac main.
- **RETURN covers:**
  - the files and their line counts;
  - each Scope item, with the design's words and the rulings;
  - the checkpoint-0 positions as ruled;
  - the product-run table and the C1 and C2 tables;
  - the gate's results;
  - T9 and the per-crate counts against the baseline;
  - the mutation table;
  - the callers;
  - the allocation counts from C;
  - the derivations;
  - the toolchain and host;
  - the delegation mechanism;
  - what was not done.
- **RETURN has an "F2a interface" section with exact signatures,** as K1's §12 did for F1b. It includes:
  - the case loop's attempt order and its PP-private types and functions (exact signatures), and where F2a's W1 attempt and the coexistence decision attach;
  - the mapping from F1b's outcomes to §4.3's trigger list. "A `Range` error that W2 scaling could not resolve" is F1b's refusal set, named variant by variant;
  - the `range_scaling:` line and refusal templates, and the composition order F2a's `RETAINED_PRECISION_*` diagnostics must respect;
  - the K2b publication types PP now uses (for F2a's unification with `Binary64Outcome`);
  - the guard's function, constants and `SparseStorageCounts` use (for V-P);
  - the final NI and PP pin lists F2a will extend.

## Open questions for ROOT (each with a recommendation)

**Q1. The nonlinear loop (the F1 row's third item).**
- **The conflict.** The row puts "the nonlinear loop in `nonlinear_integration/src/lib.rs` (with T5)" in F1, and §4.6 says W3 moves the loop to the pattern. But:
  - the selection's hard constraint is "The nonlinear loop stays on the legacy binary64 variants until T5";
  - K1's pattern path offers no binary64 (option-(c)) binding (`sparse.rs:1287-1291`);
  - K1's and K2b's pins keep the loop off the pattern and scaled entries.
- **(a) Out of F1b.** The loop stays dense and binary64, byte-identical. Its move goes to T5, with a kernel slice for a pattern-path binary64 binding if T5 wants one.
- **(b) In F1b, with T5's agreement.** That needs FK and SA work (the binding) and a pin redesign. That is kernel scope in a facade slice.
- **Recommendation: (a).** A nonlinear model therefore still assembles densely in the loop, in both modes. That is disclosed as an M32 remainder with T5 (§8's "Friction influence solves" row gains "the loop's base assembly").

**Q2. Where W2 engages.**
- **(a) After the ordinary b = 0 attempt and exact-block.** W2 applies only to a linear case whose b = 0 attempt range-triggers and which exact-block does not recover. Invocations with a nonlinear support never engage it.
- **(b) The orchestrator as the only linear entry** (K2b ruling 6's "one entry"), for every linear case.
- **(c) As (b), but only for invocations that are not source-eligible.**
- **Recommendation: (a).**
  - On main, a range-refused case that exact-block does not recover blocks its whole invocation (`append_integrity_failure` is blocking, and `has_blocking` then applies).
  - So (a) changes only invocations main publishes nothing for. Given the sparse wiring's bit identity, **every case main publishes stays byte-identical by construction**. That includes exact-block-recovered range cases, whose rejected-attempt diagnostic and receipt would change text under (b).
  - It mirrors §4.4's coexistence rule for W1, and K2b's loop pin.
  - **The cost:** step 1 runs twice on the range path (once in PP, once inside the orchestrator), and PP carries a range classifier pinned equal to SA's. The orchestrator stays the only W2 entry.

**Q3. What W2 admits at b ≠ 0.** K2b's RETURN §14 records the problem: loads PP forms at b = 0 from out-of-range products have already lost bits, and a term that underflowed to exactly zero is invisible to the census. A2.5 records a second: no end-action helper takes load terms at scale.
- **(a) Admit only straight frames, ground springs, rigid restraints, prescribed motion and authored nodal loads** at b ≠ 0. Any other family is refused by name at b ≠ 0.
- **(b) Full coverage.** Form each PP load producer under b, and add scaled recovery for curved, user and effort elements. That means edits to the S11 producers and to SP, CB and FK.
- **(c) Admit everything, and rely on the census.** That is unsafe: a formed zero is silent.
- **Recommendation: (a).**
  - Every excluded case is refused on main today, so there is no availability regression.
  - (b) is W1b-scale work. It can follow when a real case needs it.
  - This departs from the design's letter (§4.7's admitted range assumes every load component is an exact input), so it is recorded as a ruling.

**Q4. Which publication functions run at b = 0.**
- **(a) Today's functions at b = 0,** and K2b's checked helpers only at b ≠ 0. Today's functions are E12 `reactions` (K1 states it is bit-identical to `restrained_reactions`), SP's recovery and `-k·u`.
- **(b) The checked helpers at every b.** Their values at b = 0 carry the same bits where they publish, but they refuse wherever any single product is subnormal (RV11D-N2). That would be a new published refusal class on cases main publishes.
- **Recommendation: (a).**
  - RV11D-N2's cost is then measured as ROOT ruled ("F1b's gate measures it"): a scratch probe counts, on the gate corpus, how often the checked helpers would refuse at b = 0, from the published displacements and the corpus models' primitives.
  - It is an observation, not product code. The method is proposed at checkpoint 0.

**Q5. S11-G's R-b′ guard at b ≠ 0.** Its bound is formed at b = 0 (`SP:726`), meets K2a's refusal on a range case, and then fires as "unavailable" (`formation_guard.rs:456-459`). Every straight-member case at b ≠ 0 would publish Sensitive.
- **(a) Accept the fail-closed demotion.** LEF-large CONT, reach_zero and reach_lef publish Sensitive at product level; ruling C's "CONT Passed" holds at kernel level only.
- **(b) A scale-aware bound:** a new SP entry that takes the scaled `FrameElement` (for example `bending_formation_bound_for(&FrameElement, u)`), unscaled upward by 2^-b. That is a declared SP write-set extension.
- **(c) Replicate SP's formula in PP.** That is drift risk for a load-bearing guard.
- **Recommendation: (a) for F1b,** with (b) as a follow-up before F2a if ROOT wants Passed standing on range cases. No wrong value is published either way.

**Q6. How a W2 refusal is published.**
- **(a) Per case, `NUMERICAL_INTEGRITY_UNRESOLVED`, blocking** (§4.7: "The case stays `NUMERICAL_INTEGRITY_UNRESOLVED` with a precise reason").
  - The message is a ruled template, for example `Load case <id>: <r Display>; trigger=<{:?} of the step-1 trigger>; global_dof_map=<…>`.
  - For K2a-triggered models this changes the code from `SOLVER_SYSTEM_BLOCKED` to `NUMERICAL_INTEGRITY_UNRESOLVED` (C1; no committed byte). With `trigger={:?}`, `pressure_membrane_range.rs`'s `contains("Range(\"arithmetic outside normal range\")")` stays true.
- **(b) Keep `SOLVER_SYSTEM_BLOCKED` for formation-triggered refusals.**
- **(c) RV11-N2's b on the non-range path,** in one of three ways:
  - (c1) PP recomputes b with FK's public `ForceScaleCensus`, pinned equal to the orchestrator's b on the success path;
  - (c2) the message names the trigger and "range scaling attempted" without b;
  - (c3) an SA change, which is K5's file.
- **Recommendation: (a) with (c1).** ROOT fixes the exact template, as it did for F1a.

**Q7. The `range_scaling:` line's template.**
- **Recommendation:** `range_scaling: force_scale_exponent=<b>; basis=exact power-of-two`, then for each non-normal outcome, in DOF order:
  - `; subnormal=<label>:<quantity>:relative_precision=<{:?}>`;
  - `; record=<record field>@<label>:<subnormal(relative_precision=<{:?}>)|underflow|overflow>`.
- `<label>` is `integrity_dof_label`, with its `global_dof=<i>` fallback, and every f64 uses `{:?}`.
- The line is appended **after** F1a's `formation_check:` line, separated by one space. So F1a's composition is a prefix, and a b = 0 case is byte-identical to main.
- ROOT may prefer a count-and-first-N form, as S11-G's `NAMED` limit does, to bound the line length on large models.

**Q8. The resource guard, and the conflict with C4's measurement order.** C4 selects the M32 ceilings "from the K6 and V-P measurements". K6 has not run, and V-P comes after F1b and F2a.
- **(a) The mechanism, with provisional ceilings ROOT sets now from deterministic counts,** revisited after K6 and V-P:
  - dense scrutiny refuses when the estimate 100·n² bytes exceeds 6 GiB (the gate's heap cap). §2.1 gives "at least about 100 bytes per n² entry" as an estimate from the source; the constant is ROOT's to fix. The ceiling is then n ≤ about 8,000, about 1,330 chain members. The 1,000-member dense gate cases (about 3.4 GiB) still run as today, and every 10,000-member dense run is refused before allocating;
  - the sparse profile ceiling is none in F1b, recorded. The pattern path's profile is only known after ordering inside SA's solve. Computing it in PP first would mean a second prepare and RCM;
  - the guard message names the formula and the value;
  - the `linear_solver_mode_basis` row's `dense_scrutiny_available=true` is unchanged, because it describes the product's modes, not one model.
- **(b) Wait for K6.** F1b then cannot merge sooner.
- **(c) The mechanism with no ceiling.** That gives no protection.
- **(d) The representation in dense scrutiny:** pattern evidence with `LinearSolveMode::DenseScrutiny`, or today's dense `AssemblyEvidence`. K1 pins both as bit-identical.
- **Recommendation: (a),** and for (d) the pattern evidence (§1's "one representation"), falling back to dense evidence if T9 or the gate shows any dense-mode byte difference.

**Q9. The dense view for retained-source recovery.**
- **(a)** `source_recovery::Input.stiffness` stays `&[Vec<f64>]`. PP builds the dense view only when n ≤ 256, and passes an empty slice otherwise.
  - `prepare_sources` refuses n > 256 by budget (`:466`) before any stiffness read (`:470-471`), so main's charge and bytes are kept.
  - A test pins that order.
  - `source_receipt.rs` is unchanged: its replay assembles its own n ≤ 256 dense K.
- **(b)** `Input.stiffness: &SparseStiffness`, with the fold check on `get`. That touches `source_receipt.rs:309-319` and `:660`.
- **Recommendation: (a).**

**Q10. The restated product-level expectations** (Scope §6), and the existing tests they change.
- **(a) Approve §6 as the expectation.** ROOT rules on the product-run table at A2, where predictions become results. Then approve the declared changes:
  - `k2a_formation_range_runtime.rs` (linear variants);
  - `pressure_membrane_range.rs`, only if Q6's template breaks its `contains` assertion.
- **PHYS-R4:** the design's "The public fixture then passes the evidence stage" (§4.7) and the F1 row's "the PHYS-R4 public fixture" are **not reachable with K2b's census**. The fixture's own pressure load is subnormal at formation.
  - **(a)** Restate it as a named refusal, with a no-pressure variant that solves.
  - **(b)** Extend W2 to form exact-pressure operands at 2^b. That needs `pressure_exact` edits and an orchestrator entry taking a pre-scaled ledger: kernel scope.
- **Recommendation: (a) for both.**

**Q11. The pins and site tests** (not purely additive, unlike K1's and K2b's extensions).
- **NI `s11k_tests.rs`:** the product halves of the K-D5 and K2b pins change from "one call" and "none before F1b" to "exactly F1b's declared PP sites". The loop halves are unchanged.
- **PP `s11f_site_test.rs`:**
  - rule-8 counts fall where dense accumulations are deleted;
  - new rows are added for new PP functions;
  - `t10b` is kept, or amended if the routing site's shape changes.
- **Recommendation: approve, under K1's conditions adapted:**
  - every change listed with the site it removes or adds;
  - no count rises except on a new, disposed row;
  - a binary64-fold mutant in each new PP function is killed by the table;
  - the original pins' mutants are re-run and killed at the same sites;
  - it is declared in CHANGE_RECORD and RETURN.
- A site that fits no disposition stops the work.

**Q12. The b-rule refinement** (K2b, `97000ab9f`, ruling 3: "F1b's brief must consider it").
- **(a) Wire the rule as merged.** Report every `ScaledEvaluation` refusal on the gate corpus and in D. For each one, a scratch probe states whether a forced even b in the window (the public `assemble_sparse_stiffness(…with_force_scale)`, `SparseAssemblyEvidence::new_force_scaled` and `solve_force_scaled_with_formation_check`) would have solved it.
- **(b) Refine it in F1b.** That is kernel work.
- **Recommendation: (a).**
  - The limitation fails safe: a named refusal, and nothing wrong is published.
  - Its reach needs about 1,450 binary orders between a load and the stiffness in its row.
  - Any refinement is its own kernel slice, with a derivation and a review, if (a)'s report shows reach.

**Q13. The gate on the Mac.**
- **(a)** ROOT runs Mac main's gate for the F1b base once: parts 1 and 2, `runs.jsonl` kept and hashed. It is the baseline for the standing and byte comparisons. The implementer runs the candidate with the same tools, in ROOT-serialized slots.
  - **Part 1:** the calibration driver, with at most 4 workers for runs under 10,000 members, and **the 12 sparse 10,000-member runs one or two at a time**. They now hold memory; on main they aborted at once.
  - **Part 2:** interleaved, one at a time, on a quiet host.
- **(b)** The implementer runs both.
- **Recommendation: (a).** The base run also settles C1's list at A2, from Mac data rather than the K2a gate's Linux record.

**Q14. One slice, or two** (F1b-1: the sparse wiring and the guard; F1b-2: W2 in the product).
- **Recommendation: one slice,** with checkpoint A split into A1 and A2, and one gate.
  - A1's T9 finds a sparse-wiring defect before W2 lands.
  - The gate is the expensive part. By estimate it is about 50 minutes of part 1 per tree (the Linux figure; the 10,000-member sparse runs now add to it), plus up to 4 hours of part 2 for both trees on this Mac. Running it once covers both.
  - Two PRs would make each diff attributable, as the design prefers for K2a and S11-K. If ROOT values that more than one gate run, split at A1.

## Design text made stale by merged slices (for ROOT; `DESIGN.md` stays hash-pinned)

Items 1–11 are not yet recorded as rulings. Items 12–15 are already ruled and are listed so that F1b's reader has one place to look.

1. **§6's F1 row and §4.8's citations have drifted.**
   - The assembly (`PP:1620`, `:1751`) is now `:1826`, `:1957` and the 0.4.0 `assemble_case_stiffness` at `:2276` (called at `:1873`).
   - The reduction (`:2330-2342`) is now `:2726-2735`.
   - The reactions (`:2697`) are now `:3119` → `:2514-2533`.
   - `solve_preview_reduced_system` is now at `:4392`.
   - §2.1's `dense_fallback_message` (`PP:4004`) is now `:4490`.
   - The headless typed entry is unchanged at `headless/src/lib.rs:804`.
2. **§6's F1 row and §4.6/§4.8 "the nonlinear loop moves to sparse"** conflict with the selection's hard constraint ("The nonlinear loop stays on the legacy binary64 variants until T5") and with K1's pattern path, which has no binary64 binding (`sparse.rs:1287-1291`). See Q1.
3. **§6's F1 row, §4.10 and §7.1, "LEF-small and LEF-large solved,"** restated at product level (Scope §6):
   - LEF-small is refused at model build (`PIPE_ELEMENT_INPUT_INVALID`, `DegenerateAxis`) on both entries;
   - LEF-large is refused at capture on the captured entry;
   - on the typed entry W2 applies, with the base case's powers of two, and standing as ruled (Q5).
4. **§4.7, "The public fixture then passes the evidence stage," and the F1 row's PHYS-R4 test.** The fixture's own exact-pressure end-cap operand is subnormal at formation (about 3 quanta of 2^-1074), so K2b's census refuses it (Q10). The hand arithmetic covered stiffness only.
5. **§4.7's admitted range ("every nonzero … load component is normal binary64")** assumes loads are exact inputs. PP forms some loads at b = 0 from products, and a term formed to exactly zero is invisible to the census (K2b RETURN §14). No end-action helper takes load terms at scale (A2.5). See Q3.
6. **§4.7's "Refusal. The case stays `NUMERICAL_INTEGRITY_UNRESOLVED`."** Since K2a, a formation-triggered range case is refused at the invocation's assembly as `SOLVER_SYSTEM_BLOCKED` (`PP:1826-1833` → `:12082-12095`), so "stays" is wrong for it. See Q6.
7. **§4.7's evidence line, and §5 item 6's "relative precision stated in the evidence line":** the design gives no template for the subnormal precision or for ruling B's residual-record outcomes (RV11-N3), and no composition with F1a's line. See Q7.
8. **§4.7's mechanism ("`AssemblyEvidence` gains a private `force_scale_exponent: i32` … `solve()` scales K and f").** K2b implemented it differently:
   - `force_scale: ForceScale` (even only) on both evidences;
   - force-scaled siblings and an orchestrator;
   - existing entries refuse a scaled evidence.
9. **§4.8's "Resource guard … ROOT picks both from measurement," with C4's "from the K6 and V-P measurements":** V-P follows F1b, and K6 has not run. See Q8.
10. **§4.8's "`source_recovery` gets a dense view for n ≤ 256 only, if option B were chosen."** The n ≤ 256 limit is `source_recovery.rs:466`. The captured replay also assembles a dense K (`source_receipt.rs:309-319`), a file the F1 row does not list. See Q9.
11. **§4.7 with §4.4.** The design does not order W2 against exact-block. Engaging W2 first would change which method publishes wherever exact-block recovers a range-triggered case today, and §4.4's coexistence rule covers only W1. See Q2.
12. *(Ruled.)* §4.7 step 3's midpoint has become even b (K2b ruling 1).
13. *(Ruled.)* §4.7 step 5: residual records are descriptive, with explicit outcomes (ruling B).
14. *(Ruled.)* §6's K2b row: LEF restated at kernel level (the K2b LEF ruling and ruling C), and spring-carried not restored (ruling A). §4.7's "PHYS-R4 … b ≈ 500" is 536 at kernel level (K2b RETURN §10).
15. *(Ruled.)* §6's F1 row, SUP-17 and the D-5 line (§4.3.1 D5C-3, §5 item 5a, "From F1"), and §9 D-10's facade order "S11-F → F1 → F2a": F1 is split, F1a is merged, and S11-G was inserted before F1a ("F1 split"; the S11-G rulings).

**Recommendation:** ROOT records items 1–11 as rulings when it rules on this brief, as it did for K4's list. `DESIGN.md` is not edited.
