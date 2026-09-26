# CP2_EXTENSION return: additive analytical-reference extension and fixture README

**Status: complete.** SF1, SF2 and N6 now have maintained references. The SF3
temperature-identity reference is added as the brief asked.

## Scope and identity

- **Who ran it.** A Type 2 TASK, which is not the implementer of the kernels.
  ROOT (HELP_HUMAN) spawned it for the load-state WORKING_ITEMS manager
  (`a3675abb28ada0834`). It delegated nothing and made no Git writes.
- **Paths.** Paths are WORKING_ROOT-relative, with
  `LSI = execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/LOAD_STATE_IMPLEMENTATION`.
- **Verified before starting.**
  - The as-issued brief has sha256 `fe864ffa…c5d8`.
  - The fixture's pre-edit sha256 is `5478bba8…b891` (Git blob `c2342a84`, the same at `5511596af` and `3293c319a`).
  - Root and Piping `AGENTS.md` were read.
  - `VERIFICATION.md` was read via `git show 9e8a55d` (`061f1f8f…`), and the law names were taken from `INTERFACE.md` (`18587811…`).
- **The brief was republished during the run.** The manager's commit
  `3293c319a` re-published the brief as `6a92fdc9…`. The only differences are
  portability edits: the checkout path is replaced by a phrase, and a footnote
  was added. The as-issued bytes are kept at
  `LSI/_run_records/session2/issued_briefs/CP2_EXTENSION_BRIEF.as_issued.md`;
  that copy was verified as `fe864ffa…` and diffed against the new one.
- **Independence.**
  - No product kernel, product test, `.rs` test or product binary was read, imported or run.
  - The manager's `sf1_multisegment_derivation.*` and the reviewer's probe crate and logs were not read.
  - Two limited reads are disclosed in `_run_records/RUNS.json`:
    - The session-1 generator's formatting helpers were read, so the numeric convention matches. No values were taken from it.
    - One grep over `src` looked only for the unit-label literals `degC`/`degF`/`degR`.
  - The reviewer's four probe targets were compared only after my own derivation, and all four match.

## Files written

| File | SHA-256 |
|---|---|
| `core/product_physics/tests/fixtures/load_reference_states/reference_cases.json` (additive edit) | `4d7b777708806c211cda0e126d7cd4eb7acba9b658b1da63cca36fd0523beeab` |
| `core/product_physics/tests/fixtures/load_reference_states/README.md` (new) | `fe8149c0545453275e012af4276e7808bd9179c4accac866e0681d38da0979fb` |
| `LSI/ANALYTICAL_REFERENCE/CP2_EXTENSION/_run_records/extend_reference_cases.py` | `28a7aba31f55a5d60dfd1fa71dd040d65d9a94204398e5d72754103c4c962b0a` |
| `LSI/ANALYTICAL_REFERENCE/CP2_EXTENSION/_run_records/check_extension.py` | `0bd587dc6313489fc5228938a6e12468008b29cecf94e735785df413f47df461` |
| `…/_run_records/pre_edit_state.txt` | `65ffa77ae9814e309c334ee656f106f8396a0a8cec6395efea7d196b8feac128` |
| `…/_run_records/extend_run.log` | `c67574203cc43b5259668ab0c72168e079abeb1d87e4166c7d08153b2833d1da` |
| `…/_run_records/check_extension.log` | `0ac5c9b7f571b2e7892cdd7b44ffc6ec1e76fb76461ab71626867a499b78c439` |
| `…/_run_records/manager_independent_fixture_check.log` | `06122b286dc67eebec490c7badf8a3a82412c42572531c5f6cb03fd6d917883d` |
| `…/_run_records/git_diff_numstat.log` | `439a70870985531ed6800ce97af0f34e9a23130a10c54fe29d6f70e4280569b5` |
| `…/_run_records/reproduce.log` | `4fc4ebaacb51fbf75b59c8cfb831335c94e7c1e6ef2379aefe4ab11ea16c07fd` |
| `…/_run_records/RUNS.json` | (the run record: mechanism, what was read and not read, commands and output hashes) |

