# Checksum / Grammar-Version Binding Corpus (TP-C1-GRAMMAR-001)

Golden evidence for the DEC-022 rule that the declared expression
`grammar_version` (semver) sits **inside** the JCS-canonicalized bytes hashed
by `rule_pack_checksum`, so the FR-016 reported checksum binds the grammar
version. Executed by
`core/rules/rule_pack_lifecycle::tests::grammar_version_binding_fixture_corpus_is_stable`
under plain `cargo test` in that crate directory.

All payload contents are invented (`PUBLIC_DOMAIN_OR_ORIGINAL`); ids,
versions, and structure are synthetic and carry no engineering meaning.

## Files

| File | Bound grammar version |
|---|---|
| `payload_grammar_v1_0_0.json` | `1.0.0` |
| `payload_grammar_v1_1_0.json` | `1.1.0` (identical payload except the version member) |
| `payload_without_grammar_version.json` | none (identical payload minus the version member) |

Payload files are **exact bytes** (JCS-style: sorted keys, no insignificant
whitespace, no trailing newline). Do not reformat them; the golden hashes pin
byte stability.

## MANIFEST.tsv

Tab-separated rows: `file name`, `sha256 hex over the exact file bytes`,
`bound grammar version or "none"`. The test asserts:

1. each payload's SHA-256 equals its golden manifest value (byte + hash
   stability);
2. all three hashes are pairwise distinct (changing or removing the
   `grammar_version` member changes the rule-pack checksum — the binding
   property itself);
3. `payload_declares_grammar_version` byte-containment evidence agrees with
   the manifest's bound-version column, and the binding-enforcing checksum
   constructor accepts each bound payload.

Regenerate hashes only when a payload intentionally changes:
`shasum -a 256 payload_*.json`, then update the manifest in the same change
and explain the payload change in review.
