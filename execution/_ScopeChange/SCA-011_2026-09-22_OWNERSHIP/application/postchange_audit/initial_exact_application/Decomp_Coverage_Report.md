# SCA-011 actual postchange decomposition audit

Overall: **WARNINGS**. Closure-readiness verdict: **WARN**. This is derivative structural audit evidence for Group 3 preparation, not Group-3 acceptance or product readiness.

The audit binds the actual working files to the owner’s Group-2 snapshot `execution/_ScopeChange/checkpoint_snapshots/SCA-011_GROUP-2_2026-09-22/` and exact accepted package commit `3e18334eca72509475684cc86786b3eeade83572`. The initial applied state matches all 83 accepted postimage hashes; all 58 carrier and 20 dependency-local targets are covered by that enumeration. Source hashes and per-file checks are retained.

| Check | Name | Verdict | Evidence / limit |
|---|---|---|---|
|1|Forward packages|PASS|18/18 packages materialized.|
|2|Forward deliverables|PASS|106/106 deliverables materialized.|
|3|Reverse folders|PASS|No orphan folders; all declared and materialized sets equal.|
|4|ID consistency|PASS|No duplicate or reused IDs; old 102 deliverables and 77 scope IDs retained; exactly four deliverables and SOW-078/079 added.|
|5|Context fidelity|PASS|106 identity/name/package/type/envelope records match; four new descriptions and finite boundaries match source registers.|
|6|Artifact presence|INCOMPLETE|All core packages present: 97 validated SOW_V1, eight D-43 architecture references and one accepted DEL-07-09 OPEN custom contract. Anticipated implementation-artifact realization is not recounted.|
|7|Objective mapping|PASS|All 18 objective identities/statements preserved and supported. All displayed deliverable support sets agree; OBJ-018 baseline discrepancy resolved.|
|8|Ledger integrity|WARNING|79 markdown/CSV scope mappings agree and resolve. Stock local-register quality result remains nonzero; see explicit baseline delta below.|
|9|Derivative package parity|SKIPPED|DOMAIN-specific check not variant-owned; SOFTWARE register/decomposition parity is explicitly checked under 5, 7 and 8.|
|9b|Package shape|WARNING|Baseline absence of explicit companion inventory and heavy inline duplication remains; no new derived artifact substituted for amendment authority.|
|10|Snapshot and handoff|PASS|Accepted SCA-010 compact contract remains active; decomposition 0.13 applied pending Group 3; DAG-011 staged and unapproved; active pointers byte-identical to accepted basis.|
|11|Lifecycle distribution|PASS|102 existing lifecycle states unchanged; all four additions OPEN. Whole corpus: 100 IN_PROGRESS, one ISSUED, five OPEN. No lifecycle promotion.|

## Actual-versus-baseline comparison

Whole decomposition: 18 packages / 102 deliverables / 77 scope items / 18 objectives → 18 / 106 / 79 / 18. Existing identifiers are retained; new identities are DEL-04-07, DEL-07-11, DEL-07-12, DEL-16-06 and SOW-078/079. The four control sets are complete, OPEN, and semantically PLACEHOLDER, with three L envelopes and one M. Descriptions and boundaries constrain solve integration, workspace shell, finite interoperability review, and the sole controlled-operation mutation seam; no ownership of underlying mathematics, schema meaning, feature panels, storage policy or professional acceptance is inferred.

Baseline matrix scope was 14 packages / 80 deliverables; this audit expands to all 18 / 106. Raw lifecycle totals therefore have different denominators. The audit compares each of the 102 existing current states directly with accepted-source bytes and recognizes the existing eight architecture-reference contracts under D-43. Memory/status pairs were read; memory is continuity evidence only.

The baseline objective-summary warning is resolved: OBJ-018’s displayed support set now agrees with the companion deliverable register, including DEL-13-02 and the new DEL-07-12, without the erroneous prior DEL-12-02 support. All 18 objective statements and identities remain unchanged; authorized support mappings were updated.

The package-shape warning remains. Canonical machine truth is in `docs/_Registers/{Deliverables,ScopeLedger,ContextBudgetQA}.csv`, while the main document duplicates substantial tables and lacks an explicit companion inventory. The exact amendment preserves that baseline structure and the audit verifies parity rather than requiring unrelated restructuring.

The stock validator exits 1 and reports **2,265 errors**, versus **2,097 baseline errors**. All 2,097 original finding keys remain; 168 are new: 84 EVQ-003 blank inline EvidenceQuote findings and 84 DRB-006 ID-prefix findings on accepted SCA011-E001..084 rows. The stock tool also skips cross-register checks because companions are in docs/_Registers. This audit independently parses those actual companions and confirms parity; it never treats the stock result as a pass.

Each added dependency row has a separate actual-source backcheck with a nonblank literal quote and SHA-256 identity; all 84 were independently rechecked here. The selected dependency extraction method makes EvidenceQuote optional and preserves existing row IDs; these accepted bytes and separate evidence explain the mismatch but do not erase the stock quality findings. These are newly materialized findings, not mislabeled baseline debt. The parent subsequently authorized a bounded correction to those new dependency fields; this initial evidence is preserved and a distinct final audit must bind the corrected bytes.

## Stable claims and retained boundaries

All 24 affected ScopeOfWork documents pass the repository validator. Existing CLM/AC/VER/OUT identifiers are retained as subsets of the actual identifier sets, and the validator checks local references and output/evaluation hooks. The whole corpus contains 97 validated SoWs; the remaining nine use established alternate reference/control contracts. This establishes document structure and traceability, not claim satisfaction.

The applied decomposition status is `applied_group3_pending`. `_ScopeChange/_LATEST.md` remains SCA-010 and `_DAG/_LATEST.md` remains DAG-010. SCA-010 is assessed against its own accepted compact contract, not retroactively rejected against a newer full template. Current SCA-011 handoff truthfully describes derivative/audit work still completing. Group 3 is the existing human checkpoint, not a new blocker manufactured by this audit.

Canonical product core, apps, schemas and tests; DAG-010; SCA-009; and the original whole-corpus R6 reconciliation paths show no changes from the accepted package commit. Draft result wire schemas are absent from the 83-target manifest and remain unpromoted. No product tests, numerical/engineering conformance, private-data authorization, external activation, original satisfaction evidence, lifecycle issuance or release acceptance is claimed. Concurrent App and Root-owned changes are outside this Piping audit’s containment claim.

## Material findings and next action

See `Decomp_Coverage_IssueLog.csv` for the two grouped warnings and two informational findings, with source paths. There is no mismatch in the initial exact application or structural inventory. Preserve this initial audit, complete the newly authorized dependency-field correction, then re-audit corrected hashes, source quotes, row identity crosswalk, unchanged relation semantics and baseline preservation. The manager’s separate fresh complete-poststate review remains required before the Group-3 package is presented.
