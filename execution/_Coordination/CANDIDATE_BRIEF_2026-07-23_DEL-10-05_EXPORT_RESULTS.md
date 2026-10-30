---
doc_id: CB-2026-07-23-DEL-10-05-EXPORT-RESULTS-001
doc_kind: coordination.candidate_brief
status: adopted_v2_execution_released
prepared: 2026-07-23
version: 2
package_id: PKG-10
deliverable_id: DEL-10-05
decision_basis: DAG-008, DEC-025, DEC-065, DEC-081, accepted DEL-08-01 report-package seam
---

# CANDIDATE Brief — DEL-10-05 export-results report-package binding

**Status:** `ADOPTED v2 — N4 EXECUTION RELEASED`

**Prepared by:** WORKING_ITEMS `W-PKG10` for HELP_HUMAN

**Managed run:** `HELP-HUMAN-PIPING-20260723-DEL1005-EXPORT-RESULTS-R17`

## 1. Purpose and accepted basis

Replace only the current `openpipestress-runner export-results`
`HEADLESS_RUNNER_OPERATION_STUB_REQUIRES_DOWNSTREAM_PAYLOAD` branch. The bound
operation consumes the accepted DEL-08-01 report-package wire contract and
returns an exact deterministic package payload inside the runner's existing
DEC-065 controlled structured-JSON output.

Frozen execution basis:

- Git SHA `1f2ecc1d06375c01a409041b8380e4d65b2a9f9a`;
- branch `codex/piping-pkg10-export-results`;
- active, approved, owner-accepted DAG-008;
- DEL-10-05 `IN_PROGRESS`, with exactly one Remaining row: this seam;
- all ten active DEL-10-05 execution-upstream edges `SATISFIED` in both
  DAG-008 and the deliverable-local register;
- DEL-08-01 report-package seam accepted and merged through PR #316;
- current `software-workflow.json` under
  `docs/SOFTWARE_WORKFLOW_PROFILE.md`.

The standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

## 2. Exact in-scope outcome

### 2.1 One shared accepted wire adapter

Move the existing owned DTOs and checked conversions from the desktop-only
bridge into:

`core/reporting/report_package/src/wire.rs`

The shared API is:

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

The extraction is behavior-preserving. It must:

- retain the exact accepted `ReportPackageRequest` shape;
- reject missing or `TBD` required wire values;
- require bare lowercase 64-hex wherever SHA-256 is declared;
- preserve every audit/result/reference/provenance/status/family/dimension
  conversion;
- validate both audit and result-envelope solver identities against the
  caller-supplied linked identity;
- preserve the current active-rule-pack-unavailable boundary;
- call the unchanged `assemble_report_package_container`.

`core/reporting/report_package/src/lib.rs` only exports the wire module.
The desktop bridge becomes a thin compatibility wrapper that derives linked
identity from `open_pipe_stress_product_physics` and delegates to the shared
adapter. Its Tauri command shape and behavior remain unchanged.

The runner independently derives the same linked identity and calls the same
adapter. PKG-08 retains ownership of the adapter and desktop wrapper; shared use
does not transfer ownership to PKG-10.

### 2.2 Runner input and output

Add exactly one optional verb-named wrapper member beside the existing
`request`, `solve`, `benchmark`, and `regression` members:

```json
{
  "request": {
    "operation": "export_results"
  },
  "export_results": {
    "...": "accepted ReportPackageRequest DTO"
  }
}
```

No runner verb or `RunnerOperation` value changes.

On success, add `report_package` to the CLI output only for this operation.
It is omitted for every other operation and every blocked result. Its bounded
projection contains:

- `container_file_name`;
- `document_kind`;
- `package_identity_sha256_hex`;
- `container_sha256_hex`;
- exact `container_bytes` using serde's existing `Vec<u8>` JSON-array form;
- members with `role`, `file_name`, `media_type`, `hash_basis`,
  `sha256_hex`, `byte_length`, and `carries_canonical_identity`.

This is deliberately not the entire recursive producer outcome: it omits
renderer internals and duplicate member bytes. A summary alone would not
provide a usable package payload, while base64 would invent a new encoding
contract. The bounded projection preserves exact package bytes, identity, and
manifest/member evidence without changing the package or its members.

## 3. Binding and fail-closed predicates

Before calling the shared wire adapter:

1. existing request validation and CLI-operation matching pass;
2. runner `model_ref` equals the package source-model ref, the audit model-hash
   payload ref, every result-envelope model ref, and every result-envelope
   reproducibility model-hash payload ref;
3. runner `input_manifest_ref.ref_id` equals the audit manifest
   input-manifest-hash payload ref;
