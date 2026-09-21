VERDICT: FINDINGS

Reviewed HEAD: `26e617022b7898c875053177a40b46a0ab1e838a` on `claude/piping-recon-r0-20260921`. The candidate is `e631e078643f317c4ed5d8c3284f13d3db8eecd7..HEAD`: 72 files changed. The freeze checkout is at `00115c71931bcae79909602d653740d3bb72dfa1`. Two untracked files that Agent 0 is writing, `briefs/R1-INV_area_inventory_brief.md` and `briefs/R1-INV_areas.json`, are outside the candidate and were not reviewed.

**Checks that passed:**
- **Reruns are byte-identical.**
  - Rerunning `extract_claims_v2.py` gives the same `CLAIM_KEYS_V2.csv`: 12,854 units, 7,257 required.
  - Rerunning `build_canonical.py` gives the same `EVIDENCE_MAP.csv` and `CANONICAL_ASSIGNMENTS.csv`.
- **Hashes match.** Every hash in `RUN_STATE.jsonl` (`SEALED_INPUTS`, `SEALED`, `RETURN`, `BOUND_INPUTS`), `RUN_BASIS.md` and `WORK_GRAPH.json` matches its file. So do all pinned method and authority blobs.
- **Owner quotations match.**
  - The R0 ruling block is 1,131 bytes with SHA `0b2502d9…`.
  - Direction 5 is 88 bytes (`1baa99b7…`) and 140 bytes (`41221e64…`).
- **Tree bindings hold.**
  - The PR #834 head's Piping tree is `38cbfc64…`.
  - The 9d55 sweep differs from the freeze only under `execution/`.
- **Gate evidence is verified.** All 20 `GATE_EVIDENCE` files match their manifest by blob and SHA-256, and match the source commit `44a30112`.
- **The D-73 addendum is consistent with the owner's words.**
- **No protected data; one exception to repo-relative paths.** CI tokens are masked. The absolute paths are covered in finding 16.
- **The departure's counts are correct.**
  - There are 5,597 `.rNN` keys. Making them mandatory would add 76.8% to v1's 7,285 rows.
  - Under v2, rows needing judgment are 7,257 − 809 pre-typed − 811 canonical = 5,637. That is a 22.6% reduction, so the conflict with §5's 20–25% is real.
- **`SharedTextCount` and `DuplicateOf` are internally consistent.** Recomputing both gave zero mismatches.
- **Section parenting, AC/VER/OUT bounds, `.rNN` keys and the merged DEL-07-09 `ROWS` are correct.** Checked on four deliverables:
  - DEL-04-04 (Scope of Work);
  - DEL-00-05 (architecture basis);
  - DEL-07-09 (bespoke, 28 merged rows);
  - DEL-07-02 (11 Remaining items, the same as v1).
- **Evidence map.** The 5 rows I spot-checked match their parity files and the frozen SOW hashes: DEL-04-04, 07-02, 11-01, 12-03 and 17-05.

## Findings

1. **BLOCKING — the CS-06 Package Reference check covers one field only, so 16 rows are wrongly ALIGNED.**
   - Where: `tools/build_canonical.py:137-140`; `CANONICAL_SITUATIONS.md:35` ("All 306 are OK at the freeze").
   - Evidence:
     - The check compares only `Package Scope`. The ALIGNED disposition covers the whole block, which also carries `Package Assigned Scope Items` and `Package Exclusions`.
     - I compared those fields against the SOFTWARE_DECOMP rev 0.12 package table. 16 blocks list assigned scope items that differ from rev 0.12. For example, SOW-064 is missing for PKG-01 and SOW-065 for PKG-02.
     - Affected: DEL-01-01..04, DEL-02-02..05 and DEL-07-01..08.
     - The Objective Support and Scope Coverage checks are not vacuous. All 102 of each have non-empty ID sets and also agree with the rev 0.12 DEL table.
   - Smallest fix:
     - Compare the package name, the assigned-items set and the exclusions too.
     - Regenerate the outputs and correct the "306 OK" sentence.
     - Update the hashes in `BOUND_INPUTS` and `RUN_BASIS`.

