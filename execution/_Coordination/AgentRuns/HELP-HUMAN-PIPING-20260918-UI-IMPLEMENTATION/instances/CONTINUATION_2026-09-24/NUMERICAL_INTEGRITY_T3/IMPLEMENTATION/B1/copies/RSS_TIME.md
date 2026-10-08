# B1 SQ: peak resident memory and run time (PLAN_v2 §3.6; A1-S-1, A1-N-9)

*Paths `g6/…` are under `_run_records/g6/`; WT, NUM, R, P, PP as in B1_COMMON.*

I104, for ROOT's R9 reading against the owner's direction (target machines 32 GB; 16 GB "solving within practical timeframes", owner 2026-10-07).

## Method

- **Builds.** Dev/test: the registered scratch copy (b1-q `69002bc862` + `registration.diff`, so the Direct entry is admitted at M = 10.5 GiB), target `WT/targets/i104-sq-reg`. Release: the same tree, `--release`, target `WT/targets/i104-sq-rel`; the release build is Stale at admission, so it is measured through the private driver's witness tests.
- **Runs.** Every measurement under `WT/tools/t3_exclusive.sh` (all four lock slots, the memory guard up, no other heavy job), one process per measurement and one mode per process, the binary run directly: `/usr/bin/time -l <binary> <test> --exact --ignored --test-threads=1 --nocapture`. **3 repetitions;** the tables give the median and the maximum (`g6/rss/`).
- **Dev/test** runs the challenge binary itself, so its counting-allocator peak (requested heap, like-for-like with E_mov,max) and the process's RSS come from the same process. Its allocator does SeqCst atomics on every allocation, which slows these timings (A1-N-9).
- **Release** runs the witness entry points; their printed timings split the observed ordinary run (with the parse) from W1.
- **Furthest phase** (A1-S-1): from the witness-driver run of the same input, mode and build (QUAL_B1.md §4), mapped as PLAN_v2 §3.5: Preparation, Native or Candidate → W2; Staging or Serializer → W3; Precommit → W4; a successor → W5 (the whole W1 transaction).

## 1. Headline, against 16 GB and 32 GB

| | Measured (largest of every run) | 16 GB (16 GiB) | 32 GB (32 GiB) |
|---|---|---|---|
| **Release, max RSS** | **206.5 MiB** (the three-case input, dense, a successor) | 1.3 % | 0.6 % |
| Release, peak footprint | 134.4 MiB (the same run) | 0.8 % | 0.4 % |
| Dev/test, max RSS | 161.0 MiB (the three-case input, dense, Direct) | 1.0 % | 0.5 % |
| Dev/test, requested-heap peak | 113.3 MiB (the same run): **1.21 % of E_mov,max** | 0.7 % | 0.3 % |
| **Priced worst case, E_mov,max + R** (dense; not a measurement) | 9,859,807,510 B = 9.18 GiB | 57 % | 29 % |
| **Release, wall time** | **≤ 1.77 s** per invocation; W1 ≤ 1.73 s (the three-case input) | | |
| Dev/test, wall time | ≤ 51.8 s per invocation (the three-case input; debug, with the counting allocator) | | |

- **Every challenge peak is within its bound**, with the bound chosen as A1-S-1 states (§2). Each entry point asserts it, and all 81 dev/test runs passed.
- **The largest measured heap is about 1 % of the priced E_mov,max,** because the profile prices every owner at the D1 caps as if live at once (the I65 method). It is not an estimate of typical use.

## 2. Dev/test: the challenge binary, registered build (3 repetitions; median / max)

Bound: E_mov,max (without R) for a run that did W1 work, shown on the public surface as a successor or an N1 notice; otherwise the W1 phase. Requested-heap peak is the challenge's counting allocator.

