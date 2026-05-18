---
amendment_id: SCA-002
doc_kind: scope_change.review_handoff
package_role: snapshot / handoff artifact
created: 2026-05-03
status: corrective_action_applied_and_accepted_for_downstream_refresh_planning
---

# SCA-002 Review Handoff To Next Instance

## Purpose

This handoff records a hindsight review of the SCA-002 implementation. The conceptual amendment was sound, but the revision 0.5 package/register state needed correction before `ORCHESTRATOR`, `PREPARATION`, or any Type 2 implementation could consume it.

## Corrective Cleanup Applied

This handoff has been actioned. The cleanup corrected package-local direct scope mappings, reciprocal scope-ledger mappings, package-table assignments for `PKG-01`, `PKG-02`, and `PKG-07`, the decomposition latest pointer, and canonical supersession CSV schema. Local validation now passes for counts, deliverable/package coupling, package-local scope coverage, reciprocal scope-ledger coverage, package table parity, and markdown/register deliverable parity.

Downstream use remains blocked until the human accepts the corrected revision 0.5 basis or explicitly authorizes an owning workflow to refresh downstream artifacts.

## Acceptance Recorded

The human project authority accepted corrected SCA-002 for downstream refresh planning on 2026-05-03. Downstream refresh planning is now routed through `plans/SCA-002_DOWNSTREAM_REFRESH_PLAN.md`; direct Type 2 dispatch remains blocked until refreshed coordination surfaces are accepted.

The next instance should start from the normal bootstrap files, then read:

1. `execution/_Decomposition/SOFTWARE_DECOMP.md`
2. `docs/_Registers/ScopeLedger.csv`
3. `docs/_Registers/Deliverables.csv`
4. `docs/_Registers/ContextBudgetQA.csv`
5. `execution/_ScopeChange/SCA-002_2026-05-02_1854/*`
6. this file

## Current State

- SCA-002 updated `execution/_Decomposition/SOFTWARE_DECOMP.md` to revision `0.5`.
- SCA-002 added `SOW-064` through `SOW-076`, `OBJ-014` through `OBJ-018`, `PKG-13` through `PKG-16`, and 19 deliverables.
- Register counts validate mechanically: 76 scope rows, 92 deliverables, 92 context-budget rows, context counts `S=9, M=66, L=17, XL=0`.
- Downstream coordination, DAG, dependency, lifecycle, implementation evidence, and package-local production artifacts remain stale by design.

## Findings Fixed By Cleanup

### 1. Package-Local Scope Rule Violations

Several new deliverables claim `CoversScopeItems` / `Scope Items` assigned to other packages. This violates the SOFTWARE_DECOMP invariant that deliverables are single-domain and belong to exactly one package.

Observed cross-package claims:

|Deliverable|Package|Cross-package scope claims to remove or remap|
|---|---|---|
|`DEL-07-08`|`PKG-07`|`SOW-069` (`PKG-16`), `SOW-071` (`PKG-14`), `SOW-073` (`PKG-14`)|
|`DEL-08-06`|`PKG-08`|`SOW-075` (`PKG-15`), `SOW-073` (`PKG-14`), `SOW-074` (`PKG-15`)|
|`DEL-13-01`|`PKG-13`|`SOW-065` (`PKG-02`)|
|`DEL-13-04`|`PKG-13`|`SOW-065` (`PKG-02`)|
|`DEL-14-01`|`PKG-14`|`SOW-065` (`PKG-02`)|
|`DEL-15-04`|`PKG-15`|`SOW-064` (`PKG-01`)|
|`DEL-16-02`|`PKG-16`|`SOW-068` (`PKG-13`)|
|`DEL-16-04`|`PKG-16`|`SOW-064` (`PKG-01`)|

Recommended correction: trim each deliverable's direct scope mapping to package-local scope only. Preserve cross-package relationships as dependencies, objective support, notes, or downstream handoff obligations, not as direct scope coverage.

### 2. Package Table Incomplete

