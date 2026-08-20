# WORKING_ITEMS Run — R6 N1 Manager Remediation

- Run: `HELP-HUMAN-PIPING-20260820-R6-ENGINEERING`
- Package/deliverable: `PKG-02 / DEL-02-04`
- Trigger: attempt-3 fresh review findings.

Manager remediation stayed inside the frozen N1 product/test fence:

- quantity provenance with `protected_suspected` or `quarantined` now produces quarantine;
- plugin and unit diagnostic sources use the accepted `payload` reference type;
- diagnostic provenance normalizes invalid redistribution status to `TBD` and forces `review_status=rejected`;
- protected-content classification takes precedence over quantity/unit classification.
- positive adapter, plugin, and quantity quarantine markers are retained before
  incomplete metadata exits, so protected-content evidence cannot be downgraded
  to a generic rejection.
- diagnostic provenance preserves those canonical quarantine markers while
  normalizing unrelated missing fields to `TBD`; it forces `rejected` only when
  no positive quarantine marker exists.
- malformed adapter capabilities now emit a blocking finding without throwing,
  allowing simultaneous protected/quarantined provenance to remain visible and
  control the fail-closed outcome.
- adapter diagnostic context now selects operation-result provenance for
  `operation_result.*` findings and declaration provenance for declaration
  findings, preserving the exact affected source record.
- exact and descendant result/declaration roots are routed strictly; unrelated
  top-level findings receive absent fail-closed provenance instead of an
  unrelated declaration record.

Focused and existing adapter/plugin tests, including composed-result validation against the canonical adapter-operation-result schema and distinct-provenance missing-operation-result routing: `85 passed in 0.45s`. Scoped diff check: PASS. No schema, fixture, lifecycle, register, decision, DAG, decomposition, runtime-loader, network, telemetry, Git, or receipt change.

## Integrated-review amendment 1

The integrated 100% review found that composed top-level result privacy and
provenance still used public-reviewed invented fixture defaults. Amendment 1:

- removes invented provenance and public classification from `build_result`
  defaults, using absent fail-closed attribution instead;
- derives composed top-level boundaries from caller-supplied manifest,
  quantity, adapter-result, and adapter-declaration records;
- applies protected over private over review-required over public precedence;
- preserves the exact provenance selected by that restrictive boundary.

Manifest, quantity, and adapter-result private/protected regression cases plus
the complete focused/existing suite: `91 passed in 0.88s`. Canonical composed
result schema assertions pass for each new boundary case. Fresh full amended
N1 diff review remains required before manager/status/shared fan-in updates.

V13 review found that absent/malformed provenance and incomplete
public/accepted records could be masked by another public source. Remediation
ranks all absent, malformed, incomplete, and non-cleared provenance at least
review-required, allows public-reviewed only for complete canonical cleared
records, and treats malformed quantity/adapter nesting as conservative boundary
evidence. Missing quantity provenance, incomplete manifest provenance,
malformed unit evidence, and missing operation-result regressions now assert
non-public fail-closed top-level envelopes. Complete suite: `93 passed in
0.46s`; fresh V14 review required.

V14 review found incomplete private provenance lost private dominance and
missing/malformed required privacy could still rank below public provenance.
Remediation detects positive private provenance before completeness, converts
invalid required manifest/adapter privacy into review-required evidence,
preserves positive private/protected privacy markers despite unrelated control
gaps, and keeps quantity privacy explicitly not-applicable. Manifest, quantity,
and adapter incomplete-private plus required-privacy regressions pass. Complete
suite: `100 passed in 0.54s`; fresh V15 review required.

V15 review found manifest `private_data_access` and adapter privacy boolean
requirements were omitted from public-boundary completeness. Remediation checks
all canonical manifest privacy fields before public ranking, treats `TBD`
private-data access as review-required, and extends adapter validation/public
ranking to require canonical classification plus explicit
`export_review_required` and `private_payload_redacted` booleans. Missing,
invalid, and unresolved manifest access plus missing/wrong-type adapter boolean
regressions pass. Complete suite: `107 passed in 0.47s`; fresh V16 review
required.

## Integrated-review Amendment 2

Integrated review v2 found schema authentication relied only on canonical
identity markers and mapping shapes. Amendment 2 binds the caller-supplied,
already-loaded plugin schema to the exact canonical structural/content SHA-256
fingerprint before any manifest can verify; it does not load files or select a
runtime. Regressions remove checksum/professional-boundary required rules,
relax the checksum definition, and alter the professional-boundary definition;
all are rejected as noncanonical. Complete suite: `111 passed in 0.49s`;
fresh full amended-diff review required.

