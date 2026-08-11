# Executed register mutation manifest — TM-PIP-040

Status: `EXECUTED EXACTLY — OWNER-RULED CLOSURE`

## Register identities

| Surface | Before SHA-256 | After SHA-256 | Before Git blob | After content blob |
| --- | --- | --- | --- | --- |
| `projects/chirality-piping/execution/_Coordination/_TaskManagement/REGISTER.csv` | `60a8e4956c4f94cc7b64a886fb5c8060f026b010c0bc012d8296fd2044b2a30c` | `2175d2c4db7a480cd6ff77b9964d3815ff7558361df3a132838763d49a49ebfe` | `8574d9df2ff4fdf2ca85cd51dd1b74ddd99fefdd` | `4d19f55bd90dac938b12b970abf4d3729daa0154` |
| `projects/chirality-piping/execution/_Coordination/_TaskManagement/REGISTER_CLOSED.csv` | `9f57d89c8c1298c3b033d32bb4494a6ddeb765586f8640eb2c8310aeaddc837f` | `a92c7c7ebceca79a6bfbbf5b1eb94063a6c1099b734b9e26167bf5726556369f` | `bc3540959788c649ed189f4d9aba96b5fbc64aeb` | `72834422d5308636c6ee7554344cc9c7d0af03d2` |

The after content blobs are local `git hash-object` identities, not commit or
merge claims.

## Exact eight-field mutation

Only `TM-PIP-040` changed semantically:

| Field | Before | After |
| --- | --- | --- |
| `Status` | `OPEN` | `CLOSED` |
| `Disposition` | empty | `RESOLVED_BY_DECISION` |
| `EvidenceRef` | empty | `execution/_Reconciliation/DeliverableConcordance/TM_PIP_038_040_TREATMENT_2026-08-09/OWNER_RULING_2026-08-09_TM_PIP_038_040/OWNER_TREATMENT_RULINGS.md; execution/_Reconciliation/DeliverableConcordance/TM_PIP_038_040_TREATMENT_2026-08-09/OWNER_RULING_2026-08-09_TM_PIP_038_040/TM_PIP_040_LOST_OUTCOME_RECORD.md` |
| `EvidenceSha` | empty | `dfc3b8faf0cfe336f4c8a47e4593ea9add134c9b; cc7770df165286d4fb523131f28b7340d41216b8` |
| `EvidenceQuote` | empty | `TM-PIP-040 outcome selection: LOST; Propose closure of TM-PIP-040 as RESOLVED_BY_DECISION at the next TASK_MANAGEMENT closure session, citing this ruling and the recorded outcome; disposition applies only on my closure ruling.` |
| `LastReviewed` | `2026-08-08` | `2026-08-10` |
| `Closed` | empty | `2026-08-10` |
| `Notes` | frozen historical value | frozen historical value plus the exact owner-closure clause in the proposed manifest |

Every other field on `TM-PIP-040` and every field on every other row is
byte-preserved after accounting for the helper's deterministic CSV storage.

## Exact row before mutation

SHA-256 of the following CSV record including terminal LF:
`a7f1620e09ab76fdf073239dd841977649e6a2a018e185000f1256a26c8649b6`.

```csv
"1.0","TM-PIP-040","Determine fate of Addendum-9 frozen-worktree artifact sets","The D-41 frozen evidence worktree that carried six untracked ignored artifact sets no longer exists, and no committed record establishes whether the sets were restored or lost; the unresolved evidence-outcome question must be investigated and dispositioned without treating the absent worktree as proof either way.","execution/_Coordination/_TaskManagement/CANDIDATE_HARVEST_REPORT_2026-08-08.md HC-PIP-20260808-003; execution/_Coordination/NOTICE_2026-08-03_ROOT_PIPING_RESUME_RESIDUALS.md item 3; execution/_Reconciliation/DeliverableConcordance/DELIVERABLE_CONCORDANCE_2026-07-11_1305/RUN_BASIS.md Addendum-9 incident","7768e0768d0a2987181eb276f2c6da7d873ae3de; ac5086f3e02a72bd598fe6ba46eec03e65176f49; f4d8a44324e8a8bdb6edb74577d05f0d32aac44a","HC-PIP-20260808-003","Checking; Decisions; Work","D-41 deliverable concordance; Addendum-9 frozen evidence; six ignored artifact sets","","","RECONCILIATION through human-routed handoff / bounded evidence investigation (R); TASK_MANAGEMENT prepares; A=human-only","MEDIUM","Owner ruling 2026-08-08: substantive unresolved evidence-loss question","OPEN","","","","","","","2026-08-08","2026-08-08","","OWNER_RULING_2026-08-08_HARVEST.md: PROMOTE exactly as recommended. Resolution requires a bounded evidence investigation and an evidence-bound owner disposition; the absent worktree alone proves neither restoration nor loss. No investigation dispatch, reconciliation write, or disposition is authorized before the closeout-routed handoff and its owning gates."
```

