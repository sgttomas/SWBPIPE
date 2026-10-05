# I72: U8-4, the fail-closed Pass B on the U8 candidate

TASK (Type 2). Read `BRIEFS/U8_COMMON.md` first; it binds you. You hold the Pass B role that I65 held. Read I65's records first:
- `R/I65/u4_g7_03/` to `u4_g7_06/` (the tool and the reviewed table);
- `R/I65/u9_refreeze_01/` (the no-build run);
- RV89's addenda in `R/REVIEW_RV89/u4_g7_03/`.

## The task

Run Pass B on the U8 candidate head that ROOT gives you, against the same bases as `u4_g7_06`: Pass A `ba1faa1c…` and the registered `0c7827b6ad`.

**Which tool:**
- **Preferred:** the fail-closed `g7_pass.sh` from `R/I65/u4_g7_06/_run_records/`. It builds, so run it only when ROOT confirms the host is free.
- **If ROOT asks for a no-build run:** first restore RV89 N-1's items in a copy of `g7_pass_nobuild.sh`:
  - the single verdict and its exit code;
  - the early stops;
  - the `text_summary` gate;
  - the delta-tool failure handling;
  - a corrected header.
  
  Record the diff from I65's version.

**Expected:**
- **The delta** is test-class rows only: `retained_facade_tests.rs`, the reader tests, the fixtures and the corpus.
- **Every other gate is 0:**
  - the tree;
  - the entry, byte-identical to `0c7827b6ad`'s, M included;
  - the law tests (build run);
  - statics;
  - the line map and premise pins;
  - TEXT (D 14,734);
  - FORMS and §11;
  - the controls;
  - the witnesses and the challenge (build run).
- **All 11 reviewed entries** still match.
- **The maxima are unchanged** (0.8881 / 0.8929 M).

**Any production-class row is a stop.**

## Records

`NUM/R/I72/u8_passb_01/`: RETURN.md (the verdict line, gate codes, the delta rows, and a comparison with `u4_g7_06` and `u9_refreeze_01`), `_run_records/` and SHA256SUMS.

## Budget and return

1–1.5 h, plus build time if you run the full tool. Return once, with the verdict, the gate codes and the RETURN sha256.