V29 fresh review confirmed the V28 manifest/unit-marker fixes but found raw
adapter capabilities were still traversed without a width bound after snapshot
failure and over-limit adapter/unit strings could enter diagnostic references.
Remediation is restricted to deterministic fallback traversal/reference bounds,
exact regressions, and a fresh full amended-diff review.

V29 remediation bounds raw capability fallback and constrains adapter/unit
diagnostic references to exact canonical strings of at most 256 UTF-8 bytes,
otherwise using deterministic bounded fallback identifiers. Nine adversarial
regressions cover overflow, noncanonical, and quarantine combinations. Complete
suite: `283 passed in 0.78s`; containment/diff PASS; V30 fresh full amended-
diff review required.

V30 fresh review found raw preflight invalid-path segments could enter
diagnostic references across every caller surface, and caller plugin-schema
normalization was not fully bounded/exception-contained. Remediation is limited
to bounded canonical diagnostic paths, bounded exact schema snapshotting,
adversarial regressions, and a fresh full amended-diff review.

V30 remediation sanitizes every preflight invalid-path component before it can
enter diagnostics and applies fully exception-contained bounded exact-JSON
normalization to caller plugin schemas before hashing/evaluation. Twenty-four
focused regressions cover adversarial keys and hostile/deep schema payloads.
Complete suite: `306 passed in 0.77s`; containment/diff PASS; V31 fresh full
amended-diff review required.

V17 review found the authenticated bytes and executed object could diverge for
a hostile Mapping accessor. Remediation serializes once, fingerprints those
exact bytes, parses a plain JSON snapshot, and uses only the snapshot for
identity and complete schema evaluation. A custom dict subclass whose
`get("required")` lies while underlying serialized items remain canonical can
no longer bypass root checksum/professional-boundary requirements. Complete
suite: `112 passed in 0.79s`; fresh V18 review required.

## Integrated-review Amendment 3

Integrated review v3 found adapter capabilities were converted from arbitrary
iterables to a set and were not checked against the canonical enum. Amendment 3
requires an exact list of canonical `AdapterCapability` strings, requires the
schema's operational `contains` constraint, and fails closed without throwing
for strings, non-list iterables, arbitrary tokens, mixed valid/invalid values,
and nested/unhashable entries. Canonical duplicates remain accepted because the
schema has no uniqueness constraint. Protected/quarantined provenance remains
visible and controls outcome precedence alongside malformed capabilities.
Complete suite: `126 passed in 0.49s`; fresh full amended-diff review required.

V19 fresh review found an unhashable `str` subclass could pass the broad string
shape check and raise during enum membership, masking declaration quarantine
provenance in the composed verifier. Remediation requires exact plain-string
items before enum membership. Direct declaration and composed-verifier
regressions combine the hostile string-like item with both protected and
quarantined provenance markers and require `QUARANTINE` without exception.
Complete suite: `129 passed in 0.50s`; fresh full amended-diff review remains
required.

## Integrated-review Amendment 4

Integrated review v4 found canonical-but-non-cleared adapter declaration/result
provenance and protected privacy could reach verification-passed blocked-runtime
results. Amendment 4 emits a blocking `ADAPTER_PROVENANCE_NOT_CLEARED` finding
unless complete canonical provenance has cleared redistribution and accepted
review, and emits quarantine for `protected_suspected` privacy. Direct and
composed regressions cover declaration and operation-result paths, malformed
capabilities, exact diagnostic routing, protected top-level dominance,
canonical result-envelope schema, and runtime non-dispatch. Complete suite:
`146 passed in 0.51s`; fresh full amended-diff review required.

V21 fresh review found truthy non-string provenance fields could pass
completeness and hostile unhashable status subclasses could throw before
protected privacy quarantine. Remediation requires every provenance field to be
an exact nonblank plain string, retains positive protected/quarantined markers
before completeness, and hardens diagnostic normalization plus composed
canonicalization/boundary ranking without mutating caller evidence. Direct and
composed declaration/result regressions include protected privacy and malformed
capabilities. Complete suite: `163 passed in 0.53s`; V22 fresh full amended-diff
review required.

V22 fresh review found plugin-manifest provenance retained the hostile status
membership path. Remediation applies exception-contained reads, exact
plain-string positive-marker handling, and a shared canonical snapshot before
membership/clearance. Direct manifest and composed protected-privacy tests
cover hostile redistribution/review values across declaration/result surfaces,
preserving structured rejection/quarantine, exact caller evidence, canonical
envelopes, and runtime non-dispatch. Complete suite: `169 passed in 0.54s`;
V23 fresh full amended-diff review required.

