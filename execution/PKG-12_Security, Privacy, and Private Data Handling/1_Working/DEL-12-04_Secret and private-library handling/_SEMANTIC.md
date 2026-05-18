# Semantic Lens: DEL-12-04 Secret and private-library handling

**Generated:** 2026-04-30
**DECOMP_VARIANT:** SOFTWARE
**Perspective:** This deliverable frames secret and private-library handling as a local-first security control for protecting user-owned rule packs, material/component libraries, private paths, credential references, and project models. The lens identifies categories for registry metadata, denied-by-default access, redaction/export safeguards, and diagnostics without selecting a concrete cloud service, operating-system secret provider, or final storage container.
**Framework:** Chirality Semantic Algebra
**Audit Result:** PASS

**Inputs Read:**
- `_CONTEXT.md` - deliverable identity, scope, anticipated artifacts, and architecture basis.
- `_STATUS.md` - lifecycle state.
- `Datasheet.md` - descriptive registry and test surface.
- `Specification.md` - normative requirements and verification hooks.
- `Guidance.md` - principles, trade-offs, and human-ruling items.
- `Procedure.md` - operational steps and records.
- `_REFERENCES.md` - governing source list.
- `docs/CONTRACT.md`, `docs/_Decomposition/SOFTWARE_DECOMP.md`, `docs/PRD.md`, `docs/IP_AND_DATA_BOUNDARY.md`, and architecture basis documents - perspective conditioning sources.

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

### Construction: Dot product A with B

Each interpreted cell used the three required steps: axis anchor, coordinate-conditioned projections, and centroid selection. Projection work was conditioned by the DEL-12-04 perspective and by source slices on local-first private data handling, registry metadata, provenance, redaction/export, denied-by-default permissions, and diagnostics.

| Cell | Axis anchor | Projection summary | Centroid |
|---|---|---|---|
| C[normative, necessity] | rule obligation | direction, practice, determination, and audit contributors converge on a binding private-boundary fact | binding privacy boundary |
| C[normative, sufficiency] | warranted obligation | contributors converge on enough evidence for a defensible control basis | adequate control basis |
| C[normative, completeness] | whole obligation | contributors converge on a full map of protected handling slots | complete handling map |
| C[normative, consistency] | stable obligation | contributors converge on repeated private/public vocabulary | stable terminology |
| C[operative, necessity] | executable need | contributors converge on actionable steps that protect private assets | actionable protection path |
| C[operative, sufficiency] | workable warrant | contributors converge on usable registry evidence for implementation | workable registry evidence |
| C[operative, completeness] | whole execution | contributors converge on full boundary workflow coverage | full workflow coverage |
| C[operative, consistency] | repeatable execution | contributors converge on predictable safeguard behavior | repeatable safeguard behavior |
| C[evaluative, necessity] | review need | contributors converge on reviewable exposure-risk basis | reviewable risk basis |
| C[evaluative, sufficiency] | warranted review | contributors converge on an audit trail sufficient for assessment | adequate audit trail |
| C[evaluative, completeness] | whole review | contributors converge on a complete assurance picture | complete assurance picture |
| C[evaluative, consistency] | stable review | contributors converge on coherent acceptance judgment | coherent acceptance judgment |

### Result

| | **necessity** | **sufficiency** | **completeness** | **consistency** |
|---|---|---|---|---|
| **normative** | binding privacy boundary | adequate control basis | complete handling map | stable terminology |
| **operative** | actionable protection path | workable registry evidence | full workflow coverage | repeatable safeguard behavior |
| **evaluative** | reviewable risk basis | adequate audit trail | complete assurance picture | coherent acceptance judgment |

## Matrix F - Requirements (3x4)

### Construction: Dot product C with B

Each interpreted cell used the same three-step interpretation procedure, now using Matrix C as the formulation input and Matrix B as the conceptual conditioning grid.

