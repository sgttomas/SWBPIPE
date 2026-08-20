# MEMORY - DEL-02-04 Plugin and extension domain contracts

## Session 2026-05-01

Human project authority authorized one bounded DAG item of ORCHESTRATOR's
choosing. ORCHESTRATOR selected `DEL-02-04` to complete the remaining PKG-02
extension/API boundary foundation after `DEL-02-01`, `DEL-02-02`,
`DEL-02-03`, and `DEL-02-05`.

### Scope Used

- Sealed dispatch brief:
  `execution/_Coordination/DEV-001_DISPATCH_DEL-02-04.md`.
- Active upstream dependencies from approved `DAG-001` only.
- Candidate edges excluded.
- No lifecycle transition, dependency-register edit, or blocker queue refresh.

### Decisions And Constraints

- Plugin manifests are denied by default; requests are not runtime grants.
- Entrypoints must declare both extension point and domain surface.
- Domain/API compatibility must carry the DEL-02-04 domain contract,
  `api/api_boundary_contract.yaml`, JSON Schema 2020-12 basis, and host API
  status.
- No-bypass controls now explicitly cover analysis-boundary state,
  persistence, and external human-acceptance boundaries.
- Privacy posture must preserve local-first behavior, telemetry off by default,
  export permission checks, and redaction support.
- Sandbox posture must declare capabilities and deny arbitrary code execution,
  filesystem access, network access, and process spawning by default.

### Remaining TBDs

- Runtime plugin loader and isolation technology.
- Permission grant storage, consent UX, revocation, signing, and update
  mechanism.
- Public API transport and concrete external import/export formats.
- Rule expression grammar and sandbox implementation details.
- Canonicalization edge cases for non-JSON payloads.
- CI gates for plugin submission, protected-content screening, and
  security/privacy review.

## 2026-05-11 TP-RECON-01 Reconciliation

TP-RECON-01 reconciled the DEL-02-04 history from the assigned source bundle:
the dispatch-matrix row, archived DEV-001 implementation evidence rows,
REV05 lifecycle snapshot, archived DEV-001 DEL-02-04 dispatch brief, SCA-002
inventory/reconciliation request, commit metadata for `ef44f4c`, and the
current deliverable-local memory, status, and run records.

### Reconciled Evidence

- Historical DEV-001 evidence maps DEL-02-04 to committed bounded item commit
  `ef44f4c` (`schema: tighten plugin extension contract`) on 2026-04-30, with
  handoff commit `a37a0a1`. The REV05 evidence-status row preserves DEL-02-04
  as `API_CONTRACT`, `SOW-038`, and `OBJ-009`, while noting that completeness
  still depends on refreshed graph/context review.
- Commit metadata for `ef44f4c` shows changes to `docs/SPEC.md`,
  `docs/TYPES.md`, `docs/architecture/extension_domain_contracts.md`,
  `schemas/plugin_manifest.schema.yaml`, `tests/test_plugin_manifest_schema.py`,
  this deliverable memory file, the DEL-02-04 dispatch brief, and coordination
  state.
- The archived dispatch brief bounded the implementation to plugin manifest
  contract tightening, extension-domain documentation, public contract docs,
  focused manifest-schema tests, and evidence records. It explicitly excluded
  lifecycle transitions, candidate-edge changes, dependency-register edits, and
  blocker-queue refresh.
- The implemented contract evidence centers on JSON Schema 2020-12 manifest
  strictness, entrypoint declarations for extension point and domain surface,
  denied-by-default permissions, sandbox posture, provenance/privacy controls,
  no-bypass constraints, diagnostics, checksums, persistence and
  analysis-boundary state preservation, report controls, solver boundaries, and
  external human-gate boundaries.
- Deliverable-local run records show successful document, semantic-matrix,
  lens-register, and dependency-extract passes, with remaining human-ruling
  items kept as `TBD`. No tests were rerun during this reconciliation; the
  historical commit evidence includes the focused plugin manifest schema test
  file as a changed artifact.

### Deferred Scope And Boundaries

- Current lifecycle is preserved as `CHECKING`, consistent with the current
  status file and REV05 lifecycle snapshot.
- Remaining implementation decisions stay deferred: runtime plugin loader and
  isolation, permission grants and revocation, signing/update flow, public API
  transport, concrete import/export formats, rule expression grammar and
  sandbox details, canonicalization for non-JSON payloads, and CI/security
  gates.
- This entry records historical implementation evidence only. It does not
  create a release gate decision, engineering reliance decision, or standards
  compliance conclusion.

## 2026-05-16 PKG-02 Foundation-Slice Hardening

Scope executed:

- Converted `tests/test_plugin_manifest_schema.py` from script-only assertions
  into pytest-collected contract and fixture tests while preserving direct
  `python3` execution.
- Added `fixtures/plugin_manifest/invented_manifest_no_bypass.json` as invented
  public evidence for denied-by-default permissions, sandbox declaration,
  telemetry-off privacy posture, no-bypass controls, provenance, checksums, and
  declared domain surfaces.

