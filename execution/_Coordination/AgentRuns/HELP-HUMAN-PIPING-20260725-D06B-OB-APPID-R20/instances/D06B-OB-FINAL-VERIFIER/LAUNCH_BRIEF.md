# Launch brief — D06B-OB-FINAL-VERIFIER

You are one fresh Agent 2 ephemeral generalist. Perform a read-only terminal
verification of R20. Write nothing. Return one evidence-backed verdict:
`PASS / COMMIT-SAFE` or `BLOCK`.

Verify all of the following:

1. The exact 238-byte owner direction and its SHA-256; D-06b O-B semantics;
   the exact Explicit App ID description `OpenPipeStress Technical Preview`,
   bundle ID `org.openpipestress.technical-preview`, Explicit type, and no
   added selectable capability.
2. The external evidence is truthfully attributed as relayed by HELP_HUMAN:
   team `8A7JL35U4S`, resource ID `V49VYB9W92`, initial absence, final exact
   row, zero selectable capabilities, and disabled baseline In-App Purchase
   distinguished from an operator-selected capability.
3. Exactly one D-06b row is `RULED (O-B; 2026-07-25)` with proposal, ruling,
   and `DEC-089` traceability; no O-C successor exists.
4. `DEC-089` is the single next-free append after `DEC-088`, and the ruling,
   register, decision log, action brief/result, and handoff consistently say:
   only App ID registration completed; future signing/notarization remains
   policy-only; the unsigned posture continues; all certificate/profile/key,
   signing/notarization, build/release, and publication gates remain open.
5. The exact R20 write fence: dated ruling, one existing register row,
   one `DEC-089` append, and R20 subtree only. Verify R20 JSON, whitespace,
   diff, containment, and no hidden-effect wording.
6. Recompute all bound protected hashes in the R20 verification manifest,
   confirm the D-06b proposal and R19 evidence are unchanged, Receipt-73
   remains last and Receipt-74 is absent, and confirm no product/configuration,
   code/test, documentation/status, lifecycle, build/release, certificate,
   profile, key, signing/notarization, publication, Git, or manager network
   effect is represented or introduced.

The verifier may inspect Git and files read-only. Do not access the network or
Apple credentials. Do not alter any file, Git state, or external state.

