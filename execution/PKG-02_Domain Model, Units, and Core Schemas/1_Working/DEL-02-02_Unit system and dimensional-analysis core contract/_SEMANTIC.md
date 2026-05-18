# Deliverable: DEL-02-02 Unit system and dimensional-analysis core contract

**Generated:** 2026-04-30
**DECOMP_VARIANT:** SOFTWARE
**Perspective:** This deliverable frames the unit and dimensional-analysis contract as a domain-core lens for carrying explicit unit meaning through schemas, conversions, adapters, rule checks, diagnostics, and tests. It should help ask whether unit-bearing values remain traceable, deterministic, and visibly unresolved when source authority or compatibility is absent.
**Framework:** Chirality Semantic Algebra
**Authority Boundary:** This file is a semantic lens for question-shaping only. It is not an engineering authority, design approval, code-compliance statement, or source of implementation particulars.

**Inputs Read:**
- `_CONTEXT.md` - deliverable identity, scope, objectives, package exclusions, and architecture-basis injection.
- `_STATUS.md` - lifecycle state observed as `INITIALIZED` before this run.
- `Datasheet.md` - identification, attributes, conditions, construction, and references for the unit contract draft.
- `Specification.md` - scope, requirements, open contract decisions, standards, verification, and documentation obligations.
- `Guidance.md` - purpose, principles, considerations, trade-offs, examples, and conflict table.
- `Procedure.md` - prerequisites, steps, verification, and records for later implementation work.
- `_REFERENCES.md` - governing references, decomposition/register pointers, and package-specific reference notes.
- `docs/CONTRACT.md` - invariant catalog, especially unit, data-boundary, professional-boundary, and agent-discipline invariants.
- `docs/_Registers/Deliverables.csv` - row `DEL-02-02`.
- `docs/_Decomposition/SOFTWARE_DECOMP.md` - PKG-02, DEL-02-02, SOW-025, OBJ-001, and OBJ-012 context.

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

`L_C(i,j) = sum_k (A(i,k) * B(k,j))`; then `C(i,j) = I(row_i, col_j, L_C(i,j))`.

### Interpretation Work

#### C(normative, necessity)

- Intermediate collection: `{prescriptive direction * essential fact = rule fact basis; mandatory practice * essential signal = required action signal; compliance determination * fundamental understanding = conformance premise; regulatory audit * essential discernment = oversight trigger}`
- Step 1: `a = normative * necessity = binding prerequisite`
- Step 2: `a * rule fact basis = binding source fact`; `a * required action signal = obligatory trigger`; `a * conformance premise = controlled requirement`; `a * oversight trigger = enforceable review cue`
- Step 3: centroid of the projected contributors selects **Binding Unit Premise**.

#### C(normative, sufficiency)

- Intermediate collection: `{prescriptive direction * adequate evidence = rule evidence basis; mandatory practice * adequate context = required use frame; compliance determination * competent expertise = conformance judgment skill; regulatory audit * adequate judgment = oversight adequacy check}`
- Step 1: `a = normative * sufficiency = binding adequacy frame`
- Step 2: `a * rule evidence basis = bounded proof rule`; `a * required use frame = obligated practice frame`; `a * conformance judgment skill = qualified conformance basis`; `a * oversight adequacy check = audit-ready threshold`
- Step 3: centroid of the projected contributors selects **Adequacy Rule Frame**.

#### C(normative, completeness)

- Intermediate collection: `{prescriptive direction * comprehensive record = rule record span; mandatory practice * comprehensive account = required account coverage; compliance determination * thorough mastery = conformance mastery basis; regulatory audit * holistic insight = oversight coverage insight}`
- Step 1: `a = normative * completeness = binding coverage frame`
- Step 2: `a * rule record span = governed record reach`; `a * required account coverage = obligatory account span`; `a * conformance mastery basis = complete conformance grasp`; `a * oversight coverage insight = audit coverage sense`
- Step 3: centroid of the projected contributors selects **Coverage Obligation Map**.

#### C(normative, consistency)

- Intermediate collection: `{prescriptive direction * reliable measurement = rule measurement basis; mandatory practice * coherent message = required signal alignment; compliance determination * coherent understanding = conformance coherence basis; regulatory audit * principled reasoning = oversight reasoning control}`
- Step 1: `a = normative * consistency = binding coherence frame`
- Step 2: `a * rule measurement basis = governed measurement stability`; `a * required signal alignment = obligated semantic alignment`; `a * conformance coherence basis = stable conformance rationale`; `a * oversight reasoning control = disciplined audit rationale`
- Step 3: centroid of the projected contributors selects **Coherence Control Rule**.

#### C(operative, necessity)

- Intermediate collection: `{procedural direction * essential fact = procedure fact basis; practical execution * essential signal = execution trigger; performance assessment * fundamental understanding = performance premise; process audit * essential discernment = process review cue}`
- Step 1: `a = operative * necessity = execution prerequisite`
- Step 2: `a * procedure fact basis = required process datum`; `a * execution trigger = actionable runtime cue`; `a * performance premise = measurable execution basis`; `a * process review cue = operational control signal`
- Step 3: centroid of the projected contributors selects **Required Runtime Evidence**.

#### C(operative, sufficiency)

- Intermediate collection: `{procedural direction * adequate evidence = procedure proof basis; practical execution * adequate context = execution context; performance assessment * competent expertise = performance judgment skill; process audit * adequate judgment = process adequacy check}`
- Step 1: `a = operative * sufficiency = execution adequacy frame`
- Step 2: `a * procedure proof basis = usable process proof`; `a * execution context = situated action frame`; `a * performance judgment skill = practical assessment capability`; `a * process adequacy check = workable audit threshold`
- Step 3: centroid of the projected contributors selects **Executable Unit Context**.

#### C(operative, completeness)

- Intermediate collection: `{procedural direction * comprehensive record = procedure record span; practical execution * comprehensive account = execution account coverage; performance assessment * thorough mastery = performance mastery basis; process audit * holistic insight = process coverage insight}`
- Step 1: `a = operative * completeness = execution coverage frame`
- Step 2: `a * procedure record span = full process trace`; `a * execution account coverage = complete action account`; `a * performance mastery basis = whole performance grasp`; `a * process coverage insight = audit process panorama`
- Step 3: centroid of the projected contributors selects **Coverage Work Pattern**.

#### C(operative, consistency)