| Input | Mode | Route | Outcome | Requested-heap peak, MiB (median / max) | Bound | Peak / bound | Max RSS, MiB | Peak footprint, MiB | Real, s | Call, s |
|---|---|---|---|---|---|---|---|---|---|---|
| b2_k1e3 | dense | direct | notices(1) | 12.6 / 12.6 | E_mov_max 9,339.0 | 0.00135 | 37.6 / 38.0 | 26.5 / 26.8 | 20.50 / 20.52 | 20.50 / 20.52 |
| b2_k1e3 | dense | ordinary | ordinary_route | 12.6 / 12.6 | W1_phase 4,890.9 | 0.00257 | 31.3 / 31.3 | 21.8 / 21.8 | 0.17 / 0.17 | 0.17 / 0.17 |
| b2_k1e3 | sparse | direct | notices(1) | 12.6 / 12.6 | E_mov_max 9,282.7 | 0.00136 | 35.4 / 36.2 | 24.2 / 25.1 | 20.69 / 20.71 | 20.69 / 20.71 |
| b2_k1e3 | sparse | ordinary | ordinary_route | 12.6 / 12.6 | W1_phase 4,834.5 | 0.00260 | 29.3 / 29.3 | 19.7 / 19.7 | 0.09 / 0.09 | 0.09 / 0.09 |
| c1 | dense | direct | successor(2177405 B) | 40.0 / 40.0 | E_mov_max 9,339.0 | 0.00428 | 72.4 / 73.2 | 54.3 / 54.4 | 17.21 / 17.23 | 17.15 / 17.17 |
| c1 | dense | ordinary | ordinary_route | 12.7 / 12.7 | W1_phase 4,890.9 | 0.00259 | 31.2 / 31.2 | 21.6 / 21.6 | 0.14 / 0.14 | 0.14 / 0.14 |
| c1 | sparse | direct | successor(2176063 B) | 39.9 / 39.9 | E_mov_max 9,282.7 | 0.00430 | 69.8 / 71.8 | 51.8 / 52.2 | 17.23 / 17.29 | 17.17 / 17.21 |
| c1 | sparse | ordinary | ordinary_route | 12.7 / 12.7 | W1_phase 4,834.5 | 0.00262 | 28.7 / 28.7 | 19.0 / 19.0 | 0.09 / 0.09 | 0.09 / 0.09 |
| i3_three_case | dense | direct | successor(6651380 B) | 113.3 / 113.3 | E_mov_max 9,339.0 | 0.01213 | 161.0 / 161.0 | 112.3 / 112.3 | 51.56 / 51.72 | 51.38 / 51.55 |
| i3_three_case | dense | ordinary | ordinary_route | 30.2 / 30.2 | W1_phase 4,890.9 | 0.00618 | 56.0 / 56.0 | 46.4 / 46.4 | 0.39 / 0.39 | 0.39 / 0.39 |
| i3_three_case | sparse | direct | successor(6647340 B) | 113.2 / 113.2 | E_mov_max 9,282.7 | 0.01220 | 159.1 / 160.8 | 111.1 / 112.3 | 51.83 / 51.84 | 51.66 / 51.66 |
| i3_three_case | sparse | ordinary | ordinary_route | 30.2 / 30.2 | W1_phase 4,834.5 | 0.00625 | 53.4 / 53.4 | 43.8 / 43.8 | 0.26 / 0.26 | 0.25 / 0.25 |
| milestone | dense | direct | successor(114894 B) | 3.4 / 3.4 | E_mov_max 9,339.0 | 0.00036 | 20.5 / 20.5 | 7.8 / 7.9 | 0.65 / 0.65 | 0.65 / 0.65 |
| milestone | dense | ordinary | ordinary_route | 0.5 / 0.5 | W1_phase 4,890.9 | 0.00011 | 12.5 / 12.5 | 3.1 / 3.1 | 0.00 / 0.00 | 0.00 / 0.00 |
| milestone | sparse | direct | successor(113733 B) | 3.4 / 3.4 | E_mov_max 9,282.7 | 0.00036 | 20.5 / 20.5 | 7.8 / 7.8 | 0.65 / 0.70 | 0.65 / 0.69 |
| milestone | sparse | ordinary | ordinary_route | 0.5 / 0.5 | W1_phase 4,834.5 | 0.00011 | 12.5 / 12.5 | 3.1 / 3.1 | 0.00 / 0.00 | 0.00 / 0.00 |
| w2 | dense | direct | no_w1_work | 1.9 / 1.9 | W1_phase 4,890.9 | 0.00038 | 11.7 / 11.7 | 5.2 / 5.2 | 0.02 / 0.02 | 0.02 / 0.02 |
| w2 | dense | ordinary | ordinary_route | 1.9 / 1.9 | W1_phase 4,890.9 | 0.00038 | 10.9 / 10.9 | 5.1 / 5.1 | 0.02 / 0.02 | 0.02 / 0.02 |
| w2 | sparse | direct | no_w1_work | 1.9 / 1.9 | W1_phase 4,834.5 | 0.00039 | 11.7 / 11.7 | 5.2 / 5.2 | 0.02 / 0.02 | 0.02 / 0.02 |
| w2 | sparse | ordinary | ordinary_route | 1.9 / 1.9 | W1_phase 4,834.5 | 0.00039 | 10.9 / 10.9 | 5.1 / 5.1 | 0.02 / 0.02 | 0.02 / 0.02 |
| w_c2 | dense | direct | successor(455479 B) | 9.4 / 9.4 | E_mov_max 9,339.0 | 0.00101 | 27.6 / 27.8 | 14.9 / 15.1 | 1.22 / 1.24 | 1.21 / 1.22 |
| w_c2 | dense | ordinary | ordinary_route | 2.6 / 2.6 | W1_phase 4,890.9 | 0.00053 | 15.2 / 15.2 | 5.7 / 5.7 | 0.02 / 0.02 | 0.02 / 0.02 |
| w_c2 | sparse | direct | successor(455001 B) | 9.4 / 9.4 | E_mov_max 9,282.7 | 0.00101 | 27.7 / 27.9 | 15.0 / 15.2 | 1.21 / 1.23 | 1.20 / 1.22 |
| w_c2 | sparse | ordinary | ordinary_route | 2.6 / 2.6 | W1_phase 4,834.5 | 0.00053 | 15.3 / 15.6 | 5.7 / 6.0 | 0.02 / 0.02 | 0.02 / 0.02 |
| w_c2_ac | sparse | direct | successor(342907 B) | 7.2 / 7.2 | E_mov_max 9,282.7 | 0.00077 | 25.5 / 25.6 | 12.8 / 12.9 | 1.17 / 1.19 | 1.16 / 1.18 |
| w_c2_ac | sparse | ordinary | ordinary_route | 1.7 / 1.7 | W1_phase 4,834.5 | 0.00035 | 14.4 / 14.4 | 4.8 / 4.8 | 0.01 / 0.01 | 0.01 / 0.01 |
| process floor | — | — | — | — | — | — | 2.7 / 2.7 | 1.6 / 1.6 | 0.00 / 0.02 | — |

