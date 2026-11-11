---
name: swbpipe-standing
description: Carry SWBPIPE work under its standing authority and boundaries.
metadata:
  kind: instruction
  surface: standing
  upstream:
    source: workflows/project-setup/resources/project-entry.md
    edition: library-v3.3.0
---
# SWBPIPE repository instructions

This repository owns SWBPIPE. Preserve commitments, numerical references,
source meaning and unrelated work.

The release target, its exit criteria and the opening duty at session start are
in `docs/RELEASE_TARGET_v1.0.md`.

## Shared roles and methods

Use the selected Chirality Library (the Library) edition: Chirality library
3.3.0, tag `library-v3.3.0`, commit `ec3ee2d1b398eba1fb3227ccf93e2569df95bacd`
of `https://github.com/sgttomas/chirality-library`. Read it from the
release's source archive, or from an owner-selected copy of that edition
elsewhere; do not read another revision as this edition. Changing the
edition is a deliberate SWBPIPE change. 

| Role | Type | Contribution |
|---|---|---|
| HELP_HUMAN | 0 | Purpose, alignment and continuity with the owner |
| HELPS_HUMANS | 1 | Design workflows, tools and projects with the owner |
| WORKING_ITEMS | 1 | Own implementation, assignments and integration |
| TASK | 2 | Execute one bounded assignment; does not delegate |
| INTERLOCUTOR | 2 | Opt-in advice to Agent 0 on its effectiveness and direction |

Use only the hierarchy the undertaking needs. 

Agent 0 delegates a Type 1 role (HELPS_HUMANS or WORKING_ITEMS) to its own
standalone session or thread, never as a child within Agent 0's session or
thread. Agent 0 or a Type 1 agent delegates to children within its own session or thread.
A WORKING_ITEMS agent may call in a HELPS_HUMANS agent directly, which like
every Type 1 runs in its own standalone session or thread; other engagements
between Type 1 agents go through Agent 0. Peers do not own one another; Agent 0
keeps them aligned. A Type 2 role runs as a child within the session or
thread of the agent that delegates it, whether Agent 0 or a Type 1; it never
gets a standalone session. Agent 0 gives each Type 1 its full
assignment at the start, including how to reach Agent 0. A Type 1 reports to
Agent 0 by message when it calls in a HELPS_HUMANS agent, reaches a result, has
a PR ready to merge, is blocked or meets a reserved choice, stating the result,
links to its evidence (with the reviewed head SHA for a PR), limits and the next
actor. Agent 0 checks a report against the artifacts it names and reads the
Type 1's session only when a report is missing or disagrees with them. If the
host cannot start a standalone session, Agent 0 tells the owner rather than
nesting the role.

INTERLOCUTOR is opt-in per session and is not selected by default for any
session or for this repository. When session steering selects it, Agent 0 starts
one INTERLOCUTOR as a child within its own session and continues that same child
by message for the life of the Agent 0 session; a replacement starts only if it
is lost or the owner asks. Agent 0 sends periodic activity summaries and
consults it at least before every PR merge, as `agents/AGENT_INTERLOCUTOR.md`
describes. Its advice is not a required check, sign-off or veto and changes no
owner-reserved choice. It may use a method only where that method's roles list
names it.

Use `workflows/coordinated-knowledge-work/WORKFLOW.md` when coordinating related
contributions, the library's `workflows/index.json` to select other methods, and
`workflows/create-workflow/WORKFLOW.md` before creating/revising a reusable
workflow. Retain selected source identity and expose collisions instead of
silently switching sources. Roles grant no host permissions; model/effort
choices come from session steering. Give delegated work an input basis, scope,
read/write boundaries, integration/review route and reserved decisions.

Use plain prose with spaced identifiers in messages to delegates.

The Library's own work locations, duties and Git grants do not transfer to
this project. Shared-method consultation does not authorize changes to another
repository. Ordinary product build/test/CI uses this repository's maintained
tools and fixtures and must not require the Library checkout. 

## Coordination between projects

