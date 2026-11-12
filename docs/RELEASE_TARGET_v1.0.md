---
name: swbpipe-release-target-v1-0
description: Set the v1.0 capability target, verification criteria and reserved decisions.
metadata:
  kind: instruction
  surface: requirements
---
# SWBPIPE v1.0 (publication-ready): release target

**Status:** PRD companion (`docs/PRD.md` §24 V1). It is the target for the
first publication-ready release, accepted by the owner on 2026-10-28
("Accepted."). It comprises the direction, the owner's rulings, the benchmark method and the
exit criteria. It is also the record on `main` of the owner's 2026-10-28
rulings, given in conversation with HELP_HUMAN. The exit criteria are the
definition of done. Capability and status are not kept here: each criterion
names the deliverables that own it, and their `ScopeOfWork.md` states
what each commits to and what is undecided. Meeting the exit criteria
is evidence for the owner's release decision, not the decision: acceptance,
issuance and release remain the owner's acts (F-PIP-3). SWBPIPE computes mechanics from recorded inputs and evaluates
the user's own rule packs; it does not certify, approve or establish code
compliance. Standard claim fence applies (claims taxonomy per DEC-081).

Paths are relative to the repository root. **PF** is the pressure-free solve
route (a model with no `pressure_contract`); **EX** is the exact pressure route
(`3.0.0/exact_pressure_v3`). Labels such as `RT-A7` in code and records name rows of
an earlier capability table, removed from this file on 2026-11-10 and kept in
Git history; the owning deliverable holds the current account.

## Direction

**Target.** Development is a hill climb to the first publication-ready release,
v1.0.

