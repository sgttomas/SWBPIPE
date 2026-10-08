# I94 B2-KD: the kernel design for B2 (documents and code reading only)

TASK (Type 2), I94, a designer for ROOT (HELP_HUMAN, Agent 0), who is the return path. I am a fresh instance and made no delegation. 2026-10-07 UTC.

**The brief:** `R/BRIEFS/B2KD_KERNEL_DESIGN.md`, sha256 `c7b84097648369c548d46e35e6c2750e31ed0d16b4d693d6606a9519055f141e`, verified before reading. I read NUM's `AGENTS.md` and `agents/AGENT_TASK.md` first.

**What I did.** Documents and code reading at NUM's maintained tree, plus one small standard-library Python check run once with VENV in scratch (`_run_records/`). No cargo, no native or solver job, no install, no Git write; Git reads used `GIT_OPTIONAL_LOCKS=0`. Scratch was `WT/scratch/i94_b2_kd/` only. Nothing went to the system temp directory. **No code is proposed as maintained text; the sketches below are design sketches.**

**Notation.** WT, NUM, P, PP (= `P/core/product_physics/src`), T, R, RR and VENV as in the dispatch. Also:
- **FK** = `P/core/solver/frame_kernel`, the solver kernel crate PP depends on (`open_pipe_stress_frame_kernel` in PP's `Cargo.toml`). **FKR** = `FK/src/structural/retained` (the plan's "FK"). **FKT** = `FK/tests`.
- **SC1, C1, C2, C3** = `R/I32/f2a_wire_c1/SOURCE_COMBINATIONS.md`, `R/I32/f2a_wire_c1/WIRE_CONTRACT.md`, `R/I32/f2a_wire_c2/CONTRACT_DELTA.md`, `R/I52/prepared_public_contract_02/C3_DELTA.md`. **DN** = `T/DESIGN_NUMERICS/DESIGN.md`.
- **PLAN** = `R/I93/b2b3_plan_01/PLAN.md`; **REV** = its `REVISION_01.md`; **RV114** = `R/REVIEW_RV114/b2b3_plan_01/REVIEW.md`.
- **DEF-O** = `P/fixtures/results/retained_precision_prepared_ordinary_v1.json` (`RP-PREPARED-ORDINARY-DUAL-v1`). **DEF-C** = the suggested `RP-PREPARED-COMBINATION-DUAL-v1` (REV §1.1).
- "Combination" means a `mechanics` combination unless stated. h = the number of operands (repeats counted). N_g = the exact combined net at global DOF g: Σ_i c_i Σ_{j: dof(ij)=g} v_ij.
- **P2** = this design's prescription class: every operand's prescribed value is zero (§3.2).

## Basis

