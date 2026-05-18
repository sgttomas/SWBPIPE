# Deliverable: DEL-13-02 Constraint entity and provenance model

**Generated:** 2026-05-04
**DECOMP_VARIANT:** SOFTWARE
**Perspective:** This deliverable frames how constraint records carry provenance, missing-data visibility, and reviewable diagnostic meaning within a schema-backed physical-design model. The lens is for organizing questions about representation, evidence, verification, and professional-boundary controls; it is not an engineering authority.
**Framework:** Chirality Semantic Algebra
**Inputs Read:**
- _CONTEXT.md - deliverable identity, scope coverage, architecture basis, and context envelope.
- _STATUS.md - lifecycle state before semantic generation.
- Datasheet.md - source-grounded descriptive draft.
- Specification.md - source-grounded normative draft.
- Guidance.md - source-grounded directional draft.
- Procedure.md - source-grounded operational draft.
- _REFERENCES.md - deliverable-local reference index.

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

### Construction: Dot product A . B

| Cell | Intermediate collection L | Step 1: axis anchor | Step 2: projections | Step 3: centroid |
|---|---|---|---|---|
| C[normative,necessity] | {prescriptive direction * essential fact = prescribed evidence; mandatory practice * essential signal = required signal; compliance determination * fundamental understanding = compliance basis; regulatory audit * essential discernment = audit discernment} | a = normative * necessity = binding need frame | a * prescribed evidence = bounded evidence duty; a * required signal = required input cue; a * compliance basis = obligation basis; a * audit discernment = authority check | binding evidence basis |
| C[normative,sufficiency] | {prescriptive direction * adequate evidence = prescribed proof; mandatory practice * adequate context = required context; compliance determination * competent expertise = competent compliance; regulatory audit * adequate judgment = audit judgment} | a = normative * sufficiency = proof adequacy frame | a * prescribed proof = proof directive; a * required context = adequate obligation context; a * competent compliance = competent proof; a * audit judgment = reviewable proof | sufficient compliance proof |
| C[normative,completeness] | {prescriptive direction * comprehensive record = prescribed record; mandatory practice * comprehensive account = required account; compliance determination * thorough mastery = complete compliance; regulatory audit * holistic insight = audit whole} | a = normative * completeness = obligation closure frame | a * prescribed record = complete directive record; a * required account = account obligation; a * complete compliance = closure proof; a * audit whole = audit completeness | complete obligation record |
| C[normative,consistency] | {prescriptive direction * reliable measurement = reliable direction; mandatory practice * coherent message = coherent practice; compliance determination * coherent understanding = stable compliance; regulatory audit * principled reasoning = principled audit} | a = normative * consistency = stable control frame | a * reliable direction = stable directive; a * coherent practice = consistent obligation; a * stable compliance = controlled finding; a * principled audit = audit rationale | coherent control basis |
| C[operative,necessity] | {procedural direction * essential fact = process fact; practical execution * essential signal = execution signal; performance assessment * fundamental understanding = performance basis; process audit * essential discernment = audit cue} | a = operative * necessity = work need frame | a * process fact = required work fact; a * execution signal = action cue; a * performance basis = work basis; a * audit cue = process need | required execution input |
| C[operative,sufficiency] | {procedural direction * adequate evidence = process proof; practical execution * adequate context = usable context; performance assessment * competent expertise = performance competence; process audit * adequate judgment = audit adequacy} | a = operative * sufficiency = work adequacy frame | a * process proof = usable proof; a * usable context = action context; a * performance competence = competent execution; a * audit adequacy = process proof | usable workflow evidence |
| C[operative,completeness] | {procedural direction * comprehensive record = process record; practical execution * comprehensive account = execution account; performance assessment * thorough mastery = performance completeness; process audit * holistic insight = audit whole} | a = operative * completeness = work closure frame | a * process record = complete process record; a * execution account = action account; a * performance completeness = full work basis; a * audit whole = closure evidence | complete work record |
| C[operative,consistency] | {procedural direction * reliable measurement = reliable procedure; practical execution * coherent message = coherent action; performance assessment * coherent understanding = stable performance; process audit * principled reasoning = process rationale} | a = operative * consistency = stable work frame | a * reliable procedure = stable method; a * coherent action = repeatable action; a * stable performance = reliable work basis; a * process rationale = durable rationale | stable execution message |
| C[evaluative,necessity] | {value orientation * essential fact = value fact; merit application * essential signal = merit signal; worth determination * fundamental understanding = worth basis; quality appraisal * essential discernment = appraisal discernment} | a = evaluative * necessity = review need frame | a * value fact = review fact basis; a * merit signal = appraisal cue; a * worth basis = decision basis; a * appraisal discernment = review discernment | review evidence basis |
| C[evaluative,sufficiency] | {value orientation * adequate evidence = value proof; merit application * adequate context = merit context; worth determination * competent expertise = worth competence; quality appraisal * adequate judgment = appraisal judgment} | a = evaluative * sufficiency = appraisal adequacy frame | a * value proof = adequate review proof; a * merit context = judgment context; a * worth competence = competent appraisal; a * appraisal judgment = review threshold | adequate appraisal context |
| C[evaluative,completeness] | {value orientation * comprehensive record = value record; merit application * comprehensive account = merit account; worth determination * thorough mastery = worth completeness; quality appraisal * holistic insight = appraisal whole} | a = evaluative * completeness = review closure frame | a * value record = complete review record; a * merit account = appraisal account; a * worth completeness = decision closure; a * appraisal whole = full quality view | complete judgment record |
| C[evaluative,consistency] | {value orientation * reliable measurement = reliable value; merit application * coherent message = coherent merit; worth determination * coherent understanding = stable worth; quality appraisal * principled reasoning = principled appraisal} | a = evaluative * consistency = stable review frame | a * reliable value = stable value basis; a * coherent merit = coherent appraisal cue; a * stable worth = durable judgment; a * principled appraisal = review rationale | coherent review rationale |

