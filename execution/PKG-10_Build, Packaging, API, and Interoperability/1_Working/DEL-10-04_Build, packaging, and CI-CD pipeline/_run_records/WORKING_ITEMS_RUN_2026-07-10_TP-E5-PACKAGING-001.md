---
doc_id: WORKING_ITEMS_RUN_2026-07-10_TP-E5-PACKAGING-001
doc_kind: execution.run_record
status: completed
created: 2026-07-10
agent: WORKING_ITEMS
tranche_id: TP-E5-PACKAGING-001
deliverable_id: DEL-10-04
package_id: PKG-10
---

# WORKING_ITEMS Run Record - TP-E5-PACKAGING-001

## Scope

The separate E5 implementation tranche named by the `DEC-057` ruling (D-06
Option O-A, adopted as recommended with both riders;
`execution/_Decomposition/SOFTWARE_DECOMP.md` §12; packet
`execution/_Coordination/_DECISIONS/D-06_release_matrix_installers_publication.md`;
completion-plan row E5, `plans/PLAN_2026-06-17_prd_completion.md`): the ruled
v0.1 packaging mechanics for macOS Apple Silicon (`aarch64-apple-darwin`)
only.

Delivered: `bundle.active` enabled with explicit `["app"]` targets and a
real multi-resolution `.icns` in `apps/desktop/src-tauri/`; the invented
OpenPipeStress mark and its deterministic generator; the deterministic
zip + SHA-256 checksum packaging path; and the `docs/BUILD_AND_RELEASE.md`
§8 release-artifact-record emitter carrying the DEC-057 unsigned-install
caveat and the checksum + commit-bound-sweep authenticity chain.

Deliverable basis: plan row E5 names DEL-10-04 (RGAP-003); the governing
guide `docs/BUILD_AND_RELEASE.md` is DEL-10-04's deliverable; prior
release-engineering tooling run records live in this `_run_records/`
directory (`WORKING_ITEMS_RUN_2026-06-11_TP-SWEEP-001_evidence_sweep.md`,
`WORKING_ITEMS_RUN_2026-06-15_TP-C4-SWEEPHARDEN-001.md`).

**Hard fences honored:** no release act, no publication, no version tag, no
GitHub Release, no signing/notarization (D-06b stays `NOT_PREPARED` and no
D-06b packet was prepared), and no release-readiness claim. This tranche is
packaging mechanics only (F-PIP-2 development evidence).

## Files Touched

- `apps/desktop/src-tauri/tauri.conf.json`: `bundle.active: true`, explicit
  `targets: ["app"]` (the ruled `.app`-only installer format), icon list
  `icons/icon.icns` + `icons/icon.png`. Previously `active: false` with an
  empty icon list, which forced the `--bundles app` flag (SMOKE TP-MAC-141);
  `npm run tauri -- build` now produces the `.app` from config alone.
- `apps/desktop/src-tauri/icons/icon.icns` (new) and `icon.png` (replaced —
  the prior file was a 105-byte 32x32 placeholder): the invented
  OpenPipeStress mark — a pipe cross-section annulus with an amber stressed
  sector on a dark steel rounded square. Original invented content; no
  third-party art. The `.icns` carries PNG members at 16/32/64/128/256/512/
  1024 px including the @2x pairs (icp4..ic14), assembled in pure Python.
- `tools/release/generate_app_icon.py` (new): deterministic pure-stdlib
  generator (SDF render at 1024, integer box downscale on premultiplied
  alpha, minimal PNG encoder, PNG-member `.icns` assembler). The committed
  assets are byte-identical to the generator's output (guarded by test).
- `tools/release/package_release_artifact.py` (new): the DEC-057 packaging
  entrypoint — dry-run by default; `--execute` zips the built `.app`
  deterministically (sorted entries, commit-derived UTC timestamps, unix
  permissions and symlinks preserved, no extra fields, deflate level 9),
  writes `<zip>.sha256` (`shasum -a 256`-compatible), enforces the ruled
  thin-arm64 Mach-O check on the bundle's `CFBundleExecutable`, and emits
  the §8 record
  `validation/evidence/release_artifacts/RELEASE_ARTIFACT_<utc>_<commit12>[-dirty].json`
  with the DEC-057 ruled-shape fields, the verbatim unsigned-install caveat,
  and the authenticity chain (checksum + commit-bound `DEC-025` sweep +
  record). Exit 0 only when the chain is `verified` (clean HEAD matching a
  passing clean-tree sweep, or trailing it by evidence-only closeout
  commits per the TP-MAC-141 provenance precedent). Reuses the sweep's
  git-binding/runtime-capture helpers by module load; the `DEC-025` sweep
  itself is unmodified (guarded by test).
