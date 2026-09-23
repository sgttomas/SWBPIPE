# Deferred decision slate — include/exclude review before register mutation

This is the manager's include/exclude decision after examining 15 preliminary
ideas. `DEFERRED_TM_CANDIDATES.csv` now contains only the ten selected,
compactly numbered schema-shaped rows. The five excluded ideas have labels
`EXC-PIP-RET-*` here and reserve no register IDs. No canonical register row
had been written at the review boundary. Agent 0 reviewed the ten selected
rows and directed their canonical registration under the owner's default
deferral. `REGISTER.csv` now carries `TM-PIP-043` through `TM-PIP-052` as
DEFERRED, with compact IDs and the source holds below. The
owner's later default deferral applies to **actual unanswered decisions or
missing owners**, not every future product requirement. Source locators below
are exact account keys in `CANDIDATE_ROW_ACCOUNT.csv`; that file retains the
source path and pinned SHA-256. The proposed CSV binds current source hashes,
triggers and holds. D-GOV-33 was rerun with COMPLETE four-register coverage
and zero writes; before any mutation, recheck current IDs and dedup.

## Include as decision attention, with source holds intact

| Candidate | Source and reason this decision is live now | Dedup / trigger and hold |
|---|---|---|
| `TM-PIP-043` | `OWNER_DIRECTION.md` question 2: exact non-issued receiving amendments were asked of the owner and remain specifically unanswered; later default says DEFERRED. | No local row covers this one-time amendment ruling. Trigger exact owner approval/revision/declination of `PROPOSED_RECEIVING_AMENDMENTS.patch`; `DEFER_WITH_HOLD`, no patch application or source removal. |
| `TM-PIP-044` | D-74 ruling plus issued DEL-01-01 `ScopeOfWork.md`: current license conflict is real, and the formal-change target/reissuance act is unruled now. | No existing Piping row; `FORMAL_CHANGE_CANDIDATE.md` stays the owning route. Trigger approved issued postimage and issuance record; `DEFER_WITH_HOLD`, stale issued text cannot be current-license evidence. |
| `TM-PIP-045` | `DEL-04-04:1` and `DEL-04-05:2`: PDU-035 explicitly holds the owner-selected dimensional/conversion basis and formal REVIEW sufficiency. | Not TM-PIP-037, which closed different DEL-09-04 decisions. Trigger selected basis plus each actual REVIEW outcome; `DEFER_WITH_HOLD`, no mechanics acceptance from metadata alone. |
| `TM-PIP-046` | `DEFERRED_RF_PDU_DECISION_DOCKET.md` names 18 source keys whose formal human disposition is explicitly `TBD` or equivalent now, even where technical evidence exists. | No Piping live/closed row covers this finite PDU-060 review forum. Trigger individual owning rulings for every cited finding (or owner-directed split); `DEFER_WITH_HOLD` for each source finding. This is one review-attention row, not 18 product tasks. |
| `TM-PIP-047` | `DEL-07-06:5/:6` and `DEL-07-03:2`: a separately authorized independent usability/security basis is needed before PDU-045/046/049 upgrade. | D-68 ruled touched-control criteria, not independent validation; no TM duplicate. Trigger owner-selected independent basis and candidate-bound review; `DEFER_WITH_HOLD`, `VERIFIED_NOT_VALIDATED` persists. |
| `TM-PIP-048` | `DEL-05-04:1` and `ScopeOfWork.md` storage/presentation TBD: external human-acceptance owner and stale-record invalidation remain genuinely unselected. | No existing row; source FG-DEL-05-04-01 remains open. Trigger owning route and verified negative; `DEFER_WITH_HOLD`, no stale acceptance reuse. |
| `TM-PIP-049` | `DEL-16-04:3`: DEC-094/SCA-009 expressly reserve route/support candidate-generator ownership to a separate act. | TM-PIP-001 concerns D-58 live-provider mechanism, not generator ownership. Trigger owner decomposition/scope-change landing; no presumed DEL-16-04 implementation. |
| `TM-PIP-050` | `DEL-05-03:3`: connector treatment is named but has no identified receiving contract; pressure/public-result parts have homes. | No duplicate found. Trigger owner selection and applied receiving scope; keep connector source clause live; no implementation claim. |
| `TM-PIP-051` | `DEL-12-04:1`: PDU-034 explicitly awaits quarantine/readiness taxonomy and destructive-workflow policy selected by the owner. | No matching row; trigger exact policy selection and PDU review; `DEFER_WITH_HOLD`, no invented destructive behavior. |
| `TM-PIP-052` | `DEL-17-08:2`: PDU-031 exact timestamp/generator metadata behavior is expressly owner-unselected. | TM-PIP-008/021 cover GLB/glTF identity/review geometry, not timestamp behavior. Trigger exact owner policy and export-contract check; `DEFER_WITH_HOLD`, no normative deterministic claim. |

## Exclude from this register mutation

| Candidate | Exact source | Why excluded now; retained trigger and owner |
|---|---|---|
| `EXC-PIP-RET-03` | `DEL-01-03:1`, DEC-027/079 | External intake is intentionally closed. A *future* activation and legal-instrument choice is conditional on owner/counsel initiative, not a current unanswered application. Keep closed-intake source gate; reassess only when activation is proposed. |
| `EXC-PIP-RET-04` | `DEL-01-03:2`, PRD §17.5 | Pre-release legal review is an owned release gate with no selected release candidate. Keep it in DEL-01-03; no register row until a concrete public-release act is proposed. |
| `EXC-PIP-RET-05` | `DEL-00-02:1/:2`, PDU-007 | Required layer/module map and evidence sufficiency are owned architecture production/review work. No complete artifact is before the owner for disposition; keep PDU-007 hold at source. |
| `EXC-PIP-RET-06` | `DEL-03-04:2`, PDU-019 | Bounded negative schema evidence alone is insufficient for formal REVIEW; the owning review route must first assemble the candidate. Keep PDU-019 source hold. |
| `EXC-PIP-RET-08` | `DEL-04-06:1` | Formal REVIEW, current scope parity and claim-map review are owned deliverable review work, not presently a stand-alone owner decision ready to rule. Keep at the DEL-04-06 source. |

Other explicit dedup exclusions: TM-PIP-002–022 already preserve PKG-17
source/adapter TBD attention (including DEL-17-09 signoff family); closed
TM-PIP-034 routed the DEC-092 derivative into DEL-09-04's `_STATUS.md`;
closed TM-PIP-037 covers earlier DEL-09-04 owner gates. D-72 criteria are
ruled and its successor demonstration is owned evidence work. The DEL-07-09
N7 missing-intake recovery is owned evidence work. DEL-17-06 withheld fixture
witnesses remain owned verification. None is promoted just because its
`Remaining` bullet is old.

The ten included rows are now registered as `DEFERRED`. `DEFER_WITH_HOLD` means
attention can wait while the named reliance act cannot proceed or be claimed.
No row grants a product assignment, formal-change authority, release or
source deletion. Registration used the owner's later default-deferral act as
the human basis after Agent 0 reviewed the matrix; independent review of the
actual register diff is still required before integration.
