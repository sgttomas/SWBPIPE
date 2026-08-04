# Candidate Harvest Report — Chirality Piping — 2026-08-03

Status: `DECISION SUPPORT — CANDIDATES ONLY — NOT AUTHORITY`

Invoking loop: `chirality-piping`

Register home: `projects/chirality-piping/execution/_Coordination/_TaskManagement/`

Mode: candidate harvest (generational pass, step 2)

Examined Git basis: `def4437d1586e730446a1537adfb8af1c512f626`

Prior harvest basis: `cad3553bf530c2c8b00e2b058bbc9d0bba690266` (report of
2026-08-02; owner ruling of record `OWNER_RULING_2026-08-02_HARVEST_SLATE.md`)

Register writes: `0`

`SourceSha` below is the Git blob SHA required by the Task Management schema.
Candidates and proposed dispositions create no scope, priority, approval,
lifecycle, reliance, or work-selection effect. All paths are relative to
`projects/chirality-piping/` unless stated otherwise.

## 1. Federation preflight

Deterministic preflight (`taskmgmt federation`), run before this mode:

```text
taskmgmt federation COMPLETE: 4 register(s), 66 finding(s), 44 presented
coverage: COMPLETE; register_writes: 0
PEC: OPEN=11 DEFERRED=1; ROOT: OPEN=5 DEFERRED=32; APP: OPEN=7 DEFERRED=4; PIP: OPEN=4 DEFERRED=23
LOCAL_LINK_TO_FOREIGN: 23; REMOTE_CLOSED_LOCAL_OPEN: 42; LOCAL_CLOSED_REMOTE_OPEN: 1
```

All four canonical registers validated `PASS`; zero writes; no excluded
lookalikes, unreadable inputs, or operational errors. The dominant
PIP-relevant signal — 21 `REMOTE_CLOSED_LOCAL_OPEN` findings pairing
`TM-PIP-002..022` with now-CLOSED root rows `TM-ROOT-077..097` — is deferral-
review input (step 3), not harvest input, and is carried forward there.

## 2. Sweep coverage

| §5.1 class | Method | Result |
| --- | --- | --- |
| Decision-register non-ruled rows | `taskmgmt scan` | 0 for Piping |
| Notices vs. ledger | `taskmgmt scan` | 13 unledgered on the Piping surface; 8 previously ruled (HC-20260802-003/004, TM-PIP-025); 5 new — see §3/§4 |
| Evaluation `FINDINGS.csv` | `taskmgmt scan` + SHA comparison | 32 row signals; all three files byte-identical to the ruled 2026-08-02 population (blob SHAs `40107ab`, `b284d59`, `527d981`) — previously ruled, not re-presented |
| TBD registers | `taskmgmt scan` + SHA comparison | 21 rows; `TP-EXPORT-006_TBD_Register.csv` byte-identical (`7bedff5`) — all carried by `TM-PIP-002..022` |
| Packet fields | `taskmgmt scan` + manual check | 0 open `HumanRuling=TBD` rows (the two May `_Aggregation` `Conflicts.csv` files carry none) |
| Handoff blockers | `taskmgmt scan` + manual recheck | R13 `HANDOFF_STATE.md` re-verified against current DEL-09-04 state — yields candidate HC-007; other blockers are ordinary deliverable residuals or released |
| `Review_Findings.csv` | git diff since prior basis | zero files changed since `cad3553bf`; the ruled HC-20260802-012/013 populations stand |
| HOLD registers | find sweep | none exist under the Piping root |
| Slates / `## Remaining` | fenced | no held-open decision harvested; fence respected |
| Receipt parked lanes | manual (Receipts 87–88 delta) | no `Parked lanes:` field in this loop's receipt format; Gate-Outcome deferral language harvested — HC-004, HC-006 |
| Run-record markers | manual sweep (dispatched verification) | zero live `TM-CANDIDATE:` markers anywhere; all `NEEDS_HUMAN_RULING:`/`MISSING:` occurrences independently verified historical, null, or duplicates of ruled candidates — see §5 |
| New run records since basis | manual read (dispatched sweep) | DEC-092 WORKING_ITEMS run + candidate brief read in full — HC-004, HC-005, HC-006 |
| New coordination surfaces since basis | manual read (dispatched sweep) | 5 notices + 1 coordination response read in full — HC-001, HC-002, HC-003; three notices carry "no follow-on" and are so dispositioned in §4 |

