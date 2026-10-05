"""I65 U4 G5 part 2: generate retained_memory.rs's cap-priced profile from the profile tree
(stdlib only).

Input: profile_tree.json, written by this folder's g4_caps.py (PROFILE_TREE=...). It holds every
term of the admission maximum as a linear form over layout atoms (counts already substituted at the
D1 caps, l <= 128), the sums and maxima that combine them, and the phases X1-W5 per mode.

Output: the Rust block between the GENERATED PROFILE markers of PP/src/retained_memory.rs. Every
atom is bound here to exactly one of:
  InBuild      size_of/align_of of the actual type, in the PP build (directly, or through the
               kernel's `structural::retained_resource` and SR's `elastic_extrema::NODE_STRIDE`);
  SourceUpper  a source-derived upper bound for a type the product cannot name (cited);
  Text         a text atom: a byte count from the T08 text closure at this basis (layout-free);
  Estimate     a G3 design-record stride with no single in-build type identified yet. The profile
               is INCOMPLETE while any Estimate carries weight: cap_priced_maximum fails closed.
Every maximum is taken in the build, over the evaluated candidates (never chosen here).
Usage: python3 g5_profile.py <profile_tree.json> <retained_memory.rs>   (rewrites the block in place)
"""
import json, re, sys

tree = json.load(open(sys.argv[1]))
target = sys.argv[2]

def sz(t):
    return ("InBuild", f"size_of::<{t}>()", "")
def fk(c):
    return ("InBuild", f"fkr::{c}", "kernel export structural::retained_resource")
def node(k, v):
    return ("InBuild", f"btree_node_upper(size_of::<{k}>(), align_of::<{k}>(), size_of::<{v}>(), align_of::<{v}>())",
            "BUILD.md §4: max(Leaf_up, Internal_up) at the in-build size and alignment of K and V")
def est(why):
    return ("Estimate", None, why)

