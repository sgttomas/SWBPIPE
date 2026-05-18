# Deliverable: DEL-12-05 Security threat model

**Generated:** 2026-04-30
**DECOMP_VARIANT:** SOFTWARE
**Perspective:** This deliverable frames the risks that arise when private project, rule-pack, component, report, plugin/import, and supply-chain data crosses local-first workflow boundaries. It needs to carry threat categories, disclosure pressure points, conservative controls, and updateable gaps without drifting into legal, certification, or implementation claims.
**Framework:** Chirality Semantic Algebra
**Inputs Read:**
- _CONTEXT.md — deliverable-local identity, scope, and architecture-basis injection
- _STATUS.md — deliverable-local lifecycle state
- Datasheet.md — deliverable-local threat-model datasheet
- Specification.md — deliverable-local threat-model specification
- Guidance.md — deliverable-local threat-model guidance
- Procedure.md — deliverable-local threat-model procedure
- _REFERENCES.md — deliverable-local source pointer list

## Matrix A — Orientation (3×4) — Canonical
| | guiding | applying | judging | reviewing |
|---|---|---|---|---|
| normative | prescriptive direction | mandatory practice | compliance determination | regulatory audit |
| operative | procedural direction | practical execution | performance assessment | process audit |
| evaluative | value orientation | merit application | worth determination | quality appraisal |

## Matrix B — Conceptualization (4×4) — Canonical
| | necessity | sufficiency | completeness | consistency |
|---|---|---|---|---|
| data | essential fact | adequate evidence | comprehensive record | reliable measurement |
| information | essential signal | adequate context | comprehensive account | coherent message |
| knowledge | fundamental understanding | competent expertise | thorough mastery | coherent understanding |
| wisdom | essential discernment | adequate judgment | holistic insight | principled reasoning |

## Matrix C — Formulation (3×4)
### Construction: Dot product A · B
#### Cell C[normative, necessity]
- Step 1: axis anchor = prescriptive direction * necessity
- Step 2: projections = {prescriptive direction * essential fact, prescriptive direction * essential signal, prescriptive direction * fundamental understanding, prescriptive direction * essential discernment}
- Step 3: atomic unit = evidence requirement
#### Cell C[normative, sufficiency]
- Step 1: axis anchor = prescriptive direction * sufficiency
- Step 2: projections = {prescriptive direction * adequate evidence, prescriptive direction * adequate context, prescriptive direction * competent expertise, prescriptive direction * adequate judgment}
- Step 3: atomic unit = adequate coverage
#### Cell C[normative, completeness]
- Step 1: axis anchor = prescriptive direction * completeness
- Step 2: projections = {prescriptive direction * comprehensive record, prescriptive direction * comprehensive account, prescriptive direction * thorough mastery, prescriptive direction * holistic insight}
- Step 3: atomic unit = full scope record
#### Cell C[normative, consistency]
- Step 1: axis anchor = prescriptive direction * consistency
- Step 2: projections = {prescriptive direction * reliable measurement, prescriptive direction * coherent message, prescriptive direction * coherent understanding, prescriptive direction * principled reasoning}
- Step 3: atomic unit = coherent evidence
#### Cell C[operative, necessity]
- Step 1: axis anchor = procedural direction * necessity
- Step 2: projections = {procedural direction * essential fact, procedural direction * essential signal, procedural direction * fundamental understanding, procedural direction * essential discernment}
- Step 3: atomic unit = required action
#### Cell C[operative, sufficiency]
- Step 1: axis anchor = procedural direction * sufficiency
- Step 2: projections = {procedural direction * adequate evidence, procedural direction * adequate context, procedural direction * competent expertise, procedural direction * adequate judgment}
- Step 3: atomic unit = adequate action
#### Cell C[operative, completeness]
- Step 1: axis anchor = procedural direction * completeness
- Step 2: projections = {procedural direction * comprehensive record, procedural direction * comprehensive account, procedural direction * thorough mastery, procedural direction * holistic insight}
- Step 3: atomic unit = full workflow
#### Cell C[operative, consistency]
- Step 1: axis anchor = procedural direction * consistency
- Step 2: projections = {procedural direction * reliable measurement, procedural direction * coherent message, procedural direction * coherent understanding, procedural direction * principled reasoning}
- Step 3: atomic unit = stable workflow
#### Cell C[evaluative, necessity]
- Step 1: axis anchor = value orientation * necessity
- Step 2: projections = {value orientation * essential fact, value orientation * essential signal, value orientation * fundamental understanding, value orientation * essential discernment}
- Step 3: atomic unit = risk priority
#### Cell C[evaluative, sufficiency]
- Step 1: axis anchor = value orientation * sufficiency
- Step 2: projections = {value orientation * adequate evidence, value orientation * adequate context, value orientation * competent expertise, value orientation * adequate judgment}
- Step 3: atomic unit = measured adequacy
#### Cell C[evaluative, completeness]
- Step 1: axis anchor = value orientation * completeness
- Step 2: projections = {value orientation * comprehensive record, value orientation * comprehensive account, value orientation * thorough mastery, value orientation * holistic insight}
- Step 3: atomic unit = complete risk picture
#### Cell C[evaluative, consistency]
- Step 1: axis anchor = value orientation * consistency
- Step 2: projections = {value orientation * reliable measurement, value orientation * coherent message, value orientation * coherent understanding, value orientation * principled reasoning}
- Step 3: atomic unit = aligned judgment

