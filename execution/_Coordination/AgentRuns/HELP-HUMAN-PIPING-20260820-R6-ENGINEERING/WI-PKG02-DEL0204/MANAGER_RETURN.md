# WORKING_ITEMS package return — PKG-02 / DEL-02-04

Run: `HELP-HUMAN-PIPING-20260820-R6-ENGINEERING`
Instance: `WI-PKG02-DEL0204`
Parent graph node: `N1`
Status: `COMPLETE / VALID FOR AGENT 0 FAN-IN`

## Product and test outputs

- `core/adapters/framework/__init__.py`
- `core/adapters/framework/adapter_framework.py`
- `core/adapters/framework/plugin_verification.py`
- `tests/test_adapter_framework_contract.py`
- `tests/test_adapter_plugin_verification_del_02_04.py`

The exact DEL-02-04 Remaining item is closed: unit-safety, provenance,
diagnostics, protected-content, malformed-input, and adapter/plugin regression
layers now execute in memory and fail closed. Valid declarations remain
`BLOCKED_RUNTIME_NOT_SELECTED`; no runtime is dispatched or selected.

Integrated-review Amendment 1 removes invented/public top-level envelope
defaults and conservatively derives exact caller boundaries. Protected/private
evidence dominates, incomplete or unresolved evidence is review-required, and
public-reviewed output requires complete canonical provenance and privacy.

Integrated-review Amendment 2 binds the already-loaded caller schema to the
exact canonical structural/content fingerprint. The verifier fingerprints and
evaluates one plain snapshot, so weakened rules and hostile Mapping accessors
cannot diverge after authentication.

Integrated-review Amendment 3 enforces the canonical capability array, enum,
and operational `contains` contract without throwing. Hostile string-like
inputs fail closed while protected/quarantined provenance retains precedence;
schema-permitted duplicates remain accepted.

Integrated-review Amendment 4 enforces declaration/result provenance clearance
and protected-privacy quarantine. Exact raw-JSON preflight plus one detached
snapshot contains hostile, deep, cyclic, nonfinite, and overloaded caller
evidence without mutation; direct and composed verification remain structured,
schema-valid, and runtime-blocked.

Integrated-review Amendment 5 applies bounded detached exact-JSON snapshots to
every caller manifest, adapter, catalog, evidence, and plugin-schema surface.
It contains hostile/deep/cyclic/nonfinite/oversized inputs, bounds all fallback
inspection and diagnostic references, and preserves safely observable
quarantine markers without dispatching a runtime.

Integrated-review Amendment 6 canonicalizes and byte-bounds all post-snapshot
schema mismatch paths/messages and routes normalized manifest plugin diagnostic
references through the same exact canonical 256-byte helper used by malformed
fallbacks. Finite adversarial/near-limit keys and huge schema-valid plugin IDs
cannot enter diagnostic or envelope text.

Integrated-review Amendment 7 makes normalized schema-valid quarantined
metadata force the same protected/quarantine top-level boundary as malformed
fallback across public/private/protected companion evidence; it can never
produce public-reviewed output.

## Validation

- Focused plus existing adapter/plugin suites: `324 passed in 0.90s`.
- Composed result-envelope validation against the canonical adapter schema:
  PASS.
- Write containment and diff checks: PASS.
- Integrated review attempt 1: FAIL, invented/public top-level envelope finding.
- V13–V15 fresh reviews: exact conservative-boundary findings remediated.
- V16 fresh read-only review: PASS, zero actionable findings, all nine hashes
  and line counts matched, 100% of 3,930 frozen lines reviewed.
- Integrated review v2: FAIL, weak lookalike-schema authentication finding.
- V17 review: FAIL, authenticated-bytes/original-Mapping divergence finding.
- V18 fresh review: PASS, zero actionable findings, all nine hashes and line
  counts matched, 100% of 4,038 frozen lines reviewed.
- Integrated review v3: FAIL, malformed/unknown capability acceptance finding.
- V19 review: FAIL, hostile unhashable string-like exception/quarantine masking
  finding.
- V20 fresh review: PASS, zero actionable findings, all nine hashes and line
  counts matched, 100% of the 4,230-line frozen set and full diff reviewed.
- Integrated review v4: FAIL, non-cleared provenance/protected-privacy finding.
- V21–V25 reviews: exact malformed/hostile/deep manifest containment findings
  remediated in place.
- V26 fresh review: PASS, zero actionable findings, all fifteen hashes and line
  counts matched, 100% of the 5,563-line frozen set and full diff reviewed.
- Integrated review v5 and V27–V30 reviews: raw-input snapshot, marker fallback,
  deterministic resource-bound, diagnostic-reference, and caller-schema
  containment findings remediated in place.
- V31 fresh review: PASS, zero actionable findings, all twenty hashes and line
  counts matched, 100% of the 8,082-line frozen set and original-basis amended
  product/test diff reviewed.
- Integrated review v6 and V32: finite schema-mismatch path/text and normalized
  plugin-reference bounds remediated in place.
- V33 fresh review: PASS, zero actionable findings, all twenty-two hashes and
  line counts matched, 100% of 8,470 frozen lines and the original-basis amended
  diff reviewed; independent suite rerun `318 passed in 0.78s`.
- Integrated review v7 quarantined-metadata normalized/fallback boundary
  inconsistency remediated in place.
- V34 fresh review: PASS, zero findings, all twenty-three hashes/line counts and
  8,654 frozen lines matched; full original-basis amended diff reviewed;
  independent suite rerun `324 passed in 0.79s`.

## Deliverable effect and residuals

DEL-02-04 `_STATUS.md`, `MEMORY.md`, and run records were updated once at
fan-in. Lifecycle remains `IN_PROGRESS`. Separately governed runtime,
transport, capability-grant, and permission-persistence choices remain
owner-held; runtime dispatch stays blocked. No node blocker, waiver, decision,
or focused rerun remains.

Run-local coordination records are derivative evidence bound to accepted basis
`357a58b56726feba49507534159c3fbc4656b818`, DAG-009, R5, and the final V34
frozen diff; they do not replace deliverable or decomposition truth.

Requested Agent 0 action: accept N1 into integrated fan-in and route the tranche
to CHANGE. Broader registered `piping-pytest`, `evidence-sweep`,
`harness-pytest`, and `harness-self-check` remain integrated closeout surfaces.
No Git action was taken by this manager.
