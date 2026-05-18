# Semantic Lens: DEL-08-01 Calculation report generator

**Generated:** 2026-04-30
**DECOMP_VARIANT:** SOFTWARE
**Perspective:** This deliverable defines the report-generation knowledge needed to assemble auditable calculation reports from model inputs, result envelopes, diagnostics, provenance records, and rule-pack references. It carries boundaries for reproducibility, protected-content exclusion, and professional-review notices without specifying renderer code or engineering particulars.
**Framework:** Chirality Semantic Algebra
**Audit Result:** PASS - final result cells contain no algebra leaks, no operator leaks, and no cell value over 80 characters.

**Inputs Read:**
- `_CONTEXT.md` - deliverable identity, scope coverage, objective support, architecture basis injection
- `_STATUS.md` - lifecycle state
- `Datasheet.md` - setup attributes and boundaries
- `Specification.md` - requirements and verification targets
- `Guidance.md` - principles, trade-offs, and open questions
- `Procedure.md` - setup and future implementation workflow
- `_REFERENCES.md` - governing references and register pointers

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

### Construction: Dot product A * B

For every cell, `L_C(i,j)` is the ordered collection of four products from row `A(i,*)` and column `B(*,j)`, then `C(i,j) = I(row_i, col_j, L_C(i,j))`.

### Interpretation Work

| Cell | Step 1 - Axis anchor | Step 2 - Projections | Step 3 - Centroid |
|---|---|---|---|
| C[normative, necessity] | `normative * necessity = binding need frame` | `p1 = frame * essential fact = required report fact`; `p2 = frame * essential signal = required report signal`; `p3 = frame * fundamental understanding = required report basis`; `p4 = frame * essential discernment = required boundary signal` | mandatory report basis |
| C[normative, sufficiency] | `normative * sufficiency = binding adequacy frame` | `p1 = frame * adequate evidence = evidence floor`; `p2 = frame * adequate context = context floor`; `p3 = frame * competent expertise = reviewable basis`; `p4 = frame * adequate judgment = boundary judgment` | evidence threshold |
| C[normative, completeness] | `normative * completeness = binding coverage frame` | `p1 = frame * comprehensive record = full trace`; `p2 = frame * comprehensive account = full narrative`; `p3 = frame * thorough mastery = full control`; `p4 = frame * holistic insight = full limitation view` | full trace record |
| C[normative, consistency] | `normative * consistency = binding alignment frame` | `p1 = frame * reliable measurement = aligned value`; `p2 = frame * coherent message = aligned statement`; `p3 = frame * coherent understanding = aligned rationale`; `p4 = frame * principled reasoning = aligned boundary` | aligned claim framing |
| C[operative, necessity] | `operative * necessity = execution need frame` | `p1 = frame * essential fact = input fact`; `p2 = frame * essential signal = input signal`; `p3 = frame * fundamental understanding = input method`; `p4 = frame * essential discernment = input boundary` | required input flow |
| C[operative, sufficiency] | `operative * sufficiency = execution adequacy frame` | `p1 = frame * adequate evidence = workable evidence`; `p2 = frame * adequate context = workable context`; `p3 = frame * competent expertise = workable method`; `p4 = frame * adequate judgment = workable review` | workable generation path |
| C[operative, completeness] | `operative * completeness = execution coverage frame` | `p1 = frame * comprehensive record = artifact coverage`; `p2 = frame * comprehensive account = section coverage`; `p3 = frame * thorough mastery = workflow coverage`; `p4 = frame * holistic insight = limitation coverage` | artifact coverage map |
| C[operative, consistency] | `operative * consistency = execution alignment frame` | `p1 = frame * reliable measurement = stable value`; `p2 = frame * coherent message = stable message`; `p3 = frame * coherent understanding = stable method`; `p4 = frame * principled reasoning = stable boundary` | stable rendering contract |
| C[evaluative, necessity] | `evaluative * necessity = review need frame` | `p1 = frame * essential fact = review fact`; `p2 = frame * essential signal = review signal`; `p3 = frame * fundamental understanding = review basis`; `p4 = frame * essential discernment = review boundary` | review basis signal |
| C[evaluative, sufficiency] | `evaluative * sufficiency = review adequacy frame` | `p1 = frame * adequate evidence = audit evidence`; `p2 = frame * adequate context = audit context`; `p3 = frame * competent expertise = audit method`; `p4 = frame * adequate judgment = audit judgment` | audit evidence threshold |
| C[evaluative, completeness] | `evaluative * completeness = review coverage frame` | `p1 = frame * comprehensive record = review record`; `p2 = frame * comprehensive account = review account`; `p3 = frame * thorough mastery = review scope`; `p4 = frame * holistic insight = review limitation view` | full review record |
| C[evaluative, consistency] | `evaluative * consistency = review alignment frame` | `p1 = frame * reliable measurement = review alignment`; `p2 = frame * coherent message = review messaging`; `p3 = frame * coherent understanding = review rationale`; `p4 = frame * principled reasoning = review boundary` | aligned review criteria |

