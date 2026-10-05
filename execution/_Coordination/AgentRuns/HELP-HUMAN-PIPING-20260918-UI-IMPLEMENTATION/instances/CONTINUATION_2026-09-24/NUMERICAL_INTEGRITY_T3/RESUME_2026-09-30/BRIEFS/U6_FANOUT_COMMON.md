# U6 fan-out: what every U6 unit shares (U6b, U6c, U6d, U6e)

Each unit is a TASK (Type 2) under ROOT (HELP_HUMAN, Agent 0). ROOT is the return path, and none delegates. **I66 owns U6 as a whole.** Unit authors own their units through repairs. I66 answers design questions about the plan, routed through ROOT.

## The basis

- **The plan:** `R/I66/u6_scoping_01/PLAN.md` (sha256 `8742d105…`): §1 (the inventory rows for your unit), §3 (standing), §4 (the receipt's survival) and §6 (your unit's fence, deliverable and tests).
- **The rulings** in `T3/ROOT_RULINGS_V1.md`:
  - "U6 plan accepted: D-U6-1 to D-U6-9";
  - "U6a verified and committed; fan-out";
  - "RV86 on U5" (the milestone's claim and its limits);
  - "The F2a readers accepted and fanned in".
- **The slice you build on:** U6a at `844448112f` (`R/I66/u6a_slice_01/RETURN.md`). Its fixtures are the byte-identical successor copies (sparse `ac6986b0…`, dense `6cd1d249…`) and `fixtures/results/retained_precision_carrier_cases.json`, the 14-case three-language standing-parity file.

## Binding rules

- **Standing comes only from the verified receipt, and stays `needs_recompute`.** No reader's completeness flag changes; that is U7.
- **A successor offered under a base identity is refused** (the downgrade guard, `RETAINED_PRECISION_DOWNGRADE_FORBIDDEN`). Each language must refuse it.
- **No carrier may present** the sharper stop-rule bound, or truth-enclosure by the extrema intervals, as a claim.
- **Existing identities stay unchanged:** the same dispatch, standing, freshness, binding and bytes as base. Prove it with your own sweep.
- **Nothing weakened.** Removing or narrowing a check to meet an expectation is a stop condition (workflow §2).

## Host

- **Git:** no Git writes. Reads use `GIT_OPTIONAL_LOCKS=0`. ROOT commits and merges.
- **Never:** installs, new tooling, or native, solver-at-scale or DEC-025 jobs. Cargo `--locked --offline` with `CARGO_BUILD_JOBS=4` and `RUST_TEST_THREADS=2`, one job at a time. Use existing `node_modules` and prebuilt WASM only, linked or copied, never built, and disclose how.
- **Scratch** under `WT/scratch/<your-id>_<unit>/`, never the system temp directory. The memory guard must be running.
- **Records** in `NUM/R/<your-id>/<unit>_01/`: RETURN.md (changed-file hashes, controls, mutants, findings), outcome files and SHA256SUMS, with placeholder paths only.
- **Stops:**
  - a contract reading beyond the rulings;
  - a write outside your fence;
  - an existing identity's behaviour changing;
  - a check removed or weakened.
  
  Report the stop, and continue the work it does not affect.
