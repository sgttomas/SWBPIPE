# Deliverable: DEL-16-01 Structured model operation schema

**Generated:** 2026-05-04
**DECOMP_VARIANT:** SOFTWARE
**Perspective:** This deliverable frames model edits as structured operation records that can be proposed, checked, previewed, and controlled before accepted model state changes. The lens is about operation-schema categories and validation boundaries, not implementation particulars, engineering values, or approval authority.
**Framework:** Chirality Semantic Algebra
**Audit Result:** PASS - final Result cells contain compact semantic units with no algebra/operator leaks.

**Inputs Read:**
- `_CONTEXT.md` - deliverable identity, scope, architecture basis injection.
- `_STATUS.md` - current lifecycle state before semantic pass: INITIALIZED.
- `Datasheet.md` - setup draft produced by four-documents P1/P2.
- `Specification.md` - setup draft produced by four-documents P1/P2.
- `Guidance.md` - setup draft produced by four-documents P1/P2.
- `Procedure.md` - setup draft produced by four-documents P1/P2.
- `_REFERENCES.md` - governing reference list; no additional authority claimed from semantic matrices.

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

### Construction: Dot product A by B

Intermediate collections use `L_C(row,col) = collection of A(row,k) * B(k,col)` followed by the three-step interpretation operator.

| Cell | Intermediate collection L | Step 1 axis anchor | Step 2 projections | Step 3 centroid |
|---|---|---|---|---|
| C[normative,necessity] | t1=directive fact; t2=practice signal; t3=compliance understanding; t4=audit discernment | normative * necessity = binding need | a*t1=declared input basis; a*t2=required action cue; a*t3=obligation insight; a*t4=reviewable authority | binding input rationale |
| C[normative,sufficiency] | t1=direction evidence; t2=practice context; t3=compliance expertise; t4=audit judgment | normative * sufficiency = proof threshold | a*t1=supported directive; a*t2=accepted practice basis; a*t3=qualified proof; a*t4=reviewable judgment | accepted proof threshold |
| C[normative,completeness] | t1=direction record; t2=practice account; t3=compliance mastery; t4=audit insight | normative * completeness = obligation coverage | a*t1=covered directive; a*t2=covered practice; a*t3=mastered obligation; a*t4=auditable coverage | covered obligation record |
| C[normative,consistency] | t1=direction measurement; t2=practice message; t3=compliance understanding; t4=audit reasoning | normative * consistency = stable conformance | a*t1=measured directive; a*t2=coherent practice; a*t3=aligned obligation; a*t4=reasoned audit | stable conformance message |
| C[operative,necessity] | t1=procedure fact; t2=execution signal; t3=assessment understanding; t4=process discernment | operative * necessity = work prerequisite | a*t1=procedure input; a*t2=execution cue; a*t3=assessment basis; a*t4=process filter | required execution input |
| C[operative,sufficiency] | t1=procedure evidence; t2=execution context; t3=assessment expertise; t4=process judgment | operative * sufficiency = workable basis | a*t1=evidenced procedure; a*t2=situated execution; a*t3=qualified assessment; a*t4=practical judgment | workable evidence basis |
| C[operative,completeness] | t1=procedure record; t2=execution account; t3=assessment mastery; t4=process insight | operative * completeness = work coverage | a*t1=procedure coverage; a*t2=activity account; a*t3=assessment scope; a*t4=process insight | finished activity account |
| C[operative,consistency] | t1=procedure measurement; t2=execution message; t3=assessment understanding; t4=process reasoning | operative * consistency = repeatable behavior | a*t1=measured procedure; a*t2=coherent execution; a*t3=stable assessment; a*t4=reasoned process | repeatable process signal |
| C[evaluative,necessity] | t1=orientation fact; t2=application signal; t3=worth understanding; t4=appraisal discernment | evaluative * necessity = appraisal prerequisite | a*t1=value cue; a*t2=merit signal; a*t3=worth basis; a*t4=appraisal filter | critical appraisal basis |
| C[evaluative,sufficiency] | t1=orientation evidence; t2=application context; t3=worth expertise; t4=appraisal judgment | evaluative * sufficiency = balanced support | a*t1=evidenced value; a*t2=situated merit; a*t3=qualified worth; a*t4=appraisal judgment | balanced judgment support |
| C[evaluative,completeness] | t1=orientation record; t2=application account; t3=worth mastery; t4=appraisal insight | evaluative * completeness = review breadth | a*t1=value coverage; a*t2=merit account; a*t3=worth scope; a*t4=appraisal insight | full review perspective |
| C[evaluative,consistency] | t1=orientation measurement; t2=application message; t3=worth understanding; t4=appraisal reasoning | evaluative * consistency = coherent rationale | a*t1=measured value; a*t2=coherent merit; a*t3=stable worth; a*t4=reasoned appraisal | coherent quality rationale |