### Result

| | **necessity** | **sufficiency** | **completeness** | **consistency** |
|---|---|---|---|---|
| **normative** | mandatory report basis | evidence threshold | full trace record | aligned claim framing |
| **operative** | required input flow | workable generation path | artifact coverage map | stable rendering contract |
| **evaluative** | review basis signal | audit evidence threshold | full review record | aligned review criteria |

## Matrix F - Requirements (3x4)

### Construction: Dot product C * B

For every cell, `L_F(i,j)` is the ordered collection of four products from row `C(i,*)` and column `B(*,j)`, then `F(i,j) = I(row_i, col_j, L_F(i,j))`.

### Interpretation Work

| Cell | Step 1 - Axis anchor | Step 2 - Projections | Step 3 - Centroid |
|---|---|---|---|
| F[normative, necessity] | `normative * necessity = binding duty frame` | `p1 = frame * mandatory report basis = report duty`; `p2 = frame * evidence threshold = evidence duty`; `p3 = frame * full trace record = trace duty`; `p4 = frame * aligned claim framing = claim duty` | bounded report duties |
| F[normative, sufficiency] | `normative * sufficiency = binding evidence frame` | `p1 = frame * mandatory report basis = basis proof`; `p2 = frame * evidence threshold = source proof`; `p3 = frame * full trace record = trace proof`; `p4 = frame * aligned claim framing = claim proof` | source evidence rules |
| F[normative, completeness] | `normative * completeness = binding inclusion frame` | `p1 = frame * mandatory report basis = required inclusion`; `p2 = frame * evidence threshold = evidence inclusion`; `p3 = frame * full trace record = record inclusion`; `p4 = frame * aligned claim framing = claim inclusion` | artifact inclusion controls |
| F[normative, consistency] | `normative * consistency = binding alignment frame` | `p1 = frame * mandatory report basis = basis alignment`; `p2 = frame * evidence threshold = source alignment`; `p3 = frame * full trace record = record alignment`; `p4 = frame * aligned claim framing = claim alignment` | claim alignment rules |
| F[operative, necessity] | `operative * necessity = workflow duty frame` | `p1 = frame * required input flow = input duty`; `p2 = frame * workable generation path = path duty`; `p3 = frame * artifact coverage map = coverage duty`; `p4 = frame * stable rendering contract = contract duty` | input assembly duties |
| F[operative, sufficiency] | `operative * sufficiency = workflow adequacy frame` | `p1 = frame * required input flow = input adequacy`; `p2 = frame * workable generation path = path adequacy`; `p3 = frame * artifact coverage map = coverage adequacy`; `p4 = frame * stable rendering contract = handoff adequacy` | renderer handoff rules |
| F[operative, completeness] | `operative * completeness = workflow coverage frame` | `p1 = frame * required input flow = input coverage`; `p2 = frame * workable generation path = path coverage`; `p3 = frame * artifact coverage map = output coverage`; `p4 = frame * stable rendering contract = contract coverage` | generation coverage controls |
| F[operative, consistency] | `operative * consistency = workflow alignment frame` | `p1 = frame * required input flow = input stability`; `p2 = frame * workable generation path = path stability`; `p3 = frame * artifact coverage map = coverage stability`; `p4 = frame * stable rendering contract = output stability` | repeatable output contract |
| F[evaluative, necessity] | `evaluative * necessity = review duty frame` | `p1 = frame * review basis signal = basis duty`; `p2 = frame * audit evidence threshold = evidence duty`; `p3 = frame * full review record = record duty`; `p4 = frame * aligned review criteria = criteria duty` | review evidence rules |
| F[evaluative, sufficiency] | `evaluative * sufficiency = review adequacy frame` | `p1 = frame * review basis signal = basis adequacy`; `p2 = frame * audit evidence threshold = audit adequacy`; `p3 = frame * full review record = record adequacy`; `p4 = frame * aligned review criteria = criteria adequacy` | audit trail controls |
| F[evaluative, completeness] | `evaluative * completeness = review coverage frame` | `p1 = frame * review basis signal = basis coverage`; `p2 = frame * audit evidence threshold = evidence coverage`; `p3 = frame * full review record = finding coverage`; `p4 = frame * aligned review criteria = criteria coverage` | finding coverage controls |
| F[evaluative, consistency] | `evaluative * consistency = review alignment frame` | `p1 = frame * review basis signal = basis alignment`; `p2 = frame * audit evidence threshold = evidence alignment`; `p3 = frame * full review record = record alignment`; `p4 = frame * aligned review criteria = boundary alignment` | review boundary rules |

