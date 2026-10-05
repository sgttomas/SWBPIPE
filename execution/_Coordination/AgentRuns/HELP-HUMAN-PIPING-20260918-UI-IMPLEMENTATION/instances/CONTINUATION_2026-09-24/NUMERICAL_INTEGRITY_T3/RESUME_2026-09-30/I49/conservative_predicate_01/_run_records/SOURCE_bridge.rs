//! I33/RV45's conditional native source-law bridge. This constructs the source
//! problem by replacing only member constitutive coefficients in the actual
//! owner's common frame/load/constraint/spring law. Proposed D/t/material facts
//! are NOT PP provenance, eligibility or a final product row certificate.
use super::super::adaptive::{
    RetainedSolve, SourceBridgeView, SourceBridgeViewIssue, SourceBridgeViewWork,
};
use super::super::recover::{End, QuantityId};
use super::super::source::{Component, Dof, PrimitiveSource, StraightMember};
use super::*;

#[derive(Debug, Clone, Copy)]
pub(crate) struct ProposedMemberLaw<'a> {
    pub(crate) member: &'a StraightMember,
    pub(crate) diameter: f64,
    pub(crate) effective_wall: f64,
    pub(crate) material: MaterialOperands,
    pub(crate) represented_z: f64,
}
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum BridgeError {
    View(SourceBridgeViewIssue),
    Numeric(NumericError),
    MemberOwner,
    UnsupportedDirectionalSpring,
    MissingRadius(usize),
    MissingUniquenessWarrant(u32),
    RowIdentity(usize),
    CountRange,
    Storage,
    Alpha { block: usize, alpha_hi: Endpoint },
}
impl From<NumericError> for BridgeError {
    fn from(e: NumericError) -> Self {
        Self::Numeric(e)
    }
}
#[derive(Debug)]
pub(crate) struct BridgeWork {
    pub(crate) numeric: NumericWork,
    pub(crate) view: SourceBridgeViewWork,
    /// Actual named loop/entry events only. Native constraint/pattern lookups,
    /// comparisons, vector initialization, allocation and fixed scalar bit work
    /// are auxiliary costs, not a qualified complete facade visit ledger.
    pub(crate) visits: WorkTotal,
    pub(crate) member_builds: WorkTotal,
    pub(crate) b_products: WorkTotal,
    pub(crate) d_products: WorkTotal,
    pub(crate) h_products: WorkTotal,
    /// Actual requested/capacity endpoint slots, not bytes or an M1 permit.
    pub(crate) endpoint_capacity_peak: usize,
    pub(crate) data_capacity: usize,
}
impl BridgeWork {
    fn new() -> Self {
        Self {
            numeric: NumericWork::new(),
            view: SourceBridgeViewWork::default(),
            visits: WorkTotal::zero(),
            member_builds: WorkTotal::zero(),
            b_products: WorkTotal::zero(),
            d_products: WorkTotal::zero(),
            h_products: WorkTotal::zero(),
            endpoint_capacity_peak: 0,
            data_capacity: 0,
        }
    }
    pub(crate) fn status(&self) -> WorkStatus {
        self.numeric
            .status()
            .join(self.view.visits.status())
            .join(self.view.f64_operations.status())
            .join(self.visits.status())
            .join(self.member_builds.status())
            .join(self.b_products.status())
            .join(self.d_products.status())
            .join(self.h_products.status())
    }
    fn visit(&mut self) -> Result<(), BridgeError> {
        self.visits = self.visits.add(WorkTotal::exact_count(1));
        self.check()
    }
    fn check(&self) -> Result<(), BridgeError> {
        match self.status().fault() {
            Some(f) => Err(NumericError::Arithmetic(AttemptStop::WorkAccounting(f)).into()),
            None => Ok(()),
        }
    }
    fn add(&mut self, a: Endpoint, b: Endpoint) -> Result<Endpoint, BridgeError> {
        Ok(self.numeric.scalar(Entry::Add, &a, &b, Toward::Up)?)
    }
    fn mul(&mut self, a: Endpoint, b: Endpoint) -> Result<Endpoint, BridgeError> {
        Ok(self.numeric.scalar(Entry::Mul, &a, &b, Toward::Up)?)
    }
    fn div(&mut self, a: Endpoint, b: Endpoint) -> Result<Endpoint, BridgeError> {
        Ok(self.numeric.scalar(Entry::Div, &a, &b, Toward::Up)?)
    }
}
#[derive(Debug)]
pub(crate) struct NativeSourceError {
    /// |q_G - q_K|, separate from the existing private |x - q_K| radius.
    pub(crate) source_error: Endpoint,
    pub(crate) source: Enclosure,
}
#[derive(Debug)]
pub(crate) struct ConditionalNativeSource<'a> {
    pub(crate) view: SourceBridgeView<'a>,
    /// The exact proposed operands of this conditional mathematical problem.
    /// This borrow records no normalized PP/material-selection provenance.
    pub(crate) proposed_laws: &'a [ProposedMemberLaw<'a>],
    pub(crate) rows: Vec<NativeSourceError>,
    pub(crate) alpha: Vec<Endpoint>,
    pub(crate) tau: Vec<Endpoint>,
}
#[derive(Debug)]
pub(crate) struct BridgeSpent<'a> {
    result: Result<ConditionalNativeSource<'a>, BridgeError>,
    pub(crate) work: BridgeWork,
}
impl<'a> BridgeSpent<'a> {
    pub(crate) fn result(&self) -> Result<&ConditionalNativeSource<'a>, BridgeError> {
        match &self.result {
            Err(e) => Err(e.clone()),
            Ok(v) => {
                self.work.check()?;
                Ok(v)
            }
        }
    }
}
fn buffer<T: Clone>(len: usize, value: T) -> Result<Vec<T>, BridgeError> {
    std::alloc::Layout::array::<T>(len).map_err(|_| BridgeError::CountRange)?;
    let mut out = Vec::new();
    out.try_reserve_exact(len)
        .map_err(|_| BridgeError::Storage)?;
    out.resize(len, value);
    Ok(out)
}
fn add_count(a: usize, b: usize) -> Result<usize, BridgeError> {
    a.checked_add(b).ok_or(BridgeError::CountRange)
}
fn mul_count(a: usize, b: usize) -> Result<usize, BridgeError> {
    a.checked_mul(b).ok_or(BridgeError::CountRange)
}
fn dofs(m: &StraightMember) -> [usize; 12] {
    std::array::from_fn(|k| (if k < 6 { m.node_i } else { m.node_j }) as usize * 6 + k % 6)
}
fn scaled(view: &SourceBridgeView<'_>, g: usize, value: Endpoint) -> Result<Endpoint, BridgeError> {
    let a = view.group().ordering.position[g];
    if a == usize::MAX {
        Ok(Endpoint::ZERO)
    } else {
        Ok(shift(&value, view.scales()[a])?)
    }
}
fn motion(
    view: &SourceBridgeView<'_>,
    g: usize,
    work: &mut BridgeWork,
) -> Result<Endpoint, BridgeError> {
    work.visit()?;
    if let Some(value) = view.source().constraint(g) {
        return Ok(lift(value)?.abs());
    }
    let (row, radius) = view.row(g);
    if row.id != QuantityId::Displacement(Dof::from_global(g)) {
        return Err(BridgeError::RowIdentity(g));
    }
    let x = row.value.value().ok_or(BridgeError::MissingRadius(g))?;
    work.add(
        lift(x)?.abs(),
        lift(radius.ok_or(BridgeError::MissingRadius(g))?)?,
    )
}
fn correction(
    view: &SourceBridgeView<'_>,
    tau: &[Endpoint],
    g: usize,
) -> Result<Endpoint, BridgeError> {
    let a = view.group().ordering.position[g];
    if a == usize::MAX {
        Ok(Endpoint::ZERO)
    } else {
        scaled(view, g, tau[view.group().blocks.of[a] as usize])
    }
}

