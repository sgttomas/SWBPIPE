# Dual-WASM dependency provisioning diagnosis

RUN_STATUS: SUCCESS (read-only diagnosis complete; repair unimplemented and unvalidated)
ControlSurface: MERGED; Role: ephemeral Agent 2; TaskProfile: NONE.
Method: generic bounded diagnosis; software-defect-diagnosis v1 and TASK instructions inspected as requested, not instantiated as TASK. The sealed ephemeral brief explicitly permits read-only shell/parser and own evidence writes; no registered check, source execution, network, install, build, tests, Git command or delegation used.
Runtime model: unavailable. Non-delegation: instruction+config asserted.

## Evidence and causal chain

Parent-declared source basis: `079f865bc5507fc28ad52bc4b3186e563bb26ce0`, PR715 run33970080166. Revision association is parent supplied, not independently Git verified (Git forbidden); input hashes capture the inspected working bytes. CWD plus `.git` file establish the active worktree location. Raw log mapping: `{TEMP_ROOT}/piping-ui-pr715-desktop-failed.log` copied byte-for-byte to `_run_records/piping-ui-pr715-desktop-failed.log`; hashes and package tuples in `_run_records/dependency-comparison.json`.

1. `.github/workflows/piping-desktop-e2e.yml` fetch step runs unconditionally before build, but its sole `cargo fetch --locked --manifest-path` names `core/model_operations/operation_applier/Cargo.toml`. No `--target` means it intends complete locked graph provisioning including host/transitive dependencies.
2. `apps/desktop/scripts/build-wasm-engine.mjs` inside the project enumerates operation_applier then loads/self_weight_wasm. Each gets a separate crate-local `cargo build --offline --target wasm32-unknown-unknown --features wasm --release`; all subprocesses also get `CARGO_NET_OFFLINE=true`. Current builds do NOT pass `--locked`; preserve their offline behavior and use explicit locked builds in verification rather than claiming the flag already exists.
3. Both crates pin wasm-bindgen=0.2.123, but their independently resolved lockfiles differ. Registry sets contain 29 operation and 30 self-weight tuples, 19 shared. Eleven self-weight registry tuples do not appear in operation graph: libc0.2.189, memchr2.8.3, proc-macro2 1.0.107, quote1.0.47, serde/serde_core/serde_derive1.0.229, serde_json1.0.151, syn2.0.118, syn3.0.3, zmij1.0.23. Exact versions and checksums are preserved in JSON. These are absent from the *operation graph*, not all proven absent from the runner cache or active target closure.
4. Raw log lines20–23 show operation offline compile/glue success followed by second offline compile. Lines27–30 show missing download `memchr v2.8.3`, forbidden HTTP offline. That directly supports incomplete dependency provisioning before self-weight compilation; browser execution had not begun. Only memchr's absence is individually proved by this failed-step log.
5. Registry/target cache covers registry plus operation target only, and key hashes only operation Cargo.lock. Broad restore prefix may restore old or incidental entries. No source self-weight target cache and no self-weight lock key input are contributing cache incompleteness/performance concerns, not a substitute for unconditional fetch. wasm-bindgen CLI installation/cache may incidentally warm registry but cannot supply a contract for either app graph.

## Smallest corrective fence

Required source-write fence: `.github/workflows/piping-desktop-e2e.yml`, existing locked-fetch step only. Convert the folded scalar to a multiline literal and run both commands, retaining the existing working-directory and unconditional execution before build:

```yaml
      - name: Fetch complete locked WASM dependency graphs
        run: |
          cargo fetch --locked --manifest-path core/model_operations/operation_applier/Cargo.toml
          cargo fetch --locked --manifest-path core/loads/self_weight_wasm/Cargo.toml
        working-directory: projects/chirality-piping
```

