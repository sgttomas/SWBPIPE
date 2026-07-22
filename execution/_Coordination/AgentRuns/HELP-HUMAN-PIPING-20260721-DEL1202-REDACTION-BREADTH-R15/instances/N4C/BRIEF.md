# N4C Sealed Brief — Leaf-explicit remediation attempt 3

AgentRole: TASK  
TaskSkill: `software-bounded-implementation`  
ApplyEdits: true  
RequestedBy: HELP_HUMAN through WORKING_ITEMS  
RunID: HELP-HUMAN-PIPING-20260721-DEL1202-REDACTION-BREADTH-R15  
ChildInstanceID: N4C  
PackageID: PKG-12  
DeliverableID: DEL-12-02  
Branch: `codex/piping-pkg12-redaction`

## Objective

Fix only N5B's cross-language payload-wide public-basis defect:

- in Python `core/security/redaction/route_control.py`;
- in TypeScript `apps/desktop/src/features/redaction-controls/redactionExportControls.ts`.

Root/container privacy or provenance may not promote arbitrary sibling leaves.
Public treatment must be leaf-explicit and require both explicit privacy and
redistribution metadata at the leaf or an exact bounded structural projection.
Containers carry structure only; missing leaf metadata remains unknown.

Add focused Python and TypeScript regressions with public envelope provenance/
privacy and an opaque unmetadataed sibling. Prove the sibling redacts in public/
shared/downstream contexts and does not become public; local-private behavior
must preserve the existing unknown `warning_only` posture.

## Fence and preservation

Candidate v6 §6 remains governing. For attempt 3, product/test writes are
limited to the two projector files above and their existing focused Python/
TypeScript tests under the candidate fence, plus N4C evidence. Do not change
any other implementation behavior, prior evidence, state, receipt, final run
record, protected/release tools, parity corpus/vocabulary, or Git state. Do not
delegate. Stop on any broader need.

## Verification and sweep

Run focused cross-language regressions, then full affected registered checks,
H4 source/dist, containment/protected/claims/path/JSON/diff validators. Only
after all pass, execute exactly one late attempt-3 DEC-025 sweep. Preserve
attempts 1 and 2 byte-identically and label both superseded/non-acceptance;
only attempt 3 is acceptance-eligible.

Return exact changes, tests/evidence, attempt-3 sweep identity and checksum,
prior-sweep checksums, containment, residual risk, and blockers. No W3/state/
Git closeout.

