# Semantic Lensing Register: DEL-14-02 Analysis run records

**Generated:** 2026-05-03
**Deliverable Folder:** `execution/PKG-14_Model States, Analysis Runs, and Comparison/1_Working/DEL-14-02_Analysis run records`
**Warnings:** none

**Inputs Read:**
- `_CONTEXT.md` - deliverable identity and scope
- `_STATUS.md` - current state SEMANTIC_READY
- `_SEMANTIC.md` - matrices A, B, C, F, D, X, E
- `Datasheet.md` - production document
- `Specification.md` - production document
- `Guidance.md` - production document
- `Procedure.md` - production document
- `_REFERENCES.md` - reference inventory

**Purpose:** Apply `semantic-matrix-build` matrix cells as lenses over the production documents, capturing warranted enrichment inputs for a later enrichment pass.

## Summary

- Total warranted items: 6
- By document:
  - Datasheet: 0
  - Specification: 3
  - Guidance: 2
  - Procedure: 1
- By matrix:
  - A: 1  B: 0  C: 0  F: 2  D: 1  X: 1  E: 1
- By type:
  - Conflict: 0
  - VerificationGap: 2
  - MissingSlot: 2
  - WeakStatement: 0
  - RationaleGap: 1
  - Normalization: 0
  - TBD_Question: 1
  - MatrixError: 0
- Notable conflicts: 0
- Matrix parse errors: 0

## Matrix A - Orientation

### Lens Coverage

| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| A:[normative]:[guiding] | normative | guiding | prescriptive direction | 0 | NO_ITEMS | No warranted enrichment beyond existing source-grounded scope. |
| A:[normative]:[applying] | normative | applying | mandatory practice | 1 | HAS_ITEMS | Schema field/cardinality detail remains TBD. |
| A:[normative]:[judging] | normative | judging | compliance determination | 0 | NO_ITEMS | Professional-boundary constraints are already captured. |
| A:[normative]:[reviewing] | normative | reviewing | regulatory audit | 0 | NO_ITEMS | Audit/professional-boundary language is present. |
| A:[operative]:[guiding] | operative | guiding | procedural direction | 0 | NO_ITEMS | Procedure records setup-stage workflow. |
| A:[operative]:[applying] | operative | applying | practical execution | 0 | NO_ITEMS | Implementation execution is explicitly TBD. |
| A:[operative]:[judging] | operative | judging | performance assessment | 0 | NO_ITEMS | Verification table captures expected checks. |
| A:[operative]:[reviewing] | operative | reviewing | process audit | 0 | NO_ITEMS | Records section is present. |
| A:[evaluative]:[guiding] | evaluative | guiding | value orientation | 0 | NO_ITEMS | Guidance principles are present. |
| A:[evaluative]:[applying] | evaluative | applying | merit application | 0 | NO_ITEMS | Trade-offs are captured without choosing unsupported design. |
| A:[evaluative]:[judging] | evaluative | judging | worth determination | 0 | NO_ITEMS | No conflicting claims found. |
| A:[evaluative]:[reviewing] | evaluative | reviewing | quality appraisal | 0 | NO_ITEMS | No matrix parsing issue found. |

### Warranted Items

| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| A-001 | A:[normative]:[applying] | MissingSlot | Specification | Specification | Record TBD for exact analysis-run schema field names and cardinality. | The Specification lists binding categories from SOW-072 but no accessible source defines exact field names or cardinality. | `Specification.md` | Requirements; Standards | N/A | Keep category-level requirements until implementation or human design source exists. | TBD |

## Matrix B - Conceptualization

### Lens Coverage

| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| B:[data]:[necessity] | data | necessity | essential fact | 0 | NO_ITEMS | Identity facts are present. |
| B:[data]:[sufficiency] | data | sufficiency | adequate evidence | 0 | NO_ITEMS | Source table cites are present. |
| B:[data]:[completeness] | data | completeness | comprehensive record | 0 | NO_ITEMS | Missing values are marked TBD. |
| B:[data]:[consistency] | data | consistency | reliable measurement | 0 | NO_ITEMS | Values are consistent across documents. |
| B:[information]:[necessity] | information | necessity | essential signal | 0 | NO_ITEMS | Scope signals are present. |
| B:[information]:[sufficiency] | information | sufficiency | adequate context | 0 | NO_ITEMS | Context is bounded to available sources. |
| B:[information]:[completeness] | information | completeness | comprehensive account | 0 | NO_ITEMS | No unsupported detail added. |
| B:[information]:[consistency] | information | consistency | coherent message | 0 | NO_ITEMS | Terminology is materially consistent. |
| B:[knowledge]:[necessity] | knowledge | necessity | fundamental understanding | 0 | NO_ITEMS | Domain purpose is stated. |
| B:[knowledge]:[sufficiency] | knowledge | sufficiency | competent expertise | 0 | NO_ITEMS | Professional-boundary limits are stated. |
| B:[knowledge]:[completeness] | knowledge | completeness | thorough mastery | 0 | NO_ITEMS | Setup-stage documents do not need field-level completeness. |
| B:[knowledge]:[consistency] | knowledge | consistency | coherent understanding | 0 | NO_ITEMS | No conflict found. |
| B:[wisdom]:[necessity] | wisdom | necessity | essential discernment | 0 | NO_ITEMS | Unsupported choices are deferred. |
| B:[wisdom]:[sufficiency] | wisdom | sufficiency | adequate judgment | 0 | NO_ITEMS | Human ruling points remain TBD. |
| B:[wisdom]:[completeness] | wisdom | completeness | holistic insight | 0 | NO_ITEMS | Trade-offs are captured. |
| B:[wisdom]:[consistency] | wisdom | consistency | principled reasoning | 0 | NO_ITEMS | Boundary principles are preserved. |

## Matrix C - Formulation

### Lens Coverage

| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| C:[normative]:[necessity] | normative | necessity | binding prerequisite | 0 | NO_ITEMS | Required source constraints are present. |
| C:[normative]:[sufficiency] | normative | sufficiency | enforceable justification | 0 | NO_ITEMS | Citations are present. |
| C:[normative]:[completeness] | normative | completeness | total obligation record | 0 | NO_ITEMS | Unsupported field detail remains TBD. |
| C:[normative]:[consistency] | normative | consistency | coherent conformance basis | 0 | NO_ITEMS | Cross-doc terminology is consistent enough for setup stage. |
| C:[operative]:[necessity] | operative | necessity | execution prerequisite | 0 | NO_ITEMS | Prerequisites are listed. |
| C:[operative]:[sufficiency] | operative | sufficiency | usable work basis | 0 | NO_ITEMS | Procedure gives a bounded future workflow. |
| C:[operative]:[completeness] | operative | completeness | complete work trace | 0 | NO_ITEMS | Records section is present. |
| C:[operative]:[consistency] | operative | consistency | stable process evidence | 0 | NO_ITEMS | No process contradiction found. |
| C:[evaluative]:[necessity] | evaluative | necessity | decision-critical basis | 0 | NO_ITEMS | Human decisions are marked TBD. |
| C:[evaluative]:[sufficiency] | evaluative | sufficiency | reviewable judgment basis | 0 | NO_ITEMS | Conflict table exists. |
| C:[evaluative]:[completeness] | evaluative | completeness | holistic appraisal record | 0 | NO_ITEMS | Guidance covers trade-offs. |
| C:[evaluative]:[consistency] | evaluative | consistency | coherent review rationale | 0 | NO_ITEMS | No severe review gap found. |

## Matrix F - Requirements

### Lens Coverage

| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| F:[normative]:[necessity] | normative | necessity | mandatory evidence basis | 1 | HAS_ITEMS | Verification acceptance detail is intentionally TBD. |
| F:[normative]:[sufficiency] | normative | sufficiency | enforceable context proof | 0 | NO_ITEMS | Requirement sources are cited. |
| F:[normative]:[completeness] | normative | completeness | bounded obligation trace | 0 | NO_ITEMS | Category coverage is present. |
| F:[normative]:[consistency] | normative | consistency | stable compliance record | 0 | NO_ITEMS | Boundary wording is consistent. |
| F:[operative]:[necessity] | operative | necessity | required execution inputs | 0 | NO_ITEMS | Prerequisites are present. |
| F:[operative]:[sufficiency] | operative | sufficiency | adequate workflow evidence | 0 | NO_ITEMS | Procedure is conservative. |
| F:[operative]:[completeness] | operative | completeness | complete execution record | 1 | HAS_ITEMS | Test harness/fixture details remain TBD. |
| F:[operative]:[consistency] | operative | consistency | repeatable process trace | 0 | NO_ITEMS | Reproducibility intent is present. |
| F:[evaluative]:[necessity] | evaluative | necessity | review basis evidence | 0 | NO_ITEMS | Review basis is source-grounded. |
| F:[evaluative]:[sufficiency] | evaluative | sufficiency | defensible appraisal context | 0 | NO_ITEMS | No unsupported compliance claim found. |
| F:[evaluative]:[completeness] | evaluative | completeness | complete review record | 0 | NO_ITEMS | Conflict table is present. |
| F:[evaluative]:[consistency] | evaluative | consistency | coherent decision trace | 0 | NO_ITEMS | No contradiction found. |