- **Scope for the climb.** The goal is an app on par with or superior to
  CAEPIPE and other industry applications in modelling and user interface
  (owner, 2026-10-27: "At this point we're hill-climbing towards an app that
  would be on par with or considered superior to CAEPIPE and other industry
  standard applications in this domain, in terms of the modelling
  characteristics and the user interface, so if there's a change or expansion
  in scope that would clearly benefit that kind of hill climb then we want to do
  it and you can decide under the extension of the same standing delegation
  authorization. If it fixes a mistake, if it plugs an obvious gap, if it's
  about making something more accurate, truthful, or capable without
  considerable risks of any sort, you can take initiative and drive scope
  towards a successful completion of the first publications ready version of
  SWBPIPE."). HELP_HUMAN decides such scope changes, including commitment and
  acceptance-criterion text, and may reverse earlier deferrals to v0.2 (owner,
  2026-10-27: "Yes, decide them too"). It approves feature proposals and reports
  what it adopted, deferred and why (owner, 2026-10-27: "I decide, you get a
  summary"). The governing text records each decision and its reason.

**Opening duty.** At session start, Agent 0 reports the path from the current
state to that target, the work that can proceed independently, what it needs
from the owner to keep that work moving (decisions, credentials, host
conditions) and any open `nightly` issue. Once the owner has steered, it runs
the independent work in parallel in standalone Type 1 sessions. 

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

## Owner rulings

**Definition** (owner, 2026-10-27):
- Publication-ready means "a version good enough to publish engineering results
  with".
- Analyses: "We should be able to do seismic plus hydrostatic, and the
  sustained, expansion, operation as you clearly know too. Anything else that
  would be considered standard?"
- "For now, it won't proclaim code compliance. I actually want the user to go
  to CAEPIPE for that purpose. This software does all the design iteration."
- CAEPIPE is the validation to SWBPIPE: features on par or better, "given real
  thought for user experience and human factors in the design and taste in the
  aesthetics", with a benchmark set comparing non-code outputs "across as wide a
  range as practicable".

**Rulings.** The owner answered ten open choices (OC-1 to
OC-10) on 2026-10-28. Where the owner chose an option as drafted, the item
quotes the option's drafted text; the owner's own words are quoted as such.
- **Published results (OC-1).** The owner chose (a) (owner, 2026-10-28), drafted as: "The
  user's engineer issues SWBPIPE's mechanics results under their own
  responsibility: support and equipment loads, displacements, frequencies,
  fundamental stresses."
- **One route (OC-2).** The owner chose (a) (owner, 2026-10-28: "OC-2, a, is
  anything desirable or necessary omitted without also selecting b?").
  HELP_HUMAN answered that nothing necessary is omitted provided EX admits
  every family PF has, and that (b), pressure on PF, would be revisited only if
  EX failed the performance targets (exit criterion 9). EX is extended; PF gets no
  pressure.
- **Seismic (OC-3).** The owner chose (a) (owner, 2026-10-28), drafted as: "Must:
  user-entered spectra, modal and directional combination, a missing-mass
  correction, and snubbers as a dynamic-only restraint." The rule (owner,
  2026-10-28): "I said you should only develop a response spectrum capability
  for seismic analysis if that's a functionality that CAEPIPE has, otherwise
  you can go with only static for now." That condition is met: the vendor's
  public manual describes response-spectrum analysis with mode and directional
  combination and missing mass
  ([dynamic analysis](https://docs.sstcae.com/tech_manual/dynamic_analysis.htm),
  [spectrums](https://docs.sstcae.com/users_manual/spectrums.htm),
  [load cases](https://docs.sstcae.com/users_manual/load_cases.htm)). The
  governed scope change DEC-078 requires is SCA-014.
- **Hangers (OC-4).** The owner chose (a) (owner, 2026-10-28), drafted as: "Must: size
  hangers from the user's private catalogue (invented example records only)
  and produce a hanger table."
- **Hydrotest (OC-5).** The owner chose (a) (owner, 2026-10-28), drafted as: "A flagged
  test condition with test pressure, test contents density and spring supports
  held, marked as a test case in results and reports."
- **Rule packs (OC-6).** The owner answered "OC-6, as stated" (owner,
  2026-10-28). OC-6 as drafted: "Keep what is built as a design aid, and defer the extensions:
  C2, E1, E2, E5 and E6. Drop E3 (fatigue usage) and E4 (pressure-design
  thickness) from the roadmap. … Show user-rule outcomes as "within / exceeds
  user limit" rather than pass/fail, so that no outcome reads as compliance. …
  It covers 16 ScopeOfWork items: keep 11, split 1, defer 4."
- **Usability (OC-7)** (owner, 2026-10-28): "I want you to use your own
  judgment after concocting some typical use cases an engineer would go
  through (I will also get actual user testing but that's after you put this
  through the wringer)."
- **Release path (OC-8)** (owner, 2026-10-28): "go straight to v1.0 and first
  macOS but we will revisit immediately after for Windows too."
- **Licence (OC-9)** (owner, 2026-10-28): "it's MIT license."
- **Benchmarks (OC-10)** (owner, 2026-10-28): "first screen the results with
  the tolerance profile and flag anything of merit, then I will review the full
  record." Benchmark agreement with CAEPIPE will be published (owner,
  2026-10-28: "Benchmark agreement with CAEPIPE will indeed be published. That
  part should be out in the open because the whole point is to feed into
  CAEPIPE and expect the results to come out the same (but validated to Code
  and to the reputation of CAEPIPE)."), subject to the owner's check of the
  CAEPIPE licence for benchmark clauses.
  The 23 human-disposition and review holds (RF-001/RF-002, PDU-019, PDU-035
  and PDU-060 findings) do not gate v1.0; they are dispositioned in one batch
  (OC-10 (b); owner, 2026-10-28: "As recommended").
- **Pressure effects** (owner, 2026-10-28: "As recommended"). The Bourdon
  effect is a Should option for v1.0, off by default and clearly labelled
  (DEL-04-07). Pressure-dependent flexibility and SIF corrections are out of scope:
  users enter their own k and SIF.
- **Public repository** (owner, 2026-10-29: "Split repo replaces RT-V8" and
  "Push now, sign before v1.0"). The independent public repository
  `sgttomas/SWBPIPE` is SWBPIPE's public home, replacing the sanitized-export
  mechanism. The verified import may be published before scan signature. The
  owner-signed D-20/DEC-058 repository scan, including the full private-term
  scan and finding dispositions, gates v1.0 (DEL-10-04:1; exit criterion 16).
- **Exact-route results under user release** (owner, 2026-10-29: "Open it, as recommended"). Route 010 admits exact-route results with exact source binding, exact sections and limitations, Diagnostics binding and producer item 9 (DEL-08-01 Design `report-user-release.md` §11).
- **Embedded agent** (owner, 2026-10-28: "I want you to add the App v4
  integration into SWBPIPE for the v1.0 target. I want you two to discuss how
  to make it connect, but I thought this had already been decided on (check
  the deliverables). I'm not interested in having all the governance for the
  workflows working perfectly, my focus is just on getting the agent harness
  into SWBPIPE and able to operate that application effectively."). The
  Chirality App v4 agent harness is embedded in the desktop (DEL-16-06). Earlier
  designs, DEC-041 among them, are historical context only (owner, 2026-10-28,
  relayed from the App session: "take note of what was done in the past, but
  don't take from it. There's better examples now elsewhere and just from your
  own ideas."). The connection is designed afresh with App v4's HELP_HUMAN and
  recorded in the owning Design.

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
  validation lane, under the owner's ruling to "first screen the results with
  the tolerance profile and flag anything of merit, then I will review the full
  record". The owner's review of the full record decides, not the profile. Each
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
   - **Family list (owner, 2026-11-08; option 3, "Family list amended").**
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
    pushed before signature (owner, 2026-10-29: "Split repo replaces RT-V8";
    "Push now, sign before v1.0"). A merge, passing check or unsigned scan
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

- Changing an exit criterion or its value.
- Declaring a limitation acceptable. The release limitations that criterion 3
  refers to are kept in `docs/RELEASE_LIMITATIONS_v1.0.md`. Lanes draft entries
  there; accepting one is the owner's act.
- Acceptance, issuance and release.
- Risk acceptance and holds.
- Purchases.
- The native-smoke `expected.json` values.

Decisions edit the text they govern. This file is not a log and holds no
status; Git keeps its history.
