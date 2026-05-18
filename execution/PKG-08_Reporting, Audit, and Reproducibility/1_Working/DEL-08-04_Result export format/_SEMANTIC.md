# Semantic Lens: DEL-08-04 Result export format

**Generated:** 2026-04-30
**DECOMP_VARIANT:** SOFTWARE
**Perspective:** This deliverable frames the governed machine-readable export of OpenPipeStress result sets for review, regression comparison, and downstream tooling. It partitions the work into types of envelope contract, unit-aware payload, diagnostics, provenance, status separation, and no-bypass interoperability without specifying exporter code, external formats, project-specific values, or protected payloads.
**Framework:** Chirality Semantic Algebra
**Inputs Read:**
- _CONTEXT.md - deliverable identity, scope, objectives, and architecture basis injection.
- _STATUS.md - lifecycle state before setup.
- Datasheet.md - setup draft for descriptive export fields.
- Specification.md - setup draft for normative requirements.
- Guidance.md - setup draft for rationale and boundaries.
- Procedure.md - setup draft for workflow and records.
- _REFERENCES.md - governing reference list.

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

`L_C(i,j) = A(i,guiding)*B(data,j), A(i,applying)*B(information,j), A(i,judging)*B(knowledge,j), A(i,reviewing)*B(wisdom,j)`, then `C(i,j) = I(row_i, col_j, L_C(i,j))`.

| Cell | Intermediate collection | Step 1: axis anchor | Step 2: projected contributors | Step 3: centroid | Result |
|---|---|---|---|---|---|
| C[normative,necessity] | direction fact; practice signal; determination understanding; audit discernment | rule basis | p1=binding source; p2=required cue; p3=decidable rationale; p4=inspection trigger | required envelope | envelope mandate |
| C[normative,sufficiency] | direction evidence; practice context; determination expertise; audit judgment | rule adequacy | p1=proof threshold; p2=context fit; p3=competent basis; p4=review judgment | enough proof | evidence threshold |
| C[normative,completeness] | direction record; practice account; determination mastery; audit insight | rule coverage | p1=covered instruction; p2=complete account; p3=mastered scope; p4=holistic review | complete contract | contract coverage |
| C[normative,consistency] | direction measurement; practice message; determination understanding; audit reasoning | rule coherence | p1=stable measure; p2=aligned message; p3=coherent basis; p4=principled audit | stable schema | schema coherence |
| C[operative,necessity] | direction fact; execution signal; assessment understanding; audit discernment | work need | p1=input trigger; p2=execution cue; p3=usable basis; p4=inspection need | required export | export prerequisite |
| C[operative,sufficiency] | direction evidence; execution context; assessment expertise; audit judgment | work adequacy | p1=work proof; p2=context ready; p3=operable skill; p4=checkable judgment | ready payload | payload readiness |
| C[operative,completeness] | direction record; execution account; assessment mastery; audit insight | work coverage | p1=listed input; p2=workflow account; p3=complete handling; p4=insightful trace | full results | result inventory |
| C[operative,consistency] | direction measurement; execution message; assessment understanding; audit reasoning | work coherence | p1=repeatable measure; p2=stable message; p3=understood flow; p4=reasoned audit | stable flow | stable ordering |
| C[evaluative,necessity] | orientation fact; application signal; determination understanding; appraisal discernment | value need | p1=salient fact; p2=value cue; p3=decision basis; p4=appraisal trigger | review basis | boundary criterion |
| C[evaluative,sufficiency] | orientation evidence; application context; determination expertise; appraisal judgment | value adequacy | p1=supporting proof; p2=context fit; p3=competent review; p4=judged worth | defensible proof | review evidence |
| C[evaluative,completeness] | orientation record; application account; determination mastery; appraisal insight | value coverage | p1=covered value; p2=application account; p3=mastered evidence; p4=quality insight | full closure | closure inventory |
| C[evaluative,consistency] | orientation measurement; application message; determination understanding; appraisal reasoning | value coherence | p1=reliable measure; p2=aligned application; p3=coherent grasp; p4=reasoned appraisal | reliable comparison | comparison reliability |

### Result

| | **necessity** | **sufficiency** | **completeness** | **consistency** |
|---|---|---|---|---|
| **normative** | envelope mandate | evidence threshold | contract coverage | schema coherence |
| **operative** | export prerequisite | payload readiness | result inventory | stable ordering |
| **evaluative** | boundary criterion | review evidence | closure inventory | comparison reliability |