### Result

| | **necessity** | **sufficiency** | **completeness** | **consistency** |
|---|---|---|---|---|
| **normative** | bounded report duties | source evidence rules | artifact inclusion controls | claim alignment rules |
| **operative** | input assembly duties | renderer handoff rules | generation coverage controls | repeatable output contract |
| **evaluative** | review evidence rules | audit trail controls | finding coverage controls | review boundary rules |

## Matrix D - Objectives (3x4)

### Construction: A plus resolution * F

For every cell, `L_D(i,j)` contains the matching Matrix A cell and the resolved Matrix F cell, then `D(i,j) = I(row_i, col_j, L_D(i,j))`.

### Interpretation Work

| Cell | Step 1 - Axis anchor | Step 2 - Projections | Step 3 - Centroid |
|---|---|---|---|
| D[normative, guiding] | `normative * guiding = policy frame` | `p1 = frame * prescriptive direction = policy direction`; `p2 = frame * bounded report duties = report closure` | report policy closure |
| D[normative, applying] | `normative * applying = template frame` | `p1 = frame * mandatory practice = template practice`; `p2 = frame * source evidence rules = source control` | template control closure |
| D[normative, judging] | `normative * judging = boundary frame` | `p1 = frame * compliance determination = prohibited overclaim`; `p2 = frame * artifact inclusion controls = inclusion boundary` | claim boundary closure |
| D[normative, reviewing] | `normative * reviewing = audit frame` | `p1 = frame * regulatory audit = audit notice`; `p2 = frame * claim alignment rules = notice alignment` | audit notice closure |
| D[operative, guiding] | `operative * guiding = workflow frame` | `p1 = frame * procedural direction = generation path`; `p2 = frame * input assembly duties = input closure` | generation workflow closure |
| D[operative, applying] | `operative * applying = execution frame` | `p1 = frame * practical execution = renderer path`; `p2 = frame * renderer handoff rules = handoff closure` | render handoff closure |
| D[operative, judging] | `operative * judging = result frame` | `p1 = frame * performance assessment = result measure`; `p2 = frame * generation coverage controls = result coverage` | result assembly closure |
| D[operative, reviewing] | `operative * reviewing = run frame` | `p1 = frame * process audit = run evidence`; `p2 = frame * repeatable output contract = repeatability` | reproducible run closure |
| D[evaluative, guiding] | `evaluative * guiding = support frame` | `p1 = frame * value orientation = review support`; `p2 = frame * review evidence rules = evidence support` | review support closure |
| D[evaluative, applying] | `evaluative * applying = appraisal frame` | `p1 = frame * merit application = evidence appraisal`; `p2 = frame * audit trail controls = trail appraisal` | evidence appraisal closure |
| D[evaluative, judging] | `evaluative * judging = triage frame` | `p1 = frame * worth determination = finding value`; `p2 = frame * finding coverage controls = finding coverage` | finding triage closure |
| D[evaluative, reviewing] | `evaluative * reviewing = acceptance frame` | `p1 = frame * quality appraisal = review quality`; `p2 = frame * review boundary rules = human boundary` | human review closure |