- Intermediate collection: `{procedural direction * reliable measurement = procedure measurement basis; practical execution * coherent message = execution signal alignment; performance assessment * coherent understanding = performance coherence basis; process audit * principled reasoning = process reasoning control}`
- Step 1: `a = operative * consistency = execution coherence frame`
- Step 2: `a * procedure measurement basis = stable process metric`; `a * execution signal alignment = aligned action signal`; `a * performance coherence basis = coherent performance rationale`; `a * process reasoning control = disciplined process rationale`
- Step 3: centroid of the projected contributors selects **Stable Flow Signal**.

#### C(evaluative, necessity)

- Intermediate collection: `{value orientation * essential fact = value fact basis; merit application * essential signal = merit trigger; worth determination * fundamental understanding = worth premise; quality appraisal * essential discernment = appraisal cue}`
- Step 1: `a = evaluative * necessity = review prerequisite`
- Step 2: `a * value fact basis = essential value evidence`; `a * merit trigger = acceptance cue`; `a * worth premise = justified worth basis`; `a * appraisal cue = quality review trigger`
- Step 3: centroid of the projected contributors selects **Review Trigger Basis**.

#### C(evaluative, sufficiency)

- Intermediate collection: `{value orientation * adequate evidence = value evidence basis; merit application * adequate context = merit context; worth determination * competent expertise = worth judgment skill; quality appraisal * adequate judgment = appraisal adequacy check}`
- Step 1: `a = evaluative * sufficiency = review adequacy frame`
- Step 2: `a * value evidence basis = acceptable value proof`; `a * merit context = qualified acceptance frame`; `a * worth judgment skill = appraisal capability`; `a * appraisal adequacy check = quality threshold`
- Step 3: centroid of the projected contributors selects **Acceptance Evidence Frame**.

#### C(evaluative, completeness)

- Intermediate collection: `{value orientation * comprehensive record = value record span; merit application * comprehensive account = merit account coverage; worth determination * thorough mastery = worth mastery basis; quality appraisal * holistic insight = appraisal coverage insight}`
- Step 1: `a = evaluative * completeness = review coverage frame`
- Step 2: `a * value record span = full value trace`; `a * merit account coverage = complete merit account`; `a * worth mastery basis = integrated worth grasp`; `a * appraisal coverage insight = quality panorama`
- Step 3: centroid of the projected contributors selects **Assurance Coverage Model**.

#### C(evaluative, consistency)

- Intermediate collection: `{value orientation * reliable measurement = value measurement basis; merit application * coherent message = merit signal alignment; worth determination * coherent understanding = worth coherence basis; quality appraisal * principled reasoning = appraisal reasoning control}`
- Step 1: `a = evaluative * consistency = review coherence frame`
- Step 2: `a * value measurement basis = stable value measure`; `a * merit signal alignment = aligned merit signal`; `a * worth coherence basis = coherent worth rationale`; `a * appraisal reasoning control = principled quality rationale`
- Step 3: centroid of the projected contributors selects **Quality Coherence Basis**.

### Result

| | **necessity** | **sufficiency** | **completeness** | **consistency** |
|---|---|---|---|---|
| **normative** | Binding Unit Premise | Adequacy Rule Frame | Coverage Obligation Map | Coherence Control Rule |
| **operative** | Required Runtime Evidence | Executable Unit Context | Coverage Work Pattern | Stable Flow Signal |
| **evaluative** | Review Trigger Basis | Acceptance Evidence Frame | Assurance Coverage Model | Quality Coherence Basis |

## Matrix F - Requirements (3x4)

### Construction: Dot product C * B

`L_F(i,j) = sum_k (C(i,k) * B(k,j))`; then `F(i,j) = I(row_i, col_j, L_F(i,j))`.

### Interpretation Work

#### F(normative, necessity)

- Intermediate collection: `{Binding Unit Premise * essential fact = binding unit fact; Adequacy Rule Frame * essential signal = rule adequacy signal; Coverage Obligation Map * fundamental understanding = obligation understanding; Coherence Control Rule * essential discernment = control discernment}`
- Step 1: `a = normative * necessity = binding prerequisite`
- Step 2: `a * binding unit fact = required unit condition`; `a * rule adequacy signal = mandatory evidence cue`; `a * obligation understanding = governed condition grasp`; `a * control discernment = enforceable control insight`
- Step 3: centroid of the projected contributors selects **Mandated Unit Conditions**.

#### F(normative, sufficiency)

- Intermediate collection: `{Binding Unit Premise * adequate evidence = unit proof basis; Adequacy Rule Frame * adequate context = rule context threshold; Coverage Obligation Map * competent expertise = obligation competence; Coherence Control Rule * adequate judgment = control adequacy judgment}`
- Step 1: `a = normative * sufficiency = binding adequacy frame`
- Step 2: `a * unit proof basis = accepted unit proof`; `a * rule context threshold = governed threshold`; `a * obligation competence = competent obligation use`; `a * control adequacy judgment = disciplined adequacy finding`
- Step 3: centroid of the projected contributors selects **Rule Evidence Threshold**.

#### F(normative, completeness)

- Intermediate collection: `{Binding Unit Premise * comprehensive record = unit record span; Adequacy Rule Frame * comprehensive account = rule account span; Coverage Obligation Map * thorough mastery = obligation mastery; Coherence Control Rule * holistic insight = control coverage insight}`
- Step 1: `a = normative * completeness = binding coverage frame`
- Step 2: `a * unit record span = complete unit trace`; `a * rule account span = full rule account`; `a * obligation mastery = mastered obligation span`; `a * control coverage insight = governed panorama`
- Step 3: centroid of the projected contributors selects **Contract Coverage Standard**.

#### F(normative, consistency)

- Intermediate collection: `{Binding Unit Premise * reliable measurement = unit measurement reliability; Adequacy Rule Frame * coherent message = rule message alignment; Coverage Obligation Map * coherent understanding = obligation coherence; Coherence Control Rule * principled reasoning = control reasoning principle}`
- Step 1: `a = normative * consistency = binding coherence frame`
- Step 2: `a * unit measurement reliability = stable unit measure`; `a * rule message alignment = aligned rule signal`; `a * obligation coherence = coherent obligation basis`; `a * control reasoning principle = principled control rationale`
- Step 3: centroid of the projected contributors selects **Governed Coherence Standard**.

#### F(operative, necessity)

