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

This repository owns SWBPIPE. Enter through `loop/LOOP_INIT.md`; requirements,
product/data boundaries, adopted work localities and verification are local.
Preserve commitments, numerical references, source meaning and unrelated work.

## Shared roles and methods

Use the selected Chirality Library (the Library) edition: Chirality library
3.3.0, tag `library-v3.3.0`, commit `ec3ee2d1b398eba1fb3227ccf93e2569df95bacd`
of `https://github.com/sgttomas/chirality-library`. Read it from that commit in
`/Users/ryan/dev/chirality-library` on this workspace
(`git -C /Users/ryan/dev/chirality-library show <commit>:<path>`), from the
release's source archive, or from an owner-selected copy of that edition
elsewhere. That checkout's working tree is the selected source only while it is
at that commit; do not read another revision as this edition. Changing the
edition is a deliberate SWBPIPE change. Record the resolved Library revision
with results that depend on it. Resolve shared `agents/` and
`workflows/` paths against the selected source. Tool-use
instructions are repository content kept beside the tool they instruct, not
harness skills. Load the active role's `agents/AGENT_<ROLE>.md` and other method bodies
only when needed. HELP_HUMAN retains purpose/alignment; HELPS_HUMANS designs;
WORKING_ITEMS owns implementation/integration; TASK executes a bounded assignment
and does not delegate. The owner-facing agent retains Agent 0 responsibilities.

Agent 0 delegates a Type 1 role (HELPS_HUMANS or WORKING_ITEMS) to its own
standalone session or thread, never as a child within Agent 0's session or
thread. A Type 1 agent delegates to children within its own session or thread.
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
one INTERLOCUTOR session and continues it by message for the life of the Agent 0
session; a replacement starts only if that session is lost or the owner asks.
Agent 0 sends periodic activity summaries and consults it at least before every
PR merge, as `agents/AGENT_INTERLOCUTOR.md` describes. Its advice requires no
sign-off and carries no veto; Agent 0 keeps its decisions and the owner keeps
reserved choices. Its advice is not a required check. Sessions that do not
select it have no consultation step.
INTERLOCUTOR delegates nothing. If a matter is still unresolved after three
rounds of debate, Agent 0 explains its view and proceeds within its authority;
INTERLOCUTOR may raise pertinent consequences later. It may use a method only
where that method's roles list names it. Its standing assignment is to advise
that Agent 0 throughout the session; tools and permitted writes come from the
actual assignment and host authority.

Use `workflows/coordinated-knowledge-work/WORKFLOW.md` when coordinating related
contributions, the library's `workflows/index.json` to select other methods, and
`workflows/create-workflow/WORKFLOW.md` before creating/revising a reusable
workflow. Retain selected source identity and expose collisions instead of
silently switching sources. Roles grant no host permissions; model/effort
choices come from session steering. Give delegated work an input basis, scope,
read/write boundaries, integration/review route and reserved decisions.

The Library's own work locations, duties and Git grants do not transfer to
SWBPIPE. Shared-method consultation does not authorize changes to another
repository. Ordinary product build/test/CI uses this repository's maintained
tools and fixtures and must not require the Library checkout. Product-side agent inputs
remain source-bound local resources; consulting newer development methods does
not update those inputs or establish product adoption.

## Coordination between projects

The Chirality Library (`sgttomas/chirality-library`), Chirality Root
(`sgttomas/chirality`), App v4 (`sgttomas/chirality-app`) and SWBPIPE
(`sgttomas/SWBPIPE`) each have one top-level Agent 0, working in its own T3 Code
session. When work here needs another of these projects, such as a shared
interface, a Library change a consumer will adopt, a defect found in the other
project, or a shared release gate, this project's Agent 0 messages that
project's Agent 0 directly in T3 Code. Read the recipient's session first to confirm that it is
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
Records written before 2026-11-05 use "Root" for the former combined repository
that held the Library.

## Work and verification

