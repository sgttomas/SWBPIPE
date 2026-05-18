# Semantic Lens: DEL-07-03 Material, component, and rule-pack editors

**Generated:** 2026-04-30
**DECOMP_VARIANT:** SOFTWARE
**Perspective:** This deliverable frames the editor workflow as a bounded GUI setup surface for user-supplied/private engineering data, rule-pack references, provenance, and missing-data visibility. The lens partitions what the documents must ask about editor structure, validation behavior, workflow evidence, and review boundaries without specifying project values, code clauses, component libraries, or implementation packages.
**Framework:** Chirality Semantic Algebra
**Audit:** PASS - final cells in matrices C, F, D, X, and E were checked for algebra leaks, operator leaks, empty values, and long uninterpreted expansions.

**Inputs Read:**
- _CONTEXT.md - `# Context: DEL-07-03`
- _STATUS.md - `# Status: DEL-07-03 Material, component, and rule-pack editors`
- Datasheet.md - `# Datasheet: DEL-07-03 Material, component, and rule-pack editors`
- Specification.md - `# Specification: DEL-07-03 Material, component, and rule-pack editors`
- Guidance.md - `# Guidance: DEL-07-03 Material, component, and rule-pack editors`
- Procedure.md - `# Procedure: DEL-07-03 Material, component, and rule-pack editors`
- _REFERENCES.md - `# References: DEL-07-03 Material, component, and rule-pack editors`

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

Formula: `L_C(i,j) = A(i,1)*B(1,j) + A(i,2)*B(2,j) + A(i,3)*B(3,j) + A(i,4)*B(4,j)`, then `C(i,j)=I(row_i,col_j,L_C(i,j))`.

### Interpretation Ledger

| Cell | L_C contributors | Step 1 axis anchor | Step 2 projections | Step 3 centroid result |
|---|---|---|---|---|
| C[normative,necessity] | directive fact; required signal; conformance understanding; audit discernment | normative * necessity = binding need | a*t1=input threshold; a*t2=required cue; a*t3=rule basis; a*t4=audit trigger | binding input threshold |
| C[normative,sufficiency] | directive evidence; required context; conformance expertise; audit judgment | normative * sufficiency = warranted adequacy | a*t1=proof basis; a*t2=context fit; a*t3=expert warrant; a*t4=decision support | warranted acceptance basis |
| C[normative,completeness] | directive record; required account; conformance mastery; audit insight | normative * completeness = closure demand | a*t1=record coverage; a*t2=account span; a*t3=mastery closure; a*t4=insight boundary | full coverage obligation |
| C[normative,consistency] | directive measurement; required message; conformance understanding; audit reasoning | normative * consistency = aligned rule | a*t1=measurement order; a*t2=message order; a*t3=understood order; a*t4=reasoned order | coherent mandate |
| C[operative,necessity] | procedural fact; execution signal; assessment understanding; process discernment | operative * necessity = task need | a*t1=work input; a*t2=action cue; a*t3=assessment basis; a*t4=process trigger | essential task input |
| C[operative,sufficiency] | procedural evidence; execution context; assessment expertise; process judgment | operative * sufficiency = usable adequacy | a*t1=work proof; a*t2=task context; a*t3=capability fit; a*t4=process warrant | usable workflow context |
| C[operative,completeness] | procedural record; execution account; assessment mastery; process insight | operative * completeness = action closure | a*t1=step record; a*t2=workflow account; a*t3=capability closure; a*t4=process span | full action record |
| C[operative,consistency] | procedural measurement; execution message; assessment understanding; process reasoning | operative * consistency = stable work | a*t1=measured stability; a*t2=message stability; a*t3=assessment order; a*t4=process logic | stable workflow signal |
| C[evaluative,necessity] | value fact; merit signal; worth understanding; quality discernment | evaluative * necessity = review need | a*t1=review fact; a*t2=merit cue; a*t3=worth basis; a*t4=quality trigger | critical review basis |
| C[evaluative,sufficiency] | value evidence; merit context; worth expertise; quality judgment | evaluative * sufficiency = appraisal adequacy | a*t1=value proof; a*t2=merit context; a*t3=worth expertise; a*t4=quality warrant | sound appraisal context |
| C[evaluative,completeness] | value record; merit account; worth mastery; quality insight | evaluative * completeness = assessment closure | a*t1=value record; a*t2=merit account; a*t3=worth closure; a*t4=quality span | full assessment record |
| C[evaluative,consistency] | value measurement; merit message; worth understanding; quality reasoning | evaluative * consistency = aligned appraisal | a*t1=value order; a*t2=merit order; a*t3=worth coherence; a*t4=quality logic | aligned review rationale |

