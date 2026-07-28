# Handoff state

| Field | Value |
|---|---|
| Accepted upstream | decomposition revision 0.11 / SCA-008 |
| Graph examined | DAG-008 |
| Derivative status | `COMPLETE` |
| Audit status | `PASS` |
| Disposition | `CURRENT_BY_REVALIDATION` |
| DAG mutation | `NONE` |
| Dependency mutation | `NONE` |
| DAG pointer mutation | `NONE` |
| Rerun requirement | rerun if any dependency register, DAG artifact, DAG pointer, or accepted decomposition dependency relation changes |

The snapshot is an immutable evaluation derivative. It does not substitute for
the accepted decomposition, local dependency registers, or DAG-008. The
targeted DEC-063 / DEC-091 / DEL-16-04 reconciliation remains separately
blocked on durable D-59 activation.
