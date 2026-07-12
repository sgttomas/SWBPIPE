# Run Summary — DELIVERABLE_CONCORDANCE_2026-07-11_1305

> **Derivative, agent-authored, non-operative evidence.** This summary closes
> the ruled D-41 discovery/reconciliation run at R6. Every disposition,
> conflict classification, routing entry, and proposed update remains an agent
> judgment, never an owner, engineering, review, lifecycle, compatibility,
> security, legal, release, or professional ruling. Accepted upstream source
> snapshot: frozen `main` at
> `551f84ef6be656f1603ce0acfa5e3935aa9683c7`. Method: pinned
> `plans/PLAN_2026-07-10_deliverable_implementation_reconciliation.md` §§6–8
> at that SHA, with the owner-adopted `R1_CONVENTIONS.md` addenda.

## Outcome

R0/R0b calibration, R1 corpus indexing, all five R2 waves, R3 cross-package
reconciliation, and R6 backcheck are complete. The final corpus covers all 101
live deliverables and all 1,100 inventory requirement IDs. R6 verdict:
**BACKCHECK COMPLETE WITH PRESERVED OPEN FINDINGS**.

Completion means the derivative evidence is structurally complete and
backchecked. It does **not** mean every deliverable is complete, every test was
rerun, every remaining item was repaired, or any compatibility, suitability,
release, compliance, security, or professional claim was accepted.

## Phase record

### R0 and R0b — calibration

- R0 produced three pilot ledgers and an independent review. Verdict:
  `READY WITH NAMED CONVENTIONS`.
- Owner-directed R0b tested the proposed rules on three different
  deliverables. Verdict: `SCALE-READY`.
- The owner adopted the convention set for R1+. R0/R0b ledgers remain
  calibration evidence and were excluded from aggregation; their six
  deliverables were re-encoded in ordinary R2 waves.

### R1 — live corpus indexes

- `DELIVERABLE_INVENTORY.csv`: 101 deliverables, 100 `IN_PROGRESS`, one
  `ISSUED` (`DEL-01-01`).
- `IMPLEMENTATION_SURFACES.csv`: 231 material surfaces, 223 attributed and
  eight retained as authoritative `NONE_FOUND` attribution gaps.
- Verification/provenance indexes: 147 suites, 1,592 static tests, and 396
  validation/provenance records at the recorded evidence grain.
- Authority/source map and decision index completed with no open authority
  conflict established at R1.

### R2 — claim-level discovery and package fan-in

- 101 corrected deliverable ledgers and 101 notes across W1–W5.
- Ordinary claim rows: W1 504 + W2 242 + W3 399 + W4 491 + W5 848 =
  **2,484**.
- Eighteen package summaries and eighteen independent package fan-in reports
  are present. The package reports implement the required self-flagged,
  non-ALIGNED, and representative-ALIGNED review design.
- All corrections were routed through owning pilots. Final ordinary ledgers
  are exact-width 20-column RFC-4180 CRLF and have unique controlled ClaimIDs.

### R3 — cross-package synthesis

- Deterministic assembly appended eight R1 attribution gaps as
  `UNMAPPED-SURF-011/021/050/104/170/211/212/213`, producing the 2,492-row
  `CLAIM_CONCORDANCE.csv` and mirrored `UNMAPPED_IMPLEMENTATION.csv`.
- `CONFLICTS_AND_UNKNOWNS.csv` contains 20 deduplicated, provenance-backed
  findings with authority route and smallest next action. Statuses: 15
  `OPEN_CANDIDATE`, one `UNKNOWN_PRESERVED`, one `ROUTED_NOT_DECIDED`, one
  `GATED_OR_OPEN_CANDIDATE`, and two `NO_CONFLICT_CONFIRMED`.
- The sole corpus UNKNOWN remains `DEL-07-05-REM-002`, the unresolved home for
  rotational-deformation visualization. It was preserved, not guessed closed.
- CU-018 and CU-019 found no present duplicate-ownership conflict: DEL-16-01
  remains the operation-contract owner with consumers downstream; DEL-17-02
  remains the common export-contract owner and DEL-17-01 the source-basis
  owner, with target-local implementation in DEL-17-03..09.
- Evidence-only engineering and owner decision findings stop at routing and
  smallest next action. They are not R4 options or rulings.
- `PROPOSED_DELIVERABLE_UPDATES.csv` has 77 grouped rows covering all 532
  non-ALIGNED ClaimIDs exactly once. Zero `DEFERRED_AGENT_WORKFLOW` rows were
  found; `AGENT_WORKFLOW_OBSERVATIONS.md` records that zero result.

### R6 — deterministic backcheck

