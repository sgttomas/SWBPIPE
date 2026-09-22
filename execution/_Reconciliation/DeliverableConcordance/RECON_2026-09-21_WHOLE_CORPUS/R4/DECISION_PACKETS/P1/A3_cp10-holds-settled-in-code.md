# A3 — Holds that code settled with no ruling (CP-10)

PROPOSAL for the R4 gate (R3 integration, TASK P1). It decides nothing and changes no row.

Conventions: `F:` is `{FREEZE}/projects/chirality-piping/` at `00115c719`. `RUN/` is the run folder.

## 1. Decision

Two linked questions:
1. **Treatment rule.** When a deliverable declares an implementation choice open and the frozen code has since fixed it with no located ruling, does closing the hold need an owner ruling (CP-10), or is it F3 record catch-up (the "resolved-TBD" reading)? (T11 T-03.)
2. **Per topic.** For each of the 13 hold topics below, does the implemented choice close the hold?

**Holder: OWNER.** Question 1 is a convention reading under a RULED convention (CP-10), so it also concerns HELPS_HUMANS if the owner wants CP-10 amended; the per-topic confirmations are owner rulings.

## 2. Background

- **Convention.** CP-10 (`RUN/CANONICAL_SITUATIONS.md:83`) routes a declared-open hold settled in code without a ruling as IMPLEMENTED_DIFFERENTLY · AUTHORITY_UNCLEAR · PROJECT_BASELINE · OWNER (C6(d)). CONVENTIONS A1: code is evidence, not authority. A2: a merged PR does not amend a PROJECT_BASELINE claim.
- **Rulings.** DEC-009 leaves "exact dependency versions and component/state libraries" TBD (`F:execution/_Decomposition/SOFTWARE_DECOMP.md:600`). DEC-012 holds implementation-level TBDs open (cited by DEL-00-02 .s03 and DEL-07-08 CLM-026). DEC-074 O3 (`:665`) assigned root build evidence to DEL-10-04; whether it selects the package manager is disputed (T11 SS-03). No ruling selecting any of the 13 choices was found in the register or SOFTWARE_DECOMP §12 (T7-C01).
- **What the code does** (examples from the ledgers, spot-read at the freeze): no third-party GUI component or state library, state in React hooks (DEL-00-05 .s01 Notes); the desktop store resolves from the Tauri app-local data directory, `F:apps/desktop/src-tauri/src/lib.rs:548` (`app_store_path`); rule comparisons are exact, pinned by the DEC-022 corpus (DEL-06-02 CLM-015.r04 Notes); npm workspaces with a lockfile at `F:package.json` (T11 SS-03).
- **The split (T11 T-03).** 49 rows take CP-10; comparable rows take STALE_SETUP_SPECIFICATION or DOC_BEHIND_CODE with no owner. The split holds on one topic (schema file layout: DEL-02-01 CLM-005.s01 and DEL-02-02 CLM-015.r11 are CP-10; DEL-06-01 CLM-012.r03 is DOC_BEHIND_CODE) and inside single ledgers (DEL-10-03 REQ-09 against CLM-015; DEL-07-02 CLM-010.r02 against .r01/.r04/.r05). DEL-04-01 cites a worker-notebook "resolved-TBD rule" (`DEL-04-01:SOW#CLM-012.r01` Notes). DEL-07-08 CLM-026 draws the line at DEC-012 implementation-level TBDs.

## 3. Options

