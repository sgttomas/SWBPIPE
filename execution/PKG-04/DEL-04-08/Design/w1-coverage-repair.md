# W1 coverage repair: the represented-Z width floor and the halfway scale corner

This is the implementation design for the sixth post-T3 shared epoch ([b7-plan.md](b7-plan.md), "After B7b", item 6). It repairs the causes of W1's first two fail-closed corners ([w1-member-load-certification.md](w1-member-load-certification.md) §4.4, "Corners that fail closed"; DEL-04-08 ScopeOfWork REQ-006 (b)). Every gate, search limit, verification law, tolerance and domain limit stays unchanged. The epoch starts with a census of how often current sections and maxima reach each corner (section 3).

**Status: as built. The census has reported (3.5); corner 2 is implemented through #1483; the owner ruled Z1 not adopted (2026-10-20, "I approve 1A, 2A, 3A."; section 11), and corner 1 is implemented through #1484.** The basis is main `257b44b910`: epoch 4 (#1461), O4 (#1468, merged at `fa019d9cef`) and the stack-headroom repair (#1476, b7-plan item 5a, D3 at `6b90748dec`) are merged. Section 0 records each assumption as re-checked against that source. Corner 2 landed first as its own PR (#1483). Corner 1's production switch, with the decided REQ-003 (e) clause and the owner's section 10 edits, is #1484, stacked on it.

**Revision 12** records the owner's rulings of 2026-10-20 ("I approve 1A, 2A, 3A."): Z1 is not adopted, so corner 1 lands as designed (1A); section 10's proposed ScopeOfWork edits are decided, with their placement (2A); and the REQ-006 (e) amendment is applied on main (3A, w1e §6.1). Corner 2 lands as its own PR first.

**Revision 11** answers the Claude review of the census report (second round): where native and final scales diverge is restated with the support-magnitude cause the census shows dominates (Summary, 2.1, 3.5), the gating check's scope is stated as built (2.2, 3.5, section 5), the regression condition's stack-witness part is reported separately (3.5), and the C1-b de-duplicated escalation, the pre-registered count, the self-check's limits and two inputs' scope are stated (3.5).

**Revision 10** answers the second round of dual review (the census method and results): the corner-2 claim in 3.5 is stated against the head that carries it, the fixed-scale exception of the search and any-value variants is documented (3.2), C1-b's section count is corrected, and the W1-E records gain the per-row diagnostics w1e §6.2 specifies.

**Revision 9** adds the census report (3.5) and corrects 2.1's account of where native and final scales diverge (3.5, "A correction to 2.1").

**Revision 8** answers the dual review of revision 7 (Claude Opus 5.5 and Codex). No decision changes. The changes:
- under switches (b) and (c), the annular lane's Z is the admitted hull, so its maximum follows the same drive-and-hull rule, and (a) narrows both lanes in the combined runs; a switched-path control checks it (3.2);
- (b) keeps each lane's selected material law (on route L both lanes read the same E and G, so (b) and (c) coincide there) (3.2);
- the final scales are carried on the certified proof, outside the priced work owner (2.2, 0 row 5a-1, 6);
- loaded-member stack use is measured, not inferred from the six nodal witnesses (9);
- the probe gains a manifest guard, a recording-identity check, a null lane override and a committed-bytes comparison; the census reseals the receipt hash of its reader test copy (3.2);
- `lower_hull` is defined over the curve points the search evaluates (1.2, 5); the exhaustion control's fallback hook avoids `[dev-dependencies]` (1.4);
- stale citations and wording are corrected.

**Revision 7** re-checks the design against main as built after O4 and 5a (coordinated-knowledge-work §5). None of the design's decisions is invalidated. The changes:
- section 0 is re-based on `257b44b910`, with two new rows (O4 and 5a) and refreshed citations;
- the soundness statement of 1.2 is corrected: on the tangent bound a driven upper can be lower than today's interval upper, never higher (1.2, 9 item 2);
- the search runs twice per lane, at projection and at the gate; the repair applies at both (1.2, 1.3);
- 1.4 adds the four group-3 trials' pinned case-1 exhaustion, which the repair removes;
- 2.2 states where the final scales live as built and how they reach `selection`;
- the census probe is one opt-in cargo feature, as w1e-sharper-exact-coverage.md §0 amends, and the counterfactual switches are specified at the leaf level (3.2);
- section 8's rebase points and section 6's D3 starting point are re-stated as built.

**Revision 6** re-checks the design against epoch 4 as built (coordinated-knowledge-work §5) and records the owner's corner-1 decision. The changes:
- section 0 records each assumption's as-built basis and verdict. None is invalidated; two gain qualifiers (the 66-row bound and the trigger's exclusions);
- corner 1 is decided (owner, 2026-10-18, "I concur, adopt option 2"), and REQ-003 (e)'s clause is quoted for the implementation PR (sections 10 and 11);
- the proposed stage-2 "Stop" text agrees with that clause (section 10);
- line citations are refreshed, and the reader's scale law is named `numeric_owner` (2.1).

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
  - *Owner decision.* No predicate, limit or tolerance value changes. The owner decided on 2026-10-18 ("I concur, adopt option 2") that this repair stands unchanged and that REQ-003 (e) states its meaning explicitly. The clause is added in the repair's implementation PR (sections 10 and 11).
- **Corner 2: the halfway scale.**
  - *Cause.* The receipt's `cases[].selection.body_scales` are copied from the native candidate's publication (`retained_wire.rs` `selection`, `owner.evidence().body_scales`). Every other scale-dependent field (the classes, the absolute bounds and the product gate itself) uses the scales the product certificate forms from the final published rows (`final_case.rs` `row_scales`/`couple`). The readers recompute that same final-row law at G5b. The two coincide unless a native candidate value and its final published value round to different binary64 numbers. The census (3.5) finds three ways that happens: a support force or moment magnitude formed from already-rounded projected components (the most frequent); a projected value whose readouts' midpoint, or a millimetre row's double rounding, crosses a rounding boundary the native value does not; and an exact binary64 halfway.
  - *Repair.* The receipt states the final scales the product gate used.
  - *Byte preservation.* Every committed successor passes G5b's equality, so its native scales equal the readers' law. The product gate's final scales must also equal that law. A gating check establishes this on every committed successor and corpus base before the change, and the census shows it on every census input where it is observable (2.2, 3.5). Given that check, no committed receipt moves.
- **What does not change:**
  - the G5b check, every gate predicate, the stop rule's value 2^-64·L, the 256-live, depth-64 and 1,024-split limits, R7, DEF-O's text, D1 and the work limits;
  - every reviewed input. The repair moves priced source only, so it still needs its actual-source D3.
- **What does move** (1.4): the committed loaded members on interval-hull sections (their proof work and anything binding it; where their bending lower bound is nonzero, also U, n and the maximum evidence, with every gate verdict and work-limit outcome held by a regression condition, 3.3); the genuine-exhaustion control and its 13 corpus mutations, whose basis this repair removes; and the formerly refused controls, which now publish.
- **What remains after the repair.** A loaded maximum whose certified hull is too wide for the unchanged gate still falls back (REQ-006 (f)). The census attributes that width to the admitted-against-annular section difference, which every stress row inherits (stage 2 §4.4, last corner), to the represented-Z hull alone, or to both (3.5: of 24 failing C1-b maxima, 6 to the hull alone, 6 to the lanes alone, 12 to either or jointly).
- **Merge with O4: no.** The corners share no law or code path with O4's Timoshenko and section-basis extension (section 8). The census runs as soon as epoch 4 merges; the repair is implemented after O4.

## 0. Basis and dependencies