## Matrix F - Requirements (3x4)

### Construction: Dot product C dot B

`L_F(i,j) = C(i,necessity)*B(data,j), C(i,sufficiency)*B(information,j), C(i,completeness)*B(knowledge,j), C(i,consistency)*B(wisdom,j)`, then `F(i,j) = I(row_i, col_j, L_F(i,j))`.

| Cell | Intermediate collection | Step 1: axis anchor | Step 2: projected contributors | Step 3: centroid | Result |
|---|---|---|---|---|---|
| F[normative,necessity] | mandate fact; threshold signal; coverage understanding; coherence discernment | rule basis | p1=required identity; p2=minimum cue; p3=coverage basis; p4=stable discernment | binding envelope | envelope contract |
| F[normative,sufficiency] | mandate evidence; threshold context; coverage expertise; coherence judgment | rule adequacy | p1=provable source; p2=adequate context; p3=complete competence; p4=stable judgment | defensible provenance | provenance proof |
| F[normative,completeness] | mandate record; threshold account; coverage mastery; coherence insight | rule coverage | p1=required record; p2=accounted proof; p3=complete scope; p4=coherent insight | exhaustive schema | schema completeness |
| F[normative,consistency] | mandate measurement; threshold message; coverage understanding; coherence reasoning | rule coherence | p1=stable measure; p2=aligned threshold; p3=understood coverage; p4=reasoned stability | stable units | unit stability |
| F[operative,necessity] | prerequisite fact; readiness signal; inventory understanding; ordering discernment | work need | p1=capture trigger; p2=ready cue; p3=list basis; p4=ordered need | required workflow | export workflow |
| F[operative,sufficiency] | prerequisite evidence; readiness context; inventory expertise; ordering judgment | work adequacy | p1=workflow proof; p2=ready context; p3=inventory competence; p4=ordered judgment | referenced version | version referencing |
| F[operative,completeness] | prerequisite record; readiness account; inventory mastery; ordering insight | work coverage | p1=needed list; p2=complete account; p3=inventory mastery; p4=ordered insight | complete listing | result enumeration |
| F[operative,consistency] | prerequisite measurement; readiness message; inventory understanding; ordering reasoning | work coherence | p1=repeatable measure; p2=stable readiness; p3=understood list; p4=ordered reason | reproducible output | deterministic generation |
| F[evaluative,necessity] | criterion fact; evidence signal; inventory understanding; reliability discernment | value need | p1=boundary fact; p2=proof cue; p3=closure basis; p4=reliability trigger | boundary test | boundary verification |
| F[evaluative,sufficiency] | criterion evidence; evidence context; inventory expertise; reliability judgment | value adequacy | p1=review proof; p2=adequate context; p3=complete competence; p4=reliable judgment | comparison trust | regression confidence |
| F[evaluative,completeness] | criterion record; evidence account; inventory mastery; reliability insight | value coverage | p1=assessment record; p2=evidence account; p3=closure mastery; p4=quality insight | coverage review | coverage assessment |
| F[evaluative,consistency] | criterion measurement; evidence message; inventory understanding; reliability reasoning | value coherence | p1=integrity measure; p2=stable proof; p3=understood closure; p4=reliable reasoning | stable check | integrity check |

### Result

| | **necessity** | **sufficiency** | **completeness** | **consistency** |
|---|---|---|---|---|
| **normative** | envelope contract | provenance proof | schema completeness | unit stability |
| **operative** | export workflow | version referencing | result enumeration | deterministic generation |
| **evaluative** | boundary verification | regression confidence | coverage assessment | integrity check |

## Matrix D - Objectives (3x4)

### Construction: Addition A plus resolution-transformed F

`L_D(i,j) = A(i,j), resolution*F(i,j_mapped)`, then `D(i,j) = I(row_i, col_j, L_D(i,j))`.

