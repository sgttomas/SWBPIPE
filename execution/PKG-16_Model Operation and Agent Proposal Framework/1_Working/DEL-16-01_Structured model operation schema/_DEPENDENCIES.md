# Dependencies: DEL-16-01 Structured model operation schema

## Coordination Mode
- **Mode:** FULL_GRAPH
- **Graph Authority:** `execution/_DAG/DAG-007/` is the current approved canonical graph authority.
- **Authority Boundary:** Candidate/non-gating edges are not represented through `Status=CANDIDATE` in current canonical registers.

## Declared Upstream Dependencies
- None recorded.

## Declared Downstream Dependencies
- None recorded.

## Extracted Dependency Register
- **Local Register:** `Dependencies.csv`
- **Register schema version:** `v3.1`
- **Semantic refresh:** 2026-06-16, MODE=UPDATE, STRICTNESS=CONSERVATIVE, CONSUMER_CONTEXT=RECONCILIATION.
- **Rows:** 16 total; 16 ACTIVE; 0 RETIRED.
- **Classes:** ANCHOR=2; EXECUTION=14.
- **PKG-00 architecture-basis rows reviewed:** 7 reviewed; 0 changed; 0 retired.

| Class | Type | Count |
|---|---:|---:|
| ANCHOR | OTHER | 2 |
| EXECUTION | CONSTRAINT | 1 |
| EXECUTION | ENABLES | 3 |
| EXECUTION | INTERFACE | 3 |
| EXECUTION | PREREQUISITE | 7 |

## Run Notes
- Defaults recorded: `SOURCE_DOCS=AUTO`, `DOC_ROLE_MAP=DEFAULT`, `ANCHOR_DOC=AUTO`, `EXECUTION_DOC_ORDER=AUTO`.
- Source documents reviewed: `_CONTEXT.md`, `Datasheet.md`, `Specification.md`, `Procedure.md`, `Guidance.md`, `_REFERENCES.md`, existing `Dependencies.csv`, existing `_DEPENDENCIES.md`, and `execution/_Decomposition/SOFTWARE_DECOMP.md`.
- PKG-00 consistency tracker rows were checked against `_CONTEXT.md` Architecture Basis Injection and the decomposition architecture basis table; no contradiction was found.
- No conservative evidence was found for adding rows, retiring rows, or changing CSV row semantics.
- One ACTIVE parent anchor exists; no `[WARNING] FLOATING_NODE` or `[WARNING] AMBIGUOUS_ANCHOR` condition applies.
- This dependency refresh does not authorize implementation, lifecycle promotion, release claims, professional approval, certification, sealing, authentication, or code-compliance claims.

## Run History
- 2026-06-16 18:25 - `TASK + dependency-extract`, MODE=UPDATE, STRICTNESS=CONSERVATIVE, decomposition `execution/_Decomposition/SOFTWARE_DECOMP.md` present; warnings: none; ACTIVE rows: 16; RETIRED rows: 0.

## Lifecycle Summary
- **ACTIVE:** 16
- **RETIRED:** 0
- **Satisfaction:** SATISFIED=9; TBD=7.
- **Closure state:** Register remains open for downstream reconciliation consumption; no row-level closure change was made.

## Downstream Handoff Notes
- Consumer context: RECONCILIATION.
- The local register remains source-supported as a semantic dependency surface for DEL-16-01.
- Candidate or non-gating ideas were not promoted; none were added to the canonical register.