### Result

| | **guiding** | **applying** | **judging** | **reviewing** |
|---|---|---|---|---|
| **normative** | report policy closure | template control closure | claim boundary closure | audit notice closure |
| **operative** | generation workflow closure | render handoff closure | result assembly closure | reproducible run closure |
| **evaluative** | review support closure | evidence appraisal closure | finding triage closure | human review closure |

## Matrix K - Transpose of D (4x3)

### Construction

`K(i,j) = D(j,i)`.

### Result

| | **normative** | **operative** | **evaluative** |
|---|---|---|---|
| **guiding** | report policy closure | generation workflow closure | review support closure |
| **applying** | template control closure | render handoff closure | evidence appraisal closure |
| **judging** | claim boundary closure | result assembly closure | finding triage closure |
| **reviewing** | audit notice closure | reproducible run closure | human review closure |

## Matrix G - Truncation of B (3x4)

### Construction

Matrix G removes the `wisdom` row from canonical Matrix B.

### Result

| | **necessity** | **sufficiency** | **completeness** | **consistency** |
|---|---|---|---|---|
| **data** | essential fact | adequate evidence | comprehensive record | reliable measurement |
| **information** | essential signal | adequate context | comprehensive account | coherent message |
| **knowledge** | fundamental understanding | competent expertise | thorough mastery | coherent understanding |

## Matrix X - Verification (4x4)

### Construction: Dot product K * G

For every cell, `L_X(i,j)` is the ordered collection of three products from row `K(i,*)` and column `G(*,j)`, then `X(i,j) = I(row_i, col_j, L_X(i,j))`.

### Interpretation Work

