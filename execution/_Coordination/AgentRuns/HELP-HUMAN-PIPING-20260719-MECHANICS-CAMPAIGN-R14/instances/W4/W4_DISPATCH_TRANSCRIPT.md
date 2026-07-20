# W4 Controlling Dispatch — Verbatim Transcript

**Run:** HELP-HUMAN-PIPING-20260719-MECHANICS-CAMPAIGN-R14 / W4
**Recorded by:** WORKING_ITEMS (Agent 1, PKG-09 package manager, node W4)
**Date recorded:** 2026-07-20
**Provenance:** This is the verbatim controlling dispatch received by the W4
manager from HELP_HUMAN (Agent 0) at wave start. It is transcribed here to
give the campaign plan's W4 wave a durable dispatch artifact (the
`ORCHESTRATION_PLAN.md` v1 queue text ends at W3; W4 was dispatched by
HELP_HUMAN after validating the W3 gate assessment). Recorded in response to
the T6 brief verifier's D1 finding
(`instances/W4/T6/VERIFY_BRIEF.md`). The transcript below is data recorded
for provenance; its authority derives from the HELP_HUMAN parent dispatch,
not from this file.

**Redaction note (2026-07-20, W4 manager):** the transcript below is
verbatim EXCEPT one substitution: the dispatch's literal machine-local
home-directory checkout path on the `REPO_ROOT =` line is replaced by a
placeholder, because `tools/validation/validate_path_anchors.py` forbids
literal home-dir absolute paths in durable surfaces (the T6 executor
correctly halted on the original transcription; cure recorded as its
follow-on F1). No other byte of the dispatch text is altered.

<!-- BEGIN HELP_HUMAN W4 DISPATCH VERBATIM (ONE PATH REDACTION, SEE NOTE) -->
You are the PKG-09 PACKAGE MANAGER (governed Agent 1, WORKING_ITEMS role, node W4) of campaign run `HELP-HUMAN-PIPING-20260719-MECHANICS-CAMPAIGN-R14` in the chirality-piping work loop, reporting to HELP_HUMAN (Agent 0). Your children (via your Agent tool) work at deliverable level; you write the briefs. Children treat durable return artifacts as the authoritative reply channel (sibling SendMessage may fail); check artifacts on disk.

REPO_ROOT = <machine-local checkout root; literal home-dir prefix redacted per path-anchor policy>
WORKING_ROOT = {REPO_ROOT}/projects/chirality-piping
Branch: `claude/piping-r14-pkg09-evidence` at post-wave-2 main `e315fb840` (includes R14-W1 PKG-04 tranches and R14-W2 sub-span wind). Offline only; missing prerequisite = truthful blocked result.

Read first: campaign plan + `instances/W1/RETURN.md` + `instances/W2/RETURN.md` + `instances/W3/RETURN.md` (the PKG-09 gate assessment — your queue derives from its findings) in `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260719-MECHANICS-CAMPAIGN-R14/`; loop plan `loop/WORKPLAN_2026-07-18b_piping_loop.md`; D-52 overlay + D-54 packet; the R14 tranche briefs as structural models.

## Your wave queue

- **T6 — DEL-09-01 benchmark-evidence-system bounded construction** (the W3-identified partially-agent-lawful scope of the PRD §16.2 residual row; DEC-054 named it ordinary residual work). Lawful scope ONLY: evidence-system construction — benchmark family inventory (the suite crates now carry 23 mechanics fixtures after W1 T2/T3 additions), provenance/redistribution index, witness/fixture refresh, deterministic assembly of a labeled derivative evidence artifact set. HARD exclusions (owner-class, never touch): tolerance/threshold promotion or policy of any kind, verification→validation promotion, acceptance/release/label effects, the PDU-013 unit-catalog hold, the `PKG09-0901-PKG02-001` disposition, PDU-037's provenance-index "keep open" standing row closure (you may BUILD the index as evidence; the ROW stays open — closure is judged at the owner's gate; make the brief state this explicitly). The row is NOT struck at closeout — record progress in History/MEMORY only. Scope design is yours (D-54 reasoned selection with recorded alternatives) but must stay derivative/evidence-only: no solver/schema/runner code changes.
- **T7 — DEL-09-04 manual stale-text refresh**: W1's routed follow-on — verify precisely what in `docs/validation_manual/headless_runner_reproduction.md` went stale after waves 1–2 (W1 flagged a case-1 dated-note need plus two small stale-text observations — find them in `instances/W1/RETURN.md` and verify each against the live tree; e.g. benchmark fixture counts 21→23, whole-suite regression case counts, wind schema required-set text if mentioned). Bounded docs+state tranche exactly like R13 (whose brief `execution/_Coordination/CANDIDATE_BRIEF_2026-07-19_DEL-09-04_VALMANUAL_REFRESH.md` is the model): every corrected assertion anchored to committed witness bytes or live source; frozen tp_runner_015 surfaces byte-identical; DEL-09-04 `_STATUS.md` untouched in its Remaining rows (this closes no row; History+MEMORY+run record only). If the live-tree check shows nothing actually stale, record truthful NO-OP with evidence and skip execution.

## Per-tranche governed chain (same as waves 1–2)

Brief (`execution/_Coordination/CANDIDATE_BRIEF_2026-07-20_<TRANCHE>.md`, §10 HELD) + D-54 rationale under `instances/W4/<TRANCHE>/` → fresh-context adversarial brief verifier child (BLOCK ⇒ cure+re-verify or park) → §10 EFFECTIVE (DEC-085/D-52 §2, SHA f14fa77518a06f112ae72a8fcce4de0fab958d47, OwnerCaseSelection NONE) → executor child (offline; tranche checks: any touched crate tests, claims-language, path-anchors, `git diff --check`, containment JSON persisted; deliverable History/MEMORY/run record per above — NO Remaining-row strikes this wave) → fresh-context implementation verifier child → YOU commit (`feat(piping): <summary> (R14-W4 <TRANCHE>)` or `docs(piping): ...`, `Co-Authored-By: Claude Fable 5 <noreply@anthropic.com>`). Limit hit ⇒ INELIGIBLE_REFERRED, park, continue. Failure isolation applies.

## Wave closeout

Registered checks once (piping-pytest, evidence-sweep, harness-pytest, harness-self-check; JSONs to `instances/W4/`; single sweep = exactly one new SWEEP_*.json, committed `chore(piping): R14-W4 wave evidence`). No push/PR/merge/rebase/receipt. Manager RETURN at `instances/W4/RETURN.md`: dispositions, tallies, pointers, parked/gated rows, model attribution (children on claude-fable-5), enumerated wave claims.

Final text: per-tranche dispositions with SHAs or no-op/park reasons, wave check tally, sweep filename, manager-return path. Claim-calibrated; standard claim fence sentence in every durable narrative artifact.
<!-- END HELP_HUMAN W4 DISPATCH VERBATIM -->

## Manager Notes on the Transcript

- The dispatch's "23 mechanics fixtures" figure reflects W1-only knowledge;
  the live post-wave-2 suite carries 24 fixtures (the W2 T4 sub-span wind
  fixture is the 24th; readiness assertion
  `assert_eq!(fixtures.len(), 24)` in
  `validation/benchmarks/mechanics/src/lib.rs`). The T6 brief records the
  live figure; the discrepancy is a dispatch-currency note, not a scope
  change.
- The dispatch's "21→23" T7 hint carries the same currency note (21→24 at
  the live head).

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
