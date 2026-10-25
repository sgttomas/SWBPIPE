//! Fixed ordinary product recipes. Endpoints and radius authority stay here.
use super::super::{
    adaptive,
    origins::RecordedInvocation,
    recover::{End, Kind, QuantityId},
    source::{Component, Dof},
};
use super::source_residual::ResidualWork;
use super::*;
#[path = "loaded_maximum.rs"]
mod loaded_maximum;
/// The loaded-maximum search's capacity hook (W1 coverage repair §1.4, option 2),
/// compiled only under the opt-in `loaded-maximum-hooks` feature.
#[cfg(feature = "loaded-maximum-hooks")]
pub use loaded_maximum::hooks as loaded_maximum_hooks;
/// The W1 coverage census probe and switches (`w1-coverage-repair.md` §3.2),
/// compiled only under the opt-in `census-probe` feature.
#[cfg(feature = "census-probe")]
#[path = "census_probe.rs"]
pub mod census_probe;
/// The corner-1 repair's switch (W1 coverage repair §1.2). On in every build
/// without `census-probe`: the production search drives an interval represented
/// Z at z_lo. Under `census-probe` it stays the probe's switch, so the census can
/// still compare today's search with corner 1's.
fn corner1() -> bool {
    #[cfg(feature = "census-probe")]
    { census_probe::switches().corner1 }
    #[cfg(not(feature = "census-probe"))]
    { true }
}

