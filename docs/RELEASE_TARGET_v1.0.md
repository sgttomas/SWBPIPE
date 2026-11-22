---
name: swbpipe-release-target-v1-0
description: Set the v1.0 capability target, verification criteria and reserved decisions.
metadata:
  kind: instruction
  surface: requirements
---
# SWBPIPE v1.0 (publication-ready): release target

**Status:** PRD companion (`docs/PRD.md` §24 V1). It is the target for the
first publication-ready release, accepted by the owner on 2026-10-28. It
comprises the direction, the benchmark method and the exit criteria. The exit
criteria are the definition of done. Capability and status are not kept here:
each criterion names the deliverables that own it, and their `ScopeOfWork.md`
states what each commits to and what is undecided. Meeting the exit criteria
is evidence for the owner's release decision, not the decision: acceptance,
issuance and release remain the owner's acts (F-PIP-3). SWBPIPE computes mechanics from recorded inputs and evaluates
the user's own rule packs; it does not certify, approve or establish code
compliance. Standard claim fence applies (claims taxonomy per DEC-081).

Paths are relative to the repository root. **PF** is the pressure-free solve
route (a model with no `pressure_contract`); **EX** is the exact pressure route
(`3.0.0/exact_pressure_v3`). Labels such as `RT-A7` in code and records name rows of
an earlier capability table, removed from this file on 2026-11-10 and kept in
Git history; the owning deliverable holds the current account. Labels OC-1 to
OC-10 in records name ten choices the owner settled on 2026-10-28; Direction
states what each decided.

## Direction

**Target.** Development is a hill climb to the first publication-ready release,
v1.0. Publication-ready means "a version good enough to publish engineering
results with".

- **The goal.** An app on par with or superior to CAEPIPE and other industry
  applications in modelling and user interface, given real thought for user
  experience and human factors in the design and taste in the aesthetics.
  CAEPIPE is the validation to SWBPIPE: a benchmark set compares non-code
  outputs across as wide a range as practicable (Benchmark method, below).
- **Scope for the climb.** Agent 0 may make a change or expansion in scope
  that clearly benefits the climb: one that fixes a mistake, plugs an obvious
  gap, or makes the product more accurate, truthful or capable without
  considerable risk of any sort. It decides such changes under the grant in
  `AGENTS.md` ("Posture for this phase"), including commitment and
  acceptance-criterion text and the reversal of earlier deferrals to v0.2. It
  approves feature proposals and reports what it adopted, deferred and why.
  The governing text records each decision and its reason.
- **What is published.** The user's engineer issues SWBPIPE's mechanics
  results under their own responsibility: support and equipment loads,
  displacements and fundamental stresses, and frequencies once modal analysis
  follows after v1.0 (Seismic, below). SWBPIPE does the design
  iteration and does not proclaim code compliance; the user goes to CAEPIPE
  for that.
- **Analyses.** Sustained, expansion, operating, hydrostatic test and static
  seismic, with the others exit criterion 1 lists as standard.
- **One route.** EX is extended and PF gets no pressure. Nothing necessary is
  omitted by this provided EX admits every family PF has. Pressure on PF is
  revisited only if EX fails the performance targets (exit criterion 9).
