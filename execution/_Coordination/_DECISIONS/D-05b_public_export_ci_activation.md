# D-05b - Public Sanitized-Export Repo CI Activation (Deferred From DEC-025)

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

Decide the disposition of the hosted-CI follow-up that the `D-05` Option D
ruling (`DEC-025`, 2026-06-11) explicitly deferred to this packet: whether to
activate hosted continuous-evidence CI on a **public sanitized-export
OpenPipeStress repository** now, activate it conditionally at the first
public publication event behind named prerequisites, redirect hosted CI to
the private monorepo instead, or keep the deferral open with no named
condition. Includes: the CI provider for the hosted lane; what the
sanitize-and-export pipeline must guarantee before any activation; and what
evidence the hosted lane adds over the `DEC-025` five-surface local sweep.

Register row `D-05b`
(`projects/chirality-piping/execution/_Coordination/_DECISIONS/_REGISTER.md:44`):
"Public sanitized-export repo CI activation (deferred from the D-05 Option D
ruling, `DEC-025`; prepare with D-06)" — blocks **Phase E hosted continuous
evidence** (completion plan E5).

**Out of scope:** the release matrix, installer formats, signing/notarization,
and publication targets (`D-06`, register row `:33` — shared surface
cross-cited in §6, not decided here); the protected-content scan owner and
procedure (`D-20`, register row `:53` — cross-cited in §5, not absorbed
here); maintainer quorum / release authority (`D-07`, ruled `DEC-027`); any
merge-gate change for the private monorepo (the `DEC-025` local sweep remains
the merge gate under every option below); any release, publication, or
milestone claim.

All paths below are relative to `projects/chirality-piping/` unless prefixed
`{REPO_ROOT}/` (the private monorepo root).

## 2. Verified Facts (Checked Cold, 2026-07-04)

