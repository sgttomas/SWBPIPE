# Deliverable: DEL-13-01 Design knowledge schema and provenance model

**Generated:** 2026-05-04
**DECOMP_VARIANT:** SOFTWARE
**Perspective:** This deliverable frames the schema and provenance surface that lets a physical design model carry user-supplied design context, source notes, and assumptions. It must help later constraint and transformation work ask what evidence, boundaries, and missing slots are needed without acting as engineering authority.
**Framework:** Chirality Semantic Algebra
**Inputs Read:**
- `_CONTEXT.md` - deliverable identity, scope, architecture-basis injection
- `_STATUS.md` - lifecycle state observed after four-doc setup
- `Datasheet.md` - production document
- `Specification.md` - production document
- `Guidance.md` - production document
- `Procedure.md` - production document
- `_REFERENCES.md` - reference index

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

### Construction: Dot Product A dot B

| Cell | Intermediate collection L | Step 1: Axis anchor | Step 2: Projected contributors | Step 3: Centroid |
|---|---|---|---|---|
| C[normative, necessity] | prescriptive direction * essential fact; mandatory practice * essential signal; compliance determination * fundamental understanding; regulatory audit * essential discernment | normative * necessity = obligation trigger | obligation trigger * directive fact = mandate evidence; obligation trigger * required signal = need cue; obligation trigger * compliance basis = rule basis; obligation trigger * audit discernment = control trigger | Binding evidence threshold |
| C[normative, sufficiency] | prescriptive direction * adequate evidence; mandatory practice * adequate context; compliance determination * competent expertise; regulatory audit * adequate judgment | normative * sufficiency = adequacy frame | adequacy frame * directive proof = control support; adequacy frame * practice context = rule context; adequacy frame * competence test = qualified check; adequacy frame * audit judgment = evidence sufficiency | Adequacy control basis |
| C[normative, completeness] | prescriptive direction * comprehensive record; mandatory practice * comprehensive account; compliance determination * thorough mastery; regulatory audit * holistic insight | normative * completeness = obligation closure | obligation closure * directive record = complete mandate; obligation closure * practice account = complete rule trace; obligation closure * mastery check = fulfilled basis; obligation closure * audit insight = closure evidence | Complete compliance basis |
| C[normative, consistency] | prescriptive direction * reliable measurement; mandatory practice * coherent message; compliance determination * coherent understanding; regulatory audit * principled reasoning | normative * consistency = rule coherence | rule coherence * measured direction = stable control; rule coherence * coherent practice = aligned rule; rule coherence * understood compliance = consistent basis; rule coherence * principled audit = reasoned control | Coherent compliance basis |
| C[operative, necessity] | procedural direction * essential fact; practical execution * essential signal; performance assessment * fundamental understanding; process audit * essential discernment | operative * necessity = action trigger | action trigger * procedure fact = input basis; action trigger * execution signal = task cue; action trigger * performance understanding = work basis; action trigger * audit discernment = process trigger | Execution input basis |
| C[operative, sufficiency] | procedural direction * adequate evidence; practical execution * adequate context; performance assessment * competent expertise; process audit * adequate judgment | operative * sufficiency = execution adequacy | execution adequacy * procedure proof = usable evidence; execution adequacy * work context = practical context; execution adequacy * assessment competence = performance support; execution adequacy * audit judgment = sufficient process | Workable context basis |
| C[operative, completeness] | procedural direction * comprehensive record; practical execution * comprehensive account; performance assessment * thorough mastery; process audit * holistic insight | operative * completeness = process closure | process closure * procedure record = step record; process closure * execution account = full account; process closure * assessment mastery = checked work; process closure * audit insight = complete trace | Complete process account |
| C[operative, consistency] | procedural direction * reliable measurement; practical execution * coherent message; performance assessment * coherent understanding; process audit * principled reasoning | operative * consistency = process coherence | process coherence * measured procedure = repeatable input; process coherence * execution message = aligned work; process coherence * assessment understanding = stable assessment; process coherence * audit reasoning = reasoned process | Repeatable process message |
| C[evaluative, necessity] | value orientation * essential fact; merit application * essential signal; worth determination * fundamental understanding; quality appraisal * essential discernment | evaluative * necessity = value trigger | value trigger * value fact = review basis; value trigger * merit signal = appraisal cue; value trigger * worth understanding = judgment basis; value trigger * quality discernment = quality trigger | Judgment evidence basis |
| C[evaluative, sufficiency] | value orientation * adequate evidence; merit application * adequate context; worth determination * competent expertise; quality appraisal * adequate judgment | evaluative * sufficiency = judgment adequacy | judgment adequacy * value evidence = support basis; judgment adequacy * merit context = qualified context; judgment adequacy * worth competence = review support; judgment adequacy * quality judgment = quality sufficiency | Contextual merit basis |
| C[evaluative, completeness] | value orientation * comprehensive record; merit application * comprehensive account; worth determination * thorough mastery; quality appraisal * holistic insight | evaluative * completeness = appraisal closure | appraisal closure * value record = review record; appraisal closure * merit account = complete merit; appraisal closure * worth mastery = assessed completeness; appraisal closure * quality insight = closure judgment | Holistic appraisal basis |
| C[evaluative, consistency] | value orientation * reliable measurement; merit application * coherent message; worth determination * coherent understanding; quality appraisal * principled reasoning | evaluative * consistency = appraisal coherence | appraisal coherence * measured value = stable value; appraisal coherence * merit message = aligned merit; appraisal coherence * worth understanding = consistent worth; appraisal coherence * quality reasoning = reasoned appraisal | Coherent quality rationale |

