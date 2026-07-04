# D-06 - Release Matrix, Installer Formats, Signing/Notarization, Publication Targets (RGAP-003/006)

**Date prepared:** 2026-07-04
**Prepared by:** bridge work loop agent at owner direction (Ryan Tufts,
2026-07-04) preparing all eight open register rows.
**Epistemic status:** PROPOSAL (non-governing). Only the human project
authority rules. Nothing here changes lifecycle state, issues deliverables,
creates release readiness, professional approval, certification, sealing,
authentication, code-compliance acceptance, or asserts that any PRD milestone
is met until the human records the ruling.

---

## 1. Decision Statement And Scope

Decide the v0.1 **release matrix** (which OS/architecture targets a packaged
OpenPipeStress build is produced for), the **installer/package format per
platform**, the **code-signing/notarization posture** (including the realistic
no-signing/self-distribution posture available to a sole maintainer under
`DEC-027`), and the **publication targets/channels** through which release
artifacts reach external engineers. This is the register row D-06 decision
(`execution/_Coordination/_DECISIONS/_REGISTER.md:33`), blocking Phase E
packaging (plan item E5), and it carries the release-matrix /
installer / signing / publication residue of gap rows RGAP-003 and RGAP-006.

Also in scope, per the `DEC-028` ruling text ("extension/document-kind naming
lands with the D-06 packaging decision",
`execution/_Decomposition/SOFTWARE_DECOMP.md:606`): the invented file
extension and document-kind name for the ruled multi-member-archive project
container.

**Out of scope:**

- **D-05b** (public sanitized-export repo CI activation). The register row
  says "prepare with D-06" (`_REGISTER.md:44`) and `DEC-025` names it the
  hosted-CI re-decision point (`SOFTWARE_DECOMP.md:603`). D-05b receives its
  own packet in this same preparation tranche; this packet cross-cites the
  coupling — the publication *channel* chosen here and the D-05b public-export
  repo are naturally the same surface — but does not absorb D-05b's CI-activation
  scope.
- CI provider/workflow location (ruled, `DEC-025`); maintainer quorum and
  release authority (ruled, `DEC-027`); the container's logical format
  (ruled, `DEC-028` — only its *naming* lands here).
- The release-time protected-content scan owner/procedure (D-20, its own row),
  the PDF emitter (D-10b), the headless runner CLI (plan E1), and release-label
  vocabulary (PB-TBD-003, plan E8 — human-gated separately).
- Any release **claim**. Ruling D-06 selects targets and posture; an actual
  release additionally requires the E5 implementation tranche, the D-20 scan
  record, gate records, and the human release authority's acceptance
  (`docs/BUILD_AND_RELEASE.md` §6/§8).

## 2. Verified Facts (Checked Cold, 2026-07-04)

All paths are relative to `projects/chirality-piping/` unless prefixed
`<monorepo>/`.