| Cell | Intermediate collection | Step 1: axis anchor | Step 2: projected contributors | Step 3: centroid | Result |
|---|---|---|---|---|---|
| D[normative,guiding] | prescriptive direction; resolved envelope contract | policy frame | p1=bounded instruction; p2=closed envelope | governed direction | boundary charter |
| D[normative,applying] | mandatory practice; resolved provenance proof | policy action | p1=required practice; p2=closed evidence | required export | export mandate |
| D[normative,judging] | compliance determination; resolved schema completeness | policy decision | p1=decision rule; p2=closed contract | integrity decision | integrity ruling |
| D[normative,reviewing] | regulatory audit; resolved unit stability | policy inspection | p1=audit frame; p2=closed unit basis | governed inspection | audit protocol |
| D[operative,guiding] | procedural direction; resolved export workflow | work frame | p1=method direction; p2=closed workflow | usable method | workflow direction |
| D[operative,applying] | practical execution; resolved version referencing | work action | p1=execution move; p2=closed reference | assembled envelope | envelope assembly |
| D[operative,judging] | performance assessment; resolved result enumeration | work decision | p1=performance check; p2=closed result list | operational review | execution assessment |
| D[operative,reviewing] | process audit; resolved deterministic generation | work inspection | p1=process trace; p2=closed reproducibility | workflow trace | process trace |
| D[evaluative,guiding] | value orientation; resolved boundary verification | appraisal frame | p1=value frame; p2=closed boundary | reasoned value | value rationale |
| D[evaluative,applying] | merit application; resolved regression confidence | appraisal action | p1=applied merit; p2=closed comparison | applied proof | evidence application |
| D[evaluative,judging] | worth determination; resolved coverage assessment | appraisal decision | p1=worth check; p2=closed coverage | fitness decision | fitness ruling |
| D[evaluative,reviewing] | quality appraisal; resolved integrity check | appraisal inspection | p1=quality check; p2=closed integrity | quality review | quality review |

### Result

| | **guiding** | **applying** | **judging** | **reviewing** |
|---|---|---|---|---|
| **normative** | boundary charter | export mandate | integrity ruling | audit protocol |
| **operative** | workflow direction | envelope assembly | execution assessment | process trace |
| **evaluative** | value rationale | evidence application | fitness ruling | quality review |

## Matrix K - Transpose of D (4x3)

### Construction: K(i,j) = D(j,i)

### Result

| | **normative** | **operative** | **evaluative** |
|---|---|---|---|
| **guiding** | boundary charter | workflow direction | value rationale |
| **applying** | export mandate | envelope assembly | evidence application |
| **judging** | integrity ruling | execution assessment | fitness ruling |
| **reviewing** | audit protocol | process trace | quality review |

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

`L_X(i,j) = K(i,normative)*G(data,j), K(i,operative)*G(information,j), K(i,evaluative)*G(knowledge,j)`, then `X(i,j) = I(row_i, col_j, L_X(i,j))`.

| Cell | Intermediate collection | Step 1: axis anchor | Step 2: projected contributors | Step 3: centroid | Result |
|---|---|---|---|---|---|
| X[guiding,necessity] | charter fact; workflow signal; rationale understanding | orientation need | p1=charter source; p2=workflow cue; p3=rationale basis | required rationale | charter evidence |
| X[guiding,sufficiency] | charter evidence; workflow context; rationale expertise | orientation adequacy | p1=principle proof; p2=context fit; p3=expert rationale | supported principle | principle support |
| X[guiding,completeness] | charter record; workflow account; rationale mastery | orientation coverage | p1=full charter; p2=complete workflow; p3=mastered reason | complete rationale | coverage rationale |
| X[guiding,consistency] | charter measurement; workflow message; rationale understanding | orientation coherence | p1=stable charter; p2=aligned workflow; p3=coherent reason | stable rationale | coherent rationale |
| X[applying,necessity] | mandate fact; assembly signal; evidence understanding | action need | p1=required input; p2=assembly cue; p3=proof basis | run readiness | input readiness |
| X[applying,sufficiency] | mandate evidence; assembly context; evidence expertise | action adequacy | p1=proofed mandate; p2=fit assembly; p3=competent proof | execution proof | workflow proof |
| X[applying,completeness] | mandate record; assembly account; evidence mastery | action coverage | p1=complete mandate; p2=full assembly; p3=mastered proof | complete capture | complete capture |
| X[applying,consistency] | mandate measurement; assembly message; evidence understanding | action coherence | p1=stable mandate; p2=aligned assembly; p3=understood proof | stable action | stable execution |
| X[judging,necessity] | ruling fact; assessment signal; ruling understanding | decision need | p1=integrity fact; p2=assessment cue; p3=fitness basis | decision basis | integrity basis |
| X[judging,sufficiency] | ruling evidence; assessment context; ruling expertise | decision adequacy | p1=acceptance proof; p2=contexted assessment; p3=expert basis | enough acceptance | acceptance proof |
| X[judging,completeness] | ruling record; assessment account; ruling mastery | decision coverage | p1=recorded ruling; p2=complete assessment; p3=mastered closure | closing evidence | closure evidence |
| X[judging,consistency] | ruling measurement; assessment message; ruling understanding | decision coherence | p1=stable ruling; p2=aligned assessment; p3=coherent fitness | stable decision | consistent decision |
| X[reviewing,necessity] | protocol fact; trace signal; review understanding | inspection need | p1=audit fact; p2=trace cue; p3=quality basis | inspection trigger | audit trigger |
| X[reviewing,sufficiency] | protocol evidence; trace context; review expertise | inspection adequacy | p1=protocol proof; p2=trace context; p3=quality competence | enough review | review evidence |
| X[reviewing,completeness] | protocol record; trace account; review mastery | inspection coverage | p1=complete protocol; p2=full trace; p3=quality mastery | audit coverage | review coverage |
| X[reviewing,consistency] | protocol measurement; trace message; review understanding | inspection coherence | p1=stable protocol; p2=aligned trace; p3=coherent quality | stable trace | trace consistency |

