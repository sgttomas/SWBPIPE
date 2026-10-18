# W1 coverage repair: the represented-Z width floor and the halfway scale corner

This is the implementation design for the sixth post-T3 shared epoch ([b7-plan.md](b7-plan.md), "After B7b", item 6). It repairs the causes of W1's first two fail-closed corners ([w1-member-load-certification.md](w1-member-load-certification.md) §4.4, "Corners that fail closed"; DEL-04-08 ScopeOfWork REQ-006 (b)). Every gate, search limit, verification law, tolerance and domain limit stays unchanged. The epoch starts with a census of how often current sections and maxima reach each corner (section 3).

**Status: designed, not implemented.** The census is designed here and executed after epoch 4 merges, when Agent 0 schedules it. The basis is main `ca8d6d5c23` plus the epoch-4 design (DEL-04-07 Design `u4b-u5b-implementation.md` at `c2e450ac7f`). Section 0 names what is taken from epoch 4.

**Revision 2** answers the first round of dual review (Claude Opus 5.5 and Codex). The changes:
- corner 1's change in what the fixed stop tolerance bounds is stated plainly, and put to the owner as a reading of REQ-003 (e) (sections 1.2, 11);
- the committed bytes that corner 1 moves are listed, and a replacement exhaustion control is specified (1.4);
- corner 2's byte argument gains its missing step, as a gating check (2.2);
- the census is made deterministic, with counterfactual attribution and pre-registered thresholds (3).

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

**Commission.** The owner, 2026-10-17: "We will need to address that W1 coverage gap at some point." b7-plan item 6: "The unit's design starts with a census of how often current sections and maxima reach each corner. It then repairs the cause and leaves every gate unchanged: no search, verification, tolerance or domain limit is relaxed. Its design proceeds alongside O4, which revisits the same section bases and W1-E admission. Implementation follows O4 unless that design shows the two should merge." ScopeOfWork REQ-007 (b) states the same duty, and AC-007 (b) its discharge conditions.

## Summary

- **Corner 1: the represented-Z width floor.**
  - *Cause.* On route L, the admitted lane's section modulus is a hull of two different binary64 representations of one quantity: hull(Ẑ, Î/c) (DEF-O `rows.component_stress`). The loaded-maximum search applies its stop rule, U − L ≤ 2^-64·L, to a field over that hull. Subdivision in t cannot remove the hull's own width, so the search exhausts.
  - *Reach.* A design-time computation from the product's preparation law gives the hull's relative width as 98.41 × 2^-64 at D = 0.2 m, t = 0.01 m, the value w1-member-load-certification.md §4.4 states. Across a 3,142-section grid, the width is at least 1 × 2^-64 for 99.0 % of sections; the median is 871 × 2^-64. It is zero only where Î/c is exactly representable, for example when D/2 is a power of two. Every committed loaded-member trial with an ordinary section, F6/partial, T18, T19 and T19/pre-K-D5 in both modes, ends in this exhaustion today. So the corner is effectively total for loaded members on ordinary sections.
  - *Repair.* The admitted lane's search is driven at the hull endpoint where the field is largest. The search's own convergence is decided there, on a point problem, by the unchanged rule and limits. The lane's returned lower end is evaluated over the whole hull, so the input width passes soundly to the enclosure, and the unchanged product gate judges it. When the hull is a point, the search is today's search.
  - *What it changes.* The fixed stop tolerance then bounds the search's convergence on the point problem, not the width of the lane's returned enclosure, which also carries the represented-Z hull (up to about 3,338 × 2^-64 relative on the design grid). Whether that row publishes is decided by the unchanged gate's 2^-53·|n| allowance.
  - *Owner reading.* No predicate, limit or tolerance value changes. Because REQ-003 (e) says "No search, verification, tolerance or domain limit is relaxed", this repair is put to the owner as a reading of that sentence (section 11), not self-certified.
- **Corner 2: the halfway scale.**
  - *Cause.* The receipt's `cases[].selection.body_scales` are copied from the native candidate's publication (`retained_wire.rs` `selection`, `owner.evidence().body_scales`). Every other scale-dependent field (the classes, the absolute bounds and the product gate itself) uses the scales the product certificate forms from the final published rows (`final_case.rs` `row_scales`/`couple`). The readers recompute that same final-row law at G5b. The two coincide unless a native candidate value and its final published value round to different binary64 numbers. That happens essentially only when the exact value is a binary64 halfway.
  - *Repair.* The receipt states the final scales the product gate used.
  - *Byte preservation.* Every committed successor passes G5b's equality, so its native scales equal the readers' law. The product gate's final scales must also equal that law. A gating check establishes this on every committed successor and census input before the change (2.2). Given that check, no committed receipt moves.
- **What does not change:**
  - the G5b check, every gate predicate, the stop rule's value 2^-64·L, the 256-live, depth-64 and 1,024-split limits, R7, DEF-O's text, D1 and the work limits;
  - every reviewed input. The repair moves priced source only, so it still needs its actual-source D3.
- **What does move** (1.4): the committed loaded members on interval-hull sections (their proof work and anything binding it; where their bending lower bound is nonzero, also U, n and the maximum evidence, with every gate verdict and work-limit outcome held by a regression condition, 3.3); the genuine-exhaustion control and its 13 corpus mutations, whose basis this repair removes; and the formerly refused controls, which now publish.
- **What remains after the repair.** A loaded maximum whose two-lane hull is too wide for the unchanged gate still falls back. That widening comes from the admitted-against-annular section difference, which every stress row inherits (stage 2 §4.4, last corner). It is outside REQ-006 (b), and the census measures it.
- **Merge with O4: no.** The corners share no law or code path with O4's Timoshenko and section-basis extension (section 8). The census runs as soon as epoch 4 merges; the repair is implemented after O4.

## 0. Basis and epoch-4 dependencies

