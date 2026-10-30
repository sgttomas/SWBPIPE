# Source route: a bind-only finalization replay

This design examines whether the source route's finalization (`pressure-source-1`) should replace its **full arithmetic replay** with a **bind-only replay**. It is the "Source-route replay design" unit of b7-plan's "After B7b" (as revised in PR #1498), commissioned by the owner on 2026-10-21 ("A accept your recommendations for the entire set."). Today, finalization rebuilds the selected case from the captured invocation and re-executes its whole exact evaluation on the case's own ledger, comparing every result with the retained proof. The bind-only replay would rebuild from the capture in the same way, then bind source, descriptors, limits, policy and ledger token without re-executing arithmetic. The full replay would remain as a maintained test-side check.

**Status: not adopted (A0).** The owner ruled on 2026-10-21 to keep the full replay ("14 confirm, 15A, 16 start now.", 15A being A0 with Q1 (b) and Q3 (b); Q2 does not arise). The design and the prototype measurements stay as the evidence for a later ruling under section 6's "When to revisit". No product file, fixture, reviewed input, ScopeOfWork or limit changes here. Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

**Revision 2** answers round 1 of the dual review (Claude Opus 5.5 and Codex, on `669b803bd8`). Both found section 2's argument sound, with corrections, and neither found a fault-free counterexample. The changes:
- **The recommendation** becomes "not now" (A0). The reviews showed three things:
  - REQ-003 (e) and AX-005 bear against removal on their plain readings;
  - the test-side replay would reach least where coverage is gained;
  - in the product configuration the gain is 24 runs and 16 invocations, not the R-A-off sets (Q0, Q1).
- **Q1** separates REQ-003 (e) from AX-005 and states reading (a) as a definitional ruling plus risk acceptance, which now explicitly includes F4. Q1 also addresses DEF-O and DEF-E's "independent Rust replay" sentence. Q2 quotes AC-002's "for R-A" scope.
- **A5** is shown to lose detection on cases selected today (both reviews).
- **Section 2.6** states the test-side replay's real reach and what adoption would add.
- **Measurements.**
  - The case and replay maxima include PEXACT.
  - The Z-hinge replay ranges are attributed correctly; a budget-truncated figure is marked as a lower bound.
  - The coverage table's phi2 row counts PEXACT-D.
  - F1's remaining product checks are named, and live-projection faults are added.
- **Other repairs.** Smaller repairs to L3, L4 and section 4's inventory.

**Revision 3** answers round 2, on `dc54f41289`. Both reviews confirmed every round-1 repair and found the A0 recommendation supported. The changes:
- **Q0** states A0's cost: the 24 runs and 16 invocations stay outside REQ-003 (c)'s ordinary route until the next unit, which can repair their publication without removing the replay. Its revisit condition gains the invocation-ledger trigger and no longer extrapolates work into problem size.
- **"A1 is ready"** becomes "designed and measured".
- **The "only product check" wording** is narrowed to re-execution of the arithmetic and coherent corruptions.
- **Q1** separates G2 from its G4 cost, and attributes DEF-O and DEF-E's "independent Rust replay" correctly, to the retained-precision route's validation-lane audit.
- **Section 7** leads with the A0 case.
- **Smaller repairs** to summary figures and labels.
- **Section 7's misleading-cause statement** follows this design's census, in which the cause does not recur on main (Opus review, round 3).

## Summary

- **What the full replay guarantees** (section 1).
  - At every selected case's finalization it checks two things. The capture rebuilds a source and plan bitwise equal to the retained proof's (G1). Re-executing the method on them reproduces every retained arithmetic object bit for bit (G2). It also checks coverage and policy (G3) and must fit the case ledger (G4).
  - No commitment requires producer replay. The readers never replay (REQ-005; PRD §12.8).
  - Design reliance is real and is listed: SF-1's screen, RF §2.6, §2.9 and RF-14, O4 §3.5's work law, R-A §2.2 and §3.1, and T25's pricing.
- **Soundness of bind-only** (section 2). Four invariants are read from the code:
  - one constructor;
  - no mutation after construction;
  - the arithmetic reads only what the snapshot records;
  - no hidden state in the product build.

  Given these, every arithmetic comparison of the full replay is implied by bind-only's checks. The two then differ only in `Budget` refusals (the coverage gained) and in faults outside the model (the detection given up). Check by check, bind-only keeps G1, G3 and the live binding (G5), and removes G2 and the budget exposure.
- **Detection given up** (section 2.4): per-invocation detection of transient faults in the live arithmetic or retained memory (F1), and of later-introduced hidden-state, construction or input-closure defects (F2 to F4). After adoption these are caught only test-side, on exercised inputs.
- **Measured** (section 3; scratch prototype, never merged).
  - Bind-only's replay stage costs 65,832–131,988, against 122,404–2,506,432 on the controls and 2.55M–3.23M on decimal Timoshenko Z-hinge inputs and, on PEXACT inputs, a complete replay of up to 4.06M (the shadow's; the case-ledger replay does not complete there today) (R-A on).
  - **The gain in the product configuration (R-A on)** is 24 phi2 PEXACT Timoshenko runs (12 documents: PEXACT-A r1e-08 and both PEXACT-B) and 16 of the 36 8–10-case invocations. These are every measured source-route case blocked at finalization today; the Z-hinge sweep already selects with R-A, and refusals the replay's saving cannot reach do not move (for example J7's source-closure `Budget` entries). A live-pass refusal in a later case of a many-case invocation can move, since earlier cases' savings free shared invocation ledger. For example, `zhinge122 (0.168, 0.007) long r=1e-12 eb decimal zero cases=10` selects nine cases under the full replay, with the tenth refused at `retention`, and all ten under bind-only (Codex review, round 3). With R-A off, bind-only also selects the 32 Z-hinge sweep blocks, 36 two-case blocks, 32 blocked many-case invocations and 48 phi2 blocks (including PEXACT-D), so R-A is not needed for coverage. No selection is lost. The largest case charge is 4.92M with R-A on and 5.56M with R-A off, both on PEXACT-B r1e-08 Timoshenko.
  - Every newly selected case matches the direct method bit for bit, passes the shadow full replay, both readers and the oracle.
  - Only work counters, `receipt_sha256` and, in J7, 4 envelope digests move.
  - The mutants confirm the cost: faults in the live arithmetic or the retained proof (M-RATIO, M-DISP, M-HIDDEN, M-WIT, M-RET) are refused by the full replay and **published** by bind-only. Both readers accept them, and the oracle catches only M-DISP on Z-hinge inputs.
- **Recommendation: not now (A0).** In the product configuration the gain is narrow: 24 runs on 12 invented φ = 2 documents, and 16 near-ledger invocations, all of them fail-closed today. The loss is per-invocation integrity detection on every invocation; the full replay is the only product check that re-executes the source arithmetic, and so the only one that refuses a bound corrupted coherently in the live arithmetic or in the retained Ratios. And the plain readings of REQ-003 (e) and AX-005 bear against removal (Q0, Q1). A1 is designed and measured for a later coverage need that R-A-class reductions cannot meet; its implementation, section 2.6's additions and review remain. Meanwhile the next unit can give today's blocked cases ordinary results. If the owner prefers the coverage now, Q1 (a), Q2 and Q3 (a) set out what adoption requires. The alternatives are compared in section 5.
- **What would move** at adoption (section 4):
  - one FF bind method, with the full replay made test-only;
  - the stage strings;
  - every pressure-source fixture's work and hashes, and 4 J7 envelope digests;
  - RF-14 and two fallback-test texts;
  - the U4b and O4 work-law terms;
  - T25's pricing and the D3 registration;
  - RF, O4, R-A and pricing design text.

  The 0.4 screen stays until the next unit decides it (section 7).

## 0. Basis and terms

