# Semantic Lensing Register: DEL-17-07 Conservative PCF subset exporter

**Generated:** 2026-05-18
**Deliverable Folder:** /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-07_Conservative PCF subset exporter

**Inputs Read:**
- _CONTEXT.md - /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-07_Conservative PCF subset exporter/_CONTEXT.md#context-del-17-07
- _STATUS.md - /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-07_Conservative PCF subset exporter/_STATUS.md#status-del-17-07
- _SEMANTIC.md - /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-07_Conservative PCF subset exporter/_SEMANTIC.md#conservative-pcf-subset-exporter-del-17-07-conservative-pcf-subset-exporter
- Datasheet.md - /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-07_Conservative PCF subset exporter/Datasheet.md#datasheet-del-17-07-conservative-pcf-subset-exporter
- Specification.md - /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-07_Conservative PCF subset exporter/Specification.md#specification-del-17-07-conservative-pcf-subset-exporter
- Guidance.md - /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-07_Conservative PCF subset exporter/Guidance.md#guidance-del-17-07-conservative-pcf-subset-exporter
- Procedure.md - /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-07_Conservative PCF subset exporter/Procedure.md#procedure-del-17-07-conservative-pcf-subset-exporter
- _REFERENCES.md - /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-07_Conservative PCF subset exporter/_REFERENCES.md#references-del-17-07-conservative-pcf-subset-exporter

**Purpose:** Apply `semantic-matrix-build` matrix cells as lenses over the production documents, capturing warranted enrichment inputs for a later enrichment pass.

## Summary

- Total warranted items: 9
- By document:
  - Datasheet: 2
  - Specification: 3
  - Guidance: 2
  - Procedure: 2
- By matrix:
  - A: 1  B: 2  C: 1  F: 2  D: 1  X: 1  E: 1
- By type:
  - Conflict: 1
  - VerificationGap: 3
  - MissingSlot: 2
  - WeakStatement: 0
  - RationaleGap: 1
  - Normalization: 1
  - TBD_Question: 1
  - MatrixError: 0
- Notable conflicts: 1
- Matrix parse errors: 0

## Matrix A - Orientation

### Lens Coverage
| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| A:[normative]:[guiding] | normative | guiding | prescriptive direction | 1 | HAS_ITEMS | Prescriptive subset direction is present, but the human-ruling conflict table leaves candidate scope versus selected target profile unresolved. |
| A:[normative]:[applying] | normative | applying | mandatory practice | 0 | NO_ITEMS | Mandatory practices for source authority, profile fields, writer limits, loss reporting, and fixture boundaries are expressed as shall requirements. |
| A:[normative]:[judging] | normative | judging | compliance determination | 0 | NO_ITEMS | Compliance posture is bounded by review for absence of release, compatibility, code-compliance, and professional-acceptance claims. |
| A:[normative]:[reviewing] | normative | reviewing | regulatory audit | 0 | NO_ITEMS | The documents explicitly exclude design-code and professional approval claims, so regulatory audit content is intentionally out of scope. |
| A:[operative]:[guiding] | operative | guiding | procedural direction | 0 | NO_ITEMS | Procedure steps sequence source basis, profile definition, subset selection, writer behavior, fixtures, and verification. |
| A:[operative]:[applying] | operative | applying | practical execution | 0 | NO_ITEMS | Later implementation actions are deferred and bounded to approved tranches rather than prescribed in this documentation pass. |
| A:[operative]:[judging] | operative | judging | performance assessment | 0 | NO_ITEMS | Performance judgment is framed through future deterministic emission and diagnostics checks, not current runtime metrics. |
| A:[operative]:[reviewing] | operative | reviewing | process audit | 0 | NO_ITEMS | Procedure verification and records identify the audit trail expected for profile, sidecar, diagnostics, loss report, and run records. |
| A:[evaluative]:[guiding] | evaluative | guiding | value orientation | 0 | NO_ITEMS | Guidance values narrowness, explicit defaults, identity preservation, cautious support semantics, and no overclaims. |
| A:[evaluative]:[applying] | evaluative | applying | merit application | 0 | NO_ITEMS | Trade-offs state the merits and costs of narrow profile, sidecar IDs, ambiguity blocking, and invented-only fixtures. |
| A:[evaluative]:[judging] | evaluative | judging | worth determination | 0 | NO_ITEMS | Successful PCF creation is characterized as export evidence only, preventing inflated worth claims. |
| A:[evaluative]:[reviewing] | evaluative | reviewing | quality appraisal | 0 | NO_ITEMS | Quality appraisal is tied to loss visibility and boundary-control review rather than informal compatibility confidence. |