### Result

| | **necessity** | **sufficiency** | **completeness** | **consistency** |
|---|---|---|---|---|
| **normative** | binding evidence basis | sufficient compliance proof | complete obligation record | coherent control basis |
| **operative** | required execution input | usable workflow evidence | complete work record | stable execution message |
| **evaluative** | review evidence basis | adequate appraisal context | complete judgment record | coherent review rationale |

## Matrix F - Requirements (3x4)

### Construction: Dot product C . B

| Cell | Intermediate collection L | Step 1: axis anchor | Step 2: projections | Step 3: centroid |
|---|---|---|---|---|
| F[normative,necessity] | {binding evidence basis * essential fact = obligated fact; sufficient compliance proof * essential signal = proof signal; complete obligation record * fundamental understanding = obligation understanding; coherent control basis * essential discernment = control discernment} | a = normative * necessity = binding need frame | a * obligated fact = input duty; a * proof signal = required proof cue; a * obligation understanding = obligation basis; a * control discernment = control duty | binding input obligation |
| F[normative,sufficiency] | {binding evidence basis * adequate evidence = obligated proof; sufficient compliance proof * adequate context = proof context; complete obligation record * competent expertise = competent obligation; coherent control basis * adequate judgment = control judgment} | a = normative * sufficiency = proof adequacy frame | a * obligated proof = proof obligation; a * proof context = sufficient control context; a * competent obligation = competent requirement; a * control judgment = defensible proof | proof threshold rule |
| F[normative,completeness] | {binding evidence basis * comprehensive record = obligated record; sufficient compliance proof * comprehensive account = proof account; complete obligation record * thorough mastery = obligation closure; coherent control basis * holistic insight = control whole} | a = normative * completeness = obligation closure frame | a * obligated record = complete duty record; a * proof account = proof envelope; a * obligation closure = closure requirement; a * control whole = full control scope | complete compliance envelope |
| F[normative,consistency] | {binding evidence basis * reliable measurement = reliable obligation; sufficient compliance proof * coherent message = coherent proof; complete obligation record * coherent understanding = stable obligation; coherent control basis * principled reasoning = principled control} | a = normative * consistency = stable control frame | a * reliable obligation = stable duty; a * coherent proof = durable proof; a * stable obligation = fixed meaning; a * principled control = control rationale | stable requirement meaning |
| F[operative,necessity] | {required execution input * essential fact = required fact; usable workflow evidence * essential signal = workflow cue; complete work record * fundamental understanding = work basis; stable execution message * essential discernment = execution discernment} | a = operative * necessity = work need frame | a * required fact = action precondition; a * workflow cue = work trigger; a * work basis = necessary task basis; a * execution discernment = precondition judgment | required action precondition |
| F[operative,sufficiency] | {required execution input * adequate evidence = execution proof; usable workflow evidence * adequate context = workflow context; complete work record * competent expertise = work competence; stable execution message * adequate judgment = execution judgment} | a = operative * sufficiency = work adequacy frame | a * execution proof = workable proof; a * workflow context = usable task context; a * work competence = competent practice; a * execution judgment = adequate work basis | workable evidence threshold |
| F[operative,completeness] | {required execution input * comprehensive record = input record; usable workflow evidence * comprehensive account = workflow account; complete work record * thorough mastery = work closure; stable execution message * holistic insight = execution whole} | a = operative * completeness = work closure frame | a * input record = complete input record; a * workflow account = full workflow account; a * work closure = finished contract; a * execution whole = complete method view | complete execution contract |
| F[operative,consistency] | {required execution input * reliable measurement = reliable input; usable workflow evidence * coherent message = coherent workflow; complete work record * coherent understanding = stable work; stable execution message * principled reasoning = execution rationale} | a = operative * consistency = stable work frame | a * reliable input = stable input basis; a * coherent workflow = repeatable workflow; a * stable work = durable execution; a * execution rationale = grounded method | stable workflow basis |
| F[evaluative,necessity] | {review evidence basis * essential fact = review fact; adequate appraisal context * essential signal = appraisal signal; complete judgment record * fundamental understanding = judgment basis; coherent review rationale * essential discernment = rationale discernment} | a = evaluative * necessity = review need frame | a * review fact = decision precondition; a * appraisal signal = review cue; a * judgment basis = finding basis; a * rationale discernment = review need | reviewable decision basis |
| F[evaluative,sufficiency] | {review evidence basis * adequate evidence = review proof; adequate appraisal context * adequate context = appraisal context; complete judgment record * competent expertise = judgment competence; coherent review rationale * adequate judgment = rationale judgment} | a = evaluative * sufficiency = appraisal adequacy frame | a * review proof = adequate review proof; a * appraisal context = judgment context; a * judgment competence = competent decision; a * rationale judgment = defensible appraisal | judgment proof threshold |
| F[evaluative,completeness] | {review evidence basis * comprehensive record = review record; adequate appraisal context * comprehensive account = appraisal account; complete judgment record * thorough mastery = judgment closure; coherent review rationale * holistic insight = rationale whole} | a = evaluative * completeness = review closure frame | a * review record = complete review record; a * appraisal account = full appraisal account; a * judgment closure = closure finding; a * rationale whole = full rationale scope | complete appraisal envelope |
| F[evaluative,consistency] | {review evidence basis * reliable measurement = reliable review; adequate appraisal context * coherent message = coherent appraisal; complete judgment record * coherent understanding = stable judgment; coherent review rationale * principled reasoning = principled rationale} | a = evaluative * consistency = stable review frame | a * reliable review = stable review basis; a * coherent appraisal = aligned appraisal; a * stable judgment = durable decision; a * principled rationale = grounded review | stable review criterion |