- `tests/test_release_packaging_script.py` (new): 17 focused tests — ruled
  constants, bundle-config shape, `.icns` structure, icon reproducibility,
  deterministic zip byte-identity + mode/symlink preservation, checksum
  format, Mach-O arch check, `CFBundleExecutable` resolution, authenticity
  chain states (incl. evidence-only-delta acceptance and code-delta
  rejection via a scratch git repo), §8 record fields + caveat, record
  filename commit binding, execute end-to-end with a fake bundle, loud
  failures on missing bundle / wrong arch, sweep-plan-unmodified guard.
- `docs/BUILD_AND_RELEASE.md`: §2 authority boundary updated to the ruled
  DEC-057 posture; §6 checklist + new §6.1 ruled v0.1 package path; §8
  gains §8.1 (record emitter mechanics) and §8.2 (authenticity chain +
  verbatim unsigned-install caveat); §9 matrix/installer/signing and
  container-naming TBDs closed to the DEC-057/DEC-028 pointers.
- `docs/RELEASE_QUALITY_GATES.md` §10: the release-matrix/signing/
  attestation TBD closes to the DEC-057 ruled pointer (attestation =
  checksum + commit-bound sweep + §8 record until D-06b).
- `plans/PLAN_2026-06-17_prd_completion.md` E5 row: packaging portion marked
  landed; remaining E5 scope (provider workflows §7, DEC-059 conditional
  public-export CI activation, D-06b signing) left in the row.
- `plans/PLAN_COMPLETION_LOG.md` is **not** edited by this tranche; the
  proposed log entry text was delivered to the owner for appending.

No dependency was added to any manifest or lockfile: the generator and the
packaging entrypoint are Python stdlib only, and the Tauri bundler is
already provided by the existing `apps/desktop` toolchain. One byproduct:
`apps/desktop/src-tauri/Cargo.lock` was refreshed by the release build to
register the existing in-repo path crate `open_pipe_stress_curved_bend`
(the committed lockfile was stale relative to the crates' manifests at
HEAD); the delta contains no external package.

## Build Verification (what was actually proven)

All on this host (macOS arm64, Node v24.5.0, rustc 1.92.0, Python 3.13.7),
2026-07-10/11 UTC:

- `npm ci` at the tranche worktree: clean install, exit 0.
- `npm run tauri -- build` in `apps/desktop` (no `--bundles` flag): release
  compile finished and the bundler ran from config alone — "Finished 1
  bundle" at `apps/desktop/src-tauri/target/release/bundle/macos/OpenPipeStress Technical Preview.app`
  (14 MB), with `Contents/Resources/icon.icns` byte-identical to the
  committed icon and `CFBundleIconFile = icon.icns` in the bundle's
  `Info.plist`. Main executable `Contents/MacOS/openpipestress-desktop` is a
  thin `Mach-O 64-bit executable arm64` (the ruled matrix).
- Packaging shakedown (dirty tree, artifacts to a scratch directory, not
  committed): two `--execute` runs produced **byte-identical** 5.2 MB zips
  (sha256 `903efa2b…cb3b7`), a `shasum`-compatible `.sha256`, and a
  `-dirty` record with chain status `working_tree_dirty` and exit 1 — the
  honest non-verified outcome for a dirty tree.
- Zip round-trip boot check (TP-MAC-140 method): the `.app` unzipped from
  the artifact preserved the 0755 executable bit, launched, stayed alive
  for 6 s with zero bytes of stderr, and terminated cleanly on SIGTERM.
  Boot check only — not the human GUI journey (SMOKE checklist unchanged).
- Committed-state packaging run at clean HEAD with the commit-bound sweep:
  see Evidence.

## Checks

