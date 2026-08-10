# Structured return — TM-PIP-039

- Parent: managed `RECONCILIATION` Agent 1
  `/root/reconciliation_tm038_040_treatment`.
- Child form: ephemeral Agent 2, sealed brief
  `CHILD_BRIEFS/TM_PIP_039_BRIEF.md` SHA-256
  `38a8d7a69e49d60f9170f026d1d554c856353b092dcbe1b21d40dc4a66faba41`.
- Frozen basis: `RUN_BASIS.md` SHA-256
  `d7d2ad304c7b9f605f7549156327638edaf8a2dcad2d0e656138daba8902525e`;
  source state `da40d7dc4192c9aa2f49e9438729179aae281b61`.
- Status: `PASS — DURABLE SUPERSESSION RECORD PRODUCED`.
- Treatment verdict: `ADOPT — RECORD-ONLY TREATMENT COMPLETE`.
- Recommended eventual register disposition: `RESOLVED_WITH_CHANGE`, held
  for a separate owner ruling and TASK_MANAGEMENT action.

## Outputs

1. `TM_PIP_039_SUPERSESSION_RECORD.md` — derivative, non-authoritative
   supersession record with the historical quote preserved verbatim, stale
   clauses identified, committed supersession chain, and present-only
   operating statement.
2. `SOURCE_EVIDENCE/TM_PIP_039_SUPERSESSION_EVIDENCE.md` — exact source
   identities, byte counts and SHA-256 values, byte-exact W3 entry, Git
   ancestry/commit evidence, prompt deletion evidence, and preservation
   result.
3. `CHILD_RETURNS/TM_PIP_039_RETURN.md` — this structured return.

## Validation and containment

- Historical D-41 `RUN_BASIS.md` blob at the frozen base:
  `f4d8a44324e8a8bdb6edb74577d05f0d32aac44a`; no working-tree diff.
- Exact W3 entry: 1,102 bytes, SHA-256
  `b92362c99302d77eeb7bbe82c6e52a40357d40717552e92aff3cd94372fdcb3a`.
- Exact owner quote: 188 bytes, SHA-256
  `70123ce6525f7d93cb42793ce97a0b89a4076f6f4f96975b2d880c701ec56757`.
- Receipt-25 rescission, R6/R7 completion, and owner-directed prompt deletion
  commits are ancestors of the frozen source state.
- `init/piping-resume-one-time.md` is absent from the frozen source tree; its
  last pre-deletion blob is retained in Git history.
- Changed-path manifest: exactly the three output paths named above.
- No historical source, register, decision, receipt, lifecycle, deliverable,
  or Git mutation was performed.

## Boundary and residuals

The result records prospective supersession only. It does not change or
reinterpret the historical owner quote or pause act, revive the one-time
prompt, authorize new D-41 work, or close `TM-PIP-039` in the Action Item
register. There is no execution blocker within the sealed treatment scope.

Model attribution: inherited session capability, no override or mid-task
substitution; exact platform model string was not exposed to this child.
