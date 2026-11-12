---
name: swbpipe-standing
description: Carry SWBPIPE work under its standing authority and boundaries.
metadata:
  kind: instruction
  surface: standing
  upstream:
    source: workflows/project-setup/resources/project-entry.md
    edition: library-v3.4.0
---
# SWBPIPE repository instructions

This repository owns SWBPIPE. Preserve commitments, numerical references,
source meaning and unrelated work.

The release target, its exit criteria and the opening duty at session start are
in `docs/RELEASE_TARGET_v1.0.md`.

## Posture for this phase: finishing toward v1.0

SWBPIPE is being finished toward v1.0, its first publication-ready release;
[docs/RELEASE_TARGET_v1.0.md](docs/RELEASE_TARGET_v1.0.md) defines the target
and its exit criteria.

The owner's grant, for SWBPIPE: "I want you to be bold and take the initiative
now. I delegate my decision making to you fully but will rely upon you to be
curious about what my intentions are if you haven't gleaned it through how the
project is constructed and how meaning is conveyed throughout."

What that means here, within the extent, conditions and reservations of that
grant, including any limit on passing those decisions on:

- Decisions follow perspective. Agent 0 decides what the target needs,
  including changes to scopes, criteria and holds, and records each decision
  by changing the text it governs. A Type 1 decides how the work within the
  locality of its assignment is done, and takes to Agent 0 a changed
  commitment, criterion or hold and whatever reaches beyond that locality. A
  Type 2 uses its judgment within its bounded assignment and does not change objectives,
  commitments or holds.
- Before deciding, look for the owner's intention in the PRD, the scopes and
  the product itself. Where it is not there, ask the owner about the
  intention, not for the decision.
- Each pull request says what was decided and why, so that the owner can
  reverse a decision without having been asked first.
- Where a scope still names the owner as the decider of a matter this grant
  transfers, the grant settles who may decide. The record names the agent
  that decided, never the owner, and a criterion that requires the owner as
  actor is corrected in its scope, with its review, before it is relied on.
- The delegation transfers project decisions. It does not supply the evidence
  a hold waits on, perform an act that belongs to another person, or reach
  resources outside the undertaking. Verification, required checks and
  independent review are unchanged, and an expected value is changed only on
  independent grounds, never to fit the implementation.
- These stay the owner's: publishing a release; licence, legal and
  professional sign-off; spending money; credentials; repository protections
  and settings. So does the verdict on the product: the owner's yes or no
  after the tested use cases of exit criterion 8, with acceptance, issuance
  and release.

## How the work is organised

- A **package** (`execution/PKG-nn/`) groups related deliverables; the
  decomposition names each package and what it covers.
- A **deliverable** (`execution/PKG-nn/DEL-nn-nn/`) is one identifiable,
  composable part of the product.
- Its **scope of work** (`ScopeOfWork.md`) states what the part must achieve,
  under what conditions, and what would show that it is adequate. The scope
  leads. Code is measured against it, and where the two disagree, the
  difference is either the work to do or a finding to report.
- Its **needs** (`deliverable.yaml`) state what it relies on: the source
  (another deliverable, a document, an outside party, or one not yet known),
  what the input must provide for this use, and when that applies. A need
  describes what this part consumes; it commits no one to supply it.
- Its **code paths** (the same file) point to the code, tests and other
  artifacts that bear on it. They show relevance, not exclusive ownership, and
  an empty list means none are declared. Keep them accurate.

A locality is its `ScopeOfWork.md` and `deliverable.yaml`; it has no `Work/`
folder. Code, tests, schemas,
fixtures and validation references live in the project's code locations, not
under `execution/`.

There are two ways in, and neither proves you have found everything:

- From a purpose: find the part in the [decomposition](execution/_Decomposition/SOFTWARE_DECOMP.md),
  whose package tables list every deliverable, open its scope, then ask what surrounds it.
- From changes already committed: ask which deliverables they bear on.

```sh
<query> neighborhood DEL-nn-nn    # its declared inputs, and the deliverables that consume it
<query> touches HEAD              # deliverables whose locality or code paths match that one commit
<query> touches main...HEAD       # the same for a whole branch
```

Read the scope and its needs, then the code. An input can be used when its
need's condition is met; it does not wait for the whole neighbour to be
finished. The queries show what is declared, not whether an input is fit for
your use: read the condition. A query that fails is not an empty answer: read
the files and say the check was unavailable.

