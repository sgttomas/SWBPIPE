# R0 Calibration Notes — DEL-04-01 "3D frame stiffness kernel" (PKG-04)

**Run:** DELIVERABLE_CONCORDANCE_2026-07-11_1305 / R0_CALIBRATION
**Source state (all evidence reads):** frozen read-only worktree at commit `551f84ef6be656f1603ce0acfa5e3935aa9683c7`
**Method:** `plans/PLAN_2026-07-10_deliverable_implementation_reconciliation.md` §§3, 6, 7 (read at the frozen revision)
**Ledger:** `R0_CLAIM_CONCORDANCE_DEL-04-01.csv` — 35 claim rows
**Posture:** agent dispositions only; nothing here is an owner or engineering ruling. R0 was read-only: no test was rerun; verification counts are the recorded run-record values at the frozen SHA.

## 1. Sources read (repo-relative, frozen tree)

Deliverable kit and state:
- `execution/PKG-04_Solver Core and Numerical Methods/1_Working/DEL-04-01_3D frame stiffness kernel/` — `Specification.md`, `Datasheet.md`, `Guidance.md`, `Procedure.md`, `_CONTEXT.md`, `_STATUS.md`, `_REFERENCES.md`, `_DEPENDENCIES.md`, `Dependencies.csv`, `Review_Findings.csv`, `_REVIEW.md`, `MEMORY.md`
- `.../_run_records/` — full listing; read in depth: `TASK_RUN_2026-06-11_TP-D03-SPARSE-001.md` (via MEMORY summary), `WORKING_ITEMS_RUN_2026-07-10_TP-PMM-P1-CURVEDBEND-004.md`

Method/authority:
- `plans/PLAN_2026-07-10_deliverable_implementation_reconciliation.md` (full)
- `plans/PLAN_2026-07-09_physical_model_mechanics.md` (§§3–5)
- `execution/_Coordination/_DECISIONS/_REGISTER.md` (D-41, D-34 rows)
- `execution/_Decomposition/SOFTWARE_DECOMP.md` §12 (DEC-023, DEC-024, DEC-026, DEC-044, DEC-046, DEC-050, DEC-060, DEC-067, DEC-070)
- `execution/_Reconciliation/LifecycleCorrection/LIFECYCLE_CORRECTION_2026-05-11_2052/Lifecycle_Correction_Register.csv` (DEL-04-01 reversal row)
- `docs/_Registers/Deliverables.csv` (rows DEL-04-01, DEL-04-05, DEL-04-06)

Implementation / verification / validation:
- `core/solver/frame_kernel/{src/lib.rs, README.md, Cargo.toml}` (1873-line crate; 36 `#[test]`)
- `core/solver/{sparse_direct, curved_bend, nonlinear_integration, diagnostics, straight_pipe, linear_supports, nonlinear_supports, performance_harness}` (existence + key surfaces; diagnostics envelope fields read directly)
- `validation/hand_calcs/mechanics/` (inventory; read: `portal_frame_sway.md`, `cantilever_tip_force.md`; provenance blocks)
- `validation/benchmarks/mechanics/src/lib.rs` (fixture inventory, witness IDs, frame_kernel dependency, 33 tests)
- `validation/benchmarks/sparse_*.dec050/dec053.json`; `validation/evidence/sweeps/` (latest artifacts)
- `validation/witness/` (existence of content dictionaries/tooling)

## 2. Activation-basis observation (source-state)

The dispatch names activation "D-41/DEC-073". At the frozen SHA the register row `D-41` reads **`AWAITING_RULING`** and `DEC-073` does not exist anywhere in the frozen tree (register or `SOFTWARE_DECOMP.md` §12). This is consistent with the plan's own mechanics (the ruling lands on `main` after the tree froze), but it means **R0 cannot verify its own activation basis from its evidence tree**. The `_STATUS.md` bootstrap item still carries `(gated: D-41)` at the frozen revision. Recorded here as a method fact, not a defect finding.

## 3. Method ambiguities hit

