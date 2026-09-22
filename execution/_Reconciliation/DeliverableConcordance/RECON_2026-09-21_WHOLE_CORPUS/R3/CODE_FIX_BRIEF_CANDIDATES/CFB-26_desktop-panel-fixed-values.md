# CFB-26 — Desktop panels showing fixed packet values

**Candidate brief (H2). Not executed.** Area: Desktop GUI (DEL-07-08, DEL-10-04). Run HELP-HUMAN-PIPING-20260921-RECONCILIATION, R3 integration, TASK H2. Execution, if the owner selects it, goes through an owner-steered production brief and the chirality-change PR path, followed by re-verification in a later concordance (ledger rows are not edited in place). Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

## Scope

Replace the fixed `core_contract_evidence` count block in DesignWorkspacePanel (freeze `projects/chirality-piping/apps/desktop/src/features/design-workspace/DesignWorkspacePanel.tsx:204`) with values from upstream records, or label it as fixture evidence; make BuildReadinessPanel read bundle state from the Tauri config instead of hard-coding `bundle_active: false` (freeze `apps/desktop/src/features/build-readiness/BuildReadinessPanel.tsx:116` vs `apps/desktop/src-tauri/tauri.conf.json:27` `"active": true`) and report DEC-057/DEC-089 items as ruled.

## Affected claims

3 claim rows on 2 deliverable(s): DEL-07-08, DEL-10-04.

Classes (portion in this brief / class total): T7-C06 3/42 (Authority REVIEW).

Reproducing filter: `CODE_FIX_ROWS.csv` where `CFB == "CFB-26"`. Each key below is in `R3/CLASS_ASSIGNMENTS.csv`, `R3/TASKS/T8_ROWS.csv` or `R3/TASKS/T12_UNREACHED.csv` with route CODE_FIX_CANDIDATE, except class T7-C05 and NOT_DIVERGENT rows, which enter through T8 only.

| Key | Class | Authority | BlockedOnPacket | T8/T12 view | Remaining work (effective; OC = OtherCorrections) |
|---|---|---|---|---|---|
| `DEL-07-08:SOW#CLM-011/REQ-07-08-002` | T7-C06 | REVIEW | H3[T7-C06] | — | Replace the fixed core_contract_evidence counts with values from upstream records, or label them as fixture evidence. |
| `DEL-07-08:SOW#CLM-025` | T7-C06 | REVIEW | H3[T7-C06] | — | As REQ-07-08-002. |
| `DEL-10-04:SOW#CLM-013.s02` | T7-C06 | REVIEW | H3[T7-C06] | — | Bounded code repair of the panel packet: read bundle state from the Tauri config (bundle.active true, targets [app]) and report DEC-057/DEC-089 as ruled, or relabel the… |

## Evidence

Sealed ledgers (reliability: sealed R2 ledger rows with effective values from adopted resolutions; verified where the wave verifier sampled them, otherwise worker reading):

- DEL-07-08: `projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/WAVES/W1/PKG-07/DEL-07-08/DEL-07-08_forward.csv`; ImplementationEvidence cited: `apps/desktop/src/features/design-workspace/DesignWorkspacePanel.tsx#L203`, `apps/desktop/src/services/previewService.ts`, `apps/desktop/src/features/comparison/ComparisonPanel.tsx`, `apps/desktop/src/services/operationService.ts`.
- DEL-10-04: `projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/WAVES/W3/PKG-10/DEL-10-04/DEL-10-04_forward.csv`; ImplementationEvidence cited: `apps/desktop/src/features/build-readiness/BuildReadinessPanel.tsx`, `apps/desktop/src-tauri/tauri.conf.json`, `apps/desktop/src/App.tsx`.

Freeze line citations in Scope were re-read at the freeze (`00115c719`) by H2 as code reading only; no build or test was run. Classification sources: `R3/TASKS/T4A_CLASSES.md`, `T6_CLASSES.md`, `T7_CLASSES.md`, `T8_CLUSTERS.md`, `T12_UNREACHED.md` (proposals, not accepted results).

## Acceptance checks

- A test shows the Design Workspace line changes with the project's records, or is labelled fixture evidence.
- A test shows the build-readiness packet reflects the config bundle state; BUILD-READINESS-BUNDLE-INACTIVE does not fire when bundling is active.
- Independent review (H3 item for T7-C06).
- The affected ledger rows are re-verified in a later concordance; no ALIGNED status is claimed from this brief.

## Protected-content status

No protected subject: no affected row is at INVARIANT tier or carries an IP_DATA, CLAIMS or SECURITY layer. 
This brief quotes no protected, private or third-party content. Execution uses invented or synthetic fixtures only and introduces no standards text, tables or equation sources (DEC-043).

## BlockedOnPacket

3 of 3 claim rows carry a block: H3[T7-C06] (3). `H3[<class>]` names the H3 register item for that class (review before repair). All rows are blocked.

## Notes and open views

- Two panels, one pattern (static packet values); may be split into two PRs at execution.

## Dependencies

H3[T7-C06].