Manual sweep executed per the TM-launcher remediation steer via two dispatched
read-only Agent 2 generalist sweeps (opus-5), fan-in validated against source
bytes by this manager (every load-bearing quote re-grepped before
presentation).

## 3. Candidates presented for owner promotion ruling

### HC-PIP-20260803-001 — Deterministic harvest scanner has declared marker-class blind spots

- SourceRef: `execution/_Coordination/NOTICE_2026-08-03_TM_LAUNCHER_REMEDIATION.md`; corroborating tool source repo-root `tools/taskmgmt/taskmgmt.py` (scan exclusions declaration)
- SourceSha: notice `95cc87d6857f8f24919bf376c84783b7a8db0be6`; tool `a9d05b2fe055a5bc6eb795b1c5a5b009ec65de41`
- Concern: `taskmgmt scan` v0 skips run-record markers (`TM-CANDIDATE:`, `NEEDS_HUMAN_RULING:`, `MISSING:`), review-report sections, and per-document token mode, so every harvest depends on an unvalidated manual sweep for those classes; this session's sweep additionally proved the colon-form grep misses heading-form `## NEEDS_HUMAN_RULING` markers (three PKG-04 fan-in records recovered only by widened regex `(^|##\s*)(NEEDS_HUMAN_RULING|MISSING)\b:?`).
- Domain lenses: `Checking; Work; Planning`
- Proposed disposition: `RESOLVED_WITH_CHANGE` — eventually, by a root-owned scanner extension implementing the missing classes (including heading-form markers), with fixtures.
- Recommendation: **PROMOTE AND ELEVATE** to Root through a draft routed notice in the closeout tranche (the `TM-PIP-030` shape — the tool is root-owned; Piping performs no shared-tool repair). Priority LOW: the blind spots are declared, not silent, and this generation's manual completion found zero live markers.

### HC-PIP-20260803-002 — Cross-consumer comparison-basis identity is an unrouted coordination question

- SourceRef: `execution/_Coordination/COORDINATION_RESPONSE_2026-08-02_PIPING_RUNTIME_SURFACE_NEEDS.md` §6
- SourceSha: `a71145ec0952cc5ad62b1b12635be44deebffbd3`
- Concern: the runtime-surface response self-labels the cross-consumer identity need (basis ID/version, canonical basis hash, subject IDs/hashes, sandbox/policy identity, outcome vocabulary, from E-11/E-13/E-18/E-19) "a coordination candidate" — but no register row, decision instrument, or owning workflow carries it, and it dissolves silently if neither Root nor Piping picks it up.
- Domain lenses: `Decisions; Planning; Deliverables`
- Proposed disposition: `RESOLVED_BY_DECISION` — eventually, by whichever instrument rules (a Root runtime ruling on `TM-ROOT-105`/`TM-ROOT-109`, or a Piping product-basis act).
- Recommendation: **PROMOTE as DEFERRED**, trigger: Root rules on its runtime rows `TM-ROOT-105`/`TM-ROOT-109`, or an owner-ruled Piping product-basis act addresses generic-runtime identity; cross-relate to `TM-PIP-025`.

### HC-PIP-20260803-003 — Preview fixture-fallback lacks a fail-closed distinction for any future agent route

