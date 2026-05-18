# Deliverable: DEL-03-06 Expansion joint component model

**Generated:** 2026-04-30
**DECOMP_VARIANT:** SOFTWARE
**Perspective:** This deliverable frames expansion joint component data as supplied, unit-aware, provenance-bearing model information. Its semantic role is to separate required data categories from unknown particulars so later implementation can validate fields without inventing manufacturer values or protected defaults.
**Framework:** Chirality Semantic Algebra
**Inputs Read:**
- _CONTEXT.md - deliverable identity, architecture basis, and package scope
- _STATUS.md - lifecycle state
- Datasheet.md - setup data categories
- Specification.md - setup requirements and verification hooks
- Guidance.md - constraints and trade-offs
- Procedure.md - setup-to-implementation workflow
- _REFERENCES.md - setup source pointers

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

## Derivation Work Convention

For each interpreted cell below, the same three-step interpretation method was applied: Step 1 axis anchor, Step 2 coordinate-conditioned projections for every contributor in the listed collection, and Step 3 centroid selection into one compact semantic unit. This setup evidence records representative collections and final centroids; it does not create engineering authority or project particulars.

## Matrix C - Formulation (3x4)

### Construction: Dot product A with B

| Cell | Intermediate collection L | I(r,c,L) summary |
|---|---|---|
| C:normative:necessity | prescriptive essential fact; mandatory essential signal; compliance fundamental understanding; audit essential discernment | anchor = frame obligation; projections converge on binding input need; centroid = source obligation |
| C:normative:sufficiency | prescriptive adequate evidence; mandatory adequate context; compliance competent expertise; audit adequate judgment | anchor = adequacy obligation; projections converge on defensible evidence; centroid = evidence threshold |
| C:normative:completeness | prescriptive comprehensive record; mandatory comprehensive account; compliance thorough mastery; audit holistic insight | anchor = closure obligation; projections converge on complete trace; centroid = trace completeness |
| C:normative:consistency | prescriptive reliable measurement; mandatory coherent message; compliance coherent understanding; audit principled reasoning | anchor = coherence obligation; projections converge on controlled agreement; centroid = controlled coherence |
| C:operative:necessity | procedural essential fact; practical essential signal; performance fundamental understanding; process essential discernment | anchor = execution need; projections converge on action input; centroid = required input |
| C:operative:sufficiency | procedural adequate evidence; practical adequate context; performance competent expertise; process adequate judgment | anchor = execution adequacy; projections converge on workable proof; centroid = workable basis |
| C:operative:completeness | procedural comprehensive record; practical comprehensive account; performance thorough mastery; process holistic insight | anchor = execution closure; projections converge on covered workflow; centroid = workflow coverage |
| C:operative:consistency | procedural reliable measurement; practical coherent message; performance coherent understanding; process principled reasoning | anchor = execution coherence; projections converge on stable process; centroid = process coherence |
| C:evaluative:necessity | value essential fact; merit essential signal; worth fundamental understanding; quality essential discernment | anchor = review need; projections converge on decision input; centroid = decision need |
| C:evaluative:sufficiency | value adequate evidence; merit adequate context; worth competent expertise; quality adequate judgment | anchor = review adequacy; projections converge on judgment support; centroid = judgment support |
| C:evaluative:completeness | value comprehensive record; merit comprehensive account; worth thorough mastery; quality holistic insight | anchor = review closure; projections converge on audit coverage; centroid = review coverage |
| C:evaluative:consistency | value reliable measurement; merit coherent message; worth coherent understanding; quality principled reasoning | anchor = review coherence; projections converge on stable appraisal; centroid = appraisal coherence |

### Result

| | **necessity** | **sufficiency** | **completeness** | **consistency** |
|---|---|---|---|---|
| **normative** | source obligation | evidence threshold | trace completeness | controlled coherence |
| **operative** | required input | workable basis | workflow coverage | process coherence |
| **evaluative** | decision need | judgment support | review coverage | appraisal coherence |

## Matrix F - Requirements (3x4)

### Construction: Dot product C with B

