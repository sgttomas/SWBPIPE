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
Product assets retain their owning verification. Changes to code, CI, file
existence or mode outside `execution/`, or an unavailable diff retain the
product route. The required `harness` validates classification and every
selected job; a cancelled, missing or failed selection cannot produce success.

`execution/` holds declarative specifications only (owner direction,
2026-11-08): code that runs lives in the project's code locations. Any regular,
non-executable UTF-8 change there of a declarative kind (Markdown, YAML, JSON,
CSV, text, SHA256 lists, patches), including additions, deletions and renames,
takes the document route. It adds the maintained boundary guard
(`tools/validation/execution_boundary.py`), parsing of each changed JSON, YAML
and CSV file, and the parity and graph checks those specifications feed. An
invalid specification fails those checks rather than selecting product suites.
On the product route a changed `execution/` path selects no product suite, the
guard runs in the repository checks, and the Python product job, like the other
product jobs, checks out the tree without `execution/`, so a product read of it
fails visibly. The guard's threat model and stated limits are in its docstring;
it holds the 22 accepted SCA-011 helper sources as hash-checked fidelity
exceptions that are never run.

The owner requested documentation-only CI below two minutes on 2026-11-08.
Elapsed time is measured from the event through `harness`, including runner
queueing and setup. The clock target does not supply authority to omit an actual
content consumer. New document-content readers must enter the inventory as a
focused check or as inputs that retain product verification. No dependency,
release, signing, engineering acceptance or product qualification changes.

The reader inventory is source-bound. Any non-Markdown source/configuration
change, or any change to tracked paths/modes, outside `execution/` invalidates
its source digest; declarative `execution/` changes neither need nor stale it.
Subsequent documentation edits then keep product verification until changed
document readers have been audited and the inventory refreshed. This prevents
a new runtime or test reader silently inheriting a prose-only exemption.
