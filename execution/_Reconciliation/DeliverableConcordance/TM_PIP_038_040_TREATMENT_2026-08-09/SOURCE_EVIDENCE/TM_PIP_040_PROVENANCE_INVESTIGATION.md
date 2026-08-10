# TM-PIP-040 committed-provenance investigation

## Identity and boundary

- Item: `TM-PIP-040`.
- Investigator: bounded ephemeral Agent 2 under managed `RECONCILIATION`.
- Parent: `/root/reconciliation_tm038_040_treatment`.
- Frozen treatment source state: `da40d7dc4192c9aa2f49e9438729179aae281b61`.
- Method: read-only inspection of committed provenance, Git history and current
  worktree metadata, followed by a bounded check for already-present local
  sources. No artifact was deleted, restored, copied, regenerated, or fetched;
  no Git state was changed.
- Terminal limit: this investigation does not select or perform any
  `RESTORED`, `LOST`, or `UNDETERMINED` disposal act.

## Reliability labels

| Label | Meaning |
|---|---|
| `C1_COMMITTED_CONTEMPORANEOUS` | Committed record made while the frozen worktree and its ignored paths were still observable; strongest available evidence of historical state. |
| `C2_COMMITTED_SUBSEQUENT` | Later committed coordination or register record; reliable for what was recorded later, but not direct observation of the earlier deletion/restoration mechanism. |
| `G1_GIT_OBJECT_OR_METADATA` | Deterministic Git object/tree/current-worktree metadata. Strong for tracked objects and present registration, incapable by itself of preserving ignored untracked bytes or proving why a removed worktree disappeared. |
| `F1_CURRENT_FILESYSTEM` | Read-only observation on 2026-08-09. Strong for present local state only; not identity evidence for 2026-07-12 objects unless a preserved hash or custody chain joins them. |
| `I1_INFERENCE` | Explicit inference from labeled evidence, never a direct observation or owner act. |

## Exact subject population

The contemporaneous `RUN_SUMMARY.md` enumerates the six sets as:

1. `projects/chirality-piping/.pytest_cache/`
2. `projects/chirality-piping/core/reporting/report_generator/Cargo.lock`
3. `projects/chirality-piping/core/reporting/result_export/Cargo.lock`
4. `projects/chirality-piping/core/reporting/state_comparison_handoff_sections/__pycache__/`
5. `projects/chirality-piping/tests/__pycache__/`
6. `projects/chirality-piping/validation/benchmarks/nonlinear/target/`

All six match committed ignore rules at the frozen/current source lineage:
project `.gitignore` ignores `.pytest_cache/` and `__pycache__/`; the two
crate-local `.gitignore` files ignore `Cargo.lock`; and the nonlinear
benchmark `.gitignore` ignores `target/`. `git ls-tree -r` at frozen source
commit `551f84ef6be656f1603ce0acfa5e3935aa9683c7` returns no entry for any of
the six. [`G1_GIT_OBJECT_OR_METADATA`]

## Committed chronology

### Creation and incident disclosure

The immutable historical run basis identifies the frozen tree as detached
`main` at `551f84ef6be656f1603ce0acfa5e3935aa9683c7`, materialized at
`.claude-worktrees/piping-frozen-551f84ef6`. Its Addendum-9 incident records:

- two lockfiles written by W3 Cargo re-executions;
- the project pytest cache written by the DEL-08-05 re-execution;
- two Python bytecode-cache directories written by the DEL-08-06
  re-execution; and
- the nonlinear benchmark target as pre-existing W2-era contamination.

It also records that the orchestrator's scoped deletion attempt was blocked,
was not worked around, and that the six sets remained carried as known
contamination. The test results and ledger encodings were explicitly not
invalidated. Source: historical `RUN_BASIS.md`, Git blob
`f4d8a44324e8a8bdb6edb74577d05f0d32aac44a`, SHA-256
`c5655c4c43ada8edb3b3cc71a1dbf15ffded92d64d910f1c741e572632d25e61`.
[`C1_COMMITTED_CONTEMPORANEOUS`]

### Last positive observation

The final R6 `RUN_SUMMARY.md` records frozen HEAD at `551f84ef6...`, no
tracked frozen-tree change, exactly the six enumerated ignored paths, and no
seventh path. The later R5 final-closeout entry, committed as
`9b8b6c91966fdd4aab1988ef422176239cc11651` on 2026-07-12, again records
`frozen set 6`. Its `R5_RUN_SUMMARY.md` is Git blob
`f674a9cfc980fc5fd1813b17ac30ca823590906d`, SHA-256
`5141cfd625c5b095030fe7b05643b346f5bbec5af2a81b027c0b12fa1200eadf`.
This is the latest located committed positive observation of the set count.
[`C1_COMMITTED_CONTEMPORANEOUS`]