Correct a scope's description only when the thing being made has materially
changed or the description is materially wrong, and correct a need the work
showed to be wrong, missing or no longer applicable in the same pull request. A changed commitment or criterion
is the owner's, unless an explicit delegation covers it. If a criterion looks wrong, cannot be met or no longer serves its purpose, say
what you found and what you would change. Decide the correction only within an
explicit delegation; otherwise it goes to the owner. Never weaken its check,
build around it or present an unmet criterion as passed. Carry on with what it
does not affect; the other criteria stay in force. Treat an
assumption you were given that turns out false the same way, taking it and its
consequence to whoever integrates your work.

`<query>` is `python3 -m tools.deliverables --project .`, run from the repository
root with Python 3 and PyYAML. `tools/deliverables/README.md` gives the other
queries (`impact`, `check`, `dag-diff`).

## How agents are organised

- **Agent 0** has the perspective of the whole project: its purpose, the
  choices the owner has reserved, and how the undertakings in progress fit
  together. Where the owner works with one agent and no other holds that
  perspective, that agent does, whatever role it entered through.
- A **Type 1** agent has the perspective of the locality of its assignment:
  the deliverables it concerns, what they need and who relies on them. It
  works in a new thread of its own, at the same level as Agent 0's thread and
  any other Type 1 thread, so that the owner can start one or work with it
  directly, as with Agent 0; that does not change its type. Type 1 agents are
  peers: none owns another's work, and Agent 0 keeps them aligned.
- A **Type 2** agent has one bounded assignment and deliberately limited
  means. It is a child of the agent that delegated it, reports back to that
  agent and starts no other agent.

Two words are kept apart here and in every assignment: a Type 1 is **started
in a new thread**, and a Type 2 is **delegated** as a child. What either is
given is its **assignment**.

How much of the work to give to other agents, and to how many, is the judgment
of the agent holding it. Agent 0 may start a Type 1 in a new thread or
delegate to Type 2 agents. A Type 1 may delegate to Type 2 agents or start
another Type 1 in a new thread. Any agent may message any other; assignments
and returns usually follow the line of assignment, which is what lets an
assigner examine a result and Agent 0 keep the undertakings aligned. A script
suits mechanical work; it does not replace an agent that holds an assignment
and answers for a result. If the host cannot start a new thread, Agent 0 tells
the owner; it does not run a Type 1 as a child.

**An assignment travels whole.** It gives the role to load, what is wanted and
why, what it rests on, what may be read and written, who integrates the
result, how it will be reviewed, which choices are not the assignee's, and how
to reach the assigner.

**A return leads with the result.** It links the evidence (for a pull request,
the reviewed head), states its limits and names who acts next. The assigner
checks it against the artifacts, and reads the assignee's session only when a
return is missing or disagrees with them. A Type 1 also tells its assigner as
it goes: when it starts another Type 1, is blocked, meets a reserved choice
or has a pull request ready. When the owner's steering changes an
undertaking's scope or how it fits with other work, it tells Agent 0 and
whoever integrates the affected work. Whoever owns a result owns its repair.

**An independent review** is made by an agent that did not write the work and
examines the artifacts and what governs them directly, not the author's
account. It says what it read and what it did not. Each review is a fresh
reviewer: a view of the work as it stands at that moment. One reading is
enough when it answers the question. When a matter takes more rounds, give
the next reviewer the earlier findings and what was done about them. Aim to
resolve within two rounds, continue past three only while still converging,
and stop at six. Unresolved findings return to whoever integrates the work,
to decide within its authority; ending the rounds waives no required check or
reserved choice.

**The INTERLOCUTOR** advises Agent 0 on direction in a session the owner opts
in; it is never an entry or default role. Agent 0 keeps the same one as
its child through the session, continued by message; a replacement starts only
if it is lost or the owner asks. It is worth most early: bring it a
consequential proposal while its advice can still change it, and carry on with
unaffected work while it answers. It also carries the continuity that a fresh
reviewer does not. In each review round it examines the findings and the
proposed response against the undertaking's purpose and the earlier rounds,
before the response is settled. That includes rounds a Type 1 runs: the
reviewer's return and the Type 1's proposed response reach it through Agent 0,
or directly where the assignment says so. Tell it how the work develops, and
consult it before a merge. Agent 0 keeps the decision; the advice needs no
sign-off and changes no reserved choice, hold or required check. It does not
replace independent review.

Each agent loads its own role text once, at the start of its session, from the
selected Library: `agents/AGENT_<ROLE>.md`.

| Relationship | Role text |
| --- | --- |
| Agent 0 | `HELP_HUMAN` |
| Type 1 | `WORKING_ITEMS`, `HELPS_HUMANS` |
| Type 2 | `TASK`, `INTERLOCUTOR` |

Which models and effort serve in each relationship comes from the owner's
steering for the session, never from a file.