### Result

| | **necessity** | **sufficiency** | **completeness** | **consistency** |
|---|---|---|---|---|
| **normative** | Binding evidence threshold | Adequacy control basis | Complete compliance basis | Coherent compliance basis |
| **operative** | Execution input basis | Workable context basis | Complete process account | Repeatable process message |
| **evaluative** | Judgment evidence basis | Contextual merit basis | Holistic appraisal basis | Coherent quality rationale |

## Matrix F - Requirements (3x4)

### Construction: Dot Product C dot B

| Cell | Intermediate collection L | Step 1: Axis anchor | Step 2: Projected contributors | Step 3: Centroid |
|---|---|---|---|---|
| F[normative, necessity] | Binding evidence threshold * essential fact; Adequacy control basis * essential signal; Complete compliance basis * fundamental understanding; Coherent compliance basis * essential discernment | normative * necessity = obligation trigger | obligation trigger * binding fact = required proof; obligation trigger * control signal = evidence cue; obligation trigger * compliance understanding = rule comprehension; obligation trigger * discerned coherence = boundary need | Required evidence boundary |
| F[normative, sufficiency] | Binding evidence threshold * adequate evidence; Adequacy control basis * adequate context; Complete compliance basis * competent expertise; Coherent compliance basis * adequate judgment | normative * sufficiency = adequacy frame | adequacy frame * binding evidence = proof support; adequacy frame * control context = supported control; adequacy frame * compliance competence = qualified proof; adequacy frame * coherent judgment = sufficient rationale | Sufficient proof package |
| F[normative, completeness] | Binding evidence threshold * comprehensive record; Adequacy control basis * comprehensive account; Complete compliance basis * thorough mastery; Coherent compliance basis * holistic insight | normative * completeness = obligation closure | obligation closure * binding record = required record; obligation closure * control account = full basis; obligation closure * compliance mastery = obligation closure; obligation closure * holistic coherence = complete rationale | Complete obligation record |
| F[normative, consistency] | Binding evidence threshold * reliable measurement; Adequacy control basis * coherent message; Complete compliance basis * coherent understanding; Coherent compliance basis * principled reasoning | normative * consistency = rule coherence | rule coherence * measured proof = stable evidence; rule coherence * control message = aligned control; rule coherence * compliance understanding = coherent basis; rule coherence * principled reasoning = reasoned control | Consistent control rationale |
| F[operative, necessity] | Execution input basis * essential fact; Workable context basis * essential signal; Complete process account * fundamental understanding; Repeatable process message * essential discernment | operative * necessity = action trigger | action trigger * input fact = needed input; action trigger * context signal = work cue; action trigger * process understanding = execution basis; action trigger * repeatable discernment = action boundary | Required input pathway |
| F[operative, sufficiency] | Execution input basis * adequate evidence; Workable context basis * adequate context; Complete process account * competent expertise; Repeatable process message * adequate judgment | operative * sufficiency = execution adequacy | execution adequacy * input evidence = supported input; execution adequacy * workable context = sufficient setting; execution adequacy * process competence = qualified workflow; execution adequacy * repeatable judgment = adequate action | Sufficient execution context |
| F[operative, completeness] | Execution input basis * comprehensive record; Workable context basis * comprehensive account; Complete process account * thorough mastery; Repeatable process message * holistic insight | operative * completeness = process closure | process closure * input record = complete input; process closure * context account = full setting; process closure * process mastery = complete workflow; process closure * repeatable insight = closure trace | Complete workflow record |
| F[operative, consistency] | Execution input basis * reliable measurement; Workable context basis * coherent message; Complete process account * coherent understanding; Repeatable process message * principled reasoning | operative * consistency = process coherence | process coherence * measured input = stable input; process coherence * context message = aligned context; process coherence * process understanding = consistent workflow; process coherence * repeatable reasoning = coherent action | Consistent process rationale |
| F[evaluative, necessity] | Judgment evidence basis * essential fact; Contextual merit basis * essential signal; Holistic appraisal basis * fundamental understanding; Coherent quality rationale * essential discernment | evaluative * necessity = value trigger | value trigger * judgment fact = required review; value trigger * merit signal = review cue; value trigger * appraisal understanding = assessment basis; value trigger * quality discernment = value boundary | Required review basis |
| F[evaluative, sufficiency] | Judgment evidence basis * adequate evidence; Contextual merit basis * adequate context; Holistic appraisal basis * competent expertise; Coherent quality rationale * adequate judgment | evaluative * sufficiency = judgment adequacy | judgment adequacy * judgment evidence = supported review; judgment adequacy * merit context = adequate merit; judgment adequacy * appraisal competence = qualified assessment; judgment adequacy * quality judgment = sufficient rationale | Sufficient judgment context |
| F[evaluative, completeness] | Judgment evidence basis * comprehensive record; Contextual merit basis * comprehensive account; Holistic appraisal basis * thorough mastery; Coherent quality rationale * holistic insight | evaluative * completeness = appraisal closure | appraisal closure * judgment record = review record; appraisal closure * merit account = full merit; appraisal closure * appraisal mastery = complete assessment; appraisal closure * quality insight = closure rationale | Complete assessment record |
| F[evaluative, consistency] | Judgment evidence basis * reliable measurement; Contextual merit basis * coherent message; Holistic appraisal basis * coherent understanding; Coherent quality rationale * principled reasoning | evaluative * consistency = appraisal coherence | appraisal coherence * measured judgment = stable assessment; appraisal coherence * merit message = aligned merit; appraisal coherence * appraisal understanding = consistent review; appraisal coherence * quality reasoning = reasoned quality | Consistent appraisal rationale |

