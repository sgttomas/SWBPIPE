# P1 — detection run on main (pre-change baseline)

Execution TASK. Read `_COMMON.md` first. It runs only after the manager confirms two things: ROOT has released the host, and V2 has refuted R1's references with ROOT's acknowledgement (the references at the commit the manager names at spawn).

## Purpose

Record what main publishes today for every T3 reference quantity, before any T3 implementation. This is the baseline against which T3's repairs are judged, as T0R's detection adapter was (`DEFAULT_ROUTE_DESIGN/DESIGN.md` §9.2). It changes no product code and passes or fails nothing about main. It records outcomes.

## What to run

- **An out-of-repository probe crate** in `<scratch>`, depending by path on `P/core/product_physics` from an exported copy of `c61a540ea` (`git archive c61a540ea projects/chirality-piping/core ...` into `<scratch>`). Build with `RUSTUP_TOOLCHAIN=1.97.1`, `CARGO_INCREMENTAL=0` and `CARGO_TARGET_DIR=<t3-target>`, `--offline --locked` where the lockfile allows.
- **The captured public entry** `run_linear_static_preview_value_with_mode`, in both `SparseInteractive` and `DenseScrutiny` modes, for every reference case the product can author:
  - R1's families in `T3/REFERENCES/references.json`;
  - the existing N01–N09, R01–R07 and NP-A–NP-D cases in `P/validation/benchmarks/numerical_integrity/fixtures.json`;
  - V1's check L and the S11 cancellation cases (RF-CANCEL).
- **Input construction** from each case's stated model (use `references.py --model <id>` or `--full <id>` for large cases): invented properties, stated units, no library data. Where the product cannot author a case (for example a spring along a non-global direction), record `not_authorable` with the reason. Never approximate the case to make it authorable.
- **For each case and mode, record:**
  - the producer identity and semantic contract;
  - the per-case `numerical_quality` and diagnostics;
  - whether retained-source recovery was selected;
  - for every expected quantity, the observed value, the comparison `|obs − exp| ≤ 1e-9·max(|exp|, scale)` using the reference's stated scale, and pass or mismatch;
  - for every negative control, whether main's value matches the wrong answer;
  - the standing main would give the result (Current-eligible or not), by calling the Rust standing function on the envelope.
- **For RF-LARGE,** also record wall time and peak RSS per size, measured in fresh processes (`/usr/bin/time -v` or equivalent). This is the first M32 memory observation; label it an observation, not a limit.

## Expected outcomes to confirm or refute

Take them from D1 revision 2 §7.2 and `MANAGER_NOTES/S11_MAP.md`:
- N05-class accuracy misses on the ordinary route;
- order > 2, skewed and weakly coupled cases outside retained-source scope;
- range refusals (PHYS-R4-like);
- RF-MECH refused;
- S11 cancellation cases published as Passed with wrong values;
- the capture refusal of |x| ≥ 2^53 (V1-S5).

Report every unexpected outcome prominently, especially any Passed or Current-eligible result that mismatches its reference. That would be a new silent-wrong finding.

A Passed breach of an RF-CANCEL case with realistic magnitudes reopens ROOT's S11 no-interim ruling (`ROOT_RULINGS_V1.md`). Report any such breach to the manager at once, before finishing the rest of the run.

## Write set

`T3/DETECTION/**` only: `RETURN.md`, `results.json` (per case, mode and quantity), probe sources with `.txt` suffixes (`Cargo.toml.txt`, `main.rs.txt`), run logs, `toolchain.txt` and `SHA256SUMS`. Delete build output from `<t3-target>` when done, and keep free disk above about 8 GB.

## Return

`T3/DETECTION/RETURN.md`:
- a summary table by family: authorable, pass, mismatch, refused, not_authorable, each in both modes;
- every Passed-but-wrong result;
- the memory and runtime observations;
- what was not run.

Then send the manager a SendMessage summary.