Re-checked against epoch 4's merged head before the census runs.

| # | Assumption from epoch 4 | Where it matters |
|---|---|---|
| E4-1 | The route-L ordinary identity becomes `preview-physics-2`. `preview-physics-retained-1` inherits it by hash. DEF-O's meaning hash moves in epoch 4 (u4b §1.2). | Inputs and pins are read on epoch 4's bytes. This repair moves no reviewed input. |
| E4-2 | U4b adds σ⁺/σ⁻ and τ_v,max on straights as `ProductRecipe::NotCovered` rows, formed from the successor's own projected rows. They take no part in body scales, verified counts or the loaded-maximum search (u4b §4.1). | Corner 2's final scales are formed from the four scale kinds only (translation, rotation, force, moment), so the derived rows do not enter them. Corner 1 does not interact with them. |
| E4-3 | Straight maxima and their reader checks are unchanged by U4b (u4b §2.3, step 5: "Straight maxima, and their reader checks, are unchanged"). The failed-ordinary manifest grows from 51 to 66 rows per member. | The loaded-maximum recipe and its evidence are epoch 4's unchanged ones. |
| E4-4 | The corrected ordinary displacement fl(u + w + w2) leaves W1's trigger unchanged: ordinary results that are not `checks_passed`. | The census's trigger set is measured on the merged producer. |
| E4-5 | Epoch 4 regenerates the retained corpus, including the four `w1_member_interior_*` records and the member-load bases, and re-registers D3. | The census and the byte checks read epoch 4's regenerated inputs. The repair's D3 starts from epoch 4's profile, or from O4's if O4 lands first, as planned (section 8). |

## 1. Corner 1: the represented-Z width floor

### 1.1 The cause, from the code

- **The admitted lane's section modulus is a hull.** `product_certificate.rs` (about 630–645) sets `represented_z = (I_K/c).hull_point(Ẑ)`. I_K is the binary64 second moment, c = D/2, and Ẑ is the binary64 section modulus, each prepared as the correctly rounded value of the exact annulus (`prepare_product_annulus`). DEF-O states the rule: "admitted-K bending Z is hull(actual Z,I_K/c)".
- **Two roundings, one quantity.** Ẑ = RN(I/c) and I_K/c = RN(I)/c are two different roundings of the same exact I/c. Their relative difference is up to about 2^-52, and zero only when I_K/c is itself a binary64 number.
- **The search runs over the hull.** `loaded_maximum::search` (`product_certificate/loaded_maximum.rs`) receives that hull as `z`. Its node uppers use `corners(…, z, Div).hi`, the value at the smaller endpoint. Its lower uses `.lo`, the value at the larger endpoint. Its stop rule, `gap ≤ 2^-64·lower`, therefore asks the search to close a gap that contains (bending part)·(1/z_lo − 1/z_hi), whatever the subdivision depth. When that floor exceeds 2^-64·L the search can only exhaust. It returns `loaded maximum search exhausted`, and the case is unavailable (stage 2 §4.4, "Limits").
- **The annular lane is not affected.** Its z is a 1024-bit enclosure of the one exact I/c, so its width is far below the stop rule's resolution.

So the corner's cause is not the search's tolerance or capacity. A convergence rule is applied to a field whose input carries an irreducible interval. The rule is meant for the search's own approximation error.

### 1.2 The repair

**Monotonicity.** For fixed resultants, the field f(t; Z) = |N(t)|/A + ‖M(t)‖/Z is non-increasing in Z > 0. Over Z ∈ [z_lo, z_hi], the lane's enclosure of max_t f is therefore [max_t f(t; z_hi), max_t f(t; z_lo)].

**The admitted lane, when its represented-Z enclosure is not a point:**
1. **Drive.** The branch-and-bound runs on the point problem Z = z_lo. Every node upper, the `lower` used for the stop rule and for pruning, and the stop rule itself are today's, evaluated with `Enclosure::point(z_lo)`. The search's own convergence, U − L ≤ 2^-64·L, is decided on that point problem with today's tolerance. The point-coefficient control in stage 2 §4.4 shows such a search converges.
2. **Hull-valid lower.** Beside the driving lower, the search keeps `lower_hull`: the maximum over every evaluated control point of the objective evaluated with the whole hull z, whose lower end is the value at z_hi.
   - It is a lower bound of max_t f(t; Z) for every Z in the hull, since any point's value at z_hi is at most the maximum at z_hi, and that is at most the maximum at any smaller Z.
   - It costs one more objective evaluation per evaluated point, which can share the axial and bending terms.
3. **Result.** On convergence the lane returns [lower_hull, U]. U is the driving search's maximum live upper, which bounds max_t f(t; z_lo) and so the maximum over the whole hull. The enclosure is sound, and its width is the search's convergence width plus the input hull's own effect.
4. **Exhaustion** under the unchanged limits returns the named kernel failure, as today. The diagnostic `{row, member, lower, upper}` reports [lower_hull, the maximum upper over live nodes and the unexpanded parent], an enclosing interval of the lane's maximum over the hull, as stage 2 requires. Its encoding and G5 checks are unchanged.

**When the represented-Z enclosure is a point** (z_lo = z_hi, for example D/2 a power of two): exactly today's code path, with the same operations, work and bytes. The four committed `w1_member_interior_*` records use such sections and keep their bytes.

**The annular lane** keeps today's search on its tight z. Its z is an enclosure of one exact quantity, not a hull of two declared representations, so the drive and hull split does not apply. Its work and bytes are unchanged.

**What decides publication.** The hull of the two lanes' enclosures is projected and gated as today: n = RN64(H(hull(k, s))) and the unchanged predicates (stage 2 §4.4). A hull too wide for the predicate fails the gate. That is the unchanged gate deciding, not a search failure.