4. runner `unit_system_ref.ref_id` equals the audit and every result-envelope
   unit-system ref;
5. runner load-basis refs equal each result envelope's load-basis set and occur
   among package source-basis refs;
6. every result-envelope audit-manifest ref equals the supplied audit manifest
   ID;
7. every result-envelope run ref occurs among package source-basis refs.

Reference comparisons are exact and order-insensitive only where the contract
defines a set. No missing, duplicate, mismatched, or `TBD` reference is silently
accepted.

Stable diagnostics:

| Condition | Diagnostic | Exit |
|---|---|---|
| missing `export_results` | `HEADLESS_RUNNER_EXPORT_RESULTS_PAYLOAD_MISSING` | 1 |
| cross-binding mismatch | `HEADLESS_RUNNER_EXPORT_RESULTS_BINDING_MISMATCH` | 1 |
| DTO/conversion failure | `HEADLESS_RUNNER_EXPORT_RESULTS_PAYLOAD_INVALID` | 1 |
| producer `export_blocked` | `HEADLESS_RUNNER_EXPORT_RESULTS_PACKAGE_BLOCKED` | 1 |

The conversion-failure message preserves the underlying stable
`REPORT-PACKAGE-*` code. Any failure returns no `report_package` payload and
creates no output file. Producer-blocked container/member bytes are withheld.

Exit 0 is possible only when request validation, binding, conversion, producer
validation, and exposure control all pass.

## 4. Redaction, persistence, and no-bypass boundary

Add one bounded aggregate case to the existing Rust runner's fixed
`local_private` controller. At the exact serialized CLI path
`$.report_package`, classify the entire subtree once and do not descend into
its scalars. No new context, action, reason, precedence, corpus case, route, or
caller-selected setting is introduced.

The existing outer controlled-export envelope remains the sole stdout and file
exposure:

- without `--explicit-local-private-intent`, emit exactly one subtree
  `block_export` decision/finding with
  `LOCAL_PRIVATE_INTENT_REQUIRED`, set the controlled payload to null, and
  write no file;
- with explicit intent, emit exactly one subtree `warning_only`
  decision/finding with `PRIVATE_LOCAL_ALLOWED_WITH_WARNING` and retain the
  exact unmodified `report_package` subtree;
- source-carried intent is stripped and cannot satisfy the gate;
- the controlled payload, decisions, findings, blocked state, and summary are
  emitted before any downstream caller can access package bytes.

This aggregate case is required for bounded behavior: the accepted
packaged-native report package is 3,189,621 bytes, and scalar descent would
produce millions of decisions and findings. Every existing non-package path
keeps its prior scalar behavior.

Desktop and runner persistence remain different contracts:

- desktop owns its user picker and caller-owned atomic `.opsproj` save;
- runner `--output` remains the existing explicit structured-JSON copy of
  stdout under DEC-065;
- runner does not treat `--output` as an `.opsproj` destination, invoke or copy
  the desktop atomic writer, choose a path, silently write the producer's
  suggested file name, or claim atomicity.

A success test must prove that an explicitly named `runner-result.json` is the
only created file and is JSON-equivalent to stdout.

## 5. No-collateral-behavior requirements

- All five runner verbs and operation mappings remain unchanged.
- Existing `solve`, `validate-input`, `run-benchmark`, and `run-regression`
  output bytes and exit behavior remain unchanged for the frozen fixtures.
- `benchmark_binding.rs`, suite inventories, comparison predicates,
  thresholds, claim posture, and R12 witnesses remain byte-identical.
- The headless request/result public schema remains unchanged; verb wrapper
  payloads continue the existing `solve`/`benchmark`/`regression` precedent.
- Report-package assembler behavior, package/member vocabulary, schema version,
  hashes, professional notices, and producer blocking semantics remain
  unchanged.
- Desktop TypeScript request construction, redaction projection, menu, picker,
  save service, and atomic writer remain unchanged.
- Input fixture/source bytes are not mutated.

## 6. Serialized implementation tasks

After exact owner adoption only, N4 is the sole serialized implementation
owner:

1. Freeze hashes for the desktop bridge, report-package producer/tests, runner
   verbs, R12 benchmark/regression fixtures/witnesses, schemas, and excluded
   desktop save paths.
2. Extract the shared wire adapter and make the desktop wrapper delegate to it.
3. Add the runner's report-package path dependency and `export_results`
   binding with exact cross-binding and diagnostics.
4. Add the bounded success projection and exact aggregate report-package
   local-private classification.
5. Add deterministic invented success, missing, binding-mismatch,
   producer-blocked, and no-intent witnesses.
