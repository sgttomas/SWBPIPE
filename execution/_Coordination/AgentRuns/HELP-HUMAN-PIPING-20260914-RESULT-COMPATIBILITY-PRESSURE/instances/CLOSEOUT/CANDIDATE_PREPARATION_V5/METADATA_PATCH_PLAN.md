# Unapplied metadata patch plan V5

Status: **UNAPPLIED**. Apply only after `<FINAL_NATIVE_PATH_SHA256>`, `<FULL_DEC025_PATH_SHA256>` and `<ROOT_COMBINED_ACCEPTANCE_PATH_SHA256>` exist, bind source `8ad37207cf088025623aa1e777a97a6fcb802f48`, and carry passing/accepting dispositions. Before applying, require every target to match its before-hash in `_run_records/CONTEXT_MANIFEST.json`; stop and rebase this plan on any mismatch.

Accepted source basis:

- Source: `8ad37207cf088025623aa1e777a97a6fcb802f48`
- Release: `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260914-RESULT-COMPATIBILITY-PRESSURE/instances/ROOT/_run_records/FINAL_SOURCE_RELEASE_V6.json`, SHA-256 `ad18bceaa8e6d958a9182d94f5f6095619fe197eb227c1234edae1abea56dc72`
- Review: same run `instances/ROOT/_run_records/FINAL_REVIEW_V6/REVIEW_RETURN_V6.json`, SHA-256 `a0589d0739907f790baa1020bf868a17da5f82a29bd1251e9555a5457fab6adc`, `PASS_SOURCE_REVIEW`
- Compatibility: same run `instances/ROOT/_run_records/FINAL_COMPATIBILITY_FANIN_V4.json`, SHA-256 `2721ca0797afa0794de43b3e3be1d54934b3ddcb85d7c44130a691ad213813d6`
- Pressure: same run `instances/ROOT/PRESSURE_ACCEPTANCE_V1.json`, SHA-256 `0b197f78e8f971a1e44c0111beb0b8d09c3e2ef280aaf370b16da112bf326067`

All three primary `_STATUS.md` files retain `**Current State:** IN_PROGRESS` and change `**Last Updated:**` to the actual acceptance date, `2026-09-15` if accepted that day.

## DEL-14-02

In `execution/PKG-14_Model States, Analysis Runs, and Comparison/1_Working/DEL-14-02_Analysis run records/_STATUS.md`, preserve the first Remaining bullet and replace only the second with:

> - Preserve PDU-033 for producer-supplied physical dimensions and other governed result metadata absent from received rows. The accepted foundation implements exact-version analysis record 0.2 under checked profile `openpipestress_jcs_ijson_v1`, retains both historical 0.1 profiles, records immutable rule-check revisions, and scopes legacy interpretation. Raw source rows remain unchanged; semantic interpretation beside them does not create missing producer evidence, and explicit `unverifiable` states do not close PDU-033.

Insert as the first History entry:

> - 2026-09-15 - Result compatibility foundation accepted at `8ad37207cf088025623aa1e777a97a6fcb802f48`; PDU-033 remains. Lifecycle and formal dependency dispositions are unchanged. Acceptance: `<ROOT_COMBINED_ACCEPTANCE_PATH_SHA256>`.

Append to `execution/PKG-14_Model States, Analysis Runs, and Comparison/1_Working/DEL-14-02_Analysis run records/MEMORY.md`:

> ## 2026-09-15 — Result compatibility foundation
>
> Accepted source `8ad37207cf088025623aa1e777a97a6fcb802f48` implements strict analysis-record 0.2 under checked profile `openpipestress_jcs_ijson_v1`, preserves historical 0.1 profiles and received source rows, separates mechanics-run identity from immutable record-revision identity, and preserves truthful Current/Historical and `unverifiable` states. PDU-033 and producer diagnostic breadth remain open. Source release: `instances/ROOT/_run_records/FINAL_SOURCE_RELEASE_V6.json` (SHA-256 `ad18bceaa8e6d958a9182d94f5f6095619fe197eb227c1234edae1abea56dc72`). Review: `instances/ROOT/_run_records/FINAL_REVIEW_V6/REVIEW_RETURN_V6.json` (SHA-256 `a0589d0739907f790baa1020bf868a17da5f82a29bd1251e9555a5457fab6adc`). Native: `<FINAL_NATIVE_PATH_SHA256>`. Acceptance: `<ROOT_COMBINED_ACCEPTANCE_PATH_SHA256>`. Lifecycle and formal dependency dispositions remain unchanged.

