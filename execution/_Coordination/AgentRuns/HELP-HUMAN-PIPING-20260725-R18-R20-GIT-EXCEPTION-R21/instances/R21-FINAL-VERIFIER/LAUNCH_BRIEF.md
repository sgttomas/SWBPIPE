# Fresh Agent 2 launch brief — D-57 / DEC-090

Act as a fresh read-only ephemeral Agent 2 verifier. Do not write files,
delegate, stage, commit, push, merge, or use network.

Verify:

1. owner payload exact bytes/hash and D-57 case-specific semantics;
2. D-57 and `DEC-090` are live next-free identifiers, with one matching ruled
   register row and no duplicate;
3. frozen branch/HEAD and the exact 105-path pre-governance index list/count/
   hash;
4. `git diff --cached --check` truthfully exits `2` with exactly the bound 26
   findings and no other finding;
5. every affected staged blob matches its bound SHA-256 row, with exactly 26
   unique paths and no normalization;
6. the governance addition is limited to D-57, one register-row edit, one
   `DEC-090` append, R21, and future Receipt-75; all new text has exactly one
   newline at EOF and adds no finding;
7. previous R18–R20 terminal semantics and protected hashes remain unchanged;
8. receipt contract through Receipt-74, R18–R21 JSON, claims/path validation,
   and whitespace/diff checks except the exact 26 all pass;
9. no hidden product/configuration, deliverable lifecycle, DAG, build,
   packaging, release, publication, external, push, merge, or network effect;
10. any index/finding/hash/scope/protected-state drift returns `BLOCK`.

Return `PASS / COMMIT-SAFE` or `BLOCK`, exact commands and hashes, and the
closeout gate `PASS_WITH_OWNER_EXCEPTION_DEC_090` only if every predicate
passes. Receipt-75 is not present during this verification and may be appended
only after an accepted pass.