### Pending owner act remained explicit

The later-consumed resume carrier still described frozen-tree restoration as
a pending owner act and instructed subsequent work to allow-list the six until
restored. Its last committed pre-deletion content is Git blob
`aaf78fa9f22b8ca7e1fccd209133a7e0485f3437`, SHA-256
`de90e65464edf0ddc028f94ebef0ba185bc2ac75bdc0acdeefc7083c7a31f5dd`.
The file was removed from the accepted lineage by commit
`6bb723fc373db30206d36505fa4194d97af756ff` on 2026-08-03 as a consumed
one-time prompt. That commit deletes the carrier file; it contains no
evidence that the frozen worktree or any of its ignored sets was cleaned,
recreated, retained, or copied. [`C2_COMMITTED_SUBSEQUENT`]

### First durable absence question

The 2026-08-03 inbound carrier states that the six-set worktree no longer
exists and that no record says whether the sets were restored or lost. Source:
Git blob `ac5086f3e02a72bd598fe6ba46eec03e65176f49`, SHA-256
`8fa5303c7387dc545c5360443f640ba2c7b6ae646c37d40719b140e880204125`.
The Piping owner subsequently promoted that unresolved question as
`TM-PIP-040`; the source-state register row remains `OPEN`. These records are
reliable for the durable unresolved state, but neither is an eyewitness record
of the physical mechanism. [`C2_COMMITTED_SUBSEQUENT`]

## Git and current-filesystem observations on 2026-08-09

1. `git worktree list --porcelain` has no registration for
   `.claude-worktrees/piping-frozen-551f84ef6`, and the common Git directory's
   current `worktrees/` metadata contains no entry naming that worktree.
   [`G1_GIT_OBJECT_OR_METADATA`]
2. The recorded relative path under the present primary repository root,
   `.claude-worktrees/piping-frozen-551f84ef6`, is absent, as is the parent
   `.claude-worktrees/` directory. A bounded Spotlight
   filename query under the current user home returned no matching directory.
   [`F1_CURRENT_FILESYSTEM`; Spotlight completeness is not assumed]
3. The clean treatment worktree and inspected clean Codex worktrees contain
   none of the six exact relative paths. [`F1_CURRENT_FILESYSTEM`]
4. The primary checkout contains present objects in four corresponding path
   classes: `.pytest_cache/`, both `__pycache__/` directories, and the
   nonlinear `target/`. Their filesystem birth times are all 2026-07-31, more
   than two weeks after the 2026-07-12 last positive frozen-tree observation.
   The two `Cargo.lock` paths are absent there. These are later filesystem
   objects in a different checkout, not a custody-linked copy of the frozen
   sets. [`F1_CURRENT_FILESYSTEM`]
5. The frozen commit and all tracked source bytes remain available in Git, so
   a clean tree at the pinned source revision is logically reproducible. Git
   does not contain the six ignored untracked sets. Recreating a clean tree or
   regenerating caches/build output would establish a new state, not the
   historical fate or byte identity of the former ignored objects.
   [`G1_GIT_OBJECT_OR_METADATA`; `I1_INFERENCE` for the identity limit]

## Evidence gaps

No located committed or current allowed source supplies any of the following:

- an owner restoration/loss act naming these six sets;
- output from a scoped clean, removal, or worktree-recreation operation;
- a worktree-removal record that states whether ignored files were first
  deleted, retained elsewhere, or discarded with the directory;
- original file inventories, byte sizes, content hashes, timestamps, archive,
  or custody-linked copy for the six sets; or
- a present source whose identity can be joined to the original sets.

Ignored untracked files are outside Git's object graph. Consequently, object
history and `git fsck` cannot fill these gaps. Present lookalikes can show that
similar generated outputs exist or are reproducible, but cannot show what
happened to the original filesystem objects. [`G1_GIT_OBJECT_OR_METADATA`;
`I1_INFERENCE`]

## Evidentiary conclusion, not a disposal act

The available record establishes a bounded chronology: the six sets existed
in the frozen worktree through the last committed positive check; restoration
was still described as pending; later durable records found the worktree
absent and the outcome undocumented. The allowed evidence does not establish
which intervening physical action occurred. Absence is therefore not converted
into proof of either restoration or loss, and no outcome label is selected in
this investigation.

The committed D-41 discovery and reconciliation evidence remains available.
Its own incident record says the test results and ledger encodings were not
invalidated; this investigation found no contrary evidence and makes no new
validation, lifecycle, release, reliance, or corpus-closure claim.

## Rerun trigger

Rerun this bounded investigation if an owner supplies a custody-linked copy,
backup/snapshot evidence, a contemporaneous cleanup/recreation record, or a
verbatim personal act statement identifying what was done to the six sets.
Any such source must be captured with origin, date, path, and hashes where
applicable before reconsidering outcome support.
