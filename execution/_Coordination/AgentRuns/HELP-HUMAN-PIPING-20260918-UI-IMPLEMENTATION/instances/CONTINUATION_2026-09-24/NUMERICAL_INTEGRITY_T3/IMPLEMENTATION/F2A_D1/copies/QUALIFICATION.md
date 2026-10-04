# I65 U4 G6: the qualification record for D1

**Agent:** I65, TASK (Type 2) under ROOT. No descendants.

**Grant:** `BRIEFS/I65_U4_G6_QUALIFICATION.md` (NUM `ac5b318c5b`), with:
- RR "U4 G5 part 2 verified and committed; in-build maximum 0.889 M; G6 granted";
- RR "RV87 on S-2 in G5 part 2: NOT CONFIRMED; a class-wide identifier audit goes into G6";
- RV83's R-4 confirmation (`R/REVIEW_RV83/u4_g2_04/REVIEW.md`, N-3);
- RR "RV89 on U4 G5 part 2: PASS; the routing into G6 and the margin standard".

**Code:** WT/f2a-memory, on top of `cba3e9fda7` (part 2), uncommitted: +396 / −189 lines in 5 files, all inside the G6 fence, including ruling 2(a)'s G-C fact (`_run_records/candidate_g6.diff`). G6 adds no new file. It changes:
- `retained_memory.rs`: the regenerated profile;
- the law, witness and challenge tests;
- the FK resource module: new exports.

**Registration is not applied.** `REGISTERED_PROFILES` is still `&[]`. The change is prepared as `registration.diff` (§8). ROOT applies it after G6's independent review; RV89 has passed part 2.

**Revised by the pre-registration repair** (RR "RV87 … NOT CONFIRMED" and RR "RV89 on U4 G6 with registration.diff: PASS"; RETURN.md, Addendum 2). G6 is committed unregistered as `2bb81ec1ea`; the repair is uncommitted in WT/f2a-memory on top of it, in 3 files inside the fence (`retained_memory.rs`, the law tests and the challenge, +142 / −14 lines, `_run_records_g6r/candidate_g6r.diff`; the witness tests and the FK module are unchanged). This record's numbers are the repaired ones; G6's sealed text is at NUM `c4a1bcefa2`. The repaired TEXT inputs and outputs are in `_run_records_g6r/`.

## 1. What G6 did

| Step | Result |
|---|---|
| 1. Close the 42 Estimates | **Done.** `profile::ESTIMATES` = 0: 190 InBuild atoms, 16 SourceUpper, 6 Text (§2). The bound is now **priced** in every build. Admission still refuses at D1.1 until a profile is registered |
| ROOT's addition: the identifier-class audit | **Done.** 795 entries priced by source (G6: 747; the repair adds 48), and the run enforces coverage by type, whichever rule prices a candidate, and fails on stale site keys. TEXT +34.2 MB over part 2; RV87's 18 sites are exactly at RV87's bounds, and SF-1/SF-2 are priced by source (`ID_CLASS_AUDIT.md`) |
| Re-run TEXT; regenerate the profile and the record | Done (`_run_records/text_g6/`, `per_identity/`) |
| In-build maximum ≤ 0.9 M | **Yes.** Dense W3: E_mov,max + R = 3,595,488,734 B = **0.8929 M**, 28,389,922 B under 0.9 M. Sparse: 0.8881 M (§3). G6 was 0.8927 / 0.8878 M |
| 2. Choose the build identity | **The dev/test build on this host** (§5). The release build is qualified too, but not proposed for registration |
| 3. Gate the pinned record on build identity | **Done.** `profile_in_build_record` and `challenge_bounds_are_the_profile` assert only when `COMPILED_IDENTITY` equals `PINNED_RECORD_IDENTITY`, and otherwise print `I65_G6_RECORD_SKIP`. Seen in the release build |
| 4. This record | This file |
| 5. Prepare the registration | `registration.diff` (§8). With it applied in a scratch copy, the PP suite passes except for the known Mac t13. `admit` grants the milestone a permit in both modes |

## 2. The 42 Estimate atoms, closed (generator `_run_records/g5_profile.py`)

Each atom is bound to the actual owner its roster line denotes, read at the G5 basis:
- **InBuild:** `size_of`/`align_of` of the type in this build, directly or through `structural::retained_resource` (which gains 21 kernel exports and 3 alignments);
- **SourceUpper:** a field-sum upper bound over a private type's nameable field types. This is sound because a `repr(Rust)` struct is no larger than its fields laid out in declaration order, each padded to the struct's alignment.

**Where an owner is already priced elsewhere, the atom is bound to that owner again**, a conservative double count. That holds for:
- the C2 input maps, which are ProductCapture's identity vectors (T11.1/T11.3);
- the conversion records;
- the lane terminals.

