# Dependencies: DEL-17-07 Conservative PCF subset exporter

## Coordination Mode
- **Mode:** FULL_GRAPH
- **Graph Authority:** `execution/_DAG/DAG-005/`.
- **Authority Boundary:** `DAG-005` is the approved active graph authority.

## Declared Upstream Dependencies
- `DEL-17-02`
- `DEL-03-02`
- `DEL-13-04`
- `DEL-15-02`

## Declared Downstream Dependencies
- To be materialized by DAG-005 and later dependency-extract.

## Extracted Dependency Register
- **Register:** `Dependencies.csv`
- **RegisterSchemaVersion:** `v3.1`
- **Last refreshed:** 2026-05-18 12:26 -0600
- **Mode:** `UPDATE`
- **Strictness:** `CONSERVATIVE`
- **Consumer context:** `NONE`
- **Active rows:** 14
- **Retired rows:** 0

| DependencyID | Class | Direction | Type | Target | Origin | Status |
|---|---|---|---|---|---|---|
| DEP-17-07-001 | ANCHOR | UPSTREAM | OTHER | PKG-17 Export Format Interoperability | EXTRACTED | ACTIVE |
| DEP-17-07-002 | ANCHOR | UPSTREAM | OTHER | SOW-030 Public APIs/plugins/import-export adapters | EXTRACTED | ACTIVE |
| DEP-17-07-003 | ANCHOR | UPSTREAM | OTHER | SOW-074 Schema-compliant handoff packages | EXTRACTED | ACTIVE |
| DEP-17-07-004 | ANCHOR | UPSTREAM | OTHER | OBJ-009 Interoperability and governance boundaries | EXTRACTED | ACTIVE |
| DEP-17-07-005 | ANCHOR | UPSTREAM | OTHER | OBJ-017 Traceable handoff packages | EXTRACTED | ACTIVE |
| DEP-17-07-006 | ANCHOR | UPSTREAM | OTHER | OBJ-018 Professional and IP boundaries | EXTRACTED | ACTIVE |
| DEP-17-07-007 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-17-02 Export package/profile/stable-ID map contracts | DECLARED | ACTIVE |
| DEP-17-07-008 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-03-02 Pipe section and component library schema | DECLARED | ACTIVE |
| DEP-17-07-009 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-13-04 Physical-to-analytical transformation contract | DECLARED | ACTIVE |
| DEP-17-07-010 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-15-02 Target mapping and unsupported-behavior contract | DECLARED | ACTIVE |
| DEP-17-07-011 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-17-01 CAEPIPE and export-format source basis | EXTRACTED | ACTIVE |
| DEP-17-07-012 | EXECUTION | UPSTREAM | PREREQUISITE | PLAN-EXPORT-INTEROP | EXTRACTED | ACTIVE |
| DEP-17-07-013 | EXECUTION | UPSTREAM | PREREQUISITE | CAEPIPE-PCF | EXTRACTED | ACTIVE |
| DEP-17-07-014 | EXECUTION | UPSTREAM | CONSTRAINT | IP-DATA | EXTRACTED | ACTIVE |

## Run Notes
- Defaults applied: `SOURCE_DOCS=AUTO`, `DOC_ROLE_MAP=DEFAULT`, `ANCHOR_DOC=AUTO`, `EXECUTION_DOC_ORDER=AUTO`.
- Chosen anchor doc: `Datasheet.md`.
- Chosen execution doc order: `Procedure.md`, `Specification.md`, `Guidance.md`, `Datasheet.md`.
- Decomposition path used: `/Users/ryan/ai-env/projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md`.
- `_REFERENCES.md` was used to resolve document targets for `PLAN-EXPORT-INTEROP`, `CAEPIPE-PCF`, and `IP-DATA`.
- Existing `Dependencies.csv` was absent; this run created the v3.1 register.
- Declared upstream dependencies from the previous `_DEPENDENCIES.md` were preserved as `Origin=DECLARED`: `DEL-17-02`, `DEL-03-02`, `DEL-13-04`, `DEL-15-02`.
- Conservative unresolved-target posture: no unresolved deliverable IDs were invented; non-deliverable document targets use explicit reference IDs and locations from local references.
- No downstream dependencies were extracted because the four documents do not explicitly name downstream consumers.
- Parent anchor check: exactly one active `IMPLEMENTS_NODE` anchor.
- Schema validation passed with command: `python3 tools/validation/validate_dependencies_schema.py "<DeliverablePath>/Dependencies.csv"`.
- Enum validation passed for emitted enum values.
- `[WARNING] ID_FORMAT_TOOL_STALE`: `tools/validation/validate_id_format.sh` expects legacy `PKG-###` / `DEL-###-##` formats and rejects active decomposition IDs such as `PKG-17` and `DEL-17-07`; active IDs were therefore preserved from the decomposition instead of rewritten to satisfy the stale tool pattern.

## Run History
- 2026-05-18 12:26 -0600 — TASK + `dependency-extract`; `MODE=UPDATE`; `STRICTNESS=CONSERVATIVE`; decomposition path present; warnings: `[WARNING] ID_FORMAT_TOOL_STALE`; ACTIVE counts: 6 ANCHOR, 8 EXECUTION, 14 total; RETIRED count: 0.

## Lifecycle Summary

| Status | Count |
|---|---:|
| ACTIVE | 14 |
| RETIRED | 0 |

| SatisfactionStatus | Count |
|---|---:|
| NOT_APPLICABLE | 6 |
| TBD | 8 |

## Notes
- Candidate rows, if any, remain non-gating until explicit promotion and graph revalidation.
- Dependencies do not authorize implementation, lifecycle promotion, release claims, or professional claims by themselves.