Nothing else was written. `models/`, `src/`, test `.rs` files and Git were not touched.

## New keys

All values are invented and exact.

- **`cases.multi_segment_free_length`** (source: VERIFICATION control 5, extended). The inputs are the brief's: datum 300 K, install 350 K, operating 550 K, split 450 K.
  - **Variant `linear_coefficient_table`** (`differential_per_datum_length`, `logarithmic_per_current_length`). Integrals:
    - I(300→350) = 1/1600;
    - I(300→450) = 19/8000;
    - I(300→550) = 3/800;
    - I(350→550) = 1/320, made up of the three segment pieces 7/8000, 3/2000 and 3/4000.

    | Strain | Datum-length | Logarithmic |
    |---|---|---|
    | Forward | 5/1601 | exp(1/320)−1 |
    | Reverse | −5/1606 | exp(−1/320)−1 |
    | 350→450 | 14/8005 | exp(7/4000)−1 |
    | 450→550 | 1/729 | exp(11/8000)−1 |
    | 550→450 | −1/730 | exp(−11/8000)−1 |
    | 450→350 | −14/8019 | exp(−7/4000)−1 |

    Composition and reversal are exact.
  - **Variant `linear_dilation_table`** (`engineering_dilation`, table 0 / 1e-3 / 1.5e-3 / 3.5e-3). The dilations are d = 1/2000, 1/800 and 1/400 at 350, 450 and 550 K.

    | Strain | Value |
    |---|---|
    | Forward | 4/2001 |
    | Reverse | −4/2005 |
    | 350→450 | 1/1334 |
    | 450→550 | 1/801 |
    | 550→450 | −1/802 |
    | 450→350 | −1/1335 |

  - **`wrong_result_discriminators`** (forward only). Each is given as datum-length, logarithmic and dilation values:

    | Discriminator | Datum-length | Logarithmic | Dilation |
    |---|---|---|---|
    | `first_segment_only` | 7/8005 | exp(7/8000)−1 | 1/2001 |
    | `last_segment_only.interval_difference_form` | 6/8005 | exp(3/4000)−1 | 2/2001 |
    | `last_segment_only.datum_integral_form` | 1/8005 | exp(1/8000)−1 | 1/2001 |
    | `interior_breakpoints_skipped` | 28/8005 | exp(7/2000)−1 | — |
    | `endpoint_alpha_times_interval` | 1/250 | — | — |

    Each wrong value is separated from the correct one by more than 1e-3 relative.
- **`cases.thermal_datum_ratio.variants.verification_two_point`**
  - **Inputs.** The exact control-4 inputs: a two-point table at 50 and 150 °C, with no point at the 20 °C datum.
  - **`admissibility_policy`.** This block states ROOT's rule: coverage and positivity are required only over [min(T_install,T), max(T_install,T)], because λ(T_m)=1. It also records that the top-level `datum_outside_thermal_coverage` control still applies to the integral and dilation definitions, not to engineering_secant. The manager confirmed that reading.
  - **Expected values.**
    - The strain is 43/25009.
    - The required coverage is 323.15–423.15 K, and the minimum datum stretch is 25009/25000.
    - The annular L = 1 m, E = 200 GPa companion is 43/25009 m free tip, 0 N free wall and ∓16340000000/25009·π N fixed. These equal the existing `annular_companion` values key for key.
  - **Schema.** The variant uses the existing variants' schema, so the manager's checker also re-derives it.
- **`cases.temperature_unit_identity.variants.exact_affine_identity`**
  - **Identity groups.** These must compare equal:
    - −50 °C ≡ 223.15 K (4463/20);
    - 242 °C ≡ 467.6 °F ≡ 515.15 K (10303/20);
    - 20 °C ≡ 527.67 °R (5863/20).
  - **Non-equal control.** −49.999999 °C = 223150001/1000000 K against 223.15 K. The difference is exactly 1e-6 K, about 4.5e-9 relative.
  - **Discriminator `binary64_affine_conversion`.** It is informative. `-50.0+273.15` gives 223.14999999999998, and `(467.6+459.67)*5/9` gives 515.1500000000001; neither equals the binary64 target.

