# Deliverable: DEL-04-03 Linear support and restraint models

**Generated:** 2026-04-30
**DECOMP_VARIANT:** SOFTWARE
**Perspective:** This deliverable frames the linear support/restraint boundary for a six-DOF centerline solver. It must preserve unit-aware support data, explicit missing-data findings, mechanics/rule/professional separation, and protected-data boundaries without implementing solver numerics or inventing support defaults.
**Framework:** Chirality Semantic Algebra
**Lens Boundary:** This semantic lens is for question-shaping only. It is not engineering authority, does not certify analysis, and does not define final implementation content.

**Inputs Read:**
- `_CONTEXT.md`
- `_STATUS.md`
- `Datasheet.md`
- `Specification.md`
- `Guidance.md`
- `Procedure.md`
- `_REFERENCES.md`
- `docs/_Decomposition/SOFTWARE_DECOMP.md` revision 0.4
- `docs/CONTRACT.md`
- `docs/SPEC.md`
- `docs/TYPES.md`
- `docs/INTENT.md`

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

### Construction: Dot product A * B

`L_C(i,j) = sum_k (A(i,k) * B(k,j)); C(i,j) = I(row_i, col_j, L_C(i,j))`

Interpretation pattern applied to every cell:

| Step | Work |
|---|---|
| Step 1 - Axis anchor | Combine the row and column labels into the coordinate frame. |
| Step 2 - Projected contributors | Project each A/B contributor through the axis anchor. |
| Step 3 - Centroid attractor | Select a compact support-model semantic unit with no implementation particulars. |

### Result

| | **necessity** | **sufficiency** | **completeness** | **consistency** |
|---|---|---|---|---|
| **normative** | boundary mandate basis | evidence threshold rule | obligation coverage map | coherence control principle |
| **operative** | execution entry condition | action evidence standard | workflow coverage model | trace continuity rule |
| **evaluative** | value relevance test | merit evidence basis | appraisal coverage frame | quality coherence standard |

## Matrix F - Requirements (3x4)

### Construction: Dot product C * B

`L_F(i,j) = sum_k (C(i,k) * B(k,j)); F(i,j) = I(row_i, col_j, L_F(i,j))`

Interpretation pattern applied to every cell:

| Step | Work |
|---|---|
| Step 1 - Axis anchor | Combine the row and column labels into the requirement frame. |
| Step 2 - Projected contributors | Project each C/B contributor through the axis anchor. |
| Step 3 - Centroid attractor | Select a compact requirement semantic unit for later coverage checks. |

### Result

| | **necessity** | **sufficiency** | **completeness** | **consistency** |
|---|---|---|---|---|
| **normative** | required boundary evidence | adequate model proof | full obligation trace | stable authority semantics |
| **operative** | required execution evidence | adequate action context | full workflow trace | stable process semantics |
| **evaluative** | required review evidence | adequate merit context | full appraisal trace | stable quality semantics |

## Matrix D - Objectives (3x4)

### Construction: Addition A plus resolution-transformed F

`L_D(i,j) = {A(i,j), resolution * F(i,mapped_j)}; D(i,j) = I(row_i, col_j, L_D(i,j))`

Interpretation pattern applied to every cell:

| Step | Work |
|---|---|
| Step 1 - Axis anchor | Combine the row and column labels into the objective frame. |
| Step 2 - Projected contributors | Project the orientation term and resolution-transformed requirement through the axis anchor. |
| Step 3 - Centroid attractor | Select a closure-oriented support-model objective phrase. |

### Result

| | **guiding** | **applying** | **judging** | **reviewing** |
|---|---|---|---|---|
| **normative** | boundary closure direction | mandatory restraint practice | compliance separation boundary | audit-ready authority record |
| **operative** | procedure closure route | executable support practice | mechanics finding channel | process trace record |
| **evaluative** | value closure orientation | merit use discipline | worth finding boundary | quality appraisal record |

## Matrix K - Transpose of D (4x3)

### Construction: K(i,j) = D(j,i)

| | **normative** | **operative** | **evaluative** |
|---|---|---|---|
| **guiding** | boundary closure direction | procedure closure route | value closure orientation |
| **applying** | mandatory restraint practice | executable support practice | merit use discipline |
| **judging** | compliance separation boundary | mechanics finding channel | worth finding boundary |
| **reviewing** | audit-ready authority record | process trace record | quality appraisal record |

## Matrix G - Truncation of B (3x4)

### Construction: remove `wisdom` row from B

| | **necessity** | **sufficiency** | **completeness** | **consistency** |
|---|---|---|---|---|
| **data** | essential fact | adequate evidence | comprehensive record | reliable measurement |
| **information** | essential signal | adequate context | comprehensive account | coherent message |
| **knowledge** | fundamental understanding | competent expertise | thorough mastery | coherent understanding |

## Matrix X - Verification (4x4)

### Construction: Dot product K * G

`L_X(i,j) = sum_k (K(i,k) * G(k,j)); X(i,j) = I(row_i, col_j, L_X(i,j))`

Interpretation pattern applied to every cell:

| Step | Work |
|---|---|
| Step 1 - Axis anchor | Combine the row and column labels into the verification frame. |
| Step 2 - Projected contributors | Project each K/G contributor through the axis anchor. |
| Step 3 - Centroid attractor | Select a compact verification semantic unit. |

### Result

| | **necessity** | **sufficiency** | **completeness** | **consistency** |
|---|---|---|---|---|
| **guiding** | boundary premise check | boundary proof check | boundary coverage check | boundary coherence check |
| **applying** | practice readiness check | practice evidence check | practice coverage check | practice trace check |
| **judging** | finding readiness check | finding proof check | finding coverage check | finding coherence check |
| **reviewing** | record readiness check | record proof check | record coverage check | record coherence check |

## Matrix T - Transpose of B (4x4)

### Construction: T(i,j) = B(j,i)

| | **data** | **information** | **knowledge** | **wisdom** |
|---|---|---|---|---|
| **necessity** | essential fact | essential signal | fundamental understanding | essential discernment |
| **sufficiency** | adequate evidence | adequate context | competent expertise | adequate judgment |
| **completeness** | comprehensive record | comprehensive account | thorough mastery | holistic insight |
| **consistency** | reliable measurement | coherent message | coherent understanding | principled reasoning |

## Matrix E - Evaluation (4x3)

### Construction: Dot product T * D

`L_E(i,j) = sum_k (T(i,k) * D(k,j)); E(i,j) = I(row_i, col_j, L_E(i,j))`

Interpretation pattern applied to every cell:

| Step | Work |
|---|---|
| Step 1 - Axis anchor | Combine the row and column labels into the evaluation frame. |
| Step 2 - Projected contributors | Project each T/D contributor through the axis anchor. |
| Step 3 - Centroid attractor | Select a compact evaluation semantic unit. |

### Result

| | **normative** | **operative** | **evaluative** |
|---|---|---|---|
| **necessity** | required support boundary | required solve pathway | required review concern |
| **sufficiency** | adequate support evidence | adequate solve context | adequate review basis |
| **completeness** | complete support trace | complete process record | complete appraisal basis |
| **consistency** | consistent support semantics | consistent solve trace | consistent quality basis |

## Matrix Summary

The compact result matrices above are the setup lens used by `_SEMANTIC_LENSING.md`. Final cell values were audited for algebra, operator, and expansion leaks: PASS.