VEC = "Vec<u8>"
BIND = {
    # serde_json, std and primitive layouts
    "s(Value)": sz("Value"), "s(String)": sz("String"), "s(Vec)": sz(VEC), "s(&Value)": sz("&'static Value"),
    "s(&str)": sz("&'static str"), "s(usize)": sz("usize"), "s(u64)": sz("u64"), "s(u32)": sz("u32"), "s(f64)": sz("f64"),
    "s(Option<usize>)": sz("Option<usize>"), "s(Option<f64>)": sz("Option<f64>"), "s(Option<i32>)": sz("Option<i32>"),
    "s(Option<[f64;3]>)": sz("Option<[f64; 3]>"), "s([bool;6])": sz("[bool; 6]"),
    "s(Vec<f64>)": sz("Vec<f64>"), "s(Vec<usize>)": sz("Vec<usize>"), "s(Vec<Expansion>)": sz(VEC), "s(Vec<Wide>)": sz(VEC),
    "s(Vec<&ForceTerm>)": sz(VEC),
    "s((&str,&T))": sz("(&'static str, &'static u8)"), "s((&str,&Value))": sz("(&'static str, &'static Value)"),
    "s((&str,usize))": sz("(&'static str, usize)"), "s((&str,usize,u64))": sz("(&'static str, usize, u64)"),
    "s((String,String))": sz("(String, String)"), "s((String,[f64;3]))": sz("(String, [f64; 3])"),
    "s((String,[f64;6]))": sz("(String, [f64; 6])"), "s((String,f64,f64))": sz("(String, f64, f64)"),
    "s((String,usize))": sz("(String, usize)"), "s((f64,f64))": sz("(f64, f64)"), "s((usize,(f64,f64)))": sz("(usize, (f64, f64))"),
    "s((usize,[f64;3],usize,[f64;3]))": sz("(usize, [f64; 3], usize, [f64; 3])"), "s((usize,f64))": sz("(usize, f64)"),
    "s((usize,f64,f64))": sz("(usize, f64, f64)"), "s((usize,usize))": sz("(usize, usize)"), "s((usize,usize,bool))": sz("(usize, usize, bool)"),
    "s((Content,Content))": ("SourceUpper", "64", "serde 1.0.228 private::de::Content: its largest variants (String, ByteBuf, Seq, Map) hold one 24-byte owner, plus the tag, <= 32; a pair <= 64 (G6 witness)"),
    # PP's own types
    "s(ResultItem)": sz("crate::ResultItem"), "s(Diagnostic)": sz("crate::Diagnostic"), "s(ResultBasisRef)": sz("crate::ResultBasisRef"),
    "s(MechanicsEnvelope)": sz("crate::MechanicsEnvelope"), "s(MaterialInput)": sz("crate::MaterialInput"),
    "s(PreviewNode)": sz("crate::PreviewNode"), "s(PreviewPipe)": sz("crate::PreviewPipe"), "s(PreviewSupport)": sz("crate::PreviewSupport"),
    "s(PreviewLoadCase)": sz("crate::PreviewLoadCase"), "s(TemperaturePoint)": sz("crate::MaterialTemperaturePointInput"),
    "s(PrimitiveLoadInput)": sz("crate::PreviewPrimitiveLoad"),
    "s(Authored<Vec<ExpansionLawInput>>)": sz("crate::Authored<Vec<crate::ExpansionLawInput>>"),
    "s((String,DerivedSection))": sz("(String, crate::DerivedSection)"), "s((String,ResultItem))": sz("(String, crate::ResultItem)"),
    "s(StationResultants)": sz("crate::StationResultants"), "s(MemberRecord)": sz("crate::preview_physics::MemberRecord"),
    "s(RecoveryRecord)": sz("crate::formation_guard::RecoveryRecord"), "s(MemberRecovery)": sz("crate::source_recovery::MemberRecovery"),
    "s(SpringAction)": sz("crate::source_recovery::SpringAction"), "s(SupportActions)": sz("crate::source_recovery::SupportActions"),
    "s(FinalizedSourceBlockCase)": sz("crate::source_receipt::FinalizedSourceBlockCase"),
    "s(BasisRecord)": sz("crate::retained_product::BasisRecord"), "s(MemberIdentity)": sz("crate::retained_product::MemberIdentity"),
    "s(TermIdentity)": sz("crate::retained_product::TermIdentity"), "s(SpringIdentity)": sz("crate::retained_product::SpringIdentity"),
    "s(OperationalSpent)": sz("crate::retained_product::OperationalSpent"), "s(OrdinarySeed)": sz("crate::retained_product::OrdinarySeed"),
    "s(SolverObservations)": sz("crate::retained_product::SolverObservations"),
    "s(MaterialSelection)": sz("crate::retained_product::MaterialSelection"), "s(CaptureError)": sz("crate::retained_product::CaptureError"),
    "s(PreparedAttemptView)": sz("crate::retained_receipt::PreparedAttemptView<'static>"), "s(W1Fallback)": sz("crate::W1Fallback"),
    "s(ThreadPacketOutput)": sz("Option<std::thread::Result<Option<Result<crate::RetainedPreviewOutput, String>>>>"),
    # dependency crates the product names
    "s(Validation)": sz("open_pipe_stress_result_export::retained_precision::Validation"),
    "s(RowClassification)": sz("open_pipe_stress_result_export::retained_precision::RowClassification"),
    "s(LinearSupport)": sz("open_pipe_stress_linear_supports::LinearSupport"), "s(SpringEntry)": sz("open_pipe_stress_linear_supports::SpringEntry"),
    "s(SupportFinding)": sz("open_pipe_stress_linear_supports::SupportFinding"),
    "s(PrimitiveLoad)": sz("open_pipe_stress_primitive_loads::PrimitiveLoad"),
    "s(NodalLoadContribution)": sz("open_pipe_stress_primitive_loads::NodalLoadContribution"),
    "s(SymmetricMatrixEntry)": sz("open_pipe_stress_sparse_direct::SymmetricMatrixEntry"),
    "s(StraightPipeElement)": sz("open_pipe_stress_straight_pipe::StraightPipeElement"),
    "s(FrameNode)": sz("open_pipe_stress_frame_kernel::FrameNode"), "s(FrameElement)": sz("open_pipe_stress_frame_kernel::FrameElement"),
    "s((usize,usize,Matrix12))": sz("(usize, usize, open_pipe_stress_frame_kernel::Matrix12)"),
    "s(AnalysisStatus)": sz("open_pipe_stress_stress_recovery::AnalysisStatus"), "s(StressFinding)": sz("open_pipe_stress_stress_recovery::StressFinding"),
    "s((&str,StressRecoveryResult))": sz("(&'static str, open_pipe_stress_stress_recovery::StressRecoveryResult)"),
    "s(QuadraticStressSpan)": sz("open_pipe_stress_stress_recovery::elastic_extrema::QuadraticStressSpan"),
    "s(Node_SR)": ("InBuild", "open_pipe_stress_stress_recovery::elastic_extrema::NODE_STRIDE", "SR export elastic_extrema::NODE_STRIDE"),
    # the kernel, through its resource module
    "s(Wide<4>)": fk("WIDE_4"), "s(Wide<8>)": fk("WIDE_8"), "s(Wide<16>)": fk("WIDE_16"),
    "s(Option<Wide<4>>)": fk("OPTION_WIDE_4"), "s(Option<Wide<8>>)": fk("OPTION_WIDE_8"), "s(Option<Wide<16>>)": fk("OPTION_WIDE_16"),
    "s(MemberOperators<4>)": fk("MEMBER_OPERATORS_4"), "s(MemberOperators<8>)": fk("MEMBER_OPERATORS_8"), "s(MemberOperators<16>)": fk("MEMBER_OPERATORS_16"),
    "s(BoundedCoefficients<4>)": fk("BOUNDED_COEFFICIENTS_4"), "s(BoundedCoefficients<8>)": fk("BOUNDED_COEFFICIENTS_8"),
    "s(BoundedCoefficients<16>)": fk("BOUNDED_COEFFICIENTS_16"),
    "s(BlockBound<4>)": fk("BLOCK_BOUND_4"), "s(BlockBound<8>)": fk("BLOCK_BOUND_8"), "s(BlockBound<16>)": fk("BLOCK_BOUND_16"),
    "s(BlockCertificate<4>)": fk("BLOCK_CERTIFICATE_4"), "s(BlockCertificate<8>)": fk("BLOCK_CERTIFICATE_8"), "s(BlockCertificate<16>)": fk("BLOCK_CERTIFICATE_16"),
    "s(BlockNorms<4>)": fk("BLOCK_NORMS_4"), "s(BlockNorms<8>)": fk("BLOCK_NORMS_8"), "s(BlockNorms<16>)": fk("BLOCK_NORMS_16"),
    "s(BodyReport<4>)": fk("BODY_REPORT_4"), "s(BodyReport<8>)": fk("BODY_REPORT_8"), "s(BodyReport<16>)": fk("BODY_REPORT_16"),
    "s(PivotScreen<4>)": fk("PIVOT_SCREEN_4"), "s(PivotScreen<8>)": fk("PIVOT_SCREEN_8"), "s(PivotScreen<16>)": fk("PIVOT_SCREEN_16"),
    "s(Shared<4>)": ("InBuild", "max_usize(fkr::SHARED_4_4, fkr::SHARED_4_8)", "the two L = 4 instantiations, Shared<4,4> and Shared<4,8>"),
    "s(Shared<8>)": fk("SHARED_8_16"), "s(Shared<16>)": fk("SHARED_16_16"),
    "s(Solved<4>)": fk("SOLVED_4"), "s(Solved<8>)": fk("SOLVED_8"), "s(Solved<16>)": fk("SOLVED_16"),
    "s(VerifyShared<4>)": fk("VERIFY_SHARED_4_8"), "s(VerifyShared<8>)": fk("VERIFY_SHARED_8_16"), "s(VerifyShared<16>)": fk("VERIFY_SHARED_16_16"),
    "s(VerificationReport<4>)": fk("VERIFICATION_REPORT_4"), "s(VerificationReport<8>)": fk("VERIFICATION_REPORT_8"),
    "s(VerificationReport<16>)": fk("VERIFICATION_REPORT_16"),
    "s(ExactWideSum)": fk("EXACT_WIDE_SUM"), "s(Expansion)": fk("EXPANSION"), "s(PrecisionState)": fk("PRECISION_STATE"),
    "s(ArcCasePrep)": ("InBuild", "2 * size_of::<usize>() + fkr::CASE_PREP", "Arc<CasePrep>'s allocation: two counters and the value"),
    "s(ArcGroupPrep)": ("InBuild", "2 * size_of::<usize>() + fkr::GROUP_PREP", "Arc<GroupPrep>'s allocation: two counters and the value"),
    "s(AttemptRecord)": fk("ATTEMPT_RECORD"), "s(BlockRefusal)": fk("BLOCK_REFUSAL"), "s(BlockWitness)": fk("BLOCK_WITNESS"),
    "s(BodyGeometry)": fk("BODY_GEOMETRY"), "s(Binary64Outcome)": fk("BINARY64_OUTCOME"), "s(AffineTerm)": fk("AFFINE_TERM"),
    "s(FunctionalDescriptor)": fk("FUNCTIONAL_DESCRIPTOR"), "s(QualifiedFunctionalProjection)": fk("QUALIFIED_FUNCTIONAL_PROJECTION"),
    "s(RetainedFunctionalProjection)": fk("RETAINED_FUNCTIONAL_PROJECTION"), "s(QualifiedProjection)": fk("QUALIFIED_PROJECTION"),
    "s(RetainedProjection)": fk("RETAINED_PROJECTION"), "s(Ratio)": fk("RATIO"), "s(ForceContribution)": fk("FORCE_CONTRIBUTION"),
    "s(QuantityMeta)": fk("QUANTITY_META"), "s((QuantityId,u64))": fk("QUANTITY_ID_U64"),
    "s((QuantityId,Binary64Outcome))": fk("QUANTITY_ID_OUTCOME"), "s((u32,Kind,u64))": fk("U32_KIND_U64"),
    "s((u32,SpringKind))": fk("U32_SPRING_KIND"), "s(ProductMemberFacts)": fk("PRODUCT_MEMBER_FACTS"),
    "s(ProductRecipe)": fk("PRODUCT_RECIPE"), "s(ProductRowSpec)": fk("PRODUCT_ROW_SPEC"), "s(ProductRowVerdict)": fk("PRODUCT_ROW_VERDICT"),
    "s(PublishedRow)": fk("PUBLISHED_ROW"), "s(RecordedCase)": fk("RECORDED_CASE"), "s(RecordedInvocation)": fk("RECORDED_INVOCATION"),
    "s(RetainedSolve)": fk("RETAINED_SOLVE"),
    "s(SrcCoord)": fk("SOURCE_COORD"), "s(SrcMember)": fk("SOURCE_MEMBER"), "s(SrcSpring)": fk("SOURCE_SPRING"),
    "s(SrcConstraint)": fk("SOURCE_CONSTRAINT"), "s(SrcNodal)": fk("SOURCE_NODAL"), "s(SrcStation)": fk("SOURCE_STATION"),
    "s(SrcSupport)": fk("SOURCE_SUPPORT"),
    "s(StiffnessContribution)": fk("STIFFNESS_CONTRIBUTION"), "s(LoadFidelityRow)": fk("LOAD_FIDELITY_ROW"),
    "s(PivotEvidence)": fk("PIVOT_EVIDENCE"), "s(ResidualRow)": fk("RESIDUAL_ROW"), "s(ContributionRounding)": fk("CONTRIBUTION_ROUNDING"),
    "s(PublishedValue)": fk("PUBLISHED_VALUE"), "s(RecordOutcome)": fk("RECORD_OUTCOME"), "s(ForceTerm)": fk("FORCE_TERM"),
    "s(ExactAccumulator)": fk("EXACT_ACCUMULATOR"), "s((usize,ExactAccumulator,bool))": fk("INDEXED_ACCUMULATOR"),
    "s((usize,LedgerNet))": fk("INDEXED_LEDGER_NET"), "s((usize,PublishedValue))": fk("INDEXED_PUBLISHED_VALUE"),
    "s([PublishedValue;12])": fk("PUBLISHED_VALUES_12"), "s((EWS,EWS,f64))": fk("EXACT_WIDE_SUM_PAIR"),
    "s((Wide,Wide,Wide))": fk("WIDE_TRIPLE_16"), "s((bool,f64,Wide))": fk("FLAGGED_WIDE_16"),
    "s(Option<BoundRefusal>)": fk("OPTION_BOUND_REFUSAL"),
    # BTreeMap nodes at in-build key/value layouts
    "Node(String,Value)": node("String", "Value"), "Node(String,ResultItem)": node("String", "crate::ResultItem"),
    "Node(usize,ResultItem)": node("usize", "crate::ResultItem"), "Node(&str,&ResultItem)": node("&'static str", "&'static crate::ResultItem"),
    "Node(&str,())": node("&'static str", "()"), "Node(String,())": node("String", "()"), "Node(usize,())": node("usize", "()"),
    "Node(&String,())": node("&'static String", "()"), "Node(usize,String)": node("usize", "String"),
    "Node((String,String),())": node("(String, String)", "()"),
    "Node(String,BTreeMap)": node("String", "std::collections::BTreeMap<String, String>"),
}
# G6 (I65 u4_g6_01 QUALIFICATION.md §2): the 42 design-record atoms, each bound to the actual owner its
# roster line denotes, read at the G5 basis. "InBuild" sums are sums of in-build sizes; "SourceUpper"
# terms are field-sum upper bounds over a private type's (nameable) field types. Where a roster line's
# owner is already priced elsewhere (the C2 maps are ProductCapture's identity vectors, T11.1/T11.3),
# the atom is bound to that owner again: a conservative double count, stated.
def ins(expr, why):
    return ("InBuild", expr, why)
