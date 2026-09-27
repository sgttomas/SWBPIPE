# S11-G receipt-coverage refusal: path 2 and path 1 constructions (for D1's revision 2.2)

I5 (Type 2 TASK), 2026-09-27. Requested by the T3 manager after ROOT's "S11-G ruling 3, revision 1" (numerics `70907245b`).

> **Historical record (added at hand-back).** This note describes the candidate as it stood under revision 2.1, before any cargo run.
> - Revision 2.2 (G-1 to G-3, erratum E-1) has since removed path 2 and path 1's load-row variant. T18 and T19 confirm this by run.
> - The characterization test named below was replaced by T18.
> - Path 1's R-b′ variant remains the disclosed residual. See `RETURN.md` §5a and §6a, and `CHANGE_RECORD.md`.
> - The line numbers below refer to the candidate at that time.

**Status of the evidence.**
- Everything below is traced from source.
- **No cargo has run yet** (the token is still held elsewhere). The characterization test `s11g_tests::ruling3_characterization_multi_case_captured_receipt_residual` builds path 2 and is written, but it has not been compiled or executed.
- Where a claim depends on a run, it is marked **(to be confirmed by the run)**.

**Trees.**
- *Candidate* = `<wt>/s11g` working tree: HEAD `3d844fea4` plus I5's uncommitted S11-G changes, which implement rev 2.1 as specified, including the rev 2.1 routing gate.
- *Base* = HEAD `3d844fea4` itself: main at `72d5ff864` plus merges, no S11-G.
- Line numbers are `PP` = `core/product_physics/src/lib.rs` in each tree, and `SR` = `source_receipt.rs`, `SRec` = `source_recovery.rs` (the same file in both trees).

**The end state of both paths.** The end state is **not** an envelope with a blocking diagnostic.
- `finalize_for` fails. PP pushes `SOURCE_BLOCK_RECOVERY_FINALIZATION_FAILED` (blocking) onto the envelope (candidate PP:2138, base PP:2057). The envelope keeps the source-blocks semantic identity and no receipt.
- The captured entry `run_linear_static_preview_value_with_mode` then returns **`Err("SOURCE_BLOCKS_FINALIZATION_FAILED")`** (candidate PP:1565-1569, base PP:1484-1488). **No envelope is returned.**
- **The consumers of the captured entry propagate the `Err`:**
  - the desktop's `solve_preview_mechanics_with_mode` (`apps/desktop/src-tauri/src/lib.rs:1562`, `?`);
  - headless `run_preview_model_value_mode`, via `run_preview_with_producer` (`core/runner/headless/src/lib.rs:739`, then `let mechanics = produce()?;` at `:867`).
- The typed entry `run_linear_static_preview_with_mode` has no capture, so it has no receipt and no refusal.
- **Load-state (0.4.0) captured invocations take ROOT CP3 SF-1's republication instead** (PP `run_linear_static_preview_captured`, candidate PP:1586ff). The join failure is recorded, and the invocation is republished on the ordinary route without a receipt; it is not refused. So both paths below are **pre-0.4 captured invocations** only.

---

## Path 2: the routing gate on an already-Sensitive case (reachable in ordinary models)

### The model

This is N05 (`fixtures/product_preview/numerical_sensitive_torsion_model.json`): a 2 m cantilever along x.
- **Section:** OD 0.2 m, wall 0.01 m; E = 200 GPa, G = 80 GPa.
- **Supports:** anchor `root` (UX, UY, UZ, RY, RZ), and a soft torsion spring `soft` at `root` (RX, 1e-4 N·m/rad). There is one modulus basis.

It has two load cases (`s11g_tests::n05_request`):
- **Case A (`case`):** tip torques (1e8, 0.3, −1e8) N·m on RX. This is S11-F's F4 case. Nodal only.
- **Case B (`case-b`), `n05_tip_noise`:**
  - a uniform load W = 1e8 N/m along global y on `pipe`;
  - a nodal −1e8 N at `tip` along global y, which cancels the fixed-end transverse term W·L/2 exactly (net 0);
  - a nodal (1e8/3 + 0.2) N·m at `tip` RZ.

  So the tip RZ row's intended net is about 0.2 N·m. Its formed fixed-end term, rotation_j ≈ −W·L²/12, carries a formation defect of order u·3.3e7. That is far above the row's threshold, about 1e-9·max(0.2, S\*), with S\* = max(M, L·F) = 0.2 because the free UY net is exactly 0.

