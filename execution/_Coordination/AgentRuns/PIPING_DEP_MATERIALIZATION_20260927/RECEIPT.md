# Loop receipt — PIPING_DEP_MATERIALIZATION_20260927

> Derivative run account, not scope, decision, lifecycle or release authority.
> The work graph, coordination record, evidence and Git/PR state govern if they
> disagree with this receipt.

## Result

The PR #985 review ran the materializer over Piping from DAG-011 in default
mode, and 93 of 98 `Dependencies.csv` files differed. The comparison was
reproduced on scratch copies in both modes. The differences had four causes:
the aggregate header (92 files), row order (68), aggregate `legacy_*` `Notes`
suffixes (480 rows in 49 files), and `RETIRED` rows left out (83 in 31 files,
plus DEL-13-04's 2 declared rows with `--canonical-output`). The registers are
not stale. Their 1,571 rows equal DAG-011's in every core field, and
`accepted_dag` is `NO_DEPARTURE_FOUND`.

Piping builds its DAG from the local registers, keeps retired rows, and applies
accepted changes as exact field edits. No register is therefore rewritten. The
[coordination record](../../COORDINATION_RESPONSE_2026-09-27_DEPENDENCY_MATERIALIZATION.md)
states the canonical form and where the materializer applies.

Deleting retired rows broke the rule that rows are retired, never deleted, so
the Root materializer was fixed in the same candidate. A whole-file rewrite now
keeps every local `RETIRED` row whose ID the output does not already carry.
With `--canonical-output`, it also keeps declared `RETIRED` rows. Kept rows are
sorted by `DependencyID` with the rest; nothing else in the output changes. Over
Piping, both modes now keep all 83 retired and 183 declared rows and give
byte-identical output. The remaining differences are header (92 files), row
order (71) and aggregate `Notes` (480 rows in 49 files). The docstring and
`tools/REGISTRY.md` row describe the fix and say that a rewrite is not a
currency check (tranche `PIPING-DEP-MATERIALIZATION-NOTE-20260927`; the export
manifest hashes were regenerated).

## Checks and limits

The candidate was examined at `0adfbc747` and rebased onto `8bbd022b9`, which changes only App v4 files. These checks ran on the final candidate, with diff-based checks against `8bbd022b9`:

- Schema validator: 98/98 local registers PASS (none changed).
- G0–G3: PASS, exit 0.
- G4 (`--added-manifests-only`): PASS, exit 0.
- Conflict markers and run-record leaks: PASS, exit 0.
- `build_workflow_index.py --check`, `git diff --check` and the Piping receipt validator: exit 0.
- `run_affected_tests.py` (coordination, practitioner harness, validation): 1,208 passed, exit 0.
- Harness self-check: exit 0; it raises no finding on the new paths.
- Materializer tests: the three new tests fail against the tool before the fix (3 failed, 20 passed) and pass with it (23 passed).

No SCC case exists in Piping, so the SCC case validator does not apply. The
`--refresh-pointers` runs also rewrite `_DEPENDENCIES.md`. That is a separate
act and is not assessed here. Independent review, CI and merge belong to the
integrating session. No release is claimed.

## Cursor and pointers

- **Receipt-ID:** `PIPING_DEP_MATERIALIZATION_20260927`.
- **Examined-Through:** `0adfbc7476df33521883ce1573781237cd24d384`.
- **Parent-Receipt:** none; historical `loop/LOOP_RECEIPTS.md` unchanged.
- **Owner-Direction:** CHAT_TRANSCRIPTION — EVIDENCE, NOT RULING, 2026-09-27:
  `You can take care of those remaining items now.  Include the items with the "other owners".  You can make changes as necessary.`
- **Pointers:** [work graph](../../WorkGraphs/PIPING_DEP_MATERIALIZATION_20260927/WORK_GRAPH.md)
  and its [evidence](../../WorkGraphs/PIPING_DEP_MATERIALIZATION_20260927/evidence/).
  No deliverable `MEMORY.md` is affected.
- **Model-Attribution:** a Claude Code TASK subagent of the integrating parent
  session prepared the candidate and committed it locally; it did not push.
- **Gate-Outcome:** candidate ready for independent review and integration.
  The undertaking completes when the integrating session merges it after
  required CI.
