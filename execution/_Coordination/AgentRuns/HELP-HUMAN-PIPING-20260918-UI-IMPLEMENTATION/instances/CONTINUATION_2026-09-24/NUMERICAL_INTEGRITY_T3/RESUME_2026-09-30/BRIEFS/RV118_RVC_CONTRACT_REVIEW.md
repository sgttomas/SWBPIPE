# RV118 (RV-C): independent review of B2-C, the B2 contract (documents only)

TASK (Type 2), an independent reviewer dispatched by ROOT (HELP_HUMAN, Agent 0). ROOT is your return path, and you do not delegate. **You are a fresh instance and wrote none of this contract.** You hold RV-C for B2: this contract review, and later B2's statics at J1.

**Why it matters.** The contract drafts statics that land at J1's interim registration:
- a new formation definition (DEF-C);
- PTABLE's one revision;
- SCHEMA's B2 `$defs`, merged with B3b's change.

Every later B2 slice implements it: the kernel, the producer, admission, three readers and 07o. An error costs a re-registration, or a reader that accepts what it should refuse.

## The candidate

- **The contract:** `R/I97/b2_c_01/CONTRACT.md` (sha256 `165cd4b1c0b5ed3f83f9230435c0a3ab2284211c90908daa647610bba81d9d28`).
  - Its draft statics are in `statics/`: DEF-C, the revised PTABLE, `SCHEMA_B2.diff`, the merged J1 SCHEMA and `SCHEMA_J1.diff`.
  - Its generator, checks and collision scan are in `_run_records/`. ROOT rebuilt all five statics byte for byte.
- **The brief it answered:** `R/BRIEFS/B2C_CONTRACT.md` (`a604a2c3…`).
- **The plan and its rulings:**
  - I93's PLAN.md (§1.2, decisions 5–10 and 20–21) and REVISION_01 (§1.1, §1.3, §1.4, §2; decisions 28–31);
  - RV114's review;
  - RR "B2/B3 R1: …", "I93's REVISION_01 accepted; …", "RV115 (RV-K) accepts B2-KD with amendments; R-1 to R-11 ruled", "RV116 confirms B3-D's revision 01; B3-D is final for J1" and "RV109 passes SP in RV-P round 2; …".
- **The kernel design:** I94's `R/I94/b2_kd_01/DESIGN.md`, with RV115's review. **RV115 (RV-K) checks DEF-C's numerical content separately** (CONTRACT §3, and R-7, R-8 and R-10). Weigh it only as it affects the contract.

## Its basis

- **B0's contract:** DESIGN_v2 §4 (C3a's P1 text and reserved spellings) and decision 12. Also C1 §5–§6, C2, C3 and SC1 §2, by the plan's notation.
- **B1's transaction, T-1 to T-13,** as RV109's round 2 confirmed it, on `b1` at `603e238517` (read-only, `WT/b1`).
- **B3-D, final:** `R/I96/b3_d_01/` with REVISION_01 and `statics/r1/`. Its `receipt_bindings` and `SCHEMA_ENUM.diff` must compose with B2's.
- **The code** at NUM's maintained tree (main `54f1ba1f6d`): FK, PP, the three readers, PTABLE, DEF-O, SCHEMA and CORPUS.

## Review, in priority order