### Why each case is Sensitive and source-eligible

- **Sensitive.** The kernel's quality is `Sensitive` when rcond < √ε or the load audit flags (FK `structural.rs:1400`, `quality: if rcond < f64::EPSILON.sqrt() || load_fidelity.is_some()`).
  - N05's soft RX spring makes the stiffness ill-conditioned. Both cases share the one basis's stiffness, so both reports are Sensitive.
  - Case A's Sensitive report is what routes F4 to retained-source recovery, which F4 asserts is selected.
- **Source-eligible (base predicate, base PP:2592).** Captured entry, no nonlinear supports, no combinations. Both cases qualify.
- **Retained-source scope.** Case A is nodal only; recovery accepts it and F4 shows it selected. Case B has an element (uniform) load, which recovery refuses: `SRec:590-601`, `unsupported("authored non-nodal load family, including zero-valued inputs")`.

### Trace on the candidate (rev 2.1 gate), case B

1. **The formed term.** `add_uniform_element_loads` pushes the straight uniform equivalent with `push_formed(…, Formation::Exact{…}, 0.0, false)`.
2. **The guard fires.** The load-row guard `formation_guard::load_row_finding` (candidate PP:2602) fires at `tip:RZ`: `|A_net| + 12B − 12T0 > 0`. **(To be confirmed by the run;** the test asserts it as a precondition.)
3. **The ordinary attempt is Sensitive.** `ordinary_attempt = OrdinaryAttempt::passed(solver_mode, &report, …)` (candidate PP:2692-2693) records outcome `"sensitive"` from the report (`SR:454-476`).
4. **Recovery is needed.** `needs_source_recovery = true`, because the report is Sensitive (candidate PP:2702-2705).
5. **The gate blocks it.** `source_eligible(capture.is_some(), …, load_row_finding.as_ref())` (candidate PP:2711-2716) is **false**, because the load-row finding is `Some` (rev 2.1 §3.5, `&& load_row_finding.is_none()`). **No retained-source attempt is made** (PP:2717). `selected_source = None` and `source_failure = None`, and no `SOURCE_BLOCK_RECOVERY_UNAVAILABLE` diagnostic is pushed.
6. **The case has no receipt entry.** In the per-case finalization, case B is not selected, has no failure, and is not exact-pressure, so PP calls `FinalizedSourceBlockCase::ordinary(capture, …, ordinary_attempt, …)` (candidate PP:3855).
   - `ordinary()` returns `Err("ordinary selection not p1 checks-passed")`, because the outcome is `"sensitive"` (`SR:716-718`).
   - The `Err` branch (candidate PP:3866) records nothing, because case B is not source-selected. So `source_case = None`: **case B has no receipt entry.**
7. **Coverage fails.** At envelope level, case A is source-selected (candidate PP:1943), so PP calls `FinalizedSourceBlockReceipt::finalize` (candidate PP:2117-2124). `finalize_for` checks `cases.len() != request.model.load_cases.len()` and returns `Err(bad("invocation case coverage"))` (`SR:955-961`: 1 case entry against 2 load cases).
8. **The refusal.** The blocking `SOURCE_BLOCK_RECOVERY_FINALIZATION_FAILED` is pushed (candidate PP:2138). The captured entry returns `Err("SOURCE_BLOCKS_FINALIZATION_FAILED")` (candidate PP:1565-1569).

### Does the same model already refuse on main (no guard)? No, from the code.

- **Step 5 differs.** On base, `source_eligible` is true (base PP:2592), so recovery is attempted for case B (base PP:2593).
- The attempt fails with `unsupported("authored non-nodal load family …")` (`SRec:590-601`). The `Err(failure)` arm pushes `SOURCE_BLOCK_RECOVERY_UNAVAILABLE` (info) and sets `source_failure` (base PP ~2626-2640).
- The per-case finalization then calls `FinalizedSourceBlockCase::failed(…, failure, "diagnostic:source-recovery:<case>", …)` (base PP:3684), which gives an **unsupported** receipt entry.
- Coverage holds, the receipt finalizes (case A qualified, case B unsupported), and there is **no refusal**.
- **This is exactly S11-F's F6 shape** (`s11f_tests::f6_multi_case_pre_0_4_invocation_never_errs_on_cancelling_loads`, `s11f_tests.rs:1236`): a case B with an element load beside the selected F4 case. F6 asserts no blocking diagnostic and a receipt, and passes on main.
- Path 2's model differs from F6 only in case B's loads. They make the load-row guard fire, and only that engages the gate.

