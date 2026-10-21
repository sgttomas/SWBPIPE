# W1 member-load certification unit (group 3 stage 2)

**Status: as built.** The owner released stage 2 for implementation after RF (#1397) merged as `03f39bb49d9`; the implementation builds on that merged head. The kernel, the product and the independent consuming readers implement this design. Section 9 gives the actual-source D3 registration and section 13 the maintained verification. The design basis is main `4971d62dc9` (B7b, T4-U6/U8, #1392).

This governs the third post-T3 shared epoch in [b7-plan.md](b7-plan.md) ("After B7b", item 3). Section 8 states the RF interface and the registered baseline stage 2 builds on. This file replaces b7-plan's four 2026-10-12 "Member loads" paragraphs and the stage-2 part of its "Size, tests and review". Section 1 re-checks each of their premises, and section 14 gives the blast radius.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

**Commission.** b7-plan, "Staging" (T3, 2026-10-12, under the owner's "(b) Widen domain" decision): "Stage 2 is the follow-on unit **W1 member-load certification** … It needs its own go and independent numerical review." HELP_HUMAN accepted the staging on 2026-10-12. The owner's resumption steer of 2026-10-14 orders the shared epochs: "in this order from merged main …, each under its own design and independent review".

## Summary

- **What it does.** On W1 route L (the retained preview route: pressure-free, Euler–Bernoulli), a case whose loads include full-span uniform `distributed_force` member loads builds a retained source and can be certified. This retires stage 1's interim `MEMBER_LOAD_CAUSE`, under which such a case published `unavailable` (section 6.1). In the census trials F6/partial, T18 and T19, the other case gains its own retained status. It is `selected` when its certificate passes G0–G8, and otherwise `unavailable` with a named kernel or facade cause. Case 0 stays selected, as stage 1 established.
- **Settled ruling: option B** (HELP_HUMAN, 2026-10-17). The member load is a declared exact value at 1024 bits, formed once and carried in the exact ledger. R7's law does not change. The certificate certifies the declared-load problem. Its difference from the exact consistent load is at most 2^-1022 relative per declared entry, and that bound is disclosed (section 2).
- **What changed against 2026-10-12:**
  - f_w is no longer re-formed at p, q, 2p or q_W. It is declared once.
  - m_i and d_i^b take no counted load term.
  - S* takes no load term.
  - The station terms change sign to W1's j-side convention.
  - The lanes add exact points, not load enclosures.
  - A loaded member's published maximum is the kernel's RN64 projection of its certified enclosure, not PP's binary64 value, which fails the unchanged sharper predicate. Unloaded members keep PP's value and evidence byte for byte (sections 4.4 and 4.6).
  - The loaded-span search uses a quadratically convergent bound.
- **Memory.** Stage 2 does not fit by itself.
  - It falsifies a stage-1 pricing credit worth 76.3 MB in W4 (design evaluation), which is retired.
  - HELP_HUMAN adopted the full byte-preserving lever set. Real reductions come first (L4, L1a, L3, L5); then pricing-accuracy rules (L1b, L1c, L2), each on lifetimes that the code facts show.
  - With them, the actual-source registration leaves merged dense W4 at +113,188,985 B, against RF's registered +35,451,012 B (section 9).
  - Decision 22 is unchanged: M, the 0.9M margin, R, the 5 % TEXT reserve and the budget model stay as they are.
- **No change** to any reference, criterion, tolerance, D1 limit, work limit, frozen benchmark, DEF-E, route E, the model format or ordinary bytes.

## 1. Premise re-check (2026-10-12 design on main `4971d62dc9`)

Each premise is marked HOLDS, CHANGED or RETIRED, with its current evidence. Paths are relative to `projects/chirality-piping/`. The kernel is `core/solver/frame_kernel/src/structural/retained/` (written `retained/`). PP is `core/product_physics/src/`.

### The represented source equation

| # | 2026-10-12 premise | Verdict | Evidence on main | Consequence |
|---|---|---|---|---|
| P1 | `PrimitiveSource` gains `member_loads: Vec<UniformMemberLoad { member, intensity: [f64; 3], source_id }>` (global N/m), validated finite with the member present, sorted by member, source id and bits | CHANGED (representation); validation and order HOLD | `SourceParts` and `StraightMember` (`retained/source.rs` 81-93, 164-175, 270-287); load validation 596-604; nodal sort 605-611. Adapter reservations expose source and containing-slot sizes (section 6.2) | Member-only sidecars preserve existing source/capture layouts (section 2.1) |
| P2 | Encoded in the K4SRC identity after the nodal loads; K4STF unchanged | CHANGED | K4SRC is header, stiffness part, loads, stations, supports (source.rs 777-808); K4STF is header and stiffness part (812-816). An always-written block would move `kernel_source_sha256` in every W1 receipt | The member block is appended after the supports **only when non-empty**. Nodal-only K4SRC bytes do not move |
| P3 | K u = f_n + f_w, with f_w the consistent equivalent loads: node i (L/2)·w and (L/12)·(d × w), node j (L/2)·w and −(L/12)·(d × w) | HOLDS for the recipe; CHANGED for the equation (ruling B) | Signs verified exactly (section 2.3): the global form equals Tᵀ·f_eq(local), straight_pipe `spanned_uniform_equivalent_terms` (`core/solver/straight_pipe/src/lib.rs` 1484-1540) and FK `uniform_straight_balls` (`formation_check.rs` 2034-2098) | K u = f_n + f_d, f_d declared (section 2) |
| P4 | f_w formed at the working precision (d and L by the frame's own formation; each cross-product component one exact expansion; every product and quotient rounded once), never rounded to binary64 | RETIRED (ruling B) | d is rounded at p (assemble.rs 298-304); L is re-formed at q, 2p and q_W (adaptive.rs 1488-1524; verify.rs 464-488). R7 has no term for an inexact load (P11) | Declared once at 1024 bits; d is never formed (section 2.2) |
| P5 | `reduced_rhs` adds each formed contribution as one more summand beside the exact ledger | CHANGED | `reduced_rhs` (assemble.rs 871-901) adds the ledger's exact per-DOF net with `add_to` | Declared entries are folded into the ledger's nets. `reduced_rhs` is unchanged |

### Residual, verification and scales

| # | 2026-10-12 premise | Verdict | Evidence on main | Consequence |
|---|---|---|---|---|
| P6 | "The laws stay as they are": gate \|r_i\|(2^p − m_i) ≤ 64·m_i·d_i^b, stop rule and thresholds | HOLDS | Gate adaptive.rs 1633-1700; fallback d^b 1786-1904; thresholds 66-67, 382-472 | Unchanged (ruling B) |
| P7 | Residual at q re-forms f_w at q | RETIRED | | Never re-formed |
| P8 | m_i = 2·(K products + formed load terms) + 2 | RETIRED | m counts only the nonzero K^q·u products (adaptive.rs 1677). The ledger summand is exact and not counted | Declared entries are ledger data; m_i is unchanged |
| P9 | d_i^b adds an upward \|f_w,i\| formed by the same recipe | RETIRED | d and d^b take \|net\| of the exact ledger (adaptive.rs 1829-1834) | d = \|nodal + declared\| exactly, as for the ledger |
| P10 | The 2p verification re-forms f_w at 2p | RETIRED, and the fact behind it CHANGED | R7's r and r₂ are formed at q_W = min(3p + 64, 1024) (verify.rs 383-385, 829-849, 886-917). t₁ = 2^(7−q_W)·B_b·N_u covers only K's formation error (1105-1122) | The declared ledger enters r and r₂ via `add_to`; t₁ gains no term |
| P11 | The gate's counted term and d_i^b bound a formed term's formation error | RETIRED | True for the gate (each component ≤ 4.5u relative against ≥ about 256u of allowance, with the moment bound taken over absolute operands). But R7's verification assumes an exact load (t₁, E, Ŵ, 7a) | This gap led to the ruling (section 2) |
| P12 | S* for a body's force and moment kinds includes \|w\|·L and \|w\|·L²/8, formed upward at 2p | RETIRED | S* is formed in `scales_at` (adaptive.rs 2147-2190) and `publication_with` (2971-3020) as the maximum of published rows | Redundant (section 3) |
| P13 | Work is charged to `rhs`, `residual_formation` and `wide_formation` within unchanged limits; excess refuses by budget | CHANGED (stages); limits HOLD | `residual_formation` and `wide_formation` are per stiffness group, shared by cases with one K4STF (adaptive.rs 1488-1524; verify.rs 464-488). Limits are case 20e9 and invocation 60e9 (`retained_wire.rs` 65-68) | The declared formation is per case, in `rhs` (section 5) |

### Recovery and the product certificate

| # | 2026-10-12 premise | Verdict | Evidence on main | Consequence |
|---|---|---|---|---|
| P14 | `recover` subtracts each member's own equivalent loads, in local axes, from its end actions | HOLDS | `recover` (recover.rs 239-489) | f_eq is formed locally from the member axes and w (section 4.1) |
| P15 | Station terms: forces w_local·tL, M_y + w_z(tL)²/2, M_z − w_y(tL)²/2 | CHANGED: wrong signs | These are the i-side signs of straight_pipe's helper (`station_resultants_from_i_end_with_spans` 1140-1185), which PP negates (`straight_section_resultants`, PP `lib.rs` 10660-10676). W1 stations are j-side ("j-end action of the sub-member i→x", recover.rs 12-15), as are K-D5's sections (`certified_recovery.rs` 874-952) | Corrected j-side forms (section 4.1) |
| P16 | Both lanes include f_w in `source_residual.rs`'s free-row and reaction sums as a directed-1024 enclosure from the exact w and the enclosed L | CHANGED | Free rows 627-685 (individual nodal points 662-673); reactions 720-750 and 1509-1520 | Each declared entry is added as an exact `Enclosure::point` |
| P17 | `final_case::action` adds the distributed terms to station enclosures | CHANGED | `action` (final_case.rs 715-728) only reads the lane's row enclosures | Station terms belong in `source_residual::recover` (1384-1520); `action` is unchanged |
| P18 | `ProductRecipe::CircularMaximum` keeps its two-end recipe for an unloaded member | HOLDS | final_case.rs 783-800 | Unchanged. Unloaded members keep PP's published value and evidence (section 4.6) |
| P19 | A loaded member's maximum is enclosed by a fixed-capacity subdivision; lower endpoint at the published governing station; the direct certificate applies its unchanged predicates | CHANGED | A box-corner bound overestimates by O(h) and exhausts 256 nodes on skew and axial-slope controls (section 4.4). Using PP's station leaves the enclosure 212 to 10,686 × 2^-53 wide | Branch-and-bound with a quadratically convergent bound; lower endpoint at the search's best point |
| P20 | PP's `prepared_maxima` passes the member's local spanned loads to `exact_straight_summary_extrema` in place of `&[]`; the published value is PP's | HOLDS for the call; CHANGED for a loaded member's value | Call at retained_product.rs 5637 (U6 added `pressure_state` to the signature, PP `lib.rs` 10809-10815). PP's value fails the unchanged sharper predicate on almost every loaded control (section 4.4). The engine is shared with ordinary callers (PP `lib.rs` 5764, 6035; `membrane_publication_range.rs` 172/248), so only the argument changes | A loaded member's value is the kernel's projection. Its evidence is PP's bounder evidence with a symmetric interval about that value (section 4.4) |
| P21 | A combination with a member-load operand is unavailable by name until DEF-C's combined-ledger law is extended | HOLDS | `combine.rs::validate_preps` 233-252; `RetainedLedger::combined` (ledger.rs 209-229) reads only nodal loads | Refused in the kernel, with a backstop in the ledger (section 4.5) |

### Producer, definitions and readers

| # | 2026-10-12 premise | Verdict | Evidence on main | Consequence |
|---|---|---|---|---|
| P22 | `capture_case_source` no longer fails "unsupported producer present" for `element_uniform_loads` | CHANGED | Route L already diverts to `capture_member_load_case` (retained_product.rs 1785-1834); the flag at 1790 now matters only on route E | The flag becomes "route E with element loads, or imposed displacements" (section 6.2) |
| P23 | A new `capture_case_member_terms` captures each normalized element occurrence, as `check_captured_nodal_leaves` does for nodal terms, into `SourceParts.member_loads` and `TermIdentity` | CHANGED (two checks) | The early raw binding (4249-4278) is skipped for an element-load case (4124-4125). Occurrence binding is at 2113-2225. `TermIdentity` has a `dof` and is priced | Generalized early check; one merged occurrence capture with one "used" tracker; a separate `MemberTermIdentity` (section 6.2) |
| P24 | `check_early_occurrences` covers both families | HOLDS | 4291-4311 | Real per-term checks replace the stage-1 branch |
| P25 | DEF-O changes `scope.loads` and `rows.maximum` ("empty loads/no pressure") | HOLDS, with additions | DEF-O `scope.loads = "individual_normalized_nodal_terms"`; `rows.maximum` names the binary64 bounder | New `lanes.loads` and the disclosure; `rows.maximum` keeps its unloaded case and gains the loaded case (sections 4.4 and 6.4) |
| P26 | DEF-C `scope.excludes` gains `operand_member_loads`; DEF-E unchanged | HOLDS | DEF-C excludes list; D1.7 refuses element loads on route E (`retained_memory.rs` 1006-1008) | Also the SCHEMA `CombinationReason` tag |
| P27 | The receipt's `sources[]` gains `member_load_terms` (constructor ordinal, member, primitive load index, source id, intensity bits) beside `nodal_terms`; SCHEMA gains the closed definition | HOLDS, made optional | `nodal_terms` at retained_wire.rs 1051-1057; SCHEMA `CaseSource` (line 3961), nodal terms (4165) | Present only when non-empty |
| P28 | The Rust reader's G8 rederives each member term and refuses a missing, extra, reordered or changed term; G1's source hash covers it; the Python verifier mirrors both | HOLDS | G8 nodal rederivation `core/reporting/result_export/src/retained_precision.rs` 5139-5183; K4SRC rebuild 5542-5640; G1 837-845 covers the whole source object; verifier `tests/verifiers/retained_precision.py` 2117-2126, 366-410 | Section 6.5 |
| P29 | DEF-O, DEF-C and SCHEMA pins and the registered profile move only in this unit's own actual-source D3 | HOLDS, wider | `REVIEWED_INPUTS` (`build_identity.rs` 158-181) | Moves #2 SCHEMA, #3 DEF-O, #4 the preview-retained table (DEF hashes and the limitation), #6 DEF-C |
| P30 | Timoshenko and dual-basis declines remain until O4 | HOLDS | Route L cannot carry a shear model or an allowance (`SHEAR_MODEL_WITHOUT_PRESSURE_CONTRACT`, PP `lib.rs` 268-272; `CORROSION_ALLOWANCE_WITHOUT_PRESSURE_CONTRACT`, `pressure_runtime.rs` 284-286). W1-E declines `ShearModel`/`SectionAllowance` (`retained_memory.rs` 840-853, 899-910) | Section 7. Nonzero mill tolerance is admitted on route L as one effective wall (PP `lib.rs` 10538-10553); pressure-free documents have one basis (H-1) |

### Staging, fallback, memory and size

| # | 2026-10-12 premise | Verdict | Evidence on main | Consequence |
|---|---|---|---|---|
| P31 | Stage 1: a member-load case ends at preparation with `MEMBER_LOAD_CAUSE` | RETIRED by this unit | Constant retained_product.rs 43; uses at 170-173, 1794-1834, 2253-2255, 4122-4125, 4293-4300, 4540-4545, 5157; test `w1_domain_stage1_tests.rs` 143 | Removed (section 6.1) |
| P32 | The census is met by case 0 | HOLDS | `w1_source_census_tests.rs` 383-475, 519-538, 547-588 | A pinned table of the other case's status and cause is added (section 10) |
| P33 | The Preparation fallback control: this unit adds a real Preparation input once its member-load cause is settled | CHANGED | `retained_facade_tests.rs` 953-985 covers Candidate and Native. Today the milestone plus a UDL and the two single-case UDL controls (T1/F1a-UDL-W1e8, F1a/UDL-W1e8-soft-branch) reach Preparation only through the interim cause | Section 6.7 |
| P34 | Memory: about 1.05 GB of margin; a 16 MiB loaded-span workspace; 100 MB allowance | RETIRED | The U6/U8 baseline left +51.8 MB; current W1 headroom is in section 9 | Section 9 |
| P35 | Stage-1 pricing premises stay valid | CHANGED | `tools/retained_pricing/base/b7a_phase_model.py`: route L's `formed_cases = max(actual_cases − 1, 0)` and `success_findings = "max(lc-1,0)"`. Their own INVALIDATION text names stage 2 | Removed (section 9) |
| P36 | About 2,500 product and tool lines (kernel about 1,300) and 1,500 test lines | RETIRED | The unit is built | No size estimate is carried |
| P37 | Corpus 45 bases, 593 mutations, 91 must-pass | CHANGED | The expanded corpus file had 45 bases, 599 mutations and 90 must-pass | The corpus has 61 bases, 640 mutations and 108 explicit positives (section 13) |

## 2. The declared member load (ruling B)

**Ruling** (HELP_HUMAN, 2026-10-17, settled): "Option B. The member load is declared as a fixed, fully specified high-precision exact value (about 1024 bits), formed once and carried like the exact nodal ledger. R7's law does not change. The certificate certifies the declared-load problem, and its maximum relative difference from the exact consistent load (about 2^-1019) is disclosed in the DEF-O/DEF-C wording and the limitation text."

Its conditions:
- Rust and Python can each recompute the declared load bit-exactly.
- The design confirms that the 7a data flag, Ŵ and E treat it exactly like the ledger; if not, it returns to HELP_HUMAN.
- The declared formation is priced in memory and in work (RF's metered rule) and included in the D3 feasibility.
- The independent numerical review checks all of this.

On 2026-10-17 HELP_HUMAN replaced "about 2^-1019" with the proven bound of section 2.4 ("State the proven bound, 2^-1022 relative per declared entry. That supersedes my 'about 2^-1019'").

**Basis.** The gate can absorb an inexact formed term (P11), but R7's verification assumes an exact load:
- t₁ covers only K's formation error;
- E's absolute operands, Ŵ's empty-ledger recovery and the 7a data flag all read the ledger.

A declared exact value meets that assumption without changing R7's law.

### 2.1 The source

- **`UniformMemberLoad`** is `{ member: u32, intensity: [f64; 3], source_id: String }`, with global intensity in N/m and one nonzero component (the D1.7 direction). It has no extent field: full span is a type-level fact. A partial span therefore cannot reach the kernel, including after O4, where the Euler–Bernoulli and Timoshenko vectors differ for a partial span (U6/U8 §3.4).
- **Validation.** Finite intensity, the member present, and a length check that is already exact (source.rs refuses a zero-length member).
- **Canonical order.** By member, source-id bytes, the three intensity bit patterns, then constructor ordinal, the same discipline as the nodal loads.
- **K4SRC.** After the supports, a member block (count, then each term's member, intensity bits and source id) is appended only when the list is non-empty. K4STF is unchanged. A nodal-only source keeps its K4SRC bytes and `kernel_source_sha256`.
- **Representation and API.** Keep `SourceParts` and `PrimitiveSource` unchanged in layout and nodal construction. A new validated `MemberLoadSet` owns the member vector, and `MemberSourceRef` borrows the primitive source plus that sidecar. Its constructor validates the member association against that source; its canonical identity is the base K4SRC followed by the nonempty member block. The composite's `source_counts` adds the member stride. Existing nodal APIs and all 29 `SourceParts` literals stay unchanged. A member-aware recorded-invocation entry accepts aligned source/optional-sidecar views, preserving shared stiffness groups and case indices; a missing or wrong-case sidecar is an association refusal. A case without member terms uses the original source path. Here, “source has member loads” refers to the validated composite view, not a new `PrimitiveSource` field.

### 2.2 The declared value

Each declared entry is a `Wide<16>` (P = 1024 bits, the lanes' `Endpoint` type). All arithmetic uses one `WideContext<16>` and an `ExactWideSum`, whose primitives already exist:
- `ExactWideSum::add_product` (wide_sum.rs 586) forms exact products;
- `ExactWideSum::round` (756) rounds an exact sum once, to nearest with ties to even;
- `WideContext::mul`, `div` and `sqrt` (wide/multi.rs 1172/1177/1182) each return the exact result rounded once, and `div` and `sqrt` are bit-serial with a sticky bit;
- `mul_pow2` is exact.

RN means "round once to 1024 bits, to nearest, ties to even". Coordinates x and intensities w are binary64 values lifted exactly.

Per loaded member (nodes i and j):
- **D1.** S = RN(Σ_a [x_j,a·x_j,a − (2x_i,a)·x_j,a + x_i,a·x_i,a]): one exact sum of nine binary64 products, rounded once. The chord d is never formed or rounded.
- **D2.** L̂ = RN(√S).
- **D3.** g = RN(L̂/12).

Per load on that member:
- **D4.** F_k = RN(L̂·w_k)·2⁻¹ for k = 0, 1, 2. If w_k = ±0, then F_k = +0 and no operation runs.
- **D5.** For each axis a, with (b, c) = (1, 2), (2, 0), (0, 1): M_a = RN(T), where T is the exact sum of the four terms g·x_j,b·w_c − g·x_i,b·w_c − g·x_j,c·w_b + g·x_i,c·w_b. Each x·w is an exact two-product, at most 106 bits. So M_a = RN(g·(d × w)_a) with d × w exact.

Node i receives (F, +M) and node j receives (F, −M). Exact zeros are stored as +0.

**Why d is not formed.** D1 does not bound coordinate magnitudes. Retained D1.2–D1.11 cap counts and text, and source.rs requires only finite coordinates. So x_j − x_i can need 2,097 bits, for example x_i = 2⁻¹⁰⁷⁴ and x_j = 2¹⁰²³. In the 469 design draws of section 2.4, 53 had a component that is not exact at 1024 bits. The recipe is therefore deliberately not the frame's own order (round d, then the dot product, then the square root): both p-bit and 1024-bit rounding of d change L̂ on named controls.

**Range.** For finite inputs and a nonzero chord:
- L̂ ∈ [2⁻¹⁰⁷⁵, 2¹⁰²⁶);
- forces' leading bits lie in [2⁻²¹⁵⁰, 2²⁰⁴⁹] and moments' in [2⁻³²²⁸, 2³⁰⁷³];
- every sum in D1–D5 spans at most about 5,220 bits;
- a per-DOF net (declared plus binary64 nodal terms) spans at most 7,325 bits, below `SPAN_LIMIT_BITS` = 8,128 (wide_sum.rs 48).

Formation and netting therefore never refuse. A later row sum that adds such a net to K·u products can still refuse with `Span` on adversarial magnitudes. That is the existing terminal, named refusal, never a truncation.

**Ties.** A square root never ties, but RN(L̂·w) can (the control w = 0.375 hits one). Ties to even are part of the specification and of the Python oracle.

**Granularity: per load.** L̂ and g are formed once per member, and each load keeps its own six entries (twelve ledger terms, zeros included, with each term's nonzero flag set only where the value ≠ 0). This keeps the ledger's granularity of one term per authored contribution (ledger.rs 3-5), so 7a's "a nonzero ledger term, even when the net is 0" applies literally. A cancelling pair of loads on one member still counts as data, as a cancelling nodal pair does. Summing per member first would break that equivalence.

### 2.3 Sign conventions (verified exactly)

An exact check builds the retained element as assemble.rs does: B_local, D, K = BᵀDB, and recover.rs's end actions, asserted equal to K_local·d. It solves exactly with fractions. For a skew member with d = (3, 4, 12)/2 and y_ref = (4, −3, 0), the global node recipe equals Tᵀ·f_eq(local) exactly, and it matches the ordinary vectors. The solutions match the closed forms:
- cantilever: tip deflection wL⁴/8EI;
- simply supported span: θ_i = wL³/24EI, midspan moment wL²/8;
- fixed-fixed member: end moments wL²/12, midspan wL²/24;
- the same cases on skew members.

### 2.4 The disclosed bound

With u = 2⁻¹⁰²⁴, each rounding gives x(1 + δ) with |δ| ≤ u:
- S = s(1 + δ₁), so L̂ = L(1 + θ) with |θ| ≤ 1.5u + u².
- Force: F = (Lw/2)(1 + θ)(1 + δ), so |F − exact| < 2.5u(1 + u)·|exact|.
- Moment: M = (L(d × w)/12)(1 + θ)(1 + δ_g)(1 + δ_M), so |M − exact| < 3.5u(1 + 2u)·|exact|.

Both are below 4u = **2⁻¹⁰²²**, relative to the entry's own exact value. Because d × w is exact, a cancelling cross product carries no penalty, and the bound also holds against the absolute-operand form (L/12)(|d_b||w_c| + |d_c||w_b|). Exact zeros are exact. At a DOF with several entries, the net differs from the exact consistent net by at most 2⁻¹⁰²²·Σ|exact entries|; nodal terms are exact.

An exact design check over 469 cases (9 named controls and 460 random draws with exponents up to ±1000, each decided exactly by squaring) observed worst errors of 1.630u for forces and 2.017u for moments.

**The certified problem** is K u = f_n + f_d, with f_d the declared entries. The difference between f_d and the exact consistent load is disclosed, not certified, and enters no published bound (section 6.4).

### 2.5 The ledger, and "exactly like the ledger"

`ExactAccumulator` takes only binary64 values and binary64 products (exact_sum.rs 3-21), so it cannot hold declared entries. For a source with member loads, each loaded DOF's net is formed in an `ExactWideSum`: the nodal net through `add_integer`, then each declared entry through `add_wide`. One new accessor returns the result in `LedgerNet`'s canonical odd form, because `ExactWideSum::net` is private (wide_sum.rs 732). Each entry sets `nonzero[k] |= entry ≠ 0`. A nodal-only source keeps today's code path and today's K4LED bytes.

**HELP_HUMAN's condition.** Exact ledger inclusion establishes 7a's per-term data flags and Ŵ's omission on two requirements:
- the entries are folded into `RetainedLedger`'s per-DOF nets and per-term flags;
- `recover` reads every member-load term only through its `ledger` argument, never from `source`.

Then `RetainedLedger::empty()` (Ŵ) and `ledger = None` (‖ā_q S‖₁) omit every member-load term by construction. **E is not established by inclusion alone.** A near-parallel frame is a counterexample to unamplified load terms in E. HELP_HUMAN ruled the conditioning repair within the settled declared-load semantics (2026-10-17), and section 4.2 derives the full corrected recovery-DAG bound. That establishes E's warrant separately from ledger inclusion; the implementation must preserve that DAG.

| Consumer | Today | With declared entries | Verdict |
|---|---|---|---|
| rhs at p | `reduced_rhs` (assemble.rs 875-901): `add_to` (891), then −K_ic·u_c, rounded once | inside the net | same |
| residual at q, m_i, d | `residual_rows` (adaptive.rs 1633-1700): r via `add_to`; d via `add_integer(\|net\|)`; m counts only nonzero K^q·u products (1677) | inside the net; m_i unchanged; d = \|nodal + declared\| | same |
| d^b fallback | `bounded_fallback` (adaptive.rs 1786-1904; ledger 1829-1834) | same | same |
| R7 r, r₂ at q_W | verify.rs 833, 890 (`add_to`) | same; never re-formed | same |
| t₁ | verify.rs 1105-1122 (K's formation error only) | the declared problem's load is exact | no new term |
| E | `formation_scale` (verify.rs 86-288): ledger reactions use \|f_c\| (233) under `Some(ledger)`; ‖ā_q S‖₁ passes `None` (959) | reactions automatic; local recovery needs conditioned absolute-operand bounds and g-check applicability (section 4.2) | established for the specified DAG by section 4.2's error derivation |
| Ŵ | verify.rs 851-883, `RetainedLedger::empty()` at 864 | the empty ledger omits every member-load term | same, by construction |
| 7a data flag | `data_blocks` (bound.rs 127-157); `fill_data_blocks` (162-197); `has_nonzero_term` (ledger.rs 141-165) | a nonzero declared entry sets the flag even if the net cancels | same |
| uniqueness warrant | source_residual.rs 970-982; bridge.rs 486-492 | data from `fill_data_blocks` | same |
| lane free rows | individual nodal points (source_residual.rs 662-673); combination nets (692-716) | each declared entry added as `Enclosure::point` (exact, being a `Wide<16>`) | same |
| lane reactions | source_residual.rs 1509-1520 | same pattern | same |
| binding | `ledger_encoding` (adaptive.rs 4908); `encoding_matches` (5409-5417); receipt `cases[].selection.ledger_sha256` (retained_wire.rs 1271) | K4LED binds the declared problem | same |

**Formed once.** The case's formed ledger is stored once in `CasePrep`, behind an accessor that errors when the source has member loads and the ledger is not yet formed. No consumer can then read a nodal-only ledger by mistake. It is formed in the case's first own-stage run (section 5).

### 2.6 Bit-exact recomputation in Rust and Python

- **Python.** `core/solver/frame_kernel/tests/retained_k4/gen_k4_vectors.py` is already the K4 bit oracle: `rp` (546) rounds to nearest-even on Fractions, `sqrt_p` (566) is `isqrt` with a sticky bit, and `ledger_bytes` (1674) encodes K4LED. It gains `declared(...)` implementing D1–D5 independently, and a `declared.txt` vector file. Per control and load, that file holds the six entries as (sign, exponent, 1024-bit significand) tokens, plus the case's K4LED bytes and SHA-256. `--check` regenerates it byte for byte.
- **Rust.**
  - `declared_loads_match_python_bit_for_bit` compares `Wide<16>::parts()` of each entry.
  - `declared_ledger_k4led_matches_python` compares K4LED bytes.
  - A third test compares the formation's per-kind operation counts with Python's operation list.
- **Controls:**
  - axis-aligned members;
  - skew members: d = (3, 4, 12)/2, and one with irrational L;
  - a large offset (coordinates about 1e15);
  - d ∥ w (moments exactly 0), and near-parallel d and w (heavy cancellation);
  - an exponent spread (1e±300);
  - a tiny chord, and huge values (moments about 2³⁰³⁹);
  - a tie (w = 0.375);
  - the rounding-order control (RN(g·c) ≠ RN(RN(L̂·c)/12)) and the two d-rounding controls;
  - a nodal load on a declared DOF;
  - a cancelling pair of loads on one member.
- **Independent reader gates (HELP_HUMAN, 2026-10-17).** For each selected route-L case with member terms, the Rust reader and Python verifier independently recompute D1–D5, combine the declared entries with that case's normalized nodal terms exactly, and bind the result to `cases[].selection.ledger_sha256`. No receipt digest is added: K4LED's existing hash binds the declared problem, while K4SRC binds its source. Existing attempt/selection ledger-hash equality checks remain, so a selected attempt cannot attest a different declared problem. Nodal-only cases retain today's attestation path and verdict behavior.
  - **Rust, including wasm.** A reader-owned bounded signed-integer/dyadic implementation performs RN1024 with guard, round and sticky information, exact quotient/remainder comparisons for division, and integer-square-root midpoint comparisons for square root. It implements D1–D5 directly, without depending on the producer, the retained kernel's arithmetic or the Python oracle. It needs no new Cargo dependency. Each arithmetic intermediate has an explicit limb/span bound; an exceeded bound or arithmetic inconsistency refuses validation, never truncates. The implementation must show the D1 domain fits those bounds.
  - **Python.** The verifier implements its own integer/Fraction RN-even, square root and canonical ledger encoding. It neither calls Rust nor imports or executes `gen_k4_vectors.py`; the generator remains a third, test-only oracle. All three agree on the named controls above, including ties, exponent spread, cancellations, zero signs and rounding order.
  - **K4LED.** Rebuild the complete ascending DOF list from nodal and member occurrences, including DOFs whose authored contributions are zero or cancel. Normalize each net to its odd magnitude, sign and exponent, with the existing canonical zero `(false, 0, empty)`; stream the existing `K4LED\x01` encoding into SHA-256. Nonzero-per-term flags are recomputed separately for the data checks; they are not K4LED fields. Both validators reject a selected member case whose recomputed digest differs, through G8 `PREPARATION_MISMATCH`, after the existing source/occurrence checks. Earlier gate precedence is unchanged.
  - **Mutants.** Change every attested ledger hash together and refresh the outer receipt hash, so existing equality and G1 checks do not mask the new gate; both validators must reject. Also pin wrong RN ties, square-root rounding, D5 order/sign, omitted entries, cancelled nets and zero-occurrence DOFs against the bit vectors. Genuine loaded bases pass in native Rust, wasm and Python; existing nodal-only corpus verdicts stay unchanged.
  - **Invocation-unbound result.** G8, including this recomputation, runs only with a captured invocation, as today (Rust retained_precision.rs 6204-6212; Python verifier 2980-2983). Without one, preserve `needs_recompute` standing; do not describe the result as having passed invocation-bound ledger validation.
  - **Memory.** Per-case arithmetic, declared entries, net storage, canonical hashing and all call-stack/capacity staging have a design cap of 1 MiB, with a separate full moving allowance. Section 9 charges both positively at the existing three-case convention. This design bound requires layout/source pins and a cap-maximal native/wasm check; section 9 states the source-derived reader inventory beneath it.
  - **Early guard scope.** Before hashing or staging an invocation, source-key presence (`member_load_terms`, even malformed) activates the member bound regardless of namespace or route. With that key omitted, the guard activates only for preview route L's current pressure-free namespace and an authored element `distributed_force` with dimension `force_per_length`. Direction, extent, unit and ID validity are deliberately not prerequisites for this bound. Thus a dropped-key forgery of a newly admitted member invocation remains bounded, while source-key-absent legacy pressure, exact-route and nodal inputs retain their established hash/namespace precedence. The applicability check borrows existing values; Python mirrors the namespace checks without its general helper's section-list allocation.

## 3. Residual, verification and scales

R7's law, the gate, the stop rule, the classification thresholds and the case and invocation limits are unchanged. The 2026-10-12 counted-term recipe (P8, P9, P11) is replaced by section 2.5: the declared entries are exact ledger data.

**S* takes no load term (P12).** The member's exact free-body identity, S_F(0) − S_F(1) = w_loc·L, makes its two published end-force rows give S*_fo ≥ ‖w_loc‖∞·L/2. The coupling `coupled_scales` (mo ≥ L_b·fo with L_b ≥ L) then gives S*_mo ≥ ‖w_loc‖∞·L²/2. That exceeds the proposed ‖w‖L²/8, and the force term would raise S*_fo by at most a factor of 2√3. S* carries no soundness (R7 does). Dropping the term leaves both S* computations and the readers' S* rederivation unchanged.

## 4. Recovery, stations and the product certificate

### 4.1 Recovery at p

**Definition.** The certified declared-load problem is:
- u and the reactions solve K u = f_n + f_d;
- each loaded member's end actions are K_e·u_e − f_eq,local(T·w, L), and its stations add the exact span terms.

f_eq is formed locally from the member's axes and w, not by rotating the declared global value. The stations need w_local and L anyway. All inputs at p are p-bit or binary64. Rotating the 1024-bit declared value would put spurious terms of about 2⁻¹⁰²² into the torsion rows. Before numerical stiffness/recovery error, the discrepancy between declared and consistent nodal loading is bounded by section 2.4's sum of per-entry absolute errors; it is not a relative bound on a cancelled net. Published finite-p end actions also have recovery error, bounded separately by R7 and the lanes. No 2⁻¹⁰²² total equilibrium-error claim is made for those published actions.

`MemberOperators` gains `length`, which `form_member` already computes (assemble.rs 305) and discards. This is a `Wide<L>` field, 144 B at 1024 bits. Actual-source D3 must use each in-build operator stride, including this field, rather than an estimate.

Per loaded member, with the existing local quantities Q and `inv` = 1/L:
- w_loc[r] = RN_p(Σ_ℓ Σ_c ax[r][c]·w_ℓ,c)
- wl = RN_p(w_loc·L), wll = RN_p(wl·L), h = RN_p(wll/12)

The end actions are each one exact sum, rounded once:

| Row | i end | Row | j end |
|---|---|---|---|
| A0 | −Q0 − wl0/2 | A6 | Q0 − wl0/2 |
| A1 | inv·Q2 + inv·Q3 − wl1/2 | A7 | −inv·Q2 − inv·Q3 − wl1/2 |
| A2 | −inv·Q4 − inv·Q5 − wl2/2 | A8 | inv·Q4 + inv·Q5 − wl2/2 |
| A3 | −Q1 | A9 | Q1 |
| A4 | Q4 + h2 | A10 | Q5 − h2 |
| A5 | Q2 − h1 | A11 | Q3 + h1 |

**j-side station terms (corrected; P15).** At fraction t:
- force_c = (BᵀQ)_j,c + wl_c/2 − wl_c·t
- torsion = Q1
- M_y = t·Q5 + t·Q4 − Q4 − h2 + (wll2/2)·t − (wll2/2)·t²
- M_z = t·Q3 + t·Q2 − Q2 + h1 − (wll1/2)·t + (wll1/2)·t²

t² is exact (at most 106 bits). These equal A_j + w_loc·L(1 − t), then t·A10 + (t − 1)·A4 + w_z L² t(1 − t)/2, and t·A11 + (t − 1)·A5 − w_y L² t(1 − t)/2. On K-D5's base they are −F_i′ − w_local·tL, then M_y = −M_y,i′ − tL·V_z,i′ − w_z(tL)²/2 and M_z = −M_z,i′ + tL·V_y,i′ + w_y(tL)²/2, which are K-D5's own code signs.

The force base matters: today's station forces come from the j-end action (recover.rs 392), which equals −F_i only for an unloaded member. For a loaded interior station, expand the raw `(BᵀQ)_j` products into the station's single exact sum; do not first round a recovered j-end force and then add load terms. At t = 0 and t = 1, return the sign-adjusted corrected end action directly, with canonical +0, so station(0) = −A_i and station(1) = A_j are bit-identical even at rounding boundaries. The interior identities above are algebraic identities; separately rounded evaluations need not be bit-identical. Unloaded members keep today's path and bits. Reactions already subtract the ledger net, which now includes declared entries.

### 4.2 E for a loaded member

**Conditioning repair (HELP_HUMAN, 2026-10-17).** E applies under `Some(ledger)` only when that ledger carries the member contributions, mirroring recovery with absolute operands. B̄ puts 1 at every axis component, so one w̄ serves all three local axes. The member's existing g_m = 2^g_exp amplifies **every new frame-dependent load-recovery operand**, just as it amplifies the existing displacement-dependent member recovery:
- w̄ = stage(Σ|w_ℓ,c|), w̄l = stage(w̄·L), w̄ll = stage(w̄l·L), h̄ = RN(w̄ll/12).
- End rows: stage((raw + w̄l/2)·2^g) on the six force rows and stage((raw + h̄)·2^g) on the four bending rows; torsion rows are unchanged.
- Station forces: stage((raw_j + w̄l/2 + w̄l·|t|)·2^g).
- Station moments: stage((old + h̄ + (w̄ll/2)|t| + (w̄ll/2)t²)·2^g).

Here raw, raw_j and old denote the corresponding unamplified absolute recovery chain. Use the existing `StageRounding` discipline at each sum/product/division stage; exact power-of-two amplification is applied before combining with the returned E bound. Endpoint branches use the corresponding end bound. No new load term bypasses g_m. The source/ledger boundary prevents a source-side load from leaking into an omitted-ledger recovery. Unloaded members, all-zero member contributions, `None`, empty-ledger and nodal-only inputs retain the existing paths and bits.

**Existing g-check, extended applicability only.** Preserve prior nodal-only applicability and the threshold g_exp ≤ P − 16. Additionally check any member whose ledger carries an authored member term with any exact binary64 intensity component ≠ 0. Determine this before local projection or netting: cancelling terms, an exactly vanishing local component, and fully fixed zero-prescription members cannot hide activity. All-zero contributions add no activity. Under `None` or an empty ledger no new member-load activity exists. Count every executed arithmetic operation and refusal's arithmetic under the existing work algebra. Native W1 LME counts the eight `WidthWork` operation tariffs plus `SumWork`; the Boolean ID/nonzero applicability scans, like the prior g-check's comparisons, are outside that algebra. No new counter, tariff, policy, precision/domain/work limit or acceptance predicate is introduced.

**Required error warrant.** The separate numerical review must trace the full corrected DAG: coordinate differences and length, Gram–Schmidt projection/subtraction/normalization, cross-product/normalization, w_loc's exact accumulation, wl/wll/h, corrected end sums, interior single-round station sums, endpoint dispatch and E's own rounding. It must derive |q_P − R*(u_P)| ≤ 69·2^-P·E_q under the existing g-check, including higher-order terms; magnitude domination E_q ≥ |q_P| is insufficient. The declared global ledger is exact data; this warrant concerns local frame-dependent affine recovery, not a new f_d formation-error term in R7. Factor 69, R7's equations and P−16 remain unchanged. If the derivation cannot justify them, return the concrete conflict to HELP_HUMAN.

Independent exact/directed controls must test actual formation error, not merely E magnitude, at P = 128, 256, 512 and 1024: the failing chord (1,1,2^-80), y_ref = (1,1,0), global-x unit load and u = 0; ordinary aligned/skew frames; g_exp at P−16 and above; fully fixed members; zero and cancelling loads; and `None`/empty-ledger exclusions. Section 10 names the controls.

**The derived bound.** The following majorant, including higher-order terms and E's own formation, was derived by the separate numerical review from the specified DAG and the source. Let ε = 2^-P and η = g_m ε ≤ 2^-16. Coordinate/norm/square-root error gives |L̂/L − 1| < 3ε. For the Gram–Schmidt residual v, with R = ‖y_ref‖, |v̂ − v| ≤ C_v εR where C_v = 12 + 34ε + 38ε² + 15ε³. The exact g comparison gives R/‖v̂‖ ≤ g_m; normalization bounds the y-axis error by C_v η/(1 − C_v η) + 3ε < 15.003g_m ε. The rounded cross product and normalization give z-axis error < 24.009g_m ε. Thus 25g_m ε bounds each axis's absolute error.

For each computed nonnegative E stage B, propagate |X̂| ≤ MB and |X̂ − X*| ≤ dηB. If a coefficient obeys |C* − C| ≤ cε|C|, a rounded sum-of-products stage has the conservative recurrence

`M' = (1+ε)M/(1−ε)` and `d' = [d + (c/g_m)(M+dη) + M/g_m]/(1−ε)`.

The denominator covers E's own rounding, and the c·dη term retains coefficient/error cross terms. Positive sums take the largest incoming count with absolute-operand weights. Reverse coefficient counts are 4 for L, 5 for 1/L and 6 for section terms. Evaluating exact rational bounds at ε ≤ 2^-128, η ≤ 2^-16 and 1/g_m ≤ 1 gives:

| DAG stage | Error count in units of ε times its amplified E majorant, upper bound |
|---|---|
| Local displacement/load projection | 26.001 |
| Strain; Q; raw shear | 32.002; 39.005; 45.008 |
| Corrected ends, including a conservative extra E combining stage | 47.008 |
| Displacement-dependent interior station moments | 41.005 |
| wl; wll; h | 31.002; 36.004; 37.004 |
| Load contribution at ends/stations, including E staging | 39.004 |

Exact t, t², half-scalings and g_m scalings add no rounding; endpoint dispatch inherits the end bound. Therefore **|q_P − R*(u_P)| < 47.1·2^-P·E_q** for the full corrected member recovery. Reactions/support groups retain R7's existing paths; their higher-order and E-formation bound is `68/(1−68·2^-16)^2/(1−256·2^-128) < 68.142 < 69`. The unchanged factor 69 and P−16 therefore cover the combined DAG. This is a derived design bound, not an observed maximum or a new verification law. Section 13's actual-error cohort (`member_load_recovery_error_is_bounded_by_69uE`) checks the factor-69 bound on the maintained recovery vectors.

### 4.3 The lanes (`source_residual::recover`, 1384-1520)

- Keep the length enclosure that `frame()` computes (360-425). `member()` (339-358) currently discards it.
- Per loaded member, form w_loc = Σ ax_enc·point(w), then wl, wll and h with directed operations.
- Corrected local ends: Ã = local − (wl0/2, wl1/2, wl2/2, 0, −h2, h1, wl0/2, wl1/2, wl2/2, 0, h2, −h1).
- Stations keep the lane's present structure on the corrected ends:
  - forces Ã_j,c + wl_c(1 − t);
  - M_y = t·Ã10 + (t − 1)·Ã4 + (wll2/2)·t(1 − t);
  - M_z = t·Ã11 + (t − 1)·Ã5 − (wll1/2)·t(1 − t).

The member global actions used for reactions (1416) stay uncorrected, because reactions already take the ledger. `final_case::action` (715-728) reads the lanes' row enclosures, so stress rows at ends and stations need no change there. The bridge's F3 station errors (bridge.rs 563-600) do not depend on loads.

### 4.4 The maximum row on route L

**Ruling** (HELP_HUMAN, 2026-10-17, option 1): "An unloaded member's maximum is fixed by its end actions. PP's value, evaluated at the published end rows, is the tighter and derivable statement, while the kernel's enclosure of a near-zero end action straddles zero and gives a worse one (2.5e-24 against 1e-96). A loaded member's maximum lies inside the span, where only the kernel's certified enclosure can certify it. So the two value rules follow where the maximum is determined."

**The constraints.** Two checks constrain any recipe:
- *The product gate* (final_case.rs `gate`, about 866-960) judges each published row value n against its certified enclosure, with h = max(n − lo, hi − n). A relative-class row's SharperExact predicate needs h ≤ 2^-64·max(|n|, S) + 2^-85·max(|n|, S) + 2^-53·|n| + 2^-1074.
- *G7's base validator* (`EXTREMA_BOUNDS`, `core/reporting/result_export/src/preview_physics_evidence.rs` 454-470), with PP's own "maximum midpoint" check (retained_product.rs 2985-2992), requires of each `pipe_stress_extrema` record:
  - `value_lower_pa` ≥ 0;
  - lower ≤ value ≤ upper;
  - upper = `global_upper_bound_pa`;
  - value = lower + 0.5·(upper − lower) in binary64;
  - the fixed basis strings `piecewise_quadratic_straight_section_statics`, `j_side_section_equilibrium_binary64_enclosures` and "supplied_binary64_polynomial_coefficient_enclosures; solution error is separate".

Both the predicates and G7 are unchanged.

**One recipe, two cases defined by where the maximum is determined.** For each `CircularMaximum` row on route L (DEF-O):

1. **Unloaded member: the maximum is at an end**, fixed by the end actions.
   - The published value and evidence are PP's: `prepared_maxima` (retained_product.rs 5589-5640) evaluates |N|/A + ‖M‖/Z at the published binary64 end rows. The value is `value_lower + RN(0.5·RN(value_upper − value_lower))` from `certified_publication::fallback_circular_maximum`, delivered through `complete_maxima`, and its eight evidence fields are as today.
   - The kernel's enclosure stays the two-end recipe (final_case.rs 783-800), and the gate judges PP's value against it, as today.
   - This statement is derivable from the published end rows and tighter than the kernel's for a near-zero end action. The kernel's enclosure of a straddling end action has range [0, hi], whose midpoint (2.5e-24 on the milestone) is not derivable from rows of about 1e-96.
   - Every committed maximum byte reproduces; section 4.6 distinguishes this recipe check from the authorized definition-hash re-pins.
2. **Loaded member: the maximum can be interior**, where only the kernel's certified enclosure can certify it.
   - **Value.** n = RN64(H(hull(k, s))), DEF-O's projection H of the hull of the two lanes' search enclosures (below), as `project_hull` forms it for every other row. In `ProductProofDraft::project` (final_case.rs about 1795-1830), a loaded member's `CircularMaximum` row is projected there rather than skipped, so `complete_maxima` receives no value for it.
   - **PP's bounder still runs**, with the member's local full-span loads in place of `&[]`. `straight_local_uniform_loads` (PP `lib.rs` 10605-10640) builds them from captured global intensities and the rebuilt pipe's binary64 orientation. PP keeps `span_index`, `subdivisions`, `station_fraction` and `local_fraction`. Here **L_pp = maximum.value_lower** and **U_pp = maximum.upper_bound**, the interval containing the global maximum (`elastic_extrema.rs` 67-68, 319-320). `maximum.value_upper` is only the witness-value upper and must not substitute for the global upper. The shared ordinary engine does not change; only the retained call's argument and subsequent loaded-row evidence construction do.
   - **Evidence interval.** [lower, upper] = [n − δ, n + δ], the least exact symmetric finite binary64 interval about n containing both the kernel's [lo, hi] and PP's [L_pp, U_pp], with δ ≥ ulp(n) and lower ≥ 0. Search all admissible endpoint grids as specified below; restricting δ to powers of two is unsound at binade edges. Then `global_upper_bound_pa` = upper and `certified_gap_pa` = upper − lower = 2δ.
   - **Half-upper form, including near zero.** If no admissible symmetric interval exists, use evidence [0, upper] and value upper/2, where upper is the least finite binary64 ≥ max(hi, U_pp) whose half is exactly representable. Start at RU64 of that maximum and advance one binary64 step if its exact half is not representable. This applies to every odd subnormal and to odd low-normal significands whose half is subnormal, not just the smallest subnormal. If no finite upper exists, refuse by the named maximum-evidence cause. The unchanged gate judges the resulting value; a finite fallback is not an assurance of passage, even when the enclosure reaches zero.

**Complete symmetric construction.** For n > 0 form exact dyadics L_req = min(lo, L_pp), U_req = max(hi, U_pp), then **upper = RU64(max(U_req, 2n − L_req, n + ulp(n)))**. If upper is finite and upper ≤ 2n, set lower = 2n − upper; otherwise no admissible symmetric interval exists. Form 2n and the threshold expressions exactly, never with potentially overflowing binary64 intermediates. The reflected lower is exactly representable by Sterbenz's lemma (its unbounded-exponent statement when 2n exceeds binary64 range); the result lies in binary64 range. The threshold ensures containment, δ ≥ ulp(n) and lower ≥ 0. Every feasible upper must exceed that threshold, so this is the least upper and least radius. For n = 0 use the half-upper form. This also finds the 3·2^-53 radius that the rejected power-of-two search misses at n = nextDown(1).

Use bounded exponent/significand comparisons and sticky rounding, never an unchecked common-denominator shift; an arithmetic-capacity failure is a named refusal, never truncation. All exact construction work belongs to the existing product-certificate finalization owner (`NumericWork` and sum/visit counters), before completion is sealed. PP supplies its global bounder interval before that owner closes; a row cannot publish from an unmetered post-completion helper. Arithmetic and failure work are charged at actual cost under existing limits/checkpoints, with source pins and tests for the complete helper call chain. Section 10's evidence-construction mutants name the product controls.

**Why G7 holds by construction** (HELP_HUMAN's condition):
- *Symmetric form.* Exact lower + upper = 2n and 0 ≤ lower ≤ n imply n ≤ upper ≤ 2n. Therefore δ = upper − n is exactly representable by Sterbenz's lemma; 2δ = upper − lower is finite and exactly representable, and lower + δ = n exactly. G7's binary64 midpoint identity follows bit for bit. lower ≥ 0 and containment follow from the grid constraints; upper = `global_upper_bound_pa` by assignment. Containing PP's interval preserves its supplied-coefficient enclosure statement and fixed basis strings; the kernel separately certifies solution error.
- *Near-zero form.* lower = 0, and upper − 0 = upper exactly. 0.5·upper is exact because upper's half is exact by construction, and 0 + upper/2 = value. 0 ≤ value ≤ upper, and upper = `global_upper_bound_pa`.

**Basis for the kernel projection.** In the design simulations the kernel projection passed all four predicates in all 252 end-to-end trials; the evidence width does not enter the gate. PP's own value would fail. Its binary64 bounder stops at a 1e-12 relative gap (`elastic_extrema.rs` 113, 327), so its midpoint is off by about 1e-14 to 1e-13 relative for a non-dyadic interior maximum, by up to 1.1e-12 at small magnitudes, and biased upward by the axial widening when |N|/A is below the transport width. In a re-implementation that reproduces the Rust unit expectations, PP's value passed on only one loaded control (126 of 200 trials), and even an exact evaluation from PP's binary64 inputs passed only 37–94 %. These are simulations, not product runs or a guarantee that a real lane's search meets its stopping width.

**Route E** (DEF-E) is unchanged: PP's value and `complete_maxima` for every member. No ordinary or route-E byte moves.

**The loaded-member enclosure, per lane:**
- **Field.** N(t) = (1 − t)N_i + t·N_j and M(t) = (1 − t)S_i + t·S_j + q·t(1 − t), with q_y = w_z L²/2 and q_z = −w_y L²/2. S_i = action(End I) and S_j = action(End J) are the lanes' corrected end enclosures. The q enclosures come once per row from the lane's frame helper, for that one member, and are passed to both `recipe` calls (`final_case::recipe` sees only `values`, `sec` and `facts` today). Each lane uses its own A and Z.
- **Search.** Branch-and-bound on dyadic subintervals, with Bernstein controls N = [N_i, N_j] and M = [S_i, (S_i + S_j + q)/2, S_j], and outward de Casteljau splitting at the midpoint in directed 1024-bit arithmetic.
- **Upper bound per node: min(paired, tangent).** Both bounds are rounded up.
  - *Paired*: the objective at each joint Bernstein control point, the law of `control_upper`. It overestimates by O(h²).
  - *Tangent*: the Bernstein controls of sN/A + (Q/Z² + ρ²)/(2ρ), with Q = M_y² + M_z² as a degree-4 polynomial, s = ±1 on a sign-constant N, and any ρ > 0 (an approximate ‖M(mid)‖/Z).
  - The box-corner bound (separate maxima of |N|, M_y and M_z) overestimates by O(h). It exhausts 256 nodes on the skew two-axis and axial-slope controls, and live nodes grow 4× for every 16× tighter tolerance. It is not used.
- **Lower bound.** The search's best evaluated point, rounded down. The published governing station's value is not used: it would leave the enclosure 212 to 10,686 × 2^-53 wide.
- **Stop.** When U − L ≤ 2^-64·L. The tolerance bounds the search's own convergence on the problem it solves. On the admitted lane, whose represented Z is the interval hull(Ẑ, Î/c), the search converges at the hull's binding endpoint, the smaller Z, where the field is largest, under the unchanged tolerance and limits. The lane returns [the lower evaluated over the whole hull, U], so the hull's width is carried soundly into the enclosure, and the unchanged gate decides the projected row (REQ-003 (e)).
- **Limits.** 256 live nodes, depth 64, and 1,024 splits per lane. Exhausting any limit preserves [L, the maximum upper bound over live nodes and the unexpanded parent] as an enclosing diagnostic, and **returns a named kernel exhaustion failure directly**, making the case unavailable. Width alone cannot guarantee failure of an absolute-class gate. Exhaustion prohibits publication even if that interval would satisfy the gate; limits, failure propagation and all spent work are retained.
- **Basis for the limits** (design controls):
  - the main controls certify in ≤ 33 splits, ≤ 18 live nodes and depth ≤ 33;
  - near-degenerate rotating moments down to e = 1e-8 certify in ≤ 26 splits and ≤ 13 live nodes;
  - a tie at 1e-58 certifies in 30 splits;
  - an axial sign change and a moment passing through zero both certify.
- **Memory.** The actual `SearchNode` has 19 Endpoints plus indices/padding: 2,744 B. Its 256-record backing is 702,464 B (about 0.67 MiB) per lane. The complete source union is 1,021,952 B, including helper/return/moving staging, within the 1 MiB charge; the W1 coverage repair's driven search adds four endpoint slots, giving 1,022,528 B, still within it (retained-pricing-rules.md §8.11, W1R-LM). The lanes run one after the other with one member live, within L4's 1 MiB cap (section 9).
- **Work.** Section 5.

**Corners that fail closed.**
- **Actual-lane width boundary (repaired, corner 1).** Actual PP preparation, native solve and freeze for a simply supported length-2 member (D = 0.2 m, wall = 0.01 m, global-y UDL 3.5 N/m, no nodal loads, translations/RX fixed and RY/RZ free at both ends) reaches native selection. Exact arithmetic shows that the represented `Z` hull, from separately rounded `Î/c` and `Ẑ`, alone imposes relative width **5.3348979419179844·10^-18**, or **98.41149699 times 2^-64**. Subdivision cannot remove that input-width floor, so while the stop rule was applied to the hull the search refused at row 81 with the named loaded-search exhaustion. As built (the W1 coverage repair's corner 1, [w1-coverage-repair.md](w1-coverage-repair.md) §1.2), the admitted lane's search is driven at the hull's binding endpoint z_lo, converges there under the unchanged tolerance and limits, and returns [the lower over the whole hull, U]; the unchanged gate passes every row in both modes, and the member publishes beside an independent trigger (`w1_represented_z_hull_converges_on_ordinary_sections`, which writes the corpus bases `w1_member_hull_d02_{sparse_interactive,dense_scrutiny}`). A point hull and the annular lane keep today's search, work and bytes (`w1_point_hull_search_is_unchanged`, `w1_annular_lane_search_is_unchanged`). A distinct invented section with a point represented-Z hull and source-I relative rounding below 4.159·10^-22 passes actual preparation/native/freeze in both modes for symmetric **1.5 Pa** and asymmetric **73/24 Pa** interior maxima. These are numerical controls, not external physics references. No search, verification or domain limit changes.
- **Exact scale boundary.** Adding an independent sensitive member supplies a genuine ordinary W1 trigger. The 1.5 Pa control passes full publication in both modes. The 73/24 Pa candidate reaches freeze. Its exact force maximum `(7/2)Z` and coupled moment `7Z` are binary64 halfways, and native approximate scales round one ulp above the published-row maxima. Until the W1 coverage repair's corner 2 ([w1-coverage-repair.md](w1-coverage-repair.md) §2), the receipt stated those native scales, so the reader's body-1 force and moment scales differed from the receipt's by one ulp and G5b refused it. As built, the receipt states the final scales the product gate judged every row with, the unchanged G5b check passes, and the control publishes in both modes (`w1_loaded_scale_halfway_publishes`; the native publication keeps its own scales, `w1_native_scales_stay_native_evidence`). It was never a transport-unit mismatch. A separate full-publication asymmetric control uses end moments −Z/−3Z, giving exact force and moment scales 4Z/8Z and the independently derived maximum **11/3 Pa at t = 2/3**. Both shapes pass the complete private preparation/native/freeze/serialization/native-reader path and the actual registered public Direct entry in both solver modes; public bytes equal the private proof's successor exactly. The maintained writer is `w1_member_interior_publications_bind_the_actual_direct_entry`; its four `w1_member_interior_{symmetric,asymmetric}_{sparse_interactive,dense_scrutiny}` records are corpus inputs.
- Near-degenerate loaded maxima with a varying axial force could exhaust even min(paired, tangent). This is theoretical and was not observed; a Taylor bound with an f″ enclosure would close it at higher cost.
- The two lanes use different sections (binary64 A and the Z hull against geometric A and Z), which widens every stress row's hull, maxima included. In a rough model it alone fails a perfect projection in 296 of 4,000 trials. This is inherited by all stress rows and not created here.

The first two corners declined ordinary cases. Both are repaired as built by shared epoch 6 in [b7-plan.md](b7-plan.md) ("After B7b"; [w1-coverage-repair.md](w1-coverage-repair.md)): the scale corner (corner 2) and the width corner (corner 1). The search still exhausts only where its own convergence fails under the unchanged limits (the theoretical near-degenerate case above); exhaustion still fails closed. The maintained genuine-exhaustion control is therefore hook-driven: a real member at reduced search limits, through the kernel's opt-in `loaded-maximum-hooks` feature (`w1_member_load_exhaustion_retains_typed_interval_work`; w1-coverage-repair.md §1.4, option 2). A loaded maximum whose certified hull is too wide for the unchanged gate keeps its ordinary result, with the gate's named cause (REQ-006 (f)). The census attributes that width to the two lanes' section difference (the last corner above), to the represented-Z hull alone, or to both (w1-coverage-repair.md §3.5: of 24 failing C1-b maxima, 6 to the hull alone).

### 4.5 Combinations

`combine.rs::validate_preps` (233-252) refuses, after `NestedCombination` (238) and before `OperandsDiffer`, any operand whose source has member loads, as a new `CombinationReason::OperandMemberLoads`. This matches DEF-C's `operand_member_loads`. As a backstop, `RetainedLedger::combined` (ledger.rs 209-229), which reads only nodal loads, refuses such a source with a new `LedgerRefusal`, mapped in `CasePrep::combination` (adaptive.rs 985).

### 4.6 The maximum census (HELP_HUMAN's request, 2026-10-17; measured)

**The question.** HELP_HUMAN asked: "State, measured over the committed successors and corpus bases: how many published maxima change, and by how many ulps at most; that each changed value passes the unchanged predicate; whether any nodal-only successor's verdict or standing moves."

**How it was measured.** On main `4971d62dc9`, through the producer:
- 41 route-L publications, 35 of them distinct: the milestone, L0, W-C2 and combination successor fixtures in both modes; CX-F ×2; N06 ×4; the 25 producer-solved corpus bases through the public Direct entry; and the two hook bases through their private driver;
- the 54 trials of `b4_w1_covers_every_formerly_selected_source_block_case`, whose successors are not committed.

There were 244 maximum rows, all on unloaded members and all AbsoluteVerified; the controls add 6 relative rows. The two exact successors were byte-identical in every mode.

**Under the adopted maximum recipe, nothing moves in this measurement.** Every original committed maximum is on an unloaded member, so case 1 keeps PP's value and evidence. With PP's value kept, all 43 committed byte strings reproduced exactly. The maximum recipe alone moves no maximum, verdict, standing, receipt or publication byte, and the 16 control trials keep their standing. The complete stage-2 implementation separately re-pins the authorized DEF-O-bound receipt hashes (section 6.4); that is not an exclusion from maximum/publication byte preservation. The original committed successors and corpus bases had no loaded member; the new member-load bases establish their maxima under case 2.

**Why the rejected alternative (case 2 for every member) would have moved bytes:**
- *Values.* 74 distinct maxima would change, in 33 publications:
  - 70 milestone/W-CB1 M1 rows, from 8.17e-92 to 2.51e-24 Pa;
  - 4 N06 torsion-only rows, from +0 to 2.14e-42 Pa.

  The ulp distance is either 0 or above 1e18; measured against the absolute bound, 3.1e-7·b and 3.7e-11·b. Every changed value passes the unchanged absolute predicate, and all 46 case certificates pass.
- *The cause is not the location of the maximum.* The kernel's end-action enclosures straddle zero (milestone M1: |N| ≤ 8.18e-27 N and |M_y|, |M_z| ≤ 7.2e-28 N·m, against a torsion of 0.0144 N·m). So |·| of the enclosure has range [0, hi], and the RN64 midpoint of that range is far above PP's value at the published end rows.
- *Standing.* With evidence RD64(lo)/RU64(hi), CX-F ×2 loses its successor at G7 `EXTREMA_BOUNDS`. With symmetric evidence, every committed successor would keep its standing, but 16 control trials would lose theirs: F4/cancelling, F4/net, T20/refused, T20/control, F6/partial, T18, T19 and T19/pre-K-D5, each in both modes. Their [0, hi] ranges admit no lower ≥ 0 whose midpoint is RN64(hi/2).

**The maintained census** (`w1_committed_publications_preserve_all_but_authorized_definition_hashes`). The 43 committed inputs keep maximum, publication, ordinary and all other nodal-only bytes; only the specifically authorized definition-bound receipt hashes of section 6.4 are re-pinned, and the two route-E successors are entirely byte-identical. No capacity counter, numeric evidence, maximum or verdict is masked. The 54 trials keep their standing over all 62 formerly selected cases (`b4_w1_covers_every_formerly_selected_source_block_case`; 70 since the W1 coverage repair adds case 1 of the eight group-3 trials below), and each new loaded maximum passes G7 and the gate. For F6/partial, T18, T19 and T19/pre-K-D5 in both modes, case 0 stays selected. Case 1 reaches a prepared selected Run. Before the W1 coverage repair it was unavailable at the facade with the exact product error `proof → association → loaded maximum search exhausted` (the represented-Z width corner, 4.4). As built (corner 1), its search converges and the unchanged gate passes every row, so case 1 is selected and each of the eight trials is numerically eligible; the B4 table records these eight `case-b` cases too (70 cases). An eight-case consuming subset checks the outcome and its case/attempt bindings, and the maintained helper derives the expected status from the attempt's own certificate check (`w1_partial_case_cause`).

## 5. Work

**Where it is counted.** The declared ledger is formed at the start of the case's first own-stage run (`solve_case_at`, adaptive.rs 1908-1954), in its `rhs` stage. It uses that function's existing 1024-bit context and accumulator. The work flows into the attempt's `work`, `k4_work` and `stages.rhs` once, and is charged once to the case limit and the invocation meter. `guard.test` runs after each member (all its loads) and after netting, the same checkpoint discipline as `form_members` (assemble.rs 521-534). The stage fields are published and closed in the schema (retained_wire.rs 283-286), so no stage is added.

**Wide-arithmetic cost** (multi.rs 86-93 at L = 16, in limb-multiply equivalents; `SumWork` is added below):
- per loaded member, at most 38,822: 2,880 for nine two-products, 32 for the rounding, 18,468 for the square root, 17,442 for the division;
- per load, at most 7,776;
- plus `SumWork`, so the formation is never zero;
- these wide-only figures do not bound complete formation or checkpoint overrun.

**Complete conservative formation/checkpoint bound, including `SumWork`** (independently derived, with the new canonical-net accessor bounded here):
- A nonzero `add_raw` touches at most 128 term/carry limbs plus 256 anchor-shift limbs: at most 384 LME (wide_sum.rs 405-506). One sum rounding adds at most 256 net limbs plus 128 rounded limbs: 384 LME.
- Fixed per member: 38,822 + 9 × 384 + 384 = **42,662 LME**. Per load, a conservative 24 raw insertions and three roundings add 24 × 384 + 3 × 384 to 7,776: **18,144 LME**.
- One member holding all 128 loads therefore costs at most **2,365,094 LME** between member checkpoints.
- Final netting over at most 192 DOFs inserts at most 1,536 declared terms and 192 nodal nets, then nets each DOF: (1,536 + 192) × 384 + 192 × 256 = 712,704 LME through the existing sum API. The new canonical odd-net extraction adds at most three 128-limb scans/shifts per DOF, explicitly charged, or 73,728 LME. Total netting checkpoint: **786,432 LME**. Its source pin and test must bind the accessor to those scans; adding another pass requires updating this bound.
- Across 32 members, 128 loads and final netting, complete declared formation is at most **4,474,048 LME per case**. This deliberately overstates mixed nodal/member input because D1's 128 authored loads cap is shared.
- Maximum additional declared-formation work between two checkpoints is **2,365,094 LME**, hence a checkpoint may detect at most that much overrun (about 0.01183% of 20e9). The final netting checkpoint's bound is smaller. This is a conservative design bound for these operations, not a strict pre-execution cap or a bound on the whole solve/maximum search. Implementation must establish the prescribed operation schedule and bound against source; no unpriced/unmetered canonicalization is allowed.

The loaded maximum's search counts through `NumericWork` plus its wide and sum work:
- up to 14,498 directed entries per lane per loaded member in the observed controls (about 440 per split), about 29k for both lanes; this is not the split-budget maximum;
- at least 238 with no split;
- at most about 0.45M per lane under the split budget.

**RF's metered rule, applied.** RF's rule (rf-source-method-extension.md §5.1) is scoped to `exact_boundary`'s `Work`, which no W1 path uses. Stage 2 applies its substance through W1's existing counters:
- every operation is counted at its actual executed cost (`WideContext` charges each operation as it executes, multi.rs 1162-1186; the accumulator charges per limb touched);
- nothing is precharged;
- nothing is counted as zero;
- there is one rule, with no member-load regime.

**Refusal timing (R-1, HELP_HUMAN, 2026-10-17).** Preserve W1's actual-cost counters and `StageGuard` checkpoints after each member and after netting (adaptive.rs 239-295). RF's pre-step refusal belongs to `exact_boundary::Context`; it does not change W1's policy. These are checkpoint limits, not strict pre-execution caps. A checkpoint permits equality and refuses when used work exceeds case or invocation room. All executed work, including the member or netting that exhausts the room, remains counted once in the stage, attempt, case and invocation totals. Exhaustion stops the attempt before any subsequent formation, solve or publication; there is no unmetered continuation. The limits are unchanged. The preceding bounds include both `SumWork` and wide arithmetic.

## 6. Producer, definitions, receipt, schema and readers

### 6.1 Retiring stage 1's interim cause

Delete the following:
- `MEMBER_LOAD_CAUSE` (retained_product.rs 43);
- `capture_member_load_case` (1794-1834);
- the stage-1 skip of the captured-invocation check (4124-4125).

Restore the check in `finish` (2253) that every captured case has a source. `MEMBER_LOAD_CAUSE`'s 86 B of TEXT leaves the profile.
Keep the existing `member_loads` Boolean fields in their old layouts; they now bind sidecar presence instead of selecting the interim refusal. Removing or enlarging those fields is not part of this design.

### 6.2 Capture (`retained_product.rs`)

- **Early raw check.** `check_captured_nodal_leaves` becomes a check over both families, run for every case. For an element leaf it requires:
  - the raw target `element` with the same pipe;
  - category `distributed_force`, dimension `force_per_length`, and a direction in `global_x|y|z` (D1.7);
  - the same id;
  - the raw value converted to N/m giving the same bits, with the normalized unit `N/m`;
  - no extent.
- **Occurrence capture.** One capture of the case's terms with one shared "used" tracker (generalizing 2113-2225). Each member occurrence must match exactly one unused authored load with the same id and magnitude bits, the target pipe equal to the model's pipe at that index, and the same global direction. The member is the model pipe index as `u32`, which is the kernel member id. The intensity has only the load's axis set; the off-axis entries are +0.
  - Then come the single unconsumed-load check, the unchanged `PrimitiveSource::new` for the nodal part and the validated member sidecar, and one canonical map per family.
  - New refusal texts: "member load occurrence not authored" and "canonical member term map".
- **`MemberTermIdentity`** (about 64 B) and the member loads live in a member-only per-case sidecar. `ProductCapture` owns optional invocation-level storage keyed by requested case index, outside `per_case_capture!`, `CaseSlot`, `EarlyCaseSource` and `PrimitiveSource`. It is created/reserved only for actual member cases, retains early/late occurrence custody and basis identity, and is borrowed with the matching case during `with_case` and submission. No sidecar state is swapped through old slots. Basis reuse never reuses another case's load sidecar.
- **Layout preservation is required, not inferred from conditional events.** `Adapter::reserve` emits capacity × `size_of::<T>()` (3997-4004), including `Option<EarlyCaseSource>` (4097), `CaseSlot` (469) and multi-case `PrimitiveSource` submission (5196). Those types and the original nodal reservation calls remain unchanged. Member-aware submission uses an additional separately reserved aligned view vector only when member cases exist; every sidecar/vector allocation and move is counted truthfully there. No old nodal size substitutes for a larger allocated type. A nodal-only invocation, including route E, has neither new allocations nor new events. D3 prices the invocation-level header, sidecar and view staging positively; the actual-source owners are bound in `tools/retained_pricing/base/w1_member_load_model.py` (section 9).
- **Gating.** New sidecar/view adapter events and reservations occur only on a member-aware invocation path; a wholly nodal invocation keeps its old submission path. New case-local formation charges occur only for member cases. Adapter counts and `prepared_capacity_bytes` are receipt bytes, and the adapter records an allocation event even for an empty vector (3987-4006).
- **Preparation copy.** `prepare_active_case` (5274-5375) copies the source's member loads into prepared capacity slot 15 (unused today), with slot 10 for ids.
- **Late check.** `check_early_occurrences` compares counts per family, and each member term's id, member, axis, bits and extent.
- **Self-weight ids.** The D1 law (`GeneratedLoadIdentity`) refuses them, and the readers mirror that refusal.
- **Route E.** It keeps refusing element loads (D1.7; DEF-E unchanged). The "unsupported producer present" flag (1790) becomes "route E with element loads, or imposed displacements".

### 6.3 Receipt and SCHEMA (`schemas/retained_precision_mp_v2.schema.json`, reviewed input #2)

**`$defs/CaseSource/properties` gains `member_load_terms`.** It is optional, not in `required`:

```json
"member_load_terms": {"type":"array","minItems":1,"items":{"type":"object",
  "properties":{"constructor_ordinal":{"$ref":"#/$defs/U"},"source_id":{"$ref":"#/$defs/Text"},
    "primitive_load_index":{"$ref":"#/$defs/U"},"member":{"$ref":"#/$defs/U"},
    "intensity":{"type":"array","items":{"$ref":"#/$defs/Bits"},"minItems":3,"maxItems":3}},
  "required":["constructor_ordinal","source_id","primitive_load_index","member","intensity"],
  "additionalProperties":false}}
```

- The constructor ordinal is the authored primitive-load index, as for nodal terms (wire 1053-1057).
- The intensity is the global N/m vector.
- The field is emitted only when non-empty, after `nodal_terms`, in the kernel's canonical order.

**Other SCHEMA changes:**
- `CombinationReason` gains `operand_member_loads` (about line 7845), and the wire's `combination_reason` (1487-1498) gains it too.
- Kernel member-load `SourceError` variants either receive SCHEMA tags (line 1282, with the wire census at retained_wire.rs 2410-2424) or fail as an association error, as directional springs do (597-600). No genuine input reaches them.
- `LedgerError` retains the nodal accumulator object and adds the closed tag-only `member_loads` and `not_formed` alternatives.
- A member lane records `member_load_curvatures` as its tenth named capacity; nodal lane traces retain their nine-entry slice.
- A failed loaded search adds optional `proof.loaded_maximum_diagnostic = {row, member, lower, upper}`. Each endpoint is `{negative, exponent, limbs}` with 16 little-endian lowercase 16-digit hex limbs and a signed safe-integer exponent. G2 checks those encodings; G5 checks normalized binary1024 parts, canonical positive zero, nonnegative ordered endpoints, the active source member and its next unprojected maximum row. This field is required exactly for the named projection-stage `loaded maximum search exhausted` failure and is rejected on ready, nodal, combination or other failures. The serializer borrows the existing proof failure, preserving the nodal typed-view layouts and copy counts.
- Failed active-member values completion transfers its arithmetic owner into the proof once and reports `completion.kind = merged`; the failure's arithmetic fault owner is that proof. Nodal, all-zero and combination values failures retain `separate_failure`. Both readers derive this distinction from exact authored member activity, and apply the existing terminal-stage and work checks. This does not change R-1's native checkpoints or any work limit.

**Receipt bytes for a member-load case:** `member_load_terms`, `kernel_source_sha256` (K4SRC with the member block), `selection.ledger_sha256` (K4LED with the declared entries), the rows and maximum evidence, the adapter counts and slot 15. The interim preparation error and the `source_unavailable`/`preparation` reason disappear.

**Nodal-only receipts.** These stay byte-identical except for the DEF-O-bound hashes of section 6.4. Their maxima do not move (section 4.6).

### 6.4 Definitions and the disclosure (HELP_HUMAN, 2026-10-17: "accepted as you recommend")

The disclosure is placed in three places: the DEF-O/DEF-C wording; table #4's `supported_profile_limitations`; and a fixed member-load message on each selected member-load case's `RETAINED_PRECISION_SELECTED` diagnostic. Neither nodal-only nor ordinary bytes move.

- **DEF-O** (`fixtures/results/retained_precision_prepared_ordinary_v1.json`, #3; keep its sorted-key compact form):
  - `scope.loads` becomes `"individual_normalized_nodal_terms_and_declared_full_span_uniform_member_terms"`.
  - New `lanes.loads`: "the case's K4LED: each loaded DOF's exact net of its individual normalized nodal terms and its declared member-load entries; per full-span uniform member term (member i-j, global intensity w in N/m, binary64) six fixed binary1024 entries formed once: s=d.d exact from nine binary64 products with d never formed, S=RN1024(s), L=RN1024(sqrt(S)), g=RN1024(L/12); force RN1024(L*w)/2 at i and at j; moment RN1024(g*(d x w)) at i and its negation at j, d x w exact; folded exactly into the same per-DOF nets and per-term data flags; residual, verification, reactions and Ŵ/E use them exactly as the ledger; member end actions and stations use the member-local consistent load from w and L".
  - New `acceptance.declared_member_load`: "The certificate certifies the declared-load problem K u = f_n + f_d. Each declared entry differs from its exact consistent value ((L/2)w and ±(L/12)(d x w) at the exact length) by at most 2^-1022 of that value's magnitude; this difference is disclosed, not certified, and enters no published bound." The bound is stated per entry, because entries from different members can cancel at a node.
  - `rows.maximum` keeps its present text for an unloaded member, whose maximum is at an end, and adds the loaded case. The new text: "unloaded member (maximum at an end): existing binary64 span-statical coefficient bounder from final projected endpoint actions and prepared section, no loads/no pressure; regenerate all eight numeric evidence fields; output RN64(lo+RN64(0.5*RN64(hi-lo))); retain coefficient scope/location/tie/coverage and certify actual output against separate dual physical maximum. Loaded member (maximum may be interior): output H of hull(admitted_k, annular_source) directed branch-and-bound circular maxima of |N(t)|/A+‖M(t)‖/Z over t in [0,1] with the member's local full-span uniform loads; the bounder above runs with the same loads for span/station/subdivision evidence; value_lower/value_upper are the least exact symmetric binary64 interval about the output containing both the certified hull and the bounder interval, with lower >= 0, or, if none, output upper/2 with evidence [0, upper]; global_upper_bound = value_upper; certify actual output against separate dual physical maximum".
  - The meaning hash moves in the producer (retained_wire.rs 34-35), the Rust reader (9-11), the verifier (`DEFINITION_HASH`), the table (its formation-definition hash) and desktop test support.
  - Its trust wording distinguishes unchanged nodal-only ledger attestation from selected member cases' independent invocation-bound Rust/Python K4LED validation. That validation binds declared-load data; it does not rerun the solve or certify the disclosed physical-load discrepancy.
- **DEF-C** (`retained_precision_prepared_combination_v1.json`, #6): `scope.excludes` appends `"operand_member_loads"`, and `inherits.operand_definition.sha256` binds the revised DEF-O. Its disclosure states: "A full-span uniform member load is represented by six declared binary1024 equivalent nodal entries per authored term, formed once from binary64 inputs; each entry differs from its exact consistent value by at most 2^-1022 of that value's magnitude. Retained certificates certify that declared-load problem. This combination definition excludes every operand carrying member terms, including zero or cancelling terms."
- **DEF-E** (#7) is unchanged.
- **Preview-retained table** (`semantic_contract_v0_3_preview_physics_retained_1.json`, #4):
  - the two hashes in `product_formation_definitions` (about line 1173) are updated;
  - `supported_profile_limitations` appends: "A full-span uniform member load is represented by its declared equivalent nodal load: six binary1024 entries per load, formed once from the binary64 inputs, each within 2^-1022 of its exact consistent value relative to that value. Retained-precision certificates certify that declared-load problem. A mechanics combination with an operand that carries member loads is not retained."
  - Its existing list is a seven-entry subset of the base's nine, and no reader consumes it. That observation is outside this unit, and the list is not rebuilt here.
- **Selected-case diagnostic** (retained_wire.rs 69-71, `selected_case_envelope` 798-822). A selected case whose source has member terms takes a second fixed message, `SELECTED_MEMBER_LOAD_MESSAGE`, under the same `RETAINED_PRECISION_SELECTED` code: "Retained-precision recovery (contribution_preserving_multiprecision_v1) is selected for this load case. Its full-span uniform member loads are represented by declared equivalent nodal loads, each entry within 2^-1022 of its exact consistent value relative to that value; the certificate certifies that declared-load problem. Its published rows carry recovery_method; the retained_precision receipt binds their certified classes, the native attempts and work, and the ordinary-attempt evidence." Nodal-only cases keep today's message, so their bytes do not move. Both readers require the member-load message exactly when the case's source has `member_load_terms`.
- **Corpus effect.** The DEF-O wording moves the DEF-O-bound hashes of every preview W1 successor and corpus base: `sources[].preparation.sha256`, `cases[].source_identity_sha256`, combination operand identities and `receipt_sha256`. `publication_sha256` and ordinary bytes do not move. This follows the E14 and N06 precedent; HELP_HUMAN, 2026-10-17: "The DEF-O hash moves under the E14 precedent are fine."

### 6.5 Rust reader and Python verifier

The Rust reader is `core/reporting/result_export/src/retained_precision.rs`; the Python verifier is `tests/verifiers/retained_precision.py`.

- **G8 member terms** (Rust from 5139; Python 2117-2126). For each authored load in the captured invocation:
  - a node target follows the existing nodal rules;
  - an element target is admitted only on route L. Category, dimension and direction follow D1.7, and the id must not have the self-weight form (mirroring `self_weight_form`). The member is the pipe's model index. The value is the unit-converted magnitude in force per length (Rust `unit_value` 4507; Python `unit(..., "force_per_length")`; N/m and lbf/ft are in `units.py`);
  - anything else refuses with `PREPARATION_MISMATCH`.

  The member terms are sorted by member, source-id bytes, the three hex intensity words, then constructor ordinal. `member_load_terms` must be absent when there are none and exactly equal otherwise. That catches missing, extra, reordered, changed and duplicated terms.
- **K4SRC** (Rust `native_bytes` 5542-5640; Python 366-410): after the supports, the member block is appended only when present, in the kernel's byte layout.
- **Data flags** (Rust about 4022-4046; Python about 1600-1606): derive member data from independently recomputed individual entries, before netting, at free DOFs. A nonzero declared force or moment contributes data only when its corresponding translation or rotation is free. Nonzero cancelling entries remain data; moments acting only on constrained rotations do not. Nodal-only checks remain unchanged. The broader g-check activity of section 4.2 is a separate predicate: nonzero member-source recovery may require conditioning checks even without a free-DOF data flag.
- **Combinations:** a `retained_selected` combination has no operand source carrying member terms, and an `operand_member_loads` reason requires one (extending the cause check at 3113-3121).
- **Selected-case diagnostic:** the member-load message exactly when the case's source has member terms (section 6.4).
- **Maximum evidence on route L:** G7's unchanged `EXTREMA_BOUNDS` judges the loaded member's evidence. It needs no reader change, because the evidence satisfies it by construction (section 4.4).
- **Pins:** the DEF-O, DEF-C and table hashes. G0 needs no code change, and G1's source hash already covers the whole source object.
- **K4LED:** both validators independently recompute the selected member-load case's declared problem and digest, with matching G8 `PREPARATION_MISMATCH` behavior (section 2.6). Nodal-only cases keep the existing path. The independent Rust arithmetic also runs in wasm, and its transients are priced explicitly.
- **Desktop.** Product TypeScript reads through the Rust reader in wasm, so no product TypeScript changes. Test support changes: the `DEF_O` constant (`retainedPrecisionReaderInputs.ts` 11), the successor-fixture list if a member-load fixture is added, and the pins in `retainedPrecisionResultExport.test.tsx` 51-52.

### 6.6 PP's preparation of the maximum

`prepared_maxima` (retained_product.rs 5589) follows section 4.4's two cases.
- **Unloaded member.** Unchanged.
- **Loaded member.**
  - It passes the member's local full-span loads to `exact_straight_summary_extrema` in place of `&[]`, and keeps that call's span, station and subdivision evidence and its interval.
  - It takes the kernel's projected value and certified hull, and forms the symmetric (or near-zero) evidence interval.
  - It no longer passes a value for this row to `complete_maxima`.
  - PP's "maximum midpoint" check (2985-2992) applies unchanged.

Route E is unchanged, and the shared ordinary engine is untouched.

### 6.7 The Preparation fallback control

The interim member-load Preparation cause is removed. Member-load inputs now build their source and reach Native; later numerical/search refusals retain their own named cause.

A genuine annulus boundary now covers Preparation: diameter bits `300fffffffffffc2`, wall bits `2fc762b28a18eb2c`. The ordinary section's I rounds to the least normal binary64 (`0010000000000000`), so the old source guard admits it. Closed annulus preparation rounds I subnormal and refuses; both modes have a real ordinary trigger and return the Preparation fallback. `w1_annulus_boundary_is_a_real_preparation_fallback` retains this control. A plainly smaller section instead refuses at the earlier `SubnormalDerivedPrimitive(SecondMomentY)` source guard, as `w1_subnormal_annulus_is_declined_before_preparation` checks. Neither control uses a member-load placeholder or changes a range rule. Fault and custody-tamper controls remain beside the real input.

## 7. U6/U8 and the later O4 epoch

**This unit certifies Euler–Bernoulli straight members only.** Route L carries no shear model and no allowance (P30). `StraightMember::from_frame` refuses a Timoshenko leaf by name (`RETAINED_TIMOSHENKO_REFUSAL`, assemble.rs 226-281). W1's retained member names its Euler–Bernoulli leaf (retained_product.rs 1998-2016, 5620-5632). Timoshenko and nonzero-allowance documents were declined by O4's interim declines in `prepare_sources` and at W1-E's admission (`ShearModel`, `SectionAllowance`) until O4 admitted them on route E and the source route (`o4-source-extension.md`); route E refuses element loads (D1.7), so no Timoshenko member reaches this unit's loaded-member search.

**The Euler–Bernoulli leaf:**
- f_d is section 2.2's recipe.
- Its closed-form controls are:
  - cantilever: tip deflection wL⁴/8EI;
  - simply supported span: θ_i = wL³/24EI, midspan moment wL²/8, and the interior maximum (wL²/8)/Z;
  - fixed-fixed member: end moments wL²/12, midspan wL²/24;
  - each of these skewed, with load components on two local axes, and at large coordinate offsets.

**What O4 will need from this unit:**
- **Load and statics carry over.** For prismatic Timoshenko members, the full-span equivalents and station statics are identical (U6/U8 §3.4–3.5). The declared recipe, the station terms, E's load terms and the loaded maximum therefore carry over unchanged.
- **O4 supplies the stiffness.** It must supply a Timoshenko retained K, because `StraightMember` has no beam theory today. Its deflection controls need Timoshenko twins.
- **Full span stays a type-level fact.** `UniformMemberLoad` has no extent, so a stale Euler–Bernoulli partial-span vector cannot reach a Timoshenko member.
- **The maximum recipe carries over.** Section 4.4's two cases follow where the maximum is determined, so they hold for any beam theory. If a later unit admits member loads on route E, DEF-E would adopt case 2 for loaded members (per basis after D-5). Route E has none today (D1.7).
- **Headroom.** O4 and U4b/U5b need memory headroom of their own (section 9).

## 8. RF interactions

- **Metered charging.** Section 5. Stage 2 adds no `exact_boundary` work, so RF's pre-step refusal does not apply. R-1 (HELP_HUMAN, 2026-10-17) preserves W1's actual-cost counters and checkpoints after each member and netting: all executed work, including a refused checkpoint's work, is counted once in stage, case and invocation totals, and exhaustion permits no continuation or publication. No second member-load regime or W1-wide policy change is introduced; limits stay unchanged.
- **The J7 RF census.** It does not interact. The census runs `run_linear_static_preview_value_with_mode`, which never selects a retained caller profile (PP `lib.rs` 2322-2328). It excludes its three member-load cases as `non_nodal_load` (`j7_rf_census.rs` 432, 444). Stage 2 cannot move the census pin, and RF cannot move stage 2's route-L controls.
- **Stage 2 changes the W1 capture path, not the source method.** Stage 2 changes `retained_product::capture_case_source` and its successors (section 6.2), and the W1 kernel. It does not change `source_recovery::prepare_sources`, whose non-nodal decline (`Unsupported("non-nodal load producer present")`) stays.
- **The `load_reference_fallback_uz` fixture depends on that decline** (`core/reporting/result_export/tests/fixtures/load_reference_fallback_uz-*.raw.json`). RF keeps it an ordinary-only pressure-1 raw by adding a zero full-span uniform member load (`uniform:0`). `prepare_sources` declines that load by name, at 37,280 work. T13 (`s11g_tests.rs`) asserts the pressure-1 identity and the named decline.
  - Stage 2 leaves the fixture's outcome unchanged. Its request is 0.5.0 with `3.0.0/exact_pressure_v3`; W1-E keeps refusing element loads at D1.7 (`LoadTarget`), and `prepare_sources` is untouched.
  - Any later unit that admits member loads in `prepare_sources` must re-handle this fixture and its consumers (T13, the reader-verdict and base-reader corpora, `loadReferenceReaderCases`, `load_reference_source_contract.rs`, the desktop load-reference tests and `test_ci_numerical.py`; RF §5.5).
- **The retained corpus.** RF expects `fixtures/results/retained_precision_cases.json` unchanged for its own effect. Stage 2's corpus is regenerated on top of RF's bytes (section 6.4).
- **D3.** Stage 2 re-registers from RF's registered profile (RF merged as `03f39bb49d9`). RF's actual native delta is 16,361,805 B per phase, and its registered dense headroom is W4 +35,451,012 B and X1 +60,972,710 B (RF §9). The source-method decline and J2/J7 pins remain unchanged.

## 9. Memory (D3)

Fixed throughout: M = 12 GiB = 12,884,901,888 B, floor(0.9M) = 11,596,411,699 B, R = 64 MiB, and the upward 5 % TEXT reserve (TAV_W/TAV_X). Headroom is measured below the floor after the reserve.

**Basis.** Stage 2 re-registers from RF's registered profile (section 8). U6/U8 registered merged dense W4 +51,812,817 B, merged dense X1 +77,334,515 B and merged sparse W4 +110,944,161 B; RF adds 16,361,805 B per phase, leaving merged dense W4 +35,451,012 B and X1 +60,972,710 B.

**Actual-source registration.** The registered profile's comment names the W1 actual-source D3 at `11b1dbfa69`; the pinned native record and its phase bounds are at `32bfb4d441`, whose coefficients differ from it only by the G8-scope TEXT below. The canonical 100-form union binds 297 native/text values. The stage-2 owners are bound in `tools/retained_pricing/base/w1_member_load_model.py`, beside the reduction cohort in `base/w1_reductions_model.py` (the tool's README, "W1 reductions" and "W1 stage 2 actual-source owners"). Stage 1's route-L `lc−1` credit is retired (below). At the actual native atoms, every one of the 70 fixed-law evaluations fits:

| Phase | Merged dense headroom B | Merged sparse headroom B |
|---|---:|---:|
| W1 | 5,944,651,283 | 6,003,782,627 |
| W2 | 5,613,260,465 | 5,672,391,809 |
| W3 | 318,538,197 | 377,669,541 |
| W4 | **113,188,985** | 172,320,329 |
| W5 | 4,900,850,906 | 4,959,982,250 |
| X1 | 233,445,316 | 292,576,660 |
| X2 | 4,756,483,686 | 4,815,615,030 |

- These are source-priced evaluations at actual native atoms, not an RSS measurement. Only the unmerged combination points reuse RF's explicitly unchanged actual 648 B `Node(String,BTreeMap)` atom; no illustrative fallback is used.
- Registration binds all twelve raw reviewed inputs in order, moving only #2/#3/#4/#6 from RF, nine native reader layouts and the unchanged build identity and M.
- The D1 comment correction preserves admission code and remains bound by the admission-prefix, domain-family and duplicate B5 carrier-family pins.
- The G8 reader raw pins and the closed-test-suffix audit bind the reader source; `w1_member_load_model.py` pins the Rust reader at SHA-256 `7772f539ff2a2a2d75805a1e1a1c634b23fa56fdc63b8596954c89b067d1533e`. Format sites keep their existing STATIC64/IN128 classes, and the source/site check detects stale audit coordinates.
- **G8 early-guard TEXT.** The guard's `preview_namespace` call adds one conservative 32-member traversal through the existing B7b-ZA shared exact/preview body, pricing 32 extra `unit_by_symbol` copies at 2 × 2,173 B each: 139,072 B of `TAV_W` and `success__TAV_W` requested TEXT in each W phase, plus its upward reserve. The actual preview branch only reads keys; the full existing union charge is retained. Route E, `TAV_X`, the X phases, R and the separately rounded reserve law do not change.
- The maintained generator produces the registered 100-form, 1,584-line block, including the positive TEXT delta and its transcription checks; `generate --check` is its currentness check.
- **Line-map pruning.** The line map prunes a deleted rule by its original identity before inserting remapped keys, so a deleted key cannot overwrite or remove an unrelated live rule (`tools/retained_pricing/tests/test_linemap_prune_collision.py`).

With this registration, the follow-on epochs 4 (U4b/U5b) and 5 (O4) start with more merged dense W4 headroom than RF registered (+113,188,985 B against +35,451,012 B).

**Member-ID copies and occurrence matching (W1-MID, W1-MP).** Charged through the generic 2,914-byte `AdapterWork::copy` class, the 15,360 member-ID copies would add 89,518,080 B of TEXT, and a general direction-parser call inside member occurrence matching 58,982,400 B. The three member-ID sites therefore use a separate `copy_member_id` with the identical allocation/events/error body, source-bound to admitted IDs of at most 128 bytes; the generic copier remains. W1-MID positively prices only that copier's 128-byte ID class, and a byte/event/fault comparison control checks it against the generic copier. The member occurrence predicate uses the already-required `global_x/global_y/global_z` axis match directly; the nodal parser remains, and W1-MP binds the member-parser removal and the retained nodal parser. These rules preserve the existing budget model and do not zero a real call edge.

**What stage 2 falsifies.** Route L's credit `formed_cases = max(actual_cases − 1, 0)` and `success_findings = "max(lc-1,0)"` (`base/b7a_phase_model.py`) rested on stage 1 forcing at least one selected case to be nodal-only. It is retired: the successful formation count is all authored `lc` cases. In the design evaluation, removing it added 76,293,661 B to W4 in both modes (mostly `success__O_base`: uniform-constructor staging, typed formation-source and W2 scaled-force copies, assembled children and the recovery source copy). `MEMBER_LOAD_CAUSE` carried no credit: the producer model already prices T12–T15 for every attempted case.

**Levers: the full set (HELP_HUMAN, 2026-10-17).** HELP_HUMAN's direction: "Adopt the full set: L4, L1a, L3 and L5, plus rules 1b, 1c and 2. Each rule needs its code-fact source pin and independent review, and a lifetime test wherever one can be written." Real reductions come first. The rules are "pricing accuracy, not credits", valid only where the code facts show the lifetimes they claim.

*Real reductions* (byte-preserving source or design changes):
- **L4. Loaded-span workspace ≤ 1 MiB per attempted case, one member live.** Section 4.4: the actual backing is 702,464 B per lane, with the lanes sequential; the complete source union remains below 1 MiB. Saves 47,185,920 B against the 16 MiB cap.
- **L1a. Borrow the typed primitive copy.**
  - `FormationSource.loads` becomes a borrow (`Cow<'a, [LoadPrimitive]>` or equivalent), in place of `f.load_primitives().to_vec()` at `core/solver/nonlinear_integration/src/structural_adapter.rs` 298 (linear selected) and 567 (W2), and of PP's recovery copy at `certified_publication.rs` 16. The adapter's other production assignments at 373, 910 and 1105 adopt the same representation; only the selected D1 sites supply the priced saving.
  - The elements and their order are the same, and `Cow`'s Debug prints the slice, so outputs are bit-identical.
  - Saves 30,720,000 B.
  - Touches FK `formation_check.rs`, `structural.rs`, `structural/sparse.rs` and `certified_recovery.rs`; nonlinear integration's `structural_adapter.rs`; PP `certified_publication.rs`; the performance harness's `k6/staged.rs`; and about 30 kernel-test `FormationSource` constructions. The closure mutants use `to_mut()`; focused nonlinear-integration verification covers all callers.
  - No reviewed input moves. The tool has a source-bound check on the borrow sites. The borrow retains the priced vector header: `formation_source_borrow_is_byte_identical` asserts `Cow`/`Vec` size equality as a const check, natively and in the wasm test build.
- **L3. Move the formation Value instead of copying it.**
  - serde_json 1.0.149's `json!` expands to `to_value(&expr)` (macros.rs 279), so `finding` in `ordinary_members` (retained_wire.rs 826) and `formation` in `ordinary_entry` (1574) are deep-copied.
  - Moving them gives identical Values: there is no `arbitrary_precision`, and maps are BTreeMap. The move uses borrowed `Value::get_mut` assignments, so it makes no temporary query-key allocation; the complete Value/receipt byte control checks it.
  - Saves 18,923,890 B in every phase. Touches retained_wire.rs only, plus a receipt byte-identity test and a source-bound pricing check.
- **L5. Route-L aggregation guard** (the U6-RB analogue).
  - **Sites.** In `run_linear_static_preview_observed_inner`, `rows_by_base_id` is filled at PP `lib.rs` 2981-2986, already under U6-RB's `if preview.is_some()`, and read only by the call to `append_combination_results` at 2993. That callee (`preview_physics.rs` 854-862) clones `base_ids` and `case_rows`, which are read only inside `for combination in &model.combinations`, its last statement.
  - **The change.** One binding at the caller, `let combining = preview.is_some() && !model.combinations.is_empty();`, guards both the insert and the call (otherwise `Vec::new()`).
  - **Byte preservation.** Without combinations the loop runs zero times, so `results`, `diagnostics` and `gates` are identical whether or not the clones are made, and nothing else reads them. `tests/pressure_free_byte_identity.rs` covers both branches: `invented_demo` and `invented_preview` have combinations, `rf_skew_t_cant_off_122` has none; section 10's guard test also covers a multi-case invocation without combinations.
  - **Saving.** 32,320,674 B in the merged W phases and 30,615,138 B in the merged X phases (design evaluation); 103,046,834 B at L_30. Route E's coefficients dominate the merged tree.
  - **Pricing.** The rule names `base_ids` and `case_rows` as their own family at z ≥ 1, which is 88,526,482 B per point (design evaluation). Without it they would be covered only in aggregate by the union's Text(row) multiplicity.
  - **The U6-RB re-pin is an explicit, tested part of the change** (HELP_HUMAN, 2026-10-17). U6-RB's pricing pin (`o10_rules.py`, `if _RB_GUARDED in _agg`) and its test `u6_rb_rows_by_base_id_is_built_and_read_only_with_the_preview` must match the exact guard text. If the guard text changed without the re-pin, the pin would silently stop applying, and route E would be charged the clone again: about −131.9 MB at E_30 and merged (derived).
    - Both the pin and its test are re-pinned to L5's guard text.
    - The tool stops (exit 4), rather than skipping, when the pinned text is absent.
    - A tool test, `u6_rb_and_l5_guard_pin_binds_the_current_text`, fails on the former text and on a missing guard.

*Pricing-accuracy rules.* Each states code facts on a lifetime and gets a source pin, independent review, and a lifetime test where one can be written:
- **L1b.** The closure comparison builds one `p.closure_entry()` at a time (`formation_check.rs` 647), so one transient entry is charged, not q. Saves 18,014,760 B.
- **L1c.** `AssembledForce::force_scaled` (`load_ledger.rs` 413, via `LoadPrimitive::force_scaled`, `formation_check.rs` 296) exists only in the b ≠ 0 evaluation and is dropped when that call returns. The b = 0 evaluation borrows `f` and runs first (`nonlinear_integration/src/structural_adapter.rs` 2197-2217); the return tail holds no scaled owner. So the scaled children are charged once, not three times. Saves 27,377,664 B (design evaluation). The implementation control `force_scaled_owner_is_call_local_and_unscaled_borrows` checks the source's branch, call-local owner, return types and evaluation order. It is a lexical/ownership warrant, not a runtime Drop or allocation observation; the priced rule also pins the complete relevant source.
- **L2.** Uniform-constructor staging is sequential.
  - `add_uniform_element_loads` (PP `lib.rs` 10975) calls `equivalent_global_nodal_loads_with_spans_formed(&[global])` once per load. Its staging (`exact_scaled_intended`, `monomial_bounds`, straight_pipe `lib.rs` 1900) is local and freed on return.
  - The returned `Vec<Formation>` is consumed by `.zip` into the ledger, which is already priced as resident.
  - `certified_loads::attach_last` (`certified_loads.rs` 122-160) stages one term at a time.
  - The bound per case is one call's owners plus one term's attach staging (95,408 B), keeping the ×c convention and the moving allowance. D1 has one single-threaded call per case, and D1.4 refuses arcs, so only the straight branch is reached.
  - Saves 54,865,392 B (design evaluation). The implementation control `uniform_constructor_and_attach_staging_is_sequential` checks the synchronous single-load call, consumption and sequential attachment. This source-backed control and the pricing pins establish the lifetime warrant; it is not a runtime allocation observation.

*Not used.* Two levers need more than this unit carries:
- A shared force-scale exponent outside the primitive (about 13.7 MB) would change the K-D5 closure binding.
- One maximum helper per invocation (about 96.5 MB) would narrow the per-case convention and need a ruling.

**Independent-reader charge.** Section 2.6 adds reader-owned integer arithmetic and K4LED reconstruction. It is not covered by `G8_BYTES`: those source-identity buffers coexist with it. The additional bounded inventory is capped at 1 MiB per case, plus an equal moving allowance, charged at c = 3 in every phase: **6,291,456 B**. It includes reader arithmetic, the sidecar/view header staging and section 4.2's extra conditioning-activity/bound scratch; these owners must fit together even if they coexist. No per-case lifetime credit or budget-model change is taken. Native and wasm layouts must independently fit the cap; Python is not part of the in-product D3 profile. Exact amplification adds no new retained numeric type; all new scale/evidence arithmetic is metered under its existing owner; Boolean g-applicability scans remain outside native LME as specified in section 4.2. Actual-source D3 must enumerate the DAG's storage and TEXT rather than treating this allowance as a source rule.

At D1's 32 nodes and 128 member terms per case, even storing all twelve 144 B declared node entries per term costs 221,184 B; at most 192 net magnitudes of 128 limbs cost 196,608 B. Remaining workspace covers indices/headers/flags and bounded arithmetic/hash scratch. Streaming K4LED hashing avoids a second full encoded ledger. The 1 MiB cap includes capacity growth and the complete call chain; precise layouts, stack owners and source pins are obligations of actual-source D3. The charge deliberately applies even in phases/routes that cannot reach that reader. Member-only sidecar headers and aligned submission views (section 6.2) also fit this conservative additional allowance; actual-source D3 must enumerate them separately.

**Reader inventory beneath the cap.** The source-derived production call-chain inventory bounds encoding/staging at 333,502 B and arithmetic at 457,728 B, including a 128 KiB stack allowance; these are capacity bounds from the source, not measurements. Native full G8 runs with a 128 KiB thread stack and at most 812,710 B of widened-control heap; wasm's corresponding heap is at most 619,654 B. The wasm control's measured stack is 43,032 B, and the complete 128 KiB allowance is retained: with it, the totals are 943,782 B native and 750,726 B wasm, each below 1 MiB. All helper allocations return to zero. The existing G8 owners and the complete 6,291,456 B charge remain charged.

**TEXT and STATICS classes.**
- DEF-O, DEF-C, the table and SCHEMA are STATICS, not TEXT.
- `SELECTED_MEMBER_LOAD_MESSAGE` (493 B) exceeds SUCC_DELTA's 300 B per case and is priced as a typed delta.

**Storage requirements and a known uncounted cut.**
- The lanes accumulate per row (per-contribution storage would add about 0.88 MB per case), and the declared rows are held once per attempted case, not cloned with the source (each clone would add about 0.67 MB). Both are requirements in sections 2.5 and 4.3.
- Route E's `lib.rs` 2985 text is charged inside U6-RB's guard (2,140,416 B). It is a possible further cut of about 2.25 MB on the merged X phases, not counted.

## 10. Mutants and killing tests

Each is introduced into a scratch copy of the implementation and must be killed by the named test (new unless marked existing). Reader mutations are shared-corpus entries run through native Rust, wasm (the R1 records) and the Python verifier.

**Kernel: the declared load (sections 2 and 3).**

| Fault | Killed by | Control |
|---|---|---|
| Force or moment sign at j; w × d instead of d × w; wrong scale | `declared_loads_match_python_bit_for_bit`; `member_load_cantilever_tip` | cantilever wL⁴/8EI; fixed-fixed wL²/12 and wL²/24; simply supported θ = wL³/24EI; skew versions |
| d rounded; formed at p, q, 2p or q_W; rounded to binary64; wrong rounding order; ties not to even | the bit vectors; `member_load_residual_reads_the_declared_ledger` | exponent-spread, d-rounding, rounding-order and tie controls |
| Declared entries omitted from rhs or residual; d taken as \|n\| + \|d\|; counted in m_i | the residual test; an m_i/d accessor test | a nodal and a member load at one DOF |
| Omitted from reactions (recover or lanes) | `member_load_fixed_fixed_reactions` | all DOFs fixed |
| Omitted from E | `member_load_scale_dominates_rows` (E_q ≥ \|q_P\|); an E-UNIT vector | a member with u = 0, so A = −f_eq |
| Ŵ includes member-load terms | `member_load_correction_recovery_is_linear` (`recover(empty, 0)` = 0) | cantilever |
| 7a ignores declared entries | `member_load_data_flag_counts_declared_entries` | a cantilever with only a member load; a cancelling pair |
| Lanes omit the declared points | lane hooks (like `hooks::representative`) | cantilever |
| K4SRC drops or reorders member terms; nodal-only K4SRC bytes move | encoding tests; the existing K4 vectors unchanged | — |
| Partial span constructible | type-level: `UniformMemberLoad` has no extent (compile-fail doc test) | — |

**Kernel: recovery and the maximum (section 4).**

| Fault | Killed by | Control |
|---|---|---|
| Recovery omits f_eq; station signs as on 2026-10-12; global w in local slots; t ↔ 1 − t; a load term in torsion | `member_load_station_signs_are_j_side`, plus end-action and torsion tests | simply supported at t = ¼ and ½; skew cantilever |
| E omits frame-conditioning amplification; g-check skips an active fixed member | `member_load_recovery_error_is_bounded_by_69uE`; `member_load_g_check_covers_fixed_and_cancelling_sources` | independent exact/directed full-DAG error control at P = 128/256/512/1024; near-parallel counterexample, ordinary frames, g_exp = P−16 and above; fully fixed, zero/cancelling sources and None/empty-ledger cases |
| Rounded j-end used as an interior force base; endpoint bit identity merely assumed | `member_station_single_round_and_endpoint_bits` | p = 128 rounding-boundary shear/load control; exact endpoint dispatch, interior single expansion; unloaded pins unchanged |
| Two-end recipe on a loaded member; q dropped | `w1_loaded_maximum_interior_controls_certify` | simply supported (wL²/8)/Z; end moments with a non-dyadic argmax |
| q sign or axis flipped | `w1_loaded_maximum_bubble_sign_and_axes` | non-dyadic argmax; skew with two load axes |
| Station orientation reversed | `w1_loaded_maximum_station_orientation` | argmax 0.59915 against 0.40085 |
| Lower endpoint above the true maximum | `w1_loaded_maximum_enclosure_contains_independent_reference` | exact rational check, plus 200 random members against a 2048-bit reference |
| Exhaustion narrows or permits publication through a passing absolute gate | `w1_loaded_maximum_exhaustion_widens_and_refuses` (capacity 4 / depth 6 hooks) | diagnostic contains reference; named exhaustion makes case unavailable, including an absolute-class interval that would otherwise pass the gate |
| Box-corner bound used; tangent bound dropped | `w1_loaded_maximum_rotating_and_axial_slope_within_capacity` (≤ 64 splits, ≤ 32 live); the near-degenerate control at e = 1e-5 | — |
| PP's value used for a loaded member (it fails the sharper predicate) | `w1_loaded_maximum_is_the_kernel_projection` | non-dyadic argmax: row bits equal the kernel's RN64(H(hull)); PP's value substituted fails SharperExact |
| Kernel projection applied to an unloaded member (moves census bytes) | `w1_unloaded_maxima_keep_the_prepared_value` | section 4.6 census: all 43 inputs' maxima/publications unchanged with only authorized definition-bound receipt hashes re-pinned; 54 trials' standing unchanged; milestone M1 stays 8.17e-92 Pa |
| Asymmetric loaded evidence interval (for example RD64(lo)/RU64(hi)) | `w1_loaded_maximum_evidence_is_symmetric_and_g7_valid` | lower = n − δ, upper = n + δ exactly; G7 `EXTREMA_BOUNDS` and PP's midpoint check pass; the interval contains both the kernel hull and PP's bounder interval |
| Symmetric search restricted to powers of two; overflowed 2n; half rounded at an odd subnormal | `w1_loaded_maximum_evidence_covers_binade_and_subnormal_edges` | n = 0x1.fffffffffffffp-1 needs δ = 3·2^-53; odd subnormals 1, 3, 5 ulps and odd low-normal halves; largest finite endpoint; exact integer grid oracle |
| Near-zero form skipped | `w1_loaded_near_zero_maximum_takes_the_half_upper_form` | a loaded member whose enclosure reaches 0: value upper/2, evidence [0, upper], G7 valid, absolute gate decides |
| PP's bounder run without the member's spans for a loaded member | `w1_loaded_maximum_bounder_receives_the_member_spans` | the bounder interval contains the interior maximum (simply supported span) |
| PP witness upper mistaken for global maximum upper | `w1_loaded_evidence_contains_pp_global_upper` | non-dyadic interior maximum with `value_upper < upper_bound`; evidence must contain `upper_bound` |
| Route E's maxima moved | `w1e_maxima_keep_the_prepared_value` (existing route-E successor pins byte-identical) | — |
| Combination with a member-load operand selected | `combination_with_member_load_operand_is_refused_by_name`; ledger backstop test | — |

**Kernel: work (section 5).**

| Fault | Killed by | Control |
|---|---|---|
| Formation work zero, re-formed per attempt, or unguarded | `member_load_formation_is_charged_once_in_the_first_rhs`; `member_load_formation_refuses_by_budget_at_a_member_checkpoint` | a case limit set between the two totals |
| Refused checkpoint work discarded, counted twice, or followed by a solve/publication | `member_load_checkpoint_equality_and_exhaustion_are_metered_once`; `member_load_netting_checkpoint_stops_before_solve` | case/invocation room at exact work and ±1; stage/case/invocation totals include refused member/netting; retry does not re-form; no later checkpoint or selected rows |
| SumWork or canonical-net extraction omitted from overrun bound | `member_load_d1_cap_checkpoint_work_is_bounded` | maximal 128-load member and 192-DOF netting; prescribed three-scan canonical accessor; independently counted actual work ≤ 2,365,094 / 786,432 LME |
| Maximum evidence built after finalization accounting closes | `loaded_maximum_evidence_work_is_counted_before_completion` | endpoint-grid success and failure; no unmetered helper; final certificate failure keeps its work |
| Maximum search work zero | `w1_loaded_maximum_work_is_counted` | — |

**Producer and readers (section 6).**

| Fault | Killed by |
|---|---|
| Member term with changed bits, unnormalized kN/m, wrong member index or occurrence-based ordinal | `w1_member_terms_bind_the_normalized_authored_occurrence`, `w1_member_term_ordinal_is_the_authored_index`; corpus `member_term_{bits,member,ordinal}_changed` |
| Term dropped, duplicated or reordered | `w1_unconsumed_or_unauthored_member_occurrence_refuses`; corpus `member_term_{dropped,duplicated}`, `member_terms_reordered` |
| Stage-1 skip of the early raw check kept | extend `b4_early_captured_binding_refuses_changed_raw_context_and_normalizes_units` (existing) to element leaves |
| Late hook ignores member terms | rewrite `b7a_member_load_case_late_hook_re_observes_its_member_loads` (existing) |
| Preparation drops member loads | `w1_prepared_source_keeps_its_member_loads`; reader G8 |
| Self-weight id admitted | `every_family_clause_refuses_with_its_fact` (existing); corpus `invocation_member_load_self_weight_id` |
| Element load admitted on route E | `b7a_d1_exact_route_keeps_member_loads_and_selectors_out` (existing); `b3b_exact_capture_refuses_a_member_load`; Rust/Python `g8_exact_route_refuses_an_element_load` |
| Rotational, non-force-per-length or non-global element load | the existing law test; corpus `invocation_member_load_{direction_ux,dimension_force,category_weight}` |
| Nodal-only bytes moved (an empty-list event, `member_load_terms: []`, an empty K4SRC block, the member-load diagnostic on a nodal case) | re-pinned nodal successors with a field-masked diff; corpus `member_load_terms_{empty,on_nodal_source}`, `member_load_message_on_nodal_case` |
| New vectors enlarge adapter-reserved nodal layouts | `member_sidecars_preserve_nodal_adapter_capacity_bytes` | size/layout pins and full capacity-counter bytes for single-case early capture, multi-case parking/submission, reused basis and route E; only DEF-O-bound hashes may be masked |
| Early member guard shadows established legacy/nodal gates or a dropped source key bypasses the bound | `b3a_legacy_pressure_contract_namespace_at_g8`; `b3a_malformed_invocation_keeps_existing_gate_precedence`; `member_early_guard_preserves_old_namespace_and_bounds_source_forgeries`; `full_g8_rejects_mixed_overcap_before_any_hash_or_json_staging`: unchanged legacy 29 controls, public malformed 9-shape × 2-base matrix, source-key null/empty and omitted-key malformed member candidates; native/wasm/Python |
| K4SRC member block omitted or wrong | genuine member-load must-pass bases; corpus `kernel_source_without_member_block` |
| Data rule missing in a reader | corpus `member_load_body_has_data_false` |
| Constrained rotations or cancelled nets misclassify member data | `member_reader_data_uses_free_individual_entries` | all induced moments constrained; cancelling nonzero member pair; mixed nodal/member cancellation |
| Declared problem attested without independent recomputation | `selected_member_ledger_is_recomputed_in_rust_wasm_and_python`; corpus `member_ledger_hashes_changed_together` refreshes all equal hashes and the outer receipt hash and requires G8 `PREPARATION_MISMATCH` in all three validators |
| Reader RN-even, square root, D5 order/sign, cancellation or zero-occurrence encoding wrong | `reader_declared_entries_match_independent_bit_vectors`; `reader_member_k4led_keeps_cancelled_and_zero_dofs`; genuine member-load bases pass native/wasm/Python, while nodal-only verdict pins stay unchanged |
| Reader arithmetic allocation or stack bound omitted | `member_reader_workspace_fits_priced_cap_native_and_wasm`; layout/call-chain source pins and cap-maximal reader tests |
| Unbound member result treated as invocation-validated; missing nodal term or arithmetic capacity silently truncated | `member_reader_unbound_keeps_needs_recompute`; corpus `member_ledger_nodal_term_omitted`, `member_ledger_zero_dof_dropped`; bounded arithmetic exhaustion hooks refuse in Rust/wasm/Python |
| Combination selected with a member-load operand; spurious `operand_member_loads` | `b2p_member_load_operand_combination_is_unavailable_by_name`; corpus `combination_with_member_load_operand_selected`, `operand_member_loads_without_member_operand` |
| Disclosure missing (DEF-O, DEF-C, table, diagnostic) | `def_o_def_c_and_table_state_the_declared_member_load` (Rust and Python); corpus `member_load_case_selected_message_missing`; the G0 hash pins |
| Schema left open | corpus `member_term_extra_field` (G1) |

**Levers (section 9).**

| Fault | Killed by |
|---|---|
| L1a changes an output (a reordered or rebuilt primitive) | `formation_source_borrow_is_byte_identical` (K-D5 closure and receipt bytes against the pre-lever pins); the existing closure mutants via `to_mut()` |
| L3 changes a receipt byte | `ordinary_formation_value_move_is_byte_identical` (text and hash against the former construction, as for U6-DH) |
| L5 skips a clone that combinations read | `append_combination_results_guard_keeps_combination_bytes` (byte-identical invocations with combinations, without them, and multi-case without them) |
| L5 changes the guard text without re-pinning U6-RB (route E silently charged the clone again, about −132 MB) | `u6_rb_and_l5_guard_pin_binds_the_current_text` (tool test: fails on the former text and on a missing guard); the tool exits 4 when the pinned text is absent; the re-pinned `u6_rb_rows_by_base_id_is_built_and_read_only_with_the_preview` |
| A pricing rule's lifetime premise is false | the rules' source pins (the tool stops when a pinned function changes); for L1c, `force_scaled_owner_drops_at_the_scaled_evaluation_return` proves none exists in b = 0 and the b ≠ 0 owner/children drop before the caller's return tail; for L2, at most one call's staging is live |

**Census.** `w1_source_census_tests` pins the other case's status and cause for F6/partial, T18 and T19 in both modes. Case 0 stays selected.

## 11. Rulings

**Applied** (HELP_HUMAN, 2026-10-17):
- **Option B, the declared member load.** Sections 2 and 3. The proven bound 2^-1022 per entry supersedes "about 2^-1019".
- **Disclosure placement.** DEF-O/DEF-C, table #4's limitation, and the selected member-load diagnostic (section 6.4).
- **Lever policy.** Real reductions first; rules 1b, 1c and 2 as pricing accuracy, with pins, review and tests.
- **DEF-O hash moves** under the E14 precedent.
- **Route-L maxima, option 1:** one recipe with two cases, following where the maximum is determined (section 4.4). It comes with the loaded-member evidence construction, the near-zero form, G7 shown by construction, the four mutants, and independent review of the evidence construction and the census.
- **The census:** section 4.6.
- **The full lever set**, with the U6-RB re-pin to L5's guard text as an explicit, tested part of the change (section 9).
- **RF's interface note:** `load_reference_fallback_uz`'s dependency on `prepare_sources`' non-nodal decline (section 8).
- **R-1, W1 checkpoint timing** (HELP_HUMAN, 2026-10-17): preserve actual-cost counters and checks after each member and after netting; count refused work once in every containing total, stop on exhaustion, and keep limits unchanged (sections 5 and 8). RF's pre-step policy is specific to `exact_boundary::Context`.
- **Frame-conditioning repair** (HELP_HUMAN, 2026-10-17): within the settled declared-load ruling, amplify new frame-dependent recovery operands and extend the existing g-check's applicability to exact nonzero member-source activity, including fully fixed members. Factor 69, P−16, prior nodal-only applicability and the verification law stay unchanged. Ledger inclusion proves 7a/Ŵ omission, not E's formation-error theorem; the separate full-DAG derivation/checks of section 4.2 must establish that theorem.

**Decided in this design**, for the independent review (section 12) to check:
- per-load granularity of the declared entries (2.2);
- d never formed, and L̂ from the exact S (2.2);
- recovery from the member-local consistent load (4.1);
- S* without a load term (3);
- the min(paired, tangent) bound with exhaustion failing closed (4.4);
- `member_load_terms` optional and K4SRC's block conditional (2.1, 6.3);
- independent Rust-reader and Python-verifier recomputation and K4LED binding for selected member-load cases, with the nodal-only path preserved (2.6);
- the combination refusal's position (4.5).

**Needed:** no further design ruling is requested. Section 4.2's derivation establishes the corrected E DAG's warrant under unchanged factor 69 and P−16. If a change cannot preserve that derivation, return the concrete conflict to HELP_HUMAN; do not change the law to obtain a pass.

**Reserved and untouched:**
- D1's limits and the domain;
- every ScopeOfWork commitment, acceptance criterion, frozen reference, criterion and tolerance (including the product gate's predicates);
- decision 22 (M, the 0.9M margin, R, the 5 % reserve and the budget model);
- b7-plan's "Held by the owner";
- B8 and the frozen UI benchmark;
- desktop outputs for the exact identity.

**G10** stays owner-held. Stage 2 moves the milestone's retained-successor bytes only in the DEF-O-bound hashes; its maxima do not move (section 4.6). The milestone's ordinary bytes do not move either. Whether G10 is witnessed before or after is the owner's choice.

## 12. Tests and the independent numerical review

**Tests:**
- the census of the twelve trials in both modes (case 0 retained; the other case's status and cause pinned);
- D1 law tests for every admitted and refused form;
- custody at c = 2 and 3 with a member load on a selector case, on either case, on both cases, and with a reused basis;
- kernel controls against the vetted closed forms of section 7, axis-aligned and skew;
- the Python bit oracle;
- the loaded-maximum controls;
- the mutants of section 10;
- the regenerated law and witness tests at a cap-maximal widened input (three cases, three bases, 128 member loads each);
- the maximum-projection census (4.6);
- ordinary and route-E byte preservation;
- the lever byte-identity tests.

**Independent numerical review checklist.** The review is by agents separate from the implementer. It must check:

1. **The declared recipe D1–D5:**
   - its sign conventions (node i and j; the d × w orientation) against the closed forms;
   - the full specification of every rounding (to nearest, ties to even, at 1024 bits);
   - the independence of the Python oracle, Rust reader and Python verifier from the producer and from each other's arithmetic; their bit-exact entries, complete K4LED and selected-case digest binding; native/wasm/Python behavior on genuine and rehashed mutant member cases; the unchanged nodal-only path.
2. **The disclosed bound.** That 2^-1022 per entry holds, and that DEF-O, DEF-C, the table limitation and the selected member-load diagnostic state it.
3. **"Exactly like the ledger"** for rhs, the residual and m_i (not counted), d and d^b (|net|), R7's r and r₂ at q_W, t₁ (no term), E, Ŵ (the empty ledger omits every member-load term, and `recover` reads them only through the ledger), the 7a data flag (a nonzero entry sets the flag even when the net cancels), the uniqueness warrant, and the lanes' free rows and reactions. If any of 7a, Ŵ or E does not hold, the unit returns to HELP_HUMAN.
4. **Recovery.** That u/reactions come from f_d and end actions/stations from the disclosed member-local affine recovery; check the corrected end and station DAG, frame-conditioned E load terms, exact-source g-check activity (fully fixed and cancelling terms included), unchanged factor-69 error theorem under P−16, endpoint branches and interior single rounding, lanes and unloaded/None/empty-ledger/nodal-only exclusions. A separate review derives the actual full-DAG error bound, not merely magnitude domination.
5. **S* without a load term.** The free-body and coupling argument.
6. **The maximum row on route L:**
   - the two cases defined by where the maximum is determined;
   - for a loaded member: the field (N linear; M quadratic with q); that each node bound is an upper bound under directed rounding; the lower endpoint, the stop rule and the limits; exhaustion widening and failing closed; and the RN64(H(hull)) value;
   - the evidence construction: the least exact symmetric interval containing the kernel hull and PP's bounder interval, and the near-zero form, with G7's unchanged identity, basis strings and lower ≥ 0 holding by construction (HELP_HUMAN's condition);
   - for an unloaded member, PP's value and evidence unchanged;
   - route E's bytes unchanged;
   - the census of 4.6 on the implementation's head: all 43 committed inputs' maximum/publication/ordinary bytes reproduce, only authorized definition-bound receipt hashes are re-pinned, and the 54 trials, including the 16 named there, keep their standing.
7. **Combinations refused** in the kernel, the ledger backstop and both readers.
8. **Work.** Counted once at actual cost in the first own `rhs`; guard checks per member; the maximum search in `NumericWork`; limits unchanged; nothing counted as zero.
   - R-1's post-member and post-netting checkpoints, equality/exhaustion boundary controls, all refused work retained once in stage/case/invocation totals, no execution/publication after exhaustion, and an independently checked overrun bound including `SumWork`.
9. **Ranges.** No refusal in formation or netting (span ≤ 7,325 < 8,128 bits); later `Span` refusals are the existing terminal refusal.
10. **Custody.** Per-case custody under basis reuse with member loads.
11. **Pricing:**
    - every new rule (the stage-2 owners, L1b, L1c, L2, L5's family) argued from code facts with a source pin;
    - L1a's, L3's and L5's byte preservation;
    - the U6-RB pin re-pinned to L5's guard text, with its test, so route E cannot silently be charged the clone;
    - the lifetime tests of L1c and L2;
    - the independent reader's native/wasm arithmetic and complete call-chain transient cap, including capacity growth, canonical hashing and its moving allowance;
    - the credit removal;
    - the actual-source D3 registration.

## 13. Verification and maintained controls

**Checks** for a change to this unit:
- the mutants of section 10;
- the suites `select_affected_checks.py` selects (frame kernel, product physics including the census and facade tests, result_export, headless, the Python verifier tests, and desktop test support);
- the corpus in native Rust, wasm and Python;
- the hosted `harness` on the exact head.

Independent scrutiny is required for a change to numerical meaning: the checklist of section 12, by agents separate from the implementer.

**Kernel controls.**
- **Recovery error.** `member_load_recovery_error_is_bounded_by_69uE` checks actual formation error against the factor-69 bound on the 40 maintained recovery vectors (`member_recovery.txt`): 36 cases / 1,512 rows assert the error bound, and the other four cases / 168 rows check above-threshold g applicability and do not assert it. Full-path g-boundary tests, lane/reaction/residual/data-flag controls and declaration-once escalation controls accompany it.
- **Maximum.** The controls include 200 general noncollinear 2048-bit references formed by the maintained exact-polynomial writer (`member_maximum.txt`), 200 exact-rational controls, the rotating `9/8` case, station orientation, unequal area/section-modulus divisors, and the unchanged-cap near-degenerate control. Identically zero axial force satisfies both sign tests; the tangent bound gives the positive sign precedence, so that case takes the s = +1 tangent branch rather than skipping both, and it certifies at the unchanged 256-live / 64-depth / 1,024-split limits.
- **Controls that kill specific mutants.** The declared vector `d_rounding_halfway_1024` (`declared.txt`) distinguishes premature chord rounding; a direct RHS consumer control detects an omitted declared RHS even when residual refinement would repair the final solve; and the general noncollinear reference detects a load-bubble sign flip hidden by zero-end norm symmetry.
- **PP caller mutants.** Omitting the member spans, or replacing the loaded kernel projection with PP's representative, fails unchanged `SharperExact`; substituting `maximum.value_upper` for the required global `maximum.upper_bound` fails the complete-global-interval assertion (section 10's rows). They are runtime failures, with the production preimages restored byte for byte; no reference, threshold or tolerance changes.

**Reader controls.**
- The consuming readers implement source-aware maximum coverage, semantically bound exhaustion diagnostics, explicit endpoint encoding checks, and member-only preallocation guards before G8 staging. The native, wasm and Python consumers read all 16 independent bit vectors, the positive and search-exhaustion successors (the latter hook-driven since the W1 coverage repair, w1-coverage-repair.md §1.4) and the active-member Values-completion edge in both modes. Refreshed-hash ledger forgeries reject at G8; diagnostic and merged-work mutants reject at G2/G5.
- The production admission guard and exact occurrence comparisons establish the joint 128-term and 128-byte-ID bounds before encoding; the helper's wider standalone input domain is not substituted for that proof. Section 9 gives the source-derived inventory beneath the 1 MiB cap.
- **Early guard scope** (section 2.6). Authored applicability is limited to the new family, because treating every element occurrence as a member candidate would let the new bound shadow the old invocation/namespace refusal; source-key forgery coverage is retained. Python's early guard uses borrowed field/list/text views, so null primitive lists, null primitive entries and null targets cannot shadow the old hash gate, matching Rust; malformed request/model/roster shapes still fail earlier at G3. The unchanged legacy 29 controls and the additive public 18-input matrix (section 10) run in native Rust, Python and wasm.
- **Python combination witness.** It checks the existing reason/cause types before inspecting the member-only tag, because W-CB4a legitimately retains a string refusal reason. It changes no expected verdict or declared arithmetic.
- **Optional route in the Python verifier.** `_g5_products(body, rows)` keeps its documented `route=None` behavior: one correction per lane and no route-specific member extensions. Member Values-completion classification and `_owner_projected`'s loaded-maximum extension require a present route before reading `exact`. The two original D19/D30 acceptance/refusal tests are unchanged, and `tests/test_declared_member_load_reader.py` covers an active loaded maximum with explicit `None` beside the preview/exact/cancelling/zero controls. Public validation resolves a concrete route at G0, including unbound reads, so the member ledger, completion and diagnostic predicates do not depend on that default.

**Corpus.** The corpus has 61 bases, 640 mutations and 108 explicit positives (809 entries), including the four registered Direct interior controls of section 4.4. The headless reader test asserts 61 bases and the browser corpus test 61/640/108; the original 45 bases, 599 mutations and 90 must-pass entries remain exact prefixes. Stage 2's regeneration keeps every original R1 per-reading outcome hash and complete summary; its input digests move only for the authorized definition-bound hash changes. Historical original entries and byte-preserved exact-route fixtures keep their verdicts and explicit provenance.

**Cap-law controls.**
- `w1_three_basis_member_cap_input_is_inside_d1_and_refuses_129th_term`: the additive cap-law input preserves 32 nodes/members/supports, 4+4 materials and 16-point rosters, and replaces its leaves with **three cases, three named bases and 128 member terms per case**. Admission in both modes accepts all 384 terms and refuses the 129th term in a case under unchanged D1. Its output reports the profile state: `Registered` in the registered build and `Stale` in an emulated-unregistered one.
- The 4 MiB stack witnesses `witness_w1_member_load_three_case_three_basis_128` and `witness_w1_member_load_three_case_three_basis_128_dyadic` (each sparse and dense, `#[ignore]`, run as separate cap commands) use the narrow two-node/one-member geometry and do not claim all geometry caps. The first reaches staged selection in all three cases. Its body-0 translation and rotation scales differ by one ulp between the native candidate and the final rows, so it retained a G5b scale refusal until the W1 coverage repair's corner 2; as built it publishes, and its staged bytes pass the full Rust reader in both modes. The dyadic positive uses E/G = 2^30/2^29 times [1,2,4] and per-term axial intensity `E_case·Â·2^-28` on the length-2 member: the ideal tip displacement is exactly 2^-20 m, avoiding scale halfways and unit-roundtrip loss. In both modes it publishes through the actual private W1 stack path and passes invocation-bound native G0–G8 for all three selected cases/384 member terms. Its private witness provenance is not a public permit claim.
- The maintained annulus helper chooses only this invented test input; it is not imported into independent reader arithmetic. Original nodal input helpers and hash pins remain unchanged.

## 14. Blast radius

**Code**, by owning deliverable:
- **Frame kernel (DEL-04-01).**
  - The retained kernel: `structural/retained/` `source.rs`, `assemble.rs`, `ledger.rs`, `adaptive.rs`, `verify.rs`, `bound.rs`, `recover.rs`, `combine.rs`, `wide_sum.rs` (the netting accessor), `product_certificate/source_residual.rs` and `final_case.rs` (the loaded maximum search), and `retained_resource.rs` (layouts).
  - The `retained_api` exports in `structural.rs`.
  - Lever L1a: `formation_check.rs`, `structural.rs`, `structural/sparse.rs` and `certified_recovery.rs`.
  - The tests in `tests/retained_k4/`, including `gen_k4_vectors.py`, and the S11 site table.
- **Product physics (DEL-04-07):**
  - `retained_product.rs`: capture, preparation, maxima, and the stage-1 removals;
  - `retained_wire.rs`: terms, the combination reason, the diagnostic, and L3;
  - `retained_memory.rs`: the D1 comment and the registered profile;
  - `lib.rs`: L5's caller guard;
  - `certified_publication.rs` (L1a);
  - the tests in `w1_domain_stage1_tests.rs`, `w1_source_census_tests.rs`, `retained_facade_tests.rs`, `retained_wire_tests.rs`, `retained_memory_law_tests.rs`, `s11f_site_test.rs` and `pressure_free_byte_identity.rs`.
- **Nonlinear integration:** `core/solver/nonlinear_integration/src/structural_adapter.rs` (all L1a assignments and its focused verification).
- **Readers (DEL-05-02):** `result_export/src/retained_precision.rs` (including independent bounded integer/dyadic arithmetic and streaming K4LED encoder, shared by native/wasm) and `semantic_contract.rs` (pins), `tests/verifiers/retained_precision.py` (independent integer/Fraction recomputation) and `compatibility.py`, `core/analysis_runs/compatibility.py` (pins), and the Python contract and schema tests.
- **Desktop:** test support only, as listed in section 6.5.
- **Performance harness:** the k6 W1 adapter (L1a).

**Reviewed inputs.**
- Moving: #2 SCHEMA, #3 DEF-O, #4 the preview-retained table, and #6 DEF-C.
- Unchanged: #0 `Cargo.lock` (no new dependency), #1 (RF's), #5 the preview-physics-1 table, #7 DEF-E, and #8–#11.

**Priced layouts.**
- New: `UniformMemberLoad`, `MemberTermIdentity`, `MemberLoadSet`, the composite source/submission views and invocation-owned member sidecars, the loaded-maximum search workspace, and the reader's bounded integer/declared-entry/net/hash workspace with native/wasm layouts and moving allowance.
- Changed: `RetainedLedger` (declared entries in the nets), `CasePrep` (the formed-ledger slot and member-sidecar association), `MemberOperators` (+`length`), invocation-level `ProductCapture`, `FormationSource` (L1a's borrow), and receipt/reader transients. `SourceParts`, `PrimitiveSource`, `CaseSlot`, `EarlyCaseSource` and their adapter-reserved option layouts stay unchanged.

**TEXT.**
- Added: the new capture refusal texts and member-term keys; the `operand_member_loads` tag; `SELECTED_MEMBER_LOAD_MESSAGE` (a typed delta); and the kernel's exhaustion and member-load causes.
- Removed: `MEMBER_LOAD_CAUSE`.

**Fixtures and corpora**, each regenerated by its own writer:
- the corpus `fixtures/results/retained_precision_cases.json` (`tools/fixtures/retained_precision_corpus.py` expand/pack). The DEF-O-bound hashes move in every base, and new genuine member-load bases are added (a T18-style selected member case, a kN/m multi-member case, and a member-load-operand combination, in both modes) together with section 10's mutations;
- the preview successor fixtures (`I105_B2P_OUT`, `B4_FAILED_ORDINARY_FIXTURE_OUT`, `I76_T6S2_OUT`, the milestone, L0 and W-C2 pins, the carriers and the derivative goldens);
- the R1 reader records (`RETAINED_READER_OUTCOMES_OUT`) and the desktop WASM equivalence inputs;
- the K4 vectors, with the new `declared.txt`;
- the hash pins: the readers, `semantic_contract.rs` 123, `retained_memory_law_tests.rs` 738, both `compatibility.py`, `test_retained_precision_schema.py` 98, the table-hash constants in `analysis_run.v0.3.schema.json` (262, 301, 839, 854) and `stress_neutral_export.v0.3.schema.json` (3879), the DEF-C pin `retained_facade_tests.rs` 4018, and the desktop test support;
- the registered profile.

**Must stay byte-identical:**
- every ordinary and pressure-free byte, and G10's ordinary bytes;
- every route-E successor, RV120 input and other route-E fixture;
- the J2 corpus and the J7 RF census pin;
- the R2 base-reader outcomes;
- every committed maximum (section 4.6);
- in nodal-only receipts, everything except the DEF-O-bound hashes;
- `validation/references/**`, the frozen UI benchmark and DEF-E.

**Pricing tool and rules:**
- `tools/retained_pricing`:
  - `base/b7a_phase_model.py` (the credit);
  - the `base/b7a_domain_model.py` pins;
  - `producer_caps.py`, `g4_caps.py` and `g5_profile.py` (the owners);
  - `rules/o10_rules.py` (the L5 rule, the U6-RB re-pin, L1b, L1c and L2);
  - its tests.
- New labels and their arguments in `retained-pricing-rules.md`.

**Deliverables.**
- Affected: DEL-05-02 (this design, the readers and the pricing tool), DEL-04-07 (PP) and DEL-04-01 (the frame kernel, including the retained kernel and L1a).
- `tools.deliverables impact DEL-05-02` also lists DEL-07-03 and DEL-11-01. This unit changes no editor or guide surface.