2. **BLOCKING — extractor v2 loses one real claim (DEL-17-06 CLM-042).**
   - Where: `tools/extract_claims_v2.py:113-118` and `:132-137`.
   - Evidence:
     - CLM-042 ("Versioned 0.2 JSON hash successor (D-67)", SOW lines 525–531) is written unquoted.
     - v2 ends a claim block at the first unquoted line, so the CLM-042 block comes out empty and is pre-typed `NON_NORMATIVE`.
     - Its text falls to `#governing-values-and-decisions-axiology`. That section is also pre-typed, because the wrapper test only looks at lines before the first child heading.
     - Result: no assessed required unit covers the D-67 ruling text.
     - A corpus scan found this is the only case. The other 206 pre-typed CLM blocks carry only a `&lt;!-- … --&gt;` marker.
   - Smallest fix:
     - When the first non-blank line after a `###` heading is unquoted and is not an ID bullet, bound the block at the next heading.
     - Compute the wrapper's own lines over every line the section owns.
     - Regenerate the keys.

3. **ACTIONABLE — `DuplicateOf` and `SharedTextCount` hash the heading line, so repeated declarations are not caught.**
   - Where: `extract_claims_v2.py:124-125` and `:276-288`.
   - Evidence:
     - Only 1 `DuplicateOf` exists in the whole corpus.
     - Yet 25 PDU-054/055 "current declaration" blocks have identical bodies within 15 deliverables. The heading line carries the differing CLM number, so their hashes never match. Example: DEL-02-02 CLM-010, CLM-021, CLM-031 and CLM-048.
     - §5 item 6 therefore has no practical effect.
   - Smallest fix: hash the block body without its heading line, or normalise `CLM-nnn` in the heading.

4. **ACTIONABLE — the evidence map only finds parity files at one path shape.**
   - Where: `build_canonical.py:87`.
   - Evidence:
     - The map only picks up `…/checks/DEL-*/parity.md`. 21 SOW deliverables get no row.
     - Other parity records exist under `WORKING-P1-PKG03/manager-validation/`, `…/members/`, and `SOW-PACKAGE-BATCH-ADOPTION-20260714-01/…/parity-{1,2}.md`.
     - DEL-03-01, 03-02, 03-05 and 03-07 each have a PASS parity whose hash equals the frozen SOW. Under CP-09 those rows would be ALIGNED, but the map sends workers to the "no record" branch.
   - Smallest fix: widen the search to `parity*.md` or `parity_*.md` under the SOW-STAGE2 and batch-adoption runs, choosing the latest per deliverable. Alternatively, state the map's scope in CP-09 and in `RUN_BASIS`.

5. **ACTIONABLE — CS-07 marks all Context Envelope notes as making no claim, but some do.**
   - Where: `CANONICAL_SITUATIONS.md:36`; `build_canonical.py:149-150`.
   - Evidence: several envelope notes state normative constraints. Examples:
     - "Schema deliverable must use JSON Schema 2020-12 baseline from SCA-001 …";
     - "Operations are the only model mutation route for GUI and agent proposals";
     - "Local-first policy must align with the SCA-003 … no-network/no-cloud MVP posture";
     - "No code formulas".
   - Smallest fix: limit CS-07 to Context Budget QA and the envelope size letter. Judge envelope notes normally, or require a `.sNN` for any normative note.

6. **ACTIONABLE — 18 shared-text hashes have no canonical row.**
   - Where: `CANONICAL_SITUATIONS.md:15-19`, against the adopted `R0_REVIEW.md` §5 item 5 ("one canonical row per hash") and the owner package ("per shared-text hash").
   - Evidence:
     - 18 shared hashes covering 50 required units have no row.
     - They include the exact F2 conflict cases: DEL-07-01, 07-02, 07-06 and 07-09 Remaining R03/R04, and 07-02 R11 against 07-09 R09.
     - They also include DEL-17 Architecture Basis Injection (8 units), PKG-00 `STATUS#history` (4) and DEL-15 Scope Detail (3).
   - Smallest fix: add keyed rows per hash, or record why pattern coverage (CP-07 and CP-08) suffices.

7. **ACTIONABLE — the precedence clause in `CONVENTIONS.md` is inverted.**
   - Where: `CONVENTIONS.md:7-8` ("the ruling governs, then this file").
   - Evidence: as written, this file overrides the `R0_REVIEW.md` §4–§7 texts that the owner adopted "as proposed".
   - Smallest fix: "the ruling, including the R0_REVIEW §4–§7 texts it adopts, governs; then this file."

8. **ACTIONABLE — `CONVENTIONS.md` drops candidate text that §4 kept.**
   - Where: `CONVENTIONS.md:182-198`.
   - Evidence:
     - §4 marks both C3 and C4 "Append", so the candidate text should remain.
     - C3 drops "specification" and the INVARIANT subject list: professional boundary, IP/data, security and privacy, protected checks, engineering validation.
     - C4 drops the value glosses: FROZEN_CONTRACT (hash, result-semantics or schema versions), PROTECTED_CHECK (tests, tolerances, oracles, limits) and RULED_CRITERION (for example D-68, D-72).
     - The new PROTECTED_CHECK sentence defines the class by event ("removed, weakened or contradicted"), while the line above it says the class names the diverging artifact.
     - The "stale test count" half of that sentence is well founded (F12 and the named DEL-07-09 repair).
   - Smallest fix: restore the candidate text, and word PROTECTED_CHECK as "the diverging artifact is a protected check (…) that was removed, weakened or contradicted; a stale count in prose is not."

