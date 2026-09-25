# F1 repair backcheck

F1 is closed at source-review level. The repaired fixture consumer join is ready for the parent-owned focused previewService tests, affected App tests and TypeScript compilation. No additional actionable findings were identified. Runtime validation remains pending; this is not a test pass or whole-cut clearance.

The complete one-file repair diff was reviewed and reconstructed exactly from its preserved preimage and after-image. The preimage is the originally reviewed App test `97e6b7af…`; current and frozen repaired bytes both match `475aa1b3e82adcfb3ed5a9ff017bc6dc17a44bc21e11a3913f6babf700d59a2d`. The repair contains only the two helper imports and the bounded status-test split.

The two unregistered statuses now use a copy of the historical fixture for display-only helper/chip, blocked-deformation and actual DiagnosticsPanel checks. They do not invoke a native job or strict fresh-result builder. The helper call preserves the exact token and its Solver fallback; the supplied cell types fit the actual helper interface. The diagnostics control checks rendered content, and the fixture copy prevents historical-import mutation.

The connected test now uses registered `MODEL_INCOMPLETE`, which the existing V03 validator admits. It retains the full raw rows as synthetic inspection input and the diagnostic, blocked deformation, disabled report/rule/comparison, absent solve-proof and no-report-save assertions. This change lowers mechanics standing; it does not invent passing producer or numerical evidence. The unchanged `numericalResultStanding` refuses Current eligibility when mechanics is not solved, and the unchanged `solveProofStatus` returns null for this status. The expected known-status label matches the display helper used by the deformation path. Arbitrary-token fresh admission remains prohibited by the unchanged validator.

All nine static backchecks passed. Other source files recorded by the original review are unchanged, including previewService and its tests, V03 validation, workspace/current-result state, and the generator. All 54 generation inputs and both actual stdout fixtures still match their hashes; the old fixture and default model remain unchanged. Complete diff reconstruction confirms no stale/cancel assertion changes in this repair.

Evidence: `_run_records/STATIC_BACKCHECK.json`, `backcheck.py`, `scope.stdout.json`, and `ORIGINS.json`. The original `FIXTURE_ROUTE_REVIEW/RETURN.md` remains historical; this backcheck closes its sole finding for the repaired App bytes.

Execution boundary: same reviewer TASK `/root/solver_manager/fixture_route_review`, parent `/root/solver_manager`, delegated-harness-native followup; no descendants, Git, source edits, npm, Cargo, builds, tests, native or browser execution. Instruction origins from the prior review were checked unchanged. No native, physics, engineering-acceptance or whole-cut qualification is claimed.
