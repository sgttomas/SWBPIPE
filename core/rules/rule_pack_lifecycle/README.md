# Rule-Pack Lifecycle

This crate is the bounded implementation slice for `DEL-06-04`. It records
rule-pack lifecycle metadata, privacy/redistribution state, checksum evidence,
and audit/report-facing manifest hooks without storing private rule-pack
payloads in the public repository.

## Scope

- Rule-pack identity, display name, schema version, rule-pack version,
  expression grammar version (DEC-022), lifecycle status, source notice,
  privacy class, redistribution status, review status, and protected-content
  review flags.
- SHA-256 checksum records over caller-supplied payload bytes.
- Explicit checksum metadata that distinguishes caller-supplied JCS-compatible
  bytes from JSON canonicalization this crate has actually verified.
- Grammar-version checksum binding per DEC-022: the declared `grammar_version`
  (strict semver) must sit inside the JCS-hashed rule-pack payload bytes so
  the FR-016 reported checksum binds it. The crate provides the exact JCS
  member encoding (`grammar_version_jcs_member`), a byte-containment evidence
  check (`payload_declares_grammar_version` — a necessary-condition check on
  caller bytes, not JSON parsing), and a binding-enforcing checksum
  constructor.
- Deterministic lifecycle findings for missing source notices, missing or
  unknown redistribution state, missing/malformed/unbound grammar versions,
  missing or stale checksums, suspected protected content, attempted public
  export of private content, and professional-boundary violations.
- Audit-manifest references that expose only identity, version, grammar
  version, checksum, source notice, and privacy/redistribution metadata.

## Boundary

The crate does not parse JSON, canonicalize JSON, implement storage paths,
encrypt private rule packs, manage access control, handle secrets, run a GUI,
render reports, define API transport, or evaluate engineering/code rules.
Callers own canonical JSON serialization before hashing (the grammar-version
binding check is byte containment over caller-supplied bytes, never JSON
parsing). PKG-12 owns storage, access, and secret-handling policy.

## Verification

Unit tests cover deterministic SHA-256 checksums, unverified caller-supplied
JCS-byte metadata recording, missing metadata diagnostics, grammar-version
findings (missing, malformed, unbound), the binding-enforcing checksum
constructor, private export blocking, suspected protected content quarantine
findings, stale checksum detection, audit hook redaction, and
professional-boundary enforcement. The golden checksum-binding corpus
`fixtures/rule_expressions/checksum_binding/` pins byte/hash stability and
the DEC-022 property that changing or removing the `grammar_version` member
changes the rule-pack checksum.
