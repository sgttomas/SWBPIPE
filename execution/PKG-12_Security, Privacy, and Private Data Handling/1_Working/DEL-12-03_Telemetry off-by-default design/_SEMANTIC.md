# Deliverable: DEL-12-03 Telemetry off-by-default design

**Generated:** 2026-04-30
**DECOMP_VARIANT:** SOFTWARE
**Perspective:** This deliverable defines the privacy and configuration boundary for any telemetry behavior in a local-first piping stress workbench. Its semantic lens must preserve default-off operation, explicit opt-in, no private engineering data transmission, and human approval before any telemetry design is implemented.
**Framework:** Chirality Semantic Algebra
**Inputs Read:**
- `_CONTEXT.md` - deliverable identity, scope, objective, envelope, architecture basis
- `_STATUS.md` - current state SEMANTIC_READY
- `Datasheet.md` - production document
- `Specification.md` - production document
- `Guidance.md` - production document
- `Procedure.md` - production document
- `_REFERENCES.md` - governing references

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

### Construction

Matrix C interprets Matrix A orientation through Matrix B conceptual categories for the telemetry privacy boundary. Each cell records the three required interpretation steps: axis anchor, coordinate-conditioned projections, and centroid.

| Cell | Step 1: axis anchor | Step 2: projections | Step 3: centroid |
|---|---|---|---|
| C[normative,necessity] | normative with necessity -> binding need | direction with fact -> privacy mandate; practice with signal -> opt-in trigger; determination with understanding -> boundary rule; audit with discernment -> approval need | policy direction |
| C[normative,sufficiency] | normative with sufficiency -> binding adequacy | direction with evidence -> approval basis; practice with context -> consent basis; determination with expertise -> policy proof; audit with judgment -> review basis | consent basis |
| C[normative,completeness] | normative with completeness -> binding coverage | direction with record -> full policy; practice with account -> full default; determination with mastery -> full prohibition; audit with insight -> full review | boundary coverage |
| C[normative,consistency] | normative with consistency -> binding coherence | direction with measurement -> stable policy; practice with message -> stable default; determination with understanding -> stable boundary; audit with reasoning -> stable review | policy coherence |
| C[operative,necessity] | operative with necessity -> execution need | procedure with fact -> config default; execution with signal -> opt-in action; assessment with understanding -> runtime guard; audit with discernment -> startup check | default behavior |
| C[operative,sufficiency] | operative with sufficiency -> execution adequacy | procedure with evidence -> test basis; execution with context -> control basis; assessment with expertise -> implementation proof; audit with judgment -> acceptance basis | control basis |
| C[operative,completeness] | operative with completeness -> execution coverage | procedure with record -> all routes; execution with account -> all states; assessment with mastery -> all payloads; audit with insight -> all bypass checks | route coverage |
| C[operative,consistency] | operative with consistency -> execution coherence | procedure with measurement -> deterministic default; execution with message -> stable config; assessment with understanding -> repeatable result; audit with reasoning -> reproducible check | operational coherence |
| C[evaluative,necessity] | evaluative with necessity -> value need | value with fact -> privacy premise; merit with signal -> trust signal; worth with understanding -> approval boundary; quality with discernment -> leakage concern | privacy value |
| C[evaluative,sufficiency] | evaluative with sufficiency -> value adequacy | value with evidence -> review proof; merit with context -> approval context; worth with expertise -> judgment basis; quality with judgment -> acceptance basis | approval basis |
| C[evaluative,completeness] | evaluative with completeness -> value coverage | value with record -> full rationale; merit with account -> full tradeoff; worth with mastery -> full consequence; quality with insight -> full assurance | review coverage |
| C[evaluative,consistency] | evaluative with consistency -> value coherence | value with measurement -> stable privacy; merit with message -> stable consent; worth with understanding -> stable authority; quality with reasoning -> stable trust | trust coherence |

### Result

| | necessity | sufficiency | completeness | consistency |
|---|---|---|---|---|
| normative | policy direction | consent basis | boundary coverage | policy coherence |
| operative | default behavior | control basis | route coverage | operational coherence |
| evaluative | privacy value | approval basis | review coverage | trust coherence |

## Matrix F - Requirements (3x4)

### Construction

Matrix F interprets Matrix C against Matrix B to reveal requirement categories for the telemetry design.