### Result

| | **necessity** | **sufficiency** | **completeness** | **consistency** |
|---|---|---|---|---|
| **normative** | Required evidence boundary | Sufficient proof package | Complete obligation record | Consistent control rationale |
| **operative** | Required input pathway | Sufficient execution context | Complete workflow record | Consistent process rationale |
| **evaluative** | Required review basis | Sufficient judgment context | Complete assessment record | Consistent appraisal rationale |

## Matrix D - Objectives (3x4)

### Construction: Addition A and Resolution-Transformed F

| Cell | Intermediate collection L | Step 1: Axis anchor | Step 2: Projected contributors | Step 3: Centroid |
|---|---|---|---|---|
| D[normative, guiding] | prescriptive direction; resolution * Required evidence boundary | normative * guiding = rule direction frame | rule direction frame * prescriptive direction = controlled mandate; rule direction frame * resolved evidence = closed proof direction | Controlled obligation direction |
| D[normative, applying] | mandatory practice; resolution * Sufficient proof package | normative * applying = rule practice frame | rule practice frame * mandatory practice = enforced action; rule practice frame * resolved proof = supported practice | Enforced practice basis |
| D[normative, judging] | compliance determination; resolution * Complete obligation record | normative * judging = rule decision frame | rule decision frame * compliance determination = determinate control; rule decision frame * resolved obligation = closed record | Determinate compliance basis |
| D[normative, reviewing] | regulatory audit; resolution * Consistent control rationale | normative * reviewing = rule audit frame | rule audit frame * regulatory audit = audit control; rule audit frame * resolved rationale = reasoned record | Auditable control record |
| D[operative, guiding] | procedural direction; resolution * Required input pathway | operative * guiding = action direction frame | action direction frame * procedural direction = actionable route; action direction frame * resolved input = clear pathway | Actionable workflow direction |
| D[operative, applying] | practical execution; resolution * Sufficient execution context | operative * applying = action practice frame | action practice frame * practical execution = executable work; action practice frame * resolved context = supported execution | Validated execution basis |
| D[operative, judging] | performance assessment; resolution * Complete workflow record | operative * judging = action decision frame | action decision frame * performance assessment = measured work; action decision frame * resolved workflow = reviewable process | Measured performance basis |
| D[operative, reviewing] | process audit; resolution * Consistent process rationale | operative * reviewing = action audit frame | action audit frame * process audit = auditable workflow; action audit frame * resolved rationale = reasoned process | Auditable process record |
| D[evaluative, guiding] | value orientation; resolution * Required review basis | evaluative * guiding = value direction frame | value direction frame * value orientation = principled direction; value direction frame * resolved review = grounded value | Principled value direction |
| D[evaluative, applying] | merit application; resolution * Sufficient judgment context | evaluative * applying = value practice frame | value practice frame * merit application = applied merit; value practice frame * resolved judgment = qualified value | Reasoned merit basis |
| D[evaluative, judging] | worth determination; resolution * Complete assessment record | evaluative * judging = value decision frame | value decision frame * worth determination = determinate worth; value decision frame * resolved assessment = closed judgment | Determinate worth basis |
| D[evaluative, reviewing] | quality appraisal; resolution * Consistent appraisal rationale | evaluative * reviewing = value audit frame | value audit frame * quality appraisal = auditable quality; value audit frame * resolved rationale = reasoned appraisal | Auditable quality record |

