---
doc_id: R17-DEL1005-AMENDMENT-V2
doc_kind: coordination.candidate_amendment
status: proposed_pending_independent_refutation
created: 2026-07-23
candidate: CB-2026-07-23-DEL-10-05-EXPORT-RESULTS-001
---

# Candidate v2 amendment — bounded report-package subtree redaction

## Trigger

N3 v1 returned `BLOCK`: scalar-recursive traversal of a 3,189,621-byte
accepted package would create millions of decisions/findings and defeat the
candidate's bounded-output claim.

## Exact amendment

At the exact serialized CLI path `$.report_package`, the existing Rust
local-private controller classifies the entire subtree once and does not
descend:

- no explicit intent: one `block_export` decision/finding with
  `LOCAL_PRIVATE_INTENT_REQUIRED`; controlled payload null; no `--output`
  file;
- explicit intent: one `warning_only` decision/finding with
  `PRIVATE_LOCAL_ALLOWED_WITH_WARNING`; exact subtree retained unchanged.

All existing paths keep their prior scalar behavior. No export context, action,
reason, precedence, corpus case, route, schema, or package encoding changes.

## Required proof

- decision/finding cardinality is constant as package byte length changes;
- at least 3,189,621 package bytes survive explicit-intent control exactly;
- no-intent control returns exactly one blocking subtree finding and no file;
- existing redaction parity and every pre-existing runner route remain green.

No matrix path is added. The `redaction_binding.rs` authorized effect and
candidate v2 acceptance text are amended accordingly.

