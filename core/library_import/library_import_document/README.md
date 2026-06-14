# `open_pipe_stress_library_import_document`

Runtime validation seam for material, section, and component **library import
payloads**. This crate is the runtime Rust port of the DEL-03-07 import
provenance contract authored in
[`core/library_import/provenance_checker.py`](../provenance_checker.py): the
Python module is the design authority, and this crate reproduces its semantics
so the Tauri/Rust desktop runtime can enforce the same import boundary the rest
of the application is built on (mirroring how
`open_pipe_stress_rule_pack_document` underpins the rule-pack editor).

It checks, at the already-parsed payload boundary:

- required provenance fields on each library object and record;
- redistribution and review disposition for public imports;
- private-only data handling (private imports may remain local);
- suspected protected-content **quarantine**;
- unit and value-level provenance metadata for imported numeric values;
- PKG-02-style import-boundary diagnostic envelope projection.

It does **not** parse external file formats and does **not** make legal
license or redistribution determinations. Unresolved rights remain review
findings for the human project authority. No status emitted here is a
professional, certification, sealing, authentication, or code-compliance
claim — every status is a software finding only.

## Public API

- `validate_library_import(payload, library_kind, intended_visibility) -> ImportValidationResult`
- `validate_library_import_tokens(payload, "material|section|component", "public|private")`
  — the seam entry point; returns `Err` for unsupported tokens (never guesses).
- `ImportValidationResult { outcome, library_kind, intended_visibility, accepted, findings }`
  with `diagnostics()` projecting findings into the PKG-02 envelope.

`outcome` is one of `QUARANTINE`, `REJECTED`, `REVIEW_REQUIRED`,
`PRIVATE_LOCAL_ONLY`, or `ACCEPTED_PUBLIC` (severity precedence in that order).

## Parity

`tests/provenance_parity.rs` mirrors `tests/test_library_import_provenance.py`
one-for-one over the **same** invented fixtures
(`fixtures/material/…`, `fixtures/component/…`). The two implementations move
together; the parity test is the guard against silent divergence.

```sh
cargo test --manifest-path core/library_import/library_import_document/Cargo.toml
```
