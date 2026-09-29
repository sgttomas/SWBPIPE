# I17 checkpoint 0: the V-K plan (VP-ROBUST kernel lane, `numerical_robustness`)

**Status: plan only.** Nothing was built or run beyond read-only, standard-library Python over `references.json` and `floor_kinds.json` (`python3 -B`, no bytecode written). No Git write, no index operation. This file is uncommitted, in `<wt>/vk`.

**Abbreviations** (as the brief's): `P/` = `projects/chirality-piping/`; `T3/` = the NUMERICAL_INTEGRITY_T3 folder; `FK` = `P/core/solver/frame_kernel/src/`; `K4R` = `FK/structural/retained/`; `K4T` = `P/core/solver/frame_kernel/tests/retained_k4/`; `SD` = `P/core/solver/sparse_direct/src/`; `VR` = `P/validation/benchmarks/numerical_robustness/` (new); `H` = `P/core/solver/performance_harness/`; R1 = `T3/REFERENCES/`; R7 = `T3/DESIGN_NUMERICS/REV_5A3_CANDIDATE/D1_REV_5A3_SSTAR_RESOLUTION_R7.md`.

**Bases.** Code: `<wt>/vk`, branch `codex/piping-vk-20260929`, main `ab02ee3a6` (K4 merged). K6b's A0 export does not exist yet; every export reference below is to K4 `RETURN.md` §16. Records: numerics head `eac809604`.

**Read** (sha256 prefixes where they matter): Root `AGENTS.md`; `agents/AGENT_TASK.md`; `_COMMON.md` (`6bb845bf`); `I8R_K1_RESUME.md:24-50`; `I17_VK_IMPLEMENTATION.md` (`06b16b82`); `I15_K6_IMPLEMENTATION.md` (Scope §6, Gates, Return, Constraints); `I16_K6B_IMPLEMENTATION.md`; `DESIGN.md` rev 5a.2 (`fb62ef4a`) §4.1.6, §4.1.6.1, §4.7, §4.8's parity protocol, §4.10, §6 (V-K row `:1023`), §7.1, §7.3; `ROOT_RULINGS_V1.md` (`a5e70d7a`): "K4: spawn and rulings" (stale items 4, 9, 10, `:1140-1150`), "K4: Q5 amended" (`:1169-1177`), "K4: rulings on I12's checkpoint-0 plan", `:1866` (the five byte-identical `factor.rs` functions), "K4: rulings on RV19's review" (amendment A1), "K4 merged", "K6b and V-K: spawn"; K4 `RETURN.md` (`78b576e8`) §12, §13, §14, §16, §19, addenda 1 and 2; `K4T/models.rs` (`20150654`), `K4T/references_tests.rs` (structure only); R1 `README.md` (`5a89bad9`), `references.json` (`7b176dbb`, checked), `references.py` (`80d473a7`, checked; `main`, `model_json` only); `floor_kinds.json` (`83265615`, checked) and `floor_kinds.py`'s docstring; K6 `RETURN.md` §6.1, §8.5, §13; `H/runner/k6_runner.py` (`launch`, `binary_argv`); R7 §7; the code sites cited below. I have **not** read GEN's `r1_adapt` body and will not (Q6).

---

## 1. Scope items 1–12: where each is met

| Scope | Plan section |
|---|---|
| 1 The crate | §2; Q1 |
| 2 The case adapters | §3, §4; Q2, Q6 |
| 3 The kernel lane (+ binary64 sparse gate for RF-MECH and RF-LARGE parity) | §5 |
| 4 What is compared | §5, §6 |
| 5 The predicate, represented basis | §6 |
| 6 Floor check, reporting, `pass_absolute_range`, enumerated `not_covered` | §6, §7 |
| 7 Discrimination check | §8 |
| 8 RF-RANGE | §9 |
| 9 Seeded faults, feature, guard | §10; Q4 |
| 10 RCM equality | §11 |
| 11 Per-case records | §12 |
| 12 Scale runs | §13; Q5 |

## 2. Crate layout and dependencies

```
VR/Cargo.toml        package piping_numerical_robustness, publish = false
                     [dependencies] open_pipe_stress_frame_kernel (path), open_pipe_stress_sparse_direct (path),
                                    serde_json = { version = "1", features = ["float_roundtrip"] }
                     [features] seeded-faults = ["open_pipe_stress_frame_kernel/mutation-controls"]   (no default)
VR/Cargo.lock        in-repo FK, SD; registry serde_json/serde/itoa/ryu/memchr at numerical_integrity's locked versions (offline)
VR/README.md         scope, the two lanes (kernel now, product V-P), the gate, how to regenerate and run
VR/cases/            gen_vk_cases.py (stdlib generator, §3); r1_<family>.jsonl (10 files); not_covered.json;
                     absolute_range_rows.json; large_models.sha256; SHA256SUMS
VR/src/              lib.rs; cases.rs (parse → SourceParts, R1 key map); exact.rs (std-only bignum, exact predicate);
                     sha256.rs (std-only, FIPS 180-4 vectors); lane.rs (W1 per case, observed rows, derived tw/ext);
                     compare.rs (outcomes, counts, report); floor.rs (S*, §7); controls.rs (§8);
                     binary64.rs (sparse/dense gate parity, §5.3); records.rs (§12); rcm.rs (§11)
VR/tests/            adapter.rs, lane_<family>.rs, engine.rs, floor.rs, controls.rs, invariance.rs, parity.rs,
                     rcm.rs, records.rs, feature_guard.rs; seeded_faults.rs (#![cfg(feature = "seeded-faults")])
VR/examples/         vk_scale.rs (RF-LARGE ≥ 1,000, counting capped allocator); vk_records.rs (regenerate records)
VR/runner/           vk_scale_runner.py (drives vk_scale through K6's runner functions, §13); run_seeded_faults.py (§10)
VR/observations/     kernel_lane/<family>.json; scale/…; seeded/kill_matrix.jsonl; SHA256SUMS
```
- **FK changes (the only ones):** `FK/../Cargo.toml` gains `[features] mutation-controls = []`; the cfg-gated sites of §10, and one cfg-gated selector file `K4R/seeded.rs` declared in `K4R/mod.rs` (§10.2). A0's export arrives by cherry-pick; V-K makes no other visibility change.
- **CI discovery:** `P/tools/release/check_release_readiness.py:28-31,70-81` finds every `Cargo.toml` under `validation/benchmarks`, so VR becomes the 40th manifest of the numerical job (`cargo test --offline --manifest-path … --locked`, no `--features`). The DEC-025 sweep gains the same manifest.
- **No `product_physics`** (Q1). No `nonlinear_integration`: the binary64 gate is reachable from FK and SD alone (`SparseStructuralSystem::new` takes optional contributions and symmetry, `FK/structural/sparse.rs:873`; `SD/structural.rs:101`).

## 3. How R1's cases reach CI (Q2)

CI omits `projects/*/execution/` (`.github/workflows/piping-desktop-e2e.yml:188`). So R1 reaches CI only through committed, generated files in `VR/cases/`, pinned to R1's hashes.

- **Generator** `VR/cases/gen_vk_cases.py`, standard library only, `sys.dont_write_bytecode = True`. It checks the sha256 of `references.json` (`7b176dbb…`), `references.py` (`80d473a7…`) and `floor_kinds.json` (`83265615…`) before reading them, and refuses on a mismatch. Modes:
  - default: write `VR/cases/*` and `SHA256SUMS`;
  - `--check`: regenerate in memory and compare bytes with the committed files (as K4's GEN);
  - `--large <dir>`: write the adapted RF-LARGE models at 1,000 and 10,000 members (not committed; about 1–15 MB each) and verify them against `VR/cases/large_models.sha256`;
  - `--compare-k4`: the one-off recorded comparison of §4.4.
- **R1's full models** come from `references.py` imported as a module (its family builders, then `model_json(defn, full=True)`, which is exactly what `--model <id>` prints, `references.py:2984-2994`). That is K6's method (`IMPLEMENTATION/K6/_run_records/crosscheck/k6_crosscheck.py`). For three cases (one of 10,000 members) the generator also runs `python3 -B references.py --model <id>` as a subprocess and asserts byte equality with the in-process JSON, so the equivalence is checked, not assumed. The whole build takes about 6–7 min (K6: 6.5 min for R1's full run).
- **Committed case files**, JSON Lines, one case per line, `sort_keys`, compact separators, one file per family (about 2.7 MB in all):
  - `id`, `family`, `basis`, `flags`, `units`, `refuse`;
  - `k4src_sha256`: the sha256 of K4SRC bytes built by the generator's **second, independent path** from the `--model` JSON (§4.3);
  - `model` (null for n ≥ 1,000): node names and coordinates as 16-hex binary64 bits in R1's order; members as (R1 name, node indices, E, G, A, Iy, Iz, J, y_reference bits); springs and directional springs; rigid DOFs; loads (nonzero components) and RF-CANCEL's contributions in authored order;
  - `scales`: R1's class-scale decimal strings, by class (with RF-WEAK's `@region` classes);
  - `rows`: `[R1 key, expected decimal string, class, row scale or null]`; the row scale is RF-CANCEL's recommended (net-governed, binding) column, null elsewhere; for the two represented-basis cases, `expected_represented`'s strings;
  - `controls`: `[id, discriminates, "value" | "outcome", {key: decimal string}]`;
  - `s_full` (n ≥ 1,000 only): S(kind) of the complete reference solution (§7.2), as decimal strings.
  - **Every number from R1 is R1's decimal string, byte for byte.** No expected value, scale or control value is ever converted to binary64 for a decision.
- **Sub-range decimals** (RF-LARGE-CONT-n10000: u.C2500.UZ, u.C4999.UZ, R.S1250.UZ and Mb.A3750.i in AX; Mb.A3750.i in ROT; about 1e-714 to 1e-2864) are carried as R1's strings like every other row, parsed exactly by `VR/src/exact.rs` (sign, arbitrary-length integer mantissa, decimal exponent). `absolute_range_rows.json` copies the five rows and their class scales so CI's engine tests use R1's own values (§6.4).
- **CI check of the files:** a test verifies `VR/cases/SHA256SUMS` with the in-crate SHA-256 (std-only; K3 and K4 tests have the same precedent, `K4T/support.rs:119`).

## 4. The adapter and its independence (Q6)

### 4.1 The adapter's rules (reviewed code: the generator's path A plus `VR/src/cases.rs`)

Written from R1's README §2 and the JSON, D1 and K4's `source.rs` interface; not from `K4T/models.rs`'s adapter logic or GEN's `r1_adapt`.
- **Inputs rounded once:** every R1 input string (decimal, `p/q`, `d*2^k`) is parsed as an exact rational and rounded once to binary64 (`float(Fraction)`, correctly rounded): coordinates, E, G, spring k, loads, contributions. THIN-A and THIN-B's G = fl(E/(2(1 + ν))) from the exact ν.
- **Section:** A = fl(PI_Q·(OD² − ID²)/4), Iy = Iz = fl(PI_Q·(OD⁴ − ID⁴)/64), J = fl(PI_Q·(OD⁴ − ID⁴)/32) (= 2·Iy exactly), with R1's own PI_Q (`references.py:68`), each one exact rational rounded once. Nothing in the adapter calls `powi`, `hypot` or a transcendental.
- **Order and ids:** nodes in R1's order (index = position); members numbered 1.. in R1's member order; springs and directional springs numbered 1.. in R1's support order, one id space (K4 refuses a shared id).
- **Springs:** a spring whose direction has one nonzero component is a global-axis `Spring` on that DOF; any other is a kernel-only `DirectionalSpring` with R1's integer direction. **Per spring, not per node** (differs from K4, §4.4).
- **Rigid restraints:** `Constraint { dof, value: 0.0 }`. No support groups (R1 publishes no magnitude rows).
- **Loads:** one `NodalLoad` per nonzero component, source `l<i>`; RF-CANCEL's contributions one each, source `c<k>` in authored order. An exact zero component is omitted.
- **Stations:** one per member at fraction 0.5 (id = member id), for `Mb.<m>.mid`.
- **y_reference:** the global unit axis on which the chord has the smallest |component| (lowest index on ties), decided exactly on the binary64 chord. Deliberately not K4's rule (§4.4). Every R1 quantity is convention-free, so any valid y_reference gives the same references.
- **RF-MECH-K0:** R1's k = 0 spring is omitted (a zero stiffness contributes nothing). A separate test asserts that K4's source refuses the literal model with `SourceError::NonPositiveSpring`; the refusal is recorded (K4's position, `K4T/references_tests.rs`; ROOT confirms, C10).
- **RF-CANCEL UDL cases** (3) are W1b's and excluded, as K4 did.

### 4.2 The R1 key map (in `cases.rs`)
`u.<n>.U*`, `th.<n>.R*` → `Displacement(Dof)`; `R.<n>.<DOF>` → `Reaction(Dof)` (R = K u − f, support on structure, as R1); `S.<n>.<i>.F*|M*` → `SpringAction` or `DirectionalSpringAction` (−k u, action on the structure, as R1); `N.<m>` and `T.<m>` → the j-end `EndAction` components Ux and Rx (node-on-element, local; tension and R1's torque sign positive); `Mb.<m>.i|j` → the end's (Ry, Rz) magnitude; `Mb.<m>.mid` → the station's (Ry, Rz) magnitude; `tw.<m>`, `ext.<m>` → derived (§5.2). A global-axis spring's off-axis components (and a rotational spring's force components) have no published row: they are **structural zeros**, asserted exactly `0` in R1 and counted separately (§6.3; 282 rows, against K4's 150 in its families).

### 4.3 The independent checks of the adapter
1. **CI, canonical bytes:** for every case, `sha256(PrimitiveSource::new(parts).encoding())` in Rust (parts from path A's committed model) equals `k4src_sha256`, which the generator computed from the `--model` JSON through a **separate** K4SRC encoder (path B, written from K4 `RETURN.md` §13 and `K4R/source.rs:628-659`), applying the same stated rules independently. This ties the Rust-built kernel source to `references.py --model` rounded once, node and member order included. 201 cases in CI; the 12 large models in `vk_scale` before any solve.
2. **Generator `--check`:** field-by-field equality of the committed model with fl() of `--model`, recorded.
3. **One-off comparison with K4's committed adapter output** (`K4T/r1_cases.txt`, `r1_large.txt`, read-only, `--compare-k4`): every numeric input must be bit-identical (coordinates, E, G, A, Iy, Iz, J, k, load values and order). The expected differences are enumerated in §4.4; any other is a finding.
4. **The reviewer's own check** against `references.py` (the brief's gate).

### 4.4 Where V-K's adapter is expected to differ from K4's (and why that is useful)
- **y_reference:** K4 uses (0,0,1) unless the member is parallel to Z; V-K uses the smallest-component axis. Twelve cases have a member parallel to Z. Local frames, and so p-level roundings, differ; convention-free results must agree within the predicate.
- **Mixed spring nodes:** the six RF-SKEW-{T,A}-CANT-AX-345 cases have one axis spring beside two skew ones at N0. K4 makes all three directional; V-K makes the axis one a global `Spring`. This exercises O1's spanning rule with a global-axis spring and 12 more structural zeros (96 in RF-SKEW, against 84).
- Zero load components omitted (if K4 kept them); station and spring ids; RF-MECH-K0 as K4.
- Derived quantities: V-K decides `Mb` exactly (no rounded magnitude, §6.2), and forms k_t, k_a with an exact pre-scaling where binary64 overflows or underflows (§5.2, C1).

## 5. The kernel lane

### 5.1 Per case
- `solve_case(source, CaseLimit::new(u64::MAX), &mut InvocationMeter::new(u64::MAX))`, one case per invocation (R1's cases share no stiffness). A limit that were ever reached is recorded and reported; it cannot be at u64::MAX.
- **Families in CI** (201 cases, 25,704 R1 rows, 677 controls): RF-CHAIN 30, RF-SKEW 36, RF-WEAK 9, RF-LARGE 12 (10 and 100 members), RF-INVARIANCE 25, RF-RANGE 32, RF-ZERO 4, RF-FINITE 6, RF-MECH 9, RF-CANCEL 38. **Examples:** RF-LARGE at 1,000 and 10,000 members (12 cases, 2,048 sampled rows, 38 controls).
- `needs_directional_spring` cases (22) run with `DirectionalSpring`.
- **Outcome rules:** every non-MECH case must be `Selected`; anything else is a failure of every row of the case. RF-MECH's eight must be `Refused` or `Unresolved` with **no rows**; the companion must be `Selected`. Any published row of a mechanism fails.

### 5.2 Observed and derived quantities
- Published rows by `QuantityId`: `Normal`/`Subnormal` values as published; `Underflow { negative }` observed as ±0 (its binary64 rounding); `Overflow` fails.
- **Magnitudes** `Mb`: never formed in binary64; the comparison is decided exactly on My² + Mz² (§6.2).
- **Twist and extension** (§4.10, F2): tw = fl(T/k_t), ext = fl(N/k_a), with k_t = fl(fl(G·J)/L), k_a = fl(fl(E·A)/L), L = FK's `norm` of the chord (`FK/lib.rs:1881`, `dot(v, v).sqrt()`), never differenced. **Exception (C1):** where fl(G·J) or fl(E·A) is not a normal number (RF-RANGE LEF-small: G·J = 2^-1078; LEF-large: 2^1122), the same two operations are applied to operands pre-scaled by an exact power of two, s chosen so every intermediate is normal, and the result is unscaled exactly. Where the design's formula is defined, s = 0 and the bits are identical.

### 5.3 The binary64 sparse gate (RF-MECH and RF-LARGE parity)
Read as §4.8's parity items 1–3 on these two families (C4):
- through FK's public pattern path (`assemble_sparse_stiffness`, `SparseStructuralSystem::new` with contributions and symmetry evidence, `SD::solve_sparse_structural`), built from the same adapted model as `FrameElement`s;
- **item 1:** bitwise K, pattern against dense assembly, at ≤ 100 members (and RF-MECH-DISC at 105 nodes);
- **item 2:** M03 outcome-class parity sparse against dense at ≤ 100 members; RF-MECH must be refused in both modes (no published solution), sparse only for LINE-IN-CHAIN1000 (host rule: dense stops at 1,000 members);
- **item 3:** sparse–dense displacement delta within the DEC-053 basis (1e-9 of the dense magnitude) at ≤ 100 members;
- in examples: sparse only at 1,000 and 10,000 members (no n² path; K6 measured dense at 1,000);
- the binary64 lane's values against R1 are **recorded, not gated** (W1 is the lane's method).

## 6. The comparison and report engine (`compare.rs`, `exact.rs`)

### 6.1 The predicate, decided exactly
`|obs − exp| ≤ 10⁻⁹·max(|exp|, scale)`, as exact rationals: obs is a binary64 (an exact dyadic), exp and scale are R1's decimal strings, 10⁻⁹ is the exact decimal. Everything is brought to integers by one common factor 2^a·10^b and compared with a std-only big integer (u32 limbs; parse, add, sub, mul, shift, compare). No rounding enters a decision. This differs from K4's f64 evaluation (`K4T/references_tests.rs`, `passes`) only within about 1e-7 of the threshold; any such row is listed.

### 6.2 Magnitudes exactly
For `Mb` with t = 10⁻⁹·max(exp, scale): pass iff My² + Mz² ≤ (exp + t)² and (exp − t ≤ 0 or My² + Mz² ≥ (exp − t)²), all exact. No `hypot` (the standing lesson; §4.10's `hypot(My, Mz)` is read as the Euclidean magnitude).

### 6.3 Outcomes and counts
Per comparison, exactly one of:
- `pass` (covered, predicate holds);
- `fail` (covered, predicate fails; blocks the gate; a stop at any checkpoint);
- `not_covered` (the scale is below R·S*, §7; the observed error and whether the predicate held are recorded; never a pass);
- `pass_absolute_range` (exp's binary64 rounding is ±0 or ±∞ while exp ≠ 0; covered; decided exactly; separately counted), or `fail`;
- `structural_zero` (§4.2: no published row by construction; exp must be exactly `0`, else `fail`).

The report prints passes, absolute-range passes and not-covered as three separate numbers (and structural zeros and failures beside them). A row-accounting assertion requires the five to sum to the case's row count, so no row can be dropped silently.

- **Represented basis:** selected by R1's `basis` field (`RF-SKEW-A-CANT-AX-122-r1e-12`, `RF-FINITE-THIRTIETHS-O1e6`); the generator asserts `basis == "represented"` iff `finite_input.exceeds_1e-9`, so "every `finite_input` case" and the two named cases coincide (checked: 2 of 208).
- **RF-CANCEL:** the row's recommended column is the binding scale (ROOT_RULINGS_V2 §1).
- **Projected CI pins** (to be confirmed at A1): passes 25,371; absolute-range 0; not_covered 51; structural zeros 282; failures 0. Per family: CHAIN 2,700 / 0 / 0 (+60); SKEW 982 / 0 / 2 (+96); WEAK 566 / 0 / 46 (+6); LARGE 9,054; INVARIANCE 7,056 (+40); RANGE 2,640 (+80); ZERO 158; FINITE 748; MECH 26 (8 refused); CANCEL 1,441 / 0 / 3. K4's six families reproduce K4's §12.2 counts except SKEW's 12 extra structural zeros.
- **Examples** (recorded at B): 2,048 rows; 5 absolute-range passes expected; 0 not_covered expected.

### 6.4 The absolute-range path in CI
RF-LARGE-CONT-n10000 runs only as an example, so CI's pinned absolute-range count is 0. The path is tested in CI by `engine.rs` with R1's five actual rows (`absolute_range_rows.json`): an `Underflow` of either sign and +0 pass; a normal value above 1e-9·scale fails; an `Overflow` fails; the rows are counted as absolute-range, never as passes.

## 7. The floor check and the committed `not_covered` list

### 7.1 The rule (`floor.rs`)
- **S\*** per connected body (members connect; springs and restraints do not; every compared R1 case is one body, asserted) and kind, from the reference values: S(kind) = max |fl(exp)| over the case's rows of that kind (R1's class with `@region` stripped: translation, rotation, force, moment). L_b and the coupling exactly as §4.1.6.1 items 5 and 6, in binary64 from the adapted coordinates. A test pins V-K's S\* bit for bit against K4's exported `coupled_scales` and `body_extent` on every case (an independent re-implementation cross-checked).
- **Variant F:** S\*_tw(m) = fl(mo/k_t), S\*_ext(m) = fl(fo/k_a), with §5.2's k_t and k_a.
- **Covered** iff max(|exp|, scale) ≥ R·S\* with R = 2^-34 (bits `0x3DD0000000000000`, pinned by a unit test), decided exactly. The exact comparison differs from one against fl(R·S\*) only where R·S\* is subnormal: RF-RANGE E+960 and F-960 (twist, extension) and THIN-B (force, moment), whose class scales lie below 2^-988 (checked). Their coverage is not near the boundary; recorded at A1.

### 7.2 The committed list
- `VR/cases/not_covered.json`: 51 entries (family, case, R1 key, kind, comparison scale, R·S\*), generated from `floor_kinds.json`'s variant-F lists (`F_rec` for RF-CANCEL): RF-WEAK 46, RF-CANCEL 3, RF-SKEW 2 (§4.10 `:905-910`; K4 §12.2). The generator also recomputes the rule in exact rationals and asserts equality with `floor_kinds.json` (two derivations, then Rust's third).
- **CI:** the not-covered set per family equals the committed list exactly. Any addition or removal fails the test and is a stop for ROOT's review.
- **n ≥ 1,000:** `floor_kinds.py` took S from R1's sampled rows, a lower bound ("S there is a lower bound", its docstring). V-K computes S\* both from the sample and from `s_full` (the complete solution from `--full`) and reports both; both are expected empty (C9).
- **Class correspondence** (the kernel analogue of §4.10's product-lane check, classes not bits): each not-covered comparison's row (both component rows for `Mb`; the T or N row for tw and ext) is `absolute_verified` or `Unpublishable` in K4's publication. This is V-K's addition; it is the kill for fault VK-F17 (ROOT confirms, Q4).

## 8. The discrimination check (`controls.rs`)
- **Value controls** (690 of the lane's 715 have a `values` map; 652 in CI): a control fails if any of its values fails the same exact predicate against its row (same scale column). A `discriminates: true` control that passes is **reported as non-discriminating, never dropped**, and is a stop. A `discriminates: false` control is counted (171 in CI) and listed; one that fails anyway is listed as a difference from R1's analysis.
- **Outcome controls** (25): RF-MECH's NC-REGULARIZED, NC-ZERO-RESIDUAL and NC-RESTRAINT-COUNT (any published solution) are discriminated by the refusal with no rows; NC-FALSE-MECHANISM (refusal of the companion) by the companion's selection.
- CI pins 506 discriminating controls (all fail), 171 non-discriminating (677 = 652 value + 25 outcome); examples 38, all discriminating.

## 9. RF-RANGE (§4.7, §4.10; stale item 10)
- All 32 cases run through W1. **LEF-small and LEF-large must be solved**; a named range refusal, and any other non-selection, is recorded as a failure of the case.
- **Whether W1's source admits LEF-small (prediction, confirmed at A1):** yes. `PrimitiveSource::new` decides zero length and degenerate axes exactly, with no axis tolerance (`K4R/source.rs:15-19`, `:406-416`); LEF-small's derived A ≈ 2^-408 and Iy, J ≈ 2^-815 are normal, so no `SubnormalDerivedPrimitive`; W1 forms G·J at p with a 64-bit exponent, and `assess_rigid_body` normalizes by the characteristic length (`FK/rigid_body.rs:33-66`). If the source refuses, V-K records the `SourceError` and reports it to ROOT at once; K4 is not changed.
- **For ROOT's context, not V-K's lane:** the ordinary route's 1e-12 m `AXIS_TOLERANCE` (`FK/lib.rs:24`) refuses every RF-RANGE vector with negative length exponent (L-240, LEF-small, SIM-a: 9 cases), not only LEF-small.
- The capture-boundary requests (2^53 − 1, 2^53, 1e16; §4.7, V1-S5) are product-lane only; R1 has none. They are V-P's.
- The harness's own twist derivation needs C1 for the LEF cases.

## 10. Seeded faults (Q4)

### 10.1 The feature
- `FK/../Cargo.toml`: `[features] mutation-controls = []`. Each site is `#[cfg(any(test, feature = "mutation-controls"))]`. VR's `seeded-faults` feature is the only thing that enables it, and only VR's mutation run passes `--features seeded-faults`.
- **With the feature off,** every site is compiled out of every product and dependency build. In FK's own test build (`cfg(test)`) the sites are present and inactive, so FK's suite is unchanged in effect. The reviewer checks both.

### 10.2 Selection without new API
- `K4R/seeded.rs` (cfg-gated, `pub(crate)`, declared in `K4R/mod.rs`): the active fault is read once (`OnceLock`) from the environment variable `FK_SEEDED_FAULT`. Unset means NONE; an unknown id panics, so a typo cannot pass as NONE.
- One process per fault, so a fault cannot leak into another. The FK site in `structural/sparse.rs` reaches it as `super::retained::seeded` (visible within `structural`).
- No `pub` item is added, even with the feature on. This needs ROOT's confirmation that the selector file and its `mod` line count as "cfg-gated fault sites" (Q8).

### 10.3 Placement rules
- No site inside `factor()`, `pivot_passes`, `negative_pair`, `solve_scaled` or `solve` (`ROOT_RULINGS_V1.md:1866`), nor inside `bound.rs`'s shifted-factorization loop.
- Sites sit at function entries and exits or at a single decision.
- Before any site is written, run K4's source scan (`adaptive_tests.rs`, `k4s_files_call_no_binary64_transcendental_or_fused_function`) and `tests/s11_site_table.rs` on the new text. A scan that flags a site is a stop (those tests are outside the write set).

### 10.4 The list

| Id | Maps to | Site | Fault | Expected to fail | Kill kind |
|---|---|---|---|---|---|
| VK-F01 | §7.3-1 | `K4R/assemble.rs`, after each K entry is rounded to p | entry := lift(fl64(entry)): the stored binary64 K promoted | RF-CHAIN r1e-08…12 root rows (R1's NC-STORED-ASSEMBLY), RF-SKEW r1e-08/12, RF-WEAK; or unresolved via W | value or outcome |
| VK-F02 | §7.3-2 | `K4R/recover.rs`, recovery entry | u rounded to binary64 before recovery | RF-CHAIN-T/A r1e-10/12 T, N, tw, ext (NC-SUBTRACT-ROUNDED class) | value or outcome |
| VK-F03 | §7.3-4 (a contribution omitted) | `K4R/assemble.rs`, per-entry exact sum | the smallest contribution to each diagonal entry dropped (R1's NC-LOST-SOFT) | RF-CHAIN and RF-SKEW soft roots: unresolved or wrong | value or outcome |
| VK-F04 | §7.3-4 (an axis in B) | `K4R/assemble.rs`, member frame | e_x's first two components swapped (R1's NC-WRONG-TRANSFORM) | RF-SKEW, RF-INVARIANCE-ROT, RF-LARGE-ROT, RF-WEAK-W-L | value |
| VK-F06 | §7.3-6 | `K4R/adaptive.rs`, the decision | candidate accepted without the 2p verification | attempts and verification precision against §12's committed records | evidence |
| VK-F07 | §7.3-7 | `K4R/factor.rs`, the geometry-first function | every body treated as NumericallyUnresolved | RF-MECH: `Refused` → `Unresolved` against the records; value only if a mechanism is published | evidence |
| VK-F08 | §7.3-8 | `FK/structural/sparse.rs`, `assemble_sparse_stiffness` | member 1's i–j coupling block left out of the pattern (sparse mode only) | §5.3 items 1–3 on RF-LARGE ≤ 100 | parity |
| VK-F10 | §7.3-10 (list-order form) | `K4R/source.rs`, `PrimitiveSource::new` | canonical sort skipped (caller's order kept) | the list-permutation test (§Q7): K4SRC and publication bits differ | bit |
| VK-F13 | §7.3-13 | `K4R/ledger.rs` | per-DOF net folded at p in listed order, not the exact sum | RF-CANCEL G1e80 GnG and nGG net-governed rows (NC-FLOAT-SUM) | value |
| VK-F17 | §7.3-17 | `K4R/adaptive.rs`, classification | every scaled row `relative_verified` | §7.2's class correspondence on the 51 | class |
| VK-R02 | R7-M2 | `K4R/verify.rs`, `e_hat` | ê uncoupled | RF-LARGE-TREE-n00100-AX unresolved (R7 §7) | outcome |
| VK-R28 | R7-M28 | `K4R/bound.rs`, the shift decision (not the loop) | no shift (Uc alone) | RF-LARGE-CHAIN-n00100-AX at 512, TREE-AX at 256 (R7 §7), against the records | evidence (availability) |
| VK-S1 | R1 directional | `K4R/assemble.rs`, directional block | k·n·nᵀ without the division by nᵀn | every directional case with \|d\|² ≠ 1 (22 cases) | value |
| VK-S2 | R1's NC-SIGN | `K4R/recover.rs`, reactions | reactions with the opposite sign | every case with a nonzero reaction | value |

**Not seeded, with the reason (for ROOT; C2, C3):**
- **§7.3-5 (no escalation):** K4 selects all 120 of its solved R1 cases at 128 (K4 §12.2), so the fault changes nothing on them. It is seeded only if A1 finds a lane case selected above 128; otherwise it is reported as undiscriminated by R1 (§7.3: "its case set is extended"), which V-K cannot do (R1 is read-only).
- **§7.3-10, relabelling half:** W1's answer does not depend on the order within its claim, so no 1e-9 comparison can see it. The list-order half is VK-F10.
- **§7.3-16:** R1's smallest ratio is 1e-12, far from the 2^-300 the control needs; K4's DUPLICATE control kills it.
- **§7.3-3:** R1 has no nonzero prescribed motion.
- **§7.3-9, 11, 12, 14, 15, 18–20 and 23–32:** outside the kernel lane (W2, the facade, combinations, K2a, S11, D2 readers, D-5/K-D5).
- **§7.3-21 and 22** are harness mutants (§14.3).
- **R7's other mutants** need constructed controls outside R1; K4's controls kill them.

### 10.5 The kill matrix
- `VR/runner/run_seeded_faults.py` (stdlib) builds once in `<wt>/vk-mut/` with `--features seeded-faults` (`-j 4`), then runs the VR test binary once per fault with `FK_SEEDED_FAULT=<id>`, NONE first (which must pass everything).
- The lane tests print one JSON line of failures per test (family, case, key, kind of kill).
- The matrix `VR/observations/seeded/kill_matrix.jsonl` records, per fault, the killing tests, comparisons and kill kind.
- A fault with no kill is a stop.

### 10.6 The feature guard (CI-run, in VR: `VR/tests/feature_guard.rs`)
It walks every `Cargo.toml` under `P/` (not `target/`, not `execution/`; 41 manifests with VR, 40 today) and asserts:
- the string `mutation-controls` occurs only in FK's own `[features]` table (as a key, not in `default`) and in VR's `[features]` table (as the value of `seeded-faults`, not in `default`);
- no `[dependencies]`/`[dev-dependencies]` table anywhere names it;
- `check_release_readiness.py`'s cargo command carries no `--features`/`--all-features`;
- no VR Rust source names a path into `execution/`.

Self-tests on synthetic manifest strings kill the guard's own mutants.

## 11. RCM equality (K4's brief Q7; stale item 4)
- **The same adjacency to both:** K4's exported `retained::factor::reverse_cuthill_mckee` (`K4R/factor.rs:290`, returns `Vec<usize>`, requires in-range input) and SD's `reverse_cuthill_mckee` (`SD/lib.rs:505`, `Result`).
- V-K builds the free–free DOF adjacency itself (member 12×12 blocks, spring diagonals, directional 3×3 node blocks, restrained DOFs removed): every CI model, and the 12 large ones in `vk_scale`.
- **Also seeded synthetic graphs:** paths, stars, grids, several components, isolated nodes, duplicate and self edges.
- The orders must be equal element for element. Inputs out of range are not compared (K4's port asserts range only in debug).

## 12. Per-case records (Scope 11)
- **Schema** `vk-case-record-v1`, one object per case, in `VR/observations/kernel_lane/<family>.json`:
  - id, family, `k4src_sha256`;
  - the outcome (Selected p | Refused reason | Unresolved reason), the selected and verification precisions;
  - `attempts[]`: precision, role, outcome and reason, corrections, residual basis, `GateTest`; work = `AttemptWork` total and per `WidthWork`; `SumWork`; all 19 `StageWork` fields; `shared_work`, `shared_stages`, `shared_built_here`, `stop_rule_work`, `verification_work`, `verification_shared_work`; `StorageCounts`;
  - the report counts; geometry.
  - Floats are written as bits; there is no time field.
- **CI:** each family's lane test regenerates its records in memory and requires equality with the committed file. The method evidence (outcome, precisions, attempts) is part of "what is compared" (Scope 4). A difference is a finding; `vk_records --write` regenerates only by explicit decision. The records are deterministic integer counts (K4's `golden_work_counts` runs on both Linux CI and the Mac).
- **Time** is an observation only: `vk_records --timed` (release, one process under K6's `launch`, load and `memorystatus` before and after) writes `VR/observations/kernel_lane_timed.json`, never compared.
- **RETURN's summaries** (per family: the outcomes and selected precisions; the distribution, maximum and cases at the maximum of work by stage and precision) are computed from the committed records.

## 13. Scale runs (Q5)

### 13.1 How they run
- `vk_scale` (example, `--release`, from a `git archive` of the exact commit into `<scratch>`), one process per model. It:
  - reads the model file written by `gen_vk_cases.py --large`, checks its sha256 and its K4SRC sha256;
  - runs W1 once, then §6–§8's report, RCM equality and the binary64 sparse lane;
  - writes the per-case record with wall time.
- **Protection:**
  - a counting global allocator in the example only, with the heap cap C − 512 MiB, aborting on refusal;
  - `VR/runner/vk_scale_runner.py` imports K6's runner by path and calls its `launch()` (`H/runner/k6_runner.py:933`) for `/usr/bin/time -l`, the 100 ms RSS watchdog on the binary's pid, process-group SIGKILL, load and `memorystatus`;
  - K6's `admission()` rule, with V-K's estimate;
  - C = 8 GiB.
- **One run per model,** no repeats. V-K makes no timing claim.
- **Ascent** per (family, orientation): 100 (release, measured first, for the ratio) → 1,000 → 10,000. **10,000 only as ROOT approves.**
- `<wt>/guard/memguard.log` is checked after each tier.

### 13.2 Estimate and schedule (provisional)
- **The estimate** E_W1 ≈ 96·P + 144·F + 5,760·m + 960·DOFs bytes, derived from the storage alive during a 256 verification of a 128 candidate:
  - two K states at L = 4 (48 B per `Wide<4>`, `K4R/wide.rs:200`) over P pattern entries;
  - three RCM profiles (both factors and the shift's copy) over F entries;
  - the contributions;
  - per-DOF vectors.
- **With escalation to 512 and 1024,** add 224·(P + F).
- It is replaced by K6b's derived E_W1, or by V-K's own count from the code at A2, before B.
- P and F come from K6 `RETURN.md` §6.1.

| Model (n = 1,000 / 10,000) | P | F (RCM) | E_W1 128/256 | with 512+1024 | ×2 ≤ C/2 = 4 GiB | projected release time |
|---|---|---|---|---|---|---|
| CHAIN-AX | 108,036 / 1,080,036 | 17,988 / 179,988 | 23 / 234 MiB | 50 / 503 MiB | admitted | ~0.5–2 s / 5–20 s |
| CHAIN-ROT | same | 55,969 / 559,969 | 29 / 286 MiB | 64 / 636 MiB | admitted | ~1–3 s / 10–35 s |
| TREE-AX | same | 49,933 / 499,933 | 28 / 277 MiB | 62 / 615 MiB | admitted | ~0.5–2 s / 6–20 s |
| TREE-ROT | same | 67,915 / 679,915 | 30 / 302 MiB | 68 / 678 MiB | admitted | ~1–3 s / 10–35 s |
| CONT-AX | same | 10,493 / 104,993 | 22 / 223 MiB | 48 / 476 MiB | admitted | ~0.5–2 s / 5–15 s |
| CONT-ROT | same | 30,474 / 304,974 | 25 / 251 MiB | 55 / 547 MiB | admitted | ~0.5–2 s / 8–25 s |

- **Times** scale K4's debug figures at 100 members (1.4–3.2 s, K4 `_run_records/c/results.jsonl`) by an assumed 10–30× release speed-up and linear growth (RCM half-bandwidth 3–19, bounded). Every stage of W1 (factor, Uc, est, shift, recovery) is linear in the profile for bounded bandwidth. Escalation would multiply them by about 3–10.
- **Slot:**
  - the 1,000s: under 1 min;
  - the 10,000s: about 1–4 min, 30 min worst case with escalation;
  - I ask for a 30-minute slot per tier. If a 1,000-member run exceeds 10× its projection, I stop and report before 10,000.
- **`--large` generation** (Python exact solves, about 10 min) runs outside K6b's timed slots.

## 14. Tests, projected CI time and the mutants

### 14.1 CI tests (debug, `cargo test --offline --locked`)
- **`adapter.rs`:** K4SRC sha256 of all 201 cases against path B; `SHA256SUMS`; SHA-256 known answers; source-encoding determinism.
- **`lane_<family>.rs`** (10 tests, parallel):
  - outcomes;
  - the family's pinned counts (§6.3) and zero failures;
  - the row accounting;
  - the not-covered set against the committed list;
  - the discrimination pins;
  - the class correspondence;
  - equality with the committed records;
  - RF-MECH's refusals with no rows, and K0's literal-model `NonPositiveSpring`.
- **`engine.rs`:**
  - the exact predicate at its boundary (equality and one ulp either side), including the magnitude form and the represented basis;
  - the absolute-range path on R1's five rows;
  - the parsers (decimal, bits).
- **`floor.rs`:** R's bits; V-K's S\* equal to K4's exported `coupled_scales`/`body_extent` on every case; variant F with §5.2's k_t and k_a; the exact boundary.
- **`controls.rs`:** value and outcome controls.
- **`invariance.rs`** (Q7): the list-permutation identity; the cross-variant observations (recorded, not asserted).
- **`parity.rs`:** §5.3 items 1–3 at ≤ 100 members; RF-MECH in both modes.
- **`rcm.rs`:** §11.
- **`records.rs`:** the schema; determinism of two runs.
- **`feature_guard.rs`:** §10.6.
- **Only under the feature:** `seeded_faults.rs` (the selector's NONE and unknown-id behaviour).

### 14.2 Projected time (measured at A1; reported if VR adds more than 3 min to CI)
- **Tests:**
  - K4's lane of 128 small cases took 6.5 s at debug;
  - RF-LARGE-100 takes 1.4–3.2 s each (six);
  - RF-INVARIANCE-TREE100 takes about 2–3 s each (four);
  - the rest is under 5 s;
  - exact comparisons of about 26,000 rows take under 2 s.
  - Total ≈ 50 s CPU, **≈ 20–40 s wall at 4 threads**, ≤ 60 s at 2.
- **Build of VR's test profile** from a fresh target (FK, SD, serde_json, VR): ≈ 60–90 s.
- **Added to CI's numerical job:** ≈ 2–3 min, against 18.3–20.1 min at K4's merge (budget 45 min, `.github/workflows/piping-desktop-e2e.yml:176`).
- **The mutation run** (not CI): one build plus 15 runs of ≈ 20–40 s, about 10 min.

### 14.3 Mutants of the harness (checkpoint C; NONE first; one clean copy and target each under `<wt>/vk-mut/`, at most three at `-j 4`)

| # | Mutant | Expected kill |
|---|---|---|
| VK-H0 | NONE | — |
| VK-H1 | a comparison dropped (every `ext.*` row skipped) | row accounting; the family pins |
| VK-H2 | RF-CANCEL on the class scale, not the recommended column | the not-covered list (3 → 0); CANCEL controls pass (discrimination) |
| VK-H3 | `not_covered` counted as `pass` (§7.3-22) | the pins; the list equality |
| VK-H4 | R = 10⁹·2⁻⁶⁴ | not by the list (identical by §4.10's statement); by the R-bits unit test |
| VK-H5 | tw and ext differenced from rotations and translations (§7.3-21) | RF-CHAIN N05-class twists fail |
| VK-H6 | max(\|obs\|, scale) in place of max(\|exp\|, scale) | engine boundary tests |
| VK-H7 | magnitude: upper bound only | engine; Mb rows with obs = 0 |
| VK-H8 | represented basis ignored | RF-SKEW-A-CANT-AX-122-r1e-12 fails by input rounding (R1: 8.4e-9) |
| VK-H9 | absolute-range rows counted as passes | engine; the three-count pin |
| VK-H10 | adapter: node order swapped / a member dropped / a section property wrong / y_ref rule changed | K4SRC sha256 |
| VK-H11 | a discriminating control dropped when it passes | the controls pins |
| VK-H12 | feature guard ignores `[dependencies]` features | its synthetic self-test |

## 15. Open questions, with options and a recommendation

**Q1. `product_physics` now, or added by V-P.**
- (a) **Now,** as §4.10's list says. It is unused in the kernel lane. It adds PP's whole dependency closure (about 20 in-repo crates plus its registry packages) to VR's own target, compiled fresh per CI run: an estimated +1.5–3 min debug build per numerical job, and a larger `Cargo.lock`. It puts VR among PP's dependants, although nothing calls PP.
- (b) **V-P adds it** with its first product-lane request (V-P's write set is `numerical_robustness/**`).
- **Recommendation: (b).** V-K has no product caller (brief: "No product-lane run"), and an unused dependency costs CI time for nothing. I can measure both builds from a `git archive` at A1 if ROOT wants the figure.

**Q2. How VR carries R1's cases and expected values.**
- (a) **A generated, committed cases file.** R1's sha256 is pinned, with `--check` regeneration (K4's precedent). Model inputs are carried as bits; every expected value, scale and control is R1's decimal string verbatim. A std-only exact engine decides every comparison, so the sub-range decimals need no special carrier. `k4src_sha256` from an independent `--model` path is checked in CI; large models are generated on demand with committed sha256.
- (b) R1's trimmed JSON is committed and adapted in Rust. The exact section (PI_Q, rational rounding) would then need a Rust bignum rational divider. It gives more independence from Python but more code, and there is still no CI access to `--model`.
- (c) As K4: expected values rounded to binary64 in a text file. Sub-range values cannot be carried (they round to 0), and decisions round.
- **Recommendation: (a),** as §3–§4 specify (about 2.7 MB committed across 10 files; large models not committed).

**Q3. Which comparisons are CI tests and which are examples.**
- (a) **CI:** all 201 cases at ≤ 100 members (LINE-IN-CHAIN1000's 1,005 members included: refused by geometry before any factor), the binary64 parity at ≤ 100, RCM, floor, controls, records and the guard. **Examples:** RF-LARGE at 1,000 and 10,000.
- (b) Add RF-LARGE at 1,000 to CI. That breaks the host rule ("Debug tests stay at 100 members or fewer", K6 and K6b) and adds an estimated 5–15 min debug time.
- **Recommendation: (a).** Projected: ≈ 20–40 s of tests at 4 threads plus a ≈ 60–90 s build; ≈ 2–3 min added to the 45-minute job (§14.2).

**Q4. The seeded-fault list.**
- (a) **§10.4's 14 faults,** with value, outcome, parity, class and evidence kills declared per fault.
  - §7.3-5 is seeded only if a lane case is selected above 128.
  - VK-F06, VK-F07 and VK-R28 are evidence-level kills against the committed records. §4.10 accepts both `Refused` and `Unresolved` for RF-MECH, so §7.3-7's "RF-MECH is then recovered" does not occur as the design predicts.
  - VK-F17's kill needs §7.2's class correspondence.
- (b) Value kills only. VK-F06, VK-F07, VK-R28 and VK-F17 are dropped as undiscriminated by R1 and reported.
- (c) (a), plus one non-R1 control for §7.3-5 and 6 (for example K4's k = 1e-28 model), as a fault-kill control only, outside the gate.
- **Recommendation: (a).** Evidence-level kills are accepted under O5's two conditions: the kill is on returned evidence, and the published-value equivalence is derived in RETURN. §7.3-5 is reported as a finding if R1 cannot discriminate it. ROOT rules on (c).

**Q5. The scale-run schedule.**
- (a) **§13:** V-K's own models, one release process per model under K6's `launch()` and `admission()`, heap cap C − 512 MiB, ascent 100 → 1,000 → 10,000. The 10,000s only as ROOT approves. After K6b's W1 runs at the same size, so K6b's measured heap ratio feeds admission.
- (b) Reuse K6b's W1 processes and post-process their published rows. K6b's models come from K6's generator with explicit products and `std::f64::consts::PI` (K6 Q10), so A, I and J may differ from fl(exact) in the last bit. Its rows are then not V-K's adapted models, and K6b does not dump rows.
- **Recommendation: (a).** One slot of about 30 min per tier: 12 runs, each projected at seconds to half a minute. K6b's W1 comparisons at 1,000 and 10,000 overlap V-K's (C5). ROOT may drop one of them.

**Q6. How the adapter is checked independently of `K4T/models.rs`, and where they differ.**
- (a) **§4.3:**
  - a CI byte check of every Rust-built source against a separate `--model` path;
  - the generator's `--check`;
  - a one-off, recorded input-bit comparison with K4's committed `r1_cases.txt`/`r1_large.txt`, in which every numeric input must be equal and the §4.4 differences are expected (y_reference; the six mixed spring nodes; omitted zero loads; ids; derived-quantity methods);
  - the reviewer's own check.
- (b) CI compares V-K's models with K4T's files directly. That couples VR's CI to K4's test fixtures.
- **Recommendation: (a).**

**Q7. RF-INVARIANCE: what "the same answers" means per quantity.**
- (a) **The gate:** each of the 25 variants against its own R1 expectations. R1 derived every transformed value exactly and asserted it equal to a direct solve (README §5), and §4.8 item 6 says "the same answers against the references".
  - Recorded, not asserted, per quantity and variant, in predicate units:
    - OFF: every row equal to BASE's;
    - ROT (Q3, Q9): u, θ, R and S rotate by the exact rational Q (applied exactly to BASE's rows); N, T, Mb, tw and ext unchanged;
    - RELABEL: nodes and members mapped by name, member direction reversed, so N, T, tw and ext are unchanged and Mb.i ↔ Mb.j;
    - UNITS-mm: lengths and moments ×1,000, forces and rotations unchanged.
  - Bit identity is not claimed across relabelling: RCM's tie-breaks follow labels, so p-level roundings may differ.
  - **Asserted exactly:** a V-K list permutation (members, springs, loads, constraints and stations reordered, labels kept) gives identical K4SRC bytes, publication bits and evidence (K4's canonical sort; VK-F10 targets it).
- (b) Assert bit identity across OFF and RELABEL. It is not guaranteed by W1: false failures.
- **Recommendation: (a).**

**Q8. What V-K needs beyond the export list.**
- **Fields and methods, not only type names.** V-K constructs `SourceParts` and its element structs (`StraightMember`, `Spring`, `DirectionalSpring`, `Constraint`, `NodalLoad`, `Station`, `Dof`). It reads `Publication.rows`, `PublishedRow`'s five fields, `RetainedEvidence`'s fields, `AttemptRecord`, `StageWork`, `StorageCounts`, `AttemptWork`/`WidthWork`/`SumWork`, and the methods `Binary64Outcome::value`, `Component::{index, from_index, ALL}` and `Dof::{global, from_global}`. §16 lists them as `pub(crate)` fields.
  - (a) A0 raises the listed items **with their fields and those methods.**
  - (b) A0 raises type names only. V-K then cannot build a source; a stop.
  - **Recommendation: (a)**, confirmed with I16 before A0 is committed.
- **The fault selector** `K4R/seeded.rs` and its cfg-gated `mod` line (§10.2): no `pub` item.
  - (a) Accepted as part of "the cfg-gated fault sites".
  - (b) A cfg-gated `pub` selector for in-process switching.
  - **Recommendation: (a).**
- **Nothing else.** FK's and SD's public binary64 API covers §5.3. `reverse_cuthill_mckee` is in the list.

## 16. Conflicts found between the brief, the design and the code

- **C1.** §4.10's derived twist and extension use k_t = fl(fl(G·J)/L) and k_a = fl(fl(E·A)/L), and §4.1.6.1 item 7 uses fl(mo/k_t). For RF-RANGE LEF-small, fl(G·J) (2^-1078) rounds to 0, so k_t = 0; for LEF-large, fl(G·J) (2^1122) overflows. The harness cannot derive tw or its floor scale there as written. Proposed: the exact power-of-two pre-scaling of §5.2 (bit-identical wherever the design's formula is defined). ROOT rules.
- **C2.** §7.3 item 5 cannot be killed by any R1 comparison if every lane case selects at 128, which is K4's result for its R1 cases. Neither can the relabelling half of item 10, since W1's answer does not depend on the order within its claim. §7.3 says the case set is then extended, but R1 is read-only and §4.10's lane is R1's. See Q4.
- **C3.** §7.3 item 7 predicts "RF-MECH is then recovered", but disabling geometry makes the mechanisms fail the pivot screen and end `Unresolved`, which §4.10 accepts. The kill is evidence-level only (the outcome against the records). The same holds for items 6 and R7-M28. See Q4.
- **C4.** Scope 3's "binary64 sparse gate for the parity checks" does not name the parity items. V-K reads them as §4.8 items 1–3 on RF-MECH and RF-LARGE at ≤ 100 members, with sparse only above that (§5.3). K6 already recorded binary64 at every size on its own models.
- **C5.** Scope 12 overlaps K6b Scope 3/4. Both run W1 on RF-LARGE at 1,000 (and 10,000 as approved) and compare with R1. See Q5.
- **C6.** K4 `RETURN.md` §16's export list names types whose fields are `pub(crate)`. A0 raising names only would not let any consumer outside FK build a source. See Q8.
- **C7.** Scope 9's `cfg(any(test, …))` compiles the sites into FK's own test build, so FK's source-text scans (K4's no-transcendental scan, the S11 site table) see them. ROOT's `:1866` ruling also fixes five `factor.rs` functions byte for byte. Sites are placed accordingly and the scans are run before writing (§10.3).
- **C8.** §4.10's "`hypot(My, Mz)`" conflicts with the standing no-libm lesson as written. V-K decides the magnitude comparison exactly (§6.2).
- **C9.** §4.10's enumerated list was computed from R1's sampled rows at n ≥ 1,000 (a lower bound on S, per `floor_kinds.py`), while §4.1.6.1's S is over all rows. V-K reports both; both are expected empty.
- **C10.** R1's RF-MECH-K0 has a k = 0 spring, which K4's source refuses (`NonPositiveSpring`). That is neither "refused with a witness" nor "unresolved". V-K omits the zero spring (the case is then refused by geometry) and records the literal refusal, as K4 did. ROOT confirms.
- **C11** (note). Scope 6's `pass_absolute_range` case (CONT-n10000) is an example, so CI's pinned absolute-range count is 0. The path is CI-tested on R1's five rows (§6.4).
- **C12** (note, for ROOT's context). The ordinary route's 1e-12 m `AXIS_TOLERANCE` refuses nine RF-RANGE cases (L-240, LEF-small and SIM-a on the three bases), not only LEF-small (`ROOT_RULINGS_V1.md:742` names LEF-small only). V-K's W1 lane is unaffected.

## 17. Host and write set
- **Host:** `I8R_K1_RESUME.md:24-50` and `I15_K6_IMPLEMENTATION.md` §6:
  - `RUSTUP_TOOLCHAIN=1.97.1`, `RUSTUP_AUTO_INSTALL=0`, `CARGO_INCREMENTAL=0`, `--offline --locked`;
  - `-j 8` at most, `RUST_TEST_THREADS=4`, at most two cargo jobs of my own;
  - target `<wt>/vk-target`, scratch `<wt>/scratch/i17`, mutants `<wt>/vk-mut/`;
  - nothing heavy during K6b's timed slots; scale runs only in slots ROOT grants;
  - the memory guard's log checked after heavy phases;
  - no dense matrix at 1,000 members or more in any test, and none at 10,000 anywhere.
- **Write set:**
  - `VR/**`;
  - `FK/../Cargo.toml` (the feature);
  - the cfg-gated sites in `K4R` (`assemble.rs`, `recover.rs`, `adaptive.rs`, `factor.rs` outside the five functions, `source.rs`, `ledger.rs`, `verify.rs`, `bound.rs` outside the loop), `K4R/seeded.rs` and its `mod` line (Q8), and `FK/structural/sparse.rs`;
  - `T3/IMPLEMENTATION/VK/`.
  - R1 is read-only.
- **Checkpoint A1** delivers §2–§9 and §11–§12 at CI scale with the report. **A2** delivers §10 and the guard.
