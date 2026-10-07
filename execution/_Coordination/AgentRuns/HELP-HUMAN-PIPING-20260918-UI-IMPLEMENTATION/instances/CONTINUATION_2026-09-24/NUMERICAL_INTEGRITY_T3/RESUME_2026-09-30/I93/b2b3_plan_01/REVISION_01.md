# I93 B2/B3-P: REVISION_01, the amendments of B2/B3 R1

TASK (Type 2), I93, for ROOT (HELP_HUMAN, Agent 0), who is the return path. I made no delegation. 2026-10-07 UTC. Documents only: no cargo, no Git write, no new computation.

**What this amends.** `PLAN.md` in this folder (sha256 `e1147dbda247c470b2127dbf9413dd10213227216da95495814715e6f8a1238a`, unchanged beside this file). PLAN.md plus this revision is the plan. Where they differ, this revision governs.

**What it answers:**
- **The review:** `R/REVIEW_RV114/b2b3_plan_01/REVIEW.md`, sha256 `bc6918ce474a188d02f8bab018947b073d4da0ed2a7cdca6af6e535b0a2a9ee6` (verified). ACCEPT WITH AMENDMENTS, 0/4/12.
- **The ruling:** RR "B2/B3 R1: I93's plan accepted with RV114's amendments; I93 writes REVISION_01; the owner informed".

Placeholders and abbreviations are PLAN.md's. **DEF-O** = `P/fixtures/results/retained_precision_prepared_ordinary_v1.json`, the ordinary definition.

## 1. The four SHOULD-FIX amendments

### 1.1 S-1: the combination's own formation definition and product attempt (into B2-C; decisions 9 and 13)

**The gap.**
- DEF-O's `scope` has owners `load_case` and loads `individual_normalized_nodal_terms`, and its excludes list `prepared_combinations`.
- SCHEMA has `ProductAttempt.definition_id` `const` the ordinary id, and `owner_ref.kind` `const "case"`. `Body.sources` admits only `CaseSource`.
- PTABLE's `product_formation_definitions` and `formation_warrant` name DEF-O only.

So no published combination could carry a product attempt today.

**Added to B2-C's contract** (B2-KD supplies the numerical content; RV-C and RV-K review it RV69-style):
1. **A new definition** (decision 28): `RP-PREPARED-COMBINATION-DUAL-v1` is suggested; it has 0 hits outside `P/execution` at NUM. Its file is `P/fixtures/results/retained_precision_prepared_combination_v1.json`, and ROOT reserves the name and its H domain at B2-C. It binds:
   - **owners:** `mechanics_combination`;
   - **loads:** the combined exact ledger (each operand's terms times its factor, as exact products; K4LED);
   - **prescriptions:** exact positive zero (D1);
   - **section terms and materials:** those of the operands' prepared sources (equal across operands, by the kernel's `OperandsDiffer` check);
   - **the proof:** the dual-readout certificate through B2-KD's combination source view, with DEF-O's final predicates and classes unchanged;
   - **excludes:** subtraction, range, and exact-profile formation.

   DEF-O's bytes stay unchanged, so its hash pins (RV114's `checks.txt` §4) do not move.
2. **The combination's `ProductAttempt`.**
   - Its owner kind is `combination`.
   - It enters no preparation stage: preparation belongs to the operands' case attempts or `OperandPreparation`s.
   - Its stages are native, proof and projection, as the definition states.
   - `product_attempts[]` keeps actual start order, so case attempts come first.
   - Each retained combination entry carries `product_attempt_ref`.
3. **SCHEMA, in B2 (not B3b):**
   - `ProductAttempt.definition_id` and `OperandPreparation.definition_id` become an enum. It holds DEF-O's id and the combination id in B2, and the exact id in B3b.
   - `owner_ref.kind` admits `combination`.
   - `Body.sources` admits `CombinationSource`.
4. **PTABLE, at decision 9's one revision:** `product_formation_definitions` gains the combination definition `{id, sha256}`, and `formation_warrant` names it.
5. **`REVIEWED_INPUTS`:** the combination definition is a precommit static, so it is +1. With B3b's two statics the count goes **14 → 17** (PLAN §1.4 said 16).
6. **Readers' G0:** each `definition_id` is a member of PTABLE's `product_formation_definitions` and equal to the reader's constant set (§2).

