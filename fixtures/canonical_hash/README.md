# Canonical-Hash Parity Corpus (H1 / F-5a)

Pins the engine's canonical JSON form (recursively sorted object keys,
serde serialization) and its SHA-256 for invented inputs, as the
cross-lane regression surface for the unified frontend hash seam.

- **Provenance:** all values invented for this corpus; no project data, no
  protected standards content.
- **Blessing authority:** the native Rust engine.
  `CANONICAL_HASH_BLESS=1 cargo test` in
  `core/model_operations/operation_applier` regenerates
  `expected_canonical` / `expected_sha256`; review the diff before
  committing. The native test enforces a 12-case coverage floor.
- **Lanes:** native (`tests/canonical_hash_parity.rs`) and wasm
  (`apps/desktop/src/services/hashService.test.ts` through the
  `canonical_json_string` / `canonical_sha256_hex` exports).
- **Inputs are raw JSON text** (`input_json` strings) so both lanes parse
  identical bytes: number normalization (`1e9` -> `1000000000.0`,
  `1e21` -> `1e+21`) is pinned behavior, and pre-parsed values would erase
  it. Production payloads reach the engine through `JSON.stringify`, so
  integers beyond 2^53 never arrive distinct from JS — the corpus stays
  inside f64-exact range by design.
