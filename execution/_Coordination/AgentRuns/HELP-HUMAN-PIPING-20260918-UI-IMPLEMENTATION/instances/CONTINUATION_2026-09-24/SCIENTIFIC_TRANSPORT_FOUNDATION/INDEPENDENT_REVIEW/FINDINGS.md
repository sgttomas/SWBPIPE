# Scientific transport foundation — initial finding

Reviewer `/root/physics_resume/joined_consumer_review` is an independent TASK,
original actual parent `/root/physics_resume`. ROOT directly assigned this
cross-manager review; implementation manager is
`/root/scientific_transport_manager`. No delegation or source writes occurred.
The initial source basis is `SOURCE_FREEZE_01.json` and its ten-file patch,
SHA256 `57a2bc688a9d85f0b7e32a60aafb6a68ed7cc7d7c8ee580672086564b6299614`.

## SJ-R1 / P2 — snapshot and batch byte caps are enforced after expansion

Status: confirmed on the original freeze; repaired in SOURCE_FREEZE_02 and
independently backchecked in `RESOURCE_REPAIR_PROBES.json`.

In the new Python `canonicalize_binary64_batch`, `_freeze_binary64` limits
depth/nodes but does not account for serialized bytes. The function materializes
every document with `json.dumps` before `canonicalize_binary64_text_batch`
checks each document's byte limit. The text-batch helper similarly constructs
the complete escaped request before checking its 64MiB size. A small snapshot
with repeated references to a shared string can therefore expand far beyond
the advertised resource boundary before rejection. Across a batch, the helper
can retain all expanded document strings before discovering the cumulative
request limit.

The independent bounded `probe_resource_order.py` used one 1MiB string with
nine references (ten value nodes). The actual helper allocated a 9,437,212
character JSON string before returning `BINARY64-JSON-BYTE-LIMIT`.
`RESOURCE_ORDER.json` binds the original adapter hash and observation. This
was a small allocation-order probe, not an OOM experiment. Increasing references
while staying below the node/item caps can exhaust process memory before the
refusal is reached.

The Rust `canonical_json_binary64_v1_value` has the analogous order:
`validate_value` checks types/depth/nodes, then `write_value` fully expands
escaped output, and only afterward does `output.len()` check the document cap.
An already bounded host string can expand severalfold under JSON escaping.

Remediation: account for new-profile serialized document bytes before snapshot
materialization, enforce cumulative escaped envelope bytes as items are
considered, and bound the Rust snapshot writer or preflight before its output
allocation. Include exact-boundary and alias/escaping amplification controls.
The original-text parser's document and preallocation pointer limits should
remain unchanged, as should all old-profile APIs and failures.

The original shared-string probe now refuses before full snapshot
serialization. Raw and snapshot batches refuse cumulative expansion before
full request serialization, and small exact byte accounting checks agree with
actual UTF-8 encoding. Rust snapshot preflight was inspected with its executed
escaped-output/exact-boundary controls. No source/parser or legacy path was
changed by this repair.

## SJ-R2 / P2 — text canonicalization can emit a document it cannot read

Status: resolved in SOURCE_FREEZE_04. The byte-size path was repaired in
SOURCE_FREEZE_03 and independently checked; final shared pointer accounting
closes direct Value parity and has current executed Rust/Python controls.

`Binary64Document::canonical_json` and
`canonical_json_binary64_v1_text` still call the unrestricted writer after
checking only the original input size. Short exponent tokens can grow during
canonical rendering. The new snapshot path checks canonical output size, but
the text/document path bypasses that check.

The actual rebuilt CLI accepted a 5,194,308-byte input containing one 4MiB
string and 200,000 `1e20` numbers (200,002 value nodes, within all input limits).
It emitted 8,594,308 canonical bytes, exceeding the declared 8MiB document cap.
Feeding its exact output back to the same CLI then failed with
`BINARY64-JSON-BYTE-LIMIT`. `CANONICAL_OUTPUT_LIMIT.json` records byte counts,
exit codes, canonical checksum and binary identity;
`probe_canonical_output_limit.py` preserves the bounded original reproduction.
No product, native UI or heavy build was executed.

Remediation: apply bounded canonical-output admission to the parsed document
method and text helper before writing. A fallible direct document method is
necessary so callers cannot bypass that limit. Include short-number expansion
and exact output-boundary controls. Keep original token conversion, raw ingress
checks and all historical APIs unchanged.

`CANONICAL_OUTPUT_LIMIT_REPAIR.json` records the successful actual CLI
backcheck: the original expansion now refuses with no stdout, while an exactly
8MiB canonical document succeeds and round-trips unchanged. Static follow-up
identified one remaining instance of this same resource-parity issue: direct
Rust Value preflight does not account for `MAX_CAPTURE_PATH_BYTES`, so a Value
with a 4MiB key and four numeric array children can serialize successfully even
though the documented original-text test rejects those bytes at the cumulative
16MiB pointer budget. The manager accepted sharing length-only pointer accounting
between the direct Value and text paths. No new numeric semantics are requested.

No other actionable issue has been found in numeric conversion, lexical count
checks, Unicode, profile/CLI selection or historical preservation. Final
clearance and checked hashes are recorded in `RETURN.md` and
`FINAL_CHECKED_BASIS.json`.
