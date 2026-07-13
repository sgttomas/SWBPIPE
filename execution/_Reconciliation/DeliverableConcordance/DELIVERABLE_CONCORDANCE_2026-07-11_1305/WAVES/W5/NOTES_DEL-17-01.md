# NOTES — DEL-17-01 CAEPIPE and export-format source basis (R2 W5)

Frozen evidence: `main` @
`551f84ef6be656f1603ce0acfa5e3935aa9683c7`. Deliverable:
PKG-17 / DEL-17-01, lifecycle `IN_PROGRESS`. Role/model: **GPT-5
deliverable-grained owning pilot**.

Unqualified deliverable documents refer to
`execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-01_CAEPIPE and export-format source basis/`
at the frozen SHA; other paths are relative to `projects/chirality-piping/`.

## Census and histograms

The exact adopted 20-column ledger contains **30 rows**.

| ClaimType | Count |
|---|---:|
| REQUIREMENT | 11 |
| ACCEPTANCE | 6 |
| EXCLUSION | 1 |
| DECLARED_STATE | 6 |
| REMAINING_WORK | 6 |
| **Total** | **30** |

Disposition is ALIGNED 29 / STALE_SETUP_SPECIFICATION 1; all confidence
values are HIGH. AuthorityNeeded is NO 23 / OWNER 7. SourceReliability is UNVERIFIED 24 /
NOT_APPLICABLE 6. All 30 rows are mechanically non-selectable.

REQ IDs preserve the Specification's non-contiguous source labels by mapping
them to contiguous ledger IDs: 001, 002, 002A, 003, 004, 005, and 010–014
become REQ-001 through REQ-011. The declaration census is the four-document
kit plus `_STATUS.md` and `MEMORY.md`; no deliverable-owned README exists.
Six ACCEPTANCE rows preserve the six distinct acceptance bullets.

The six source-basis TBDs are six REMAINING_WORK rows. The seven dossier
questions are not double-counted: CQ-001/CQ-007 both map to target-version
TBD-001, and CQ-004/CQ-005 both map to harness/parser TBD-004. These formal
TBDs are not `_STATUS.md` remaining items, so their RecordedRemaining and gate
cells remain `NONE_RECORDED`; closure authority is routed OWNER without
selecting work under D-41.

## Self-flags for package fan-in

- **DECL-004 — STALE_SETUP_SPECIFICATION, OWNER.** Procedure displays
  repository-root `tools/validation/...` commands beside project-root
  `execution/...` target paths. No single cwd resolves both bases: repository
  root needs a `projects/chirality-piping/execution/...` target, while project
  root has no local `tools/validation/check_*`. Refresh to one executable path
  basis is R5-only.
- **REQ-002/003 and ACC-002 — source reliability grain.** D-41 verified the
  frozen citation structure and admission boundaries, not live reachability or
  present content of external CAEPIPE/Khronos pages. The rows remain
  UNVERIFIED and make no external compatibility claim.
- **REQ-006 — downstream-consumption breadth.** Frozen downstream kits and
  run records visibly cite DEL-17-01 and carry relevant TBDs, but the rule is
  target-relevant rather than a claim that every target consumes every source.
- **REQ-007/008 and ACC-005 — bounded content review.** Negative boundary
  review is not legal clearance, redistribution advice, or security assurance.
- **REM-001..006 — intentionally open source-basis controls.** Their ALIGNED
  disposition means they are correctly visible and carried, not that their
  underlying target choices have been answered. OWNER denotes the required
  support/project/profile decision route.
- **DECL-005 — bootstrap only.** The sole `_STATUS.md` remaining item is the
  byte-exact D-41 bootstrap, recorded only on the status declaration and
  excluded from residual, gate, and selectability analysis.

## Verification and addendum-9 containment

Re-executed in the frozen worktree with `PYTHONDONTWRITEBYTECODE=1` and
external `PYTHONPYCACHEPREFIX=/tmp/d41-del1701-pycache`:

- root `tools/validation/check_four_documents.sh <DEL-17-01>` — PASS.
- root `tools/validation/check_min_viable_fileset.sh <DEL-17-01>` — PASS.
- `projects/chirality-piping/tools/validation/validate_dependencies_schema.py <DEL-17-01>/Dependencies.csv`
  — VALID, 29 columns, 16 rows.
- root `tools/validation/validate_semantic_matrix.py <DEL-17-01>` — VALID.
- root `tools/validation/validate_lens_register.py <DEL-17-01>` — VALID.

The Procedure's mixed command path bases are encoded as
STALE_SETUP_SPECIFICATION on DECL-004. The intended checks remain reproducible
when invoked from repository root with the full
`projects/chirality-piping/execution/...` target; that successful workaround
does not make the displayed command block executable as written.

No pytest or Cargo command and no in-tree `py_compile` ran. Ignored-aware
porcelain before and after contained exactly the six known allow-listed paths
and no seventh path. Tracked porcelain remained empty; the frozen worktree was
not modified or cleaned.

## Cross-ledger and R3 risks

1. **Source authority versus implementation:** DEL-17-01 owns admitted-source
   and claim-boundary records; later DEL-17 deliverables own schemas, writers,
   parsers, harnesses, packages, SDKs, and target-specific verification.
2. **External-source currentness:** source IDs and headings are frozen evidence
   pointers. R3 must not upgrade them into independently revalidated vendor
   behavior or target compatibility.
3. **TBD carryforward:** six source-basis TBDs recur across downstream
   ledgers. Deduplicate by TBD ID and affected decision rather than counting
   each dossier question and each consumer copy as a new defect.
4. **CAEPIPE evidence posture:** optional user-owned executable runs and parsed
   results remain regression/handoff evidence, never professional acceptance
   or formal solver validation.
5. **PCF/glTF roles:** PCF is conservative interoperability with translator
   caveats; GLB/glTF is review geometry. Neither is a solver-fidelity proof.
6. **Dependency state:** seven PENDING dependency satisfaction cells authorize
   no dependency, DAG, lifecycle, or source-authority edits.
7. **Tool path grain:** root validators passed with a corrected target basis,
   while Procedure remains stale at command-path grain. R3 should not infer
   failure of the substantive document checks or erase the reproducibility
   defect.

## Fences

All dispositions are agent judgments, never human, owner, vendor,
engineering, professional, security, legal, validation, compatibility, or
release rulings. Writes were limited to this CSV and notes. No product,
deliverable, lifecycle, status, dependency, DAG, register, review, source
admission, decision, R4, or R5 surface changed. No approval, certification,
sealing, authentication, code-compliance, professional reliance, legal
clearance, security assurance, target compatibility, or external validation
claim is made.