| Atom | Binding | In-build value (B) | G3 stride (B) | Bound to |
|---|---|---|---|---|
| `Node(TrackerKey,())` | SourceUpper | 256 | 512 | TrackerSet.holding: BTreeSet<(RuleTest, u32, Kind)> |
| `Node(TrackerKey,Tracker)` | SourceUpper | 1,224 | 4,096 | TrackerSet.trackers: BTreeMap<(RuleTest, u32, Kind), BoundedExtremeTracker>; RuleTest (private, fieldless) <= 1 byte |
| `s((usize,VecPair))` | InBuild | 32 | 32 | CasePrep.prescribed: Vec<(usize, Vec<(f64, f64)>)> (adaptive.rs:926) |
| `s(AdapterSnapshot)` | InBuild | 296 | 512 | PreparedTrace.adapter: PrivateAdapterSnapshot |
| `s(AliasE)` | InBuild | 80 | 32 | the two alias LocatedQuantity values (their strings are B(P_final*RID)) |
| `s(ArcPrepared)` | InBuild | 32 | 512 | Arc<ProofAnchor{owner: ProductOwnerStamp}>: two counters and the value |
| `s(BodyCoverage)` | InBuild | 64 | 32 | per body: the kernel's and PP's ProductSummaryCoverage and row_scales' scales [f64; 4] |
| `s(BodyMapE)` | SourceUpper | 52 | 56 | C2 body map entry (two u32 child lists and an id): no distinct owner at this basis (source.body_of_node is in PrimitiveSource; body_nodes() temporaries are covered by C_u32(n,b)); the entry upper is kept |
| `s(ConstraintMapE)` | InBuild | 32 | 40 | C2 constraint map: the kernel Constraint or a support identity entry, the larger |
| `s(ContributionE)` | InBuild | 8 | 40 | assemble.rs: Structure.items Vec<Contribution> |
| `s(ConversionE)` | InBuild | 32 | 48 | the projection conversion record (usize, Binary64Outcome): no second owner; a double count |
| `s(ConversionEvent)` | InBuild | 40 | 64 | the 9 inline conversion outcomes per member (inside SectionPreparationWork): a double count |
| `s(CoverageFact)` | InBuild | 24 | 32 | coverage_facts' coordinates Vec<[f64; 3]> (<= n) |
| `s(EnclosureE)` | SourceUpper | 288 | 304 | Enclosure {lo, hi: Wide<16>} (product_certificate.rs:16): LaneReadouts.rows |
| `s(Expansion6)` | InBuild | 192 | 144 | rigid_body.rs:210: [Expansion; 6] per node |
| `s(FinalRowConversion)` | InBuild | 32 | 64 | the final-row conversion record (usize, Binary64Outcome): a double count of projection_outcomes |
| `s(IntervalE)` | SourceUpper | 288 | 32 | run_case's k: Vec<Enclosure> (Q); the second Q term is slack |
| `s(LaneTerminal)` | InBuild | 4,168 | 256 | the lanes' terminal work (Option<ResidualWork> x2 inline in ProductCertificateSpent): the whole record, per lane |
| `s(LawE)` | SourceUpper | 112 | 64 | bridge::ProposedMemberLaw {&StraightMember, D, t, MaterialOperands (<= 9 f64 + tag), z} by field sum |
| `s(LazyE)` | InBuild | 4,304 | 2,232 | BoundedExtremeTracker.lazy: (ExactWideSum, ExactWideSum, u64, u64) |
| `s(MaterialDescriptor)` | InBuild | 272 | 96 | ProductCapture.selections / materials, the larger |
| `s(MaximumE)` | SourceUpper | 193 | 128 | per member: the kernel maxima slot (usize, u32), PP's ProductMaximumValue output, PP's PreparedMaximumPatch {usize, usize, u32, [Number; 8]} (field sum) and the completion flag |
| `s(MemberMapE)` | InBuild | 64 | 64 | C2 member map = ProductCapture.members (MemberIdentity); also in T11.3 |
| `s(NodalMapE)` | InBuild | 64 | 48 | C2 nodal-load map = ProductCapture.terms (TermIdentity); also in T11.3 |
| `s(NodeMapE)` | InBuild | 48 | 40 | C2 node map = ProductCapture.nodes (String, [f64; 3]); also priced in T11.1 |
| `s(Pair)` | InBuild | 16 | 16 | the prescribed pairs (f64, f64) (adaptive.rs:926) |
| `s(PreparedMemberEvent)` | InBuild | 1,304 | 256 | per member: trace.members PreparationEntry, associations, preparations PreparedAnnulus, preparation_work SectionPreparationWork |
| `s(ProductRow)` | InBuild | 112 | 96 | per final row: the descriptor ProductFinalRow and PP's verdict copy |
| `s(ProductValue)` | InBuild | 9 | 32 | per final row: the values f64 and (21m <= P_final) derivative_coverage bools |
| `s(ProjectionOutcome)` | InBuild | 32 | 64 | ProductCertificateSpent.projection_outcomes: Vec<(usize, Binary64Outcome)> |
| `s(RegistryE)` | SourceUpper | 1,608 | 128 | OriginStore's six registries (origins.rs:290-297), each <= 8 entries for one case: one entry of each, SelectedOrigin {Arc, usize, usize, SlotSnapshot} by field sum |
| `s(Row6)` | InBuild | 48 | 48 | rigid_body.rs: Vec<[f64; 6]> rows and motions |
| `s(RunRow)` | InBuild | 1 | 32 | row_scales' native_coverage Vec<bool> (Q) |
| `s(SectionMapE)` | InBuild | 160 | 64 | C2 section map = ProductCapture.facts (ProductMemberFacts); also in T11.3 |
| `s(SpringMapE)` | InBuild | 32 | 48 | C2 spring map = ProductCapture.spring_map (SpringIdentity); also in T11.3 |
| `s(StationMapE)` | InBuild | 16 | 32 | C2 station map: the kernel Station (SourceParts.stations) |
| `s(SupportCoverage)` | InBuild | 6 | 64 | row_scales' support_coverage Vec<bool> (6g) |
| `s(SupportMapE)` | InBuild | 38 | 64 | C2 support map = ProductCapture.supports (String, usize) and support_fixed [bool; 6]; also in T11.3 |
| `s(SupportVector)` | InBuild | 72 | 80 | CaseRecord.support_vectors: Vec<(String, [f64; 6])> (preview_physics.rs:170) |
| `s(TableE)` | SourceUpper | 56 | 48 | BoundedExtremeTracker.table: (u64, Evaluated); Evaluated (private) <= tag + seq u64 + AttemptStop by field sum |
| `s(Tag)` | InBuild | 16 | 8 | assemble.rs: tagged Vec<(usize, Contribution)> |
| `s(TrackerE)` | SourceUpper | 4,472 | 64 | one BoundedExtremeTracker over F offers, per offer: a lazy row, a table and a kept-table entry and the stable-sort scratch entry (P3 ENVELOPE Tracker(O) law; the pivot, residual and fallback trackers) |

