# I4: implement slice S11-F (exact load sums, facade side)

This is an implementation TASK. Read `_COMMON.md` first; this brief overrides it where they differ (writes and builds). S11 is ROOT's top priority.

## Purpose

Implement slice S11-F of the selected S11 containment, so that the product never publishes a Passed result built from an absorbed or cancelled load. That means:
- the exact per-case load ledger at every product producer;
- the switch of `product_physics` (`PP`) to S11-K's typed seams;
- the facade's exact recovery sums;
- the Sensitive mapping;
- T1's three source-recovery and receipt sites.

After S11-F, the pinned S11 exceptions of the "no Passed breach" gate are empty on both entries. It must never publish a silently wrong value, strip a load or feature, skip a test or raise a timeout.

## Governing basis (read in this order)

1. `T3/ROOT_SELECTION_DESIGNS.md` (the §4 hard constraints) and `T3/ROOT_SELECTION_S11.md`.
2. `T3/DESIGN_NUMERICS/S11_CONTAINMENT.md` **revision 5a.2** (`932698d7a`, sha256 `e6507587…`):
   - §2.1–2.3 (reach), §4 (the rule), §4.2 (contribution granularity), §4.3 (the force built only from the ledger, and the site test);
   - §4.4 (exact recovery sums), §4.5 (T1's sites), §5 (guard, scale, invariant), §6 (a detected loss means Sensitive, never a refusal);
   - **§8.2 (the S11-F write set, ordering and both entries)** and §8.3 (disclosure and the fixture stop rule);
   - **§9, S11-F tests F1–F14 and mutations M1f–M1l, M1n, M1o, M2–M5, M8, M9 and M12**.
3. `T3/ROOT_RULINGS_V1.md`: D-S11-1 to D-S11-4, S11B-1, the no-interim ruling, option (c), the S-H/S11-F ordering, the exception re-pin, and the S11-K approvals.
4. S11-K as merged on main (PR973, merge `3488a236a`), with its records under `T3/IMPLEMENTATION/S11K/` (on main): the typed seams, `ExactAccumulator`, `LoadLedger` and `AssembledForce`, and the option (c) variants and pins.
5. RV1's S11-K review, `T3/REVIEW/S11K_REVIEW.md` (sha256 `4ef2f948…`, in the numerics worktree), for the carried NOTEs below.
6. The frozen references: `T3/REFERENCES/` at `c0f14201c` (RF-CANCEL, with the net-governed "recommended" column binding), and `T3/GATE/S11_EXCEPTIONS.json` (88 triples in 13 cases captured, 140 in 22 typed).

## Base, worktree and branch

- ROOT creates `<s11f-worktree>` (suggested `/home/user/wt/s11f`), on a new branch `codex/piping-s11f-20260927` from **main at `3488a236a` or later**.
- Make no Git writes. The manager commits.
- **Build the two authority targets first,** before any Python run: `tools/serialization/build_checked_json.py` and `tools/units/build_units_authority.py`. **Never delete** `core/serialization/canonical_json/target` or `core/units/target`.

## Write set (S11 §8.2, plus the carried items)

- **`PP`** (`P/core/product_physics/src/lib.rs`):
  - the ledger at every §4.2 producer;
  - the switch to the typed seams: `solve_load_case`, with the force built only from the ledger (today `global_load_vector` at the force build), plus `solve_preview_reduced_system`;
  - E5, E7–E12, E15 and E16;
  - the Sensitive mapping;
  - the product-level site test (§4.3), which reads the `PP`, `FK`, `SA`, `nonlinear_integration` and `sparse_direct` sources.
- `P/core/product_physics/src/pressure_runtime.rs` (push group operands).
- `source_recovery.rs` and `source_receipt.rs` at T1's three sites (§4.5), plus T1's 0.4.0 prescribed-motion wiring onto the typed seams (KS1–KS3 through `StructuralSystem`).
- **Removal of the `&[f64]` product entry points** that S11-K added beside the typed ones, wherever no non-product caller remains. **Keep the nonlinear loop's `_binary64` legacy variants** (option (c), T5's), and keep I1's pins green.
- **Carried items:**
  - **The sparse_direct typed sibling.** Either add a typed sibling in `sparse_direct` for the product path, or show and record why SA's `factor_structural_ldlt` path makes one unnecessary. Either way, the site test covers it.
  - **The `global_load_vector` doc note.** In `P/core/loads/primitive_loads/src/lib.rs`, add a doc note that it is not for solve input (review and diagnostics only). If no product caller remains, say so.
  - **RV1-N1:** the site-table scan also detects self-assignment folds (`x = x ± …`, `x = checked_value(x ± …)`), in both `frame_kernel/tests/s11_site_table.rs` and your product site test. Reword the header to state the scan's real limits, and re-baseline the counts. Report any new hit.
  - **RV1-N5, before wiring the typed path:** make `audit_load_fidelity` non-fatal. Either accumulate with `ExactAccumulator` instead of radix-scaled expansions, or map an audit range error to "Sensitive, unaudited row". **A detected loss or an unauditable row must never turn a solve into `Err`** (§6). Add a test with a ledger row such as (1e80, −1e80, 1e-300).
  - **RV1-N6:** resolve the naming asymmetry of the public `evaluate_original_residual`, which is binary64 while its siblings are exact. Either rename it `evaluate_original_residual_binary64` and update its callers (`product_equilibrium`, SA gap scrutiny, the tests), or keep the name with a pin. **Tell the manager before you rename anything in `nonlinear_integration`.** That crate is T5-serialized, and a rename there needs ROOT's agreement.
  - **RV1-N8:** add positive twins to the `compile_fail` doctests in `structural_adapter.rs` (`solve_assembled`) and `nonlinear_integration/src/lib.rs` (`solve_active_set_frame_assembled`), compiling with an `AssembledForce`. These are doctest additions only.
  - **RV1-B-N1:** add `::solve(` and `::solve_assembled(` to the option (c) pin's exact entry points, and extend the scan to `structural_adapter.rs` outside the defining variants. Or state in the doc comment that the behavioural pins are authoritative. Prefer doing both.
