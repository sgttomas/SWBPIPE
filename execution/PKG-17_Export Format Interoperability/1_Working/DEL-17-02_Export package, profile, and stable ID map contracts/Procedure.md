# Procedure: DEL-17-02 Export package, profile, and stable ID map contracts

## Population Procedure

1. Confirm DAG-006 is the approved active graph authority.
2. Confirm `DEL-17-01` has committed implementation evidence before treating `DEL-17-02` as the next unblocked PKG-17 population target.
3. Read the local `DEL-17-01` four-document kit, `Source_Basis_Register.md`, and `CAEPIPE_Question_Dossier.md`.
4. Populate this four-document kit at contract level only.
5. Build semantic artifacts that map source basis, requirements, downstream consumers, and boundary exclusions.
6. Extract the local dependency register into `Dependencies.csv`.
7. Refresh deliverable memory and create a run record.
8. Validate the four-document kit, minimum fileset, dependency schema, coordination blocker queue, and diff hygiene.

## Downstream Consumption Procedure

For each later PKG-17 target deliverable:

1. Start from this contract and the `DEL-17-01` source-basis dossier.
2. Declare the target profile and target version basis.
3. Cite the consumed `DEL-17-01` source IDs or record `location TBD` for unresolved source locations.
4. Declare exported, omitted, approximated, delegated, unsupported, and `TBD` behavior.
5. Define stable ID behavior before target writing or parser work, distinguishing direct target-carried IDs from sidecar mappings.
6. Require an export manifest and loss report.
7. Keep target-specific implementation inside that deliverable's approved write scope.
8. Preserve project data-boundary and professional-boundary exclusions.

## Contract Population Checklist

When refining or consuming this contract:

1. Confirm `Source_Basis_Register.md#Public and Official Source Evidence` is the source for target source IDs.
2. Confirm `CAEPIPE_Question_Dossier.md#Question Register` is the source for CAEPIPE open questions.
3. Carry forward `TBD-17-01-001` through `TBD-17-01-006` unless a later admissible source and human scope authority closes the item.
4. Treat `TBD-17-01-003` as blocking direct MBF stable-ID carrier claims.
5. Use sidecar mappings whenever direct or metadata carrier evidence is absent, ambiguous, or `TBD`.
6. Record target executor use only as optional user-owned, license-bound harness metadata.
7. Keep target code/check options as pass-through target configuration unless separately admitted by a public rule-pack design.

## Validation Commands

Run from repository root:

```bash
tools/validation/check_four_documents.sh "execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-02_Export package, profile, and stable ID map contracts"
tools/validation/check_min_viable_fileset.sh "execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-02_Export package, profile, and stable ID map contracts"
python3 tools/validation/validate_dependencies_schema.py "execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-02_Export package, profile, and stable ID map contracts/Dependencies.csv"
python3 tools/validation/validate_semantic_matrix.py "execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-02_Export package, profile, and stable ID map contracts"
python3 tools/validation/validate_lens_register.py "execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-02_Export package, profile, and stable ID map contracts"
rg -n "certify|certified|approve|approved|issue|issued|code compliance|code-compliant|ASME table|protected table|proprietary|validation|validated|formal acceptance|compatibility|CAEPIPE requirement|reverse engineer" "execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-02_Export package, profile, and stable ID map contracts"
rg -n "TBD|tbd|location TBD" "execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-02_Export package, profile, and stable ID map contracts"
git diff --check -- "execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-02_Export package, profile, and stable ID map contracts"
```

If an unscoped worktree check reports findings outside this project's write scope, record the finding as external-scope noise and do not treat it as a blocker for DEL-17-02 closeout. Scoped deliverable checks remain the controlling diff-hygiene evidence for this deliverable.

## Closeout Procedure

Closeout requires:

- four-document validation pass;
- minimum viable fileset validation pass;
- dependency schema validation pass;
- semantic matrix validation pass;
- lens-register validation pass;
- overclaim/prohibited-term scan reviewed for negative guardrail language versus unsupported affirmative claims;
- TBD scan reviewed and summarized in `MEMORY.md`;
- scoped diff-hygiene validation pass, with any outside-project findings recorded as external-scope bypasses rather than blockers;
- memory update with touched files, validation results, remaining TBDs, and boundary exclusions;
- run record creation;
- no edits to `DEL-17-03` through `DEL-17-09` production documents;
- no lifecycle promotion;
- no code, schema, release, compatibility, professional, or code-compliance claim.

## Semantic Enrichment Verification

Pass 3 semantic-lensing enrichment checked `_SEMANTIC_LENSING.md` items `A-001`, `B-001`, `X-001`, and `E-001` against `Specification.md` architecture-basis requirements, `Datasheet.md` local artifact inventory, this procedure's validation and closeout checks, and `Guidance.md` reviewer checklist. The resulting closeout checks require downstream target profiles to preserve the architecture basis, keep the local contract artifact set auditable, validate semantic/lens artifacts before consumption, and carry semantic readiness evidence into review.