- SourceRef: `execution/_Coordination/COORDINATION_RESPONSE_2026-08-02_PIPING_RUNTIME_SURFACE_NEEDS.md` §7.7 (E-06)
- SourceSha: `a71145ec0952cc5ad62b1b12635be44deebffbd3`
- Concern: the browser/test preview helper's fixture fallback can substitute a fixture when native invocation fails; the response records that this "is not an acceptable evidence basis for an authoritative agent-run success and would need an explicit fail-closed distinction in any future agent route" — a guard condition carried on no deliverable `_STATUS.md` Remaining and no register row.
- Domain lenses: `Checking; Deliverables; Work`
- Proposed disposition: `RESOLVED_WITH_CHANGE` — eventually, by the fail-closed distinction landing with whatever deliverable first proposes an agent route consuming preview results.
- Recommendation: **PROMOTE as DEFERRED**, trigger: any proposal or activation of an agent route consuming preview/browser-run results.

### HC-PIP-20260803-004 — DEL-09-04 validation-manual derivative regeneration deferral is unrouted

- SourceRef: `execution/PKG-05_Loads, Load Cases, and Stress Recovery/1_Working/DEL-05-02_Load-case algebra engine/_run_records/WORKING_ITEMS_RUN_2026-08-02_DEC092_TEMPERATURE_G_IMPLEMENTATION.md` line 672 ("Validation-manual regeneration: `DEFERRED` to DEL-09-04; no foreign write"); `loop/LOOP_RECEIPTS.md` Receipt 87 Gate-Outcome line 2110
- SourceSha: run record `b9d9fc9e59a8c23ac8bee131892c565304853715`; receipts `3c4f00294866c035244d9d4f455e63eb1d425d41`; corroborating DEL-05-02 `_STATUS.md` `393af31e9d16b31f898804bacfe429f85b19a07a`, candidate brief `e161e80da08521befd41bf6fd1097671d386e701`
- Concern: the DEC-092 closeout deferred validation-manual derivative regeneration to DEL-09-04 and the candidate brief promised a "later routed follow-up" — but the deferral exists only on the producing deliverable's surfaces (DEL-05-02 `_STATUS.md`/`MEMORY.md`) and in Receipt 87; DEL-09-04's own `_STATUS.md` (last updated 2026-07-20, blob `e2eb2d4`) contains no DEC-092/temperature-G item, and no register row carries it. The receiving deliverable has no breadcrumb; the routed follow-up was never routed.
- Domain lenses: `Action Item; Deliverables; Assignment; Planning; Checking`
- Proposed disposition: `RESOLVED_WITH_CHANGE` — by a routed deliverable amendment (resolution path 1) placing the regeneration item on DEL-09-04's own surface through its owning production lane.
- Recommendation: **PROMOTE as OPEN**. Strongest candidate of this harvest: a governance breadcrumb gap, exactly what the register exists to preserve.

### HC-PIP-20260803-005 — Unremediated dependency advisories recorded in N2 provisioning

- SourceRef: same WORKING_ITEMS run record, lines 539–542 ("npm reported two low and three high transient dependency advisories; external `buf_redux` / `multipart` packages reported future-incompatibility warnings. No source, lock, or remediation action was taken.")
- SourceSha: `b9d9fc9e59a8c23ac8bee131892c565304853715`
- Concern: three high-severity npm advisories and two Rust future-incompatibility warnings were observed and explicitly left unactioned, with no owning instrument named. This is outside the owner's Receipt-88 declination, which covered only *resolved* execution telemetry.
- Domain lenses: `Checking; Work; Deliverables; Prioritization`
- Proposed disposition: `RESOLVED_WITH_CHANGE` — by a bounded dependency-refresh/triage task in the owning deliverable lanes (advisories may already be transient; the bounded work is to re-observe and either remediate or record a reasoned acceptance).
- Recommendation: **PROMOTE as OPEN**, priority LOW.

### HC-PIP-20260803-006 — Receipt 87 claims hosted PR-check outcomes the pointed records do not carry