### Result
| | necessity | sufficiency | completeness | consistency |
|---|---|---|---|---|
| normative | evidence requirement | adequate coverage | full scope record | coherent evidence |
| operative | required action | adequate action | full workflow | stable workflow |
| evaluative | risk priority | measured adequacy | complete risk picture | aligned judgment |

## Matrix F — Requirements (3×4)
### Construction: Dot product C · B
#### Cell F[normative, necessity]
- Step 1: axis anchor = evidence requirement * necessity
- Step 2: projections = {evidence requirement * essential fact, evidence requirement * essential signal, evidence requirement * fundamental understanding, evidence requirement * essential discernment}
- Step 3: atomic unit = threat evidence
#### Cell F[normative, sufficiency]
- Step 1: axis anchor = evidence requirement * sufficiency
- Step 2: projections = {evidence requirement * adequate evidence, evidence requirement * adequate context, evidence requirement * competent expertise, evidence requirement * adequate judgment}
- Step 3: atomic unit = sufficient coverage
#### Cell F[normative, completeness]
- Step 1: axis anchor = evidence requirement * completeness
- Step 2: projections = {evidence requirement * comprehensive record, evidence requirement * comprehensive account, evidence requirement * thorough mastery, evidence requirement * holistic insight}
- Step 3: atomic unit = complete coverage
#### Cell F[normative, consistency]
- Step 1: axis anchor = evidence requirement * consistency
- Step 2: projections = {evidence requirement * reliable measurement, evidence requirement * coherent message, evidence requirement * coherent understanding, evidence requirement * principled reasoning}
- Step 3: atomic unit = consistent evidence
#### Cell F[operative, necessity]
- Step 1: axis anchor = required action * necessity
- Step 2: projections = {required action * essential fact, required action * essential signal, required action * fundamental understanding, required action * essential discernment}
- Step 3: atomic unit = disclosure action
#### Cell F[operative, sufficiency]
- Step 1: axis anchor = required action * sufficiency
- Step 2: projections = {required action * adequate evidence, required action * adequate context, required action * competent expertise, required action * adequate judgment}
- Step 3: atomic unit = adequate mitigation
#### Cell F[operative, completeness]
- Step 1: axis anchor = required action * completeness
- Step 2: projections = {required action * comprehensive record, required action * comprehensive account, required action * thorough mastery, required action * holistic insight}
- Step 3: atomic unit = complete mitigation
#### Cell F[operative, consistency]
- Step 1: axis anchor = required action * consistency
- Step 2: projections = {required action * reliable measurement, required action * coherent message, required action * coherent understanding, required action * principled reasoning}
- Step 3: atomic unit = stable mitigation
#### Cell F[evaluative, necessity]
- Step 1: axis anchor = risk priority * necessity
- Step 2: projections = {risk priority * essential fact, risk priority * essential signal, risk priority * fundamental understanding, risk priority * essential discernment}
- Step 3: atomic unit = priority threat
#### Cell F[evaluative, sufficiency]
- Step 1: axis anchor = risk priority * sufficiency
- Step 2: projections = {risk priority * adequate evidence, risk priority * adequate context, risk priority * competent expertise, risk priority * adequate judgment}
- Step 3: atomic unit = sufficient threat
#### Cell F[evaluative, completeness]
- Step 1: axis anchor = risk priority * completeness
- Step 2: projections = {risk priority * comprehensive record, risk priority * comprehensive account, risk priority * thorough mastery, risk priority * holistic insight}
- Step 3: atomic unit = complete threat
#### Cell F[evaluative, consistency]
- Step 1: axis anchor = risk priority * consistency
- Step 2: projections = {risk priority * reliable measurement, risk priority * coherent message, risk priority * coherent understanding, risk priority * principled reasoning}
- Step 3: atomic unit = aligned threat

