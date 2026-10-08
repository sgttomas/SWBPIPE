# B2-KD: the kernel design for B2 (documents and code reading only)

TASK (Type 2), a designer dispatched by ROOT (HELP_HUMAN, Agent 0). ROOT is your return path, and you do not delegate. **You are a fresh instance.** Cite the records, and assume nothing beyond them.

## Why

B2 (combinations, and preparation-only and mixed invocations) needs kernel (FK) work that does not exist (RV114 confirmed it in FK's code):
- `RecordedInvocation::solve_combination` takes only selected operands;
- `product_case_owner` requires `NativeOwner::Case`;
- the source bridge view refuses combinations.

B2-K implements the kernel change. **You design it first,** so that a fresh numerical reviewer (RV-K) can check the design before any code.

## The basis

- **The plan:**
  - `R/I93/b2b3_plan_01/PLAN.md` (`e1147dbd…`) and `REVISION_01.md` (`63abb73f…`), especially REVISION §1.2 (S-2), §1.1 item 1, §5 (your outline) and decisions 6, 16 and 28;
  - RV114's review (`R/REVIEW_RV114/b2b3_plan_01/REVIEW.md`, `bc6918ce…`) §1;
  - RR "B2/B3 R1: I93's plan accepted with RV114's amendments; …".
- **The contract records:** SC1, C1 §1–§2, C2 §3–§4 and C3 §1–§3. The plan cites their paths; find them under `R/` and `T/DESIGN_NUMERICS/`.
- **The code** at NUM's maintained tree (main `2007709549`): FK's `combine.rs`, `origins.rs`, `adaptive.rs` (`ExecutionOutcome`, `SourceBridgeView`) and `product_certificate/*`.
- **The earlier kernel records:** I42–I44, with RV56's reviews.

## What the design must give (`DESIGN.md`)

1. **The prepared-operand API** (SC1 §1) and its recorded variant:
   - signatures and invariants;
   - how a combination's operands are named and checked;
   - `NativeOwner::Combination` as a product owner.
2. **S-2's three kernel facts, each with alternatives and a recommendation:**
   - **(a)** an unavailable operand's prep is rebuilt as a `PreparedCaseSource` and identity-checked, not retained (`ExecutionOutcome` unchanged);
   - **(b)** a `not_required` operand's source is registered through a new `OriginCapacity` prepared-source count;
   - **(c)** the combination runs on the first selected operand's group.
3. **The combination source view** in bridge, residual and tightening, and how `UnsupportedCombination` is lifted, or kept for what B2 does not cover.
4. **The combination definition's numerical content** (REVISION §1.1 item 1, the suggested `RP-PREPARED-COMBINATION-DUAL-v1`): owners `mechanics_combination`, combined exact-ledger loads, zero prescriptions, and the operands' section terms. Leave the reviewed-input JSON to B2-C, but state what it must bind.
5. **The final-row certificate for combinations.** This is the numerical heart. State:
   - what is proved;
   - over which rows;
   - its error terms, with their derivation;
   - how it composes with the operands' certificates.
6. **The oracle (N-1):** the specification for extending `product_certificate_vectors.py` with combination vectors, so RV-K has an independent oracle.
7. **The test list:** SC1 §5's C01–C06 and W05–W06, plus whatever the design adds.
8. **Whether B3b needs an exact annulus version** of any of this (B3-D uses your answer).
9. **A stop list** for B2-K (changes that would need ROOT), and **a refined estimate** for B2-K (the plan says 14–24 h).

**No code.** Sketches in the records are fine.

## Rules

- **Documents and code reading only.** You may run read-only Python with VENV against committed files.
- **Not allowed:** cargo, native jobs, installs, and Git writes. Read Git with `GIT_OPTIONAL_LOCKS=0`.
- **Writes:** absolute paths only. Scratch goes in `WT/scratch/<id>_b2_kd/`. Records hold placeholder paths only, never `~/`, `/Users/`, `/private/` or the worktree's name. No record folder is named `build`.

## Output

- **The record:** `R/<id>/b2_kd_01/DESIGN.md` plus SHA256SUMS.
- **Budget:** 7–11 h.
- **End your turn with:**
  - DESIGN.md's sha256;
  - the API in brief;
  - the S-2 decisions;
  - the certificate's claim and error terms in brief;
  - the B3b answer;
  - the stop list;
  - B2-K's refined estimate;
  - anything ROOT must rule on.

RV-K (a fresh numerical reviewer) reviews the design.
