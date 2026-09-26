# TASK — additive analytical-reference extension and fixture README

Parent of record: the session-2 WORKING_ITEMS load-state manager (SendMessage id
`a3675abb28ada0834`). ROOT spawns this TASK on the manager's request. Report to
the manager by SendMessage and send the final report to ROOT. Do not delegate
further. Paths are WORKING_ROOT-relative. `WORKING_ROOT` is
`<checkout>/projects/chirality-piping`; the checkout is
the manager's load-state worktree, branch `codex/piping-load-states-20260925`.

## Why

The independent checkpoint-1 review (`LOAD_STATE_IMPLEMENTATION/REVIEW_CHECKPOINT_1/RETURN.md`)
found three gaps, dispositioned by ROOT:

- **SF1:** multi-segment thermal integration had no maintained reference.
- **SF2:** VERIFICATION control 4's exact two-point secant table is valid.
  The existing fixture instead added an invented 20 °C coefficient point that
  never affects the result.
- **N6:** the fixture has no source README.

You are not the implementer of the thermal or material kernels. Derive every
value independently. Do not read, import or run the product kernels or their
tests to obtain expectations.

## Exclusive write boundary

- `core/product_physics/tests/fixtures/load_reference_states/reference_cases.json`.
  Additive changes only:
  - Add new top-level `cases` entries and/or new variants.
  - Leave every existing key, value, case, variant and hash-bearing field
    byte-for-byte unchanged in meaning.
  - Keep valid JSON and the existing numeric representation convention
    (`exact` authoritative; `decimal`; binary64 `value`; units).
- `core/product_physics/tests/fixtures/load_reference_states/README.md` (new).
- `LOAD_STATE_IMPLEMENTATION/ANALYTICAL_REFERENCE/CP2_EXTENSION/` (RETURN.md and
  `_run_records/`, including your generator/check script). Machine-specific
  paths only inside `_run_records/`.

Do not write anything else. That excludes `tests/fixtures/load_reference_states/models/`,
which the runtime-test TASK owns, and any `src/`, test `.rs` or Git.

## Content

1. A new case `multi_segment_free_length`, with `source` VERIFICATION control 5
   extended to more than one segment.
   - Inputs are invented and labelled invented: datum 300 K and a 4-point,
     3-segment piecewise-linear coefficient table
     (300 K: 1e-5/K, 400 K: 2e-5/K, 500 K: 1e-5/K, 600 K: 3e-5/K), with
     T_install 350 K and T 550 K (both interior).
   - Expected values, forward and reversed:
     - the differential-per-datum-length strain;
     - the logarithmic-per-current-length strain (Decimal `exp`, symbolic
       kind like the existing log controls);
     - the split composition through 450 K.
   - A dilation-table variant: a 4-point piecewise-linear dilation table
     (300 K: 0, 400 K: 1e-3, 500 K: 1.5e-3, 600 K: 3.5e-3), forward and reversed.
   - Include `wrong_result_discriminators` for "last segment only" and
     "first segment only" integration.
   - The reviewer's probe targets (5/1601, −5/1606, expm1(1/320), 4/2001) are
     an independent cross-check. Reproduce them from your own derivation; do
     not copy them.
2. A new variant `verification_two_point` under the existing
   `thermal_datum_ratio` case. It uses VERIFICATION control 4's exact inputs:
   α_sec 12e-6/K at 50 °C and 16e-6/K at 150 °C, datum 20 °C, installation
   50 °C, operating 150 °C, with no coefficient point at the datum. State the
   admissibility policy ROOT selected: coverage and positivity are required
   only over [min(T_install,T), max(T_install,T)], because λ(T_m)=1 by
   definition. Expected: 43/25009 and the same companion annular/fixed values
   as the existing annular variant if you include them.
3. A new case `temperature_unit_identity`. It lists exact-rational kelvin
   identities for authored temperatures that must compare equal:
   −50 °C ≡ 223.15 K; 242 °C ≡ 467.6 °F ≡ 515.15 K; 20 °C ≡ 527.67 °R. It also
   gives one nearby non-equal control. Use exact unit definitions:
   K = °C + 273.15, K = (°F + 459.67)·5/9, K = °R·5/9.
4. `README.md`, concise:
   - the file's purpose and schema_version;
   - the numeric representation convention;
   - the source revision and design hashes (from the file's `sources` block);
   - an authorship history: the session-1 TASK membrane_backcheck and its
     generator path, the manager's non-author recheck, the independent CP1
     review, and this extension;
   - that all values are invented and not library or code data;
   - how consumers must use `exact`/`value` with the protected relative 1e-9
     criterion.

   Portable paths only.

## Checks and return

- Standard-library Python only (fractions, decimal).
- Before editing, record the pre-edit sha256 of `reference_cases.json`. After
  editing, check that every pre-existing JSON path has an identical value. Keep
  that comparison's log.
- Re-run the manager's independent checker
  (`LOAD_STATE_IMPLEMENTATION/_run_records/session2/independent_fixture_check.py <fixture>`)
  to confirm existing cases still pass. That checker does not know your new
  cases, so your own script checks them, including value-projection checks.
- No Cargo, npm, browser, native or UI. No Git writes.

SendMessage the manager a short plan first, then the return. The return lists:
file paths and SHA-256; the new case and variant keys; commands run with raw
logs; the before/after preservation result; and any design ambiguity. Write
RETURN.md in this directory.

_Portable publication of the brief. The as-issued bytes that ROOT verified (sha256 `fe864ffa5afc6695068920fdb84329aca82d142a411c81a321c91e6aeeb5c3d8`), including machine-local paths, are kept under the manager's `_run_records/session2/issued_briefs/`._
