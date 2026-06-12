# Canonical-Hash Parity Corpus (H1 / F-5a; RFC 8785 since H5)

Pins the engine's canonical JSON form and its SHA-256 for invented inputs,
as the cross-lane regression surface for the unified frontend hash seam.
The canonical form is RFC 8785 (JCS) since completion-plan hardening row
H5: object keys sorted by UTF-16 code units, ECMAScript `Number::toString`
number rendering (shortest round-trip, exact ties to even, `-0` as `0`),
`JSON.stringify` string escaping. Implementation:
`core/serialization/canonical_json`, re-exported by the operation-applier
engine and consumed by the wasm exports.

- **Provenance:** all values invented for this corpus; no project data, no
  protected standards content.
- **Blessing authority:** the native Rust engine.
  `CANONICAL_HASH_BLESS=1 cargo test` in
  `core/model_operations/operation_applier` regenerates
  `expected_canonical` / `expected_sha256`; review the diff before
  committing. The native test enforces a 20-case coverage floor.
- **Lanes:** native (`tests/canonical_hash_parity.rs`) and wasm
  (`apps/desktop/src/services/hashService.test.ts` through the
  `canonical_json_string` / `canonical_sha256_hex` exports).
- **Inputs are raw JSON text** (`input_json` strings) so both lanes parse
  identical bytes. Number rendering is pinned behavior
  (`1e9` → `1000000000`, `200.0` → `200`, `1e21` → `1e+21`), and
  pre-parsed values would erase it.
- **One documented divergence from a strict double-only JCS:** integers
  beyond 2^53 keep their exact serde i64/u64 rendering instead of
  collapsing to the nearest IEEE double (see
  `number-beyond-2-53-integer-kept-exact`). Such values sit outside the
  I-JSON envelope RFC 8785 assumes, and production payloads reach the
  engine through `JSON.stringify`, so a JS lane can never produce that
  text distinct from its rounded double.
