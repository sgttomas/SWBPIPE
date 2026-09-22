DONE DEL-06-01 forward=9492ece504b8e4098df0830a4d3ebd87e1e30b53c835db7117cfe2e250ebfdb6 reverse=e1d84f1fc2aeafc4351766805fc2dece751000534ba13de2b0c76fdf5b109d73 notes=0212cf522a1604b83e436a6254592fa7c4bb7b867d4716120f396f478b6c968a validator=PASS
DONE DEL-06-02 forward=2a2869d67b27a4aa09cb6e23572586cc1951cfe55e5d481894c67d7a7db23549 reverse=305d94dc209b036ec700984338bc40f84fda61af473319868312b74854f3368e notes=0c8dce53c23caa451ac7bf1265159d0c545e12265125150ebf9ab33ae5a0a0ec validator=PASS
DONE DEL-06-03 forward=f36bc044ea87bd0dad4db57e5ab093acb237990d91373c27e619950e0e5cfab4 reverse=8b970319bac342cfdec4e78ff7df39faf72d1af31374da3627f1cb0166311118 notes=8553707792c550ea5e48c44c00f505b49c53f84ec93dccfada2837ecdad68f45 validator=PASS
BATCH PASS 0 findings

- **Disposition counts (322 rows across the three ledgers):** ALIGNED 153, STALE_SETUP_SPECIFICATION 52, STALE_REVIEW_OR_EVIDENCE 42, NOT_ASSESSED 29, COVERED_BY_CHILDREN 22, PARTIALLY_IMPLEMENTED 12, UNKNOWN 10, IMPLEMENTED_DIFFERENTLY 2. The reverse files have 292 capability answers each: 4 CLAIMED_BY, 5 PARTIAL and 10 COVERS in total, and the rest NOT_MINE.
- **Top cause tags:** BASIS_POINTER_STALE 29, SCOPE_REDIRECTED_BY_RULING 25 (mostly grammar TBDs ruled by DEC-022), DOC_BEHIND_CODE 20, PARTIAL_SLICE 12, EVIDENCE_NOT_LOCATED 10.
- **Invariant (IP_DATA) rows, UNKNOWN:** 10 rows, all needing the same missing evidence. No protected-content review of the public rule-pack schema, the invented demo pack or the evaluator fixtures was found. There is no DEC-058 scan record (`validation/evidence/releases/` does not exist at the freeze) and no reviewer sign-off; the demo pack only declares its own review status.
- **Invariant rows, PARTIALLY_IMPLEMENTED:**
  - DEL-06-01 REQ-011: the schema does not force public examples to use invented values or carry a notice.
  - DEL-06-02 OUT-001 (SECURITY): the evaluator and document decoder have no depth or size limit.
  - DEL-06-02 REQ-011 and CLM-023: the public-example protected-content tests and the plugin bypass-attempt tests do not exist.
  - DEL-06-03 CLM-014: no test checks that protected defaults are not shipped.
- **Owner items:**
  - The comparison-tolerance hold was settled in code by exact float comparison with no ruling (CP-10: DEL-06-02 CLM-006.r03 and CLM-015.r04). The owner may decide DEC-022's frozen test corpus counts as the ruling.
  - DEL-06-01's SOW names the former product name (CP-04). Code identifiers carrying the former name are listed for R3 in the notes, not as rows.
- **Baseline items (AB-00-06, PROJECT_BASELINE):** evaluator and completeness findings carry no remediation, class or provenance fields, and are not bound into a governed result envelope.
- **Possible defect for R3:** SOFTWARE_DECOMP rev 0.12's own OI-006 row still says the grammar is TBD, although DEC-022 ruled it.
- **Checks:** there are no ISSUED or AUTHORITY_CONFLICT rows. All three Remaining items in DEL-06-02 are accurate and point to the rows that carry the open work (F2). Nothing in the reverse pass changed a sealed row.

Scratch files were deleted. All outputs are in `/Users/ryan/dev/chirality/.claude/worktrees/task-management-gen-pass-518da2/projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/WAVES/W3/PKG-06/`: one folder per deliverable (`DEL-06-01/`, `DEL-06-02/`, `DEL-06-03/`), plus `_WORKER_DEL-06-01_NOTES.md`.