- Tests in the touched crates; records in `T3/IMPLEMENTATION/S11F/**` in `<s11f-worktree>`.

## Hard constraints

- **S-H never lands before S11-F** (ROOT, `b6fe1eb75`). S-H (D2's capture fix) is **not** in this slice. The captured entry therefore still refuses |x| ≥ 2^53 at capture; F11's G = 1e80 cases run through the **typed entry** (headless `run_preview_in_memory_mode`), and through the captured route only once S-H lands.
- **Both entries.** S11-F's tests run through the captured entry (`run_linear_static_preview_value_with_mode`) and the historical typed entry, **including RF-CANCEL at G = 1e80** (F, M, ORTHO, INPLANE, UDL-W1e80) on the typed entry (F11).
- **The exception list is empty after S11-F** (F12). Run the frozen-reference cases of `GATE/S11_EXCEPTIONS.json` through both entries, in both modes, and show that none of the 88 captured and 140 typed triples remains a Passed breach under the unchanged criterion `|obs − exp| ≤ 1e-9·max(|exp|, scale)`, with the binding net-governed scale.
  - **Derive the expected values from `REFERENCES/references.json` (`c0f14201c`, sha256 `7b176dbb…`) with a committed generator** (standard-library Python), and record its input hash. Do not hand-copy values.
  - A breach outside the list is a gate failure, never a new exception.
- **P1's S11-PROBE-A** as product-level tests (F14), through both entries, with **exact expected nets** derived in the test (w = 3/10, L = 2, EI from the product's section). The sources are `T3/DETECTION/probe/*.txt` and `scripts/gen.py.txt` (committed on the T3 branch).
- **RV1's lesson for every pin.** Every pin (the site test, source pins, typed-seam pins) is backed by a behavioural test that first asserts, inside the test, that the two paths differ (the binary64 fold against the correctly rounded net, or the legacy path against the exact path), and only then asserts the outcome. **A text-only pin is not enough.** This extends §9's precondition rule (S11B-6) to every pin.
- **Option (c) stays.** The nonlinear active-set loop's closed-gap prescribed solves keep the legacy binary64 variants, and DEC-046 is unchanged.
- **No in-band marker** (D-S11-4).

