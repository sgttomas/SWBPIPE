# W1-E coverage: the sharper allowance against both readouts

This design examines coverage finding (a) of O4 (PR #1468, "Coverage findings"; [o4-source-extension.md](o4-source-extension.md) §5, "Coverage"). On O4's judged set, W1-E (`physics-retained-1`) publishes almost no successors. Where it attempts a case, its existing proof fails closed most often at the SharperExact predicate (44 of the attempted runs; alpha 24, preparation 16). The owner decided on 2026-10-20 ("All as recommended") that a remedy be designed now. The design goal is a remedy that keeps the verified claim and relaxes no verification.

**Status: design only.** Nothing here changes a product file, a reviewed input or a ScopeOfWork. The basis is main `fa019d9cef` (O4 merged as #1468). Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

## Summary

- **The finding.** The gap cannot be closed by any change to the proof. The product gate measures each published value against the hull of both readouts' certified enclosures. Every sound enclosure contains its readout's exact value, so a row can pass only if it would pass with exact, zero-width enclosures (section 2). That depends only on the document, the definitions and the projection, not on the proof.
- **The ceiling, measured on O4's frozen references** (section 3; a design-time computation with no product build). With exact enclosures, no sensitive judged document can pass the unchanged gate: **0 of 54**. These are the documents designed to trigger W1-E. Over all 172 judged documents, 13 can pass, and none of the 13 is a sensitive design.
- **Three causes, all fixed by the method's definitions** (section 3.3). Each alone blocks most sensitive documents:
  1. *The represented-Z hull.* The admitted readout's bending stress is defined over hull(Ẑ, Î/c) (DEF-O and DEF-E `rows.component_stress`). That hull is up to about one binary64 unit wide.
  2. *The section leaves.* The admitted readout's A, I, J and Z are binary64 roundings of exact annulus values, which involve π. Every stress row inherits that difference directly, even where both readouts' actions agree exactly. Solved rows mostly inherit it too, but on the sensitive designs the readout gap stays below 2^-53 relative.
  3. *The published unit.* A millimetre or megapascal row is rounded once in its unit and again when the gate normalizes it to SI (DEF-E `projection.H`; `final_case.rs` `normalize`). On its own, that fails about 8 % of millimetre rows and 2 % of megapascal rows at the body scale.
  
  Removing the first and third together would let 45 of the 54 sensitive documents pass. Removing all three would let all 54 pass, before alpha and preparation.
- **What that means for the suggested remedies** (section 4).
  - *A better proof or projection.* Neither can help. The projection already takes the hull's midpoint, which is optimal for SI rows. For unit rows, the projection is a DEF-E definition with no search.
  - *A representation that makes the readouts agree.* Binary64 leaves cannot equal an exact annulus. Leaves with more precision would change what the admitted readout is, and that is a change to the verified claim.
  - *Certifying against a readout hull.* The gate already does this. Certifying against the exact problem alone would drop a readout from the claim.
  - *Stating the corner as a limit.* This needs no product change.
- **Recommended path (no ruling on definitions needed):**
  - state the corner as a limit (proposed REQ-006 (e), section 6.1; a ScopeOfWork edit, which is the owner's);
  - keep W1-E as built;
  - run a product-side confirmation census (section 6.2) before the owner rules on any definition lever.
  
  Expected coverage gain: none. Cost: one ScopeOfWork text, one maintained AC-006 test, and the census.
- **Options that would gain coverage** (section 5). Each changes a reviewed definition, the verified claim or a predicate, so each needs the owner's ruling. No recommendation is made.

  | Option | What it changes | Ceiling, sensitive documents |
  |---|---|---|
  | Z1 | the admitted Z becomes the single Ẑ | 24/54 |
  | U1 | how a unit row's SI value is judged | 2/54 alone; 45/54 with Z1 |
  | R1 | the sharper allowance claimed against one readout | 32/54 (annular readout) |
  | C1 | rows that fail the sharper allowance become `not_covered`, and the successor publishes | about 94 % of the relative rows on the sensitive documents stay verified |

## 0. Basis and dependencies

**Basis.** Main `fa019d9cef`. Paths are relative to `projects/chirality-piping/`. The kernel is `core/solver/frame_kernel/src/structural/retained/`; PP is `core/product_physics/src/`.

| Fact | Where |
|---|---|
| The final gate judges each row against `hull(represented, geometric)`, the two lanes' enclosures. | kernel `product_certificate/final_case.rs` `check_intervals` (1536), call at 1659; `hull` (660) |
| SharperExact passes when h ≤ 2^-64·M + 2^-85·M + 2^-53·\|n\| + 2^-1074, with M = max(\|n\|, S) and h = max(n − lo, hi − n). The test is evaluated exactly. | `gate` (1002–1077), `exact_test` (959), `distance` (952) |
| n is the published value normalized to SI in binary64: y/1000 for mm, y·10^6 for MPa. | `normalize` (598–604) |
| The published value is the projection of the hull's midpoint. For mm it is RN64(RN1024(mid·1000)). For MPa it is RN64(RN64(mid)/10^6). | `project_hull` (1961–2000); DEF-E `projection.H` |
| A row is relative-class when S ≥ 2^-988 and \|n\| ≥ 2^-34·S. S is the body's coupled scale, from the final published native and support rows; a stress row's S is S_force/A + k·S_moment/Z. | `gate`; `row_scales_reduced` (1313–1532); `check_intervals` (1544–1557, 1595–1615); adaptive `FLOOR_RATIO_BITS` (67) |
| The admitted lane's bending Z is hull(Î/c, Ẑ). | kernel `product_certificate.rs` (657–671); DEF-O/DEF-E `rows.component_stress` |
| The O4 SharperExact count comes from this gate (`Cause::Predicate`). | PP `o4_m3_judging_tests.rs` `w1_causes` (260–296) |
| The frozen references give both readouts' exact rows to 40 significant digits. | `validation/references/o4_retained_source/` (README "The two readouts") |

**Assumptions taken from concurrent work, and sequencing.**

- **Stack-headroom repair (b7-plan item 5a).** It shrinks debug stack frames in `freeze_case`, `source_residual::recover` and `member` on the retained W1 path.
  - The recommended path changes no product code, so it has no interaction with 5a.
  - The census (6.2) adds a probe to the gate's caller, which is on that chain. Assumption: 5a's frames are measured in a build without the census probe. The probe is therefore a dev-only cargo feature, not `#[cfg(test)]`, so the witness build compiles none of it. The census runs after 5a merges. It does not use or report witness outcomes from its own build.
  - Any option in section 5 that is later implemented changes code on that chain. It follows 5a, and it re-runs the six stack witnesses at R/64 in the registered profile.
- **W1 coverage repair** ([w1-coverage-repair.md](w1-coverage-repair.md), epoch 6, after 5a). Three of its premises are reused here:
  - its census already designs the counterfactual switches (a) "the admitted lane's Z set to the single value Ẑ", (b) "both lanes given the same section" and (c) "both lanes given the same whole readout" (§3.3);
  - its pre-registered escalation names Z1 as "the next lever", which "changes a reviewed definition's meaning and the verified claim, so it needs a ruling" (§3.3);
  - it states that the two-lane section difference "is inherited by all stress rows and not created here" and leaves it out of scope (W1 Design §4.4, last corner).
  
  Sequencing: the confirmation census (6.2) reuses switches (a) to (c) and adds (d), the unit switch. So it runs on the W1 repair's census branch, or after it, and shares its probe module. Z1 is one question for both routes: DEF-O on route L and DEF-E on route E. It goes to the owner once, with both censuses' tables, and not separately per route.
- **The O4 design and references.** As built (o4-source-extension.md revision 7). This design reads the frozen references and writes nothing under `validation/references`.

## 1. The verified claim and the gate

REQ-001: a `relative_verified` row's value "lies within 10^-9·|value| of the exact solution, and also within the kernel's sharper allowance (DEF-O `acceptance.final`)". "The bound holds for both of the formation's stiffness readouts: the admitted binary64 section products and the exact annulus geometry." DEF-E's `acceptance.final` is the "direct error certificate of the actual frozen raw and SI values against both physical readouts; unchanged absolute bound, two sharper predicates, raw/SI decimal checks …".

The gate applies four predicates to a relative row: SharperExact, SharperBinary64, DecimalSi and DecimalRaw. The 10^-9 predicates are far from binding: the worst published relative margin on O4's judged successors is 1.1e-7 of the tolerance. The binding predicate is SharperExact. Its allowance is about 2^-53·|n|, or between one half and one binary64 unit of n, plus 2^-64·max(|n|, S). The second term matters only for rows well below their body's scale. So SharperExact asks, in effect, that the SI value of the published number be a faithful rounding of every readout value at once.

## 2. A necessary condition that no proof can change

Let x_A be the admitted readout's exact value of a row and x_G the annular readout's. For a bending stress, x_A is the whole interval over hull(Ẑ, Î/c). Every sound lane enclosure contains its readout's exact value, so the gate's hull H ⊇ hull(x_A, x_G) = [ℓ, u]. Then for any n, h(n, H) ≥ max(n − ℓ, u − n).

The published n is a fixed function of H's midpoint (DEF-E `projection.H`, `search_or_retry: none`). At 1024 bits the lanes' enclosure widths are of order 2^-900 relative, so H's midpoint equals (ℓ + u)/2 except where the projection is decided within that width of a rounding boundary.
- **The condition.** A row can pass only if it passes the same gate with H = [ℓ, u] and n = projection((ℓ + u)/2).
- **Why it holds whatever the proof.** The right-hand side depends only on the two readouts' exact values, the classification scale and the fixed projection. No tightening of the proof, no added precision and no other lane arithmetic can move it.
- **The SI case.** For an SI row the projection is RN64 of the midpoint, which minimizes max(n − ℓ, u − n) over all binary64 n. So the condition is exactly whether *some* binary64 value can pass.
- **The unit case.** For a millimetre or megapascal row the condition is weaker than "some value can pass". The fixed projection is not optimal for the SI distance, and choosing a different value would be a search (section 5, U1).

## 3. The ceiling census (design-time, on the frozen references)

### 3.1 Method

A standard-library Python replay was run in `/tmp` scratch. It is not committed, and it reads only `validation/references/o4_retained_source/{cases,oracle,source_twins_cases,source_twins_oracle}.json`. For every case and every row it forms [ℓ, u] from both readouts' 40-digit values in exact rationals. It projects the midpoint by DEF-E's H, including the unit conversions, and normalizes as `normalize` does. It then applies the gate's class rule and SharperExact, or the absolute bound RU64(2^-64·S), exactly.

- **Rows judged:**
  - free nodal translations (mm) and rotations (rad);
  - member end actions and the five station section actions (N, N·m);
  - stresses on both bases (MPa), with the admitted bending rows over the represented-Z interval;
  - support reaction components (N, N·m).
- **Rows not judged:** circular maxima (their own projection rule), displacement and support magnitudes, and the not-covered derived rows. Leaving rows out can only raise the pass counts.
- **The scales S are an upper estimate:**
  - the per-kind maximum over every row of the body and both readouts, a superset of the product's native and support rows;
  - coupled by the body's bounding-box diagonal, as `couple` and `body_extent` do;
  - for stresses, S_force/Â + S_moment/Ẑ with the prepared leaves;
  
  A larger S only enlarges the allowance. The p = 512 Φ floor is not modelled; it raises S only for near-zero bodies (o4-source-extension.md §4.2).
- **Robustness.** Re-running with every reference value perturbed independently by ±10^-37 relative, under three seeds, changed no row verdict.

So each count below is an upper bound on what the unchanged gate can publish on these documents, under any proof. Alpha and preparation failures, which also stop cases, are not counted here.

### 3.2 Results

Documents whose every judged row passes, under exact enclosures. Each cell gives all judged documents (of 172) and then the sensitive ones (of 54; tag `sensitive_design`: soft springs at k/a ≈ 1e-8 and 1e-12, stiff contrast and short members). The column says which readouts the gate is applied against.

| Represented Z | Unit rounding | Both readouts (claim as defined) | Admitted only | Annular only |
|---|---|---|---|---|
| hull(Ẑ, Î/c), as defined | as published, as defined | **13 / 0** | 30 / 1 | 97 / 32 |
| the single Ẑ | as published | 47 / 24 | 79 / 28 | 97 / 32 |
| hull, as defined | none (n = RN64 of the SI midpoint) | 24 / 2 | 69 / 8 | 172 / 54 |
| the single Ẑ | none | 79 / 45 | 172 / 54 | 172 / 54 |

- **The 13 judged documents that can pass as defined** are cantilevers, `FF-SYM`, `GUIDED` and two `PROP-PHI2-EXACT-A` Euler–Bernoulli documents. None is a sensitive design. They are the documents whose ordinary results are expected to pass their checks, so W1 does not attempt them on the plain binding.
- **Rows.** On the sensitive documents, 609 of 9,548 relative-class rows fail as defined (6.4 %). Per document, between 86.7 % and 100 % of rows pass (median 93.2 %). Stress rows dominate the failures: over all judged documents, 2,536 of the 3,196 failing relative rows are megapascal stresses.
- **Source-control twins** (104). As defined, 82 can pass. Every failure is a bending stress row (26 rows in 22 twins: the J2 `fields` twins and the RF PIN-OFF-345 r1e-12 allowance twins). The represented-Z hull and the unit rounding do not change any twin's verdict: with either removed, still 82 pass. Gating on one readout alone clears all 104.
- **Consistency with the product.** O4 records 6 judged-set and 112 twin successors through W1's own trigger path, which attaches a separate trigger body. Both are within these ceilings: 6 ≤ 2 × 13 runs and 112 ≤ 2 × 82. The replay predicts that the 6 are among the 13, and the census checks it (6.2). On the plain binding, no judged document published.

### 3.3 The causes

- **The represented-Z hull.** The admitted lane's bending Z is the hull of two binary64 representations of one quantity. W1 coverage repair §3.1 measures its relative width: a median of 871 × 2^-64 (about 0.43 × 2^-53) across 3,142 sections, and zero only where Î/c is representable. A row's half-width then joins the projection's own rounding of up to 2^-53·|n|, so the hull alone can fail a row. With the Ẑ point instead, 24 of 54 sensitive documents can pass, where none can as defined.
- **The section leaves.** Â, Î, Ĵ and Ẑ are correctly rounded exact annulus values, so each differs from its exact value by up to half a unit, and π makes equality impossible.
  - Stress rows divide by these leaves directly. So their two readouts differ by up to about 2^-53 relative even where the actions agree exactly. Every failing source-twin row is such a bending stress.
  - On the sensitive designs the solved rows differ by less: every sensitive document's largest readout gap on point rows lies in [2^-55, 2^-53) relative. That is half a unit or less, and the soft springs do not amplify it.
  - Removing the readout gap alone (the admitted-only column) clears 1 of 54 sensitive documents.
- **The published unit.** n = RN64(RN64(1000·m)/1000) for a millimetre row. This differs from m by up to about one unit of n, against an allowance of half a unit to one unit.
  - In an independent sampling of 200,000 random non-representable values, 8.4 % of millimetre rows and 2.3 % of megapascal rows fail on this alone at the body scale (|n| ≈ S).
  - The annular-only column shows its reach: 97 of 172 documents pass with the conversion, and all 172 without it.
- **The φ ≈ 2 documents.** In the eight `PROP-PHI2-*` Timoshenko documents the near-end bending rows carry (2 − φ)/(4 + φ), which is an exact zero, or nearly one, in one readout and not in the other. Their readout gaps are 5 × 10^10 units of 2^-53 or more. No option except R1 changes those rows.

Each cause is fixed by a definition: DEF-O and DEF-E `rows.component_stress`, the preparation law `RP-PREPARED-ANNULUS-v1` with REQ-001's "admitted binary64 section products", and DEF-E `projection.H` with the published units of the pressure-2 base contract. The causes overlap. Only the represented Z together with the unit rounding clears most sensitive documents.

## 4. The suggested remedies, examined

1. **A tighter or different proof.** It cannot raise any ceiling in section 3.2 (section 2).
2. **A better projection without a search.**
   - For SI rows, RN64 of the midpoint is already the minimax choice.
   - For unit rows, any other deterministic recipe edits DEF-E `projection.H`. The map y ↦ RN64(y/1000) can skip binary64 values, so even a perfect recipe cannot always reach the SI-nearest value. Its gain is bounded by the "none" row of the table, and it is a reviewed-definition change (section 5, U1).
3. **A representation that makes the readouts agree.**
   - With binary64 leaves this is impossible: the exact annulus values are irrational multiples of π.
   - The best binary64 leaf is the correctly rounded one, which preparation already uses.
   - Extended-precision leaves (a double-double A, for example) would no longer be "the admitted binary64 section products". The admitted readout would stop being the problem the ordinary route represents, which is a change to the verified claim (section 5, R1's family).
   - A narrower variant would prepare Ẑ as RN64(Î/c) instead of RN64(Z_exact). That can at most halve the represented-Z hull, and it changes the preparation law's meaning ("each the correctly rounded exact annulus value"). It belongs to Z1's family, and the census can measure it as a variant.
4. **Certifying against a readout hull or the exact problem.**
   - The gate already certifies against the hull of both readouts. That hull is what the claim requires.
   - Certifying against the exact problem alone (the annular readout) is R1, a claim change.
5. **Stating the corner as a limit.** No product change; a ScopeOfWork statement (section 6.1).

**Conclusion.** Within the unchanged claim, definitions, predicates and projection, no remedy can publish the sensitive documents this gap concerns. Every coverage-gaining remedy needs a ruling.

## 5. Options that need the owner's ruling (no recommendation)

The ceilings are section 3.2's, for the 54 sensitive judged documents, under exact enclosures. Alpha (24 runs) and preparation (16 runs) failures would still fail closed under every option. Implementing any option needs its own design, the independent numerical review, and an actual-source D3, because each moves reviewed inputs. Each also re-pins committed route-E successors and their DEF-E-bound hashes, and Z1 also route L's.

- **Z1. The admitted bending stress on the single Ẑ.**
  - *What changes:* DEF-O and DEF-E `rows.component_stress` ("admitted-K bending Z is hull(actual Z,I_K/c)"). The admitted readout's bending stress would mean M/Ẑ only.
  - *Ceiling:* alone, 0 → 24 of 54; with U1, 45 of 54.
  - *Consequence:* the verified claim no longer covers a bending stress formed from the stiffness second moment, M·c/Î. Such a value would still lie within 10^-9, but not within the sharper allowance.
  - *Notes:* the W1 coverage repair's escalation names this lever for route L too (§3.3). One ruling should cover both routes.
- **U1. How a unit row's SI value is judged.** Three forms, each with its own consequence:
  - (i) A projection that searches for the published unit value whose SI normalization passes. REQ-003 (e) fixes "`projection_retry: false` and `search_or_retry: none`" as reviewed inputs, so this changes an item REQ-003 (e) names.
  - (ii) Applying SharperExact to the raw unit value instead of its SI normalization. This weakens the SI-value guarantee from about 2^-53 to about 2^-52 relative. It is a predicate change, and REQ-003 (e) and AX-005 bear on it.
  - (iii) Publishing the retained rows in SI units. This changes the pressure-2 base contract's units for every consumer.
  - *Ceiling:* alone, 0 → 2 of 54, because the represented-Z hull still binds. With Z1, 45 of 54.
- **R1. The sharper allowance against one readout, with 10^-9 kept for both.**
  - *What changes:* REQ-001's guarantee and DEF-E `acceptance.final`.
  - *Ceiling:* 32 of 54 against the annular readout; 1 of 54 against the admitted one.
  - *Consequence:* a verification is removed from the gate, so REQ-003 (e) and AX-005 bear on it. Consumers relying on the sharper statement for the other readout lose it.
- **C1. Per-row coverage.** A relative-class row that fails the sharper allowance would publish as `not_covered` with its ordinary value, and the rest of the successor would publish.
  - *What changes:* REQ-001's classification law (today a row of a verifiable kind is never `not_covered` for this reason); the table's `accuracy_classification`; DEF-E `acceptance.final` ("every row passes"); and the reader's G3/G5c coverage checks.
  - *Ceiling:* on the sensitive documents, about 94 % of the relative rows would publish verified, the rest `not_covered`. Publication would then be limited by alpha and preparation.
  - *Consequence:* the reader cannot rederive why a verifiable row is not covered, and a successor no longer verifies all of its verifiable rows. No published bound is loosened, but the successor's acceptance condition is relaxed, so REQ-003 (e) and AX-005 bear on it.

**If the owner rules on none of these,** W1-E stays as built. It publishes where the corner does not bind and fails closed elsewhere, keeping the ordinary result with the one `RETAINED_PRECISION_UNAVAILABLE` notice (REQ-003 (b)).

## 6. The recommended path

### 6.1 Stating the corner as a limit (proposed ScopeOfWork text, not applied)

DEL-04-08 ScopeOfWork REQ-006 gains an item after (d):

> **(e) The sharper allowance against both readouts.** A retained-precision successor publishes only when each `relative_verified` row's value, normalized to SI, lies within the kernel's sharper allowance of both stiffness readouts' exact values. Three definitions of the method make that unattainable for many rows:
> - the admitted readout's bending stress ranges over hull(Ẑ, Î/c);
> - the two readouts' binary64 and exact section leaves differ by up to half a binary64 unit, and every stress row inherits that difference;
> - a millimetre or megapascal value is rounded in its unit and again when normalized to SI.
>
> Such a case keeps its ordinary result with the one `RETAINED_PRECISION_UNAVAILABLE` notice. On O4's judged set, no sensitive document can meet this condition under any proof (Design `w1e-sharper-exact-coverage.md`).

**AC-006 consequence.** "Each decline in REQ-006 is exercised by a maintained test that asserts its refusal." The implementation therefore adds one maintained test, which is test code and not this PR. Its input is a sensitive Timoshenko document of the judged set, for example `RFLIKE-SKEWT-PIN-OFF-345-r1e-08|zero|timo`, in both modes. The test asserts:
- the final gate's `Cause::Predicate` with `SharperExact`;
- the ordinary bytes plus the one notice, at the facade.

### 6.2 The product-side confirmation census (specified; executed later)

**Purpose.** Replace section 3's design-time replay with the product's own gate on its own enclosures, scales and projection, before any ruling on section 5.

- **Probe.** A dev-only cargo feature in the kernel, like RF's `exact-operation-count`, and not `#[cfg(test)]` (section 0, 5a). At `check_intervals`, for every judged row, it records:
  - the class, n, S, h and the allowance;
  - all four predicate verdicts.
  
  It also records the same gate re-evaluated under these counterfactual switches, on the lanes' actual enclosures and with no re-solve:
  - (a) the admitted Z set to Ẑ;
  - (b) and (c), the W1 coverage repair's same-section and same-readout lanes;
  - (d) the unit switch: n = RN64 of the SI midpoint;
  - (e) each lane alone.
  
  The probe changes no verdict and no published byte.
- **Inputs.**
  - The O4 judged set and the source-control twins, in both solver modes, on the plain binding and the augmented binding (`o4_m3_judging_tests.rs` `augmented`).
  - The committed route-E successors and RV120's inputs, as regression: every verdict there must be unchanged with the probe on.
- **Records.** For each attempted case:
  - its first failure: preparation, native, alpha or predicate, with the row;
  - for each failing row, its class under the switches, using W1 coverage repair §3.3's attribution classes extended by (d).
- **Report.** Section 3.2's table, from the product, in the implementation PR or as one revision of this section. The alpha and preparation counts are reported beside it.
- **Pre-registered reading.**
  - The product's counts can only be at or below section 3.2's, except where the p = 512 floor raises S.
  - Any count above the ceiling stops the census for diagnosis, because it would contradict section 2.
  - If no sensitive document can pass the unchanged gate, section 6.1's text stands, and section 5's options go to the owner with the product's table.
- **What it needs.**
  - A debug build of the `frame_kernel` and `product_physics` tests, with the feature, under the host's heavy-run slot and memory rules.
  - Private drivers only, so no registered identity is needed.
  - It runs after 5a merges, and with or after the W1 coverage repair's census (section 0).
  - A pilot of ten documents measures time and memory first.
  - It writes nothing under `validation/references` or the fixtures.

## 7. What stays unchanged

Every gate, predicate, tolerance, limit, precision ladder and projection; DEF-O, DEF-E and the tables; the verified claim; every committed byte. The recommended path adds a limit statement and, later, one test and a test-only probe.

## 8. Verification of this design

- The design rests on code facts (section 0) and on a replay of frozen, refuted references.
- The replay's arithmetic is exact and robust to perturbation (3.1), and its scale estimate is an upper bound.
- The product census (6.2) is the product-side check.
- This design goes through the dual review: an independent Claude reviewer and an independent Codex reviewer, on the same revision.

## 9. Reserved for the owner

- The REQ-006 (e) text (6.1). If it is not adopted, the corner stays an unstated fail-closed outcome.
- Any of Z1, U1, R1 or C1 (section 5), each with its consequence. No recommendation is made.
