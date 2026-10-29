# SWBPIPE development entry

SWBPIPE is an analysis-grade piping design engine and stress-model authoring
environment. Development is a hill climb to the first publication-ready
release, v1.0. Its target, capability rows and exit criteria are in
`docs/RELEASE_TARGET_v1.0.md`, which the owner accepted on 2026-10-28. Briefs
and PRs for that work name the `RT-` row they serve.

Work in `projects/chirality-piping`. Start from the affected
`execution/PKG-nn/DEL-nn-nn/` and its ScopeOfWork, Design and dependency
conditions. Consult `docs/PRD.md` and its companions for requirements and
interfaces involved in this slice, not as a compulsory census.

From the repository root:

```sh
python3 -m tools.deliverables --project projects/chirality-piping neighborhood DEL-nn-nn
```

Use `impact <DEL>` for downstream reach and `touches <rev|PR:n>` to locate
relevant deliverables; `check` reports structural errors and unresolved
references. Update needs only when their conditions change.

Each implementation return must reconcile its affected `ScopeOfWork.md`,
`Design/` and dependency conditions in `deliverable.yaml` against the implemented
behaviour and verification evidence before integration. The implementation
manager owns this reconciliation: it carries the rule into every brief and
names the affected deliverables, and TASKs report the effects of their slices.
Update changed meaning in the same PR; leave unchanged declarations alone, and
say so briefly when nothing needs editing. The return identifies the
reconciliation and any remaining conditions or reserved choices. Agent 0
checks that the implementation and its declarations agree before integration.

Carry each unit to a usable, verified result (owner, 2026-10-26: "we are
nearing a level of maturity where there needs to be a concerted drive towards
goals and finishing things to a practical level. Continuous improvement goes
after."). Record improvements found along the way as remaining items, and take
them up after the unit unless they correct a wrong output or block the goal.

## Standing delegation to HELP_HUMAN

- **Wording.** HELP_HUMAN may edit acceptance-criterion wording that must
  change only to describe consequences of decisions the owner has already made,
  with every protected fact unchanged (owner, 2026-10-19). It may also replace
  ScopeOfWork, Design and acceptance wording as it recommends, without asking
  first, once an independent review has examined the exact old and new text and
  HELP_HUMAN is clear in the matter (owner, 2026-10-23: "You can continue to
  replace wording as you recommend without asking me any more so long as an
  independent review has been done and you're clear in the matter."). The PR
  quotes the old and new text and the review.
- **Engineering judgment.** HELP_HUMAN resolves a matter of precision or detail
  that cannot otherwise be resolved with a practical resolution within
  normative engineering practice (owner, 2026-10-26: "units conversions that
  are routinely done in engineering applications are permissible as a
  solution." and "I will leave with you Agent 0, in your capacity of
  HELP_HUMAN, with exercising engineering judgment where it can help resolve a
  matter that is due to precision or details that can't be resolved except to
  allow a practical resolution that is within normative engineering
  practices."). The owner's basis is that the design, physics and numerical
  methods are established, so such a resolution is not a degraded output. The
  governing Design text records each resolution and its reason.
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
- **Proportionate review** (HELP_HUMAN's proposal, approved by the owner on
  2026-10-28: "These are all good changes to bring about and I approve.").
  Independent review rounds are for numerical meaning, persistence and
  permissions; another change gets at most one review round. Verification
  tooling (scanners, judges, guards) is reviewed against a threat model stated
  in its brief, for at most three adversarial rounds: a finding outside the
  model is recorded, not repaired, and a new kind of finding inside it goes to
  Agent 0. Re-pricing registrations whose coefficients do not move may share
  one D3 turn. Returns and briefs are as short as their content allows.

Under every delegation, changed commitment text still gets an independent
review, and a change of numerical meaning gets targeted regression checks and
independent scrutiny. Scope beyond this delegation, release, acceptance and
issuance, risk acceptance, holds, purchases, the native-smoke `expected.json`
values and the reservations in `docs/RELEASE_TARGET_v1.0.md` §5 stay with the
owner. The product and data boundaries (`docs/PRODUCT_BOUNDARIES.md`,
`docs/IP_AND_DATA_BOUNDARY.md`) are unchanged.

## Build and verification

Registered checks are in `software-workflow.json`. From the repository root,
select by project-relative changed paths:

```sh
python3 tools/software_workflow/select_affected_checks.py projects/chirality-piping/software-workflow.json <paths>
```

`docs/CI_STRATEGY.md` describes numerical, persistence, browser and Python
selection. The hosted `harness` result on the exact PR head is the merge check.
Broad suites run for an identified need or release preparation.

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
- SWBPIPE does not proclaim code compliance. Benchmark agreement with CAEPIPE
  is published as `docs/RELEASE_TARGET_v1.0.md` §3 provides; other evaluation
  stays private (§1 records the owner's rulings).
