---
doc_id: CB-2026-08-11-DEL1005-MECHANICS-RUNNER-BINDING-001
doc_kind: coordination.candidate_brief
status: CANDIDATE_AWAITING_OWNER_ADOPTION
prepared: 2026-08-11
primary_package_id: PKG-10
primary_deliverable_id: DEL-10-05
prerequisite_package_id: PKG-09
prerequisite_deliverable_id: DEL-09-01
accepted_base: f1e311fb7ab1c2a0800b1d32c59445368428dee9
execution_posture: SERIALIZED_CROSS_PACKAGE_AGENT0_FAN_IN
---

# CANDIDATE Brief — DEL-10-05 Mechanics Case-Runner Binding Completion

**Status:** `CANDIDATE — AWAITING OWNER ADOPTION — NO EXECUTION AUTHORITY`

**Prepared for:** HELP_HUMAN (Agent 0)

**Proposed branch:**
`codex/piping-dec025-case-runner-binding-20260811`

**Pinned base:**
`f1e311fb7ab1c2a0800b1d32c59445368428dee9`

This brief is a non-authoritative execution proposal. It does not authorize
source, test, evidence, state, receipt, Git, lifecycle, publication, release,
or reliance changes. Adoption is the owner's act. If adopted, HELP_HUMAN
supervises the serialized cross-package graph below; no single WORKING_ITEMS
instance manages more than one package.

## 1. Path and base resolution

Every executor resolves the active checkout rather than using a recorded
machine path:

```text
REPO_ROOT=$(git rev-parse --show-toplevel)
WORKING_ROOT=${REPO_ROOT}/projects/chirality-piping
```

Before any execution write, CHANGE must verify all of the following:

1. `HEAD` and the branch point are exactly
   `f1e311fb7ab1c2a0800b1d32c59445368428dee9`.
2. The branch is exactly
   `codex/piping-dec025-case-runner-binding-20260811`.
3. The index and worktree are clean, including zero ignored drift within
   `WORKING_ROOT`.
4. The Piping tree at the branch point is byte-identical to the accepted base.
5. The accepted inputs and identities in section 3 remain exact.

Any failed gate stops execution before source writes. Fetching, rebasing,
resetting, cleaning, or silently changing the base is not a repair mechanism.

## 2. Product position and governing boundaries

The product-delivery direction and design-tool boundary are already adopted
together without amendment by D-64. This tranche applies that adopted basis:
OpenPipeStress is a local-first limited-purpose design tool, with robust
user-facing and semantically equivalent agent-facing execution surfaces,
while engineering judgment and validation authority remain external.

The tranche completes a deterministic agent-facing benchmark execution seam.
It does not make the application fully agentic, perform engineering judgment,
or convert regression evidence into professional validation.

The following are fixed and are not reopened:

- DEC-046 Option C-B is settled. Its record is consumed byte-identically and
  applies only to the nonlinear active-set convergence axis. It does not
  supply mechanics result-comparison values and does not govern this binding's
  mechanics comparisons.
- No public result-comparison number or policy is selected. The open owner act
  remains open.
- Manual case pages and fixtures are governed evidence. They are never edited
  to fit the runner.
- Every validation-manual page remains `DRAFT_EVIDENCE`; no page promotion is
  authorized.
- Task Management rows and archives are outside this tranche.
- The MR-A campaign is outside this graph. It may proceed only as a separately
  governed lane whose complete write scope is disjoint; this brief neither
  launches it nor depends on it.
- Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

## 3. Accepted evidence and exact reconnaissance

The accepted execution base is the post-PR-550 merge commit named above. The
following inputs are frozen at that base:

| Input | Git blob | SHA-256 | Role |
|---|---|---|---|
| `validation/evidence/comparison_measurement/DEL0904_VD_20260811/CURRENT_25_FIXTURE_RUNNER_OUTPUT.json` | `51ee331ef771dbffb7034796fbe66b3e2c48309d` | `e41f8545cf6fc0603cb41a0bdc08d90726f59ce57e13ad6809036589db3ded8f` | Exact current whole-suite reconnaissance |
| `validation/benchmarks/mechanics/src/lib.rs` | `eb65e53075110995a4ddcd93b4181b15392f91d5` | `8f015dfeeb1fd670065f467335419ab0da8b0c9a4e6d6b3fb565ceea2d6f6a26` | Suite-owned fixture identity, expected values, solver access, and predicates |
| `core/runner/headless/src/benchmark_binding.rs` | `75fa69df616dd803ebd8409683d2468536c4b6ac` | `1dadf7636f99b9a1931e76daf28bfcf3c49e502cecef9b51c4fd351711050d39` | Current 11-case runner observation binding and fail-closed catch-all |
| `validation/benchmarks/nonlinear/release_convergence_policy.dec046.c-b.json` | `468d6dd4a85525b64989ff520a5f4ff10e7c6e6f` | `1829a2a6a608dbbdd479248133f2c2a05406c0eabe7a03be57b998735fbe78b6` | Settled nonlinear-only C-B policy |

