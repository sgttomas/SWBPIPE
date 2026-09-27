# D1 — T3 numerical methods and scale design

HELPS_HUMANS-style design TASK. Read `_COMMON.md` first.

## Purpose

Write the T3 design for the numerical core: general solve accuracy, general retained-source recovery, range and sparse scale. It must be concrete enough for ROOT to select and for a manager to implement in slices, following the T0R pattern (`DEFAULT_ROUTE_DESIGN/DESIGN.md`). It is a proposal until ROOT selects it after independent review.

## Scope

1. **One general accuracy method (W1)** for M03 general accuracy, N05 ordinary accuracy and general retained-source recovery (`STAGE0_MAP.md` §2.1, §2.4, §2.5).
   - Start from the selected basis in `CORRECTNESS_DESIGN/CONTRIBUTION_PRECISION/CONTRACT.md`, with its refutation and prototype. Compare it against a relative-coordinate or basic-deformation reformulation, and against generalizing the merged exact-block method (order > 2, non-axis-aligned transforms, more than 256 DOFs). Recommend one, with evidence.
   - The method must cover larger, skewed and weakly coupled systems. State exactly which element, support and load families it covers at first, and how each excluded family is refused (the merged exclusions are listed in §2.5).
   - Say what happens to the existing identities: preview-physics-1 ordinary results, source-blocks-1 and physics-source-1. Whether the new method runs on the ordinary route by default, when it triggers, and what evidence it publishes. Propose, do not allocate, any new identity or version; ROOT reserves those.
   - Say how it interacts with M03-INTEGRITY-v1 and M03-PRODUCT-PREVIEW-EQUILIBRIUM-v1. No certified inertia or forward-error claim follows from operational screens.
   - Nonlinear and contact recovery is T5 (M06). Say where the method's boundary with T5 lies.
2. **Range (W2).** PHYS-R4: the subnormal transformation-error allowance refused in `structural::transform_roundoff` (`ENGINE_INTEGRATION/RETURN.md`, "PHYS-R4 applicability and arithmetic"). Evaluate exact power-of-two scaling so the scaled problem is normal-range before any refusal, and define the admitted range that remains, with its refusal diagnostic.
3. **Sparse scale (W3), M32.** One sparse assembly, reduction and reaction path (`STAGE0_MAP.md` §2.2), with bounded scrutiny and fallback behaviour, sparse/dense parity (small controls, several modulus bases, nonlinear paths), and a memory and runtime measurement protocol on sealed large models (`P/core/solver/performance_harness` is the likely home). Say how the dense fallback stays available and how its result is labelled.
4. **M03 residual items (W4).** The rigid-null witness for bodies with user-matrix or curved elements (`structural_adapter.rs:264`), and the SUP-17 wording at `PP:1362`.
5. **VP-ROBUST harness (W5).** Where the robustness cases live, how they are run through the public entry in both modes, what is compared (fields, reactions, member invariants), seeded faults, and how memory and runtime observations are recorded. Use the reference families in `STAGE1_PLAN.md` §3 (R1 writes them; do not write references yourself). Every comparison uses the unchanged relative form `|obs − exp| ≤ 1e-9 · max(|exp|, scale)` with the references' stated zero scales. Propose no new tolerance.
6. **Build feasibility.** If the method needs an arbitrary-precision or extended-precision backend, check what resolves offline and locked, for CI (`.github/workflows/`) and a macOS build, and what the per-crate lockfiles would change. Report facts, not assumptions. A system C library dependency (GMP/MPFR) must be weighed against pure-Rust options.
7. **Slices and file plan.** Use `STAGE0_MAP.md` §3. Separate what can be built before T1 merges (disjoint files only: kernel crates, performance harness, validation assets) from what waits for the merge (anything in `PP`, `source_recovery.rs`, `source_receipt*`, `pressure_runtime.rs`). Give each slice its write set, owner type and tests.
8. **Verification plan.** Frozen references (existing N/R/NP sets, T0R's where relevant, and R1's new families); a detection run on main before implementation (which quantities are expected to mismatch today); mutation controls that must fail; native witnesses (on the owner's Mac); hosted CI including surface 4; a clean DEC-025 sweep.

**Out of scope:** reader standing, joined eligibility and binding, the composite finalization fix, source-blocks re-homing, transport and display range. Those are D2's. If your method changes what readers must verify, state the evidence it publishes as an interface note for D2 in a separate section; do not design the readers.

## Basis to read

`STAGE0_MAP.md` (all of §1–§5), then the records it lists for M03, M32, M34, N05 and retained-source recovery, especially `CORRECTNESS_DESIGN/{NUMERICAL_IMPLEMENTATION,M03_RESIDUAL_SUCCESSOR_ADOPTION,NUMERICAL_REFERENCE}.md`, `CONTRIBUTION_PRECISION/**`, `NUMERICAL_POLICY_REVIEW/RETURN.md`, `COMPOSITE_ENGINE/{SELECTION,RESOURCE_POLICY}.md` and `DEFAULT_ROUTE_DESIGN/{DESIGN,ROOT_SELECTION,ROOT_RULINGS}.md`. Product source at `c61a540ea`: `P/core/solver/{frame_kernel,sparse_direct,nonlinear_integration,performance_harness}`, `P/core/product_physics/src/{lib.rs,source_recovery.rs,source_receipt*}`. T1's facade changes: `git diff origin/main...f3270ea79 -- P/core/product_physics`.

## Probes

Reading and standard-library Python need no permission. A Rust probe against the product (an out-of-repository crate, as in T0R's design probe) is allowed only after the manager says ROOT has released the host, and then only in `<scratch>` with `<t3-target>`. Record the probe source and its output in your run records, with `.txt` suffixes on `Cargo.toml` and `.rs` files so no tool treats the record as a crate.

## Write set

`T3/DESIGN_NUMERICS/**` only: `DESIGN.md` (the return), and `_run_records/` for probe sources, outputs, a `toolchain.txt` if Rust ran, and `SHA256SUMS`.

## Return

`T3/DESIGN_NUMERICS/DESIGN.md`, structured like the T0R design: recommendation in brief; the current state with citations; options compared; the recommended design per workstream; the interface note for D2; slices, order and T1 serialization; verification plan; what T3 completes and what remains per group; decisions for ROOT, with any owner-level question separated and given options plus a recommendation; sources, probes and limits. Then send the manager a SendMessage summary.