**DEF-O is unchanged.** Its `rows.maximum` states what is enclosed: "directed branch-and-bound circular maxima of |N(t)|/A+‖M(t)‖/Z over t in [0,1]", with the admitted lane's Z the `component_stress` hull. DEF-O does not state the stop rule. The repaired search encloses exactly that. No reviewed input moves, so the preview successors' DEF-O-bound hashes do not move.

**What changes, stated plainly.**
- **Unchanged:** the values of the stop tolerance, the limits and every gate predicate.
- **Today's postcondition.** Stage 2 §4.4 states the stop rule with its purpose: "Stop. When U − L ≤ 2^-64·L. A hull width ≤ 2^-63·|n| suffices for the RN projection to pass." On a successful search, the lane's returned enclosure is at most 2^-64·L wide.
- **After the repair,** on an interval hull:
  - the stop rule bounds the convergence of the search's point problem at z_lo;
  - the lane's returned enclosure [lower_hull, U] is that width plus the represented-Z hull's own effect, up to about (bending share) × 3,338 × 2^-64 relative on the design grid;
  - whether the row publishes is decided by the unchanged gate, whose allowance includes 2^-53·|n| (`final_case.rs` `gate`).
- **Exhaustion** still prohibits publication.
- **The trade.** Today the search refuses every such member whatever the gate would say. After the repair the unchanged gate says.
- **Soundness.** The node uppers at the point z_lo equal today's uppers over the interval, because `loaded_maximum.rs` divides the bending upper by z_lo in every term. Pruning on the z_lo lower is sound for U. The hull-valid lower is a valid lower bound over the whole hull. No published guarantee changes.
- **Why it goes to the owner.** This is a change in what a fixed search tolerance bounds. REQ-003 (e), an owner-approved commitment, says "No search, verification, tolerance or domain limit is relaxed to obtain a bound". So the design puts its reading to the owner (section 11) and proposes the matching edit to stage 2 §4.4's text (section 10).

### 1.3 Work and memory

- **Work.** The extra hull-valid evaluations are counted at actual cost in `NumericWork`, before completion is sealed, with nothing counted as zero (stage 2 §5 and R-1). A point-hull search does no extra work.
- **Memory.** No search node grows. `lower_hull` is one more `Endpoint` in the search frame. The live-node backing, L4's 1 MiB cap and the lanes' sequential execution are unchanged. The actual-source D3 prices the frame growth and the TEXT positively (section 6).

### 1.4 Committed bytes that move, and the exhaustion control

- **Loaded members on interval-hull sections that converge today.** `w1_member_load_product_consuming_successor` uses the D = 0.2 m, t = 0.01 m pipe with an axial global-x load. Its bending share is zero, so the hull adds no width and the search converges today. Under the repair its admitted lane takes the driven path, and the extra hull-valid evaluations change its metered proof work.
  - Its receipts' work counters, and the hashes binding them, move: the `w1_member_load_*` corpus entries, and any multi-member or values-failure entry built on the same pipe.
  - When the bending lower bound is zero at every evaluated point (an axial load), the driving and hull lowers coincide and the search follows today's path. Then every number, class, selection and maximum value stays the same, and a check over the regenerated entries enforces it (`w1_hull_repair_moves_only_work_on_converging_members`).
  - A converging member whose bending lower bound is nonzero at some evaluated point may stop at another point, so U, n and the maximum evidence can change. The census classifies each committed member (C1-a), and any such move is listed with its reason.
  - The census lists every such committed member (section 3.2, input C1-a).
- **The genuine-exhaustion control.**
  - `w1_domain_stage1_tests.rs` `w1_member_load_exhaustion_retains_typed_interval_work` produces `w1_member_exhaustion_{sparse_interactive,dense_scrutiny}`. The corpus describes them as "ACTUAL MAINTAINED PRODUCER OUTPUT … whose span-maximum search exhausts".
  - 13 `member_exhaustion_diagnostic_*` mutations in `fixtures/results/retained_precision_cases.json` are built on them, and so are `declared_member_load_workspace.rs` and the genuine-exhaustion test in `tests/test_declared_member_load_reader.py`.
  - Its model (`fixtures/product_preview/numerical_sensitive_torsion_model.json`) is the D = 0.2 m section with a global-y UDL, exactly the corner this repair removes, so it is expected to stop exhausting.
  - The replacement, in order of preference:
    1. **A genuine input.** One that still exhausts under the unchanged limits on a point-hull section: a near-degenerate rotating moment with varying axial force, the theoretical case stage 2 §4.4 names. The census's C1 sweep includes candidates (section 3.2).
    2. **A hook-driven input.** If no genuine input is found, the existing test capacity hook (`loaded_maximum::hooks::with`), exposed through a dev-only feature as RF's operation counter is, drives a real member to exhaustion at reduced limits. The corpus entry then says so, and does not claim production limits. The feature stays out of the registered compiled identity, as `mutation-controls` does.
  - Either way the 13 mutations are rebased on the new base by the corpus writer, and the readers' expected outcomes are unchanged in kind.
- **The two corner controls** in VER-003 change from refusals to publications (section 5). Their new successors join the corpus.
- **Nothing else moves for corner 1.** Point-hull sections, which include the four `w1_member_interior_*` records, the annular lane, route E, the source route and every nodal-only successor keep their bytes.

## 2. Corner 2: the halfway scale

### 2.1 The cause, from the code

