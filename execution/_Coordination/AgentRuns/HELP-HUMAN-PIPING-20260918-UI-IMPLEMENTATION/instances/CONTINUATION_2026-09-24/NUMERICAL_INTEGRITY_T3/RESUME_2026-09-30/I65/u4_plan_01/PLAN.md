# I65 U4 grant 1: derivation plan for the memory profile and capture permit

**Every U4 term has a closing route inside a bounded first admission domain except one: the numeric stack figure.** Stack can be bounded structurally from source, but its frame sizes are facts of the compiled build that no source argument supplies. That is the single stop item, and it needs a ROOT decision (§5, D-3).

**Native context, tail and backing close for this domain by exclusion, not by proof.** The first-party Tauri caller is outside the domain by construction, so no admitted invocation contains those owners. Native W1 qualification stays held exactly as RR:7015 left it (§4).

**The cap rule is the plan's backbone.** Every term is evaluated at the domain caps in the actual build. Each existing gate (G-A, G-B and G-C) then checks the actual census facts against the caps and the cap-priced bound against M. Out-of-domain, partial, stale or overflowing facts refuse to the unchanged ordinary path.

**Grant sequence:** three derivation grants (G2–G4) and one implementation grant (G5), then a qualification record (G6). Estimate: about 33–49 agent-hours of authoring plus 11–15 hours of independent review (§7). The tail of U4 depends on the frozen U1 and U3 source unless ROOT adopts the design-to-budget option in §7.

This is grant 1, the read-only plan. No code, Cargo, solver, native or DEC-025 job ran. No Git or index write happened; Git reads used `GIT_OPTIONAL_LOCKS=0`. The existing memory guard (PID 5387) was running at the start and at the write.

**Basis:** NUM at `dca3b65b3d` (step 4 planned; D38; this brief).

**NUM moved during this run.** It advanced to `43a6368c21`: records only, with no maintained-source change (verified by a Git diff read). The new ruling (RR:8836) records three things:
- experiment 03 passed: the facade receipt is byte-identical to the private driver's;
- D39 maps the `legacy_source` dispositions;
- U1 grant 1 is confirmed.

Its consequences here:
- The decision-5 runtime dependency belongs to U3 (RR:8862). T17 therefore follows U3's freeze.
- D39's `Ok(recovery)` row (exact-block selected, W1 not attempted) matches T07: deep legacy-exact storage falls inside G-A's ordinary span even when W1 is then bypassed.

All source citations below are at `dca3b65b3d`. They are unchanged at `43a6368c21`.

**Abbreviations:**
- P = projects/chirality-piping
- PP = P/core/product_physics/src
- FK = P/core/solver/frame_kernel/src/structural/retained
- FKS = P/core/solver/frame_kernel/src/structural
- T3 = the NUMERICAL_INTEGRITY_T3 record root
- R = T3/RESUME_2026-09-30
- RR = T3/ROOT_RULINGS_V1.md at `dca3b65b3d`
- WT = the T3 worktree parent; NUM = WT/numerics

## 1. Settled items this plan keeps

It keeps every item in R/I61/step4_plan_01/PLAN.md §1 and the step-4 ruling (RR:8807). In particular:
- **Admission architecture:** I51 public_producer_admission_05 (RR:7059). That means census before capture, the three successive reservations G-A, G-B and G-C, a frozen candidate, then validation, then an infallible transfer.
- **The ordinary formulas** at their partial scope (RR:7313, RV75).
- **Decision 5:** precommit Rust reader validation in PP.
- **Decision 6:** the first-domain shape.
- **Decision 9:** the machine reading of M is owner-held.
- **W1 policy** (RR:4938): M and the allowance are separate decisions. The 3.75 GiB figure is a provisional technical target (RR:4954).
- **"Do not enable a permit on symbolic or partially priced terms"** (handoff:158–160). No new serializer, guard, probe framework, host tooling or availability exception (handoff:164–166).

Nothing below reopens a settled item. §8 states how each decision stays within these.

## 2. The bounded first admission domain (D1) and its cap set

### 2.1 Domain predicate

An invocation is in D1 only if **every** condition below holds. Each is checked from allocation-free census facts before any W1 owner exists, at G-A. The family and kind conditions are read from the borrowed typed request by the extended census (T03), with no parse, clone or graph build. A condition that cannot be read that way is outside D1.

