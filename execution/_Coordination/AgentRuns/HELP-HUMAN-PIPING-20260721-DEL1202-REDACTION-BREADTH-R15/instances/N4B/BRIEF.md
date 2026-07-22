# N4B Sealed Brief — Exact N5 remediation attempt 2

AgentRole: TASK  
TaskSkill: `software-bounded-implementation`  
ApplyEdits: true  
RequestedBy: HELP_HUMAN through WORKING_ITEMS  
RunID: HELP-HUMAN-PIPING-20260721-DEL1202-REDACTION-BREADTH-R15  
ChildInstanceID: N4B  
PackageID: PKG-12  
DeliverableID: DEL-12-02  
AcceptedBasis: `0c066652cd527eb1559f715e914262d2bda42602` plus immutable N4 attempt 1 delta  
Branch: `codex/piping-pkg12-redaction`

## Objective

Repair exactly the five findings in `instances/N5/RETURN.md`, within adopted
candidate v6 §6 and no other behavior:

1. Bind `NativePackagePanel` to immutable
   `DOTH-HANDOFF-002/downstream_tool`; correct mapping and tests so no fallback
   to `DOTH-JSON-001/local_private` exists.
2. Remove the TypeScript global-public inference based only on false payload/
   protected flags. Missing or incomplete per-leaf metadata remains unknown.
   Add an arbitrary unknown-field regression test.
3. In Rust, classify null/TBD/unknown values before private-path name hints so
   local-private unknowns remain `warning_only`. Test every private-named
   runner field (`mechanics_envelope`, `runner_result`, `suite_run`) for null
   and representative TBD/unknown values.
4. Suppress report iframe, save, and print when either pre-IPC redaction or the
   renderer returns `export_blocked`. Add a test that reaches a mocked
   renderer-blocked outcome rather than being masked by pre-IPC blocking.
5. Add subprocess-level tests invoking both existing binaries. Cover required
   active final-runner verbs, compatibility runner, stdout, `--output`, and
   exits 0/1/2, including blocking/no-file behavior and unchanged
   `export-results` stub. Do not add or change a verb.

## Fence and preservation

Candidate v6 §6 remains the exact write fence. Only files necessary for these
five repairs and focused tests may change, plus N4B evidence. Do not rewrite or
delete N4/N5 evidence or attempt-1 sweep. Do not change deliverable state,
receipt, final deliverable run record, Git state, protected/release tools,
parity vocabulary/corpus, transport, plugin/bug-report scope, or professional
claims. Stop on any need outside the fence. Do not delegate.

## Verification and sweep

Run focused tests for all five repairs, then the affected/full registered
checks and H4 source/dist flows. Only after they pass, execute exactly one new
late DEC-025 sweep for attempt 2. Preserve attempt 1's sweep unchanged and
label it superseded/non-acceptance; only attempt 2's sweep is eligible for
final acceptance. Run containment, protected-file, claims/path/JSON, and
`git diff --check` validation.

Return changed files, exact focused/subprocess behavior, commands/evidence,
attempt-2 sweep identity, scope validation, residual risk, and blockers. No
W3/state/Git closeout.