| Cell | Axis anchor | Projection summary | Centroid |
|---|---|---|---|
| F[normative, necessity] | rule obligation | privacy boundary and essential-source contributors converge on a binding guardrail | mandatory privacy guardrail |
| F[normative, sufficiency] | warranted obligation | control basis and evidence contributors converge on substantiated protection | evidence-backed protection |
| F[normative, completeness] | whole obligation | handling-map contributors converge on exhaustive boundary capture | exhaustive boundary capture |
| F[normative, consistency] | stable obligation | terminology and measurement contributors converge on private marking discipline | consistent private marking |
| F[operative, necessity] | executable need | protection path and essential-signal contributors converge on implementable registry control | executable registry control |
| F[operative, sufficiency] | workable warrant | registry evidence and context contributors converge on verified indirection | verified secret indirection |
| F[operative, completeness] | whole execution | workflow coverage and account contributors converge on end-to-end safeguards | end-to-end safeguard coverage |
| F[operative, consistency] | repeatable execution | safeguard behavior and message contributors converge on deterministic handling | deterministic handling behavior |
| F[evaluative, necessity] | review need | risk basis and understanding contributors converge on a reviewable finding | reviewable threat finding |
| F[evaluative, sufficiency] | warranted review | audit trail and expertise contributors converge on exposure evidence | auditable exposure evidence |
| F[evaluative, completeness] | whole review | assurance picture and mastery contributors converge on readiness gating | complete readiness gate |
| F[evaluative, consistency] | stable review | acceptance judgment and understanding contributors converge on risk disposition | coherent risk disposition |

### Result

| | **necessity** | **sufficiency** | **completeness** | **consistency** |
|---|---|---|---|---|
| **normative** | mandatory privacy guardrail | evidence-backed protection | exhaustive boundary capture | consistent private marking |
| **operative** | executable registry control | verified secret indirection | end-to-end safeguard coverage | deterministic handling behavior |
| **evaluative** | reviewable threat finding | auditable exposure evidence | complete readiness gate | coherent risk disposition |

## Matrix D - Objectives (3x4)

### Construction: Orientation plus resolution-transformed requirements

Each interpreted cell combined the canonical orientation contributor with the requirement-resolution contributor, then used the three-step interpretation procedure.

| Cell | Axis anchor | Projection summary | Centroid |
|---|---|---|---|
| D[normative, guiding] | rule direction | prescriptive and guardrail contributors converge on bounded policy direction | bounded policy direction |
| D[normative, applying] | rule practice | mandatory and evidence-backed contributors converge on enforced privacy practice | enforced privacy practice |
| D[normative, judging] | rule determination | compliance and boundary-capture contributors converge on an exposure decision gate | exposure decision gate |
| D[normative, reviewing] | rule audit | regulatory and marking contributors converge on protected-content audit | protected-content audit |
| D[operative, guiding] | executable direction | procedural and registry-control contributors converge on local handling route | local handling route |
| D[operative, applying] | executable practice | practical and indirection contributors converge on controlled registry use | controlled registry use |
| D[operative, judging] | executable assessment | performance and safeguard-coverage contributors converge on safeguard performance check | safeguard performance check |
| D[operative, reviewing] | executable audit | process and deterministic-behavior contributors converge on workflow audit trail | workflow audit trail |
| D[evaluative, guiding] | review direction | value and threat-finding contributors converge on privacy value framing | privacy value framing |
| D[evaluative, applying] | review practice | merit and exposure-evidence contributors converge on risk-based application | risk-based application |
| D[evaluative, judging] | review determination | worth and readiness-gate contributors converge on assurance determination | assurance determination |
| D[evaluative, reviewing] | review audit | quality and risk-disposition contributors converge on review quality signal | review quality signal |

### Result

| | **guiding** | **applying** | **judging** | **reviewing** |
|---|---|---|---|---|
| **normative** | bounded policy direction | enforced privacy practice | exposure decision gate | protected-content audit |
| **operative** | local handling route | controlled registry use | safeguard performance check | workflow audit trail |
| **evaluative** | privacy value framing | risk-based application | assurance determination | review quality signal |

## Matrix K - Transpose of D (4x3)

### Construction: K(i,j) = D(j,i)

### Result

| | **normative** | **operative** | **evaluative** |
|---|---|---|---|
| **guiding** | bounded policy direction | local handling route | privacy value framing |
| **applying** | enforced privacy practice | controlled registry use | risk-based application |
| **judging** | exposure decision gate | safeguard performance check | assurance determination |
| **reviewing** | protected-content audit | workflow audit trail | review quality signal |

## Matrix G - Truncation of B (3x4)

### Construction: remove wisdom row from B

### Result