The exact current reconnaissance is:

- 25 mechanics cases requested;
- 11 executed and matched;
- 0 executed and mismatched;
- 14 blocked;
- 206 recorded values total;
- 91 observed values on the original executing 11 cases;
- 115 unobserved values on the blocked 14 cases.

The blocked set and required observation counts are:

| Case ID | Required observed values |
|---|---:|
| `MECH-CANTILEVER-TIP-FORCE` | 2 |
| `MECH-STRAIGHT-PIPE-WEIGHT-RECOVERY` | 2 |
| `MECH-TP-DEC092-TEMPERATURE-INDEXED-SHEAR-MODULUS-TORSION` | 6 |
| `MECH-SUPPORT-BOUNDARY-MIXED` | 3 |
| `MECH-PRIMITIVE-LOAD-PREP` | 3 |
| `MECH-FIXED-FIXED-THERMAL-AXIAL` | 2 |
| `MECH-IMPOSED-DISPLACEMENT-SPRING` | 1 |
| `MECH-INCLINED-MEMBER-TRANSFORM` | 2 |
| `MECH-EXPANSION-LOOP-CURVED-BEND-THERMAL` | 21 |
| `MECH-CURVED-BEND-DISTRIBUTED-FIXED-END` | 42 |
| `MECH-CURVED-BEND-PRESSURE-THRUST-ARC` | 12 |
| `MECH-TP-PMM-P3-OCCLOADGEN-EQUIVALENT-STATIC` | 6 |
| `MECH-TP-PMM-P3-SUBSPAN-WIND-EXPOSURE` | 8 |
| `MECH-CONSTANT-EFFORT-SUPPORT-APPLIED-LOAD` | 5 |
| **Total** | **115** |

The earlier 13-case/109-value census remains truthful for its immutable July
20 evidence. Current scope includes the later six-value DEC-092 case, so the
execution acceptance target is the complete current 14-case/115-value gap.

## 4. Objective and acceptance contract

Complete the mechanics case-runner observation binding without modifying
fixture truth, by first exposing one suite-owned additive observation API and
then consuming that API from DEL-10-05.

The tranche is accepted only when all predicates below hold together:

1. A whole-suite mechanics run requests exactly 25 cases and reports exactly
   25 `executed_and_matched`, 0 `executed_and_mismatched`, and 0 `blocked`.
2. Every one of the 206 recorded values has one finite, correctly named,
   unit-consistent observed counterpart and a populated delta/comparison
   result: `206/206` complete.
3. The 14 formerly blocked cases supply all 115 required observations with no
   missing or extra observation name.
4. The original 11 cases preserve all 91 existing observations exactly:
   same case IDs, names, units, dimensions, observed numeric representations,
   deltas, comparison results, case outcomes, and ordering as the frozen
   reconnaissance. No existing observation is recomputed through a different
   path merely for uniformity.
5. The mechanics suite remains the single owner of fixture identity, expected
   values, observation production, and encoded predicates. The runner does
   not re-encode formulas, expected values, private fixture internals, or a
   new tolerance.
6. Unknown IDs, suite execution failures, non-finite observations, incomplete
   observation sets, duplicate names, and observation/expected-name mismatch
   remain structured fail-closed outcomes with no silent skip.
7. The C-B JSON named in section 3 remains byte-identical, and no mechanics
   code path reads it as a comparison policy. Existing nonlinear behavior and
   tests remain unchanged.
8. No case page, fixture JSON, hand calculation, expected-value record,
   benchmark README, public comparison policy, manual page state, Task
   Management row, GUI, export/CAEPIPE surface, or lifecycle state is changed.
9. All focused, package, repository, evidence-sweep, containment, and closeout
   gates in section 8 pass from a clean commit.

## 5. Serialized cross-package execution graph