6. Run the full pre-sweep gate union and packaged/native proofs.
7. Only after every preceding gate passes, run exactly one DEC-025 sweep.
8. Freeze the tree; perform no product/test/schema/Cargo edit after the sweep.
9. Dispatch fresh N5.

Any excluded-path need, schema/member change, output encoding change, runner
verb change, benchmark/regression delta, desktop persistence change, or
acceptance change returns for a versioned owner amendment.

## 7. Acceptance predicates

1. The exact shared wire adapter is the only owned DTO/conversion
   implementation used by desktop and runner.
2. Missing, invalid, mismatched, producer-blocked, and no-intent requests fail
   closed with the specified diagnostics, no package payload, and no output
   file.
3. Success exits 0 only after every gate in §3 and §4 passes.
4. Success carries exact package bytes and identity/hash/member evidence;
   independent ZIP and member-hash checks reproduce the accepted six-member
   package.
5. Two identical runs produce identical container bytes, container hash,
   package identity, and member summaries.
6. No-intent exposure produces exactly one blocking subtree decision and no
   file; explicit intent produces exactly one warning subtree decision and
   retains exact package bytes. Decision/finding cardinality is independent of
   byte length and is proven at a native-size payload of at least 3,189,621
   bytes.
7. No source request, fixture, package input, result, or manifest value is
   mutated.
8. Audit/result/manifest/provenance/unit/dimension/professional-boundary checks
   remain enforced.
9. Desktop bridge behavior remains equivalent, including packaged-native
   cancel, new-save, and replace proof.
10. Benchmark/regression behavior and existing witnesses remain byte-identical.
11. No schema, package member, runner verb, decomposition, dependency,
    lifecycle, stage, release, issuance, or publication change occurs.
12. Every changed path is contained by the exact adopted matrix.

## 8. Exact write fence

The exact proposal is:

`execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260723-DEL1005-EXPORT-RESULTS-R17/WRITE_MATRIX.csv`

The three PKG-08/shared rows are owner-visible and require explicit adoption.
No PKG-08 or DEL-08-01 status, memory, Remaining, lifecycle, or closeout effect
is authorized.

W3, if later released by fresh N5 `COMMIT-SAFE`, is limited to DEL-10-05
`_STATUS.md`, `MEMORY.md`, and one new bounded run record. CHANGE separately
owns receipt, Git, PR, CI, and merge closeout.

## 9. Verification union

Registered affected-path union:

- `desktop-test`;
- `desktop-build`;
- `piping-pytest`;
- `evidence-sweep`;
- `harness-pytest`;
- `harness-self-check`.

Focused and native evidence:

- `cargo fmt --check` for affected Rust crates;
- report-package crate tests, including wire conversion, linked identity,
  SHA-256, enums, dimensions, and producer blocking;
- runner library and both binary targets;
- focused Python runner contract tests;
- deterministic witness regeneration with clean diff;
- native runner success, deterministic repeat, independent ZIP/hash/member
  verification, missing/mismatch/producer-block/no-intent failures, native-size
  aggregate-redaction cardinality and byte-integrity proof, and JSON `--output`
  proof;
- packaged-native desktop report-package cancel/new-save/replace regression
  proof because its native adapter is refactored;
- frozen hash/behavior checks for R12 benchmark/regression and excluded schema/
  desktop persistence surfaces;
- claims language, path anchors, JSON parsing, receipt validation, exact scope
  containment, `git diff --check`, and staged-diff checks;
- exactly one terminal DEC-025 sweep;
- fresh independent N5 after that sweep.

No new Playwright workflow is required because no user-visible route changes.

## 10. Authority and closeout

This candidate does not self-adopt. It authorizes no implementation until:

1. fresh N3 returns `COMMIT-SAFE`; and
2. the human owner adopts this exact version and exact matrix, including the
   PKG-08/shared extraction.

Lifecycle, dependency, stage, release, issuance, publication, professional
acceptance, Git, PR, and merge acts remain separate.

### Exact owner adoption language

> I ADOPT v2 of `CB-2026-07-23-DEL-10-05-EXPORT-RESULTS-001` and explicitly
> authorize the bounded shared-integration scope in `WRITE_MATRIX.csv`,
> including the PKG-08-owned shared wire-adapter extraction and desktop
> wrapper and the exact aggregate `$.report_package` local-private
> classification, with N4 as sole serialized implementation owner, N5 as fresh
> verifier, and DEL-10-05-only W3 closeout only after N5 returns
> `COMMIT-SAFE`. Runner `--output` remains controlled structured JSON; no
> desktop picker/atomic-save, package member/schema, runner-verb, or other
> scope change is authorized.
