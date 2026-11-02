# Owner decisions: recording hanger and modal task runs

This is the decision packet for the owner. It condenses the reviewed proposal
(`joint-analysis-run-proposal.md` r5, merged at 58fb1783e), and the proposal holds the
detail and the sources. Nothing here is adopted. No format, store migration or exception
is in force until the owner decides.

**What is being decided.** Hanger sizing, and later modal analysis, produce typed result
documents. Analysis record 0.3.0 is the only record format, and it records only one
mechanics solve, so a sizing result cannot be saved as a recorded run today. The three
choices below decide how such runs are recorded.

**The rule in force.** The owner decided on 2026-10-10: "as you recommended,
no-compatibility principle carries". DEL-14-02 CLM-004 records that each format keeps
only its current version, so the 0.1 sorted-compact and 0.2 records are refused by
name. The analysis-run dispatcher puts it this way: "no migration exists. Re-run the
analysis to record a current run."

## Decision 1: which record carries task runs

| Option | What it is | Consequence for stored 0.3.0 records | Consequence when modal is added |
|---|---|---|---|
| **A** | analysis_run 0.4.0 replaces 0.3.0, with a run-kind discriminator | Retired under the rule. Their bytes remain, but they are no longer admitted, so they cannot be reopened or re-saved as supported records unless you make an exception: two current versions, or an archival 0.3.0 reader. All 0.3 producers, readers and the 24 committed fixtures are retired or repinned | A further 0.4.x/0.5.0 version, the same question again |
| **C-closed** | A separate task-record format, with a closed list of families; 0.3.0 unchanged | Unchanged | A successor task-record version, which under the rule retires stored hanger task records |
| **C-layered** | A separate task-record format: a stable envelope plus a separately versioned binding per family; 0.3.0 unchanged | Unchanged | A new modal binding. The envelope and the hanger binding are unchanged, so stored hanger records stay readable. This needs Decision 2 |

The task record never records a mechanics solve. It binds a whole family document,
whereas 0.3.0 binds rows of the six mechanics tables, and its readers are the family's
own. It is not 0.3.0 renamed.

**Recommendation: C-layered,** if Decision 2 permits it. Otherwise C-closed, accepting
that adding modal later retires the hanger task records stored before it.

## Decision 2: may families be added under a stable envelope?

C-layered widens the set of records the format accepts each time a family binding is
added. That is the question for you, not a technicality. It is permitted only under four
rules:
1. **Two layers.** The envelope schema checks structure. The named family binding, at
   its named version, admits the content.
2. **Unknown families.** A family the build does not know is refused as a recognized
   run. It can be listed for inspection by its envelope only.
3. **Bindings only grow.** New bindings, and new versions alongside existing ones, may
   be added. An existing binding's meaning never changes.
4. **Fixed records.** A record's binding version is hashed and never changes, and a
   re-save keeps it byte for byte.

| | Hanger record | Modal record (later) |
|---|---|---|
| Build from before task records | Cannot open the store (Decision 3) | Same |
| Build with the hanger binding only | Read | Refused as unrecognized; envelope inspection only |
| Build with both bindings | Read | Read |
| 0.3.0 records in any build | Unchanged | Unchanged |

**Deferred, and not needed now:** when a family gets a breaking new binding version,
does the earlier version stay readable? That is the same rule at family level. It is
decided at the first breaking change.

**Recommendation: permit,** under the four rules.

## Decision 3: the store change and older builds

Recording task runs needs new store tables, and so a store migration. The desktop
refuses a store newer than itself, all of it: "STORE-SCHEMA-NEWER … Nothing was read or
written; open it with that build."

| Option | Data access by older builds | Integrity | Cost |
|---|---|---|---|
| **(i) One store migration, with an automatic, SQLite/WAL-consistent copy of the store taken before the migration, and recovery from an interrupted migration** | After the first open by a new build, older builds refuse the whole store, every project in it. The copy restores the earlier snapshot only; work done afterwards is not carried back | Full: foreign keys to model states, project cascade, the immutability triggers | The backup and recovery are new implementation requirements |
| (ii) The same migration, without an automatic copy | As (i), with no snapshot unless the user made one | Full | Lowest |
| (iii) Task runs in a separate store file, with the main store version unchanged | Older builds keep opening every project; they do not see task runs | Weaker. Links to model states cannot be enforced across files, so an older build can delete a state that a task run refers to. The new build must detect and report orphaned runs | A second file to back up, move and keep consistent |

The migration runs when a new build first opens the store, not when the first task run is
recorded. The migration number is allocated at implementation; none is reserved.

**Recommendation: (i).**

## Also for decision, after the above (Agent 0 or DEL-00-06, not reserved to the owner)

- **Status word.** A completed task run carries a new family-neutral status such as
  `TASK_COMPLETED`, meaning the algorithm completed and nothing more, beside the mandatory
  `HUMAN_REVIEW_REQUIRED`. The alternative is to reuse 0.3.0's `MECHANICS_SOLVED`. The
  recommendation is the new status, for DEL-00-06's review.
- **Names and versions** of the format and the first hanger binding.
- **Modal's binding.** The dynamics lane's open items (proposal §5) decide it.

## Conditions that hold whatever is decided

- **Privacy (D-10).** Reports today copy every input-manifest setting into the report.
  Before any private library or modal material is placed in a manifest, the report
  projection becomes an exact allow-list. It is identical in the TypeScript, Rust and
  renderer paths, and tested with marker values.
- **Inputs.** Only the run's name, tags, notes and free-text remarks, which nothing
  reads, stay outside the hash. Every assumption that is used is hashed input.
- **Recording.** Only fresh results, under native custody, can be recorded. Hanger
  records only `sized` documents on the pressure-free route. The exact route and
  preload stay held.
- **0.3.0** stays as it is under any C option: no migration and no repinning.

## Review limits

- **Independent review** (gpt-6-astra, medium) covered proposal r3. Its findings were
  applied in r4. The r5 changes, which restated Decision 2 and limited the unhashed
  members, were checked by Agent 0 but not independently re-reviewed.
- **Dynamics** checked r2 as a consumer: no architecture objection, with five
  corrections applied. That was not an independent review.
- **No adopted text.** The governing text for DEL-00-06 and DEL-14-02 (proposal §12) is
  proposed, not reviewed in adopted form. It gets its own review.
- **C1 native custody** is cited at ee7a7f13b, which is PR #59, still open. Its lines
  may change.
- **Modal's** publication is governed intent, not implemented; its inputs were relayed
  answers.
- **Nothing is implemented.** Option (iii)'s orphan handling and option (i)'s backup and
  recovery are unbuilt requirements.
