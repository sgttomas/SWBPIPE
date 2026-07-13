---
doc_id: OPS-DEVELOPER-GUIDE
doc_kind: documentation.developer_guide
status: draft
created: 2026-05-03
deliverable_id: DEL-11-02
package_id: PKG-11
scope_item: SOW-033
---

# OpenPipeStress Developer Guide

This guide is for contributors extending the solver, rule-pack, report, and
adapter surfaces of OpenPipeStress. It is documentation for the public project;
it does not define protected rule content, private design bases, legal rights,
or professional engineering acceptance.

OpenPipeStress keeps three responsibilities separate:

- the open solver computes code-neutral mechanics;
- user-supplied rule packs evaluate user-defined acceptability checks;
- professional use remains a human responsibility outside automatic software
  output.

Current authority basis: `execution/_Decomposition/SOFTWARE_DECOMP.md`
revision `0.7` and approved `execution/_DAG/DAG-007/`. `DAG-007` is graph
coordination authority only; it does not dispatch implementation work, move
lifecycle states, approve deliverables, or create professional, release, or
code-compliance claims.

## 1. Governing Artifacts

Read these project artifacts before changing solver or rule-pack behavior:

| Artifact | Use |
|---|---|
| [`docs/DIRECTIVE.md`](../DIRECTIVE.md) | Founding intent, stop rules, scope boundaries, local-first/privacy posture, and human-authority limits. |
| [`docs/CONTRACT.md`](../CONTRACT.md) | Binding invariants for IP, data, units, solver behavior, rule packs, privacy, reports, and agent work. |
| [`docs/IP_AND_DATA_BOUNDARY.md`](../IP_AND_DATA_BOUNDARY.md) | Public/private content rules, provenance requirements, and quarantine process. |
| [`docs/SPEC.md`](../SPEC.md) | Current technical architecture, domain objects, unit model, solver and rule-pack boundaries, reports, and V&V posture. |
| [`docs/TYPES.md`](../TYPES.md) | Canonical vocabulary for domain objects, statuses, diagnostics, rule packs, results, reports, and extension boundaries. |
| [`docs/VALIDATION_STRATEGY.md`](../VALIDATION_STRATEGY.md) | Benchmark families, release-quality evidence categories, and validation source rules. |
| [`docs/architecture/code_neutral_analysis_boundary.md`](../architecture/code_neutral_analysis_boundary.md) | Separation between mechanics solve authority, user-rule-check authority, and external human acceptance records. |
| [`docs/architecture/analysis_status_semantics.md`](../architecture/analysis_status_semantics.md) | Allowed automatic analysis statuses and the external, hash-bound human acceptance boundary. |
| [`docs/architecture/persistence_contract.md`](../architecture/persistence_contract.md) | Project persistence envelope, SCA-003 local SQLite profile, canonical JSON/JCS hash basis, and application-service persistence boundary. |
| [`docs/architecture/plugin_boundary.md`](../architecture/plugin_boundary.md) | API/plugin boundary rules, permissions, no-bypass constraints, checksums, and provenance expectations. |
| [`docs/architecture/extension_domain_contracts.md`](../architecture/extension_domain_contracts.md) | Domain rules for plugins and adapters that touch models, rule packs, diagnostics, reports, or results. |
| [`docs/security/local_first_storage_policy.md`](../security/local_first_storage_policy.md) | Local-first storage, private path, telemetry-off-by-default, and storage no-bypass policy. |

Unresolved implementation choices remain `TBD` unless a human-approved
architecture or implementation deliverable records the decision. Current `TBD`
items include the production sparse solver integration, rule-expression
implementation-library integration, dependency versions, CI thresholds, external transport,
plugin loading/isolation details, operating-system storage roots, product/DB
migration rollback semantics, redaction workflow, cloud exception workflow, and
portable project export/copy workflow. SCA-003 has resolved the MVP storage
substrate as a local SQLite-backed project store/index. SCA-004 has admitted
export interoperability as explicit boundary work, but concrete writers,
runtime invocation, endpoint syntax, target coverage, and permission grant
mechanics remain governed deliverable work or `TBD`.

## 2. Architecture Map

OpenPipeStress is organized as a code-neutral workbench:

```text
GUI / UX Layer
  -> Application Services
       -> Domain Core
            -> Geometry + Units + Data Schemas
            -> Solver Core
            -> Loads + Stress Recovery
            -> Rule-Pack Evaluator
            -> Reporting / Audit Manifest
       -> Adapters
            -> Private Libraries
            -> Import / Export Plugins
            -> Local FEA Handoff
            -> Headless Runner / Export Packages
            -> Storage / Packaging
```

