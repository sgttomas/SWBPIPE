# Legacy profile refutation — early source diagnosis

Status: COMPLETE_WITH_FINDINGS; not a final candidate review or acceptance. Ephemeral Agent 2 → HELP_HUMAN; configured gpt-6-astra/high; delegated-harness-native; non-delegation/write scope instruction-asserted. Product source was read only; no product tests, builds, browser/UI, network, Git mutations, or delegation were executed. Independent Node/Python serializer calculations were executed without importing product code.

Inspected mutable compatibility source was captured at **2026-09-15T01:28:46.679531+00:00**. Exact bytes, origins, lengths, and SHA-256 values are in `EVIDENCE_MANIFEST.json`; `snapshot/` retains those mutable bytes, `baseline/` retains the inspected `eff9a58dd712ff9673fa26b9fa809a2f725f6f97` producer/schema bytes, and `context/` retains supplied instructions and authority. Findings below reference these captured source line numbers, not a later writer state.

## Actionable findings

**F1 — P2: an unknown original desktop locale can cause false corruption.** `HistoricalRunContext.tsx:21–27,63,65,69,85,87` invokes the reopening runtime's default `localeCompare`. The historical producer likewise used an unspecified default locale; saved records do not carry its locale or exact serialized result/row preimages. Therefore two legitimate runtimes can serialize the same received object differently. This affects arbitrary nested metadata keys and ordering of non-ASCII result IDs. The current unequal-digest branches report `mismatch` without establishing that the reopened serialization is the historical one.

Minimal independently calculated vector:

| Locale | Serialized `{z:1,"ä":2}` | SHA-256 |
|---|---|---|
| `en` | `{"ä":2,"z":1}` | `aebaf850cdb107bb9d7d528ee0c02528804edaac037352a39fb8ddef450fd003` |
| `sv` | `{"z":1,"ä":2}` | `7832a5d6150a56da1a4f0c8fa00c26a7350389b0fc8696707cd2abbbd32be0c1` |

Reproduction uses `JSON.stringify(Object.fromEntries(Object.entries(value).sort(([a],[b]) => a.localeCompare(b, locale))))`; place those keys inside a result row's metadata to exercise the producer's recursive rule. This is independent serializer evidence, not an executed product fixture.

Practical disposition within the existing contract: retain the locale serializer. An exact hash match for an established profile/projection verifies the candidate preimage; an unequal reconstruction needs `unverifiable` when original collation is unknown and ordering is ambiguous. Keep true `mismatch` for exact supplied preimages or an established unambiguous serialization basis. Do not substitute JCS, rewrite claims, or require new runtime output for source-only fixtures.

**F2 — P2: desktop profile admission is weaker than the declared exact abbreviated contract.** `HistoricalRunContext.tsx:41–52` recognizes the profile from labels and absence of only `created_at`, `solver_version`, and `provenance`. It does not establish the strict desktop envelope or reject hybrid/malformed profile evidence. For example, remove `professional_boundary` from the otherwise valid desktop envelope built by the existing legacy test helper: none of the checked fields or legacy reduced preimages changes, so the same match remains reachable. The exact desktop schema requires that field. The open path `projectService.ts:541–552` returns the typed native payload directly or clones browser state; this TypeScript annotation is not validation. Recognize the complete profile (or equivalent exact discriminator validation) before claiming verification. Preserve malformed/ambiguous history and report explicit invalid/unverifiable evidence. Add a malformed/hybrid profile case rather than only the current full-Python exclusion test.

**F3 — P2: the exported Python verifier can verify a claim whose declared algorithm contradicts SHA-256.** `core/analysis_runs/legacy.py:15–34` reads only `claimed.value`; supplying an established profile plus bytes whose SHA-256 equals that value returns `match` even if `claimed.algorithm` is `sha512`, or a contradictory canonicalization/profile is present. This is an API-level source finding; current tests supply incomplete claims with only `value` and `canonicalization`, and therefore do not exercise full-claim validation. Validate algorithm, supported historical profile/label relationship, and required claim identity before reporting the received checksum as verified. Do not relabel historical `JCS`/`rfc8785_jcs` strings. Caller-established preimage provenance remains a separate requirement; a new hash of unrelated preservation bytes cannot establish an old claim.

## Additional bounded evidence gap

`HistoricalRunContext.tsx:49–52` makes every full Python 0.1 record and its record checksum unverifiable because JavaScript loses numeric type distinctions. That is warranted for numeric result/row preimages after a lossy transport, but not automatically for the frozen Python **record** projection. `records.py:_run_hash_payload` contains run/model IDs, status, result IDs, `source_dimension` declarations, diagnostic IDs and manifest ref/hash; it does not contain result numeric values. When an exact Python profile and those original fields survive, the record claim can be independently checked while the result claims remain unverifiable. The current Python helper can check a supplied projection, but the desktop consumer does not dispatch that path. Treat this as a targeted coverage/implementation gap against the available-preimage requirement, not evidence that any supplied historical result payload must be regenerated.

## Exact producer distinctions and verification rules

