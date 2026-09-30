# K6b (I16), checkpoint 0: the plan

**Status: read and design only.** No cargo, build, test or run. This file is the only file written.

- **Base:** `<wt>/k6b`, branch `codex/piping-k6b-20260929`, HEAD main `ab02ee3a6` (K4 merged as PR #1054, head `5a46a6278`). Every code line cited below is on this base.
- **T3 records** were read at `<wt>/numerics`, head `eac809604`.
- **Paths:** `P/`, `T3/`, `H`, `FK`, `SD`, `NI`, `SA` and `PP` as in `I15_K6_IMPLEMENTATION.md`. `K4R` = `FK/structural/retained/`. `K4T` = `P/core/solver/frame_kernel/tests/retained_k4/`.
- **Delegation:** a harness-native background subagent of ROOT's HELP_HUMAN session (D-GOV-35). I made no Git writes and no index operations.
- **Read:**
  - Root `AGENTS.md`, `agents/AGENT_TASK.md`, `_COMMON.md` and `I8R_K1_RESUME.md:24-50`; the I16 brief and the I15 brief.
  - `ROOT_RULINGS_V1.md`: "K6b and V-K: spawn", "K4 merged", "K4: Q5 amended", every K6 section, and K4's stale-design item 9.
  - K6 `RETURN.md` §8.5–§8.7 and §13; K4 `RETURN.md` §12, §14, §16 and addenda 1 and 2.
  - The code: all of `H` (`src/k6/*`, `src/bin/k6_observe/*`, `runner/*`, the test list), `FK/structural.rs:1-30`, every declaration in `K4R/*.rs`, `K4T/models.rs:306-425` and the format of `K4T/r1_large.txt`.
  - The I17 brief, for coordination only.

## 1. Scope 1–5

| Scope | Plan | Checkpoint |
|---|---|---|
| 1. The export | §2 | A0 |
| 2. The W1 mode, the adapter, counts, admission, runner | §3, §4 | A1, A2 |
| 3. What each run records | §3.4, §5.4 | A1, B |
| 4. Models, sizes, ascent | §5 | A2, B |
| 5. Seconds per work unit | §5.3, Q5 | B |

## 2. A0: the export

### 2.1 Rulings needed before A0

**A0-1 (conflict C-1): the flat `pub use` does not compile.**
- `FK/structural.rs:28` already defines `pub const POLICY: &str = "M03-INTEGRITY-v1"`. The export list includes `adaptive::POLICY` (`"M03-INTEGRITY-MP-v1"`, `K4R/adaptive.rs:57`).
- So `pub use retained::adaptive::{POLICY, …}` beside `mod retained;` is a duplicate definition (E0255).
- The flat form would also put about 70 generic names into `structural` (`Kind`, `End`, `Component`, `Dof`, `Spring`, `Constraint`, `Station`, `Refusal`, `classify`, `threshold`, `layout`). `FK/structural/sparse.rs:36` glob-imports `structural`.
- Options:
  - **(a) A facade module** in `structural.rs`, right after the private `mod retained;`: `pub mod retained_api { pub use super::retained::…; }`. K4's names stay verbatim, nothing collides, and the scan is one grep.
  - (b) Flat, with one alias: `POLICY as RETAINED_POLICY`.
  - (c) `pub mod retained` with `pub mod` submodules. This exposes K4's internal layout, and the brief keeps `mod retained;` private.
- **Recommend (a).** V-K cherry-picks the same commit.

**A0-2 (conflict C-2): `PrecisionState` leaks a private type.**
- `PrecisionState` (`K4R/adaptive.rs:1891`) is an enum. Its variants hold `Arc<Solved<4|8|16>>`, and `Solved` (`:864`) is on §16's "Not exported" list.
- Enum variant fields take the enum's visibility. Raising it to `pub` therefore fires rustc's warn-by-default `private_interfaces` lint four times (`PrecisionState::P128::0` and so on).
- H's non-test build would then show FK's warnings.
- Options:
  - **(i) Export it,** with `#[allow(private_interfaces)] // the payload (Solved) stays crate-private (K4 RETURN §16)` on the enum. This is a lint attribute, not a code change. Consumers can name the type and call its three methods, but cannot read a payload.
  - (ii) Leave `PrecisionState` and `RetainedSolve::state` crate-private: two items fewer than the list.
  - (iii) Also export `Solved`. Its `where Wide<L>: SupportedWidth` bound then fires `private_bounds`, and the problem cascades. Not viable.
- **Recommend (i).** The list is kept exactly, and the attribute is A0's only token that is not a visibility change. K6b itself does not use `PrecisionState`.

**A0-3: methods and fields are exactly those §16 names.**
- §16 names the methods of `CaseLimit`, `InvocationMeter`, `RetainedSolve`, `PrecisionState`, `PrimitiveSource`, `Dof` and `RetainedCombination`. It shows the fields of the structs it lists; those fields are raised to `pub`.
- It names no fields or methods for `Binary64Outcome`, `AttemptWork`, `WidthWork` or `SumWork` ("as field types of the evidence"). These four are exported as types only. An enum's variants are public with the enum.
- These stay crate-private: `Component::{ALL, index, from_index, is_translation}`, `Kind::{ALL, index}`, `SpringKind::offset`, `AttemptStop::escalates`, `StageWork::add` and `RetainedSolve`'s four fields.
- Q7 says what this leaves unobservable.

**A0-4: markers.** The 19 `#[allow(dead_code)] // F2a API…` markers on exported items are removed (marked "m" in §2.3). Markers on items that stay private are unchanged.

### 2.2 The facade (form (a)), in `FK/structural.rs` after `:5` (`mod retained;`)

```rust
/// W1a's public surface: T3 K4 RETURN §16's export list (K6b A0). No product crate names it.
pub mod retained_api {
    pub use super::retained::adaptive::{
        absolute_bound, body_extent, classify, classify_rows, classify_rows_floored, coupled_scales,
        intensified_k, solve_case, solve_cases, stress_scale, threshold, AttemptOutcome, AttemptReason,
        AttemptRecord, AttemptRole, AttemptStop, BudgetScope, CaseLimit, CaseOutcome, GateTest,
        InvocationMeter, PrecisionState, Publication, PublishedRow, Refusal, RetainedEvidence,
        RetainedSolve, RowClass, StageWork, StorageCounts, UnresolvedReason, VerificationSummary,
        FLOOR_RATIO_BITS, K_SQRT2_BITS, K_TWO_SQRT2_BITS, METHOD_TOKEN, POLICY, PRECISIONS, RCOND_LABEL,
    };
    pub use super::retained::combine::{CombinationOutcome, CombinationReason, RetainedCombination};
    pub use super::retained::factor::{reverse_cuthill_mckee, BodyGeometry};
    pub use super::retained::ledger::LedgerRefusal;
    pub use super::retained::recover::{layout, End, Kind, QuantityId, QuantityMeta};
    pub use super::retained::source::{
        Component, Constraint, DirectionalSpring, Dof, MemberProperty, NodalLoad, PrimitiveSource,
        SourceError, SourceParts, Spring, SpringKind, Station, StraightMember, SupportGroup,
    };
    pub use super::retained::verify::{e_hat, phi_512, resolution_hats, PHI_SCALE_BITS};
    pub use super::retained::wide::multi::{AttemptWork, Binary64Outcome, WidthWork};
    pub use super::retained::wide::WideError;
    pub use super::retained::wide_sum::SumWork;
}
```

- The facade names 39 + 3 + 2 + 1 + 5 + 14 + 4 + 3 + 1 + 1 = **73 items**. That is §16's main list (63) plus the ten that revision 5a.3 adds.
- `retained/mod.rs` is unchanged: its `pub(crate) mod` paths are reachable from a child of `structural`.
- The payload types that are already public are `StructuralError` (`FK/structural.rs:223`) and `SumError` (`FK/exact_sum.rs:25`).

### 2.3 Item by item: `pub(crate)` → `pub` at each line; "m" marks a marker line removed

| File | Items (line) | Methods (line) | Fields (lines, count) |
|---|---|---|---|
| `K4R/adaptive.rs` | METHOD_TOKEN 55, POLICY 57, RCOND_LABEL 59, PRECISIONS 62, FLOOR_RATIO_BITS 64, K_SQRT2_BITS 66, K_TWO_SQRT2_BITS 69 (m 68); CaseLimit 75; InvocationMeter 89; BudgetScope 119; AttemptStop 126; body_extent 271; coupled_scales 291; stress_scale 306 (m 305); intensified_k 313 (m 312); threshold 338; absolute_bound 344; RowClass 356; classify 409; StageWork 779; GateTest 880; PrecisionState 1891 (+A0-2 attribute); VerificationSummary 1944; AttemptRole 2034; AttemptReason 2041; AttemptOutcome 2076; StorageCounts 2087; AttemptRecord 2096; Refusal 2139; UnresolvedReason 2157; PublishedRow 2182; Publication 2191; classify_rows 2199 (m 2198); classify_rows_floored 2210; RetainedEvidence 2273; RetainedSolve 2313; CaseOutcome 2352 (m 2350); solve_cases 3015 (m 3014); solve_case 3075 (m 3074) | CaseLimit::new 79 (m 78), get 82; InvocationMeter::new 96 (m 95), charged 100 (m 99), limit 104 (m 103), exhausted 107; PrecisionState::precision 1899, published 1909, encoding 1917; RetainedSolve::publish 2328 (m 2327), evidence 2332 (m 2331), source 2336 (m 2335), selected_precision 2340 (m 2339), state 2344 (m 2343) | StageWork 780–803 (19); VerificationSummary 1946–1955 (8); StorageCounts 2088–2090 (3); AttemptRecord 2097–2134 (21); PublishedRow 2183–2187 (5); Publication 2192, 2194 (2); RetainedEvidence 2274–2308 (27). RetainedSolve's 2314–2319 stay `pub(crate)` |
| `K4R/source.rs` | Component 29; Dof 63; StraightMember 82; Spring 97; SpringKind 104; DirectionalSpring 122; Constraint 132; NodalLoad 139; Station 147; SupportGroup 156; SourceParts 166; MemberProperty 178; SourceError 190; PrimitiveSource 271 (its fields stay private) | Dof::global 69, from_global 73; PrimitiveSource::new 353 (m 352), nodes 570, members 573, springs 576, directional_springs 579, constraints 582, loads 585, stations 588, supports 591, node_count 594, dof_count 597, constraint 601, free_dofs 605, body_of_node 610, body_count 613, body_nodes 617, member_index 622, encoding 628, stiffness_encoding 663 | Dof 64–65 (2); StraightMember 83–92 (10); Spring 98–100 (3); DirectionalSpring 123–127 (5); Constraint 133–134 (2); NodalLoad 140–142 (3); Station 148–150 (3); SupportGroup 157–161 (5); SourceParts 167–174 (8) |
| `K4R/recover.rs` | Kind 30; End 46; QuantityId 53; QuantityMeta 80; layout 101 | — | QuantityMeta 81–85 (4) |
| `K4R/factor.rs` | BodyGeometry 60; reverse_cuthill_mckee 290 | — | — |
| `K4R/ledger.rs` | LedgerRefusal 72 | — | — |
| `K4R/combine.rs` | CombinationReason 45; CombinationOutcome 68 (m 66); RetainedCombination 77 | RetainedCombination::solve 82 (m 81) | — |
| `K4R/verify.rs` | PHI_SCALE_BITS 51; e_hat 321; resolution_hats 335; phi_512 360 | — | — |
| `K4R/wide.rs` | WideError 150 | — | — |
| `K4R/wide/multi.rs` | Binary64Outcome 666; WidthWork 1002; AttemptWork 1074 | — | (A0-3: none) |
| `K4R/wide_sum.rs` | SumWork 77 | — | (A0-3: none) |

### 2.4 Tally

- **73 items** re-exported.
- **239 one-token visibility edits:**
  - 73 declarations;
  - 36 methods: `CaseLimit` 2, `InvocationMeter` 4, `RetainedSolve` 5, `PrecisionState` 3, `PrimitiveSource` 19, `Dof` 2, `RetainedCombination` 1;
  - 130 fields.
- 19 marker lines removed, the facade block, and one attribute line (A0-2(i)).
- **Files:** `FK/structural.rs`, and `K4R/{adaptive,source,recover,factor,ledger,combine,verify,wide,wide_sum}.rs` and `K4R/wide/multi.rs`. Nothing else.
- The exported signatures and public fields name only exported or already-public types. I checked every field and variant; `PrecisionState` is the only exception (A0-2).

### 2.5 A0's checks (then I report A0)

1. **FK's non-test build has no warning.** This is the `private_interfaces` check.
2. **FK's full suite passes** (`--offline --locked`, `-j 8`, `RUST_TEST_THREADS=4`), with the count of K4's final run: 394 (K4 RETURN A2.3).
   - The S11 site table (`FK/tests/s11_site_table.rs`) counts fold shapes per function, so a visibility edit changes no count.
   - K4's source scan (`K4T/adaptive_tests.rs:850-879`) looks only for transcendental calls.
3. **A throwaway probe crate** under `<wt>/scratch/i16/` (outside the repository). It depends on FK by path, `use`s all 73 names and builds without warnings. This shows the export is nameable from outside FK. H's committed sufficiency test comes at A1.
4. **rustfmt** on the changed files only.
5. **The scan** (`_run_records/a0/scan.txt`):
   - `git grep -n retained_api -- P/` finds only `FK/structural.rs` (and, after A1, H);
   - `git grep -nwE 'solve_case|solve_cases|PrimitiveSource|SourceParts|RetainedSolve|CaseLimit|InvocationMeter' -- P/`, outside FK, finds nothing;
   - no manifest or lockfile changes.
   - So PP, SA, NI, SD, the runners and the apps name no exported item. T9 and the both-entry gate are not run (brief, Gates).

## 3. The W1 mode

### 3.1 The mode and its CLI (Q2)

- `Mode::W1a`, spelled `"w1a"`, with `materializes_n2() = false` (`H/src/k6/mod.rs:24-57`).

```
k6_observe --model <id> | --model-file <f>  --mode w1a  --heap-cap-bytes <n>
           [--repeats 5] [--time-budget-s <s>] [--first-repeat-limit-s 600] [--counts-file <f>]
           [--case-limit <u64>] [--invocation-limit <u64>]     (default u64::MAX for both; §5.1)
           [--w1-prefixes]                                      (Q1(c); after the timed repeats)
           [--dump-published <f>]                               (the rows, for the R1 comparison)
k6_observe --emit-source --model <id>                           (K4SRC bytes in hex; Q6)
```

- `--entry-repeats` does not apply to w1a (there is no SA entry).

### 3.2 The adapter (`H/src/k6/w1/adapter.rs`): `K6Model` → `SourceParts`

- **Nodes:** `model.nodes[i].1`, in R1's order. The node index is K6's index.
- **Members:** for member k in R1's order, `StraightMember { id: k + 1, node_i, node_j, E, G, A, Iy, Iz, J from model.section, y_reference }`.
  - The ids are R1's 1-based member numbers, which are also the keys of `K4T/r1_large.txt`.
- **Constraints:** every restrained DOF, at value 0.0.
- **Loads:** one `NodalLoad` per nonzero `(dof, value)`, with `source_id = "k6:<global dof>"`.
- **Stations:** one per member at fraction 0.5 (id = the member id), as K4's adapter has, for R1's mid-span `Mbs` rows (N-3).
- **Springs, directional springs and support groups:** none. RF-LARGE and the nine have none.
- **Validation:** an `Err(SourceError)` from `PrimitiveSource::new` is an outcome (`SourceRefused`, with the error), never a panic (Q4).
- **Differences from K4's adapter (K4 RETURN §12.1), by design.** The results agree with R1 under the 1e-9 predicate, but their bits differ:
  - the section bits: K6's formula with `PI` products (`H/src/k6/models.rs:278-297`), not fl(A_basis) with PI_Q;
  - the `y_reference`: P1's rule (ROOT's ruling N4), not the (0, 0, 1) default;
  - the load source ids.