**Larger than G3's stride, and why:**
- `TrackerE` is **4,472**, not 64. G3 priced a tracker per offer as one small entry. One `BoundedExtremeTracker` over F offers holds, per offer:
  - a lazy row (two `ExactWideSum`s and two u64s: 4,304 B in this build);
  - a table entry, a kept-table entry and a stable-sort scratch entry.
  
  These are P3 ENVELOPE's Tracker(O) law, applied to the pivot, residual and fallback trackers.
- `LazyE` is 4,304, against 2,232.
- `LaneTerminal` is 4,168: the whole `ProductCertificateSpent`, per lane.
- `RegistryE` is 1,608: one entry of each of OriginStore's six registries.
- `PreparedMemberEvent` is 1,304: per member, the trace entry, the association, the prepared annulus and its `SectionPreparationWork`.
- `IntervalE`/`EnclosureE` are 288: an `Enclosure` is two `Wide<16>`.

Closing the 42 atoms adds **+9,771,352 B** to the maximum in both modes. Part 2's in-build W3, plus the audit's +4,316,002 B on TAV_W, was 3,565,145,772 (sparse) and 3,584,856,220 (dense); it is now 3,574,917,124 and 3,594,627,572.

## 3. The in-build maximum: E_req,max and E_mov,max per caller and mode

**Callers.** D1.0 refuses Headless (D-2), so only the **Direct** caller is admitted. The Headless caller has no admitted maximum.

**Phase totals** (requested + moving, without R) in the qualified dev/test build, the pinned record (`per_identity/profile_record.test.txt`). The **release build's record is byte-identical** (`profile_record.release.txt`): layouts do not depend on the opt level here.

| Phase | Sparse | Dense |
|---|---|---|
| W1 ordinary span | 1,856,156,348 | 1,875,866,796 |
| W2 G-B, G-C, T12–T15, N1 reserve | 1,963,966,754 | 1,983,677,202 |
| **W3 publication (T16) + staged copy** | **3,508,669,422** | **3,528,379,870** |
| W4 precommit (T17) + successor + invocation | 3,482,311,587 | 3,502,022,035 |
| W5 transfer and Direct completion | 2,136,222,836 | 2,155,933,284 |
| X1 ordinary span with T25 | 3,256,308,814 | 3,276,019,262 |
| X2 X completion | 1,777,046,728 | 1,796,757,176 |

**The repair's change to every phase** (both modes, against G6's record): W1–W5 +861,162 = TAV_W +861,146 plus 16; X1–X2 +845,828 = TAV_X +845,812 plus 16. The 16 B is `s(ThreadPacketOutput)`: `RetainedPreviewOutput` holds the admission report, whose private law record gains S-3's `required: Option<u64>`. No other atom or form moved (`_run_records_g6r/per_identity/`).

| Direct | Sparse | Dense |
|---|---|---|
| E_req,max (W3 requested) | 3,319,366,141 | 3,339,076,589 |
| W3 moving extra | 189,303,281 | 189,303,281 |
| **E_mov,max** (W3) | **3,508,669,422** | **3,528,379,870** |
| E_mov,max + R | 3,575,778,286 = **0.8881 M** | 3,595,488,734 = **0.8929 M** |
| Below 0.9 M (3,623,878,656) | 48,100,370 | 28,389,922 |

**The margin standard** (RR "RV89 on U4 G5 part 2: PASS; … the margin standard").
- **The text-error budget**, the fraction of TAV_W that would consume the 0.9 M margin, is:
  - **dense:** 28,389,922 / 1,570,041,862 = **1.81 %** (G6: 1.86 %);
  - **sparse:** 48,100,370 / 1,570,041,862 = 3.06 % (G6: 3.12 %).
- The figure was about 2.8 % at part 2. The audit and the closed Estimates used the difference.
- **The in-build maximum is still ≤ 0.9 M after the identifier audit.** So the phase-aware span (−0.085 M) stays in reserve, unused.

**The 0.9 M rule holds after the Estimates closed, the audit and its repair.** The in-build evaluation is the admission's own bound: `cap_priced_maximum(mode)` returns E_mov,max, and `admission_bound` adds the constant R and compares with the registered M (RV89 G6 S-3: a named pure function, tested at M − R − 1, M − R and M − R + 1; `admit` prices its bound only through it, and the law record keeps the `required` bytes it computed). So a build whose layouts moved the maximum above M would be refused at admission, not admitted.