The domain core owns schema validation, unit and dimensional checks,
provenance checks, diagnostics, result envelopes, rule-pack sandbox controls,
and public/private data boundaries. Adapters and plugins translate data at the
boundary; they do not own the core validation policy and must not bypass it.

Schema-first JSON contracts are the public interchange baseline. SCA-003 local
SQLite storage is a private payload/index substrate behind application-service
boundaries, not a public SQL contract. Canonical JSON/JCS-compatible payload
bytes remain the domain, hash, and interchange truth. Contributors must not
add direct SQL access, raw SQLite handle exposure, table-name dependencies, or
direct project-store mutation paths for plugins, adapters, CLI runners, or
other extension code.

The main surfaces for solver and rule-pack work are:

- `schemas/model.schema.yaml` for canonical project and model structure;
- `schemas/units.schema.yaml` for unit-bearing quantities and dimension checks;
- `schemas/analysis_boundary.schema.yaml` and
  `schemas/analysis_status.schema.yaml` for status separation;
- `schemas/rule_pack.schema.yaml` for user-owned rule-pack structure;
- `schemas/results.schema.yaml` for result envelopes and downstream result
  exports;
- `schemas/project_persistence.schema.yaml` for versioned persistence envelopes,
  local SQLite storage profile metadata, migration status, round-trip
  manifests, and application-service operation records;
- `schemas/headless_runner.schema.yaml` for headless solve, validation,
  benchmark, regression, and result-export request/result envelopes;
- `api/api_boundary_contract.yaml` for command, query, job, and result-envelope
  boundaries;
- `schemas/plugin_manifest.schema.yaml` and
  `schemas/adapter_framework.schema.yaml` for governed extension surfaces;
- `schemas/local_fea_handoff.schema.yaml`, `schemas/handoff_package.schema.json`,
  and `schemas/target_mapping.schema.json` for governed downstream handoff
  metadata, target mapping, unsupported-behavior records, and professional
  boundary notices;
- `schemas/native_json_export.schema.json`,
  `schemas/stress_neutral_export.schema.json`,
  `schemas/caepipe_mbf_export.schema.json`,
  `schemas/caepipe_external_run.schema.json`,
  `schemas/pcf_export.schema.json`, `schemas/review_geometry_export.schema.json`,
  and `schemas/export_adapter_sdk.schema.json` for SCA-004 export package,
  profile, evidence, and adapter-SDK boundaries.

When a contributor adds a new behavior, the implementation path should preserve
schema validation first, then units, provenance, privacy classification,
diagnostics, result envelopes, checksums, and report controls as applicable.
API, plugin, adapter, runner, and export code must route persistence through
application-service commands, queries, or jobs. They must not expose SQL, raw
SQLite handles, table names, direct project-store mutation, or private
storage-path defaults as public contracts.

Current cross-cutting implementation surfaces include:

| Surface | Contributor expectation |
|---|---|
| `core/project_persistence/service.py` | Treat SQLite as the local store/index substrate and canonical JSON/JCS-compatible payload bytes as domain truth. |
| `core/adapters/framework/adapter_framework.py` | Validate format-neutral adapter declarations for units, provenance, privacy, protected-content screening, result envelopes, and no-bypass controls without selecting real external formats. |
| `core/runner/headless` | Preserve request/result envelope, privacy, provenance, checksum, diagnostics, progress/cancellation, and professional-boundary controls for headless workflows. |
| `core/handoff/*` | Keep handoff and export packages provider-neutral, hash/provenance-bound, loss-report-aware, and non-authoritative for professional reliance. |
| `core/security/*` | Preserve local-first storage, redaction/export, telemetry-off-by-default, secret/private-library, and threat-model boundaries for any touched surface. |

## 3. Solver Architecture

The solver side computes mechanics for a 3D centerline/frame analytical model.
Contributors should treat the schema-backed physical model as source-of-truth
input and derive solver-ready analytical objects through explicit, reviewable
transformations.

Expected solver concepts include:

- nodes with coordinates and six degree-of-freedom state semantics;
- elements connecting stable node references with material, section, optional
  component, local-coordinate, and result-station hooks;
- local/global orientation handling for element behavior;
- assembly, boundary-condition reduction, and solve execution behind the
  solver boundary;
- load application for explicit primitive loads, user loads, imposed
  displacements, and result-state algebra;
- stress recovery from mechanical resultants and user-supplied section or
  pressure inputs;
- deterministic diagnostics for missing inputs, unit mismatch, singular or
  invalid model state, convergence state, and unsupported behavior;
- nonlinear support active-state and nonconvergence reporting where applicable.

Existing implementation and module surfaces include:

