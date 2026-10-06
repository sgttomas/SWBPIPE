# I76 checkpoint 1: the T6S-2 Rust goldens

TASK (Type 2), I76, for ROOT. 2026-10-06 UTC.

**Briefs:** `R/BRIEFS/T6S_COMMON.md` (sha256 `c6ac6c65…`) and `R/BRIEFS/I76_T6S_SCHEMA_TESTS.md` (`86eee696…`). **Basis:** I74's plan `R/I74/t6_slice_plan_01/PLAN.md` (`0350c918…`, checked) and RR "I73's checkpoint 1 and I74's plan ruled; D2 5b.3; the T6 slice dispatched" (RR at `0e5b8f5c…` when read).

**Read first:** `NUM/AGENTS.md` (`f96feb19…`), `NUM/agents/AGENT_TASK.md` (`1a13a5b0…`), `NUM/P/AGENTS.md` (`d9f2b23a…`).

**State.** T6S-2's goldens exist and pass. T6S-1 (the dispatcher) and RV95 N-5 are not started; they follow when ROOT continues me. Everything is uncommitted in `WT/t6-outputs` (base main `c1bfc460fc`). No Git writes. Cargo ran only through `WT/tools/t3_cargo.sh`, with `--locked --offline`, `CARGO_BUILD_JOBS=4`, `RUST_TEST_THREADS=2` and `CARGO_TARGET_DIR=WT/targets/i76-t6s`.

## 1. The goldens

| File | sha256 | Bytes |
|---|---|---|
| `P/fixtures/results/retained_precision_successor_derivative_sparse_interactive.json` | `958df02e276538a96c4732302a46cb36a53c2298ca7e1f4deb88a3748c67c661` | 381,178 |
| `P/fixtures/results/retained_precision_successor_derivative_dense_scrutiny.json` | `3f9905ad4c4bba688687714751abe965d18cea834701c32c379d81f573dcf682` | 384,426 |

- **What a golden is:** Rust `derivative::derive_document` applied to the pinned successor `P/fixtures/results/retained_precision_milestone_successor_<mode>.json` (file sha256 `ac6986b0…` sparse, `6cd1d249…` dense; checked in the test).
- **Its bytes** are the document's canonical JSON text (`open_pipe_stress_canonical_json::canonical_json`, profile `openpipestress_jcs_ijson_v1`), with no trailing newline. So **the file sha256 equals the document's canonical digest**: Rust `d::digest(&doc)`, and in TypeScript `canonicalSha256Hex(doc)` (or sha256 of `canonicalJsonString(doc)`). The test asserts that equality.
- **Inputs to `derive_document`:** the fixed base and origin below; `model` = the pinned file's `invocation.request.model`; `request` = none (the desktop passes none, so `request_hash` is null).
- **Shape facts for parity** (both modes; from the reader without an invocation):
  - rows: 98 sparse, 99 dense. Classes relative / absolute / input-derived / non-quantity: 25 / 69 / 3 / 1 (sparse) and 25 / 69 / 3 / 2 (dense). **No `not_covered` row** exists in either (as U6a recorded for every validated statement).
  - so `row_disclosures` = 69 `retained_precision_absolute_verified` plus the non-quantity row(s) with their ordinary reason; 28 quantity values; 0 review evidence; 28 unit witnesses.
  - b is printed by Rust `{:e}`: shortest round-trip digits, no `+` (e.g. `b = 5.4215527659630466e-24 m (binary64 3b1a378ea78c5ce9)`).
  - the receipt's own `body.invocation` is its hash statement; the document carries no `{request, solver_mode}`.
- **Schema:** both goldens are valid under `results.v0.3.schema.yaml` (validate_instance). The current dispatcher `results.schema.yaml` refuses them (F-U6c-2), which T6S-1 repairs.

## 2. The fixed desktop-shaped base and origin

