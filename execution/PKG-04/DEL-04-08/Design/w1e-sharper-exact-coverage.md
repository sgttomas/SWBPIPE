# W1-E coverage: the sharper allowance against both readouts

This design examines coverage finding (a) of O4 (PR #1468, "Coverage findings"; [o4-source-extension.md](o4-source-extension.md) §5, "Coverage"). On O4's judged set, W1-E (`physics-retained-1`) publishes almost no successors. Where it attempts a case, its existing proof fails closed most often at the SharperExact predicate (44 of the attempted runs; alpha 24, preparation 16). The owner decided on 2026-10-20 ("All as recommended") that a remedy be designed now. The design goal is a remedy that keeps the verified claim and relaxes no verification.

**Status: design only.** Nothing here changes a product file, a reviewed input or a ScopeOfWork. The basis is main `fa019d9cef` (O4 merged as #1468). Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

**Revision 2** answers the first round of dual review (Claude Opus 5.5 and Codex). The changes:
- section 2's bound is restated. Revision 1 claimed that no proof could change the outcome, but a sound proof whose enclosure is wider and asymmetric can move the published value. The universal bound is now taken over every published value (2.1), and the fixed-projection ceiling is stated with its premise (2.2);
- U1 is split into its three forms, each with its own ceiling. Moving SharperExact alone to the raw value would change nothing, because SharperBinary64 still judges the SI value (5);
- the φ ≈ 2 failures are absolute-class rows, which R1 as defined does not reach (3.3, 5);
- the section-leaf cause now includes the shear modulus and the shear-area leaf;
- the source-twin, row and readout-gap figures are corrected (3.2, 3.3);
- the census probe is an opt-in feature outside every test build by default, and its counterfactuals recompute the projection, scales and classes. The switches that change a lane's problem re-solve it (6.2);
- the REQ-006 (e) text is qualified, and the AC-006 test input is taken from the census (6.1);
- the W1 coverage repair's ScopeOfWork edit and its corner-2 scale fix are sequenced (0).

**Revision 3** answers the second round. The changes:
- the fixed-projection ceiling keeps its premise wherever it is concluded (Summary, 4, 6.1). An enclosure deliberately shaped to move the projection is now option U1 (iv), whose admissibility is a reading of DEF-E for the owner (5);
- section 2.1's bound now ranges over both classification classes and every admissible scale up to an upper estimate;
- that estimate now includes the displacement and support magnitudes, and section 3 is recomputed with it. The documents counted do not change; some row counts do;
- R1 is split into R1, which changes only the sharper checks, and R1′, which also projects the selected lane (5);
- the p = 512 floor is exempted in both census rules, and the W1 repair's probe mechanism is stated in full (0, 6.2);
- wording repairs in U1 (ii) and in REQ-006 (e).

## Summary

- **The finding.** The product gate measures each published value against the hull of both readouts' certified enclosures, and every sound enclosure contains its readout's exact value. So whether a row can pass depends on the two readouts' exact values, the classification scale and the published value, not on how the proof reached them (section 2).
  - With the method's projection and enclosures as narrow as the existing proof's, no sensitive judged document can publish.
  - Even with any published value whatever, at most two can.
- **The bounds, on O4's frozen references** (section 3; a design-time computation, no product build). There are 54 sensitive judged documents, the ones designed to trigger W1-E.
  - *Any value (2.1).* Whatever the proof and whatever binary64 value is published, at most **2 of 54** could pass the unchanged gate (24 of all 172 documents). The two are `SHORT-MEMBER|zero|eb` and `SHORT-MEMBER|zero|timo`.
  - *The fixed projection (2.2).* With the definition's own projection and enclosures as narrow as the existing proof's, **0 of 54** can pass (13 of 172). None of those 13 is a sensitive design.
- **Three causes, all fixed by the method's definitions** (section 3.3):
  1. *The represented-Z hull.* The admitted readout's bending stress is defined over hull(Ẑ, Î/c) (DEF-O and DEF-E `rows.component_stress`). That hull is up to about one binary64 unit wide.
  2. *The binary64 leaves.* The admitted readout uses binary64 A, I, J, Z, shear area and G, where the annular readout uses the exact annulus and the exact E/(2(1 + ν)). Stress rows inherit the difference directly, even where both readouts' actions agree exactly.
  3. *The published unit.* A millimetre or megapascal row is rounded once in its unit and again when the gate normalizes it to SI (DEF-E `projection.H`; `final_case.rs` `normalize`). On its own, that fails about 8 % of millimetre rows and 2 % of megapascal rows at the body scale.
  
  Where only one cause is present, the represented-Z hull blocks 46 of the 54 sensitive documents, the unit rounding 22 to 26, and the readout gap 9. Removing the hull and the unit rounding together lets 45 of 54 pass. Removing all three lets all 54 pass, before alpha and preparation.
- **The suggested remedies** (section 4).
  - *A better proof.* It cannot raise the any-value bound. Under the fixed projection, it can change the outcome only by moving the hull's midpoint into another projection cell. An enclosure shaped for that is a choice of the published value; whether DEF-E admits it is the owner's reading (U1 (iv)).
  - *A better projection.* That is a DEF-E change, bounded by the any-value figure.
  - *A representation that makes the readouts agree.* Binary64 leaves cannot equal an exact annulus.
  - *Certifying against a readout hull or the exact problem.* The first is what the gate does today. The second drops a readout from the claim.
  - *Stating the corner as a limit.* This needs no product change.
- **Recommended path (no ruling on any definition or on the claim):**
  - state the corner as a limit (proposed REQ-006 (e), section 6.1; a ScopeOfWork edit, which is the owner's);
  - keep W1-E as built;
  - run the product-side confirmation census (section 6.2) before the owner rules on any definition lever.
  
  Expected coverage gain: none. Cost: one ScopeOfWork text, one maintained AC-006 test, and the census.
- **Options that would gain coverage** (section 5). Each changes a reviewed definition, the verified claim or a predicate, or (U1 (iv)) needs the owner's reading of one, so each needs the owner's ruling. No recommendation is made. Ceilings are for the 54 sensitive documents.

  | Option | What it changes | Ceiling alone | With Z1 |
  |---|---|---|---|
  | Z1 | the admitted Z becomes the single Ẑ | 24/54 | — |
  | U1 (i) | a search over the published unit value | 1/54 | 42/54 |
  | U1 (ii) | both sharper predicates judged on the raw unit value | 0/54 | 12/54 |
  | U1 (iii) | retained rows published in SI units | 2/54 | 45/54 |
  | U1 (iv) | lane enclosures shaped to move the projection's cell | at most 2/54 (any-value bound) | at most 45/54 |
  | R1 | the sharper checks against the annular readout only, with the dual-hull projection kept | 25/54 | 26/54 |
  | R1′ | R1, and the selected lane projected alone | 32/54 | — |
  | C1 | rows that fail the allowance become `not_covered`, and the successor publishes | 609 rows (about 6 % of the relative rows) would be `not_covered`, if a coverage law for the scales is also designed | — |

## 0. Basis and dependencies

**Basis.** Main `fa019d9cef`. Paths are relative to `projects/chirality-piping/`. The kernel is `core/solver/frame_kernel/src/structural/retained/`; PP is `core/product_physics/src/`.

| Fact | Where |
|---|---|
| The final gate judges each row against `hull(represented, geometric)`, the two lanes' enclosures. | kernel `product_certificate/final_case.rs` `check_intervals` (1536), call at 1659; `hull` (660) |
| SharperExact passes when h ≤ 2^-64·M + 2^-85·M + 2^-53·\|n\| + 2^-1074, with M = max(\|n\|, S) and h = max(n − lo, hi − n). SharperBinary64 applies the same allowance computed in binary64. Both judge n. | `gate` (1002–1107), `exact_test` (959), `distance` (952) |
| n is the published value normalized to SI in binary64: y/1000 for mm, y·10^6 for MPa. | `normalize` (598–604) |
| The published value is the projection of the hull's midpoint. For mm it is RN64(RN1024(mid·1000)). For MPa it is RN64(RN64(mid)/10^6). | `project_hull` (1961–2000); DEF-E `projection.H` |
| A row is relative-class when S ≥ 2^-988 and \|n\| ≥ 2^-34·S; otherwise it takes the absolute bound RU64(2^-64·S). S is the body's coupled scale, from the final published native and support rows, raised by the floor at p = 512. A stress row's S is S_force/A + k·S_moment/Z. | `gate`; `row_scales_reduced` (1313–1532); `check_intervals` (1544–1557, 1595–1615); adaptive `FLOOR_RATIO_BITS` (67) |
| The admitted lane's bending Z is hull(Î/c, Ẑ). | kernel `product_certificate.rs` (657–671); DEF-O/DEF-E `rows.component_stress` |
| The lanes run at the case's selected precision, 128 to 1024 bits. | `final_case.rs` (about 1931), `source_residual::source_residual_for_law` (`source_residual.rs:965`) |
| The O4 SharperExact count comes from this gate (`Cause::Predicate`). | PP `o4_m3_judging_tests.rs` `w1_causes` (260–296) |
| The frozen references give both readouts' exact rows to 40 significant digits. | `validation/references/o4_retained_source/` (README "The two readouts") |

**Assumptions taken from concurrent work, and sequencing.**

- **Stack-headroom repair (b7-plan item 5a).** It shrinks debug stack frames in `freeze_case`, `source_residual::recover` and `member` on the retained W1 path, and puts the six stack witnesses into a maintained check.
  - The recommended path changes no product code, so it has no interaction with 5a.
  - The census probe (6.2) sits beside the gate, on that chain. Assumption: the witnesses are built and run without the probe. A feature reached through `[dev-dependencies]` would compile into every test build of the crate, the witness build included; `exact-operation-count` is reached that way through `core/product_physics/Cargo.toml`. So the probe is an opt-in feature that no `[dev-dependencies]` entry enables. It is switched on only by the census commands' `--features`, and a guard test asserts that it is off in the ordinary test build.
  - The census runs after 5a merges. It neither uses nor reports witness outcomes from its own build.
  - Any section 5 option that is later implemented changes code on that chain. It follows 5a, and it re-runs the six witnesses at R/64 in the registered profile.
- **W1 coverage repair** ([w1-coverage-repair.md](w1-coverage-repair.md), epoch 6, after 5a). This design relies on five of its facts:
  - its census designs the counterfactual switches (a) "the admitted lane's Z set to the single value Ẑ", (b) "both lanes given the same section" and (c) "both lanes given the same whole readout" (§3.3);
  - its pre-registered escalation names Z1 as "the next lever", which "changes a reviewed definition's meaning and the verified claim, so it needs a ruling" (§3.3);
  - it leaves the two-lane section difference out of scope ("inherited by all stress rows and not created here"; W1 Design §4.4, last corner);
  - **its own ScopeOfWork edit.** The owner decided (2026-10-18, "I concur, adopt option 2") that REQ-003 (e) gains a clause, applied in that repair's implementation PR. This design proposes REQ-006 (e) and classifies options under REQ-003 (e). If both edits go to the owner together, REQ-006 (e) is written against REQ-003 (e) as amended;
  - **its corner-2 fix**, which writes the gate's final scales to the receipt. That changes precommit G5b outcomes but not the gate, S or any ceiling here. A census run after that fix sees cases reach the gate that G5b withdrew before.
  
  Sequencing: the confirmation census (6.2) shares switches (a) to (c) with the W1 repair's census, so it runs on that census's branch, or after it. The W1 repair's census specifies a kernel `#[cfg(test)]` hook and "a dev-only cargo feature where `product_physics` tests need it, as RF's `exact-operation-count` is" (§3.2). The first cannot reach product_physics tests. The second is reached through `[dev-dependencies]` and so compiles into the witness build. The two probes are therefore reconciled into one opt-in module under 5a's assumption above, and w1-coverage-repair.md §3.2 takes the same opt-in amendment when that shared module is built (a design edit for the W1 repair's owner). Z1 is one question for both routes, DEF-O on route L and DEF-E on route E. It goes to the owner once, with both censuses' tables.
- **The O4 design and references.** As built (o4-source-extension.md revision 7). This design reads the frozen references and writes nothing under `validation/references`.

## 1. The verified claim and the gate

REQ-001: a `relative_verified` row's value "lies within 10^-9·|value| of the exact solution, and also within the kernel's sharper allowance (DEF-O `acceptance.final`)". "The bound holds for both of the formation's stiffness readouts: the admitted binary64 section products and the exact annulus geometry." DEF-E's `acceptance.final` is the "direct error certificate of the actual frozen raw and SI values against both physical readouts; unchanged absolute bound, two sharper predicates, raw/SI decimal checks …".

The gate applies four predicates to a relative row: SharperExact, SharperBinary64, DecimalSi and DecimalRaw. The 10^-9 predicates are far from binding: the worst published relative margin on O4's judged successors is 1.1e-7 of the tolerance. The binding predicates are the two sharper ones, which agree on every judged row.
- Their allowance is about 2^-53·|n| (one half to one binary64 unit of n), plus 2^-64·max(|n|, S). The second term matters only for rows well below their body's scale.
- So the sharper predicates ask, in effect, that the SI value of the published number be a faithful rounding of every readout value at once.

An absolute-class row must lie within RU64(2^-64·S) of both readouts.

## 2. What a proof can and cannot change

Let x_A be the admitted readout's exact value of a row and x_G the annular readout's. For a bending stress, x_A is the whole interval over hull(Ẑ, Î/c). Write [ℓ, u] = hull(x_A, x_G). Every sound lane enclosure contains its readout's exact value, so the gate's hull H ⊇ [ℓ, u]. Hence, for any n, h(n, H) ≥ max(n − ℓ, u − n).

### 2.1 The any-value bound

Let S_hi be an upper bound on the row's true scale (3.1). For any n, the row is judged in one of two classes:
- **Relative class.** It requires |n| ≥ 2^-34·S. Its allowance grows with S, so the best admissible S is min(S_hi, 2^34·|n|).
- **Absolute class.** It requires S > 2^34·|n|. Its bound RU64(2^-64·S) grows with S, so the best admissible S is S_hi.

A row can pass for some n and some S ≤ S_hi only if one of the following candidates passes, each judged in its own best class:
- **The SI values nearest the midpoint (ℓ + u)/2.** Within one class, a step of n changes the allowance by a small fraction of a unit: about 2^-53 of a unit, or up to about 2^-30 where S = 2^34·|n|. Meanwhile h changes by up to a unit. So it suffices to evaluate the seven values nearest the midpoint.
- **The class boundaries.** The smallest-magnitude relative-class values, ±RU64(2^-34·S_hi), are evaluated too, because the allowance jumps there. The Codex reviewer constructed a fixed-S row that passes only at the boundary.

The bound therefore holds whatever the proof, whatever the enclosures' widths or shape, and whatever value is published, including a value chosen by a search. Its premises are S ≤ S_hi and the ordinary-scale regime S ≥ 2^-988. Below 2^-988 the gate uses `small_row_bound` (`final_case.rs` about 1043; `directed/certificate.rs` about 167), which this argument does not model. On O4's judged set the smallest positive scale is of order 10^-7 (6.1 × 10^-7 and 8.2 × 10^-7 in the two reviewers' replays), and on the source-control twins about 5 × 10^-17, so that branch is not reached there. The p = 512 floor, which section 3 does not model, can raise the true S above S_hi for near-zero bodies.

### 2.2 The ceiling under the fixed projection

DEF-E fixes the published value as the projection H of the hull's midpoint, with `search_or_retry: none`.
- **The premise.** When the lanes' enclosures are narrow enough that H's midpoint stays in the same projection cell as (ℓ + u)/2, the published value is projection((ℓ + u)/2). The row can then pass only if it passes with H = [ℓ, u].
- **The existing lanes.** They work at the selected precision, 128 to 1024 bits. Their widths are expected to be many orders below a binary64 unit. The census records the widths and the cell test (6.2) rather than assuming them.
- **For SI rows,** RN64 of the midpoint minimizes h, so the fixed projection and the any-value bound agree up to the dependence of the allowance on n.
- **For unit rows** they differ. A sound proof that returned a wider, asymmetric enclosure could move the midpoint into another cell, and so publish a different value that passes.
  - The Codex reviewer constructed sound, outward-rounded binary1024 enclosures for `SHORT-MEMBER|zero|timo` that clear all five of its failing rows under the unchanged projection and all four predicates: `N2.uz` and four bending rows.
  - That is not a tighter proof but a choice of the published value through the enclosure's shape.
  - DEF-E fixes the lanes' recipes ("fresh residual and alpha check …; fresh final residual/radius and complete recovery") and the projection ("`search_or_retry: none`"). Whether those texts exclude a lane that widens its enclosure to steer the projection is a reading of a reviewed definition. That reading is the owner's, and section 5 presents it as U1 (iv).
  - Its reach is bounded by the any-value bound (2.1): at most 2 of 54 sensitive documents, both `SHORT-MEMBER`.

## 3. The ceilings, computed on the frozen references

### 3.1 Method

A standard-library Python replay was run in `/tmp` scratch. It is not committed, and it reads only `validation/references/o4_retained_source/{cases,oracle,source_twins_cases,source_twins_oracle}.json`. For every case and every row it forms [ℓ, u] from both readouts' 40-digit values in exact rationals. It then forms n, either by DEF-E's H with the unit conversions and `normalize` (the fixed projection), or as each of the seven SI values nearest the midpoint (any value). Finally it applies the gate's class rule, SharperExact, or the absolute bound RU64(2^-64·S), exactly.

- **Rows judged:**
  - free nodal translations (mm) and rotations (rad);
  - member end actions and the five station section actions (N, N·m);
  - stresses on both bases (MPa), with the admitted bending rows over the represented-Z interval;
  - support reaction components (N, N·m).
- **Rows not judged:** circular maxima (their own projection rule), displacement and support magnitudes, and the not-covered derived rows. Leaving rows out can only raise the pass counts.
- **The scales S are an upper estimate:**
  - the per-kind maximum over every row of the body and both readouts, a superset of the product's native and support rows. It includes the displacement and support magnitudes, which are native rows (`recover.rs` 113–119, 184–196; `final_case.rs` 1500–1515), bounded above by the sum of the components' magnitudes;
  - multiplied by 1 + 2^-40, to cover the binary64 rounding of the published values;
  - coupled by the body's bounding-box diagonal, as `couple` and `body_extent` do;
  - for stresses, S_force/Â + S_moment/Ẑ with the prepared leaves.
  
  Every row is judged at its best admissible S ≤ S_hi in each class (2.1). The Claude reviewer's replay with S formed from the published |n|, without the magnitudes, reproduced revision 1's table. Its round-3 replay with the magnitudes and the best-class rule reproduces this revision's table, and the Codex reviewer's magnitude-inclusive replay gave the same both-readout counts. The p = 512 Φ floor is not modelled; it raises S only for near-zero bodies (o4-source-extension.md §4.2).
- **Robustness.** Re-running with every reference value perturbed independently by ±10^-37 relative, under three seeds, changed no verdict. A reviewer's run at ±10^-24 changed none either.
- **Independent reproduction.** Both reviewers reproduced every cell of revision 1's table independently, from their own code. That table's last two rows (no unit rounding) equal the any-value rows here for the two readouts together and against the annular readout. They also reproduced the U1 (i) and U1 (ii) figures of section 5 or the facts they rest on.

### 3.2 Results

Documents whose every judged row passes. Each cell gives all judged documents (of 172) and then the sensitive ones (of 54; tag `sensitive_design`: soft springs at k/a ≈ 1e-8 and 1e-12, stiff contrast and short members). The column says which readouts the gate is applied against.

| Represented Z | Published value | Both readouts (claim as defined) | Admitted only | Annular only |
|---|---|---|---|---|
| hull, as defined | fixed projection (2.2) | **13 / 0** | 30 / 1 | 97 / 32 |
| the single Ẑ | fixed projection | 47 / 24 | 82 / 28 | 97 / 32 |
| hull, as defined | any value (2.1) | **24 / 2** | 71 / 8 | 172 / 54 |
| the single Ẑ | any value | 79 / 45 | 172 / 54 | 172 / 54 |

For the two readouts together, the "any value" counts equal those computed with n = RN64 of the SI midpoint, that is, with no unit rounding.

- **The 13 judged documents that can pass as defined** are cantilevers, `FF-SYM`, `GUIDED` and two `PROP-PHI2-EXACT-A` Euler–Bernoulli documents. None is a sensitive design. They are the documents whose ordinary results are expected to pass their checks, so W1 does not attempt them on the plain binding.
- **Rows.** On the sensitive documents, 609 of 15,740 judged rows cannot pass as defined (3.9 %), all of them relative-class rows. That is about 6 % of the relative rows. Per document, between 93.4 % and 98.9 % of judged rows can pass (median 95.7 %).
  - Over all judged documents, 3,066 rows cannot pass under the fixed projection. 2,387 of them are megapascal stresses.
  - The 70 absolute-class rows that fail are all in the eight `PROP-PHI2-*` Timoshenko documents (3.3).
- **Source-control twins** (104). As defined, 82 can pass. Every failure is a bending stress row (26 rows in 22 twins: the J2 `fields` twins and the RF PIN-OFF-345 r1e-12 allowance twins).
  - With either the single Ẑ or no unit rounding, still 82 pass.
  - Against the annular readout alone, all 104 pass. Against the admitted readout alone, 102 pass: two reduced-section bending rows of the PIN-OFF-345 r1e-12 mill twins fail on the represented-Z hull, and they pass with the single Ẑ.
- **Consistency with the product.** O4 records 6 judged-set and 112 twin successors through W1's own trigger path, which attaches a separate trigger body. Both are within these ceilings: 6 ≤ 2 × 13 runs and 112 ≤ 2 × 82. The replay predicts that the 6 are among the 13, and the census checks it (6.2). On the plain binding, no judged document published.

### 3.3 The causes

- **The represented-Z hull.** The admitted lane's bending Z is the hull of two binary64 representations of one quantity. W1 coverage repair §3.1 measures its relative width: a median of 871 × 2^-64 (about 0.43 × 2^-53) across 3,142 sections, and zero only where Î/c is representable. A row's half-width then joins the projection's own rounding of up to 2^-53·|n|, so the hull alone can fail a row. Against the admitted readout alone, with no unit rounding, 8 of 54 sensitive documents pass; the hull blocks the other 46.
- **The binary64 leaves.** The admitted readout uses:
  - prepared A, I, J and Z, each the correctly rounded binary64 of the exact annulus value (the reference README, "The two readouts"; DEF-E `preparation`), so each differs from its exact value by up to half a unit, and π makes equality impossible;
  - Ĝ = RN64(E/RN64(2·RN64(1 + ν))), up to about 1.07 × 2^-53 from the exact G on the judged set;
  - for Timoshenko members, κ̂ in a fixed binary64 order and Â_s = RN(κ̂·Â), up to about 3.5 × 2^-53 and 2.9 × 2^-53 from κ_E and its shear area (reviewer measurements).
  
  Stress rows divide by the section leaves directly. So their two readouts differ by up to about 2^-53 relative even where the actions agree exactly, and every failing source-twin row is such a bending stress. On the sensitive designs, every document's largest readout gap on point rows is below 2^-53 relative; on the `SOFT-RUN` designs it is far below. With the single Ẑ and no unit rounding, the readout gap alone blocks 9 of 54 sensitive documents.
- **The published unit.** n = RN64(RN64(1000·m)/1000) for a millimetre row. This differs from m by up to about one unit of n, against an allowance of half a unit to one unit.
  - In an independent sampling of 200,000 random non-representable values, 8.4 % of millimetre rows and 2.3 % of megapascal rows fail on this alone at the body scale (|n| ≈ S). Both reviewers reproduced these rates.
  - On its own (one readout, single Ẑ, fixed projection), it blocks 22 of 54 sensitive documents against the annular readout and 26 against the admitted one.
- **The φ ≈ 2 documents.** In the eight `PROP-PHI2-*` Timoshenko documents, the near-end bending rows carry (2 − φ)/(4 + φ), which is zero, or nearly zero, in one readout and not in the other. Their readout gaps are at least 5 × 10^10 units of 2^-53. The failing rows there are absolute-class: for example, `PROP-PHI2-EXACT-A|zero|timo` `P1.end_i.My` exceeds its absolute bound about 258 times. They lie outside the sensitive set. No option of section 5 reaches them. R1 would reach them only if it were extended to judge the absolute bound against one readout too (5).

Each cause is fixed by a definition:
- the represented-Z hull, by DEF-O and DEF-E `rows.component_stress`;
- the leaves, by the preparation law `RP-PREPARED-ANNULUS-v1` and the admitted readout of REQ-001 ("the admitted binary64 section products");
- the unit rounding, by DEF-E `projection.H` and the published units of the pressure-2 base contract.

Only the represented Z and the unit rounding together clear most sensitive documents.

## 4. The suggested remedies, examined

1. **A tighter or different proof.** It cannot raise the any-value bound (2.1). Under the fixed projection, a proof can change the outcome only by moving the hull's midpoint into another projection cell. That chooses the published value through the enclosure's shape, which section 5 presents as U1 (iv), with its admissibility left to the owner's reading of DEF-E.
2. **A better projection.**
   - For SI rows, RN64 of the midpoint is already the best value.
   - For unit rows, any other recipe edits DEF-E `projection.H`. A deterministic recipe that targets the SI value is bounded by the any-value rows, because y ↦ RN64(y/1000) can skip binary64 values. A recipe that searches is U1 (i).
3. **A representation that makes the readouts agree.**
   - With binary64 leaves this is impossible: the exact annulus values are irrational multiples of π, and the best binary64 leaf is the correctly rounded one, which preparation already uses.
   - Leaves with more precision (a double-double A, for example) would no longer be "the admitted binary64 section products". The admitted readout would stop being the problem the ordinary route represents. That is a change to the verified claim.
   - A narrower variant prepares Ẑ as RN64(Î/c) instead of the rounding of the exact Z. That changes how the represented-Z hull's width is bounded, not necessarily its width on a given section. It changes the preparation law's meaning, so it belongs to Z1's family, and the census can measure it as a variant.
4. **Certifying against a readout hull or the exact problem.**
   - The gate already certifies against the hull of both readouts, which is what the claim requires.
   - Certifying against the exact problem alone (the annular readout) is R1, a claim change.
5. **Stating the corner as a limit.** No product change; a ScopeOfWork statement (section 6.1).

**Conclusion.**
- Within the unchanged claim, definitions, predicates and projection, and with enclosures as narrow as the existing proof's, no remedy can publish the sensitive documents this gap concerns.
- Allowing any published value, by whatever proof or route, at most 2 of 54 could pass.
- Every coverage-gaining remedy needs a ruling.

## 5. Options that need the owner's ruling (no recommendation)

The ceilings are for the 54 sensitive judged documents, from section 3's replay under the fixed projection unless stated otherwise. Alpha (24 runs) and preparation (16 runs) failures would still fail closed under every option. Implementing any option needs its own design, the independent numerical review, and an actual-source D3. Every option except U1 (iv) moves reviewed inputs and re-pins committed route-E successors and their DEF-E-bound hashes, and Z1 also route L's. U1 (iv) does so only if the owner's reading requires a definition change.

- **Z1. The admitted bending stress on the single Ẑ.**
  - *What changes:* DEF-O and DEF-E `rows.component_stress` ("admitted-K bending Z is hull(actual Z,I_K/c)"). The admitted readout's bending stress would mean M/Ẑ only.
  - *Ceiling:* alone, 0 → 24 of 54.
  - *Consequence:* the verified claim no longer covers a bending stress formed from the stiffness second moment, M·c/Î. Such a value would still lie within 10^-9, but not within the sharper allowance. It narrows the interval the gate measures against, so AX-005 ("A gate is never loosened to obtain a pass") bears on it as well as the definition ruling.
  - *Notes:* the W1 coverage repair's escalation names this lever for route L too (§3.3). One ruling should cover both routes.
- **U1. How a unit row's published value and SI value are judged.** Four forms, each with its own consequence:
  - **(i) A search** for the published unit value whose SI normalization passes, within ±4 binary64 steps.
    - *Ceiling:* 1/54 alone, 42/54 with Z1.
    - *Consequence:* REQ-003 (e) fixes "`projection_retry: false` and `search_or_retry: none`" as reviewed inputs. A search to obtain a pass is what REQ-003 (e) and AX-005 prohibit.
  - **(ii) Both sharper predicates judged on the raw unit value** instead of its SI normalization.
    - *Ceiling:* 0/54 alone, 12/54 with Z1. The megapascal raw value is itself rounded twice, so this form fails more rows than SI judging.
    - *Consequence:* changing only SharperExact would gain nothing, because SharperBinary64 would still judge the SI value; it could only add failures. Moving both weakens the SI-value guarantee from about 2^-53 to about 2^-52 relative. That is a predicate change on which REQ-003 (e) and AX-005 bear.
  - **(iii) Publishing the retained rows in SI units.**
    - *Ceiling:* 2/54 alone, 45/54 with Z1. These are the any-value rows of 3.2.
    - *Consequence:* it changes the pressure-2 base contract's units for every consumer.
  - **(iv) Lane enclosures shaped to move the projection's cell** (2.2).
    - *Ceiling:* at most the any-value bound: 2/54 alone, both `SHORT-MEMBER`; at most 45/54 with Z1.
    - *Consequence:* the published value would be chosen through the proof's enclosure rather than fixed by the readouts. Whether DEF-E's lane recipes and `search_or_retry: none` admit this is the owner's reading of a reviewed definition. If they admit it, the construction still needs its own design, with soundness, determinism and replay arguments. If they do not, it is a search by another route, and REQ-003 (e) and AX-005 bear on it.
- **R1. The sharper checks against one readout, with 10^-9 and the absolute bound kept for both.**
  - *What changes:* REQ-001's guarantee and DEF-E `acceptance.final`. The published value stays the projection of the dual-readout hull.
  - *Ceiling:* 25 of 54 against the annular readout (26 with Z1); 0 of 54 against the admitted one.
  - *R1′, which also projects the selected lane alone,* changes DEF-E `rows.component` ("H of hull(admitted_k functional,annular_source functional)") as well. Ceiling: 32 of 54 against the annular readout; 1 of 54 against the admitted one.
  - *Consequence:* a verification is removed from the gate, so REQ-003 (e) and AX-005 bear on it. Consumers relying on the sharper statement for the other readout lose it. R1 and R1′ leave the absolute bound judged against both readouts, so neither reaches the φ ≈ 2 failures (3.3). Reaching those would also need the absolute bound against one readout.
- **C1. Per-row coverage.** A relative-class row that fails the sharper allowance would publish as `not_covered` with its ordinary value, and the rest of the successor would publish.
  - *What changes:*
    - REQ-001's classification law (today a row of a verifiable kind is never `not_covered` for this reason);
    - the table's `accuracy_classification`;
    - DEF-E `acceptance.final`'s direct certificate of every row;
    - the reader's G3/G5c coverage checks.
  - *A coverage law it would also need.* `row_scales_reduced` takes S only from native and support rows and refuses missing native coverage. A `not_covered` native row would change other rows' S and classes, so C1 also needs a new scale and coverage law.
  - *Ceiling:* on the sensitive documents, about 94 % of the relative rows would publish verified, the rest `not_covered`, assuming S unchanged. Publication would then be limited by alpha and preparation.
  - *Consequence:* the reader cannot rederive why a verifiable row is not covered, and a successor no longer verifies all of its verifiable rows. No published bound is loosened, but the successor's acceptance condition is relaxed, so REQ-003 (e) and AX-005 bear on it.

**If the owner rules on none of these,** W1-E stays as built. It publishes where the corner does not bind and fails closed elsewhere, keeping the ordinary result with the one `RETAINED_PRECISION_UNAVAILABLE` notice (REQ-003 (b)).

## 6. The recommended path

### 6.1 Stating the corner as a limit (proposed ScopeOfWork text, not applied)

DEL-04-08 ScopeOfWork REQ-006 gains an item after (d):

> **(e) The sharper allowance against both readouts.** A retained-precision successor publishes only when each `relative_verified` row's value, normalized to SI, lies within the kernel's sharper allowance of both stiffness readouts' exact values. Three definitions of the method keep some rows from meeting it:
> - the admitted readout's bending stress ranges over hull(Ẑ, Î/c);
> - the admitted readout's binary64 section, shear and modulus leaves differ from the exact annulus readout's by up to a few binary64 units, and every stress row inherits the section leaves' difference;
> - a millimetre or megapascal value is rounded in its unit and again when normalized to SI.
>
> A case with such a row keeps its ordinary result with the one `RETAINED_PRECISION_UNAVAILABLE` notice. On O4's frozen judged set, the design-time ceiling admits no sensitive document under the method's projection with enclosures as narrow as the existing proof's (Design `w1e-sharper-exact-coverage.md`).

This text should be adopted after census 6.2 has confirmed the ceiling on the product, or with its last sentence held until then.

**AC-006 consequence.** "Each decline in REQ-006 is exercised by a maintained test that asserts its refusal." The implementation therefore adds one maintained test, which is test code and not this PR. Its input is a sensitive judged-set document that the census (6.2) records ending at the final gate with `Cause::Predicate` and `SharperExact`, in both solver modes. No committed record establishes such a document today. The test asserts:
- that refusal;
- the ordinary bytes plus the one notice, at the facade.

### 6.2 The product-side confirmation census (specified; executed later)

**Purpose.** Replace section 3's design-time replay with the product's own gate on its own enclosures, scales and projection, before any ruling on section 5.

- **The probe.** An opt-in cargo feature, forwarded from `product_physics` to the kernel. No `[dev-dependencies]` entry enables it, and it is on only when the census commands pass `--features` (section 0, 5a). A guard test asserts that it is off in the ordinary test build, and the stack witnesses run without it. The probe lives on the census branch. If it is ever merged, it is priced source, and the actual-source D3 comparison applies before merge (REQ-004).
- **What it records, at `check_intervals`, for every judged row:**
  - the class, n, S, h and the allowance, and all four predicate verdicts;
  - both lanes' enclosure widths relative to a binary64 unit of n, and whether H's midpoint lies in the same projection cell as the readouts' tight midpoint (2.2's premise; the tight midpoint comes from the references, test-side).
- **Counterfactual gates.** Each counterfactual recomputes the projection from its own hull, the coupled scales from the counterfactual published rows, and the classes, and then judges every predicate.
  - Recomputed from the lanes' actual enclosures, with no re-solve:
    - (a) the admitted Z set to Ẑ;
    - (d) form U1 (iii), n = RN64 of the SI midpoint;
    - (d′) form U1 (i), the ±4-step unit search;
    - (d″) form U1 (ii), raw-unit judging of both sharper predicates;
    - (e) the sharper checks against each lane alone, with the dual-hull projection kept (R1);
    - (e′) each lane alone, projected alone (R1′);
    - the combinations (a)+(d), (a)+(d′), (a)+(d″) and (a)+(e).
  - With a re-solve of the counterfactual lanes: the W1 coverage repair's switches (b), the same section in both lanes, and (c), the same whole readout. Those change the lanes' problem, so their enclosures are certified afresh through the existing lane construction (`source_residual.rs` about 1040–1147). The pilot prices them.
- **Inputs.**
  - The O4 judged set and the source-control twins, in both solver modes, on the plain binding and the augmented binding (`o4_m3_judging_tests.rs` `augmented`).
  - The committed route-E successors and RV120's inputs, as regression. With the probe on, every verdict and published byte must equal the probe-off run, input for input.
- **Records.** For each attempted case:
  - its first failure: preparation, native, alpha or predicate, with the row;
  - for each failing row, its attribution class, using W1 coverage repair §3.3's classes extended by (d), (d′), (d″) and (e).
  
  A case that never reaches the gate is `not_reached`, with its stopping stage, and is kept out of the gate's denominators.
- **Report.** Section 3.2's tables, from the product, in the implementation PR or as one revision of this section, with the alpha, preparation and `not_reached` counts beside them.
- **Pre-registered reading.**
  - Under the fixed projection, a document can exceed section 3.2's ceiling only by failing the premise check. That would show an enclosure moving its projection cell, or the p = 512 floor raising S. Any such document is listed, with its widths and S.
  - A document above the any-value bound stops the census for diagnosis, because that contradicts section 2.1. The exception is a document whose S the p = 512 floor raises above section 3's estimate; it is listed with its S, as above.
  - If no sensitive document can pass the unchanged gate, section 6.1's text stands, and section 5's options go to the owner with the product's tables.
- **What it needs.**
  - A debug build of the `frame_kernel` and `product_physics` tests with the feature, under the host's heavy-run slot and memory rules.
  - Private drivers only, so no registered identity is needed.
  - It runs after 5a merges, and with or after the W1 coverage repair's census (section 0).
  - A pilot of ten documents measures time and memory first.
  - It writes nothing under `validation/references` or the fixtures.

## 7. What stays unchanged

Every gate, predicate, tolerance, limit, precision ladder and projection; DEF-O, DEF-E and the tables; the verified claim; every committed byte. The recommended path adds a limit statement and, later, one maintained test. The census probe stays opt-in and off the production and witness builds.

## 8. Verification of this design

- The design rests on code facts (section 0) and on a replay of frozen, refuted references.
- The replay's arithmetic is exact and robust to perturbation (3.1). Both independent reviewers reproduced revision 1's table from their own code.
- The product census (6.2) is the product-side check.
- The dual review (Claude Opus 5.5 and Codex) ran on revisions 1 and 2. Revisions 2 and 3 answer them.

## 9. Reserved for the owner

**Owner decisions, 2026-10-20** ("For the five decisions you put to me I approve your recommended option in each case"):
- **REQ-006 (e).** HELP_HUMAN applies section 6.1's text to the ScopeOfWork, and reports it, once census 6.2 reproduces the ceiling on the product. The census runs in b7-plan's epoch 6.
- **Coverage options.** Z1 is decided together with the W1 coverage repair's own definition question. The other options are not pursued unless the owner later chooses to change the claim for coverage.

The options as presented:

- The REQ-006 (e) text (6.1). If it is not adopted, the corner stays an unstated fail-closed outcome.
- Any of Z1, U1 (i)–(iv), R1, R1′ or C1 (section 5), each with its consequence. U1 (iv) also needs the owner's reading of DEF-E. No recommendation is made.
