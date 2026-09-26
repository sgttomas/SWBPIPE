# V1-S11 — containment of cancelled load contributions

D1 (TASK), 2026-09-26, first deliverable of revision 2. It answers the T3 manager's request and ROOT's rulings of 2026-09-26: ruling 2 in `T3/ROOT_RULINGS_V1.md`, and the hard constraint relayed afterwards.

- **Basis.** T3 branch at `065c9ff60`, product source `c61a540ea`, T1 `f3270ea79` (read only).
- **Paths.** `P/`, `PP`, `FK` and `SA` are as in `DESIGN.md`.
- **Scope of work.** Read-only on product source. I ran three standard-library Python probes (§7). No build ran, and no Git write was made.

## 1. Answer in brief

1. **The map is right, with six corrections and additions (§2).** The retained-source exact context does use individual force terms and sums them exactly. `source_recovery.rs:589-595` does re-fold the loads in binary64 and demand bit-equality with the actual force. Two points the map lacks:
   - A binary64 fold of one or two terms is always correctly rounded, so only a DOF with three or more contributions can go wrong.
   - No committed product JSON puts more than one nodal load on a DOF within a case.
2. **Nothing that runs live in the product can land before T1 merges without breaking ROOT's constraint (§3).**
   - **C1** changes the folded values. A case whose recovery C1 knocks out can then turn an exact-route (physics-source-1) invocation into an invocation-level `Err`, and can strip eligibility from a source-blocks-1 envelope.
   - **C2** raises a blocking finding, which blocks the whole envelope, including correct retained-source cases.
   - Every other lever is in `PP`, which T1 changes.
   - **What can land before T1 merges: the dormant, T1-disjoint half (slice S11-K).** It is an exact accumulator, an exact load audit, and new library methods, with kernel tests. It changes no product behaviour.
3. **Chosen: C3-full as the first facade slice after T1 merges (slice S11-F).**
   - Every force contribution goes into one per-case exact ledger. The solve's force vector is the correctly rounded net per DOF.
   - Retained source compares against that same rounded net, so it is never knocked out.
   - M03's intended-action audit sees the exact per-DOF load.
   - It repairs rather than demotes. So it demotes no case, and it adds no path to the composite `Err`.
   - **A fallback, C3-detect,** keeps the values and only demotes flagged cases. It must land with or after D2's S-D (§3.4).
4. **What a detected loss does.** The case is marked **Sensitive**, never refused. It carries the existing `NUMERICAL_INTEGRITY_SENSITIVE` warning and a new warning, `LOAD_CONTRIBUTION_ABSORBED`. A refusal would be a blocking diagnostic, and on main any blocking diagnostic blocks the whole envelope (`PP:1430-1432`, `:1497-1499`). Under C3-full a loss can only come from a producer not routed through the ledger, so the audit acts as a guard.
5. **Scale.** The existing M03 componentwise scale of the row that carries the load: `d_i = |f_i^exact| + Σ_j |K_ij|·|u_j|`. That is the net load plus the row's own structural force scale, never the gross load. It applies to free and constrained rows. Force rows are in N, moment rows in N·m. The target is M03's unchanged `64·γ(m_i)`.

## 2. Reach: the map confirmed, corrected and extended