### Result

| | **necessity** | **sufficiency** | **completeness** | **consistency** |
|---|---|---|---|---|
| **normative** | binding input obligation | proof threshold rule | complete compliance envelope | stable requirement meaning |
| **operative** | required action precondition | workable evidence threshold | complete execution contract | stable workflow basis |
| **evaluative** | reviewable decision basis | judgment proof threshold | complete appraisal envelope | stable review criterion |

## Matrix D - Objectives (3x4)

### Construction: Addition A + resolution-transformed F

| Cell | Intermediate collection L | Step 1: axis anchor | Step 2: projections | Step 3: centroid |
|---|---|---|---|---|
| D[normative,guiding] | {prescriptive direction, resolution * binding input obligation = resolved input duty} | a = normative * guiding = authority direction frame | a * prescriptive direction = directive authority; a * resolved input duty = closed input duty | authoritative constraint direction |
| D[normative,applying] | {mandatory practice, resolution * proof threshold rule = resolved proof rule} | a = normative * applying = authority practice frame | a * mandatory practice = required practice; a * resolved proof rule = enforceable proof | enforced schema practice |
| D[normative,judging] | {compliance determination, resolution * complete compliance envelope = resolved compliance scope} | a = normative * judging = authority finding frame | a * compliance determination = binding finding; a * resolved compliance scope = closed finding scope | compliance finding basis |
| D[normative,reviewing] | {regulatory audit, resolution * stable requirement meaning = resolved requirement meaning} | a = normative * reviewing = authority audit frame | a * regulatory audit = audit control; a * resolved requirement meaning = stable audit meaning | audit-ready control record |
| D[operative,guiding] | {procedural direction, resolution * required action precondition = resolved precondition} | a = operative * guiding = work direction frame | a * procedural direction = method direction; a * resolved precondition = ready action basis | procedure-ready direction |
| D[operative,applying] | {practical execution, resolution * workable evidence threshold = resolved work proof} | a = operative * applying = work practice frame | a * practical execution = applied method; a * resolved work proof = controlled work proof | controlled execution method |
| D[operative,judging] | {performance assessment, resolution * complete execution contract = resolved work contract} | a = operative * judging = work finding frame | a * performance assessment = performance finding; a * resolved work contract = closed work scope | performance finding basis |
| D[operative,reviewing] | {process audit, resolution * stable workflow basis = resolved workflow basis} | a = operative * reviewing = work audit frame | a * process audit = process record; a * resolved workflow basis = stable work evidence | process evidence record |
| D[evaluative,guiding] | {value orientation, resolution * reviewable decision basis = resolved decision basis} | a = evaluative * guiding = review direction frame | a * value orientation = value rationale; a * resolved decision basis = review direction basis | value-framed rationale |
| D[evaluative,applying] | {merit application, resolution * judgment proof threshold = resolved judgment proof} | a = evaluative * applying = review practice frame | a * merit application = applied appraisal; a * resolved judgment proof = proofed appraisal | judgment application method |
| D[evaluative,judging] | {worth determination, resolution * complete appraisal envelope = resolved appraisal scope} | a = evaluative * judging = review finding frame | a * worth determination = merit finding; a * resolved appraisal scope = complete appraisal basis | merit finding basis |
| D[evaluative,reviewing] | {quality appraisal, resolution * stable review criterion = resolved review criterion} | a = evaluative * reviewing = review audit frame | a * quality appraisal = quality record; a * resolved review criterion = stable review evidence | quality evidence record |