def upr(expr, why):
    return ("SourceUpper", expr, why)
RP = "crate::retained_product::"
RR = "crate::retained_receipt::"
BIND.update({
    # T12.1 C2 maps -> ProductCapture identity vectors (retained_product.rs:81-150), double-counted with T11
    "s(NodeMapE)": ins("size_of::<(String, [f64; 3])>()", "C2 node map = ProductCapture.nodes (String, [f64; 3]); also priced in T11.1"),
    "s(MemberMapE)": ins(f"size_of::<{RP}MemberIdentity>()", "C2 member map = ProductCapture.members (MemberIdentity); also in T11.3"),
    "s(SpringMapE)": ins(f"size_of::<{RP}SpringIdentity>()", "C2 spring map = ProductCapture.spring_map (SpringIdentity); also in T11.3"),
    "s(SupportMapE)": ins("size_of::<(String, usize)>() + size_of::<[bool; 6]>()", "C2 support map = ProductCapture.supports (String, usize) and support_fixed [bool; 6]; also in T11.3"),
    "s(ConstraintMapE)": ins("max_usize(fkr::SOURCE_CONSTRAINT, size_of::<(String, usize)>())", "C2 constraint map: the kernel Constraint or a support identity entry, the larger"),
    "s(NodalMapE)": ins(f"size_of::<{RP}TermIdentity>()", "C2 nodal-load map = ProductCapture.terms (TermIdentity); also in T11.3"),
    "s(StationMapE)": ins("fkr::SOURCE_STATION", "C2 station map: the kernel Station (SourceParts.stations)"),
    "s(SectionMapE)": ins("fkr::PRODUCT_MEMBER_FACTS", "C2 section map = ProductCapture.facts (ProductMemberFacts); also in T11.3"),
    "s(MaterialDescriptor)": ins(f"max_usize(size_of::<{RP}MaterialSelection>(), size_of::<(String, f64, f64)>())", "ProductCapture.selections / materials, the larger"),
    # T12.5-T12.10 preparation
    "s((usize,VecPair))": ins("size_of::<(usize, Vec<(f64, f64)>)>()", "CasePrep.prescribed: Vec<(usize, Vec<(f64, f64)>)> (adaptive.rs:926)"),
    "s(Pair)": ins("size_of::<(f64, f64)>()", "the prescribed pairs (f64, f64) (adaptive.rs:926)"),
    "s(BodyMapE)": upr("2 * size_of::<Vec<u32>>() + size_of::<u32>()", "C2 body map entry (two u32 child lists and an id): no distinct owner at this basis (source.body_of_node is in PrimitiveSource; body_nodes() temporaries are covered by C_u32(n,b)); the entry upper is kept"),
    "s(Row6)": ins("size_of::<[f64; 6]>()", "rigid_body.rs: Vec<[f64; 6]> rows and motions"),
    "s(Expansion6)": ins("fkr::EXPANSION_6", "rigid_body.rs:210: [Expansion; 6] per node"),
    "s(Tag)": ins("fkr::TAGGED_CONTRIBUTION", "assemble.rs: tagged Vec<(usize, Contribution)>"),
    "s(ContributionE)": ins("fkr::CONTRIBUTION", "assemble.rs: Structure.items Vec<Contribution>"),
    "s(RegistryE)": upr("fkr::CALL_ORIGIN + fkr::SOURCE_ORIGIN + fkr::GROUP_ORIGIN + fkr::BUILD_ORIGIN + fkr::RUN_ORIGINS + 3 * size_of::<usize>() + fkr::SLOT_SNAPSHOT",
                        "OriginStore's six registries (origins.rs:290-297), each <= 8 entries for one case: one entry of each, SelectedOrigin {Arc, usize, usize, SlotSnapshot} by field sum"),
    # T13.2 stop rule (adaptive.rs:681-900)
    "s(LazyE)": ins("fkr::LAZY_ROW", "BoundedExtremeTracker.lazy: (ExactWideSum, ExactWideSum, u64, u64)"),
})
EVAL_A = "max_usize(8, fkr::ATTEMPT_STOP_ALIGN)"
EVAL_UP = f"(up(8, {EVAL_A}) + up(fkr::ATTEMPT_STOP, {EVAL_A}) + up(1, {EVAL_A}))"
TABLE_UP = f"(up(8, {EVAL_A}) + {EVAL_UP})"
KEY_A = "max_usize(4, fkr::KIND_ALIGN)"
KEY_UP = f"(up(1, {KEY_A}) + up(4, {KEY_A}) + up(fkr::KIND, {KEY_A}))"
BIND.update({
    "s(TableE)": upr(TABLE_UP, "BoundedExtremeTracker.table: (u64, Evaluated); Evaluated (private) <= tag + seq u64 + AttemptStop by field sum"),
    "s(TrackerE)": upr(f"fkr::LAZY_ROW + 3 * {TABLE_UP}", "one BoundedExtremeTracker over F offers, per offer: a lazy row, a table and a kept-table entry and the stable-sort scratch entry (P3 ENVELOPE Tracker(O) law; the pivot, residual and fallback trackers)"),
    "Node(TrackerKey,Tracker)": upr(f"btree_node_upper({KEY_UP}, {KEY_A}, fkr::TRACKER, fkr::TRACKER_ALIGN)", "TrackerSet.trackers: BTreeMap<(RuleTest, u32, Kind), BoundedExtremeTracker>; RuleTest (private, fieldless) <= 1 byte"),
    "Node(TrackerKey,())": upr(f"btree_node_upper({KEY_UP}, {KEY_A}, 0, 1)", "TrackerSet.holding: BTreeSet<(RuleTest, u32, Kind)>"),
    # T14 lanes, projection/maximum, final certificate (product_certificate/*.rs)
    "s(LawE)": upr("size_of::<&'static open_pipe_stress_frame_kernel::structural::retained_api::StraightMember>() + 3 * size_of::<f64>() + 9 * size_of::<f64>() + size_of::<u64>()",
                   "bridge::ProposedMemberLaw {&StraightMember, D, t, MaterialOperands (<= 9 f64 + tag), z} by field sum"),
    "s(EnclosureE)": upr("2 * fkr::WIDE_16", "Enclosure {lo, hi: Wide<16>} (product_certificate.rs:16): LaneReadouts.rows"),
    "s(ProjectionOutcome)": ins("fkr::INDEXED_OUTCOME", "ProductCertificateSpent.projection_outcomes: Vec<(usize, Binary64Outcome)>"),
    "s(ConversionE)": ins("fkr::INDEXED_OUTCOME", "the projection conversion record (usize, Binary64Outcome): no second owner; a double count"),
    "s(MaximumE)": upr("size_of::<(usize, u32)>() + fkr::PRODUCT_MAXIMUM_VALUE + 2 * up(8, max_usize(8, align_of::<serde_json::Number>())) + up(4, max_usize(8, align_of::<serde_json::Number>())) + up(8 * size_of::<serde_json::Number>(), max_usize(8, align_of::<serde_json::Number>())) + size_of::<bool>()",
                       "per member: the kernel maxima slot (usize, u32), PP's ProductMaximumValue output, PP's PreparedMaximumPatch {usize, usize, u32, [Number; 8]} (field sum) and the completion flag"),
    "s(AliasE)": ins("size_of::<crate::LocatedQuantity>()", "the two alias LocatedQuantity values (their strings are B(P_final*RID))"),
    "s(CoverageFact)": ins("size_of::<[f64; 3]>()", "coverage_facts' coordinates Vec<[f64; 3]> (<= n)"),
    "s(RunRow)": ins("size_of::<bool>()", "row_scales' native_coverage Vec<bool> (Q)"),
    "s(SupportCoverage)": ins("6 * size_of::<bool>()", "row_scales' support_coverage Vec<bool> (6g)"),
    "s(IntervalE)": upr("2 * fkr::WIDE_16", "run_case's k: Vec<Enclosure> (Q); the second Q term is slack"),
    "s(BodyCoverage)": ins("2 * fkr::PRODUCT_SUMMARY_COVERAGE + size_of::<[f64; 4]>()", "per body: the kernel's and PP's ProductSummaryCoverage and row_scales' scales [f64; 4]"),
    "s(ProductValue)": ins("size_of::<f64>() + size_of::<bool>()", "per final row: the values f64 and (21m <= P_final) derivative_coverage bools"),
    "s(ProductRow)": ins("fkr::PRODUCT_FINAL_ROW + fkr::PRODUCT_ROW_VERDICT", "per final row: the descriptor ProductFinalRow and PP's verdict copy"),
    "s(ArcPrepared)": ins("2 * size_of::<usize>() + fkr::PRODUCT_OWNER_STAMP", "Arc<ProofAnchor{owner: ProductOwnerStamp}>: two counters and the value"),
    # T15 C3 typed trace (retained_receipt.rs, retained_product.rs:3279-3300)
    "s(PreparedMemberEvent)": ins(f"size_of::<{RR}PreparationEntry>() + size_of::<{RP}PreparedAssociation>() + fkr::PREPARED_ANNULUS + fkr::SECTION_PREPARATION_WORK",
                                  "per member: trace.members PreparationEntry, associations, preparations PreparedAnnulus, preparation_work SectionPreparationWork"),
    "s(ConversionEvent)": ins("fkr::OPTION_PREPARATION_CONVERSION", "the 9 inline conversion outcomes per member (inside SectionPreparationWork): a double count"),
    "s(LaneTerminal)": ins("fkr::PRODUCT_CERTIFICATE_SPENT", "the lanes' terminal work (Option<ResidualWork> x2 inline in ProductCertificateSpent): the whole record, per lane"),
    "s(FinalRowConversion)": ins("fkr::INDEXED_OUTCOME", "the final-row conversion record (usize, Binary64Outcome): a double count of projection_outcomes"),
    "s(AdapterSnapshot)": ins(f"size_of::<{RR}PrivateAdapterSnapshot>()", "PreparedTrace.adapter: PrivateAdapterSnapshot"),
    # Ordinary recovery
    "s(SupportVector)": ins("size_of::<(String, [f64; 6])>()", "CaseRecord.support_vectors: Vec<(String, [f64; 6])> (preview_physics.rs:170)"),
})
# G5 part 2: field-sum upper bounds for the product's private record types (the field types are
# nameable here; the struct is not). A repr(Rust) struct is no larger than its fields laid out in
# declaration order, each padded to the struct's alignment A = max(field alignments): any layout
# rustc picks is at most that (it reorders only to remove padding). G6 witnesses each against the
# actual size_of where the type is named (source_receipt, outside this fence).
def align_expr(types):
    e = f"align_of::<{types[0]}>()"
    for t in types[1:]:
        e = f"max_usize({e}, align_of::<{t}>())"
    return e
