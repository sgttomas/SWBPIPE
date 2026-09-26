# Scope-change, closure-audit and task-management update

Owner-authorized Root tranche `ROOT-WORKFLOW-WAVE2A-CHANGE-CONCERNS-20260926` revises bundled `scope-change`, `audit-scope-closure`, `scope-change-packet` and `task-management` from run evidence:
- Retiring a deliverable makes no lifecycle change (owner ruling): annotate its decomposition row `[RETIRED — {AMENDMENT_ID}]` and append one `_STATUS.md` history line (for example `- {date} — Retired under {AMENDMENT_ID}; lifecycle state remains {CURRENT_STATE} ({actor})`); `write_status.sh` is not used. A historical run whose accepted decision authorized a hand-written RETIRED state is checked against that decision.
- Carriers named in the accepted group-2 boundary (ScopeOfWork, PRD, instruction files) may be amended, each under its own authority.
- Group 1 writes `Intake_Actions.csv` (PROPOSED); `Amendment_Actions.csv` is the accepted group-2 register. A run whose group-1 snapshot already binds `Amendment_Actions.csv` keeps it unchanged and writes its group-2 register under a distinct name (for example `Amendment_Actions_CP2.csv`), bound in the group-2 manifest and named in `Handoff_State.md`.
- New `NOT_STARTED` and `NOT_APPLICABLE` state values; post-change findings are classified `EXPECTED_CONSEQUENCE` or new, with raw `AuditState` and `AdjustedAuditState`; a variant-neutral acceptance-conditional edit step writes `_PostAcceptanceValidation/`.
- Project-qualified amendment IDs are documented, and `tools/query/scan_next_amendment_id.sh` accepts an optional PREFIX (default behavior unchanged).
- `audit-scope-closure` reads `Handoff_State.md`, treats human deferrals as `DEFERRED_BY_HUMAN`/`NOT_ACTIVATED` rather than MAJOR, resolves the accepted register from the group-2 manifest, uses the Pre/Post change coverage files, adds `SUPERSEDED_BY`, judges reruns by input hash, and uses `ASC-ISS-{NNN}` issue IDs.
- `scope-change-packet` maps to the three checkpoint groups (its `SCOPE_CHANGE_Gate` column name is kept for validator compatibility).
- `task-management` treats a TASK dispatch as inspection-only, lists the nine domains, generalizes receipt wording, and routes persistent federation findings.

Historical runs remain readable and are not rewritten. This loop decides its own adoption; this source tranche grants no release.