### 3.3 The staged sequence, under K6's allocator and observer

1. **`w1_source`:** the adapter and `PrimitiveSource::new`, with their time and heap.
2. **`w1_solve`:** `solve_case(source, CaseLimit::new(Lc), &mut InvocationMeter::new(Li))`, the whole call. Its stage peak is the whole-call heap peak.
3. **Then, outside any stage,** the outcome, attempt and parity lines.

- After the repeats, `--w1-prefixes` runs the prefix calls `w1_prefix_1..k` (Q1(c)), then the summary.
- `solve_case` equals `solve_cases` on a one-element slice (`K4R/adaptive.rs:3075-3084`). No model here has several cases (C-9), so `solve_cases` is exercised only by the equality test.

### 3.4 What each run records

**The counts line (every mode).** K6's keys are unchanged, and W1's keys are appended from the O(nnz) counts phase:
- `w1_source_ok` and `w1_source_error`; `w1_nodes`, `w1_members`, `w1_stations`, `w1_constraints`, `w1_loads`, `w1_free_dofs`, `w1_bodies`;
- `w1_pattern_entries` (both triangles; checked equal to K6's `pattern_entries` for frames);
- `w1_profile_entries` and `w1_half_bandwidth`, by K4's own rule (`K4R/factor.rs:351-398`): the structural free–free adjacency, the exported `reverse_cuthill_mckee`, and first = the minimum rank over each row's neighbours and itself;
- `w1_rows` = `layout(&source).len()`, which is 7N + 18m + r here (N nodes, m members, r constrained DOFs);
- `w1_limbs_per_entry_{128,256,512,1024}` = 4, 4, 8, 16, K4's table (`K4R/adaptive.rs:2455-2459`);
- `w1_storage_bytes_<p>` = (pattern + profile) × limbs × 8, as K4 counts;
- `w1_wide_bytes_{4,8,16}` = 48, 80, 144, the in-memory sizes (§3.5);
- `w1_source_encoding_len` and `w1_source_encoding_fnv64` (K4SRC);
- `estimate_adm_bytes_w1a` (E_max), `estimate_w1_sel128_bytes`, and every term of §3.5 by precision.

**`attempt` lines** (a new kind; Q2), one per attempt per repeat:
- repeat, index, precision, role, and outcome with its reason;
- residual_basis, corrections, gate, pivot_margin_min, rcond, residual_worst;
- `storage_{pattern,profile,limbs}`;
- `own_<the 19 StageWork fields>` and `shared_<the 19>`;
- shared_work, shared_built_here, stop_rule_work, verification_work, verification_shared_work, verification_shared_built_here;
- the verification summary: data_blocks, shift_factorizations, uc_missing, g_max, g_violation.

**`outcome` (the existing kind, with W1's fields):**
- the class: `Selected`, `Refused`, `Unresolved` or `SourceRefused`;
- selected_precision, verification_precision, the reason (a bounded `Debug`), and the attempts;
- the rows by class (relative_verified, absolute_verified, input_derived, unpublishable);
- `publication_fnv64`, `evidence_fnv64`, `retained_state_fnv64`, `meter_charged` and `budget_reached`.

**`parity` items (the existing kind).** A false item is a stop; the runner already stops on parity failures.
- `w1_profile_equals_storage`, `w1_pattern_equals_storage`, `w1_limbs_equal_table`, `w1_source_encoding_equals_evidence`.
- `w1_work_closes`: the sum of the own stages, plus shared_work and verification_shared_work where built here, equals `meter.charged()`.
- `w1_shared_stages_close`, per attempt: the sum of the shared_stages fields equals shared_work + verification_shared_work.
- `repeat_determinism` (an existing item): the publication, the evidence and the charged work are equal across repeats.
- The two closure identities are my reading of `K4R/adaptive.rs:2484-2503`, `:2636-2658` and `:2796-2799`. A1 confirms them on the 10-member models before anything relies on them. If K4's accounting does not close, I report it; K4R is not edited.

**Stage lines:** `w1_source`, `w1_solve` and `w1_prefix_<j>`, each with its time, heap peaks in both models, and allocation calls.

**Runner records:** RSS, footprint and load, as in K6.

**`--dump-published <f>`** (`k6b-rows v1`), one line per row:
- the key (`u.n.c`, `mag.n`, `end.m.i|j.c`, `st.s.c`, `R.n.c`), the kind, the body, the class with its bound bits;
- the value bits, or `underflow±` or `overflow±`.

### 3.5 The W1 admission estimate (derived as K6 derived E_adm)

**The size of a wide value.** `Wide<L>` is `{ negative: bool, exponent: i64, significand: [u64; L] }` (`K4R/wide.rs:200-204`). So w_L = 8L + 16 bytes: 48, 80 and 144. It is not the 8L that "limbs × 8" counts (C-4).

**The widths.** Each attempt runs at p with (L, R) = (4, 4) at 128, (4, 8) at 256, (8, 16) at 512 and (16, 16) at 1024 (`K4R/adaptive.rs:2529-2532`). Each verification runs at P with (L_P, W) = (4, 8) at 256, (8, 16) at 512 and (16, 16) at 1024 (`:2667-2669`).

**The terms** (m members, n DOFs, n_f free DOFs, nnz pattern entries, P profile entries, B blocks, rows published rows):
- **S(p), `Shared<L,R>`** (`:834-861`), kept in the group cache until the call returns (`:2374-2384`):
  - m·(164 w_L + 16): `MemberOperators<L>` (`K4R/assemble.rs:65-83`: 9 axis, 5 coefficient, 72 B and 78 K_e values);
  - nnz·(w_L + w_R): k and k_q;
  - m·(5 w_R + 8): bounded_q;
  - P·w_L + n_f·(2 w_L + 56): the factor rows, order, first, scale and screens;
  - B·w_L.
- **T(p), S(p)'s build transient** when q ≠ p: m·(164 w_R + 16), the `members_q` of `:949`, dropped after `.bounded()` (`:962`).
- **U(p), the state** (`Solved<L>` and `Recovered<L>`, `K4R/recover.rs:203-210`): (n + 6m + rows)·w_L.
- **V(P), `VerifyShared<L_P,W>`** (`K4R/verify.rs:383-399`), kept: nnz·w_{L_P} + 144m·w_W + B·(4 w_{L_P} + 8).
- **X(P), the verification pass's transient:** P·w_{L_P} (the shifted factorization), plus 5·rows·(w_{L_P} + 8) (the per-row `Option<Wide>` vectors, `:571-600`), plus 2n_f·w_{L_P}.
- **E_fix:** O(n + m + rows). It covers the case and group (the source twice, the K4SRC identity, the layout, the ledger, `Structure`, `Ordering` and the blocks), the evidence (encodings, attempts, publication) and the harness's own model. The coefficients come from the definitions at A1.

**The estimates.**
- **E_max** = E_fix + the maximum, over the schedule's time order (128, 256, v256, 512, v512, 1024, v1024), of everything kept so far plus the current transient.
- **E_sel128** is the same up to v256: the path RF-LARGE took at 10 and 100 members (K4 RETURN §22.8).

**Sizes.** The sizes of crate-private types are constants derived from their struct definitions, with the lines cited, as K6 derived `EXPANSION_BYTES` (`H/src/k6/counts.rs:39-41`). The sizes of exported types come from `size_of`. A test pins E for one model against a hand derivation (mutant M6).

**Order of magnitude** (checkpoint-0 arithmetic, with P ≤ 60, 90 and 45 per member for CHAIN, TREE and CONT; the exact values come from the counts at A2):
- E_max ≈ 0.28 MB per member, and E_sel128 ≈ 0.08 MB per member.
- The profile terms are under 10%. The per-member operators dominate: 164 wide values per member per precision, plus 144 per member for each verification's K_e.
- **So on RF-LARGE, "profile entries × limbs × 8 B" is not what W1's memory is.** The estimate carries both, and the ratios report both.

### 3.6 Where the code goes

- **`H/src/k6/mod.rs`:** `Mode::W1a` and `pub mod w1;`. `H/src/lib.rs` is untouched, so the DEC-050/053 pins see no change.
- **`H/src/k6/w1/{mod,adapter,counts,staged,rows}.rs`:** the adapter, W1's counts and estimate, the staged sequence and the prefixes, and the row keys with R1's derived quantities:
  - N and T; Mb and Mbs, as |·| by `sqrt` with power-of-two scaling, never `hypot`;
  - tw = T/k_t and ext = N/k_a, with k_t = fl(fl(G·J)/L) and k_a = fl(fl(E·A)/L).
- **`H/src/k6/counts.rs`:**
  - `K6Counts` gains `w1: Option<W1Counts>`, parsed when present, so K6's committed counts lines still parse;
  - `admission_estimate_bytes(Mode::W1a, …)` returns E_max, or `u128::MAX` without W1 counts, which the half-cap rule then refuses;
  - `f1b_estimate_bytes(W1a)` = `None`.
- **`H/src/bin/k6_observe/{main,w1}.rs`:** the W1a branch and its lines.
- **`H/README.md`:** a K6b subsection.

## 4. The runner (`H/runner/k6_runner.py`)

- **K6's tiers keep a frozen `K6_MODES = ('sparse', 'dense', 'lane-id', 'lane-lu')`.**
  - T1 and T2 take `MODES` directly today (`:347-349`). Extending `MODES` in place would add W1 rows to K6's tiers (C-7).
  - `MODES = K6_MODES + ('w1a',)`.
- **`TIERS` gains W1-T1 to W1-T4 (§5)**, with `W1_PAIR = ('w1a', 'sparse', 'w1a', 'sparse')`.
  - `rotate()` then gives ABAB for an even model index and BABA for an odd one.
  - K6's 138 rows keep their order and ids, and W1's rows follow them.
- **Unchanged functions:** `estimate_key('w1a')` gives `estimate_adm_bytes_w1a` with no change, and `refusal_by_name` needs none either (w1a is never refused by name).
- **`run_parameters`:** w1a runs 5 repeats (Q3), with K6's timeouts and caps.
- **`binary_argv`:** for the first w1a process of each model in its tier (pass 1) only, `--dump-published` and `--w1-prefixes`.
- **Reused unchanged:** the admission rule, the allocator, the watchdog, `launch`, the record schema and the quiet-host wait.
- **New: `H/runner/k6b_analysis.py`** (standard library). It builds K6b's packet from the records, per model:
  - W1's outcome and attempts, work by stage and precision, and storage;
  - heap, RSS and footprint, with their ratios to E_max and E_sel128;
  - the prefix increments;
  - s/LME per process and per pair, and the W1/binary64 multiple;
  - the growth fits of heap and RSS against members, per family.
  - Its `--packet` and `--project` handle w1a.
- **Tests go into the existing classes of `runner/test_k6_runner.py`** (`PlanAdmission`, `Aggregation`, `ModelHashes`), so `P/tests/test_performance_harness_runner.py` needs no edit.
  - The two 138 pins (`:375`, `:512`) become "K6's tiers still give 138 rows, unchanged", plus W1's own count.

## 5. The schedule (Q3)

### 5.1 Limits and caps

- **Budgets:** `CaseLimit::new(u64::MAX)` and `InvocationMeter::new(u64::MAX)`.
  - `exhausted()` needs charged ≥ `u64::MAX`, which only saturation reaches.
  - Both are recorded in `start`. `budget_reached`, or `Unresolved(Budget(_))` on a full call, is a stop.
- **Caps:** C = 8 GiB for the RSS watchdog, and a heap cap of 7.5 GiB.
- **Admission:** K6's rule as ruled at the B2 stop (footprint ρ, and projected RSS ≤ 0.8C), with ρ = 2 by default, and the ascent.

### 5.2 The rows

AX and ROT share every count W1's estimate uses, because K4's profile is on the structural pattern. So one row per family and size covers both orientations.

| Tier (slot) | Models | Processes (A = w1a, B = sparse) | E_max | E_sel128 | K6's E_adm, sparse | Admission and reason | Projected time |
|---|---|---|---|---|---|---|---|
| W1-T1 (K6B-S1) | RF-LARGE at 10 (×6); the DEC-053 nine (≤ 97 members, ≤ 336 DOFs) | 15 × 4 = 60 | 2.7–2.9 MiB; nine ≤ 27 MiB | 0.7–0.8 MiB | 0.39–0.40 MiB | admitted: E × 2 ≪ C/2 | ≤ 3 min |
| W1-T2 (K6B-S1) | RF-LARGE at 100 (×6) | 24 | 26–28 MiB | 7.3–8.0 MiB | 3.8–4.0 MiB | admitted | ≤ 5 min |
| W1-T3 (K6B-S2) | RF-LARGE at 1,000 (×6) | 24 | 262–282 MiB | 73–79 MiB | 37.9–39.5 MiB | admitted: E × 2 ≤ 0.6 GiB | 1–15 min |
| W1-T4 (K6B-S3, only as ROOT approves) | RF-LARGE at 10,000 (×6) | 24 | 2.56–2.76 GiB | 0.71–0.78 GiB | 379–395 MiB | at ρ = 2 deferred (5.1–5.5 GiB > C/2); admitted on ρ measured at ≥ 100 members if ≤ 1.45. If 1,000 selects at 128, ρ ≈ 0.3 | 10 min to 2 h |

- **The time basis.** K4's 100-member frames take 1–4 s each in the debug suite (K4 RETURN §22.8). No release time has been measured.
- **A bracket,** linear in members at fixed bandwidth: 0.02–0.4 s per call at 100 members, 0.2–8 s at 1,000, and 2–80 s at 10,000.
- **A w1a process** is its repeats, plus the prefixes (about two more calls), plus counts read from the file.
- **Projections before each grant.** A2's `--smoke` measures 10 and 100 members in release, and `--project` projects each tier before its grant. The two-hour stop applies per slot.
  - At 10,000 members, if the projection exceeds two hours, the options are three repeats and no prefixes. That is ROOT's call.

### 5.3 The order within a slot

- Tiers run in ascending size. Within a tier, models follow `rf_ids`: CHAIN, TREE and CONT, each AX then ROT.
- Each model's four processes run consecutively (ABAB or BABA). One process runs at a time.
- Before each process: wait while the 1-minute load is above 8, until it is below 6, and require `memorystatus_level` ≥ 80. The memory guard runs throughout.
- A run is admitted only after its previous size (same family, orientation and mode) has been recorded.

### 5.4 The R1 comparison (records, not CI)

`T3/IMPLEMENTATION/K6B/_run_records/scripts/k6b_r1_compare.py`, standard library, with bytecode writing off:
- It reads each pass-1 rows dump and R1's `references.json` (`7b176dbb…`).
- **Coverage:** every row at 10 and 100 members. At 1,000 and 10,000, the eight sampled positions R1 publishes, with scales from the full solution (`REFERENCES/README.md:199`).
- **Exact values:** every value is parsed exactly (`Fraction`), so CONT n10000's expectations below binary64's range compare exactly.
- **The predicate is unchanged:** |obs − exp| ≤ 1e-9·max(|exp|, scale).
- **A verified row that fails it is a stop** (Q7, item 7).

## 6. Tests

Debug tests stay at 100 members or fewer. No test asserts a time or memory bound.

### Rust tests (H)

- **`tests/k6b_export.rs` (the export is sufficient):**
  - `use` all 73 names from `retained_api`;
  - build a two-member invented cantilever: `SourceParts`, `PrimitiveSource::new`, `solve_case`;
  - read `publish().rows`, their classes, `evidence().attempts[_].{stages, shared_stages, storage}` and `InvocationMeter::charged`;
  - call `classify`, `threshold`, `layout` and `reverse_cuthill_mckee`;
  - run `RetainedCombination::solve` on two load cases of one stiffness.
- **`tests/k6b_adapter.rs`:**
  - the FNV-1a of the adapter's K4SRC equals the committed digest (`H/observations/k6b/sources.txt`, written by Q6's independent Python) for RF-LARGE at 10 and 100 members and for the nine;
  - `encoding()` is deterministic across two builds, and does not depend on the order of the members, constraints and loads given to `new`.
- **`tests/k6b_w1.rs`:**
  - **Determinism:** two runs on the six 10-member models give equal `Publication`, `RetainedEvidence` and charged work.
  - **K4's own entry:** the staged `w1_solve` equals a direct `solve_case`, and `solve_cases(&[src])[0]`, on the same source, in publication and evidence.
  - **R1 at 10 and 100 members:** every R1 row of the 12 models (the `ref` lines of `K4T/r1_large.txt`, read with `include_str!`; N-1), including the derived N, T, Mb, Mbs, tw and ext, under the unchanged predicate.
  - **Counts:** `w1_profile_entries`, the pattern and the limbs equal every attempt's `storage`.
  - **Work:** the two closure identities, and the per-precision aggregation, on one 10-member model.
  - **Prefixes:** prefix j's completed attempts equal the full call's first j, and its outcome is `Unresolved(Budget(Case))`.
  - **The estimate:** E for RF-LARGE-CHAIN-n00010-AX equals a hand derivation written in the test.
- **`tests/k6b_bin.rs`:**
  - `--mode w1a` on a 10-member model with 2 repeats prints the W1 counts keys, the attempt and outcome lines, and every parity item true;
  - `--emit-source` equals the library's encoding;
  - under a small cap, w1a is refused by the half-cap estimate rule only;
  - w1a has no by-name refusal at 10,000 members.

### Python tests (runner, standard library)

- **`PlanAdmission`:**
  - K6's 138 rows are unchanged;
  - W1's rows: their count, ABAB/BABA, `run_parameters` and `estimate_key('w1a')`;
  - admission by E × ρ, deferral at ρ = 2 at 10,000 members, and no refusal by name.
- **`Aggregation`:**
  - s/LME is the median `w1_solve` time divided by `meter_charged`;
  - the prefix increments and the pair multiple.
- **`ModelHashes`:** the Python K4SRC of the independent generator's models equals `sources.txt`.

### Debug time

H's debug suite took 59–68 s at K6 (K6 RETURN §11). K6b adds about 12 debug solves at 100 members (1–4 s each) plus small ones, so I expect +30–60 s. It is measured before and after at A1.

## 7. Mutants (at C)

Rules: NONE first. Each mutant runs from a clean `git archive`, with its own target under `<wt>/k6b-mut/<id>/`, at most three at once at `-j 4`.

| # | Mutant | Intended kill |
|---|---|---|
| M1 | The adapter drops the last member | the adapter digest; R1 at 10 |
| M2 | The adapter swaps two nodes' coordinates (node order) | the digest; R1 |
| M3 | The adapter passes Iy as J | the digest; R1 (T, tw, rotations) |
| M3e | The adapter swaps Iy and Iz | **equivalent on this model set:** Iy = Iz in every RF-LARGE and DEC-053 section (`H/src/k6/models.rs:286-293`, `H/src/lib.rs:358`, `:415`). Derived and reported |
| M4 | A stage (the shift) dropped from the per-precision work | `w1_work_closes`; the aggregation test |
| M5 | An attempt's work filed under the next precision, or the limbs table at 256 set to 8 | the aggregation test; `w1_limbs_equal_table` |
| M6 | The estimate uses w_L for k_q, or omits V(P) | the hand-derived estimate test; the runner's plan test |
| M7 | The runner treats w1a as an n² mode, or skips ρ for w1a | `PlanAdmission` |
| M8 | `StageWork` left out of the facade | `k6b_export` does not compile (a compile-time kill, recorded as such) |
| M9 | W1 tiers without alternation (AABB) | `PlanAdmission`: ABAB/BABA |
| M10 | A prefix limit of b_j − 1 | the prefix test |
| M11 | s/LME divided by own-stage work instead of charged | `Aggregation` |
| M12 | The row keys swap ends i and j | R1 at 10 (Mb, N) |
| M13 | W1's profile from K6's value-based pattern instead of the structural one | `w1_profile_equals_storage` |

Survivors and equivalences are reported. No test is weakened.

## 8. Questions Q1–Q7

### Q1. Peak memory per precision

**The facts.** `solve_case` is one call. The group cache keeps every precision's shared build (`Shared`, `VerifyShared`) until the call returns (`K4R/adaptive.rs:2374-2384`). So memory accumulates by precision, and the observer sees only the call's peak.

**Options:**
- **(a) The whole-call peak.** The heap peak in both allocator models, with the selected precision and the attempts. Per-precision storage then comes from `StorageCounts` (limbs × 8) and §3.5's terms, checked against measured peaks at small sizes. On its own, (a) checks only the sum along the path taken (128, 256 and v256 on RF-LARGE).
- **(b) A stage observer inside K4R,** behind a feature. It edits K4R, so it needs its own ruling. Not needed.
- **(c) Budget-truncated prefixes, through the public API.**
  - From the full call's attempt records, take the case-work boundary b_j after each segment in time order (solve 128; solve 256; the v256 pass; …):
    - b_1 = a1.shared_work + a1's own solve work;
    - b_2 = b_1 + a2.shared_work + a2's own solve work;
    - b_3 = b_2 + a2.verification_shared_work + a2.verification_work.
  - `solve_case` under `CaseLimit::new(b_j)` stops at segment j + 1's first budget check (`StageGuard::test`, `:225-233`, checked per member in `form_members`, `K4R/assemble.rs:466-469`), with `Unresolved(Budget(Case))`.
  - Its heap peak is the high-water mark through segment j, plus a derivable overshoot: the next build's `Vec::with_capacity(m)` of `MemberOperators` (`:466`) and its contexts.
  - The increments give each precision's memory and each verification's; the prefix times give segment times (Q5).
  - It is deterministic, changes nothing in FK beyond A0, and changes nothing in the allocator.
- **(d) An invented model that escalates,** to measure the 512 and 1024 increments. RF-LARGE selects at 128 at 10 and 100 members (K4 RETURN §22.8), so without (d) the 512 and 1024 terms stay derived.

**Recommend (a), with (c) as its per-precision measurement,** at every admitted size in pass 1 (dropped at 10,000 if the projection needs it). Add (d) only if ROOT wants measured 512 and 1024 terms.

### Q2. The mode's name and CLI, the counts-line fields, and the JSONL additions

- **Name:** `w1a`, the method K4 implements; W1b is separate. The alternative is `w1`. The CLI is in §3.1, and the counts, attempt, outcome, parity and stage fields are in §3.4.
- **The attempt records fit no existing kind.**
  - (i) A new kind, `attempt`. The existing kinds stay unchanged, and the runner's readers filter by kind, so they ignore it.
  - (ii) A JSON array inside `outcome`: this needs an array writer, and gives one long line per repeat.
  - **Recommend (i).** I read the brief's "JSONL kinds reused unchanged" as "K6's kinds keep their schema"; ROOT to confirm.
- **"The counts line gains … work by stage and precision" (C-3).** Work exists only after the solve, so it goes into the `attempt` lines. The counts line gains W1's static counts, the limbs table and the estimate.

### Q3. The schedule, the limits and the slot times

See §5. The per-case limit and the invocation meter are both `u64::MAX`, recorded in `start`. Reaching either is a stop.

### Q4. Which of the DEC-053 nine W1a covers

**All nine.**
- They are the harness's invented chains (8, 24, 32 and 48 members) and planar grids (4×3 to 7×8 nodes).
- Each is built from straight `FrameElement`s with one section, rigid restraints and nodal forces (`H/src/lib.rs:705-752`, `:358`, `:415`; `H/src/k6/models.rs`, `from_fixture`). None has a curved member, a user stiffness element, a spring or a nonlinear support.
- W1a covers straight frames with global-axis restraints and springs, so none is refused by coverage.
- `PrimitiveSource::new` could still refuse one in validation (a degenerate axis, or a subnormal derived primitive). That would be recorded as `SourceRefused` with its `SourceError`, and reported.
- The labels "mixed_nonlinear" and "two_span_opposing_gap" name topology proxies only; the fixtures are linear frames.

### Q5. The interleaving

- **Pairs:** A = w1a and B = K6's sparse mode, per model. Each is its own process of the same release binary. The four processes of a model run consecutively in one slot: ABAB for an even model index in its tier, BABA for an odd one.
- **Repeats:** 5 in-process repeats in each process, with the 600 s first-repeat limit. The prefixes run after A's repeats in pass 1 only, and are excluded from the timed figures.
- **The figure:** s/LME = the median over repeats of `w1_solve`'s elapsed time, divided by `meter_charged`. The charged work is deterministic and equal in every repeat and pass; this is checked. The minimum is reported beside the median.
  - Per model: the pass-1 and pass-2 values, and their ratio (drift), each with the load before and after and `memorystatus_level`.
- **Per segment (a prefix):** (t(prefix_j) − t(prefix_{j−1})) / (b_j − b_{j−1}). This is one sample each, and labelled so.
- **Context:** the W1/binary64 multiple, per pass: W1's median `w1_solve` divided by B's median `entry_checked` (and by B's staged total) in the adjacent process.
- **Across models:** a log-log fit of time against charged work, as an observation. No claim is made; RETURN's claims stay the heap and RSS growth fits and the heap-to-E ratios.

### Q6. Checking the adapter independently of `K4T/models.rs` and of K6's generator

**The facts.** K6's canonical bytes (`k6-model v1`) carry no stations, constraints or load ids, so they cannot check the adapter's output (C-13). K4SRC can (`K4R/source.rs:628-662`; K4 RETURN §13).

**Options:**
- **(a) A standard-library Python check from R1.** `_run_records/scripts/k6b_adapter_check.py` builds each model's K4SRC directly from `references.py --model <id>`, imported as K6's crosscheck imports it, with bytecode writing off:
  - R1's node and member order, and its coordinates rounded once;
  - K6's stated section formula in Python floats (the same operations give the same bits; K6's crosscheck already pins them);
  - P1's `y_reference` rule from R1's integer coordinates;
  - the adapter's constraint, load and station rules;
  - its own K4SRC writer, written from §13.
  - It is compared with `k6_observe --emit-source` for all 24 RF-LARGE models, 10,000 members included (K6's crosscheck took 50 s).
  - On a difference it prints the first differing field. A difference is a stop.
- **(b) Compare with `K4T/r1_large.txt`'s model lines** at 10 and 100 members. This is not independent of K4, and it differs by design in the section bits, `y_reference` and the load ids. Context only.
- **(c) A CI chain.** The runner's independent Python generator (not the Rust one), with the same writer, produces `H/observations/k6b/sources.txt`: the sha256 and FNV-1a of each model's K4SRC, for 33 models.
  - A Python test regenerates the file.
  - A Rust test checks the adapter's FNV-1a against it at 10 and 100 members and on the nine.

**Recommend (a) and (c), with (b) as context.** K6's own crosscheck (`K6/_run_records/crosscheck/k6_crosscheck.py`) is re-run once and recorded, to show that K6's models still equal R1 on this base.

### Q7. What K4's evidence cannot show through the export list, and what I propose instead (no edit beyond A0)

1. **Time per K4 stage:** not observable, since the call is one call. Q1(c) gives segment times.
2. **Memory per precision:** see Q1.
3. **`AttemptWork`'s per-width operation counts and totals, and `SumWork`'s parts:** exported as types only (A0-3).
   - Instead, K6b uses `StageWork` (all 19 stages, per attempt, own and shared) and `InvocationMeter::charged`, tied together by the closure identities.
   - Per-width counts would also need `SupportedWidth` exported, since `AttemptWork::width` has it as a private bound. I do not propose that.
4. **The verification pass's per-block internals** (Uc_c, S_c, est_c, the shift's σ tries): only `VerificationSummary` shows them (θ, B_b, data blocks, shift factorizations, uc_missing, g).
   - This is enough for work and memory. The shift's work is `StageWork::shift`.
5. **The sizes of crate-private types** (`Wide<L>`, `MemberOperators`, and so on): derived constants, with their lines (§3.5).
6. **K4's structural profile before a solve:** recomputed in H with the exported `reverse_cuthill_mckee`, and checked against every attempt's `storage` (a parity line).
7. **Row claims at 2^-64:** there is no exact truth for K6b's source bits. K4's `solve_hp` expectations were computed for K4's adapter's bits.
   - So "outside its claim" is detected only through R1's 1e-9 predicate. R1's intended model differs from K6b's binary64 source by about 1e-16 relative, so a verified row that fails the predicate is outside its claim. A row that passes is not thereby shown to be within it.
   - K4 checked the claims on its own RF-LARGE sources at 10 and 100 members (addendum 1), and V-K is the standing check.
   - Options: (a) as stated; (b) also run K4's GEN `solve_hp` on K6b's sources at 10 members, as a recorded one-off. **Recommend (a).**
8. **`PrecisionState`'s payload:** see A0-2.

## 9. Conflicts between the brief and the code, and notes

- **C-1:** the `POLICY` collision (A0-1).
- **C-2:** `PrecisionState`'s private payload (A0-2).
- **C-3:** work cannot appear in the counts line, which is written before the solve (Q2).
- **C-4:** "limbs × 8 B" understates a wide value in memory, which is 8L + 16 bytes: 1.5× at L = 4, 1.25× at L = 8 and 1.125× at L = 16 (§3.5).
- **C-5: K6's profile is not W1's profile.**
  - K6's `rcm_profile_entries` is value-based: it skips zero entries (`H/src/k6/counts.rs:4-9`). K4's profile is structural (`K4R/factor.rs:351-398`).
  - So K6's counts cannot stand for W1's storage. On the AX models K6's count is smaller (CHAIN n10: AX 168, ROT 529); K4's is the same for both orientations.
- **C-6:** the group cache accumulates across precisions (Q1).
- **C-7:** K6's T1 and T2 use `MODES`, and the 138-row schedule is pinned twice (§4).
- **C-8:** the JSONL needs a new kind (Q2).
- **C-9:** no RF-LARGE or DEC-053 model has several cases, so `solve_cases` is exercised only by the equality test.
- **C-10:** the Iy↔Iz adapter mutant is equivalent on this model set (M3e).
- **C-11:** R1's JSON samples eight positions at 1,000 members and above (§5.4).
- **C-12:** claims can be checked only at R1's 1e-9 (Q7, item 7).
- **C-13:** K6's canonical bytes cannot check the adapter; K4SRC is used instead (Q6).
- **C-14:** the brief cites K4 at `7d8fa9c0e`; every line here is re-located on the merged code. The list's only change is the variant `CombinationReason::OperandsDiffer` (RV19-3), which does not affect A0.
- **N-1: R1's rows for the CI test.**
  - Recommended: `include_str!` of `K4T/r1_large.txt`. It holds R1's values at 10 and 100 members, pinned by GEN to R1's sha256. CI checks out FK's tests, so nothing is duplicated.
  - The alternative is a committed extract in H, about 0.6 MB.
- **N-2: W1 counts-only at A2.** A W1 counts-only pass at 1,000 and 10,000 members during A2 does O(nnz) work: the adapter, validation and RCM, with no solve. It would run one process at a time under a 512 MiB cap, as ROOT's N7 allowed K6. **I ask ROOT's approval.**
- **N-3: stations add work.** The stations add 6m of the 7N + 18m + r rows, and their share of the stop rule's and recovery's work.
- **N-4: the B runs double as a cross-check.** They re-run K6's sparse mode on K6b's binary. Their deterministic heap peaks should equal K6's B1 records, since the allocation sequence is the same. This is recorded as a cross-check, not a stop.
- **N-5: W1 against binary64, as context.** Per model, W1's published displacements against B's `--dump-solution` (up to 1,000 members): context only, no claim.

## 10. Write set, host, and what is not done

- **Write set:**
  - `FK/structural.rs` and `K4R/**`: A0 only;
  - `H/**`;
  - `T3/IMPLEMENTATION/K6B/`.
  - `P/tests` is untouched (§4). No new dependency and no lockfile change.
- **Host** (`I8R_K1_RESUME.md:24-50`):
  - `RUSTUP_TOOLCHAIN=1.97.1`, `RUSTUP_AUTO_INSTALL=0`, `CARGO_INCREMENTAL=0`, `--offline --locked`, at most `-j 8`, `RUST_TEST_THREADS=4`, and at most two cargo jobs;
  - target `<wt>/k6b-target`, scratch `<wt>/scratch/i16`, mutants `<wt>/k6b-mut/`;
  - the observation binary is `--release`, built from `git archive` of the exact commit;
  - the memory guard's log is checked after every heavy phase;
  - no dense or n² path anywhere in K6b: only W1 and sparse.
- **Not done, by the brief:**
  - no limit is proposed;
  - no product-level run (V-P) and nothing on Linux;
  - no V-K reference family, floor check or seeded fault;
  - K6's ceiling and grid runs are out of scope.
- **Next:** A0, once ROOT rules on A0-1 to A0-4.
