# Fresh Agent 2 launch brief — R20 terminal-label correction

Act as a fresh read-only ephemeral Agent 2 verifier. Do not write files,
delegate, use Git mutation, or use network.

Verify:

1. exactly the two intended stale terminal labels changed:
   - R20 `ORCHESTRATION_PLAN.md` status is
     `TERMINAL_VERIFIED_RECEIPT_INTEGRATED`;
   - `SOFTWARE_DECOMP.md` `DEC-089` effect label contains
     `APP_ID_EXTERNAL_RESULT_VERIFIED_RECEIPT_74_INTEGRATED`;
2. all other substantive text in those two targets is unchanged from the
   pre-correction forms documented by the task and correction record;
3. `Receipt-74` exists exactly once, follows `Receipt-73`, has the correct
   parent, and records the accepted fresh Agent 2 `PASS / COMMIT-SAFE`;
4. D-06b O-B ruling, exact App ID result, unsigned posture, and all residual
   gates remain semantically unchanged;
5. `VERIFICATION_MANIFEST.md` remains byte-identical to its frozen
   pre-verification snapshot;
6. protected hashes outside the two correction targets and newly added audit
   records remain unchanged;
7. R20 JSON parses, whitespace/diff checks pass, and no hidden product,
   configuration, lifecycle, DAG, build, release, Git, network, or external
   effect appears.

Return `PASS / COMMIT-SAFE` or `BLOCK` with commands, evidence, exact hashes,
and any finding.