### Warranted Items
| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| A-001 | A:[normative]:[guiding] | Conflict | Multi | Multi | Preserve the conflict as a human ruling item before profile support classes are finalized. | Guidance records a live conflict between the plan-listed initial subset and unresolved target-version/profile behavior; the Specification and Datasheet also keep target basis as TBD. | /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-07_Conservative PCF subset exporter/Guidance.md; /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-07_Conservative PCF subset exporter/Specification.md; /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-07_Conservative PCF subset exporter/Datasheet.md | Guidance.md#Conflict Table (for human ruling); Specification.md#Profile Requirements; Datasheet.md#Attributes | Guidance.md#Conflict Table (for human ruling); Specification.md#Profile Requirements; Datasheet.md#Attributes | PROPOSAL: Treat plan-listed subset as candidate scope only until the target profile/version decision is made. | TBD |

## Matrix B - Conceptualization

### Lens Coverage
| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| B:[data]:[necessity] | data | necessity | essential fact | 1 | HAS_ITEMS | Essential profile identity is blocked by the explicitly TBD target version basis. |
| B:[data]:[sufficiency] | data | sufficiency | adequate evidence | 0 | NO_ITEMS | Source basis is limited to DEL-17-01, DEL-17-02, the plan, public CAEPIPE-PCF evidence, and governance references. |
| B:[data]:[completeness] | data | completeness | comprehensive record | 0 | NO_ITEMS | Artifact expectations include profile, writer, unsupported behavior report, invented fixtures, manifest, sidecar IDs, diagnostics, and loss report. |
| B:[data]:[consistency] | data | consistency | reliable measurement | 1 | HAS_ITEMS | Unit policy is required, but the concrete PCF unit encoding and negative-test acceptance remain unresolved. |
| B:[information]:[necessity] | information | necessity | essential signal | 0 | NO_ITEMS | The key information signal is that unsupported, approximated, delegated, omitted, and TBD behavior must be visible. |
| B:[information]:[sufficiency] | information | sufficiency | adequate context | 0 | NO_ITEMS | Context states PCF is secondary interoperability and not the first validation backbone. |
| B:[information]:[completeness] | information | completeness | comprehensive account | 0 | NO_ITEMS | The documents cover source authority, profile, writer, loss reporting, fixtures, verification, and documentation expectations. |
| B:[information]:[consistency] | information | consistency | coherent message | 0 | NO_ITEMS | Messaging consistently rejects hidden translator defaults and overclaims across Datasheet, Specification, Guidance, and Procedure. |
| B:[knowledge]:[necessity] | knowledge | necessity | fundamental understanding | 0 | NO_ITEMS | Fundamental understanding is captured as translator fallbacks being target-path facts rather than local exporter defaults. |
| B:[knowledge]:[sufficiency] | knowledge | sufficiency | competent expertise | 0 | NO_ITEMS | Expertise boundaries are preserved by routing PCF behavior claims to admitted public or project-owned evidence. |
| B:[knowledge]:[completeness] | knowledge | completeness | thorough mastery | 0 | NO_ITEMS | Mastery is intentionally incomplete until source/profile questions are resolved, and that incompleteness is documented. |
| B:[knowledge]:[consistency] | knowledge | consistency | coherent understanding | 0 | NO_ITEMS | Support/restraint caution, sidecar identity, and loss reporting align across the four documents. |
| B:[wisdom]:[necessity] | wisdom | necessity | essential discernment | 0 | NO_ITEMS | Essential discernment is to block or report solver-relevant inferred behavior rather than silently carrying it forward. |
| B:[wisdom]:[sufficiency] | wisdom | sufficiency | adequate judgment | 0 | NO_ITEMS | Adequate judgment appears as narrow-profile trade-off language with explicit costs. |
| B:[wisdom]:[completeness] | wisdom | completeness | holistic insight | 0 | NO_ITEMS | Holistic insight is represented through data-boundary, identity, units, support semantics, and claim-boundary considerations. |
| B:[wisdom]:[consistency] | wisdom | consistency | principled reasoning | 0 | NO_ITEMS | Reasoning consistently prioritizes source evidence and professional-boundary controls over import convenience. |

