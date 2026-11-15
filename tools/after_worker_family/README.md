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

The after-worker judge (`apps/desktop/src/test-support/after-worker/afterWorkerReplies.ts`, DEL-04-05) imports some of the web's acceptance predicates and restates the others. `family-table.md` names each predicate, its source and whether the judge imports it, restates it, or runs it inside an imported function. `family-table.digests.json` keeps the SHA-256 of each restated row's cited text. `family_table.py` generates both and checks them against the source. It uses only the standard library.

Every row is found by text, never by line number. A declaration is found by its name. A passage is found by a start text that occurs exactly once in its file, or by a declaration's name, and it ends where the row says. The check also asserts:

- every web name the judge imports has a row;
- an imported row's name is exported and the judge imports it from that file;
- a restated row's name is not imported, the judge names it, and the row's claim about export holds;
- a run-inside row's name is called by the imported function it names.

The check gives traceability. It does not show that the judge equals the web, or that the table is complete: a predicate the judge neither imports nor names in a row is not found.

## When the check fails

```bash
python3 -I tools/after_worker_family/family_table.py .            # the check
python3 -I tools/after_worker_family/family_table.py . --write    # regenerate the table and the digests
```

Run both from the repository root. The test `tests/test_after_worker_family_table.py` runs the check, under the registered check `after-worker-family-table`.

If a row's anchor text or cited text changed:

1. If an anchor no longer occurs exactly once, update that row's anchor in `ROWS` in `family_table.py` to the source's new text.
2. Run the regeneration command. It prints each row whose digest it recorded anew.
3. Read each printed row's cited source and the judge's restatement.
4. Say in the pull request which rows were regenerated and that you read them.

This is a regeneration by the generator. No separate reader has to approve it.

If a row's **meaning** would change, and not only its text, do not just regenerate. Meaning changes when the judge would no longer equal the web there: a restated predicate now decides differently, a predicate the judge imports is no longer exported, or the web gains a predicate of the session's acceptance that the judge does not apply. Report it as a finding to SWBPIPE's exact route performance lane. If that lane has ended, report it to SWBPIPE's Agent 0.

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