### 1.2 S-2: three kernel facts (into B2-KD and B2-K; decisions 6 and 16)

These rows join PLAN §1.2.3's table. B2-KD must decide each, and RV-K's design round checks them.

| Fact (FK, at NUM) | Consequence | Recommendation for B2-KD |
|---|---|---|
| (a) `ExecutionOutcome::Unresolved` and `Refused` hold no `CasePrep` (only attempts and geometry; `adaptive.rs` `ExecutionOutcome`) | An `unavailable` operand has no kernel prep to borrow | **Rebuild:** `PreparedCaseSource::new` from PP's retained prepared `PrimitiveSource` for that case, identity-checked (K4SRC bytes) against its registered `SourceOrigin`. `ExecutionOutcome` and SC1 §4's consumers are untouched. The alternative, retention, changes `ExecutionOutcome` and brings SC1 §4's consumer list into lane K |
| (b) Sources are registered only by `solve_cases` and `solve_combination`, and `store.sources` is sized at `capacity.runs` (`origins.rs` `RecordedInvocation::new`, `OriginCapacity::for_calls`) | A `not_required` operand's source cannot be registered | **Add** `OriginCapacity` a prepared-source count (a `for_calls` argument) and one recording entry that registers a prepared source with owner `NativeOwner::Case(i)` and no Run. C2 §3's "registered at construction" then holds |
| (c) `RetainedCombination::solve` and `RecordedInvocation::solve_combination` run on `operands[0]`'s group (`combine.rs`; `origins.rs`) | Operand 0 may be prepared | **Use the first selected operand's group** in authored order (decision 5 guarantees one). The combined `CasePrep` still uses operand 0 as representative, as today |

**Also for RV-K (N-1):**
- An independent oracle: `P/core/solver/frame_kernel/tests/retained_k4/product_certificate_vectors.py` gains combination vectors, including SC1's A + B − A2 cancellation.
- RV-K reads I42 (`source_bridge_01`, `source_bridge_rv56_repair_02`), I43, I44 and `REVIEW_RV56` (`source_bridge_01`, `source_bridge_repair_02`).

**B2-K's estimate becomes 14–24 h.**

### 1.3 S-3: one owner per file (decision 29)