## DEL-17-06

In `execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-06_Stress-neutral CSV JSON package/_STATUS.md`, replace the sole Remaining bullet with these two bullets:

> - For the accepted 830-row sparse fixture only, resolve or formally disposition the two explicitly withheld unit-preservation witnesses for diagnostic-work rows `result:nonlinear-support:free-dof-work-residual` and `result:loadcase:load-L-200:nonlinear-support:free-dof-work-residual`. The strict 0.2 package retains every raw row; this fixture-scoped hold is not a dropped-row, schema-failure, universal-row-count, or other-producer claim.
> - Regenerate `_SEMANTIC.md` and `_SEMANTIC_LENSING.md` against the accepted post-D-67/CLM-042 state, or formally retire them from current use; their 2026-05-18 statements that exact schema/hash partitioning is unresolved are superseded and must not be used as current evidence. Refresh `_DEPENDENCIES.md` against DAG-010 through the owning dependency workflow; `Dependencies.csv` and DAG authority remain unchanged until then.

Insert as the first History entry:

> - 2026-09-15 - Bounded versioned 0.2 stress-neutral export foundation accepted at `8ad37207cf088025623aa1e777a97a6fcb802f48`; the accepted 830-row fixture retains two named diagnostic-work witness holds. Deferred semantic/lensing and DAG-010 dependency-summary work remains with its owning workflows. Lifecycle and dependency satisfaction are unchanged. Acceptance: `<ROOT_COMBINED_ACCEPTANCE_PATH_SHA256>`.

Append to `execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-06_Stress-neutral CSV JSON package/MEMORY.md`:

> ## 2026-09-15 — Versioned stress-neutral 0.2 foundation
>
> Historical 0.1 retains its Python sorted-key compact ASCII-escaped serializer and labels. Strict 0.2 uses checked Rust profile `openpipestress_jcs_ijson_v1` for JSON member/package hashes, keeps normalized CSV separately labelled, distinguishes received carrier claims from computed derivative claims, preserves raw result rows, and emits the accepted materializable member set. The accepted 830-row sparse fixture retains two explicitly withheld diagnostic-work witnesses; that count and withholding do not generalize to other producers. Public producer DTO rows do not guarantee a `dimension` field: 0.2 qualification uses the accepted semantic contract without inserting or fabricating dimensions in raw rows, while legacy dimension enrichment remains restricted to 0.1. Source: `8ad37207cf088025623aa1e777a97a6fcb802f48`. Source release: `instances/ROOT/_run_records/FINAL_SOURCE_RELEASE_V6.json` (SHA-256 `ad18bceaa8e6d958a9182d94f5f6095619fe197eb227c1234edae1abea56dc72`). Review: `instances/ROOT/_run_records/FINAL_REVIEW_V6/REVIEW_RETURN_V6.json` (SHA-256 `a0589d0739907f790baa1020bf868a17da5f82a29bd1251e9555a5457fab6adc`). Native/materialization: `<FINAL_NATIVE_PATH_SHA256>`. Acceptance: `<ROOT_COMBINED_ACCEPTANCE_PATH_SHA256>`. `_SEMANTIC.md`, `_SEMANTIC_LENSING.md`, and `_DEPENDENCIES.md` remain deferred derivatives and are not current authority.

Append to `execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-06_Stress-neutral CSV JSON package/_REFERENCES.md`:

