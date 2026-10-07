# RV114: independent review of I93's B2/B3 plan (documents only)

TASK (Type 2), RV114, for ROOT (HELP_HUMAN, Agent 0), who is the return path. I made no delegation. I am a fresh instance and wrote none of the plan. 2026-10-07 UTC.

**Brief (verified before work):** `R/BRIEFS/RV114_B2B3_PLAN_REVIEW.md`, sha256 `b7555eeec443d361dba135404e6557e17ce1385d1cb6a31945a4bbcde7485690`. I read NUM's `AGENTS.md` and `agents/AGENT_TASK.md` first.

**The candidate:** `R/I93/b2b3_plan_01/PLAN.md`, sha256 `e1147dbda247c470b2127dbf9413dd10213227216da95495814715e6f8a1238a`; its SHA256SUMS check 9/9 OK. The brief it answered, `R/BRIEFS/B2B3_PLAN.md`, is `174550b0…0c8a` as cited.

**Placeholders:** WT, NUM, P, PP (= `P/core/product_physics/src`), FK (= `P/core/solver/frame_kernel/src/structural/retained`), RE, T, R, RR and VENV as in the dispatch. PLAN (I61's roadmap), DESIGN (I78's DESIGN_v2), PLAN_v2, STUDY, ADD, C1, C2, C3, SC1, DN, D2, PTABLE, SCHEMA and CORPUS as the plan defines them. "The plan" is I93's PLAN.md; its sections are cited as §n.

**Basis.** NUM `b0b691f17e`; outside `P/execution` its tree equals main `2007709549` (`git diff --quiet`). B1's branches read at `b1` `603e238517`, `b1-r` `b5cb7faaeb`, `b1-p` `11cc14e3e6`, `b1-t` `7e47e51b5d`, `b1-a` `9812c83ded` (the plan's heads); none touches PTABLE, the ordinary definition or SCHEMA. Inputs and their hashes are in `evidence/checks.txt` §1–§2.

**Limits kept.** Documents and code reading, plus read-only Python (VENV) on committed bytes, including I82's evaluator. No cargo, vitest, native or solver job; no install; no Git write. Scratch only in `WT/scratch/rv114_b2b3_plan/`; nothing in the system temp directory. `evidence/RUN.md` has the commands.

## Verdict

**ACCEPT WITH AMENDMENTS: 0 BLOCKING, 4 SHOULD-FIX, 12 NOTE.**

The plan's two load-bearing findings hold. **The kernel finding is true in code**: FK's `solve_combination` takes only recorded selected solves, and the product certificate refuses combination owners and combination sources. So B2 needs SC1's prepared-operand API and a combination certificate, and putting FK in B2's planned scope, with a fresh numerical reviewer, is right. **The memory answer is right**: I reproduced I93's pricing byte for byte and re-priced it independently. Three D1-cap cases plus a D1-cap combination need 12.25 / 13.25 / 14.0 GiB (LOW / MID / HIGH, 5 % text budget). The proposed shape C_eq = c + z ≤ 3 prices within 6 MB of S3, at about a 9.4 % budget at 10.5 GiB.

Four gaps should be fixed before phase 0's briefs are written:
- **S-1:** the combination's own formation definition and product attempt are missing from B2's contract, although the reviewed ordinary definition explicitly excludes `prepared_combinations`.
- **S-2:** three kernel facts that decide the prepared-operand path are missing from B2-K's list.
- **S-3:** the statics' lane owner overlaps lane T and is stated two ways.
- **S-4:** there is no plan for the interim in which every PP build on `b2` is Stale. Changing any reviewed input (SCHEMA, PTABLE, a definition) makes the registered build Stale until SQ2's registration.

All 27 decisions are AGREE, several subject to a finding. None decides an owner-held item.

## Findings