**Furthest phase reached** (A1-S-1: the witness-driver run of the same input, mode and build; QUAL_B1.md §4):

| Input | Witness outcome (both modes) | Furthest W1 phase | Direct here |
|---|---|---|---|
| milestone, W-C2, W-C2's (A, C), c1, three-case | Successor | **W5** (the whole transaction) | a successor, bounded by E_mov,max |
| `b2_k1e3` | Fallback(Candidate) | W2 | one N1 notice, bounded by E_mov,max |
| W2 (`law_tests::cap_maximal`, escaped, depth 16) | Fallback(Preparation) | W2 on the private driver | **no W1 work** through Direct: no successor and no notice, so W1 did not start; bounded by the W1 phase |

## 3. Release: the witness entry points (Stale at admission; the private driver), 3 repetitions

The witness's own timings split the observed ordinary run (with the parse) from W1. W-C2's witness runs W1 twice (at 4 MiB, then at 1 MiB), so its real time covers both; the W1 column is the larger. The `ordinary` rows are `control_ordinary_*`: the ordinary value route alone, on the witness stack.

| Input | Mode | Entry | Witness outcome | Max RSS, MiB (median / max) | Peak footprint, MiB | Real, s | Ordinary run (+ parse), s | W1, s |
|---|---|---|---|---|---|---|---|---|
| b2_k1e3 | dense | ordinary | — | 28.7 / 29.3 | 24.0 / 24.6 | 0.01 / 0.01 | 0.02 / 0.02 | — |
| b2_k1e3 | dense | witness | Fallback("Candidate") | 36.4 / 36.5 | 30.2 / 30.3 | 0.68 / 0.68 | 0.01 / 0.02 | 0.67 / 0.67 |
| b2_k1e3 | sparse | ordinary | — | 26.1 / 27.1 | 21.4 / 22.5 | 0.01 / 0.01 | 0.01 / 0.01 | — |
| b2_k1e3 | sparse | witness | Fallback("Candidate") | 33.3 / 34.3 | 27.2 / 28.1 | 0.68 / 0.68 | 0.01 / 0.01 | 0.67 / 0.67 |
| c1 | dense | ordinary | — | 29.3 / 29.3 | 24.6 / 24.6 | 0.01 / 0.01 | 0.01 / 0.01 | — |
| c1 | dense | witness | Successor | 83.9 / 83.9 | 62.4 / 62.4 | 0.59 / 0.60 | 0.01 / 0.01 | 0.58 / 0.58 |
| c1 | sparse | ordinary | — | 26.6 / 26.6 | 21.8 / 21.8 | 0.01 / 0.01 | 0.01 / 0.01 | — |
| c1 | sparse | witness | Successor | 81.2 / 81.3 | 59.7 / 59.8 | 0.58 / 0.59 | 0.01 / 0.01 | 0.57 / 0.58 |
| floor | — | floor | — | 2.8 / 2.8 | 1.7 / 1.7 | 0.00 / 0.01 | — | — |
| i3_three_case | dense | ordinary | — | 55.1 / 55.2 | 50.4 / 50.4 | 0.04 / 0.04 | 0.04 / 0.04 | — |
| i3_three_case | dense | witness | Successor | 206.3 / 206.5 | 134.3 / 134.4 | 1.74 / 1.76 | 0.04 / 0.04 | 1.69 / 1.72 |
| i3_three_case | sparse | ordinary | — | 51.2 / 52.2 | 46.5 / 47.5 | 0.03 / 0.03 | 0.03 / 0.03 | — |
| i3_three_case | sparse | witness | Successor | 201.8 / 203.8 | 131.4 / 133.0 | 1.73 / 1.77 | 0.03 / 0.03 | 1.69 / 1.73 |
| milestone | dense | ordinary | — | 7.8 / 7.8 | 3.2 / 3.2 | 0.00 / 0.00 | 0.00 / 0.00 | — |
| milestone | dense | witness | Successor | 16.0 / 16.2 | 8.5 / 8.7 | 0.03 / 0.03 | 0.00 / 0.00 | 0.03 / 0.03 |
| milestone | sparse | ordinary | — | 7.8 / 7.8 | 3.1 / 3.2 | 0.00 / 0.00 | 0.00 / 0.00 | — |
| milestone | sparse | witness | Successor | 16.2 / 16.2 | 8.6 / 8.6 | 0.03 / 0.03 | 0.00 / 0.00 | 0.03 / 0.03 |
| w2 | dense | ordinary | — | 8.3 / 8.3 | 5.3 / 5.3 | 0.00 / 0.00 | 0.00 / 0.00 | — |
| w2 | dense | witness | Fallback("Preparation") | 8.4 / 8.4 | 5.3 / 5.3 | 0.00 / 0.00 | 0.00 / 0.00 | 0.00 / 0.00 |
| w2 | sparse | ordinary | — | 8.3 / 8.4 | 5.3 / 5.3 | 0.00 / 0.00 | 0.00 / 0.00 | — |
| w2 | sparse | witness | Fallback("Preparation") | 8.4 / 8.9 | 5.3 / 5.8 | 0.00 / 0.00 | 0.00 / 0.00 | 0.00 / 0.00 |
| w_c2 | dense | ordinary | — | 10.5 / 10.6 | 5.9 / 5.9 | 0.00 / 0.00 | 0.00 / 0.00 | — |
| w_c2 | dense | witness | Successor | 24.6 / 24.8 | 16.9 / 17.1 | 0.11 / 0.12 | 0.00 / 0.00 | 0.06 / 0.06 |
| w_c2 | sparse | ordinary | — | 10.6 / 10.7 | 5.9 / 6.0 | 0.00 / 0.00 | 0.00 / 0.00 | — |
| w_c2 | sparse | witness | Successor | 24.5 / 24.6 | 16.8 / 16.9 | 0.12 / 0.12 | 0.00 / 0.00 | 0.06 / 0.06 |
| w_c2_ac | sparse | witness | Successor | 22.6 / 22.7 | 14.9 / 15.0 | 0.05 / 0.08 | 0.00 / 0.01 | 0.05 / 0.06 |


