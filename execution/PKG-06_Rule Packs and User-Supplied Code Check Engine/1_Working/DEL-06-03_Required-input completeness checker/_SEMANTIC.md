# Semantic Lens: DEL-06-03 Required-input completeness checker

**Generated:** 2026-04-30
**DECOMP_VARIANT:** SOFTWARE
**Perspective:** This deliverable defines the checker boundary that turns rule-pack required-input declarations into explicit missing-data findings and rule-check gating. It must preserve user-supplied code data, protected-data, and professional-responsibility boundaries while remaining implementation-ready for future backend work.
**Framework:** Chirality Semantic Algebra
**Inputs Read:**
- `_CONTEXT.md` - deliverable identity, SOW-004, OBJ-002, OBJ-005
- `_STATUS.md` - setup lifecycle state
- `Datasheet.md` - setup facts and boundaries
- `Specification.md` - requirements and verification hooks
- `Guidance.md` - principles and conflicts
- `Procedure.md` - prerequisites and records
- `_REFERENCES.md` - governing source pointers

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

### Construction: Dot product A and B

Each cell used the three-step interpretation operator. Compact work records list the axis anchor, projected contributors, and centroid.

| Cell | Axis anchor | Projected contributors | Centroid |
|---|---|---|---|
| C[normative,necessity] | binding need | mandatory datum; required signal; controlling understanding; governing discernment | binding input threshold |
| C[normative,sufficiency] | binding adequacy | supported rule fact; contextual proof; competent basis; judged support | evidenced rule basis |
| C[normative,completeness] | binding coverage | full public record; complete account; mastered basis; holistic guardrail | full governance record |
| C[normative,consistency] | binding alignment | reliable measure; coherent signal; stable understanding; principled reason | traceable rule alignment |
| C[operative,necessity] | execution need | required task fact; necessary workflow signal; practical understanding; action discernment | required execution input |
| C[operative,sufficiency] | execution adequacy | usable proof; workable context; competent practice; practical judgment | usable evidence basis |
| C[operative,completeness] | execution coverage | complete task record; comprehensive workflow account; thorough practice; holistic process | full workflow record |
| C[operative,consistency] | execution alignment | reliable process measure; coherent task message; stable practice; principled execution | stable implementation signal |
| C[evaluative,necessity] | review need | required finding fact; necessary assessment signal; evaluable understanding; review discernment | review trigger basis |
| C[evaluative,sufficiency] | review adequacy | adequate finding proof; contextual assessment; competent review basis; judgment support | judgment evidence basis |
| C[evaluative,completeness] | review coverage | full finding record; comprehensive assessment account; thorough review basis; holistic appraisal | full assessment record |
| C[evaluative,consistency] | review alignment | reliable finding measure; coherent assessment message; stable review understanding; principled appraisal | coherent review finding |

### Result

| | **necessity** | **sufficiency** | **completeness** | **consistency** |
|---|---|---|---|---|
| **normative** | binding input threshold | evidenced rule basis | full governance record | traceable rule alignment |
| **operative** | required execution input | usable evidence basis | full workflow record | stable implementation signal |
| **evaluative** | review trigger basis | judgment evidence basis | full assessment record | coherent review finding |

## Matrix F - Requirements (3x4)

### Construction: Dot product C and B

| Cell | Axis anchor | Projected contributors | Centroid |
|---|---|---|---|
| F[normative,necessity] | binding need | threshold fact; rule signal; governance understanding; trace discernment | mandatory input gate |
| F[normative,sufficiency] | binding adequacy | threshold evidence; rule context; governance expertise; trace judgment | verified basis gate |
| F[normative,completeness] | binding coverage | threshold record; rule account; governance mastery; trace insight | full record gate |
| F[normative,consistency] | binding alignment | threshold measure; rule message; governance understanding; trace reasoning | traceable rule gate |
| F[operative,necessity] | execution need | task input fact; evidence signal; workflow understanding; implementation discernment | execution readiness gate |
| F[operative,sufficiency] | execution adequacy | input evidence; usable context; workflow expertise; implementation judgment | usable context gate |
| F[operative,completeness] | execution coverage | input record; workflow account; implementation mastery; process insight | workflow coverage gate |
| F[operative,consistency] | execution alignment | input measure; workflow message; implementation understanding; process reasoning | stable process gate |
| F[evaluative,necessity] | review need | trigger fact; judgment signal; assessment understanding; finding discernment | review readiness gate |
| F[evaluative,sufficiency] | review adequacy | trigger evidence; judgment context; assessment expertise; finding judgment | judgment basis gate |
| F[evaluative,completeness] | review coverage | trigger record; assessment account; finding mastery; appraisal insight | assessment coverage gate |
| F[evaluative,consistency] | review alignment | trigger measure; assessment message; finding understanding; appraisal reasoning | finding trace gate |