- Intermediate collection: `{Required Runtime Evidence * essential fact = runtime fact basis; Executable Unit Context * essential signal = execution unit signal; Coverage Work Pattern * fundamental understanding = work pattern premise; Stable Flow Signal * essential discernment = flow trigger insight}`
- Step 1: `a = operative * necessity = execution prerequisite`
- Step 2: `a * runtime fact basis = required runtime condition`; `a * execution unit signal = actionable unit trigger`; `a * work pattern premise = work prerequisite grasp`; `a * flow trigger insight = operational readiness cue`
- Step 3: centroid of the projected contributors selects **Runtime Unit Preconditions**.

#### F(operative, sufficiency)

- Intermediate collection: `{Required Runtime Evidence * adequate evidence = runtime proof basis; Executable Unit Context * adequate context = executable context; Coverage Work Pattern * competent expertise = work competence; Stable Flow Signal * adequate judgment = flow adequacy judgment}`
- Step 1: `a = operative * sufficiency = execution adequacy frame`
- Step 2: `a * runtime proof basis = usable runtime proof`; `a * executable context = action-ready context`; `a * work competence = competent workflow use`; `a * flow adequacy judgment = practical adequacy finding`
- Step 3: centroid of the projected contributors selects **Executable Evidence Threshold**.

#### F(operative, completeness)

- Intermediate collection: `{Required Runtime Evidence * comprehensive record = runtime record span; Executable Unit Context * comprehensive account = executable account span; Coverage Work Pattern * thorough mastery = work mastery; Stable Flow Signal * holistic insight = flow coverage insight}`
- Step 1: `a = operative * completeness = execution coverage frame`
- Step 2: `a * runtime record span = full runtime trace`; `a * executable account span = complete action account`; `a * work mastery = mastered workflow span`; `a * flow coverage insight = process panorama`
- Step 3: centroid of the projected contributors selects **Workflow Coverage Standard**.

#### F(operative, consistency)

- Intermediate collection: `{Required Runtime Evidence * reliable measurement = runtime measurement reliability; Executable Unit Context * coherent message = executable message alignment; Coverage Work Pattern * coherent understanding = workflow coherence; Stable Flow Signal * principled reasoning = flow reasoning principle}`
- Step 1: `a = operative * consistency = execution coherence frame`
- Step 2: `a * runtime measurement reliability = stable runtime measure`; `a * executable message alignment = aligned execution signal`; `a * workflow coherence = coherent work rationale`; `a * flow reasoning principle = principled process rationale`
- Step 3: centroid of the projected contributors selects **Deterministic Signal Standard**.

#### F(evaluative, necessity)

- Intermediate collection: `{Review Trigger Basis * essential fact = review fact basis; Acceptance Evidence Frame * essential signal = acceptance signal; Assurance Coverage Model * fundamental understanding = assurance premise; Quality Coherence Basis * essential discernment = quality discernment trigger}`
- Step 1: `a = evaluative * necessity = review prerequisite`
- Step 2: `a * review fact basis = required review evidence`; `a * acceptance signal = acceptance trigger`; `a * assurance premise = assurance prerequisite`; `a * quality discernment trigger = quality review cue`
- Step 3: centroid of the projected contributors selects **Review Evidence Preconditions**.

#### F(evaluative, sufficiency)

- Intermediate collection: `{Review Trigger Basis * adequate evidence = review proof basis; Acceptance Evidence Frame * adequate context = acceptance context; Assurance Coverage Model * competent expertise = assurance competence; Quality Coherence Basis * adequate judgment = quality adequacy judgment}`
- Step 1: `a = evaluative * sufficiency = review adequacy frame`
- Step 2: `a * review proof basis = accepted review proof`; `a * acceptance context = qualified acceptance frame`; `a * assurance competence = competent assurance basis`; `a * quality adequacy judgment = quality threshold finding`
- Step 3: centroid of the projected contributors selects **Acceptance Evidence Threshold**.

#### F(evaluative, completeness)

- Intermediate collection: `{Review Trigger Basis * comprehensive record = review record span; Acceptance Evidence Frame * comprehensive account = acceptance account span; Assurance Coverage Model * thorough mastery = assurance mastery; Quality Coherence Basis * holistic insight = quality coverage insight}`
- Step 1: `a = evaluative * completeness = review coverage frame`
- Step 2: `a * review record span = full review trace`; `a * acceptance account span = complete acceptance account`; `a * assurance mastery = mastered assurance span`; `a * quality coverage insight = quality panorama`
- Step 3: centroid of the projected contributors selects **Assurance Coverage Standard**.

#### F(evaluative, consistency)

- Intermediate collection: `{Review Trigger Basis * reliable measurement = review measurement reliability; Acceptance Evidence Frame * coherent message = acceptance message alignment; Assurance Coverage Model * coherent understanding = assurance coherence; Quality Coherence Basis * principled reasoning = quality reasoning principle}`
- Step 1: `a = evaluative * consistency = review coherence frame`
- Step 2: `a * review measurement reliability = stable review measure`; `a * acceptance message alignment = aligned acceptance signal`; `a * assurance coherence = coherent assurance basis`; `a * quality reasoning principle = principled quality rationale`
- Step 3: centroid of the projected contributors selects **Quality Coherence Standard**.

### Result

| | **necessity** | **sufficiency** | **completeness** | **consistency** |
|---|---|---|---|---|
| **normative** | Mandated Unit Conditions | Rule Evidence Threshold | Contract Coverage Standard | Governed Coherence Standard |
| **operative** | Runtime Unit Preconditions | Executable Evidence Threshold | Workflow Coverage Standard | Deterministic Signal Standard |
| **evaluative** | Review Evidence Preconditions | Acceptance Evidence Threshold | Assurance Coverage Standard | Quality Coherence Standard |

## Matrix D - Objectives (3x4)

### Construction: Addition A + resolution-transformed F

`L_D(i,j) = A(i,j) + ("resolution" * F(i,j))`, using positional alignment between `D` columns and `F` columns; then `D(i,j) = I(row_i, col_j, L_D(i,j))`.

### Interpretation Work

#### D(normative, guiding)

- Intermediate collection: `{prescriptive direction; resolution * Mandated Unit Conditions = closed unit obligation}`
- Step 1: `a = normative * guiding = governing direction`
- Step 2: `a * prescriptive direction = directed rule intent`; `a * closed unit obligation = resolved unit mandate`
- Step 3: centroid of the projected contributors selects **Directed Unit Obligation**.

