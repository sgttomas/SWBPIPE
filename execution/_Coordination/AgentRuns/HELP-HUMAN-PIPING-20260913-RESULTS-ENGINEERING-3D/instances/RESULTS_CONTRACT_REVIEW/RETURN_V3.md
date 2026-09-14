# Focused RCR-03 contract backcheck

Verdict: **PASS — RCR-03 resolved in the selected RESULTS V4 candidate.** No remaining contract finding in this focused scope. This continues the same independent Type 2 review; RCR-01/02 remain resolved by RETURN_V2. Prior findings, counterexamples, snapshots and historical failed pins remain unchanged.

The new schema redirects only SourceAnnotations.metadata to RawSourceAnnotationMetadata, whose allowed JSON types make the exact received partial, empty, extended and nested annotations representable. The only other schema change is descriptive text. Every existing definition except that one annotation field—including strict canonical ResultMetadata and its enums—is structurally identical. The field contract mirrors the redirect and retains every prior semantic requirement in its original order, adding exact raw-preservation and canonical-projection rules. No source-origin, authored-model proof, Current, dimension, version or seven-function legacy fence is weakened.

All 28 frozen cases were read, with independent comparisons against their two pinned original rows. Each preserves every nonmetadata field and copies received annotation metadata exactly. The 20 single-field changes match precisely the declared removal or empty-string operation, covering all five fields for review signature043 and required physical signature009. All eight exact RCR-03 counterexamples match a corresponding new fixture. The eight additional cases cover unknown nested members (valid supplementary Unicode, array values and numeric object keys), empty object, array and explicit null for both signatures. The 26 incomplete cases disclose once without a quantity/review target or canonical metadata invention under the unchanged accounting contract. The two complete unknown-extension cases retain the full raw annotation; review043 remains review-only and physical009 projects exactly its five existing canonical metadata fields.

Absent metadata retains the established null annotation convention, with the exact received row hash and pointer preserving actual field absence. The 28 additions do not include a separate absent-field case; that existing policy remains selected unchanged. Raw annotation JSON permits syntax only: finite-number, safe-integer, valid-Unicode and plain-JSON/IJSON guards remain mandatory serializer/semantic checks. The broad raw-JSON schema is not claimed to encode or execute every guard. Arrays or primitive annotation syntax cannot supply missing physical facts or attest origin/Current. Contradictory nonempty kind/unit/component still blocks under the existing rules.

Hash checks pass for the 28 selected artifacts, 31 manager-manifest entries, 97 accepted original final-manifest entries, 23 prior winning artifacts, 16 prior review-manifest entries, and supplied brief/top/predecessor pins. The two historical auxiliary RETURN pins remain explicitly classified as obsolete failures; their bytes and the accepted final97 freeze are preserved. The selected structural preparation record is correctly labelled preparation, carries an earlier prepared-field hash and does not substitute for the exact frozen V4 field hash checked here.

## Exact input snapshot

- Sealed brief RESULTS_CONTRACT_BACKCHECK_V3.md: `d88e716823626025e79776daef7827c0db0cac3d2a86be2f693af5dce7424bb3`.
- RESULTS RETURN_V4.md: `8fc4f4a4a788c2f36c2862429ff785cdc5b7fc86d1b4c47f2d57581447059daa`.
- RESULTS MANIFEST_V4.json: `2674cb84a67f9223a886f7d7acd8c55c370e2da308429bb0a86aadc558628937`.
- CONSOLIDATED_CANDIDATE_INDEX_V4.json: `fc87130ed6c98acf4913a24c0f21d141b655a6ebc9d553c35b20debf168953b2`.
- Selected schema: `5fb3fc2ec6a94f748781f525087956d22daa6b1bd15bd21a470d0bf6c42ed485`.
- Selected field contract: `3f4ea29ff3f1a490c59abea823a0f840b1ef61ced637ea0d4b0d7fbfb5e807f2`.
- Raw/canonical contract: `c611f71027c8e3343045e38ef01d7adb95959da8ce53dc505ae7ce7bc653fa1d`.
- 28 annotation expectations: `dc839f3165ca557e45381deec0853c1a902d9a85b6dfcb024b8a84a023d499bc`.
- Historical auxiliary pin disposition: `c2f00b4c037e61deecb25da2e344fe6e80832f3c3149fcfd952e04a6ef9430bc`.

Exact paths, per-file hash results, structural differences and all case comparisons are in HASH_BACKCHECK_V3.json, EXACT_CONTRACT_DIFF_V3.json and FOCUSED_INSPECTION_RECORD_V3.json under this review's _run_records. The brief is copied byte-for-byte there. Complete affected content was reviewed; unaffected semantic conclusions carry forward from RETURN_V2 rather than being relabelled as a fresh review.

## Handoff and limits

This is derivative review evidence against the approved V3 plan and selected V4 candidate, not authoritative decomposition truth or an implementation/runtime pass. Contract-review closure: PASS, all three findings resolved across preserved review history. Root owns candidate acceptance and the existing sole product writer's release. Implementation, serializer/semantic/hash/schema/Historical verification, independent source-diff review, DEC025, simple native lifecycle and formal applicability/publication gates remain as previously allocated. The nine legacy supported-variant throws and four legacy dimension differences remain explicit limitations; this annotation correction does not resolve them.

No product source, tests, builds, schema runtime, native/browser work, Git effects, network or delegation occurred. One display-only KeyError used fixtures instead of cases; corrected, then all 28 cases read. No preparatory expansion or unrelated rerun is requested.