| | **necessity** | **sufficiency** | **completeness** | **consistency** |
|---|---|---|---|---|
| **data** | essential fact | adequate evidence | comprehensive record | reliable measurement |
| **information** | essential signal | adequate context | comprehensive account | coherent message |
| **knowledge** | fundamental understanding | competent expertise | thorough mastery | coherent understanding |

## Matrix X - Verification (4x4)

### Construction: Dot product K with G

Each interpreted cell used Matrix K as the objective-facing orientation and Matrix G as the truncated evidence grid. Projection summaries below record the three-step interpretation in compact form.

| Cell | Axis anchor | Projection summary | Centroid |
|---|---|---|---|
| X[guiding, necessity] | direction need | policy, route, and value contributors converge on a source check for private-boundary facts | policy evidence gate |
| X[guiding, sufficiency] | direction warrant | policy, route, and value contributors converge on adequate proof for private handling | adequate policy proof |
| X[guiding, completeness] | direction wholeness | policy, route, and value contributors converge on complete traceability | complete policy trace |
| X[guiding, consistency] | direction stability | policy, route, and value contributors converge on stable user-facing messages | stable policy message |
| X[applying, necessity] | practice need | privacy practice, registry use, and risk contributors converge on required implementation input checks | implementation input check |
| X[applying, sufficiency] | practice warrant | practice, registry, and risk contributors converge on usable control evidence | usable control evidence |
| X[applying, completeness] | practice wholeness | practice, registry, and risk contributors converge on full workflow proof | complete workflow proof |
| X[applying, consistency] | practice stability | practice, registry, and risk contributors converge on consistent runtime signal | consistent runtime signal |
| X[judging, necessity] | determination need | exposure gate, safeguard check, and assurance contributors converge on decision evidence | decision evidence check |
| X[judging, sufficiency] | determination warrant | exposure gate, safeguard check, and assurance contributors converge on measured exposure basis | measured exposure basis |
| X[judging, completeness] | determination wholeness | exposure gate, safeguard check, and assurance contributors converge on complete risk proof | complete risk proof |
| X[judging, consistency] | determination stability | exposure gate, safeguard check, and assurance contributors converge on coherent finding message | coherent finding message |
| X[reviewing, necessity] | audit need | protected-content, workflow, and quality contributors converge on audit input checks | audit input check |
| X[reviewing, sufficiency] | audit warrant | protected-content, workflow, and quality contributors converge on adequate audit evidence | adequate audit evidence |
| X[reviewing, completeness] | audit wholeness | protected-content, workflow, and quality contributors converge on complete review trace | complete review trace |
| X[reviewing, consistency] | audit stability | protected-content, workflow, and quality contributors converge on consistent review message | consistent review message |

### Result

| | **necessity** | **sufficiency** | **completeness** | **consistency** |
|---|---|---|---|---|
| **guiding** | policy evidence gate | adequate policy proof | complete policy trace | stable policy message |
| **applying** | implementation input check | usable control evidence | complete workflow proof | consistent runtime signal |
| **judging** | decision evidence check | measured exposure basis | complete risk proof | coherent finding message |
| **reviewing** | audit input check | adequate audit evidence | complete review trace | consistent review message |

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

### Construction: Dot product X with T

Each interpreted cell used Matrix X as the verification surface and Matrix T as the evaluation conditioner. Projection summaries below record the three-step interpretation in compact form.

| Cell | Axis anchor | Projection summary | Centroid |
|---|---|---|---|
| E[guiding, data] | direction fact | policy-evidence and essential-fact contributors converge on sourceable privacy fact | privacy fact source |
| E[guiding, information] | direction signal | policy-proof and essential-signal contributors converge on boundary context | boundary context source |
| E[guiding, knowledge] | direction understanding | policy-trace and understanding contributors converge on control understanding | control understanding source |
| E[guiding, wisdom] | direction discernment | policy-message and discernment contributors converge on principled privacy stance | principled privacy stance |
| E[applying, data] | practice fact | implementation-check and essential-fact contributors converge on implementation input evidence | implementation input source |
| E[applying, information] | practice signal | control-evidence and context contributors converge on handling context | handling context source |
| E[applying, knowledge] | practice understanding | workflow-proof and expertise contributors converge on registry understanding | registry understanding source |
| E[applying, wisdom] | practice discernment | runtime-signal and judgment contributors converge on protected practice stance | protected practice stance |
| E[judging, data] | determination fact | decision-evidence and essential-fact contributors converge on exposure fact basis | exposure fact basis |
| E[judging, information] | determination signal | exposure-basis and context contributors converge on risk context | risk context basis |
| E[judging, knowledge] | determination understanding | risk-proof and mastery contributors converge on assurance understanding | assurance understanding basis |
| E[judging, wisdom] | determination discernment | finding-message and reasoning contributors converge on principled risk stance | principled risk stance |
| E[reviewing, data] | audit fact | audit-input and essential-fact contributors converge on audit fact basis | audit fact basis |
| E[reviewing, information] | audit signal | audit-evidence and context contributors converge on review context | review context basis |
| E[reviewing, knowledge] | audit understanding | review-trace and understanding contributors converge on quality understanding | quality understanding basis |
| E[reviewing, wisdom] | audit discernment | review-message and reasoning contributors converge on principled review stance | principled review stance |