| Surface | Contributor expectation |
|---|---|
| `core/solver/frame_kernel` | Keep frame mechanics deterministic and unit-compatible; sparse production solve policy remains `TBD`. |
| `core/solver/straight_pipe` | Adapt user-supplied pipe properties into the frame element boundary without supplying protected pipe tables or code-specific values. |
| `core/solver/linear_supports` | Map explicit support data to solver degrees of freedom without adding catalog defaults. |
| `core/solver/nonlinear_supports` | Report active-state and nonconvergence facts; do not hide unresolved nonlinear behavior. |
| `core/solver/diagnostics` | Emit stable diagnostics and statuses for solver facts and blocking conditions. |
| `core/solver/performance_harness` | Use invented or public-permissive fixtures with provenance for repeatability and regression evidence. |
| `core/loads/primitive_loads` and `core/loads/user_loads` | Prepare explicit load contributions; code-specific load combinations remain user/rule-pack supplied. |
| `core/loads/load_case_algebra` | Preserve unit and status intent for combinations, result-state subtraction, and ranges. |
| `core/loads/stress_recovery` | Recover open mechanics stress components only; do not add protected formulas, allowables, SIFs, flexibility factors, or public code-check logic. |

Missing solve-required data must become explicit findings, not silent defaults.
If a contribution needs protected formulas, protected tables, proprietary
commercial examples, private owner standards, or private project data, stop and
route the issue through the project data-boundary process.

## 4. Rule-Pack Architecture

Rule packs are user-owned, schema-governed artifacts. The public repository may
define the schema, lifecycle metadata, evaluator boundary, invented examples,
and tests. It must not bundle protected standards content, protected formulas,
material allowables, SIF/flexibility tables, protected dimensional tables,
private owner requirements, or proprietary catalog values as public defaults.

The rule-pack schema surface is `schemas/rule_pack.schema.yaml`. A valid
contributor-facing change should preserve these categories:

- identity, version, lifecycle status, and source notice;
- public/private classification and redistribution status;
- checksum metadata and canonical payload references where applicable;
- required input declarations with unit and dimension expectations;
- variables, formula declarations, value slots, and check definitions;
- diagnostic policy for missing inputs, unit mismatch, provenance gaps,
  redistribution gaps, protected-content suspicion, and evaluator errors;
- provenance and review status for public or contributed data;
- report notice and professional-boundary controls.

Rule-pack evaluation is declarative, sandboxed, deterministic, and unit-aware.
It cannot execute arbitrary host-language code, access the filesystem or
network, spawn processes, or bypass unit/provenance/privacy checks. DEC-022
fixes the canonical grammar as `open_pipe_stress_declared_expression`: a
checksum-bound, versioned typed AST with no text parser and the governed
boolean, conditional, aggregate, interpolation, lookup, and dimensional-product
operations. DEC-037 keeps writable text input deferred and permits only a
display-only AST-to-text rendering. Remaining evaluator/library integration
choices stay `TBD`; they do not reopen the canonical grammar.

Existing implementation surfaces include:

| Surface | Contributor expectation |
|---|---|
| `core/rules/expression_evaluator` | Evaluate declared expression structures with explicit variable bindings and dimension metadata only. |
| `core/rules/completeness_checker` | Compare required-input declarations against supplied evidence and emit rule-check-blocking findings when data is missing. |
| `core/rules/rule_pack_lifecycle` | Record privacy, redistribution, protected-content review, checksum, and lifecycle findings without exposing private formulas or values. |
| `examples/rule_packs/invented_demo.yaml` | Use only as an invented software demonstration pattern, not as engineering design guidance. |

`MECHANICS_SOLVED` means the mechanics result exists for the stated model and
evidence set. It does not mean a user rule check is complete. `USER_RULE_CHECKED`
and `USER_RULE_FAILED` are software computations using user-supplied rule-pack
data. OpenPipeStress does not currently define `USER_RULE_PASSED`; a checked
outcome with no reported failures remains `USER_RULE_CHECKED` plus details, not
a compliance result. `HUMAN_REVIEW_REQUIRED` remains visible for professional
use.

## 5. Data, Privacy, And Provenance

Every public data contribution that affects mechanics, rule checks, reports,
fixtures, examples, or libraries needs source, provenance, license or
redistribution status, contributor certification, and review disposition.

Public contribution candidates may include:

- open mechanics algorithms;
- blank schema slots and templates;
- invented non-engineering examples with clear notices;
- public-domain or permissively licensed data after review;
- original validation fixtures with documented provenance.

Public contribution candidates must not include:

- protected standards text, tables, figures, examples, or commentary;
- copied protected formulas or code-derived acceptance methods;
- material allowable tables copied from standards;
- protected SIF, flexibility-factor, dimensional, or rating tables;
- proprietary vendor catalogs without redistribution rights;
- commercial software benchmark files or report templates without permission;
- private project models, owner standards, company design bases, or private
  rule packs;