### Result

| | **guiding** | **applying** | **judging** | **reviewing** |
|---|---|---|---|---|
| **normative** | Controlled obligation direction | Enforced practice basis | Determinate compliance basis | Auditable control record |
| **operative** | Actionable workflow direction | Validated execution basis | Measured performance basis | Auditable process record |
| **evaluative** | Principled value direction | Reasoned merit basis | Determinate worth basis | Auditable quality record |

## Matrix K - Transpose of D (4x3)

### Construction: K(i,j) = D(j,i)

### Result

| | **normative** | **operative** | **evaluative** |
|---|---|---|---|
| **guiding** | Controlled obligation direction | Actionable workflow direction | Principled value direction |
| **applying** | Enforced practice basis | Validated execution basis | Reasoned merit basis |
| **judging** | Determinate compliance basis | Measured performance basis | Determinate worth basis |
| **reviewing** | Auditable control record | Auditable process record | Auditable quality record |

## Matrix G - Truncation of B (3x4)

### Construction: remove `wisdom` row from B

### Result

| | **necessity** | **sufficiency** | **completeness** | **consistency** |
|---|---|---|---|---|
| **data** | essential fact | adequate evidence | comprehensive record | reliable measurement |
| **information** | essential signal | adequate context | comprehensive account | coherent message |
| **knowledge** | fundamental understanding | competent expertise | thorough mastery | coherent understanding |

## Matrix X - Verification (4x4)

### Construction: Dot Product K dot G

