# T1_WP4_HEADLESS — return

- **Role:** TASK (Type 2). Reports to the T1 WORKING_ITEMS manager.
- **Checkout:** load-state worktree, branch `codex/piping-load-states-20260925`, HEAD `6810b67ff`. Its ancestor is the spawn base `fe3e2dbbc`, and it adds only a docs brief.
- **Git:** no writes.
- **Paths:** relative to WORKING_ROOT.
- **Status:** tests complete; no product semantics changed. This is execution evidence, not acceptance.

## Ruling applied during execution

The brief's item 2 says the joined canonical document "selects the joined branch". **The manager's ruling of 2026-09-26 supersedes that phrase.**

- `result_envelope_binding` builds a canonical document only for a `numerically_eligible` result.
- `T1_WAVE1_RULINGS.md` §7 makes `load-reference-source-1` never eligible in T1.
- So T1 has no headless joined document and no joined `QualifiedPreviewEvidence`. This is intended.
- The committed `fixtures/results/load_reference_source_*.document.json` carriers come from `derive_document` in `core/reporting/result_export/tests/load_reference_source_contract.rs`, with a test origin. They do not come from the binding.
- A binding route for joined results is T3 work, together with eligibility.

The joined lanes therefore:

- assert no document, no proof, and the exact unavailability text;
- assert the receipt's invocation binding;
- assert `needs_recompute` in every reader;
- assert the reparse refusal;
- emit artifacts without a document.

The manager also ruled that a 0.3.0 document carrying 0.4.0 keys is pinned as observed:

- with the exact contract, `physics-1`;
- without it, `preview-physics-1`.

In both cases it is blocked with `LOAD_STATE_CONTRACT_VERSION_MISMATCH`, has no results, and never carries a T1 identity.

## Files changed

| File | sha256 before | sha256 after | Change |
|---|---|---|---|
| `core/runner/headless/src/lib.rs` | `08695f025b38b140dc96ded47e301809ad41936e7f2fa7222fc76c91ce6873cd` | `83bf616c115ab255666297f280d95f0514f3afab31658dc093262e3c740fbc35` | +2 lines: `#[cfg(test)] mod load_reference_route_tests;`. Test-only; no production line changed. |
| `core/runner/headless/src/load_reference_route_tests.rs` | new | `482e6855c6add390fee551052031d372f2c9036cc5d897c6ddd5b3c8e07a7a05` | Library value-route tests (7). These sit in the crate because `QualifiedPreviewEvidence` fields are crate-private. |
| `core/runner/headless/tests/load_reference_cli.rs` | new | `a607608ae8f727092f155d7d40f601568e2697b96fae19ff6428e66dccbb2be7` | Tests that spawn the actual `openpipestress-runner` process (7). |
| `tests/test_load_reference_headless_artifacts.py` | new | `0d78985e89e1f711f11508074d6e9ee5f6a81cfd9fbe004ea0727e2ffc6a661d` | Python consumer of the actual artifacts (15). |
| `LSI/T1_WP4_HEADLESS/**` | new | — | This record, `_run_records/COMMANDS.md`, `mutants.py`, and the logs. |

No production code was added. The artifact lanes write from test code only, like the existing `HEADLESS_SOURCE_BLOCK_OUTPUT_DIR` and `HEADLESS_PRECISION_OUTPUT_DIR` lanes. They use two new variables:

- `HEADLESS_LOAD_REFERENCE_OUTPUT_DIR`;
- `HEADLESS_LOAD_REFERENCE_SOURCE_OUTPUT_DIR`.

## What the tests pin (both solver modes throughout)

### 1. load-reference-1

The inputs are `fixtures/product_preview/load_reference/{connected,pressure}.request.json`.

**Library route:**

- The identity and profile are load-reference-1. The run is `MECHANICS_SOLVED` and `checks_passed`, with no receipt.
- The raw is byte-for-byte the committed producer raw.
- The runner result validates clean.
- Standing is `numerically_eligible`, with and without the invocation.

**`QualifiedPreviewEvidence`:**

- `solve_payload` is the exact request Value.
- `actual_invocation` is `{request, solver_mode}`.
- All six digests and `run_id` are bound.

**Canonical document:**

- 0.3.0, DEL-08-04, with `semantic_contract_ref` = load-reference-1.
- The producer, numerical quality, formulation basis and `contract_evidence` are verbatim from the raw.
- There is a row account for every row.
- `received_carrier_checksum` is the raw digest, and `qualification_ref` is `run_id:invocation_digest`.
- `validate_document` passes on the raw and on reparsed raw.
- The runner payload validation is clean.
- Rebuilding with the proof explicitly gives the same document.
- The typed route (no proof) gives `Err`.
- A document relabelled to load-reference-source-1 or preview-physics-1 is blocked by the runner.

**Proof tampers:**

- These are refused:
  - a mode tamper without a re-digest;
  - a payload tamper;
  - a re-digested invocation whose request is not the payload;
  - a re-digested invocation with a mode outside the closed set;
  - a changed result row;
  - a forged runner checksum;
  - a substituted runner request.
