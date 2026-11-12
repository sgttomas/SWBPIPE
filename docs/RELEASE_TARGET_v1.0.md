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
- **Scope for the climb.** A change or expansion in scope that clearly
  benefits the climb is made: one that fixes a mistake, plugs an obvious gap,
  or makes the product more accurate, truthful or capable without considerable
  risk of any sort. Agent 0 decides such changes under the grant in
  `AGENTS.md` ("Posture for this phase"), including commitment and
  acceptance-criterion text and the reversal of earlier deferrals to v0.2. It
  approves feature proposals and reports what it adopted, deferred and why.
  The governing text records each decision and its reason.
- **What is published.** The user's engineer issues SWBPIPE's mechanics
  results under their own responsibility: support and equipment loads,
  displacements, frequencies, fundamental stresses. SWBPIPE does the design
  iteration and does not proclaim code compliance; the user goes to CAEPIPE
  for that.
- **Analyses.** Sustained, expansion, operating, hydrostatic test and seismic,
  with the others exit criterion 1 lists as standard.
- **One route.** EX is extended and PF gets no pressure. Nothing necessary is
  omitted by this provided EX admits every family PF has. Pressure on PF is
  revisited only if EX fails the performance targets (exit criterion 9).
- **Seismic.** Response-spectrum analysis is in v1.0: user-entered spectra,
  modal and directional combination, a missing-mass correction, and snubbers
  as a dynamic-only restraint (DEL-04-10; scope change SCA-014). It is
  developed because CAEPIPE has it; had CAEPIPE not, static seismic alone
  would do for now. The vendor's public manual describes response-spectrum
  analysis with mode and directional combination and missing mass
  ([dynamic analysis](https://docs.sstcae.com/tech_manual/dynamic_analysis.htm),
  [spectrums](https://docs.sstcae.com/users_manual/spectrums.htm),
  [load cases](https://docs.sstcae.com/users_manual/load_cases.htm)).
- **Hangers.** v1.0 sizes hangers from the user's private catalogue (invented
  example records only) and produces a hanger table.
- **Hydrotest.** A flagged test condition with test pressure, test contents
  density and spring supports held, marked as a test case in results and
  reports.
- **Rule packs.** What is built is kept as a design aid. The extensions C2,
  E1, E2, E5 and E6 are deferred, and E3 (fatigue usage) and E4
  (pressure-design thickness) are dropped from the roadmap. User-rule outcomes
  are shown as "within / exceeds user limit" rather than pass/fail, so that no
  outcome reads as compliance.
- **Pressure effects.** The Bourdon effect is a Should option for v1.0, off by
  default and clearly labelled (DEL-04-07). Pressure-dependent flexibility and
  SIF corrections are out of scope: users enter their own k and SIF.
- **Exact-route results under user release.** Route 010 admits exact-route results with exact source binding, exact sections and limitations, Diagnostics binding and producer item 9 (DEL-08-01 Design `report-user-release.md` §11).
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
  was done, but do not take from it. The connection is designed afresh with
  App v4's HELP_HUMAN and recorded in the owning Design.

**Opening duty.** At session start, Agent 0 reports the path from the current
state to that target, the work that can proceed independently, what it needs
from the owner to keep that work moving (decisions, credentials, host
conditions) and any open `nightly` issue. Once the owner has steered, it runs
the independent work in parallel, starting each Type 1 agent in a new thread.

**Working posture.** The owner's standing instruction (2026-11-09 00:29 UTC):
"This is just a standing instruction for you: Your top priority is to keep
productive work moving forward. Don't stall out in repetitive cycles of testing
and documenting of work. Some of that is necessary but easy to get absorbed in.
Keep development moving towards the publication goals. Tell your interlocutor
this is important for them to remind you of too, when relevant circumstances
arise." In practice:

- Testing and documenting serve the next step, not their own completeness.
- Group known CI failures by cause instead of re-examining each job.
- Use CI waits for other work.
- When something outside Agent 0's authority blocks work (the host, another
  project, a reserved choice), bring it to the owner at once as one decision
  with a recommendation, and keep the other lanes moving.
- Act on authority already given. Do not re-ask a settled decision.
- Budgets, caps and gates that Agent 0 sets itself are its own to change.
  Label them as its own when stating them.

**Testing.** The owner delegated testing (2026-11-09 04:42 UTC): "Ok I need to
be clear that I'm not going to be approving every test you want to run. Either
do the testing as you see fit or put controls in place that you can manage
yourself. Your role is to keep productive work moving forward. If that means
certain necessary tests then orchestrate that." Agent 0 decides and orchestrates
the tests a change needs. Controls it adds are its own, under the posture above.

## Benchmark method (summary)

The validation lane proposes the set; this section records the method only:
- **Scope.** Mechanics only: displacements, rotations, element forces and
  moments, support loads, frequencies, mode shapes and spectrum responses.
  Stresses and code checks are left out.
- **Models.** All invented. A seeded generator covers model families from
  straight spans to nonlinear supports, hangers and dynamics. Closed-form
  members give an independent third value.
- **Inputs.** Each code-dependent input that changes mechanics is held equal on
  both sides. Where it cannot be held equal, it is named and carried by a
  one-parameter series. SWBPIPE is never tuned to match.
- **Screening profile.** Agent 0 screens with these values, proposed by the
  validation lane. The owner's review of the full record decides, not the profile. Each
  is a relative tolerance, plus a printed-resolution floor per result kind:

  | Class | Tolerance |
  |---|---|
  | Every input held exactly | 1e-4 |
  | A named inexact input remains | 5e-3 |
  | Nonlinear | 2e-2, and support status must match exactly |
  | Modal | 5e-3 on frequencies, MAC ≥ 0.99 |

  The spectrum class is set before any comparison run, from its closed-form
  references, never from comparison results, and reported to the owner.
  Values are only tightened.
- **Findings.** Any result outside the profile, an unmatched row, or a status,
  refusal or mode-count mismatch is a finding.
  - Triage order: mapping check, closed form, then SWBPIPE's own invariants,
    then a convention series.
  - Each finding is classed as SWBPIPE's to fix, a documented convention
    difference, or unexplained.
- **Review and publication.** Agent 0 screens every run against the profile
  and flags anything of merit; the owner then reviews the full record.
  Agreement is published once the owner has checked the CAEPIPE licence for
  benchmark clauses, and `docs/IP_AND_DATA_BOUNDARY.md` §6.2 is amended to
  match. Other evaluation stays private.

## Exit criteria

Thresholds marked *(provisional)* are Agent 0's targets. Testing of the use cases informs
them; they become final only on the owner's acceptance after the owner's verdict under
criterion 8.

1. **One model, every required analysis.** An invented reference system solves on
   EX in the desktop with no admitted family refused. It contains:
   - pressure, realized bends, a tee with user modifiers, an eccentric reducer,
     valves and flanges, and an expansion joint with tie rods;
   - linear restraints, a gap, lift-off, friction, a variable spring and a
     constant-effort support.

   The analyses are: sustained, operating, expansion with its basis,
   hydrotest, wind, three-axis static seismic with SRSS and absolute-sum
   combination, seismic anchor movement, modal, response spectrum and hanger
   sizing.

   *Owning deliverables:* DEL-04-07 (one route); DEL-03-03 to DEL-03-06 (components); DEL-04-03 and DEL-04-04 (supports); DEL-05-01 and DEL-05-02 (loads, cases, combinations); DEL-04-09 (modal); DEL-04-10 (response spectrum); DEL-04-11 (hanger sizing).
2. **Benchmarks.** Each analysis in criterion 1 passes closed-form or hand-calculation
   references frozen in `validation/references/` with an independent generator.
   Modal also passes at least one published piping benchmark (DEL-04-09
   AC-010).

   *Owning deliverables:* each analysis's deliverable under criterion 1, with DEL-09-01 for the reference system.
3. **Limitations.** Every remaining refusal of a capability these criteria require is listed in the
   release limitations and accepted by the owner.

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
     unmet, and no reference-system round-trip claim is made.

   *Owning deliverables:* DEL-17-04, DEL-07-12.
7. **Authoring time.** From blank, the reference system is entered by run sheet
   and solved in ≤ 30 minutes *(provisional)*.

   *Owning deliverables:* DEL-07-02, DEL-07-06.
8. **Usability.**
   - Every use case in DEL-07-06's acceptance criteria completes in the desktop
     with no step outside it. This is tested thoroughly before the owner is
     asked.
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
11. **Benchmark set.** Every case is screened against the benchmark
    method's tolerance profile, and the
    findings are triaged. The owner has reviewed the full record.

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

## Reserved to the owner

- Acceptance, issuance and release, and the verdict on the product: the
  owner's yes or no under exit criterion 8.
- Publishing a release, and publishing benchmark agreement.
- Licence, legal and professional sign-off: the content and repository scans
  of exit criteria 14 and 16, the check of the CAEPIPE licence for benchmark
  clauses, and signing the build.
- The owner's review of the full benchmark record (exit criterion 11), and the
  private CAEPIPE evaluation record and trial it rests on.
- Purchases, credentials, and repository protections and settings.
- The native-smoke `expected.json` values.

Agent 0 decides what else the target needs, under the grant in `AGENTS.md`
("Posture for this phase"): changing an exit criterion or its value, declaring
a limitation acceptable, risk acceptance and technical holds. A hold that
waits on evidence still waits on it, and an expected value is changed only on
independent grounds, never to fit the implementation. The release limitations
that criterion 3 refers to are kept in `docs/RELEASE_LIMITATIONS_v1.0.md`,
where lanes draft entries.

Decisions edit the text they govern. This file is not a log and holds no
status; Git keeps its history.