| ID | Sev. | Section | Finding and evidence | Remedy |
|---|---|---|---|---|
| **S-1** | SHOULD-FIX | §1.2.2, §1.2.7, §3.2; decisions 9, 13 | **B2's contract omits the combination's own formation definition and product attempt.**<br>• The reviewed ordinary definition (`P/fixtures/results/retained_precision_prepared_ordinary_v1.json`, one of the 14 `REVIEWED_INPUTS`) has `scope.owners: load_case`, `scope.loads: individual_normalized_nodal_terms`, and `scope.excludes` including `prepared_combinations` (`checks.txt` §4).<br>• C3 itself: "C3 grants no prepared-combination proof route" (C3_DELTA:67); "Exact-profile/combination formation completion … remain" (C3_DELTA:416).<br>• SCHEMA: `ProductAttempt.definition_id` is `const` the ordinary id and `ProductAttempt.owner_ref.kind` is `const "case"`; `Body.sources` admits only `CaseSource`.<br>• PTABLE's `product_formation_definitions` and `formation_warrant` name only the ordinary definition; C3a's `OperandPreparation` carries the same id with purpose `combination_operand` (DESIGN §4.2).<br>• The plan revises only PTABLE's scope clause (decision 9), schedules the `definition_id` enum for B3b only (§1.4 item 5), and §1.2.4 speaks of "combination attempts" without a contract for them. | B2-C, with B2-KD, designs the combination formation: either a new definition (collision-checked name and H domain, reserved by ROOT) or a versioned revision of the ordinary one.<br>The same work specifies:<br>• the combination's ProductAttempt: owner kind `combination`, which stages it enters (no preparation), and `product_attempt_ref` on combination entries;<br>• SCHEMA's `owner_ref`, `definition_id` and `CombinationSource` home, in B2;<br>• PTABLE's `product_formation_definitions` and `formation_warrant`, in decision 9's same revision;<br>• the definition-hash cascade (6 maintained files, `checks.txt` §4) if revised, or `REVIEWED_INPUTS` + 1 if added;<br>• G0 in §1.2.5's reader table.<br>RV-C and RV-K review it, RV69-style. Consider designing it with B3b's definition (one review). About +4–8 h agent, +1–2 h review. |
| **S-2** | SHOULD-FIX | §1.2.3; decisions 6, 16 | **B2-K's change list omits three kernel facts the prepared-operand path depends on.**<br>(a) **An `unavailable` operand's kernel CasePrep.** `ExecutionOutcome::Unresolved` and `Refused` hold no `Arc<CasePrep>` (FK `adaptive.rs:4404–4416`). Decision 6's "that prepared source" therefore needs either retention, which changes `ExecutionOutcome` and brings in SC1 §4's consumer list, or a `PreparedCaseSource` rebuilt and identity-checked against the registered source.<br>(b) **Registering a `not_required` operand's source** in `RecordedInvocation`. Sources are pushed only by `solve_cases` and `solve_combination` (`origins.rs:367, :488`), and `store.sources` is sized at `capacity.runs` = cases + combinations (`:319`, `:56–58`). So `OriginCapacity` and a registration entry change.<br>(c) **The GroupPrep.** `solve_combination` and `RetainedCombination::solve` run with `operands[0]`'s group (`origins.rs:533`; `combine.rs:140–142`). SC1 §1 says "the selected operand need not be first" and requires a selected operand's GroupPrep. | Make (a)–(c) required content of B2-KD and rows of §1.2.3's table, including `OriginCapacity::for_calls`'s new count. If (a) retains the prep, add SC1 §4's consumers to lane K's conditional set. RV-K's design round checks all three. B2-K's 12–20 h looks low with these: consider 14–24 h. |
| **S-3** | SHOULD-FIX | §1.2.2, §1.2.7, §1.4 write sets, §2.1 | **The statics' owner overlaps lane T and is stated two ways.**<br>• "Statics (one owner, lane A or T)" writes `analysis_run.v0.3.schema.json` (4 PTABLE-hash constants) and `stress_neutral_export.v0.3.schema.json` (1). Lane T edits the same two files for B3b's successor branches.<br>• SCHEMA's installer is "the readers' lanes" in §1.2.2 and the statics owner in §1.2.7.<br>• `PP/build_identity.rs` and `build.rs` (PP code) are also in the statics row.<br>PLAN_v2's SF-1 standard is lanes split by file. | Name one owner per file. For example: T owns the two carrier schemas, including their hash constants; A owns SCHEMA, PTABLE, the definitions and `build_identity.rs`/`build.rs`.<br>State the J point at which SCHEMA and PTABLE land relative to the reader lanes' forks from J0. |
| **S-4** | SHOULD-FIX | §2.3 phases 1–5, §3.2, §1.2.7 ("never touched without a stop") | **No interim for the registered build between the statics landing (phase 1) and SQ2's registration (phase 5).**<br>• `build.rs` digests the 14 `REVIEWED_INPUTS` at build time. `build_status()` returns `Stale` unless `compiled_inputs == Some(registered_inputs)` (`retained_memory.rs:983–1003`).<br>• So the first edit to SCHEMA, PTABLE or a definition makes every PP build on `b2` Stale, and W1 takes the ordinary route.<br>• The tests' `registered()` reads only the compiler identity (`retained_facade_tests.rs:534–542`). So Direct-entry tests that expect `Registered` fail, and the c = 1 and B1 successor pins cannot run: the plan's own stop.<br>• B1 avoided this by making any reviewed-input change an R8 stop (PLAN_v2 §4). B2 is the first unit to change reviewed inputs after a registration. | Plan the interim explicitly. Either:<br>(i) ROOT applies an interim `reviewed_inputs` re-pin, with the law tests' pinned text, together with each statics change (at J1), reviewed by RV-Q2 round 1, and SQ2's registration re-derives it; or<br>(ii) the statics land late, before G5. Producers and readers then work on the private driver and synthetic receipts meanwhile, and the plan says which tests run Stale.<br>Either way, the c = 1 byte-identity control stays runnable in a Registered build throughout. |
| N-1 | NOTE | §0 item 2, §1.2.3 | **The kernel finding is confirmed, with precise wording** (§1 below).<br>• The `MissingSelectedOrigin` path is a recorded `OriginRefusal` (an `Ok`), reached after `validate_operands`. The `&RetainedSolve` operand type already excludes an unsolved operand.<br>• A selected combination run is recorded in `store.selected`, but `product_case_owner` requires `NativeOwner::Case`.<br>• The residual reads the representative source's `loads()` and `constraint()` (`source_residual.rs:623, :1161`; `bridge.rs:162`), which is why the combination view is numerical design. | Keep RV-K fresh and numerical. Require an independent oracle for the combination certificate: extend `retained_k4/product_certificate_vectors.py` with combination vectors, including SC1's A + B − A2 cancellation. Give RV-K I42–I44's and RV56's records. |
| N-2 | NOTE | §0 item 6, §3.3 | **Memory reproduced and re-priced independently; one margin is thin.** LOW for 3 + 1 (dense) misses the 12 GiB 5 % line by only 29,917,134 B, and sparse LOW fits at 12.0 GiB. LOW assumes a retained combination repeats none of a case's W1 TEXT. Any repeated share ≥ 0 fails: 13.0 GiB at my W1-file share of 0.498, and 13.25 GiB at 0.586 and at I93's 0.651 (`checks.txt` §10). | State the owner-facing answer as MID's "about 13.25 GiB (13.0–14.0)", not "under all three estimates". |
| N-3 | NOTE | §3.3 method | **HIGH's ledger surcharge uses h = c,** but D1.4 as proposed admits h = 3 terms (repeats counted) at c < 3. With h = 3, C_eq ≤ 3 rises to at most 5,981,880 B over S3 (c = 1, z = 2), with a 9.38–9.46 % budget at 10.5 GiB. The l-dependence measured from I82's trees, about 3.5 kB (T12) and 0.7 kB (T13) per load per case, is within the surcharge. | Correct the method text; the conclusion is unchanged. |
| N-4 | NOTE | §3.3 tier 2, decisions 9, 10, 22 | **The work meter is a second reason for C_eq ≤ 3.** C1 §2 pins Lc = 20B per case and per retained combination, and Li = 60B per invocation (WIRE_CONTRACT:58). C_eq ≤ 3 keeps Li from binding. Every C_eq = 4 alternative (tier 2, decision 22) would make Li bind, so a fourth run could be invocation-exhausted, or would need a new work policy, which decision 9 would by then have bound into PTABLE. | Add this to §3.3's tier-2 costs and to decision 22's package. |
| N-5 | NOTE | §2.5, §0 item 12 | **The estimates are honest as reading estimates, but understated at the top.**<br>• The rows sum to 198–314 h agent (stated "about 195–300") and 59–92 h review.<br>• At B1's own conversion (PLAN_v2 §4: 69–107 serial hours ≈ 8–12 sessions), 105–155 serial hours is about 12–17 sessions, not 10–15.<br>• The total is ×3.9 I61's figure; B1's plan was ×3.<br>• B1's actuals so far point the same way: SP re-estimated by about +20 % (RR R3′), and repair rounds in ST, SR-RS and SR-PY. U8 and T6S finished faster. | Restate the totals: about 200–315 h agent, 60–90 h review, about 12–17 sessions. |
| N-6 | NOTE | §1.2.2 R-COMB-1; decision 8 | **R-COMB-1 needs a stated wire representation, and its owner note should name DN.**<br>• C1 §5's `retained_unavailable` and `ordinary` entries carry no class members; `not_covered` lives in `selection`. So R-COMB-1 is either reader-derived (from disposition and operand statuses) or a new receipt member. C1 §6 sends exactly this "new standing semantics" question to ROOT, and decision 8 is that return.<br>• DN §4.2 lists subtraction and range under W1a "at the facade"; C1 §5 and SC1 §2 keep them on their base contract. | B2-C states the representation and how D-U6-2 and T6S's withheld-witness path see it. The owner note says that subtraction and range over a Sensitive case will be withheld from binding in F2a. |
| N-7 | NOTE | §1.2.6, §1.2.2 | **W-CB1's one-case proxy carries 2l = 256 nodal loads,** outside D1.9's l ≤ 128, so Direct refuses it on main. A netted proxy changes the cancellation that the combined ledger preserves.<br>"T-9b, after T-9" actually runs after T-10's check (§1.2.4 has it right). | B2-W states whether it uses the private driver or a netted proxy. Rename T-9b and T-9c to follow T-10. |
| N-8 | NOTE | §1.2.7 TS row; decision 21 | **T6S already maps `combination`.** `basisReference` in `StressNeutralExportPanel.tsx:1055–1060` maps it, and `sourceBasisReference`, which is in `services/analysisRunCompatibility.ts:49–53`, not in the panel, also maps it. B2's T6S work is tests on a combination successor. | Name `analysisRunCompatibility.ts`'s tests in the TS write set if a pin is added; drop the conditional. |
| N-9 | NOTE | §2.3 phase 0; decision 17 | **Phase 0 TASKs count against the three-implementer cap.** RR counted SW and I93 as implementers, and B1's phases 4–5 use three.<br>B2-C also builds on B1's T-1 to T-13, which are still moving (SP). | State that phase 0 yields to B1. Write B2-C after RV-P round 2 confirms SP, or accept rework under risk 12. |
| N-10 | NOTE | §7 | **§7 omits risk 1's coverage-only outcome** (every mechanics combination `ordinary`). The plan itself calls that an owner-facing scope change (RR:7051). | Add it to §7. |
| N-11 | NOTE | §1.3; decision 12 | **Zero-magnitude legacy pressure loads fall back to the ordinary route.** The ordinary route admits them (`pressure_runtime.rs` refuses only non-zero), and D1.7 keeps them outside W1, so they fall back with ordinary bytes, as 0.1/0.2 models do today. This is a reading of DN §4.3's "zero pressure", not a defect. | Record the reading in B3-D. |
| N-12 | NOTE | §1.2.7; decision 9 | **The PTABLE cascade is verified** (§6 below). The plan does not say whether the readers' G0 then reads the newly bound policies or keeps its constants. | Specify G0's source and add the matching 07o mutations. |

