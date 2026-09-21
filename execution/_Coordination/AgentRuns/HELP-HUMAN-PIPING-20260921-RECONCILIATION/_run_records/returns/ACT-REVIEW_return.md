VERDICT: FINDINGS

Scope reviewed: the complete diff `00115c719..5438c1c98`, 10 files, all under `projects/chirality-piping/`. Nothing in it writes outside the stated scope. It adds no deliverable, code, DAG, lifecycle or instruction edits and does not activate R5 or R6.

**1. The pinned profile makes binding two method extensions the packet says are not adopted**
- **Severity:** BLOCKING
- **Where:** `projects/chirality-piping/docs/RECONCILIATION_PROFILE.md`, lines 95–104 (§5 "Authority tiers") and lines 80–81 (§4 layer 4). The conflicting statement is in `projects/chirality-piping/execution/_Coordination/_DECISIONS/D-73_reconciliation_activation.md`, lines 73–78 (Item 2).
- **What is wrong:** Item 2 says authority tiers and the extended ledger schema are **not** adopted by this ruling. It says they are proposed and ruled at R0. The same Item pins the profile as ruled method, and the owner ruled "Items 1–3 as written".
  - Profile §5 says "Every non-aligned claim carries one tier" (`LOCAL_DESIGN`, `PROJECT_BASELINE`, `INVARIANT`).
  - Profile §4 layer 4 says "Each row records its baseline class and that class's change path".
  - `PLAN.md` Step 2.5 lists both `AuthorityTier` and `BaselineClass` as extensions for the R0 addendum.
- **Consequence:** The ruled instrument contradicts itself. R0 workers could cite the pinned profile to apply extensions the owner has not ruled on.
- **Note:** Profile §4's other requirement, that every row record which divergence layers apply, is not affected. The contract's "Project divergence is preserved" invariant already requires it.
- **Smallest fix:** Mark profile §5 and the per-row baseline-class sentence as "candidate for the run's R0 conventions; not in effect until ruled at R0". Then record the new profile SHA-256 and this change under "Changes after ruling" in `D-73_RULING_2026-09-21.md`, which is the mechanism the ruling already provides. Alternatively, return the contradiction to the owner as an `AUTHORITY_CONFLICT`, as profile §1 prescribes.

**2. The packet cites the wrong kernel section for the profile**
- **Severity:** MINOR
- **Where:** `D-73_reconciliation_activation.md`, line 117 (Item 5a).
- **What is wrong:** The packet calls the profile "kernel §5's recurring process asset for this project".
  - Kernel §5(3) defines that asset as checking-entry profiles and maturity feedback. For Piping that is `docs/CHECKING_ENTRY_PROFILES.md`, which Item 5d makes read-only.
  - The profile itself, at line 8, correctly calls itself the kernel §7 adoption record.
- **Smallest fix:** Replace "kernel §5's recurring process asset" with "kernel §7's project adoption record".

**3. The register row's Decision cell still says "three run surfaces"**
- **Severity:** MINOR
- **Where:** `projects/chirality-piping/execution/_Coordination/_DECISIONS/_REGISTER.md`, line 110 (D-73 row, Decision cell).
- **What is wrong:** The cell ends "…; three run surfaces". Item 5 was amended before the ruling into the complete write and read-only boundary (5a–5d), so the wording is stale. The State and Ruling cells are correct.
- **Smallest fix:** Change it to "run surfaces and the complete write/read-only boundary".

**4. Machine-specific transcript path**
- **Severity:** MINOR
- **Where:** `projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260921-RECONCILIATION/OWNER_DIRECTIONS.md`, line 5.
- **What is wrong:** The line cites `~/.claude/projects/<project>/efe0b4ff-….jsonl`, a host-specific location. The Piping `AGENTS.md` requires repository-relative paths in durable records, not machine-specific ones. The ruling record, at line 23, cites only the file name.
- **Smallest fix:** Cite the session transcript file name only. This file is not pinned by any hash in the candidate except `WORK_GRAPH.json`'s record hash, which would need updating too.
- **Leave alone:** `instances/ROOT/ENTRY_BRIEF_2026-09-21.md` contains `~/.claude/plans/…`. It is hash-bound verbatim input, so it should stay as is.

**5. How the ruled-on bytes were produced is not written down**
- **Severity:** MINOR
- **Where:** `D-73_RULING_2026-09-21.md`, lines 21–24.
- **Packet:** The stated hash `0241867d…` is not the packet at either commit.
  - I reproduced it only by taking the candidate packet and replacing the §6 body with `*(Awaiting ruling.)*\n`.
  - The packet at the proposal commit `4a1b6fdd3` hashes to `73a70b85…`.
- **Profile:** The profile was also changed between the proposal commit (`ac29229a…`) and the ruling (`0930b22e…`).
  - The change adds the §6 evidence-root rows and the paragraph on writes outside the repository.
  - The packet's amendment note mentions only Item 5.
- **Smallest fix:** Add one sentence to "Accepted subject" stating the reproduction rule. Add a note that profile §6 was amended before the ruling together with Item 5.

**6. A read-only surface is named without a path**
- **Severity:** MINOR
- **Where:** `D-73_reconciliation_activation.md`, line 150 (Item 5d).
- **What is wrong:** "the Task Management register" is given without a path.
- **Smallest fix:** Add `execution/_Coordination/_TaskManagement/REGISTER.csv`.