> - **D-67:** `execution/_Coordination/_DECISIONS/D-67_RULING_2026-09-14.md` — owner-adopted versioned 0.2 hash-contract amendment preserving 0.1.
> - **Foundation source review:** `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260914-RESULT-COMPATIBILITY-PRESSURE/instances/ROOT/_run_records/FINAL_REVIEW_V6/REVIEW_RETURN_V6.json` — cumulative review of source `8ad37207cf088025623aa1e777a97a6fcb802f48`.
> - **Foundation final acceptance:** `<ROOT_COMBINED_ACCEPTANCE_PATH_SHA256>` — bounded compatibility/pressure/native acceptance; no lifecycle or dependency promotion.

## DEL-05-03

In `execution/PKG-05_Loads, Load Cases, and Stress Recovery/1_Working/DEL-05-03_Fundamental stress recovery module/_STATUS.md`, replace the three Remaining bullets with:

> - The private dormant exact-annulus pressure kernel is implemented in accepted source `8ad37207cf088025623aa1e777a97a6fcb802f48` and qualified against the V3 independent refutation. Production pressure activation remains open with Poisson/material authority, closure topology/defaults, typed public force/result compatibility, curved and expansion-joint treatment, thresholds, and verification fixtures. No lifecycle, dependency, or runtime-pressure claim is implied.
> - The 2026-09-09 endpoint section-cut, curved-frame metadata, and genuine-pressure eligibility repair remains accepted. Its preserved 2026-09-09 raw-envelope observation reported 196 unchanged baseline public result-contract errors and zero errors on the repair-affected rows; 196 is historical snapshot evidence, not a new or universal count. Versioned public-result-contract work remains open.
> - Broader pressure behavior, connector treatment, and public result-contract work remain open. No lifecycle or DAG promotion is recorded.

Insert as the first History entry:

> - 2026-09-15 - Private dormant exact-annulus pressure-kernel foundation accepted at `8ad37207cf088025623aa1e777a97a6fcb802f48`; production pressure, connector and public-result-contract work remain open. Lifecycle and formal dependency dispositions are unchanged. Acceptance: `<ROOT_COMBINED_ACCEPTANCE_PATH_SHA256>`.

Append to `execution/PKG-05_Loads, Load Cases, and Stress Recovery/1_Working/DEL-05-03_Fundamental stress recovery module/MEMORY.md`:

> ## 2026-09-15 — Private pressure-kernel foundation
>
> Accepted source `8ad37207cf088025623aa1e777a97a6fcb802f48` binds `core/product_physics/src/pressure_exact.rs` SHA-256 `baa83a5a62ea34d0677a5c5c5d6cd2fd2f977b6b62a5e2d4ddd06fa18951abc0` and its private declaration in `core/product_physics/src/lib.rs` SHA-256 `aa91613c48346654dc7e8b110f22fc74cf9578fb16f21ae9c40cec863b910f15`. `instances/ROOT/PRESSURE_ACCEPTANCE_V1.json` (SHA-256 `0b197f78e8f971a1e44c0111beb0b8d09c3e2ef280aaf370b16da112bf326067`) and `instances/PRESSURE_ORACLE/REFUTATION_V3/ADJUDICATED_SUMMARY.json` (SHA-256 `80b513905c0dc737f16ec91d7a01afda1fc61d263b6898774291c6ee0617c75a`) accept the representability repair and dormant call graph. This is private kernel qualification, not runtime pressure qualification. The 196-error statement remains a preserved 2026-09-09 raw-envelope observation, not a current universal measurement. Final acceptance: `<ROOT_COMBINED_ACCEPTANCE_PATH_SHA256>`. Lifecycle and formal dependency dispositions remain unchanged.

## Six conditional consumer MEMORY appends

Append the following only after final native and combined acceptance. No other deliverable receives a note, and specifically no DEL-14-04 note is added.

