# Worker G1 carry-forward notebook — W3 PKG-10 (DEL-10-01, DEL-10-02, DEL-10-03)

Running record of how recurring situations are judged, so the three ledgers
treat the same situation the same way. Agent judgments, not owner rulings.
Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

## Recurring situations and the treatment chosen

| # | Situation | Treatment |
|---|---|---|
| J1 | SOW `Source preamble` blocks (YAML frontmatter of the former Datasheet/Specification/Procedure/Guidance: `doc_kind`, `status: draft`, `created`) | CP-01 four-document residue. Metadata, so F3's metadata exception keeps `STALE_REVIEW_OR_EVIDENCE` (not `STALE_SETUP_SPECIFICATION`) · `REPRESENTATION_MIGRATED` · LOCAL_DESIGN · NONE · RECORD · NO |
| J2 | D-41 R5 T7 PDU-055 "current declaration" blocks (pin rev 0.8 + DAG-007; delegate residuals to `## Remaining`) | Own DECLARED_STATE row (CP-03); pins superseded revision → CP-02 `STALE_REVIEW_OR_EVIDENCE · BASIS_POINTER_STALE`. Delegation clause recorded as not relied on (A4). CanonicalSituation CP-02 |
| J3 | Setup-era text first present at `7bee9ae41` that says an artifact is out of scope / not created / deliverable-local when the implementation later created it under a human-authorised dispatch | `STALE_SETUP_SPECIFICATION · DOC_BEHIND_CODE · LOCAL_DESIGN · NONE · RECORD · NO` |
| J4 | Setup text naming the four documents (Datasheet/Specification/Guidance/Procedure) as current artifacts, gates or records | CP-01 `STALE_SETUP_SPECIFICATION · REPRESENTATION_MIGRATED` |
| J5 | Setup TBD overtaken by a later ruling (SCA-004/OI-004 export targets; DEC-019 migration; DEC-022 rule grammar; DEC-023 solver; DEC-025 CI provider; DEC-028 container) | `STALE_SETUP_SPECIFICATION · SCOPE_REDIRECTED_BY_RULING · LOCAL_DESIGN · NONE · RECORD · NO` |
| J6 | A Human-Ruling-Queue ("declared-open hold") item that code settled under an agent dispatch with no governing ruling (schema file placement / field layout / label vocabulary) | CP-10 `IMPLEMENTED_DIFFERENTLY · AUTHORITY_UNCLEAR · PROJECT_BASELINE · NONE · RECORD · OWNER`. Human-authorised dispatch briefs are context (A3), not rulings |
| J7 | Requirement rows whose cited PRD section was renumbered by PRD v0.4 (for example `PRD §19.3 Public API`, `§13.5`, `§18.2/18.3`, `§20`) | C6(b): the requirement is judged on substance; the stale citation is recorded in Notes. Blocks whose subject *is* the pointers (References, Standards pins) take CP-02 |
| J8 | Contract/schema artifact whose only consumers are tests (DEL-10-01 API contract; DEL-10-02 Python validator/gate) | F7: satisfies claims about the contract/validator itself; ALIGNED rows carry `PRODUCT_CALLER: NONE`. Claims about runtime behaviour (import/export paths, hooks) are `PARTIALLY_IMPLEMENTED · PARTIAL_SLICE` (CP-11) |
| J9 | DEL-10-03 handoff schema is consumed by the desktop `LocalFeaHandoffPanel` packet builder | Product caller exists; no `PRODUCT_CALLER: NONE` mark on DEL-10-03 contract rows |
| J10 | Rename residue (SOW prose, schema `$id`/title, contract const, desktop export filenames carrying `openpipestress`) | CP-04 once on the SOW SURFACE row; default variant (none of the four frozen identifiers are involved) |
| J11 | `_STATUS.md` Last Updated 2026-07-12 while History has 2026-07-16 | CP-05 on the STATUS SURFACE row |
| J12 | `_CONTEXT.md` SURFACE row | Whole-file declared state carries the stale revision pin (0.7, `current_basis`) → CP-02; blocks judged individually |
| J13 | Architecture Basis Injection | CS-04 keyed row for the pin; `.s01` for the "PKG-00 at SEMANTIC_READY" statement (PKG-00 members are IN_PROGRESS at the freeze) → `STALE_REVIEW_OR_EVIDENCE · RECORD_DRIFT` (review/lifecycle state, F3 exception); `.s02` for "Still TBD" items since ruled → J5 |
| J14 | Output matrix OUT-001 / VER-001 | CP-09: PASS parity records exist but none matches the frozen SOW (`AnyPassMatchesFrozen=NO`) → `STALE_REVIEW_OR_EVIDENCE · EVIDENCE_OVERTAKEN` |
| J15 | Remaining item whose gate (D-12) was ruled | CP-07 `REMAINING_STATE_MISMATCH · RECORD_DRIFT · RULED_CRITERION` |
| J16 | Remaining item accurate with open action carried by a non-aligned governing row | ALIGNED + `OPEN_ACTION: <key>` (F2) |
| J17 | Verification evidence | Suite-level gate `GATE:GATE_EVIDENCE/B4_4_SWEEP_9D55/SUMMARY.json` (`pytest -q tests` PASS 1,138; desktop unit 1,621) plus the test file path; per-test status not rerun (A6) |
| J18 | Shared bodies inside my set: DEL-10-01 CLM-024 = DEL-10-03 CLM-025 (Records); DEL-10-01 CLM-025 = DEL-10-03 CLM-026 (Completion Condition) | Records → CP-01 (J4); Completion Condition → J3, identical profile in both ledgers |
