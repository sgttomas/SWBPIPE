# T3 handoff to the next ROOT — 2026-10-03

**Gracefully stopped for the owner-requested handoff. All TASKs are complete,
reader source is frozen, and no solver/compiler/test job remains. The existing
memory guard stays running. Public F2a and all reader eligibility remain held.**

The owner asked this ROOT to reach a graceful conclusion and prepare a handoff
to another agent. No further slice, public activation or PR was started after
that request. This remains HELP_HUMAN, Type 0, Agent 0 work under the owner's
existing directions and delegated technical authority.

## Start here

Read Root AGENTS.md and agents/AGENT_HELP_HUMAN.md, P/AGENTS.md, this handoff,
[ROOT_CURRENT](RESUME_2026-09-30/ROOT_CURRENT.md), and the latest append-only
[ROOT rulings](ROOT_RULINGS_V1.md). Use the
[publication map](RESUME_2026-09-30/FIRST_PUBLICATION_PATH.md) and
[glossary](RESUME_2026-09-30/GLOSSARY.md) for navigation. Do not reread every
historical packet or treat an older draft as a selected contract.

P = projects/chirality-piping; T3 = this directory; R = RESUME_2026-09-30.
WT on the M5 host is
/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-control-layer-8a41be/.claude/t3.
NUM=WT/numerics, CODE=WT/f2a, READER=WT/f2a-readers. Use explicit worktree cwd:
the App's 92ea checkout is not this integration tree. Check the actual host first.
This host is the M5 Max with 128 GiB; the existing guard is WT/guard/memguard.sh.
No new host tooling/guard is needed. VENV is the parent control checkout's
projects/chirality-piping/.venv. All heavy work stays on this M5.

## Accepted progress and remaining numerical milestone

- A1 reached main in PR1070 at `3a0251874d`; C17 established the real witness,
  and the independently rederived correction certifies the actually published
  value. K6c reached main in PR1071 at `49034a940f`, with its recorded limits.
  Historic KF3 published-value equality remains unclaimed under the owner's
  accepted comparison route. The owner caught/stopped PR1066's tooling drift;
  that draft was closed without merging. Its selective inputs are historical.
- The prepared private producer at `922db9dce3` passes the named case in both
  modes and was accepted after RV68 review, local merge `458603880a`.
- Caller/census source `24af17c470` was accepted after RV72, local merge
  `a9c2256076`; every production profile/M/permit remains absent.
- Typed private evidence source `fc23cff95f` was accepted after complete RV74
  review and prior-cause repair, local merge `b56b905251`. Original errors,
  conversion/helper/evaluator/lane/work prefixes and private terminal ownership
  survive. RV74 REVIEW owns the checks/limits. The public transaction is absent.
- Corrected C3/F1 and wire completions 06+07+08 are selected (RV69; ROOT
  `e0055868da`). They are prospective contracts, not qualified public execution.

**First public milestone is still open:** RF-SKEW-T-CANT-OFF-122-r1e-04 through
the actual captured facade in both solver modes, under M03-INTEGRITY-MP-v2,
matching its independent reference and preserving refusal/coexistence controls.
The typed entry keeps its ordinary route and required both-entry checks.
The passing private candidate and synthetic reader fixtures do not satisfy this.

Complete F2a still needs the producer receipt/freeze transaction, resource and
caller qualification, all three readers/carriers, complete invocation, preparation-
only/mixed combinations and promised exact routes, native Current and PR gates.
Then S-I, F2b per domain and F3 remain. No scope omission is authorized.

## What stopped at the usage limit, and what this recovery did

R/RESUME_2026-10-03_1904/STATE.json preserves the exact recovered reader WIP.
I51/I52/I55/I56/RV73 stopped with usage-limit errors; none was live on resumption.
Old deadlines and partial/late checkpoints are retained. New recovery TASKs
I57/I58/I59/I60 and fresh reviewers RV75/RV76 have their own recorded receipts;
the old clocks were not silently extended. Main remained `381be775ae`.