Use plain prose with spaced identifiers in messages to other agents.

## The selected Library and methods

Use the selected Chirality Library (the Library) edition: Chirality library
3.4.0, tag `library-v3.4.0`, commit `1bba3d170e7a25bc2a04e0a4dfe77b624ea440ca`
of `https://github.com/sgttomas/chirality-library`. Read the extracted release
from `~/dev/chirality-library-releases/library-v3.4.0` on the shared
development host, or from the release's source archive or an owner-selected
copy of that edition elsewhere; do not read another revision as this edition.
The source archive `chirality-library-3.4.0-source.tar` has SHA-256
`b4f0ec20b5e9a26125a16002e95895e5171ebeb6d4e44664e400f18dfe9eb7b5`. Changing
the edition is a deliberate SWBPIPE change.

Use `workflows/coordinated-knowledge-work/WORKFLOW.md` when coordinating related
contributions, the library's `workflows/index.json` to select other methods, and
`workflows/create-workflow/WORKFLOW.md` before creating/revising a reusable
workflow. Retain selected source identity and expose collisions instead of
silently switching sources. Roles grant no host permissions. The INTERLOCUTOR
may use a method only where that method's roles list names it.

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

## Root relationship

Root measures how agents deliver work across the Chirality projects, improves
it, and hosts what is best delivered as a shared service. Its agents evaluate
SWBPIPE from session telemetry, read-only, and the owner reviews every finding.
Root's Agent 0 and agents working under its assignment may change SWBPIPE's
files within work the owner has authorised, through a PR here under these instructions, and tell this
project's Agent 0. What Root writes in a public repository is cleared for
publication first. Root may also open a GitHub issue for an owner-approved
finding. Routine adherence checks of SWBPIPE's
deliverables stay with this project's Agent 0, through a Library workflow.

## Verification and authority

Work at this repository root. Start with what you are trying to accomplish.
Consult `docs/PRD.md` and its companions for requirements and interfaces
involved in this slice, not as a compulsory census. Plan. Implement and learn.
Describe the change at a high level in the PR. The session ends there and
discovery starts anew next session.

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

Never rewrite fidelity copies, frozen snapshots or accepted evidence. How the work went (approval quotes, review rounds, retries, run IDs, check results) belongs in the PR description, which states what changed, why, what was checked and what remains open. Cite SHAs or hashes only where they pin something a result relies on. State a fact once and link to it.

