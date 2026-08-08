# Candidate Harvest Report — Chirality Piping — 2026-08-08

Status: `DECISION SUPPORT — OWNER-RULED PROMOTIONS RECORDED`

Invoking loop: `chirality-piping`

Examined Git basis: `eed5dc4f02ec328b6d5a4d10c42de6db36194c9c`

Register writes during harvest presentation: `0`

## Federation preflight

The mandatory deterministic preflight completed with four canonical
registers, complete coverage, no exclusions or operational errors, and zero
register writes. Status totals were:

| Register | OPEN | DEFERRED | ELEVATED | CLOSED | Archived |
| --- | ---: | ---: | ---: | ---: | ---: |
| PEC | 17 | 3 | 0 | 1 | 4 |
| ROOT | 12 | 11 | 0 | 0 | 99 |
| APP | 11 | 3 | 0 | 0 | 26 |
| PIP | 7 | 24 | 0 | 0 | 6 |

Federation findings were `FOREIGN_LINK_TO_LOCAL=1`,
`LOCAL_LINK_TO_FOREIGN=23`, and `REMOTE_CLOSED_LOCAL_OPEN=22`; all invalid,
unreadable, ambiguous, orphan, missing-notice, and operational-error classes
were zero.

## Sweep coverage

`taskmgmt scan` returned 342 repository-wide observations, including 72 under
the Piping tree: 32 evaluation findings, 18 notices absent from a ledger, 21
TBD-register rows, and one syntactic handoff-blocker match. It found no
non-ruled Piping decision, open packet field, or tracked-open Piping notice.

The manual PRD §5.1 completion sweep covered all `Review_Findings.csv` files,
HOLD registers, widened run-record marker forms, notice/report
`TM-CANDIDATE:` markers, handoff blockers, receipt parked lanes, and new
review-report surfaces. It found 75 review-row signals (28
`TECHNICALLY_ADDRESSED_PENDING_HUMAN`, 46 `OPEN`, one `DEFERRED`), no Piping
HOLD register, eight run-record files with substantive marker-form headings,
and zero live `TM-CANDIDATE: <concern> | <evidence-ref>` markers. The review,
evaluation, TBD, marker, and handoff populations were unchanged from their
previously owner-ratified harvest classifications.

## Owner-presented candidates

All three candidates arise from
`execution/_Coordination/NOTICE_2026-08-03_ROOT_PIPING_RESUME_RESIDUALS.md`,
Git blob `ac5086f3e02a72bd598fe6ba46eec03e65176f49`.

### HC-PIP-20260808-001 — PKG-06/07/08 pilot attribution defect

The three package summaries state “All pilots fable” although W3 discovery
pilots ran on opus. Proposed row `TM-PIP-038`, `OPEN / LOW`, with eventual
`RESOLVED_WITH_CHANGE` through a routed RECONCILIATION correction. Target
blobs: PKG-06 `1740d083616599990c7412e675deb1c3704a3d7f`, PKG-07
`78607755f645967d4fcebb5d5bd484d65cffabc8`, and PKG-08
`9d88043e410df474813e3f1e8648f6ccbabc9faf`.

### HC-PIP-20260808-002 — stale W3 pause/resume wording

The historical `RUN_BASIS.md` pause entry retains forward-looking W4
instructions under the rescinded named-model steer and points to the deleted
one-time resume file. Proposed row `TM-PIP-039`, `OPEN / LOW`, with eventual
`RESOLVED_WITH_CHANGE` through a routed RECONCILIATION amendment that
preserves the historical quote while recording its supersession. Target blob:
`f4d8a44324e8a8bdb6edb74577d05f0d32aac44a`.

### HC-PIP-20260808-003 — unresolved Addendum-9 frozen-evidence outcome

The frozen worktree carrying six untracked ignored artifact sets no longer
exists, and committed evidence does not establish whether those sets were
restored or lost. Proposed row `TM-PIP-040`, `OPEN / MEDIUM`, for bounded
evidence investigation routed through RECONCILIATION, followed by an
evidence-bound owner disposition. Current `RUN_BASIS.md` target blob:
`f4d8a44324e8a8bdb6edb74577d05f0d32aac44a`.

## Screened without promotion

The owner-ratified screen covers: Root TM-ROOT-105/109 notices already
consumed by the merged TM-PIP-032 closure; the TM-ROOT-113 response as
deferral-review evidence for existing TM-PIP-030; the outbound D-64/App
notice; the existing PRE-GATE-1 D-64 SCOPE_CHANGE handoff; foreign Root
TM-ROOT-102 closure echo; previously screened unledgered notices; duplicate
TBD/evaluation/review populations; and fenced slates, `## Remaining`, work
graphs, and ordinary lifecycle residue.

## Owner ruling

The exact ruling is preserved in
`OWNER_RULING_2026-08-08_HARVEST.md`. It promotes all three candidates exactly
as recommended and ratifies the screened-without-promotion classifications.
Promotion records attention only and creates no reconciliation correction,
evidence determination, scope, priority beyond the ruled row fields,
lifecycle, or foreign-loop effect.
