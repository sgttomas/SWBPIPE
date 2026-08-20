# N2-R1 review return — attempt 1

Status: `FAILED / ACTIONABLE FINDING / FAN-IN INVALID`

Frozen hashes matched and 100% of the three-file diff was reviewed. Present, missing-warning/non-accepted, malformed fail-closed, and private/pending behavior are otherwise adequate.

Blocking finding: `fixtures/reports/invented/component_provenance_cross_layer_projection.json` declares `schema_version` and a root object shape, but the TypeScript and Rust tests extract only `present_component` and `missing_component`. Root version changes or unexpected root fields would not fail either test. This violates the frozen requirement that the shared fixture cannot drift silently.

Required remediation: bind the complete root object in TypeScript and deserialize a typed, version-checked, unknown-field-denying wrapper in Rust. Then rerun focused checks, re-freeze hashes, and obtain a fresh 100% review.

Reviewer made no edits and ran no tests.