1. **No `DECLARED_STATE` ClaimType.** The task contract says to extract "declared-state statements" as claims, but §6's `ClaimType` enum is `REQUIREMENT | ACCEPTANCE | EXCLUSION | IMPLEMENTED_UNMAPPED | REMAINING_WORK`. I folded declared-state statements (Datasheet TBD register rows, MEMORY Open TBDs, README Current Limitation) into the `DeclaredState` column of their nearest requirement/exclusion rows, and used `REMAINING_WORK` for the two evidence-backed-residual-omitted rows (C17, C18). A `DECLARED_STATE`/`STATE_ASSERTION` ClaimType (or explicit guidance to fold) would remove this judgment call.
2. **Column count: "20 columns" vs `PackageID` / `DeliverableID`.** §6 lists them in one table row; split, the ledger has 21 columns. I merged them into a single `PackageID/DeliverableID` column (value `PKG-04/DEL-04-01`) to honor the 20-column contract and to match the sibling DEL-10-05 pilot's resolution. R1 should fix the contract wording either way.
3. **One disposition per row vs stale-wording + true-substance rows.** Most requirement rows are *implemented, verified, and witness-validated* while their normative text still says "Future implementation shall … TBD". §7 `ALIGNED` requires declaration agreement, so these land as `STALE_SETUP_SPECIFICATION` even though the mechanics substance is in the best evidentiary state in the deliverable. Convention adopted: `STALE_SETUP_SPECIFICATION` when the declaration materially misdescribes existence ("future", "TBD", "no code in this setup run"); `ALIGNED` for present-tense boundary/negative claims that hold (REQ-009/010/012, most exclusions). A reviewer should confirm this precedence rule; without it, disposition histograms across deliverables will not be comparable.
4. **`ClaimClass` for diagnostics/data-policy claims.** REQ-008 (missing values → explicit findings) sits between MECHANICS, REPORTING, and WORKFLOW. Classified MECHANICS (solver input behavior); REQ-011 (envelope field preservation) classified SCHEMA. A one-line class rubric for diagnostics-behavior claims would help consistency.
5. **`SelectableUnderCurrentLoop` under suspension.** The owner's 2026-07-11 suspension declaration is not a gate suffix. The ungated Remaining item 1 (arc pressure-thrust) is nominally selectable by DAG/gate rules but the tree is suspended; recorded `UNKNOWN`. The contract should say whether suspension forces `NO`.
6. **`SourceReliability` semantics.** For mechanics rows the formulation sources are project-original hand-calc witnesses with in-repo cross-checks (quadrature, independent models, symmetry identities) — graded `REVIEWED` (not `VETTED`: no external maintainer-vetted textbook citation is recorded; equation-source boundary honored, see §5). Guidance on what earns `VETTED` is needed.
7. **`RecordedRemaining` is per-claim, but residual items are per-deliverable.** Most claim rows have no claim-specific residual; recorded `NONE_RECORDED` there and quoted verbatim text only on the rows the residual actually touches (C19, C20, UNMAPPED-02). If the intent was "copy the whole `## Remaining` block on every row", the contract should say so.
8. **Per the dispatch special rule**, the bootstrap item "Run claim-level concordance … (gated: D-41)" was treated as program mechanics and given **no claim row**; it is quoted verbatim here for completeness: `- Run claim-level concordance per the reconciliation method (source: plans/PLAN_2026-07-10_deliverable_implementation_reconciliation.md §§6–8 at the D-41-pinned main revision) (gated: D-41)`.

## 4. Evidence I could not bind (deferred, with reasons)

- **C20 (`UNKNOWN`)** — mechanics-program §5 completion state for gap rows G2/G4 and methods defects M2/M3: closure evidence lives under DEL-04-04/DEL-05-01/DEL-05-02 and `plans/ASSESSMENT_2026-07-09_physical_model_evaluation.md`, outside this deliverable-scoped pilot. Smallest next check named in the row. G1 (bend flexibility) does show closure-shaped evidence (DEC-070 + CURVEDBEND-001..004 + witness/benchmark), but promoting that to "closed" is exactly the §5 verification the Remaining item reserves — left open.
- **C18 (`REMAINING_STATE_MISMATCH`, LOW)** — "final result-envelope integration" TBD may be homed in another deliverable's `## Remaining` (DEL-04-06, DEL-00-06, or an app-side deliverable); I did not sweep other deliverables' status files. If homed elsewhere, this row downgrades to no-finding.
- **UNMAPPED-DEL0401-03** — `core/solver/nonlinear_integration` validation posture (nonlinear witnesses under `validation/hand_calcs/nonlinear/`) was inventoried but not audited row-by-row; its concordance belongs to whichever deliverable the owner homes it in.
- **Test counts not rerun.** All verification counts (36/18/16/14/71/33/24/19) are recorded run-record values at or before the frozen SHA; R0 executed nothing.

## 5. Equation-source boundary (§3 boundary 3)

No `domains/piping-design` PDF/OCR artifact was used or cited as validation anywhere in this ledger. All mechanics validation cited is in-repo, project-original ("project-original-public-content" provenance blocks): hand-calc witnesses with independent cross-derivations, witness-bound benchmark fixtures at the DEC-026 analytic tier, and the dense-oracle/hand-checked-LDLT parity chain for sparse. No compliance, release-readiness, or professional-reliance statement is made or implied by any row.

## 6. AUTHORITY_CONFLICT

None found in this deliverable's live normative sources. The nearest candidates were resolved without conflict:
- Setup-spec exclusion "no solver implementation in this setup run" vs the implemented kernel — not a live conflict: the DEV-001 human dispatch authorized the implementation; classified `STALE_SETUP_SPECIFICATION` (C01).
- Datasheet "Solver numerical library TBD" vs DEC-023 — decision register cleanly supersedes the setup Datasheet; stale document, not conflicting authority (C02).
- DEC-044's deliverable-less ownership of `nonlinear_integration` is an ownership *gap*, not a conflict between sources (UNMAPPED-03, AuthorityNeeded OWNER).

## 7. Disposition histogram (35 rows)

| Disposition | Count |
|---|---:|
| ALIGNED | 12 |
| STALE_SETUP_SPECIFICATION | 11 |
| PARTIALLY_IMPLEMENTED | 4 |
| IMPLEMENTED_UNDOCUMENTED | 3 |
| ACCEPTED_DIVERGENCE | 2 |
| REMAINING_STATE_MISMATCH | 2 |
| UNKNOWN | 1 |

Notable: zero `VERIFIED_NOT_VALIDATED` rows — DEL-04-01's mechanics claims all carry witness/benchmark validation at the frozen SHA (the hand-calc + benchmark-crate discipline is doing its job). The dominant defect mode is documentation staleness, exactly as the plan's pilot observation predicted, plus three material implementation surfaces (sparse_direct, curved_bend, nonlinear_integration) that live under DEL-04-01 run records with decision bases but no accepted deliverable-document scope mapping.