| Cell | Intermediate collection L | Step 1: Axis anchor | Step 2: Projected contributors | Step 3: Centroid |
|---|---|---|---|---|
| X[guiding, necessity] | Controlled obligation direction * essential fact; Actionable workflow direction * essential signal; Principled value direction * fundamental understanding | guiding * necessity = orientation trigger | orientation trigger * obligation fact = needed direction; orientation trigger * workflow signal = route cue; orientation trigger * value understanding = purpose basis | Required direction evidence |
| X[guiding, sufficiency] | Controlled obligation direction * adequate evidence; Actionable workflow direction * adequate context; Principled value direction * competent expertise | guiding * sufficiency = orientation adequacy | orientation adequacy * obligation evidence = supported direction; orientation adequacy * workflow context = usable route; orientation adequacy * value competence = qualified purpose | Adequate direction context |
| X[guiding, completeness] | Controlled obligation direction * comprehensive record; Actionable workflow direction * comprehensive account; Principled value direction * thorough mastery | guiding * completeness = orientation closure | orientation closure * obligation record = complete direction; orientation closure * workflow account = full route; orientation closure * value mastery = mature purpose | Complete direction record |
| X[guiding, consistency] | Controlled obligation direction * reliable measurement; Actionable workflow direction * coherent message; Principled value direction * coherent understanding | guiding * consistency = orientation coherence | orientation coherence * obligation measurement = stable direction; orientation coherence * workflow message = aligned route; orientation coherence * value understanding = coherent purpose | Coherent direction rationale |
| X[applying, necessity] | Enforced practice basis * essential fact; Validated execution basis * essential signal; Reasoned merit basis * fundamental understanding | applying * necessity = practice trigger | practice trigger * enforced fact = needed practice; practice trigger * execution signal = action cue; practice trigger * merit understanding = action basis | Required practice evidence |
| X[applying, sufficiency] | Enforced practice basis * adequate evidence; Validated execution basis * adequate context; Reasoned merit basis * competent expertise | applying * sufficiency = practice adequacy | practice adequacy * enforced evidence = supported practice; practice adequacy * execution context = workable action; practice adequacy * merit competence = qualified action | Adequate execution context |
| X[applying, completeness] | Enforced practice basis * comprehensive record; Validated execution basis * comprehensive account; Reasoned merit basis * thorough mastery | applying * completeness = practice closure | practice closure * enforced record = complete practice; practice closure * execution account = full action; practice closure * merit mastery = complete merit | Complete practice record |
| X[applying, consistency] | Enforced practice basis * reliable measurement; Validated execution basis * coherent message; Reasoned merit basis * coherent understanding | applying * consistency = practice coherence | practice coherence * enforced measurement = stable practice; practice coherence * execution message = aligned action; practice coherence * merit understanding = reasoned action | Coherent execution rationale |
| X[judging, necessity] | Determinate compliance basis * essential fact; Measured performance basis * essential signal; Determinate worth basis * fundamental understanding | judging * necessity = decision trigger | decision trigger * compliance fact = needed decision; decision trigger * performance signal = assessment cue; decision trigger * worth understanding = value basis | Required decision evidence |
| X[judging, sufficiency] | Determinate compliance basis * adequate evidence; Measured performance basis * adequate context; Determinate worth basis * competent expertise | judging * sufficiency = decision adequacy | decision adequacy * compliance evidence = supported decision; decision adequacy * performance context = adequate assessment; decision adequacy * worth competence = qualified decision | Adequate assessment context |
| X[judging, completeness] | Determinate compliance basis * comprehensive record; Measured performance basis * comprehensive account; Determinate worth basis * thorough mastery | judging * completeness = decision closure | decision closure * compliance record = complete decision; decision closure * performance account = full assessment; decision closure * worth mastery = complete judgment | Complete decision record |
| X[judging, consistency] | Determinate compliance basis * reliable measurement; Measured performance basis * coherent message; Determinate worth basis * coherent understanding | judging * consistency = decision coherence | decision coherence * compliance measurement = stable decision; decision coherence * performance message = aligned assessment; decision coherence * worth understanding = coherent judgment | Coherent assessment rationale |
| X[reviewing, necessity] | Auditable control record * essential fact; Auditable process record * essential signal; Auditable quality record * fundamental understanding | reviewing * necessity = audit trigger | audit trigger * control fact = needed audit; audit trigger * process signal = review cue; audit trigger * quality understanding = review basis | Required audit evidence |
| X[reviewing, sufficiency] | Auditable control record * adequate evidence; Auditable process record * adequate context; Auditable quality record * competent expertise | reviewing * sufficiency = audit adequacy | audit adequacy * control evidence = supported audit; audit adequacy * process context = review context; audit adequacy * quality competence = qualified audit | Adequate inspection context |
| X[reviewing, completeness] | Auditable control record * comprehensive record; Auditable process record * comprehensive account; Auditable quality record * thorough mastery | reviewing * completeness = audit closure | audit closure * control record = complete audit; audit closure * process account = full review; audit closure * quality mastery = mature audit | Complete audit record |
| X[reviewing, consistency] | Auditable control record * reliable measurement; Auditable process record * coherent message; Auditable quality record * coherent understanding | reviewing * consistency = audit coherence | audit coherence * control measurement = stable audit; audit coherence * process message = aligned review; audit coherence * quality understanding = reasoned audit | Coherent audit rationale |

