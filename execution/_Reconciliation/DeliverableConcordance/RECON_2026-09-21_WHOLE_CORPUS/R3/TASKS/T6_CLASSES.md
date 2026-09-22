# R3 T6 — implementation gaps (PARTIAL_SLICE, NOT_STARTED, DEFERRED_BY_RULING, OWNERSHIP_ELSEWHERE)

T6 classifies the 521 divergent rows of `R3/CORPUS_CLAIMS.csv` whose effective
CauseTag is PARTIAL_SLICE (419), NOT_STARTED (40), DEFERRED_BY_RULING (36) or
OWNERSHIP_ELSEWHERE (26). Almost every row is on an IN_PROGRESS deliverable
(519 of 521; two are on the ISSUED DEL-01-01) and describes work that has not
landed, not work that landed wrongly. The partition therefore turns on who
must act before the gap can close. Ten classes result. Three are ordinary
implementation remainders routed as code-fix brief candidates: deliverable-local
(C01, 180), bound to an accepted baseline (C02, 116), and on a protected subject
(C03, 34). One class is blocked on a held owner selection (C04, 87). One awaits
human review dispositions (C05, 35), and one awaits an engineering convention or
qualification (C06, 4). One is an unresolved placement of behaviour that landed in
another deliverable's area (C07, 17). The remaining three need no action: gaps
held by a named ruling (C08, 36), the SCA-009 re-point of DEL-07-03 (C09, 9,
kept visible because six of its rows are CONTESTED), and the DEL-11-03 conditional
example rules met only by absence (C10, 3, kept visible because a FIRM
resolution names an unresolved reading). Ten classes exceed the "about 3–8"
guide because the plan says exceptions stay visible as their own classes. C09 and
C10 are those exceptions, and C06 has a route of its own. T6 classifies and
proposes routing only. It re-disposes no row and makes no engineering-adequacy
claim.

## Classification rule

The rule is applied in order to effective values. The first match wins.
`AuthorityNeeded` and `AuthorityTier` corrections carried in `OtherCorrections`
(FIRM or FIELD) are taken into account (brief, "Other corrections"). Five rows
move because of this: DEL-12-01:SOW#CLM-024.r06, DEL-12-02:SOW#CLM-027 and
DEL-12-04:SOW#CLM-024 (FIRM: AuthorityNeeded NO to OWNER, so C04), and
DEL-12-04:SOW#CLM-011.r05 and DEL-12-04:SOW#CLM-011.r07 (FIELD: AuthorityTier
PROJECT_BASELINE to INVARIANT; C03 and C04 respectively).

1. CauseTag DEFERRED_BY_RULING goes to **C08**.
2. CauseTag OWNERSHIP_ELSEWHERE goes to **C09** if the Disposition is ACCEPTED_DIVERGENCE, otherwise to **C07**.
3. The three DEL-11-03 conditional-example rows go to **C10** (named key list; see C10).
4. AuthorityNeeded OWNER goes to **C04**. REVIEW goes to **C05**. ENGINEERING goes to **C06**.
5. The rest have AuthorityNeeded NO. Tier INVARIANT goes to **C03**, PROJECT_BASELINE to **C02** and LOCAL_DESIGN to **C01**.

## Classes

| ID | Name | Rows | Pkgs | Dels | Authority | Route |
|---|---|---:|---:|---:|---|---|
| T6-C01 | Deliverable-local implementation remainder | 180 | 15 | 50 | NONE | CODE_FIX_CANDIDATE |
| T6-C02 | Baseline-bound implementation remainder | 116 | 13 | 33 | NONE | CODE_FIX_CANDIDATE |
| T6-C03 | Protected-subject implementation remainder | 34 | 10 | 18 | REVIEW | CODE_FIX_CANDIDATE |
| T6-C04 | Gap blocked on a held owner selection | 87 | 12 | 27 | OWNER | OWNER_DECISION |
| T6-C05 | Human review disposition pending | 35 | 10 | 23 | REVIEW | REVIEW |
| T6-C06 | Engineering convention or qualification pending | 4 | 4 | 4 | ENGINEERING | ENGINEERING_AUTHORITY |
| T6-C07 | Behaviour landed elsewhere, placement unresolved | 17 | 9 | 11 | SCOPE_CHANGE | SCOPE_CHANGE_HANDOFF |
| T6-C08 | Gap held by a named ruling | 36 | 9 | 11 | OWNER | NO_ACTION |
| T6-C09 | Sanctioned SCA-009 re-point (exception: CONTESTED) | 9 | 1 | 1 | OWNER | NO_ACTION |
| T6-C10 | Conditional example rules met by absence (exception) | 3 | 1 | 1 | NONE | NO_ACTION |

Total: 521 rows.

### T6-C01 — Deliverable-local implementation remainder (180)

**Description.** These are bounded slices on IN_PROGRESS deliverables where the
rest of a deliverable-local design claim has not landed. The missing parts are
schema fields, tests, fixtures, exporters and record fields that no ruling or
hold blocks. Typical examples are exporter profiles lacking the common field
list, comparison CSV/JSON exporters that do not exist, and absent negative
tests. Each row's RemainingWork states the missing element, usually as
"implement X, or narrow the requirement". Because the tier is LOCAL_DESIGN,
narrowing is itself a deliverable-local record repair and needs no decision.

- **Signature.** PARTIAL_SLICE · LOCAL_DESIGN · PARTIALLY_IMPLEMENTED (169); NOT_STARTED · LOCAL_DESIGN · DOCUMENTED_UNIMPLEMENTED (11). Layers are RECORD 166, VALIDATION 8 and SECURITY 6. AuthorityNeeded is NO on every row.
- **Population.** 180 rows. Five are STATUS#remaining rows.
  - **Packages (15):** PKG-01, PKG-02, PKG-03, PKG-04, PKG-06, PKG-07, PKG-08, PKG-09, PKG-10, PKG-11, PKG-12, PKG-13, PKG-14, PKG-15 and PKG-17.
  - **Deliverables (50):** DEL-01-01 (2), DEL-01-03 (1), DEL-02-01 (1), DEL-02-02 (2), DEL-02-03 (1), DEL-02-04 (2), DEL-02-05 (6), DEL-03-05 (1), DEL-04-01 (1), DEL-04-05 (1), DEL-06-02 (1), DEL-06-04 (6), DEL-07-01 (1), DEL-07-02 (2), DEL-07-03 (1), DEL-07-04 (2), DEL-08-01 (1), DEL-08-02 (3), DEL-08-03 (4), DEL-08-04 (5), DEL-08-05 (3), DEL-08-06 (1), DEL-09-01 (7), DEL-09-03 (1), DEL-09-05 (2), DEL-10-01 (3), DEL-10-02 (7), DEL-10-03 (2), DEL-11-01 (3), DEL-11-03 (3), DEL-11-04 (1), DEL-11-05 (1), DEL-12-01 (1), DEL-12-02 (2), DEL-12-03 (4), DEL-12-04 (6), DEL-13-03 (1), DEL-14-03 (1), DEL-14-04 (4), DEL-14-05 (12), DEL-15-02 (1), DEL-15-04 (4), DEL-17-02 (11), DEL-17-03 (1), DEL-17-04 (4), DEL-17-05 (18), DEL-17-06 (3), DEL-17-07 (11), DEL-17-08 (10) and DEL-17-09 (8).
  - The full key list is in `T6_CLASSES.csv`.