### Result

| | **necessity** | **sufficiency** | **completeness** | **consistency** |
|---|---|---|---|---|
| **guiding** | charter evidence | principle support | coverage rationale | coherent rationale |
| **applying** | input readiness | workflow proof | complete capture | stable execution |
| **judging** | integrity basis | acceptance proof | closure evidence | consistent decision |
| **reviewing** | audit trigger | review evidence | review coverage | trace consistency |

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

`L_E(i,j) = X(i,necessity)*T(necessity,j), X(i,sufficiency)*T(sufficiency,j), X(i,completeness)*T(completeness,j), X(i,consistency)*T(consistency,j)`, then `E(i,j) = I(row_i, col_j, L_E(i,j))`.

| Cell | Intermediate collection | Step 1: axis anchor | Step 2: projected contributors | Step 3: centroid | Result |
|---|---|---|---|---|---|
| E[guiding,data] | evidence fact; support evidence; rationale record; rationale measurement | orientation datum | p1=source fact; p2=principle proof; p3=covered reason; p4=stable rationale | source principles | source principles |
| E[guiding,information] | evidence signal; support context; rationale account; rationale message | orientation message | p1=context cue; p2=principle context; p3=reason account; p4=coherent message | contextual reason | context rationale |
| E[guiding,knowledge] | evidence understanding; support expertise; rationale mastery; rationale understanding | orientation expertise | p1=understood source; p2=expert support; p3=mastered reason; p4=coherent grasp | method grasp | method understanding |
| E[guiding,wisdom] | evidence discernment; support judgment; rationale insight; rationale reasoning | orientation judgment | p1=boundary discernment; p2=judged principle; p3=insightful reason; p4=principled reason | boundary sense | boundary discernment |
| E[applying,data] | readiness fact; proof evidence; capture record; execution measurement | action datum | p1=ready fact; p2=workflow proof; p3=captured record; p4=stable measure | exported facts | export facts |
| E[applying,information] | readiness signal; proof context; capture account; execution message | action message | p1=ready cue; p2=workflow context; p3=captured account; p4=stable message | export context | execution context |
| E[applying,knowledge] | readiness understanding; proof expertise; capture mastery; execution understanding | action expertise | p1=understood readiness; p2=competent workflow; p3=mastered capture; p4=stable grasp | working skill | operator knowhow |
| E[applying,wisdom] | readiness discernment; proof judgment; capture insight; execution reasoning | action judgment | p1=readiness judgment; p2=workflow judgment; p3=capture insight; p4=reasoned execution | practical judgment | implementation judgment |
| E[judging,data] | basis fact; proof evidence; evidence record; decision measurement | decision datum | p1=integrity fact; p2=acceptance proof; p3=closure record; p4=decision measure | integrity indicators | integrity indicators |
| E[judging,information] | basis signal; proof context; evidence account; decision message | decision message | p1=integrity cue; p2=acceptance context; p3=closure account; p4=decision message | acceptance signals | acceptance signals |
| E[judging,knowledge] | basis understanding; proof expertise; evidence mastery; decision understanding | decision expertise | p1=integrity grasp; p2=expert proof; p3=closure mastery; p4=decision grasp | review skill | review expertise |
| E[judging,wisdom] | basis discernment; proof judgment; evidence insight; decision reasoning | decision judgment | p1=integrity discernment; p2=acceptance judgment; p3=closure insight; p4=principled decision | professional judgment | professional judgment |
| E[reviewing,data] | trigger fact; evidence evidence; coverage record; consistency measurement | inspection datum | p1=audit fact; p2=review proof; p3=coverage record; p4=trace measure | audit records | audit records |
| E[reviewing,information] | trigger signal; evidence context; coverage account; consistency message | inspection message | p1=audit cue; p2=review context; p3=coverage account; p4=trace message | trace messages | trace messages |
| E[reviewing,knowledge] | trigger understanding; evidence expertise; coverage mastery; consistency understanding | inspection expertise | p1=audit grasp; p2=review skill; p3=coverage mastery; p4=trace grasp | quality insight | quality insight |
| E[reviewing,wisdom] | trigger discernment; evidence judgment; coverage insight; consistency reasoning | inspection judgment | p1=audit discernment; p2=review judgment; p3=coverage insight; p4=trace reason | responsibility reasoning | responsibility reasoning |