| Input | sha256 (verified) |
|---|---|
| The brief | `c7b84097…055f141e` |
| PLAN; REV | `e1147dbd…8a1238a`; `63abb73f…e8bba0c` (both match RR's acceptance) |
| RV114 | `bc6918ce…2a9ee6` |
| SC1; C1; C2; C3 | `28bc4dd6…4054`; `c8ab2318…67e3`; `923da0b9…0869`; `fd00d2c1…292e` (equal to PLAN §9's citations) |
| DN | `fb62ef4a…` (combination passages only) |
| I42 RETURN; I42 RV56-F1 repair RETURN | `191396ad…2b560`; `5bf98ec7…687a4ae` |
| I43 RETURN | `cf3b682b…f865` |
| I44 RETURN | `8a1ef094…2ed3f` |
| RV56 REVIEW, DERIVATION; RV56 repair RETURN | `9add0288…06c1`, `c7a2b54e…996b`; `25db9a2a…f334` |
| RR (read: "B2/B3 R1…", "I93's REVISION_01 accepted…" and the sections they cite) | `2b82a0b1…8d9c` (whole file, as read) |
| DEF-O (raw file) | `3e0779a4…e296` |

**Code**, cited by symbol and line at NUM `098b63b768`, whose `P/core`, `P/fixtures` and `P/schemas` trees equal main `2007709549` (`git diff --quiet`, checked). Files read in full or in the cited parts (sha256 prefixes): FKR `combine.rs` `730b7d24`, `origins.rs` `c4bde5de`, `adaptive.rs` `6a2fc382` (CasePrep, ExecutionOutcome, RetainedSolve, GroupCache, solve_cases, run_core, SourceBridgeView, source_correction), `ledger.rs` `20c3b86a`, `source.rs` `9956c084` (encodings), `bound.rs` `bdffbeb8` (data blocks), `verify.rs` `66022cc7` (prescribed terms), `product_certificate.rs` `64e09224`, `product_certificate/bridge.rs` `0cbc2049`, `source_residual.rs` `3f66c03f`, `final_case.rs` `d4dbaf3c`; `FK/src/structural.rs` `39164ca2`; `FK/src/structural/retained_resource.rs` `f8471544`; FKT `retained_k4/product_certificate_vectors.py` `88fc0765`, `source_residual_vectors.py` `5242da1e`, `source_residual_tests.rs` `eadf3bca`, `s11_site_table.rs` `2ee1079d`; PP `preview_physics.rs` `2b826540` (combination rows), `lib.rs` (combination records), `retained_product.rs` (row binding, capture), `retained_wire.rs` and `retained_memory.rs` (the consumers named in §1.1).

**B1's branches.** `codex/piping-t3-b1-20261007` at `603e238517` changes no file under `P/core/solver` against main (`git diff --stat`, empty). Its PP keeps the same kernel consumers (`for_calls`, the exhaustive `OriginError` and `NativeOwner` matches). So lane K's base at J0 is main's FK.

## 0. Findings in brief

1. **The change is small in arithmetic and moderate in custody.** The combination's own native solve is already correct for combinations (K4, I12 F-1): its schedule, residuals, verification and reactions read only the owner's combined ledger and combined prescription terms. What is missing is (i) a way to name operands that are not selected solves, (ii) the custody that registers and checks them, (iii) the product owner generalized to combinations, (iv) the certificate's two load-read sites and its row-coverage rule.
2. **Two constraints the plan did not name decide the API shape** (§1.1):
   - **in-build layout:** `FK/src/structural/retained_resource.rs` exports `size_of` of `CasePrep`, `RecordedInvocation`, `RetainedSolve`, `SourceOrigin`, `CallOrigin`, `ProductCertificateSpent` (with its inline `ResidualWork`) and others; PP's registered profile (`retained_memory.rs` `ATOM_VALUES`, `PINNED_RECORD`) is built from them. **Any size change is a registration change, so B2-K must be layout-neutral;**
   - **PP's exhaustive matches** on `OriginError`, `ViewFailure`, `BridgeFailure`, `ProductFailureView`, `ReadoutLaw`, `GroupPreparation` and `NativeOwner`, and its calls to `OriginCapacity::for_calls`: **no new variant in those enums and no changed signature**, or PP stops compiling at J3.
3. **The API** (§1): `PreparedCaseSource` (a `CasePrep` only), `CombinationOperand::{Retained, Prepared}`, `RetainedCombination::solve_sources`, and, recorded, `OriginCapacity::for_invocation`, `RecordedInvocation::register_prepared_source` and `solve_combination_sources` with `RecordedOperand::{Selected, Prepared{source, prepared}}`. `solve` and `solve_combination` become byte-identical wrappers. One new unit variant, `CombinationReason::NoSelectedOperand`.
4. **S-2** (§2): (a) **rebuild** an unavailable operand's prep with `PreparedCaseSource::new` and check its full K4SRC bytes against the registered source; `ExecutionOutcome` unchanged. (b) **register** a `not_required` operand's source with `register_prepared_source`, counted in `OriginCapacity`'s existing `cases` field (native case ordinals) through a new constructor, so no struct grows. (c) **the first selected operand's group** in authored order; operand 0 stays the representative.
5. **The combination source view** (§3): the only numerical change is the load term. The residual (I44) reads the **combined exact ledger** (K4LED nets, enclosed outward at 1024 bits) instead of the representative's individual load terms, at its two read sites (free residual rows; reaction offsets). Prescriptions are exactly zero by the view's check (P2). Tightening (I43) and the I42 bridge need no change. `UnsupportedCombination` is lifted for P2 and kept, with its existing wire tag, for any combination with a nonzero prescribed term.
6. **The certificate** (§5) is DEF-O's dual readout applied to the combination's own exact problem: K u = N and G u = N with u_C = 0. Its only new error term is the outward enclosure width of each net N_g (≤ 2^(⌊log2|N_g|⌋−1023), zero when N_g has at most 1024 significant bits). It uses no operand radius, row, state or certificate.
7. **A coverage rule the plan did not name** (§5.2): a mechanics combination publishes **no circular maximum, no intensified row and no mode, parity or modulus record** (PP `preview_physics.rs:854–1010`, `lib.rs:10019`). DEF-O's `row_scales` requires each of those for a case, so the combination owner needs its own coverage rule (20 of 21 derivative slots per member; no records).
8. **One existing FK test pins the old refusal** (`source_residual_combination_and_missing_uniqueness_are_explicit_refusals`, FKT `retained_k4/source_residual_tests.rs:787`). It must be re-pinned; that is the plan's risk 9 by letter, and needs ROOT's declared exception.
9. **B3b** (§8): **no exact annulus version is needed for anything here** (combinations never reach the exact route), and FK's annulus preparation is route-independent. One FK gap B3-D should know: the public `ProductMaterial` has no E/ν route, although the internal `MaterialOperands::ExactENu` and its oracle exist.
10. **B2-K's refined estimate: 18–28 h** (REV: 14–24 h), mainly from the oracle, the coverage rule, and layout-neutral custody (§9.2).

## 1. The prepared-operand API and its recorded variant

### 1.1 Constraints found in code

| Constraint | Evidence | Consequence for B2-K |
|---|---|---|
| **In-build strides** | `retained_resource.rs` exports `CASE_PREP`, `RECORDED_INVOCATION`, `RETAINED_SOLVE`, `RECORDED_CASE`, `SOURCE_ORIGIN`, `CALL_ORIGIN`, `GROUP_ORIGIN`, `BUILD_ORIGIN`, `RUN_ORIGINS`, `PRODUCT_OWNER_STAMP`, `PRODUCT_CERTIFICATE_SPENT`, `PRODUCT_ROW_SPEC`, `PRODUCT_FINAL_ROW`, `PRODUCT_MEMBER_FACTS`, `LEDGER_NET` and others. PP `retained_memory.rs:1656, 1705, 1768, 1770, 1776` build atoms from them; `retained_memory_law_tests.rs` `profile_in_build_record` asserts `PINNED_RECORD` in the registered identity build | **No field is added to any of these types or to anything stored inline in them** (`OriginCapacity` inside `RecordedInvocation`; `ResidualWork` and `SourceBridgeViewWork` inside `ProductCertificateSpent`; `RetainedLedger` inside `CasePrep`). New behaviour goes into new types, new functions and derived state |
| **PP's exhaustive matches** | PP `retained_wire.rs:471–478` (`ViewFailure`), `:485–494` (`BridgeFailure`), `:508–519` (`ProductFailureView`), `:573–579` (`OriginError`), `:1243` (`ReadoutLaw`), `:1385` (`NativeOwner`), `:1389` (`GroupPreparation`) at main; the same matches on `b1` | No new variant in those enums. `CombinationReason` is matched exhaustively nowhere outside FKR (FK tests use `assert_eq!`; the harness test only imports it), so one unit variant is admissible |
| **PP's calls** | `OriginCapacity::for_calls(&[n], &[])`, `RecordedInvocation::{new, solve_cases, certify_product_case, begin_prepared_product}`, `prepare_product_annulus` (main and `b1`) | Signatures unchanged; new entry points are additive |
| **S11 site table** | FKT `s11_site_table.rs` scans `combine.rs`, `origins.rs`, `adaptive.rs`, `ledger.rs`, `final_case.rs`, `source_residual.rs` and others for `+=`, `-=`, `.sum(`, `fold(` and self-assignment folds, with an exact count per function | New code avoids those shapes (use `checked_add` and `WorkTotal::add` as the existing code does). A new counted site is a visible table row, added in lane K |

### 1.2 Types and functions (sketch, additive)

```rust
// FKR/adaptive.rs
/// A case's precision-independent preparation only: CasePrep::new, exactly as
/// solve_cases builds it. No solve, factor, group, cache, verification or radius.
#[derive(Debug)]
pub struct PreparedCaseSource { prep: Arc<CasePrep> }
impl PreparedCaseSource {
    pub fn new(source: PrimitiveSource) -> Result<Self, LedgerRefusal>; // CasePrep::new
    pub fn source(&self) -> &PrimitiveSource;
    pub fn identity(&self) -> &[u8];               // K4SRC bytes = source.encoding()
}

// FKR/combine.rs
pub enum CombinationOperand<'a> {
    Retained(&'a RetainedSolve),
    Prepared(&'a PreparedCaseSource),
}
pub enum CombinationReason { /* existing variants unchanged */, NoSelectedOperand }
impl RetainedCombination {
    pub fn solve_sources(operands: &[(f64, CombinationOperand<'_>)],
        case_limit: CaseLimit, meter: &mut InvocationMeter) -> CombinationOutcome;
    pub fn solve_sources_recorded(operands: &[(f64, CombinationOperand<'_>)],
        case_limit: CaseLimit, meter: &mut InvocationMeter) -> RecordedCombination;
    // unchanged signatures, now wrappers over the same core (all Retained):
    pub fn solve(..) -> CombinationOutcome;  pub fn solve_recorded(..) -> RecordedCombination;
}

// FKR/origins.rs
impl OriginCapacity {
    /// cases = Σ case_batch_lengths + prepared_sources (native case ordinals);
    /// runs = Σ case_batch_lengths + combinations; builds = 7·runs; others as for_calls.
    pub fn for_invocation(case_batch_lengths: &[usize],
        combination_operand_lengths: &[usize], prepared_sources: usize)
        -> Result<Self, OriginError>;
    // for_calls(a, b) == for_invocation(a, b, 0), byte for byte.
}
pub enum RecordedOperand<'a> {
    Selected(&'a RetainedSolve),
    Prepared { source: usize, prepared: &'a PreparedCaseSource },
}
impl RecordedInvocation {
    /// Registers a prepared case source: one SourceOrigin, owner Case(next native
    /// case ordinal), no Call, no Run, no meter change. Returns the source id.
    pub fn register_prepared_source(&mut self, prepared: &PreparedCaseSource)
        -> Result<usize, OriginError>;
    pub fn solve_combination_sources(&mut self, operands: &[(f64, RecordedOperand<'_>)],
        limit: CaseLimit) -> Result<RecordedKernelCombination, OriginError>;
    // solve_combination(&[(f64, &RetainedSolve)]) keeps its signature; it maps every
    // operand to RecordedOperand::Selected and calls the same core.
    pub(crate) fn product_owner(&self, run: usize, solve: &RetainedSolve) -> Option<NativeOwner>;
}
```

Notes on the choices:
- `PreparedCaseSource::new` returns the existing `LedgerRefusal`, because `CasePrep::new`'s only failure is the ledger's; `solve_cases` maps the same failure to `Refusal::LedgerUnavailable` (`adaptive.rs` `solve_cases_projected`). C3a's `operand_preparation_failure` (B2-C) wraps it. SC1's provisional `PreparationFailure` would add a type with one variant.
- `RecordedInvocation::new` reserves `store.sources` at `cases + combinations` (checked) instead of `runs`. Under `for_calls` the two are equal, so today's reservation is unchanged. `solve_cases` gains a dormant check that case runs never exceed `runs` (it cannot fire under `for_calls`; it stops a prepared ordinal being spent as a batch case).
- `RecordedOperand::Prepared` carries the registered source id explicitly. Finding the source by identity bytes would be ambiguous, because two cases may have equal K4SRC bytes and are still different owners (C2 §3).

### 1.3 Invariants

| # | Invariant | Where it is enforced |
|---|---|---|
| I1 | A `PreparedCaseSource` is exactly `CasePrep::new(source)`: no solve, factor, group, cache, verification report or radius; immutable behind its `Arc` | Constructor; no other constructor; fields private |
| I2 | `solve(ops)` ≡ `solve_sources(ops as Retained)` and `solve_combination(ops)` ≡ `solve_combination_sources(ops as Selected)`: identical outcome bytes, attempts, work, meter and recorded custody | One core; C01 |
| I3 | No operand is solved, mutated or re-prepared by a combination call; one `PreparedCaseSource` may serve several combinations (decision 6: one shared `OperandPreparation`) | Borrowed operands; C04 |
| I4 | The combination's group is the first `Retained`/`Selected` operand's `GroupPrep` in authored order; its cache is `GroupCache::merged` over those operands' caches in authored order; prepared operands contribute no slot; new slots stay local | Core; W06, K-05 |
| I5 | The representative source (operand 0's, any kind) supplies only stiffness, layout, stations, supports, members, springs and the constrained DOF set. **Its loads and constraint values are never used as the combination's**: the solve uses `prep.ledger` and `prep.prescribed`, and the certificate uses the ledger's nets and zero prescriptions | Solve (already true); §3 |
| I6 | One `SourceOrigin` per native case ordinal (batch or prepared) and one per successfully prepared combination; a prepared registration has no Call, no Run and no meter change | `register_prepared_source`; K-02 |
| I7 | Each `Prepared{source, prepared}` names an in-range registered source with owner `Case(_)`, no combination ledger, and identity bytes equal to `prepared.identity()` (full-byte comparison). The kernel does not require that source to have no selected run: a case whose kernel run selected but whose freeze failed is publicly `unavailable`, and decision 6 sends it as a prepared operand | Custody; K-03 |
| I8 | A combination is never selected without at least one `Retained`/`Selected` operand (decision 5) | `NoSelectedOperand`; K-04 |
| I9 | The product certificate accepts a combination owner only through a recorded selected run whose owner is `NativeOwner::Combination` and whose prep has nonempty factors; a case owner only with empty factors | `product_owner`; K-06 |
| I10 | Layout and enum neutrality (§1.1) | J3 acceptance (§9.1) |

### 1.4 How a combination's operands are named and checked

**Naming.** In the kernel an operand is its authored position i, its factor bits c_i, and a handle: a selected solve (`Retained`/`Selected`) or a prepared source (`Prepared`, with its registered source id in the recorded API). The recorded call stores `RequestedOperand { source, factor_bits }` per operand in authored order, unchanged in shape: `source` is the selected origin's source id for a selected operand, or the registered source id for a prepared one (None when out of range). PP maps these ids to C2's `requested_operands[].source_ref` and `CombinationSource.operands[] = {case_index, factor, source_ref, source_identity_sha256}`, with `representative_source_ref` = operand 0's source (C2 §3). Repeated operands stay repeated.

**Checks, in order** (the first failing check decides):

| Step | Unrecorded `solve_sources` | Recorded `solve_combination_sources` | Result |
|---|---|---|---|
| 0 | — | Capacity: calls, combinations, operands | `Err(OriginError::Capacity)`, nothing recorded (as today) |
| 1 | Empty, or any non-finite factor | same, after the Call is recorded | `NoOperands` (pre-source, `OperandValidation`) |
| 2 | Any operand prep with nonempty factors | same | `NestedCombination` |
| 3 | Any operand differs from operand 0 in K4STF bytes, layout, stations or supports | same | `OperandsDiffer` |
| 4 | No `Retained` operand | No `Selected` operand | `NoSelectedOperand` (new; pre-source, `OperandValidation`) |
| 5 | — | Per operand in authored order: `Selected` → today's check (`Arc::ptr_eq` with a selected origin, equal identity bytes, cache matches its finish snapshot); `Prepared` → I7 | `OriginRefusal(MissingSelectedOrigin{operand})` (ROOT ruling R-1) |
| 6 | `CasePrep::combination` over the operands' preps in authored order | same | `LedgerUnavailable` / `CountRange` (pre-source, `CombinedPreparation`) |
| 7 | Run on the first selected operand's group with the merged cache | same, with origins, imports from selected operands only, and `finish_run` | `WithRun` |

Steps 1–3 and 6 are today's, in today's order (SC1 §1: no operands → nested → operands differ), applied to `&CasePrep` whatever the operand kind. Step 4 comes after native validation and before custody, so an all-prepared request with differing operands still reports `OperandsDiffer`.

### 1.5 `NativeOwner::Combination` as a product owner

`product_case_owner` (`origins.rs:795–800`) becomes `product_owner(run, solve) -> Option<NativeOwner>`: the same selected-origin conditions (run, `Arc::ptr_eq` on the prep, identity bytes, cache snapshot), plus the owner-kind consistency of I9; it returns the run's owner. `product_owner_stamp`, `certify_product_case` (`final_case::certify` → `run_case`) and `begin_prepared_product` accept either kind. `ProductOwnerStamp { prep, run }` and `ProofAnchor` are unchanged in type and size. The certificate learns the owner kind from `owner.prep.factors.is_empty()`, already checked against the recorded owner.

`ProductFinalRow.case_id` and `ProductRowSpec`'s `case_id` carry the owner's id; for a combination that is the combination id. FK only compares these strings for equality (`final_case.rs:1163`), so no rename is needed.

### 1.6 What stays byte-identical

- Every case path: `solve_cases`, `solve_case`, recorded batches, the case view, residual, bridge, correction and final certificate, including their work counts (receipt bytes).
- Every all-selected combination: outcome, attempts, work, meter, and the recorded Call/Source/Group/Build/Run origins (I2).
- `ExecutionOutcome`, `CaseOutcome`, `CombinationOutcome`, `RecordedCombination` and every type in §1.1's first row.

## 2. S-2's three kernel facts

### 2.1 (a) An unavailable operand's prep

**Fact.** `ExecutionOutcome::Unresolved` and `Refused` hold attempts and geometry but no `Arc<CasePrep>` (`adaptive.rs:4404–4416`). The source *was* registered by `solve_cases` before its run (`origins.rs:364–376`).

| Option | What it costs | Verdict |
|---|---|---|
| **(a1) Rebuild** with `PreparedCaseSource::new(source)` from PP's retained prepared `PrimitiveSource`, and check its full K4SRC bytes against the registered `SourceOrigin.identity` (I7) | One more `CasePrep` per unavailable operand (memory, priced at SQ2; no LME); a byte comparison of the K4SRC length. `CasePrep::new` is deterministic, so the rebuilt prep equals the one the run used | **Recommended** |
| (a2) Retain the prep in `ExecutionOutcome::{Unresolved, Refused}` | Changes a pub enum stored inline in `RecordedCase` (atom `RECORDED_CASE`, §1.1) and brings SC1 §4's consumers (H, VR, FK tests) into lane K | Rejected: layout change and a wider write set |
| (a3) `PreparedCaseSource::from_selected(&RetainedSolve)` sharing the prep of a kernel-selected, freeze-failed case | Saves one rebuild in that one case; adds a second constructor and path | Not needed; (a1) covers it uniformly |

**Consequences.** A source whose `CasePrep::new` failed (run phase `SourcePreparation`, `Refusal::LedgerUnavailable`) fails again on rebuild, deterministically: that is C3a's `operand_preparation_failure` and the combination becomes `retained_unavailable` (decision 7). An unavailable case without a registered source (T-7) has no operand (decision 6).

### 2.2 (b) Registering a `not_required` operand's source

**Fact.** Sources are pushed only by `solve_cases` and `solve_combination` (`origins.rs:367, :488`), and `store.sources` is reserved at `capacity.runs` = cases + combinations (`:319`, `:56–58`).

| Option | What it costs | Verdict |
|---|---|---|
| **(b1) `for_invocation(batches, operands, prepared)`** whose existing `cases` field counts native case ordinals (batch + prepared); `store.sources` reserved at `cases + combinations`; `register_prepared_source` pushes one `SourceOrigin { owner: Case(used_cases), identity: K4SRC, stiffness: K4STF, combination_ledger: None }` and advances `used_cases` | No new field anywhere; `for_calls` unchanged in signature and effect; PP maps the new ordinals to request indices (B2-P) | **Recommended** |
| (b2) A new `prepared` field in `OriginCapacity` and a `used_prepared` counter in `RecordedInvocation` | Grows `RecordedInvocation` (atom `RECORDED_INVOCATION`) | Rejected: layout change |
| (b3) No registration; prepared operands carry no `SourceOrigin` | C2 §3's "registered at construction" fails; `source_ref` would not resolve | Rejected |

**Consequences.** Registration is not a Call and touches no meter (C3a: preparation is outside LME). It may happen at any time; PP does it after the case batch and before the first combination Call (C3a's placement). A registration beyond the declared count is `OriginError::Capacity`.

### 2.3 (c) The combination's group

**Fact.** `RetainedCombination::solve` and `RecordedInvocation::solve_combination` run with `operands[0]`'s `GroupPrep` (`combine.rs:140–142`; `origins.rs:533`). SC1 §1: "the selected operand need not be first".

| Option | What it costs | Verdict |
|---|---|---|
| **(c1) The first selected operand's group in authored order** | None: decision 5 guarantees one; `GroupPrep` (structure, ordering, geometry, blocks) depends only on the stiffness source, which step 3 makes equal for all operands. For all-selected requests it is operand 0's, as today | **Recommended** |
| (c2) A fresh `prepare_group(representative)` | Repeats geometry, structure and ordering; may refuse; imported cache slots must then match a second, equal ordering | Rejected |
| (c3) Require operand 0 to be selected | Forbids a prepared first operand; authored order is semantic (K4CMB, `representative_source_ref`) | Rejected |

**Consequences.** The combined `CasePrep` keeps operand 0 as representative (`CasePrep::combination`, `adaptive.rs:961–1009`), whatever its kind (I5). The combination's `RetainedSolve.group` is that first selected operand's `Arc<GroupPrep>`, which the view and the correction then use (§3).

## 3. The combination source view (bridge, residual and tightening)

### 3.1 Where the case path reads load and prescription data

| Site | Reads | Combination-correct today? |
|---|---|---|
| Native solve: `reduced_rhs`, residuals, recovery reactions, verification (`assemble.rs:826–841`, `recover.rs:419–426`, `verify.rs:799–833`) | `prep.ledger` (combined) and `prep.prescribed` (h terms per constrained DOF) | **Yes** (I12 F-1). The representative's `loads()` are read only by `CasePrep::new`, the combination's count and a test-only seeded fault |
| View: prescription check and data flags (`adaptive.rs:5483–5496`, `fill_data_blocks`) | `prep.prescribed` must be `[(1, c.value)]`; flag `c.value != 0` | **No**: refused earlier at `:5359–5361` |
| Residual, free rows (`source_residual.rs:622–629`) | the representative's individual `loads()` | **No**: operand 0's loads |
| Residual and recovery, constrained coordinates (`gather`, `:490–512`) | the representative's `constraint(g)` value | Only if the combined value equals operand 0's |
| Recovery, reaction offsets (`:1161–1169`) | the representative's individual `loads()` at constrained DOFs | **No** |
| Correction (`adaptive.rs:5733–5767`) | the owner's cache slot at P and its scales | **Yes** (K-only) |
| I42 bridge `motion` and its InputDerived check (`bridge.rs:156–174, :695–706`) | the representative's `constraint(g)` value; no loads | Only if the combined value equals operand 0's |
| K lane InputDerived rows (`final_case.rs` `run_case`) | the published prescribed value | Yes (published from the exact combined terms) |

### 3.2 The view's combination branch (FKR `adaptive.rs`)

The blanket refusal at `:5359–5361` is removed. Every other check keeps its order and its work. At the prescription check (`:5483–5496`), a combination owner (`!prep.factors.is_empty()`) is checked instead by:
1. `prep.prescribed.len() == source.constraints().len()`, and per constrained DOF, in order: the DOF matches; `terms.len() == prep.factors.len()`; `terms[i].0.to_bits() == prep.factors[i].to_bits()` for every i. Otherwise `CertificateIssue::PairIdentity`, as for a case.
2. **P2:** every `terms[i].1 == 0.0`. Otherwise `SourceBridgeViewIssue::UnsupportedCombination` (the existing variant and wire tag, now meaning "a combination outside the zero-prescription class").
3. The data flags' prescribed vector is all false. That equals the native verification's own predicate (`verify.rs:819–823`: the exact sign of Σ c_i v_i), so every data block the view finds has its certified bound and θ in the combination's own evidence.

Visits: one per constrained DOF and one per term examined. No new field in `SourceBridgeViewWork`.

**Why P2, not the general sum.** DEF-O binds `prescriptions: exact_positive_zero`, C2 §3 states "W1a product operands have zero values", and REV §1.1 binds the same for DEF-C. Under P2 the representative's constraint value is exactly the combined value (both zero), so every reader of `constraint(g)` (residual `gather`, I42 `motion` and InputDerived) is already correct, and the published prescribed rows are exactly +0. The general alternative is in §3.8.

### 3.3 The residual (I44, FKR `product_certificate/source_residual.rs`)

Two branches, selected by `view.owner().prep.factors.is_empty()`; the case branch is untouched.

- **Free rows** (`residual`, after the member and before the spring contributions): for each free position a with DOF g, if the owner's ledger has a net N_g that is not exactly zero, `out[a] ← out[a] ⊕ [RD1024(N_g), RU1024(N_g)]`.
- **Reaction offsets** (`recover`): for each constrained DOF g with a nonzero net, `reaction[ci] ← reaction[ci] ⊖ [RD1024(N_g), RU1024(N_g)]`.
- **Constrained coordinates** (`gather`): unchanged; under P2 it returns exact zero.

**The enclosure primitive.** `[RD1024(N_g), RU1024(N_g)]` is formed as the existing directed operations are (`product_certificate.rs` `scalar_owned`): a fresh `WideContext<16>` at 1024 bits and a fresh `ExactWideSum`; `ledger.add_to(g, &mut sum, false)` (the exact integer net, as the native residual does); `directed::round_toward(.., Down)`; again for `Up`. Each endpoint is booked as one `Entry::Add` ("an exact sum rounded once toward a direction"), and the context and sum work are collected on every exit. A net with at most 1024 significant bits gives `lo == hi` exactly. The lookup is `RetainedLedger::net(g)` (no new ledger accessor), with one visit per DOF examined.

**Why the nets, not the operands' individual terms.** The nets are the owner's own K4LED, already bound by the view (`encoding_matches` against `evidence.ledger_encoding`), already used by the native solve, and exactly the quantity C2 §3 names. Using the operands' terms would need the operands' sources inside the certificate, which either grows `CasePrep` (layout) or needs a caller-supplied operand list re-verified against the K4CMB bytes. The nets are also at least as tight: one outward rounding of the exact sum instead of one per interval addition. Cancellation stays visible where it matters: the data flag is per individual product (`ledger.rs:225`, `factor != 0 && value != 0`), so A − A2's cancelled DOFs remain data blocks (as I42/RV56 require for cancelled terms).

### 3.4 Tightening (I43): no change

`SourceBridgeView::source_correction` applies the owner's verification-precision factor (`cache.s{P}`) once. The factor is K-only, and the combination's cache slot at P was imported from a selected operand or built by the combination's own run; the view's existing slot checks bind it (`checked_slot!`). The I43 theorem is valid for any finite center (I43 §3), so nothing in it depends on the load vector except the residual of §3.3.

### 3.5 The I42 bridge: no change

`product_certificate/bridge.rs` never reads loads (the reaction offset cancels between u_K and u_G; RV56 DERIVATION "Identical nodal ledger terms cancel in the reaction offset"), and reads prescribed values only through `constraint(g)`, which is exact zero under P2. Once the view admits a combination owner, the bridge is valid for it unchanged. It is not on the product path (only tests call `source_bridge`); one test control is enough (§7).

### 3.6 `UnsupportedCombination`: lifted, and kept for what B2 does not cover

| Combination | View outcome after B2-K |
|---|---|
| Every operand prescription zero (D1, DEF-C) | Admitted; checked as §3.2 |
| Any nonzero prescribed term, in any operand (including one where operand 0's value is zero) | `UnsupportedCombination` (existing variant; C3's `unsupported_combination` tag) |
| Nested combination | Cannot be selected (`NestedCombination` at solve) |
| Factor or term shape mismatch (corruption) | `PairIdentity` |

### 3.7 Work, storage and receipt shape

- **Numeric entries:** each nonzero net costs two `add` entries per pass (residual: two passes over free DOFs per lane; recovery: one pass over constrained DOFs per lane), plus their actual wide and sum work. No eighth entry kind, so C3's `NumericTrace.entries:[Count;7]` shape serves DEF-C unchanged.
- **Storage:** no new vector; the nine `ResidualWork.capacities` slots are unchanged (C3's `CapacityName` list holds).
- **Visits:** one per DOF examined and per prescribed term checked.
- **Layout:** no new field in `ResidualWork`, `SourceBridgeViewWork`, `SourceBridgeView` or `ProductCertificateSpent`.

### 3.8 Alternatives considered

| Alternative | Why not now |
|---|---|
| **P1: general combined prescriptions** (enclose Σ c_i v_i per constrained DOF; data flag by its exact sign; InputDerived rows hold only when the sum is binary64-representable) | Outside D1 and DEF-C; needs `gather`, the I42 bridge, the K-lane InputDerived row and the data flag to change, plus a review. About +3–5 h. The view's refusal keeps it a declared limit, not a silent one |
| Operands' individual products c_i·v_ij in the residual | §3.3: needs operand sources (layout or re-verification) for no gain |
| Rounded or p-precision nets | Unsound: the residual must contain the exact load (I43 §3) |

## 4. The combination definition's numerical content (for B2-C)

B2-C writes the JSON (`P/fixtures/results/retained_precision_prepared_combination_v1.json`, REV §1.1). Mirroring DEF-O member by member, it must bind:

| DEF-O member | DEF-C content |
|---|---|
| `id`, `version` | `RP-PREPARED-COMBINATION-DUAL-v1`, 1 (name reserved by ROOT at B2-C) |
| `inherits` | DEF-O's eight values unchanged (base and semantic contracts, kernel policy, method token, projection, work and facade policies, canonicalization), **plus** `operand_definition: RP-PREPARED-ORDINARY-DUAL-v1` with the hash PTABLE binds: every operand source is a DEF-O prepared source (selected and unavailable operands through their CaseSource `preparation`; `not_required` operands through C3a's `OperandPreparation`) |
| `scope.owners` | `mechanics_combination` (T0R-admitted, ≥ 1 selected operand after T-9, decision 5) |
| `scope.loads` | `combined_exact_ledger`: for each operand i in authored order and each of its individual normalized nodal terms j (CaseSource `nodal_terms`, canonical order), the exact product c_i·v_ij, summed exactly per DOF (K4LED; `RetainedLedger::combined`). No binary64 product or net is formed |
| `scope.prescriptions` | `exact_positive_zero` for every operand term; a nonzero term is refused by the kernel view (`unsupported_combination`) |
| `scope.operand_sources` (new) | selected operand → its solve (cache imports allowed); unavailable-with-source operand → its registered source, rebuilt and identity-checked; `not_required` operand → one registered `OperandPreparation`, shared; no operand is solved for a combination |
| `scope.operand_equality` (new) | equal K4STF bytes, layout, stations and supports (kernel `OperandsDiffer`) **and** equal prepared section facts (normalized D, effective wall, material selection, and prepared A, I, J, Z, c bits) across all operands and the combination's own facts. The kernel checks only K primitives; t_eff and Z are not in K4STF, and two nearby t_eff bit patterns can round to the same A, I and J (ROOT ruling R-8) |
| `scope.materials`, `supports`, `source` | as DEF-O (the base gate `COMBINATION_MODULUS_BASIS_MIXED` keeps one modulus basis) |
| `scope.excludes` | DEF-O's list minus `prepared_combinations`, plus `result_state_subtraction`, `range_envelope`, `nested_combinations`, `combination_maxima`, `intensified_rows`, `nonzero_operand_prescriptions` |
| `preparation` | none for the combination (its ProductAttempt's preparation stage is `not_entered`; REV §1.1 item 2) |
| `lanes` | DEF-O's, with the load law replaced by: "each loaded DOF's exact combined net enclosed outward at 1024 bits, in free residual rows and constrained reaction offsets; prescribed coordinates exact zero"; owner = the combination's recorded run, K4CMB source, cache and one non-reused anchor |
| `precision` | as DEF-O, for the combination's own native schedule (stop rule with S\* from its own body scale; C1 §2's 20B case limit per combination) |
| `projection` | as DEF-O |
| `rows` | `component`, `component_stress`, `displacement_magnitude`, `support_magnitude`, `prescribed` as DEF-O; **`maximum`: not published** (PP emits `COMBINATION_STRESS_MAXIMUM_UNAVAILABLE`); **`ancillary`: none** (no mode, parity or modulus record for a mechanics combination) |
| `acceptance` | as DEF-O's `final`, for the combination's own exact problem; plus: "no operand radius, row, retained state or certificate is used; a combination's rows are not the factor-weighted sum of its operands' published rows" (DESIGN §4.2's disclosed property) |
| `hash_domains` | recommend reusing `retained_precision_formation_v1` for the definition object (a domain names an object type, and the ids differ); the receipt, publication and source domains as DEF-O; no preparation domain (R-10) |
| `trust` | G7 unchanged; G8 adds the combination expression (terms and factor bits in authored order) equal to the invocation's, operand source identities, K4CMB through `kernel_source_sha256`, and `ledger_sha256` = sha256(K4LED). Optional for B2-C: an exact reader recomputation of K4LED from the operands' `nodal_terms` and factor bits (exact integer arithmetic in all three languages) |
| `work` | as DEF-O without preparation; combined-net enclosures are counted as `add` entries |

## 5. The final-row certificate for combinations

### 5.1 The claim

Let O be a combination owner admitted by `product_owner` (§1.5): h ≥ 1 factors c_i, representative source S, combined exact ledger N (O's K4LED), every prescribed term zero (§3.2). Let the member facts Φ (D, t_eff, material, A, I, J, Ẑ, c) pass the existing checks against S's members (`final_case.rs` `run_case` and `begin_prepared_product`: A, I, J, radius = D/2 bits; material E/G bits in `build_member`). For each law L ∈ {K, G}, let u^L be the exact solution of

  L_FF u_F = N_F, u_C = 0,

where K is the admitted law (exact products E·A, G·J, E·I of S's members, exact frames) and G is the annular source law from Φ, with S's springs and supports. Then **for every row r of the combination's final row set (§5.2) with a mechanical recipe ρ_r:**
- q_r^L ∈ E_r^L for both lanes, where q_r^L = ρ_r(u^L) is the exact row value; and
- the published normalized value n_r satisfies DEF-O's unchanged final predicate on H_r = hull(E_r^K under the represented recipe, E_r^G under the geometric recipe): h_r ≤ b_r for an absolute row (b_r = RU64(2^-64 S\*), or the small-scale bound); h_r ≤ A_exact, h_r ≤ A_f64, 10^9 h_r ≤ |n_r| and the raw decimal test for a relative row; h_r = 0 for an InputDerived row, where h_r = sup_{q ∈ H_r} |n_r − q|, computed upward.

S\* is the combination's own body scale from its own final values (`row_scales`, `couple`, the p = 512 floor). By linearity and the common operator, u^L = Σ_i c_i u_i^L, so for every linear row q_r^L = Σ_i c_i q_{r,i}^L: the certificate is about the exact superposition of the operands' exact responses, but it never forms that sum from operand rows or states (SC1 C03).

### 5.2 Over which rows

The combination's final rows, as PP publishes them (`preview_physics.rs:854–1010`; `lib.rs:10019` emits `combination_modulus_basis_record` only for subtraction and range):

| Family | Rows | Recipe |
|---|---|---|
| Displacement components | 6n (constrained ones are InputDerived, exactly +0) | `Native(Displacement)` |
| Displacement magnitudes | n (combined vector magnitude) | `Native(DisplacementMagnitude)` |
| End and station actions | 12m + 18m (three stations per member) | `Native(EndAction / StationAction)` |
| Signed support components | 6g | `SupportComponent` (each reaction or spring native row is one contributor) |
| Support magnitudes | 2g | `Native(SupportForce/MomentMagnitude)` |
| Stresses | 20m (axial, bending y, bending z, torsion at 2 ends and 3 stations) | `Stress` |
| **Not published:** circular maxima (m), intensified rows, mode, parity and modulus records | — | — |

So a combination has 7n + 50m + 8g final rows, against a case's 7n + 51m + 8g plus one to three records. **The coverage rule in `row_scales` (`final_case.rs:1114–1288`) gains a combination branch:**
- `NonQuantity`, `DenseParityObservation`, `ModulusBasisRecord` and `CircularMaximum` rows are refused for a combination owner (`bad("combination row family")`);
- completeness requires every native row and derivative slots 0–19 of each member (slot 20, the maximum, must stay empty), and does not require the mode record;
- every other rule (unique ids, one owner id, bodies, units, support slots, native coverage exactly once) is unchanged.

Every covered row is certified; a missing, duplicated or extra row, or one failing predicate, fails the whole proof (the combination becomes `retained_unavailable` with a facade certificate failure, decision 7). If B2-C or B2-W finds a different published row set, the rule returns to ROOT (stop S-14).

### 5.3 The premises, re-checked for a combination owner

I43 §3's theorem and I44's implementation rest on these premises. Each holds for O as follows.

| Premise (I33/I43/I44) | For a combination owner O |
|---|---|
| Immutable owner, one anchor | O's `RetainedSolve`; `ProductOwnerStamp(prep Arc, run)` through `product_owner` |
| Source and ledger identity | `prep.identity` (K4CMB) = `evidence.source_encoding` = the recorded `SourceOrigin.identity`; `prep.ledger` encoding = `evidence.ledger_encoding` = `SourceOrigin.combination_ledger` |
| Maps, stations, supports, constrained DOF set, frame, springs | S's, equal for every operand by K4STF, layout, stations and supports equality |
| Prescriptions | exact zero for every term (§3.2) |
| Positive source laws | the fact checks against S's member bits; Φ equal across operands (DEF-C, R-8) |
| Complete pattern, ordering, blocks | the first selected operand's `GroupPrep`, equal to any operand's (same stiffness source) |
| Matching P = 2p verification factor | O's own cache slot at P (imported or built by O's run) |
| Bound B and θ per data block | O's own verification, with O's own data flags (its ledger's individual-product flags; zero prescriptions; its own state) |
| Rows, radii, classes | O's own publication |

### 5.4 Error terms and their derivation

For each lane, take the initial center x (K lane: O's published free displacements; G lane: the midpoints of the K lane's free-displacement enclosures, DEF-O's `annular_seed`), the corrected center y = RN1024(x + S·z0) on data blocks after the one verification-factor application (y = 0 on no-data blocks), and y_C = 0. Define the exact residual r(y) = N_F − L_{F,all} y. At both x and y the kernel encloses

  ρ_a ⊇ s_a · ( [N_g] − Σ_{m ∋ g} [(L_m y)_g] − Σ_{springs at g} [k]·y_g ),  g = free(a),

with [N_g] = [RD1024(N_g), RU1024(N_g)] (§3.3), member contributions from enclosed coefficients and exact frame enclosures, and outward 1024-bit operations; s_a = 2^scale(a) is exact.

1. **Solution radius** (I43 §3, unchanged): e = u^L − y has e_C = 0 and (S L_FF S) z = S r(y) with z = S^-1 e_F. Blocks do not couple through free pattern entries (the view checks this), so per data block c, ‖z_c‖∞ ≤ β_c/(1−α_c) · ω_c ≤ ε_c, with β_c = 2B (O's certified bound; the R7 transfer theorem), α_c = RU(β_c η_c) < 1 (η_c = 0 in the K lane; the |G−K| majorant in the G lane), ω_c = max over the block of |ρ| endpoints, and ε_c = RU(RU(β_c ω_c)/RD(1−α_c)). Hence u^L_a ∈ [RD(y_a − s_a ε_c), RU(y_a + s_a ε_c)]. The correction makes ε small, but the bound assumes nothing about the factor's accuracy: it holds for any finite y, because ω_c comes from the residual recomputed at y (I43 §3).
2. **No-data blocks:** no nonzero individual product at any free DOF of the block (so N is exactly zero there), no adjacency to a nonzero prescription (none exist), and the anchor warrant: then u^L = 0 on the block (RV56 DERIVATION, fixed-anchor uniqueness). A missing anchor refuses (`MissingUniquenessWarrant`).
3. **Recovery:** each row is a fixed functional of the displacement box under law L (end actions H^T D B x, station actions with exact fractions, member global actions summed into reactions, −k u for springs, norms with crossing-zero lower bounds), evaluated outward. **Reactions subtract [N_g] at constrained g.** So E_r^L ∋ q_r^L.
4. **Recipes and hull** (DEF-O unchanged): stresses by directed signed-corner division (represented A, Ẑ, J, c in the K lane; geometric section enclosures in the G lane); H_r = hull of the two lanes; projection RN1024 midpoint then RN64.

**The error terms in E_r^L, by source:**

| Term | Size | New for combinations? |
|---|---|---|
| E1 solution radius | s_a ε_c per free coordinate, carried into each row by Σ_c ε_c Σ_{a∈c} \|a_{r,a}\| s_a | No (its inputs are O's own) |
| E2 coefficient enclosure widths (G lane: π bracket, annulus interval arithmetic, material interpolation) | as DEF-O | No |
| E3 frame enclosure widths | zero for axis-aligned exact frames; outward otherwise | No |
| E4 outward 1024-bit rounding in residual, recovery and recipes | ≤ a few 2^-1023 relative per operation | No |
| **E5 combined-load enclosure width** δ_g = RU1024(N_g) − RD1024(N_g) | δ_g ≤ 2^(⌊log2\|N_g\|⌋ − 1023), and **δ_g = 0 when N_g has ≤ 1024 significant bits**. It enters ω_c as s_a δ_g at free g and reaction rows as δ_g at constrained g | **Yes** |
| **E6 prescription** | exactly 0 (P2) | **Yes** (by restriction) |
| E7 law gap \|q^G − q^K\| | covered by the hull, not bounded separately | No |
| E8 projection and unit normalization (RN1024 midpoint, RN64, mm and MPa scalings) | measured by the gate's actual distance h_r | No |

The only new term, E5, is negligible against any allowance (2^-1023 relative against at least 2^-64·max(|n|, S\*)), and it vanishes in every specimen except a deliberately wide one (§6: C6 needs 1201 bits). A case's residual adds individual terms with one outward rounding each, so the combination's term is never wider than a case's would be for the same exact loads.

### 5.5 How it composes with the operands' certificates

1. **It uses none of them.** No operand radius, published row, retained state, verification or certificate enters the combination's proof (C2 §3: "Combinations bind their own solve, never operand radii"). Operand cases and prepared operands contribute only their source data, through the combined ledger and the common stiffness.
2. **Its validity does not depend on operand status.** It holds whether each operand is selected, unavailable with a source, or `not_required`. Decision 5 (≥ 1 selected) is a routing rule, not a premise of the proof.
3. **Operands are unaffected.** A combination never changes an operand's values, evidence, cache snapshot or standing (C04; I3).
4. **Both proofs bound the same law pair.** The operands' and the combination's facts Φ are equal (DEF-C, R-8), so all of them bound K and the same G.
5. **A corollary, not a claim.** When every operand and the combination pass, for each linear row ℓ and either law, |n_comb − Σ_i c_i n_i| ≤ a_comb + Σ_i |c_i| a_i (exact arithmetic on the published binary64 values; a = each row's allowance). Magnitudes are not linear. B2 publishes nothing from this; readers must not derive reliance from operands for a combination or the reverse (C1 §6).

### 5.6 What it does not prove

The relation between published combination rows and published operand rows (disclosed, §5.5 item 5); anything for nonzero prescriptions, subtraction, range, maxima, intensified rows or nested combinations; that PP bound the combination to the authored expression (K4CMB identity, factor bits and G8 do that); memory or admission.

### 5.7 The certificate's code changes (FKR `final_case.rs`)

- `run_case` and `begin_prepared_product`: `product_owner` instead of `product_case_owner` (§1.5).
- `row_scales`: the combination coverage branch (§5.2).
- Nothing else: the lanes, seeds, projection, values builder (`complete_maxima(&[])` for an owner without maxima), `check_intervals`, `gate`, `summary_coverage_data` and the trace are owner-generic once the residual reads O's ledger.

## 6. The oracle (N-1)

**Home.** Extend FKT `retained_k4/product_certificate_vectors.py` with a combination section that writes a **new** fixture `product_certificate_combination_vectors.rs`. The existing `CASES` block and `product_certificate_vectors.rs` stay byte-identical; the script's default mode verifies both files and prints both sha256 values. Standard library only (`fractions`, `math.isqrt`, `struct`, `json`, `hashlib`); no import of production code and no copy of kernel arithmetic.

**Specimen** (one member, continuing I42/I44's so RV-K can compare): nodes (0,0,0) fully restrained (one rigid support, the anchor) and (1,0,0) free; member along X with y-reference along Y; D = 0.1, t = 0.005, E = 210e9 and independent G = 80e9 (preview); K-law section bits as fixed test primitives; stations at 0.25, 0.5 and 0.75. The tip flexibility is derived by an exact Fraction solve of the textbook 12×12 frame element reduced to the tip block, not hand formulas; the scratch check (`_run_records/`) confirms the coupling signs uy/Mz = rz/Fy = +L²/(2EI) and uz/My = ry/Fz = −L²/(2EI). Each tip motion depends on one coefficient (EA, GJ or EI), so the G-law interval is the hull of the values at the coefficient's two endpoints (π by a fresh bracket: Machin 16·atan(1/5) − 4·atan(1/239), as I41 and RV56 used).

**Operands** (tip terms, each an individual nodal term with its source id):

| Operand | Terms |
|---|---|
| A | Fx = 1, Fy = 1, Mx = 1 (I42's loaded witness) |
| B | Fz = 3, My = −2, and a cancelling pair Fy = +0.5, Fy = −0.5 |
| A2 | identical to A (a separate case; equal K4SRC bytes) |
| P | Fx = 1 and Fx = 2^-60 (two terms) |
| Q | Fx = −1 |
| H | Fy = 2^600 and Fy = 2^-600 |

**Combinations, with their exact nets** (from the scratch check):

| Id | Expression | What it discriminates | Exact nets (DOF: value) |
|---|---|---|---|
| C1 | A + B − A2 | SC1 C03 cancellation; cancelled DOFs stay data | Fx 0, Fy 0, Fz 3, Mx 0, My −2 (Fx, Fy, Mx: individual nonzero, net exactly 0) |
| C2 | 0.1·A + 3·B | non-dyadic factor `0x3fb999999999999a`; exact products | Fx = Fy = Mx = fl(0.1) exactly; Fz 9; My −6 |
| C3 | P + Q | (P, ε) − P: the binary64 sum of operand rows is exactly 0, the truth is 2^-60/(EA) | Fx 2^-60 |
| C4 | 2·B (prepared, first) + A (selected) | prepared representative; group from operand 1; authored order changes identity, not numerics | Fx 1, Fy 1, Fz 6, Mx 1, My −4 |
| C5 | 2^700·A − 2^700·A2 + 2^-700·B | huge cancelling factors; nets far below the products | Fz 3·2^-700, My −2^-699, others exactly 0 |
| C6 | 1·H | a net needing 1201 significant bits: a nondegenerate outward enclosure | Fy 2^600 + 2^-600 |

**Outputs per combination** (exact targets as outward 1024-bit dyadic pairs, in source_residual_vectors.py's `pair()` form):
1. The K4LED nets (sign, odd magnitude, exponent per DOF) and the sha256 of the K4LED bytes in `ledger.rs`'s documented encoding: an independent check of `RetainedLedger::combined`.
2. K-law and G-law tip motions (6 each), root and tip end actions and the three station actions (statically determinate, so law-independent and exact), root reactions (= −N moved to the root), displacement and support magnitudes (integer square-root bounds).
3. The exact published prescribed rows: +0.
4. Discriminators: (i) C3's operand-row sum (0) against the exact response; (ii) the response to operand 0's loads alone (the representative-source mutation) for C1, C2 and C4; (iii) for C6, the exact net and both directed 1024-bit endpoints: the nearest rounding differs from the exact net, so a kernel that rounds to nearest instead of outward gives a one-point enclosure that excludes the truth.

**Diagnose mode** (as source_residual_vectors.py `diagnose`): it parses `B2K_LEDGER` and `B2K_ROW` lines printed by the kernel tests (owner sha256 of K4CMB, row index and id, published bits, class, bound and scale bits, both lanes' endpoints, verdict) and checks independently: (a) both lanes contain the exact truth on every row; (b) the hull predicates, reproducing the five binary64 operations of A_f64, A_exact and the decimal tests; (c) the kernel's verdict equals the recomputed one on every row; (d) the K4LED bytes' sha256. It reports pass counts per combination (I44's target was all rows; a predicate failure is reported, not asserted away).

**Independence.** The implementer writes the generator from closed mechanics. RV-K re-derives a subset independently (a different π series, its own element solve) and checks the generator imports nothing from production.

## 7. The test list

**SC1 §5 (FK parts):**

| Id | Test | Discriminating mutation |
|---|---|---|
| C01 | For each existing combination specimen in `combine_tests.rs`, `origins_tests.rs`, `method_tests.rs`, `kf1_tracker_tests.rs`: old API vs source API give identical outcome bytes (source, ledger and state encodings, publication bits, attempts, work, meter) and identical recorded origins. Mixed A (selected) + B (prepared) selects with **no** run, source or meter charge for B, and gives the same ledger, state and publication bits as the all-selected A + B (only built-here flags may differ) | Route a prepared operand through `solve_cases`; drop the compatibility wrapper |
| C02 | Each of K4STF bits, a layout entry, a station fraction and a support membership changed independently, on a **prepared** operand and on a selected one → `OperandsDiffer` before any schedule; equal ids alone do not pass | Compare ids instead of bytes |
| C03 | C1 and C3 (§6): the combined ledger equals the oracle's K4LED; replacing the combined ledger by the sum of published operand rows or Σ c_i u_i fails containment (oracle) | Fold at p; sum published rows |
| C04 | A combination's escalation or stop leaves operand values, evidence, cache snapshots and the `PreparedCaseSource` unchanged; one prepared source serves two combinations; a later combination does not reuse an earlier combination's new slots | Mutate operand cache; reuse a combination's cache |
| C05 (FK part) | The certificate refuses a maximum row, a mode, parity or modulus record, and a missing stress slot for a combination owner (§5.2) | Case coverage applied to a combination |
| C06 (FK part) | A combination with a nonzero prescribed term in operand 2 (operand 0's zero) → `UnsupportedCombination`; the view never reads operand 0's value as the combined value | Use the representative's `constraint(g)` |
| W05 | Terminal `Refused` after nonzero work survives in `RecordedCombination` through `solve_sources_recorded`; the legacy projection still clears it | Clear attempts early |
| W06 | A slot built by an earlier same-batch case is imported by a mixed combination from its selected operand only; a prepared operand sharing the stiffness imports nothing; independent `solve_case` calls rebuild and charge | Import from a prepared operand; reverse first-slot order |

**Added by this design:**

| Id | Test |
|---|---|
| K-01 | `PreparedCaseSource::new` equals the solve path's prep: identity = K4SRC, same ledger bytes, same layout; a ledger-refusing source gives the same `LedgerRefusal` as `solve_cases` |
| K-02 | `register_prepared_source`: owner `Case(next ordinal)`, no Call, no Run, meter unchanged; capacity exhausted → `Capacity`; `for_calls(a,b)` and `for_invocation(a,b,0)` reserve identically; the dormant runs check never fires under `for_calls` |
| K-03 | S-2(a): B ends unavailable in the same batch as a selected A (for example `Budget(Case)` under a case limit that A's 128-bit selection fits and B's escalation does not; the combination call takes its own limit); `Prepared{B's source, rebuilt}` selects. Refusals: rebuilt from a changed source; out-of-range id; a combination's source id → `OriginRefusal` at the first offending operand. A kernel-selected, freeze-failed source as `Prepared` is accepted (I7) |
| K-04 | All-prepared → `NoSelectedOperand` (recorded and unrecorded), after `NoOperands`, `NestedCombination` and `OperandsDiffer` |
| K-05 | Prepared operand first: the group is operand 1's (`Arc::ptr_eq`); imports name operand index 1; the representative is operand 0's source; publication bits equal the reordered combination's, identity differs (C4) |
| K-06 | `product_owner` returns `Combination(i)` for a recorded selected combination run; both certificate entries accept it; another run id, a legacy unrecorded combination clone and a case-kind mismatch are refused |
| K-07 | View: P2 admitted; factor-bit or term-count corruption → `PairIdentity`; data flags equal the native verification's for C1–C6 |
| K-08 | Residual and recovery on C1–C6, both lanes: oracle containment on every row; exact-zero nets add nothing; C6's enclosure is nondegenerate and contains the exact net; entry counts are two `add` per nonzero net per pass |
| K-09 | Full prepared proof (`begin_prepared_product` → `project` → `complete_maxima(&[])` → `certify_final`) on C1–C5 with the combination row set; oracle diagnose agrees on every verdict |
| K-10 | Mutation controls (test-only): representative loads instead of the ledger; Σ operand rows as the center without correction (C3); nearest instead of outward net rounding (C6): each fails containment or a predicate |
| K-11 | The re-pinned `source_residual_combination_and_missing_uniqueness_are_explicit_refusals`: its combination half becomes an admitted 1·s combination whose enclosures contain the oracle truth, plus a nonzero-prescription refusal; its uniqueness half is unchanged |
| K-12 | The I42 bridge on a P2 combination owner: enclosures contain the oracle truth |
| K-13 | Case-path byte identity: every existing source_bridge, source_residual, product_final_case and product_certificate test passes unchanged, with identical printed work counts |
| K-14 | Layout neutrality: at J3, PP's `profile_in_build_record` and the retained-memory law tests pass in the registered identity build |

## 8. B3b: does it need an exact annulus version of any of this?

**No, for everything in this design.** Combinations never reach the exact route: `pressure_runtime::validate_profile` blocks them (`EXACT_PRESSURE_COMBINATION_UNSUPPORTED`) and exact-block requires no combinations (PLAN §0 item 4, confirmed by RV114). The prepared-operand API, the combination view and DEF-C are route-independent and unused by B3b.

**For B3-D's own question (the exact route's case preparation),** what FK shows:
- `prepare_product_annulus(diameter, effective_wall)` (`AnnulusPreparationVersion::NormalizedAnnulusV1`) is material-independent and route-independent: it maps normalized (D, t_eff) bits to correctly rounded [A, I, J, Z, c]. If the exact producer's operational section is the normalized annulus of (OD, effective wall), as C2 §3 and `SOURCE_ODWALL_EXPECTATIONS` state, it applies unchanged. I found no FK reason for an exact version; B3-D must still confirm the exact producer's section bits.
- **One FK gap B3b will need closed:** the public `ProductMaterial` (`final_case.rs:20–43`) offers `Base`, `Point` and `Interpolated`, which map to `MaterialOperands::Ordinary` and `Interpolated`. The internal `MaterialOperands::ExactENu { e, nu }` (`product_certificate.rs:46, 416–431`; G derived as E/(2(1+ν)) by directed division) exists and is covered by I41's `exact_normal` oracle case and RV56's ν = 0.3125 control, but PP cannot reach it. A public E/ν variant is likely B3-K's content: about 2–4 h with tests. PP's current matches on `ProductMaterial` end in wildcard arms, so the variant compiles in PP; constructing it on the exact route is lane P's. It must not grow `ProductMemberFacts` (atom `PRODUCT_MEMBER_FACTS`); two f64 fields are smaller than `Interpolated`'s, so it should not, but B3-K checks it (S-4's rule).

## 9. Stop list and refined estimate

### 9.1 Stop list for B2-K (each needs ROOT)

1. **S-1** Any change to the stop rule, schedule, precisions, verification at 2p, ceiling, prices, LME accounting or charge partition (PLAN §1.2.3).
2. **S-2** Any change to a case's bytes or work: case publication rows, classes, radii, evidence, K4SRC, K4STF, K4LED, K4CMB or K4RST encodings, attempts; the case branch of the view, residual, bridge, correction or final certificate, including their visit and entry counts.
3. **S-3** Any change to an all-selected combination's outcome, work, meter or recorded origins (I2).
4. **S-4** Any size or alignment change of a type exported by `retained_resource.rs`, or of anything stored inline in one (§1.1). This includes a new `CombinationReason` variant that enlarges `CallOrigin`.
5. **S-5** A new variant in an enum PP matches exhaustively, or a changed signature of an item PP calls (§1.1), or any edit outside FK.
6. **S-6** Retaining a prep in `ExecutionOutcome`, or any edit to SC1 §4's consumers (H, VR).
7. **S-7** Lifting P2 (nonzero combined prescriptions), or admitting a combination without a selected operand.
8. **S-8** Any change to DEF-O's final predicates, classes, scale formation, allowances, projection, lane order or seed.
9. **S-9** A combination proof that reads an operand's radius, row, state or certificate.
10. **S-10** Any change to K4CMB or K4LED encodings, or to `CasePrep::combination`'s representative or prescribed construction.
11. **S-11** Any changed byte in an existing oracle fixture or sealed vector (`retained_k4/SHA256SUMS`, `product_certificate_vectors.rs`, `source_residual_vectors.rs`, `source_bridge_vectors.rs`), or any existing FK test outcome change other than K-11's declared re-pin.
12. **S-12** A weakened S11 site-table row, or a new counted site without its row.
13. **S-13** Any FK change beyond this section's file list: FKR `adaptive.rs`, `combine.rs`, `origins.rs`, `product_certificate/source_residual.rs`, `product_certificate/final_case.rs` (and `product_certificate.rs` if the net helper lives there); `FK/src/structural.rs` (re-exports of `PreparedCaseSource`, `CombinationOperand`, `RecordedOperand`); FKT `retained_k4/{combine,origins,source_residual,source_bridge,product_final_case}_tests.rs`, `product_certificate_vectors.py`, the new `product_certificate_combination_vectors.rs`; FKT `s11_site_table.rs` only for an added row. `bridge.rs` and `ledger.rs` are expected unchanged.
14. **S-14** A published combination row set that differs from §5.2 (records, maxima or missing families), found by B2-C, B2-W or the tests.

### 9.2 Refined estimate for B2-K

| Item | Agent hours |
|---|---|
| `PreparedCaseSource`, `CombinationOperand`, the shared core, `solve_sources(_recorded)`, wrappers, `NoSelectedOperand` | 2–3 |
| Recorded custody: `for_invocation`, `register_prepared_source`, `RecordedOperand`, `solve_combination_sources`, `product_owner` | 2–4 |
| The view's combination branch | 1–2 |
| The residual's two load branches and the net enclosure with its work collection | 2–3 |
| `row_scales` coverage branch; both certificate entries | 1–2 |
| The oracle extension and its diagnose mode | 3–5 |
| Tests (C01–C06 FK parts, W05–W06, K-01 to K-14) and the K-11 re-pin | 5–7 |
| Runs (focused debug and optimized, the FK suite, PP compile and suite, the S11 table), byte-identity evidence | 2–3 |
| **Total** | **18–28 h** (REV: 14–24 h) |

The increase comes from the oracle (N-1, now specified), the combination coverage rule and its tests, and the layout-neutral custody. RV-K's code round stays 5–8 h, plus about 1 h to check the oracle's independence. The kernel lane runs beside B3b-P (PLAN §2.4), so the increase is near-critical only if RV-K asks for a second design round.

## 10. For ROOT

**Rule on:**
1. **R-1, the custody refusal for a prepared operand.** Recommended: reuse `OriginError::MissingSelectedOrigin { operand }`, documented as "no matching recorded origin for this operand", because a new variant breaks PP's exhaustive `origin_error` at J3 and decision 7 abandons the whole successor on any origin refusal, so the tag never reaches a published receipt. Alternative: a new variant, with a one-arm PP edit in `retained_wire.rs` at J3 and a B2-C wire tag.
2. **R-2, P2.** Combinations with any nonzero prescribed term stay `UnsupportedCombination` (D1 and DEF-C have none). Alternative P1 (§3.8), about +3–5 h and a wider review.
3. **R-3, `CombinationReason::NoSelectedOperand`** (a unit variant; B2-C adds its tag `combination/no_selected_operand`), declining SC1's optional source-only path for combinations with no selected operand.
4. **R-4, layout and enum neutrality as stops** (S-4, S-5), with K-14 in J3's acceptance.
5. **R-5, the declared re-pin** of `source_residual_combination_and_missing_uniqueness_are_explicit_refusals` (K-11) as the one expected FK test-outcome change, an exception to PLAN risk 9.
6. **R-6, lane K's write set** as S-13 lists it (it adds `source_residual_tests.rs`, `source_bridge_tests.rs`, `product_final_case_tests.rs`, the oracle files and, if needed, `s11_site_table.rs` and `product_certificate.rs` to PLAN §1.2.7's K row).
7. **R-7, the combination coverage rule** (§5.2): B2-C confirms the published row set; W-CB1's pins witness it.
8. **R-8, operand facts equality** in DEF-C (§4): the kernel checks K primitives only.
9. **R-9, for B3-D/B3-K:** the public E/ν material route (§8).
10. **R-10, DEF-C's hash domain:** reuse `retained_precision_formation_v1` (recommended) or reserve a new one at B2-C.
11. **R-11, the freeze-failed operand:** a kernel-selected case whose freeze failed is passed as `Prepared` (decision 6's "or its freeze failed"), and the kernel accepts a prepared operand whose source has a selected run (I7).

**For B2-C and B2-P (no ruling needed):** combined-net enclosures appear as `add` entries in DEF-C's numeric trace; `case_id` carries the combination id in the kernel row types; PP registers prepared sources after the case batch and maps the new native case ordinals; PP must pass `Prepared` for every operand that is not publicly selected.

## 11. What I read, execution record and limits

**Read:** the brief; `AGENTS.md`, `agents/AGENT_TASK.md`; PLAN in full; REV in full; RV114 in full; RR "B2/B3's plan dispatched as I93…" through "I93's REVISION_01 accepted; phase 0 opens with B2-KD (I94)"; SC1 in full; C1 §1–§6; C2 §1–§7; C3 §1–§3; DN's combination passages (§3.2, §4.1.1, §4.1.6.1 item 2's non-quantity list); DEF-O in full; I42 RETURN and its RV56-F1 repair RETURN; I43 RETURN; I44 RETURN; RV56 REVIEW, DERIVATION and repair RETURN. Code as listed under Basis, plus PP `retained_product.rs` (`bind_rows_view`, the capture path, `prepare_product_annulus` call), `retained_wire.rs` (the kernel-enum matches), `retained_memory.rs` (`ATOM_VALUES`), `retained_memory_law_tests.rs` (`profile_in_build_record`), `annulus_geometry.rs`, and `b1`'s PP uses of the kernel API.

**Executed** (Python 3.13.14, VENV; `_run_records/RUN.md` has the command with placeholders): `_run_records/b2kd_checks.py`, once, standard library only, reading and writing no repository file. It computes §6's exact nets and their bit spans, the cantilever coupling signs by an exact Fraction solve, and C3's operand-row discriminator. Output: `_run_records/b2kd_checks.out.json`.

**Limits:**
- **No code was compiled or run.** Every statement about behaviour is read from source; the design's claims (byte identity, layout neutrality, exact pass counts) are for B2-K's runs and RV-K's review to establish.
- **The coverage rule rests on PP's current combination publication** (`preview_physics.rs`, `lib.rs`); B2-P could change it, which is S-14.
- **The certificate theorem is I43's,** re-applied with a different load vector and zero prescriptions; I re-checked its premises for a combination owner (§5.3) but did not re-derive the R7 transfer theorem or the RV56 uniqueness proof.
- **Line numbers** are at NUM `098b63b768`'s maintained tree (main `2007709549`); lane K starts from J0, which B1 leaves unchanged in FK.
- **The estimate** is a reading estimate, calibrated against I42–I44's records.