### Result

| | **necessity** | **sufficiency** | **completeness** | **consistency** |
|---|---|---|---|---|
| **guiding** | Required direction evidence | Adequate direction context | Complete direction record | Coherent direction rationale |
| **applying** | Required practice evidence | Adequate execution context | Complete practice record | Coherent execution rationale |
| **judging** | Required decision evidence | Adequate assessment context | Complete decision record | Coherent assessment rationale |
| **reviewing** | Required audit evidence | Adequate inspection context | Complete audit record | Coherent audit rationale |

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

### Construction: Dot Product X dot T

| Cell | Intermediate collection L | Step 1: Axis anchor | Step 2: Projected contributors | Step 3: Centroid |
|---|---|---|---|---|
| E[guiding, data] | Required direction evidence * essential fact; Adequate direction context * adequate evidence; Complete direction record * comprehensive record; Coherent direction rationale * reliable measurement | guiding * data = evidence orientation frame | evidence orientation frame * required fact = sourced orientation; evidence orientation frame * adequate proof = supported orientation; evidence orientation frame * complete record = traced orientation; evidence orientation frame * reliable measure = stable orientation | Source-grounded orientation |
| E[guiding, information] | Required direction evidence * essential signal; Adequate direction context * adequate context; Complete direction record * comprehensive account; Coherent direction rationale * coherent message | guiding * information = context orientation frame | context orientation frame * required signal = cue orientation; context orientation frame * adequate context = grounded orientation; context orientation frame * complete account = explained orientation; context orientation frame * coherent message = aligned orientation | Contextual orientation brief |
| E[guiding, knowledge] | Required direction evidence * fundamental understanding; Adequate direction context * competent expertise; Complete direction record * thorough mastery; Coherent direction rationale * coherent understanding | guiding * knowledge = competence orientation frame | competence orientation frame * required understanding = understood orientation; competence orientation frame * expertise = competent orientation; competence orientation frame * mastery = mature orientation; competence orientation frame * coherent understanding = stable model | Competent orientation model |
| E[guiding, wisdom] | Required direction evidence * essential discernment; Adequate direction context * adequate judgment; Complete direction record * holistic insight; Coherent direction rationale * principled reasoning | guiding * wisdom = discernment orientation frame | discernment orientation frame * required discernment = prudent orientation; discernment orientation frame * adequate judgment = reasoned orientation; discernment orientation frame * holistic insight = integral orientation; discernment orientation frame * principled reasoning = principled rationale | Principled orientation rationale |
| E[applying, data] | Required practice evidence * essential fact; Adequate execution context * adequate evidence; Complete practice record * comprehensive record; Coherent execution rationale * reliable measurement | applying * data = evidence practice frame | evidence practice frame * required fact = sourced action; evidence practice frame * adequate proof = supported action; evidence practice frame * complete record = traced action; evidence practice frame * reliable measure = stable action | Source-grounded action basis |
| E[applying, information] | Required practice evidence * essential signal; Adequate execution context * adequate context; Complete practice record * comprehensive account; Coherent execution rationale * coherent message | applying * information = context practice frame | context practice frame * required signal = action cue; context practice frame * adequate context = execution setting; context practice frame * complete account = explained action; context practice frame * coherent message = aligned execution | Contextual execution brief |
| E[applying, knowledge] | Required practice evidence * fundamental understanding; Adequate execution context * competent expertise; Complete practice record * thorough mastery; Coherent execution rationale * coherent understanding | applying * knowledge = competence practice frame | competence practice frame * required understanding = understood action; competence practice frame * expertise = competent execution; competence practice frame * mastery = mature execution; competence practice frame * coherent understanding = stable model | Competent execution model |
| E[applying, wisdom] | Required practice evidence * essential discernment; Adequate execution context * adequate judgment; Complete practice record * holistic insight; Coherent execution rationale * principled reasoning | applying * wisdom = discernment practice frame | discernment practice frame * required discernment = prudent action; discernment practice frame * adequate judgment = reasoned execution; discernment practice frame * holistic insight = integral execution; discernment practice frame * principled reasoning = principled rationale | Principled execution rationale |
| E[judging, data] | Required decision evidence * essential fact; Adequate assessment context * adequate evidence; Complete decision record * comprehensive record; Coherent assessment rationale * reliable measurement | judging * data = evidence decision frame | evidence decision frame * required fact = sourced decision; evidence decision frame * adequate proof = supported decision; evidence decision frame * complete record = traced decision; evidence decision frame * reliable measure = stable decision | Source-grounded decision basis |
| E[judging, information] | Required decision evidence * essential signal; Adequate assessment context * adequate context; Complete decision record * comprehensive account; Coherent assessment rationale * coherent message | judging * information = context decision frame | context decision frame * required signal = decision cue; context decision frame * adequate context = assessment setting; context decision frame * complete account = explained decision; context decision frame * coherent message = aligned assessment | Contextual assessment brief |
| E[judging, knowledge] | Required decision evidence * fundamental understanding; Adequate assessment context * competent expertise; Complete decision record * thorough mastery; Coherent assessment rationale * coherent understanding | judging * knowledge = competence decision frame | competence decision frame * required understanding = understood decision; competence decision frame * expertise = competent assessment; competence decision frame * mastery = mature assessment; competence decision frame * coherent understanding = stable model | Competent assessment model |
| E[judging, wisdom] | Required decision evidence * essential discernment; Adequate assessment context * adequate judgment; Complete decision record * holistic insight; Coherent assessment rationale * principled reasoning | judging * wisdom = discernment decision frame | discernment decision frame * required discernment = prudent decision; discernment decision frame * adequate judgment = reasoned assessment; discernment decision frame * holistic insight = integral decision; discernment decision frame * principled reasoning = principled rationale | Principled decision rationale |
| E[reviewing, data] | Required audit evidence * essential fact; Adequate inspection context * adequate evidence; Complete audit record * comprehensive record; Coherent audit rationale * reliable measurement | reviewing * data = evidence audit frame | evidence audit frame * required fact = sourced audit; evidence audit frame * adequate proof = supported audit; evidence audit frame * complete record = traced audit; evidence audit frame * reliable measure = stable audit | Source-grounded audit basis |
| E[reviewing, information] | Required audit evidence * essential signal; Adequate inspection context * adequate context; Complete audit record * comprehensive account; Coherent audit rationale * coherent message | reviewing * information = context audit frame | context audit frame * required signal = audit cue; context audit frame * adequate context = review setting; context audit frame * complete account = explained audit; context audit frame * coherent message = aligned audit | Contextual audit brief |
| E[reviewing, knowledge] | Required audit evidence * fundamental understanding; Adequate inspection context * competent expertise; Complete audit record * thorough mastery; Coherent audit rationale * coherent understanding | reviewing * knowledge = competence audit frame | competence audit frame * required understanding = understood audit; competence audit frame * expertise = competent audit; competence audit frame * mastery = mature audit; competence audit frame * coherent understanding = stable model | Competent audit model |
| E[reviewing, wisdom] | Required audit evidence * essential discernment; Adequate inspection context * adequate judgment; Complete audit record * holistic insight; Coherent audit rationale * principled reasoning | reviewing * wisdom = discernment audit frame | discernment audit frame * required discernment = prudent audit; discernment audit frame * adequate judgment = reasoned audit; discernment audit frame * holistic insight = integral audit; discernment audit frame * principled reasoning = principled rationale | Principled audit rationale |

