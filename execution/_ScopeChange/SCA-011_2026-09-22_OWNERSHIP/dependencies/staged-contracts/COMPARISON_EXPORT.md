# SCA011-IF-COMPARISON-EXPORT — staged contract draft v0.1

Status: candidate dependency-contract interpretation; not acceptance of the wire draft. Contract owner DEL-14-05; producer mechanics DEL-14-03/04; consumer/export mapper DEL-08-04. Required maturity SEMANTIC_READY; present readiness TBD; dependency PENDING.

## Three distinct artifacts

1. DEL-08-04's base result-envelope contract supplies the supported envelope/version dispatch and preservation constraints. Existing schema-first requirements and current 0.1/0.2 dispatch remain unchanged. This is the input named by preserved DAG-002-E0790; it does not require the later comparison-export extension already to be implemented.
2. DEL-14-05's common comparison contract binds the producer's left/right source identities and hashes, mappings and unmatched records, unit-aware tolerance references, diagnostics, provenance, limitations and export shape to an explicitly selected supported base-envelope version. The concrete review draft is `../../interfaces/ComparisonResult.v0.1.draft.schema.json`, with meaning and limits in `../../interfaces/INTERFACE_CONTRACTS.md`. Those paths resolve from this file's directory. Its draft wrapper/join is an unaccepted wire change; a named draft is not an authoritative published schema.
3. DEL-08-04's comparison mapping/export implementation consumes the accepted common comparison contract and the base envelope. It preserves the original producer output and the distinction between received assertions and checked derivative evidence. CSV columns/report layout, compatibility windows, tolerance values and engineering meaning are not selected here.

## Readiness and negative witnesses

Before this dependency is satisfied, the source owners must accept exact common-contract bytes and the base version it imports, then produce identity/mapping/unit/diagnostic/privacy round trips and invalid/stale/unmatched/unsupported-version cases. Refer to interface hooks IF-001/004/005/006/007. An invented fixture or a schema-shape pass alone cannot close source binding or engineering claims. Retain the existing DEL-14-03/04 upstream edges and require their producing outputs to conform separately.

The contract stage may be prepared before the dependent exporter implementation. If inspection shows the base contract cannot be supplied independently, revise this stage model rather than silently waiving E0790 or marking a cyclic prerequisite satisfied.