| Cell | Intermediate collection L | I(r,c,L) summary |
|---|---|---|
| F:normative:necessity | source essential fact; evidence essential signal; trace fundamental understanding; controlled essential discernment | anchor = obligation need; projections converge on required source; centroid = source requirement |
| F:normative:sufficiency | source adequate evidence; evidence adequate context; trace competent expertise; controlled adequate judgment | anchor = obligation adequacy; projections converge on accepted proof; centroid = acceptance basis |
| F:normative:completeness | source comprehensive record; evidence comprehensive account; trace thorough mastery; controlled holistic insight | anchor = obligation closure; projections converge on record closure; centroid = record closure |
| F:normative:consistency | source reliable measurement; evidence coherent message; trace coherent understanding; controlled principled reasoning | anchor = obligation coherence; projections converge on normalized control; centroid = normalized control |
| F:operative:necessity | input essential fact; basis essential signal; coverage fundamental understanding; coherence essential discernment | anchor = action need; projections converge on necessary input; centroid = input requirement |
| F:operative:sufficiency | input adequate evidence; basis adequate context; coverage competent expertise; coherence adequate judgment | anchor = action adequacy; projections converge on usable basis; centroid = usable basis |
| F:operative:completeness | input comprehensive record; basis comprehensive account; coverage thorough mastery; coherence holistic insight | anchor = action closure; projections converge on complete workflow; centroid = complete workflow |
| F:operative:consistency | input reliable measurement; basis coherent message; coverage coherent understanding; coherence principled reasoning | anchor = action coherence; projections converge on stable execution; centroid = stable execution |
| F:evaluative:necessity | decision essential fact; judgment essential signal; review fundamental understanding; appraisal essential discernment | anchor = review need; projections converge on evaluation input; centroid = evaluation input |
| F:evaluative:sufficiency | decision adequate evidence; judgment adequate context; review competent expertise; appraisal adequate judgment | anchor = review adequacy; projections converge on defensible judgment; centroid = defensible judgment |
| F:evaluative:completeness | decision comprehensive record; judgment comprehensive account; review thorough mastery; appraisal holistic insight | anchor = review closure; projections converge on full review; centroid = full review |
| F:evaluative:consistency | decision reliable measurement; judgment coherent message; review coherent understanding; appraisal principled reasoning | anchor = review coherence; projections converge on consistent review; centroid = consistent review |

### Result

| | **necessity** | **sufficiency** | **completeness** | **consistency** |
|---|---|---|---|---|
| **normative** | source requirement | acceptance basis | record closure | normalized control |
| **operative** | input requirement | usable basis | complete workflow | stable execution |
| **evaluative** | evaluation input | defensible judgment | full review | consistent review |

## Matrix D - Objectives (3x4)

### Construction: Addition A with resolution-transformed F

| Cell | Intermediate collection L | I(r,c,L) summary |
|---|---|---|
| D:normative:guiding | prescriptive direction; resolved source requirement | anchor = governing purpose; projections converge on bounded direction; centroid = bounded directive |
| D:normative:applying | mandatory practice; resolved acceptance basis | anchor = governing practice; projections converge on enforceable use; centroid = enforceable practice |
| D:normative:judging | compliance determination; resolved record closure | anchor = governing judgment; projections converge on decision closure; centroid = closure determination |
| D:normative:reviewing | regulatory audit; resolved normalized control | anchor = governing review; projections converge on controlled audit; centroid = control review |
| D:operative:guiding | procedural direction; resolved input requirement | anchor = action purpose; projections converge on execution route; centroid = execution route |
| D:operative:applying | practical execution; resolved usable basis | anchor = action practice; projections converge on implementable operation; centroid = implementable operation |
| D:operative:judging | performance assessment; resolved complete workflow | anchor = action judgment; projections converge on workflow assessment; centroid = workflow assessment |
| D:operative:reviewing | process audit; resolved stable execution | anchor = action review; projections converge on process assurance; centroid = process assurance |
| D:evaluative:guiding | value orientation; resolved evaluation input | anchor = value purpose; projections converge on review intent; centroid = review intent |
| D:evaluative:applying | merit application; resolved defensible judgment | anchor = value practice; projections converge on judgment use; centroid = judgment use |
| D:evaluative:judging | worth determination; resolved full review | anchor = value judgment; projections converge on review decision; centroid = review decision |
| D:evaluative:reviewing | quality appraisal; resolved consistent review | anchor = value review; projections converge on appraisal control; centroid = appraisal control |

### Result

| | **guiding** | **applying** | **judging** | **reviewing** |
|---|---|---|---|---|
| **normative** | bounded directive | enforceable practice | closure determination | control review |
| **operative** | execution route | implementable operation | workflow assessment | process assurance |
| **evaluative** | review intent | judgment use | review decision | appraisal control |

## Matrix K - Transpose of D (4x3)

### Construction: K(i,j) = D(j,i)

### Result

| | **normative** | **operative** | **evaluative** |
|---|---|---|---|
| **guiding** | bounded directive | execution route | review intent |
| **applying** | enforceable practice | implementable operation | judgment use |
| **judging** | closure determination | workflow assessment | review decision |
| **reviewing** | control review | process assurance | appraisal control |

## Matrix G - Truncation of B (3x4)

### Construction: remove wisdom row from B

### Result