/// Fixed member scratch only; rebuilt in each of three passes, never cached.
struct Majorants {
    inv: Endpoint,
    dg: [Endpoint; 4],
    dd: [Endpoint; 4],
}
fn majorants(
    view: &SourceBridgeView<'_>,
    law: &ProposedMemberLaw<'_>,
    index: usize,
    work: &mut BridgeWork,
) -> Result<Majorants, BridgeError> {
    work.member_builds = work.member_builds.add(WorkTotal::exact_count(1));
    work.check()?;
    let m = &view.source().members()[index];
    if !std::ptr::eq(m, law.member) {
        return Err(BridgeError::MemberOwner);
    }
    let input = MemberOperands {
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
    };
    let c = build_member(&input, &mut work.numeric)?;
    let mut ell = Endpoint::ZERO;
    for k in 0..3 {
        work.visit()?;
        let xj = lift(view.source().nodes()[m.node_j as usize][k])?;
        let xi = lift(view.source().nodes()[m.node_i as usize][k])?;
        let d = work.numeric.sub_pair(&xj, &xi)?;
        // A lower bound for |exact difference|, including a crossing interval.
        let lower = if d.lo.cmp_value(&Endpoint::ZERO) != Ordering::Greater
            && d.hi.cmp_value(&Endpoint::ZERO) != Ordering::Less
        {
            Endpoint::ZERO
        } else {
            min(d.lo.abs(), d.hi.abs())
        };
        ell = max(ell, lower);
    }
    if !positive(&ell) {
        return Err(NumericError::InvalidGeometry.into());
    }
    let inv = work.div(Endpoint::ONE, ell)?;
    let mut dg = [Endpoint::ZERO; 4];
    let mut dd = [Endpoint::ZERO; 4];
    for k in 0..4 {
        dg[k] = work.div(c.coefficients[k].hi, ell)?;
        dd[k] = work.div(c.coefficient_differences[k], ell)?;
    }
    Ok(Majorants { inv, dg, dd })
}
// The exact structural patterns from I33/RV45: 48 Bbar and 16 Hbar entries.
fn b_entry(r: usize, j: usize, inv: Endpoint) -> Option<Endpoint> {
    match r {
        0 if j % 6 < 3 => Some(Endpoint::ONE),
        1 if j % 6 >= 3 => Some(Endpoint::ONE),
        2..=5 if j % 6 < 3 => Some(inv),
        2 | 4 if (3..6).contains(&j) => Some(Endpoint::ONE),
        3 | 5 if (9..12).contains(&j) => Some(Endpoint::ONE),
        _ => None,
    }
}
fn h_entry(r: usize, j: usize, inv: Endpoint) -> Option<Endpoint> {
    match (r, j) {
        (0, 0 | 6) | (1, 3 | 9) | (2, 5) | (3, 11) | (4, 4) | (5, 10) => Some(Endpoint::ONE),
        (2 | 3, 1 | 7) | (4 | 5, 2 | 8) => Some(inv),
        _ => None,
    }
}
fn b_forward(
    inv: Endpoint,
    x: &[Endpoint; 12],
    work: &mut BridgeWork,
) -> Result<[Endpoint; 6], BridgeError> {
    let mut out = [Endpoint::ZERO; 6];
    for (r, value) in out.iter_mut().enumerate() {
        for (j, &xj) in x.iter().enumerate() {
            work.visit()?;
            if let Some(b) = b_entry(r, j, inv) {
                work.b_products = work.b_products.add(WorkTotal::exact_count(1));
                work.check()?;
                let term = work.mul(b, xj)?;
                *value = work.add(*value, term)?;
            }
        }
    }
    Ok(out)
}
fn d_apply(
    d: &[Endpoint; 4],
    x: &[Endpoint; 6],
    work: &mut BridgeWork,
) -> Result<[Endpoint; 6], BridgeError> {
    let mut out = [Endpoint::ZERO; 6];
    for (r, c, property, power) in [
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
    ] {
        work.visit()?;
        work.d_products = work.d_products.add(WorkTotal::exact_count(1));
        work.check()?;
        let term = work.mul(shift(&d[property], power)?, x[c])?;
        out[r] = work.add(out[r], term)?;
    }
    Ok(out)
}
fn transpose(
    inv: Endpoint,
    x: &[Endpoint; 6],
    local: bool,
    work: &mut BridgeWork,
) -> Result<[Endpoint; 12], BridgeError> {
    let mut out = [Endpoint::ZERO; 12];
    for (j, value) in out.iter_mut().enumerate() {
        for (r, &xr) in x.iter().enumerate() {
            work.visit()?;
            let entry = if local {
                h_entry(r, j, inv)
            } else {
                b_entry(r, j, inv)
            };
            if let Some(b) = entry {
                if local {
                    work.h_products = work.h_products.add(WorkTotal::exact_count(1));
                } else {
                    work.b_products = work.b_products.add(WorkTotal::exact_count(1));
                }
                work.check()?;
                let term = work.mul(b, xr)?;
                *value = work.add(*value, term)?;
            }
        }
    }
    Ok(out)
}
fn product(
    inv: Endpoint,
    d: &[Endpoint; 4],
    x: &[Endpoint; 12],
    work: &mut BridgeWork,
) -> Result<[Endpoint; 6], BridgeError> {
    let bx = b_forward(inv, x, work)?;
    d_apply(d, &bx, work)
}
fn set_error(
    view: &SourceBridgeView<'_>,
    errors: &mut [Endpoint],
    index: usize,
    expected: QuantityId,
    value: Endpoint,
    work: &mut BridgeWork,
) -> Result<(), BridgeError> {
    work.visit()?;
    if index >= errors.len() || view.row(index).0.id != expected {
        return Err(BridgeError::RowIdentity(index));
    }
    errors[index] = value;
    Ok(())
}
// A sufficient certificate warrant only. Each body is a connected member
// graph, each positive member energy has exactly its rigid-motion nullspace,
// and fixing all six motions at one node removes that nullspace throughout the
// connected graph. Additional positive springs preserve uniqueness. Failure of
// this recognizer says nothing about uniqueness under other valid supports.
fn anchored(
    view: &SourceBridgeView<'_>,
    body: u32,
    work: &mut BridgeWork,
) -> Result<bool, BridgeError> {
    for node in 0..view.source().node_count() {
        work.visit()?;
        if view.source().body_of_node(node as u32) != body {
            continue;
        }
        let mut fixed = true;
        for c in 0..6 {
            work.visit()?;
            fixed &= view.source().constraint(node * 6 + c).is_some();
        }
        if fixed {
            return Ok(true);
        }
    }
    Ok(false)
}

