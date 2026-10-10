//! Conditional exact-source enclosure about an arbitrary private finite center.
//! One owner-bound verification-factor application; no publication replacement,
//! PP provenance, resource admission, retry, or factor accuracy assumption.
use super::super::adaptive::{
    RetainedSolve, SourceBridgeView, SourceBridgeViewWork, SourceCorrectionError,
    SourceCorrectionWork,
};
use super::super::recover::{End, QuantityId};
use super::super::source::{Component, Dof, PrimitiveSource, StraightMember};
use super::bridge::{BridgeError, ProposedMemberLaw};
use super::*;

type Error = BridgeError;
const ZERO: Enclosure = Enclosure {
    lo: Endpoint::ZERO,
    hi: Endpoint::ZERO,
};
const ONE: Enclosure = Enclosure {
    lo: Endpoint::ONE,
    hi: Endpoint::ONE,
};
fn neg(x: Enclosure) -> Enclosure {
    Enclosure {
        lo: x.hi.neg(),
        hi: x.lo.neg(),
    }
}
fn point(x: f64) -> Result<Enclosure, Error> {
    Ok(Enclosure::point(lift(x)?))
}
fn buffer<T: Clone>(n: usize, value: T) -> Result<Vec<T>, Error> {
    std::alloc::Layout::array::<T>(n).map_err(|_| Error::CountRange)?;
    let mut v = Vec::new();
    v.try_reserve_exact(n).map_err(|_| Error::Storage)?;
    v.resize(n, value);
    Ok(v)
}
fn count_add(a: usize, b: usize) -> Result<usize, Error> {
    a.checked_add(b).ok_or(Error::CountRange)
}
fn count_mul(a: usize, b: usize) -> Result<usize, Error> {
    a.checked_mul(b).ok_or(Error::CountRange)
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReadoutLaw { AdmittedK, AnnularSource }

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum ViewFailure {
    Certificate(super::super::adaptive::CertificateIssue),ForeignOwner,UnsupportedCombination,
    VerificationCache,Ordering,BodyBound,CountRange,Work(super::super::work::WorkFault),
}
#[derive(Debug)]
pub enum BridgeFailure<'a> {
    View(ViewFailure),Numeric(&'a NumericError),MemberOwner,UnsupportedDirectionalSpring,
    MissingRadius(usize),MissingUniquenessWarrant(u32),RowIdentity(usize),CountRange,Storage,
    AlphaCondition{block:usize},
}
pub(crate) fn bridge_failure(error:&Error)->BridgeFailure<'_> {
    use super::super::adaptive::SourceBridgeViewIssue as V;
    match error {
        Error::View(v)=>BridgeFailure::View(match v {
            V::Certificate(c)=>ViewFailure::Certificate(*c),V::ForeignOwner=>ViewFailure::ForeignOwner,
            V::UnsupportedCombination=>ViewFailure::UnsupportedCombination,V::VerificationCache=>ViewFailure::VerificationCache,
            V::Ordering=>ViewFailure::Ordering,V::BodyBound=>ViewFailure::BodyBound,V::CountRange=>ViewFailure::CountRange,
            V::Work(f)=>ViewFailure::Work(*f)}),
        Error::Numeric(e)=>BridgeFailure::Numeric(e),Error::MemberOwner=>BridgeFailure::MemberOwner,
        Error::UnsupportedDirectionalSpring=>BridgeFailure::UnsupportedDirectionalSpring,
        Error::MissingRadius(n)=>BridgeFailure::MissingRadius(*n),Error::MissingUniquenessWarrant(n)=>BridgeFailure::MissingUniquenessWarrant(*n),
        Error::RowIdentity(n)=>BridgeFailure::RowIdentity(*n),Error::CountRange=>BridgeFailure::CountRange,
        Error::Storage=>BridgeFailure::Storage,Error::Alpha{block,..}=>BridgeFailure::AlphaCondition{block:*block},
    }
}
#[derive(Debug)]
pub struct LaneWorkTrace<'a> {
    pub numeric:NumericTrace,pub point_lme:WorkTotal,
    pub view_visits:WorkTotal,pub view_f64_operations:WorkTotal,pub prescribed_capacity:usize,pub view_data_capacity:usize,
    pub correction_cast_lme:WorkTotal,pub correction_factor_lme:WorkTotal,pub correction_visits:WorkTotal,pub correction_calls:WorkTotal,
    pub correction_capacities:[usize;3],pub visits:WorkTotal,pub member_builds:WorkTotal,pub frame_builds:WorkTotal,
    pub b_products:WorkTotal,pub d_products:WorkTotal,pub h_products:WorkTotal,
    pub capacities:&'a [(&'static str,usize)],pub data_capacity:usize,pub status:WorkStatus,
}
#[derive(Debug)]
pub(crate) struct ResidualWork {
    pub(crate) readout_law: ReadoutLaw,
    pub(crate) numeric: NumericWork,
    pub(crate) point: AttemptWork,
    pub(crate) view: SourceBridgeViewWork,
    pub(crate) correction: SourceCorrectionWork,
    /// Named entered loop events only, not a complete facade visit ledger.
    /// Library searches, checks, initialization and scalar bit operations remain
    /// explicit auxiliary work; no selected price makes those operations free.
    pub(crate) visits: WorkTotal,
    pub(crate) member_builds: WorkTotal,
    pub(crate) frame_builds: WorkTotal,
    pub(crate) b_products: WorkTotal,
    pub(crate) d_products: WorkTotal,
    pub(crate) h_products: WorkTotal,
    /// Actual vector capacities, recorded by owner. No total byte/profile claim.
    pub(crate) capacities: [(&'static str, usize); 9],
    capacity_entries: usize,
    pub(crate) data_capacity: usize,
}
impl ResidualWork {
    pub(crate) fn typed_trace<'a>(&'a self,copies:&mut TraceCopyWork)->LaneWorkTrace<'a> {
        copies.record::<LaneWorkTrace<'_>>();
        LaneWorkTrace {numeric:self.numeric.trace(copies),point_lme:self.point.checked_lme(),
            view_visits:self.view.visits,view_f64_operations:self.view.f64_operations,
            prescribed_capacity:self.view.prescribed_capacity,view_data_capacity:self.view.data_capacity,
            correction_cast_lme:self.correction.cast.checked_lme(),correction_factor_lme:self.correction.factor.checked_lme(),
            correction_visits:self.correction.visits,correction_calls:self.correction.calls,
            correction_capacities:[self.correction.rhs_capacity,self.correction.output_capacity,self.correction.converted_capacity],
            visits:self.visits,member_builds:self.member_builds,frame_builds:self.frame_builds,b_products:self.b_products,
            d_products:self.d_products,h_products:self.h_products,capacities:&self.capacities[..self.capacity_entries],
            data_capacity:self.data_capacity,status:self.status()}
    }
    fn new() -> Self {
        Self {
            readout_law: ReadoutLaw::AnnularSource,
            numeric: NumericWork::new(),
            point: AttemptWork::default(),
            view: SourceBridgeViewWork::default(),
            correction: SourceCorrectionWork::default(),
            visits: WorkTotal::zero(),
            member_builds: WorkTotal::zero(),
            frame_builds: WorkTotal::zero(),
            b_products: WorkTotal::zero(),
            d_products: WorkTotal::zero(),
            h_products: WorkTotal::zero(),
            capacities: [("", 0); 9],
            capacity_entries: 0,
            data_capacity: 0,
        }
    }
    pub(crate) fn status(&self) -> WorkStatus {
        self.numeric
            .status()
            .join(self.point.checked_lme().status())
            .join(self.correction.status())
            .join(self.view.visits.status())
            .join(self.view.f64_operations.status())
            .join(self.visits.status())
            .join(self.member_builds.status())
            .join(self.frame_builds.status())
            .join(self.b_products.status())
            .join(self.d_products.status())
            .join(self.h_products.status())
    }
    fn capacity(&mut self, name: &'static str, n: usize) -> Result<(), Error> {
        let at = self.capacity_entries;
        if at >= self.capacities.len() {
            return Err(Error::CountRange);
        }
        self.capacities[at] = (name, n);
        self.capacity_entries = count_add(at, 1)?;
        Ok(())
    }
    fn check(&self) -> Result<(), Error> {
        self.status().fault().map_or(Ok(()), |f| {
            Err(NumericError::Arithmetic(AttemptStop::WorkAccounting(f)).into())
        })
    }
    fn visit(&mut self) -> Result<(), Error> {
        self.visits = self.visits.add(WorkTotal::exact_count(1));
        self.check()
    }
    fn add(&mut self, a: Enclosure, b: Enclosure) -> Result<Enclosure, Error> {
        Ok(self.numeric.add(a, b)?)
    }
    fn sub(&mut self, a: Enclosure, b: Enclosure) -> Result<Enclosure, Error> {
        Ok(Enclosure {
            lo: self
                .numeric
                .scalar(Entry::Sub, &a.lo, &b.hi, Toward::Down)?,
            hi: self.numeric.scalar(Entry::Sub, &a.hi, &b.lo, Toward::Up)?,
        })
    }
    fn corners(&mut self, a: Enclosure, b: Enclosure, entry: Entry) -> Result<Enclosure, Error> {
        if matches!(entry, Entry::Div) && !positive(&b.lo) {
            return Err(NumericError::NonpositiveDenominator.into());
        }
        let mut lo = None;
        let mut hi = None;
        for av in [a.lo, a.hi] {
            for bv in [b.lo, b.hi] {
                let l = self.numeric.scalar(entry, &av, &bv, Toward::Down)?;
                let h = self.numeric.scalar(entry, &av, &bv, Toward::Up)?;
                lo = Some(lo.map_or(l, |v| min(v, l)));
                hi = Some(hi.map_or(h, |v| max(v, h)));
            }
        }
        Ok(Enclosure {
            lo: lo.unwrap(),
            hi: hi.unwrap(),
        })
    }
    fn mul(&mut self, a: Enclosure, b: Enclosure) -> Result<Enclosure, Error> {
        self.corners(a, b, Entry::Mul)
    }
    fn div(&mut self, a: Enclosure, b: Enclosure) -> Result<Enclosure, Error> {
        self.corners(a, b, Entry::Div)
    }
    fn square(&mut self, a: Enclosure) -> Result<Enclosure, Error> {
        let upper = max(a.lo.abs(), a.hi.abs());
        let lower = if a.lo.cmp_value(&Endpoint::ZERO) != Ordering::Greater
            && a.hi.cmp_value(&Endpoint::ZERO) != Ordering::Less
        {
            Endpoint::ZERO
        } else {
            min(a.lo.abs(), a.hi.abs())
        };
        Ok(Enclosure {
            lo: self
                .numeric
                .scalar(Entry::Mul, &lower, &lower, Toward::Down)?,
            hi: self
                .numeric
                .scalar(Entry::Mul, &upper, &upper, Toward::Up)?,
        })
    }
    fn sqrt_one(&mut self, a: Endpoint, t: Toward) -> Result<Endpoint, Error> {
        self.numeric.begin(Entry::Sqrt)?;
        let spent = directed::certificate::sqrt_endpoint(&a, t);
        self.numeric.wide.merge(&spent.work());
        self.numeric.sums.merge(&spent.sum_work());
        self.numeric.status = self.numeric.status.join(spent.status());
        let result = spent.result().copied().map_err(|e| match e {
            directed::certificate::HelperError::Arithmetic(e) => NumericError::Arithmetic(e),
            _ => NumericError::Arithmetic(AttemptStop::WorkAccounting(WorkFault::Inconsistent)),
        });
        Ok(self.numeric.checked(result)?)
    }
    fn norm(&mut self, x: &[Enclosure; 3]) -> Result<Enclosure, Error> {
        let mut v = self.square(x[0])?;
        for xi in &x[1..] {
            let t = self.square(*xi)?;
            v = self.add(v, t)?;
        }
        Ok(Enclosure {
            lo: self.sqrt_one(v.lo, Toward::Down)?,
            hi: self.sqrt_one(v.hi, Toward::Up)?,
        })
    }
    fn normalize(&mut self, x: [Enclosure; 3]) -> Result<([Enclosure; 3], Enclosure), Error> {
        let length = self.norm(&x)?;
        if !positive(&length.lo) {
            // Fixed-width certificate insufficiency, not physical singularity.
            return Err(NumericError::NonpositiveDenominator.into());
        }
        let mut out = [ZERO; 3];
        for k in 0..3 {
            out[k] = self.div(x[k], length)?;
        }
        Ok((out, length))
    }
    fn nearest_add(&mut self, a: Endpoint, b: Endpoint) -> Result<Endpoint, Error> {
        self.check()?;
        let mut ctx = WideContext::<16>::new(1024)
            .map_err(AttemptStop::from)
            .map_err(NumericError::from)?;
        let result = ctx.add(&a, &b).map_err(AttemptStop::from);
        self.point.record(&ctx);
        let value = result.map_err(NumericError::from)?;
        self.check()?;
        Ok(value)
    }
}
struct Member {
    ex: [Enclosure; 3],
    ey: [Enclosure; 3],
    ez: [Enclosure; 3],
    iy: [Enclosure; 3],
    iz: [Enclosure; 3],
    inv: Enclosure,
    d: [Enclosure; 4],
    dd: [Endpoint; 4],
    lower_length: Endpoint,
}
fn coefficients(
    view: &SourceBridgeView<'_>,
    law: &ProposedMemberLaw<'_>,
    i: usize,
    w: &mut ResidualWork,
) -> Result<MemberEnclosures, Error> {
    w.member_builds = w.member_builds.add(WorkTotal::exact_count(1));
    w.check()?;
    let m = &view.source().members()[i];
    if !std::ptr::eq(m, law.member) {
        return Err(Error::MemberOwner);
    }
    let mut coefficients = build_member(
        &MemberOperands {
            diameter: law.diameter,
            effective_wall: law.effective_wall,
            material: law.material,
            admitted: AdmittedOperands {
                e: m.elastic_modulus,
                g: m.shear_modulus,
                a: m.area,
                j: m.torsion_constant,
                iy: m.second_moment_y,
                iz: m.second_moment_z,
                z_hat: law.represented_z,
            },
        },
        &mut w.numeric,
    )?;
    if w.readout_law == ReadoutLaw::AdmittedK {
        for i in 0..4 {
            w.visit()?;
            coefficients.coefficients[i]=Enclosure::point(coefficients.admitted_products[i]);
            coefficients.coefficient_differences[i]=Endpoint::ZERO;
        }
    }
    Ok(coefficients)
}
fn member(
    view: &SourceBridgeView<'_>,
    law: &ProposedMemberLaw<'_>,
    i: usize,
    w: &mut ResidualWork,
) -> Result<Member, Error> {
    let c = coefficients(view, law, i, w)?;
    let source = view.source();
    let m = law.member;
    let (mut member, length) = frame(
        source.nodes()[m.node_i as usize],
        source.nodes()[m.node_j as usize],
        m.y_reference,
        w,
    )?;
    for k in 0..4 {
        member.d[k] = w.div(c.coefficients[k], length)?;
    }
    member.dd = c.coefficient_differences;
    Ok(member)
}
fn frame(
    xi: [f64; 3],
    xj: [f64; 3],
    y_reference: [f64; 3],
    w: &mut ResidualWork,
) -> Result<(Member, Enclosure), Error> {
    w.frame_builds = w.frame_builds.add(WorkTotal::exact_count(1));
    w.check()?;
    let mut chord = [ZERO; 3];
    let mut yr = [ZERO; 3];
    let mut lower_length = Endpoint::ZERO;
    for k in 0..3 {
        w.visit()?;
        chord[k] = w.sub(point(xj[k])?, point(xi[k])?)?;
        yr[k] = point(y_reference[k])?;
        let v = chord[k];
        let lower = if v.lo.cmp_value(&Endpoint::ZERO) != Ordering::Greater
            && v.hi.cmp_value(&Endpoint::ZERO) != Ordering::Less
        {
            Endpoint::ZERO
        } else {
            min(v.lo.abs(), v.hi.abs())
        };
        lower_length = max(lower_length, lower);
    }
    let (ex, length) = w.normalize(chord)?;
    let mut projection = w.mul(yr[0], ex[0])?;
    for k in 1..3 {
        let t = w.mul(yr[k], ex[k])?;
        projection = w.add(projection, t)?;
    }
    let mut yc = [ZERO; 3];
    for k in 0..3 {
        let p = w.mul(projection, ex[k])?;
        yc[k] = w.sub(yr[k], p)?;
    }
    let (ey, _) = w.normalize(yc)?;
    let mut zc = [ZERO; 3];
    for (k, (p, q)) in [(1, 2), (2, 0), (0, 1)].into_iter().enumerate() {
        let a = w.mul(ex[p], ey[q])?;
        let b = w.mul(ex[q], ey[p])?;
        zc[k] = w.sub(a, b)?;
    }
    let (ez, _) = w.normalize(zc)?;
    let inv = w.div(ONE, length)?;
    let mut iy = [ZERO; 3];
    let mut iz = [ZERO; 3];
    for k in 0..3 {
        iy[k] = w.mul(inv, ey[k])?;
        iz[k] = w.mul(inv, ez[k])?;
    }
    Ok((
        Member {
            ex,
            ey,
            ez,
            iy,
            iz,
            inv,
            d: [ZERO; 4],
            dd: [Endpoint::ZERO; 4],
            lower_length,
        },
        length,
    ))
}
fn b(m: &Member, r: usize, j: usize) -> Option<Enclosure> {
    let k = j % 3;
    let at_j = j >= 6;
    let rotation = j % 6 >= 3;
    match r {
        0 if !rotation => Some(if at_j { m.ex[k] } else { neg(m.ex[k]) }),
        1 if rotation => Some(if at_j { m.ex[k] } else { neg(m.ex[k]) }),
        2 | 3 if !rotation => Some(if at_j { neg(m.iy[k]) } else { m.iy[k] }),
        4 | 5 if !rotation => Some(if at_j { m.iz[k] } else { neg(m.iz[k]) }),
        2 if rotation && !at_j => Some(m.ez[k]),
        3 if rotation && at_j => Some(m.ez[k]),
        4 if rotation && !at_j => Some(m.ey[k]),
        5 if rotation && at_j => Some(m.ey[k]),
        _ => None,
    }
}
fn h(m: &Member, r: usize, j: usize) -> Option<Enclosure> {
    match (r, j) {
        (0, 0) | (1, 3) => Some(neg(ONE)),
        (0, 6) | (1, 9) | (2, 5) | (3, 11) | (4, 4) | (5, 10) => Some(ONE),
        (2 | 3, 1) | (4 | 5, 8) => Some(m.inv),
        (2 | 3, 7) | (4 | 5, 2) => Some(neg(m.inv)),
        _ => None,
    }
}
const D_PATTERN: [(usize, usize, usize, i64); 10] = [
    (0, 0, 0, 0),
    (1, 1, 1, 0),
    (2, 2, 2, 2),
    (2, 3, 2, 1),
    (3, 2, 2, 1),
    (3, 3, 2, 2),
    (4, 4, 3, 2),
    (4, 5, 3, 1),
    (5, 4, 3, 1),
    (5, 5, 3, 2),
];
fn apply(m: &Member, x: &[Enclosure; 12], w: &mut ResidualWork) -> Result<[Enclosure; 6], Error> {
    let mut bx = [ZERO; 6];
    for r in 0..6 {
        for j in 0..12 {
            w.visit()?;
            if let Some(v) = b(m, r, j) {
                w.b_products = w.b_products.add(WorkTotal::exact_count(1));
                w.check()?;
                let t = w.mul(v, x[j])?;
                bx[r] = w.add(bx[r], t)?;
            }
        }
    }
    let mut q = [ZERO; 6];
    for (r, c, p, k) in D_PATTERN {
        w.visit()?;
        w.d_products = w.d_products.add(WorkTotal::exact_count(1));
        w.check()?;
        let t = w.mul(shift_interval(m.d[p], k)?, bx[c])?;
        q[r] = w.add(q[r], t)?;
    }
    Ok(q)
}
fn transpose(
    m: &Member,
    q: &[Enclosure; 6],
    local: bool,
    w: &mut ResidualWork,
) -> Result<[Enclosure; 12], Error> {
    let mut out = [ZERO; 12];
    for j in 0..12 {
        for r in 0..6 {
            w.visit()?;
            if let Some(v) = if local { h(m, r, j) } else { b(m, r, j) } {
                if local {
                    w.h_products = w.h_products.add(WorkTotal::exact_count(1));
                } else {
                    w.b_products = w.b_products.add(WorkTotal::exact_count(1));
                }
                w.check()?;
                let t = w.mul(v, q[r])?;
                out[j] = w.add(out[j], t)?;
            }
        }
    }
    Ok(out)
}
fn dofs(m: &StraightMember) -> [usize; 12] {
    std::array::from_fn(|k| (if k < 6 { m.node_i } else { m.node_j }) as usize * 6 + k % 6)
}
fn gather(
    view: &SourceBridgeView<'_>,
    center: &[Endpoint],
    epsilon: Option<&[Endpoint]>,
    g: usize,
    w: &mut ResidualWork,
) -> Result<Enclosure, Error> {
    w.visit()?;
    if let Some(c) = view.source().constraint(g) {
        return point(c);
    }
    let a = view.group().ordering.position[g];
    let y = center[a];
    if let Some(eps) = epsilon {
        let e = shift(&eps[view.group().blocks.of[a] as usize], view.scales()[a])?;
        Ok(Enclosure {
            lo: w.numeric.scalar(Entry::Sub, &y, &e, Toward::Down)?,
            hi: w.numeric.scalar(Entry::Add, &y, &e, Toward::Up)?,
        })
    } else {
        Ok(Enclosure::point(y))
    }
}
// Same 48-entry conservative Bbar and original lower-length bound as I42.
fn bbar(r: usize, j: usize, inv: Endpoint) -> Option<Endpoint> {
    match r {
        0 if j % 6 < 3 => Some(Endpoint::ONE),
        1 if j % 6 >= 3 => Some(Endpoint::ONE),
        2..=5 if j % 6 < 3 => Some(inv),
        2 | 4 if (3..6).contains(&j) => Some(Endpoint::ONE),
        3 | 5 if (9..12).contains(&j) => Some(Endpoint::ONE),
        _ => None,
    }
}
fn eta_member(
    view: &SourceBridgeView<'_>,
    m: &Member,
    globals: [usize; 12],
    eta: &mut [Endpoint],
    w: &mut ResidualWork,
) -> Result<(), Error> {
    let inv = w
        .numeric
        .scalar(Entry::Div, &Endpoint::ONE, &m.lower_length, Toward::Up)?;
    let mut ds = [Endpoint::ZERO; 4];
    for k in 0..4 {
        ds[k] = w
            .numeric
            .scalar(Entry::Div, &m.dd[k], &m.lower_length, Toward::Up)?;
    }
    let mut bx = [Endpoint::ZERO; 6];
    for r in 0..6 {
        for j in 0..12 {
            w.visit()?;
            let a = view.group().ordering.position[globals[j]];
            if let Some(v) = bbar(r, j, inv) {
                w.b_products = w.b_products.add(WorkTotal::exact_count(1));
                w.check()?;
                let s = if a == usize::MAX {
                    Endpoint::ZERO
                } else {
                    shift(&Endpoint::ONE, view.scales()[a])?
                };
                let t = w.numeric.scalar(Entry::Mul, &v, &s, Toward::Up)?;
                bx[r] = w.numeric.scalar(Entry::Add, &bx[r], &t, Toward::Up)?;
            }
        }
    }
    let mut q = [Endpoint::ZERO; 6];
    for (r, c, p, k) in D_PATTERN {
        w.visit()?;
        w.d_products = w.d_products.add(WorkTotal::exact_count(1));
        w.check()?;
        let t = w
            .numeric
            .scalar(Entry::Mul, &shift(&ds[p], k)?, &bx[c], Toward::Up)?;
        q[r] = w.numeric.scalar(Entry::Add, &q[r], &t, Toward::Up)?;
    }
    for j in 0..12 {
        let a = view.group().ordering.position[globals[j]];
        if a == usize::MAX {
            continue;
        }
        let mut v = Endpoint::ZERO;
        for r in 0..6 {
            w.visit()?;
            if let Some(c) = bbar(r, j, inv) {
                w.b_products = w.b_products.add(WorkTotal::exact_count(1));
                w.check()?;
                let t = w.numeric.scalar(Entry::Mul, &c, &q[r], Toward::Up)?;
                v = w.numeric.scalar(Entry::Add, &v, &t, Toward::Up)?;
            }
        }
        eta[a] = w.numeric.scalar(
            Entry::Add,
            &eta[a],
            &shift(&v, view.scales()[a])?,
            Toward::Up,
        )?;
    }
    Ok(())
}
fn residual(
    view: &SourceBridgeView<'_>,
    laws: &[ProposedMemberLaw<'_>],
    center: &[Endpoint],
    mut eta: Option<&mut [Endpoint]>,
    w: &mut ResidualWork,
) -> Result<Vec<Enclosure>, Error> {
    let mut out = buffer(center.len(), ZERO)?;
    w.capacity("residual", out.capacity())?;
    for (i, law) in laws.iter().enumerate() {
        w.visit()?;
        let m = member(view, law, i, w)?;
        let globals = dofs(law.member);
        let mut x = [ZERO; 12];
        for j in 0..12 {
            x[j] = gather(view, center, None, globals[j], w)?;
        }
        let q = apply(&m, &x, w)?;
        let action = transpose(&m, &q, false, w)?;
        for j in 0..12 {
            w.visit()?;
            let a = view.group().ordering.position[globals[j]];
            if a != usize::MAX {
                out[a] = w.sub(out[a], action[j])?;
            }
        }
        if let Some(rows) = eta.as_deref_mut() {
            eta_member(view, &m, globals, rows, w)?;
        }
    }
    if view.owner().prep.factors.is_empty() {
        // Actual individual terms, including cancelling terms; no pre-rounded net.
        for load in view.source().loads() {
            w.visit()?;
            let a = view.group().ordering.position[load.dof.global()];
            if a != usize::MAX {
                out[a] = w.add(out[a], point(load.value)?)?;
            }
        }
    } else {
        combination_free_loads(view, &mut out, w)?;
    }
    for spring in view.source().springs() {
        w.visit()?;
        let a = view.group().ordering.position[spring.dof.global()];
        if a != usize::MAX {
            let t = w.mul(point(spring.stiffness)?, Enclosure::point(center[a]))?;
            out[a] = w.sub(out[a], t)?;
        }
    }
    for (a, v) in out.iter_mut().enumerate() {
        w.visit()?;
        *v = shift_interval(*v, view.scales()[a])?;
    }
    Ok(out)
}
/// B2-K (KD §3.3, free rows): a combination owner's load is its own combined
/// exact ledger (K4LED, bound by the view), never the representative's
/// individual terms. Each free DOF with a nonzero net adds that net enclosed
/// outward at 1024 bits; one visit per DOF examined.
fn combination_free_loads(
    view: &SourceBridgeView<'_>,
    out: &mut [Enclosure],
    w: &mut ResidualWork,
) -> Result<(), Error> {
    #[cfg(test)]
    if hooks::representative(hooks::Site::Free) {
        for load in view.source().loads() {
            w.visit()?;
            let a = view.group().ordering.position[load.dof.global()];
            if a != usize::MAX {
                out[a] = w.add(out[a], point(load.value)?)?;
            }
        }
        return Ok(());
    }
    let ledger = &view.owner().prep.ledger;
    for (a, &g) in view.group().ordering.free.iter().enumerate() {
        w.visit()?;
        if ledger.net(g).is_some_and(|net| !net.is_zero()) {
            let net = w.numeric.ledger_net(ledger, g)?;
            out[a] = w.add(out[a], net)?;
        }
    }
    Ok(())
}
/// B2-K (KD §3.3, reaction offsets): each constrained DOF with a nonzero
/// combined net subtracts that net, enclosed outward, from its reaction.
fn combination_reaction_loads(
    view: &SourceBridgeView<'_>,
    reaction: &mut [Enclosure],
    w: &mut ResidualWork,
) -> Result<(), Error> {
    let source = view.source();
    #[cfg(test)]
    if hooks::representative(hooks::Site::Constrained) {
        for load in source.loads() {
            w.visit()?;
            if let Ok(ci) = source
                .constraints()
                .binary_search_by_key(&load.dof.global(), |c| c.dof.global())
            {
                reaction[ci] = w.sub(reaction[ci], point(load.value)?)?;
            }
        }
        return Ok(());
    }
    let ledger = &view.owner().prep.ledger;
    for (ci, c) in source.constraints().iter().enumerate() {
        w.visit()?;
        let g = c.dof.global();
        if ledger.net(g).is_some_and(|net| !net.is_zero()) {
            let net = w.numeric.ledger_net(ledger, g)?;
            reaction[ci] = w.sub(reaction[ci], net)?;
        }
    }
    Ok(())
}
/// B2-K K-08 (test-only): both combination load branches alone on a fresh work
/// record, so their own entries and visits are observable.
#[cfg(test)]
pub(crate) fn test_combination_loads(
    view: &SourceBridgeView<'_>,
) -> Result<(Vec<Enclosure>, Vec<Enclosure>, ResidualWork), Error> {
    let mut w = ResidualWork::new();
    let mut free = buffer(view.group().ordering.free.len(), ZERO)?;
    combination_free_loads(view, &mut free, &mut w)?;
    let mut reaction = buffer(view.source().constraints().len(), ZERO)?;
    combination_reaction_loads(view, &mut reaction, &mut w)?;
    Ok((free, reaction, w))
}
/// B2-K K-10 and SF-2's mutation controls (test-only): read the
/// representative's individual load terms instead of the combined ledger at the
/// free rows or at the constrained reaction offsets.
#[cfg(test)]
pub(crate) mod hooks {
    use std::cell::Cell;
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub(crate) enum Site {
        Free,
        Constrained,
    }
    thread_local! {
        static REPRESENTATIVE: Cell<[bool; 2]> = const { Cell::new([false; 2]) };
    }
    pub(crate) fn set_representative(free: bool, constrained: bool) {
        REPRESENTATIVE.with(|h| h.set([free, constrained]));
    }
    pub(crate) fn representative(site: Site) -> bool {
        REPRESENTATIVE.with(|h| h.get()[site as usize])
    }
}
fn anchored(view: &SourceBridgeView<'_>, body: u32, w: &mut ResidualWork) -> Result<bool, Error> {
    for node in 0..view.source().node_count() {
        w.visit()?;
        if view.source().body_of_node(node as u32) != body {
            continue;
        }
        let mut fixed = true;
        for c in 0..6 {
            w.visit()?;
            fixed &= view.source().constraint(node * 6 + c).is_some();
        }
        if fixed {
            return Ok(true);
        }
    }
    Ok(false)
}
#[derive(Debug)]
pub(crate) struct ConditionalSourceResidual<'a> {
    pub(crate) view: SourceBridgeView<'a>,
    pub(crate) proposed_laws: &'a [ProposedMemberLaw<'a>],
    pub(crate) rows: Vec<Enclosure>,
    pub(crate) alpha: Vec<Endpoint>,
    pub(crate) epsilon: Vec<Endpoint>,
    #[cfg(all(test))]
    pub(crate) center: Vec<Endpoint>,
    #[cfg(all(test))]
    pub(crate) rho: Vec<Enclosure>,
    #[cfg(all(test))]
    pub(crate) initial: Vec<Enclosure>,
    #[cfg(all(test))]
    pub(crate) correction: Vec<Endpoint>,
}
#[derive(Debug)]
pub(crate) struct ResidualSpent<'a> {
    result: Result<ConditionalSourceResidual<'a>, Error>,
    pub(crate) work: ResidualWork,
}
impl<'a> ResidualSpent<'a> {
    pub(crate) fn result(&self) -> Result<&ConditionalSourceResidual<'a>, Error> {
        match &self.result {
            Err(e) => Err(e.clone()),
            Ok(v) => {
                self.work.check()?;
                Ok(v)
            }
        }
    }
}
#[derive(Debug)]
pub(crate) struct LaneReadouts {
    anchor: std::sync::Arc<super::final_case::ProofAnchor>,
    law: ReadoutLaw,
    pub(crate) rows: Vec<Enclosure>,
    pub(crate) data: Vec<bool>,
    pub(crate) alpha: Vec<Endpoint>,
    pub(crate) epsilon: Vec<Endpoint>,
}
impl ResidualSpent<'_> {
    pub(super) fn into_readouts(mut self, anchor:&std::sync::Arc<super::final_case::ProofAnchor>) -> (Result<LaneReadouts,Error>, ResidualWork) {
        let result=match self.result {
            Err(e)=>Err(e),
            Ok(native)=>match self.work.status().fault() {
                Some(f)=>Err(Error::Numeric(NumericError::Arithmetic(AttemptStop::WorkAccounting(f)))),
                None=>{
                    let checked=self.work.visit().and_then(|()| {
                        if anchor.matches_owner(native.view.owner()){Ok(())}else{Err(Error::MemberOwner)}
                    });
                    checked.and_then(|()|self.work.visit()).map(|()|LaneReadouts {anchor:std::sync::Arc::clone(anchor),law:self.work.readout_law,
                        rows:native.rows,data:native.view.into_data(),alpha:native.alpha,epsilon:native.epsilon})
                },
            },
        };
        (result,self.work)
    }
}
pub(crate) fn source_residual<'a>(
    owner: &'a RetainedSolve,
    source: &'a PrimitiveSource,
    identity: &[u8],
    precision: u32,
    laws: &'a [ProposedMemberLaw<'a>],
) -> ResidualSpent<'a> {
    source_residual_for_law(owner,source,identity,precision,laws,ReadoutLaw::AnnularSource)
}
pub(crate) fn source_residual_for_law<'a>(
    owner:&'a RetainedSolve, source:&'a PrimitiveSource, identity:&[u8], precision:u32,
    laws:&'a [ProposedMemberLaw<'a>], readout_law:ReadoutLaw,
) -> ResidualSpent<'a> {
    source_residual_inner(owner,source,identity,precision,laws,readout_law,None)
}
pub(super) fn source_residual_prepared<'a>(owner:&'a RetainedSolve, source:&'a PrimitiveSource,
    identity:&[u8], precision:u32, laws:&'a [ProposedMemberLaw<'a>],
    anchor:&std::sync::Arc<super::final_case::ProofAnchor>, seed:&LaneReadouts)->ResidualSpent<'a> {
    source_residual_inner(owner,source,identity,precision,laws,ReadoutLaw::AnnularSource,Some((anchor,seed)))
}
fn source_residual_inner<'a>(owner:&'a RetainedSolve,source:&'a PrimitiveSource,identity:&[u8],precision:u32,
    laws:&'a [ProposedMemberLaw<'a>],readout_law:ReadoutLaw,
    seed:Option<(&std::sync::Arc<super::final_case::ProofAnchor>,&LaneReadouts)>)->ResidualSpent<'a> {
    let mut work = ResidualWork::new();
    work.readout_law=readout_law;
    let view = owner.source_bridge_view(source, identity, precision);
    work.view = view.work;
    work.data_capacity=work.view.data_capacity;
    let result = match view.result {
        Err(e) => Err(Error::View(e)),
        Ok(view) => {
            work.data_capacity=view.data_capacity();
            let check=match seed {None=>Ok(()),Some((anchor,k))=>validate_seed(&view,anchor,k,&mut work)};
            check.and_then(|()|run(view,laws,seed.map(|(_,k)|k),&mut work))
        },
    };
    let result = match result {
        Err(e) => Err(e),
        Ok(v) => work.check().map(|()| v),
    };
    ResidualSpent { result, work }
}
pub(super) fn validate_seed(view:&SourceBridgeView<'_>,anchor:&std::sync::Arc<super::final_case::ProofAnchor>,
    seed:&LaneReadouts,w:&mut ResidualWork)->Result<(),Error> {
    w.visit()?;
    if w.readout_law!=ReadoutLaw::AnnularSource || seed.law!=ReadoutLaw::AdmittedK
        || !std::sync::Arc::ptr_eq(anchor,&seed.anchor) || !anchor.matches_owner(view.owner())
        || seed.rows.len()!=view.owner().publish().rows.len() || seed.data.len()!=view.data().len() {return Err(Error::MemberOwner);}
    for (a,b) in seed.data.iter().zip(view.data()) {w.visit()?;if a!=b{return Err(Error::MemberOwner);}}
    // Endpoint has no NaN/infinity representation; every constructor checks its finite range.
    for g in 0..view.source().dof_count() {
        w.visit()?;
        let (row,_)=view.row(g);
        let expected=if g%6<3 {super::super::recover::Kind::Translation}else{super::super::recover::Kind::Rotation};
        if row.id!=QuantityId::Displacement(Dof::from_global(g)) || row.kind!=expected
            || seed.rows[g].lo.cmp_value(&seed.rows[g].hi)==Ordering::Greater {return Err(Error::RowIdentity(g));}
    }
    Ok(())
}
fn run<'a>(
    view: SourceBridgeView<'a>,
    laws: &'a [ProposedMemberLaw<'a>],
    seed: Option<&LaneReadouts>,
    w: &mut ResidualWork,
) -> Result<ConditionalSourceResidual<'a>, Error> {
    if !view.source().directional_springs().is_empty() {
        return Err(Error::UnsupportedDirectionalSpring);
    }
    if laws.len() != view.source().members().len() {
        return Err(Error::MemberOwner);
    }
    let nf = view.group().ordering.free.len();
    let nb = view.data().len();
    for (i, law) in laws.iter().enumerate() {
        w.visit()?;
        coefficients(&view, law, i, w)?;
    }
    let mut center = buffer(nf, Endpoint::ZERO)?;
    w.capacity("center", center.capacity())?;
    for (a, &g) in view.group().ordering.free.iter().enumerate() {
        w.visit()?;
        let (row, _) = view.row(g);
        if row.id != QuantityId::Displacement(Dof::from_global(g)) {
            return Err(Error::RowIdentity(g));
        }
        center[a] = if let Some(k)=seed {
            w.visit()?;
            let interval=k.rows[g];
            let sum=w.nearest_add(interval.lo,interval.hi)?;
            w.visit()?;
            let midpoint=shift(&sum,-1)?;
            w.visit()?;
            midpoint
        }else{lift(row.value.value().ok_or(Error::MissingRadius(g))?)?};
    }
    let mut eta = buffer(nf, Endpoint::ZERO)?;
    w.capacity("eta", eta.capacity())?;
    let initial = residual(&view, laws, &center, Some(&mut eta), w)?;
    let mut alpha = buffer(nb, Endpoint::ZERO)?;
    let mut epsilon = buffer(nb, Endpoint::ZERO)?;
    w.capacity("alpha", alpha.capacity())?;
    w.capacity("epsilon", epsilon.capacity())?;
    for b in 0..nb {
        w.visit()?;
        let body = view.group().blocks.body[b];
        if !view.data()[b] {
            if !anchored(&view, body, w)? {
                return Err(Error::MissingUniquenessWarrant(body));
            }
            for &a in &view.group().blocks.positions[b] {
                center[a] = Endpoint::ZERO;
            }
            continue;
        }
        let beta = shift(
            &pos_lift(view.bound(body).ok_or(Error::View(
                super::super::adaptive::SourceBridgeViewIssue::BodyBound,
            ))?)?,
            1,
        )?;
        let mut e = Endpoint::ZERO;
        for &a in &view.group().blocks.positions[b] {
            w.visit()?;
            e = max(e, eta[a]);
        }
        alpha[b] = w.numeric.scalar(Entry::Mul, &beta, &e, Toward::Up)?;
        if alpha[b].cmp_value(&Endpoint::ONE) != Ordering::Less {
            return Err(Error::Alpha {
                block: b,
                alpha_hi: alpha[b],
            });
        }
    }
    drop(eta);
    #[cfg(test)]
    let initial_trace = initial.clone();
    #[cfg(test)]
    let mut correction_trace = Vec::new();
    if view.data().iter().any(|v| *v) {
        let mut midpoint = buffer(nf, Endpoint::ZERO)?;
        w.capacity("midpoint", midpoint.capacity())?;
        for (a, r) in initial.iter().enumerate() {
            w.visit()?;
            midpoint[a] = shift(&w.nearest_add(r.lo, r.hi)?, -1)?;
        }
        drop(initial);
        let (result, spent_work) = view.source_correction(midpoint).into_parts();
        w.correction = spent_work;
        let correction = result.map_err(|e| match e {
            SourceCorrectionError::Arithmetic(e) => Error::Numeric(NumericError::Arithmetic(e)),
            SourceCorrectionError::Storage => Error::Storage,
            SourceCorrectionError::CountRange => Error::CountRange,
            SourceCorrectionError::Identity => {
                Error::View(super::super::adaptive::SourceBridgeViewIssue::VerificationCache)
            }
        })?;
        w.check()?;
        for a in 0..nf {
            w.visit()?;
            if view.data()[view.group().blocks.of[a] as usize] {
                center[a] = w.nearest_add(center[a], shift(&correction[a], view.scales()[a])?)?;
            }
        }
        #[cfg(test)]
        {
            correction_trace = correction;
        }
    } else {
        drop(initial);
    }
    let rho = residual(&view, laws, &center, None, w)?;
    radius(&view, &alpha, &rho, &mut epsilon, w)?;
    #[cfg(not(test))]
    drop(rho);
    let rows = recover(&view, laws, &center, &epsilon, w)?;
    Ok(ConditionalSourceResidual {
        view,
        proposed_laws: laws,
        rows,
        alpha,
        epsilon,
        #[cfg(all(test))]
        center,
        #[cfg(all(test))]
        rho,
        #[cfg(all(test))]
        initial: initial_trace,
        #[cfg(all(test))]
        correction: correction_trace,
    })
}
fn radius(
    view: &SourceBridgeView<'_>,
    alpha: &[Endpoint],
    rho: &[Enclosure],
    epsilon: &mut [Endpoint],
    w: &mut ResidualWork,
) -> Result<(), Error> {
    for b in 0..view.data().len() {
        w.visit()?;
        if !view.data()[b] {
            continue;
        }
        let body = view.group().blocks.body[b];
        let beta = shift(
            &pos_lift(view.bound(body).ok_or(Error::View(
                super::super::adaptive::SourceBridgeViewIssue::BodyBound,
            ))?)?,
            1,
        )?;
        let mut omega = Endpoint::ZERO;
        for &a in &view.group().blocks.positions[b] {
            w.visit()?;
            omega = max(omega, max(rho[a].lo.abs(), rho[a].hi.abs()));
        }
        let denominator = w
            .numeric
            .scalar(Entry::Sub, &Endpoint::ONE, &alpha[b], Toward::Down)?;
        if !positive(&denominator) {
            return Err(Error::Alpha {
                block: b,
                alpha_hi: alpha[b],
            });
        }
        let numerator = w.numeric.scalar(Entry::Mul, &beta, &omega, Toward::Up)?;
        epsilon[b] = w
            .numeric
            .scalar(Entry::Div, &numerator, &denominator, Toward::Up)?;
    }
    Ok(())
}
#[cfg(test)]
pub(crate) fn test_at_center(
    native: &ConditionalSourceResidual<'_>,
    center: &[Endpoint],
) -> Result<(Vec<Enclosure>, Vec<Endpoint>, Vec<Enclosure>, ResidualWork), Error> {
    let mut w = ResidualWork::new();
    let rho = residual(&native.view, native.proposed_laws, center, None, &mut w)?;
    let mut epsilon = buffer(native.alpha.len(), Endpoint::ZERO)?;
    radius(&native.view, &native.alpha, &rho, &mut epsilon, &mut w)?;
    let rows = recover(&native.view, native.proposed_laws, center, &epsilon, &mut w)?;
    Ok((rho, epsilon, rows, w))
}
#[cfg(test)]
pub(crate) fn test_arithmetic_controls() {
    let mut w = ResidualWork::new();
    let a = Enclosure {
        lo: lift(-3.).unwrap(),
        hi: lift(2.).unwrap(),
    };
    let b = Enclosure {
        lo: lift(-5.).unwrap(),
        hi: lift(7.).unwrap(),
    };
    let m = w.mul(a, b).unwrap();
    assert_eq!(m.lo, lift(-21.).unwrap());
    assert_eq!(m.hi, lift(15.).unwrap());
    let q = w.square(a).unwrap();
    assert!(q.lo.is_zero());
    assert_eq!(q.hi, lift(9.).unwrap());
    let div = w
        .div(
            a,
            Enclosure {
                lo: lift(2.).unwrap(),
                hi: lift(4.).unwrap(),
            },
        )
        .unwrap();
    assert_eq!(div.lo, lift(-1.5).unwrap());
    assert_eq!(div.hi, lift(1.).unwrap());
    let before = w.numeric.entries[Entry::Sqrt as usize];
    assert!(w.sqrt_one(lift(-1.).unwrap(), Toward::Down).is_err());
    assert_eq!(
        w.numeric.entries[Entry::Sqrt as usize].exact(),
        Ok(before.exact().unwrap() + 1)
    );
    assert_eq!(w.numeric.wide.width::<16>().sqrt, 1);
    // Exact positive source cross product is (0,0,2^-1074). The chord
    // difference 1+2^-1074 cannot be resolved at fixed p1024. This is
    // certificate insufficiency for a nonparallel exact source frame.
    let mut near = ResidualWork::new();
    let outcome = frame(
        [-f64::from_bits(1), 0., 0.],
        [1., 1., 0.],
        [1., 1., 0.],
        &mut near,
    );
    assert!(matches!(
        outcome,
        Err(Error::Numeric(NumericError::NonpositiveDenominator))
    ));
    assert!(near.numeric.wide.checked_lme().exact().unwrap() > 0);
    let huge = shift(&Endpoint::ONE, 9000).unwrap();
    let mut failed = ResidualWork::new();
    let error = failed.add(Enclosure::point(huge), ONE).unwrap_err();
    assert!(failed.numeric.sums.checked_lme().exact().unwrap() > 0);
    failed.visits = WorkTotal::exact_count(u64::MAX).add(WorkTotal::exact_count(1));
    assert!(failed.status().fault().is_some());
    assert!(matches!(error, Error::Numeric(NumericError::Arithmetic(_))));
}
fn set(
    view: &SourceBridgeView<'_>,
    rows: &mut [Enclosure],
    at: usize,
    id: QuantityId,
    value: Enclosure,
    w: &mut ResidualWork,
) -> Result<(), Error> {
    w.visit()?;
    if at >= rows.len() || view.row(at).0.id != id {
        return Err(Error::RowIdentity(at));
    }
    rows[at] = value;
    Ok(())
}
fn recover(
    view: &SourceBridgeView<'_>,
    laws: &[ProposedMemberLaw<'_>],
    center: &[Endpoint],
    epsilon: &[Endpoint],
    w: &mut ResidualWork,
) -> Result<Vec<Enclosure>, Error> {
    let source = view.source();
    let n = source.dof_count();
    let q = view.owner().publish().rows.len();
    let ends = count_add(n, source.node_count())?;
    let stations = count_add(ends, count_mul(laws.len(), 12)?)?;
    let springs = count_add(stations, count_mul(source.stations().len(), 6)?)?;
    let reactions = count_add(springs, source.springs().len())?;
    let supports = count_add(reactions, source.constraints().len())?;
    if count_add(supports, count_mul(source.supports().len(), 2)?)? != q {
        return Err(Error::CountRange);
    }
    let mut rows = buffer(q, ZERO)?;
    w.capacity("rows", rows.capacity())?;
    let mut reaction = buffer(source.constraints().len(), ZERO)?;
    w.capacity("reaction", reaction.capacity())?;
    for (i, law) in laws.iter().enumerate() {
        w.visit()?;
        let m = member(view, law, i, w)?;
        let globals = dofs(law.member);
        let mut x = [ZERO; 12];
        for j in 0..12 {
            x[j] = gather(view, center, Some(epsilon), globals[j], w)?;
        }
        let basic = apply(&m, &x, w)?;
        let local = transpose(&m, &basic, true, w)?;
        let global = transpose(&m, &basic, false, w)?;
        for j in 0..12 {
            set(
                view,
                &mut rows,
                ends + i * 12 + j,
                QuantityId::EndAction {
                    member: law.member.id,
                    end: if j < 6 { End::I } else { End::J },
                    component: Component::from_index(j % 6),
                },
                local[j],
                w,
            )?;
            if let Ok(ci) = source
                .constraints()
                .binary_search_by_key(&globals[j], |c| c.dof.global())
            {
                reaction[ci] = w.add(reaction[ci], global[j])?;
            }
        }
        for (si, station) in source.stations().iter().enumerate() {
            w.visit()?;
            if station.member != law.member.id {
                continue;
            }
            let t = point(station.fraction)?;
            let tm1 = w.sub(t, ONE)?;
            for c in 0..6 {
                let v = if c < 4 {
                    local[6 + c]
                } else {
                    let a = w.mul(t, local[6 + c])?;
                    let b = w.mul(tm1, local[c])?;
                    w.add(a, b)?
                };
                set(
                    view,
                    &mut rows,
                    stations + si * 6 + c,
                    QuantityId::StationAction {
                        station: station.id,
                        component: Component::from_index(c),
                    },
                    v,
                    w,
                )?;
            }
        }
    }
    for g in 0..n {
        let v = gather(view, center, Some(epsilon), g, w)?;
        set(
            view,
            &mut rows,
            g,
            QuantityId::Displacement(Dof::from_global(g)),
            v,
            w,
        )?;
    }
    for node in 0..source.node_count() {
        let v = w.norm(&[rows[node * 6], rows[node * 6 + 1], rows[node * 6 + 2]])?;
        set(
            view,
            &mut rows,
            n + node,
            QuantityId::DisplacementMagnitude(node as u32),
            v,
            w,
        )?;
    }
    for (si, spring) in source.springs().iter().enumerate() {
        w.visit()?;
        let action = neg(w.mul(point(spring.stiffness)?, rows[spring.dof.global()])?);
        set(
            view,
            &mut rows,
            springs + si,
            QuantityId::SpringAction {
                spring: spring.id,
                component: spring.dof.component,
            },
            action,
            w,
        )?;
        if let Ok(ci) = source
            .constraints()
            .binary_search_by_key(&spring.dof.global(), |c| c.dof.global())
        {
            reaction[ci] = w.sub(reaction[ci], action)?;
        }
    }
    if view.owner().prep.factors.is_empty() {
        for load in source.loads() {
            w.visit()?;
            if let Ok(ci) = source
                .constraints()
                .binary_search_by_key(&load.dof.global(), |c| c.dof.global())
            {
                reaction[ci] = w.sub(reaction[ci], point(load.value)?)?;
            }
        }
    } else {
        combination_reaction_loads(view, &mut reaction, w)?;
    }
    for (ci, c) in source.constraints().iter().enumerate() {
        set(
            view,
            &mut rows,
            reactions + ci,
            QuantityId::Reaction(c.dof),
            reaction[ci],
            w,
        )?;
    }
    for (si, support) in source.supports().iter().enumerate() {
        let mut components = [ZERO; 6];
        for (ci, c) in source.constraints().iter().enumerate() {
            w.visit()?;
            if c.dof.node == support.node && support.restrained[c.dof.component.index()] {
                let k = c.dof.component.index();
                components[k] = w.add(components[k], reaction[ci])?;
            }
        }
        for id in &support.springs {
            for (ki, spring) in source.springs().iter().enumerate() {
                w.visit()?;
                if spring.id == *id {
                    let k = spring.dof.component.index();
                    components[k] = w.add(components[k], rows[springs + ki])?;
                }
            }
        }
        let force = w.norm(&[components[0], components[1], components[2]])?;
        let moment = w.norm(&[components[3], components[4], components[5]])?;
        set(
            view,
            &mut rows,
            supports + si * 2,
            QuantityId::SupportForceMagnitude(support.id),
            force,
            w,
        )?;
        set(
            view,
            &mut rows,
            supports + si * 2 + 1,
            QuantityId::SupportMomentMagnitude(support.id),
            moment,
            w,
        )?;
    }
    Ok(rows)
}
