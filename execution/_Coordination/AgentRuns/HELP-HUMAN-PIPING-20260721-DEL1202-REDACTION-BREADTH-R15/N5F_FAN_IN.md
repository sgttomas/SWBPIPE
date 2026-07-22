# WORKING_ITEMS Fan-in — N5F fresh attempt-6 verification

**Verdict:** `BLOCK`

N5F confirmed the ordinary additive gate and H4 rollback, but reproduced three
material gaps:

1. `target_mapping_contract=None`, `target_fixture=None`, and manifest
   `units_manifest=None` raise on pre-diagnostic dereferences before their
   declared missing-source blocking diagnostics can reach the gate.
2. Lossless-only payload withholding returns `blocked=true` while summary
   `exposure_blocking_count=0`, making blocker evidence internally inconsistent.
3. The seven passing workflow tests do not cover those missing-input or
   lossless-summary cases.

The reviewer otherwise reconciled reachable blocking/warning behavior, A3
rollback, 31/31 route/disposition/owner coverage, six immutable sweep hashes,
239-path scope containment, and protected/state/Git holds.

No repair, sweep, DEL-12-02 closeout, receipt, lifecycle, release, or Git
effect occurred. W3 remains held. Further work requires bounded HELP_HUMAN
authorization and a fresh implementation/verifier pair.

