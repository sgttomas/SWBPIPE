# Candidate Harvest Report — Chirality Piping — 2026-08-02

Status: `DECISION SUPPORT — CANDIDATES ONLY — NOT AUTHORITY`

Invoking loop: `chirality-piping`

Register home: `projects/chirality-piping/execution/_Coordination/_TaskManagement/`

Mode: candidate harvest

Examined Git basis: `cad3553bf530c2c8b00e2b058bbc9d0bba690266`

Register writes: `0`

`SourceSha` below is the Git blob SHA required by the Task Management schema.
Where a routed notice is material, its SHA-256 is also stated separately.
Candidates and proposed dispositions create no scope, priority, approval,
lifecycle, reliance, or work-selection effect.

## 1. Federation preflight

Deterministic preflight:

```text
taskmgmt federation COMPLETE: 4 register(s), 24 finding(s), 24 presented
coverage: COMPLETE; register_writes: 0
LOCAL_LINK_TO_FOREIGN: 23
LOCAL_CLOSED_REMOTE_OPEN: 1
```

All four canonical Git-tracked registers were discovered, read, and validated.
There were no excluded lookalikes, invalid or unreadable registers, duplicate
global IDs, missing notices, orphaned links, or ambiguous references. The 23
local-to-foreign links are the intended `TM-PIP-001..023` linked-row lineage.
The sole closure echo is `TM-PIP-023` CLOSED while linked root row
`TM-ROOT-053` remains OPEN; it is display-only and remains root-owned.

## 2. Sweep coverage and limitation recovered manually

The deterministic v0 scan completed, but it does not implement
`Review_Findings.csv`, review-report sections, receipt fields, or run-record
markers. It also folds candidates on `(basename, class, id)` without loop
identity. That cross-loop fold hid Piping copies of shared-name notices,
including all four `NOTICE_2026-08-02_*` files. The manual completion therefore
covered every PRD §5.1 class and recovered the omitted Piping surfaces.

| Surface | Piping result |
| --- | --- |
| Decision-register non-ruled rows | 0 |
| Coordination notices | 12 files: 4 `TRACKED_OPEN`, 8 absent from the historical program ledger |
| Evaluation `FINDINGS.csv` | 32 row signals in 3 immutable reports |
| `Review_Findings.csv` | 75 row signals in 40 files: 28 `TECHNICALLY_ADDRESSED_PENDING_HUMAN`, 46 `OPEN`, 1 `DEFERRED` |
| HOLD registers | none present under the Piping working root |
| Handoff blockers | no current unreleased blocker after rechecking later handoffs and Receipt 86 |
| Packet fields | no open `HumanRuling=TBD` packet row |
| TBD registers | 21 rows, all already linked as `TM-PIP-002..022` |
| Slates / work graphs / `## Remaining` | fenced from work discovery; no new held-open decision harvested |
| New `plans/` review reports | none since the prior harvest; the maturity-survey HTML remains candidate input, not findings of record |
| Receipt parked lanes | no new current parked-lane candidate; the legacy line is superseded by current receipts and live registers |
| Run-record markers | one substantive `MISSING:` block; its source-gap concern duplicates the deliverable review deferral |

## 3. Candidates presented for owner promotion ruling

### HC-PIP-20260802-001 — Product-delivery direction requires a Piping-owned product-basis act

- SourceRef: `projects/chirality-piping/execution/_Coordination/NOTICE_2026-08-02_PRODUCT_DELIVERY_DIRECTION.md`, `## Follow-on for this loop`
- SourceSha: `1ddc966d3c43e7f5d5b021288c6929f444ca96c2`
- Source SHA-256: `0386b64a87b49e77163bbf4b7ff467427255e5a6afe73a66bc96649637b6a73e`
- Reciprocal context: root owner-intent record Git blob `4b1bdafdb4a8e39e6942e74ebaf4028162436be3`; prior Piping design-tool-boundary record Git blob `2c3d9aca71844052734c232a3f342f6d71391ad6`.
- Concern: The loop must adopt, amend, defer, or decline the recorded direction through a separate owner-initiated product-basis act before publication, tailored runtime/harness, robust UI, or semantically equivalent agent API can affect Piping scope or planning.
- Domain lenses: `Decisions; Planning; Deliverables`
- Proposed disposition: `RESOLVED_BY_DECISION` — eventually, by the Piping-owned product-basis record; the candidate must remain open/deferred until that act exists.
- Promotion recommendation: **PROMOTE** as the next local row, cross-related to `TM-PIP-001` and root `TM-ROOT-102`, without treating either intent record as scope.

