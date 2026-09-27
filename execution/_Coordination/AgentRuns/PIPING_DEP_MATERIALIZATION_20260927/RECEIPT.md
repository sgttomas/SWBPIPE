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

Deleting retired rows broke the rule that rows are retired, never deleted
(`docs/SPEC.md` §6.6), so the Root materializer was fixed in the same
candidate. For each local non-declared row whose ID the output does not already
carry, a rewrite now keeps it if `RETIRED` and writes the aggregate's `RETIRED`
row if the aggregate retired that ID. Otherwise the aggregate still replaces it,
and its ID is listed as `DroppedLocalRows`. With `--canonical-output`, declared
`RETIRED` rows are also kept. `Status` is compared case-insensitively. Kept rows
are sorted by `DependencyID` with the rest. Aggregate row content and `Notes`
handling are unchanged. There are three exceptions to "nothing else changes":
- a column carried only by a kept `RETIRED` row stays in the header;
- `--refresh-pointers` row counts include the kept rows;
- a local row the aggregate retired is written as that `RETIRED` row.

Over Piping, both modes now keep all 83 retired and 183 declared rows. No
aggregate `RETIRED` row replaces a local one and no row is dropped. Both modes
give byte-identical output. The remaining differences are header (92 files), row
order (71) and aggregate `Notes` (480 rows in 49 files). The docstring and
`tools/REGISTRY.md` row describe the fix and say that a rewrite is not a
currency check (tranche `PIPING-DEP-MATERIALIZATION-NOTE-20260927`; the export
manifest hashes were regenerated). The four 2026-09-26 dependency follow-up
notices (Piping, PEC, Runtime, App) each carry a dated update routed by that
tranche.

## Checks and limits

The candidate was examined at `0adfbc747` and rebased onto `8bbd022b9`, which changes only App v4 files. These checks ran on the final candidate, with diff-based checks against `8bbd022b9`:

- Schema validator: 98/98 local registers PASS (none changed).
- G0–G3: PASS, exit 0.
- G4 (`--added-manifests-only`): PASS, exit 0.
- Conflict markers and run-record leaks: PASS, exit 0.
- `build_workflow_index.py --check`, `git diff --check` and the Piping receipt validator: exit 0.
- `run_affected_tests.py` (coordination, practitioner harness, validation): 1,208 passed, exit 0.
- Harness self-check: exit 0; it raises no finding on the new paths.
- Materializer tests: the first three new tests fail on the tool at `05c0b6e94` (3 failed, 20 passed). The three review-repair tests fail on the tool at `f32807f11` (3 failed, 23 passed). All 26 pass on the final tool.
- Evidence: `compare_materializer.py` output is byte-identical across two runs for each tool revision.

No SCC case exists in Piping, so the SCC case validator does not apply. The
`--refresh-pointers` runs also rewrite `_DEPENDENCIES.md`. That is a separate
act and is not assessed here. Independent review, CI and merge belong to the
integrating session. No release is claimed.

Follow-up, not in this candidate: the generated `_DEPENDENCIES.md` register
section still says the register holds "aggregate rows plus its local
`Origin=DECLARED` rows" and that declared rows are kept on a rewrite; it does
not mention kept `RETIRED` rows. It is left unchanged because the App scaffold
copies it byte for byte, so changing it needs a coordinated App change.

## Cursor and pointers

- **Receipt-ID:** `PIPING_DEP_MATERIALIZATION_20260927`.
- **Examined-Through:** `0adfbc7476df33521883ce1573781237cd24d384`.
- **Parent-Receipt:** none; historical `loop/LOOP_RECEIPTS.md` unchanged.
- **Owner-Direction:** CHAT_TRANSCRIPTION — EVIDENCE, NOT RULING, 2026-09-27:
  `You can take care of those remaining items now.  Include the items with the "other owners".  You can make changes as necessary.`
- **Pointers:** PR: [#1015](https://github.com/sgttomas/chirality/pull/1015)
  (branch `worktree-agent-a522f6a3bf5db58d9`);
  [work graph](../../WorkGraphs/PIPING_DEP_MATERIALIZATION_20260927/WORK_GRAPH.md)
  and its [evidence](../../WorkGraphs/PIPING_DEP_MATERIALIZATION_20260927/evidence/).
  No deliverable `MEMORY.md` is affected.
- **Model-Attribution:** a Claude Code TASK subagent of the integrating parent
  session prepared the candidate and committed it locally; it did not push.
- **Gate-Outcome:** candidate ready for independent review and integration.
  The undertaking completes when the integrating session merges it after
  required CI.