### Warranted Items

| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| F-001 | F:[normative]:[necessity] | VerificationGap | Specification | Specification | Add concrete schema-validation acceptance criteria after repository schema pattern is selected. | The Specification identifies schema validation but the exact local schema gate and `$id`/placement convention are not source-supported yet. | `Specification.md` | Verification; Standards | N/A | Keep verification method generic until implementation context supports exact criteria. | TBD |
| F-002 | F:[operative]:[completeness] | MissingSlot | Procedure | Procedure | Record TBD for reproducibility test harness, fixtures, and assertion scope. | Procedure requires reproducibility tests, but no accessible source defines the exact fixture policy or test harness for this deliverable. | `Procedure.md` | Steps; Verification | N/A | Preserve the test obligation and defer harness selection. | TBD |

## Matrix D - Objectives

### Lens Coverage

| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| D:[normative]:[guiding] | normative | guiding | resolved directive basis | 0 | NO_ITEMS | Scope/objective are stated. |
| D:[normative]:[applying] | normative | applying | enforceable practice closure | 0 | NO_ITEMS | Requirements are conservative. |
| D:[normative]:[judging] | normative | judging | compliance finding basis | 0 | NO_ITEMS | No compliance overclaim found. |
| D:[normative]:[reviewing] | normative | reviewing | audit-ready obligation record | 1 | HAS_ITEMS | PRD source text is cited indirectly but not locally accessible. |
| D:[operative]:[guiding] | operative | guiding | executable workflow basis | 0 | NO_ITEMS | Procedure gives a future workflow. |
| D:[operative]:[applying] | operative | applying | controlled execution closure | 0 | NO_ITEMS | No implementation evidence is claimed. |
| D:[operative]:[judging] | operative | judging | performance finding basis | 0 | NO_ITEMS | Verification is listed. |
| D:[operative]:[reviewing] | operative | reviewing | process evidence record | 0 | NO_ITEMS | Records are named. |
| D:[evaluative]:[guiding] | evaluative | guiding | value-framed decision basis | 0 | NO_ITEMS | Guidance principles are present. |
| D:[evaluative]:[applying] | evaluative | applying | justified merit closure | 0 | NO_ITEMS | Trade-offs are present. |
| D:[evaluative]:[judging] | evaluative | judging | worth finding basis | 0 | NO_ITEMS | Unsupported choices remain TBD. |
| D:[evaluative]:[reviewing] | evaluative | reviewing | quality review record | 0 | NO_ITEMS | Conflict table found no direct source conflict. |

### Warranted Items

| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| D-001 | D:[normative]:[reviewing] | TBD_Question | Guidance | Guidance | TBD: confirm whether PRD v0.2 text is accessible before enriching requirements beyond SOW-072. | Decomposition/register rows cite PRD v0.2 sections, but only the register/decomposition summaries were locally read. | `Guidance.md` | Considerations | N/A | Do not derive clause-level PRD detail without the actual source text. | TBD |

## Matrix X - Verification

### Lens Coverage

| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| X:[guiding]:[necessity] | guiding | necessity | directive evidence check | 0 | NO_ITEMS | Verification targets are present. |
| X:[guiding]:[sufficiency] | guiding | sufficiency | justified instruction basis | 0 | NO_ITEMS | Source basis is cited. |
| X:[guiding]:[completeness] | guiding | completeness | complete direction trace | 0 | NO_ITEMS | Traceability is adequate for setup. |
| X:[guiding]:[consistency] | guiding | consistency | stable directive record | 1 | HAS_ITEMS | Hash assertion detail remains TBD. |
| X:[applying]:[necessity] | applying | necessity | practice readiness proof | 0 | NO_ITEMS | Future implementation steps are listed. |
| X:[applying]:[sufficiency] | applying | sufficiency | execution context check | 0 | NO_ITEMS | No unsupported setup step found. |
| X:[applying]:[completeness] | applying | completeness | complete practice trace | 0 | NO_ITEMS | Records section names outputs. |
| X:[applying]:[consistency] | applying | consistency | repeatable action record | 0 | NO_ITEMS | Reproducibility intent is present. |
| X:[judging]:[necessity] | judging | necessity | finding evidence check | 0 | NO_ITEMS | Diagnostic/professional boundaries are captured. |
| X:[judging]:[sufficiency] | judging | sufficiency | defensible assessment basis | 0 | NO_ITEMS | No overclaim found. |
| X:[judging]:[completeness] | judging | completeness | complete determination trace | 0 | NO_ITEMS | Verification table is complete enough for setup. |
| X:[judging]:[consistency] | judging | consistency | stable finding record | 0 | NO_ITEMS | No contradiction found. |
| X:[reviewing]:[necessity] | reviewing | necessity | audit evidence check | 0 | NO_ITEMS | Review records are listed. |
| X:[reviewing]:[sufficiency] | reviewing | sufficiency | appraisal context proof | 0 | NO_ITEMS | Conflict table exists. |
| X:[reviewing]:[completeness] | reviewing | completeness | complete audit trace | 0 | NO_ITEMS | No additional setup item. |
| X:[reviewing]:[consistency] | reviewing | consistency | stable quality record | 0 | NO_ITEMS | No matrix error. |

### Warranted Items

| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| X-001 | X:[guiding]:[consistency] | VerificationGap | Specification | Specification | Add reproducibility-hash assertions after canonicalization library and payload partitions are chosen. | The Specification requires stable hash behavior, but `docs/SPEC.md` leaves non-JSON/binary partitioning TBD and the implementation library is not selected. | `Specification.md` | Requirements; Verification | N/A | Keep JSON/JCS-compatible basis and mark unresolved partitions TBD. | TBD |

## Matrix E - Evaluation

### Lens Coverage

| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| E:[guiding]:[data] | guiding | data | directive fact trace | 0 | NO_ITEMS | Datasheet facts are cited. |
| E:[guiding]:[information] | guiding | information | contextual direction basis | 0 | NO_ITEMS | Context envelope is present. |
| E:[guiding]:[knowledge] | guiding | knowledge | skilled instruction frame | 0 | NO_ITEMS | No unsupported instruction found. |
| E:[guiding]:[wisdom] | guiding | wisdom | principled direction frame | 0 | NO_ITEMS | Boundary principles are present. |
| E:[applying]:[data] | applying | data | action fact evidence | 0 | NO_ITEMS | Procedure keeps implementation TBD. |
| E:[applying]:[information] | applying | information | practical context trace | 0 | NO_ITEMS | Steps are source-bounded. |
| E:[applying]:[knowledge] | applying | knowledge | skilled execution basis | 0 | NO_ITEMS | No implementation claim made. |
| E:[applying]:[wisdom] | applying | wisdom | prudent action basis | 0 | NO_ITEMS | Protected/private boundaries are preserved. |
| E:[judging]:[data] | judging | data | finding fact basis | 0 | NO_ITEMS | Verification facts are present. |
| E:[judging]:[information] | judging | information | assessment context trace | 0 | NO_ITEMS | No conflict found. |
| E:[judging]:[knowledge] | judging | knowledge | expert determination basis | 0 | NO_ITEMS | Professional authority is reserved. |
| E:[judging]:[wisdom] | judging | wisdom | principled finding frame | 0 | NO_ITEMS | Unsupported choices remain TBD. |
| E:[reviewing]:[data] | reviewing | data | audit fact evidence | 0 | NO_ITEMS | Records are named. |
| E:[reviewing]:[information] | reviewing | information | quality context trace | 0 | NO_ITEMS | No review issue found. |
| E:[reviewing]:[knowledge] | reviewing | knowledge | expert appraisal basis | 0 | NO_ITEMS | No matrix issue. |
| E:[reviewing]:[wisdom] | reviewing | wisdom | principled audit frame | 1 | HAS_ITEMS | Result storage/reference rationale remains a design decision. |

### Warranted Items

| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| E-001 | E:[reviewing]:[wisdom] | RationaleGap | Guidance | Guidance | Add rationale after deciding whether run records embed results, reference result envelopes, or support both. | Guidance identifies the trade-off but no accessible source makes the design decision. | `Guidance.md` | Trade-offs | N/A | Treat representation as TBD until schema/API design is approved. | TBD |
