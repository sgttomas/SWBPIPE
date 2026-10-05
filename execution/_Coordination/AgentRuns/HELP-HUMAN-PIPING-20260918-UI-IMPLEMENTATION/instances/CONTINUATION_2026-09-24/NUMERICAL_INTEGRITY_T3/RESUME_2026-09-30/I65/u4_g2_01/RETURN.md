# I65 U4 G2: return

**G2 is complete and ready for independent derivation review.**

| Deliverable | File | Result |
|---|---|---|
| D1 predicate, field-level cap table, refusal map, U8 fit | DOMAIN.md | done |
| T01/T10 build binding, identity facts, D-6 check design | BUILD.md | done |
| Residual closure | RESIDUALS.md | **T02, T06, T07 and T22 close at the caps; T03 and T08 close in part** |
| D-4 reconciliation draft | D4_RECONCILIATION.md | done; one sub-decision for ROOT (D-4b) |
| T20 under S1 | STACK_PLAN.md | done |
| U3/U4 interface | API.md | done |

The residual-closure results:
- **T02, raw request backing:** about 15.6–15.8 MB at the caps (ASSUMED layout).
- **T06, H_formation128:** heap is 0 by source.
- **T07, deep legacy-exact:** closed under the existing exact-boundary limits; about 34.8 MiB at the caps, everything live at once, ASSUMED layout.
- **T22, scalar admission:** every named site is checked and representable at the caps.
- **T03, nested typed owners:** the owner roster is complete; the census reads are G5's.
- **T08, text:** a 707-site template inventory and the spelling maxima are done; per-site multiplicity and the composite-Debug bounds move to G3, as planned.

## What ROOT should know

1. **D-4 needs one sub-decision, D-4b.** For an overflow fault, I recommend O → `work_counter_range` inside the accepted typed `receipt_failure` enum. That needs no schema or reader change. I34's `work_counter_overflow` token would need a schema addition.

   ROOT's in-grant disposition "Checked work custody in U1" (NUM `21fe3e923d`) already adopts items 1, 2 and 4 of D4_RECONCILIATION §3.

2. **Decision S-1, from STACK_PLAN §3.** I propose reporting R separately and also checking `E_mov,max + R ≤ M`.

   The proposed values are R = 64 MiB and witness k = 16 (4 MiB). The basis is that experiment 03 ran the full facade path for the milestone on 2 MiB debug test-thread stacks. That is an observation, not a bound.

3. **The D-6 design** (BUILD.md §2):
   - a PP `build.rs` writes a canonical identity text: rustc release, commit, host and LLVM; target cfgs; panic; profile and opt-level; debug assertions; encoded rustflags; package name and version;
   - a byte-exact mismatch with the registered list, or any false `const` layout witness, gives `ProfileStatus::Stale`. That refuses the permit, and the ordinary path runs;
   - the witnesses guard the 64-bit target, serde_json `arbitrary_precision` off, `preserve_order` off, and the Value and String layouts. They are never compile errors.

   **Stated residual:** a downstream workspace's `Cargo.lock` cannot be observed by a build script, so the consumer lock stays a reviewed record. This is mitigated by D1 admitting Direct only and by the native-exclusion test.

4. **The private layouts close without guessing.** Production code uses source-derived upper formulas: BTree Leaf_up/Internal_up, and the accepted hashbrown and Vec laws. G5's allocation-Layout witness tests measure the actual Layout in the qualified build and require measured ≤ upper. For `(String, Value)` the upper bound is 640/736, against the DWARF observation of 632/728.

5. **Two gaps found along the way, both now covered:**
   - **The census records totals but no maximum string or key length.** The 128-byte text cap needs `max_string_bytes` and `max_key_bytes`; these are added to G5 (DOMAIN §2).
   - **Experiment 02's emitter reads legacy `u64` work fields** (D4 §3 item 1). ROOT has dispositioned this for U1.

6. **NUM moved during the grant,** from `f32f51d2aa` through `a2c26cc885` to `21fe3e923d`. All three commits are records only; no maintained source changed (Git diff read). Citations hold at all three.

## Execution record

TASK I65 (Type 2) under ROOT. No descendants.

**Timing.** Grant 2 started at 21:29 MDT (2026-10-04T03:29Z) and returned at about 21:58 MDT (2026-10-04T03:58Z). That is well inside the 12-hour budget.

**Memory guard.** `WT/guard/memguard.sh`, PID 5387, seen running at the start, during the work and before sealing.

**What was not done.** No code changed, no Cargo or compile ran, and no solver, native or DEC-025 job ran.

**What was run:**
- `rustc -vV`, an identity read only;
- stdlib Python over source text and the fixture, kept under `_run_records/`:
  - `caps_arithmetic.py` and its output;
  - `recursion_scan.py` and its output (a lead scan);
  - `template_inventory.py` and `template_d1_prefilter.py` and their outputs;
  - `std_src.py`, which reads the installed rustdoc source pages;
  - `identity_sites.txt`;
- `ORIGINS.json`: 50 source and record hashes at NUM `21fe3e923d`, plus the toolchain identity.

**Git:** reads only (`log`, `status`, `diff --stat`/`--name-only`, `rev-parse`), with `GIT_OPTIONAL_LOCKS=0`.

**Writes:** only R/I65/u4_g2_01/. All paths are placeholders; a search found no machine path.

**Read for this grant:**
- the coordinator's G2 message and the rulings at RR:8865 ("U4 plan"), 8902 ("The owner decides D-3 and D-6") and the in-grant "Checked work custody in U1";
- the C1 §2 text, C2 §2, I34's design and API, the I37 RETURN and the I52 integration basis;
- I54 BOUND, REQUIRED_FACTS, COEFFICIENTS and RESIDUALS; I51 COMPOSITION and C0;
- the PP, FK, canonical_json and result_export sources listed in ORIGINS;
- the installed std `btree/node.rs` and `core/fmt/float.rs`, and the cached serde_json `de.rs`.