#### D(normative, applying)

- Intermediate collection: `{mandatory practice; resolution * Rule Evidence Threshold = closed evidence threshold}`
- Step 1: `a = normative * applying = governing practice`
- Step 2: `a * mandatory practice = required enactment`; `a * closed evidence threshold = resolved proof practice`
- Step 3: centroid of the projected contributors selects **Enforced Evidence Practice**.

#### D(normative, judging)

- Intermediate collection: `{compliance determination; resolution * Contract Coverage Standard = closed coverage standard}`
- Step 1: `a = normative * judging = governing decision`
- Step 2: `a * compliance determination = conformance decision`; `a * closed coverage standard = resolved coverage ruling`
- Step 3: centroid of the projected contributors selects **Coverage Decision Rule**.

#### D(normative, reviewing)

- Intermediate collection: `{regulatory audit; resolution * Governed Coherence Standard = closed coherence standard}`
- Step 1: `a = normative * reviewing = governing review`
- Step 2: `a * regulatory audit = formal audit check`; `a * closed coherence standard = resolved coherence control`
- Step 3: centroid of the projected contributors selects **Coherence Audit Standard**.

#### D(operative, guiding)

- Intermediate collection: `{procedural direction; resolution * Runtime Unit Preconditions = closed runtime preconditions}`
- Step 1: `a = operative * guiding = execution direction`
- Step 2: `a * procedural direction = process route`; `a * closed runtime preconditions = resolved runtime entry`
- Step 3: centroid of the projected contributors selects **Execution Preconditions Path**.

#### D(operative, applying)

- Intermediate collection: `{practical execution; resolution * Executable Evidence Threshold = closed executable threshold}`
- Step 1: `a = operative * applying = execution practice`
- Step 2: `a * practical execution = applied work`; `a * closed executable threshold = resolved usable proof`
- Step 3: centroid of the projected contributors selects **Runtime Evidence Practice**.

#### D(operative, judging)

- Intermediate collection: `{performance assessment; resolution * Workflow Coverage Standard = closed workflow standard}`
- Step 1: `a = operative * judging = execution decision`
- Step 2: `a * performance assessment = measured work appraisal`; `a * closed workflow standard = resolved work coverage`
- Step 3: centroid of the projected contributors selects **Coverage Assessment Method**.

#### D(operative, reviewing)

- Intermediate collection: `{process audit; resolution * Deterministic Signal Standard = closed signal standard}`
- Step 1: `a = operative * reviewing = execution review`
- Step 2: `a * process audit = process check`; `a * closed signal standard = resolved repeatable signal`
- Step 3: centroid of the projected contributors selects **Deterministic Process Check**.

#### D(evaluative, guiding)

- Intermediate collection: `{value orientation; resolution * Review Evidence Preconditions = closed review preconditions}`
- Step 1: `a = evaluative * guiding = review direction`
- Step 2: `a * value orientation = value-bearing aim`; `a * closed review preconditions = resolved appraisal entry`
- Step 3: centroid of the projected contributors selects **Review Basis Orientation**.

#### D(evaluative, applying)

- Intermediate collection: `{merit application; resolution * Acceptance Evidence Threshold = closed acceptance threshold}`
- Step 1: `a = evaluative * applying = review practice`
- Step 2: `a * merit application = applied merit use`; `a * closed acceptance threshold = resolved acceptance proof`
- Step 3: centroid of the projected contributors selects **Acceptance Evidence Use**.

#### D(evaluative, judging)

- Intermediate collection: `{worth determination; resolution * Assurance Coverage Standard = closed assurance standard}`
- Step 1: `a = evaluative * judging = review decision`
- Step 2: `a * worth determination = worth finding`; `a * closed assurance standard = resolved assurance scope`
- Step 3: centroid of the projected contributors selects **Assurance Decision Basis**.

#### D(evaluative, reviewing)

- Intermediate collection: `{quality appraisal; resolution * Quality Coherence Standard = closed quality standard}`
- Step 1: `a = evaluative * reviewing = review appraisal`
- Step 2: `a * quality appraisal = quality check`; `a * closed quality standard = resolved quality coherence`
- Step 3: centroid of the projected contributors selects **Quality Appraisal Check**.

### Result

| | **guiding** | **applying** | **judging** | **reviewing** |
|---|---|---|---|---|
| **normative** | Directed Unit Obligation | Enforced Evidence Practice | Coverage Decision Rule | Coherence Audit Standard |
| **operative** | Execution Preconditions Path | Runtime Evidence Practice | Coverage Assessment Method | Deterministic Process Check |
| **evaluative** | Review Basis Orientation | Acceptance Evidence Use | Assurance Decision Basis | Quality Appraisal Check |

## Matrix K - Transpose of D (4x3)

### Construction: K(i,j) = D(j,i)

This is a structural transpose of the completed Matrix D; no interpretation operator is required.

### Result

| | **normative** | **operative** | **evaluative** |
|---|---|---|---|
| **guiding** | Directed Unit Obligation | Execution Preconditions Path | Review Basis Orientation |
| **applying** | Enforced Evidence Practice | Runtime Evidence Practice | Acceptance Evidence Use |
| **judging** | Coverage Decision Rule | Coverage Assessment Method | Assurance Decision Basis |
| **reviewing** | Coherence Audit Standard | Deterministic Process Check | Quality Appraisal Check |

## Matrix G - Truncation of B (3x4)

### Construction: remove `wisdom` row from B

This is a structural truncation of canonical Matrix B; no interpretation operator is required.

### Result

| | **necessity** | **sufficiency** | **completeness** | **consistency** |
|---|---|---|---|---|
| **data** | essential fact | adequate evidence | comprehensive record | reliable measurement |
| **information** | essential signal | adequate context | comprehensive account | coherent message |
| **knowledge** | fundamental understanding | competent expertise | thorough mastery | coherent understanding |

## Matrix X - Verification (4x4)

### Construction: Dot product K * G

`L_X(i,j) = sum_k (K(i,k) * G(k,j))`; then `X(i,j) = I(row_i, col_j, L_X(i,j))`.

### Interpretation Work

#### X(guiding, necessity)

