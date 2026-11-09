---
name: swbpipe-development-entry
description: Locate and carry SWBPIPE's release work under the standing project instructions.
metadata:
  kind: instruction
  surface: operating-entry
  upstream:
    source: workflows/project-setup/resources/project-entry.md
    edition: library-v3.3.0
---
# SWBPIPE development entry

SWBPIPE is an analysis-grade piping design engine and stress-model authoring
environment.

## Direction

**Target.** Development is a hill climb to the first publication-ready release,
v1.0, defined by the Must rows in
[`docs/RELEASE_TARGET_v1.0.md` §2](../docs/RELEASE_TARGET_v1.0.md#2-capability-rows)
and the exit criteria in [§4](../docs/RELEASE_TARGET_v1.0.md#4-exit-criteria)
(accepted by the owner, 2026-10-28). Briefs and PRs name the `RT-` row they
serve.

**Opening duty.** At session start, Agent 0 states the path from the current
state to that target, identifies the work that can proceed independently, and
runs it in parallel in standalone Type 1 sessions.

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

## Working locality

Work at this repository root. Read local `AGENTS.md` for shared-library
selection and the applicable active role. Start from the affected
`execution/PKG-nn/DEL-nn-nn/` and its ScopeOfWork, `Work/` and dependency
conditions. Consult `docs/PRD.md` and its companions for requirements and
interfaces involved in this slice, not as a compulsory census. References
written before 2026-10-30 to a deliverable's `Design/` mean its `Work/`.

From the repository root:

```sh
python3 -m tools.deliverables --project . neighborhood DEL-nn-nn
```

Use `impact <DEL>` for downstream reach and `touches <rev|PR:n>` to locate
relevant deliverables; `check` reports structural errors and unresolved
references. Update needs only when their conditions change.

Each implementation return must reconcile its affected `ScopeOfWork.md`,
`Work/` and dependency conditions in `deliverable.yaml` against the implemented
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

The [standing delegation in AGENTS.md](../AGENTS.md#standing-delegation-to-help_human)
records the owner's wording, scope, conditions and reservations. Apply it within
those bounds.

## Build and verification

**Windows development (RT-V7).** macOS is the release baseline and leads. The SWBPIPE Agent 0 owns shared code, schemas and design and decides changes to them; Windows work adds platform implementations at the native boundaries in `execution/PKG-10/DEL-10-04/Work/windows-port.md` and does not change shared behaviour without that agreement. Where a Windows change overlaps macOS work, the macOS work integrates first and the Windows change merges main and regenerates generated files with their generators, never by hand. A Windows change must pass the macOS checks unchanged. Local host resources go to macOS work first; Windows evidence comes from hosted Windows runners. Coordinate through draft PRs that list the shared files they touch before editing; talk directly between sessions for timing, conflicts and host-resource windows; record agreed shared-design decisions by editing the governing text.

Registered checks are in `software-workflow.json`. From the repository root,
select by project-relative changed paths:

```sh
python3 tools/software_workflow/select_affected_checks.py software-workflow.json <paths>
```

`docs/CI_STRATEGY.md` describes numerical, persistence, browser and Python
selection. The hosted `harness` result on the exact PR head is the merge check.
Broad suites run for an identified need or release preparation. In an opted-in
session, send pre-merge consultations to its existing INTERLOCUTOR session.

On the shared T3 development host, start local pytest inside the maintained
`t3_slot.sh` wrapper: session setup can build Cargo CLIs even for Python-only
test selections. The wrapper refuses to start while the host's memory guard is
not running. When that happens, report it to the owner at once as a host
blocker; restarting the guard is not Agent 0's to do. Meanwhile, keep testing:
run the needed local jobs directly, no more at once than the wrapper's slots
allow, and use hosted CI for broad selections. Return to the wrapper when the
guard is back.

Pytest builds the Cargo binaries it needs unless `OPENPIPESTRESS_NO_CARGO_BUILD=1`
is set together with `OPENPIPESTRESS_RUNNER_BIN`, `OPENPIPESTRESS_UNITS_BIN`,
`OPENPIPESTRESS_CHECKED_JSON_BIN`, `OPENPIPESTRESS_PREVIEW_RUNNER_BIN` and
`OPENPIPESTRESS_BINARY64_JSON_BIN` (`tests/conftest.py`). The binaries must be
built from the head under test.

Frozen numerical references are in `validation/references/`. Use the existing
generators and preserve the independent reference basis. Numerical changes
require the targeted checks and independent scrutiny specified in local
`AGENTS.md`.

A change that can move the retained route's memory needs re-pricing with
`tools/retained_pricing/README.md` (rules in DEL-04-08
`Work/retained-pricing-rules.md`) and independent review of the registration.

Ordinary builds and tests run concurrently across isolated worktrees; do not
serialise them for caution (owner, 2026-11-09 04:51 UTC: "Do you need to be so
cautious?"; 05:46 UTC: "you ought to be able to continue testing as you
wanted."). Isolation belongs to a measurement whose evidence concurrent load
would disturb. AC-004's stack witnesses measure memory and timing, so they run
in the supervisor's short exclusive window (`tools/ac004_supervisor.md`), and
other work resumes when it closes.

## Hard boundaries

Apply the [local boundaries in AGENTS.md](../AGENTS.md#local-boundaries), including
F-PIP-3 and the code-compliance boundary.