### Result

| | **necessity** | **sufficiency** | **completeness** | **consistency** |
|---|---|---|---|---|
| **normative** | binding input rationale | accepted proof threshold | covered obligation record | stable conformance message |
| **operative** | required execution input | workable evidence basis | finished activity account | repeatable process signal |
| **evaluative** | critical appraisal basis | balanced judgment support | full review perspective | coherent quality rationale |

## Matrix F - Requirements (3x4)

### Construction: Dot product C by B

Intermediate collections use `L_F(row,col) = collection of C(row,k) * B(k,col)` followed by the three-step interpretation operator.

| Cell | Intermediate collection L | Step 1 axis anchor | Step 2 projections | Step 3 centroid |
|---|---|---|---|---|
| F[normative,necessity] | t1=rule input fact; t2=proof signal; t3=obligation understanding; t4=conformance discernment | normative * necessity = binding requirement | a*t1=required schema premise; a*t2=accepted proof cue; a*t3=covered duty basis; a*t4=stable control filter | controlling schema prerequisite |
| F[normative,sufficiency] | t1=input evidence; t2=proof context; t3=obligation expertise; t4=conformance judgment | normative * sufficiency = acceptance basis | a*t1=evidenced schema need; a*t2=threshold context; a*t3=obligation expertise; a*t4=alignment judgment | reviewable evidence threshold |
| F[normative,completeness] | t1=input record; t2=proof account; t3=obligation mastery; t4=conformance insight | normative * completeness = coverage demand | a*t1=recorded premise; a*t2=proof coverage; a*t3=obligation span; a*t4=alignment insight | full obligation coverage |
| F[normative,consistency] | t1=input measurement; t2=proof message; t3=obligation understanding; t4=conformance reasoning | normative * consistency = stable alignment | a*t1=measured premise; a*t2=coherent proof; a*t3=stable duty; a*t4=reasoned control | stable rule alignment |
| F[operative,necessity] | t1=execution fact; t2=evidence signal; t3=activity understanding; t4=process discernment | operative * necessity = execution gate | a*t1=work input fact; a*t2=evidence cue; a*t3=activity basis; a*t4=process filter | executable input gate |
| F[operative,sufficiency] | t1=execution evidence; t2=evidence context; t3=activity expertise; t4=process judgment | operative * sufficiency = action basis | a*t1=work proof; a*t2=situated evidence; a*t3=activity skill; a*t4=process judgment | validated action basis |
| F[operative,completeness] | t1=execution record; t2=evidence account; t3=activity mastery; t4=process insight | operative * completeness = workflow span | a*t1=work record; a*t2=evidence account; a*t3=activity scope; a*t4=process insight | whole workflow coverage |
| F[operative,consistency] | t1=execution measurement; t2=evidence message; t3=activity understanding; t4=process reasoning | operative * consistency = repeatable application | a*t1=measured work; a*t2=coherent evidence; a*t3=stable activity; a*t4=reasoned process | repeatable application behavior |
| F[evaluative,necessity] | t1=appraisal fact; t2=judgment signal; t3=review understanding; t4=quality discernment | evaluative * necessity = decision checkpoint | a*t1=appraisal input; a*t2=judgment cue; a*t3=review basis; a*t4=quality filter | decision basis checkpoint |
| F[evaluative,sufficiency] | t1=appraisal evidence; t2=judgment context; t3=review expertise; t4=quality judgment | evaluative * sufficiency = review support | a*t1=appraisal proof; a*t2=situated judgment; a*t3=review skill; a*t4=quality judgment | balanced review support |
| F[evaluative,completeness] | t1=appraisal record; t2=judgment account; t3=review mastery; t4=quality insight | evaluative * completeness = appraisal span | a*t1=appraisal record; a*t2=judgment account; a*t3=review scope; a*t4=quality insight | end-to-end appraisal record |
| F[evaluative,consistency] | t1=appraisal measurement; t2=judgment message; t3=review understanding; t4=quality reasoning | evaluative * consistency = acceptance rationale | a*t1=measured appraisal; a*t2=coherent judgment; a*t3=stable review; a*t4=reasoned quality | coherent acceptance rationale |

