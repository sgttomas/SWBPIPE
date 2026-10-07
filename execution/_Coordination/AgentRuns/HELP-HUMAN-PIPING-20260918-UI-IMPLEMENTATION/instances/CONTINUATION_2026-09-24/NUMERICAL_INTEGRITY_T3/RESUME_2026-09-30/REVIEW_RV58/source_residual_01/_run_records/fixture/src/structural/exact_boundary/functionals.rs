//! Declared-source affine arithmetic, before displacement projection.
//!
//! These constructors prove arithmetic over the supplied ordered atom recipes.
//! They do not prove that a caller's row is a complete mechanical recovery row.
//! The source-owning adapter must establish that separate provenance/closure.
//! A single non-cloneable ledger covers preparation through retention and replay.
use super::*;
use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AttemptStage {
    SourceClosure,
    Preparation,
    Solve,
    Plan,
    Evaluation,
    Projection,
    Retention,
    Replay,
}

/// One attempt, including failed calls and denied reservations. This ledger has
/// no reset or clone operation. A fresh ledger cannot reuse its live Context.
#[derive(Debug)]
pub struct AttemptBudget {
    work: Work,
    token: Arc<()>,
    stage: AttemptStage,
}
impl AttemptBudget {
    pub fn new(limits: Limits) -> Self {
        Self {
            work: Work {
                limits,
                used: 0,
                rejected: 0,
            },
            token: Arc::new(()),
            stage: AttemptStage::Preparation,
        }
    }
    pub fn report(&self) -> WorkReport {
        WorkReport {
            charged: self.work.used,
            rejected: self.work.rejected,
            limit: self.work.limits.operations,
        }
    }
    pub fn stage(&self) -> AttemptStage {
        self.stage
    }
    pub fn remaining(&self) -> usize {
        self.work.limits.operations.saturating_sub(self.work.used)
    }
    pub fn limits(&self) -> Limits {
        self.work.limits
    }
    /// Reserve adapter work before source/metadata traversal or copies.
    pub fn charge(&mut self, stage: AttemptStage, operations: usize) -> Result<(), Error> {
        self.stage = stage;
        self.work.charge(operations)
    }
    fn run<T>(
        &mut self,
        stage: AttemptStage,
        f: impl FnOnce(&mut Work, &Arc<()>) -> Result<T, Error>,
    ) -> Attempt<T> {
        self.stage = stage;
        let result = f(&mut self.work, &self.token);
        Attempt {
            result,
            work: self.report(),
        }
    }
}
fn binding(context: &Context, token: &Arc<()>, w: &mut Work) -> Result<(), Error> {
    w.charge(4)?;
    if !context
        .functional_attempt
        .as_ref()
        .is_some_and(|t| Arc::ptr_eq(t, token))
        || context.limits != w.limits
    {
        return Err(Error::Invalid("functional attempt/source binding"));
    }
    Ok(())
}