- **Owning authority.** NONE. These are deliverable-local choices, and no decision is reserved.
- **Recommended routing.** CODE_FIX_CANDIDATE, as one brief per deliverable carrying its rows' RemainingWork.
- **On-ruling mechanism.** An R4 ruling accepting the class authorises Agent 0 to issue per-deliverable candidate briefs under `R3/CODE_FIX_BRIEF_CANDIDATES/`. The briefs are not executed. Execution is later owner-steered work-graph selection under the ordinary chirality-change path, followed by R5 ledger catch-up. Where a brief chooses the "narrow the requirement" branch that RemainingWork offers, the change is a deliverable Scope of Work edit under R5_RECORD_REPAIR. At LOCAL_DESIGN tier it needs no owner decision.
- **Risk if unrepaired.** Deliverable claims keep overstating delivered scope. Downstream consumers, such as exporters that read the common profile field list, rely on fields that do not exist. The gap is record-level (layers mostly RECORD) and touches no protected subject.
- **Representative keys.**
  - `DEL-17-02:SOW#CLM-009`, in `WAVES/W3/PKG-17/DEL-17-02/DEL-17-02_forward.csv`. Exporter profiles lack `entity_coverage`. `coordinate_policy` is absent from MBF and stress-neutral, and `stable_id_policy` is named `identity_policy` in three profiles. RemainingWork: align the profiles or narrow the claim.
  - `DEL-14-05:SOW#CLM-013.r05`, in `WAVES/W3/PKG-14/DEL-14-05/DEL-14-05_forward.csv`. No exporter emits comparison values. Only `schemas/comparison_mapping.schema.json` and `schemas/comparison_tolerance.schema.json` exist, so the comparison_review_csv_v1/json_v1 exporters are absent.
  - `DEL-06-04:SOW#CLM-010/R-06-04-005`, in `WAVES/W3/PKG-06/DEL-06-04/DEL-06-04_forward.csv`. Only the `non_json_asset_manifest` payload-scope value exists. Nothing produces or checks a non-JSON manifest hash.
- **Class notes.**
  - **Exceptions.** `DEL-12-04:SOW#CLM-027` is OBSERVED: its Notes wrongly say that no product private-library or rule-pack store exists, and `lib.rs::save_local_library` and `save_local_rule_pack` do exist. `DEL-15-02:SOW#CLM-013/V-004` is FIELD: its layers change from RECORD to VALIDATION. No CONTESTED rows.
  - **CP-11 (7 rows).**
    - Three are the DEL-17-04 pass-through-option rows (`SOW#CLM-005.r10`, `SOW#CLM-009/DEL-17-04-REQ-006` and `CONTEXT#context-envelope`), all NOT_STARTED. Their RemainingWork is "implement as profile metadata, or narrow".
    - Four are RESOLVED_PAIR rows: `DEL-17-02:SOW#CLM-019/DEL-17-02-REQ-014`, `DEL-17-02:SOW#CLM-023/DEL-17-02-REQ-053`, `DEL-17-07:SOW#CLM-016/DEL-17-07-REQ-034` and `DEL-17-09:SOW#CLM-013/DEL-17-09-REQ-010`.
    - All seven stay in C01 because they carry an implement-or-narrow action. They differ from C10, where nothing requires the governed content to exist.
  - **Other resolution classes.** The six FIRM rows (for example `DEL-11-03:SOW#CLM-004.r01` and the DEL-02-05 FG-DEL-02-05-06 rows) and the two WEAK rows are named in `OtherCorrections`. None of them changes the class.
  - **PRODUCT_CALLER: NONE (3 rows).** `DEL-13-03:SOW#CLM-005.r04` (WEAK), `DEL-14-03:SOW#CLM-012/REQ-14-03-001` and `DEL-15-04:SOW#purpose-and-objective-traceability/OUT-001` (WEAK).
  - **Record or document work (5 rows).** These rows need record or document work, not code: `DEL-17-06:SOW#CLM-015/DEL-17-06-VER-006` and `DEL-17-08:SOW#CLM-020/A-001` (requirement-to-evidence matrices), `DEL-17-06:STATUS#remaining/R02` (semantic and dependency regeneration workflows), `DEL-11-05:SOW#CLM-011/REQ-11-05-07` (guide text, or narrowing) and `DEL-17-09:STATUS#remaining/R01` (a loader-binding assignment, with candidate owners DEL-10-01 and DEL-10-02). The brief carries them as record items.

### T6-C02 — Baseline-bound implementation remainder (116)

**Description.** These are bounded slices where the unmet part restates a ruled
decision, accepted scope or baseline (tier PROJECT_BASELINE), and no owner hold
blocks it. Examples:
- rule-pack identity and checksums not bound into product report and audit manifests (PKG-08);
- no constraint-validation stage on the runtime operation route (PKG-16);
- model-state hashing labelled as not implementing the DEC-010 JCS basis (DEL-14-01);
- handoff-package export not wired to a product path (DEL-15-03);
- incomplete producer diagnostic breadth (DEL-14-02);
- architecture-basis job control and transaction boundaries (DEL-00-03).

Completing the slice is ordinary implementation. But the "or narrow the
requirement or obtain a ruling" branch many rows offer is not deliverable-local,
because narrowing a baseline needs owner or scope-change authority.

- **Signature.** PARTIAL_SLICE · PROJECT_BASELINE · PARTIALLY_IMPLEMENTED (115); NOT_STARTED · PROJECT_BASELINE · DOCUMENTED_UNIMPLEMENTED (1: `DEL-08-01:STATUS#remaining/R01`). Layers are BASELINE 50, RECORD 28, VALIDATION 22, BASELINE;RECORD 13 and SECURITY 3. AuthorityNeeded is NO.
- **Population.** 116 rows. Three are STATUS#remaining rows.
  - **Packages (13):** PKG-00, PKG-02, PKG-03, PKG-04, PKG-05, PKG-06, PKG-08, PKG-09, PKG-12, PKG-13, PKG-14, PKG-15 and PKG-16.
  - **Deliverables (33):** DEL-00-02 (4), DEL-00-03 (6), DEL-00-04 (3), DEL-00-06 (1), DEL-02-02 (1), DEL-02-03 (1), DEL-02-05 (7), DEL-03-01 (2), DEL-03-02 (1), DEL-04-01 (2), DEL-04-05 (1), DEL-05-02 (3), DEL-05-03 (2), DEL-06-02 (2), DEL-06-03 (1), DEL-06-04 (2), DEL-08-01 (9), DEL-08-02 (10), DEL-08-03 (7), DEL-08-05 (1), DEL-09-01 (3), DEL-09-02 (3), DEL-12-03 (1), DEL-12-04 (3), DEL-13-03 (1), DEL-14-01 (7), DEL-14-02 (11), DEL-14-03 (3), DEL-15-03 (9), DEL-15-04 (1), DEL-16-02 (2), DEL-16-03 (5) and DEL-16-04 (1).
- **Owning authority.** NONE for completing the slice within the accepted baseline. Any narrowing branch is OWNER or SCOPE_CHANGE, and the brief must not take it on its own.
- **Recommended routing.** CODE_FIX_CANDIDATE.
- **On-ruling mechanism.** An R4 ruling accepting the class authorises candidate briefs under `R3/CODE_FIX_BRIEF_CANDIDATES/`, which are not executed. Each brief states the governing baseline item it completes (for example DEC-010, SOW-069 or the architecture-basis REQ). Where a row's RemainingWork offers "or obtain a ruling that narrows", the brief lists that alternative as an owner item for a later decision packet and does not execute it.
- **Risk if unrepaired.** The accepted baseline stays unmet in product behaviour. Examples: report packages refuse rule-pack binding (`REPORT-PACKAGE-RULE-PACK-BINDING-UNAVAILABLE`), and runtime edits skip constraint validation. Records that cite the baseline read as satisfied when they are not. 22 rows touch validation and provenance (VALIDATION layer).
- **Representative keys.**
  - `DEL-08-02:SOW#CLM-004.r04`, in `WAVES/W3/PKG-08/DEL-08-02/DEL-08-02_forward.csv`. The engine records rule-pack identity and checksum, but the product request sends empty `rule_pack_refs` (freeze `projects/chirality-piping/apps/desktop/src/features/report/reportPackageRequest.ts:286`, `:318`). The row also carries F7.
  - `DEL-16-02:SOW#CLM-010/REQ-16-02-003`, in `WAVES/W1/PKG-16/DEL-16-02/DEL-16-02_forward.csv`. Runtime intents carry `"constraint_validation": "not_run"` (freeze `projects/chirality-piping/apps/desktop/src-tauri/src/lib.rs:4213`), and the applier has no constraint stage. SOW-069 names constraint validation before controlled application.
  - `DEL-14-01:SOW#CLM-012`, in `WAVES/W3/PKG-14/DEL-14-01/DEL-14-01_forward.csv`. The persistence service labels its canonicalization `SORTED_COMPACT_JSON` (freeze `projects/chirality-piping/core/project_persistence/service.py:28`), which is explicitly not RFC 8785/JCS, against the DEC-010 JCS-compatible basis. RemainingWork: implement JCS, or obtain a narrowing ruling (an owner alternative).