- SourceRef: `loop/LOOP_RECEIPTS.md` Receipt 87 Checks line 2108 ("…and configured PR checks are recorded with actual outcomes in the pointed run/evidence records"); against the same WORKING_ITEMS run record lines 591–592 ("Hosted PR checks are absent at this pre-commit stage and are not claimed passed") and §9 (N8), which records commit, sweeps, and sealed-environment evidence but no hosted PR-check outcome.
- SourceSha: receipts `3c4f00294866c035244d9d4f455e63eb1d425d41`; run record `b9d9fc9e59a8c23ac8bee131892c565304853715`
- Concern: for the single check class "configured PR checks", the receipt's "recorded with actual outcomes in the pointed run/evidence records" claim is unsupported by those records — the sealed brief itself scheduled hosted PR checks as later N8 work (line 396), and N8 recorded none. GitHub-side PR checks may well have run and passed at merge; the defect is the receipt's evidence claim, not (necessarily) the checks.
- Domain lenses: `Checking; Approval; Decisions`
- Proposed disposition: `RESOLVED_WITH_CHANGE` — bounded verification of the actual PR-check outcome for the DEC-092 merge, then a receipt-conformant corrective record through the loop's ordinary receipt mechanism; or `INFORMATIONAL_NO_ACTION` if the owner rules the receipt's pointer language tolerable as-is.
- Recommendation: **PROMOTE as OPEN** for bounded verification; the owner may instead decline with `INFORMATIONAL_NO_ACTION` — presented honestly with both options since the Receipt-88 declination covered the *corrected* Receipt 87 prose, and this is a distinct, unresolved evidence-claim defect.

### HC-PIP-20260803-007 — DEC-046 public-benchmark tolerance promotion is a held-open owner gate with no representation

- SourceRef: `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260719-DEL0904-VALMANUAL-REFRESH-R13/HANDOFF_STATE.md` ("Tolerance promotion stays owner-gated (DEC-046)"); `execution/PKG-09_Verification, Validation, and Quality Oracles/1_Working/DEL-09-04_Validation manual skeleton/_STATUS.md` line 8 ("Promote final public-benchmark release tolerances under the DEC-046 convention (gated: owner threshold promotion)")
- SourceSha: handoff `cd95dc4b881078b5e170284489168155750f5839`; status `e2eb2d4bab75d5b8e2353a3d535b1590ee7f0b5a`
- Concern: a held-open owner decision — final public-benchmark release-tolerance promotion under DEC-046 — has been gated since at least 2026-07-19 with no trigger, no named forum, and no register representation; the related R14 clean-checkout reproduction acceptance is likewise an owner gate that has never fired (bundle `INTERNALLY_VERIFIED` 2026-07-20, no owner acceptance).
- Domain lenses: `Decisions; Approval; Prioritization`
- Proposed disposition: `RESOLVED_BY_DECISION` — eventually, by the owner threshold-promotion ruling (and, if folded in, the reproduction-acceptance ruling).
- Recommendation: **PROMOTE as DEFERRED**, trigger: an owner threshold-promotion ruling under the DEC-046 convention. Owner option: fold the R14 reproduction-acceptance gate into the same row as one "DEL-09-04 owner-gated promotions" item, or leave the latter unpromoted as ordinary deliverable residue.

## 4. Harvested signals not recommended for new rows

