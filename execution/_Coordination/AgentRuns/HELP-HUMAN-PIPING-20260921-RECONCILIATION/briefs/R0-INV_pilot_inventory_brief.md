# Brief — R0 pilot reverse inventory (owner-free capability rows)

Parent: HELP_HUMAN Agent 0, run `HELP-HUMAN-PIPING-20260921-RECONCILIATION`
(activation D-73 / DEC-110). Role: TASK (Type 2), read-only except the one
output file. Do not delegate.

The launch message supplies `{FREEZE}` (absolute path of a read-only checkout
of the frozen state `00115c71931bcae79909602d653740d3bb72dfa1`) and `{REPO}`
(the repository checkout holding the run folder).

## Purpose

Describe **what the code does**, from the code itself, so that later workers
can say which deliverable, if any, owns each capability. You must not know or
guess ownership. That independence is the point of your work.

## Boundary

- Read only these areas of `{FREEZE}/projects/chirality-piping/`:
  - `core/solver/`
  - `core/security/`
  - `core/adapters/`
  - `apps/desktop/src/features/model-tree/`
  - `apps/desktop/src/features/workspace/`
  - the tests that exercise them (under `tests/` and the feature folders)
- Also read enough of `apps/desktop/src/App.tsx` and
  `apps/desktop/src/services/` to see how those two features are reached.
- **Do not read anything under `execution/`, `plans/`, `loop/` or `docs/`,**
  or any document that maps code to deliverables. Do not name deliverables,
  packages, requirement IDs or decisions in your output.
- Write only
  `{REPO}/projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/R0_CALIBRATION/PILOT_CAPABILITIES.csv`.
- No builds, installs, test runs, git writes or network access.
- Never quote protected standards, vendor or private data. Standard claim
  fence applies (F-PIP-2; claims taxonomy per DEC-081).

## Output

RFC-4180 CSV, CRLF, UTF-8, with this header:

```
CapabilityID, Area, Kind, Capability, EntryPoints, Tests, Notes
```

- `CapabilityID`: `CAP-<AREA>-NNN`, where `<AREA>` is one of `SOLVER`, `SEC`,
  `ADAPT`, `TREE` or `WS`.
- `Kind`: one of
  - `ENGINE` (computation);
  - `DATA_CONTRACT` (schema, serialization, validation rule);
  - `UI_OPERATION` (something a user can do);
  - `UI_SURFACE` (panel, view, display);
  - `COMMAND` (application or native command);
  - `DIAGNOSTIC` (warning, error, status reporting);
  - `INTEGRATION` (external tool, file format, process);
  - `SECURITY_CONTROL`;
  - `TEST_ONLY` (harness or fixture capability with no product path).
- `Capability`: one plain sentence, at most 200 characters, saying what the
  code does for a user or for the system. Describe behaviour, not files.
- `EntryPoints`: semicolon-separated repository-relative paths, with `::symbol`
  where it helps.
- `Tests`: semicolon-separated test files (with `::case` where useful) that
  exercise it, or `NONE_FOUND`.
- `Notes`: limits, partial behaviour and dead code, in your own words.

Grain: one row per distinct capability a reviewer could meaningfully ask
"who owns this?" about. That is a user action, a solver feature, a validation
rule family, a panel, or an external integration. It is not one row per file
or function. Expect roughly 40–90 rows across the five areas. Cover every
non-test source file in the areas at least once, through `EntryPoints`.

End the file with a final record `#END,,,,,,<row count>`.

## Return

```
DONE PILOT_CAPABILITIES rows=<n> sha256=<sha256>
```

Follow it with at most five lines: areas covered, and any source file you
could not place.