- real secrets, credentials, tokens, or private storage paths.

If protected or private content is suspected, stop ingestion, mark the content
as suspected or private as appropriate, quarantine it through the approved
process, and request human review. Do not paraphrase protected tables into
public examples and do not convert private values into public defaults.

Private project data, private material/component libraries, owner standards,
private rule packs, generated reports, diagnostics, SQLite stores, sidecar
indexes, caches, and secrets are user-controlled by default. Public repository
paths are not default durable storage for those payloads. Telemetry is off by
default and must not become a storage, synchronization, or support-bundle path
for private engineering data.

## 6. Diagnostics And Result Envelopes

Solver, rule-pack, report, and adapter outputs should expose structured
diagnostics rather than hiding uncertainty. Diagnostics should identify the
source, severity, affected object, remediation path, provenance where relevant,
and whether the finding blocks a solve, blocks a user-rule check, warns about
privacy/provenance/IP risk, or records an unsupported `TBD`.

Result envelopes should preserve:

- schema version and solver or evaluator version where available;
- model, input, run, and result references;
- unit-aware result values;
- analysis statuses and user-rule statuses without collapsing them into one
  outcome;
- diagnostics, warnings, assumptions, and limitations;
- rule-pack references by ID, version, checksum, source notice, and
  redistribution status;
- reproducibility and checksum metadata.

Reports and exports may reference private rule-pack identity, version, checksum,
and source notes without exposing private formulas or protected values in public
templates.

Export and handoff envelopes should also preserve stable model identity, unit
manifests, privacy/redaction state, target mapping, loss or unsupported-behavior
records, external-run evidence boundaries, and professional-boundary notices.
External-run evidence is user-owned regression or handoff evidence only. It is
not a bundled commercial-solver path, formal validation claim, professional
acceptance, or code-compliance result.

## 7. Test Discipline

Solver and rule-pack changes need deterministic evidence before release use.
The required evidence depends on the touched surface:

| Change area | Evidence expected |
|---|---|
| Units and schemas | Schema validation tests, dimension checks, invalid unit rejection, and round-trip behavior where applicable. |
| Frame mechanics | Deterministic mechanics benchmarks using original, public-domain, or permissively licensed fixtures. |
| Loads and stress recovery | Regression tests for explicit load application, result-state algebra, and open mechanics stress components. |
| Nonlinear supports | Active-state traces, convergence or nonconvergence diagnostics, and repeatable unresolved-state reporting. |
| Rule packs | Required-input checks, unit mismatch tests, unsafe-expression rejection, invented-example tests, checksum/lifecycle checks, and privacy/provenance findings. |
| Reports and exports | Reproducibility, checksum stability, warning inclusion, privacy filtering, and protected-content lint coverage. |
| Adapters, plugins, and APIs | No-bypass tests for schema validation, units, provenance, privacy, protected-content screening, diagnostics, result envelopes, application-service persistence, and sandboxed rule-pack access. |
| Headless runners and export/handoff packages | Request/result envelope tests, progress or cancellation diagnostics where applicable, stable-ID/loss-report checks, privacy/redaction checks, external-run evidence boundaries, and professional-boundary assertions. |

Validation fixtures and public examples must be original, public-domain, or
permissively licensed with review evidence. Mechanics verification and workflow
validation support engineering review; they do not replace project-specific
human judgment.

## 8. Contribution Checklist

Before proposing a solver, rule-pack, report, or adapter change, verify:

- the change stays inside the assigned deliverable scope and write scope;
- affected schemas, modules, docs, and tests use current project vocabulary;
- unit-bearing values carry explicit units and dimensions;
- missing solve-required or rule-check-required values emit diagnostics;
- provenance, redistribution status, review status, and public/private markers
  are preserved;
- local SQLite remains an application-service storage/index substrate, not a
  public API, plugin, adapter, report, or interchange contract;
- plugins, adapters, APIs, runners, and export workflows do not bypass schema,
  unit, provenance, privacy, protected-content, diagnostic, persistence,
  rule-sandbox, report, or human-acceptance controls;
- rule-pack evaluation remains sandboxed and deterministic;
- public docs, fixtures, examples, and tests contain no protected standards
  content, proprietary commercial examples, private project data, or real
  secrets;
- public examples use invented, public-domain, or permissively licensed data
  with review evidence and do not include public default rule values;
- unresolved choices are labeled `TBD`;
- inferred statements are labeled `ASSUMPTION` and proposed future behavior is
  labeled `PROPOSAL`;
- result and report wording keeps mechanics results, user-rule checks, and
  external human review separate.

Agent outputs, generated artifacts, and software results remain drafts or
decision-support records until accepted by the appropriate human project gate.