Re-checked against main `257b44b910` (revision 7): epoch 4 (#1461), O4 (#1468, merged at `fa019d9cef`) and the stack-headroom repair (#1476). Revision 6's basis was `084ff961f8`. Paths are relative to `projects/chirality-piping/`; PP is `core/product_physics/src/`, RE `core/reporting/result_export/src/`, the kernel `core/solver/frame_kernel/src/structural/retained/`.

| # | Assumption | As built at `257b44b910` | Verdict; where it matters |
|---|---|---|---|
| E4-1 | The route-L ordinary identity is `preview-physics-2`; `preview-physics-retained-1` inherits it by hash; DEF-O's meaning hash moved in epoch 4. | PP `preview_physics.rs:14`; DEF-O's hash at `retained_wire.rs:34–35`. O4 moved DEF-E's and the physics-retained-1 table's hashes, not DEF-O's. | Holds. This repair moves no reviewed input. |
| E4-2 | U4b's derived rows are `ProductRecipe::NotCovered`; they take no part in body scales, verified counts or the loaded-maximum search. | Kernel `final_case.rs`: the recipe (201), its `row_scales_reduced` arm writes no scale (1411–1415), `check_intervals` skips it (1564); the search is reached only from `CircularMaximum` on a loaded member (909–919). The rows enter `selection.not_covered` (`retained_wire.rs:1286–1295`). | Holds. Corner 2's final scales come from the four scale kinds only. |
| E4-3 | Straight maxima and their reader checks are unchanged; the failed-ordinary manifest is at most 66 rows per member, 61 when a member's ν is out of domain and its five shear rows are omitted. | `loaded_maximum.rs`, `product_certificate.rs`'s represented-Z hull (657–671) and stress_recovery's `elastic_extrema` are unchanged by O4 and 5a. `retained_product.rs:338` `FAILED_MEMBER_ROWS = 66`, `:340` `FAILED_SHEAR_ROWS = 5` (the maintained control expects 61 for an out-of-domain ν, `w1_source_census_tests.rs` about 1300). | Holds, with 66 as the per-member bound. |
| E4-4 | W1's trigger is the ordinary results that are not `checks_passed`, less the structural exclusions. | `lib.rs:3490` `case_triggers`, `:3518` `dn_trigger_excluded`, unchanged by O4 and 5a. | Holds. The census measures the trigger set on the merged producer. |
| E4-5 | The retained corpus carries the four `w1_member_interior_*` records and the member-load bases. | `fixtures/results/retained_precision_cases.json` (65 cases, 668 mutations) carries `w1_member_load_*`, `w1_member_exhaustion_*`, `w1_member_values_failure_*`, `w1_member_multi_*`, `w1_member_combination_*`, `w1_member_route_e_baseline_*` and `w1_member_interior_*`, each with its invocation. | Holds. The census reads them through their invocations. |
| O4-1 | O4 does not change the represented-Z hull, the search or the provenance of `body_scales` (o4-source-extension.md §8, coupling rule 1). | O4 adds reduced sections (`recipe_reduced`, `row_scales_reduced`, `ReducedSectionSet`) and Timoshenko lane entries. A reduced row's Z is its reduced section's own hull (`final_case.rs` 875–889). Route L carries no allowance, so no loaded member reaches a reduced section. `selection` (`retained_wire.rs:1260–1320`) gained `section_terms` beside the unchanged native `body_scales` (1265–1268, 1310). | Holds. The repair applies to the represented lane wherever its Z is an interval, reduced rows included (none on route L). |
| 5a-1 | 5a reorganizes only `freeze_case`'s closure (stack-headroom-repair.md). | `freeze_case_certify` (`retained_product.rs:6623`) calls `certify_final` (6632), which runs `check_intervals` and so the gate's search; `freeze_case_values` (6577) runs `project` and so the projection's search. The call chain below them is unchanged. D3 re-registered at `6b90748dec` (`retained_memory.rs:1192–1239`; retained-pricing-rules §8.10); tightest evaluation merged dense W3 with 2,795,665,581 B headroom. | Holds. The repair's D3 starts from 5a's registered profile (section 6). The driven search's frame growth is on the search's chain. The final scales are carried on `CertifiedProductProof`, not in `ProductCertificateSpent`, whose size is a priced constant (`retained_resource.rs` `PRODUCT_CERTIFICATE_SPENT`) and which sits in frames on 5a's peak chain (2.2). The six stack witnesses never reach the search, so loaded-member stack use is measured separately (section 9). |

The limits this design cites are unchanged: `CASE_LIMIT` = 20e9 and `INVOCATION_LIMIT` = 60e9 (`retained_wire.rs:67–68`), `COMBINATION_CASE_LIMIT` = 20e9 (`retained_product.rs:7063`), and the search's 256-live, depth-64 and 1,024-split limits (`loaded_maximum.rs:5–7`).

## 1. Corner 1: the represented-Z width floor

### 1.1 The cause, from the code

- **The admitted lane's section modulus is a hull.** `product_certificate.rs` (657–671) sets `represented_z = (I_K/c).hull_point(Ẑ)`. I_K is the binary64 second moment, c = D/2, and Ẑ is the binary64 section modulus, each prepared as the correctly rounded value of the exact annulus (`prepare_product_annulus`). DEF-O states the rule: "admitted-K bending Z is hull(actual Z,I_K/c)".
- **Two roundings, one quantity.** Ẑ = RN(I/c) and I_K/c = RN(I)/c are two different roundings of the same exact I/c. Their relative difference is up to about 2^-52, and zero only when I_K/c is itself a binary64 number.
- **The search runs over the hull.** `loaded_maximum::search` (`product_certificate/loaded_maximum.rs`) receives that hull as `z`. Its node uppers use `corners(…, z, Div).hi`, the value at the smaller endpoint. Its lower uses `.lo`, the value at the larger endpoint. Its stop rule, `gap ≤ 2^-64·lower`, therefore asks the search to close a gap that contains (bending part)·(1/z_lo − 1/z_hi), whatever the subdivision depth. When that floor exceeds 2^-64·L the search can only exhaust. It returns `loaded maximum search exhausted`, and the case is unavailable (stage 2 §4.4, "Limits").
- **The annular lane is not affected.** Its z is a 1024-bit enclosure of the one exact I/c, so its width is far below the stop rule's resolution.

So the corner's cause is not the search's tolerance or capacity. A convergence rule is applied to a field whose input carries an irreducible interval. The rule is meant for the search's own approximation error.

### 1.2 The repair

**Monotonicity.** For fixed resultants, the field f(t; Z) = |N(t)|/A + ‖M(t)‖/Z is non-increasing in Z > 0. Over Z ∈ [z_lo, z_hi], the lane's enclosure of max_t f is therefore [max_t f(t; z_hi), max_t f(t; z_lo)].

**The admitted lane, when its represented-Z enclosure is not a point:**
1. **Drive.** The branch-and-bound runs on the point problem Z = z_lo. Every node upper, the `lower` used for the stop rule and for pruning, and the stop rule itself are today's, evaluated with `Enclosure::point(z_lo)`. The search's own convergence, U − L ≤ 2^-64·L, is decided on that point problem with today's tolerance. The point-coefficient control in stage 2 §4.4 shows such a search converges.
2. **Hull-valid lower.** Beside the driving lower, the search keeps `lower_hull`: the maximum, over the curve points the search evaluates for its lower (`point_max`: each node's two end controls and its de Casteljau midpoint, all points of the curve), of the objective evaluated with the whole hull z, whose lower end is the value at z_hi. The interior Bernstein control is not a curve point; its value bounds nothing from below and is never used for `lower_hull`.
   - It is a lower bound of max_t f(t; Z) for every Z in the hull, since any point's value at z_hi is at most the maximum at z_hi, and that is at most the maximum at any smaller Z.
   - It costs one more objective evaluation per evaluated point, which can share the axial and bending terms.
3. **Result.** On convergence the lane returns [lower_hull, U]. U is the driving search's maximum live upper, which bounds max_t f(t; z_lo) and so the maximum over the whole hull. The enclosure is sound, and its width is the search's convergence width plus the input hull's own effect.
4. **Exhaustion** under the unchanged limits returns the named kernel failure, as today. The diagnostic `{row, member, lower, upper}` reports [lower_hull, the maximum upper over live nodes and the unexpanded parent], an enclosing interval of the lane's maximum over the hull, as stage 2 requires. Its encoding and G5 checks are unchanged.

**When the represented-Z enclosure is a point** (z_lo = z_hi, for example D/2 a power of two): exactly today's code path, with the same operations, work and bytes. The four committed `w1_member_interior_*` records use such sections and keep their bytes.

**Where the search runs.** A loaded member's maximum is searched twice per lane, with the same inputs: at projection (`ProductProofDraft::project`, which publishes n) and again at the gate (`check_intervals` through `recipe_reduced`). The repair applies at both, so the gate judges the n that the projection formed from the same enclosure.

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
- **Soundness.** Every driven node upper is a valid upper bound of max_t f(t; z_lo), and so of the maximum over the whole hull. It is never larger than today's upper over the interval:
  - the paired upper and ρ divide the bending upper by z_lo, in today's code and at the point alike;
  - the tangent bound's Q/Z² term divides each Bernstein coefficient of Q by the Z² enclosure. Today a wholly negative coefficient's upper is divided by z_hi², at the point by z_lo², so a driven upper can be lower than today's, node for node on the same controls (revision 7). A coefficient is wholly negative only when the weighted sum of the M_y and M_z control products is; a sign change in a moment component is necessary for that, not sufficient.
  
  Pruning on the z_lo lower is sound for U, because it keeps every node whose upper exceeds a value attained at z_lo. The hull-valid lower is a valid lower bound over the whole hull. No published guarantee changes.
- **The owner's decision.** This is a change in what a fixed search tolerance bounds. REQ-003 (e), an owner-approved commitment, says "No search, verification, tolerance or domain limit is relaxed to obtain a bound". The owner decided that this repair stands as designed and that REQ-003 (e) gains a clause stating it (section 11). The matching edit to stage 2 §4.4's "Stop" text is in section 10.

### 1.3 Work and memory

- **Work.** The extra hull-valid evaluations are counted at actual cost in `NumericWork`, at projection and at the gate, before completion is sealed, with nothing counted as zero (stage 2 §5 and R-1). A point-hull search does no extra work.
- **Memory.** No search node grows. `lower_hull` is one more `Endpoint` in the search frame. The live-node backing, L4's 1 MiB cap and the lanes' sequential execution are unchanged. The actual-source D3 prices the frame growth and the TEXT positively (section 6).

### 1.4 Committed bytes that move, and the exhaustion control

- **Loaded members on interval-hull sections that converge today.** `w1_member_load_product_consuming_successor` uses the D = 0.2 m, t = 0.01 m pipe with an axial global-x load. Its bending share is zero, so the hull adds no width and the search converges today. Under the repair its admitted lane takes the driven path, and the extra hull-valid evaluations change its metered proof work.
  - Its receipts' work counters, and the hashes binding them, move: the `w1_member_load_*` corpus entries, and any multi-member or values-failure entry built on the same pipe.
  - The `w1_member_multi_*` and `w1_member_combination_*` entries carry axial loads on the same D = 0.2 m section; they converge today, and their work moves too.
  - When the bending lower bound is zero at every evaluated point (an axial load) and no tangent coefficient is wholly negative, the driving and hull lowers and the uppers coincide with today's and the search follows today's path. Then every number, class, selection and maximum value stays the same, and a check over the regenerated entries enforces it (`w1_hull_repair_moves_only_work_on_converging_members`).
  - A converging member whose bending lower bound is nonzero at some evaluated point may stop at another point, so U, n and the maximum evidence can change. The census classifies each committed member (C1-a), and any such move is listed with its reason.
  - The census lists every such committed member (section 3.2, input C1-a).
- **The genuine-exhaustion control.**
  - `w1_domain_stage1_tests.rs` `w1_member_load_exhaustion_retains_typed_interval_work` produces `w1_member_exhaustion_{sparse_interactive,dense_scrutiny}`. The corpus describes them as "ACTUAL MAINTAINED PRODUCER OUTPUT … whose span-maximum search exhausts".
  - 13 `member_exhaustion_diagnostic_*` mutations in `fixtures/results/retained_precision_cases.json` are built on them, and so are `declared_member_load_workspace.rs` and the genuine-exhaustion test in `tests/test_declared_member_load_reader.py`.
  - Its model (`fixtures/product_preview/numerical_sensitive_torsion_model.json`) is the D = 0.2 m section with a global-y UDL, exactly the corner this repair removes, so it is expected to stop exhausting.
  - The replacement, in order of preference:
    1. **A genuine input.** One that still exhausts under the unchanged limits on a point-hull section: a near-degenerate rotating moment with varying axial force, the theoretical case stage 2 §4.4 names. The census's C1 sweep includes candidates (section 3.2).
    2. **A hook-driven input.** If no genuine input is found, the existing test capacity hook (`loaded_maximum::hooks::with`) drives a real member to exhaustion at reduced limits. It is exposed through an opt-in feature that no `[dev-dependencies]` entry enables (not the route RF's operation counter takes, which compiles into every test build, the stack-witness build included; w1e-sharper-exact-coverage.md §0), and the control's maintained command passes it explicitly. The corpus entry then says so, and does not claim production limits. The feature stays out of the registered compiled identity, as `mutation-controls` does. As built (#1484) it is `loaded-maximum-hooks`, and its maintained command, run by hand like `mutation-controls`' seeded faults, is `cargo test --offline --manifest-path core/product_physics/Cargo.toml --lib --features loaded-maximum-hooks -- w1_member_load_exhaustion_retains_typed_interval_work w1_final_scales_equal_reader_scales`. The release sweep and hosted CI pass no cargo features (`validation/benchmarks/numerical_robustness/tests/feature_guard.rs` `ci_passes_no_cargo_features`), so neither runs it; the ordinary build skips the two exhaustion corpus bases in the gating check.
  - Either way the 13 mutations are rebased on the new base by the corpus writer, and the readers' expected outcomes are unchanged in kind.
- **The two corner controls** in VER-003 change from refusals to publications (section 5). Their new successors join the corpus.
- **The group-3 trials' pinned exhaustion** (revision 7). `w1_source_census_tests.rs` pins case 1 of F6/partial, T18, T19/pre-K-D5 and T19, in both modes, as unavailable with "loaded maximum search exhausted" (`w1_partial_case_cause`, through `b4_w1_covers_every_formerly_selected_source_block_case` and `w1_partial_other_case_statuses_and_causes_are_pinned`), and stage 2 §4.6 records the same. These members use ordinary sections with a global-y UDL, so the repair removes that cause. Their new expectation is the outcome the unchanged gate gives, taken from the census (C1-a) and asserted case by case; no committed successor binds them. The B4 table (`FORMERLY_SELECTED`, about 519–532) records `["case"]` and `needs_recompute` for these trials; it is updated to the new outcome, although its permissive check would not fail.
- **Nothing else moves for corner 1.** Point-hull sections, which include the four `w1_member_interior_*` records, the annular lane, route E, the source route and every nodal-only successor keep their bytes.

## 2. Corner 2: the halfway scale

### 2.1 The cause, from the code

- **The product certificate's gate uses final-row scales.** `final_case.rs` `row_scales` forms each body's translation, rotation, force and moment scales from the final published rows (`spent.normalize(unit, value)`). `check_intervals` couples them with the body extent (`couple`, binary64 `max` with `extent·ro`, `tr/extent`, `mo/extent`, `extent·fo`) and applies the p = 512 floor. It then gates, classifies and bounds every row with those scales.
- **The readers recompute that law.** The Rust reader (`retained_precision.rs` `numeric_owner`, 3845–3911, and `g5b`, 4424–4450) and the verifier recompute exactly that law from the published rows. G5b then requires `selection.body_scales` to equal it bit for bit.
- **The receipt states native scales instead.** `retained_wire.rs` `selection` (1260–1320) writes `body_scales` from `owner.evidence().body_scales` (1265–1268). These are the native candidate's publication scales (`adaptive.rs` `publication_with`, from the native candidate's binary64 values), not the final rows'.
- **Where they diverge.** (Corrected by the census, revisions 9 and 11; 3.5, "A correction to 2.1". A support magnitude is the norm of the already-rounded projected components, so it can round apart from the native magnitude with no readout gap; a projected row's final value is the projection of the two readouts' hull midpoint, so a readout gap or a millimetre row's double rounding also separates them. The halfway is one cause among three.) In the halfway case: a native candidate value is accurate to about 2^-p relative, and the final value is RN64 of the product's projection, so for a projected row the two can round apart when the exact value lies within the native error of a binary64 rounding boundary. For an exact halfway they round apart about half the time, so a body maximum that is an exact halfway gives receipt scales one ulp away from the reader's.
- **Observed instances:**
  - the 73/24 Pa asymmetric control, whose force maximum (7/2)Z and moment 7Z are halfways (`w1_loaded_scale_halfway_keeps_the_existing_g5b_refusal`);
  - the three-case cap witness `witness_w1_member_load_three_case_three_basis_128`, which "truthfully retains its exact G5b scale refusal" (stage 2 §13). Its divergence is on body 0's translation and rotation scales, not a halfway (3.5).
- **Why it withdraws everything.** The G5b refusal is a precommit fault, so it withdraws the whole successor (REQ-006 (b)).

The corner is a provenance inconsistency in one receipt field. The gate, the classes and the bounds were already judged on the final scales.

### 2.2 The repair

- The product certificate keeps the final per-body scales it computed in `check_intervals`, coupled and floored, as part of its proof, and exposes them with the verdicts. As built, those scales are the `scales` vector that `row_scales_reduced` reserves (its capacity is `capacities[6]`) and `check_intervals` couples and floors in place (`final_case.rs` 1536–1557). The repair returns that vector from `check_intervals` to `certify_final`, which moves it into the `CertifiedProductProof` it builds, instead of dropping it. So no allocation is added and no capacity moves. The vector is not stored in the work owner `ProductCertificateSpent`: that type's size is a priced constant (`retained_resource.rs` `PRODUCT_CERTIFICATE_SPENT`) embedded in every draft, failure and proof, some of them in frames on 5a's peak chain (revision 8). What changes is one `Vec` field on the certified proof and the heap vector's lifetime, from the end of `check_intervals` to the proof's drop after serialization; D3 prices both (section 6). Following O4's layout discipline (o4-source-extension.md §4.2), every serialized proof structure and capacity is unchanged, including `capacities[6]`. A pin test asserts it (`w1_final_scales_reuse_the_reserved_capacity`).
- `selection` writes `body_scales` from those final scales, for selected cases (`retained_wire.rs` about 2240) and for combinations (about 2146) alike. Both call `selection` with the certified product proof in hand.
- **Same bodies.** The native publication lists every body 0..nb, four kinds each (`adaptive.rs` `publication_with`, 3136–3144), and the final scales have one entry per body of the same source (`row_scales_reduced` sizes them by `body_count`). The receipt's body list does not change shape.
- The native evidence's scales stay native evidence. They still feed the native publication certificate unchanged, and no other receipt field changes source.

**Consequences:**
- The precommit G5b body-scale equality now holds by construction, wherever the product's final law and the readers' law agree. They are the same binary64 law over the same published rows. Any disagreement is still refused by the unchanged precommit G5b and falls back, so it fails closed.
- A former halfway withdrawal publishes when every gate passes. If a row's gate fails under the final scales, that case alone is unavailable with its named cause, as for any gate failure. The halfway no longer withdraws every case.
- **No committed byte moves, given one gating check.** G5b's acceptance of every committed successor shows that its native scales equal the readers' recomputed law. The repair writes the product gate's final scales instead. So byte preservation also needs the final scales to equal the readers' law on every body and kind, including kinds no row's class depends on. The two computations differ in code:
  - The product's `row_scales_reduced` (`final_case.rs` 1313–1532, coupled and floored in `check_intervals` 1544–1557) takes native rows whose native class is not input-derived, plus support components into the force and moment slots. It normalizes the final values, couples with `couple`, and applies the floor from native evidence.
  - The readers' `numeric_owner` (`retained_precision.rs` 3845–3911) and `g5b` (4424–4450) take the published rows of each scale kind, less prescribed components. They couple in the same binary64 form and apply φ recomputed from `resolution_scale`, after checking the receipt's `floor` against it.

  These must be shown to select the same row sets and the same floor. The implementation argues that row-set mapping kind by kind. A gating test (`w1_final_scales_equal_reader_scales`) asserts final scales = readers' scales on every committed successor fixture and every producer-solved or hook-produced corpus base, with that base's own hook, before `selection` is switched. The census establishes the same equality on every census input where a candidate passes G5b; where none does, the case published nothing, so no receipt carries the scales (3.5). If any body or kind differs, the repair stops for diagnosis. Such a difference would be a second inconsistency between gate and reader, and it would need its own design; it would not justify writing a scale the reader rejects.
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

**Order.** The census measures what the repair would do before the repair is committed. So its first commit, on the repair's branch, adds the two repairs behind test-only switches, plus the probe. Production paths are unchanged until the census is reported.

**The probe and switches** (revision 7, adopting w1e-sharper-exact-coverage.md §0's amendment; one module serves both censuses):
- **One opt-in cargo feature,** `census-probe`, declared by the frame kernel and forwarded by `product_physics` (`census-probe = ["open_pipe_stress_frame_kernel/census-probe"]`). No `[dev-dependencies]` entry enables it, so it is off in every ordinary test build of either crate, the stack-witness build included. Only the census commands pass `--features census-probe`.
- **A guard test** in the ordinary `product_physics` test build asserts that the kernel was compiled without it (the kernel's `retained_api::CENSUS_PROBE` constant is `false`). The stack witnesses run without the feature, in their ordinary commands.
- **Off means absent.** Every switch, record and counterfactual is compiled only under the feature. Without it, each switch reads as off and the production path's operations, work and bytes are today's.
- **The switches** are thread-local, so a census thread sets them around one invocation's retained work and leaves concurrent tests untouched:
  - *corner 1*: the driven search of 1.2 on an interval represented Z;
  - *corner 2*: `selection` writes the product proof's final scales (2.2);
  - *(b) the same section*: the annular lane, in its solve and in its stress recipe, takes the admitted lane's section leaves Â, Î, Ĵ, the represented Z and, on a Timoshenko member, the shear-area leaf Â_s, each paired with the annular lane's own selected material law (revision 8). On route L both lanes read the same independent E and G (`ProductMaterial` Base or Point maps to `Ordinary`), so (b) and (c) coincide there; on route E the annular lane keeps its exact E/ν modulus;
  - *(c) the same readout*: (b) with the admitted lane's moduli as well (the binary64 E and Ĝ), so the annular lane solves the admitted readout's problem (its coefficients are the admitted products and its shear rigidity Ĝ·Â_s) and the lanes differ only by their own arithmetic.
  - *The annular lane's Z under (b) and (c)* is the admitted lane's represented hull, so its loaded maximum follows the same drive-and-hull rule as the admitted lane (1.2): the search is driven when the lane's Z is the represented hull (the represented lane, or the annular lane under (b) or (c)) and that hull is not a point. Without that, (b) and (c) would recreate corner 1 in the annular lane and exhaust before their gate (both reviewers). The selection key is that flag and a non-point hull, never a non-point Z alone: the annular lane's own Z is a non-point 1024-bit enclosure (`product_certificate.rs` 638). In the combined runs (a)+(b) and (a)+(c), (a) sets Z to Ẑ in both lanes. A switched-path control checks this: under (b), the stage-2 D = 0.2 m control converges.

  (b) and (c) change the lanes' problem, so a census re-runs the whole invocation with the switch on: the lanes are certified afresh, and the projection, scales, classes and every predicate follow from them. W1-E's census uses the same mechanism (w1e-sharper-exact-coverage.md §6.2). The switches' coefficient arithmetic runs in a scratch work owner, so the counted work of a re-run is the lane solve's own.
- **Counterfactuals without a re-solve** are judged at the gate, after the actual verdicts, in a separate work owner that no receipt reads. Switch (a), the admitted Z set to the single Ẑ, re-forms the represented lane's bending-stress and maximum enclosures (the maximum by a search with Z = Ẑ). Each counterfactual recomputes every projected value from its own hull, the support magnitudes from their components, the coupled and floored scales from the counterfactual published rows, then each row's class and all four predicates, with the gate's own arithmetic. Two variants are exceptions (revision 10): the ±4-step unit search (U1 (i)) and the any-value columns judge each row's candidates at the scales of their fixed-projection counterpart, one row at a time, without re-forming the scales, magnitudes and classes from a jointly chosen statement. Their cells are therefore fixed-scale feasibility diagnostics, not counts of a publishable statement. The probe checks itself on every case: its unaltered counterfactual must reproduce the published values, the final scales and every actual verdict bit for bit.
- **Identity checks** (revision 8):
  - a *manifest guard*: `census-probe` appears only as the kernel's `[features]` key and the `product_physics` forward, in no `[dependencies]` or `[dev-dependencies]` entry (the `mutation-controls` precedent, `numerical_robustness/tests/feature_guard.rs`), because the registered build identity names no features and `Cargo.lock` records none;
  - *recording moves nothing*: on every C1-a and C2-a input, today's run with the probe recording and the same run without it give the same candidate, value for value;
  - *the committed bytes*: on every committed input (corpus bases, successor fixtures), today's candidate in the census build equals the committed document, which a probe-off build produced;
  - *a null lane override*: the (b) code path run with the annular lane's own leaves reproduces today's candidate, counted work included.
- **The reader test copy.** Replacing `selection.body_scales` changes the receipt body, which G1's receipt hash binds. The census reseals the test copy's `receipt_sha256` with the product's own domain hash, and counts a scale disagreement only when the reader reaches G5b and refuses there; a refusal before G5b is reported as such.
- **Census targets** are `#[ignore]` tests in `core/product_physics`, compiled only under the feature and re-runnable on demand.

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
  - the native, final and readers' scales per body and kind, and their equalities. The readers' law is read without a reader change: the unchanged Rust reader validates the candidate as produced (native scales) and again with `selection.body_scales` replaced by the final scales (resealed), and G5b's body-scale equality decides each. Per-body readers' values are observable only where one of the two passes G5b; where both fail, the census reports the case as such;
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
- **C2-b: a generated sweep.** 64 route-L invocations per mode, each with an independent soft-spring member that makes the ordinary result Sensitive, so that W1 is reached (an ordinary `checks_passed` case is `NotRequired`; PP `lib.rs` `case_triggers`, 3490).
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
- **Regression condition for already-converging members.** For every C1-a member that converges today, the census reports before and after: the gate verdict, class (relative or absolute) and published absolute bound of each certified row; the case's selection and status; the counted work against W1's case and invocation limits (`CASE_LIMIT` = 20e9 and `INVOCATION_LIMIT` = 60e9, `retained_wire.rs` 67–68; `COMBINATION_CASE_LIMIT` = 20e9 for any member in a combination successor, `retained_product.rs` 7063); and the 4 MiB stack witnesses' outcomes. A row that passes today and fails after the repair, a case that changes from publishing to withdrawn, or a work-limit refusal that does not occur today stops the repair for diagnosis. It does not proceed under a changed gate. Numerical moves that leave every verdict and refusal unchanged, including any change of row class or absolute bound, are listed in the implementation PR with their reason (1.4).
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

- **Timing.** After 5a merges, on the repair's branch from main `257b44b910` (revision 7; w1e-sharper-exact-coverage.md §0).
- **Build and host.** It builds `frame_kernel` and `product_physics` tests in the debug profile with `--features census-probe`, in its own target directory. It uses the qualification host's heavy-run slot and memory rules, within the 64 GiB test ceiling. The census uses private drivers, so it needs no registered build identity. The cap witnesses run as separate commands with their 4 MiB stacks.
- **Pilot.** A pilot of ten generated invocations per sweep measures time and memory before the full run. No duration is assumed here.
- **What it writes.** Nothing under `validation/references` or the committed fixtures. The probe and switches land on the repair's own branch as test-only code and stay as `#[ignore]` targets that can be re-run.

### 3.5 Census report (revision 9)

**What ran.** The census targets of section 3.2 at `15b8651ff4` (the census commit on the repair's branch, main `257b44b910` plus the probe), in one `--features census-probe` debug build: 28 targets, 16 threads, 4,696 s, peak resident 4.19 GB (job pid 45973, slot 2, `2026-10-20T19:59:22Z`–`21:17:38Z`, rc 0). The pilot ran first: ten invocations per sweep, 405–463 s, peak 4.76 GB. Production paths were unchanged; the guard and manifest tests pass in the ordinary build. Each input ran today, with corner 1, with corner 2 and with both; inputs with a failing verdict under corner 1 also ran (b) and (c).

**The probe's own checks held everywhere.**
- The unaltered counterfactual reproduced the published values, the final scales and every actual verdict on all 4,122 gate-case records: the 3,338 of the corner runs (C1-a 188, C1-b 1,428, C2-a 680, C2-b 532, W1-E 510) and W1-E's 784 (b)/(c) re-run records.
- **The self-check's limits.** It is a check of the probe's recomputation, so it is trivially satisfied on rows the probe copies rather than recomputes (unloaded maxima, records and not-covered rows). Recording and the null override are trivially satisfied on the 19 of 173 C1-a and C2-a invocations that produce no candidate. No gate case ran at p = 512, so the floor branch of the final scales is not exercised by the census; it rests on section 5's `w1_receipt_scales_keep_the_p512_floor`, whose case has an active floor. An independent re-tabulation also checked invariants over all 801,168 row records with no violation: on non-bending rows (a) equals the base and (a)+X equals X; a base pass implies a pass under any value, the unit search and R1; on SI rows (d) and (d'') equal the base; and on w = 0 loaded members (a) equals the base.
- On all 173 C1-a and C2-a invocations, recording moved nothing and the null lane override reproduced today's candidate.
- Today's candidate equalled the committed document on 73 committed inputs. The 7 that differ are corpus bases written with a hook or derived statement, run here without it (`d38_beside_selected`, four `b2_*` bases, `o4_w1e_unavailable` as corpus base and fixture). The maintained gating check applies their hooks (below).
- The switched-path control held: under (b) and (c) both lanes are driven and converge on the D = 0.2 m control.

**C1 (corner 1).** Denominator: loaded members reaching the search, each mode counted.

| Set | Members | Exhausted today | After: gate passes | After: gate fails | After: exhausted | Not reaching the gate |
|---|---:|---:|---:|---:|---:|---:|
| C1-a | 48 | 10 | 46 | 0 | 0 | 2 (the values-failure hook entries) |
| C1-b | 490 | 460 | 466 | 24 | 0 | 0 |

- **C1-a.** The ten exhausting today are case 1 of F6/partial, T18, T19/pre-K-D5 and T19, and the genuine-exhaustion control `w1_member_exhaustion_*`, each in both modes; all are on the D = 0.2 m section (w = 98.4 × 2^-64). The 38 converging today are axial loads on that section, the four point-hull interior records and the cap inputs.
- **C1-b.** Its 49 section entries name 48 distinct sections: `grid/20x2` and the committed 0.02 × 0.002 m section coincide, with identical results, and both keep their sampling weight. Section 3.2 pre-registered (40 + 3 + 7) × 5 × 2 = 500 invocations; 490 ran, because 3.1's last committed row is the D/2-power-of-two family already among the three added sections. The 30 converging today are the three D/2-power-of-two sections (w = 0). Every member with w > 0 exhausts today and converges after the repair, in at most a few splits.
- **By w and b** (C1-b, after the repair):

  | w (× 2^-64) | Pass | Fail |
  |---|---:|---:|
  | 0 | 30 | 0 |
  | (0, 160) | 60 | 0 |
  | [160, 871) | 202 | 8 |
  | [871, 2,037) | 146 | 14 |
  | ≥ 2,037 | 28 | 2 |

  | b at the best point | Pass | Fail |
  |---|---:|---:|
  | [0.25, 0.75) | 164 | 8 |
  | [0.75, 1] | 302 | 16 |

- **Attribution of the 24 failures** (12 members × 2 modes; every failing predicate is SharperExact): *Z-hull only* 6, *either* 6, *lane only* 6, *joint* 6, *other* 0. The failing members are S1 and S2 on 769 × 28 mm (*lane only*) and 881 × 22 mm (*joint*); S1 and S2 on the committed 0.12 × 0.01 m section (*Z-hull only*); S3 on 111 × 4 mm (*joint*) and 881 × 22 mm (*Z-hull only*); S4 on 1,287 × 20 mm (*either*); S5 on 958 × 8 mm and the committed 0.114 × 0.006 m section (*either*) and on 1,287 × 20 mm (*lane only*). Hull widths relative to |n| are 532 to 2,316 × 2^-64.
- **Escalation (pre-registered, 3.3).** *Z-hull only* + *either* + *joint* = 18 of 460 converged C1-b members with w > 0 = **3.9 %**, below 5 %. It does not fire. Counting the coincident section once, it is 18 of 450 = 4.0 %, also below 5 %.
- **Stop conditions (3.3).** None holds at census level: no post-repair exhaustion on an ordinary section, no *other* failure, and the regression condition's census part holds (next). Its stack-witness part is measured on the production heads (below).
- **Regression condition, already-converging members** (C1-a, 30 cases, 3,240 rows): no row changes verdict, class, value, scale or absolute bound; no case changes status; the charged case and invocation work is unchanged and far below 20e9 and 60e9. Only the proof's metered work moves, by +2.9 % to +5.1 % (`wide_lme`, for example 52,441,312 to 54,373,520 on `w1_member_load_*`), from the driven path's hull-valid evaluations (1.4). An independent re-tabulation over all 4,568 gate rows of every case reaching the gate today also found every verdict, class, failing predicate, value, normalized value and scale unchanged.
- **Regression condition, the stack-witness part** (3.3 names "the 4 MiB stack witnesses' outcomes"). The census ran each invocation on a 512 MiB thread, so it does not measure them. They are run in the ordinary build, without the census feature, on each production head (section 9): the corner-2 head's results are in the implementation PR; corner 1's follow its production switch.
- **The cap witnesses' inputs** (census outcomes, not stack runs): `..._128` withdraws today at G5b (corner 2, below) and publishes with corner 2; `..._128_dyadic` publishes. The cap-law input, as run, is the law tests' nodal cap-maximal ring (`cap_maximal_cases(3)`), not its solvable variant: its ordinary solve does not publish, so W1 stops at preparation in both modes and it contributes no loaded member to C1-a. The loaded-member cap shape is the two 128-term witnesses'.
- **Baseline.** The converged members' 20 element-local stress rows pass the unchanged gate at 9,528 of 9,800 (97.2 %) in C1-b and 920 of 920 in C1-a.
- **Case level.** With both repairs, 296 of 490 C1-b invocations publish; 194 fall back on a gate failure, 170 of them on rows other than the loaded maximum (the inherited two-readout corner; most often a support or end force, an S3 or S5 bending stress, or a displacement).
- **No genuine exhaustion input was found.** No census input exhausts after the repair, so section 1.4's replacement for the genuine-exhaustion control is the hook-driven input (its option 2).

**C2 (corner 2).**

| Set (run) | Invocations at the gate | Native ≠ final | Final ≠ readers | G5b withdrawals today | Publish with corner 2 |
|---|---:|---:|---:|---:|---:|
| C2-a committed (today) | 126 (165 cases) | 0 | 0 | 0 | 126 |
| C1-a (today; with corner 1) | 28 | 2 (the `_128` cap witness, body 0 translation and rotation, both modes) | 0 | 2 | 28 |
| C2-b halfways (today) | 64 (32 per mode) | 36 | 0 | 32 | 58 |
| C2-b decimal (with corner 1) | 64 | 0 | 0 | 0 | 32 |
| C2-b controls, 73/24 Pa (today) | 2 | 2 | 0 | 2 | 2 |
| C1-b (with corner 1) | 490 | 152 | 0 of 296 observable | 30 | 296 |

- **Coverage (pre-registered).** 32 confirmed halfways per mode reach the native-versus-final stage, at least the 24 required.
- **Final = readers** on every case where the readers' scales are observable (one candidate passes G5b). Where no candidate exists the case failed the gate first, and the comparison is not observable.
- **Not reached.** C2-a: 15 `NoTriggeredCase` (the synthetic statements) and 2 preparation. RV120 is not a separate C2-a input: its shapes are reader-side mutations of corpus bases, which C2-a runs. C2-b decimal: 32 reach the gate only with corner 1 (the simply supported shape exhausts today).
- **A correction to 2.1** (revisions 9 and 11). Native and final scales round apart in three ways, not only at exact halfways:
  - **Support magnitudes.** A support force or moment magnitude is the norm of the already-rounded projected components (`project`, `support_hypot`), so it can round apart from the native magnitude with no readout gap at all. This is the S4 (skew member) cause, which carries 98 of the 152 C1-b invocations that differ, on force; the moment differs through the coupling. For example, on `c1b/S4/grid/20x2` the root support's force magnitude publishes 4.949747468305834 while all four lane bounds round to the native 4.949747468305833.
  - **Projected rows.** A projected row's final value is the projection of the midpoint of the two readouts' hull, and the native value is the admitted readout's own rounding. A readout gap, or a millimetre row's double rounding, can move the midpoint across a rounding boundary. This is the S1, S2 and S5 cause (18 invocations each, on translation and rotation; two S5 millimetre rows are double rounding).
  - **Exact halfways,** as 2.1 described (C2-b, the 73/24 Pa control).

  With corner 1 on, 152 of 490 C1-b invocations differ, and 30 would withdraw at G5b without corner 2. No committed successor differs, so 3.3's stop rule ("any native ≠ final instance on a committed successor") does not hold. The repair writes the scales the gate used, and the readers recompute from the same published rows, so it covers every such case whatever the cause.
- **The gating check.** The census establishes final = readers on every census input where it is observable; where it is not, the case published nothing, so no receipt carries the scales and the rule's purpose is met. The maintained gating check, `w1_final_scales_equal_reader_scales`, runs on the production change itself (the implementation commits that follow this report, not the census commit) and covers committed documents, not census inputs: over every committed successor fixture and every producer-solved or hook-produced corpus base, with each base's own hook applied, the receipt's scales equal the committed ones and the reader's G5b holds. Corner 2 is implemented on that basis (3.3).

**Under Z1 (switch (a), the admitted Z set to Ẑ), route L.** In the corner-1 run, Z1's world is the (a) counterfactual: the represented lane is a point and is searched undriven.

| C1-b (490) | Corner 1 as designed | Z1 |
|---|---:|---:|
| Loaded maximum passes the gate | 466 | 478 |
| Case publishes (every row passes) | 296 | 320 |
| … with any SI value (`any_both`) | 388 | 428 |
| … with SI publication (U1 (iii)) | 326 | 350 |

C1-a is unchanged under Z1: 46 of 46 maxima and 52 of 52 cases pass either way. Route E's figures are in w1e-sharper-exact-coverage.md §6.3.

**Decision rules applied (3.3).**
- *Corner 1:* the regression condition holds, no stop condition holds and the escalation does not fire. Its production switch waited for the owner's Z1 ruling; Z1 is not adopted (owner, 2026-10-20, "I approve 1A, 2A, 3A."; section 11), so corner 1 lands as designed.
- *Corner 2:* final = readers on every census input where observable; implemented once the maintained gating check holds on the production change (committed fixtures and corpus bases).

## 4. What stays unchanged

- the loaded-maximum stop rule's value (2^-64·L), its 256-live, depth-64 and 1,024-split limits and its failure propagation. What the rule bounds on an interval hull changes, as section 1.2 states and REQ-003 (e)'s decided clause states (section 11);
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
| `lower_hull` taken over all three Bernstein controls, the interior one included (not a curve point) | `w1_hull_search_lower_encloses_the_far_endpoint` (a member whose interior control's value at z_hi exceeds the true maximum there) |
| Under (b) or (c), the annular lane searched undriven on the admitted hull | the census's switched-path control (3.2): under (b) the D = 0.2 m control converges |
| The extra evaluations unmetered | `w1_hull_search_work_is_counted` |
| Receipt scales still native | `w1_loaded_scale_halfway_publishes` (the 73/24 Pa control in both modes: precommit G5b passes, and every row's gate verdict is as the final scales give); replaces `w1_loaded_scale_halfway_keeps_the_existing_g5b_refusal` (existing) |
| Receipt scales final but not coupled or floored as the gate used them | `w1_receipt_scales_are_the_gate_scales` (active coupling at the selected precision) and `w1_receipt_scales_keep_the_p512_floor` (as built: a b3k cantilever whose prescribed 1 m root translation selects p = 512, with the floor binding body 0's force and moment scales): every verdict's scale is the proof's final scale, derived from the rows, extent and floor. A floor-only mutant is killed by the second, which the first does not see. `w1_final_scales_reuse_the_reserved_capacity` is the pin test of 2.2 |
| Native evidence scales replaced, or used for anything new | `w1_native_scales_stay_native_evidence` (the native publication certificate's inputs unchanged) |
| A committed byte moves beyond section 1.4's list | the existing receipt-byte tests over the corpus, successor fixtures, R1 records and route-E successors; `w1_committed_publications_preserve_all_but_authorized_definition_hashes` (existing), whose exceptions gain exactly section 1.4's entries; `w1_hull_repair_moves_only_work_on_converging_members` |
| Final scales written where they differ from the readers' law | `w1_final_scales_equal_reader_scales` (the gating check of 2.2, over every committed successor fixture and corpus base, hooks applied) |
| The genuine-exhaustion control left on a base that no longer exhausts | `w1_member_load_exhaustion_retains_typed_interval_work` (existing), rebuilt on section 1.4's replacement; the 13 `member_exhaustion_diagnostic_*` mutations rebased |
| The cap witness still withdrawn | `witness_w1_member_load_three_case_three_basis_128` (existing, `#[ignore]`): its expectation is derived from the gate verdicts under the final scales, row by row, not copied from a census observation. The reason for the change is the repaired provenance |

The two corner tests named in VER-003 change from asserting the refusal to asserting the repaired outcome. That edits a test expectation, so the reason is stated: the cause each refusal recorded is repaired, and the gates are unchanged. AC-007 (b) asks for exactly this ("the decline tests they retire are replaced by admission and refusal tests").

## 6. D3

- No reviewed input moves, and the compiled identity and reader layouts are unchanged.
- **Priced source moves:**
  - the kernel's `loaded_maximum.rs`: its frame and TEXT;
  - `final_case.rs`: the retained final scales, one `[f64; 4]` per body, at most 32 bodies per case, now live from `check_intervals` until the certified proof drops, plus one `Vec` field on `CertifiedProductProof` (not on the priced work owner; 2.2), times the existing multiplicities;
  - `retained_wire.rs` `selection`: TEXT.
- The census probe and switches are compiled only under `census-probe`, which no product or ordinary test build enables; their source is still in the priced files, so the actual-source D3 audits it with the repair.
- The repair therefore re-registers on its actual source (D3; REQ-004), starting from 5a's registered profile (`6b90748dec`, retained-pricing-rules §8.10).
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
- **Rebase (as built, revision 7).** O4 and 5a are on main, so the repair is implemented on main `257b44b910`. Its shared files are `retained_wire.rs` `selection` (`body_scales`, 1265–1268 and 1310, beside O4's `section_terms`, 1270–1284) and the registered profile, which 5a last registered. It re-registers on its own head.

## 9. Verification and independent review

**Checks on the repair's head:**
- the mutants of section 5;
- the census report, re-run on the head for the C1 and C2 inputs that changed outcome;
- the suites `select_affected_checks.py` selects: frame kernel (`retained_k4`), product physics (the W1 census, facade and stage-1/stage-2 tests), result_export and the desktop wasm equivalence;
- the corpus in native Rust, wasm and Python;
- the actual-source D3 (`generate --check`);
- the six stack witnesses at R/64 and R/16, in the registered build and without the census feature (stack-headroom-repair.md, "Maintained check"), with the guard test showing the feature off. Those six run nodal inputs and never reach the loaded-maximum search (revision 8), so also:
  - the two `#[ignore]` 128-term member-cap witnesses at R/16 (`witness_w1_member_load_three_case_three_basis_128` and `..._dyadic`), which do;
  - a painted-stack peak of a loaded member on an interval-hull section (the D = 0.2 m control), before and after the repair, in the registered build, with the measurement probe appended for measurement only, as 5a measured;
- the hosted `harness` on the exact head.

**Independent numerical review checklist:**
1. The monotonicity argument and the hull-valid lower: sound for every Z in the hull, under directed rounding.
2. That the driven search's limits equal today's, that its uppers are valid and never above today's (equal except where the tangent bound divides a negative coefficient), that pruning is sound, that the stated change in what the stop rule bounds is accurate (1.2), and that exhaustion still fails closed.
3. That a point hull and the annular lane are byte-identical to today.
4. That DEF-O's text describes the repaired enclosure unchanged.
5. That the final scales are exactly the scales the gate used, coupled and floored, and the kind-by-kind row-set mapping to the readers' law (2.2).
6. That only section 1.4's committed bytes move, each for its stated reason.
7. The census's counterfactual attribution and its pre-registered rules (3.3).
8. The D3 delta.

## 10. Proposed ScopeOfWork and b7-plan text

These edit commitments. They are written against the ScopeOfWork as edited by epoch 4 (#1461, main `084ff961f8`) and, since it lands first, by O4 (o4-source-extension.md §10).

**Decided, applied in the repair's implementation PR** (owner, 2026-10-18, "I concur, adopt option 2"; section 11):
- **DEL-04-08 ScopeOfWork REQ-003 (e).** One sub-bullet is added directly under "(e) No relaxation. No search, verification, tolerance or domain limit is relaxed to obtain a bound.", which stays unchanged, as do its existing sub-bullets:
  - "A search's stop tolerance bounds the search's own convergence on the problem it solves. Where a declared input is itself an interval (the admitted lane's represented-Z hull), the search converges at the interval's binding endpoint under the unchanged tolerance and limits, the input's width is carried soundly into the result's enclosure, and the unchanged gate decides publication."

It is applied in that PR, when its meaning comes true; not before, and not in O4.

**Decided by the owner, applied when the repair lands** (owner, 2026-10-20, "I approve 1A, 2A, 3A."; 2A, relayed by HELP_HUMAN). As decided:
- REQ-006 (b) is deleted, leaving a retired marker in (a)'s style: "[retired by the W1 coverage repair, #1484]". The optional sentence is added as a new item after (e), titled "(f) Loaded-member maxima too wide for the gate.", with the sentence below verbatim.
- REQ-007 (b) is deleted or marked discharged in (a)'s style; the PR says which.
- AC-003's third bullet and the VER-003 replacement, as below.
- **Split between the two PRs.** Corner 2's PR carries only its own parts: the halfway half of the VER-003 replacement and REQ-006 (b)'s halfway-scale bullet. Everything else lands with corner 1.

The text as proposed (on the repair's discharge):
- **DEL-04-08 ScopeOfWork:**
  - **REQ-006 (b).** Delete the item, the two corners being repaired. Optionally record, under REQ-006 (d) or as a sentence in REQ-001's route-L domain: "A loaded member's maximum whose certified hull is too wide for the unchanged gate keeps its ordinary result; the case is unavailable with its named cause." That records the inherited corner as a stated limit, if the owner wants it stated.
  - **REQ-007 (b).** Delete on discharge.
  - **AC-003,** third bullet. Remove "the two W1 corner cases … keep their internal refusals and fall back". After O4 the bullet carries nothing else, so it is removed whole.
  - **VER-003.** Replace `w1_loaded_maximum_refuses_irreducible_represented_section_width` and `w1_loaded_scale_halfway_keeps_the_existing_g5b_refusal` with `w1_represented_z_hull_converges_on_ordinary_sections` and `w1_loaded_scale_halfway_publishes`.

- **b7-plan,** item 6: its design pointer is in place (added after epoch 4). When the repair lands, b7-plan's item-6 paragraph is replaced by a one-line "as built" pointer, as items 1–3 were.

**DEL-04-08 Design `w1-member-load-certification.md` §4.4** (edited when the repair lands, as an as-built revision; it follows the decided REQ-003 (e) clause):
- the "Stop" bullet becomes: "When U − L ≤ 2^-64·L. The tolerance bounds the search's own convergence on the problem it solves. On the admitted lane, whose represented Z is the interval hull(Ẑ, Î/c), the search converges at the hull's binding endpoint, the smaller Z, where the field is largest, under the unchanged tolerance and limits. The lane returns [the lower evaluated over the whole hull, U], so the hull's width is carried soundly into the enclosure, and the unchanged gate decides the projected row (REQ-003 (e))."
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

**Decided by the owner:**
- **Corner 1 and REQ-003 (e)** (owner, 2026-10-18, through Agent 0: "I concur, adopt option 2"). Four options were put to the owner: (1) the section 1.2 repair alone, read as compliant; (2) the section 1.2 repair plus an explicit clause in REQ-003 (e); (3) narrowing DEF-O's admitted Z to Ẑ; (4) keeping the corner. Option 2 is decided:
  - the section 1.2 repair stands unchanged;
  - REQ-003 (e) gains one sub-bullet directly under "(e) No relaxation. No search, verification, tolerance or domain limit is relaxed to obtain a bound.", which stays unchanged, as do its existing sub-bullets: "A search's stop tolerance bounds the search's own convergence on the problem it solves. Where a declared input is itself an interval (the admitted lane's represented-Z hull), the search converges at the interval's binding endpoint under the unchanged tolerance and limits, the input's width is carried soundly into the result's enclosure, and the unchanged gate decides publication."
  - the clause is applied in the repair's implementation PR, when its meaning comes true; not before, and not in O4 (section 10);
  - stage 2 §4.4's "Stop" text is edited to agree with it when the repair lands (section 10).
  Narrowing was not chosen. The census's pre-registered escalation (3.3) and the DEF-O-narrowing contingency below stay as designed. Corner 2's repair never depended on this decision.

- **Z1 and section 10** (owner, 2026-10-20, relayed by HELP_HUMAN: "I approve 1A, 2A, 3A."):
  - **1A.** Z1, the single Ẑ in DEF-O and DEF-E, is not adopted; it was ruled once for both routes with this census's tables and W1-E's (w1e-sharper-exact-coverage.md §6.3, §9). Corner 1 lands as designed (1.2), with the REQ-003 (e) sub-bullet and the §4.4 "Stop" text above; every stop condition of 3.3 still applies. The other coverage options are not pursued unless the owner later chooses to change the claim for coverage.
  - **2A.** Section 10's five proposed ScopeOfWork edits are applied as section 10 now records.
  - **3A.** The REQ-006 (e) amendment of w1e §6.1 is applied on main by HELP_HUMAN through #1482 (`816ec66225`; w1e §6.1).

**Contingent, to HELP_HUMAN:** narrowing DEF-O's admitted Z, if the census's pre-registered escalation rule fires (3.3), and a D3 shortfall.

**Reserved and untouched:**
- Decision 22;
- TBD-001 and TBD-002;
- G10 and B8;
- the frozen UI benchmark;
- release.

## 12. Blast radius

**Code:**
- **Frame kernel (DEL-04-01):** `structural/retained/product_certificate/loaded_maximum.rs` (the driven search and the hull-valid lower) and `final_case.rs` (retaining and exposing the final scales; the admitted-lane call site); the census probe module and its two lane switches (`product_certificate/census_probe.rs`, `source_residual.rs` `coefficients`, `final_case.rs` `recipe_reduced`), compiled only under `census-probe`; the feature in `Cargo.toml`; tests in `tests/retained_k4/`.
- **Product physics (DEL-04-07):** `retained_wire.rs` `selection` (the scale source); `retained_memory.rs` (registered profile); `Cargo.toml` (the forwarded feature); the W1 census and test files (`w1_domain_stage1_tests.rs`, `w1_source_census_tests.rs`, `retained_facade_tests.rs`), including the census targets and the feature guard test.
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