### Warranted Items
| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| B-001 | B:[data]:[necessity] | TBD_Question | Multi | Datasheet | TBD: identify who or what source selects the first supported PCF target profile and target version basis. | The target version/profile basis is explicitly TBD in the Datasheet, Specification, and Procedure; a future exporter cannot finalize profile identity without this input. | /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-07_Conservative PCF subset exporter/Datasheet.md; /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-07_Conservative PCF subset exporter/Specification.md; /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-07_Conservative PCF subset exporter/Procedure.md | Datasheet.md#Attributes; Specification.md#Profile Requirements; Procedure.md#2. Define the PCF profile | NA | PROPOSAL: Source this from admitted public evidence or explicit human project authority. | TBD |
| B-002 | B:[data]:[consistency] | VerificationGap | Specification | Specification | Add acceptance criteria for unit policy and missing or ambiguous unit diagnostics once the PCF profile is selected. | The documents require unit policy and diagnostics for unit ambiguity, and the Datasheet notes PCF UNITS-BORE dependence, but the verification section does not yet define concrete pass/fail checks for unit representation. | /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-07_Conservative PCF subset exporter/Datasheet.md; /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-07_Conservative PCF subset exporter/Specification.md | Datasheet.md#Conditions; Specification.md#Writer Requirements; Specification.md#Verification | NA | PROPOSAL: Keep criteria profile-bound and avoid inventing PCF unit values in this register. | TBD |

## Matrix C - Formulation

### Lens Coverage
| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| C:[normative]:[necessity] | normative | necessity | Binding Evidence Basis | 0 | NO_ITEMS | Binding evidence basis is directed to DEL-17-01 and DEL-17-02 source-basis and export-contract authorities. |
| C:[normative]:[sufficiency] | normative | sufficiency | Adequate Rule Support | 0 | NO_ITEMS | Rule support is adequate for the Phase A documents because unsupported target behavior remains TBD, delegated, unsupported, or loss-reported. |
| C:[normative]:[completeness] | normative | completeness | Complete Control Record | 1 | HAS_ITEMS | The required profile classification record is named, but the candidate entity-family classification table is not yet present. |
| C:[normative]:[consistency] | normative | consistency | Coherent Compliance Basis | 0 | NO_ITEMS | Claim boundaries are coherent across scope, standards, guidance, procedure verification, and documentation sections. |
| C:[operative]:[necessity] | operative | necessity | Required Execution Evidence | 0 | NO_ITEMS | Required execution evidence is deferred to future invented fixtures, diagnostics, target file, manifest, sidecar map, and loss report. |
| C:[operative]:[sufficiency] | operative | sufficiency | Practical Context Basis | 0 | NO_ITEMS | Practical context identifies source readings and downstream dependency review before profile work. |
| C:[operative]:[completeness] | operative | completeness | Complete Process Account | 0 | NO_ITEMS | Procedure steps span source basis through records without adding implementation code in the current pass. |
| C:[operative]:[consistency] | operative | consistency | Consistent Workflow Message | 0 | NO_ITEMS | Workflow consistently says to emit supported profile content and report missing, ambiguous, unsupported, or delegated data. |
| C:[evaluative]:[necessity] | evaluative | necessity | Essential Appraisal Basis | 0 | NO_ITEMS | Appraisal basis centers on whether hidden defaults and unsupported semantics are exposed. |
| C:[evaluative]:[sufficiency] | evaluative | sufficiency | Adequate Merit Context | 0 | NO_ITEMS | Merit context is stated through trade-offs for narrow scope, sidecar IDs, ambiguity blocking, and invented fixtures. |
| C:[evaluative]:[completeness] | evaluative | completeness | Complete Quality Insight | 0 | NO_ITEMS | Quality insight includes identity, units, support/restraint caution, approximations, and data-boundary limits. |
| C:[evaluative]:[consistency] | evaluative | consistency | Coherent Value Reasoning | 0 | NO_ITEMS | Value reasoning remains aligned with source-grounded conservative export rather than downstream import assurance. |

