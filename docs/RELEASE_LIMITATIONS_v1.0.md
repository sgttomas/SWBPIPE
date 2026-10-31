# SWBPIPE v1.0 release limitations

**Status:** companion to `docs/RELEASE_TARGET_v1.0.md`, for exit criterion 3:
"Every remaining refusal in a Must row is listed in the release limitations and
accepted by the owner." This is the single list. Each entry is drafted by the
lane that owns the refusal and links to the deliverable `Work/` text that
explains it. An entry records a refusal and its reason. It does not accept
it: acceptance is the owner's act (F-PIP-3, release target §5), recorded in
the entry's status only when the owner gives it. The release notes'
"Known Limitations And Open Risks" section (`docs/RELEASE_NOTES_TEMPLATE.md`)
is written from the accepted entries.

## Entry form

Each entry is one subsection:

- **Id:** `LIM-nnn`, assigned in order and never reused.
- **Rows:** the `RT-` rows it affects (Must rows; Should rows may be listed
  for completeness).
- **Code:** the refusal or warning code the product emits, if any.
- **What is refused:** in the user's terms.
- **Why:** the engineering or source reason, in one or two sentences.
- **What the user does instead:** the supported alternative, if any.
- **Basis:** the deliverable and `Work/` section that governs it.
- **Status:** `owner's acceptance pending` until the owner rules; then the
  owner's words and date. A refusal later removed by a landed row is marked
  `withdrawn` with the PR, not deleted.

Entries are added in the same PR that makes the refusal standing. The PR names
the entry, and the entry names the PR.

## Entries

None yet.
