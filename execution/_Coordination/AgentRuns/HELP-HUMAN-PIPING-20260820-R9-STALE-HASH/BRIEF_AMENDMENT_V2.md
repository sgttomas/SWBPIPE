# N1 brief amendment v2 — truthful runtime binding status

- RequestedBy: `HELP_HUMAN`
- AppliedBy: `WORKING_ITEMS-PKG05-DEL0504-STALE-HASH`
- Date: 2026-08-20
- Classification: `AMEND` N1 in place; no new node and no acceptance change.
- Reason: the current `operation_outcome.schema.json` admits only the historical `claimed_hash_echoed_cross_canonicalization_equality_not_evaluated` status and the no-claim status. Retaining that claimed-hash status after runtime equality evaluation would be contradictory.
- Added write targets:
  - `projects/chirality-piping/schemas/operation_outcome.schema.json`
  - directly coupled focused schema test/type expectation required to keep the runtime output contract truthful
- Required semantics: stable truthful binding statuses for matched, mismatched, invalid, unsupported, and absent/no-claim states. Compatibility is preserved only where semantically true.
- Unchanged objective and exclusions: this remains the same DEL-05-04 runtime hash-binding objective; all lifecycle, release, receipt, DAG, decision, PRD, root, commit, push, and PR exclusions remain.

This amendment was frozen before edits to the added write targets.