### Result
| | necessity | sufficiency | completeness | consistency |
|---|---|---|---|---|
| normative | threat evidence | sufficient coverage | complete coverage | consistent evidence |
| operative | disclosure action | adequate mitigation | complete mitigation | stable mitigation |
| evaluative | priority threat | sufficient threat | complete threat | aligned threat |

## Matrix D — Objectives (3×4)
### Construction: Addition A + resolution-transformed F
#### Cell D[normative, guiding]
- Step 1: axis anchor = prescriptive direction * guiding
- Step 2: projections = {prescriptive direction * threat evidence, prescriptive direction * adequate coverage}
- Step 3: atomic unit = disclosure guidance
#### Cell D[normative, applying]
- Step 1: axis anchor = prescriptive direction * applying
- Step 2: projections = {prescriptive direction * full scope record, prescriptive direction * sufficient coverage}
- Step 3: atomic unit = control application
#### Cell D[normative, judging]
- Step 1: axis anchor = prescriptive direction * judging
- Step 2: projections = {prescriptive direction * coherent evidence, prescriptive direction * consistent evidence}
- Step 3: atomic unit = risk judgment
#### Cell D[normative, reviewing]
- Step 1: axis anchor = prescriptive direction * reviewing
- Step 2: projections = {prescriptive direction * stable workflow, prescriptive direction * aligned threat}
- Step 3: atomic unit = review guidance
#### Cell D[operative, guiding]
- Step 1: axis anchor = procedural direction * guiding
- Step 2: projections = {procedural direction * required action, procedural direction * disclosure action}
- Step 3: atomic unit = action guidance
#### Cell D[operative, applying]
- Step 1: axis anchor = procedural direction * applying
- Step 2: projections = {procedural direction * adequate action, procedural direction * adequate mitigation}
- Step 3: atomic unit = mitigation practice
#### Cell D[operative, judging]
- Step 1: axis anchor = procedural direction * judging
- Step 2: projections = {procedural direction * stable workflow, procedural direction * consistent evidence}
- Step 3: atomic unit = operational judgment
#### Cell D[operative, reviewing]
- Step 1: axis anchor = procedural direction * reviewing
- Step 2: projections = {procedural direction * risk priority, procedural direction * priority threat}
- Step 3: atomic unit = review practice
#### Cell D[evaluative, guiding]
- Step 1: axis anchor = value orientation * guiding
- Step 2: projections = {value orientation * risk priority, value orientation * disclosure action}
- Step 3: atomic unit = risk framing
#### Cell D[evaluative, applying]
- Step 1: axis anchor = value orientation * applying
- Step 2: projections = {value orientation * measured adequacy, value orientation * sufficient threat}
- Step 3: atomic unit = measured control
#### Cell D[evaluative, judging]
- Step 1: axis anchor = value orientation * judging
- Step 2: projections = {value orientation * complete risk picture, value orientation * complete threat}
- Step 3: atomic unit = complete judgment
#### Cell D[evaluative, reviewing]
- Step 1: axis anchor = value orientation * reviewing
- Step 2: projections = {value orientation * aligned judgment, value orientation * aligned threat}
- Step 3: atomic unit = aligned review

