# Audit Manifest

This crate is the bounded implementation slice for `DEL-08-02`. It records
reproducibility metadata for model/run evidence without storing private project
payloads or claiming professional approval.

## Scope

- Project-local deterministic JSON value serialization with sorted object keys
  for hash input.
- SHA-256 hashes for model payloads, manifest entries, rule-pack checksum
  references, and non-JSON assets.
- Solver version stamp, unit-system reference, payload references, asset
  manifest entries, and rule-pack audit references.
- Deterministic findings for missing model hash, solver version, unit-system
  reference, rule-pack checksum/source notice, asset hash, provenance, and
  professional-boundary violations.

## Boundary

The crate does not parse project files, choose a physical project container,
canonicalize arbitrary caller text, implement or validate full JCS
canonicalization, store private rule-pack payloads, import protected standards
content, authenticate engineering work, or emit professional/code-compliance
claims. Callers own project persistence, private storage, report rendering, API
transport, and final result-envelope integration.

## Verification

Unit tests cover project-local key-order stability, explicit non-JCS
canonicalization metadata, hash change on material input changes, non-JSON asset
hash recording, rule-pack checksum capture, missing manifest findings, private
payload redaction, and absence of professional or code-compliance statuses.
