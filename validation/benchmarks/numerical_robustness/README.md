# VP-ROBUST: `numerical_robustness`

R1's frozen references (T3 `REFERENCES/references.json`, sha256 `7b176dbb…`)
checked against the W1a kernel method (`frame_kernel::structural::retained_api`),
independently of K4's own tests. This crate is T3 slice V-K, the kernel lane
(T3 D1 §4.10). The product lane (V-P) extends it later.

## What it checks

- **The kernel lane.** Every R1 case of RF-CHAIN, RF-SKEW, RF-WEAK, RF-LARGE,
  RF-INVARIANCE, RF-RANGE, RF-ZERO, RF-FINITE, RF-MECH and RF-CANCEL (without
  its three UDL cases, which are W1b's) runs through `solve_case`. Cases with
  `needs_directional_spring` use the kernel-only `DirectionalSpring`.
- **The predicate.** Each R1 row is judged by the unchanged predicate
  `|obs − exp| ≤ 1e-9·max(|exp|, scale)`, with R1's class scales. RF-CANCEL
  uses its binding net-governed column.
  - The predicate is decided exactly: R1's decimal strings against the
    published binary64 values (`src/exact.rs`).
  - Bending magnitudes are decided without forming a square root.
  - Twist and extension are derived as `T/k_t` and `N/k_a`, never
    differenced.
- **Each row's outcome.** A row passes or fails, is `not_covered` (below the
  zero-scale floor R·S\*, R = 2^-34; never a pass), is `pass_absolute_range`
  (the expected value lies outside binary64's range), or is a structural zero.
  - The not-covered set must equal the committed list,
    `cases/not_covered.json`: RF-WEAK 46, RF-CANCEL 3, RF-SKEW 2.
- **The negative controls.** Every discriminating negative control of R1 must
  fail the same predicate. RF-MECH's mechanisms must be refused, or end
  unresolved, with no rows.
- **Expected-unresolved cases.** Every other R1 case must be selected. The
  exceptions are the cases on `cases/expected_unresolved.json`: RF-RANGE-THIN-A
  and THIN-B, whose stiffness ratio EA/(12EI/L³) of about 2^507 is beyond W1a's
  512-bit candidate ceiling (ROOT's ruling; K4's generator gives the same
  schedule).
  - A listed case must end unresolved with no rows. Its rows are counted apart,
    never as passes.
  - A case leaving or joining the list fails.
- **The binary64 parity checks.** RF-LARGE (up to 100 members) and RF-MECH also
  run through the binary64 sparse gate, for D1 §4.8's parity items 1–3.
  - The pattern assembly must equal the dense assembly bit for bit.
  - The M03 outcome class must be the same in both modes.
  - Where both modes are Passed, the displacements must agree within the
    DEC-053 basis (1e-9 of the dense magnitude).
  - RF-MECH is refused in both modes.
- **The equality checks.**
  - K4's canonical source bytes must equal the generator's, built from
    `references.py --model` (the adapter check).
  - K4's RCM port must equal `sparse_direct`'s.
- **Per-case records.** Each case's outcome, precisions, attempts and work
  are recorded in `observations/kernel_lane/`, for ROOT's W1 limits.

## Files

- `cases/gen_vk_cases.py` is the generator, standard library only. It reads
  R1's pinned files, which CI never checks out, and writes `cases/*`.
  - `python3 cases/gen_vk_cases.py --check` regenerates in memory and compares
    with the committed files.
  - `--large <dir>` writes the RF-LARGE models at 1,000 and 10,000 members.
    These run as examples only, and are checked against
    `cases/large_models.sha256`.
- `observations/kernel_lane/<family>.json` are the committed per-case records.
  `invariance.json` holds RF-INVARIANCE's cross-variant observations and
  `parity.json` the binary64 parity observations. Both are recorded, not
  asserted.
  - `cargo run --release --example vk_records -- --write` regenerates them.
    That is a decision, never done by CI: the lane tests compare the records
    and never write them.

## The seeded faults (the mutation run; never CI)

FK carries V-K's seeded faults behind its `mutation-controls` feature.
- **The sites:** every site is gated by
  `#[cfg(any(test, feature = "mutation-controls"))]`. The selector is in
  `retained/seeded.rs`.
- **Selection:** a site is inactive unless the environment variable
  `FK_SEEDED_FAULT` names its fault. An unknown id panics.
- **Enabling:** this crate's `seeded-faults` feature is the only thing that
  enables the FK feature. `tests/feature_guard.rs` checks that no other
  manifest names it and that CI passes no features.
- **The kill matrix:** `runner/run_seeded_faults.py --out <dir>` builds once
  with the feature, then runs the tests with no fault first, then with each
  fault, then with an unknown id. Each fault must fail at least one test.
  - The matrix is committed in `observations/seeded/kill_matrix.jsonl`.
    `--from-logs` rebuilds it from an earlier run's logs without running.
  - `observations/seeded/evidence.json` holds the detail of the evidence-level
    kills (VK-R28's release selections, VK-F07's RF-MECH outcomes). It also
    lists the D1 §7.3 items that are not seeded, each with K4's killing
    control.
- **The products:** `runner/check_fault_sites.py <base revision>` checks that
  removing the gated items from FK leaves only added comments. With the
  feature off, every product build is FK's base code.
  - `--allow-commit <rev>` also allows that commit's own patch lines, each at
    most as often as the patch has it. This lets the base be main while K6b's
    A0 export is on this branch but not yet on main.

## The harness mutants (checkpoint C; never CI)

`runner/run_harness_mutants.py --out <dir> --copies <dir> [--rev <rev>]`
mutates this crate's own harness: the engine, the floor, the lane, the
adapter (both its Rust half and the generator's path A) and the feature
guard.
- **Isolation:** each run gets a clean `git archive` copy of the piping tree
  at `<rev>` and its own fresh target. Cargo runs one job at a time.
- **Controls:** NONE runs first and must pass every test. NONE-GEN
  regenerates the case files with the unmutated generator; they must come out
  byte-identical, and every test must pass.
- **Mutants:** each is exact text edits with checked match counts, and
  `--dry-run` checks them against `<rev>`. A generator mutant regenerates the
  case files before the tests run.
- **Verdict:** a mutant must build and fail at least one test. A survivor is
  a stop.

## The scale runs (checkpoint B; examples, never CI)

RF-LARGE runs at 100, 1,000 and 10,000 members, one model per release
process.
- **`examples/vk_scale.rs`** runs one model. It checks the model file's and
  K4SRC's sha256, prints W1's O(nnz) counts and the admission estimate
  (`src/scale.rs`), then runs the case through the CI lane's code. It prints
  the report, including C9's floor set from the complete solution, the
  per-case record, RCM equality and the binary64 sparse class.
  - A counting, capped allocator records each phase's heap peaks.
  - An estimate above half the heap cap is refused (exit 3).
- **`runner/vk_scale_runner.py`** uses K6's runner by path for the wrapper,
  the RSS watchdog, the quiet-host wait and the admission rule. Its commands
  are `--counts` (no solve), `--plan` and `--run V1|V2|V3`, where V3 (10,000
  members) runs only with `--approve-10000`.
- **The models** at 1,000 and 10,000 members come from
  `cases/gen_vk_cases.py --large <dir>` and are checked against
  `cases/large_models.sha256`.
- **`tests/scale.rs`** checks the counts against K4's `StorageCounts` in the
  committed records.

## Running

```
cargo test --offline --locked --manifest-path validation/benchmarks/numerical_robustness/Cargo.toml
```

No threshold, time or memory bound is asserted anywhere. Time is an
observation of the scale runs only.