V23 fresh review found raw manifest schema enum comparison could invoke a
caller-overloaded equality operator before provenance validation. Remediation
serializes the manifest once with strict JSON settings, evaluates schema
against the detached plain snapshot, leaves caller evidence untouched, and
then rejects its non-exact provenance shape. Direct manifest and composed
protected-privacy regressions cover both status fields with raising comparators,
structured reject/quarantine, schema validity, and runtime non-dispatch.
Complete suite: `175 passed in 0.55s`; V24 fresh review required.

V24 fresh review found later manifest semantic checks still compared raw caller
values. Remediation adds exception-contained exact raw-JSON preflight for
subclasses, cycles, nonfinite numbers, and hostile containers, then routes
schema, all manifest semantics, composed diagnostic context, and envelope
boundaries through one detached strict snapshot. Raw provenance keeps its
specific structural finding; caller evidence remains unchanged. Complete suite:
`179 passed in 0.82s`; V25 fresh full amended-diff review required.

V25 fresh review found recursive raw preflight could overflow before its
exception boundary. Remediation replaces it with iterative active-ancestor
cycle tracking, exact-type checks, exception containment, and deterministic
maximum JSON depth 512 before strict serialization. Direct and composed deep,
cyclic, nonfinite, and hostile-container regressions require structured reject/
quarantine, protected envelope, schema validity, and runtime non-dispatch.
Complete suite: `190 passed in 0.52s`; V26 fresh review required.

## Integrated-review Amendment 5

Integrated review v5 found raw adapter, unit-catalog, and unit-evidence inputs
could escape public composed verification and malformed manifest structure could
discard safely readable protected markers. Amendment 5 normalizes each caller
input into a bounded exact-JSON detached snapshot before validation, lookup,
diagnostics, boundary ranking, or envelope construction. Hostile/custom
accessors, subclasses, cycles, depth/node/byte overflow, nonfinite values, and
serialization failures produce structured conservative contexts; safely
observable exact quarantine markers retain precedence. Direct/composed tests
cover every caller surface and malformed/protected combinations. Complete suite:
`219 passed in 0.56s`; fresh full amended-diff review required.

V27 fresh review found malformed manifest safe-marker fallback was incomplete,
manifest snapshotting lacked node/text/byte limits, and unit-evidence fallback
stopped scanning at 1,024 entries. Remediation preserves safely observable
provenance/metadata quarantine markers, enforces deterministic 10,000-node,
1 MiB text, depth-512, and 1 MiB serialized limits, and scans the complete
bounded evidence list. Direct/composed regressions cover marker positions 1,024
and 1,500 plus malformed manifest combinations. Complete suite: `253 passed in
0.64s`; V28 fresh full amended-diff review required.

V28 fresh review verified all 17 files/6,964 lines but found three blocking
fallback defects: hostile colliding dictionary keys could escape safe-marker
observation, unit fallback could scan arbitrary-width raw lists after snapshot
failure, and malformed-manifest fallback reused looser adapter provenance
bounds and raw identifiers. Remediation is limited to exception-contained
marker observation, deterministic fallback bounds, manifest-specific limits,
and exact regressions; fresh full amended-diff review remains required.

V28 remediation replaces raw dictionary lookup with bounded exact-key scans,
caps unit fallback at 2,048 entries and 64 keys/object using marker-only
detached evidence, and applies manifest-specific limits plus a canonical
256-byte ceiling to fallback plugin identifiers. Twenty-one direct/composed
regressions cover hostile keys and both accepted/excluded marker boundaries.
Complete suite: `274 passed in 0.67s`; containment and diff checks PASS; V29
fresh full amended-diff review required.

## Integrated-review Amendment 6

Integrated review v6 found finite caller-controlled plugin-manifest property
names and raw instance text could enter post-snapshot schema mismatch paths and
diagnostic references without canonical byte bounds. Amendment 6 canonicalizes
and bounds every schema-generated path segment and sanitizes/bounds mismatch
instance text. Direct/composed finite adversarial and near-limit key regressions
require protected-marker precedence, schema-valid envelopes, and runtime non-
dispatch. Complete suite: `314 passed in 0.85s`; containment/diff PASS; fresh
full original-basis amended-diff review required.

V32 review confirmed schema mismatch paths/messages are bounded but found
normalized schema-valid plugin IDs could bypass the existing 256-byte
diagnostic reference fence. Remediation routes every normalized diagnostic
plugin ID through `_safe_manifest_plugin_ref`; four over-limit/near-1-MiB direct
and composed regressions pass. Complete suite: `318 passed in 0.87s`;
containment/diff PASS; V33 fresh full amended-diff review required.
