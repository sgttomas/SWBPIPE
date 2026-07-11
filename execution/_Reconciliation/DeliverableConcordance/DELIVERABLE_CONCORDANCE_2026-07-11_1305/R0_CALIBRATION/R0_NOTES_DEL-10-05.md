# R0 Calibration Notes — DEL-10-05 Headless CLI and structured I/O analysis runner

**Run:** DELIVERABLE_CONCORDANCE_2026-07-11_1305 / R0_CALIBRATION (activation D-41/DEC-073)
**Evidence source state:** frozen worktree at commit `551f84ef6be656f1603ce0acfa5e3935aa9683c7` (read-only; no frozen-tree file modified)
**Method:** `plans/PLAN_2026-07-10_deliverable_implementation_reconciliation.md` §§3, 6, 7 as read from the frozen tree
**Ledger:** `R0_CLAIM_CONCORDANCE_DEL-10-05.csv` — 18 rows (9 requirement rows, 1 exclusion, 1 acceptance, 2 spec-staleness claims, 3 recorded residuals, 2 unmapped-implementation rows)

All dispositions below are agent dispositions under §7 of the plan, never owner or engineering rulings.

## Sources read (repo-relative, frozen tree)

Deliverable kit:
- `execution/PKG-10_Build, Packaging, API, and Interoperability/1_Working/DEL-10-05_Headless CLI and structured I-O analysis runner/Specification.md`, `Datasheet.md`, `Guidance.md`, `Procedure.md`
- same folder: `_STATUS.md` (incl. `## Remaining`), `_CONTEXT.md`, `_REFERENCES.md`, `_DEPENDENCIES.md`, `Dependencies.csv` (header + counts), `MEMORY.md`, `_REVIEW.md`, `Review_Findings.csv`
- `_run_records/`: `WORKING_ITEMS_RUN_2026-07-05_TP-RUNNER-015.md` (full); directory listing; targeted use of TP-RUNNER-013/014, TP-VERIFY-011/012B/013B, TP-PHYS-015D2, TP-UNITS-BTAIL entries via `MEMORY.md` summaries

Authority and decisions:
- `execution/_Decomposition/SOFTWARE_DECOMP.md` §12 rows DEC-025, DEC-053/054 (context), DEC-057, DEC-059, DEC-060, DEC-064, DEC-065, DEC-070, DEC-071, DEC-072
- `execution/_Coordination/_DECISIONS/_REGISTER.md` rows D-33 (RULED → DEC-065) and D-41 (AWAITING_RULING at the frozen SHA — see ambiguity 1)
- `execution/_Coordination/_DECISIONS/D-33_headless_runner_cli_process_policy.md` (O-A evidence clause)
- `docs/_Registers/ScopeLedger.csv` rows SOW-032, SOW-046, SOW-054, SOW-061
- `execution/_DAG/_LATEST.md` (DAG-007 pointer re-verified per plan §3 boundary 6)
- `loop/LOOP_RECEIPTS.md` Receipts 9, 10 (residual provenance)

Implementation / verification / validation evidence:
- `core/runner/headless/Cargo.toml`, `src/lib.rs`, `src/bin/openpipestress-runner.rs`, `src/bin/headless_preview_runner.rs`
- `schemas/headless_runner.schema.yaml`; `tests/test_headless_runner_contract.py`
- `fixtures/product_preview/` (invented fixture inventory)
- `validation/witness/generated/tp_runner_015_final_cli_{solve,validation_blocking,benchmark_stub}.json` (solve witness result_refs directly counted: 822), `tp_runner_014_headless_entrypoint_preview_run.json`
- `docs/SPEC.md` (headless runner boundary ~line 856), `docs/TYPES.md` schema table
- `apps/desktop/src/features/headless-runner/HeadlessRunnerPanel.tsx` (existence check for the unmapped row)

## Verbatim program-mechanics item (excluded from claim rows per special rule)

`_STATUS.md` `## Remaining` bootstrap item, copied verbatim and not treated as a deliverable residual claim:

> - Run claim-level concordance per the reconciliation method (source: plans/PLAN_2026-07-10_deliverable_implementation_reconciliation.md §§6–8 at the D-41-pinned main revision) (gated: D-41)

## Method ambiguities encountered

