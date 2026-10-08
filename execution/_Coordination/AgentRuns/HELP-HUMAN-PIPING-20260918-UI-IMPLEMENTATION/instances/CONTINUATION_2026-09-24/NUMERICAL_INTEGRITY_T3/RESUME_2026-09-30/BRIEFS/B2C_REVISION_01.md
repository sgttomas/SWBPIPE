# B2-C revision 01 (I97): RV118's amendments, RV115's notes and I98's witnesses

Read your own `R/I97/b2_c_01/CONTRACT.md` (`165cd4b1…`), then:
- RV118's review, `R/REVIEW_RV118/b2_c_01/REVIEW.md` (`fe640ca5…`);
- RV115's addendum on DEF-C, `R/REVIEW_RV115/b2_kd_01/ADDENDUM_02.md` (`7cd7a1b4…`);
- I98's B2-W probe, `R/I98/b2_w_probe_01/PROBE.md` (`e224899a…`);
- RR "RV118 (RV-C) accepts B2-C with amendments; B2-C ruled; C-1 to C-16 selected; I97 revises", "RV115 (RV-K) accepts DEF-C's numerical content" and "I98's B2-W verified; …". Their rulings are your specification.

## What to change

1. **S-4 (a): DEF-C's combination displacement magnitude takes DEF-O's support-magnitude pattern:** binary64 hypot of the published components, certified.
   - State the recipe, why G7's base guard (64ε) then holds by construction, and the certificate's coverage of it.
   - Regenerate DEF-C, its H, and PTABLE with `b2c_statics.py`, run twice byte-identical.
2. **S-4 (b):** define the combination's observables stage. It runs G7's `combination_magnitudes` guard, so any failure is a per-combination `facade_certificate`.
3. **N-7:** PTABLE's `accuracy_classification.scope` states R-COMB-1. **S-3:** `combination_modulus_basis_record` rows stay `non_quantity`, and R-COMB-1 applies to quantity rows only.
4. **S-1:** §2.2 gains the four B1 sites that refuse any model combination: the three T-2 capture hooks (`normalized`, `prepared_case_seen`, `prepared_case_source`), and `observables_view` at T-9.
5. **S-2:** T-6′ follows the producer's actual row layout, with the modulus-basis records after every combination's rows. Show that no in-domain authored order falls back for layout. Include c = 1 `[range(A), 2·A]`.
6. **S-5:** split W-CB4 into W-CB4a (A − B) and W-CB4b (range(A, B)), each with C_eq = 3. Fix m28's base.
7. **S-6:** define 07o's "rehash all" for B2's derived hashes, in dependency order, so that every designed first failure is reachable. State what SC2's three harnesses must add, including PY's operand-prepared source.
8. **N-12:** SCHEMA refuses an OperandPreparation carrying DEF-C's id. State the schema gate's first failure in all three readers, and align C3a-7's G0 row and §8 row 9.
9. **C-4** is recorded as an amendment of C3a rule 4, superseding KD §2.1's mapping (N-2).
10. **The other NOTEs taken:**
    - N-1: any rebuild refusal leads to `CombinationCustody`.
    - N-3: list every RS branch point, including G3's attempt bijection, Run/`execution_order` equality and owner lookup, and `g5_native`.
    - N-4: correct m50, m36 and `b2_base_withheld`'s designed failures.
    - N-5: G5 refuses a `CaptureError {kind: origin}` in a combination attempt or operand preparation.
    - N-6: state that no code reads `formation_warrant`, and that its list shape differs from XTABLE's object.
    - N-8: J1's RS edit is the constant list only.
    - N-9: name the freeze's maps and member-fact source.
    - N-10: B2-A censuses combination strings and places the cap rows before `ControlBytes`.
    - N-11: G3 checks the order of `operand_preparations[]` and of the operand-prepared CaseSources.
    - N-14: the collision table adds the `count_range` tag in space `combination` and its member `name`.
    - N-15: restate the estimates.
11. **The witnesses (I98):**
    - W-CB1 is 1·A + 0.5·B (base `r7_cb1_halfb.json`), with A + B as a labelled cancellation pin;
    - W-CB2 is `r7_cb2.json`; W-CB3 is `r7_cb3_v1.json`.
    - **D6b is case-only:** state that no combination analogue refuses a `retained_selected` combination for being ordinarily `checks_passed`.
    - Update §10's bases and must-pass entries.
12. **RV115's NB-1 and NB-3 wording, in DEF-C:**
    - NB-1: "the selected material operands of every member, bit for bit";
    - NB-3: data flags stay per individual product, never per net.

**Sealed files are never replaced in place.** Write the revised statics to `statics/r1/`, beside the unchanged v0 `statics/`. Write `REVISION_01.md` with `SHA256SUMS.revision_01`, and leave CONTRACT.md and SHA256SUMS untouched.

## Rules

As your first brief (`R/BRIEFS/B2C_CONTRACT.md`):
- documents and code reading only, plus read-only Python with VENV (`PYTHONDONTWRITEBYTECODE=1`);
- no cargo, no installs and no Git writes;
- absolute paths only;
- scratch in `WT/scratch/i97_b2_c/`;
- placeholder paths only, no symlink and no `build` folder;
- screen with the strict pattern, the host name and `.local` (judged by what it names);
- run `git status --ignored`.

## Output

- `R/I97/b2_c_01/REVISION_01.md`, `statics/r1/` and `SHA256SUMS.revision_01`.
- **Budget:** 4–6 h.
- **End your turn with:**
  - REVISION_01.md's sha256;
  - the regenerated hashes (DEF-C raw and H; PTABLE; SCHEMA_B2; the J1 SCHEMA);
  - one line per amendment;
  - anything for ROOT.