### Result

| | **guiding** | **applying** | **judging** | **reviewing** |
|---|---|---|---|---|
| **normative** | authoritative constraint direction | enforced schema practice | compliance finding basis | audit-ready control record |
| **operative** | procedure-ready direction | controlled execution method | performance finding basis | process evidence record |
| **evaluative** | value-framed rationale | judgment application method | merit finding basis | quality evidence record |

## Matrix K - Transpose of D (4x3)

### Construction: K(i,j) = D(j,i)

### Result

| | **normative** | **operative** | **evaluative** |
|---|---|---|---|
| **guiding** | authoritative constraint direction | procedure-ready direction | value-framed rationale |
| **applying** | enforced schema practice | controlled execution method | judgment application method |
| **judging** | compliance finding basis | performance finding basis | merit finding basis |
| **reviewing** | audit-ready control record | process evidence record | quality evidence record |

## Matrix G - Truncation of B (3x4)

### Construction: remove `wisdom` row from B

### Result

| | **necessity** | **sufficiency** | **completeness** | **consistency** |
|---|---|---|---|---|
| **data** | essential fact | adequate evidence | comprehensive record | reliable measurement |
| **information** | essential signal | adequate context | comprehensive account | coherent message |
| **knowledge** | fundamental understanding | competent expertise | thorough mastery | coherent understanding |

## Matrix X - Verification (4x4)

### Construction: Dot product K . G

