# Semantic Lens: DEL-04-04 Nonlinear support active-set solver

**Generated:** 2026-04-30
**DECOMP_VARIANT:** SOFTWARE
**Perspective:** This deliverable frames nonlinear support behavior as a bounded solver feature for iterative active-set state, convergence reporting, diagnostics, and deterministic verification. The lens separates support mechanics and reporting obligations from compliance judgment, invented defaults, and implementation-level numerical choices.
**Framework:** Chirality Semantic Algebra
**Inputs Read:**
- _CONTEXT.md - deliverable identity, scope, objective, and architecture basis
- _STATUS.md - lifecycle state before semantic update
- Datasheet.md - production document
- Specification.md - production document
- Guidance.md - production document
- Procedure.md - production document
- _REFERENCES.md - governing reference list

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

### Construction: Dot product A dot B

For each cell, `L_C(row,col)` is interpreted through three steps: Step 1 axis anchor, Step 2 projected contributors, Step 3 centroid selection. Final cells are compact semantic units and not engineering authority.

| Cell | L_C contributors | Step 1 axis anchor | Step 2 projections | Step 3 centroid |
|---|---|---|---|---|
| C:normative:necessity | prescriptive fact; mandatory signal; compliance understanding; audit discernment | binding need | required fact; required signal; bounded judgment; review discernment | required evidence frame |
| C:normative:sufficiency | prescriptive evidence; mandatory context; compliance expertise; audit judgment | binding adequacy | adequate proof; adequate context; competent boundary; judged adequacy | adequate proof basis |
| C:normative:completeness | prescriptive record; mandatory account; compliance mastery; audit insight | binding coverage | complete record; complete account; bounded mastery; audit insight | complete obligation record |
| C:normative:consistency | prescriptive measurement; mandatory message; compliance understanding; audit reasoning | binding coherence | reliable measure; coherent instruction; stable boundary; reasoned review | coherent rule record |
| C:operative:necessity | procedural fact; execution signal; performance understanding; process discernment | action need | needed input; execution signal; practical understanding; process discernment | actionable input basis |
| C:operative:sufficiency | procedural evidence; execution context; performance expertise; process judgment | action adequacy | usable evidence; adequate context; competent execution; judged process | usable execution context |
| C:operative:completeness | procedural record; execution account; performance mastery; process insight | action coverage | complete steps; complete account; thorough execution; process insight | complete process account |
| C:operative:consistency | procedural measurement; execution message; performance understanding; process reasoning | action coherence | reliable measurement; coherent workflow; stable understanding; reasoned process | stable execution signal |
| C:evaluative:necessity | value fact; merit signal; worth understanding; quality discernment | appraisal need | salient fact; merit signal; worth basis; quality discernment | appraisal evidence need |
| C:evaluative:sufficiency | value evidence; merit context; worth expertise; quality judgment | appraisal adequacy | adequate value proof; merit context; expert worth; quality judgment | sufficient review basis |
| C:evaluative:completeness | value record; merit account; worth mastery; quality insight | appraisal coverage | value record; merit account; thorough worth; quality insight | complete review record |
| C:evaluative:consistency | value measurement; merit message; worth understanding; quality reasoning | appraisal coherence | measured value; coherent merit; worth coherence; quality reasoning | consistent review rationale |

### Result

| | **necessity** | **sufficiency** | **completeness** | **consistency** |
|---|---|---|---|---|
| **normative** | required evidence frame | adequate proof basis | complete obligation record | coherent rule record |
| **operative** | actionable input basis | usable execution context | complete process account | stable execution signal |
| **evaluative** | appraisal evidence need | sufficient review basis | complete review record | consistent review rationale |

## Matrix F - Requirements (3x4)

### Construction: Dot product C dot B

Each cell applies the same three-step interpretation procedure to `L_F(row,col)`.

