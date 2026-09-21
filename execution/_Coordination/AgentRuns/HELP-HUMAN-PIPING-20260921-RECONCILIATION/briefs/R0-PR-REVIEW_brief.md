# Brief — R0 PR review (fresh, read-only)

Parent: HELP_HUMAN Agent 0, run `HELP-HUMAN-PIPING-20260921-RECONCILIATION`
(D-73 / DEC-110). Role: TASK (Type 2), fresh context, **read-only**. Do not
edit anything in the repository. Do not delegate.

The launch message supplies:
- `{REPO}`: the repository checkout;
- `{FREEZE}`: a read-only checkout of the frozen state
  `00115c71931bcae79909602d653740d3bb72dfa1`.

`RUN` means
`projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS`.

## Candidate

The complete diff `e631e078643f317c4ed5d8c3284f13d3db8eecd7..HEAD` on branch
`claude/piping-recon-r0-20260921`, as it stands when you start. Record the HEAD
you reviewed.

The diff contains:
- the D-73 gate-evidence ruling addendum and register pointer;
- `RUN_BASIS.md`, `RUN_STATE.jsonl`, `RESUME.md` and `GATE_EVIDENCE/`;
- the v1 calibration tooling, `CLAIM_KEYS.csv` and the calibration ledgers,
  which are calibration evidence only;
- `R0_REVIEW.md`, `R0_OWNER_PACKAGE.md` and `R0_RULING.md`;
- the bound `CONVENTIONS.md`;
- `tools/extract_claims_v2.py` and `CLAIM_KEYS_V2.csv`;
- `tools/build_canonical.py`, `CANONICAL_ASSIGNMENTS.csv`,
  `CANONICAL_SITUATIONS.md` and `EVIDENCE_MAP.csv`;
- `tools/validate_ledger_v2.py`;
- run-record updates, and the sealed briefs that were used.

## Check

1. **Ruling fidelity.** Does `CONVENTIONS.md` implement exactly what the owner
   ruled? The ruling is in `RUN/R0_CALIBRATION/R0_RULING.md`, including its
   verbatim block. It adopts `R0_REVIEW.md` §4–§7 as proposed, plus items 2–4.
   Report:
   - every place where `CONVENTIONS.md` adds, drops or changes an adopted
     text;
   - any clarification that goes beyond the adopted text. Say whether it is
     a faithful consolidation or needs the owner.

   Known Agent 0 additions to check:
   - C4's sentence on `PROTECTED_CHECK`;
   - Part D's `AuthorityNeeded` guidance;
   - C6(g) and C6(h), carried over from the candidates.
2. **The disclosed departure.** `.rNN` table-row keys are issued as optional
   (all-or-none per block), not as mandatory rows. See `RUN_STATE.jsonl`
   `DISCLOSED_DEPARTURE` and the extractor docstring. Is the stated conflict
   real? §5 item 2 against its expected 20–25% reduction: verify the counts.
   Does the resolution keep coverage provable? Is it within Agent 0's
   discretion, or does it need the owner?
3. **Extractor v2 correctness.**
   - Rerun it into a scratch path and confirm byte identity with
     `CLAIM_KEYS_V2.csv`.
   - On at least four deliverables of different representations (a SOW, an
     architecture basis, DEL-07-09, and one with Remaining items), check:
     - section parenting;
     - claim-block bounds (unquoted AC/VER/OUT bullets parented to their
       section);
     - `.rNN` keys;
     - pre-typing: no real claim should be pre-typed `NON_NORMATIVE`;
     - the merged DEL-07-09 CSV rows;
     - `SharedTextCount` and `DuplicateOf`.
   - Report any lost unit: text in a v1 unit that no required v2 unit covers.
4. **Canonical table.**
   - Rerun `build_canonical.py` into scratch and confirm the outputs are
     byte-identical.
   - For each CS situation, spot-check at least three assignments against
     `{FREEZE}`. Is the situation matched correctly? Are the field values
     right under `CONVENTIONS.md`?
   - Check that the CS-06 mechanical comparisons actually compare what they
     claim. Every one returned OK; look for a vacuous check.
   - Are the CP pattern treatments consistent with `CONVENTIONS.md` and the
     ruling, especially CP-04, which must treat rename residue as a finding?
5. **Evidence map.** Spot-check five rows against the parity files and the
   frozen SOW hashes.
6. **Validator v2.** Does it enforce Part D and the C-rules it claims to? Are
   there gaps that would let a non-conforming ledger pass, or false blocks on
   a conforming one? Exercise it on synthetic ledgers in scratch.
7. **Records.**
   - Hashes stated in `RUN_BASIS.md`, `RUN_STATE.jsonl` and `WORK_GRAPH.json`
     match the files.
   - Owner quotations match their stated SHA-256.
   - Repository-relative paths only, in committed records other than briefs.
   - No protected or private data.
   - The claim fence is present.
   - The D-73 addendum is consistent with the owner's words.
8. **Validators.** Run and report:
   - `python3 tools/validation/validate_claims_language.py`;
   - `python3 tools/validation/validate_piping_loop_receipts.py --repo-root .`;
   - `python3 tools/practitioner_harness/harness.py self-check` (report
     BLOCKs only).

   Run all with `PYTHONDONTWRITEBYTECODE=1`.

Use scratch files only under the session scratch path given in the launch
message, in a `review/` subfolder.

## Return

A single message:

1. first line `VERDICT: PASS` (no actionable finding) or `VERDICT: FINDINGS`;
2. the reviewed HEAD;
3. numbered findings, each with file and line, severity (`BLOCKING`,
   `ACTIONABLE`, `MINOR`), evidence and the smallest fix;
4. a separate list of items that need the **owner** rather than a repair;
5. validator outputs;
6. last line `END-OF-RETURN`.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
