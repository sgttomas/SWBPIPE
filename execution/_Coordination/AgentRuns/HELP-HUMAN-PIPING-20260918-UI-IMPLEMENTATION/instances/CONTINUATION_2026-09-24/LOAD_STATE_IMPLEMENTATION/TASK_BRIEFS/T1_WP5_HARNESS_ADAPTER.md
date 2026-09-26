# TASK brief — T1_WP5_HARNESS_ADAPTER (VP-HARNESS `load-reference-1` adapter)

Read `_T1_COMMON.md` first. Return folder: `LSI/T1_WP5_HARNESS_ADAPTER/`.

## Assignment

Build `tools/validation/qualification_load_reference.py`, a closed adapter for the `load-reference-1` transport. It mirrors `tools/validation/qualification_physics.py` and its helpers and follows the gate design in `validation/qualification/HARNESS_CUT.md` and `GATE_USAGE.md`. Reuse `qualification_process.py` and `qualification_gate.py` rather than copying them. Read `tests/test_qualification_physics*.py` and `tests/qualification_fixture_support.py` for the test patterns.

**Pinned identities:**

| Item | Value |
|---|---|
| Transport ID | your choice, in the style of `ordinary_physics_1_cli_1.0_raw0.2`, for example `load_reference_1_cli_1.0_raw0.2` |
| Contract | `openpipestress.result_semantics/0.3.0/load-reference-1` |
| Profile | `resolved_straight_load_state_v1` |
| Table | `fixtures/results/semantic_contract_v0_3_load_reference_1.json`, sha256 `44bc41c06f589fab6ce931ac0eaa5344765ff64fd5f880cc2dd69ecb839c4f4d` |
| Reader module hash | `core/analysis_runs/load_reference_evidence.py` at the start commit. Record it; the manager updates the pin at integration if WP1 changes that file |
| Units module | as the physics adapter pins it |

**Case binding.** The adapter must **not** hard-code case or assertion IDs. The WP6 fixtures are being written in parallel, so the adapter reads a locked case manifest, `validation/qualification/fixtures/load_reference/MANIFEST.json`, with this format:

```
{"format": "openpipestress.load_reference_qualification_manifest/1",
 "cases": [{"case_id", "runner_input": {path, sha256}, "product_request": {path, sha256},
            "reference": {path, sha256}, "selectors": {path, sha256}, "criteria": {path, sha256},
            "analytical_reference": {"path": "core/product_physics/tests/fixtures/load_reference_states/reference_cases.json", "sha256", "case_key"},
            "modes": ["sparse_interactive", "dense_scrutiny"], "required_scalar_rows": N}]}
```

- The selector, reference and criteria files use the first-static formats (`openpipestress.first_static_selector_candidate/1`, `openpipestress.qualification_reference_values/1`, and the criteria `tolerance_profile`), with `producer_contract` = load-reference-1.
- Selectors may also address the per-case `contract_evidence.load_reference_states` record: member, support-component and contribution fields. Specify that selector extension precisely in your return.

**Required behaviour** (the HARNESS_CUT gate rules):

- Materialize the full required ledger before execution.
- Every required selector resolves exactly once, with case, entity, kind, component, frame, location, sign, unit and definition matching.
- Refuse duplicates, non-finite values, substitution of a required case, a wrong contract or profile, and a missing or insufficient standing.
- Report the numerical standing (`checks_passed` versus `sensitive`) separately from outcomes.
- A missing value stays unavailable and never becomes zero.
- Interrupted, truncated or timed-out runs fail correctly.
- Emit a JSON report and a short Markdown assessment from the same ledger.

The joined `load-reference-source-1` transport is out of scope for this adapter; a separate identity comes later.

**Tests.** Add them as `tests/test_qualification_load_reference.py`. Use synthetic comparator controls and the committed load-reference-1 raws (`fixtures/product_preview/load_reference/*.raw.json`) with a synthetic manifest. Include seeded faults for each refusal. Do not run the product solver or the runner binary; WP6 does real runs after its freeze.

## Write boundary

- `tools/validation/qualification_load_reference.py`, plus a small helper module next to it if needed (new files only).
- `tests/test_qualification_load_reference.py` (new).
- Your return folder.

Do not write `validation/qualification/fixtures/load_reference/**`; that is WP6's. Use temporary synthetic manifests in tests.

## Checks

- Your tests.
- `tests/test_qualification_physics*.py` and `tests/test_qualification_gate.py`, unchanged and passing.
- Mutants removing each refusal.