| Cell | Step 1: axis anchor | Step 2: projections | Step 3: centroid |
|---|---|---|---|
| F[normative,necessity] | normative with necessity -> required mandate | policy direction with fact -> disabled default; consent basis with signal -> explicit opt-in; boundary coverage with understanding -> no private data; policy coherence with discernment -> human approval | default disabled |
| F[normative,sufficiency] | normative with sufficiency -> required adequacy | policy direction with evidence -> approval evidence; consent basis with context -> consent adequacy; boundary coverage with expertise -> privacy proof; policy coherence with judgment -> review adequacy | approval sufficiency |
| F[normative,completeness] | normative with completeness -> required coverage | policy direction with record -> default record; consent basis with account -> consent record; boundary coverage with mastery -> forbidden classes; policy coherence with insight -> complete review | forbidden coverage |
| F[normative,consistency] | normative with consistency -> required coherence | policy direction with measurement -> stable disabled state; consent basis with message -> stable consent; boundary coverage with understanding -> stable prohibition; policy coherence with reasoning -> stable policy | policy consistency |
| F[operative,necessity] | operative with necessity -> implementation need | default behavior with fact -> fail closed; control basis with signal -> opt-in gate; route coverage with understanding -> no side channel; operational coherence with discernment -> startup proof | fail closed |
| F[operative,sufficiency] | operative with sufficiency -> implementation adequacy | default behavior with evidence -> default proof; control basis with context -> explicit setting; route coverage with expertise -> filter proof; operational coherence with judgment -> test proof | explicit opt-in |
| F[operative,completeness] | operative with completeness -> implementation coverage | default behavior with record -> config states; control basis with account -> all controls; route coverage with mastery -> all routes; operational coherence with insight -> all checks | no bypass coverage |
| F[operative,consistency] | operative with consistency -> implementation coherence | default behavior with measurement -> repeatable disabled; control basis with message -> repeatable opt-in; route coverage with understanding -> repeatable filtering; operational coherence with reasoning -> repeatable no-op | deterministic behavior |
| F[evaluative,necessity] | evaluative with necessity -> review need | privacy value with fact -> privacy check; approval basis with signal -> approval signal; review coverage with understanding -> audit need; trust coherence with discernment -> trust risk | privacy review |
| F[evaluative,sufficiency] | evaluative with sufficiency -> review adequacy | privacy value with evidence -> evidence basis; approval basis with context -> approval adequacy; review coverage with expertise -> review proof; trust coherence with judgment -> trust proof | evidence sufficiency |
| F[evaluative,completeness] | evaluative with completeness -> review coverage | privacy value with record -> rationale record; approval basis with account -> approval record; review coverage with mastery -> full audit; trust coherence with insight -> full assurance | audit coverage |
| F[evaluative,consistency] | evaluative with consistency -> review coherence | privacy value with measurement -> stable privacy; approval basis with message -> stable approval; review coverage with understanding -> stable audit; trust coherence with reasoning -> stable trust | trust consistency |

### Result

| | necessity | sufficiency | completeness | consistency |
|---|---|---|---|---|
| normative | default disabled | approval sufficiency | forbidden coverage | policy consistency |
| operative | fail closed | explicit opt-in | no bypass coverage | deterministic behavior |
| evaluative | privacy review | evidence sufficiency | audit coverage | trust consistency |

## Matrix D - Objectives (3x4)

### Construction

Matrix D interprets Matrix A orientation with resolution-conditioned Matrix F requirements.

