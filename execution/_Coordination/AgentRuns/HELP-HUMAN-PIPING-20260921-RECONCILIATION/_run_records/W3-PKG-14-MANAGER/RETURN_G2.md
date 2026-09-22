# Return — W3 PKG-14 G2 worker a1e25eb9e506cc5b3 (launched directly by Agent 0), verbatim

Relayed to the PKG-14 manager (ad9c7b7758f14d2a5), which copies it verbatim as RETURN_G2.md.

---- WORKER RETURN (verbatim) ----
DONE DEL-14-04 forward=637f8ee04f410e68f5c6b4b61edca22f52f953ca41d3952985b31d81533ba981 reverse=3573beca72bf68e17e79f7044e1044e4c87e5853b5b7efad4e60c77cf3b0e63f notes=657291d86ab728b8c76a420b8f1388a553a96d29cc2d1f71317413c8a868bb0f validator=PASS
DONE DEL-14-05 forward=c191f720b0c7acdee0df5e58503360c3d1dc13263ef9e759ec8aead9fdb4a20c reverse=8d4574cd24169c4155475610a2474bc992fb6721efeabff6134ddc168f6b6003 notes=95d18cd53a4793b7625391520c3a3ec69915b257f02f6420badcf5a18bc53726 validator=PASS
BATCH PASS 0 findings

- Dispositions (219 rows across both):
  - ALIGNED 96, STALE_REVIEW_OR_EVIDENCE 34, STALE_SETUP_SPECIFICATION 25, PARTIALLY_IMPLEMENTED 21, NOT_ASSESSED 18, COVERED_BY_CHILDREN 16
  - IMPLEMENTED_DIFFERENTLY 5, VERIFIED_NOT_VALIDATED 3, DOCUMENTED_UNIMPLEMENTED 1
  - UNKNOWN 0; no canonical departures; the brief hash matched.
- Top cause tags: BASIS_POINTER_STALE 24 (stale revision and DAG pins), PARTIAL_SLICE 21, DOC_BEHIND_CODE 21 (setup-era "TBD" wording overtaken by code), EVIDENCE_OVERTAKEN 6 (parity records, CP-09), AUTHORITY_UNCLEAR 5, SCOPE_REDIRECTED_BY_RULING 5.
- Authority item, DEL-14-04 FG-04: DEC-009 sets a Rust core baseline, but the comparison engine is Python under core/. No ruling permitting this was found. Two rows, IMPLEMENTED_DIFFERENTLY · AUTHORITY_UNCLEAR · PROJECT_BASELINE · OWNER, MEDIUM confidence; the same question likely applies across the corpus.
- Owner item, DEL-14-05 FG-03 (CP-10): the SOW held the unmatched-classification values open pending human approval, but the schema fixes an eight-value list with no approval record found. Three rows, MEDIUM.
- Validation rows (VERIFIED_NOT_VALIDATED, INVARIANT): DEL-14-04 CLM-008.r02 and CLM-017.s02 (PDU-047 validation hold), and DEL-14-05 STATUS R01 (tolerance suitability and independent validation still open).
- Open gaps:
  - DEL-14-04: there is no accepted comparison-output schema (PDU-011, governing row R-14-04-007).
  - DEL-14-04: node, support and terminal comparison is never tested, because fixtures use only element references (R-14-04-004).
  - DEL-14-05: report-section references lack units, analysis statuses and limitations (R008).
  - DEL-14-05: no code produces the "comparison exporters" the register names; the decomposition only says "export semantics".
- Rename residue: the DEL-14-05 Scope of Work uses the old product name (CP-04, owner item). Also noted but not keyed: the old name in the DEL-14-05 schemas' $id and titles, and in a DEL-14-04 engine docstring and provenance string.
- No product caller: the engine is reached only from tests, so every aligned engine row is marked PRODUCT_CALLER: NONE. There were no protected-check or ISSUED rows, and no possible defects.

Files are in RUN/WAVES/W3/PKG-14/ (under DEL-14-04/, DEL-14-05/, plus _WORKER_DEL-14-04_NOTES.md). Scratch files are deleted.
---- END ----
