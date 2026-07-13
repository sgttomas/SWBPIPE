# Package Concordance Summary — PKG-15 (Handoff and External Prover Workflow)

> **Epistemic status: derivative, agent-authored, non-operative evidence**
> (R2 wave W5, run DELIVERABLE_CONCORDANCE_2026-07-11_1305). Computed from
> four corrected W5 ledgers after independent high-effort fan-in and an
> owning-pilot correction. Accepted upstream evidence snapshot: frozen `main`
> at `551f84ef6be656f1603ce0acfa5e3935aa9683c7`. This summary is not
> decomposition truth and makes no owner, engineering, lifecycle,
> professional, security, or release ruling.

## Census

Four `IN_PROGRESS` deliverables cover the canonical handoff package, target
mapping, downstream modeling export, and external-prover boundary metadata.
The corrected ledgers contain **117 claim rows** (31/31/30/25). Four distinct
GPT-5 deliverable-pilot assignments ran in two owner-authorized
capacity-bounded batches (three then one). Independent highest-capability
high-effort verification is recorded in `WAVES/W5/W5_VERIFICATION_PKG-15.md`:
pre-correction 3 SOUND / 1 DEFECTIVE and 49 PASS / 0 QUALIFIED / 3 FAIL across
52 scoped rows. The three failures were one addendum-13 confidence species on
DEL-15-02 REM-001..003, corrected by the owning pilot without changing types,
dispositions, authority routing, or package scope.

## ClaimType × Disposition (computed from corrected ledgers)

| ClaimType | total | ACCEPTED_DIVERGENCE | ALIGNED | PARTIALLY_IMPLEMENTED | REMAINING_STATE_MISMATCH | STALE_SETUP_SPECIFICATION |
|---|---:|---:|---:|---:|---:|---:|
| REQUIREMENT | 44 | 1 | 41 | 2 | 0 | 0 |
| ACCEPTANCE | 23 | 1 | 22 | 0 | 0 | 0 |
| EXCLUSION | 16 | 0 | 16 | 0 | 0 | 0 |
| DECLARED_STATE | 24 | 0 | 15 | 0 | 0 | 9 |
| REMAINING_WORK | 10 | 0 | 1 | 0 | 9 | 0 |
| **Package total** | **117** | **2** | **95** | **2** | **9** | **9** |

All 117 rows are mechanically `SelectableUnderCurrentLoop=NO`. The only
substantive program residual is stage-gated on DEL-15-01; nine review findings
are technically addressed but remain formally OPEN/TBD and unhomed. There are
no IMPLEMENTED_UNMAPPED, UNKNOWN, AUTHORITY_CONFLICT,
VERIFIED_NOT_VALIDATED, or lifecycle-reassessment rows.

## Package reading

The handoff package, target-mapping, export workflow, and external-prover
boundary contracts have substantial implemented schema/workflow/test slices.
The two partial rows concern runtime privacy/redaction enforcement and
stable-JSON-versus-JCS/architecture-basis fidelity. DEC-028 permits the
archive strategy while the DEL-15-01 schema and setup kit retain a
container-TBD representation, yielding two bounded ACCEPTED_DIVERGENCE rows.

Nine setup/current-authority declaration rows are stale at document-surface
grain. Nine REMAINING_STATE_MISMATCH rows preserve technically addressed but
undispositioned findings; they are review-state/home observations, not
permission to close or remove findings. SourceReliability is UNVERIFIED 93 /
NOT_APPLICABLE 24; Confidence is HIGH 99 / MEDIUM 18.

## Decision findings (routing, not rulings)

AuthorityNeeded is **NO 87 / OWNER 29 / ENGINEERING 1**. OWNER routes current
strategy/authority declarations, privacy/interface choices, and formal human
disposition of open findings. ENGINEERING appears only on the bounded unit
metadata grain. These are agent routing judgments, not owner or engineering
rulings. Negative professional, IP, privacy, and protected-content rows use
explicit-reason reliability/validation text; no convention-6 sufficiency
marker was manufactured.

## Verification and correction record

Pilots and verifier exercised handoff schemas, target mapping, export workflow,
boundary metadata, dependency mirrors, and focused tests under addendum-9
controls. The only correction changed DEL-15-02 REM-001..003 Confidence from
HIGH to MEDIUM and the notes histogram from 25/6 to 22/9, applying addendum 13
to the technically addressed but pending-human/open-TBD species. No other cell
or histogram changed.

## Cross-ledger risks carried forward (remaining W5 calibration / R3)

1. **Repeated review findings:** RF-001/RF-002 and related PKG-02 findings
   recur across DEL-15-01..04; R3 must deduplicate the underlying formal
   disposition/home question rather than count each consumer as separate work.
2. **Container strategy:** DEC-028's archive strategy and remaining
   container-TBD schema/document representations require one cross-surface
   currentness reading, not multiple independent divergences.
3. **Canonicalization:** deterministic stable JSON is not automatically full
   JCS/RFC-8785 fidelity; deduplicate with PKG-08 and PKG-14 hashing findings.
4. **Privacy enforcement:** privacy metadata shape and caller-context
   propagation do not prove runtime redaction enforcement or unsafe embedded-
   payload diagnostics.
5. **Ownership chain:** PKG-15 consumes model-state/run/comparison contracts
   from PKG-14 and supplies handoff/export contracts to PKG-17; shared schemas
   must not be interpreted as duplicate authoritative ownership.
6. **Conditional human acceptance:** recorded human-acceptance boundaries do
   not constitute professional approval, external-prover validation, or
   release readiness.

## Fences

Frozen HEAD remained `551f84ef6be656f1603ce0acfa5e3935aa9683c7`.
Ignored-aware porcelain before/after contained exactly the six addendum-9
allow-listed paths and no seventh path. Python used
`PYTHONDONTWRITEBYTECODE=1`, external pycache where applicable, and pytest
`-p no:cacheprovider`; no in-tree `py_compile` or lockless Cargo run occurred.
No product, deliverable, lifecycle, DAG, register, dependency, R4, or R5
change is authorized or performed by this derivative summary.
