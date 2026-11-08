# Documentation CI routing

Ordinary prose changes have no solver, fixture or runtime effect. Python's
conservative read inference treats unresolved paths as any path, which selects
expensive numerical and mutation checks even for an introductory wording edit.
Splitting those checks into shards reduces elapsed time but does not remove
that unnecessary work.

The governing selection is in `docs/CI_STRATEGY.md` and
`tools/software_workflow/documentation_ci.py`. A complete, exact-candidate diff
of in-place human Markdown edits runs dependency-structure, changed-content
credential and conflict checks and the actual document-content consumers.
Product assets and preserved source inputs retain their owning verification.
Changes to code, CI, file existence or mode, or an unavailable diff retain the
product route. The required `harness` validates classification and every
selected job; a cancelled, missing or failed selection cannot produce success.

The owner requested documentation-only CI below two minutes on 2026-11-08.
Hosted run time is measured through `harness`; runner queue delay is reported
separately. The clock target does not supply authority to omit an actual
content consumer. New document-content readers must enter the inventory as a
focused check or as inputs that retain product verification. No dependency,
release, signing, engineering acceptance or product qualification changes.