Use local `execution/PKG-nn/DEL-nn-nn/ScopeOfWork.md`, `Work/` and
`deliverable.yaml`; reconcile changed implementation meaning and dependency
conditions in the same change. Use `python3 -m tools.deliverables --project .`
with `check`, `neighborhood`, `impact`, `touches` or `dag-diff`. Preserve unchanged
declarations and historical identities.

`Work/` holds the current account: findings with their reasons, working artifacts and outputs. Revise it in place rather than appending history; never rewrite fidelity copies, frozen snapshots or accepted evidence. How the work went (approval quotes, review rounds, retries, run IDs, check results) belongs in the PR description, which states what changed, why, what was checked and what remains open. Cite SHAs or hashes only where they pin something a result relies on. State a fact once and link to it.

- **What the repository keeps.** Produce the work and the minimum information
  needed to review, use or recover it. A record needs a concrete consumer,
  acceptance requirement or recovery need. Decisions edit the text they govern,
  not append registers. No routine receipts, notices, run records or handoff
  chains; never hand-maintain derived progress or lifecycle state or manufacture
  acceptance, lifecycle advancement or standing from old records. Preserve
  unrelated work and accepted historical evidence. Keep one replaceable recovery
  note only where continuity needs it. Roles and methods come from repository
  files, never host-side agent definitions or harness configuration. Session
  routes are ephemeral: find current sessions by listing T3 threads. Agents save
  no memories; durable knowledge lives in Chirality. Scratch holds evidence,
  outputs and logs, never instructions. Once work is integrated or abandoned,
  each contributor deletes its scratch, worktrees, branches and build output
  unless an open review, named consumer or recovery need still uses them, or the
  owner asks to keep them. Before removal, check for wanted uncommitted work;
  remove worktrees with `git worktree remove` so Git's records stay consistent.

Lead owner updates with the result
or decision needed. Use plain prose with spaced identifiers in messages to
delegates.

Code that runs (implementation, tests, generators, harnesses) lives outside `execution/`, in the project's code locations, and deliverables link to it; `Work/` may keep documentation-only examples that are never run.