| # | Map statement | Finding at `c61a540ea` |
|---|---|---|
| R1 | Nodal loads are folded in binary64 in contribution order | **Confirmed.** `LoadApplication::global_load_vector` (`P/core/loads/primitive_loads/src/lib.rs:1401-1409`), called at `PP:1810`. T1 does not change `P/core/loads/**` |
| R2 | Other producers add into the same entries | **Confirmed**, and one site is added. The complete list of force-accumulation sites in `PP`: `:1840` exact-pressure assembled loads, `:7759`, `:7797` (element uniform-load equivalents), `:8000-8001`, `:8068-8069` (thermal and thrust axial pairs), `:8035-8036`, `:8040` (curved thrust and wall loads), `:8090`, `:9808` (constant effort). T1 adds its 0.4.0 eigen-load equivalents to the same vector |
| R3 | The M03 audit sees only the folded force | **Confirmed.** `audit_intended_action` starts from `system.force[i]` (`FK/structural.rs:513-538`). Stiffness contributions are audited exactly; loads are not |
| R4 | Retained source carries each nodal load as an exact term | **Confirmed.** `source_recovery.rs:584-588` pushes one `ForceContribution` per nodal contribution. These are passed as `ForceBasis::IdentifiedContributions` (`:201`, `:223`, `:1097`). `exact_boundary.rs:373-392` sums them exactly as `Expansion`s, and the solve uses those expansions (`:529`, `:565`, `:983`). **Correction:** the exact context's own coverage check accepts `system.force` equal to either the expansion's naive projection or the ordered binary64 fold (`:387-392`). A correctly rounded exact sum can differ from both |
| R5 | `source_recovery.rs:589-595` re-folds in binary64 and compares bits | **Confirmed.** `folded_force[dof] += value` in contribution order (`:580`), then bit-equality with `input.force`. A mismatch returns `SourceMismatch("complete ordered load fold differs from actual force")`, recorded per case as `SOURCE_BLOCK_RECOVERY_UNAVAILABLE` (`PP:1885-1897`) |
| R6 | `source_receipt.rs:154` recomputes the vector | **Confirmed.** Replay rebuilds the force with `global_load_vector` and re-runs recovery, so it moves together with any change to that method |
| R7 | Routes | **Confirmed:** preview-physics-1, physics-1 and T1's load-reference-1. **Added:** the nonlinear active-set loop receives the same folded `force` (`PP:1985` → `append_nonlinear_support_loop_results`). **Added:** loads on restrained DOFs. A fold error there enters the published reaction directly (`PP:2131-2140`), and no free-row gate sees it |
| R8 | Retained source "likely correct" | **Correct when it runs,** but it runs only when the ordinary attempt is Sensitive or rejected (`PP:1879-1885`). For the S11 path, an ordinary-stiffness model that is Passed, it never runs today |
| R9 | (new) Which DOFs can be affected | IEEE addition rounds the exact sum of its two operands correctly, and folding from 0 makes the first addition exact. **So a fold differs from the correctly rounded net only on a DOF with three or more contributions.** Contributions from every producer count, not only nodal loads |
| R10 | (new) Committed fixtures | A scan of every JSON file under `P/` outside `execution/` found 183 load cases in 117 files. **None puts more than one nodal load on one (node, direction)** (`_run_records/scan_load_fold.*`). The scan does not cover element-equivalent and other producers at shared nodes |

**Arithmetic.** An ordinary-stiffness 2 m cantilever, solved exactly with the folded load. The screen of §5 uses the exact load; the probe is `_run_records/probe_s11_audit.*`.

| Tip contributions (authored order) | Net-load relative error | Tip response error, on its own scale | Screen | Worst ratio / target |
|---|---|---|---|---|
| UY (1e5, 0.3, −1e5) N | 9.7e-12 | 9.7e-12 | flagged | 28 |
| UY (1e6, 0.3, −1e6) N | 1.6e-10 | 1.6e-10 | flagged | 455 |
| UY (1e7, 0.3, −1e7) N | 2.5e-9 | 2.5e-9 | flagged | 7.3e3 |
| UY (1e8, 0.3, −1e8) N | 9.9e-9 | 9.9e-9 | flagged | 2.9e4 |
| RZ (1e5 … 1e8, 0.3, −G) N·m | same as the force rows | same | flagged | same |
| UY (1e80, 1e-8, −1e80) with UZ 2e-8 (V1 check L) | 1.0 | 0.5 | flagged | 2.4e13 |
| UY (1e80, 1e-8, −1e80), loads only | 1.0 | 1.0 | flagged | 2.4e13 |
| UY (1e7, 0.3, −1e7) with RX 5 N·m elsewhere | 2.5e-9 | 2.5e-9 | flagged | 7.3e3 |
| Control: (1e8, −1e8, 0.3) | 0 | 0 | passes | 0 |
| Control: two terms (1e8, 0.3) | 3.0e-17 | 3.0e-17 | passes | 8.7e-5 |
| Control: (0.1, 0.2, 0.3) | 1.4e-16 | 1.4e-16 | passes | 4.1e-4 |
| Control: same-sign (1234.5, 987.25, 55.125) | 0 | 0 | passes | 0 |
| Control: mild cancellation (1000.1, −1000, 0.05) | 9.3e-17 | 9.3e-17 | passes | 2.7e-4 |

