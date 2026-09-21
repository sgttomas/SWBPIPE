# Reconciliation Profile — SWBPIPE

> **Status: PROPOSED with D-73.** Takes effect only when the owner rules D-73
> and the ruling pins this file by SHA-256. It is this project's adoption
> record for the root `reconciliation` workflow
> (`workflows/reconciliation/`) and the shared kernel
> (`docs/DELIVERABLE_CONCORDANCE_METHOD.md` at the repository root), as
> kernel §7 expects of each adopting project. It holds project parameters,
> the project divergence layers and the fences. It selects no work, activates
> nothing and grants no authority. A later revision governs a run only when
> that run's activation ruling pins it.

## 1. Relationship to the shared method

The workflow and kernel govern the method. This profile specializes them for
Piping and may extend them; it never weakens a kernel evidence distinction or
invariant. Where this profile and the pinned workflow genuinely disagree, the
conflict is recorded as `AUTHORITY_CONFLICT` and brought to the owner; no agent
resolves it by precedence.

Historical project reconciliation plans and earlier run folders are
provenance. They are not method for a run under this profile unless that
run's own ruling adopts a named element.

## 2. Authority basis a run binds

A run records, at its frozen source state:

- the product requirements (`docs/PRD.md`) and the current scope-change
  pointer (`docs/_ScopeChange/_LATEST.md` in the project and
  `execution/_ScopeChange/_LATEST.md`);
- `docs/CONTRACT.md`, `docs/DIRECTIVE.md`, `docs/SPEC.md`, `docs/TYPES.md`,
  `docs/IP_AND_DATA_BOUNDARY.md`, `docs/PROFESSIONAL_BOUNDARY.md`,
  `docs/claims_registry.md`;
- the decomposition `execution/_Decomposition/SOFTWARE_DECOMP.md` and its §12
  decision log;
- the decision register `execution/_Coordination/_DECISIONS/_REGISTER.md` and
  its ruling records;
- the dependency graph named by `execution/_DAG/_LATEST.md`;
- the deliverable folders under `execution/PKG-*/1_Working/DEL-*/`.

## 3. Corpus representation

The corpus mixes three deliverable representations. A run's claim extractor
handles each deterministically, and its R0 calibration samples each:

| Representation | Marker | Claim grain |
|---|---|---|
| Scope of Work | `ScopeOfWork.md`, `schema: chirality-deliverable-sow/v1` | `CLM-NNN` blocks, with numbered `REQ`/`AC` items inside them |
| Architecture basis (PKG-00) | `ArchitectureBasis.md`, `chirality-architecture-basis/v1` | `REQ` and `AB` items |
| Bespoke contract | e.g. DEL-07-09's contract documents and coverage tables | table rows and numbered contract items |

`CLM` identifiers are unique only within a deliverable. Run claim keys are
therefore qualified by deliverable identifier and are issued by the run's
extractor, never minted by workers.

`_STATUS.md`, `MEMORY.md`, `_CONTEXT.md` and `_run_records/**` are
declared-state and history surfaces. They are audited; they are not scope
authority.

## 4. Project divergence layers

These layers are kept explicit in every ledger and are never flattened into a
single disposition. Each row records which layers apply.

1. **Product claims boundary.** F-PIP-2 and `docs/claims_registry.md` govern
   what the product and its documents may say. A ledger row never asserts, and
   a disposition never implies, anything the registry forbids. Standard claim
   fence applies (F-PIP-2; claims taxonomy per DEC-081).
2. **Engineering validation and provenance.** Verification (tests pass) and
   validation (benchmark, witness, oracle or vetted-source basis) are separate
   columns. A unit test is never promoted to validation. Engineering formula
   claims rest only on the maintainer's vetted sources; unreviewed extracted
   equation artifacts from the external piping-design corpus are never
   evidence for them (DEC-043). Source reliability is recorded per row.
3. **IP and data boundary.** F-PIP-1, `docs/IP_AND_DATA_BOUNDARY.md` and the
   contract's data and IP clauses. Run artifacts never quote protected
   standards content, vendor catalogue data or private user data; they cite
   locations.
4. **Accepted baselines and protected criteria.** These baselines and their
   change paths are preserved. How a ledger records them per row (for
   example a baseline-class column) is a candidate convention for a run's
   R0, not in effect until ruled there:
   - `ISSUED` deliverables (currently DEL-01-01): read-only in discovery;
     any change routes through the governed scope-change process;
   - frozen contracts, including the ruled hash and result-semantics
     contracts and schema versions;
   - protected checks, tolerances, oracles and limits: never moved to obtain
     agreement; loosening is a governance event;
   - ruled acceptance criteria (for example D-68 and D-72) and explicit
     owner holds: recorded, never lifted by a run.