### Result

| | **data** | **information** | **knowledge** | **wisdom** |
|---|---|---|---|---|
| **guiding** | source principles | context rationale | method understanding | boundary discernment |
| **applying** | export facts | execution context | operator knowhow | implementation judgment |
| **judging** | integrity indicators | acceptance signals | review expertise | professional judgment |
| **reviewing** | audit records | trace messages | quality insight | responsibility reasoning |

---

## Matrix Summary

### Matrix C

| | **necessity** | **sufficiency** | **completeness** | **consistency** |
|---|---|---|---|---|
| **normative** | envelope mandate | evidence threshold | contract coverage | schema coherence |
| **operative** | export prerequisite | payload readiness | result inventory | stable ordering |
| **evaluative** | boundary criterion | review evidence | closure inventory | comparison reliability |

### Matrix F

| | **necessity** | **sufficiency** | **completeness** | **consistency** |
|---|---|---|---|---|
| **normative** | envelope contract | provenance proof | schema completeness | unit stability |
| **operative** | export workflow | version referencing | result enumeration | deterministic generation |
| **evaluative** | boundary verification | regression confidence | coverage assessment | integrity check |

### Matrix D

| | **guiding** | **applying** | **judging** | **reviewing** |
|---|---|---|---|---|
| **normative** | boundary charter | export mandate | integrity ruling | audit protocol |
| **operative** | workflow direction | envelope assembly | execution assessment | process trace |
| **evaluative** | value rationale | evidence application | fitness ruling | quality review |

### Matrix K

| | **normative** | **operative** | **evaluative** |
|---|---|---|---|
| **guiding** | boundary charter | workflow direction | value rationale |
| **applying** | export mandate | envelope assembly | evidence application |
| **judging** | integrity ruling | execution assessment | fitness ruling |
| **reviewing** | audit protocol | process trace | quality review |

### Matrix G

| | **necessity** | **sufficiency** | **completeness** | **consistency** |
|---|---|---|---|---|
| **data** | essential fact | adequate evidence | comprehensive record | reliable measurement |
| **information** | essential signal | adequate context | comprehensive account | coherent message |
| **knowledge** | fundamental understanding | competent expertise | thorough mastery | coherent understanding |

### Matrix X

| | **necessity** | **sufficiency** | **completeness** | **consistency** |
|---|---|---|---|---|
| **guiding** | charter evidence | principle support | coverage rationale | coherent rationale |
| **applying** | input readiness | workflow proof | complete capture | stable execution |
| **judging** | integrity basis | acceptance proof | closure evidence | consistent decision |
| **reviewing** | audit trigger | review evidence | review coverage | trace consistency |

### Matrix T

| | **data** | **information** | **knowledge** | **wisdom** |
|---|---|---|---|---|
| **necessity** | essential fact | essential signal | fundamental understanding | essential discernment |
| **sufficiency** | adequate evidence | adequate context | competent expertise | adequate judgment |
| **completeness** | comprehensive record | comprehensive account | thorough mastery | holistic insight |
| **consistency** | reliable measurement | coherent message | coherent understanding | principled reasoning |

### Matrix E

| | **data** | **information** | **knowledge** | **wisdom** |
|---|---|---|---|---|
| **guiding** | source principles | context rationale | method understanding | boundary discernment |
| **applying** | export facts | execution context | operator knowhow | implementation judgment |
| **judging** | integrity indicators | acceptance signals | review expertise | professional judgment |
| **reviewing** | audit records | trace messages | quality insight | responsibility reasoning |

## Audit Result

PASS. Final result cells are populated, compact, free of algebra notation, and contain no implementation particulars, numeric engineering values, protected payloads, or professional approval claims.
