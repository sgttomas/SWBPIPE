# Deliverable: DEL-03-03 Bend and elbow component model fields

**Generated:** 2026-04-30
**DECOMP_VARIANT:** SOFTWARE
**Perspective:** This deliverable shapes bend and elbow component records so geometry, user-entered SIFs, user-entered flexibility factors, provenance, and validation state can be carried without bundling protected engineering content. It must support later solver, rule, persistence, and adapter workflows while keeping missing data and source boundaries explicit.
**Framework:** Chirality Semantic Algebra
**Lens Boundary:** This semantic lens is for setup question-shaping only. It is not engineering authority, does not define final schema content, and does not certify analysis or code compliance.

**Inputs Read:**
- `_CONTEXT.md` - `./_CONTEXT.md#Context-DEL-03-03`
- `_STATUS.md` - `./_STATUS.md#Status-DEL-03-03-Bend-and-elbow-component-model-fields`
- `Datasheet.md` - `./Datasheet.md#Attributes`
- `Specification.md` - `./Specification.md#Requirements`
- `Guidance.md` - `./Guidance.md#Principles`
- `Procedure.md` - `./Procedure.md#Steps`
- `_REFERENCES.md` - `./_REFERENCES.md#Governing-References`

**Dispatch Trace:**
- `docs/_Decomposition/SOFTWARE_DECOMP.md` revision 0.4 rows `DEL-03-03`, `SOW-007`, `OBJ-004`, `PKG-03`, and architecture basis `AB-00-01`, `AB-00-02`, `AB-00-04`, `AB-00-06`, `AB-00-07`, `AB-00-08`.
- `docs/_Registers/Deliverables.csv` row `DEL-03-03`.
- `docs/_Registers/ScopeLedger.csv` row `SOW-007`.
- `docs/_Registers/ContextBudgetQA.csv` row `DEL-03-03`.
- `docs/CONTRACT.md` invariant slices for protected content, user-supplied data, provenance, unit awareness, invented rule examples, centerline mechanics, and agent draft boundaries.

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

## Matrix C - Formulation (3x4)

### Construction

`L_C(i,j) = sum_k (A(i,k) * B(k,j)); C(i,j) = I(row_i, col_j, L_C(i,j))`

Each interpreted cell used the required three-step pattern: axis anchor, projected contributors, and centroid selection. The cell terms below are semantic categories only; they are not field names or engineering requirements.

| | **necessity** | **sufficiency** | **completeness** | **consistency** |
|---|---|---|---|---|
| **normative** | authority premise boundary | evidence threshold rule | obligation coverage map | coherence control principle |
| **operative** | execution entry condition | action evidence standard | workflow coverage model | trace continuity rule |
| **evaluative** | value relevance test | merit evidence basis | appraisal coverage frame | quality coherence standard |

## Matrix F - Requirements (3x4)

### Construction

`L_F(i,j) = sum_k (C(i,k) * B(k,j)); F(i,j) = I(row_i, col_j, L_F(i,j))`

| | **necessity** | **sufficiency** | **completeness** | **consistency** |
|---|---|---|---|---|
| **normative** | required authority evidence | adequate boundary proof | full obligation trace | stable authority semantics |
| **operative** | required execution evidence | adequate action context | full workflow trace | stable process semantics |
| **evaluative** | required review evidence | adequate merit context | full appraisal trace | stable quality semantics |

## Matrix D - Objectives (3x4)

### Construction

Column positions map from F to D as: `guiding <- necessity`, `applying <- sufficiency`, `judging <- completeness`, `reviewing <- consistency`.

`L_D(i,j) = {A(i,j), resolution * F(i, mapped_j)}; D(i,j) = I(row_i, col_j, L_D(i,j))`

| | **guiding** | **applying** | **judging** | **reviewing** |
|---|---|---|---|---|
| **normative** | source boundary direction | protected-data practice | authority separation finding | audit-ready provenance |
| **operative** | procedure closure route | executable field practice | validation finding channel | process trace record |
| **evaluative** | value closure orientation | merit use discipline | worth finding boundary | quality appraisal record |

## Matrix K - Transpose of D (4x3)