5. **Security and privacy.** Local-first operation, telemetry off by default
   and private-data handling (PKG-12, F-PIP-1) are contract-level
   obligations.
6. **Lifecycle.** Discovery records `LIFECYCLE_REASSESSMENT_REQUIRED` where
   evidence warrants and never applies a transition (F-PIP-3).

## 5. Authority tiers — candidate for R0

**Not in effect until a run's owner rules it at R0.** The candidate
convention: every non-aligned claim carries one tier, because the tier
decides what kind of act a divergence needs.

| Tier | Meaning | Typical resolution |
|---|---|---|
| `LOCAL_DESIGN` | The claim's authority is a deliverable-local design choice | The deliverable may catch up to the implementation by owner-ruled edit |
| `PROJECT_BASELINE` | The claim restates a ruled decision, accepted scope or baseline in §2 | Owner ruling; a structural change routes to scope-change |
| `INVARIANT` | The claim restates a contract, specification or boundary invariant, or a §4 layer 1–5 obligation | A divergence is a candidate implementation defect or a governance amendment, never a deliverable catch-up |

## 6. Evidence roots

| Root | Use | Rule |
|---|---|---|
| The Piping tree at the frozen source state | Deliverables, code, tests, schemas, fixtures, validation assets | Read in a detached scratch checkout outside the repository; never written |
| Root `tools/practitioner_harness/`, `tools/validation/`, root `docs/` governance | Registered checks and governance context | Read-only |
| Root `.github/workflows/piping-*.yml` and `.github/actions/setup-piping-e2e/` | Hosted check definitions | Read-only |
| `_DomainEngines/**` | Tier-0 engine surfaces | Read-only; owned by their own loops (F-PIP-4) |
| Merged pull requests, their descriptions and hosted CI run records | What was done, why, and what was checked | Read-only references, read through the GitHub CLI |
| App-dev packages Piping consumes under D-30 | Consumed runtime and harness contracts | Read-only |
| AgentRuns records, approved plans, design specifications and frames, owner-direction records | Recorded direction and rationale | Read-only references; see §7 |

Writes outside the repository are limited to a detached scratch checkout in
the session's scratch directory and the shared user caches (`~/.cargo`,
`~/.npm`, Playwright browsers) that a fresh install may populate. The
activation ruling of each run states that run's complete write and
read-only boundary.

Registered checks are those in `software-workflow.json`. A run executes them
once for its gate transcript in the scratch checkout, with caches and outputs
redirected outside the frozen tree, and workers cite that transcript instead
of running suites. Native desktop behaviour is cited from existing native
witnesses unless a run's ruling asks for new ones.

## 7. Direction sources

A run distinguishes governing authority (§2) from recorded direction that is
context: AgentRuns direction records, approved plans, design specifications,
handoffs and merged pull requests. Context explains a divergence and informs
clustering. Whether context changes a disposition, and how a merged pull
request counts as owner intent, are conventions the owner rules at the run's
R0; this profile does not predetermine them.

## 8. Routing of outcomes

| Outcome | Route |
|---|---|
| Deliverable text changes | Owner-ruled repair tranches, partitioned by owning deliverable |
| Retire, merge, split or create deliverables | Handoff package to WORKING_ITEMS (workflow: `scope-change`) |
| Implementation defects | Candidate bounded implementation briefs for separate owner authorization |
| Lifecycle acceptance | WORKING_ITEMS (workflow: `review`) |
| Source-control closeout | WORKING_ITEMS (workflow: `change`) |
| Engineering formulation, tolerance or validation questions | Owner as engineering authority |
| Findings about workflows or agent instructions | Observations routed to HELPS_HUMANS; never repaired in a run |
| Dependency graph changes | A separate owner-directed DAG rebuild |

## 9. Standing rules for a run

- Activation is a register decision merged to `main` before any dispatch.
- Discovery writes only inside the run's own record and evidence folders.
- The frozen source state is recorded; material change marks affected rows
  `STALE_INPUT` and they are rerun.
- Agent dispositions are never presented as owner rulings.
- Every wave passes deterministic structural validation and a fresh,
  evidence-only verifier; defective ledgers are rerun by a fresh worker.
- Only the run's Agent 0 commits. Independent fresh-context review covers the
  activation, R0, synthesis and decision, repair and closure pull requests.
- Closure is evidence coherence. It is not issuance, acceptance or release.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
