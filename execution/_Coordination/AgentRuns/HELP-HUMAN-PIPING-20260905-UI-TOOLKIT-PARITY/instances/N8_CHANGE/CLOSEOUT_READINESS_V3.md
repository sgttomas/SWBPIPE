# CHANGE terminal preparation, before execution activation

N7 V2 review return SHA-256 is verified as `3f11e7e1be3a350e436e0a1ff40ca6c4725eeff02c78e7dc831194e0a4a20676`. It records PASS, no actionable findings, and F1–F3 closure. CHANGE independently checked every one of the 110 V2 frozen member hashes against the current checkout: zero drift. Original V1 CHANGES_REQUIRED remains unchanged. Exact N7 V2 membership accounting includes 94 unchanged, 14 changed, two added and no removed members; unchanged coverage is explicitly bound to that reviewer's earlier full inspection.

`READINESS_V3.json` records provisional status and local executable versions. `STAGING_CANDIDATE_V3.txt` is the readable exact observed path list; `.nul` preserves path boundaries for future Git pathspec use. Neither has been staged or executed. All 1010 observed paths are project-scoped; index remains empty. These candidates exclude the N8 files just written and any later fan-in evidence: regenerate exact final membership at activation rather than blindly using this snapshot. Required package acceptance and parent final handoff are still pending as of the dispatch; CHANGE has not substituted reviewer PASS for parent acceptance.

## Prepared clean-sweep environment

Resolve `REPO_ROOT` from Git and `WORKING_ROOT={REPO_ROOT}/projects/chirality-piping`. Resolve `PY313_BIN` and `CARGO_TARGET_DIR` from the N_ENV provisioned session environment. For this session the prepared environment directory basename is `piping-ui-parity-py313` in the operating-system temporary root; its `bin/python3` reports Python 3.13.14. The prepared Cargo target directory basename is `piping-ui-parity-target` in that same temporary root. Both stay outside the checkout. Use that Python bin directory first in PATH so sweep subprocesses use the checked interpreter. Verified tools: Node 24.18.0, npm 11.16.0, Cargo/rustc 1.97.1, wasm-bindgen 0.2.123, installed wasm32 target. No manifest/toolchain changes or downloads were performed by CHANGE.

After parent activation, source commit, and verified globally clean checkout, execute the complete sweep from `{WORKING_ROOT}` with this environment:

```sh
PATH="${PY313_BIN}:$PATH" CARGO_TARGET_DIR="${PIPING_CARGO_TARGET}" CARGO_NET_OFFLINE=true PLAYWRIGHT_WORKERS=1 "${PY313_BIN}/python3" tools/release/run_evidence_sweep.py --execute
```

`PY313_BIN` must resolve to the provisioned environment's **bin directory**; `PIPING_CARGO_TARGET` resolves to the prepared temporary target root. The tool call at execution will contain the actual resolved paths and exact command. Capture console output outside the checkout until the sweep returns. The existing sweep already forces offline Cargo, probes cached manifests in temporary copied source roots, verifies local Node/browser prerequisites and Python minimum before surfaces. Its complete execution is intentionally held until the clean source commit; this preparation claims tool availability, not a completed full offline-cache preflight or DEC-025 PASS. If the full preflight identifies an uncached newly added crate, provision its exact dependency cache through the already authorized environment owner and repeat the same clean source check; do not rewrite dependencies or relax offline mode.

The complete host-inclusive command requires exact-command sandbox escalation because Chromium needs host capabilities. Project AGENTS authorizes requesting and executing this host surface; no extra owner gate is invented. A partial capability-selected sweep is never the final complete evidence. The sweep itself remains strictly sequential and the new dual Wasm builder must not run concurrently with another writer/build. No active agent may write tracked or untracked checkout files until its clean Git snapshot has been captured and all source-bound execution finishes.

## Remaining prerequisites

1. N1 accepted final package handoff and parent immutable cross-manager fan-in/closure direction, explicitly releasing Git closeout.
2. Recheck final inventory, 110 frozen hashes, preserved review hash, all current check/evidence bindings, receipt validator, and any post-review documentation classification. New source/contracts require renewed review; evidence-only finalization requires evidence checks and no false source-hash change claim.
3. Source commit only after the parent's explicit activation; complete DEC-025 next; then source-unchanged evidence/receipt commit and freshly verified ordinary push to `origin/refs/heads/codex/piping-ui-toolkit-parity`, PR and actual CI verdicts. No merge/rebase.

Prepared check basis reported by parent and independently covered by N7 V2: Piping V3 1011; harness V5 350 with baseline-only selfcheck; desktop V9 732/build/built-dist; exact regenerated F1 blocked and four US cases PASS. These are precommit proofs, not DEC-025. Full tranche closeout remains NOT_CLOSED. No Git mutation or DEC-025 execution occurred during this preparation.
