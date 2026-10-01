# Commands and recoverable raw evidence

All Git reads in this assignment explicitly used GIT_OPTIONAL_LOCKS=0, including
Git subprocesses used for evidence hashing. No Git or index mutation was issued.

Actual source reads used `git show REV:PATH`, with some excerpts passed through
`nl -ba | sed -n RANGE`, and runner searches through `rg -n -C N PATTERN`.
SOURCES.json records every consulted blob, source-qualified revision, full-byte
SHA256/Git blob hash and displayed selection. These immutable original blobs are
the canonical raw source outputs; regenerate them with each listed read argv.
The VENV hash pass read the same full blobs for identity; it did not represent
unshown portions as separately reviewed. Full bytes are not duplicated here.

The first discovery was an overbroad `git ls-tree -r --name-only CONTROL T3`;
its displayed output was truncated. It was used only to locate the already-named
I21 brief. A second listing scoped to `K6C82cc9fa9d5:R` found source_01/02/03.
A further source-path listing was filtered by
`rg '(alloc|staged|main.rs|w1.rs|counts.rs|scale|runner)'` over H/src,H/runner,
VR/src,VR/examples,VR/runner. No source conclusion rests on a truncated listing.

One combined read display of checkpoint/return packets was truncated; required
returns/gaps were read again separately, and CHECKPOINT_0:140-340 was later read
explicitly. FORMULAS was displayed1-260 (whole226-line file). LIBRARY_CONTRACTS
was displayed1-195 only. The counts.rs combined display omitted20 tokens near
DECIDE_TRACKER_SETS; its exact value is not a new finding or premise here.
No failed read was treated as evidence. Each displayed command exited0.

Actual comparison commands included:
- git diff --stat 3cf296e36645d97e4c657c8ad1a6322bc4163f16
  cb13fcf3ea560c0d07ad9ac02dac78e9d2e06e00 -- K/adaptive.rs H VR
- git diff --name-only same revisions -- K
- The CHECKS.json repeated read-only name comparisons preserve argv and raw
  stdout/stderr/exit. Empty stdout means source continuity for that scope only.
- git rev-parse --show-toplevel in K6C returned the supplied K6C checkout.

The packet was created with the supplied VENV Python -B via a quoted here-document:
Path.mkdir/new write_text, hashlib.sha256, json, and subprocess git reads.
It is one-off evidence assembly, not installed or maintained host tooling.
The tool-call transcript retains the original commands and displayed returns;
this inventory is a compact replay index, not a fabricated verbatim transcript.

Task-origin instruction bodies are at CONTROL417517e671a1f5ecc0c296d16dd80f719a3e9cbb.
The live task also received Root AGENTS text and host/developer guidance through
conversation injection. SOURCES hashes the actually loaded committed instruction
bytes; it makes no byte-identity claim about serialization of the chat injection.
The parent brief and later steering messages are preserved in the parent/task
transcript, not assigned a fabricated filesystem hash.