HELP_HUMAN owns the graph and validates each fan-in. Each WORKING_ITEMS child
manages exactly one package. The graph posture is serialized because PKG-10
consumes the exact accepted PKG-09 API result.

```text
N0 CHANGE: branch/base/setup verification
  -> N1 WORKING_ITEMS(PKG-09 / DEL-09-01)
       -> A2-09 author: one-file suite-owned observation API + in-file tests
       -> A2-09 verifier: fresh, read-only independent verification
       -> N1 fan-in: accept exact lib.rs hash or HOLD
  -> N2 WORKING_ITEMS(PKG-10 / DEL-10-05), only after N1 acceptance
       -> A2-10 author: runner consumption + runner-local tests
       -> N2 fan-in: accept exact runner delta or HOLD
  -> N3 fresh independent cross-package verifier
  -> N4 integration/state/evidence closeout
  -> CHANGE: exact delta validation, local commit, publication-token stop
```

No node may start from an unaccepted predecessor. N2 receives N1's accepted
commit/tree identity, exact `lib.rs` hash, API contract, test result, and
structured return as sealed dependencies. A defect returns to a fresh bounded
author attempt; the verifier never repairs author output.

## 6. Sealed child contracts

Every managed child records `RequestedBy`, `RunID`, `ParentInstanceID`,
`ChildInstanceID`, package/deliverable, exact accepted basis, dependencies,
exclusions, declared reads, allowed tools, allowed writes, expected outputs,
acceptance checks, escalation conditions, start/finish status, and a durable
structured return under the run root selected by HELP_HUMAN.

### N1 — WORKING_ITEMS for PKG-09 / DEL-09-01

**Objective:** produce and accept a suite-owned, additive, value-addressable
observation API for every mechanics fixture.

**A2-09 author write scope:** exactly one production file:

- `validation/benchmarks/mechanics/src/lib.rs`

All suite-owned tests required by this node must live in that file's existing
test module. No second mechanics file is writable. The API must return named
observations derived through the suite's existing public-original solvers and
fixture logic. It must preserve existing fixture inventory, expected values,
comparison predicate, provenance, and all current tests.

**Declared reads:** section 3 inputs; mechanics `Cargo.toml`; relevant open
solver/kernel APIs; DEL-09-01 `ScopeOfWork.md`, `_STATUS.md`, `MEMORY.md`,
`_CONTEXT.md`, dependencies, and recent run records; applicable project
instructions and software workflow profile.

**A2-09 verifier:** fresh and read-only over source/project truth. It writes
only its launch/status/return artifacts beneath the managed run root. It
recomputes the 14-case/115-value coverage, tests exact names and completeness,
checks the 91-observation preservation projection, and rejects any fixture,
expected-value, predicate, tolerance, or C-B change.

**N1 return:** accepted `lib.rs` hash; API signature/semantics; exact test
commands/results; `14/14`, `115/115`, and original-91 preservation evidence;
write-containment proof; blockers and rerun triggers. N1 must return `HOLD`
instead of widening scope.

### N2 — WORKING_ITEMS for PKG-10 / DEL-10-05

**Dependency:** accepted N1 return and exact accepted `lib.rs` bytes.

**Objective:** replace the mechanics catch-all for the current 14 cases by
thin consumption of the accepted suite-owned observation API while preserving
the existing 11-case output exactly.

**A2-10 author write scope:** only:

- `core/runner/headless/src/benchmark_binding.rs`
- `core/runner/headless/src/bin/openpipestress-runner.rs` only if a runner-
  local test or thin invocation adjustment is strictly necessary; otherwise
  it remains read-only
- `tests/test_headless_runner_contract.py` only for runner-contract assertions
  that cannot live in existing Rust test modules; otherwise it remains
  read-only

No runner schema change is anticipated or authorized. No mechanics file is
writable in N2. The runner must continue to obtain inventory and expected
values from the mechanics suite and use the existing mechanics comparison
predicate. The accepted suite observation API supplies observed values only.

**Declared reads:** N1 return and accepted source; section 3 inputs;
`core/runner/headless/Cargo.toml`, runner library/binary/test surfaces;
existing witness inputs/outputs; DEL-10-05 `ScopeOfWork.md`, `_STATUS.md`,
`MEMORY.md`, `_CONTEXT.md`, dependencies, and recent run records; applicable
instructions and software workflow profile.

**N2 return:** exact changed-path/hash manifest; focused and whole-suite test
results; current whole-suite output showing `25/25`, `206/206`, zero mismatch,
zero block; byte/semantic comparison of the frozen original 91 observations;
C-B identity; containment; blockers and rerun triggers.

