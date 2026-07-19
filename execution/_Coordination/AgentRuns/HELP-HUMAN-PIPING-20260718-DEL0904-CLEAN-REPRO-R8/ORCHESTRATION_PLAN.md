# ORCHESTRATION PLAN — DEL-09-04 Clean Reproduction R8

- RunID: `HELP-HUMAN-PIPING-20260718-DEL0904-CLEAN-REPRO-R8`
- Parent: `HELP_HUMAN`
- Manager: `WORKING_ITEMS`
- Package: `PKG-09`
- Selected deliverable: `DEL-09-04`
- Posture: `TERMINAL_FAN_OUT_IN`
- Selection authority: explicit HELP_HUMAN launch under the effective adopted candidate brief
- Plan version: `1`
- Frozen before dispatch: `2026-07-19T03:32:49Z`
- SOURCE_COMMIT: `89a93d7ca21d64c57cc344955d17deef709fd685`
- Reproduction RUN_ID: `REPRO_DEL0904_20260719T033249Z_89a93d7ca21d`
- Branch: `codex/piping-del0904-clean-repro-20260718-r8`

## Accepted Basis and Live Revalidation

- Cleanup merge `525ef0903e68b536ff5b22f985263ca737a67986` is an ancestor of SOURCE_COMMIT.
- The branch and source tree were clean before this record was written: empty short status beyond the branch header, `git diff --exit-code` 0, and `git diff --cached --exit-code` 0.
- Active DAG pointer resolves to approved `DAG-007`.
- All eight active DEL-09-04 `EXECUTION / UPSTREAM` rows are `SATISFIED`: `DEL-00-01`, `DEL-00-02`, `DEL-00-06`, `DEL-00-08`, `DEL-09-01`, `DEL-09-02`, `DEL-09-03`, and `DEL-01-04`.
- DEL-09-04 remains `IN_PROGRESS`; the exact actor-neutral clean-checkout reproduction Remaining bullet is present.
- Candidate brief SHA-256: `72521c0ae90fc04d5d2e22ff3e3d0be5e96561fe3e2d3847b546c4fa26af1951`.
- E1 procedure SHA-256: `5fee14dd6ed62e4b75ef833af2ad9de2e711a83889be94d10b9a0a7de230418d`.
- Project workflow profile SHA-256: `123249634475e87207cd75740dc25e5061c08cc7a1708aa239105b27e30c9c2f`.
- Root workflow contract SHA-256: `f97af1d323524f9a2be1dab8b5b33c1350c8f48c38cd3c6f8d0f8a9cd9821ea2`.
- Receipt 56 remains the applicable prerequisite-repair cursor.
- The exact registered prerequisite function was re-run from WORKING_ROOT with `PYTHONDONTWRITEBYTECODE=1` and `CARGO_NET_OFFLINE=true`; it returned zero errors.
- Fresh manager and bundle paths were absent before the first durable write.

## Immutable History Protections

- R3 managed-run aggregate hash: `f530f329a32270664fbcaf19a15b7f51f04e2d3950acd3be8b096cfd62bf101b`.
- R3 reproduction-bundle aggregate hash: `81e9ee96bf1866e6642b39dbf24fd2e6f6d0431934f5bffb9159700b7b7d4d54`.
- R7 managed-run aggregate hash: `306eed9a0e90a75315a64aa1a099a4a2b2aba08c1e271aeda5d69f28386bcf53`.
- R7 reproduction-bundle aggregate hash: `df565da8799a27b21d37b1cdf772eef040d04385d502440b1f1118a30bbd2d0f`.
- P1 managed-run aggregate hash: `bcb1324ed72b895abef6be68a603376f81280eab441408aa77e81c2f2ec612fb`.
- R3 is immutable terminal FAIL; R7 is immutable terminal BLOCKED; P1 is provisioning audit only. None may be modified, reused, copied as reproduction evidence, overwritten, amended, or reinterpreted.

## Work Graph

Exactly one serialized Bash-bearing Agent 2 owns the bounded project-root integration stage. It executes the full accepted candidate objective, including the clean temporary clone, offline runtime/review commands, predicate checks, derivative bundle, PASS-only closeout, four registered profile checks, exactly one evidence sweep, Receipt 57, and final immutable checksum. The child does not delegate.

No second execution or verifier child exists. WORKING_ITEMS independently validates fan-in after the terminal return.

## Fan-In Gates

1. Child return schema and terminal objective status are complete and truthful.
2. Runtime, exact predicates, review commands, registered profile checks, and validators match the candidate brief.
3. Exactly one new sweep exists on PASS; none is inferred or reused.
4. Bundle layout, manifest provenance, raw command records, JSON/JSONL parsing, checksum coverage, checksum verification, and immutability pass.
5. Source/temporary clone cleanliness and candidate write containment, including untracked paths, pass.
6. R3/R7/P1 protected aggregate hashes remain unchanged.
7. On PASS only, exactly the target Remaining bullet is removed, lifecycle stays `IN_PROGRESS`, other Remaining bullets remain, MEMORY and exactly one new DEL-09-04 WORKING_ITEMS run record are appended, and exactly Receipt 57 is appended with Parent Receipt 56.
8. No ignored build artifact is treated as source or reproduction evidence.

## Failure Posture

Any failed or blocked runtime, review, profile, validation, containment, or checksum predicate terminalizes the unique R8 bundle/run truthfully, keeps `_STATUS.md`, `MEMORY.md`, and Receipt 56 unchanged, performs no repair, and returns the exact blocker. No install, download, network, provisioning, Git closeout, acceptance, lifecycle/stage/release/prover/publication, or external action is authorized.
