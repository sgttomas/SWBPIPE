# WORKING_ITEMS run — DEL-05-04 runtime stale-hash invalidation

- RunID: `HELP-HUMAN-PIPING-20260820-R9-STALE-HASH`
- Package / deliverable: `PKG-05` / `DEL-05-04`
- Branch / base: `codex/piping-product-20260820b` / `cd823be3badd034c86390f2707dcf01952c782f0`
- Accepted basis: approved DAG-009; DEC-020; current R5 stage; Receipt-120; exact open PDU-037 Remaining item.
- Activation and work graph: `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260820-R9-STALE-HASH/`.

## Product result

The authoritative Rust operation applier now evaluates a supplied claimed
model hash only when `algorithm=sha256`, `canonicalization=rfc8785_jcs`, and
`payload_scope=model_payload` make comparison with the backend hash meaningful.
A matching claim preserves ordinary validation/application. A mismatch blocks
with `OP-CLAIMED-MODEL-HASH-MISMATCH`; malformed evidence blocks with
`OP-CLAIMED-MODEL-HASH-METADATA-INVALID`; unsupported comparison metadata
blocks with `OP-CLAIMED-MODEL-HASH-METADATA-UNSUPPORTED`. Blocked outcomes
carry no applied model, applied-model hash, or user-application acceptance. No-claim
behavior retains the before-state staleness guard.

The operation-outcome schema and focused schema test now bind truthful
matched, mismatched, invalid, unsupported, and no-claim statuses. Existing
accepted corpus cases 01–03 are preserved byte-identically; case 15 proves a
matching claim, and new cases 79–81 prove the three negative classes across
native and Wasm execution. Both runners require exactly 81 cases.

## Verification completed before review dispatch

- `cargo test claimed_model_hash_gate_accepts_only_the_current_supported_model_basis` — PASS (1 focused test).
- `CORPUS_BLESS=1 cargo test --test contract_corpus contract_corpus_cases_match_the_rust_contract_reference` — PASS; generated expectations reviewed in the diff.
- `cargo test` — PASS (76 unit, 1 canonical-hash parity, 2 contract-corpus tests).
- `/Users/ryan/.local/share/mise/installs/python/3.13/bin/python3 -m pytest -q tests/test_operation_result_schemas.py` — PASS (5).
- `npm run build:wasm:desktop` — PASS.
- `npm test --workspace apps/desktop -- --run src/services/operationContractCorpus.test.ts src/services/operationService.test.ts` — PASS (171).
- `npm run test:desktop` — PASS (539).
- `npm run build:desktop` — PASS.
- Initial direct full-pytest attempt with the local Python installation — FAIL because `jsonschema` was not installed (813 passed, 30 failed, 5 collection errors, 17 skipped); classified as environment/dependency setup, not a product failure.
- `UV_CACHE_DIR=/tmp/chirality-r9-uv-cache uv run --python /Users/ryan/.local/share/mise/installs/python/3.13/bin/python3 --with-requirements requirements-dev.txt -- python -m pytest -q tests -n auto --dist loadscope` — PASS (902) using the pinned dev requirements in an isolated temporary environment.
- `/Users/ryan/.local/share/mise/installs/python/3.13/bin/python3 tools/practitioner_harness/harness.py self-check` — PASS/exit 0; pre-existing informational/review findings remain outside this tranche.
- `/Users/ryan/.local/share/mise/installs/python/3.13/bin/python3 -m pytest -q tools/practitioner_harness` — PASS (350).
- `git diff --exit-code HEAD --` for original corpus cases 01–03 — PASS; byte-identical.

The fresh read-only code review and DEC-025 sweep run after the integrated diff
is frozen; their terminal evidence is persisted in the named AgentRuns root.

## Boundary

Implementation evidence only. No lifecycle promotion, release, publication,
issuance, professional acceptance, certification, sealing, authentication, or
code-compliance claim. Standard claim fence applies (F-PIP-2; claims taxonomy
per DEC-081).