### Result

| | **data** | **information** | **knowledge** | **wisdom** |
|---|---|---|---|---|
| **guiding** | Source-grounded orientation | Contextual orientation brief | Competent orientation model | Principled orientation rationale |
| **applying** | Source-grounded action basis | Contextual execution brief | Competent execution model | Principled execution rationale |
| **judging** | Source-grounded decision basis | Contextual assessment brief | Competent assessment model | Principled decision rationale |
| **reviewing** | Source-grounded audit basis | Contextual audit brief | Competent audit model | Principled audit rationale |

---

## Matrix Summary

### Matrix C - Formulation

| | **necessity** | **sufficiency** | **completeness** | **consistency** |
|---|---|---|---|---|
| **normative** | Binding evidence threshold | Adequacy control basis | Complete compliance basis | Coherent compliance basis |
| **operative** | Execution input basis | Workable context basis | Complete process account | Repeatable process message |
| **evaluative** | Judgment evidence basis | Contextual merit basis | Holistic appraisal basis | Coherent quality rationale |

### Matrix F - Requirements

| | **necessity** | **sufficiency** | **completeness** | **consistency** |
|---|---|---|---|---|
| **normative** | Required evidence boundary | Sufficient proof package | Complete obligation record | Consistent control rationale |
| **operative** | Required input pathway | Sufficient execution context | Complete workflow record | Consistent process rationale |
| **evaluative** | Required review basis | Sufficient judgment context | Complete assessment record | Consistent appraisal rationale |