| Axis | D1 condition | Basis |
|---|---|---|
| Caller | `Entry::Direct` only (recommended, D-2); optionally `Entry::Headless` | PP/retained_memory.rs:308–311; I51 COMPOSITION §6 |
| Invocation | exactly one load case; zero combinations; zero components | decision 6; RR:8415 (T3 multi-case is wider F2a) |
| Family | preview family; no pressure (no exact-pressure profile, no pressure loads); straight members only. Supports are rigid restraints and scalar positive axis springs (no directional springs, nonlinear supports or hangers). Loads are nodal force/moment primitives only (no distributed, thermal, generated or load-reference state; imposed motion is already refused before parse). Zero material expansion laws. Default basis only, with no selected modulus basis | I54 direct_ordinary_bound_02/BOUND.md:32 (which cites retained_product.rs at `24af17c470`) |
| Exact-block | unchanged: exact-block runs first; if it selects, W1 is bypassed (D-15). The observation prefix allocated before that point is priced in T11 | RR:189, RR:229 |
| Census | raw, typed and (if headless) caller census complete; no depth, value or arithmetic refusal; profile status not Missing or Stale | PP/retained_memory.rs:284–295 |
| Counts and capacities | every fact within the caps in §2.2 | this plan |
| Build | the registered build identity (T01): aarch64-apple-darwin, 64-bit; rustc 1.97.1 (`8bab26f4f`); the PP consumer lock (serde_json 1.0.149 with `std,float_roundtrip`; no `preserve_order`, `arbitrary_precision`, `raw_value` or `unbounded_depth`); production `cfg(test)=false`; FK `mutation-controls` off | I54 direct_container_profile_01/COEFFICIENTS.md:5–13; PP/Cargo.lock |
| Modes | `sparse_interactive` and `dense_scrutiny`, each priced separately | I54 BOUND:28 |
| Precision | every reachable native rung, at most four physical precision records per Run. No p128 premise | I51 COMPOSITION §1 |

### 2.2 Proposed cap set (D-1)

| Fact | Cap | Milestone (I54 BOUND:87–93; RV75 recount) |
|---|---|---|
| nodes n | ≤ 32 (so N = 6n ≤ 192) | 2 (N = 12, F = 9) |
| straight members m | ≤ 32 | 1 |
| supports g | ≤ 32 | 4 |
| raw restraint entries r; spring entries s | ≤ 192 each | k = 3, s = 3 |
| nodal load primitives l | ≤ 192 | 3 |
| model materials; request materials | ≤ 4 each; ≤ 16 temperature points per material | 1; 0 (no temperature points; fixture read in this grant) |
| sections | ≤ 32 | absent in the fixture |
| any identifier, name or string value | ≤ 128 UTF-8 bytes | max 46 |
| raw JSON values; depth | ≤ 16,384 (the existing VALUE_LIMIT); ≤ 16 | 150; 7 |
| total string bytes; total key bytes | ≤ 65,536 each | 1,039; 855 |
| array capacity elements; string and key capacity bytes | ≤ 32,768; ≤ 131,072 each | actual census |
| captured digest capacity | ≤ 128 bytes | actual census |

**Why these caps:**
- **N ≤ 192 stays inside the existing source limits.** It is below `DENSE_SOURCE_DOF_LIMIT` = 256 (PP/source_recovery.rs:30) and the exact-boundary `dofs` = 256 (FKS/exact_boundary.rs:57–71). So legacy source recovery never changes regime inside D1.
- **Polynomial terms stay small at the caps.** Dense terms are at most N² = 36,864 entries per matrix.
- **The exhaustive law checks stay finite and quick** (T01).
- **The caps give at least 16× headroom over the milestone** for the U8 witnesses.

Caps bound only the evaluation domain. A cap is not a claim that larger inputs fail; they simply take the ordinary path.

**Fit of the milestone and the U8 witnesses:**
- **The milestone** (RF-SKEW-T-CANT-OFF-122-r1e-04, both modes) is inside every cap.
- **The L = 0 base** is "case 0 plus one memberless, fully restrained node" (R/I62/coverage_shared_python_01/SNAPSHOT_05_PLAN.md:86). That is n + 1 nodes, inside the caps.
- **The native Ceiling witness does not exist yet.** RR:7636 says it "needs a case with genuinely different numerics". If its construction needs more than the caps, it stays ordinary unless D1 widens. Widening is cheap before G5 and costly after it (D-9).
- **The two-load-case synthetic witness is outside D1** (multi-case is wider F2a).

### 2.3 Pricing rule and gates

For each caller c and mode μ, the profile evaluates `E_req,max(c,μ)` and `E_mov,max(c,μ)` at the caps. Each phase's union is taken as in I51 COMPOSITION §1, or as the conservative sum of phase maxima that RV75 accepted (I54 BOUND:16–26). Every stride is evaluated in the actual build (T01). Every addition and multiplication is checked; a failed calculation is a refusal.

The three gates stay where I51 put them. Each gate checks two things:
- the actual facts available at that point lie within the caps;
- the cap-priced bound of the phases still ahead is ≤ M.

Inside D1 a gate can fail only on an actual fact outside its cap (for example an unexpected capacity). That is a refusal to the ordinary path, never a rerun or a block.

Releasing memory refunds no work. The SourceRecoveryBudget and the W1 InvocationMeter stay separate.

## 3. Term table

States:
- **priced:** a complete formula at the stated scope; only the cap and in-build layout evaluation remain;
- **partial:** an accepted formula with named residuals;
- **symbolic:** an accepted owner roster or equation with unbound coefficients;
- **missing:** no derivation exists;
- **out of domain:** excluded from D1 by construction.

The grant numbers refer to §7.