def field_upper(types):
    a = align_expr(types)
    return " + ".join(f"up(size_of::<{t}>(), {a})" for t in types)
ROW_TREATMENT = ["String", "&'static str", "Option<String>", "Option<&'static str>", "Vec<String>"]
PROJECTION = ["String", "String", "String", "&'static str", "f64", "String", "&'static str", "[f64; 2]", "f64", "f64", "f64",
              "&'static str"]
DERIVED = ["crate::ResultItem", "&'static str", "Vec<String>"]
BIND["s(RowTreatment)"] = ("SourceUpper", field_upper(ROW_TREATMENT),
    "source_receipt.rs:597 RowTreatment {result_id: String, treatment: &str, projection_id: Option<String>, recipe_id: Option<&str>, input_result_ids: Vec<String>}: field-sum upper")
BIND["s(Projection)"] = ("SourceUpper", field_upper(PROJECTION),
    "source_receipt.rs:582 Projection {4 String, 3 &str, 4 f64, [f64; 2]}: field-sum upper")
BIND["s(Derived)"] = ("SourceUpper", field_upper(DERIVED),
    "source_receipt/rows.rs:324 Derived {row: ResultItem, recipe: &str, inputs: Vec<String>}: field-sum upper")
BIND["Node(String,RowTreatment)"] = ("SourceUpper",
    f"btree_node_upper(size_of::<String>(), align_of::<String>(), {field_upper(ROW_TREATMENT)}, {align_expr(ROW_TREATMENT)})",
    "BUILD.md §4 node at K = String and V = RowTreatment's field-sum upper (monotone in the value size)")