- All 101 frozen `_STATUS.md ## Remaining` bullet arrays equal the owning
  ledger `RecordedRemaining` sequences: 100 bootstrap bullets and 50
  substantive bullets, each in source order.
- Bootstrap-derived source/gate metadata count is zero. Substantive metadata
  and selectability reproduce the substantive status items for all 101
  deliverables.
- All 1,100 inventory requirement IDs resolve exactly in their owning ledgers;
  final ClaimIDs are unique; all 18 package-summary histograms recompute.
- Eleven earlier LF-ledger record terminators were normalized to CRLF after
  parsed-row identity checks. Bounded status transcription/metadata defects
  found by the independent audit were corrected through owning pilots.
- The aggregate was rebuilt after all ledger corrections. Its first 2,484
  rows reproduce the ordered ordinary source rows and its final eight rows
  reproduce the unmapped file.
- `COVERAGE_AND_QA.md` records the complete deterministic command set,
  correction trail, limitations, and final stop boundary.

## Final corpus and hashes

| Item | Final value |
|---|---:|
| Deliverables | 101 |
| Ordinary ledgers / notes | 101 / 101 |
| Package summaries / verification reports | 18 / 18 |
| Ordinary claim rows | 2,484 |
| Unmapped run-level rows | 8 |
| Final claim rows / unique ClaimIDs | 2,492 / 2,492 |
| Requirement IDs covered | 1,100 / 1,100 |
| Non-ALIGNED rows routed by proposed updates | 532 / 532 |

Final ClaimType histogram: REQUIREMENT 1,100; ACCEPTANCE 345; EXCLUSION 310;
DECLARED_STATE 625; REMAINING_WORK 104; IMPLEMENTED_UNMAPPED 8.

Final Disposition histogram: ALIGNED 1,960; STALE_SETUP_SPECIFICATION 307;
PARTIALLY_IMPLEMENTED 135; REMAINING_STATE_MISMATCH 34;
ACCEPTED_DIVERGENCE 24; VERIFIED_NOT_VALIDATED 13;
IMPLEMENTED_UNDOCUMENTED 8; DOCUMENTED_UNIMPLEMENTED 5;
IMPLEMENTED_DIFFERENTLY 3; STALE_REVIEW_OR_EVIDENCE 2; UNKNOWN 1.

Other final facts: SourceReliability UNVERIFIED 1,634 / NOT_APPLICABLE 670 /
REVIEWED 188; Confidence HIGH 1,938 / MEDIUM 553 / LOW 1; LifecycleState
IN_PROGRESS 2,462 / ISSUED 22 / UNMAPPED 8; Selectability NO 2,429 / YES 63.
The 63 YES values are mechanical recorded states, not authorization to select
or execute work.

AuthorityNeeded histogram: NO 1,974; OWNER 434; ENGINEERING 43; REVIEW 32;
SCOPE_CHANGE 4; D-38 2; D-12 2; D-07b 1. Decision IDs are gate pointers, not
human-authority categories or completed decisions.

Hashes:

- final `CLAIM_CONCORDANCE.csv` SHA-256:
  `32095986662b4932d9b1bf403e1756addb87dac3142b488f56acd926178361e0`;
- `UNMAPPED_IMPLEMENTATION.csv` SHA-256:
  `9391174a59ede1ed18d1393c666ead464c1291060351bc1fcce0167df989b442`;
- ordered ordinary-record byte prefix SHA-256:
  `e5ad64a12e50cf975678ac9aa627f6dab08119cc240da0bb2568497c49c928c3`;
- SHA-256 over the ordered list of 101 ledger digests:
  `a9bb036c2a6ae5f211a9c2b4b4fe0db9b5ce0c7be54b72cff575c1b7b273e8c3`;
- digest over the 18 package-verification report digests:
  `07e2ee521dd63ee199082779a3817ee1aae3b643ab0a6c780050ddc0e7de7c06`.

## Preserved findings and routes

R6 intentionally retains 34 `REMAINING_STATE_MISMATCH` rows, the sole
UNKNOWN, eight unmapped surfaces, 13 `VERIFIED_NOT_VALIDATED` rows, and the
open R3 candidates. Repeated review findings were deduplicated by underlying
species/home rather than counted as independent technical defects.

Engineering routes cover canonicalization/JCS fidelity, unit and numeric
normalization validation, threshold/tolerance suitability, production-path
oracle binding, runtime no-bypass control reach, scalar/result-envelope
traceability, review-geometry transforms/diagnostics, and benchmark-envelope
integration. Owner/product/review routes cover the sole UNKNOWN, property-
inspector ownership, unmapped surface attribution, the ISSUED change path,
active-kit currentness, pending finding disposition/home, privacy defaults,
runtime schema-version policy, stale evidence records, CAEPIPE live-profile
gates, format scope, external-source review, and application-service homes.