### Warranted Items
| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| C-001 | C:[normative]:[completeness] | MissingSlot | Multi | Datasheet | Add a profile classification table slot for each candidate entity family and attribute class: exported, omitted, approximated, delegated, unsupported, or TBD. | Specification requires each candidate entity family to be classified, and Procedure step 2 repeats that task; the current Datasheet lists candidates but does not provide the classification record shape. | /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-07_Conservative PCF subset exporter/Datasheet.md; /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-07_Conservative PCF subset exporter/Specification.md; /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-07_Conservative PCF subset exporter/Procedure.md | Datasheet.md#Attributes; Specification.md#Profile Requirements; Procedure.md#2. Define the PCF profile | NA | PROPOSAL: Treat this as a future profile artifact slot, not as final classification content. | TBD |

## Matrix F - Requirements

### Lens Coverage
| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| F:[normative]:[necessity] | normative | necessity | Mandatory Evidence Need | 0 | NO_ITEMS | Mandatory evidence needs are stated through source authority requirements and public/project-owned source boundaries. |
| F:[normative]:[sufficiency] | normative | sufficiency | Sufficient Rule Context | 0 | NO_ITEMS | Rule context identifies DEL-17-01, DEL-17-02, the plan, CAEPIPE-PCF, and governance sources without expanding authority. |
| F:[normative]:[completeness] | normative | completeness | Complete Compliance Record | 0 | NO_ITEMS | Compliance record expectations include source basis, profile fields, loss report, fixture provenance, and boundary review. |
| F:[normative]:[consistency] | normative | consistency | Reliable Control Basis | 0 | NO_ITEMS | Control basis consistently rejects hidden defaults, proprietary inputs, code compliance claims, and professional acceptance claims. |
| F:[operative]:[necessity] | operative | necessity | Required Execution Proof | 0 | NO_ITEMS | Required execution proof is deferred to future implementation tests using invented fixtures. |
| F:[operative]:[sufficiency] | operative | sufficiency | Adequate Process Context | 1 | HAS_ITEMS | Procedure dependencies extend beyond the Datasheet reference basis, creating a reference/dependency normalization issue. |
| F:[operative]:[completeness] | operative | completeness | Complete Workflow Mastery | 1 | HAS_ITEMS | Diagnostics and loss-report tests are required, but acceptance coverage by diagnostic class is not yet fully specified. |
| F:[operative]:[consistency] | operative | consistency | Reliable Procedure Logic | 0 | NO_ITEMS | Procedure logic separates source-basis establishment, profile selection, subset selection, writer behavior, fixtures, and verification. |
| F:[evaluative]:[necessity] | evaluative | necessity | Essential Merit Evidence | 0 | NO_ITEMS | Merit evidence is tied to visible unsupported behavior and sidecar traceability rather than external-tool acceptance. |
| F:[evaluative]:[sufficiency] | evaluative | sufficiency | Adequate Value Judgment | 0 | NO_ITEMS | Value judgment is adequately framed by the trade-off table and no-overclaim principle. |
| F:[evaluative]:[completeness] | evaluative | completeness | Complete Appraisal Insight | 0 | NO_ITEMS | Appraisal insight includes units, OD/wall, loads, material labels, component mappings, free ends, and CAEPIPE limits. |
| F:[evaluative]:[consistency] | evaluative | consistency | Reliable Quality Reasoning | 0 | NO_ITEMS | Quality reasoning stays consistent with conservative interoperability and explicit loss reporting. |