## 1. The B2 kernel finding (item 1): TRUE

**Checked against FK's code** at NUM (`evidence/checks.txt` §3):
- **`RecordedInvocation::solve_combination(operands: &[(f64, &RetainedSolve)], …)`** (`origins.rs:413–415`) accepts only retained solves.
  - After `combine::validate_operands` (`:466`), each operand must match a recorded `SelectedOrigin` with an equal source identity and cache inventory. Otherwise it records `OriginError::MissingSelectedOrigin { operand }` and returns `RecordedKernelCombination::OriginRefusal` (`:476–478`).
  - A prepared or unavailable operand cannot be expressed, let alone accepted.
- **`product_case_owner`** (`origins.rs:795–800`) requires `matches!(self.store.runs[run].owner, NativeOwner::Case(_))`.
  - `certify` and `begin_prepared_product` both go through it (`final_case.rs:1047`, `:1630`).
  - A selected combination run is pushed into `store.selected` (`:700`) with owner `Combination`, and is then refused as a product owner.
- **`build_source_bridge_view`** returns `SourceBridgeViewIssue::UnsupportedCombination` whenever `self.prep.factors` is non-empty (`adaptive.rs:5359–5360`).
  - The residual and recovery read the representative source's individual loads and constraints (`source_residual.rs:623`, `:1161`; `bridge.rs:162`).
  - For a combination these are operand 0's, not the combined ledger's.
