# Task Management Adoption Packet — Piping loop (Stage B, presented 2026-07-31)

Status: `RULED — OPTION A ADOPTED AND EXECUTED; REGISTER/NOTICE CURRENT 2026-08-01`
Prepared by: Root loop, HELP_HUMAN (Agent 0), per Stage-A workplan step 7.
This packet is coordination, never authority. Nothing below has effect
until the Piping loop's owner rules under its own instruments (F-PIP-5).
Decline and defer are lawful outcomes (K-TM-4); silence binds nothing.

## Basis

D-GOV-32 (adoption of the Task Management PRD Rev 2, subject SHA-256
`97e2ae65…f35d18`); K-TM-1..6 at `docs/CONTRACT.md` §1.14; the root
Stage-A precedent (Receipts 66–70): register + seed + first triage with
46/101 first-session closures, TASK_MANAGEMENT agent in lieu of entry
binding (root's own §14 ruling), resolution orchestration per workplan
Amendment 2.

## Proposed adoption (effective only on ruling of D-63)

1. **Register mint.** `projects/chirality-piping/execution/_Coordination/_TaskManagement/REGISTER.csv`, schema 1.0 (25 columns,
   `RegisterSchemaVersion` column 1, mandatory `Status`/`Disposition`,
   K-TM-6), IDs `TM-PIP-<seq>`. A coordination-surface subdirectory,
   not a SPEC §1.2 tool root. Owned and written only by this loop
   (K-TM-2/K-TM-3).
2. **Binding — Option A (recommended; the root pattern).** No entry
   binding: the loop invokes TASK_MANAGEMENT (`agents/AGENT_TASK_MANAGEMENT.md`)
   on demand or on an owner-scheduled routine; sessions record residue
   in receipts as today. `projects/chirality-piping/loop/LOOP_INIT.md` is not amended.
3. **Binding — Option B (PRD §14 pattern, if preferred).** Amend
   `projects/chirality-piping/loop/LOOP_INIT.md` so session entry reads open register rows and session close
   records dispositions or receipt residue. Exact text supplied on
   request; one loop-owned file, one loop ruling.
4. **Decision row.** D-63 in this loop's `_DECISIONS/_REGISTER.md`
   recording the ruling (adopt A / adopt B / amend / decline / defer),
   with this packet as subject.
5. **Migration.** 22 root rows are deferred against this loop's tranches/adoption (TM-ROOT-037 successor mechanism; TM-ROOT-077-097, the 21 TP-EXPORT-006 TBDs), and TM-ROOT-053 (D-45, OPEN at priority HIGH) migrates as a linked row while the owner ruling remains outstanding. Migration is by linked rows (PRD §6.2): the root
   row closes or cross-cites; this loop's row cites `SourceRef` back.
   No actor ever writes across registers.

## Fences carried

K-TM-1..6 in full; no work-discovery queue; no resolution surface;
rows carry no directives; no agent as accountable (A); registers never
gate; scanner projections gitignored (D-GOV-01). Adoption creates no
duty for any other loop.

## Root-side effects on this loop's ruling

Root register row TM-ROOT-099 closes `RESOLVED_BY_DECISION` citing D-63; the
deferred rows whose triggers reference this adoption wake for migration
review at the next root TASK_MANAGEMENT invocation.

## Owner ruling (2026-08-01)

The owner ruled verbatim: `1) D-63 ruling: Option A.`

The durable ruling record is
[`_DECISIONS/D-63_RULING_2026-08-01.md`](_DECISIONS/D-63_RULING_2026-08-01.md).
Execution evidence is the current
[`_TaskManagement/REGISTER.csv`](_TaskManagement/REGISTER.csv) and ordinary
[`NOTICE_2026-08-01_D63_TASK_MANAGEMENT_ADOPTION_AND_D45_CLOSURE.md`](NOTICE_2026-08-01_D63_TASK_MANAGEMENT_ADOPTION_AND_D45_CLOSURE.md).
Option A authorizes the register mint, linked migration, and ordinary
root-loop notice exactly as proposed. Option B is not selected:
`projects/chirality-piping/loop/LOOP_INIT.md` remains unchanged. This appended
status and pointer preserve the proposal history above and create no product,
decomposition, lifecycle, stage, release, or root-register effect.
