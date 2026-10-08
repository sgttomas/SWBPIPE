# RV115 (RV-K): independent numerical review of B2-KD, the kernel design for B2 (documents only)

TASK (Type 2), an independent reviewer dispatched by ROOT (HELP_HUMAN, Agent 0). ROOT is your return path, and you do not delegate. **You are a fresh instance and wrote none of this design.** You hold RV-K for B2's kernel work: this design review now, and B2-K's code review later.
- B2-K changes FK (the solver kernel) and adds a numerical certificate for combinations, so an error here reaches published numbers.
- ROOT rules on I94's eleven points (R-1 to R-11) only after your review.

## The candidate

- **The design:** `R/I94/b2_kd_01/DESIGN.md` (sha256 `4e8c33a45616d4f0975aa74f159157ed9c5e0da7f3d306a002646bc704aabed3`), with `_run_records/`.
- **The brief it answered:** `R/BRIEFS/B2KD_KERNEL_DESIGN.md` (`c7b84097…`).
- **The plan:** I93's PLAN.md and REVISION_01.md (`R/I93/b2b3_plan_01/`), RV114's review, and RR "B2/B3 R1: …" and "I93's REVISION_01 accepted; phase 0 opens with B2-KD (I94)".

## Its basis

- **The contract records:** SC1, C1 §1–§2, C2 §3–§4 and C3 §1–§3 (paths in the plan); DEF-O, the ordinary formation definition.
- **The earlier kernel work:**
  - I42 (the bridge), I43 (tightening, and the certificate theorem) and I44 (the residual), under `R/I42`, `R/I43` and `R/I44`;
  - RV56's reviews (`R/REVIEW_RV56/`), including its uniqueness proof;
  - the R7 transfer theorem, as cited.
- **The code** at NUM's maintained tree (main `2007709549`):
  - FK's `combine.rs`, `origins.rs`, `adaptive.rs` and `product_certificate/*`;
  - `FK/src/structural/retained_resource.rs`;
  - PP's exhaustive matches on FK's enums;
  - `product_certificate_vectors.py`.

## Review, in priority order

1. **The certificate is sound.** I94 claims DEF-O's dual readout applies to the combination's own exact problem (`K u = N`, `G u = N`, `u_C = 0`, N the combined exact ledger), with all of DEF-O's terms unchanged plus two new ones: E5, the outward width of each 1024-bit net, and E6, the prescription term, zero under P2.
   - **Re-derive it yourself.** Check that I43's theorem's premises hold for a combination owner, where I94 re-checked them but did not re-derive R7's transfer or RV56's uniqueness.
   - Check E5's bound, δ_g ≤ 2^(⌊log2|N_g|⌋ − 1023).
   - Check that no operand quantity enters the proof.
   - Check the row set (7n + 50m + 8g) and the coverage rule's derivative slots.
   - Say whether anything published could be outside its enclosure.
2. **The kernel facts and S-2's decisions hold in the code:**
   - (a) rebuild, not retain, with a full K4SRC identity check;
   - (b) registration through the existing `cases` count, with no struct growth, and `for_calls(a, b) == for_invocation(a, b, 0)`;
   - (c) the first selected operand's group, with operand 0's loads and constraint values never used.
3. **The two code constraints are true and complete:**
   - the pinned sizes in `retained_resource.rs` and their use in PP's registered profile;
   - PP's exhaustive matches on FK's enums.

   Check that the API is purely additive under both, with `solve` and `solve_combination` byte-identical wrappers and only `CombinationReason::NoSelectedOperand` as a new variant. Does PP match `CombinationReason` exhaustively?
4. **The source view:** the bridge (I42) and tightening (I43) are unchanged. The residual (I44) reads the combined exact ledger at both load-read sites. P2 keeps `UnsupportedCombination` for any nonzero prescription.
5. **The oracle (N-1).** Is the extension of `product_certificate_vectors.py` independent of the kernel, and do its six cases (C1–C6) cover the certificate's new terms (E5 with a 1201-bit net; 2^±700 factors; a prepared first operand)?
6. **The stop list (S-1 to S-14) and the estimate (18–28 h).** Is anything missing?
7. **I94's eleven points (R-1 to R-11).** AGREE or DISAGREE with each, with a reason.
8. **B3b's answer** (no exact annulus version needed) and **the E/ν gap** for B3-D and B3-K.

## Host and method

- **Documents and code reading only,** plus read-only Python with VENV against committed files. You may run `product_certificate_vectors.py`'s existing parts read-only, or write your own exact-arithmetic checks in scratch.
- **Not allowed:** cargo, native jobs, installs and Git writes. Read Git with `GIT_OPTIONAL_LOCKS=0`.
- **Writes:** absolute paths only. Scratch goes in `WT/scratch/rv115_rvk/`. Records hold placeholder paths only. No record folder is named `build`.

## Output

- **The report:** `R/REVIEW_RV115/b2_kd_01/REVIEW.md` with `evidence/` and SHA256SUMS. It contains:
  - a verdict: ACCEPT, ACCEPT WITH AMENDMENTS, or REVISE;
  - counts of BLOCKING, SHOULD-FIX and NOTE findings;
  - a findings table;
  - a section per item;
  - the R-1 to R-11 table.
- **Budget:** 5–8 h.
- **End your turn with:**
  - the verdict;
  - the counts, with one line per finding;
  - the report's sha256;
  - what ROOT must rule on.
