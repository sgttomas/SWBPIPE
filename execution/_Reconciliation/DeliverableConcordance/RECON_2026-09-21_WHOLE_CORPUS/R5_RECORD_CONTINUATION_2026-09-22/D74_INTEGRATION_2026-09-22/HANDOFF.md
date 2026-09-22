# D-74 postintegration overlay

This append-only overlay binds the D-74 license ruling from PR #854 into the
R5 Piping record-reconciliation candidate. D-74 is recorded in
`execution/_Coordination/_DECISIONS/D-74_RULING_2026-09-22.md`; its source
commit is `67e4738b2f276e3623cb16be6f8c2d7803f51521`. The merge base is
`0fb42b36df5c93c34c02e209670f3cede937ce84`, and the preintegration R5
candidate is `a89b5ddecfb6d1ea8cca1b68d4895ab511e5c370`.

The owner selected MIT. The D-74 commit changes 20 Piping paths: nine also
repaired by R5 and eleven touched only by D-74. `MANIFEST.json` binds each
postimage, each D-74 source image and, for the nine overlaps, the a89 preimage
against its original `SOURCE_BINDINGS.csv` `AfterSHA256`. Eight D-74-only
paths remain byte-identical to the ruling commit. Three live authority carriers
were corrected against accepted decisions: `docs/DIRECTIVE.md` and
`governance/MAINTAINERS.md` now reflect DEC-027/079; the development workflow
also carries its accepted DEC-025/059/060/093 routes. Their exact postimages
are separately bound in the manifest.

One additional current carrier, `docs/BUILD_AND_RELEASE.md`, was outside
D-74's 20 paths. Independent review found stale authority language there.
This overlay binds its preimage from a89 and its current postimage separately.
The correction records DEC-027 maintainer and release authority, DEC-057's
unsigned v0.1 posture and D-06b follow-up, DEC-059 conditional public-export
CI and DEC-093 surface-4 evidence. Coverage, performance, tolerance and
permitted-variance thresholds remain open.

The original R5 subtree remains byte-for-byte equal to a89: all 34 preexisting
files, including `CHECK_RESULT.json`, the three `PHYSICAL_EDITS.json`
manifests, `SOURCE_BINDINGS.csv`, and the independent review. The historical
R5 `CHECK_RESULT` remains a PASS for its a89 source basis only; this overlay
does not claim that the original backcheck validates post-D-74 bytes. No
existing R5 record or physical manifest was rewritten.

The current approved graph pointer remains DAG-011. D-74 directs later
reconciliation to record its license effect against DEL-01-01 and DEL-01-03;
this overlay records that downstream note while preserving the original
deliverable boundaries. DEL-01-01 remains ISSUED with its census `NONE` entry,
which is not completion evidence. DEL-01-03 remains IN_PROGRESS with three
recorded items: contribution intake stays closed under DEC-027/079 pending a
future owner activation and legal-instrument choice on legal advice; the
pre-release legal review remains open; and sanitized public issue-template
wiring remains open. No deliverable lifecycle or product/release acceptance
changes here.

Run `python3 verify_integration.py` from this folder to check the exact
21-path source overlay, the complete preserved a89 R5 subtree, MIT and DAG
pointers, the DEC-027/079 authority carriers, the DEL-01-01/-03 residual
records, and an in-memory tamper rejection through the same postimage
acceptance predicate. The overlay is bounded byte and statement checking; it
does not replace full semantic review, legal advice, product verification,
release approval, or a new full R5 backcheck against an integrated revision.