| ID | Term | State and source | Closing argument | Evidence needed | Grant |
|---|---|---|---|---|---|
| T01 | **Final build and layout association** (RR:7313: "final build/layout association"; part of `BootstrapBuildAndStack`) | **Partial.** Container laws are accepted on installed std 1.97.1 (RR:7126, RV73). Strides are symbolic throughout I54, P1, P3 and I51. 43 private layouts are per-artifact DWARF observations, not associated with the final build (I54 container RETURN:9–14). The two consumer locks differ: PP has serde_json 1.0.149, headless has 1.0.151 | **Source plus in-build evaluation:**<br>(a) every public or crate-visible stride and alignment is taken with `size_of`/`align_of` inside the profile code, so the association to the compiled build is automatic. FK-private and SR-private types are evaluated in their own crate and exported as numbers (fence §7);<br>(b) private std and dependency layouts (BTree LeafNode and InternalNode, hashbrown buckets and control bytes) use source-derived upper formulas over in-build K/V sizes, with per-field rounding to the maximum alignment. Check: for `Map<String,Value>`, 8+2+2+11·24+11·32 = 628, which rounds to 632; Internal is 632+12·8 = 728. Both equal the existing DWARF observation (I54 container RETURN:14);<br>(c) build-identity enforcement per D-6;<br>(d) exhaustive in-domain checks of every growth law relied on (Vec, String, VecDeque, BinaryHeap and HashMap capacity sequences over the cap ranges), run in the actual build. This is finite because D1 is bounded | std 1.97.1, hashbrown 0.17.1 and serde_json 1.0.149 source citations; the list of every type whose stride enters the bound; compile-time layout witnesses; the law-test outputs; review | G2 derive; G5 implement |
| T02 | **Raw request backing R_raw** | **Priced** (I54 BOUND:36–44, RV75), given complete census facts; strides symbolic | Cap evaluation plus T01. The census is already complete-or-refuse | the existing census tests plus cap-boundary tests | G2; G5 |
| T03 | **Nested typed owners** (`NestedTypedOwners`; RR:7313 "nested input capacities and construction") | **Missing.** The census reads only eleven top-level CapacityFacts (PP/retained_memory.rs:171–201) | **Count cap plus source.** Extend the borrowed typed census to every nested owner reachable in D1: each String and child Vec `len`/`capacity`, section property trees, and provenance Values through `borrowed_value_census`. The extension stays read-only and allocation-free. Arrays that D1 forbids must have length 0, and their actual capacity is still read. Use actual capacities, not construction-history laws, because serde's visitor laws are more fragile | a complete enumeration of the `LinearStaticPreviewRequest` type tree, checked by the reviewer against the type definitions; tests for spare-capacity inputs | G2 enumerate; G5 implement |
| T04 | **Census workspace and bootstrap** | **Priced (heap 0).** No clone, parser, encoding or heap; a fixed `[Option<Frame>; 64]` array (PP/retained_memory.rs:95–166) | **Source argument already in code.** Its stack goes to T20 | add an allocation-count assertion using the existing `CountAlloc` test allocator (P/core/product_physics/tests/retained_precision_admission.rs:6–33) | G5 |
| T05 | **Ordinary active and suffix** (`OrdinaryActiveAndSuffix`) | **Partial.** I54 02+03 are accepted at partial scope (RR:7313; RV75: 46+23 rows). The residuals are T01, T06, T07, T08 and T20 | **Source plus cap.** Evaluate the accepted formulas at the caps per mode: the sparse-or-dense typed path, W2 (at most two evaluations), the dense parity tail, the legacy recovery prefix, maximum, preview rewrite, aggregation and error prefix. Prove each formula is monotone nondecreasing in its counts before substituting caps; for example `Zf ≤ min(Z,F²)` and `H ≤ F(F+1)/2` need the check. Typed G-b/G-l capture that U1(a) adds at ordinary sites enters through T11 | a cap arithmetic table (stdlib Python under `_run_records`, as I54 and RV75 used); a monotonicity lemma per row; review | G3 |
| T06 | **H_formation128** (RR:7313) | **Missing** (named residual). A preliminary scan in this grant found:<br>– FK/wide.rs has zero `Vec`/`Box`/`String` occurrences;<br>– `WideArith` holds only `{precision, WorkCounter}` (FK/wide.rs:871–885);<br>– the re-formation loop (FKS/formation_check.rs:229–268) and its helpers (`frame_matrix`, `rotate`, `mul6`, `invert6`) use fixed arrays (`Element = [[Wide2;12];12]`, FKS/formation_check.rs:460–462) | **Source argument:** heap 0 on success and on every error path. Error detail Strings move to T08; large fixed frames move to T20. Curved and user elements are outside D1 | a complete call-tree read with citations; review | G2 |
| T07 | **Generic deep legacy-exact** (RR:7313) | **Missing in general.** It is excluded only for the named skew case: the direction (1,2,2) is not a signed permutation (PP/source_recovery.rs:437–458; I54 BOUND:97) | **Source derivation bounded by the existing limits:** exact-boundary `Limits{dofs:256, source_terms:16,384, expansion_terms:256}` (FKS/exact_boundary.rs:57–71) together with N ≤ 192. The derivation must show storage is bounded by the dof and term limits, not by the operation budget. Fallback (not recommended): a pre-observer exclusion of signed-permutation members, which would duplicate the source_recovery check (D-8) | Context/Response/functional ownership derivation; review | G2 |
| T08 | **Text grammar** (RR:7313: report, row, diagnostic and error text) | **Partial.** RecoveryFinding and stress-finding owners have polynomials (I54 correction_03 RESIDUALS). Still open:<br>– StraightPipeError/FrameKernelError Display;<br>– ExtremaError Debug;<br>– float and integer spellings;<br>– formatter helper storage;<br>– wider report and row text;<br>– new U1 text (`RETAINED_PRECISION_*` fixed product text, C2 causes) | **Count cap plus source.** The 128-byte identifier cap closes the dynamic parts. Enumerate every template reachable in D1 with the maximum spelling of each field, taken from the pinned `core::fmt` per formatter. Caution: f64 **Display** is positional and can run to hundreds of bytes. Take each formatter's maximum (Display, Debug, LowerExp) from the pinned source; do not assume a 24-byte spelling. String capacity uses max(8, 2·len), the convention RV75 accepted for these templates | a template table with citations; review | G2 (ordinary); G4 (U1 text) |
| T09 | **Allocator overhead, RSS, concurrency** | **Non-claim.** I51 COMPOSITION §1 keeps them separate | M counts requested and moving heap bytes per invocation only. A machine or concurrency reading is owner-held (D-7) | stated limit in the profile record | none |
| T10 | **Object backing and container laws** (`ObjectBackingAndContainerLaws`) | **Partial.** RV73's laws are accepted as source proofs; hashbrown correspondence is accepted as inference; private layouts were observed | T01(b) source uppers with in-build sizes, plus T01(d) law tests | as T01 | G2; G5 |
| T11 | **Observation overlap** (G-A `W_observation`; G-B late capture) | **Partial.** The owners are identified: the observer and the late capture (I51 COMPOSITION §2, which cites `922db9dce3` source lines). The P1 old-source capture counts are accepted (RV48). Strides are symbolic | **Source plus cap,** including U1(a)'s typed G-b/G-l capture fields once frozen, and the observation prefix that remains on an exact-block bypass | derivation; delta pass after U1(a) | G3 |
| T12 | **Preparation** (P1 and P2) | **Partial.** P1 counts and ownership are accepted (RV48). P2 covers source and geometry, the RCM graph, ledger and layout, the K4SRC/K4STF/K4LED encodings and failed prefixes | **Source plus cap** (I51 COMPOSITION §3, rows 1–2) | derivation; review | G3 |
| T13 | **Native retained run** (P3 owner union) | **Symbolic.** The P3 envelope was reviewed (RV52 plus backcheck) with "coefficients remain unbound". The I37 layouts changed and need build facts (R/I37/checked_work_a/RETURN.md:154–160) | **Source plus cap, evaluated in FK** where the private strides are visible. It covers all reachable rungs, failed-cache copies, hats, floor, trackers and moving backings; per run at most one Call, Source, Group and Run, seven Build opportunities and four precision records | derivation; FK-side evaluation tests | G3; G5 (FK) |
| T14 | **Proof lanes, projection and maximum, final certificate** | **Partial.** The roster is in I51 COMPOSITION §3. The maximum helper's heap is 262,144·s(Node) requested and 393,216·s(Node) moving, run sequentially per member (I54 BOUND:76). The mask upper is N_dof+2B | **Source plus cap.** s(Node) is SR-private, so it is evaluated in SR or by a source upper | derivation; review | G3 |
| T15 | **C3 typed trace and evidence completion** | **Partial.** The counts are closed: at most m entries, 9m conversions, two lane terminals and P_final row conversions (I51 COMPOSITION §4). The trace is accepted at `b56b905251` | **Source plus cap** | derivation | G3 |
| T16 | **Publication staging, hashing and canonical serialization** (U1) | **Partial.** The laws exist: I54 container J-rows; Bigint 1,440 requested / 2,080 moving; the 128-byte render start. The U1 code does not exist yet | **Source plus cap over U1's frozen source.** Receipt length comes from the closed C1/C2/C3 grammar (Text(b) ≤ 6b+2, H = 66 and so on; I51 COMPOSITION §5). Count the ancestor key vectors, duplicate-key sets and old/new output growth | derivation or budget (§7 option) | G4 |
| T17 | **Precommit Rust reader validation** (decision 5; runtime dependency installed by U3, RR:8862) | **Missing.** Decision 5 introduced this phase. P/core/reporting/result_export/src/retained_precision.rs is 4,335 lines with about 65 clone, collect or format sites. It is accepted (RR:8761) but will be touched by the D38 reader round | **Source plus cap,** bounded by the receipt size at the caps. Re-derive the rows the D38 round changes | derivation; review | G4 |
| T18 | **Transfer, fallback and refusal reserve** (U3) | **Missing.** The FrozenPublicationCandidate split and the pre-reserved `RETAINED_PRECISION_UNAVAILABLE` notice are not yet written | **Source property over U3's frozen source:** no fallible allocation after the first mutation; the fallback notice is reserved before execution; the ordinary base stays intact | derivation; review | G4 |
| T19 | **Caller completion** (`CallerCompletion`) | **Partial.** Direct: output transfer only; external copies are excluded (I51 COMPOSITION §6). Headless: the owner roster is named (I51 COMPOSITION §6), including six derivative arrays coexisting and the full `validate_document` clone. Site weights are unbound, and its lock is serde_json 1.0.151 | **Direct:** source plus cap, small. **Headless:** source plus cap with its own build identity, only if D-2 includes it | derivation | G4 |
| T20 | **Stack** (RR:7313; part of `BootstrapBuildAndStack`) | **Missing — the stop item.** No stack figure exists anywhere. I51 C0 explicitly declines to "advertise ABI spills" (R/I51/prepared_producer_implementation_03/C0_HELPER_LAYOUT.md:30,53) | **Structural part, closable by source:** a recursion inventory for D1. serde_json parse is limited to 128 levels (cached serde_json 1.0.149 src/de.rs:63, with `unbounded_depth` off); Value drop and canonical rendering are bounded by the depth cap and the fixed receipt grammar; the census is iterative; the exact helpers are finite and nonrecursive (C0 schedule). **Numeric part: not closable by source** — see §5 | see §5 | G2 (structure); per D-3 |
| T21 | **Moving and temporary owners** (`MovingAndTemporaryOwners`) | **Partial.** The moving laws are accepted (RV73); there are per-family moving uppers (I54) and P1's R0+2H′ pairs | Carry E_mov alongside E_req in every phase and evaluate at the caps | as each phase | G2–G4 |
| T22 | **Upstream no-wrap premise** (G-h; C1 §2) | **Partial.** The I29 no_wrap_bound_03 lemmas hold only within their premises (RV44). Checked/sticky WorkTotal is designed (RR:5210) and accepted in source at `fdae294643b` (RR:5468). I52's integration basis says it "supersedes C1 §2's historical proposed global static no-wrap prerequisite" (R/I52/reader_integration_04/RETURN.md:23–25; released at RR:7157). Two items remain:<br>– closed scalar admission (R/I34/f2a_work_exactness_api_02/API_PLAN.md:244–245, 304–306);<br>– the explicit C1 §2 acceptance that I34 DESIGN.md:230–233 requires | **Count cap plus a ruling.**<br>(a) Tabulate every scalar site (6n, offsets, residual m/64m, tracker sequences, r·r, and the 6n/3m/u32 boundaries of C2) with its cap-evaluated maximum against its width; all are trivially representable at the caps;<br>(b) U1's projection takes Run, Build and Call amounts only through `WorkTotal::exact()` (FK/work.rs:64); an O or I status falls back to the ordinary path with `work_counter_*`;<br>(c) a ROOT reconciliation ruling (D-4) | the scalar table; U1 projection tests; the ruling | G2; U1; D-4 |
| T23 | **Native context, tail and backing overlap** (I53/RV70) | **Out of domain.** The external lifetime warrant is unproved (RR:6995, RR:7015) | **Domain restriction, enforced by source** (§4) | the citations in §4 and the existing source test | none in U4 |
| T24 | **Whole producer, publication and caller composition** (RR:7313) | **Symbolic.** The equation is complete in named phases (I51 COMPOSITION §§1–3, 6) | **Composition at the caps:** T02–T21 per caller and mode; check E_req and E_mov ≤ M at each gate placement | the composition table; review | G4; G6 |