FORMATION_A = "max_usize(fkr::FORMATION_ALIGN, align_of::<f64>())"
BIND["s(Option<FormationRecord>)"] = ("SourceUpper",
    f"up(fkr::FORMATION, {FORMATION_A}) + up(size_of::<f64>(), {FORMATION_A}) + up(size_of::<bool>(), {FORMATION_A}) + up(size_of::<bool>(), {FORMATION_A})",
    "FK load_ledger.rs:101 FormationRecord {formation: Formation, operand_bound: f64, self_equilibrated: bool} (private): field-sum upper at the exported Formation layout, plus one aligned slot for the Option tag")
BIND["s(RowBinding)"] = ("InBuild", "fkr::PRODUCT_FINAL_ROW",
    "bind_rows' element (retained_product.rs:1742): the kernel's ProductFinalRow, exported by structural::retained_resource")

forms = tree["forms"]; exprs = tree["exprs"]; phases = tree["phases"]
text = tree["text_atoms"]; assumed = tree["assumed"]
atoms = sorted({a for f in forms.values() for a in f if a != "1"})
missing = [a for a in atoms if a not in BIND and not a.startswith("Text(")]
if missing:
    raise SystemExit("unbound atoms: " + ", ".join(missing))
idx = {a: i for i, a in enumerate(atoms)}

