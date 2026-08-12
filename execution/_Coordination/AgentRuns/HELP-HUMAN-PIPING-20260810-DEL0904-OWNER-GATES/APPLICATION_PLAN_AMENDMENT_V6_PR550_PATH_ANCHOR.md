# Application Plan Amendment V6 — PR #550 Historical Path-Anchor Repair

- RunID: `HELP-HUMAN-PIPING-20260810-DEL0904-OWNER-GATES`
- Parent: `HELP_HUMAN` / Agent 0
- Manager: `WORKING_ITEMS` / Agent 1
- Repair basis: PR #550 head `1613ebfae29634242cf140d55b7309e945ba43f1`
- Owner authority: direct 2026-08-11 direction to repair PR #550 and get it
  merged
- Scope effect: none on the DEL-09-04 owner rulings, TM-PIP-037 closure,
  Receipt 99, or DEC-025 evidence

## Defect and preservation rule

The PR harness reports `ABS_PATH_IN_PROJECT_SURFACE` for the two
machine-specific anchors in
`instances/A2_APPLICATION_VERIFY_POST_CLEANUP/LAUNCH_BRIEF.md`. That sealed V1
brief is completed historical control evidence. Amendment V5 explicitly
preserved its brief, status, and return unchanged and dispatched the tokenized
V2 verifier as the accepted terminal basis. Rewriting V1 would falsify that
record and contradict the accepted V1/V2 provenance chain.

The project portability policy provides a hash-bound
`control_path_exception` mechanism for exactly this class of immutable
completed launch record. The repair therefore preserves V1 byte-for-byte,
binds its current SHA-256 in that policy, and changes no execution meaning.

## Frozen work graph V6

- Posture: terminal author then fresh verifier.
- Selection authority: direct owner repair direction and Agent 0 launch.
- N1 author: `A2-PR550-PATH-ANCHOR-EXCEPTION-AUTHOR`; add exactly one
  hash-bound historical-control exception and write its own structured return.
- N2 verifier: `A2-PR550-PATH-ANCHOR-EXCEPTION-VERIFY`; after N1 fan-in,
  independently validate byte preservation, policy exactness, path-anchor
  semantics, relevant harness, receipt continuity, containment, whitespace,
  and Git state; it may write only its own return/status records.
- Edge: N1 terminal return accepted before N2 dispatch.
- Integration owner: WORKING_ITEMS for the manager validation and handoff.
- Escalation: any change to the frozen V1 brief, any finding outside the known
  two lines, any policy issue, any unrelated delta, or any requirement for a
  semantic owner-ruling amendment is `HOLD`.

No staging, commit, push, merge, fetch, rebase, reset, clean, deletion,
receipt append, lifecycle action, release act, or professional-approval effect
is authorized in this WORKING_ITEMS tranche.