### Result

| | **necessity** | **sufficiency** | **completeness** | **consistency** |
|---|---|---|---|---|
| **normative** | binding input threshold | warranted acceptance basis | full coverage obligation | coherent mandate |
| **operative** | essential task input | usable workflow context | full action record | stable workflow signal |
| **evaluative** | critical review basis | sound appraisal context | full assessment record | aligned review rationale |

## Matrix F - Requirements (3x4)

### Construction: Dot product C dot B

Formula: `L_F(i,j) = C(i,1)*B(1,j) + C(i,2)*B(2,j) + C(i,3)*B(3,j) + C(i,4)*B(4,j)`, then `F(i,j)=I(row_i,col_j,L_F(i,j))`.

### Interpretation Ledger

| Cell | L_F contributors | Step 1 axis anchor | Step 2 projections | Step 3 centroid result |
|---|---|---|---|---|
| F[normative,necessity] | threshold fact; acceptance signal; coverage understanding; mandate discernment | normative * necessity = binding need | a*t1=entry gate; a*t2=approval cue; a*t3=coverage premise; a*t4=mandate trigger | enforceable input gate |
| F[normative,sufficiency] | threshold evidence; acceptance context; coverage expertise; mandate judgment | normative * sufficiency = warranted adequacy | a*t1=proof adequacy; a*t2=acceptance fit; a*t3=coverage proof; a*t4=rule warrant | sufficient acceptance proof |
| F[normative,completeness] | threshold record; acceptance account; coverage mastery; mandate insight | normative * completeness = closure demand | a*t1=gate record; a*t2=acceptance account; a*t3=coverage closure; a*t4=rule scope | full requirement set |
| F[normative,consistency] | threshold measurement; acceptance message; coverage understanding; mandate reasoning | normative * consistency = aligned rule | a*t1=gate alignment; a*t2=acceptance wording; a*t3=coverage meaning; a*t4=rule logic | consistent rule language |
| F[operative,necessity] | task-input fact; workflow signal; action understanding; workflow discernment | operative * necessity = task need | a*t1=work precondition; a*t2=action cue; a*t3=operation basis; a*t4=workflow trigger | executable task precondition |
| F[operative,sufficiency] | task-input evidence; workflow context; action expertise; workflow judgment | operative * sufficiency = usable adequacy | a*t1=input proof; a*t2=implementation context; a*t3=capability proof; a*t4=workflow warrant | usable implementation context |
| F[operative,completeness] | task-input record; workflow account; action mastery; workflow insight | operative * completeness = action closure | a*t1=input record; a*t2=workflow account; a*t3=operation closure; a*t4=trace span | full workflow trace |
| F[operative,consistency] | task-input measurement; workflow message; action understanding; workflow reasoning | operative * consistency = stable work | a*t1=input stability; a*t2=operation wording; a*t3=workflow meaning; a*t4=service logic | stable operation contract |
| F[evaluative,necessity] | review-basis fact; appraisal signal; assessment understanding; rationale discernment | evaluative * necessity = review need | a*t1=evidence need; a*t2=appraisal cue; a*t3=assessment premise; a*t4=quality trigger | reviewable evidence need |
| F[evaluative,sufficiency] | review-basis evidence; appraisal context; assessment expertise; rationale judgment | evaluative * sufficiency = appraisal adequacy | a*t1=evidence proof; a*t2=appraisal fit; a*t3=quality capability; a*t4=balanced warrant | balanced rationale basis |
| F[evaluative,completeness] | review-basis record; appraisal account; assessment mastery; rationale insight | evaluative * completeness = assessment closure | a*t1=evidence record; a*t2=appraisal account; a*t3=quality closure; a*t4=rationale span | full quality record |
| F[evaluative,consistency] | review-basis measurement; appraisal message; assessment understanding; rationale reasoning | evaluative * consistency = aligned appraisal | a*t1=evidence order; a*t2=appraisal wording; a*t3=quality meaning; a*t4=review logic | coherent review standard |