### Result

| | **data** | **information** | **knowledge** | **wisdom** |
|---|---|---|---|---|
| **guiding** | privacy fact source | boundary context source | control understanding source | principled privacy stance |
| **applying** | implementation input source | handling context source | registry understanding source | protected practice stance |
| **judging** | exposure fact basis | risk context basis | assurance understanding basis | principled risk stance |
| **reviewing** | audit fact basis | review context basis | quality understanding basis | principled review stance |

---

## Matrix Summary

### Matrix C

| | **necessity** | **sufficiency** | **completeness** | **consistency** |
|---|---|---|---|---|
| **normative** | binding privacy boundary | adequate control basis | complete handling map | stable terminology |
| **operative** | actionable protection path | workable registry evidence | full workflow coverage | repeatable safeguard behavior |
| **evaluative** | reviewable risk basis | adequate audit trail | complete assurance picture | coherent acceptance judgment |

### Matrix F

| | **necessity** | **sufficiency** | **completeness** | **consistency** |
|---|---|---|---|---|
| **normative** | mandatory privacy guardrail | evidence-backed protection | exhaustive boundary capture | consistent private marking |
| **operative** | executable registry control | verified secret indirection | end-to-end safeguard coverage | deterministic handling behavior |
| **evaluative** | reviewable threat finding | auditable exposure evidence | complete readiness gate | coherent risk disposition |

### Matrix D

| | **guiding** | **applying** | **judging** | **reviewing** |
|---|---|---|---|---|
| **normative** | bounded policy direction | enforced privacy practice | exposure decision gate | protected-content audit |
| **operative** | local handling route | controlled registry use | safeguard performance check | workflow audit trail |
| **evaluative** | privacy value framing | risk-based application | assurance determination | review quality signal |

### Matrix K

| | **normative** | **operative** | **evaluative** |
|---|---|---|---|
| **guiding** | bounded policy direction | local handling route | privacy value framing |
| **applying** | enforced privacy practice | controlled registry use | risk-based application |
| **judging** | exposure decision gate | safeguard performance check | assurance determination |
| **reviewing** | protected-content audit | workflow audit trail | review quality signal |

### Matrix G

| | **necessity** | **sufficiency** | **completeness** | **consistency** |
|---|---|---|---|---|
| **data** | essential fact | adequate evidence | comprehensive record | reliable measurement |
| **information** | essential signal | adequate context | comprehensive account | coherent message |
| **knowledge** | fundamental understanding | competent expertise | thorough mastery | coherent understanding |

### Matrix X

| | **necessity** | **sufficiency** | **completeness** | **consistency** |
|---|---|---|---|---|
| **guiding** | policy evidence gate | adequate policy proof | complete policy trace | stable policy message |
| **applying** | implementation input check | usable control evidence | complete workflow proof | consistent runtime signal |
| **judging** | decision evidence check | measured exposure basis | complete risk proof | coherent finding message |
| **reviewing** | audit input check | adequate audit evidence | complete review trace | consistent review message |

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
| **guiding** | privacy fact source | boundary context source | control understanding source | principled privacy stance |
| **applying** | implementation input source | handling context source | registry understanding source | protected practice stance |
| **judging** | exposure fact basis | risk context basis | assurance understanding basis | principled risk stance |
| **reviewing** | audit fact basis | review context basis | quality understanding basis | principled review stance |
