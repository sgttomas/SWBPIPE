---
doc_id: CB-2026-07-21-DEL-12-02-REDACTION-BREADTH-001
doc_kind: coordination.candidate_brief
status: adopted_v6_n4_authorized
prepared: 2026-07-21
package_id: PKG-12
deliverable_id: DEL-12-02
decision_basis: REXC-REQ-006, REXC-REQ-007, REXC-REQ-008, REXC-REQ-011, REXC-REQ-012, REXC-REQ-013, REXC-REQ-014, DEC-025, DEC-065, DEC-081, DEC-085, DEC-087
agent_classification: classify_eligible_pending_independent_refutation
rule_activation: not_activated
---

# CANDIDATE Brief — DEL-12-02 Redaction Breadth and No-Bypass Exposure Gates

**Status:** `ADOPTED V6 — N4 BOUNDED IMPLEMENTATION AUTHORIZED`  
**Review history:** v1 N3, v2 N3B, v3 N3C, and v4 N3D `BLOCK` returns are
preserved; v6 N3F returned `COMMIT-SAFE`.

**Prepared by:** WORKING_ITEMS for HELP_HUMAN  
**Managed run:** `HELP-HUMAN-PIPING-20260721-DEL1202-REDACTION-BREADTH-R15`  
**Frozen basis:** `0c066652cd527eb1559f715e914262d2bda42602`

This document proposes the single DEL-12-02 `_STATUS.md ## Remaining` item:
extend the existing redaction seam across the REXC-REQ-012 breadth. It does
not adopt itself, authorize code writes, or change lifecycle state. The owner
Gate-3 decision requested in §12 is required before N4 implementation.

## 1. Purpose and accepted basis

Bind every current model/report/result export exposure recorded in the frozen
`ROUTE_MATRIX.csv` through the existing DEL-12-02 redaction contract before
filesystem, stdout, browser download, DOM/iframe preview, print, process/UI,
or downstream-handoff exposure.

The executor resolves:

```text
REPO_ROOT=$(git rev-parse --show-toplevel)
WORKING_ROOT=${REPO_ROOT}/projects/chirality-piping
```

Accepted basis:

- root and project `AGENTS.md`; `agents/AGENT_WORKING_ITEMS.md`;
- `software-workflow.json` and the ratified software workflow profile;
- DEL-12-02 `ScopeOfWork.md`, `_STATUS.md`, `_CONTEXT.md`, `_REFERENCES.md`,
  `Dependencies.csv`, `MEMORY.md`, and the TP-E4 redaction run record;
- `core/security/redaction/controls.py`, its public exports, strict schema,
  shared parity corpus, Python tests, TypeScript mirror/tests, and security doc;
- approved DAG-007 and the current loop workplan/receipt cursor at execution;
- D-52/DEC-085 and D-54/DEC-087; DEC-025; DEC-065; DEC-081;
- the managed run's `ROUTE_MATRIX.csv`, `FAN_IN.md`, child returns, and
  `CURRENT_CANDIDATE_RATIONALE.md`.

## 2. Frozen route set and exclusions

The route matrix is normative for this tranche and contains:

- seven Python handoff/export writers plus the provider-neutral downstream
  workflow;
- both active headless-runner binaries: final-runner stdout/`--output` and
  compatibility-runner stdout;
- desktop report DOM/JSON/render-IPC/HTML/save/print/lint paths;
- every other frozen desktop `download` source, grouped without omitting any
  of the 33 source panels found by deterministic enumeration;
- in-process report/result constructors, classified as construction rather
  than final exposure and therefore gated at their callers;
- deny-only adapter and nonexistent plugin/bug-report routes, covered only by
  future-route regression assertions.
- the protected-content-lint diagnostic CLI, explicitly classified as a
  separately governed DEC-058/DEC-059 diagnostic rather than a product-export
  route and protected by a no-change/no-product-use assertion.

Explicitly out of scope:

- DEC-059 public-source release export, project persistence, and application
  binary packaging;
- a plugin loader/runtime, bug/crash-report feature, new runner verb, cloud or
  network transport, telemetry, public API transport, filesystem/storage-root
  choice, config persistence, external format redesign, or legal workflow;
- destructive quarantine movement, real private/protected data, secrets,
  professional acceptance, lifecycle/stage/release/issuance, or merge.

If execution discovers an exposure sink absent from the frozen matrix, it
must stop and return a brief amendment; it must not silently add scope.

## 3. One controlled-export behavior

