# Canonical JSON profiles

The existing unrestricted `canonical_json`, checked `openpipestress_jcs_ijson_v1`
APIs and `openpipestress_jcs_ijson` executable retain their historical behavior.
They do not automatically retry with another profile.

`openpipestress_jcs_binary64_v1` is a separate scientific transport foundation.
Every JSON number in its generic text API denotes the nearest finite binary64
value. It uses the Rust ECMAScript formatter for canonical bytes, including
large finite integral-valued reals. For example `9007199254740993`,
`9007199254740993.0` and `9.007199254740993e15` all canonicalize to
`9007199254740992`; a semantic hash cannot authenticate their original spelling.
This is not exact decimal or arbitrary-precision integer storage.

The new parser rejects duplicate names after escape decoding, invalid JSON,
lone surrogates, Unicode noncharacters (also in root strings), overflow,
nonzero tokens rounding to zero, and negative-zero input. Representable
subnormals are admitted. String normalization is not performed. Property names
sort by UTF-16 code units. The underflow and negative-zero policies are
scientific ingress restrictions; the unchanged standalone formatter still
renders historical IEEE negative zero as `0`.

## Rust APIs and exact integer fields

Use `binary64::parse_binary64_v1_text` for raw text. The returned document
retains number lexemes so a typed schema adapter can call
`exact_integer_at("/count", 0, limit)` **before** extracting its `Value` or
building a DTO. Count/index fields accept only integer tokens (no exponent,
decimal point or negative zero), with bounds inside ±9007199254740991. Safe
integer tokens keep their serde integer representation for typed DTOs. Generic
canonical rendering nevertheless uses binary64 for every number. No field is
inferred to be a counter from its name; each adopting schema must inventory
and check its exact fields, required fields and narrower bounds.

`canonical_json_binary64_v1_text` is the generic scientific renderer.
`canonical_json_binary64_v1_value` checks an actual supplied serde snapshot:
unsafe i64/u64 values require an explicit typed-owner conversion to f64, or the
raw-text scientific route. It cannot detect source text already discarded by a
foreign parser, or recover NaN previously converted into JSON null.
`exact_integer_token` and `exact_integer_value` support bounded typed adapters;
they do not qualify a complete record by themselves.

New-profile resource limits are 8MiB UTF-8 for both received and canonical
documents, depth128 (root depth0),
262144 value nodes,4096bytes per numeric token, and16MiB cumulative JSON Pointer
construction. The latter is checked before escaping/allocation to bound
repeated long-key amplification in token capture. Direct Rust snapshots apply
the same path-length accounting without allocating paths, so their output cannot
bypass the parser's resource contract. Object keys are not counted
as value nodes. These are prospective transport admission limits, not solver
capability limits or changes to old-profile acceptance. Canonical output bytes (including the parsed-document method), programmatic
snapshot output bytes and cumulative escaped request bytes are checked before full
serialization; repeated aliases pay their full expansion each time.

## CLI and Python

Build explicitly; runtime discovery never builds, searches PATH, invokes a
shell, or falls back to a different serializer:

```sh
python tools/serialization/build_checked_json.py --profile openpipestress_jcs_binary64_v1
```

The executable is `openpipestress_jcs_binary64`, beside the old executable in
`canonical_json/target/checked-json/release`. Python can instead use an explicit
`OPENPIPESTRESS_BINARY64_JSON_BIN` path. The old environment variable continues
to select only the old authority.

The new CLI reads one UTF-8 JSON request from stdin, requiring exactly
`protocol_version`, `profile`, and `items`:

```json
{"protocol_version":"1.0.0","profile":"openpipestress_jcs_binary64_v1","items":[{"id":"value","json_text":"1e160"}]}
```

It returns the same protocol/profile plus ordered `{id, canonical_json}` rows.
Requests have a64MiB envelope limit and4096-item maximum; every item also obeys
the document limits. Unknown/duplicate fields, duplicate/empty IDs and unsupported
protocol/profile pairs fail the entire request with exit2 and no partial stdout.
A caller must not interpret the profile echo as a schema or numerical-method
qualification.

Python `canonicalize_binary64_text_batch` and `canonical_json_binary64_v1_text`
send original text directly to Rust. Their generic numeric domain is scientific
real; schema-owned lexical exact-count checks are not performed by these generic
calls. Python snapshot APIs `canonicalize_binary64_batch` and
`canonical_json_binary64_v1` accept only plain dict/list, str, bool, null, safe
integers and finite floats, rejecting unsupported types, cycles, invalid Unicode
and negative zero before JSON encoding. `scientific_real(int_or_float)` is an
explicit real-domain conversion; `exact_integer(value, minimum, maximum)` checks
programmatic counters and rejects bool/float. Decimal/Fraction and custom classes
need a separate explicit conversion or string representation.

The SHA-256 helpers hash **Rust-produced canonical bytes**. They do not implement
Python canonicalization, domain-separated receipts or raw-text custody. New
record carriers must name their profile/payload scope/version and use their own
versioned receipt domains. Original source bytes and their scoped digest must
be retained separately wherever raw custody is claimed; reserialization does
not recover lost input. Hashes from old and new profiles must never be mixed.

## Adoption boundary

This addition does not activate new solver producers, result readers,
operation/native/WASM exports, browser object freezing, input capture receipts,
public-zero projection, stress units, Current/rules/export standing or schema
versions. Those are explicit successor integrations, including schema-specific
integer inventories and actual invocation binding. No solver accuracy follows
from successfully transporting a finite value. Computed -0 requires a separately
versioned producer projection **before** row binding; this parser will not repair
an untrusted negative-zero result.

Tests exercise Rust native APIs and CLI, Python and stock Node parsing/rendering,
adversarial ingress, declared limits and existing-profile behavior. Stock Node
reference checks are not product WASM or frontend adapter qualification.

References: [RFC8785](https://www.rfc-editor.org/rfc/rfc8785.html),
[RFC7493](https://www.rfc-editor.org/rfc/rfc7493.html), and the prospective design
at `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/CORRECTNESS_DESIGN/SCIENTIFIC_JSON_PROFILE/`.
