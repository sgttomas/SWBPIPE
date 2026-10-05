# I61: the U8 plan, the F2a-breadth roadmap, and S-I1's readiness

TASK (Type 2), I61, for ROOT. Brief: `T3/RESUME_2026-09-30/BRIEFS/I61_U8_PLAN.md` (sha256 `07a27cd0…`, committed at NUM `cf4149f6ba`). 2026-10-05 UTC.

**Planning only.** I made no source edits, no Git writes, and ran no cargo, solver or native jobs. Git reads used `GIT_OPTIONAL_LOCKS=0`. The one merge test (§4) wrote to standard output only (`git merge-file -p --object-id`, `git hash-object` without `-w`). I wrote nothing to the system temp directory.

**Notation.**
- **WT:** the working checkout.
- **NUM:** the T3 integration branch `codex/piping-numerical-integrity-20260926`.
- **P:** `projects/chirality-piping`.
- **PP:** `P/core/product_physics`.
- **FK:** `P/core/solver/frame_kernel/src`.
- **T3:** `P/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3`.
- **R:** `T3/RESUME_2026-09-30`.
- **RR:** `T3/ROOT_RULINGS_V1.md`, append-only, so its line numbers are stable.
- **CR:** the merged package's `T3/IMPLEMENTATION/F2A_D1/CHANGE_RECORD.md`.
- **QUAL:** the same package's `copies/QUALIFICATION.md`.
- **S:** the PR's 140 maintained files (CR §1).
- **DOMAIN:** `R/I65/u4_g2_01/DOMAIN.md`.
- **DESIGN_NUMERICS:** `T3/DESIGN_NUMERICS/DESIGN.md`.
- **D2:** `T3/DESIGN_STANDING/DESIGN.md`.

**Basis.**
- Main `0b00b8e8b6`, the #1082 merge.
- NUM `697b402779`. ROOT merged main into NUM while I was planning, and NUM's maintained source now equals main's (§4).
- Code is cited at that basis.

## 0. Findings in brief

1. **Only the L = 0 base can produce a receipt inside D1.**
   - In a one-case invocation, every W1 failure publishes the ordinary bytes plus one N1 notice. No successor is published (`PP/src/lib.rs:3102–3170`; RR:8437, "a one-case invocation whose case is unavailable has no successor publication").
   - RV93 N-5 therefore yields a notice-only witness, and so does a native Ceiling inside D1.
   - **A receipt that carries a native Ceiling row needs a second load case that selects.** That is D1.4, the domain's one-case clause. No cap stands in the way: my construction is 4 nodes, 2 members and 5 supports.
   - I therefore propose splitting the Ceiling witness:
     - **W-C1, in U8:** a real-input Native fallback with the Ceiling reason, inside D1;
     - **W-C2, in breadth unit B1:** the receipt Ceiling row, as B1's acceptance witness, together with D38's pin, which needs the same multi-case shape (RR:9117).
2. **The L = 0 base fits D1 and its caps** (DOMAIN §4; D-9 at RR:8884). Its one open question is the producer's. Does the preview producer admit a memberless, fully restrained node? And if it does, does W1 then select and certify? U8 answers both with a probe, before any test is committed.
3. **N-5 is cheap and fits D1.** RV93 built the input: the milestone with only its first load (RX moment). It falls back at Candidate. The same RV93 probe also reached Preparation from real input, through a 1e-300 spring. Neither is committed today: the committed Native, Candidate and Preparation fallback tests are driven by hooks (`PP/src/retained_facade_tests.rs:600–646`).
4. **U8 can stay test-only.** It needs:
   - PP test additions;
   - two fixtures;
   - a corpus snapshot;
   - updates to the reader tests.
   
   If no reader defect appears, no production text changes. That leaves the D1 call graph and the registered identity untouched, so Pass B's no-build path applies.
5. **NUM has already absorbed main** (`697b402779`; parents `cf4149f6ba` and `0b00b8e8b6`).
   - The merge-base is now `0b00b8e8b6`.
   - Outside the execution records, NUM and main differ in 0 files.
   - My dry run beforehand predicted exactly this: no conflicts, and both changed S files resolving to main's recorded merges (§4).
6. **S-I1 can start now, in parallel.**
   - Its write set lies outside S and outside PP's dependency closure.
   - It needs no identity and no re-qualification.

## 1. U8, concretely

### 1.1 What D1 permits (facts at the basis)

- **The coexistence check comes first.** `retained_w1` returns `Coexistence` when exact-block selected (`PP/src/lib.rs:3107–3109`), and `LateGate` on a G-B refusal (`:3110–3113`). It returns `Domain` outside D1.4 (`:3114–3116`, `w1_case_id` `:3088–3094`: exactly one load case and no combinations).
- **Then the notice is reserved, and the W1 stages run:**
  1. preparation;
  2. native;
  3. the frozen candidate;
  4. staging;
  5. the serializer;
  6. the precommit reader.
  
  Every failure publishes the ordinary bytes plus one N1 notice (`:3117–3166`).
