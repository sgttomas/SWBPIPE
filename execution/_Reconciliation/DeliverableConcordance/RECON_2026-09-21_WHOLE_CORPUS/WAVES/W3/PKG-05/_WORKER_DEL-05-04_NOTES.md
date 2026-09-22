# Worker notebook — W3 PKG-05 worker G2 (DEL-05-04, DEL-05-05)

Carry-forward record of how recurring situations were judged, so both
deliverables get the same treatment. Agent judgments, not owner rulings.

| Situation | Treatment used |
|---|---|
| SOW frontmatter `decomposition_basis` pinned to `SOFTWARE_DECOMP.md@eaad463c0` (revision 0.8) | SOW SURFACE row: CP-02 · STALE_REVIEW_OR_EVIDENCE · BASIS_POINTER_STALE · LOCAL_DESIGN · NONE · RECORD · NO |
| Named active identifier carries the former product name (schema `$id`/title; Cargo package name) | `SOW.s01` SUBCLAIM, CP-04 default variant (LOCAL_DESIGN · NONE · RECORD · OWNER); none of the four frozen-contract/persistence identifiers is involved |
| Block cites "SOFTWARE_DECOMP.md revision 0.7" | CP-02 · STALE_REVIEW_OR_EVIDENCE (pins stay SROE under F3) |
| Block names Datasheet/Specification/Guidance/Procedure as current | CP-01; class by F3 origin test (`git log -S` on the deliverable folder): text first present at `7bee9ae41` → STALE_SETUP_SPECIFICATION, later → STALE_REVIEW_OR_EVIDENCE |
| Output-matrix OUT-001 and VER-001 | CP-09 · STALE_REVIEW_OR_EVIDENCE · EVIDENCE_OVERTAKEN (PASS parity records exist, none matches the frozen SOW) |
| Purpose-block OUT-001 (output statement) | Judged on substance, not CP-09 |
| D-41 "current declaration" blocks | CP-03, own DECLARED_STATE row, disposition by the declared subject (C6(b)); duplicate copy carries `DUPLICATE_OF` and identical fields |
| Architecture Basis Injection | CS-04 row for the pin; `.s01` PKG-00 SEMANTIC_READY (STALE_REVIEW_OR_EVIDENCE · SCOPE_REDIRECTED_BY_RULING, Agent 0 reading / SR-1); `.s02` Still TBD items since ruled (STALE_SETUP_SPECIFICATION by F3, SCOPE_REDIRECTED_BY_RULING) |
| Section wrappers and title-only blocks pre-typed NON_NORMATIVE | NOT_ASSESSED |
| Tables with optional `.rNN` keys | Not split unless parts take different dispositions; DEL-05-05 splits CLM-003, CLM-010 and CLM-012 |
| Engine/crate with no product caller | F7: ALIGNED only for claims about the engine, with `PRODUCT_CALLER: NONE` in Notes |
| Claim that holds only because governed behaviour does not exist (e.g. "records must not survive hash changes" with no records) | CP-11 → PARTIALLY_IMPLEMENTED · PARTIAL_SLICE; tier by the boundary touched (F8) |
| Suite pass status | Cited as `GATE:GATE_EVIDENCE/B4_4_SWEEP_9D55/SUMMARY.json` with "not rerun" |

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
