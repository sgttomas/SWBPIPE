# Dependencies: DEL-17-08 GLB/glTF review geometry export

## Coordination Mode
- **Mode:** FULL_GRAPH
- **Graph Authority:** `execution/_DAG/DAG-006/`.
- **Authority Boundary:** `DAG-006` is the approved active graph authority.

## Declared Upstream Dependencies
- `DEL-17-02`
- `DEL-02-01`
- `DEL-07-01`
- `DEL-13-04`
- `DEL-15-01`

## Declared Downstream Dependencies
- No active downstream dependency is declared in this local register unless listed in `Dependencies.csv`; historical DAG-005 extraction wording is superseded by DAG-006 active graph authority.

## Extracted Dependency Register
- **Register:** `Dependencies.csv`
- **Register schema version:** `v3.1`
- **Active rows:** 12
- **Retired rows:** 0
- **DependencyClass counts:** ANCHOR 5; EXECUTION 7
- **Origin counts:** DECLARED 5; EXTRACTED 7

| DependencyID | Class | Direction | Type | Target | Origin | Status | Notes |
|---|---|---|---|---|---|---|---|
| DEP-17-08-001 | ANCHOR | UPSTREAM | OTHER | PKG-17 | EXTRACTED | ACTIVE | Parent package anchor. |
| DEP-17-08-002 | ANCHOR | UPSTREAM | OTHER | SOW-030 | EXTRACTED | ACTIVE | Scope trace anchor. |
| DEP-17-08-003 | ANCHOR | UPSTREAM | OTHER | SOW-074 | EXTRACTED | ACTIVE | Scope trace anchor. |
| DEP-17-08-004 | ANCHOR | UPSTREAM | OTHER | OBJ-009 | EXTRACTED | ACTIVE | Objective trace anchor. |
| DEP-17-08-005 | ANCHOR | UPSTREAM | OTHER | OBJ-017 | EXTRACTED | ACTIVE | Objective trace anchor. |
| DEP-17-08-006 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-17-02 | DECLARED | ACTIVE | Declared edge preserved; also explicit in requirements. |
| DEP-17-08-007 | EXECUTION | UPSTREAM | INTERFACE | DEL-02-01 | DECLARED | ACTIVE | Declared edge preserved; schema/model interface. |
| DEP-17-08-008 | EXECUTION | UPSTREAM | INTERFACE | DEL-07-01 | DECLARED | ACTIVE | Declared edge preserved; no extra semantics inferred. |
| DEP-17-08-009 | EXECUTION | UPSTREAM | INTERFACE | DEL-13-04 | DECLARED | ACTIVE | Declared edge preserved; no extra semantics inferred. |
| DEP-17-08-010 | EXECUTION | UPSTREAM | INTERFACE | DEL-15-01 | DECLARED | ACTIVE | Declared edge preserved; manifest/package context. |
| DEP-17-08-011 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-17-01 | EXTRACTED | ACTIVE | Source-basis authority prerequisite. |
| DEP-17-08-012 | EXECUTION | UPSTREAM | CONSTRAINT | GLTF-2.0 | EXTRACTED | ACTIVE | Public target-format constraint/reference. |

## Run Notes
- **Run timestamp:** 2026-05-18T18:27:07Z
- **TaskSkill:** `dependency-extract`
- **Mode:** `UPDATE`
- **Strictness:** `CONSERVATIVE`
- **Consumer context:** `NONE`
- **Scope:** `DEL-17-08`
- **Run root:** `/Users/ryan/ai-env/projects/chirality-piping/execution`
- **Decomposition path:** `/Users/ryan/ai-env/projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md`
- **Decomposition status:** found and used for target validation/labels.
- **Source document selection:** `SOURCE_DOCS=AUTO`.
- **Anchor document:** `Datasheet.md` selected from AUTO role heuristic.
- **Execution document order:** `Procedure.md`, `Specification.md`, `Guidance.md`, `Datasheet.md` selected from AUTO role heuristic.
- **Reference resolver:** `_REFERENCES.md` read; used to resolve `GLTF-2.0` URL.
- **Required maturity default:** `INITIALIZED` for execution edges; `NOT_APPLICABLE` for anchors.
- **Parent anchor check:** exactly one ACTIVE `IMPLEMENTS_NODE` anchor; no `FLOATING_NODE` or `AMBIGUOUS_ANCHOR` warning.
- **Conservative target policy:** declared-only targets were preserved as declared rows; no downstream target rows were added because no downstream deliverable ID was explicit.
- **Warnings:** none.

## Run History
- 2026-05-18T18:27:07Z - TASK dependency-extract; mode `UPDATE`; strictness `CONSERVATIVE`; decomposition found at `/Users/ryan/ai-env/projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md`; warnings none; ACTIVE rows 12 (ANCHOR 5, EXECUTION 7).

## Lifecycle Summary
- **ACTIVE:** 12
- **RETIRED:** 0
- **Closure state breakdown:** `NOT_APPLICABLE` 5; `TBD` 7
- **Required maturity breakdown:** `NOT_APPLICABLE` 5; `INITIALIZED` 7
- **Declared edges preserved:** 5 of 5
- **Unresolved targets invented:** 0

## Notes
- Candidate rows, if any, remain non-gating until explicit promotion and graph revalidation.
- Dependencies do not authorize implementation, lifecycle promotion, release claims, or professional claims by themselves.