// Legacy preparation has no failure WorkReport and does not charge every source
// copy. This entry uses its charged implementation plus bounded source preflight
// before inspecting/copying dynamic input. It does not alter the legacy path.
fn response_binding(response: &Response<'_>, token: &Arc<()>, w: &mut Work) -> Result<(), Error> {
    binding(response.context, token, w)?;
    if !response.functional_solve {
        return Err(Error::Invalid("unmetered functional solve"));
    }
    Ok(())
}
fn source_preflight(
    system: &StructuralSystem<'_>,
    identity: &str,
    force: ForceBasis<'_>,
    w: &mut Work,
) -> Result<(), Error> {
    w.charge(16)?;
    let n = system.force.len();
    let cs = system
        .contributions
        .ok_or(Error::Invalid("complete stiffness contributions required"))?;
    let fs = match force {
        ForceBasis::DeclaredVector => &[][..],
        ForceBasis::IdentifiedContributions(t) => t,
    };
    if n > 256
        || n > w.limits.dofs
        || cs
            .len()
            .saturating_add(if matches!(force, ForceBasis::DeclaredVector) {
                n
            } else {
                fs.len()
            })
            > 16_384
        || cs
            .len()
            .saturating_add(if matches!(force, ForceBasis::DeclaredVector) {
                n
            } else {
                fs.len()
            })
            > w.limits.source_terms
    {
        return Err(Error::Budget);
    }
    if system.stiffness.len() != n || system.free_dofs.len() > n || system.prescribed.len() > n {
        return Err(Error::Invalid("functional source dimensions"));
    }
    // Reserve matrix validation, graph traversals, scalar copies and term headers.
    w.charge(
        n.saturating_mul(n)
            .saturating_mul(8)
            .saturating_add(n.saturating_mul(24))
            .saturating_add(cs.len().saturating_mul(8))
            .saturating_add(fs.len().saturating_mul(8))
            .saturating_add(identity.len()),
    )?;
    if system.stiffness.iter().any(|row| row.len() != n) {
        return Err(Error::Invalid("functional source row dimensions"));
    }
    for f in fs {
        w.charge(f.source.len())?;
    }
    if let Some(s) = &system.symmetry {
        if s.absolute_roundoff.len() != n || s.operation_counts.len() != n {
            return Err(Error::Invalid("functional symmetry dimensions"));
        }
        w.charge(
            n.saturating_mul(n)
                .saturating_mul(4)
                .saturating_add(s.basis.len()),
        )?;
        if s.absolute_roundoff.iter().any(|r| r.len() != n)
            || s.operation_counts.iter().any(|r| r.len() != n)
        {
            return Err(Error::Invalid("functional symmetry rows"));
        }
    }
    Ok(())
}
fn prepare(
    system: &StructuralSystem<'_>,
    identity: &str,
    force: ForceBasis<'_>,
    token: &Arc<()>,
    w: &mut Work,
) -> Result<Context, Error> {
    source_preflight(system, identity, force, w)?;
    let mut c = Context::new_charged(system, identity, force, w.limits, w)?;
    c.functional_attempt = Some(Arc::clone(token));
    Ok(c)
}
impl Context {
    pub fn prepare_with_budget(
        system: &StructuralSystem<'_>,
        identity: &str,
        force: ForceBasis<'_>,
        budget: &mut AttemptBudget,
    ) -> Attempt<Self> {
        budget.run(AttemptStage::Preparation, |w, token| {
            prepare(system, identity, force, token, w)
        })
    }
    pub fn solve_with_budget(&self, budget: &mut AttemptBudget) -> Attempt<Response<'_>> {
        budget.run(AttemptStage::Solve, |w, token| {
            binding(self, token, w)?;
            solve(self, w)
        })
    }
}
fn solve<'a>(context: &'a Context, w: &mut Work) -> Result<Response<'a>, Error> {
    // Bound legacy vector/scalar clones and traversals before solve arithmetic.
    let n = context.f.len();
    w.charge(
        n.saturating_mul(n)
            .saturating_mul(4)
            .saturating_add(n.saturating_mul(8)),
    )?;
    let scalars = context
        .k
        .iter()
        .flatten()
        .chain(&context.f)
        .fold(0usize, |a, x| a.saturating_add(x.terms.len()));
    w.charge(
        n.saturating_mul(n)
            .saturating_mul(8)
            .saturating_add(scalars.saturating_mul(n.saturating_add(4)))
            .saturating_add(n.saturating_mul(16)),
    )?;
    let mut response = context.solve_charged(w)?;
    response.functional_solve = true;
    Ok(response)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum MemberEnd {
    I,
    J,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum FunctionalUnit {
    Newton,
    NewtonMetre,
    Metre,
    Millimetre,
    Radian,
    Dimensionless,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum FunctionalConvention {
    NodeOnElement,
    SectionLocal,
    SpringOnStructure,
    SupportOnStructure,
    SourceEquilibrium,
    DeclaredAffine,
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum FunctionalQuantity {
    NodeDisplacement {
        node: String,
        dof: usize,
    },
    MemberEnd {
        member: String,
        end: MemberEnd,
        row: u8,
    },
    MemberSection {
        member: String,
        station_bits: u64,
        component: u8,
    },
    GroundSpring {
        support: String,
        dof: usize,
    },
    SupportAction {
        support: String,
        node: usize,
        component: u8,
    },
    SourceReaction {
        dof: usize,
    },
    DeclaredAffine {
        label: String,
    },
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct FunctionalKey {
    pub case_id: String,
    pub quantity: FunctionalQuantity,
    pub unit: FunctionalUnit,
    pub convention: FunctionalConvention,
}
/// Each inner vector is a product; their ordered sum is exact. An empty sum is
/// zero. An empty product is invalid; use [1.0] for the constant one. Products
/// retain atom order and bits, including signed zero. No rounded aggregate is
/// silently substituted for an atom recipe.
pub type Products = Vec<Vec<f64>>;
#[derive(Debug, Clone)]
pub struct AffineTerm {
    pub dof: usize,
    pub products: Products,
}
#[derive(Debug, Clone)]
pub struct FunctionalDescriptor {
    pub key: FunctionalKey,
    pub offset: Products,
    pub terms: Vec<AffineTerm>,
}
#[derive(Debug)]
pub struct FunctionalPlan<'a> {
    context: &'a Context,
    descriptors: Vec<FunctionalDescriptor>,
}
impl<'a> FunctionalPlan<'a> {
    pub fn new(
        context: &'a Context,
        descriptors: &[FunctionalDescriptor],
        budget: &mut AttemptBudget,
    ) -> Attempt<Self> {
        budget.run(AttemptStage::Plan, |w, token| {
            binding(context, token, w)?;
            descriptors_charge(descriptors, context.f.len(), w)?;
            Ok(Self {
                context,
                descriptors: descriptors.to_vec(),
            })
        })
    }
    pub fn descriptors(&self) -> &[FunctionalDescriptor] {
        &self.descriptors
    }
}
fn key_bytes(key: &FunctionalKey) -> usize {
    let label = match &key.quantity {
        FunctionalQuantity::NodeDisplacement { node, .. } => node,
        FunctionalQuantity::MemberEnd { member, .. }
        | FunctionalQuantity::MemberSection { member, .. } => member,
        FunctionalQuantity::GroundSpring { support, .. }
        | FunctionalQuantity::SupportAction { support, .. } => support,
        FunctionalQuantity::DeclaredAffine { label } => label,
        FunctionalQuantity::SourceReaction { .. } => "",
    };
    key.case_id
        .len()
        .saturating_add(label.len())
        .saturating_add(8)
}
fn valid_key(key: &FunctionalKey, n: usize) -> bool {
    if key.case_id.is_empty() {
        return false;
    }
    let force_unit = |row: u8| {
        key.unit
            == if row < 3 {
                FunctionalUnit::Newton
            } else {
                FunctionalUnit::NewtonMetre
            }
    };
    match &key.quantity {
        FunctionalQuantity::NodeDisplacement { node, dof } => {
            !node.is_empty()
                && *dof < n
                && matches!(
                    key.unit,
                    FunctionalUnit::Metre | FunctionalUnit::Millimetre | FunctionalUnit::Radian
                )
                && key.convention == FunctionalConvention::DeclaredAffine
        }
        FunctionalQuantity::MemberEnd { member, row, .. } => {
            !member.is_empty()
                && *row < 6
                && force_unit(*row)
                && key.convention == FunctionalConvention::NodeOnElement
        }
        FunctionalQuantity::MemberSection {
            member,
            station_bits,
            component,
        } => {
            !member.is_empty()
                && f64::from_bits(*station_bits).is_finite()
                && *component < 6
                && force_unit(*component)
                && key.convention == FunctionalConvention::SectionLocal
        }
        FunctionalQuantity::GroundSpring { support, dof } => {
            !support.is_empty()
                && *dof < n
                && matches!(
                    key.unit,
                    FunctionalUnit::Newton | FunctionalUnit::NewtonMetre
                )
                && key.convention == FunctionalConvention::SpringOnStructure
        }
        FunctionalQuantity::SupportAction { support, node, component } => {
            !support.is_empty() && *component < 6
                && node.checked_mul(6).and_then(|dof| dof.checked_add(*component as usize)).is_some_and(|dof| dof < n)
                && force_unit(*component)
                && key.convention == FunctionalConvention::SupportOnStructure
        }
        FunctionalQuantity::SourceReaction { dof } => {
            *dof < n
                && matches!(
                    key.unit,
                    FunctionalUnit::Newton | FunctionalUnit::NewtonMetre
                )
                && key.convention == FunctionalConvention::SourceEquilibrium
        }
        FunctionalQuantity::DeclaredAffine { label } => {
            !label.is_empty() && key.convention == FunctionalConvention::DeclaredAffine
        }
    }
}
fn products_charge(products: &Products, count: &mut usize, w: &mut Work) -> Result<(), Error> {
    w.charge(products.len().saturating_mul(4))?;
    *count = count.saturating_add(products.len());
    if *count > w.limits.source_terms.min(16_384) {
        return Err(Error::Budget);
    }
    for p in products {
        w.charge(p.len().saturating_mul(4))?;
        *count = count.saturating_add(p.len());
        if *count > w.limits.source_terms.min(16_384) {
            return Err(Error::Budget);
        }
        if p.is_empty() || p.iter().any(|x| !x.is_finite()) {
            return Err(Error::Invalid("functional product atoms"));
        }
    }
    Ok(())
}
fn descriptors_charge(ds: &[FunctionalDescriptor], n: usize, w: &mut Work) -> Result<(), Error> {
    w.charge(8)?;
    if ds.len() > w.limits.source_terms.min(16_384) {
        return Err(Error::Budget);
    }
    w.charge(ds.len().saturating_mul(16))?;
    let mut count = ds.len();
    // Binary insertion compares borrowed keys logarithmically. Moving the
    // integer indices still has a quadratic bound, separately reserved before
    // each insertion; long IDs therefore never multiply that copy workload.
    let mut ordered: Vec<usize> = Vec::with_capacity(ds.len());
    for (i, d) in ds.iter().enumerate() {
        let comparisons = (usize::BITS - (i.saturating_add(1)).leading_zeros()) as usize + 1;
        w.charge(
            key_bytes(&d.key)
                .saturating_mul(comparisons.saturating_add(4))
                .saturating_add(i)
                .saturating_add(8),
        )?;
        if !valid_key(&d.key, n) {
            return Err(Error::Invalid("functional typed identity"));
        }
        // Explicit binary search makes the charged comparison bound independent
        // of a library's sort algorithm or hash-table collision behavior.
        let (mut low, mut high) = (0usize, ordered.len());
        while low < high {
            let middle = low + (high - low) / 2;
            match ds[ordered[middle]].key.cmp(&d.key) {
                std::cmp::Ordering::Equal => {
                    return Err(Error::Invalid("duplicate functional identity"))
                }
                std::cmp::Ordering::Less => low = middle + 1,
                std::cmp::Ordering::Greater => high = middle,
            }
        }
        ordered.insert(low, i);
        products_charge(&d.offset, &mut count, w)?;
        w.charge(d.terms.len().saturating_mul(8))?;
        count = count.saturating_add(d.terms.len());
        if count > w.limits.source_terms.min(16_384) {
            return Err(Error::Budget);
        }
        for t in &d.terms {
            if t.dof >= n {
                return Err(Error::Invalid("functional dof"));
            }
            products_charge(&t.products, &mut count, w)?;
        }
    }
    Ok(())
}
fn same_products(a: &Products, b: &Products) -> bool {
    a.len() == b.len() && a.iter().zip(b).all(|(a, b)| same_terms(a, b))
}
fn same_descriptors(a: &[FunctionalDescriptor], b: &[FunctionalDescriptor]) -> bool {
    a.len() == b.len()
        && a.iter().zip(b).all(|(a, b)| {
            a.key == b.key
                && same_products(&a.offset, &b.offset)
                && a.terms.len() == b.terms.len()
                && a.terms
                    .iter()
                    .zip(&b.terms)
                    .all(|(a, b)| a.dof == b.dof && same_products(&a.products, &b.products))
        })
}
fn copy_exp(e: &Expansion, w: &mut Work) -> Result<Expansion, Error> {
    w.charge(e.terms.len())?;
    Ok(e.clone())
}
fn add(a: &Expansion, b: &Expansion, w: &mut Work) -> Result<Expansion, Error> {
    w.charge(a.terms.len())?;
    w.sum(a, b, 1.0)
}
fn mul(a: &Expansion, b: &Expansion, w: &mut Work) -> Result<Expansion, Error> {
    w.charge(4)?;
    if a.is_zero() || b.is_zero() {
        return Ok(Expansion::default());
    }
    // Exact ±1 scaling does not require product/radix machinery or round a recipe.
    for (unit, other) in [(a, b), (b, a)] {
        if unit.terms.len() == 1 && unit.terms[0].abs() == 1.0 {
            if unit.terms[0] == 1.0 {
                return copy_exp(other, w);
            }
            return w.sum(&Expansion::default(), other, -1.0);
        }
    }
    w.mul(a, b)
}
fn recipe(products: &Products, w: &mut Work) -> Result<Expansion, Error> {
    let mut out = Expansion::default();
    for product in products {
        let mut p = w.scalar(product[0])?;
        for &atom in &product[1..] {
            let a = w.scalar(atom)?;
            p = mul(&p, &a, w)?;
        }
        out = add(&out, &p, w)?;
    }
    Ok(out)
}
fn combine(a: Ratio, b: Ratio, w: &mut Work) -> Result<Ratio, Error> {
    w.charge(ratio_charge(&a).saturating_add(ratio_charge(&b)))?;
    if a.numerator.is_zero() {
        return Ok(b);
    }
    if b.numerator.is_zero() {
        return Ok(a);
    }
    if same_terms(&a.denominator.terms, &b.denominator.terms) {
        return Ok(Ratio {
            numerator: add(&a.numerator, &b.numerator, w)?,
            denominator: a.denominator,
        });
    }
    let an = mul(&a.numerator, &b.denominator, w)?;
    let bn = mul(&b.numerator, &a.denominator, w)?;
    Ok(Ratio {
        numerator: add(&an, &bn, w)?,
        denominator: mul(&a.denominator, &b.denominator, w)?,
    })
}
fn evaluate(
    response: &Response<'_>,
    ds: &[FunctionalDescriptor],
    w: &mut Work,
) -> Result<Vec<Ratio>, Error> {
    let c = response.context;
    let n = c.f.len();
    w.charge(
        ds.len()
            .saturating_mul(n.saturating_mul(4).saturating_add(8)),
    )?;
    let mut values = Vec::with_capacity(ds.len());
    for d in ds {
        let mut coefficients = vec![Expansion::default(); n];
        for t in &d.terms {
            let coefficient = recipe(&t.products, w)?;
            coefficients[t.dof] = add(&coefficients[t.dof], &coefficient, w)?;
        }
        let mut fixed = recipe(&d.offset, w)?;
        for &(i, value) in &c.source.prescribed {
            if coefficients[i].is_zero() {
                continue;
            }
            let prescribed = w.scalar(value)?;
            let p = mul(&coefficients[i], &prescribed, w)?;
            fixed = add(&fixed, &p, w)?;
        }
        let mut q = constant(w, fixed)?;
        for block in &c.blocks {
            let mut numerator = Expansion::default();
            for &i in block {
                if coefficients[i].is_zero() {
                    continue;
                }
                let p = mul(&coefficients[i], &response.displacement[i].numerator, w)?;
                numerator = add(&numerator, &p, w)?;
            }
            if numerator.is_zero() {
                continue;
            }
            // Context's two-DOF solve constructs the SAME positive denominator
            // for both coordinates. Combine their numerators before any ratio sum.
            let denominator = copy_exp(&response.displacement[block[0]].denominator, w)?;
            q = combine(
                q,
                Ratio {
                    numerator,
                    denominator,
                },
                w,
            )?;
        }
        values.push(q);
    }
    Ok(values)
}

#[derive(Debug)]
pub struct FunctionalSet<'r, 'p, 'a> {
    response: &'r Response<'a>,
    plan: &'p FunctionalPlan<'a>,
    values: Vec<Ratio>,
}
impl<'a> Response<'a> {
    pub fn gap_proof_with_budget(
        &self,
        dof: usize,
        sense: i8,
        gap: f64,
        budget: &mut AttemptBudget,
    ) -> Attempt<GapProof<'_, 'a>> {
        budget.run(AttemptStage::Evaluation, |w, token| {
            response_binding(self, token, w)?;
            Ok(GapProof {
                response: self,
                contact: contact_charged(self, dof, sense, gap, w)?,
                functional_attempt: Some(Arc::clone(token)),
            })
        })
    }
    pub fn evaluate_functionals<'r, 'p>(
        &'r self,
        plan: &'p FunctionalPlan<'a>,
        budget: &mut AttemptBudget,
    ) -> Attempt<FunctionalSet<'r, 'p, 'a>> {
        budget.run(AttemptStage::Evaluation, |w, token| {
            response_binding(self, token, w)?;
            if !std::ptr::eq(self.context, plan.context) {
                return Err(Error::Invalid("functional plan response"));
            }
            let values = evaluate(self, &plan.descriptors, w)?;
            Ok(FunctionalSet {
                response: self,
                plan,
                values,
            })
        })
    }
    pub fn project_with_budget(
        &self,
        quantity: Quantity,
        dof: usize,
        relative_limit: f64,
        budget: &mut AttemptBudget,
    ) -> Attempt<QualifiedProjection<'_, 'a>> {
        budget.run(AttemptStage::Projection, |w, token| {
            response_binding(self, token, w)?;
            let r = match quantity {
                Quantity::Displacement => self.displacement.get(dof),
                Quantity::Reaction => self.reactions.get(dof),
            }
            .ok_or(Error::Invalid("projection dof"))?;
            let p = project(w, r, relative_limit)?;
            Ok(QualifiedProjection {
                response: self,
                quantity,
                dof,
                value: p.value,
                interval: p.interval,
                absolute_error: p.absolute,
                relative_error: p.relative,
                relative_limit,
                basis: p.basis,
                functional_attempt: Some(Arc::clone(token)),
            })
        })
    }
}
#[derive(Debug)]
pub struct QualifiedFunctionalProjection<'s, 'r, 'p, 'a> {
    set: &'s FunctionalSet<'r, 'p, 'a>,
    index: usize,
    data: ProjectionData,
    relative_limit: f64,
}
impl QualifiedFunctionalProjection<'_, '_, '_, '_> {
    pub fn key(&self) -> &FunctionalKey {
        &self.set.plan.descriptors[self.index].key
    }
    pub fn value(&self) -> f64 {
        self.data.value
    }
    pub fn interval(&self) -> [f64; 2] {
        self.data.interval
    }
    pub fn absolute_error_bound(&self) -> f64 {
        self.data.absolute
    }
    pub fn relative_error_bound(&self) -> f64 {
        self.data.relative
    }
    pub fn relative_limit(&self) -> f64 {
        self.relative_limit
    }
    pub fn basis(&self) -> ProjectionBasis {
        self.data.basis
    }
    pub fn exact_sign(&self) -> Sign {
        self.set.values[self.index].sign()
    }
    pub fn index(&self) -> usize {
        self.index
    }
    pub fn is_for(&self, set: &FunctionalSet<'_, '_, '_>, index: usize) -> bool {
        std::ptr::eq(self.set, set) && self.index == index
    }
}
impl<'r, 'p, 'a> FunctionalSet<'r, 'p, 'a> {
    pub fn len(&self) -> usize {
        self.values.len()
    }
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
    pub fn ratio(&self, index: usize) -> Option<&Ratio> {
        self.values.get(index)
    }
    pub fn project(
        &self,
        index: usize,
        relative_limit: f64,
        budget: &mut AttemptBudget,
    ) -> Attempt<QualifiedFunctionalProjection<'_, 'r, 'p, 'a>> {
        budget.run(AttemptStage::Projection, |w, token| {
            response_binding(self.response, token, w)?;
            let ratio = self
                .values
                .get(index)
                .ok_or(Error::Invalid("functional index"))?;
            let data = project(w, ratio, relative_limit)?;
            Ok(QualifiedFunctionalProjection {
                set: self,
                index,
                data,
                relative_limit,
            })
        })
    }
    /// Requires every plan slot once, in plan order, and the legacy 2*n DOF leaves.
    /// The existing response retention constructor remains the sole owner of its
    /// DOF/contact coverage semantics. No caller report can populate either proof.
    pub fn retain(
        &self,
        contacts: &[GapProof<'_, '_>],
        dof_projections: &[QualifiedProjection<'_, '_>],
        projections: &[QualifiedFunctionalProjection<'_, '_, '_, '_>],
        budget: &mut AttemptBudget,
    ) -> Attempt<RetainedFunctionalSet> {
        budget.run(AttemptStage::Retention, |w, token| {
            response_binding(self.response, token, w)?;
            w.charge(projections.len().saturating_mul(16).saturating_add(8))?;
            if projections.len() != self.values.len() {
                return Err(Error::Invalid("functional retention coverage"));
            }
            for (i, p) in projections.iter().enumerate() {
                if !std::ptr::eq(p.set, self) || p.index != i {
                    return Err(Error::Invalid("functional retention identity/order"));
                }
            }
            w.charge(
                dof_projections
                    .len()
                    .saturating_add(contacts.len())
                    .saturating_mul(4),
            )?;
            if dof_projections.iter().any(|p| {
                !p.functional_attempt
                    .as_ref()
                    .is_some_and(|t| Arc::ptr_eq(t, token))
            }) || contacts.iter().any(|p| {
                !p.functional_attempt
                    .as_ref()
                    .is_some_and(|t| Arc::ptr_eq(t, token))
            }) {
                return Err(Error::Invalid("unmetered functional retention leaves"));
            }
            descriptors_charge(&self.plan.descriptors, self.response.context.f.len(), w)?;
            w.charge(
                self.values
                    .iter()
                    .fold(0usize, |n, r| n.saturating_add(ratio_charge(r))),
            )?;
            let a = self.response.retain_with_work(
                contacts,
                dof_projections,
                w.limits.operations.saturating_sub(w.used),
            );
            // The legacy call was capped at the decreasing remainder, and its
            // failed/denied reservations are folded back before returning error.
            w.used = w.used.saturating_add(a.work.charged);
            w.rejected = w.rejected.saturating_add(a.work.rejected);
            let response = a.result?;
            Ok(RetainedFunctionalSet {
                token: Arc::clone(token),
                response,
                descriptors: self.plan.descriptors.clone(),
                values: self.values.clone(),
                projections: projections
                    .iter()
                    .map(|p| RetainedFunctionalProjection {
                        index: p.index,
                        value: p.data.value,
                        interval: p.data.interval,
                        absolute_error: p.data.absolute,
                        relative_error: p.data.relative,
                        relative_limit: p.relative_limit,
                        basis: p.data.basis,
                        exact_sign: p.exact_sign(),
                    })
                    .collect(),
            })
        })
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct RetainedFunctionalProjection {
    index: usize,
    value: f64,
    interval: [f64; 2],
    absolute_error: f64,
    relative_error: f64,
    relative_limit: f64,
    basis: ProjectionBasis,
    exact_sign: Sign,
}
impl RetainedFunctionalProjection {
    pub fn index(&self) -> usize {
        self.index
    }
    pub fn value(&self) -> f64 {
        self.value
    }
    pub fn interval(&self) -> [f64; 2] {
        self.interval
    }
    pub fn absolute_error_bound(&self) -> f64 {
        self.absolute_error
    }
    pub fn relative_error_bound(&self) -> f64 {
        self.relative_error
    }
    pub fn relative_limit(&self) -> f64 {
        self.relative_limit
    }
    pub fn basis(&self) -> ProjectionBasis {
        self.basis
    }
    pub fn exact_sign(&self) -> Sign {
        self.exact_sign
    }
    fn matches(&self, q: &ProjectionData) -> bool {
        self.value.to_bits() == q.value.to_bits()
            && same_terms(&self.interval, &q.interval)
            && self.absolute_error.to_bits() == q.absolute.to_bits()
            && self.relative_error.to_bits() == q.relative.to_bits()
            && self.basis == q.basis
    }
}
/// Owned companion; its contained response and plan are retained together from
/// one live response. There is no import constructor. Inspecting this record is
/// not correspondence with a current invocation; use replay_against for that.
#[derive(Debug, Clone)]
pub struct RetainedFunctionalSet {
    token: Arc<()>,
    response: RetainedResponse,
    descriptors: Vec<FunctionalDescriptor>,
    values: Vec<Ratio>,
    projections: Vec<RetainedFunctionalProjection>,
}
#[derive(Debug)]
pub struct FunctionalReplayCheck {
    dofs: usize,
    blocks: usize,
    functionals: usize,
    dof_projections: usize,
}
impl FunctionalReplayCheck {
    pub fn dofs(&self) -> usize {
        self.dofs
    }
    pub fn blocks(&self) -> usize {
        self.blocks
    }
    pub fn functionals(&self) -> usize {
        self.functionals
    }
    pub fn dof_projections(&self) -> usize {
        self.dof_projections
    }
}
impl RetainedFunctionalSet {
    pub fn response(&self) -> &RetainedResponse {
        &self.response
    }
    pub fn descriptors(&self) -> &[FunctionalDescriptor] {
        &self.descriptors
    }
    pub fn ratios(&self) -> &[Ratio] {
        &self.values
    }
    pub fn projections(&self) -> &[RetainedFunctionalProjection] {
        &self.projections
    }
    /// Compare the complete expected source and ordered recipe against the
    /// immutable private proof. This is a binding check, not numerical replay.
    /// Live finalization performs one full independent captured-source replay,
    /// then uses this check for a second view of the same actual invocation.
    pub fn check_binding_with_budget(
        &self,
        expected: &StructuralSystem<'_>,
        identity: &str,
        force_basis: ForceBasis<'_>,
        expected_descriptors: &[FunctionalDescriptor],
        budget: &mut AttemptBudget,
    ) -> Attempt<()> {
        budget.run(AttemptStage::Replay, |w, token| {
            source_preflight(expected, identity, force_basis, w)?;
            descriptors_charge(expected_descriptors, expected.force.len(), w)?;
            if !Arc::ptr_eq(&self.token, token)
                || self.response.limits != w.limits
                || !self.response.source.matches(expected, identity, force_basis)
                || !same_descriptors(&self.descriptors, expected_descriptors)
            {
                return Err(Error::Invalid("functional complete expected source/plan binding"));
            }
            Ok(())
        })
    }

    /// Independently supply current source AND ordered current recovery recipes.
    /// Rebuild once under this same ledger; compare retained bytes only after
    /// bounded sizing, and never use an unvalidated retained expansion's sign.
    pub fn replay_against(
        &self,
        expected: &StructuralSystem<'_>,
        identity: &str,
        force_basis: ForceBasis<'_>,
        expected_descriptors: &[FunctionalDescriptor],
        expected_contacts: &[(usize, i8, f64)],
        relative_limit: f64,
        budget: &mut AttemptBudget,
    ) -> Attempt<FunctionalReplayCheck> {
        budget.run(AttemptStage::Replay, |w, token| {
            w.charge(24)?;
            if !Arc::ptr_eq(&self.token, token)
                || self.response.limits != w.limits
                || !relative_limit.is_finite()
                || relative_limit < 0.0
            {
                return Err(Error::Invalid("functional replay policy/limits"));
            }
            let n = expected.force.len();
            if n > 256 || n > w.limits.dofs {
                return Err(Error::Budget);
            }
            descriptors_charge(expected_descriptors, n, w)?;
            descriptors_charge(&self.descriptors, n, w)?;
            if !same_descriptors(&self.descriptors, expected_descriptors) {
                return Err(Error::Invalid("functional expected plan binding"));
            }
            if self.values.len() != self.descriptors.len()
                || self.projections.len() != self.descriptors.len()
            {
                return Err(Error::Invalid("functional replay coverage"));
            }
            retained_charge(&self.response, n, w)?;
            for r in &self.values {
                ratio_preflight(r, w)?;
            }
            w.charge(self.projections.len().saturating_mul(16))?;
            let context = prepare(expected, identity, force_basis, token, w)?;
            if !context.matches(
                &self.response.source.system(),
                &self.response.source.identity,
                self.response.source.force_basis(),
            ) {
                return Err(Error::Invalid("functional expected source binding"));
            }
            if context.witnesses.len() != self.response.witnesses.len()
                || !context
                    .witnesses
                    .iter()
                    .zip(&self.response.witnesses)
                    .all(|(a, b)| a.same_bits(b))
            {
                return Err(Error::Invalid("functional retained block/minor mismatch"));
            }
            let response = solve(&context, w)?;
            if response.displacement != self.response.displacement
                || response.reactions != self.response.reactions
            {
                return Err(Error::Invalid("functional retained response mismatch"));
            }
            replay_leaves(
                &response,
                &self.response,
                expected_contacts,
                relative_limit,
                w,
            )?;
            let values = evaluate(&response, expected_descriptors, w)?;
            if values != self.values {
                return Err(Error::Invalid("functional retained ratio mismatch"));
            }
            for (i, (p, r)) in self.projections.iter().zip(&values).enumerate() {
                if p.index != i || p.relative_limit.to_bits() != relative_limit.to_bits() {
                    return Err(Error::Invalid(
                        "functional replay projection identity/policy",
                    ));
                }
                let q = project(w, r, relative_limit)?;
                if !p.matches(&q) || p.exact_sign != r.sign() {
                    return Err(Error::Invalid("functional retained projection mismatch"));
                }
            }
            Ok(FunctionalReplayCheck {
                dofs: n,
                blocks: context.blocks.len(),
                functionals: values.len(),
                dof_projections: 2 * n,
            })
        })
    }
}
fn ratio_preflight(r: &Ratio, w: &mut Work) -> Result<(), Error> {
    w.charge(4)?;
    if r.numerator.terms.len() > w.limits.expansion_terms
        || r.denominator.terms.is_empty()
        || r.denominator.terms.len() > w.limits.expansion_terms
    {
        return Err(Error::Invalid("functional retained ratio dimensions"));
    }
    w.charge(ratio_charge(r).saturating_mul(2))
}
fn retained_charge(r: &RetainedResponse, n: usize, w: &mut Work) -> Result<(), Error> {
    w.charge(8)?;
    if r.source.force.len() != n
        || r.displacement.len() != n
        || r.reactions.len() != n
        || r.projections.len() != 2 * n
        || r.contacts.len() > n
        || r.witnesses.len() > n
    {
        return Err(Error::Invalid("functional retained response dimensions"));
    }
    source_preflight(
        &r.source.system(),
        &r.source.identity,
        r.source.force_basis(),
        w,
    )?;
    for value in r.displacement.iter().chain(&r.reactions) {
        ratio_preflight(value, w)?;
    }
    for b in &r.witnesses {
        w.charge(8)?;
        if b.dofs.is_empty()
            || b.dofs.len() > 2
            || b.scalar_count() > 4 * w.limits.expansion_terms + 2
        {
            return Err(Error::Invalid("functional retained witness dimensions"));
        }
        w.charge(b.scalar_count().saturating_mul(2))?;
    }
    for c in &r.contacts {
        ratio_preflight(&c.penetration, w)?;
        ratio_preflight(&c.normalized_reaction, w)?;
    }
    w.charge(
        r.projections
            .len()
            .saturating_mul(24)
            .saturating_add(r.contacts.len().saturating_mul(16)),
    )
}
fn replay_leaves(
    response: &Response<'_>,
    retained: &RetainedResponse,
    contacts: &[(usize, i8, f64)],
    criterion: f64,
    w: &mut Work,
) -> Result<(), Error> {
    let n = response.displacement.len();
    w.charge(n.saturating_mul(8))?;
    if contacts.len() != retained.contacts.len() {
        return Err(Error::Invalid("functional expected contacts"));
    }
    let mut contact_seen = vec![false; n];
    for (&(d, s, g), c) in contacts.iter().zip(&retained.contacts) {
        if d >= n || contact_seen[d] || d != c.dof || s != c.sense || g.to_bits() != c.gap.to_bits()
        {
            return Err(Error::Invalid("functional expected contact binding"));
        }
        contact_seen[d] = true;
        let fresh = contact_charged(response, d, s, g, w)?;
        if !c.same_bits(&fresh) {
            return Err(Error::Invalid("functional retained contact mismatch"));
        }
    }
    let mut seen = vec![false; 2 * n];
    for p in &retained.projections {
        if p.dof >= n || p.relative_limit.to_bits() != criterion.to_bits() {
            return Err(Error::Invalid("functional retained DOF projection policy"));
        }
        let i = 2 * p.dof + usize::from(p.quantity == Quantity::Reaction);
        if seen[i] {
            return Err(Error::Invalid(
                "functional retained DOF projection coverage",
            ));
        }
        seen[i] = true;
        let r = match p.quantity {
            Quantity::Displacement => &response.displacement[p.dof],
            Quantity::Reaction => &response.reactions[p.dof],
        };
        let q = project(w, r, criterion)?;
        if !p.matches_data(&q) {
            return Err(Error::Invalid(
                "functional retained DOF projection mismatch",
            ));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests;