I58 owns shared schema/corpus/Python, I59 Rust, I60 TypeScript, on disjoint paths.
The original single-author three-reader assignment was too large; ROOT recorded
that allocation error. No reader source has independent full acceptance.
The synthetic corpus initially had zero native hashes and internally inconsistent
native record fields. Corrected frozen snapshots preserve the failed drafts;
source-byte reconstruction is a consistency check, not solver execution.

Final reader source/record identities, verified archive and exact write set are
in [READER_STATE.json](RESUME_2026-09-30/HANDOFF_2026-10-03/READER_STATE.json).
All sixteen authorized source paths were verified against the authors’ freezes;
thirteen differ from the reader baseline. The archive was read back and checked.
The reader source remains uncommitted WIP deliberately, with recoverable copies;
no local component fan-in or main acceptance is credited. Do not stage the
ROOT-owned node_modules symlink, generated assets or unrelated historical links.

## Reader checkpoint and exact unfinished work

All three author packets are preserved verbatim in NUM and still present in
READER. Their RETURNs are the shortest reliable restart points:

| Owner and return | Final bounded checks | Final principal source hash |
|---|---|---|
| [I58 shared/Python](RESUME_2026-09-30/I58/python_shared_01/RETURN.md) | 53 Python/schema tests | `a9819e6a7a` |
| [I59 Rust](RESUME_2026-09-30/I59/rust_reader_01/RETURN.md) | 8 Rust tests, including complete controls, mutations and rational helpers | `b43595136e` |
| [I60 TypeScript](RESUME_2026-09-30/I60/typescript_reader_01/RETURN.md) | 50 Vitest tests and desktop typecheck | `f2e2d1868c` |

These are suite-test counts, not comparable numbers of engineering cases.
All used shared snapshot 03, corpus `67d5cbcc00`, with synthetic sparse/dense
records. The source/stiffness hashes are computed from their maps; ledger/state
and execution statements remain labelled synthetic. The fixtures are not the
actual named public witness. Failed accounting/typecheck attempts are preserved.

The three readers have not received complete independent source review or a
joint acceptance decision. Python’s public entry is disabled and its draft path
was tested; Rust/TypeScript retain completeness holds. Isolated passing runs
therefore do not establish equivalent qualified public entry behavior.
Besides implementing selected coverage, the returns identify unfinished native
schedule/cache/reason/reference and C3 failure-prefix audits; missing complete
unavailable/old-Err/new-Ready/K-only/maxima-abandon controls; actual p512 floor
rederivation; final row/native coverage; material/units/alpha refusals; literal
G7 mutations and cross-language first-failure parity. Do not toggle a completeness
flag merely because the current selected-shaped fixtures pass.

Runtime dependencies are available on this host: the parent P/.venv, ROOT-owned
READER/P/node_modules link, and source-bound CLI/WASM outputs from I52’s earlier
builds. The reader returns/manifests pin the exact paths and artifact hashes.
Set both OPENPIPESTRESS_CHECKED_JSON_BIN and OPENPIPESTRESS_UNITS_BIN for pytest,
otherwise its conftest can launch hidden Cargo. Use the existing test setup;
no mocked authorities, dependency install, new runner or guard. Existing caches
are WT/targets/i52-readers/{result_export,canonical_json,units,...}.

## Design and memory decisions at the stop

- **Ordinary source terms accepted at their partial scope:** ROOT ruling
  `8a47c763c2` adopts I54 original02 plus correction03 after fresh RV75’s
  complete combined review. [RV75 REVIEW](RESUME_2026-09-30/REVIEW_RV75/ordinary_bound_01/REVIEW.md)
  owns the source/arithmetic scope. This is fresh verification, not a claim that
  interrupted RV73 finished its backcheck. No complete profile/M/fit follows.