The response error equals the net-load error, because the response is linear in the net load. Against 1e-9 on the response's own scale, the 1e7 and 1e8 rows fail. The screen flags them, and the 1e5 and 1e6 rows too (§5).

## 3. Options against ROOT's constraint

**ROOT's constraint.** Wherever an option meets the fold check (`source_recovery.rs:589-595`) or `source_receipt.rs:154`, it fails closed per case. Knocking out correct retained-source recovery is acceptable only as a declared, temporary refusal with a named diagnostic. It must never cause a silent loss, an invocation-level `Err`, or a blocked envelope for other cases.

### 3.1 C1 — exact per-DOF sum of nodal loads in `global_load_vector` (T1-disjoint)

What happens once the vector changes on a DOF with three or more nodal contributions whose fold is not correctly rounded:
- **Live solve.** The fold check fails, so recovery is refused for that case (`SOURCE_BLOCK_RECOVERY_UNAVAILABLE`, stage text "complete ordered load fold differs …"). Alone, that is per case.
- **Exact route, more than one case.** If another case in the invocation is selected, the physics-source-1 receipt needs every case qualified (`source_receipt/composite.rs:938-947`; D2 §3.1). The refused case makes the receipt invalid, and the value route returns `Err("SOURCE_BLOCKS_FINALIZATION_FAILED")` (`PP:1249-1253`). **That is an invocation-level `Err`, which the constraint forbids.**
- **Legacy route, more than one case.** The source-blocks-1 receipt becomes `partial`. Readers admit only `qualified` (`result_export/src/source_blocks.rs:1225-1245`), so the other, correctly selected cases lose eligibility. **That is a loss of standing for other cases.**
- **Replay** (`source_receipt.rs:154`) stays consistent, because it uses the same method.
- **Coverage.** Nodal loads only. Element, thermal, thrust, pressure and constant-effort contributions remain folded in binary64 in `PP`.

**Verdict: rejected for landing on its own.** The failure scenario is narrow, but it exists, and the constraint is absolute.

### 3.2 C2 — a blocking `primitive_loads` finding when the fold differs (T1-disjoint)

- `PP` maps every load finding to a blocking `LOAD_INPUT_INVALID` (`PP:1754-1778`). The case loop then returns `blocked_envelope` for the whole invocation (`PP:1430-1432`, `:1497-1499`).
- That blocks the affected case and every other case. It also blocks the retained-source answer that would have been correct, because the exact context sums the terms exactly (R4).
- It needs a bound on the load side alone, without the row's structural scale, so it would be a new tolerance.

**Verdict: rejected.** It produces a blocked envelope for other cases, and it refuses correct results.

### 3.3 C3-full — one exact ledger, rounded once, plus the audit (chosen)

Every force producer in `PP` records identified contributions in a per-case `LoadLedger`: source id, global DOF and binary64 value, summed per DOF as an exact expansion. The case's force vector is each DOF's correctly rounded exact sum.
- **Bit-identity.** Where the binary64 fold was already correctly rounded, the value is unchanged. That covers every DOF with at most two contributions, and every nodal case in the committed JSON. Where it was not, the value becomes the correctly rounded net load. That is the repair.
- **Retained source is never knocked out by the change.**
  - `source_recovery.rs:589-595` compares against the correctly rounded exact sum of its own terms, instead of its binary64 re-fold.
  - The exact context's coverage check (`exact_boundary.rs:387-392`, `FK`) also accepts the correctly rounded exact sum.
  - `source_receipt.rs:154` builds the force through the same ledger function.
  - The checks therefore agree by construction. If they ever disagree, recovery still fails per case with the existing named `SOURCE_BLOCK_RECOVERY_UNAVAILABLE` and its stage text. That is the declared per-case refusal ROOT allows.
- **No demotion.** Loads become correct, so no case moves from Passed to Sensitive. The composite rule and source-blocks-1 standing therefore meet nothing new.
- **Audit.** `AssemblyEvidence` gets the ledger's exact per-DOF expansions (§5). With correct values it passes. It is the guard that catches a producer left outside the ledger and the mutation "fold in binary64 again".
- **Nonlinear loop.** It consumes the same force vector, so it is repaired too. The audit inside the loop's linearized solves waits for T5 coordination (see `DESIGN.md` §4.6).

### 3.4 C3-detect — the audit only, values unchanged (fallback)