**7. A step in the approved plan was dropped without a record**
- **Severity:** MINOR
- **Where:** `PLAN.md`, line 52.
- **What is wrong:** The approved plan says it will create `NOTICE_2026-09-21_RECONCILIATION_ACTIVATION.md`. The candidate has no such file. Packet Item 5b now allows notices only when another loop is affected. The change from the approved plan is not recorded in `HANDOFF_STATE.md` or `WORK_GRAPH.json`.
- **Smallest fix:** Add one line to the handoff saying the notice was dropped because no other loop is affected, or create the notice.

**8. The candidate convention on `## Remaining` sits beside a contract invariant without saying so**
- **Severity:** MINOR
- **Where:** `D-73_reconciliation_activation.md`, lines 90–92.
- **What is wrong:** The R0 candidate convention says existing `## Remaining` entries are stale and never authority. The contract's invariant says "Remaining is executable truth" (where the project uses it). Validity item 12 requires a final Remaining census. The packet does not flag the tension, so R0 could miss it.
- **Smallest fix:** Add a clause saying the R0 convention must be reconciled with this contract invariant and with validity item 12.
- **Kept separate from this finding:** The no-seeding variance itself is correctly described. Kernel §6 is where seeding comes from. The contract's "Activation before dispatch" invariant needs the ruling, scope, pinned method and run pointer, not seeding. The workflow's `method.md` has no seeding step.

**Checked and found consistent:**
- Freeze SHA, R0–R4 scope, 16-agent cap and opus-5 at high reasoning agree across the packet, ruling, register, `DEC-110`, `WORK_GRAPH.json`, handoff and `PLAN.md`.
- The corpus counts in the packet hold: 102 deliverables, 18 packages, 100 `IN_PROGRESS` / 1 `ISSUED` / 1 `OPEN`, and 98 of 101 scope surfaces last changed in 2026-07.
- The PR history figures hold: 979 Piping commits since `551f84ef6`, and `apps/desktop` +81,070 lines.
- The register row has 6 columns, like the header and the D-70–D-72 rows, and sits after D-72.
- `DEC-110` has 4 columns, like `DEC-100`–`DEC-109`.
- The ruling record's frontmatter matches the D-64 record's.
- The claim fence sentence is present in every new file.
- No protected or private data appears, and no absolute `/Users` or `/tmp` path.
- The owner's words are kept separate from Agent 0's reading throughout.
- Item 6 "topology" is my reading of the owner's "The roles and caps … are all acceptable". It is reasonable but inferred, not quoted.

**Not a finding:** The brief for this review is untracked under `briefs/`, and the `ACT-REVIEW` node in `WORK_GRAPH.json` does not yet carry a brief path or hash, launch mechanism, or return hash. Both should be committed with this review's return.

**Hash recomputations**

| Item | Stated | Recomputed | Match |
|---|---|---|---|
| OWNER_DIRECTIONS Direction 1 (1825 B) | ac397039…1a62 | ac397039…1a62, 1825 B | yes |
| Direction 2 (765 B) | ab4cc637…e297 | ab4cc637…e297, 765 B | yes |
| Direction 3 (110 B) | 78cff483…f4dd | 78cff483…f4dd, 110 B | yes |
| Direction 4 (82 B) | 94ebe9ac…b1ea | 94ebe9ac…b1ea, 82 B | yes |
| Ruling Act 1 (844 B) | 19d90885…b9c | 19d90885…b9c, 844 B | yes |
| Ruling Act 2 (39 B) | e40b902e…700c | e40b902e…700c, 39 B | yes |
| ENTRY_BRIEF (8784 B) | 3767db25…7823 | 3767db25…7823, 8784 B | yes |
| PLAN.md | 18d39600…6381 | 18d39600…6381 | yes |
| OWNER_DIRECTIONS.md (whole file) | 6454fbda…cd96 | 6454fbda…cd96 | yes |
| WORKFLOW.md blob / SHA-256 @00115c719 | b7aa037d… / 75948a77… | same / same | yes |
| execution.json | 4774e200… / 35e108bc… | same / same | yes |
| contract.md | dd997a40… / da47475c… | same / same | yes |
| method.md | 8bccdfb4… / d7b5e22a… | same / same | yes |
| DELIVERABLE_CONCORDANCE_METHOD.md | 137209cb… / abf3e78f… | same / same | yes |
| Pinned files unchanged at candidate | — | all five blobs identical | yes |
| Profile @ candidate vs ruling | 0930b22e…b06a | 0930b22e…b06a | yes (the proposal commit's profile was ac29229a…; see finding 5) |
| Packet as ruled (before §6) | 0241867d…a879 | 0241867d…a879 (candidate with §6 = `*(Awaiting ruling.)*\n`) | yes, by reconstruction |
| Piping tree @00115c719 vs @620ff6387 | 38cbfc64…efb | 38cbfc64…efb for both | yes |

**Validator outputs**

- `PYTHONDONTWRITEBYTECODE=1 python3 tools/validation/validate_claims_language.py` printed "VALID claims-language surfaces: 356 files scanned; DEC-081 registry taxonomy satisfied" (exit 0).
- `PYTHONDONTWRITEBYTECODE=1 python3 tools/validation/validate_piping_loop_receipts.py --repo-root .` printed "VALID …/projects/chirality-piping/loop/LOOP_RECEIPTS.md: frozen through Receipt-44; versioned receipt contract satisfied" (exit 0).
- The working tree is unchanged by this review. The only untracked path is the pre-existing `…/HELP-HUMAN-PIPING-20260921-RECONCILIATION/briefs/`.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

END-OF-RETURN