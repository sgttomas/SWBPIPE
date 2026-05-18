# Semantic Lens: DEL-07-05 Results viewer

**Generated:** 2026-04-30
**DECOMP_VARIANT:** SOFTWARE
**Perspective:** This deliverable carries setup knowledge for a GUI results viewer that makes mechanics results, user-rule outputs, diagnostics, assumptions, and report/export readiness reviewable. It shapes later implementation questions without supplying protected engineering values, defining code thresholds, or asserting professional compliance.
**Framework:** Chirality Semantic Algebra
**Audit Result:** PASS

**Inputs Read:**
- _CONTEXT.md - identity, scope, objectives, and architecture-basis injection
- _STATUS.md - lifecycle state before setup
- Datasheet.md - deliverable metadata and boundary slots
- Specification.md - requirements and acceptance criteria
- Guidance.md - rationale, tradeoffs, and open issues
- Procedure.md - setup procedure and verification records
- _REFERENCES.md - source index

## Matrix A - Orientation - Canonical

| | **guiding** | **applying** | **judging** | **reviewing** |
|---|---|---|---|---|
| **normative** | prescriptive direction | mandatory practice | compliance determination | regulatory audit |
| **operative** | procedural direction | practical execution | performance assessment | process audit |
| **evaluative** | value orientation | merit application | worth determination | quality appraisal |

## Matrix B - Conceptualization - Canonical

| | **necessity** | **sufficiency** | **completeness** | **consistency** |
|---|---|---|---|---|
| **data** | essential fact | adequate evidence | comprehensive record | reliable measurement |
| **information** | essential signal | adequate context | comprehensive account | coherent message |
| **knowledge** | fundamental understanding | competent expertise | thorough mastery | coherent understanding |
| **wisdom** | essential discernment | adequate judgment | holistic insight | principled reasoning |

## Matrix C - Formulation

### Construction
Dot product A with B. Intermediate collections are interpreted per cell.

### Interpretation Work
- I(normative, necessity, L): Step 1 axis anchor = normative necessity; Step 2 projected contributors = result authority, boundary rule, visible diagnostic; Step 3 centroid attractor = `review boundary`.
- I(normative, sufficiency, L): Step 1 axis anchor = normative sufficiency; Step 2 projected contributors = result authority, source basis, warning evidence; Step 3 centroid attractor = `evidence threshold`.
- I(normative, completeness, L): Step 1 axis anchor = normative completeness; Step 2 projected contributors = result category, status class, handoff record; Step 3 centroid attractor = `coverage scope`.
- I(normative, consistency, L): Step 1 axis anchor = normative consistency; Step 2 projected contributors = status term, unit label, boundary notice; Step 3 centroid attractor = `terminology alignment`.
- I(operative, necessity, L): Step 1 axis anchor = operative necessity; Step 2 projected contributors = envelope input, selected case, available value; Step 3 centroid attractor = `input prerequisite`.
- I(operative, sufficiency, L): Step 1 axis anchor = operative sufficiency; Step 2 projected contributors = table view, overlay view, diagnostic view; Step 3 centroid attractor = `display method`.
- I(operative, completeness, L): Step 1 axis anchor = operative completeness; Step 2 projected contributors = result category, filter state, review surface; Step 3 centroid attractor = `panel coverage`.
- I(operative, consistency, L): Step 1 axis anchor = operative consistency; Step 2 projected contributors = service route, UI state, export signal; Step 3 centroid attractor = `workflow coherence`.
- I(evaluative, necessity, L): Step 1 axis anchor = evaluative necessity; Step 2 projected contributors = review criterion, blocked state, human boundary; Step 3 centroid attractor = `review criterion`.
- I(evaluative, sufficiency, L): Step 1 axis anchor = evaluative sufficiency; Step 2 projected contributors = model hash, solver version, checksum; Step 3 centroid attractor = `trace evidence`.
- I(evaluative, completeness, L): Step 1 axis anchor = evaluative completeness; Step 2 projected contributors = category coverage, warning coverage, export coverage; Step 3 centroid attractor = `quality coverage`.
- I(evaluative, consistency, L): Step 1 axis anchor = evaluative consistency; Step 2 projected contributors = status separation, professional boundary, report readiness; Step 3 centroid attractor = `decision coherence`.

### Result
| | **necessity** | **sufficiency** | **completeness** | **consistency** |
|---|---|---|---|---|
| **normative** | review boundary | evidence threshold | coverage scope | terminology alignment |
| **operative** | input prerequisite | display method | panel coverage | workflow coherence |
| **evaluative** | review criterion | trace evidence | quality coverage | decision coherence |