**Cross-reference to the brief's named items:**
- **RR:7313 residuals:**
  - final build/layout association → T01;
  - nested input capacities and construction → T03;
  - H_formation128 → T06;
  - text grammar → T08;
  - deep legacy-exact → T07;
  - stack → T20;
  - whole composition → T24.
- **Native context, tail and backing** → T23.
- **The seven `MissingAdmissionTerm`s** (PP/retained_memory.rs:212–229):
  - BootstrapBuildAndStack → T01, T04, T20;
  - ObjectBackingAndContainerLaws → T10, T02;
  - NestedTypedOwners → T03;
  - OrdinaryActiveAndSuffix → T05–T08;
  - PreparationNativeProofAndPublication → T11–T18;
  - CallerCompletion → T19;
  - MovingAndTemporaryOwners → T21.
- **No-wrap** → T22.

**Summary by state** (24 terms):

| State | Count | Terms |
|---|---|---|
| Priced | 2 | T02, T04 |
| Partial | 12 | T01, T05, T08, T10, T11, T12, T14, T15, T16, T19, T21, T22 |
| Symbolic | 2 | T13, T24 |
| Missing | 5 | T03, T06, T07, T17, T18 |
| Missing and stopped | 1 | T20 |
| Out of domain | 1 | T23 |
| Non-claim | 1 | T09 |

