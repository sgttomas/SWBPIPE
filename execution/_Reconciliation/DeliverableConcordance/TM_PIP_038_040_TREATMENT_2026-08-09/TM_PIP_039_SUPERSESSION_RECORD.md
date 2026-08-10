# TM-PIP-039 supersession record — 2026-08-09

Status: `DERIVATIVE — SUPERSESSION RECORDED — NON-AUTHORITATIVE`

Treatment verdict: `ADOPT — RECORD-ONLY TREATMENT COMPLETE`

This record binds stale prospective wording in the immutable D-41 W3 pause
entry to later committed evidence. It does not amend that entry, reinterpret
the historical owner direction, revive the consumed resume prompt, or perform
a Task Management register disposition.

## Historical owner quote and act preserved

The historical owner quote remains verbatim in the accepted D-41
`RUN_BASIS.md`:

```text
"Pause after W3 lands
  clean and do not set off on W4.  Instead, prepare for handoff to another
  agent in a new session.  Update the `init/piping-resume-one-time.md` file
  accordingly."
```

The act recorded with that quote also remains unchanged: the run paused after
W3, prepared the handoff, left W4 undispatched at that boundary, and identified
the then-current resume point and one-time prompt. The source entry's exact
bytes, including its contemporaneous state and owner-act wording, are
reproduced and hashed in
`SOURCE_EVIDENCE/TM_PIP_039_SUPERSESSION_EVIDENCE.md`. This derivative record
does not recode later events into that earlier act.

## Prospective clauses that no longer operate

The following clauses are true evidence of the 2026-07-12 pause state but are
not present instructions:

1. `opus pilots per the Receipt-17 steer` was a forward-looking assignment.
   Owner direction recorded by Receipt 25 rescinded that named-model steer
   going forward while preserving it as history of how earlier waves ran.
2. The direction to resume through `init/piping-resume-one-time.md` was a
   one-time pointer. The committed prompt was subsequently updated for later
   D-41 stages, consumed, and deleted by the owner-directed deletion commit.
3. `W4 ... is NOT dispatched` states the pause-boundary fact. Later committed
   D-41 records establish W4 and W5 completion, R6 backcheck completion, and
   the bounded R7 DEC-076/PDU-077 re-extraction. Those later facts supersede
   only the clause's prospective operating effect; they do not alter what was
   true when the pause was recorded.

## Committed supersession chain

| Order | Commit | Committed effect relevant to TM-PIP-039 |
|---|---|---|
| 1 | `92baf0b7e3a0a0e09829a766b72a6b3ebae2166a` | Recorded the W3 wave boundary, pause entry, package summaries, and Receipt 24. |
| 2 | `a02feaae9d0640e9a7d597db090e477fb2e08767` | Corrected the contemporaneous PR #198 merge status in the pause record without changing the owner quote or resume clauses. |
| 3 | `61c0d115832db8e7619eec7fe2cd119d313ab19d` | Recorded Receipt 25 and replaced live named-model assignments with model-agnostic operating rules; prior steers survive only as historical evidence. |
| 4 | `bdcfb7dcaf8394e3aedb3c00ebc7366a7a0f0e9c` | Committed the model-agnostic one-time D-41 resume prompt. |
| 5 | `4b2a9992da6177552f261326892e138e5b546727` | Updated that prompt for the later W5 dispatch posture and recorded the corresponding D-41 basis. |
| 6 | `4d4ea08b457a80a7d925f99a4dcc45c0b9f5eaf6` | Completed the D-41 R6 backcheck and run summary. |
| 7 | `e2ed4d3471df38bca2371ed621ac53db19cb3fe6` | Completed the bounded R7 DEC-076/PDU-077 re-extraction record. |
| 8 | `6bb723fc373db30206d36505fa4194d97af756ff` | Owner-directed deletion of the consumed `init/piping-resume-one-time.md`; the commit message identifies D-41 as complete and preserves the residual route. |

Every commit in this chain is an ancestor of the frozen treatment base
`da40d7dc4192c9aa2f49e9438729179aae281b61` where tested. The final base tree
does not contain `init/piping-resume-one-time.md`; its last pre-deletion blob
remains retrievable from Git history.

## Present-only operating statement

At the frozen 2026-08-09 treatment base, no prospective instruction in the W3
pause entry is active. D-41's one-time resume pointer is retired. Any new work
enters through the current Piping loop, current committed plan and runtime
instructions, and a current owner direction; current `LOOP_INIT.md` section 7
expressly says historical section references do not override current plan or
canonical runtime instructions.

This statement is descriptive only. It grants no dispatch, lifecycle,
evidence-disposal, reconciliation-repair, Git, or register authority.

## Authority and closure boundary

- This record is a derivative supersession record, not decomposition truth or
  a replacement for the immutable D-41 run record.
- Treatment recommends `RESOLVED_WITH_CHANGE` for later owner consideration;
  it does not enact that disposition.
- TASK_MANAGEMENT retains register disposition authority under a separate
  owner closure ruling.
- No owner, engineering, lifecycle, release, compatibility, security, legal,
  or professional-reliance decision is made here.