## Exact archived row after mutation

SHA-256 of the following CSV record including terminal LF:
`a3bcc1b866cdd5f1b8411f30d118c39a0738ec82dd8a451935d344b0016eb5c1`.

```csv
"1.0","TM-PIP-040","Determine fate of Addendum-9 frozen-worktree artifact sets","The D-41 frozen evidence worktree that carried six untracked ignored artifact sets no longer exists, and no committed record establishes whether the sets were restored or lost; the unresolved evidence-outcome question must be investigated and dispositioned without treating the absent worktree as proof either way.","execution/_Coordination/_TaskManagement/CANDIDATE_HARVEST_REPORT_2026-08-08.md HC-PIP-20260808-003; execution/_Coordination/NOTICE_2026-08-03_ROOT_PIPING_RESUME_RESIDUALS.md item 3; execution/_Reconciliation/DeliverableConcordance/DELIVERABLE_CONCORDANCE_2026-07-11_1305/RUN_BASIS.md Addendum-9 incident","7768e0768d0a2987181eb276f2c6da7d873ae3de; ac5086f3e02a72bd598fe6ba46eec03e65176f49; f4d8a44324e8a8bdb6edb74577d05f0d32aac44a","HC-PIP-20260808-003","Checking; Decisions; Work","D-41 deliverable concordance; Addendum-9 frozen evidence; six ignored artifact sets","","","RECONCILIATION through human-routed handoff / bounded evidence investigation (R); TASK_MANAGEMENT prepares; A=human-only","MEDIUM","Owner ruling 2026-08-08: substantive unresolved evidence-loss question","CLOSED","","","RESOLVED_BY_DECISION","execution/_Reconciliation/DeliverableConcordance/TM_PIP_038_040_TREATMENT_2026-08-09/OWNER_RULING_2026-08-09_TM_PIP_038_040/OWNER_TREATMENT_RULINGS.md; execution/_Reconciliation/DeliverableConcordance/TM_PIP_038_040_TREATMENT_2026-08-09/OWNER_RULING_2026-08-09_TM_PIP_038_040/TM_PIP_040_LOST_OUTCOME_RECORD.md","dfc3b8faf0cfe336f4c8a47e4593ea9add134c9b; cc7770df165286d4fb523131f28b7340d41216b8","TM-PIP-040 outcome selection: LOST; Propose closure of TM-PIP-040 as RESOLVED_BY_DECISION at the next TASK_MANAGEMENT closure session, citing this ruling and the recorded outcome; disposition applies only on my closure ruling.","2026-08-08","2026-08-10","2026-08-10","OWNER_RULING_2026-08-08_HARVEST.md: PROMOTE exactly as recommended. Resolution requires a bounded evidence investigation and an evidence-bound owner disposition; the absent worktree alone proves neither restoration nor loss. No investigation dispatch, reconciliation write, or disposition is authorized before the closeout-routed handoff and its owning gates.; OWNER_CLOSURE_RULING_2026-08-10: CLOSED / RESOLVED_BY_DECISION on the accepted owner LOST outcome recorded in OWNER_TREATMENT_RULINGS.md and TM_PIP_040_LOST_OUTCOME_RECORD.md. Closure records disposition of this attention row only: the original ignored objects remain unavailable, further recovery is declined, and historical test results and ledger encodings remain evidence of record. No TM-PIP-038/039 closure, lifecycle, release, reliance, scope, reconstruction, recovery, product-validation, or professional-approval effect is created."
```

## Storage result

- Before: live 34, archive 6, combined 40.
- After: live 33, archive 7, combined 40.
- `TM-PIP-040` is absent from live and occurs once in archive.
- The six pre-existing archive rows retain exact order and field identity; the
  closed `TM-PIP-040` row is appended seventh.
- `TM-PIP-038` and `TM-PIP-039` remain live and `OPEN`.

The relocation is storage mechanics only. The owner ruling is the disposition
act.