Apart from T09 (a non-claim) and T20's numeric part, every term has a source-derivation, count-cap or domain-restriction route. None of those routes needs a host tool.

## 4. Native context, tail and backing: the explicit answer

**Yes: within D1 they close without new host tooling, by a source-enforced caller-profile restriction.** They are absent from every admitted invocation, not zero by assumption.

**The source facts at NUM `dca3b65b3d`:**
- **The Tauri first-party caller uses only the ordinary wrapper.** P/apps/desktop/src-tauri/src/lib.rs:1557–1562 calls `run_linear_static_preview_value_with_mode`, and :1696 is the worker call through the same function.
- **A source test locks this in.** P/core/product_physics/tests/retained_precision_admission.rs:217–222 asserts that the native source contains neither `with_retained_direct` nor `with_retained_headless`.
- **No native permit is possible.** `Entry` has only `Direct` and `Headless` (PP/retained_memory.rs:308–311), and `CapturePermit` is `pub(super)` (:300). No native path can mint or inherit a permit.
- **The accepted design already holds native.** I51 COMPOSITION §6 says "Native is held".

**So no admitted invocation contains any native owner.** The I53/RV70 owners are:
- **C_a:** the original request, envelope, arguments, URL and headers, and the resolver and transport contexts;
- **B_call:** task-map backing;
- **O_call:** ordinary response work;
- **L_i and D_j:** the W1 reply allocations.