| Cell | L_F contributors | Step 1 axis anchor | Step 2 projections | Step 3 centroid |
|---|---|---|---|---|
| F:normative:necessity | evidence fact; proof signal; obligation understanding; rule discernment | binding need | required fact; proof signal; obligation comprehension; rule discernment | required state evidence |
| F:normative:sufficiency | evidence proof; proof context; obligation expertise; rule judgment | binding adequacy | adequate evidence; contextual proof; expert obligation; judged rule | sufficient solver proof |
| F:normative:completeness | evidence record; proof account; obligation mastery; rule insight | binding coverage | complete evidence; proof account; obligation coverage; rule insight | complete behavior coverage |
| F:normative:consistency | evidence measure; proof message; obligation understanding; rule reasoning | binding coherence | measured evidence; coherent proof; stable obligation; rule reasoning | consistent state semantics |
| F:operative:necessity | input fact; context signal; process understanding; execution discernment | action need | required input; context signal; process comprehension; execution discernment | required workflow input |
| F:operative:sufficiency | input evidence; context adequacy; process expertise; execution judgment | action adequacy | adequate input; usable context; process competence; execution judgment | sufficient iteration context |
| F:operative:completeness | input record; context account; process mastery; execution insight | action coverage | complete input; context account; process coverage; execution insight | complete iteration coverage |
| F:operative:consistency | input measurement; context message; process understanding; execution reasoning | action coherence | reliable input; coherent context; stable process; execution reasoning | consistent iteration signal |
| F:evaluative:necessity | appraisal fact; review signal; review understanding; rationale discernment | appraisal need | salient fact; review signal; review comprehension; rationale discernment | required review evidence |
| F:evaluative:sufficiency | appraisal evidence; review context; review expertise; rationale judgment | appraisal adequacy | adequate appraisal; review context; expert review; judged rationale | sufficient review proof |
| F:evaluative:completeness | appraisal record; review account; review mastery; rationale insight | appraisal coverage | complete appraisal; review account; review coverage; rationale insight | complete review coverage |
| F:evaluative:consistency | appraisal measurement; review message; review understanding; rationale reasoning | appraisal coherence | measured appraisal; coherent review; stable rationale; reasoned review | consistent review basis |

### Result

| | **necessity** | **sufficiency** | **completeness** | **consistency** |
|---|---|---|---|---|
| **normative** | required state evidence | sufficient solver proof | complete behavior coverage | consistent state semantics |
| **operative** | required workflow input | sufficient iteration context | complete iteration coverage | consistent iteration signal |
| **evaluative** | required review evidence | sufficient review proof | complete review coverage | consistent review basis |

## Matrix D - Objectives (3x4)

### Construction: Addition A plus resolution-transformed F

Each cell interprets `L_D(row,col) = {A(row,col), resolution transformed F(row,matching)}` using the same three-step procedure.

| Cell | L_D contributors | Step 1 axis anchor | Step 2 projections | Step 3 centroid |
|---|---|---|---|---|
| D:normative:guiding | prescriptive direction; resolved state evidence | binding guidance | directed obligation; resolved evidence | evidence-bound direction |
| D:normative:applying | mandatory practice; resolved solver proof | binding action | required practice; resolved proof | proof-bound practice |
| D:normative:judging | compliance determination; resolved behavior coverage | binding judgment | bounded judgment; resolved coverage | coverage-bound judgment |
| D:normative:reviewing | regulatory audit; resolved state semantics | binding review | audit check; resolved semantics | semantics-bound review |
| D:operative:guiding | procedural direction; resolved workflow input | action guidance | directed procedure; resolved input | input-bound procedure |
| D:operative:applying | practical execution; resolved iteration context | action application | execution practice; resolved context | context-bound iteration |
| D:operative:judging | performance assessment; resolved iteration coverage | action judgment | performance check; resolved coverage | coverage-bound assessment |
| D:operative:reviewing | process audit; resolved iteration signal | action review | process audit; resolved signal | signal-bound audit |
| D:evaluative:guiding | value orientation; resolved review evidence | appraisal guidance | value frame; resolved evidence | evidence-bound appraisal |
| D:evaluative:applying | merit application; resolved review proof | appraisal application | merit use; resolved proof | proof-bound appraisal |
| D:evaluative:judging | worth determination; resolved review coverage | appraisal judgment | worth decision; resolved coverage | coverage-bound appraisal |
| D:evaluative:reviewing | quality appraisal; resolved review basis | appraisal review | quality review; resolved basis | basis-bound appraisal |