The package table in `execution/_Decomposition/SOFTWARE_DECOMP.md` omits new scope assigned to pre-existing packages:

|Package|Missing assigned scope item|
|---|---|
|`PKG-01`|`SOW-064`|
|`PKG-02`|`SOW-065`|
|`PKG-07`|`SOW-076`|

Recommended correction: update the package table `Assigned Scope Items` cells to match `docs/_Registers/ScopeLedger.csv` after resolving the scope-locality issue.

### 3. Decomposition Latest Pointer Corrected

`execution/_Decomposition/_LATEST.md` previously pointed at a retired docs-side revision. It now points to `execution/_Decomposition/SOFTWARE_DECOMP.md` revision `0.5`.

### 4. Supersession CSV Schema Corrected

`tools/coordination/accumulate_supersession_map.py` expects canonical supersession columns:

```text
AmendmentID, DecisionID, SupersededAuthorityRole, SupersededAuthorityPath,
SupersededAuthorityRef, SupersededFactKey, SupersededFactTextOrValue,
OverrideType, ReplacementFactTextOrValue, AppliesToRoots, AppliesToFacilities,
AppliesToSections, Notes
```

SCA-002 initially wrote `Supersession_Delta.csv` and `Supersession_Map.csv` with narrative columns:

```text
AmendmentID, BindingType, AuthorityDocument, AuthorityReference,
OriginalValue, ReplacementValue, ScopeAffected, Notes
```

Correction applied: SCA-002 supersession files now use the canonical schema and the map has been regenerated/check-validated with the accumulator.

### 5. Gate/Approval Wording Corrected

`Brief.md` now records corrective cleanup and SCA-002 human acceptance. The explicit acceptance record is `ACCEPTANCE_RECORD.md`.

## Cleanup Plan Applied

1. Treat this as a corrective SCA-002 cleanup, not downstream implementation.
2. Patch `execution/_Decomposition/SOFTWARE_DECOMP.md` and the two affected registers so every deliverable only covers package-local scope.
3. Update package table assigned-scope cells for `PKG-01`, `PKG-02`, and `PKG-07`.
4. Update `execution/_Decomposition/_LATEST.md` after cleanup.
5. Normalize `Supersession_Delta.csv` / `Supersession_Map.csv` to the canonical schema expected by `tools/coordination/accumulate_supersession_map.py`, or add a clearly named canonical replacement and mark the current files narrative.
6. Update SCA-002 handoff state to say corrective cleanup was applied and full downstream use remains blocked until human acceptance.
7. Re-run validation checks:

```bash
python3 - <<'PY'
import csv, re
from pathlib import Path
root = Path('.')
ledger = {r['ScopeItemID']: r['PackageID'] for r in csv.DictReader((root/'docs/_Registers/ScopeLedger.csv').open())}
errors = []
for r in csv.DictReader((root/'docs/_Registers/Deliverables.csv').open()):
    did = r['DeliverableID']
    pkg = r['PackageID']
    m = re.fullmatch(r'DEL-(\d{2})-\d{2}', did)
    if not m or pkg != f'PKG-{m.group(1)}':
        errors.append(f'bad deliverable/package coupling: {did} -> {pkg}')
    for sid in [x.strip() for x in r['CoversScopeItems'].split(',') if x.strip()]:
        if ledger.get(sid) and ledger[sid] != pkg:
            errors.append(f'cross-package scope: {did} ({pkg}) covers {sid} ({ledger[sid]})')
print('\\n'.join(errors) or 'OK')
raise SystemExit(1 if errors else 0)
PY
```

Also re-check counts:

```text
ScopeLedger rows: 76
Deliverables rows: 92
ContextBudgetQA rows: 92
Context envelope counts: S=9, M=66, L=17, XL=0
```

## Hard Boundary

Do not refresh downstream package-local production docs, DAG files, implementation evidence, lifecycle states, dependency registers, or dispatch briefs as part of this cleanup unless the human explicitly changes the scope. The needed action is to make the SCA-002 decomposition/register truth coherent and safe for later orchestration.