- The same audit, but the force stays the binary64 fold. Committed bytes change nowhere; a flagged case is demoted to Sensitive.
- A demoted case in an exact-route invocation where another case is selected, and where its own recovery is unavailable (for example, it also has an element load), meets the composite `Err` of `PP:1249-1253`. This is the existing trigger class of `STAGE0_MAP.md` §2.6.
- **C3-detect therefore lands only with or after D2's S-D**, the pre-0.4 port of T1's SF-1.
- **Use it** only if ROOT wants the first live slice to change no force value.

### 3.5 When each piece can land

| Slice | Files | T1 | Behaviour change |
|---|---|---|---|
| **S11-K** (before T1 merges) | `FK/structural.rs`: public `exact_rounded_sum`, and the audit taking exact per-DOF force expansions through a new entry point; `StructuralSystem`'s layout is unchanged. `FK/structural/exact_boundary.rs`: the coverage check also accepts the correctly rounded exact sum. `SA`: `AssemblyEvidence::with_force_terms`. `P/core/loads/primitive_loads/src/lib.rs`: new `exact_global_load_vector()` and `nodal_force_terms()`; `global_load_vector` itself untouched | disjoint | None. Product callers are unchanged |
| **S11-F** (the first facade slice after T1 merges, ahead of F1) | `PP` (ledger at every §2 R2 site plus T1's eigen sites; `AssemblyEvidence::with_force_terms`; the Sensitive mapping), `source_recovery.rs:580-595`, `source_receipt.rs:154` | T1-touched | C3-full |

## 4. What a detected loss does

- **The kernel.** When the load audit fails, `finish_checked_factor` returns `Ok` with `quality = SolveQuality::Sensitive`, not `Err`. It returns a separate `LoadFidelityReport` beside `StructuralReport`, so the `Debug`-published report is byte-unchanged when nothing is flagged. The report holds, per flagged row: the global DOF, the exact net and folded values as bit strings, the ratio, the target, and the contributing source ids.
- **The facade.**
  - The existing integrity diagnostic is `NUMERICAL_INTEGRITY_SENSITIVE` (warning).
  - A new warning, `LOAD_CONTRIBUTION_ABSORBED`, has `affected_refs` = the case id and the contributing load ids. Model entity ids are allowed by ROOT's F-1 amendment. Its message names the DOF through `integrity_dof_map`.
- **Standing.** Sensitive rows stay inspectable and are withheld from Current, rule use and export by today's standing. Today's standing is envelope-level: `numerical_quality.status` must be `checks_passed` (`result_export/src/semantic_contract.rs:405-409`). So the other cases of that invocation also lose Current. This is the same consequence any Sensitive case has today; their rows are kept, and the envelope is not blocked. Per-case standing is D2's.
- **Recovery.** Sensitive triggers retained-source recovery (`PP:1879-1885`). For in-scope nodal-load cases, recovery then publishes the correct answer from the exact terms, as a selected case.
- **Why not refuse.** A refusal would be a blocking diagnostic, and it would block every case in the invocation (§3.2).

## 5. The scale that decides whether a loss matters

- **The predicate.** It is M03-INTEGRITY-v1's intended-action predicate, unchanged in form. The only change is that the load term is the exact per-DOF sum. For each row i that carries two or more contributions, free rows and restrained rows alike:
  - `ratio_i = |Σ_j K_ij·u_j − f_i^exact| / d_i`, with `d_i = |f_i^exact| + Σ_j |K_ij|·|u_j|`;
  - flagged when `ratio_i > 64·γ(m_i)`, with m_i counted as `audit_intended_action` counts it today.
  - On restrained rows the residual is the reaction. There the check compares the published reaction's load term with the exact load term, `|f_i^fold − f_i^exact| ≤ 64·γ(m_i)·d_i`.
- **The scale is the row's own componentwise scale:** the net load plus the magnitudes of the structural forces meeting at that DOF. Force rows are in N, moment rows in N·m.
- **Not the gross load.** A gross scale would never flag, because the fold error is always at most about `m·u·Σ|contributions|`.
- **Not 1e-9 on the response.** That would be a new tolerance.
- **The consequence, stated plainly.** The screen flags net-load errors from about 1e-13 of the row scale upward. In the probe, the 1e5 : 0.3 row (9.7e-12) is flagged even though it is inside 1e-9. That is conservative, consistent with how M03 already treats absorbed stiffness contributions (`FK/structural.rs:410-414`), and resolved by recovery where in scope. Under C3-full no committed or correctly assembled case reaches the screen.
- **No new constant.** The factor 64 and γ are the reviewed M03 screen.

## 6. Committed bytes and retained-source recovery

- **Fixture bytes (C3-full).**
  - **Nodal part:** byte-identical for every committed JSON request, because no case has two or more nodal loads on one DOF (§2 R10).
  - **Other producers:** a DOF with three or more contributions whose binary64 fold was not correctly rounded (for example, element equivalents from two adjacent spans plus a nodal load) changes by the rounding difference. S11-F runs every committed request through the base and the candidate in both modes, following the T0R and D2 practice. It lists every changed output with its DOF and reason. Only derived producer fixtures may be regenerated, and only by the actual producer.
  - **Frozen references, historical raws and hashes** do not change.
- **C3-detect** changes no value. Only flagged cases change: status, diagnostics and standing.
- **Retained-source recovery.** It is never knocked out by S11-F (§3.3). Its selected answers are unchanged, because it already solved from exact terms. Where a force bit changes, the receipt's recorded force changes with it, and replay follows through the same ledger function.

## 7. Tests and controls

**S11-K (kernel and library, before T1 merges).**
1. `exact_rounded_sum` equals the correctly rounded `Fraction` sum. Use seeded random terms, the ≥3-term cancellations of §2, and the edge classes of `DESIGN.md` §4.11 (ties, overflow-adjacent values, subnormal nets).
2. `exact_global_load_vector` equals `global_load_vector` bit for bit on every DOF with at most two contributions. It differs, and equals the correctly rounded exact sum, on (1e8, 0.3, −1e8).
3. The audit with force terms, on the probe's cantilever (§2 table):
   - flags every G ∈ {1e5, 1e6, 1e7, 1e8} row, for force and for moment, and V1's check L in both forms;
   - passes every control;
   - passes with bit-identical `StructuralReport` output when nothing is flagged.
4. A load on a restrained DOF, (1e8, 0.3, −1e8) N at the root: flagged on the reaction row.
5. The exact context accepts a force vector equal to the correctly rounded exact sum and still refuses a corrupted one.

**S11-F (product, after T1 merges), both modes, through `run_linear_static_preview_value_with_mode`.**
1. The RF-CANCEL references (`T3/TASK_BRIEFS/R1_ADDENDUM_CANCEL.md`), once V2 has checked them: every case meets `|obs − exp| ≤ 1e-9·max(|exp|, scale)` with R1's scales. The binary64-fold negative controls fail that predicate in the orders in which they differ.
2. On main, the same requests publish `NUMERICAL_INTEGRITY_CHECKS_PASSED` with the fold's answer. That is P1's detection baseline.
3. The ordinary route, the exact route and 0.4.0 (after T1): element-equivalent cancellation at a shared node (RF-CANCEL case 4) is repaired as well.
4. Retained source: an N05-class Sensitive case with (1e8, 0.3, −1e8) at the tip is selected, and its answer equals the exact-term answer.
5. A multi-case exact-route invocation: the selected case A, plus case B carrying the cancelling loads and an element load (so out of retained-source scope). **No `Err`, and no blocked envelope.**
6. Byte identity over every committed request, with the changed list attached.
7. Mutations that must fail:
   - restore the binary64 fold in `PP` → test 1 and the audit flag (under C3-full, the mutated case must become Sensitive with `LOAD_CONTRIBUTION_ABSORBED`);
   - drop restrained rows from the audit → test K4 misses;
   - leave `source_recovery`'s binary64 re-fold in place → test 4 loses selection with a mismatch;
   - route one producer, for example thermal, outside the ledger → RF-CANCEL case 4 variant fails.

## 8. Run records (`_run_records/`)

- **`probe_s11_audit.py` → `probe_s11_audit.stdout.json`.** It imports the binary64 local stiffness from `probe_skew_precision.py`.
- **`scan_load_fold.py` → `scan_load_fold.stdout.json`.** Run from `P/` with argument `.`.
- Python 3.11.15, run at `nice 19`, each under 1 s.
- **Not done.** No product build, and no product reproduction; that is P1 and S11-F. Element-equivalent contributions in the committed fixtures were not scanned.