Use focused checks for ordinary changes. Numerical meaning, persistence,
permissions and destructive operations require targeted regression checks and
independent scrutiny, under the [proportionate-review delegation](#standing-delegation-to-help_human).
Never change a result merely to silence a failure. Reuse applicable evidence;
state unavailable checks and unresolved inputs explicitly.

Commitments, risk acceptance, protected baselines, holds, issuance and release
remain subject to the owner's authority and explicit local delegations.
Repository settings, branch protections and required-check changes must be
presented to the owner before changing them.

When an owner instruction overrides a standing rule, note the exception in the
PR and ask once whether the standing rule should change.

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

## Standing Git grant

For `sgttomas/SWBPIPE` only, agents may commit, push, open, update and merge PRs
within authorized work after the verification above and required checks pass on
the exact head. The Agent 0 of the session that opened a PR, directly or
through its delegates, merges it after the review its change requires and with
required checks green on the exact head, having first triaged any open issue
labelled `nightly` (the PR that repairs one may merge). Delegated agents report a PR ready to
merge and wait for Agent 0 to merge it. Explicit holds and reserved decisions
apply. This grant excludes protection bypasses, history rewrites, force pushes,
permission changes and releases. Use [Git conventions](#git-conventions) below.

## Git conventions

Apply these project conventions while using ordinary Git judgment and the authorization already present in the session. Authorized briefs and these repository conventions govern Git work here.

- Treat `main` as SWBPIPE's integration branch. Name task branches with a prefix that identifies the authoring agent, such as `codex/` or `claude/`, unless the user or an applicable owning-loop rule specifies otherwise. Integrate a PR with a merge commit (no fast-forward) unless the PR or its owning loop chooses otherwise.
- Use a separate worktree when the user requests isolation or when concurrent sessions make isolation proportionate, such as overlapping writes to governance surfaces, risky refactors, or tool interference. Do not require a worktree for every change. High-risk overlap paths here include `AGENTS.md`, `loop/`, governance documents under `docs/`, accepted snapshots, generated derivative packages, and shared control roots such as `execution/_Coordination/`; warn when two lanes overlap on them.
- Close authorized work through a PR under [SWBPIPE AGENTS.md's standing Git grant](#standing-git-grant). No separate commit, push, PR or merge approval is needed when its conditions hold. Use the configured authenticated identity and retain truthful authorship; agent review is not personal owner review.
- Verify in proportion to consequence and confirm required checks pass on the exact head. Obtain independent scrutiny where the grant requires it. Clear blocking findings and reassess affected checks after candidate changes; a passing rerun does not establish that a known defect was repaired. Verify the source HEAD when merging.
- Git integration, human acceptance and product release remain distinct. Explicit holds and reserved decisions apply. The grant is limited to `sgttomas/SWBPIPE`; it does not authorize protection bypasses, history rewrites, force pushes, permission changes or releases.
- Update affected live consumers and references in the change itself.

For implementation and closeout in this repository:

- Separate maintained source and test fixtures from run evidence. Permanent tests and public packaging should not depend on a dated agent-run directory. Keep useful regression cases in the maintained test suite; preserve one-time migration evidence as history rather than requiring the migration to be re-proved forever.
- Keep working summaries revisable until a real review or acceptance boundary. Format authored documents and generate derived files before binding their bytes for review. Preserve raw tool output verbatim when needed; do not make it an editable, style-normalized source artifact. A later correction needs evidence for the affected change, not another full copy of the previous packet.
- Cosmetic whitespace is not a commit or merge blocker. Use formatting diagnostics only when helpful; preserve intentional Markdown hard breaks and raw evidence rather than creating repair cycles for their appearance.
- Judge a check by the failure it prevents. Keep behavioral, permission, containment, compatibility, and reproducibility checks. Remove obsolete migration assertions, incidental wording/order assertions, and duplicate evidence checks when they protect no current requirement. When a governing instrument requires such a check, propose the corresponding amendment rather than silently bypassing it or adding another workaround.
- For changes crossing the engine, the desktop app and the embedded agent's provider adapters, validate the actual connecting path early. Separate component passes do not establish integration. Check the deliverable from committed or explicitly selected files, including its maintained fixtures and resources; untracked local evidence must not supply a hidden dependency.
- Match preflight to the actual candidate and affected CI entrypoints. Scope candidate checks to files being submitted, and run required range checks against the complete PR diff. Re-run affected checks after repairs; repeat broader suites only when the change or an unresolved concern warrants it.

Do not invent approval-token syntax, add fresh approval gates, or infer authority beyond the user's request and the applicable owning-loop rules.

## Local boundaries

The owner selected this repository as the public home replacing RT-V8's sanitized
export. Verified migration history may be pushed before scan signature (owner,
2026-10-29: "Push now, sign before v1.0"). Before v1.0 release, the owner must
complete and sign the D-20/DEC-058 scan, including the full private-term scan and
finding dispositions. Unsigned rehearsal results, CI passes and merges supply
no signature, clearance or release authority. The release target's reserved
wording is updated by SWBPIPE Agent 0 under the owner's delegation.

Keep credentials and private/proprietary engineering inputs out of Git. Preserve
the local IP/data and professional boundaries. No supplier acquisition, sign-in,
keychain use, signing or live qualification follows from a repository migration.

- **F-PIP-3 (acceptance and issuance):** acceptance, issuance and release of any
  deliverable, package or product baseline are the owner's acts. Agents do not
  record, imply or derive acceptance, issuance or lifecycle standing from completed
  work, checks, reviews or merges. A baseline the owner has issued or protected
  changes only through a change the owner approves.
- SWBPIPE does not proclaim code compliance. Benchmark agreement with CAEPIPE
  is published as `docs/RELEASE_TARGET_v1.0.md` §3 provides; other evaluation
  stays private (§1 records the owner's rulings).