- **The product certificate's gate uses final-row scales.** `final_case.rs` `row_scales` forms each body's translation, rotation, force and moment scales from the final published rows (`spent.normalize(unit, value)`). `check_intervals` couples them with the body extent (`couple`, binary64 `max` with `extent·ro`, `tr/extent`, `mo/extent`, `extent·fo`) and applies the p = 512 floor. It then gates, classifies and bounds every row with those scales.
- **The readers recompute that law.** The Rust reader (`retained_precision.rs` `case_numeric` and `g5b`) and the verifier recompute exactly that law from the published rows. G5b then requires `selection.body_scales` to equal it bit for bit.
- **The receipt states native scales instead.** `retained_wire.rs` `selection` (about 1257–1300) writes `body_scales` from `owner.evidence().body_scales`. These are the native candidate's publication scales (`adaptive.rs` `publication_with`, from the native candidate's binary64 values), not the final rows'.
- **Where they diverge.** A native candidate value is accurate to about 2^-p relative, and the final value is RN64 of the product's projection. The two round alike unless the exact value lies within the native error of a binary64 rounding boundary. For an exact halfway they round apart about half the time, so a body maximum that is an exact halfway gives receipt scales one ulp away from the reader's.
- **Observed instances:**
  - the 73/24 Pa asymmetric control, whose force maximum (7/2)Z and moment 7Z are halfways (`w1_loaded_scale_halfway_keeps_the_existing_g5b_refusal`);
  - the three-case cap witness `witness_w1_member_load_three_case_three_basis_128`, which "truthfully retains its exact G5b scale refusal" (stage 2 §13).
- **Why it withdraws everything.** The G5b refusal is a precommit fault, so it withdraws the whole successor (REQ-006 (b)).

The corner is a provenance inconsistency in one receipt field. The gate, the classes and the bounds were already judged on the final scales.

### 2.2 The repair

- The product certificate keeps the final per-body scales it computed in `check_intervals`, coupled and floored, as part of its proof, and exposes them with the verdicts. Following O4's layout discipline (o4-source-extension.md §4.2), the exposure leaves every serialized proof structure and capacity unchanged, including `capacities[6]` (the scales buffer). It reuses the existing scales buffer, or else a separately owned copy that no receipt counter records. A pin test asserts it.
- `selection` writes `body_scales` from those final scales, for selected cases and for combinations alike. Both call `selection` with the product certificate in hand.
- The native evidence's scales stay native evidence. They still feed the native publication certificate unchanged, and no other receipt field changes source.

**Consequences:**
- The precommit G5b body-scale equality now holds by construction, wherever the product's final law and the readers' law agree. They are the same binary64 law over the same published rows. Any disagreement is still refused by the unchanged precommit G5b and falls back, so it fails closed.
- A former halfway withdrawal publishes when every gate passes. If a row's gate fails under the final scales, that case alone is unavailable with its named cause, as for any gate failure. The halfway no longer withdraws every case.
- **No committed byte moves, given one gating check.** G5b's acceptance of every committed successor shows that its native scales equal the readers' recomputed law. The repair writes the product gate's final scales instead. So byte preservation also needs the final scales to equal the readers' law on every body and kind, including kinds no row's class depends on. The two computations differ in code:
  - The product's `row_scales` (`final_case.rs` about 1136–1314) takes native rows whose native class is not input-derived, plus support components into the force and moment slots. It normalizes the final values, couples with `couple`, and applies the floor from native evidence.
  - The readers' `case_numeric` (`retained_precision.rs` about 3765–3825) and `g5b` (about 4344–4370) take the published rows of each scale kind, less prescribed components. They couple in the same binary64 form and apply the floor from the receipt's `floor`.

  These must be shown to select the same row sets and the same floor. The implementation argues that row-set mapping kind by kind. A gating test (`w1_final_scales_equal_reader_scales`) asserts final scales = readers' scales on every committed successor and every census input, before `selection` is switched. If any body or kind differs, the repair stops for diagnosis. Such a difference would be a second inconsistency between gate and reader, and it would need its own design; it would not justify writing a scale the reader rejects.
- **Not a relaxation.** The G5b check, every predicate and every law are unchanged. The receipt stops stating a value the gate did not use.

## 3. The census

The census measures, through the actual producer, how often current sections and maxima reach each corner. It also measures what the repair turns them into before the repair is committed. It is executed after epoch 4 merges, when Agent 0 schedules it; it builds and runs the product, so it is not run in this design PR.

### 3.1 Already known (design-time computation, no product build)

These figures recompute the kernel's preparation law in exact rational arithmetic, with π to 110 digits. The design's light Python (`/tmp` scratch; not committed) reproduces stage 2's 98.41149699 × 2^-64 for D = 0.2 m, t = 0.01 m exactly:

| Section (D, effective wall) | Hull relative width, × 2^-64 |
|---|---|
| 0.2 m, 0.01 m | 98.41 |
| 0.168 m, 0.007 m (the most common committed route-L section) | 289.68 |
| 0.1683 m, 0.00711 − 0.000889 m | 382.23 |
| 0.114 m, 0.006 m | 405.90 |
| 0.12 m, 0.01 m | 1,512.83 |
| 0.02 m, 0.002 m | 142.51 |
| 0.25 m or 0.5 m, 0.01 m (D/2 a power of two) | 0 |

- **The grid.** D from 20 to 1,500 mm in 7 mm steps, with even walls from 2 to 30 mm and 2t < D: 3,142 sections.
  - 99.0 % have width ≥ 1 × 2^-64, and 98.3 % have width ≥ 16 × 2^-64;
  - p10 is 160, the median 871, p90 2,037 and the maximum 3,338;
  - 30 sections have width 0.
- **The trials.** Committed evidence (stage 2 §4.6) already shows every committed loaded-member trial on an ordinary section exhausting at this corner: F6/partial, T18, T19 and T19/pre-K-D5, in both modes, with "proof → association → loaded maximum search exhausted".
- **Reading.** A loaded member reaches corner 1 whenever its governing maximum's bending share b/(a + b) exceeds about 1/width. On ordinary sections that is essentially every loaded member with bending.

### 3.2 What the census runs