| Check | Result |
|---|---|
| The deferral is explicit and named | `execution/_Decomposition/SOFTWARE_DECOMP.md:603` (`DEC-025`): "hosted CI is re-decided at a named follow-up (D-05b — public sanitized-export CI activation, prepared with D-06 at the Phase E lead-up); GitHub Actions on the private monorepo remains prohibited absent an explicitly recorded `docs/BUILD_AND_RELEASE.md` §7 private-data-handling authorization." |
| What "public sanitized-export repo" means in the parent packet | `execution/_Coordination/_DECISIONS/D-05_ci_provider_workflow.md:94` (Option B): "Create the (currently nonexistent, §2.4) public OpenPipeStress export with an export profile mirroring `exports/chirality-app/` discipline; hosted CI runs the five surfaces against the public package only. The private canonical tree stays local-evidence-only." The D-05 recommendation named Option B as the suggested D-05b Phase E activation (`D-05_ci_provider_workflow.md:117,120`). |
| Five-surface local sweep today | `docs/BUILD_AND_RELEASE.md` §5.1 (:121-167): deterministic entrypoint `tools/release/run_evidence_sweep.py` runs, sequentially in F-4-safe order: (1) Rust crate sweep (`check_release_readiness.py --profile cargo --execute`), (2) Python pytest, (3) desktop Vitest with the wasm engine built first, (4) Playwright e2e, (5) desktop production build; each execute run writes `validation/evidence/sweeps/SWEEP_<utc>_<commit12>[-dirty].json` bound to the commit hash. Script self-describes as "the deterministic local merge gate" (`tools/release/run_evidence_sweep.py:2-11`). |
| Sweep practice is live | `validation/evidence/sweeps/` holds 236 summary artifacts at check time; the latest clean-HEAD sweep `SWEEP_20260623T051552Z_16cca07f3b64.json` is the one accepted in the `DEC-054` conditional R4 gate (`execution/_Decomposition/SOFTWARE_DECOMP.md:632`). |
| F-4 atomic-build rider: discharged | `docs/BUILD_AND_RELEASE.md:69-71`: the wasm build "writes the glue to a sibling temp directory and renames it into place, so a concurrent reader never sees a half-written artifact set (`DEC-025` F-4 rider)." The rider is implemented, not pending. |
| No OpenPipeStress export mechanism exists | The only export profile in the monorepo is `{REPO_ROOT}/exports/chirality-app/export_public.py`, and its `SKIP_DIRS` excludes `projects` and `domains` entirely (`export_public.py:38-63`, members at `:60-61`); its boundary check hard-forbids `projects` at top level (`forbidden_top`, `export_public.py:204-208`). It cannot produce a piping tree. `{REPO_ROOT}/exports/` contains only `README.md` and `chirality-app/`. |
| No public OpenPipeStress repo exists | `{REPO_ROOT}/exports/README.md:5` states the real public repository remains the sibling `chirality-app` checkout outside this monorepo (named there by its local machine path) — a public surface for the tier-0 chirality-app, not for OpenPipeStress. No piping export profile, public remote, or mirror config was found anywhere in the monorepo (search 2026-07-04). |
| GitHub is the incumbent hosting | Monorepo git config (`{REPO_ROOT}/.git/config`, local state checked cold): single remote `origin = https://www.github.com/sgttomas/chirality`, matching the D-05 §2.4 FACT (`D-05_ci_provider_workflow.md:61`). `{REPO_ROOT}/.github/workflows/` holds three GitHub Actions workflows (`desktop-release-template.yml`, `governance-harness.yml`, `harness-premerge.yml`); a grep for "piping" across them returns nothing — none reference this project. |
| Hosted-data prohibition | `docs/BUILD_AND_RELEASE.md:202-205` (§7): "Hosted CI must not receive private project data, private rule packs, private material/component libraries, protected standards content, signing secrets, or publishing credentials unless a later security and release-governance decision explicitly authorizes that handling." §7 also requires future provider workflows to map to the provider-neutral phases with the five-surface sweep as the command basis "so local and hosted evidence stay comparable" (:189-192). |
| Protected-content boundary | `docs/IP_AND_DATA_BOUNDARY.md` §3 (:44-53): the public repository must not contain standards-body text/tables/figures, copied material-allowable or SIF/flexibility tables, protected dimensional tables, proprietary catalogs, or user private rule packs / owner design bases. `docs/CONTRACT.md:23` (`OPS-K-IP-1`) and `:41` (`OPS-K-PRIV-1`) bind the same boundary; `OPS-K-PRIV-1` names "Public-commit CI checks" as an enforcement surface. |
| Runtime-residency relaxation does NOT relax public commits | `DEC-051` / D-24: the open-residency ruling relaxed the runtime agent/model-provider channel only, "preserving the public-commit / third-party-IP boundary" (`_REGISTER.md:57`; `D-24_runtime_residency_reconciliation.md:59,160`; `docs/IP_AND_DATA_BOUNDARY.md:91` §6.1: "Third-party copyrighted standards content remains barred from the public repository regardless of this ruling"). |
| Release-gap anchors | `execution/_Aggregation/TP-RELEASE-GAP-REGISTER-REFRESH-001_2026-05-31/Gap_Disposition_Register.csv` row 4 (`RGAP-003`): CI/workflow/release-matrix/signing/publishing decisions "remain unresolved", `BLOCKER_TO_RELEASE_CLAIM`; row 6 (`RGAP-005`): "Release artifact scan is not recorded" — no legal/protected-data release gate exists yet. |
| Plan coupling | `plans/PLAN_2026-06-17_prd_completion.md:95` (row D-05b: "With D-06"); `:203` (E5 "CI + release implementation … public sanitized-export CI activation" gated on `D-06` and `D-05b`); `:298` ("E5 (CI/release) gates on `D-06`/`D-05b`; E7 (protected-content scan) gates on `D-20`"). Current target stage is PRD R5 per `DEC-054` (`SOFTWARE_DECOMP.md:632`) — the "Phase E lead-up" named in `DEC-025` is now. |
| Release-machinery residuals stand | `DEC-056` (D-21 ruling) carries the v0.1 R5 release-machinery deliverables (signed releases, issue templates, redaction workflow, IP contribution process) as explicit R6-entry residuals (`D-21_prd_scope_change_v0_2_milestone_set.md:182-192`; `SOFTWARE_DECOMP.md:634`) — the publication machinery this CI lane would eventually serve is itself still open work under `D-06`. |

**Unresolved (not asserted):** whether the three root `.github/workflows/`
files currently execute successfully on the remote (no run evidence in-repo;
carried TBD from `D-05_ci_provider_workflow.md:63`); hosted-runner
capabilities, images, Chrome availability, and cost multipliers (no network
calls were made; carried ASSUMPTION/TBD from the parent packet, e.g.
`D-05_ci_provider_workflow.md:76,87`); whether Playwright budgets measured on
maintainer macOS hardware transfer to hosted runners
(`D-05_ci_provider_workflow.md:51,74`).