| Cell | Intermediate collection L | Step 1: axis anchor | Step 2: projections | Step 3: centroid |
|---|---|---|---|---|
| X[guiding,necessity] | {authoritative constraint direction * essential fact = authority fact cue; procedure-ready direction * essential signal = procedure signal cue; value-framed rationale * fundamental understanding = rationale basis cue} | a = guiding * necessity = direction need frame | a * authority fact cue = needed direction fact; a * procedure signal cue = needed instruction signal; a * rationale basis cue = needed rationale basis | directional evidence need |
| X[guiding,sufficiency] | {authoritative constraint direction * adequate evidence = authority proof cue; procedure-ready direction * adequate context = procedure context cue; value-framed rationale * competent expertise = rationale expertise cue} | a = guiding * sufficiency = direction adequacy frame | a * authority proof cue = adequate direction proof; a * procedure context cue = usable instruction context; a * rationale expertise cue = competent rationale | instruction proof threshold |
| X[guiding,completeness] | {authoritative constraint direction * comprehensive record = authority record cue; procedure-ready direction * comprehensive account = procedure account cue; value-framed rationale * thorough mastery = rationale closure cue} | a = guiding * completeness = direction closure frame | a * authority record cue = complete direction record; a * procedure account cue = full instruction account; a * rationale closure cue = closed rationale basis | complete direction record |
| X[guiding,consistency] | {authoritative constraint direction * reliable measurement = authority measurement cue; procedure-ready direction * coherent message = procedure message cue; value-framed rationale * coherent understanding = rationale coherence cue} | a = guiding * consistency = stable direction frame | a * authority measurement cue = stable direction evidence; a * procedure message cue = coherent instruction; a * rationale coherence cue = aligned rationale | coherent guidance trace |
| X[applying,necessity] | {enforced schema practice * essential fact = schema fact cue; controlled execution method * essential signal = method signal cue; judgment application method * fundamental understanding = judgment basis cue} | a = applying * necessity = practice need frame | a * schema fact cue = needed practice fact; a * method signal cue = action input cue; a * judgment basis cue = decision input basis | implementation input basis |
| X[applying,sufficiency] | {enforced schema practice * adequate evidence = schema proof cue; controlled execution method * adequate context = method context cue; judgment application method * competent expertise = judgment expertise cue} | a = applying * sufficiency = practice adequacy frame | a * schema proof cue = adequate practice proof; a * method context cue = usable method context; a * judgment expertise cue = competent application | practice evidence threshold |
| X[applying,completeness] | {enforced schema practice * comprehensive record = schema record cue; controlled execution method * comprehensive account = method account cue; judgment application method * thorough mastery = judgment closure cue} | a = applying * completeness = practice closure frame | a * schema record cue = complete practice record; a * method account cue = full method account; a * judgment closure cue = closed application basis | complete action record |
| X[applying,consistency] | {enforced schema practice * reliable measurement = schema measurement cue; controlled execution method * coherent message = method message cue; judgment application method * coherent understanding = judgment coherence cue} | a = applying * consistency = stable practice frame | a * schema measurement cue = stable schema evidence; a * method message cue = coherent method; a * judgment coherence cue = aligned application | stable practice trace |
| X[judging,necessity] | {compliance finding basis * essential fact = finding fact cue; performance finding basis * essential signal = performance signal cue; merit finding basis * fundamental understanding = merit basis cue} | a = judging * necessity = finding need frame | a * finding fact cue = decision fact need; a * performance signal cue = finding input signal; a * merit basis cue = appraisal input basis | decision evidence need |
| X[judging,sufficiency] | {compliance finding basis * adequate evidence = finding proof cue; performance finding basis * adequate context = performance context cue; merit finding basis * competent expertise = merit expertise cue} | a = judging * sufficiency = finding adequacy frame | a * finding proof cue = adequate decision proof; a * performance context cue = finding context; a * merit expertise cue = competent finding | finding proof threshold |
| X[judging,completeness] | {compliance finding basis * comprehensive record = finding record cue; performance finding basis * comprehensive account = performance account cue; merit finding basis * thorough mastery = merit closure cue} | a = judging * completeness = finding closure frame | a * finding record cue = complete decision record; a * performance account cue = full finding account; a * merit closure cue = closed appraisal basis | complete decision record |
| X[judging,consistency] | {compliance finding basis * reliable measurement = finding measurement cue; performance finding basis * coherent message = performance message cue; merit finding basis * coherent understanding = merit coherence cue} | a = judging * consistency = stable finding frame | a * finding measurement cue = stable finding evidence; a * performance message cue = coherent decision; a * merit coherence cue = aligned finding | coherent finding trace |
| X[reviewing,necessity] | {audit-ready control record * essential fact = audit fact cue; process evidence record * essential signal = process signal cue; quality evidence record * fundamental understanding = quality basis cue} | a = reviewing * necessity = audit need frame | a * audit fact cue = audit input need; a * process signal cue = review signal need; a * quality basis cue = review basis | audit input basis |
| X[reviewing,sufficiency] | {audit-ready control record * adequate evidence = audit proof cue; process evidence record * adequate context = process context cue; quality evidence record * competent expertise = quality expertise cue} | a = reviewing * sufficiency = audit adequacy frame | a * audit proof cue = sufficient audit proof; a * process context cue = review context; a * quality expertise cue = competent review | audit evidence threshold |
| X[reviewing,completeness] | {audit-ready control record * comprehensive record = audit record cue; process evidence record * comprehensive account = process account cue; quality evidence record * thorough mastery = quality closure cue} | a = reviewing * completeness = audit closure frame | a * audit record cue = complete audit record; a * process account cue = full review account; a * quality closure cue = closed quality basis | complete audit record |
| X[reviewing,consistency] | {audit-ready control record * reliable measurement = audit measurement cue; process evidence record * coherent message = process message cue; quality evidence record * coherent understanding = quality coherence cue} | a = reviewing * consistency = stable audit frame | a * audit measurement cue = stable audit evidence; a * process message cue = coherent review signal; a * quality coherence cue = aligned review | stable review trace |

