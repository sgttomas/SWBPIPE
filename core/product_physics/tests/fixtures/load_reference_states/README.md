# Load reference-state analytical references

`reference_cases.json` holds independent analytical expectations for the load and
reference-state route. These are maintained references, not product request DTOs
and not observed solver outputs. The schema is
`independent.load_reference_state_examples/1.0.0`.

All values are invented test quantities. None of them is material-library,
component-library or code-rule data. Do not copy them into operational libraries.

## Numeric representation

Each quantity is `{unit, exact, decimal, value}`.

- `exact` is authoritative. Its kinds are:
  - `rational`;
  - `rational_times_pi`, which keeps pi symbolic;
  - `symbolic`, such as `exp(1/320)-1`, labelled `"evaluation": "Decimal.exp, precision 85"`.
- `decimal` is an 85-significant-digit `Decimal` rendering. Pi is taken from
  `numeric_representation.pi_decimal`.
- `value` is the binary64 projection. It is a convenience, never a product
  observation.

Authored decimal inputs are exact rational targets. Their binary64 operands are
generally not exact.

## Consumer rules

- Read the expectations from this file at test time. Do not copy constants.
- Compare observed binary64 results with `value`, or with `exact` evaluated at
  higher precision. Use the existing protected relative criterion
  `|observed - expected| <= 1e-9 * |expected|`. For zero expectations, use the
  existing dimension-aware zero-reference handling.
- This file allocates no new or relaxed threshold. Never insert these constants
  as observations.
- `wrong_result_discriminators` are values a correct product must not produce.
  `limits` state each case's scope.

## Sources

`source_revision` is `9e8a55daecdeb9669131fd3e53c0e0303ee550d6`. The reviewed
design is at
`execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/CORRECTNESS_DESIGN/LOAD_REFERENCE_STATES/`.
Its hashes match the file's `sources` block:

| Document | sha256 |
|---|---|
| DESIGN.md | `2839c65ae56c4ff4674da12848eda4b60f1385663b4c0b5e556099eae7fe4f64` |
| INTERFACE.md | `18587811735a262187cf4a26bb934d6c3ef03ceb37c78228bb39b7f8afe073bf` |
| VERIFICATION.md | `061f1f8fa53d426e3af9db58bf684bdfca790b13f3df780935e9353d5402fc57` |
| REFERENCES.md | `ad3147c69a14ee16d5e4cbbcf8300b94dc092802dfb8b5775beb62e690a1b5fa` |
| HYDROSTATIC_CONTROL.md | `30041992674e22115722d688a2d8cb2b589febf20425a1bd56434c7fdf3486ef` |
| RETURN.md | `49e8b8d12a1352c9d8ff182dce9c77f51bbea3dca03524ddcc489773e0b9f554` |

## Authorship history

Paths are relative to
`execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/LOAD_STATE_IMPLEMENTATION/`.

1. **Original generation.** The session-1 TASK `membrane_backcheck` wrote the
   original 11 cases. It used the generator
   `ANALYTICAL_REFERENCE/_run_records/generate_references.py`, and the run is
   recorded in `generation.json`. The output sha256 was
   `5478bba846bcc515187f38366eeda3ac9fb87b339039a5cf3ec2d56dced4b891`.
2. **Manager recheck.** The session-2 WORKING_ITEMS manager, who did not write
   the file, recomputed it with `_run_records/session2/independent_fixture_check.py`.
   All 188 checks passed and all five mutations were detected (`CHECKPOINT_1.md`).
3. **Independent checkpoint-1 review.** A fresh-context TASK reviewed the file in
   `REVIEW_CHECKPOINT_1/RETURN.md`. It re-derived 886 checks with no failures and
   found three gaps:
   - SF1: there was no multi-segment reference;
   - SF2: control 4 carried an invented 20 degC coefficient point;
   - N6: there was no README.
4. **CP2 extension.** A TASK that does not implement the kernels made additive
   changes only. Every pre-existing path and value is unchanged. The generator is
   `ANALYTICAL_REFERENCE/CP2_EXTENSION/_run_records/extend_reference_cases.py`,
   and it was checked by `check_extension.py` in the same folder. The resulting
   file sha256 is `4d7b777708806c211cda0e126d7cd4eb7acba9b658b1da63cca36fd0523beeab`.
   It added:
   - `cases.multi_segment_free_length`, which extends VERIFICATION control 5 to
     three segments for the datum-length, logarithmic and dilation definitions
     (SF1);
   - `cases.thermal_datum_ratio.variants.verification_two_point`, which uses
     control 4's exact two-point table (SF2). ROOT's policy applies: coverage and
     positivity are required only over `[min(T_install,T), max(T_install,T)]`.
     The existing variants keep their invented 20 degC point;
   - `cases.temperature_unit_identity`, which gives exact-rational kelvin
     identities across degC, K, degF and degR, plus one nearby non-equal control
     (CP1 review SF3).

   This README closes N6.