1. **Frozen tree predates the activation flip.** At `551f84ef6` the decision register still shows `D-41 | AWAITING_RULING`, no `DEC-073` exists in `SOFTWARE_DECOMP.md` §12, and the bootstrap item still carries `(gated: D-41)` unflipped. The activation (D-41/DEC-073) named by this run's dispatch is therefore not observable in the evidence source state. This is presumably lawful (ruling merged to `main` after the freeze SHA was declared), but the method gives no rule for which side of the ruling the *frozen evidence tree* should sit on, and `SelectableUnderCurrentLoop` becomes hard to derive when the gate state at the evidence SHA differs from the gate state at dispatch time. Not recorded as `AUTHORITY_CONFLICT` because the two states are time-separated, not contradictory.
2. **No `ClaimType` for declared-state statements.** Task intake asks for "declared-state statements" as claims, but the §6 `ClaimType` enum is only `REQUIREMENT/ACCEPTANCE/EXCLUSION/IMPLEMENTED_UNMAPPED/REMAINING_WORK`. Standalone stale declared-state prose (rows DEL-10-05-C12, C16) was force-fitted to `ClaimType=REQUIREMENT` with `ClaimClass=DOCUMENTATION`/`SCHEMA`; the `DeclaredState` column otherwise absorbs declared-state text onto requirement rows.
3. **Column count.** §6 lists `PackageID` / `DeliverableID` as one contract row; the ledger uses a single combined `PackageID/DeliverableID` column to keep exactly 20 columns. If two separate columns were intended the header contract should say so explicitly.
4. **`SelectableUnderCurrentLoop` vs the suspension declaration.** The three real residuals are ungated and the deliverable is `IN_PROGRESS`, so gate/lifecycle mechanics say YES; but the owner's 2026-07-11 suspension declaration ("I will suspend work in Chirality Piping for the time being", cited in the D-41 packet) makes live selection doubtful. Recorded `UNKNOWN` on residual-bearing rows. Smallest next check: confirm whether the D-41 ruling text (post-freeze) treats suspension as blocking ordinary loop selection during the concordance run.
5. **Disposition granularity on honestly-partitioned requirements.** R-01..R-05 are written for the "future runner" while a bounded slice is implemented and separately declared. Where the implemented slice fully satisfies the requirement's testable content, `ALIGNED` was used with the deferred remainder noted; where the requirement's own content is only partially covered (R-01 service-boundary breadth, R-04 export payload), `PARTIALLY_IMPLEMENTED` was used. The boundary between "ALIGNED slice + recorded residual" and "PARTIALLY_IMPLEMENTED" is a judgment call the method does not fully pin down; reviewer attention requested.
6. **Verification evidence is recorded-pass, not re-executed.** Tests were not run in the read-only frozen tree; `VerificationEvidence` cites test files plus the recorded passes in commit-bound run records, flagged "not re-executed at frozen SHA". The method should state whether R0 requires re-execution (which mutates `target/`) or accepts recorded passes bound to earlier source states.

## AUTHORITY_CONFLICT

None found. The Guidance conflict table records none; the one review finding (PKG10-DEL1005-PKG02-W001) is human-dispositioned ACCEPT_AS_IS/RESOLVED (2026-06-07). The D-41 register-state timing issue (ambiguity 1) was deliberately not classed as a conflict.

## Dispositions of note (calibration expectations vs findings)

- **ACCEPTED_DIVERGENCE was genuinely decision-backed.** The two payload-binding deferrals (C13, C14) rest on the D-33 O-A evidence clause — "at least one export-result or benchmark/regression evidence stub if the operation is not fully implemented yet" — ruled as `DEC-065`. This satisfied the "human decision actually permits deferral" bar without inference. The witness-refresh residual (C15) has no such decision and stayed `STALE_REVIEW_OR_EVIDENCE`.
- **Expected DOCUMENTED_UNIMPLEMENTED largely did not materialize.** The deferred slice is either decision-permitted stubs (ACCEPTED_DIVERGENCE) or lives inside partially-implemented requirement rows; nothing presented as a current unimplemented requirement without either implementation or a permitting decision. The honest partition in the spec is why.
- **Residual setup-era prose survives inside an otherwise current spec** (C12, C16) — the deliverable partitions honestly at section level while sentence-level "future runner"/"exact schema files TBD" wording is overtaken. Claim-level extraction caught what keyword-flagging would have over- or under-called.

## Claims deferred

None deferred to `DEFERRED_AGENT_WORKFLOW` (no claim depended on agent-instruction or workflow analysis). No `UNKNOWN` or `ENGINEERING_AUTHORITY_REQUIRED` dispositions were needed; the two `IMPLEMENTED_UNDOCUMENTED` rows carry `AuthorityNeeded=OWNER` for scope mapping (desktop headless-runner panel ownership; optional full-envelope payload validation representation) rather than manufactured closure.
