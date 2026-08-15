# Candidate Harvest Report — Chirality Piping — 2026-08-15

Status: `DECISION SUPPORT — OWNER-RULED PROMOTION RECORDED`

Invoking loop: `chirality-piping`

Examined Git basis: `dab32a212a961af8430b08dbc417bf62d30ebc69`

Branch: `codex/piping-taskmgmt-generational-20260815`

Register writes during harvest presentation: `0`

Register writes during owner-ruled application: `1` appended row

This report is decision support only. It creates no Action Item, duty,
priority, scope, approval, lifecycle, or foreign-loop effect. Promotion is
an owner act under K-TM-3.

## Mandatory federation preflight

The deterministic preflight was rerun from the current merged-main basis
before candidate harvest. It completed with four canonical registers,
complete coverage, no excluded lookalikes, no operational errors, no invalid,
unreadable, ambiguous, duplicate-ID, orphaned-link, or missing-notice inputs,
and zero register writes.

| Register | OPEN | DEFERRED | ELEVATED | CLOSED | Archived | Validation |
| --- | ---: | ---: | ---: | ---: | ---: | --- |
| PEC | 16 | 1 | 0 | 1 | 7 | PASS |
| ROOT | 11 | 10 | 0 | 0 | 102 | PASS |
| APP | 13 | 3 | 0 | 0 | 26 | PASS |
| PIP | 9 | 23 | 0 | 0 | 8 | PASS |

Federation findings were `FOREIGN_LINK_TO_LOCAL=1`,
`LOCAL_LINK_TO_FOREIGN=23`, and `REMOTE_CLOSED_LOCAL_OPEN=22`; every other
finding class was zero. The ignored projection
`.candidates/federation.json` is 54,287 bytes at SHA-256
`6fce229a8b6922dfe7d3c1fe8ed80f62c5aa239b2138981ea7d20d59f3714678`.

## Deterministic `taskmgmt scan`

The helper returned 343 repository-wide observations after folding 73
canonical-copy duplicates:

| Implemented class | Count |
| --- | ---: |
| decision non-ruled | 0 |
| notice tracked-open | 58 |
| notice absent from every ledger | 66 |
| evaluation finding open | 156 |
| packet field open | 64 |
| TBD-register row | 21 |
| syntactic handoff blocker | 51 |

The Piping subtree contributed 72 observations: 32 evaluation findings, 18
notices absent from a ledger, 21 TBD-register rows, and one syntactic handoff
blocker. That Piping population is unchanged from the 2026-08-08 harvest.
The ignored projection `.candidates/scan.json` is 123,755 bytes at SHA-256
`55f3123d3a975c775706f5aa3852a0cbfb0d4280a4a65e242ecbfe327b23dff6`.

## Manual PRD §5.1 completion sweep

The manual pass covered every source class omitted or only partially
implemented by `taskmgmt scan`.

| Manual class | Result |
| --- | --- |
| `Review_Findings.csv` | 93 files / 133 rows / 75 open-style signals: 46 `OPEN`, 28 `TECHNICALLY_ADDRESSED_PENDING_HUMAN`, one `DEFERRED`; no file changed since the prior owner-ratified harvest basis. |
| HOLD registers | No Piping `APP_HOLD_REGISTER.csv`, `ACTIVE_RELIANCE_HOLDS.csv`, or `*HOLD*REGISTER*.csv` exists. |
| Packet open-question/conflict fields | No open Piping packet field; the two aggregated `Conflicts.csv` files contain zero rows. |
| Run-record and managed-return markers | 13 source files contain `MISSING` / `NEEDS_HUMAN_RULING` headings (eight prior, five added since the prior harvest). Four new non-null occurrences are intermediate DEL-09-04 verification/measurement cleanup, runtime, and selection gates; every one was consumed or superseded by the later owner-gates application, TM-PIP-037 closure, successful DEC-025 evidence, and merged closeouts. No new row is proposed. |
| Inline `TM-CANDIDATE:` grammar | Zero live `TM-CANDIDATE: <concern> | <evidence-ref>` markers. Occurrences are instruction examples, scanner-defect prose already represented by `TM-PIP-031`, or historical register/report text. |
| Handoff blockers and coverage gaps | All newly landed handoff blockers are historical intermediate gates or identify later Git/receipt/verification work already completed in merged commits. The helper's one Piping match remains the old R13 handoff already covered by prior rulings. |
| Slates and review reports | No tracked Piping plan, review report, or next-work-slate path changed since the prior harvest basis. The one existing next-work slate and ordinary `## Remaining`/work-graph content remain fenced from work discovery by PRD §5.5. |
| Receipt parked lanes | Zero literal `Parked lanes:` entries. Receipt 99's public comparison-number residual remains governed DEL-09-04 state, not new session residue or a Task Management work queue. |
| Notices and reports | No Piping `NOTICE_*.md` changed since the prior harvest. The 18 unledgered notices retain the prior owner-ratified screened classifications. The new basename-matched DEC-025 bridge note contains the one new explicit follow-on residual presented below. |