def rust_ident(name):
    return re.sub(r"[^A-Za-z0-9]+", "_", name).strip("_").upper()

L = []
w = L.append
w("// ---- BEGIN GENERATED PROFILE (part2/_run_records/g5_profile.py from profile_tree.json; do not edit by hand) ----")
w("/// U4 G5 part 2: the cap-priced admission maximum as named in-build expressions. Every term is a")
w("/// linear form over layout atoms at the D1 caps (l <= 128); every maximum (stages, phases, moving")
w("/// candidates) is taken here, in the build. Source: the G4 chain with RV84/RV87's corrections at")
w(f"/// {tree['basis']}.")
w("pub(super) mod profile {")
w("    #![allow(clippy::all, dead_code)]")
w("    use open_pipe_stress_frame_kernel::structural::retained_resource as fkr;")
w("    use serde_json::Value;")
w("    use std::mem::{align_of, size_of};")
w("")
w("    #[derive(Debug, Clone, Copy, PartialEq, Eq)]")
w("    pub(crate) enum Binding {")
w("        /// size_of/align_of of the actual type in this build.")
w("        InBuild,")
w("        /// A source-derived upper bound for a type the product cannot name (cited).")
w("        SourceUpper,")
w("        /// A text byte count from the T08 closure (layout-free).")
w("        Text,")
w("        /// A G3 design-record stride with no in-build type identified: the profile is incomplete.")
w("        Estimate,")
w("    }")
w("    const fn up(x: usize, a: usize) -> usize {")
w("        (x + a - 1) / a * a")
w("    }")
w("    const fn max_usize(a: usize, b: usize) -> usize {")
w("        if a > b { a } else { b }")
w("    }")
w("    /// BUILD.md §4: a BTreeMap<K, V> node, max(Leaf_up, Internal_up), at A = max(8, align K, align V).")
w("    pub(crate) const fn btree_node_upper(sk: usize, ak: usize, sv: usize, av: usize) -> usize {")
w("        let a = max_usize(8, max_usize(ak, av));")
w("        let leaf = up(8, a) + up(2, a) + up(2, a) + up(11 * sk, a) + up(11 * sv, a);")
w("        let internal = up(up(leaf, 8) + 12 * 8, a);")
w("        max_usize(leaf, internal)")
w("    }")
w("    /// The text atoms of the T08 closure at this basis (byte counts, layout-free), and the")
w("    /// longest-string atoms of the hash route (RV84 C-N1; RV87 N-3).")
for k_, v_ in sorted(text.items()):
    w(f"    pub(crate) const TEXT_{rust_ident(k_)}: u64 = {int(v_)}; // {k_}")