### Warranted Items
| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| F-001 | F:[operative]:[sufficiency] | Normalization | Multi | Datasheet | Align the upstream dependency/reference list for DEL-17-03/13/15 inputs or explain why only DEL-17-01 and DEL-17-02 are source references. | Procedure prerequisites require review of DEL-17-02, DEL-03-02, DEL-13-04, and DEL-15-02, while the Datasheet references table only lists DEL-17-01 and DEL-17-02 as deliverable-local source inputs. | /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-07_Conservative PCF subset exporter/Procedure.md; /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-07_Conservative PCF subset exporter/Datasheet.md | Procedure.md#Prerequisites; Datasheet.md#References | NA | PROPOSAL: Clarify distinction between dependency prerequisites and source-reference authorities. | TBD |
| F-002 | F:[operative]:[completeness] | VerificationGap | Specification | Specification | Add a verification matrix covering diagnostics and loss-report expectations for missing units, coordinates, nominal size, OD/wall, material/spec labels, identity, end connections, support/restraint semantics, approximations, and TBD behavior. | Specification requires diagnostics for multiple data classes and loss report entries for omitted, approximated, delegated, unsupported, and TBD behavior, but verification currently summarizes these areas without enumerating acceptance coverage by class. | /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-07_Conservative PCF subset exporter/Specification.md; /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-07_Conservative PCF subset exporter/Procedure.md | Specification.md#Writer Requirements; Specification.md#Unsupported and Loss-Report Requirements; Specification.md#Verification; Procedure.md#6. Verify outputs | NA | PROPOSAL: Keep the matrix as acceptance scaffolding until implementation tranche details are sealed. | TBD |

## Matrix D - Closure

### Lens Coverage
| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| D:[normative]:[guiding] | normative | guiding | Directed Evidence Closure | 0 | NO_ITEMS | Directed closure points all PCF behavior claims back to admitted evidence and source-basis records. |
| D:[normative]:[applying] | normative | applying | Mandated Rule Closure | 0 | NO_ITEMS | Mandated closure exists through shall requirements for profile declaration, writer limits, loss reporting, and fixtures. |
| D:[normative]:[judging] | normative | judging | Compliance Record Closure | 0 | NO_ITEMS | Compliance record closure is bounded by no-completeness, no-compatibility, no-code-compliance, and no-professional-acceptance clauses. |
| D:[normative]:[reviewing] | normative | reviewing | Audit Control Closure | 0 | NO_ITEMS | Audit controls include boundary review and records for profile, target file, manifest, sidecar, diagnostics, and loss report. |
| D:[operative]:[guiding] | operative | guiding | Procedural Proof Closure | 0 | NO_ITEMS | Procedural proof is carried by the ordered six-step Procedure and its records section. |
| D:[operative]:[applying] | operative | applying | Execution Context Closure | 0 | NO_ITEMS | Execution context closure is intentionally deferred to a later implementation tranche rather than asserted in Phase A. |
| D:[operative]:[judging] | operative | judging | Performance Mastery Closure | 0 | NO_ITEMS | Performance mastery is not claimed; future tests focus deterministic emission and diagnostics. |
| D:[operative]:[reviewing] | operative | reviewing | Process Logic Closure | 0 | NO_ITEMS | Process logic closure is represented by fixture provenance, verification, and run-record expectations. |
| D:[evaluative]:[guiding] | evaluative | guiding | Value Evidence Closure | 0 | NO_ITEMS | Value evidence closes around conservative export usefulness without overstating downstream semantics. |
| D:[evaluative]:[applying] | evaluative | applying | Merit Judgment Closure | 0 | NO_ITEMS | Merit judgment is framed through explicit costs and benefits of conservative decisions. |
| D:[evaluative]:[judging] | evaluative | judging | Worth Insight Closure | 1 | HAS_ITEMS | Support/restraint worth judgments need clearer criteria for block, delegate, TBD, or reliable preservation. |
| D:[evaluative]:[reviewing] | evaluative | reviewing | Quality Reasoning Closure | 0 | NO_ITEMS | Quality reasoning closes with explicit limitation that PCF is conservative interoperability only. |

### Warranted Items
| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| D-001 | D:[evaluative]:[judging] | RationaleGap | Guidance | Guidance | Clarify criteria that distinguish blocked, delegated, TBD, unsupported, and reliably preserved support/restraint semantics. | Guidance cautions that support/restraint semantics require separate review and should not imply reliable preservation unless the profile proves it, but the decision criteria for each outcome are not yet explained. | /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-07_Conservative PCF subset exporter/Guidance.md; /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-07_Conservative PCF subset exporter/Specification.md | Guidance.md#Treat support and restraint semantics cautiously; Specification.md#Unsupported and Loss-Report Requirements | NA | PROPOSAL: Put rationale in Guidance and keep enforceable profile classifications in the profile artifact. | TBD |

## Matrix X - Verification

