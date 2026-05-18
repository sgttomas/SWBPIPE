# Specification: DEL-17-05 CAEPIPE external run harness and CSV parser

## Scope

DEL-17-05 shall define the Phase A contract for an optional CAEPIPE external run harness and CAEPIPE CSV parser used as regression and handoff evidence.

This deliverable covers:

- user-owned external executable boundary;
- run directory and execution metadata requirements;
- CSV parser coverage boundaries;
- evidence classification for parsed CAEPIPE output;
- skip behavior when no executable is configured;
- carried `TBD` items for invocation profile, target version/profile, and stable CSV sections.

This Phase A deliverable shall not implement code, schemas, parser logic, target fixtures, GUI workflow, public API endpoints, CAEPIPE executable discovery, CAEPIPE bundling, license validation bypasses, release behavior, compatibility claims, code-compliance claims, professional acceptance claims, or formal validation claims.

## Requirements

| Req ID | Requirement |
|---|---|
| DEL-17-05-REQ-001 | The harness shall be optional and disabled unless a user supplies a local executable path and accepts responsibility for license and execution environment. |
| DEL-17-05-REQ-002 | The project shall not bundle, download, install, redistribute, wrap for license bypass, or imply availability of CAEPIPE or CAEPIPE 3D+. |
| DEL-17-05-REQ-003 | The first executable invocation profile shall remain `TBD` until public documentation and/or CAEPIPE support clarification reconciles the command-line and batch-mode patterns. |
| DEL-17-05-REQ-004 | The harness shall record the command shape actually used, working directory, input MBF path, expected CSV path, observed CSV path, exit status if available, stdout/stderr capture if available, and diagnostics. |
| DEL-17-05-REQ-005 | The harness shall treat the MBF input as an output of the accepted DEL-17-04 profile/writer tranche; if the required MBF profile is unresolved, the run record shall carry that `TBD`. |
| DEL-17-05-REQ-006 | The run directory contract shall include or reference the export manifest, ID map, loss report, MBF input, CSV output when produced, and CAEPIPE run metadata. |
| DEL-17-05-REQ-007 | The harness shall verify only operational evidence: input file presence, process invocation attempt, CSV discovery, parser execution, expected section/count checks when source-confirmed, and diagnostics. |
| DEL-17-05-REQ-008 | The harness shall not classify a successful CAEPIPE run or parsed CSV output as professional acceptance, code compliance, formal validation, or compatibility proof. |
| DEL-17-05-REQ-009 | The CSV parser shall parse only source-confirmed or fixture-confirmed sections and shall mark unsupported, unrecognized, unstable, missing, or unmapped sections as diagnostics or `TBD`. |
| DEL-17-05-REQ-010 | Stable result correlation shall use canonical IDs through DEL-17-02/DEL-17-04 sidecars and manifests where possible; any correlation by target row order or names alone shall be marked weak or `TBD`. |
| DEL-17-05-REQ-011 | Parser results shall be bound to source model/export/run metadata before use as regression or handoff evidence. |
| DEL-17-05-REQ-012 | Tests that require the external executable shall skip when no executable path is configured and shall report the skip reason without failing normal public CI. |
| DEL-17-05-REQ-013 | Public parser fixtures shall be invented, project-owned, public/permissive, or rights-cleared; proprietary CAEPIPE examples, private user projects, and protected standards-derived data shall not be committed. |
| DEL-17-05-REQ-014 | User-provided CAEPIPE CSV outputs shall default to private/user-controlled handling unless provenance and redistribution rights are documented. |
| DEL-17-05-REQ-015 | Diagnostics shall preserve unit, coordinate, ID-map, loss-report, and parser-coverage uncertainty instead of silently normalizing or accepting target output. |

### Required Placeholders for Later Implementation

These fields do not close the carried CAEPIPE TBDs; they define the evidence shape a later implementation tranche must fill before claiming repeatable harness behavior.

| Placeholder ID | Required placeholder | Source basis |
|---|---|---|
| DEL-17-05-PH-001 | Configuration surface for the executable path: `TBD` until a later tranche selects the environment variable, config key, CLI option, GUI field, or equivalent user-owned input. | REQ-001; `docs/IP_AND_DATA_BOUNDARY.md`; `CAEPIPE-IMPORT-MBF` command-line operation, lines 15-20; `CAEPIPE-BATCH` batch-mode notes, lines 40-52. |
| DEL-17-05-PH-002 | Run-directory record shape: `TBD` filenames or manifest links for MBF input, export manifest, ID map, loss report, target CSV artifact, run metadata, and parser diagnostics. | REQ-004/006; DEL-17-02 export package contract; DEL-17-04 MBF profile contract. |
| DEL-17-05-PH-003 | Parser coverage register shape: section name, source-or-fixture basis, supported status, expected row/field handling where known, unmapped-row handling, diagnostic severity, and fixture provenance. | REQ-009/013/015; DEL-17-01 `TBD-17-01-004`; `CAEPIPE-EXPORT-DATA`, lines 31-43. |
| DEL-17-05-PH-004 | Skip-without-executable evidence fields: configuration key checked, configured-path absence or invalidity, skip reason, public-CI nonfailure classification, parser-only tests still executed, and boundary note. | REQ-001/012; package exclusion against bundled solvers or license bypass. |