| Evidence | Python 0.1 | Desktop 0.1 |
|---|---|---|
| Envelope | Full strict run: created time, solver/settings/unit refs, diagnostics, libraries/rule packs, run and row provenance; physical container object | Abbreviated run without those fields; `physical_project_container: "TBD"` |
| Own result/row/record labels | `SORTED_COMPACT_JSON` in frozen producer | `rfc8785_jcs` in frozen producer; historically misleading and retained |
| Manifest label | `JCS`, copied established manifest hash | `rfc8785_jcs`, copied established manifest hash |
| Serializer | Python sorted-key compact JSON, `ensure_ascii=True`, UTF-8 | Recursive `Object.entries` locale sort → `Object.fromEntries` → `JSON.stringify` → TextEncoder UTF-8 |
| Result preimage | Deep-copied received mechanics envelope, no dimension insertion | Historical `bindSourceResultDimensions` enriched envelope |
| Row preimage | Complete original row | Complete enriched row |
| Record projection | `input_manifest_hash`; `result_dimensions` entries use `source_dimension`; Python code-point sorting; no `load_basis_refs` in reduced payload | `input_manifest_sha256`; dimensions use `dimension`; `load_basis_refs`; effective rule status; default `.sort()` for ID arrays but locale sorting for result-dimension/ref arrays |

Neither old record checksum covers the complete record. Uncovered historical fields must not be described as protected by that checksum. Preserve original misleading labels, dimensions (including old work-as-moment interpretation), status and hashes. Reopened records remain Historical even when a hash matches; missing manifest payload does not become current solve evidence.

Python numeric spelling matters: independently serialized `{"a":"café","z":1.0}` gives ASCII-escaped `{"a":"caf\\u00e9","z":1.0}` with SHA-256 `69b6b480ea24ca9b447c24010e473288a86a29bab1ee75fc545c714ca76d851d`; changing the Python value to integer `1` gives `7498fad05d17e51961ab7834fe2286af2fe9984a7ab22a747321a68747fcb26c`. The JSON text contains a single escape backslash; doubled backslashes here distinguish the written escape. Desktop `1` versus `1.0` has no original JS type distinction and negative zero serializes to `0`; that alone is not grounds to reject a known desktop preimage. ECMAScript integer-key enumeration also survives the intermediate sorted insertion: `{ "2":"b", "10":"a" }` stringifies in numeric-key order (`2`, then `10`), not naïve lexicographic concatenation.

Required judgment is per claim: established profile + exact projection/serializer evidence + equal digest → match; same established basis + unequal digest → mismatch; missing preimage, ambiguous profile/collation or lost required Python numeric evidence → unverifiable. Exact serialized bytes, when provenance establishes the original scoped preimage, are stronger than a reserialized object and do not require restoration of erased numeric types.

## Existing evidence and corrections

- The two newly supplied analysis fixtures contain the analysis records only. They do not supply raw received result payloads, numeric-type provenance or serialized preimage bytes alongside those records. They support source/schema checks; this report makes no runtime-output or whole-fixture digest verification claim for them.
- `test_legacy_hash_profiles_require_exact_preimage_evidence` supplies a Python object with an actual `1.0` float and an explicit desktop UTF-8 serialized preimage. Its missing-evidence and numeric-loss cases are useful, but do not establish claim metadata validation or historical locale reconstruction.
- The original aggregation defect (one status emitted as both result and record mismatch) was reported promptly. The captured source now separates result/record status, so that initial defect is **not** an outstanding finding here. Row mismatches still share result status; final review should assess wording against actual covered claim.
- The active 0.1 schema `$id` change is **authorized**, not a finding: root `COMPATIBILITY_SOURCE_RELEASE.json`, SHA-256 `68ec3c0be138eaa85b2ec5cd6ebfd8d371af7e04d9075ab6d219700f77ffe31d`, supersedes the frozen contract narrowly. The inspected active Python schema differs from the baseline only by `$id` and trailing whitespace; original bytes remain in the frozen baseline. Later consumer/lock fences were read and do not alter these legacy rules.
- `records.py` is byte-identical to the frozen Python producer. Mutable `previewService.ts` changes new production to 0.2; this diagnosis therefore inspected the actual desktop 0.1 builder through `git show eff9a58:...`, not an invented current legacy builder.
- Context construction returns original received objects and a separately recomputed preservation hash. No rewriting of old claims was observed in the inspected verifier. This source observation does not replace native unchanged-save/attachment qualification.

## Key evidence hashes

| Inspected item | SHA-256 |
|---|---|
| Frozen/current Python `records.py` | `709f3a16f01b617084dd8d522081138826cae5d390fa0fc56dec23cb6c1a6bb0` |
| Frozen desktop `previewService.ts` | `2c4d459e62689a24d831d88ab92770b7f923f63f3419947b0963e9d413b62800` |
| Captured `HistoricalRunContext.tsx` | `fb218e27886f35f8362d16aef59d035e111c3b378c954cc680c25c6c5a5d6950` |
| Captured `legacy.py` | `934925228fd355e78375b47499858c9c756046fd99634b999513e85ba1aa9e89` |
| Supplied Python 0.1 record fixture | `03040257859d1a82b2a05166f743451e97266c5bfe897a8b75ae886a5b275ebc` |
| Supplied desktop 0.1 record fixture | `b69214394e2f8183b50149aa0db0827bd5f94fee79f4dc2add00026ef9760860` |

## Handoff

Derivative diagnosis consumes accepted `eff9a58` producer truth, the approved plan/D-67, frozen manager expectations, and the superseding source release. It is not authoritative decomposition truth. Closure: bounded diagnosis complete; source acceptance remains open. Root routes F1–F3 and the numeric-free Python record coverage gap to the sole compatibility writer. No repair or final candidate verification is claimed. Rerun requirements: targeted affected checks after repairs, followed by the already-required fresh 100% integrated candidate review and owning verification; do not add unrelated gates. Remaining blockers are the unresolved findings on the captured mutable source and the still-required final review/qualification owned by root.