This candidate does not authorize product-direction work and does not expand
the two intent records. Re-verification for any later product act must use
current product, decomposition, dependency, UI, and API evidence rather than
the maturity-survey HTML as findings of record.

### HC-PIP-20260802-002 — Cross-loop notice dedup causes false-negative harvest results

- SourceRef: `tools/taskmgmt/taskmgmt.py`, `scan_notice_ledgers`, `dedup_canonical`, and the completed scan output; evidence notices are the four tracked Piping `NOTICE_2026-08-02_*.md` files.
- SourceSha: `423ed7540ba9b7e01260313408d8ca33c809faab`
- Source SHA-256: `224dab8092e901a2ef0d03fef257babb714cac1ec4b28fbfb81c7af07b33c243`
- Inbound report for reciprocal routing: `projects/chirality-piping/execution/_Coordination/NOTICE_2026-08-02_TASK_MANAGEMENT_FEDERATION_PREFLIGHT.md`, SHA-256 `0e8f2df1327f8bbc4798ece158cf58543bf3d45c4985495ffcc6055966de683f`.
- Concern: Candidate dedup omits loop identity, so same-named notices routed to multiple loops collapse to one candidate and a loop-scoped harvest can falsely report that its own notice was not found.
- Domain lenses: `Action Item; Checking`
- Proposed disposition: `RESOLVED_WITH_CHANGE` — by a root-owned scanner repair with a regression fixture proving same-basename notices in different loops remain distinct.
- Promotion recommendation: **PROMOTE AND ELEVATE** through a Piping row plus ordinary draft notice to the root loop; no foreign register write.

The required root-directed draft notice is not written yet because its
reciprocal-citation contract requires the actual promoted Piping row ID. If
the owner promotes this candidate, the row is minted first and the draft is
then prepared under the session closeout tranche; it is not sent or written
to Root without the later closeout gate.

## 4. Harvested signals not recommended for new rows

Each grouped entry below is a deduplicated candidate. All underlying row IDs
and their SourceSha values are retained in §5.