**What this does not do:**
- It qualifies no native W1 reply. The finite native-context and attributable-backing warrant stays the external prerequisite RV70 named.
- It selects no H, δ, K, availability restriction or finite-use rule.
- It changes no native code.

Enabling native W1 later is wider-F2a work. It needs that warrant, or an owner availability decision among the alternatives I53 recorded: a finite qualification episode (native_caller_qualification_01 RETURN), or a claim limited to the qualified per-request and K-admitted components (first_party_profile_02 RETURN, final section). U4 does not need it, because the milestone runs through the facade and not through the desktop caller.

## 5. Stack: the stop item (T20)

**What closes:** a source argument that D1's W1-added path has no count-unbounded recursion, so peak stack is a finite constant of the build once the depth caps are fixed. G2 delivers this inventory.

**What cannot close by source:** the numeric frame sizes of the compiled build along the deepest call chains. Some frames are large fixed arrays: `Element = [[Wide2;12];12]`, the ExactWideSum arrays, and the 3,888-byte 27-Endpoint preparation frame (I51 C0_HELPER_LAYOUT.md:13). Rust has no in-language frame-size fact equivalent to `size_of`. A stack overflow aborts the process, which would also destroy the ordinary result, so admission cannot treat stack as unknown.

**The routes, for ROOT (D-3):**
- **S1 — reserved stack, structural bound and measured witness (no host tool; recommended if ROOT accepts this evidence standard).**
  - The retained entry runs its whole invocation on a scoped thread built with `std::thread::Builder::stack_size(R)`. R is a profile constant (for example 64 MiB, which is address space reserved lazily, not resident). A spawn failure declines W1 before any W1 owner and returns the ordinary path; a panic is re-raised with its original payload via `join` and `resume_unwind`. The spawn's own heap allocations are priced in T19.
  - Qualification adds a std-only witness test in the actual build. The cap-maximal D1 inputs run, in both modes and through each reachable refusal branch the tests can drive, on a thread with stack `R/k` (for example k = 16).
  - This is a measured witness with a margin, not a proof. It must be recorded as such.
  - Moving the invocation to another thread must not change behaviour through thread-local state. G2 inventories every thread-local on the retained path. This grant's scan of PP found only `historical_pressure_reference::ACTIVE` (PP/historical_pressure_reference.rs:10), which is private and test-only, outside D1 because pressure is excluded, and asserted not to propagate to spawned threads (PP/lib.rs:14446–14457). It also found the `cfg(test)` dense-ceiling override (PP/lib.rs:2989–2994). A scan of every P/core crate's non-test source found no other `thread::spawn`, `thread::scope` or `rayon` use (performance_harness excluded).
  - The placement edits PP/lib.rs at the retained entries only, so U3 and U4 must share one integration owner for that site (D-5).
- **S2 — a frame census of the actual artifact.** Read prologue frame sizes and the call graph from the build's disassembly with stock Xcode tools. Doing this for the W1 path needs an analysis script, which is new host tooling and is excluded by handoff:164–166. It needs the owner's authorization. Not proposed.
- **S3 — hold the permit.** Leave stack unqualified and keep the permit disabled. The milestone waits.

G2–G4 do not depend on D-3, because stack is separate from M (I51 COMPOSITION §1). G5 cannot enable a permit until D-3 is resolved and its evidence exists.

## 6. The no-wrap premise (T22) in brief

**C1 §2 names this alternative itself:** "A separately authorized checked/sticky-overflow seam is an alternative only after its own bounded design, maintained write-set and review" (R/I32/f2a_wire_c1/WIRE_CONTRACT.md:64). That design (I34, RV46) and its source (I37, RV51) are accepted.