**Test-only layouts.** The pinned record comes from the test harness's build of the crate. The production build of the same identity compiles without `#[cfg(test)]`. The only PP types with test-only fields are `ProductCapture`, `FrozenCandidate` and `ReservedNotice`:
- none is a profile atom;
- none is contained in an atom's type;
- `RetainedPreviewOutput` (inside `s(ThreadPacketOutput)`) holds neither.

The kernel and SR are compiled without `cfg(test)` in both cases. **So the production build's evaluation equals the record.**

## 4. R and the S1 stack evidence (measured evidence for these builds and inputs, not a proof)

- **R** = `RESERVED_STACK_BYTES` = 64 MiB per invocation thread. The witness stack is R/k with k = 16, so 4 MiB.
- **The nine witnesses** (`per_identity/witnesses.{test,release}.txt`) ran one process each, in **both** the dev/test and the release build. All pass; none overflowed, aborted or panicked:

| Witness | Sparse | Dense |
|---|---|---|
| W1 milestone | Successor | Successor |
| W2 cap-maximal D1 (quote and backslash in every provenance, raw depth 16) | Fallback(Preparation), asserted | Fallback(Preparation), asserted |
| **W2-deep** (RV89 S-2): the milestone with a quote and backslash in every provenance and raw depth 16 | **Successor at 4 MiB and at 1 MiB** | **Successor at 4 MiB and at 1 MiB** |
| W2b cap-maximal, solvable | Fallback(Candidate) after the full native run, asserted | Fallback(Candidate), asserted |
| W3 n05 exact-selected (T25) | ExactSelected | ExactSelected |
| W4 preparation refusal | Fallback(Preparation) | Fallback(Preparation) |
| W5 | the dense halves above | |
| W6 force-scaled (PHYS-R4 cantilever) | Fallback(Native) | Fallback(Native) |
| W7 U3 faults (sparse) | Native; Serializer(Encoding); Staging; Precommit G8; Precommit G1 | |
| Headroom: W1 at R/64 = 1 MiB | Successor | Successor |

