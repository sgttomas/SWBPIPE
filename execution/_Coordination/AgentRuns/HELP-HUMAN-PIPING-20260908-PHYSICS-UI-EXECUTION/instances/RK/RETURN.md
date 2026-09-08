# RK independent K8 review return

Verdict: `CHANGES_REQUIRED` for the frozen candidate design. The coverage, source-kind classification, checksum distinction, and fail-closed version/kind foundations are reproducible and sound, but the packet is not yet implementation-ready at the schema/runtime and public integration boundaries.

## Findings

### RK-001 — HIGH — Required lossless invariants have no mandatory runtime validator contract

`ROUTE_A_DESIGN.md` says `source_record` is controlling and requires exact equality, unique IDs, and validation before interpretation. The schema necessarily defines the mirrored fields independently (`RESULTS_SCHEMA_0_2_0_CANDIDATE.json` around lines 3075–3135), while `build_candidate.py` checks equality and uniqueness only for the generated nonlinear fixture (lines 298–367). No normative runtime validator, invocation points, or error taxonomy closes the properties JSON Schema cannot express.

Focused probes show the frozen schema accepts duplicate result IDs; disagreement in result ID, value, unit, metadata, and object/source reference mirrors; `NaN` and infinity in the Python validator data model; and a P5-pending classification carrying `owner_semantics=PKG08_TRANSPORT_ONLY`. These are not hypothetical formatting differences: they let a schema-valid document contradict the field declared controlling or falsely assign semantic ownership.

Required repair:

1. Define one mandatory version-specific semantic validator and require both the writer before emission and every reader after version dispatch/schema validation, before interpretation, hashing as a governed result, report generation, conversion, or UI rendering.
2. At minimum reject duplicate `result_id` across all result sets; unequal `result_id/source_record.id`, `magnitude/source_record.value`, and `unit/source_record.unit`; unequal metadata including absence/presence; and reference/projection disagreement under an explicit mapping rule. Preserve source-result-ref ordering.
3. Reject non-finite values in both mirrors before strict JSON serialization and require parsers/serializers that reject non-JSON numeric tokens.
4. Bind `owner_semantics` to each mapping branch in the schema where possible; the generated `quantity_branches` currently constrain the other four classification fields but omit owner (builder lines 176–194).
5. Freeze stable outcomes such as `RESULT_EXPORT_DUPLICATE_RESULT_ID`, `RESULT_EXPORT_SOURCE_RECORD_ID_MISMATCH`, `RESULT_EXPORT_SOURCE_RECORD_VALUE_MISMATCH`, `RESULT_EXPORT_SOURCE_RECORD_UNIT_MISMATCH`, `RESULT_EXPORT_SOURCE_RECORD_METADATA_MISMATCH`, `RESULT_EXPORT_SOURCE_RECORD_REFERENCE_MISMATCH`, `RESULT_EXPORT_NON_FINITE_NUMBER`, and `RESULT_EXPORT_CLASSIFICATION_OWNER_MISMATCH`, with adversarial tests at writer and reader boundaries.
6. State that required row `semantic_status` controls interpretation even when the enclosing current-shaped result set remains `set_type=mechanics`; alternatively split transport observations/user-review rows into explicitly non-mechanics sets. The generated witness currently groups 26 transport-observation rows and 40 unstandardized rows in the one mechanics set.

### RK-002 — MEDIUM — Version negotiation is not attached to an exact current API boundary

The design says the writer accepts a requested version at “the adapter boundary” and later requires a structured request/refusal at “the chosen public boundary” (`ROUTE_A_DESIGN.md` lines 58–64 and 78–84). The audit-D05 response only selects K-A/K-B/K-C plus K-U1/K-U2. It does not select the request field, DTO, function/command boundary, or response shape, so the advertised Owner response does not close the remaining implementation decision.

Required repair: name the exact request and result types/functions for the first writer and each current consumer. In particular, state whether negotiation belongs in the library-only headless result-export attachment, the `export-results` report-package request, a new shared PKG10 adapter API, or more than one explicit boundary; freeze the requested-version field, absence/default rule after Owner selection, supported-version registry owner, structured refusal payload, and how both desktop generators obtain the same canonical document instead of regenerating it.

### RK-003 — MEDIUM — Pressure acceptance sequencing is ambiguous and may unnecessarily block lossless transport

The candidate correctly preserves all pressure rows and marks them P5-pending. The ordered implementation plan nevertheless places accepted pressure semantics before versioned DTO/schema and exhaustive adapter implementation (`ROUTE_A_DESIGN.md` lines 88–95), which can be read as gating lossless route-A transport on P5 standardization.

Required repair and root decision question: does the P5/PKG04-05 step gate only changing a pressure row from pending/source-specific status into standardized mechanics semantics, while exact `source_record` transport with the pending status remains permissible after public compatibility selection? If yes, state that explicitly and move lossless pending-semantic transport outside the pressure-acceptance gate. If no, identify why even unchanged native transport must wait. No pressure status or meaning should be changed by this repair.

## Verified without finding

- K8 manifest and all 12 outputs match their declared hashes.
- Exact coverage is correct: 2,348 current canonical rows; 9/33/39 unique omitted rows; native totals 786/830/813; 2,010 valid rows; 338 invalid rows; 518 invalid metadata occurrences.
- The 45 `(kind, unit)` rows are unique and equal to both schema discriminator sets. Full nonlinear native `source_record` equality covers 830/830 rows.
- Required `semantic_status` values reject attempts to relabel pending pressure or transport observations as existing canonical categories. Pressure meanings remain pending P5.
- Outer and inner unknown versions, unknown kind/unit pairs, and checksum label/claim mismatches fail schema validation.
- Checksum algorithm/payload-ref/value triplets are unchanged, legacy `rfc8785_jcs` remains explicitly non-conformity-proving, and the new enclosing-document digest distinction is correctly stated.
- Compatibility/version choices remain candidates for the Owner; no public adoption is represented.

## Handoff

Derivative status: this RK packet is independent review evidence over the frozen K8 derivative design packet. It is not decomposition truth, public compatibility adoption, lifecycle acceptance, or physical approval.

Blocker: K8 successor repair and fresh independent backcheck. Rerun the focused candidate reproduction and every adversarial probe after the successor is frozen. Next owner: `/root` HELP_HUMAN to route the findings to K8's owning author scope and decide/route the pressure-sequencing clarification. No source or K8 artifact was edited by RK.