### Result

| | **necessity** | **sufficiency** | **completeness** | **consistency** |
|---|---|---|---|---|
| **guiding** | directional evidence need | instruction proof threshold | complete direction record | coherent guidance trace |
| **applying** | implementation input basis | practice evidence threshold | complete action record | stable practice trace |
| **judging** | decision evidence need | finding proof threshold | complete decision record | coherent finding trace |
| **reviewing** | audit input basis | audit evidence threshold | complete audit record | stable review trace |

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

### Construction: Dot product X . T

| Cell | Intermediate collection L | Step 1: axis anchor | Step 2: projections | Step 3: centroid |
|---|---|---|---|---|
| E[guiding,data] | {directional evidence need * essential fact = direction fact cue; instruction proof threshold * adequate evidence = instruction evidence cue; complete direction record * comprehensive record = direction record cue; coherent guidance trace * reliable measurement = trace measurement cue} | a = guiding * data = direction source frame | a * direction fact cue = source direction fact; a * instruction evidence cue = source instruction proof; a * direction record cue = source direction record; a * trace measurement cue = measured source trace | source direction basis |
| E[guiding,information] | {directional evidence need * essential signal = direction signal cue; instruction proof threshold * adequate context = instruction context cue; complete direction record * comprehensive account = direction account cue; coherent guidance trace * coherent message = trace message cue} | a = guiding * information = direction context frame | a * direction signal cue = contextual direction signal; a * instruction context cue = contextual instruction; a * direction account cue = contextual account; a * trace message cue = coherent context trace | context direction basis |
| E[guiding,knowledge] | {directional evidence need * fundamental understanding = direction understanding cue; instruction proof threshold * competent expertise = instruction expertise cue; complete direction record * thorough mastery = direction mastery cue; coherent guidance trace * coherent understanding = trace understanding cue} | a = guiding * knowledge = direction expertise frame | a * direction understanding cue = expert direction basis; a * instruction expertise cue = expert instruction proof; a * direction mastery cue = mastered direction record; a * trace understanding cue = coherent expertise trace | expert direction basis |
| E[guiding,wisdom] | {directional evidence need * essential discernment = direction discernment cue; instruction proof threshold * adequate judgment = instruction judgment cue; complete direction record * holistic insight = direction insight cue; coherent guidance trace * principled reasoning = trace reasoning cue} | a = guiding * wisdom = direction discernment frame | a * direction discernment cue = principled direction need; a * instruction judgment cue = reasoned instruction; a * direction insight cue = holistic direction record; a * trace reasoning cue = principled trace | principled direction basis |
| E[applying,data] | {implementation input basis * essential fact = practice fact cue; practice evidence threshold * adequate evidence = practice proof cue; complete action record * comprehensive record = action record cue; stable practice trace * reliable measurement = practice measurement cue} | a = applying * data = practice source frame | a * practice fact cue = source practice fact; a * practice proof cue = source practice proof; a * action record cue = source action record; a * practice measurement cue = measured practice trace | source practice basis |
| E[applying,information] | {implementation input basis * essential signal = practice signal cue; practice evidence threshold * adequate context = practice context cue; complete action record * comprehensive account = action account cue; stable practice trace * coherent message = practice message cue} | a = applying * information = practice context frame | a * practice signal cue = contextual practice signal; a * practice context cue = usable practice context; a * action account cue = contextual action account; a * practice message cue = coherent practice trace | context practice basis |
| E[applying,knowledge] | {implementation input basis * fundamental understanding = practice understanding cue; practice evidence threshold * competent expertise = practice expertise cue; complete action record * thorough mastery = action mastery cue; stable practice trace * coherent understanding = practice understanding trace} | a = applying * knowledge = practice expertise frame | a * practice understanding cue = expert practice basis; a * practice expertise cue = competent practice proof; a * action mastery cue = mastered action record; a * practice understanding trace = coherent expertise trace | expert practice basis |
| E[applying,wisdom] | {implementation input basis * essential discernment = practice discernment cue; practice evidence threshold * adequate judgment = practice judgment cue; complete action record * holistic insight = action insight cue; stable practice trace * principled reasoning = practice reasoning cue} | a = applying * wisdom = practice discernment frame | a * practice discernment cue = principled practice need; a * practice judgment cue = reasoned practice proof; a * action insight cue = holistic action record; a * practice reasoning cue = principled practice trace | principled practice basis |
| E[judging,data] | {decision evidence need * essential fact = finding fact cue; finding proof threshold * adequate evidence = finding proof cue; complete decision record * comprehensive record = decision record cue; coherent finding trace * reliable measurement = finding measurement cue} | a = judging * data = finding source frame | a * finding fact cue = source finding fact; a * finding proof cue = source finding proof; a * decision record cue = source decision record; a * finding measurement cue = measured finding trace | source finding basis |
| E[judging,information] | {decision evidence need * essential signal = finding signal cue; finding proof threshold * adequate context = finding context cue; complete decision record * comprehensive account = decision account cue; coherent finding trace * coherent message = finding message cue} | a = judging * information = finding context frame | a * finding signal cue = contextual finding signal; a * finding context cue = usable finding context; a * decision account cue = contextual decision account; a * finding message cue = coherent finding trace | context finding basis |
| E[judging,knowledge] | {decision evidence need * fundamental understanding = finding understanding cue; finding proof threshold * competent expertise = finding expertise cue; complete decision record * thorough mastery = decision mastery cue; coherent finding trace * coherent understanding = finding understanding trace} | a = judging * knowledge = finding expertise frame | a * finding understanding cue = expert finding basis; a * finding expertise cue = competent finding proof; a * decision mastery cue = mastered decision record; a * finding understanding trace = coherent expertise trace | expert finding basis |
| E[judging,wisdom] | {decision evidence need * essential discernment = finding discernment cue; finding proof threshold * adequate judgment = finding judgment cue; complete decision record * holistic insight = decision insight cue; coherent finding trace * principled reasoning = finding reasoning cue} | a = judging * wisdom = finding discernment frame | a * finding discernment cue = principled finding need; a * finding judgment cue = reasoned finding proof; a * decision insight cue = holistic decision record; a * finding reasoning cue = principled finding trace | principled finding basis |
| E[reviewing,data] | {audit input basis * essential fact = audit fact cue; audit evidence threshold * adequate evidence = audit proof cue; complete audit record * comprehensive record = audit record cue; stable review trace * reliable measurement = review measurement cue} | a = reviewing * data = audit source frame | a * audit fact cue = source audit fact; a * audit proof cue = source audit proof; a * audit record cue = source audit record; a * review measurement cue = measured review trace | source audit basis |
| E[reviewing,information] | {audit input basis * essential signal = audit signal cue; audit evidence threshold * adequate context = audit context cue; complete audit record * comprehensive account = audit account cue; stable review trace * coherent message = review message cue} | a = reviewing * information = audit context frame | a * audit signal cue = contextual audit signal; a * audit context cue = usable audit context; a * audit account cue = contextual audit account; a * review message cue = coherent review trace | context audit basis |
| E[reviewing,knowledge] | {audit input basis * fundamental understanding = audit understanding cue; audit evidence threshold * competent expertise = audit expertise cue; complete audit record * thorough mastery = audit mastery cue; stable review trace * coherent understanding = review understanding cue} | a = reviewing * knowledge = audit expertise frame | a * audit understanding cue = expert audit basis; a * audit expertise cue = competent audit proof; a * audit mastery cue = mastered audit record; a * review understanding cue = coherent expertise trace | expert audit basis |
| E[reviewing,wisdom] | {audit input basis * essential discernment = audit discernment cue; audit evidence threshold * adequate judgment = audit judgment cue; complete audit record * holistic insight = audit insight cue; stable review trace * principled reasoning = review reasoning cue} | a = reviewing * wisdom = audit discernment frame | a * audit discernment cue = principled audit need; a * audit judgment cue = reasoned audit proof; a * audit insight cue = holistic audit record; a * review reasoning cue = principled review trace | principled audit basis |

