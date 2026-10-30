# From Design Intent to Defensible Engineering Work

**The SWBPIPE Thesis**

*Why agent participation must become particular to an engineering practice*

## Abstract

Piping design begins before the conditions of a final calculation are settled.
A route, support arrangement or equipment connection expresses an intention
under developing constraints, and analysis can reveal consequences that change
that intention. A useful design environment supports this inquiry as well as
the calculation.

SWBPIPE connects design intent to explicit physical models, analytical
idealizations and meaningful operations. Agents can extend the range and pace
of investigation. The person's judgment shapes what is modeled, how results
are understood and what they will stand behind.

The application gives Chirality an engineering expression: its objects have
physical referents, its abstractions omit details that matter, and plausible
numerical results may be wrong. Common operations, explicit transformations,
examined numerical methods and contextual handoff preserve the relationships
needed to assess the work.

The approach must earn its value through use. Source-level cases support parts
of the architecture; complete App–SWBPIPE integration and transfer to another
practitioner remain to be demonstrated.

This essay develops the evaluative view of the work introduced in the
[shared introduction][S1]. SWBPIPE is an instance of domain software in which
Root's normative orientation and the App's operative hypotheses meet physical
referents and engineering consequences. Evaluation asks whether the design
environment is adequate throughout engineering inquiry. The
[support-arrangement example][S2] connects the three views through an
illustrative revision.

## 1. Why the design environment matters

A piping route is never merely a line waiting for a stress value. Its position
can concern equipment access, drainage, a construction sequence, a supportable
structure or a relationship to a neighbouring system. An arrangement that
performs favourably in one analysis may be impractical to build or maintain.
Conversely, an initially convenient layout may impose loads or movements that
change what counts as a useful design.

These relationships develop through work: early information is incomplete,
assumptions become more explicit, new constraints arrive, and calculation
reveals an effect that was not obvious in the initial arrangement. An
environment that requires every decision to be settled before any useful
examination is possible can make this development unnecessarily difficult. An
environment that silently completes the missing decisions can make it
deceptively easy.

**Why and under what conditions can an open,
agent-enabled piping design environment expand useful engineering inquiry
while preserving the meaning, evidence and human responsibility needed for
subsequent reliance?**

SWBPIPE supports early route and support design: forming alternatives,
examining mechanical consequences and preparing a coherent handoff.[P1]

There are credible alternatives: a practitioner can work directly in an
established analysis tool, combine a modeling application with scripts, or use
a general coding agent to manipulate files. SWBPIPE must offer a benefit beyond
the availability of an agent or an additional solver. Its case is strongest
where repeated translation among intent, geometry, assumptions, calculations
and review otherwise consumes effort or loses meaning. Where an existing tool
already serves the activity well, replacing it may add little value.

Design iteration, downstream validation and professional acceptance serve
different purposes. That distinction does not excuse weak mechanics: wrong
feedback can redirect a design long before final review.[P1]

