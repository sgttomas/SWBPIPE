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

V17 review found the authenticated bytes and executed object could diverge for
a hostile Mapping accessor. Remediation serializes once, fingerprints those
exact bytes, parses a plain JSON snapshot, and uses only the snapshot for
identity and complete schema evaluation. A custom dict subclass whose
`get("required")` lies while underlying serialized items remain canonical can
no longer bypass root checksum/professional-boundary requirements. Complete
suite: `112 passed in 0.79s`; fresh V18 review required.
