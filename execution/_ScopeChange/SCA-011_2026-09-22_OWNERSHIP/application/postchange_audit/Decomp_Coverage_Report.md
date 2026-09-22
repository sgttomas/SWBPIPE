# SCA-011 corrected actual postchange decomposition audit

Overall: **WARNINGS**. Closure-readiness: **WARN**. No unresolved application or structural blocker was found. The remaining grouped warning is baseline package-shape debt; stock validation still has its 2,097 unchanged baseline findings. This is derivative evidence for Group 3 preparation, not poststate acceptance or product readiness.

The original 83-target exact application is preserved under `initial_exact_application/`. The current state retains 63 original postimage hashes and 20 explicitly corrected local dependency postimages. All 25 hashes in `application/dependencies/POSTIMAGE_DEVIATIONS.json` verify:20 locals, one staged edge CSV andfour staged provenance/approval/handoff/manifest files. `APPLY_MANIFEST.csv` remains byte-identical to the accepted commit; its hashes were not rewritten to conceal the correction.

| Check | Name | Verdict | Evidence / limit |
|---|---|---|---|
|1|Forward packages|PASS|18/18 packages materialized.|
|2|Forward deliverables|PASS|106/106 deliverables materialized.|
|3|Reverse folders|PASS|No orphan folders; declared and materialized sets equal.|
|4|ID consistency|PASS|102 prior deliverable IDs and 77 prior scope IDs retained; four new deliverables and SOW-078/079 added without reuse.84 new dependency IDs corrected by explicit bijection.|
|5|Context fidelity|PASS|106 name/identity/package/type/envelope rows match; four new descriptions and finite boundaries match companion sources.|
|6|Artifact presence|INCOMPLETE|All core contracts present:97 validated SOW_V1,eight D-43 architecture references, one accepted OPEN custom contract. Full anticipated production-artifact realization not recounted.|
|7|Objective mapping|PASS|All 18 objective IDs/statements unchanged and supported. Displayed support sets match; OBJ018 baseline discrepancy resolved.|
|8|Ledger integrity|WARNING|79 markdown/CSV scope mappings agree and resolve. Stock validator retains 2,097 baseline quality findings; no new findings after 84-row metadata repair.|
|9|Derivative package parity|SKIPPED|DOMAIN-specific check not variant-owned; actual SOFTWARE register parity is checked under 5,7,8.|
|9 b|Package shape|WARNING|Baseline missing companion inventory and heavy inline duplication retained; no derived artifact substituted for amendment authority.|
|10|Snapshot and handoff|PASS|SCA010 compact contract remains active; actual 0.13 applied pendingGroup 3; stagedDAG011 unapproved. Active pointers unchanged.|
|11|Lifecycle distribution|PASS|All 102 existing states unchanged; four additions OPEN. Whole corpus 100 IN_PROGRESS, one ISSUED,five OPEN.|

## Actual-versus-baseline comparison

The source snapshot is `execution/_ScopeChange/checkpoint_snapshots/SCA-011_GROUP-2_2026-09-22/`, binding exact accepted package commit `3 e 18334 eca 72509475684 cc 86786 b 3 eeade 83572`. Whole topology changes from 18 packages/102 deliverables/77 scope items/18 objectives to 18/106/79/18. New homes are DEL-04-07,DEL-07-11,DEL-07-12,DEL-16-06. Each has complete controls,OPEN lifecycle,PLACEHOLDER semantics and a finite accepted context envelope:three L andone M. Existing IDs,package membership and lifecycle are preserved.

The preserved prechange audit covered 14 packages/80 deliverables; this audit covers all 18/106. The larger lifecycle and core-contract denominators are disclosed rather than compared as if scope were equal. All 102 prior status records were directly compared with immutable accepted-source bytes; status/memory pairs were read,memory only as continuity. The eight PKG00 architecture references retain their D-43 contract rather than being subjected retroactively to the SoW format.

