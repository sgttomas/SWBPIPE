# K8 WORKING_ITEMS repair return V2

Verdict: `REPAIR_COMPLETE_RK_BACKCHECK_REQUIRED`.

## Resolved findings

- `RK-001`: added a mandatory version-specific writer/reader validator contract, stable error taxonomy, strict JSON/finite guards, global duplicate-ID and mirrored-field/reference checks, ordered source-result-ref preservation, row-status-over-set-type rule, and schema-bound owner semantics for all 45 mappings.
- `RK-002`: froze the exact recommended K-A+K-U1 request field, Rust writer API, write/read registries, refusal payload, report-package request field, Tauri command, desktop cache key and single-document consumption path. These remain candidate details, not an Owner selection.
- `RK-003`: made lossless pending pressure transport explicitly permissible as `preserve_only`; only status/meaning standardization remains gated by PKG04/05 acceptance.

## Evidence and tests

- `RESULTS_SCHEMA_0_2_0_CANDIDATE_V2.json`: additive schema candidate revision with 45 owner-bound branches.
- `SOURCE_KIND_MAPPING_V2.csv`: the same 45 native pairs plus explicit `owner_semantics`.
- `semantic_validator_v2.py`: executable reference validator for both boundaries.
- `build_candidate_v2.py`: full 830-row in-memory witness generator and writer/reader validation.
- `adversarial_tests_v2.py` and `ADVERSARIAL_VALIDATION_V2.json`: thirteen invariant checks covering every RK probe and the additional absence/order/cross-set variants.
- `API_CONTRACT_V2.json`, `ROUTE_A_DESIGN_V2.md`, and `VALIDATOR_CONTRACT_V2.md`: implementation-ready candidate API, migration, interpretation and error behavior.

Focused validation passed: Draft 2020-12 schema construction, 45 mappings, 830/830 exact source records, strict writer/reader round trip, duplicate IDs across sets, all mirror/reference mismatches, metadata presence, source-reference order, NaN/infinity, unsupported pair, false owner at runtime and schema boundaries, and preserve-only status inside a mechanics set.

## Limits and next owner

This is derivative design evidence. It does not adopt K-A+K-U1, implement a public schema/API, change pressure physics, or satisfy lifecycle acceptance. Next owner: RK for fresh independent successor backcheck, then `/root` HELP_HUMAN for fan-in and later audit D05 Owner presentation.