These routes identify authority and smallest next checks only. They do not
authorize repairs, select work, establish compatibility, or supply a ruling.

## Model-role and temporary capacity attribution

Durable run records preserve the historical role labels used when work ran:
R0/R0b calibration and review, W1/W3 discovery recorded with the directed
opus/fable role split and fable fan-in, W2 with fable pilots/fan-in, and W4/W5
with GPT-5 deliverable pilots plus independent highest-capability/high-effort
fan-in. R3 and R6 are attributed to GPT-5 synthesis/QA roles in their durable
artifacts. These are model-role attributions, not authority designations.

For the resumed W5 session, the runtime exposed **four total active-agent
slots including the coordinator**. Under the owner's temporary session-only
exception, packages retained exactly one owning pilot per deliverable but ran
in capacity-bounded batches with at most three delegated pilots plus the
coordinator active. PKG-13 used three delegated pilots plus the coordinator;
PKG-14..17 used maximum-capacity rolling/two-batch schedules. Independent
package fan-in, owning-pilot correction routing, and package checkpoint
validation were preserved. This variance is not a permanent rewrite of the
standing package-wide concurrency preference.

## Harness, verification, and containment evidence

The final R6 QA reran the practitioner-harness self-check and pytest surface
with `PYTHONDONTWRITEBYTECODE=1`, an external `PYTHONPYCACHEPREFIX`, and
`pytest -p no:cacheprovider`:

- harness self-check: exit 0; only pre-existing REVIEW/WARN findings and no
  D-41 artifact BLOCK;
- practitioner-harness pytest: **263 passed, 1 skipped in 32.21s**.

This validates the harness and D-41 artifact checks, not all 147 indexed
suites, every external source, engineering suitability, compatibility,
release readiness, or professional acceptance.

Frozen worktree HEAD remained
`551f84ef6be656f1603ce0acfa5e3935aa9683c7`. Final containment checks found no
tracked frozen-tree change and exactly the six known ignored paths:

1. `projects/chirality-piping/.pytest_cache/`
2. `projects/chirality-piping/core/reporting/report_generator/Cargo.lock`
3. `projects/chirality-piping/core/reporting/result_export/Cargo.lock`
4. `projects/chirality-piping/core/reporting/state_comparison_handoff_sections/__pycache__/`
5. `projects/chirality-piping/tests/__pycache__/`
6. `projects/chirality-piping/validation/benchmarks/nonlinear/target/`

The earlier ignored-artifact incident remains disclosed in `RUN_BASIS.md`.
W4+ used ignored-aware porcelain, the exact allow-list, copy-out execution for
lockless Cargo crates, cache-disabled pytest, and no in-tree `py_compile`. No
seventh ignored path appeared.

## Proven Git and PR state at summary closeout

Read-only closeout inspection proved:

- branch: `claude/chirality-piping-d41-concordance-9811cb`;
- local HEAD: `2c9c9dd16d79aedd6144250d3a065dd30ef51798`
  (`D-41: complete R3 cross-package synthesis`);
- remote tracking head:
  `965206d36e0b854f25ebc8b60aabc73a9d27dabb`; local branch is one commit
  ahead and zero behind;
- PR #211 is OPEN, non-draft, merge state `CLEAN`, base `main`, head branch
  `claude/chirality-piping-d41-concordance-9811cb`, URL
  `https://github.com/sgttomas/chirality/pull/211`;
- PR #211's proven head OID is `965206d36e0b854f25ebc8b60aabc73a9d27dabb`
  (updated `2026-07-12T21:16:26Z`), so it contains W5 through the full-wave
  closeout but does not yet contain the local R3 commit or uncommitted R6
  derivative evidence;
- the working tree contains uncommitted changes only inside this D-41 run
  folder for R6 correction/aggregate/QA/summary evidence.

No later push, PR-head update, merge, or close state is claimed.

## Limitations

- R6 did not rerun every indexed suite or independently revisit every URL.
- UNVERIFIED remains the correct reliability for 1,634 rows; QA does not
  upgrade them.
- Repair acceptance is intentionally unsatisfied because R4 and R5 did not
  occur.
- Open findings, mechanically selectable residuals, and routed candidate
  actions remain evidence, not a work queue.

## Hard stop

**STOP. R4 and R5 were not performed and are not authorized by this summary.**

This run made no lifecycle transition, product repair, dependency/DAG/register
mutation, decomposition change, owner/engineering/review disposition, release
decision, compatibility claim, certification, sealing, authentication,
professional-approval claim, or code-compliance ruling. Any later R4 decision
gate or R5 repair requires separate explicit authority and a new bounded task.
