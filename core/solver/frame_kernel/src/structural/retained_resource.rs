//! U4 G5 part 2 (BUILD.md §3, T01): the kernel's in-build strides for the product's
//! retained-memory profile. Each constant is `size_of`/`align_of` of a kernel type,
//! taken here where the type is nameable, and exported as a plain number; the
//! product crate names none of these types. Nothing here allocates, runs or changes
//! a kernel behaviour: these are compile-time constants.
use std::mem::{align_of, size_of};

use super::exact_boundary::functionals::{
    AffineTerm, FunctionalDescriptor, QualifiedFunctionalProjection, RetainedFunctionalProjection,
};
use super::exact_boundary::{BlockWitness, ForceContribution, QualifiedProjection, Ratio, RetainedProjection};
use super::retained::adaptive::{AttemptStop, BoundedExtremeTracker, CasePrep, GroupPrep, PrecisionState, Shared, Solved};
use super::retained::assemble::{BoundedCoefficients, Contribution, MemberOperators};
use super::retained::origins::ProductOwnerStamp;
use super::retained::bound::{BlockBound, BlockCertificate};
use super::retained::factor::PivotScreen;
use super::retained::ledger::LedgerNet;
use super::retained::verify::{BlockNorms, BodyReport, VerificationReport, VerifyShared};
use super::retained::wide::Wide;
use super::retained::wide_sum::ExactWideSum;
use super::retained_api::{
    AttemptRecord, Binary64Outcome, BlockRefusal, BodyGeometry, BoundRefusal, BuildOrigin, CallOrigin, Constraint,
    GroupOrigin, Kind, NodalLoad, PreparationConversion, PreparedAnnulus, ProductCertificateSpent, ProductMaximumValue,
    ProductSummaryCoverage, RunOrigins, SectionPreparationWork, SlotSnapshot, SourceOrigin,
    ProductFinalRow, ProductMemberFacts, ProductRecipe, ProductRowSpec, ProductRowVerdict, PublishedRow,
    QuantityId, QuantityMeta, RecordedCase, RecordedInvocation, RetainedSolve, Spring, SpringKind,
    Station, StraightMember, SupportGroup,
};
use super::{
    ContributionRounding, Expansion, LoadFidelityRow, PivotEvidence, PublishedValue, RecordOutcome,
    ResidualRow, StiffnessContribution,
};
use crate::exact_sum::ExactAccumulator;
use crate::load_ledger::{ForceTerm, Formation};