for k_, v_ in sorted(tree["L"].items()):
    w(f"    pub(crate) const {k_}: u64 = {int(v_)};")
w(f"    pub(crate) const ATOMS: usize = {len(atoms)};")
w("    /// The atoms, in index order: their names (as the records write them) and bindings.")
w("    pub(crate) const ATOM_NAMES: [&str; ATOMS] = [")
for a in atoms:
    w(f"        {json.dumps(a)},")
w("    ];")
w("    pub(crate) const ATOM_BINDINGS: [Binding; ATOMS] = [")
for a in atoms:
    kind = "Text" if a.startswith("Text(") else BIND[a][0]
    w(f"        Binding::{kind},")
w("    ];")
w("    /// The in-build value of each atom (Estimate atoms carry their design-record stride).")
w("    pub(crate) const ATOM_VALUES: [u64; ATOMS] = [")
for a in atoms:
    if a.startswith("Text("):
        w(f"        {int(text[a])}, // {a}: text closure")
    else:
        kind, expr, why = BIND[a]
        if kind == "Estimate":
            w(f"        {int(assumed[a])}, // {a}: ESTIMATE ({why})")
        else:
            note = f" ({why})" if why else ""
            w(f"        ({expr}) as u64, // {a}{note}")
w("    ];")
w("    /// G3/G4's illustrative values, for the transcription check against the Python chain.")
w("    #[cfg(test)]")
w("    pub(crate) const ATOM_ASSUMED: [u64; ATOMS] = [")
for a in atoms:
    v = text[a] if a.startswith("Text(") else assumed[a]
    w(f"        {int(v)},")
w("    ];")
w("    /// One term: a constant plus coefficient x atom pairs.")
w("    pub(crate) struct Form {")
w("        pub(crate) name: &'static str,")
w("        pub(crate) constant: u64,")
w("        pub(crate) terms: &'static [(usize, u64)],")
w("    }")
form_names = sorted(forms)
fidx = {n: i for i, n in enumerate(form_names)}
w(f"    pub(crate) const FORMS: [Form; {len(form_names)}] = [")
for n in form_names:
    f = forms[n]
    terms = ", ".join(f"({idx[a]}, {c})" for a, c in sorted(f.items(), key=lambda kv: idx.get(kv[0], -1)) if a != "1")
    w(f"        Form {{ name: {json.dumps(n)}, constant: {int(f.get('1', 0))}, terms: &[{terms}] }},")
w("    ];")
for n in form_names:
    w(f"    pub(crate) const F_{rust_ident(n)}: usize = {fidx[n]};")