| Cell | Step 1: axis anchor | Step 2: projections | Step 3: centroid |
|---|---|---|---|
| D[normative,guiding] | normative with guiding -> objective direction | direction with disabled requirement -> default mandate; direction with approval requirement -> approval guard; direction with forbidden coverage -> boundary guard; direction with consistency -> stable rule | default mandate |
| D[normative,applying] | normative with applying -> objective practice | practice with disabled requirement -> default control; practice with approval requirement -> consent control; practice with forbidden coverage -> payload control; practice with consistency -> stable control | consent control |
| D[normative,judging] | normative with judging -> objective judgment | determination with disabled requirement -> default judgment; determination with approval requirement -> approval judgment; determination with forbidden coverage -> prohibition judgment; determination with consistency -> policy judgment | payload prohibition |
| D[normative,reviewing] | normative with reviewing -> objective review | audit with disabled requirement -> default review; audit with approval requirement -> approval review; audit with forbidden coverage -> payload review; audit with consistency -> policy review | approval review |
| D[operative,guiding] | operative with guiding -> execution direction | procedure with fail-closed need -> config guidance; procedure with opt-in basis -> opt-in guidance; procedure with coverage -> route guidance; procedure with determinism -> test guidance | configuration guide |
| D[operative,applying] | operative with applying -> execution practice | execution with fail-closed need -> runtime control; execution with opt-in basis -> consent gate; execution with coverage -> route guard; execution with determinism -> repeatable behavior | runtime control |
| D[operative,judging] | operative with judging -> execution judgment | assessment with fail-closed need -> startup assessment; assessment with opt-in basis -> consent assessment; assessment with coverage -> bypass assessment; assessment with determinism -> repeatability assessment | bypass check |
| D[operative,reviewing] | operative with reviewing -> execution review | process audit with fail-closed need -> config review; process audit with opt-in basis -> consent review; process audit with coverage -> bypass review; process audit with determinism -> regression review | test review |
| D[evaluative,guiding] | evaluative with guiding -> value direction | value with privacy review -> privacy rationale; value with evidence -> review rationale; value with audit -> assurance rationale; value with trust -> trust rationale | privacy rationale |
| D[evaluative,applying] | evaluative with applying -> value practice | merit with privacy review -> privacy merit; merit with evidence -> approval merit; merit with audit -> audit merit; merit with trust -> trust merit | approval judgment |
| D[evaluative,judging] | evaluative with judging -> value judgment | worth with privacy review -> privacy worth; worth with evidence -> evidence worth; worth with audit -> audit worth; worth with trust -> trust worth | audit judgment |
| D[evaluative,reviewing] | evaluative with reviewing -> value review | quality with privacy review -> privacy review; quality with evidence -> approval review; quality with audit -> assurance review; quality with trust -> trust review | trust review |

### Result

| | guiding | applying | judging | reviewing |
|---|---|---|---|---|
| normative | default mandate | consent control | payload prohibition | approval review |
| operative | configuration guide | runtime control | bypass check | test review |
| evaluative | privacy rationale | approval judgment | audit judgment | trust review |

## Matrix K - Transpose of D (4x3)

### Construction

K transposes Matrix D after interpretation.

### Result

| | normative | operative | evaluative |
|---|---|---|---|
| guiding | default mandate | configuration guide | privacy rationale |
| applying | consent control | runtime control | approval judgment |
| judging | payload prohibition | bypass check | audit judgment |
| reviewing | approval review | test review | trust review |

## Matrix G - Truncation of B (3x4)

### Construction

G removes the wisdom row from canonical Matrix B.

### Result

| | necessity | sufficiency | completeness | consistency |
|---|---|---|---|---|
| data | essential fact | adequate evidence | comprehensive record | reliable measurement |
| information | essential signal | adequate context | comprehensive account | coherent message |
| knowledge | fundamental understanding | competent expertise | thorough mastery | coherent understanding |

## Matrix X - Verification (4x4)

### Construction

Matrix X interprets transposed objectives against truncated conceptual categories to reveal verification categories.