1. `execution/PKG-02_Domain Model, Units, and Core Schemas/1_Working/DEL-02-05_Project persistence and round-trip serialization/MEMORY.md`

   > ## 2026-09-15 — Result compatibility foundation
   >
   > Accepted source `8ad37207cf088025623aa1e777a97a6fcb802f48` persists distinct 0.2 record revisions by complete record checksum and adopts validated native model normalization while preserving historical profile bytes and claims. Container/migration compatibility-window work remains open. Acceptance: `<ROOT_COMBINED_ACCEPTANCE_PATH_SHA256>`.

2. `execution/PKG-08_Reporting, Audit, and Reproducibility/1_Working/DEL-08-02_Audit manifest and model hash/MEMORY.md`

   > ## 2026-09-15 — Checked result-record profile
   >
   > Accepted source `8ad37207cf088025623aa1e777a97a6fcb802f48` adds checked profile `openpipestress_jcs_ijson_v1` for bounded 0.2 analysis/stress payloads. Existing model, persistence, input-manifest and operation hash profiles remain unchanged. Acceptance: `<ROOT_COMBINED_ACCEPTANCE_PATH_SHA256>`.

3. `execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-05_Results viewer/MEMORY.md`

   > ## 2026-09-15 — Version-aware result designation
   >
   > Accepted source `8ad37207cf088025623aa1e777a97a6fcb802f48` preserves profile-aware Current/Historical display and scoped legacy match/mismatch/unverifiable findings; 0.2 record-revision identity remains distinct from mechanics-run identity. Rotational visualization and upstream criteria sufficiency remain open. `PRODUCTION_UI_FOUNDATION_CANDIDATE.md` is next-tranche preparation, not production UI implementation or personal visual approval. Acceptance: `<ROOT_COMBINED_ACCEPTANCE_PATH_SHA256>`.

4. `execution/PKG-08_Reporting, Audit, and Reproducibility/1_Working/DEL-08-04_Result export format/MEMORY.md`

   > ## 2026-09-15 — Versioned result export foundation
   >
   > Accepted source `8ad37207cf088025623aa1e777a97a6fcb802f48` adds strict source-preserving 0.2 result derivatives and guarded local-private browser/native delivery while retaining 0.1 dispatch. For 0.2, raw rows remain unchanged and semantic qualification supplies derivative interpretation; legacy dimension binding remains 0.1-only. Target-format numerical conversion remains open. Acceptance: `<ROOT_COMBINED_ACCEPTANCE_PATH_SHA256>`; native evidence: `<FINAL_NATIVE_PATH_SHA256>`.

5. `execution/PKG-08_Reporting, Audit, and Reproducibility/1_Working/DEL-08-01_Calculation report generator/MEMORY.md`

   > ## 2026-09-15 — Versioned report result selection
   >
   > Accepted source `8ad37207cf088025623aa1e777a97a6fcb802f48` makes report consumers select the received 0.2 result and retained 0.1 result envelope under their respective profiles, with absent producer dimensions disclosed. The `.opsproj` compatibility window remains open. Acceptance: `<ROOT_COMBINED_ACCEPTANCE_PATH_SHA256>`.

6. `execution/PKG-12_Security, Privacy, and Private Data Handling/1_Working/DEL-12-02_Private data redaction and export controls/MEMORY.md`

   > ## 2026-09-15 — Controlled stress export route
   >
   > Accepted source `8ad37207cf088025623aa1e777a97a6fcb802f48` admits the controlled stress JSON route and paired native filename only after explicit local-private intent and existing screening. This adds no general route, privacy-policy, public-transport, or local-first change. Acceptance: `<ROOT_COMBINED_ACCEPTANCE_PATH_SHA256>`; native evidence: `<FINAL_NATIVE_PATH_SHA256>`.

## Application boundary

After these edits, lifecycle stays `IN_PROGRESS`; dependencies, DAG-010, SCA-009, decomposition 0.12 and R5 are unchanged. Historical History text remains untouched. The removed current-tranche clauses are limited to native validation, DEC-025 and CHANGE/Git pending language that has become obsolete at application time. Hosted CI, PR, merge and product release remain distinct and are recorded later through the PR/Git state and final local evidence rather than reintroduced as deliverable Remaining items.
