# Handoff V2: browser provenance amendment approval pending

Status: `OWNER_DECISION_REQUIRED`

Resume at [`PROPOSED_OWNER_AMENDMENT_V2.md`](PROPOSED_OWNER_AMENDMENT_V2.md). It supersedes V1 only by clarifying that the browser failure occurred before the changed fixture/deformation numeric assertion and by binding the verified portable patch resolver. The one-file three-line scope, candidate post-image, RF review, required tests, downstream gates, order, and expiry are unchanged.

The original six-path changes are applied and frozen; five hashes remain fixed, while candidate `be0650f640986a9282db43803107439f0c08d71a7c37639508e3758fa6c13248` is reviewed but unapplied. Decode applicable patch bytes only from archive `020ec2c68f9fc317f7de0ffb0162563555947caff4b767a8feb90c424e255c46` through resolver `e9c1ffc76e3289fba2c15a48bb3f9f9eb28155ce4b3de9e57730821a83173ab4`, and verify decoded SHA-256 `e7fc8179bd12f00969c8692ef2bc3ee5a6209f3a1f3dfa9014d79ea0d202fbcc`. The old `.diff` path is a non-applicable pointer. Root verified the portability package; global candidate-whitespace and path-anchor validators pass with zero findings.

Without Owner approval, do not apply the candidate or resume native/release work. After approval, run both named journeys in both Playwright projects and require fresh RF applied backcheck before the preserved native and release sequence. No native run, clean full DEC-025, Receipt 137, publication, or Git completion occurred for this amendment.
