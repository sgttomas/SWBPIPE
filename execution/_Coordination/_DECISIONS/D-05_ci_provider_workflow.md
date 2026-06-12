# D-05 — CI Provider + Hosted Workflow Location (RGAP-003)

**Status:** RULED — 2026-06-11 the human project authority selected **Option D** (defer hosted CI; the five-surface local sweep codified as one deterministic commit-bound entrypoint is the merge gate for parallel agent branches; F-4 atomic-build fix as rider; D-05b public-export CI follow-up prepared with D-06). Recorded as `DEC-025` in `execution/_Decomposition/SOFTWARE_DECOMP.md` §12; register row updated.
**Prepared:** 2026-06-11 by TASK (Type 2), requested by WORKING_ITEMS (Type 1), tranche TP-DECIDE-PREP-001 decision-preparation subscope.
**Register row:** `execution/_Coordination/_DECISIONS/_REGISTER.md` row D-05.
**Plan basis:** `plans/PLAN_2026-06-10_prd_completion.md` §2 row D-05.
**Epistemic posture:** evidence below is `FACT` with citations pinned at repo HEAD `5079a8fa7`; inferences and provider-capability statements (no network calls were made) are labeled `ASSUMPTION`; the recommendation is labeled `PROPOSAL`; unknowns stay `TBD`. This packet decides nothing.

---

## 1. Decision statement and scope

**Decide:** the CI provider and the hosted workflow location for OpenPipeStress continuous evidence — or an explicit deferral of hosted CI — including which of the five existing evidence surfaces run where, how the wasm toolchain and a real Chrome binary are provisioned, what gates merges of the planned parallel agent development branches, and what data the chosen location exposes against the IP/data boundary.

**In scope:** provider selection; workflow file location (private monorepo root vs a public sanitized export repo vs self-hosted vs none-for-now); the merge-gate semantics of "continuous evidence" for agent branches; provisioning preconditions (wasm32 target, pinned wasm-bindgen, Chrome); the F-4 atomic-wasm-build precondition for parallel lanes.

**Out of scope:** release matrix, installer formats, signing/notarization, publication targets (D-06, "with or after D-05", `plans/PLAN_2026-06-10_prd_completion.md` §2 row D-06); numerical tolerance and coverage thresholds (D-04); maintainer quorum and release authority (D-07); any release claim — RGAP-003 closure for a *release* claim also needs D-06/D-07.

D-05 blocks Phase E item E4 ("CI + release implementation per D-05/D-06", `plans/PLAN_2026-06-10_prd_completion.md` §3 E4) and "continuous evidence for all phases" (§2 row D-05); §5 states "E starts after D-05/D-06 decisions". A8's remaining scope names "CI browser provisioning policy" (§3 row A8).

---

## 2. Current state evidence

Citations pinned at HEAD `5079a8fa7`. Note: same-day D-10 ruling records exist as uncommitted working-tree edits to the register/plan/decomposition at preparation time; nothing below depends on them.

### 2.1 The decision is a standing, deliberately held TBD

- **FACT:** RGAP-003 wording: "CI provider, workflow path, release matrix, thresholds, signing, attestation, and publishing decisions remain unresolved" — `BLOCKER_TO_RELEASE_CLAIM`, owner PKG-09/PKG-10 (`execution/_Aggregation/TP-RELEASE-GAP-REGISTER-REFRESH-001_2026-05-31/Gap_Disposition_Register.csv` row RGAP-003).
- **FACT:** `docs/RELEASE_QUALITY_GATES.md` §10: "TBD: CI provider, release matrix, signing, release attestation, and maintainer quorum" (:161–162); the checklist itself "is not … CI implementation" (:24).
- **FACT:** `docs/BUILD_AND_RELEASE.md` §2: "no CI provider is selected; no `.github/` or other live workflow file is created by this deliverable" (:33–34); §9 "TBD: CI provider and hosted workflow location" (:168). §7 maps future provider workflows onto the provider-neutral readiness phases and adds the governing constraint: "Hosted CI must not receive private project data, private rule packs, private material/component libraries, protected standards content, signing secrets, or publishing credentials unless a later security and release-governance decision explicitly authorizes that handling" (:144–147). **D-05 is the natural site of that "later decision."**
- **FACT:** `execution/_Decomposition/SOFTWARE_DECOMP.md` §12: DEC-011 accepts Cargo/Vitest/Playwright/validation gates as the test baseline with "CI provider, coverage thresholds, and performance thresholds remain TBD" (:581); DEC-012 keeps the CI provider an implementation-level TBD "unless a sealed brief or later human ruling resolves them" (:582); DEC-017 prohibits hosted DB/daemon/required network/cloud sync/telemetry **in the product** (:587). **ASSUMPTION (boundary reading):** DEC-017 governs product runtime, not development infrastructure — CI is dev infrastructure — but the IP/data boundary still governs what data may reach any hosted surface.
- **FACT:** `docs/AGENTIC_DEVELOPMENT_WORKFLOW.md` keeps "CI provider/coverage thresholds" TBD pending the proper governance surface ("Data And Claim Boundaries", :331–337), and its contributor loop already requires per-run focused validation with recorded commands/results and a handoff evidence block ("Default Development Loop" step 6, :296; "Handoff Minimum", :355–373) — i.e., evidence-per-change is already mandatory; only its *execution location* is undecided.