### Result

| | **necessity** | **sufficiency** | **completeness** | **consistency** |
|---|---|---|---|---|
| **normative** | enforceable input gate | sufficient acceptance proof | full requirement set | consistent rule language |
| **operative** | executable task precondition | usable implementation context | full workflow trace | stable operation contract |
| **evaluative** | reviewable evidence need | balanced rationale basis | full quality record | coherent review standard |

## Matrix D - Objectives (3x4)

### Construction: Addition A plus resolution-transformed F

Formula: `L_D(i,j) = A(i,j) + ("resolution" * F(i,j))`, then `D(i,j)=I(row_i,col_j,L_D(i,j))`.

### Interpretation Ledger

| Cell | L_D contributors | Step 1 axis anchor | Step 2 projections | Step 3 centroid result |
|---|---|---|---|---|
| D[normative,guiding] | prescriptive direction; resolved enforceable gate | normative * guiding = directed rule | a*t1=rule direction; a*t2=closed gate | directed obligation path |
| D[normative,applying] | mandatory practice; resolved acceptance proof | normative * applying = required practice | a*t1=required practice; a*t2=closed proof | practice closure target |
| D[normative,judging] | compliance determination; resolved requirement set | normative * judging = rule decision | a*t1=decision rule; a*t2=closed set | decision boundary target |
| D[normative,reviewing] | regulatory audit; resolved rule language | normative * reviewing = audit rule | a*t1=audit control; a*t2=closed wording | audit-ready control objective |
| D[operative,guiding] | procedural direction; resolved task precondition | operative * guiding = workflow direction | a*t1=work route; a*t2=closed entry | workflow enablement path |
| D[operative,applying] | practical execution; resolved implementation context | operative * applying = active work | a*t1=work action; a*t2=closed context | execution closure target |
| D[operative,judging] | performance assessment; resolved workflow trace | operative * judging = task decision | a*t1=performance decision; a*t2=closed trace | performance decision target |
| D[operative,reviewing] | process audit; resolved operation contract | operative * reviewing = process assurance | a*t1=process audit; a*t2=closed contract | process assurance objective |
| D[evaluative,guiding] | value orientation; resolved evidence need | evaluative * guiding = value direction | a*t1=value route; a*t2=closed evidence | value-aligned direction |
| D[evaluative,applying] | merit application; resolved rationale basis | evaluative * applying = merit use | a*t1=merit action; a*t2=closed rationale | merit closure target |
| D[evaluative,judging] | worth determination; resolved quality record | evaluative * judging = worth decision | a*t1=worth decision; a*t2=closed record | worth decision objective |
| D[evaluative,reviewing] | quality appraisal; resolved review standard | evaluative * reviewing = quality assurance | a*t1=quality appraisal; a*t2=closed standard | quality assurance objective |

### Result

| | **guiding** | **applying** | **judging** | **reviewing** |
|---|---|---|---|---|
| **normative** | directed obligation path | practice closure target | decision boundary target | audit-ready control objective |
| **operative** | workflow enablement path | execution closure target | performance decision target | process assurance objective |
| **evaluative** | value-aligned direction | merit closure target | worth decision objective | quality assurance objective |

## Matrix K - Transpose of D (4x3)

### Construction: K(i,j) = D(j,i)

### Result

| | **normative** | **operative** | **evaluative** |
|---|---|---|---|
| **guiding** | directed obligation path | workflow enablement path | value-aligned direction |
| **applying** | practice closure target | execution closure target | merit closure target |
| **judging** | decision boundary target | performance decision target | worth decision objective |
| **reviewing** | audit-ready control objective | process assurance objective | quality assurance objective |