### Lens Coverage
| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| X:[guiding]:[necessity] | guiding | necessity | Directed Fact Basis | 0 | NO_ITEMS | Directed facts include source-basis consumption, plan-listed subset, and CAEPIPE-PCF caveats. |
| X:[guiding]:[sufficiency] | guiding | sufficiency | Contextual Direction Proof | 0 | NO_ITEMS | Direction proof is contextualized by PCF as secondary interoperability with translator fallback risk. |
| X:[guiding]:[completeness] | guiding | completeness | Mastered Direction Record | 0 | NO_ITEMS | Direction records list profile, manifest, sidecar, diagnostics, loss report, and fixture provenance. |
| X:[guiding]:[consistency] | guiding | consistency | Reliable Direction Logic | 0 | NO_ITEMS | Direction logic consistently refuses hidden defaults and unsupported compatibility claims. |
| X:[applying]:[necessity] | applying | necessity | Mandated Fact Proof | 0 | NO_ITEMS | Mandated fact proof is future-facing and tied to invented fixtures and deterministic writer output. |
| X:[applying]:[sufficiency] | applying | sufficiency | Executable Context Basis | 0 | NO_ITEMS | Executable context depends on the later sealed implementation tranche and selected profile. |
| X:[applying]:[completeness] | applying | completeness | Complete Practice Record | 0 | NO_ITEMS | Practice record requirements include target file, manifest, stable ID sidecar, diagnostics, and loss report. |
| X:[applying]:[consistency] | applying | consistency | Coherent Practice Logic | 0 | NO_ITEMS | Practice logic keeps canonical IDs separate from target-generated identity. |
| X:[judging]:[necessity] | judging | necessity | Compliance Fact Proof | 0 | NO_ITEMS | Compliance fact proof is limited to source-grounding and boundary controls, not engineering code compliance. |
| X:[judging]:[sufficiency] | judging | sufficiency | Assessment Context Basis | 1 | HAS_ITEMS | Verification context names future tests but lacks an explicit external-tool/import boundary check. |
| X:[judging]:[completeness] | judging | completeness | Complete Finding Record | 0 | NO_ITEMS | Finding records are expected to connect canonical IDs to emitted records or loss-report entries. |
| X:[judging]:[consistency] | judging | consistency | Coherent Determination Logic | 0 | NO_ITEMS | Determination logic distinguishes export evidence from solver validation and downstream compatibility. |
| X:[reviewing]:[necessity] | reviewing | necessity | Audit Fact Proof | 0 | NO_ITEMS | Audit fact proof is the absence of protected examples, standards data, release claims, compatibility claims, and professional claims. |
| X:[reviewing]:[sufficiency] | reviewing | sufficiency | Process Review Context | 0 | NO_ITEMS | Review context includes four-document existence, headings, source-grounding gaps, and claim-boundary checks. |
| X:[reviewing]:[completeness] | reviewing | completeness | Complete Audit Record | 0 | NO_ITEMS | Complete audit record expectations are enumerated in Procedure records and Specification documentation. |
| X:[reviewing]:[consistency] | reviewing | consistency | Reliable Appraisal Logic | 0 | NO_ITEMS | Appraisal logic remains reliable by treating unresolved future implementation verification as TBD. |

### Warranted Items
| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| X-001 | X:[judging]:[sufficiency] | VerificationGap | Specification | Procedure | Add a verification check that generated PCF export evidence is not treated as downstream import compatibility, solver validation, release readiness, code compliance, or professional acceptance. | Specification and Guidance repeatedly deny compatibility and validation overclaims, while Procedure verification focuses on emitted files and loss records; an explicit check would protect the boundary during future implementation. | /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-07_Conservative PCF subset exporter/Specification.md; /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-07_Conservative PCF subset exporter/Guidance.md; /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-07_Conservative PCF subset exporter/Procedure.md | Specification.md#Scope; Specification.md#Verification; Guidance.md#Avoid professional and compatibility overclaims; Procedure.md#6. Verify outputs | NA | PROPOSAL: Make this a future implementation QA assertion rather than an external-tool approval claim. | TBD |

## Matrix E - Evaluation