### Matrix D - Objectives

| | **guiding** | **applying** | **judging** | **reviewing** |
|---|---|---|---|---|
| **normative** | Controlled obligation direction | Enforced practice basis | Determinate compliance basis | Auditable control record |
| **operative** | Actionable workflow direction | Validated execution basis | Measured performance basis | Auditable process record |
| **evaluative** | Principled value direction | Reasoned merit basis | Determinate worth basis | Auditable quality record |

### Matrix K - Transpose of D

| | **normative** | **operative** | **evaluative** |
|---|---|---|---|
| **guiding** | Controlled obligation direction | Actionable workflow direction | Principled value direction |
| **applying** | Enforced practice basis | Validated execution basis | Reasoned merit basis |
| **judging** | Determinate compliance basis | Measured performance basis | Determinate worth basis |
| **reviewing** | Auditable control record | Auditable process record | Auditable quality record |

### Matrix G - Truncation of B

| | **necessity** | **sufficiency** | **completeness** | **consistency** |
|---|---|---|---|---|
| **data** | essential fact | adequate evidence | comprehensive record | reliable measurement |
| **information** | essential signal | adequate context | comprehensive account | coherent message |
| **knowledge** | fundamental understanding | competent expertise | thorough mastery | coherent understanding |

### Matrix X - Verification

| | **necessity** | **sufficiency** | **completeness** | **consistency** |
|---|---|---|---|---|
| **guiding** | Required direction evidence | Adequate direction context | Complete direction record | Coherent direction rationale |
| **applying** | Required practice evidence | Adequate execution context | Complete practice record | Coherent execution rationale |
| **judging** | Required decision evidence | Adequate assessment context | Complete decision record | Coherent assessment rationale |
| **reviewing** | Required audit evidence | Adequate inspection context | Complete audit record | Coherent audit rationale |

### Matrix T - Transpose of B

| | **data** | **information** | **knowledge** | **wisdom** |
|---|---|---|---|---|
| **necessity** | essential fact | essential signal | fundamental understanding | essential discernment |
| **sufficiency** | adequate evidence | adequate context | competent expertise | adequate judgment |
| **completeness** | comprehensive record | comprehensive account | thorough mastery | holistic insight |
| **consistency** | reliable measurement | coherent message | coherent understanding | principled reasoning |

### Matrix E - Evaluation

| | **data** | **information** | **knowledge** | **wisdom** |
|---|---|---|---|---|
| **guiding** | Source-grounded orientation | Contextual orientation brief | Competent orientation model | Principled orientation rationale |
| **applying** | Source-grounded action basis | Contextual execution brief | Competent execution model | Principled execution rationale |
| **judging** | Source-grounded decision basis | Contextual assessment brief | Competent assessment model | Principled decision rationale |
| **reviewing** | Source-grounded audit basis | Contextual audit brief | Competent audit model | Principled audit rationale |

## Audit Result

Semantic audit: PASS. Final result cells are populated, compact phrases; no final result cell contains `Sigma`, `intersection`, or unresolved addition operators. `_SEMANTIC.md` is a lens only and does not authorize engineering values, standards content, implementation details, or professional conclusions.
