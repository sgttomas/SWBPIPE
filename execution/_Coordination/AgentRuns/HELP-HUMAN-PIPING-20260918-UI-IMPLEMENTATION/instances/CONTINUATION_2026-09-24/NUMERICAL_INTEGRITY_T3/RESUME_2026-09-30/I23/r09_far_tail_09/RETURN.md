# I23 R09 far-tail09 preparation return

One frozen append-only proposal and two independent source copies prepared.
Start **2026-10-01 15:55:59 UTC**; return **2026-10-01T15:58:56Z**
(178.0 seconds, within12 minutes). No runtime or acceptance.

The sole new test selects TARGETED.lines().nth(5), checks the unchanged line
identity and calls existing run_targeted<4> at p128 with exactly three original
terms:1,2^-128,2^-428. Existing helper line102 must compare against the unchanged
expected token +80000000000000000000000000000001p0 (1+2^-127). Earlier line2
failures, selector/setup/refusal/compile failures do not earn credit.

Independent exact fractions place the sum strictly above halfway between1 and
1+2^-127 by2^-428. The unchanged mutant's seven-limb net trimmed to L+1=5 limbs
drops128 low bits, removes that positive tail and leaves the exact midpoint,
whose even endpoint is1. This is a fixed-corpus arithmetic prediction, not an
observed mutant result or a changed oracle. Generator/production arithmetic was
not executed.

BASELINE and R09 each preserve the complete116-file immutable40129 FK archive;
only wide_sum_tests.rs has the same append-only overlay. All232 file hashes,
original test prefixes, distinct inodes and containment checks passed. Targets
and sibling logs are empty. The original K4-M2 fault remains hash-bound,
unchanged and UNAPPLIED in both copies. Every original corpus/test assertion
and maintained source byte stays intact.

PROPOSAL.json binds exact filter, test/corpus/patch hashes, expected postimages,
qualifying helper assertion and future baseline/fault/returned-control commands.
ARITHMETIC.json and SOURCE_WARRANTS.json bind the original line and rational
warrant. COPIES/SOURCE_MANIFEST/CONTAINMENT/VALIDATION preserve the actual checks;
exact host paths are in owned logs/HOST_PATHS.json. RV29 must review before any
ROOT runtime transfer/slot/guard release. No Rust/build/test/solver/model,
Git/index mutation, maintained edit, expectation change or delegation occurred.
SHA256SUMS seals this additive packet.