## Conflict boundary with K-D5 (I3)

K-D5 (I3, branch `codex/piping-kd5-20260926`, based on K3a) also touches `PP`, at the single call site `solve_preview_reduced_system` (merged `PP:3965`). It switches that call to SA's new `solve_with_formation_check`, passing the curved macro elements and `selected = built.nonlinear_supports.is_empty()`. K-D5 also edits `FK/src/structural.rs` (the formation check in `finish_checked_factor`) and SA (`solve_with_formation_check` beside `solve`).

- **Landing order: S11-F lands first.** S11 is top priority, and S11-F's `PP` changes are broad; K-D5's are one call.
- **K-D5 merges forward** onto main after S11-F (and after K3a). At that merge, K-D5 re-applies its one `PP:3965` change onto S11-F's typed call: `solve_with_formation_check` takes the same typed force S11-F passes to `solve`. K-D5 also integrates its nonlinear pins into I1's module.
- **What S11-F must not do:**
  - touch the formation-check design;
  - change `finish_checked_factor`'s residual-gate structure beyond what N5 requires;
  - restructure `solve_preview_reduced_system` beyond switching it to the typed call.
  Keep that function's shape recognisable, so K-D5's one-line change re-applies cleanly.
- If S11-F must change `finish_checked_factor`, SA's `solve` signature or `solve_preview_reduced_system`'s shape in a way that conflicts with this, **tell the manager first**.

## Tests and mutations

- S11 §9: **F1–F14**, each with the precondition where §9 requires it.
- Mutations: **M1f–M1l, M1n and M1o** (one per `PP` E-site), **M2, M3, M4, M5, M8, M9 and M12**, at the required kill set G = 1e8 and 1e80. Record each patch, the command and the killing test. A survivor is a defect; never weaken a test to kill it.
- **The committed-fixture diff** (S11 §8.3): run every committed request under `P/fixtures/**` through base (main at your cut) and candidate, in both modes. **Any committed byte change stops the work** and is reported to the manager with its site, reason and size, before anything is regenerated. Regeneration happens only after ROOT's decision, and only by the actual producer. §8.3's expectation: committed force DOFs carry at most two contributions, so most envelopes are unchanged, but `pipe:P-120`'s four-term end-force sums (E5) and other sums of three or more terms may move in the last bits. Measure and report them.
- **Every existing suite** of the touched crates and their path dependents, with `--no-fail-fast`, including:
  - `product_physics`, `runner/headless`, `result_export`, `operation_applier`, `apps/desktop/src-tauri`;
  - `validation/benchmarks/*` (mechanics, nonlinear with DEC-046 unchanged, stress, physics_audit_regression, numerical_integrity);
  - the Python suites (load_reference, preview_physics, stress_neutral, qualification);
  - the desktop vitest and build if any fixture or reader moves. Link `node_modules` from `/home/user/wt/engine` only after checking that package-lock.json is byte-identical, and remove the links before the manager commits.

## Build and cargo

- `RUSTUP_TOOLCHAIN=1.97.1`, `RUSTUP_AUTO_INSTALL=0`, `CARGO_INCREMENTAL=0`, `--offline --locked`, and your own `CARGO_TARGET_DIR` (suggested `/home/user/wt/s11f-target`).
- **Cargo priority** (ROOT): RV2 (the K3a review) first. Then **you and I3 (K-D5) are equal and alternate by job**: one heavy job each in turn, and never two at once. Check `pgrep -x cargo` before each job. **Hold all cargo while a `run_evidence_sweep.py` process runs.**
- Keep free disk above about 8 GB. Prune only your own output.
- Run `cargo fmt` on changed files. Check whitespace with `git diff --no-index --check` per untracked file, or grep. **Make no Git index operations.**

## Disclosure and return

- **`T3/IMPLEMENTATION/S11F/CHANGE_RECORD.md`**, following `.agents/skills/chirality-change/SKILL.md`, per S11 §8.3. It covers:
  - which quantities move and why;
  - the non-cancelling bound (at most one rounding of the gross);
  - that no case changes status unless it was absorbing a load;
  - the measured fixture diff;
  - that there is no in-band marker;
  - that the exception list is now empty;
  - the carried items and how each was resolved.