### Result

| | **necessity** | **sufficiency** | **completeness** | **consistency** |
|---|---|---|---|---|
| **normative** | controlling schema prerequisite | reviewable evidence threshold | full obligation coverage | stable rule alignment |
| **operative** | executable input gate | validated action basis | whole workflow coverage | repeatable application behavior |
| **evaluative** | decision basis checkpoint | balanced review support | end-to-end appraisal record | coherent acceptance rationale |

## Matrix D - Objectives (3x4)

### Construction: Addition A with resolution-transformed F

Intermediate collections use `L_D(row,col) = A(row,col) collected with resolution-transformed F(row,col)` followed by the three-step interpretation operator.

| Cell | Intermediate collection L | Step 1 axis anchor | Step 2 projections | Step 3 centroid |
|---|---|---|---|---|
| D[normative,guiding] | t1=prescriptive direction; t2=resolved schema prerequisite | normative * guiding = binding direction | a*t1=declared control; a*t2=closed prerequisite | bounded directive closure |
| D[normative,applying] | t1=mandatory practice; t2=resolved evidence threshold | normative * applying = enforced practice | a*t1=required enactment; a*t2=accepted proof gate | controlled practice gate |
| D[normative,judging] | t1=compliance determination; t2=resolved obligation coverage | normative * judging = binding decision | a*t1=conformance decision; a*t2=covered obligation test | verified acceptance decision |
| D[normative,reviewing] | t1=regulatory audit; t2=resolved rule alignment | normative * reviewing = audit control | a*t1=authority review; a*t2=stable alignment evidence | traceable audit posture |
| D[operative,guiding] | t1=procedural direction; t2=resolved input gate | operative * guiding = work direction | a*t1=procedure route; a*t2=execution prerequisite | repeatable work direction |
| D[operative,applying] | t1=practical execution; t2=resolved action basis | operative * applying = work enactment | a*t1=performed route; a*t2=validated action proof | validated execution route |
| D[operative,judging] | t1=performance assessment; t2=resolved workflow coverage | operative * judging = work check | a*t1=measured outcome; a*t2=covered workflow test | measured outcome check |
| D[operative,reviewing] | t1=process audit; t2=resolved application behavior | operative * reviewing = process inspection | a*t1=process evidence; a*t2=repeatable behavior review | process evidence review |
| D[evaluative,guiding] | t1=value orientation; t2=resolved decision checkpoint | evaluative * guiding = value direction | a*t1=principled orientation; a*t2=decision checkpoint filter | principled review orientation |
| D[evaluative,applying] | t1=merit application; t2=resolved review support | evaluative * applying = merit use | a*t1=reasoned merit; a*t2=balanced support | reasoned judgment use |
| D[evaluative,judging] | t1=worth determination; t2=resolved appraisal record | evaluative * judging = quality decision | a*t1=worth decision; a*t2=appraisal evidence | quality decision basis |
| D[evaluative,reviewing] | t1=quality appraisal; t2=resolved acceptance rationale | evaluative * reviewing = appraisal inspection | a*t1=quality evidence; a*t2=acceptance reasoning | evidence appraisal posture |

### Result

| | **guiding** | **applying** | **judging** | **reviewing** |
|---|---|---|---|---|
| **normative** | bounded directive closure | controlled practice gate | verified acceptance decision | traceable audit posture |
| **operative** | repeatable work direction | validated execution route | measured outcome check | process evidence review |
| **evaluative** | principled review orientation | reasoned judgment use | quality decision basis | evidence appraisal posture |

## Matrix K - Transpose of D (4x3)

### Construction: K(row,col) = D(col,row)

### Result

| | **normative** | **operative** | **evaluative** |
|---|---|---|---|
| **guiding** | bounded directive closure | repeatable work direction | principled review orientation |
| **applying** | controlled practice gate | validated execution route | reasoned judgment use |
| **judging** | verified acceptance decision | measured outcome check | quality decision basis |
| **reviewing** | traceable audit posture | process evidence review | evidence appraisal posture |

## Matrix G - Truncation of B (3x4)

### Construction: remove `wisdom` row from B

### Result

| | **necessity** | **sufficiency** | **completeness** | **consistency** |
|---|---|---|---|---|
| **data** | essential fact | adequate evidence | comprehensive record | reliable measurement |
| **information** | essential signal | adequate context | comprehensive account | coherent message |
| **knowledge** | fundamental understanding | competent expertise | thorough mastery | coherent understanding |

