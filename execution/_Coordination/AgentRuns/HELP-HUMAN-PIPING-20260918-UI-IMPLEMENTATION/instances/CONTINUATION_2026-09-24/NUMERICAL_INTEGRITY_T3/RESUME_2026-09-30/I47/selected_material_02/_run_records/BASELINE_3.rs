//! Fixed ordinary product recipes. Endpoints and radius authority stay here.
use super::super::{
    adaptive,
    origins::RecordedInvocation,
    recover::{End, Kind, QuantityId},
    source::Component,
};
use super::source_residual::ResidualWork;
use super::*;

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
}
impl ProductMaterial {
    fn operands(self) -> MaterialOperands {
        match self {
            Self::Base { e, g } | Self::Point { e, g, .. } => MaterialOperands::Ordinary { e, g },
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
    Stress {
        member: u32,
        site: ProductSite,
        stress: ProductStress,
    },
    CircularMaximum {
        member: u32,
    },
    NonQuantity,
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
    Helper(directed::certificate::HelperError),
}
#[derive(Debug, Clone)]
pub struct ProductFailure {
    cause: Cause,
}
impl ProductFailure {
    pub fn category(&self) -> &'static str {
        match self.cause {
            Cause::Association(_) => "association",
            Cause::Numeric(_) => "numeric",
            Cause::Native(_) => "native_source",
            Cause::Accounting(_) => "work_accounting",
            Cause::CountRange(_) => "count_range",
            Cause::Storage => "storage",
            Cause::G5a(_) => "g5a",
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
pub struct ProductCertificateSpent<'a> {
    rows: &'a [ProductFinalRow<'a>],
    verdicts: Vec<ProductRowVerdict>,
    failure: Option<ProductFailure>,
    coverage: Vec<ProductSummaryCoverage>,
    native: Option<ResidualWork>,
    numeric: NumericWork,
    comparisons: SumWork,
    visits: WorkTotal,
    scalar_operations: WorkTotal,
    pub capacities: [usize; 4],
}
impl<'a> ProductCertificateSpent<'a> {
    fn new(rows: &'a [ProductFinalRow<'a>]) -> Self {
        Self {
            rows,
            verdicts: Vec::new(),
            failure: None,
            coverage: Vec::new(),
            native: None,
            numeric: NumericWork::new(),
            comparisons: SumWork::default(),
            visits: WorkTotal::zero(),
            scalar_operations: WorkTotal::zero(),
            capacities: [0; 4],
        }
    }
    pub fn status(&self) -> WorkStatus {
        let mut s = self
            .numeric
            .status()
            .join(self.comparisons.checked_lme().status())
            .join(self.visits.status())
            .join(self.scalar_operations.status());
        if let Some(w) = &self.native {
            s = s.join(w.status());
        }
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
        self.native.as_ref().map(|w| w.correction.calls)
    }
    pub fn native_work(&self) -> Option<&impl std::fmt::Debug> {
        self.native.as_ref()
    }
    pub fn work_summary(&self) -> impl std::fmt::Debug + '_ {
        (
            &self.numeric,
            &self.comparisons,
            self.visits,
            self.scalar_operations,
        )
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
fn recipe(
    owner: &adaptive::RetainedSolve,
    w: &mut NumericWork,
    values: &[Enclosure],
    r: ProductRecipe,
    section: Option<&MemberEnclosures>,
    facts: &[ProductMemberFacts],
    represented: bool,
) -> Result<Enclosure, ProductFailure> {
    if let ProductRecipe::Native(id) = r {
        return Ok(values[native_index(owner, id)?]);
    }
    let member = match r {
        ProductRecipe::Stress { member, .. } | ProductRecipe::CircularMaximum { member } => member,
        _ => return Err(bad("nonquantity recipe")),
    };
    let i = owner
        .source()
        .member_index(member)
        .ok_or_else(|| bad("member"))?;
    let (sec, f) = (section.ok_or_else(|| bad("section recipe"))?, &facts[i]);
    let (a, z, j, c) = if represented {
        (
            Enclosure::point(lift(f.area)?),
            sec.represented_z.ok_or_else(|| bad("represented Z"))?,
            Enclosure::point(lift(f.torsion_constant)?),
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
    let mut spent = ProductCertificateSpent::new(rows);
    let result = run_case(invocation, run, owner, facts, &mut spent);
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
    spent: &mut ProductCertificateSpent<'_>,
) -> Result<(), ProductFailure> {
    if !invocation.product_case_owner(run, owner) {
        return Err(bad("recorded owner"));
    }
    let source = owner.source();
    if facts.len() != source.members().len() {
        return Err(bad("member count"));
    }
    let mut laws = reserve(facts.len())?;
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
    let result = (|| {
        let native = residual.result().map_err(|e| ProductFailure {
            cause: Cause::Native(e),
        })?;
        let mut k = reserve(owner.publish().rows.len())?;
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
        spent.coverage = summary_coverage(owner, &native.view, spent)?;
        let nb = source.body_count() as usize;
        let mut scales = reserve(nb)?;
        scales.resize(nb, [0.0f64; 4]);
        let mut native_coverage = reserve(k.len())?;
        native_coverage.resize(k.len(), false);
        let derivative_count = facts.len().checked_mul(21).ok_or_else(|| ProductFailure {
            cause: Cause::CountRange("derivative rows"),
        })?;
        let mut derivative_coverage = reserve(derivative_count)?;
        derivative_coverage.resize(derivative_count, false);
        let mut nonquantity = false;
        spent.verdicts = reserve(spent.rows.len())?;
        spent.capacities = [
            laws.capacity(),
            native_coverage.capacity(),
            k.capacity(),
            spent.verdicts.capacity(),
        ];
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
            match r.recipe {
                ProductRecipe::NonQuantity => {
                    if nonquantity || r.unit != ProductUnit::Record {
                        return Err(bad("nonquantity coverage"));
                    }
                    nonquantity = true;
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
                    let j = mi * 21 + site_index * 4 + component;
                    if derivative_coverage[j] || r.unit != ProductUnit::Megapascal {
                        return Err(bad("stress coverage"));
                    }
                    derivative_coverage[j] = true;
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
                    let j = mi * 21 + 20;
                    if derivative_coverage[j] {
                        return Err(bad("maximum coverage"));
                    }
                    derivative_coverage[j] = true;
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
        if native_coverage.iter().any(|v| !*v)
            || derivative_coverage.iter().any(|v| !*v)
            || !nonquantity
        {
            return Err(bad("missing final coverage"));
        }
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
            if recipe_id == ProductRecipe::NonQuantity {
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
                    let sc = scales[r.body as usize];
                    let axial = spent.f64_op(Scalar64::Div, sc[2], f.area)?;
                    let bending = spent.f64_op(Scalar64::Div, sc[3], f.section_modulus)?;
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
                    Some(build_member(
                        &MemberOperands {
                            diameter: f.diameter,
                            effective_wall: f.effective_wall,
                            material: f.material.operands(),
                            admitted: AdmittedOperands {
                                e: m.elastic_modulus,
                                g: m.shear_modulus,
                                a: m.area,
                                j: m.torsion_constant,
                                iy: m.second_moment_y,
                                iz: m.second_moment_z,
                                z_hat: f.section_modulus,
                            },
                        },
                        &mut spent.numeric,
                    )?)
                }
                _ => None,
            };
            let represented = recipe(
                owner,
                &mut spent.numeric,
                &k,
                recipe_id,
                section.as_ref(),
                facts,
                true,
            )?;
            let geometric = recipe(
                owner,
                &mut spent.numeric,
                &native.rows,
                recipe_id,
                section.as_ref(),
                facts,
                false,
            )?;
            let v = gate(spent, i, hull(represented, geometric), scale, input)?;
            spent.verdicts.push(v);
        }
        Ok(())
    })();
    spent.native = Some(residual.work);
    result
}
#[cfg(test)]
#[path = "../../../../tests/retained_k4/product_final_case_tests.rs"]
mod product_final_case_tests;

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
        let has_data = view
            .data()
            .iter()
            .enumerate()
            .any(|(i, data)| *data && view.group().blocks.body[i] == body);
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