- **Class notes.**
  - **CONTESTED rows (9), kept visible.**
    - `DEL-02-05:SOW#CLM-005.r05`, `DEL-02-05:SOW#CLM-014/REQ-02-05-005`, `DEL-02-05:CONTEXT#context-envelope` and `DEL-02-05:CONTEXT#sca-003-storage-profile-injection`. This is the FROZEN_CONTRACT versus NONE BaselineClass split shared with DEL-02-02 FG-DEL-02-02-01, and it needs one resolution.
    - `DEL-03-01:SOW#CLM-011/REQ-03-01-007` and `DEL-03-01:SOW#production-and-verification-method-praxeology/VER-001`. The Notes overstate the absence of material round-trip evidence, and DEL-03-05 CLM-010.r08 judges the same evidence class the other way.
    - `DEL-14-01:SOW#CLM-005`, `DEL-14-03:SOW#CLM-006` and `DEL-14-03:CONTEXT#architecture-basis-injection.s03`. These belong to the DEC-009 cluster, which Agent 0 settles corpus-wide (T8).
  - **PRODUCT_CALLER: NONE (4 rows).** `DEL-14-01:SOW#CLM-004`, `SOW#CLM-005`, `SOW#CLM-011.r01` and `SOW#CLM-011.r04`.
  - **CP-11.** One row: `DEL-14-02:SOW#CLM-011.r09` (RESOLVED_PAIR).
  - **Owner alternative.** `DEL-00-06:AB#normative-requirements/REQ-06-03` (FIRM) is AuthorityNeeded NO, becoming OWNER if the storage commands are to be exempted from the diagnostics envelope. The brief should surface that alternative.

### T6-C03 — Protected-subject implementation remainder (34)

**Description.** These are bounded slices whose unmet part lies on a protected
subject: the IP and data boundary, security and privacy, the claims boundary,
unit validation or no-bypass. The tier is INVARIANT, and no owner or review
hold is recorded. Examples:
- protected-content detection covers only synthetic markers (DEL-08-05);
- the evaluator's public-example and bypass-attempt test families are absent (DEL-06-02);
- unit handling is presence-only, with no unit-against-dimension check (DEL-15-02, DEL-03-08);
- redistribution status is not carried into export decisions (DEL-12-02, DEL-03-08);
- external-reference ingestion is not bound to privacy screening (DEL-14-01).

The remedy is implementation or tests, but it lands on invariants, so a repair
needs review before reliance.

- **Signature.** PARTIAL_SLICE · INVARIANT · PARTIALLY_IMPLEMENTED (34). Layers are IP_DATA 11, SECURITY 7, VALIDATION 4, IP_DATA;SECURITY 4, IP_DATA;CLAIMS 3, BASELINE 3, CLAIMS;BASELINE 1 and CLAIMS 1. AuthorityNeeded is NO.
- **Population.** 34 rows.
  - **Packages (10):** PKG-02, PKG-03, PKG-04, PKG-06, PKG-08, PKG-11, PKG-12, PKG-14, PKG-15 and PKG-16.
  - **Deliverables (18):** DEL-02-03 (1), DEL-02-04 (2), DEL-03-04 (1), DEL-03-08 (3), DEL-04-04 (1), DEL-04-05 (1), DEL-06-01 (1), DEL-06-02 (6), DEL-06-03 (1), DEL-08-04 (1), DEL-08-05 (4), DEL-11-04 (1), DEL-12-01 (1), DEL-12-02 (1), DEL-12-04 (3), DEL-14-01 (2), DEL-15-02 (3) and DEL-16-04 (1).
- **Owning authority.** REVIEW. The invariant subjects require an independent review of any repair under the review workflow. The fix itself needs no owner decision.
- **Recommended routing.** CODE_FIX_CANDIDATE.
- **On-ruling mechanism.** An R4 ruling authorises candidate briefs under `R3/CODE_FIX_BRIEF_CANDIDATES/`, which are not executed. They are flagged protected-subject, with a mandatory independent review step and focused negative tests named per row. Where RemainingWork offers "or narrow the principle" (for example `DEL-03-08:SOW#CLM-026` and `DEL-06-01:SOW#CLM-011/REQ-06-01-011`), narrowing an INVARIANT restatement is not deliverable-local and goes to the owner as a decision-packet item.
- **Risk if unrepaired.** Protected-boundary controls hold only partially. Copied prose or formulas pass the linter without a finding (DEL-08-05), unit-bearing values with an omitted value_kind pass with no diagnostic (DEL-15-02), and export decisions ignore redistribution metadata (DEL-12-02). This class carries the highest exposure among the implementation remainders.
- **Representative keys.**
  - `DEL-08-05:SOW#CLM-011/DEL-08-05-REQ-002`, in `WAVES/W3/PKG-08/DEL-08-05/DEL-08-05_forward.csv`. Code text, figures, formulas and proprietary data are flagged only when a planted synthetic marker is present (freeze `projects/chirality-piping/core/reporting/protected_content_linter/src/lib.rs:4`, `:294`, `:366`).
  - `DEL-15-02:SOW#CLM-005.r01`, in `WAVES/W3/PKG-15/DEL-15-02/DEL-15-02_forward.csv`. Unit handling is presence-only (freeze `core/handoff/target_mapping/contract.py` L44 and L274–286, and `schemas/target_mapping.schema.json` L294–318, as cited in the ledger). This row is also PRODUCT_CALLER: NONE.
  - `DEL-06-02:SOW#CLM-013/REQ-06-02-011`, in `WAVES/W3/PKG-06/DEL-06-02/DEL-06-02_forward.csv`. The public-example protected-content tests and the evaluator bypass-attempt tests were not located (Remaining R01).
- **Class notes.**
  - **Exceptions.**
    - CONTESTED: `DEL-03-04:SOW#CLM-011/DEL-03-04-RQ-005` (VALIDATION versus BASELINE layer for a units subject) and `DEL-08-05:SOW#completion-and-reliance-basis-epistemology/AC-001` (an intra-PKG-08 pattern divergence on whether AC-001 judges contract text or implementation).
    - CONTESTED;FIELD: `DEL-11-04:SOW#CLM-011/R-DEL-11-04-002`. The candidate reading makes it a stale requirement overtaken by the DEC-081/DEC-107 authoring directive.
    - FIELD: `DEL-08-04:SOW#CLM-013/V-7` (add the DEC-058 scan evidence) and `DEL-12-04:SOW#CLM-011.r05` (tier corrected to INVARIANT, layer to IP_DATA).
    - RESOLVED_PAIR: the three DEL-06-02 no-bypass rows, whose tier INVARIANT is already effective.
  - **CP-11 (3 rows).** `DEL-06-02:SOW#CLM-006.r05`, `SOW#CLM-013/REQ-06-02-010` and `SOW#CLM-016/REQ-06-02-010`. No-bypass holds only because no adapter or plugin dispatch path exists, and they overlap C04 decision D2.
  - **PRODUCT_CALLER: NONE (3 rows).** The DEL-15-02 rows.

### T6-C04 — Gap blocked on a held owner selection (87)

**Description.** These are partial or unstarted claims whose remaining work
cannot proceed until the owner makes a selection the records leave open: a
PDU or OI hold, an unselected runtime or execution model, an unratified policy
or an unanswered product choice. AuthorityNeeded is OWNER, applying the FIRM
corrections. The rows fall into 15 decision groups, listed below. In each group
the options are those stated in the evidence. Most groups have the same shape:
select or rule (then implement with tests), or keep the hold and narrow or
annotate the claim.