**Question 1 (treatment rule).**
- **1a. Keep CP-10.** Every settled hold needs an owner confirmation. The DOC_BEHIND_CODE rows on the same topics (T11 T-03) are then under-routed and should join the topic rulings.
- **1b. Resolved-TBD.** DEC-012 implementation-level TBDs settled in code are F3 record catch-up. The 50 rows move to R5 record repair with no ruling; CP-10 is amended through HELPS_HUMANS for the run's conventions.
- **1c. Line at DEC-012.** Implementation-level TBDs (DEC-012) are catch-up; holds that a DEC or SOFTWARE_DECOMP entry names as needing human authority (for example DEC-009's component/state library clause, "routed to a human ruling" in DEL-00-05 .s01) stay CP-10.

**Question 2 (per topic), as T7-C01 states them:**
- **(a) Confirm** the implemented choice and record it as a ruling or accepted implementation-level choice (offered explicitly in DEL-00-02 .s03 Remaining).
- **(b) Rule a different choice**; the code becomes the thing to catch up.
- **(c) Keep the hold open**, with the code marked provisional.

Parts that stay open whatever is ruled (T7-C01): code-generation tooling (DEL-02-01, 02-02), diagnostic code namespace (DEL-00-06), lint tooling (DEL-00-02), transport / FEA format / adapter (DEL-10-03).

| # | Topic | Implemented choice (evidence) | Rows | Keys |
|---|---|---|---|---|
| 1 | GUI component and state library | React built-ins, no third-party library | 15 | `DEL-00-05:AB#open-holds-and-routed-questions.s01`; `DEL-07-01:SOW#CLM-004.r05`, `#CLM-005.r05`, `#CLM-012.r05`, `#CLM-028.r03`; `DEL-07-02:SOW#CLM-004.r03`, `#CLM-010.r02`; `DEL-07-03:SOW#CLM-003.r05`, `#CLM-012/DEL-07-03-R-009`, `#CLM-014/DEL-07-03-V-004`, `#CLM-035`; `DEL-07-05:SOW#CLM-005.r02`, `#CLM-033.r02`; `DEL-07-08:SOW#CLM-006.s01`, `#CLM-026` |
| 2 | Undo/redo storage mechanism | as landed (ledger Notes) | 1 | `DEL-00-05:AB#open-holds-and-routed-questions.s02` |
| 3 | Package manager (lint stays open) | npm workspaces + lockfile (`F:package.json`) | 1 | `DEL-00-02:AB#open-holds-and-routed-questions.s03` |
| 4 | Severity taxonomy (namespace stays open) | as landed | 1 | `DEL-00-06:AB#open-holds-and-routed-questions.s01` |
| 5 | Schema file layout and QuantityKind (dimensionless, ratio) classification | as landed | 4 | `DEL-02-01:SOW#CLM-005.s01`; `DEL-02-02:SOW#CLM-015.r03`, `#CLM-015.r07`, `#CLM-015.r11` |
| 6 | Status axes split | as landed | 5 | `DEL-02-03:SOW#CLM-004.s01`, `#CLM-005`, `#CLM-010.r12`, `#CLM-026`, `#CLM-027` |
| 7 | Redistribution / review vocabulary | as landed; draft `docs/IP_AND_DATA_BOUNDARY.md` §4 defines most of it | 2 | `DEL-03-07:SOW#CLM-026.r01` (CONTESTED); `DEL-06-04:SOW#CLM-028/C-06-04-002` |
| 8 | Rule comparison tolerance | exact comparison, DEC-022 corpus | 2 | `DEL-06-02:SOW#CLM-006.r03`, `#CLM-015.r04` |
| 9 | Benchmark fixture schema and result-envelope comparison format | as landed | 2 | `DEL-09-01:SOW#CLM-008.r03`, `#CLM-035.r03` |
| 10 | API and handoff schema placement and vocabulary | as landed | 5 | `DEL-10-01:SOW#CLM-034.r02`; `DEL-10-03:SOW#CLM-013/DEL-10-03-REQ-09`, `#CLM-035.r01`, `#CLM-035.r04`, `#CLM-035.r05` |
| 11 | Private-data root / desktop store root | Tauri app-local data dir (`lib.rs:548`) | 8 | `DEL-12-01:SOW#CLM-003.r06`, `#CLM-004.r03`, `#CLM-010/LFSP-REQ-005`, `#CLM-013.s02`, `#CLM-024.r03`, `#CLM-031.r01`, `#CLM-033/LFSP-OI-002`; `DEL-12-02:SOW#CLM-037/REXC-OI-010` |
| 12 | REXC-CON-002 explicit local-private intent | as landed (worker-raised; not verifier-confirmed per W3) | 1 | `DEL-12-02:SOW#CLM-038/REXC-CON-002` |
| 13 | Unmatched-classification enum | as landed (W3 item; worker-raised) | 3 | `DEL-14-05:SOW#CLM-006.r02`, `#CLM-011.r03`, `#CLM-013.r03` |

Topic assignment is by this task from the ledgers' ClaimSummary text and T7-C01's groups; it is not a sealed field.

**Consequences.** Confirmations (a) produce record repair only. Any (b) produces a code-fix brief on that topic's code. (c) leaves rows open and code provisional. Under 1b, no per-topic ruling is needed and all 50 rows move to H4. Topic 11 bears on SPEC §4.4 (private-data root) and on B12 (private-library storage roots, a separate held selection). Topic 3 interacts with DEC-074 O3.

## 4. Evidence and reliability

| Source | Shows | Reliability |
|---|---|---|
| CP-10, C6(d), A1, A2 (`RUN/CANONICAL_SITUATIONS.md:83`; `RUN/CONVENTIONS.md`) | The routing rule | RULED run conventions |
| `SOFTWARE_DECOMP.md:600` (DEC-009), `:665` (DEC-074) | Which choices are held open | Governing sources |
| T7-C01 (`RUN/R3/TASKS/T7_CLASSES.md`) | 50 rows, groups, options | R3 task proposal over verifier-checked ledgers |
| T11 T-03, SS-03 (`RUN/R3/TASKS/T11_METHOD.csv`) | The corpus split; the npm workspace disagreement | R3 scripted screen plus hand review |
| W2 list (`RUN/WAVES/W2/W2_GATE_ASSESSMENT.md`, "Owner rulings needed"); W3 list (`RUN/WAVES/W3/W3_ASSESSMENT.md:83`) | Owner items; W3 marks the store root, unmatched list and REXC-CON-002 as worker-raised, not verifier-confirmed | Agent 0 assessments |
| `F:apps/desktop/src-tauri/src/lib.rs:548`; `F:package.json` | Implemented choices for topics 11 and 3 | Frozen code, read by this task |

Verified: the CP-10 text, the rulings' TBD clauses, the two code facts above. From worker notes only: the implemented choices for topics 2, 4–10, 12, 13, and all three W3 authority items.

## 5. Affected claims

**This packet's portion: 50 rows**, the whole of T7-C01 (Authority OWNER). Filter: `CLASS_ASSIGNMENTS.csv` `ClassID == 'T7-C01'`. Packages PKG-00, 02, 03, 06, 07, 09, 10, 12, 14; 20 deliverables.

**Exceptions kept visible.** `DEL-03-07:SOW#CLM-026.r01` is CONTESTED: whether draft `IP_AND_DATA_BOUNDARY.md` §4 counts as governing decides both its CP-10 treatment and its owner routing. That draft-policy question is C6's; this packet only needs its answer for topic 7. `DEL-07-01:SOW#CLM-005.r05` is FIRM (effective AuthorityNeeded NO; the correction says CP-10 and OWNER).

**Rows on other routes that the rule affects (not claimed).**
- `DEL-00-01:AB#open-holds-and-routed-questions.s01` (T4B-C08, R5): CONTESTED; the verifier would split the state-library clause into a CP-10 `.sNN` like DEL-00-05 .s01 (topic 1).
- DOC_BEHIND_CODE rows on the same topics (T11 T-03): `DEL-06-01:SOW#CLM-012.r03`, `DEL-10-03:SOW#CLM-015`, `DEL-07-02:SOW#CLM-010.r01`, `.r04`, `.r05`, `DEL-10-04:SOW#CLM-031.s03`, `DEL-04-01:SOW#CLM-012.r01`. Under 1a they join the topic rulings; under 1b they stay on H4.
- `DEL-10-04:SOW#CLM-013.s01` and `#CLM-004.r07` (ALIGNED "under DEC-074 O3", T11 SS-03).

**Rows known only from `OtherCorrections`** (not divergent): `DEL-16-02:STATUS#remaining/R01` (OBSERVED: worker asked whether CP-10 applies to the center-of-gravity hold; not decided), `DEL-02-01:SOW#CLM-024` (CONTESTED: "SCA-001 TBDs are not silently resolved" kept ALIGNED while CLM-005.s01 is CP-10).

## 6. Risks

- **Undecided.** Unruled implementation choices become de facto baseline, and deliverables keep declaring holds the product no longer honours. 15 rows depend on the GUI library choice alone (T7-C01). The same topic is routed to the owner in one ledger and to catch-up in another.
- **1a.** About 13 rulings for choices the owner may regard as routine; the DOC_BEHIND_CODE rows need re-reading.
- **1b.** Loses the owner's sight of choices that a DEC named as needing human authority (DEC-009's library clause); amends a RULED convention mid-run.
- **1c.** Needs a per-hold judgement of which TBDs a DEC reserved; a boundary dispute can recur.
- **Per topic (b).** Rework on landed code in many surfaces (topic 1 touches six deliverables).