### 2.2 The five-surface local evidence sweep (current de-facto practice)

**FACT:** the full evidence set, as last independently re-run (`plans/VERIFICATION_2026-06-11_operation_seam_unification.md`, "Independently re-run evidence" table):

| # | Surface | Command | Last result |
|---|---|---|---|
| 1 | Rust crate sweep | `python3 tools/release/check_release_readiness.py --profile cargo --execute` | 446 passed / 0 failed across 25 crate manifests |
| 2 | Python | `python3 -m pytest -q tests` | 342/342 |
| 3 | Desktop Vitest | `npm test --workspace apps/desktop` | 140/140 (7 files) |
| 4 | Playwright e2e | `npm run test:e2e:desktop` (root) → `npm run build:wasm && playwright test` (`apps/desktop/package.json:10`) | 1/1 (after the F-1 budget repair at HEAD `5079a8fa7`) |
| 5 | Production build | `npm run build --workspace apps/desktop` | green, index chunk at baseline |

Provisioning facts that any CI location must reproduce:

- **FACT:** Vitest requires the wasm engine artifact *before* it runs: `apps/desktop/src/test/setup.ts` pre-warms the engine (`await loadWasmEngine()`, :8–10) and fails loudly with `WASM-ENGINE-ASSET-ABSENT` if `npm run build:wasm --workspace apps/desktop` has not run (setup file header; wired via `setupFiles` in `apps/desktop/vite.config.ts:29`).
- **FACT:** the wasm build pins: `wasm32-unknown-unknown` Rust target plus the wasm-bindgen CLI at exactly `0.2.123` (`apps/desktop/scripts/build-wasm-engine.mjs:15` `PINNED_WASM_BINDGEN_VERSION`; crate pin `core/model_operations/operation_applier/Cargo.toml:25` `wasm-bindgen = { version = "=0.2.123" }`); every missing prerequisite fails with the exact remediation command (`build-wasm-engine.mjs:46–93`). `docs/BUILD_AND_RELEASE.md` §3 records the same prerequisite (:55–63); generated artifacts under `__generated__/` are never committed.
- **FACT:** Playwright resolves a **real Chrome binary**: `PLAYWRIGHT_CHROMIUM_EXECUTABLE_PATH` env override, else the macOS Chrome install path, else `undefined` (`apps/desktop/playwright.config.ts:4–11`). The config is already CI-aware (`reuseExistingServer: !process.env.CI`, :31), and the 120s budget was set from runtimes **measured on the maintainer's macOS hardware** (:15–19). `TBD`: whether a Playwright-managed Chromium (network download at provision time) is an acceptable substitute — no such path is exercised in-repo, and the seam plan defers "CI browser provisioning policy" (§2.3).

### 2.3 Standing constraints carried into this decision

- **FACT:** the seam plan's deferral list includes "Packaged-Tauri smoke over a saved edited project; CI browser provisioning policy" (`plans/PLAN_2026-06-11_operation_seam_unification.md` §9 item 5).
- **FACT (F-4):** "Do not run the cargo sweep and the desktop suites concurrently … `build:wasm` rewrites `src/services/wasmEngine/__generated__/` in place while Vitest/dev-server may read it. Sequential evidence runs … are fine. If parallel CI lanes arrive with `D-05`, give the wasm build an atomic temp-write-and-rename" (`plans/VERIFICATION_2026-06-11_operation_seam_unification.md` F-4). **ASSUMPTION:** hosted-runner jobs that each get a fresh, isolated checkout do not collide on `__generated__/`; F-4 bites for intra-job parallelism and for self-hosted/local shared worktrees.
- **FACT:** `docs/IP_AND_DATA_BOUNDARY.md` §3 lists what must never be public (standards text/tables, vendor catalogs without rights, user private rule packs/design bases, …; :44–53); §6 keeps private libraries/rule packs in user-controlled private paths (:85–87).

