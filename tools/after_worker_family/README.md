---
name: swbpipe-after-worker-family
description: Keep the after-worker judge's family table describing the web sources it cites, while leaving those sources free to change.
metadata:
  kind: instruction
  surface: tool
---
# After-worker family table

**What the check prevents:** the table silently ceasing to describe the sources it cites.

**What it leaves available:** ordinary edits to those sources by other lanes. A line shift passes, as does any edit that leaves each cited passage, anchor and claim as it was. An edit to a restated row's cited passage fails only until the table is regenerated and the changed row read.

The after-worker judge (`apps/desktop/src/test-support/after-worker/afterWorkerReplies.ts`, DEL-04-05) imports some of the web's acceptance predicates and restates the others. `family-table.md` names each predicate, its source and whether the judge imports it, restates it, or runs it inside an imported function. `family-table.digests.json` keeps the SHA-256 of each restated row's cited text. A restated row's cited text also covers each source whose ordinary edit could make the row's claim false while the row's own passage stays unchanged: a project helper or constant that decides the behaviour the claim states, followed as far down as that behaviour is decided. The table lists each such source after "with". Unrelated behaviour, ordinary library operations, and an implementation the judge imports and so shares with the web are not covered. `family_table.py` generates both and checks them against the source. It uses only the standard library.

Every row is found by text, never by line number. A declaration is found by its name. A passage is found by a start text that occurs exactly once in its file, or by a declaration's name, and it ends where the row says. The check also asserts:

- every web name the judge imports has a row;
- an imported row's name is exported and the judge imports it from that file;
- a restated row's name is not imported, the judge names it, and the row's claim about export holds. An inline restated row makes the same three assertions for each web declaration it names: "readNativeSummary's text refusal" names `readNativeSummary`, and "readNativeVerdict and readNativeSummary (their cache keys)" names both. The inline rows that name no declaration have only their anchors and their digest: "the session gates 1 and 2", "the session's manifest arguments", "the standing-model text check", "the model-reference check", "the solver mode", "the panel's first page and the empty-page re-read" and "the semantics' recorded-table agreement";
- a run-inside row's name is called by the imported function it names.

The check traces the committed table to the sources it cites, and prompts re-examination when they change. It does not prove that the judge is semantically equivalent to the web, and it does not discover every predicate the table omits.

## When the check fails

```bash
python3 -I tools/after_worker_family/family_table.py .            # the check
python3 -I tools/after_worker_family/family_table.py . --write    # regenerate the table and the digests
```

Run both from the repository root. The test `tests/test_after_worker_family_table.py` runs the check, under the registered check `after-worker-family-table`. That check is selected by the files the table cites. The hosted Python job derives the test's reads from its code, so it selects the test for any change under `apps/desktop/src/`.

If a row's anchor text or cited text changed:

1. If an anchor no longer occurs exactly once, update that row's anchor in `ROWS` in `family_table.py` to the source's new text.
2. Run the regeneration command. It prints each row whose digest it recorded anew.
3. Read each printed row's cited source and the judge's restatement.
4. Say in the pull request which rows were regenerated and that you read them.

This is a regeneration by the generator. No separate reader has to approve it.

If a row's **meaning** would change, and not only its text, do not just regenerate. Meaning changes when the judge would no longer equal the web there: a restated predicate now decides differently, a predicate the judge imports is no longer exported, or the web gains a predicate of the session's acceptance that the judge does not apply. So does a row whose claim an edit to an uncovered source could make false: cover that source (its own row, or the row's cited text widened) or narrow the claim to what holds. Report it as a finding to SWBPIPE's exact route performance lane. If that lane has ended, report it to SWBPIPE's Agent 0.

## Replaying the judge over a reply set

The reply bytes a harness run kept are held outside Git. The check above does not need them. A holder of a reply set can replay the judge over it:

1. Check out `d7769cd9b` in a clean clone. Round 5's acceptance was made at that commit because its decoder trees equal the timed heads'. The judge refuses replies from other decoders by design (`checkRevision`), so each reply manifest's head must be in the clone's history.
2. Install the dependencies (`npm ci` at the repository root). Then, in `apps/desktop`, build the wasm engine the judge's manifest builder loads:

   ```bash
   npm run build:wasm
   ```

3. In `apps/desktop`, run the judge under both configurations, naming the reply set as the argument:

   ```bash
   SWBPIPE_AFTER_WORKER_REPLIES=<path to a reply set> npx vitest run --config src/test-support/after-worker/vitest.config.ts
   SWBPIPE_AFTER_WORKER_REPLIES=<path to a reply set> npx vitest run src/test-support/after-worker/afterWorkerReplies.test.ts --testTimeout=600000 --maxWorkers=4
   ```

   The first command uses the after-worker config. The second uses the desktop default config. With the variable set, every run subdirectory of the reply set is judged and nothing is skipped.