## Matrix G - Truncation of B (3x4)

### Construction: remove `wisdom` row from B

### Result

| | **necessity** | **sufficiency** | **completeness** | **consistency** |
|---|---|---|---|---|
| **data** | essential fact | adequate evidence | comprehensive record | reliable measurement |
| **information** | essential signal | adequate context | comprehensive account | coherent message |
| **knowledge** | fundamental understanding | competent expertise | thorough mastery | coherent understanding |

## Matrix X - Verification (4x4)

### Construction: Dot product K dot G

Formula: `L_X(i,j) = K(i,1)*G(1,j) + K(i,2)*G(2,j) + K(i,3)*G(3,j)`, then `X(i,j)=I(row_i,col_j,L_X(i,j))`.

### Interpretation Ledger

| Cell | L_X contributors | Step 1 axis anchor | Step 2 projections | Step 3 centroid result |
|---|---|---|---|---|
| X[guiding,necessity] | obligation fact; workflow signal; value understanding | guiding * necessity = directional need | a*t1=source fact; a*t2=route cue; a*t3=value premise | directional input test |
| X[guiding,sufficiency] | obligation evidence; workflow context; value expertise | guiding * sufficiency = adequate direction | a*t1=source proof; a*t2=context fit; a*t3=value capability | basis adequacy check |
| X[guiding,completeness] | obligation record; workflow account; value mastery | guiding * completeness = direction closure | a*t1=record span; a*t2=route account; a*t3=value closure | coverage review check |
| X[guiding,consistency] | obligation measurement; workflow message; value understanding | guiding * consistency = directed coherence | a*t1=measured fit; a*t2=message fit; a*t3=value order | coherence check path |
| X[applying,necessity] | practice fact; execution signal; merit understanding | applying * necessity = action need | a*t1=action fact; a*t2=execution cue; a*t3=merit basis | execution input test |
| X[applying,sufficiency] | practice evidence; execution context; merit expertise | applying * sufficiency = action adequacy | a*t1=practice proof; a*t2=implementation fit; a*t3=merit capability | implementation adequacy check |
| X[applying,completeness] | practice record; execution account; merit mastery | applying * completeness = action closure | a*t1=practice record; a*t2=execution account; a*t3=merit closure | workflow coverage check |
| X[applying,consistency] | practice measurement; execution message; merit understanding | applying * consistency = action coherence | a*t1=practice order; a*t2=execution message; a*t3=merit order | operation coherence check |
| X[judging,necessity] | boundary fact; performance signal; worth understanding | judging * necessity = decision need | a*t1=boundary fact; a*t2=performance cue; a*t3=worth basis | decision evidence test |
| X[judging,sufficiency] | boundary evidence; performance context; worth expertise | judging * sufficiency = decision adequacy | a*t1=boundary proof; a*t2=performance context; a*t3=worth capability | assessment adequacy check |
| X[judging,completeness] | boundary record; performance account; worth mastery | judging * completeness = decision closure | a*t1=boundary record; a*t2=performance account; a*t3=worth closure | finding coverage check |
| X[judging,consistency] | boundary measurement; performance message; worth understanding | judging * consistency = decision coherence | a*t1=boundary order; a*t2=performance wording; a*t3=worth order | decision coherence check |
| X[reviewing,necessity] | audit fact; process signal; quality understanding | reviewing * necessity = audit need | a*t1=audit fact; a*t2=process cue; a*t3=quality basis | audit input test |
| X[reviewing,sufficiency] | audit evidence; process context; quality expertise | reviewing * sufficiency = audit adequacy | a*t1=audit proof; a*t2=process context; a*t3=quality capability | process evidence check |
| X[reviewing,completeness] | audit record; process account; quality mastery | reviewing * completeness = audit closure | a*t1=audit record; a*t2=process account; a*t3=quality closure | record coverage check |
| X[reviewing,consistency] | audit measurement; process message; quality understanding | reviewing * consistency = audit coherence | a*t1=audit order; a*t2=process wording; a*t3=quality order | assurance coherence check |