### Result

| | **data** | **information** | **knowledge** | **wisdom** |
|---|---|---|---|---|
| **guiding** | source direction basis | context direction basis | expert direction basis | principled direction basis |
| **applying** | source practice basis | context practice basis | expert practice basis | principled practice basis |
| **judging** | source finding basis | context finding basis | expert finding basis | principled finding basis |
| **reviewing** | source audit basis | context audit basis | expert audit basis | principled audit basis |

## Matrix Summary

### Matrix C - Formulation

| | **necessity** | **sufficiency** | **completeness** | **consistency** |
|---|---|---|---|---|
| **normative** | binding evidence basis | sufficient compliance proof | complete obligation record | coherent control basis |
| **operative** | required execution input | usable workflow evidence | complete work record | stable execution message |
| **evaluative** | review evidence basis | adequate appraisal context | complete judgment record | coherent review rationale |

### Matrix F - Requirements

| | **necessity** | **sufficiency** | **completeness** | **consistency** |
|---|---|---|---|---|
| **normative** | binding input obligation | proof threshold rule | complete compliance envelope | stable requirement meaning |
| **operative** | required action precondition | workable evidence threshold | complete execution contract | stable workflow basis |
| **evaluative** | reviewable decision basis | judgment proof threshold | complete appraisal envelope | stable review criterion |