- **The deepest call chain** from the Direct root is **40 frames** (RV83 N-3; R-4's call graph). That is about 1.47 MiB by G3's frame arithmetic, so R and k are unchanged.
- **Stated limits:**
  - **Carry 8.** W1's successor is validated by the precommit reader, so the 1 MiB pass bounds the reader's schema walk on W1's input. Whether that walk reaches the deepest 36-level `$ref` chain is not observed: that needs reader instrumentation, outside U4's fence.
  - **The panic-hook path.** A panic on the reserved thread runs the panic hook on that thread, on top of the panicking frame. With `RUST_BACKTRACE`, that includes backtrace capture and symbolication. No witness drives a panic (STACK_INVENTORY.md §2).
  - **The call graph is lexical, not a compiler one** (R4_CALLGRAPH.md §3).
  - **Envelope depth is fixed by the grammar,** so only counts grow, and frames do not scale with counts.

## 5. The build identity, reviewed inputs and layouts

**Registered (ROOT's ruling 1): one identity**, the dev/test build on this host:

```
v1;rustc.release=1.97.1;rustc.commit=8bab26f4f68e0e26f0bb7960be334d5b520ea452;rustc.host=aarch64-apple-darwin;rustc.llvm=22.1.6;target=aarch64-apple-darwin;target.arch=aarch64;target.pointer_width=64;target.endian=little;target.os=macos;target.env=;panic=unwind;profile=debug;opt_level=0;debug_assertions=true;rustflags=;pkg=open_pipe_stress_product_physics@0.2.0
```

That is: profile `debug` (the test profile inherits dev), opt-level 0, debug assertions on, `panic=unwind`, no RUSTFLAGS, rustc 1.97.1 (`8bab26f4f68e`), aarch64-apple-darwin.

**Why this one:**
- It is the build in which `cargo test` runs PP on this host. That covers every U3/U4/U5 control so far.
- It is the build the milestone run will use: U3 grant 2's facade E2E and U7's publication through the facade in both modes, which are PP tests.
- So are U9's both-entry gates, where they run on this host.

**Qualified, ready but unregistered (ROOT's ruling 1): the release identity** (`profile=release;opt_level=3;debug_assertions=false`, otherwise identical). It is **qualified here**: its record equals the dev record and all nine witnesses pass (`per_identity/*.release.txt`). Nothing named in U3 grant 2, U7 or U9 runs a release PP build on this host, and the brief's rule is to register nothing that will not be run. **When a release run is named, registering it is a one-entry change:** its identity text from `per_identity/profile_record.release.txt`, the same reviewed inputs and reader layouts, and the same M.

**Not qualifiable here:** hosted CI's Linux builds (another target and toolchain). Each such build is `Stale` at D1.1 and keeps the ordinary route, so it is fail-closed. Registering one needs its own record, witnesses and reviewed inputs on that host.

**Reviewed inputs** (D-6; the PP lock and the reader's 13 statics, SHA-256), as compiled in the qualified build. Every input was read, with no `=unavailable` (RV89 N-4's guard):

```
v1;Cargo.lock=4f494db6d8a6eca87e7a16d8561197f20b1951a033bd3a6c424acfff5613475b;../../schemas/physics_source_recovery.schema.json=3bb969555d5616af6eefdb68788ee4a74a8a3681c42fe9aae577a5d25a51ac5c;../../schemas/retained_precision_mp_v2.schema.json=07951edacfedd410c153929ee75bb5bada15dbd222369ec63240c678b233b61c;../../fixtures/results/retained_precision_prepared_ordinary_v1.json=3e0779a45a74cf0bb3a4ed08ed3a6b44347aea8a3c33b59e9dd92130426ee296;../../fixtures/results/semantic_contract_v0_3_preview_physics_retained_1.json=c74742ce6a936384e00986006e6a0b2e6bb11f190451e876eed9ffa11903c6a8;../../fixtures/results/semantic_contract_v0_2.json=4d6886d19e304db897e5e9f8f0054cbee91ba7795868f9698e2bbe070bde94da;../../fixtures/results/semantic_contract_v0_3_precision_1.json=d75aacee175e178dbdeb256d89a65f4b375265f7da077725ee635af33df51d7e;../../fixtures/results/semantic_contract_v0_3_physics_1.json=9a2cf6268b57bd5265a1a115497c07450819dd4d03cd5ab618097bd9d19da8cc;../../fixtures/results/semantic_contract_v0_3_load_reference_1.json=44bc41c06f589fab6ce931ac0eaa5344765ff64fd5f880cc2dd69ecb839c4f4d;../../fixtures/results/semantic_contract_v0_3_load_reference_source_1.json=d1628194a7730f427843b00228dd233cf92b8e7d26f3bc31c660a3ea59e28337;../../fixtures/results/semantic_contract_v0_3_preview_physics_1.json=ae55503d44a4750714a35c423623e38cf4132099134097193024d1635bfbc88a;../../fixtures/results/semantic_contract_v0_3_physics_source_1.json=ba13f2aefd7a38bd725e5f111e6ec30144bc8776aa957c6278ee7b1178298ba1;../../fixtures/results/semantic_contract_v0_3_source_blocks_1.json=5f299065f15a157bbedf9467a598994ae684c4ecb3f851bbcb291981ec550a9f;../../schemas/source_block_recovery.schema.json=544e196d2f7bef27276acc160aa19ab738a4f7949e846d2e8871328d2208129c
```

**Reader layouts** (`READER_LAYOUTS`):

| Type | Size | Align |
|---|---|---|
| Validation | 56 | 8 |
| ValidationError | 64 | 8 |
| RowClassification | 96 | 8 |
| AccuracyClass | 16 | 8 |

The std entry-tuple layouts are pinned by the compiler identity (RV89 N-3, accepted).

## 6. The D1 predicate and M (D-7)

**D1** is as amended (DOMAIN.md §1–§3; G2_AMENDMENTS; ADDENDUM_L128; RR "U4 G4" for D1.10/D1.11). The clauses are checked in order:
- **D1.0** Direct caller;
- **D1.1** the build is registered (identity, reviewed inputs and layouts all bind);
- **D1.2** complete censuses;
- **D1.3** the legacy namespace (schema 0.1.0/0.2.0; no pressure contract, reference configurations, expansion laws, sections or section refs);
- **D1.4** one load case, no combinations or components;
- **D1.5** case fields absent;
- **D1.6** no hanger or nonlinear support, family exact or none;
- **D1.7** nodal force/moment loads;
- **D1.8** straight members;
- **D1.9** the cap table (n, m, g ≤ 32; Σr ≤ 192; l ≤ 128; text ≤ 128 B; raw ≤ 16,384 values, depth ≤ 16; …);
- **D1.10** no object-like provenance;
- **D1.11** no control bytes.

Refusals map to `caller`, `resource_admission` or `source_family`, as in DOMAIN §3.

**Proposed M (D-7): 4,026,531,840 B**, the per-invocation W1 admission threshold on requested and moving heap bytes. In the qualified build, E_mov,max + R = 0.8927 M, which meets the 0.9 M margin.

**Non-claims (D-7):**
- **Not RSS, allocator overhead or fragmentation.** M counts requested and moving heap bytes per invocation.
- **No concurrency claim.** One invocation's bound; concurrent invocations are not composed.
- **No supported-machine claim.** A machine reading is owner-held.
- **No stack claim beyond §4's measured evidence.**
- **The reader's process-lifetime statics** (13 `OnceLock` caches, `STATICS` = 5,397,696 B) are counted in **every** phase (RV84 C-N3(b)).
- **Not a proof of the ordinary or native solvers' internal bounds** beyond the accepted rosters. The profile prices the accepted rosters (I29 P1/P3, I51 COMPOSITION, I54) at the D1 caps, with in-build strides.

## 7. The identifier-class audit (ROOT's addition)

See `ID_CLASS_AUDIT.md`. In short:
- **Coverage.** Every identifier-bearing copy on the D1 graph (795 entries, 641 sites) is priced by its source: input 128, the node-DOF label 131, result id 1,024, diagnostic id 2,330, its own template, composite 600, or static.
- **Enforcement (repaired, RV87 SF-3).** Every identifier-bearing candidate at a positive-multiplicity site, chosen by type and whichever rule would price it, must be in the table, or TEXT is incomplete (`id-unaudited`). A site key matching no row (`stale-key`), or an entry naming no expression at its site (`stale-audit-entry`), also makes it incomplete. Eight controls each fail with their own finding, including RV87's `primitive_loads/src/lib.rs:299` removal and a stale key (`_run_records_g6r/controls/`); G6's own control (`lib.rs:5592`) still fails.
- **TEXT** goes to 2,150,800,830 B: +33,295,754 B at G6 and +898,784 B in the repair. TAV_W is 1,570,041,862 (+861,146 in the repair) and TAV_X is 1,440,401,002 (+845,812).

## 8. The registration change (prepared, not applied): `registration.diff`

**What it changes** (5 files, 337 lines; G6's reviewed 287-line version is kept as `registration.g6.diff`, and the repair's 323-line version as `registration.g6r.diff`). The last three files are outside U4's fence; ROOT ruled that such flips are applied with the entry, as part of the same reviewed change (ruling 3; RV89 G6 S-1 adds the runner's):
- **`retained_memory.rs`:** `REGISTERED_PROFILES` gets one entry: the identity, the reviewed inputs, the reader layouts and `threshold_bytes: 4_026_531_840`. Decision 7 holds: this is the production profile, registered by reviewed change. There is no test permit and no constructor.
- **`retained_memory_law_tests.rs`:**
  - `the_registered_profile_is_the_only_permit_source` replaces `no_profile_or_permit_is_constructible`. It checks: exactly one entry, the pinned identity, M; a forged index past the list or a refused report mints nothing; and this build is Registered or Stale, never Missing.
  - **`admit_grants_a_permit_for_the_milestone_in_the_registered_build`:** in the registered build, `admit` returns a permit for the milestone in both modes, with no refusal, its law record's `required` equal to `cap_priced_maximum(mode) + R` (RV89 G6 S-3), and `required ≤ 0.9 M`. Headless is still refused at D1.0. A two-case variant is refused at D1.4. In another build: `Stale`, with nothing `required`.
  - Two tests now expect D1.1's status from `build_status()` instead of `Missing`.
  - **`registered_g_c_declines_only_unattempted_solves`** (ROOT's ruling 2(a)), in the registered build:
    - each not-attempted example takes G-C's `CompleteGate(OrdinarySolveNotAttempted)` fallback, and Direct's bytes equal the value route's exactly, with no notice. The examples are an invalid document kind, an invalid load category, no supports and a lone spring;
    - the milestone publishes in both modes;
    - a failed attempt (a 1e-300 spring, NumericallyUnresolved) and the `rejected_stress_range` pair still reach W1.
- **`retained_memory.rs` `tests`:**
  - `missing_stale_overflow_and_unknowns_cannot_mint_a_permit` sets the law's refusal with the forged profile status;
  - `actual_retained_entry_dispatches_ordinary_once` counts the unpermitted dispatch on a two-case (out-of-D1) milestone.
- **`retained_facade_tests.rs`** (outside U4's fence): `u3_no_permit_entries_are_the_ordinary_route` uses the two-case milestone, which no build admits.
- **`tests/retained_precision_admission.rs`** (outside U4's fence):
  - the expected profile is `Registered` in the registered build and `Stale` otherwise;
  - the invalid-document request's Direct bytes equal the value route's exactly in every build: its ordinary route never attempts the solve, so G-C declines.
- **`runner/headless/tests/retained_precision_admission.rs`** (outside U4's fence; RV89 G6 S-1): `explicit_headless_refusal_…` expects the Headless report's profile to be this PP build's own status (`Registered` in the qualified build, `Stale` otherwise, never `Missing`). The runner's workspace builds PP with the registered identity, so under registration the report reads `Registered`. Headless itself stays refused at D1.0, and the output still equals the ordinary run's. **The oracle is the Direct entry on the two-case variant,** which D1.4 refuses after the build clause (RV89 G6r N-1). The report still carries the build's profile, but the runner workspace, whose lock is not PP's reviewed one, is never granted a permit and never runs W1. The test also asserts the ordinary bytes, no successor and two load cases. A scratch permit probe confirms that no runner test is granted a permit (RETURN.md, Addendum 3).

**Tested in a scratch copy** (`_run_records/registration/`):
- **The repair's re-run** (`_run_records_g6r/registration/`, the updated diff): PP 699 passed, 1 failed (t13), 11 ignored; runner/headless identical to base, with S-1's test passing; the sweep sha256 unchanged at `3b22de97…0f60`; the challenge peaks 3,541,898 / 2,252,863 B against E_mov,max 3,508,669,422 / 3,528,379,870 B. G6's figures follow.
- **PP:** 698 passed, 1 failed (the known Mac t13), 11 ignored.
- **Challenge:** the milestone now runs the **permitted** path with a successor, peaking at 3,541,866 B (sparse) and 2,252,831 B (dense), against E_mov,max 3.51/3.53 GB. The large input is not permitted (no successor) and stays within its W1 phase.
- **The fixture sweep changes, as registration should** (`sweep_reg_diff.txt`):
  - every retained report's `profile` goes Missing → Registered (134 lines, report-only);
  - the milestone publishes a successor in both modes (its ordinary envelope bytes are unchanged);
  - the `rejected_stress_range` pair's Direct envelopes gain the N1 notice. Their ordinary solve **ran** (a Sensitive report), and only the legacy source-block finalization then blocked the envelope. So by ROOT's rule G-C lets them proceed, their W1 falls back, and U3's designed fallback applies;
  - no admitted sweep fixture is an unattempted solve, so G-C's new fact changes no sweep line. The registered sweep's sha256 is the same as before the rule: `3b22de97…0f60`.

**Two consequences for ROOT and U3 to know before applying:**
1. **Blocked ordinary runs: ROOT's ruling 2(a) is implemented (§8a).** G-C declines W1, with exact ordinary bytes and no notice, when the ordinary route returned without attempting the case's solve. A case whose solve ran, at any quality, still proceeds. That includes `rejected_stress_range`, whose solve ran before its envelope was blocked: it keeps U3's N1 notice. Public activation remains U7's.
2. **The permitted path's single ordinary dispatch (U3's B-1) is not counted by the existing hook.** `ordinary_dispatch_entered()` counts only the unpermitted dispatch, and counting the permitted one needs a lib.rs hook, outside U4's fence. The witnesses and the facade bytes show one observed run, but no counter asserts it. **Routed to I61's U3 grant 2** (ROOT's ruling 2(b)).

## 8a. G-C's attempt fact (ROOT's ruling 2(a))

**The fact.** `PhaseFact::OrdinarySolveNotAttempted` is the last of G-C's 19 facts, and its bound is 0. Its observed value is 1 when `ordinary_solve_attempted(capture)` is false. A refusal there is `W1Fallback::CompleteGate`: `permitted_run` returns the ordinary envelope untouched, and the N1 notice is reserved and rendered only inside `retained_w1`. So the bytes are exact and there is no notice. The fact lives in `retained_memory.rs`, so no `lib.rs` change is needed.

**The predicate, from source** (`ordinary_solve_attempted`, cited in its doc comment): `!capture.ordinary.is_empty() && capture.ordinary.iter().all(|seed| seed.initial.is_some())`.
- **The observer's own G-b record decides, not the envelope.** `blocked_envelope` (lib.rs:13636, `status.mechanics = MODEL_INCOMPLETE`, `run_id = run:preview-linear-static-blocked`) is returned both before and after an attempted solve:
  - `solver_blocked`, :13703, after a solver error;
  - `has_blocking` after `solve_load_case_observed` returns, :2705.
  
  So the status is ambiguous, and the seed is not.
- **`OrdinarySeed::initial` is set exactly at the attempt.** It is `ordinary_initial_failure` at lib.rs:4146–4148 for `Err` from the attempt at :4125. That covers a structural failure and a deferred-basis formation refusal, the F1b attempt that routes to W2. Otherwise it is set at the published report, `ordinary_report` at :4543. That is the only exit of an `Ok` attempt inside D1: the other exits after :4125 are :4351 and :4371/:4382, which are `Err`-only, and :4510, which is nonlinear-only and excluded by D1.6.
- **Every return before the attempt leaves no seed, or a seed whose `initial` is None.** That covers:
  - validation's `blocked_envelope`s in `run_linear_static_preview_observed`, :2412–2467;
  - the mechanism and stiffness `solver_blocked` before the case solve;
  - `solve_load_case_observed`'s load-input return (:3918) and its ledger and reduction `?` exits (:4011, :4042, :4098);
  - the load-state returns :3954 and :4064, which D1.3 excludes.
- **D1.4 admits one case, so there is one seed.** The predicate is unambiguous for every ordinary outcome inside D1. **No stop.**

**Every example checked** (`g_c_declines_w1_when_the_ordinary_solve_was_not_attempted`, both modes; probe at `_run_records/controls/g6_attempt_probe.txt`):

| Example (inside D1) | Ordinary outcome | Attempted | G-C |
|---|---|---|---|
| Invalid `document_kind` | validation: `PREVIEW_DOCUMENT_KIND_INVALID`, blocked; no seed | no | declines: exact bytes, no notice |
| Invalid load category | `LOAD_INPUT_INVALID` (:3918), blocked; no seed | no | declines |
| No supports | validation: `SUPPORT_INPUT_MISSING`; no seed | no | declines |
| A lone spring support | `SOLVER_SYSTEM_BLOCKED` before the attempt; no seed | no | declines |
| The milestone | solved, `NUMERICAL_INTEGRITY_SENSITIVE` report | yes | proceeds, publishes |
| A 1e-300 spring | the attempt fails `NumericallyUnresolved`, then blocked | yes | proceeds to W1 |
| `rejected_stress_range` (both) | solved Sensitive, then `SOURCE_BLOCK_RECOVERY_FINALIZATION_FAILED` blocks | yes | proceeds; N1 notice on fallback |
| W6, PHYS-R4 force-scaled | the attempt fails Range, then W2 publishes | yes | proceeds |
| K2a's partial-underflow product reach (repair, RV89 G6 S-2) | F1b's deferred formation: the attempt's seed is `FormationFailure` (asserted) | yes | proceeds; under registration W1 falls back at `Candidate` with the N1 notice (RV89's observation) |

**The deferred-formation arm is pinned** (repair, RV89 G6 S-2). `attempted_examples()` now includes K2a's `product-reach-partial-underflow` shape (built in the law tests as K2a's `PARTIAL_UNDERFLOW` request), and `g_c_declines_…` asserts that its one seed is `InitialSeed::FormationFailure`. So `g_c_declines_…` and the registered `registered_g_c_…` both pin the arm; RV89's R8 (a `FormationFailure` seed not counted) is now killed (§9).

## 9. Controls (unregistered G6 candidate, against base `8abb5274a9`; `_run_records/controls/`)

**The repair's re-run** (the same controls on the repaired code, `_run_records_g6r/controls/`) is summarised in RETURN.md, Addendum 2; the table below is G6's.

**Host:** the default toolchain (rustc 1.97.1), `--locked --offline`, `CARGO_BUILD_JOBS=4`, `RUST_TEST_THREADS=2`, one cargo job at a time, with memguard (PID 5387) checked before each job.

**The candidate copy** is `1e323058f3`'s archive with the G6 tree's seven changed files overlaid, plus base's `apps/`, which is identical at `8abb5274a9` and `1e323058f3`. `diff -rq` confirms it equals the worktree's core tree.

| Control | Result |
|---|---|
| No published byte changes (unregistered) | **The fixture sweep is byte-identical to base**, sha256 `0690bc64…41e1` for both, before and after the RV89 part-2 tests. With nothing registered, every report still says `profile: Missing`, and a priced bound is never reached |
| Nothing weakened: PP | 696 passed, 1 failed, 11 ignored (after ruling 2(a)). Against base, the only differences are the U4 tests (`retained_memory::*`, 9 ignored witnesses) and the challenge. The failure is the known Mac t13, as at base |
| runner/headless | 85 passed, 2 failed: identical to base |
| FK | 546 passed, 1 ignored: identical to base |
| SR | 48 passed: identical to base |
| Registration applied (scratch copy, §8) | PP 698 passed, 1 failed (t13), 11 ignored |
| The audit enforcement | Removing one audited entry makes TEXT incomplete (`controls/audit_enforcement.txt`) |
| Mutants | **167 run, 163 killed by a test, 0 compile-only** (`controls/mutants_g6.out.jsonl`). The sets are part 1's 84, RV89's 24, part 2's 40 and G6's 19. G6's 19 are nine FK exports, four profile bindings, RV89's Q10 (killed by `maximum_takes_every_phase`) and five mutants of the new G-C attempt fact, all killed by `g_c_declines_w1_when_the_ordinary_solve_was_not_attempted`. **The 4 survivors are all recorded:**<br>– "build_status ignores bindings" is equivalent by decision 7, since nothing is registered in the worktree;<br>– RV89's V19 (build.rs) cannot be observed in-crate;<br>– "Estimate count ignored" and "estimates not counted" are equivalent now that ESTIMATES = 0. `priced_maximum(1, ·)` is still tested as `Unpriced` |

## 10. RV89 on part 2: the routed items (RR "RV89 on U4 G5 part 2: PASS; …")

- **S-1 (inside the identifier audit): the five result-id copies are priced by source.** Each is RES, at 1,024 B (ID_CLASS_AUDIT.md §2, `id_audit_table.json`):
  - `source_receipt.rs:1016`, `r.result_id`: a `RowTreatment.result_id`, which is a copy of a ResultItem id;
  - `rows.rs:533`, `vec![primary[&function].id.clone()]`: `primary` is a `&BTreeMap<usize, ResultItem>`;
  - `rows.rs:590` and `:592`, `binding.result_id`: a `FunctionalRowBinding.result_id`, built from `matched[0].id` at lib.rs:5592;
  - `lib.rs:5592`, `matched[0].id`: `matched` is a `Vec<&ResultItem>`.
  
  None is priced by receiver spelling. The `^(row|result)\.id$` rule is no longer what prices any audited site: the audit table answers first, and an un-audited identifier hit makes TEXT incomplete.
- **S-2: the deep-input witness is committed,** as `witness_w2_deep_milestone_publishes`. It takes the milestone, puts a quote and a backslash in every provenance and adds a raw value of depth 16. It is inside D1 (`raw.maximum_depth == 16` is asserted) and **publishes a successor at R/16 = 4 MiB and at 1 MiB, in both modes, in both qualified builds** (`per_identity/witnesses.*.txt`).
  - W2 now escapes every provenance too, and asserts `Fallback(Preparation)`.
  - W2b asserts `Fallback(Candidate)`.
- **S-3: `maximum_takes_every_phase`** is a pure test. Each of the 7 phases in turn is the largest, by its total and by its moving part alone. A tie keeps the earliest phase, and an overflow in any phase (requested, moving or their sum) gives no maximum. RV89's Q10 (`while i < 5`) is in the G6 mutant set and is killed (§9).
- **Re-pin per registered identity:** done. `PINNED_RECORD_IDENTITY` is the proposed identity, and the record test skips in any other build.
- **RV85 U1 is not required before registration** (ROOT); recorded.

## 11. Carries and routed items

- **RV83 N-3:** 40 frames, carried in §4. N-1 and N-2 (optional) are not taken: the `as_deref_mut()` unwrap drops one text-free call.
- **RV85 U1** and **carry 8 as partial:** accepted as stated by ROOT; restated in §4 and §8.
- **ROOT's ruling on decision 1:** the pinned record is identity-gated (§1).
- **ROOT's ruling on decision 2:** N-6 stays a part-2 erratum, unchanged.
- **Re-qualification obligation (RV87 G6r N-1).** The identifier audit's candidate predicate is syntactic (ID_CLASS_AUDIT.md §1, Residual). At this basis RV87's by-type sweep of all 410 non-candidates finds no identifier alias, so the residual is accepted for registration. At the next re-qualification, re-run that sweep, or close the residual with an explicit-row rule, before the profile is re-registered.