- **Preparation checks no ordinary quality.** It requires `MECHANICS_SOLVED`, the preview contract and no exact-block selection (`PP/src/retained_product.rs:3324–3327`). So any admitted case in D1 reaches W1, whether Passed or Sensitive. This is what made RV93's variants reachable, and W6 (below).
- **Native solves one source** (`retained_product.rs:3462`). It is `Err(NativeUnavailable)` unless the case is Selected (`:3463–3466`). So an `Unresolved(Ceiling)` case is a Native fallback, with no row on the wire.
- **The ladder ends `Unresolved(Ceiling)`** after the third candidate, or after an escalating verification failure (`FK/structural/retained/adaptive.rs:4505–4766`, `run_schedule_inner`; the reason is at `:2885`).

### 1.2 The three witnesses

| | **RV93 N-5: real-input Candidate** (plus the Preparation sibling) | **L = 0 base** | **Native Ceiling** |
|---|---|---|---|
| **What it proves** | <ul><li>The Candidate fallback (native selected, then the product certificate or full-case check refuses, `retained_product.rs:3652ff`) is reached from a real D1 input, not only from a hook.</li><li>It publishes the plain bytes plus exactly one notice, from one ordinary run that reaches G-C once, in both modes.</li><li>The sibling proves the same for Preparation.</li></ul> | <ul><li>**The producer's coverage of an extent-0 body:** a single-node body, all six displacement rows input-derived, a translation with one non-input row and a rotation with none, stop `[F,F,F,F]`, has_data false (R/I62/coverage_shared_python_01/SNAPSHOT_05_PLAN.md §1.2).</li><li>**The feasibility rule's L = 0 branch on producer-solved bytes,** in all three readers. Today only each reader's own tests cover it (RR:7636).</li></ul> | <ul><li>**W-C1 (U8):** the real-input Native fallback. W1 runs, the kernel ends `Unresolved(Ceiling)`, and the product publishes the plain bytes plus one notice. Today Native is reached only through `withdraw_next_native_source` (`retained_facade_tests.rs:606`).</li><li>**W-C2 (B1):** the receipt's unavailable row with `selection` `{space: unresolved, tag: ceiling}` (schema `retained_precision_mp_v2.schema.json:739–752`) beside a selected case. This is the "genuinely different numerics" of RR:7636.</li></ul> |
| **Real input** | **RV93's construction** (`R/REVIEW_RV93/u3_grant2_01/evidence/probe/zz_rv93.rs:292–315`):<ul><li>`first_load_only`: the milestone request with `primitive_loads[0]` alone;</li><li>`tiny_spring`: support 1's stiffness set to 1e-300.</li></ul>Both are derived in the test from `P/fixtures/product_preview/rf_skew_t_cant_off_122_r1e-04.request.json` | **The milestone request plus:**<ul><li>one node that no member references;</li><li>one rigid support on that node, restraining UX, UY, UZ, RX, RY and RZ.</li></ul>It is derived in the test; no new request fixture is needed | **W-C1, preferred:** case B of W-C2's pair (below), so that U8 commits the input B1 needs.<br><br>**Fallback: W6's input** (`retained_memory_witness_tests.rs:181–199`, PHYS-R4's cantilever, OD 4e-77 m, in the 0.1.0 namespace). It already ends `Fallback("Native")` in both modes (`R/I65/u4_g6_01/_run_records/per_identity/witnesses.test.txt`).<br><br>**The native reason** is not recorded in any record I found. THIN-A/B, the same geometry at kernel level, end `Unresolved(Ceiling)` (RR "V-K: rulings on I17's A1 stop", RR:2175). The U8-0 probe establishes W6's reason |
| **Inside D1?** | **Yes.** RV93 recorded it in D1, and only the load count falls (l = 1). Every other fact is the milestone's (QUAL §6: D1.0–D1.11) | **Yes**, with no cap change (DOMAIN §4; D-9):<ul><li>n 3, g 5, Σr 12, l 3, unchanged materials;</li><li>raw values about 180 against 16,384, at the same depth.</li></ul>D1 places no connectivity condition | **W-C1:** yes. W6 and pair case B are each one case, in the 0.1.0 namespace, with an anchor or rigid support, nodal loads and a straight member. W6 reached W1, so it passed G-A.<br><br>**W-C2:** **no.** D1.4 allows one load case. The pair is otherwise inside the caps (n 4, m 2, g 5, materials 2, l ≤ 3 per case), so D-9's cap concern does not arise. Its cost is B1's (§2) |
| **Producer path** | Direct entry, then `permitted_dispatch`, then `retained_w1`:<ul><li>**Candidate:** `freeze_candidate` returns Err, giving `W1Fallback::Candidate` (`lib.rs:3130–3133`);</li><li>**Preparation:** `prepare_case` returns Err, giving `W1Fallback::Preparation` (`:3121–3124`).</li></ul>No receipt | Direct entry, then `retained_w1`, then:<ol><li>preparation (two bodies; the kernel's extent-0 handling, FK `adaptive.rs` `coupled_scales`);</li><li>native Selected;</li><li>freeze;</li><li>staging;</li><li>`serialize_frozen`;</li><li>precommit;</li><li>the successor transfer.</li></ol>The receipt is emitted | **W-C1:** `solve_native` returns `Err(NativeUnavailable)`, giving `W1Fallback::Native` (`lib.rs:3127–3129`). No receipt.<br><br>**W-C2:** B1's multi-case transaction. Case A is Selected and case B is Unresolved(Ceiling), giving one successor with case B's ordinary outcome and its unavailable row (C1's unavailable branch, D9b/D19) |
| **Corpus and the three readers** | **No corpus change**, because there is no successor. The notice in an ordinary envelope is already accepted by the base readers (`retained_facade_tests.rs` `u3_r2_base_readers_accept_the_unavailable_notice`). No reader changes | **Snapshot 07l** (I62), §1.4.<br><br>**Expected:** no reader source changes. Python, Rust and TS change only their test counts and pins: (15, 278, 24) becomes (17, 278 + k, 24 + j); see `tests/test_retained_precision_contract.py:640–643` and `result_export/tests/retained_precision_contract.rs:276–281`.<br><br>**If any reader rejects a faithful base,** that is a reader defect:<ul><li>a Python or TS fix is a reader review only;</li><li>a fix in the **Rust reader's source** is a change to the D1 call graph (precommit). It re-runs TEXT, RV87's non-candidate review, Pass B and RV89 (RR:10474; QUAL §11).</li></ul> | **W-C1:** none (no successor).<br><br>**W-C2 (B1's snapshot):**<ul><li>a producer-solved two-case base with the Ceiling row;</li><li>D38's relaxation in three readers (RR:9117), which is reader source, so it is re-qualified with B1;</li><li>the reader-local pin D13, "the Ceiling after a p128 verification-solve failure" (RR:8072ff), stays local unless the producer chain matches it</li></ul> |
| **Tests and controls** | **One committed test** beside the hooked one, looping over both modes and both variants. **Registered build:**<ul><li>the expected cause;</li><li>`ONE_RUN_THROUGH_G_C`;</li><li>`notices == 1`;</li><li>the bytes equal `with_notice(plain_variant, case, None)`;</li><li>admission refusal `None` (in D1);</li><li>`hooks::armed_names()` empty before and after.</li></ul>**Unregistered build (hosted CI):** the plain bytes and `ONE_RUN`.<br><br>**Controls:**<ul><li>the full milestone still publishes its pinned successor (the existing test);</li><li>mutant: restoring all three loads must fail the Candidate assertion;</li><li>mutant: dropping the notice must fail the byte assertion</li></ul> | **The PP pin test, in both modes:**<ul><li>B′, the envelope equals the plain run;</li><li>`ONE_RUN_THROUGH_G_C`;</li><li>the pinned file and receipt sha256;</li><li>the publication is the successor;</li><li>it writes the fixture under an output variable, as `I61_U3G2_OUT` does.</li></ul>**D-U6-5 style:** the corpus base and the fixture are byte-identical to the live successor, checked by sha256.<br><br>**Value controls:**<ul><li>body 1's rows are input-derived or exact zeros;</li><li>body 0's rows agree with the milestone's independent reference (U5) within the unchanged criterion. Equality of bits with the milestone is reported, not assumed.</li></ul>**Mutations** (from SNAPSHOT_05_PLAN §1.2, e.g. `isolated_rotation_stop` at G5a SCALE) fail at their expected gate in all three readers | **W-C1, registered build:** cause `Native`, one notice, the plain bytes, admitted. **Unregistered build:** plain.<br><br>**The reason "Ceiling"** is recorded by the U8-0 probe, not asserted in committed code (decision 3). The wire-level assertion arrives with W-C2.<br><br>**Control:** pair case A, alone, publishes a successor. That shows the Ceiling comes from case B's loads, not from the model |

### 1.3 The W-C2 construction (built in U8, used in B1)

**The aim** is one model with two cases that genuinely differ numerically.
- Cases in one invocation share the stiffness, so the difference must come from the loads.
- **Candidate pair (two bodies):**
  - body 0 is the milestone's skew cantilever;
  - body 1 is PHYS-R4's cantilever (OD 4e-77 m, L = 1 m; EA/(12EI/L³) ≈ 2^507);
  - case A loads only body 0, and case B loads only body 1.
- **Why case A should select (expected, not established):** body 1 is a disconnected, unloaded block, so its rows should be exact zeros at every precision, and with S\* = 0 they need only exact agreement.
- **Why case B should end Ceiling:** it does at kernel level, for THIN-A/B.
- **Untested risk:** a group-level check, such as the factor screens or THIN's 512-bit charge test (d), could also reject case A.
- **The fallback pair:** one body, PHYS-R4's x-aligned cantilever, with case A axial-only (the axial and bending DOFs are exactly decoupled for an axis-aligned member) and case B transverse.

**The probe (U8-0) uses only D1.** It runs each case as its own one-case request through the real Direct entry:
- A should publish a successor;
- B should give a Native fallback, with the reason recorded.

The per-case native outcome depends only on the model and that case's loads, so the two one-case runs predict the two-case invocation's rows. B1 then commits the two-case invocation as its witness.

### 1.4 Slices, write sets and allocation

| Slice | Owner | Write set | Content | Estimate |
|---|---|---|---|---|
| **U8-0, probe** | I61 | WT/scratch/i61_u8_probe/ (a disposable `git archive` of NUM); WT/targets/i61-u8/; NUM R/I61/u8_probe_01/ | In the registered dev/test build, run probe tests (not maintained code):<ol><li>`first_load_only` and `tiny_spring` in both modes, recording the cause and the cfg(test) `I51_FROZEN_REFUSAL` line;</li><li>W6 in both modes, recording the kernel `UnresolvedReason` with a probe-only print;</li><li>the L = 0 variant: the ordinary outcome, the admission and the W1 outcome in both modes, and, if a successor publishes, all three readers on it with the invocation;</li><li>W-C2's pairs (§1.3), each case alone.</li></ol>Output: PROBE.md and SHA256SUMS | 2–3 h |
| **U8-1, PP witness tests** | I61 (the PP integration owner, D-5) | `PP/src/retained_facade_tests.rs`; new `P/fixtures/results/retained_precision_l0_successor_{sparse_interactive,dense_scrutiny}.json` (only if L = 0 publishes) | **Three tests:**<ul><li>`u8_real_input_fallbacks_append_one_notice` (Candidate and Preparation, and Native via W-C1's input);</li><li>`u8_l0_isolated_node_publishes_pinned_successor`;</li><li>a D-U6-5 equality test for the L = 0 fixtures.</li></ul>**W-C1's input** is written inline. W6's `w6_input()` is `pub(super)` in I65's witness file, and U8 does not edit that file | 2–3 h |
| **U8-2, corpus 07l** | I62 | `P/fixtures/results/retained_precision_cases.json`; `P/tests/test_retained_precision_contract.py` | **Additions:**<ul><li>two producer-solved bases, with case `provenance` naming the producer, the entry and the build;</li><li>the L = 0 mutations and must-pass entries;</li><li>the top-level `provenance.claim` amended (decision 6).</li></ul>Python passes, or a defect is returned | 2–3 h |
| **U8-3, Rust and TS alignment** | I63 (Rust), I64 (TS) | `result_export/tests/retained_precision_contract.rs`; `apps/desktop/src/features/results/retainedPrecision.test.ts` | Counts and pins; each runs the full 07l. **Not** the Rust reader's `src/` `cfg(test)` module | 1–1.5 h each |
| **U8-4, Pass B (no-build gates, under the build-gate argument RV89 accepted on F′)** | I65 | records only: NUM R/I65/u8_passb_01/ | **The expected delta** is test-class rows only: the facade tests, the reader tests, the fixtures and the corpus. Every no-build gate is 0, and the registered entry is unchanged.<br><br>**The tool** is I65's fail-closed `g7_pass.sh` (R/I65/u4_g7_03/_run_records/). The one-off `g7_pass_nobuild.sh` is reused only after RV89 N-1's restorations: the verdict, the exit code, the early stops, the `text_summary` gate and the delta-tool failure handling | 1–1.5 h |
| **U8-5, reviews** | RV93 (PP tests and the probe; author of N-5); RV78 (07l parity across three readers; RV79 if Python changes); RV89 (confirms Pass B) | their own R/REVIEW_* folders | Independent oracles. **RV93** re-derives the causes and bytes. **RV78** re-runs the 07l mutations in all three readers | 2–3 h, 2–3 h, about 1 h |
| **U8-6, freeze gates** | ROOT | — | The full 40-manifest DEC-025 on the candidate (RR "DEC-025 on F finds…"); hosted CI (Stale route) | about 2 h machine |

**Order.**
1. U8-0.
2. U8-1. RV93 may start once U8-1 returns.
3. U8-2.
4. U8-3a and U8-3b in parallel.
5. U8-4.
6. RV78 and RV89.
7. U8-6.

S-I1 runs alongside throughout (§3). The host rule stays as before: one cargo job at a time, memguard 5387, `--locked --offline`, `CARGO_BUILD_JOBS=4`, `RUST_TEST_THREADS=2`, own target directories.

**Stop rules.**
- **L = 0 not admitted, or W1 falls back:** L = 0 stays deferred with the recorded cause. U8 makes no producer change, since admission is ordinary-route behaviour and the D1 call graph. Re-route it to B1 (decision 5).
- **W6's reason is not Ceiling and neither pair yields a Ceiling:** return with the probe record. W-C1 then commits only the "real-input Native" fact, with its actual reason.
- **A Rust reader defect in 07l:** stop and return. Re-qualification is ROOT's call.

**The U8 estimate** (agent hours / review hours):

| Case | Agent | Review | Plus |
|---|---|---|---|
| L = 0 publishes | 11–16 h | 5–7 h | about 2 h of DEC-025 machine time. Elapsed: about 1.5–2.5 working sessions |
| L = 0 deferred at the probe | 6–9 h | 3–4 h | |
| Contingency: a Rust reader source fix | +8–14 h | +3–4 h | |

**For comparison,** the step-4 plan put the two producer-solved witnesses at 4–8 h (`R/I61/step4_plan_01/PLAN.md` §U8). This estimate adds N-5, W-C1, W-C2's probe and the reviews.

## 2. The F2a-breadth roadmap (U8 to the release registration)

### 2.1 Units

The §5.2 obligations come from CR §5.2 and RR:11873–11881.

| Unit | §5.2 obligations | Changes the D1 call graph? | Widens the domain? | Changes the contract or readers? | Depends on | Rough estimate (agent / review) |
|---|---|---|---|---|---|---|
| **B0. Contract and identities (design)** | The receipt and freeze transaction beyond D1, specified. The C3 amendment for the preparation-only operand capability (RR:7051–7056). D38's reader relaxation, as text. ROOT's reservation of `<physics-retained>` (DESIGN_NUMERICS §4.4) | No (documents) | No | Contract text: yes | U8 | 6–10 h / 3–4 h |
| **B1. Multi-case** | The transaction for n cases. Multi-case invocations without combinations. The complete invocation's exact-block no-attempt rule (coexistence over every case, DESIGN_NUMERICS §4.4). D38's pin. **W-C2.** | **Yes:** `w1_case_id`, the notice per case, preparation, `solve_native` with all sources, freeze, staging, serializer, and the Rust reader's D38 relaxation | **Yes:** D1.4 to n cases. The caps restated per invocation (D1.9's l counts `load_cases[0]` only). M's profile re-priced: the dense in-build maximum is already 0.8929 M with 28.4 MB of headroom under 0.9 M (QUAL §3) | **Readers:** D38 ×3. **Corpus:** two-case producer-solved bases, including the Ceiling row | B0 | 30–45 h / 10–14 h, including the re-qualification below |
| **B6. Reader items (corpus)** | RV78-N1, the rehash-index rule (RR:9461). The N-3 G7 codes (RR:11246). Routed: RV94 N-5 | No, **if** TS aligns to the Python and Rust code (decision 11) | No | **Readers:** harnesses ×3, the corpus format, TS's base code | 07l; a single corpus writer, so serialized with B1's snapshot | 5–8 h / 2–3 h |
| **B2. Combinations, preparation-only and mixed** | Combinations; preparation-only and mixed invocations | Yes | **Yes:** combinations (D1.4), possibly components | **Contract:** the C3 amendment. **Readers** ×3: combination receipts. **Carriers** | B1 | 25–40 h / 8–12 h |
| **B3. The promised exact routes** | 0.3.0 exact with explicitly empty pressure regions, under `<physics-retained>`. Also 0.3.0 `legacy_pressure_v1` with zero pressure on the ordinary route (DESIGN_NUMERICS §4.3) | Yes | **Yes:** D1.3 (namespace) | **New identity and table.** The table becomes a reviewed static, so the identity text changes and needs a registration diff. **Readers:** S-G1's `<physics-retained>` branch ×3 (D2 §7). **Carriers** | B1 (and B0's reservation) | 25–40 h / 8–12 h |
| **B4. Cap growth (proposed; not in §5.2)** | Resource qualification above D1.9's caps | Possibly (pricing in `retained_memory.rs`) | **Yes:** D1.9 | No | B1–B3's final profile | First a 3–4 h study, then 15–30 h / 5–8 h |
| **Re-qualification, per main-bound candidate** | CR §5.2 item 2; QUAL §11; RR:10474 | — | — | — | each PR | See below |

**Re-qualification cost** (the G7-style rerun on the registered dev/test identity).
- **A call-graph change with no domain change:** TEXT, the identifier audit, RV87's non-candidate sweep (or its explicit-row inversion), Pass B and RV89. About 6–10 h agent and 3–4 h review.
- **A widening:** the above, plus:
  - the profile re-derived for the new domain (G5);
  - the in-build maximum against M (G6);
  - the S1 stack witnesses for the new paths (W1–W7, plus a multi-case witness);
  - the registration diff.
  
  About 15–25 h agent and 6–10 h review.
- **Every freeze** first runs the full 40-manifest suite.

**Order.**
1. U8.
2. B0.
3. B1 and B6. B6 follows 07l, and its corpus edits are serialized before B1's snapshot.
4. **PR-B1** to main.
5. B2 and B3 in either order. B3 may start after B1, given B0's reservation.
6. B4, if ruled.
7. **PR-B2** to main.
8. The release registration (B7).
9. Activation (B8).

Two main-bound PRs hold re-qualification to two full runs (decision 8). The total for B0–B3 plus B6, without B4: about **91–143 h agent and 31–45 h review**, plus two PR gate runs of about 6–10 h wall each. These are reading estimates of the same kind as the step-4 plan's.

### 2.2 The end of F2a: release registration, activation, and the T6 conflict

**B7: the release identity, registered once.** It comes after the last D1 call-graph change of breadth (step 3 of the owner's order).
- **The identity:** profile `release`, `opt_level=3`, `debug_assertions=false`; otherwise the registered text (QUAL §5).
- **The work:**
  - re-run the profile record and witnesses on it (at G6 they equalled the dev record);
  - re-establish the milestone's, U8's and breadth's witness bytes and verdicts on it (RV95 N-6, CR §5.2: platform `hypot`, target-dependent dense bytes);
  - add the registration-table entry (production data; Pass B and RV89 confirm);
  - teach the tests' `registered()` helper both identities.
- **Estimate:** 6–10 h agent and about 3 h review, plus machine time.

**B8: public activation** (step 4), under the checklist (CR §4; RR:11254).

| Item | What it needs |
|---|---|
| 1 | Tauri's `qualify_rule_mechanics_with_context` accepting the successor with its invocation |
| 2 | Memoizing the binding |
| 3 | **Native Current for successors:** the owner's Mac (G10's kind, an owner action) |
| 4 | T6's outputs |
| 5 | RV94 N-4 |
| 6 | The 32-bit review |
| 7 | A fresh review |

**Also: the desktop is a new Direct caller from another workspace.** It has its own lock, which requires caller qualification (RR:10407, N-1), and the native-window premise that I53 left unproved (RR "Corrected caller interface accepted…").
- **Estimate:** 20–35 h agent and 8–12 h review. This excludes T6, and the native-caller qualification is the large unknown.

**The T6 conflict.**
- **Checklist item 4** requires T6's successor outputs: result export and stress-neutral, replacing the explicit N-5 panel refusal, plus the v0.3 dispatcher (CR §5.5).
- **It blocks** step 4, and through the ruled order, step 5: S-I2, F2b per family and F3.
- **It does not block** U8, breadth, B7 or S-I1.
- **The owner must choose one of:**
  - **(a)** pull a narrow T6 successor-output slice forward, in parallel with breadth (it writes the export paths and panels, not PP);
  - **(b)** activate with the explicit refusal kept and disclosed, which amends item 4. Exports of W1-selected results would be refused until T6, where today's ordinary results export;
  - **(c)** hold activation, and with it F2b and F3, until T6 runs in its planned slot.
- **Recommendation: (a),** raised now rather than when breadth is nearly done, because its lead time is otherwise serial (decision 12).

**After registration.**
- **S-I2** edits `semantic_contract.rs`, in the precommit reader's crate. **F2b** edits PP's selection order, and **F3** edits PP's load builders.
- F2b and F3 are on the D1 call graph. So each re-qualifies **both** registered identities, which is the cost ROOT told the owner (RR:11868).
- S-I2 can avoid this if its Rust functions stay off the precommit call graph (decision 14).

## 3. S-I1's readiness

**The write set** (D2 §4.11.6, §7):
- `P/core/rules/expression_evaluator/src/lib.rs` (2,331 lines) and its tests;
- `P/core/rules/rule_check_runner/src/lib.rs` (1,764 lines) and its tests;
- new `P/core/analysis_runs/rule_interval.py`;
- new `rule_interval_cases.json`. I propose a new folder, `P/fixtures/rule_interval/`, so that the conformance-corpus walkers stay unaffected (they read `fixtures/rule_expressions/conformance_corpus/`).

**Dependencies.**
- The accepted design: D2 §4.11 (DD-13 ruled C).
- The formula grammar it covers: `expression_evaluator` and `rule_pack_document`.
- The runner's synthesized `Compare`.
- No identity, no F2a code, no S-G receipt. The new codes go into existing free-string fields (`rule_check_run_result.schema.json`: `diagnostic_codes` and `RunFinding.code` are `string`), so no schema change.

**Can it start now without touching F2a's files? Yes.**
- **Disjoint from S:** none of the 140 files is under `core/rules`, and neither branch changed `core/rules` since `381be775ae`.
- **Outside PP's dependency closure:** PP's dependencies are canonical_json, curved_bend, frame_kernel, load_case_algebra, linear_supports, nonlinear_integration, nonlinear_supports, primitive_loads, solver diagnostics, sparse_direct, straight_pipe, stress_recovery, units and result_export. result_export depends only on serde_json, canonical_json, sha2 and units.
- **So:** no TEXT, no Pass B, and no change to the registered identity's reviewed inputs (the PP lock and the 13 statics).
- **The only consumer outside the rules crates** is src-tauri, through `rule_check_runner`. S-I1 adds no dependency, so the src-tauri lock in S does not move.
- **The shared resource is the host:** one cargo job at a time, with U8.

**Recommended route.** Its own branch from main, with records on NUM and its own PR to main, independent of F2a (decision 13).

**Review needs.**
- **One fresh independent reviewer,** not an F2a reviewer, for:
  - the soundness table, rule by rule;
  - the two named mutants (removing the outward step; the U arm of `compare` replaced by T);
  - Rust/Python parity over the case file;
  - the negative controls (D2 §4.11.5).
- **The committed differential:** byte-identical `RuleCheckRunResult` over the committed rule packs and run fixtures.
- **Gates:** hosted CI and the full 40-manifest DEC-025 (rules crates and src-tauri). There is no D1 gate.

**Estimate.**
- **Code (D2's sizing):** about 700–1,000 Rust lines, about 400 Python lines and about 60 cases.
- **Agent time:** 10–15 h.
- **Review:** 4–6 h.
- **Gates:** about 3–5 h wall.

## 4. Main's movement

**State now.**
- ROOT merged main `0b00b8e8b6` into NUM as `697b402779` (parents `cf4149f6ba`, `0b00b8e8b6`) during this planning, so the merge-base is now **`0b00b8e8b6`**.
- Outside the execution records, `git diff --name-only 0b00b8e8b6 697b402779` lists **0 files**: NUM's maintained source is main's.
- **The merge brought in 16 non-record files:**
  - main's PR1078/PR1080 versions of `compatibility.py`, `source_blocks.rs` and their two tests;
  - D-GOV-52's `AGENTS.md` and governance, workflow and export files;
  - main's records, including the package `T3/IMPLEMENTATION/F2A_D1/`.

**My prediction, made before the merge,** held exactly.
- **From the old base `381be775ae`, against the pre-merge NUM head `cf4149f6ba`,** 140 files had changed on both sides (exactly S). 138 were byte-identical. I re-ran this check against `cf4149f6ba` explicitly after the merge appeared.
- **For the other two, three-way merges to standard output** (`git merge-file -p --object-id`) were conflict-free. Each equalled main's blob, which is the recorded three-way merge (CR §2):
  - `compatibility.py` gave `767da34027c5`;
  - `source_blocks.rs` gave `e8aadb4189d9`.
- **There were no S conflicts.**

**The procedure for each later absorption,** before a U8 or breadth implementation grant:
1. **Dry run.** List the files changed on both sides since the merge-base, and run `merge-file` to standard output for each that differs. This needs no Git writes.
2. **Flag:**
   - any S file;
   - any crate in PP's dependency closure;
   - PP's `Cargo.lock` or any of the 13 reviewed statics. These make the registered build Stale at D1.1 until re-registered.
3. **After ROOT merges:**
   - G9b's Direct-caller scan;
   - a re-qualification note if the D1 call graph moved (PR1080's precedent: RV89 showed unreachability).

**Open PRs against main** (read with `gh` at about 02:05Z).
- **#1083 (App v4):** touches only `projects/chirality-app-v4/**`. No S overlap.
- **#885 (draft, piping live control, last updated 2026-09-25):**
  - It touches `apps/desktop/src-tauri/{Cargo.toml, src/lib.rs, …}` and desktop workspace files, none of them in S.
  - It overlaps S-I2's and checklist item 1's write set (`src-tauri/src/lib.rs`).
  - It calls no retained entry.
  - Watch the src-tauri lock, which is in S, if its dependencies change.

## 5. Decisions needed

| # | Decider | Decision | Recommendation |
|---|---|---|---|
| 1 | ROOT | **Grant U8's runs:** U8-0's probe (a disposable archive) and U8-1's tests, in the registered dev/test build, with memguard and one cargo job, in their own target directories. No DEC-025, native or solver-at-scale jobs | **Grant** with U8's dispatch |
| 2 | ROOT (owner informed) | **Split the Ceiling witness:** W-C1 (a real-input Native fallback, in D1) in U8; W-C2 (the receipt Ceiling row, which needs two cases) moved to B1 as its acceptance witness | **Adopt.** The alternative, a minimal two-case widening inside U8, is a full widening re-qualification (about 35–55 h with reviews) that B1 would then repeat. The owner's order is kept: only the row moves |
| 3 | ROOT | **How W-C1's Ceiling reason is evidenced** | **Record it by the U8-0 probe,** with no `cfg(test)` hook added to PP production files. The wire assertion comes with W-C2 |
| 4 | ROOT | **Whether RV93's real-input Preparation (`tiny_spring`) joins N-5's test** | **Include it.** Same test, about 15 min, and it closes the hook-only gap for Preparation as N-5 does for Candidate |
| 5 | ROOT | **The L = 0 stop rule** | **If the producer refuses the memberless node, or W1 falls back,** L = 0 stays deferred with its cause and moves to B1. U8 makes no producer change |
| 6 | ROOT | **How producer-solved bases enter the corpus,** and the snapshot name | **07l.** Producer-solved bases go in `retained_precision_cases.json` with case-level `provenance`, as D-U6-5 byte-identical copies of the PP-pinned live successors. The top-level claim is amended to "synthetic controls plus listed producer-solved bases; no native Current evidence" |
| 7 | ROOT | **U8's route to main** | **Its own small PR:**<ul><li>test-only;</li><li>G5–G8 carried by ruling, as for F to F′;</li><li>fresh no-build Pass B, DEC-025, hosted CI and one complete-diff confirmation.</li></ul>About 3–5 h of wall time, mostly machine. It keeps B1's PR reviewable. **Alternative:** it rides with PR-B1 |
| 8 | ROOT | **Breadth cadence** | **Two main-bound PRs** (PR-B1: B0, B1, B6; PR-B2: B2, B3, and B4 if ruled), with **re-qualification once per PR candidate** on the registered dev/test identity. No product caller exists meanwhile |
| 9 | ROOT (to the owner if machine-bound) | **Cap growth.** D1's caps (32 nodes) give activation little user value, and M's dense maximum is already 0.8929 M | **Commission a 3–4 h read-only study (I65) early in B1:** the target caps, and whether TEXT pricing can scale under M. If M must exceed the provisional 3.75 GiB target, or needs a machine statement, that goes to the owner (owner-held) |
| 10 | ROOT | **Reserve the exact-route identity** (suggested `openpipestress.result_semantics/0.3.0/physics-retained-1`) | **Reserve it at B0,** so that B3's reviewed static is known before PR-B2 |
| 11 | ROOT | **The direction of the N-3 G7 code alignment** | **TS aligns to Python/Rust's `SOURCE_NUMERICAL_CASE_INVALID`,** so the Rust base validators on the precommit graph do not change |
| 12 | **Owner** (via ROOT) | **The T6 conflict** (checklist item 4): (a) pull T6's successor-output slice forward; (b) activate with the refusal kept; (c) hold activation until T6 | **(a), raised now** rather than at breadth's end, so that it runs in parallel. (b) removes exports that work today for W1-selected models |
| 13 | ROOT | **Dispatch S-I1** | **Now,** as a fresh TASK on its own branch from main and its own PR, with a fresh reviewer, sharing the host's one-job rule with U8 |
| 14 | ROOT (the owner only if the order changes) | **S-I2 against the release registration** | **Keep the owner's order,** and require S-I2's Rust edits to stay off the precommit call graph, as verified by Pass B's classification. Only if that is impossible, ask the owner to move S-I2 before B7 |

**Flagged, not a decision now.** Activation's new Direct caller (the desktop workspace, with its own lock) and native Current both need the owner's Mac and a caller qualification that does not exist yet (RR:10407; I53's open premise). B8's planning should start from these.

## 6. Estimate summary

| Unit | Agent | Review | Notes |
|---|---|---|---|
| **U8,** L = 0 publishes | 11–16 h | 5–7 h | Plus about 2 h of DEC-025 machine time (ROOT). Elapsed: about 1.5–2.5 sessions |
| U8, L = 0 deferred | 6–9 h | 3–4 h | |
| U8's own PR (decision 7) | 2–3 h | 1–2 h | Plus 3–5 h wall |
| Contingency: a Rust reader fix | +8–14 h | +3–4 h | Re-qualification |
| **S-I1** (parallel) | 10–15 h | 4–6 h | Plus 3–5 h of gate wall |
| B0 | 6–10 h | 3–4 h | |
| B1, with re-qualification | 30–45 h | 10–14 h | |
| B6 | 5–8 h | 2–3 h | |
| B2 | 25–40 h | 8–12 h | |
| B3 | 25–40 h | 8–12 h | |
| **Breadth B0–B3 + B6** | **91–143 h** | **31–45 h** | Plus two PR gate runs of 6–10 h wall each |
| B4 (if ruled) | 3–4 h study, then 15–30 h | 5–8 h | |
| B7, release registration | 6–10 h | about 3 h | Plus machine time |
| B8, activation | 20–35 h | 8–12 h | Excludes T6 and the native-caller qualification |

## 7. What I read, and limits

**Read:**
- the brief;
- RR at 2175ff, 7617–7640, 8072–8090, 8436–8437, 8865–8902, 9105–9125, 9450–9465, 10400–10480, 10836–10875, 11055–11075, 11246 and 11812–11896;
- the merge record and ERRATA;
- CR §1, §2, §4 and §5, and QUAL §3, §5, §6 and §11, both at main `0b00b8e8b6`;
- DOMAIN §1, §2 and §4;
- the step-4 plan's §U8;
- DESIGN_NUMERICS §4.3–§4.4.1 and §6;
- D2 §4.11 and §7.

**Code read** at the basis:
- PP `lib.rs`, `retained_product.rs`, `retained_facade_tests.rs`, `retained_memory_witness_tests.rs` and `build_identity.rs`;
- FK `adaptive.rs`;
- the schema;
- the corpus inventory.

**Also read:** RV93's probe, I65's G6 witness logs, and `gh` PR metadata and diffs (#1083, #885).

**Limits.**
- **Nothing was run.** Every outcome attributed to U8's inputs is either quoted from a record (RV93's causes; W6's `Native`; THIN's Ceiling) or marked for the U8-0 probe:
  - W6's native reason;
  - L = 0's admission and selection;
  - the W-C2 pairs.
- **Estimates are from reading.**
