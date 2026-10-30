# TP-EXPORT-006-C Dependency And DAG Alignment Review

## Inputs Read

- `AGENTS.md`
- `agents/AGENT_TASK.md`
- `execution/_DAG/DAG-005/DependencyEdges.csv`
- `execution/_DAG/DAG-005/DeliverableNodes.csv`
- `execution/_DAG/DAG-005/DEV-001_BLOCKER_QUEUE.md`
- `execution/_DAG/DAG-005/DEV-001_BLOCKER_QUEUE.csv`
- `execution/_DAG/DAG-005/APPROVAL_RECORD.md`
- `execution/_Coordination/DEV-001_IMPLEMENTATION_EVIDENCE.csv`
- `execution/_Coordination/DEV-001_BLOCKER_QUEUE.md`
- `execution/_Coordination/DEV-001_BLOCKER_QUEUE.csv`
- `execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-01_CAEPIPE and export-format source basis/Dependencies.csv`
- `execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-01_CAEPIPE and export-format source basis/_DEPENDENCIES.md`
- `execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-02_Export package, profile, and stable ID map contracts/Dependencies.csv`
- `execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-02_Export package, profile, and stable ID map contracts/_DEPENDENCIES.md`
- `execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-03_Native open JSON export package/Dependencies.csv`
- `execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-03_Native open JSON export package/_DEPENDENCIES.md`
- `execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-04_CAEPIPE MBF export profile and deterministic writer/Dependencies.csv`
- `execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-04_CAEPIPE MBF export profile and deterministic writer/_DEPENDENCIES.md`
- `execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-05_CAEPIPE external run harness and CSV parser/Dependencies.csv`
- `execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-05_CAEPIPE external run harness and CSV parser/_DEPENDENCIES.md`
- `execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-06_Stress-neutral CSV JSON package/Dependencies.csv`
- `execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-06_Stress-neutral CSV JSON package/_DEPENDENCIES.md`
- `execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-07_Conservative PCF subset exporter/Dependencies.csv`
- `execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-07_Conservative PCF subset exporter/_DEPENDENCIES.md`
- `execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-08_GLB glTF review geometry export/Dependencies.csv`
- `execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-08_GLB glTF review geometry export/_DEPENDENCIES.md`
- `execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-09_Export adapter SDK and additional targets/Dependencies.csv`
- `execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-09_Export adapter SDK and additional targets/_DEPENDENCIES.md`

Read-only validation commands used:

- `python3` CSV row-width validation over all `DEL-17-* / Dependencies.csv` files.
- `python3` CSV extraction of PKG-17 DAG-005 edges and local active execution edges.
- `rg` evidence search for `PKG-17`, `DEL-17`, `0768fa29`, and `5961995f` across DAG/root queue and implementation evidence files.
- `git log --oneline -n 20`
- `git show --stat --oneline --name-only 0768fa29 --`
- `git show --stat --oneline --name-only 5961995f --`

## Verdict

BLOCKED.

## Findings

| FindingID | Severity | DeliverableID | Location | Description | ProposedDisposition |
|---|---|---|---|---|---|
| TP-EXPORT-006-C-F001 | BLOCKER | DEL-17-05 | `execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-05_CAEPIPE external run harness and CSV parser/Dependencies.csv:12` | `DEP-017-05-011` has 31 CSV fields against the 29-column v3.1 header. The unquoted target location `../DEL-17-02_Export package, profile, and stable ID map contracts/` is split at commas, corrupting downstream columns including `Statement`, `EvidenceFile`, `SatisfactionStatus`, `Confidence`, `Origin`, `Status`, and `Notes`. This means the DEL-17-05 dependency register is not schema-valid in principle and the local `DEL-17-05 -> DEL-17-02` interface row cannot be safely machine-read. | Repair the CSV row by quoting the comma-containing field or using a comma-free path token, then rerun row-width/schema validation. Do not consume the DEL-17-05 register as authoritative until fixed. |
| TP-EXPORT-006-C-F002 | WARNING | DEL-17-03, DEL-17-04, DEL-17-05, DEL-17-06, DEL-17-07, DEL-17-08 | Local `Dependencies.csv` active extracted source-basis/interface rows; DAG-005 `DependencyEdges.csv` rows `DAG-005-R0968` through `DAG-005-R0988` | Several local registers contain active extracted execution edges to `DEL-17-01` beyond the approved DAG-005 gating shape, and DEL-17-05 also records a local interface to `DEL-17-02`. DAG-005 gating edges preserve the approved sequence `DEL-17-01 -> DEL-17-02`, `DEL-17-02 -> DEL-17-03/04/06/07/08/09`, and `DEL-17-04 -> DEL-17-05`, plus declared additional non-PKG-17 predecessors. The root and DAG blocker queues are not treating the local extracted source-basis rows as active blockers. | Keep these rows as local extracted evidence unless a later DAG amendment explicitly promotes them. If intended to gate sequencing, reconcile through DAG proposal/revalidation rather than treating them as current blockers. |

