# Deliverable: DEL-07-06 Accessibility and usability baseline

**Generated:** 2026-04-30
**DECOMP_VARIANT:** SOFTWARE
**Decomposition Path:** `/Users/ryan/ai-env/projects/chirality-piping/docs/_Decomposition/SOFTWARE_DECOMP.md`
**Perspective:** This deliverable frames accessibility and usability as a bounded baseline for engineering-review GUI and report-facing workflows. The matrices are a question-shaping lens only; they do not choose a final WCAG target, implement UI behavior, introduce protected data, or make professional compliance claims.
**Audit Result:** PASS

**Inputs Read:**
- `_CONTEXT.md` - deliverable identity, scope, objective, and architecture-basis injection.
- `_STATUS.md` - lifecycle state observed as `INITIALIZED` before this semantic run.
- `Datasheet.md` - setup attributes, conditions, and boundary notes.
- `Specification.md` - baseline requirements, standards status, and verification hooks.
- `Guidance.md` - principles, trade-offs, examples, and conflict table.
- `Procedure.md` - setup procedure, verification checks, and records.
- `_REFERENCES.md` - governing source list; references used as pointers only.

## Matrix A - Orientation (3x4) - Canonical

| | **guiding** | **applying** | **judging** | **reviewing** |
|---|---|---|---|---|
| **normative** | prescriptive direction | mandatory practice | compliance determination | regulatory audit |
| **operative** | procedural direction | practical execution | performance assessment | process audit |
| **evaluative** | value orientation | merit application | worth determination | quality appraisal |

## Matrix B - Conceptualization (4x4) - Canonical

| | **necessity** | **sufficiency** | **completeness** | **consistency** |
|---|---|---|---|---|
| **data** | essential fact | adequate evidence | comprehensive record | reliable measurement |
| **information** | essential signal | adequate context | comprehensive account | coherent message |
| **knowledge** | fundamental understanding | competent expertise | thorough mastery | coherent understanding |
| **wisdom** | essential discernment | adequate judgment | holistic insight | principled reasoning |

## Interpretation Work

All list-valued constructions below were interpreted with `I(row,column,L)` by: Step 1, forming the row-column anchor; Step 2, projecting each contributor through that anchor; Step 3, selecting a compact centroid phrase. The derivation records are intentionally semantic and do not introduce engineering values or implementation particulars.

### C trace example

For `C[normative,necessity]`, `L` contains four projected orientation/concept contributors from A and B. Step 1 anchor: `normative * necessity` establishes a binding-needed coordinate frame. Step 2 projections: direction/fact becomes access obligation, practice/signal becomes required status cue, determination/understanding becomes review basis, audit/discernment becomes accountability check. Step 3 centroid: `binding access basis`.

### F trace example

For `F[operative,completeness]`, `L` contains C/B contributors under execution coverage. Step 1 anchor: `operative * completeness` establishes a full-workflow coordinate frame. Step 2 projections: execution basis/record becomes workflow evidence, practice basis/account becomes task coverage, workflow frame/mastery becomes interaction coverage, process basis/insight becomes review continuity. Step 3 centroid: `complete interaction coverage`.

### D trace example

For `D[normative,applying]`, `L` contains the Matrix A cell plus the resolution-transformed Matrix F cell. Step 1 anchor: `normative * applying` establishes a binding-practice coordinate frame. Step 2 projections: mandatory practice becomes enforceable interaction, resolved assurance becomes boundary closure. Step 3 centroid: `enforceable interaction boundary`.

### X trace example

For `X[judging,necessity]`, `L` contains K/G contributors under decision evidence. Step 1 anchor: `judging * necessity` establishes a decision-needed coordinate frame. Step 2 projections: closure/fact becomes readiness evidence, performance/signal becomes status signal, appraisal/understanding becomes review basis. Step 3 centroid: `readiness evidence proof`.

### E trace example

