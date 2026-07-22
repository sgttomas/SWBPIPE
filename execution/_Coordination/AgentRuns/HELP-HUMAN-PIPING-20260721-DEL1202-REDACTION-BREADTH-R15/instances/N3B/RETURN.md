# N3B Return — Candidate v2 independent refutation

**Verdict:** `BLOCK`

The v2 amendment cures the five N3 defects concerning missed CLI routes,
route-by-route dispositions, fixed route contexts, the post-N5 closeout gate,
and the existence of an affected-owner map. Two material defects remain:

1. **Route wrappers do not yet own local-private intent.** The existing
   Python/TypeScript parity contract intentionally honors item-carried
   `local_private_intent`, `explicit_local_private_intent`, and `user_intent`
   values, including nested `export_policy` forms. The v2 brief's proposed
   caller argument does not by itself prevent source payload metadata from
   authorizing private retention. The brief must require route projectors to
   remove or override all payload-carried intent before invoking the unchanged
   contract and must test that only wrapper/UI/CLI/writer intent controls a
   route. It must also preserve the contract's current local-private behavior
   for unknown values: unknown values can be `warning_only` without intent and
   must not be mislabeled private or publicly releasable.
2. **The affected-owner map is not exact.** It contains a `shared desktop
   surface` placeholder and grouped RouteIDs in the deliverable column. It must
   be replaced with an exact path-to-package-to-deliverable/shared-owner map,
   including an explicit no-state/handoff disposition for every affected
   non-DEL-12-02 surface.

Owner adoption is not ready. Preserve this return and submit both corrections
to a fresh N3C refuter.

