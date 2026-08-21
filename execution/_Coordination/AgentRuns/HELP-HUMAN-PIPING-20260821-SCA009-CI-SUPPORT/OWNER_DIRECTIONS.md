# Owner directions — durable chat transcription

Classification: `CHAT_TRANSCRIPTION — EVIDENCE, NOT RULING`.

## Original iteration steer

> Steer (this run): STANDING DIRECTION — Piping iteration steer (SCA-009 continuation + CI hardening). This iteration's shape is owner-directed; measure it against this direction, not the default Step-0 selection. N=4 is authorized for this iteration.
>
> Basis gate. Branch from the main that contains merge b1876a5e0 (PR #598). Your basis must show SOFTWARE_DECOMP revision 0.12, `_ScopeChange/_LATEST.md` at SCA-009, DEL-07-09 in the PKG-07 table, and the claimed-model-hash gate (`check_claimed_model_hash`) already present in the operation applier — Node 2 writes that same file on top of it. Read `execution/_Coordination/NOTICE_2026-08-20_PIPING_SCA-009_DEL-07-09_VOCABULARY_PALETTE.md` and the SCA-009 `Vocabulary_Annex.md` before selecting anything — the NORMATIVE-NOW coverage rows with their "Implementation lands in" column are the directed engineering backlog.
>
> Node 1 — SCA-009 instrument obligations (bounded, mechanical; do not let it swell). Execute the ruled downstream obligations per `execution/_ScopeChange/SCA-009_2026-08-20_0000/Propagation_Plan.md` and `Handoff_State.md`: PREPARATION scaffold for DEL-07-09; DAG-008 rebuild (topology changed: +DEL-07-09, +SOW-077, three additive edges DEL-07-09→{DEL-16-01, DEL-07-01, DEL-07-02}); Dependencies.csv extract for DEL-07-09; targeted RECONCILIATION current-authority refresh for DEL-07-09, DEL-07-03, and the boundary-adjacent rows. The formal AUDIT_DECOMP pre/post comparison may ride here if its runner is available in-session; otherwise park it named. These are obligations, not the iteration's product — keep them to one node.
>
> Node 2 — first vocabulary-coverage engineering target (the product of this iteration). Priority target: unblock component creation. The operation applier currently hard-blocks `insert_component_symbol` pending "completion plan A3" (lib.rs ~1684). This direction explicitly authorizes retiring that guard in favor of an implemented `create_component` resolver with explicit geometry/connectivity inputs (DEL-16-01 scope), subject to the instruments' validation gates, plus the emitting surfaces: viewport component tools (DEL-07-01 scope) and inspector creation forms (DEL-07-02 scope). This single seam gates the largest set of NORMATIVE-NOW rows (bend, tee, reducer, valve, flange, expansion-joint creation). If your instruments size it beyond one node, take the largest bounded slice — resolver + one component kind end-to-end — and record the remainder against the coverage rows. Alternative if component creation is blocked for a reason you record: the `hanger.*`/`nonlinear.*` authoring field-paths (also DEL-16-01 + DEL-07-02). Boundary reminder from the ruling: DEL-07-09 owns the contract and the coverage ledger only — implementation lands in the deliverables the landing column names. Close coverage rows by pointing them at landed evidence.
>
> Node 3 — CI fragility fixes (write-disjoint from Nodes 1–2). Two defects observed 2026-08-21 when GitHub served pre-externalization objects slowly (~40s per fetch); a branch from current main will not reproduce them — these are preventive hardening, do not attempt reproduction or burn re-runs. (a) Desktop E2E lane: `.github/workflows/piping-desktop-e2e.yml` does a plain depth-1 checkout and Playwright's default GitCommitInfo collector then fetches the merge-base with a 3-second internal timeout — under slow serving it crashed the lane before any test with `RangeError: Invalid string length`, four consecutive times. Fix: disable git-info capture in `apps/desktop/playwright.config.ts` (and the dist config if it runs in CI) — run identity is already carried by the evidence sweep's commit binding — or pre-fetch the merge-base as an explicit workflow step so the collector's fetch is a local no-op; pick one, state why. (b) Governance harness: the checkout already uses full-history/blob:none and the "Candidate whitespace" step already falls back to HEAD^ when the base SHA is unresolvable, yet under slow serving its lazy base-blob fetches hung until the 15-minute job cap killed it, reporting as cancellation. Fix: a step-level `timeout-minutes` bound (with a legible failure message) so a slow fetch fails fast and diagnosably. Item (b) touches the root governance-harness workflow — explicitly authorized by this steer. Both workflow edits are instruction-surface changes: ship the G4 tranche manifest(s) in the same tranche and update any policy test that pins the touched workflow text. None of this iteration's nodes requires the unsigned-artifact macOS lane — do not apply the `artifact-proof` label.
>
> Node 4 — support-family emission fix (write-disjoint from all other nodes). The preview builder collapses partial-DOF restraint sets to `SupportFamily::Guide`; `LineStop` and `VerticalSupport` exist in the solver crate but are never emitted. Fix the family mapping in `product_physics` so named families reach the solver, with focused tests. Closes the corresponding NORMATIVE-NOW row; lands in DEL-04-03 territory per the landing ledger.
>
> Closeout. Steer v3 defaults apply: one tranche, one branch, one PR against main (predecessors merged), commits in dependency order, shared-surface writes once at fan-in, receipt written after the fact pointing at nodes and evidence. Parked owner holds are unchanged and not selectable: PKG-02 runtime/transport/permission choices, `.opsproj` policy, PDU-011/PDU-047, `MAINTAINER_REVIEWED` promotion. If a node fails its checks, land what passed, record the failure, stop.

## Later exact owner amendment

> You can make those fixes for Node 2

The amendment applies only to the two N2 review findings enumerated in `N2_AMENDMENT_V2.md`; it does not authorize N4 remediation, a broad rerun, or the final evidence sweep.

## Final owner resolution — ordinary repair and closeout

> Good. I shouldn't have said that though. Try to close everything out. Resolve failures.

This direction withdraws the earlier stop rule for ordinary bounded repair and closeout. It does not erase the recorded N1/N4 failures, authorize unrelated scope, merge PR #599, select parked owner holds, require the unsigned-artifact lane, or create lifecycle/release/professional-reliance effects.

## Merge and second upstream-sync authorization

Owner direction relayed through HELP_HUMAN after PR #599 reached green required checks:

> User explicitly authorizes merging PR #599.

When the merge gate found that `main` had advanced substantively through PR #601, the owner then explicitly authorized:

> A non-rewriting sync of current main into PR #599, revalidation, and merge.

This authorizes the exact protected PR merge after current-main containment and all required post-sync checks pass. It does not authorize a rebase, force push, protection bypass, `artifact-proof` label, branch deletion, or any lifecycle/release/professional-reliance effect.