Evidence:

- Pytest now collects and passes `tests/test_plugin_manifest_schema.py`.
- Direct execution with `python3 tests/test_plugin_manifest_schema.py` passes.
- Fixture checks verify that plugin manifests grant nothing by themselves and
  cannot bypass units, provenance, privacy, rule sandboxing, analysis-boundary,
  persistence, schema validation, diagnostics, checksums, protected-content,
  report, solver, or human-acceptance controls.

Boundaries preserved:

- No plugin loader, signing flow, permission store, public transport, import/
  export format, network capability, process capability, or runtime sandbox was
  implemented.
- Lifecycle remains `IN_PROGRESS`; no lifecycle transition was made.
- No dependency register, DAG, blocker queue, or candidate-edge edits.

## 2026-06-03 - TP-CODE-EVIDENCE-MIGRATION-RECONCILIATION-001 CHECKING transition
- Human approval accepted non-resolving DEV-001 evidence commits as migration-caused aberrations and approved lifecycle advancement to `CHECKING` for formal review.
- Evidence basis: `TP-CODE-EVIDENCE-AUDIT-001_2026-06-03` found current source/schema/fixture/test evidence and passing targeted/full-gate checks; `TP-CODE-EVIDENCE-MIGRATION-RECONCILIATION-001_2026-06-03` reconciled the migration-era commit-pointer gap.
- Local `_STATUS.md`, DEV-001 blocker queue lifecycle displays, and DAG-005 deliverable display surfaces were aligned to `CHECKING` where applicable.
- Boundary preserved: this is review-readiness only; no `ISSUED`, release-readiness, external compatibility, code-compliance, protected-IP/private-data, or professional-engineering authentication claim is made.

## 2026-06-04 - TP-AUTHORITY-REFRESH-0_7-DAG006

- WORKING_ITEMS/TASK workforce current-authority refresh applied to active deliverable-local surfaces for `DEL-02-04`.
- Current authority basis is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision `0.7` plus approved `execution/_DAG/DAG-006/` active graph authority.
- Historical run records, historical DAG row IDs, review dispositions, lifecycle `_STATUS.md`, aggregate DAG artifacts, candidate edges, repo-level governance files, schemas, code, and tools were intentionally not changed by this refresh.
- Preserved historical references remain evidence of earlier work, not current authority claims.

## 2026-06-17 - Lifecycle Housekeeping

- Housekeeping lifecycle reset: `_STATUS.md` current state set to `IN_PROGRESS` to reflect current code development in progress. This does not change review, issuance, release readiness, professional approval, certification, sealing, authentication, or code-compliance status.
## 2026-07-12 - D-41 R5 T6 PDU-037 verification refresh

- Plugin schema/no-bypass fixture verification refreshed within the 19/19 Python set. Unit-safety, provenance, diagnostic, protected-content, and runtime adapter/plugin regression layers remain downstream.
- Evidence: `_run_records/WORKING_ITEMS_RUN_2026-07-12_D41-R5-T6-PDU037.md`. Lifecycle remains `IN_PROGRESS`; the D-41 bootstrap remains for T7.

## 2026-08-20 - R6 N1 executable adapter/plugin verification

- Added a pure in-memory adapter/plugin composition verifier that executes the
  caller-loaded canonical plugin schema, caller-loaded unit catalog and
  quantity evidence, provenance/privacy/no-bypass checks, protected-content
  quarantine, complete diagnostic envelopes, and existing adapter declaration
  validation without selecting or dispatching a runtime.
- Unit evidence fails closed for missing/malformed/nonfinite values,
  noncanonical or incompatible dimensions/units, malformed/non-cleared
  provenance, and protected/quarantined content.
- Adapter, plugin, quantity, and operation-result diagnostics preserve the exact
  source, affected object, class, and provenance. Malformed nested adapter
  capabilities no longer raise or mask a simultaneous quarantine marker.
- Integrated-review Amendment 1 removed `build_result`'s invented/public
  top-level defaults. Composed results now derive privacy and provenance from
  caller manifest, quantity, adapter-result, and adapter-declaration evidence:
  protected dominates private, incomplete/non-cleared evidence remains
  review-required, and public-reviewed requires complete canonical clearance.
  Required manifest/adapter privacy fields fail closed, while positive private/
  protected markers retain precedence through unrelated incompleteness.
- Evidence: focused plus existing adapter/plugin suites `107 passed in 0.47s`;
  composed result envelopes conform to the canonical adapter operation-result
  schema across public/private/protected/incomplete cases; diff and containment
  checks passed; V16 fresh read-only review covered all 3,930 frozen lines and
  returned PASS with zero actionable findings. The integrated-review attempt 1
  finding and V13–V15 review findings are closed.
- The exact PDU-037 Remaining item is closed. Lifecycle remains `IN_PROGRESS`.
  Runtime loader/isolation, transport, capability grant, and permission
  persistence choices remain separately governed and dispatch remains blocked.