9. **ACTIONABLE — C1 does not document how optional `.rNN` keys work, and the validator falsely blocks a conforming split.**
   - Where: `CONVENTIONS.md:152-154`; `validate_ledger_v2.py:216-223`.
   - Evidence:
     - C1 says only "Table rows are issued as deterministic `.rNN` keys". It does not say they are optional and all-or-none, or that a split parent must be `CONTAINER`.
     - The validator forces the parent to `CONTAINER`, which contradicts C1's "a block that has children and substance of its own is assessed directly".
     - Synthetic test 04 reproduces the false block. It splits the rows of DEL-00-05 `AB#realized-artifacts` and assesses the block directly. The same blocks is `AB#resolved-decisions…`, whose own prose line 31 carries the calibrated "§8.4" finding.
     - Test 05 shows the determinism gap: `.sNN` sub-claims can replace a block's issued `.rNN` keys, and the ledger passes.
   - Smallest fix:
     - Document the optional and all-or-none rule in C1.
     - Allow a non-`CONTAINER` parent when it has its own substance.
     - Reject `.sNN` on a block that has `.rNN` keys.

10. **ACTIONABLE — validator v2 lets several non-conforming ledgers pass.**
    - Where: `validate_ledger_v2.py:69`, `:103-104`, `:147-173` and `:235-240`.
    - Evidence: each of these synthetic ledgers returned PASS:
      - empty evidence columns (`SPECIAL` includes `""`);
      - `LIFECYCLE_REASSESSMENT_REQUIRED` with no `FindingGroup` (breaks C6(d));
      - `ACCEPTED_DIVERGENCE` with `DecisionBasis=NOT_APPLICABLE`;
      - `ACCEPTED_DIVERGENCE` citing DEC-101 with a cause other than `RENAME_OR_IDENTITY`;
      - `REMAINING_STATE_MISMATCH` on a unit that is not a Remaining item (breaks C6(c));
      - `GATE:CONVENTIONS.md`, since any file in the run folder is accepted;
      - a quiet row with layers `NONE;RECORD`;
      - a `UnitKind` that differs from the key's;
      - `CANONICAL_DEPARTURE` with no justification text;
      - reverse answers: `CLAIMED_BY` with an empty ClaimKey, the `STATUS#remaining` block key, and a duplicate CapabilityID.
    - The adopted §4 Part D warning for same-`TextSHA256` or same-`CanonicalSituation` rows that differ is not implemented at all.
    - Smallest fix: add these checks, plus a multi-ledger consistency mode.

11. **MINOR — `RESUME.md` is stale.**
    - Where: `RESUME.md:10-12` and `:18`.
    - Evidence: it still points at `tools/validate_ledger.py` (v1) and says "Current phase: R0 calibration".
    - Smallest fix: point it at v2, `CLAIM_KEYS_V2.csv` and `CONVENTIONS.md`.

12. **MINOR — CP-04 has an unsourced basis and a possibly wrong `AuthorityNeeded`.**
    - Where: `CANONICAL_SITUATIONS.md:48`.
    - Evidence:
      - "Hard identifiers kept by owner ruling" is sourced only to an AgentRuns `ORCHESTRATION_PLAN.md:10`, which is a context record ("named exception to DEC-101 (iv)").
      - `AuthorityNeeded=NO` may not fit ruling item 3's "single R4 ruling".
    - Otherwise CP-04 treats rename residue as a finding and never as an accepted divergence, as the ruling requires.
    - Smallest fix: cite a governing basis or an A3a record, and see the owner item below.

13. **MINOR — CS-04 bundles substance under a pointer-staleness row.**
    - Where: `CANONICAL_SITUATIONS.md:33`.
    - Evidence: the row is LOCAL_DESIGN/NO, but it also covers "Resolved Baseline", the Applicable Basis IDs and the PKG-00 `SEMANTIC_READY` state. The `.sNN` guidance covers only "Still TBD".
    - Smallest fix: extend the `.sNN` rule to any of those parts that diverge.

