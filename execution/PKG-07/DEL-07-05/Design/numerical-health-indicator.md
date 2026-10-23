# Numerical health indicator

This design gives each desktop result a plain numerical-health indicator, built from diagnostics the engine already computes for an ordinary solve. Paths are relative to `projects/chirality-piping/`. Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

## Basis

**Owner, 2026-10-23,** accepting Agent 0's recommendation: "I agree with your recommendation. Proceed accordingly." The recommendation had three parts:

- **No verified bounds in v0.1.** The retained-precision method stays dormant and owner-held. B8 is not activated, and DEL-04-08 AX-004 and PRD §12.8's last bullet stand unchanged.
- **A health indicator instead.** This unit becomes a numerical health indicator. It uses the residual, a conditioning estimate expressed roughly as "digits lost", and the specific cause where it is known.
- **Coverage and wording.** It covers every model size at small cost, with plain green, red and grey states. It is not a guarantee and must not say "verified", "certified" or "bound".

The requirement it serves is PRD FR-SOL-015: "Solver reports singularities, ill-conditioning, residuals, convergence issues, and invalid elements."

It is a results-viewer presentation (DEL-07-05) of solver diagnostics. DEL-04-01's kernel and DEL-04-06's diagnostics produce those diagnostics, and DEL-04-07 publishes them.

## 1. What the engine already provides

**Per load case**, from the ordinary solve:

- **Verdict.** The structured verdict is `numerical_quality.cases[].solve_quality`: `checks_passed`, `sensitive`, `unresolved`, `failed` or `not_assessed`. `structural_status` accompanies it.
  - Both are derived from the case's integrity diagnostic code: `NUMERICAL_INTEGRITY_CHECKS_PASSED`, `_SENSITIVE`, `_PHYSICAL_MECHANISM`, `_NEGATIVE_ENERGY`, `_UNRESOLVED`, `_ASSEMBLY_UNRESOLVED` or `_FAILED` (`core/product_physics/src/lib.rs` `append_integrity_report`, `integrity_failure_code`).
  - `numerical_quality.status` is the worst case's verdict.
- **The kernel's thresholds** (`core/solver/frame_kernel/src/structural.rs` `finish_checked_factor`, policy `M03-INTEGRITY-v1`; DEL-04-01 Design `kd5-member-relative-bound.md`):
  - A case is `sensitive` when the reciprocal condition estimate rcond < √ε (about 1.5·10⁻⁸). It is also `sensitive` when a load contribution is absorbed in rounding (load fidelity), or when the formation checks demote it (K-D5, S11-G).
  - A case is unresolved when the original-equation residual fails `guarded ≤ 64·γ(ops)` after at most 3 refinement passes.
  - A mechanism, negative energy, a zero diagonal or an asymmetry fails the case.
- **Numbers.** rcond, the per-row residual ratios (`guarded_ratio`, `target`), the refinement count and the pivots are computed for every solved case, on both dense and sparse paths.
  - They reach the published result only as the integrity diagnostic's message. The `StructuralReport` is written there in Rust's `Debug` form: `… reciprocal_condition_estimate: 1.0578136350861028e-11, residual_rows: [ResidualRow { … guarded_ratio: …, target: …, passed: … }], refinement_attempts: …`.
  - rcond is the 1-norm estimate (Hager–Higham) of the power-of-two diagonally scaled matrix. It is an estimate, not a bound.
- **Located causes**, as diagnostic message text:
  - the K-D5 `formation_check: … row={node}:{DOF}`;
  - S11-G's named rows (at most 6);
  - a mechanism's direction;
  - a zero diagonal's DOF.

  As structured `affected_refs`:
  - load fidelity (`LOAD_CONTRIBUTION_ABSORBED`, with its ledger sources);
  - `SOLVER_SYSTEM_BLOCKED` (`supports`);
  - node and pipe input errors.
- **Not computed:** stiffness contrast between members, rigid-element stiffness and near-duplicate nodes.

**What the desktop shows today:**

- **Results panel.** It shows "Numerical integrity: {status}. … Numerical checks do not establish engineering correctness." (`ResultsPanel.tsx:110`).
- **Solve panel.** It has a diagnostics count tile.
- **Diagnostics panel.** It lists every message. Clicking selects the first resolvable `affected_ref`, which for integrity diagnostics is the load case.
- **The "Needs recompute" problem.** The Rust reader marks any result whose numerical quality is not `checks_passed` as `NUMERICAL_INTEGRITY_NOT_QUALIFIED`, so it is not numerically eligible. Such a result is therefore not Current: its outputs refuse, and the shell chip reads "Needs recompute" (`shellLayout.ts:396`). For a sensitive or failed case that wording misleads, because recomputing the same model gives the same answer.

