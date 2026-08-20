# Integrated review attempt 4

Frozen range: `357a58b56726feba49507534159c3fbc4656b818..f8e0b496b223b847c82a6aba03b1b67e48048de9`
Verdict: `FAIL`

The reviewer covered all 22 paths (3,752 insertions, 32 deletions); ancestry,
scope, and whitespace passed. It found: (1) protected privacy and canonical but
non-cleared adapter provenance could still reach `verification_passed`; and
(2) the N2 TypeScript test mutated the report model after freezing the mechanics
result, manifest, and run, so report provenance was not bound to the hashed
analysis input.

Disposition: N1 Amendment 4 landed as
`fd213b1ff5d8bdd7df78d013856b25415499439d`; its consolidated fail-closed JSON
preflight/snapshot path and direct/composed clearance/quarantine regressions pass
190 tests, and V26 fresh review passed with zero findings over 5,563 lines. N2
Amendment 2 landed as `93932d75ed220250b26012d1a58ec48be5df1240`;
mutation now precedes every dependent evidence build, manifest payload equality
is asserted, all focused checks pass, and fresh full-N2 review attempt 3 passed.