| Candidate | SourceRef + SourceSha | Concern in one sentence | Domain lenses | Proposed disposition | Recommendation |
| --- | --- | --- | --- | --- | --- |
| HC-PIP-20260802-003 | `NOTICE_2026-08-02_ROOT_HARNESS_SCOPE_CORRECTION.md` `5e2bf4ae4d5aa52efebd082c836ff786dc1de5b9`; `NOTICE_2026-08-02_TASK_MANAGEMENT_INSTRUCTION_SCHEMA_REPAIR.md` `32a5c22c5ac835f79652639ea7617cb9163d5cb9`; `NOTICE_2026-08-02_TASK_MANAGEMENT_FEDERATION_PREFLIGHT.md` `b300381d99937d8485e740c6f8ce8cf34a37aaae` | The three Root governance notices request no current Piping change unless a local live mirror or pin is later identified. | Checking; Decisions | `INFORMATIONAL_NO_ACTION` | Do not promote. |
| HC-PIP-20260802-004 | Eight earlier notice files listed in §5.1 | Earlier coordination residues are either already adopted/consumed, superseded by D-58/D-63/current Root doctrine, or expressly non-gating. | Checking; Decisions | `OBE` for superseded mismatch/adoption notices; otherwise `INFORMATIONAL_NO_ACTION` | Do not promote; source ledgers remain external state. |
| HC-PIP-20260802-005 | DAG-009 `FINDINGS.csv` rows 001–008, `b284d596a82b5cd8a152479569143a7da57e40de` | Validation and exact-application observations supported the now-completed DAG-009 activation and create no residual attention item. | Checking; Deliverables | `INFORMATIONAL_NO_ACTION` | Do not promote. |
| HC-PIP-20260802-006 | DAG-009 `FINDINGS.csv` row 009, `b284d596a82b5cd8a152479569143a7da57e40de` | The recorded activation owner gates were resolved by the DAG-009 approval and pointer activation while DEC-092 implementation remains separately gated. | Approval; Decisions | `RESOLVED_BY_DECISION` | Do not promote; Receipt 86 and the approval record are closure evidence. |
| HC-PIP-20260802-007 | DAG-008 `FINDINGS.csv` rows 001–007, `40107abf6fd164ab4a309697524cd2b071de85d0` | DAG-008 validation and selection-gate signals are historical after accepted DAG-009 succession. | Checking; Decisions | `OBE` | Do not promote. |
| HC-PIP-20260802-008 | DAG-008 `FINDINGS.csv` row 008, `40107abf6fd164ab4a309697524cd2b071de85d0` | The separate D-45 owner gate was closed by the recorded O-B ruling and `TM-PIP-023`. | Decisions | `DUPLICATE` of `TM-PIP-023` | Do not promote. |
| HC-PIP-20260802-009 | R15 dependency-readiness rows 001–013, `527d98120dc5ce48f8fe46364cd7a5ac5313e9eb` | The stale dependency-currency concerns were discharged through the DAG-008 and DAG-009 successor changes and are historical audit inputs. | Checking; Deliverables | `RESOLVED_WITH_CHANGE` | Do not promote. |
| HC-PIP-20260802-010 | R15 rows GATE-001/GATE-002, `527d98120dc5ce48f8fe46364cd7a5ac5313e9eb` | The selection gate was resolved by later owner DAG acts and the exact deferral boundary remains evidence, not an open action item. | Approval; Decisions | `RESOLVED_BY_DECISION` for GATE-001; `INFORMATIONAL_NO_ACTION` for GATE-002 | Do not promote. |
| HC-PIP-20260802-011 | `TP-EXPORT-006_TBD_Register.csv`, `7bedff5460302f860bb37df635e81f82333fcb77`, all 21 rows | Every structurally uncloseable TBD signal already survives as one exact linked local row. | Deliverables | `DUPLICATE` of `TM-PIP-002..022` | Do not promote. |
| HC-PIP-20260802-012 | 28 `TECHNICALLY_ADDRESSED_PENDING_HUMAN` review rows listed in §5.4 | Technical responses are recorded but their owning review surfaces retain human disposition, so bulk register promotion would duplicate REVIEW rather than establish current closure. | Checking; Approval | `RESOLVED_WITH_CHANGE` only after a current bounded REVIEW backcheck and owner disposition | Do not promote in this harvest; route through REVIEW if the owner wants the historical rows regularized. |
| HC-PIP-20260802-013 | 46 `OPEN` review rows listed in §5.5 | The rows describe ordinary deliverable work, stale-document cleanup, dependency evidence, or bounded target-support TBDs and therefore do not pass the §7.1 promotion gate as a single durable action item. | Deliverables; Work; Checking | `INFORMATIONAL_NO_ACTION`; target-support duplicates use `DUPLICATE` of `TM-PIP-002..022` | Do not promote en masse; owning deliverable/review instruments remain responsible. |
| HC-PIP-20260802-014 | DEL-11-03 review row RF-11-03-C-003 `76c20c68e6de78d576c4d9c2a5b126444c1c4c1a` and run record `c3ee30e08dc955af3f699e97ad31c5620dfc2d70` | The public-history, open-frame, and local-FEA source gaps are one explicit deferred deliverable concern repeated in both a review row and a `MISSING:` block. | Deliverables; Checking | `DUPLICATE` at harvest; retain the review row as the surviving source signal | Do not promote separately. |

## 5. Source inventory for grouped candidates

### 5.1 Notice signals