- **Signature.** PARTIAL_SLICE · LOCAL_DESIGN (34); PARTIAL_SLICE · INVARIANT (23); PARTIAL_SLICE · PROJECT_BASELINE (21), all PARTIALLY_IMPLEMENTED; NOT_STARTED · LOCAL_DESIGN (6); NOT_STARTED · INVARIANT (3), both DOCUMENTED_UNIMPLEMENTED. Layers are RECORD 45, BASELINE 12, CLAIMS 10, IP_DATA 10, SECURITY 8 and BASELINE;VALIDATION 1. BaselineClass is OWNER_HOLD on 3 rows and RULED_CRITERION on 1.
- **Population.** 87 rows. Twelve are STATUS#remaining rows.
  - **Packages (12):** PKG-00, PKG-02, PKG-04, PKG-05, PKG-07, PKG-09, PKG-10, PKG-12, PKG-13, PKG-14, PKG-15 and PKG-17.
  - **Deliverables (27):** DEL-00-01 (2), DEL-00-02 (1), DEL-00-07 (1), DEL-02-02 (6), DEL-02-05 (2), DEL-04-01 (1), DEL-04-04 (1), DEL-05-02 (3), DEL-05-04 (9), DEL-07-05 (8), DEL-07-07 (2), DEL-07-08 (9), DEL-09-01 (2), DEL-09-04 (1), DEL-09-05 (1), DEL-10-01 (3), DEL-10-02 (6), DEL-12-01 (2), DEL-12-02 (3), DEL-12-03 (3), DEL-12-04 (9), DEL-13-03 (2), DEL-13-04 (1), DEL-14-04 (5), DEL-15-01 (1), DEL-17-08 (1) and DEL-17-09 (2).
- **Owning authority.** OWNER.
- **Recommended routing.** OWNER_DECISION, as one R4 packet per decision group. A group may combine with other tasks' packets on the same hold.
- **Decision groups.** Each entry gives the decision, the options in the evidence and the rows.
  - **D1. Human acceptance workflow (OI-007).** Options: (a) rule an acceptance workflow, then add a runtime hash-change invalidation check and a negative test; (b) leave it unruled, so hash binding stays declaration-only (`schemas/project_persistence.schema.yaml` `invalidates_on_hash_change`, freeze `projects/chirality-piping/schemas/project_persistence.schema.yaml:506`, `:527`). Rows: DEL-05-04 FG-DEL-05-04-01 (9: `SOW#CLM-004`, `SOW#CLM-008`, `SOW#CLM-011/REQ-05-04-008`, `SOW#CLM-011/REQ-05-04-014`, `SOW#CLM-013/REQ-05-04-008`, `SOW#CLM-013/REQ-05-04-014`, `SOW#completion-and-reliance-basis-epistemology/AC-001`, `SOW#CLM-020` and `SOW#CLM-023`). This links to C08 DEL-15-04 (the same unruled workflow, held there by ruling).
  - **D2. Plugin and adapter runtime execution model and grant model.** Options: (a) authorise and select a runtime execution model, then add no-bypass tests and a permission grant model with private-library and rule-pack scopes; (b) keep the runtime unselected, so the gate stays deny-only (`BLOCKED_RUNTIME_NOT_SELECTED`, freeze `projects/chirality-piping/core/adapters/framework/adapter_framework.py:532`) and the claims stay partially met. Rows:
    - `DEL-00-07:AB#normative-requirements/REQ-07-03`;
    - `DEL-12-01:SOW#CLM-010/LFSP-REQ-010` and `DEL-12-01:SOW#CLM-024.r06`;
    - `DEL-12-02:SOW#CLM-004`, `DEL-12-02:SOW#CLM-012/REXC-REQ-012` and `DEL-12-02:SOW#CLM-027`;
    - `DEL-12-04:SOW#CLM-004.r06` and `DEL-12-04:SOW#CLM-011.r07`;
    - DEL-10-02 FG-DEL-10-02-01 (6: `SOW#CLM-011/REQ-10-02-02`, `-03`, `-04`, `-05`, `-07` and `-08`).
  - **D3. Quarantine and readiness taxonomy, reviewer and signoff taxonomy (PDU-034, PDU-004).** Options: (a) select the taxonomy and destructive-workflow policy, then route helper blocks to review and add quarantine-routing tests, and align CHECKLIST_CATEGORIES; (b) keep the hold and amend REQ-007. Rows: `DEL-12-04:SOW#CLM-004.r05`, `SOW#CLM-007.r05`, `SOW#CLM-011.r06` and `SOW#CLM-024`; `DEL-17-09:SOW#CLM-013/DEL-17-09-REQ-007` and `DEL-17-09:STATUS#remaining/R03` (OWNER_HOLD).
  - **D4. Private-library storage roots and secret provider.** Options: (a) make the owning storage-root decision and select a secret provider, then wire product registration and credential-placeholder flows; (b) keep the helper-only posture. Rows: `DEL-12-04:SOW#CLM-004.r01`, `SOW#CLM-004.r02` and `SOW#CLM-011.r01`. Note the OBSERVED and FIELD correction on `CLM-004.r01` and `CLM-011.r01`: `lib.rs::save_local_library` and `save_local_rule_pack` exist, so the remaining gap is rule-pack storage and root placement, not the absence of any store.
  - **D5. Telemetry consumer routes.** Options: (a) separately authorise routing consumer telemetry attempts through the seam, with tests (TEL-TEST-006) and a product-level no-outbound test with a security review record; (b) keep only the modelled panel attempt. Rows: `DEL-12-03:SOW#CLM-011/TEL-REQ-009`, `SOW#CLM-011/TEL-REQ-010` and `SOW#CLM-013/TEL-TEST-006`.
  - **D6. Unit namespace, alias policy, diagnostic-code namespace and angle semantics (U-010, U-016).** Options: select each policy (then add parser and mapping tests), or leave it TBD. Rows: DEL-02-02 (6: `SOW#CLM-014/U-010`, `SOW#CLM-014/U-016`, `SOW#CLM-015.r02`, `SOW#CLM-015.r08`, `SOW#CLM-015.r12` and `SOW#CLM-017/U-010`). This may intersect T8's unit-vocabulary cluster.
  - **D7. Result-envelope home and comparison-result schema.** Options: (a) accept an application-service or result-envelope home for ValidationResult and TransformResult, and an authoritative analysis-run comparison-result/export schema, then add conformance tests; (b) leave the outputs unhomed. Rows: `DEL-13-03:SOW#CLM-013.r06`, `DEL-13-03:SOW#CLM-028.r06` and `DEL-13-04:SOW#CLM-014.s02`; DEL-14-04 FG-DEL-14-04-01 (5: `SOW#CLM-008.r01`, `SOW#CLM-013/R-14-04-007`, `SOW#CLM-017.s01`, `SOW#CLM-023` and `SOW#CLM-035/OQ-14-04-002`).
  - **D8. Results-viewer scope.** The decisions are rotational visualisation semantics (DEC-074 O1 / PDU-061), the supported-ratio and equipment-load path into the viewer, and graphical comparison overlays with a state/run browser. Options: define and implement each, or narrow or amend the claim. Rows: DEL-07-05 (8: FG-DEL-07-05-01 and -02, including `STATUS#remaining/R01`), DEL-07-08 FG-DEL-07-08-01 (9) and DEL-07-07 FG-DEL-07-07-01 (2: producers carrying class, remediation and provenance). Two DEL-07-05 rows are CP-11 RESOLVED_PAIR: `SOW#CLM-004.r07` and `SOW#CLM-011/REQ-07-05-005`.
  - **D9. ADR form versus decision-log form for architecture decisions after 2026-06-11.** Options: (a) record later decisions as ADRs; (b) rule that DEC rows in the decision log satisfy AB-00-01. Rows: `DEL-00-01:AB#purpose.s03`, `DEL-00-01:AB#normative-requirements/REQ-01-04` and `DEL-00-02:AB#resolved-decisions-former-tbd-and-human-ruling-q.r06`.
  - **D10. Package compatibility window (DEC-028).** Options: rule the compatibility window, then implement package open, round-trip and explicit migrate; or keep it open. Rows: `DEL-02-05:STATUS#remaining/R01` and `R02`. This relates to C02 `DEL-08-01:STATUS#remaining/R01`.
  - **D11. Rule-pack combination supply (SOW-014).** Options: (a) build the DEL-06-02 combination handoff; (b) record an owner reading that user-authored combinations satisfy it. Rows: DEL-05-02 FG-DEL-05-02-02 (3).
  - **D12. Adapter family list after PRD v0.4 dropped §19.3.** Options: register the model-creation, load-case and rule-pack-evaluation families, or explicitly de-select them. Rows: DEL-10-01 FG-DEL-10-01-01 (3).
  - **D13. Mechanics residuals.** The decisions are the owner re-disposition of assessment rows G1, G2, G4, M2 and M3 (`DEL-04-01:STATUS#remaining/R01`; options: re-dispose, or close them by evidence) and the typed vocabulary for nonlinear result rows (`DEL-04-04:STATUS#remaining/R07`, which the evidence routes to DEL-08-04 as a result-semantics contract change).
  - **D14. Release, QA and stage gates.** The decisions are:
    - the QA policy classifying benchmarks as release-gating or advisory (`DEL-09-01:SOW#CLM-032.r04`, CP-11);
    - presenting the benchmark evidence system at the owner's gate (`DEL-09-01:STATUS#remaining/R03`, DEC-054);
    - public benchmark comparison values (`DEL-09-04:STATUS#remaining/R02`);
    - release-label vocabulary (PB-TBD-003, `DEL-09-05:STATUS#remaining/R01`);
    - the Phase H R6 stage gate (`DEL-15-01:STATUS#remaining/R01`).
    Options: select, or keep the gate closed.
  - **D15. Export timestamp and generator policy (PDU-031).** Options: select a policy, or keep the fixed generator with no timestamp, as the tests enforce today. Row: `DEL-17-08:STATUS#remaining/R01` (OWNER_HOLD).