14. **MINOR — CS-01 reverses a unanimous calibration choice without saying why.**
    - Where: `CANONICAL_SITUATIONS.md:30`.
    - Evidence: CS-01 picks `STALE_REVIEW_OR_EVIDENCE`, where all calibration workers used `STALE_SETUP_SPECIFICATION`. Both readings are defensible because C6(c)'s "setup-era origin text" and "revision pins" overlap.
    - Smallest fix: state the reason in the Basis column.

15. **MINOR — consolidation slips in `CONVENTIONS.md`.**
    - Agent 0 clarifications are labelled RULED. These are the PROTECTED_CHECK sentence, the `AuthorityNeeded` guidance, "Start with the evidence map", and the pre-type override.
    - Small candidate texts are dropped without note:
      - "Workers never mint keys other than `.sNN`";
      - "MEMORY is a single history unit";
      - BaselineClass empty on quiet rows;
      - A2's "by itself".
    - Changes from the adopted texts:
      - the sentinel moves from Notes to the last field;
      - project-root tokens are no longer allowed, although §4 Part D lists them;
      - the history rule becomes a closed list instead of "every HISTORY-typed unit".
    - Ruling item 3's R3 clustering and item 4's "ISSUED change path" are omitted.
    - Smallest fix: add an AGENT origin label, and note each narrowing.

16. **MINOR — gate-evidence copies contain absolute local paths.**
    - Where: `GATE_EVIDENCE/B4_4_SWEEP_9D55/START.json:8-15`, `registered/SWEEP_*.json` and the `.log.gz` files.
    - Evidence: they contain paths such as `/Users/ryan/...` and `/tmp/...`. They are blob-verified verbatim copies, so editing them would break custody.
    - Smallest fix: disclose this in `SOURCE_MANIFEST.tsv` or `RUN_BASIS`.

17. **MINOR — three sealed calibration notes lack the claim fence.**
    - Files: `DEL-01-01_notes.md`, `DEL-04-04_notes.md` and `DEL-07-09_notes.md`.
    - Smallest fix: record this in `RUN_STATE`. Do not patch sealed files.

18. **MINOR — the `build_canonical.py` docstring overstates its checks.**
    - Where: `build_canonical.py:13-15` and `:118-122`.
    - Evidence: the docstring claims a ScopeLedger.csv comparison. `sreg` is loaded but never used.
    - Smallest fix: use it or drop it from the docstring.

On ruling fidelity overall: the three named Agent 0 additions are faithful consolidations. C6(g) and C6(h) carry over correctly, because §4 C6 adds to the candidate rules rather than replacing them, and (g)'s A3a clause matches ruling item 2. The `AuthorityNeeded` guidance meets §7's request for definitions and agrees with C3. The C4 sentence needs the rewording in finding 8.

## Owner items (need the owner rather than a repair)

- **The `.rNN` departure.** The conflict is real (counts above), and the validator enforces all-or-none, so coverage stays provable. That holds once finding 9 closes the `.sNN` bypass. It still changes an adopted §5 text ("one row per issued key"), so it needs owner confirmation, at the latest at the first-wave checkpoint. The alternative is to reverse it as `RUN_STATE` describes.
- **CP-04.** Whether rename residue rows need owner authority (`AuthorityNeeded`), given ruling item 3's single R4 ruling. Whether the four kept "hard identifiers" rest on an owner decision; the only record found is an AgentRuns plan.
- **D-73 addendum.** It reads the owner's question ("What value is there in running even a trimmed test suite?") as a direction. The R0 ruling's adoption of the `GATE:` token implicitly confirms that reading; no action is needed unless the owner wants explicit confirmation.

## Validator outputs (`PYTHONDONTWRITEBYTECODE=1`)

- `python3 tools/validation/validate_claims_language.py`: `VALID claims-language surfaces: 356 files scanned; DEC-081 registry taxonomy satisfied` (exit 0).
- `python3 tools/validation/validate_piping_loop_receipts.py --repo-root .`: `VALID …/projects/chirality-piping/loop/LOOP_RECEIPTS.md: frozen through Receipt-44; versioned receipt contract satisfied` (exit 0).
- `python3 tools/practitioner_harness/harness.py self-check`: exit 0, **0 BLOCK**. Severities were INFO 14, NOT_APPLICABLE 1, REVIEW 4, WARN 112. No finding touches this PR's files.

Synthetic ledgers and check scripts are in `/private/tmp/claude-501/-Users-ryan-dev-chirality--claude-worktrees-task-management-gen-pass-518da2/efe0b4ff-c1b6-4a97-af70-b04220464651/scratchpad/review/` (`val/`, `cs06*.py`, `parity.py`, `lost.py`).

END-OF-RETURN