This is the smallest causal repair. Parent's final fence additionally includes both exact Cargo.lock paths in the existing cache key hashFiles call, covering each independently locked graph. Preserve existing cache paths and restore prefix; target-cache expansion is excluded. Cache-key coverage remains a contributing maintenance fix, while unconditional dual fetch provides correctness. Do not gate fetch on cache-hit. No lockfile churn, dependency downgrade, build-network relaxation, broad root-governance changes, browser timeout changes, or product source edits are required. Parent has confirmed this workflow-specific write scope, with no nested .github AGENTS and only the Piping path trigger/job; pins, security settings, events, and optional target-cache expansion are excluded.

## Check plan (not executed)

1. Freeze proposed workflow and both manifests/locks; record hashes and exact Rust1.97.1/wasm-bindgen0.2.123/Node24 versions. In an authorized isolated disposable checkout use a fresh empty task-specific CARGO_HOME and fresh CARGO_TARGET_DIR, while keeping the installed toolchain and pinned CLI available. Do not share registry or compilation target cache with a previous run. Pin CLI availability before the cold graph exercise so CLI installation cannot incidentally populate the new Cargo registry.
2. Execute the exact two locked fetch commands from the proposed workflow, online and without --target (complete host/transitive graph), against that fresh CARGO_HOME. Record outputs and exit codes.
3. With the same fresh Cargo home/target, execute `cargo build --locked --offline --manifest-path <each of the two manifests> --target wasm32-unknown-unknown --features wasm --release`, setting CARGO_NET_OFFLINE=true. Then run actual `npm run build:wasm:desktop` with that same environment to prove metadata resolution and both pinned wasm-bindgen outputs. Record artifact hashes, success of both engines, and unchanged lockfile hashes. npm dependencies may be preprovisioned separately.
4. Optional causal counterfactual in a separate empty cache: fetch operation alone, attempt self-weight locked/offline build; expect absent dependency. Never count incidental warm-cache success as a disproof of the omission.
5. Narrow structural regression if authorized beyond the minimum fence: execute the real build script in an existing fake-tool harness that records cargo build --manifest-path calls; parse the actual workflow fetch step and assert every observed manifest has an unconditional --locked fetch before build. Require working-directory resolution and retain offline build contract. Avoid hardcoding a second mirror list as the asserted inventory. Current fake-tool test verifies artifact target resolution and rollback; it cannot establish registry provisioning. A regression addition is optional scope expansion, not necessary to the one-file repair.
6. Finally rerun actual hosted Piping Desktop E2E on the repaired exact source commit, including source-mode browser projects. Cold dependency proof establishes provisioning, not browser test pass. Capture run URL/id, commit association and full logs. Existing local/host project gates remain manager-owned.

## Handoff state

Derivative diagnosis package; not decomposition truth, release approval, or accepted repair snapshot. Accepted upstream is parent-declared source079f865; this packet awaits parent fan-in acceptance. Closure verdict: diagnosis complete, CI failure unresolved until repair and checks. Rerun requirements: authorized cold-cache proof and exact-commit hosted CI. Remaining blockers: implementation and validation in the parent-authorized follow-on; no source/network/build/test authority in this child. BUILD_AND_RELEASE.md contains older hosted-CI deferral wording and a legacy artifact path; route applicability to parent without modifying governance or inferring a waiver.

## Tool and write compliance

ToolsUsed: zsh cat, sed, rg, pwd; python3 pathlib/tomllib/hashlib/json (read/parse/hash and explicitly permitted own-evidence writes). ToolPolicyCompliance: PASS against sealed ephemeral brief. WriteAuthorization: EXPLICIT_BRIEF_TEXT, own DIAGNOSE evidence only. AppliedChanges: preserved failed log, comparison JSON, this report, SHA256 manifest. Source modifications: none. Missing: full CI provisioning/cache-step logs and independent source revision proof; causal claim is calibrated to failed-step log and inspected source. NeedsHumanRuling: none in this child; parent owns fence acceptance.