| Cell | Step 1 - Axis anchor | Step 2 - Projections | Step 3 - Centroid |
|---|---|---|---|
| X[guiding, necessity] | `guiding * necessity = direction need frame` | `p1 = frame * report policy closure = policy need`; `p2 = frame * generation workflow closure = workflow need`; `p3 = frame * review support closure = support need` | policy evidence check |
| X[guiding, sufficiency] | `guiding * sufficiency = direction adequacy frame` | `p1 = frame * report policy closure = policy evidence`; `p2 = frame * generation workflow closure = workflow evidence`; `p3 = frame * review support closure = support evidence` | source threshold check |
| X[guiding, completeness] | `guiding * completeness = direction coverage frame` | `p1 = frame * report policy closure = policy coverage`; `p2 = frame * generation workflow closure = workflow coverage`; `p3 = frame * review support closure = support coverage` | coverage trace check |
| X[guiding, consistency] | `guiding * consistency = direction alignment frame` | `p1 = frame * report policy closure = policy alignment`; `p2 = frame * generation workflow closure = workflow alignment`; `p3 = frame * review support closure = support alignment` | alignment check |
| X[applying, necessity] | `applying * necessity = use need frame` | `p1 = frame * template control closure = template need`; `p2 = frame * render handoff closure = handoff need`; `p3 = frame * evidence appraisal closure = appraisal need` | workflow input check |
| X[applying, sufficiency] | `applying * sufficiency = use adequacy frame` | `p1 = frame * template control closure = template adequacy`; `p2 = frame * render handoff closure = handoff adequacy`; `p3 = frame * evidence appraisal closure = appraisal adequacy` | handoff adequacy check |
| X[applying, completeness] | `applying * completeness = use coverage frame` | `p1 = frame * template control closure = template coverage`; `p2 = frame * render handoff closure = handoff coverage`; `p3 = frame * evidence appraisal closure = appraisal coverage` | output coverage check |
| X[applying, consistency] | `applying * consistency = use alignment frame` | `p1 = frame * template control closure = template alignment`; `p2 = frame * render handoff closure = handoff alignment`; `p3 = frame * evidence appraisal closure = appraisal alignment` | repeatability check |
| X[judging, necessity] | `judging * necessity = decision need frame` | `p1 = frame * claim boundary closure = boundary need`; `p2 = frame * result assembly closure = result need`; `p3 = frame * finding triage closure = finding need` | boundary evidence check |
| X[judging, sufficiency] | `judging * sufficiency = decision adequacy frame` | `p1 = frame * claim boundary closure = boundary adequacy`; `p2 = frame * result assembly closure = result adequacy`; `p3 = frame * finding triage closure = finding adequacy` | result adequacy check |
| X[judging, completeness] | `judging * completeness = decision coverage frame` | `p1 = frame * claim boundary closure = boundary coverage`; `p2 = frame * result assembly closure = result coverage`; `p3 = frame * finding triage closure = finding coverage` | finding coverage check |
| X[judging, consistency] | `judging * consistency = decision alignment frame` | `p1 = frame * claim boundary closure = boundary alignment`; `p2 = frame * result assembly closure = result alignment`; `p3 = frame * finding triage closure = finding alignment` | status coherence check |
| X[reviewing, necessity] | `reviewing * necessity = audit need frame` | `p1 = frame * audit notice closure = notice need`; `p2 = frame * reproducible run closure = run need`; `p3 = frame * human review closure = human need` | audit input check |
| X[reviewing, sufficiency] | `reviewing * sufficiency = audit adequacy frame` | `p1 = frame * audit notice closure = notice adequacy`; `p2 = frame * reproducible run closure = run adequacy`; `p3 = frame * human review closure = human adequacy` | record adequacy check |
| X[reviewing, completeness] | `reviewing * completeness = audit coverage frame` | `p1 = frame * audit notice closure = notice coverage`; `p2 = frame * reproducible run closure = run coverage`; `p3 = frame * human review closure = human coverage` | manifest coverage check |
| X[reviewing, consistency] | `reviewing * consistency = audit alignment frame` | `p1 = frame * audit notice closure = notice alignment`; `p2 = frame * reproducible run closure = run alignment`; `p3 = frame * human review closure = human alignment` | review notice check |

### Result

| | **necessity** | **sufficiency** | **completeness** | **consistency** |
|---|---|---|---|---|
| **guiding** | policy evidence check | source threshold check | coverage trace check | alignment check |
| **applying** | workflow input check | handoff adequacy check | output coverage check | repeatability check |
| **judging** | boundary evidence check | result adequacy check | finding coverage check | status coherence check |
| **reviewing** | audit input check | record adequacy check | manifest coverage check | review notice check |

## Matrix T - Transpose of B (4x4)

### Construction

`T(i,j) = B(j,i)`.

### Result

| | **data** | **information** | **knowledge** | **wisdom** |
|---|---|---|---|---|
| **necessity** | essential fact | essential signal | fundamental understanding | essential discernment |
| **sufficiency** | adequate evidence | adequate context | competent expertise | adequate judgment |
| **completeness** | comprehensive record | comprehensive account | thorough mastery | holistic insight |
| **consistency** | reliable measurement | coherent message | coherent understanding | principled reasoning |

## Matrix E - Evaluation (4x4)

### Construction: Dot product X * T

For every cell, `L_E(i,j)` is the ordered collection of four products from row `X(i,*)` and column `T(*,j)`, then `E(i,j) = I(row_i, col_j, L_E(i,j))`.

### Interpretation Work

