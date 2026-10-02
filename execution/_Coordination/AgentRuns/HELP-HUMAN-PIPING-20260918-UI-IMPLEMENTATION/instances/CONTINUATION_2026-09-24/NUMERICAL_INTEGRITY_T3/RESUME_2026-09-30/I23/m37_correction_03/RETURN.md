# I23 K4-M37 correction 03 return

One corrected current-source patch prepared; no runtime or semantic acceptance.
Start **2026-10-01 13:27:01 UTC**; return **2026-10-01T13:28:42Z**
(101.4 seconds, within ten minutes).

K4-M37 retains its registered meaning: Phi participates in R7's stop rule but
is omitted from final publication classification/scales. The one-line fault
changes the actual shared producer's `if let Some(phi) = floor` to
`if let Some(phi) = floor.filter(|_| false)`. Both public floored classification
and the metered certificate use this producer. Original coupling stays active;
only force/moment max-with-Phi is suppressed. Rule computation, transported
Some floor, floor shape/finite validation and accepted-draft construction are
unchanged. This is a current-source reconstruction, not historical byte identity.

The two existing exact filters are mapped in PATCH_MAP.json. The classification
test's binding force row must become AbsoluteVerified with the fixed floor-bound;
the fault instead leaves it RelativeVerified. The certificate test must reach
Accepted(d) for a valid p512 floor, then preserve floor bits in force/moment
body scales. With the least-subnormal floor, the intended force-scale failure
is 0 versus 1 after acceptance. A Shape rejection, generic accepted-draft panic,
compiler failure, empty filter or unrelated failure supplies no kill.

Static checks passed: one unique anchor in publication_with, exact unified-diff
application, expected postimage, reversal, and byte-identical rule/transport/
certificate validation/finalization functions. No test or diagnostic was added.
The old protected_01 patch and its incomplete-discriminator history remain
sealed and unchanged; it is explicitly unready for current semantic credit
because replacing the floor argument with None trips Shape first.

Source: 40129a225d73860ac2a53da9a2fa73869df668f3.
Patch SHA256: `809471ddeb17c83f7279c711be98bbbb4da6d413e7725a36d87f71ce3f02312f`.
Postimage SHA256: `a8599ba9345d2d0cfc521f7b17676311f42d9e8e27b1aa1d2a1697606a207f9d`.

ROOT and RV29 must read this exact patch before any separate runtime grant.
No Rust/build/test/solver, Git command/index mutation, maintained edit or
delegation occurred. SHA256SUMS seals this additive correction packet.
