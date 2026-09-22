# DEL-05-01 notes (W3, PKG-05, worker G1)

## Path aliases

- Project-root evidence tokens (`core/...`, `validation/...`, `schemas/...`) resolve under `projects/chirality-piping/`.
- `execution/_Coordination/AgentRuns/SOW-STAGE2-EXEC-20260712-01/...` (parity reports, claim maps) is the **repository-root** AgentRuns tree; project-level AgentRuns records are cited with the explicit `projects/chirality-piping/` prefix.
- Deliverable-local files are cited by their full path with spaces (resolves at the freeze).
- `GATE:GATE_EVIDENCE/B4_4_SWEEP_9D55/sweep.log.gz` carries per-crate cargo test counts (primitive_loads 49, load_case_algebra 18, stress_recovery 26); no suite was rerun.
- `#Lnn` anchors on SOFTWARE_DECOMP.md point at §12 rows: DEC-017 L608, DEC-018 L609, DEC-022 L613, DEC-023 L614, DEC-025 L616, DEC-026 L617, DEC-028 L619, DEC-068 L659, DEC-077 L668, DEC-092 L683.

## Judgment calls

- **DEC-068 catch-up (FG-DEL-05-01-02).** Text written before 2026-07-10 says wind/seismic/occasional are explicit loads only and procedure generation is TBD. The crate now generates seismic loads from user g-factors and model mass and wind loads from user pressure/shape on marked (and partial) spans, and product_physics calls both. Rows stating "only explicit"/"generation TBD" (CLM-005, CLM-006, CLM-014/REQ-05-01-008, CLM-017, CLM-029, CLM-033, MEMORY.s01) are `STALE_* · SCOPE_REDIRECTED_BY_RULING · LOCAL_DESIGN · NONE · RECORD · NO`, following the CS-05 precedent (superseded by ruling, catch-up needs no decision). Rows whose own exception clause admits later authorised generation (CLM-021, CLM-028, CLM-030) stay ALIGNED. CLM-017 also carries DEC-018 (unit catalog) and DEC-024/026 (tolerance) overtakes.
- **REQ-05-01-003** ("shall not encode ... procedure generators") read with "code-specific" qualifying the whole list; the DEC-068 generators take every factor from the user, so ALIGNED.
- **REQ-05-01-010** (provenance on load inputs) ALIGNED at MEDIUM: the crate `PrimitiveLoad` has no provenance field, but the product input validation requires public-preview provenance on every primitive load and generation input before conversion. The verifier should check this reading.
- **CLM-016/REQ-05-01-003** (boundary review): the last located boundary/protected-content review (2026-06-05) predates the generator code, so `STALE_REVIEW_OR_EVIDENCE · EVIDENCE_OVERTAKEN · INVARIANT · IP_DATA;RECORD · REVIEW` (F8: the overtaken check concerns the protected-content boundary). My own reading found no protected content; that is not a review.
- **Stale test count (FG-DEL-05-01-01):** SOW says 40 tests; the freeze has 49 and the gate sweep records 49 passed. CLM-008 also labels a cargo run as "validation evidence" (A5 noted).
- **F7:** load-case records, boundary metadata, equivalent-static preparation, lumping, axial helpers, assembly and the diagnostic bridge have no product caller; ALIGNED rows about them carry `PRODUCT_CALLER: NONE` (claims are about the crate). product_physics calls prepare_loads, the category/load types and the generators.
- **VER-001** judged as a verification-method claim and ALIGNED on the PASS parity record whose hash matches the frozen SOW (not labelled CP-09, which is for output rows).
- **CLM-023** (verification procedure) ALIGNED with a GAP_WORDING_CHECKED clause: applying its scope check today would report the stale pins carried on CLM-002/010/012; that is document state, not the method.
- **CONTEXT SURFACE** disposed as a pin (`BASIS_POINTER_STALE`) because the file as a whole declares revision 0.7 as current basis; SOW SURFACE carries CP-04 (CLM-027 "OpenPipeStress").
- **MEMORY** split: dated history ALIGNED; undated header sections ("Boundaries Preserved", "Remaining TBDs", first present at 7bee9ae41) judged on `MEMORY.s01` as `STALE_SETUP_SPECIFICATION` (F3).

## Canonical departures

None. CS rows inherit their fields. CP-03 blocks that pin revision 0.8 / DAG-007 (CLM-002, CLM-012) take CP-02 fields as CP-03 directs. Architecture-basis `.s01` follows the recorded W2 Agent 0 reading (readiness state → `STALE_REVIEW_OR_EVIDENCE`, cause per CS-05); `.s02` (Still-TBD list, origin 7bee9ae41) follows the W2 verifier reading (`STALE_SETUP_SPECIFICATION · SCOPE_REDIRECTED_BY_RULING`).

## Convention friction

- A requirement whose "only X unless later scope authorises dynamic Y" wording is overtaken by a ruling for static Y: treated as ruled overtaking (stale), not as divergence or ACCEPTED_DIVERGENCE (no "at next amendment" trigger in DEC-068).
- Many REQ/test rows legitimately use words like "missing" (finding-code names); GAP_WORDING_CHECKED clauses added where the validator listed them.

## Reverse pass

Answers: CLAIMED_BY 10, PARTIAL 5, COVERS 4, UNKEYED 2 (self-weight planning RC-05-0263 and its wasm engine RC-05-0408, nearest key CLM-006), NOT_MINE 398. The reverse pass did not change my view of any sealed row; the unkeyed self-weight generation reinforces the stale Weight row in CLM-006 (mass source and gravity no longer purely TBD). F5: every NOT_MINE on a capability whose entry points hit a path this ledger cites (product_physics lib.rs/validation.rs, the registers) carries a capability-specific reason.

## Batch consistency

`validate_ledger_v2.py --batch` over the three sealed forward ledgers: **PASS, 0 consistency findings**. Shared bodies (CONTEXT Decomposition Reference, Architecture Basis Injection and its .s01/.s02, Package Reference, PREPARATION Notes, Identification Type/Context envelope rows) took identical fields in all three ledgers. No flagged pair to justify or correct.

## Selectability

`SelectableUnderCurrentLoop` is `NOT_APPLICABLE` on every row (C9): since 2026-09-19 Piping selects work through owner-steered work graphs, not `## Remaining`.

## UNKNOWN rows

None in this ledger.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081). No row states or implies a release, approval, compliance or certification claim; dispositions are agent judgments, not owner rulings.
