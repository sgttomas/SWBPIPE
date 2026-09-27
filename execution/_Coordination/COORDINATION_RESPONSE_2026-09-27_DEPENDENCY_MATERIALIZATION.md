---
doc_id: COORDINATION-RESPONSE-2026-09-27-DEPENDENCY-MATERIALIZATION
doc_kind: coordination.response
status: loop_disposition_not_owner_ruling
created: 2026-09-27
responds_to: execution/_Coordination/NOTICE_2026-09-26_DEPENDENCY_FOLLOWUPS.md (item 3; "rerunning any tool ... are this loop's decisions")
repository_basis: 0adfbc7476df33521883ce1573781237cd24d384
work_graph: execution/_Coordination/WorkGraphs/PIPING_DEP_MATERIALIZATION_20260927/WORK_GRAPH.md
---

# Piping response — materializer reruns against DAG-011

## Finding and owner direction

The review of Root PR #985 ran `tools/coordination/materialize_local_dependencies.py`
over Piping in default mode from the accepted DAG. It found that 93 of the 98
`Dependencies.csv` files it writes would differ from the committed files.
Piping's decision on that difference was open.

The owner directed on 2026-09-27, in the Claude Code conversation
(CHAT_TRANSCRIPTION — EVIDENCE, NOT RULING): "You can take care of those
remaining items now.  Include the items with the "other owners".  You can
make changes as necessary."

This record is the loop's disposition under that direction. It is not an owner
ruling, and it changes no dependency register, DAG version, pointer or scope.

## What a rerun changes, and why

The accepted DAG is `DAG-011` (`execution/_DAG/_LATEST.md`). The comparison ran
on scratch copies of the execution tree, never on the committed files. Both
modes were run, each with `--refresh-pointers`. The script and raw output are in
the work graph's [evidence folder](WorkGraphs/PIPING_DEP_MATERIALIZATION_20260927/evidence/).

Default and `--canonical-output` runs both write 98 registers and change the
same 93. Four kinds of difference account for all of them:

| Kind | Files | Rows | Cause |
|---|---:|---:|---|
| Header gains `EstimateImpactClass`, `ConsumerHint` | 92 | — | The rewrite uses DAG-011's 31-column header. 92 local registers use the 29-column v3.1 core, the form the 2026-06-16 rectification plan measured. Those two columns are blank in DAG-011 except for nine DEL-04-04 rows, whose local register already has both columns and the same values. |
| Row order | 68 | — | The rewrite sorts by `DependencyID`. Local registers keep their authoring order. |
| `Notes` gains a `legacy_*` suffix | 49 | 480 | The DAG-007 aggregate build appended `legacy_*` annotations of each row's input field values to `Notes`, in the aggregate only. DAG-008 to DAG-011 carry those rows unchanged. The rewrite takes non-declared rows, `Notes` included, from the aggregate. |
| `RETIRED` rows dropped | 31 | 83 (`EXTRACTED`) | The rewrite writes only `ACTIVE` and `CANDIDATE` aggregate rows and skips DAG-011's 85 `RETIRED` rows. Local non-declared copies of those rows are lost. With `--canonical-output`, DEL-13-04's two declared `RETIRED` rows (`DEL-13-04-D001`, `-D002`) are also dropped. |

Examples: DEL-03-02 `SEMREF-2026-06-16-DEL-03-02-A001` gains
`; legacy_anchortype=IMPLEMENTS_NODE; ...` in `Notes`. DEL-01-04 loses
`DAG-002-E0392` (`EXTRACTED`, `RETIRED`). DEL-02-04 loses `DAG-002-E0395`
to `-E0397`.

The five unchanged files are DEL-04-07, DEL-07-09, DEL-07-11, DEL-07-12 and
DEL-16-06. They were written from DAG-010 or DAG-011 in the 31-column form.
No rewrite adds a row or changes a core field. The nine `DeclaredIdCollisions`
(DEL-04-04 two, DEL-12-04 seven) differ only in `Notes`. The local row wins, so
they produce no difference.

## The local registers are current