### Result

| | **necessity** | **sufficiency** | **completeness** | **consistency** |
|---|---|---|---|---|
| **guiding** | directional input test | basis adequacy check | coverage review check | coherence check path |
| **applying** | execution input test | implementation adequacy check | workflow coverage check | operation coherence check |
| **judging** | decision evidence test | assessment adequacy check | finding coverage check | decision coherence check |
| **reviewing** | audit input test | process evidence check | record coverage check | assurance coherence check |

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

Formula: `L_E(i,j) = X(i,1)*T(1,j) + X(i,2)*T(2,j) + X(i,3)*T(3,j) + X(i,4)*T(4,j)`, then `E(i,j)=I(row_i,col_j,L_E(i,j))`.

### Interpretation Ledger

| Cell | L_E contributors | Step 1 axis anchor | Step 2 projections | Step 3 centroid result |
|---|---|---|---|---|
| E[guiding,data] | input-test fact; adequacy evidence; coverage record; coherence measurement | guiding * data = directional fact | a*t1=fact check; a*t2=evidence check; a*t3=record check; a*t4=measurement check | directional fact assurance |
| E[guiding,information] | input-test signal; adequacy context; coverage account; coherence message | guiding * information = directional context | a*t1=signal check; a*t2=context check; a*t3=account check; a*t4=message check | context assurance path |
| E[guiding,knowledge] | input-test understanding; adequacy expertise; coverage mastery; coherence understanding | guiding * knowledge = directional expertise | a*t1=understanding check; a*t2=expertise check; a*t3=mastery check; a*t4=meaning check | expertise framing check |
| E[guiding,wisdom] | input-test discernment; adequacy judgment; coverage insight; coherence reasoning | guiding * wisdom = directional discernment | a*t1=discernment check; a*t2=judgment check; a*t3=insight check; a*t4=reasoning check | judgment framing check |
| E[applying,data] | execution-test fact; implementation evidence; workflow record; operation measurement | applying * data = action fact | a*t1=fact use; a*t2=evidence use; a*t3=record use; a*t4=measurement use | execution fact assurance |
| E[applying,information] | execution-test signal; implementation context; workflow account; operation message | applying * information = action context | a*t1=signal use; a*t2=context use; a*t3=account use; a*t4=message use | context execution check |
| E[applying,knowledge] | execution-test understanding; implementation expertise; workflow mastery; operation understanding | applying * knowledge = action expertise | a*t1=understanding use; a*t2=expertise use; a*t3=mastery use; a*t4=meaning use | expertise use check |
| E[applying,wisdom] | execution-test discernment; implementation judgment; workflow insight; operation reasoning | applying * wisdom = action discernment | a*t1=discernment use; a*t2=judgment use; a*t3=insight use; a*t4=reasoning use | discernment use check |
| E[judging,data] | decision-test fact; assessment evidence; finding record; decision measurement | judging * data = decision fact | a*t1=fact basis; a*t2=evidence basis; a*t3=record basis; a*t4=measurement basis | decision fact assurance |
| E[judging,information] | decision-test signal; assessment context; finding account; decision message | judging * information = decision context | a*t1=signal basis; a*t2=context basis; a*t3=account basis; a*t4=message basis | context decision check |
| E[judging,knowledge] | decision-test understanding; assessment expertise; finding mastery; decision understanding | judging * knowledge = decision expertise | a*t1=understanding basis; a*t2=expertise basis; a*t3=mastery basis; a*t4=meaning basis | expertise assessment check |
| E[judging,wisdom] | decision-test discernment; assessment judgment; finding insight; decision reasoning | judging * wisdom = decision discernment | a*t1=discernment basis; a*t2=judgment basis; a*t3=insight basis; a*t4=reasoning basis | discernment assessment check |
| E[reviewing,data] | audit-test fact; process evidence; record coverage; assurance measurement | reviewing * data = audit fact | a*t1=fact audit; a*t2=evidence audit; a*t3=record audit; a*t4=measurement audit | audit fact assurance |
| E[reviewing,information] | audit-test signal; process context; record account; assurance message | reviewing * information = audit context | a*t1=signal audit; a*t2=context audit; a*t3=account audit; a*t4=message audit | context audit check |
| E[reviewing,knowledge] | audit-test understanding; process expertise; record mastery; assurance understanding | reviewing * knowledge = audit expertise | a*t1=understanding audit; a*t2=expertise audit; a*t3=mastery audit; a*t4=meaning audit | expertise review check |
| E[reviewing,wisdom] | audit-test discernment; process judgment; record insight; assurance reasoning | reviewing * wisdom = audit discernment | a*t1=discernment audit; a*t2=judgment audit; a*t3=insight audit; a*t4=reasoning audit | discernment audit check |