| File | Owner (lane) |
|---|---|
| SCHEMA (every `$defs` change, B2's and B3b's; the readers implement validation only) | I-A (A) |
| PTABLE; DEF-O (unchanged); the combination definition; the exact definition; `semantic_contract_v0_3_physics_retained_1.json` | I-A (A) |
| `PP/build_identity.rs`, `P/core/product_physics/build.rs` | I-A (A) |
| `PP/retained_memory.rs` (admission; `REGISTERED_PROFILES`; later the generated block) and `PP/retained_memory_law_tests.rs` | I-A (A), then SQ2 (also I-A) |
| `P/schemas/analysis_run.v0.3.schema.json`, `stress_neutral_export.v0.3.schema.json`, `results.v0.3.schema.yaml` (B3b's branches and their hash constants) | I-TS (T) |
| RS and RE `semantic_contract.rs`; PY and `compatibility.py` and the two PY schema tests; TS and `numericalResultQuality.ts` | their reader lane |
| CORPUS | SC2's writer (I-PY) |

**The one exception is J1's package (§1.4).** It is ROOT's, prepared by I-A, and it changes hash constants in files that reader lanes and lane T own. Those lanes fork only after J1, so no file has two concurrent editors.

### 1.4 S-4: the interim registration at J1 (decision 30, as ruled)

**Why.**
- `build.rs` digests every `REVIEWED_INPUTS` file.
- `build_status()` (`PP/retained_memory.rs`) is `Stale` unless the compiled text equals `REGISTERED_PROFILES[0].reviewed_inputs`.
- The facade tests' `registered()` reads only the compiler identity.

So B2's first static edit would make every `b2` build Stale: Direct tests would fail, and the c = 1 and B1 pins could not run.

**Where J1 sits.**
- J1 is the first integration point after J0.
- J0 is unchanged: after PR-B1 merges and NUM absorbs main, ROOT cuts `b2` from main.
- J1 needs B2-C and B3-D selected (§4). **Lanes A, P, RS, PY and TS fork from J1**; lane K forks from J0, since FK is not a reviewed input.
- If B3-D's statics are not ready, B2's statics land at J1 and B3b's at **J1′**, by the same procedure with a fresh re-pin. Any later statics correction also goes through lane A as a J1′.

**The J1 package** (I-A prepares it as `statics_j1.diff` with its sums; RV-Q2 round 1 checks that it is complete and mechanical; RV-C and RV-D check fidelity to their designs; ROOT applies it on `b2` as one commit and records it in RR as an interim registration, `b2` only):
1. **The statics:**
   - SCHEMA (B2's `$defs` per §1.1 and PLAN §1.2.2, plus B3b's enum value);
   - PTABLE revised (§2);
   - the combination definition;
   - the physics-retained-1 table;
   - the exact definition.
2. **`REVIEWED_INPUTS`:** 14 → 17 in `build_identity.rs`. Also `encode_reviewed_inputs`' array length and `build.rs`'s digest array.
3. **The re-pin:** `REGISTERED_PROFILES[0].reviewed_inputs` in `PP/retained_memory.rs`, re-derived from the new files' sha256 in `REVIEWED_INPUTS` order.
4. **The law tests:**
   - `reviewed_inputs_bind_the_lock_and_the_reader_statics`: its `REVIEWED` hashes and `files` list, at 17;
   - `an_unreadable_reviewed_input_never_binds`: its literal 14s become 17.
5. **PTABLE's hash cascade, constants only, in the 12 files** (PLAN §1.2.7). RS's `TABLE_HASH` is on the precommit path: without it every successor would fall back at G0.

**What J1 does not re-pin:**
- the identity text;
- `reader_layouts`;
- `threshold_bytes` (B1's M);
- the generated profile;
- the cap constants;
- any successor or corpus byte (no fixture carries PTABLE's hash; RV114 §6).

**J1's acceptance, in a Registered build:**
- **byte-identical:** every c = 1 pin and B1's multi-case pins (W-C2, coexistence);
- **passing:**
  - `the_registered_profile_is_the_only_permit_source` (one entry; B1's threshold);
  - the Direct-entry facade tests;
  - the two law tests above;
  - the three readers' suites;
  - the carrier schema tests.
- **Stale:** every output is still the plain bytes.

A failure means the package is not applied (stop).

**The interim's honesty.** From J2 (admission) until SQ2, the registered dev/test build admits combinations and the legacy contract priced on B1's S3 profile. That is the I2 precedent (RR "I89's SA verified and ruled; …", ruling 3). HIGH prices C_eq ≤ 3 within about 6 MB of S3 (N-3). The exact route's admission (B3b-A) lands only after B3-S shows it fits B1's M (decision 26); otherwise it waits for SQ2. There is no product caller, and nothing reaches NUM before SQ2.

**How SQ2 supersedes it.**
- SQ2's `registration.diff` re-derives `reviewed_inputs` from the frozen statics (J7), together with the profile, `threshold_bytes`, the cap re-pins and `PINNED_RECORD`.
- ROOT replaces the interim entry, so `REGISTERED_PROFILES` stays one entry.
- Pass B checks the entry against SQ2's applied registration.
- The interim never exists in NUM or main.

## 2. Decision 9, re-ruled, and N-12

**PTABLE is revised once, at B2, in J1's package.** In one revision:
- `accuracy_classification.scope` gains the prepared-combination extension;
- `product_formation_definitions` and `formation_warrant` gain the combination definition (S-1);
- **RV78-N1's policies are bound:**
  - the projection policy `RP-LOGICAL-ATTEMPTS-v1`;
  - the work policy `W1-LME-20B-60B-v1`, with `case_limit` 20,000,000,000 and `invocation_limit` 60,000,000,000;
  - the method token `contribution_preserving_multiprecision_v1`;
  - the canonicalization `openpipestress_jcs_ijson_v1`.

The contract id, profile, rows, inherited hash and every other member are unchanged. B3b's `physics-retained-1` table binds the same policies from its first version.

**N-12: G0 reads the bound values from the table and keeps the constants as a cross-check,** in all three languages. Concretely:
- **The table read.** G0 compares the receipt's `projection_policy`, `work_policy`, `work.case_limit`, `work.invocation_limit` and `canonicalization`, and every attempt's `definition_id`, with the values in PTABLE (or `physics-retained-1`'s table on that route). The tables are read through each reader's packaged copy, as `TABLE_HASH` is today.
- **The constants kept as a cross-check:**
  - RS's G0 literal list (`policy`, `projection_policy`, `work_policy`, `facade_policy`, `canonicalization`, `case_limit`, `invocation_limit`), `METHOD` and `DEFINITION_ID`, which becomes the set of definition ids;
  - PY's and TS's equivalents.
- **G0 fails with the existing unsupported-contract code** if a table value differs from its constant. So neither can drift alone.
- **The tests:**
  - 07o gains one mutation per bound receipt member (each refused at G0 in all three readers);
  - each reader gets a reader-local unit test of the table/constant cross-check, using a test-only table. A corpus entry cannot change a table.

## 3. RV114's notes N-1 to N-11

| Note | Change to the plan |
|---|---|
| N-1 | RV-K's oracle and reading list: §1.2 above |
| N-2 | The owner-facing figure for three cases plus a combination at D1's caps is **"about 13.25 GiB (13.0–14.0)"**, not "under all three estimates". LOW misses the 5 % line by only 29,917,134 B; any repeated W1 TEXT share fails it |
| N-3 | PLAN §3.3's method: HIGH's ledger surcharge assumed h = c. With h = 3 at c < 3, C_eq ≤ 3 is at most 5,981,880 B over S3 (c = 1, z = 2), with a 9.38–9.46 % budget at 10.5 GiB. The conclusion is unchanged |
| N-4 | **A second reason for C_eq ≤ 3:** C1 §2's Li = 60B per invocation and Lc = 20B per run. Three runs cannot make Li bind; a fourth could be invocation-exhausted, or would need a new work policy, which PTABLE now binds. This joins §3.3's tier-2 costs and decision 22's package |
| N-5 | **The corrected sums of PLAN §2.5's rows: 198–314 h agent and 59–92 h review, about 12–17 working sessions** (B1's conversion). With this revision's additions the totals are in §5 |
| N-6 | **R-COMB-1's representation** (reader-derived from disposition and operand statuses, or a new receipt member) is B2-C's design question, selected by ROOT at B2-C's review. My lean is reader-derived: both inputs are already in the receipt, and D-U6-2 and T6S's withheld-witness path take classes from the reader. **The owner note names DN §4.2:** it listed subtraction and range under W1a "at the facade", and in F2a their rows over a Sensitive case are withheld from binding |
| N-7 | **W-CB1's one-case proxy carries 2l = 256 loads,** outside D1.9's l ≤ 128. B2-W runs it unnetted, with probe-only code in its disposable archive that bypasses G-A and G-B (I81's method; never maintained code), and records the over-cap fact. A netted proxy is rejected, because it changes the cancellation that the combined ledger preserves. The committed W-CB1 is in domain (two 128-load cases plus the combination; L = 256 ≤ 384). **Step labels:** PLAN's "T-9b" and "T-9c" become **T-10a** (dispositions, after T-10's check) and **T-10b** (operand preparations, combination Calls and freezes) |
| N-8 | **T6S needs tests only for B2.** `basisReference` (`StressNeutralExportPanel.tsx`) and `sourceBasisReference` (`P/apps/desktop/src/services/analysisRunCompatibility.ts`) already map `combination`. B2's TS lane adds tests on a combination successor in `StressNeutralExportPanel.test.tsx` and `analysisRunCompatibility.test.ts`; the conditional source edit is dropped |
| N-9 | Phase 0 counts against the three-implementer cap, and B2-C waits for B1's transaction to settle (§4) |
| N-10 | **PLAN §7 gains item 5:** if RV-K finds the combination certificate unsound and ROOT takes no second design round, B2 ships coverage-only, with every mechanics combination `ordinary`. That is an owner-facing narrowing of F2a's promised combination route (RR:7051). It also joins the owner note |
| N-11 | **B3-D records the reading of DN §4.3's "zero pressure":** zero-magnitude legacy pressure loads stay outside D1.7 and fall back to ordinary bytes, as 0.1/0.2 models do today |

## 4. Phases and integration points, restated

**Phase 0** (documents, Python and probes; **every phase-0 TASK counts against the three-implementer cap**; B1's work and its repairs go first):
1. **First, the work independent of B1's moving transaction:** B2-KD; B3-S; then B3-D, which uses B2-KD's finding on whether an exact annulus version is needed.
2. **B2-C after SP's RV-P round 2 settles T-1 to T-13.**
3. **B2-W and B3-W under `WT/guard/cargo_job.lock`,** when the host is free and a slot is open.
4. **Reviews:** RV-K (design) after B2-KD; RV-D after B3-D; RV-C after B2-C (with RV-K's numerical read of the combination definition).
5. ROOT selects each design. **J1 needs B2-C and B3-D selected.**

**After J0:**

| Point | What lands | Then |
|---|---|---|
| **J0** | `b2` cut from main after PR-B1 | Lane K (B2-K) forks |
| **J1** (J1′ if split) | The statics package and the interim registration (§1.4) | Lanes A, P, RS, PY and TS fork from J1 |
| **J2** | Admission (B3a-A, B2-A, and B3b-A if B3-S allows), after RV-Q2's round on the expressions | — |
| **J3** | B2-K, after RV-K round 2 and its repairs | — |
| **J4** | B3b-P, after RV-P2 round 1 | B2-P starts on J3 and J4 |
| **J5** | B3a's and B3b's readers, after RV-R2 | — |
| **J6** | B2-P, after RV-P2 round 2; B2's readers, after RV-R2 | SC2 writes 07o; I-RS and I-TS update their pins |
| **J7** | **Code freeze** | SQ2 (its registration supersedes J1's), then PLAN §2.3 phases 5–6 unchanged |

**The critical path** gains J1's package: about 2–4 h of I-A's time plus RV-Q2's check.

## 5. Phase-0 TASK briefs (outlines for ROOT)

Common to all six:
- a fresh TASK under ROOT, no delegation;
- records in `R/<id>/<unit>_01/` with SHA256SUMS and placeholder paths only;
- scratch in `WT/scratch/<id>_<unit>/`;
- no record folder named `build`;
- read Git with `GIT_OPTIONAL_LOCKS=0`; no Git writes; no installs; nothing in the system temp directory;
- basis: PLAN.md and this revision, RV114's review, RR "B2/B3 R1…", and NUM's maintained tree at the dispatch.

| TASK | Purpose | Basis (beyond the common) | Write set | Output | Budget |
|---|---|---|---|---|---|
| **B2-KD,** the kernel design | Design B2-K: the prepared-operand API (SC1 §1) and its recorded variant; S-2's (a)–(c); `NativeOwner::Combination` as a product owner; the combination source view in bridge, residual and tightening; the combination definition's numerical content (§1.1 item 1); the oracle vectors' specification (N-1); the test list (SC1 §5 C01–C06, W05–W06). It also says whether B3b needs an exact annulus version | SC1, C1 §1–§2, C2 §3–§4, C3 §1–§3; FK `combine.rs`, `origins.rs`, `adaptive.rs` (`ExecutionOutcome`, `SourceBridgeView`), `product_certificate/*`; I42–I44 with RV56; RV114 §1 | Records only | `DESIGN.md`: signatures, invariants, the stop list, each S-2 decision with alternatives, the test and oracle list, and a refined estimate. No code | 7–11 h |
| **B3-S,** the exact-route pricing | Price the exact route at C = 3 (and C_eq = 3) with I82's chain, rebinding the D1.3 zero rules (PLAN §0 item 9: `append_exact_pressure_results`, `composite.rs`, `pressure_material.rs`, the `build_pressure_case_with_members` edges, and the exact route's precommit validator), on a `git archive` snapshot of main. Also state the tree's change at c = 1 against I82's | STUDY (method), ADD; I82's `_run_records/` chain tools; I65's u4_g7_06 chain; an exact-route request fixture for the census | Records only; Python (VENV); no lock needed | `STUDY.md`: E+R per mode, the 5 % M, binding terms, the rules rebound with their multiplicities, and whether route caps are needed (decision 26) | 3–5 h |
| **B3-D,** the B3 design | B3b's contract (PLAN §1.4 items 1–5): `RP-PREPARED-EXACT-DUAL-v1` (draft definition JSON, H domain, collision check), the physics-retained-1 table text (RV78-N1's policies from version 1; the inherited hash), the exact route's row families and their recipes or classes, D1.3 and D1.5 texts, the readers' `<physics-retained>` branch, the carrier branches and the output-policy entry. **B3a's D1.3 text and N-11's reading** | DESIGN §5, decisions 13–15; DN §4.3–§4.4; D2 §4.9; C2 §3 (`SOURCE_ODWALL_EXPECTATIONS`); C3 and DEF-O; PP `pressure_runtime.rs`, `source_recovery.rs`, the exact producer path; physics-1's table; the physics-source fixtures; B2-KD's annulus finding; I74 §4.3 | Records only | `DESIGN.md` plus the draft statics as records (the definition, the table and SCHEMA's enum diff), ready for J1's package; the decisions and a collision log | 10–16 h |
| **B2-C,** the B2 contract | B2's contract (PLAN §1.2.2 with the T-10a and T-10b labels): C3a's completions; the combination formation definition and `ProductAttempt` (§1.1); decisions 5–8 as texts; **R-COMB-1's representation**; SCHEMA's `$defs`; PTABLE's revised text (§2); the D1.4 text and cap rows; gate and code placement; G0 per N-12 | DESIGN §4 and decision 12; C1 §5–§6, C2, C3, SC1 §2; B2-KD's selected design; **B1's T-1 to T-13 as RV-P round 2 confirmed them**; PTABLE; DEF-O; SCHEMA | Records only | `CONTRACT.md` plus the draft statics as records (SCHEMA diff, PTABLE revision, the combination definition JSON), ready for J1; names for ROOT to reserve | 11–18 h |
| **B2-W,** the combination probe | One-case proxies in both modes on main's registered build: W-CB1's `A + B` (256 loads, unnetted, probe-only bypass in the archive; N-7); W-CB3's candidate `not_required` operands (for example a force on the L = 0 base's restrained isolated node), up to 6 variants; W-CB2's prediction checked against case C. It records verdicts, seeds, the W1 stage reached and native terminals | I81's and I86's PROBE method and inputs (U8's two-body model, SW's cap-maximal A, B, C); PLAN §1.2.6 | A disposable `git archive` in scratch; target `WT/targets/<id>-b2-w/`; records. **Every cargo run through `WT/tools/t3_cargo.sh` under the lock;** one wait per job | `PROBE.md` with the controls (two runs identical; reproduction of I81's lines) and the stop rule's result | 3–5 h |
| **B3-W,** the exact-route probe | The milestone authored as 0.3.0 exact (E and ν; explicitly empty regions), with n05 and n06, in both modes through the ordinary route on main: verdicts, physics-source-1's selection (`source_block_recovery`), and the rows the exact route publishes with empty regions. Also the milestone as 0.3.0 `legacy_pressure_v1` (B3a), checking that the ordinary bytes match 0.1.0's except the model echo | DN §4.3; physics-source fixtures; PP `pressure_runtime.rs`, `source_recovery.rs` | As B2-W | `PROBE.md`: the inputs (sha256), the outcomes, and a witness recommendation for B3b and B3a | 3–5 h |

**Phase 0 in total:** 37–60 h agent, in up to three concurrent TASKs when B1 frees the slots.

## 6. Decisions whose recommendation changes

| # | Was (PLAN §5) | Now |
|---|---|---|
| 3 | Phase 0 now; J0 then lanes | Phase 0's order as §4. J1 is the statics landing with the interim registration; lanes other than K fork from J1; the J points are renumbered (§4) |
| 6 | Operand sources as C3a rule 1 | Plus S-2: an unavailable operand's prep is **rebuilt and identity-checked**, not retained; a `not_required` operand's source is **registered** (`OriginCapacity` prepared-source count); the group is the **first selected operand's** |
| 8 | R-COMB-1 row-level `not_covered` | **Unchanged in substance.** Its representation is B2-C's question (lean: reader-derived), and the owner note names DN §4.2 |
| 9 | Revise PTABLE in place with RV78-N1 | **As re-ruled:** one revision at B2, in J1's package, adding S-1's formation entries; G0 reads the table with the constants as a cross-check (§2) |
| 10 | C_eq ≤ 3 | **Unchanged.** Reason added: Li stays non-binding (N-4) |
| 13 | B3b's exact definition at B3-D | Plus B2's combination definition at B2-C (S-1). `REVIEWED_INPUTS` 14 → 17 |
| 16 | B2-K as planned FK scope | Plus S-2's three items and N-1's oracle; **14–24 h** |
| 17 | Phase 0 now, documents and Python | **Counts against the cap;** B2-KD, B3-S, B3-D first; B2-C after RV-P round 2; probes under the lock |
| 21 | B2: combination basis references in T6S | **Tests only** (N-8) |
| 22 | Decision 17's package | "About 13.25 GiB (13.0–14.0)"; Li binds at C_eq = 4 |
| 25 | Witnesses | W-CB1's proxy runs unnetted with a probe-only bypass (N-7) |
| **28 (new)** | — | **The combination formation is a new definition** (`RP-PREPARED-COMBINATION-DUAL-v1`, suggested), not a revision of DEF-O, whose bytes and hash pins stay. ROOT reserves it at B2-C. **Decider: ROOT** |
| **29 (new)** | — | **One owner per file,** as §1.3. **Decider: ROOT** |
| **30 (new)** | — | **The interim registration at J1** (and J1′), as §1.4, superseded by SQ2's. **Decider: ROOT (ruled)** |
| **31 (new)** | — | **G0 reads the table's bound policies, with the constants as a cross-check** (§2). **Decider: ROOT** |

Every other decision (1, 2, 4, 5, 7, 11, 12, 14, 15, 18–20, 23, 24, 26, 27) is unchanged. None is owner-held except 22–24, which stay prepared and undecided.

## 7. Estimates, restated

**The plan's rows, corrected (N-5):** 198–314 h agent and 59–92 h review.

**This revision adds:**
- B2-C: +3–6 h (S-1);
- B2-KD: +2–3 h;
- B2-K: +2–4 h (S-2);
- J1's package: +1–2 h;
- review: RV-C +1–2 h, RV-Q2 +1 h.

**Totals: about 206–329 h agent and 61–95 h review, about 12–17 working sessions after PR-B1** (the additions sit mostly off the post-J0 critical path). This is within the owner's "about 200–315 h and 60–90 h" to rounding at the low end, and up to 14 h agent and 5 h review above it at the top.

## 8. What I read, and limits

**Read:**
- RV114's REVIEW (whole);
- RR "I93's B2/B3 plan returned…" and "B2/B3 R1…";
- this folder's PLAN.md;
- at NUM:
  - FK `adaptive.rs` (`ExecutionOutcome`), `origins.rs` (`OriginCapacity`, `RecordedInvocation::new`);
  - PP `retained_memory.rs` (`bindings_hold`, `build_status`, `REGISTERED_PROFILES`), `retained_memory_law_tests.rs` (the two reviewed-input tests and `the_registered_profile_is_the_only_permit_source`), `build.rs`, `build_identity.rs`, `retained_facade_tests.rs` (`registered()`);
  - RS's G0 (`TABLE_HASH`, the policy literals, `DEFINITION_ID`) and PY's and TS's G0 constants;
  - DEF-O's `scope`;
  - TS `analysisRunCompatibility.ts` (`sourceBasisReference`);
  - the FK `retained_k4/` test folder listing.

**Collision checks** (`git grep -F`, outside `P/execution`, at NUM): `RP-PREPARED-COMBINATION-DUAL-v1`, `retained_precision_prepared_combination_v1`, `RP-PREPARED-EXACT-DUAL-v1` and `retained_precision_prepared_exact_v1` have 0 files each.

**Limits:**
- No new computation; the figures are RV114's and PLAN.md's.
- §1.2's recommendations are for B2-KD to confirm or replace.
- §1.4's acceptance lists the tests that bind `reviewed_inputs` today. A test added by B1 after NUM's tree that pins it would join the list at J1.