- This is **characterized**, not extended (see Design questions): a re-digested relabel to the other mode is **accepted**. The only differences are the qualification reference and the derivative hash.

**AnalysisRun (Python, over the actual artifacts):**

- `build_analysis_run_v0_3` → `validate_analysis_run_v0_3` → `verify_analysis_run_record == "match"`.
- The received hash equals the document's `received_carrier_checksum`.
- The record equals the committed `fixtures/results/load_reference_*.analysis_run.json` by value.

**Schema:** the actual document validates against `schemas/results.v0.3.schema.yaml` and matches exactly one `ResultEnvelope` branch, the load-reference-1 branch, located by identity.

**CLI (actual process):**

- Exit 0.
- The `ControlledExport` payload is not blocked, has no blocking findings, and its raw and runner result equal the library's.
- No library-only field reaches the CLI surface.
- The default mode's stdout is byte-identical to `--solver-mode sparse_interactive`.
- `--output` receives the stdout.
- **ControlledExport characterized:** without `--explicit-local-private-intent`, a solved 0.4.0 output (load-reference-1 and joined alike) is withheld. The payload is null, the exit is 1, the finding is `LOCAL_PRIVATE_INTENT_REQUIRED`, and no output file is written. No run id or receipt hash reaches stdout. This is the existing DEC-065 policy, not T1's.

### 2. load-reference-source-1

The inputs are the five committed joined witnesses: `fixtures/product_preview/load_reference_source/*.request.json`.

- The identity, profile and receipt policy are the joined ones. The run is `MECHANICS_SOLVED`, and not `checks_passed`.
- The raw equals the committed raw.
- The runner result validates clean.
- The receipt's `body.invocation.value` equals `domain_hash("source_blocks_invocation_v1", {request, solver_mode})` for the mode that ran, and not for the other mode. This holds in Rust and in Python, and on the CLI output.
- The joined readers admit the raw without eligibility: Rust returns `Ok(false)` and Python returns `False`.
  - The joined reader takes no invocation parameter, so the invocation binding is asserted through the receipt hash.
- Standing is `needs_recompute` in Rust (with and without context) and in Python.
- There is no document and no proof. `canonical_export_unavailability` is `"result-envelope production failed structurally: CURRENT_NUMERICAL_INTEGRITY_NEEDS_RECOMPUTE"`.
- A mechanics envelope carrying reparsed public receipt bytes gets `Err` from `build_result_export_document`.
- The AnalysisRun matches, carries the receipt and `contract_evidence`, and equals the committed carrier by value.
- The artifacts are request, invocation, raw and manifest. The consumer asserts that no document file exists.

### 3. Refusals through the runner

- **Explicit null** (`materials[0].expansion_laws`, on both load-reference requests and a joined witness):
  - a blocked load-reference-1 envelope with `LOAD_STATE_EXPLICIT_NULL_UNSUPPORTED`;
  - `MODEL_INCOMPLETE`, with no results, no result refs and no `MECHANICS_SOLVED`;
  - `validate_result` blocking, no document or proof, and `SOURCE_NOT_SOLVED`;
  - CLI exit 1.
- **Null on a required key** (`analysis_state.history`): a typed request-boundary `Err` (`invalid type: null`).
- **Unknown field** (in `analysis_state` or a reference configuration):
  - the library returns `Err` containing ``unknown field `…` ``;
  - the CLI refuses at its typed input gate with exactly one blocking `HEADLESS_RUNNER_SOLVE_PAYLOAD_INVALID`, a null runner result and a null envelope, and exit 1.
- **Pre-0.4 carrying 0.4.0 keys:** blocked with `LOAD_STATE_CONTRACT_VERSION_MISMATCH`, as `physics-1` (exact contract) or `preview-physics-1` (no contract). Never a T1 identity. Nothing is published as solved. CLI exit 1.

### 4. Blocked 0.4.0 envelopes

Three cases: a missing pressure contract, a legacy pressure contract, and a joined witness without a contract. Each gives, through both the library and the CLI:

- a blocked load-reference-1 envelope with the load-state profile;
- the empty namespace `{"pressure":[],"connector":[],"exact_cases":[],"load_reference_states":[]}`;
- never preview-physics-1.

### 5. SF-1 fallback

The input is the CP3 probe P9: the eigen_motion witness plus 1, 2 or 3 extra loads, the same transform as `load_state_fallback_tests.rs`.

- `MECHANICS_SOLVED` on load-reference-1, with no receipt and no blocking diagnostic.
- Exactly one `SOURCE_BLOCK_RECOVERY_UNAVAILABLE`, naming "captured replay reservation" and `Exact(Budget)`.
- All records are `not_joined` with an ordinary recovery method.
- The runner raw equals the direct product call.
- The run is clean and the CLI exit is 0.
- The result is sensitive, so standing is `needs_recompute` and there is no document.
- It is not a blocked result.

## Checks