### N3 — fresh independent cross-package verifier

N3 is a fresh non-repairing ephemeral Agent 2 under HELP_HUMAN, or under a
single explicitly named verification-owning manager. It does not inherit an
author session. Its project reads are the complete candidate delta and frozen
basis. Its only writes are its managed launch/status/return artifacts and
external temporary logs directed outside durable project paths.

N3 independently verifies every section 4 predicate, including all 25 case
IDs, all 206 values, the exact 115 newly supplied observations, the original
91 projection, failure behavior, package boundaries, C-B identity and
nonlinear-only applicability, and exact write containment. It returns
`COMMIT_SAFE` or `BLOCK`; it performs no repair.

### N4 — integration, evidence, and closeout

N4 starts only after N3 returns `COMMIT_SAFE`. One named integration owner
may write only:

- DEL-09-01 `_STATUS.md`, `MEMORY.md`, and one new
  `_run_records/WORKING_ITEMS_RUN_*.md`;
- DEL-10-05 `_STATUS.md`, `MEMORY.md`, and one new
  `_run_records/WORKING_ITEMS_RUN_*.md`;
- the HELP_HUMAN managed run root, including final package returns, independent
  verification return, runtime telemetry/summary when required, exact
  manifest, and terminal `HANDOFF_STATE.md`;
- exactly one new clean-commit
  `validation/evidence/sweeps/SWEEP_<utc>_<commit>.json` produced by the
  mandatory DEC-025 gate;
- `loop/LOOP_RECEIPTS.md`, append-only, for exactly one next versioned receipt
  after every other gate passes.

The state updates record bounded implementation evidence only. They do not
change lifecycle state, close the public comparison-number residual, promote
manual pages, or alter Task Management. The terminal handoff distinguishes:
suite API produced, runner binding executed, verification evidence produced,
remaining owner decisions, derivative evidence status, rerun requirements,
and publication-token/merge gates.

## 7. Complete durable write fence

After adoption, the full tranche may durably change only:

1. `validation/benchmarks/mechanics/src/lib.rs`;
2. `core/runner/headless/src/benchmark_binding.rs`;
3. conditionally, only if N2 proves necessity within its sealed return,
   `core/runner/headless/src/bin/openpipestress-runner.rs` and
   `tests/test_headless_runner_contract.py`;
4. the two selected deliverables' `_STATUS.md`, `MEMORY.md`, and one new run
   record each;
5. one new HELP_HUMAN managed run directory under
   `execution/_Coordination/AgentRuns/`;
6. exactly one clean-commit DEC-025 sweep JSON;
7. `loop/LOOP_RECEIPTS.md`, append-only for exactly one receipt.

No other path is writable. In particular, the tranche must not edit:

- `docs/validation_manual/**`, including every case page and generator;
- any fixture JSON, hand calculation, expected-value record, benchmark
  README, mechanics `Cargo.toml`, or Cargo lockfile;
- `validation/benchmarks/nonlinear/**`, including C-B;
- schemas, public comparison policy/value records, existing evidence bundles,
  decision registers, DAG/decomposition, PRD, claims registry, workplans, or
  Task Management surfaces;
- GUI, report/export, CAEPIPE, `_DomainEngines/**`, app-dev, PEC, root-loop,
  external, private, user-model, or protected-content surfaces.

Network, cloud, daemon, telemetry, publication, external service, and private-
data operations are prohibited. Dependency provisioning is not authorized by
this brief; a missing prerequisite returns `HOLD` for a separate bounded
owner direction.

## 8. Evidence and validation plan

Each command is run as a separate halting step. Exact commands may be rendered
by the accepted software workflow profile, but their semantic coverage may
not be narrowed.

### Package-focused gates

1. `cargo fmt --manifest-path validation/benchmarks/mechanics/Cargo.toml --check`
2. `cargo test --manifest-path validation/benchmarks/mechanics/Cargo.toml`
3. `cargo fmt --manifest-path core/runner/headless/Cargo.toml --check`
4. `cargo test --manifest-path core/runner/headless/Cargo.toml`
5. focused runner-contract Python test if that surface is touched
6. exact 25-case whole-suite execution and machine verification of:
   `25/25`, `206/206`, `14/14`, `115/115`, original `91/91`, zero mismatch,
   zero block, no duplicate/extra/missing name, and finite observations
