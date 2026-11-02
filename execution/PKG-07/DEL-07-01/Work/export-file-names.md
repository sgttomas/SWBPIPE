# Export file names (shared convention)

This document gives the one user-facing file-name convention for every file the desktop
suggests when the user exports or saves something outside the local store. It covers CSV
tables, result JSON, handoff packages, MBF files when they come (RT-R7), and every later
export. Agent 0 set the requirement on 2026-10-28: one convention, no developer text and no
project name. It was agreed between the W-17 lane and the reports lane, which owns RT-R6 CSV
export and DEL-08-04. Both lanes cite this document instead of restating it.

It serves release-target rows RT-U1 (coherent visual design, including the user-facing
wording), RT-R6 (CSV export) and RT-R7 (MBF export), in `docs/RELEASE_TARGET_v1.0.md`.

## The rule

A suggested name has the form `swbpipe-<kind>[-<qualifier>].<ext>`.

- **Tokens.** Every token is lowercase ASCII letters and digits, and tokens are separated by
  single hyphens. There is no leading, trailing or doubled hyphen. The regular expression for
  the stem is `^swbpipe(-[a-z0-9]+)+$`.
- **`<kind>`** is a fixed slug that names what the file holds (the table below). A kind may
  have several tokens, for example `results-stress-neutral`.
- **`<qualifier>`** is optional. It is the slug of the case, filter, run or state the file
  belongs to. A user label that slugs to nothing falls back to the stable key it labels (a case
  id or a run number), slugged the same way. Without a qualifier the name is
  `swbpipe-<kind>.<ext>`.
- **No project or model identity.** A suggested name carries no project name, project id,
  model id or path, which matches the autosave privacy choice (DEL-02-05 Design
  `autosave-and-backup.md`). The file's contents may still identify the project. Only the
  suggested name is constrained.
- **Collisions.** The native save chooser suggests the name. On a collision in the chosen
  folder, the existing native behaviour appends ` (n)` before the extension. That suffix is
  the only part of a name outside the token rule.
- **The user decides.** The rule covers the suggested name only. The user may type any name
  the chooser accepts.
- **Native checks.** Some native commands validate the suggested name that the webview sends.
  For example, `native_result_download.rs` `valid_name` checks a fixed prefix and then
  lowercase hyphen-separated tokens. Such a command validates against this rule's
  `swbpipe-<kind>-` prefix for its kind, using the same token check. It does not constrain the
  path the user then chooses.

## Kinds

| Kind | Extension | Qualifier | Owner |
|---|---|---|---|
| `results-<group>` (one results table group) | `.csv` | case or filter | reports (RT-R6) |
| `displacement-extremes` | `.csv` | case or filter | reports (RT-R6) |
| `stress-ratio` | `.csv` | case or filter | reports (RT-R6) |
| `support-loads` / `support-loads-all` | `.csv` | support or filter | reports (RT-R6) |
| `weight` / `weight-cog` | `.csv` | filter | reports (RT-R6) |
| `run-comparison` / `state-comparison` | `.csv` | the two runs or states | reports (RT-R6) |
| `results` (result JSON) | `.json` | `run-<n>` | reports (DEL-08-04) |
| `results-stress-neutral` (stress-neutral result JSON) | `.json` | `run-<n>` | reports (DEL-08-04) |
| `handoff` (handoff package folder) | `.swbhandoff` | saved state label, or the date `yyyy-mm-dd` | W-17 (DEL-02-05 / handoff) |
| `model` (MBF export, RT-R7) | `.mbf` | none, or the user's label | W-17 desktop side, with the CAEPIPE lane's writer |
| identity file of an MBF export (derived, not suggested) | `.swbpipe-ids.json` | none: native code names it `<chosen stem>.swbpipe-ids.json` beside the chosen `.mbf` | DEL-17-04 (Design `desktop-mbf-export-import.md` §3.2) |
| `report` (report package) | the package's existing extension | report label | reports |

A new export adds a row here in the PR that introduces it.

## What this replaces

- **Result downloads.** These drop the old developer prefixes `openpipestress-preview-results-*` and
  `openpipestress-preview-stress-neutral-*`, set in `native_result_download.rs`'s
  `RESULT_FILE_PREFIX` / `STRESS_FILE_PREFIX` and in their callers in `lib.rs`.
- **Developer-panel downloads.** These drop the old `openpipestress-preview-<panel>-<project id>`
  names, for example in `AccessibilityBaselinePanel.tsx` and `BuildReadinessPanel.tsx`. Each
  of these panels either moves to this rule, or it is a developer surface outside the product
  and is labelled or removed under the walkthrough's developer-text item (RT-U2).

The renames themselves are code changes and are made in their own PRs, by the lane that owns
each surface.

- **Smoke and fixtures.** Any native smoke step that pins a suggested name needs checking before
  a rename. `apps/desktop/native-smoke/expected.json` is owner-reserved: a rename that would
  change it stops and is reported.
- **Recorded artefacts.** Names inside stored or recorded artefacts, such as recorded runs and
  fixtures, are not renamed retroactively.