### TBD Closure Gates

| Gate | Closure condition |
|---|---|
| Invocation profile | REQ-003 may move out of `TBD` only after support clarification, an explicitly selected public command/batch pattern, or a documented project-owned adapter decision reconciles the command-line and batch-mode source patterns. |
| Parser section coverage | REQ-009 may move out of `TBD` for a CSV section only when the section is confirmed by public/official source text or by an invented/rights-cleared fixture with documented provenance and expected parser behavior. |
| First live MBF profile | REQ-005 may move out of `TBD` only when DEL-17-04 identifies the accepted MBF profile/writer tranche that the harness is allowed to consume. |

## Standards

| Source | Requirement treatment |
|---|---|
| `docs/CONTRACT.md` | OPS-K-IP-1 through OPS-K-IP-3, OPS-K-DATA-2, OPS-K-AUTH-1, OPS-K-UNIT-1, OPS-K-REPORT-1/2, OPS-K-PRIV-1, OPS-K-AGENT-1 through OPS-K-AGENT-4 apply. |
| `docs/IP_AND_DATA_BOUNDARY.md` | Public repository content must avoid protected standards text/tables, proprietary examples, private user data, and commercial software artifacts without redistribution rights. |
| DEL-17-01 source basis | CAEPIPE-specific facts require official/public CAEPIPE evidence or explicit `TBD`; external runs and parsed CSVs remain non-authoritative regression/handoff evidence. |
| DEL-17-02 export contract | Export packages require manifest, stable-ID map, loss report, boundary notes, and external execution as optional user-owned metadata. |
| DEL-17-04 MBF profile | CAEPIPE version/profile, MBF record subset, stable-ID strategy, and unsupported-entity severity remain carried gates for the harness. |
| Official CAEPIPE public documentation | Evidence supports MBF import/export, CSV output surfaces, and batch/command-line CSV production, but does not close parser-section coverage or OpenPipeStress compatibility. |

## Verification

| Verification ID | Verification approach |
|---|---|
| VER-001 | Four-document review confirms all default sections exist and all CAEPIPE-specific claims cite DEL-17-01 source IDs, DEL-17-02/DEL-17-04 contracts, or public CAEPIPE pages. |
| VER-002 | Boundary review confirms no bundled executable, proprietary example, protected standards data, license-bypass workflow, compatibility claim, release claim, code-compliance claim, professional claim, or formal validation claim. |
| VER-003 | TBD review confirms target version/profile, invocation profile, stable CSV sections, and stable ID correlation are not overstated. |
| VER-004 | Future implementation review shall confirm skip-without-executable behavior and no external executable requirement in public CI, including the configuration field checked, absence/invalidity evidence, skip reason, parser-only continuation, and public-CI nonfailure classification. |
| VER-005 | Future parser tests shall use invented or rights-cleared CSV fixtures and shall record parser coverage by section name, source-or-fixture basis, supported status, unmapped-row handling, diagnostic severity, and fixture provenance. |
| VER-006 | Future harness tests with a user-provided executable shall remain opt-in and record run metadata, output discovery, parser diagnostics, and evidence classification. |
| VER-007 | Future regression comparison shall bind parsed rows to canonical IDs or explicitly mark weak/unmapped evidence. |
| VER-008 | Future acceptance review shall confirm that the user-supplied executable path, license/environment responsibility, and selected configuration surface are recorded before any live external execution occurs. |

## Documentation

Required Phase A documentation artifacts are:

- `Datasheet.md`;
- `Specification.md`;
- `Guidance.md`;
- `Procedure.md`;
- `_STATUS.md` safe update from `OPEN` to `INITIALIZED` if the P1/P2 run completes;
- `_run_records/TASK_RUN_*.md`.

Future implementation documentation shall include:

- harness configuration guidance that states executable path, license, and execution environment are user-owned;
- run directory contract;
- CSV parser coverage register;
- skip-without-executable test behavior;
- fixture provenance notes;
- evidence and limitation notes for reports or handoff records.
