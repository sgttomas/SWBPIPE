# Candidate result-export validator contract V4

Status: successor to frozen V3; not adopted or implemented.

The runtime validator must be total over its documented input domain without catch-all exception swallowing. It must complete strict host-JSON representability and a central Draft 2020-12 structural/type phase before it accesses a semantic relation. A relation includes `.get`, mapping-key construction or lookup, string operations, equality projections, owner/status dispatch and interpretation dispatch.

Structural/type errors return `RESULT_EXPORT_SCHEMA_INVALID`. Non-finite host numbers return `RESULT_EXPORT_NON_FINITE_NUMBER`. After structural/type validity is established, existing stable codes remain required for duplicate IDs, mirror/reference disagreement, unknown well-typed native pair, false owner, other classification disagreement and unsupported version.

Acceptance requires a generated matrix across every relation-consumed field and null, boolean, number, string, array and object substitutions at both writer and reader boundaries. It also requires a writer-only non-string-key case, finite-number cases, well-typed semantic regressions, a valid full witness, and zero uncaught exceptions. The matrix must be updated whenever a new relation-consumed field is added.
