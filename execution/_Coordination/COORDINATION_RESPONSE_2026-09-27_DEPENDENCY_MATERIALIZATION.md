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
on scratch copies of the materializer's inputs, never on the committed files.
Both modes were run, each with `--refresh-pointers`. The script and raw outputs
are in the work graph's [evidence folder](WorkGraphs/PIPING_DEP_MATERIALIZATION_20260927/evidence/).

**With the tool as reviewed in PR #985** (`comparison_before_retired_fix.json`),
default and `--canonical-output` runs both changed the same 93 of 98 registers.
Four kinds of difference accounted for all of them: header (92 files), row
order (68), aggregate `Notes` (480 rows in 49 files), and `RETIRED` rows
dropped. The rewrite skipped DAG-011's 85 `RETIRED` rows and lost the local
copies: 83 `EXTRACTED` rows in 31 files, for example DEL-01-04 `DAG-002-E0392`
and DEL-02-04 `DAG-002-E0395` to `-E0397`. With `--canonical-output` it also
dropped DEL-13-04's two declared `RETIRED` rows (`DEL-13-04-D001`, `-D002`).

That loss broke the rule that rows are retired, not deleted, so the tool was
fixed in the same candidate (see "Root tool change" below). **With the fixed
tool** (`comparison.json`), both modes produce the same output. They change the
same 93 registers, and three kinds of difference remain:

| Kind | Files | Rows | Cause |
|---|---:|---:|---|
| Header gains `EstimateImpactClass`, `ConsumerHint` | 92 | — | The rewrite uses DAG-011's 31-column header. 92 local registers use the 29-column v3.1 core, the form the 2026-06-16 rectification plan measured. Those two columns are blank in DAG-011 except for nine DEL-04-04 rows, whose local register already has both columns and the same values. |
| Row order | 71 | — | The rewrite sorts every row by `DependencyID`. Local registers keep their authoring order. The kept retired rows are now sorted too, which shows three more files out of order. |
| `Notes` gains a `legacy_*` suffix | 49 | 480 | The DAG-007 aggregate build appended `legacy_*` annotations of each row's input field values to `Notes`, in the aggregate only. DAG-008 to DAG-011 carry those rows unchanged. The rewrite takes non-declared, non-retired rows, `Notes` included, from the aggregate. |

Example: DEL-03-02 `SEMREF-2026-06-16-DEL-03-02-A001` gains
`; legacy_anchortype=IMPLEMENTS_NODE; ...` in `Notes`. All 83 local
non-declared `RETIRED` rows and all 183 declared rows are now kept, in both
modes, and no declared row is set aside.

The five unchanged files are DEL-04-07, DEL-07-09, DEL-07-11, DEL-07-12 and
DEL-16-06. They were written from DAG-010 or DAG-011 in the 31-column form.
No rewrite adds or drops a row or changes a core field. The nine
`DeclaredIdCollisions` (DEL-04-04 two, DEL-12-04 seven) differ only in `Notes`.
The local row wins, so they produce no difference.

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
   mode, is not how Piping maintains existing registers. Even with the fixed
   tool it would replace local `Notes` with aggregate annotations on 480 rows,
   and it would rewrite 93 files whose rows and arcs already match DAG-011.
2. **Canonical form.** A committed local `Dependencies.csv` is canonical as it
   stands. It is v3.1, `RETIRED` rows are kept, and its own `Notes` and row
   order stand. When an accepted DAG change must reach a local register, make
   exact field-level edits under the change's own authority, as in R5, or use
   the `dependency-extract` refresh. Then build or check the DAG from the
   registers.
3. **Where the materializer applies.** Use it per deliverable, with
   `--deliverable-id`, for a register that was written from the DAG. Examples
   are DEL-07-09 under DAG-010, and D-78 row D, whose `--refresh-pointers` run
   left `Dependencies.csv` byte-identical. Both modes of the fixed tool
   reproduce those registers exactly. Refreshing `_DEPENDENCIES.md` pointers in other
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

## Root tool change

The integrating session, under the same owner direction, had the materializer
fixed in this candidate. A whole-file rewrite now keeps rows it used to drop,
in both modes:

- Every local `RETIRED` row is kept with its field values unchanged, unless the
  output already carries its `DependencyID` from the aggregate or a kept
  declared row. The aggregate's own `RETIRED` rows are still not materialized,
  so each retired row is written once.
- With `--canonical-output`, declared rows are now kept when `ACTIVE` or
  `RETIRED`, the canonical v3.1 statuses. Other declared rows, such as
  `CANDIDATE`, are still set aside.

Kept rows are sorted by `DependencyID` with the rest, as the tool already sorts
every row. Aggregate row content, header selection, the order of aggregate rows
and the `Notes` handling are unchanged. One exception: a local column that only
a kept retired row carries now stays in the output header, as it already did
for kept declared rows.

The docstring and `tools/REGISTRY.md` row also say a rewrite is not a currency
check, list the remaining effects (header, row order, aggregate `Notes`), and
point to the `accepted_dag` comparison. Tranche manifest:
`docs/governance_harness/tranche_manifests/PIPING-DEP-MATERIALIZATION-NOTE-20260927.yaml`.

## Boundary

No `Dependencies.csv`, `_DEPENDENCIES.md`, `_STATUS.md`, DAG version, pointer,
decomposition, satisfaction or lifecycle state changes. No deliverable is
affected, so no `MEMORY.md` entry is added. No Task Management intake
qualifies: the concern is resolved here. Standard claim fence applies (F-PIP-2;
claims taxonomy per DEC-081).
