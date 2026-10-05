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
