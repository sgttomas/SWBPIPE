# N1 integrated-review Amendment 2

Trigger: integrated review v2 after Amendment 1 / V16 fan-in.

Blocking finding: `_verify_manifest_schema()` authenticated a caller-supplied
schema using identity markers and container types only. A weakened lookalike
could retain those markers while removing required checksum or professional-
boundary rules, relaxing definitions, and allowing a noncanonical manifest to
reach verification.

Bounded remediation: deterministically canonicalize the already-loaded schema
mapping in memory and require its SHA-256 structural/content fingerprint to
match the canonical plugin-manifest contract before evaluating the manifest.
No file/runtime loading is introduced. Exact removed-required and altered-
definition regressions are included.

Checks before freeze: full focused/existing N1 suite `111 passed in 0.49s`;
composed envelope schema assertions included; runtime dispatch remains blocked.
Manager/handoff/shared fan-in updates remain deferred until fresh full amended-
diff review passes.