## Matrix F - Requirements

### Construction
Dot product C with B. Intermediate collections are interpreted per cell.

### Interpretation Work
- I(normative, necessity, L): Step 1 axis anchor = normative necessity; Step 2 projected contributors = review boundary, essential fact, source status; Step 3 centroid attractor = `required result boundary`.
- I(normative, sufficiency, L): Step 1 axis anchor = normative sufficiency; Step 2 projected contributors = evidence threshold, adequate context, warning source; Step 3 centroid attractor = `adequate source basis`.
- I(normative, completeness, L): Step 1 axis anchor = normative completeness; Step 2 projected contributors = coverage scope, comprehensive account, result categories; Step 3 centroid attractor = `complete category set`.
- I(normative, consistency, L): Step 1 axis anchor = normative consistency; Step 2 projected contributors = terminology alignment, coherent message, status terms; Step 3 centroid attractor = `consistent status model`.
- I(operative, necessity, L): Step 1 axis anchor = operative necessity; Step 2 projected contributors = input prerequisite, essential fact, envelope presence; Step 3 centroid attractor = `required envelope input`.
- I(operative, sufficiency, L): Step 1 axis anchor = operative sufficiency; Step 2 projected contributors = display method, adequate evidence, review use; Step 3 centroid attractor = `adequate review surface`.
- I(operative, completeness, L): Step 1 axis anchor = operative completeness; Step 2 projected contributors = panel coverage, comprehensive record, category surface; Step 3 centroid attractor = `complete panel set`.
- I(operative, consistency, L): Step 1 axis anchor = operative consistency; Step 2 projected contributors = workflow coherence, reliable measurement, state route; Step 3 centroid attractor = `consistent interaction route`.
- I(evaluative, necessity, L): Step 1 axis anchor = evaluative necessity; Step 2 projected contributors = review criterion, essential signal, blocked state; Step 3 centroid attractor = `required review evidence`.
- I(evaluative, sufficiency, L): Step 1 axis anchor = evaluative sufficiency; Step 2 projected contributors = trace evidence, adequate judgment, handoff proof; Step 3 centroid attractor = `adequate audit basis`.
- I(evaluative, completeness, L): Step 1 axis anchor = evaluative completeness; Step 2 projected contributors = quality coverage, holistic insight, readiness signal; Step 3 centroid attractor = `complete export signal`.
- I(evaluative, consistency, L): Step 1 axis anchor = evaluative consistency; Step 2 projected contributors = decision coherence, principled reasoning, professional notice; Step 3 centroid attractor = `consistent decision trail`.

### Result
| | **necessity** | **sufficiency** | **completeness** | **consistency** |
|---|---|---|---|---|
| **normative** | required result boundary | adequate source basis | complete category set | consistent status model |
| **operative** | required envelope input | adequate review surface | complete panel set | consistent interaction route |
| **evaluative** | required review evidence | adequate audit basis | complete export signal | consistent decision trail |

## Matrix D - Objectives

### Construction
Addition of Matrix A with resolution-transformed Matrix F. Intermediate collections are interpreted per cell.

### Interpretation Work
- I(normative, necessity, L): Step 1 axis anchor = normative necessity; Step 2 projected contributors = mandatory boundary, required result boundary, source rule; Step 3 centroid attractor = `boundary closure`.
- I(normative, sufficiency, L): Step 1 axis anchor = normative sufficiency; Step 2 projected contributors = adequate source basis, documented evidence, warning basis; Step 3 centroid attractor = `basis ruling`.
- I(normative, completeness, L): Step 1 axis anchor = normative completeness; Step 2 projected contributors = complete category set, scope ledger, result class; Step 3 centroid attractor = `category mandate`.
- I(normative, consistency, L): Step 1 axis anchor = normative consistency; Step 2 projected contributors = consistent status model, authority boundary, naming rule; Step 3 centroid attractor = `status governance`.
- I(operative, necessity, L): Step 1 axis anchor = operative necessity; Step 2 projected contributors = required envelope input, service route, data prerequisite; Step 3 centroid attractor = `envelope closure`.
- I(operative, sufficiency, L): Step 1 axis anchor = operative sufficiency; Step 2 projected contributors = adequate review surface, display control, user workflow; Step 3 centroid attractor = `usable method`.
- I(operative, completeness, L): Step 1 axis anchor = operative completeness; Step 2 projected contributors = complete panel set, filters, overlay options; Step 3 centroid attractor = `panel procedure`.
- I(operative, consistency, L): Step 1 axis anchor = operative consistency; Step 2 projected contributors = consistent interaction route, state route, export handoff; Step 3 centroid attractor = `review workflow`.
- I(evaluative, necessity, L): Step 1 axis anchor = evaluative necessity; Step 2 projected contributors = required review evidence, blocked status, human boundary; Step 3 centroid attractor = `review closure`.
- I(evaluative, sufficiency, L): Step 1 axis anchor = evaluative sufficiency; Step 2 projected contributors = adequate audit basis, trace evidence, source proof; Step 3 centroid attractor = `evidence sufficiency`.
- I(evaluative, completeness, L): Step 1 axis anchor = evaluative completeness; Step 2 projected contributors = complete export signal, report readiness, reproducibility cue; Step 3 centroid attractor = `export assessment`.
- I(evaluative, consistency, L): Step 1 axis anchor = evaluative consistency; Step 2 projected contributors = consistent decision trail, status separation, professional notice; Step 3 centroid attractor = `decision consistency`.

