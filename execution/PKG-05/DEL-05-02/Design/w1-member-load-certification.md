# W1 member-load certification unit (group 3 stage 2)

**Status: design, not implemented.** No product code, test, fixture, schema, table, generator or reference has changed. The basis is main `4971d62dc911f11c6d9867a9c2fe85e180d43006` (B7b, T4-U6/U8 merged as #1392). The RF source-method extension is designed ([rf-source-method-extension.md](rf-source-method-extension.md)) and being implemented on `codex/piping-rf-source-method-20261017`.

This is the third post-T3 shared epoch in [b7-plan.md](b7-plan.md) ("After B7b", item 3). It lands after RF, from merged main, under its own independent review. Its implementation starts from RF's merged head and re-checks section 8 there. This file replaces b7-plan's four 2026-10-12 "Member loads" paragraphs and the stage-2 part of its "Size, tests and review". Section 1 re-checks each of their premises.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

**Commission.** b7-plan, "Staging" (T3, 2026-10-12, under the owner's "(b) Widen domain" decision): "Stage 2 is the follow-on unit **W1 member-load certification** … It needs its own go and independent numerical review." HELP_HUMAN accepted the staging on 2026-10-12. The owner's resumption steer of 2026-10-14 orders the shared epochs: "in this order from merged main …, each under its own design and independent review".

## Summary

- **What it does.** On W1 route L (the retained preview route: pressure-free, Euler–Bernoulli), a case whose loads include full-span uniform `distributed_force` member loads builds a retained source and can be certified. Today such a case publishes `unavailable` with stage 1's interim `MEMBER_LOAD_CAUSE`. In the census trials F6/partial, T18 and T19, the other case gains its own retained status. It is `selected` when its certificate passes G0–G8, and otherwise `unavailable` with a named kernel or facade cause. Case 0 stays selected, as stage 1 established.
- **Settled ruling: option B** (HELP_HUMAN, 2026-10-17). The member load is a declared exact value at 1024 bits, formed once and carried in the exact ledger. R7's law does not change. The certificate certifies the declared-load problem. Its difference from the exact consistent load is at most 2^-1022 relative per declared entry, and that bound is disclosed (section 2).
- **What changed against 2026-10-12:**
  - f_w is no longer re-formed at p, q, 2p or q_W. It is declared once.
  - m_i and d_i^b take no counted load term.
  - S* takes no load term.
  - The station terms change sign to W1's j-side convention.
  - The lanes add exact points, not load enclosures.
  - PP no longer computes route L's published maximum. The kernel projects it from its certified enclosure (section 4.4).
  - The loaded-span search uses a quadratically convergent bound.
- **Memory.** Stage 2 does not fit by itself. It falsifies a stage-1 pricing credit worth 76.3 MB in W4 (measured). With RF's stated ~8 MB, merged dense W4 then goes from +51.8 MB to −32.5 MB. Byte-preserving levers close the gap: real reductions first, then pricing-accuracy rules on lifetimes that the code facts show. Section 9 gives the per-phase figures and their labels. Decision 22 is unchanged: M, the 0.9M margin, R, the 5 % TEXT reserve and the budget model stay as they are.
- **No change** to any reference, criterion, tolerance, D1 limit, work limit, frozen benchmark, DEF-E, route E, the model format or ordinary bytes.

## 1. Premise re-check (2026-10-12 design on main `4971d62dc9`)

Each premise is marked HOLDS, CHANGED or RETIRED, with its current evidence. Paths are relative to `projects/chirality-piping/`. The kernel is `core/solver/frame_kernel/src/structural/retained/` (written `retained/`). PP is `core/product_physics/src/`.

### The represented source equation

| # | 2026-10-12 premise | Verdict | Evidence on main | Consequence |
|---|---|---|---|---|
| P1 | `PrimitiveSource` gains `member_loads: Vec<UniformMemberLoad { member, intensity: [f64; 3], source_id }>` (global N/m), validated finite with the member present, sorted by member, source id and bits | HOLDS | `SourceParts` and `StraightMember` (`retained/source.rs` 81-93, 164-175, 270-287); load validation 596-604; nodal sort 605-611 | Section 2.1 |
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
| P18 | `ProductRecipe::CircularMaximum` keeps its two-end recipe for an unloaded member | HOLDS for the enclosure | final_case.rs 783-800 | Unchanged |
| P19 | A loaded member's maximum is enclosed by a fixed-capacity subdivision; lower endpoint at the published governing station; the direct certificate applies its unchanged predicates | CHANGED | A box-corner bound overestimates by O(h) and exhausts 256 nodes on skew and axial-slope controls (section 4.4). Using PP's station leaves the enclosure 212 to 10,686 × 2^-53 wide | Branch-and-bound with a quadratically convergent bound; lower endpoint at the search's best point |
| P20 | PP's `prepared_maxima` passes the member's local spanned loads to `exact_straight_summary_extrema` in place of `&[]`; the published value is PP's | RETIRED | Call at retained_product.rs 5637 (signature gained `pressure_state` in U6, PP `lib.rs` 10809-10815). PP's value fails the unchanged sharper predicate on almost every loaded control (section 4.4). The engine is shared with ordinary callers (PP `lib.rs` 5764, 6035; `membrane_publication_range.rs` 172/248) | On route L the kernel projects every `CircularMaximum` row; PP regenerates the evidence from the kernel's certificate |
| P21 | A combination with a member-load operand is unavailable by name until DEF-C's combined-ledger law is extended | HOLDS | `combine.rs::validate_preps` 233-252; `RetainedLedger::combined` (ledger.rs 209-229) reads only nodal loads | Refused in the kernel, with a backstop in the ledger (section 4.5) |

### Producer, definitions and readers

| # | 2026-10-12 premise | Verdict | Evidence on main | Consequence |
|---|---|---|---|---|
| P22 | `capture_case_source` no longer fails "unsupported producer present" for `element_uniform_loads` | CHANGED | Route L already diverts to `capture_member_load_case` (retained_product.rs 1785-1834); the flag at 1790 now matters only on route E | The flag becomes "route E with element loads, or imposed displacements" (section 6.2) |
| P23 | A new `capture_case_member_terms` captures each normalized element occurrence, as `check_captured_nodal_leaves` does for nodal terms, into `SourceParts.member_loads` and `TermIdentity` | CHANGED (two checks) | The early raw binding (4249-4278) is skipped for an element-load case (4124-4125). Occurrence binding is at 2113-2225. `TermIdentity` has a `dof` and is priced | Generalized early check; one merged occurrence capture with one "used" tracker; a separate `MemberTermIdentity` (section 6.2) |
| P24 | `check_early_occurrences` covers both families | HOLDS | 4291-4311 | Real per-term checks replace the stage-1 branch |
| P25 | DEF-O changes `scope.loads` and `rows.maximum` ("empty loads/no pressure") | HOLDS, with additions | DEF-O `scope.loads = "individual_normalized_nodal_terms"`; `rows.maximum` names the binary64 bounder | New `lanes.loads`, the disclosure, and the kernel-projected maximum (section 6.4) |
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
| P34 | Memory: about 1.05 GB of margin; a 16 MiB loaded-span workspace; 100 MB allowance | RETIRED | Registered headroom is now +51.8 MB (merged dense W4, U6/U8) | Section 9 |
| P35 | Stage-1 pricing premises stay valid | CHANGED | `tools/retained_pricing/base/b7a_phase_model.py`: route L's `formed_cases = max(actual_cases − 1, 0)` and `success_findings = "max(lc-1,0)"`. Their own INVALIDATION text names stage 2 | Removed (section 9) |
| P36 | About 2,500 product and tool lines (kernel about 1,300) and 1,500 test lines | CHANGED | Section 12 | |
| P37 | Corpus 45 bases, 593 mutations, 91 must-pass | CHANGED | The expanded corpus file now has 45 bases, 599 mutations and 90 must-pass | Counts at implementation |

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
- `source_counts` (source.rs 367-462) gains the member-load stride. The 29 `SourceParts { … }` literals take `member_loads: vec![]` (via `..Default::default()` where the literal allows), including the performance harness's k6 W1 adapter.

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

**Why d is not formed.** D1 does not bound coordinate magnitudes. Retained D1.2–D1.11 cap counts and text, and source.rs requires only finite coordinates. So x_j − x_i can need 2,097 bits, for example x_i = 2⁻¹⁰⁷⁴ and x_j = 2¹⁰²³. In scratch draws, 53 of 469 cases had a component that is not exact at 1024 bits. The recipe is therefore deliberately not the frame's own order (round d, then the dot product, then the square root): both p-bit and 1024-bit rounding of d change L̂ on named controls.

**Range.** For finite inputs and a nonzero chord:
- L̂ ∈ [2⁻¹⁰⁷⁵, 2¹⁰²⁶);
- forces' leading bits lie in [2⁻²¹⁵⁰, 2²⁰⁴⁹] and moments' in [2⁻³²²⁸, 2³⁰⁷³];
- every sum in D1–D5 spans at most about 5,220 bits;
- a per-DOF net (declared plus binary64 nodal terms) spans at most 7,325 bits, below `SPAN_LIMIT_BITS` = 8,128 (wide_sum.rs 48).

Formation and netting therefore never refuse. A later row sum that adds such a net to K·u products can still refuse with `Span` on adversarial magnitudes. That is the existing terminal, named refusal, never a truncation.

**Ties.** A square root never ties, but RN(L̂·w) can (the control w = 0.375 hits one). Ties to even are part of the specification and of the Python oracle.

**Granularity: per load.** L̂ and g are formed once per member, and each load keeps its own six entries (twelve ledger terms, zeros included, with each term's nonzero flag set only where the value ≠ 0). This keeps the ledger's granularity of one term per authored contribution (ledger.rs 3-5), so 7a's "a nonzero ledger term, even when the net is 0" applies literally. A cancelling pair of loads on one member still counts as data, as a cancelling nodal pair does. Summing per member first would break that equivalence.

### 2.3 Sign conventions (verified exactly)

The scratch check `/Users/ryan/dev/chirality-t3/scratch/w1-design-kernel/sign_check.py` (log `/private/tmp/w1-design-kernel/sign_check.log`) builds the retained element as assemble.rs does: B_local, D, K = BᵀDB, and recover.rs's end actions, asserted equal to K_local·d. It solves exactly with fractions. For a skew member with d = (3, 4, 12)/2 and y_ref = (4, −3, 0), the global node recipe equals Tᵀ·f_eq(local) exactly, and it matches the ordinary vectors. The solutions match the closed forms:
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

The scratch check (`declared.py` → `/private/tmp/w1-design-kernel/declared.log`) covered 469 cases: 9 named controls and 460 random draws with exponents up to ±1000, each decided exactly by squaring. The worst observed errors were 1.630u for forces and 2.017u for moments.

**The certified problem** is K u = f_n + f_d, with f_d the declared entries. The difference between f_d and the exact consistent load is disclosed, not certified, and enters no published bound (section 6.4).

### 2.5 The ledger, and "exactly like the ledger"

`ExactAccumulator` takes only binary64 values and binary64 products (exact_sum.rs 3-21), so it cannot hold declared entries. For a source with member loads, each loaded DOF's net is formed in an `ExactWideSum`: the nodal net through `add_integer`, then each declared entry through `add_wide`. One new accessor returns the result in `LedgerNet`'s canonical odd form, because `ExactWideSum::net` is private (wide_sum.rs 732). Each entry sets `nonzero[k] |= entry ≠ 0`. A nodal-only source keeps today's code path and today's K4LED bytes.

**Confirmation (HELP_HUMAN's condition): 7a, Ŵ and E treat the declared entries exactly like the ledger.** This holds on two requirements:
- the entries are folded into `RetainedLedger`'s per-DOF nets and per-term flags;
- `recover` reads every member-load term only through its `ledger` argument, never from `source`.

Then `RetainedLedger::empty()` (Ŵ) and `ledger = None` (‖ā_q S‖₁) omit every member-load term by construction.

| Consumer | Today | With declared entries | Verdict |
|---|---|---|---|
| rhs at p | `reduced_rhs` (assemble.rs 875-901): `add_to` (891), then −K_ic·u_c, rounded once | inside the net | same |
| residual at q, m_i, d | `residual_rows` (adaptive.rs 1633-1700): r via `add_to`; d via `add_integer(\|net\|)`; m counts only nonzero K^q·u products (1677) | inside the net; m_i unchanged; d = \|nodal + declared\| | same |
| d^b fallback | `bounded_fallback` (adaptive.rs 1786-1904; ledger 1829-1834) | same | same |
| R7 r, r₂ at q_W | verify.rs 833, 890 (`add_to`) | same; never re-formed | same |
| t₁ | verify.rs 1105-1122 (K's formation error only) | the declared problem's load is exact | no new term |
| E | `formation_scale` (verify.rs 86-288): the ledger enters reactions as \|f_c\| (233) under `Some(ledger)`; ‖ā_q S‖₁ passes `None` (959) | reactions automatic; a loaded member's end-action and station rows gain absolute-operand load terms under the same `Option` switch (section 4.2) | same rule |
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
- **No receipt digest is added.** K4LED's existing hash binds the declared problem for a selected case, and K4SRC binds the source for every case. Neither reader recomputes K4LED today; it is attested (DEF-O `trust.attestation`). The Python verifier gains a **test**, outside the validation gates, that recomputes `cases[].selection.ledger_sha256` with Python integers for every selected preview case in the genuine bases and member-load fixtures. As a gate, it would split the Rust and Python verdicts on ledger mutations. The Rust reader does not rederive it: that would need an independent correctly rounded 1024-bit square root and a K4LED encoder in the product reader and wasm (about 400 lines and priced transients), or a dependency on the producer's arithmetic.

## 3. Residual, verification and scales

R7's law, the gate, the stop rule, the classification thresholds and the case and invocation limits are unchanged. The 2026-10-12 counted-term recipe (P8, P9, P11) is replaced by section 2.5: the declared entries are exact ledger data.

**S* takes no load term (P12).** The member's exact free-body identity, S_F(0) − S_F(1) = w_loc·L, makes its two published end-force rows give S*_fo ≥ ‖w_loc‖∞·L/2. The coupling `coupled_scales` (mo ≥ L_b·fo with L_b ≥ L) then gives S*_mo ≥ ‖w_loc‖∞·L²/2. That exceeds the proposed ‖w‖L²/8, and the force term would raise S*_fo by at most a factor of 2√3. S* carries no soundness (R7 does). Dropping the term leaves both S* computations and the readers' S* rederivation unchanged.

## 4. Recovery, stations and the product certificate

### 4.1 Recovery at p

**Definition.** The certified declared-load problem is:
- u and the reactions solve K u = f_n + f_d;
- each loaded member's end actions are K_e·u_e − f_eq,local(T·w, L), and its stations add the exact span terms.

f_eq is formed locally from the member's axes and w, not by rotating the declared global value. The stations need w_local and L anyway. All inputs at p are p-bit or binary64. Rotating the 1024-bit declared value would put spurious terms of about 2⁻¹⁰²² into the torsion rows. The member's free body is exact: station(0) = −A_i and station(1) = A_j hold bit for bit. Nodal equilibrium between the published end actions and the declared loads holds to within the disclosed 2⁻¹⁰²².

`MemberOperators` gains `length`, which `form_member` already computes (assemble.rs 305) and discards. This is a priced layout change: 8 B per member.

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

The force base matters: today's station forces come from the j-end action (recover.rs 392), which equals −F_i only for an unloaded member. For an unloaded member every load term is zero and the bits equal today's. Reactions are unchanged, because they already subtract the ledger net, which now includes the declared entries.

### 4.2 E for a loaded member

E applies under `Some(ledger)`, mirroring recovery with absolute operands. B̄ puts 1 at every axis component, so one w̄ serves all three local axes:
- w̄ = stage(Σ|w_ℓ,c|), w̄l = stage(w̄·L), w̄ll = stage(w̄l·L), h̄ = RN(w̄ll/12).
- End rows: stage(raw·2^g + w̄l/2) on the six force rows and stage(raw·2^g + h̄) on the four bending rows; torsion rows are unchanged.
- Station forces: stage(raw_j·2^g + w̄l/2 + w̄l·|t|).
- Station moments: stage(old·2^g + h̄ + (w̄ll/2)|t| + (w̄ll/2)t²).

Load terms are not scaled by g_m. Unloaded members, and the `None` case, keep today's bits.

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

**Finding.** The product gate (final_case.rs `gate`, about 866-960) judges each published row value n against its certified enclosure: h = max(n − lo, hi − n). A relative-class row's SharperExact predicate needs h ≤ 2^-64·max(|n|, S) + 2^-85·max(|n|, S) + 2^-53·|n| + 2^-1074. These predicates are unchanged; they are frozen criteria.

PP publishes the maximum (retained_product.rs about 5637-5640) as `value_lower + RN(0.5·RN(value_upper − value_lower))` from the binary64 bounder (`certified_publication::fallback_circular_maximum`, then `elastic_extrema.rs` with `RELATIVE_GAP = 1e-12`, lines 113, 327). That value is off by about 1e-14 to 1e-13 relative for a non-dyadic interior maximum, and by up to 1.1e-12 at small magnitudes. The axial widening also biases it upward when |N|/A is below the transport width. A scratch re-implementation that reproduces the Rust unit expectations shows PP's current value passing on only one loaded control (126 of 200 trials). An exact evaluation from PP's binary64 inputs passes 37–94 %, because those inputs are themselves RN64 projections. The kernel's RN64 projection of its own enclosure passes in every trial (`/private/tmp/w1-design-maximum/exp1_published.log`, `exp2_random.log`, `exp8_recipe.log`). (Simulation, not product runs.)

**Recipe (route L, DEF-O).** In `ProductProofDraft::project` (final_case.rs about 1795-1830), route L's `CircularMaximum` rows are projected like every other row: `project_hull(hull(recipe(k), recipe(s)))`. This holds for loaded and unloaded members alike, so route L has one recipe. PP no longer computes their value, and `complete_maxima` receives none for route L. PP regenerates the maximum evidence from the kernel's certificate:
- station and local fraction: the kernel's witness t̂;
- `value_lower_pa`: RD64(lo);
- `value_upper_pa` and `global_upper_bound_pa`: RU64(hi);
- gap: RU64(hi − lo);
- subdivisions: the kernel's splits.

The remaining evidence fields (coefficient scope, location kind, tie, coverage) are mapped from the kernel's search outcome in the implementation, under the review.

Route E (DEF-E) keeps PP's value and `complete_maxima`, unchanged, and no shared engine (`RELATIVE_GAP`, `fallback_circular_maximum`, `exact_straight_summary_extrema`, `elastic_extrema`) changes. So no ordinary byte moves, and no route-E byte moves.

**Existing nodal-only successors.** Route L's unloaded maxima switch to the kernel projection too. The census of section 4.6 measures what that moves.

**The loaded-member enclosure, per lane:**
- **Field.** N(t) = (1 − t)N_i + t·N_j and M(t) = (1 − t)S_i + t·S_j + q·t(1 − t), with q_y = w_z L²/2 and q_z = −w_y L²/2. S_i = action(End I) and S_j = action(End J) are the lanes' corrected end enclosures. The q enclosures come once per row from the lane's frame helper, for that one member. Each lane uses its own A and Z.
- **Search.** Branch-and-bound on dyadic subintervals, with Bernstein controls N = [N_i, N_j] and M = [S_i, (S_i + S_j + q)/2, S_j], and outward de Casteljau splitting at the midpoint in directed 1024-bit arithmetic.
- **Upper bound per node: min(paired, tangent).** Both bounds are rounded up.
  - *Paired*: the objective at each joint Bernstein control point, the law of `control_upper`. It overestimates by O(h²).
  - *Tangent*: the Bernstein controls of sN/A + (Q/Z² + ρ²)/(2ρ), with Q = M_y² + M_z² as a degree-4 polynomial, s = ±1 on a sign-constant N, and any ρ > 0 (an approximate ‖M(mid)‖/Z).
  - The box-corner bound (separate maxima of |N|, M_y and M_z) overestimates by O(h). It exhausts 256 nodes on the skew two-axis and axial-slope controls, and live nodes grow 4× for every 16× tighter tolerance. It is not used.
- **Lower bound.** The search's best evaluated point, rounded down. Its t̂ becomes the published governing station.
- **Stop.** When U − L ≤ 2^-64·L. A hull width ≤ 2^-63·|n| suffices for the RN projection to pass.
- **Limits.** 256 live nodes, depth 64, and 1,024 splits per lane. Exhausting any limit returns [L, the maximum upper bound over the live nodes and the unexpanded parent]. That interval still contains the maximum and is wide, so the unchanged gate fails the row and the case publishes `unavailable` (fails closed), with a named kernel cause.
- **Scratch results** (`exp3_kernel_*`, `exp5*_degenerate`, `exp9_edges`):
  - the main controls certify in ≤ 33 splits, ≤ 18 live nodes and depth ≤ 33;
  - near-degenerate rotating moments down to e = 1e-8 certify in ≤ 26 splits and ≤ 13 live nodes;
  - a tie at 1e-58 certifies in 30 splits;
  - an axial sign change and a moment passing through zero both certify;
  - judged end to end against the actual enclosure, n = RN64(midpoint) passes all four predicates in 252 of 252 trials.
- **Memory.** About 17 Endpoints of 144 B per node, so 256 nodes take about 0.60 MiB per lane. The lanes run one after the other with one member live, within the 1 MiB cap of section 9.
- **Work.** Section 5.

**Corners that fail closed.**
- Near-degenerate maxima with a varying axial force could exhaust even min(paired, tangent). This is theoretical and was not observed; a Taylor bound with an f″ enclosure would close it at higher cost.
- The two lanes use different sections (binary64 A and the Z hull against geometric A and Z). That widens every stress row's hull, maxima included. In a rough model it alone fails a perfect projection in 296 of 4,000 trials. This is inherited by all stress rows and is not created here; the implementation measures it on real cases (section 13).

### 4.5 Combinations

`combine.rs::validate_preps` (233-252) refuses, after `NestedCombination` (238) and before `OperandsDiffer`, any operand whose source has member loads, as a new `CombinationReason::OperandMemberLoads`. This matches DEF-C's `operand_member_loads`. As a backstop, `RetainedLedger::combined` (ledger.rs 209-229), which reads only nodal loads, refuses such a source with a new `LedgerRefusal`, mapped in `CasePrep::combination` (adaptive.rs 985).

### 4.6 The maximum-projection census

> *Measured at the design stage on the committed successors and corpus bases; this section is completed by the census run (section 13, "Evidence").*

## 5. Work

**Where it is counted.** The declared ledger is formed at the start of the case's first own-stage run (`solve_case_at`, adaptive.rs 1908-1954), in its `rhs` stage. It uses that function's existing 1024-bit context and accumulator. The work flows into the attempt's `work`, `k4_work` and `stages.rhs` once, and is charged once to the case limit and the invocation meter. `guard.test` runs after each member (all its loads) and after netting, the same checkpoint discipline as `form_members` (assemble.rs 521-534). The stage fields are published and closed in the schema (retained_wire.rs 283-286), so no stage is added.

**Cost** (multi.rs 86-93 at L = 16, in limb-multiply equivalents):
- per loaded member, at most 38,822: 2,880 for nine two-products, 32 for the rounding, 18,468 for the square root, 17,442 for the division;
- per load, at most 7,776;
- plus `SumWork`, so the formation is never zero;
- at the D1 caps (32 members, 128 loads per case), about 2.24M per case, against the unchanged 20e9 case and 60e9 invocation limits.

The loaded maximum's search counts through `NumericWork` plus its wide and sum work:
- at most 14,498 directed entries per lane per loaded member (about 440 per split), about 29k for both lanes;
- at least 238 with no split;
- at most about 0.45M per lane under the split budget.

**RF's metered rule, applied.** RF's rule (rf-source-method-extension.md §5.1) is scoped to `exact_boundary`'s `Work`, which no W1 path uses. Stage 2 applies its substance through W1's existing counters:
- every operation is counted at its actual executed cost (`WideContext` charges each operation as it executes, multi.rs 1162-1186; the accumulator charges per limb touched);
- nothing is precharged;
- nothing is counted as zero;
- there is one rule, with no member-load regime.

**Refusal timing.** W1's `StageGuard` refuses at checkpoints after a step (adaptive.rs 239-295). A refusal can therefore land up to one member's formation past the limit: about 39k plus 7.8k per load plus `SumWork`, at most about 1.04M against 20e9. Refusing before each step, as RF's Context does, would be a second regime if applied to this formation alone, and a new rule if applied to all of W1. It is not proposed (ruling R-1, section 11).

## 6. Producer, definitions, receipt, schema and readers

### 6.1 Retiring stage 1's interim cause

Delete the following:
- `MEMBER_LOAD_CAUSE` (retained_product.rs 43);
- `capture_member_load_case` (1794-1834);
- the `member_loads` flag (173, 295, 376);
- the stage-1 skip of the captured-invocation check (4124-4125).

Restore the check in `finish` (2253) that every captured case has a source. `MEMBER_LOAD_CAUSE`'s 86 B of TEXT leaves the profile.

### 6.2 Capture (`retained_product.rs`)

- **Early raw check.** `check_captured_nodal_leaves` becomes a check over both families, run for every case. For an element leaf it requires:
  - the raw target `element` with the same pipe;
  - category `distributed_force`, dimension `force_per_length`, and a direction in `global_x|y|z` (D1.7);
  - the same id;
  - the raw value converted to N/m giving the same bits, with the normalized unit `N/m`;
  - no extent.
- **Occurrence capture.** One capture of the case's terms with one shared "used" tracker (generalizing 2113-2225). Each member occurrence must match exactly one unused authored load with the same id and magnitude bits, the target pipe equal to the model's pipe at that index, and the same global direction. The member is the model pipe index as `u32`, which is the kernel member id. The intensity has only the load's axis set; the off-axis entries are +0.
  - Then come the single unconsumed-load check, `PrimitiveSource::new`, and one canonical map per family.
  - New refusal texts: "member load occurrence not authored" and "canonical member term map".
- **`MemberTermIdentity`** (about 64 B) has its own per-case vector, so `TermIdentity` and nodal-only layouts do not move.
- **Gating.** Every new adapter event, reservation, capacity and kernel work charge happens only when the case has member loads. Adapter counts and `prepared_capacity_bytes` are receipt bytes, and the adapter records an allocation event even for an empty vector (3987-4006).
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

**Receipt bytes for a member-load case:** `member_load_terms`, `kernel_source_sha256` (K4SRC with the member block), `selection.ledger_sha256` (K4LED with the declared entries), the rows and maximum evidence, the adapter counts and slot 15. The interim preparation error and the `source_unavailable`/`preparation` reason disappear.

**Nodal-only receipts.** These stay byte-identical except for the DEF-O-bound hashes of section 6.4 and any maximum bits that section 4.6's census shows moving.

### 6.4 Definitions and the disclosure (HELP_HUMAN, 2026-10-17: "accepted as you recommend")

The disclosure is placed in three places: the DEF-O/DEF-C wording; table #4's `supported_profile_limitations`; and a fixed member-load message on each selected member-load case's `RETAINED_PRECISION_SELECTED` diagnostic. Neither nodal-only nor ordinary bytes move.

- **DEF-O** (`fixtures/results/retained_precision_prepared_ordinary_v1.json`, #3; keep its sorted-key compact form):
  - `scope.loads` becomes `"individual_normalized_nodal_terms_and_declared_full_span_uniform_member_terms"`.
  - New `lanes.loads`: "the case's K4LED: each loaded DOF's exact net of its individual normalized nodal terms and its declared member-load entries; per full-span uniform member term (member i-j, global intensity w in N/m, binary64) six fixed binary1024 entries formed once: S=d.d exact from nine binary64 products with d never formed, L=RN1024(sqrt(S)), g=RN1024(L/12); force RN1024(L*w)/2 at i and at j; moment RN1024(g*(d x w)) at i and its negation at j, d x w exact; folded exactly into the same per-DOF nets and per-term data flags; residual, verification, reactions and Ŵ/E use them exactly as the ledger; member end actions and stations use the member-local consistent load from w and L".
  - New `acceptance.declared_member_load`: "The certificate certifies the declared-load problem K u = f_n + f_d. Each declared entry differs from its exact consistent value ((L/2)w and ±(L/12)(d x w) at the exact length) by at most 2^-1022 of that value's magnitude; this difference is disclosed, not certified, and enters no published bound." The bound is stated per entry, because entries from different members can cancel at a node.
  - `rows.maximum` becomes: "H of hull(admitted_k maximum, annular_source maximum); per lane the member's circular maximum of |N(t)|/A+‖M(t)‖/Z over t in [0,1]: two ends for an unloaded member, directed branch-and-bound with the member's local full-span uniform loads for a loaded member; regenerate all eight numeric evidence fields from the certificate (station = witness, lower = RD64(lo), upper = RU64(hi)); no pressure".
  - The meaning hash (now `c4aebcef…`) moves in the producer (retained_wire.rs 34-35), the Rust reader (9-11), the verifier (35), the table (line 1176) and desktop test support.
- **DEF-C** (`retained_precision_prepared_combination_v1.json`, #6): `scope.excludes` appends `"operand_member_loads"`.
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
- **Data flags** (Rust about 4022-4046; Python about 1600-1606): if intensity component k is nonzero and a node of the member has translation k free, that body must have data. A moment component j ≠ k also implies data when the coordinate difference along the third axis is nonzero. Both are exact comparisons.
- **Combinations:** a `retained_selected` combination has no operand source carrying member terms, and an `operand_member_loads` reason requires one (extending the cause check at 3113-3121).
- **Selected-case diagnostic:** the member-load message exactly when the case's source has member terms (section 6.4).
- **Maximum evidence on route L:** the regenerated evidence fields' structure (section 4.4).
- **Pins:** the DEF-O, DEF-C and table hashes. G0 needs no code change, and G1's source hash already covers the whole source object.
- **K4LED** is not rederived by either reader (section 2.6); the Python test recomputes it.
- **Desktop.** Product TypeScript reads through the Rust reader in wasm, so no product TypeScript changes. Test support changes: the `DEF_O` constant (`retainedPrecisionReaderInputs.ts` 11), the successor-fixture list if a member-load fixture is added, and the pins in `retainedPrecisionResultExport.test.tsx` 51-52.

### 6.6 PP's preparation of the maximum

On route L, `prepared_maxima` (retained_product.rs 5589) no longer calls `exact_straight_summary_extrema`. It takes the kernel's projected value and certificate outcome, and writes the evidence of section 4.4. On route E it is unchanged. Its stage-2 replacement of `&[]` (P20) is therefore not needed. The shared ordinary engine is untouched.

### 6.7 The Preparation fallback control

After stage 2, no member-load input should reach the Preparation fallback (inference): the interim cause was its only route. The milestone plus a UDL, and the two single-case UDL controls, become positive real-input controls that build a source and run Native.

For a genuine Preparation input, the implementation probes an annulus-preparation refusal: a section whose 1024-bit enclosure of I lands on a subnormal or non-normal binary64 value while the ordinary run attempts the case. If no such genuine input exists, the 2026-10-15 note is closed with that finding, and the fallback stays covered by the fault and tamper controls in `retained_facade_tests::u8_real_input_fallbacks_append_one_notice` and beside it.

## 7. U6/U8 and the later O4 epoch

**This unit certifies Euler–Bernoulli straight members only.** Route L carries no shear model and no allowance (P30). `StraightMember::from_frame` refuses a Timoshenko leaf by name (`RETAINED_TIMOSHENKO_REFUSAL`, assemble.rs 226-281). W1's retained member names its Euler–Bernoulli leaf (retained_product.rs 1998-2016, 5620-5632). Timoshenko and nonzero-allowance documents stay declined by O4's interim declines in `prepare_sources` and at W1-E's admission (`ShearModel`, `SectionAllowance`), and route E refuses element loads (D1.7).

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
- **The maximum recipe can extend to route E.** O4 revises DEF-E in any case and can adopt the kernel projection of section 4.4 for route E's maxima (per basis after D-5).
- **Headroom.** O4 and U4b/U5b need memory headroom of their own (section 9).

## 8. RF interactions

- **Metered charging.** Section 5. Stage 2 adds no `exact_boundary` work, so RF's rule does not reach it automatically. Its substance (actual executed cost, no precharge, never zero, one rule) holds through W1's counters. W1's checkpoint refusal timing is unchanged (ruling R-1).
- **The J7 RF census.** It does not interact. The census runs `run_linear_static_preview_value_with_mode`, which never selects a retained caller profile (PP `lib.rs` 2322-2328). It excludes its three member-load cases as `non_nodal_load` (`j7_rf_census.rs` 432, 444). Stage 2 cannot move the census pin, and RF cannot move stage 2's route-L controls.
- **The retained corpus.** RF expects `fixtures/results/retained_precision_cases.json` unchanged for its own effect. Stage 2 lands second and regenerates the corpus on top of RF's bytes (section 6.4).
- **Shared files.** Whichever unit lands second rebases onto the other's merged bytes. Stage 2 lands second.

  | File | RF | Stage 2 |
  |---|---|---|
  | FK `src/structural.rs` | `Expansion` charged `add_product` and counters | `retained_api` exports (12-61); lever L1a's `FormationSource` borrow |
  | FK `formation_check.rs`, `load_ledger.rs`, `certified_recovery.rs`, `structural/sparse.rs` (DEL-04-01 K-D5 formation code) | frame-kernel work accounting nearby (RF §9) | lever L1a; byte preservation pinned by a test, as for U6-DH/U6-SH |
  | FK `Cargo.toml` | dev-only `exact-operation-count` feature | none expected |
  | FK `tests/s11_site_table.rs`, PP `tests/s11f_site_test.rs` | one row | rows for new retained accumulations |
  | PP `retained_memory.rs` and its law-test digest array | moves reviewed input #1, re-registers | moves #2, #3, #4, #6, re-registers from RF's registered profile |
  | `retained_facade_tests.rs` | B3B RV123 S2 route-E pins | W1 route-L tests |
  | `tools/retained_pricing/**`, `retained-pricing-rules.md` | T07 order-4 and congruence rules | stage-2 owners and lever rules |
  | `b7-plan.md` | its RF row, Status and J7 paragraph | the member-load pointer (this change) |
  | `fixtures/product_preview/*_generation.json` | if regenerated | if regenerated |

- **D3.** Stage 2 re-registers from RF's registered profile. Section 9 uses RF's stated ~8 MB (derived, RF §9). At implementation start, RF's actual registered headroom replaces that figure. If RF's real delta exceeds 8 MB, section 9's lever set must still leave positive headroom, or the shortfall returns to HELP_HUMAN.

## 9. Memory (D3) feasibility

Fixed throughout: M = 12 GiB = 12,884,901,888 B, floor(0.9M) = 11,596,411,699 B, R = 64 MiB, and the upward 5 % TEXT reserve (TAV_W/TAV_X). Headroom is measured below the floor after the reserve.

**Labels:**
- *Measured* means the pricing tool run on a source revision, evaluated at the registered in-build atoms.
- *Tool-evaluated* means scratch rules or surrogates run through the tool on main `4971d62dc9` at those atoms.
- *Derived* means computed from tool-evaluated deltas.
- *Estimated* means a hand estimate.

**Basis (measured).** `price.py run 4971d62dc9` reproduces U6/U8's registered profile exactly: all 1,456 coefficients equal, `generate --check` differs only in the basis line, and all 70 evaluations equal the registered figures. Merged dense W4 is +51,812,817 B; merged dense X1 +77,334,515 B; merged sparse W4 +110,944,161 B. Atoms come from `/private/tmp/t4-u6u8-reg2-logs/native/N_profile.log`.

**What stage 2 falsifies (measured).** Route L's credit `formed_cases = max(actual_cases − 1, 0)` and `success_findings = "max(lc-1,0)"` (`base/b7a_phase_model.py`) rest on stage 1 forcing at least one selected case to be nodal-only. Removing them adds 76,293,661 B to W4 in both modes:

| Component | Bytes |
|---|---|
| `success__O_base` | 54,467,046 |
| — uniform constructor staging, per formed case | 18.38 MB |
| — typed formation-source copies (KD5O3) | 3 × 4.56 MB |
| — W2 scaled-force copies | 3 × 4.56 MB |
| — assembled children | 6.14 MB |
| — recovery source copy | 2.56 MB |
| `success__TAV_W` | 12,369,408 |
| `success__FORMATION_VALUE_BUILD` | 6,308,454 |
| `success__T11` | 3,148,753 |

W3 also gains `T16_P2` +6,293,222 B. `MEMBER_LOAD_CAUSE` carried no credit: the producer model already prices T12–T15 for every attempted case.

**Stage 2's own owners (tool-evaluated surrogates).** These are totals over 3 cases, with l member terms beside the nodal ones. Layouts were probed with a rustc mirror: `Wide<16>` 144 B, `UniformMemberLoad` 56 B, member term identity 64 B.

| Owner | Bytes |
|---|---|
| T11.3 member term identities | 122,880 |
| T11.5 `member_loads` | 124,488 |
| T12.1 member-term map entries | 64,512 |
| T12.2 member loads, sort scratch and declared-row scratch | 172,032 |
| T12.3 source clone | 124,416 |
| T12.4 declared rows (12 × `Wide<16>` + index per load) and ledger | 1,324,224 |
| T12.5 E_src/E_led | 229,248 |
| T13.1 retain / T13.2 scratch | 266,112 / 40,608 |
| T14.1 lanes | 463,104 |
| T14.2 loaded-span workspace and spans | 3,194,880 at 1 MiB per case; 50,380,800 at 16 MiB |
| T14.3 station terms | 15,552 |
| receipt `member_load_terms` | +1,311,264 SUCC, +2,621,056 T16 P2 |
| G8_BYTES | 393,216 |
| STATICS (DEF-O, DEF-C, SCHEMA and table text) | 243,008, estimated |
| TEXT, a phase-level constant on TAV_W and TAV_X | 2,097,152 plus 104,858 reserve, estimated |

**Levers.** HELP_HUMAN, 2026-10-17: "Prefer real memory reductions to accounting arguments", take them first. Rules 1b, 1c and 2 are "acceptable as pricing accuracy, not credits, if code facts show the lifetimes they claim. Each needs a source pin and independent review, and a test where one can pin the lifetime."

*Real reductions* (byte-preserving source or design changes):
- **L4. Loaded-span workspace ≤ 1 MiB per attempted case, one member live** (section 4.4: about 0.60 MiB per lane, lanes sequential). Saves 47,185,920 B against the 16 MiB cap.
- **L1a. Borrow the typed primitive copy.**
  - `FormationSource.loads` becomes a borrow (`Cow<'a, [LoadPrimitive]>` or equivalent), in place of `f.load_primitives().to_vec()`: PP `structural_adapter.rs` 298 (linear selected) and 567 (W2), and the recovery copy at `certified_publication.rs` 16.
  - Same elements and order, and `Cow`'s Debug prints the slice, so outputs are bit-identical.
  - Saves 30,720,000 B.
  - Touches FK `formation_check.rs`, `structural.rs`, `structural/sparse.rs` and `certified_recovery.rs`, PP `structural_adapter.rs` and `certified_publication.rs`, the performance harness's `k6/staged.rs`, and about 30 kernel-test `FormationSource` constructions; closure mutants use `to_mut()`.
  - No reviewed input moves. The tool gains a source-bound check on the borrow sites.
- **L3. Move the formation Value instead of copying it.** serde_json 1.0.149's `json!` expands to `to_value(&expr)` (macros.rs 279), so `finding` in `ordinary_members` (retained_wire.rs 826) and `formation` in `ordinary_entry` (1574) are deep-copied. Moving them gives identical Values (no `arbitrary_precision`; maps are BTreeMap). Saves 18,923,890 B in every phase. Touches retained_wire.rs only, plus a receipt byte-identity test and a source-bound pricing check.
- **L5. Route-L aggregation guard** (the U6-RB analogue). In `append_combination_results` (`preview_physics.rs` 854-860), the `rows_by_base_id` clone and the `case_rows`/`base_ids` clones are built only when `!model.combinations.is_empty()`, because they are read only inside the combinations loop. Byte-preserving. Saves about 103 MB at L_30 but about 30.6 MB merged, because route E's coefficients dominate the maximum (derived). The `case_rows` clone is also named as its own pricing family; today it is covered only in aggregate.

*Pricing-accuracy rules* (code facts on lifetimes, each with a source pin, independent review and a lifetime test where one can be pinned):
- **L1b.** The closure comparison builds one `p.closure_entry()` at a time (`formation_check.rs` 647), so one transient entry is charged, not q. Saves 18,014,760 B.
- **L1c.** `AssembledForce::force_scaled` (`load_ledger.rs` 413, via `LoadPrimitive::force_scaled`, `formation_check.rs` 296) exists only in the b ≠ 0 evaluation and is dropped when that call returns. The b = 0 evaluation borrows `f`, and the return tail holds none. So the scaled children are charged once, not three times. Saves 27,377,664 B.
- **L2.** Uniform-constructor staging is sequential. `add_uniform_element_loads` (PP `lib.rs` 10975) calls `equivalent_global_nodal_loads_with_spans_formed(&[global])` once per load, and its staging (`exact_scaled_intended`, `monomial_bounds`, straight_pipe `lib.rs` 1900) is local and freed on return. The returned `Vec<Formation>` is consumed by `.zip` into the ledger, which is already priced as resident. `certified_loads::attach_last` (`certified_loads.rs` 122-160) stages one term at a time. The bound per case is one call's owners plus one term's attach staging (95,408 B), keeping the ×c convention and the moving allowance. D1 has one single-threaded call per case, and D1.4 refuses arcs, so only the straight branch is reached. Saves 54,865,392 B.

*Not used.* Two levers need more than this unit carries:
- A shared force-scale exponent outside the primitive (about 13.7 MB) changes the K-D5 closure binding.
- One maximum helper per invocation (about 96.5 MB) narrows the per-case convention and would need a ruling.

**Per-phase headroom.** RF's ~8 MB is included in every phase. Merged dense W4 is the binding phase.

| Tree / mode / phase | (a) U6/U8, measured | (b) + RF, derived | (c1) + credit removed, measured + RF | (d) + stage-2 owners at L4 | Real: L4 + L1a + L3 | Real + rules: + L1b + L1c + L2 |
|---|---|---|---|---|---|---|
| merged dense W4 | +51,812,817 | +43,812,817 | −32,480,844 | −42,379,181 | +7,264,709 | +107,522,525 |
| merged sparse W4 | +110,944,161 | +102,944,161 | +26,650,500 | +16,752,163 | +66,396,053 | +166,653,869 |
| merged dense W3 | +264,289,581 | +256,289,581 | +173,702,698 | +162,494,569 | +212,138,459 | +312,396,275 |
| merged sparse W3 | +323,420,925 | +315,420,925 | +232,834,042 | +221,625,913 | +271,269,803 | +371,527,619 |
| merged dense X1 | +77,334,515 | +69,334,515 | +69,334,515 | +66,642,129 | +116,286,019 | +216,543,835 |
| merged sparse X1 | +136,465,859 | +128,465,859 | +128,465,859 | +125,773,473 | +175,417,363 | +275,675,179 |
| L_30 dense W4 | +177,800,942 | +169,800,942 | +90,353,790 (derived) | +80,455,453 | +130,099,343 | +230,357,159 |
| E_30 dense W4 | +509,292,997 | +501,292,997 | +501,292,997 | +491,394,659 | +491,394,659 | +491,394,659 |

Labels for the table:
- Columns (d), "Real" and "Real + rules" are tool-evaluated (scratch tool `/Users/ryan/dev/chirality-t3/scratch/w1-design-price-tool-s2b/`, runs `…/w1-design-price-runs/s2b_*`).
- The "Real" column excludes L5, which is derived at about +30.6 MB merged; with L5 the real-only merged dense W4 is about +37.9 MB.
- Every lever's tool delta equals its analytic saving exactly, and the levers add.
- All phases not shown (W1, W2, W5, X2) keep more than 4.6 GB.
- Route E is unchanged by stage 2, except that the surrogates are also applied to it, which overstates it.

> *The real-only set with L5, and the full set with L5, are being run through the tool as single runs; their measured figures replace the derived ones here (section 13).*

**Verdict.** Stage 2 fits at M = 12 GiB with the unchanged margin and reserve only with levers. Its own need in merged dense W4 is about 86.2 MB: the falsified credit, 76.3 MB measured, plus its owners, 9.9 MB tool-evaluated at L4. The real reductions alone recover about 80.2 MB with L5 (derived). That leaves about +37.9 MB in merged dense W4, below today's post-RF +43.8 MB. With the three pricing-accuracy rules, about +138 MB (derived with L5).

Epochs 4 (U4b/U5b: arc quantities and tie rods) and 5 (O4) follow and need headroom of their own. The real-only set leaves them less than they have today; the full set leaves about three times as much. The implementation applies the real reductions first, then the rules, each with its pin, review and test.

**What remains unpriced or estimated:**
- The TEXT constant of 2 MiB is estimated. The full set tolerates about 100 MB more; the real-only set about 37 MB.
- `s(ExactWideSum)` grows if its limb capacity must widen for 1024-bit summands. It does not need to for these spans (section 2.2: ≤ 7,325 bits), but this is to be confirmed by the native profile.
- The surrogates assume that the lanes accumulate per row (per-contribution storage would add about 0.88 MB per case) and that the declared rows are held once per attempted case, not cloned with the source (each clone would add about 0.67 MB). Both are design requirements in sections 2.5 and 4.3.
- The actual-source D3 at implementation replaces every figure here.

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
| Two-end recipe on a loaded member; q dropped | `w1_loaded_maximum_interior_controls_certify` | simply supported (wL²/8)/Z; end moments with a non-dyadic argmax |
| q sign or axis flipped | `w1_loaded_maximum_bubble_sign_and_axes` | non-dyadic argmax; skew with two load axes |
| Station orientation reversed | `w1_loaded_maximum_station_orientation` | argmax 0.59915 against 0.40085 |
| Lower endpoint above the true maximum | `w1_loaded_maximum_enclosure_contains_independent_reference` | exact rational check, plus 200 random members against a 2048-bit reference |
| Exhaustion narrows instead of widening | `w1_loaded_maximum_exhaustion_widens_and_refuses` (capacity 4 / depth 6 hooks) | must contain the reference, fail the gate, and leave the case unavailable |
| Box-corner bound used; tangent bound dropped | `w1_loaded_maximum_rotating_and_axial_slope_within_capacity` (≤ 64 splits, ≤ 32 live); the near-degenerate control at e = 1e-5 | — |
| PP's value published on route L, or PP's 1e-12 gap used | `w1_retained_maximum_is_the_kernel_projection` | row bits equal the kernel's RN64 midpoint |
| Route E's maxima moved | `w1e_maxima_keep_the_prepared_value` (existing route-E successor pins byte-identical) | — |
| Combination with a member-load operand selected | `combination_with_member_load_operand_is_refused_by_name`; ledger backstop test | — |

**Kernel: work (section 5).**

| Fault | Killed by | Control |
|---|---|---|
| Formation work zero, re-formed per attempt, or unguarded | `member_load_formation_is_charged_once_in_the_first_rhs`; `member_load_formation_refuses_by_budget_at_a_member_checkpoint` | a case limit set between the two totals |
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
| K4SRC member block omitted or wrong | genuine member-load must-pass bases; corpus `kernel_source_without_member_block` |
| Data rule missing in a reader | corpus `member_load_body_has_data_false` |
| Combination selected with a member-load operand; spurious `operand_member_loads` | `b2p_member_load_operand_combination_is_unavailable_by_name`; corpus `combination_with_member_load_operand_selected`, `operand_member_loads_without_member_operand` |
| Disclosure missing (DEF-O, DEF-C, table, diagnostic) | `def_o_def_c_and_table_state_the_declared_member_load` (Rust and Python); corpus `member_load_case_selected_message_missing`; the G0 hash pins |
| Schema left open | corpus `member_term_extra_field` (G1) |

**Levers (section 9).**

| Fault | Killed by |
|---|---|
| L1a changes an output (a reordered or rebuilt primitive) | `formation_source_borrow_is_byte_identical` (K-D5 closure and receipt bytes against the pre-lever pins); the existing closure mutants via `to_mut()` |
| L3 changes a receipt byte | `ordinary_formation_value_move_is_byte_identical` (text and hash against the former construction, as for U6-DH) |
| L5 skips a clone that combinations read | `append_combination_results_guard_keeps_combination_bytes` (combination and no-combination invocations byte-identical) |
| A pricing rule's lifetime premise is false | the rules' source pins (the tool stops when a pinned function changes); lifetime tests where pinnable: for L1c, the scaled force is dropped before the b = 0 evaluation returns; for L2, at most one call's staging is live |

**Census.** `w1_source_census_tests` pins the other case's status and cause for F6/partial, T18 and T19 in both modes. Case 0 stays selected.

## 11. Rulings

**Applied** (HELP_HUMAN, 2026-10-17):
- **Option B, the declared member load.** Sections 2 and 3. The proven bound 2^-1022 per entry supersedes "about 2^-1019".
- **Disclosure placement.** DEF-O/DEF-C, table #4's limitation, and the selected member-load diagnostic (section 6.4).
- **Lever policy.** Real reductions first; rules 1b, 1c and 2 as pricing accuracy, with pins, review and tests.
- **DEF-O hash moves** under the E14 precedent.
- **This census:** section 4.6.

**Decided in this design, for the independent review to check:**
- per-load granularity of the declared entries (2.2);
- d never formed, and L̂ from the exact S (2.2);
- recovery from the member-local consistent load (4.1);
- S* without a load term (3);
- route L's maxima, loaded and unloaded, projected by the kernel, with route E unchanged (4.4);
- the min(paired, tangent) bound with exhaustion failing closed (4.4);
- `member_load_terms` optional and K4SRC's block conditional (2.1, 6.3);
- no K4LED rederivation in the Rust reader, and a Python test instead (2.6);
- the combination refusal's position (4.5).

**Needed:**
- **R-1. What "RF's metered rule" requires of W1.**
  - *Recommendation:* counting at actual executed cost through W1's existing counters, with no precharge, never zero, one rule, and W1's existing checkpoint refusal timing unchanged (section 5).
  - *Consequence if refuse-before-each-step is required:* either a second regime inside W1 for this formation alone, or a new W1-wide rule changing at-limit refusal timing for every W1 attempt, which is itself a ruling. Work totals would not change either way.

**Reserved and untouched:**
- D1's limits and the domain;
- every ScopeOfWork commitment, acceptance criterion, frozen reference, criterion and tolerance (including the product gate's predicates);
- decision 22 (M, the 0.9M margin, R, the 5 % reserve and the budget model);
- b7-plan's "Held by the owner";
- B8 and the frozen UI benchmark;
- desktop outputs for the exact identity.

**G10** stays owner-held. Stage 2 moves the milestone's retained-successor bytes: the DEF-O-bound hashes, and any maximum bits the census shows. It does not move the milestone's ordinary bytes. Whether G10 is witnessed before or after is the owner's choice.

## 12. Size, tests and the independent numerical review

**Size** (inference from the touch list; the 2026-10-12 figures were 2,500 product and tool lines and 1,500 test lines):

| Part | Lines |
|---|---|
| Kernel: source, declared ledger, recovery, E, lanes, combination refusal | about 1,800–2,600 product |
| Kernel: maximum search | about 400 product |
| Producer, wire, readers, verifier and pricing tool | about 600–800 |
| Levers L1a, L3, L5 | about 200 |
| Tests | about 2,000 |
| Corpus data | additional |

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

**Independent numerical review checklist** (carried forward from 2026-10-12 and updated). The review is by agents separate from the implementer. It must check:

1. **The declared recipe D1–D5:**
   - its sign conventions (node i and j; the d × w orientation) against the closed forms;
   - the full specification of every rounding (to nearest, ties to even, at 1024 bits);
   - the independence of the Python oracle from the producer.
2. **The disclosed bound.** That 2^-1022 per entry holds, and that DEF-O, DEF-C, the table limitation and the selected member-load diagnostic state it.
3. **"Exactly like the ledger"** for rhs, the residual and m_i (not counted), d and d^b (|net|), R7's r and r₂ at q_W, t₁ (no term), E, Ŵ (the empty ledger omits every member-load term, and `recover` reads them only through the ledger), the 7a data flag (a nonzero entry sets the flag even when the net cancels), the uniqueness warrant, and the lanes' free rows and reactions. If any of 7a, Ŵ or E does not hold, the unit returns to HELP_HUMAN.
4. **Recovery.** That the certified problem is self-consistent: u and the reactions come from f_d, and end actions and stations from the member-local load. Also the end-action formulas at p, the j-side station terms, E's absolute-operand load terms, the lanes' corrected ends and stations, and that unloaded members' bits are unchanged.
5. **S* without a load term.** The free-body and coupling argument.
6. **The loaded-span maximum:**
   - the field (N linear; M quadratic with q);
   - that each node bound is an upper bound under directed rounding;
   - the lower endpoint, the stop rule, the limits, and exhaustion widening and failing closed;
   - the RN64 projection;
   - route L only, with route E's bytes unchanged;
   - the evidence mapping;
   - the census of 4.6 (counts, ulps, predicates, verdict and standing).
7. **Combinations refused** in the kernel, the ledger backstop and both readers.
8. **Work.** Counted once at actual cost in the first own `rhs`; guard checks per member; the maximum search in `NumericWork`; limits unchanged; nothing counted as zero.
9. **Ranges.** No refusal in formation or netting (span ≤ 7,325 < 8,128 bits); later `Span` refusals are the existing terminal refusal.
10. **Custody.** Per-case custody under basis reuse with member loads.
11. **Pricing:**
    - every new rule (the stage-2 owners, L1b, L1c, L2, L5's family) argued from code facts with a source pin;
    - L1a's, L3's and L5's byte preservation;
    - the credit removal;
    - the actual-source D3 registration.

## 13. Implementation order, verification and evidence

**Order of work**, from merged main after RF:
1. Re-check section 8 on RF's merged head, including RF's registered headroom against section 9's assumption.
2. The real reductions L1a, L3 and L5 with their byte-identity tests. Then the pricing rules L1b, L1c and L2 with pins and lifetime tests. Re-price without stage 2: the gain must match section 9.
3. Kernel: source, declared ledger and Python oracle; recovery, E and lanes; the maximum search; the combination refusal.
4. Product: capture, preparation, wire, DEF-O, DEF-C, table #4, SCHEMA, the route-L kernel-projected maximum and its evidence, and the diagnostic.
5. Readers and verifier; the corpus and successor regeneration by their own writers.
6. The pricing rules for stage 2's owners, and the actual-source D3 re-registration at M = 12 GiB with the unchanged 0.9M margin, R and 5 % TEXT reserve.

**Checks:**
- the mutants of section 10;
- the suites `select_affected_checks.py` selects (frame kernel, product physics including the census and facade tests, result_export, headless, the Python verifier tests, and desktop test support);
- the corpus in native Rust, wasm and Python;
- the hosted `harness` on the exact head.

Independent scrutiny is required because this changes numerical meaning: the checklist of section 12, by agents separate from the implementer.

**Evidence relied on by this design.** All of it is outside the repository and may not survive a reboot.
- U6/U8's registration: `/private/tmp/t4-u6u8-reg2-logs/native/N_profile.log` and `eval70_p1.log`, and `/Users/ryan/dev/chirality-t3/scratch/t4-u6u8-d3r-runs/{p588,p3e68}`.
- Pricing: `/private/tmp/w1-design-price/` (`price_main_4971.log`, `compare_main_vs_p588.json`, `eval70_main.log`, `s2a_tool.diff`, `eval70_s2a.log`, `s2b_tool.diff`, `table_s2b_cde.txt`, `table_s2b_combos.txt`, `s2b_layout_probe.log`). Scratch tools are `/Users/ryan/dev/chirality-t3/scratch/w1-design-price-tool-{s2a,s2b}/`, with runs under `…/w1-design-price-runs/`.
- Kernel: `/Users/ryan/dev/chirality-t3/scratch/w1-design-kernel/{sign_check,declared,recovery_b}.py`, with logs in `/private/tmp/w1-design-kernel/`.
- Maximum: `/Users/ryan/dev/chirality-t3/scratch/w1-design-maximum/`, with logs in `/private/tmp/w1-design-maximum/` (`exp1`–`exp9`).
- Census of today's trial outcomes: `/private/tmp/w1-design-producer/` (extracted from the B7a/B7b logs).
