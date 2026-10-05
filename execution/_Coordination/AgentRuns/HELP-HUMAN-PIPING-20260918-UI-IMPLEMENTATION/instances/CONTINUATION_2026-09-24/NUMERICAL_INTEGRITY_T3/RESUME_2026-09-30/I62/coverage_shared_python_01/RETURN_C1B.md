# I62 return: checkpoint C1b (snapshot 05b)

**Basis:** the rulings at NUM 6c8c0f30e8, "Snapshot 05a verified; floor and ladder are existing contract" and "Rust and TypeScript aligned to snapshot 05a; …; two more parity rules".

**Run window:** 2026-10-03T20:45:21Z to the 2026-10-03T20:56:52Z SHARED_SNAPSHOT_05B freeze, about 12 minutes of the 2-hour box. The memory guard (PID 5387) was running.

**Limits held:**
- No Git writes or index operations.
- No Cargo, solver or native job.
- I63/I64 files were not touched.

Paths use the placeholders.

## Correction to RETURN_C1A

C1A said Rust had no Φ check. In fact Rust's inherited G5b already checked `floor == phi_512(e_hat(E, L))`, as ROOT's ruling at 312f14f4a1 records. My statement was wrong.

## Changed READER files

| File | Before (05a) | Now |
|---|---|---|
| P/fixtures/results/retained_precision_cases.json | 159ef78c47… | ea6fe2c7574fb80b409c2931a7979451313d00a7c1d863ec993ec572b68dc808 (2850052 B) |
| P/core/analysis_runs/retained_precision.py | 27fc1797c2… | 3b12ca7511c698143b77af4058b7e41932026714453dad97240031183ed87994 (77207 B) |

- **Unchanged:** the contract test file (7eab5b3793), the schema test, the schema, the table, the yaml and the definition.
- **05a is preserved byte for byte:** all 6 cases, 104 mutations and 15 must-pass entries (asserted).
- **No existing expected outcome moved.** The C1a Python reader differs from 05b only on the four new parity-pin mutations (`c1a_reader_vs_05b.json`).

## Parity pins: contract readings and Python alignment

**Gate order across cases: confirmed.** C3_DELTA §4 says "Keep C1 order G0,…,G8. Within a gate use this deterministic order, then ascending attempt/member/lane/row index. First failure wins." C1 §6 says "All readers execute G0→G8 in the same order".
- Python `_g5_numeric` now runs G5a for every case, then G5b, then G5c.
- The prerequisites of the G5a resolution tests (a non-empty body and finite normalized values) now fail at G5a, as Rust's `numeric_cases`/`body_extent` do.

**Record body order.** The verification record's resolution and theta entries must list bodies exactly 0..n−1, in order, for both selected and unavailable attempts. This replaces Python's sorted-set comparison.

## Snapshot 05b contents (SHARED_SNAPSHOT_05B.json)

The snapshot JSON has the full tables, the native paths, the Python observations and raising lines, and the bulk listing. All new cases are labelled synthetic.

**New cases (3):**
- **`two_body_synthetic`:** a second, separately anchored, unloaded cantilever. Coverage is `[{0,[T;4],T},{1,[F;4],F}]`.
- **`p512_ladder_synthetic`:** the two-body model through the native ladder.
  - The p128 candidate is rejected; its v256 is reused as the candidate and rejected; v512 is reused and accepted; v1024 verifies.
  - It has seven builds.
  - The floor is Φ = `phi_512(ê)`: positive for body 0 and zero for body 1.
  - The rejections are synthetic attestations of private stop-rule comparisons. The same model is accepted at p128 in `two_body_synthetic`, which is a separate receipt; nothing within the p512 receipt contradicts itself.
- **`two_case_synthetic`:** case 0 loaded and case 1 with zero loads, both selected. One call, a shared stiffness group and a reused cache.

**New mutations (17), each failing in Python at the intended check:**

| Base | Mutation | Expected first failure |
|---|---|---|
| two-body | has_data swapped | G5a |
| two-body | has_data swapped with the bound lists made consistent (detected by body 0's free loads) | G5a |
| two-body | stop flags swapped without their stop_rule entries | G5a |
| two-body | body order swapped | G3 |
| two-body | body missing | G3 |
| two-body | resolution order swapped | G5a |
| p512 | Φ off by one ulp | **G5b** |
| p512 | positive floor forces stop | G5a |
| p512 | charge follows the estimate on the zero-floor body | G5a |
| p512 | floor null | G5a |
| p512 | a positive floor on the unloaded body (feasibility comes before the Φ check) | G5a |
| two-case | no-data flags copied into the loaded case | G5a |
| two-case | source_ref/run_ref rebound only | G5 PRODUCT_ATTEMPT |
| two-case and F | cross-case G5b plus G5a, ×2 | **G5a** |
| F | record resolution duplicated; record theta duplicated (body-order pin) | G5a |

**New must-pass entries (4):**
- the undetectable stop swap between bodies, with consistent stop_rule entries;
- p512 charge equals stop on the zero-floor body;
- loaded flags copied into the zero-load case, with consistent bound lists;
- `cert_failed_after_summary_accounting` (see the defect below).

## Tests

Final run `python_schema_D2`: **167 passed, 0 failed.** It ran the brief's command with both BIN variables and a 1,200 s wall, and its input hashes match the final files.

## Stopped or deferred, reported early

- **The Ceiling row was dropped.** In the two-case template, case 1 repeats case 0's inputs on the same stiffness and cache. Case 0 accepts at p128, so case 1 cannot be rejected at every precision with the same inputs. A faithful Ceiling needs a case with genuinely different numerics, which means producer-solved rows. The builder and its four-record reuse chain stay in scratch (`build_c1b.ceiling_row`).
- **L = 0 and source-construction failure** stay deferred, per ruling.

## Defect found in 05a (left byte-identical; ROOT to rule)

**Two must-pass entries attest a numeric predicate failure that native code can't produce:** `cert_failed_after_summary` and `cert_failed_predicate_null_undetectable`. Both put the failure in case 1 of base F, whose inputs repeat case 0, and case 0 passed its certificate. Identical inputs cannot give a different numeric verdict, so these triggers are not native-faithful.
- Their public outcome (pass) is unaffected.
- 05b adds `cert_failed_after_summary_accounting` as a faithful replacement: a work-accounting refusal after the summary is assigned (`final_case.rs:1195–1200`).
- **Proposal:** retire or retarget the two 05a entries.

**Also flagged:** 05a's `old_operational_error_new_ready`, promoted from I58's Python-only control, attests a `coefficient_range` refusal for ordinary-magnitude old operands. I haven't established that native code can produce that trigger.

The other 05a rows use resource or accounting triggers, which can differ between cases with identical inputs.

## Open for ROOT

- The 05a defect disposition.
- How a producer-solved witness should be obtained for the Ceiling and L = 0.
