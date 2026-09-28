# I12: implement slice K4 (the W1a kernel method: source, ledger, assembly, factor, recovery, combinations and the adaptive schedule)

> This is an implementation TASK. Read Root `AGENTS.md`, `agents/AGENT_TASK.md` and `_COMMON.md` first. The Mac host rules in `I8R_K1_RESUME.md` ("The Mac host") override `_COMMON.md`'s host section, and apply to you in full, with K4's paths below in place of K1's.

## Roles

- ROOT (HELP_HUMAN) dispatches you directly, as a background subagent, and is your return path. There is no separate T3 manager on the Mac.
- Make no Git writes and no index operations. ROOT commits.
- Record the delegation mechanism in RETURN.

## Purpose

K4 is the §6 row "K4: W1a kernel method | after K1 and K3 | new files under `FK/structural/retained/` only (`source`, `ledger`, `assemble`, `factor`, `recover`, `combine`, `adaptive`), declared from `retained/mod.rs` | N05, N06, NP-A (intended), R1's discriminating RF-CHAIN, RF-SKEW, RF-WEAK and RF-FINITE (represented basis where marked), RF-MECH, RF-CANCEL, the k = 1e-28 case, the B1 and S8 controls (§7.3), the published-row S\* and classification (§4.1.6.1), the exact-block oracle in scope, and mutation controls".
- K1 (PR #1034) and K3 (PR #1041, main `57617b0fb`) have merged, so K4 can start. The selected order is "the rest of K3 runs in parallel after K3a, then K4, then K6 and V-K" (`ROOT_SELECTION_DESIGNS.md`, Selected item 2).
- **Coverage** is W1a: "straight frames, global-axis linear springs, rigid restraints, nodal loads, and combinations through T0R's gates" (§1). At kernel level W1a also covers nonzero prescribed support motion (§4.2: "kernel yes").

**What W1 is for, and why nothing calls it yet** (context; none of this is K4's work):
- The method "runs automatically, per case, on the ordinary route and on the exact route, whenever the ordinary M03 outcome is not `Passed`" (§1; the trigger list is §4.3).
- For Passed cases, the formation check "demotes Passed to Sensitive in K-D5; from F2a it routes the case to W1" (§4.3, §4.3.1).
- "F2a wires W1 and the identities and retires nothing" (§4.4.1). Until a family retires at F2b, "W1 is not attempted in any invocation in which exact-block selects a case" (§4.4, coexistence rule).
- A case with any nonlinear support "is not selected for W1 … no proof, no successor identity, no `RETAINED_PRECISION_UNAVAILABLE` refusal of its result" (§4.3, revision 5a.1; ROOT's "D2 revision 5b choices" item 2). The kernel has no nonlinear family, so such a case cannot be constructed. The selection rule is F2a's. K4 must not add any refusal reason for it.

So K4 builds the kernel method that F2a's routing and trigger will call, with no product caller.

### What K4 builds on (merged)

- **K3a** (PR #983): `Wide<2>` with correctly rounded + − × ÷ √ at p ≤ 128, the split, the arctangent and `WorkCounter`. It is on the product path through K-D5, so its results, `Debug` tokens and `WideError` `Display` strings are a published-byte surface (I11's brief, "Scope").
- **K3** (PR #1041): `Wide<L>` at L = 4, 8 and 16 in `retained/wide/multi.rs`. It provides `WideContext<L>` (+ − × ÷ √ at 2 ≤ p ≤ 64L), `to_binary64() -> Binary64Outcome`, `widen`, `round::<M>`, `two_sum`, `two_product`, `from_integer`, and `WidthWork`/`AttemptWork` in limb-multiply equivalents.
  - The exact signatures are in K3's `RETURN.md` §14.
  - **The widths per precision** (ROOT's K3 checkpoint-0 ruling 8; K3 RETURN §15): "128, 192 and 256 at L = 4; 320 and 512 at L = 8; 576 and 1024 at L = 16".
- **K1** (PR #1034): `SparsePattern` (`from_connectivity`, `from_positions`; K1 RETURN §12). This is the structure W1's factor runs on: "a sparse profile LDLᵀ at precision p, on the same sparse structure W3 introduces" (§1 item 2).
- **S11-K**: `ExactAccumulator` (`FK/exact_sum.rs`) and the load ledger (`FK/load_ledger.rs`: `ForceTerm`, `accumulate`, `accumulate_dof`).
- **K-D5**: the `Wide<2>` frame re-formation in `formation_check.rs`. K4 uses it only as an independent cross-check of its own formation.

### What K4 adds (D1 §4.1; re-locate every citation on your base)

Proposed names are placeholders for ROOT (§4.1): method token `contribution_preserving_multiprecision_v1`, policy `M03-INTEGRITY-MP-v1`, and diagnostics `RETAINED_PRECISION_SELECTED` and `RETAINED_PRECISION_UNAVAILABLE`. K4 emits no diagnostic. It returns kernel outcomes that F2a maps.

1. **`source.rs`: `PrimitiveSource`**, "an immutable per-case declaration with an identity digest" (§4.1.1):
   - node coordinates (binary64);
   - `StraightMember {node_i, node_j, E, G, A, Iy, Iz, J, y_reference}`, all binary64 operands;
   - springs `(dof, k > 0)`;
   - constraints `(dof, value)`;
   - nodal loads `(dof, value, source_id)`;
   - **subject to Q6:** a kernel-only directional spring.
   - The rules: "Only supported families can be constructed, so exclusion is a type-level fact". "Validation rejects nonfinite or nonpositive properties and incomplete partitions. It also rejects any derived primitive that is subnormal".
   - The identity (digest or canonical encoding) is decided under Q10.
2. **`ledger.rs`: the load ledger at p** (§4.1.2 item 5). "Each DOF's load is the exact sum of its identified contributions, rounded once to p". It is "the same ledger S11-F introduces for the ordinary route … with the same contribution granularity". "`ExactAccumulator` holds the exact sum and gains one further projection, to `Wide<L>` at p".
   - The projection is K3's `from_integer` fed by the `exact_sum.rs` accessor (Q3).
   - **Wherever the ledger joins another sum** (a prescribed-coupled right-hand side, a residual, a reaction), it enters that sum **exactly**, never as a value already rounded to p. Otherwise the sum is rounded twice.
3. **The correctly rounded exact multi-term sum** (RV12's S1; K3 RETURN addendum 2; home per Q2). This is the primitive behind D1's exact-sum rule: "Every multi-term sum outside the factorization and the triangular solves is formed exactly and rounded once" (§4.1.2). **Contract:**
   - Its terms are p-bit `Wide` values, exact products of two p-bit values, binary64 values, and the ledger's exact net.
   - The exact sum is rounded **once** to the target p, to nearest with ties to even. Ties are decided by the whole tail, however far down it lies.
   - An exact zero is +0 ("An exact zero is +0.0", §4.1.2).
   - An exponent or span it cannot hold is refused, never truncated or wrapped.
   - There is no heap allocation per term.
   - Every operation is charged to the attempt's work.
4. **`assemble.rs`: formation, assembly and reduction at p** (§4.1.2 items 1–4 and 6):
   - The frame: `d = x_j − x_i`, `L = √(d·d)`, `e_x = d/L`, Gram-Schmidt of `y_reference`, "No axis tolerance is used at p".
   - `B_local` (6×12) and `D` (6×6); `K_e = Bᵀ D B`.
   - "Each pattern entry of K is formed as one exact expansion of its p-bit element contributions and binary64 spring stiffnesses … rounded once to p", on K1's `SparsePattern`.
   - "Each `rhs_i` is one exact expansion: the ledger terms plus the exact products `−K_ic·u_c` … rounded once to p. Neither K nor rhs is ever rounded back to binary64."
5. **`factor.rs`: factor, screens and mechanism handling** (§4.1.3):
   - "Geometry first": FK's `assess_rigid_body` runs per connected body before any factor. "A witnessed mechanism is refused and never escalated."
   - The ordering (Q7), then a profile LDLᵀ at p.
   - The pivot screen `d_i > 64·γ_p(m_i)·c_i`, with `γ_p(m) = m·2^-p/(1 − m·2^-p)`. "A failed pivot at p escalates to the next p … It is never read as a mechanism."
   - Negative energy "for pattern pairs only, at p, against the intended K".
   - Hager–Higham with the p-factor. "`rcond ≤ 2^-(p-1)` counts as unresolved at p and escalates." It is published as model information, with the label "sensitivity to matrix-entry perturbation, not to authored parameters".
6. **Solve and refinement** (§4.1.4; in `factor.rs` or `adaptive.rs`):
   - "Evaluate `r = f − K u` against the intended system re-formed at p + 64".
   - "Each `r_i` is one exact expansion of the p-bit products and the ledger terms".
   - Each free row is gated with the componentwise guarded ratio against `64·γ_p(m_i)`.
   - At most three corrections; then escalate.
   - The ceiling's solve is ruled under Q4.
7. **`recover.rs`: recovery before rounding** (§4.1.5):
   - `d_local = T u`, `e = B_local d_local`, `Q = D e`, and end actions `B_localᵀ Q`;
   - station actions, which are linear for nodal loads;
   - spring actions `−k u`;
   - reactions `(K u − f)` on the constrained rows.
   - "Each component … is one exact expansion of its (at most five) product terms, rounded once." Each reaction is "one exact expansion of `K_cj·u_j` products and the ledger terms".
   - Each published quantity is rounded to binary64 once, with its `Binary64Outcome` (§5 item 7). There is never a silent value.
8. **`combine.rs`: `RetainedCombination::form(&[(factor, &RetainedSolve)], p)`** (§4.1.1):
   - "`Σ cᵢ·uᵢ` and `Σ cᵢ·fᵢ` as one exact expansion per component: TwoProduct of the binary64 factor and the p-bit state, then TwoSum accumulation". Each is rounded once, and actions and reactions are recovered from the combined state at p.
   - It is formed at p and at 2p, and "Its published outputs go through the stop rule", with S\* from the combination's own body scale.
   - It "escalates independently of its operand cases". At the ceiling it is withheld (`combination_unresolved`), and "its operand cases keep their standing".
   - Range combinations (envelopes) and T0R's gates stay at the facade.
9. **`adaptive.rs`: the schedule, the stop rule, the scales, the classification and the evidence** (§4.1.6, §4.1.6.1, §5 item 1):
   - **The schedule.** "Candidates at p = 128, 256 and 512, each verified at 2p. The ceiling is 1024 bits." "The verification solve at 2p repeats formation from the binary64 operands". "A rejected 128 candidate's 256 verification becomes the next candidate … at most four solves".
   - **The stop rule.** "`|q_p − q_2p| ≤ 2^-64 · max(|q_2p|, S*)`" on **every** published quantity, compared exactly. It uses S\* per connected body and kind, with the coupling of §4.1.6. "A body with all scales zero … must agree exactly."
   - **The classification** (§4.1.6 items 1–2 and §4.1.6.1 items 1, 2a and 4–7) is on the published value: `absolute_verified` iff `|q| < fl(R·S*)`, with R = 2^-34 (`0x3DD0000000000000`). S\* < 2^-988 makes every row of that body and kind `absolute_verified`. The bound is `b = fl↑(2^-64·S*)`. Restrained and prescribed DOFs are `input_derived` (rule 2a). The per-member stress scales use the pinned k constants, with `k_i` rounded upward.
   - **The closed (kind, unit) table and the unit factors** (§4.1.6.1 items 2–3) map product row kinds. They are F2a's, not K4's.
   - **Failure.** "At the ceiling, or when the budget runs out, the case is unresolved … with the attempted precisions and the reason … Nothing is relabelled as solved."
   - **The evidence F2a's receipt needs** (§5 item 1), as kernel types:
     - the attempts (p, outcome, reason, work);
     - the selected p and the verification p;
     - the stop-rule summary: the worst normalized disagreement per body and kind, rounded upward so that D2's G5a check "≤ 2^-64" stays sound;
     - R's bits, and S\* per body and kind as bits;
     - `input_derived_dofs`;
     - the `absolute_verified` ids with their b, and the `not_covered` ids;
     - the pivot margin minimum, rcond at p, and the retained-residual summary;
     - the ledger and retained-state encodings (Q10).
10. **Budgets and work** (§4.1.7; Q5): "Counted in limb-multiply equivalents per attempt. Successful, failed and verification work is all charged." Factor reuse: "Linear cases that share a modulus basis and state share one p-factor per precision" (the position is taken at checkpoint 0).
11. **Determinism** (§4.1.8): "The same `PrimitiveSource` therefore gives a bit-identical retained state on every platform." The retained-state digest is "over the canonical limbs of u_p and every member's Q" (K3's `parts()`).
12. **The opaque result.** "`RetainedSolve` is bound to its source and precision. `RetainedSolve::publish()` rounds each quantity once." "No caller can supply a matrix, factor, closure or label", which mirrors the KREV boundary of `finish_structural` (now `FK/structural.rs:1883` on main).

### What K4 does not do

- No product caller, no trigger and no D-5 routing (F2a).
- No receipt JSON, identities, diagnostics or row kinds (F2a with D2's S-G1).
- No derived stress rows. The product's stress formulas are applied at the facade to the once-rounded actions (§4.1.5).
- No W1b family (element uniform loads, thermal, thrust, constant effort, user matrices, generated equivalent-static; F3). The three RF-CANCEL UDL cases are W1b.
- No curved element (W1c, with T4). No nonlinear or contact support (T5).
- No sparse_direct, SA or PP change, and no public export from `FK/structural.rs` (Q9).
- No change to `wide.rs`. No change to `multi.rs` unless ROOT rules Q2(b) or Q4(b).
- No dependency and no lockfile change (§4.11).
- No timing or memory-growth claim (K6 owns them).

## ROOT rulings for this slice (2026-09-28)

A TASK drafted this brief. ROOT reviewed it and rules on its open questions as follows. The questions, with their options, remain at the end for the record. The stale-design list is recorded as rulings in `ROOT_RULINGS_V1.md`, "K4: spawn and rulings (ROOT)".

1. **Q1: kernel only.** K4 has no product caller; W1 is wired at F2a. The evidence is:
   - T9 at 112 of 112 (Mac-only);
   - K3a's, K3's, K-D5's, K2b's and `exact_sum`'s suites unchanged.

   **The both-entry gate is not run.**
2. **Q2: (a), K4's own file under `retained/`,** built only on K3's §14 API. The algorithm is chosen at checkpoint 0: the recommended signed wide-integer accumulator, rounded once through `from_integer`, with a span limit that refuses and never truncates. Stop and ask for (b) if (a) cannot meet the contract without per-term allocation.
3. **Q3: the `exact_sum.rs` accessor is approved,** as one additive `pub(crate)` function and a declared write-set extension.
   - It returns the netted (sign, 68-limb magnitude, exponent −2148), netting with the file's own compare and subtract.
   - `exact_sum`'s tests and T9 must be unchanged. **Any behaviour change in `exact_sum.rs` stops the work.**
4. **Q4: (a), with no p + 64 residual on the ceiling's verification-only solve.**
   - The residual at 1024 is formed as one exact expansion, rounded once, and gated at 64·γ_1024, with at most three corrections. The attempt records its residual basis as p.
   - **Condition** (standing lesson): the forward-error argument is written step by step in RETURN, **including its reliance on the uncertified rcond estimate,** and K4's independent reviewer checks it.
5. **Q5: the mechanism ships, with no numeric limits.**
   - The per-case and per-invocation limits are required parameters with no default.
   - K4 measures deterministic counts only; debug wall times are observations.
   - **ROOT's reading of §4.1.7, recorded as a ruling:** "No implementing slice ships without them" binds the slice that wires W1 into the product. **F2a does not merge without ROOT's limits,** which are set from the K6 and V-P measurements (C4). **[Amended: from K6 and V-K, which precede F2a, with K4's deterministic counts; V-P revisits them after F2a. `ROOT_RULINGS_V1.md`, "K4: Q5 amended" (RV13-S3).]**
   - K3's N2 and N3 are handled in K4's attempt record, as recommended.
6. **Q6: add the kernel-only `DirectionalSpring`** to W1a's source, formed at p as k·n nᵀ/(nᵀn).
   - The geometric screen counts a node's directional springs of one kind as grounding that kind only when their directions span R³, decided exactly. Otherwise the case is refused as unsupported.
   - F2a never builds one.
   - The design gap is recorded as a ruling.
7. **Q7: (a), K4 ports a deterministic RCM into `factor.rs`,** with sparse_direct's tie-break rules and FK-local tests on sparse_direct's small graphs. The cross-crate equality test goes to V-K.
8. **Q8: `retained/*.rs` (K4's files) join `FK/tests/s11_site_table.rs` SOURCES,** under K1's five conditions, with the new disposition "p-bit exact expansion, rounded once". K2b is now on main (`e7d930d49`), so the sequencing condition is met.
9. **Q9: the first consumer outside FK adds the public export,** V-K or F2a. K4's RETURN lists the exact items.
10. **Q10: K4 defines and tests canonical byte encodings.** F2a and V-K hash them. The tests pin the encodings' sha256 through the generator's `hashlib` output.
11. **Q11: K4 publishes with K3's `Binary64Outcome` and D1's +0.0 rule for exact sums.** F2a unifies the types with K2b's.
12. **Q12: one slice,** with the optional A1/A2 checkpoint split. K3's N5 streams (10^6 operations at 128, 192, 320 and 576) are required. They add several minutes to FK's debug suite; hosted CI's numerical job (about 11 minutes of 45) has room. Record the measured CI time in the merge record.

## Scope: kernel only (ruled, Q1)

- `retained` is private to `FK/structural.rs` (`mod retained;` at `:5`, MOD-D). K4's items are `pub(crate)`, so no other crate can call them, and no FK code outside `retained` calls them. K4 has no product caller.
- **K4 changes no published byte, by construction.**
  - K4 does not touch `wide.rs`, the only `retained` file on the product path (through K-D5).
  - The `exact_sum.rs` accessor (Q3) is additive and read-only.
  - The evidence is T9 at 112 of 112 (Mac-only), K3a's, K3's and K-D5's suites unchanged, and `exact_sum`'s own tests unchanged.
- **The both-entry gate is not run** (as for K1, K2b and K3). It runs when F2a wires W1.

## Write set (re-locate every line on your base)

| File | Change | Status |
|---|---|---|
| new `FK/src/structural/retained/{source, ledger, assemble, factor, recover, combine, adaptive}.rs` | the method (items 1, 2 and 4–12 above) | in the K4 row |
| new `FK/src/structural/retained/<name>.rs` (named at checkpoint 0) | the correctly rounded exact multi-term sum (item 3) | **approved** (Q2(a)) |
| `FK/src/structural/retained/mod.rs` | the `pub(crate) mod` lines for K4's files, and documentation | in the row ("declared from `retained/mod.rs`") |
| `FK/src/exact_sum.rs` | one additive `pub(crate)` read accessor (K3's Q4) | **approved** (Q3; a declared write-set extension) |
| new `FK/tests/retained_k4/` | the test modules, compiled as `#[cfg(test)] #[path = …] mod tests;` of K4's files (as K3's are of `multi.rs`); a standard-library generator with `--check`; adapters from the frozen references; vectors; the stream manifest and samples; its own `SHA256SUMS` | in the row (tests). A new directory is needed: K3a's and K3's vector checks cover their own directories |
| `FK/tests/s11_site_table.rs` | K4's `retained/` files added to SOURCES, additively, on K1's precedent | **approved** (Q8; K2b is on main at the base) |
| `FK/src/structural/retained/wide/multi.rs` | nothing, unless Q2(b) or Q4(b) | **nothing** (Q2(a) and Q4(a) ruled) |
| `T3/IMPLEMENTATION/K4/**` in `<wt>/k4` | records | certain |

**Not in scope. Stop and ask before touching any of these:**
- `retained/wide.rs` (K3a; K-D5's published-byte surface). Any change to a `Wide<2>` result, `Debug` token or `WideError` `Display` string stops the work.
- `retained/wide/multi.rs`, and K3a's and K3's test directories, generators, vectors and `SHA256SUMS` (unless Q2(b) or Q4(b)).
  - K3's `// K4 API` allowances stay: K4 is now their caller.
- `FK/structural.rs` (the `mod retained;` line exists; the public export is Q9).
- `FK/lib.rs`, `sparse.rs`, `load_ledger.rs`, `rigid_body.rs`, `exact_boundary.rs` and `formation_check.rs`, all read only.
- `sparse_direct`, `SA`, `NI`, `PP` (including PP's `s11f_site_test.rs`), `curved_bend` and `numerical_robustness` (V-K's).
- **The frozen references and fixtures, which are never edited:**
  - N01–N09, R01–R07 and NP-A–NP-D (`P/validation/benchmarks/numerical_integrity/`);
  - R1's `REFERENCES/references.{json,py}` and `README.md`.
  - A mismatch goes to ROOT. It never changes a reference, a scale or the criterion.
- Dependencies and lockfiles; the committed fixtures.

**Constraints on the implementation:**
- **Integers only in the retained path** (§4.1.1, §4.1.8). Binary64 arithmetic appears only in:
  - the exact lifts;
  - the single publication rounding;
  - the pinned binary64 formulas of §4.1.6.1 (+ − × ÷ √ and max, each IEEE-exact).
- **No function of unspecified precision** in K4's files: no `powi`, `powf`, `exp*`, `ln*`, `log*`, trigonometric or hyperbolic function, `hypot` or `cbrt`.
  - The one binary64 libm dependence is inside FK's `assess_rigid_body` (`hypot`). It decides a refusal and contributes no published value. State this in RETURN.
- **No heap allocation per arithmetic operation or per summed term** (§4.11). Per-model and per-sum storage may allocate.
- **The trust boundary.** Nothing caller-supplied beyond the `PrimitiveSource`, and (subject to Q7) integer ordering data (§4.1.1).
- **Errors and outcomes.** K4 adds no `WideError` variant, because `wide.rs` is out of scope. Its own error and outcome types are new, in its own files.
- **`dead_code`.** Each item with no non-test caller gets a per-item `#[allow(dead_code)] // <reason>`, naming the real consumer: "F2a API" or "V-K API" (ROOT's K3 ruling 2). The non-test FK build has no warnings. No module-wide or `cfg_attr` allowance.
- **Tests** follow ROOT's Q7-reversed lesson. No test depends on how the compiler evaluates a function of unspecified precision. Every primitive, including A, Iy, Iz and J, comes from the generator as binary64 bits. The bending magnitude used in comparisons is `sqrt(My² + Mz²)`, written out, not `hypot`.
- **No file named `tests/retained_k4/main.rs`.** Cargo would build it as an integration target, which cannot reach `pub(crate)` items.
- **Memory (I8R).**
  - Never materialize a dense matrix above 1,000 members.
  - RF-MECH-LINE-IN-CHAIN1000 (1,005 members) must go through the geometric screen and the pattern only.

## Basis (read in this order)

T3's records are read at `<wt>/numerics`, which carries ROOT's latest rulings. K3's own records (`IMPLEMENTATION/K3/`) and the code are read on your base.

1. Root `AGENTS.md`, `agents/AGENT_TASK.md`, `_COMMON.md`, and `I8R_K1_RESUME.md` "The Mac host" and "Platform calibration".
2. `T3/DESIGN_NUMERICS/DESIGN.md` revision 5a.2 (sha256 `fb62ef4a…`; hash-pinned, don't edit):
   - §1 (the W1 bullets) and §3.1–§3.2 (the probe evidence, and the controls K4 reproduces);
   - **§4.1, all of it**: 4.1.1–4.1.9;
   - §4.2, the W1a column and the nonlinear-support row;
   - §4.3 and §4.3.1 (what W1 is for and the D-5 routing; context);
   - §4.4 and §4.4.1 (identities; gate conditions 2 and 4, which K4's evidence feeds);
   - §4.10, the kernel lane: what is compared, the predicate, R1's package, the zero-scale floor check and its enumerated list;
   - §4.11;
   - §5 items 1 and 7;
   - §6: the K3, K4, V-K and F2a rows, and the order;
   - §7.1, §7.3 and §7.4.
3. `T3/ROOT_SELECTION_DESIGNS.md`: the order, and C4 (budgets).
4. `T3/ROOT_RULINGS_V1.md`:
   - "D2 revision 5b choices" item 2 (the nonlinear-support rule);
   - "K3: spawn and rulings";
   - "K3: rulings on I11's checkpoint-0 plan", items 2 and 8;
   - "K3: Q7 reversed" (the lesson on functions of unspecified precision; full-count streams at opt-level 0);
   - "K2b: kernel only, and the LEF expectation restated";
   - "K2b: rulings on I10's checkpoint-A stop", ruling A (spring-carried goes on K4's list);
   - "K2b: the b-rule's window misses …" (the standing lesson on derived claims);
   - "K1: the S11 site table for sparse.rs and formation_check.rs" (the extension precedent);
   - "I4's F12 stop", item 4, and "Amendment: the 'no worse' condition" (N05's transverse tip, routed to T3's N05 item).
5. `T3/ROOT_SELECTION_REFERENCES.md` (the freeze and its rules) and `T3/ROOT_RULINGS_V2.md` §1 (the RF-CANCEL binding scale).
6. **K3's records** (on your base):
   - `IMPLEMENTATION/K3/RETURN.md`: §3 item 7, §7, §14, §15, and addenda 1 and 2 (S1; N2–N5);
   - `CHANGE_RECORD.md`;
   - `K3_MERGE/RECORD.md`, "For K4's brief";
   - `REVIEW/K3_REVIEW.md`: S1, N2–N5, §3.4 and §3.6.
7. **K1's records:** `IMPLEMENTATION/K1/RETURN.md` §12 (`SparsePattern` and the pattern path).
8. **K2b's records:** `IMPLEMENTATION/K2B/RETURN.md` §6, §9 (ruling A), §15 and §19. K2b is merged (PR #1040), so read them on your base.
9. **K-D5's records:** `IMPLEMENTATION/KD5/RETURN.md` §3a (the 122 true positive) and the D5C-1 controls in its tests.
10. **The references:**
    - `T3/REFERENCES/README.md`, `references.json` (sha256 `7b176dbb…`) and `references.py` (`80d473a7…`). `references.py --model <id>` prints RF-MECH-LINE-IN-CHAIN1000's full model.
    - `P/validation/benchmarks/numerical_integrity/{README.md, fixtures.json}` (N05, N06, NP-A).
    - D1's probes `DESIGN_NUMERICS/_run_records/probe_skew_precision.py` and `probe_rev2_b1.py`. These give the model definitions of the k = 1e-28, six-member, B1 and S8-W controls. They are emulations, not references: K4 computes its own exact expectations.
11. **The code on your base:**
    - `retained/{mod.rs, wide.rs, wide/multi.rs}`;
    - `FK/tests/retained_wide/**` and `retained_wide_k3/**` (read only; the stream scheme);
    - `FK/exact_sum.rs`: the magnitudes at `:45-48`, `compare` at `:103` and `subtract` at `:113`;
    - `FK/load_ledger.rs`: `ForceTerm::accumulate` at `:39` and `accumulate_dof` at `:312`;
    - `FK/structural.rs`, the M03 stages: `gamma` `:521`, `evaluate_original_residual` `:1383`, `screen_pivot` `:1494`, `estimate_rcond` `:1521`, `finish_checked_factor` `:1635`, `finish_structural` `:1883`, `factor_structural_profile` `:2027`, `verify_negative_direction` `:2123` and `negative_pair_witness` `:2166`;
    - `FK/structural/sparse.rs`: `SparsePattern` `:48`, `from_positions` `:58` and `from_connectivity` `:82`;
    - `formation_check.rs`, `rigid_body.rs` (`assess_rigid_body` `:33`), `exact_boundary.rs` (the oracle) and `FK/lib.rs` (the product's frame algorithm);
    - `sparse_direct/src/lib.rs:505` (`reverse_cuthill_mckee`);
    - `FK/tests/s11_site_table.rs`.
12. `T3/OWNER_DIRECTION.md`, "Owner decision (2026-09-28): DEC-025 on the Mac".
13. `TASK_BRIEFS/I10_K2B_IMPLEMENTATION.md` and `I11_K3_IMPLEMENTATION.md`, for coordination.

## Base, branch and paths

- **Branch:** `codex/piping-k4-20260928`, from current main. ROOT creates it in `<wt>/k4` and records the SHA at spawn.
  - **At spawn, the base is main `e7d930d49`** (K1, K3 and K2b merged). ROOT created the branch there.
  - K2b merged before the spawn, so Q8's file overlap does not arise.
- **Target:** `<wt>/k4-target`.
- **Scratch:** `<wt>/scratch/i12`.
- **Mutants:** one clean copy and one clean target per mutant, under `<wt>/k4-mut/<mutant>/`. Delete each target afterwards.
- **Python:** `<VENV>`. The generator uses the standard library only (`fractions`, `math.isqrt`, `hashlib`).
  - It may import K3's `gen_wide_k3_vectors.py` and R1's `references.py` read-only. Record their hashes.

## Coordination

**K2b (I10, PR #1040, merged at `e7d930d49`, K4's base).**
- Its write set:
  - `FK/lib.rs`, `FK/structural.rs`, `FK/structural/sparse.rs` and `FK/load_ledger.rs` (`force_scaled`);
  - `SA`;
  - `NI/src/s11k_tests.rs`, `FK/tests/s11_site_table.rs` and `PP/tests/s11f_site_test.rs`;
  - new tests: `FK/tests/k2b_force_scaling.rs`, and NI's `k2b_models.rs` and `k2b_tests.rs`.
- **Write-set overlap: none** in K4's certain rows. K2b does not write `retained/` or `exact_sum.rs`.
  - The one possible overlap was `FK/tests/s11_site_table.rs` (Q8, approved). K2b's edit is already on K4's base, so K4 edits the file on top of it.
- **Semantic touchpoints:**
  1. **Two publication types.** K2b publishes with `Representability`/`PublishedValue`; K3 gives `Binary64Outcome` (Q11).
  2. **The zero conventions.** K2b gives a reaction's exact zero as +0.0, and "keeps its sign for an action". D1 §4.1.2 gives +0.0 for every exact sum.
  3. **The spring-carried G = 1e-300 case** is on K4's list (ruling A).
  4. K2b's use of `ExactAccumulator::round_scaled` is unaffected by K4's additive accessor.

**K5 (W4: `FK/rigid_body.rs`, `SA`) and F1b (`PP`, `source_recovery.rs`, the NI loop), if they run in parallel.**
- **Write-set overlap: none.**
- K4 reads `assess_rigid_body`. K5 adds `assess_constrained_bodies` beside it (§4.9). If K5 changes `assess_rigid_body` itself, K4 re-runs its RF-MECH tests after merging main.
- F1b shares no file and no type with K4.

**K3's and K-D5's files:** K4 writes none of them (subject to Q2 and Q4).

**V-K** follows K4. It consumes K4's API through the public export (Q9). K4 does not build `numerical_robustness`.

**Merge order.** Any slice may merge first. The second merges main, then re-runs its suites and T9 before its PR merges.

**Host.** Implementers share the Mac under I8R's caps: at most two cargo jobs of your own, `-j 8`, `RUST_TEST_THREADS=4`, and at most three mutants at once at `-j 4`. ROOT may serialize the heavy phases: T9 builds, mutation batches, stream generation.

## Required tests

The predicate is the unchanged `|obs − exp| ≤ 1e-9·max(|exp|, scale)`, with R1's scales and the RF-CANCEL net-governed column. No new tolerance is introduced anywhere.

**A. Nothing existing moves**
- K3a's and K3's test directories are byte-identical. `gen_wide_vectors.py --check` and `gen_wide_k3_vectors.py --check` give OK.
- K3a's, K3's and K-D5's tests, and `exact_sum`'s own tests, pass unchanged.
- FK's full suite, and CI's 39-manifest profile with `--no-fail-fast`, match ROOT's Mac baseline of main. Every failure must be identical to a Mac-main failure.
- The non-test FK build has no warnings.
- The S11 site table passes unchanged, apart from the declared Q8 extension.
- T9 (see Gates).

**B. The exact multi-term sum (RV12's S1)**
- **Against `Fraction`**, at every K4 precision (128, 192, 256, 320, 512, 576 and 1024) at its width.
- **RV12's counterexample:** at p = 128, e₁ = 1 + 2^-127, e₂ = 2^-128 and e₃ = −2^-400 must give 1 + 2^-127.
- **Ties decided by a far tail,** including at p = 64L.
- Total cancellation to an exact zero, which gives +0.
- Mixed signs; exponent gaps beyond the width; exact products as terms; binary64 terms and the ledger's exact net as terms.
- The span or exponent limit refuses, and never truncates.
- **A seeded differential** of at least 10^5 sums per precision, with 2 to 64 terms, in K3's scheme: a recorded seed, chunk and stream digests, and the first 1,000 records committed.

**C. The ledger and the accessor (K3's N4)**
- The accessor nets `ExactAccumulator`'s positive and negative magnitudes once, with the accumulator's own `compare` and `subtract`. It returns the sign, the magnitude and the 2^-2148 quantum. A zero is (+, 0).
- Accessor plus `from_integer` at p = 53 gives `round()`'s bits wherever `round()` is defined. The allowed differences are only those K3 ruled: the exact −0 and overflow.
- At every K4 precision, it matches `Fraction` on seeded ledgers, including cancellation and ties.
- Per-DOF granularity: RF-CANCEL's authored-order contributions, and V1's check L (1e80, 1e-8, −1e80), are exact.
- A prescribed-coupled right-hand-side row whose correct rounding is a tie decided by the ledger's tail (it catches a ledger rounded to p before it joins the sum).

**D. The arithmetic at K4's working precisions (K3's N5)**
- 10^6-operation streams for + − × ÷ √ at p = 128 and 192 (L = 4), 320 (L = 8) and 576 (L = 16), in K3's scheme, with the `Fraction` oracle.
- Record the debug wall times as observations. K3's RETURN §8 measured about 31 s, 78 s and 215 s per 10^6 operations at L = 4, 8 and 16 (at p = 256, 512 and 1024), so these four streams may add about six minutes to FK's debug suite. That is an estimate, to be measured at A.
- They stay at full count in the default suite, at opt-level 0, with no `#[ignore]` (ROOT's Q7-reversed ruling). Hosted CI's numerical-job time goes in the merge record.

**E. Formation, assembly and reduction**
- **The rigid modes.** For the six rigid motions of the binary64 geometry, `K_e·r` is O(2^-p·a) at every p, on axis-aligned, skewed (3,4,0) and oblique (2,3,6) members (§3.1).
- **Against the product's element.** At p = 53 the element agrees with FK's binary64 local stiffness to about 1e-16, with the same zero pattern (§4.1.2 item 3).
- **Against K-D5.** At p = 128 it agrees with K-D5's `Wide<2>` re-formation within a stated few-ulp bound. This is an independent formula.
- **Assembly entries** equal the exact sum of the p-rounded contributions, rounded once, from the generator's `Fraction` oracle.
- **Order independence.** Permuting the source's member, spring and load lists gives bit-identical K, rhs and retained-state encodings.
- **§7.3-16's duplicate-operand control:** two identical members meeting at a node, and a third member 2^-300 as stiff.
- **A prescribed-motion control** (the KREV-02 analogue; §7.3-3), with an exact reference.

**F. Factor, screens, solve and refinement**
- The pivot screen at p, escalating on failure.
- The `rcond ≤ 2^-(p−1)` escalation.
- **Negative energy:** a unit test of the pattern-pair check on a constructed indefinite pair. No valid source can reach it, because frames and positive springs have non-negative energy (§4.3).
- **Geometry first:** RF-MECH's 8 refusal cases are refused, with no rows and no escalation. RF-MECH-LINE345-RX-COMPANION solves.
  - RF-MECH-K0's zero spring is either rejected by the source's validation (springs are k > 0; R1 accepts rejecting a zero spring as a refusal) or omitted by the adapter, so that geometry refuses the case. State which.
- **The p + 64 residual:** a control on which a residual formed at p would accept while the p + 64 residual refines or escalates. If no admissible control exists, derive why, and report it; ROOT rules, as for M31b.
- At most three corrections.
- The ceiling's solve, as ruled under Q4.

**G. The schedule and the stop rule**
- **The k = 1e-28 case** (§3.1): 128 is rejected; 256 is accepted against 512, within 1e-9.
- **The six-member skew run, k = 1e-12** (39 free DOFs): 128 is rejected (2.1e-19 > 2^-64). Record its accepted p.
- N05 and N06 are accepted at 128.
- The attempts list; the reuse of each verification as the next candidate; and at most four solves.
- Every published kind is in the comparison.
- A structural-zero kind is accepted through the coupled S\* (§4.1.6, rationale).
- An all-zero body agrees exactly.
- The ceiling and budget exhaustion give an unresolved outcome with the attempts and the reason, never a relabelled result.

**H. Recovery and publication**
- End actions, stations, spring actions and reactions, each rounded once with its `Binary64Outcome`.
- An exact zero is +0.0.
- The subnormal, underflow and overflow outcomes, on constructed values.
- N06's torque and spring action are nonzero and correct (§7.3-2).

**I. Combinations**
- **B1-C:** A + B − A2, with A = A2 = 1e80 and B = 1e-8, is exact.
- **B1-E:** (P, ε) − P, with ε/P = 1e-45. 128 is rejected, and 256 is accepted against 512.
- A combination escalates independently of its operands.
- At the ceiling it gives `combination_unresolved`, and its operands keep their outcomes.
- S\* comes from the combination's own body.

**J. S\* and the classification (§4.1.6.1)**
- Bit for bit against the generator's reimplementation of the pinned binary64 formulas: coupling order, L_b, S(kind) and the per-member stress scales with k₁, k√2 (`0x3FF6A09E667F3BCD`), k_{2√2} (`0x4006A09E667F3BCD`), k₄ and `k_i = fl↑(k√2·i)`.
- A row within one ulp of `t = fl(R·S*)` on each side (§7.3-20).
- S\* < 2^-988; b = `fl↑(2^-64·S*)`, including 0 < S\* < 2^-1011; and b = 0 only at S\* = 0.
- `input_derived` for restrained and prescribed DOFs (rule 2a).
- **S8-W:** the far-node quantities are `absolute_verified` (§7.3-17), with R = 2^-34 in place of the probe's older constant.

**K. The references and routed cases (the row)**
- **N05, N06 and NP-A on the intended basis,** "on every quantity, including internal torque and the spring action". A result equal to NP-A's represented solution is a failure (§7.1).
- **N05 with a transverse tip force.** This is S11-F's retained-replay budget overflow, routed to T3's N05 item. W1 solves it within 1e-9.
- **R1, frozen at `c0f14201c`**, through adapters generated from `references.json`, whose sha256 the generator checks:
  - RF-CHAIN, all 30;
  - RF-SKEW, 36 (or 18 under Q6);
  - RF-WEAK, 9;
  - RF-FINITE, 6. THIRTIETHS-O1e6 is compared on the represented basis;
  - RF-MECH, 9;
  - RF-CANCEL, the 38 nodal-load cases. UDL-W1e5, W1e8 and W1e80 are W1b.
  - The represented basis also applies to RF-SKEW-A-CANT-AX-122-r1e-12 (Q6).
  - **The adapter** forms every binary64 primitive exactly as `references.py` defines the represented inputs, and states each property it uses.
  - Twist and extension are derived as `T/k_t` and `N/k_a` (§4.10), never differenced.
  - **The zero-scale floor check.** A comparison whose scale is below R·S\* is `not_covered` and never counts as a pass. Report three numbers: passes, absolute-range passes and not-covered comparisons. The not-covered set must equal §4.10's enumerated list restricted to these cases: RF-WEAK 46, RF-CANCEL 3 and RF-SKEW 2. A difference is reported, not absorbed.
  - **The discrimination check.** Each discriminating negative control must fail the same predicate.
- **The exact-block oracle in scope.** On cases where exact-block selects (every free block of order ≤ 2; for example N01, N08 and N09), W1's values match exact-block's projections within the criterion (§4.4, table). Enumerate the cases at checkpoint 0.
- **The routing targets:**
  - K-D5's required true positive, RF-SKEW-T-CANT-OFF-122-r1e-04, is solved within 1e-9;
  - so are K-D5's D5C-1 controls: the solve-error-only case, the absorbed spring and the axis-aligned bending-soft case.
- **The spring-carried G = 1e-300 case** (ROOT's K2b ruling A: "W1 (K4 and F2a) … is to evaluate it"). Report its outcome. A failure to solve it is a finding, not a stop.
- **B1-L:** check L is exact with the ledger (§3.2).

**L. Budgets, work and determinism**
- **Golden work counts per case,** in limb-multiply equivalents by stage and precision. They are deterministic and pinned. They kill a double-recorded context (K3's N2).
- `from_integer`'s magnitude length is charged (K3's N3; Q5).
- Failed and verification work is charged.
- **Budget limits** are passed in as parameters. Exhaustion gives the unresolved outcome, per case and per invocation.
- **Determinism:**
  - the same source gives a bit-identical retained-state encoding across runs and under the permutations of E;
  - factor reuse, if implemented, is bit-identical to separate solves.
- **A source scan** pins that K4's files call no function of unspecified precision (see the constraints).

## Mutants

Run from clean copies, with a NONE control first. Each mutant must be killed at a behavioural assertion; name the killing test. A survivor is a defect to report; never weaken a test to kill it. Where no admissible control exists, derive the equivalence and report it; ROOT rules.

**The design's method mutations (§7.3) that touch W1a:**

| # | Mutant | Intended kill |
|---|---|---|
| D1 | Promote the rounded binary64 K to p | N05 (wrong k); N06 (nonpositive pivot, so unavailable); NP-A's "equal to represented" assertion |
| D2 | Round u to binary64 before recovery | N06's torque becomes 0 (H) |
| D3 | Drop `K_fc·u_c`, or round the reduced rhs to binary64 | the prescribed-motion control (E) |
| D4 | Omit one contribution, or flip an axis sign in B | the rigid-mode test (E); N and R1 (K) |
| D5 | Fix p at 128, with no escalation | the k = 1e-28 case (G) |
| D6 | Skip the 2p verification, accepting on the pivot screen alone | the k = 1e-28 case (G; margin 3.4, error 1.8e-4 at 128) |
| D7 | Disable the geometric mechanism check | RF-MECH "recovered" (F, K) |
| D10 | (adapted) Order from the source's list order or labels instead of the pattern | the permutation determinism test (E, L) |
| D13 | Fold loads at p instead of the ledger | B1-L (accepted with error 0.5); RF-CANCEL (K) |
| D14 | Combine retained states term by term at p | B1-C (error 1.0) (I) |
| D15 | Take combination outputs out of the stop rule | B1-E (published from 128, error 1.0) (I) |
| D16 | Assemble stiffness entries or recovery sums by sequential addition at p | the duplicate-operand control (E) |
| D17 | Drop the verified-accuracy classification | S8-W (J) |
| D20 | Classify on the p value; or R = 10^9·2^-64 with a rounded threshold; or S\* from anything but published values | the one-ulp boundary rows (J) |
| D25 | (kernel part) k = 1 for every stress kind, or √2 and 2 for the span-statics rows | the per-member scale bits (J) |

§7.3's items 8, 9, 11 and 12 are W3's, W2's, the facade's and W4's. Items 18, 19 and 21–32 belong to other slices or to V-K's harness.

**K4's own:**

| # | Mutant | Intended kill |
|---|---|---|
| K4-M1 | The multi-term sum folds at p | B (RV12's counterexample) |
| K4-M2 | The sum drops the far tail (sticky) | B (ties decided by a far tail) |
| K4-M3 | The sum rounds ties away from zero, or truncates | B |
| K4-M4 | The sum returns −0 for an exact zero | B; H |
| K4-M5 | The sum truncates at its span limit instead of refusing | B |
| K4-M6 | The accessor returns the positive magnitude only (no netting) | C |
| K4-M7 | The accessor's sign or quantum is wrong (for example 2^-2147) | C |
| K4-M8 | The ledger is rounded to p before it joins a coupled rhs or a reaction | C (the tie row) |
| K4-M9 | The pivot screen uses γ with u = 2^-53 | N06 (refused at every p) (F, K) |
| K4-M10 | The rcond escalation is skipped | F (a constructed case) |
| K4-M11 | The residual is formed at p, not p + 64 | F (or a derived equivalence, for ROOT) |
| K4-M12 | The residual is folded, not one exact expansion | F; G |
| K4-M13 | The 2p verification reuses the p formation | the k = 1e-28 case (common-mode formation error accepted) (G) |
| K4-M14 | The stop rule uses \|q_p\| instead of \|q_2p\|, or 2^-53 instead of 2^-64 | the six-member case (accepted at 128) (G) |
| K4-M15 | The uncoupled S(kind) is used instead of S\* | the structural-zero control (never accepted) (G) |
| K4-M16 | Reactions or spring actions are left out of the stop rule | a control disagreeing only in that kind (G) |
| K4-M17 | An attempt's context is recorded twice | golden work counts (L) |
| K4-M18 | `from_integer`'s length is not charged | golden work counts (L) |
| K4-M19 | Failed or verification work is not charged | golden work counts (L) |
| K4-M20 | Budget exhaustion is ignored, or an exhausted case is reported selected | L |
| K4-M21 | A witnessed mechanism is escalated instead of refused | RF-MECH's attempts (F, K) |
| K4-M22 | Gram-Schmidt uses FK's binary64 axes | RF-SKEW; the oblique rigid-mode test (E, K) |
| K4-M23 | (Q6) A directional spring is formed from the binary64-normalized direction | RF-SKEW AX r1e-12 (K) |
| K4-M24 | A combination's escalation is tied to its operands, or its operands' outcomes change at the ceiling | I |
| K4-M25 | b rounded to nearest, or the S\* < 2^-988 rule dropped | J |
| — | Your own, at least two | — |

## Gates (ROOT runs the PR)

- **Suites.** FK's full suite, and CI's 39-manifest profile with `--no-fail-fast`, against ROOT's Mac baseline of main.
- **T9 (Mac-only).** The committed-fixture diff, built on this Mac from `git archive` copies of base and candidate and compared with each other.
  - 112 of 112 byte-identical is expected.
  - **Any committed-byte change stops the work.**
- **The both-entry gate is not run** (subject to Q1).
- **An independent complete-diff review,** with oracles independent of K4's generator:
  - an exact-rational solve of a subset of the named references;
  - an independent `Fraction` check of the multi-term sum and of the classification bits.
- **Hosted CI** green on the candidate head. Record the numerical job's time.
- **DEC-025** under the owner's Mac decision (`OWNER_DIRECTION.md`, 2026-09-28):
  - the Mac sweep, whose only cargo failures are the three known platform tests, identical to Mac main;
  - pytest, vitest and the build pass;
  - hosted Linux CI's numerical cargo job supplies the clean Linux cargo run;
  - the deviation is recorded in the merge record.
- **No native witnesses.** K4 is kernel only.

## Checkpoints

End your turn at each one with a status for ROOT: the changed files, the results, and any stop. ROOT verifies, commits and resumes you.

- **0: a plan, before any product code.** It covers:
  - the signatures of every K4 file, including the multi-term sum, the accessor and the outcome and evidence types;
  - the multi-term sum's algorithm (Q2), and its span or exponent limit;
  - the widths per precision, and how values cross widths;
  - the published-quantity set, including where stations come from (the source, as binary64 positions, is recommended);
  - how S\* is formed for the stop rule (at 2p) and for the classification (on published values), and the rounding direction of the summary ratio;
  - the combination's precision rule when its operands were accepted at different p;
  - factor reuse, and how its work is attributed;
  - the budget API (Q5);
  - the canonical encodings (Q10);
  - the ordering (Q7) and directional springs (Q6);
  - the ceiling (Q4);
  - the generator and the adapters: the reference hashes, and how `references.py` defines the represented inputs;
  - the exact-block in-scope case list;
  - the dead-code plan;
  - the test and mutant list;
  - your position on every open question still unresolved.
- **A:** a clean compile; the targeted tests B–L; K3a's, K3's and K-D5's tests unchanged; a warning-free non-test build; and the debug wall times for B and D.
  - At ROOT's request, A may be reported in two parts: A1 (B–F at a fixed p) and A2 (G–L).
- **B:** the suites against the Mac baseline of main, and T9.
- **C:** the mutation table, with the NONE control first.
- **D:** CHANGE_RECORD and RETURN, with `_run_records/` and SHA256SUMS.

**Stop and report** (end your turn) on any of these:
- a change to any existing result: a K3a or K3 vector or digest, a K-D5 test, an `exact_sum` test, or a T9 byte;
- a needed edit outside the write set, including `wide.rs`, `multi.rs`, `FK/structural.rs`, `sparse.rs`, `load_ledger.rs` or `sparse_direct`;
- a new dependency;
- **a reference mismatch.** Never edit a reference; the case goes to ROOT;
- a discriminating control that does not fail;
- a not-covered set that differs from §4.10's list;
- a mutant that survives;
- a p-bit sum that cannot be made exact and rounded once;
- a binary64 load, force or right-hand-side fold in a K4 file (a real S11 rule-7 violation);
- a design item that cannot be implemented as specified (integers only; no allocation per operation);
- a SIGKILL from the memory guard. Check `<wt>/guard/memguard.log`, and do not retry blindly.

## Return

- **Files:** `T3/IMPLEMENTATION/K4/` on the K4 branch: CHANGE_RECORD (following `.agents/skills/chirality-change/SKILL.md`), RETURN, `_run_records/` and SHA256SUMS.
  - Use placeholders only (`<wt>`, `<scratch>`, `<VENV>`), with no machine paths and no model identifiers.
  - State the platform (`aarch64-apple-darwin`, rustc 1.97.1), and that T9 is a Mac-only comparison.
- **RETURN covers:**
  - the files and their line counts;
  - how each K4 item was met, with the design's words;
  - the checkpoint-0 positions as ruled;
  - the per-case reference results: passes, absolute-range passes, not-covered comparisons and failures, with the not-covered list compared with §4.10's;
  - the work and storage tables: limb-multiply equivalents by stage and precision, pattern and profile entries, and limbs per entry;
  - the streams' seeds, counts and digests;
  - the mutation table;
  - T9 and the per-crate counts against the baseline;
  - the toolchain and host;
  - the delegation mechanism;
  - what was not done.
- **Every general claim is derived and independently checkable** (ROOT's standing lesson): Q4's ceiling argument, any mutant equivalence, and the multi-term sum's correctness argument.
- **RETURN has an "F2a and V-K interface" section with exact signatures,** as K1's §12 did for F1b and K3's §14 did for K4. It includes the export list Q9 needs.

## Open questions for ROOT (each with a recommendation)

**Q1. Scope and evidence.**
- **Recommendation:** kernel only, with no product caller. The evidence is T9 at 112 of 112, and K3a's, K3's, K-D5's and `exact_sum`'s suites unchanged. **The both-entry gate is not run.** This matches K1, K2b and K3; the gate runs when F2a wires W1.

**Q2. The home of the correctly rounded exact multi-term sum (RV12's S1).**
- **(a) K4's own file under `retained/`,** built only on K3's §14 API.
- **(b) A declared `multi.rs` extension,** with private access to the limb core.
- **Recommendation: (a).**
  - It keeps K3's merged, independently reviewed file and its hash-checked vectors closed. Under (b), K3's review would no longer cover `multi.rs`.
  - It stays within the row's letter: "new files under `retained/` only".
  - The API suffices. RV12 found `two_sum`, `two_product`, `widen`, `round`, `cmp_value`, `mul_pow2` and `from_integer` correct and value-tested.
- **The algorithm is chosen at checkpoint 0.** I recommend a signed wide-integer accumulator: separate positive and negative magnitudes, netted once, in the manner of `ExactAccumulator`, in a fixed stack buffer. It is rounded once through K3's `from_integer`, which is already tested on 68-limb magnitudes with ties and far sticky bits. It has a stated span limit that refuses, never truncates. The limit is justified against the binary64 input range, and it fails closed through escalation.
- The alternative is a TwoSum expansion with a final correct rounding. That needs its own tie logic at p = 64L.
- If (a) cannot meet the contract without per-term allocation, K4 stops and asks for (b).

**Q3. The `exact_sum.rs` read accessor (K3's Q4), a declared write-set extension.**
- **Recommendation: approve one additive `pub(crate)` function.**
  - It returns the netted (sign, 68-limb magnitude, exponent −2148). It nets with the accumulator's own `compare` and `subtract` (K3's N4). A zero is (+, 0).
  - It adds no accumulation shape, so the site table is unchanged.
  - `exact_sum`'s tests and T9 must be unchanged. `exact_sum.rs` is live S11 code, so any behaviour change stops the work.

**Q4 (K3's Q8). The ceiling's refinement.** A 512-bit candidate is verified at 1024, and a p + 64 residual there needs 1088 bits. `Wide<16>` holds 1024.
- **(a) No p + 64 residual on the ceiling's verification-only solve.** Its residual is formed at 1024 against K re-formed at 1024, as one exact expansion rounded once, and gated at 64·γ_1024, with at most three corrections. The attempt records its residual basis as p, not p + 64.
- **(b) One more width** (L = 17 or 18), as a declared `multi.rs` extension. It would need K3-grade vectors and streams, and a fourth `AttemptWork` slot.
- **(c) No residual at all** on the ceiling's solve.
- **Recommendation: (a).**
  - The 1024 solve is never a candidate (§4.1.6: candidates are 128, 256 and 512). It serves only as q_2p, which must be "accurate well beyond 2^-64·S\*".
  - The 512 candidate's own screen gives rcond > 2^-511. A 1024 solve whose backward error is gated at about 2^-1018·m then has a forward error of about cond·2^-1018·m. That is far below 2^-64.
  - This argument rests on the rcond estimate, which is not certified, as everywhere in M03. K4 writes it out step by step in RETURN, and the reviewer checks it.

**Q5. The budgets, and how K4 measures without timing claims.**
- **The tension.** §4.1.7 says "No implementing slice ships without them". But `ROOT_SELECTION_DESIGNS.md` C4 selects the D-8 limits "from the K6 and V-P measurements", which come after K4.
- **Recommendation:** K4 ships the mechanism and no numeric limits.
  - Work is charged, including failed and verification work.
  - The per-case and per-invocation limits are required parameters with no default.
  - Exhaustion gives the unresolved outcome with its attempts.
  - §4.1.7's sentence is read as binding the slice that wires W1 into the product: **F2a does not merge without ROOT's limits.**
- **K4's measurement is deterministic counts only:** limb-multiply equivalents per case, by stage and precision; pattern and profile entries; and limbs per entry, for every named case. They are committed as a table.
  - Debug wall times are recorded only as observations, for the CI budget.
  - K6 turns work units into time on the owner's Mac under `_COMMON.md`'s interleaved-run rule, and ROOT sets the limits from K6 and V-P.
- **K3's N3:** K4 charges `from_integer`'s magnitude length in its own attempt record, with no `multi.rs` change.
- **K3's N2:** one context per width per attempt, recorded once at the end, and pinned by the golden counts.

**Q6. Directional springs (a design gap).**
- §4.1.1's W1a source has springs `(dof, k > 0)` only, but §4.10 says "Springs can lie along any direction at kernel level".
- R1 flags 18 of RF-SKEW's 36 cases `needs_directional_spring`. They are all the AX cases, whose soft spring lies along the member axis: the core N05-class skew cases. They include the represented-basis case RF-SKEW-A-CANT-AX-122-r1e-12, and both of §4.10's RF-SKEW not-covered rows (`tw.M1` in T-CANT-AX-122-r1e-12 and -345-r1e-12).
- **Recommendation:** add a kernel-only `DirectionalSpring { node, kind: translation | rotation, direction: [f64; 3], k > 0 }` to W1a's source.
  - It is formed at p as `k·n nᵀ/(nᵀn)` from the binary64 direction, as R1 says ("normalize in the product").
  - For the geometric screen, a node's directional springs of one kind count as grounding that kind's three DOFs only when their directions span R³, decided exactly. Otherwise the case is refused as unsupported. R1's AX triads, such as (1,2,2), (2,1,−2) and (−2,2,−1), span R³.
  - F2a never builds one, because the product authors global-axis springs only.
- **The alternative:** run only RF-SKEW's 18 global-spring cases, and move the AX cases to V-K, which then needs the same primitive.

**Q7. The ordering (made stale by K1).**
- §4.1.3 says "RCM ordering from the pattern. The ordering is integer data, shared with the binary64 sparse path."
- K1 placed that RCM in `sparse_direct` (`reverse_cuthill_mckee`, `order_sparse_structural`). `sparse_direct` depends on FK, so FK cannot call it.
- **(a)** K4 ports a deterministic RCM into `factor.rs`: integers only, with sparse_direct's tie-break rules, and FK-local tests on sparse_direct's own small graphs. The cross-crate equality test goes to V-K, whose crate sees both.
- **(b)** The caller supplies the order as validated integer data, as `factor_structural_profile` does. The order then becomes part of the replay identity.
- **(c)** Move RCM into FK. That is outside K4's write set.
- **Recommendation: (a).** It keeps "the same `PrimitiveSource` gives a bit-identical retained state" (§4.1.8) true without extra inputs.

**Q8. The S11 site table.**
- K4's files carry the load ledger, right-hand sides, reactions and recovery sums, which are S11 rule 7's subject.
- **Recommendation:** add `retained/*.rs` (K4's files) to `FK/tests/s11_site_table.rs` SOURCES, under K1's five conditions:
  - additive only;
  - a disposition for every match;
  - a binary64-fold mutant in a K4 file killed by the table;
  - declared in CHANGE_RECORD and RETURN;
  - a site that fits no disposition stops the work.
- A new disposition is needed: "p-bit exact expansion, rounded once".
- The table's stated limit applies: folds written through `WideContext::add` are invisible to the scan. So the numeric mutants D13–D16 and K4-M1 are load-bearing.
- **Sequence:** only after K2b, which edits the same file, is on main.

**Q9. The public export (no slice owns it).**
- K4's items are `pub(crate)` in the private `retained` module. V-K's crate and F2a's `PP` need a `pub use` in `FK/structural.rs`, but MOD-D gives K4 "files under `retained/`" only.
- **Recommendation:** the first consumer outside FK adds the export: V-K or F2a, whichever runs first. K4's RETURN lists the exact items to export.
- **The alternative:** one declared export line in K4.

**Q10. Digests.**
- D1 names an "identity digest" (§4.1.1) and a sha256 "retained-state digest" (§4.1.8). D2's G5a requires 64 lowercase hex for the retained-state and load-ledger digests.
- FK has no sha256. PP already depends on `sha2`.
- **Recommendation:**
  - K4 defines and tests canonical byte encodings of the source, the ledger and the retained state (K3's `parts()` limbs).
  - F2a, and V-K's replay, hash them.
  - `RetainedSolve` binds to its source by holding it.
  - K4's tests pin the encodings' sha256 through a test-only implementation, or through the generator's `hashlib` output.
- **The alternative:** an in-repo sha256 in `retained/`, with FIPS 180-4 known answers.

**Q11. The publication types (K3's Q5 deferral).**
- **Recommendation:** K4 publishes with K3's `Binary64Outcome` and its own +0.0 rule for exact sums (D1 §4.1.2). It does not import K2b's `Representability`/`PublishedValue`.
- F2a, the first slice that consumes both, maps them onto one publication rule. That includes the zero conventions: K2b keeps an action's sign, and D1 gives +0.0 for exact sums.

**Q12. One slice, or two.**
- K4 is large: about 2,500–3,000 lines by estimate, plus tests.
- **Recommendation: one slice,** with the optional A1/A2 checkpoint split.
  - A K4a without the adaptive schedule would ship a fixed-p solver.
  - "The pivot screen alone is not enough" (§3.1). The stop rule is "required, not optional".
  - The method mutations D5, D6 and D13–D16 need the whole pipeline.

## Design text that K1, K2b and K3 have made stale (for ROOT; `DESIGN.md` stays hash-pinned)

1. **§4.1.1's `wide.rs`** is now `wide.rs` (K3a, `Wide<2>`) and `wide/multi.rs` (K3, L = 4, 8 and 16). K4's p = 128 and 192 run at L = 4 (K3 ruling 8).
2. **§4.1.2's "one accumulation discipline … shared with the binary64 route's `FK/exact_sum.rs`"** holds for binary64 terms only. p-bit sums need K4's primitive (RV12's S1; Q2).
3. **§4.1.2 item 5's projection** is K3's `from_integer` plus K4's accessor (Q3). The accessor must net the split magnitudes (K3's N4).
4. **§4.1.3's shared RCM** is in `sparse_direct`, which FK cannot reach (K1; Q7).
5. **§4.1.4 with §4.1.6:** p + 64 at the ceiling is 1088 bits, beyond `Wide<16>` (K3's Q8; Q4).
6. **§4.1.7's "No implementing slice ships without them"** conflicts with C4's measurement order (Q5).
7. **§4.1.1's springs** against §4.10's directional kernel springs (Q6).
8. **§5 item 7's outcomes** are carried by two types: K2b's and K3's (Q11). K2b's action zero keeps its sign, where §4.1.2 gives +0.0.
9. **MOD-D** gives no slice the public export V-K and F2a need (Q9).
10. **§4.10 and §7.1's "LEF-small … solved"** was restated for the ordinary route (K2b: `DegenerateAxis` at the 1e-12 m tolerance). W1's source has "No axis tolerance … at p". Whether it admits LEF-small is V-K's question. RF-RANGE is not in K4's row.
11. **Line drift.** For example, `finish_structural` is at `FK/structural.rs:1883` on main (§4.1.1 cites `:1086-1107` at `c61a540ea`). `FK/lib.rs`'s frame algorithm has moved since `:511-525`.

**Recommendation:** ROOT records items 1–10 as rulings when it rules on this brief, as it did for the K2b LEF restatement. `DESIGN.md` is not edited.
