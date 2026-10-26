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
quotes the old and new text and the review. Scope, risk acceptance, holds,
release and the native-smoke `expected.json` values stay with the owner.

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