For `E[reviewing,wisdom]`, `L` contains X/T contributors under audit judgment. Step 1 anchor: `reviewing * wisdom` establishes an audit-judgment coordinate frame. Step 2 projections: evidence/discernment becomes reliance record, assurance/judgment becomes acceptance boundary, coverage/insight becomes review completeness, coherence/reasoning becomes defensible trace. Step 3 centroid: `audit judgment record`.

## Matrix C - Formulation (3x4)

### Construction: Dot product A with B, then interpretation

| | **necessity** | **sufficiency** | **completeness** | **consistency** |
|---|---|---|---|---|
| **normative** | binding access basis | warranted review basis | complete visibility frame | coherent usability frame |
| **operative** | required navigation basis | usable execution basis | whole workflow frame | stable interaction frame |
| **evaluative** | critical review basis | defensible judgment basis | integral review frame | principled quality frame |

## Matrix F - Requirements (3x4)

### Construction: Dot product C with B, then interpretation

| | **necessity** | **sufficiency** | **completeness** | **consistency** |
|---|---|---|---|---|
| **normative** | mandatory access threshold | review assurance threshold | full visibility coverage | coherent baseline criteria |
| **operative** | required navigation evidence | adequate workflow proof | complete interaction coverage | stable workflow assurance |
| **evaluative** | essential review evidence | reasoned usability assurance | holistic review coverage | principled quality assurance |

## Matrix D - Objectives (3x4)

### Construction: Matrix A plus resolution-transformed F, then interpretation

| | **guiding** | **applying** | **judging** | **reviewing** |
|---|---|---|---|---|
| **normative** | governed access charter | enforceable interaction boundary | judged readiness basis | audit evidence record |
| **operative** | executable navigation charter | governed workflow protocol | measured usability basis | process evidence record |
| **evaluative** | value-aligned review frame | merit-grounded usability frame | defensible review basis | quality appraisal record |

## Matrix K - Transpose of D (4x3)

### Construction: K(i,j) = D(j,i)

| | **normative** | **operative** | **evaluative** |
|---|---|---|---|
| **guiding** | governed access charter | executable navigation charter | value-aligned review frame |
| **applying** | enforceable interaction boundary | governed workflow protocol | merit-grounded usability frame |
| **judging** | judged readiness basis | measured usability basis | defensible review basis |
| **reviewing** | audit evidence record | process evidence record | quality appraisal record |

## Matrix G - Truncation of B (3x4)

### Construction: remove `wisdom` row from B

| | **necessity** | **sufficiency** | **completeness** | **consistency** |
|---|---|---|---|---|
| **data** | essential fact | adequate evidence | comprehensive record | reliable measurement |
| **information** | essential signal | adequate context | comprehensive account | coherent message |
| **knowledge** | fundamental understanding | competent expertise | thorough mastery | coherent understanding |

## Matrix X - Verification (4x4)

### Construction: Dot product K with G, then interpretation

| | **necessity** | **sufficiency** | **completeness** | **consistency** |
|---|---|---|---|---|
| **guiding** | chartered access proof | charter assurance proof | charter coverage proof | charter coherence proof |
| **applying** | interaction evidence proof | workflow assurance proof | workflow coverage proof | workflow coherence proof |
| **judging** | readiness evidence proof | readiness assurance proof | status coverage proof | status coherence proof |
| **reviewing** | audit evidence proof | audit assurance proof | audit coverage proof | audit coherence proof |

## Matrix T - Transpose of B (4x4)

### Construction: T(i,j) = B(j,i)

| | **data** | **information** | **knowledge** | **wisdom** |
|---|---|---|---|---|
| **necessity** | essential fact | essential signal | fundamental understanding | essential discernment |
| **sufficiency** | adequate evidence | adequate context | competent expertise | adequate judgment |
| **completeness** | comprehensive record | comprehensive account | thorough mastery | holistic insight |
| **consistency** | reliable measurement | coherent message | coherent understanding | principled reasoning |

## Matrix E - Evaluation (4x4)

### Construction: Dot product X with T, then interpretation

