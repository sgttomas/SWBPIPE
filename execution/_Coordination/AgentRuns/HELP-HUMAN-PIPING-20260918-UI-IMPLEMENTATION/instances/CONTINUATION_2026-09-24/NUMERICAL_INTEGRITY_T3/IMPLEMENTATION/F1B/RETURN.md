# I13 return: facade slice F1b (the product's sparse wiring, the dense-scrutiny guard, and W2 at formation in the product)

**Status:** implemented and verified through the gate. Candidate head `130445db2`. The both-entry gate on that head is a PASS, accepted by ROOT (`ROOT_RULINGS_V1.md`, "F1b: gate re-run on 130445db2 accepted; the Mac is released", numerics `74850700d`). These are the checkpoint-D records.

**Three things for the reader first.**
1. **The first gate FAILED.** On `948e0bb99`, 4 C2 sparse runs (RF-LARGE-CONT-n10000-AX and -ROT, both entries) aborted at the heap cap in main's unchanged DEC-050/053 observation lane. ROOT ruled a guard for that lane (`9fa4d1b59`). It was implemented as `130445db2`, and the re-run passed (§9).
2. **A correction to A2.** A2 reported three admission checks as unreachable in the product; ROOT approved that with "the unreachability derived in RETURN". Two are (`user_stiffness_element`, `non_nodal_load_term`). **`pressure_thrust_load` is reachable:** a zero-valued pressure element load on the legacy route reaches it. The case is refused by name on both entries and in both modes, main refuses the same runs, and the b = 0 twin is byte-identical. No product change is needed. A product-level pin is offered (§15, D11).
3. **The coexistence derivation** (ROOT's A2 condition) is written step by step in §15, D10. Byte identity wherever exact-block selects is **proven by construction**. That no range-triggered case can be selected is proven for formation triggers and for large magnitudes, and **observed, not proven,** for small-magnitude evaluation triggers.

- **Branch:** `codex/piping-f1b-20260928`, from main `e7d930d49`, in `<wt>/f1b`.
- **Commits (made by ROOT):** `94e543a24` (A1), `e215c6007` (A2), `948e0bb99` (C, tests only), `130445db2` (the lane guard).
- **Uncommitted when these records were written:** this folder only, in the numerics worktree.
- **Platform:** Mac, arm64 (macOS 26.6.2), aarch64-apple-darwin, rustc 1.97.1. **T9 and the gate are Mac-only comparisons against Mac main.** Nothing was run on Linux or on hosted CI.
- **Placeholders:**
  - `<wt>`: the T3 worktrees root; scratch is `<wt>/scratch/i13`;
  - `<VENV>`: the DEC-025 venv;
  - `<scratch>`: the session scratchpad;
  - `<home>`: the user's home;
  - `T3/`: the T3 folder;
  - `P/` = `projects/chirality-piping/`.
- **Abbreviations:** PP = `P/core/product_physics/src/lib.rs` (or the crate); FK = `P/core/solver/frame_kernel`; SA = `P/core/solver/nonlinear_integration/src/structural_adapter.rs`; NI = `P/core/solver/nonlinear_integration`; SD = `P/core/solver/sparse_direct`; SP = `straight_pipe`; CB = `curved_bend`.
- **Line numbers** are at `130445db2`, unless marked `main:` (`e7d930d49`). SA and FK are unchanged by F1b, so their lines are the same on both.

---

## 1. Brief, basis, delegation and rulings

**Delegation mechanism.**
- I13 is a Type 2 TASK. ROOT (HELP_HUMAN) launched it directly, as a Claude Code background subagent of ROOT's session, through the Agent tool (a delegated-harness-native descendant under D-GOV-35).
- **Parentage:** ROOT. There is no T3 manager on the Mac. I13 delegated nothing.
- **Supplied basis:**
  - the brief `T3/TASK_BRIEFS/I13_F1B_IMPLEMENTATION.md` (sha256 `57cce4f6…`, committed at `28bb8dfa8`), read in the numerics worktree;
  - Root `AGENTS.md` (loaded by the host);
  - ROOT's messages at each checkpoint, with the rulings recorded in `ROOT_RULINGS_V1.md`.
- **Scopes:**
  - writes to F1b's write set in `<wt>/f1b`;
  - scratch at `<wt>/scratch/i13`;
  - targets `<wt>/f1b-target`, the gate targets `<wt>/gate-cand-full-target` and `<wt>/gate-cand2-full-target`, and scratch targets under `<wt>/scratch/i13`;
  - mutant copies under `<wt>/f1b-mut/<id>/`, deleted after each run;
  - these records, in the numerics worktree.
- **No Git writes and no index operations.** ROOT made every commit. My Git operations were reads: `log`, `rev-parse`, `show`, `diff` and `archive`. `git status` was run once; it can refresh the index stat cache (disclosed, §20).
- **Enforcement limits:** the write set, the no-Git rule and the host caps were kept by instruction. The host sandbox did not enforce them mechanically.
- **Returns:** one subagent handback to ROOT at each checkpoint: 0, A1, the A2 stop, A2, C, the pause, the lane guard, the gate, and D.

**Basis read.** In the brief's order:
- Root `AGENTS.md`, `agents/AGENT_TASK.md`, `_COMMON.md`, `I8R_K1_RESUME.md` ("The Mac host", "Platform calibration");
- the I7 brief with addendum 1;
- D1 `DESIGN.md` revision 5a.2 (`fb62ef4a…`; hash-pinned, not edited): §1, §2.1, §4.3–§4.4, §4.6, **§4.7 and §4.8 in full**, §4.10, §5 items 5a–8, §6, §7.1, §7.3, §7.5–§7.6, §9;
- `ROOT_SELECTION_DESIGNS.md`;
- the rulings named below;
- K1 RETURN §3, §9 and §12; K2b RETURN §3–§6, §9, §13.3, §14 and §15 with addenda 1–2; `K2B_MERGE/RECORD.md` "For F1b"; `REVIEW/K2B_REVIEW.md`; `F1A_MERGE/RECORD.md` and F1a RETURN §3 and §6; the K-D5 and K2a merge records; `PLATFORM_CALIBRATION_MAC/RECORD.md`;
- the code at `e7d930d49`.

**ROOT's rulings for this slice, in order:**

| Source | Rulings |
|---|---|
| The brief (`28bb8dfa8`) | Q1 (a): the loop is out, T5's. Q2 (a): W2 after the ordinary attempt and exact-block, linear only. Q3 (a): admission at b ≠ 0. Q4 (a): today's functions at b = 0. Q5 (a): R-b′'s fail-closed demotion. Q6 (a): per-case `NUMERICAL_INTEGRITY_UNRESOLVED`. Q7: the line, bounded by `NAMED`. Q8 (a)+(d): a provisional dense ceiling, the pattern evidence in both modes. Q9 (a): the dense view at n ≤ 256. Q10 (a): Scope §6 as expected outcomes; PHYS-R4 restated. Q11: pin and site-test edits under K1's conditions. Q12 (a): the b-rule as merged. Q13 (a): ROOT supplies Mac main's baseline. Q14: one slice, A split into A1 and A2. The stale design text is recorded as rulings 1–11. |
| Checkpoint 0 (`2612c3c86`) | The plan is approved. OQ1: c2, not c1. OQ2: option B. OQ3–OQ7 and OQ9–OQ15 approved. OQ8 approved as a declared write-set extension. OQ13 narrowed. The two templates are fixed. The C1 list is ruled at A2. The gate's envelope scope is to be stated. |
| The A2 stop (`2f48511b7`) | (a), tightened: the audit test's third assertion is exact. A new C1 outcome class. |
| A2 (`52ead31ed`) | (a) the admission order and (b) the unreachability derivation approved; (c) coexistence accepted, on condition it is written step by step and the reviewer checks it; (d)–(f) approved; the C1 list of 28 approved; the full-envelope probe variant is the method for every gate; RV11D-N2 on the large cases at the gate. |
| C (message) | C verified; the two survivor tests committed as `948e0bb99`; the gate on that head. |
| The pause and the heap-cap finding (`7a6231dfa`, `9fa4d1b59`) | Guard the DEC-050/053 lane in PP, with a derivation, tests, a mutant, and the suites, T9 and gate re-run. |
| The lane guard (message) | The new info code, the per-DOF array and the unguarded dense parity lane accepted; the derivation's framing confirmed. Run the gate on `130445db2`. |
| The gate (`74850700d`) | PASS accepted; D. |

---

## 2. Files and line counts (against `e7d930d49`, at `130445db2`)

| File (under `P/core/`) | Lines | +/− | sha256 (first 16) | What |
|---|---:|---|---|---|
| `product_physics/src/lib.rs` | 23,681 | +1,141 −164 | `b6c8d32de0853582` | Items 1–3 and 5 of the scope |
| `product_physics/src/source_recovery.rs` | 1,811 | +59 −1 | `af58eaf7a1810dc7` | `DENSE_SOURCE_DOF_LIMIT`, `solve_ordinary`, the OQ2 decline |
| `product_physics/src/formation_guard.rs` | 502 | +1 −1 | `408f4f38b17acbb7` | OQ8: `pub(crate) const NAMED` |
| `product_physics/src/f1b_tests.rs` (new) | 2,602 | +2,602 | `6dbeedc495d567d5` | 23 unit tests |
| `product_physics/tests/f1b_w2_runtime.rs` (new) | 1,046 | +1,046 | `4bea18e4b93123c7` | 11 product-level tests |
| `product_physics/tests/f1b_sparse_pattern_memory.rs` (new) | 165 | +165 | `ba3db09ffd4a640f` | C-SPARSE (own binary, counting allocator) |
| `product_physics/tests/f1b_guard_ceiling.rs` (new) | 163 | +163 | `0ee1466fabf0e409` | C-CEILING (own binary, capped allocator) |
| `product_physics/tests/k2a_formation_range_runtime.rs` | 639 | +144 −8 | `16283920c4ef9732` | Q10: the linear variants restated |
| `product_physics/tests/s11f_site_test.rs` | 1,576 | +23 −4 | `2318e730e5865705` | Q11 rows, t10b |
| `product_physics/src/f1a_tests.rs` | 664 | +3 −2 | `2ef7e6af8ef02d4a` | call sites: a trailing `None` |
| `product_physics/src/s11f_tests.rs` | 2,179 | +3 −2 | `0439a45fe32c933d` | call sites |
| `product_physics/src/source_receipt/load_state_join_tests.rs` | 1,016 | +1 −1 | `e0f09f66e00cc11b` | call site (OQ9) |
| `product_physics/src/source_receipt/load_state_tests.rs` | 522 | +2 −1 | `87251c94f487e298` | call site (OQ9) |
| `solver/nonlinear_integration/src/s11k_tests.rs` | 1,805 | +178 −4 | `d0fe3051d0719952` | the K2b pin's product half (Q11), and 1 test |

**Total:** 14 files, +5,531 −188. Unchanged, as ruled: every FK, SA, SD, SP and CB file; NI `lib.rs` (Q1); `source_receipt.rs` (Q9); `pressure_membrane_range.rs` (Q6's template keeps its `contains` assertion true).

**Formatting and warnings.**
- The new and changed test files are rustfmt-clean.
- `lib.rs` has 80 rustfmt deviation hunks, against main's 79. The extra one is A1's `mod f1b_tests` line splitting main's existing mod-order hunk; F1b's own code adds none (`_run_records/checkpoint_a2/rustfmt/`).
- The non-test build has 9 warnings: main's 10, less `not_attempted`, which W2 now uses (`checkpoint_a2/build_nontest.log`).

---

## 3. The scope items, with the design's words and the rulings

**Scope 1: the sparse wiring (§4.8 "Facade after T1 merges"; W3).** Done as the brief states it.
- **Per-basis assembly** (main: PP `:1826`, `:1957` and the ordinary route's `:1873` → `assemble_case_stiffness`) is now `assemble_basis_stiffness` (`:2816`). It calls FK's `assemble_sparse_stiffness(…, &SparseAssemblyOptions::new())` with the realized curved bends as `StiffnessBlock`s and the springs in today's order.
- **Formation errors.** `form_basis_stiffness` (`:2857`) defers K2a's `NumericalRange` on a linear invocation into `BasisStiffness::RangeDeferred`. Every other formation error, and a range error on a nonlinear invocation, keeps main's `solver_blocked`.
- **The rest of the wiring:**
  - the partition is `reduce_assembled_sparse_system` (`:3508`);
  - the legacy observation force reads `SparseStiffness::get` (`:3261`);
  - the finiteness scan reads the stored values (`:3455`);
  - the linear attempt runs on `SparseAssemblyEvidence` in both modes (Q8(d); `solve_preview_reduced_system`, `:5334`);
  - reactions at b = 0 are `SparseStiffness::reactions` (`restrained_reactions`, `:3297`).
- **Dense views exist only:**
  - in dense scrutiny: SA's own view inside the attempt, then the protected LU lane and the parity rows, built after the attempt (`:3829-3866`), all behind the guard;
  - for retained-source recovery, only at n ≤ 256 (`:3583`).
- **No automatic dense fallback.** Mode code 3 is unused, and `dense_fallback_message` stays `None`.
- **The loop is untouched** (Q1).

**Scope 2: the resource guard (§4.8, Q8).**
- 96 bytes per n² entry, counted in the code (§6.1); a provisional 6 GiB ceiling.
- It runs once per dense-scrutiny invocation, after basis 0's formation and before the case loop (`:2375`).
- The refusal is `SOLVER_SYSTEM_BLOCKED`, naming the estimate.
- **Sparse:** no ceiling on the main solve (Q8). By ROOT's later ruling, only the DEC-050/053 observation lane is guarded (§6.2).

**Scope 3: W2 in the product (§4.7; K2b §15, corrected).** Done as ruled (§7):
- engagement order (Q2);
- admission (Q3);
- publication through K2b's checked helpers (RV11-1, RV11D-1);
- the unscaled ledger (RV11-N4);
- the `range_scaling:` line (Q7);
- the refusal template (Q6, c2);
- R-b′'s fail-closed demotion (Q5 (a));
- the b-rule as merged (Q12).

**Scope 4: `source_recovery.rs`.** Done:
- `solve_ordinary` refuses scaled evidence with zero work (unreachable in the product, D3);
- the dense view at n ≤ 256 (Q9);
- the OQ2 decline.

**Scope 5: F1a's follow-ups.**
- N1: a comment at the unreachable `row=none` branch of `formation_check_evidence_line`.
- N2: disclosed in CHANGE_RECORD.

**Scope 6: the LEF, reach and PHYS-R4 expectations.** Every row was confirmed by a product run at A2 (§8). None was contradicted.

---

## 4. The checkpoint-0 positions, as ruled

The plan is `_run_records/checkpoint0/CHECKPOINT0_PLAN.md` (sha256 `ba90de63…`, 856 lines), approved at `2612c3c86`.

| OQ | Position | Ruling | As built |
|---|---|---|---|
| OQ1 | c1 composed from FK's public census | **c2** | b printed only where the outcome carries it; `none` at steps 2–3; omitted at step 4 and for non-range failures. No census replica in PP. |
| OQ2 | B: zero-work named decline | B | `range_formation_decline_without_attempt()` |
| OQ3 | the `solve_ordinary` wrapper; t10b anchor; lexer pin | approved | as proposed |
| OQ4 | a non-range failure at the chosen b keeps `append_integrity_failure`'s code | approved | `integrity_failure_code` shared |
| OQ5 | no DEC-050/053 lanes at b ≠ 0 | approved | the mode row's lane fields are `not_observed` |
| OQ6 | derived rows stay binary64 | approved, disclosed | a non-finite derived row keeps today's refusal (the new C1 class) |
| OQ7 | admission after the orchestrator | confirmed | as built |
| OQ8 | `pub(crate) const NAMED` | approved (declared extension) | visibility only; S11-G's tests unchanged |
| OQ9 | call-site-only edits in the `source_receipt` tests | confirmed | as built |
| OQ10 | leave the K-D5 pin unchanged | approved | unchanged |
| OQ11 | the site-table expectations | approved | §14 |
| OQ12 | guard input dimension², after basis 0 | approved | as built |
| OQ13 | refuse a zero nodal term at b ≠ 0 | **narrowed** | the authored value is not available at admission, so every exactly-zero nodal term (±0) is refused. Disclosed. |
| OQ14 | exact-profile models without operands admitted | approved | PHYS-R4 without pressure publishes |
| OQ15 | the receipt follows W2's verdict, and fails closed after R-b′ | approved | pinned by the mixed invocation |

The two templates, fixed at checkpoint 0:
- **The line (Q7):** `range_scaling: force_scale_exponent=<b>; basis=exact power-of-two[; <entry>]{0..NAMED}[; more=<k>]`. The entries are `record=…`, `subnormal=reaction@…`, `subnormal=spring_action@…` and `subnormal=end_action@<member>.<i|j>:<component>:relative_precision=<p>`.
- **The refusal (Q6):** `Load case <id>: <reason>; range_scaling: attempted; step1_trigger=<trigger:?>[<b field>]; global_dof_map=<…>; no structural rejection is bypassed by generic LU or output quantization`.

---

## 5. The case loop, as built

1. **Invocation, before the cases.**
   - `linear = built.nonlinear_supports.is_empty()` (`:2367`).
   - Basis 0 is formed through `form_basis_stiffness(.., linear)` (`:2368`).
   - The dense-scrutiny guard runs once (`:2375`).
   - Each further basis (`:2504`) and each 0.4.0 resolved case (`:2419`) is formed by the same call.
   - `basis_solve_states` holds a `BasisStiffness`, never a dense matrix.
2. **`solve_load_case` (`:3307`), unchanged up to the ledger and S11-G's load-row finding.** Then:
   1. `formed: Option<&SparseStiffness>` (`:3449`).
   2. The finiteness scan: the stored values, then the force (`:3455`; D6).
   3. The partition: `reduce_assembled_sparse_system` when formed (`:3508`). For a deferred basis, the ascending complement of the restrained DOFs (`:3521`; D6).
   4. The observation force, formed only when formed (`:3531`).
   5. The ordinary attempt: `solve_preview_reduced_system`, or `OrdinaryFailure::Formation` for a deferred basis (`:3544`).
   6. **Exact-block, unchanged where eligible** (`:3599`), except:
      - its dense `stiffness` is the view at n ≤ 256 (`:3583`);
      - the attempt goes through `solve_ordinary(…, ForceScale::UNSCALED)`;
      - a deferred basis is declined with zero work (`:3606`).
   7. **The outcome match** (`:3685`):
      - `Ok` → published;
      - `Structural` with a selection → main's info record;
      - the contact-seed arm → main's;
      - **the final `Err(failure)` arm** (`:3714`): if `ordinary_range_trigger(&failure)` is `Some` and the invocation is linear, W2 runs (`force_scaling_attempt`). It publishes, or it refuses the case with `append_force_scaling_refusal` and returns empty. Otherwise it is main's `append_integrity_failure`, and the unreachable `(None, Formation)` returns main's `solver_blocked` error.
   8. **The receipt's `OrdinaryAttempt`** (`:3798`) is formed after the match, by one `passed(` call (t10b). It follows the published verdict: for a W2-published case, W2's report.
   9. **Publication:**
      - displacements unchanged;
      - the mode row with its lane fields;
      - dense scrutiny's protected lanes, only at b = 0 and only when formed (`:3829`);
      - reactions: W2's checked values at b ≠ 0, `restrained_reactions` at b = 0 (`:4041`);
      - spring actions: W2's checked values at b ≠ 0 (`:4083`);
      - member end actions: W2's checked values at b ≠ 0, SP's recovery at b = 0 (`:4236`);
      - every other row: today's binary64 function of these values (OQ6).

---

## 6. The resource guards

### 6.1 Dense scrutiny (Q8)

**The constant, 96 bytes per n² entry**, is counted at the dense path's peak. The peak is inside SA's DenseScrutiny branch, in FK's `prepare_bound` → `audit_contributions`, where six n²-sized buffers are alive together:

| Buffer | Bytes per entry |
|---|---:|
| The dense K view | 8 |
| Symmetry allowances | 8 |
| Symmetry operation counts (`usize`) | 8 |
| The prepared matrix | 8 |
| Contribution sums (`Expansion`) | 32 |
| Contribution differences (`Expansion`) | 32 |
| **Total** | **96** |

- The derivation, site by site, is plan §6.1. The later phases are smaller (72).
- PP's own dense-mode buffers come after the attempt (24).
- The dense `AssemblyEvidence` fallback also reaches 96.

**The formula and ceiling.**
- estimate = 96 · dimension², with dimension = 6 · nodes. dimension² is `SparseStorageCounts::dense_entries` by definition; a test pins the equality.
- `DENSE_SCRUTINY_CEILING_BYTES` = 6 GiB = 6,442,450,944 (provisional: the gate's heap cap on this Mac).
- So the guard admits at most 8,192 DOFs: 1,365 nodes, or 1,364 chain members.
- A 1,000-member chain is estimated at 3,462,915,456 B and runs. 1,365 members is estimated at 6,448,743,936 B and is refused.
- The refusal message: `dense scrutiny resource guard: estimated dense-path peak <e> bytes (96 bytes x <n²> dense entries, <n> global DOFs squared) exceeds the provisional ceiling <c> bytes; the model is refused before any n^2 allocation. …` (`dense_scrutiny_refusal_diagnostic`, `:2933`).

**The test hook.** `#[cfg(test)]` builds read a thread-local override (`:2925`). Non-test builds read the constant.

### 6.2 The DEC-050/053 observation lane (ROOT, `9fa4d1b59`)

- **The constant, 24 bytes per profile entry.** `SymmetricProfileMatrix::from_entries_with_order` grows its `values` vector of f64 by `resize`. At its last amortized growth, the old capacity (fewer than P entries) and the new capacity (at most 2P) are alive together. That is at most 3P · 8 = 24P bytes for P final entries. Not counted: the RCM-ordered profile, its factor, and the lane's O(n) and O(nnz) vectors.
- **The estimate: `observation_lane_profile` (`:2966`).**
  - It works from the lane's own entry system in O(nnz).
  - It skips zero-valued entries, as the lane does.
  - It returns the profile's entry count and maximum half-bandwidth.
  - **Accepted deviation from "no allocation":** one first-column index per reduced DOF (8 bytes per DOF; 0.36 MB for CONT n10000). The lane's entries arrive element by element and unsorted, and the global pattern cannot reproduce which entries the lane skips as zero.
- **The guard: `observation_lane_guard` (`:3007`).**
  - It uses the dense guard's ceiling function, including the test hook.
  - It runs in `solve_preview_reduced_system` before `solve_symmetric_system_from_entries`.
- **When it refuses:**
  - the mode row's seven lane fields are published as `not_observed`;
  - `sparse_entry_count` stays observed;
  - one info diagnostic is published: `diagnostic:sparse-observation:<case>:resource-guard`, code `SPARSE_OBSERVATION_LANE_NOT_RUN`, refs `[case, "DEC-053"]`, with the count, half-bandwidth, estimate and ceiling in its message (`:3026`).
- **Where it can fire:** in sparse mode only (D13).

**CONT n10000 (dimension 45,000 reduced):**

| Case | Profile entries | Half-bandwidth | Estimated bytes | Lane |
|---|---:|---:|---:|---|
| AX | 562,627,485 | 30,003 | 13,503,059,640 | not run |
| ROT | 675,174,982 | 30,005 | 16,204,199,568 | not run |

- The pause state's 675,179,982 (quoted in ROOT's `9fa4d1b59`) was a scratch pattern count that did not skip the zero-valued entries the lane skips, and gave AX = ROT. The guard's estimate equals the lane's own count on every basis where the lane can be run (§10.4).
- CHAIN and TREE n10000 need 11.5–17.9 MB, and RF-MECH-LINE-IN-CHAIN1000 needs 1.2 MB, so the lane runs for them.

---

## 7. W2 in the product (A2)

**The classifier: `ordinary_range_trigger` (`:1331`).**
- Two arms: `Formation(NumericalRange)` becomes `RangeTrigger::Formation`, and `Structural(Range(_))` becomes `RangeTrigger::Evaluation`.
- It equals the orchestrator's step 1 (D1), pinned by D-CLASS on 18 non-range and 14 range pairs in both modes.

**The attempt: `force_scaling_attempt` (`:1399`).**
- It builds the `ForceScalingCase` exactly as the ordinary route formed the case at b = 0: the same frames, users, curved slots and sources, springs, the case's unscaled ledger `force`, `prescribed` and mode; `selected: true`; `representation: Pattern`.
- It calls `solve_with_force_scaling`. An `Ok` at b = 0 is `NotEngaged`, a defensive arm that is unreachable by D1.
- It then runs admission, then publication, **before any row is built**.

**Admission: `force_scaling_admission` (`:1521`; Q3, OQ7, OQ13 narrowed).** Nine ordered checks. The first that fails names the family.

| # | Check | Family token | Reached in the product? |
|---|---|---|---|
| 1 | a user-stiffness element | `user_stiffness_element` | no (D11) |
| 2 | a realized curved bend | `curved_bend_macro_element` | yes (A2) |
| 3 | thermal or 0.4.0 eigen loads | `thermal_or_eigen_load` | yes (A2) |
| 4 | pressure-thrust loads | `pressure_thrust_load` | **yes (D; see D11)** |
| 5 | exact-pressure operands | `exact_pressure_operand` | yes (A2) |
| 6 | element-uniform loads | `uniform_element_load` | yes (A2) |
| 7 | a consumed constant-effort support | `constant_effort_support` | yes (A2) |
| 8 | a ledger term that is not an authored nodal `Term` | `non_nodal_load_term` | no (D11) |
| 9 | an exactly-zero nodal `Term` | `zero_nodal_load_term` | yes (A2) |

- Checks 3–5 come before 6, a change from the plan (approved (a)). Every element-targeted primitive is an `element_uniform_loads` entry, so in the plan's order a thermal load was named `uniform_element_load`. Only the names changed; the set of refused cases is the same.

**Publication: `force_scaled_publication` (`:1583`).**
- Reactions: `outcome.stiffness.force_scaled_reactions(u, force, scale, &restrained_sorted)`.
- Spring actions: `force_scaled_spring_action((dof, k), u, scale)`.
- End actions: `pipe.frame_element()?.force_scaled_end_actions(u, scale)`.
- A `PublicationOutsideBinary64` refusal becomes `Publication { quantity, b }`. The quantity is `reaction@<dof>`, `spring_action@<dof>` or `end_actions@<member>`.
- Everything else PP publishes at b ≠ 0 is today's binary64 function of these values and of u (plan §3.4, OQ6).

**The unscaled ledger (RV11-N4).**
- PP never forms a scaled ledger. The NI pin keeps `force_scaled(` at zero occurrences in PP (`F1B_PRODUCT_NEVER`), and the `force` passed is `finish_case_ledger`'s binding.
- The behaviour is tested: `f1b_rv11_n4_a_pre_scaled_ledger_is_detected_by_the_values` shows that a twice-scaled force moves u by 2^b.

**The line and the refusal:** `range_scaling_evidence_line` (`:1670`) and `append_force_scaling_refusal` (`:1766`), exactly the checkpoint-0 templates.
- The line is appended after F1a's `formation_check:` line through `append_integrity_report`'s new last parameter.
- The composition is `<base>[ <S11-G load-row>][ <formation_check>][ <range_scaling>][ <R-b′>]`.

**RV11D2-N1 (informational):** the helpers form the scaled product (one rounding), then unscale once. So a subnormal outcome carries two roundings, within its stated precision. M8's tie (§13.3) is that effect.

---

## 8. The product runs (A2, `e215c6007`; `_run_records/checkpoint_a2/`)

**Tools:** P1's probe with the heap cap, plus the full-envelope variant (`main.rs` `cd1052f7…`; §9.1), built twice: base `git archive e7d930d49`, candidate the A2 sources. P1's `run_one`, at most 4 workers.

**RF-RANGE (32 cases, 128 runs):**
- 100 runs are full-envelope identical.
- 28 runs changed: exactly the proposed and ruled C1 list (§9.3).
- The base's `run.envelope` spans are byte-identical to G1's for all 128.
- Tables: `rf_range_table.txt` and `rf_range_table_final.json`.

**Scope §6 rows, all confirmed:**

| Row | Result |
|---|---|
| LEF-small, both entries | Refused at model build, `PIPE_ELEMENT_INPUT_INVALID`, byte-identical |
| LEF-large, captured | Refused at capture (`CHECKED-JSON-UNSAFE-INTEGRAL-FLOAT`), byte-identical |
| LEF-large, typed (D8) | **Published at b = −702.** Every displacement, rotation, support component and member end-action row equals its RF base case's times the stated power of two, **bit for bit** (198, 66 and 402 rows per run for CHAIN, SKEW, CONT). Dense mode has one row fewer: the parity row (OQ5). Standing: CHAIN and SKEW Sensitive from their reports; CONT Sensitive by R-b′ (`lef_large_check.out`) |
| reach_zero, reach_lef | Published at b = 734, within 1e-9 of `0x3fc001034445ee29` and `0x3fd7a6bdbbfabce5` m; Sensitive by R-b′, with the line before R-b′'s text |
| spring-carried | `ScaledEvaluation`, trigger `Formation(NumericalRange { name: "GJ/L: G*J" })`, no b (c2). The forced probe finds no solving b (the rule's b is 548) |
| partial underflow | Published at b = 898, within 1e-9 of `0x3f63671db398fdda` m, with ruling B's record outcomes rendered; identical to K2b's kernel case |
| `open_gap` (nonlinear) variants | Byte-identical to main |
| PHYS-R4 with pressure | `range: subnormal stiffness or load at formation` (`SubnormalAtFormation`), trigger `Evaluation(Range("arithmetic outside normal range"))`: the census refuses the subnormal end-cap operand (Q10's condition confirmed) |
| PHYS-R4 without pressure | Published at b = 536, `CHECKS_PASSED`, within 1e-9 of exact references |
| The limitation chain | `ScaledEvaluation` (typed); captured refuses at capture. A documented limitation, not fixed |

**The extra corpus (55 cases, 220 runs):** 104 runs are identical; the 116 changed runs were all range refusals on main (`extra_table.txt`). Admission b values:
- uniform 516, thermal 516, constant effort 514, exact pressure 536;
- curved 478 (dense) and 454 (sparse);
- zero term 734.
- F-A2 and F-S (overflow) are refused `at=reaction@N0:RZ` and `at=spring_action@N0:RZ`, b = −482.
- The subnormal derived section value is a census refusal; its b = 0 twin is identical.

**The b-rule (Q12).** 38 `ScaledEvaluation` refusals were probed at every even b in [−1100, 1100] (`obs_summary.out`). Only the limitation chain has a solving b: 408 or 410 through 580, Passed, the same displacements at every b and within about 3e-17 of exact. The rule's b, 312, is refused. No b solves SKEW-E+960 (rule −462), SKEW-F-960 (498), the coexistence candidates (518), or the curved and spring-carried cases.

**RV11D-N2 (Q4), the b = 0 availability cost:**
- The checked helpers were run at `UNSCALED` against every ordinary-route b = 0 publication in 766 runs (RF-RANGE, the extras, every gate case under 1,000 members, typed).
- They covered 3,666 reactions, 251 spring actions and 3,456 end-action sets: **0 refused, 0 bit mismatches.**
- The large cases were observed at the gates (§9.5).

**The new C1 class's cause:** `displacement_magnitude` rows overflow to inf: 5 nodes per run in CONT-E-1000 and 1 in SKEW-E-1000. The non-W2 F+960 typed runs on main fail the same way (`obs_summary.out`, non-finite callers).

---

## 9. The gates

### 9.1 Method (both gates)

- **Probe:** P1's `main.rs` plus the calibration's heap cap, with `full_envelope()`. That function emits `run.envelope_sha256` over `serde_json::to_vec(&MechanicsEnvelope)` and writes the bytes to `T3_FULL_ENVELOPE_PATH` (approved at `52ead31ed`). `main.rs` `cd1052f7…`, `Cargo.lock` `d7bdd546…`. Built `--release --offline` from a `git archive` of the candidate, in its own target.
- **Base:** G1's full-envelope re-run of main `e7d930d49`: `runs.jsonl` `9139140c80a6b5233ef67c01b8eaec0df85e7c4ed44df41249cbb73ff902a38d`; the binary for part 2 is `577b10d4…`.
- **Part 1:** G1's `gate_run_base_full.py`, unchanged. Phase A (the 24 runs at 10,000 members) ran 2 workers, alone; phase B at most 4 workers.
- **Comparison:** `compare_gate_f1b.py`, built on G1's `compare.py`:
  - C1 is the ruled 28 runs, C2 the 24 at 10,000 members, C3 every other run;
  - C3 compares outcome, ok, exit, the full-envelope sha256, the summary sha256, the error text, and the gate_check row;
  - `gate_check.py` (`8ad89fca…`) is unchanged.
- **Part 2:** `gate_part2.py`, the four dense 1,000-member runs, interleaved base then candidate, one at a time, at 1,800 s. At `130445db2`, with ROOT's load wait: when the 1-minute load exceeds 8 at a run's start, the driver waits until it drops below 6.
- **What P1's summary covers and omits** (A2 §7, the reason for the variant):
  - **Covers:** `producer`, `numerical_quality` and `status`; `standing`; parts of the receipt; every diagnostic's id, code, severity and message (messages cut at 600 bytes except `NUMERICAL_INTEGRITY*` and `SOURCE_BLOCK*`); `result_count`; 13 result kinds, with some fields.
  - **Omits:** `summary` and several envelope fields; diagnostics' `source` and `affected_refs`; every other result kind (magnitudes, stresses, the mode row, the parity and preview rows); most of the receipt.
  - So C3 byte identity is judged on the full hash.

### 9.2 Gate 1, candidate `948e0bb99`: FAIL (`_run_records/gate1_948e0bb99/`)

- **Part 1** (17:28:08Z–17:33:19Z; candidate `runs.jsonl` `95430026…`):
  - `gate_check` PASS: 764 evaluated, 332 trusted, 0 trusted breaches;
  - C3: 832 runs, 0 differences; C1: the ruled 28; nothing changed outside C1 and C2;
  - C2 dense: 12 of 12 are the guard's refusal;
  - C2 sparse: 8 named M03 refusals, and **4 aborts at the heap cap: RF-LARGE-CONT-n10000-AX and -ROT, both entries.**
  - **The comparison's verdict: `RESULT: FAIL`**, on the brief's condition "no C2 sparse run aborting at the heap cap".
- **The diagnosis** (`heap_cap_diagnosis/cont_ax_typed.err`, a backtrace probe on the capped allocator):
  - an 8,589,934,592-byte request, with 4,550,048,885 B in use;
  - the path: PP `solve_preview_reduced_system` → SD `solve_symmetric_system_from_entries` → `SymmetricProfileMatrix::from_entries_with_order` → `Vec<f64>::resize`, in identity order;
  - this is main's DEC-050/053 observation lane, unchanged by F1b. It builds the original-order profile only to count it.
  - On main, the same runs aborted earlier, at the dense K.
- **RV11D-N2** on the 13 cases of 1,000 or more members (observation build of `948e0bb99`):
  - the 6 n01000 cases: all equal (6 or 1,506 reactions and 1,000 end-action sets each);
  - not completed: the mechanism, and CHAIN and TREE n10000 (M03 refusals, nothing published at b = 0); CONT n10000 aborted as above.
- **Part 2:** interrupted by ROOT's pause at 18:50Z. One base run had completed (a timeout; void for strict interleaving). The interrupted candidate run is void (`PAUSE_STATE.md`).
- **How it was resolved:** ROOT ruled the lane guard (§6.2). It was implemented and committed as `130445db2`, and the gate was re-run in full. The re-run's part 1 differs from this one in exactly the 4 CONT sparse runs (880 of 884 identical in exit, ok, full sha256, summary sha256, error and timeout).

### 9.3 Gate 2, candidate `130445db2`: PASS (`_run_records/gate2_130445db2/`)

- **Candidate probe:** `f4535939c705d36dae33e62395d7b1dfe044d47b104c299f9a01d023b23b6a17`.
- **Part 1:** 884 runs, 21:57:40Z–22:03:19Z (339 s).
  - `runs.jsonl` sha256: candidate `30d99bf0102a2fd41246fc4767af11599a6b17abb38bdf20e101b93f248d0014`, base `9139140c…`. Both are kept uncommitted; they are indexed by `part1/envelope_sha256.tsv` and hashed in `uncommitted_sha256.txt`.
  - `gate_check`: **PASS.** 764 evaluated, 332 trusted (base 328; the 4 extra are the C1 THIN-B runs, now `numerically_eligible`), 0 trusted breach triples, 0 violations.
  - **C3:** 832 runs, 0 differences.
  - **C1:** 28 changed, all on the ruled list. Nothing changed outside C1 and C2.
  - **C2:** 0 sparse heap-cap aborts; 0 dense runs that are not the guard's refusal.
  - Memory: `memguard.log` has no new entry; the lowest `kern.memorystatus_level` was 93.

**The C1 table** (`c1_table.tsv`; b and standing from A2; every row's outcome, exit, blocking codes, standing and full sha256 are identical to gate 1's):

| Runs | Main | F1b | b |
|---|---|---|---|
| CHAIN-E-1000 ×4 | `NUMERICAL_INTEGRITY_UNRESOLVED` Range | published, Sensitive, `needs_recompute` | 540 |
| THIN-B ×4 | `NUMERICAL_INTEGRITY_UNRESOLVED` Range | published, `CHECKS_PASSED`, `numerically_eligible` | 536 |
| {CHAIN, SKEW, CONT}-LEF-large typed ×2 each | K2a `SOLVER_SYSTEM_BLOCKED` | published, Sensitive, `needs_recompute` | −702 |
| SKEW-F-960 ×4 | `NUMERICAL_INTEGRITY_UNRESOLVED` Range | W2 refusal, `ScaledEvaluation` (trigger dense `Range("product overflow or underflow")`, sparse `Range("arithmetic outside normal range")`) | — |
| SKEW-E+960 typed ×2 | `NUMERICAL_INTEGRITY_UNRESOLVED` Range | W2 refusal, `ScaledEvaluation`, trigger `Range("radix scaling loses normal range")` | — |
| CONT-E-1000 ×4 | `NUMERICAL_INTEGRITY_UNRESOLVED` Range | W2 published (record `CHECKS_PASSED`), then today's `SOLVER_SYSTEM_BLOCKED` "computed mechanics must be finite, got inf" (the new class) | 514 |
| SKEW-E-1000 ×4 | `NUMERICAL_INTEGRITY_UNRESOLVED` Range | W2 published (record Sensitive), then the same refusal (the new class) | 542 |

**The C2 table** (`c2_table.tsv`; every base run is `memory_refused`, exit −6):

| Runs | F1b |
|---|---|
| Dense, 12 of 12 | `refused_blocked`, exit 0, the guard's `SOLVER_SYSTEM_BLOCKED`, 0 rows; 0.12–0.62 s; peak RSS 182–524 MB |
| CHAIN and TREE n10000 sparse, 8 | `refused_blocked`, exit 0, `NUMERICAL_INTEGRITY_UNRESOLVED` (M03), 0 rows; 0.70–1.42 s |
| **CONT n10000 sparse, 4 (AX and ROT, both entries)** | **no longer abort.** Exit 0, `MECHANICS_SOLVED`, 620,016 rows, no blocking diagnostic; `numerical_quality` `sensitive` (`NUMERICAL_INTEGRITY_SENSITIVE`), standing `needs_recompute`; one `SPARSE_OBSERVATION_LANE_NOT_RUN` each (with §6.2's estimates); the mode row's lane fields `not_observed`; the captured runs also carry the usual `SOURCE_BLOCK_RECOVERY_UNAVAILABLE`; 9.1–10.6 s; peak RSS 5,178,327,040–5,553,553,408 B (4.82–5.17 GiB; `ru_maxrss` is in bytes on macOS) |

**Part 2** (`part2/{base,cand}_runs.jsonl`; minimum `kern.memorystatus_level` 92):

| Run (dense) | Base start (UTC) | Base: load at start → end | Candidate: load at start → end | Result |
|---|---|---|---|---|
| CHAIN-n01000-ROT captured | 22:04:52 | 5.87 → 8.62 (one other cargo job was running) | 5.94 → 4.28, after a 120 s load wait (8.62 at 22:34:52, 5.94 at 22:36:52) | both time out at 1,800 s (exit −9) |
| CHAIN-n01000-ROT typed | 23:06:52 | 4.28 → 6.11 | 6.11 → 5.69 | both time out |
| TREE-n01000-AX captured | 00:06:52 | 5.69 → 5.25 | 5.07 → 7.01 | both time out |
| TREE-n01000-AX typed | 01:06:53 | 7.01 → 4.23 | 4.23 → 5.98 | both time out |

- Walls were 1,800.03–1,800.06 s. There is no base/candidate mismatch, so no finding (ROOT links this to K6's predicted dense-path timeouts).

### 9.4 RV11D-N2 on the large cases (Q4, at the gates)

- **Gate 1 (observation build of `948e0bb99`):** the 6 n01000 cases are equal (§9.2).
- **Gate 2 (observation build of `130445db2`, the same `obs_patch.py` and `obs_block.rs`):** CONT n10000 AX and ROT, typed, sparse: 15,006 reactions and 10,000 end-action sets each, **all equal bit for bit**. The observation build's full envelopes are byte-identical to the gate candidate's.
- **In total:** 8 of the 13 cases of 1,000 or more members are observed, all equal. The other 5 (the mechanism case and the four CHAIN and TREE n10000 M03 refusals) publish nothing at b = 0.

---

## 10. Suites, T9 and the per-crate counts

### 10.1 A1 (`94e543a24`; `_run_records/checkpoint_a1/`)

- **The build:** clean; the 10 warnings are main's.
- **PP:** 534 passed (main's 525 plus 9), 1 failed (the known Mac `s11g_tests::t13_committed_fallback_uz_is_byte_identical`, at the same line), 1 ignored.
- **NI `s11k_tests`:** 13/13. The K-D5 and K2b pins, `s11f_site_test`, `formation_check_runtime`, `k2a_formation_range_runtime` and `pressure_membrane_range` all pass.
- **T9:** 112/112 byte-identical to `git archive e7d930d49`, and both equal `PLATFORM_CALIBRATION_MAC/t9/output_sha256_main_mac_native.txt`.
- **B1–B3 at unit level**, on the declared subset (the K-D5 elbow; the invented preview model with realized curved bends, a user element and nonlinear supports; the preview-physics model; dec092 with three bases; two 0.4.0 prescribed-motion requests; a skew chain):
  - every K entry, the dense view, and `dense_entries == n²`;
  - the free DOFs and the reduced force;
  - the E12 reactions, including the sign of a zero;
  - the observation force;
  - the pattern attempt against the dense attempt, `Debug`-equal in both modes.
- **B1–B3 corpus-wide** (a scratch observation build, never committed): every T9 request, every gate request of at most 1,000 members, and the extra corpus. **272 requests, 301 bases and 311 cases, all bit-identical.** The attempt check covered models up to 606 DOFs.
- **B4:** the candidate probe on 198 gate cases (792 runs, both entries and both modes, everything but RF-LARGE) matches G1's base in summary bytes, errors, exits and classes. The gate's C3 later checked all of them on the full hash.
- **B5:** dec092's three bases, each on its own values.
- **B6:** forward, reversed and permuted numberings agree within 1e-9, and each is byte-identical to Mac main (a scratch extra-corpus T9 run).
- **B7:** gap, one-way, friction and mechanism models are byte-identical to Mac main (extra corpus 16/16, and T9).
- **C:** §12.

### 10.2 A2 (`e215c6007`; `_run_records/checkpoint_a2/`)

- **39 manifests, `--no-fail-fast`, against `K2B_MERGE/dec025/suites.log`** (`suites_a2_vs_k2b_baseline.txt`):
  - 37 manifests have identical counts;
  - PP goes 525 → 556, and NI 120 → 121 (new tests only);
  - the only failures are the three known Mac platform tests (PP `t13`, headless ×2). Their failure blocks are identical to the Mac calibration logs of main, with thread ids normalized.
- **T9:** 112/112; the extra corpus 16/16.

### 10.3 The lane guard, before its commit (`948e0bb99` plus the three files; `_run_records/lane_guard/`)

- PP 561 passed, 1 failed (`t13`), 1 ignored; NI 121 of 121.
- T9 112/112, extra 16/16.
- The corpus check: 293 bases equal, 0 mismatches.
- The mutant `F1B-LANE-UNGUARDED` was killed. Its NONE control ran concurrently with it, on the pre-format bytes (disclosed).
- The T9 tree (`lane/trees/t9cand`) is byte-identical to `git archive 130445db2` except for `execution/` (`diff -rq`).

### 10.4 Checkpoint D: B and C re-run on `130445db2` as committed (`_run_records/d_130445db2/`)

All runs below are from clean `git archive 130445db2` trees with no overlay.

| Check | Result |
|---|---|
| NONE (FK, SD, NI and PP, `--no-fail-fast`, `-j 4`) | FK 249, SD 30, NI 121 passed; **PP 561 passed, 1 failed (`t13`, `s11g_tests.rs:1666`), 1 ignored** |
| `F1B-LANE-UNGUARDED` (the guard removed), after NONE, alone | **KILLED:** `f1b_tests::f1b_lowered_ceiling_skips_the_observation_lane_with_a_named_reason` at `f1b_tests.rs:1018` (the mode-row assertion). No abort |
| The estimate against the lane's profile (unit, 14 bases: B subset and D models) | `f1b_observation_lane_estimate_equals_the_lanes_profile` passes (in NONE) |
| The lowered-ceiling hook test | `f1b_lowered_ceiling_skips_the_observation_lane_with_a_named_reason` passes (in NONE). At a ceiling equal to the estimate, the envelope is byte-identical to the unguarded one; 1 byte below, the exact `not_observed` mode row and the exact diagnostic, with everything else unchanged, on both entries |
| The estimate against the lane's profile, corpus-wide (a scratch test on the archive) | **294 bases equal, 0 mismatches, 8 estimate-only (above 1,001 nodes).** The corpus is the whole product tree and the gate requests: a superset of the lane-guard run's, adding two desktop e2e fixtures. All 304 common rows are identical to that run |
| T9 (release; S11-K's harness `ec089c1d…`) | **112/112 identical** to A1's base outputs of `e7d930d49` and to the calibration hashes; extra corpus 16/16 |

---

## 11. Tests added (37; all pass at `130445db2`)

**`src/f1b_tests.rs` (23):**
- **B:** the sparse wiring is bit-identical on the declared subset; each modulus basis has its own values; relabelling and permutation.
- **C:** the guard below, at and above the ceiling; the lowered ceiling refuses dense scrutiny before any mechanics; no automatic dense fallback; the lane estimate equals the lane's profile; the lowered ceiling skips the lane with a named reason.
- **E and Q9:** the budget refusal precedes every stiffness read; `solve_ordinary` refuses force-scaled evidence with zero work; the range-formation decline is named and charges nothing.
- **A:** the b = 0 oracle (no range record is byte-identical to a verbatim copy of main's `append_integrity_report`).
- **D:** the line's exact text with `NAMED` and `more=`; its composition and the no-op rule; the refusal template for every failure; the non-range failure at b keeps its code; a non-range failure never engages W2 (C); D-CLASS; RV11-N4; admission names each family; publication refuses an underflowing reaction by name; an in-range reaction is the scaled exact sum rounded once; a subnormal spring action is the checked helper's value (C; M8's tie).

**`tests/f1b_w2_runtime.rs` (11, product level, both entries where reachable, both modes):**
- PHYS-R4 with pressure is refused by the census; without pressure it publishes at b = 536;
- the limitation chain is refused by name and not fixed;
- the LEF-large analogue scales by exact powers of two;
- admission refuses each reachable family by name, with b = 0 twins;
- publication refuses a value outside binary64 (F-A2, F-S);
- the line renders subnormal values and records;
- the non-range failure at the chosen b keeps its code;
- the subnormal derived section value is refused on the range path;
- the mixed captured invocation finalizes with a consistent receipt;
- exact-block does not select the searched coexistence candidates.

**Own test binaries:** `f1b_sparse_pattern_memory.rs` (C-SPARSE) and `f1b_guard_ceiling.rs` (C-CEILING).

**NI `s11k_tests.rs`:** `f1b_top_level_item_names_the_enclosing_item`.

**Edited existing tests (declared):**
- `k2a_formation_range_runtime.rs`'s linear variants (Q10): spring-carried is a W2 refusal (`ScaledEvaluation`, `GJ/L: G*J`, no b); partial underflow is published at 898; exact-zero and least-subnormal are published at 734. The nonlinear assertions, the main-side preconditions and the test names are unchanged.
- The approved assertion change: `lib.rs` `tests::audit_nonfinite_computed_mechanics_never_publishes_solved_rows`, third assertion only (ROOT `2f48511b7`). It now asserts the case's integrity record contains ` range_scaling: force_scale_exponent=-494; basis=exact power-of-two; `, that an `ELEMENT_FORCE_RECOVERY_FAILED` blocking diagnostic is present, and exactly one `SOLVER_SYSTEM_BLOCKED` (`diagnostic:physics:solver`, blocking, "computed mechanics must be finite, got inf"). Confirmed in both modes.
- t10b and the NI pin (§14).

---

## 12. Memory: the C tests, allocation counts, and the peak's attribution

**The C tests** (debug, in the default suite, not ignored):

| Test | Model | Result | Debug wall |
|---|---|---|---|
| C-DECISION (unit) | synthetic | 8,192 DOFs passes; 8,193 refused; 10,000 members refused without overflow | < 1 s |
| C-LOWERED (unit, hook) | 12 DOFs | at 96·143 B, refused on both entries with the exact message and no rows; at 96·144, solves | < 1 s |
| C-CEILING (own binary, 512 MiB cap) | 1,365 members, dense | refused on both entries, estimate 6,448,743,936 B named; peak increment 6.5 MB typed, 20.6 MB captured | 0.5 s |
| C-SPARSE (own binary, 6 GiB cap) | 4,000 members, sparse | peak increment 824,827,959 B typed, 827,680,747 B captured, under the 1 GiB bound; the dense view alone is 4,610,304,288 B (4.29 × the bound) | 23.3 s |

- **The C-SPARSE departure (approved at A1).** The approved size (1,000 members, 64 MiB) cannot show the gap. The sparse path's peak is linear, about 206 KB per member (206 MB at 1,000 members), against the dense view's 289 MB there. The dense view reaches 4 × a bound above the peak only from about 2,900 members.
- **The measurements** (`checkpoint_a1/c_sparse_scaling.txt`; deterministic counts, not a memory-growth claim):

  | Members | Typed peak | Per member |
  |---:|---:|---:|
  | 125 | 25,713,447 | 205,708 |
  | 250 | 51,451,701 | 205,807 |
  | 500 | 102,930,139 | 205,860 |
  | 1,000 | 205,888,078 | 205,888 |
  | 2,000 | 412,201,265 | 206,101 |
  | 3,000 | 633,832,808 | 211,278 |
  | 4,000 | 824,827,959 | 206,207 |

- **The C mutants:** M10 (sparse mode materializes the dense view) reaches the C-SPARSE assertion (about 4.72e9 B), a counted kill with no abort. M9a, M9b and M17 abort C-CEILING's binary at its cap; that is not counted, and each is killed separately at an assertion (§13).

**The attribution of the ~206 KB per member** (ROOT's A1 observation; for K6 and V-P, not a claim). A scratch test (`d_130445db2/peak_attribution/`, never committed) uses the same counting allocator. It measures the same axis-aligned chain, typed and sparse, on `git archive 130445db2`, with probes at the product's stages (`probe_patch.diff`, scratch only) and each solver stage measured alone:

| At 1,000 members (6,006 DOFs) | Bytes | Per member |
|---|---:|---:|
| The whole call's peak increment | 205,888,078 | 205.9 KB |
| The envelope it returns (58,016 result rows; 37.7 MB as JSON) | 47,152,473 retained | 47.2 KB |
| Model, built model and boundary | 0.54 MB | 0.5 KB |
| The basis's sparse K | 3.35 MB retained | 3.4 KB |
| The pattern evidence (`SparseAssemblyEvidence::new`) | 10.0 MB retained, 11.7 MB peak | 10–12 KB |
| The checked solve (M03, K-D5, skyline LDL) | 12.4 MB peak, 2.1 MB retained | 12.4 KB |
| The DEC-050/053 lane (entries, then its solve) | 0.85 MB + 2.0 MB peak | 2.9 KB |
| After the case loop (every `LoadCaseSolve` held, with its rows) | 48.4 MB held (running peak 51.4 MB) | |
| After preview physics | 65.8 MB held (running peak 86.2 MB) | |
| After the results-assembly loop | 157.5 MB held; **the 205.9 MB peak is reached inside this loop** | |

- **Holders of the peak.**
  - The solver side (K, evidence, M03 with K-D5 and the LDL, and the lane) is at most about 27 KB per member, about 13% of the peak.
  - The rest is result-row publication: 58 rows per member, each a `ResultItem` with strings and metadata. The case's rows are held in `LoadCaseSolve`; preview physics adds records.
  - The peak itself is in `run_linear_static_preview_captured_once`'s results-assembly loop (main: unchanged code). It holds, together:
    - the rows moved into `results`, with `Vec` growth transients;
    - a `result.clone()` of every row in `rows_by_base_id` (for combinations);
    - each case's `id_map` of row ids;
    - the not-yet-consumed case rows.
- **This is not an F1b change.** F1b removed the dense K and dense reduction main held. It is an observation for K6 and V-P, and nothing was optimized. At 250 members the same stages scale linearly (`attr_final.txt`).

---

## 13. Mutations (checkpoint C at `e215c6007`; the lane mutant at `130445db2`; `_run_records/mutations/`, `lane_guard/mutant/`, `d_130445db2/lane_mutant/`)

### 13.1 Method

- Each mutant ran on a clean `git archive` (without `execution/`) with its own target under `<wt>/f1b-mut/<id>/`. Both were deleted after the run.
- NONE ran first and alone; then at most 2 mutants at once (ROOT's C ruling), at `-j 4`, `RUST_TEST_THREADS=4`, `--no-fail-fast`.
- NI and PP ran in full for every mutant; FK and SD too, for NONE and for any patch that touches them.
- **The kill rule:** a test that fails in the mutant and not in NONE. NONE fails only `t13`. An abort is never a kill.
- 51 runs, mean 2.0 minutes, max 5.8.
- The patchers: K1's and K2b's are reused unchanged; F1b's are exact one-match replacements, all checked to apply and compile.

### 13.2 The table: 50 mutants, all killed

**F1b's 32:**

| Mutant | Change | Killed by |
|---|---|---|
| DES8 | a spring omitted from the basis's pattern assembly | the B1 bitwise test, and 89 others |
| DES9 | publish without unscaling by 2^-b | the LEF-large analogue's bit relation; the mixed invocation; the audit test |
| DES10 | frames assembled in reverse order | `f1b_w2_non_range_failure_at_the_chosen_b_keeps_its_code` (note a) |
| DES11 | one modulus basis reused for every case | 14 tests: main's dec092 and modulus-basis product tests, and F1b's mixed invocation (note b) |
| LEAK-LINE | the line printed at b = 0 | the A-oracle, F1a's oracle, composition, mixed and admission tests |
| LEAK-W2 | W2 engaged for a non-range failure | first survived; killed on re-run by `f1b_a_non_range_failure_never_engages_w2` (`948e0bb99`) |
| M1 | W2 on a nonlinear invocation | the K2a `open_gap` variants (2) |
| M2 | W2 before exact-block | the coexistence test; the mixed invocation |
| M3 | the line omitted at b ≠ 0 | 7 tests, including PHYS-R4 without pressure and the K2a reach tests |
| M4 | a subnormal value without its precision | the line-text test; the subnormal line test |
| M5 | record outcomes not rendered | 4 tests, including both line tests |
| M6 | reactions via `reactions` on the scaled K | the F-A2/F-S test, the in-range exact-sum test, and 4 others |
| M7 | end actions via SP's b = 0 recovery | the K2a reach tests (`ELEMENT_FORCE_RECOVERY_FAILED`), 5 tests |
| M8 | an unchecked `-k·u` spring action at b ≠ 0 | first survived; killed on re-run by `f1b_a_subnormal_spring_action_is_the_checked_helpers_value` (`948e0bb99`; §13.3) |
| M9a | the guard call removed | C-LOWERED |
| M9b | the guard comparison inverted | C-DECISION, and 205 others |
| M10 | sparse mode materializes the dense view | the C-SPARSE assertion (about 4.72e9 B against 1 GiB; no abort) |
| M11 | the scaled-evidence refusal removed | E's `solve_ordinary` test |
| M12 | admission dropped | the unit and product admission tests |
| M13 | a pre-scaled ledger passed to the orchestrator | the NI K2b pin, and 9 value tests |
| M14 | the step-1 trigger dropped from the refusal | the template test; spring-carried; 9 others |
| M15 | a range formation still blocks linear invocations | the K2a reach tests and others (9) |
| M16 | the classifier admits a non-range error | D-CLASS |
| M17 | 96 → 32 bytes per entry | C-DECISION (2) |
| M18 | `>` → `>=` in the guard | C-DECISION; C-LOWERED |
| M19 | admission check 9 removed | the unit and product admission tests |
| M20 | the `NAMED` limit ignored | the line-text test (8 outcomes); the partial-underflow line test |
| M21 | the receipt's attempt taken from the b = 0 attempt | the mixed invocation |
| PIN-SECOND | a second orchestrator call in a new product item | the NI K2b pin (the declared-site table) |
| PIN-HELPER | a scaled helper named in `solve_load_case` | the NI K2b pin |
| PIN-NEVER | the product names `with_force_scale` | the NI K2b pin (`F1B_PRODUCT_NEVER`) |
| FOLD | a binary64 `+=` fold in every new product function | PP rule 8. Its message names all 11 (the 10 new `lib.rs` functions and `source_recovery::solve_ordinary`) |

**The original pins' 18** (K2b's `K2B-PIN-LOOP`, `-PLUMBING` and `-THIRD`; `KD5-M32a`, `-M32b` and `-E4`; `S11K-RV-OPT1`, `-OPT3`, `-OPT4` and `-PUB`; `K1-PIN-BINARY64` and `-B`; `K1-PIN-THIRD`; `K1-PIN-LOOP` and `-B`; `K1-S11F-KERNEL`; `K1-SITE-SPARSE`; `K1-SITE-FC`):
- all are killed at the same sites as K2b's checkpoint-C table, with lines mapped through `git diff -U0 70828d4d6 e215c6007`: **0 sites lost** (`kill_site_comparison.txt`);
- the added sites come only from PP binaries K2b did not run, and from the baseline `t13`.

**The lane guard's mutant:** `F1B-LANE-UNGUARDED` (the guard's `lane_refusal` set to `None`) is killed at the lowered-ceiling lane test's mode-row assertion (`f1b_tests.rs:1018`), both before the commit and re-run on `130445db2` (§10.4).

**Aborts, not counted:** in M9a, M9b and M17, the `f1b_guard_ceiling` binary aborted at its 512 MiB cap (SIGABRT). Each is killed at an assertion.

### 13.3 The survivors and the notes

- **(a) DES10.** B1's bitwise check does not see a reversed frame order on the declared subset. The observable effect is which member's K2a check fires first: the step-1 trigger becomes `EA/L: E*A` instead of `12EIy/L^3`. That is a real product assertion.
- **(b) DES11.** B5's unit test checks per-basis assembly, not the case loop. Main's modulus-basis product tests and the mixed invocation kill it.
- **LEAK-W2 (first round).** No test exercised a non-range failure on a linear, range-free case at product level with W2 visible. The new test shows a mechanism case publishing exactly `append_integrity_failure`'s diagnostic, with no `range_scaling`, on both entries and in both modes. It is not equivalent: the mutant engages W2 there.
- **M8, the tie derivation (not equivalent):**
  - **Where they match:** where `-k·u` is normal, the helper's value is bit-identical to it. The helper forms −(k·2^b)·u, which is exact scaling around one rounding, and unscales exactly.
  - **Case 1, underflow or overflow at scale:** the helper refuses first, in `force_scaled_publication`, before the mutated line is reached. So the mutant cannot differ.
  - **Case 2, a subnormal outcome:** the helper rounds twice (once at scale to 53 bits, then to the subnormal grid when unscaling). The unchecked `-k·u` rounds once, directly to the subnormal grid. The two differ only when the first rounding lands exactly on a tie of the second.
  - **The construction:** a kernel-level search (`mutations/m8_tie_search/search_test.rs.txt`) found such a tie on K2a's least-subnormal member, with spring k = `0x0410240000000000` and load 2^-1021 N at b = 734. The helper gives `…72aa`, the unchecked product `…72a9`, in both modes.
  - The test pins that the product publishes the helper's value for that spring's Fy support component, on both entries. This is RV11D2-N1's double rounding, within the stated precision.

---

## 14. Pins and site tests (Q11)

**PP `tests/s11f_site_test.rs`:**

| Change | Site removed or added |
|---|---|
| `run_linear_static_preview_captured_once` 4 → 2 | the two spring-diagonal `+=` (main `:1836`, `:1970`) moved into FK's sparse assembly |
| the row `("PP/lib.rs", "multiply_matrix_vector", 1, …)` removed | the function is deleted (no caller) |
| `restrained_reactions` 0 (E12): the disposition text only | the body delegates to `SparseStiffness::reactions` |
| the row `("PP/lib.rs", "observation_lane_profile", 1, "integer: …")` added | the lane guard's `.sum()` of profile entry counts (an integer) |
| a rule-1 assertion added | `solve_load_case` contains `reduce_assembled_sparse_system(` |
| t10b's anchor | `source_recovery::solve(` → `source_recovery::solve_ordinary(`, with the single argument ending `ForceScale::UNSCALED`, and no other non-test product caller of `source_recovery::solve(`. Its ordering assertions are unchanged |

- **Unchanged:** `assemble_case_stiffness` 1 and `add_curved_bend_stiffness_contributions` 1 (they keep `source_receipt.rs`'s n ≤ 256 replays); `legacy_observation_force` 1.
- No count rises except on the new, disposed row.
- The binary64-fold mutant in each new function is killed by rule 8 (FOLD).

**NI `s11k_tests.rs`:**
- The K-D5 pin is unchanged. PP still calls `.solve_assembled_with_formation_check(` exactly once, in `solve_preview_reduced_system`.
- The K2b pin keeps its name and its loop half. Its product half is now:
  - **`F1B_PRODUCT_SITES`:** 20 rows of (module, enclosing top-level item, token, count), for the `use` blocks, `force_scaling_attempt`, `force_scaled_publication`, `ForceScalingFailure`, `ForceScaledPublication` (struct and impl), `append_force_scaling_refusal`, `append_integrity_report`, `range_scaling_evidence_line`, `solve_load_case`, and `source_recovery.rs`'s `solve_ordinary` and `use`;
  - **`F1B_PRODUCT_NEVER`:** `force_scaled(`, `new_force_scaled`, `solve_force_scaled` and `with_force_scale` stay at zero in PP;
  - the new helper `top_level_item`, with its own test.
- The NI pin mutants (a second call site, a helper named outside its item, a `NEVER` token) are killed (§13).

---

## 15. The derivations

Each is written to be checked independently from the code. Where a step rests on observation rather than proof, it says so.

### D1. PP's range classification equals the orchestrator's step 1

1. **The orchestrator's step 1 is `evaluate_force_scaled(case, ForceScale::UNSCALED)`** (SA `:1722` → `:1606`).
2. **It forms K** by `assemble_sparse_stiffness(node_count, frames, users, blocks, springs, &SparseAssemblyOptions::new().with_force_scale(UNSCALED))` (SA `:1619-1626`).
   - With `UNSCALED`, `assemble_sparse_stiffness` takes the unscaled branch (FK `sparse.rs:601`), which is exactly what `SparseAssemblyOptions::new()` gives.
   - PP's `assemble_basis_stiffness` (`:2816`) calls the same function, with `new()` and the same arguments:
     - `built.frame_elements` and `built.user_stiffness_elements`, the same slices;
     - blocks from each curved element's `global_stiffness`, which PP's builder took from `element.global_stiffness()` of the same `CurvedBendMacroElement` (`:7141`), while the orchestrator's slots take `element.global_stiffness()` of the same macro element (NI `lib.rs:216-232`, `from_macro_element`), so the bits are equal;
     - springs as `(node_dof.global_index(), stiffness.value)` in the same order.
   - So K, or the first formation error, is the same. A K2a `NumericalRange` is classified by SA as `Range(Formation(e))` (`:1627-1632`) and by PP as `RangeTrigger::Formation(e)` (`:1331`, through `RangeDeferred`). Any other formation error is SA's `Failed`, and PP's blocking `Err` (main's `solver_blocked`).
3. **The evidence and the solve.**
   - SA builds `SparseAssemblyEvidence::new_force_scaled(pattern, …, UNSCALED)`, which returns `Self::new(…)` with the same arguments (SA `:519-521`).
   - It solves with `solve_force_scaled_with_formation_check(&k, force, &free, prescribed, mode, curved_sources, selected)`, which at `UNSCALED` uses `f` itself (`:838-845`) and `formation_source` (`:1446-1448`). SA documents it as `solve_assembled_with_formation_check`, unchanged (`:819-822`).
   - PP's `solve_preview_reduced_system` makes the same calls with the same inputs:
     - `free`, the ascending complement of the prescribed DOFs;
     - the same `prescribed` and mode;
     - the curved slots from the same macro elements;
     - `selected = built.nonlinear_supports.is_empty()`, which is true wherever W2 can engage;
     - the case's unscaled ledger.
   - The DEC-050/053 lane that follows in PP cannot fail the attempt (`.ok()`).
4. **The classification.** SA maps `Structural(Range(r))` to a range trigger (`:1688-1690`) and every other `Structural` to `Failed` (`:1697`). PP maps `Structural(Range(_))` to `Some`, and everything else to `None`.
5. **SA's third arm** (`Refused(ScaledEvaluation)` → range, `:1691-1696`) comes only from `AssembledForce::force_scaled` or a scaled formation. At `UNSCALED`, neither is called (steps 2–3). So it cannot occur at b = 0.
6. **The curved-evidence error:** PP maps a failing `from_macro_element` to `InvalidInput("curved formation evidence")` before the evidence. That is a non-range error on both sides.
7. **Conclusion:** for every linear case, PP's `Some(t)` holds iff the orchestrator's step 1 range-triggers, with the same `t`; and PP's `None` holds iff it returns the same `Ok` or the same non-range error.
- **Pinned** by D-CLASS (`f1b_range_classification_equals_the_orchestrators`: 18 non-range and 14 range pairs, both modes, `Debug`-equal outcomes). Killed mutant: M16.

### D2. Exact-block cannot select a formation-range case

- A formation trigger is K2a's `NumericalRange` at the basis assembly. Users and blocks are not K2a-checked, so it is a frame's `local_stiffness`.
- **On main,** the invocation is blocked at that assembly (`solver_blocked`) before any case runs. So exact-block never sees it.
- **On F1b,** the case reaches exact-block only through `RangeDeferred`, and OQ2's decline replaces the attempt (`:3606`).
- Even without the decline, `prepare_sources` builds `AssemblyEvidence::new(built.nodes.len(), &built.frame_elements, &[], &[], &springs)` (`source_recovery.rs:938-944`). That forms each frame's `local_stiffness()` again, deterministically, fails on the same member, and maps to `Exact(Arithmetic(…))` before any exact solve. So no selection is possible, and the decline loses nothing.

### D3. The scaled-evidence refusal is unreachable at product level

- `solve_ordinary` has exactly one non-test product caller, in `solve_load_case`'s exact-block branch (`:3615-3622`), whose third argument is the constant `ForceScale::UNSCALED`.
  - t10b pins this: the call's last argument is `ForceScale::UNSCALED`, and no other non-test product module calls `source_recovery::solve(`.
- So the `attempt_scale != UNSCALED` arm never executes in the product. It is a defensive guard, and its unit test kills M11.
- Independently, the only product source of a b ≠ 0 value is the orchestrator's outcome, and W2 runs after exact-block (Q2), in the outcome match (`:3685`).

### D4. Byte identity by construction wherever main publishes

1. **The sparse wiring is bit-identical** to main's dense path for K, the partition, the right-hand side, the observation force, E12's reactions and the attempt's `StructuralSolution` (K1 RETURN §9; B1–B3 on 311 cases; D6). The dense view at n ≤ 256 equals main's dense K, so exact-block, selection, replay and receipt run on main's inputs.
2. **Every changed control-flow path lands where main publishes nothing:**
   - **(a) `RangeDeferred`** occurs only for K2a's `NumericalRange` on a linear invocation. Main blocks the whole invocation there (`solver_blocked`).
   - **(b) The W2 arm** is the final `Err(failure)` arm with a range trigger on a linear invocation, reached only when exact-block did not select. Main pushes `append_integrity_failure` (blocking) there, and `has_blocking` makes the whole invocation a blocked envelope. So main publishes no mechanics for any invocation in which W2 engages, and its other cases' rows were not published either.
   - **(c) The OQ2 decline** occurs only under (a).
   - **(d) The dense guard's refusal** and **(e) the lane guard's `not_observed`:** the provisional classes below.
3. **On every other path,** main's code runs on bit-identical inputs:
   - the arms `Ok`, selected, contact-seed and non-range `Err` are main's;
   - `OrdinaryAttempt` is formed after the match, with main's arguments on main's paths (`:3798`);
   - `append_integrity_report` receives `None` for `range_scaling`, and the A-oracle shows the output is then byte-identical to a verbatim copy of main's;
   - the mode row, the lanes and the publication branches select main's code whenever `w2_publication` is `None`.
4. **Therefore** every invocation main publishes, on either entry and in either mode, receipts included, is byte-identical, **except** the two provisional classes:
   - dense scrutiny above 6 GiB estimated (main aborts at this Mac's heap cap, or needs more memory than that);
   - the lane's fields above 6 GiB estimated (D12: main needs more than the ceiling first).
- **Checked independently:** gate C3 (832 runs, full-envelope hash), T9 112/112, B4 and B6–B7.

### D5. The dense guard's constant and placement

- **The constant:** §6.1 (plan §6.1, site by site).
- **Placement:**
  - the guard runs at `:2375`, after basis 0's formation and before the case loop;
  - basis formation is FK's sparse assembly, O(nnz);
  - nothing n²-sized exists before the first case's attempt, whose first n² allocation is SA's dense view (`k.to_dense()` in the DenseScrutiny branch);
  - Q9's view and `source_receipt`'s replays are bounded by 256² entries;
  - so the refusal comes before any n² allocation.
- **Observed:** the gate's 12 dense 10,000-member runs are refused at peak RSS 182–524 MB, where a dense view alone would be 28.8 GB.

### D6. The deferred partition, and the finiteness scan's first entry

- **The partition.** `reduce_assembled_sparse_system` forms `free_dofs` as the ascending DOFs not in `restrained_dofs` (FK `sparse.rs:729`). The deferred branch forms the same list (`:3521-3528`). This is pinned on formed cases, by comparing the two in the B tests.
- **The first non-finite entry.**
  - Main scans the dense K row-major, then the force.
  - F1b scans `SparseStiffness::values()`, which holds the stored entries in row-major order, then the same force.
  - Every entry the dense K has and the sparse K does not store is +0.0, which is finite. So the first non-finite value in the dense scan is the first non-finite stored value, with the same bits. `require_finite_mechanics`' error carries only the value, so it is the same.
  - The deferred branch scans only the force, but main never reaches that state.

### D7. Admission soundness

- **The loads.** After checks 1–9 pass, every ledger term is a `Term` whose source is an authored node-targeted primitive (check 8), and it is nonzero (check 9). The census refuses a subnormal term (`SubnormalAtFormation`), so every term is normal: an exact input to the orchestrator's scaling (§4.7's premise).
- **The published values.** Every force-unit value published at b ≠ 0 is either a K2b checked helper's `PublishedValue` (reactions, spring actions, end actions, each unscaled once, refused rather than flushed) or an unchanged binary64 function of published values and u, the same function main applies at b = 0.
- Displacements are never scaled.
- So nothing is published at b ≠ 0 that main's formulas would not publish from the same values. A derived row that leaves binary64 keeps today's refusal (OQ6).

### D8. LEF-large's product b, and the powers of two

- The census span of the three LEF-large cases is [336, 1124]: e(G) = 336 and (12·E)·I = 1124 (`checkpoint0/exp/lef_large_census.py`). Loads and springs lie strictly inside it, so b does not depend on them. The window rule's even midpoint is b = −702, the same as K2b's kernel b.
- Binary64 arithmetic commutes with exact powers of two where no range event occurs. So each published displacement is the base case's times 2^(pf−pm−pl) (translations) or 2^(pf−pm−2pl) (rotations), with pl = 200, pm = 300, pf = 600. Forces are 2^pf and moments 2^(pf+pl) times the base's, bit for bit; otherwise a helper refuses by name.
- **Confirmed by product run:** 198, 66 and 402 rows per run, all bit-equal. The magnitude rows go through libm and are not pinned (the brief's rule).

### D9. Mutant equivalences

- None is claimed. The two first-round survivors were shown not equivalent and killed (§13.3).

### D10. Coexistence with exact-block (Q2; ROOT's condition at `52ead31ed`)

**The claim.** Exact-block does not select a case whose b = 0 ordinary attempt range-triggers.

**The claim the gate relies on** (proven by construction, step 5): wherever exact-block selects, F1b publishes main's bytes, whatever the trigger.

1. **When a selection is possible** (main, unchanged by F1b except Q9's view). All of these must hold:
   - the captured entry (`source_eligible`: a capture, no nonlinear support, no combinations);
   - `needs_source_recovery` (an attempt error, Sensitive, or a load-row finding);
   - `prepare_sources` succeeds: n ≤ 256; at most 16,384 sources; frames and springs only (a qualified passive family); `AssemblyEvidence::new` of the built frames succeeds; the folded stiffness equals the input stiffness bit for bit;
   - the exact context's preparation and solve succeed (`exact::Context::prepare_with_budget`, `solve_with_budget`);
   - the functional plan and the projections succeed;
   - the formation decline and the 0.4.0 replay reservation pass.
2. **Formation triggers: proven.** By D2, main never reaches exact-block for them, and F1b declines them with zero work.
3. **Large-magnitude evaluation triggers: proven not to reach the captured entry.**
   - `validate_checked_value` (`canonical_json/src/lib.rs:94-116`) refuses every integral float above 2^53 − 1 in the captured request. Every binary64 with |v| ≥ 2^53 is integral, so every captured numeric input has |v| < 2^53.
   - So LEF-large (loads 2^600) and every +960 or +240 RF-RANGE case are refused at capture (the gate: `refused_capture`).
   - What remains on the captured entry: inputs below 2^53 in magnitude, and member lengths above 1e-12 m (`AXIS_TOLERANCE`, else `DegenerateAxis`).
   - **Bound sketch, not a proof:** K entries and loads are products of a few such inputs and unit factors, which keeps them many binades below 2^1024. The overflow side of a range trigger is out of reach there; the triggers left are on the small-magnitude side.
4. **Small-magnitude evaluation triggers: observed, not proven.**
   - M03's range refusals at b = 0 come from `radix_scale` ("radix scaling loses normal range"), the checked products, quotients and values ("product overflow or underflow", "arithmetic outside normal range", …), and `exact_radix` ("exact radix loses represented bits") in its own audits.
   - Exact-block's exact solve builds expansions of the same K contributions and force terms. Its products go through `exact_boundary.rs`'s `mul` (`:116-136`) into `Expansion::add_product` (FK `structural.rs:705-727`), which splits each operand with `exact_radix` and refuses any exact part that is not representable.
   - Every small-magnitude construction tried on Mac main failed there, with `Exact(Arithmetic(Range("exact radix loses represented bits")))` at the stage "exact source solve", or earlier at source closure:
     - tip loads of 1e-310, 1e-305, 2.3e-308 and 2.5e-308 N;
     - the 2.5e-308 N load with a 1e12 N/m spring;
     - E ≈ 1.5e-290;
     - a tiny spring with a tiny load.
   - G1's gate base has 16 selections, and none follows a Range trigger.
   - No step shows in general that an M03 small-magnitude range event implies an exact-solve refusal. This step is empirical, as ROOT's acceptance records.
5. **Byte identity regardless: proven by construction.**
   - In F1b's outcome match (`:3685`), the arm `Err(OrdinaryFailure::Structural(error)) if selected_source.is_some()` (`:3689`) precedes the W2 arm (`:3714`). It is main's arm, with main's info record.
   - A formation trigger never has a selection (step 2).
   - So if exact-block selects any range-triggered case, F1b publishes it exactly as main does, receipt included, and W2 never runs (D4).
- **The test.** The brief's coexistence test ("a case main's exact-block recovers, published byte-identically") cannot be built by steps 2–4. The candidate side is pinned by `f1b_w2_exact_block_does_not_select_the_searched_coexistence_candidates`: every searched candidate shows the exact-block attempt's `SOURCE_BLOCK_RECOVERY_UNAVAILABLE`, then W2's outcome. F1B-M2 (W2 before exact-block) is killed by it and by the mixed invocation.

### D11. The admission checks that the product cannot reach, and one it can (correction)

- **Check 1, `user_stiffness_element`: unreachable.** `built.user_stiffness_elements` is non-empty only if the ordinary route realizes a user joint, and every realizable one is refused before the case loop:
  - with lateral stiffness ≠ 0 over a nonzero length, by M07's `JOINT_ELEMENT_EQUILIBRIUM_UNQUALIFIED` (blocking);
  - with lateral stiffness 0, as invalid input;
  - with coincident nodes, by pipe validation.
  - A2's product attempts (`A2-FAMILY-user_element-*`) were all refused before W2. ROOT's K5 ruling records the same fact: "the ordinary route realizes no user element".
- **Check 8, `non_nodal_load_term`: unreachable.**
  - `case_force_ledger` (`:3138`) has exactly six producers, in order: `push_nodal_loads`, `add_uniform_element_loads`, `add_pressure_thrust_loads`, `add_thermal_equivalent_loads`, `push_exact_pressure_operands`, `add_constant_effort_support_loads`.
  - Each of the last five pushes only for entries of the list that checks 6, 4, 3, 5 and 7 test (in that order of producers), and those checks come first.
  - `push_nodal_loads` pushes one `Term` per `nodal_loads` entry, sourced by its primitive id. A support-targeted primitive becomes an imposed displacement, never a term.
  - So once checks 1–7 pass, every term is an authored nodal `Term`, and check 8 cannot fire. It stays as the catch-all, pinned at unit level (`f1b_admission_names_each_family`).
- **Check 4, `pressure_thrust_load`: REACHABLE.** This corrects A2 (b).
  - `build_pressure_thrust_loads` makes one `PressureThrustLoad` per pressure-category element load.
  - On the legacy route, a *nonzero* pressure load is refused before the solve (`PRESSURE_MODEL_REAUTHOR_REQUIRED`), but a **zero-valued** one is not. On the exact route, every pressure primitive is refused (`EXACT_PRESSURE_REQUIRES_REGION`).
  - So a legacy model with a zero-valued pressure element load on a range-triggered linear case reaches check 4.
  - **Product run on `130445db2`** (`d_130445db2/admission_probe/`; A2's `A2-FAMILY-uniform-range` with its uniform load replaced by a 0 Pa pressure element load):
    - F1b refuses `range: family not admitted under force scaling: pressure_thrust_load; …; force_scale_exponent=516; basis=exact power-of-two`, on both entries and in both modes;
    - main refuses the same four runs, `NUMERICAL_INTEGRITY_UNRESOLVED` `Range("arithmetic outside normal range")`;
    - the b = 0 twin is byte-identical to main on both entries and in both modes (full-envelope sha256 `18ef3dae…` sparse, `7261827912b7…` dense).
  - So it is a named refusal on a case main refuses, the C1 class in kind. Nothing wrong is published, and no product change is needed. The unit pin exists; **a product-level pin is offered to ROOT** (tests only).

### D12. The lane guard: no case main publishes within the budget can reach it (ROOT's condition 1 at `9fa4d1b59`)

Let n be the global DOFs, n_f the free DOFs, P the lane's identity-order profile entries, and C the ceiling (6 GiB).

1. **On main, when the lane runs** (inside `solve_preview_reduced_system`, after a successful checked solve), `solve_load_case` still holds the basis's dense global K (8n² bytes; `basis_solve_states` holds it for every basis) and the dense reduced system (8n_f² bytes). The lane then holds its `values` vector: at least 8P bytes.
2. **P ≤ n_f(n_f + 1)/2,** so n_f² ≥ 2P − n_f. Since n ≥ n_f, main's live bytes are at least 8n² + 8n_f² + 8P ≥ 16n_f² + 8P ≥ 40P − 16n_f.
3. **The guard fires iff 24P > C.** Then main needs more than (40/24)·C − 16n_f = (5/3)·C − 16n_f bytes. That exceeds C whenever n_f < C/24 = 268,435,456.
4. **For n_f ≥ 268,435,456,** main's dense K alone is 8n² ≥ 5.7e17 bytes, which is far above C.
5. **So whenever the guard fires, main needs more than C bytes before its lane completes.** Within a heap budget of C (the gate's cap on this Mac), no case main publishes reaches the guard, and no such case sees a byte change.
6. **On an uncapped host,** main could publish such a case. CONT n10000 needs about 28.8 GB of dense K and 16.2 GB of dense reduced K first, about 45 GB. There, F1b changes only the mode row's seven lane fields, to `not_observed`, and adds the info diagnostic; every result row is unchanged. That is the lane's counterpart of Q8's dense class, and it is provisional like it (ROOT's framing, confirmed).

### D13. The dense-mode lanes are bounded by the dense guard (why the parity lane is not guarded)

- In dense scrutiny, the case loop runs only if 96n² ≤ C.
- **The sparse-entry lane:** 24P ≤ 24 · n_f(n_f + 1)/2 ≤ 12n² + 12n < 96n² ≤ C, for every n ≥ 1. So the lane guard cannot fire in dense scrutiny.
- **The protected dense lanes** (`legacy_dense_observation` and `append_sparse_live_path_evidence`, `:3829-3866`) run after the attempt. They hold at most the dense view (8n², dropped before the solve), the dense reduced system (8n_f²) and `solve_dense`'s working copy (8n_f²): at most 24n² ≤ C/4.
- So neither dense-mode lane can exceed the ceiling, and a guard there would be dead code (accepted by ROOT).

### D14. The new info code has no consumer with a closed code set (ROOT's request)

The code is `SPARSE_OBSERVATION_LANE_NOT_RUN`, severity `info`, id `diagnostic:sparse-observation:<case>:resource-guard`, refs `[case, "DEC-053"]`. Every consumer of envelope diagnostics was found by scanning `P/` for code literals (`_run_records/callers.txt`, plus a scan for `LOAD_CATEGORY_PREVIEW_MAPPED`, `SOURCE_BLOCK_RECOVERY_UNAVAILABLE` and `NUMERICAL_INTEGRITY_SENSITIVE`):

| Consumer | What it does with diagnostics | Effect of the new code |
|---|---|---|
| result_export `load_reference.rs` (`:555-620`) and its Python mirror `analysis_runs/load_reference_evidence.py` (`:408-416`) | filters by specific codes (`SOURCE_BLOCK_RECOVERY_SELECTED`, `LOAD_STATE_SOURCE_RECOVERY_NOT_JOINED`, `SOURCE_BLOCK_RECOVERY_UNAVAILABLE`; `load_reference.rs:37-42`) and specific ids, counting only those | ignored: a different code and id |
| desktop `loadReferenceEvidence.ts` (`:386-397`) | the same filters | ignored |
| desktop `sourceBlockRecovery.ts` (`:140`, `:167`, `:182-183`) | ≤ 16,384 diagnostics; every id non-empty and unique; the codes of the diagnostics referenced by a structural report or failure must be integrity codes | the id is unique per case and never referenced; source-block evidence needs a selection (n ≤ 256), where the lane guard cannot fire (below) |
| desktop `loadReferenceSourceEvidence.ts` (`:300-302`, `:537-539`) and result_export `source_blocks.rs` (`:269`, `:771`), Python `source_blocks.py` (`:115`, `:281`) | the same shape: referenced ids' codes, id uniqueness, the 16,384 bound | as above |
| desktop `previewPhysicsEvidence.ts` (`:14`, `:235`) | a **deny-list** of retired codes | not on it |
| desktop `HistoricalRunContext.tsx` (`:60`) | code is a string; severity ∈ {info, warning, error, blocking} | valid |
| desktop `nativeResultSave.ts` (`:53`) | an error code matches `^[A-Z_]+$` | valid (if it ever reached it) |
| `schemas/` | the `code` enums there are for other objects: `results*.schema.yaml` result-item issue codes, `source_block_recovery` and `physics_source_recovery` failure codes, the `load_reference_state` `source_recovery` object. None constrains `MechanicsEnvelope.diagnostics[].code` | none |

- **No consumer enumerates a closed set of envelope diagnostic codes.**
- **The lane guard never fires in an invocation with a receipt.** A receipt requires a selection, so n ≤ 256. Then P ≤ 256 · 257/2 = 32,896, and 24P ≤ 789,504 B, which is far below C. So `source_receipt.rs`'s 12× reservation (F1a N2) never meets this diagnostic.

---

## 16. The new info code and the diagnostics F1b adds

| Code | Severity | Where | When |
|---|---|---|---|
| `SOLVER_SYSTEM_BLOCKED` (existing code), id `diagnostic:physics:dense-scrutiny-resource-guard` | blocking | invocation | the dense guard refuses |
| `NUMERICAL_INTEGRITY_UNRESOLVED` (existing), or `integrity_failure_code(e)` for a non-range failure at b (OQ4); the integrity id | blocking | per case | a W2 refusal |
| **`SPARSE_OBSERVATION_LANE_NOT_RUN` (new)**, id `diagnostic:sparse-observation:<case>:resource-guard` | info | per case | the lane guard (sparse mode) |
| `SOURCE_BLOCK_RECOVERY_UNAVAILABLE` (existing), with the OQ2 decline's text | info | per case | a formation-range case on a source-eligible invocation |

- `SPARSE_OBSERVATION_LANE_NOT_RUN` is the only new code. It is ROOT-accepted as an info-level code (codes are not a registered vocabulary in `schemas/` or `docs/`). Its consumer check is D14.
- The brief's "no new diagnostic code" constraint (§4.7) stands for W2 and the dense guard. This code came from ROOT's later lane ruling.

---

## 17. Callers (`_run_records/callers.txt`; `scan_callers.sh.txt`)

- **Method:** a word-bounded lexer scan with `/usr/bin/grep -rnw` over `projects/` and `tools/` (`*.rs`, `*.ts`, `*.tsx`, `*.py`), with `target/`, `node_modules/`, `.venv/` and `execution/` excluded, on `git archive 130445db2`. Every occurrence is listed as file:line with its text.
- **Covered:** every existing product function F1b changed (`solve_load_case`, `solve_preview_reduced_system`, `run_linear_static_preview_captured_once`, `append_integrity_report`, `append_integrity_failure`, `restrained_reactions`, `legacy_observation_force`, `legacy_dense_observation`, `prepare_sources`); every new product item; `NAMED`; and the kernel entries F1b newly calls from the product.
- **Findings:**
  - Every changed function's callers are in PP, except `solve_load_case`'s two test callers in `src/source_receipt/` (OQ9) and `prepare_sources`' `source_recovery.rs` callers.
  - Every new item's references are in PP and its tests, plus NI `s11k_tests.rs` for the declared pin tokens.
  - The deleted PP `multiply_matrix_vector` has no product_physics occurrence except a doc comment in `f1b_tests.rs` and SP's row in `s11f_site_test.rs`. The other hits are other crates' own functions of that name.
  - The product is a new caller of `assemble_sparse_stiffness`, `reduce_assembled_sparse_system`, `SparseAssemblyEvidence`, `solve_with_force_scaling`, `force_scaled_reactions`, `force_scaled_end_actions` and `force_scaled_spring_action`, at the declared sites only.

---

## 18. F2a interface (exact signatures, at `130445db2`)

**The case loop's attempt order** (in `solve_load_case`, `:3307`):
1. The ordinary attempt (`:3544`).
2. Exact-block where eligible (`:3599`).
3. The outcome match (`:3685`): published; selected (main's info record); contact seed; **then the final `Err(failure)` arm**, where W2 engages for a range trigger on a linear invocation (`:3714`).
4. `OrdinaryAttempt` (`:3798`), after the match, with one `passed(` call.

**Where F2a attaches:**
- **F2a's W1 attempt** belongs in the outcome match, after the selected arm (`:3689`) and before or inside the final `Err` arm (`:3714`), as §4.4's coexistence rule requires. The W1-against-W2 order there is F2a's to rule.
- **The receipt** must keep following the published verdict: one `passed(` call (t10b), formed after every attempt.

```rust
enum BasisStiffness { Formed(SparseStiffness), RangeDeferred(FrameKernelError) }
fn assemble_basis_stiffness(built: &BuiltModel, springs: &[SpringEntry]) -> Result<SparseStiffness, FrameKernelError>;
fn form_basis_stiffness(built: &BuiltModel, springs: &[SpringEntry], linear: bool) -> Result<BasisStiffness, FrameKernelError>;

#[derive(Debug)]
enum OrdinaryFailure { Structural(StructuralError), Formation(FrameKernelError) }
fn ordinary_range_trigger(failure: &OrdinaryFailure) -> Option<RangeTrigger>;

#[derive(Debug, Clone)]
struct ForceScaledPublication {
    force_scale_exponent: i32,                         // b, never 0
    reactions: Vec<(usize, PublishedValue)>,           // restrained DOFs ascending
    spring_actions: Vec<PublishedValue>,               // per spring_entries entry
    end_actions: Vec<[PublishedValue; ELEMENT_DOF]>,   // per BuiltModel::pipes entry
    members: Vec<String>,
    spring_dofs: Vec<usize>,
    records: Vec<RecordOutcome>,                       // ForceScaledSolution::records
}
impl ForceScaledPublication { fn reaction_values(&self, dimension: usize) -> Vec<f64>; }

#[derive(Debug, Clone)]
enum ForceScalingFailure {
    Refused(ForceScalingRefusal),
    Failed(ForceScaledError),
    NotAdmitted { family: &'static str, b: i32 },
    Publication { quantity: String, b: i32 },
    NotEngaged,
}
#[allow(clippy::too_many_arguments)]
fn force_scaling_attempt(model: &PreviewModel, built: &BuiltModel, spring_entries: &[SpringEntry],
    restrained_dofs: &[usize], load_case: &PreviewLoadCase, load_application: &LoadApplication,
    thermal_loads: &[ThermalElementLoad], pressure_thrust_loads: &[PressureThrustLoad],
    exact_pressure: Option<&pressure_runtime::ExactPressureCase>, force: &AssembledForce,
    prescribed: &[(usize, f64)], solver_mode: PreviewSolverMode)
    -> Result<(PreviewLinearSolve, ForceScaledPublication), ForceScalingFailure>;
#[allow(clippy::too_many_arguments)]
fn force_scaling_admission(model: &PreviewModel, built: &BuiltModel, load_case: &PreviewLoadCase,
    load_application: &LoadApplication, thermal_loads: &[ThermalElementLoad],
    pressure_thrust_loads: &[PressureThrustLoad],
    exact_pressure: Option<&pressure_runtime::ExactPressureCase>, force: &AssembledForce, b: i32)
    -> Result<(), ForceScalingFailure>;
fn force_scaled_publication(model: &PreviewModel, built: &BuiltModel, spring_entries: &[SpringEntry],
    restrained_dofs: &[usize], outcome: &ForceScalingOutcome, force: &AssembledForce)
    -> Result<ForceScaledPublication, ForceScalingFailure>;
fn range_scaling_evidence_line(model: &PreviewModel, publication: &ForceScaledPublication) -> String;
fn append_force_scaling_refusal(diagnostics: &mut Vec<Diagnostic>, case_id: &str,
    failure: &ForceScalingFailure, trigger: &RangeTrigger, model: &PreviewModel);
fn integrity_failure_code(error: &StructuralError) -> &'static str;
fn append_integrity_report(diagnostics: &mut Vec<Diagnostic>, case_id: &str, report: &StructuralReport,
    model: &PreviewModel,
    equilibrium: Option<&open_pipe_stress_nonlinear_integration::product_equilibrium::ProductEquilibriumReport>,
    formation: Option<&formation_guard::FormationFinding>, formation_check: Option<&FormationCheck>,
    range_scaling: Option<&ForceScaledPublication>);

fn solve_load_case(model: &PreviewModel, built: &BuiltModel, materials: &[MaterialInput],
    stiffness: &BasisStiffness, restrained_dofs: &[usize], spring_entries: &[SpringEntry],
    load_case: &PreviewLoadCase, modulus_basis_record: Option<&str>, solver_mode: PreviewSolverMode,
    capture: Option<&source_receipt::CapturedInvocation>, source_budget: &mut SourceRecoveryBudget,
    load_state: Option<&case_state::resolve::ResolvedCase>, diagnostics: &mut Vec<Diagnostic>)
    -> Result<LoadCaseSolve, FrameKernelError>;
fn solve_preview_reduced_system(solver_mode: PreviewSolverMode, stiffness: &SparseStiffness,
    _reduced_force: &[f64], built: &BuiltModel, spring_entries: &[SpringEntry],
    global_force: &AssembledForce, observation_force: &[f64], prescribed: &[(usize, f64)],
    load_case: &PreviewLoadCase, diagnostics: &mut Vec<Diagnostic>)
    -> Result<PreviewLinearSolve, StructuralError>;
fn restrained_reactions(stiffness: &SparseStiffness, displacements: &[f64], force: &AssembledForce) -> Vec<f64>;
fn legacy_observation_force(force: &AssembledForce, stiffness: &SparseStiffness, restrained_dofs: &[usize],
    prescribed: &[(usize, f64)], coupled: bool) -> Vec<f64>;

// the guards (V-P revisits the constants)
const DENSE_SCRUTINY_BYTES_PER_ENTRY: u128 = 96;
const DENSE_SCRUTINY_CEILING_BYTES: u128 = 6 * 1024 * 1024 * 1024;
struct DenseScrutinyRefusal { dimension: usize, dense_entries: u128, estimated_bytes: u128, ceiling_bytes: u128 }
fn dense_scrutiny_estimate_bytes(dense_entries: u128) -> u128;
fn dense_scrutiny_guard(dimension: usize, ceiling_bytes: u128) -> Result<(), DenseScrutinyRefusal>;
fn dense_scrutiny_ceiling_bytes() -> u128;          // cfg(test): a thread-local override
fn dense_scrutiny_refusal_diagnostic(refusal: &DenseScrutinyRefusal) -> Diagnostic;
const SPARSE_OBSERVATION_BYTES_PER_PROFILE_ENTRY: u128 = 24;
fn observation_lane_profile(system: &ReducedSparseEntrySystem) -> (u128, usize);
struct ObservationLaneRefusal { profile_entries: u128, max_half_bandwidth: usize, estimated_bytes: u128, ceiling_bytes: u128 }
fn observation_lane_guard(system: &ReducedSparseEntrySystem, ceiling_bytes: u128) -> Result<(), ObservationLaneRefusal>;
fn observation_lane_refusal_diagnostic(load_case_id: &str, refusal: &ObservationLaneRefusal) -> Diagnostic;

// source_recovery.rs
pub(super) const DENSE_SOURCE_DOF_LIMIT: usize = 256;
pub(super) fn solve_ordinary(input: Input<'_>, limits: exact::Limits, attempt_scale: ForceScale)
    -> Result<SelectedSourceRecovery, RecoveryFailure>;
pub(super) fn range_formation_decline_without_attempt() -> RecoveryFailure;
// formation_guard.rs
pub(crate) const NAMED: usize = 6;
```

**The guard's input** is `dimension²`, equal to `SparseStorageCounts::dense_entries` by definition (pinned by a test). The guard runs before any assembly-dependent count exists, so it also covers a `RangeDeferred` basis 0.

**F1b's outcomes mapped to §4.3's trigger list.** "A `Range` error that W2 scaling could not resolve" is F1b's refusal set:
- `Refused(r)`, where `r.reason` is:
  - `SubnormalAtFormation` (step 2);
  - `InfeasibleWindow { e_min, e_max }` (step 3);
  - `ScaledEvaluation` (step 4; also the orchestrator's mapping of a scaled evaluation's own refusal);
- `Publication { quantity, b }` (step 5, `PublicationOutsideBinary64` at a published quantity);
- `NotAdmitted { family, b }` (Q3; nine families, §7);
- `Failed(Structural(e))`: a non-range failure at the chosen b, published under `integrity_failure_code(e)` (OQ4);
- `Failed(Formation(e))`: a census formation error;
- `NotEngaged` (defensive);
- **and the new class:** W2 published at b ≠ 0, then the invocation refused by today's derived-row non-finite check (`SOLVER_SYSTEM_BLOCKED`).

**The text and its composition order.**
- The line's and the refusal's templates are §4.
- The integrity message is composed `<base>[ <S11-G load-row>][ <formation_check>][ <range_scaling>][ <R-b′>]`, and R-b′'s sentence follows under S11-G's no-op rule.
- **F2a's `RETAINED_PRECISION_*` diagnostics must not be spliced into this message.** They should be their own diagnostic records, or else come after the `range_scaling:` line and before R-b′'s sentence, under the same no-op rule. At b = 0, the A-oracle must stay byte-identical.

**K2b types PP now uses** (for F2a's unification with K3's `Binary64Outcome`): `ForceScale`, `force_scaled_spring_action`, `ForceScaleReason`, `ForceScaledError`, `ForceScalingRefusal`, `PublishedValue`, `Representability`, `RangeTrigger`, `RecordOutcome`, `RecordRepresentability`, SA's `solve_with_force_scaling`, `ForceScalingCase`, `ForceScalingOutcome` and `EvidenceRepresentation`.

**The final pin lists F2a extends:**
- NI `F1B_PRODUCT_SITES` (20 rows) and `F1B_PRODUCT_NEVER` (4 tokens);
- PP `s11f_site_test.rs` rule 8, with the new row `observation_lane_profile` 1 and the changed rows (§14);
- t10b's `solve_ordinary` anchor.

---

## 19. Toolchain and host (`_run_records/toolchain.txt`)

- **Toolchain:**
  - arm64 (macOS 26.6.2), aarch64-apple-darwin;
  - rustc 1.97.1 (`8bab26f4f`), LLVM 22.1.6; cargo 1.97.1; rustfmt 1.9.0-stable;
  - Python 3.13.14 (the standard library only).
- **Cargo settings:** `RUSTUP_TOOLCHAIN=1.97.1`, `RUSTUP_AUTO_INSTALL=0`, `CARGO_INCREMENTAL=0`, `--offline` (`--locked` for tests); `-j 8` and `RUST_TEST_THREADS=4`; mutations at `-j 4`, at most two at once.
- **The host:**
  - the Mac is shared with I12 (K4), I14 (K5), I15 (K6), G1 and reviewers;
  - the memory guard `<wt>/guard/memguard.sh` ran throughout, and `memguard.log` has no KILLED line;
  - the gates' heavy phases were run in ROOT-scheduled slots.
- **No dense matrix was formed at 10,000 or more members.** Dense parity stops at 1,000 members.

---

## 20. Disclosures, deviations and open items

**Procedural disclosures:**
- `/tmp` was used twice, briefly, before the rule was restated. The files were deleted.
- At C, one short, warm-target run of a new test overlapped two mutant builds: three cargo jobs of mine for a short time.
- The lane mutant's NONE and mutant runs at `948e0bb99` plus the overlay ran concurrently, on the pre-format `lib.rs`. They were re-run properly (NONE first, alone) on `130445db2` at D.
- `git status --short` was run once. It can refresh the index stat cache.
- In gate 1's part 2, one base run completed before the pause. It was superseded by gate 2's strict interleaving.

**Deviations and approved departures:**
- C-SPARSE's size and bound (A1).
- The admission order (A2 (a)).
- OQ13 narrowed: every zero nodal term is refused.
- The lane estimate's per-DOF array.
- The dense parity lane is not guarded (D13).
- The new info code.
- The audit test's third assertion.

**Corrections in these records:**
- A2's claim that three admission checks are unreachable becomes two unreachable and `pressure_thrust_load` reachable (D11).
- The pause state's CONT n10000 profile count (675,179,982, from a pattern count) is superseded by the guard's estimates (562,627,485 AX; 675,174,982 ROT; §6.2).

**NOTE for T3's close list (pre-existing on main; no change in F1b):** the mode row's format string has a double space after `profile_pivot_residual_observation_basis=legacy_unscaled_DEC050_DEC053;` (main `lib.rs:4502`, `130445db2` `:5459`). It is published in every `linear_solver_mode_basis` row. Fixing it would change every published mode row, so it needs its own decision.

**Not done, or left to others:**
- The loop's move to the pattern (T5). Its dense base assembly in both modes is an M32 remainder.
- A scale-aware R-b′ bound (Q5(b)): F2a's input list and the T3-close list.
- PHYS-R4 with pressure (scaled exact-pressure formation): the T3-close list.
- W2 beyond nodal loads (Q3(b)): the T3-close list, if a real case needs it.
- The b-rule refinement: the T3-close list.
- b on the step-4 and non-range refusal paths (c2): waits for an SA change (K5 or F2a).
- The ceilings (Q8 and the lane guard): provisional, from K6's and V-P's measurements. The owner is told.
- **Not re-run on `130445db2`:** the 38 manifests other than PP (NI, FK and SD were re-run). C and the lane guard changed only PP files. PP's dependents (headless, result_export and others) were last run at A2. DEC-025's Mac sweep on the final head covers them.
- **Not run by I13 (ROOT's PR steps):** the independent complete-diff review (with an exact-rational solve of a subset of C1, an independent derivation of D4, and a re-run of the NI pins' mutants); hosted CI with the full-SHA dispatch; DEC-025; the src-tauri suite; native witnesses.

**Offered to ROOT:** a product-level test pinning `pressure_thrust_load` (tests only; D11).

---

## 21. Records (`_run_records/`; SHA256SUMS covers every file in this folder)

- **Sanitization:** every file was copied by `assemble_run_records.py.txt`, with machine paths replaced by placeholders. That is the only change to any raw log; a scan finds no machine path left.
- **Trailing whitespace:** raw tool output keeps its bytes, trailing whitespace included. 22 files have lines ending in whitespace; they are listed with their counts in `trailing_whitespace.txt` (mostly diffs, `MUTANTS.txt` lines, and `rf_range_base.out`).
- **Large files kept uncommitted:** `uncommitted_sha256.txt` lists the gates' `runs.jsonl` files (606 MB and 260 MB), the RV11D-N2 large runs, and the gate-2 scratch digest list (3,456 files, every full envelope included). The committed `envelope_sha256.tsv`, the C tables and the summaries index them.

| Folder | Contents |
|---|---|
| `checkpoint0/` | the approved plan; checkpoint-0 experiments |
| `checkpoint_a1/` | build, PP and NI logs; T9 (scripts, harness manifests, hashes, logs); B1–B3 corpus (the scratch probe block, results); B4 (`compare_b4_final.json`); the extra corpus and its generator; `c_sparse_scaling.txt`; rustfmt |
| `checkpoint_a2/` | build; the 39-manifest suites with the baseline comparison; T9; the product-run tables; `lef_large_check.out`; `obs_summary.out` (RV11D-N2, forced-even-b, non-finite callers); the observation build's diff; the probe variants; the A2 requests; scripts; rustfmt |
| `mutations/` | C's table, logs, scripts, kill sites and comparison; the re-run of the two survivors; the M8 tie search |
| `lane_guard/` | the lane-guard checks before the commit (PP, NI, T9, the corpus, the mutant) |
| `d_130445db2/` | D's re-runs on the committed head (NONE and the lane mutant, T9, the corpus check); the peak attribution; the admission probe (D11) |
| `gate1_948e0bb99/` | the failed gate: comparison, logs, gate_check, `envelope_sha256.tsv`, the heap-cap backtrace, `PAUSE_STATE.md`, RV11D-N2 on the large cases, the void part-2 base line, scripts |
| `gate2_130445db2/` | the passing gate: comparison, logs, gate_check, `envelope_sha256.tsv`, the C1 and C2 tables, part 2's records, RV11D-N2 on CONT n10000, scripts, the scratch digest list |
| top level | `callers.txt` and `scan_callers.sh.txt`, `toolchain.txt`, `uncommitted_sha256.txt`, `trailing_whitespace.txt`, `assemble_run_records.py.txt` |