The Chirality Library (`sgttomas/chirality-library`), Chirality Root
(`sgttomas/chirality`), App v4 (`sgttomas/chirality-app`) and SWBPIPE
(`sgttomas/SWBPIPE`) each have one top-level Agent 0, working in its own 
session. When work here needs another of these projects, such as a shared
interface, a Library change a consumer will adopt, a defect found in the other
project, or a shared release gate, this project's Agent 0 messages that
project's Agent 0 directly. Read the recipient's session first to confirm that it is
that project's current Agent 0; a retired session is not a route.

A message states what is needed, the revision and files it concerns, and
whether it blocks the sender. The recipient decides and acts within its own
repository and authority. A message carries no owner authority and changes
nothing in another repository. Record an agreed decision by editing the
governing text in the repository that owns it, and take reserved choices to the
owner. Do not message a project the work does not affect.

Root measures how agents deliver work across the Chirality projects, improves
it, and hosts what is best delivered as a shared service. Its agents evaluate
SWBPIPE from session telemetry, read-only, and the owner reviews every finding.
Root changes no SWBPIPE files, branches or PRs; its only action here is a GitHub
issue for an owner-approved finding. Routine adherence checks of SWBPIPE's
deliverables stay with this project's Agent 0, through a Library workflow.

## Work and verification

There is the work, which is the code, and what expresses its goal, which is
the scopes of work; the scope leads (the Library's
`workflows/scope-of-work/WORKFLOW.md`, "What a scope says").

Start with what you are trying to accomplish. Read the scopes you think
relevant and what you find along the way. Plan. Implement and learn. Reconcile
a scope only if the thing being made, or its description within the six parts,
materially changed; correct an affected need in the same PR. Describe the
change at a high level in the PR. The session ends there and discovery starts
anew next session.

If you conclude that a criterion is wrong, cannot be met or no longer serves
its purpose, do not change it, weaken its check or build around it. Say what
you found and what you would change. Decide it only within an explicit
delegation; otherwise it goes to the owner. The other criteria stay in force.

Never rewrite fidelity copies, frozen snapshots or accepted evidence. How the work went (approval quotes, review rounds, retries, run IDs, check results) belongs in the PR description, which states what changed, why, what was checked and what remains open. Cite SHAs or hashes only where they pin something a result relies on. State a fact once and link to it.

- **What the repository keeps.** Produce the work and the minimum information
  needed to review, use or recover it. A record needs a concrete consumer,
  acceptance requirement or recovery need. Decisions edit the text they govern,
  not append registers. No routine receipts, notices, run records or handoff
  chains; never hand-maintain derived progress or lifecycle state or manufacture
  acceptance, lifecycle advancement or standing from old records. Preserve
  unrelated work and accepted historical evidence. Keep one replaceable recovery
  note only where continuity needs it. Roles and methods come from repository
  files, never host-side agent definitions or harness configuration. Agents save
  no memories; durable knowledge lives in Chirality. Scratch holds evidence,
  outputs and logs, never instructions. Once work is integrated or abandoned,
  each contributor deletes its scratch, worktrees, branches and build output
  unless an open review, named consumer or recovery need still uses them, or the
  owner asks to keep them. Before removal, check for wanted uncommitted work;
  remove worktrees with `git worktree remove` so Git's records stay consistent.

Code that runs (implementation, tests, generators, harnesses) lives outside `execution/`, in the project's code locations, and deliverables link to it.