- **Basis.** Main `fa0028a7d7` (#1497, R-A merged). Paths are relative to `projects/chirality-piping/`. PP is `core/product_physics/src/`, FK is `core/solver/frame_kernel/src/structural/`, FB is FK `exact_boundary.rs` and FF is FK `exact_boundary/functionals.rs`. Line numbers are on that main.
- **Prior work.** An Agent 0 study and its Codex scrutiny, neither in the repository, were taken as hypotheses. Section 2 answers Codex's blocking correction: a mutant inventory cannot establish that no relied-upon guarantee is lost, so the argument is structural, check by check, with the detection given up stated explicitly. The mutants (section 3.4) are supporting evidence only.
- **Measurements** come from a scratch prototype on branch `claude/piping-replay-proto-20261021`, which is never merged (section 3). Figures are tagged **[M]** measured there, **[D]** derived from the code, **[C]** from a committed design, or **[E]** estimated from measured figures by stated arithmetic.
- **Terms.**
  - *Live attempt*: `source_recovery::solve` (PP `source_recovery.rs:1852`), from source closure to retention, on one `AttemptBudget`.
  - *Retained proof*: the `RetainedFunctionalSet` it builds (FF:984), owned by `SelectedSourceRecovery` with the live ledger (PP `source_recovery.rs:186–198`).
  - *Captured rebuild*: finalization's re-derivation of the case's product input from the captured request `Value` (PP `source_receipt.rs:290–388` pre-0.4, 173–289 load/reference-state), followed by `prepare_sources` on it.
  - *Full replay*: `RetainedFunctionalSet::replay_against` (FF:1061–1158) on the captured rebuild.
  - *Binding check*: `check_binding_with_budget` (FF:1029–1059), which compares source and descriptors with no arithmetic.
  - *Bind-only replay*: the captured rebuild followed by every non-arithmetic check of the full replay (section 2.3), with no re-preparation, solve, evaluation or projection.

## 1. What the full replay guarantees, and who relies on it

### 1.1 Where it runs [D]

A selected case is finalized in `FinalizedSourceBlockCase::composite_exact` (PP `source_receipt/composite.rs:741`), called from the case solver (PP `lib.rs:6601`) on both the pre-0.4 and the 0.4 load/reference-state paths. In order, on the case's own ledger:

1. **Reservation.** 1024 + 64 × rows + `finalization_size` + supports × `RECIPE_WORK` (`composite.rs:757–761`).
2. **Captured rebuild and full replay.** `CapturedInvocation::check_input_with_physical` (`composite.rs:765`) parses the captured `Value` again, normalizes, resolves materials, builds, assembles, prepares loads and boundary, and calls `SelectedSourceRecovery::replay_against` (PP `source_recovery.rs:338–361`). That runs `prepare_sources` on the captured input ("replay source closure"), then FF `replay_against` ("expected invocation replay"), with `RELATIVE_LIMIT` and no contacts. A failure reads "captured source replay: …" (`source_receipt.rs:272`, 388).
3. **Binding check against the live input.** `check_binding_against` (`source_recovery.rs:317–335`, called at `composite.rs:773`) runs `prepare_sources` on the live input and `check_binding_with_budget`. A failure reads "composite current source: …".
4. **Row binding and publication.** `rows::expected_primary_for`, the derived checks, endpoint maxima and receipt hashing. Each published row is bound to its retained projection by value bits, relative limit and interval (`source_receipt/rows.rs:243–270`).

Any error in 2–4 makes the case push the blocking `SOURCE_BLOCK_RECOVERY_FINALIZATION_FAILED` (`lib.rs:6618–6631`). On 0.4 a failed join republishes the invocation ordinarily (SF-1R, `record_load_state_join_failure`). On pre-0.4 the envelope was the blocked envelope with no result rows (R-A design §1); since [pre04-failure-unification.md](pre04-failure-unification.md) both routes screen selection and republish ordinarily.

On the 0.4 path only, selection is screened by `reserve_captured_replay` (`source_recovery.rs:268–281`, `lib.rs:4991–5011`). The screen requires the live charge to be at most the limit less itself, because "captured replay repeats the live attempt's source closure and exact solve in the same ledger". The pre-0.4 path has no screen (RF §2.9; RF-14 `pre04_selection_takes_no_replay_screen`).

### 1.2 What it establishes [D]

The full replay (FF:1061–1158, with `retained_charge` FF:1171, `replay_leaves` FF:1214) establishes, for every selected case at finalization:

- **(G1) Capture correspondence.** The captured request, rebuilt through the product's own normalization and source closure, yields a source bitwise equal to the retained snapshot (FF:1102–1109, through `Snapshot::matches`, FB:1745) and descriptors equal to the retained plan (FF:1088). The limits, ledger token and relative-limit policy agree (FF:1075–1081).
- **(G2) Re-execution equality.** Executing the method again on that rebuilt source, on the remainder of the case ledger, reproduces every retained arithmetic object bit for bit:
  - the block partition and witnesses, including order 3–4 entries and minors (FF:1110–1118);
  - the displacement and reaction Ratios (FF:1119–1124);
  - the 2n DOF projections (`replay_leaves`);
  - every row Ratio (FF:1134–1140);
  - every row projection and its exact sign (FF:1141–1151).
- **(G3) Retained coverage and policy.** Every row has one value and one projection in index order. Every DOF slot is projected exactly once. Every projection is labelled with the expected relative limit. The contact list is the expected one (FF:1090–1095, 1142–1146, `replay_leaves`).
- **(G4) Affordability.** All of this fits the case's remaining ledger. That condition is what blocks the cases of section 3.

The binding check (step 3) adds **(G5)**: the live input, as finalization holds it, still equals the retained source and plan.

What the full replay does **not** establish: that the method's arithmetic is correct, that the descriptors are the right mechanical rows, or that the capture's normalization is faithful to the request. A deterministic defect in any of these reproduces identically in the replay. Those rest on the kernel's construction and independent oracles (RF §5.4, the declared-problem oracle, J7, the readers), not on the replay.

### 1.3 Commitments and design reliance

**Commitments** (ScopeOfWork, PRD, claims registry). None requires a producer full replay. Each bears on its removal as follows:

| Text | What it says | Bearing |
|---|---|---|
| PRD §12.8 (`docs/PRD.md:792`); REQ-005 | Separate consumers check the published statement; "They do not re-run the solve or replay its proof." | Readers never replay. Nothing in the reader path moves. |
| REQ-002 eligibility | "the solve captured its invocation"; "the per-case and per-invocation work limits" | Capture stays and is rebuilt. Limits stay. |
| REQ-003 (e); AX-005 | "No search, verification, tolerance or domain limit is relaxed to obtain a bound." "A gate is never loosened to obtain a pass." | **Owner question 1.** Is removing a per-invocation re-execution check to gain coverage a relaxation of verification? |
| AC-002 | "its charged work changes only as a reviewed work-reduction change requires, with every charge still at least the executed work; its envelope and diagnostics digests change only as a stated … work change requires" | **Owner question 2.** Its reading for a change that removes a check's work rather than reducing its arithmetic. |
| AX-006 | Numerical changes need targeted regression checks and independent review. | Applies to adoption. |
| REQ-003 (c) | A declined or failed attempt stays on its ordinary route, with the stage recorded. | Unchanged. The failure stage strings move (section 4). |

Neither `docs/claims_registry.md` nor the published schema (`schemas/physics_source_recovery.schema.json`) mentions a replay. The receipt's `work` fields are "known metered/reserved work". No reader recomputes or checks the replay's cost:
- the Rust reader (`core/reporting/result_export/src/source_blocks.rs:783–806`, 1072–1078) checks the case and invocation ledgers' structure and limits;
- the Python verifier (`tests/verifiers/source_blocks.py:246–255`, 325–326) does the same;
- the wasm reader is the same Rust code.

**Design reliance.** These are the reviewed rulings that rely on the full replay; each is reconciled in section 4.

| Where | What it relies on |
|---|---|
| ROOT CP3 SF-1 (archived; `git show 6235f6b430:projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/LOAD_STATE_IMPLEMENTATION/REVIEW_CHECKPOINT_3/ROOT_DISPOSITION.md`) and CP4 SF-1R | "Reserve the captured-replay work budget before selecting the join, so a case is selected only when replay can complete." SF-1R: "The replay reservation is a screen. … The guarantee is the fallback." Restated in code at PP `source_recovery.rs:258–267` and `lib.rs:4994–5006`. |
| RF §2.6 (`rf-source-method-extension.md:205`) | "Replay recomputes the Context, so a changed retained minor or entry is refused as "functional retained block/minor mismatch"." Mutant RF-7 is killed by `order_four_replay_refuses_a_mutated_minor` (RF §8, line 579). |
| RF §2.9 (226–241) | "On the pre-0.4 route, a selected case is replayed on its own ledger before publication." "Every new selection fits within 8,000,000 including replay." Its table's final case charges include the replay. |
| RF §5.4 | The "charged ≥ counted" control covers "every exact attempt and every replay" (432–435). The independent replay of the metering rule is a test-side oracle (370). |
| RF-14 (RF §8, line 587) | A pre-0.4 selection "is not declined at "captured replay reservation": it selects, then finalizes or takes the named refusal". `pre04_selection_takes_no_replay_screen` reaches the named refusal because the full replay does not fit. |
| O4 §3.5 (`o4-source-extension.md:190`) | "method work moves by 5·ΔI: 2·ΔI in the live attempt, 2·ΔI in the captured replay and ΔI at the binding check"; asserted by `o4_source_work_deltas_follow_the_d5_law` (`source_receipt/o4_source_tests.rs:640–690`). |
| R-A §2.2 and §3.1 (`source-work-nondyadic-inputs.md:157`, 257) | "The full exact pass runs twice, live and in the captured replay"; "the replay recomputes every row from the current invocation by the same rule. Nothing retained from the live attempt is reused." R-A rejected reusing live cofactors in the replay because "it would replace the replay's recomputation with retained state". Bind-only goes further, and that is why it is an owner ruling, not a work reduction within R-A's rule. |
| b7-plan D5/MJ9 (`b7-plan.md:252`) | The replay sits inside the local charges Q_k. |
| `retained-pricing-rules.md:433`, 458, 780–791 | T25, "the receipt replay, selected finalization", prices the replay's Context, Response, values, hints and `Shortcuts` (T25.1f, `T25_S1`). |
| FK doc comments (FF:980–982, 1025–1028) | "use replay_against for that"; "Live finalization performs one full independent captured-source replay, then uses this check for a second view." |

DEL-04-07 `pressure-contract.md:48` states the contract on which this depends: "Failed source admission or finalization remains a named refusal, not an inferred valid or eligible publication." It names no replay. RF §2.9 and R-A §1 quote "the pre-0.4 path retains its named refusal"; that phrase does not appear verbatim in `pressure-contract.md`.

## 2. Structural soundness of bind-only

### 2.1 The claim

Let g be the method's deterministic arithmetic, from a source snapshot S, descriptors Δ, limits L, order cap c and relative limit ρ to the retained arithmetic objects: witnesses W, Ratios D (displacement and reaction), DOF projections π_D, row Ratios V and row projections π_V. Write P for the retained proof, which records its own S_P, Δ_P, L_P and labels ρ_P.

**Claim.** If (L1) provenance, (L2) immutability, (L3) input closure and (L4) determinism hold, and bind-only's checks (L5) pass, then every arithmetic comparison of the full replay passes. The two replays then differ only:
- where the full replay refuses with `Budget` because the shared case or invocation ledger runs out; this is the coverage that bind-only gains;
- where a fault outside the model has made P differ from g of its own recorded inputs; this is the detection given up (section 2.4).

### 2.2 The invariants, from the code

**(L1) Provenance: P = g(S_P, Δ_P, L_P, c, ρ_P), recorded by one constructor.**
- `RetainedFunctionalSet` has one constructor, `FunctionalSet::retain` (FF:861–933; the only `RetainedFunctionalSet {` outside tests is FF:914). It has no import or wire constructor (FF:980–982 doc).
- `retain` requires the response to be bound to this ledger's token, order cap and limits (`response_binding`, FF:93; `binding`, FF:76). It requires one functional projection per value, each from this `FunctionalSet` (`ptr::eq`) at its own index (FF:870–878). Every DOF projection must carry this ledger's token (FF:885–895).
- It copies `values` and `descriptors` from the same `FunctionalSet`. Its plan is pointer-bound to the response's Context (FF:725–727). `evaluate` reads only that response and `plan.descriptors` (FF:728).
- Each `RetainedFunctionalProjection` copies a `QualifiedFunctionalProjection`. That object holds `project(w, values[index], relative_limit)`'s output and the `relative_limit` argument of the same call (FF:837–859). So the label is the limit used. `exact_sign` is `values[index].sign()` (FF:929, via FF:817–819).
- The retained response comes from `retain_with_work` (FB:1940–2012), which copies `self.context.source`, `self.context.witnesses`, the response's own displacement and reaction Ratios, and DOF projections pointer-checked to this response (FB:1951–1963). The only `RetainedResponse {` construction outside tests is FB:1990.
- In product code the live attempt is the only caller: `functionals.retain(&[], &dof_projections, &functional_projections, …)` (PP `source_recovery.rs:1899`). Every projection there uses `RELATIVE_LIMIT` (1882, 1892), and no contact is retained.

**(L2) Immutability from construction to finalization.**
- `RetainedFunctionalSet`, `RetainedResponse`, `RetainedFunctionalProjection`, `RetainedProjection`, `BlockWitness` and `Snapshot` have private fields. No method on them takes `&mut self` (FF:1012–1158; FB:2015–2056).
- `SelectedSourceRecovery.retained` is private and exposed only as `&RetainedFunctionalSet` (PP `source_recovery.rs:194`, 221–223).
- Only code in FK's `exact_boundary` module tree could write the fields. In product code it does so nowhere after construction. The tests that do (for example `block_order_tests.rs:756–800`) are test-only.
- Neither crate contains `unsafe` outside test text. So, in safe Rust, P at finalization is the object `retain` built.

**(L3) Input closure: g reads nothing outside (S, Δ, L, c, ρ) and the ±hints of R-A.**
- `Context::new_charged` (FB:724–985) reads `system.stiffness`, `force`, `free_dofs`, `prescribed`, `contributions`, the `congruence` list, `force_terms`, `limits` and `order_cap`. It copies each source field into the `Snapshot` it keeps (FB:960–978); `limits` is kept in `RetainedResponse.limits` and compared (FF:1077), and the order cap is a constant (below), and `Snapshot::matches` compares each bitwise, in order, including force-term source names (FB:1745–1808). It does not read `contribution_groups`, which PP passes as `None` (PP `source_recovery.rs:386`). The symmetry evidence is copied and compared, and PP passes `None`.
- The order cap is the constant `FUNCTIONAL_ORDER_CAP` on both sides (FF:185; `binding` checks it, FF:83).
- `solve` (FF:213–235; FB:1011) reads only the Context.
- `evaluate` (FF:603–692) reads the response, its Context's prescribed list and blocks, the descriptors, the hints and the shortcut flag. The hints and flag do not change any value. Structurally, `Sources::hint` depends only on the descriptor index and `nodal_start` (PP `source_recovery.rs:396–403`), which the descriptor layout fixes, so it is the same rule on both sides; and R-A's bit-identity argument (R-A design §3.1) is checked by its gating tests (`source_negated_rows_equal_the_long_evaluation`, `o4_source_controls_negated_rows_equal_the_long_evaluation`, `ra_gating_zhinge_negated_rows_take_b1`).
- `project` (FB:1325–1404) reads only the Ratio and ρ.
- Work charging decides only whether a step runs (`Budget`); it never changes a value. The expansion-length cap can only raise `Budget` (R-A design §3.1, B2).

**(L4) Determinism in the product build.**
- The exact path has no `thread_local`, `static mut`, interior mutability, hashing or I/O in FB, FF, or the expansion arithmetic they call (FK `structural.rs` `Expansion::add` and `add_product_charged`, and `crate::exact_sum`; Opus review, round 1).
- The only conditional code there is `#[cfg(any(test, feature = "exact-operation-count"))]` counting, which records and never alters a value (FB:17, 290–378; FF:629–739, 1267, 1382). No product binary compiles it.
- The `census-probe` and `loaded-maximum-hooks` features are off in every product and registered build, and their guards hold (R-A design §5).
- Binary64 arithmetic in Rust is IEEE 754 round-to-nearest with no implicit contraction.
- The replay runs in the same process and binary as the live attempt. So g applied twice to bitwise-equal inputs gives bitwise-equal outputs.
- Codex's scrutiny found no production dependency on thread-locals, feature switches or material-method resolution in the captured rebuild (PP `source_receipt.rs:173–203`, 299–327), and this design's reading agrees. That rebuild is unchanged under bind-only in any case.

**(L5) What bind-only checks.** Section 2.3 lists the checks. Together they establish: S_C ≡ S_P bitwise, where S_C is the captured rebuild's source; Δ_C ≡ Δ_P; L equal and the token the same ledger; every ρ_P label equal to `RELATIVE_LIMIT`; full coverage; and the expected (empty) contact list.

**The step.** By L5, the full replay's g(S_C, Δ_C, L, c, ρ) equals g(S_P, Δ_P, L_P, c, ρ_P). By L3 and L4, that equals what the live attempt computed. By L1 and L2, that is P. So every arithmetic comparison of the full replay holds. A refusal of the full replay that is not a comparison failure can only be one that depends on something other than (S, Δ, L, c, ρ). Within the method, only the ledger's remaining work does: a `Budget` refusal. ∎

### 2.3 Check by check

Every check that FF `replay_against`, `retained_charge` and `replay_leaves` perform, with its disposition under bind-only. *Kept* means bind-only performs the same check. *Redundant* names the invariants that make it so. *Given up* names the fault class whose per-invocation detection is lost (section 2.4).

| # | Check (FF line) | Arithmetic? | Bind-only |
|---|---|---|---|
| 1 | opening charge 24 (1074) | no | kept |
| 2 | token is this ledger's; limits equal; relative limit finite and ≥ 0 (1075–1081) | no | kept |
| 3 | n ≤ 256 and ≤ `limits.dofs`, else `Budget` (1082–1085) | no | kept |
| 4 | `descriptors_charge` of both lists; `same_descriptors` (1086–1090) | no | kept |
| 5 | values and projections cover the descriptors (1091–1095) | no | kept (G3) |
| 6 | `retained_charge`: dimensions of the retained response; `source_preflight` of the retained source; witness count, order ≤ 4 and size bound; Ratio term counts (1096, 1171–1213) | no | kept as a dimension check. Its Ratio preflights size retained expansions before they are compared; bind-only compares none. The prototype keeps the dimension tests at 4 units per Ratio and drops the 2 × size reservation (section 3.1). |
| 7 | `ratio_preflight` of every retained row Ratio; projections × 16 (1097–1100) | no | the dimension tests are kept. The size reservations are redundant: they size expansions for comparisons 11, 14 and 16, which bind-only does not make (section 3.1). |
| 8 | `prepare` on the captured source: `source_preflight`, `Context::new_charged`, with every source validation and the SPD warrant (1101) | **yes** (Preparation) | `source_preflight(expected)` kept. Re-preparation redundant (L3, L4, L5: the same bits passed the same validation live). Given up: F1 in live Preparation. |
| 9 | captured source equals the retained snapshot (1102–1109) | no | **kept**, as `Snapshot::matches(expected)` on the retained snapshot, as the binding check does. It now runs before any preparation (section 2.5). |
| 10 | witness count and `same_bits` (1110–1118) | **yes** (compares recomputed W) | redundant (L1 to L5). Given up: F1, F2, F3. |
| 11 | solve; displacement and reaction Ratios equal (1119–1124) | **yes** (Solve) | redundant (L1 to L5). Given up: F1, F2, F3. |
| 12 | `replay_leaves`: n × 8 charge; contacts equal the expected list in count; per contact dof, sense and gap binding, recomputation and `same_bits`; per DOF projection `dof < n`, label = ρ, 2n slots each once, re-projection and `matches_data` (1214–1260) | binding: no. Re-projection and contact recomputation: **yes** | binding, label, coverage and contact-list checks **kept** (G3). Re-projection redundant (L1: the data are `project(D_i, ρ)`; L4). Contacts: product retains none (L1), and the expected list is empty, so no recomputation exists to lose. |
| 13 | hints rebuilt from the rule (1129–1133) | no | not needed: no evaluation runs, and the hints change no value (L3). |
| 14 | `evaluate`; every row Ratio equal (1134–1140) | **yes** (Evaluation, the largest stage) | redundant (L1 to L5). Given up: F1, F2, F3. |
| 15 | per row projection: index = i and label = ρ (1141–1146) | no | **kept** (G3). |
| 16 | per row: re-projection, `matches` and `exact_sign` (1147–1151) | **yes** (Projection) | redundant (L1: data = `project(V_i, ρ_i)`, sign = `V_i.sign()`). Given up: F1, F3. |

The binding check against the live input (G5) and every publication-stage check (row binding by value bits and limit, `rows.rs:243–270`; derived checks; receipt hashing) are unchanged.

**What bind-only keeps, in short:** G1 (capture correspondence), G3 (coverage and policy) and G5. **What it removes:** G2 (re-execution equality) and the `Budget` exposure of G4.

### 2.4 The detection given up

The full replay is a second execution of g on bitwise-equal inputs in the same process. It can therefore fail only when P ≠ g(S_P, Δ_P, L_P, c, ρ_P), or when the second execution itself errs. These are the fault classes in which that happens, each with what would still detect it after adoption.

- **F1. Transient faults in the live computation or in retained memory.** Examples: a bit flip in an expansion during live Preparation, Solve, Evaluation or Projection, or in the retained copy before finalization. A fault in a live projection before both its copies are made (the published value and the retained projection) is not modelled by any mutant of section 3.4; it would agree with itself at the row binding, and so would publish.
  - Today the replay detects such a fault on every selected case, unless the same flip recurs.
  - After adoption, the arithmetic is no longer re-executed. A fault that survives the kept checks goes undetected: the source, dimension, coverage and policy checks, and the row binding of published values to retained projections (`rows.rs:243–270`), which refused M-PROJ (section 3.4). A corrupted Ratio is projected outward from the corrupted value, so the published interval can be wrong while it passes every structural check, the row binding and the readers. The readers check the statement, not the solve (REQ-005).
  - A transient fault during the replay itself today blocks the case (fail-closed). Bind-only has no such exposure.
  - No product check other than the full replay re-executes the source route's arithmetic. Section 3.4 measures the consequence.
  - **Rate.** This design does not characterize how often such faults occur on the target machines, and makes no claim about it. The owner is asked to accept the class, not a measured rate (Q1).
- **F2. Hidden-state dependence introduced by a later change.** Examples: a value that depends on a thread-local, a cached object reused across attempts, a feature switch reaching a product build, or iteration order. That breaks L4.
  - The full replay detects it per invocation, but only where the hidden state differs between the live attempt and the replay. A dependence that is the same at both times passes both.
  - After adoption it is caught by the test-side full replay on every exercised input (section 2.6), by review, and by the existing feature guards. It is not caught on product inputs the tests do not exercise.
- **F3. A construction or retention defect introduced by a later change.** Examples: retaining values from another evaluation, re-labelling a projection, or copying witnesses from another Context. That breaks L1.
  - Today's structure prevents these through `ptr::eq`, token and coverage checks, which were written with the replay as a backstop (FF doc at 1025–1028: "Live finalization performs one full independent captured-source replay").
  - After adoption they are caught by the test-side full replay and by the mutant tests (section 3.4) on exercised inputs, not per invocation.
- **F4. An input g reads outside the snapshot.** This breaks L3; an example would be a new field of `StructuralSystem` read by `Context` but not copied into the `Snapshot`.
  - The full replay recomputes from the captured system. So it detects a difference in such a field between the live input and the captured rebuild, but not a field on which both agree.
  - After adoption: review of `Snapshot`'s completeness, and the test-side replay where the field differs. The source route uses no such field today (L3).

**What was never detected, and stays so:** deterministic defects in the method, the descriptors or the capture's normalization. These reproduce in the replay (section 1.2).

**Disposition.** F1 is the only class that arises without a code change. F2 to F4 require a defect to be introduced. They then move from per-invocation detection to test-side detection on exercised inputs, plus review. Whether that disposition is acceptable is owner question 1.

### 2.5 Refusals, order and stage

Under L1–L4, bind-only refuses exactly where the full replay refuses, except in its `Budget` cases and in fault-induced mismatches. The refusal **stage and text** can differ in two ways:
- **A capture that does not match.** The full replay prepares the captured source before comparing it (FF:1101–1109), as Codex's scrutiny noted. A mismatching capture can therefore refuse first in Preparation, with `Invalid("stiffness source mismatch")`, a validation refusal or `Budget`. Bind-only compares first and reports the binding refusal. Both are blocking finalization failures; the message differs.
  Measured: on M-SRC the full replay refused in Preparation ("force source mismatch"), and bind-only at the binding ("functional expected source binding") (section 3.4). Every committed capture-perturbation test refused with identical text in both modes, because the full replay refuses those cases at the same plan or source binding (section 3.3). Its plan-binding refusals precede all arithmetic; its source-binding refusals follow its re-preparation of the Context (FF:1101–1109), but precede Solve and Evaluation.
- **Removed messages.** Bind-only never emits "functional retained block/minor mismatch", "… response mismatch", "… ratio mismatch", "… projection mismatch", "… DOF projection mismatch" or "… contact mismatch". The test-side full replay keeps them.

### 2.6 Keeping the invariants maintained after adoption

The full replay becomes a test-side check. It stays in FK and is reached only from tests, under `#[cfg(any(test, feature = "exact-operation-count"))]`, the pattern of `evaluate_functionals_long`. Its real reach after adoption is narrower than per-invocation detection, and is stated here as it would be built:

- **FK unit tests.** The existing replay tests stay as tests of the test-side check: `retention_replays_expected_plan_and_rejects_corruption_order_and_source_changes`, `order_four_replay_refuses_a_mutated_minor`, `congruence_replay_refuses_a_changed_operand_map_or_range`, the metered sweeps and the oracle (section 4).
- **J7.** The J7 metered control already replays every selected direct-method attempt (`j7_rf_census.rs:684`). That is RF's census, at RF's sizes.
- **PP lib unit tests.** A `cfg(test)` shadow hook would run the full replay on a private ledger after each product finalization in PP's lib unit tests. That covers the raw source controls, `o4_source_controls`, PIN-OFF-345 and the fallback and perturbation tests, wherever those run inside the lib test binary.
- **Not reached.**
  - PP's integration tests, among them MJ6/MJ9 (`tests/pressure_source_join.rs`, `tests/pressure_source_join/consequences.rs`), do not see `cfg(test)`. PP cannot gate code on FK's `exact-operation-count` without a feature of its own, and no finalization-observer hook exists today; the existing observer is `solver_observations`, `retained_product.rs:1367`.
  - The headless and pytest suites and every release binary.
  - The heavy inputs that bind-only newly covers: the Z-hinge, PEXACT and 9–10-case invocations are census-only `#[ignore]` targets today.
- **What adoption therefore adds**, as part of its implementation, so that the newly covered domain is not the least checked:
  - the PP `cfg(test)` hook above;
  - one maintained, non-ignored test that runs the shadow on a small set of today's blocked inputs: one PEXACT-B Timoshenko run, one decimal Z-hinge document and one 10-case invocation, sized to the hosted budget;
  - bind-only's own refusal tests for every kept check. These include the non-mutation relative-limit refusal that today exercises the replay (`functionals/tests.rs:484–495`).

  Integration tests, headless, pytest and release binaries stay outside the shadow's reach. That gap is part of what Q1 asks the owner to accept.
- **So after adoption,** F2 to F4 are caught on the exercised test inputs and in review, not per invocation, and most thinly on the newly covered heavy inputs.

## 3. Measurements

### 3.1 The prototype

- **Branch.** `claude/piping-replay-proto-20261021` at `b0091c7475`, on main `fa0028a7d7`. It is a scratch branch: never merged, and no PR.
- **Gating.** Every line of the prototype is compiled only under the existing opt-in `census-probe` feature, under R-A's census discipline (R-A design §5). No manifest or dev-dependency line was added. `census_probe_is_named_only_by_its_definition_and_forward` and `census_probe_is_off_in_the_ordinary_test_build` pass in the ordinary build. Census targets are `#[ignore]`.
- **Builds.** PP lib tests at the hosted-CI product_physics profile, `--features census-probe`; the headless runner with the forwarded feature. Work counts do not depend on the optimization level.
- **Switches.** These are thread-local, with environment defaults so that subprocess runs take part. W1's switches are set to their production values in every thread.
  - `ra_off`: the plan's and the replay's hint rules are dropped and `evaluate`'s shortcut flag is false, so every evaluation takes R-A's long path.
  - `bind_only`: PP `SelectedSourceRecovery::replay_against` runs `prepare_sources` on the captured input unchanged, then a census-gated FF `bind_against` instead of `replay_against`.
  - `shadow`: after bind (and, as a self-check, after a full replay), the full replay's body runs on a private ledger that no case reads. Its token check is necessarily skipped there. A disagreement is a finding.
  - Mutant selectors (section 3.4).
- **`bind_against`'s charges,** each made before its inspection:
  - 24;
  - `descriptors_charge` of both lists;
  - `retained_charge`'s opening 8, its retained `source_preflight`, 4 per retained Ratio and 8 per witness, for dimension checks only;
  - `source_preflight(expected)`, before `Snapshot::matches`;
  - 4 per functional projection;
  - n × 8, plus 16 per expected contact and 4 per DOF projection.

  The full replay's comparison reservations are not charged: two units per Ratio term, two per witness scalar, 24 per DOF projection and 16 per functional projection. They amount to 4,070–5,764 per Z-hinge or (3,4,0) case [M]. An adopted implementation fixes its own schedule, with the oracle.

### 3.2 Charges [M]

Controls (the 24 raw source controls, `o4_source_controls`, PIN-OFF-345 and MJ6/MJ9; 35 documents), R-A on:

| | Full replay | Bind-only |
|---|---|---|
| Replay stage, min / mean / max | 122,404 / 442,327 / 2,506,432 | 65,832 / 72,348 / 131,988 |
| Case total, min / mean / max | 818,749 / 1,590,062 / 5,667,706 | 762,177 / 1,220,084 / 3,227,228 |

- With R-A off, the full replay's stage is 127,913 / 486,764 / 2,806,861.
- On decimal Timoshenko Z-hinge inputs, the replay stage is 2,545,027–3,227,563 under the full replay with R-A on, and 80,128–80,188 under bind-only. With R-A off, completed replays cost 3,052,640–3,588,246; the largest figure there, 3,645,145, is a replay cut off at `Budget`, so it is a lower bound. The complete replay on the shadow's private ledger reaches 3,879,846. The largest case charge on these inputs falls from 7,242,115 to 4,094,740 with R-A on (Opus review, round 1, corrected the attribution).
- **Controls.**
  - With R-A off and the full replay, all 35 documents and all 322 J7 entries equal main `9e03991d39`'s committed bytes, which precede R-A.
  - With R-A on and the full replay, they equal main `fa0028a7d7`'s.
  - So the prototype's baseline is the product's.
- **What moves under bind-only.**
  - In the 35 controls, only `cases/#/work/charged`, `invocation_work/charged` and `receipt_sha256` move. `publication_sha256`, `publication_charged`, every diagnostic text and every reader verdict are unchanged. MJ9's accounting holds, and nothing falls outside the allowance.
  - In J7, classification is identical in all four configurations. The 62 method-only entries move nothing, and the 4 published entries move only `public/envelope_sha256`.
  - `diagnostics_sha256` does not move: the SELECTED text prints the live work, which bind-only leaves unchanged.

### 3.3 Coverage [M]

Selected, per configuration:

| Inputs | R-A off, full | R-A off, bind-only | R-A on, full (today) | R-A on, bind-only |
|---|---|---|---|---|
| Z-hinge EB decimal (60) | 50 | 60 | 60 | 60 |
| Z-hinge Timoshenko decimal (60) | 38 | 60 | 60 | 60 |
| Z-hinge dyadic, and all (3,4,0) (360) | all | all | all | all |
| Two-case EB / Timoshenko (60 / 60) | 46 / 38 | 60 / 60 | 60 / 60 | 60 / 60 |
| 8-case invocations (12) | 4 | 12 | 12 | 12 |
| 9-case invocations (12) | 0 | 12 | 8 | 12 |
| 10-case invocations (12) | 0 | 12 | 0 | 12 |
| phi2 runs blocked under the full replay with R-A off (48: PEXACT-A, -B and -D Timoshenko) | 0 | 48 | 24 | 48 |
| Required selections, `j2`/`rf`/`judged`/`phi2` (578 triples) | 578 | 578 | 578 | 578 |

- **R-A off, full replay.** This reproduces the R-A census. 26 sweep cases fail at "captured source replay" and 6 at "composite current source", all `Exact(Budget)` and all as the blocked `pressure-2` envelope.
- **R-A on, full replay (today).** The 24 phi2 runs still blocked are PEXACT-A r1e-08 (8) and both PEXACT-B (16) Timoshenko. The 16 blocked many-case invocations are all bound by the invocation ledger. 8 end as `Err` at the composite publication reservation, 4 as `Err` at the invocation publication reservation, and 4 as blocked envelopes at a replay limited by what remains.
- **Bind-only.** Every invocation selects. The largest invocation charge is 45,364,361 with R-A on and 52,019,893 with R-A off. The largest case charge with R-A on is 4,924,492, on PEXACT-B r1e-08 Timoshenko (4,094,740 on the sweep). With R-A off it is 5,555,025 on the same document (4,747,098 on the sweep). On that document the shadow full replay costs 4,056,554 with R-A on and 4,687,087 with R-A off.
- **Judging each newly selected case** (sweep, two-case, many-case, PEXACT):
  - the shadow full replay agrees;
  - every published projection equals the direct method's bit for bit (1,920, 4,320, 26,880 and 2,160 projections);
  - the Rust reader returns the selected standing;
  - the Python verifier passes every envelope;
  - the declared-problem oracle passes every judged input, including the phi2 batch's 384 case verdicts with R-A on.
- **Preserved cases** differ only in work, `receipt_sha256` and, in published envelopes, the digests binding them. No selection is lost, and no shadow disagrees.
- **R-A's necessity.** With bind-only, R-A is not needed for any measured case: every Z-hinge decimal and PEXACT case selects with R-A off. R-A stays as built; it still removes 17–18 % of the live pass.
- **The 0.4 screen.** No census case reaches it. Only the headless SF-1 cliff inputs are declined there, at live charges of 4,645,422, 5,304,649 and 5,961,136, identically in both modes. That is expected, since the prototype leaves the screen unchanged (section 4).

**Existing tests under bind-only** (census build; bind on, with bind off as the control):

| Test | Result |
|---|---|
| RF-14 `pre04_selection_takes_no_replay_screen` (limit c + c/2) | **No longer refuses.** The case finalizes and publishes; bind-only finalization costs about 0.62M on PIN-OFF-345. The assertion `source_block_recovery.is_none()` fails (`source_recovery.rs:3100`). |
| `a_join_that_passes_the_replay_screen_but_cannot_finalize_falls_back_at_the_public_limit` | Still falls back ordinarily; its cause moves to "composite current source: … current source binding … Exact(Budget)" at 7,999,930. The text assertion (line 421) fails. |
| `a_fallback_after_invocation_limit_exhaustion_stays_within_the_invocation_limit` | Still falls back; its cause moves to "invocation receipt: composite publication reservation". The text assertion (line 557) fails. |
| The other 7 `load_state_fallback_tests` | Pass. `the_pre04_composite_finalization_failure_is_unchanged_by_this_fallback` passes in both modes: its `Err` is the invocation-level "composite requires source selection and complete case qualification", not the replay. |
| Capture perturbations: `post_capture_perturbations_accepted_live_are_refused_by_captured_replay` (all five, including `evidence_only` and `strain_split_same_total`); `a_resolved_operand_perturbed_after_capture_is_refused_by_receipt_replay` (all four); `actual_capture_replay_rejects_changed_material_or_load_source`; `composite_captured_exact_empty_regions_replay_and_overrides` | Every perturbation still refuses, **with the same text** in both modes. The full replay refuses them at the same plan binding (before any arithmetic) or source binding (after its re-preparation of the Context, before Solve and Evaluation). |
| `o4_source_work_deltas_follow_the_d5_law` | Passes in both modes. Measured (live, replay, binding) identity deltas: EB pairs (36, 36, 18) or (0, 0, 0) in both; Timoshenko (144, 144, 0) full, (144, 0, 0) bind-only; Timoshenko dual basis (−1120, −1122, 18) full, (−1120, 36, 18) bind-only. |
| Headless `sf1_budget_cliff_publishes_the_ordinary_route_through_the_runner_not_a_blocked_result` | Passes in both modes. |

### 3.4 Mutants (supporting evidence only) [M]

The inputs are eight documents with R-A on: physics_source n05, n06, fields and mixed; PIN-OFF-345 r1e-08; load_reference_source n05; and two decimal Z-hinge documents (EB and Timoshenko). Each mutant is injected into the live attempt, the retained proof or the captured rebuild, never into the replay. "Published" means the case selected and published.

| Mutant | Fault class | Full replay | Bind-only |
|---|---|---|---|
| M-RATIO: one bit of one live row Ratio flipped before projection | F1 | refuses ("ratio mismatch") on 8/8 | **published** on 8/8; 1–2 tokens differ |
| M-DISP: one bit of one live displacement numerator flipped after solve | F1 | refuses ("response mismatch") | **published**; 22–102 tokens differ |
| M-WIT: one bit of a retained witness flipped | F1/F3 | refuses ("block/minor mismatch") | **published**; tokens identical |
| M-RET: one retained row Ratio changed after retention | F1/F3 | refuses ("ratio mismatch") | **published**; tokens identical |
| M-HIDDEN: a thread-local perturbs only the first evaluation in the thread | F2 | refuses ("ratio mismatch") | **published**; 1–2 tokens differ |
| M-PROJ: a retained projection value moved by one ulp | F1/F3 | refuses ("projection mismatch") | refuses, at the row binding ("projection binding/bounds") |
| M-LIM: one row projected live at 1e-8 | policy | refuses (identity/policy) | refuses (identity/policy) |
| M-LABEL: a retained projection's limit label changed | policy | refuses | refuses |
| M-SRC: one captured force value moved by one ulp | capture | refuses ("force source mismatch", in Preparation) | refuses ("functional expected source binding") |
| M-DESC: one captured descriptor atom changed | capture | refuses (plan binding) | refuses (plan binding) |
| M-CONTACT-RET and -EXP: a retained contact, or a nonempty expected list | policy | refuses | refuses |

On the load/reference-state document the full replay's refusal republishes the invocation ordinarily, as SF-1R states. For the five mutants that bind-only publishes:
- The Rust reader and the Python verifier accept every one.
- The declared-problem oracle fails only M-DISP on the two Z-hinge documents: their intervals no longer contain the exact value. It passes M-RATIO and M-HIDDEN, whose one-bit changes stay inside the published intervals.
- The shadow full replay catches all five. That is the test-side check section 2.6 maintains.
- On the load/reference-state document the readers give a numerically ineligible standing even unmutated, so they say nothing there.

These results bear out section 2.4: F1 to F3, the classes these mutants model, are detected per invocation today and would not be after adoption. The policy, capture and plan classes keep their detection. A wrong published token (M-RATIO, M-DISP, M-HIDDEN) reaches no product check under bind-only.

### 3.5 Jobs and evidence

- **Job log.** All runs went through `tools/t3_slot.sh`, from the first build (END 2026-10-21T20:17:50Z) to the last (END 20:45:49Z), in `/Users/ryan/dev/chirality-t3/guard/cargo_jobs.log`:
  - controls 20:24:40Z–20:24:43Z;
  - J7 20:24:43Z–20:24:46Z;
  - single-member sweep (480 documents, 65.2 s) 20:24:46Z–20:25:51Z;
  - two-case 20:25:51Z–20:26:30Z;
  - many-case 20:26:30Z–20:27:44Z;
  - required batches 20:30:21Z–20:38:15Z;
  - PEXACT 20:40:14Z–20:40:23Z;
  - mutants 20:40:49Z–20:40:51Z, oracle 20:41:27Z;
  - existing tests 20:42:05Z–20:45:16Z;
  - the ordinary-build feature-guard check 20:45:23Z–20:45:49Z.
- **Return codes.** Every job returned rc=0, except the two bind-on runs of the existing tests (END 20:42:21Z and 20:44:23Z, rc=101), whose failures are the three of section 3.3.
- **Raw output** is in host scratch, not committed: `scratch/replay-proto/run1/`.
- **Not run:** the matched-input ablations and the small-frame sweep (neither reaches a moved stage), the FK unit suite under the feature, mutants with R-A off, and the registered build's Direct route.

## 4. What would move at adoption

Adoption would be one implementation PR. It needs its gating tests, the actual-source D3, the independent numerical review AX-006 requires, and a reconciliation of this file's dependants. The list below is the inventory at `fa0028a7d7`. It is complete for the categories named, but the implementation re-derives it from its own diff.

**Product code (priced source).**
- **FF.** A new `RetainedFunctionalSet::bind_against`, which performs the kept checks of section 2.3. `replay_against` moves under `#[cfg(any(test, feature = "exact-operation-count"))]`, the pattern of `evaluate_functionals_long`. The doc comments at FF:980–982 and 1025–1028 are restated. `FunctionalReplayCheck` is read only by FK tests today; bind keeps its counts.
- **PP `source_recovery.rs`.**
  - `replay_against` becomes the bind call. Its stage strings ("replay source closure", "expected invocation replay") are renamed, and its doc (336–337) and the struct's (196–197) are restated.
  - The test-side full replay keeps a `cfg(test)` wrapper for J7 (`j7_rf_census.rs:684`) and RF-10 (`source_recovery.rs:2785–2799`).
  - `reserve_captured_replay` and its doc (258–281): see the screen, below.
- **Doc comments to restate:** PP `check_binding_against` (`source_recovery.rs:315–316`, "after the independent captured-source replay"), and the SF-1 comment at `lib.rs:4994–5006` ("captured replay repeats the live attempt's source closure and exact solve"), which becomes false under bind-only even while the screen stays.
- **PP `source_receipt.rs`.** The error prefix "captured source replay: …" (272, 388) is restated. The captured rebuild (173–289, 290–388) stays.

**The 0.4 replay screen.** SF-1's premise is that the replay "repeats the live attempt's source closure and exact solve". Under bind-only that premise is false: the finalization that follows selection costs about the rebuild plus binding plus reservation, not a second live pass. There are three options:
- (a) **Keep it unchanged.** 0.4 behaviour does not move, but the screen then declines joins that would finalize. That is a stale estimate kept as a rule, which RF §5's ruling ("a stale estimate, not a sound requirement") counts against.
- (b) **Resize it** to bind-only's finalization cost.
- (c) **Remove it.** SF-1R's ordinary republication remains the guarantee ("The guarantee is the fallback").

The screen governs 0.4 publication, which the next unit unifies with pre-0.4 (section 7). This design therefore recommends (a) at bind-only's adoption and leaving the choice between (b) and (c) to that unit. Section 3 reports whether any measured case is declined at the screen.

**Fixtures** (regenerated by their own writers; each moves only `work.charged`, `invocation_work.charged`, the hashes that bind them and printed work figures, under a work-only diff like R-A's):
- the 24 raw source controls, under `fixtures/product_preview/pressure_source/physics_source/` and `load_reference_source/`, and J2 `fixtures/results/pressure_source_reader_corpus.json`;
- `fixtures/product_preview/pressure_source/o4/o4_source_controls.json`;
- the 30 load/reference-state twins `fixtures/results/pressure_source_load_state_*`;
- J7 `fixtures/results/j7_rf_census_expected.json`. J7's `first_cause.work.charged` is direct-method work from `solve_ordinary`, which runs no finalization (`j7_rf_census.rs:252–296`), so none of its 66 method entries moves. Only the 4 publicly selected PIN-OFF-345 entries' `envelope_sha256` moves, because the envelope's receipt work includes the replay. Section 3 confirms which digests move;
- MJ6 `CURRENT_SOURCE_BASIS` (`tests/pressure_source_join.rs:472–497`): its sha256 values, and a length wherever a work figure loses a digit;
- the generation records `fixtures/product_preview/demo_fixture_generation.json` and `preview_physics_fixture_generation.json`, which hash every PP and FK source and test file;
- `tools/hosted-ci-routing.json`: the FK and PP `source_trees` digests, the headless digest if its tests are edited, and the per-reader `reads[].sha256` of each edited test file (`pressure_source_context_tests.rs`, `o4_source_tests.rs`, `pressure_source_join.rs`, and others as edited).

**Work-only diff checks.** `tests/test_ra_work_only_envelope_diff.py` (base `9e03991d39`; its docstring says "a later change to a listed file restates its own base here") is re-anchored, and a sibling check binds A1's moved bytes from the pre-adoption main. `tests/test_o4_text_only_envelope_diff.py` reads R-A's files at its own base and is unaffected.

**Budget cliffs and the screen's tests** (`source_receipt/load_state_fallback_tests.rs`, and the headless `load_reference_route_tests.rs:844` and `tests/load_reference_cli.rs:548`).
- These are RF §5.5's cliffs, the SF-1 scenarios and N-2/N-3. They are tuned to the replay's cost or assert "captured source replay" as the cause.
- The screen tests are unchanged under option (a).
- The cliffs whose finalization failure is the replay (`a_selected_case_whose_own_finalization_fails_after_the_reservation_falls_back`, `a_join_that_passes_the_replay_screen_but_cannot_finalize_falls_back_at_the_public_limit`, `a_fallback_after_invocation_limit_exhaustion_stays_within_the_invocation_limit`) are re-derived by RF §5.5's rule: recompute the cliff, scale it within D1's caps, or restate it with a private limit and a positive control.
- As measured (section 3.3), two flip, and only in their asserted cause; both still fall back ordinarily. `a_join_that_passes_the_replay_screen_but_cannot_finalize…` now fails at the live binding, and `a_fallback_after_invocation_limit_exhaustion…` at the composite publication reservation. Their causes are restated, or their inputs re-derived so that the intended stage is the one that fails. The other seven pass unchanged.

**Work laws.**
- **U4b** (`pressure_source_context_tests.rs:735–826`). `PRE_U4B_WORK` holds historical counters that include the full replay, and R-A's replay term (795) disappears. The law therefore gains a measured "replay removed" term per control, or is rebased with the rebase stated, as R-A did.
- **O4 D5 law** (`o4_source_tests.rs:640–690`, doc 604–638). The test passes unchanged in both modes (section 3.3). On the Euler–Bernoulli twins, bind's replay identity charge is the same 2ΔI, from `retained_charge`'s and bind's two `source_preflight`s. On the Timoshenko twins the measured replay delta changes (section 3.3), and the test's assertions still hold. O4 §3.5's "2·ΔI in the captured replay" sentence and the test's doc (604–638, 667–673) are restated to say that the replay term is the bind's two preflights.
- **MJ9** (`tests/pressure_source_join/consequences.rs:913`). Its accounting structure holds; its values move.
- **RF §5.4's control.** The J7 metered control keeps replaying every selected attempt test-side, so "charged ≥ counted for every replay" still runs.

**Tests that assert replay refusals on the product path.**
- RF-14 (`pre04_selection_takes_no_replay_screen`, limit c + c/2). Its named refusal exists because the full replay cannot fit. Under bind-only it no longer refuses: the case finalizes at about 0.62M of finalization work (section 3.3). It is restated to pin the absence of a screen with a private limit at which bind-only's finalization fails, plus a positive control.
- The capture-perturbation tests:
  - `load_state_join_tests.rs:549` (K5, K7, K10);
  - `load_state_tests.rs:225`;
  - `source_receipt/tests.rs:138` and 282.

  They must still refuse through the bind. Their asserted prefix moves with the stage string (section 3.3).
- The FK tests of section 2.6, `metered_tests.rs`' replay entries (1117–1278, the oracle's entry list at 1409) and `metered_tests/explicit.rs:597–621` stay as tests of the test-side replay. `metered_mul_charges_match_the_independent_oracle` gains bind's charges.
- `s11f_site_test.rs:2289–2295` pins `reserve_captured_replay(` in `solve_prepared_load_case`; it is unchanged under option (a).

**D3.**
- The edited FF, `source_recovery.rs` and `source_receipt.rs` are priced source.
- T25's `S1` stage ("check_input (requested, rebuild, replay)", `tools/retained_pricing/base/t25_g4.py:144`, 169–177, 298) loses T25.1f, the replay's Context, Response, values, hints and `Shortcuts`. So `T25_S1` and `success__T25_S1` (`retained_memory.rs:2699`, 2749) are re-priced.
- `ra_owners_model.py`'s `BODY_PINS` and `STATEMENTS` (33–39) name the replay's body and its hint and evaluate statements, and are restated.
- Full-file pins move in `b7a_formation_model.py`, `b7a_domain_model.py`, `b7a_retired_carry.py` and `b7a_phase_model.py`'s line keys, and among others `rules/sq2_decisions_E.py`'s line keys.
- `PINNED_RECORD` (`retained_memory_law_tests.rs:1775–1787`) is re-checked with the profile. T25_S1 is not a phase maximum (`retained-pricing-rules.md:791`), so X1 probably does not move; the pricing run decides.
- The registration starts from the profile registered at adoption.

**Design text.**
- RF §2.6 (205), §2.7 and §2.9 (the measured "including replay" figures stay as history and gain a pointer), §5.4 (432–435), RF-7 and RF-14 (579, 587);
- O4 §3.5 (190), 128, 181 and 543;
- R-A §2.2 (157), §3.1 "The replay" (257), and the stage and hint text at 197, 239 and 243;
- `retained-pricing-rules.md:433`, 458 and 780–791;
- b7-plan (this unit, D5/MJ9's Q_k at 252);
- the FK and PP doc comments.

**What does not move.** No value, interval, bound, basis, row, class, first cause, selection or `response_digest` of a case selected today moves. The same holds for the descriptors, `normalized_source_sha256`, `functional_plan_sha256` and `finalization_size`, for every limit and C₂ = 96, and for every reader. The readers check the ledger's structure, never the replay's cost. The legacy `RetainedResponse::replay_against` (FB:2057), used by the nonlinear strict-gap path, is a different method and is untouched.

## 5. Alternatives on the same evidence

Each alternative is compared on the measurements of section 3. "Detection" means the per-invocation detection of F1 to F4.

| Option | Coverage of today's blocked cases | Detection | What it needs and touches |
|---|---|---|---|
| **A0. Status quo** (full replay) | None gained. Still blocked: the 24 phi2 PEXACT runs and the 16 9–10-case invocations [M]. | Kept. | Nothing. |
| **A1. Bind-only** (this design) | All of them [M]. | F1 to F4 given up; the policy, capture and plan classes kept. | Owner rulings Q1–Q3; the implementation, D3, re-pins and review (section 4). |
| **A2. Bind-only plus an a posteriori certificate** | As A1, if the certificate fits. | Partial, depending on the certificate. For example, an exact per-block residual check K·x = f would catch M-DISP but not M-RATIO, M-HIDDEN or M-RET, because row evaluation would not be re-checked. | **Unpriced and undesigned.** It needs a new metered check, its own soundness and charge oracle, and possibly a new primitive, and so RF §5's "one rule" ruling. Its cost is not measured. A residual over the free blocks is about the Solve's reaction stage, 19 % of the live pass [C, R-A §5.1]. |
| **A3. Separate replay ledger** (the replay gets its own per-case 8M) | The per-case blocks (PEXACT, the sweep with R-A off) clear, since a complete replay costs at most 4.69M [M: the shadow's private replay, PEXACT-B r1e-08 Timoshenko, R-A off; 3.88M on the Z-hinge sweep]. The 16 many-case blocks do not, if the replay still debits the 64M invocation ledger: they are bound by that ledger today [M]. They clear only if the invocation ledger is separated or raised as well. | Kept. | Not the same as raising limits. Each ledger keeps 8M, but a case's total work becomes up to 2 × 8M, so what `PHYSICS_SOURCE_WORK_LIMIT` bounds changes. That is a fixed-limit change under REQ-003 (e), the readers' ledger structure changes, and it needs D3. |
| **A4. Raised limits** | Per case: PEXACT-B r1e-08 Timoshenko needs its bind-only charge plus a complete replay, the full replay in place of bind's, 4,924,492 − about 80k + 4,056,554 ≈ 8.90M with R-A on [E, from the measured figures of section 3.3]. Per invocation: a 10-case decimal Z-hinge invocation needs about 10 × 6.5M–7.2M plus publication, so 66M–73M [E, from the measured full-replay case charges of 9-case invocations]. | Kept. | Prohibited as a means to a bound by REQ-003 (e) ("No … limit is relaxed to obtain a bound") and AX-005, and excluded by R-A design §3.3. A priced-source change with D3. Coverage is bounded by the new numbers, not by structure. |
| **A5. Opportunistic replay** (bind-only, then the full replay when a reservation for it remains) | As A1, if the full replay is skipped whenever it does not fit [D]. | Kept only where the replay fits after bind. That is **not** every case that selects today. PEXACT-A r1e-12 Timoshenko selects today at about 7.94M (live about 3.67M, replay about 3.57M). After bind about 3.66M remains, less than a live-charge reservation, and running the replay anyway would total about 8.02M [M/E, Opus review, round 1]. On the 9-case invocation `zhinge122 (0.219, 0.0082) near_phi2_mm r=1e-12 timo decimal zero`, today's full-replay invocation charge is 63,810,222. Bind's additional checks (733,576) take bind-then-replay to 64,543,798, over the 64,000,000 invocation limit [M, Codex review round 1], so at least one replay there must be skipped. Detection is also given up on the newly covered cases, which are the heaviest. | Needs a reservation design of its own: today's live-charge reservation is a screen, not a guarantee (`source_recovery.rs:258–267`). It also brings two finalization regimes keyed on remaining budget, published work that depends on which ran, and the kind of seam RF §5's ruling rejected for charging. Both paths need tests, D3 and their own cliffs. Undesigned. |

A cheap integrity digest of the retained proof, taken at retention and rechecked at finalization, would detect changes to retained memory between those points (M-WIT, M-RET). It would not detect faults in the live arithmetic before retention (M-RATIO, M-DISP, M-HIDDEN). Safe Rust excludes software writes to the retained proof, so the digest's value is against hardware memory faults, which are what M-WIT and M-RET model. It is a partial, inexpensive option and is not designed here; it could accompany A1 if A1 is adopted.

## 6. Owner questions

These are reserved for the owner and routed through Agent 0. Each is presented with its consequence and a recommendation. Revision 2 changed the overall recommendation from adoption to "not now". Both round-1 reviews found the argument sound. The change follows from their findings on what adoption gives up and gains: the plain readings of REQ-003 (e) and AX-005, the test-side reach, and the narrow gain in the product configuration (Opus review, round 1).

**Q0. Adopt bind-only now?** The three questions below decide whether A1 can be adopted. Weighed against its cost, the recommendation is **not now (A0)**. The design and prototype are kept as the evidence for a later ruling.
- **The gain in the product configuration is narrow.** With R-A on, bind-only newly selects 24 runs on 12 of the oracle's invented, exactly φ = 2 sensitive documents, and 16 invocations of 9–10 load cases near the 64M ledger (section 3.3). Each of these fails closed today, with no wrong output, but also with no ordinary results. The 24 runs and 4 of the invocations end as pre-0.4 blocked envelopes with no result rows; the other 12 invocations end as a run-level `Err` (section 3.3). R-A's design records that form as "fail-closed, but … not REQ-003 (c)'s ordinary route" (`source-work-nondyadic-inputs.md:49`).
- **The loss applies to every invocation.** The full replay is the only product check that re-executes the source arithmetic, and so the only one that refuses a bound corrupted coherently in the live arithmetic or in the retained Ratios. M-DISP published intervals that no longer contain the exact value, and both readers accepted them (section 3.4). After adoption the test-side replay reaches least where coverage is gained (section 2.6).
- **The plain readings of REQ-003 (e) and AX-005 bear against removal** (Q1).
- **When to revisit.** A coverage need that the R-A-class reductions cannot meet. Examples: a domain extension whose live pass exceeds half the 8M case limit, or invocations whose full-replay total approaches the 64M invocation ledger, which is what binds the 16 invocations today. Removing one arithmetic pass then frees per-case and per-invocation work capacity: the replay stage, which is about the live pass. Coverage would be measured for the proposed extension, since work does not grow linearly with problem size. A1 is designed and measured; its implementation, the additions of section 2.6, D3 and the independent review remain.
- **Consequence of A0.** Nothing changes now. The 24 runs and 16 invocations remain live deviations from REQ-003 (c), with no ordinary results, until the pre-0.4 route failure unification lands. That unit can give them ordinary results, with their failure recorded, without removing the replay. So A0 followed by that unit is the coherent path: it repairs publication, though it gains no verified bounds (section 7) (Opus review, round 2).

**Q1. REQ-003 (e) and AX-005: is removing the full replay a relaxation of verification, or a loosened gate?**
- **What is at stake.**
  - The full replay re-executes the method. It does not establish the bound in a fault-free run: the bound rests on the live exact arithmetic and the outward projection (section 1.2). Under L1–L4, its fault-free outcome differs from bind-only's only where it runs out of work (section 2.2).
  - Under fault, however, it is the only product check that re-executes the arithmetic behind the published bound. The kept publication checks refuse some corruptions (M-PROJ), but not the coherent ones (M-RATIO, M-DISP, M-HIDDEN; section 3.4). Removing it gives up the per-invocation detection of F1 to F4 (section 2.4).
- **REQ-003 (e)** ("No search, verification, tolerance or domain limit is relaxed to obtain a bound").
  - The plain reading counts the replay as a verification: it is a per-invocation check that refuses publication, and it would be removed so that more cases publish a bound. That is reading (b), and it closes A1.
  - Reading (a) is a definitional ruling. "Verification" in (e) would mean verification of the bound's method (its searches, gates, tolerances and domain limits), not re-execution integrity. That ruling would be made together with an explicit acceptance of F1 to F4's residual risk. The design does not characterize the rate of F1 on the target machines.
- **AX-005** ("A gate is never loosened to obtain a pass").
  - The full replay (G2, at its G4 cost) is a refusal condition. In a fault-free run its comparisons never refuse, and its only refusals are `Budget` from its own work. Removing it is what obtains the new passes. So under AX-005's plain reading, too, removal loosens a gate.
  - R-A was different: it removed redundant arithmetic and kept every check. Bind-only removes a check.
  - Reading (a) would rule that a check whose only fault-free refusals are budget refusals is not a gate in AX-005's sense.
- **Consequences.** Under (b), A1 is closed, and so are A3 and A4, which also bear on (e). Under (a), A1 can be adopted with the conditions of section 2.6 and the independent numerical review.
- **Recommendation:** (b), the plain readings, consistent with Q0. If the owner prefers the coverage, (a) is the only reading that permits A1. It is then both a definitional ruling and a risk acceptance, and both are the owner's.
- **A related reviewed input.** DEF-O's and DEF-E's `trust.validation` (`fixtures/results/retained_precision_prepared_{ordinary,exact}_v1.json`, digest-bound and checked at G0) reads: "independent Rust replay and producer review validate algorithm/custody on exact implementation; Python/TS do not replay private proofs". REQ-005 cites DEF-O `trust`.
  - These definitions govern retained-precision successors (REQ-001), not `pressure-source-1` receipts. The sentence entered with the F2a D1 retained-precision successor (`5a0461661f`). Its design names the replay there: a Rust validation-lane audit in `numerical_robustness` that replays a captured invocation and compares a retained-state digest, "not a standing input" (archived D1 `DESIGN.md` §4.1.8 at `5a0461661f`, `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/DESIGN_NUMERICS/DESIGN.md:522–527`; Codex review, round 2, corrected revision 2's attribution to the reader's G5 schedule replay). So it is a test-side audit of the retained-precision route, and this design reads it as not bearing on the source route's producer replay. It moves nothing.
  - If the owner reads it as also covering the source replay, A1 would change a reviewed input: a new digest, both readers' G0 tables, `REVIEWED_INPUTS` and D3 (Opus review, round 1).

**Q2. AC-002's reading (only if A1 is adopted).**
- **The measured effect.**
  - J7's classification, counts, first causes, selections, charged work and frozen-row verdicts are unchanged. Its `first_cause.work.charged` is direct-method work and does not move.
  - Only the 4 published entries' `envelope_sha256` moves, because their receipts' work moves. `diagnostics_sha256`, `results_sha256` and `summary_sha256` do not move (section 3.2).
- **The text.** AC-002 reads: "its envelope and diagnostics digests change only as a stated product-text, identity, published-row or work change requires … (owner, 2026-10-20, approving this wording for R-A, Design `source-work-nondyadic-inputs.md` §6)".
- **Readings.**
  - (a) The wording governs, and bind-only's moved digests bind a stated work change.
  - (b) The approval was given for R-A, a reduction of arithmetic. A change that removes a check needs the owner's confirmation, or an amended wording, before it re-pins J7.
- **Recommendation:** if A1 is adopted, the owner confirms (a) explicitly for this change. No textual amendment is needed, since no charge falls below executed work: bind-only charges every inspection it makes.

**Q3. Keep producer self-replay regardless?**
- **Options.**
  - (a) No. A1: the full replay becomes test-side only.
  - (b) Yes, always. That is A0: no coverage is gained, and the unit closes with this design as its record.
  - (c) Opportunistically. That is A5: the full replay runs only when a reservation for it remains after bind. Detection is **not** kept on every case selected today (section 5: a 9-case invocation and PEXACT-A r1e-12 Timoshenko lose it), and it is lost on every newly covered case. It needs a reservation design of its own, and brings a second finalization regime keyed on budget.
- **Recommendation:** (b), consistent with Q0. A5 is not recommended in any case: it keeps detection unevenly, adds a budget-keyed seam, and is undesigned.

**Not reserved** (design choices made here, revisable on review):
- if A1 is adopted, the 0.4 screen stays as it is at adoption, and its future is decided in the next unit (section 4);
- RF-14 is restated, not deleted;
- the full replay is kept test-side under `exact-operation-count`;
- `retained_charge`'s dimension checks are kept in bind.

Decision 22, G10, B8 and release are untouched.

## 7. The next unit: pre-0.4 route failure unification

*Taken up in [pre04-failure-unification.md](pre04-failure-unification.md) (owner, 2026-10-21, "16 start now."; text approved 2026-10-22, decision 17).*

b7-plan orders "Pre-0.4 route failure unification" after this ruling, "since both change finalization". The ruling decides the unit's inputs.

**Under A0, the recommendation.**
- **Live occurrences.** The 24 phi2 PEXACT runs and 16 many-case invocations stay live occurrences of today's pre-0.4 failure forms. The runs end as blocked envelopes with no result rows. The invocations end as 4 blocked envelopes and 12 run-level `Err`s (section 3.3).
- **The unit designs their outcome.** That includes ordinary republication with the failure recorded, as on the 0.4 route (SF-1R). That repair gives these cases ordinary results without removing the replay.
- **Its census inputs.** These measured inputs, plus the constructed cliffs (RF §5.5 and RF-14, unchanged under A0).
- **Its two forms.** It defines both fail-closed forms. Many-case invocations move between the blocked envelope and the run-level `Err` as work shifts (R-A design §5.1).
- **The misleading cause.** "Source functional row binding is not unique" on a support-norm budget refusal (R-A design §5.1) is in the shared case solver, independent of the replay. R-A's census observed it on the 10-case invocation `zhinge122 (0.168, 0.007) long r=1e-12 eb decimal zero`, at its census revision. On main, in this design's census, that invocation instead ends at the invocation publication reservation (charged 63,998,247) as a run-level `Err`; "not unique" occurs nowhere in the many-case output. R-A's census charged B1 and B2 differently from the adopted implementation (R-A design §3.1, "What the census measured"), which is consistent with the difference but not shown to cause it. So R-A §5.1's witness is historical: the cause is reachable today only by construction. The unit establishes a current reproducer or a constructed control before it repairs the cause (Opus and Codex reviews, round 3).
- **The 0.4 screen** stays as SF-1 built it. Whether pre-0.4 gains a screen is part of the unit's single failure behaviour; RF-14 pins that it has none today.

**If A1 is adopted instead.**
- **Which failures remain.** No measured input fails pre-0.4 finalization for want of replay work (section 3.3). What remains are constructed cliffs (RF §5.5, RF-14 as restated), binding refusals from faults or defects, and large invocations that exhaust a publication or support-norm reservation. The unit's census then has to construct its failures.
- **The misleading cause.** As under A0, it is reachable only by a constructed support-norm exhaustion, so the repair stays in the unit.
- **The 0.4 screen.** Its sizing (section 4, options b and c) joins the unit's single failure behaviour.
- **Sequence.** Both units edit `composite_exact`, PP `source_recovery.rs` and the finalization arm of `lib.rs` (6590–6635). So A1's implementation lands first, the unit is designed on its as-built code, and the two are not implemented concurrently.

## 8. Verification and review

- **This design.**
  - Dual review on the same revision, by an independent Claude Opus 5.5 reviewer and an independent Codex reviewer. Each also gives independent numerical scrutiny of section 2's argument.
  - The rounds and their dispositions are recorded in the PR that carries this file.
- **At adoption.**
  - The implementation's own gating tests: every kept check's refusal, and the shadow comparison in test builds.
  - The mutants of section 3.4 as maintained tests of the test-side replay.
  - A work-only envelope diff against the pre-adoption main.
  - VER-002's checks and the J7 census.
  - The oracle extended with bind's charges.
  - The actual-source D3.
  - The independent numerical review that Root `AGENTS.md` and AX-006 require.