## Matrix X - Verification (4x4)

### Construction: Dot product K by G

Intermediate collections use `L_X(row,col) = collection of K(row,k) * G(k,col)` followed by the three-step interpretation operator.

| Cell | Intermediate collection L | Step 1 axis anchor | Step 2 projections | Step 3 centroid |
|---|---|---|---|---|
| X[guiding,necessity] | t1=directive fact; t2=work signal; t3=review understanding | guiding * necessity = needed direction | a*t1=input instruction; a*t2=work cue; a*t3=review basis | directive input basis |
| X[guiding,sufficiency] | t1=directive evidence; t2=work context; t3=review expertise | guiding * sufficiency = support direction | a*t1=evidenced instruction; a*t2=situated work; a*t3=qualified review | instruction evidence threshold |
| X[guiding,completeness] | t1=directive record; t2=work account; t3=review mastery | guiding * completeness = coverage direction | a*t1=recorded instruction; a*t2=work account; a*t3=review span | coverage signal map |
| X[guiding,consistency] | t1=directive measurement; t2=work message; t3=review understanding | guiding * consistency = stable direction | a*t1=measured instruction; a*t2=coherent work; a*t3=stable review | stable direction message |
| X[applying,necessity] | t1=practice fact; t2=execution signal; t3=judgment understanding | applying * necessity = action prerequisite | a*t1=practice input; a*t2=execution cue; a*t3=judgment basis | action prerequisite check |
| X[applying,sufficiency] | t1=practice evidence; t2=execution context; t3=judgment expertise | applying * sufficiency = action proof | a*t1=evidenced practice; a*t2=situated execution; a*t3=qualified judgment | execution proof basis |
| X[applying,completeness] | t1=practice record; t2=execution account; t3=judgment mastery | applying * completeness = action coverage | a*t1=practice record; a*t2=execution account; a*t3=judgment span | workflow coverage record |
| X[applying,consistency] | t1=practice measurement; t2=execution message; t3=judgment understanding | applying * consistency = repeatable action | a*t1=measured practice; a*t2=coherent execution; a*t3=stable judgment | repeatable enactment signal |
| X[judging,necessity] | t1=acceptance fact; t2=outcome signal; t3=quality understanding | judging * necessity = decision input | a*t1=acceptance premise; a*t2=outcome cue; a*t3=quality basis | decision input test |
| X[judging,sufficiency] | t1=acceptance evidence; t2=outcome context; t3=quality expertise | judging * sufficiency = determination proof | a*t1=acceptance evidence; a*t2=situated outcome; a*t3=quality skill | determination evidence basis |
| X[judging,completeness] | t1=acceptance record; t2=outcome account; t3=quality mastery | judging * completeness = assessment span | a*t1=acceptance record; a*t2=outcome account; a*t3=quality scope | assessment coverage record |
| X[judging,consistency] | t1=acceptance measurement; t2=outcome message; t3=quality understanding | judging * consistency = decision coherence | a*t1=measured acceptance; a*t2=coherent outcome; a*t3=stable quality | coherent decision signal |
| X[reviewing,necessity] | t1=audit fact; t2=process signal; t3=appraisal understanding | reviewing * necessity = inspection input | a*t1=audit premise; a*t2=process cue; a*t3=appraisal basis | audit prerequisite basis |
| X[reviewing,sufficiency] | t1=audit evidence; t2=process context; t3=appraisal expertise | reviewing * sufficiency = inspection proof | a*t1=audit evidence; a*t2=situated process; a*t3=appraisal skill | inspection evidence threshold |
| X[reviewing,completeness] | t1=audit record; t2=process account; t3=appraisal mastery | reviewing * completeness = inspection span | a*t1=audit record; a*t2=process account; a*t3=appraisal scope | inspection coverage record |
| X[reviewing,consistency] | t1=audit measurement; t2=process message; t3=appraisal understanding | reviewing * consistency = audit coherence | a*t1=measured audit; a*t2=coherent process; a*t3=stable appraisal | stable audit message |

### Result

| | **necessity** | **sufficiency** | **completeness** | **consistency** |
|---|---|---|---|---|
| **guiding** | directive input basis | instruction evidence threshold | coverage signal map | stable direction message |
| **applying** | action prerequisite check | execution proof basis | workflow coverage record | repeatable enactment signal |
| **judging** | decision input test | determination evidence basis | assessment coverage record | coherent decision signal |
| **reviewing** | audit prerequisite basis | inspection evidence threshold | inspection coverage record | stable audit message |