7. explicit negative tests for unknown ID, incomplete observation set,
   duplicate name, non-finite observation, suite execution failure, and
   expected/observed name mismatch
8. C-B byte/hash identity plus a code-path check that no mechanics comparison
   consumes C-B
9. applicable nonlinear tests proving no nonlinear behavior change

### Registered and repository gates

1. registered `piping-pytest`
2. full practitioner-harness pytest
3. practitioner-harness `self-check`
4. claims-language validation
5. path-anchor validation
6. JSON validation for every new JSON artifact
7. receipt validator before and after the single append
8. `git diff --check`
9. exact allowed-path containment and candidate canonical manifest
10. zero staged, untracked, or ignored drift outside the exact candidate

### DEC-025 merge gate

After the exact implementation/state candidate is committed locally, run
exactly one clean-commit five-surface DEC-025 sweep:

```text
python3 tools/release/run_evidence_sweep.py --execute
```

Require schema 2, the exact clean branch and commit, all five surfaces and
every command `PASS`, and overall `PASS`. The single summary is then handled
through the governed summary-only CHANGE step so the final branch contains
exactly one new sweep JSON. A failed sweep is retained as evidence outside the
accepted final delta, the branch returns `HOLD`, and no push or PR occurs.

## 9. Failure, side-effect, and cleanup rules

- Fail closed on stale base/input identity, dirty or ignored drift, scope
  need, missing dependency, incomplete observation, regression, verifier
  block, or validation failure.
- On failure, do not update deliverable completion claims, append the receipt,
  stage, commit accepted closeout, push, or open a PR. Preserve truthful run
  status and the exact blocker in the governed run root when that path is
  already authorized.
- A child never repairs outside its write fence. A manager never accepts a
  partial `25/25` or `206/206` result.
- Do not use `git clean`, reset, rebase, checkout-overwrite, or broad deletion.
  Generated ignored lockfiles, targets, caches, sweep failures, or other
  side effects require exact inventory and separate owner authorization before
  deletion unless they are wholly inside a predeclared disposable external
  directory whose removal was expressly authorized.
- Any cleanup authorization must identify the exact file or verified
  non-symlink directory, expected identity/manifest, and post-removal check.
  Cleanup never changes source or supplies validation evidence.
- No publication token may be inferred from implementation adoption.

## 10. Git, publication, and merge discipline

This is one tranche on the exact branch named in section 1 and produces one
non-draft PR targeting current `main`. Nothing is merged in session unless the
owner later directs that merge after required checks pass.

The sequence is:

1. owner adopts this exact brief;
2. CHANGE verifies or performs branch-first setup under a separate Git grant;
3. managed N1 through N4 execute and validate;
4. CHANGE stages only the exact validated manifest and creates the authorized
   local commit(s) needed to bind the clean DEC-025 sweep;
5. HELP_HUMAN presents the exact final commit, path manifest, tests, handoff,
   and sweep identity;
6. owner supplies a separate publication token authorizing push, upstream,
   and non-draft PR creation;
7. CI must pass; owner alone directs merge.

No child pushes, creates a PR, merges, force-pushes, rebases, or changes the
target branch. One PR carries both serialized packages so the prerequisite
API and its sole consumer land atomically.

## 11. Exact owner adoption token

To adopt this brief without amendment, return exactly:

```text
ADOPT CB-2026-08-11-DEL1005-MECHANICS-RUNNER-BINDING-001 EXACTLY AS PREPARED — EXECUTE THE SERIALIZED PKG-09/DEL-09-01 SUITE-OBSERVATION API THEN PKG-10/DEL-10-05 RUNNER-CONSUMPTION GRAPH FROM PINNED BASE F1E311FB7AB1C2A0800B1D32C59445368428DEE9 ON BRANCH CODEX/PIPING-DEC025-CASE-RUNNER-BINDING-20260811; REQUIRE 25/25 CASES, 206/206 VALUES, EXACT ORIGINAL 91-OBSERVATION NON-REGRESSION, BYTE-IDENTICAL NONLINEAR-ONLY DEC-046 C-B, FRESH INDEPENDENT VERIFICATION, ONE DEC-025-GATED PR, PUBLICATION TOKEN BEFORE PUSH, AND OWNER MERGE; ALL STATED FENCES REMAIN IN FORCE.
```

Any amendment, shortened token, changed base/branch, relaxed count, widened
write scope, or omitted fence requires a superseding brief or explicit exact
amendment before execution.