#[derive(Clone, Copy)]
enum Scalar64 {
    Add,
    Sub,
    Mul,
    Div,
    Sqrt,
}
#[derive(Debug, Clone, Copy)]
pub enum ProductMaterial {
    Base {
        e: f64,
        g: f64,
    },
    Point {
        ordinal: usize,
        e: f64,
        g: f64,
    },
    Interpolated {
        lower: usize,
        upper: usize,
        t_lo: f64,
        t: f64,
        t_hi: f64,
        e_lo: f64,
        e_hi: f64,
        g_lo: f64,
        g_hi: f64,
        e_hat: f64,
        g_hat: f64,
    },
    /// B3-K K3-1: the base common E and Poisson's ratio of the exact route. The
    /// source lane encloses G = E/(2(1+nu)) from the exact E and nu (DEF-E); the
    /// admitted K keeps the represented shear modulus as given.
    BaseENu {
        e: f64,
        nu: f64,
    },
}
impl ProductMaterial {
    fn operands(self) -> MaterialOperands {
        match self {
            Self::Base { e, g } | Self::Point { e, g, .. } => MaterialOperands::Ordinary { e, g },
            Self::BaseENu { e, nu } => MaterialOperands::ExactENu { e, nu },
            Self::Interpolated {
                t_lo,
                t,
                t_hi,
                e_lo,
                e_hi,
                g_lo,
                g_hi,
                e_hat,
                g_hat,
                ..
            } => MaterialOperands::Interpolated {
                t_lo,
                t,
                t_hi,
                e_lo,
                e_hi,
                g_lo,
                g_hi,
                e_hat,
                g_hat,
            },
        }
    }
}
#[derive(Debug, Clone)]
pub struct ProductMemberFacts {
    pub member: u32,
    pub diameter: f64,
    pub effective_wall: f64,
    pub material: ProductMaterial,
    pub area: f64,
    pub second_moment: f64,
    pub torsion_constant: f64,
    pub section_modulus: f64,
    pub radius: f64,
}
/// O4 (DEL-04-08 Design `o4-source-extension.md` §4.5): one member's reduced
/// stress section, present only where its reduced section differs from the
/// nominal one in any of A, Z, I, J or the effective wall. The nominal
/// `ProductMemberFacts` do not grow; this sidecar carries the reduced facts
/// and the final-row indices of the member's reduced stress and maximum rows.
#[derive(Debug, Clone, PartialEq)]
pub struct ReducedSection {
    pub member: u32,
    /// t_r = (t − c_m) − c_a in binary64: the annular lane's reduced annulus
    /// is (D, t_r).
    pub effective_wall: f64,
    /// `prepare_product_annulus(D, t_r)`'s binary64 A_r, Î_r, Ĵ_r and Ẑ_r.
    pub area: f64,
    pub second_moment: f64,
    pub torsion_constant: f64,
    pub section_modulus: f64,
    /// The final-row indices of this member's reduced stress and maximum
    /// rows (the row-index map the kernel reads a row's basis from).
    pub rows: Vec<u32>,
}
/// O4 §4.5: the reduced sections of an invocation's distinct-basis members,
/// sorted by member. Empty for every document whose sections are bit-equal,
/// which then takes today's path, coverage, capacities and bits.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ReducedSectionSet {
    sections: Vec<ReducedSection>,
}
static EMPTY_REDUCED: ReducedSectionSet = ReducedSectionSet { sections: Vec::new() };
impl ReducedSectionSet {
    /// Validates the entries (kernel cause "reduced section"): members
    /// sorted and unique; every value finite and positive; each entry has at
    /// least one row; no final-row index appears twice in the set.
    pub fn new(mut sections: Vec<ReducedSection>) -> Result<Self, ProductFailure> {
        sections.sort_by_key(|s| s.member);
        for pair in sections.windows(2) {
            if pair[0].member == pair[1].member {
                return Err(bad("reduced section member"));
            }
        }
        let mut seen: Vec<u32> = Vec::new();
        for s in &sections {
            for v in [s.effective_wall, s.area, s.second_moment, s.torsion_constant, s.section_modulus] {
                if !(v.is_finite() && v > 0.0) {
                    return Err(bad("reduced section value"));
                }
            }
            if s.rows.is_empty() {
                return Err(bad("reduced section rows"));
            }
            seen.extend_from_slice(&s.rows);
        }
        seen.sort_unstable();
        if seen.windows(2).any(|p| p[0] == p[1]) {
            return Err(bad("reduced row index"));
        }
        Ok(Self { sections })
    }
    pub fn empty() -> &'static Self {
        &EMPTY_REDUCED
    }
    pub fn is_empty(&self) -> bool {
        self.sections.is_empty()
    }
    pub fn sections(&self) -> &[ReducedSection] {
        &self.sections
    }
}
// O4 §4.5: ProductRecipe carries no basis tag; its receipt-priced layout is
// pinned at 16 bytes.
const _: () = assert!(std::mem::size_of::<ProductRecipe>() == PRODUCT_RECIPE_BYTES);
pub(crate) const PRODUCT_RECIPE_BYTES: usize = 16;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProductSite {
    End(End),
    Station(u32),
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProductStress {
    Axial,
    BendingY,
    BendingZ,
    Torsion,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProductRecipe {
    Native(QuantityId),
    /// The identified ordinary rigid/scalar-spring support law, including empty components.
    SupportComponent { support: u32, component: Component },
    Stress {
        member: u32,
        site: ProductSite,
        stress: ProductStress,
    },
    CircularMaximum {
        member: u32,
    },
    NonQuantity,
    /// Selected-material presence record; PP owns its exact source/text binding.
    ModulusBasisRecord,
    /// T4-U4b (DEL-04-07 Design u4b-u5b-implementation.md §4.1): a recipe-free
    /// `not_covered` passthrough row at final-row position `row`. It enters no
    /// lane, functional, certificate, scale, bound or coverage set; PP supplies
    /// its value after projection (`ProductValuesBuilder::complete_not_covered`)
    /// from the successor's own published rows, and its verdict takes the
    /// class-`None` path.
    NotCovered { row: u32 },
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProductUnit {
    Millimetre,
    Radian,
    Newton,
    NewtonMetre,
    Megapascal,
    Pascal,
    Record,
}
impl ProductUnit {
    fn normalize(self, y: f64) -> f64 {
        match self {
            Self::Millimetre => y / 1000.0,
            Self::Megapascal => y * 1_000_000.0,
            _ => y,
        }
    }
}
#[derive(Debug)]
pub struct ProductFinalRow<'a> {
    pub id: &'a str,
    pub case_id: &'a str,
    pub value: &'a f64,
    pub unit: ProductUnit,
    pub body: u32,
    pub recipe: ProductRecipe,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProductPredicate {
    Absolute,
    SharperExact,
    SharperBinary64,
    DecimalSi,
    DecimalRaw,
    InputDerived,
}
#[derive(Debug, Clone)]
pub struct ProductRowVerdict {
    pub row: usize,
    pub normalized_bits: u64,
    pub scale_bits: u64,
    pub class: Option<adaptive::RowClass>,
    pub passed: bool,
    pub failed: Option<ProductPredicate>,
    /// Relative order: exact, binary64, SI decimal, raw decimal. Absolute/input use slot zero.
    pub predicates: [Option<bool>; 4],
}
#[derive(Debug, Clone)]
enum Cause {
    Association(&'static str),
    Numeric(NumericError),
    Native(bridge::BridgeError),
    Accounting(WorkFault),
    CountRange(&'static str),
    Storage,
    G5a(&'static str),
    Predicate { row:usize, predicate:ProductPredicate },
    Helper(directed::certificate::HelperError),
}
#[derive(Debug, Clone)]
pub struct ProductFailure {
    cause: Cause,
}
#[derive(Debug)]
pub enum HelperFailure<'a> {Arithmetic(&'a AttemptStop),InvalidSmallBoundInput,Binary64Range,Invariant}
#[derive(Debug)]
pub enum ProductFailureView<'a> {
    Association(&'static str),Numeric(&'a NumericError),Native(source_residual::BridgeFailure<'a>),
    Accounting(WorkFault),CountRange(&'static str),Storage,G5a(&'static str),
    Predicate{row:usize,predicate:ProductPredicate},Helper(HelperFailure<'a>),
}
impl ProductFailure {
    pub fn typed_cause(&self,copies:&mut TraceCopyWork)->ProductFailureView<'_> {
        copies.record::<ProductFailureView<'_>>();
        match &self.cause {
            Cause::Association(s)=>ProductFailureView::Association(s),Cause::Numeric(e)=>ProductFailureView::Numeric(e),
            Cause::Native(e)=>ProductFailureView::Native(source_residual::bridge_failure(e)),Cause::Accounting(f)=>ProductFailureView::Accounting(*f),
            Cause::CountRange(s)=>ProductFailureView::CountRange(s),Cause::Storage=>ProductFailureView::Storage,Cause::G5a(s)=>ProductFailureView::G5a(s),
            Cause::Predicate{row,predicate}=>ProductFailureView::Predicate{row:*row,predicate:*predicate},
            Cause::Helper(e)=>ProductFailureView::Helper(match e {
                directed::certificate::HelperError::Arithmetic(e)=>HelperFailure::Arithmetic(e),
                directed::certificate::HelperError::InvalidSmallBoundInput=>HelperFailure::InvalidSmallBoundInput,
                directed::certificate::HelperError::Binary64Range=>HelperFailure::Binary64Range,
                directed::certificate::HelperError::Invariant=>HelperFailure::Invariant}),
        }
    }
    pub fn category(&self) -> &'static str {
        match self.cause {
            Cause::Association(_) => "association",
            Cause::Numeric(_) => "numeric",
            Cause::Native(_) => "native_source",
            Cause::Accounting(_) => "work_accounting",
            Cause::CountRange(_) => "count_range",
            Cause::Storage => "storage",
            Cause::G5a(_) => "g5a",
            Cause::Predicate { .. } => "numeric_predicate",
            Cause::Helper(_) => "numeric_helper",
        }
    }
}
impl From<NumericError> for ProductFailure {
    fn from(e: NumericError) -> Self {
        Self {
            cause: Cause::Numeric(e),
        }
    }
}
fn bad(s: &'static str) -> ProductFailure {
    ProductFailure {
        cause: Cause::Association(s),
    }
}
#[derive(Debug)]
pub struct LaneTrace<'a> {pub law:source_residual::ReadoutLaw,pub result:Result<(),source_residual::BridgeFailure<'a>>,pub work:source_residual::LaneWorkTrace<'a>}
#[derive(Debug)]
pub struct ProductProofTrace<'a> {
    pub lanes:[Option<LaneTrace<'a>>;2],pub numeric:NumericTrace,pub comparisons_lme:WorkTotal,
    pub visits:WorkTotal,pub scalar_operations:WorkTotal,pub projection_conversions:WorkTotal,
    /// Actual SI midpoint conversion for MPa Stress rows; raw conversion for every other unit.
    pub projection_outcomes:&'a [(usize,super::super::wide::multi::Binary64Outcome)],
    pub capacities:[usize;7],pub prepared_capacity_bytes:[usize;6],pub completion_merged:bool,
    pub trace_copy_work:&'a TraceCopyWork,
    /// I61/I57 §3: borrowed view of the proof-owned `coverage` (assigned atomically
    /// at check_intervals). Empty means no complete vector was retained. Never the
    /// adapter's fallible copy; no coverage is computed here.
    pub summary_coverage:&'a [ProductSummaryCoverage],
}
/// I57 §1 compact per-body payload: the four native stop outcomes and has_data.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CoverageBody { pub body:u32, pub stop:[bool;4], pub has_data:bool }
impl CoverageBody {
    fn of(entry:&ProductSummaryCoverage)->Self { Self{body:entry.body,stop:entry.stop,has_data:entry.has_data} }
}
/// I57 §2 public facts for one body: per-kind presence in the bound layout, the
/// recomputed native extent L, the selected native verification E (force/moment),
/// the selected native p512 floor if any, and native p. No private value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CoverageFacts {
    pub present:[bool;4], pub extent_bits:u64, pub resolution_bits:[u64;2],
    pub floor_bits:Option<[u64;2]>, pub precision:u32,
}
/// I57 §2: rederive estimate and charge from the compact payload and public facts.
/// p512 charge is the force/moment stop pair; p128/p256 charge is estimate. The
/// fixed 1024 proof/projection precision is not an input, so it can neither
/// change p nor create a floor. Necessary public consistency is also required:
/// a floor exists iff p == 512, an absent kind has no stop, and a positive floor
/// forces its present force/moment stop.
pub fn rederive_coverage(payload:CoverageBody,facts:&CoverageFacts)->Result<ProductSummaryCoverage,ProductFailure> {
    match (facts.precision,facts.floor_bits) {
        (128|256,None)|(512,Some(_))=>{},
        _=>return Err(bad("coverage precision/floor")),
    }
    if (0..4).any(|k|payload.stop[k] && !facts.present[k]) {return Err(bad("coverage absent-kind stop"));}
    if let Some(floor)=facts.floor_bits {
        for k in 0..2 {
            if facts.present[2+k] && f64::from_bits(floor[k])>0.0 && !payload.stop[2+k] {return Err(bad("coverage floor stop"));}
        }
    }
    let mut hats=[f64::from_bits(facts.resolution_bits[0])>0.0,f64::from_bits(facts.resolution_bits[1])>0.0];
    if f64::from_bits(facts.extent_bits)!=0.0 {hats=[hats[0]||hats[1];2];}
    let estimate=[facts.present[2]&&hats[0],facts.present[3]&&hats[1]];
    let charge=if facts.precision==512 {[payload.stop[2],payload.stop[3]]} else {estimate};
    Ok(ProductSummaryCoverage{body:payload.body,stop:payload.stop,estimate,charge,has_data:payload.has_data})
}
/// Public facts from the owner's bound source layout/maps and selected native Run.
/// L is recomputed by adaptive::body_extent over the body's nodes in ascending
/// (native) order and must equal the bound preparation extent bit for bit.
fn coverage_facts(owner:&adaptive::RetainedSolve,body:u32,copies:&mut TraceCopyWork)->Result<CoverageFacts,ProductFailure> {
    copies.record::<CoverageFacts>();
    let source=owner.source();
    let mut present=[false;4];
    for meta in &owner.prep.layout {
        if meta.body!=body {continue;}
        let kind=match meta.kind {Kind::Translation=>0,Kind::Rotation=>1,Kind::Force=>2,Kind::Moment=>3};
        // recover::layout marks only constrained displacement/rotation rows input-derived.
        if kind>=2 && meta.input_derived {return Err(bad("coverage input-derived force/moment layout"));}
        present[kind]=true;
    }
    let count=(0..source.nodes().len()).filter(|&n|source.body_of_node(n as u32)==body).count();
    let mut coordinates=reserve::<[f64;3]>(count)?;
    for (n,p) in source.nodes().iter().enumerate() {
        if source.body_of_node(n as u32)!=body {continue;}
        copies.record::<[f64;3]>();coordinates.push(*p);
    }
    let extent=adaptive::body_extent(&coordinates);
    let bound=owner.prep.extents.get(body as usize).ok_or_else(||bad("coverage extent"))?;
    if extent.to_bits()!=bound.to_bits() {return Err(bad("coverage extent"));}
    let e=owner.evidence().resolution_scale.iter().find(|e|e.0==body).ok_or_else(||bad("coverage resolution"))?;
    let floor_bits=match &owner.evidence().floor {
        None=>None,
        Some(f)=>Some(f.iter().find(|f|f.0==body).map(|f|[f.1,f.2]).ok_or_else(||bad("coverage floor"))?),
    };
    Ok(CoverageFacts{present,extent_bits:extent.to_bits(),resolution_bits:[e.1,e.2],floor_bits,precision:owner.selected_precision()})
}
impl<'a> ProductProofTrace<'a> {
    /// I61: encode each proof-owned entry to its compact payload, rederive all nine
    /// flags from public facts, and compare bit for bit. A partial vector or any
    /// mismatch is an association failure; the vector is never altered. Callers
    /// map an empty vector to null before calling this.
    pub fn check_summary_coverage(&self,owner:&adaptive::RetainedSolve,copies:&mut TraceCopyWork)->Result<(),ProductFailure> {
        if self.summary_coverage.len()!=owner.source().body_count() as usize {return Err(bad("coverage body count"));}
        for (i,entry) in self.summary_coverage.iter().enumerate() {
            copies.record::<CoverageBody>();
            let payload=CoverageBody::of(entry);
            if payload.body as usize!=i {return Err(bad("coverage body order"));}
            let facts=coverage_facts(owner,payload.body,copies)?;
            copies.record::<ProductSummaryCoverage>();
            if rederive_coverage(payload,&facts)?!=*entry {return Err(bad("coverage flag mismatch"));}
        }
        copies.status().fault().map_or(Ok(()),|f|Err(ProductFailure{cause:Cause::Accounting(f)}))
    }
}
#[derive(Debug)]
pub struct LoadedMaximumDiagnostic {
    pub row:usize,
    pub member:u32,
    pub lower:(bool,i64,[u64;16]),
    pub upper:(bool,i64,[u64;16]),
}
#[derive(Debug)]
pub struct ProductCertificateSpent<'a> {
    rows: &'a [ProductFinalRow<'a>],
    verdicts: Vec<ProductRowVerdict>,
    failure: Option<ProductFailure>,
    coverage: Vec<ProductSummaryCoverage>,
    native: Option<ResidualWork>,
    native_k: Option<ResidualWork>,
    numeric: NumericWork,
    comparisons: SumWork,
    visits: WorkTotal,
    scalar_operations: WorkTotal,
    /// laws, native coverage, represented intervals, verdicts, derivatives, support slots, scales.
    pub capacities: [usize; 7],
    /// Prepared values, descriptor rows, maximum slots, completion flags, anchor layout, conversion records (bytes).
    pub prepared_capacities: [usize; 6],
    projection_outcomes:Vec<(usize,super::super::wide::multi::Binary64Outcome)>,
    projection_conversions:WorkTotal,
    lane_errors:[Option<bridge::BridgeError>;2],completion_merged:bool,
    trace_copy_work:TraceCopyWork,
    /// RV77-N4 (I61 U2, failure path): the proof's anchor once it exists, so the
    /// work of a refused proof stays structurally bound to its selected owner.
    anchor:Option<std::sync::Arc<ProofAnchor>>,
    loaded_diagnostic:Option<Box<LoadedMaximumDiagnostic>>,
    loaded_search_bytes:usize,
    /// O4 §4.5: the reduced-row index map's and the reduced-coverage vector's
    /// capacities (elements), zero when the `ReducedSectionSet` is empty. The
    /// serialized `capacities` keep their seven slots.
    reduced_capacities:[usize;2],
}
impl<'a> ProductCertificateSpent<'a> {
    fn new(rows: &'a [ProductFinalRow<'a>]) -> Self {
        Self {
            rows,
            verdicts: Vec::new(),
            failure: None,
            coverage: Vec::new(),
            native: None,
            native_k: None,
            numeric: NumericWork::new(),
            comparisons: SumWork::default(),
            visits: WorkTotal::zero(),
            scalar_operations: WorkTotal::zero(),
            capacities: [0; 7],
            prepared_capacities: [0; 6],
            projection_outcomes:Vec::new(),projection_conversions:WorkTotal::zero(),
            lane_errors:[None,None],completion_merged:false,trace_copy_work:TraceCopyWork::default(),
            anchor:None,loaded_diagnostic:None,loaded_search_bytes:0,reduced_capacities:[0;2],
        }
    }
    /// O4 §4.5: [reduced-row index map, reduced-coverage vector] capacities
    /// (elements); [0, 0] for an empty `ReducedSectionSet`.
    pub fn reduced_capacities(&self)->[usize;2] { self.reduced_capacities }
    /// RV77-N4 (I61 U2): whether this work belongs to a proof started on `owner`'s
    /// own prepared solve. False before the anchor exists (a refused proof start).
    pub fn loaded_maximum_diagnostic(&self)->Option<&LoadedMaximumDiagnostic> { self.loaded_diagnostic.as_deref() }
    pub fn loaded_maximum_workspace_bytes(&self)->usize { self.loaded_search_bytes }
    pub fn owner_matches(&self,owner:&adaptive::RetainedSolve)->bool {
        self.anchor.as_ref().is_some_and(|anchor|anchor.matches_owner(owner))
    }
    pub fn typed_trace<'b>(&'b self,copies:&mut TraceCopyWork)->ProductProofTrace<'b> {
        copies.record::<ProductProofTrace<'_>>();
        let lane=|work:&'b ResidualWork,index:usize,copies:&mut TraceCopyWork| LaneTrace {
            law:work.readout_law,result:match &self.lane_errors[index] {None=>Ok(()),Some(e)=>Err(source_residual::bridge_failure(e))},
            work:work.typed_trace(copies)};
        ProductProofTrace {lanes:[self.native_k.as_ref().map(|w|lane(w,0,copies)),self.native.as_ref().map(|w|lane(w,1,copies))],
            numeric:self.numeric.trace(copies),comparisons_lme:self.comparisons.checked_lme(),visits:self.visits,
            scalar_operations:self.scalar_operations,projection_conversions:self.projection_conversions,
            projection_outcomes:&self.projection_outcomes,capacities:self.capacities,prepared_capacity_bytes:self.prepared_capacities,
            completion_merged:self.completion_merged,trace_copy_work:&self.trace_copy_work,
            summary_coverage:&self.coverage}
    }
    pub fn status(&self) -> WorkStatus {
        let mut s = self
            .numeric
            .status()
            .join(self.comparisons.checked_lme().status())
            .join(self.visits.status())
            .join(self.scalar_operations.status()).join(self.projection_conversions.status());
        if let Some(w) = &self.native { s = s.join(w.status()); }
        if let Some(w) = &self.native_k { s = s.join(w.status()); }
        s
    }
    pub fn summary_coverage(&self) -> &[ProductSummaryCoverage] {
        &self.coverage
    }
    pub fn verdicts(&self) -> &[ProductRowVerdict] {
        &self.verdicts
    }
    pub fn failure(&self) -> Option<&ProductFailure> {
        self.failure.as_ref()
    }
    pub fn passed(&self) -> bool {
        self.status().is_exact()
            && self.failure.is_none()
            && self.verdicts.len() == self.rows.len()
            && self.verdicts.iter().all(|v| v.passed)
    }
    pub fn source_correction_calls(&self) -> Option<WorkTotal> {
        match (&self.native_k,&self.native) {
            (None,None)=>None,(Some(k),None)=>Some(k.correction.calls),(None,Some(s))=>Some(s.correction.calls),
            (Some(k),Some(s))=>Some(k.correction.calls.add(s.correction.calls)),
        }
    }
    pub fn native_work(&self) -> Option<&impl std::fmt::Debug> {
        self.native.as_ref()
    }
    pub fn prepared_lane_work(&self) -> impl std::fmt::Debug + '_ { (&self.native_k,&self.native) }
    pub fn work_summary(&self) -> impl std::fmt::Debug + '_ {
        (
            &self.numeric,
            &self.comparisons,
            self.visits,
            self.scalar_operations,
            self.prepared_capacities,
            (&self.projection_outcomes,self.projection_conversions),
        )
    }
    fn rebind<'b>(self, rows:&'b [ProductFinalRow<'b>]) -> ProductCertificateSpent<'b> {
        ProductCertificateSpent { rows, verdicts:self.verdicts, failure:self.failure,
            coverage:self.coverage, native:self.native, native_k:self.native_k,
            numeric:self.numeric, comparisons:self.comparisons, visits:self.visits,
            scalar_operations:self.scalar_operations, capacities:self.capacities, prepared_capacities:self.prepared_capacities,
            projection_outcomes:self.projection_outcomes,projection_conversions:self.projection_conversions,
            lane_errors:self.lane_errors,completion_merged:self.completion_merged,trace_copy_work:self.trace_copy_work,anchor:self.anchor,
            loaded_diagnostic:self.loaded_diagnostic,loaded_search_bytes:self.loaded_search_bytes,
            reduced_capacities:self.reduced_capacities }
    }
    fn retain_lane(&mut self,result:Result<source_residual::LaneReadouts,bridge::BridgeError>,work:ResidualWork)
        ->Result<source_residual::LaneReadouts,ProductFailure> {
        let index=match work.readout_law {source_residual::ReadoutLaw::AdmittedK=>0,source_residual::ReadoutLaw::AnnularSource=>1};
        self.lane_errors[index]=result.as_ref().err().cloned();self.trace_copy_work.record::<Option<bridge::BridgeError>>();
        if index==0{self.native_k=Some(work);}else{self.native=Some(work);}
        result.map_err(|e|ProductFailure{cause:Cause::Native(e)})
    }
    fn prepared_reserve<T>(&mut self, slot:usize, n:usize)->Result<Vec<T>,ProductFailure> {
        self.visit()?;
        let v=reserve::<T>(n)?;
        self.prepared_capacities[slot]=v.capacity().checked_mul(std::mem::size_of::<T>())
            .ok_or(ProductFailure{cause:Cause::CountRange("prepared capacity bytes")})?;
        Ok(v)
    }
    fn visit(&mut self) -> Result<(), ProductFailure> {
        self.visits = self.visits.add(WorkTotal::exact_count(1));
        self.status().fault().map_or(Ok(()), |f| {
            Err(ProductFailure {
                cause: Cause::Accounting(f),
            })
        })
    }
    fn f64_op(&mut self, operation: Scalar64, a: f64, b: f64) -> Result<f64, ProductFailure> {
        if let Some(f) = self.status().fault() {
            return Err(ProductFailure {
                cause: Cause::Accounting(f),
            });
        }
        let scalar = self.scalar_operations.add(WorkTotal::exact_count(1));
        let visits = self.visits.add(WorkTotal::exact_count(1));
        let status = scalar.status().join(visits.status());
        if let Some(f) = status.fault() {
            self.numeric.status = self.numeric.status.join(status);
            return Err(ProductFailure {
                cause: Cause::Accounting(f),
            });
        }
        self.scalar_operations = scalar;
        self.visits = visits;
        Ok(match operation {
            Scalar64::Add => a + b,
            Scalar64::Sub => a - b,
            Scalar64::Mul => a * b,
            Scalar64::Div => a / b,
            Scalar64::Sqrt => a.sqrt(),
        })
    }
    fn normalize(&mut self, u: ProductUnit, y: f64) -> Result<f64, ProductFailure> {
        match u {
            ProductUnit::Millimetre => self.f64_op(Scalar64::Div, y, 1000.0),
            ProductUnit::Megapascal => self.f64_op(Scalar64::Mul, y, 1_000_000.0),
            _ => Ok(y),
        }
    }
    fn absolute(&mut self, s: f64) -> Result<f64, ProductFailure> {
        let divisor = 18_446_744_073_709_551_616.0;
        let nearest = self.f64_op(Scalar64::Div, s, divisor)?;
        let back = self.f64_op(Scalar64::Mul, nearest, divisor)?;
        Ok(if back < s {
            adaptive::next_up(nearest)
        } else {
            nearest
        })
    }
    fn collect_small(
        &mut self,
        e: &directed::certificate::EntrySpent<f64>,
    ) -> Result<f64, ProductFailure> {
        let original = e.result().copied();
        self.numeric.wide.merge(&e.work());
        self.numeric.sums.merge(&e.sum_work());
        self.numeric.status = self.numeric.status.join(e.status());
        self.scalar_operations = self
            .scalar_operations
            .add(WorkTotal::exact_count(u64::from(e.f64_operations())));
        self.visits = self
            .visits
            .add(WorkTotal::exact_count(1 + u64::from(e.comparisons())));
        match original {
            Err(e) => Err(ProductFailure {
                cause: Cause::Helper(e),
            }),
            Ok(v) => self.status().fault().map_or(Ok(v), |f| {
                Err(ProductFailure {
                    cause: Cause::Accounting(f),
                })
            }),
        }
    }
    fn small_bound(&mut self, value: f64, scale: f64) -> Result<f64, ProductFailure> {
        if let Some(f) = self.status().fault() {
            return Err(ProductFailure {
                cause: Cause::Accounting(f),
            });
        }
        let e = directed::certificate::small_row_bound(value, scale);
        self.collect_small(&e)
    }
}
fn reserve<T>(n: usize) -> Result<Vec<T>, ProductFailure> {
    std::alloc::Layout::array::<T>(n).map_err(|_| ProductFailure {
        cause: Cause::CountRange("array layout"),
    })?;
    let mut v = Vec::new();
    v.try_reserve_exact(n).map_err(|_| ProductFailure {
        cause: Cause::Storage,
    })?;
    Ok(v)
}
fn hull(a: Enclosure, b: Enclosure) -> Enclosure {
    Enclosure {
        lo: min(a.lo, b.lo),
        hi: max(a.hi, b.hi),
    }
}
fn negate(a: Enclosure) -> Enclosure {
    Enclosure {
        lo: a.hi.neg(),
        hi: a.lo.neg(),
    }
}
fn corners(
    w: &mut NumericWork,
    a: Enclosure,
    b: Enclosure,
    entry: Entry,
) -> Result<Enclosure, NumericError> {
    let mut lo = None;
    let mut hi = None;
    for x in [&a.lo, &a.hi] {
        for y in [&b.lo, &b.hi] {
            let l = w.scalar(entry, x, y, Toward::Down)?;
            let h = w.scalar(entry, x, y, Toward::Up)?;
            lo = Some(lo.map_or(l, |v| min(v, l)));
            hi = Some(hi.map_or(h, |v| max(v, h)));
        }
    }
    Ok(Enclosure {
        lo: lo.unwrap(),
        hi: hi.unwrap(),
    })
}
fn abs_range(a: Enclosure) -> Enclosure {
    let lo = if a.lo.cmp_value(&Endpoint::ZERO) != Ordering::Greater
        && a.hi.cmp_value(&Endpoint::ZERO) != Ordering::Less
    {
        Endpoint::ZERO
    } else {
        min(a.lo.abs(), a.hi.abs())
    };
    Enclosure {
        lo,
        hi: max(a.lo.abs(), a.hi.abs()),
    }
}
fn sqrt(w: &mut NumericWork, a: Endpoint, t: Toward) -> Result<Endpoint, NumericError> {
    w.begin(Entry::Sqrt)?;
    let spent = directed::certificate::sqrt_endpoint(&a, t);
    w.wide.merge(&spent.work());
    w.sums.merge(&spent.sum_work());
    w.status = w.status.join(spent.status());
    let r = spent.result().copied().map_err(|e| match e {
        directed::certificate::HelperError::Arithmetic(e) => NumericError::Arithmetic(e),
        _ => NumericError::Arithmetic(AttemptStop::WorkAccounting(WorkFault::Inconsistent)),
    });
    w.checked(r)
}
fn norm2(w: &mut NumericWork, a: Enclosure, b: Enclosure) -> Result<Enclosure, NumericError> {
    let (a, b) = (abs_range(a), abs_range(b));
    let aa = corners(w, a, a, Entry::Mul)?;
    let bb = corners(w, b, b, Entry::Mul)?;
    let sum = w.add(aa, bb)?;
    Ok(Enclosure {
        lo: sqrt(w, sum.lo, Toward::Down)?,
        hi: sqrt(w, sum.hi, Toward::Up)?,
    })
}
fn native_index(owner: &adaptive::RetainedSolve, id: QuantityId) -> Result<usize, ProductFailure> {
    owner
        .publish()
        .rows
        .iter()
        .position(|r| r.id == id)
        .ok_or_else(|| bad("native row missing"))
}
/// Resolve only the existing zero-or-one attributed law. Native API rows remain
/// independent: this helper does not grant them a support-component coverage slot.
fn support_component(
    owner: &adaptive::RetainedSolve,
    support: u32,
    component: Component,
    spent: &mut ProductCertificateSpent<'_>,
) -> Result<(usize, Option<usize>), ProductFailure> {
    let source = owner.source();
    let mut found = None;
    for (i, group) in source.supports().iter().enumerate() {
        spent.visit()?;
        if group.id == support {
            found = Some(i);
            break;
        }
    }
    let i = found.ok_or_else(|| bad("support group"))?;
    let group = &source.supports()[i];
    if !group.directional_springs.is_empty() {
        return Err(bad("directional support law"));
    }
    let mut contributor = group.restrained[component.index()].then_some(QuantityId::Reaction(
        super::super::source::Dof {
            node: group.node,
            component,
        },
    ));
    for id in &group.springs {
        spent.visit()?;
        let mut spring = None;
        for s in source.springs() {
            spent.visit()?;
            if s.id == *id {
                spring = Some(s);
                break;
            }
        }
        let spring = spring.ok_or_else(|| bad("support spring"))?;
        if spring.dof.node != group.node {
            return Err(bad("support spring node"));
        }
        if spring.dof.component == component {
            if contributor.is_some() {
                return Err(bad("multiple support contributors"));
            }
            contributor = Some(QuantityId::SpringAction {
                spring: *id,
                component,
            });
        }
    }
    let mut index = None;
    if let Some(id) = contributor {
        for (j, row) in owner.publish().rows.iter().enumerate() {
            spent.visit()?;
            if row.id == id {
                index = Some(j);
                break;
            }
        }
        if index.is_none() {
            return Err(bad("support contributor missing"));
        }
    }
    Ok((i, index))
}
fn action_id(member: u32, site: ProductSite, c: Component) -> QuantityId {
    match site {
        ProductSite::End(end) => QuantityId::EndAction {
            member,
            end,
            component: c,
        },
        ProductSite::Station(station) => QuantityId::StationAction {
            station,
            component: c,
        },
    }
}
fn action(
    owner: &adaptive::RetainedSolve,
    values: &[Enclosure],
    member: u32,
    site: ProductSite,
    c: Component,
) -> Result<Enclosure, ProductFailure> {
    let a = values[native_index(owner, action_id(member, site, c))?];
    Ok(if site == ProductSite::End(End::I) {
        negate(a)
    } else {
        a
    })
}
/// The nominal-section recipe (tests): `recipe_reduced` without a reduced row.
#[cfg(test)]
#[allow(clippy::too_many_arguments)]
fn recipe(
    owner: &adaptive::RetainedSolve,
    spent: &mut ProductCertificateSpent<'_>,
    values: &[Enclosure],
    r: ProductRecipe,
    section: Option<&MemberEnclosures>,
    facts: &[ProductMemberFacts],
    represented: bool,
    curvatures: &[(u32,[Enclosure;2])],
    row:usize,
) -> Result<Enclosure, ProductFailure> {
    recipe_reduced(owner, spent, values, r, section, facts, None, represented, curvatures, row)
}
#[allow(clippy::too_many_arguments)]
fn recipe_reduced(
    owner: &adaptive::RetainedSolve,
    spent: &mut ProductCertificateSpent<'_>,
    values: &[Enclosure],
    r: ProductRecipe,
    section: Option<&MemberEnclosures>,
    facts: &[ProductMemberFacts],
    reduced: Option<&ReducedSection>,
    represented: bool,
    curvatures: &[(u32,[Enclosure;2])],
    row:usize,
) -> Result<Enclosure, ProductFailure> {
    if let ProductRecipe::Native(id) = r {
        return Ok(values[native_index(owner, id)?]);
    }
    if let ProductRecipe::SupportComponent { support, component } = r {
        let (_, contributor) = support_component(owner, support, component, spent)?;
        return Ok(contributor.map_or(Enclosure::point(Endpoint::ZERO), |i| values[i]));
    }
    let w = &mut spent.numeric;
    let member = match r {
        ProductRecipe::Stress { member, .. } | ProductRecipe::CircularMaximum { member } => member,
        _ => return Err(bad("nonquantity recipe")),
    };
    let i = owner
        .source()
        .member_index(member)
        .ok_or_else(|| bad("member"))?;
    let (sec, f) = (section.ok_or_else(|| bad("section recipe"))?, &facts[i]);
    // O4 §4.5: a reduced row reads its member's reduced facts (A_r, J_r; Z
    // from the reduced section's represented hull(Ẑ_r, Î_r/c)); r_o = D/2.
    let (area, torsion) = match reduced {
        Some(rs) => (rs.area, rs.torsion_constant),
        None => (f.area, f.torsion_constant),
    };
    // W1 coverage repair census (§3.2, switches (b) and (c)): the annular
    // lane's stress recipe takes the admitted lane's section. Off without
    // `census-probe`.
    #[cfg(feature = "census-probe")]
    let shared = census_probe::switches().shares_section();
    #[cfg(not(feature = "census-probe"))]
    let shared = false;
    let (a, z, j, c) = if represented || shared {
        (
            Enclosure::point(lift(area)?),
            sec.represented_z.ok_or_else(|| bad("represented Z"))?,
            Enclosure::point(lift(torsion)?),
            Enclosure::point(lift(f.radius)?),
        )
    } else {
        (sec.a, sec.z, sec.j, Enclosure::point(sec.c))
    };
    match r {
        ProductRecipe::Stress { site, stress, .. } => {
            let component = match stress {
                ProductStress::Axial => Component::Ux,
                ProductStress::BendingY => Component::Ry,
                ProductStress::BendingZ => Component::Rz,
                ProductStress::Torsion => Component::Rx,
            };
            let x = action(owner, values, member, site, component)?;
            Ok(match stress {
                ProductStress::Axial => corners(w, x, a, Entry::Div)?,
                ProductStress::BendingY | ProductStress::BendingZ => corners(w, x, z, Entry::Div)?,
                ProductStress::Torsion => {
                    let p = corners(w, x, c, Entry::Mul)?;
                    corners(w, p, j, Entry::Div)?
                }
            })
        }
        ProductRecipe::CircularMaximum { .. } => {
            if owner.member_is_loaded(member) {
                let curvature = curvatures.iter().find(|c| c.0 == member).ok_or_else(|| bad("loaded maximum curvature"))?.1;
                let mut ends = [[Enclosure::point(Endpoint::ZERO);3];2];
                for (e,end) in [End::I,End::J].into_iter().enumerate() {
                    for (c,component) in [Component::Ux,Component::Ry,Component::Rz].into_iter().enumerate() {
                        ends[e][c] = action(owner,values,member,ProductSite::End(end),component)?;
                    }
                }
                // W1 coverage repair §1.2: a lane whose Z is the represented hull
                // and not a point is driven at z_lo (the corner-1 switch).
                let driven = corner1() && (represented || shared) && z.lo.cmp_value(&z.hi) != Ordering::Equal;
                #[cfg(feature = "census-probe")]
                census_probe::lane(represented);
                return loaded_maximum::search(spent,row,member,ends,curvature,a,z,driven);
            }
            let mut best = None;
            for end in [End::I, End::J] {
                let site = ProductSite::End(end);
                let n = abs_range(action(owner, values, member, site, Component::Ux)?);
                let my = action(owner, values, member, site, Component::Ry)?;
                let mz = action(owner, values, member, site, Component::Rz)?;
                let bending = norm2(w, my, mz)?;
                let axial = corners(w, n, a, Entry::Div)?;
                let bending = corners(w, bending, z, Entry::Div)?;
                let v = w.add(axial, bending)?;
                best = Some(best.map_or(v, |b: Enclosure| Enclosure {
                    lo: max(b.lo, v.lo),
                    hi: max(b.hi, v.hi),
                }));
            }
            Ok(best.unwrap())
        }
        _ => Err(bad("recipe")),
    }
}
fn raw_interval(
    w: &mut NumericWork,
    a: Enclosure,
    u: ProductUnit,
) -> Result<Enclosure, NumericError> {
    match u {
        ProductUnit::Millimetre => corners(w, a, Enclosure::point(lift(1000.0)?), Entry::Mul),
        ProductUnit::Megapascal => corners(w, a, Enclosure::point(lift(1_000_000.0)?), Entry::Div),
        _ => Ok(a),
    }
}
fn distance(w: &mut NumericWork, x: f64, a: Enclosure) -> Result<Endpoint, NumericError> {
    let x = lift(x)?;
    Ok(max(
        w.scalar(Entry::Sub, &x, &a.lo, Toward::Up)?,
        w.scalar(Entry::Sub, &a.hi, &x, Toward::Up)?,
    ))
}
fn exact_test(
    spent: &mut ProductCertificateSpent<'_>,
    h: Endpoint,
    x: f64,
    s: f64,
    sharper: bool,
) -> Result<bool, ProductFailure> {
    let mut sum = ExactWideSum::new();
    let result = (|| -> Result<bool, NumericError> {
        let ax = lift(x)?.abs();
        if sharper {
            let m = max(ax, lift(s)?);
            sum.add_wide_scaled(&m, false, 1, -64)
                .map_err(|e| NumericError::Arithmetic(e.into()))?;
            sum.add_wide_scaled(&m, false, 1, -85)
                .map_err(|e| NumericError::Arithmetic(e.into()))?;
            sum.add_wide_scaled(&ax, false, 1, -53)
                .map_err(|e| NumericError::Arithmetic(e.into()))?;
            sum.add_binary64(f64::from_bits(1), false)
                .map_err(|e| NumericError::Arithmetic(e.into()))?;
            sum.add_wide(&h, true)
                .map_err(|e| NumericError::Arithmetic(e.into()))?;
        } else {
            sum.add_wide(&ax, false)
                .map_err(|e| NumericError::Arithmetic(e.into()))?;
            sum.add_wide_scaled(&h, true, 1_000_000_000, 0)
                .map_err(|e| NumericError::Arithmetic(e.into()))?;
        }
        Ok(sum
            .signum()
            .map_err(|e| NumericError::Arithmetic(e.into()))?
            >= 0)
    })();
    spent.comparisons.merge(&sum.work());
    let collected = spent.visit();
    match result {
        Err(e) => Err(e.into()),
        Ok(v) => {
            collected?;
            Ok(v)
        }
    }
}
fn gate(
    spent: &mut ProductCertificateSpent<'_>,
    row: usize,
    interval: Enclosure,
    scale: f64,
    input: bool,
) -> Result<ProductRowVerdict, ProductFailure> {
    let r = &spent.rows[row];
    let (y, u) = (*r.value, r.unit);
    let n = spent.normalize(u, y)?;
    let h = distance(&mut spent.numeric, n, interval)?;
    let small = f64::from_bits(0x0230_0000_0000_0000);
    let relative = if input || scale < small {
        false
    } else {
        let threshold = spent.f64_op(
            Scalar64::Mul,
            f64::from_bits(adaptive::FLOOR_RATIO_BITS),
            scale,
        )?;
        !(n.abs() < threshold)
    };
    let class = if input {
        adaptive::RowClass::InputDerived
    } else if relative {
        adaptive::RowClass::RelativeVerified
    } else {
        adaptive::RowClass::AbsoluteVerified { bound_bits: 0 }
    };
    let mut final_class = class;
    let mut predicates = [None; 4];
    let fail = match class {
        adaptive::RowClass::InputDerived => {
            predicates[0] = Some(h.is_zero());
            if h.is_zero() {
                None
            } else {
                Some(ProductPredicate::InputDerived)
            }
        }
        adaptive::RowClass::AbsoluteVerified { .. } => {
            let b = if scale > 0.0 && scale < f64::from_bits(0x0230_0000_0000_0000) {
                spent.small_bound(n, scale)?
            } else {
                spent.absolute(scale)?
            };
            final_class = adaptive::RowClass::AbsoluteVerified {
                bound_bits: b.to_bits(),
            };
            predicates[0] = Some(h.cmp_value(&lift(b)?) != Ordering::Greater);
            if h.cmp_value(&lift(b)?) == Ordering::Greater {
                Some(ProductPredicate::Absolute)
            } else {
                None
            }
        }
        adaptive::RowClass::RelativeVerified => {
            let a0 = spent.f64_op(
                Scalar64::Mul,
                f64::from_bits((1023u64 - 64) << 52),
                n.abs().max(scale),
            )?;
            let a1 = spent.f64_op(Scalar64::Mul, a0, f64::from_bits(0x3ff0_0000_8000_0000))?;
            let u0 = spent.f64_op(Scalar64::Mul, f64::from_bits((1023u64 - 53) << 52), n.abs())?;
            let u1 = spent.f64_op(Scalar64::Add, u0, f64::from_bits(1))?;
            let allowance = spent.f64_op(Scalar64::Add, a1, u1)?;
            if !allowance.is_finite() {
                return Err(bad("allowance range"));
            }
            let raw = raw_interval(&mut spent.numeric, interval, u)?;
            let hu = distance(&mut spent.numeric, y, raw)?;
            let tests = [
                (
                    ProductPredicate::SharperExact,
                    exact_test(spent, h, n, scale, true)?,
                ),
                (
                    ProductPredicate::SharperBinary64,
                    h.cmp_value(&lift(allowance)?) != Ordering::Greater,
                ),
                (
                    ProductPredicate::DecimalSi,
                    exact_test(spent, h, n, scale, false)?,
                ),
                (
                    ProductPredicate::DecimalRaw,
                    exact_test(spent, hu, y, scale, false)?,
                ),
            ];
            for (i, (_, ok)) in tests.iter().enumerate() {
                predicates[i] = Some(*ok);
            }
            tests.into_iter().find(|(_, ok)| !*ok).map(|(p, _)| p)
        }
        _ => return Err(bad("unpublishable final")),
    };
    Ok(ProductRowVerdict {
        row,
        normalized_bits: n.to_bits(),
        scale_bits: scale.to_bits(),
        class: Some(final_class),
        passed: fail.is_none(),
        failed: fail,
        predicates,
    })
}
fn couple(
    s: [f64; 4],
    extent: f64,
    spent: &mut ProductCertificateSpent<'_>,
) -> Result<[f64; 4], ProductFailure> {
    if extent == 0.0 {
        return Ok(s);
    }
    let [tr, ro, fo, mo] = s;
    Ok([
        tr.max(spent.f64_op(Scalar64::Mul, extent, ro)?),
        ro.max(spent.f64_op(Scalar64::Div, tr, extent)?),
        fo.max(spent.f64_op(Scalar64::Div, mo, extent)?),
        mo.max(spent.f64_op(Scalar64::Mul, extent, fo)?),
    ])
}
fn body_extent(
    source: &super::super::source::PrimitiveSource,
    body: u32,
    spent: &mut ProductCertificateSpent<'_>,
) -> Result<f64, ProductFailure> {
    let mut bounds = None;
    for (i, p) in source.nodes().iter().enumerate() {
        spent.visit()?;
        if source.body_of_node(i as u32) != body {
            continue;
        }
        let (mut lo, mut hi) = bounds.unwrap_or((*p, *p));
        for a in 0..3 {
            lo[a] = lo[a].min(p[a]);
            hi[a] = hi[a].max(p[a]);
        }
        bounds = Some((lo, hi));
    }
    let (lo, hi) = bounds.ok_or_else(|| bad("body nodes"))?;
    let d = [
        spent.f64_op(Scalar64::Sub, hi[0], lo[0])?,
        spent.f64_op(Scalar64::Sub, hi[1], lo[1])?,
        spent.f64_op(Scalar64::Sub, hi[2], lo[2])?,
    ];
    let x = spent.f64_op(Scalar64::Mul, d[0], d[0])?;
    let y = spent.f64_op(Scalar64::Mul, d[1], d[1])?;
    let xy = spent.f64_op(Scalar64::Add, x, y)?;
    let z = spent.f64_op(Scalar64::Mul, d[2], d[2])?;
    let xyz = spent.f64_op(Scalar64::Add, xy, z)?;
    let extent = spent.f64_op(Scalar64::Sqrt, xyz, 0.0)?;
    if !extent.is_finite() {
        return Err(bad("body extent"));
    }
    Ok(extent)
}
pub(crate) fn certify<'a>(
    invocation: &RecordedInvocation,
    run: usize,
    owner: &adaptive::RetainedSolve,
    facts: &[ProductMemberFacts],
    rows: &'a [ProductFinalRow<'a>],
) -> ProductCertificateSpent<'a> {
    certify_reduced(invocation, run, owner, facts, ReducedSectionSet::empty(), rows)
}
/// `certify` with O4 §4.5's reduced sections (an empty set is `certify`).
pub(crate) fn certify_reduced<'a>(
    invocation: &RecordedInvocation,
    run: usize,
    owner: &adaptive::RetainedSolve,
    facts: &[ProductMemberFacts],
    reduced: &ReducedSectionSet,
    rows: &'a [ProductFinalRow<'a>],
) -> ProductCertificateSpent<'a> {
    let mut spent = ProductCertificateSpent::new(rows);
    let result = run_case(invocation, run, owner, facts, reduced, &mut spent);
    if let Err(e) = result {
        spent.failure = Some(e);
    }
    spent
}
fn run_case(
    invocation: &RecordedInvocation,
    run: usize,
    owner: &adaptive::RetainedSolve,
    facts: &[ProductMemberFacts],
    reduced: &ReducedSectionSet,
    spent: &mut ProductCertificateSpent<'_>,
) -> Result<(), ProductFailure> {
    if invocation.product_owner(run, owner).is_none() {
        return Err(bad("recorded owner"));
    }
    let source = owner.source();
    if facts.len() != source.members().len() {
        return Err(bad("member count"));
    }
    let mut laws = reserve(facts.len())?;
    spent.capacities[0] = laws.capacity();
    for (i, f) in facts.iter().enumerate() {
        spent.visit()?;
        let m = &source.members()[i];
        let half_diameter = spent.f64_op(Scalar64::Div, f.diameter, 2.0)?;
        if m.id != f.member
            || m.area.to_bits() != f.area.to_bits()
            || m.second_moment_y.to_bits() != f.second_moment.to_bits()
            || m.second_moment_z.to_bits() != f.second_moment.to_bits()
            || m.torsion_constant.to_bits() != f.torsion_constant.to_bits()
            || half_diameter.to_bits() != f.radius.to_bits()
        {
            return Err(bad("section bits"));
        }
        let material = f.material.operands();
        laws.push(bridge::ProposedMemberLaw {
            member: m,
            diameter: f.diameter,
            effective_wall: f.effective_wall,
            material,
            represented_z: f.section_modulus,
        });
    }
    // Own producing residual work even when the conditional numeric result fails.
    let residual = source_residual::source_residual(
        owner,
        source,
        &owner.evidence().source_encoding,
        owner.selected_precision(),
        &laws,
    );
    let lane_result=residual.result();
    spent.lane_errors[1]=lane_result.as_ref().err().cloned();spent.trace_copy_work.record::<Option<bridge::BridgeError>>();
    let result = (|| {
        let native = lane_result.map_err(|e| ProductFailure {
            cause: Cause::Native(e),
        })?;
        let mut k = reserve(owner.publish().rows.len())?;
        spent.capacities[2] = k.capacity();
        for i in 0..owner.publish().rows.len() {
            spent.visit()?;
            let (r, radius) = native.view.row(i);
            let x = lift(r.value.value().ok_or_else(|| bad("native unpublished"))?)?;
            k.push(if r.class == adaptive::RowClass::InputDerived {
                Enclosure::point(x)
            } else {
                let rad = lift(radius.ok_or_else(|| bad("missing radius"))?)?;
                Enclosure {
                    lo: spent.numeric.scalar(Entry::Sub, &x, &rad, Toward::Down)?,
                    hi: spent.numeric.scalar(Entry::Add, &x, &rad, Toward::Up)?,
                }
            });
        }
        check_intervals(owner,facts,reduced,&k,&native.rows,native.view.data(),spent,&native.curvatures,&native.curvatures)?;
        Ok(())
    })();
    spent.native = Some(residual.work);
    result
}
/// The stress section operands of member `mi`: its nominal facts, or (O4
/// §4.5) its reduced section's facts on the reduced annulus (D, t_r).
fn section_operands(m:&super::super::source::StraightMember,f:&ProductMemberFacts,reduced:Option<&ReducedSection>)
    -> MemberOperands {
    match reduced {
        None=>MemberOperands{diameter:f.diameter,effective_wall:f.effective_wall,material:f.material.operands(),
            admitted:AdmittedOperands{e:m.elastic_modulus,g:m.shear_modulus,a:m.area,j:m.torsion_constant,
                iy:m.second_moment_y,iz:m.second_moment_z,z_hat:f.section_modulus}},
        Some(r)=>MemberOperands{diameter:f.diameter,effective_wall:r.effective_wall,material:f.material.operands(),
            admitted:AdmittedOperands{e:m.elastic_modulus,g:m.shear_modulus,a:r.area,j:r.torsion_constant,
                iy:r.second_moment,iz:r.second_moment,z_hat:r.section_modulus}},
    }
}
/// O4 §4.5: the reduced-row index map of `n` final rows, `u32::MAX` where a
/// row is not a reduced row, else its `ReducedSectionSet` entry. Allocated
/// only for a non-empty set (an empty set returns an empty map, no work).
/// Refuses an entry whose member is not the owner's, an entry whose five
/// operands (A, Z, I, J, wall) are bit-equal to the nominal facts, and an
/// index outside the final rows.
fn reduced_index(owner:&adaptive::RetainedSolve,facts:&[ProductMemberFacts],reduced:&ReducedSectionSet,n:usize,
    spent:&mut ProductCertificateSpent<'_>)->Result<Vec<u32>,ProductFailure> {
    if reduced.is_empty() {return Ok(Vec::new());}
    let mut map=reserve::<u32>(n)?;
    spent.reduced_capacities[0]=map.capacity();
    map.resize(n,u32::MAX);
    for (e,r) in reduced.sections().iter().enumerate() {
        spent.visit()?;
        let mi=owner.source().member_index(r.member).ok_or_else(||bad("reduced section member"))?;
        let f=&facts[mi];
        if [f.area,f.section_modulus,f.second_moment,f.torsion_constant,f.effective_wall].map(f64::to_bits)
            ==[r.area,r.section_modulus,r.second_moment,r.torsion_constant,r.effective_wall].map(f64::to_bits) {
            return Err(bad("reduced section equal to nominal"));
        }
        for &row in &r.rows {
            spent.visit()?;
            let slot=map.get_mut(row as usize).ok_or_else(||bad("reduced row index"))?;
            if *slot!=u32::MAX {return Err(bad("reduced row index"));}
            *slot=u32::try_from(e).map_err(|_|ProductFailure{cause:Cause::CountRange("reduced sections")})?;
        }
    }
    Ok(map)
}
/// The reduced section of final row `row`, if it is one (empty map: none).
fn reduced_of<'r>(reduced:&'r ReducedSectionSet,map:&[u32],row:usize)->Option<&'r ReducedSection> {
    map.get(row).filter(|e|**e!=u32::MAX).map(|e|&reduced.sections()[*e as usize])
}
fn row_scales(owner:&adaptive::RetainedSolve,facts:&[ProductMemberFacts],spent:&mut ProductCertificateSpent<'_>)
    -> Result<Vec<[f64;4]>,ProductFailure> {
    row_scales_reduced(owner,facts,ReducedSectionSet::empty(),spent).map(|v|v.0)
}
/// `row_scales` with O4 §4.5's reduced rows: a row the map names is a
/// reduced stress or maximum row of its entry's member (else refused by
/// "reduced row identity"), covered in the separate reduced-coverage vector,
/// allocated only for a non-empty set. Returns the scales and the map.
fn row_scales_reduced(owner:&adaptive::RetainedSolve,facts:&[ProductMemberFacts],reduced:&ReducedSectionSet,
    spent:&mut ProductCertificateSpent<'_>)
    -> Result<(Vec<[f64;4]>,Vec<u32>),ProductFailure> {
    let source=owner.source();
    // B2-K (KD §5.2, R-7): a mechanics combination owner publishes no circular
    // maximum, intensified row, mode or modulus record. Its coverage is
    // every native row and derivative slots 0-19 of each member (slot 20 stays
    // empty); no mode record is required. Every other rule is the case's.
    let combination=!owner.prep.factors.is_empty();
        let nb = source.body_count() as usize;
        let mut scales = reserve(nb)?;
        spent.capacities[6] = scales.capacity();
        scales.resize(nb, [0.0f64; 4]);
        let mut native_coverage = reserve(owner.publish().rows.len())?;
        spent.capacities[1] = native_coverage.capacity();
        native_coverage.resize(owner.publish().rows.len(), false);
        let derivative_count = facts.len().checked_mul(21).ok_or_else(|| ProductFailure {
            cause: Cause::CountRange("derivative rows"),
        })?;
        let mut derivative_coverage = reserve(derivative_count)?;
        spent.capacities[4] = derivative_coverage.capacity();
        derivative_coverage.resize(derivative_count, false);
        let mut nonquantity = false;
        let mut modulus_basis = false;
        if spent.verdicts.capacity()<spent.rows.len() {
            spent.visit()?;
            // Release the previous allocation before reserving; normal prepared
            // begin/final uses the first capacity unchanged.
            drop(std::mem::take(&mut spent.verdicts));
            spent.verdicts=reserve(spent.rows.len())?;
        }else{spent.verdicts.clear();}
        spent.capacities[3] = spent.verdicts.capacity();
        let support_count = source.supports().len().checked_mul(6)
            .ok_or_else(|| ProductFailure { cause: Cause::CountRange("support rows") })?;
        spent.visit()?; // entered support-slot allocation
        let mut support_coverage = reserve::<bool>(support_count)?;
        spent.capacities[5] = support_coverage.capacity();
        support_coverage.capacity().checked_mul(std::mem::size_of::<bool>())
            .ok_or_else(|| ProductFailure { cause: Cause::CountRange("support capacity bytes") })?;
        for _ in 0..support_count { spent.visit()?; support_coverage.push(false); }
        let reduced_map = reduced_index(owner, facts, reduced, spent.rows.len(), spent)?;
        let mut reduced_coverage = if reduced.is_empty() { Vec::new() } else {
            let n = reduced.sections().len().checked_mul(21)
                .ok_or_else(|| ProductFailure { cause: Cause::CountRange("reduced rows") })?;
            let mut v = reserve::<bool>(n)?;
            spent.reduced_capacities[1] = v.capacity();
            v.resize(n, false);
            v
        };
        let final_rows = spent.rows;
        for (i, r) in final_rows.iter().enumerate() {
            spent.visit()?;
            if r.id.is_empty()
                || r.case_id.is_empty()
                || !r.value.is_finite()
                || r.body as usize >= nb
            {
                return Err(bad("final row"));
            }
            if spent.rows[..i].iter().any(|a| a.id == r.id) {
                return Err(bad("duplicate final id"));
            }
            if r.case_id != spent.rows[0].case_id {
                return Err(bad("case association"));
            }
            // O4 §4.5: a mapped row is its entry member's reduced stress or
            // maximum row, covered in the reduced-coverage vector.
            let reduced_entry = reduced_map.get(i).copied().filter(|e| *e != u32::MAX);
            if let Some(e) = reduced_entry {
                spent.visit()?;
                let member = reduced.sections()[e as usize].member;
                let ok = match r.recipe {
                    ProductRecipe::Stress { member: m, .. } | ProductRecipe::CircularMaximum { member: m } => m == member,
                    _ => false,
                };
                if !ok { return Err(bad("reduced row identity")); }
            }
            // §2.7/§4.2: a combination publishes no never_combined (not-covered derived) row.
            // W-7 (DEL-04-01 Design `global-end-actions.md`): a not-covered force or moment row
            // (a member's global end action, signed linear) is combined and admitted; the
            // derived not-covered rows are stresses.
            if combination && (matches!(r.recipe, ProductRecipe::NonQuantity
                | ProductRecipe::ModulusBasisRecord | ProductRecipe::CircularMaximum { .. })
                || (matches!(r.recipe, ProductRecipe::NotCovered { .. })
                    && !matches!(r.unit, ProductUnit::Newton | ProductUnit::NewtonMetre))) {
                return Err(bad("combination row family"));
            }
            match r.recipe {
                ProductRecipe::NonQuantity => {
                    if nonquantity || r.unit != ProductUnit::Record {
                        return Err(bad("nonquantity coverage"));
                    }
                    nonquantity = true;
                }
                ProductRecipe::ModulusBasisRecord => {
                    if modulus_basis || r.unit != ProductUnit::Record {
                        return Err(bad("modulus basis coverage"));
                    }
                    modulus_basis = true;
                }
                // §4.1: position-bound, a quantity unit, and nothing else: no
                // scale, coverage slot or verified count.
                ProductRecipe::NotCovered { row } => {
                    if row as usize != i || r.unit == ProductUnit::Record {
                        return Err(bad("not-covered row"));
                    }
                }
                ProductRecipe::SupportComponent { support, component } => {
                    let (si, contributor) = support_component(owner, support, component, spent)?;
                    let group = &source.supports()[si];
                    let slot = if component.index() < 3 { 2 } else { 3 };
                    let unit = if slot == 2 { ProductUnit::Newton } else { ProductUnit::NewtonMetre };
                    if source.body_of_node(group.node) != r.body || r.unit != unit {
                        return Err(bad("support body/unit"));
                    }
                    let j = si.checked_mul(6).and_then(|v| v.checked_add(component.index()))
                        .ok_or_else(|| ProductFailure { cause: Cause::CountRange("support slot") })?;
                    spent.visit()?;
                    if support_coverage[j] { return Err(bad("duplicate support component")); }
                    support_coverage[j] = true;
                    if let Some(index) = contributor {
                        spent.visit()?;
                        if native_coverage[index] { return Err(bad("native coverage")); }
                        native_coverage[index] = true;
                    }
                    let n = spent.normalize(r.unit, *r.value)?;
                    scales[r.body as usize][slot] = scales[r.body as usize][slot].max(n.abs());
                }
                ProductRecipe::Stress {
                    member,
                    site,
                    stress,
                } => {
                    let mi = source
                        .member_index(member)
                        .ok_or_else(|| bad("stress member"))?;
                    if source.body_of_node(source.members()[mi].node_i) != r.body {
                        return Err(bad("stress body"));
                    }
                    let site_index = match site {
                        ProductSite::End(End::I) => 0,
                        ProductSite::End(End::J) => 1,
                        ProductSite::Station(id) => {
                            let station = source
                                .stations()
                                .iter()
                                .find(|s| s.id == id && s.member == member)
                                .ok_or_else(|| bad("stress station"))?;
                            [0.25f64, 0.5, 0.75]
                                .iter()
                                .position(|f| f.to_bits() == station.fraction.to_bits())
                                .ok_or_else(|| bad("stress fraction"))?
                                + 2
                        }
                    };
                    let component = match stress {
                        ProductStress::Axial => 0,
                        ProductStress::BendingY => 1,
                        ProductStress::BendingZ => 2,
                        ProductStress::Torsion => 3,
                    };
                    let (coverage, base) = match reduced_entry {
                        Some(e) => (&mut reduced_coverage, e as usize * 21),
                        None => (&mut derivative_coverage, mi * 21),
                    };
                    let j = base + site_index * 4 + component;
                    if coverage[j] || r.unit != ProductUnit::Megapascal {
                        return Err(bad("stress coverage"));
                    }
                    coverage[j] = true;
                }
                ProductRecipe::CircularMaximum { member } => {
                    let mi = source
                        .member_index(member)
                        .ok_or_else(|| bad("maximum member"))?;
                    if source.body_of_node(source.members()[mi].node_i) != r.body
                        || r.unit != ProductUnit::Pascal
                    {
                        return Err(bad("maximum body/unit"));
                    }
                    let (coverage, j) = match reduced_entry {
                        Some(e) => (&mut reduced_coverage, e as usize * 21 + 20),
                        None => (&mut derivative_coverage, mi * 21 + 20),
                    };
                    if coverage[j] {
                        return Err(bad("maximum coverage"));
                    }
                    coverage[j] = true;
                }
                _ => {}
            }
            if let ProductRecipe::Native(id) = r.recipe {
                let index = native_index(owner, id)?;
                let nr = &owner.publish().rows[index];
                if native_coverage[index] || nr.body != r.body {
                    return Err(bad("native coverage"));
                }
                native_coverage[index] = true;
                if nr.class != adaptive::RowClass::InputDerived {
                    let n = spent.normalize(r.unit, *r.value)?;
                    let slot = match nr.kind {
                        Kind::Translation => 0,
                        Kind::Rotation => 1,
                        Kind::Force => 2,
                        Kind::Moment => 3,
                    };
                    scales[r.body as usize][slot] = scales[r.body as usize][slot].max(n.abs());
                }
            }
        }
        for covered in &support_coverage {
            spent.visit()?;
            if !covered { return Err(bad("missing support coverage")); }
        }
        if native_coverage.iter().any(|v| !*v)
            || derivative_coverage.iter().enumerate().any(|(j, v)| !*v && !(combination && j % 21 == 20))
            || !(nonquantity || combination)
        {
            return Err(bad("missing final coverage"));
        }
        if reduced_coverage.iter().enumerate().any(|(j, v)| !*v && !(combination && j % 21 == 20)) {
            return Err(bad("missing reduced coverage"));
        }
    Ok((scales, reduced_map))
}

#[allow(clippy::too_many_arguments)]
fn check_intervals(owner:&adaptive::RetainedSolve, facts:&[ProductMemberFacts], reduced:&ReducedSectionSet,
    k:&[Enclosure], source_values:&[Enclosure], data:&[bool], spent:&mut ProductCertificateSpent<'_>,
    k_curvatures:&[(u32,[Enclosure;2])], s_curvatures:&[(u32,[Enclosure;2])])
    -> Result<Vec<[f64;4]>,ProductFailure> {
    let source=owner.source();
        #[cfg(feature = "census-probe")]
        census_probe::context(spent.rows.first().map_or("", |r| r.case_id), "gate");
        #[cfg(feature = "census-probe")]
        let mut probe_rows: Vec<census_probe::GateRow> = Vec::new();
        spent.coverage = summary_coverage_data(owner, data, spent)?;
        let nb=source.body_count() as usize;
        let (mut scales, reduced_map)=row_scales_reduced(owner,facts,reduced,spent)?;
        for b in 0..nb {
            let extent = body_extent(source, b as u32, spent)?;
            scales[b] = couple(scales[b], extent, spent)?;
            if owner.selected_precision() == 512 {
                let floor = owner
                    .evidence()
                    .floor
                    .as_ref()
                    .and_then(|f| f.iter().find(|f| f.0 == b as u32))
                    .ok_or_else(|| bad("floor"))?;
                scales[b][2] = scales[b][2].max(f64::from_bits(floor.1));
                scales[b][3] = scales[b][3].max(f64::from_bits(floor.2));
            }
        }
        for i in 0..spent.rows.len() {
            spent.visit()?;
            let r = &spent.rows[i];
            let recipe_id = r.recipe;
            if matches!(
                recipe_id,
                ProductRecipe::NonQuantity | ProductRecipe::ModulusBasisRecord | ProductRecipe::NotCovered { .. }
            ) {
                spent.verdicts.push(ProductRowVerdict {
                    row: i,
                    normalized_bits: r.value.to_bits(),
                    scale_bits: 0,
                    class: None,
                    passed: true,
                    failed: None,
                    predicates: [None; 4],
                });
                continue;
            }
            let (scale, input) = match recipe_id {
                ProductRecipe::Native(id) => {
                    let index = native_index(owner, id)?;
                    let nr = &owner.publish().rows[index];
                    let slot = match nr.kind {
                        Kind::Translation => 0,
                        Kind::Rotation => 1,
                        Kind::Force => 2,
                        Kind::Moment => 3,
                    };
                    (
                        scales[r.body as usize][slot],
                        nr.class == adaptive::RowClass::InputDerived,
                    )
                }
                ProductRecipe::SupportComponent { component, .. } => (
                    scales[r.body as usize][if component.index() < 3 { 2 } else { 3 }], false
                ),
                ProductRecipe::Stress { member, .. }
                | ProductRecipe::CircularMaximum { member } => {
                    let f = &facts[source
                        .member_index(member)
                        .ok_or_else(|| bad("stress member"))?];
                    let k = if matches!(recipe_id, ProductRecipe::CircularMaximum { .. }) {
                        f64::from_bits(adaptive::K_TWO_SQRT2_BITS)
                    } else {
                        1.0
                    };
                    // O4 §4.5: a reduced row's body scale is divided by its own section.
                    let (area, modulus) = match reduced_of(reduced, &reduced_map, i) {
                        Some(rs) => (rs.area, rs.section_modulus),
                        None => (f.area, f.section_modulus),
                    };
                    let sc = scales[r.body as usize];
                    let axial = spent.f64_op(Scalar64::Div, sc[2], area)?;
                    let bending = spent.f64_op(Scalar64::Div, sc[3], modulus)?;
                    let bending = spent.f64_op(Scalar64::Mul, k, bending)?;
                    (spent.f64_op(Scalar64::Add, axial, bending)?, false)
                }
                _ => unreachable!(),
            };
            if !scale.is_finite() || scale < 0.0 {
                return Err(bad("final scale"));
            }
            // One active section only. No member-sized coefficient cache.
            let section = match recipe_id {
                ProductRecipe::Stress { member, .. }
                | ProductRecipe::CircularMaximum { member } => {
                    let mi = source
                        .member_index(member)
                        .ok_or_else(|| bad("recipe member"))?;
                    let (m, f) = (&source.members()[mi], &facts[mi]);
                    let operands = section_operands(m, f, reduced_of(reduced, &reduced_map, i));
                    Some(build_member(&operands, &mut spent.numeric)?)
                }
                _ => None,
            };
            let row_reduced = reduced_of(reduced, &reduced_map, i);
            let represented = recipe_reduced(
                owner,
                spent,
                k,
                recipe_id,
                section.as_ref(),
                facts,
                row_reduced,
                true,
                k_curvatures,
                i,
            )?;
            let geometric = recipe_reduced(
                owner,
                spent,
                source_values,
                recipe_id,
                section.as_ref(),
                facts,
                row_reduced,
                false,
                s_curvatures,
                i,
            )?;
            let v = gate(spent, i, hull(represented, geometric), scale, input)?;
            spent.verdicts.push(v);
            #[cfg(feature = "census-probe")]
            if census_probe::switches().record {
                probe_rows.push(census_probe::GateRow { row: i, k: represented, s: geometric, scale, input });
            }
        }
        #[cfg(feature = "census-probe")]
        if census_probe::switches().record {
            census_probe::gate_records(&census_probe::GateInputs { owner, facts, reduced, reduced_map: &reduced_map,
                k, s: source_values, k_curvatures, s_curvatures, rows: spent.rows, verdicts: &spent.verdicts,
                scales: &scales, gate_rows: &probe_rows });
        }
    // W1 coverage repair §2.2: the coupled, floored scales the gate used, for the
    // certified proof (the vector `row_scales_reduced` reserved; no new allocation).
    Ok(scales)
}

#[cfg(test)]
#[path = "../../../../tests/retained_k4/product_final_case_tests.rs"]
mod product_final_case_tests;

#[cfg(test)]
#[path = "../../../../tests/retained_k4/o4_product_tests.rs"]
mod o4_product_tests;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProductSummaryCoverage {
    pub body: u32,
    pub stop: [bool; 4],
    pub estimate: [bool; 2],
    pub charge: [bool; 2],
    pub has_data: bool,
}
fn verification_nonzero(
    owner: &adaptive::RetainedSolve,
    index: usize,
) -> Result<bool, ProductFailure> {
    use adaptive::PrecisionState::*;
    let state = owner
        .state(owner.selected_precision() * 2)
        .ok_or_else(|| bad("verification state"))?;
    let value = match state {
        P128(s) | P256(s) => !s
            .recovered
            .values
            .get(index)
            .ok_or_else(|| bad("verification row"))?
            .is_zero(),
        P512(s) => !s
            .recovered
            .values
            .get(index)
            .ok_or_else(|| bad("verification row"))?
            .is_zero(),
        P1024(s) => !s
            .recovered
            .values
            .get(index)
            .ok_or_else(|| bad("verification row"))?
            .is_zero(),
    };
    Ok(value)
}
fn summary_coverage(
    owner: &adaptive::RetainedSolve,
    view: &adaptive::SourceBridgeView<'_>,
    spent: &mut ProductCertificateSpent<'_>,
) -> Result<Vec<ProductSummaryCoverage>, ProductFailure> {
    summary_coverage_data(owner,view.data(),spent)
}
fn summary_coverage_data(owner:&adaptive::RetainedSolve, data:&[bool], spent:&mut ProductCertificateSpent<'_>)
    -> Result<Vec<ProductSummaryCoverage>,ProductFailure> {
    if data.len()!=owner.group.blocks.body.len() { return Err(bad("coverage data length")); }
    let mut out = reserve(owner.source().body_count() as usize)?;
    for body in 0..owner.source().body_count() {
        spent.visit()?;
        let mut positive = [false; 4];
        let mut present = [false; 4];
        for (i, meta) in owner.prep.layout.iter().enumerate() {
            if meta.body != body {
                continue;
            }
            let kind = match meta.kind {
                Kind::Translation => 0,
                Kind::Rotation => 1,
                Kind::Force => 2,
                Kind::Moment => 3,
            };
            present[kind] = true;
            if !meta.input_derived && verification_nonzero(owner, i)? {
                positive[kind] = true;
            }
        }
        if owner.prep.extents[body as usize] != 0.0 {
            positive = [
                positive[0] || positive[1],
                positive[0] || positive[1],
                positive[2] || positive[3],
                positive[2] || positive[3],
            ];
        }
        if let Some(floor) = &owner.evidence().floor {
            let f = floor
                .iter()
                .find(|f| f.0 == body)
                .ok_or_else(|| bad("coverage floor"))?;
            positive[2] |= f64::from_bits(f.1) > 0.0;
            positive[3] |= f64::from_bits(f.2) > 0.0;
        }
        let mut stop = [false; 4];
        for (i, meta) in owner.prep.layout.iter().enumerate() {
            if meta.body != body {
                continue;
            }
            let kind = match meta.kind {
                Kind::Translation => 0,
                Kind::Rotation => 1,
                Kind::Force => 2,
                Kind::Moment => 3,
            };
            stop[kind] |= positive[kind] || verification_nonzero(owner, i)?;
        }
        let e = owner
            .evidence()
            .resolution_scale
            .iter()
            .find(|e| e.0 == body)
            .ok_or_else(|| bad("coverage resolution"))?;
        let mut hats = [f64::from_bits(e.1) > 0.0, f64::from_bits(e.2) > 0.0];
        if owner.prep.extents[body as usize] != 0.0 {
            hats = [hats[0] || hats[1]; 2];
        }
        let estimate = [present[2] && hats[0], present[3] && hats[1]];
        let charge = if owner.selected_precision() == 512 {
            [present[2] && positive[2], present[3] && positive[3]]
        } else {
            estimate
        };
        let has_data = data.iter().enumerate()
            .any(|(i, data)| *data && owner.group.blocks.body[i] == body);
        out.push(ProductSummaryCoverage {
            body,
            stop,
            estimate,
            charge,
            has_data,
        });
    }
    Ok(out)
}


// I51: one owner-bound dual draft, with no residual recomputation at projection
// or final checking. Scalar intervals never cross the crate boundary.
#[derive(Debug)]
pub struct ProductRowSpec<'a> {
    id:&'a str, case_id:&'a str, unit:ProductUnit, body:u32, recipe:ProductRecipe,
    observed:Option<u64>,
}
impl<'a> ProductRowSpec<'a> {
    pub fn mechanical(id:&'a str,case_id:&'a str,unit:ProductUnit,body:u32,recipe:ProductRecipe)
        -> Result<Self,ProductFailure> {
        if id.is_empty() || case_id.is_empty() || unit==ProductUnit::Record
            || matches!(recipe,ProductRecipe::NonQuantity|ProductRecipe::ModulusBasisRecord) {
            return Err(bad("mechanical descriptor"));
        }
        Ok(Self{id,case_id,unit,body,recipe,observed:None})
    }
    pub fn mode(id:&'a str,case_id:&'a str,body:u32,code:u8)->Result<Self,ProductFailure> {
        if !matches!(code,1|2) {return Err(bad("mode code"));}
        Ok(Self{id,case_id,body,unit:ProductUnit::Record,recipe:ProductRecipe::NonQuantity,observed:Some(f64::from(code).to_bits())})
    }
    pub fn material_record(id:&'a str,case_id:&'a str,body:u32)->Self {
        Self{id,case_id,body,unit:ProductUnit::Record,recipe:ProductRecipe::ModulusBasisRecord,observed:Some(1f64.to_bits())}
    }
    fn row<'v>(&'v self,value:&'v f64)->ProductFinalRow<'v> {
        ProductFinalRow{id:self.id,case_id:self.case_id,value,unit:self.unit,body:self.body,recipe:self.recipe}
    }
    fn matches(&self,r:&ProductFinalRow<'_>,work:&mut ProductCertificateSpent<'_>)->Result<bool,ProductFailure> {
        work.visit()?;
        if self.unit!=r.unit || self.body!=r.body || self.recipe!=r.recipe{return Ok(false);}
        for (a,b) in [(self.id,r.id),(self.case_id,r.case_id)] {
            work.visit()?;if a.len()!=b.len(){return Ok(false);}
            for (x,y) in a.bytes().zip(b.bytes()) {work.visit()?;work.visit()?;if x!=y{return Ok(false);}}
        }
        Ok(true)
    }
}
#[derive(Debug)]
pub(super) struct ProofAnchor { owner:super::super::origins::ProductOwnerStamp }
impl ProofAnchor {pub(super) fn matches_owner(&self,owner:&adaptive::RetainedSolve)->bool {
    std::sync::Arc::ptr_eq(&self.owner.prep,&owner.prep)
}}
struct ProofData<'s,'m> {
    owner:&'s adaptive::RetainedSolve, facts:&'s [ProductMemberFacts], reduced:&'s ReducedSectionSet,
    specs:&'m [ProductRowSpec<'m>],
    k:source_residual::LaneReadouts, source:source_residual::LaneReadouts,
    anchor:std::sync::Arc<ProofAnchor>, values:Vec<f64>,
}
pub struct ProductProofStartSpent<'s,'m> {
    result:Result<ProofData<'s,'m>,ProductFailure>, work:ProductCertificateSpent<'static>,
}
pub struct ProductProofFailure { failure:ProductFailure, work:ProductCertificateSpent<'static> }
impl std::fmt::Debug for ProductProofFailure {
    fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result {
        f.debug_struct("ProductProofFailure").field("failure",&self.failure).field("work",&self.work).finish()
    }
}
impl ProductProofFailure {
    pub fn failure(&self)->&ProductFailure { &self.failure }
    pub fn work(&self)->&ProductCertificateSpent<'static> { &self.work }
    /// RV77-N4 (I61 U2, failure path): structural owner binding of a refused proof.
    pub fn owner_matches(&self,owner:&adaptive::RetainedSolve)->bool { self.work.owner_matches(owner) }
}
pub struct ProductProofDraft<'s,'m> { data:ProofData<'s,'m>, work:ProductCertificateSpent<'static> }
impl<'s,'m> ProductProofStartSpent<'s,'m> {
    pub fn into_ready(self)->Result<ProductProofDraft<'s,'m>,ProductProofFailure> {
        match self.result {Ok(data)=>Ok(ProductProofDraft{data,work:self.work}),
            Err(failure)=>Err(ProductProofFailure{failure,work:self.work})}
    }
}
pub(crate) fn begin_prepared_product<'s,'m>(invocation:&RecordedInvocation,run:usize,
    owner:&'s adaptive::RetainedSolve,facts:&'s [ProductMemberFacts],specs:&'m [ProductRowSpec<'m>],
    radius_law:source_residual::RadiusLaw)
    -> ProductProofStartSpent<'s,'m> {
    begin_prepared_product_reduced(invocation,run,owner,facts,ReducedSectionSet::empty(),specs,radius_law)
}
/// `begin_prepared_product` with O4 §4.5's reduced sections: each reduced
/// stress or maximum row is certified on its member's reduced section by
/// its nominal twin's recipe. An empty set is `begin_prepared_product`.
pub(crate) fn begin_prepared_product_reduced<'s,'m>(invocation:&RecordedInvocation,run:usize,
    owner:&'s adaptive::RetainedSolve,facts:&'s [ProductMemberFacts],reduced:&'s ReducedSectionSet,
    specs:&'m [ProductRowSpec<'m>],radius_law:source_residual::RadiusLaw)
    -> ProductProofStartSpent<'s,'m> {
    let mut work=ProductCertificateSpent::new(&[]);
    let result=(|| {
        work.visit()?;
        let stamp=invocation.product_owner_stamp(run,owner).ok_or_else(||bad("prepared recorded owner"))?;
        let source=owner.source();
        work.visit()?;
        work.prepared_capacities[4]=std::mem::size_of::<ProofAnchor>().checked_add(2*std::mem::size_of::<usize>())
            .ok_or(ProductFailure{cause:Cause::CountRange("proof anchor layout")})?;
        let anchor=std::sync::Arc::new(ProofAnchor{owner:stamp});
        work.anchor=Some(std::sync::Arc::clone(&anchor));
        if facts.len()!=source.members().len() || specs.is_empty() {return Err(bad("prepared facts/specs"));}
        let mut values=work.prepared_reserve(0,specs.len())?;
        for spec in specs {
            work.visit()?;
            let expected=match spec.recipe {
                ProductRecipe::Native(id)=>match owner.publish().rows[native_index(owner,id)?].kind {
                    Kind::Translation=>ProductUnit::Millimetre,Kind::Rotation=>ProductUnit::Radian,
                    Kind::Force=>ProductUnit::Newton,Kind::Moment=>ProductUnit::NewtonMetre},
                ProductRecipe::SupportComponent{component,..}=>if component.index()<3{ProductUnit::Newton}else{ProductUnit::NewtonMetre},
                ProductRecipe::Stress{..}=>ProductUnit::Megapascal,ProductRecipe::CircularMaximum{..}=>ProductUnit::Pascal,
                // `mechanical` admits a not-covered row with any quantity unit, never Record.
                ProductRecipe::NotCovered{..}=>spec.unit,
                _=>ProductUnit::Record,
            };
            if spec.unit!=expected {return Err(bad("prepared descriptor unit"));}
            values.push(spec.observed.map_or(0.0,f64::from_bits));
        }
        // Authenticate complete descriptor coverage before any residual or projection.
        let mut rows=work.prepared_reserve(1,specs.len())?;
        for (spec,value) in specs.iter().zip(&values) {work.visit()?;rows.push(spec.row(value));}
        let mut bind=std::mem::replace(&mut work,ProductCertificateSpent::new(&[])).rebind(&rows);
        let validity=row_scales_reduced(owner,facts,reduced,&mut bind).map(|_|());
        work=bind.rebind(&[]);
        validity?;
        drop(rows);
        let mut laws=reserve(facts.len())?;
        work.capacities[0]=laws.capacity();
        for (i,f) in facts.iter().enumerate() {
            work.visit()?;
            let m=&source.members()[i];
            let half=work.f64_op(Scalar64::Div,f.diameter,2.0)?;
            if f.member!=m.id || f.area.to_bits()!=m.area.to_bits()
                || f.second_moment.to_bits()!=m.second_moment_y.to_bits()
                || f.second_moment.to_bits()!=m.second_moment_z.to_bits()
                || f.torsion_constant.to_bits()!=m.torsion_constant.to_bits()
                || half.to_bits()!=f.radius.to_bits() {return Err(bad("prepared section facts"));}
            laws.push(bridge::ProposedMemberLaw{member:m,diameter:f.diameter,effective_wall:f.effective_wall,
                material:f.material.operands(),represented_z:f.section_modulus});
        }
        let kspent=source_residual::source_residual_for_law(owner,source,&owner.evidence().source_encoding,
            owner.selected_precision(),&laws,source_residual::ReadoutLaw::AdmittedK);
        let (kr,kw)=kspent.into_readouts(&anchor);let k=work.retain_lane(kr,kw)?;
        let sspent=source_residual::source_residual_prepared(owner,source,&owner.evidence().source_encoding,
            owner.selected_precision(),&laws,&anchor,&k,radius_law);
        let (sr,sw)=sspent.into_readouts(&anchor);let source_rows=work.retain_lane(sr,sw)?;
        if k.data!=source_rows.data || k.rows.len()!=owner.publish().rows.len()
            || source_rows.rows.len()!=k.rows.len() {return Err(bad("lane coverage/data"));}
        if let Some(f)=work.status().fault() {return Err(ProductFailure{cause:Cause::Accounting(f)});}
        work.visit()?;
        work.prepared_capacities[4]=std::mem::size_of::<ProofAnchor>().checked_add(2*std::mem::size_of::<usize>())
            .ok_or(ProductFailure{cause:Cause::CountRange("proof anchor layout")})?;
        Ok(ProofData{owner,facts,reduced,specs,k,source:source_rows,anchor,values})
    })();
    ProductProofStartSpent{result,work}
}
#[cfg(test)]
fn section_for(owner:&adaptive::RetainedSolve,facts:&[ProductMemberFacts],r:ProductRecipe,
    work:&mut ProductCertificateSpent<'_>)->Result<Option<MemberEnclosures>,ProductFailure> {
    section_for_reduced(owner,facts,r,None,work)
}
fn section_for_reduced(owner:&adaptive::RetainedSolve,facts:&[ProductMemberFacts],r:ProductRecipe,
    reduced:Option<&ReducedSection>,work:&mut ProductCertificateSpent<'_>)->Result<Option<MemberEnclosures>,ProductFailure> {
    let member=match r {ProductRecipe::Stress{member,..}|ProductRecipe::CircularMaximum{member}=>member,_=>return Ok(None)};
    let mut found=None;
    for (i,m) in owner.source().members().iter().enumerate() {work.visit()?;if m.id==member {found=Some(i);break;}}
    let i=found.ok_or_else(||bad("projection member"))?;
    let (m,f)=(&owner.source().members()[i],&facts[i]);
    Ok(Some(build_member(&section_operands(m,f,reduced),&mut work.numeric)?))
}
fn project_hull(interval:Enclosure,unit:ProductUnit,row:usize,work:&mut ProductCertificateSpent<'_>)->Result<f64,ProductFailure> {
    work.visit()?;
    let mut ctx=WideContext::<16>::new(1024).map_err(AttemptStop::from).map_err(NumericError::from)?;
    let result=(|| -> Result<f64,ProductFailure> {
        work.visit()?;
        let sum=ctx.add(&interval.lo,&interval.hi).map_err(AttemptStop::from).map_err(NumericError::from)?;
        work.visit()?;let midpoint=shift(&sum,-1)?;
        let raw=match unit {
            ProductUnit::Millimetre=>{work.visit()?;let scale=lift(1000.)?;
                ctx.mul(&midpoint,&scale).map_err(AttemptStop::from).map_err(NumericError::from)?},
            // begin_prepared_product admits MPa only for the typed Stress recipe.
            // DEF-O/E/C: convert its SI midpoint before binary64 MPa publication.
            ProductUnit::Megapascal=>midpoint,
            ProductUnit::Record=>return Err(bad("record projection")),_=>midpoint,
        };
        work.visit()?;
        if work.projection_outcomes.len()>=work.projection_outcomes.capacity(){return Err(bad("projection outcome capacity"));}
        work.projection_conversions=work.projection_conversions.add(WorkTotal::exact_count(1));
        if let Some(f)=work.status().fault(){return Err(ProductFailure{cause:Cause::Accounting(f)});}
        let outcome=raw.to_binary64();
        // Reserved and charged before entry; no fallible operation can lose the actual outcome.
        work.projection_outcomes.push((row,outcome));
        let value=match outcome {
            super::super::wide::multi::Binary64Outcome::Normal(y)|super::super::wide::multi::Binary64Outcome::Subnormal{value:y,..}=>if y==0.0 {0.0}else{y},
            super::super::wide::multi::Binary64Outcome::Underflow{..}=>0.0,
            _=>return Err(NumericError::Binary64Range.into()),
        };
        // Other units retain their existing raw conversion and trace word.
        let published=if unit==ProductUnit::Megapascal {
            work.f64_op(Scalar64::Div,value,1_000_000.)?
        } else {value};
        if !published.is_finite() {return Err(NumericError::Binary64Range.into());}
        Ok(if published==0.0 {0.0}else{published})
    })();
    work.numeric.wide.record(&ctx);
    result
}
pub(crate) struct LoadedValue { row:usize, member:u32, hull:Enclosure, reconciled:bool }
pub(crate) struct MemberValuesWork { hulls:Vec<LoadedValue>, numeric:NumericWork, failure:Option<ProductFailure> }
pub(crate) const LOADED_SEARCH_NODE:usize=std::mem::size_of::<loaded_maximum::SearchNode>();
pub(crate) const LOADED_SEARCH_HEAP_MAX:usize=loaded_maximum::LIVE*LOADED_SEARCH_NODE;
#[derive(Debug,Clone,Copy)]
pub struct ProductMaximumEvidence { value:f64, lower:f64, upper:f64, gap:f64 }
impl ProductMaximumEvidence {
    pub fn value(&self)->f64{self.value} pub fn lower(&self)->f64{self.lower}
    pub fn upper(&self)->f64{self.upper} pub fn gap(&self)->f64{self.gap}
}
pub struct ProductValuesBuilder {
    member:Option<Box<MemberValuesWork>>,
    values:Vec<f64>, maxima:Vec<(usize,u32)>, anchor:std::sync::Arc<ProofAnchor>, visits:WorkTotal,
    /// §4.1: not-covered rows still awaiting their PP value (their slots hold NaN).
    pending_not_covered:usize,
}
impl ProductValuesBuilder {
    pub fn value(&mut self,row:usize)->Result<Option<f64>,ProductFailure> {
        if let Some(f)=self.member.as_ref().and_then(|m|m.failure.as_ref()){return Err(f.clone());}
        for m in &self.maxima {values_visit(&mut self.visits)?;if m.0==row{return Ok(None);}}
        values_visit(&mut self.visits)?;
        // A not-covered row's slot is NaN until PP completes it: it has no projected value.
        Ok(self.values.get(row).copied().filter(|v|!v.is_nan()))
    }
    /// §4.1: PP's value of a not-covered row, formed after projection from the
    /// successor's own published rows. Each pending row exactly once, finite.
    pub fn complete_not_covered(&mut self,row:usize,value:f64)->Result<(),ProductFailure> {
        values_visit(&mut self.visits)?;
        if let Some(f)=self.member.as_ref().and_then(|m|m.failure.as_ref()){return Err(f.clone());}
        let slot=self.values.get_mut(row).ok_or_else(||bad("not-covered completion identity"))?;
        if !slot.is_nan() || self.pending_not_covered==0 {return Err(bad("not-covered completion identity"));}
        if !value.is_finite() {return Err(bad("not-covered completion value"));}
        values_visit(&mut self.visits)?;
        *slot=if value==0.0 {0.0} else {value};
        self.pending_not_covered-=1;
        Ok(())
    }
    pub fn abandon(self)->ValuesCompletionWork {ValuesCompletionWork{visits:self.visits,capacity:0,anchor:self.anchor,member:self.member}}
    pub fn reconcile_loaded_maximum(&mut self,row:usize,member:u32,pp_lower:f64,pp_upper:f64)->Result<ProductMaximumEvidence,ProductFailure> {
        values_visit(&mut self.visits)?;
        let state=self.member.as_mut().ok_or_else(||bad("loaded maximum owner"))?;
        if let Some(f)=&state.failure{return Err(f.clone());}
        let result=(|| {
            let entry=state.hulls.iter_mut().find(|h|h.row==row&&h.member==member).ok_or_else(||bad("loaded maximum identity"))?;
            if entry.reconciled{return Err(bad("loaded maximum repeated evidence"));}
            let evidence=loaded_maximum::evidence(&mut state.numeric,self.values[row],entry.hull,pp_lower,pp_upper,&mut self.visits)?;
            self.values[row]=evidence.value;entry.reconciled=true;Ok(evidence)
        })();
        if let Err(f)=&result{state.failure=Some(f.clone());} result
    }
    pub fn complete_maxima(mut self,maxima:&[ProductMaximumValue])->ProductValuesSpent {
        let mut visits=self.visits;let mut capacity=0;
        let result=(|| {
            values_visit(&mut visits)?;
            if let Some(state)=&self.member {
                if let Some(f)=&state.failure{return Err(f.clone());}
                for h in &state.hulls{values_visit(&mut visits)?;if !h.reconciled{return Err(bad("loaded maximum evidence incomplete"));}}
            }
            let mut seen=reserve::<bool>(self.maxima.len())?;capacity=seen.capacity();
            for _ in &self.maxima {values_visit(&mut visits)?;seen.push(false);}
            if maxima.len()!=self.maxima.len() {return Err(bad("maximum completion count"));}
            if self.pending_not_covered!=0 {return Err(bad("not-covered completion incomplete"));}
            for m in maxima {
                values_visit(&mut visits)?;
                let mut found=None;
                for (i,x) in self.maxima.iter().enumerate() {values_visit(&mut visits)?;if *x==(m.row,m.member){found=Some(i);break;}}
                let i=found.ok_or_else(||bad("maximum completion identity"))?;
                if seen[i] || !m.value.is_finite() || m.value<0.0 {return Err(bad("maximum completion value"));}
                values_visit(&mut visits)?;seen[i]=true;
                values_visit(&mut visits)?;self.values[m.row]=m.value;
            }
            if let Some(f)=visits.status().fault() {return Err(ProductFailure{cause:Cause::Accounting(f)});}
            Ok(())
        })();
        let result=result.and_then(|()|values_visit(&mut visits));
        match result {
            Err(e)=>ProductValuesSpent{result:Err(e),work:ValuesCompletionWork{visits,capacity,anchor:self.anchor,member:self.member}},
            Ok(())=>{
                let work=ValuesCompletionWork{visits,capacity,anchor:std::sync::Arc::clone(&self.anchor),member:self.member};
                ProductValuesSpent{result:Ok(FrozenProductValues{values:self.values,anchor:self.anchor}),work}
            }
        }
    }
}
fn merge_numeric(into:&mut NumericWork,other:NumericWork) {
    into.wide.merge(&other.wide);into.sums.merge(&other.sums);
    for i in 0..7{into.entries[i]=into.entries[i].add(other.entries[i]);}
    into.f64_arithmetic=into.f64_arithmetic.add(other.f64_arithmetic);into.status=into.status.join(other.status);
}
fn values_visit(visits:&mut WorkTotal)->Result<(),ProductFailure> {
    *visits=visits.add(WorkTotal::exact_count(1));
    visits.status().fault().map_or(Ok(()),|f|Err(ProductFailure{cause:Cause::Accounting(f)}))
}
pub struct ProductMaximumValue {member:u32,row:usize,value:f64}
impl ProductMaximumValue { pub fn new(member:u32,row:usize,value:f64)->Result<Self,ProductFailure> {
    if !value.is_finite() || value<0.0 {return Err(bad("maximum value"));} Ok(Self{member,row,value})
}}
pub struct FrozenProductValues {values:Vec<f64>,anchor:std::sync::Arc<ProofAnchor>}
impl FrozenProductValues {pub fn value(&self,row:usize)->Option<&f64>{self.values.get(row)} pub fn len(&self)->usize{self.values.len()}}
pub struct ValuesCompletionWork {visits:WorkTotal,capacity:usize,anchor:std::sync::Arc<ProofAnchor>,member:Option<Box<MemberValuesWork>>}
pub struct ProductValuesSpent {result:Result<FrozenProductValues,ProductFailure>,work:ValuesCompletionWork}
pub struct ProductValuesFailure {pub failure:ProductFailure, pub visits:WorkTotal,pub capacity:usize, work:Option<ValuesCompletionWork>}
impl ProductValuesFailure {
    /// Take the member-only helper owner exactly once on a refused completion.
    /// Nodal failures preserve the established attestation/trace behavior.
    pub fn take_work(&mut self)->Option<ValuesCompletionWork>{self.work.take()}
}
impl std::fmt::Debug for ProductValuesFailure {fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result {
    f.debug_struct("ProductValuesFailure").field("failure",&self.failure).field("visits",&self.visits).field("capacity",&self.capacity).finish()
}}
impl ProductValuesSpent {pub fn into_ready(self)->Result<(FrozenProductValues,ValuesCompletionWork),ProductValuesFailure> {
    match self.result {Ok(v)=>Ok((v,self.work)),Err(failure)=>{let visits=self.work.visits;let capacity=self.work.capacity;
            let work=if self.work.member.is_some(){Some(self.work)}else{None};
            Err(ProductValuesFailure{failure,visits,capacity,work})}}
}}
pub struct ProjectedProofDraft<'s,'m> {data:ProofData<'s,'m>,work:ProductCertificateSpent<'static>}
pub struct ProductProjectionSpent<'s,'m> {result:Result<(ProofData<'s,'m>,ProductValuesBuilder),ProductFailure>,work:ProductCertificateSpent<'static>}
impl<'s,'m> ProductProjectionSpent<'s,'m> {
    pub fn into_ready(self)->Result<(ProjectedProofDraft<'s,'m>,ProductValuesBuilder),ProductProofFailure> {
        match self.result {Ok((data,values))=>Ok((ProjectedProofDraft{data,work:self.work},values)),
            Err(failure)=>Err(ProductProofFailure{failure,work:self.work})}
    }
}
impl<'s,'m> ProductProofDraft<'s,'m> {
    pub fn lane_debug(&self)->impl std::fmt::Debug+'_ {(&self.data.k,&self.data.source)}
    pub fn project(mut self)->ProductProjectionSpent<'s,'m> {
        let combination=!self.data.owner.prep.factors.is_empty();
        #[cfg(feature = "census-probe")]
        census_probe::context(self.data.specs.first().map_or("", |s| s.case_id), "projection");
        let result=(|| {
            let mut maxima=self.work.prepared_reserve(2,self.data.facts.len())?;
            let mut loaded=if self.data.owner.member_loads().iter().any(|l|l.intensity.iter().any(|w|*w!=0.0)) {
                reserve::<LoadedValue>(self.data.facts.len())? } else {Vec::new()};
            self.work.projection_outcomes=self.work.prepared_reserve(5,self.data.specs.len())?;
            let reduced_map=reduced_index(self.data.owner,self.data.facts,self.data.reduced,self.data.specs.len(),&mut self.work)?;
            let mut not_covered=0usize;
            for (i,spec) in self.data.specs.iter().enumerate() {
                self.work.visit()?;
                if let Some(bits)=spec.observed {self.work.visit()?;self.data.values[i]=f64::from_bits(bits);continue;}
                // §4.1: no projection; PP completes the value after projection.
                if let ProductRecipe::NotCovered{..}=spec.recipe {self.work.visit()?;self.data.values[i]=f64::NAN;not_covered+=1;continue;}
                if let ProductRecipe::CircularMaximum{member}=spec.recipe {
                    if !self.data.owner.member_is_loaded(member){self.work.visit()?;maxima.push((i,member));continue;}
                }
                if matches!(spec.recipe,ProductRecipe::Native(QuantityId::SupportForceMagnitude(_)|QuantityId::SupportMomentMagnitude(_))){continue;}
                // B2-K (DEF-C r2, A-1): a combination's displacement magnitude is
                // formed from its frozen components in the second pass.
                if combination && matches!(spec.recipe,ProductRecipe::Native(QuantityId::DisplacementMagnitude(_))){continue;}
                let row_reduced=reduced_of(self.data.reduced,&reduced_map,i);
                let section=section_for_reduced(self.data.owner,self.data.facts,spec.recipe,row_reduced,&mut self.work)?;
                let k=recipe_reduced(self.data.owner,&mut self.work,&self.data.k.rows,spec.recipe,section.as_ref(),self.data.facts,row_reduced,true,&self.data.k.curvatures,i)?;
                let s=recipe_reduced(self.data.owner,&mut self.work,&self.data.source.rows,spec.recipe,section.as_ref(),self.data.facts,row_reduced,false,&self.data.source.curvatures,i)?;
                self.work.visit()?;
                let interval=hull(k,s);
                let value=project_hull(interval,spec.unit,i,&mut self.work)?;
                if let ProductRecipe::CircularMaximum{member}=spec.recipe {loaded.push(LoadedValue{row:i,member,hull:interval,reconciled:false});}
                self.work.visit()?;self.data.values[i]=value;
            }
            for (i,spec) in self.data.specs.iter().enumerate() {
                let (support,first,unit)=match spec.recipe {
                    ProductRecipe::Native(QuantityId::SupportForceMagnitude(s))=>(s,0,ProductUnit::Newton),
                    ProductRecipe::Native(QuantityId::SupportMomentMagnitude(s))=>(s,3,ProductUnit::NewtonMetre),_=>continue,
                };
                let value=support_hypot(self.data.specs,&self.data.values,support,first,unit,&mut self.work)?;
                self.work.visit()?;self.data.values[i]=value;
            }
            if combination {
                for (i,spec) in self.data.specs.iter().enumerate() {
                    let ProductRecipe::Native(QuantityId::DisplacementMagnitude(node))=spec.recipe else {continue};
                    let value=displacement_norm(self.data.specs,&self.data.values,node,&mut self.work)?;
                    self.work.visit()?;self.data.values[i]=value;
                }
            }
            self.work.visit()?;
            let member=(!loaded.is_empty()).then(||Box::new(MemberValuesWork{hulls:loaded,numeric:NumericWork::new(),failure:None}));
            let values=ProductValuesBuilder{member,values:std::mem::take(&mut self.data.values),maxima,
                anchor:std::sync::Arc::clone(&self.data.anchor),visits:WorkTotal::zero(),pending_not_covered:not_covered};
            Ok(values)
        })();
        ProductProjectionSpent{result:result.map(|v|(self.data,v)),work:self.work}
    }
}
fn support_hypot(specs:&[ProductRowSpec<'_>],values:&[f64],support:u32,first:usize,unit:ProductUnit,
    work:&mut ProductCertificateSpent<'_>)->Result<f64,ProductFailure> {
    let mut v=[0.0;3];
    if specs.len()!=values.len(){return Err(bad("support projection shape"));}
    for j in 0..3 {
        let mut found=false;
        for (i,spec) in specs.iter().enumerate() {
            work.visit()?;
            if spec.recipe!=(ProductRecipe::SupportComponent{support,component:Component::ALL[first+j]}){continue;}
            if found || spec.unit!=unit || !values[i].is_finite(){return Err(bad("support projection identity/unit"));}
            work.visit()?;v[j]=values[i];found=true;
        }
        if !found{return Err(bad("support projection missing component"));}
    }
    work.visit()?;work.scalar_operations=work.scalar_operations.add(WorkTotal::exact_count(1));
    if let Some(f)=work.status().fault(){return Err(ProductFailure{cause:Cause::Accounting(f)});}
    work.visit()?;work.scalar_operations=work.scalar_operations.add(WorkTotal::exact_count(1));
    if let Some(f)=work.status().fault(){return Err(ProductFailure{cause:Cause::Accounting(f)});}
    // I109: one correctly rounded 3-norm (platform-independent), still charged as the two
    // scalar operations of the former `hypot` chain.
    let value=open_pipe_stress_correct_norm::norm3(v[0],v[1],v[2]);
    if !value.is_finite() || value<0.0{return Err(bad("support projection hypot range"));}
    Ok(if value==0.0{0.0}else{value})
}
/// B2-K (DEF-C r2 `rows.displacement_magnitude`; REVISION_02 §2.2 as RR's
/// SA4-1 amends it): a combination's displacement magnitude is RN64, ties to
/// even, of the exact 3-norm of its node's frozen projected translations (mm).
/// The components are found as `support_hypot` finds its slots: exactly one
/// each, unit mm, finite; counted visits and one counted scalar operation.
fn displacement_norm(specs:&[ProductRowSpec<'_>],values:&[f64],node:u32,
    work:&mut ProductCertificateSpent<'_>)->Result<f64,ProductFailure> {
    let mut v=[0.0;3];
    if specs.len()!=values.len(){return Err(bad("displacement norm shape"));}
    for j in 0..3 {
        let recipe=ProductRecipe::Native(QuantityId::Displacement(Dof{node,component:Component::ALL[j]}));
        let mut found=false;
        for (i,spec) in specs.iter().enumerate() {
            work.visit()?;
            if spec.recipe!=recipe{continue;}
            if found || spec.unit!=ProductUnit::Millimetre || !values[i].is_finite(){return Err(bad("displacement norm identity/unit"));}
            work.visit()?;v[j]=values[i];found=true;
        }
        if !found{return Err(bad("displacement norm missing component"));}
    }
    work.visit()?;work.scalar_operations=work.scalar_operations.add(WorkTotal::exact_count(1));
    if let Some(f)=work.status().fault(){return Err(ProductFailure{cause:Cause::Accounting(f)});}
    let spent=rn64_norm3(v);
    work.numeric.wide.merge(&spent.wide);work.numeric.sums.merge(&spent.sums);
    let value=spent.result.map_err(|e|ProductFailure{cause:Cause::Helper(e)})?;
    if let Some(f)=work.status().fault(){return Err(ProductFailure{cause:Cause::Accounting(f)});}
    Ok(value)
}
/// The exact 3-norm's spent result: every context and sum's work is kept,
/// failed prefixes included.
pub(super) struct NormSpent {
    pub(super) result:Result<f64,directed::certificate::HelperError>,
    pub(super) wide:super::super::wide::multi::AttemptWork, pub(super) sums:SumWork,
}
/// RN64, ties to even, of sqrt(x^2 + y^2 + z^2) for finite binary64 x, y, z,
/// decided exactly: S formed exactly; a 1024-bit nearest sqrt of RN1024(S)
/// rounded once to binary64 as the estimate y0; then the exact sides
/// sign(S - m^2) at y0's two midpoints. SA4-1: (a) the midpoints come from y0's
/// actual binary64 neighbours, averaged exactly (at 2^-1022 both gaps are
/// 2^-1074); (b) at y0 = MAX the upper midpoint is MAX + 2^970 and S at or
/// above its square refuses; (c) an overflowing estimate is decided by that
/// same comparison from y0 = MAX; (d) at most one correction step, a second is
/// an invariant failure; (e) S > 0 with a zero or underflowing estimate is an
/// invariant failure. S = 0 gives canonical +0.
pub(super) fn rn64_norm3(v:[f64;3])->NormSpent {
    use directed::certificate::HelperError;
    let mut sums=SumWork::default();
    let mut wide=super::super::wide::multi::AttemptWork::default();
    let mut ctx=match WideContext::<16>::new(1024) {
        Ok(c)=>c, Err(e)=>return NormSpent{result:Err(HelperError::Arithmetic(e.into())),wide,sums},
    };
    let result=norm3_core(v,&mut ctx,&mut sums);
    wide.record(&ctx);
    NormSpent{result,wide,sums}
}
const MAX_BITS:u64=0x7fef_ffff_ffff_ffff;
fn norm3_core(v:[f64;3],ctx:&mut WideContext<16>,sums:&mut SumWork)
    ->Result<f64,directed::certificate::HelperError> {
    use directed::certificate::HelperError as H;
    let arith=|e:AttemptStop|H::Arithmetic(e);
    let mut x=[Endpoint::ZERO;3];
    for (k,c) in v.iter().enumerate() {
        x[k]=lift(*c).map_err(|e|match e {NumericError::Arithmetic(e)=>H::Arithmetic(e),_=>H::Invariant})?;
    }
    // S exactly, its sign, and the nearest 1024-bit sqrt of its 1024-bit rounding.
    let mut s=ExactWideSum::new();
    let estimate=(|| -> Result<Option<Endpoint>,AttemptStop> {
        for c in &x {s.add_product(ctx,c,c,false)?;}
        if s.signum()?==0 {return Ok(None);}
        let rounded=s.round(ctx)?;
        Ok(Some(ctx.sqrt(&rounded)?))
    })();
    sums.merge(&s.work());
    let Some(q)=estimate.map_err(arith)? else {return Ok(0.0)};
    use super::super::wide::multi::Binary64Outcome as B;
    let mut y=match q.to_binary64() {
        B::Normal(y)|B::Subnormal{value:y,..} if y>0.0 =>y.to_bits(),
        B::Overflow{negative:false}=>MAX_BITS,
        _=>return Err(H::Invariant),
    };
    // The exact value of a positive binary64 bit pattern; +inf's pattern is 2^1024.
    let at=|bits:u64|->Result<Endpoint,H> {
        if bits==0x7ff0_0000_0000_0000 {shift(&Endpoint::ONE,1024).map_err(|_|H::Invariant)}
        else {lift(f64::from_bits(bits)).map_err(|_|H::Invariant)}
    };
    for step in 0..2 {
        let here=at(y)?;
        let mut side=|neighbour:u64|->Result<i8,H> {
            let m=shift(&ctx.add(&here,&at(neighbour)?).map_err(|e|arith(e.into()))?,-1).map_err(|_|H::Invariant)?;
            let mut d=ExactWideSum::new();
            let sign=(|| -> Result<i8,AttemptStop> {
                for c in &x {d.add_product(ctx,c,c,false)?;}
                d.add_product(ctx,&m,&m,true)?;
                Ok(d.signum()?)
            })();
            sums.merge(&d.work());
            sign.map_err(arith)
        };
        let odd=y&1==1;
        let upper=side(y+1)?;
        let lower=side(y-1)?;
        let up=upper>0 || (upper==0 && odd);
        let down=lower<0 || (lower==0 && odd);
        if !up && !down {return Ok(f64::from_bits(y));}
        if step==1 || (up && down) {return Err(H::Invariant);}
        if up && y==MAX_BITS {return Err(H::Binary64Range);}
        if down && y==1 {return Err(H::Invariant);}
        // One step to the actual binary64 neighbour (bit patterns are ordered).
        let next=if up {y.checked_add(1)} else {y.checked_sub(1)};
        y=next.ok_or(H::Invariant)?;
    }
    Err(H::Invariant)
}
pub struct CertifiedProductProof {
    work:ProductCertificateSpent<'static>, anchor:std::sync::Arc<ProofAnchor>,
    /// W1 coverage repair §2.2: the per-body coupled, floored scales the gate used
    /// (translation, rotation, force, moment), which the receipt states. Held here,
    /// not in the work owner, whose layout is priced.
    final_scales:Vec<[f64;4]>,
}
impl CertifiedProductProof {
    /// The per-body scales the product gate judged every row with (W1 coverage repair §2.2).
    pub fn final_scales(&self)->&[[f64;4]] {&self.final_scales}
    pub fn passed(&self)->bool {self.work.failure.is_none() && self.work.status().is_exact() && self.work.verdicts.iter().all(|v|v.passed)}
    pub fn verdicts(&self)->&[ProductRowVerdict] {&self.work.verdicts}
    pub fn summary_coverage(&self)->&[ProductSummaryCoverage] {&self.work.coverage}
    pub fn work(&self)->&ProductCertificateSpent<'static> {&self.work}
    pub fn matches_values(&self,values:&FrozenProductValues)->bool {std::sync::Arc::ptr_eq(&self.anchor,&values.anchor)}
    /// RV77-N4 (I61 U2): whether this proof was started on `owner`'s own prepared
    /// solve. Structural owner binding through the proof anchor, not custody.
    pub fn owner_matches(&self,owner:&adaptive::RetainedSolve)->bool {self.anchor.matches_owner(owner)}
}
pub struct ProductFinalSpent {result:Result<CertifiedProductProof,ProductProofFailure>}
impl ProductFinalSpent {pub fn into_ready(self)->Result<CertifiedProductProof,ProductProofFailure>{self.result}}
impl<'s,'m> ProjectedProofDraft<'s,'m> {
    pub fn abandon(self)->ProductProofFailure {ProductProofFailure{failure:bad("PP abandoned prepared draft"),work:self.work}}
    pub fn abandon_values(mut self,values_work:ValuesCompletionWork)->ProductProofFailure {
        self.work.completion_merged=true;self.work.trace_copy_work.record::<bool>();
        self.work.visits=self.work.visits.add(values_work.visits);
        self.work.prepared_capacities[3]=values_work.capacity;
        if let Some(member)=values_work.member {merge_numeric(&mut self.work.numeric,member.numeric);}
        ProductProofFailure{failure:bad("PP abandoned completed values"),work:self.work}
    }
    pub fn certify_final(self,values:&FrozenProductValues,rows:&[ProductFinalRow<'_>],values_work:ValuesCompletionWork)->ProductFinalSpent {
        let mut work=self.work.rebind(rows);
        work.completion_merged=true;work.trace_copy_work.record::<bool>();
        work.visits=work.visits.add(values_work.visits);
        work.prepared_capacities[3]=values_work.capacity;
        if let Some(member)=values_work.member {merge_numeric(&mut work.numeric,member.numeric);}
        let result=(|| {
            if !std::sync::Arc::ptr_eq(&self.data.anchor,&values.anchor)
                || !std::sync::Arc::ptr_eq(&self.data.anchor,&values_work.anchor)
                || values.values.len()!=self.data.specs.len() || rows.len()!=values.values.len() {
                return Err(bad("frozen projection owner/shape"));
            }
            for ((spec,r),y) in self.data.specs.iter().zip(rows).zip(&values.values) {
                work.visit()?;
                if !spec.matches(r,&mut work)? || r.value.to_bits()!=y.to_bits()
                    || spec.observed.is_some_and(|b|b!=y.to_bits()) {return Err(bad("frozen descriptor/value"));}
            }
            let scales=check_intervals(self.data.owner,self.data.facts,self.data.reduced,&self.data.k.rows,&self.data.source.rows,&self.data.k.data,&mut work,&self.data.k.curvatures,&self.data.source.curvatures)?;
            if let Some(v)=work.verdicts.iter().find(|v|!v.passed) {
                return Err(ProductFailure{cause:Cause::Predicate{row:v.row,predicate:v.failed.unwrap_or(ProductPredicate::SharperExact)}});
            }
            if let Some(f)=work.status().fault(){return Err(ProductFailure{cause:Cause::Accounting(f)});}
            Ok(scales)
        })();
        let mut owned=work.rebind(&[]);
        let result=match result {Ok(final_scales)=>Ok(CertifiedProductProof{work:owned,anchor:self.data.anchor,final_scales}),
            Err(failure)=>{owned.failure=Some(failure.clone());Err(ProductProofFailure{failure,work:owned})}};
        ProductFinalSpent{result}
    }
}