### Matrix D - Objectives

| | **guiding** | **applying** | **judging** | **reviewing** |
|---|---|---|---|---|
| **normative** | authoritative constraint direction | enforced schema practice | compliance finding basis | audit-ready control record |
| **operative** | procedure-ready direction | controlled execution method | performance finding basis | process evidence record |
| **evaluative** | value-framed rationale | judgment application method | merit finding basis | quality evidence record |

### Matrix K - Transpose of D

| | **normative** | **operative** | **evaluative** |
|---|---|---|---|
| **guiding** | authoritative constraint direction | procedure-ready direction | value-framed rationale |
| **applying** | enforced schema practice | controlled execution method | judgment application method |
| **judging** | compliance finding basis | performance finding basis | merit finding basis |
| **reviewing** | audit-ready control record | process evidence record | quality evidence record |

### Matrix G - Truncation of B

| | **necessity** | **sufficiency** | **completeness** | **consistency** |
|---|---|---|---|---|
| **data** | essential fact | adequate evidence | comprehensive record | reliable measurement |
| **information** | essential signal | adequate context | comprehensive account | coherent message |
| **knowledge** | fundamental understanding | competent expertise | thorough mastery | coherent understanding |

### Matrix X - Verification

| | **necessity** | **sufficiency** | **completeness** | **consistency** |
|---|---|---|---|---|
| **guiding** | directional evidence need | instruction proof threshold | complete direction record | coherent guidance trace |
| **applying** | implementation input basis | practice evidence threshold | complete action record | stable practice trace |
| **judging** | decision evidence need | finding proof threshold | complete decision record | coherent finding trace |
| **reviewing** | audit input basis | audit evidence threshold | complete audit record | stable review trace |

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
| **guiding** | source direction basis | context direction basis | expert direction basis | principled direction basis |
| **applying** | source practice basis | context practice basis | expert practice basis | principled practice basis |
| **judging** | source finding basis | context finding basis | expert finding basis | principled finding basis |
| **reviewing** | source audit basis | context audit basis | expert audit basis | principled audit basis |

## Audit Result

Final result cells for matrices C, F, D, X, and E were checked for algebra leaks, uninterpreted expansions, and operator leaks. No final cell contains `SUM`, an algebraic addition expression, or an overlong expansion. The semantic product remains a lens and does not create engineering authority.
