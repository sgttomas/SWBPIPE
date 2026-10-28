# SWBPIPE development entry

SWBPIPE is an analysis-grade piping design engine and stress-model authoring
environment. Work in `projects/chirality-piping`, starting from the affected
`execution/PKG-nn/DEL-nn-nn/`. Consult `docs/PRD.md` and its technical
companions for the requirements involved. From the repository root:

```sh
python3 -m tools.deliverables --project projects/chirality-piping neighborhood DEL-nn-nn
```

## Implementation and reconciliation

Each implementation return must include reconciliation of its affected Scope of Work, Design and dependency conditions before integration.

The implementation manager owns this reconciliation. Carry the rule into every
implementation brief and identify the affected deliverables. TASKs report the
effects of their slices; the manager reconciles the combined result with
`ScopeOfWork.md`, `Design/` and `deliverable.yaml` as implementation and
verification progress. Include warranted edits in the same implementation PR; a
brief explanation in the PR suffices when no edits are warranted. HELP_HUMAN may
edit acceptance-criterion wording that must change only to describe consequences
of decisions the owner has already made, with every protected fact unchanged,
and reports the exact old and new text (owner, 2026-10-19). HELP_HUMAN may also
replace ScopeOfWork, Design and acceptance wording as it recommends, without
asking first, once an independent review has examined the exact old and new text
and HELP_HUMAN is clear in the matter (owner, 2026-10-23: "You can continue to
replace wording as you recommend without asking me any more so long as an
independent review has been done and you're clear in the matter."). The PR
quotes the old and new text and the review. Scope (except as delegated below),
risk acceptance, holds, release and the native-smoke `expected.json` values
stay with the owner.

HELP_HUMAN exercises engineering judgment to resolve a matter of precision or
detail that cannot otherwise be resolved, choosing a practical resolution
within normative engineering practice, such as a unit conversion routinely done
in engineering applications (owner, 2026-10-26: "units conversions that are
routinely done in engineering applications are permissible as a solution." and
"I will leave with you Agent 0, in your capacity of HELP_HUMAN, with exercising
engineering judgment where it can help resolve a matter that is due to
precision or details that can't be resolved except to allow a practical
resolution that is within normative engineering practices."). The owner's
basis is that the engineering design, physics and numerical methods are
established, so such a resolution reflects engineering practice and is not a
degraded output. The governing Design text records each resolution and its
reason. A resolution that changes numerical meaning still takes targeted
regression checks and independent scrutiny. The reservations above still apply.

The current phase drives toward finishing (owner, 2026-10-26: "we are nearing a
level of maturity where there needs to be a concerted drive towards goals and
finishing things to a practical level. Continuous improvement goes after.").
Carry each unit to a usable, verified result. Record improvements found along
the way as remaining items, and take them up after the unit unless they
correct a wrong output or block the goal.

The goal is an app on par with or superior to CAEPIPE and other industry
applications in modelling and user interface, beyond v0.1 parity (owner,
2026-10-27: "At this point we're hill-climbing towards an app that would be on
par with or considered superior to CAEPIPE and other industry standard
applications in this domain, in terms of the modelling characteristics and the
user interface, so if there's a change or expansion in scope that would clearly
benefit that kind of hill climb then we want to do it and you can decide under
the extension of the same standing delegation authorization. If it fixes a
mistake, if it plugs an obvious gap, if it's about making something more
accurate, truthful, or capable without considerable risks of any sort, you can
take initiative and drive scope towards a successful completion of the first
publications ready version of SWBPIPE."). HELP_HUMAN therefore decides scope
changes and expansions that clearly serve that goal and carry no considerable
risk, including commitment and acceptance-criterion text, and may reverse
earlier deferrals to v0.2 where the climb warrants it (owner, 2026-10-27:
"Yes, decide them too"). HELP_HUMAN also approves feature proposals, such as
the CAEPIPE survey's, and reports what it adopted, deferred and why (owner,
2026-10-27: "I decide, you get a summary"). The governing text records each
such decision with its reason. Changed commitment text still gets the
independent review above, and numerical meaning still gets targeted regression
checks and independent scrutiny. Release, acceptance and issuance, risk
acceptance, holds, purchases and the native-smoke `expected.json` values stay
with the owner. The product and data boundaries (`docs/PRODUCT_BOUNDARIES.md`,
`docs/IP_AND_DATA_BOUNDARY.md`) are unchanged.

The owner defined the target on 2026-10-27. Publication-ready means "a version
good enough to publish engineering results with". Its analyses cover the
sustained, expansion and operating cases, hydrostatic test and seismic, and the
other analyses considered standard ("We should be able to do seismic plus
hydrostatic, and the sustained, expansion, operation as you clearly know too.
Anything else that would be considered standard?"). SWBPIPE does not proclaim
code compliance ("For now, it won't proclaim code compliance. I actually want
the user to go to CAEPIPE for that purpose. This software does all the design
iteration."). CAEPIPE is the validation to SWBPIPE: design and analysis features
are to be on par or better, "given real thought for user experience and human
factors in the design and taste in the aesthetics", and a benchmark set compares
SWBPIPE's non-code analysis outputs with CAEPIPE's "across as wide a range as
practicable", which HELP_HUMAN investigates and proposes. HELP_HUMAN drafts the
release target, its exit criteria and the benchmark set for the owner's
acceptance; release remains the owner's. Seismic analysis is static g-loads,
plus response spectrum only where CAEPIPE has that capability (owner,
2026-10-28: "I said you should only develop a response spectrum capability for
seismic analysis if that's a functionality that CAEPIPE has, otherwise you can
go with only static for now."). Benchmark agreement with CAEPIPE is published
(owner, 2026-10-28: "Benchmark agreement with CAEPIPE will indeed be published.
That part should be out in the open because the whole point is to feed into
CAEPIPE and expect the results to come out the same (but validated to Code and
to the reputation of CAEPIPE)."); `docs/IP_AND_DATA_BOUNDARY.md` §6.2 is to be
amended to match, and other evaluation stays private. The target's rows and exit
criteria are in `docs/RELEASE_TARGET_v1.0.md`. Briefs and PRs for work toward the
target name the `RT-` row they serve (HELP_HUMAN's coordination rule, adopted in
its HELP_HUMAN role).

Agent 0 checks that the implementation and its declarations agree before
integration.

## Build and verification

Registered checks are in `software-workflow.json`. From the repository root,
select by project-relative changed paths:

```sh
python3 tools/software_workflow/select_affected_checks.py projects/chirality-piping/software-workflow.json <paths>
```

`docs/CI_STRATEGY.md` describes numerical, persistence, browser and Python
selection. The hosted `harness` result on the exact PR head is the merge check;
DEC-025's blanket local sweep is no longer required. Broad suites run for an
identified need or release preparation.

Pytest builds the Cargo binaries it needs unless `OPENPIPESTRESS_NO_CARGO_BUILD=1`
is set together with `OPENPIPESTRESS_RUNNER_BIN`, `OPENPIPESTRESS_UNITS_BIN`,
`OPENPIPESTRESS_CHECKED_JSON_BIN`, `OPENPIPESTRESS_PREVIEW_RUNNER_BIN` and
`OPENPIPESTRESS_BINARY64_JSON_BIN` (`tests/conftest.py`). The binaries must be
built from the head under test.

Frozen numerical references are in `validation/references/`. Use the existing
generators and preserve the independent reference basis. Numerical changes
require the targeted checks and independent scrutiny specified in Root
`AGENTS.md`.

A change that can move the retained route's memory needs re-pricing with
`tools/retained_pricing/README.md` (rules in DEL-04-08
`Design/retained-pricing-rules.md`) and independent review of the registration.
AC-004's stack witnesses run with no concurrent builds.

## Hard boundaries

- **F-PIP-3 (acceptance and issuance):** acceptance, issuance and release of any
  deliverable, package or product baseline are the owner's acts. Agents do not
  record, imply or derive acceptance, issuance or lifecycle standing from completed
  work, checks, reviews or merges. A baseline the owner has issued or protected
  changes only through a change the owner approves.