## Preservation (before and after)

- **Every pre-existing path is unchanged.** `check_extension.py` found that all 2656 pre-existing JSON paths keep an identical value and type. It also found that the key order of pre-existing objects and all array lengths are unchanged, and that 1035 paths were added, all under the three new keys. The pre-edit copy came from `git show HEAD:<fixture>`, sha256 `5478bba8`.
- **The text diff is insertions only.** `git diff --numstat` gives 1360 added lines and 0 deleted.
- **The JSON is valid.** A strict parse (duplicate keys and NaN rejected) passes, `json.tool` passes, and the file keeps its serialisation convention (`json.dumps(indent=2)+"\n"`). The pre-edit bytes round-trip byte-identically under that convention.
- **The generator is guarded.** It refuses any input whose sha256 is not `5478bba8`. It reproduces `4d7b7777…` byte-identically from the Git blob, and a second application is refused (`reproduce.log`).

## Checks and commands

The raw logs are in `_run_records/`. Python 3.11.15, standard library only.

1. **`extend_reference_cases.py <fixture>`** (`extend_run.log`). It uses exact per-segment trapezoids with Fraction, and Decimal.exp at precision 85 for the log form. It asserts its own composition and reversal identities.
2. **`check_extension.py <pre> <fixture>`** (`check_extension.log`): 1002 checks, 0 failures.
   - It re-derives values by different methods: segment antiderivatives, bisect interpolation, expm1 at precision 150, and π from Machin's formula. That π matches the fixture's π string.
   - It checks every one of the 126 new quantities:
     - `value` is the correctly rounded binary64 projection of `exact`;
     - `decimal` follows the precision-85 convention;
     - units are present.
   - It scans for library and code terms and finds none.
   - It cross-checks the reviewer's probes. The check targets 5/1601, −5/1606, expm1(1/320) and 4/2001; all match.
   - All ten mutations were detected:
     - consistent last-segment and log mutations;
     - a 2-ulp change to a `value`;
     - reversal by negation;
     - a 20 °C point added back to the two-point table;
     - a binary64 kelvin value;
     - an edited or removed pre-existing path;
     - a broken split composition;
     - an equalised non-equal control.
3. **`LSI/_run_records/session2/independent_fixture_check.py <fixture>`** (`manager_independent_fixture_check.log`): 200 checks (188 before), 0 failures. The existing cases still pass, and the checker also derives the new two-point variant.
4. **Read-only `git diff --numstat` and a removed-line listing** (`git_diff_numstat.log`).

No Cargo, npm, browser, native, UI or product runs.

## Design ambiguities and carry-forward

1. **"Last segment only" has two plausible formulations.** A bug in the interval integral from T_install to T gives 6/8005. A bug in the datum integral used for λ(T) gives 1/8005. Both are included and labelled. The first-segment forms coincide here, because T_install lies in the first segment. The discriminators cover the forward direction only, because "first" and "last" swap under reversal. The correct reverse values are pinned.
2. **The existing `thermal_datum_ratio.limits[0]` text is still accurate.** It says the existing two variants carry an invented 20 °C point, and it was left unchanged. The top-level `negative_contract_controls` list is also unchanged. The ROOT policy and the scope of the datum-coverage control are stated only inside the new variant.
3. **What the non-equal control does and does not catch.** It fails implementations that equate values after coarse rounding, such as a 0.01 K grid. It does not target values within 1 ulp; exact identity is the design intent.
4. **`degR`.** Its presence follows the brief. The product has a `degR` label, but this reference does not assert that the public boundary accepts °R for authored temperatures.
5. **Consumers now see an extra variant.** Consumers that iterate `thermal_datum_ratio.variants` will now meet `verification_two_point`, which has the same schema. I did not read or run the runtime tests. The manager's full-crate freeze run, which reads this file, should confirm it.
6. **No mechanics companion for the multi-segment case.** `multi_segment_free_length` gives only free-length strains. Its limits state that the 1 m tip UX equals the strain and that fixed N = −E·A·ε.
7. **Binary64 discriminator values depend on operation order.** They are informative only.

No out-of-brief request was received.