### Integrated-review Amendment 2

- Caller-supplied plugin schema evidence is serialized once in memory with a
  deterministic canonical JSON form. Those exact bytes must match the pinned
  canonical plugin-manifest SHA-256, are parsed into a plain snapshot, and only
  that snapshot is used for identity and complete schema evaluation.
- Lookalikes that remove checksum/professional-boundary requirements, alter
  definitions, or expose hostile Mapping accessors fail closed before manifest
  verification. No schema file or runtime is loaded by the verifier.
- Evidence: complete focused/existing suite `112 passed in 0.79s`; composed
  schema, containment, and diff checks PASS; V18 fresh review covered all 4,038
  frozen lines and returned PASS with zero actionable findings. Integrated
  review v2 and V17 findings are closed.
- Any authorized future canonical plugin-schema revision must deliberately
  update the pinned fingerprint and rerun the weakened/hostile-schema suite and
  fresh full-diff review.

### Integrated-review Amendment 3

- Adapter declaration capabilities now require an exact list/JSON-array shape,
  exact plain-string items from the canonical `AdapterCapability` enum, and at
  least one operational capability required by the schema's `contains` rule.
- Strings and other non-list iterables, unknown or mixed tokens, non-string and
  nested/unhashable values, and hostile unhashable string subclasses fail
  closed without raising. Canonical duplicate items remain accepted because
  the schema has no uniqueness requirement.
- Capability defects do not mask protected/quarantined adapter provenance;
  direct declaration validation and public composed verification retain both
  quarantine marker forms and keep runtime dispatch false.
- Evidence: complete focused/existing suite `129 passed in 0.50s`; composed
  schema, containment, and diff checks PASS; V20 fresh review matched all nine
  hashes/line counts, covered the complete 4,230-line frozen set and full diff,
  and returned PASS with zero findings. Integrated review v3 and V19 findings
  are closed.

### Integrated-review Amendment 4

- Complete canonical adapter declaration and operation-result provenance now
  requires cleared redistribution (`public_permissive` or `private_only`) and
  accepted review. Canonical unresolved/rejected postures emit blocking
  `ADAPTER_PROVENANCE_NOT_CLEARED`; protected privacy emits quarantine.
- Provenance fields require exact nonblank plain strings. Positive protected/
  quarantined markers retain precedence, and diagnostic normalization plus
  composed boundary ranking cannot hash or compare hostile subclasses.
- Caller manifest evidence receives iterative exact-type, cycle-aware,
  exception-contained raw-JSON preflight with deterministic depth 512 before
  strict one-time serialization. The detached snapshot alone drives schema,
  manifest semantics, diagnostic context, and envelope boundaries; caller
  evidence is not mutated or invented.
- Direct and composed tests cover declaration/result provenance and privacy,
  malformed capabilities, hostile equality/hash/container behavior, deep and
  cyclic structures, nonfinite values, exact diagnostic routing, protected
  envelope dominance, canonical result schema, and runtime non-dispatch.
- Evidence: complete suite `190 passed in 0.51s`; composed schema, containment,
  and diff checks PASS; V26 fresh review matched all fifteen hashes/line counts,
  covered the complete 5,563-line frozen set and full diff, and returned PASS
  with zero findings. Integrated review v4 and V21–V25 findings are closed.

### Integrated-review Amendment 5

- Every caller manifest, adapter declaration/result, unit catalog, unit
  evidence, and plugin schema is converted into a bounded detached exact-JSON
  snapshot before validation, traversal, lookup, diagnostics, boundary ranking,
  hashing, or schema evaluation. Hostile accessors, subclasses, cycles, depth,
  node/text/serialized-byte overflow, and nonfinite values fail closed.
- Safely observable exact protected/quarantined markers retain precedence when
  unrelated structure is malformed. Fallback marker inspection and capability
  inspection are explicitly width/key bounded; raw hostile keys are never
  compared, and all diagnostic paths/IDs are canonical and byte bounded.
- Malformed manifest fallback uses plugin-specific resource limits; caller
  plugin schema evidence is bounded and fully exception-contained before the
  canonical fingerprint check. No input is mutated or loaded at runtime.
- Direct/composed regressions cover adversarial keys, hostile/deep/cyclic/
  nonfinite/oversized schemas and evidence, marker boundaries, overflowed IDs,
  quarantine precedence, canonical envelopes, and runtime non-dispatch.
- Evidence: complete suite `306 passed in 0.76s`; composed schema, containment,
  and diff checks PASS; V31 matched all 20 hashes/line counts, reviewed the full
  8,082-line frozen set and original-basis amended diff, and returned PASS with
  zero findings. Integrated review v5 and V27–V30 findings are closed.
- Exact PDU-037 Remaining stays closed. Runtime loader/isolation, transport,
  capability grant, and permission persistence remain owner-held and dispatch
  remains blocked.