Use focused checks for ordinary changes. Numerical meaning, persistence,
permissions and destructive operations require targeted regression checks and
independent scrutiny, under the [proportionate-review delegation](#standing-delegation-to-help_human).
Never change a result merely to silence a failure. Reuse applicable evidence;
state unavailable checks and unresolved inputs explicitly.

Commitments, risk acceptance, protected baselines, holds, issuance and release
remain subject to the owner's authority and explicit local delegations.
Repository settings, branch protections and required-check changes must be
presented to the owner before changing them.

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
values and the reservations in `docs/RELEASE_TARGET_v1.0.md` ("Reserved to the owner") stay with the
owner. The product and data boundaries (`docs/PRD.md` §4.3,
`docs/IP_AND_DATA_BOUNDARY.md`) are unchanged.

When an owner instruction overrides a standing rule, note the exception in the
PR and ask once whether the standing rule should change.

## Standing Git grant

For `sgttomas/SWBPIPE` only, agents may commit, push, open, update and merge PRs
within authorized work after the verification above and required checks pass on
the exact head. The Agent 0 of the session that opened a PR, directly or
through its delegates, merges it after the review its change requires and with
required checks green on the exact head, having first triaged any open issue
labelled `nightly` (the PR that repairs one may merge). If that session has ended, the current
Agent 0 takes that duty. Agent 0 may arm GitHub
auto-merge instead of merging by hand. It arms only after the review and any
INTERLOCUTOR consultation are done and the last push is made, against the
reviewed head, and it triages `nightly` issues when arming. It disarms before
any further push to the branch. `main` does not require an up-to-date branch:
before merging or arming, Agent 0 updates the branch and waits for a fresh run
when a PR merged since the branch's base touched the same files or the same
deliverable. Open PRs that overlap this way merge in turn: the later one stays
unarmed until the earlier has landed and the later has been updated. Delegated agents report a PR ready to
merge and wait for Agent 0 to merge it. Explicit holds and reserved decisions
apply. This grant excludes protection bypasses, history rewrites, force pushes,
permission changes and releases. Use [Git conventions](#git-conventions) below.

## Git conventions

- Treat `main` as SWBPIPE's integration branch. Name task branches with a prefix that identifies the authoring agent, such as `codex/` or `claude/`, unless the user specifies otherwise. Integrate a PR with a merge commit (no fast-forward) unless the PR chooses otherwise.
- Use a separate worktree when the user requests isolation or when concurrent sessions make isolation proportionate, such as overlapping writes to governance surfaces, risky refactors, or tool interference. Do not require a worktree for every change. High-risk overlap paths here include `AGENTS.md`, governance documents under `docs/`, accepted snapshots, generated derivative packages, and shared control roots such as `execution/_Coordination/`; warn when two lanes overlap on them.
- Close authorized work through a PR. No separate commit, push, PR or merge approval is needed when its conditions hold. Use the configured authenticated identity and retain truthful authorship; agent review is not personal owner review.
- Verify in proportion to consequence and confirm required checks pass on the exact head. Obtain independent scrutiny where the grant requires it. Clear blocking findings and reassess affected checks after candidate changes; a passing rerun does not establish that a known defect was repaired. Verify the source HEAD when merging.
- Git integration, human acceptance and product release remain distinct.
- Update affected live consumers and references in the change itself.

For implementation and closeout in this repository:

- Separate maintained source and test fixtures from run evidence. Permanent tests and public packaging should not depend on a dated agent-run directory. Keep useful regression cases in the maintained test suite; preserve one-time migration evidence as history rather than requiring the migration to be re-proved forever.
- Keep working summaries revisable until a real review or acceptance boundary. Format authored documents and generate derived files before binding their bytes for review. Preserve raw tool output verbatim when needed; do not make it an editable, style-normalized source artifact. A later correction needs evidence for the affected change, not another full copy of the previous packet.
- Cosmetic whitespace is not a commit or merge blocker. Use formatting diagnostics only when helpful; preserve intentional Markdown hard breaks and raw evidence rather than creating repair cycles for their appearance.
- Judge a check by the failure it prevents. Keep behavioral, permission, containment, compatibility, and reproducibility checks. Remove obsolete migration assertions, incidental wording/order assertions, and duplicate evidence checks when they protect no current requirement. When a governing instrument requires such a check, propose the corresponding amendment rather than silently bypassing it or adding another workaround.
- For changes crossing the engine, the desktop app and the embedded agent's provider adapters, validate the actual connecting path early. Separate component passes do not establish integration. Check the deliverable from committed or explicitly selected files, including its maintained fixtures and resources; untracked local evidence must not supply a hidden dependency.
- Match preflight to the actual candidate and affected CI entrypoints. Scope candidate checks to files being submitted, and run required range checks against the complete PR diff. Re-run affected checks after repairs; repeat broader suites only when the change or an unresolved concern warrants it.

## Working locality

Work at this repository root. Start from the affected
`execution/PKG-nn/DEL-nn-nn/`: its `ScopeOfWork.md` and the dependency
conditions in `deliverable.yaml`. Consult `docs/PRD.md` and its companions for
requirements and interfaces involved in this slice, not as a compulsory census.

From the repository root:

```sh
python3 -m tools.deliverables --project . neighborhood DEL-nn-nn
```

`tools/deliverables/README.md` gives the other queries (`impact`, `touches`,
`check`, `dag-diff`). Update needs only when their conditions change.

Each implementation return must reconcile its affected `ScopeOfWork.md` and
dependency conditions in `deliverable.yaml`. The implementation
manager owns this reconciliation: it carries the rule into every brief and
names the affected deliverables, and TASKs report the effects of their slices.
Update changed meaning in the same PR; leave unchanged declarations alone, and
say so briefly when nothing needs editing. The return identifies the
reconciliation and any remaining conditions or reserved choices. Agent 0
checks that the implementation and its declarations agree before integration.

Carry each unit to a usable, verified result. Record improvements found along
the way as remaining items, and take them up after the unit unless they correct
a wrong output or block the goal.

## Build and verification

**Windows development.** macOS is the release baseline and leads. The SWBPIPE Agent 0 owns shared code, schemas and design and decides changes to them; Windows work adds platform implementations at the native boundaries DEL-10-04 owns (`execution/PKG-10/DEL-10-04/ScopeOfWork.md`) and does not change shared behaviour without Agent 0's agreement. Where a Windows change overlaps macOS work, the macOS work integrates first and the Windows change merges main and regenerates generated files with their generators, never by hand. A Windows change must pass the macOS checks unchanged. Local host resources go to macOS work first; Windows evidence comes from hosted Windows runners. Coordinate through draft PRs that list the shared files they touch before editing; talk directly between sessions for timing, conflicts and the AC-004 measurement window; record agreed shared-design decisions by editing the governing text.

Registered checks are in `software-workflow.json`. From the repository root,
select by project-relative changed paths:

```sh
python3 tools/software_workflow/select_affected_checks.py software-workflow.json <paths>
```

`docs/CI_STRATEGY.md` describes numerical, persistence, browser and Python
selection. The hosted `harness` result on the exact PR head is the merge check.
Broad suites run for an identified need or release preparation.

On the shared T3 development host, start local pytest inside the maintained
`t3_slot.sh` wrapper: session setup can build Cargo CLIs even for Python-only
test selections. The wrapper refuses to start while the host's memory guard is
not running. When that happens, report it to the owner at once as a host
blocker; restarting the guard is not Agent 0's to do. Meanwhile, keep testing.
Agent 0 coordinates the needed local jobs across all lanes within the wrapper's
shared slot allowance, because direct jobs are invisible to the slot locks.
Direct jobs finish before an AC-004 exclusive window begins, and nothing here
bypasses the AC-004 supervisor's own refusal. Where that coordination cannot be
kept, use hosted CI. Hosted CI stays the default for broad selections. Return
to the wrapper when the guard is back.

Pytest builds the Cargo binaries it needs unless prebuilt ones are supplied as
`tests/conftest.py` describes (`core/serialization/canonical_json/README.md` for
the binary64 executable). They must be built from the head under test.

Ordinary builds and tests run concurrently across isolated worktrees; do not
serialise them for caution. Isolation belongs to a measurement whose evidence
concurrent load would disturb: AC-004's stack witnesses run in the supervisor's
exclusive window (`tools/ac004_supervisor.md`).

## Hard boundaries

Keep credentials and private/proprietary engineering inputs out of Git. Preserve
the local IP/data and professional boundaries. No supplier acquisition, sign-in,
keychain use, signing or live qualification follows from a repository migration.

- **F-PIP-3 (acceptance and issuance):** acceptance, issuance and release of any
  deliverable, package or product baseline are the owner's acts. Agents do not
  record, imply or derive acceptance, issuance or lifecycle standing from completed
  work, checks, reviews or merges. A baseline the owner has issued or protected
  changes only through a change the owner approves.
- SWBPIPE does not proclaim code compliance. Benchmark agreement with CAEPIPE
  is published as `docs/RELEASE_TARGET_v1.0.md` ("Benchmark method") provides; other evaluation
  stays private ("Owner rulings" records them).
