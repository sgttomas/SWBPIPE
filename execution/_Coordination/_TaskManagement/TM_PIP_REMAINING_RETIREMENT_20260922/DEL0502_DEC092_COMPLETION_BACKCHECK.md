# DEL-05-02 DEC-092 source/current-result comparison

This bounded comparison tests whether four historical census keys still
represent live work. The pinned `DEL-05-02/_STATUS.md` preimage is SHA-256
`42f417c0e28a26ca023f09c406aed4351a1327522a749ec4004e184313dccd24`.
After the two verified bullet removals, its SHA-256 is
`36e63ae7656915398098011ff6378b702d2cc1d18643a880caf79fcdfd62eb12`.
Its `## Remaining` has three actual bullets: item 1 (rule-pack combination
interface) stays open; item 2 is a bounded past-result note; CSV items 3–5
are one wrapped bullet saying the DEL-09-04 DEC-092 derivative is deferred.

| Census key | Original meaning preserved in `CANDIDATE_ROW_ACCOUNT.csv` | Current evidence and disposition |
|---|---|---|
| `DEL-05-02:2` | DEC-092 product implementation and Receipt 87 were already closed; no suite was rerun for the later R5 record repair. It also said the derivative below was still deferred. | Separate producing evidence: DEL-05-02 `MEMORY.md` 2026-08-03 implementation row, `_run_records/WORKING_ITEMS_RUN_2026-08-02_DEC092_TEMPERATURE_G_IMPLEMENTATION.md` (SHA-256 `0a644ac4f8b1ac3c843dee328fdf973a12279a956aab01ae01cfada611a50d1f`), implementation commit `c394365ca72b8383c7d7203ce5be2cb9ea67d508`, Receipt 87 and passing commit-bound sweep `SWEEP_20260803T194132Z_c394365ca72b.json` (SHA-256 `7c15d42cd369c24f883a32192b069458da5eecbaba8c97d87a65735b3daee97b`). The product-result part is historical and bounded; the later derivative-deferral clause is stale, proven separately below. No fresh product pass or lifecycle claim follows. Candidate treatment: retire this past-result Remaining bullet with its evidence still at source. |
| `DEL-05-02:3` | First physical line of the one wrapped bullet: “DEL-09-04 validation-manual derivative regeneration remains deferred to that”. | DEL-09-04's owner-adopted 2026-08-09 run regenerated the exact DEC-092 page and index row. The page is `docs/validation_manual/cases/mechanics/mech-tp-dec092-temperature-indexed-shear-modulus-torsion.md` (SHA-256 `f6019a2e216cdbc37493c6e77d388f6ad39ced5363e1e75db1d05f5820933110`), and `docs/validation_manual/index.md` (whole-file SHA-256 `f6d611059545505bd8ec94595406411487db4db2d23db9983a4162e93e691ee6`) links the DEC-092 page at row 104. The owning run record `WORKING_ITEMS_RUN_2026-08-09_DEL0904_DEC092_VALMANUAL.md` is SHA-256 `3e2d9ce81907b8422962ad3df19847a3e3da16a35dcad455db47c1058c069c78`; it records targeted check/render, 64/64 temporary full render, 63/63 historical parity, mechanics oracle 1/1, product physics 98/98 and harness pytest 349/349. DEL-09-04 `MEMORY.md` and `_STATUS.md` record the exact derivative bullet discharged, with lifecycle still IN_PROGRESS. Candidate treatment: fulfilled wrapped bullet, not three tasks. |
| `DEL-05-02:4` | Continuation: “deliverable's owning cadence. It does not change this deliverable's validated”. | Same one receiving page/index/run evidence as key `:3`; the 2026-08-09 work followed DEL-09-04's own cadence. The DEL-05-02 producing implementation remains the separate bounded result above. Candidate treatment: same fulfilled wrapped bullet, original key retained as provenance. |
| `DEL-05-02:5` | Continuation: “DEC-092 implementation result.” | Same one receiving page/index/run evidence as key `:3`. The derivative page is `DRAFT_EVIDENCE`, not `MAINTAINER_REVIEWED`; current DEL-09-04 `_STATUS.md` still lists maintainer review and public comparison numbers as separate open work. Candidate treatment: same fulfilled wrapped bullet, no whole-manual completion claim. |

PR #859's later validation-case/test reconciliation changed other case pages,
DEL-09-04 `MEMORY.md` and a new run record. Across the old merged instruction
basis and rebased `origin/main`, the DEC-092 case page, index row and
DEL-09-04 `_STATUS.md` bytes are unchanged (SHA-256 values above; `_STATUS.md`
`0ae4fa9cf2e2b506e19f0b6f8836e1ceba309c057680e44d8f042acd8421e889`).
The new MEMORY entry does not reverse the 2026-08-09 completion statement.
The current source still requires DEL-09-04 maintainer/page review and public
benchmark comparison values; they remain open at that deliverable.

The earlier proposal to *regenerate* this derivative in DEL-09-04 Scope of
Work is now withdrawn from the unapplied receiving-document patch. The
owner's conditional fulfilled/bounded-result principle was applied only to
the two verified DEL-05-02 source bullets above after exact source comparison.
`APPLIED_ROW_LEDGER.csv` binds all four historical census keys to the same
status postimage while distinguishing item 2's separate bounded result from
the one wrapped item 3–5. Independent review of this later account/status
diff remains required. Item 1 and all unrelated holds remain.
