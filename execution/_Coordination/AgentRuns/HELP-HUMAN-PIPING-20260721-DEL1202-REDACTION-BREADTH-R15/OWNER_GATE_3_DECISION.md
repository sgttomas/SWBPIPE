# Owner Gate-3 Decision — Candidate v6 adoption

**Recorded:** 2026-07-21 owner session  
**Candidate:** `CB-2026-07-21-DEL-12-02-REDACTION-BREADTH-001`, v6  
**Decision:** `ADOPTED`

The human adopts v6 and explicitly authorizes the bounded cross-package
integration scope in `AFFECTED_OWNER_MAP.csv`, with:

- N4 as the sole serialized implementation owner;
- N5 as the fresh independent verifier; and
- DEL-12-02-only W3 closeout only after N5 returns `COMMIT-SAFE`.

The human separately authorizes HELP_HUMAN to self-merge its PRs. Commit,
push, PR, and merge remain outside WORKING_ITEMS scope and route through
CHANGE after validation.

This decision activates only candidate-v6 N4 implementation within its exact
write fence. It does not authorize scope expansion, a new runner verb, plugin
or bug-report features, lifecycle/stage/release/issuance effects, receipt
closeout, professional/legal/security sufficiency claims, or any N4/N5 write
to deliverable state.

