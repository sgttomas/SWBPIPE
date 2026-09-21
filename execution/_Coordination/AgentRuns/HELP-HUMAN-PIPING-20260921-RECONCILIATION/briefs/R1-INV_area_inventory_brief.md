# Brief — R1 reverse inventory, one code area (owner-free capability rows)

Parent: HELP_HUMAN Agent 0, run `HELP-HUMAN-PIPING-20260921-RECONCILIATION`
(D-73 / DEC-110; R0 ruling item 1 adopted the whole-tree inventory,
`R0_REVIEW.md` §6). Role: TASK (Type 2). Read-only except your one output
file. Do not delegate.

The launch message supplies:
- `{AREA}`: a short code, for example `VIEW`;
- `{PATHS}`: the paths you own, relative to `projects/chirality-piping/`;
- `{FREEZE}`: a read-only checkout of the frozen state
  `00115c71931bcae79909602d653740d3bb72dfa1`;
- `{REPO}`: the repository checkout holding the run folder.

## Purpose

Describe **what the code and artifacts in your area do**, from the artifacts
themselves, so that later workers can say which deliverable owns each
capability, if any. Do not know or guess ownership: that independence is the
point of the work.

## Boundary

- Read your `{PATHS}` in `{FREEZE}/projects/chirality-piping/`.
- You may also read, to find how your area is reached and tested:
  - `tests/`, `apps/desktop/e2e/`, and in-crate or in-folder tests;
  - callers anywhere in the product tree.
- **Do not read anything under `execution/`, `plans/` or `loop/`.** Do not read
  any document that maps code to deliverables.
- In `docs/`, read only the files your `{PATHS}` name. A documentation area
  owns its documents as artifacts. Never use documentation to infer ownership
  of code.
- Do not name deliverables, packages, requirement or scope IDs, or decisions
  in `Capability` or `Notes`. Literal file paths that happen to contain such
  tokens are fine in `EntryPoints` and `Tests`.
- Write only
  `{REPO}/projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/R1_INVENTORY/INV_{AREA}.csv`.
- No builds, installs, test runs, git writes or network access.
- Never quote protected standards, vendor or private data. Standard claim
  fence applies (F-PIP-2; claims taxonomy per DEC-081).

## Output

RFC-4180 CSV, CRLF record endings, UTF-8. Header:

```
CapabilityID,Area,Kind,Capability,EntryPoints,Tests,Notes
```

- `CapabilityID`: `CAP-{AREA}-NNN`, numbered from 001.
- `Kind`: one of
  - `ENGINE`: computation;
  - `DATA_CONTRACT`: schema, serialization or validation rule;
  - `UI_OPERATION`: something a user can do;
  - `UI_SURFACE`: a panel, view or display;
  - `COMMAND`: an application or native command;
  - `DIAGNOSTIC`: warning, error or status reporting;
  - `INTEGRATION`: external tool, file format or process;
  - `SECURITY_CONTROL`;
  - `DOCUMENT`: a user, developer, theory, validation or governance document;
  - `CHECK`: a CI workflow, validator, release or evidence tool;
  - `FIXTURE_SET`: a family of fixtures, examples or benchmarks;
  - `TEST_ONLY`: a harness capability with no product path.
- `Capability`: one plain sentence, at most 200 characters, saying what it
  does for a user or for the system. Describe behaviour, not files.
- `EntryPoints`: repository-root paths (`projects/chirality-piping/...`),
  `;`-separated with no spaces, with `::symbol` where useful.
- `Tests`: repository-root paths of the tests that exercise the capability
  (with `::case` where useful), or `NONE_FOUND`.
- `Notes`: limits, partial or dead code, and anything a reviewer should know,
  in your own words.

**Grain.** One row per distinct capability that a reviewer could meaningfully
ask "who owns this?" about: a user action, an engine feature, a validation
rule family, a panel, an external integration, a document, a check, or a
fixture family. It is not one row per file or function.

**Coverage.** Every non-test source file in your area must appear in at least
one `EntryPoints` cell. For data, fixture and document areas, cover each
top-level family or document.

**Sentinel.** End the file with the record `#END,,,,,,<row count>`.

## Return

```
DONE INV_{AREA} rows=<n> sha256=<sha256>
```

Follow it with at most five lines covering:
- the areas covered;
- any file you could not place;
- anything that looked unreachable or dead.
