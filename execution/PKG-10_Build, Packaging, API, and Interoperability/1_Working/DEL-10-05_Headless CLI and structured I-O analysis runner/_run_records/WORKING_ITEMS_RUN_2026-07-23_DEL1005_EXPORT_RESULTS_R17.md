# WORKING_ITEMS RUN — 2026-07-23 — DEL-10-05 Export-Results Report-Package Binding (R17)

**Run:** `HELP-HUMAN-PIPING-20260723-DEL1005-EXPORT-RESULTS-R17`  
**Package / deliverable:** `PKG-10` / `DEL-10-05`  
**Frozen basis:** `1f2ecc1d06375c01a409041b8380e4d65b2a9f9a` on
`codex/piping-pkg10-export-results`  
**Active dependency basis:** owner-accepted DAG-008; all ten active
DEL-10-05 execution-upstream edges recorded `SATISFIED` at activation  
**Candidate:** `CB-2026-07-23-DEL-10-05-EXPORT-RESULTS-001`, owner-adopted v2
plus the sealed owner-authorized v3 remediation  
**Disposition:** `SUCCESS` for the bounded implementation-and-verification
tranche. Fresh N5-V3 returned `COMMIT-SAFE`; this record does not perform or
claim a merge, lifecycle transition, stage change, release, issuance,
publication, professional acceptance, certification, sealing, authentication,
or code-compliance determination.

## 1. Authority and orchestration

HELP_HUMAN supervised WORKING_ITEMS for the single activated package. The
managed run preserves reconnaissance, the v1 refutation, aggregate-redaction
amendment v2, fresh refutation, owner adoption, sole serialized N4
implementation, initial N5 verification, sealed v3 remediation, replacement
sweep, and fresh N5-V3 verification:

`execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260723-DEL1005-EXPORT-RESULTS-R17/`

The exact v2 owner adoption authorized the bounded PKG-08 shared wire-adapter
extraction and desktop wrapper, the PKG-10 runner binding, aggregate
`$.report_package` local-private classification, N4 as sole implementation
owner, and N5-before-W3 gating. It preserved runner `--output` as controlled
structured JSON and excluded a desktop picker/atomic-save route, package
member/schema change, runner-verb change, and all other scope changes.

Initial N5 reproduced one defect: malformed but present `export_results`
content failed in the outer CLI parser with the generic CLI-input diagnostic
instead of the adopted operation-specific invalid-payload diagnostic. The
owner-authorized v3 remediation was limited to the runner binary source,
focused Python contract test, managed evidence, and exactly one replacement
DEC-025 sweep. No other semantic or write-scope change was authorized.

## 2. Changed surfaces and implemented behavior

Within the adopted 24-row write matrix, the implementation:

- extracted the existing report-package wire DTO/conversions to
  `core/reporting/report_package/src/wire.rs` and exported that module from
  `core/reporting/report_package/src/lib.rs`;
- reduced
  `apps/desktop/src-tauri/src/report_package_bridge.rs` to a thin compatibility
  wrapper over the shared adapter, with caller-supplied linked solver identity;
- added the existing report-package crate as a runner path dependency and
  bound only the existing `export-results` operation in
  `core/runner/headless/src/bin/openpipestress-runner.rs`;
- added the exact aggregate `$.report_package` local-private behavior in
  `core/runner/headless/src/redaction_binding.rs`;
- extended `tests/test_headless_runner_contract.py` with focused subprocess,
  binding, diagnostics, redaction, persistence, determinism, ZIP/hash/member,
  and malformed-present-payload checks; and
- added the deterministic invented `del1005_export_results_*` input generator,
  four input fixtures, and five controlled generated witnesses.

The runner validates the request/model, input-manifest, unit-system, load-basis,
audit-manifest, result-envelope, and run-reference cross-bindings before
calling the shared adapter. Successful explicit-intent execution returns one
bounded `report_package` projection: the exact container bytes, file/document
identity, package/container hashes, and six-member summary. Missing payload,
cross-binding mismatch, invalid DTO/conversion, producer-blocked output, and
missing explicit intent fail closed without a report-package payload or output
file.

At the exact serialized path `$.report_package`, the existing
`local_private` controller classifies the complete subtree once rather than
descending through the byte array. No explicit intent produces one blocking
decision/finding and no file; explicit intent produces one warning
decision/finding and retains the exact unmodified subtree. Decision/finding
cardinality remains constant for both the tracked 29,126-byte package and the
transient 3,229,152-byte native-size package.

Runner `--output` continues to write only the caller-named controlled
structured-JSON result that is JSON-equivalent to stdout. It does not invoke,
copy, or claim the desktop `.opsproj` picker or atomic save. The ignored local
runner `Cargo.lock` is only the matrix-authorized resolver consequence and is
not a tracked tranche output.

## 3. N5 block and sealed v3 remediation