## 4. W1's increment: the Direct or witness run less the ordinary route alone (medians)

| Input | Mode | Dev/test requested heap | Dev/test RSS | Release RSS | Release time |
|---|---|---|---|---|---|
| c1 | sparse | +27.3 MiB | +41.1 MiB | +54.6 MiB | +0.57 s |
| c1 | dense | +27.3 MiB | +41.3 MiB | +54.5 MiB | +0.58 s |
| The three-case input | sparse | +83.0 MiB | +105.7 MiB | +150.6 MiB | +1.70 s |
| The three-case input | dense | +83.1 MiB | +105.0 MiB | +151.2 MiB | +1.70 s |
| W-C2 | sparse | +6.8 MiB | +12.4 MiB | +13.9 MiB | +0.12 s |
| W-C2 | dense | +6.8 MiB | +12.4 MiB | +14.0 MiB | +0.11 s |
| W-C2's (A, C) | sparse | +5.5 MiB | +11.0 MiB | — | — |
| `b2_k1e3` (fallback) | sparse | +0.0 MiB | +6.1 MiB | +7.3 MiB | +0.67 s |
| `b2_k1e3` (fallback) | dense | +0.0 MiB | +6.3 MiB | +7.6 MiB | +0.67 s |
| milestone | sparse | +2.9 MiB | +7.9 MiB | +8.4 MiB | +0.03 s |
| milestone | dense | +2.9 MiB | +8.0 MiB | +8.2 MiB | +0.03 s |
| W2 (no W1 work through Direct) | sparse | +0.0 MiB | +0.8 MiB | +0.0 MiB | +0.00 s |
| W2 | dense | +0.0 MiB | +0.7 MiB | +0.1 MiB | +0.00 s |