### Result

| | **guiding** | **applying** | **judging** | **reviewing** |
|---|---|---|---|---|
| **normative** | evidence-bound direction | proof-bound practice | coverage-bound judgment | semantics-bound review |
| **operative** | input-bound procedure | context-bound iteration | coverage-bound assessment | signal-bound audit |
| **evaluative** | evidence-bound appraisal | proof-bound appraisal | coverage-bound appraisal | basis-bound appraisal |

## Matrix K - Transpose of D (4x3)

### Construction: K(i,j) = D(j,i)

### Result

| | **normative** | **operative** | **evaluative** |
|---|---|---|---|
| **guiding** | evidence-bound direction | input-bound procedure | evidence-bound appraisal |
| **applying** | proof-bound practice | context-bound iteration | proof-bound appraisal |
| **judging** | coverage-bound judgment | coverage-bound assessment | coverage-bound appraisal |
| **reviewing** | semantics-bound review | signal-bound audit | basis-bound appraisal |

## Matrix G - Truncation of B (3x4)

### Construction: remove wisdom row from B

### Result

| | **necessity** | **sufficiency** | **completeness** | **consistency** |
|---|---|---|---|---|
| **data** | essential fact | adequate evidence | comprehensive record | reliable measurement |
| **information** | essential signal | adequate context | comprehensive account | coherent message |
| **knowledge** | fundamental understanding | competent expertise | thorough mastery | coherent understanding |

## Matrix X - Verification (4x4)

### Construction: Dot product K dot G

Each cell applies the same three-step interpretation procedure to `L_X(row,col)`.

| Cell | L_X contributors | Step 1 axis anchor | Step 2 projections | Step 3 centroid |
|---|---|---|---|---|
| X:guiding:necessity | direction fact; procedure signal; appraisal understanding | guidance need | factual direction; procedural signal; appraisal understanding | necessary guidance evidence |
| X:guiding:sufficiency | direction evidence; procedure context; appraisal expertise | guidance adequacy | evidenced direction; procedural context; appraisal expertise | sufficient guidance context |
| X:guiding:completeness | direction record; procedure account; appraisal mastery | guidance coverage | recorded direction; procedural account; appraisal mastery | complete guidance record |
| X:guiding:consistency | direction measurement; procedure message; appraisal understanding | guidance coherence | measured direction; procedural message; coherent appraisal | consistent guidance meaning |
| X:applying:necessity | practice fact; iteration signal; appraisal understanding | application need | practice fact; iteration signal; appraisal understanding | necessary application evidence |
| X:applying:sufficiency | practice evidence; iteration context; appraisal expertise | application adequacy | practice proof; iteration context; appraisal expertise | sufficient application context |
| X:applying:completeness | practice record; iteration account; appraisal mastery | application coverage | practice record; iteration account; appraisal mastery | complete application record |
| X:applying:consistency | practice measurement; iteration message; appraisal understanding | application coherence | measured practice; iteration message; coherent appraisal | consistent application meaning |
| X:judging:necessity | judgment fact; assessment signal; appraisal understanding | judgment need | judgment fact; assessment signal; appraisal understanding | necessary judgment evidence |
| X:judging:sufficiency | judgment evidence; assessment context; appraisal expertise | judgment adequacy | judgment proof; assessment context; appraisal expertise | sufficient judgment context |
| X:judging:completeness | judgment record; assessment account; appraisal mastery | judgment coverage | judgment record; assessment account; appraisal mastery | complete judgment record |
| X:judging:consistency | judgment measurement; assessment message; appraisal understanding | judgment coherence | measured judgment; assessment message; coherent appraisal | consistent judgment meaning |
| X:reviewing:necessity | review fact; audit signal; appraisal understanding | review need | review fact; audit signal; appraisal understanding | necessary review evidence |
| X:reviewing:sufficiency | review evidence; audit context; appraisal expertise | review adequacy | review proof; audit context; appraisal expertise | sufficient review context |
| X:reviewing:completeness | review record; audit account; appraisal mastery | review coverage | review record; audit account; appraisal mastery | complete review record |
| X:reviewing:consistency | review measurement; audit message; appraisal understanding | review coherence | measured review; audit message; coherent appraisal | consistent review meaning |