| Cell | Step 1: axis anchor | Step 2: projections | Step 3: centroid |
|---|---|---|---|
| X[guiding,necessity] | guiding with necessity -> verification need | default mandate with fact -> disabled fact; configuration guide with signal -> config signal; privacy rationale with understanding -> privacy understanding | default evidence |
| X[guiding,sufficiency] | guiding with sufficiency -> verification adequacy | default mandate with evidence -> default proof; configuration guide with context -> config context; privacy rationale with expertise -> privacy proof | consent evidence |
| X[guiding,completeness] | guiding with completeness -> verification coverage | default mandate with record -> default record; configuration guide with account -> config account; privacy rationale with mastery -> rationale record | boundary record |
| X[guiding,consistency] | guiding with consistency -> verification coherence | default mandate with measurement -> stable default; configuration guide with message -> stable config; privacy rationale with understanding -> stable privacy | coherence check |
| X[applying,necessity] | applying with necessity -> control need | consent control with fact -> opt-in fact; runtime control with signal -> runtime signal; approval judgment with understanding -> approval understanding | config fact |
| X[applying,sufficiency] | applying with sufficiency -> control adequacy | consent control with evidence -> consent proof; runtime control with context -> runtime context; approval judgment with expertise -> approval proof | optin proof |
| X[applying,completeness] | applying with completeness -> control coverage | consent control with record -> consent record; runtime control with account -> runtime account; approval judgment with mastery -> approval record | bypass record |
| X[applying,consistency] | applying with consistency -> control coherence | consent control with measurement -> stable consent; runtime control with message -> stable runtime; approval judgment with understanding -> stable approval | deterministic check |
| X[judging,necessity] | judging with necessity -> judgment need | payload prohibition with fact -> forbidden fact; bypass check with signal -> bypass signal; audit judgment with understanding -> audit understanding | privacy fact |
| X[judging,sufficiency] | judging with sufficiency -> judgment adequacy | payload prohibition with evidence -> privacy proof; bypass check with context -> bypass proof; audit judgment with expertise -> audit proof | approval proof |
| X[judging,completeness] | judging with completeness -> judgment coverage | payload prohibition with record -> payload record; bypass check with account -> route record; audit judgment with mastery -> audit record | audit record |
| X[judging,consistency] | judging with consistency -> judgment coherence | payload prohibition with measurement -> stable prohibition; bypass check with message -> stable bypass check; audit judgment with understanding -> stable audit | claim check |
| X[reviewing,necessity] | reviewing with necessity -> review need | approval review with fact -> approval fact; test review with signal -> test signal; trust review with understanding -> trust understanding | noops fact |
| X[reviewing,sufficiency] | reviewing with sufficiency -> review adequacy | approval review with evidence -> approval evidence; test review with context -> test context; trust review with expertise -> trust proof | test proof |
| X[reviewing,completeness] | reviewing with completeness -> review coverage | approval review with record -> approval record; test review with account -> test record; trust review with mastery -> trust record | review record |
| X[reviewing,consistency] | reviewing with consistency -> review coherence | approval review with measurement -> stable approval; test review with message -> stable testing; trust review with understanding -> stable trust | release check |

### Result

| | necessity | sufficiency | completeness | consistency |
|---|---|---|---|---|
| guiding | default evidence | consent evidence | boundary record | coherence check |
| applying | config fact | optin proof | bypass record | deterministic check |
| judging | privacy fact | approval proof | audit record | claim check |
| reviewing | noops fact | test proof | review record | release check |

## Matrix T - Transpose of B (4x4)

### Construction

T transposes canonical Matrix B after interpretation.

### Result

| | data | information | knowledge | wisdom |
|---|---|---|---|---|
| necessity | essential fact | essential signal | fundamental understanding | essential discernment |
| sufficiency | adequate evidence | adequate context | competent expertise | adequate judgment |
| completeness | comprehensive record | comprehensive account | thorough mastery | holistic insight |
| consistency | reliable measurement | coherent message | coherent understanding | principled reasoning |

## Matrix E - Evaluation (4x4)

### Construction

Matrix E interprets Matrix X through Matrix T to reveal evaluation categories for the telemetry deliverable.

