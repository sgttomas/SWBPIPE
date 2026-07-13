# Procedure: DEL-17-05 CAEPIPE external run harness and CSV parser

<!-- D41-R5-T7-PDU055-CURRENTNESS -->
## D-41 R5 T7 PDU-055 current declaration

Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.

Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.

PDU-055 cited claim(s): `DEL-17-05-DECL-004`.

## Purpose

This procedure defines how to populate, review, and later implement the optional CAEPIPE external run harness and CSV parser without bundling commercial software, bypassing licenses, overclaiming compatibility, or converting regression evidence into professional acceptance.

## Prerequisites

For this Phase A documentation pass:

1. Read AGENTS/TASK/ORCHESTRATOR instructions and the `four-documents` skill contract.
2. Read DEL-17-05 local `_CONTEXT.md`, `_STATUS.md`, `_REFERENCES.md`, `_DEPENDENCIES.md`, and `MEMORY.md`.
3. Read DEL-17-01, DEL-17-02, and DEL-17-04 four-document kits.
4. Read DEL-17-01 `Source_Basis_Register.md` and `CAEPIPE_Question_Dossier.md`.
5. Read the DEL-17-05 decomposition entry, scope items, objectives, project invariants, and data-boundary policy.
6. Verify that DEL-17-04 is the declared upstream dependency and that unresolved DEL-17-04 target-profile gates are carried forward.

For a later implementation tranche:

1. Confirm the implementation tranche has explicit write scope for code, schemas, tests, or fixtures.
2. Confirm the executable path is user-configured and not bundled.
3. Confirm the user owns license and execution-environment responsibility.
4. Confirm the explicit configuration surface for the executable path; until selected, carry `TBD` for the environment variable, config key, CLI option, GUI field, or equivalent.
5. Confirm whether the first invocation profile has been resolved or remains an explicit `TBD`.
6. Confirm the accepted DEL-17-04 MBF profile/writer tranche that may be used for live external-run tests; until accepted, carry the profile identifier as `TBD`.
7. Confirm parser fixture provenance before adding public fixtures.
8. Confirm no private user/project CAEPIPE output is committed without documented redistribution rights.

## Steps

### Phase A document population

1. Create the four-document kit: `Datasheet.md`, `Specification.md`, `Guidance.md`, and `Procedure.md`.
2. Ground CAEPIPE-specific facts in DEL-17-01 source IDs and official/public CAEPIPE references.
3. Carry forward unresolved target behavior as `TBD`, especially:
   - first CAEPIPE version/profile;
   - first MBF profile required by the harness;
   - exact invocation profile;
   - stable CSV sections for parser coverage;
   - stable canonical-ID correlation strategy for parsed rows.
4. Add a conflict table when public references are insufficient to choose behavior.
5. Avoid implementation details that would become code, schema, fixture, CLI, GUI, release, compatibility, code-compliance, or professional claims.
6. Run four-document and minimum-fileset validation.
7. Update `_STATUS.md` from `OPEN` to `INITIALIZED` only if the P1/P2 document pass completes successfully.
8. Record run evidence in `_run_records/TASK_RUN_*.md`.

### Future external-run procedure

This section is a contract for later implementation, not an implementation in this tranche.

