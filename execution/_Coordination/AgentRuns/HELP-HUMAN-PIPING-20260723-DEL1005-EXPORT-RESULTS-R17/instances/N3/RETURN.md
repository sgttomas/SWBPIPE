---
doc_id: R17-DEL1005-N3-RETURN
doc_kind: coordination.agent2_return
status: block
created: 2026-07-23
candidate_version: 1
---

# N3 independent refutation — candidate v1

Verdict: `BLOCK`.

## P1 — scalar-recursive redaction is not bounded for package bytes

Candidate v1 required `container_bytes` as a JSON `Vec<u8>` array while adding
`report_package` to the current scalar-recursive local-private walker.

The walker visits every array scalar and emits a decision plus finding for
every private scalar. The accepted R16 packaged-native report package is
3,189,621 bytes. Candidate v1 would therefore create at least 3,189,621
decisions and findings for the byte array alone, plausibly expanding output to
multiple gigabytes and failing memory/runtime behavior. A small invented
witness would not prove the accepted live-size route.

## Minimal v2 repair

- Preserve exact raw `Vec<u8>` JSON-array encoding.
- Classify the exact `report_package` subtree once without scalar descent.
- Reuse only `LOCAL_PRIVATE_INTENT_REQUIRED` and
  `PRIVATE_LOCAL_ALLOWED_WITH_WARNING`.
- No intent: one blocking subtree decision, `payload: null`, no file.
- Explicit intent: one warning subtree decision and the exact unmodified
  subtree.
- Add cardinality tests independent of byte length and a native-size
  package proof at least 3,189,621 bytes.
- Explicitly include this bounded aggregate classification in owner adoption.

No new path is required; only the existing candidate text and the
`redaction_binding.rs` matrix effect need amendment.

## Validated positives

- Frozen basis, branch, sole Remaining row, and ten satisfied dependencies are
  correct.
- Shared-adapter extraction and the three explicit PKG-08 owner-gated rows are
  coherent.
- Linked identity and cross-binding predicates are fail-closed and cover the
  live reference set.
- DEC-065 structured JSON remains separate from desktop picker/atomic save.
- Verb, schema, package-member, benchmark/regression, lifecycle, and claim
  fences are explicit.
- All six registered checks, focused/native proof, one terminal sweep, fresh
  N5, and N5-before-W3 are correctly gated.
- Matrix: 24 unique rows; otherwise equivalent to the candidate.

No implementation was released.