## Matrix T - Transpose of B (4x4)

### Construction: T(row,col) = B(col,row)

### Result

| | **data** | **information** | **knowledge** | **wisdom** |
|---|---|---|---|---|
| **necessity** | essential fact | essential signal | fundamental understanding | essential discernment |
| **sufficiency** | adequate evidence | adequate context | competent expertise | adequate judgment |
| **completeness** | comprehensive record | comprehensive account | thorough mastery | holistic insight |
| **consistency** | reliable measurement | coherent message | coherent understanding | principled reasoning |

## Matrix E - Evaluation (4x4)

### Construction: Dot product X by T

Intermediate collections use `L_E(row,col) = collection of X(row,k) * T(k,col)` followed by the three-step interpretation operator.

| Cell | Intermediate collection L | Step 1 axis anchor | Step 2 projections | Step 3 centroid |
|---|---|---|---|---|
| E[guiding,data] | t1=input fact; t2=evidence trace; t3=coverage record; t4=direction measurement | guiding * data = directive evidence | a*t1=instruction fact; a*t2=trace proof; a*t3=coverage record; a*t4=measured direction | directive evidence trace |
| E[guiding,information] | t1=input signal; t2=evidence context; t3=coverage account; t4=direction message | guiding * information = instruction context | a*t1=instruction signal; a*t2=context proof; a*t3=coverage account; a*t4=coherent direction | instruction context frame |
| E[guiding,knowledge] | t1=input understanding; t2=evidence expertise; t3=coverage mastery; t4=direction understanding | guiding * knowledge = practice insight | a*t1=instruction understanding; a*t2=qualified evidence; a*t3=coverage mastery; a*t4=stable direction | practice insight basis |
| E[guiding,wisdom] | t1=input discernment; t2=evidence judgment; t3=coverage insight; t4=direction reasoning | guiding * wisdom = principled direction | a*t1=instruction discernment; a*t2=evidence judgment; a*t3=coverage insight; a*t4=reasoned direction | principled direction sense |
| E[applying,data] | t1=action fact; t2=execution evidence; t3=workflow record; t4=enactment measurement | applying * data = execution evidence | a*t1=action fact; a*t2=execution proof; a*t3=workflow record; a*t4=measured enactment | execution fact trace |
| E[applying,information] | t1=action signal; t2=execution context; t3=workflow account; t4=enactment message | applying * information = work context | a*t1=action signal; a*t2=situated execution; a*t3=workflow account; a*t4=coherent enactment | work context frame |
| E[applying,knowledge] | t1=action understanding; t2=execution expertise; t3=workflow mastery; t4=enactment understanding | applying * knowledge = practical capability | a*t1=action understanding; a*t2=execution skill; a*t3=workflow mastery; a*t4=stable enactment | practical capability basis |
| E[applying,wisdom] | t1=action discernment; t2=execution judgment; t3=workflow insight; t4=enactment reasoning | applying * wisdom = situated judgment | a*t1=action discernment; a*t2=execution judgment; a*t3=workflow insight; a*t4=reasoned enactment | situated action judgment |
| E[judging,data] | t1=decision fact; t2=determination evidence; t3=assessment record; t4=decision measurement | judging * data = decision evidence | a*t1=decision fact; a*t2=determination proof; a*t3=assessment record; a*t4=measured decision | decision evidence trace |
| E[judging,information] | t1=decision signal; t2=determination context; t3=assessment account; t4=decision message | judging * information = assessment context | a*t1=decision signal; a*t2=situated determination; a*t3=assessment account; a*t4=coherent decision | assessment context frame |
| E[judging,knowledge] | t1=decision understanding; t2=determination expertise; t3=assessment mastery; t4=decision understanding | judging * knowledge = determination expertise | a*t1=decision understanding; a*t2=determination skill; a*t3=assessment mastery; a*t4=stable decision | determination expertise basis |
| E[judging,wisdom] | t1=decision discernment; t2=determination judgment; t3=assessment insight; t4=decision reasoning | judging * wisdom = balanced decision | a*t1=decision discernment; a*t2=determination judgment; a*t3=assessment insight; a*t4=reasoned decision | balanced decision judgment |
| E[reviewing,data] | t1=audit fact; t2=inspection evidence; t3=coverage record; t4=audit measurement | reviewing * data = audit evidence | a*t1=audit fact; a*t2=inspection proof; a*t3=coverage record; a*t4=measured audit | audit evidence trace |
| E[reviewing,information] | t1=audit signal; t2=inspection context; t3=coverage account; t4=audit message | reviewing * information = inspection context | a*t1=audit signal; a*t2=situated inspection; a*t3=coverage account; a*t4=coherent audit | inspection context frame |
| E[reviewing,knowledge] | t1=audit understanding; t2=inspection expertise; t3=coverage mastery; t4=audit understanding | reviewing * knowledge = appraisal capability | a*t1=audit understanding; a*t2=inspection skill; a*t3=coverage mastery; a*t4=stable audit | appraisal capability basis |
| E[reviewing,wisdom] | t1=audit discernment; t2=inspection judgment; t3=coverage insight; t4=audit reasoning | reviewing * wisdom = principled audit | a*t1=audit discernment; a*t2=inspection judgment; a*t3=coverage insight; a*t4=reasoned audit | principled audit judgment |