- **`T3/IMPLEMENTATION/S11F/RETURN.md`**, with logs under `_run_records/`, `SHA256SUMS` and no machine paths. It covers:
  - the files changed, with line counts;
  - each §8.2 write-set item and each carried item;
  - the full caller list of every changed function (by lexer scan);
  - F1–F14 results;
  - the mutation table;
  - the fixture diff, and any stop report;
  - per-suite counts;
  - the toolchain;
  - what was not done.

Send the manager a SendMessage summary. Message the manager **at once** if the stop rule triggers, if a design item cannot be implemented as specified, or if the K-D5 conflict boundary cannot be kept. Don't improvise a different design.

## Addendum: boundary decisions (manager, 2026-09-27; accepted by ROOT)

1. **Typed PP:3965 and SA `solve` narrowed to `pub(crate)`.** PP's `solve_preview_reduced_system` calls `assembly.solve_assembled(…, &AssembledForce, …)`, with the function's shape unchanged. SA's `&[f64]` `solve` becomes `pub(crate)`.
   - **Conditions:**
     - a lexer caller scan across all crates proving no non-test caller of `AssemblyEvidence::solve` / `StructuralAssembly::solve` remains outside `nonlinear_integration`;
     - every dependent crate compiles and passes;
     - **the option (c) pins stay green unchanged,** and the nonlinear loop still reaches only the named `_binary64` variants. I1's behavioural tests are not edited.
   - **On EV4:** the narrowing closes the UFCS `AssemblyEvidence::solve` route **for callers outside `nonlinear_integration`** at compile time, as intended. Within the crate (where RV1's EV4 was written, in `lib.rs`), a `pub(crate)` item is still reachable, so there the behavioural pins remain the guarantee. Record it this way.
   - K-D5 (I3) is told that its public `solve_with_formation_check` must be typed.
2. **N6: no rename.** `evaluate_original_residual` keeps its name. It is backed by S11-K's behavioural pin plus a new PP site-test rule that the product never calls the public binary64 residual. **That rule gets its own behavioural test,** with a paths-differ precondition.
3. **N5: a per-row audit range error becomes a flagged, unaudited row, so the case is Sensitive.** `finish_checked_factor` never propagates an audit error.
   - Tests: a (1e80, −1e80, 1e-300) row (no `Err`, unaudited, Sensitive, never Passed), and audited rows bit-identical.
   - **No new envelope field.** If one would be needed, stop, and it goes to ROOT.
4. **No sparse_direct typed sibling.** The typed path goes through `prepare_assembled_structural`, then `factor_structural_ldlt(&PreparedSystem)` carrying the ledger binding. The reasoning is recorded, with behavioural site-test coverage.

## Addendum 2: F12 and the formation-class list (ROOT ruling, 2026-09-27, option (c); `db665f2cb`)

- **The S11 list is now 221 triples** (`GATE/S11_EXCEPTIONS.json`: 87 in 12 cases captured, 134 in 18 cases typed). The **7 formation and formed-term triples** move to `GATE/FORMATION_EXCEPTIONS.json` (14 rows with mode):
  - UDL-W1e80 th.S1.RZ (typed);
  - UDL-W1e8 th.S1.RZ (captured and typed);
  - F-G1e80-GnG-INPLANE Mb.M1.j and Mb.M2.i (typed);
  - M-G1e80-GnG-INPLANE Mb.M2.i and Mb.M2.j (typed).
- **F12, in its literal form:** after S11-F, the S11 list is empty on both entries and both modes. The gate fails on any breach outside **both** lists. Wire the harness to read both pinned files (by hash), and report each list's residual separately.
- **The 7 formation rows:** show that each is **bit-identical, or no worse**, between base (main at your cut) and candidate, in every mode. Record obs(base), obs(candidate), exp, ratio(base) and ratio(candidate). S11-F does not repair them; slice S11-G does (a formation-noise guard, designed separately).
- **F10:** runs under the test-only historical pressure scope, recorded as such.
- **N05:** confirm the transverse-tip budget overflow on base. If it is pre-existing, record it as routed to T3's N05 item. If it is a regression, stop and report.