- **Seismic.** Static seismic is v1.0's seismic analysis: three-axis static
  seismic with SRSS and absolute-sum combination, and seismic anchor movement
  (exit criterion 1). Modal analysis and response-spectrum analysis, with
  user-entered spectra, modal and directional combination, a missing-mass
  correction and snubbers as a dynamic-only restraint (DEL-04-09, DEL-04-10;
  scope changes SCA-013 and SCA-014), follow after v1.0. The work already done
  on them stays in the repository and is not withdrawn. Agent 0 decided this on
  2026-11-20 under the grant in `AGENTS.md` and the owner's instruction:
  "Static seismic is enough for v1.0 but response spectrum will follow later
  so keep what's there." Response spectrum follows because CAEPIPE has it; the
  vendor's public manual describes response-spectrum analysis with mode and
  directional combination and missing mass
  ([dynamic analysis](https://docs.sstcae.com/tech_manual/dynamic_analysis.htm),
  [spectrums](https://docs.sstcae.com/users_manual/spectrums.htm),
  [load cases](https://docs.sstcae.com/users_manual/load_cases.htm)).
  `docs/RELEASE_LIMITATIONS_v1.0.md` LIM-035 states the omission.
- **Hangers.** v1.0 sizes hangers from the user's private catalogue (invented
  example records only) and produces a hanger table.
- **Hydrotest.** A flagged test condition with test pressure, test contents
  density and spring supports held, marked as a test case in results and
  reports. It is in v1.0 as a load case of its own, and no combination
  includes it (LIM-003). The owner, 20 November 2026: "Hydrotest is an
  essential part of stress analysis so it must be in." and "No combined
  envelope. They are separate cases."
- **Rule packs.** What is built is kept as a design aid. The extensions C2,
  E1, E2, E5 and E6 are deferred, and E3 (fatigue usage) and E4
  (pressure-design thickness) are dropped from the roadmap. User-rule outcomes
  are shown as "within / exceeds user limit" rather than pass/fail, so that no
  outcome reads as compliance.
- **Pressure effects.** Pressure elongation is a Should option for v1.0, off
  by default, applied only in the operating load cases the user names and
  stated on the result of each case that has pressure regions (DEL-04-07). Its
  correspondence with the Bourdon flag of a CAEPIPE model file is a benchmark
  hypothesis. The opening of a bend under pressure, which some texts also call
  the Bourdon effect, and pressure-dependent flexibility and SIF corrections
  are out of scope: users enter their own k and SIF.
- **Numerical claims.** For v1.0 the product makes no verified-bound claim it
  cannot support. A result says what the solver's checks found, and that
  those checks do not measure the error, rather than that it passed an
  integrity check; a converged nonlinear support state is stated as the
  solver's report, not as confirmed (exit criterion 18). Such a result is
  still read, checked against the user's rules, exported and reported. The full proof, the verified inverse-norm bound
  (DEL-04-01/REQ-015), follows after v1.0. Agent 0 decided this on 2026-11-20 under the grant in
  `AGENTS.md`; LIM-026 states the present defect.
- **Exact-route results under user release.** Route 010 admits exact-route results with exact source binding, exact sections and limitations, Diagnostics binding and producer item 9 (DEL-08-01, REQ-019).
- **Usability.** Agent 0 uses its own judgment on typical use cases an
  engineer would go through, and puts the desktop through them thoroughly
  before the owner is asked (exit criterion 8). The owner gets actual user
  testing after that.
- **Release path.** Straight to v1.0, on macOS first. Windows is revisited
  immediately after.
- **Licence.** MIT.
- **Benchmarks.** Agent 0 first screens the results with the tolerance profile
  and flags anything of merit; the owner then reviews the full record.
  Benchmark agreement with CAEPIPE is published, because the whole point is
  to feed the same models into CAEPIPE and expect the results to come out the
  same, validated to Code and to the reputation of CAEPIPE. Publication is
  subject to the owner's check of the CAEPIPE licence for benchmark clauses.
- **Review holds.** The 23 human-disposition and review holds (RF-001/RF-002,
  PDU-019, PDU-035 and PDU-060 findings) do not gate v1.0; they are
  dispositioned in one batch.
- **Public repository.** The independent public repository `sgttomas/SWBPIPE`
  is SWBPIPE's public home, replacing the sanitized-export mechanism. The
  verified import may be published before scan signature. The owner-signed
  D-20/DEC-058 repository scan, including the full private-term scan and
  finding dispositions, gates v1.0 (DEL-10-04:1; exit criterion 16).
- **Embedded agent.** The Chirality App v4 agent harness is embedded in the
  desktop (DEL-16-06). The aim is an agent in SWBPIPE that operates the
  application effectively, ahead of workflow governance working perfectly.
  Earlier designs, DEC-041 among them, are historical context only: note what
  was done, but do not take from it, because there are better examples
  elsewhere now and the connection is better designed from fresh ideas. The
  connection is designed afresh with
  App v4's HELP_HUMAN and recorded in the owning scope (DEL-16-06).

**Opening duty.** At session start, Agent 0 reports the path from the current
state to that target, the work that can proceed independently, what it needs
from the owner to keep that work moving (decisions, credentials, host
conditions) and any open `nightly` issue. Once the owner has steered, it runs
the independent work in parallel, starting each Type 1 agent in a new thread.

**Working posture.** Agent 0's top priority is to keep productive work moving
toward the publication goals. Testing and documenting are necessary and easy
to get absorbed in, so repeated cycles of them are not allowed to stall
development. The INTERLOCUTOR, where one is selected, reminds Agent 0 of this
when the circumstances arise. In practice:

- Testing and documenting serve the next step, not their own completeness.
- Group known CI failures by cause instead of re-examining each job.
- Use CI waits for other work.
- When something outside Agent 0's authority blocks work (the host, another
  project, a reserved choice), bring it to the owner at once as one decision
  with a recommendation, and keep the other lanes moving.
- Act on authority already given. Do not re-ask a settled decision.
- Budgets, caps and gates that Agent 0 sets itself are its own to change.
  Label them as its own when stating them.

**Testing.** Agent 0 decides and orchestrates the tests a change needs; the
owner does not approve each test. Agent 0 either tests as it sees fit or puts
controls in place that it can manage itself, because its role is to keep
productive work moving. Controls it adds are its own, under the posture above.

## Benchmark method (summary)

The validation lane proposes the set; this section records the method only:
- **Scope.** Mechanics only: displacements, rotations, element forces and
  moments, support loads, frequencies, mode shapes and spectrum responses.
  Stresses and code checks are left out.
- **Models.** All invented. A seeded generator covers model families from
  straight spans to nonlinear supports, hangers and dynamics. Closed-form
  members give an independent third value. Where no closed form exists, the
  comparison with the program is the validation: in the owner's words of
  14 November 2026, CAEPIPE is "a trusted validator model".
- **Inputs.** Each code-dependent input that changes mechanics is held equal on
  both sides. Where it cannot be held equal, it is named and carried by a
  one-parameter series. Comparisons are used to find and fix SWBPIPE's defects;
  no factor, default or tolerance is set in SWBPIPE only to make a result
  agree.
- **Screening profile.** Agent 0 screens with these values, proposed by the
  validation lane. The owner's review of the full record decides, not the profile. Each
  is a relative tolerance, plus a printed-resolution floor per result kind:

  | Class | Tolerance |
  |---|---|
  | Every input held exactly | 1e-4 |
  | A named inexact input remains | 5e-3 |
  | Nonlinear | 2e-2, and support status must match exactly |
  | Modal | 5e-3 on frequencies, MAC ≥ 0.99 |
  | Spectrum | 1e-2 on displacements and rotations; 5e-3 on element forces, moments and support loads |

  The modal and spectrum classes, and the references frozen for them, apply
  when those analyses are taken up after v1.0 (Direction, "Seismic").

  The spectrum class was set before any spectrum comparison run, from its
  closed-form references alone, never from comparison results, and was
  reported to the owner. It applies to combined magnitudes whose two sides
  correspond: the same periodic modes, matched mode by mode, with no repeated
  cluster; the same modal, directional and rigid-response rules; and every
  periodic mode and the rigid frequency inside the spectrum's tabulated range.
  A row that fails one of these conditions, including a repeated-cluster or
  mode-set mismatch, is a finding, not a tolerated difference. On the
  references' own computation, with no product result involved, SWBPIPE's
  stated formulation has a smallest margin of 5.87 over the consistent-mass
  combined and rigid-response population at 8 members or more (a force under
  the missing-mass correction with the continuum's periodic classification,
  8.52e-4 as summarized), and 34.3 on the benchmark pipe's case C at 8 members
  (the tip support reaction under the absolute sum, TWO spectrum). DEL-04-10
  REQ-008 states its derivation, applicability and governing rows. The formulation
  difference predicted with the references before any comparison is a frozen
  reference and is changed only on independent grounds. Values are only
  tightened.
- **Findings.** Any result outside the profile, an unmatched row, or a status,
  refusal or mode-count mismatch is a finding.
  - Triage order: mapping check, closed form, then SWBPIPE's own invariants,
    then a convention series.
  - Each finding is classed as SWBPIPE's to fix, a documented convention
    difference, or unexplained.
- **Review and publication.** Agent 0 screens against the profile each run
  that exit criterion 11 counts as screenable, names every other run with its
  reason, and flags anything of merit; the owner then reviews the full record,
  unmatched results included.
  Agreement is published once the owner has checked the CAEPIPE licence for
  benchmark clauses, and `docs/IP_AND_DATA_BOUNDARY.md` §6.2 is amended to
  match. Other evaluation stays private.

## Exit criteria

Thresholds marked *(provisional)* are Agent 0's targets. Agent 0 sets them from
intended use and representative examination; a revision explains why the
target remains adequate. A failure alone does not justify relaxing one.

1. **One model, every required analysis.** An invented reference system solves on
   EX in the desktop with no admitted family refused. It contains:
   - pressure, realized bends, a tee with user modifiers, an eccentric reducer,
     valves and flanges, and an expansion joint with tie rods;
   - linear restraints, a gap, lift-off, friction, a variable spring and a
     constant-effort support.

   The analyses are: sustained, operating, expansion as the operating minus
   sustained difference, hydrotest, wind, three-axis static seismic with SRSS
   and absolute-sum combination, seismic anchor movement and hanger sizing.
   The declared basis of the expansion difference on EX follows after v1.0
   (LIM-038), as do modal and response-spectrum analysis (Direction,
   "Seismic").

   *Owning deliverables:* DEL-04-07 (one route); DEL-03-03 to DEL-03-06 (components); DEL-04-03 and DEL-04-04 (supports); DEL-05-01 and DEL-05-02 (loads, cases, combinations); DEL-04-11 (hanger sizing).
2. **Benchmarks.** Each analysis in criterion 1 passes closed-form or
   hand-calculation benchmarks on invented cases that exercise that analysis,
   using references frozen in `validation/references/` with an independent
   generator.

   In both solver modes, the solved reference-system fixture passes every
   reference check that its cases admit (DEL-09-01 AC-009). The fixture
   specification names them (`fixtures/reference_system/SPECIFICATION.md`
   §9). An applicable check counts as met only when demonstrated. Checks that
   require pending cases apply when those cases are admitted.
   - The checks include support force and moment balance against case
     weight, including hydrotest weight. The change in support force and
     moment resultants from sustained balances each wind or static seismic
     load resultant. The anchor-movement reaction difference is
     self-equilibrated. The expansion difference and the static seismic SRSS
     and absolute-sum combinations are checked against their definitions.
     The checks also include the presence of the specified tee stress rows
     and agreement of the constant-effort support's published applied load and
     support action with its entered load in cases that use that load.
   - For numerical comparisons, use independent references applicable to the
     fixture revision under test. The packages `reference_system`,
     `reference_system_r1_ce`, `reference_system_pending_cases` and
     `reference_system_generated_loads` state their respective bases and
     status. `reference_system_generated_loads` holds per-load and warning
     references for the wind and static seismic cases;
     `reference_system_pending_cases` remains the reference for EQ-6.
     Section 9 also defines identities that require no package.
   - No independent solution of the full system's response is frozen, so
     these checks do not establish the response throughout the system.

   *Owning deliverables:* each analysis's deliverable under criterion 1, with DEL-09-01 for the reference system.
3. **Limitations.** Every remaining refusal of a capability these criteria require is listed in the
   release limitations and declared acceptable by Agent 0. The entry states
   the requirement the refusal affects, what that means in practice for a
   user, and the reason, and the limitations are shown to the owner with the
   request for the verdict. Declaring a refusal acceptable is a limitation
   decision: it does not demonstrate the refused capability, and it does not
   make an unmet criterion met.

   *Owning deliverables:* the deliverable that owns each refusal; the list is `docs/RELEASE_LIMITATIONS_v1.0.md`.
4. **Shareable report.** A user releases, saves and prints a report for results
   on either route, with the §19.3 notice and a reproducible canonical hash.

   *Owning deliverables:* DEL-08-01, DEL-12-02.
5. **CSV export.** Every results table in the desktop exports to CSV.

   *Owning deliverables:* DEL-08-04.
6. **MBF round trip.**
   - The desktop saves the reference system as MBF with no family of criterion 1's
     reference system omitted
     or unsupported in the loss report.
   - The desktop imports it back with the same entity identities.
   - **Family list.**
     DEL-17-04 excludes
     combinations, which combine solved results, from criterion 6's families.
     Combination coverage is judged under criterion 1 (DEL-05-02). The other refusals in DEL-17-04
     still stand; criterion 6 remains
     unmet.

   *Owning deliverables:* DEL-17-04, DEL-07-12.
7. **Authoring time.** From blank, the reference system is entered by run sheet
   and solved in ≤ 30 minutes *(provisional)*.

   *Owning deliverables:* DEL-07-02, DEL-07-06.
8. **Usability.**
   - Every use case in DEL-07-06's acceptance criteria completes in the desktop
     with no step outside it. This is tested thoroughly before the owner is
     asked.
   - Before the owner or any other person is asked to use the desktop, two
     independent reviewers each score its interface, from a walkthrough of
     those use cases in the launched application, on DEL-07-06's nine
     interface criteria. Both give at least 4 overall on every criterion, and
     neither gives any use case less than 3 on any criterion that applies to
     it, with no applicable criterion left unscored (DEL-07-06 REQ-017 and
     AC-008).
   - The owner then uses the desktop and answers yes or no on whether it is
     ready. The criterion is met on yes.

   *Owning deliverables:* DEL-07-06.
9. **Performance.** On a 2,000-node rack model, on the 16 GiB base machine,
   measured by the DEL-04-05 harness *(provisional)*:
   - linear static ≤ 10 s;
   - nonlinear operating ≤ 60 s.

   *Owning deliverables:* DEL-04-05.
10. **Autosave.** It is on by default; a crash loses at most one interval.

    *Owning deliverables:* DEL-02-05.
11. **Benchmark set.** A run is screened against the benchmark method's
    tolerance profile, and its findings are triaged, where the benchmark
    manifest gives it a SWBPIPE counterpart and the conditions it names for
    that run are met. A run without a counterpart, such as an NRL mode-sum run
    or the member of an alternative pair that was not analysed, or one whose
    conditions are not met, is named with its reason and is not counted as
    screened. Dynamics runs with a counterpart are captured as references and
    are screened when modal and response-spectrum analysis are taken up after
    v1.0. Unmatched results stay in the full record, which the owner has
    reviewed.

    *Owning deliverables:* DEL-17-05.
12. **Reproduction.** A fresh reproduction bundle exists at the release
    candidate.

    *Owning deliverables:* DEL-09-04.
13. **Documentation.** The user guide and theory notes cover every analysis
    in criterion 1: inputs, method, assumptions and limitations. They state that
    SWBPIPE does not determine code compliance.

    *Owning deliverables:* DEL-11-01, DEL-11-03.
14. **Content check.** The release-candidate protected-content scan is signed
    off by the owner (DEL-08-05:2).

    *Owning deliverables:* DEL-08-05.
15. **Build and licence.** The build is signed and notarized, and MIT is
    recorded consistently.

    *Owning deliverables:* DEL-10-04.
16. **Public repository.** `sgttomas/SWBPIPE` is SWBPIPE's independent public
    home, with the release candidate bound to its repository revision and
    standalone build/test/CI verification. Before v1.0 release, the owner has
    completed and signed the D-20/DEC-058 scan of that repository, including
    the full private-term scan and per-finding dispositions; unresolved
    blocking findings prevent release. The verified migration import may be
    pushed before signature. A merge, passing check or unsigned scan
    does not supply the owner's signature or release acceptance (DEL-10-04:1).

    *Owning deliverables:* DEL-10-04.
17. **Embedded agent.** From a person's request in the desktop's conversation
    panel, the embedded agent builds the reference system, solves it and
    reports its key results, through SWBPIPE's capability catalog. Every change
    goes through the one operation route and is applied on the person's
    acceptance or within autonomy the person granted; the agent records no
    acceptance, approval or reliance (F-PIP-3).

    *Owning deliverables:* DEL-16-06.
18. **Numerical claims.** A case governed by DEL-04-01/REQ-015 that is
    presented as having passed its integrity check, or as established, has the
    bound and standing that requirement states. Only the modal eigenproblem's
    integrity checks governed by DEL-04-09/REQ-003 and REQ-004 are outside
    this criterion. A static state-case solve used by a modal analysis,
    including its nonlinear support-state confirmation, remains within this
    criterion. The error-bound argument is justified under its stated
    arithmetic model, and the product establishes its case-dependent
    conditions; an unproved multiple of a condition estimate does not
    establish an upper bound. Where the integrity check finds sensitivity, or
    is unavailable for a reason DEL-04-01/REQ-015 names, the solve is reported
    as numerically sensitive with its named reason. Where the solver's checks
    find nothing against a case and the bound they rest on is not verified for
    that case, a person is told what was done and found: that the solver's
    checks found no problem, and that these checks look for numerical trouble
    and do not measure the error. The case is not reported as numerically
    sensitive on that ground, and it is not presented as having passed an
    integrity check, as verified or as established. A converged nonlinear
    state is presented as confirmed only when DEL-04-01/REQ-016's enclosures
    establish every nonlinear support's state, as DEL-04-04/REQ-013 and
    DEL-04-07/CLM-002 require. Where the solver's confirmation reports a state
    established while the bound its enclosures rest on is not verified, a
    person is told that the solver reports it established and that the
    confirmation rests on the solver's own error estimate; where the
    confirmation did not run or did not establish a state, that is said with
    its named reason. Readers and consumers preserve these outcomes and apply
    their existing standing and admission rules; admission asserts no bound.
    That the bound is not verified is carried as a field in every verdict and
    file output a consumer reads. For v1.0 the product verifies the bound for
    no case (LIM-026). A claim derived from a case's bound, such as a
    combination certificate's verdict over its operand cases or a statement
    that a case is stress-free within a stated bound, is the solver's own
    message and is shown as one, under the statement that SWBPIPE has not
    independently checked the error bound it rests on. A capability these criteria require that is refused on this ground is
    a refusal under criterion 3; declaring that refusal acceptable does not
    make an unmet criterion met.

    *Owning deliverables:* DEL-04-01 (integrity bound and nonlinear enclosures); DEL-04-04 and DEL-04-07 (support-state confirmation and publication); DEL-05-02 (ordinary product reader); DEL-04-08 (retained-result reader, reader bindings and test verifier). The viewer, report, export, rule-binding and modal-state consumers apply their existing rules under their own deliverables (DEL-07-05, DEL-08-01, DEL-08-04, DEL-06-03, DEL-04-09).

## Reserved to the owner

- Acceptance, issuance and release, and the verdict on the product: the
  owner's yes or no under exit criterion 8.
- Publishing a release, and publishing benchmark agreement.
- Licence, legal and professional sign-off: the content and repository scans
  of exit criteria 14 and 16, the check of the CAEPIPE licence for benchmark
  clauses, and signing the build.
- The owner's review of the full benchmark record (exit criterion 11). Agents
  operate the owner's CAEPIPE, read its results and compare them privately; what
  is reserved is the owner's review of the benchmark record and the licence
  check before any agreement is published.
- Purchases, credentials, and repository protections and settings.
- Release authority: waiving a release gate, or accepting a risk as the act of
  releasing.
- The native-smoke `expected.json` values.
- Amending `docs/PRD.md`.

For the native-smoke `expected.json` values only: where a value must change
solely as the direct consequence of a decision Agent 0 has made on the
product's own grounds, Agent 0 may make the change, with independent review,
and states it in the pull request. The owner, on 2026-11-20: "You can have my
delegated authority in matters consistent with this pattern I've just
provided. (coherent and consistency in those four aspects in support of
engineering judgment to keep the design implementation moving forward)"; the
four aspects are what the application is, what it can know, what it does and
what it values. Every other item listed above stays the owner's: acceptance,
issuance, release and the verdict on the product; publication, including
benchmark agreement; licence, legal and professional sign-off; the review of the
full benchmark record; purchases,
credentials, and repository protections and settings; release authority; and
amending `docs/PRD.md`. An expected value still never changes to fit the
implementation.

Agent 0 decides what else the target needs, under the grant in `AGENTS.md`
("Posture for this phase"): changing an exit criterion or its value on product
grounds, declaring a remaining refusal acceptable (criterion 3), technical
risk decisions and technical holds, however near release is. Agent 0 may
revise what the target requires; it may not treat an unmet requirement as
passed, and it does not exercise the owner's release authority. A hold that
waits on evidence still waits on it. A numerical reference expectation is
changed only on independent grounds, never to fit the implementation. The
release limitations that criterion 3 refers to are kept in
`docs/RELEASE_LIMITATIONS_v1.0.md`, where lanes draft entries.

Decisions edit the text they govern. This file is not a log and holds no
status; Git keeps its history.