Initial N5 verified the shared extraction, cross-bindings, package projection,
aggregate redaction, witnesses, native proof, collateral preservation,
containment, and initial passing sweep, but returned `BLOCK` on malformed
present payload routing. Removing required `package_id` caused exit 2 with
`HEADLESS_RUNNER_CLI_INPUT_JSON_INVALID`.

Under the sealed v3 authorization, N4 changed only:

1. `core/runner/headless/src/bin/openpipestress-runner.rs`; and
2. `tests/test_headless_runner_contract.py`.

A present `export_results` value, including JSON `null`, now remains raw JSON
until the operation handler. DTO deserialization failures return exit 1 with
`HEADLESS_RUNNER_EXPORT_RESULTS_PAYLOAD_INVALID`, a message beginning with the
existing `REPORT-PACKAGE-WIRE-INCOMPLETE` prefix, no report-package payload,
and no output file. Missing-required-field, malformed-field-type, and
explicit-null cases prove the correction; an absent member still returns the
separate payload-missing diagnostic.

After the replacement full gates and sweep, fresh N5-V3 independently returned
`COMMIT-SAFE` with zero blocking and zero tranche-local non-blocking findings.
It verified that all 56 dirty leaf paths mapped to the adopted write matrix,
the v3 delta was exactly the two authorized files, and no implementation,
test, schema, Cargo, or witness file changed after the replacement sweep.

## 4. Verification evidence

Final focused results:

| Check | Result |
|---|---|
| Report-package formatting and tests | PASS; 16/16 |
| Runner formatting and tests | PASS; 46/46 |
| Focused Python headless-runner contract | PASS; 17/17 |
| Desktop Vitest | PASS; 516/516 |
| Desktop production build | PASS |
| Piping pytest | PASS; 548 |
| Practitioner-harness pytest | PASS; 311 |
| Repository self-check | PASS |
| Claims-language validator | PASS |
| Path-anchor validator | PASS |
| Piping receipt validator | PASS |
| R17 JSON/JSONL parsing | PASS |
| Exact write-matrix containment | PASS |
| `git diff --check` and staged-diff checks | PASS; no staged paths |

Native runner proof reproduced deterministic repeated package bytes, the exact
container hash, a valid six-member ZIP, member byte lengths and hashes, failure
paths with no output file, one caller-named controlled JSON output on success,
and one aggregate decision/finding at both tracked and native sizes. The
tracked container is 29,126 bytes with SHA-256
`8abef0f82ce8be9a1ce5b0c427d66951fdf81f984b0854e524cbe6209bc062ab`.

Packaged-native macOS proof preserved the desktop route after the wire-adapter
extraction: mechanics reached `MECHANICS_SOLVED`; no-intent save blocked;
picker cancel wrote zero bytes; new-save and same-path replace produced a valid
six-member package; replacement preserved bytes/hash/identity, changed inode,
and left no atomic temporary-file residue.

The full desktop Rust suite retained one frozen-basis residual:
`validate_rule_pack_command_reports_example_pack_clean_and_draft_findings`
still expects stale `No professional` notice text. The result was 74/75. The
test and governing source are byte-unchanged from the frozen basis, the
residual is outside this tranche, and no waiver or repair is represented.

## 5. DEC-025 sweep identities

Both owner-authorized R17 sweep artifacts report PASS:

1. Initial v2 sweep:
   `validation/evidence/sweeps/SWEEP_20260723T093917Z_1f2ecc1d0637-dirty.json`  
   SHA-256:
   `948ef9ee35f620cadc70685d9cf86efebc95e338ee37ef19644621f7712a8879`
2. Sealed-v3 replacement sweep:
   `validation/evidence/sweeps/SWEEP_20260723T100430Z_1f2ecc1d0637-dirty.json`  
   SHA-256:
   `ee50a166142468752b3e84631dc3568e1116918ab79610dacee22cf40026f3d1`

The second artifact is the exactly one v3 replacement sweep authorized after
the two-file remediation. No DEC-025 invocation occurred during W3 closeout.

## 6. Closeout and preserved boundaries

After fresh N5-V3 returned `COMMIT-SAFE`, W3 removed only the completed
`export-results`/report-package bullet from `_STATUS.md`, left the
`## Remaining` section present and empty, kept lifecycle `IN_PROGRESS`, and
updated the bounded History and MEMORY evidence plus this run record.

Preserved boundaries:

- no report-package member, schema, identity, hash basis, or encoding change;
- no runner verb, operation mapping, benchmark/regression behavior, suite
  policy, or threshold change;
- no desktop TypeScript, menu, picker, save service, or atomic-writer change;
- no source-fixture mutation, external data, public/private-data clearance, or
  portable/public exposure claim;
- no decomposition, dependency, ownership, lifecycle, stage, release,
  issuance, publication, merge, professional-acceptance, certification,
  sealing, authentication, or code-compliance effect.

Git staging, commit, push, pull request, CI, merge, receipt append, and
post-landing loop restart remain outside this W3 record and route separately
through CHANGE/HELP_HUMAN.