| Cell | Step 1 - Axis anchor | Step 2 - Projections | Step 3 - Centroid |
|---|---|---|---|
| E[guiding, data] | `guiding * data = direction fact frame` | `p1 = frame * policy evidence check = policy fact`; `p2 = frame * source threshold check = source fact`; `p3 = frame * coverage trace check = trace fact`; `p4 = frame * alignment check = alignment fact` | policy facts trail |
| E[guiding, information] | `guiding * information = direction context frame` | `p1 = frame * policy evidence check = policy context`; `p2 = frame * source threshold check = source context`; `p3 = frame * coverage trace check = trace context`; `p4 = frame * alignment check = notice context` | context notice trail |
| E[guiding, knowledge] | `guiding * knowledge = direction rationale frame` | `p1 = frame * policy evidence check = policy rationale`; `p2 = frame * source threshold check = source rationale`; `p3 = frame * coverage trace check = trace rationale`; `p4 = frame * alignment check = guardrail rationale` | rationale guardrail trail |
| E[guiding, wisdom] | `guiding * wisdom = direction judgment frame` | `p1 = frame * policy evidence check = policy judgment`; `p2 = frame * source threshold check = source judgment`; `p3 = frame * coverage trace check = trace judgment`; `p4 = frame * alignment check = boundary judgment` | judgment boundary trail |
| E[applying, data] | `applying * data = use fact frame` | `p1 = frame * workflow input check = input fact`; `p2 = frame * handoff adequacy check = handoff fact`; `p3 = frame * output coverage check = output fact`; `p4 = frame * repeatability check = repeatability fact` | input fact trace |
| E[applying, information] | `applying * information = use context frame` | `p1 = frame * workflow input check = input context`; `p2 = frame * handoff adequacy check = handoff context`; `p3 = frame * output coverage check = output context`; `p4 = frame * repeatability check = repeatability context` | handoff context trace |
| E[applying, knowledge] | `applying * knowledge = use rationale frame` | `p1 = frame * workflow input check = input rationale`; `p2 = frame * handoff adequacy check = handoff rationale`; `p3 = frame * output coverage check = output rationale`; `p4 = frame * repeatability check = procedure rationale` | procedure rationale trace |
| E[applying, wisdom] | `applying * wisdom = use judgment frame` | `p1 = frame * workflow input check = input judgment`; `p2 = frame * handoff adequacy check = handoff judgment`; `p3 = frame * output coverage check = output judgment`; `p4 = frame * repeatability check = review judgment` | review judgment trace |
| E[judging, data] | `judging * data = decision fact frame` | `p1 = frame * boundary evidence check = boundary fact`; `p2 = frame * result adequacy check = result fact`; `p3 = frame * finding coverage check = finding fact`; `p4 = frame * status coherence check = status fact` | boundary fact check |
| E[judging, information] | `judging * information = decision context frame` | `p1 = frame * boundary evidence check = boundary context`; `p2 = frame * result adequacy check = result context`; `p3 = frame * finding coverage check = finding context`; `p4 = frame * status coherence check = status context` | status context check |
| E[judging, knowledge] | `judging * knowledge = decision rationale frame` | `p1 = frame * boundary evidence check = boundary rationale`; `p2 = frame * result adequacy check = result rationale`; `p3 = frame * finding coverage check = finding rationale`; `p4 = frame * status coherence check = criteria rationale` | criteria rationale check |
| E[judging, wisdom] | `judging * wisdom = decision judgment frame` | `p1 = frame * boundary evidence check = boundary judgment`; `p2 = frame * result adequacy check = result judgment`; `p3 = frame * finding coverage check = finding judgment`; `p4 = frame * status coherence check = authority boundary` | authority boundary check |
| E[reviewing, data] | `reviewing * data = audit fact frame` | `p1 = frame * audit input check = audit fact`; `p2 = frame * record adequacy check = record fact`; `p3 = frame * manifest coverage check = manifest fact`; `p4 = frame * review notice check = notice fact` | audit fact packet |
| E[reviewing, information] | `reviewing * information = audit context frame` | `p1 = frame * audit input check = audit context`; `p2 = frame * record adequacy check = record context`; `p3 = frame * manifest coverage check = manifest context`; `p4 = frame * review notice check = notice context` | record context packet |
| E[reviewing, knowledge] | `reviewing * knowledge = audit rationale frame` | `p1 = frame * audit input check = audit rationale`; `p2 = frame * record adequacy check = record rationale`; `p3 = frame * manifest coverage check = manifest rationale`; `p4 = frame * review notice check = evidence rationale` | evidence rationale packet |
| E[reviewing, wisdom] | `reviewing * wisdom = audit judgment frame` | `p1 = frame * audit input check = audit judgment`; `p2 = frame * record adequacy check = record judgment`; `p3 = frame * manifest coverage check = manifest judgment`; `p4 = frame * review notice check = human review` | human review packet |