### Result
| | guiding | applying | judging | reviewing |
|---|---|---|---|---|
| normative | disclosure guidance | control application | risk judgment | review guidance |
| operative | action guidance | mitigation practice | operational judgment | review practice |
| evaluative | risk framing | measured control | complete judgment | aligned review |

## Matrix K — Transpose of D (4×3)
### Construction: K(i,j) = D(j,i)
### Result
| | normative | operative | evaluative |
|---|---|---|---|
| guiding | disclosure guidance | action guidance | risk framing |
| applying | control application | mitigation practice | measured control |
| judging | risk judgment | operational judgment | complete judgment |
| reviewing | review guidance | review practice | aligned review |

## Matrix G — Truncation of B (3×4)
### Construction: remove `wisdom` row from B
### Result
| | necessity | sufficiency | completeness | consistency |
|---|---|---|---|---|
| data | essential fact | adequate evidence | comprehensive record | reliable measurement |
| information | essential signal | adequate context | comprehensive account | coherent message |
| knowledge | fundamental understanding | competent expertise | thorough mastery | coherent understanding |

## Matrix X — Verification (4×4)
### Construction: Dot product K · G
#### Cell X[guiding, necessity]
- Step 1: axis anchor = disclosure guidance * necessity
- Step 2: projections = {disclosure guidance * essential fact, disclosure guidance * essential signal, disclosure guidance * fundamental understanding}
- Step 3: atomic unit = disclosure check
#### Cell X[guiding, sufficiency]
- Step 1: axis anchor = disclosure guidance * sufficiency
- Step 2: projections = {disclosure guidance * adequate evidence, disclosure guidance * adequate context, disclosure guidance * competent expertise}
- Step 3: atomic unit = coverage check
#### Cell X[guiding, completeness]
- Step 1: axis anchor = disclosure guidance * completeness
- Step 2: projections = {disclosure guidance * comprehensive record, disclosure guidance * comprehensive account, disclosure guidance * thorough mastery}
- Step 3: atomic unit = complete check
#### Cell X[guiding, consistency]
- Step 1: axis anchor = disclosure guidance * consistency
- Step 2: projections = {disclosure guidance * reliable measurement, disclosure guidance * coherent message, disclosure guidance * coherent understanding}
- Step 3: atomic unit = aligned check
#### Cell X[applying, necessity]
- Step 1: axis anchor = control application * necessity
- Step 2: projections = {control application * essential fact, control application * essential signal, control application * fundamental understanding}
- Step 3: atomic unit = control check
#### Cell X[applying, sufficiency]
- Step 1: axis anchor = control application * sufficiency
- Step 2: projections = {control application * adequate evidence, control application * adequate context, control application * competent expertise}
- Step 3: atomic unit = adequate check
#### Cell X[applying, completeness]
- Step 1: axis anchor = control application * completeness
- Step 2: projections = {control application * comprehensive record, control application * comprehensive account, control application * thorough mastery}
- Step 3: atomic unit = complete control
#### Cell X[applying, consistency]
- Step 1: axis anchor = control application * consistency
- Step 2: projections = {control application * reliable measurement, control application * coherent message, control application * coherent understanding}
- Step 3: atomic unit = stable control
#### Cell X[judging, necessity]
- Step 1: axis anchor = risk judgment * necessity
- Step 2: projections = {risk judgment * essential fact, risk judgment * essential signal, risk judgment * fundamental understanding}
- Step 3: atomic unit = risk check
#### Cell X[judging, sufficiency]
- Step 1: axis anchor = risk judgment * sufficiency
- Step 2: projections = {risk judgment * adequate evidence, risk judgment * adequate context, risk judgment * competent expertise}
- Step 3: atomic unit = adequate risk
#### Cell X[judging, completeness]
- Step 1: axis anchor = risk judgment * completeness
- Step 2: projections = {risk judgment * comprehensive record, risk judgment * comprehensive account, risk judgment * thorough mastery}
- Step 3: atomic unit = complete risk
#### Cell X[judging, consistency]
- Step 1: axis anchor = risk judgment * consistency
- Step 2: projections = {risk judgment * reliable measurement, risk judgment * coherent message, risk judgment * coherent understanding}
- Step 3: atomic unit = coherent risk
#### Cell X[reviewing, necessity]
- Step 1: axis anchor = review guidance * necessity
- Step 2: projections = {review guidance * essential fact, review guidance * essential signal, review guidance * fundamental understanding}
- Step 3: atomic unit = review check
#### Cell X[reviewing, sufficiency]
- Step 1: axis anchor = review guidance * sufficiency
- Step 2: projections = {review guidance * adequate evidence, review guidance * adequate context, review guidance * competent expertise}
- Step 3: atomic unit = adequate review
#### Cell X[reviewing, completeness]
- Step 1: axis anchor = review guidance * completeness
- Step 2: projections = {review guidance * comprehensive record, review guidance * comprehensive account, review guidance * thorough mastery}
- Step 3: atomic unit = complete review
#### Cell X[reviewing, consistency]
- Step 1: axis anchor = review guidance * consistency
- Step 2: projections = {review guidance * reliable measurement, review guidance * coherent message, review guidance * coherent understanding}
- Step 3: atomic unit = stable review

