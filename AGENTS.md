# SWBPIPE repository instructions

This repository owns SWBPIPE. Enter through `loop/LOOP_INIT.md`; requirements,
product/data boundaries, adopted work localities and verification are local.
The conversation supplies the undertaking. Preserve commitments, numerical
references, source meaning and unrelated work; initialization alone starts none.

## Shared roles and methods

Use the selected Chirality library, `/Users/ryan/dev/chirality` on this workspace
or an owner-selected checkout of `https://github.com/sgttomas/chirality` elsewhere.
Resolve shared `agents/` and `workflows/` paths against that library. Tool-use
instructions are repository content kept beside the tool they instruct, not
harness skills. Load the active role's `agents/AGENT_<ROLE>.md` and other method bodies
only when needed. HELP_HUMAN retains purpose/alignment; HELPS_HUMANS designs;
WORKING_ITEMS owns implementation/integration; TASK executes a bounded assignment
and does not delegate. The owner-facing agent retains Agent 0 responsibilities.

Use `workflows/coordinated-knowledge-work/WORKFLOW.md` when coordinating related
contributions, the library's `workflows/index.json` to select other methods, and
`workflows/create-workflow/WORKFLOW.md` before creating/revising a reusable
workflow. Retain selected source identity and expose collisions instead of
silently switching sources. Roles grant no host permissions; model/effort
choices come from session steering. Give delegated work an input basis, scope,
read/write boundaries, integration/review route and reserved decisions.

Root-specific work locations, duties and Git grants do not transfer to SWBPIPE.
Shared-method consultation does not authorize changes to another repository.
Ordinary product build/test/CI uses this repository's maintained tools and
fixtures and must not require the Chirality checkout. Product-side agent inputs
remain source-bound local resources; consulting newer development methods does
not update those inputs or establish product adoption.

## Work and verification

Use local `execution/PKG-nn/DEL-nn-nn/ScopeOfWork.md`, `Design/` and
`deliverable.yaml`; reconcile changed implementation meaning and dependency
conditions in the same change. Use `python3 -m tools.deliverables --project .`
with `check`, `neighborhood`, `impact`, `touches` or `dag-diff`. Preserve unchanged
declarations and historical identities. Git records changes; do not manufacture
acceptance, lifecycle advancement or a standing backlog from old records.

Use focused checks for ordinary changes. Numerical meaning, persistence,
permissions and destructive operations require targeted regression checks and
independent scrutiny, with the proportionate-review rules in the local loop.
Never change a result merely to silence a failure. Reuse applicable evidence;
state unavailable checks and unresolved inputs explicitly.

Commitments, risk acceptance, protected baselines, holds, issuance and release
remain subject to the owner's authority and explicit local delegations.
Repository settings, branch protections and required-check changes must be
presented to the owner before changing them.

### Standing Git grant

For `sgttomas/SWBPIPE` only, agents may commit, push, open, update and merge PRs
within authorized work after the verification above and required checks pass on
the exact head. Explicit holds and reserved decisions apply. This grant excludes
protection bypasses, history rewrites, force pushes, permission changes and
releases. Use [Git conventions](#git-conventions) below.


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

## Git conventions

Prepare, review, or close out Git changes in the SWBPIPE repository using its branch, verification, merge, and authority conventions. Use only for SWBPIPE project work, not general Git tasks or other repositories.

Apply these project conventions while using ordinary Git judgment and the authorization already present in the session. They specialize the bundled, repository-agnostic `change` workflow for this repository; where both apply, these conventions supply its repository-specific defaults and authorization.

- Treat `main` as SWBPIPE's integration branch. Name task branches with a prefix that identifies the authoring agent, such as `codex/` or `claude/`, unless the user or an applicable owning-loop rule specifies otherwise. Integrate a PR with a merge commit (no fast-forward) unless the PR or its owning loop chooses otherwise.
- Preserve unrelated work and accepted historical evidence. For ordinary changes, use the Git diff/commit and one concise review or PR record identifying the checked revision, results, and remaining limits. Keep additional records only for a concrete consumer, acceptance requirement or recovery need; do not duplicate Git history in chains of source copies, hash manifests, and closeout reports.
- Use a separate worktree when the user requests isolation or when shared-monorepo risk makes isolation proportionate, such as overlapping writes to governance surfaces, risky refactors, or tool interference. Do not require a worktree for every change. High-risk overlap paths here include `agents/`, `workflows/`, `.agents/`, governance documents under `docs/`, accepted snapshots, generated derivative packages, and shared control roots such as `execution/_Coordination/`; warn when two lanes overlap on them.
- Close authorized work through a PR under [SWBPIPE AGENTS.md's standing Git grant](#standing-git-grant). No separate commit, push, PR or merge approval is needed when its conditions hold. Use the configured authenticated identity and retain truthful authorship; agent review is not personal owner review.
- Verify in proportion to consequence and confirm required checks pass on the exact head. Obtain independent scrutiny where the grant requires it. Clear blocking findings and reassess affected checks after candidate changes; a passing rerun does not establish that a known defect was repaired. Verify the source HEAD when merging.
- Git integration, human acceptance and product release remain distinct. Explicit holds and reserved decisions apply. The grant is limited to `sgttomas/SWBPIPE`; it does not authorize protection bypasses, history rewrites, force pushes, permission changes or releases.
- Update affected live consumers and references in the change itself. The PR description carries the change, rationale, checks and open matters; no downstream notice or receipt chain is required.
- Delete after merge. Once a PR merges, or its work is abandoned, its owner removes what the work created: worktrees (with `git worktree remove`, so Git's records stay consistent), local branches, build output such as Cargo target directories, and scratch. Reviewers and TASKs remove their own when they return. Before removing a worktree, check it holds no uncommitted work that is still wanted. Keep only what an open review, a named consumer or a recovery need still uses, and anything the owner has asked to keep. Build output from parallel lanes fills a disk within days, so this step is not optional.

For implementation and closeout in this repository:

- Separate maintained source and test fixtures from run evidence. Permanent tests and public packaging should not depend on a dated agent-run directory. Keep useful regression cases in the maintained test suite; preserve one-time migration evidence as history rather than requiring the migration to be re-proved forever.
- Keep working summaries revisable until a real review or acceptance boundary. Format authored documents and generate derived files before binding their bytes for review. Preserve raw tool output verbatim when needed; do not make it an editable, style-normalized source artifact. A later correction needs evidence for the affected change, not another full copy of the previous packet.
- Cosmetic whitespace is not a commit or merge blocker. Use formatting diagnostics only when helpful; preserve intentional Markdown hard breaks and raw evidence rather than creating repair cycles for their appearance.
- Judge a check by the failure it prevents. Keep behavioral, permission, containment, compatibility, and reproducibility checks. Remove obsolete migration assertions, incidental wording/order assertions, and duplicate evidence checks when they protect no current requirement. When a governing instrument requires such a check, propose the corresponding amendment rather than silently bypassing it or adding another workaround.
- For changes crossing App, Runtime, or provider adapters, validate the actual connecting path early. Separate component passes do not establish integration. Check the deliverable from committed or explicitly selected files, including its maintained fixtures and resources; untracked local evidence must not supply a hidden dependency.
- Match preflight to the actual candidate and affected CI entrypoints. Scope candidate checks to files being submitted, and run required range checks against the complete PR diff. Re-run affected checks after repairs; repeat broader suites only when the change or an unresolved concern warrants it.

Do not invent approval-token syntax, add fresh approval gates, or infer authority beyond the user's request and the applicable owning-loop rules.