**Order.** The census measures what the repair would do before the repair is committed. So its first commit, on the repair's branch, adds the two repairs behind test-only switches, plus the probe. Production paths are unchanged until the census is reported and the owner has read section 11's question.
- The switches are a `#[cfg(test)]` hook in the kernel (the existing `loaded_maximum::hooks` module is the precedent) and a dev-only cargo feature where `product_physics` tests need it, as RF's `exact-operation-count` is.
- Census targets are `#[ignore]` tests in `core/product_physics`, re-runnable on demand.

**What the probe records.**
- **C1, per loaded member:**
  - the represented-Z width w (exact, from the prepared bits, in units of 2^-64 relative);
  - the governing bending share b = (‖M‖/Z)/(|N|/A + ‖M‖/Z) at the lane's best point;
  - today's admitted- and annular-lane search outcomes, with splits, live nodes and depth;
  - with the corner-1 switch: both lanes' enclosures, the hull width relative to |n|, n, the gate outcome and, on failure, the failing predicate;
  - on a post-repair gate failure, the counterfactual gate outcomes defined under the decision rules (3.3): (a), (b) and (c) individually, and the combined runs (a)+(b) and (a)+(c). Together these distinguish *joint* from *other*;
  - for a member that converges today, its before-and-after verdicts, status and counted work, for the regression condition (3.3);
  - the case's resulting status and cause.
- **C2, per case:**
  - the native, final and readers' scales per body and kind, and their equalities;
  - the precommit G5b outcome;
  - with the corner-2 switch: the case's resulting status, and every row's gate verdict under the final scales.
- **Not reached.** A case or member that never reaches the measured stage is recorded as `not_reached`, with the stage it stopped at, and kept out of that stage's denominator. Examples: an ordinary `checks_passed` case (no W1 trigger), a preparation refusal, or, for C2, a case ending at C1's exhaustion. C2 is therefore also measured on the C1 inputs with the corner-1 switch on, so that observations C1 had cut off are reached.

