> **Historical initial application return; superseded for the current poststate.** The original bytes are preserved in [initial_exact_application/DEPENDENCY_APPLICATION_RETURN.md](initial_exact_application/DEPENDENCY_APPLICATION_RETURN.md). The current record is [CORRECTNESS_REPAIR.md](CORRECTNESS_REPAIR.md), with exact accepted-byte differences in [POSTIMAGE_DEVIATIONS.json](POSTIMAGE_DEVIATIONS.json). Use `validate_repaired.py` for current checks. The original exact-postimage scripts and hashes below describe the pre-repair application and intentionally reject the corrected state.

# Dependency application — SCA-011 Group 2

**PASS within the accepted application scope. Group 3 and graph adoption remain pending.**

Applied all 20 `GROUP2_DEPENDENCY_TASK` targets using only the accepted `LOCAL_MIRRORS_PROMOTION.json` `files` operations. Every preimage matched before writing; every actual postimage matches `APPLY_MANIFEST.csv` `AppliedSHA256`. No `group3_files` operation ran. Staged `DAG-011/DependencyEdges.csv` and `DeliverableNodes.csv` are byte-identical to `DEPENDENCY_SNAPSHOT_MANIFEST.json`. Neither active SCA nor DAG pointer was changed.

The actual Group-2 owner decision is recorded in `execution/_ScopeChange/checkpoint_snapshots/SCA-011_GROUP-2_2026-09-22/DECISION.md`. Source identity, actual native-child parentage, method selection and enforcement limits are in `SOURCES_DEPENDENCY_APPLICATION.json` here. This TASK did not delegate, run Git or modify source/control files. The carrier TASK supplied and verified actual claim postimages before mirror application; this TASK independently checked their applied hashes. Four new dependency indexes were announced to that TASK for its combined minimum-fileset validation.

## Local extraction and backcheck

Selected source-qualified method: **bundled:chirality-root/dependency-extract**, from `workflows/dependency-extract/WORKFLOW.md` and its current checks. Ten owners were examined individually: DEL-04-07, DEL-07-01/03/04/08/11/12, DEL-08-04 and DEL-16-03/06. Full local ScopeOfWork, context and reference bytes were read and hashed; analysis is bounded to the accepted additive obligations, with the applied decomposition used for identity and architecture resolution. Pass 1 establishes the 22 added parent/scope/objective anchors; pass 2 records local support for the 62 information-transfer relations. `LOCAL_SOURCE_BACKCHECK.csv` supplies one actual local source/hash/section/quote and resolution for each of the 84 rows.

This preserves the exact reviewed row IDs and CSV bytes. Historical proposal citations and candidate-time Notes remain provenance; the applied local-source support is a separate, explicit backcheck, not a silent rewrite of those citations. Mapping a local contract family to its decomposition owner is an executing TASK judgment; literal-quote and ID assertions guard its replay but do not themselves prove that semantic judgment. Architecture constraints use the existing §8.1 basis. The three document targets preserve the reviewed earlier-contract/later-evidence distinctions. No information-transfer claim is inferred merely from two neighboring folders.

Mode is UPDATE with only the accepted additive delta; strictness CONSERVATIVE; anchor source is local ScopeOfWork frontmatter; execution sources are its named claims, local context boundaries and reference locators. No baseline row is removed, retired or recertified. All six existing local CSV preimages remain hash-proven exact byte prefixes, independently of the aggregate. All ten local registers have one active parent anchor, unique IDs, correct owner identity and summaries matching their rows. Centralized execution evidence supplements the exact accepted index histories; no unreviewed metadata was added to their postimages.

## Checks and limits

- Ten registered dependency-schema checks and 31 distinct enum checks passed. All 84 added rows match the accepted delta; all ten local indexes match the exact applied hashes and row counts.
- The stock generic ID helper returned nonzero for 93 unique DEL/PKG identities because its template requires three-digit package numbers while accepted Piping uses two-digit package numbers. This applicability failure was retained in `LOCAL_CHECK_COMMANDS.json`, not labelled a pass or fixed by renaming accepted identities. Each identity was separately checked against Piping's two-digit syntax and actual applied decomposition. Initial validator execution stopped at this discrepancy before the applicability handling was added.
- The independent postchange audit also ran `validate_decomposition_registers.py` and returned exit 1: 2,097 baseline findings unchanged, plus 168 findings on the accepted delta (84 `EVQ-003` blank historical quote fields and 84 `DRB-006` non-template row IDs). See `../postchange_audit/register_comparison.json`. These are disclosed findings, not a passing stock run. The selected dependency-extract method expressly makes `EvidenceQuote` optional while requiring `EvidenceFile` and `SourceRef`; the exact accepted rows supply those and use the reviewed SCA011-E001–084 identities. The fresh local quoted source support is separately keyed in `LOCAL_SOURCE_BACKCHECK.csv`. This TASK preserves the accepted bytes and does not suppress findings, invent a waiver or rename IDs to satisfy a different template. The whole-amendment audit retains their disposition for the final checkpoint.
- Fresh stock `audit_dag.py --canonical --strict` on the actual staged DAG-011 bytes passed: 106 nodes, 1,571 rows, 1,034 active deliverable edges; zero endpoint/canonical findings, SCCs, duplicate edges or bidirectional pairs.
- The accepted stage validator was replayed on those same staged bytes and the applied canonical Deliverables register, with its checks and seven negative probes unchanged. `validate_staged.py` documents every I/O-only rebinding; it does not rewrite the accepted validator. The 1,041-edge stage expansion passed. Baseline damage, duplicate row, dangling endpoint, false satisfaction and reversal of each of the three stage bindings were detected for their expected reasons. A separate missing-local-source-literal probe also failed as intended.
- The 1,487 DAG-010 rows and 102 nodes remain exact byte prefixes. The delta has four nodes, 22 anchors and 62 execution rows; all new execution satisfaction is PENDING and actual proposed maturity is TBD. Existing maturity/satisfaction evidence was preserved, not freshly revalidated.

Local workflow work and ad hoc cross-deliverable aggregation are distinct scopes. The workflow did not authorize graph construction: that was explicitly assigned by the manager. One canonical copy of fresh audit/backcheck results lives here; DAG-011 provenance links it. No product tests, source-owner readiness witnesses, native behavior, engineering validation, lifecycle promotion or release claim is made.

## Replay and next participant

From repository root, while Group 2 remains the applied state:

```text
python3 projects/chirality-piping/execution/_ScopeChange/SCA-011_2026-09-22_OWNERSHIP/application/dependencies/apply_dependencies.py
python3 projects/chirality-piping/execution/_ScopeChange/SCA-011_2026-09-22_OWNERSHIP/application/dependencies/backcheck_local_sources.py
python3 tools/coordination/audit_dag.py --dag-dir projects/chirality-piping/execution/_DAG/DAG-011 --canonical --strict
python3 projects/chirality-piping/execution/_ScopeChange/SCA-011_2026-09-22_OWNERSHIP/application/dependencies/validate_staged.py
```

The first command verifies current postimages; `--apply` is only for a restored exact prestate and rejects an already-applied tree. Replay scripts update their application evidence reports, not accepted candidate evidence. Group-3 transformations intentionally change the expected state and require their separate audited operation.

Return to the manager for whole-amendment independent postchange audit and presentation of the actual poststate. `DAG-011/APPROVAL_RECORD.md` explicitly records **not yet approved**; DAG-010 stays current until an actual Group-3 disposition. No additional owner question was created by this task.
