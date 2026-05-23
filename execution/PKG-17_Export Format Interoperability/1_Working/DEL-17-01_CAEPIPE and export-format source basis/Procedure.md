# Procedure: DEL-17-01 CAEPIPE and export-format source basis

## Procedure Purpose

This procedure defines how to maintain and use the DEL-17-01 source basis before downstream export deliverables are populated or implemented.

## Source Intake

1. Confirm that a proposed source is listed in `_REFERENCES.md` or `PKG-17/0_References/_REFERENCE_INDEX.md`.
2. Classify the source as one of:
   - official/public vendor documentation;
   - public standards/specification documentation for an exchange format;
   - project-owned governance/schema/plan material;
   - user/private material excluded from public repository content.
3. Reject or quarantine any source that appears to include proprietary examples, protected standards data, private project data, or commercial files without redistribution rights.
4. Record admitted use and boundary in `Source_Basis_Register.md`.

## Finding Extraction

1. Extract only source-grounded facts that are needed by downstream export work.
2. Paraphrase vendor/public documentation rather than copying long excerpts.
3. Attach every finding to one or more source IDs.
4. Attach a best-effort source section/heading reference; if the location cannot be narrowed during the run, record `location TBD`.
5. Mark any inference as `INFERENCE`, not as a source fact.
6. Mark unresolved target behavior as `TBD`.

## CAEPIPE Question Dossier Maintenance

1. Add a question when public documentation is insufficient for deterministic exporter or harness behavior.
2. Include the affected downstream deliverables.
3. Include the current public evidence basis.
4. State the gating impact.
5. Do not request proprietary internals, protected standards content, commercial examples, or license-bypass instructions.
6. When a question is answered, record the answer source and update affected TBD entries.

## Downstream Consumption

Before a downstream `DEL-17-*` deliverable is populated or modified:

1. Read this four-document kit.
2. Read `Source_Basis_Register.md`.
3. Read `CAEPIPE_Question_Dossier.md`.
4. Confirm all relevant TBDs are either closed or explicitly carried forward.
5. Confirm the downstream deliverable is named in the source-basis register, a dependency register, or a current package plan before treating DEL-17-01 as a gating input.
6. Confirm the downstream deliverable consumes only the relevant source-basis findings, not a blanket claim that every DEL-17 item consumes every target-format source.
7. Preserve the professional/IP/export boundaries from this deliverable.

## Validation

Run:

```text
tools/validation/check_four_documents.sh execution/PKG-17_Export\ Format\ Interoperability/1_Working/DEL-17-01_CAEPIPE\ and\ export-format\ source\ basis
tools/validation/check_min_viable_fileset.sh execution/PKG-17_Export\ Format\ Interoperability/1_Working/DEL-17-01_CAEPIPE\ and\ export-format\ source\ basis
git diff --check -- "execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-01_CAEPIPE and export-format source basis"
```

If an unscoped worktree check reports findings outside this project's write scope, record the finding as external-scope noise and do not treat it as a blocker for DEL-17-01 closeout. Scoped deliverable checks remain the controlling diff-hygiene evidence for this deliverable.

Manual review must check:

- no proprietary examples;
- no copied protected standards data;
- no CAEPIPE compatibility overclaims;
- no code-compliance or professional-acceptance claims;
- admitted source pointers are still reachable or are marked stale/location TBD before reuse;
- unresolved target behavior remains marked `TBD`.

## Semantic Enrichment Verification

Pass 3 semantic-lensing enrichment checked `_SEMANTIC_LENSING.md` items `C-001`, `D-001`, and `E-001` against `Datasheet.md` downstream consumers, `Procedure.md` downstream consumption and validation checks, `_REFERENCES.md` package references, and `Source_Basis_Register.md` finding/downstream-use rows before applying the warranted operational updates.

Later relevant downstream deliverables must cite admitted source IDs, carry forward unresolved questions, and report exported, omitted, approximated, delegated, unsupported, and TBD behavior without converting those records into compatibility, release, code-compliance, or professional-acceptance claims.
