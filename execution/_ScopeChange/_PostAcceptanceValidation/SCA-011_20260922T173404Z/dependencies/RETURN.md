# Accepted dependency adoption and current-source handoff

**PASS. DAG-011 is adopted and current; dependency evidence is bound to the final accepted sources.**

The owner answered “Accept the audited result and adopt DAG-011,” recorded with the actual question and relay provenance in `../OWNER_DECISION.md`. This finalization applies the accepted decision to reviewed state `d6cc1482eee78ce860ff18658f11157f7efbd401`; it creates no further human checkpoint.

Executed the exact 20 `REPAIRED_GROUP3_PROMOTION.json` transformations with before/after hash guards: ten index authority transitions and ten CSV no-ops. `LOCAL_ACCEPTANCE_APPLICATION.json` records every reviewed and accepted hash. DAG-011's approval, provenance, handoff and manifest now record actual adoption, and `_DAG/_LATEST.md` names DAG-011. DAG-010 remains immutable; prior adoption metadata are preserved under `preacceptance/` here. The prior pointer bytes are stored as `preacceptance/projects/chirality-piping/execution/_DAG/latest-pointer-before.txt`, a non-active evidence filename so live-pointer discovery cannot mistake this historical copy for another authority root.

After the carrier TASK declared all 51 final source targets stable, the dependency checks rebound their actual accepted bytes. `CURRENT_SOURCE_BINDINGS.json` contains 84 current row-specific source paths, loci, verbatim quotes and before/after source hashes, plus the 31 local-source/decomposition file bindings. Every changed source hash matches the exact reviewed carrier transformation; all 84 quotes remain verbatim within their named locus. No claim or quote was altered merely to preserve a passing result. The reconciliation TASK independently completed its accepted-source derivative without touching dependency files.

The graph's CSV bytes are unchanged from the accepted reviewed state:

| Artifact | SHA-256 |
|---|---|
| DAG-011/DependencyEdges.csv | `5c056c2538026f62276a86d88aa9d95f24e262d4d7e38dcdc2f11e1705dc5c47` |
| DAG-011/DeliverableNodes.csv | `91eb889930f4e97594a4b8627708d667473f066e9cde7e5fca0bf9a2b053613f` |
| Final decomposition | `c78301c67df5729d65c57963e6a915339049e3ffaa12d13961ee201445a9b984` |

Fresh bounded checks pass: ten local schemas; exact local/aggregate parity for the 84 added canonical IDs; unchanged semantic fields and predecessor byte prefixes; 84 literal quotes in named loci; zero new-row stock evidence/owner-binding findings; the strict 106-node, 1,571-row aggregate audit; and the 1,041-edge stage expansion. Nine negative probes still detect baseline damage, duplicate/dangling/false-satisfaction errors, three reversed stage relations, missing quotes and wrong owner IDs. The historical 1,487 rows and 102 nodes are unchanged. The generic shell ID helper's three-digit template incompatibility remains historical and separately disclosed; it was not rerun or presented as a pass.

All 62 added execution relations remain PENDING with ProposedMaturity=TBD. Baseline satisfaction is preserved, not recertified. The accepted stage model does not make its named contract drafts production-ready or adopt excluded wire wrappers. Product, native, engineering, privacy/external, lifecycle and release holds remain. No full audit/reconciliation rerun, product tests, Git operation or additional delegation occurred in this TASK.

Source origins, actual scope and native-child attribution are in `SOURCES.json`. Status/memory pairs were read without modification; six existing owners have compatibility `MEMORY.md`, while the four accepted new control sets have no memory file, which was recorded rather than filled speculatively. A pointer-format assertion initially halted before the pointer write; it was corrected to match the actual preserved predecessor path, and final pointer/provenance checks passed.

All new evidence is in this single postacceptance home. Original SCA-011 candidate/application/audit records and reports remain frozen historical evidence; current reports have not overwritten them. `ADOPTION_VERIFICATION.json` and `OUTPUT_MANIFEST.json` bind final records and hashes for manager integration and independent active-state review.

Replay from repository root:

```text
python3 projects/chirality-piping/execution/_ScopeChange/_PostAcceptanceValidation/SCA-011_20260922T173404Z/dependencies/apply_acceptance.py
python3 projects/chirality-piping/execution/_ScopeChange/_PostAcceptanceValidation/SCA-011_20260922T173404Z/dependencies/validate_accepted.py
python3 tools/coordination/audit_dag.py --dag-dir projects/chirality-piping/execution/_DAG/DAG-011 --canonical --strict
```

The first command verifies accepted current state by default; its one-time `--apply` mode rejects an already-promoted tree. The second updates only this postacceptance evidence directory and requires the exact accepted carrier hashes. Final evidence paths, rather than historical application reports, now supply current source currency.