All route bindings implement the same behavior, without changing the existing
action/context/reason vocabulary:

```text
ControlledExport<T> = {
  payload: T | null,
  decisions: RedactionDecision[],
  findings: RedactionFinding[],
  blocked: boolean,
  summary: RedactionSummary
}
```

The language-specific spelling may follow existing conventions, but these
members and semantics are fixed.

For every route:

1. Copy the source into a route-specific export representation; never mutate
   source models, libraries, rule packs, results, report inputs, or packages.
2. Project every value-bearing leaf into the existing explicit metadata
   vocabulary. Containers carry structure only; container metadata may not
   prevent traversal of nested leaves.
3. Remove/override all source-carried intent keys before calling the existing
   redaction contract (or parity-pinned language mirror), then supply only the
   route's fixed context and wrapper-owned explicit-intent input.
4. Materialize only the sanitized payload into the existing target schema or
   format and compose redaction findings with existing diagnostics.
5. Make decisions, findings, summary, and blocked state observable before the
   payload link, preview, print control, stdout value, file, or handoff exists.
6. If any finding is `BLOCKING`, set payload to `null`, return exit/failure
   status appropriate to the existing route, and perform no exposure side
   effect. A blocked internal diagnostic representation may remain in memory.
7. A local-private route may retain a **known private** value only when its
   wrapper-owned explicit intent is true; retained private values carry
   `warning_only` findings. Unknown values preserve the contract's existing
   local-private `warning_only` behavior without intent. They remain unknown
   and are neither reclassified as private nor declared public/releasable.

Projection rules are fixed:

- schema identifiers, versions, public-permissive checksums, neutral status
  vocabulary, and non-sensitive provenance summaries may be `public_metadata`
  only when explicit source metadata supports that classification;
- user project identity/values, result magnitudes, material/component fields,
  private rule details, owner/design-basis content, concrete paths, captured
  process content, and free metadata map to their existing private/path class;
- missing metadata stays `unknown`; never infer sensitivity or public status
  from the value text, filename, engineering appearance, or fixture location;
- `local_private_intent`, `explicit_local_private_intent`, and `user_intent`,
  at item top level or nested inside `export_policy`, are non-authoritative in
  route source data. A route projector strips/overrides them before invoking
  the unchanged contract. It may retain a diagnostic that source intent was
  ignored, but it may not add a new action, context, reason code, or enum;
- existing unit/dimension, provenance, checksum, diagnostic, sandbox,
  protected-content/report, and professional-boundary records survive unless
  the existing contract redacts their unsafe value content.

## 4. Route contexts and materialization

| Route family | Fixed context | Materialization rule |
|---|---|---|
| calculation report preview, JSON, HTML, print/PDF, lint packet | `public_report` | field-level sanitized report; no unredacted local-report escape in this tranche |
| native model package | `shared_model` | field-level valid JSON package |
| handoff, adapter SDK, PCF, MBF, review geometry, stress-neutral, result handoff | `downstream_tool` | field-level JSON/manifest sanitization; specialized members follow the next rule |
| CAEPIPE external-run evidence, secret/private admin, active CLI output, other local administrative/evidence packets | `local_private` | retain known private values only with wrapper-owned explicit intent; unknown values retain warning-only unknown posture |
| existing redaction-control review panel | user-selected existing context | preserve current behavior and parity |

For PCF, MBF, CSV, glTF buffers, rendered HTML, and other specialized
text/binary members, the implementation must not produce a structurally valid
but semantically corrupted artifact. Emit the member only when its decisions
are all `include` or permitted `warning_only`. If the control calls for
redaction, omission, or blocking that cannot be materialized without changing
the governed format meaning, withhold the entire member using the existing
decisions/findings; do not invent a replacement format or reason-code enum.

## 5. Language and surface bindings

### 5.1 Python handoff writers

Introduce one internal shared route-control adapter under
`core/security/redaction/` and route-specific projectors beside each owned
package. Each of the seven public writer functions hard-codes the context in
§4 and returns the controlled result. A caller cannot supply or override the
context. Only `write_caepipe_external_run_package`, whose context is fixed
`local_private`, accepts required keyword-only
`explicit_local_private_intent`; the other writers expose no intent argument.
Before invoking the existing contract, every route projector removes all
source-carried intent keys identified in §3. This wrapper-owned intent rule is
route behavior only: the underlying contract and shared parity corpus remain
unchanged.
All validate/redact before creating a directory or file. Tests reject any
context-override parameter or bypass callback.
Existing builders may construct raw in-memory packages for diagnostics and
tests, but a final writer or downstream workflow cannot accept a caller flag
that bypasses the control.