OBJ018’s summary now exactly matches current support in the deliverable register,including DEL-13-02 and new DEL-07-12,and excluding the erroneous prior DEL-12-02 mapping. All 18 objective IDs/statements are unchanged. Updated scope/support mappings are accepted amendment effects,not new objectives.

## Dependency defect,repair and independent backcheck

The first actual audit observed stock validation 2,097→2,265 errors:84 blank EvidenceQuote findings and 84 invalid new ID-prefix findings. That state and its evidence remain frozen under `initial_exact_application/`. The initial interpretation that accepted conventions could be disclosed without repair is superseded:Root SPEC §6.8 binds the DEP-owner identity,and §6.5 calls for available literal support. The parent expressly authorized the bounded correction; no new ownership election or human checkpoint was invented.

The correction changes exactly five metadata fields on each of 84 added rows:DependencyID,EvidenceFile,SourceRef,EvidenceQuote,Notes. `ID_CROSSWALK.csv` preserves every old SCA011 identity; `FIELD_LEVEL_DELTA.csv` preserves 420 field changes. Original evidence locators and IDs remain in Notes. This audit independently maps all 1,571 staged rows through that bijection,checks all 1,487 baseline row dictionaries unchanged,and proves every other added-row field unchanged. Endpoints,direction,type,statement,scope,ownership,maturity,satisfaction,status and dates therefore remain unchanged. New execution relations remain PENDING with actual maturity TBD.

For all 84 rows,the actual source SHA-256 matches,the quote is nonblank,literal and at most 30 words,and it occurs within its named heading,claim,verification bullet or frontmatter field. SourceRef equals the current source file plus that locus. All 10 local CSVs preserve their prior rows and their new rows agree with the staged graph. The two optional extension columns are absent in some local schemas and present blank in the aggregate; comparison treats only those empty/missing extension values equivalently,while original local rows are compared exactly.

Final whole-project stock validation returns 2,097 errors:all 2,097 original finding identities unchanged, zero new findings,none silently waived. The tool still skips cross-register checks because companions live in docs/_Registers. The audit explicitly parses those actual companions and verifies identity,envelopes,scope mappings and objective support against decomposition; that supplement is not mislabeled a stock pass. Raw outputs and exact finding-key comparison are retained.

## Context,claims and authority boundaries

All 24 affected SoWs retain their existing CLM/AC/VER/OUT identifiers and pass reference/output-evaluation validation. All 97 current SoWs pass. New descriptions and finite boundaries preserve mathematical primitives,convergence policy,schema meanings,feature semantics,persistence,privacy and professional acceptance with their owning contracts. This demonstrates structural traceability,not implementation or claim satisfaction. Full anticipated production-artifact realization remains INCOMPLETE and is not used to manufacture a new acceptance gate.

The decomposition is 0.13,`applied_group 3_pending`. SCA010 remains the active scope-change snapshot;DAG010 remains current graph authority. SCA010 is evaluated under its accepted compact contract,not retroactively against a newer full-template artifact list. DAG011 remains staged and unapproved. Group 3 is the already-selected checkpoint. No lifecycle promotion,product/native test qualification,numerical or engineering acceptance,private-data authorization,external-prover activation,satisfaction recertification or release claim follows.

Canonical product core,apps,schemas and tests;DAG010;SCA009;and original whole-corpus R6 reconciliation paths are unchanged from accepted-package commit. Draft wire schemas remain excluded from canonical application. Concurrent App and Root-owned changes are outside this Piping audit’s containment claim. Manager handoff/summary describe applied state and outstanding review/acceptance rather than pretending a future decision happened.

## Remaining finding and next action

One grouped WARNING remains:main decomposition lacks an explicit companion inventory and duplicates heavy register truth. This was present before the amendment; actual register parity passes. Three INFO entries distinguish the corrected dependency finding,the unchanged stock baseline,and SCA010’s accepted historical compact contract. See the issue log for source references.

Return this audited package to the manager’s separately assigned fresh complete-poststate reviewer. Then present Group 3 with the explicit correction,baseline limits and retained obligations. Do not change active pointers or execute conditional Group 3 transformations without the actual owning ruling.