| | **necessity** | **sufficiency** | **completeness** | **consistency** |
|---|---|---|---|---|
| **data** | essential fact | adequate evidence | comprehensive record | reliable measurement |
| **information** | essential signal | adequate context | comprehensive account | coherent message |
| **knowledge** | fundamental understanding | competent expertise | thorough mastery | coherent understanding |

## Matrix X - Verification (4x4)

### Construction: Dot product K with G

| Cell | Intermediate collection L | I(r,c,L) summary |
|---|---|---|
| X:guiding:necessity | directive fact; route signal; intent understanding | anchor = purpose need; projections converge on required check; centroid = required check |
| X:guiding:sufficiency | directive evidence; route context; intent expertise | anchor = purpose adequacy; projections converge on enough proof; centroid = proof check |
| X:guiding:completeness | directive record; route account; intent mastery | anchor = purpose closure; projections converge on coverage check; centroid = coverage check |
| X:guiding:consistency | directive measurement; route message; intent understanding | anchor = purpose coherence; projections converge on alignment check; centroid = alignment check |
| X:applying:necessity | practice fact; operation signal; use understanding | anchor = practice need; projections converge on input check; centroid = input check |
| X:applying:sufficiency | practice evidence; operation context; use expertise | anchor = practice adequacy; projections converge on usability check; centroid = usability check |
| X:applying:completeness | practice record; operation account; use mastery | anchor = practice closure; projections converge on implementation check; centroid = implementation check |
| X:applying:consistency | practice measurement; operation message; use understanding | anchor = practice coherence; projections converge on normalization check; centroid = normalization check |
| X:judging:necessity | closure fact; assessment signal; decision understanding | anchor = decision need; projections converge on finding check; centroid = finding check |
| X:judging:sufficiency | closure evidence; assessment context; decision expertise | anchor = decision adequacy; projections converge on acceptance check; centroid = acceptance check |
| X:judging:completeness | closure record; assessment account; decision mastery | anchor = decision closure; projections converge on decision coverage; centroid = decision coverage |
| X:judging:consistency | closure measurement; assessment message; decision understanding | anchor = decision coherence; projections converge on decision alignment; centroid = decision alignment |
| X:reviewing:necessity | control fact; assurance signal; appraisal understanding | anchor = review need; projections converge on audit check; centroid = audit check |
| X:reviewing:sufficiency | control evidence; assurance context; appraisal expertise | anchor = review adequacy; projections converge on review proof; centroid = review proof |
| X:reviewing:completeness | control record; assurance account; appraisal mastery | anchor = review closure; projections converge on audit coverage; centroid = audit coverage |
| X:reviewing:consistency | control measurement; assurance message; appraisal understanding | anchor = review coherence; projections converge on audit alignment; centroid = audit alignment |

### Result

| | **necessity** | **sufficiency** | **completeness** | **consistency** |
|---|---|---|---|---|
| **guiding** | required check | proof check | coverage check | alignment check |
| **applying** | input check | usability check | implementation check | normalization check |
| **judging** | finding check | acceptance check | decision coverage | decision alignment |
| **reviewing** | audit check | review proof | audit coverage | audit alignment |

## Matrix T - Transpose of B (4x4)

### Construction: T(i,j) = B(j,i)

### Result

| | **data** | **information** | **knowledge** | **wisdom** |
|---|---|---|---|---|
| **necessity** | essential fact | essential signal | fundamental understanding | essential discernment |
| **sufficiency** | adequate evidence | adequate context | competent expertise | adequate judgment |
| **completeness** | comprehensive record | comprehensive account | thorough mastery | holistic insight |
| **consistency** | reliable measurement | coherent message | coherent understanding | principled reasoning |

## Matrix E - Evaluation (4x4)

### Construction: Dot product X with T