- Intermediate collection: `{Directed Unit Obligation * essential fact = directive fact gate; Execution Preconditions Path * essential signal = execution trigger path; Review Basis Orientation * fundamental understanding = review rationale frame}`
- Step 1: `a = guiding * necessity = initiating prerequisite`
- Step 2: `a * directive fact gate = required initiation fact`; `a * execution trigger path = action entry cue`; `a * review rationale frame = review entry rationale`
- Step 3: centroid of the projected contributors selects **Initiation Evidence Gate**.

#### X(guiding, sufficiency)

- Intermediate collection: `{Directed Unit Obligation * adequate evidence = directive proof gate; Execution Preconditions Path * adequate context = execution context path; Review Basis Orientation * competent expertise = review competence frame}`
- Step 1: `a = guiding * sufficiency = initiating adequacy frame`
- Step 2: `a * directive proof gate = adequate initiation proof`; `a * execution context path = usable action context`; `a * review competence frame = qualified review entry`
- Step 3: centroid of the projected contributors selects **Context Readiness Gate**.

#### X(guiding, completeness)

- Intermediate collection: `{Directed Unit Obligation * comprehensive record = directive record gate; Execution Preconditions Path * comprehensive account = execution account path; Review Basis Orientation * thorough mastery = review mastery frame}`
- Step 1: `a = guiding * completeness = initiating coverage frame`
- Step 2: `a * directive record gate = full initiation trace`; `a * execution account path = complete action route`; `a * review mastery frame = whole review readiness`
- Step 3: centroid of the projected contributors selects **Coverage Readiness Gate**.

#### X(guiding, consistency)

- Intermediate collection: `{Directed Unit Obligation * reliable measurement = directive measurement gate; Execution Preconditions Path * coherent message = execution message path; Review Basis Orientation * coherent understanding = review coherence frame}`
- Step 1: `a = guiding * consistency = initiating coherence frame`
- Step 2: `a * directive measurement gate = stable initiation measure`; `a * execution message path = aligned action route`; `a * review coherence frame = coherent review entry`
- Step 3: centroid of the projected contributors selects **Coherence Readiness Gate**.

#### X(applying, necessity)

- Intermediate collection: `{Enforced Evidence Practice * essential fact = practice fact gate; Runtime Evidence Practice * essential signal = runtime signal practice; Acceptance Evidence Use * fundamental understanding = acceptance premise use}`
- Step 1: `a = applying * necessity = practice prerequisite`
- Step 2: `a * practice fact gate = required practice fact`; `a * runtime signal practice = action trigger practice`; `a * acceptance premise use = acceptance entry rationale`
- Step 3: centroid of the projected contributors selects **Practice Entry Gate**.

#### X(applying, sufficiency)

- Intermediate collection: `{Enforced Evidence Practice * adequate evidence = practice proof gate; Runtime Evidence Practice * adequate context = runtime context practice; Acceptance Evidence Use * competent expertise = acceptance competence use}`
- Step 1: `a = applying * sufficiency = practice adequacy frame`
- Step 2: `a * practice proof gate = adequate practice proof`; `a * runtime context practice = usable runtime context`; `a * acceptance competence use = qualified acceptance use`
- Step 3: centroid of the projected contributors selects **Execution Evidence Gate**.

#### X(applying, completeness)

- Intermediate collection: `{Enforced Evidence Practice * comprehensive record = practice record gate; Runtime Evidence Practice * comprehensive account = runtime account practice; Acceptance Evidence Use * thorough mastery = acceptance mastery use}`
- Step 1: `a = applying * completeness = practice coverage frame`
- Step 2: `a * practice record gate = full practice trace`; `a * runtime account practice = complete runtime account`; `a * acceptance mastery use = whole acceptance practice`
- Step 3: centroid of the projected contributors selects **Workflow Coverage Gate**.

#### X(applying, consistency)

- Intermediate collection: `{Enforced Evidence Practice * reliable measurement = practice measurement gate; Runtime Evidence Practice * coherent message = runtime message practice; Acceptance Evidence Use * coherent understanding = acceptance coherence use}`
- Step 1: `a = applying * consistency = practice coherence frame`
- Step 2: `a * practice measurement gate = stable practice measure`; `a * runtime message practice = aligned runtime signal`; `a * acceptance coherence use = coherent acceptance practice`
- Step 3: centroid of the projected contributors selects **Signal Stability Gate**.

#### X(judging, necessity)

- Intermediate collection: `{Coverage Decision Rule * essential fact = decision fact gate; Coverage Assessment Method * essential signal = assessment signal method; Assurance Decision Basis * fundamental understanding = assurance premise basis}`
- Step 1: `a = judging * necessity = decision prerequisite`
- Step 2: `a * decision fact gate = required decision fact`; `a * assessment signal method = assessment trigger`; `a * assurance premise basis = assurance decision premise`
- Step 3: centroid of the projected contributors selects **Decision Evidence Gate**.

#### X(judging, sufficiency)

- Intermediate collection: `{Coverage Decision Rule * adequate evidence = decision proof gate; Coverage Assessment Method * adequate context = assessment context method; Assurance Decision Basis * competent expertise = assurance competence basis}`
- Step 1: `a = judging * sufficiency = decision adequacy frame`
- Step 2: `a * decision proof gate = adequate decision proof`; `a * assessment context method = usable assessment context`; `a * assurance competence basis = qualified assurance basis`
- Step 3: centroid of the projected contributors selects **Assessment Context Gate**.

#### X(judging, completeness)

- Intermediate collection: `{Coverage Decision Rule * comprehensive record = decision record gate; Coverage Assessment Method * comprehensive account = assessment account method; Assurance Decision Basis * thorough mastery = assurance mastery basis}`
- Step 1: `a = judging * completeness = decision coverage frame`
- Step 2: `a * decision record gate = full decision trace`; `a * assessment account method = complete assessment account`; `a * assurance mastery basis = whole assurance basis`
- Step 3: centroid of the projected contributors selects **Coverage Decision Gate**.

#### X(judging, consistency)

- Intermediate collection: `{Coverage Decision Rule * reliable measurement = decision measurement gate; Coverage Assessment Method * coherent message = assessment message method; Assurance Decision Basis * coherent understanding = assurance coherence basis}`
- Step 1: `a = judging * consistency = decision coherence frame`
- Step 2: `a * decision measurement gate = stable decision measure`; `a * assessment message method = aligned assessment signal`; `a * assurance coherence basis = coherent assurance rationale`
- Step 3: centroid of the projected contributors selects **Coherence Decision Gate**.