| SourceRef | SourceSha | Ledger state | Harvest disposition |
| --- | --- | --- | --- |
| `NOTICE_2026-07-25_helps_humans_p1_p7_applied.md` | `786a14cbc52d7cf50c2f7c42e8848ea9eee923ad` | `TRACKED_OPEN` | `INFORMATIONAL_NO_ACTION` |
| `NOTICE_2026-07-27_DAPP48_D30_MISMATCH.md` | `e1e4f952d67e2548bccafdc0b56cc9c297e36749` | `TRACKED_OPEN` | `OBE` |
| `NOTICE_2026-07-27_d30_contract_mismatch_and_root_runtime_context.md` | `2ecd55aca78f8561dfbc1ee519b745be095cd2f9` | `TRACKED_OPEN` | `OBE` |
| `NOTICE_2026-08-01_D63_TASK_MANAGEMENT_ADOPTION_AND_D45_CLOSURE.md` | `f735aa3c74d6fa7403e8c5a2a826d745a2f345cd` | absent | `OBE` after D-63/D-45 closeout |
| `NOTICE_2026-08-02_PRODUCT_DELIVERY_DIRECTION.md` | `1ddc966d3c43e7f5d5b021288c6929f444ca96c2` | absent | candidate HC-001 |
| `NOTICE_2026-08-02_ROOT_HARNESS_SCOPE_CORRECTION.md` | `5e2bf4ae4d5aa52efebd082c836ff786dc1de5b9` | absent | `INFORMATIONAL_NO_ACTION` |
| `NOTICE_2026-08-02_TASK_MANAGEMENT_FEDERATION_PREFLIGHT.md` | `b300381d99937d8485e740c6f8ce8cf34a37aaae` | absent | `INFORMATIONAL_NO_ACTION`; evidence for HC-002 |
| `NOTICE_2026-08-02_TASK_MANAGEMENT_INSTRUCTION_SCHEMA_REPAIR.md` | `32a5c22c5ac835f79652639ea7617cb9163d5cb9` | absent | `INFORMATIONAL_NO_ACTION` |
| `NOTICE_AGENT_INDEX_TASK_MANAGEMENT_2026-07-31.md` | `b18b4d0294bd8ccb7d864fa9af8b844929c01d04` | absent | `OBE` after D-63 |
| `NOTICE_D-GOV-31_MERGE_GATE_POLICY_SUCCESSION.md` | `a638d8305736de37d189eb187f91fd23331e26fb` | absent | `INFORMATIONAL_NO_ACTION`; local discipline remains controlling |
| `NOTICE_D-GOV-32_TASK_MANAGEMENT_STANDARD.md` | `d31eacd922f8b34745065941d495579f0c281951` | absent | `OBE` after D-63 |
| `NOTICE_D-T0-24_FLOW_A_ROOT_IDENTITY_SEPARATION.md` | `aac1d9cd125d09f2ecca7a24649c4fca08fd8e8d` | `TRACKED_OPEN` | `INFORMATIONAL_NO_ACTION` |

All paths in this table are relative to
`projects/chirality-piping/execution/_Coordination/`.

### 5.2 Evaluation rows

- `projects/chirality-piping/execution/_Evaluation/DAG008_CANDIDATE_EVALUATION_2026-07-22_R15/FINDINGS.csv`; SourceSha `40107abf6fd164ab4a309697524cd2b071de85d0`; rows `DAG008-EVAL-001..008`.
- `projects/chirality-piping/execution/_Evaluation/DAG009_CANDIDATE_EVALUATION_2026-08-02_R23/FINDINGS.csv`; SourceSha `b284d596a82b5cd8a152479569143a7da57e40de`; rows `DAG009-EVAL-001..009`.
- `projects/chirality-piping/execution/_Evaluation/DEPENDENCY_READINESS_AUDIT_2026-07-21_R15/FINDINGS.csv`; SourceSha `527d98120dc5ce48f8fe46364cd7a5ac5313e9eb`; rows `R15-DEP-001..013`, `R15-GATE-001..002`.

### 5.3 TBD rows

`projects/chirality-piping/execution/PKG-17_Export Format Interoperability/1_Working/_audit/TP-EXPORT-006_TBD_Register.csv`; SourceSha `7bedff5460302f860bb37df635e81f82333fcb77`; rows `TBD-17-01-001..007`, `TBD-17-03-001..004`, `TBD-17-04-001..004`, `TBD-17-05-001..002`, `TBD-17-06-001`, `TBD-17-07-001`, `TBD-17-08-001`, `TBD-17-09-001`; exact surviving rows `TM-PIP-002..022`.

### 5.4 Review rows technically addressed pending human disposition

Each source below proposes `RESOLVED_WITH_CHANGE` only after a current REVIEW
backcheck; this report does not accept the technical response.

