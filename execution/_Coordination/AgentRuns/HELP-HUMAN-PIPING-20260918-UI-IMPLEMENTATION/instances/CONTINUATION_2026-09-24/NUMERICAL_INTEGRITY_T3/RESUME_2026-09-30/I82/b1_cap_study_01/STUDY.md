# I82 B1-S: the cap and M study for multi-case (pricing only)

TASK (Type 2), I82, for ROOT (HELP_HUMAN, Agent 0), who is the return path. 2026-10-06 UTC. No descendants.

**The brief:** `R/BRIEFS/B1_S_CAP_STUDY.md`, sha256 `da1838924a3c5c9a33a84914a358ebfbc8a75c4d85d7678bae5ec3d9e65b039f` (verified before reading).

**What this is.** PLAN decision 9's read-only study, extended by DESIGN_v2 §6 steps 1–5. It proposes; ROOT selects. Nothing is installed, no maintained file changes, and no M or cap is selected here.

**What I ran.** Python only (VENV), against a `git archive` copy of main `d8c88774d0` and copies of I65's chain. No cargo, no solver, native or at-scale run, no DEC-025, no install, no Git write. Git reads used `GIT_OPTIONAL_LOCKS=0`. Scratch was in `WT/scratch/i82_b1_study/`. Nothing went to the system temp directory. §9 has the execution record.

**Notation.** WT, NUM, P, PP, T, R and RR as in the dispatch.
- **c:** the requested load cases, capped by C.
- **a:** the attempted cases, |A| ≤ c (T-4).
- **l:** loads per case, so every l_i ≤ l.
- **L:** total loads, Σ l_i ≤ L.
- **P:** P_final = 7n + 51m + 8g + 3 rows per case.
- **M_today:** 4,026,531,840 B.
- **6.0 GiB:** 6,442,450,944 B.
- **"0.9 M"** is the margin rule: E_mov,max + R ≤ 0.9 M. The budgets are 3,623,878,656 B at M_today and 5,798,205,849 B at 6.0 GiB.
- **E+R** means E_mov,max + R, in-build, in bytes.

## 0. Findings in brief

1. **The method reproduces the registered build exactly at c = 1.**
   - I ran I65's TEXT and cap-priced chain (u4_g7_06's tools, as I72's U8 Pass B ran them) on main `d8c88774d0`.
   - Every output equals G7 Pass A's: `profile_tree.json` is byte-identical, and all 2,814 TEXT rows match in all four runs.
   - I evaluated the tree with the registered build's own in-build atom values (I72's law record), using the arithmetic of `profile::maximum` and `admission_bound`. All 14 phase rows equal the law record: 3,575,778,286 sparse and 3,595,488,734 dense.
   - The multi-case chain, at c = 1, gives a byte-identical tree (§1).
2. **At D1's model caps, no c ≥ 2 fits, even at 6.0 GiB.** W3 binds in both modes.

   | c | dense E+R (B) | smallest M |
   |---|---|---|
   | 2 | 6,592,539,324 | 7,325,043,694 (6.82 GiB) |
   | 3 | 9,747,725,678 | 10.09 GiB |
   | 4 | 13,060,814,532 | 13.52 GiB |
   | 8 | 32,955,764,179 | 34.10 GiB |

   Keeping D1's model caps for c ≥ 2 is therefore owner-held (decision 17). I do not propose it.
3. **Growth is roughly linear per case, with a small c² term.**
   - At D1's caps, E+R ≈ 0.757 + 2.760·c + 0.079·c² GB. The fit through c = 1–3 misses c = 4 by 0.24 MB.
   - About 0.76 GB is per invocation. The rest is per requested or attempted case.
   - The c² terms are whole-envelope scans inside per-case work (§3.3).
   - **m is the strongest model lever.** Halving m alone saves 0.91 GB at c = 1; halving n saves 0.31 GB and halving g 0.35 GB.
   - **l and L matter more as c grows.**