**The rule.** They are the base and origin that `buildCurrentResultExport` (`DT/features/result-export/resultExportAdapter.ts:214–215` at `c1bfc460fc`) builds, with fixed `test:` stand-ins for the values it reads from the AnalysisRun and the InputManifest. No AnalysisRun or manifest is built. Every other value is the builder's own constant or its own function of the model and the result.

**The fixed stand-ins:**

| Builder input | Value used |
|---|---|
| `inputManifest.manifest_ref.ref` | `test:t6s-golden-reference-only-manifest` (into `audit_manifest_ref` and `qualification_ref`) |
| `inputManifest.manifest.solver_basis` | `solver_name` `open_pipe_stress_product_physics`, `solver_version` `0.2.0` (the producer's, as the builder requires), `solver_build_ref` `test:t6s-golden-pinned-producer-test-bytes-not-native-attestation` |
| `run.run_id` | the result's `run_id` (`run:preview-linear-static-001`) |
| `run.load_basis_refs` | `modelLoadBasisRefs(model)` → `[{ref_type: "LoadCase", ref_id: "case"}]` |
| `run.analysis_status` | the analysis builder's sorted set `["HUMAN_REVIEW_REQUIRED", "MECHANICS_SOLVED", "RULE_INPUTS_INCOMPLETE"]` |
| `run.professional_boundary` | the analysis builder's (human review true; the five claims false, including `software_makes_authentication_claim`) |
| `run.hashes` | **none: `run_hashes: []`** (no record is built, so no record hash is claimed) |

**From the builder, unchanged:** the top-level constants (DEL-08-04, PKG-08, SOW-046, objectives, `export_format_status`), `envelope_id` `result-envelope:<run_id>`, `model_ref`, `run_ref`, `unit_system_ref` `<project id>:units`, the mechanics `result_sets[0]`, `derivativeProvenance`, `model_hash: null`, `deterministic_ordering: true`, `downstream_use`, and **the diagnostics mapped from the result's own diagnostics** (code; class `ASSUMPTION_WARNING`; severity, `error` → `blocking`; source `ref('source', x.source ?? 'local_preview')`; affected object `ref('preview_entity', x.affected_refs?.[0] ?? model_ref)`; message; the fixed remediation; `derivativeProvenance`). The two successors' diagnostics differ in one message, so **the two bases differ only in `diagnostics`**.

**The origin:** the builder's dimension-absent origin (every successor row lacks `dimension`): `origin_id` `source-origin:current-received`, `origin_class` `received_current_dimension_absent`, `qualification_ref` `ref('current_manifest', 'test:t6s-golden-reference-only-manifest')`, `authentic_producer_available: false`, `received_carrier_checksum` = scoped checksum of the result (`received_current_dimension_absent_carrier`, `ref('received_current_carrier', run_id)`), `original_producer_checksum: null`, the three request fields null.
- **One departure from the builder:** its `origin_limit` text says "Qualified Current received carrier…", which a test-built golden must not claim (D-U7-6; decision 11's labelling). The golden uses: `Test-built desktop-shaped origin (T6S golden): pinned producer test bytes, not a qualified Current received carrier; independent authentic original producer bytes unavailable; dimension absence is not producer attestation`.

**The exact JSON** (canonical text, written by the test under `I76_T6S2_INPUTS_OUT`) is in `inputs/` here, for I75 to reproduce and compare:

| File | sha256 |
|---|---|
| `inputs/sparse_interactive.base.json` | `5a48e00d7976ed7e2e34967d023219a7e5e23144b35c680415d1b3786bfc44d8` |
| `inputs/sparse_interactive.origin.json` | `1a0f642ce9c9476924e6f8841ccadd3109c9f1f761fe404d7adb5c84759894cc` |
| `inputs/dense_scrutiny.base.json` | `3f054be38290bf9ae6e6a236c192d6eda8c5a0867367d8a0559509fe6f515e78` |
| `inputs/dense_scrutiny.origin.json` | `04ee7eb957b9bdc7b617e588393a01107c7a1aa0a064087e45a58c481a682231` |

The origins' carrier checksum values are `7eac1d915f6c8b69…` (sparse) and `86a241c4a0b12ba4…` (dense), each the canonical digest of that successor's `source`. Each origin equals the golden's `reproducibility.source_origin_bindings[0]`, and each base's pass-through fields equal the golden's (checked in Python).

## 3. The test and the regeneration command

**The test:** `RE/tests/retained_precision_derivative_golden.rs` (new, 422 lines, sha256 `2706d06b561756b530a234e4c488cd341061093f822b45bfccf06e75c99fc798`). The base, origin and stand-ins are literal there (`desktop_base`, `desktop_origin`, `MANIFEST_REF`, `SOLVER_BUILD_REF`, `ORIGIN_LIMIT`). Three tests:
1. `t6s2_goldens_are_the_live_successor_derivative`: the live derivative validates (`validate_document`), is deterministic, and its sha256 equals both the pinned `GOLDEN_SHA256` and the committed file's sha256. With `I76_T6S2_OUT` set it writes the goldens instead.
2. `t6s2_golden_carries_the_receipt_evidence_and_class_disclosures` (the brief's controls):
   - the receipt whole, `contract_evidence`, and the base statements as received;
   - the desktop origin, no producer hash, no `request_hash`, no `{request, solver_mode}`, and the model hash of the pinned model;
   - class counts as above;
   - **exactly one disclosure per `absolute_verified` or `not_covered` row**, with D-U6-2's code and message (equal to `class_disclosure`, and checked independently: prefix and suffix text, the printed b round-trips to the receipt's bits, no `+`), disposition `disclosed`, no value witness; no class code on any other row.
3. `t6s2_a_mutated_pinned_successor_changes_the_golden`:
   - the pinned file bytes are checked by sha256 (an edited file fails the pin);
   - one diagnostic message edited: refused unresealed, and once resealed it is derived to a document with a different sha256;
   - one `relative_verified` value moved by one ulp and resealed: refused (`RETAINED_PRECISION_PRODUCT_ATTEMPT_MISMATCH`).

**Regeneration** (from `RE`):

```
CARGO_BUILD_JOBS=4 RUST_TEST_THREADS=2 CARGO_TARGET_DIR=WT/targets/i76-t6s \
I76_T6S2_OUT=<P>/fixtures/results \
WT/tools/t3_cargo.sh test --locked --offline --test retained_precision_derivative_golden -- --nocapture
```

then set `GOLDEN_SHA256` in the test to the two printed sha256 values and rerun without `I76_T6S2_OUT`. Outside T3 the wrapper is plain `cargo`. Add `I76_T6S2_INPUTS_OUT=<folder>` to also write the four base/origin files.

**Runs** (logs in `_run_records/`, paths replaced by `WT`):
- `regen_final_test.log`: the final test file, writing into `WT/scratch/i76_t6s/gen3`; 3/3 pass; both outputs byte-identical (`cmp`) to the committed goldens.
- `compare_final_test.log`: the final test file without `I76_T6S2_OUT`; 3/3 pass.
- Earlier runs from scratch revisions of the test: the first generation (one control assertion was wrong, see below), then the generation into `P/fixtures/results`. All produced the same two hashes.

**One correction on the way:** my first control asserted that the text `"invocation"` is absent from the document. It is present as the receipt's `body.invocation` hash statement, so the check now asserts that no `request` or `solver_mode` member exists anywhere.

## 4. For ROOT

- **Relay to I75:** the two golden hashes (§1), the rule and stand-ins (§2), the four `inputs/` files, and that the model is the pinned `invocation.request.model` with no request.
- **Choices ROOT may override** (each regenerates in one run):
  - `origin_limit` replaced by the test-labelled text, not the builder's product text;
  - `run_hashes: []`, because no AnalysisRun record exists to hash;
  - diagnostics mapped per source, so the base is one rule, not one byte string.
- **Not yet done:** T6S-1, the N-5 test, the base/candidate suite comparisons, the scratch mutants, and RETURN.

## SHA256SUMS

`SHA256SUMS` covers this file, `inputs/*` and `_run_records/*`.