### Result
| | **necessity** | **sufficiency** | **completeness** | **consistency** |
|---|---|---|---|---|
| **normative** | boundary closure | basis ruling | category mandate | status governance |
| **operative** | envelope closure | usable method | panel procedure | review workflow |
| **evaluative** | review closure | evidence sufficiency | export assessment | decision consistency |

## Matrix K - Transpose of D

### Construction
Transpose of Matrix D: K(i,j) = D(j,i).

### Interpretation Work
- Direct structural transform; no list-valued dot-product cell requires interpretation.

### Result
| | **normative** | **operative** | **evaluative** |
|---|---|---|---|
| **necessity** | boundary closure | envelope closure | review closure |
| **sufficiency** | basis ruling | usable method | evidence sufficiency |
| **completeness** | category mandate | panel procedure | export assessment |
| **consistency** | status governance | review workflow | decision consistency |

## Matrix G - Truncation of B

### Construction
Truncation of Matrix B by removing the wisdom row.

### Interpretation Work
- Direct structural transform; no list-valued dot-product cell requires interpretation.

### Result
| | **necessity** | **sufficiency** | **completeness** | **consistency** |
|---|---|---|---|---|
| **data** | essential fact | adequate evidence | comprehensive record | reliable measurement |
| **information** | essential signal | adequate context | comprehensive account | coherent message |
| **knowledge** | fundamental understanding | competent expertise | thorough mastery | coherent understanding |

## Matrix X - Verification

### Construction
Dot product K with G. Intermediate collections are interpreted per cell.

### Interpretation Work
- I(guiding, necessity, L): Step 1 axis anchor = guiding necessity; Step 2 projected contributors = boundary closure, essential fact, result authority; Step 3 centroid attractor = `boundary prerequisite`.
- I(guiding, sufficiency, L): Step 1 axis anchor = guiding sufficiency; Step 2 projected contributors = basis ruling, adequate evidence, source context; Step 3 centroid attractor = `basis adequacy`.
- I(guiding, completeness, L): Step 1 axis anchor = guiding completeness; Step 2 projected contributors = category mandate, comprehensive record, SOW scope; Step 3 centroid attractor = `category coverage`.
- I(guiding, consistency, L): Step 1 axis anchor = guiding consistency; Step 2 projected contributors = status governance, reliable measurement, vocabulary; Step 3 centroid attractor = `status coherence`.
- I(applying, necessity, L): Step 1 axis anchor = applying necessity; Step 2 projected contributors = envelope closure, essential signal, input presence; Step 3 centroid attractor = `envelope prerequisite`.
- I(applying, sufficiency, L): Step 1 axis anchor = applying sufficiency; Step 2 projected contributors = usable method, adequate context, display route; Step 3 centroid attractor = `surface adequacy`.
- I(applying, completeness, L): Step 1 axis anchor = applying completeness; Step 2 projected contributors = panel procedure, comprehensive account, category set; Step 3 centroid attractor = `panel coverage`.
- I(applying, consistency, L): Step 1 axis anchor = applying consistency; Step 2 projected contributors = review workflow, coherent message, state route; Step 3 centroid attractor = `workflow coherence`.
- I(judging, necessity, L): Step 1 axis anchor = judging necessity; Step 2 projected contributors = review closure, fundamental understanding, professional boundary; Step 3 centroid attractor = `review prerequisite`.
- I(judging, sufficiency, L): Step 1 axis anchor = judging sufficiency; Step 2 projected contributors = evidence sufficiency, competent expertise, trace fields; Step 3 centroid attractor = `evidence adequacy`.
- I(judging, completeness, L): Step 1 axis anchor = judging completeness; Step 2 projected contributors = export assessment, thorough mastery, handoff record; Step 3 centroid attractor = `assessment coverage`.
- I(judging, consistency, L): Step 1 axis anchor = judging consistency; Step 2 projected contributors = decision consistency, coherent understanding, status separation; Step 3 centroid attractor = `decision coherence`.
- I(reviewing, necessity, L): Step 1 axis anchor = reviewing necessity; Step 2 projected contributors = boundary closure, essential fact, audit notice; Step 3 centroid attractor = `audit prerequisite`.
- I(reviewing, sufficiency, L): Step 1 axis anchor = reviewing sufficiency; Step 2 projected contributors = basis ruling, adequate evidence, run record; Step 3 centroid attractor = `record adequacy`.
- I(reviewing, completeness, L): Step 1 axis anchor = reviewing completeness; Step 2 projected contributors = category mandate, comprehensive record, dependency trace; Step 3 centroid attractor = `trace coverage`.
- I(reviewing, consistency, L): Step 1 axis anchor = reviewing consistency; Step 2 projected contributors = status governance, reliable measurement, report notice; Step 3 centroid attractor = `report coherence`.

