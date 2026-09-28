# M03 skew pin change record (I9, tests only)

This is the draft PR record for T3's skew M03 pin, following `.agents/skills/chirality-change/SKILL.md`.
- **Implementation:** I9, a TASK on the owner's Mac.
- **Detail:** `RETURN.md`.

- **Branch:** `codex/piping-m03-skew-pin-20260928`, from main `eb52114e9` (K1 merged).
- **Basis:**
  - `TASK_BRIEFS/I9_M03_SKEW_PIN.md`;
  - RV7's K2a review, finding B1 (`REVIEW/K2A_REVIEW.md`);
  - K2a's `RETURN_ADDENDUM_1.md` §1 to §1.4;
  - the work graph's routing of "the skew M03 pin" to K1's pattern-path M03 tests, missed at K1's merge (`IMPLEMENTATION/K1_MERGE/RECORD.md`).

## What changes

Tests only. There is no product source, fixture, reference, Cargo or lockfile change.

- **`P/core/solver/frame_kernel/tests/m03_skew_scope.rs` (new).** It forms main's pre-K2a local matrix operation for operation, since K2a's `local_stiffness` now refuses these formations. It uses FK's own orientation, `transform_global_stiffness` and `transform_roundoff`. Five tests:
  1. **Preconditions:**
     - the replica equals FK's formation bit for bit on normal inputs;
     - every member is 2^-39 m long;
     - K2a refuses every case by name;
     - the floor 2^-974.585 is behaviourally the axis-aligned floor.
  2. **The pin, on RV7's confirmed cases:**
     - The outcomes are that the axis member is refused everywhere, and both skew members are accepted from 2^-1030 to 2^-1050 and refused at 2^-1055 and in S6a. Every refusal is `Range("arithmetic outside normal range")`.
     - 4EI/L and 2EI/L are subnormal-derived and below the floor.
     - Their exact (integer) relative errors match RV7's figures and an independent Fraction check.
     - The least bound is in the coupling block, at RV7's figures.
  3. **6EI/L² is the limiting coefficient:** moving only 6EI/L² across the floor flips the skew outcome. The test also pins a second limit below RV7's cases: a product underflow inside the bound, on (1,2,2) at 2^-1055.
  4. **The same outcomes through both representations:** today's dense M03 and K1's pattern path, with the same allowances, are byte-identical in `Debug`, in two orders.
  5. **A behavioural guard:** it fails if M03 starts refusing the accepted skew rows or accepting an axis-aligned row. The accepted rows are pinned as today's documented limitation, not desired behaviour (K2a addendum §1.4).
- **`P/core/solver/nonlinear_integration/src/structural_adapter/k1_tests.rs`, one test appended (append-only):** the accepted rows through the adapter's `AssemblyEvidence` and K1's `SparseAssemblyEvidence`, in both modes, give identical outcomes. The pre-K2a matrix is carried as an explicit slot with its own evidence.

## Evidence (`RETURN.md`, `_run_records/`)

- **Suites** (Mac; frame_kernel, sparse_direct and nonlinear_integration, against a `git archive` baseline of `eb52114e9`): 0 changed, 0 removed, 6 added, all passing.
- **T9 (Mac-only):** 112 of 112 committed-fixture outputs byte-identical, base against candidate.
- **Mutations:** a NONE control, then 7 mutants of `transform_roundoff`, all killed at behavioural assertions:
  - the axis-aligned per-entry bound;
  - a raised floor;
  - the coupling block dropped;
  - my own M-CLOSE-GAP, killed by the guard;
  - three extras.
- **Product evidence:** 864 runs of a one-member skew model on archives of `134eefc24` (pre-K2a) and `eb52114e9`, both entries and both modes.
  - **Pre-K2a main publishes no trusted value** (0 `checks_passed`).
  - With G = E/2.6 it refuses every member at M03.
  - In a supplementary G sweep it publishes 52 Sensitive (untrusted) results, with errors up to 1.9e-4.
  - Current main refuses every run at formation, by name.
  - **The stop rule was not triggered.**

## Limits

- **SA route:** post-K2a, the adapter's frame route cannot carry a pre-K2a matrix, so the adapter test uses an explicit slot. The FK tests use `StiffnessBlock`.
- **Refusal parity:** the refusal parity is by construction, because both representations form the element's evidence through the same `transform_roundoff`.
- **Not run by I9:** PP and the other suites, hosted CI, DEC-025, independent review, and GEN-8 after commit.
- **Two findings for ROOT,** which I9 did not edit into any record:
  - main's Sensitive publications on skew members at intermediate G;
  - the second limit of §3.4.
