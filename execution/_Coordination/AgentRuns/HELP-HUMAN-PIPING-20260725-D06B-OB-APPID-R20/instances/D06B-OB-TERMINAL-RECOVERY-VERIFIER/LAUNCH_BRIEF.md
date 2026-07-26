# Fresh Agent 2 launch brief — R20 terminal recovery

Act as a fresh read-only ephemeral Agent 2 verifier. Do not write files,
delegate, mutate Git, or use network.

Verify:

1. the two intended terminal labels are exact:
   - R20 `ORCHESTRATION_PLAN.md` status is
     `TERMINAL_VERIFIED_RECEIPT_INTEGRATED`;
   - `SOFTWARE_DECOMP.md` `DEC-089` effect label contains
     `APP_ID_EXTERNAL_RESULT_VERIFIED_RECEIPT_74_INTEGRATED`;
2. `WORK_GRAPH.json` changes only the four authorized statuses:
   - P1/P2 are `COMPLETED_AGENT2_COMMIT_SAFE`;
   - V1 is `COMPLETED_PASS_COMMIT_SAFE`;
   - I1 is `COMPLETED_TERMINAL_HANDOFF`;
   and its before/after hashes are accurately bound;
3. the correction audit truthfully distinguishes the frozen pre-verification
   manifest's plan binding, the recoverable immediate pre-correction plan hash,
   and the current corrected plan hash without claiming unsupported identity;
4. the first terminal-label verifier's `BLOCK` return/status remain preserved;
5. Receipt-74 exists exactly once, follows Receipt-73, has the correct parent,
   and records the accepted original final verifier `PASS / COMMIT-SAFE`;
6. D-06b O-B ruling, exact App ID result, unsigned posture, and all residual
   gates remain semantically unchanged;
7. `VERIFICATION_MANIFEST.md` remains unchanged as a frozen
   pre-verification snapshot;
8. protected hashes outside the authorized correction targets and new audit
   records remain unchanged;
9. R20 JSON parses, whitespace/diff checks pass, HEAD/index are unchanged,
   and no hidden product, configuration, lifecycle, DAG, build, release, Git,
   network, or external effect appears.

Return `PASS / COMMIT-SAFE` or `BLOCK` with commands, evidence, and exact
hashes.