- `b2_k1e3`'s requested-heap peak is within 50 kB of its ordinary route's (13,249,270 against 13,201,145 B, dense). Its W1 work, a full native run and a refused candidate, adds almost nothing above the ordinary run's own peak.

## 5. What the measurements cover, and what they do not

- **Phase coverage.** W3–W5 are **measured at the caps:** c1 (one case at D1's count caps) and the three-case input (C = 3, \|A\| = 3, Σ l_i = L) publish successors through the registered Direct entry and on the release witness stack. `b2_k1e3` measures W2's full native run at the caps. W2's own cap-maximal input does no W1 work through Direct.
- **What the published inputs leave below the caps** (`g6/census_published_inputs.json`, typed counts from I86's requests):
  - **at the cap:** n = m = g = 32, l = 128, c = 3 and L = 384 (three-case), 4 + 4 materials with 16 temperature points, 128-byte identifiers, raw depth 16 (three-case);
  - **below it:** Σr = 66 of 192, springs = 21 of 192, raw values 6,249 of 16,384 (three-case; c1 3,150), string bytes 52,106 of 65,536, key bytes 34,643 of 65,536.
- **Host.** This host has 128 GB and runs under no memory pressure; it has no swap (A1-N-9). **Behaviour on a 16 GB machine (compression, swap) is not observed.** The 16 GB judgement rests on the measured peaks and run times above.
- **Build.** Debug-build times are pessimistic: the counting allocator does SeqCst atomics on every allocation (A1-N-9). **The release times are the ones B7 and B8 will read.**
- **RSS.** macOS RSS includes shared pages, so the peak footprint is recorded beside it. The process floor is 2.7 MiB (dev/test) and 2.8 MiB (release).
- **Non-claims.**
  - No supported-machine statement: that is owner-held.
  - No concurrency claim: one invocation per process.
  - M bounds requested and moving heap per invocation; it is not an RSS bound.

## 6. DEF-O's availability on the cap-maximal inputs (item 9; RR "RV115 confirms S-4 (a)'s soundness …", NC-1). Report only

**Method** (`def_o_*` entry points in the witness tests; `g6/witnesses/{dev,rel}/def_o_*`; identical in both builds). The private transaction is run to T-9 (custody, preparation, the batch call, the freezes) exactly as `w1_transaction` runs it. Then, for each case in A, the report gives:
- the case's end;
- every row verdict's failing predicate, by class:
  - **`mm_to_si`:** the rows DEF-O projects from mm to SI with a second rounding (`ProductUnit::Millimetre`: the three displacement components and the displacement magnitude);
  - **`support_magnitude`:** the support force and moment magnitudes (the nested hypot);
  - **`other`;**
- a Candidate fallback's typed cause, which is the first failing verdict (`certify_final`).

| Input (cap-maximal) | Mode | Cases in A: end | Rows per case (mm_to_si / support_magnitude / other) | Failing rows (SharperExact): mm_to_si / support_magnitude / other | The fallback's reported cause |
|---|---|---|---|---|---|
| `b2_k1e3` | sparse | 1: **Candidate** | 128 / 64 / 1,921 | **2 / 0 / 79** | row 6, `displacement_magnitude` (mm_to_si), SharperExact |
| `b2_k1e3` | dense | 1: **Candidate** | 128 / 64 / 1,922 | **2 / 0 / 79** | row 7, `displacement_magnitude` (mm_to_si), SharperExact |
| `c1` | both | 1: frozen | 128 / 64 / 1,921–1,922 | 0 / 0 / 0 | — |
| The three-case input | both | 3: all frozen | 128 / 64 / 1,921–1,922 each | 0 / 0 / 0 | — |
| W2 (`law_tests::cap_maximal`) | both | custody refused (not solved): Preparation | — | — | — |
| W-C2 (not cap-maximal) | both | A frozen; C Native | 16 / 10 / 145 | 0 / 0 / 0 | — |

**The shares,** over the two SharperExact fallbacks these inputs produce (`b2_k1e3`, one per mode):

| Cause | As the reported cause (first failing row) | As the sole cause (every failing row in the class) | Failing rows |
|---|---|---|---|
| DEF-O's mm→SI second rounding | 2 of 2 | **0 of 2** | 4 of 162 (2.5 %) |
| The support magnitude's nested hypot | 0 of 2 | 0 of 2 | 0 of 162 |
| Other rows | 0 of 2 | 0 of 2 | 158 of 162 |

- **Each `b2_k1e3` fallback stays a fallback if both DEF-O properties are repaired.** 79 rows outside both classes also fail SharperExact. The case's reported cause is a mm row only because it comes first in row order.
- **The first failing row** in each mode is a `displacement_magnitude` row. Its predicate vector is (SharperExact false, SharperBinary64 false, DecimalSi true, DecimalRaw true): both sharper tests fail, and both decimal tests pass. The report does not name the second failing mm row's kind.
- **No support magnitude fails on any cap-maximal input,** in 64 rows per case.
- **No row fails on the publishing inputs:** c1, and all three cases of the three-case input.
- **Limits:**
  - two fallbacks are a small sample;
  - the classes are assigned by row kind, not by a counterfactual single-rounding replay;
  - the `other` rows' kinds are not broken down here.