### Result

| | **necessity** | **sufficiency** | **completeness** | **consistency** |
|---|---|---|---|---|
| **normative** | mandatory input gate | verified basis gate | full record gate | traceable rule gate |
| **operative** | execution readiness gate | usable context gate | workflow coverage gate | stable process gate |
| **evaluative** | review readiness gate | judgment basis gate | assessment coverage gate | finding trace gate |

## Matrix D - Objectives (3x4)

### Construction: A plus resolved F

| Cell | Axis anchor | Projected contributors | Centroid |
|---|---|---|---|
| D[normative,guiding] | binding direction | prescriptive route; resolved input gate | source-bound direction |
| D[normative,applying] | binding practice | mandatory practice; resolved basis gate | required gate practice |
| D[normative,judging] | binding decision | compliance boundary; resolved record gate | status decision boundary |
| D[normative,reviewing] | binding inspection | audit route; resolved trace gate | audit trail boundary |
| D[operative,guiding] | execution direction | procedural route; resolved readiness gate | workflow direction |
| D[operative,applying] | execution practice | practical execution; resolved context gate | executable gating path |
| D[operative,judging] | execution decision | performance assessment; resolved coverage gate | readiness assessment path |
| D[operative,reviewing] | execution inspection | process audit; resolved process gate | process evidence audit |
| D[evaluative,guiding] | review direction | value route; resolved readiness gate | review framing basis |
| D[evaluative,applying] | review practice | merit application; resolved judgment gate | judgment application path |
| D[evaluative,judging] | review decision | worth determination; resolved coverage gate | finding decision boundary |
| D[evaluative,reviewing] | review inspection | quality appraisal; resolved trace gate | quality review basis |

### Result

| | **guiding** | **applying** | **judging** | **reviewing** |
|---|---|---|---|---|
| **normative** | source-bound direction | required gate practice | status decision boundary | audit trail boundary |
| **operative** | workflow direction | executable gating path | readiness assessment path | process evidence audit |
| **evaluative** | review framing basis | judgment application path | finding decision boundary | quality review basis |

## Matrix K - Transpose of D

| | **normative** | **operative** | **evaluative** |
|---|---|---|---|
| **guiding** | source-bound direction | workflow direction | review framing basis |
| **applying** | required gate practice | executable gating path | judgment application path |
| **judging** | status decision boundary | readiness assessment path | finding decision boundary |
| **reviewing** | audit trail boundary | process evidence audit | quality review basis |

## Matrix G - Truncation of B

| | **necessity** | **sufficiency** | **completeness** | **consistency** |
|---|---|---|---|---|
| **data** | essential fact | adequate evidence | comprehensive record | reliable measurement |
| **information** | essential signal | adequate context | comprehensive account | coherent message |
| **knowledge** | fundamental understanding | competent expertise | thorough mastery | coherent understanding |

## Matrix X - Verification (4x4)

### Construction: Dot product K and G

| Cell | Axis anchor | Projected contributors | Centroid |
|---|---|---|---|
| X[guiding,necessity] | direction need | source fact; workflow signal; review understanding | governing input proof |
| X[guiding,sufficiency] | direction adequacy | source evidence; workflow context; review expertise | direction evidence proof |
| X[guiding,completeness] | direction coverage | source record; workflow account; review mastery | full guidance trace |
| X[guiding,consistency] | direction alignment | source measure; workflow message; review understanding | stable guidance record |
| X[applying,necessity] | practice need | gate fact; execution signal; judgment understanding | implementation input proof |
| X[applying,sufficiency] | practice adequacy | gate evidence; execution context; judgment expertise | execution evidence proof |
| X[applying,completeness] | practice coverage | gate record; execution account; judgment mastery | full application trace |
| X[applying,consistency] | practice alignment | gate measure; execution message; judgment understanding | stable application record |
| X[judging,necessity] | decision need | status fact; readiness signal; finding understanding | decision input proof |
| X[judging,sufficiency] | decision adequacy | status evidence; readiness context; finding expertise | assessment evidence proof |
| X[judging,completeness] | decision coverage | status record; readiness account; finding mastery | full decision trace |
| X[judging,consistency] | decision alignment | status measure; readiness message; finding understanding | stable decision record |
| X[reviewing,necessity] | inspection need | audit fact; process signal; quality understanding | audit input proof |
| X[reviewing,sufficiency] | inspection adequacy | audit evidence; process context; quality expertise | inspection evidence proof |
| X[reviewing,completeness] | inspection coverage | audit record; process account; quality mastery | full audit trace |
| X[reviewing,consistency] | inspection alignment | audit measure; process message; quality understanding | stable inspection record |

