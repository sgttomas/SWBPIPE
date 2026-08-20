# N1 integrated-review Amendment 7

Trigger: integrated review v7 after Amendment 6 / V33 fan-in.

Blocking finding: a schema-valid normalized manifest with
`metadata.status="quarantined"` emits a quarantine finding/outcome, but
normalized-success envelope derivation ignores that marker and can leave the
top-level envelope `public_permissive_reviewed`; malformed fallback already
maps the same marker to protected/quarantine boundaries.

Bounded remediation: make normalized and fallback boundary derivation
consistent. Quarantined metadata must force protected/quarantine top-level
privacy/provenance and never public-reviewed. Add direct/composed schema-valid
quarantined-metadata regressions interacting with private/public/protected other
evidence, canonical envelope schema, and runtime non-dispatch.

No schema, runtime, lifecycle, N2/N3, Git, receipt, PR, or owner ruling.
Existing closeout remains unchanged pending complete proof and fresh review.

Implementation complete: normalized quarantined metadata contributes the same
protected conservative envelope boundary as fallback. Three direct and three
composed public/private/protected interaction regressions preserve canonical
envelopes and runtime non-dispatch. Complete suite: `324 passed in 1.10s`;
containment/diff PASS. Fresh review required.

V34 matched all 23 hashes/line counts, reviewed all 8,654 frozen lines and the
full original-basis amended diff, independently reran `324 passed`, and returned
`PASS` with zero findings. Amendment 7 is complete and valid for fan-in.