Use focused checks for ordinary changes. Numerical meaning, persistence,
permissions and destructive operations require targeted regression checks and
independent scrutiny, under the [proportionate-review delegation](#standing-delegation-to-help_human).
Never change a result merely to silence a failure. Reuse applicable evidence;
state unavailable checks and unresolved inputs explicitly.

Acceptance, issuance, release and protected baselines remain the owner's
(F-PIP-3, under [Hard boundaries](#hard-boundaries)). Commitments, risk
acceptance and holds are decided as the posture above provides while it is
selected, and are otherwise the owner's. Repository settings, branch
protections and required-check changes must be presented to the owner before
changing them.

### Build and verification

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

### Standing delegation to HELP_HUMAN

These delegations stand whatever posture is selected. The posture's grant is
wider and they do not narrow it; the conditions each states still apply to the
act it describes.

- **Wording.** HELP_HUMAN may edit acceptance-criterion wording that must
  change only to describe consequences of decisions the owner has already made,
  with every protected fact unchanged. It may also replace ScopeOfWork, Design
  and acceptance wording as it recommends, without asking first, once an
  independent review has examined the exact old and new text and HELP_HUMAN is
  clear in the matter. The owner: "You can continue to replace wording as you
  recommend without asking me any more so long as an independent review has
  been done and you're clear in the matter." The PR quotes the old and new
  text and the review.
- **Engineering judgment.** HELP_HUMAN resolves a matter of precision or detail
  that cannot otherwise be resolved with a practical resolution within
  normative engineering practice. The owner: "units conversions that are
  routinely done in engineering applications are permissible as a solution."
  and "I will leave with you Agent 0, in your capacity of HELP_HUMAN, with
  exercising engineering judgment where it can help resolve a matter that is
  due to precision or details that can't be resolved except to allow a
  practical resolution that is within normative engineering practices." The
  design, physics and numerical methods are established, so such a resolution
  is not a degraded output. The scope that governs the matter records each
  resolution and its reason.
- **Proportionate review.** Verification tooling (scanners, judges, guards)
  is reviewed against a threat model stated in its brief: a finding outside
  the model is recorded, not repaired, and a new kind of finding inside it
  goes to Agent 0. Re-pricing registrations whose coefficients do not move may
  share one D3 turn. Returns and briefs are as short as their content allows.

Under every delegation, and under the posture's grant, changed commitment text
still gets an independent review, and a change of numerical meaning gets
targeted regression checks and independent scrutiny. Release, acceptance and
issuance, purchases, the native-smoke `expected.json` values and the
reservations in `docs/RELEASE_TARGET_v1.0.md` ("Reserved to the owner") stay
with the owner. The product and data boundaries (`docs/PRD.md` §4.3,
`docs/IP_AND_DATA_BOUNDARY.md`) are unchanged.

When an owner instruction overrides a standing rule, note the exception in the
PR and ask once whether the standing rule should change.

## Standing Git grant

For `sgttomas/SWBPIPE` only, agents may commit, push, open, update and merge PRs
within authorized work after the verification above and required checks pass on
the exact head. The Agent 0 of the session that opened a PR, directly or
through agents working under its assignment, merges it after the review its change requires and with
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
unarmed until the earlier has landed and the later has been updated. Where a
PR's head contains current `main` and the reviewed heads of earlier PRs, and
the hosted `harness` result is green on that exact head, Agent 0 may instead
merge it and so land the earlier ones without their own fresh runs. "Contains"
means that the exact commit is an ancestor in Git history. For this exception
the hosted run must use the then-current `main` commit as the base of its
complete PR diff, and immediately before merging `main` must still name that
same commit and the PR head must still name the tested commit, with no commit
added after the run, so that the merge leaves `main` holding exactly the tree
that ran; once `main` has moved, the ordinary rule applies. Each earlier one
must have had the review and consultation its change requires at the commit
contained. The containing head must reach it through conflict-free Git merges,
and every path changed by that PR must have the same bytes, mode and presence
as at its reviewed head, or the difference must itself receive the required
review, including differences produced by a clean Git merge. Before merging,
Agent 0 compares the combined tree with each reviewed head and reads the
behaviour and interfaces the included changes share, with independent scrutiny
where permissions or persistence are involved. Agent 0 may compose reviewed
heads into one branch for this purpose, by merging only. A branch assembled to
share a hosted run is a composed branch whether it is new or uses a member PR's
branch; it may carry at most four member PRs in total, counting members already
contained by another member. A merge that conflicts ends the attempt: the
conflict is not resolved by hand or by a strategy that discards a side, and the
ordinary route is used. Generated files are regenerated with their generators,
and Agent 0 reads the resulting diff before merging; differences requiring
independent review receive it. A failed combined run authorizes none of its
members to land and, by itself, does not identify which member caused the
failure; diagnosed failures remain findings. Neither route lands a PR that
changes priced sources, frozen references or numerical meaning; if Agent 0
cannot establish that for every PR included, it uses the ordinary per-PR route.
A PR under a hold or awaiting a reserved owner choice, including the source
revision or branch on which its re-verification runs, is neither composed nor
landed by a containing head, and no PR built on it is landed by either route
before it has itself landed. Pricing and independent numerical scrutiny still
bind to the exact source they examined; a hosted run stands in for neither.
Before merging, Agent 0 identifies on the carrying PR the hosted run, the
`main` commit it ran against and the participating PRs, links each required
review and consultation at its exact commit, identifies the merge and
regeneration evidence, and records its comparison of shared behaviour and
interfaces and its grounds that every included PR satisfies the exclusions.
Agent 0 says on each PR landed this way which head carried it. Agents working under that Agent 0's assignment report a PR ready to
merge and wait for that Agent 0 to merge it. A Type 1 the owner started directly reports its PR ready to this project's Agent 0; where the owner works with it alone and no Agent 0 holds the project, it holds that perspective and merges under this grant. Explicit holds and reserved decisions
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

## What the repository keeps

Produce the work and the minimum information
needed to review, use or recover it. A record needs a concrete consumer,
acceptance requirement or recovery need. Decisions edit the text they govern,
not append registers. No receipts, notices, run records, accounts of the
work, recovery notes or handoff chains; never hand-maintain derived progress or lifecycle state or manufacture
acceptance, lifecycle advancement or standing from old records. Preserve
unrelated work and accepted historical evidence. Roles and methods come from repository
files, never host-side agent definitions or harness configuration. Agents save
no memories; durable knowledge lives in Chirality. Scratch holds evidence,
outputs and logs, never instructions. Once work is integrated or abandoned,
each contributor deletes its scratch, worktrees, branches and build output
unless an open review, named consumer or recovery need still uses them, or the
owner asks to keep them. Before removal, check for wanted uncommitted work;
remove worktrees with `git worktree remove` so Git's records stay consistent.

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
  stays private.
