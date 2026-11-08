# Missing-data warning navigation: open gap

`apps/desktop/src/features/missing-data/MissingDataBlockingPanel.tsx` carries
`affected_refs` in the warning type, but its warning articles render only the
message and remediation, without affected-reference navigation. OUT-001 and
CLM-004 require affected-object access, including text/assistive paths.
The generic DiagnosticsPanel does provide navigation: this finding is scoped
to the missing-data surface, not the whole App, and is source inspection only.

The proposed resolution is to render the applicable references and accessible
navigation on this surface (Agent 0 recommendation). Narrowing the target is
the alternative and requires an owner decision; none is recorded. Next actor:
Agent 0 scopes the connecting-path repair or presents the narrowing to the
owner. No requirement or implementation changes in this note.