### Result
| | **necessity** | **sufficiency** | **completeness** | **consistency** |
|---|---|---|---|---|
| **guiding** | boundary prerequisite | basis adequacy | category coverage | status coherence |
| **applying** | envelope prerequisite | surface adequacy | panel coverage | workflow coherence |
| **judging** | review prerequisite | evidence adequacy | assessment coverage | decision coherence |
| **reviewing** | audit prerequisite | record adequacy | trace coverage | report coherence |

## Matrix T - Transpose of B

### Construction
Transpose of Matrix B: T(i,j) = B(j,i).

### Interpretation Work
- Direct structural transform; no list-valued dot-product cell requires interpretation.

### Result
| | **data** | **information** | **knowledge** | **wisdom** |
|---|---|---|---|---|
| **necessity** | essential fact | essential signal | fundamental understanding | essential discernment |
| **sufficiency** | adequate evidence | adequate context | competent expertise | adequate judgment |
| **completeness** | comprehensive record | comprehensive account | thorough mastery | holistic insight |
| **consistency** | reliable measurement | coherent message | coherent understanding | principled reasoning |

## Matrix E - Evaluation

### Construction
Dot product X with T. Intermediate collections are interpreted per cell.

### Interpretation Work
- I(guiding, data, L): Step 1 axis anchor = guiding data; Step 2 projected contributors = boundary prerequisite, essential fact, result source; Step 3 centroid attractor = `source discipline`.
- I(guiding, information, L): Step 1 axis anchor = guiding information; Step 2 projected contributors = basis adequacy, adequate context, warning meaning; Step 3 centroid attractor = `context discipline`.
- I(guiding, knowledge, L): Step 1 axis anchor = guiding knowledge; Step 2 projected contributors = category coverage, competent expertise, architecture basis; Step 3 centroid attractor = `contract discipline`.
- I(guiding, wisdom, L): Step 1 axis anchor = guiding wisdom; Step 2 projected contributors = status coherence, principled reasoning, human boundary; Step 3 centroid attractor = `judgment discipline`.
- I(applying, data, L): Step 1 axis anchor = applying data; Step 2 projected contributors = envelope prerequisite, essential fact, value presence; Step 3 centroid attractor = `envelope discipline`.
- I(applying, information, L): Step 1 axis anchor = applying information; Step 2 projected contributors = surface adequacy, adequate context, view state; Step 3 centroid attractor = `workflow discipline`.
- I(applying, knowledge, L): Step 1 axis anchor = applying knowledge; Step 2 projected contributors = panel coverage, thorough mastery, category control; Step 3 centroid attractor = `display discipline`.
- I(applying, wisdom, L): Step 1 axis anchor = applying wisdom; Step 2 projected contributors = workflow coherence, adequate judgment, use pattern; Step 3 centroid attractor = `practice discipline`.
- I(judging, data, L): Step 1 axis anchor = judging data; Step 2 projected contributors = review prerequisite, essential signal, status datum; Step 3 centroid attractor = `evidence review`.
- I(judging, information, L): Step 1 axis anchor = judging information; Step 2 projected contributors = evidence adequacy, coherent message, warning context; Step 3 centroid attractor = `status review`.
- I(judging, knowledge, L): Step 1 axis anchor = judging knowledge; Step 2 projected contributors = assessment coverage, coherent understanding, result categories; Step 3 centroid attractor = `category review`.
- I(judging, wisdom, L): Step 1 axis anchor = judging wisdom; Step 2 projected contributors = decision coherence, principled reasoning, professional notice; Step 3 centroid attractor = `decision review`.
- I(reviewing, data, L): Step 1 axis anchor = reviewing data; Step 2 projected contributors = audit prerequisite, reliable measurement, run record; Step 3 centroid attractor = `record audit`.
- I(reviewing, information, L): Step 1 axis anchor = reviewing information; Step 2 projected contributors = record adequacy, coherent message, export signal; Step 3 centroid attractor = `export audit`.
- I(reviewing, knowledge, L): Step 1 axis anchor = reviewing knowledge; Step 2 projected contributors = trace coverage, coherent understanding, dependency edge; Step 3 centroid attractor = `trace audit`.
- I(reviewing, wisdom, L): Step 1 axis anchor = reviewing wisdom; Step 2 projected contributors = report coherence, principled reasoning, IP boundary; Step 3 centroid attractor = `boundary audit`.

