# SCA-011 dependency metadata correction

**Applied correction; Group 3 remains pending.** This record supersedes the initial return's suggestion that disclosure alone disposed of the 168 newly introduced stock-validator findings. The original Group-2 candidate and initial application evidence remain unchanged and inspectable.

The manager relayed Agent 0's authorization to repair routine quote/ID propagation defects within the accepted ownership amendment and present the exact deviation with the audited Group-3 poststate. Root `docs/SPEC.md` §6.8 requires `DEP-{PKG}-{DEL}-{SEQ}` identity; §6.5 calls for verbatim evidence quotes of at most 30 words. The selected dependency workflow's optional-quote wording does not displace the Root identity rule, and quotable support was now available for every added row. No new owner choice about scope or ownership was needed to correct those records.

## Exact change

Only the 84 added rows were repaired, in all ten local CSVs and staged DAG-011. `ID_CROSSWALK.csv` maps every original `SCA011-E001`–`SCA011-E084` identity to an unused canonical `DEP-{owner}-{sequence}` identity. Allocation checked the IDs of all 98 currently materialized local dependency registers and the staged aggregate; old rows were not renamed. Original SCA011 IDs and proposal evidence locators remain appended in each row's Notes.

Five metadata fields change per added row: DependencyID, EvidenceFile, SourceRef, EvidenceQuote and Notes. `FIELD_LEVEL_DELTA.csv` records all 420 field changes once, keyed by old/current ID; the same metadata is present in local and aggregate copies. Evidence now points to the actual applied local ScopeOfWork claim/frontmatter, with a verbatim quote and distinct source locus. The earlier local source backcheck retains the decomposition/staged-contract reasoning that resolves the input family to its owner.

No endpoint, direction, type, statement, ownership, scope identity, maturity, satisfaction, status or date changes. The exact 1,487-row/102-node DAG-010 prefixes remain unchanged; each existing local CSV retains its exact original byte prefix. All 62 added execution relations remain PENDING with ProposedMaturity=TBD. The ten indexes display the current IDs and an explicit correction history. Current stage-validation references are derived through the bijection; original accepted stage sources remain frozen.

`REPAIR_PREIMAGES.json` and `repair_preimages/` preserve the actual pre-repair files. `POSTIMAGE_DEVIATIONS.json` records every changed path's original approved Group-2 hash (where one existed), actual before-repair hash and actual after-repair hash. Original `APPLY_MANIFEST.csv` hashes continue to describe the originally accepted/applied bytes; they were not rewritten to conceal this correction. `REPAIRED_GROUP3_PROMOTION.json` prepares new future transformation hashes for the repaired local indexes/CSVs. Those transformations have **not** been executed.

## Verification

`REPAIRED_VALIDATION.json` proves the 84-row identity bijection, all unchanged semantic fields, exact predecessor prefixes, current local/aggregate parity, verbatim source quotes (84/84), ten local schema passes and zero stock EVQ/DRB findings on the repaired rows. The registered decomposition checker predicates run unmodified. Negative blank-quote and wrong-owner-ID inputs are detected by the same predicates; no suppression or waiver was added.

The strict aggregate audit passes on actual repaired bytes: 106 nodes, 1,571 rows, 1,034 active deliverable edges, no canonical/endpoint errors, SCCs, duplicate or bidirectional edges. The accepted stage model still has 1,041 edges; its seven original negative probes pass. Its source functions were reused unchanged, and the three document-binding IDs were mapped through the crosswalk, not hidden from cycle checking. Fresh repaired reports coexist with the unchanged initial reports because they concern different actual file states.

The independent whole-project stock-validator rerun is owned by the postchange auditor. Historical baseline findings are not claimed fixed by this metadata correction. The generic ID helper's three-digit package template still differs from Piping's accepted two-digit convention; current dependency IDs satisfy Root SPEC's canonical owner prefix and the stock owner-binding predicate.

## Rerun and boundary

From repository root:

```text
python3 projects/chirality-piping/execution/_ScopeChange/SCA-011_2026-09-22_OWNERSHIP/application/dependencies/validate_repaired.py
python3 tools/coordination/audit_dag.py --dag-dir projects/chirality-piping/execution/_DAG/DAG-011 --canonical --strict
```

`repair_metadata.py` is a one-time applicator guarded by original hashes and exclusive preimage creation; do not rerun it on repaired state. Earlier exact-postimage replay scripts intentionally reject the corrected state. Use the repaired validator and deviations for current review.

DAG-010 remains current. DAG-011's approval record remains explicitly unapproved. Group 3 must review this correction together with the applied amendment; no graph adoption, human acceptance of a future poststate, source readiness, lifecycle promotion, product/native/engineering qualification or release is inferred.