### Result

| | **data** | **information** | **knowledge** | **wisdom** |
|---|---|---|---|---|
| **guiding** | policy facts trail | context notice trail | rationale guardrail trail | judgment boundary trail |
| **applying** | input fact trace | handoff context trace | procedure rationale trace | review judgment trace |
| **judging** | boundary fact check | status context check | criteria rationale check | authority boundary check |
| **reviewing** | audit fact packet | record context packet | evidence rationale packet | human review packet |

## Matrix Summary

### C - Formulation

| | **necessity** | **sufficiency** | **completeness** | **consistency** |
|---|---|---|---|---|
| **normative** | mandatory report basis | evidence threshold | full trace record | aligned claim framing |
| **operative** | required input flow | workable generation path | artifact coverage map | stable rendering contract |
| **evaluative** | review basis signal | audit evidence threshold | full review record | aligned review criteria |

### F - Requirements

| | **necessity** | **sufficiency** | **completeness** | **consistency** |
|---|---|---|---|---|
| **normative** | bounded report duties | source evidence rules | artifact inclusion controls | claim alignment rules |
| **operative** | input assembly duties | renderer handoff rules | generation coverage controls | repeatable output contract |
| **evaluative** | review evidence rules | audit trail controls | finding coverage controls | review boundary rules |

### D - Objectives

| | **guiding** | **applying** | **judging** | **reviewing** |
|---|---|---|---|---|
| **normative** | report policy closure | template control closure | claim boundary closure | audit notice closure |
| **operative** | generation workflow closure | render handoff closure | result assembly closure | reproducible run closure |
| **evaluative** | review support closure | evidence appraisal closure | finding triage closure | human review closure |

### K - Transpose of D

| | **normative** | **operative** | **evaluative** |
|---|---|---|---|
| **guiding** | report policy closure | generation workflow closure | review support closure |
| **applying** | template control closure | render handoff closure | evidence appraisal closure |
| **judging** | claim boundary closure | result assembly closure | finding triage closure |
| **reviewing** | audit notice closure | reproducible run closure | human review closure |

### G - Truncation of B

| | **necessity** | **sufficiency** | **completeness** | **consistency** |
|---|---|---|---|---|
| **data** | essential fact | adequate evidence | comprehensive record | reliable measurement |
| **information** | essential signal | adequate context | comprehensive account | coherent message |
| **knowledge** | fundamental understanding | competent expertise | thorough mastery | coherent understanding |

### X - Verification

| | **necessity** | **sufficiency** | **completeness** | **consistency** |
|---|---|---|---|---|
| **guiding** | policy evidence check | source threshold check | coverage trace check | alignment check |
| **applying** | workflow input check | handoff adequacy check | output coverage check | repeatability check |
| **judging** | boundary evidence check | result adequacy check | finding coverage check | status coherence check |
| **reviewing** | audit input check | record adequacy check | manifest coverage check | review notice check |

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
| **guiding** | policy facts trail | context notice trail | rationale guardrail trail | judgment boundary trail |
| **applying** | input fact trace | handoff context trace | procedure rationale trace | review judgment trace |
| **judging** | boundary fact check | status context check | criteria rationale check | authority boundary check |
| **reviewing** | audit fact packet | record context packet | evidence rationale packet | human review packet |

