# Checkpoint C — stopped on frozen span-evidence mismatch

**Compiled; 22 tests passed, 1 failed. No retry or expectation change.**
The independent ledger assertion failed in
pc40_independent_absolute_work_ledgers_success_and_numeric_rejection:
actual max_span_bits=1076, frozen expected=1075 (new test line515).
The helper's H result, total18103, SumWork tuple(250,188,138,21) and context17506
assertions preceding it passed. This is an accounting-evidence discrepancy,
not a false-publication finding or permission to change a frozen expectation.

## Fixed evidence and bounded hypothesis

The test follows the independently frozen H=1+h, b=2 path, h=2^-1074.
Frozen LEDGER.json SHA256:
89e088c16e3a2f6edd0d927922f8a4f25984466015611a7dabe6871bf01ba15d;
ledger seal:
8fe24165f9905950280a78c672bada583d819d3946b95abcaa28d574ff38209b.
Both remain unchanged. The maintained assertion is still1075.

Pinned wide_sum.rs:219–244 defines span as new_high−new_low+1 over incoming
terms before cancellation and merges max_span evidence by maximum.
H alone spans exponents0 to−1074 (1075 bits). The b=2 predicate adds a term
with high exponent1 and H with low exponent−1074, giving1076 term-span bits.
This is the bounded source/fixture hypothesis for the discrepancy, consistent
with unchanged limb-count prices. ROOT/RV29 must disposition it independently;
I22 has not corrected the ledger, assertion or production.

PC40 aborted before its subsequent numeric-rejection and complete small-bound
tuple assertions. Separate PC41 budget-checkpoint and PC42 Span/Exponent
tests passed. Do not call the entire independent ledger closed.

## Source/test scope and reached checks

Actual live HEAD: c6e7d35c048f36d45b78184dcf73ce0c75879c95 (records-only over dd1).
Frozen production/probe basis remains dd1f70d8ba85b19f7d948bca6ee08a44bbb12ae1.
adaptive.rs SHA256 remains
4e5618a8d809e6ffa4ddd73024c206cc45eac6d5404a2f2034674552e1a95db9;
structural.rs remains8059cdcf4b16b3ccc989f66321991fc3a5ec66f4441c476e5af9827bd50b837d.

Only publication_tests.rs changed, SHA256
57c759928d251a530286c72bc9e2cd08a8c79a62380dfcf93245780bee517d68.
TESTS.diff preserves the full authored change. It adds fixed ledger/budget/
terminal checks, both independent sharper-rounding directions, exact decimal/
threshold/O9/prescription checks, canonical pairing/shape/source/floor/tag,
draft/drop/clone/combination and closed-row error-family checks, and three
fixed extra-source controls with independently frozen equilibrium expectations.
Only their three load IDs were aligned; no primitive bits/old truth changed.

The 22 passes include C17's authored regression, both sharper rounding directions,
budget/terminal tests, pair/floor/draft/combination tests and the fixed-extra
row test. The latter allows named certificate ceiling nonselection and does not
establish that every extra source published or passed an external comparison.
No assertion was weakened to obtain these results.

Remaining planned coverage includes full completed PC40 validation after owning
disposition; focused malformed/partial-draft and source-ceiling discriminators
beyond the present combinations; PM omission/whole-counter mutants; full external
all-row bare-b comparisons, source/policy binding and protected availability.
This authored expansion does not assert exhaustive PC01–46 closure.

## Runtime

C stage started06:57:42 UTC, boundary07:42:42 UTC.
One FK job launched07:05:35.670Z, tool PTY56099; exit101 after compilation.
23 tests ran:22 pass/1 fail, no ignored,367 filtered, test time0.11s.
Command9.54s real,14.99 user,0.64 system; max RSS1,255,702,528B; zero swaps.
Exact argv/environment/tool completion and guard/source reads are in COMMANDS.json.
Used existing PID5387 guard, installed1.97.1, offline/locked/-j4, incremental0,
two test threads, own a1-target. Guard was present after the job; no owned
compiler/test process remains. No hard cap or automatic deadline is claimed.

Full FK.stdout is unchanged; FK.stderr.portable changes only machine roots.
Raw scratch hashes:
- stdout4bf2600cc13c34f8015c76bae976263a3bb8ec4cae60a4fb8f8b7b766ad52ff1
- stderr74f231943aa3d3434e0ccb9c31de591fd0561dd7c483e187973241f7cdf44812

## Explicitly unrun and unchanged

H k6b_export: UNRUN in C. Immutable-source probe: not copied/built.
C's external C17, B01–B16 and EXTRA-FM/MF/ZR sequence: all UNRUN.
C18–C24 remain UNRUN. No PM mutant, broad H/VR, observation generator, scale,
10k, timed-performance or host-tool job. The optional KF1/A3B evidence-only
patch remains undrafted; no protected old test/golden was touched.
Frozen bare-b CLI was read; no repaired output was sent to it.

No production source, existing protected test, oracle, ledger, prior seal or
Git/index mutation. New writes are this packet, new-test file, owned scratch
logs and normal Cargo artifacts in the granted target. Return for ROOT/RV29
disposition before any continuation. A1 stays BLOCKING.

