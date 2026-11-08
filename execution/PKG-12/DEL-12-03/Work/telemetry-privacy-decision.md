# Telemetry privacy: pending owner choice

`docs/security/threat_model.md` (Scope, Telemetry trust boundary and STM-004)
allows a user-selected private payload. TEL-REQ-004 and CONTRACT
OPS-K-PRIV-2 prohibit private engineering/code data in telemetry. PRD
FR-RULE-004's "unless explicitly exported" can blur export with telemetry.
Finding the contract alone does not settle this conflict.

The proposed options are:

1. Remove the threat-model exception and clarify FR-RULE-004: explicit export
   is not telemetry. This is Agent 0's recommendation; it preserves the
   restrictive contract and separately configured provider traffic under
   OPS-K-PRIV-1/DEC-051.
2. Permit selected private telemetry only through an explicit owner amendment
   to the policy, contract and matching requirements. Export permission alone
   supplies no such exception.

No owner choice is recorded. Retain the restrictive contract pending decision.
Next actor: Agent 0 presents these options to the owner; reconcile the governing
texts together after the decision. This note changes no permission or policy.