`build_handoff_export_workflow` returns a controlled result and must execute,
not merely cite, the redaction contract. Existing format, privacy, unit,
mapping, checksum, and authority diagnostics remain additive gates.

### 5.2 Desktop downloads and reports

Add one internal `ControlledExportLink`/hook in the redaction-controls feature
and route-specific projectors. It receives the fixed route ID/context, source
payload, materializer, and explicit-intent state. It renders decision/finding/
summary evidence and creates no `href` while blocked.

Every frozen `download` attribute is replaced by this controlled link or is
proved to consume a controlled result. Local-private links expose an unchecked
explicit-intent control adjacent to the link; checking it is the only way to
retain known private values. It is disabled/absent for non-local contexts.
Payload-carried intent is stripped/overridden by the route projector and
cannot enable or disable the UI-owned intent state.

`DOTH-CAEPIPE-LOCAL-006` is a distinct fixed-`local_private` route for the
desktop CAEPIPE external Harness JSON and Parser CSV downloads. It is not part
of downstream-tool `DOTH-FORMAT-003`. Its adjacent UI intent control is the
only authority for known-private retention; the specialized whole-member rule
still applies, and no `href` exists without intent or when findings block.

Report-specific requirements:

- replace the false `invented_public_example` classification of restored
  user-local project data with explicit leaf metadata;
- sanitize before DOM preview, Tauri invoke, report checksum/hash, HTML
  creation, iframe `srcDoc`, save link, and print frame;
- suppress all report payload/iframe/save/print surfaces on redaction or
  renderer blocking findings;
- preserve browser-mode `unavailable_browser_preview`, sandboxed iframe,
  renderer validation, protected-content lint, canonical sanitized HTML hash,
  and derived-print hash reference.

### 5.3 Headless runners

Do not add a verb. The final `openpipestress-runner` keeps its existing
operations and adds only:

```text
--explicit-local-private-intent
```

Its context is hard-coded `local_private`; callers cannot override it and
intent defaults false. Active
`validate-input`, `solve`, `run-benchmark`, and `run-regression` results are
wrapped as controlled output. On blocking findings, stdout contains the
controlled envelope with `payload:null`, exit is `1`, and `--output` creates
no file. Usage/malformed input remains exit `2`; successful controlled output
remains exit `0`. `export-results` stays the existing stub and no DEL-08-01
report-package binding is introduced.

Both runner projectors strip/override source-carried intent fields before the
bounded Rust mirror runs. Only the parsed CLI flag is authoritative for known
private retention; unknown values keep the unchanged warning-only posture.

The compatibility `headless_preview_runner` also has fixed context
`local_private`, adds the same explicit-intent flag after its existing required
fixture path, and emits the same controlled envelope/exit behavior. This
changes no operation or transport and removes its raw-stdout bypass.

The Rust binding is a bounded mirror of the existing contract, lives inside
the headless crate, consumes the same shared parity corpus in tests, and may
not add actions, contexts, reason codes, or policy precedence.

### 5.4 Separately governed protected-content diagnostic CLI

`protected_content_lint_cli` is not a model/report/result export route. It is
the DEC-058 diagnostic wrapper consumed by the DEC-059 public-source exporter
and release scan. Its finding excerpts are diagnostic evidence required by
those governed consumers. This tranche makes no code/schema/output change to
the CLI or either consumer. A focused repository assertion must prove that
the binary remains referenced only by those governed scan/export tools and is
not called as a product payload/report export. Any product call discovered at
execution stops for a brief amendment.

### 5.5 Plugin and bug-report future routes

Add only regression assertions/documentation that no plugin runtime or bug-
report egress exists and that any future implementation must consume the
controlled-export boundary. Do not create a loader, command, writer, bundle,
transport, UI control, or storage location.

## 6. Exact write fence after owner adoption

Before adoption, only this candidate and its managed run artifacts are
writable. After adoption, N4 is the sole integration owner and may write only
if the owner decision expressly authorizes the cross-package integration map
in §6.1.

### 6.1 Required bounded cross-package integration authorization

