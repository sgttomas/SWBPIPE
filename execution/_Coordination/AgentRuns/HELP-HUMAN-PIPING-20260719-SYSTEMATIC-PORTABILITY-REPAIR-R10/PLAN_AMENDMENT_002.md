# PLAN AMENDMENT 002 — Fail-Closed Evidence Shapes

- RunID: `HELP-HUMAN-PIPING-20260719-SYSTEMATIC-PORTABILITY-REPAIR-R10`
- Plan version: `3`
- Disposition: `AMEND`
- Applies to: existing author and verifier identities only
- Authority: HELP_HUMAN disposition within the owner-approved systematic portability objective and existing write fence

## Preserved Verifier Block

Verifier v1 returned `BLOCK`: broad evidence-token inference classified
misleading arbitrary names as evidence, including `NOT_A_RETURN.md`,
`RETURN_INSTRUCTIONS.md`, `HANDOFF_INSTRUCTIONS.md`, `SECRET_SUMMARY.md`,
`ARBITRARY_RESULT.json`, and `UNREGISTERED_MANIFEST.yaml`. Its
`RETURN_V1.md` and `STATUS_V1.json` remain immutable evidence.

## Author Remediation v2

Re-trigger the same `TASK-SYSTEMATIC-PORTABILITY-AUTHOR-R10-01` identity.
Remove broad token or substring evidence inference. Evidence classification
must use exact registered governed names/shapes or the four hash-bound
historical role overrides only. Misleading names, near matches, unknown
extensions, and arbitrary artifacts remain `UNCLASSIFIED` and fail closed.
Control precedence remains mandatory.

Add adversarial regressions for every verifier example, case and path
normalization, control precedence, unknown extensions, and exact standard
records. Rerun focused, full, semantic, containment, and governance checks.
Write versioned `RETURN_V2.md` and `STATUS_V2.json`; preserve author v1.

Do not create another sweep. Because remediation changes code after the
existing sweep, report its applicability upward before any sweep write.

## Verification v2

Only after author v2 terminal PASS, re-trigger the same
`TASK-SYSTEMATIC-PORTABILITY-VERIFY-R10-02` identity for a fresh read-only v2
review. Preserve verifier v1 and write versioned `RETURN_V2.md` and
`STATUS_V2.json`. No third child identity is authorized.
