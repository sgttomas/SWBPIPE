# Commands and runner interface

From instrument projects/chirality-piping, E=execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260915-PRODUCTION-UI/instances/ROOT/D70_BASELINE/instances/MANAGER/CONTINUATION/ONE_SUCCESS.

Focused final: D70_WRITER_ORIGINAL_RETURN set to canonical integration P/instances/RUNNER/RETURN.json; `node node_modules/@playwright/test/cli.js test --config "$E/final-node.config.ts" --grep 'owner one-success|continuation launcher consumes|real driver execution gate'` →8PASS0SKIP. Initial6 PASS retained separately. `node node_modules/typescript/bin/tsc --project "$E/tsconfig.json"` PASS. `node "$E/audit-history.mjs"` PASS readonly actualhistory. git diff --check PASS. No runtime/browser/build.

ROOT stamps POLICY_TEMPLATE.json final instrumentRevision, freezes policy/hash, then independently prepares fresh receipts. Separate commands, never an automatic loop:

```sh
node apps/desktop/e2e/ui-foundation/characterization-observations.mjs launch-slot "$POLICY" "$POLICY_SHA" 10000.1 "$R_10000_1" "$H_10000_1"
node apps/desktop/e2e/ui-foundation/characterization-observations.mjs launch-slot "$POLICY" "$POLICY_SHA" 10000.2 "$R_10000_2" "$H_10000_2"
node apps/desktop/e2e/ui-foundation/characterization-observations.mjs launch-slot "$POLICY" "$POLICY_SHA" 10000.3 "$R_10000_3" "$H_10000_3"
node apps/desktop/e2e/ui-foundation/characterization-observations.mjs launch-slot "$POLICY" "$POLICY_SHA" 10000.4 "$R_10000_4" "$H_10000_4"
node apps/desktop/e2e/ui-foundation/characterization-observations.mjs launch-slot "$POLICY" "$POLICY_SHA" 10000.5 "$R_10000_5" "$H_10000_5"
node apps/desktop/e2e/ui-foundation/characterization-observations.mjs close-one-success "$POLICY" "$POLICY_SHA" "$CLOSE_RECEIPT" "$CLOSE_RECEIPT_SHA"
```

Only launch next after an unsuccessful prior actual run and verified preconditions. Stop immediately upon validcomplete actualprocesssuccess, then use fresh same-slot close receipt. Existing targetmiss remains targetmiss. Closing without success fails; no automatic sixth. First10000receipt predecessor is1000.4claim de72c0ef4732e3a81abe5b02a7ca757cccd8622bb4a5ccfa69cbc2d96faa3c53 and requires external recovery. No new1000command is authorized.