w("    /// A form's value at the given atom values, in checked arithmetic.")
w("    pub(crate) const fn form(f: &Form, v: &[u64; ATOMS]) -> Option<u64> {")
w("        let mut total = f.constant;")
w("        let mut i = 0;")
w("        while i < f.terms.len() {")
w("            let (a, c) = f.terms[i];")
w("            let Some(x) = v[a].checked_mul(c) else { return None };")
w("            let Some(t) = total.checked_add(x) else { return None };")
w("            total = t;")
w("            i += 1;")
w("        }")
w("        Some(total)")
w("    }")
w("    pub(crate) const fn add(a: Option<u64>, b: Option<u64>) -> Option<u64> {")
w("        match (a, b) {")
w("            (Some(a), Some(b)) => a.checked_add(b),")
w("            _ => None,")
w("        }")
w("    }")
w("    pub(crate) const fn max(a: Option<u64>, b: Option<u64>) -> Option<u64> {")
w("        match (a, b) {")
w("            (Some(a), Some(b)) => Some(if a > b { a } else { b }),")
w("            _ => None,")
w("        }")
w("    }")
def node_code(n):
    if isinstance(n, str):
        if n in exprs:
            return f"{n.lower()}(v)"
        return f"form(&FORMS[{fidx[n]}], v)"
    key = "sum" if "sum" in n else "max"
    parts = [node_code(c) for c in n[key]]
    op = "add" if key == "sum" else "max"
    acc = parts[0]
    for p in parts[1:]:
        acc = f"{op}({acc}, {p})"
    return acc
for name, n in exprs.items():
    w(f"    /// {name} ({'sum' if 'sum' in n else 'maximum'} of its terms, taken in the build).")
    w(f"    pub(crate) const fn {name.lower()}(v: &[u64; ATOMS]) -> Option<u64> {{")
    w(f"        {node_code(n)}")
    w("    }")
modes = ("sparse", "dense")
phase_names = list(phases["sparse"].keys())
w(f"    pub(crate) const PHASES: usize = {len(phase_names)};")
w("    pub(crate) const PHASE_NAMES: [&str; PHASES] = [")
for p in phase_names:
    w(f"        {json.dumps(p)},")
w("    ];")
for mode in modes:
    w(f"    /// The phases for {mode} mode: requested bytes, and the largest moving extra (E_mov, without R).")
    w(f"    pub(crate) const fn phases_{mode}(v: &[u64; ATOMS]) -> [(Option<u64>, Option<u64>); PHASES] {{")
    w("        [")
    for p in phase_names:
        ph = phases[mode][p]
        req = node_code({"sum": ph["requested"]})
        mov = node_code({"max": ph["moving"]})
        w(f"            ({req}, {mov}), // {p.split(' ')[0]}")
    w("        ]")
    w("    }")
w("    /// The admission maximum without R: max over the phases of requested + moving.")
w("    pub(crate) const fn maximum(phases: &[(Option<u64>, Option<u64>); PHASES]) -> Option<(u64, usize)> {")
w("        let mut best: Option<(u64, usize)> = None;")
w("        let mut i = 0;")
w("        while i < PHASES {")
w("            let Some(e) = add(phases[i].0, phases[i].1) else { return None };")
w("            best = match best {")
w("                Some((b, j)) if b >= e => Some((b, j)),")
w("                _ => Some((e, i)),")
w("            };")
w("            i += 1;")
w("        }")
w("        best")
w("    }")
w("    /// The count of Estimate atoms (the profile is complete only when it is zero).")
w("    pub(crate) const ESTIMATES: usize = {")
w("        let mut n = 0;")
w("        let mut i = 0;")
w("        while i < ATOMS {")
w("            if matches!(ATOM_BINDINGS[i], Binding::Estimate) {")
w("                n += 1;")
w("            }")
w("            i += 1;")
w("        }")
w("        n")
w("    };")
w("    pub(crate) const SPARSE: Option<(u64, usize)> = maximum(&phases_sparse(&ATOM_VALUES));")
w("    pub(crate) const DENSE: Option<(u64, usize)> = maximum(&phases_dense(&ATOM_VALUES));")
w("    /// The Python chain's own evaluation (ASSUMED strides), for the transcription check.")
w("    #[cfg(test)]")
w(f"    pub(crate) const PYTHON_CHECK: [(u64, &str); 2] = [({tree['check']['sparse']['max'] - tree['R']}, {json.dumps(tree['check']['sparse']['phase'])}), ({tree['check']['dense']['max'] - tree['R']}, {json.dumps(tree['check']['dense']['phase'])})];")
w("}")
w("// ---- END GENERATED PROFILE ----")
block = "\n".join(L) + "\n"
src = open(target).read()
begin = src.find("// ---- BEGIN GENERATED PROFILE")
end = src.find("// ---- END GENERATED PROFILE ----")
if begin < 0 or end < 0:
    raise SystemExit("markers not found in " + target)
end = src.index("\n", end) + 1
open(target, "w").write(src[:begin] + block + src[end:])
print(json.dumps({"atoms": len(atoms), "forms": len(form_names), "estimates": sum(1 for a in atoms if not a.startswith("Text(") and BIND[a][0] == "Estimate"),
                  "source_upper": sum(1 for a in atoms if not a.startswith("Text(") and BIND[a][0] == "SourceUpper"),
                  "text": sum(1 for a in atoms if a.startswith("Text(")), "lines": len(L)}))