| Signal | SourceRef + SourceSha | Reason |
| --- | --- | --- |
| `NOTICE_2026-08-02_ROOT_TM_RUNTIME_NEEDS_RESPONSE_REQUEST.md` (`7801a274ce1cca2e3eefeecbdd2ddfb84826936a`) | inbound ask discharged | The requested inventory response exists (`COORDINATION_RESPONSE_2026-08-02_…`, reciprocal citations to `TM-ROOT-105`/`TM-ROOT-109`); residual action is Root-side. `INFORMATIONAL_NO_ACTION`. |
| `NOTICE_2026-08-02_TASK_MANAGEMENT_CLOSEOUT_RECEIPT_RULE.md` (`8a7df37efef09e8a96ec89867c903dce2186645e`) | no follow-on | Rule already practiced (Receipt 88); this session complies at closeout. `INFORMATIONAL_NO_ACTION`. |
| `NOTICE_2026-08-02_TASK_MANAGEMENT_DEFERRAL_REVIEW_MODE.md` (`61ebb4a7d093772c339af1706110150a7f5956d5`) | no follow-on | Permissive mode adoption; launcher rename applied in-tranche. `INFORMATIONAL_NO_ACTION`. |
| `NOTICE_2026-08-03_ROOT_SCA003_PRD_BASIS_RECONCILIATION.md` (`daf3b87514e5559c1d8bf2f9cd3f8c2add68fb14`) | no follow-on | "requires no Piping action by itself"; verified no Piping pin/mirror carries the superseded `PRD_ROOT` bytes (tree-wide grep: sole hit is the notice itself). `INFORMATIONAL_NO_ACTION`. |
| Runtime-surface response §7 items 1–6 (`a71145ec…`) | deliverable-owned residuals | Each is recorded on its owning deliverable `_STATUS.md`; §7 item 1 (absent agent harness/runtime) is adjacent to `TM-PIP-025` — no duplicate minted. |
| DEC-092 telemetry: Python-floor, Cargo-lock, origin-guard, count-detector traces | run record `b9d9fc9e…` | `DUPLICATE` of `TM-PIP-027/029/028/030` respectively. |
| Manager telemetry containment incident; harness maintenance-lock race; browser provisioning; origin-drift incidents; failed push relay | run record `b9d9fc9e…` | Resolved execution telemetry — the owner's Receipt-88 declination class; not re-harvested. |
| Contract-corpus "README status remains pending review" (run record line 537) | `b9d9fc9e…` | Too thin for a durable row; ordinary deliverable review residue. Disclosed only. |
| R13 handoff blockers other than the DEC-046 gate | `cd95dc4b…` | MAINTAINER_REVIEWED promotion, GUI-workflow evidence, and DEL-10-05 `export-results` binding are ordinary deliverable residuals on DEL-09-04's own surface (fenced planned work); D-45 gate released via `DEC-092`/Receipt 87. |
| Previously ruled populations | see §2 | Evaluation findings, TBD rows, `Review_Findings` classes, and 8 unledgered notices — byte-identical to the 2026-08-02 ruled populations (HC-20260802-003..014); rulings stand. |

## 5. Run-record marker verification (manual completion evidence)

A dispatched read-only verification covered all 11 files containing
`NEEDS_HUMAN_RULING`/`MISSING` content (8 colon-form, 3 heading-form
recovered by widened regex): **zero live, zero live-unverified markers**.
The prior harvest's judgment on DEL-11-03 is confirmed and conservative —
two of its three `MISSING:` bullets are independently closed
(`RF-11-03-C-001/002`, `ACCEPT_AS_IS`/`RESOLVED`; both referenced files
exist), the third survives as `RF-11-03-C-003` (`DEFERRED`). The two
substantive heading-form PKG-04 fan-in asks (human disposition of six
`TECHNICALLY_ADDRESSED_PENDING_HUMAN` review rows, `HumanDisposition=TBD`
since 2026-05-16) are already represented by ruled `HC-PIP-20260802-012`
(route through REVIEW; not promoted). Zero `TM-CANDIDATE:` markers exist
anywhere in the Piping tree (the only token hits are instruction text).
The regex gap itself is folded into HC-PIP-20260803-001.

## 6. Requested owner ruling

For each of HC-PIP-20260803-001..007: **PROMOTE** (with the recommended
Status/Trigger/elevation above, or as amended) or **DECLINE** (with the
proposed disposition recorded here as the harvest residue). Rows are written
only on your ruling; the HC-001 Root draft notice, if promoted, is prepared
only in the closeout tranche under your gate.

No register row, foreign notice, product artifact, lifecycle/status surface,
dependency, DAG, pointer, decision record, or scope instrument was modified
by this harvest.