#### X(reviewing, necessity)

- Intermediate collection: `{Coherence Audit Standard * essential fact = audit fact gate; Deterministic Process Check * essential signal = process signal check; Quality Appraisal Check * fundamental understanding = quality premise check}`
- Step 1: `a = reviewing * necessity = audit prerequisite`
- Step 2: `a * audit fact gate = required audit fact`; `a * process signal check = process trigger check`; `a * quality premise check = quality audit premise`
- Step 3: centroid of the projected contributors selects **Audit Evidence Gate**.

#### X(reviewing, sufficiency)

- Intermediate collection: `{Coherence Audit Standard * adequate evidence = audit proof gate; Deterministic Process Check * adequate context = process context check; Quality Appraisal Check * competent expertise = quality competence check}`
- Step 1: `a = reviewing * sufficiency = audit adequacy frame`
- Step 2: `a * audit proof gate = adequate audit proof`; `a * process context check = usable process context`; `a * quality competence check = qualified quality check`
- Step 3: centroid of the projected contributors selects **Process Context Gate**.

#### X(reviewing, completeness)

- Intermediate collection: `{Coherence Audit Standard * comprehensive record = audit record gate; Deterministic Process Check * comprehensive account = process account check; Quality Appraisal Check * thorough mastery = quality mastery check}`
- Step 1: `a = reviewing * completeness = audit coverage frame`
- Step 2: `a * audit record gate = full audit trace`; `a * process account check = complete process account`; `a * quality mastery check = whole quality check`
- Step 3: centroid of the projected contributors selects **Audit Coverage Gate**.

#### X(reviewing, consistency)

- Intermediate collection: `{Coherence Audit Standard * reliable measurement = audit measurement gate; Deterministic Process Check * coherent message = process message check; Quality Appraisal Check * coherent understanding = quality coherence check}`
- Step 1: `a = reviewing * consistency = audit coherence frame`
- Step 2: `a * audit measurement gate = stable audit measure`; `a * process message check = aligned process signal`; `a * quality coherence check = coherent quality check`
- Step 3: centroid of the projected contributors selects **Process Coherence Gate**.

### Result

| | **necessity** | **sufficiency** | **completeness** | **consistency** |
|---|---|---|---|---|
| **guiding** | Initiation Evidence Gate | Context Readiness Gate | Coverage Readiness Gate | Coherence Readiness Gate |
| **applying** | Practice Entry Gate | Execution Evidence Gate | Workflow Coverage Gate | Signal Stability Gate |
| **judging** | Decision Evidence Gate | Assessment Context Gate | Coverage Decision Gate | Coherence Decision Gate |
| **reviewing** | Audit Evidence Gate | Process Context Gate | Audit Coverage Gate | Process Coherence Gate |

## Matrix T - Transpose of B (4x4)

### Construction: T(i,j) = B(j,i)

This is a structural transpose of canonical Matrix B; no interpretation operator is required.

### Result

| | **data** | **information** | **knowledge** | **wisdom** |
|---|---|---|---|---|
| **necessity** | essential fact | essential signal | fundamental understanding | essential discernment |
| **sufficiency** | adequate evidence | adequate context | competent expertise | adequate judgment |
| **completeness** | comprehensive record | comprehensive account | thorough mastery | holistic insight |
| **consistency** | reliable measurement | coherent message | coherent understanding | principled reasoning |

## Matrix E - Evaluation (4x4)

### Construction: Dot product X * T

`L_E(i,j) = sum_k (X(i,k) * T(k,j))`; then `E(i,j) = I(row_i, col_j, L_E(i,j))`.

### Interpretation Work

#### E(guiding, data)

- Intermediate collection: `{Initiation Evidence Gate * essential fact = initiation fact proof; Context Readiness Gate * adequate evidence = context proof readiness; Coverage Readiness Gate * comprehensive record = coverage record readiness; Coherence Readiness Gate * reliable measurement = coherence measurement readiness}`
- Step 1: `a = guiding * data = directive fact frame`
- Step 2: `a * initiation fact proof = source initiation assurance`; `a * context proof readiness = contextual proof assurance`; `a * coverage record readiness = record coverage assurance`; `a * coherence measurement readiness = stable measurement assurance`
- Step 3: centroid of the projected contributors selects **Fact Basis Assurance**.

#### E(guiding, information)

- Intermediate collection: `{Initiation Evidence Gate * essential signal = initiation signal proof; Context Readiness Gate * adequate context = context readiness context; Coverage Readiness Gate * comprehensive account = coverage account readiness; Coherence Readiness Gate * coherent message = coherence message readiness}`
- Step 1: `a = guiding * information = directive signal frame`
- Step 2: `a * initiation signal proof = source signal assurance`; `a * context readiness context = contextual readiness assurance`; `a * coverage account readiness = account coverage assurance`; `a * coherence message readiness = aligned message assurance`
- Step 3: centroid of the projected contributors selects **Context Basis Assurance**.

#### E(guiding, knowledge)

- Intermediate collection: `{Initiation Evidence Gate * fundamental understanding = initiation understanding proof; Context Readiness Gate * competent expertise = context expertise readiness; Coverage Readiness Gate * thorough mastery = coverage mastery readiness; Coherence Readiness Gate * coherent understanding = coherence understanding readiness}`
- Step 1: `a = guiding * knowledge = directive expertise frame`
- Step 2: `a * initiation understanding proof = source comprehension assurance`; `a * context expertise readiness = qualified expertise assurance`; `a * coverage mastery readiness = mastery coverage assurance`; `a * coherence understanding readiness = coherent grasp assurance`
- Step 3: centroid of the projected contributors selects **Expertise Basis Assurance**.

#### E(guiding, wisdom)

- Intermediate collection: `{Initiation Evidence Gate * essential discernment = initiation discernment proof; Context Readiness Gate * adequate judgment = context judgment readiness; Coverage Readiness Gate * holistic insight = coverage insight readiness; Coherence Readiness Gate * principled reasoning = coherence reasoning readiness}`
- Step 1: `a = guiding * wisdom = directive judgment frame`
- Step 2: `a * initiation discernment proof = source discernment assurance`; `a * context judgment readiness = qualified judgment assurance`; `a * coverage insight readiness = holistic coverage assurance`; `a * coherence reasoning readiness = principled rationale assurance`
- Step 3: centroid of the projected contributors selects **Discernment Basis Assurance**.