## 2. Decisions

1. **States follow the kernel's verdicts. No new threshold is introduced.**
   - Green, red and grey map onto `solve_quality` and the integrity codes, under the existing `M03-INTEGRITY-v1` thresholds.
   - DEL-04-06's condition-ratio thresholds (`classify_condition_ratio`) stay unused and `TBD` (DEL-04-06 CLM-029, "a governed policy decision"). The indicator neither sets nor uses them.
   - The "digits lost" figure is informational and drives no state.
   - So no governed threshold value is decided here. This is recorded as within agent authority: it presents existing governed policy.
2. **The engine and the published format are unchanged.**
   - The numbers are read from the integrity diagnostic's `StructuralReport` text by one Rust function in the wasm exports crate: `core/model_operations/operation_applier`, `numerical_health_json`.
   - Its parse is strict and anchored: the diagnostic id `diagnostic:numerical-integrity:{case}`, the prefix `StructuralReport { policy: "M03-INTEGRITY-v1"`, and the fields `reciprocal_condition_estimate`, `residual_rows` and `refinement_attempts`. It fails closed. If any field is absent or malformed, the case keeps its state and shows no figure.
   - It is a Rust reader function (AX-001). TypeScript only renders.
   - The alternative was a structured `conditioning` field in `numerical_quality.cases[]`. It was not taken for v0.1, for these reasons:
     - it changes the persisted result schemas, including a reviewed input of the dormant retained profile;
     - it changes priced `product_physics` source, which requires D3 and re-registration;
     - it regenerates every committed result fixture.

     It remains the clean follow-on if the text seam proves brittle.
3. **"Digits lost" is stated as an estimate of the worst case.**
   - k = ⌈−log₁₀ rcond⌉, shown as "up to about {k} of the roughly 16 significant digits of working precision may be lost".
   - It is shown only where rcond parsed and 0 < rcond ≤ 1.
   - No "digits remaining" is promised.
4. **Combinations** take the worst state of the load cases they name.
5. **Stiffness contrast is not added in v0.1.** The engine does not compute it, and a desktop-side computation would re-derive section stiffness outside the engine. The cause shown is whatever the existing diagnostics name.

## 3. States and wording

The label is **"Numerical health"**. Each load case or combination has one state:

| State | When | Badge | Headline |
|---|---|---|---|
| good | `checks_passed` | green | "Numerical health: good" |
| sensitive | `sensitive` | red | "Numerical health: sensitive" |
| failed | `unresolved`, `failed`, or a failure code | red | "Numerical health: solve failed" |
| not assessed | `not_assessed`, no integrity diagnostic, or a non-linear case without the report | grey | "Numerical health: not assessed" |

**Model level.** The result header and the shell chip show the worst state over the result's sets, in the order failed, sensitive, not assessed, good. With a count, the header reads, for example, "Numerical health: sensitive (2 of 5 load cases)".

**The existing chip.** Where the reader's only non-Current reason is numerical quality (`NUMERICAL_INTEGRITY_NOT_QUALIFIED`), this chip replaces the misleading "Needs recompute". "Needs recompute" stays for the other causes: no live invocation, a saved result, or a changed model.

**Detail texts.** Every text is one module, `apps/desktop/src/features/results/numericalHealthText.ts`. Values in braces are filled in.

- **good:** "The solver's residual and conditioning checks passed for {case}. These checks look for numerical trouble; they do not measure the error, and they say nothing about whether the model, its inputs or its design are right."
  - When rcond parsed, it adds: "Estimated conditioning: up to about {k} of the roughly 16 significant digits of working precision may be lost."
- **sensitive:** "{case} is numerically sensitive. Its results are kept for inspection, but they cannot be exported or used by rule checks. Solving again will not change this; the model needs attention." Then one cause sentence, from the first that applies:
  - **conditioning** (rcond < √ε): "The stiffness is poorly conditioned: up to about {k} of the roughly 16 significant digits of working precision may be lost. This often comes from a very short or very stiff element next to flexible ones, stiffnesses that differ by many orders of magnitude, or a weakly restrained direction."
  - **load fidelity** (`LOAD_CONTRIBUTION_ABSORBED`): "Some applied loads are lost in rounding next to much larger loads at the same points ({sources})."
  - **formation** (K-D5 or S11-G): "Some stiffness or load terms are formed with rounding noise comparable to their size ({rows})."
  - **otherwise:** "The solver's checks flagged a loss of numerical precision."
