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
