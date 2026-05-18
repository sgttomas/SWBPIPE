---
doc_id: OPS-CODE-NEUTRAL-ANALYSIS-BOUNDARY
doc_kind: architecture.data_model
status: draft
created: 2026-04-30
deliverable_id: DEL-02-03
package_id: PKG-02
scope_item: SOW-002
---

# Code-Neutral Analysis Boundary

This document defines the rules implemented by `schemas/analysis_boundary.schema.yaml`. The schema is strict JSON syntax in a `.yaml` file so it can be parsed without a YAML dependency.

## Boundary Rules

- Every boundary record declares an explicit authority model:
  `solver_result_only` for mechanics, `software_computation_using_user_data`
  for user-rule checks, and `external_hash_bound_human_record_only` for human
  acceptance.
- The mechanics solver computes open mechanics results only: model validation findings, displacements, reactions, forces, moments, stresses, convergence facts, and diagnostics.
- A mechanics result is not a code check, pass/fail result, professional acceptance, certification, sealing, approval, authentication, or compliance claim.
- User-rule checks require an explicit user-supplied rule-pack reference with version, checksum, source notice, redistribution status, and provenance.
- Missing solve-required or rule-check-required inputs are explicit `MODEL_INCOMPLETE` or `RULE_INPUTS_INCOMPLETE` findings with diagnostics. They are not silently defaulted.
- Human acceptance, if present, is referenced as an external project record bound to reviewed model, rule-pack, result, or report hashes. The solver and rule-pack evaluator do not generate it.

## Prohibited Public Content

Public schemas, examples, reports, and code must not bundle code-specific defaults, protected standards text, protected tables, copied code formulas, material allowables, stress-intensification or flexibility-factor tables, protected dimensional tables, proprietary vendor data, owner requirements, or private rule-pack values.

The public model may carry references, checksums, source notices, provenance metadata, diagnostics, and private/public markings. It must not reproduce protected or proprietary source content.

## Status Separation

`MECHANICS_SOLVED` means the mechanics calculation completed for a stated subject and evidence set. It does not imply that all rule-pack inputs exist.

`RULE_INPUTS_INCOMPLETE`, `USER_RULE_CHECKED`, and `USER_RULE_FAILED` are user-rule outcomes. They depend on user-supplied rule-pack data and remain software computations using user data.

`HUMAN_REVIEW_REQUIRED` remains true for professional use. `HUMAN_APPROVED_FOR_PROJECT` is reserved for an external human acceptance record and must not appear as an automatic software status.

## Schema Surface

`schemas/analysis_boundary.schema.yaml` requires:

- `authority_model`, which records the mechanics, user-rule, and human
  acceptance authority domains;
- `mechanics_solve`, which is limited to model-incomplete or mechanics-solved
  solver states;
- `user_rule_check`, which is limited to rule-inputs-incomplete,
  user-rule-checked, or user-rule-failed states and requires a user-supplied
  rule-pack reference;
- `missing_inputs`, which carries explicit diagnostics and provenance for
  missing solve or rule-check inputs;
- `human_acceptance_refs`, which are references to external records with bound
  evidence hashes;
- `professional_boundary`, whose claim fields are fixed false for software.

## Remaining TBDs

- Exact persistence location for human acceptance records.
- Exact stale-record invalidation workflow when bound hashes change.
- Exact integration point between this boundary schema, result envelopes, and report manifests.
