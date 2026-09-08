# RK validation — K8 canonical route-A candidate

Source basis: `779dedb8670625b36af07b89fc5557470e47c50e`. K8 manifest basis: SHA-256 `a6ad4f8ab2881801a2ebabb483e5c22e4cafaa029e07cc398bf420013b576a65`.

## Frozen inventory

- Manifest hash: exact match.
- Manifest outputs: 12/12 present and byte-hash exact. `REVIEW_INVENTORY.json` records each path and hash.
- The review wrote only `{RUN_ROOT}/instances/RK/**`. Existing dirty state elsewhere was treated as external state.

## Reproduction and coverage

The frozen `build_candidate.py` was copied to an isolated temporary directory and executed from the source checkout with `uv run --with jsonschema python3`. It reproduced byte-identical candidate outputs:

| Output | Reproduced SHA-256 |
|---|---|
| candidate schema | `89f8bb5acc2d93735f44bbea0aabf669433ef9a1ac9a205cf76b86b2bdc38098` |
| source-kind mapping | `d407ecc092ccdd2064e1d3af06503eef037e4bfc733498a1e3c0b41776013ab9` |
| coverage validation | `19d5ab3546a56267f0b05d60a9d1953ffdf5b5a2b9a0aa888e492bdf7ac1aca6` |

Independent parsing confirmed:

- W10 has 2,348 unique scenario/result rows: 2,010 `SCHEMA_VALID_IDENTITY_PRESERVE` and 338 `UNSUPPORTED_MAPPING_REQUIRED`; the invalid rows contain 518 invalid metadata occurrences.
- Accepted documents contain 777/797/774 unique emitted IDs and 9/33/39 unique disclosed omitted IDs, with no emitted/omitted overlap. The reconstructed native totals are 786/830/813.
- The nonlinear native witness has 830 rows, 830 unique IDs, 44 native `(kind, unit)` pairs, and no non-finite values. The 45th mapped pair is the disclosed `pipe_section_pressure_longitudinal_stress,MPa` pair.
- `SOURCE_KIND_MAPPING.csv` has 45 rows and 45 unique pairs. The candidate schema has 45 unique native-record branches and 45 unique quantity/classification branches, exactly equal to the CSV tuples.
- Isolated candidate generation validates all 830 nonlinear rows and asserts full JSON-value equality of each `source_record` to the corresponding native row plus equality of `result_id/id`, `magnitude/value`, and `unit/unit` for that generated witness.
- Pressure mappings remain `existing_pressure_source_label_preserved_pending_P5` or `source_specific_unstandardized_pending_P5`. No pressure meaning was accepted or inferred.
- Mutating either a pending-pressure status or a transport-observation status to `existing_canonical_category` is rejected. Unknown outer/inner versions, unknown kind, and unknown kind/unit pairs are rejected. Checksum label/claim disagreement is rejected. The referenced checksum `algorithm`, `payload_ref`, and `value` triplets reproduce unchanged.

## Adversarial schema/runtime probes

Each probe began with the full valid 830-row in-memory candidate and ran `Draft202012Validator.iter_errors` against the frozen candidate schema.

| Mutation | Schema result |
|---|---|
| duplicate one complete quantity/result ID | ACCEPT, 0 errors |
| top-level `result_id` disagrees with `source_record.id` | ACCEPT, 0 errors |
| top-level `magnitude` disagrees with `source_record.value` | ACCEPT, 0 errors |
| top-level `unit` disagrees with `source_record.unit` | ACCEPT, 0 errors |
| top-level metadata disagrees with `source_record.metadata` | ACCEPT, 0 errors |
| top-level `object_ref.ref_id` disagrees with `source_record.entity_ref` | ACCEPT, 0 errors |
| `NaN` in both numeric mirrors | ACCEPT, 0 errors |
| positive infinity in both numeric mirrors | ACCEPT, 0 errors |
| P5-pending row changes `owner_semantics` to `PKG08_TRANSPORT_ONLY` | ACCEPT, 0 errors |
| unknown `source_record.kind` | REJECT |
| unsupported `source_record.unit` for a known kind | REJECT |
| unknown outer or inner schema version | REJECT |
| checksum label/claim mismatch | REJECT |

The non-finite result is a Python `jsonschema` data-model boundary: `NaN`/infinity are not valid JSON number lexemes, but the validator accepts Python float objects. A production contract therefore must require strict JSON parsing/serialization or explicit finite-number checks in addition to schema validation. Current Rust result-export validation checks `magnitude.is_finite()`, and the current desktop report-package builder uses `Number.isFinite`; the route-A design must preserve and apply that guard to both top-level and native values at every new writer/reader boundary.

## Current consumers and migration seam

Source inspection confirmed the K8 inventory's main observations: the Rust headless result document is library-only and omitted from CLI serde; `export-results` consumes a report-package request; Rust report packaging directly deserializes one result-envelope DTO; the desktop result-export panel independently emits `0.1.0` with heuristic fallback to `ratio`; and desktop report-package assembly independently emits a `1.0.0` result envelope. Schema-qualified references also exist in analysis-run, adapter, headless-runner, and handoff-package contracts.

The packet assigns owning packages and desired migration outcomes, but it does not freeze the exact version-request field/API or structured refusal shape at any one current entry point. It alternates between “adapter boundary” and “chosen public boundary,” leaving an implementation choice after the requested Owner selection.

## Limits

This was a bounded design/code review of the frozen K8 packet and directly affected current surfaces. No full harness, native build, source edit, physical-model review, pressure acceptance, public adoption, lifecycle ruling, or broad historical audit was performed. Schema acceptance is structural evidence only and is not physical approval.