- **failed:** "The solver could not produce reliable results for {case}." Then one cause sentence:
  - mechanism: "The model can move without resistance (a mechanism): add or correct supports so every direction is restrained."
  - negative energy: "An element or support contributes negative stiffness: check its properties."
  - unresolved: "The solution did not satisfy the equations to working precision even after refinement. This often comes from extreme stiffness contrast or a near-mechanism."
  - assembly unresolved: "The assembled stiffness could not be formed to working precision."
  - otherwise: "See the solver diagnostic for details."
- **not assessed:** "The solver did not assess the numerical quality of {case}."

**Every non-good state** links to its integrity diagnostic in the Diagnostics panel, "Show solver diagnostic". Located rows that the diagnostic names (`{node}:{DOF}`, load sources) are listed and are selectable through the existing selection path. A row label is shown as its node id and direction ("node N12, rotation about Z"), not a matrix index.

**Words the texts never use:** "verified", "certified", "bound", "guaranteed", "valid", "approved", "compliant" or "correct". The texts use "checks passed", "sensitive" and "may be lost". GEN-13 scans the module, and `tests/test_claims_language_surface.py` pins it.

## 4. Placement

- **Results panel.** The existing `numerical-result-standing` line becomes the health header for the generic and exact routes. Each load case and combination row shows its badge. The text "Numerical checks do not establish engineering correctness." is replaced by the state texts' boundary sentence.
  - The retained, pressure-source and unsupported texts are unchanged, apart from the health badge of the ordinary cases.
- **Shell chip.** It shows the model-level worst state (§3).
- **Solve panel.** The diagnostics tile is unchanged and gains the header badge.
- **Diagnostics panel.** It is unchanged. The links in §3 open it at the integrity diagnostic.

## 5. Exports and reports

- **Admission is unchanged.** A sensitive or failed result is already not Current, and every result output refuses it. So only good results (and not-assessed ones, where admitted today) reach exports.
- **Result JSON and stress-neutral exports** are unchanged. They already carry `numerical_quality` and the integrity diagnostics verbatim.
- **The report and rendered report** gain one line per load case and combination, placed before the diagnostics section: "Numerical health: {state} — {first sentence of the state's text}{; estimated conditioning: up to about {k} of 16 digits may be lost}."
- **Handoff, local-fea and native-package** gain the same per-case line in their notes. Where they carry `numerical_quality` already, the line is formed from it.
- **The report package** keeps refusing fresh results.

## 6. Claims registry (owner text)

The registry gains `NQ-HEALTH`. It registers:

- the display forms "Numerical health: good", "…: sensitive", "…: solve failed" and "…: not assessed", and the header's count form;
- the boundary sentence "These checks look for numerical trouble; they do not measure the error, and they say nothing about whether the model, its inputs or its design are right."

§2.1 rule 3 is amended to say that these forms are numerical-quality indicators, not analysis statuses or evidence-status labels. No ScopeOfWork or PRD text changes: the indicator implements the existing PRD FR-SOL-015 and SPEC §5.5 ("conditioning warnings must be reportable") within DEL-07-05's results-viewer contract.

## 7. Verification

- **Rust.** `numerical_health_json` has unit tests over synthetic messages: missing, malformed, extra or reordered fields, and the anchors. A fixture-wide test over every committed result fixture with an integrity diagnostic asserts:
  - a parsed rcond for every `checks_passed` and `sensitive` case of the dense and sparse kernels;
  - a state equal to `solve_quality`.

  This binds the seam to the producer's text. A producer change that breaks the parse fails it.
- **Desktop.**
  - The `numericalHealth` adapter and state tests: every code and quality, the combination rule, and the chip replacement only for the numerical-quality reason.
  - Text tests: the wording module and GEN-13.
  - Report and notes line tests on the emitted documents.
  - The full `desktop-test` with the wasm engine rebuilt, compared with main.
- **Python.** The full registered `piping-pytest` with the binary presets for the head, compared with main.
- **Native smoke.** The open path is unchanged: there is no new job and no new solve. So the native smoke and dist e2e are not required, but a manual or native-smoke observation of the badge on the milestone model is recorded if the slot is free.
- **Review.** Dual review, because "digits lost" carries numerical meaning: an independent Claude reviewer and Codex, on the final head.

## 8. Unchanged

- The engine, the persisted result formats, every threshold and every reader verdict.
- Output admission.
- The retained-precision method (dormant, owner-held).
- G10 and the frozen UI benchmark.