### Construction

`K(i,j) = D(j,i)`

| | **normative** | **operative** | **evaluative** |
|---|---|---|---|
| **guiding** | source boundary direction | procedure closure route | value closure orientation |
| **applying** | protected-data practice | executable field practice | merit use discipline |
| **judging** | authority separation finding | validation finding channel | worth finding boundary |
| **reviewing** | audit-ready provenance | process trace record | quality appraisal record |

## Matrix G - Truncation of B (3x4)

### Construction

Remove the `wisdom` row from Matrix B.

| | **necessity** | **sufficiency** | **completeness** | **consistency** |
|---|---|---|---|---|
| **data** | essential fact | adequate evidence | comprehensive record | reliable measurement |
| **information** | essential signal | adequate context | comprehensive account | coherent message |
| **knowledge** | fundamental understanding | competent expertise | thorough mastery | coherent understanding |

## Matrix X - Verification (4x4)

### Construction

`L_X(i,j) = sum_k (K(i,k) * G(k,j)); X(i,j) = I(row_i, col_j, L_X(i,j))`

| | **necessity** | **sufficiency** | **completeness** | **consistency** |
|---|---|---|---|---|
| **guiding** | boundary premise check | boundary proof check | boundary coverage check | boundary coherence check |
| **applying** | practice readiness check | practice evidence check | practice coverage check | practice trace check |
| **judging** | finding readiness check | finding proof check | finding coverage check | finding coherence check |
| **reviewing** | record readiness check | record proof check | record coverage check | record coherence check |

## Matrix T - Transpose of B (4x4)

### Construction

`T(i,j) = B(j,i)`

| | **data** | **information** | **knowledge** | **wisdom** |
|---|---|---|---|---|
| **necessity** | essential fact | essential signal | fundamental understanding | essential discernment |
| **sufficiency** | adequate evidence | adequate context | competent expertise | adequate judgment |
| **completeness** | comprehensive record | comprehensive account | thorough mastery | holistic insight |
| **consistency** | reliable measurement | coherent message | coherent understanding | principled reasoning |

## Matrix E - Evaluation (4x4)

### Construction

`L_E(i,j) = sum_k (X(i,k) * T(k,j)); E(i,j) = I(row_i, col_j, L_E(i,j))`

| | **data** | **information** | **knowledge** | **wisdom** |
|---|---|---|---|---|
| **guiding** | fact boundary assurance | signal boundary assurance | understanding boundary assurance | discernment boundary assurance |
| **applying** | fact practice assurance | signal practice assurance | understanding practice assurance | discernment practice assurance |
| **judging** | fact finding assurance | signal finding assurance | understanding finding assurance | discernment finding assurance |
| **reviewing** | fact record assurance | signal record assurance | understanding record assurance | discernment record assurance |

## Interpretation Work Record

The semantic matrix build applied the same interpretation operation to each dot-product or addition cell:

| Operation | Step 1 - Axis anchor | Step 2 - Projected contributors | Step 3 - Centroid attractor |
|---|---|---|---|
| C | Row posture multiplied by conceptual column. | Orientation terms were projected through conceptual contributors. | Compact formulation terms were selected. |
| F | Row posture multiplied by requirement column. | Formulation terms were projected through conceptual contributors. | Requirement lens terms were selected. |
| D | Row posture multiplied by orientation column. | Canonical orientation terms were combined with resolution-conditioned F terms. | Objective lens terms were selected. |
| X | Orientation objective row multiplied by verification column. | K terms were projected through truncated conceptual contributors. | Verification lens terms were selected. |
| E | Verification row multiplied by evaluation column. | X terms were projected through transposed conceptual contributors. | Evaluation lens terms were selected. |

## Audit

- Matrix order present: A, B, C, F, D, K, G, X, T, E.
- Result dimensions present: C 3x4, F 3x4, D 3x4, K 4x3, G 3x4, X 4x4, T 4x4, E 4x4.
- Final cell values are compact semantic units.
- No protected engineering values, standards clauses, SIF/flexibility formulas, equipment tags, or numeric particulars are present.
- Audit result: PASS.