| Cell | Step 1: axis anchor | Step 2: projections | Step 3: centroid |
|---|---|---|---|
| E[guiding,data] | guiding with data -> evidence fact | default evidence with fact -> default fact; consent evidence with evidence -> consent proof; boundary record with record -> boundary proof; coherence check with measurement -> stable fact | default fact |
| E[guiding,information] | guiding with information -> evidence signal | default evidence with signal -> default signal; consent evidence with context -> consent context; boundary record with account -> boundary account; coherence check with message -> stable signal | consent signal |
| E[guiding,knowledge] | guiding with knowledge -> evidence understanding | default evidence with understanding -> default understanding; consent evidence with expertise -> consent expertise; boundary record with mastery -> boundary mastery; coherence check with understanding -> stable understanding | boundary understanding |
| E[guiding,wisdom] | guiding with wisdom -> evidence discernment | default evidence with discernment -> default judgment; consent evidence with judgment -> consent judgment; boundary record with insight -> boundary insight; coherence check with reasoning -> stable discernment | privacy discernment |
| E[applying,data] | applying with data -> control fact | config fact with fact -> config proof; optin proof with evidence -> opt-in proof; bypass record with record -> route proof; deterministic check with measurement -> repeatable fact | config fact |
| E[applying,information] | applying with information -> control signal | config fact with signal -> config signal; optin proof with context -> opt-in context; bypass record with account -> route account; deterministic check with message -> repeatable signal | optin signal |
| E[applying,knowledge] | applying with knowledge -> control understanding | config fact with understanding -> config understanding; optin proof with expertise -> opt-in expertise; bypass record with mastery -> route mastery; deterministic check with understanding -> repeatable understanding | control understanding |
| E[applying,wisdom] | applying with wisdom -> control discernment | config fact with discernment -> config judgment; optin proof with judgment -> opt-in judgment; bypass record with insight -> bypass insight; deterministic check with reasoning -> fail-closed judgment | failure discernment |
| E[judging,data] | judging with data -> review fact | privacy fact with fact -> privacy proof; approval proof with evidence -> approval proof; audit record with record -> audit proof; claim check with measurement -> claim proof | evidence fact |
| E[judging,information] | judging with information -> review signal | privacy fact with signal -> privacy signal; approval proof with context -> approval context; audit record with account -> audit account; claim check with message -> claim signal | approval signal |
| E[judging,knowledge] | judging with knowledge -> review understanding | privacy fact with understanding -> privacy understanding; approval proof with expertise -> approval expertise; audit record with mastery -> audit mastery; claim check with understanding -> claim understanding | audit understanding |
| E[judging,wisdom] | judging with wisdom -> review discernment | privacy fact with discernment -> privacy judgment; approval proof with judgment -> approval judgment; audit record with insight -> audit insight; claim check with reasoning -> authority reasoning | authority discernment |
| E[reviewing,data] | reviewing with data -> release fact | noops fact with fact -> no-op proof; test proof with evidence -> test proof; review record with record -> review proof; release check with measurement -> release proof | test fact |
| E[reviewing,information] | reviewing with information -> release signal | noops fact with signal -> no-op signal; test proof with context -> test context; review record with account -> review account; release check with message -> release signal | review signal |
| E[reviewing,knowledge] | reviewing with knowledge -> release understanding | noops fact with understanding -> no-op understanding; test proof with expertise -> test expertise; review record with mastery -> review mastery; release check with understanding -> release understanding | release understanding |
| E[reviewing,wisdom] | reviewing with wisdom -> release discernment | noops fact with discernment -> no-op judgment; test proof with judgment -> test judgment; review record with insight -> review insight; release check with reasoning -> trust reasoning | trust reasoning |

### Result

| | data | information | knowledge | wisdom |
|---|---|---|---|---|
| guiding | default fact | consent signal | boundary understanding | privacy discernment |
| applying | config fact | optin signal | control understanding | failure discernment |
| judging | evidence fact | approval signal | audit understanding | authority discernment |
| reviewing | test fact | review signal | release understanding | trust reasoning |

## Matrix Summary

### Matrix C

| | necessity | sufficiency | completeness | consistency |
|---|---|---|---|---|
| normative | policy direction | consent basis | boundary coverage | policy coherence |
| operative | default behavior | control basis | route coverage | operational coherence |
| evaluative | privacy value | approval basis | review coverage | trust coherence |

### Matrix F

| | necessity | sufficiency | completeness | consistency |
|---|---|---|---|---|
| normative | default disabled | approval sufficiency | forbidden coverage | policy consistency |
| operative | fail closed | explicit opt-in | no bypass coverage | deterministic behavior |
| evaluative | privacy review | evidence sufficiency | audit coverage | trust consistency |

### Matrix D

| | guiding | applying | judging | reviewing |
|---|---|---|---|---|
| normative | default mandate | consent control | payload prohibition | approval review |
| operative | configuration guide | runtime control | bypass check | test review |
| evaluative | privacy rationale | approval judgment | audit judgment | trust review |

### Matrix X

| | necessity | sufficiency | completeness | consistency |
|---|---|---|---|---|
| guiding | default evidence | consent evidence | boundary record | coherence check |
| applying | config fact | optin proof | bypass record | deterministic check |
| judging | privacy fact | approval proof | audit record | claim check |
| reviewing | noops fact | test proof | review record | release check |

### Matrix E

| | data | information | knowledge | wisdom |
|---|---|---|---|---|
| guiding | default fact | consent signal | boundary understanding | privacy discernment |
| applying | config fact | optin signal | control understanding | failure discernment |
| judging | evidence fact | approval signal | audit understanding | authority discernment |
| reviewing | test fact | review signal | release understanding | trust reasoning |

## Audit

**Result:** PASS

- All final result cells are populated with compact atomic phrases.
- Final result tables contain no algebra symbols or construction operators.
- The file contains no matrix parse error marker.
- `_STATUS.md` should be set or verified as `SEMANTIC_READY` by the setup run.
