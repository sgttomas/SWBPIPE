# Foundation closeout inventory

Status: `PREPARED`; no closure or pass is asserted. This inventory consumes D-67 and the Step 0 basis. It preserves decomposition 0.12, SCA-009, DAG-010/R5, dependency rows, lifecycle states and all owner-only issuance/release judgments.

## Exact primary deliverable writes after combined acceptance

### DEL-14-02 — Analysis run records

Files:

- `execution/PKG-14_Model States, Analysis Runs, and Comparison/1_Working/DEL-14-02_Analysis run records/_STATUS.md`
- `execution/PKG-14_Model States, Analysis Runs, and Comparison/1_Working/DEL-14-02_Analysis run records/MEMORY.md`

Required `_STATUS.md` entries:

- Keep `Current State: IN_PROGRESS`.
- Update `Last Updated` to the actual closeout date.
- Replace only the compatibility portion of the second `Remaining` bullet after the final acceptance proves exact version dispatch, both retained legacy profiles, the new strict 0.2 record, the previously unsupported admissions, and interpretation handling. Retain the PDU-033 producer-evidence hold wherever a source dimension or other governed result metadata remains unavailable; explicit `unverifiable` evidence does not close that producer obligation.
- Keep the first producer diagnostic-breadth bullet unless final evidence separately proves it. D-67 does not itself prove that breadth.
- Add one History entry pointing to the foundation final acceptance and saying lifecycle and formal dependency dispositions are unchanged.

Required `MEMORY.md` entry: one bounded foundation note naming the accepted analysis-record versions/profiles, source-row preservation, checked-hash authority, Current/Historical behavior, native lifecycle result, exact accepted source commit and one owning final-acceptance pointer. Name retained PDU-033 and diagnostic holds without reproducing test counts.

### DEL-17-06 — Stress-neutral CSV/JSON package

Files:

- `execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-06_Stress-neutral CSV JSON package/_STATUS.md`
- `execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-06_Stress-neutral CSV JSON package/MEMORY.md`
- `execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-06_Stress-neutral CSV JSON package/_REFERENCES.md`

Required `_STATUS.md` entries:

- Keep `Current State: IN_PROGRESS`; the active dependency rows and lifecycle gate are unchanged.
- Update `Last Updated` to the actual closeout date.
- Remove the current frozen-preview-findings `Remaining` bullet only if the accepted final 0.2 emitted package closes that exact construction population and the native/producer witness exercises the actual decorated payload. A schema-only or fixture-only pass is insufficient.
- Add this retained derivative-currentness item: `Regenerate _SEMANTIC.md and _SEMANTIC_LENSING.md against the accepted post-D-67/CLM-042 state, or formally retire them from current use; their 2026-05-18 statements that exact schema/hash partitioning is unresolved are superseded and must not be used as current evidence. Refresh the dependency summary against DAG-010 through its owning dependency workflow; Dependencies.csv and DAG authority remain unchanged until then.`
- Add one History entry pointing to the foundation final acceptance and recording that technical 0.2 construction closure, if proven, does not change dependency satisfaction or lifecycle.

Required `MEMORY.md` entry: record the retained 0.1 serializer/label, strict 0.2 checked Rust profile, received-versus-computed checksum separation, exact accepted source commit, emitted package/materialization witness and one owning final-acceptance pointer. Explicitly mark `_SEMANTIC.md`, `_SEMANTIC_LENSING.md`, and `_DEPENDENCIES.md` as deferred derivative refreshes that are not current authority.

Required `_REFERENCES.md` entries:

- D-67 at `execution/_Coordination/_DECISIONS/D-67_RULING_2026-09-14.md`, described as the owner-adopted versioned 0.2 hash-contract amendment that preserves 0.1.
- The final foundation acceptance/handoff path once immutable.

Already-current authoritative amendment surfaces, to verify and leave unchanged unless final source contradicts them:

- `ScopeOfWork.md` CLM-042.
- `_CONTEXT.md` section `D-67 bounded versioned compatibility adoption`.
- D-67 and its decision-register row.

Derivative disposition: `_SEMANTIC.md` and `_SEMANTIC_LENSING.md` are generated from a pre-D-67 state and refer to missing standalone production documents; `_SEMANTIC_LENSING.md` C-002 says hash partitioning is unresolved. `_DEPENDENCIES.md` names DAG-007 while Step 0 resolves DAG-010. Do not silently treat these artifacts as current, edit historical `_REVIEW.md`, close `Review_Findings.csv` RF-001, or mutate `Dependencies.csv`/DAG. The bounded closeout may explicitly defer regeneration through the `_STATUS.md`, `MEMORY.md`, and final handoff wording above. Actual regeneration must use the owning semantic/dependency workflow after accepted source exists.