The unchanged review/evaluation/TBD/notice populations remain covered by the
screened-without-promotion ruling in
`OWNER_RULING_2026-08-08_HARVEST.md` (Git blob
`939206949f34d535c6722aecd736545532f41b28`). This report does not reopen or
reinterpret that ruling.

## Owner-presented candidate

### HC-PIP-20260815-001 — honor `CARGO_TARGET_DIR` in headless-runner contract tests

Source:
`validation/evidence/sweeps/SWEEP_20260814T152534Z_3863e76a619e.BRIDGE_NOTE.md`
lines 44–49, Git blob `cc46e4f203c5536285ee9ccdf1127f4a24559b18`,
SHA-256 `0578add4a7da202b4230c54065caca673d2b9f1b8d127c778362b5925c0b59ac`.

Concern: `tests/test_headless_runner_contract.py:329` hard-codes
`core/runner/headless/target/debug` instead of resolving runner binaries from
`CARGO_TARGET_DIR` or Cargo metadata. The first hermetic DEC-025 attempt
therefore produced 15 pytest setup errors and required a disposable,
local-exclude-bound symlink bridge before the later host sweep could pass.
The passing evidence is valid and self-discloses the bridge; the follow-on is
an engineering portability defect, not a challenge to that evidence.

Proposed promotion:

| Field | Proposed value |
| --- | --- |
| ActionItemID | `TM-PIP-041` |
| Title | Honor `CARGO_TARGET_DIR` in headless-runner contract tests |
| Status / Priority | `OPEN / LOW` |
| Domain lenses | `Checking; Work; Planning` |
| Associated with | `DEL-10-05; DEC-025; headless runner contract tests` |
| Assignment | DEL-10-05 owning production lane (R/S); TASK_MANAGEMENT monitors disposition; A is human-only |
| Resolution path | Deliverable amendment through the DEL-10-05 owning `WORKING_ITEMS` lane; no code dispatch in this harvest step |

Priority basis: current evidence passed with a disclosed disposable bridge,
so this is not a present release or evidence blocker; repairing it removes a
repeatable hermetic-run portability failure.

## Screened without promotion

- The 21 PKG-17 TBD rows remain represented by live `TM-PIP-002..022`; no
  duplicate rows are proposed.
- The 75 review-row signals remain owned by their review/lifecycle surfaces;
  no new independent session residue was found.
- The 32 evaluation findings, 18 unledgered notices, old R13 handoff match,
  and unchanged plan/slate surfaces retain the 2026-08-08 owner-ratified
  screen.
- The new DEL-09-04 and runner-binding run roots contain historical
  intermediate holds and later-satisfied closeout gates. They are immutable
  evidence, not current candidate rows.
- The DEL-09-04 public comparison-number residual is explicit governed
  deliverable state preserved by the TM-PIP-037 closure ruling. K-TM-1 and
  the no-work-discovery fence prohibit mirroring it into a queue-like row.
- `TM-PIP-031` already records the helper's marker-class blind spots; this
  manual pass confirms the defect remains but does not duplicate it.

## Owner ruling

The exact owner ruling is preserved in
`OWNER_RULING_2026-08-15_HARVEST.md`:

> promote it to `TM-PIP-041`, `OPEN / LOW`

`HC-PIP-20260815-001` is therefore recorded as exactly `TM-PIP-041`,
`OPEN / LOW`. No other candidate is promoted and no screened classification
is converted into a row. The promotion records attention only: it does not
dispatch work, modify DEL-10-05, create a lifecycle or release effect, write a
foreign register, or authorize a routed notice before the session closeout
gate.