| Check | Result |
|---|---|
| `core/runner/headless` `cargo test`, baseline (before WP4) | 69 passed |
| The same, final, with all five artifact variables set | **83 passed**, 0 failed, 0 ignored (+14: lib +7, `load_reference_cli` 7) |
| Python: the new file plus `test_load_reference_readers.py` and `test_load_reference_source_readers.py`, with `HEADLESS_LOAD_REFERENCE_OUTPUT_DIR`, `HEADLESS_LOAD_REFERENCE_SOURCE_OUTPUT_DIR`, `LOAD_REFERENCE_PARITY_OUT` and `LOAD_REFERENCE_SOURCE_PARITY_OUT` set | **407 passed, 0 skipped** (new file 15) |

Commands and logs: `_run_records/COMMANDS.md`, `cargo_headless_baseline.log`, `cargo_headless_final.log`, `pytest_final.log`.

## Mutation evidence

The run used a scratch `git archive` copy with the WP4 files copied in, and a private target that has since been deleted. The unmutated copy passes. **17 of 17 mutants were killed** (`_run_records/mutants.log`, script `mutants.py`).

- **Identity routing and export gate:**
  - R1: the binding exports non-eligible standing;
  - P1: the 0.4.0 formulation basis is not routed;
  - P1b: the blocked 0.4.0 envelope leaves the exact namespace;
  - P2: the joined publication is labelled load-reference-1.
- **Receipt and invocation binding at the runner seam:**
  - R2: the runner solves in the other mode;
  - R3: the runner forwards a payload other than the retained one;
  - R4: the runner captures the wrong mode label;
  - R5, R6, R6b: the binding drops the invocation-request/payload equality, the closed mode set, and the invocation digest check.
- **Refusals at the runner seam:**
  - R7: the CLI typed input gate is bypassed;
  - R8: the runner reports `MECHANICS_SOLVED` for a blocked envelope;
  - R9: the `SOURCE_NOT_SOLVED` gate is dropped;
  - R10: the CLI exit ignores result validation.
- **Refusal emission in the product (via the runner tests):**
  - P3: the explicit null;
  - P4: the version mismatch;
  - P5: `deny_unknown_fields` on `AnalysisStateInput`.

For some mutants the log says only "KILLED". The script keeps just the tail of the cargo output, so the failing test's name sometimes falls outside it. The verdict comes from the nonzero test exit after a successful build.

## Deviations

- **Mutants used a private target in scratch**, with debuginfo off, not the shared target.
  - Why: mutant builds live at the scratch source path, and in the shared target they cannot be pruned by owner while other TASKs build.
  - The private target was deleted. Free disk stayed at 9.1 GB or more; it was 9.5 GB at the end.
- **The brief's item 2 "canonical document" was superseded** by the manager's ruling (above).

## Not done / open

- There is no joined canonical document through the binding. That is T3, with eligibility (ruling).
- The CLI does not expose the canonical document; that is the existing DEL-10-05 `export-results` follow-on. The CLI tests therefore check the raw and runner result inside the `ControlledExport` wrapper, and the library tests check the document.
- The 4 + 10 artifact sets live in scratch and are not committed. They are regenerated by the Rust command in `COMMANDS.md`.

## Design questions for the manager

1. **Receipt-less proof custody (characterization, not a defect claim).** For `load-reference-1`, as for other receipt-less identities, no received byte binds the invocation's solver mode. A crate-internal forgery of `QualifiedPreviewEvidence` that relabels the mode and re-digests the invocation is accepted by `build_result_export_document_with_evidence`, and the document differs only in `qualification_ref` and the derivative hash. The type's opacity (crate-private fields, no constructor or Deserialize) is the only custody. The test pins this as observed, so any tightening will show up as a deliberate change.
   - Minimal repro: in `load_reference_route_tests.rs`, `mode_tamper.actual_invocation["solver_mode"]` is set to the other mode, `invocation_digest` is recomputed, and the build returns `Ok`.
   - Whether this needs a binding (a mode-evidence check against the raw, such as the dense parity rows) is a product question. I made no change.
2. **ControlledExport default.** Every solved 0.4.0 CLI output needs `--explicit-local-private-intent`. The flagged path is `$.mechanics_envelope.accepted_model_state_mutated`. This is the existing policy, pinned here as observed. The VP-STATIC and harness adapters presumably pass the flag already. Say if T1 wants it recorded elsewhere.

## Manager integration note

- **Formatting.** The manager applied `rustfmt` to the two new Rust files, giving `load_reference_route_tests.rs` `93a806fc…` and `load_reference_cli.rs` `a71e5308…`. It also added a comment to the mode-relabel pin: the behaviour is observed, not required, and routed to T6. The re-run: headless 83 passed. The Python run on manager-regenerated artifacts: 407 passed, 0 skipped.
- **Finding 1 (ROOT's question).** The manager probe (`_run_records/manager_mode_relabel_probe.log`) shows the same digest-consistent mode relabel is accepted for `physics-1` and `preview-physics-1` in both modes. It is a pre-existing proof-custody property of the identity-agnostic binding, not a T1 defect. Per ROOT, it is open work routed to T6 (reader and proof hardening), and the pin stays as observed behaviour.