### DEL-05-03 — Fundamental stress recovery module

Files:

- `execution/PKG-05_Loads, Load Cases, and Stress Recovery/1_Working/DEL-05-03_Fundamental stress recovery module/_STATUS.md`
- `execution/PKG-05_Loads, Load Cases, and Stress Recovery/1_Working/DEL-05-03_Fundamental stress recovery module/MEMORY.md`

Required `_STATUS.md` entries:

- Keep `Current State: IN_PROGRESS` and update the date.
- Amend the first `Remaining` bullet only to record that the accepted private dormant exact-annulus kernel has been implemented/refuted. Retain production pressure activation, material authority, topology/defaults, public schema, curved/expansion-joint, threshold and fixture decisions.
- Keep the unchanged public-envelope contract residual unless final work separately changes it.
- If the final native acceptance closes the currently named fresh-native foundation check, remove only that clause. Retain pressure runtime integration, connector treatment, public result-contract work and broader pressure behavior.
- Add one History entry pointing to final acceptance; no dependency or lifecycle promotion.

Required `MEMORY.md` entry: record the private module and declaration hashes, independent oracle/refutation result, representability repair disposition, dormant call-graph check, exact accepted source commit and one final-acceptance pointer. State plainly that kernel qualification is not runtime pressure qualification.

## Conditional consumer-deliverable inventory

Derive this set from `git diff --name-status origin/main...<immutable-candidate>` after all writers finish. Add a concise `MEMORY.md` note only when the named consumer path actually changed and the final acceptance covers its behavior. Do not create a metadata write merely because a path appeared in the preliminary write fence.

| Candidate path family | Owning consumer deliverable metadata |
|---|---|
| `core/project_persistence/**`, `schemas/project_persistence*`, desktop `projectService*` | DEL-02-05 `MEMORY.md`; retain container/migration/compatibility-window Remaining |
| `core/serialization/canonical_json/**`, checked `operation_applier` hash exports, desktop `hashService*` | DEL-08-02 `MEMORY.md`; describe additive checked profile and unchanged existing hash profiles |
| desktop `HistoricalRunContext*`, `ResultsPanel*`, result-history/currentness consumers | DEL-07-05 `MEMORY.md`; retain rotational visualization and upstream ratio sufficiency Remaining |
| desktop `ResultExportPanel*` or `resultExportAdapter*` | DEL-08-04 `MEMORY.md`; retain target-format conversion Remaining |
| desktop `renderableReportInput*` / rendered-report consumption | DEL-08-01 `MEMORY.md`; retain `.opsproj` compatibility-window Remaining |
| desktop `ComparisonPanel*` where designation behavior changes | DEL-14-04 `MEMORY.md`; retain comparison-output schema and suitability holds |
| stress-neutral producer/panel/package paths | DEL-17-06 primary metadata above; do not duplicate in another deliverable |
| pressure kernel private module/declaration | DEL-05-03 primary metadata above |

Generic test/setup changes inherit the deliverable of the behavior they prove. `App.tsx`, `types.ts`, shared fixtures, and `conftest.py` do not by themselves justify extra deliverable MEMORY entries.

## Surfaces that must remain unchanged in this tranche

- Decomposition, SCA-009, DAG-010 and dependency satisfaction rows.
- D-67 ruled bytes and the decision-register disposition.
- Lifecycle headers except date/history wording; no `CHECKING` or `ISSUED` transition.
- Historical 0.1 fixtures/evidence except where D-67 explicitly permits an active version-schema metadata adjustment and frozen originals remain available.
- Production UI, pressure runtime activation, connectors, CAEPIPE and live harness integration.

## Final inputs still required

1. Compatibility manager/writer final return with immutable changed-path inventory, source hashes, focused results, legacy preservation evidence and unresolved risks.
2. Pressure V2 source freeze plus independent oracle refutation over the exact integrated hashes.
3. One immutable integrated candidate commit and complete `origin/main...candidate` diff manifest.
4. Native execution return against that candidate, including save/reopen/currentness and actual 0.2 producer/package evidence.
5. Fresh separate-author review of the complete integrated source/test diff with no unresolved actionable finding.
6. Clean commit-bound complete DEC-025 summary and final affected backchecks.
7. HELP_HUMAN final technical acceptance and handoff, including production-UI readiness disposition and explicit derivative deferrals.
8. Final PR head, actual required CI results and merge result.
