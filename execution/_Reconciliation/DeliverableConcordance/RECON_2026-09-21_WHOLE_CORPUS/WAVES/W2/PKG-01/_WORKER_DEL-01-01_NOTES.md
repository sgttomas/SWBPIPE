# Worker G1 carry-forward notebook — W2 PKG-01 (DEL-01-01 to DEL-01-04)

TASK worker, run HELP-HUMAN-PIPING-20260921-RECONCILIATION, brief SHA-256
`2d793d0a…0db141`. Evidence read from the frozen checkout at `00115c719`.
These are agent judgments, not owner rulings. Standard claim fence applies
(F-PIP-2; claims taxonomy per DEC-081).

## Recurring situations and the treatment applied to all four ledgers

| Situation | Treatment |
|---|---|
| Deliverable folder paths contain spaces and commas | Cited only in `ContextRefs` (the validator rejects spaces in evidence columns). `NormativeSource` uses `ScopeOfWork.md Lnn` relative to the deliverable folder. |
| TBD value later filled by a ruling, ISSUED SOW (DEL-01-01) | `LIFECYCLE_REASSESSMENT_REQUIRED` · `SCOPE_REDIRECTED_BY_RULING` · `PROJECT_BASELINE` · `ISSUED` · `LIFECYCLE;RECORD` · `OWNER`, one group `FG-DEL-01-01-01` (R0 ruling item 4). |
| TBD value later filled by a ruling, non-ISSUED SOW | Stale class by origin (F3): `STALE_SETUP_SPECIFICATION` if first present at `7bee9ae41`, else `STALE_REVIEW_OR_EVIDENCE`; `SCOPE_REDIRECTED_BY_RULING` · `LOCAL_DESIGN` · `NONE` · `RECORD` · `NO`; one FindingGroup per deliverable. |
| Rulings that fill the governance TBDs | DEC-027 (sole maintainer, quorum one, release authority, intake closed), DEC-057 / DEC-089 (signing), DEC-079 (D-07b: review skeleton adopted; legal instrument open). Legal-review authority and the contributor legal instrument are treated as still open (accurate TBD). |
| Repo-level governance artifact lags DEC-027 (authority blocks list roster/quorum/release authority TBD, pin 0.7/DAG-007) | `PARTIALLY_IMPLEMENTED` · `RECORD_DRIFT` · `LOCAL_DESIGN` · `NONE` · `RECORD` · `NO`, on the row that claims the artifact is maintained or preserved (DEL-01-01 REQ-01-01-05 and AC-01-01-04; DEL-01-02 AC-001; DEL-01-03 REQ-08 and AC-001; DEL-01-04 AC-001). F8 boundary named in Notes. |
| Revision / DAG pins (0.7, 0.8, DAG-006, DAG-007) in SOW text | CP-02 `STALE_REVIEW_OR_EVIDENCE` · `BASIS_POINTER_STALE` · `LOCAL_DESIGN`. In the ISSUED SOW, BaselineClass `ISSUED` and AuthorityNeeded `OWNER` (CANONICAL_DEPARTURE noted). |
| D-41 PDU-054/055 declarations | CP-03 own row, disposed as the CP-02 pin they carry; delegation to Remaining not relied on (A4). |
| Four-document residue | CP-01 `STALE_SETUP_SPECIFICATION` (all such text first present at `7bee9ae41`) · `REPRESENTATION_MIGRATED`. |
| Rename residue in a SOW | CP-04 on the SOW surface row only; items judged on substance. DEL-01-01 (ISSUED) departs to `LIFECYCLE_REASSESSMENT_REQUIRED` under C6(d), keeping cause and AuthorityNeeded. |
| Output matrix OUT-001 and VER-001 | CP-09 `STALE_REVIEW_OR_EVIDENCE` · `EVIDENCE_OVERTAKEN`: every PASS parity record binds a production hash that differs from the frozen SOW. |
| `_STATUS.md` Last Updated older than a history entry | CP-05 on the STATUS surface. |
| `_CONTEXT.md` surface | `STALE_REVIEW_OR_EVIDENCE` · `BASIS_POINTER_STALE` · `LOCAL_DESIGN` (file-level basis staleness). |
| Architecture Basis Injection | CS-04 for the pin; `.s01` PKG-00 `SEMANTIC_READY` statement (`STALE_REVIEW_OR_EVIDENCE` · `SCOPE_REDIRECTED_BY_RULING`, DEC-072); `.s02` for a still-TBD list or scope-change pointer that later rulings overtook. |
| Block with one overtaken element and otherwise aligned substance | Assessed directly with the gap's disposition (F1), not split; split only where two different non-aligned findings, or aligned and non-aligned parts of real weight, sit in one block. |

## Batch consistency

`validate_ledger_v2.py --batch` over the four sealed forward ledgers: PASS,
0 findings.