- `PYTHONDONTWRITEBYTECODE=1 python3 -m pytest -q tests` from the project
  root: 402 passed pre-commit (includes the 17 new focused tests).
- `npm run build:wasm:desktop` + `npm run test:desktop`: 407/407 tests,
  19 files, pass.
- `npm run build:desktop`: green (`tsc -b && vite build`).
- `PYTHONDONTWRITEBYTECODE=1 python3 tools/practitioner_harness/harness.py
  self-check` from the repo root: exit 0.
- DEC-025 five-surface sweep at committed clean HEAD
  (`python3 tools/release/run_evidence_sweep.py --execute`): see Evidence.

## Evidence

- DEC-025 sweep retry chain (each failed attempt committed before re-run;
  every suite passes standalone throughout; no product code changed between
  attempts 1-4):
  - Attempt 1 `validation/evidence/sweeps/SWEEP_20260711T024049Z_f940b5b7c1af.json`
    — `fail` at `desktop_vitest` (exit 1 under full sweep load immediately
    after the cargo surface; 407/407 standalone before and after).
  - Attempt 2 `validation/evidence/sweeps/SWEEP_20260711T024551Z_73e4b6f7c848.json`
    — `fail` at `desktop_playwright_e2e` (vitest passed this attempt;
    18/18 e2e standalone).
  - Attempt 3 `validation/evidence/sweeps/SWEEP_20260711T024949Z_3c12cc27b158.json`
    — `fail` at `desktop_playwright_e2e`: the vite webServer on the fixed
    e2e port 5174 disappeared mid-suite (first tests passed, the remaining
    16 saw `net::ERR_CONNECTION_REFUSED` at `127.0.0.1:5174`) —
    port-collision-class interference from concurrent agent worktrees
    sharing the fixed port; no listener or orphan remained afterward.
    Waited ~90 s per the retry practice before attempt 4.
  - Attempt 4 (passing) —
    `validation/evidence/sweeps/SWEEP_20260711T025412Z_0b9944768560.json`,
    bound to clean commit `0b9944768560f096b04935051176fba5b4ff9d1f`,
    `overall_status=pass` on all five surfaces.
- The first committed-state packaging run then exposed a chain-check bug
  (`git diff --name-only` returns repository-root-relative paths, so the
  evidence-only-delta check needed `--relative` inside the monorepo); the
  fix landed as its own commit and a fresh sweep passed at that HEAD:
  `validation/evidence/sweeps/SWEEP_20260711T025817Z_3115a08cdfee.json`,
  bound to clean commit `3115a08cdfeedad3525721bc7dc22eae3a00092e`,
  `overall_status=pass` on all five surfaces.
- First §8 release-artifact record (committed state, chain `verified`):
  `validation/evidence/release_artifacts/RELEASE_ARTIFACT_20260711T030046Z_8e436704b52b.json`
  — zip `dist/release/OpenPipeStress-Technical-Preview_0.1.0_aarch64-apple-darwin.app.zip`
  (untracked by design), sha256
  `569d6fa974b45c57f095fa41de7e3f0f5020563cd58c7e43f93c4d7c668b57ef`,
  authenticity chain `verified` against the clean sweep above with the
  record commit trailing the sweep commit only by the evidence-only
  closeout commit (recorded path-by-path in the record). The `.app` it
  packaged was built by the release-compile verification run above; every
  commit after the implementation commit differs only by evidence files
  and the chain-check `--relative` repair (TP-MAC-141 provenance pattern).

## Boundary Review

- Packaging mechanics only (F-PIP-2): the tranche produces a local zip,
  checksum, and record. Nothing was tagged, published, uploaded, signed,
  or notarized; no GitHub Release or public repository exists or was
  created; no release-readiness, professional, certification, sealing,
  authentication, or code-compliance claim is made by this tranche or its
  artifacts.
- D-06b (signing/notarization re-decision) remains `NOT_PREPARED`; this
  tranche neither prepared its packet nor advanced it.
- The `DEC-025` five-surface gate is untouched (`run_evidence_sweep.py`
  unmodified, guarded by test); the packaging entrypoint sits beside it.
- The icon mark is original invented content generated by a committed
  pure-stdlib script; no third-party art, fonts, or trademarked material.
- No lifecycle transition and no `_STATUS.md` change.
