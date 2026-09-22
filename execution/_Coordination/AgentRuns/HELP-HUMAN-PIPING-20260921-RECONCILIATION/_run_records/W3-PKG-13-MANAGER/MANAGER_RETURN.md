# MANAGER_RETURN — W3 PKG-13

DEL-13-01 PASS forward=3161f00b6084f59eaa30bed8e8aa121ea3ce6e2bb238af7e671f49a5ba0cb09e reverse=b4a13e38a0001df77bd4b2cd7d2b84e086b99593caebc18e6f4d4c6c41a54d8c rows=104
DEL-13-02 PASS forward=5b5ab108e7f6e3b69120152a981f4643037267ad7624f9a2522aedc879858d76 reverse=2c01f6a53f2e729e71e5a6aa07e950b3ceabfb80777dfe69befa6d087cbd42f1 rows=85
DEL-13-03 PASS forward=02c5366df9ce622ee04e1d5d4226977946d64ec7cff7a6cef99317b1f91e96ac reverse=12563319b10abfe49ef33d72df9119fbd63314edf61a88aaaeff478af625959d rows=112
DEL-13-04 PASS forward=fcbd1656ca76a227f03992c87d35cd82f1c808d6f029a67ed018fd5f997935d6 reverse=40c5306737ef318a06ee1e8c7e791bfd1fe29d782d3bd277f568d090ba712f83 rows=110

BATCH PKG-13: PASS batch of 4 ledgers: 0 consistency findings (BATCH_PKG-13.txt)

Validation: single mode with --reverse, --inventory ROUTING/PKG-13_capabilities.csv and --notes-gap. PASS, 0 findings, for all 4 (VALIDATION_DEL-13-0*.txt). Each recomputed forward SHA-256 equals both its SEAL hash and the worker's reported hash. All files and #END sentinels are present, no scratch files remain, and no reruns were needed.

Child agents: G1 = a0c0dea384f0884a4 (general-purpose, opus, "high (inherited)", nested harness-native Agent tool, foreground). The manager's foreground call ended on an API 529 during DEL-13-04. Agent 0 resumed the same agent directly, and it completed. Agent 0 saved its return, and the manager copied it verbatim as RETURN_G1.md.

Disclosed for the verifier (worker self-reports, not manager judgments):
- Possible false alignment: the worker flagged six sealed ALIGNED rows, left unedited. These rows claim the schemas match the accepted PKG-02 vocabulary, but those schemas lack force_per_length. The rows are 13-01 CLM-005.r05, CLM-003 and CLM-011.r04, and 13-02 CLM-003, CLM-009 and CLM-012/R-13-02-005. The verifier decides.
- Invariant rows: 13-01 REQ-13-01-011 (IMPLEMENTED_DIFFERENTLY, POSSIBLE_DEFECT); 13-04 REQ-007 (VERIFIED_NOT_VALIDATED); 13-04 Remaining R03.
- Baseline and owner: 13-03 and 13-04 use a Python core, a DEC-009 departure (FG-DEL-13-03-01, FG-DEL-13-04-01).
- The return says "Workers G2–G4 never ran". PKG-13 had only one group (G1), so this has no effect.