## 3. Context: What DEC-025 Left For This Packet

`DEC-025` codified the five-surface local sweep as the sole commit-bound
merge gate and deferred hosted CI wholesale (§2 row 1). Three consequences
frame D-05b:

1. **The local sweep is working.** 236 recorded sweep artifacts, a clean-HEAD
   sweep accepted into the R4 exit ruling (`DEC-054`), and the F-4 rider
   implemented (§2). Nothing in Phase E work is currently blocked by the
   *absence* of a hosted lane except the E5 activation item itself.
2. **The private-monorepo path stays closed by default.** `DEC-025` records
   that GitHub Actions on the private monorepo "remains prohibited absent an
   explicitly recorded `docs/BUILD_AND_RELEASE.md` §7 private-data-handling
   authorization" (`SOFTWARE_DECOMP.md:603`). Any option that hosts the
   private canonical tree must carry that authorization in the ruling text.
3. **The named landing zone was Option B.** The parent packet's
   recommendation reserved this follow-up for hosted CI **on the public
   sanitized export**, "where it costs no IP exposure"
   (`D-05_ci_provider_workflow.md:117-124`).

## 4. Prerequisite Reality: The Export Mechanism Does Not Exist

This is the load-bearing fact of the packet (§2, rows "No OpenPipeStress
export mechanism exists" / "No public OpenPipeStress repo exists"):
**there is nothing to activate CI on today.** No sanitized-export pipeline,
no export profile, no staging tree, no public remote, and no public
OpenPipeStress repository exist. The only export tooling in the monorepo
(`{REPO_ROOT}/exports/chirality-app/export_public.py`) is scoped to a
different product and structurally excludes `projects/**`.

Any activation option must therefore sequence the export mechanism as an
explicit prerequisite work item, not assume it. Mirroring the
`exports/chirality-app/` discipline (the pattern the parent packet named)
means, concretely: a deterministic staging build, a `path/size/sha256`
export manifest, a generated export report, and a mechanical
`boundary_findings`-style forbidden-content check
(`export_public.py:16-18,202-215`). The include/exclude set for an
OpenPipeStress profile (which parts of `core/`, `apps/`, `tools/`, `docs/`,
schemas, and tests ship publicly; whether `execution/` governance internals
ship at all) is unresolved design work inside that prerequisite item — it is
not decided by this packet.

### 4.1 What the sanitize-and-export pipeline must guarantee before any activation

Derived from the governing surfaces cited in §2 (no new policy is invented
here):

| # | Guarantee | Governing basis |
|---|---|---|
| G1 | The staged tree contains none of the `IP_AND_DATA_BOUNDARY` §3 prohibited content (standards text/tables, copied allowables, SIF/flexibility tables, protected dimensional tables, proprietary catalogs, private rule packs / owner design bases) | `docs/IP_AND_DATA_BOUNDARY.md:44-53`; `docs/CONTRACT.md:23,41` |
| G2 | A recorded protected-content scan of the staged export passes under the `D-20`-ruled owner + procedure before any public push (`RGAP-005` closure surface) | `Gap_Disposition_Register.csv` row 6; register row `D-20` (`_REGISTER.md:53`) |
| G3 | Deterministic, manifested staging: every exported file hashed; the export bound to the canonical private-tree commit it was cut from, so hosted evidence traces to a private revision | `docs/BUILD_AND_RELEASE.md` §4 (:85-103); `export_public.py` manifest pattern (:16-18) |
| G4 | A mechanical forbidden-path/forbidden-content boundary check runs in the pipeline itself (not only human review) | `export_public.py:202-215` pattern; `OPS-K-PRIV-1` "Public-commit CI checks" (`docs/CONTRACT.md:41`) |
| G5 | The hosted lane holds no signing secrets and no publishing credentials; it is evidence-only | `docs/BUILD_AND_RELEASE.md:202-205` |
| G6 | Hosted jobs map to the provider-neutral §7 phases with the five-surface sweep as the command basis, so local and hosted evidence stay comparable | `docs/BUILD_AND_RELEASE.md:187-200` |
| G7 | Export↔canonical drift is a managed failure mode: each public push is cut from a named canonical commit; stale exports are detectable via G3 | `D-05_ci_provider_workflow.md:97` (Option B "Against") |

## 5. Constraint: Protected-Content Posture (Cross-Cite D-20, Not Absorbed)

The project holds a hard no-protected-standards-data posture on every public
surface (`RGAP-005`; `OPS-K-IP-1`; `IP_AND_DATA_BOUNDARY` §3). Register row
`D-20` (`_REGISTER.md:53`) owns the release-artifact protected-content scan
owner + recorded procedure and is being prepared in this same eight-row
tranche (expected packet `D-20_protected_content_scan.md`). This packet does
**not** decide the scan owner or procedure; it binds to whatever `D-20`
rules by making a green recorded scan of the staged export tree a hard
precondition of any public push (G2, §4.1).

The `DEC-051` open-residency ruling changes nothing here: it relaxed the
*runtime* agent/model-provider channel only and explicitly preserved the
public-commit / third-party-IP boundary (§2 row "Runtime-residency
relaxation"). A public sanitized-export repo is exactly the surface that
boundary governs, and the owner cannot waive third parties' rights by
configuration (`docs/IP_AND_DATA_BOUNDARY.md:91`).

## 6. Coupling To D-06 (Shared Surface, Not Decided Here)

The register orders this packet "prepare with D-06" (`_REGISTER.md:44`), and
the completion plan gates E5 on both rows (`PLAN_2026-06-17_prd_completion.md:203`).
The shared surface is concrete:

- **Public repo location and naming** — the sanitized-export repo is
  presumptively also the publication target for release artifacts; `D-06`
  owns publication targets, and `DEC-028` already routed container naming
  "with D-06" (`_REGISTER.md:36`).
- **Provider** — a CI provider chosen here becomes the natural host for the
  `D-06` release/packaging workflows (the tag-triggered
  `desktop-release-template.yml` pattern already exists at the monorepo
  root, §2).
- **OS matrix** — the parent packet deferred any macOS lane decision toward
  D-06 packaging (`D-05_ci_provider_workflow.md:121,135`).

`D-06` is `NOT_PREPARED` in the register at check time (`_REGISTER.md:33`)
and is being prepared in this same tranche (expected packet
`D-06_release_matrix_installers_publication.md`). This packet takes no
position on the release matrix, installers, signing, or publication targets;
it only requires (O-B condition 3, §8) that the public-repo location/naming
be fixed by or jointly with the `D-06` ruling before first publication, so
the CI lane does not have to move.

## 7. What The Hosted Lane Adds Over The Local Sweep

Honest accounting, from the parent packet's own analysis
(`D-05_ci_provider_workflow.md:96-97,110-111`):

- **Independent execution environment** — catches works-on-my-machine drift;
  adds a Linux signal to a currently macOS-only, single-machine evidence
  monoculture.
- **Proves the artifact external engineers actually receive** — the local
  sweep proves the private canonical tree; hosted CI on the export proves
  the public package, which is the reproducibility posture the R6 exit
  language targets ("external reviewers can reproduce validation examples",
  `D-21_prd_scope_change_v0_2_milestone_set.md:61`).
- **Continuity independent of agent/human discipline** — the local sweep "is
  only as continuous as agent/human discipline — an un-run sweep is a silent
  gap" (`D-05_ci_provider_workflow.md:111`); a hosted lane runs on every
  push to the public repo mechanically.
- **RGAP-003 progress** — a standing hosted-evidence lane closes part of the
  open release/CI-authority residual (the rest stays with `D-06`/`D-07`).

What it does **not** add: a merge gate for private-monorepo agent branches.
Evidence on the export arrives only after an export is cut; day-to-day
merges keep the `DEC-025` local sweep under every option below
(`D-05_ci_provider_workflow.md:97`).

## 8. Options

| ID | Option | Consequence |
|---|---|---|
| **O-A** | **Activate now:** stand up the export pipeline, public repo, and hosted CI as the immediate next tranche, ahead of the `D-06` ruling. | Fastest hosted signal, but forces the public-repo location/naming decision that is `D-06` shared surface (§6) before `D-06` rules — re-work risk if publication targets land elsewhere; still cannot legally push publicly before a `D-20`-ruled scan passes (G2), so the true critical path is identical to O-B with the sequencing risk added. |
| **O-B** | **Conditional activation (provider + location decided now; switch-on gated):** rule that the hosted lane is **GitHub Actions on the public sanitized-export OpenPipeStress repository**, activated at the first public publication event, behind three named prerequisites: (1) the OpenPipeStress sanitized-export pipeline exists and meets G1-G7 (§4.1) — a new, explicitly scheduled Phase E work item; (2) a recorded `D-20`-ruled protected-content scan of the staged export is green before any public push; (3) the public-repo location/naming is fixed by or jointly with the `D-06` ruling. Until switch-on, `DEC-025` local-sweep posture continues unchanged. | Ends the provider/location TBD now (E5 becomes plannable, RGAP-003 partially dispositioned) without inventing a repo before `D-06`/`D-20` rule; zero private data on hosted surfaces by construction; the hosted lane proves the externally received tree (§7). Cost: no hosted signal until the export pipeline exists — accepted, since no public artifact exists to prove before then. |
| **O-C** | **Redirect: activate GitHub Actions on the private monorepo instead** (parent packet Option A), path-filtered to `projects/chirality-piping/**`. | Immediate hosted signal on the canonical tree and true per-branch gating — but requires the ruling to record the explicit `BUILD_AND_RELEASE.md` §7 private-data-handling authorization that `DEC-025` made a standing prohibition (§3); exposes private project/domain content to runner VMs and third-party actions; proves the private tree, not what external engineers receive; leaves the public-export lane undecided anyway. |
| **O-D** | **Keep deferred, no named condition:** leave the row open and re-decide later. | No new work now; but E5 stays unplannable, the RGAP-003 CI residual stays fully open with no activation path, and the register carries a permanently reopening row — strictly worse than O-B, which also spends nothing until prerequisites exist. |

## 9. Recommended Disposition (PROPOSAL)

Recommend **O-B**.

Rationale:

1. **It is the disposition the parent packet reserved this slot for.** The
   D-05 recommendation named "Option B … as the Phase E activation
   (suggested D-05b)" with hosted evidence bought "where it costs no IP
   exposure" (`D-05_ci_provider_workflow.md:117-124`); `DEC-025` accepted
   that structure (`SOFTWARE_DECOMP.md:603`).
2. **GitHub Actions is the evidenced null hypothesis, not a preference.**
   The monorepo's single remote is GitHub and three GitHub Actions workflows
   already exist at the root (§2); no other provider has any footprint in
   the repo (`D-05_ci_provider_workflow.md:64`). Choosing it introduces no
   new hosting party.
3. **The sequencing is honest.** The export mechanism does not exist (§4);
   the scan gate (`D-20`) and the publication target (`D-06`) are open rows
   in this same tranche. O-B decides what only this row can decide
   (provider, location-class, activation condition, pipeline guarantees
   G1-G7) and leaves each prerequisite with its owner.
4. **Nothing is lost in the interim.** The `DEC-025` local sweep is the
   working merge gate with the F-4 rider already implemented (§2); the
   hosted lane's marginal evidence (§7) only becomes real once a public
   artifact exists to prove.

If parallel-branch volume outgrows the local sweep before the export exists,
the escalation path remains the parent packet's Option A **only** with the
explicitly recorded §7 authorization — that escalation would be a new
decision packet, not an interpretation of this one.

This recommendation is a PROPOSAL only. It confers no authority, activates
no CI, creates no public repository, and changes no state.

## 10. Human Ruling And Disposition

**Ruling recorded:** _Awaiting owner ruling._

## 11. Ruling Mechanism

Per existing practice, the human project authority selects an option or rules
directly. The ruling is appended to
`execution/_Decomposition/SOFTWARE_DECOMP.md` §12 as the next `DEC` entry
(the table ends at `DEC-056`, `SOFTWARE_DECOMP.md:634`, as of 2026-07-04;
re-check at recording time) citing this packet; the register row `D-05b`
(`_REGISTER.md:44`) then moves from `AWAITING_RULING` to `RULED` with the
decision pointer. If O-C is selected, the `BUILD_AND_RELEASE.md` §7
private-data-handling authorization must be part of the recorded decision
text, not implied (`D-05_ci_provider_workflow.md:151`). This packet does not
edit the register, the decomposition, `docs/BUILD_AND_RELEASE.md`, or any
export tooling.