**The cause is the gate's removal of the attempt.** The withheld attempt would have been refused by retained-source scope anyway, because the load-row guard can fire only where a formed non-self-equilibrated term exists (SF-3), and that family is outside retained scope. That attempt is what produces the unsupported receipt entry today. **(Observation for D1; no design proposal from I5.)**

**The recovery guard R-b′ has no path 2.** R-b′ acts after routing and never enters `source_eligible`. On an already-Sensitive case it is a no-op by the no-op rule.

---

## Path 1: a case demoted from Passed beside a source-selected case (disclosed residual; not constructed)

### The model (from the code; not run)

The two cases must have different stiffness, because Sensitive otherwise follows the invocation-wide rcond (FK `structural.rs:1400`). A pre-0.4 invocation gives per-case stiffness through modulus bases:
- `modulus_basis_ref` / `modulus_basis_temperature` on a load case selects a material temperature point;
- `run_linear_static_preview_captured` builds one `BuiltModel` and stiffness per distinct basis (`basis_solve_states`, candidate PP:1781-1920, base PP:1790-1840 region, `materials_for_modulus_basis`);
- the receipt replays per-case bases (`SR:296-298`, `composite.rs:390`).

A construction:
- **Case A:** on the base basis of an N05-like model (Sensitive, source-selected).
- **Case B:** on an invented temperature point whose E and G make the same model well conditioned (so its report is Passed). For example, E of about 3e2 Pa and G of about 1.3e2 Pa bring the element stiffnesses near the 1e-4 N·m/rad spring. Case B carries either a load-row formation defect (a UDL cancellation) or INPLANE-type rows for R-b′.

**Not verified:** that retained-source recovery selects case A in a multi-basis invocation. No run has been made.

### Trace on the candidate, case B

1. **The ordinary attempt is recorded as checks_passed.** The report is Passed, so `needs_source_recovery = false` and `ordinary_attempt` has outcome `"checks_passed"` (`SR:454-476`).
2. **The guard demotes after routing.**
   - A load-row finding: `source_eligible` is false, but no attempt was needed anyway. `append_integrity_report` demotes the case to `SENSITIVE` (candidate PP:1089-1091).
   - An R-b′ finding: the post-loop amendment demotes it (candidate PP:3752-3771).
3. **An entry exists.** `FinalizedSourceBlockCase::ordinary` succeeds (outcome `"checks_passed"`) and creates a qualified `ordinary_checked` entry (`SR:710-750`).
4. **The binding check fails.** In `finalize_for`, `case.ordinary.wire(index, …)` compares the envelope's `solve_quality` (`"sensitive"`, from the demoted diagnostic through `assessed_numerical_quality`) with the outcome `"checks_passed"`. It fails with `Err(bad("ordinary outcome changed"))` (`SR:536-543`). Were it reached, the `expected_code` check (`SR:556`) would fail too.
5. **The refusal.** The same end follows: the blocking diagnostic, then `Err("SOURCE_BLOCKS_FINALIZATION_FAILED")`.

### Does the same model refuse on main? No, from the code.

Without S11-G, case B stays `CHECKS_PASSED`. The wire check matches, and the receipt finalizes with case A qualified (source) and case B qualified (ordinary). There is no refusal.

---

## Summary for D1

| | Path 2 | Path 1 |
|---|---|---|
| Guard | load-row only (through the rev 2.1 routing gate) | load-row or R-b′ (demotion after routing) |
| Case B's ordinary report | Sensitive | Passed |
| Reachability | ordinary single-basis models (N05 plus a formation-noisy second case) | needs per-case modulus bases (not constructed) |
| Refuses on main? | no (F6 shape: an unsupported receipt entry) | no |
| End state (pre-0.4 captured) | `Err("SOURCE_BLOCKS_FINALIZATION_FAILED")`, no envelope | same |
| 0.4.0 captured | CP3 SF-1 ordinary republication, not refused | same |
| Test | `ruling3_characterization_multi_case_captured_receipt_residual` (written, not yet run) | none |