## 7. Recommended routing

No recommendation on question 1; owner's call. For question 2, the evidence supports taking the GUI-library topic first, as one ruling closes 15 rows, and taking topics 11–13 only after a verifier or the owner confirms the worker-raised facts (W3 marks them unconfirmed).

## 8. On-ruling mechanism

- **Rulings (1a/1c with (a) or (c)).** The owner records each topic ruling in the register and SOFTWARE_DECOMP §12 (a DEC entry, or an accepted-implementation-choice record). That authorises an R5 record-repair tranche of the listed SOW, AB and CONTEXT rows (cite the ruling, drop or keep the TBD), plus the matching governance text: SPEC §4.4 for topic 11, the DEL-00-05 ArchitectureBasis hold for topics 1–2 (T7-C01).
- **Per topic (b).** A CODE_FIX_CANDIDATE brief (H2) for that topic, under an accepted production brief; R5 repair after it lands.
- **1b.** A convention amendment to CP-10 through HELPS_HUMANS (run conventions), then the 50 rows join the H4 R5 tranche with no per-topic ruling.

Nothing executes until the owner acts. R5 needs separate authorisation (D-73).

## 9. Dependencies

- **Depends on:** C6 (draft-policy question) for topic 7's `DEL-03-07` row only.
- **Interacts with:** B12 (private-library storage roots and secret provider; distinct from topic 11's private-data root but adjacent), A8 (secret provider), C6 (the diagnostics boundary split, for topic 4's namespace part).
- **Blocks:** H4 repair of the 50 rows and of `DEL-00-01:AB#open-holds-and-routed-questions.s01`; any H2 brief arising from a (b) ruling.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081). No certification, code-compliance, professional-approval or engineering-acceptance claim is made.