### Result
| | necessity | sufficiency | completeness | consistency |
|---|---|---|---|---|
| guiding | disclosure check | coverage check | complete check | aligned check |
| applying | control check | adequate check | complete control | stable control |
| judging | risk check | adequate risk | complete risk | coherent risk |
| reviewing | review check | adequate review | complete review | stable review |

## Matrix T — Transpose of B (4×4)
### Construction: T(i,j) = B(j,i)
### Result
| | data | information | knowledge | wisdom |
|---|---|---|---|---|
| necessity | essential fact | essential signal | fundamental understanding | essential discernment |
| sufficiency | adequate evidence | adequate context | competent expertise | adequate judgment |
| completeness | comprehensive record | comprehensive account | thorough mastery | holistic insight |
| consistency | reliable measurement | coherent message | coherent understanding | principled reasoning |

## Matrix E — Evaluation (4×4)
### Construction: Dot product X · T
#### Cell E[guiding, data]
- Step 1: axis anchor = disclosure check * data
- Step 2: projections = {disclosure check * essential fact, disclosure check * adequate evidence, disclosure check * comprehensive record, disclosure check * reliable measurement}
- Step 3: atomic unit = disclosure datum
#### Cell E[guiding, information]
- Step 1: axis anchor = disclosure check * information
- Step 2: projections = {disclosure check * essential signal, disclosure check * adequate context, disclosure check * comprehensive account, disclosure check * coherent message}
- Step 3: atomic unit = disclosure context
#### Cell E[guiding, knowledge]
- Step 1: axis anchor = disclosure check * knowledge
- Step 2: projections = {disclosure check * fundamental understanding, disclosure check * competent expertise, disclosure check * thorough mastery, disclosure check * coherent understanding}
- Step 3: atomic unit = disclosure knowledge
#### Cell E[guiding, wisdom]
- Step 1: axis anchor = disclosure check * wisdom
- Step 2: projections = {disclosure check * essential discernment, disclosure check * adequate judgment, disclosure check * holistic insight, disclosure check * principled reasoning}
- Step 3: atomic unit = disclosure judgment
#### Cell E[applying, data]
- Step 1: axis anchor = control check * data
- Step 2: projections = {control check * essential fact, control check * adequate evidence, control check * comprehensive record, control check * reliable measurement}
- Step 3: atomic unit = control datum
#### Cell E[applying, information]
- Step 1: axis anchor = control check * information
- Step 2: projections = {control check * essential signal, control check * adequate context, control check * comprehensive account, control check * coherent message}
- Step 3: atomic unit = control context
#### Cell E[applying, knowledge]
- Step 1: axis anchor = control check * knowledge
- Step 2: projections = {control check * fundamental understanding, control check * competent expertise, control check * thorough mastery, control check * coherent understanding}
- Step 3: atomic unit = control knowledge
#### Cell E[applying, wisdom]
- Step 1: axis anchor = control check * wisdom
- Step 2: projections = {control check * essential discernment, control check * adequate judgment, control check * holistic insight, control check * principled reasoning}
- Step 3: atomic unit = control judgment
#### Cell E[judging, data]
- Step 1: axis anchor = risk check * data
- Step 2: projections = {risk check * essential fact, risk check * adequate evidence, risk check * comprehensive record, risk check * reliable measurement}
- Step 3: atomic unit = risk datum
#### Cell E[judging, information]
- Step 1: axis anchor = risk check * information
- Step 2: projections = {risk check * essential signal, risk check * adequate context, risk check * comprehensive account, risk check * coherent message}
- Step 3: atomic unit = risk context
#### Cell E[judging, knowledge]
- Step 1: axis anchor = risk check * knowledge
- Step 2: projections = {risk check * fundamental understanding, risk check * competent expertise, risk check * thorough mastery, risk check * coherent understanding}
- Step 3: atomic unit = risk knowledge
#### Cell E[judging, wisdom]
- Step 1: axis anchor = risk check * wisdom
- Step 2: projections = {risk check * essential discernment, risk check * adequate judgment, risk check * holistic insight, risk check * principled reasoning}
- Step 3: atomic unit = risk judgment
#### Cell E[reviewing, data]
- Step 1: axis anchor = review check * data
- Step 2: projections = {review check * essential fact, review check * adequate evidence, review check * comprehensive record, review check * reliable measurement}
- Step 3: atomic unit = review datum
#### Cell E[reviewing, information]
- Step 1: axis anchor = review check * information
- Step 2: projections = {review check * essential signal, review check * adequate context, review check * comprehensive account, review check * coherent message}
- Step 3: atomic unit = review context
#### Cell E[reviewing, knowledge]
- Step 1: axis anchor = review check * knowledge
- Step 2: projections = {review check * fundamental understanding, review check * competent expertise, review check * thorough mastery, review check * coherent understanding}
- Step 3: atomic unit = review knowledge
#### Cell E[reviewing, wisdom]
- Step 1: axis anchor = review check * wisdom
- Step 2: projections = {review check * essential discernment, review check * adequate judgment, review check * holistic insight, review check * principled reasoning}
- Step 3: atomic unit = review judgment