This remains a PKG-12/DEL-12-02 WORKING_ITEMS activation. It does not claim
ownership of other packages. The final-sink edits necessarily intersect
accepted surfaces owned outside PKG-12 and one reconciliation row with
`NONE_FOUND` attribution. `AFFECTED_OWNER_MAP.csv` is the exact path-to-
package-to-deliverable/shared-owner map, with one row per affected path or
explicitly co-owned path set and an exact non-state/handoff disposition.
Owner adoption must explicitly authorize N4 as the one serialized
integration owner for only those route-gate edits. Payload/format/report/
runner semantics and every non-DEL-12-02 status surface remain read-only.

Without that explicit authorization, N4 remains blocked and HELP_HUMAN must
route package-local dispositions separately. The bounded integration grant
does not weaken the one-package rule: WORKING_ITEMS retains semantic/state
ownership only for PKG-12; other packages receive no lifecycle, memory,
Remaining, or acceptance effect.

After that authorization, N4 may write only:

### Shared contract

- `core/security/redaction/__init__.py`
- `core/security/redaction/controls.py`
- new bounded files under `core/security/redaction/`
- `schemas/redaction_export_controls.schema.yaml` only if required to keep the
  existing controlled-result shape truthful; no enum additions
- `fixtures/redaction_export_controls/cases.json`
- `tests/security/test_redaction_export_controls.py`
- `apps/desktop/src/features/redaction-controls/redactionExportControls.ts`
- its existing test plus new bounded shared-control component/hook tests in
  that same feature folder

### Python final sinks

- `core/handoff/exporter/workflow.py`
- `core/handoff/native_json/package.py`
- `core/handoff/pcf_export/package.py`
- `core/handoff/review_geometry/package.py`
- `core/handoff/stress_neutral/package.py`
- `core/handoff/caepipe_mbf/package.py`
- `core/handoff/caepipe_external/run.py`
- `core/handoff/export_adapter_sdk/package.py`
- only their existing focused test modules:
  `tests/test_handoff_export_workflow.py`,
  `tests/test_native_json_export_package.py`,
  `tests/test_pcf_export_package.py`,
  `tests/test_review_geometry_export_package.py`,
  `tests/test_stress_neutral_export_package.py`,
  `tests/test_caepipe_mbf_export_package.py`,
  `tests/test_caepipe_external_run_package.py`, and
  `tests/test_export_adapter_sdk.py`
- `tests/test_adapter_framework_contract.py` and
  `tests/test_plugin_manifest_schema.py` for future-route assertions only

### Desktop final sinks

- new bounded report projector/test files under
  `apps/desktop/src/features/report/`
- `apps/desktop/src/features/report/ReportPanel.tsx`
- `apps/desktop/src/features/report/RenderedReportPanel.tsx`
- `apps/desktop/src/features/report/renderableReportInput.ts`
- `apps/desktop/src/features/report/renderedReport.test.tsx`
- `apps/desktop/src/features/report-lint/ReportLintPanel.tsx`
- `apps/desktop/src/services/reportRenderService.ts` only if the pre-IPC
  controlled-result type requires it; no transport change
- `apps/desktop/src/App.test.tsx`
- `apps/desktop/e2e/r2-smoke.spec.ts`
- the following frozen download-owning feature folders, limited to their
  existing `*Panel.tsx` and corresponding existing/new focused tests:
  `accessibility-baseline`, `adapter-framework`, `build-readiness`,
  `caepipe-external`, `caepipe-mbf`, `design-workspace`, `diff-preview`,
  `editor-contract`, `export-adapter-sdk`, `export-review`, `external-prover`,
  `handoff`, `headless-runner`, `local-fea-handoff`, `missing-data`,
  `native-package`, `operations`, `pcf-export`, `project-storage`,
  `project-validation`, `result-export`, `review-geometry`, `rule-check`,
  `secret-private-library`, `security-threat-model`, `solve`,
  `stress-neutral`, `telemetry`, and `validation-evidence`

### Runner

- `core/runner/headless/src/bin/openpipestress-runner.rs`
- `core/runner/headless/src/bin/headless_preview_runner.rs`
- `core/runner/headless/src/lib.rs`
- one new bounded redaction-binding module under
  `core/runner/headless/src/`
- `core/runner/headless/Cargo.toml` and its local ignored lockfile only if
  required by tests; no new external dependency
- `schemas/headless_runner.schema.yaml`
- `tests/test_headless_runner_contract.py`

The following are explicitly read-only and outside N4's fence:
`core/reporting/protected_content_linter/**`,
`tools/release/run_release_candidate_scan.py`, and
`tools/release/export_public_openpipestress.py`.