1. Load an export package produced under the accepted DEL-17-02/DEL-17-04 contract.
2. Confirm the MBF input exists and has associated manifest, ID-map, and loss-report records.
3. Resolve the user-configured CAEPIPE executable path without searching for or installing CAEPIPE; record the selected configuration surface or `TBD` if a later implementation brief has not selected one.
4. Build a run directory under user/project control.
5. Record execution-environment context: operating system, working directory, permissions relevant to invocation, shell or process-launch context, remote Windows or compatibility-layer notes if used, path quoting notes where applicable, and whether the host permits stdout/stderr capture.
6. Record pre-run metadata: source model/export IDs, MBF path, accepted DEL-17-04 profile ID or `TBD`, command profile, executable path provenance, working directory, environment notes, and boundary notices.
7. Invoke the external executable only when the user-owned configuration is present.
8. Capture exit status and stdout/stderr where the host environment permits.
9. Discover the target CSV artifact using the selected invocation profile and record missing-output diagnostics if it is absent.
10. Run the CSV parser on the discovered output only within the declared parser coverage.
11. Bind parsed evidence to canonical IDs through the manifest/ID map where possible.
12. Mark unparsed sections, unknown sections, unmapped rows, unit/coordinate uncertainty, and unsupported target behavior as diagnostics or `TBD`.
13. Write a run metadata record and parser diagnostics that classify the result as regression/handoff evidence only.

### Future skip-without-executable procedure

1. Check whether an executable path has been explicitly configured.
2. If absent, skip external execution tests and record the skip reason.
3. Continue parser-only and contract validation tests that do not require CAEPIPE.
4. Do not fail public CI solely because no CAEPIPE executable or license is present.

### Future run-record field list

The later implementation record shall be durable enough for review even when the external executable is absent or the CSV artifact is not produced. Minimum fields are:

- configuration surface checked;
- configured executable path state, recorded as present, absent, invalid, or private/redacted as appropriate;
- license/environment responsibility acknowledgement state;
- source model/export IDs;
- accepted DEL-17-04 MBF profile ID or `TBD`;
- MBF input path and manifest/ID-map/loss-report references;
- command profile and command shape actually used, with sensitive paths redacted when needed;
- operating system, working directory, permissions, shell/process-launch context, and compatibility or remote-Windows notes;
- expected target CSV artifact path and observed target CSV artifact path;
- exit status, stdout availability, stderr availability, and capture locations where allowed;
- output discovery status;
- parser coverage-register version or `TBD`;
- parser status, unsupported sections, unmapped rows, missing sections, and diagnostic severity summary;
- evidence classification stating regression/handoff evidence only;
- boundary note confirming no bundled executable, license bypass, protected fixture, compatibility proof, code-compliance claim, formal-validation claim, or professional-acceptance claim.

## Verification

Phase A verification:

```bash
tools/validation/check_four_documents.sh "execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-05_CAEPIPE external run harness and CSV parser"
tools/validation/check_min_viable_fileset.sh "execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-05_CAEPIPE external run harness and CSV parser"
git diff --check
```

Manual Phase A checks:

- all four default document schemas are present;
- all target-specific unresolved behavior is `TBD`;
- no proprietary examples or protected standards data are introduced;
- no implementation code, schema, fixture, release, compatibility, code-compliance, formal-validation, or professional claim is introduced;
- `_STATUS.md` changes only through the safe `OPEN` to `INITIALIZED` transition;
- `_run_records` captures sources, outputs, validation, and remaining gaps.

Future implementation checks:

- skip-without-executable tests pass in public CI;
- opt-in executable tests require explicit user configuration;
- parser coverage register matches invented or rights-cleared fixtures;
- run metadata records manifest, ID-map, loss-report, command profile, output discovery, parser diagnostics, and evidence classification;
- parsed evidence is tied to canonical IDs or explicitly marked weak/unmapped.

## Records

Phase A records:

- four-document kit;
- updated `_STATUS.md`;
- `_run_records/TASK_RUN_2026-05-18_1156.md`;
- validation command output summarized in the run record.

Future implementation records:

- executable configuration provenance;
- run directory metadata;
- external-run record;
- stdout/stderr capture where available;
- CSV parser coverage record;
- parser diagnostics;
- manifest/ID-map/loss-report references;
- fixture provenance review;
- skipped-test evidence when the executable is absent.
## D-41 R5 T3 PDU-016 Check

Verify default and partial-override packages remain `private_user_controlled`, local-only, and telemetry-disabled; embedded private/protected/commercial payload remains blocking.