#### E(applying, data)

- Intermediate collection: `{Practice Entry Gate * essential fact = practice fact proof; Execution Evidence Gate * adequate evidence = execution proof gate; Workflow Coverage Gate * comprehensive record = workflow record gate; Signal Stability Gate * reliable measurement = signal measurement gate}`
- Step 1: `a = applying * data = practice fact frame`
- Step 2: `a * practice fact proof = applied fact assurance`; `a * execution proof gate = execution evidence assurance`; `a * workflow record gate = workflow record assurance`; `a * signal measurement gate = measured signal assurance`
- Step 3: centroid of the projected contributors selects **Fact Practice Assurance**.

#### E(applying, information)

- Intermediate collection: `{Practice Entry Gate * essential signal = practice signal proof; Execution Evidence Gate * adequate context = execution context gate; Workflow Coverage Gate * comprehensive account = workflow account gate; Signal Stability Gate * coherent message = signal message gate}`
- Step 1: `a = applying * information = practice signal frame`
- Step 2: `a * practice signal proof = applied signal assurance`; `a * execution context gate = execution context assurance`; `a * workflow account gate = workflow account assurance`; `a * signal message gate = aligned signal assurance`
- Step 3: centroid of the projected contributors selects **Context Practice Assurance**.

#### E(applying, knowledge)

- Intermediate collection: `{Practice Entry Gate * fundamental understanding = practice understanding proof; Execution Evidence Gate * competent expertise = execution expertise gate; Workflow Coverage Gate * thorough mastery = workflow mastery gate; Signal Stability Gate * coherent understanding = signal understanding gate}`
- Step 1: `a = applying * knowledge = practice expertise frame`
- Step 2: `a * practice understanding proof = applied comprehension assurance`; `a * execution expertise gate = execution expertise assurance`; `a * workflow mastery gate = workflow mastery assurance`; `a * signal understanding gate = coherent practice assurance`
- Step 3: centroid of the projected contributors selects **Expertise Practice Assurance**.

#### E(applying, wisdom)

- Intermediate collection: `{Practice Entry Gate * essential discernment = practice discernment proof; Execution Evidence Gate * adequate judgment = execution judgment gate; Workflow Coverage Gate * holistic insight = workflow insight gate; Signal Stability Gate * principled reasoning = signal reasoning gate}`
- Step 1: `a = applying * wisdom = practice judgment frame`
- Step 2: `a * practice discernment proof = applied discernment assurance`; `a * execution judgment gate = execution judgment assurance`; `a * workflow insight gate = workflow insight assurance`; `a * signal reasoning gate = principled practice assurance`
- Step 3: centroid of the projected contributors selects **Judgment Practice Assurance**.

#### E(judging, data)

- Intermediate collection: `{Decision Evidence Gate * essential fact = decision fact proof; Assessment Context Gate * adequate evidence = assessment proof gate; Coverage Decision Gate * comprehensive record = decision record gate; Coherence Decision Gate * reliable measurement = decision measurement gate}`
- Step 1: `a = judging * data = decision fact frame`
- Step 2: `a * decision fact proof = decision fact assurance`; `a * assessment proof gate = assessment proof assurance`; `a * decision record gate = decision record assurance`; `a * decision measurement gate = measured decision assurance`
- Step 3: centroid of the projected contributors selects **Fact Decision Assurance**.

#### E(judging, information)

- Intermediate collection: `{Decision Evidence Gate * essential signal = decision signal proof; Assessment Context Gate * adequate context = assessment context gate; Coverage Decision Gate * comprehensive account = decision account gate; Coherence Decision Gate * coherent message = decision message gate}`
- Step 1: `a = judging * information = decision signal frame`
- Step 2: `a * decision signal proof = decision signal assurance`; `a * assessment context gate = assessment context assurance`; `a * decision account gate = decision account assurance`; `a * decision message gate = aligned decision assurance`
- Step 3: centroid of the projected contributors selects **Context Decision Assurance**.

#### E(judging, knowledge)

- Intermediate collection: `{Decision Evidence Gate * fundamental understanding = decision understanding proof; Assessment Context Gate * competent expertise = assessment expertise gate; Coverage Decision Gate * thorough mastery = decision mastery gate; Coherence Decision Gate * coherent understanding = decision understanding gate}`
- Step 1: `a = judging * knowledge = decision expertise frame`
- Step 2: `a * decision understanding proof = decision comprehension assurance`; `a * assessment expertise gate = assessment expertise assurance`; `a * decision mastery gate = decision mastery assurance`; `a * decision understanding gate = coherent decision assurance`
- Step 3: centroid of the projected contributors selects **Expertise Decision Assurance**.

#### E(judging, wisdom)

- Intermediate collection: `{Decision Evidence Gate * essential discernment = decision discernment proof; Assessment Context Gate * adequate judgment = assessment judgment gate; Coverage Decision Gate * holistic insight = decision insight gate; Coherence Decision Gate * principled reasoning = decision reasoning gate}`
- Step 1: `a = judging * wisdom = decision judgment frame`
- Step 2: `a * decision discernment proof = decision discernment assurance`; `a * assessment judgment gate = assessment judgment assurance`; `a * decision insight gate = decision insight assurance`; `a * decision reasoning gate = principled decision assurance`
- Step 3: centroid of the projected contributors selects **Reasoning Decision Assurance**.

#### E(reviewing, data)

- Intermediate collection: `{Audit Evidence Gate * essential fact = audit fact proof; Process Context Gate * adequate evidence = process proof gate; Audit Coverage Gate * comprehensive record = audit record gate; Process Coherence Gate * reliable measurement = process measurement gate}`
- Step 1: `a = reviewing * data = audit fact frame`
- Step 2: `a * audit fact proof = audit fact assurance`; `a * process proof gate = process proof assurance`; `a * audit record gate = audit record assurance`; `a * process measurement gate = measured process assurance`
- Step 3: centroid of the projected contributors selects **Fact Audit Assurance**.

#### E(reviewing, information)

- Intermediate collection: `{Audit Evidence Gate * essential signal = audit signal proof; Process Context Gate * adequate context = process context gate; Audit Coverage Gate * comprehensive account = audit account gate; Process Coherence Gate * coherent message = process message gate}`
- Step 1: `a = reviewing * information = audit signal frame`
- Step 2: `a * audit signal proof = audit signal assurance`; `a * process context gate = process context assurance`; `a * audit account gate = audit account assurance`; `a * process message gate = aligned process assurance`
- Step 3: centroid of the projected contributors selects **Context Audit Assurance**.

