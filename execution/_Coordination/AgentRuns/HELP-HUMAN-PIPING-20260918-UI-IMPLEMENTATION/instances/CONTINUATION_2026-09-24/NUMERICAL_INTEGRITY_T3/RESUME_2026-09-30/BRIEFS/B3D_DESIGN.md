# B3-D: the B3 design (B3a and B3b; documents and code reading only)

TASK (Type 2), a designer dispatched by ROOT (HELP_HUMAN, Agent 0). ROOT is your return path, and you do not delegate. **You are a fresh instance.** Cite the records, and assume nothing beyond them.

## Why

B3 brings F2a's promised exact routes into the retained transaction (I93's plan, B3 split by REVISION_01):
- **B3a:** 0.3.0 `legacy_pressure_v1` with zero pressure, on the preview successor. Small.
- **B3b:** 0.3.0 exact with explicitly empty pressure regions, under the reserved `openpipestress.result_semantics/0.3.0/physics-retained-1`. Large: a new formation definition, the new table, and a reader branch.

Your design is reviewed (RV-D) before J1, whose interim registration lands the statics you draft.

## The basis

- **The plan:**
  - `R/I93/b2b3_plan_01/PLAN.md` (`e1147dbd…`) and `REVISION_01.md` (`63abb73f…`), especially PLAN §1.4 items 1–5, decisions 12–15 and 26, and REVISION §5's B3-D outline and N-11;
  - RV114's review;
  - RR "B2/B3 R1: …" and "I93's REVISION_01 accepted; …".
- **B0's contract:** `R/I78/b0_contract_01/DESIGN_v2.md` §5. DN §4.3–§4.4; D2 §4.9; C2 §3 (`SOURCE_ODWALL_EXPECTATIONS`); C3; and DEF-O, the ordinary formation definition. The plan cites their paths.
- **RV78-N1:** the new table binds the projection and work policies, the 20B/60B limits, the method token and the canonicalization profile **from its first version** (RR "I86's SW probe accepted; I83's B6 return verified and ruled; …", ruling 1).
- **The code** at NUM's maintained tree (main `2007709549`):
  - PP's `pressure_runtime.rs`, `source_recovery.rs` and the exact producer path;
  - physics-1's table;
  - the physics-source fixtures.
- **I94's kernel design** (`R/I94/b2_kd_01/DESIGN.md`): no exact annulus version is needed. **The E/ν gap:** FK's public `ProductMaterial` has no E/ν route, although `MaterialOperands::ExactENu` exists internally. Say whether B3b needs it, and what B3-K would change.
- **I95's exact-route pricing,** `R/I95/b3_s_01/`, if it has returned when you need it. Otherwise state your assumptions.
- **The T6S consistency notes** (I74 PLAN §4.3).

## What the design must give (`DESIGN.md`, plus draft statics as records)

1. **B3b's formation definition,** `RP-PREPARED-EXACT-DUAL-v1`:
   - the draft definition JSON;
   - its H domain;
   - a collision check against existing names and hashes.
2. **The `physics-retained-1` table text:** RV78-N1's policies from version 1, the inherited hash rules, and how G0 reads them, as for the preview table under decision 31.
3. **The exact route's row families,** with their recipes or classes.
4. **D1.3 and D1.5 texts** for B3b, and **B3a's D1.3 text**, with **N-11's reading of DN §4.3**: zero-magnitude legacy pressure loads fall back to the ordinary route.
5. **The readers' `<physics-retained>` branch** in RS, PY and TS: gates, codes and first failures.
6. **The carrier branches and the output-policy entry,** with a golden (decision 21).
7. **Whether B3-K is needed** (the E/ν gap), with its scope and estimate.
8. **The decisions,** each with alternatives and a recommendation.
9. **A collision log** of every new name, for ROOT to reserve.
10. **Estimates** for B3a's and B3b's slices, refining the plan's.

**The draft statics** (records only, ready for J1's package):
- the definition JSON;
- the table;
- SCHEMA's enum diff.

## Rules

- **Documents and code reading only.** You may run read-only Python with VENV against committed files.
- **Not allowed:** cargo, native jobs, installs, and Git writes. Read Git with `GIT_OPTIONAL_LOCKS=0`.
- **Writes:** absolute paths only. Scratch goes in `WT/scratch/<id>_b3_d/`. Records hold placeholder paths only. No record folder is named `build`.

## Output

- **The record:** `R/<id>/b3_d_01/DESIGN.md`, with the draft statics, `_run_records/` and SHA256SUMS.
- **Budget:** 10–16 h.
- **End your turn with:**
  - DESIGN.md's sha256;
  - the definition and table in brief;
  - B3a's and B3b's D1 texts in brief;
  - the readers' branch in brief;
  - the B3-K answer;
  - the names to reserve;
  - the estimates;
  - anything ROOT must rule on.