**Inputs** (deterministic; each list is fixed in the census module before the run):
- **C1-a: committed route-L member-load inputs.**
  - the group-3 census trials F6/partial, T18, T19 and T19/pre-K-D5, both modes;
  - every corpus member-load base, including `w1_member_load_*`, the multi-member and values-failure entries and `w1_member_exhaustion_*`;
  - the four `w1_member_interior_*` records;
  - the cap-law input and both 4 MiB witnesses (`witness_w1_member_load_three_case_three_basis_128` and `..._dyadic`; sparse and dense, run as separate cap commands under the host's slot rules).
- **C1-b: a generated sweep.**
  - **Sections.** 40 (D, t) pairs: every 79th pair of section 3.1's grid in its stated order (D ascending, then t), starting with the first, which gives 40 of the 3,142. Three D/2-power-of-two sections are added (D = 0.25, 0.5 and 1.0 m with t = 0.01 m), and so are the seven committed sections of section 3.1's table.
  - **Shapes.** Five route-L shapes, each with one loaded member of length 2 m plus an independent soft-spring member that makes the ordinary result Sensitive, as stage 2 §4.4's controls do:
    - S1: simply supported, global-y UDL;
    - S2: cantilever, global-y UDL;
    - S3: fixed–fixed, global-y UDL;
    - S4: skew member (3,4,12)/13 with UDL components on two global axes;
    - S5: S1 plus an end axial force sized so that b ≈ 0.5 at midspan.
  - **Magnitudes.** UDL 3.5 N/m, with the S5 axial force derived from it.
  - **Both solver modes.** That is (40 + 3 + 7) × 5 × 2 = 500 invocations through the private preparation, native, freeze and native-reader path.
- **C2-a: committed W1 inputs on both routes.**
  - every corpus base and mutation base;
  - the successor fixtures;
  - `fixtures/product_preview/w1_controls/`;
  - the 54 B4/D1 trials;
  - N06 and CX-F;
  - RV120 and the route-E successors;
  - C1-a.
- **C2-b: a generated sweep.** 64 route-L invocations per mode, each with an independent soft-spring member that makes the ordinary result Sensitive, so that W1 is reached (an ordinary `checks_passed` case is `NotRequired`; PP `lib.rs` about 3471).
  - **32 constructed halfways,** following the maintained Z-scaled control. Statically determinate cantilevers and simply supported members with long-significand section operands (the stage-2 point-hull section's Ẑ). Their nodal forces and end moments are chosen as (2k + 1)/2 multiples of Ẑ-scaled values, so that the exact force or moment body maximum is a binary64 halfway. The census generator verifies each by exact rational evaluation before the run, and keeps only those it confirms.
  - **32 decimal controls:** the same shapes with lengths in {1.5, 2.5, 3.7, 6.1} m, forces in {1.1, 2.3, 4.7, 9.9} kN and the committed 0.168 m, t = 0.007 m section.
  - **Plus** the 73/24 Pa control and the cap witness.
  - **Pre-registered coverage:** at least 24 confirmed halfway cases per mode must reach the native-versus-final scale stage. Otherwise the sweep is extended by the same rule before any conclusion is drawn.

**Report** (in the repair's implementation PR, and as one revision of this design's section 3; no separate run record):
- **C1:** with the denominators stated (members reaching the search):
  - the count reaching corner 1 today;
  - after the repair: the counts converged, passing the gate, failing the gate (split by attribution class) and exhausted;
  - each tabulated against w and b.
- **C2:**
  - the native ≠ final count per input set;
  - the final ≠ readers count, which must be zero (2.2);
  - precommit G5b withdrawals today, by cause;
  - after the repair: the cases that publish, and any row that fails a gate under final scales.
- **Baseline.** The pass rate of the same converged members' ordinary stress rows (the 20 element-local stress rows per member, measured in the same runs) gives the inherited-corner baseline, in place of stage 2's rough-model figure.

### 3.3 Decision rules (pre-registered)

**Implementing corner 1's repair.** Today every member that reaches corner 1 is unavailable. After the repair each such member is either gate-decided or still exhausted, so for those members the repair cannot remove a capability, and the census does not gate implementation on their pass rate. That argument does not cover members that converge today on interval-hull sections. They take the driven path too (1.4): their work rises, and where the bending lower bound is nonzero, U, n and the maximum evidence can change. So implementation waits for the owner's reading of REQ-003 (e) (section 11) and for the regression condition below. Then:
- **Regression condition for already-converging members.** For every C1-a member that converges today, the census reports before and after: the gate verdict, class (relative or absolute) and published absolute bound of each certified row; the case's selection and status; the counted work against W1's case and invocation limits (`CASE_LIMIT` = 20e9 and `INVOCATION_LIMIT` = 60e9, `retained_wire.rs` about 67–68; `COMBINATION_CASE_LIMIT` = 20e9 for any member in a combination successor, `retained_product.rs` about 6390); and the 4 MiB stack witnesses' outcomes. A row that passes today and fails after the repair, a case that changes from publishing to withdrawn, or a work-limit refusal that does not occur today stops the repair for diagnosis. It does not proceed under a changed gate. Numerical moves that leave every verdict and refusal unchanged, including any change of row class or absolute bound, are listed in the implementation PR with their reason (1.4).
- A post-repair exhaustion on an ordinary section (w > 0) is classified. A near-degenerate maximum is the stage-2 theoretical case. Anything else is unexplained and stops the repair for diagnosis.
- **Attribution of each post-repair gate failure.** Counterfactual re-runs of the same gate under test switches, with no product effect:
  - (a) the admitted lane's Z set to the single value Ẑ;
  - (b) both lanes given the same section, the admitted lane's;
  - (c) both lanes given the same whole readout (section and material, Ĝ against the exact E/ν form), so the lanes' solution enclosures differ only by their own arithmetic.
  
  Write A for "(a) clears", B for "(b) or (c) clears" and AB for "(a) together with (b) or (c) clears". Each failure falls in exactly one class:

  | Class | A | B | AB | Enters the escalation numerator |
  |---|---|---|---|---|
  | *Z-hull only* | yes | no | — | yes |
  | *either* (each change alone suffices) | yes | yes | — | yes |
  | *lane only* (recorded as section or material) | no | yes | — | no |
  | *joint* | no | no | yes | yes |
  | *other* | no | no | no | stops the repair for diagnosis |

  The numerator counts the failures the Ẑ narrowing would clear, alone or together with the lane change.
  - A *Z-hull* failure is possible on its own. The gate's half-width is the hull's half-width plus the projection's rounding |n − mid| ≤ ulp(n)/2, which is itself up to 2^-53·|n|. So a hull half-width of up to about 1,669 × 2^-64 can fail when the rounding is near its maximum. The counterfactual measures how often.
- **Escalation, pre-registered.** If *Z-hull only* plus *either* plus *joint* failures are at least 5 % of converged C1-b members on sections with w > 0 (the boundary itself escalates), the design returns to HELP_HUMAN with the table. The next lever would narrow DEF-O's admitted Z to the single Ẑ, which changes a reviewed definition's meaning and the verified claim, so it needs a ruling. Below 5 % that lever is not raised. Any *other* failure stops the repair for diagnosis.

**Implementing corner 2's repair.** It cannot remove a capability (2.2). It is implemented once the census shows final = readers on every C2 input (the gating check). Any native ≠ final instance on a committed successor contradicts section 2.2's premise and stops the repair for diagnosis.

### 3.4 What it needs to run

- **Timing.** After epoch 4 merges, on a scratch worktree from that merge, or from O4's head if O4 has landed (section 8).
- **Build and host.** It builds `frame_kernel` and `product_physics` tests in the debug profile. It uses the qualification host's heavy-run slot and memory rules, within the 64 GiB test ceiling. The census uses private drivers, so it needs no registered build identity. The cap witnesses run as separate commands with their 4 MiB stacks.
- **Pilot.** A pilot of ten generated invocations per sweep measures time and memory before the full run. No duration is assumed here.
- **What it writes.** Nothing under `validation/references` or the committed fixtures. The probe and switches land on the repair's own branch as test-only code and stay as `#[ignore]` targets that can be re-run.

## 4. What stays unchanged

- the loaded-maximum stop rule's value (2^-64·L), its 256-live, depth-64 and 1,024-split limits and its failure propagation. What the rule bounds on an interval hull changes, as section 1.2 states, subject to the owner's reading (section 11);
- the product gate's predicates, DEF-O's `acceptance.final`, G7's `EXTREMA_BOUNDS` and the evidence construction (stage 2 §4.4);
- R7, the alpha and congruence laws, E's factor 69 and P − 16, the stop rule at 2p, the precision ladder and `projection_retry: false`;
- G0–G8, including G5b's equality and G5's diagnostic checks;
- D1, the case and invocation work limits (20e9 and 60e9), M and its margins;
- every reviewed input, including DEF-O's text, SCHEMA, the tables and DEF-C;
- route E, the source method, J2 and the J7 census;
- every frozen reference and the frozen UI benchmark.

## 5. Mutants and killing tests

Each is introduced into a scratch copy, run, reverted and recorded in the PR. Tests are new unless marked existing.

| Fault | Killed by |
|---|---|
| The driven search returns its point lower (at z_lo) as the lane's lower: unsound, it excludes the z_hi value | `w1_hull_search_lower_encloses_the_far_endpoint` (an exact rational member whose maximum at z_hi is below the driven lower; the enclosure must contain it) |
| The stop rule applied to the hull gap (today's behaviour) | `w1_represented_z_hull_converges_on_ordinary_sections` (D = 0.2 m, t = 0.01 m, the stage-2 boundary member: converges, and its outcome is decided by the gate); replaces `w1_loaded_maximum_refuses_irreducible_represented_section_width` (existing), which becomes this positive test |
| The split applied to a point hull, moving work and bytes | `w1_point_hull_search_is_unchanged` (the four `w1_member_interior_*` records byte-identical, work included) |
| The split applied to the annular lane | `w1_annular_lane_search_is_unchanged` (annular-lane work and enclosures byte-identical on every committed loaded input) |
| A limit raised or shared differently; exhaustion publishes | `w1_loaded_maximum_exhaustion_widens_and_refuses` (existing, with capacity hooks) on a non-point hull; the limit constants pinned |
| The hull-valid lower omitted from the exhaustion diagnostic | `w1_hull_exhaustion_diagnostic_encloses_the_hull_maximum` |
| The extra evaluations unmetered | `w1_hull_search_work_is_counted` |
| Receipt scales still native | `w1_loaded_scale_halfway_publishes` (the 73/24 Pa control in both modes: precommit G5b passes, and every row's gate verdict is as the final scales give); replaces `w1_loaded_scale_halfway_keeps_the_existing_g5b_refusal` (existing) |
| Receipt scales final but not coupled or floored as the gate used them | `w1_receipt_scales_are_the_gate_scales` (a p = 512 case with an active floor, and a multi-node body with active coupling: the receipt equals the product proof's scales and the readers' recomputation) |
| Native evidence scales replaced, or used for anything new | `w1_native_scales_stay_native_evidence` (the native publication certificate's inputs unchanged) |
| A committed byte moves beyond section 1.4's list | the existing receipt-byte tests over the corpus, successor fixtures, R1 records and route-E successors; `w1_committed_publications_preserve_all_but_authorized_definition_hashes` (existing), whose exceptions gain exactly section 1.4's entries; `w1_hull_repair_moves_only_work_on_converging_members` |
| Final scales written where they differ from the readers' law | `w1_final_scales_equal_reader_scales` (the gating check of 2.2, over every committed successor and census input) |
| The genuine-exhaustion control left on a base that no longer exhausts | `w1_member_load_exhaustion_retains_typed_interval_work` (existing), rebuilt on section 1.4's replacement; the 13 `member_exhaustion_diagnostic_*` mutations rebased |
| The cap witness still withdrawn | `witness_w1_member_load_three_case_three_basis_128` (existing, `#[ignore]`): its expectation is derived from the gate verdicts under the final scales, row by row, not copied from a census observation. The reason for the change is the repaired provenance |

The two corner tests named in VER-003 change from asserting the refusal to asserting the repaired outcome. That edits a test expectation, so the reason is stated: the cause each refusal recorded is repaired, and the gates are unchanged. AC-007 (b) asks for exactly this ("the decline tests they retire are replaced by admission and refusal tests").

## 6. D3

- No reviewed input moves, and the compiled identity and reader layouts are unchanged.
- **Priced source moves:**
  - the kernel's `loaded_maximum.rs`: its frame and TEXT;
  - `final_case.rs`: the retained final scales, one `[f64; 4]` per body, at most 32 bodies per case, times the existing multiplicities;
  - `retained_wire.rs` `selection`: TEXT.
- The repair therefore re-registers on its actual source (D3; REQ-004).
- The delta is small and positive: a few hundred bytes of frame and scale storage per attempted case, plus TEXT.
- If the evaluation does not fit, the measured shortfall goes to HELP_HUMAN before any budget change (Decision 22). Nothing here expects that.

## 7. Readers and verifier

No reader or verifier change. G5b's body-scale equality and the loaded-maximum diagnostic's G2 and G5 checks are unchanged. The readers do not replay the search (DEF-O `trust.validation`). The successors these repairs newly publish are read by the unchanged gates. Their corpus entries and native, wasm and Python outcomes are added by the corpus writer: the D = 0.2 m loaded member and the 73/24 Pa control, in both modes.

## 8. Relation to O4: sequence, not merge

The decision and its reasons are in [o4-source-extension.md](o4-source-extension.md) §8. From this side:

- **No shared cause.**
  - Corner 1 lives on route L, where loaded members exist. O4 extends route E and the source route; route E admits no member load, and route L carries no shear model or allowance.
  - Corner 2 is one field's provenance. O4's new successors inherit the repair once it lands.
- **The census does not wait for O4.** It needs only epoch 4. It can run while O4's independent reference unit is prepared.
- **Coupling rules** (binding on O4, restated): O4 does not change the represented-Z hull, the loaded-maximum search or the provenance of `body_scales`. O4's controls avoid halfway body maxima, or record their G5b withdrawal as the expected pre-repair fallback.
- **Rebase.** The repair is implemented on O4's head. Its shared files are `retained_wire.rs` `selection` (`body_scales` beside O4's `section_terms`) and the registered profile. It re-registers on that head.

## 9. Verification and independent review

**Checks on the repair's head:**
- the mutants of section 5;
- the census report, re-run on the head for the C1 and C2 inputs that changed outcome;
- the suites `select_affected_checks.py` selects: frame kernel (`retained_k4`), product physics (the W1 census, facade and stage-1/stage-2 tests), result_export and the desktop wasm equivalence;
- the corpus in native Rust, wasm and Python;
- the actual-source D3 (`generate --check`);
- the hosted `harness` on the exact head.

**Independent numerical review checklist:**
1. The monotonicity argument and the hull-valid lower: sound for every Z in the hull, under directed rounding.
2. That the driven search's uppers, pruning and limits equal today's in value, that the stated change in what the stop rule bounds is accurate (1.2), and that exhaustion still fails closed.
3. That a point hull and the annular lane are byte-identical to today.
4. That DEF-O's text describes the repaired enclosure unchanged.
5. That the final scales are exactly the scales the gate used, coupled and floored, and the kind-by-kind row-set mapping to the readers' law (2.2).
6. That only section 1.4's committed bytes move, each for its stated reason.
7. The census's counterfactual attribution and its pre-registered rules (3.3).
8. The D3 delta.

## 10. Proposed ScopeOfWork and b7-plan text

These edit commitments, so they are proposals. The owner decides them. They are written against the ScopeOfWork as edited by epoch 4 and, if it lands first, by O4 (o4-source-extension.md §10).

**DEL-04-08 ScopeOfWork** (on the repair's discharge):
- **REQ-006 (b).** Delete the item, the two corners being repaired. Optionally record, under REQ-006 (d) or as a sentence in REQ-001's route-L domain: "A loaded member's maximum whose certified hull is too wide for the unchanged gate keeps its ordinary result; the case is unavailable with its named cause." That records the inherited corner as a stated limit, if the owner wants it stated.
- **REQ-007 (b).** Delete on discharge.
- **AC-003,** third bullet. Remove "the two W1 corner cases … keep their internal refusals and fall back". After O4 the bullet carries nothing else, so it is removed whole.
- **VER-003.** Replace `w1_loaded_maximum_refuses_irreducible_represented_section_width` and `w1_loaded_scale_halfway_keeps_the_existing_g5b_refusal` with `w1_represented_z_hull_converges_on_ordinary_sections` and `w1_loaded_scale_halfway_publishes`.

**b7-plan,** item 6: the pointer sentence in o4-source-extension.md §10. When the repair lands, b7-plan's item-6 paragraph is replaced by a one-line "as built" pointer, as items 1–3 were.

**DEL-04-08 Design `w1-member-load-certification.md` §4.4** (proposed; edited when the repair lands, as an as-built revision):
- the "Stop" bullet becomes: "When U − L ≤ 2^-64·L on the search's point problem. On the admitted lane, whose represented Z is the hull(Ẑ, Î/c), the search is driven at the hull's smaller endpoint, and the lane returns [the lower evaluated over the hull, U]; the unchanged gate decides the projected row."
- the "Corners that fail closed" paragraph's first two bullets are replaced by a pointer to this design's repairs.

**`deliverable.yaml`.** No dependency change. The DEL-04-01 need already names "the loaded-member maximum search with its fixed limits"; its limits do not change.

## 11. Rulings

**Applied:**
- the owner's "We will need to address that W1 coverage gap at some point" (2026-10-17);
- b7-plan's "no search, verification, tolerance or domain limit is relaxed";
- stage 2's rulings (option B, route-L maxima option 1, R-1), all unchanged.

**Decided in this design,** for the independent review:
- drive the admitted-lane search at the hull's binding endpoint and carry the hull to the output (1.2);
- write the gate's final scales to the receipt (2.2);
- the census plan and its decision rules (3);
- sequence, not merge (8).

**Reserved, for the owner through Agent 0:**
- **REQ-003 (e)'s reading for corner 1.** Does applying the unchanged loaded-maximum stop rule to the search's point problem, while carrying the represented-Z hull soundly into the lane's enclosure for the unchanged gate to judge (1.2), satisfy "No search, verification, tolerance or domain limit is relaxed to obtain a bound"? Both reviewers judged it a sound repair that changes no predicate, limit or tolerance value; the round-1 Opus review asked that the owner see the reading rather than have it self-certified. If the answer is no, two alternatives remain:
  - **Keep corner 1.** Every loaded member on an ordinary section keeps falling back, about 99 % of sections by section 3.1. The census still runs and reports.
  - **Narrow DEF-O's admitted Z to the single Ẑ.** The search then runs on a point and the stop rule keeps its present postcondition, but the admitted readout no longer certifies the Î/c reading. That changes a reviewed definition's meaning, which needs its own ruling.
- Corner 2's repair does not depend on this answer and can proceed alone.

**Contingent, to HELP_HUMAN:** narrowing DEF-O's admitted Z, if the census's pre-registered escalation rule fires (3.3), and a D3 shortfall.

**Reserved and untouched:**
- the ScopeOfWork edits of section 10;
- Decision 22;
- TBD-001 and TBD-002;
- G10 and B8;
- the frozen UI benchmark;
- release.

## 12. Blast radius

**Code:**
- **Frame kernel (DEL-04-01):** `structural/retained/product_certificate/loaded_maximum.rs` (the driven search and the hull-valid lower) and `final_case.rs` (retaining and exposing the final scales; the admitted-lane call site); tests in `tests/retained_k4/`.
- **Product physics (DEL-04-07):** `retained_wire.rs` `selection` (the scale source); `retained_memory.rs` (registered profile); the W1 census and test files (`w1_domain_stage1_tests.rs`, `w1_source_census_tests.rs`, `retained_facade_tests.rs`), including the census targets.
- **Pricing:** `tools/retained_pricing` (owners and rules) and `retained-pricing-rules.md`.

**Fixtures:**
- new corpus bases (the repaired controls, in both modes), with their R1 outcomes and desktop wasm inputs;
- section 1.4's moves:
  - the work counters and binding hashes of committed loaded members on interval-hull sections;
  - where such a member's bending lower bound is nonzero at an evaluated point, its U, n and maximum evidence, under the unchanged gate and the regression condition (3.3);
  - the genuine-exhaustion control with its 13 mutations rebased;
- the census sweep inputs only as generated test inputs, not as committed records.

**Must stay byte-identical:**
- every other committed successor, corpus base and R1 or R2 record (corner 2 moves none, given the gating check). Every gate verdict, selection and work-limit outcome of an already-converging committed member stays the same, or the repair stops (3.3);
- every ordinary and pressure-free byte;
- route E, J2 and the J7 pin;
- every reviewed input;
- `validation/references/**` and the frozen UI benchmark.

**Deliverables:** DEL-04-08, DEL-04-01 and DEL-04-07. The readers (DEL-05-02) are unchanged.