### Result

| | **data** | **information** | **knowledge** | **wisdom** |
|---|---|---|---|---|
| **guiding** | directive evidence trace | instruction context frame | practice insight basis | principled direction sense |
| **applying** | execution fact trace | work context frame | practical capability basis | situated action judgment |
| **judging** | decision evidence trace | assessment context frame | determination expertise basis | balanced decision judgment |
| **reviewing** | audit evidence trace | inspection context frame | appraisal capability basis | principled audit judgment |

---

## Matrix Summary

### Matrix C - Formulation

| | **necessity** | **sufficiency** | **completeness** | **consistency** |
|---|---|---|---|---|
| **normative** | binding input rationale | accepted proof threshold | covered obligation record | stable conformance message |
| **operative** | required execution input | workable evidence basis | finished activity account | repeatable process signal |
| **evaluative** | critical appraisal basis | balanced judgment support | full review perspective | coherent quality rationale |

### Matrix F - Requirements

| | **necessity** | **sufficiency** | **completeness** | **consistency** |
|---|---|---|---|---|
| **normative** | controlling schema prerequisite | reviewable evidence threshold | full obligation coverage | stable rule alignment |
| **operative** | executable input gate | validated action basis | whole workflow coverage | repeatable application behavior |
| **evaluative** | decision basis checkpoint | balanced review support | end-to-end appraisal record | coherent acceptance rationale |

### Matrix D - Objectives

| | **guiding** | **applying** | **judging** | **reviewing** |
|---|---|---|---|---|
| **normative** | bounded directive closure | controlled practice gate | verified acceptance decision | traceable audit posture |
| **operative** | repeatable work direction | validated execution route | measured outcome check | process evidence review |
| **evaluative** | principled review orientation | reasoned judgment use | quality decision basis | evidence appraisal posture |

### Matrix K - Transpose of D

| | **normative** | **operative** | **evaluative** |
|---|---|---|---|
| **guiding** | bounded directive closure | repeatable work direction | principled review orientation |
| **applying** | controlled practice gate | validated execution route | reasoned judgment use |
| **judging** | verified acceptance decision | measured outcome check | quality decision basis |
| **reviewing** | traceable audit posture | process evidence review | evidence appraisal posture |

### Matrix G - Truncation of B

| | **necessity** | **sufficiency** | **completeness** | **consistency** |
|---|---|---|---|---|
| **data** | essential fact | adequate evidence | comprehensive record | reliable measurement |
| **information** | essential signal | adequate context | comprehensive account | coherent message |
| **knowledge** | fundamental understanding | competent expertise | thorough mastery | coherent understanding |

### Matrix X - Verification

| | **necessity** | **sufficiency** | **completeness** | **consistency** |
|---|---|---|---|---|
| **guiding** | directive input basis | instruction evidence threshold | coverage signal map | stable direction message |
| **applying** | action prerequisite check | execution proof basis | workflow coverage record | repeatable enactment signal |
| **judging** | decision input test | determination evidence basis | assessment coverage record | coherent decision signal |
| **reviewing** | audit prerequisite basis | inspection evidence threshold | inspection coverage record | stable audit message |

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
| **guiding** | directive evidence trace | instruction context frame | practice insight basis | principled direction sense |
| **applying** | execution fact trace | work context frame | practical capability basis | situated action judgment |
| **judging** | decision evidence trace | assessment context frame | determination expertise basis | balanced decision judgment |
| **reviewing** | audit evidence trace | inspection context frame | appraisal capability basis | principled audit judgment |