- **Rows.** The 98 local registers hold 1,571 rows, and so does DAG-011. The
  `DependencyID` sets are equal. Every shared row has identical core fields.
  1,036 rows have identical `Notes`. In the other 535, DAG-011's `Notes` extend
  the local `Notes`; 18 of those carry DAG-007's own
  `aggregate_only_local_register_unchanged=true` marker.
- **Arcs.** `tools/coordination/analyze_dep_closure.py projects/chirality-piping/execution`
  (exit 0) reports `accepted_dag.result = NO_DEPARTURE_FOUND` against DAG-011.
  No deliverable is `DAG pending`.

The difference is therefore not stale registers and not an error in the
columns. Piping records no non-default materializer mode, because it never
regenerates its registers from the DAG. The information flows the other way:

- DAG-007 was "built from refreshed deliverable-local dependency registers"
  (`execution/_DAG/DAG-007/APPROVAL_RECORD.md`), and later versions extend it.
- The rectification rule is that rows are retired, not deleted, and that
  legacy labels are kept as provenance
  (`plans/PLAN_2026-06-16_dependency_type_system_rectification.md`, Phase 3).
- D-59 accepted DAG-008 with the approved aggregate-only duplicate retirements preserved.
- When an accepted DAG changed a local value, Piping edited exactly that field
  and kept each file's formatting. The R5 precedent is
  `execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/R5/tools/sync_dependency_mirrors.py`
  ("exact accepted DAG-010 Status repair, not a graph rebuild"), with its
  `DEPENDENCY_MIRROR_REPAIRS.json`.
- Under D-GOV-49 and D-78, the local files are the dependency evidence.

## Disposition

1. **No register is rewritten.** A whole-tree materializer rerun, in either
   mode, is not how Piping maintains existing registers. It would drop 83
   retired evidence rows and replace local `Notes` with aggregate annotations.
   It would also rewrite 93 files whose rows and arcs already match DAG-011.
2. **Canonical form.** A committed local `Dependencies.csv` is canonical as it
   stands. It is v3.1, `RETIRED` rows are kept, and its own `Notes` and row
   order stand. When an accepted DAG change must reach a local register, make
   exact field-level edits under the change's own authority, as in R5, or use
   the `dependency-extract` refresh. Then build or check the DAG from the
   registers.
3. **Where the materializer applies.** Use it per deliverable, with
   `--deliverable-id`, for a register that was written from the DAG. Examples
   are DEL-07-09 under DAG-010, and D-78 row D, whose `--refresh-pointers` run
   left `Dependencies.csv` byte-identical. Default mode reproduces those
   registers exactly. Refreshing `_DEPENDENCIES.md` pointers in other
   deliverables is a separate act. Before a refresh, check that the
   `Dependencies.csv` rewrite is byte-identical or intended.
4. **How to check currency.** Compare arcs with the accepted DAG
   (`analyze_dep_closure.py`, `accepted_dag`), or compare rows directly as the
   evidence script does. A default-mode materializer difference is not a
   finding of drift.

Recording each deliverable's dependency mode and declarations, and any
currency audit, stay separate acts, as D-78 states. A later owner decision may
change this disposition. Reversal means deleting this record and its work-graph
pointers; no register needs restoring.

## Root tool documentation

To stop a future default-mode comparison from being read as drift, the same
candidate adds a short note to the materializer's docstring and its
`tools/REGISTRY.md` row. The note says a rewrite is not a currency check, lists
the four effects, and points to the `accepted_dag` comparison. The tool's
behaviour does not change. Tranche manifest:
`docs/governance_harness/tranche_manifests/PIPING-DEP-MATERIALIZATION-NOTE-20260927.yaml`.

## Boundary

No `Dependencies.csv`, `_DEPENDENCIES.md`, `_STATUS.md`, DAG version, pointer,
decomposition, satisfaction or lifecycle state changes. No deliverable is
affected, so no `MEMORY.md` entry is added. No Task Management intake
qualifies: the concern is resolved here. Standard claim fence applies (F-PIP-2;
claims taxonomy per DEC-081).