### Result

| | **necessity** | **sufficiency** | **completeness** | **consistency** |
|---|---|---|---|---|
| **guiding** | necessary guidance evidence | sufficient guidance context | complete guidance record | consistent guidance meaning |
| **applying** | necessary application evidence | sufficient application context | complete application record | consistent application meaning |
| **judging** | necessary judgment evidence | sufficient judgment context | complete judgment record | consistent judgment meaning |
| **reviewing** | necessary review evidence | sufficient review context | complete review record | consistent review meaning |

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

### Construction: Dot product X dot T

Each cell applies the same three-step interpretation procedure to `L_E(row,col)`.

| Cell | L_E contributors | Step 1 axis anchor | Step 2 projections | Step 3 centroid |
|---|---|---|---|---|
| E:guiding:data | guidance fact; guidance evidence; guidance record; guidance measurement | guidance data | factual guidance; evidential guidance; recorded guidance; measured guidance | guidance data quality |
| E:guiding:information | guidance signal; guidance context; guidance account; guidance message | guidance information | signal guidance; contextual guidance; account guidance; coherent guidance | guidance information quality |
| E:guiding:knowledge | guidance understanding; guidance expertise; guidance mastery; guidance coherence | guidance knowledge | understood guidance; expert guidance; mastered guidance; coherent guidance | guidance knowledge quality |
| E:guiding:wisdom | guidance discernment; guidance judgment; guidance insight; guidance reasoning | guidance wisdom | discerning guidance; judged guidance; insightful guidance; reasoned guidance | guidance judgment quality |
| E:applying:data | application fact; application evidence; application record; application measurement | application data | factual application; evidential application; recorded application; measured application | application data quality |
| E:applying:information | application signal; application context; application account; application message | application information | signal application; contextual application; account application | application information quality |
| E:applying:knowledge | application understanding; application expertise; application mastery; application coherence | application knowledge | understood application; expert application; mastered application | application knowledge quality |
| E:applying:wisdom | application discernment; application judgment; application insight; application reasoning | application wisdom | discerning application; judged application; insightful application | application judgment quality |
| E:judging:data | judgment fact; judgment evidence; judgment record; judgment measurement | judgment data | factual judgment; evidential judgment; recorded judgment; measured judgment | judgment data quality |
| E:judging:information | judgment signal; judgment context; judgment account; judgment message | judgment information | signal judgment; contextual judgment; account judgment | judgment information quality |
| E:judging:knowledge | judgment understanding; judgment expertise; judgment mastery; judgment coherence | judgment knowledge | understood judgment; expert judgment; mastered judgment | judgment knowledge quality |
| E:judging:wisdom | judgment discernment; judgment judgment; judgment insight; judgment reasoning | judgment wisdom | discerning judgment; judged judgment; insightful judgment | judgment reasoning quality |
| E:reviewing:data | review fact; review evidence; review record; review measurement | review data | factual review; evidential review; recorded review; measured review | review data quality |
| E:reviewing:information | review signal; review context; review account; review message | review information | signal review; contextual review; account review | review information quality |
| E:reviewing:knowledge | review understanding; review expertise; review mastery; review coherence | review knowledge | understood review; expert review; mastered review | review knowledge quality |
| E:reviewing:wisdom | review discernment; review judgment; review insight; review reasoning | review wisdom | discerning review; judged review; insightful review | review reasoning quality |