- **I42, I43 and I44's returns** each record combinations as unsupported (I42 RETURN:39–40, I43 RETURN:161, I44 RETURN:88).

**Therefore B2 needs both pieces:**
- **SC1 §1's prepared-operand API** (for B2b), because a `not_required` operand cannot become a `RetainedSolve` without a hidden solve (RR:7051);
- **a combination certificate** (for B2a and B2b alike), because even an all-selected combination cannot be certified today.

PP's `source_eligible` (`lib.rs:1169–1171`) and `pressure_runtime`'s blocking `EXACT_PRESSURE_COMBINATION_UNSUPPORTED` (`:202`) confirm that B2 and B3b never meet (§0 item 4).

**B2-K's scope.** Not too large. The minimal change is forced by two hard refusals, and the plan keeps the stop rule, schedule, prices, ceiling and every existing byte as stops. It is slightly too small as listed (S-2). The combination view itself is moderate: in D1's domain every prescription is exactly zero (the definition's `prescriptions: exact_positive_zero`), so the new work is chiefly the load term loops over operands' terms × factor. That is still numerical design, and I42 needed a repair round after RV56.

**The control.** A fresh numerical reviewer (RV-K) for design and code is the right control, as for I42–I44. I add an independent oracle (N-1). The FK tests already have a Python vector generator for the product certificate, which can carry combination vectors.

## 2. The memory answer (item 2): RIGHT

**Reproduction.** I93's three scripts, rerun unchanged from scratch copies, give byte-identical `b2_bracket.out.json`, `b2_mid.out.json` and `b2_table.out.txt` (`checks.txt` §9).

**Independent re-pricing** (`evidence/rv114_price.py`, written fresh; it imports only I82's `b1_eval.py`, checked at `c404e8db…`):
- **My per-form rule** is derived from which forms actually differ between I82's c and c + 1 trees: 36 forms from c = 1 to 2 and from 3 to 4, and 35 from 2 to 3, where `T17_output` is equal; the invocation forms are equal.
- **Case-like forms** come from the c + z tree: T13–T15, STAGED, T16_*, T17_* (including `T17_output`), SUCC and BODY.
- **Ordinary-side forms** come from the c tree: T11*, the X branch and TAV_X.
- **Partial forms:** O_base, TAV_W, NOTICE, NOTICE_moving and T12. LOW takes c and HIGH takes c + z.
- **The result:** LOW and HIGH (h = c) equal I93's to the byte at every point.
- **Text atoms:** swapping them to the c tree changes nothing, because the evaluator's forms carry the D-dependent text in their coefficients.

**3 cases + 1 combination at D1's caps** (dense; the smallest 256 MiB step with E+R + 5 %·TAV_W ≤ 0.9 M; both modes must fit, and dense binds):

| Estimate | E+R (B) | 5 % M | Under 0.9 × 12 GiB (11,596,411,699 B) |
|---|---|---|---|
| LOW | 11,416,844,017 | 12.25 GiB | yes, by 179,567,682 B (4.29 %); short of the 5 % line by 29,917,134 B |
| MID (I93, share 0.651) | 12,427,112,010 | 13.25 GiB | no |
| HIGH | 13,063,805,472 | 14.0 GiB | no |

**The claim that it does not fit within 12 GiB with the 5 % budget is TRUE** under each estimate (W3 binds everywhere).
- It is robust because LOW, the only estimate near the line, prices a retained combination as repeating none of a case's W1 TEXT. A combination has its own freeze, serializer entries and reader validation.
- I classified I65's c = 1 TEXT rows by file myself: 0.498 of TAV_W is in W1-side files, and 0.586 with the reader's row-side files.
- MID at those shares needs 13.0 and 13.25 GiB (`checks.txt` §10).
- A combination costs 0.50 (LOW), 0.81 (MID) and 1.00 (HIGH) of a fourth case. The plan's "about 0.8" is MID's.

**The cap shape c + z ≤ 3, against S3** (dense; S3 = 9,747,725,678 B):

| Shape | HIGH, h = c (I93) | HIGH, h = 3 (RV114) | 5 % M | Budget at 10.5 GiB |
|---|---|---|---|---|
| c = 2, z = 1 | 9,749,149,984 | 9,750,716,618 (+2,990,940 over S3) | 10.5 GiB | 9.46 % |
| c = 1, z = 2 | 9,747,441,022 | 9,753,707,558 (+5,981,880) | 10.5 GiB | 9.38 % |

**So C_eq ≤ 3 keeps B1's expected M** (10.5 GiB), with about 9.4 % against ROOT's 5 % rule and 1.5 GiB of ROOT's headroom below 12 GiB for G5's real combination and exact-route TEXT. HIGH is conservative apart from the combination-specific TEXT sites that today's rules zero, as the plan says, and I93's h = c surcharge is too small for h = 3 (N-3), by a few MB.

**Also checked:**
- the owner's-view heap figures (9.02, 11.51 and 12.10 GiB);
- the tier-2 table (each row reproduces from I82's points with the 5 % rule; for example, `t_c4_k20_l128`: 9.75 GiB).

C_eq ≤ 3 also keeps C1's 60B invocation meter from binding (N-4).

## 3. Coverage (item 3)

| Obligation | Placed in | Acceptance check | Verdict |
|---|---|---|---|
| **C3a** (DESIGN §4.2, rules 1–6; ROOT's reserved spellings) | B2-C §1.2.2 (placement, record point, gate codes); B2-P (`operand_preparations[]`); readers G3/G5/G8; 07o C3a mutations; hooks | Readers' first failures in 07o; W-CB2/W-CB3; SQ2 | Covered. Its definition id is S-1's subject |
| **The explicit-row rule** (PLAN_v2 §3.7; QUAL §11) | SQ2, decision 11 | Pass B fails closed on an unrowed site; RV-Q2 by type | Covered once |
| **RV78-N1's policies** (RR "I86's SW probe accepted…", ruling 1) | B3b's table from version 1; also PTABLE's revision (decision 9) | G0; RV-D | Covered. "Twice" only across two tables, which is intended; G0's source is unstated (N-12) |
| **The T6S consistency notes** (I74 §4.3) | B2's TS lane (combination basis refs; T6S suites); B3b-T (output-policy entry, Rust golden); decision 21 | T6S suites in the full suite; SK2 | Covered. The mapping already exists (N-8) |
| **Decisions 17–19** (DESIGN §8) | Decisions 22–24, prepared for the owner | — | Covered, owner-held, not decided |
| **`physics-retained-1`** (DESIGN §5; DN §4.4) | B3b (§1.4: table, definition, D1.3/D1.5, readers, carriers, coexistence) | B3-D's RV-D; SQ2 on the exact route; SG2 coexistence | Covered; memory unpriced (B3-S, decision 26) |
| **`legacy_pressure_v1`** (DN §4.3; DESIGN §5 item 7, N-8) | B3a (§1.3) | 07o pins and mutations; three readers' G8 | Covered (N-11's reading) |
| **"Combination formation completion"** (C3 §6; C3_DELTA:67) | — | — | **Missing (S-1)** |
| **Kernel provenance and registration for prepared and unavailable operands** (SC1 §1; C2 §3–§4) | Partly B2-K | — | **Incomplete (S-2)** |
| **The registered build across statics changes** (D-6, `build_status`) | — | — | **Missing (S-4)** |

**Assigned twice:** SCHEMA's installation (§1.2.2's readers' lanes and §1.2.7's statics owner), and the two carrier schemas (statics and lane T); both are S-3. Nothing else is double-assigned.

## 4. Units, order and estimates (item 4)

**B2 as one unit with two stages: workable and right.**
- The certificate for combination owners is shared.
- B2a alone is DESIGN §4.2's P3, which RR:7051 allows only as a declared interim, and inside one branch it never reaches main.
- B2a first and B2b second is a sensible internal order.

**B3 split into B3a and B3b: workable and right.**
- B3a is one contract value plus three readers' G8. The ordinary route's only gate is the non-zero pressure refusal, and `source_recovery` stays legacy-only.
- B3b is a new identity, table, definition and reader branch.

**Phase 0 beside B1: workable, with N-9.**
- Designs and Python pricing need no lock.
- B2-W and B3-W need cargo behind B1's heavy jobs and count as implementers.

**The order after J0: workable.** B3b-P runs on lane P while B2-K runs on lane K, and B2-P must follow B3b-P because the files are shared. The cost is that B3-D sits on the critical path; risk 4 and decision 27 cover it.

**S-4 changes the phases.** The statics' landing must come with an interim registration, or move late.

**Lane write sets: split by file, except S-3.**
- I checked every path in §1.2.7 and §1.4 against the tree. All exist where named.
- `PP-tests/retained_precision_admission.rs` (lane P) and the runner's `retained_precision_admission.rs` (lane A) are different files.
- The 12-file PTABLE cascade falls across lanes A, RS, PY, TS and statics as stated.

**The single corpus writer (07o) and one SQ2 / one PR-B2 with a split fallback (decision 27): sound.** PLAN's decision 8 is ROOT's, and a separate PR-B3 before B7 leaves the owner's order intact.

**Estimates: honest as reading estimates, but restate the totals (N-5).**
- The growth over I61 is explained: kernel certificate, B3b's definition, the readers' combination validation, and now S-1's definition.
- The calibration against B1, U8 and T6S is fair: B1 grew in planning and is seeing repair rounds, while U8 and T6S came in early.

## 5. The decisions (item 5)

Deciders checked against the work graph's owner-held list (dense and lane ceilings; PHYS-R4; observation framing; KF3; KF2; the supported-machine statement or M above 12 GiB; public meaning; the native-app witnesses; R-2). **None is decided by the plan.**

| # | Decision | Verdict | Reason |
|---|---|---|---|
| 1 | B2: one unit, two stages, one PR | **AGREE** | Shared certificate and receipt families; P3's interim never reaches main; one re-qualification |
| 2 | B3a and B3b, both in PR-B2 | **AGREE** | Different sizes and identities; PLAN decision 8's PR-B2; decision 27 keeps a split |
| 3 | Order: phase 0 now; B3a's admission first; B3b-P beside B2-K; then B2-P; one SQ2 and PR-B2 | **AGREE**, with N-9 and S-4 | Uses lane P while B2-K runs; B2-P must follow B3b-P on shared files |
| 4 | Components out of B2 | **AGREE** | FK's source has `StraightMember` only (`source.rs:168, :274`); DN §4.2 puts components in W1c and later |
| 5 | Retained iff T0R passes and ≥ 1 operand selected after T-9 | **AGREE** | SC1 §2's first row; ordinary-only mechanics stay on their path (SC1 §1); D2 §4.9.2's "a successor has a selected case" holds |
| 6 | Operand sources | **AGREE**, subject to S-2 | Restates C3a rule 1; the unavailable operand's kernel prep is unspecified |
| 7 | Per-combination unavailability; whole-successor abandonment set; no combination notice | **AGREE** | C1 §5 (rows stay, diagnostics name the combination); DESIGN decision 5's set; T-12's notices are per case in A |
| 8 | R-COMB-1 (row-level `not_covered`), owner informed | **AGREE**, with N-6 | Enforces C1 §5's "operand standing remains enforceable" and C1 §6; not public meaning while no successor is public (B0's precedent) |
| 9 | Revise PTABLE in place; bind RV78-N1's four policies at the same revision | **AGREE**, with S-1 and N-12 | Its scope text forbids B2's extension; RR's "not revised for it" rested on B1's R8 cost (RR "I86's SW probe accepted…"), which no longer applies after J0; no successor or corpus byte carries the hash (§6) |
| 10 | C_eq = c + z ≤ 3, ≤ 3 terms, ≤ 3 range operands | **AGREE** | Priced within 6 MB of S3; keeps M; also keeps Li from binding (N-4) |
| 11 | The explicit-row rule at PR-B2 | **AGREE** | PLAN_v2 §3.7 deferred it here; it saves repeated sweeps at B7, F2b, F3 and S-I2 |
| 12 | D1.7 unchanged for B3a | **AGREE**, with N-11 | Consistent with 0.1/0.2 today; zero-magnitude loads fall back with ordinary bytes |
| 13 | `RP-PREPARED-EXACT-DUAL-v1`; table binds RV78-N1 from v1; physics-1 untouched | **AGREE**, with S-1 | DESIGN §5 item 3 and decision 14; design B2's combination definition alongside |
| 14 | T-3 (c) unchanged for B3b | **AGREE** | physics-source-1 sets `source_block_recovery`; DESIGN §5 item 4 |
| 15 | B4 not now; ask the owner with R9 | **AGREE** | C_eq ≤ 3 fits B1's M; reach needs measured peaks (owner, 12 GiB decision) |
| 16 | B2-K is planned FK scope with a fresh numerical reviewer | **AGREE**, subject to S-2 and N-1 | The two refusals force it; other FK changes stay stops |
| 17 | Phase 0 now, documents and Python; probes under the lock | **AGREE**, with N-9 | No B2 code in NUM before PR-B1 |
| 18 | Fresh reviewers RV-C, RV-K, RV-D, RV-P2, RV-R2, RV-Q2, RV-X2 | **AGREE** | As B1 (SF-7); RV-K numerical, with an oracle (N-1) |
| 19 | PR-B1's gate set | **AGREE** | RR "T3's gate set…" with src-tauri, RV-X, Pass B, T9, both-entry and Direct gates |
| 20 | 07o: one writer, append-only; synthetic labelled | **AGREE** | `base_withheld` is unreachable in B2's domain (D1.5, D1.6 refuse every gate's trigger) |
| 21 | T6S consistency entries | **AGREE**, with N-8 | I74 §4.3; the mapping already exists |
| 22 | Decision 17 prepared (M above 12 GiB) | **AGREE** (owner's) | Not needed with decision 10; add N-4 to the package |
| 23 | Decision 18 prepared (R-2) | **AGREE** (owner's) | R-COMB-1 adds withheld-witness rows |
| 24 | Decision 19 prepared (native-app witnesses) | **AGREE** (owner's) | B3b adds `physics-retained-1`'s panels to B8's witness, for the owner |
| 25 | Witnesses W-CB1 to W-CB5, hooks, synthetics | **AGREE**, with N-7 | W-CB2 follows W-C2's case C; W-CB3 has a stop rule |
| 26 | B3-S before B3b-A; route caps or M ≤ 12 GiB | **AGREE** | Exact-route TEXT sites are zeroed today, so its price is unknown |
| 27 | Keep one PR-B2; ROOT may split PR-B3 | **AGREE** | PLAN decision 8 is ROOT's; the owner's order is unaffected |

**The routed owner items:**
- decisions 17–19 are prepared (22–24) and not decided;
- R-COMB-1 goes to the owner as information;
- B4's question rides R9.

All of this is correct.

**What the owner must be told before phase 0:**
- **Nothing owner-held needs a decision before phase 0.**
- **I recommend that ROOT inform the owner now, before B2-C's D1.4 text is selected, rather than at R9 or B8**, because changing these later costs more:
  1. **Reach.** At D1's caps an invocation with three load cases cannot also carry a combination within 12 GiB. The best estimate is about 13.25 GiB (13.0–14.0). B2's shape counts each combination of any kind as a case. If the owner wants 3 + 1, that is decision 17 (M above 12 GiB).
  2. **Size and kernel.** B2/B3 is about 200–315 h agent and 60–90 h review, about 4× the roadmap figure, and it brings planned changes to the solver kernel, with a fresh numerical review.
  3. **R-COMB-1.** In F2a, subtraction and range combinations over a Sensitive case, and unavailable combinations, will have their rows withheld from binding. DN §4.2 had listed subtraction and range under W1a.

## 6. Revising the preview table in place (item 6)

**The cascade is exactly as stated.** `git grep` of PTABLE's sha256 (`c74742ce…`) outside `P/execution` finds these 12 maintained files (`checks.txt` §4):
- `numericalResultQuality.ts`;
- `retainedPrecision.ts`;
- `compatibility.py`;
- `retained_precision.py`;
- `retained_memory.rs` (the registered `reviewed_inputs` text);
- `retained_memory_law_tests.rs`;
- RS `retained_precision.rs`;
- RE `semantic_contract.rs`;
- `analysis_run.v0.3.schema.json` (4 occurrences);
- `stress_neutral_export.v0.3.schema.json`;
- `test_load_reference_source_schema.py`;
- `test_retained_precision_schema.py`.

**No successor fixture, derivative fixture, corpus entry or carrier case carries the hash.** The successors name the contract id only. So the revision changes constants and hashes, never envelope bytes, as risk 5 says.

**B1's freeze: no conflict, if the revision lands on `b2` after J0.** B1's branches do not touch PTABLE. But the revision is a reviewed-input change and makes the build Stale until it is registered, which is S-4.

**B6's rulings: no conflict with B6's merged bytes.** B6 did not touch PTABLE. RR's ruling 1 ("The preview table is not revised for it") was argued from B1's critical path (an R8 stop for no change in behaviour). B2 must revise the table anyway, for its scope clause and, by S-1, its formation entries. Binding RV78-N1 in the same revision is therefore consistent with that ruling's reasoning. ROOT re-rules it explicitly (decision 9).

**With S-1:**
- if the ordinary definition is revised, its H moves, a second cascade of 6 files follows, and PTABLE's `product_formation_definitions` changes with it;
- if a combination definition is added, PTABLE gains an entry, and SCHEMA's `definition_id` becomes an enum in B2.

## 7. What would change the owner's F2a order (item 7)

**I93's list is correct:**
- B3b FK-heavy or unaffordable → activate on the preview route first: an order change;
- 3 + 1 wanted → decision 22: not an order change;
- D1-scale activation judged not worth it → B4 or W3-lowering code ahead of B7: an order change;
- R-COMB-1 → information.

**One owner-facing outcome is missing (N-10).** If RV-K finds the combination certificate unsound and ROOT does not take a second design round, B2 ships coverage-only. The plan's own risk 1 calls that an owner-facing scope change under RR:7051.

**For the owner's weighing, not a finding:** breadth before B7 is now about 12–17 working sessions after PR-B1 (N-5). The order is unchanged, but this is the schedule cost of keeping it.

## 8. For ROOT

**Rule on:**
1. The 27 decisions, with:
   - S-1 folded into decisions 9 and 13 and into B2-C's brief;
   - S-2 into decisions 6 and 16 and B2-KD's brief;
   - S-3 into §1.2.7's owners;
   - S-4 into the phase plan, choosing the interim registration (i) or late statics (ii).
2. **R-COMB-1's representation** (N-6): reader-derived or a receipt member. This is C1 §6's question returned.
3. **Re-ruling** RR's "not revised for it" for PTABLE (decision 9).
4. **When to inform the owner** (§5's three items): I recommend before B2-C is selected.
5. **How the amendments are carried:** into the phase-0 briefs, as B1's A1 amendments were (RR "B1's PLAN_v2 accepted…"), or as a short REVISION_01 from I93. S-4 changes §2.3's phases, so I lean to a short revision.

## 9. Execution record, what I read, and limits

**Executed:**
- Shell and Git reads.
- I93's three pricing scripts, rerun unchanged in scratch: outputs byte-identical.
- `evidence/rv114_price.py`, run once with I82's evaluator.
- Small read-only Python reads of committed JSON.

All of it in Python 3.13 from VENV, with `PYTHONDONTWRITEBYTECODE=1` and `TMPDIR` in scratch; `evidence/RUN.md` has the commands with placeholders. No cargo, vitest, native, solver or install, and no Git write.

**Read** (sha256 in `checks.txt` §1):
- the brief and I93's brief;
- PLAN.md in full, with its `_run_records/`;
- I61's PLAN §2.1–§2.2 and §5;
- DESIGN_v2 §0, §1.2, §4–§9;
- PLAN_v2 §3.7, §4, §8–§11;
- STUDY and ADD in full;
- C1 §2 and §4–§6, SC1 in full, C3_DELTA §1–§2 and §6;
- DN §4.2–§4.3;
- I74 §4.3;
- I42–I44's returns (combination passages);
- RV107's review (format and N-11);
- RR from "Owner decision: ROOT may raise M up to 6.0 GiB" through "I93's B2/B3 plan returned…", and RR:7036–7062;
- the work graph's T3 section, including its owner-held list.

**Code** at NUM:
- FK: `origins.rs`, `combine.rs`, `adaptive.rs` (`ExecutionOutcome`, `RetainedSolve`, `SourceBridgeView`), `product_certificate/{bridge,source_residual,final_case}.rs` (owner and load reads), `source.rs`;
- PP: `lib.rs` (`source_eligible`), `pressure_runtime.rs`, `source_recovery.rs`, `retained_memory.rs` (`family_clauses`, D-6 build status, `REGISTERED_PROFILES`), `build_identity.rs`, `build.rs`, `retained_facade_tests.rs` (`registered()`);
- SCHEMA, PTABLE and the ordinary definition JSON;
- the three readers' combination checks;
- T6S's two basis mappers;
- B1's `retained_memory.rs` and `lib.rs` on `b1`.

**Limits:**
- Like I93's, the memory figures are emulations on today's call graph through I82's chain. G5 replaces them, and B3b remains unpriced.
- S-2's and S-4's consequences are read from code, not exercised.
- I did not review B3-D's exact-route row recipes, which do not exist yet.
- I did not audit every RR line before "Owner decision: ROOT may raise M up to 6.0 GiB" beyond the citations above.