| SourceRef | SourceSha | Row IDs |
| --- | --- | --- |
| `PKG-04/DEL-04-03/Review_Findings.csv` | `61a45e5b1df555a9aa40a9b508d6c32676b7206d` | `PKG04-DEL0403-PKG02-001` |
| `PKG-04/DEL-04-04/Review_Findings.csv` | `ab46e916dfadc98153676cb9cc50ec9bcd0bc159` | `PKG04-DEL0404-PKG02-001..002` |
| `PKG-04/DEL-04-05/Review_Findings.csv` | `b0839f03a9d2f15f043173bd5a13dae5dbb2724a` | `PKG04-DEL0405-PKG02-001` |
| `PKG-04/DEL-04-06/Review_Findings.csv` | `3d71acac39f0cdb09a393c1af564f56cfb86342b` | `PKG04-DEL0406-PKG02-001..002` |
| `PKG-08/DEL-08-02/Review_Findings.csv` | `879db78a9725df43b79da207f2090fd9ca066139` | `DEL-08-02-PKG02-001..002` |
| `PKG-09/DEL-09-01/Review_Findings.csv` | `d1b83e25fac78502ebd337f29c083091a7067c6d` | `PKG09-0901-PKG02-001` |
| `PKG-09/DEL-09-02/Review_Findings.csv` | `20ecb6dc34f5bce9fa71289d973c1ee7c4337705` | `PKG09-0902-PKG02-001` |
| `PKG-09/DEL-09-03/Review_Findings.csv` | `a31dda63d69d0dd437afb3454bbd45fe59cd1a35` | `PKG09-0903-PKG02-001..002` |
| `PKG-10/DEL-10-01/Review_Findings.csv` | `35849021107f3485fb76670e84d3281fd4aa18af` | `PKG10-DEL1001-PKG02-W001` |
| `PKG-11/DEL-11-04/Review_Findings.csv` | `5c0ad92ba600746f1e3a52534a20d5b1e2fb5fd6` | `PKG11-DEL-11-04-PKG02-001..002` |
| `PKG-13/DEL-13-01/Review_Findings.csv` | `abff6ddf979a4a6b50b335708eeb559ec55b13d3` | `PKG13-DEL-13-01-PKG02-001..002` |
| `PKG-13/DEL-13-02/Review_Findings.csv` | `123e4f85cf232b0534b92be0dcb880b7503c3e3b` | `PKG13-DEL-13-02-PKG02-001` |
| `PKG-13/DEL-13-03/Review_Findings.csv` | `8ff75e99475ab8d07d9b094eb49313ad68e9ba88` | `PKG13-DEL-13-03-PKG02-001` |
| `PKG-13/DEL-13-04/Review_Findings.csv` | `294e9ef96930afc6c03587d033350fe308041229` | `PKG13-DEL-13-04-PKG02-001..002` |
| `PKG-15/DEL-15-02/Review_Findings.csv` | `760d4fb1b0680b2e7c86fd44b6986376bfe298bc` | `DEL-15-02-PKG02-001` |
| `PKG-15/DEL-15-03/Review_Findings.csv` | `7f12db062c91710bd53f3965fbdafd5ec5eb2da8` | `DEL-15-03-PKG02-001` |
| `PKG-16/DEL-16-01/Review_Findings.csv` | `05a54b21865f64a878265b87c1542a6d69e213fd` | `PKG16-DEL1601-PKG02-001..002` |
| `PKG-16/DEL-16-03/Review_Findings.csv` | `8f3828a0a2d67b9188419926ebe942685b3d62f9` | `PKG16-DEL1603-PKG02-001..002` |
| `PKG-16/DEL-16-04/Review_Findings.csv` | `aaa144be23fecfb16c8d97eb044e7319febba483` | `PKG16-DEL1604-PKG02-001` |

The abbreviated paths in §§5.4–5.5 expand under
`projects/chirality-piping/execution/<PKG name>/1_Working/<DEL name>/`.

### 5.5 Review rows left OPEN

These rows are not recommended for bulk register promotion. Their proposed
Task Management disposition is `INFORMATIONAL_NO_ACTION`, except explicit
target-support/TBD duplicates already carried by `TM-PIP-002..022`.