- **On-ruling mechanism.** Each owner ruling is recorded as a DEC or PDU disposition through the owner decision path (R4 packet, then ruling record). It authorises (i) R5_RECORD_REPAIR of the affected claims where the ruling narrows or annotates them, and (ii) candidate code-fix briefs where the ruling selects a model, which remain unexecuted until owner-steered work-graph selection.
- **Risk if unrepaired.** The claims stay partially met indefinitely. 26 rows are at INVARIANT tier, carrying the CLAIMS, IP_DATA, SECURITY and VALIDATION subjects: D1 (9), D2 (7), D3 (4), D4 (1), D5 (3) and D14 (2). If a hold persists silently, records may be read as more complete than the product is.
- **Representative keys.**
  - `DEL-05-04:SOW#CLM-011/REQ-05-04-008`, in `WAVES/W3/PKG-05/DEL-05-04/DEL-05-04_forward.csv`. The binding slice exists (`bound_hashes`, `invalidates_on_hash_change`), but it has no runtime behaviour or negative test pending OI-007.
  - `DEL-10-02:SOW#CLM-011/REQ-10-02-02`, in `WAVES/W3/PKG-10/DEL-10-02/DEL-10-02_forward.csv`. `gate_adapter_runtime_dispatch` never dispatches (freeze `adapter_framework.py:532`). The row is CP-11 in substance, per its Notes.
  - `DEL-12-04:SOW#CLM-011.r06`, in `WAVES/W3/PKG-12/DEL-12-04/DEL-12-04_forward.csv`. The helper blocks protected_suspected and quarantined records, but no review routing exists pending PDU-034. It is FIELD: add the `save_local_library` quarantine-refusal evidence.
- **Class notes.**
  - **Exceptions.** OBSERVED: `DEL-12-04:SOW#CLM-004.r01`. FIELD;OBSERVED: `DEL-12-04:SOW#CLM-011.r01`. FIELD: `DEL-12-04:SOW#CLM-011.r06` and `SOW#CLM-011.r07` (tier corrected to INVARIANT). FIRM: the three OWNER upgrades named in the classification rule. RESOLVED_PAIR: 5 rows. No CONTESTED rows.
  - **PRODUCT_CALLER: NONE.** `DEL-12-04:SOW#CLM-024`.

### T6-C05 — Human review disposition pending (35)

