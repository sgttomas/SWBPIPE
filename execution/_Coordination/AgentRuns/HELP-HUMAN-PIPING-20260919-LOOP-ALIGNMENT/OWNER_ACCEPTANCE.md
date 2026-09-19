# Standing loop alignment — owner acceptance

Date: 2026-09-19. Transcript text retained from the active Codex conversation;
not transport-byte custody.

## Request (verbatim)

~~~
what about if we just rectify the standing loop instructions to match what we're doing now?  And archive the current version as a snapshot in time (there are `.archive/` folders for that purpose, existing, or create new).
~~~

SHA-256 of the text inside the fence, without trailing newline:
`61e50914f2114d951e7b97617fe868c248fca7b71dedfa8ab6aaa05b05fbc451`.

## Acceptance (verbatim)

~~~
accepted.  Proceed accordingly.
~~~

SHA-256 of the text inside the fence, without trailing newline:
`6df09d47ebd2f4981d45a2a686db3ce9f5fa5e8cff36d26b871243d9d7ed491b`.

The preceding assistant proposal linked the complete replacement and change
inventory, and asked whether to accept integration with Agent 0 performing
closeout and one fresh Astra/high Type 2 reviewing the complete diff. The owner
accepted that proposal. This is acceptance of the standing procedure amendment
and bounded delegation, not acceptance of any product deliverable.

`ACCEPTED_DRAFT_MANIFEST.json` retains the exact reviewed draft hashes. At
initial implementation commit `571bfa8ffb4c97841c95ecc84036251262a73fcc`, only
the advertised adoption-marker substitutions separated those draft bytes from
the installed instructions. `ADOPTION_TRANSFORMS.json` records each exact
substitution and resulting hash, allowing reconstruction of the accepted draft.
The owner's subsequent separation-of-concerns direction and its implementation
are recorded in `STEERING_AMENDMENT.md`; that amendment changes the entrypoint
after the initial adoption without changing the retained draft or its verdict.

Old dated plans remain at their cited paths. The superseded project instructions,
entry and prior active plan are preserved byte-for-byte in
`loop/.archive/2026-09-19-standing-loop/`, relative to the project. The snapshot's
source commit and hashes are recorded there.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
