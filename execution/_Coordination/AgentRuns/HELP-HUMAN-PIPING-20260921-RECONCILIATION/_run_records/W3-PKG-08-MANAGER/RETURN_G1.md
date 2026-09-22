DONE DEL-08-01 forward=7e8d5b07edb3e223c4cb6b1b0b28adf6ad59d147e4a9c631961f585b9adb46b7 reverse=395b90bd411ddfe6dae5c387894860fdc096b8868dc29d6b66aac493536a66eb notes=767a1c200c94f670e092040c5422ad771b77f6e0450334a3de38630938c6b7e7 validator=PASS
DONE DEL-08-02 forward=91b9d445eba096bd3dccd184b5ae5f49df43f6057631f7bd2b67a8a0861439a8 reverse=3e44fc2bbba579ea8a85800fc81faebdd9060185ba7ff5375caea5c8f4a4ec38 notes=d53bce828204d372831b66a3a02b17aa69369ecd7aa2a63738b86f6ca31bc040 validator=PASS
DONE DEL-08-03 forward=f560b76ef4222c89e42516ccb516a4a257ff3a8573a362ca56eb1f9e089030ab reverse=52f0e8a524d6876f4a711c480e64c7208c85af01135dbd4956f81ec6bd0a3f65 notes=b3233bcd46e02c325f5c41746274e636a8f2ea3534bb5de3b2e951da045878b3 validator=PASS
BATCH PASS 0

- **Dispositions (297 forward rows):** ALIGNED 124, STALE_REVIEW_OR_EVIDENCE 42, STALE_SETUP_SPECIFICATION 39, PARTIALLY_IMPLEMENTED 33, NOT_ASSESSED 30, COVERED_BY_CHILDREN 18, IMPLEMENTED_DIFFERENTLY 10, DOCUMENTED_UNIMPLEMENTED 1. No UNKNOWN rows and no canonical departures.
- **Reverse answers:** 08-01: 11 CLAIMED_BY, 3 UNKEYED, 2 PARTIAL, 2 COVERS. 08-02: 3 CLAIMED_BY, 3 PARTIAL, 1 COVERS. 08-03: 2 CLAIMED_BY, 2 PARTIAL, 3 COVERS. Every other capability is NOT_MINE. The 3 UNKEYED are the report-package container, native save and save controls: built under DEL-08-01 but not covered by any issued key.
- **Top cause tags:** PARTIAL_SLICE 32, BASIS_POINTER_STALE 25, DOC_BEHIND_CODE 21, SCOPE_REDIRECTED_BY_RULING 11, POSSIBLE_DEFECT 10, REPRESENTATION_MIGRATED 10.
- **Possible defect, R4 code-change candidate (FG-DEL-08-02-01):** the audit-manifest crate hashes sorted-key JSON, not RFC 8785, and cannot label RFC 8785 (JCS, the canonical JSON the governing sources require). The desktop app computes the model and input-manifest hashes as RFC 8785 but labels them `project_local_deterministic_json`. Affected: 4 rows, PROJECT_BASELINE, BASELINE;RECORD, owner decision.
- **Possible defect, R4 code-change candidate (FG-DEL-08-03-01):** the desktop report adapters work out a diagnostic's class from its severity (SOLVE_BLOCKING or ASSUMPTION_WARNING) and insert a fixed remediation sentence. The DEL-08-03 contract says to carry the producer's value or mark it TBD, never infer it. Affected: 6 rows, PROJECT_BASELINE.
- **Gaps across all three deliverables:**
  - Product reports and manifests never include rule-pack references. The package request refuses whenever a rule check is active.
  - No software/application version is recorded anywhere, and the solver version is absent from the rendered HTML/PDF. This touches OPS-K-REPORT-1 and SPEC §9; tier PROJECT_BASELINE, not INVARIANT.
  - The rendered tables drop some trace fields the section record carries: class, source and affected object for diagnostics; redistribution and review status for provenance notes.
  - The product never supplies assumptions.
- **Rename residue:** DEL-08-01's STATUS Remaining names `.opsproj`, recorded as the CP-04 identity-rename variant (existing saved files must still open). Active former-name code identifiers exist but no SOW names them: the `open_pipe_stress_*` crates, the solver name, document kinds, and "OpenPipeStress Project Package".
- **Flagged for the verifier:** R-08-01-009 and CLM-004.r05 are sealed ALIGNED at MEDIUM confidence, although the routing note says only the contract test reads `report_generator.schema.yaml`. DEL-08-01 Remaining R01 is open with no owning governing row (DOCUMENTED_UNIMPLEMENTED); DEL-02-05 may own it. No protected-check, invariant-tier, ISSUED or authority-conflict rows.

Files are in `projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/WAVES/W3/PKG-08/`:
- `DEL-08-01/`, `DEL-08-02/`, `DEL-08-03/`: forward, SEAL, reverse and notes files for each deliverable
- `_WORKER_DEL-08-01_NOTES.md`: the shared notebook

Scratch files are deleted.
