# Independent resource finding and repair

The independent reviewer found that the first candidate enforced Python
snapshot/document/request byte limits after serialization, and Rust's Value
API likewise checked canonical output size after writing it. A shared1MiB
string used nine times therefore allocated over9MiB before the intended8MiB
refusal. The reviewer retained its measured original probe in INDEPENDENT_REVIEW.
This was a genuine boundedness defect despite all earlier conformance checks
passing; those earlier source/tests/logs remain immutable.

The successor candidate adds exact JSON UTF-8/escape byte accounting during
snapshot traversal, before materializing output. Every alias occurrence pays
its full expansion. Python streams one bounded snapshot at a time into
cumulative escaped request admission instead of first collecting all serialized
snapshots. Raw text's UTF-8 size and the complete escaped envelope are counted
before allocation. Rust validates canonical scalar/key/container bytes before
calling its writer. The old paths and the original token parser are unchanged.

New controls reject the real9MiB alias example before snapshot json.dumps,
exercise cumulative requests before materialization with a declared smaller
local test budget (the production64MiB constant remains pinned), compare exact
preflight bytes with Python UTF-8 JSON encoding, and check Rust escaped-output
expansion and exact8MiB/one-byte-over boundaries. Full affected checks pass:
21Rust tests and54Python tests, including the unchanged547 cross-language tokens.
No protected numerical tolerance or historical profile acceptance was changed.
Independent repair backcheck is pending against SOURCE_FREEZE_02.json.


The independent backcheck confirmed the original allocation-order repair, then
found a related text-rendering omission:5,194,308bytes of valid text (a4MiB string
and200000 short1e20 tokens) expanded to8,594,308canonical bytes. The same parser
then refused that output. The original probe remains in INDEPENDENT_REVIEW.
SOURCE_FREEZE_03 routes the parsed-document method and text canonicalizer through
the same bounded output admission, returning an explicit Result from the new
public document method. Both received and canonical documents now fit8MiB;
no over-limit canonical result is returned. New controls exercise this genuine
short-token expansion and exact output-size roundtrip. Current full checks pass
22Rust/54Python/547tokens; no source/profile history or numerical criterion moved.


Final consistency repair (SOURCE_FREEZE_04): the reviewer identified that direct
Rust Value snapshots could still bypass the parser's cumulative path cap. The
implementation now shares one escaped child-path length/cumulative accounting
helper across parsing and snapshot emission, without allocating snapshot paths.
Text/document and direct Value rendering share the same output, depth, node and
path preflight. Python never returns a canonical document without this Rust
admission, and bounds its own serialization before process transport.

The consolidated parity matrix exercises Value→text re-entry at exact depth/node
boundaries and within two bytes of the cumulative path limit,32 mixed numeric/
Unicode trees, escaped-key amplification refusal, and actual Python snapshot,
text and batch result re-entry. Raw lexical restrictions still apply where raw
text exists; no routine can recover lost original tokens from a foreign object.
Full current checks:24Rust/55Python/547cross-language tokens, zero failures or
skips. Independent final backcheck remains separate from these implementation
checks. Earlier freezes/observations are retained, not rewritten as passing.


Final independent backcheck is CLEAR against SOURCE_FREEZE_04. The reviewer
verified final hashes/logs and focused boundary probes; its return and final
checked basis are preserved under INDEPENDENT_REVIEW. This status does not alter
prior failing observations or activate a producer/reader/profile migration.