| Cell | Intermediate collection L | I(r,c,L) summary |
|---|---|---|
| E:guiding:data | check fact; proof evidence; coverage record; alignment measurement | anchor = purpose data; projections converge on data trace; centroid = data trace |
| E:guiding:information | check signal; proof context; coverage account; alignment message | anchor = purpose information; projections converge on information trace; centroid = information trace |
| E:guiding:knowledge | check understanding; proof expertise; coverage mastery; alignment understanding | anchor = purpose knowledge; projections converge on knowledge trace; centroid = knowledge trace |
| E:guiding:wisdom | check discernment; proof judgment; coverage insight; alignment reasoning | anchor = purpose judgment; projections converge on judgment trace; centroid = judgment trace |
| E:applying:data | input fact; usability evidence; implementation record; normalization measurement | anchor = practice data; projections converge on field evidence; centroid = field evidence |
| E:applying:information | input signal; usability context; implementation account; normalization message | anchor = practice information; projections converge on interface evidence; centroid = interface evidence |
| E:applying:knowledge | input understanding; usability expertise; implementation mastery; normalization understanding | anchor = practice knowledge; projections converge on implementation evidence; centroid = implementation evidence |
| E:applying:wisdom | input discernment; usability judgment; implementation insight; normalization reasoning | anchor = practice judgment; projections converge on judgment evidence; centroid = judgment evidence |
| E:judging:data | finding fact; acceptance evidence; decision record; alignment measurement | anchor = decision data; projections converge on finding evidence; centroid = finding evidence |
| E:judging:information | finding signal; acceptance context; decision account; alignment message | anchor = decision information; projections converge on acceptance context; centroid = acceptance context |
| E:judging:knowledge | finding understanding; acceptance expertise; decision mastery; alignment understanding | anchor = decision knowledge; projections converge on decision rationale; centroid = decision rationale |
| E:judging:wisdom | finding discernment; acceptance judgment; decision insight; alignment reasoning | anchor = decision judgment; projections converge on acceptance rationale; centroid = acceptance rationale |
| E:reviewing:data | audit fact; proof evidence; coverage record; alignment measurement | anchor = review data; projections converge on audit evidence; centroid = audit evidence |
| E:reviewing:information | audit signal; proof context; coverage account; alignment message | anchor = review information; projections converge on review context; centroid = review context |
| E:reviewing:knowledge | audit understanding; proof expertise; coverage mastery; alignment understanding | anchor = review knowledge; projections converge on audit rationale; centroid = audit rationale |
| E:reviewing:wisdom | audit discernment; proof judgment; coverage insight; alignment reasoning | anchor = review judgment; projections converge on review rationale; centroid = review rationale |

### Result

| | **data** | **information** | **knowledge** | **wisdom** |
|---|---|---|---|---|
| **guiding** | data trace | information trace | knowledge trace | judgment trace |
| **applying** | field evidence | interface evidence | implementation evidence | judgment evidence |
| **judging** | finding evidence | acceptance context | decision rationale | acceptance rationale |
| **reviewing** | audit evidence | review context | audit rationale | review rationale |

---

## Matrix Summary

### C - Formulation

| | **necessity** | **sufficiency** | **completeness** | **consistency** |
|---|---|---|---|---|
| **normative** | source obligation | evidence threshold | trace completeness | controlled coherence |
| **operative** | required input | workable basis | workflow coverage | process coherence |
| **evaluative** | decision need | judgment support | review coverage | appraisal coherence |

### F - Requirements

| | **necessity** | **sufficiency** | **completeness** | **consistency** |
|---|---|---|---|---|
| **normative** | source requirement | acceptance basis | record closure | normalized control |
| **operative** | input requirement | usable basis | complete workflow | stable execution |
| **evaluative** | evaluation input | defensible judgment | full review | consistent review |

### D - Objectives

| | **guiding** | **applying** | **judging** | **reviewing** |
|---|---|---|---|---|
| **normative** | bounded directive | enforceable practice | closure determination | control review |
| **operative** | execution route | implementable operation | workflow assessment | process assurance |
| **evaluative** | review intent | judgment use | review decision | appraisal control |

### K - Transpose of D

| | **normative** | **operative** | **evaluative** |
|---|---|---|---|
| **guiding** | bounded directive | execution route | review intent |
| **applying** | enforceable practice | implementable operation | judgment use |
| **judging** | closure determination | workflow assessment | review decision |
| **reviewing** | control review | process assurance | appraisal control |

### G - Truncation of B

| | **necessity** | **sufficiency** | **completeness** | **consistency** |
|---|---|---|---|---|
| **data** | essential fact | adequate evidence | comprehensive record | reliable measurement |
| **information** | essential signal | adequate context | comprehensive account | coherent message |
| **knowledge** | fundamental understanding | competent expertise | thorough mastery | coherent understanding |

### X - Verification

| | **necessity** | **sufficiency** | **completeness** | **consistency** |
|---|---|---|---|---|
| **guiding** | required check | proof check | coverage check | alignment check |
| **applying** | input check | usability check | implementation check | normalization check |
| **judging** | finding check | acceptance check | decision coverage | decision alignment |
| **reviewing** | audit check | review proof | audit coverage | audit alignment |

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
| **guiding** | data trace | information trace | knowledge trace | judgment trace |
| **applying** | field evidence | interface evidence | implementation evidence | judgment evidence |
| **judging** | finding evidence | acceptance context | decision rationale | acceptance rationale |
| **reviewing** | audit evidence | review context | audit rationale | review rationale |

## Audit

- Final result cells are populated.
- Final result cells are compact semantic units.
- Final result cells do not contain intermediate algebra notation.
- No engineering particulars, manufacturer values, or protected defaults are introduced.