1. **The transaction (§2).**
   - **T-10a and T-10b sit correctly in B1's confirmed T-1 to T-13,** with every z = 0 byte unchanged.
   - Check the four operand-source kinds against C3a rule 1 and S-2 (rebuild-and-check for an unavailable operand; registration for a `not_required` one; the first selected operand's group).
   - Check the failure split (decision 7), including R-1's abandonment through `W1Fallback::CombinationCustody` and **N-5: a combination's origin refusal is never a `CaptureError::Origin`.**
   - §2.2's list of B1 code that B2 must extend for z ≥ 1 (T-6′, T-8′, T-9′, T-11′) must be complete. Look for any B1 path that would mis-handle a combination row, Call or headline.
2. **DEF-C and `CombinationAttempt` (§3, §4).**
   - Regenerate the statics with `b2c_statics.py`.
   - Check DEF-C's H under `retained_precision_formation_v1` (`9adf5178…`), with DEF-O's `a7ed7ca0…` as the control.
   - Check that DEF-O's bytes and pins are unchanged.
   - **C-3:** is `CombinationAttempt` as a separate closed `$def` in a union with `ProductAttempt` a faithful reading of REVISION §1.1 item 3, which says `ProductAttempt.definition_id` becomes an enum? Does it compose with B3b's `SCHEMA_ENUM.diff` unchanged? Recommend one form.
3. **R-COMB-1 (§5):** (R), reader-derived, against (M), a receipt member. Check that (R) needs no output-code change for D-U6-2 and T6S, and that every input to the classification is receipt-bound. Note the owner-facing DN §4.2 text.
4. **SCHEMA (§6).**
   - Check B2's diff and the merged J1 text against main's vocabulary.
   - Re-run `b2c_checks.py`: 0 verdict changes over the committed bases, must-pass entries, mutations and successor fixtures, under both jsonschema and PY's `_shape`.
   - Add instances of your own, positive and negative, for each new `$def`.
5. **PTABLE (§7):**
   - the scope extension;
   - `product_formation_definitions` = [DEF-O, DEF-C];
   - **C-8:** `formation_warrant` becomes a list. Is that the right shape, and how does G0 read it?
   - `receipt_bindings` must equal XTABLE's, canonically and as text.
6. **G0 (§8) and gate placement (§10):**
   - every new check's gate, code and first failure, in identical order across the three readers;
   - **07o's 14 bases and must-pass entries and 64 mutations,** each with its designed first failure;
   - today's readers stop every `CombinationAttempt` base at G0. Is that the right designed failure?
7. **D1.4 and the cap rows (§9):**
   - the limits, with **C-9: combination ids disjoint from case ids**;
   - CAP_ROWS 47 → 53;
   - G-C's two new bounds;
   - N-4's Li = 3·Lc.

   Check them against I82's study, PLAN §3.3 and RV114's pricing.
8. **J1's three extra edits (§11):** RS `g0`'s constant, TS `header`'s comparison, and PP `retained_wire_tests.rs`'s `u1_constants_bound_to_in_tree_fixtures`. Are they necessary and sufficient for J1's interim registration? Is anything else needed? REVIEWED_INPUTS goes 14 → 17.
9. **Decisions C-1 to C-16 (§12):** AGREE or DISAGREE, with a reason.
   - Weigh especially C-1 (repeated-case mechanics stay `ordinary`), C-4 (the new cause `operand_source_unavailable`), C-6 (`operand_preparations` absent when empty, against C3a's "empty if none") and C-12 (K4CMB recomputed; K4LED attestation only).
   - Check the collision log (§13).
   - **Public meaning:** say whether anything here changes public meaning before B8, or touches an owner-held choice.
10. **The estimates (§14).**

## Host and method

- **Documents and code reading only,** plus read-only Python with VENV against committed files, including I97's scripts and your own checks in scratch.
- **Not allowed:** cargo, native jobs, installs, and Git writes. Read Git with `GIT_OPTIONAL_LOCKS=0`. Use `PYTHONDONTWRITEBYTECODE=1`.
- **Writes:** absolute paths only.
  - Scratch, and TMPDIR, go in `WT/scratch/rv118_rvc/`.
  - Records hold placeholder paths only.
  - No record folder is named `build`, and no record contains a symlink.
  - Remove the `hostname` attribute from any pytest junit output before it goes into a record.
  - Before returning, screen your records with the strict pattern and the machine's host name, decompressing any `.gz` file.

## Output

- **The report:** `R/REVIEW_RV118/b2_c_01/REVIEW.md` with `evidence/` and SHA256SUMS. It contains:
  - a verdict: ACCEPT, ACCEPT WITH AMENDMENTS, or REVISE;
  - counts of BLOCKING, SHOULD-FIX and NOTE findings;
  - a findings table;
  - a section per item;
  - the decision table.
- **Budget:** 6–9 h.
- **End your turn with:**
  - the verdict;
  - the counts, with one line per finding;
  - the report's sha256;
  - what ROOT must rule on.