### Result

| | **necessity** | **sufficiency** | **completeness** | **consistency** |
|---|---|---|---|---|
| **guiding** | governing input proof | direction evidence proof | full guidance trace | stable guidance record |
| **applying** | implementation input proof | execution evidence proof | full application trace | stable application record |
| **judging** | decision input proof | assessment evidence proof | full decision trace | stable decision record |
| **reviewing** | audit input proof | inspection evidence proof | full audit trace | stable inspection record |

## Matrix T - Transpose of B

| | **data** | **information** | **knowledge** | **wisdom** |
|---|---|---|---|---|
| **necessity** | essential fact | essential signal | fundamental understanding | essential discernment |
| **sufficiency** | adequate evidence | adequate context | competent expertise | adequate judgment |
| **completeness** | comprehensive record | comprehensive account | thorough mastery | holistic insight |
| **consistency** | reliable measurement | coherent message | coherent understanding | principled reasoning |

## Matrix E - Evaluation (4x4)

### Construction: Dot product X and T

| Cell | Axis anchor | Projected contributors | Centroid |
|---|---|---|---|
| E[guiding,data] | direction input | proof fact; evidence evidence; trace record; record measure | governed input notice |
| E[guiding,information] | direction context | proof signal; evidence context; trace account; record message | guided context notice |
| E[guiding,knowledge] | direction understanding | proof understanding; evidence expertise; trace mastery; record coherence | traced understanding notice |
| E[guiding,wisdom] | direction judgment | proof discernment; evidence judgment; trace insight; record reasoning | reliance boundary notice |
| E[applying,data] | practice input | proof fact; execution evidence; application record; stable measure | execution input notice |
| E[applying,information] | practice context | proof signal; execution context; application account; stable message | application context notice |
| E[applying,knowledge] | practice understanding | proof understanding; execution expertise; application mastery; stable coherence | implementation understanding notice |
| E[applying,wisdom] | practice judgment | proof discernment; execution judgment; application insight; stable reasoning | use boundary notice |
| E[judging,data] | decision input | decision fact; assessment evidence; decision record; stable measure | decision input finding |
| E[judging,information] | decision context | decision signal; assessment context; decision account; stable message | assessment context finding |
| E[judging,knowledge] | decision understanding | decision understanding; assessment expertise; decision mastery; stable coherence | evaluation understanding finding |
| E[judging,wisdom] | decision judgment | decision discernment; assessment judgment; decision insight; stable reasoning | professional boundary finding |
| E[reviewing,data] | inspection input | audit fact; inspection evidence; audit record; stable measure | audit input record |
| E[reviewing,information] | inspection context | audit signal; inspection context; audit account; stable message | audit context record |
| E[reviewing,knowledge] | inspection understanding | audit understanding; inspection expertise; audit mastery; stable coherence | quality understanding record |
| E[reviewing,wisdom] | inspection judgment | audit discernment; inspection judgment; audit insight; stable reasoning | acceptance boundary record |

### Result

| | **data** | **information** | **knowledge** | **wisdom** |
|---|---|---|---|---|
| **guiding** | governed input notice | guided context notice | traced understanding notice | reliance boundary notice |
| **applying** | execution input notice | application context notice | implementation understanding notice | use boundary notice |
| **judging** | decision input finding | assessment context finding | evaluation understanding finding | professional boundary finding |
| **reviewing** | audit input record | audit context record | quality understanding record | acceptance boundary record |

## Audit Result

PASS. Final result cells are populated, compact, and free of unresolved algebra operators. The lens remains a setup aid for question shaping and is not engineering authority.
