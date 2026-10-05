# I68: U8-0 (the probe) and U8-1 (PP's witness tests)

TASK (Type 2). Read `BRIEFS/U8_COMMON.md` first; it binds you. You are the PP integration owner for U8 (D-5). Your plan sections are PLAN §1.1–§1.4 (U8-0 and U8-1) and decisions 1–5.

## Part 1: U8-0, the probe (records only; nothing maintained)

**Where:**
- a disposable `git archive` of the U8 head into `WT/scratch/i68_u8_probe/`;
- target `WT/targets/i68-u8/`;
- records in `NUM/R/I68/u8_probe_01/` (PROBE.md and SHA256SUMS).

**What to run.** Use the registered dev/test build, in both modes (SparseInteractive and DenseScrutiny), and go through the real Direct entry `run_linear_static_preview_value_with_retained_direct`, unless noted. Probe-only test code lives in the archive copy only.

1. **RV93's two real inputs** (`R/REVIEW_RV93/u3_grant2_01/evidence/probe/zz_rv93.rs:292–315`):
   - `first_load_only`: the milestone with `primitive_loads[0]` alone;
   - `tiny_spring`: support 1's stiffness set to 1e-300.
   
   For each, record the W1 fallback cause, the `cfg(test)` `I51_FROZEN_REFUSAL` line, the notice count, and whether the bytes equal `with_notice(plain, case, None)`.
2. **W6's input** (`retained_memory_witness_tests.rs:181–199`, PHYS-R4's cantilever). Record the native `UnresolvedReason` with a probe-only print. **This establishes W-C1's reason**, which no record holds today.
3. **The L = 0 variant:** the milestone plus one node that no member references, with a rigid support on it restraining all six DOFs. Record:
   - the ordinary outcome;
   - the admission report (inside D1? caps?);
   - the W1 outcome.
   
   **If a successor publishes,** run all three readers on it with its invocation (Python `core/analysis_runs/retained_precision.py`, Rust `result_export`, TS `retainedPrecision.ts` via vitest), and record each verdict and the receipt's coverage rows for body 1. These are expected to be input-derived or exact zero, with the stop and `has_data` as PLAN §1.2 states.
4. **W-C2's candidate pairs** (PLAN §1.3), each case run alone as a one-case request:
   - the two-body pair (the milestone body plus PHYS-R4's cantilever; case A loads body 0, case B loads body 1);
   - the fallback one-body pair (PHYS-R4's x-aligned cantilever; A axial-only, B transverse).
   
   Record whether A publishes a successor and B gives a Native fallback with the Ceiling reason. **Commit nothing for W-C2; B1 uses this record.**

**Stop rules** (decision 5 and PLAN §1.4):
- **If L = 0 is not admitted, or W1 falls back,** record the cause. L = 0 is deferred to B1, so skip L = 0 in Part 2, and make no producer change.
- **If neither W6 nor a pair case gives Ceiling,** record the actual reason. W-C1 then commits the real-input Native fact with that reason.

**Return** to ROOT after Part 1 with PROBE.md's sha256 and a one-line outcome per item. **Do not start Part 2 until ROOT confirms,** because the outcomes decide Part 2's scope.

## Part 2: U8-1, the PP witness tests (maintained test code; after ROOT's go-ahead)

**Your fence** is `WT/f2a-u8`, and only these files:
- `P/core/product_physics/src/retained_facade_tests.rs`;
- **only if L = 0 publishes:** new `P/fixtures/results/retained_precision_l0_successor_sparse_interactive.json` and `…_dense_scrutiny.json`.

**Do not edit** `retained_memory_witness_tests.rs` (I65's). Write W-C1's input inline.

**The tests** (PLAN §1.2, the "Tests and controls" row):
1. **`u8_real_input_fallbacks_append_one_notice`.** It loops over both modes and three variants: `first_load_only` (Candidate), `tiny_spring` (Preparation), and W-C1's input (Native). In the registered build it asserts:
   - the expected cause;
   - `ONE_RUN_THROUGH_G_C`;
   - `notices == 1`;
   - the bytes equal `with_notice(plain_variant, case, None)`;
   - admission refusal `None`;
   - `hooks::armed_names()` empty before and after.
   
   In an unregistered build it asserts the plain bytes and `ONE_RUN`.
2. **`u8_l0_isolated_node_publishes_pinned_successor`,** if L = 0 publishes. In both modes:
   - the envelope equals the plain run (B′);
   - `ONE_RUN_THROUGH_G_C`;
   - the pinned file and receipt sha256;
   - the publication is the successor;
   - it writes the fixture under an output variable, as `I61_U3G2_OUT` does.
   
   **Value controls:**
   - body 1's rows are input-derived or exact zero;
   - body 0's rows agree with the milestone's independent reference (U5) within the unchanged criterion. Report bit equality with the milestone; do not assume it.
3. **A D-U6-5 equality test,** if L = 0 publishes: the fixture is byte-identical to the live successor, by sha256.

**Controls and mutants,** each killed by an assertion, never by a compile error:
- the full milestone still publishes its pinned successor (the existing test, unchanged);
- restoring all three loads fails the Candidate assertion;
- dropping the notice fails the byte assertion;
- for L = 0, a corrupted body-1 row fails the pin.

**Suites to run** in the registered build, compared test by test with the U8 base head:
- PP `cargo test` (expect the known Mac `t13` only);
- result_export;
- runner/headless (expect the two known `load_reference` failures).

Then an unregistered (Stale) PP run, using a separate target directory with a non-empty RUSTFLAGS.

**Records:** `NUM/R/I68/u8_witnesses_01/`, with RETURN.md (changed files and hashes, test outcomes, mutants, controls) and SHA256SUMS.

## Budget and return

Part 1: 2–3 h. Part 2: 2–3 h. Return once after each part, with sha256s and anything ROOT must rule on.
