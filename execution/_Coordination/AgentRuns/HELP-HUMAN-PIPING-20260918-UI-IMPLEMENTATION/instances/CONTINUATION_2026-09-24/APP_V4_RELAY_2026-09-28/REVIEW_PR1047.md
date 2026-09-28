# Independent review of PR #1047 (SWBPIPE answers to the App v4 relay questions)

- **Reviewer:** RV15, Type 2 TASK (independent reviewer), dispatched by ROOT (SWBPIPE HELP_HUMAN). RV15 did not write the material under review. Read-only: no Git or GitHub writes.
- **PR:** https://github.com/sgttomas/chirality/pull/1047
- **Head reviewed:** `a2323bc963fc12bcc075008f4885b55c9000a534` (merge `73519d1d2` of main `d1cc97ce4` into the branch, then `a2323bc96`). The first head in the brief, `e277942621239649037fd8faa09b88af5842e322`, was also read; ROOT moved the head during the review.
- **Base:** main `d1cc97ce45de3a2680bcf85e49b345ec3ee5bdad` (#1046). The brief's base `65e2d6c2a` was superseded when main advanced.
- **Date:** 2026-09-28
- **Verdict: MERGEABLE.** There are no BLOCKING findings. Four SHOULD-FIX items are best repaired before merge. None of them makes the answers mislead the App on substance.
- **Counts:** BLOCKING 0, SHOULD-FIX 4, NOTE 10.

Abbreviations: `D/` = `projects/chirality-app-v4/execution/PKG-09_Candidate examination and connected journeys/1_Working/DEL-09-06_Connected activity contract and workflow round trip/Design/`; `P/` = `projects/chirality-piping/`; `ANS` = `D/RELAY_ANSWERS_SWBPIPE.md` at the reviewed head; `FACTS` = `D/FACTS_SQ01_SQ32.md`; `REC/` = the SWBPIPE records folder `P/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/APP_V4_RELAY_2026-09-28/`; `WG` = `P/execution/_Coordination/WorkGraphs/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/WORK_GRAPH.md`.

## 1. Checks

| # | Check | Result | Evidence |
|---|---|---|---|
| 1 | Scope: the diff is exactly two added files | PASS | `git diff --name-status d1cc97ce4 a2323bc96` shows `A FACTS_SQ01_SQ32.md` and `A RELAY_ANSWERS_SWBPIPE.md`, nothing else. The same held against `65e2d6c2a` for `e27794262`. `gh pr diff 1047 --name-only` lists the same two files |
| 1c | ROOT addition (c): the PR diff against current main is still exactly the two added files | PASS | As above. The merge base is `d1cc97ce4`, and merge `73519d1d2` adds no other change |
| 2 | Fidelity: FACTS | PASS | sha256 `2f61d3ba4e1cc9bedb799bf15c600d67e4820b7305741a1269d4a08355b4ddfc` at the head, at `REC/` in `7092582d8`, `2828c6b69` and `43f7a0c1e`, and in `SHA256SUMS` |
| 2 | Fidelity: ANS | PASS, with S-1 | ANS sha256 `03e50257bd28d10f8b01db43328f98ae5aa45b2c0df180ab6e7da419ca4b0fd7` equals `REC/RELAY_ANSWERS_SWBPIPE.md` at `43f7a0c1e` and that commit's `SHA256SUMS`. Against `7092582d8` (`64ea4e59…`), only lines 3, 4, 9, 11, 378, 380 (heading) and 382–384 differ, all delivery or provenance lines; see §2 |
| 3 | Basis claim | PASS | `git diff --name-only 24dea2dae 65e2d6c2a -- projects/chirality-piping` and the same command against `d1cc97ce4` are both empty. All 53 paths changed by `24dea2dae..65e2d6c2a` are under `projects/chirality-app-v4/`, and so is every path changed by `24dea2dae..d1cc97ce4`. The `projects/chirality-piping` tree object is `0fdbef713…` at `24dea2dae`, `65e2d6c2a` and `d1cc97ce4` |
| 3a | ROOT addition (a): the questions file's change `c6f81a4f2 → d1cc97ce4` touches no SQ question or "App assumes" text | PASS | Hunks only at lines 3 (header status), 1037–1038 (§4 preamble), 1042–1044 (§4 Prepared, Relayed and Acknowledged rows), 1081 (+2 change-log rows) and 1133 (VC-R-04). §2 (questions) spans lines 78–926. ANS:3 describes this correctly, and sha256 `83466d67…` matches the file at `d1cc97ce4` |
| 3b | ROOT addition (b): `git diff --name-only 24dea2dae d1cc97ce4 -- projects/chirality-piping` is empty | PASS | 0 paths |
| 4 | Accuracy | PASS, with S-2, S-3, S-4 and notes | Every SQ-01…SQ-32 is answered or says what it waits on. The citations verified are in §3. No wrong citation was found. Some claims have no fact-sheet support (S-2) |
| 5 | Standing and overreach | PASS, with S-3 and notes | No commitment, delivered contribution or owner intention is claimed (ANS:5–6). Owner-reserved items are marked OWNER DECISION and left open. No SWBPIPE construction is promised. "DRAFT #885" facts are consistently labelled |
| 6 | Hygiene | PASS, with N-7 and N-9 | No user-local absolute paths, credentials, tokens or personal data. `api_key` appears only as a redaction key name. No trailing whitespace, and both files end with one newline. Every Markdown table has a consistent column count (ANS tables at 15, 55, 123, 171 and 338; FACTS tables at 13, 23, 268, 539 and 568) |

## 2. Delivery and provenance lines changed since `7092582d8`

| ANS line | Change | Judgement |
|---|---|---|
| 3 | Adds that main `d1cc97ce4` changed the questions file in place (sha256 `83466d67…`): header status, §4 ledger and change-log rows, VC-R-04; no question text changed | True (check 3a). The §4 preamble change counts as part of "the §4 ledger" |
| 4 | "At the owner's direction, this session placed this file and its fact sheet beside the questions" | Consistent with the brief. The owner's direction is ROOT's statement, and RV15 cannot observe it independently |
| 9 | "Main has since moved to `d1cc97ce4`, the base of the delivery … touches only `projects/chirality-app-v4/`" | True now (check 3). It goes stale if main moves again before merge (N-1) |
| 11 | FACTS sha256 `2f61d3ba…` | True |
| 378 | "No existing App file was changed … two new files" | True (check 1) |
| 380 | Heading "Delivery and SWBPIPE-side record" | — |
| 382 | Delivered 2026-09-28 as two new files; the delivery revision is the main commit that adds them | True once merged |
| 383 | "Fact sheet: delivered as the researcher prepared it, unchanged" | **Not accurate**; see S-1 |
| 384 | Identical copies in `REC/` with SHA256SUMS on `codex/piping-numerical-integrity-20260926` | True at `43f7a0c1e`, which is on `origin/codex/piping-numerical-integrity-20260926`. "They reach main with SWBPIPE's next records PR" is ROOT's process statement, not an owner commitment |

## 3. Citations verified against sources

All of these were read at `24dea2dae` (the piping tree is identical at `d1cc97ce4`) or at PR #885 head `12907f393`, using local `git show`. Each says what the answer uses it for.

- **SQ-01 and A-1:**
  - `P/apps/desktop/src/features/workspace/shellLayout.ts:146-147` (G-19; "Agent: not available yet") and `…/shell/AgentStrip.tsx:5-9`.
  - `…/OfflineProposalIntakePanel.tsx:17-26` (":26" attribution not verified; "no connected agent provider").
  - `P/core/model_operations/operation_applier/src/lib.rs:123-127` and `:2018-2030` (the acceptance record has no actor and no time; `user_initiated_apply_in_local_session`, `session_state_only_not_yet_saved`).
  - `P/apps/desktop/src/types.ts:983-987`, `:1013-1028` and `:1198-1210` (the envelope has no receipts).
  - `workspaceSession.ts:1789-1792` and `:1859-1862` (receipts reset on create and open); `:1029-1044` (Clear writes no record).
  - `BatchReviewPanel.tsx:30-38`.
  - `OperationLedgerPanel.tsx:175-179`; its `actor_ref` is the placeholder `local-preview-user`, so no person is named.
  - OPMAP `:292-298`, `:457`, `:464`.
  - PR885 `liveControlController.ts:508-546` (receipt: no person, no time field).
  - PR885 `LIVE_CONTROL_DEVELOPMENT.md:53`, `:57`, and ":90 Complete invented exchanges".
  - D-71 addendum "Adopted bounded effect" and "Unchanged" (G-08; an owner-authorized tranche is needed); DEC-104 is at `SD:709`.
  - `P/docs/PROFESSIONAL_BOUNDARY.md:182`, `P/docs/PRD.md:1340-1344`, `P/docs/claims_registry.md:156-159` and `:183-185`, `P/docs/CONTRACT.md:29`.
  - DEL-16-03 ScopeOfWork `:69`, `:77`, `:79`.
- **SQ-02:**
  - `atomic_batch.rs:213-231` (`exact_object`: "Unknown field") and `:285-301` (exact field list).
  - PR885 `liveControlController.ts:231-233` and `:347-348` (exact-key params).
  - `workspaceSession.ts:1391-1398` (undo checkpoint terminology).
- **SQ-03:**
  - `P/apps/desktop/src-tauri/src/lib.rs:2050-2059` (`sha256`, `rfc8785_jcs`, `model_payload`).
  - `atomic_batch.rs:18-25`, `:90`, `:95-105`, `:124-143`.
  - `lib.rs:7422` and `:7450` (`OP-STALE-BEFORE-VALUE`), `:7834`, `:7935` (`OP-CLAIMED-MODEL-HASH-MISMATCH`, in the cited range `:7847-7948`).
  - Contract draft `:45` and WIRE_PROPOSAL `:58` (Undo advances revision).
- **P1 group, other:** `atomic_batch.rs:203-208` (`requires_user_acceptance: true`, `direct_model_mutation_allowed: false`, `agent_runtime_binding: held_D58`, `source_identity_verification`) and `lib.rs:2062-2066`.
- **SQ-04 and SQ-32:**
  - Operation kinds: `lib.rs:2176` has 27 change kinds, and `:33` gives `OPERATION_APPLIER_VERSION = "0.1.0"`.
  - Solve and rule-check routes in the desktop `src-tauri/src/lib.rs`: `run_preview_mechanics` at `:1574`; the background start, poll and cancel jobs at `:1800`, `:1835` and `:1843`; `run_rule_checks` at `:2739`.
  - The status vocabulary: `claims_registry.md:171-178`.
  - PR885 `LIVE_CONTROL_DEVELOPMENT.md:9-13` and `:39`.
  - The owner activation of 2026-09-24 (`C24/OWNER_DECISIONS.md` "Bounded live-controller activation") and the contract draft `:7` (development Codex first; "Embedded Runtime follows").
- **SQ-05, SQ-06 and SQ-09:**
  - `SD:599` (OI-016, "human product decision").
  - PR885 `liveControlController.ts:471-473` (`withdrawn`/`cleared_in_review`), `:493-497` with `workspaceSession.ts:1093` (`rejected: validation_rejected` is set when the Apply fails engine validation), `:140` (`unsupported_method` text), and `:17-22` (`invalid_request`).
  - PR885 `swbpipe-control.rs:9-13` and `:65-66` (Apply is not a method; refused before transport as `unsupported_method`) and `:68-72` (`unsupported_host`).
  - OPMAP `:297` and `:474` (G-19, "Withdrawn by the agent").
  - PR885 `LIVE_CONTROL_DEVELOPMENT.md:59` (states; timeout "does not prove").
- **SQ-07 and SQ-08:**
  - `BatchReviewPanel.tsx:41-49` (whole-model stale).
  - `workspaceSession.ts:1046-1057` (queue-time basis) and `:1069-1070`.
  - PR885 `liveControlController.ts:86-104` (new workspace on generation change), `:106-112` (`stale_basis`), `:318-345` (frozen preview), `:347-360` (`keys.get` precedes `fresh()`; `idempotency_conflict`) and `:453-457`.
  - Contract draft `:43-45` and `:77` (current holds).
  - `LIVE_CONTROL_ACTIVATION_PROPOSAL.md:19`.
- **SQ-10:** `workspaceSession.ts:1447-1489` (25-deep snapshot stacks; Undo writes no receipt) and `:1787`/`:1857`; OPMAP `:293`.
- **SQ-12 to SQ-16 and SQ-28:**
  - Contract draft `:13`, `:17`, `:21-23`, `:47`, `:49`.
  - `C24/COORDINATION.md` "Current agreement" (development Codex; "no external Apply"), plus PR #827 evidence.
  - WIRE_PROPOSAL `:9` ("not verified Codex identity").
  - PR885 `LIVE_CONTROL_DEVELOPMENT.md:13-15` (off by default; 0700/0600; the directory is under the system tmp in macOS's `private` root) and `:61-70` (16 admitted connections).
  - PR885 `liveControlController.ts:246-258` (controller-assigned `author_type` and `source`).
  - `atomic_batch.rs:322-341`.
  - `SD:656` (DEC-051, "for now", "revisiting is a human decision"), `P/docs/SPEC.md:378-390` (§4.4 TBD list) and `P/docs/CONTRACT.md:41`.
- **SQ-19, SQ-20, SQ-24 and SQ-26:**
  - OPMAP `:289` (row 208), `:301` (row 220), `:463`, `:466`, `:474`.
  - `WG:123` (RUNTIME-ADOPT PLANNED).
  - `SD:696` and `REG:95` (D-58/DEC-091: no successor adopted; Piping outside the Root-runtime and App-harness client sets).
  - `SD:708` (DEC-103 item 8: five class words) and `SD:585` (SWBPIPE OI-003 = legal review).
- **SQ-27 and SQ-29:**
  - `WG:121` (LIVE-HUMAN "BLOCKED until live implementation and owner participation").
  - `C19/OWNER_CLI_PROTOCOL_DISPOSITION_PEER_2026-09-20.md:10`, `:15`, `:17` (oMLX; "no silent cloud fallback").
- **A-2 and the contradicted-assumptions list:**
  - `WG:12`, `:21`, `:46-60`, `:120-122`, and `C24/OWNER_ROUTE_DIRECTION_2026-09-25.md`, which is dated after PR #905 merged at 2026-09-25T22:13Z. Its records were committed at 23:34Z.
  - PR #885's description, last updated 2026-09-25T18:37Z, still says the owner "reconfirmed proceeding" and asks to coordinate "with the solver work, including PR905". Contradiction 11 ("predates the work graph's deferral, which governs") is therefore supported. `C24/OWNER_RESUME_2026-09-26.md` confirms "UI-SUCCESSOR stays deferred".
  - X-7 / A-4: `git grep` over `P/` at `24dea2dae` for App v4, HTML-D05, OI-021, HANDOFF_SWBPIPE and RELAY_QUESTIONS finds 8 files, all incidental (review notes and rebase notes). None mentions HTML-D05, OI-021, HANDOFF_SWBPIPE or RELAY_QUESTIONS.
  - "Minimal host loop" is at `projects/chirality-app-v4/execution/_Coordination/HANDOFF_SWBPIPE_DOMAINS.md:22`, and not in piping records.
- **Priority coverage.** All citations for SQ-01, SQ-02 and SQ-03, all 12 contradicted assumptions, and all 10 owner-decision rows were checked. More than 40 citations across SQ-04…SQ-32 were also checked.

## 4. Findings

### BLOCKING

None.

### SHOULD-FIX

**S-1. ANS:383 says the fact sheet was delivered "as the researcher prepared it, unchanged". The PR description says "unchanged as prepared".**

- **Evidence:** the researcher's text entered the records at `5fb18b56c`, with FACTS sha256 `23b4e9d9…`. ROOT's commit `7092582d8` ("reword the socket location without an absolute path (GEN-8 ABS_PATH finding)") then changed FACTS line 366: an absolute path to the macOS private tmp root became "the macOS system temporary directory (`private/tmp`)". ANS line 239 changed the same way.
- **Effect:** the change is small and hygiene-only, but the custody statement is not true as written.
- **Fix:** "delivered as prepared, except one path-hygiene rewording at SQ-15 (FACTS line 366) made by ROOT in the records commit `7092582d8`". Make the same correction in the PR description.

**S-2. ANS:11 says "every statement below rests on a file:line citation in the fact sheet". Several T3-sourced statements have no fact-sheet entry and no inline citation.**

- **The statements:**
  - ANS:105 and :108: integrity standing "Passed, Sensitive, or refused with a named `NUMERICAL_INTEGRITY_*` diagnostic".
  - ANS:300: "T9 byte identity (Mac-only on the owner's Mac)".
  - ANS:305: "T3's frozen references, the both-entry gate and T9".
  - ANS:7: "the both-entry and T9 gates".
  - ANS:91: the file is cited, but FACTS has no line for it.
- **Evidence:**
  - FACTS contains none of "T9", "both-entry", "Sensitive" or `NUMERICAL_INTEGRITY` (grep).
  - The integrity statements are true on main. `P/schemas/results.schema.yaml:3601-3608` has the status enum `not_assessed | checks_passed | sensitive | unresolved | failed`. The `NUMERICAL_INTEGRITY_CHECKS_PASSED/SENSITIVE/UNRESOLVED/FAILED/…` codes are in `P/core`.
  - "T9" is ambiguous for an App reader. In T3 records it is the committed-regeneration zero-byte-diff check (`…/NUMERICAL_INTEGRITY_T3/DESIGN_NUMERICS/S11G_GUARD.md:549`). In `WG:41`, "T9" is a different item: "coverage and validation assessment".
- **Fix:** either cite these inline (for example `P/schemas/results.schema.yaml:3601-3608` for the standing enum), or qualify ANS:11: "…except T3 facts, which ROOT states from its own work and cites inline". Also define "T9" as "the committed-fixture regeneration byte-identity check (T3), not the work graph's T9 tranche", and define "both-entry gate", or drop both terms.

**S-3. ANS:359 (contradiction 7) says "SWBPIPE plans a later 'embedded Runtime' adoption".**

- **Why it matters:** ANS:6 promises that a record's plan is cited as the record's plan, not as a SWBPIPE intention. The shorthand also understates the governing state.
- **Evidence:**
  - `WG:123` records RUNTIME-ADOPT as a "PLANNED external dependency" whose Needs column is "Concrete later adoption need and authority", with "live Piping integration unqualified".
  - `REG:95` and `SD:696` (D-58/DEC-091) keep Piping outside the Root-runtime and App-harness client sets, with no successor adopted.
  - ANS:39 states this correctly ("The embedded direction recorded is … RUNTIME-ADOPT, planned").
- **Why this list:** the App is most likely to act on the contradicted-assumptions list, because it bears on the App HANDOFF's "minimal host loop" direction.
- **Fix:** "SWBPIPE's work graph records a later 'embedded Runtime' adoption as PLANNED, contingent on a concrete need and authority. D-58 currently keeps Piping outside the Root-runtime and App-harness client sets, with no successor adopted."

**S-4. The deferral is described without saying that the owner's bounded live-control activation persists: A-2 (ANS:31-33), SQ-04 (d) (ANS:115) and §2 row 1 (ANS:340, "Whether and when UI-SUCCESSOR … resumes").**

- **Evidence:**
  - `WG:12`: "Activated live milestone (deferred to `UI-SUCCESSOR`; activation persists)".
  - `WG:60`: "The C4 rulings, the live activation and the unperformed actual-human H1/H2 witnesses are unchanged. The successor starts when the owner directs".
  - `C24/OWNER_ROUTE_DIRECTION_2026-09-25.md`, "Interpretation (ROOT)": deferred work "is not abandoned, cancelled or closed".
- **Effect:** "Whether" adds uncertainty the records do not state, and the App should know the activation remains in force. SQ-04 mentions "owner-activated" but not that the activation persists.
- **Fix:**
  - Add to A-2: "The owner's 2026-09-24 bounded activation persists, and the deferral is not abandonment (`WG:12`, `:60`)."
  - Change §2 row 1 to "When UI-SUCCESSOR, including live control, resumes (the activation persists); and whether the App v4's Codex becomes a named caller".

### NOTE

- **N-1. The PR description is stale (not part of the delivered files).**
  - It gives ANS sha256 `2c074d0f…`, but the head is `03e50257…`.
  - It says the questions are "unchanged on main since `c6f81a4f2`", but `d1cc97ce4` changed them in place (check 3a).
  - It says "Independent review: pending".
  - It says the fact sheet is "unchanged as prepared" (S-1).
  - Update these before merge. ANS:9's "Main has since moved to `d1cc97ce4`" also needs a refresh if main moves again before merge.
- **N-2. SQ-09 (a) mapping (ANS:171-182) has no rows for the App terms "accepted" and "application error", and does not address "with the evaluated basis".** Suggested rows:
  - "accepted: no separate state; Apply is the acceptance (SQ-01)".
  - "application error: none distinct; a failure at Apply is `blocked` (main) or `rejected: validation_rejected` (#885)" (FACTS:264).
- **N-3. Some sub-questions are answered only by implication.**
  - SQ-06's first question ("Does an external actor use the same grant the person sets per class?", ANS:138-141): add "No: no grants or classes exist (SQ-05)".
  - SQ-14's "Can the App read your origin mark?": yes, in #885 through `status` (SQ-01).
  - SQ-32's stream cancel: not applicable, since there is no loop.
- **N-4. Owner-decision attribution is sometimes stronger than the record.**
  - ANS:165 says durable de-duplication is "an OWNER DECISION per SWBPIPE's live-control contract draft". The draft (`:77`) lists "any required durable receipt carrier" as a current hold and does not name the owner.
  - The record for §2 row 4 (ANS:343), `PB-TBD-002`, names "Future persistence/report/governance deliverables" as its owner (`P/docs/PROFESSIONAL_BOUNDARY.md:182`). DEL-16-03's actor model is TBD with no owner named (FACTS:106).
  - Marking these OWNER DECISION is conservative and does not suggest a commitment. For precision, write "OWNER DECISION or an open deliverable TBD, per the record".
- **N-5. ANS:213 says "the owner's modern stateless MCP condition of 2026-07-28".** 2026-07-28 is the MCP protocol revision the condition requires (contract draft `:21`). The owner's disposition is dated 2026-09-20, as §2 row 6 (ANS:345) says. Reword to "…condition that it follow stateless MCP revision 2026-07-28".
- **N-6. Some future-tense questions get bare answers that read as intentions.**
  - ANS:261: SQ-18 (b)–(d) "No". (d) asks "Will you publish compatibility statements…?".
  - ANS:286: SQ-25 "Host facility only".
  - ANS:308: SQ-27 (e) "nothing at present". FACTS:482 says only "not found".
  - Qualify these, for example: "none exist, none planned in any record, not decided"; "per current records; any proxy would be an OWNER DECISION"; "no SWBPIPE record names an App-side input it needs".
- **N-7. ANS:239 and FACTS:366 render the socket location as "(`private/tmp`)".** This looks like a relative path. The source (PR885 `LIVE_CONTROL_DEVELOPMENT.md:15`) names the system-wide tmp directory under macOS's `private` root, not the per-user TMPDIR, and the difference matters for SQ-15's sandbox-reach question. A wording that stays GEN-8-clean: "the system-wide tmp directory (under macOS's `private` root), not the per-user TMPDIR".
- **N-8. SQ-30 (ANS:320) reports the DEC-051 versus local-only conflict but leaves out a related fact.** SWBPIPE's own fence F-PIP-1 still reads "local-only operation — no cloud…" (`P/loop/WORKPLAN_2026-07-18b_piping_loop.md:161-164`), subject to owning rulings (FACTS:513). The tension is therefore partly internal to SWBPIPE records, which is useful context for the App.
- **N-9. FACTS:594 names the search command by the absolute path of the system `grep` binary.** This is a system binary path, not a user-local path or personal data. It is harmless, and GEN-8 passed per the PR. No change is needed; FACTS is delivered verbatim.
- **N-10. CI was not complete at review time.** `gh pr view 1047` showed `mergeStateStatus: BLOCKED`, with `harness` and `PEC workspace tests` still IN_PROGRESS on `a2323bc96`. Merge only after the required checks pass on the reviewed head. Any content repair for S-1…S-4 changes the head and needs a delta recheck.

## 5. Scope of this review

RV15 read both delivered files in full and the questions file's §0–§2 and §4, and compared them against the records copies at `7092582d8`, `2828c6b69` and `43f7a0c1e`. Product and record citations were read with local `git show` at `24dea2dae` and `12907f393`. GitHub was read only through `gh pr view` / `gh pr diff` and one read-only `gh api …/compare` call. RV15 wrote nothing except this file. RV15 did not rerun GEN-8.

## Delta check at c322826ea

- **Head checked:** `c322826ea782695be10b504a543f31e806e99c0a`. It is one commit after `a2323bc96` and applies §4's findings. The base is main `d1cc97ce4`.
- **Date:** 2026-09-28.
- **Verdict: MERGEABLE.** No new BLOCKING finding. One new SHOULD-FIX (D-1) and two NOTEs (D-2, D-3).
- **ANS in this section:** the answers file at `c322826ea`, sha256 `6f01add3977761e42ac6b310faf72ba4fd5455e478605deb83fefb2e4d3a61c7`. FACTS is sha256 `733fb88a701317be8f0054937eca058774ba5f5f30c7a27233718996e8b2ab7e`.
- **`T3/`** = `P/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/`.

### Delta checks

| # | Check | Result | Evidence |
|---|---|---|---|
| Δ1 | Each fix does what the finding asked | PASS, with D-1 | See the table below |
| Δ2 | New citations resolve at `d1cc97ce4` and say what the text says | PASS, with D-2 and D-3 | See the list below |
| Δ3 | Scope | PASS | `git diff --name-status a2323bc96 c322826ea` shows `M` only for the two delivered files. `git diff --name-status d1cc97ce4 c322826ea` still shows exactly two `A` files, with merge base `d1cc97ce4`. The FACTS delta is line 366 only |
| Δ4 | No new overclaim and no new machine path | PASS, with D-1 | Machine-path check: the GEN-8 `MACHINE_ABS_PATH_RE` pattern finds no match in either file. Formatting: `git diff --check a2323bc96 c322826ea` is clean; there is no trailing whitespace and each file ends with one newline. Tables: all ANS and FACTS tables have consistent column counts, including SQ-09's table with its two new rows |
| Δ5 | Records sync | PASS | At `e997ff126`, `REC/` holds ANS `6f01add3…`, FACTS `733fb88a…` and this review `9ccea545…`, and all three match its `SHA256SUMS`. The review's pre-delta bytes are unchanged there |
| Δ6 | PR description (N-1) | PASS | It now gives `6f01add3…` and `733fb88a…`, describes the in-place change at `d1cc97ce4`, records FACTS line 366's two rewordings, and says this delta check is pending |

### Δ1: each fix against its finding

| Finding | Where applied in ANS at `c322826ea` | Result |
|---|---|---|
| S-1 | :391 says the fact sheet was delivered as prepared except line 366, which ROOT reworded twice: `7092582d8` for GEN-8, then per N-7. The PR description says the same | Done. It matches the FACTS delta |
| S-2 | :11 states the T3 exception and adds the `T3/` abbreviation. :7 points to SQ-27 for the definitions. :106 cites the integrity standing. :307 defines T9 and separates it from the work graph's T9 tranche. :312 defines the both-entry gate | Done. D-2 and D-3 are small citation gaps |
| S-3 | :367 says the work graph records RUNTIME-ADOPT as PLANNED and contingent, and that D-58 keeps Piping outside the Root-runtime and App-harness client sets with no successor adopted. The same applies at A-4 (:40) | Done |
| S-4 | :34 (A-2 new bullet), :116 (SQ-04 (d)) and :348 (§2 row 1: "When …, resumes (the activation persists)", citing `:12` and `:60`) | Done |
| N-2 | :178 adds an "accepted" row, :182 an "application error" row, and :188 an evaluated-basis bullet | Rows done; the bullet under-states, see D-1 |
| N-3 | :141 (SQ-06 same grant), :237 (SQ-14 origin readable; PR885 `status` → `recover` → `result()` returns `receipt`, `liveControlController.ts:113-120`, `:412-419`) and :338 (SQ-32 stream cancel) | Done |
| N-4 | :167 ("The draft names no decider; this answer treats it as an OWNER DECISION") and :351 (PB-TBD-002 owner; DEL-16-03 "no owner named") | Done |
| N-5 | :219 "the owner's condition (2026-09-20) that it follow the stateless MCP revision 2026-07-28" | Done |
| N-6 | :268 (SQ-18), :293 (SQ-25) and :315 (SQ-27 (e)) | Done |
| N-7 | ANS :246 and FACTS :366: "the system-wide tmp directory (under macOS's `private` root), not the per-user TMPDIR" | Done. It matches PR885 `LIVE_CONTROL_DEVELOPMENT.md:15` |
| N-8 | :327 adds F-PIP-1 with `P/loop/WORKPLAN_2026-07-18b_piping_loop.md:161-164` | Done |
| N-9, N-10 | No change needed; CI | CI is still running on `c322826ea`: `harness` and `App Runtime integration` were IN_PROGRESS when checked |

### Δ2: new citations at `d1cc97ce4`

- **`P/schemas/results.schema.yaml:3601-3608`** gives the status enum `not_assessed | checks_passed | sensitive | unresolved | failed`. It resolves, but see D-2.
- **`P/core/product_physics/src/lib.rs:1068-1092`** gives `append_integrity_report`, which chooses `NUMERICAL_INTEGRITY_SENSITIVE` or `NUMERICAL_INTEGRITY_CHECKS_PASSED` (the code choice sits at `:1089-1093`). Resolves.
- **`P/core/product_physics/src/lib.rs:1265-1284`** gives `append_integrity_failure`, which emits blocking `NUMERICAL_INTEGRITY_PHYSICAL_MECHANISM`, `…_NEGATIVE_ENERGY`, `…_FAILED`, `…_ASSEMBLY_UNRESOLVED` and `…_UNRESOLVED`. Resolves.
- **`T3/DESIGN_NUMERICS/S11G_GUARD.md:549`** reads "T9 | Committed regeneration | — | Zero committed-byte diff". It resolves for the definition; see D-3 for "Mac-only".
- **`T3/DESIGN_NUMERICS/DESIGN.md:894-895`** defines "No Passed breach": a case with a failed covered comparison must not be published `Passed`, and the gate runs every case through both the captured entry and the historical typed entry. Resolves.
- **Work graph `:12`** reads "deferred to `UI-SUCCESSOR`; activation persists". **`:60`** reads "the live activation … unchanged. The successor starts when the owner directs". Both resolve.
- **`OWNER_ROUTE_DIRECTION_2026-09-25.md:25`** reads "not abandoned, cancelled or closed … activations … unchanged". It resolves. It is ROOT's recorded interpretation of the owner-approved refresh, and ANS uses it with the work graph lines, which is appropriate.
- **`P/loop/WORKPLAN_2026-07-18b_piping_loop.md:161-164`** is F-PIP-1, "local-only operation — no cloud, daemon, network…". Resolves.

### New findings

**D-1 (SHOULD-FIX). ANS:188, the evaluated-basis bullet, under-states what the product returns.**

- **The text:** "The fact sheet records no evaluated-basis field on the other outcomes". That is literally true of the fact sheet, but main's outcomes do state the evaluated whole-model basis:
  - The single-operation outcome carries `model_basis: ModelBasisEvidence` {`claimed_model_hash`, `backend_model_hash`, `binding_status`, …} (`P/core/model_operations/operation_applier/src/lib.rs:104-110`, field at `:145`).
  - The batch outcome carries `initial_model_hash`, `input_backend_hash` and `submitted_initial_model_hash` (`…/operation_applier/src/atomic_batch.rs:186-191`).
  - PR #885's `preview` returns that batch `outcome` (PR885 `liveControlController.ts:338-344`).
- **Effect:** the App could conclude that outcomes lack the basis they were evaluated on.
- **Fix:** "Evaluated basis: main's outcomes state the evaluated whole-model basis. A single operation carries `model_basis` {claimed and backend hash, binding status}, and a batch carries `initial_model_hash` / `input_backend_hash` (cite the lines above). DRAFT #885's preview returns that outcome, and its committed receipt carries before and after revision and hash. There is no per-item basis." Because ANS:392 says the should-fix findings "are applied here", apply D-1 or adjust that line.

**D-2 (NOTE). ANS:106 lists the published integrity standing as `checks_passed`, `sensitive`, `unresolved` or `failed`, but the cited enum also has `not_assessed`.** A published result can carry `not_assessed` (the desktop reader's aggregate falls back to it: `P/apps/desktop/src/features/results/numericalResultQuality.ts:137-140`). Add `not_assessed`, or say "one of the standings in the cited enum".

**D-3 (NOTE). ANS:307 says T9 is "run Mac-only on the owner's Mac", but the cited `S11G_GUARD.md:549` supports only the zero-byte-diff definition.** The Mac-only part is supported by `T3/IMPLEMENTATION/K1/RETURN.md:35`: "T9 here is a Mac-only comparison". The Mac's platform libm differs from the Linux records on 12 of 112 committed outputs. Add that citation, or state it as current practice since the move to the owner's Mac.

### Delta scope

RV15 read the full `a2323bc96..c322826ea` diff of both files, the PR description and the CI roll-up (`gh pr view`), and the records tree and `SHA256SUMS` at `e997ff126`. Citations were read with local `git show` at `d1cc97ce4` and `12907f393`. RV15 wrote only this section and this file's `SHA256SUMS` entry, and made no Git or GitHub writes.
