---
doc_id: OPS-ANALYSIS-STATUS-SEMANTICS
doc_kind: architecture.data_model
status: draft
created: 2026-04-30
deliverable_id: DEL-05-04
package_id: PKG-05
scope_item: SOW-047
---

# Analysis Status Semantics

This document defines the analysis-status boundary for OpenPipeStress result envelopes. The companion schema is `schemas/analysis_status.schema.yaml`, written as strict JSON syntax so it can be parsed without a YAML dependency.

## Authority Boundary

Analysis statuses describe what the software has computed or found. They do not claim code compliance, certification, sealing, approval, authentication, or professional reliance.

The software may emit:

- `MODEL_INCOMPLETE` when solve-required physical inputs are missing.
- `MECHANICS_SOLVED` when the numerical mechanics solve completed.
- `RULE_INPUTS_INCOMPLETE` when a model may be solved but rule-pack-required values are missing.
- `USER_RULE_CHECKED` when a user-defined rule pack has evaluated the result.
- `USER_RULE_FAILED` when a user-defined rule pack produced a failing result.
- `HUMAN_REVIEW_REQUIRED` because professional use always requires competent human review.

The software must not emit `HUMAN_APPROVED_FOR_PROJECT` as an automatic status. That value is reserved for an external human acceptance record bound to the exact model, result, rule-pack, and report hashes reviewed by the human.

## Usage Rules

`MODEL_INCOMPLETE` blocks mechanics solving for the affected subject. It is a finding about missing physical data, not an instruction to invent defaults.

`MECHANICS_SOLVED` means displacements, reactions, forces, moments, stresses, or related open-mechanics quantities were computed for a specific model snapshot. It does not mean the model has all user rule-pack inputs.

`RULE_INPUTS_INCOMPLETE` may coexist with `MECHANICS_SOLVED`. It means the rule-pack evaluator lacks required user-supplied code data, owner data, provenance, or private project inputs.

`USER_RULE_CHECKED` means a user-defined rule pack ran against the available result. It is a software computation using user data and rule definitions, not a professional code-compliance statement.

`USER_RULE_FAILED` means a user-defined rule pack returned at least one failing check. The record should preserve rule-pack identity, version, checksum, diagnostics, and affected result references without embedding protected standards text or private rule values in public artifacts.

`HUMAN_REVIEW_REQUIRED` is expected on reportable result envelopes. Reports and exports must preserve the professional boundary notice.

## Human Acceptance Records

Human acceptance is represented separately from `software_status`. A human acceptance record may reference `HUMAN_APPROVED_FOR_PROJECT`, but only as an external record with:

- a human actor;
- a record timestamp;
- model/result/rule/report references where applicable;
- hashes for the reviewed payloads;
- a scope notice describing what was accepted.

If review occurred but project acceptance was not granted, the record uses an explicit non-accepted outcome and does not carry `HUMAN_APPROVED_FOR_PROJECT`. Software, agents, and external systems may write supporting status records, but they cannot be the accepting actor for a `HUMAN_APPROVED_FOR_PROJECT` record.

Human acceptance does not survive content changes. If a bound model, rule pack, result, or report hash changes, the prior human record no longer applies to the changed content.

## Result Envelope Requirements

Every status envelope carries:

- an analysis subject reference;
- one primary automatic software status and a set of automatic software statuses;
- a status stage and basis;
- source and actor metadata;
- at least one hash for the payload or evidence basis;
- optional references to diagnostics, rule packs, reports, or human acceptance records;
- professional-boundary fields that are fixed false for software compliance, certification, sealing, approval, and authentication claims.

The automatic status set intentionally excludes `HUMAN_APPROVED_FOR_PROJECT`. A result can be mechanically solved, rule checked, and still require human review. A human acceptance record can be attached later only as a separate hash-bound record.

## Transition Pattern

A typical status progression is:

1. Model validation emits `MODEL_INCOMPLETE` until solve-required data is present.
2. Solver/stress recovery emits `MECHANICS_SOLVED` after successful mechanics computation.
3. Rule evaluation emits `RULE_INPUTS_INCOMPLETE`, `USER_RULE_CHECKED`, or `USER_RULE_FAILED` depending on user rule-pack inputs and check outcomes.
4. Report/export surfaces `HUMAN_REVIEW_REQUIRED`.
5. A separate human workflow may attach a hash-bound `HUMAN_APPROVED_FOR_PROJECT` record.

These are usage semantics, not a mandatory lifecycle state machine. A result envelope may contain multiple software statuses when they truthfully apply to the same subject and snapshot.

## Protected Data Boundary

Status records may identify rule packs, reports, sources, diagnostics, and hashes. Public schemas, examples, and report templates must not embed protected standards text, protected tables, copied code formulas, proprietary vendor data, private owner requirements, private rule values, or professional approval claims.

## Remaining TBDs

- Exact integration points between this schema and the canonical model/result schema.
- Exact canonicalization edge cases for non-JSON payload hashes.
- Human acceptance workflow ownership, storage location, and UI presentation.