### Lens Coverage
| LensKey | RowLabel | ColLabel | LensValue | ItemCount | CoverageStatus | Notes |
|---|---|---|---|---:|---|---|
| E:[guiding]:[data] | guiding | data | Directed Fact Trace | 0 | NO_ITEMS | Directed fact trace is provided by Datasheet identification, attributes, conditions, construction, and references. |
| E:[guiding]:[information] | guiding | information | Context Signal Basis | 0 | NO_ITEMS | Context signal basis states PCF usefulness, secondary priority, and fallback risk. |
| E:[guiding]:[knowledge] | guiding | knowledge | Mastery Evidence Frame | 0 | NO_ITEMS | Mastery frame is intentionally limited to public/project-owned evidence and unresolved questions. |
| E:[guiding]:[wisdom] | guiding | wisdom | Reasoned Direction Insight | 0 | NO_ITEMS | Direction insight favors narrow profile and visible loss over hidden translator inference. |
| E:[applying]:[data] | applying | data | Practice Fact Trace | 0 | NO_ITEMS | Practice fact trace will be carried by emitted PCF records, manifest, sidecar ID map, diagnostics, and loss report. |
| E:[applying]:[information] | applying | information | Execution Signal Basis | 0 | NO_ITEMS | Execution signal basis is the profile classification and diagnostic behavior expected in later implementation. |
| E:[applying]:[knowledge] | applying | knowledge | Operational Mastery Frame | 0 | NO_ITEMS | Operational mastery remains future-tranche work after profile and writer scope are sealed. |
| E:[applying]:[wisdom] | applying | wisdom | Reasoned Practice Judgment | 0 | NO_ITEMS | Practice judgment blocks invented load/material/support values and routes ambiguity to diagnostics or loss report. |
| E:[judging]:[data] | judging | data | Finding Fact Trace | 0 | NO_ITEMS | Finding fact trace is expected through canonical IDs mapped to emitted records or loss-report entries. |
| E:[judging]:[information] | judging | information | Assessment Signal Basis | 0 | NO_ITEMS | Assessment signal basis is the presence of every unsupported, approximated, delegated, omitted, or TBD behavior in loss reporting. |
| E:[judging]:[knowledge] | judging | knowledge | Determination Mastery Frame | 0 | NO_ITEMS | Determination mastery is constrained by source-confirmed behavior and explicit target-generated identity separation. |
| E:[judging]:[wisdom] | judging | wisdom | Reasoned Compliance Judgment | 0 | NO_ITEMS | Compliance judgment is limited to export-contract and boundary compliance rather than code or professional approval. |
| E:[reviewing]:[data] | reviewing | data | Audit Fact Trace | 1 | HAS_ITEMS | Fixture provenance is required, but a concrete provenance record template is absent. |
| E:[reviewing]:[information] | reviewing | information | Review Signal Basis | 0 | NO_ITEMS | Review signal basis includes future checks for protected data, release claims, compatibility claims, and professional claims. |
| E:[reviewing]:[knowledge] | reviewing | knowledge | Appraisal Mastery Frame | 0 | NO_ITEMS | Appraisal mastery recognizes the current documents as Phase A scaffolding rather than implementation proof. |
| E:[reviewing]:[wisdom] | reviewing | wisdom | Reasoned Quality Judgment | 0 | NO_ITEMS | Quality judgment remains conservative by keeping unresolved future verification as TBD. |

### Warranted Items
| ItemID | LensKey | Type | AppliesToDoc | SuggestedEditDoc | CandidateInfo | WhyWarranted | SourcePath | SectionRef | Contenders | ProposedAuthority (PROPOSAL) | HumanRuling |
|---|---|---|---|---|---|---|---|---|---|---|---|
| E-001 | E:[reviewing]:[data] | MissingSlot | Procedure | Procedure | Add a fixture provenance record template covering invented model names, line IDs, component IDs, coordinates, unit-bearing values, redistribution status, and protected-source exclusions. | Specification and Procedure require invented or redistribution-safe fixture provenance, but neither provides a record template for the future fixture artifact. | /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-07_Conservative PCF subset exporter/Specification.md; /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-07_Conservative PCF subset exporter/Procedure.md; /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-07_Conservative PCF subset exporter/Guidance.md | Specification.md#Fixture and Data-Boundary Requirements; Procedure.md#5. Build invented fixtures; Guidance.md#Examples | NA | PROPOSAL: Keep the template descriptive and avoid protected standards, catalog, owner, vendor, or proprietary values. | TBD |