### 2.4 Repo/remote topology and existing CI config

- **FACT:** `projects/chirality-piping` is **not** a standalone git repo; its git toplevel is the private monorepo `/Users/ryan/ai-env/projects/chirality`, remote `origin = github.com/sgttomas/chirality` (single push/fetch remote).
- **FACT:** the monorepo `.gitignore` does **not** ignore `projects/` (this project is fully tracked), and `domains/*/_Sources/**` is excluded *except* re-included text formats (md/csv/xlsx/json/jsonl/html/txt; `.gitignore:64–75`) — so private project workspaces and domain source text **already reside on the private GitHub remote**. Hosted CI on that remote adds exposure to ephemeral runner VMs and any third-party actions, not to a new hosting party.
- **FACT:** GitHub Actions config already exists at the monorepo root — `.github/workflows/harness-premerge.yml` (PR-triggered, `ubuntu-latest`, needs `secrets.ANTHROPIC_API_KEY` and a `claude` CLI on PATH) and `.github/workflows/desktop-release-template.yml` (tag-triggered, `macos-latest`, signing secrets) — both target the monorepo `frontend/`; **neither references this project**. A nested copy exists at `projects/chirality-app-dev/.github/workflows/harness-premerge.yml`; **ASSUMPTION:** GitHub executes workflows only from the repo-root `.github/workflows/`, so a project-local workflow here would be inert. `TBD`: whether the root workflows currently execute successfully on the remote (no run evidence in-repo; the harness workflow's `claude`-CLI prerequisite suggests not without extra provisioning).
- **FACT:** no Woodpecker/Buildkite/GitLab/CircleCI/Jenkins/Drone configuration exists anywhere in the monorepo (search at HEAD).
- **FACT:** the existing public export (`exports/chirality-app/export_public.py` at the monorepo root) **excludes `projects/` and `domains/` entirely** (`SKIP_DIRS` :55–67; `boundary_findings` forbidden top-level set :203). No public OpenPipeStress repository or export profile exists yet; the "public OpenPipeStress repository" of `docs/IP_AND_DATA_BOUNDARY.md` §1 is prospective. `TBD`: the public-export mechanism for this project (Phase E surface).

---

## 3. Open questions awaiting ruling

1. **Provider and location:** GitHub Actions on the private monorepo (the only remote that exists), GitHub Actions on a future public OpenPipeStress repo, a self-hosted runner, or no hosted CI for now?
2. **Data-handling authorization:** does the human authority authorize hosted runners to receive the private monorepo contents (already resident on GitHub, §2.4) — the explicit authorization `docs/BUILD_AND_RELEASE.md` §7 (:144–147) requires — or is hosted CI confined to public/sanitized surfaces?
3. **Merge gate for parallel agent branches:** the human intends to open parallel agent development branches. Is the merge gate (i) a hosted required status check per branch, or (ii) the local five-surface sweep with a recorded artifact at fan-in (current loop practice, §2.1)? `TBD` until ruled; this is the operative meaning of "continuous evidence" mid-plan.
4. **Surface split:** which of the five surfaces (§2.2) must run hosted vs may stay local-only? In particular Playwright: real-Chrome provisioning, and budgets measured on macOS hardware (§2.2) — `TBD` whether Linux-runner timing differences require re-measured budgets.
5. **F-4 precondition:** is the atomic temp-write-and-rename for `build:wasm` a precondition of *any* parallel-lane configuration (§2.3), scheduled as a small tranche at activation?
6. **OS matrix:** Linux-only evidence lanes (cheap) vs macOS+Linux (matches the dev/packaging platform; **ASSUMPTION:** hosted macOS minutes bill at a ~10× multiplier vs Linux on GitHub-hosted runners — general knowledge, unverified).
7. **Monorepo coupling:** if workflows live at the monorepo root, OpenPipeStress lanes must be path-filtered to `projects/chirality-piping/**` and working-directory-pinned, and they would share the root `.github/` surface with the existing frontend workflows (§2.4). Acceptable, or does this push toward a later repo split? `TBD`.

---

## 4. Options

All options keep the product itself local-only (DEC-017, §2.1); they differ only in where development evidence runs.

### Option A — GitHub Actions on the existing private monorepo remote (hosted runners)

Root-level workflows path-filtered to `projects/chirality-piping/**`, jobs pinned to this directory, mapping the five surfaces to hosted jobs per `docs/BUILD_AND_RELEASE.md` §7. Provisioning per job: rustup + `rustup target add wasm32-unknown-unknown` + `cargo install wasm-bindgen-cli --version 0.2.123 --locked` (the build script's own remediation commands, §2.2), Node 20 + `npm ci`, Chrome via the runner image or `PLAYWRIGHT_CHROMIUM_EXECUTABLE_PATH` (**ASSUMPTION:** hosted Ubuntu/macOS images ship a usable Chrome/Chromium; unverified offline). Fresh per-job VMs isolate `__generated__/` (F-4 nuance, §2.3); the F-4 fix is still required before intra-job parallel steps. Merge gate: required status checks on PRs — the strongest per-branch independent evidence for the parallel-agent-branch plan.

- For: immediate hosted gating on the only remote that exists; independent execution environment (catches works-on-my-machine drift; adds a Linux signal); per-branch evidence scales with parallel agent branches instead of serializing on one machine; reuses existing repo/secrets infrastructure.
- Against: requires the explicit §7 data-handling authorization (open question 2) — private project/domain text becomes runner-visible and third-party-action-visible; macOS lanes are costly (ASSUMPTION, open question 6); monorepo coupling and CI noise across unrelated surfaces (open question 7); Playwright budgets need re-measurement on runner hardware (`TBD`); proves the private canonical tree, not what external engineers will receive.

### Option B — GitHub Actions on the public sanitized export repo only

Create the (currently nonexistent, §2.4) public OpenPipeStress export with an export profile mirroring `exports/chirality-app/` discipline; hosted CI runs the five surfaces against the public package only. The private canonical tree stays local-evidence-only.

- For: zero private data on hosted surfaces by construction — trivially satisfies `BUILD_AND_RELEASE.md` §7 and the IP boundary; CI proves exactly what external engineers get, which is the R5 exit criterion's reproducibility posture (`plans/PLAN_2026-06-10_prd_completion.md` §3 Phase E objective); clean repo-boundary story.
- Against: cannot gate private-monorepo agent branches — evidence arrives only after export/publish, so day-to-day merges still need local sweeps (Option B implies Option D for the merge gate); the export pipeline, publication decision, and RGAP-005 protected-content scan are prerequisites, pushing activation to Phase E; export↔canonical drift becomes a new failure mode to manage.

### Option C — Self-hosted runner on the maintainer's hardware

A GitHub Actions self-hosted runner registered to the private repo, executing the five surfaces on the same macOS machine that produced the measured Playwright budgets (§2.2). Execution data stays on maintainer hardware; GitHub orchestrates and stores logs/status only.

- For: hosted-style PR gating without moving execution data; toolchain (Chrome, rustup, pinned wasm-bindgen) is already provisioned; budgets already calibrated to this hardware.
- Against: single point of failure (one machine, must stay awake and reachable); standing maintenance cost; concurrency on a shared worktree directly triggers F-4 unless fixed and runner parallelism stays bounded — concurrency 1 re-serializes exactly what parallel branches were meant to parallelize; **ASSUMPTION:** standard security guidance forbids self-hosted runners on repos that could ever execute third-party PR code; log/status content must be kept evidence-only to avoid leaking private data into the hosted control plane; no additional OS signal (macOS only).

### Option D — Defer hosted CI; codify the local sweep as one deterministic entrypoint (formalize current practice)

No provider selected now. A single `make`-style entrypoint (a script, or a new `desktop`/`full` profile extending `tools/release/check_release_readiness.py`) runs the five surfaces **sequentially in F-4-safe order**, records a machine-readable summary artifact bound to the commit hash per `docs/BUILD_AND_RELEASE.md` §4, and is the required pre-push/fan-in step for every agent branch. The F-4 atomic-rename fix is done opportunistically (cheap; removes the local-concurrency hazard too). Hosted CI is re-decided at a named checkpoint.

- For: zero new IP exposure; zero provisioning work (pins already enforced by the build script's hard failures, §2.2); honest formalization of what actually carries evidence today (§2.2 is exactly this sweep); preserves the `BUILD_AND_RELEASE.md` §7 design that local and future hosted evidence stay command-comparable; cheapest now.
- Against: "continuous" evidence is only as continuous as agent/human discipline — an un-run sweep is a silent gap; no independent execution environment and no Linux signal (single-machine monoculture); parallel agent branches serialize their sweeps on one machine (~minutes each); does not advance RGAP-003 toward a release claim — the gap register's "release/CI authority" item stays open and lands on Phase E anyway.

---

## 5. Recommendation — `PROPOSAL`

Adopt **Option D now, with Option B named as the Phase E activation (suggested D-05b), and Option A available only under an explicit data-handling authorization if parallel-branch volume outgrows local sweeps first**:

1. **Now (merge gate for parallel agent branches):** formalize the five-surface sweep as one deterministic entrypoint with a recorded, commit-bound summary artifact; make it the required fan-in evidence for every agent branch (this is the existing loop's evidence discipline, §2.1, made mechanical). Schedule the **F-4 atomic temp-write-and-rename** as a rider tranche regardless of location — it also removes the local concurrent-run hazard.
2. **Phase E (suggested checkpoint D-05b, prepared with D-06):** stand up the public OpenPipeStress export and activate **Option B** hosted CI on it — hosted evidence then proves the artifact external engineers receive, with zero private-data exposure, aligned to the R5 exit criteria and `BUILD_AND_RELEASE.md` §7's mapping table.
3. **Escalation path, not default:** if parallel agent branches make local sweeps the bottleneck before Phase E, **Option A** is the named escalation — contingent on the human authority recording the §7 authorization (the private data is already resident on the remote, §2.4, but runner/action visibility is a new handling), Linux-first lanes (macOS lane optional/deferred to D-06 packaging), re-measured Playwright budgets, and the F-4 fix.
4. **Option C is not recommended:** it combines hosted CI's discipline costs with local CI's single point of failure, and re-serializes parallel branches.

Rationale: the only thing D-05 must deliver *mid-plan* is a trustworthy merge gate for parallel agent branches; the sweep already exists, is green at HEAD, and needs only mechanization (§2.2). Hosted CI's real payoff — proving the externally received package on independent infrastructure — is a Phase E property best bought where it costs no IP exposure (Option B), not by hosting the private canonical tree early (Option A's open authorization) or by a fragile always-on machine (Option C).

This recommendation is a `PROPOSAL` only. It confers no authority, selects no provider, and changes no state.

---

## 6. Downstream impact map

| Surface | Impact of this ruling |
|---|---|
| **Phase E4** | Defines E4's shape: provider workflows mapped to `docs/BUILD_AND_RELEASE.md` §7 phases at the chosen location (`plans/PLAN_2026-06-10_prd_completion.md` §3 E4; §5 "E starts after D-05/D-06"). |
| **D-06** | Sequencing partner ("with or after D-05", §2 row D-06): release matrix/signing inherit the location chosen here; a macOS lane decision can defer to D-06 packaging. |
| **Parallel agent branches** | Fixes the merge-gate semantics (hosted check vs recorded local sweep) before branch fan-out begins — open question 3. |
| **F-4 fix tranche** | Ruling activates (or explicitly defers) the atomic temp-write-and-rename for `build-wasm-engine.mjs` (`plans/VERIFICATION_2026-06-11_operation_seam_unification.md` F-4). |
| **A8 residual / seam-plan deferral** | Resolves the deferred "CI browser provisioning policy" (`plans/PLAN_2026-06-11_operation_seam_unification.md` §9 item 5; completion plan §3 A8). |
| **`docs/BUILD_AND_RELEASE.md`** | §2/§9 TBDs close partially; §7's hosted-data prohibition gains its explicit authorization record or is reaffirmed (:144–147). |
| **`docs/RELEASE_QUALITY_GATES.md`** | §10 "CI provider" TBD closes; gate records gain a named evidence-execution location (:161–162). |
| **RGAP-003** | Partial disposition (provider + workflow path); release matrix/signing/attestation/publishing remain with D-06/D-07 (`Gap_Disposition_Register.csv` row RGAP-003). |
| **Public export surface** | Option B (now or as D-05b) creates the first public OpenPipeStress export profile — a new Phase E work item with RGAP-005 scan coupling. |
| **Register row D-05** | Dispatching persona updates `_REGISTER.md` row D-05 to AWAITING_RULING → RULED with the ruling pointer; this packet does not touch the register. |

---

## 7. Authority and ruling record

Only the **human project authority** rules on D-05. Agents prepared this packet and may not certify, approve, or adopt it.

Per existing decision practice, the accepted ruling is recorded as a `DEC`/`SCA` entry in `execution/_Decomposition/SOFTWARE_DECOMP.md` §12 — D-01, D-08, and D-10 were recorded this way as `DEC-018`, `DEC-019`, and `DEC-021` — after which the dispatching persona updates `execution/_Coordination/_DECISIONS/_REGISTER.md` row D-05 from `AWAITING_RULING` to `RULED` with a pointer (`_REGISTER.md` header). If the ruling includes the Option A escalation path, the `BUILD_AND_RELEASE.md` §7 data-handling authorization must be part of the recorded decision text, not implied. This packet does not edit the register and does not resolve the decision.