/// The widths and (L, R) / (L, W) pairs the retained schedule instantiates
/// (adaptive.rs `PRECISIONS`: 128/4/4, 256/4/8, 512/8/16, 1024/16/16).
macro_rules! sizes {
    ($($name:ident = $t:ty;)*) => { $(pub const $name: usize = size_of::<$t>();)* };
}
sizes! {
    WIDE_4 = Wide<4>; WIDE_8 = Wide<8>; WIDE_16 = Wide<16>;
    OPTION_WIDE_4 = Option<Wide<4>>; OPTION_WIDE_8 = Option<Wide<8>>; OPTION_WIDE_16 = Option<Wide<16>>;
    MEMBER_OPERATORS_4 = MemberOperators<4>; MEMBER_OPERATORS_8 = MemberOperators<8>; MEMBER_OPERATORS_16 = MemberOperators<16>;
    BOUNDED_COEFFICIENTS_4 = BoundedCoefficients<4>; BOUNDED_COEFFICIENTS_8 = BoundedCoefficients<8>; BOUNDED_COEFFICIENTS_16 = BoundedCoefficients<16>;
    BLOCK_BOUND_4 = BlockBound<4>; BLOCK_BOUND_8 = BlockBound<8>; BLOCK_BOUND_16 = BlockBound<16>;
    BLOCK_CERTIFICATE_4 = BlockCertificate<4>; BLOCK_CERTIFICATE_8 = BlockCertificate<8>; BLOCK_CERTIFICATE_16 = BlockCertificate<16>;
    BLOCK_NORMS_4 = BlockNorms<4>; BLOCK_NORMS_8 = BlockNorms<8>; BLOCK_NORMS_16 = BlockNorms<16>;
    BODY_REPORT_4 = BodyReport<4>; BODY_REPORT_8 = BodyReport<8>; BODY_REPORT_16 = BodyReport<16>;
    PIVOT_SCREEN_4 = PivotScreen<4>; PIVOT_SCREEN_8 = PivotScreen<8>; PIVOT_SCREEN_16 = PivotScreen<16>;
    SHARED_4_4 = Shared<4, 4>; SHARED_4_8 = Shared<4, 8>; SHARED_8_16 = Shared<8, 16>; SHARED_16_16 = Shared<16, 16>;
    SOLVED_4 = Solved<4>; SOLVED_8 = Solved<8>; SOLVED_16 = Solved<16>;
    VERIFY_SHARED_4_8 = VerifyShared<4, 8>; VERIFY_SHARED_8_16 = VerifyShared<8, 16>; VERIFY_SHARED_16_16 = VerifyShared<16, 16>;
    VERIFICATION_REPORT_4 = VerificationReport<4>; VERIFICATION_REPORT_8 = VerificationReport<8>; VERIFICATION_REPORT_16 = VerificationReport<16>;
    EXACT_WIDE_SUM = ExactWideSum; EXPANSION = Expansion; PRECISION_STATE = PrecisionState; LEDGER_NET = LedgerNet;
    CASE_PREP = CasePrep; GROUP_PREP = GroupPrep;
    ATTEMPT_RECORD = AttemptRecord; BLOCK_REFUSAL = BlockRefusal; BLOCK_WITNESS = BlockWitness; BODY_GEOMETRY = BodyGeometry;
    BINARY64_OUTCOME = Binary64Outcome; AFFINE_TERM = AffineTerm; FUNCTIONAL_DESCRIPTOR = FunctionalDescriptor;
    QUALIFIED_FUNCTIONAL_PROJECTION = QualifiedFunctionalProjection<'static, 'static, 'static, 'static>;
    RETAINED_FUNCTIONAL_PROJECTION = RetainedFunctionalProjection;
    QUALIFIED_PROJECTION = QualifiedProjection<'static, 'static>; RETAINED_PROJECTION = RetainedProjection;
    RATIO = Ratio; FORCE_CONTRIBUTION = ForceContribution; QUANTITY_META = QuantityMeta;
    QUANTITY_ID_U64 = (QuantityId, u64); QUANTITY_ID_OUTCOME = (QuantityId, Binary64Outcome);
    U32_KIND_U64 = (u32, Kind, u64); U32_SPRING_KIND = (u32, SpringKind);
    PRODUCT_MEMBER_FACTS = ProductMemberFacts; PRODUCT_RECIPE = ProductRecipe; PRODUCT_ROW_SPEC = ProductRowSpec<'static>;
    PRODUCT_ROW_VERDICT = ProductRowVerdict; PUBLISHED_ROW = PublishedRow; RECORDED_CASE = RecordedCase;
    RECORDED_INVOCATION = RecordedInvocation; RETAINED_SOLVE = RetainedSolve;
    SOURCE_COORD = [f64; 3]; SOURCE_MEMBER = StraightMember; SOURCE_SPRING = Spring; SOURCE_CONSTRAINT = Constraint;
    SOURCE_NODAL = NodalLoad; SOURCE_STATION = Station; SOURCE_SUPPORT = SupportGroup;
    STIFFNESS_CONTRIBUTION = StiffnessContribution; LOAD_FIDELITY_ROW = LoadFidelityRow; PIVOT_EVIDENCE = PivotEvidence;
    RESIDUAL_ROW = ResidualRow; CONTRIBUTION_ROUNDING = ContributionRounding; PUBLISHED_VALUE = PublishedValue;
    RECORD_OUTCOME = RecordOutcome; FORCE_TERM = ForceTerm; EXACT_ACCUMULATOR = ExactAccumulator;
    INDEXED_ACCUMULATOR = (usize, ExactAccumulator, bool); INDEXED_LEDGER_NET = (usize, LedgerNet);
    INDEXED_PUBLISHED_VALUE = (usize, PublishedValue); PUBLISHED_VALUES_12 = [PublishedValue; 12];
    EXACT_WIDE_SUM_PAIR = (ExactWideSum, ExactWideSum, f64); WIDE_TRIPLE_16 = (Wide<16>, Wide<16>, Wide<16>);
    FLAGGED_WIDE_16 = (bool, f64, Wide<16>);
    OPTION_BOUND_REFUSAL = Option<BoundRefusal>; PRODUCT_FINAL_ROW = ProductFinalRow<'static>;
    FORMATION = Formation;
    // G6: the Estimate atoms' kernel types (I65 u4_g6_01 QUALIFICATION.md §2).
    LAZY_ROW = (ExactWideSum, ExactWideSum, u64, u64); TRACKER = BoundedExtremeTracker; ATTEMPT_STOP = AttemptStop;
    KIND = Kind; EXPANSION_6 = [Expansion; 6]; CONTRIBUTION = Contribution; TAGGED_CONTRIBUTION = (usize, Contribution);
    PRODUCT_OWNER_STAMP = ProductOwnerStamp; CALL_ORIGIN = CallOrigin; SOURCE_ORIGIN = SourceOrigin;
    GROUP_ORIGIN = GroupOrigin; BUILD_ORIGIN = BuildOrigin; RUN_ORIGINS = RunOrigins; SLOT_SNAPSHOT = SlotSnapshot;
    SECTION_PREPARATION_WORK = SectionPreparationWork; PREPARED_ANNULUS = PreparedAnnulus;
    OPTION_PREPARATION_CONVERSION = Option<PreparationConversion>; PRODUCT_SUMMARY_COVERAGE = ProductSummaryCoverage;
    PRODUCT_MAXIMUM_VALUE = ProductMaximumValue; PRODUCT_CERTIFICATE_SPENT = ProductCertificateSpent<'static>;
    INDEXED_OUTCOME = (usize, Binary64Outcome);
}
/// The alignment of `load_ledger::Formation`, for the product's field-sum upper bound on the
/// ledger's private `Option<FormationRecord>` slot (formation, an f64 and a bool).
pub const FORMATION_ALIGN: usize = align_of::<Formation>();
/// G6: alignments for the product's field-sum upper bounds over private kernel records.
pub const TRACKER_ALIGN: usize = align_of::<BoundedExtremeTracker>();
pub const ATTEMPT_STOP_ALIGN: usize = align_of::<AttemptStop>();
pub const KIND_ALIGN: usize = align_of::<Kind>();
/// The largest alignment among the exported kernel types (a witness for the
/// hashbrown premise align(K) <= 16, BUILD.md §4).
pub const MAX_ALIGN: usize = {
    let a = [align_of::<Wide<16>>(), align_of::<ExactWideSum>(), align_of::<ExactAccumulator>(), align_of::<CasePrep>()];
    let mut m = 0;
    let mut i = 0;
    while i < a.len() {
        if a[i] > m {
            m = a[i];
        }
        i += 1;
    }
    m
};