### Result

| | **data** | **information** | **knowledge** | **wisdom** |
|---|---|---|---|---|
| **guiding** | guidance data quality | guidance information quality | guidance knowledge quality | guidance judgment quality |
| **applying** | application data quality | application information quality | application knowledge quality | application judgment quality |
| **judging** | judgment data quality | judgment information quality | judgment knowledge quality | judgment reasoning quality |
| **reviewing** | review data quality | review information quality | review knowledge quality | review reasoning quality |

## Matrix Summary

### C - Formulation

| | **necessity** | **sufficiency** | **completeness** | **consistency** |
|---|---|---|---|---|
| **normative** | required evidence frame | adequate proof basis | complete obligation record | coherent rule record |
| **operative** | actionable input basis | usable execution context | complete process account | stable execution signal |
| **evaluative** | appraisal evidence need | sufficient review basis | complete review record | consistent review rationale |

### F - Requirements

| | **necessity** | **sufficiency** | **completeness** | **consistency** |
|---|---|---|---|---|
| **normative** | required state evidence | sufficient solver proof | complete behavior coverage | consistent state semantics |
| **operative** | required workflow input | sufficient iteration context | complete iteration coverage | consistent iteration signal |
| **evaluative** | required review evidence | sufficient review proof | complete review coverage | consistent review basis |

### D - Objectives

| | **guiding** | **applying** | **judging** | **reviewing** |
|---|---|---|---|---|
| **normative** | evidence-bound direction | proof-bound practice | coverage-bound judgment | semantics-bound review |
| **operative** | input-bound procedure | context-bound iteration | coverage-bound assessment | signal-bound audit |
| **evaluative** | evidence-bound appraisal | proof-bound appraisal | coverage-bound appraisal | basis-bound appraisal |

### K - Transpose of D

| | **normative** | **operative** | **evaluative** |
|---|---|---|---|
| **guiding** | evidence-bound direction | input-bound procedure | evidence-bound appraisal |
| **applying** | proof-bound practice | context-bound iteration | proof-bound appraisal |
| **judging** | coverage-bound judgment | coverage-bound assessment | coverage-bound appraisal |
| **reviewing** | semantics-bound review | signal-bound audit | basis-bound appraisal |

### G - Truncation of B

| | **necessity** | **sufficiency** | **completeness** | **consistency** |
|---|---|---|---|---|
| **data** | essential fact | adequate evidence | comprehensive record | reliable measurement |
| **information** | essential signal | adequate context | comprehensive account | coherent message |
| **knowledge** | fundamental understanding | competent expertise | thorough mastery | coherent understanding |

### X - Verification

| | **necessity** | **sufficiency** | **completeness** | **consistency** |
|---|---|---|---|---|
| **guiding** | necessary guidance evidence | sufficient guidance context | complete guidance record | consistent guidance meaning |
| **applying** | necessary application evidence | sufficient application context | complete application record | consistent application meaning |
| **judging** | necessary judgment evidence | sufficient judgment context | complete judgment record | consistent judgment meaning |
| **reviewing** | necessary review evidence | sufficient review context | complete review record | consistent review meaning |

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
| **guiding** | guidance data quality | guidance information quality | guidance knowledge quality | guidance judgment quality |
| **applying** | application data quality | application information quality | application knowledge quality | application judgment quality |
| **judging** | judgment data quality | judgment information quality | judgment knowledge quality | judgment reasoning quality |
| **reviewing** | review data quality | review information quality | review knowledge quality | review reasoning quality |

## Audit

- Final C, F, D, X, and E result cells contain no algebra notation.
- Final result cells are populated with compact semantic units.
- No nonlinear solver implementation, convergence tolerances, friction defaults, protected standards content, or compliance claims are introduced.
- Audit result: PASS.