## DAG / Queue Evidence

- `execution/_DAG/DAG-005/APPROVAL_RECORD.md` approves the SCA-004 export-interoperability graph extension over DAG-004 and explicitly limits the approval from certifying engineering results, issuing deliverables, authorizing Type 2 implementation dispatch, promoting candidate rows, or making release/code-compliance/professional claims.
- `execution/_DAG/DAG-005/DeliverableNodes.csv` contains `DEL-17-01` through `DEL-17-09` under `PKG-17` with `LifecycleState=OPEN` in the DAG node register.
- `execution/_DAG/DAG-005/DependencyEdges.csv` contains the expected active PKG-17 sequencing edges:
  - `DAG-005-R0968`: `DEL-17-02 -> DEL-17-01`
  - `DAG-005-R0969`: `DEL-17-03 -> DEL-17-02`
  - `DAG-005-R0970`: `DEL-17-04 -> DEL-17-02`
  - `DAG-005-R0971`: `DEL-17-05 -> DEL-17-04`
  - `DAG-005-R0972`: `DEL-17-06 -> DEL-17-02`
  - `DAG-005-R0976`: `DEL-17-07 -> DEL-17-02`
  - `DAG-005-R0980`: `DEL-17-08 -> DEL-17-02`
  - `DAG-005-R0985`: `DEL-17-09 -> DEL-17-02`
- DAG-005 also preserves declared additional dependencies for:
  - `DEL-17-06 -> DEL-08-04`, `DEL-14-02`, `DEL-14-05`
  - `DEL-17-07 -> DEL-03-02`, `DEL-13-04`, `DEL-15-02`
  - `DEL-17-08 -> DEL-02-01`, `DEL-07-01`, `DEL-13-04`, `DEL-15-01`
  - `DEL-17-09 -> DEL-02-04`, `DEL-10-01`, `DEL-10-02`
- Both `execution/_DAG/DAG-005/DEV-001_BLOCKER_QUEUE.*` and `execution/_Coordination/DEV-001_BLOCKER_QUEUE.*` show `PKG-17` with 9 committed deliverables and 0 blockers.
- Queue CSV/Markdown entries show `DEL-17-01` through `DEL-17-09` as `SEMANTIC_READY`, `COMMITTED`, and `UNBLOCKED`.
- `execution/_Coordination/DEV-001_IMPLEMENTATION_EVIDENCE.csv` records PKG-17 committed evidence:
  - `DEL-17-01` at `6f1e3427`
  - `DEL-17-02` at `d0e1aa80`
  - `DEL-17-03` at `019ef027`
  - `DEL-17-04` at `8ebf8b33`
  - `DEL-17-05` through `DEL-17-09` at `5961995f`
- Repository history shows `0768fa29` as the queue/evidence refresh commit and `5961995f` as the PKG-17 export workflow deliverable population commit. The queue files themselves do not embed `0768fa29`; they are modified by that commit and embed the deliverable evidence hashes.

## Dependency Alignment Notes

- Local `Dependencies.csv` headers are consistently 29 columns across all nine PKG-17 deliverables.
- CSV row-width validation passes for `DEL-17-01`, `DEL-17-02`, `DEL-17-03`, `DEL-17-04`, `DEL-17-06`, `DEL-17-07`, `DEL-17-08`, and `DEL-17-09`.
- CSV row-width validation fails for `DEL-17-05` because `DEP-017-05-011` has 31 columns.
- Approved DAG-005 sequencing is internally aligned with the requested dependency shape.
- The root and DAG blocker queues are aligned with approved DAG-005 rather than with local extracted-but-unpromoted rows, which is the correct non-gating treatment for candidate/local assumptions.
- No evidence was found that candidate or non-gating assumptions are currently counted as active blockers in the blocker queues.

## Recommendation

Do not accept TP-EXPORT-006 dependency/DAG alignment as clean until `DEL-17-05/Dependencies.csv` row `DEP-017-05-011` is repaired and revalidated. After that repair, treat the local extracted source-basis/interface rows as non-gating local evidence unless a future DAG amendment explicitly promotes them into the active DAG-005 edge set.