| Check | Result |
|---|---|
| RGAP-003 text | "CI provider, workflow path, **release matrix**, thresholds, **signing**, attestation, and **publishing** decisions remain unresolved" — `BLOCKER_TO_RELEASE_CLAIM`, owner PKG-09/PKG-10; remedial: "Run a bounded release/CI authority planning tranche before any release claim" (`execution/_Aggregation/TP-RELEASE-GAP-REGISTER-REFRESH-001_2026-05-31/Gap_Disposition_Register.csv:4`). |
| RGAP-006 text | "Runnable product target and runtime assembly were unresolved in the May 11 audit … does not prove full runtime assembly, **installer packaging**, live solver integration, or **product release target selection**" — `BLOCKER_TO_PRODUCT_RELEASE_CLAIM`, owner PKG-07/PKG-10 (`Gap_Disposition_Register.csv:7`). |
| Decision origin | Completion plan §2.2 row D-06: "Release matrix, installer formats, signing/notarization, publication targets (RGAP-003/006)", blocks "Phase E packaging (E5)", timing "At E5 lead-up" (`plans/PLAN_2026-06-17_prd_completion.md:90`); §3 Phase E row E5 names "release matrix, installer formats, signing/notarization/attestation/publication; public sanitized-export CI activation; signed releases" gated on `D-06`/`D-05b` (`:203`); "E5 (CI/release) gates on `D-06`/`D-05b`" (`:298`). |
| Build system | Tauri 2 desktop shell: `apps/desktop/src-tauri/Cargo.toml:26` (`tauri = { version = "2" }`); `docs/BUILD_AND_RELEASE.md:51-55` (root npm workspace + Tauri 2 shell; Rust crates crate-local, no root Cargo workspace). App identity: `productName: "OpenPipeStress Technical Preview"`, `version: "0.1.0"`, `identifier: "org.openpipestress.technical-preview"` (`apps/desktop/src-tauri/tauri.conf.json:3-5`). |
| Bundling is OFF today | `tauri.conf.json:26-29`: `"bundle": { "active": false, "icon": [] }` — no bundle targets configured, no bundle icons wired; `apps/desktop/src-tauri/icons/` contains only `icon.png` (no `.icns`/`.ico`). |
| Only proven packaged path (macOS) | `apps/desktop/SMOKE.md` TP-MAC-141 binary provenance (`:4185-4192`): `npm run tauri -- build --bundles app` — "no bundle targets are configured in `tauri.conf.json`, so the explicit `--bundles app` flag is required to produce the `.app` (12 MB)"; bundle path `apps/desktop/src-tauri/target/release/bundle/macos/OpenPipeStress Technical Preview.app`. Debug boot smoke via `tauri build --debug --no-bundle` (TP-MAC-140, `:4159-4163`). |
| Packaged-run evidence status | TP-MAC-141 human packaged-smoke attempts 1–3 (`SMOKE.md:4268-4330`): attempt 1 FAIL (wasm assets not shipped; repaired by TP-MAC-144, `:4527+` — `beforeBuildCommand` now chains the wasm build, `tauri.conf.json:9`); attempts 2–3 booted and partially passed; the full A12 journey was never completed (usability finding carried per `DEC-035`). No Windows or Linux packaged-build or packaged-run evidence record exists anywhere in `SMOKE.md` or `plans/` (grep for platform terms at HEAD finds none). |
| Standing TBDs this decision closes | `docs/BUILD_AND_RELEASE.md` §2: "no installer, signed binary, notarized package, attestation, or publication target is generated; no final OS/architecture release matrix is selected" (`:41-43`); §9: "TBD: final supported OS/architecture release matrix. TBD: installer/package formats. TBD: signing, notarization, checksum publication, and release attestation" (`:229-231`). `docs/RELEASE_QUALITY_GATES.md` §10: "TBD: release matrix, signing, and release attestation (D-06)" (`:165`). |
| Packaging guidance already recorded | `docs/BUILD_AND_RELEASE.md` §6 (`:169-185`): the packaging skeleton is "a checklist, not a package build"; if binaries/installers are produced, record "package path, target, build command, checksum, signing/notarization state, and publication state" (`:179-180`); "Desktop packaging is expected to follow the accepted Tauri-supported macOS/Windows/Linux architecture baseline when a GUI package exists. Exact target triples, installer formats, signing identities, notarization process, and publication destinations remain `TBD`" (`:182-185`). |
| Secrets/data boundary | `docs/BUILD_AND_RELEASE.md` §7 (`:202-205`): hosted CI must not receive "signing secrets, or publishing credentials unless a later security and release-governance decision explicitly authorizes that handling". |
| PRD requirements | §22.6 R5 "Engineering Beta" deliverables include "Signed releases" (`docs/PRD.md:1267`); exit criteria: "External engineers can reproduce validation examples" and "Public repository contains no known protected standards data" (`:1271-1272`). §19.2 selection criterion: "Ease of packaging for Windows, Linux, and macOS" (`:1124`). §23.1 metric: "Reproducibility of reports across platforms" (`:1285`). |
| License posture | `LICENSE.md:3`: `SPDX-License-Identifier: PolyForm-Noncommercial-1.0.0`. (The PRD's open license question at `docs/PRD.md:1324` predates this file; the PRD also describes the product as "free and open-source", `:33` — the reconciliation of that prose is not a D-06 matter, but publication-channel terms-of-service reasoning below uses the LICENSE.md fact.) |
| Publication surfaces that exist today | None public. The project lives inside the private monorepo (single remote `github.com/sgttomas/chirality`, recorded in `D-05_ci_provider_workflow.md` §2.4); the "public OpenPipeStress repository" of `docs/IP_AND_DATA_BOUNDARY.md:15` is prospective; the existing monorepo public-export tool excludes this project entirely (`<monorepo>/exports/chirality-app/export_public.py:38,60-61` — `SKIP_DIRS` contains `projects` and `domains`; `:204` forbids them at top level). No OpenPipeStress export profile exists. |
| Prior rulings bounding this one | `DEC-025` (`SOFTWARE_DECOMP.md:603`): hosted CI deferred; the five-surface local sweep is the commit-bound merge gate; GitHub Actions on the private monorepo prohibited absent an explicit §7 authorization; D-05b prepared with D-06. `DEC-027` (`:605`): sole maintainer is the sole release authority; external contributions closed. `DEC-028` (`:606`): project package = multi-member archive per PKG-17 contracts; evidence binds to canonical members + manifest per-member JCS hashes, not container bytes; naming lands with D-06. |
| Maintainer/dev platform | All packaged evidence is macOS (`SMOKE.md` TP-MAC-* series; `.app` bundle path above; TP-MAC-140 notes `tauri-driver` "does not support macOS", `:4165-4166`, i.e. the recorded runs were on macOS hardware). Playwright budgets were "measured on the maintainer's macOS hardware" (`D-05_ci_provider_workflow.md` §2.2). |

### 2.1 Unresolved / Not Verifiable Offline

Per K-INVENT-1 these are recorded, not asserted; none is load-bearing for the
option set, and each is labeled where used:

- **U-1 (Tauri bundler format coverage).** General knowledge says the Tauri 2
  bundler emits `app`/`dmg` on macOS, `msi`/`nsis` on Windows, and
  `deb`/`rpm`/`appimage` on Linux, and does **not** cross-compile installers
  across OS hosts. No network check was made and only `--bundles app` on macOS
  is proven in-repo (`SMOKE.md:4187-4192`). ASSUMPTION.
- **U-2 (Unsigned-install friction).** General knowledge: unsigned apps
  trigger macOS Gatekeeper quarantine (user must explicitly open/de-quarantine)
  and Windows SmartScreen warnings; Apple notarization requires a paid Apple
  Developer Program membership; Windows Authenticode requires a purchased
  certificate. Magnitudes/costs unverified offline. ASSUMPTION.
- **U-3 (Channel terms).** Whether a given hosting channel's terms are
  compatible with PolyForm-Noncommercial distribution was not verified
  offline. GitHub distribution of source-available/noncommercial-licensed
  code is commonplace (ASSUMPTION); a human check rides any publication
  activation.
- **U-4 (Remote topology currency).** The single-private-remote topology is
  cited from `D-05_ci_provider_workflow.md` §2.4 (2026-06-11) and was not
  re-verified today (this packet ran no git commands); the export-tool
  exclusion of `projects/` was re-verified cold.

## 3. Constraints Carried Into This Decision

1. **Sole maintainer (`DEC-027`).** There is exactly one human release
   authority and no contributor pipeline. Any signing identity is a personal
   enrollment/purchase by the owner; any signing step is a manual local step.
   A posture that only works with an org-grade release team is out of reach by
   ruling.
2. **Local-only evidence machinery (`DEC-025`).** Hosted CI is deferred and
   private-monorepo hosted workflows are prohibited absent a recorded §7
   authorization. Consequence: v0.1 release artifacts are **built locally on
   maintainer hardware**; a multi-OS matrix implies maintainer-managed VMs or
   additional hardware for each added OS (U-1: installers do not
   cross-compile), or waiting for D-05b public-export CI to exist and be
   authorized for artifact builds.
3. **Signing secrets stay local (`docs/BUILD_AND_RELEASE.md:202-205`).**
   Whatever signing posture is chosen, signing identities/credentials do not
   go to hosted surfaces under current rulings.
4. **Evidence discipline.** Every packaged claim so far is bound to recorded
   smoke/sweep evidence (`SMOKE.md` TP-MAC-140/141/144; `DEC-025` sweep
   artifacts). A matrix entry with no packaged-run evidence would be the first
   evidence-free shipping surface in the project — the D-26/D-27 pattern
   (evidence first, advancement second) argues against it.
5. **Container naming lands here (`DEC-028`).** The multi-member archive
   project package needs its invented extension/document-kind string ruled so
   Phase E file-type registration and PKG-17 implementation tranches have a
   name to build against.
6. **PRD §22.6 "Signed releases" tension.** The R5 deliverable list includes
   "Signed releases" (`docs/PRD.md:1267`). A no-signing v0.1 posture does not
   erase that line: it either defers signing behind a named follow-up ruled
   before the R5 exit review, or the R5 gate records an explicit
   human-accepted deviation. Silence is not a disposition (the plan applies
   the same rule to D-12, `plans/PLAN_2026-06-17_prd_completion.md:205`).

## 4. Decision Axes

- **A. Release matrix:** macOS Apple Silicon only (the only evidenced
  platform) → macOS universal → + Windows x64 → + Linux x64. PRD §19.2 wants
  eventual all-three ease of packaging (`docs/PRD.md:1124`); nothing in the
  PRD dates that to v0.1.
- **B. Installer format per platform:** the proven `.app` bundle (zip- or
  dmg-wrapped) on macOS; Tauri-default `msi`/`nsis` (Windows) and
  `deb`/`rpm`/`appimage` (Linux) if those platforms enter the matrix (U-1).
- **C. Signing posture:** none (checksums + recorded provenance only) →
  Apple Developer ID signing + notarization → + Windows Authenticode.
- **D. Publication target:** none yet (artifacts recorded locally, handed out
  directly) → GitHub Releases on the prospective public sanitized-export
  repository (the D-05b surface) → hosted releases on the private monorepo
  (not externally reachable, so not a real publication channel).

## 5. Options

| ID | Option | Matrix | Formats | Signing | Publication | Consequence |
|---|---|---|---|---|---|---|
| **O-A** | **Evidenced-platform minimum; unsigned self-distribution** | macOS Apple Silicon (`aarch64-apple-darwin`) only | `.app` via the proven `--bundles app` path, shipped zipped; `dmg` optional bounded follow-on | None for v0.1: SHA-256 checksums published beside artifacts, bound to a clean-HEAD `DEC-025` sweep + release artifact record; signing re-decided at named follow-up **D-06b** | GitHub Releases on the public sanitized-export repo once D-05b stands it up; direct hand-off until then | Cheapest true-to-evidence posture; zero secrets, zero enrollment; Gatekeeper friction for users (U-2) documented in release notes; PRD §22.6 "Signed releases" requires the D-06b ruling or an explicit R5-gate deviation record (§3 item 6); Windows/Linux users unserved in v0.1 |
| **O-B** | **Evidenced platform + paid Apple signing** | Same as O-A | Same as O-A (dmg preferred once signing exists) | Apple Developer ID + notarization before first publication (owner enrollment; signing stays a local manual step per §3 item 3) | Same as O-A | Removes Gatekeeper friction and satisfies "Signed releases" literally for the shipped platform; recurring cost + identity administration lands on the sole maintainer (U-2); notarization mechanics are new unverified machinery on the release path |
| **O-C** | **Full PRD-aspiration matrix now, unsigned** | macOS (arm64 + x86_64) + Windows x64 + Linux x64 | Tauri defaults per platform: `app`/`dmg`; `nsis` or `msi`; `appimage` + `deb` (U-1) | None (as O-A), on all platforms | Same channel as O-A | Matches PRD §19.2 breadth immediately; but two of three platforms would ship with zero packaged-run evidence (§3 item 4), and local-only builds (§3 item 2) force maintainer-managed Windows/Linux build+smoke environments before the first release — the largest new workload of any option |
| **O-D** | **Defer D-06 (no matrix now)** | — | — | — | — | E5 stays blocked (`plans/PLAN_2026-06-17_prd_completion.md:298`); RGAP-003/006 release-matrix residue stays open; `DEC-028` container naming stays unruled; the register row persists as AWAITING_RULING debt with no new information expected to arrive on its own |

Common to O-A/O-B/O-C (the naming rider, per `DEC-028`): the multi-member
archive project package takes an invented extension and document-kind string —
proposed **`.opsproj`**, document kind **"OpenPipeStress Project Package"**
(invented names, continuous with the D-09 packet's invented example; final
spelling is the owner's at ruling). OS file-type registration is bounded
implementation work after ruling, not part of the decision.

Matrix-expansion rider common to O-A/O-B: adding any OS/arch to the matrix
requires (i) a packaged build produced on/for that platform, (ii) a recorded
packaged-run smoke on real hardware or a maintainer-accepted VM, and (iii) a
release-artifact record row per `docs/BUILD_AND_RELEASE.md:179-180` — i.e.
matrix growth is evidence-gated, mirroring how TP-MAC-140/141/144 gated the
macOS path.

## 6. Recommended Disposition (PROPOSAL)

Recommend **O-A**, with the naming and matrix-expansion riders above and the
following non-binding shape:

> **v0.1 release matrix:** macOS Apple Silicon (`aarch64-apple-darwin`) only —
> the only target with packaged build + boot + partial human-smoke evidence
> (`SMOKE.md` TP-MAC-140/141/144). **Installer format:** the Tauri `.app`
> bundle produced by `npm run tauri -- build --bundles app`, distributed as a
> zip archive with a published SHA-256 checksum; enabling `bundle.active` with
> explicit targets in `tauri.conf.json` (and real `.icns` icons) is the E5
> implementation tranche. **Signing/notarization:** none for v0.1;
> authenticity is carried by the checksum + the commit-bound `DEC-025` sweep
> artifact + the `docs/BUILD_AND_RELEASE.md` §8 release artifact record, and
> release notes carry the unsigned-install caveat (U-2). Signing/notarization
> is re-decided at **D-06b** (new register row at the R5-exit lead-up), which
> must be ruled — or an explicit PRD §22.6 deviation recorded — before any R5
> "Signed releases" deliverable claim. **Publication target:** GitHub Releases
> on the prospective public sanitized-export OpenPipeStress repository, i.e.
> the same surface D-05b activates CI on; until that repo exists, artifacts
> are recorded locally per §8 and distributed directly by the owner. Windows
> and Linux enter the matrix only through the evidence-gated expansion rider.

Rationale:

1. **It is the only option whose every element is already evidenced.** The
   `.app` path is built, boot-smoked, wasm-packaging-repaired, and
   human-attempted in-repo (`SMOKE.md:4159-4192, 4268-4330, 4527+`); O-C ships
   evidence-free platforms and O-B adds unverified notarization machinery to
   the first release.
2. **It fits the ruled operating reality.** Sole maintainer (`DEC-027`),
   local-only builds and merge gate (`DEC-025`), secrets-stay-local
   (`BUILD_AND_RELEASE.md:202-205`): an unsigned, checksummed, locally built
   single-platform artifact is exactly what that machinery can produce today
   without new enrollments, hardware, or authorizations.
3. **It converts both gap rows from "unresolved" to "selected + scheduled".**
   RGAP-003's matrix/signing/publishing axes and RGAP-006's "installer
   packaging … product release target selection" get concrete selections,
   with the remaining work (E5 implementation, D-05b channel activation,
   D-06b signing) as named, registered follow-ups rather than an open-ended
   TBD.
4. **It keeps the PRD honest rather than quietly diluted.** The §22.6
   "Signed releases" line is confronted head-on via the D-06b-or-recorded-
   deviation mechanism (§3 item 6), preserving the project's
   silence-is-not-a-disposition norm.

O-B is the natural upgrade if the owner prefers to spend enrollment
cost/effort now to remove install friction on the only shipped platform; it
changes nothing else in the recommendation. O-C is not recommended at v0.1:
it maximizes surface without evidence. O-D merely defers a decision that has
no missing inputs.

This recommendation is a PROPOSAL only. It confers no authority, produces no
artifact, publishes nothing, and creates no release-readiness or milestone
claim.

## 7. Downstream Impact Map

| Surface | Impact of a ruling |
|---|---|
| Plan E5 (`PLAN_2026-06-17_prd_completion.md:203,298`) | Unblocks the CI + release implementation tranche's packaging half; E5's hosted-evidence half still awaits the D-05b ruling. |
| `docs/BUILD_AND_RELEASE.md` §2/§6/§9 | Matrix/installer/signing/publication TBDs (`:41-43, :182-185, :229-231`) close to the ruled selections; §8 release-artifact record gains the ruled fields' concrete values at first packaging run. |
| `docs/RELEASE_QUALITY_GATES.md` §10 | "TBD: release matrix, signing, and release attestation (D-06)" (`:165`) closes; attestation posture = checksum + sweep + record under O-A until D-06b. |
| RGAP-003 / RGAP-006 | Release-matrix/installer/signing/publishing axes move to selected-and-scheduled; final closure still needs E5 execution and, for any release claim, D-20 + gate records + human acceptance. |
| `DEC-028` container | Extension/document-kind naming discharges; PKG-17 implementation and OS file-type registration proceed as bounded tranches. |
| `tauri.conf.json` | E5 tranche flips `bundle.active`, pins the ruled targets, and supplies real bundle icons (`:26-29`; `icons/` currently `icon.png` only). |
| D-05b packet (this tranche) | Consumes the ruled publication target as the surface whose CI activation it decides; no scope overlap — D-06 picks *where artifacts are published*, D-05b picks *whether/how CI runs there*. |
| New register rows | O-A creates D-06b (signing/notarization re-decision at the R5-exit lead-up); matrix expansions, if ever proposed, arrive as new evidence-gated rows per the rider. |

## 8. Human Ruling And Disposition

**Ruling recorded:** _Awaiting owner ruling._

## 9. Ruling Mechanism

Per existing practice, the human project authority selects an option (with
any riders/edits) or rules directly. The ruling is appended to
`execution/_Decomposition/SOFTWARE_DECOMP.md` §12 as the next `DEC` entry
citing this packet; register row D-06 in
`execution/_Coordination/_DECISIONS/_REGISTER.md` then moves from
`AWAITING_RULING` to `RULED` with the decision pointer. Any follow-up rows
the ruling creates (e.g. D-06b) are appended to the register as
`NOT_PREPARED`. This packet does not edit the register, the decomposition,
`tauri.conf.json`, or any other file, and it does not resolve the decision.
