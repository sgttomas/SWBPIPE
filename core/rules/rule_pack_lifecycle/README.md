# Rule-Pack Lifecycle

This crate is the bounded implementation slice for `DEL-06-04`. It records
rule-pack lifecycle metadata, privacy/redistribution state, checksum evidence,
and audit/report-facing manifest hooks without storing private rule-pack
payloads in the public repository.

## Scope

- Rule-pack identity, display name, schema version, rule-pack version, lifecycle
  status, source notice, privacy class, redistribution status, review status,
  and protected-content review flags.
- SHA-256 checksum records over caller-supplied payload bytes.
- Explicit checksum metadata that distinguishes caller-supplied JCS-compatible
  bytes from JSON canonicalization this crate has actually verified.
- Deterministic lifecycle findings for missing source notices, missing or
  unknown redistribution state, missing or stale checksums, suspected protected
  content, attempted public export of private content, and professional-boundary
  violations.
- Audit-manifest references that expose only identity, version, checksum, source
  notice, and privacy/redistribution metadata.

## Boundary

The crate does not parse JSON, canonicalize JSON, implement storage paths,
encrypt private rule packs, manage access control, handle secrets, run a GUI,
render reports, define API transport, or evaluate engineering/code rules.
Callers own canonical JSON serialization before hashing. PKG-12 owns storage,
access, and secret-handling policy.

## Verification

Unit tests cover deterministic SHA-256 checksums, unverified caller-supplied
JCS-byte metadata recording, missing metadata diagnostics, private export
blocking, suspected protected content quarantine findings, stale checksum
detection, audit hook redaction, and professional-boundary enforcement.