What remains is to apply that existing alternative to D1. Nothing settled reopens:
- the cap-bound scalar admission table (G2);
- the E-only projection discipline (U1);
- ROOT's explicit reconciliation ruling (D-4).

## 7. Grant sequence

Every grant gets fresh independent review of its complete frozen packet or diff, with mutants where it has logic, and ROOT verification before reliance. For G2–G4 the review is an independent re-derivation of the formulas and cap arithmetic, not a re-reading. Reviewers are fresh IDs from RV82 onward, one per grant, and no author reviews its own grant. Grants make no Git writes. Arithmetic uses stdlib Python under each packet's `_run_records`, as I54 and RV75 did. No new tooling.

| Grant | Deliverables | Write fence | Depends on | Estimate (author; review) |
|---|---|---|---|---|
| **G2 — domain, build binding, residual closure** (records only) | **DOMAIN.md:** the predicate, the cap table, the refusal map, the U8 fit.<br>**BUILD.md:** T01/T10 binding — the stride type list, private-layout uppers, the identity mechanism per D-6, the law-test specification.<br>**RESIDUALS.md:** T03 enumeration, T06, T07, the T08 ordinary templates, the T22 scalar table.<br>**STACK_STRUCTURE.md:** T20 recursion inventory.<br>**API.md:** the U3/U4 interface for the permit consumer and gate checks (D-5) | R/I65/u4_g2_01/; WT/scratch/i65_u4/ | D-1, D-2, D-6, D-8 (proposed answers are usable provisionally) | 8–12 h; 3–4 h |
| **G3 — producer composition at the caps** (records only) | T05 cap evaluation with monotonicity lemmas; T11–T15; T21 for those phases; per-mode tables | R/I65/u4_g3_01/ | G2 BUILD.md (stride list). A delta pass after U1(a) freezes the typed capture sites | 8–12 h; 3–4 h |
| **G4 — publication, validation, transfer, caller** (records only) | T16, T17, T18, T19 (Direct; Headless if D-2), the U1 part of T08, and the T24 composition per caller and mode | R/I65/u4_g4_01/ | U1(b) and U3 frozen, **or** design-to-budget: derive the budget from the C1/C2/C3 grammar and I51 §5, then U1 and U3 reviews check conformance; a 1–2 h delta pass after their freeze | 6–9 h; 2–3 h |
| **G5 — implementation: RegisteredProfile and admission law** | In PP/retained_memory.rs:<br>– `RegisteredProfile` with cap-priced constant bounds;<br>– the D1 predicate;<br>– the nested census (T03);<br>– gate checks at G-A, G-B and G-C;<br>– an in-domain-only permit constructor;<br>– Missing and Stale handling, with every refusal preserving all facts.<br>Elsewhere:<br>– an FK resource module exporting kernel cap-bounds (a new file under FK plus its `mod` line);<br>– an SR stride export if T14 needs one;<br>– per D-6, a PP build-identity mechanism;<br>– per D-3/S1, the retained-entry thread placement in PP/lib.rs (single integration owner with U3).<br>Tests (a new PP test file):<br>– exhaustive law checks over the cap ranges;<br>– cap and cap+1, unknown, stale, overflow and partial refusals, with ordinary bytes identical;<br>– M−1/M/M+1 boundary tests on the gate arithmetic;<br>– an allocation challenge in an isolated test binary using the existing peak-tracking test-allocator pattern (P/core/product_physics/tests/f1b_sparse_pattern_memory.rs:34–72), as a challenge only and not a proof or production guard;<br>– the S1 stack witness | as listed; one focused Cargo lane (4 jobs, 2 test threads) | G2–G4 accepted; D-3, D-5, D-6 | 10–14 h; 3–4 h |
| **G6 — qualification record and M** | Evaluated `E_req,max` and `E_mov,max` per caller and mode in the actual build; the stack evidence; the permit record. ROOT selects M (D-7). The permit becomes reachable only through U3's consumer. Eligibility and public activation stay with U7 | R/I65/u4_g6_01/ | G5 accepted; U3 | 1–2 h plus ROOT |

**Totals:** about 33–49 agent-hours of authoring plus 11–15 hours of review.

I61's figure was 20–35 h plus 8–12 h. The difference comes from:
- pricing the precommit reader (T17, from decision 5);
- the nested census (T03);
- the build-identity work (T01);
- stack (T20).

**Order:**
- G2 starts now, in parallel with U1.
- G3 overlaps U1. G2 and G3 have disjoint records, so ROOT may run them concurrently under separate TASKs; I65 does not delegate.
- G4 needs U1(b) and U3 frozen, or the budget option.
- G5 follows G4.
- G6 closes U4.

**U4 cannot finish before U3 freezes** unless ROOT takes the budget option. I recommend the budget option, with a conformance delta.

## 8. Decisions needed