4. **Recommended (P1): two tiers, with M = 5.25 GiB (5,637,144,576 B).**
   - **Tier 1 (c = 1)** is D1, unchanged.
   - **Tier 2 (2 ≤ c ≤ 3):** n, m, g ≤ 16; Σr ≤ 96; l ≤ 64; L ≤ 192.

   | Tier | Mode | E+R (B) | Fraction of M | Under 0.9 M by (B) | Text-error budget |
   |---|---|---|---|---|---|
   | 2 | dense | 4,906,282,942 | 0.8703 | 167,147,176 | 8.78 % |
   | 2 | sparse | 4,894,190,830 | 0.8682 | 179,239,288 | 9.42 % |
   | 1 | both | — | 0.638 / 0.634 | — | — |

   - The smallest M for both tiers and both modes is **5,451,425,492 B (5.08 GiB)**; tier 2 dense binds.
   - W-C2 (three cases on U8's two-body model: n 4, m 2, g 5) fits tier 2.
   - §4 gives the alternatives:
     - **P0:** tier 2 at today's M, with n, m, g ≤ 8;
     - **P2:** C = 4, with 12-node models;
     - a single-tier variant;
     - a fallback ladder.
5. **What binds.**
   - W3 (publication with the staged copy) in both modes, in both tiers, at every multi-case point priced.
   - Its two largest terms are TAV_W and T16's P2 stage (the body `json!` plus hash(publication)).
   - W4 (the precommit reader) is 103 MB below W3 at P1.
6. **Measurement must confirm this before selection** (§6).
   - B1's G5 TEXT and profile on the real n-case code, which replaces this study's emulated multiplicities.
   - G6's maximum and `admission_bound` tests at the new threshold.
   - The S1 multi-case stack witness at R/16: W-C2, plus a tier-2 cap-maximal three-case input.
   - The challenge peak.
   - Per-tier G-B and G-C bounds.
7. **The text-error budget falls with c at a fixed M.**
   - Only 2.8 % of TAV_W's c = 1 bytes are purely per invocation; the rest grows with c. So a per-case text revision δ moves W3 by about c·δ.
   - P1's 8.78 % at 5.25 GiB is 4.9 times today's 1.81 %.
   - Keeping today's 1.81 % at P1 needs M ≥ 5,489,694,223 B; keeping 5 % needs M ≥ 5,557,140,220 B. 5.25 GiB meets both (§7).

## 1. Basis, method and reproduction

**The code basis** is main `d8c88774d0a73bc99fe9c6e906db6296162f8953`. NUM `e302ee8c64` equals it outside `P/execution`, apart from T6S's desktop, schema and test files.
- The 15 TEXT-chain crate source trees, `fixtures/results`, the three static schemas, PP's `Cargo.toml` and `build.rs` are tree-identical to U8's `bd6b4be2c3`, I72's Pass B basis (`_run_records/b1_tree_check.out.txt`).
- `retained_memory.rs` last changed in `35d8ae59a7`, a comment-only change. Its generated profile block equals G7's regeneration (u7_repair_01's forms gate; I72's `gate_forms`).

**The TEXT chain at c = 1** (`_run_records/b1_text_base.sh`):
- I copied u4_g7_06's `chain/`.
- I carried its line-keyed rules from Pass A `ba1faa1c85` to `bd6b4be2c3` with I65's `g7_linemap.py`: 0 remapped, 0 unmapped.
  - Mapping directly to `d8c88774d0` is refused: main's S-I1 edits to `core/rules/*/src/lib.rs` make the short `lib.rs:N` keys ambiguous.
  - The mapped rules apply to `d8c88774d0` because the trees are identical.
- I regenerated the template inventory and ran `run_text_part2.sh` (call graph, lexicon, `sens.py` at l ≤ 128, ε = 2).
- **The result** (`_run_records/base_reproduction.txt`):
  - 8 of 8 non-TEXT outputs are byte-identical to Pass A, including `profile_tree.json`.
  - The four TEXT runs differ only in line numbers: 0 changed rows of 2,814 each.
  - TAV is 2,150,800,830, TAV_W 1,570,041,862, D 14,734 and D_env 9,361.

**The in-build evaluator** (`_run_records/b1_eval.py`):
- **Its arithmetic is the generated `profile` module's:**
  - each form is a checked u64 linear form over atoms;
  - T12_T15, T16, T17 and T25 are the tree's sums and maxima;
  - each phase is requested + max(moving);
  - E_mov,max is the maximum over the seven phases, plus R = 64 MiB.
- **The atom values** are the registered dev/test build's own, as its law test printed them: 244 `I65_G5_ATOM` lines in `R/I72/u8_passb_01/_run_records/runs/u8/law_record.txt`.
- **On the reproduced tree,** it gives all 14 `I65_G5_PHASE` rows of that record exactly (requested, moving and E+R).
- Every multi-case tree uses the same 244 atoms; no point has a missing atom.
- **So a scratch Rust driver would only re-check the transcription.** B1's G5 does that anyway. I ran no cargo. The T3 lock was in use by ROOT's DEC-025, RV104 and I83 during this study.

**The multi-case chain** (`_run_records/b1_mc_chain.py`; the full patch is in `_run_records/chain_diff/rr_to_mc_chain.diff`):
- A patched copy of the line-mapped chain: 41 text patches, each required to match exactly once; 22 loop-rule rebinds; 22 added edge multiplicities.
- **At c = 1, it reproduces the base `profile_tree.json` byte for byte** (`base_reproduction.txt`, last line).
- I ran each cap point with `b1_mc_run.sh`, swept them with `b1_sweep.py`, and tabulated them with `b1_report.py` (`report.json`).
- 62 points were priced. All have complete TEXT, a converged D fixpoint, and no missing atom.

## 2. The pricing model in (c, a, l, L)

### 2.1 The variables (census and TEXT)

**`g3lib.g4_caps()` gains these keys:**
- `c` (requested cases; default 1);
- `a` (attempted; default c, the worst case under T-4);
- `L`, clipped to c·l;
- the derived `cases` = c, `att` = a and `Pall` = c·P.

Every script reads them through `G4_CAPS`, exactly as it reads n, m, g, s, r and l. `text_budget.py` takes `cases`, `att`, `L` and `Pall` into its count symbols.

**The TEXT rules** (`I82_PATCHES.json`, `loop_rebinds`):

| Rule (I65's reason) | Was | Now | Why |
|---|---|---|---|
| T17 reader: "a receipt body array other than builds" (12 rules) | 1 | `cases` | `cases[]` and `ordinary_attempts[]` hold c; `sources[]` and `product_attempts[]` hold a ≤ c; calls, groups and material bases hold 1 (bounded by c) |
| T17 reader: run and source refs of the one call; source refs of the one group (3) | 1 | `att` | One `CaseBatchCall` and one group over the a prepared sources (T-8, C2 §4) |
| T17 reader: numeric cases, one case, one load case (3) | 1 | `cases` | One entry per requested case |
| `sources.iter().enumerate()` (one case source) | 1 | `att` | One prepared CaseSource per attempted case (T-7) |
| `flat_map(\|case\| case.primitive_loads` | l | `L` | It iterates every case's loads |
| `for id in ids` (entity id lists) | l | max(l, L) | The load-id list may span every case |
| `for row in rows_list` (`validate_preview_physics_evidence`) | P | `Pall` | Every envelope row |
| `for r in &envelope.results`; `for row in &envelope.results` (2 new rules, placed first) | P | `Pall` | Every envelope row |
| Edge `validate_profile` → `problem` (`edge_per_call`) | l | L | One `problem()` per load over every case |

`cases` already governed I65's ordinary case loops (`load_cases`, `solves.iter`, `(case_index`), the `selections` loop and the `fn_cap` of `solve_load_case_observed`. They now take c.

**W1's per-case stages are emulated.** Today's code has one case, so B1's loop over A does not exist yet.
- **`retained_w1` →:**
  - `ReservedNotice::reserve`: `att`;
  - `prepare_case`: `att`;
  - `freeze_candidate`: `att`;
  - `publish`: `6*att` (I65's six exclusive fallback sites, kept).
- **`serialize_selected_from` →:**
  - `att` for selection, `product_attempt`, `one_run`, `kernel_outcome`, `run_value`, `bind_preparation`, `case_source`, the six `typed_trace` callees, `bind_rows`, and the source-identity `domain_hash`;
  - `cases` for `one_case`, `ordinary_value` and `legacy_source`.
- **`finish`'s** publication and receipt hashes stay per invocation.
- **`solve_native` stays one call.** Its per-case Runs scale through the kernel's own case loops. The FK `finish_selected` multiplicity doubles at c = 2.

**The D fixpoint** starts at 25,544·c and may take up to 12 iterations. It converged at every point.

### 2.2 The scope of every row and term

**The classification** comes from the chain's own outputs at c = 1, 2, 3 and 4 (D1 caps, a = c, L = c·l). It is in `_run_records/classify_final.json` (`b1_classify.py`):
- **invocation:** an unchanged coefficient;
- **case:** proportional to c;
- **affine:** an invocation part plus a per-case part;
- **growth:** a container law's step (PushCap, buckets, tree nodes) over a count that grows with c, or a product of two c-dependent counts. Always nondecreasing.

**Rows:** 111 named rows (rows with the same name in both modes counted once); 33 invocation, 59 per case, 3 affine, 16 growth. Each row's class and its c = 1–4 values are in `classify_final.json`.

| Family | Scope | Reason |
|---|---|---|
| T_resident (typed request) | affine | Model-level owners once; c `PreviewLoadCase`s and L `PrimitiveLoadInput`s with their strings |
| R_raw; ErrorPrefix; material copies; O-N (both); BuiltModel, helper maps, Boundary; sparse K resident; pattern construction | invocation | One request, one model, one default basis (D1.5): one stiffness, built once (`basis_solve_states[0]`) |
| CasePrefix (4); typed core (each mode); W2; legacy observation; dense parity tail; member recovery; formation-guard late maps; scalar maximum; source-row qualification; old-case staging; preview rewrite; preview tree copy; T07 | per requested case | `solve_load_case_observed` runs once per case. I65's method sums every owner as live, so c copies |
| Final aggregation (results, `rows_by_base_id`, `id_map`) | growth | One map over c·P rows (HashReq at c·P) |
| Retained diagnostics (`V(Diagnostic, D)`) | growth | D from the TEXT fixpoint |
| T11.1, T11.7 | invocation | Early model observation; the invocation digest |
| T11.2–T11.6 (solver observations, identity maps, OrdinarySeed, late old-source capture, operational records) | per requested case | The one ProductCapture holds them per requested case (T-2) |
| T12.1–T12.10, T13.1–T13.2, T14.1–T14.3, T15 | per attempted case | T-7 to T-9 keep every attempted case's preparation, Run and frozen candidate live until staging (T-8, T-9, T-11) |
| T16 publication; T17 reader; T18.1 staged copy | growth | Their Values are the c-case envelope (c·P rows, D_env(c), c preview trees) and the receipt body (`cases[]` and `ordinary_attempts[]` × c; `sources[]`, runs, selections and `product_attempts[]` × a) |
| T18.2 N1 notice | per attempted case | One reserved notice per case in A (T-5) |
| T17.0 statics; T17.1 invocation Value; T18.3; T19 | invocation | Process statics; one raw request; moves; one thread |
| T25.1a, T25.7 | affine | `requested()`: the typed request with c and L |
| T25.1b–T25.5 | invocation (one case at a time) | Per-case finalization stages run in sequence, and their locals drop |
| T25.9; `T25_carried_case` (stage form) | per case | Every case's carried outputs live at the finalize |
| T25.6, T25.8, T25.10, T25.11 | growth | Publication and body Values over c cases |
| D, D_env, Text(diag_env), Text(diag_total), TAV, TAV moving | growth | TEXT multiplicities (§3.3) |
| Text(row), Text(err), Text(sym), Text(audit_error), Text(formation_detail), Text(recovery_finding) | invocation | Per-object byte bounds, which do not depend on counts |

**Atoms.** The 244 layout atoms (`s(T)`, `Node(K,V)`, `Text(...)`) are per-object strides and do not depend on c. Scope lives in their coefficients. Of the 613 (form, atom) terms:

| Class | Terms | Where |
|---|---|---|
| invocation | 163 | O_base_sparse and O_base_dense 12 each; STATICS, INVOC, T19, the T25_S stages, … |
| per case | 332 | O_base_sparse and O_base_dense 69 each; T12 38; T13 63; T14 20; T15 9; T11 19; T25_carried_case 5; NOTICE 3; … |
| affine | 34 | — |
| growth | 84 | BODY, SUCC, T16_P1–P4, T17_V1–V6, STAGED, TAV_W, TAV_X, NOTICE_moving, T16 and T17 moving |

**The gate facts** (`late_observations`, `complete_observations`, `phase_caps`) are restated per tier in §4.3.

### 2.3 The monotonicity lemmas, restated in (c, a, l, L)

I65's lemmas (ORDINARY.md per family; G3 and G4 per row) give each single-case row as nondecreasing in n, m, g, s, r, l, identifier bytes and raw caps. The multi-case extension preserves that.

- **(M1) Per-case rows.** A per-requested-case row is c·R(n, …, l), and a per-attempted-case row is a·R(n, …, l). Each is a product of a nonnegative nondecreasing count with a nondecreasing row. An invocation with uneven loads is bounded because every l_i ≤ l and the per-case owners are summed: Σ_i R(l_i) ≤ c·R(l).
- **(M2) Envelope rows.** Each row over the c-case envelope substitutes c·P for P in a formula already nondecreasing in P: Xc, HashReq (buckets), tree_nodes, PushCap and `VF.tree()`/`text()`/`parsed()`/`hash_route` are all nondecreasing in their counts.
- **(M3) Total loads.** L enters as min(L, c·l), which is nondecreasing, and only where a loop spans every case's loads. A per-case loop priced at l under a case loop of multiplicity c bounds Σ l_i ≤ c·l.
- **(M4) TEXT.** Every site multiplicity is a product of function multiplicities and loop bounds. Each is built from sums, products and max/min of the nonnegative count symbols, so each is nondecreasing.
  - The D fixpoint iterates a nondecreasing map and is started above its value. It converged at all 62 points.
  - TAV, D and D_env are therefore nondecreasing in (c, a, l, L, n, m, g, s, r).
- **(M5) Maxima.** Stage, phase and moving maxima are maxima of nondecreasing functions. So E_mov,max(mode) is nondecreasing in every cap. Its value at a tier's cap vector, with a = c and L at its cap, bounds every invocation admitted to that tier.
  - With two tiers, the bound is the maximum over the tiers.
  - Every sweep sequence is increasing in c, in k = n = m = g, in l and in L (`report.json`).
- **(M6) The c = 1 identity.** With c = a = 1 and L = l, every substitution is the identity. The tree is byte-identical to the registered one (§1).

### 2.4 What this model is not

- **It prices B1's design, not B1's code.**
  - The 22 rule rebinds and 22 edge multiplicities stand in for loops that B1 will write: over A in `retained_w1`, and over cases in the serializer and reader.
  - B1's real loops can differ, for example a per-case scan of the whole envelope that this emulation did not see.
  - B1's G5 must re-derive TEXT on the real code (§6).
- **I65's conservatism is kept.**
  - Per-case ordinary transients are summed as if all were live; they are sequential in fact.
  - T13's group-shared S_p and VS_p are counted per attempted case, although C2 §4 shares them under one group.
  - `ordinary_attempts[].diagnostic_refs` is priced at D_env per case (c·D_env).
  - I took no liveness or sharing credit. Each would need its own review, like G3's held phase-aware span.
- **a = c is the worst case.** T-4's `not_required` and excluded cases reduce A in practice, but the bound assumes every case is Sensitive.

## 3. The maxima

### 3.1 D1's model caps (n = m = g = 32, Σr = 192, l = 128, L = c·l), a = c

| c | dense E+R (B) | sparse E+R (B) | fraction of today's M (dense) | fraction of 6.0 GiB (dense) | smallest M for 0.9 M (both modes) | fits 0.9 M today / at 6.0 GiB |
|---|---|---|---|---|---|---|
| 1 | 3,595,488,734 | 3,575,778,286 | 0.8929 M | 0.5581 M | 3,994,987,483 (3.72 GiB) | yes / yes |
| 2 | 6,592,539,324 | 6,553,118,428 | 1.6373 M | 1.0233 M | 7,325,043,694 (6.82 GiB) | no / no |
| 3 | 9,747,725,678 | 9,688,594,334 | 2.4209 M | 1.5130 M | 10,830,806,309 (10.09 GiB) | no / no |
| 4 | 13,060,814,532 | 12,981,972,740 | 3.2437 M | 2.0273 M | 14,512,016,147 (13.52 GiB) | no / no |
| 8 | 32,955,764,179 | 32,798,080,595 | 8.1847 M | 5.1154 M | 36,617,515,755 (34.10 GiB) | no / no |

The binding phase is W3 in both modes for every c. D grows from 14,734 (c = 1) to 27,547, 41,412, 56,329 and 126,517.

### 3.2 Model-cap trades

**Key:** n = m = g = s = k, Σr = 6k unless shown; a = c.

- The first column is the point's tag in `report.json`. Its `s_`, `t_`, `u_` and `v_` prefixes record only which sweep produced the point.
- "B to spare" is the smaller of the dense and sparse margins to 0.9 M.

| point | c | n = m = g = s | r | l | L | dense E+R | sparse E+R | under 0.9 x today's M (B to spare) | under 0.9 x 6.0 GiB (B to spare) | smallest M, both modes |
|---|---|---|---|---|---|---|---|---|---|---|
| s_c1_l64 | 1 | 32 | 192 | 64 | 64 | 3,359,696,222 | 3,339,985,774 | yes, 264,182,434 | yes, 2,438,509,627 | 3,732,995,803 |
| s_c1_l32 | 1 | 32 | 192 | 32 | 32 | 3,243,645,726 | 3,223,935,278 | yes, 380,232,930 | yes, 2,554,560,123 | 3,604,050,807 |
| s_c1_g16 | 1 | n 32, m 32, g 16 | 96 | 128 | 128 | 3,242,388,942 | 3,222,651,230 | yes, 381,489,714 | yes, 2,555,816,907 | 3,602,654,380 |
| s_c1_m16 | 1 | n 32, m 16, g 32 | 192 | 128 | 128 | 2,681,982,094 | 2,660,477,598 | yes, 941,896,562 | yes, 3,116,223,755 | 2,979,980,105 |
| s_c1_n16 | 1 | n 16, m 32, g 32 | 192 | 128 | 128 | 3,281,278,336 | 3,279,055,888 | yes, 342,600,320 | yes, 2,516,927,513 | 3,645,864,818 |
| s_c1_k16 | 1 | 16 | 96 | 128 | 128 | 2,090,308,096 | 2,086,277,392 | yes, 1,533,570,560 | yes, 3,707,897,753 | 2,322,564,552 |
| s_c1_k16_l64 | 1 | 16 | 96 | 64 | 64 | 1,857,200,398 | 1,853,169,694 | yes, 1,766,678,258 | yes, 3,941,005,451 | 2,063,555,998 |
| s_c1_k8 | 1 | 8 | 48 | 128 | 128 | 1,420,915,701 | 1,420,356,501 | yes, 2,202,962,955 | yes, 4,377,290,148 | 1,578,795,224 |
| t_c2_m16 | 2 | n 32, m 16, g 32 | 192 | 128 | 256 | 4,862,573,756 | 4,819,564,764 | no, 1,238,695,100 over | yes, 935,632,093 | 5,402,859,729 |
| t_c2_k28_l128 | 2 | 28 | 168 | 128 | 256 | 5,862,629,856 | 5,833,214,080 | no, 2,238,751,200 over | no, 64,424,007 over | 6,514,033,174 |
| t_c2_k24_l128 | 2 | 24 | 144 | 128 | 256 | 5,176,039,376 | 5,155,270,896 | no, 1,552,160,720 over | yes, 622,166,473 | 5,751,154,863 |
| t_c2_k24_l64 | 2 | 24 | 144 | 64 | 128 | 4,664,829,932 | 4,644,061,452 | no, 1,040,951,276 over | yes, 1,133,375,917 | 5,183,144,369 |
| t_c2_k20_l128 | 2 | 20 | 120 | 128 | 256 | 4,494,426,352 | 4,480,728,208 | no, 870,547,696 over | yes, 1,303,779,497 | 4,993,807,058 |
| t_c2_k20_l64 | 2 | 20 | 120 | 64 | 128 | 3,984,083,996 | 3,970,385,852 | no, 360,205,340 over | yes, 1,814,121,853 | 4,426,759,996 |
| t_c2_k16_l128 | 2 | 16 | 96 | 128 | 256 | 3,853,150,528 | 3,845,089,120 | no, 229,271,872 over | yes, 1,945,055,321 | 4,281,278,365 |
| v_c2_k16_l64 | 2 | 16 | 96 | 64 | 128 | 3,341,387,804 | 3,333,326,396 | yes, 282,490,852 | yes, 2,456,818,045 | 3,712,653,116 |
| v_c2_k12_l128 | 2 | 12 | 72 | 128 | 256 | 3,233,537,072 | 3,229,705,424 | yes, 390,341,584 | yes, 2,564,668,777 | 3,592,818,969 |
| t_c3_m12 | 3 | n 32, m 12, g 32 | 192 | 128 | 384 | 6,544,412,390 | 6,478,590,230 | no, 2,920,533,734 over | no, 746,206,541 over | 7,271,569,323 |
| t_c3_k24_l128 | 3 | 24 | 144 | 128 | 384 | 7,681,914,644 | 7,650,761,924 | no, 4,058,035,988 over | no, 1,883,708,795 over | 8,535,460,716 |
| t_c3_k20_l128 | 3 | 20 | 120 | 128 | 384 | 6,686,839,504 | 6,666,292,288 | no, 3,062,960,848 over | no, 888,633,655 over | 7,429,821,672 |
| t_c3_k20_l64 | 3 | 20 | 120 | 64 | 192 | 5,840,730,546 | 5,820,183,330 | no, 2,216,851,890 over | no, 42,524,697 over | 6,489,700,607 |
| t_c3_k16_l128 | 3 | 16 | 96 | 128 | 384 | 5,750,912,852 | 5,738,820,740 | no, 2,127,034,196 over | yes, 47,292,997 | 6,389,903,169 |
| **t_c3_k16_l64 (P1 tier 2)** | 3 | 16 | 96 | 64 | 192 | 4,906,282,942 | 4,894,190,830 | no, 1,282,404,286 over | yes, 891,922,907 | 5,451,425,492 |
| u_c3_k16_l64_L96 | 3 | 16 | 96 | 64 | 96 | 4,661,661,790 | 4,649,569,678 | no, 1,037,783,134 over | yes, 1,136,544,059 | 5,179,624,212 |
| u_c3_k16_l32 | 3 | 16 | 96 | 32 | 96 | 4,493,377,758 | 4,481,285,646 | no, 869,499,102 over | yes, 1,304,828,091 | 4,992,641,954 |
| t_c3_k12_l128 | 3 | 12 | 72 | 128 | 384 | 4,842,209,952 | 4,836,462,480 | no, 1,218,331,296 over | yes, 955,995,897 | 5,380,233,280 |
| u_c3_k12_l64 | 3 | 12 | 72 | 64 | 192 | 4,004,434,856 | 3,998,687,384 | no, 380,556,200 over | yes, 1,793,770,993 | 4,449,372,063 |
| v_c3_k12_l32 | 3 | 12 | 72 | 32 | 96 | 3,589,603,762 | 3,583,856,290 | yes, 34,274,894 | yes, 2,208,602,087 | 3,988,448,625 |
| v_c3_k10_l32 | 3 | 10 | 60 | 32 | 96 | 3,141,728,236 | 3,138,280,252 | yes, 482,150,420 | yes, 2,656,477,613 | 3,490,809,152 |
| **v_c3_k8_l64 (P0 tier 2)** | 3 | 8 | 48 | 64 | 192 | 3,128,451,412 | 3,126,773,812 | yes, 495,427,244 | yes, 2,669,754,437 | 3,476,057,125 |
| v_c3_k8_l32 | 3 | 8 | 48 | 32 | 96 | 2,713,918,062 | 2,712,240,462 | yes, 909,960,594 | yes, 3,084,287,787 | 3,015,464,514 |
| t_c4_m8 | 4 | n 32, m 8, g 32 | 192 | 128 | 512 | 8,183,526,367 | 8,093,920,287 | no, 4,559,647,711 over | no, 2,385,320,518 over | 9,092,807,075 |
| t_c4_k20_l128 | 4 | 20 | 120 | 128 | 512 | 9,027,966,167 | 9,000,569,879 | no, 5,404,087,511 over | no, 3,229,760,318 over | 10,031,073,519 |
| t_c4_k16_l128 | 4 | 16 | 96 | 128 | 512 | 7,917,586,207 | 7,901,463,391 | no, 4,293,707,551 over | no, 2,119,380,358 over | 8,797,318,008 |
| t_c4_k16_l64 | 4 | 16 | 96 | 64 | 256 | 6,550,615,876 | 6,534,493,060 | no, 2,926,737,220 over | no, 752,410,027 over | 7,278,462,085 |
| t_c4_k16_l32 | 4 | 16 | 96 | 32 | 128 | 5,946,306,596 | 5,930,183,780 | no, 2,322,427,940 over | no, 148,100,747 over | 6,607,007,329 |
| t_c4_k12_l128 | 4 | 12 | 72 | 128 | 512 | 6,848,220,247 | 6,840,556,951 | no, 3,224,341,591 over | no, 1,050,014,398 over | 7,609,133,608 |
| t_c4_k12_l64 | 4 | 12 | 72 | 64 | 256 | 5,358,966,100 | 5,351,302,804 | no, 1,735,087,444 over | yes, 439,239,749 | 5,954,406,778 |
| **u_c4_k12_l64_L128 (P2 tier 2)** | 4 | 12 | 72 | 64 | 128 | 4,985,983,572 | 4,978,320,276 | no, 1,362,104,916 over | yes, 812,222,277 | 5,539,981,747 |
| u_c4_k12_l32 | 4 | 12 | 72 | 32 | 128 | 4,755,391,340 | 4,747,728,044 | no, 1,131,512,684 over | yes, 1,042,814,509 | 5,283,768,156 |
| u_c4_k10_l64 | 4 | 10 | 60 | 64 | 256 | 4,790,181,051 | 4,785,583,739 | no, 1,166,302,395 over | yes, 1,008,024,798 | 5,322,423,390 |
| t_c4_k8_l128 | 4 | 8 | 48 | 128 | 512 | 5,812,609,695 | 5,810,372,895 | no, 2,188,731,039 over | no, 14,403,846 over | 6,458,455,217 |
| u_c4_k8_l64 | 4 | 8 | 48 | 64 | 256 | 4,287,586,527 | 4,285,349,727 | no, 663,707,871 over | yes, 1,510,619,322 | 4,763,985,030 |
| u_c4_k8_l32 | 4 | 8 | 48 | 32 | 128 | 3,598,684,100 | 3,596,447,300 | yes, 25,194,556 | yes, 2,199,521,749 | 3,998,537,889 |
| t_c8_k12_l64 | 8 | 12 | 72 | 64 | 512 | 14,155,131,067 | 14,139,804,475 | no, 10,531,252,411 over | no, 8,356,925,218 over | 15,727,923,408 |
| t_c8_k8_l128 | 8 | 8 | 48 | 128 | 1024 | 17,613,316,595 | 17,608,842,995 | no, 13,989,437,939 over | no, 11,815,110,746 over | 19,570,351,773 |
| t_c8_k8_l64 | 8 | 8 | 48 | 64 | 512 | 11,825,834,867 | 11,821,361,267 | no, 8,201,956,211 over | no, 6,027,629,018 over | 13,139,816,519 |
| u_c8_k8_l64_L256 | 8 | 8 | 48 | 64 | 256 | 9,712,927,251 | 9,708,453,651 | no, 6,089,048,595 over | no, 3,914,721,402 over | 10,792,141,390 |
| t_c8_k8_l32 | 8 | 8 | 48 | 32 | 256 | 8,950,984,787 | 8,946,511,187 | no, 5,327,106,131 over | no, 3,152,778,938 over | 9,945,538,653 |
| u_c8_k8_l16 | 8 | 8 | 48 | 16 | 128 | 7,517,416,171 | 7,512,942,571 | no, 3,893,537,515 over | no, 1,719,210,322 over | 8,352,684,635 |
| t_c8_k6_l64 | 8 | 6 | 36 | 64 | 512 | 10,702,956,903 | 10,701,847,911 | no, 7,079,078,247 over | no, 4,904,751,054 over | 11,892,174,337 |
| t_c8_k4_l64 | 8 | 4 | 24 | 64 | 512 | 9,589,198,419 | 9,589,854,803 | no, 5,965,976,147 over | no, 3,791,648,954 over | 10,655,394,226 |
| u_c8_k4_l16 | 8 | 4 | 24 | 16 | 128 | 5,292,713,803 | 5,293,370,187 | no, 1,669,491,531 over | yes, 504,835,662 | 5,881,522,430 |
| u_c8_k4_l8 | 8 | 4 | 24 | 8 | 64 | 4,582,395,451 | 4,583,051,835 | no, 959,173,179 over | yes, 1,215,154,014 | 5,092,279,817 |
| u_c8_k2_l16 | 8 | 2 | 12 | 16 | 128 | 4,195,941,575 | 4,196,193,095 | no, 572,314,439 over | yes, 1,602,012,754 | 4,662,436,773 |

**Notes on the table.**
- At c = 8 with n, m, g ≤ 4, the sparse mode binds: the dense-only owners are small there.
- W3 binds at every multi-case point.
- At c = 1 with k = 8, W4 binds instead.

### 3.3 The shape of the growth

**At D1's caps,** the dense first differences in c are 2,997,050,590, 3,155,186,354 and 3,313,088,854 B. The second differences are 158,135,764 and 157,902,500 B. The quadratic through c = 1–3 is 756,573,908 + 2,759,846,944·c + 79,067,882·c² B; it misses c = 4 by 233,264 B. At c = 8 the actual 32.96 GB is above that quadratic (27.9 GB), because the D fixpoint and the container steps grow faster there.

**TEXT (TAV_W) at c = 1, by how each function's text grew from c = 1 to c = 2:**

| Growth | Bytes at c = 1 | Share of TAV_W | What |
|---|---|---|---|
| Unchanged (per invocation) | 43,600,008 | 2.8 % | Supports, node normalization |
| Exactly doubled | 601,820,776 | — | — |
| Between 1× and 2× | 892,800,650 | — | Functions with both kinds of callers |
| More than doubled | 31,820,428 | — | See the c² sites below |

**The c² sites:** in the full TEXT run, the positive second differences (c = 2, 3, 4) sum to 117.6 MB.
- `integrity_diagnostic_id` (lib.rs, multiplicity 58,946 → 901,304 at c = 4): a diagnostic-indexed loop inside per-case work.
- `observation_fields` via `bind_observations`'s `for row in &envelope.results` (per case × every envelope row).
- `validate_profile` → `problem()` over L, reached once per invocation plus twice per case.
- `normalize_quantity` and `unit_conversion_diag`.
- Outside TEXT: `ordinary_attempts[].diagnostic_refs` (c × D_env(c)) and the T16/T17 hashes of the growing envelope and body.

**Model levers at c = 1** (dense; −Δ against 3,595,488,734):

| Change | Saving |
|---|---|
| m 32 → 16 | 913,506,640 |
| g 32 → 16 | 353,099,792 |
| n 32 → 16 | 314,210,398 |
| l 128 → 64 | 235,792,512 |
| n = m = g 32 → 16 | 1,505,180,638 |

**The total-load row helps at larger c:**

| Point | L | dense E+R (B) | Saving |
|---|---|---|---|
| c = 3, k 16, l 64 | 192 → 96 | 4,906,282,942 → 4,661,661,790 | 244,621,152 |
| c = 4, k 12, l 64 | 256 → 128 | 5,358,966,100 → 4,985,983,572 | 372,982,528 |
| c = 8, k 8, l 64 | 512 → 256 | 11,825,834,867 → 9,712,927,251 | 2,112,907,616 |

## 4. The proposal (ROOT selects)

### 4.1 Recommended (P1): two tiers, M = 5.25 GiB

**Tier 1 (c = 1)** is D1 exactly as registered: n, m, g ≤ 32, Σr ≤ 192, l ≤ 128, and the rest of D1.9 and D1.11.

**Tier 2 (2 ≤ c ≤ C = 3):** n, m, g ≤ 16; springs ≤ 96 ruled (effective ≤ g = 16, as DOMAIN §2); Σr ≤ 96; l_i ≤ 64 for every case; Σ l_i ≤ L = 192.
- L = C·l, so the row is stated but never binds. Setting L = 96 instead saves 244,621,152 B (`u_c3_k16_l64_L96`).
- Materials (4 + 4), temperature points (16), text (128 B), raw values (16,384) and depth (16), raw text and capacities, digest and control bytes are unchanged.
- Combinations and components are 0.
- D1.3 and D1.5 to D1.8 hold for every case. D1.5 means one default basis, so one stiffness (decision 7).

**The maxima** (in-build atoms, this emulation):

| Tier | dense E+R (B) | sparse E+R (B) | Binding phase |
|---|---|---|---|
| 2 | 4,906,282,942 | 4,894,190,830 | W3 |
| 1 | 3,595,488,734 | 3,575,778,286 | W3 |

**The smallest M ≤ 6.0 GiB** meeting 0.9 M in both modes and both tiers is **5,451,425,492 B** (5.077 GiB). Tier 2 dense binds; tier 2 sparse needs 5,437,989,812 B.

**The proposed M is 5,637,144,576 B = 5.25 GiB** (21 × 256 MiB; today's M is 15 × 256 MiB).

| Tier | Mode | E+R / M | Under 0.9 M by (B) | Text-error budget |
|---|---|---|---|---|
| 2 | dense | 0.8703 M | 167,147,176 | 8.78 % |
| 2 | sparse | 0.8682 M | 179,239,288 | 9.42 % |
| 1 | dense | 0.6378 M | 1,477,941,384 | — |
| 1 | sparse | 0.6343 M | 1,497,651,832 | — |

At 6.0 GiB, tier 2 would leave 891,922,907 B (dense) and 904,015,019 B (sparse).

**Why this point:**
- C = 3 is the least C that carries W-C2 (A, B, C on U8's two-body model: n 4, m 2, g 5, two materials). It fits tier 2.
- Tier 2 keeps half of D1's model size, against a quarter for P0.
- The proposed M is 0.75 GiB under the owner's 6.0 GiB. That headroom absorbs B1's real-code TEXT deviations, up to 46.9 % of TAV_W, without asking the owner. It also leaves room for B2 and B3's re-pricing.
- **Tier 1 is unchanged,** so:
  - every committed c = 1 witness keeps its meaning (W1–W7, W2-deep, headroom, U8's), and the cap-maximal W2 and W2b stay in the domain;
  - D1's registered maximum is unchanged.

**What two tiers cost B1:**
- the predicate selects the tier from c (c = 0 refuses at D1.4, as today; c > C refuses);
- `cap_rows` takes the tier's caps;
- the generated profile carries one form set per tier, and `cap_priced_maximum(mode)` returns the maximum over the tiers. `admission_bound` is unchanged;
- `phase_caps` takes the tier (§4.3);
- the S1 witnesses run at both tiers' cap-maximal inputs.

### 4.2 Alternatives

| Option | Tier 2 (tier 1 = D1 unless stated) | M | Tier 2 dense under 0.9 M by | Note |
|---|---|---|---|---|
| **P0: no M change** | c ≤ 3; n, m, g ≤ 8; Σr ≤ 48; l ≤ 64; L ≤ 192 | Today's 4,026,531,840 | 495,427,244 B (0.7770 M; 46.5 % of TAV_W 1,065,205,010) | The registered threshold is unchanged. W-C2 fits (g 5 ≤ 8). Smallest M 3,994,987,483 (tier 1 binds). Also at today's M: c ≤ 3 with 10/10/10 and l ≤ 32 (482,150,420 B); c ≤ 2 with 16/16/16 and l ≤ 64 (282,490,852 B); c ≤ 4 with 8/8/8 and l ≤ 32 (25,194,556 B, thin) |
| **P2: C = 4** | c ≤ 4; 12/12/12; Σr ≤ 72; l ≤ 64; L ≤ 128 | 5.25 GiB | 87,446,546 B (0.8845 M; 4.66 %) | Smallest M 5,539,981,747. At 6.0 GiB: 812,222,277 B. With L = 256 it needs 5,954,406,778 (5.75 GiB) |
| **P3: single tier** | P1's tier-2 caps for every c, c ≤ 3 | As P1 (5.25 GiB) | As P1 | Simpler predicate and profile, but it narrows c = 1 from 32/32/32 and l 128 to 16/16/16 and l 64. QUAL §4's cap-maximal W2 and W2b leave the domain and must be re-based. D1's accepted domain is narrowed |
| **c up to 8** | c ≤ 8; n, m, g ≤ 4; l ≤ 8 | ≥ 5,092,279,817 | At 5.25 GiB: 490,378,283 B (sparse binds) | 4-node models; not proposed. c = 8 with n, m, g ≤ 4 and l ≤ 16 needs 5,881,522,430 |
| **D1's model caps for c ≥ 2** | — | c = 2: 7,325,043,694 (6.82 GiB); c = 3: 10.09 GiB; c = 4: 13.52 GiB | — | Above 6.0 GiB: **owner** (decision 17). Not proposed |

**A fallback ladder for B1's measurement,** each step without the owner:
1. P1 at 5.25 GiB.
2. P1 at up to 6.0 GiB, if B1's in-build tier 2 exceeds 0.9 × 5.25 GiB but not 0.9 × 6.0 GiB.
3. Tier 2 with 12/12/12, Σr 72, l 64 and L 192: 4,004,434,856 B, under 0.9 × 6.0 GiB by 1,793,770,993 B.
4. P0's tier 2 at today's M.

### 4.3 The restated rows with P1's values (DESIGN_v2 §6's table)

| Row | Scope | Tier 1 (c = 1) | Tier 2 (2 ≤ c ≤ 3) |
|---|---|---|---|
| Load cases c (D1.4; `LoadCasesCapacity`) | Per invocation | 1 | 2–3. The typed `load_cases.capacity` ≤ 3 |
| Loads per case l_i (D1.9 `Loads`, `LoadsCapacity`) | Per case | ≤ 128 | ≤ 64, every case. Today's census reads `load_cases[0]` only |
| Total loads Σ l_i (new row) | Per invocation | ≤ 128 | ≤ 192 |
| n, m, g; their capacities | Per invocation | ≤ 32 | ≤ 16 |
| Σr; restraint capacities | Per invocation | ≤ 192 | ≤ 96 |
| Springs | Per invocation | ≤ 192 (≤ g effective) | ≤ 96 (≤ g effective) |
| Materials, temperature points, sections 0, components 0, combinations 0, expansion laws, text, raw, units, digest, control bytes | Per invocation | D1.9, D1.11 | Unchanged |

**G-B** (`late_observations`, read at each case's late capture):

| Fact | Bound |
|---|---|
| BuiltNodes | ≤ n |
| BuiltMembers | ≤ m |
| BuiltFrameElements | ≤ m |
| BuiltSupports | ≤ g |
| CaseLoads | ≤ l (per case), plus a running total ≤ L (new) |
| Restrained | ≤ min(6n, Σr) |
| Springs | ≤ the springs cap |
| Materials | ≤ 8 |
| LateObservationBytes | ≤ T11 − T11_late_capture, both at the tier's forms. These are per case × c (§2.2) |

**G-C** (`complete_observations`, once):

| Fact | Bound |
|---|---|
| EnvelopeResults | ≤ c·P_final |
| EnvelopeResultCapacity | ≤ PushCap(c·P_final) |
| EnvelopeResultTextBytes | ≤ 2·c·P_final·Text(row) |
| EnvelopeDiagnostics | ≤ D_env (the tier's) |
| Diagnostic capacity | ≤ PushCap(D_env) |
| EnvelopeDiagnosticTextBytes | ≤ 2·Text(diag_env) (the tier's) |
| EnvelopeMaxStringBytes | ≤ L_PUB |
| DiagnosticIdMaxBytes | ≤ L_DIAGID |
| Contract-evidence arrays, objects, entries, strings, keys | ≤ c × today's per-case preview facts |
| SourceBlockRecovery | 0 |
| ObservationBytes | ≤ T11 (the tier's) |
| OrdinarySeedBytes | ≤ T11_ordinary_seed (the tier's, c seeds) |
| RetainedErrorTextBytes | ≤ c·(3m + 1)·Text(err). An assumption: one retained error set per case; B1 checks the producer |
| OrdinarySolveNotAttempted | 0 over every requested case (T-3 (e)) |

**The tier's text atoms** (D_env, Text(diag_env), Text(row), L_PUB, L_DIAGID) come from that tier's TEXT run. At P1's tier 2, D_env is 11,935 and D 23,044.

**U3's budgets (`PHASE_BUDGETS`):**
- B-2 STAGED, B-3 SUCC and B-5 T17 take the tier's forms.
- B-6 is NOTICE_RESERVE_BYTES × |A|.
- B-1 stays one ordinary run (T-13).
- B-4, B-8 and B-9 are unchanged.

## 5. The rows that bind

**At P1's tier 2, dense** (sparse differs only in O_base_sparse, 464,928,308), every phase's E+R:

| Phase | dense E+R (B) |
|---|---|
| W1 | 2,461,027,214 |
| W2 | 2,690,101,376 |
| **W3** | **4,906,282,942** |
| W4 | 4,803,137,717 |
| W5 | 2,983,802,536 |
| X1 | 4,673,399,656 |
| X2 | 2,714,607,094 |

**W3 (publication with the staged copy) binds.** Its terms:

| Term | Bytes | Share of W3 |
|---|---|---|
| **TAV_W** | 1,902,865,106 | 38.8 % |
| **T16 = P2** (the body `json!` plus hash(publication)) | 1,836,344,053 | 37.4 % |
| O_base_dense | 477,020,420 | — |
| T16_moving (the publication text's last growth) | 258,134,819 | — |
| T12_T15 | 229,071,504 | — |
| STAGED | 130,091,302 | — |
| HELPER_moving | 8,388,608 | — |
| STATICS | 5,397,696 | — |
| TXT_moving | 1,360,986 | — |
| T11, T19, NOTICE | 249,178 | — |

- T16's other stages are P3 hash(body) 1,445,303,139, P4 576,630,778 and P1 233,721,000.
- **Next behind W3:** W4 (T17 V2_hash 1,553,413,618 plus SUCC, INVOC and the rest) is 103,145,225 B behind, and X1 233 MB behind.

**At c = 8 with 4-node models,** T16's binding stage moves from P2 to P3 (hash(body)), because the body's `ordinary_attempts[]` and `cases[]` grow as c·D_env.

**The cap rows that bind the domain** are c ≤ C and tier 2's m, g, n, l and L, in decreasing price effect (§3.3). Raw caps never bind in P1: a 3-case, 64-load request is far below 16,384 raw values.

## 6. What measurement must confirm before selection (D-7)

ROOT selects M by measurement. These run inside B1's re-qualification (PLAN §2.1, "a widening"). I ran none of them.

1. **G5: the profile from B1's real code, per tier.**
   - Re-run TEXT (`g7_pass.sh`'s chain) on B1's candidate, with c, a and L as census and TEXT variables. B1's real loops over A, the serializer's and reader's case loops, and the tier selection replace this study's 22 rebinds and 22 emulated edges.
   - **Must show:**
     - TEXT complete: no unmapped loop, no unclassified argument, no multi-member SCC, a converged D fixpoint;
     - the identifier audit enforced (no `id-unaudited`, `stale-key` or `stale-audit-entry`), and its controls;
     - QUAL §11's carry (RV87's non-candidate sweep, or the explicit-row rule);
     - the regenerated forms gate, with `profile::ESTIMATES` = 0;
     - the law test printing `I65_G5_PROFILE`/`PHASE` per tier and mode.
   - **Tier 1** equals today's record (3,575,778,286 / 3,595,488,734) or differs only by explained D1 call-graph deltas.
   - **Tier 2's in-build E+R** is within the text-error budget of this study's 4,906,282,942 / 4,894,190,830, or each difference is explained. The c² sites of §3.3 are re-derived on the real code.
2. **G6: the maximum and the bound at the selected M.**
   - `admission_bound` tested at M − R − 1, M − R and M − R + 1 with the new `threshold_bytes` (RV89 G6 S-3's pattern).
   - A pure `maximum` test with each phase in turn largest, including W4 and X1, which sit within 5 % of W3 at P1.
   - The registration diff (`threshold_bytes`, the tiered cap tables).
   - The 0.9 M rule in both modes and both tiers.
   - A QUAL-style record stating the text-error budget.
   - The release identity, if registered: its record equal to the dev record, as QUAL §5.
3. **The S1 stack witnesses for the new paths,** at R/k = 4 MiB (k = 16), in both modes. **Must show** no overflow, abort or panic, with every outcome asserted:
   - **W-C2** (A selected, B `not_required`, C Ceiling `unavailable`): a successor;
   - **a tier-2 cap-maximal three-case input,** every case Sensitive so |A| = 3: n = m = g = 16, 64 loads per case, a quote and backslash in every provenance, raw depth 16. Its asserted outcome; Fallback(Preparation) and Fallback(Candidate) are acceptable, as QUAL §4's W2 and W2b;
   - **W-C2 at R/64 = 1 MiB** as headroom;
   - **the re-based W6, W-C1, W2 and W2b** per DESIGN_v2 §1.3;
   - **the deepest call chain** re-derived on B1's call graph. A loop over A adds no recursion, so 40 frames is expected unchanged.
4. **The challenge** (`retained_memory_challenge`). **Must show:**
   - the measured peak of a permitted multi-case run ≤ tier 2's in-build W1 phase (or E_mov,max);
   - the milestone's peak ≤ tier 1's, as today (3,541,898 / 2,252,863 B).
5. **The gates per tier** (§4.3), unit-tested:
   - CaseLoads per case and total;
   - EnvelopeResults ≤ c·P_final;
   - D_env and Text(diag_env) at the tier's TEXT;
   - `ordinary_solve_attempted` over every requested case (T-3 (e)): one blocked case leaves later cases unseeded, and G-C must decline.

**Non-claims stay as D-7:** no RSS, allocator, concurrency, stack-beyond-witness or machine claim. 5.25 GiB is not a supported-machine statement.

## 7. The text-error budget and c

**Today:** 28,389,922 / 1,570,041,862 = **1.81 %** (dense), 3.06 % (sparse) (QUAL §3).

**How c moves it:**
- Only 2.8 % of TAV_W's c = 1 bytes are purely per invocation; the rest grows with c (§3.3).
- A text revision δ inside per-case code therefore moves W3 by about c·δ (a·δ in W1 code), and by more at the c² sites.
- At a fixed M, the margin shrinks by about 3.0–3.3 GB per added case at D1's caps, while TAV_W grows by about 1.29 GB per case. Both drive the budget down.
- **Budget as margin / TAV_W:**

  | Point | M | TAV_W (B) | Budget (dense / sparse) |
  |---|---|---|---|
  | P1 tier 2 | 5.25 GiB | 1,902,865,106 | 8.78 % / 9.42 % |
  | P1 tier 2 | 6.0 GiB | 1,902,865,106 | 46.9 % / 47.5 % |
  | P1 tier 2 | 5,451,425,492 (smallest) | 1,902,865,106 | 0 |
  | P0 tier 2 | Today's M | 1,065,205,010 | 46.5 % |
  | P2 tier 2 | 5.25 GiB | 1,878,123,282 | 4.66 % |

- **The M a target budget β needs at P1:** M ≥ (E+R + β·TAV_W)/0.9.
  - β = 1.81 % (today's): 5,489,694,223 B;
  - β = 5 %: 5,557,140,220 B.
  - 5.25 GiB = 5,637,144,576 meets both.
- **My suggestion: hold tier 2 to at least 5 %** at B1's G6, not today's 1.81 %. This study's per-case multiplicities are emulated (§2.4), and RV89 N-1's risk (text revisions, not strides) now scales with c.

## 8. For ROOT, and for the owner

**For ROOT:**
1. **Select:**
   - two tiers (P1), or a single tier (P3);
   - C: 3 (P1) or 4 (P2);
   - tier 2's n, m, g, Σr, l and L;
   - M: P1 at 5.25 GiB, or P0 at today's M.

   All are within the owner's 6.0 GiB, so none needs the owner. The fallback ladder (§4.2) keeps every B1 outcome without the owner.
2. **I recommend P1** (two tiers; tier 2 at c ≤ 3, 16/16/16, Σr 96, l 64, L 192; M 5.25 GiB, if G5 and G6 confirm), with the L row stated as C·l.
3. **Optional implementation guidance for B1, which lowers the c² terms** and is not needed for P1:
   - bind observations by case index, not by scanning `envelope.results` per case;
   - collect `ordinary_attempts[].diagnostic_refs` in one pass over the diagnostics;
   - make the integrity-report diagnostic loop per case.

   Each is a code choice B1's G5 would then price.
4. **Liveness and sharing credits** (per-case ordinary transients, T13's group-shared S_p and VS_p, G3's phase-aware span) were not taken. Each needs its own review before it could lower a selected M.
5. **The X branch** (exact-selected, T25 per case) stays below W3 at every point (X1 at P1: 4,673,399,656).

**For the owner:** nothing, if ROOT selects within 6.0 GiB.
- Only keeping D1's full model caps (32/32/32, l 128) for c ≥ 2 needs M above 6.0 GiB (decision 17): c = 2 needs ≥ 7,325,043,694 B (6.82 GiB), c = 3 ≥ 10.09 GiB, c = 4 ≥ 13.52 GiB, and c = 8 ≥ 34.10 GiB.
- I do not recommend it for B1.

## 9. Execution record, what I read, and limits

**Execution.**
- I82, TASK, no descendants. 2026-10-06, about 17:50–19:30 MDT.
- Python 3.13 from VENV. The chain scripts call `python3`, which resolved to VENV's through PATH.
- About 120 chain runs, each 15–120 s single-threaded: step 0, the identity checks and sweeps A to D. The consolidated runs are the ones reported: `points_final.json`, 62 points, run on the final chain.
- No cargo, so no target was made. The T3 lock was held by ROOT's DEC-025, then by RV104 and I83, and I neither waited on it nor touched it.
- TMPDIR was `WT/scratch/i82_b1_study/tmp`.
- Writes went to this folder and `WT/scratch/i82_b1_study/` only. The write guard did not refuse the NUM records folder.
- **Cleanup after the record was sealed.** I deleted the archive copy (`snap/`), the per-point work directories (`mc_runs/`, 290 MB), `base/` and `tmp/`.
  - `b1_snapshot.sh`, then `b1_text_base.sh` and `b1_mc_chain.py`, recreate them.
  - About 2.5 MB of small copies remain in `WT/scratch/i82_b1_study/`: the chain copies `rr/` and `mc_chain/`, the mapped `rules/`, and the sweep JSON. They are safe to delete.

**Run records** (`_run_records/`, placeholder paths only):

| File | What |
|---|---|
| `b1_tree_check.sh` (+ `.out.txt`) | The trees are identical (§1) |
| `b1_snapshot.sh` | The disposable basis snapshot |
| `b1_text_base.sh` | Step 0 |
| `base_reproduction.txt` | Step 0's evidence |
| `linemap.out.json` | The rule carry |
| `b1_eval.py` | The in-build evaluator |
| `b1_mc_chain.py` | The multi-case chain patcher |
| `I82_PATCHES.json` | Every patch, rebind and added edge |
| `chain_diff/rr_to_mc_chain.diff` | The full patch |
| `b1_mc_run.sh` | One point |
| `b1_sweep.py` | The sweeps |
| `points_A.json`, `points_B.json`, `points_C.json`, `points_D.json`, `points_final.json` | The points |
| `sweep_final.tsv`, `sweep_final.log` | The sweep output |
| `b1_report.py`, `report.json` | Every point's per-mode E+R, phases, margins, smallest M, text-error budgets, and the candidates' binding terms |
| `b1_classify.py`, `classify_final.json` | Every row and (form, atom) term with its c = 1–4 values and class |
| `profile_trees/` | The multi-case profile trees: D1 at c = 1, 2, 3, 4, 8; P0, P1 and P2's tier 2; c = 4 with 12-node models; c = 8 with 4-node models |

**Read** (sha256 where pinned):

| Input | sha256 |
|---|---|
| The brief | `da183892…` |
| `AGENTS.md`; `agents/AGENT_TASK.md` | — |
| DESIGN_v2 (§0, §1.2, §1.4, §6, §8, §9, §10) | `5933b90b…1114` |
| RV105 REVIEW (N-10, N-11) | `d4807e9f…c2b008` |
| RV105 ADDENDUM_01 | `9071403f…599b` |
| QUAL (§1–§6) | `8edbf4b4…2c29` |
| DOMAIN (§1–§5) | `08a72dde…c9fd` |
| PLAN (§2.1, decision 9) | `f274a614…3def` |
| I72's law record | `cbf34c52…313d` |
| G7 Pass A `profile_tree.json` | `148cc1c8…f82f` |
| u4_g7_06's chain (sha256 of its per-file list) | `8761920d…ce42` |
| `g7_linemap.py` | `993a2306…ce42` |
| `text_row_diff.py` | `73e20e0a…bb3d` |

- I65's G6 and G7 to U9 RETURN records, by heading.
- **RR:** D-7 (RR:8880–8888), "RV89 on U4 G5 part 2: PASS; … the margin standard", "Owner decision: ROOT may raise M up to 6.0 GiB without asking", "B0 selected on DESIGN_v2; …", and "U8 Pass B returned …" with "RV98 confirms U8's Pass B …".
- **Code at `d8c88774d0`** (`retained_memory.rs` blob `843b7ec5…`):
  - PP `retained_memory.rs`: `caps`, `family_clauses`, `cap_rows`, `REGISTERED_PROFILES`, `cap_priced_maximum`, `priced_maximum`, the generated `profile`, `admission_bound`, `late_observations`, `complete_observations`, `ordinary_solve_attempted` and `phase_caps`;
  - PP `lib.rs`: `run_linear_static_preview_observed` (the case loop and the shared basis), `permitted_run`, `ReservedNotice` and `retained_w1`;
  - PP `retained_wire.rs`: `domain_hash` sites and `ordinary_value`.

**Limits.**
- **The numbers price B1's design** through emulated multiplicities, on today's one-case call graph (§2.4). They are not B1's in-build G5.
- **The in-build atoms** are the registered dev/test build's, which is the identity being re-registered. The release identity would need its own record, as QUAL §5.
- **The scope classification** is mechanical over c = 1–4 at D1's caps. The scope reasons in §2.2 come from the transaction text (T-2 to T-13) and the code's case loop, not from running B1.
- **I did not audit** the G-B and G-C restatements in §4.3 against B1's producer. RetainedErrorTextBytes' scope is an assumption.
- **Today's precommit reader is priced with its D1 loops rebound to c.** B1's reader alignment (DESIGN_v2 §2 and §3) is not in this code and was not priced.