#### E(reviewing, knowledge)

- Intermediate collection: `{Audit Evidence Gate * fundamental understanding = audit understanding proof; Process Context Gate * competent expertise = process expertise gate; Audit Coverage Gate * thorough mastery = audit mastery gate; Process Coherence Gate * coherent understanding = process understanding gate}`
- Step 1: `a = reviewing * knowledge = audit expertise frame`
- Step 2: `a * audit understanding proof = audit comprehension assurance`; `a * process expertise gate = process expertise assurance`; `a * audit mastery gate = audit mastery assurance`; `a * process understanding gate = coherent process assurance`
- Step 3: centroid of the projected contributors selects **Expertise Audit Assurance**.

#### E(reviewing, wisdom)

- Intermediate collection: `{Audit Evidence Gate * essential discernment = audit discernment proof; Process Context Gate * adequate judgment = process judgment gate; Audit Coverage Gate * holistic insight = audit insight gate; Process Coherence Gate * principled reasoning = process reasoning gate}`
- Step 1: `a = reviewing * wisdom = audit judgment frame`
- Step 2: `a * audit discernment proof = audit discernment assurance`; `a * process judgment gate = process judgment assurance`; `a * audit insight gate = audit insight assurance`; `a * process reasoning gate = principled process assurance`
- Step 3: centroid of the projected contributors selects **Reasoning Audit Assurance**.

### Result

| | **data** | **information** | **knowledge** | **wisdom** |
|---|---|---|---|---|
| **guiding** | Fact Basis Assurance | Context Basis Assurance | Expertise Basis Assurance | Discernment Basis Assurance |
| **applying** | Fact Practice Assurance | Context Practice Assurance | Expertise Practice Assurance | Judgment Practice Assurance |
| **judging** | Fact Decision Assurance | Context Decision Assurance | Expertise Decision Assurance | Reasoning Decision Assurance |
| **reviewing** | Fact Audit Assurance | Context Audit Assurance | Expertise Audit Assurance | Reasoning Audit Assurance |

## Audit Result

**Result:** PASS

**Cells audited:** Result tables for matrices C, F, D, X, and E.

**Checks performed:**
- Algebra leak: no final cell contains `sum`, `Sigma`, or intersection notation.
- Operator leak: no final cell contains a literal semantic addition operator.
- Expansion leak: all final audited cells are compact semantic units below the expansion threshold.
- Lens boundary: no final cell asserts engineering correctness, code compliance, approval, sealing, or certification.

## Matrix Summary

### Matrix C - Formulation

| | **necessity** | **sufficiency** | **completeness** | **consistency** |
|---|---|---|---|---|
| **normative** | Binding Unit Premise | Adequacy Rule Frame | Coverage Obligation Map | Coherence Control Rule |
| **operative** | Required Runtime Evidence | Executable Unit Context | Coverage Work Pattern | Stable Flow Signal |
| **evaluative** | Review Trigger Basis | Acceptance Evidence Frame | Assurance Coverage Model | Quality Coherence Basis |

### Matrix F - Requirements

| | **necessity** | **sufficiency** | **completeness** | **consistency** |
|---|---|---|---|---|
| **normative** | Mandated Unit Conditions | Rule Evidence Threshold | Contract Coverage Standard | Governed Coherence Standard |
| **operative** | Runtime Unit Preconditions | Executable Evidence Threshold | Workflow Coverage Standard | Deterministic Signal Standard |
| **evaluative** | Review Evidence Preconditions | Acceptance Evidence Threshold | Assurance Coverage Standard | Quality Coherence Standard |

### Matrix D - Objectives

| | **guiding** | **applying** | **judging** | **reviewing** |
|---|---|---|---|---|
| **normative** | Directed Unit Obligation | Enforced Evidence Practice | Coverage Decision Rule | Coherence Audit Standard |
| **operative** | Execution Preconditions Path | Runtime Evidence Practice | Coverage Assessment Method | Deterministic Process Check |
| **evaluative** | Review Basis Orientation | Acceptance Evidence Use | Assurance Decision Basis | Quality Appraisal Check |

### Matrix K - Transpose of D

| | **normative** | **operative** | **evaluative** |
|---|---|---|---|
| **guiding** | Directed Unit Obligation | Execution Preconditions Path | Review Basis Orientation |
| **applying** | Enforced Evidence Practice | Runtime Evidence Practice | Acceptance Evidence Use |
| **judging** | Coverage Decision Rule | Coverage Assessment Method | Assurance Decision Basis |
| **reviewing** | Coherence Audit Standard | Deterministic Process Check | Quality Appraisal Check |

### Matrix G - Truncation of B

| | **necessity** | **sufficiency** | **completeness** | **consistency** |
|---|---|---|---|---|
| **data** | essential fact | adequate evidence | comprehensive record | reliable measurement |
| **information** | essential signal | adequate context | comprehensive account | coherent message |
| **knowledge** | fundamental understanding | competent expertise | thorough mastery | coherent understanding |

### Matrix X - Verification

| | **necessity** | **sufficiency** | **completeness** | **consistency** |
|---|---|---|---|---|
| **guiding** | Initiation Evidence Gate | Context Readiness Gate | Coverage Readiness Gate | Coherence Readiness Gate |
| **applying** | Practice Entry Gate | Execution Evidence Gate | Workflow Coverage Gate | Signal Stability Gate |
| **judging** | Decision Evidence Gate | Assessment Context Gate | Coverage Decision Gate | Coherence Decision Gate |
| **reviewing** | Audit Evidence Gate | Process Context Gate | Audit Coverage Gate | Process Coherence Gate |

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
| **guiding** | Fact Basis Assurance | Context Basis Assurance | Expertise Basis Assurance | Discernment Basis Assurance |
| **applying** | Fact Practice Assurance | Context Practice Assurance | Expertise Practice Assurance | Judgment Practice Assurance |
| **judging** | Fact Decision Assurance | Context Decision Assurance | Expertise Decision Assurance | Reasoning Decision Assurance |
| **reviewing** | Fact Audit Assurance | Context Audit Assurance | Expertise Audit Assurance | Reasoning Audit Assurance |