| | **data** | **information** | **knowledge** | **wisdom** |
|---|---|---|---|---|
| **guiding** | guided fact review | guided signal review | guided expertise review | guided judgment review |
| **applying** | applied fact review | applied context review | applied expertise review | applied judgment review |
| **judging** | readiness fact finding | readiness context finding | readiness expertise finding | readiness judgment finding |
| **reviewing** | audit fact record | audit context record | audit expertise record | audit judgment record |

## Matrix Summary

### C - Formulation

| | **necessity** | **sufficiency** | **completeness** | **consistency** |
|---|---|---|---|---|
| **normative** | binding access basis | warranted review basis | complete visibility frame | coherent usability frame |
| **operative** | required navigation basis | usable execution basis | whole workflow frame | stable interaction frame |
| **evaluative** | critical review basis | defensible judgment basis | integral review frame | principled quality frame |

### F - Requirements

| | **necessity** | **sufficiency** | **completeness** | **consistency** |
|---|---|---|---|---|
| **normative** | mandatory access threshold | review assurance threshold | full visibility coverage | coherent baseline criteria |
| **operative** | required navigation evidence | adequate workflow proof | complete interaction coverage | stable workflow assurance |
| **evaluative** | essential review evidence | reasoned usability assurance | holistic review coverage | principled quality assurance |

### D - Objectives

| | **guiding** | **applying** | **judging** | **reviewing** |
|---|---|---|---|---|
| **normative** | governed access charter | enforceable interaction boundary | judged readiness basis | audit evidence record |
| **operative** | executable navigation charter | governed workflow protocol | measured usability basis | process evidence record |
| **evaluative** | value-aligned review frame | merit-grounded usability frame | defensible review basis | quality appraisal record |

### K - Transpose of D

| | **normative** | **operative** | **evaluative** |
|---|---|---|---|
| **guiding** | governed access charter | executable navigation charter | value-aligned review frame |
| **applying** | enforceable interaction boundary | governed workflow protocol | merit-grounded usability frame |
| **judging** | judged readiness basis | measured usability basis | defensible review basis |
| **reviewing** | audit evidence record | process evidence record | quality appraisal record |

### G - Truncation of B

| | **necessity** | **sufficiency** | **completeness** | **consistency** |
|---|---|---|---|---|
| **data** | essential fact | adequate evidence | comprehensive record | reliable measurement |
| **information** | essential signal | adequate context | comprehensive account | coherent message |
| **knowledge** | fundamental understanding | competent expertise | thorough mastery | coherent understanding |

### X - Verification

| | **necessity** | **sufficiency** | **completeness** | **consistency** |
|---|---|---|---|---|
| **guiding** | chartered access proof | charter assurance proof | charter coverage proof | charter coherence proof |
| **applying** | interaction evidence proof | workflow assurance proof | workflow coverage proof | workflow coherence proof |
| **judging** | readiness evidence proof | readiness assurance proof | status coverage proof | status coherence proof |
| **reviewing** | audit evidence proof | audit assurance proof | audit coverage proof | audit coherence proof |

### T - Transpose of B

| | **data** | **information** | **knowledge** | **wisdom** |
|---|---|---|---|---|
| **necessity** | essential fact | essential signal | fundamental understanding | essential discernment |
| **sufficiency** | adequate evidence | adequate context | competent expertise | adequate judgment |
| **completeness** | comprehensive record | comprehensive account | thorough mastery | holistic insight |
| **consistency** | reliable measurement | coherent message | coherent understanding | principled reasoning |

### E - Evaluation

| | **data** | **information** | **knowledge** | **wisdom** |
|---|---|---|---|---|
| **guiding** | guided fact review | guided signal review | guided expertise review | guided judgment review |
| **applying** | applied fact review | applied context review | applied expertise review | applied judgment review |
| **judging** | readiness fact finding | readiness context finding | readiness expertise finding | readiness judgment finding |
| **reviewing** | audit fact record | audit context record | audit expertise record | audit judgment record |

## Audit Notes

- Final matrix cell values are compact semantic phrases.
- No derived cell contains algebraic notation as a final value.
- No engineering dimensions, code-specific values, protected data, private project data, UI implementation choices, WCAG conformance level, or compliance claims are introduced.