- **Coverage design selected, not implemented:** ROOT ruling `1342d384d5`
  selects [I57 ADDENDUM](RESUME_2026-09-30/I57/summary_coverage_01/ADDENDUM.md)
  after fresh [RV76 REVIEW](RESUME_2026-09-30/REVIEW_RV76/summary_coverage_01/REVIEW.md).
  C3 ProofTrace carries one nullable complete body roster of body, stop[4] and
  has_data from the actual proof owner. Estimate/charge are derived; native
  p512 force/moment charge equals stop. Exact owner/Run/source, null/prefix,
  gate/hash and public-consistency checks are all mandatory. The fallible PP
  adapter copy and final binary64 rows are not substitutes. Existing attestation
  limits remain: hashes do not authenticate a fully rewritten whole statement.
  No owner-held meaning change was found within that existing boundary.

No part of the newly selected coverage design was implemented in this run.

The ordinary-memory programme has concrete source formulas and corrected owners;
it does not establish a complete DirectPp total or allowance. Genuine residuals:
final consumer/compiler/features/allocator/private-layout association; complete
nested typed capacities or construction bounds; H_formation128; report/row/
diagnostic/error text grammar; generic deep legacy-exact ownership; stack; and
the new trace/publication/caller composition. Native context/tail/backing remains
unproved; that bounded vendor investigation ended at its limit. Do not restart
host-tool development or treat unknown terms as zero.

## Resume order

1. Verify the frozen trees/source archives and final manifests, fetch main/gh,
   inspect worktree status and actual processes. Preserve all raw evidence.
2. Apply only the independently selected coverage design to the owning typed
   trace/producer/schema/readers/corpus under a fresh bounded grant. The selected basis is now closed at design level. Keep actual producer attestations
   distinct from public rederivation; never infer q_P nonzero/data from final rows.
3. Complete the reader audit and failure-prefix controls, then fresh independent
   complete-source review and joint parity on the exact same frozen inputs.
   Synthetic controls cannot establish actual publication/custody.
4. Close the remaining actual memory/profile/build terms and select M only with
   the required evidence, then finish the admitted producer receipt transaction.
   Do not enable a permit on symbolic or partially priced terms.
5. Demonstrate the named real public milestone, complete wider F2a obligations,
   carrier/standing/native qualification, then the final product gates/PR.

Do not commission a new serializer, generic exact-sum engine, guard, probe framework
or availability exception merely to bypass a known boundary. Report concrete
tool/contract obstacles; escalate actual public meaning changes with reviewed
options. Existing source-returned counts, stable source/capacity laws and stock
tools remain the preferred basis.

## Git and evidence rules that continue

NUM's codex/piping-numerical-integrity-20260926 is the F2a code integration branch.
CODE's codex/piping-f2a-work-exactness-20261002 is the accepted producer component.
READER's codex/piping-f2a-readers-20261003 contains unaccepted reader WIP.
No F2a PR is open. NUM records and this handoff are pushed on the integration branch, not main.
A records PR must be cut from current main with only execution
paths and an empty non-execution diff. The final F2a PR is also cut compactly from
main with reviewed maintained-source equality, concise evidence and external
bulk manifests; do not bring the whole accumulated records branch into it.

TASKs have no Git/index writes. ROOT reads/verifies full core diffs, scope and
hashes before source acceptance/commit; ROOT does all merges, never rebases or
force-pushes. Before every main merge: fresh independent complete review and
same-reviewer repair confirmation; full-SHA hosted CI; exact-final-head Mac
DEC-025; GEN-8; product T9 and both-entry gates. Check main has not moved immediately
before match-head merge. Record merge, ruling and graph in the same pass.
No new main gate ran or was credited in this recovery.

Bulk remains under WT/scratch with hashes/size/locations. Raw AUD-T3-04 suite logs
and large KF2 gate inputs remain preserved; do not prune. Readable summaries and
decisive evidence are committed; a preserved source archive is not accepted code.
Rulings stay append-only and sealed evidence stays unchanged.

Pending owner decisions remain dense/lane ceilings, PHYS-R4 named refusal and
availability, observation framing, KF3 lambda split and KF2 dense screen, plus
any actual new public-contract decision independently demonstrated. A handoff
creates no new approval ritual; the successor follows the owner's start direction.

Next unused assignment IDs after this run are I61 and RV77, subject to a fresh
directory/brief check. Completed source/design acceptance, unaccepted drafts and
open reviews must remain separately identified.

