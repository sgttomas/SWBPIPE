# One-file dual-Wasm CI provisioning repair

RUN_STATUS: SUCCESS — implementation and isolated local proof complete; independent review and exact-commit hosted CI remain parent-owned.
ControlSurface: MERGED. Role: ephemeral Agent 2; TaskProfile: NONE. TaskSkill: software-bounded-implementation v1 method context. Exact model unavailable; nondelegation instruction+config asserted.

## Accepted basis and scope

Parent accepted plan CI_PROVISIONING_V1/ACCEPTED_PLAN_V1.json, instruction scope check, sealed IMPLEMENT/LAUNCH_BRIEF.md, and DIAGNOSE/RETURN.md supplied the bounded Piping CI integration objective. Parent-declared source revision is 079f865bc5507fc28ad52bc4b3186e563bb26ce0; no Git command independently checked this association because Git was excluded. Workflow preimage matched accepted SHA256 7aec5e6cefab45cc88d8b6c81f7e8100b99fd37531b1bf4aa378bb64a5d481cd exactly.

The sole product write is .github/workflows/piping-desktop-e2e.yml. Its cache key now hashes both existing lockfiles; the unconditional literal fetch block runs cargo fetch --locked for operation_applier and self_weight_wasm before the unchanged offline dual builder. Pins, triggers, permissions, concurrency, cache paths/restore prefix and all other workflow steps remain byte-preserved outside the two approved diff hunks. No source, manifest, lockfile, tool, Git or other workflow changes were made. Generated ignored public artifacts were rebuilt as explicitly authorized.

Frozen workflow SHA256: 4db4a1e95a581e3eeaaff39a59b359c53635cf90ec080093bbe661f6600ee86f. Source was frozen before proof and remains unchanged. Full before/frozen bytes and exact encoded patch are in _run_records/.

## Proof and results

A task-specific mkdtemp directory provided initially empty CARGO_HOME and CARGO_TARGET_DIR. No registry or target cache was copied. isolation.json records resolved runtime locations; executable proof.py derives the repository from invocation CWD and project from its portable relative anchor. The existing stable-aarch64-apple-darwin installation is exactly rustc1.97.1/cargo1.97.1; wasm-bindgen0.2.123 and Node24.18.0 were present. No tool installation or upgrade occurred.

1. Initial numeric rustup alias selection failed before download because that alias was not installed and rustup could not write its temp file. The existing stable alias was selected after verifying the exact version. This initial failure is retained.
2. Locked operation fetch in the sandbox failed DNS resolution. The exact proof command was escalated and approved, then fetched the locked graph successfully into the isolated cache. Only authorized cargo fetch commands used network.
3. With only that graph fetched, self-weight --locked --offline wasm32 release build failed as expected: missing libc0.2.189. This is a causal counterfactual, not a failed acceptance gate. The prior actual hosted failure named memchr2.8.3; the difference is dependency traversal order.
4. The exact locked self-weight fetch escalation was approved and completed, downloading the eleven additional graph tuples, including memchr2.8.3.
5. Both crates then completed cargo build --locked --offline --target wasm32-unknown-unknown --features wasm --release, exit0, with CARGO_NET_OFFLINE=true and the same isolated cache/target. Raw full compiler output is preserved.
6. Actual unchanged npm run build:wasm:desktop completed exit0 under that same isolation and offline environment, generating both public engines with pinned CLI0.2.123. The actual script uses --offline but does not add --locked; the separate explicit locked builds and unchanged lock hashes substantiate the locked proof.
7. All 135 protected hashes (workflow frozen bytes, project npm manifest/lock, builder, core manifests/locks and Rust sources) matched before/after. Eight generated artifact SHA256 hashes are in artifacts.json. Scope validator PASS used explicit --path, so it executed no Git; this validates the declared changed product path and is not a whole-worktree Git audit. Affected-check selector returned harness-self-check; it was not executed because the sealed brief authorizes the targeted provisioning proof and excludes unrelated suites.

Proof gate verdict: PASS on local macOS host with exact pinned tool versions, fresh isolated registry and target, dual locked fetch, dual locked offline compilation and actual unchanged builder. This does not claim Ubuntu runner or browser E2E success. Hosted exact-commit CI confirmation remains required after parent push.

## Evidence and reproduction

_run_records/commands.jsonl binds commands, times, exit codes and decoded original log SHA256. RAW_EVIDENCE_MAP.json maps those logical raw names and workflow.diff to *.encoded.json files. Decode each data field with base64; decoded_bytes and decoded_sha256 bind original bytes exactly. Encoded records preserve trailing whitespace and control bytes without introducing patch whitespace violations. Every encoded record was decoded and byte-compared before raw copies were removed. No .gitattributes exception was introduced. The original failed CI log is also preserved this way.

proof.py accepts a phase and attempt label, runs from {REPO_ROOT}, resolves isolation.json, and retains commands/output. For a rerun create new task-specific empty cargo/target directories and update a new copy of isolation.json; do not reuse this run's warmed paths as cold proof. Stages: fetch-operation; optional probe-selfweight (expected nonzero); fetch-selfweight; build-operation; build-selfweight; actual-builder. Preserve network escalation requirements for fetches. Commands after fetches enforce CARGO_NET_OFFLINE=true.

## Instruction, tool and write compliance

Read root/project AGENTS (root supplied in conversation), TASK, skill v1 and all three companions (BRIEF_SCHEMA, TOOL_POLICY, QA_CHECKS found), accepted plan/scope and diagnosis. Runtime CHIRALITY_INSTRUCTION_ROOT was absent; execution remained the explicitly requested ephemeral generalist with repository method contracts as declared context, not instantiated TASK runtime. The explicit root workflow integration fence and proof authority in the sealed brief override narrower generic project/method defaults.

ToolsUsed: zsh cat/rg/ls; python3 pathlib/hashlib/json/base64/difflib/tempfile/subprocess; installed rustc/cargo/rustup/wasm-bindgen/node/npm; python3 tools/software_workflow/select_affected_checks.py; python3 tools/software_workflow/validate_change_scope.py. ToolPolicyCompliance: PASS against explicit ephemeral brief. WriteAuthorization: EXPLICIT_BRIEF_TEXT; exact workflow, own evidence, isolated temporary caches and ignored generated artifacts only. No delegation, Git action or install. Source freeze and protected hashes provide bounded evidence; unrelated external changes are untouched.

## Handoff

Derivative implementation evidence based on the parent-accepted plan and parent-declared source079f865; not decomposition truth, governance acceptance, release approval or lifecycle issuance. Closure verdict: bounded implementation/local provisioning proof complete, awaiting parent fan-in and fresh full workflow review. Required reruns: hosted Piping Desktop E2E on exact repaired commit. Remaining blockers: none for local proof; review and hosted CI are pending parent gates. MISSING: independent Git revision proof intentionally excluded; hosted runner validation. NEEDS_HUMAN_RULING: none. DEPENDENCY_NOTES: none.