pub(crate) fn source_bridge<'a>(
    owner: &'a RetainedSolve,
    source: &'a PrimitiveSource,
    source_identity: &[u8],
    precision: u32,
    laws: &'a [ProposedMemberLaw<'a>],
) -> BridgeSpent<'a> {
    let mut work = BridgeWork::new();
    let spent = owner.source_bridge_view(source, source_identity, precision);
    work.view = spent.work;
    let result = match spent.result {
        Err(e) => Err(BridgeError::View(e)),
        Ok(view) => run(view, laws, &mut work),
    };
    let result = match result {
        Err(e) => Err(e),
        Ok(v) => work.check().map(|()| v),
    };
    BridgeSpent { result, work }
}
fn run<'a>(
    view: SourceBridgeView<'a>,
    laws: &'a [ProposedMemberLaw<'a>],
    work: &mut BridgeWork,
) -> Result<ConditionalNativeSource<'a>, BridgeError> {
    let source = view.source();
    if !source.directional_springs().is_empty() {
        return Err(BridgeError::UnsupportedDirectionalSpring);
    }
    if laws.len() != source.members().len() {
        return Err(BridgeError::MemberOwner);
    }
    let n = source.dof_count();
    let q = view.owner().publish().rows.len();
    let nb = view.group().blocks.len();
    // Checked row layout and explicit source-sized owner/capacity accounting.
    let ends = add_count(n, source.node_count())?;
    let stations = add_count(ends, mul_count(laws.len(), 12)?)?;
    let springs = add_count(stations, mul_count(source.stations().len(), 6)?)?;
    let reactions = add_count(springs, source.springs().len())?;
    let supports = add_count(reactions, source.constraints().len())?;
    if add_count(supports, mul_count(source.supports().len(), 2)?)? != q {
        return Err(BridgeError::CountRange);
    }
    let mut eta_rows = buffer(n, Endpoint::ZERO)?;
    let mut v_rows = buffer(n, Endpoint::ZERO)?;
    let mut tau = buffer(nb, Endpoint::ZERO)?;
    let mut alpha = buffer(nb, Endpoint::ZERO)?;
    let mut errors = buffer(q, Endpoint::ZERO)?;
    let capacity = add_count(
        add_count(eta_rows.capacity(), v_rows.capacity())?,
        add_count(
            add_count(tau.capacity(), alpha.capacity())?,
            errors.capacity(),
        )?,
    )?;
    work.endpoint_capacity_peak = capacity;
    work.data_capacity = view.data_capacity();
    // F1. Validate every source law before using any member's perturbation.
    for (i, law) in laws.iter().enumerate() {
        work.visit()?;
        majorants(&view, law, i, work)?;
    }
    // F2. All constrained columns contribute to v; eta uses free columns only.
    for (i, law) in laws.iter().enumerate() {
        work.visit()?;
        let m = majorants(&view, law, i, work)?;
        let globals = dofs(law.member);
        let mut s = [Endpoint::ZERO; 12];
        let mut mag = [Endpoint::ZERO; 12];
        for k in 0..12 {
            s[k] = scaled(&view, globals[k], Endpoint::ONE)?;
            mag[k] = motion(&view, globals[k], work)?;
        }
        let de = product(m.inv, &m.dd, &s, work)?;
        let dv = product(m.inv, &m.dd, &mag, work)?;
        let e = transpose(m.inv, &de, false, work)?;
        let v = transpose(m.inv, &dv, false, work)?;
        for k in 0..12 {
            work.visit()?;
            let g = globals[k];
            if view.group().ordering.position[g] != usize::MAX {
                eta_rows[g] = work.add(eta_rows[g], scaled(&view, g, e[k])?)?;
                v_rows[g] = work.add(v_rows[g], scaled(&view, g, v[k])?)?;
            }
        }
    }
    for b in 0..nb {
        work.visit()?;
        let body = view.group().blocks.body[b];
        if !view.data()[b] {
            if !anchored(&view, body, work)? {
                return Err(BridgeError::MissingUniquenessWarrant(body));
            }
            continue; // actual ledger/prescription/state zero, proved uniqueness
        }
        let beta = shift(
            &pos_lift(
                view.bound(body)
                    .ok_or(BridgeError::View(SourceBridgeViewIssue::BodyBound))?,
            )?,
            1,
        )?;
        let mut eta = Endpoint::ZERO;
        let mut v = Endpoint::ZERO;
        for &a in &view.group().blocks.positions[b] {
            work.visit()?;
            let g = view.group().ordering.free[a];
            eta = max(eta, eta_rows[g]);
            v = max(v, v_rows[g]);
        }
        alpha[b] = work.mul(beta, eta)?;
        if alpha[b].cmp_value(&Endpoint::ONE) != Ordering::Less {
            return Err(BridgeError::Alpha {
                block: b,
                alpha_hi: alpha[b],
            });
        }
        let denominator =
            work.numeric
                .scalar(Entry::Sub, &Endpoint::ONE, &alpha[b], Toward::Down)?;
        if !positive(&denominator) {
            return Err(BridgeError::Alpha {
                block: b,
                alpha_hi: alpha[b],
            });
        }
        let numerator = work.mul(beta, v)?;
        tau[b] = work.div(numerator, denominator)?;
    }
    eta_rows.fill(Endpoint::ZERO); // reuse for complete global reaction errors
    drop(v_rows);
    // F3. Both a_G*(u_G-u_K) and (a_G-a_K)*u_K, in fixed patterns.
    for (i, law) in laws.iter().enumerate() {
        work.visit()?;
        let m = majorants(&view, law, i, work)?;
        let globals = dofs(law.member);
        let mut t = [Endpoint::ZERO; 12];
        let mut mag = [Endpoint::ZERO; 12];
        for k in 0..12 {
            t[k] = correction(&view, &tau, globals[k])?;
            mag[k] = motion(&view, globals[k], work)?;
        }
        let response = product(m.inv, &m.dg, &t, work)?;
        let coefficient = product(m.inv, &m.dd, &mag, work)?;
        let mut basic = [Endpoint::ZERO; 6];
        for k in 0..6 {
            basic[k] = work.add(response[k], coefficient[k])?;
        }
        let local = transpose(m.inv, &basic, true, work)?;
        let global = transpose(m.inv, &basic, false, work)?;
        for k in 0..12 {
            eta_rows[globals[k]] = work.add(eta_rows[globals[k]], global[k])?;
            set_error(
                &view,
                &mut errors,
                ends + i * 12 + k,
                QuantityId::EndAction {
                    member: law.member.id,
                    end: if k < 6 { End::I } else { End::J },
                    component: Component::from_index(k % 6),
                },
                local[k],
                work,
            )?;
        }
        for (si, station) in source.stations().iter().enumerate() {
            work.visit()?; // explicit bounded M*S scan, no uncounted callback
            if station.member != law.member.id {
                continue;
            }
            let t = lift(station.fraction)?;
            let one_minus_t = work
                .numeric
                .scalar(Entry::Sub, &Endpoint::ONE, &t, Toward::Up)?;
            for c in 0..6 {
                let error = if c < 4 {
                    local[6 + c]
                } else {
                    let at_i = work.mul(one_minus_t, local[c])?;
                    let at_j = work.mul(t, local[6 + c])?;
                    work.add(at_i, at_j)?
                };
                set_error(
                    &view,
                    &mut errors,
                    stations + si * 6 + c,
                    QuantityId::StationAction {
                        station: station.id,
                        component: Component::from_index(c),
                    },
                    error,
                    work,
                )?;
            }
        }
    }
    // F4. Native components, existing native magnitude rows and actual support
    // memberships. No new magnitude of the published components is substituted.
    for g in 0..n {
        let error = correction(&view, &tau, g)?;
        set_error(
            &view,
            &mut errors,
            g,
            QuantityId::Displacement(Dof::from_global(g)),
            error,
            work,
        )?;
    }
    for node in 0..source.node_count() {
        let mut error = Endpoint::ZERO;
        for c in 0..3 {
            error = work.add(error, errors[node * 6 + c])?;
        }
        set_error(
            &view,
            &mut errors,
            n + node,
            QuantityId::DisplacementMagnitude(node as u32),
            error,
            work,
        )?;
    }
    for (i, spring) in source.springs().iter().enumerate() {
        let error = work.mul(pos_lift(spring.stiffness)?, errors[spring.dof.global()])?;
        set_error(
            &view,
            &mut errors,
            springs + i,
            QuantityId::SpringAction {
                spring: spring.id,
                component: spring.dof.component,
            },
            error,
            work,
        )?;
    }
    for (i, constraint) in source.constraints().iter().enumerate() {
        set_error(
            &view,
            &mut errors,
            reactions + i,
            QuantityId::Reaction(constraint.dof),
            eta_rows[constraint.dof.global()],
            work,
        )?;
    }
    for (i, support) in source.supports().iter().enumerate() {
        let mut bounds = [Endpoint::ZERO; 2];
        for (ci, c) in source.constraints().iter().enumerate() {
            work.visit()?;
            if c.dof.node == support.node && support.restrained[c.dof.component.index()] {
                let k = c.dof.component.index() / 3;
                bounds[k] = work.add(bounds[k], errors[reactions + ci])?;
            }
        }
        for &id in &support.springs {
            for (si, spring) in source.springs().iter().enumerate() {
                work.visit()?;
                if spring.id == id {
                    let k = spring.dof.component.index() / 3;
                    bounds[k] = work.add(bounds[k], errors[springs + si])?;
                }
            }
        }
        set_error(
            &view,
            &mut errors,
            supports + i * 2,
            QuantityId::SupportForceMagnitude(support.id),
            bounds[0],
            work,
        )?;
        set_error(
            &view,
            &mut errors,
            supports + i * 2 + 1,
            QuantityId::SupportMomentMagnitude(support.id),
            bounds[1],
            work,
        )?;
    }
    drop(eta_rows);
    std::alloc::Layout::array::<NativeSourceError>(q).map_err(|_| BridgeError::CountRange)?;
    let mut rows = Vec::new();
    rows.try_reserve_exact(q)
        .map_err(|_| BridgeError::Storage)?;
    work.endpoint_capacity_peak = work.endpoint_capacity_peak.max(add_count(
        add_count(errors.capacity(), mul_count(rows.capacity(), 3)?)?,
        add_count(tau.capacity(), alpha.capacity())?,
    )?);
    for (i, error) in errors.into_iter().enumerate() {
        work.visit()?;
        let (row, radius) = view.row(i);
        let x = lift(row.value.value().ok_or(BridgeError::MissingRadius(i))?)?;
        let interval = if let Some(r) = radius {
            let radius = work.add(lift(r)?, error)?;
            Enclosure {
                lo: work.numeric.scalar(Entry::Sub, &x, &radius, Toward::Down)?,
                hi: work.numeric.scalar(Entry::Add, &x, &radius, Toward::Up)?,
            }
        } else {
            let QuantityId::Displacement(dof) = row.id else {
                return Err(BridgeError::MissingRadius(i));
            };
            let expected = lift(
                source
                    .constraint(dof.global())
                    .ok_or(BridgeError::MissingRadius(i))?,
            )?;
            if x.cmp_value(&expected) != Ordering::Equal {
                return Err(BridgeError::RowIdentity(i));
            }
            Enclosure::point(expected) // exact prescription, never an absent solve radius
        };
        rows.push(NativeSourceError {
            source_error: error,
            source: interval,
        });
    }
    work.check()?;
    Ok(ConditionalNativeSource {
        view,
        proposed_laws: laws,
        rows,
        alpha,
        tau,
    })
}
