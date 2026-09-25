# Initial gate review findings

The four-file GATE_FREEZE_01 candidate has five actionable P2 findings. These
are synthetic gate/custody observations, not actual solver or oracle results.
The manager acknowledged the findings and owns one coherent repair. This review
made no implementation edits.

## GATE-R1 — wrapper blocking severity is ignored

`qualification_gate.py:159–169` routes ControlledExport findings through a
lowercase-only diagnostic check. The actual wrapper uses uppercase `BLOCKING`
and `WARNING` (`headless/src/redaction_binding.rs`), unlike the runner's
lowercase severity enum. A packet with `blocked:false`, a `BLOCKING` finding and
`summary.blocking_count:1` currently returns
`all_required_assertions_matched`. An inconsistent or incompletely applied
redaction wrapper must not become a passing development observation.

`BOUNDARY_PROBES.json` contains the executed synthetic reproduction. Validate
the supported wrapper vocabulary and relevant blocked/summary contradictions,
while separately preserving the runner/mechanics severity contracts. Add a
known uppercase warning positive and uppercase blocking negative.

## GATE-R2 — recorded input digest need not describe consumed stdin

`qualification_process.py:35–50` writes `stdin.bin` in the process's working
directory and opens that writable named file as its stdin. The final
`stdin_sha256` is calculated from the original Python `input_bytes`, without
checking the bytes actually available through that file. A short synthetic child
overwrites `stdin.bin` before reading stdin, reads different bytes, and emits the
otherwise matching packet. The complete gate still passes and records the old
input digest. The executed probe records unequal actual-read and reported hashes.

Deliver the bound immutable bytes through a supervised byte stream or equivalent
nonmutable input object while draining output, and verify the retained input
artifact separately. Do not infer consumed-input identity merely from a digest
of an earlier buffer. Retain timeout/interruption/deadlock controls when changing
the transport. The post-run source/reference/criterion hash check should also
remain bounded if a bound file grows; its current `read_bytes()` has no such cap.

## GATE-R3 — nonzero reference tokens can become zero before comparison

`qualification_gate.py:64–68` parses reference/criterion/result floating tokens
with `float()` and checks only finiteness. A reference value spelled `1e-999`
therefore becomes zero. At the test's unchanged exact-zero tolerance, an observed
zero passes against that nonzero source token. Hashing the original reference
retains spelling but does not repair the lost expectation used by the comparator.

The executed full-gate probe preserves this false pass. Refuse nonzero lexical
underflow before it can erase an expected value, criterion or observed value.
Keep genuine zero and representable subnormal handling explicit; do not switch
profiles, loosen a criterion or infer a new scientific producer contract.

## GATE-R4 — contradictory result dimension is discarded

`qualification_gate.py:283–285` compares kind/unit/entity/basis/metadata but omits
an explicitly supplied result `dimension`. Adding `dimension:"force"` to the
selected displacement row still matches the selector's `dimension:"length"`
and produces an all-matched outcome. Pinned semantics can supply an absent
dimension but must not silently override contradictory received evidence.

The executed mutation is in `BOUNDARY_PROBES.json`. Either reject fields outside
the closed raw-row contract or validate any supplied dimension against the
selector/table. Preserve the actual legacy producer's absent-dimension case.

## GATE-R5 — combination selector uses a nonproducer basis token

`qualification_gate.py:250` permits `load_combination`, while the actual pinned
raw0.1 producer emits `basis_ref.ref_type:"combination"`
(`product_physics/src/lib.rs:9074`; 261 retained fixture rows use that spelling).
A selector faithfully copied from a valid combination result is therefore
blocked before comparison, while the invented spelling passes this admission
step. This is a source-contract mismatch, not a requested new solver capability.

Use the actual closed token `combination` with `load_case`, or explicitly
declare combinations unsupported without admitting a nonexistent token. Include
an actual-vocabulary positive and wrong-spelling negative. This finding is a
complete static trace and retained fixture inspection; it was not a new solve.