### Result
| | **data** | **information** | **knowledge** | **wisdom** |
|---|---|---|---|---|
| **guiding** | source discipline | context discipline | contract discipline | judgment discipline |
| **applying** | envelope discipline | workflow discipline | display discipline | practice discipline |
| **judging** | evidence review | status review | category review | decision review |
| **reviewing** | record audit | export audit | trace audit | boundary audit |

---

## Matrix Summary

### Matrix C
| | **necessity** | **sufficiency** | **completeness** | **consistency** |
|---|---|---|---|---|
| **normative** | review boundary | evidence threshold | coverage scope | terminology alignment |
| **operative** | input prerequisite | display method | panel coverage | workflow coherence |
| **evaluative** | review criterion | trace evidence | quality coverage | decision coherence |

### Matrix F
| | **necessity** | **sufficiency** | **completeness** | **consistency** |
|---|---|---|---|---|
| **normative** | required result boundary | adequate source basis | complete category set | consistent status model |
| **operative** | required envelope input | adequate review surface | complete panel set | consistent interaction route |
| **evaluative** | required review evidence | adequate audit basis | complete export signal | consistent decision trail |

### Matrix D
| | **necessity** | **sufficiency** | **completeness** | **consistency** |
|---|---|---|---|---|
| **normative** | boundary closure | basis ruling | category mandate | status governance |
| **operative** | envelope closure | usable method | panel procedure | review workflow |
| **evaluative** | review closure | evidence sufficiency | export assessment | decision consistency |

### Matrix K
| | **normative** | **operative** | **evaluative** |
|---|---|---|---|
| **necessity** | boundary closure | envelope closure | review closure |
| **sufficiency** | basis ruling | usable method | evidence sufficiency |
| **completeness** | category mandate | panel procedure | export assessment |
| **consistency** | status governance | review workflow | decision consistency |

### Matrix G
| | **necessity** | **sufficiency** | **completeness** | **consistency** |
|---|---|---|---|---|
| **data** | essential fact | adequate evidence | comprehensive record | reliable measurement |
| **information** | essential signal | adequate context | comprehensive account | coherent message |
| **knowledge** | fundamental understanding | competent expertise | thorough mastery | coherent understanding |

### Matrix X
| | **necessity** | **sufficiency** | **completeness** | **consistency** |
|---|---|---|---|---|
| **guiding** | boundary prerequisite | basis adequacy | category coverage | status coherence |
| **applying** | envelope prerequisite | surface adequacy | panel coverage | workflow coherence |
| **judging** | review prerequisite | evidence adequacy | assessment coverage | decision coherence |
| **reviewing** | audit prerequisite | record adequacy | trace coverage | report coherence |

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
| **guiding** | source discipline | context discipline | contract discipline | judgment discipline |
| **applying** | envelope discipline | workflow discipline | display discipline | practice discipline |
| **judging** | evidence review | status review | category review | decision review |
| **reviewing** | record audit | export audit | trace audit | boundary audit |