### Documentation, evidence, and closeout

- `docs/security/redaction_export_controls.md`
- implementation evidence under this managed run root, excluding state
  closure fields reserved to W3
- exactly one new DEC-025 sweep artifact

N4 may not write DEL-12-02 `_STATUS.md`, `MEMORY.md`, a final deliverable run
record, or a loop receipt.

No other file is writable. Any need outside this fence returns to
HELP_HUMAN for a versioned amendment.

## 7. Implementation graph after adoption

1. **N4 — sole serialized implementation owner:** `TASK +
   software-bounded-implementation`; verify frozen basis and route matrix,
   implement shared adapter then Python, desktop/report, and runner bindings;
   run focused and full checks; return implementation evidence without any
   deliverable-state, final-run-record, or receipt closeout.
2. **N5 — fresh independent verifier:** `TASK + software-code-review`;
   read-only review of 100% of route-matrix rows, exact diff containment,
   contract parity, evidence, and the absence of premature state changes. It returns only
   `COMMIT-SAFE` or `BLOCK` and performs no repair.
3. **W3 — post-verifier manager closeout:** only after N5 `COMMIT-SAFE`,
   WORKING_ITEMS may update DEL-12-02 `_STATUS.md`, `MEMORY.md`, and one final
   `_run_records/**` entry. HELP_HUMAN/CHANGE later owns receipt and Git
   closeout under their gates.
4. A verifier `BLOCK` holds all state/receipt closeout. Remediation requires a new bounded N4
   attempt followed by a fresh N5 review.

## 8. Acceptance predicates

The implementation is acceptable only when all hold:

1. Every RouteID has the exact disposition in
   `ROUTE_VERIFICATION_DISPOSITIONS.csv`. `IMPLEMENT_CONTROLLED_SINK` rows get
   a tested gate; `CALLER_GATE_TRACE` rows prove every final caller is gated;
   `PRESERVE_NO_EGRESS` rows remain deny-only/absent;
   `PRESERVE_EXISTING_CONTROL` retains parity; and exactly
   `REXC-LINT-001` plus `REXC-REL-001` receive
   `SEPARATE_GOVERNANCE_NO_CHANGE`.
2. No direct desktop `download`, report preview/iframe/print, Python writer,
   runner stdout/file, or downstream workflow can expose a blocking/raw result.
3. Structured public/shared output redacts private/unknown values and remains
   schema-valid; specialized formats are withheld rather than corrupted.
4. Known-private local retention is impossible without wrapper-owned explicit
   intent and always produces warning evidence. Source-carried intent cannot
   influence a route. Unknown local values preserve the existing
   `warning_only` outcome while staying explicitly unknown and not asserted
   private or public/releasable.
5. Decisions, findings, blocked state, and summary are visible before payload
   exposure; existing diagnostics are preserved and composed.
6. Python, TypeScript, and bounded Rust behavior pass the same shared parity
   cases with identical actions/reason codes/blocked state/summary counts.
7. Source objects are byte/deep-equal before and after; units, dimensions,
   safe provenance/checksums, sandboxing, report controls, and claim-state
   distinctions are preserved.
8. No plugin or bug-report runtime, new runner verb, transport, network/cloud,
   telemetry, storage root, public-source-export change, or professional claim
   appears in the diff.
9. N4 and N5 leave all deliverable state unchanged. After N5 `COMMIT-SAFE`, W3
   keeps DEL-12-02 `IN_PROGRESS` and may close only its exact Remaining bullet;
   lifecycle, release, legal/security sufficiency, and merge remain gated.

## 9. Verification plan

Focused tests:

- shared Python/TypeScript/Rust parity for every corpus case;
- route-wrapper tests prove top-level and nested source-carried true/false
  intent cannot alter the wrapper-owned intent decision, while the underlying
  parity corpus (including item-intent cases) remains unchanged;
- one allow/redact/block/local-private-intent/non-mutation case per route
  family and a negative exposure assertion per final sink;
- all seven Python writer tests prove no directory/file is created on block;
- runner tests cover both binaries; all four final-runner active verbs,
  stdout, `--output`, exit 0/1/2, and unchanged `export-results` stub; and the
  compatibility runner's explicit-intent/no-raw-stdout behavior;
- report tests cover restored user-local classification, raw DOM suppression,
  pre-IPC block, sanitized hash input, iframe/save/print suppression, and
  composed renderer/linter findings;
