# RV116 (RV-D): independent review of B3-D, the B3 design (documents only)

TASK (Type 2), an independent reviewer dispatched by ROOT (HELP_HUMAN, Agent 0). ROOT is your return path, and you do not delegate. **You are a fresh instance and wrote none of this design.** You hold RV-D for B3: this design review, and later B3b's statics at J1.
- The design drafts statics that land at J1's interim registration: a new formation definition, the `physics-retained-1` table, and SCHEMA's enum.
- It changes three readers.

So an error costs a re-registration.

## The candidate

- **The design:** `R/I96/b3_d_01/DESIGN.md` (sha256 `ad7942f66c906270003c7c7fd98bb7c64900183dc47b31796844fb051e37b69b`).
  - The draft statics are in `statics/`: the definition JSON, the table JSON and `SCHEMA_ENUM.diff`.
  - Its generator `b3d_statics.py` is in `_run_records/`.
- **The brief it answered:** `R/BRIEFS/B3D_DESIGN.md` (`08b34b12…`).
- **The plan and its rulings:**
  - I93's PLAN.md and REVISION_01;
  - RV114's review;
  - RR "B2/B3 R1: …", "I93's REVISION_01 accepted; …", "I95's B3-S: …" (rulings 1–5) and "RV115 (RV-K) accepts B2-KD …".
- **The kernel design:** I94's B2-KD and RV115's review. A kernel item, **K3-2** (`build_member`'s represented-Z enclosure for `ExactENu`), is being checked separately by RV115 (RV-K). Weigh it only as it affects B3's design.

## Its basis

- **B0's contract:** DESIGN_v2 §5, decisions 12–15. DN §4.3–§4.4; D2 §4.9; C2 §3; C3; DEF-O.
- **RV78-N1's policies,** bound from version 1.
- **The code** at main `2007709549`:
  - PP's `pressure_runtime.rs`, `source_recovery.rs` and the exact producer path;
  - physics-1's table and validator;
  - physics-source-1;
  - the physics-source fixtures;
  - the three readers.
- **I95's study:** `R/I95/b3_s_01/STUDY.md`.

## Review, in priority order

1. **The definition** `RP-PREPARED-EXACT-DUAL-v1`.
   - **Is preparing on the exact route necessary** (B3D-3, B3D-4)?
     - Reproduce I96's finding that SourceAnnulus's published sections are not correctly rounded: one ulp on n05/n06's I, J and Z, and 75 % of random sections differing.
     - Reproduce that the derived G equals binary64 `e/(2*(1+nu))` in its sampled cases.
   - **Is the new `evidence` member** (the regenerated `pipe_sections` and stress extrema for a selected case) sound against D2 G5b?
   - **Regenerate the definition with `b3d_statics.py`,** and check that its H(`retained_precision_formation_v1`) is `9b66492e…`.
2. **The table** (`5bf0d0dc…`).
   - Re-run the construction, and check that it reproduces the preview table's `c74742ce…` from preview-physics-1's.
   - Check the three changed and six appended members, the inherited hash `9a2cf626…`, and `receipt_bindings` (RV78-N1's policies).
   - Check that G0 reads the table, with the constants as a cross-check (decision 31).
3. **The D1 texts.**
   - **B3a's D1.3,** with N-11's reading.
   - **B3b's D1.3, D1.4 and D1.5:** no combinations on the exact route (I95 ruling 4); c ≤ 3; every region explicitly `[]`.
   - Every refusal uses an existing family fact.
4. **The readers' `<physics-retained>` branch** in RS, PY and TS: one dispatch, G0, G5b's section bit check, G7's projection to physics-1, and G8's predicates.
   - **No new failure code.** Check first failures against I96's 07o list (§6.4).
   - B3a's G8 change closes PY's `pressure_contract: {}` difference.
   - **S-C's minimal exposure** of physics-source-1's `actual_materials` touches the base readers (B3D-12). Is it the minimal change, and does it alter any base reader's outcome?
5. **The producer requirements:**
   - P-2's budget parity: `permitted_run`'s default 4M against the ordinary route's 8M for the exact route. Can physics-source-1 select differently on the Direct entry?
   - P-4: W1 never calls the pressure-runtime builders, so I95's two zero rules hold. Check both rules by code reading, as RV83 and RV84 checked G4's.
6. **B3-K** (K3-1, K3-2 and K3-3) as B3 needs it, and J2k (B3D-16). RV115 rules on K3-2's kernel content.
7. **The decisions B3D-1 to B3D-18:** AGREE or DISAGREE, with a reason. Check the name reservations (§10) for collisions, and the single spelling of `receipt_bindings` shared with B2-C.
8. **B3b's admission timing** (J2 under the interim, about 152 MB under-priced but within 0.9 M, or after SQ2). Recommend one.

## Host and method

- **Documents and code reading only,** plus read-only Python with VENV against committed files, including `b3d_statics.py` and your own exact-arithmetic checks in scratch.
- **Not allowed:** cargo, native jobs, installs, and Git writes. Read Git with `GIT_OPTIONAL_LOCKS=0`.
- **Writes:** absolute paths only. Scratch goes in `WT/scratch/rv116_rvd/`. Records hold placeholder paths only. No record folder is named `build`.

## Output

- **The report:** `R/REVIEW_RV116/b3_d_01/REVIEW.md` with `evidence/` and SHA256SUMS. It contains:
  - a verdict: ACCEPT, ACCEPT WITH AMENDMENTS, or REVISE;
  - counts of BLOCKING, SHOULD-FIX and NOTE findings;
  - a findings table;
  - a section per item;
  - the decision table.
- **Budget:** 5–8 h.
- **End your turn with:**
  - the verdict;
  - the counts, with one line per finding;
  - the report's sha256;
  - what ROOT must rule on.