| SourceRef | SourceSha | Row IDs |
| --- | --- | --- |
| `PKG-02/DEL-02-01/Review_Findings.csv` | `df55b66fcbc094e04b24d83f6cc39995408d763c` | `RF-001..005` |
| `PKG-02/DEL-02-02/Review_Findings.csv` | `aaebbc5b350a3bcfaf1cbd0022ce866fcef5475e` | `RF-001..004` |
| `PKG-02/DEL-02-03/Review_Findings.csv` | `ccf517d6c8c73c3abe406b747db631a767b87209` | `RF-001..003` |
| `PKG-02/DEL-02-04/Review_Findings.csv` | `c60d0b4d8ab7984476e53f08dc060b35536cf9d8` | `RF-001..003` |
| `PKG-02/DEL-02-05/Review_Findings.csv` | `b4582fb5c2eca77244d34a684497219741fb628f` | `RF-001..004` |
| `PKG-12/DEL-12-01/Review_Findings.csv` | `b5aed8882d34782652c10c729ffaedb24f5ffbda` | `RF-001..002` |
| `PKG-12/DEL-12-02/Review_Findings.csv` | `521f84c573881ff477fc7ee0c11457af2397ef32` | `RF-001..002` |
| `PKG-12/DEL-12-03/Review_Findings.csv` | `7ca7b04341fa23f5187dba669def19314a149497` | `RF-001..002` |
| `PKG-12/DEL-12-04/Review_Findings.csv` | `0d7655d145e8898f4c937655d1249423fb56e2bc` | `RF-001..002` |
| `PKG-14/DEL-14-03/Review_Findings.csv` | `fc0d61d858a687ca9bf25716e5eca302f74b1886` | `RF-001..002` |
| `PKG-15/DEL-15-01/Review_Findings.csv` | `75395ea9f6b13c5a471493e8f9a07c86c676014b` | `RF-001..002` |
| `PKG-15/DEL-15-02/Review_Findings.csv` | `760d4fb1b0680b2e7c86fd44b6986376bfe298bc` | `RF-001..002` |
| `PKG-15/DEL-15-03/Review_Findings.csv` | `7f12db062c91710bd53f3965fbdafd5ec5eb2da8` | `RF-001..002` |
| `PKG-15/DEL-15-04/Review_Findings.csv` | `59c418318947d91700afdb18b7cee707d183d5fb` | `RF-001` |
| `PKG-17/DEL-17-02/Review_Findings.csv` | `179c021cdcdc90efce85c2db8b13e9162a2e980f` | `RF-001` |
| `PKG-17/DEL-17-03/Review_Findings.csv` | `9ab324525f1b2fdfc78a360bce849519c9ce0166` | `RF-002` |
| `PKG-17/DEL-17-04/Review_Findings.csv` | `8221d23e6be0b61bcc8a14aae4e3e0757d86ba3f` | `DEL-17-04-RF-002` |
| `PKG-17/DEL-17-05/Review_Findings.csv` | `80f53673db2334726650633488b2edc12ee1ac8f` | `DEL-17-05-RF-001..002` |
| `PKG-17/DEL-17-06/Review_Findings.csv` | `3f25f796fc7d9daf4f1062c443c03feb513d504c` | `DEL-17-06-RF-001` |
| `PKG-17/DEL-17-07/Review_Findings.csv` | `21797b02cca6eab32af9cf2f7aab501b97dd5b2f` | `RF-001..002` |
| `PKG-17/DEL-17-08/Review_Findings.csv` | `aa82fe312b682ddaf6bd6035dc000a60dcd87eec` | `DEL-17-08-RF-002` |
| `PKG-17/DEL-17-09/Review_Findings.csv` | `b2b77c7d8bb4724f10d1fb16d7f620db488b10d6` | `DEL-17-09-RF-001` |

### 5.6 Deferred review row and run marker

- `projects/chirality-piping/execution/PKG-11_Documentation, Examples, and Education/1_Working/DEL-11-03_Theory notes- classical to modern centerline analysis/Review_Findings.csv`, row `RF-11-03-C-003`, SourceSha `76c20c68e6de78d576c4d9c2a5b126444c1c4c1a`.
- Same deliverable `_run_records/TASK_RUN_2026-06-07_1624.md`, `MISSING:` block, SourceSha `c3ee30e08dc955af3f699e97ad31c5620dfc2d70`.

## 6. Requested owner ruling

Recommended ruling:

1. Promote `HC-PIP-20260802-001` as a new Piping Decisions/Planning/
   Deliverables-domain row, initially deferred to the owner-initiated
   product-basis act; do not treat the intent records as scope.
2. Promote `HC-PIP-20260802-002` as a Checking/Action Item-domain row and
   elevate it to Root through an ordinary draft notice prepared only after
   the local row exists.
3. Decline promotion of `HC-PIP-20260802-003..014` with the proposed
   dispositions above; this leaves their owning source surfaces unchanged.

No register row, foreign notice, product artifact, lifecycle/status surface,
dependency, DAG, pointer, decision record, or scope instrument was modified
by this harvest.