| ID | Decision | Owner | Proposed answer |
|---|---|---|---|
| D-1 | Adopt D1 and the cap set in §2, with the cap-pricing rule; the three I51 gates are kept as fact checks | ROOT | Adopt as written. This is decision 6 made concrete; it does not change the architecture |
| D-2 | First-domain callers | ROOT | Direct only for the milestone. Headless needs its own build identity (serde_json 1.0.151) and a heavy completion term; add it afterwards, about +3–5 h |
| D-3 | **The stack evidence standard (the stop item)** | ROOT; the owner if S2 | S1 if ROOT accepts "structural source bound plus reserved stack plus measured witness with margin" as stack qualification. Otherwise S3, and take S2 to the owner as a tooling decision |
| D-4 | C1 §2 reconciliation for D1: checked/sticky WorkTotal (accepted) plus cap-bound scalar admission plus E-only projection satisfy the pre-execution no-wrap premise | ROOT | Rule it. This is C1 §2's own named alternative (I34 DESIGN.md:230–233) |
| D-5 | Fences between U3 and U4 | ROOT | U4 owns PP/retained_memory.rs and the FK/SR resource exports. U3 owns the dispatch and the permit call site. The interface is fixed in G2 API.md. No concurrent edits of retained_memory.rs; one integration owner for any PP/lib.rs retained-entry edit |
| D-6 | Build-identity enforcement | ROOT | (a) A PP build script records the rustc identity, and any mismatch gives `ProfileStatus::Stale`; plus compile-time layout witnesses and reviewed consumer locks. If ROOT rules a build script out as tooling: (b) gate-bound qualification only, recording that unqualified downstream builds are not self-detected |
| D-7 | M | ROOT; the owner for any machine reading | After G6: M = 4,026,531,840 B as the per-invocation W1 admission threshold on requested and moving heap bytes, provided `E_mov,max` ≤ M. It makes no claim about RSS, allocator overhead, stack, concurrency or supported machines (RR:4954; I61 decision 9). A tight M (the priced maximum) adds nothing inside D1 |
| D-8 | Deep legacy-exact route | ROOT | Derive it under the existing exact-boundary limits (T07); do not restrict geometry |
| D-9 | U8 witness headroom | ROOT | Confirm the caps, or supply the intended Ceiling-witness counts before G5 |

**Owner-held, and not needed before U4 returns:**
- the supported-machine reading of M;
- native W1 activation (the alternatives I53 recorded);
- concurrency and RSS claims;
- the existing dense and lane ceilings, PHYS-R4, observation framing, the KF3 lambda split and the KF2 dense screen. All of these are untouched.

## 9. Stop-condition check, risks and limits

**Stop conditions:**
- **"A term cannot close by source argument":** only T20's numeric part. Reported in §5 and D-3.
- **"Would need a host tool":** only under S2, which is not proposed.
- **"A settled item would reopen":** none.
  - D-1 keeps the I51 gates.
  - D-4 applies C1 §2's named alternative.
  - Decision 5 stands; its cost is T17.
  - T23 relies on the accepted "native is held".

**Risks:**
- **U1 and U3 churn after G4** invalidates rows of T16–T18. Mitigation: the budget option plus a delta pass.
- **The Ceiling witness may exceed the caps** (D-9).
- **The T17 reader profile is the largest new derivation,** and the D38 reader round touches it.
- **Monotonicity** must be proven per formula before cap substitution. A non-monotone row needs its maximum over the cap box, not its value at the cap.

**Limits of this grant:**
- **Preliminary source scans are leads, not derivations.** These are T06's zero-heap scan, the T20 recursion notes and the BTree layout check.
- **I verified only the citations listed.**
- **No figure here is a bound.** The I61 experiment-01 producer run's maximum RSS of about 20 MB (debug build, both modes plus the refusal receipt; R/I61/receipt_experiment_01/RETURN.md:95) is a witness, not a bound.

## 10. Execution record

TASK I65 (Type 2), dispatched directly by ROOT (HELP_HUMAN, Agent 0). No descendants. Host: the M5 per the handoff.

**Timing:**
- First tool call: 2026-10-03 21:06:12 MDT (2026-10-04T03:06:12Z).
- PLAN written, checked and sealed at about 21:27 MDT (2026-10-04T03:27Z), well inside the 90-minute box (deadline 22:36 MDT).

**Memory guard:** `WT/guard/memguard.sh` (PID 5387), seen running at the start and before writing.

**Reads:**
- AGENTS.md, agents/AGENT_TASK.md and P/AGENTS.md;
- the brief; I61 PLAN; the handoff; RR rulings at :4938, :5024, :5144, :5210, :5260, :5468, :6890, :6995, :7015, :7059, :7090, :7126, :7157, :7198, :7313, :7617, :8807 and (added during the run) :8836;
- I53 (01, 02, 03) and RV70; I51 admission_05; I54 bound 02 and 03, container 01, and RV75; I29 memory plan 02 and P1; I32 C1 §2; I34 design and API; I37 RETURN; I52 integration 04;
- the source files cited above, all at NUM `dca3b65b3d`;
- the cached serde_json 1.0.149 `de.rs`.

**Writes:** only this PLAN.md and SHA256SUMS. Git reads only (`log`, `status --short`, `diff --stat`/`--name-only` for the NUM advance), with `GIT_OPTIONAL_LOCKS=0`.
