# Return — W3 PKG-13 G1 worker a0c0dea384f0884a4 (resumed by Agent 0), verbatim

Received by Agent 0 after several 529 resumes. It is relayed to the PKG-13
manager (a9d06d1bb2783e1a8), which copies it verbatim as RETURN_G1.md.

---- WORKER RETURN (verbatim) ----
DONE DEL-13-01 forward=3161f00b6084f59eaa30bed8e8aa121ea3ce6e2bb238af7e671f49a5ba0cb09e reverse=b4a13e38a0001df77bd4b2cd7d2b84e086b99593caebc18e6f4d4c6c41a54d8c notes=5c51962c6bab01b173ede17dd88ee40147ec408646d8fbb85992f158fcd8cd4c validator=PASS
DONE DEL-13-02 forward=5b5ab108e7f6e3b69120152a981f4643037267ad7624f9a2522aedc879858d76 reverse=2c01f6a53f2e729e71e5a6aa07e950b3ceabfb80777dfe69befa6d087cbd42f1 notes=f87ba62d706fbfb4749108820b0cd540f1abdf2eba22f229b584308acd4f5571 validator=PASS
DONE DEL-13-03 forward=02c5366df9ce622ee04e1d5d4226977946d64ec7cff7a6cef99317b1f91e96ac reverse=12563319b10abfe49ef33d72df9119fbd63314edf61a88aaaeff478af625959d notes=29620c795a8a7e88a67b573bf7e45a459858a11d9bec5e1d49250cc6f3a1ee9a validator=PASS
DONE DEL-13-04 forward=fcbd1656ca76a227f03992c87d35cd82f1c808d6f029a67ed018fd5f997935d6 reverse=40c5306737ef318a06ee1e8c7e791bfd1fe29d782d3bd277f568d090ba712f83 notes=5773f41978329a521d5d460fea2aabd972148951e08363858a2129187f850c42 validator=PASS
BATCH PASS 0 findings

- Dispositions (411 rows): 251 ALIGNED, 68 STALE_REVIEW_OR_EVIDENCE, 38 NOT_ASSESSED, 24 COVERED_BY_CHILDREN, 15 STALE_SETUP_SPECIFICATION, 7 PARTIALLY_IMPLEMENTED, 5 IMPLEMENTED_DIFFERENTLY, 1 VERIFIED_NOT_VALIDATED, 1 IMPLEMENTED_UNDOCUMENTED, 1 DOCUMENTED_UNIMPLEMENTED. No UNKNOWN rows. Each of the 323 reverse files is mostly NOT_MINE; CLAIMED_BY rows: 13-01 → RC-13-0287; 13-02 → RC-13-0308; 13-03 → RC-13-0148; 13-04 → RC-13-0122 and RC-13-0108.
- Top cause tags: BASIS_POINTER_STALE 42, EVIDENCE_OVERTAKEN 12, REPRESENTATION_MIGRATED 11, RECORD_DRIFT 10, POSSIBLE_DEFECT 7, PARTIAL_SLICE 5, RENAME_OR_IDENTITY 4.
- Possible defect found after sealing (13-01, 13-02; not edited, recorded in notes for the verifier and a fresh worker): the PKG-02 units.schema.yaml dimension list includes force_per_length. The design_knowledge and constraint schemas lack it, yet their SOW rows claiming they "match the accepted PKG-02 vocabulary" were sealed ALIGNED. That applies to 13-01 CLM-005.r05, CLM-003 and CLM-011.r04, and to 13-02 CLM-003, CLM-009 and CLM-012/R-13-02-005. They should be non-aligned.
- Invariant rows:
  - 13-01 REQ-13-01-011 is IMPLEMENTED_DIFFERENTLY · POSSIBLE_DEFECT · INVARIANT. The live desktop Knowledge panel reads an invented preview design-knowledge file whose provenance is a bare string. It is not in the schema's shape and is not checked against the schema.
  - 13-04 REQ-007 is VERIFIED_NOT_VALIDATED; the 3D frame-target validation basis is held. 13-04 Remaining R03 is also INVARIANT · VALIDATION.
- Baseline and authority rows:
  - 13-03 and 13-04 are Python core modules, which departs from DEC-009 (Rust core). These are IMPLEMENTED_DIFFERENTLY · PROJECT_BASELINE · OWNER, findings FG-DEL-13-03-01 and FG-DEL-13-04-01. The pattern is likely corpus-wide.
  - Two schemas: a quantity parameter can carry a bare string value with no unit metadata (13-01 REQ-006, 13-02 R-005 PARTIALLY_IMPLEMENTED).
  - 13-03 R1 conflict evaluation is a PARTIAL_SLICE: no conflicts are computed.
  - Result-envelope integration is PARTIALLY_IMPLEMENTED with OWNER needed (13-03 CLM-013.r06 and CLM-028.r06; 13-04 CLM-014.s02).
- Evidence map: EVIDENCE_MAP lists NONE_FOUND for all four, but the A3 discovery step found PASS parity records. None binds the frozen SOW bytes (overtaken by the D-48 Wave 2 edits), so those rows are CP-09 EVIDENCE_OVERTAKEN rather than UNKNOWN.
- Rename residue (CP-04 on each SOW surface): the schema $id/titles, the engine docstring, and a provenance contributor string that the transform emits in its output. There are no ISSUED rows, protected-check rows or authority-conflict rows.
- Run conditions: scratch files are deleted. Workers G2–G4 never ran; this worker covered all four deliverables. Model: opus, reasoning "high (inherited)". Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

All files are in RUN/WAVES/W3/PKG-13/: one folder per deliverable, plus _WORKER_DEL-13-01_NOTES.md.
---- END ----