- deterministic scan proves every desktop `download` consumes the controlled
  component/result and no new unguarded exposure primitive exists;
- future-route tests prove plugin and bug-report runtimes remain absent.
- a no-change/no-product-use assertion covers `REXC-LINT-001`, and a diff
  assertion proves the DEC-058/DEC-059 CLI and consumers are untouched.

Registered/full checks, in halting order:

1. focused Python, desktop Vitest, Rust crate, and contract tests;
2. affected registered checks selected from the actual diff;
3. full `piping-pytest`;
4. full `desktop-test` and `desktop-build`;
5. H4 Playwright report/download flows in source mode and production-dist mode;
6. relevant Rust crate tests for headless/report surfaces;
7. `harness-pytest` and `harness-self-check`;
8. exactly one DEC-025 evidence sweep at the final implementation head;
9. claims, path-anchor, JSON/schema, containment, whitespace, receipt, and
   `git diff --check` validators.

The native renderer has no real native-GUI automation lane. Mocked-Tauri
Vitest plus Rust command tests are the accepted pre-IPC/native evidence; H4
Playwright covers browser-visible route controls and downloads. The closeout
must record this limitation and may not claim native save/print automation.

## 10. Stop and rerun conditions

Stop before or during implementation if:

- HEAD, active DAG, DEL-12-02 Remaining, contract enums, route set, DEC-065,
  or software profile materially differs from the frozen/adopted basis;
- a route lacks an explicit metadata projection or valid materializer;
- a required repair leaves the exact write fence;
- tests require real private/protected data, network, external tools, or a new
  transport/format/runtime decision;
- an existing gate would need weakening, or a public/specialized artifact
  could be emitted with silently redacted semantics;
- the verifier returns `BLOCK`.

Rerun through a new immutable attempt and fresh verifier; never overwrite
failed evidence.

## 11. Preserved boundaries

- Redaction evidence does not establish legal clearance, security sufficiency,
  professional acceptance, code compliance, certification, sealing,
  authentication, release readiness, or human approval.
- Mechanics solved, user-rule checked, incomplete, and human-review states
  remain distinct.
- No source mutation, cloud/network behavior, hidden storage mutation, direct
  SQL/SQLite, arbitrary code, plugin loading, telemetry, or secret handling.
- Owner merges. This brief does not authorize a commit, push, PR, or merge.

## 12. Attribution and exact owner decision requested

```text
OwnerStandingApproval: DEC-085 / D-52 §2, prospectively refined by DEC-087 / D-54 §1
AgentClassification: CLASSIFY_ELIGIBLE_PENDING_INDEPENDENT_REFUTATION
RuleActivation: ACTIVATED_BY_OWNER_GATE_3
AgentJudgment: SELECT_AND_ADVANCE
SelectedOutcome: §§3–7 route-complete controlled-export design
JudgedBy: WORKING_ITEMS / HELP-HUMAN-PIPING-20260721-DEL1202-REDACTION-BREADTH-R15 W1
AdoptionAuthority: HUMAN_OWNER_BY_STANDING_APPROVAL_OR_CASE_SPECIFIC_DIRECTION
OwnerCaseSelection: ADOPT_V6_AND_AUTHORIZE_EXACT_CROSS_PACKAGE_MAP
RationaleArtifact: execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260721-DEL1202-REDACTION-BREADTH-R15/CURRENT_CANDIDATE_RATIONALE.md
IndependentVerifier: COMMIT-SAFE — instances/N3F/RETURN.md; prior BLOCK history preserved
EffectStatus: N4_IMPLEMENTATION_AUTHORIZED_WITHIN_EXACT_FENCE
PreservedGates: owner merge; lifecycle/stage/release/acceptance; legal/security/professional sufficiency; no new transport/plugin/bug-report feature; F-PIP-1..5
```

After fresh N3F returns `COMMIT-SAFE`, the exact requested owner decision is:

> Adopt v6 of `CB-2026-07-21-DEL-12-02-REDACTION-BREADTH-001` and explicitly
> authorize the bounded cross-package integration scope in
> `AFFECTED_OWNER_MAP.csv`, with N4 as the sole serialized implementation
> owner, N5 as fresh verifier, and DEL-12-02-only W3 closeout only after N5
> `COMMIT-SAFE`; or reject/amend it. Adoption does not authorize
> merge, lifecycle/stage/release/acceptance, publication, or any excluded act.

Until that decision is recorded, no product or deliverable-state write is
authorized.