The operation engine provides a concrete example of a related problem. Native
and browser editing once used separate Rust and TypeScript implementations,
and tests of each did not establish that their validation and model hashes
agreed. The browser path could therefore supply reassuring evidence about a
different implementation. The decision to use the same Rust crate through a
WebAssembly build addressed that source of divergence.[P2]
[Section 4.1](#41-common-semantics-must-reach-the-exercised-engine) examines the
change and its limits. A useful design environment needs the work being
examined to stay connected to the work actually done.

## 2. Shared orientation, engineering subject

Four perspectives meet in piping design. **Ontology** distinguishes physical
intent, analytical idealization, calculation and commitment. **Epistemology**
examines grounds, assumptions and limits. **Praxeology** follows intent through
change, examination and further action. **Axiology** considers usefulness,
consequences and the costs of control.

Moving a support changes an object, rests on assumptions, affects calculated
behaviour and serves a purpose. A favourable result matters in relation to
those conditions. None of these questions belongs exclusively to this essay
or to one of the other views; they correct one another throughout the work.

Evaluation returns findings to what produced them. A numerical defect calls for
implementation repair; an unsuitable idealization calls for model revision;
an impractical arrangement calls for a different design question. Findings can
also challenge a prior purpose, such as addressing access alone when a newly
recognized construction condition makes that framing inadequate. An agent may
investigate and revise within its assignment or adopted policy, but a change to
an adopted commitment remains with the authority that holds it. This iterative
relationship connects domain experience back to methods and normative purposes.

Agents extend **reckoning** through analysis, inference, comparison, planning,
criticism and adaptive action. **Judgment** is the person's situated and
committed engagement with what the work means, what matters and what they will
stand behind. It shapes which alternatives deserve investigation, which
assumptions are tolerable and when further evidence is needed. A final approval
or signature is one possible expression of that engagement; it does not
exhaust it.[R2]

Meaningful human control requires that the proposed change, its basis and its
consequences be available for examination.

## 3. A model is an interpreted account of a physical undertaking

The physical model is an account of the proposed arrangement and can carry
geometry, components, supports, materials, loads, constraints, provenance and
unresolved assumptions. Those objects remain connected to design intent even
where the solver has no direct use for them. An access requirement or a
construction constraint may determine whether a calculated alternative is
worth considering.[P1]

The analytical model selects and idealizes aspects of that account for a
particular calculation. Nodes, elements, restraints and constitutive properties
make a mechanical question tractable. Their explicitness is useful precisely
because they do not constitute the entire physical undertaking. A numerical
answer can be correct for that idealization and still inadequate for the
intended situation.

The handoff model performs a further interpretation. A receiving tool has its
own concepts, supported behaviours and representations, so transferring a model
involves correspondence, possible approximation and possible loss.
Treating export as a change of file extension would obscure those questions.

These distinctions explain why SWBPIPE needs a persistent physical account
rather than allowing a solver input file to become the only meaning of the
project. A designer should be able to ask what an analytical object represents,
what was omitted and which design information remains relevant outside the
calculation. A stress engineer should be able to examine that relationship
without reconstructing the designer's intent from a result table.

The PRD calls the deterministic model graph the source of truth for the
application. That is a claim about authority within the software: it identifies
which recorded state governs an operation. It does not establish that the
recorded graph is a true or sufficient account of the physical installation. The
application can preserve and check internal consistency while the person remains
responsible for determining whether the represented conditions apply.[P1]

Stable identity helps this inquiry across changes. If a support, element or load
retains an identifiable relation to its earlier state, a comparison can concern
what changed rather than merely which numbers differ. Retained model states
and analysis runs are meant to keep the subject of a claim recoverable: which
model, data, method and settings produced the result under discussion.
Their value is lost if they become an additional administrative account that
cannot be related back to the current engineering question.

### 3.1 Missing input is an engineering finding

No-silent-defaults is often expressed as a validation rule. The deeper reason
for it is that an unknown condition and an assumed condition can produce
different engineering conclusions. Supplying a plausible value may make a solver
run while making the grounds of its answer harder to inspect.

The internal solver-boundary adapter requires explicit properties, quantities,
units, dimensions and provenance. Its tests expect missing shear modulus to
leave the property binding absent, unrecognized loads to produce diagnostics,
and invalid orientations and reversed load spans to be refused.[I1][T1]
This internal path does not establish the behaviour of every graphical or
agent route.

The useful behaviour is that an unanswered question is preserved: other parts
of the model can remain available while the missing ground is exposed. This
gives the person and agent a concrete matter to investigate rather than
a successful calculation whose assumptions are concealed.

Explicitness has costs: requiring repeated manual entry of facts already
available elsewhere can discourage use and produce new errors. Assistance can
therefore locate sources, suggest a value with its basis or identify a suitable
library binding. The essential distinction is whether the value and its source
remain inspectable and appropriately adopted, rather than whether automation
was involved in obtaining it.

## 4. Agent capability becomes useful through domain meaning

An agent can suggest that a support should move. To take part in the design,
it must also establish which support, in which model state, for what intended
effect and under which constraints. The application must make the resulting
operation inspectable and keep a suggestion, a validated change and an applied
result distinct from one another.

That is why the common operation model is more important than a convenient
command interface. It permits a person's action and an agent's action to bear
on the same domain meanings. Different interfaces may serve different forms
of attention, but an object should not acquire different validation rules
merely because an agent rather than a person addresses it.

The agent can then explore an understandable design space. It can examine a
constraint, prepare an alternative and compare consequences with an identified
basis. It can also discover that the requested action is unavailable, that an
input is missing or that the proposed change is stale. These are meaningful
outcomes, not failures to produce sufficiently persuasive prose.

Autonomy is a question for the particular operation and its authority.
A direct application permitted in one context does not imply that every
engineering act is available to an external agent. Likewise, preserving human
judgment does not logically require the person to issue every low-level command.
The current project contracts and owner decisions define the actual boundary;
this thesis explains why such a boundary needs a domain subject.[P1][A2]
An assignment or adopted operation policy supplies authorization. Host
capabilities and permissions determine what can be done through that host;
their availability does not itself authorize a change.

### 4.1 Common semantics must reach the exercised engine

The June operation-engine decision provides a useful counterexample. SWBPIPE
had a Rust operation engine for native use and a separate TypeScript mirror
for browser use. The browser route gave agents and developers a fast way to
exercise editing, but tests of each engine did not establish that their
validation and canonical model hashes agreed. The accessible examination path
could therefore give reassuring evidence about a different implementation.[P2]

The decision replaced the mirror with a WebAssembly build of the same Rust
crate. The current TypeScript code routes an operation to the native command
or the WebAssembly build and contains no independent validation or application
semantics. The Rust engine separates validation from application, preserves
the input model and returns a changed model only through the application path.
Its corpus tests compare semantic outcomes, resulting documents and model
hashes, including the absence of an applied result for a refused operation.
[I2][I3][T2]

Reusing the engine reduced one source of divergence while preserving the
accessible browser examination path.

Here operation validation asks whether an edit meets defined model-operation
conditions; it does not validate the physical model for engineering use.
Implementation verification examines specified behaviour; numerical verification
examines the calculation against suitable independent reference grounds within
its stated scope.

Model applicability asks whether the idealization represents the intended
conditions. Engineering adequacy asks whether the resulting design and evidence
serve the work. Acceptance concerns what a person accepts within their
authority; professional authentication concerns their attributable commitment to
accountable reliance within a defined scope and purpose. Neither is a solver
output. These relationships can inform each other without becoming one ascending
software status.

A shared defect can affect every consumer of that engine. Reference evidence
and examination of the underlying assumptions remain necessary. Native/browser
consistency does not establish a completed external-agent integration.

## 5. Serious mechanics before professional reliance

SWBPIPE's separation between internal analysis and professional reliance could
be misunderstood as permission for approximate or weak calculation, but the
opposite follows from its design purpose. A result used to choose a route or
support concept can redirect substantial work. If that result is wrong in a
way the user cannot see, later validation may expose the problem only after
the design has become costly to change.

The analytical engine must therefore provide useful grounds for design
iteration. Determinism permits a result to be reproduced under an identified
basis. Numerical diagnostics can expose an ill-conditioned or unsupported case.
Reference comparisons and carefully selected tests can challenge the
implementation. None of these alone establishes that the physical idealization
is appropriate, that the loads are complete or that the proposal is suitable
for project reliance.

A serious engine also needs to distinguish kinds of uncertainty. Missing data,
a numerical formation defect, ill-conditioning and an unsupported mechanical
model call for different responses. Treating them all as an undifferentiated
warning leaves the practitioner unable to decide what to investigate. Treating
them all as ordinary successful results is worse.

### 5.1 Small bends expose the difference between repair and reassurance

The curved-element work supplies a concrete case. Small-angle expressions can
suffer cancellation even when the intended mechanics are meaningful. The
current Design account describes a stable half-angle form, explicit geometry
formation and a scaled inversion. The related short-arc tests compare against
frozen high-precision references, examine ordinary and large-coordinate
representations, and check rigid-motion behaviour. A separate set fixes the
formation bits for selected cases.[P3][I4][T3]

A visually reasonable bend can fail numerically for reasons unrelated to its
engineering intent. The formulation and its examination must expose that gap.

The same Design account records a remaining distinction. A short, stiff bend
beside much longer, softer elements can produce genuine ill-conditioning of
the global solve. Improving the local formulation does not remove that relation.
Some cases remain Sensitive or are refused; a further formulation change or
acceptance of a stated limitation is a separate matter.[P3]

Repair should improve the relationship between the implementation and the
intended problem. A useful limit explains why a result cannot carry the desired
inference and what might change that situation.

The mutation discussion in the same source provides a second lesson. A proposed
mutant was ruled equivalent under stated admissibility conditions, and the
former kill tests were removed. More tests would not create a meaningful
distinction where the premise required for that distinction no longer held. The
account illustrates why it is important to examine the claim a test is meant to
support, rather than equating a larger test inventory with stronger evidence. It
is a reported design disposition, not a fresh proof performed here.[P3]

## 6. Open mechanics and particular engineering knowledge

A public mechanics engine and an organization's engineering knowledge are
subject to different conditions of circulation and use. Algorithms, schemas and lawful
examples can be shared and examined. Project data, licensed sources, material
libraries, owner requirements and particular rule definitions may have
restrictions that the public project cannot erase.

Keeping these contributions separate serves more than repository hygiene: it
makes their roles available for inspection. The mechanics explains how a model
is calculated, user-supplied data identifies the conditions applied, and a rule
pack expresses an evaluation under a particular basis. The resulting conclusion
can then be questioned at the appropriate level instead of appearing as an
opaque pronouncement from the software.[P1][P4]

This separation also permits organizations to differ without pretending
that one public default captures their practice. The cost is real: users must
supply, check and maintain the relevant data. If that work is excessively
burdensome, the architecture may remain principled but unattractive. Useful
import, source identification and review support are therefore part of the
application's practical case, rather than optional administration added after
the solver is complete.

The current policy separates public redistribution from runtime handling. It
permits some owner-configured use of private data while retaining restrictions
on publication and third-party content.[P4]

Domain retrieval introduces a related epistemic problem. A passage can be
useful for concepts and terminology without being an admissible numerical
reference. The project's DEC-043 constraint concerns the corpus's extracted
equation artifacts: their unreviewed extraction cannot ground a physics model
or engineering decision. It does not dismiss the corpus's conceptual prose.
An agent-enabled design environment needs to preserve such distinctions when
bringing retrieved material into a candidate design.[P5]

Retrieved material needs a source, a standing and relevance to the question.
The resources carry recorded material, not a practitioner's whole understanding.
Even a suitable passage has to be interpreted in relation to the current
geometry, loads, conditions and purpose. The joined Domains research activity
remains prospective.[A2]

## 7. Design inquiry, values and human judgment

An agent can enlarge the set of alternatives a person can investigate. It can
make a comparison inexpensive enough to try, expose a missing constraint or
prepare a candidate that the person would not otherwise have had time to
construct. This is a substantial form of assistance even when the agent does
not determine which design should be pursued.

The value is not proportional to the number of candidates. Unexamined
alternatives can consume attention, and a ranking can conceal the values built
into its objective. A route that minimizes one result may worsen access,
construction, maintainability or another project's interface. Some conditions
are measurable, some are represented only partially, and some become apparent
through situated experience.

Applicable obligations and safety constraints condition the admissible choices;
negotiable preferences help compare alternatives within them. A construction
sequence, equipment access, maintenance task or interface to another system can
change what counts as useful. Unknown load conditions or an unresolved support
assumption must remain unknown or be explicitly adopted as assumptions; a
ranking cannot settle them by assigning convenient values. There is no universal
objective that converts these distinct concerns into engineering adequacy.

In the illustrative [support revision][S2], inquiry starts from an access
concern in the current arrangement. Any mechanical comparison must therefore be
read with that purpose, the physical arrangement and its unresolved conditions.
Discovering an additional construction or maintenance condition can require
representing an omitted constraint and trying another arrangement. It can also
broaden the initial access inquiry into finding a support concept that can be
built, accessed and maintained under the applicable constraints. The example
supplies a connecting argument, not evidence that this joined activity has
already been implemented or exercised.

Judgment enters before the final choice: it shapes what is modeled, what is
left unresolved, what deserves comparison and how much evidence is appropriate.
The person can also change the question after seeing a result. An agent's
reckoning can assist each step without making the person's commitment an
output of the optimization.

Comparable alternatives help the person inspect why a candidate was proposed,
which assumptions it uses and what its consequences might be. Selecting an
autonomy level does not settle the worth of the design.[A2]

This is also why constant human confirmation is not a sufficient account of
responsibility. A person can approve many changes without understanding them.
Repeated low-value prompts can diminish attention to a choice that matters.
Conversely, bounded continuing authority can permit useful exploration while
leaving significant commitments with the person. Whether a particular
arrangement succeeds is a question for the actual activity and its evidence,
not something guaranteed by either an approval button or an autonomy setting.

## 8. Handoff extends the undertaking beyond the application

A design environment should improve the next participant's ability to work.
It is less useful if its internal results cannot be related to the receiving
tool's objects, or if the receiving professional must reconstruct all
assumptions before beginning review.

Handoff quality therefore concerns meaning as well as format. A receiving
party needs to know which model and analysis the export represents, how entities
were mapped, which behaviours were omitted or approximated, and what remains
unresolved. The transfer should preserve enough context to examine those
questions without claiming to settle them.[P1]

The provider-neutral export implementation illustrates this bounded purpose.
It assembles a deterministic envelope carrying identity, hashes, units, source
references, mappings, warnings and unresolved assumptions. Its own description
excludes invoking external solvers, interpreting all commercial formats or
ingesting downstream results. Defined tests examine missing unit metadata,
inconsistent hash or unit bases, unsupported target behaviour and prohibited
authority metadata.[I5][T4]

The adopted product pathway includes validation in an accepted professional tool
and competent human review before project reliance. That is a project
commitment, not a theorem that a named tool guarantees adequacy. Different tools
can share assumptions, and a result from the receiving tool can still concern an
incorrectly specified problem. The professional's judgment must engage with the
model and its use, not merely with the provenance of the software that
calculated it. [P1][P6]

Interoperability also has a practical cost: mappings require maintenance as
source and target capabilities evolve, and unsupported cases need usable
responses.
If export loses too much meaning or requires repeated manual reconstruction,
the promised advantage of an early design environment may disappear. Handoff
must therefore be evaluated as part of the connected engineering activity.

## 9. How Chirality proliferates through applications

SWBPIPE is a test of transfer because it imposes requirements that a workflow
authoring exemplar cannot settle. The application must identify meaningful
engineering objects, preserve quantities and assumptions, expose useful
operations and distinguish several kinds of result. A generic agent panel
supplies none of those things by itself.

What can transfer is a set of relationships and some means for sustaining them:
a person directs work; an agent can perceive the relevant objects;
action is bounded by application semantics and authority; outcomes remain
inspectable; methods can develop through use. Shared components and interface
contracts can reduce repeated work where those relationships align. The host
still supplies its particular domain and an adequate implementation.

A capability catalog is useful in that role: it can give human and agent
interfaces a common account of an operation's purpose, inputs, preconditions,
effects and returned standing. The common account does not automatically
provide a good interface, a correct solver or a recovery path. The App contract
explicitly leaves the extent of automatic generation and remaining adapter
work to be established by an actual extension.[A2]

A complete App–SWBPIPE adjustment and checking journey remains unestablished.
An external read must concern the state later acted upon; an interrupted request
must have an intelligible outcome; host authority must still apply.[A2]

This gives proliferation a concrete meaning. Another application has joined
the pattern when the approach makes useful participation possible in its own
practice, at a defensible cost. Sharing a runtime is neither a necessary nor a
sufficient condition. A new host may reuse a panel or protocol while retaining
its own process topology, domain engine and operation policy.

There are two distinct transfer questions. Another domain tests whether the
shared means remain useful under different objects and consequences. Another
professional tests whether the application can be understood and used without
its originating author's continuous interpretation. Success at one does not
establish the other. SWBPIPE's continued development can inform both, but the
current repository does not settle them.

## 10. Re-derivation preserves purpose through changing means

The App's successive implementations make an important method visible: a
product can be reconsidered from its current purpose and requirements instead
of treating its earlier code as the permanent definition of what it must be.
A PRD gives that reconsideration a shared reference. Experience raises
questions the PRD did not anticipate, and the owner interprets what should
remain a commitment.[A1]

The same method applies to an engineering application with a demanding
qualification. A new implementation must carry forward the meaning and evidence
that remain useful. Replacing a solver formulation or operation engine is not
an opportunity to forget why its difficult cases mattered. The curved-element
work and common-engine decision illustrate revision with a specific object:
a numerical defect or divergence was identified, the means changed, and the
relevant claims and limits remained open to examination.

A PRD is therefore not a compiler from which SWBPIPE can be regenerated with
its engineering adequacy intact. It cannot fully encode the practitioner's
experience, every operational assumption or all the failures learned through
use. Re-derivation is an interpretive design activity involving current
commitments, existing implementation, examination evidence and human judgment.
Its success depends on retaining what was learned as well as on producing a new
product.

This also limits the appeal of perpetual rebuilding. An existing implementation
may be the most reliable and economical means available. A rewrite is warranted
by what it can improve, not by a general preference for novelty or a stronger
model's ability to generate it. Every reconstruction risks losing tacit
knowledge, known edge cases and familiar interactions.

So the continuity that matters is more than a preserved document or code
history. Later participants need access to the current technical condition
and the reasons it bears on the next change. Deliverable design notes and
applicable reference cases can serve that purpose. Their value depends on
being discoverable and useful at the point of work, rather than accumulating
as a second history nobody consults.

## 11. What the evidence supports, and what could overturn the case

The argument would be weakened if the design environment consistently required
more reconstruction than the tools it accompanies; if agent proposals expanded
review work without producing useful alternatives; if important physical
conditions could not be expressed; or if internal evidence made users
unreasonably confident in an unsuitable idealization. It would also be weakened
if supposed common semantics concealed different behaviour at an actual
human–agent integration boundary.

The distinction between design iteration and later reliance must be examined
in practice; disclaimers alone cannot sustain it. People must be able
to understand what the result supports, recognize material limitations and
carry the design into the intended next activity. An interface that regularly
invites stronger conclusions than its evidence supports would count against
the architecture despite correct boundary text.

Useful evaluation would examine complete activities and their costs: whether
a practitioner can form and revise an intended model; whether another person
can inspect the assumptions and reproduce the relevant analysis; whether agent
assistance improves the alternatives considered; whether an error that matters
is exposed in time; and whether handoff preserves enough meaning to continue.
Those questions require actual use and comparison, not merely larger counts
of generated models, tests or merged changes. The relevant comparison includes
preparation, review, repair, construction and maintenance implications, and
attention spent recovering meaning at handoff. An apparent saving in modeling
can be a loss elsewhere in the work.

Such findings must be able to alter the next activity: representations can gain
an omitted relationship, methods can change the order or scope of examination,
and the responsible person can reconsider prior purposes. Additional process
is justified only where it helps that correction. Evaluation need not create a
new approval sequence or permanent record for every question.

The owner supplies engineering experience and sustained direction. A second
practitioner's use would test whether the application remains understandable
without that continuing interpretation.

## 12. Conclusion

SWBPIPE connects forming a design with understanding what it entails. Useful
early investigation preserves physical intent, analytical assumptions, the
grounds of results and the conditions of further use.

Agents can extend what the person investigates and does. Their value depends
on meaningful access to the domain and inspectable consequences. Judgment shapes
the work throughout: which question matters, which assumptions are
adopted and what the person will stand behind.

Chirality's means must earn their usefulness here. Numerical methods, model
operations and handoff need evidence appropriate to engineering practice.
Neither a shared vocabulary nor a common runtime can establish that adequacy
in advance.

The application should make design inquiry more productive while keeping its
grounds available. Actual use, connected application work and another
practitioner's experience will test how well it does so. Those findings return
to the App's operative means and Root's normative questions; the three views
remain one revisable argument. The [shared conclusion][S3] states what that
argument supports and what remains to be learned.

## Source basis and reading

Product sources, implementation and Design accounts are examined at
`a602024416ace9652f61bb2043889c307fd4e09d`. Tests were inspected rather than
rerun. Product requirements retain
their authority. Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

- **R1:** [Root thesis][R1]: why the shared means of work remain revisable.
- **R2:** [Chirality Framework][R2]: situated understanding, reckoning and
  judgment, and the four connected perspectives.
- **A1:** [App v4 thesis][A1]: method authoring as the first exemplar and the
  reasons for application-specific agent participation.
- **A2:** [App v4 host integration][A2], §§2–7 and 10–11: shared operation
  meanings, host responsibilities, unresolved generation scope and the limits
  of currently established integration.
- **D1:** [Shared conjecture][D1]: unchanged formal argument, agreed definitions
  and revised concise interpretation.
- **P1:** [SWBPIPE PRD][P1], §§1, 4–5, 8, 10, 13 and 15: product purpose,
  domain objects, model/action relations, comparisons and reliance pathway.
  These are commitments, not claims that every requirement is implemented.
- **P2:** [Operation-engine unification decision][P2]: historical design
  rationale; retired development procedures are not adopted by this thesis.
- **P3:** [Curved-element numerical integrity][P3]: current formation account,
  reference conditions, mutation disposition and remaining conditioning limits.
- **P4:** [IP and data boundary][P4], §§2–3 and 6: public-source limits,
  user-controlled data and the separate runtime-residency provision.
- **P5:** [SWBPIPE development entry][P5], knowledge-source constraint:
  DEC-043's distinction concerning extracted equation artifacts.
- **P6:** [Professional boundary][P6], §§3–6: distinctions among mechanics,
  user-rule evaluation, reports, handoff and professional reliance.
- **I1:** [Internal solver-boundary adapter][I1]: explicit analytical-model
  inputs, quantities, units, dimensions, provenance and refusal diagnostics.
- **T1:** [Adapter tests][T1] and [physical-to-analytical tests][T1B]: defined
  cases for omitted input, unsupported meaning and preservation through a
  transformation.
- **I2:** [Operation routing][I2]: native and WebAssembly routes to the same
  implementation.
- **I3:** [Operation engine][I3]: validation, application, model basis and
  limited operation records.
- **T2:** [Operation corpus tests][T2]: semantic outcomes, model identity and
  blocked applications; [canonical-hash cases][T2B].
- **I4:** [Platform formation cases][I4]: frozen bits and identified numerical
  scope, not a universal portability claim from this thesis.
- **T3:** [Short-arc tests][T3]: independent reference provenance, ordinary and
  large-coordinate cases, stiffness error and rigid-motion residuals.
- **I5:** [Provider-neutral export][I5]: context preservation and explicit
  exclusions from its function.
- **T4:** [Handoff export tests][T4]: defined examinations of units, mappings,
  source basis, unsupported behaviour and authority metadata.

[R1]: ../../../../docs/thesis/README.md
[R2]: ../../../../docs/thesis/CHIRALITY_FRAMEWORK.md
[A1]: ../../../chirality-app-v4/docs/thesis/THESIS.md
[A2]: ../../../chirality-app-v4/docs/HOST_INTEGRATION.md
[D1]: appendix_d_framework_s.md
[P1]: ../PRD.md
[P2]: ../architecture/adr/ADR-0001_operation_seam_engine_unification.md
[P3]: ../../execution/PKG-04/DEL-04-01/Work/curved-numerical-integrity.md
[P4]: ../IP_AND_DATA_BOUNDARY.md
[P5]: ../../loop/LOOP_INIT.md
[P6]: ../PROFESSIONAL_BOUNDARY.md
[I1]: ../../core/model_transform/physical_to_analytical/_solver_boundary_adapter.py
[T1]: ../../tests/test_analytical_solver_boundary_adapter.py
[T1B]: ../../tests/test_physical_to_analytical_transform.py
[I2]: ../../apps/desktop/src/services/operationService.ts
[I3]: ../../core/model_operations/operation_applier/src/lib.rs
[T2]: ../../core/model_operations/operation_applier/tests/contract_corpus.rs
[T2B]: ../../core/model_operations/operation_applier/tests/canonical_hash_parity.rs
[I4]: ../../core/solver/curved_bend/src/platform_bits_tests.rs
[T3]: ../../core/solver/curved_bend/src/short_arc_tests.rs
[I5]: ../../core/handoff/exporter/workflow.py
[T4]: ../../tests/test_handoff_export_workflow.py

[S1]: ../../../../docs/thesis/INTRODUCTION.md
[S2]: ../../../../docs/thesis/WORKED_EXAMPLE.md
[S3]: ../../../../docs/thesis/CONCLUSION.md