### Result
| | data | information | knowledge | wisdom |
|---|---|---|---|---|
| guiding | disclosure datum | disclosure context | disclosure knowledge | disclosure judgment |
| applying | control datum | control context | control knowledge | control judgment |
| judging | risk datum | risk context | risk knowledge | risk judgment |
| reviewing | review datum | review context | review knowledge | review judgment |

## Matrix Summary
| Matrix | Compact view |
|---|---|
| C | evidence requirement; adequate coverage; full scope record; coherent evidence / required action; adequate action; full workflow; stable workflow / risk priority; measured adequacy; complete risk picture; aligned judgment |
| F | threat evidence; sufficient coverage; complete coverage; consistent evidence / disclosure action; adequate mitigation; complete mitigation; stable mitigation / priority threat; sufficient threat; complete threat; aligned threat |
| D | disclosure guidance; control application; risk judgment; review guidance / action guidance; mitigation practice; operational judgment; review practice / risk framing; measured control; complete judgment; aligned review |
| K | disclosure guidance; action guidance; risk framing / control application; mitigation practice; measured control / risk judgment; operational judgment; complete judgment / review guidance; review practice; aligned review |
| G | essential fact; adequate evidence; comprehensive record; reliable measurement / essential signal; adequate context; comprehensive account; coherent message / fundamental understanding; competent expertise; thorough mastery; coherent understanding |
| X | disclosure check; coverage check; complete check; aligned check / control check; adequate check; complete control; stable control / risk check; adequate risk; complete risk; coherent risk / review check; adequate review; complete review; stable review |
| T | essential fact; essential signal; fundamental understanding; essential discernment / adequate evidence; adequate context; competent expertise; adequate judgment / comprehensive record; comprehensive account; thorough mastery; holistic insight / reliable measurement; coherent message; coherent understanding; principled reasoning |
| E | disclosure datum; disclosure context; disclosure knowledge; disclosure judgment / control datum; control context; control knowledge; control judgment / risk datum; risk context; risk knowledge; risk judgment / review datum; review context; review knowledge; review judgment |

## Audit Result

**Result:** PASS

Final result tables for matrices C, F, D, X, and E contain compact interpreted phrases only. No matrix audit error, algebra notation leak, or addition-operator leak was observed in final result cells.
