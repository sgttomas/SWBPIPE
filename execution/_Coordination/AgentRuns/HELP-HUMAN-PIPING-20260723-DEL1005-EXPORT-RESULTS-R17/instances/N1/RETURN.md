---
doc_id: R17-DEL1005-N1-RETURN
doc_kind: coordination.agent2_return
status: success
created: 2026-07-23
---

# N1 read-only reconnaissance return

Verdict: `CONDITIONALLY_CANDIDATE_READY`.

There is no technical or dependency blocker. Candidate execution must remain
held until the owner explicitly authorizes the PKG-08/shared paths needed to
establish one canonical report-package wire adapter.

## Observed contracts

- Five stable runner verbs, including `export-results`, are mapped in
  `core/runner/headless/src/bin/openpipestress-runner.rs`.
- Only `export-results` retains
  `HEADLESS_RUNNER_OPERATION_STUB_REQUIRES_DOWNSTREAM_PAYLOAD`.
- Benchmark/regression payloads use their independent accepted binding and are
  not integration inputs.
- DEC-065 output is controlled structured JSON on stdout and, only when
  explicitly named, the same JSON in `--output`; no direct `.opsproj` save
  exists.
- `core/reporting/report_package/src/lib.rs` owns the deterministic package
  input/outcome, six-member container, manifest/member hashes, blocking state,
  and professional-boundary notices. It writes no file.
- The accepted owned wire DTO and conversion currently live only in
  `apps/desktop/src-tauri/src/report_package_bridge.rs`, including exact linked
  solver identity, lowercase SHA-256, enums, dimensions, and rule-pack
  boundary checks.

## Canonical-adapter conclusion

Depending on the Tauri desktop crate is invalid layering. Duplicating its
approximately 742-line mapping in the runner would create two wire authorities.

Extract the existing mapping without semantic change to
`core/reporting/report_package/src/wire.rs`, using:

```rust
pub struct LinkedSolverIdentity<'a> {
    pub solver_name: &'a str,
    pub solver_version: &'a str,
    pub solver_build_ref: &'a str,
}

pub fn assemble_wire_request(
    request: ReportPackageRequest,
    linked_solver: &LinkedSolverIdentity<'_>,
) -> Result<ReportPackageContainerOutcome, String>;
```

Desktop and runner each bind identity from the linked product-physics crate and
call the same adapter. No report-package member, assembler, or public schema
changes.

## Selected runner shape

Input adds the verb-named sibling `export_results`, whose value has the exact
accepted `ReportPackageRequest` wire shape.

Successful structured output adds a bounded `report_package` projection:

- container file name and document kind;
- package identity and container hash;
- exact `Vec<u8>` container bytes in serde's existing JSON array form;
- member role, name, media type, hash basis, hash, length, and canonical
  identity flag.

It omits recursive renderer internals and duplicate member bytes. A summary
alone is not a usable runner home; base64 would invent a new encoding contract;
the bounded projection preserves exact usable bytes and manifest evidence.

## Fail-closed binding

Before assembly, validate request/package/result agreement for model,
input-manifest, unit-system, load-basis, audit-manifest, and run/source-basis
references. Preserve request validation and CLI-operation matching.

Use stable diagnostics:

- `HEADLESS_RUNNER_EXPORT_RESULTS_PAYLOAD_MISSING`;
- `HEADLESS_RUNNER_EXPORT_RESULTS_BINDING_MISMATCH`;
- `HEADLESS_RUNNER_EXPORT_RESULTS_PAYLOAD_INVALID`;
- `HEADLESS_RUNNER_EXPORT_RESULTS_PACKAGE_BLOCKED`.

Every failure exits 1, exposes no `report_package`, and writes no output file.
Producer-blocked bytes are never returned.

## Redaction and persistence

Add `report_package` to the existing local-private runner classification. The
existing controlled output remains the only stdout/file exposure:

- no intent: block exposure and file output;
- explicit intent: local/private warning posture;
- no new context, action, reason, or precedence vocabulary.

Desktop remains the owner of picker and atomic `.opsproj` persistence. Runner
`--output` remains non-atomic controlled JSON output under DEC-065; it never
selects a destination, silently writes the producer file name, or claims
atomicity.

## Candidate matrix and checks

The exact proposed matrix is materialized at the run root. It includes three
owner-visible PKG-08/shared paths:

- `core/reporting/report_package/src/wire.rs`;
- `core/reporting/report_package/src/lib.rs`;
- `apps/desktop/src-tauri/src/report_package_bridge.rs`.

It otherwise limits product work to the runner Cargo dependency, CLI branch,
redaction classifier, contract tests, and invented witnesses. Schemas,
assembler/member behavior, desktop TypeScript/menu/picker/save/atomic paths,
other runner verbs, benchmark/regression bindings, and PKG-08 state remain
excluded.

The affected registered union is all six checks: desktop test/build, piping
pytest, evidence sweep, harness pytest, and harness self-check. Focused evidence
must add Rust/report/runner tests, native subprocess and ZIP/hash proof,
packaged-native desktop save regression proof, claims/path/JSON/receipt/
containment/diff checks, exactly one terminal sweep, and fresh N5.

## Required owner-visible condition

Explicitly authorize the three PKG-08/shared paths solely for
behavior-preserving wire-adapter extraction and DEL-10-05 consumption.
Without that authorization the candidate is `BLOCK`.