### Result

| | **data** | **information** | **knowledge** | **wisdom** |
|---|---|---|---|---|
| **guiding** | directional fact assurance | context assurance path | expertise framing check | judgment framing check |
| **applying** | execution fact assurance | context execution check | expertise use check | discernment use check |
| **judging** | decision fact assurance | context decision check | expertise assessment check | discernment assessment check |
| **reviewing** | audit fact assurance | context audit check | expertise review check | discernment audit check |

---

## Matrix Summary

### C - Formulation

| | **necessity** | **sufficiency** | **completeness** | **consistency** |
|---|---|---|---|---|
| **normative** | binding input threshold | warranted acceptance basis | full coverage obligation | coherent mandate |
| **operative** | essential task input | usable workflow context | full action record | stable workflow signal |
| **evaluative** | critical review basis | sound appraisal context | full assessment record | aligned review rationale |

### F - Requirements

| | **necessity** | **sufficiency** | **completeness** | **consistency** |
|---|---|---|---|---|
| **normative** | enforceable input gate | sufficient acceptance proof | full requirement set | consistent rule language |
| **operative** | executable task precondition | usable implementation context | full workflow trace | stable operation contract |
| **evaluative** | reviewable evidence need | balanced rationale basis | full quality record | coherent review standard |

### D - Objectives

| | **guiding** | **applying** | **judging** | **reviewing** |
|---|---|---|---|---|
| **normative** | directed obligation path | practice closure target | decision boundary target | audit-ready control objective |
| **operative** | workflow enablement path | execution closure target | performance decision target | process assurance objective |
| **evaluative** | value-aligned direction | merit closure target | worth decision objective | quality assurance objective |

### K - Transpose of D

| | **normative** | **operative** | **evaluative** |
|---|---|---|---|
| **guiding** | directed obligation path | workflow enablement path | value-aligned direction |
| **applying** | practice closure target | execution closure target | merit closure target |
| **judging** | decision boundary target | performance decision target | worth decision objective |
| **reviewing** | audit-ready control objective | process assurance objective | quality assurance objective |

### G - Truncation of B

| | **necessity** | **sufficiency** | **completeness** | **consistency** |
|---|---|---|---|---|
| **data** | essential fact | adequate evidence | comprehensive record | reliable measurement |
| **information** | essential signal | adequate context | comprehensive account | coherent message |
| **knowledge** | fundamental understanding | competent expertise | thorough mastery | coherent understanding |

### X - Verification

| | **necessity** | **sufficiency** | **completeness** | **consistency** |
|---|---|---|---|---|
| **guiding** | directional input test | basis adequacy check | coverage review check | coherence check path |
| **applying** | execution input test | implementation adequacy check | workflow coverage check | operation coherence check |
| **judging** | decision evidence test | assessment adequacy check | finding coverage check | decision coherence check |
| **reviewing** | audit input test | process evidence check | record coverage check | assurance coherence check |

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
| **guiding** | directional fact assurance | context assurance path | expertise framing check | judgment framing check |
| **applying** | execution fact assurance | context execution check | expertise use check | discernment use check |
| **judging** | decision fact assurance | context decision check | expertise assessment check | discernment assessment check |
| **reviewing** | audit fact assurance | context audit check | expertise review check | discernment audit check |