**Description.** The gap closes only through a human review act. There are two kinds.
- **Review-finding dispositions (14 NOT_STARTED STATUS#remaining rows).** `Review_Findings.csv` rows have HumanDisposition TBD, and many of them target the retired four-document kit. The records are accurate (F2); no one has acted.
- **Partial slices whose closure the ledger conditions on a formal REVIEW disposition or sufficiency act (21 rows):**
  - fixture provenance and maintainer review for the PCF and glTF exporters (DEL-17-07, DEL-17-08);
  - the PKG02-001 dimensional-basis dispositions (PDU-035) for DEL-04-04 and DEL-04-05;
  - no-bypass architecture reviews (DEL-03-04, DEL-03-05, DEL-03-06);
  - the VER-001 parity reviews (DEL-04-04, DEL-04-05, DEL-04-06).

- **Signature.** NOT_STARTED · LOCAL_DESIGN · DOCUMENTED_UNIMPLEMENTED (14); PARTIAL_SLICE · INVARIANT (13), LOCAL_DESIGN (6) and PROJECT_BASELINE (2), all PARTIALLY_IMPLEMENTED. Layers are RECORD 19, IP_DATA;RECORD 8, BASELINE 6, LIFECYCLE 2 and others.
- **Population.** 35 rows. Fifteen are STATUS#remaining rows.
  - **Packages (10):** PKG-00, PKG-03, PKG-04, PKG-09, PKG-10, PKG-11, PKG-12, PKG-14, PKG-15 and PKG-17.
  - **Deliverables (23):** DEL-00-02 (2), DEL-03-04 (1), DEL-03-05 (1), DEL-03-06 (1), DEL-04-04 (3), DEL-04-05 (3), DEL-04-06 (1), DEL-09-01 (1), DEL-09-03 (1), DEL-10-01 (1), DEL-11-04 (1), DEL-12-01 (1), DEL-12-03 (1), DEL-12-04 (1), DEL-14-03 (1), DEL-15-01 (1), DEL-15-02 (1), DEL-15-03 (1), DEL-15-04 (1), DEL-17-03 (1), DEL-17-05 (1), DEL-17-07 (8) and DEL-17-08 (1).
- **Owning authority.** REVIEW.
- **Recommended routing.** REVIEW.
- **On-ruling mechanism.** An R4 ruling authorises a batched review under the review workflow: one review packet per deliverable listing its open findings and the named sufficiency acts. Dispositions are recorded in each deliverable's `Review_Findings.csv` (HumanDisposition), which is R5 record work afterwards. Findings that target the retired four-document kit (for example DEL-12-01 RF-002, DEL-15-03 RF-001 and RF-002, and DEL-17-03 RF-002) should be read against the current `ScopeOfWork.md`, as the ledgers note. Where a review requires added fixture provenance records or tests (DEL-17-07), those are candidate briefs, not part of the review act.
- **Risk if unrepaired.** Findings marked TECHNICALLY_ADDRESSED_PENDING_HUMAN stay unclosed, so readiness cannot be relied on. PCF fixture dimensional values go unreviewed on the IP_DATA layer (DEL-17-07 REQ-041). Stale findings against retired documents accumulate.
- **Representative keys.**
  - `DEL-12-01:STATUS#remaining/R02`, in `WAVES/W3/PKG-12/DEL-12-01/DEL-12-01_forward.csv`. At the freeze, RF-001 and RF-002 in the deliverable's `Review_Findings.csv` (lines 2–3) have HumanDisposition TBD and status OPEN, and RF-002 targets the removed kit.
  - `DEL-17-07:SOW#CLM-031`, in `WAVES/W3/PKG-17/DEL-17-07/DEL-17-07_forward.csv`. One positive invented fixture exists and no provenance record to the template, and the fixture values await review.
  - `DEL-04-05:SOW#CLM-012/DEL-04-05-RQ-006`, in `WAVES/W2/PKG-04/DEL-04-05/DEL-04-05_forward.csv`. The unit basis is a reproducibility reference only, and Review_Findings PKG04-DEL0405-PKG02-001 is TECHNICALLY_ADDRESSED_PENDING_HUMAN.
- **Class notes.**
  - **Exceptions, kept visible.** OBSERVED: `DEL-04-04:SOW#production-and-verification-method-praxeology/VER-001` and `DEL-04-05:SOW#production-and-verification-method-praxeology/VER-001` raise the CP-09 question: a PASS parity record matches the frozen SOW, so under CP-09 the rows would be ALIGNED. CONTESTED: `DEL-04-06:SOW#production-and-verification-method-praxeology/VER-001`. Both readings are defensible: sealed PARTIAL_SLICE with REVIEW, or the CP-09 reading STALE_REVIEW_OR_EVIDENCE · EVIDENCE_OVERTAKEN with AuthorityNeeded NO. If R4 adopts the CP-09 reading, these three rows leave this class for the T5B partition.
  - **Lifecycle.** `DEL-12-01:STATUS#remaining/R02` and `DEL-12-03:STATUS#remaining/R03` carry the LIFECYCLE layer, which is relevant to T9.

### T6-C06 — Engineering convention or qualification pending (4)

**Description.** The gap needs an engineering decision or a validation basis
before any implementation or record change can close it:
- the COG coordinate convention and reference frame (`DEL-03-05:SOW#CLM-010.r10`);
- retained-spring mixed-friction numerical adequacy against a vetted basis (`DEL-04-04:STATUS#remaining/R03`);
- independent validation of field-level trace completeness (`DEL-13-04:STATUS#remaining/R03`);
- the disposition of two withheld diagnostic-work unit witnesses in the accepted 830-row fixture (`DEL-17-06:STATUS#remaining/R01`).

- **Signature.** NOT_STARTED · LOCAL_DESIGN · DOCUMENTED_UNIMPLEMENTED (2); PARTIAL_SLICE · INVARIANT · PARTIALLY_IMPLEMENTED (2, layer VALIDATION).
- **Population.** 4 rows, three of them STATUS#remaining.
  - **Packages:** PKG-03, PKG-04, PKG-13 and PKG-17.
  - **Deliverables:** DEL-03-05, DEL-04-04, DEL-13-04 and DEL-17-06 (1 each).
- **Owning authority.** ENGINEERING.
- **Recommended routing.** ENGINEERING_AUTHORITY.
- **On-ruling mechanism.** An R4 ruling authorises Agent 0 to place the four items under `R3/ENGINEERING_AUTHORITY/`, one item each, with the evidence the ledgers cite. An engineering authority records the convention, qualification basis or witness disposition. Only after that do code-fix candidates (for example consuming COG in mechanics) or R5 record repairs follow.
- **Risk if unrepaired.** COG values captured with units but no frame could later be consumed inconsistently; the applier keeps COG edits unsupported today. The contact-model adequacy and trace-completeness gaps sit on the validation layer. This task makes no claim about engineering adequacy; it records only that the qualification basis is absent from the records.
- **Representative keys.**
  - `DEL-03-05:SOW#CLM-010.r10`, in `WAVES/W2/PKG-03/DEL-03-05/DEL-03-05_forward.csv`. No convention or frame is defined, the D-41 R5 T2B backcheck recorded it as held, and the Notes state that an engineering convention choice is needed.
  - `DEL-04-04:STATUS#remaining/R03`, in `WAVES/W2/PKG-04/DEL-04-04/DEL-04-04_forward.csv`. No retained-spring mixed-friction adequacy evidence was found (INVARIANT / VALIDATION).
  - `DEL-17-06:STATUS#remaining/R01`, in `WAVES/W3/PKG-17/DEL-17-06/DEL-17-06_forward.csv`. The test asserts 830 rows, 828 witnesses and two SN-UNIT-WITNESS-WITHHELD-DIAGNOSTIC-WORK diagnostics.
- **Class notes.** No resolution rows.

### T6-C07 — Behaviour landed elsewhere, placement unresolved (17)

**Description.** The governed behaviour exists, but under another deliverable's
area, or the binding is only tentative, and no ruling has settled where it
belongs. Examples:
- classed diagnostics render in DiagnosticsPanel and in DEL-07-04's panel, not the DEL-07-01 viewport;
- checksum display and rule-pack and private-library reference editing live in the rule-pack manager and private-library features, not DEL-07-02;
- the runtime edit contract is the applier intent taxonomy, not DEL-16-01's schema;
- telemetry exclusion tests live in DEL-12-03, not DEL-12-04;
- DEL-09-04 carries the validation-manual regeneration that DEL-05-02 R02 records.

Nearly every RemainingWork states the same choice: record in the SOW that the
behaviour lives elsewhere (and cite it), or route it here.

- **Signature.** OWNERSHIP_ELSEWHERE · LOCAL_DESIGN · PARTIALLY_IMPLEMENTED (12); OWNERSHIP_ELSEWHERE · PROJECT_BASELINE · IMPLEMENTED_DIFFERENTLY (2); OWNERSHIP_ELSEWHERE · LOCAL_DESIGN · IMPLEMENTED_DIFFERENTLY (1), DOCUMENTED_UNIMPLEMENTED (1) and IMPLEMENTED_UNDOCUMENTED (1). All layers are RECORD. AuthorityNeeded is NO on 14 rows and OWNER on 3.
- **Population.** 17 rows. One is a STATUS#remaining row.
  - **Packages (9):** PKG-04, PKG-05, PKG-06, PKG-07, PKG-08, PKG-12, PKG-13, PKG-16 and PKG-17.
  - **Deliverables (11):** DEL-04-03 (1), DEL-05-02 (1), DEL-06-04 (1), DEL-07-01 (2), DEL-07-02 (4), DEL-07-05 (1), DEL-08-02 (1), DEL-12-04 (2), DEL-13-01 (1), DEL-16-01 (2) and DEL-17-02 (1).
- **Owning authority.** SCOPE_CHANGE, because assigning which deliverable owns a behaviour is a scope-change act. For the three OWNER rows, an owner choice comes first (see the notes below).
- **Recommended routing.** SCOPE_CHANGE_HANDOFF.
- **On-ruling mechanism.** An R4 ruling authorises Agent 0 to write the ownership assignments into `R3/SCOPE_CHANGE_HANDOFF/`, one entry per behaviour, naming the current landing area and the claiming deliverable. The scope-change workflow then records the binding. Its outcome authorises R5_RECORD_REPAIR of both SOWs: the claiming SOW cites the landing, or the landing deliverable takes the claim. No code change is implied unless the assignment moves behaviour.
- **Risk if unrepaired.** Claims stay partially met on paper while the behaviour exists, so coverage and ownership views double-count or under-count. Some rows, such as DEL-12-04 CLM-007.r03 and r04, cite no evidence at all for a boundary test that does exist elsewhere.
- **Representative keys.**
  - `DEL-07-02:SOW#CLM-018.r07`, in `WAVES/W1/PKG-07/DEL-07-02/DEL-07-02_forward.csv`. No checksum display exists in `ModelTree.tsx` or `PropertyInspector.tsx`. Checksums are generated and shown in the rule-pack manager (freeze `projects/chirality-piping/apps/desktop/src/features/rule-packs/RulePackManagerPanel.tsx:25`).
  - `DEL-16-01:SOW#CLM-009.r01`, in `WAVES/W1/PKG-16/DEL-16-01/DEL-16-01_forward.csv`. The runtime edit route is EditorOperationIntent through `core/model_operations/operation_applier` and never loads `schemas/model_operation.schema.json`. DEC-094 binds the vocabulary to the applier taxonomy. The row is FIELD: the ImplementationEvidence is `apps/desktop/src/types.ts#L645`.
  - `DEL-06-04:SOW#CLM-010/R-06-04-007`, in `WAVES/W3/PKG-06/DEL-06-04/DEL-06-04_forward.csv`. Rule-check data gaps are emitted by the DEL-06-03 completeness checker in `run_rule_checks`, not by the lifecycle.
- **Class notes.**
  - **Owner choice before handoff (3 OWNER rows).**
    - `DEL-16-01:SOW#CLM-009.r01` and `SOW#CLM-009.r03` (FG-01). Options: (a) make `model_operation.schema.json` the governing edit contract; (b) restate DEL-16-01 as the proposal/interchange schema and document the applier intent contract.
    - `DEL-17-02:CONTEXT#anticipated-artifacts`. Options: (a) keep the four schemas as contract-level tables, per DEC-076 contract-only ownership; (b) author common schema files.
    - Agent 0 may fold these into an R4 packet ahead of the handoff.
  - **Exceptions, kept visible.** FIELD: both DEL-16-01 rows. OBSERVED: `DEL-04-03:SOW#CLM-010.r12`. Under F7, a library reading gives ALIGNED with PRODUCT_CALLER: NONE, because `apply_linear_supports` is called only by tests and benchmarks. The verifier calls this an internal tension, not an error.
  - **Tentative binding.** `DEL-13-01:SOW#CLM-005.r07` is IMPLEMENTED_UNDOCUMENTED. MEMORY attributes a desktop Design Knowledge panel row to DEL-13-01, but the panel lives in the GUI area.

### T6-C08 — Gap held by a named ruling (36)

**Description.** The unmet part is deferred by a named governing ruling, and
each row's Notes name it:
- DEC-080, which keeps the live CAEPIPE external run owner-gated on lawful tool access (DEL-17-05, 17 rows);
- DEC-059 and DEC-058, on hosted CI and the release scan (DEL-08-05 4, DEL-10-04 1 and DEL-04-05 R03);
- DEC-052 and DEC-053, on post-R4/R5 threshold residuals (DEL-04-04 R04–R06 and DEL-04-05 R02);
- DEC-067, on the friction-history model (DEL-04-04 R02);
- D-67, on the dormant pressure kernel activation (DEL-05-03 R01 and R03);
- DEC-060, on the coverage-floor trigger (DEL-09-05 R02);
- DEC-078, on FR-025 post-beta (DEL-10-03 OUT-001);
- DEC-042 and DEC-094, on the R7 hold and generator ownership (DEL-16-04 R02);
- the unruled acceptance workflow (DEL-15-04, 2 rows);
- OI-014 (DEL-14-04 OQ-14-04-001; see R3 observation O2).

Under C6(a) these rows are correctly not ALIGNED. The deferral itself is sanctioned.

- **Signature.** DEFERRED_BY_RULING · LOCAL_DESIGN · PARTIALLY_IMPLEMENTED (17); PROJECT_BASELINE · PARTIALLY_IMPLEMENTED (10); PROJECT_BASELINE · DOCUMENTED_UNIMPLEMENTED (5); INVARIANT · PARTIALLY_IMPLEMENTED (2); INVARIANT · DOCUMENTED_UNIMPLEMENTED (1); LOCAL_DESIGN · DOCUMENTED_UNIMPLEMENTED (1). BaselineClass is RULED_CRITERION on 23 rows. AuthorityNeeded is OWNER on 29 rows and NO on 7.
- **Population.** 36 rows. Ten are STATUS#remaining rows.
  - **Packages (9):** PKG-04, PKG-05, PKG-08, PKG-09, PKG-10, PKG-14, PKG-15, PKG-16 and PKG-17.
  - **Deliverables (11):** DEL-04-04 (4), DEL-04-05 (2), DEL-05-03 (2), DEL-08-05 (4), DEL-09-05 (1), DEL-10-03 (1), DEL-10-04 (1), DEL-14-04 (1), DEL-15-04 (2), DEL-16-04 (1) and DEL-17-05 (17).
- **Owning authority.** OWNER, since lifting a deferral is an owner gate or stage act. No R4 decision is needed to keep the status quo.
- **Recommended routing.** NO_ACTION. Each deferral is held by the ruling named in the row, and its trigger is recorded in RemainingWork, for example "after the DEC-059 prerequisites" or "once the owner gate opens". Reopening is an ordinary owner gate or work-graph act when the trigger occurs, not an R3/R4 repair.
- **On-ruling mechanism.** None is required. If the owner chooses at R4 to lift a deferral (for example DEC-080 lawful-tool access or a DEC-052 human gate), that ruling authorises candidate code-fix briefs for the rows it names.
- **Risk if unrepaired.** Low while the triggers are unmet. The residual risk is the non-deferred residue inside deferred rows (observation O1) and records that read the absent behaviour as met; the ledgers already prevent that under CP-11 (12 rows).
- **Representative keys.**
  - `DEL-17-05:SOW#CLM-012/DEL-17-05-REQ-001`, in `WAVES/W3/PKG-17/DEL-17-05/DEL-17-05_forward.csv`. The disabled state holds by absence, since nothing launches a process: no process-launch call in `projects/chirality-piping/core/handoff/caepipe_external/run.py` at the freeze. DEC-080 owner-gates activation.
  - `DEL-08-05:SOW#CLM-011/DEL-08-05-REQ-010`, in `WAVES/W3/PKG-08/DEL-08-05/DEL-08-05_forward.csv`. Blocking severity exists, and no CI guard runs the linter, because DEC-059 defers hosted CI and DEC-058 sets the owner-run release scan.
  - `DEL-04-05:STATUS#remaining/R02`, in `WAVES/W2/PKG-04/DEL-04-05/DEL-04-05_forward.csv`. No release timing, RSS or conditioning thresholds exist, and DEC-053 and DEC-052 leave them as post-R4/R5 residuals.
- **Class notes.**
  - **Exceptions.** RESOLVED_PAIR: `DEL-15-04:SOW#CLM-010.r07` and `SOW#CLM-012.r03` (CP-11). No CONTESTED, FIELD or OBSERVED rows.
  - **CP-11 (12 rows).** Ten DEL-17-05 rows and the two DEL-15-04 rows.
  - **Link to C04 D1.** The DEL-15-04 rows and C04 D1 (DEL-05-04) turn on the same unruled human acceptance workflow. An owner ruling on OI-007 would reach both.

### T6-C09 — Sanctioned SCA-009 re-point of DEL-07-03 (exception: 6 CONTESTED) (9)

**Description.** The load-case (R-005) and support-editor (R-006) ownership
landings were re-pointed from DEL-07-03 to DEL-07-09 by SCA-009/DEC-094.
DEL-07-03's code rejects `load_case` and `support` editor kinds
(`EDITOR_KINDS = {"material", "component", "rule_pack_reference"}`, freeze
`projects/chirality-piping/core/gui/editors/engine.py:26`), and authoring exists
under the DEL-07-09 landing. The disposition is ACCEPTED_DIVERGENCE, and the
only remaining work is an optional SOW catch-up citing the landing. Six rows are
CONTESTED: the candidate reading is UNKNOWN · AUTHORITY_UNCLEAR · PROJECT_BASELINE ·
RECORD · OWNER, or it "rests on the same R-005 reading". So the class is kept
separate rather than merged into C07 or C08.

- **Signature.** OWNERSHIP_ELSEWHERE · PROJECT_BASELINE · ACCEPTED_DIVERGENCE (9). Layer RECORD.
- **Population.** 9 rows, all DEL-07-03 (PKG-07). Keys: `SOW#CLM-005.r04`, `SOW#CLM-005.r05`, `SOW#CLM-008/DEL-07-03-R-005`, `SOW#CLM-008/DEL-07-03-R-006`, `SOW#CLM-012/DEL-07-03-R-002`, `SOW#CLM-012/DEL-07-03-R-005`, `SOW#CLM-012/DEL-07-03-R-006`, `SOW#CLM-017` and `SOW#CLM-018`.
- **Owning authority.** OWNER, for the contested reading only.
- **Recommended routing.** NO_ACTION on the effective values: SCA-009/DEC-094 is a named governing ruling permitting the state (C6(g)). The six CONTESTED rows should be listed in Agent 0's contested-resolution set so R4 can confirm or replace the effective reading.
- **On-ruling mechanism.** If R4 confirms the effective reading, nothing changes; an optional R5_RECORD_REPAIR adds the DEL-07-09 citation to the DEL-07-03 SOW. If R4 adopts the contested candidate, the rows become an owner decision on whether SCA-009 closes R-002, R-005 and R-006 for DEL-07-03 (the options in evidence are "sanctioned split closes them" and "visibility is not closure", per CLM-018). That decision would be recorded as a DEC/SCA clarification.
- **Risk if unrepaired.** Low. The ownership split is ruled. The risk is only that R-002 closure stays ambiguous across DEL-07-03 and DEL-07-09.
- **Representative keys.**
  - `DEL-07-03:SOW#CLM-012/DEL-07-03-R-005` (CONTESTED), in `WAVES/W1/PKG-07/DEL-07-03/DEL-07-03_forward.csv`. SCA-009/DEC-094 re-point the landing and change no requirement.
  - `DEL-07-03:SOW#CLM-017` (CONTESTED). The Notes say the PDU-041 declaration matches `EDITOR_KINDS` and its test.
  - `DEL-07-03:SOW#CLM-012/DEL-07-03-R-006` (not contested). This is the same re-point for supports.
- **Class notes.** The CONTESTED rows are `SOW#CLM-005.r04`, `SOW#CLM-008/DEL-07-03-R-005`, `SOW#CLM-012/DEL-07-03-R-002`, `SOW#CLM-012/DEL-07-03-R-005`, `SOW#CLM-017` and `SOW#CLM-018`.

### T6-C10 — Conditional example rules met by absence (exception) (3)

**Description.** The DEL-11-03 theory note contains no numerical examples, so
the "if examples are introduced, they are invented, non-code and disclaimed"
rules hold only because the governed content does not exist (CP-11). Nothing
requires examples to be added, so no implementation gap exists to close. The
FIRM resolution on `CLM-014.r20` records an unresolved convention question. If
R3 or the owner reads conditional "if used" permissions as ALIGNED,
REQ-11-03-08 and CLM-028 move instead, and the ledger cannot keep both readings.

- **Signature.** NOT_STARTED · LOCAL_DESIGN · DOCUMENTED_UNIMPLEMENTED (3). Layer RECORD.
- **Population.** 3 rows, all DEL-11-03 (PKG-11): `SOW#CLM-011/REQ-11-03-08`, `SOW#CLM-014.r20` and `SOW#CLM-028`.
- **Owning authority.** NONE for the product. The CP-11 convention question on conditional permissions belongs to Agent 0's R4 conventions item.
- **Recommended routing.** NO_ACTION. RemainingWork is "none required unless examples are added". The rule stays in force for any future example.
- **On-ruling mechanism.** None for the product. If R4 rules on how CP-11 treats conditional permissions, the ruling authorises R5_RECORD_REPAIR of these three rows (and any like them corpus-wide) to one consistent reading.
- **Risk if unrepaired.** None to product behaviour. The inconsistency risk is limited to how conditional claims are counted.
- **Representative keys.**
  - `DEL-11-03:SOW#CLM-011/REQ-11-03-08`, in `WAVES/W3/PKG-11/DEL-11-03/DEL-11-03_forward.csv`. The note contains no examples, and nothing requires them.
  - `DEL-11-03:SOW#CLM-014.r20` (FIRM: CanonicalSituation CP-11, AuthorityNeeded NO). The evidence is `projects/chirality-piping/docs/theory/centerline_analysis.md` L175 and L183, as cited in the ledger.
- **Class notes.** Contrast C01's DEL-17-04 CP-11 rows, which carry an implement-or-narrow action.

## Coverage

- **Population.** `R3/CORPUS_CLAIMS.csv` rows with `Divergent = YES` and effective CauseTag in {PARTIAL_SLICE, NOT_STARTED, DEFERRED_BY_RULING, OWNERSHIP_ELSEWHERE}: **521**. That is PARTIAL_SLICE 419, NOT_STARTED 40, DEFERRED_BY_RULING 36 and OWNERSHIP_ELSEWHERE 26.
- **Output.** `T6_CLASSES.csv` has 521 body rows plus `#END,,521`, with CRLF line endings.
- **Check.** A read-only Python script reloaded `CORPUS_CLAIMS.csv`, rebuilt the population with the filter above, and compared it with `T6_CLASSES.csv`. It found:
  - 521 keys in the population and 521 in the output;
  - 0 missing, 0 extra and 0 duplicate keys;
  - DeliverableID matching `CORPUS_CLAIMS.csv` on every row;
  - every ClassID in {T6-C01 … T6-C10};
  - `#END` count = 521.
- **Class counts.** C01 180, C02 116, C03 34, C04 87, C05 35, C06 4, C07 17, C08 36, C09 9, C10 3. The sum is 521.
- **Resolution rows kept visible.** 19 CONTESTED rows (18 CONTESTED and 1 CONTESTED;FIELD), 7 FIELD, 5 OBSERVED and 1 FIELD;OBSERVED. Each is named in its class notes: C02 9, C03 5, C04 4, C05 3, C07 3, C09 6 and C01 2.
- **Other flags.** The 11 PRODUCT_CALLER: NONE rows are named in C01 (3), C02 (4), C03 (3) and C04 (1).
- **Ledger rows.** All 521 keys resolved to a sealed forward ledger row (no `superseded_<n>/` folder was read). No draft resolutions file (`RESOLUTIONS_DRAFT*.csv`, `RESOLUTIONS_MERGED_DRAFT.csv`) was read.

## R3 observations

These observations are not corrections. The effective values stand.

- **O1. Non-deferred residue inside C08 DEL-17-05 rows.** DEC-080 defers the live external run (invoke, capture, discover). Several C08 rows also record missing record fields that no ruling defers: `DEL-17-05:SOW#CLM-004.r02` (OS, permissions and launch context have no schema field), `SOW#CLM-026` (step 5 environment context has no schema field) and `SOW#CLM-029.s01` (run metadata lacks manifest, ID-map and loss-report refs). The same fields appear as C01 PARTIAL_SLICE work on sibling DEL-17-05 rows (`SOW#CLM-006.r02`, `SOW#CLM-006.r04` and `SOW#CLM-028`: "Complete the named record fields; the live parts wait on the owner gate"). The C01 DEL-17-05 brief should cover this residue. The cause tag on the C08 rows is defensible because the cause of the larger gap wins (C7).
- **O2. `DEL-14-04:SOW#CLM-035/OQ-14-04-001` has effective cause DEFERRED_BY_RULING, but its Notes name no deferring ruling.** They record that SOFTWARE_DECOMP OI-014 is still open, that "DEC-026 governs numerical verification tolerances, not comparison defaults", and that the subject is undecided by design. Its RemainingWork is a human product decision. On this evidence it reads as an open owner selection, which fits C04 D7 beside its sibling `OQ-14-04-002`. T6 keeps it in C08 as the effective value requires, and flags it for Agent 0.
- **O3. The CP-09 parity question moves up to three C05 rows.** If R4 adopts the CP-09 reading, `DEL-04-04`, `DEL-04-05` and `DEL-04-06` `SOW#production-and-verification-method-praxeology/VER-001` would take EVIDENCE_OVERTAKEN, moving them from T6 C05 to the T5B partition. Agent 0 should re-check exactly-once coverage after any such R4 outcome.
- **O4. `DEL-10-02` FG-DEL-10-02-01 rows (C04 D2) are CP-11 in substance** ("CP-11: not ALIGNED" in the Notes), but CanonicalSituation is blank on them. This has no routing effect.
- **O5. Hold overlap across classes.** One plugin/adapter runtime selection (C04 D2) governs the C04 D2 rows and also the no-bypass branch of the C03 DEL-06-02 CP-11 rows (`SOW#CLM-006.r05`, `SOW#CLM-013/REQ-06-02-010` and `SOW#CLM-016/REQ-06-02-010`), whose RemainingWork waits for "a governed runtime dispatch path". Similarly, C02 `DEL-08-01:STATUS#remaining/R01` depends on the C04 D10 compatibility-window ruling. Agent 0 may sequence those C03 and C02 briefs after the corresponding owner packets.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
