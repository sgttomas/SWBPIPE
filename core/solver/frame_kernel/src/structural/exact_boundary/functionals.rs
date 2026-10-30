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
                rejected: super::WorkTotal::zero(),
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
        || context.order_cap != FUNCTIONAL_ORDER_CAP
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
/// Sizes and charges a functional source before it is inspected or copied.
/// RF design §3.3: a congruence member counts its 288 operand values as
/// source terms, beside its 144 rounded contributions (which stay counted),
/// and reserves 8 per scalar (operands, map and range), as a contribution
/// reserves 8 for its three.
fn source_preflight(
    system: &StructuralSystem<'_>,
    identity: &str,
    force: ForceBasis<'_>,
    congruence: &[CongruenceMember],
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
    let count = cs
        .len()
        .saturating_add(if matches!(force, ForceBasis::DeclaredVector) {
            n
        } else {
            fs.len()
        })
        .saturating_add(congruence.len().saturating_mul(CONGRUENCE_VALUES));
    if n > 256 || n > w.limits.dofs || count > 16_384 || count > w.limits.source_terms {
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
            .saturating_add(congruence.len().saturating_mul(CONGRUENCE_SCALARS * 8))
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
    congruence: &[CongruenceMember],
    token: &Arc<()>,
    w: &mut Work,
) -> Result<Context, Error> {
    source_preflight(system, identity, force, congruence, w)?;
    // RF design §2.1: the functional entry and its replay admit order 4.
    let mut c = Context::new_charged(
        system,
        identity,
        force,
        congruence,
        w.limits,
        w,
        FUNCTIONAL_ORDER_CAP,
    )?;
    c.functional_attempt = Some(Arc::clone(token));
    Ok(c)
}
impl Context {
    /// The functional entry. `congruence` lists the members whose exact
    /// entries are their exact congruence Tᵀ·K_local·T, formed in stage
    /// Preparation, in place of their rounded contributions (RF design §3.3);
    /// empty when every member's transform is a signed permutation.
    pub fn prepare_with_budget(
        system: &StructuralSystem<'_>,
        identity: &str,
        force: ForceBasis<'_>,
        congruence: &[CongruenceMember],
        budget: &mut AttemptBudget,
    ) -> Attempt<Self> {
        budget.run(AttemptStage::Preparation, |w, token| {
            prepare(system, identity, force, congruence, token, w)
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
    /// R-A's B1 hints (DEL-04-08 Design `source-work-nondyadic-inputs.md` §3.1): one
    /// optional source-row index per descriptor, or empty. Not hashed and not part of
    /// any descriptor; every hint is verified before use, and a missing or mismatched
    /// one takes the long path.
    hints: Vec<Option<u32>>,
}
/// R-A's B1 hint rule: the caller's map from a descriptor's index to its candidate
/// source row. The kernel charges it before it builds the hints from it.
pub type HintRule<'h> = &'h dyn Fn(usize) -> Option<u32>;
/// The hints a rule gives for `count` descriptors, charged before they are built:
/// one unit per descriptor for the rule's evaluation and one for its store.
fn build_hints(rule: HintRule<'_>, count: usize, w: &mut Work) -> Result<Vec<Option<u32>>, Error> {
    w.charge(count.saturating_mul(2))?;
    Ok((0..count).map(rule).collect())
}
impl<'a> FunctionalPlan<'a> {
    pub fn new(
        context: &'a Context,
        descriptors: &[FunctionalDescriptor],
        budget: &mut AttemptBudget,
    ) -> Attempt<Self> {
        Self::plan(context, descriptors, None, budget)
    }
    /// [`FunctionalPlan::new`] with R-A's B1 hints, built here from `rule` after
    /// their charge.
    pub fn new_with_hints(
        context: &'a Context,
        descriptors: &[FunctionalDescriptor],
        rule: HintRule<'_>,
        budget: &mut AttemptBudget,
    ) -> Attempt<Self> {
        Self::plan(context, descriptors, Some(rule), budget)
    }
    fn plan(
        context: &'a Context,
        descriptors: &[FunctionalDescriptor],
        rule: Option<HintRule<'_>>,
        budget: &mut AttemptBudget,
    ) -> Attempt<Self> {
        budget.run(AttemptStage::Plan, |w, token| {
            binding(context, token, w)?;
            descriptors_charge(descriptors, context.f.len(), w)?;
            let hints = match rule {
                Some(rule) => build_hints(rule, descriptors.len(), w)?,
                None => Vec::new(),
            };
            Ok(Self {
                context,
                descriptors: descriptors.to_vec(),
                hints,
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
/// The functional evaluator. With `shortcuts`, R-A's B1 and B2 apply (DEL-04-08 Design
/// `source-work-nondyadic-inputs.md` §3.1): both product callers pass `true`; only the
/// test-only reference entry passes `false`, which is the long evaluation.
fn evaluate(
    response: &Response<'_>,
    ds: &[FunctionalDescriptor],
    hints: &[Option<u32>],
    shortcuts: bool,
    w: &mut Work,
) -> Result<Vec<Ratio>, Error> {
    let c = response.context;
    let n = c.f.len();
    w.charge(
        ds.len()
            .saturating_mul(n.saturating_mul(4).saturating_add(8)),
    )?;
    // RF design §2.4: a block's numerators are combined below over its first
    // DOF's denominator, which is sound only if every DOF of the block carries
    // the bit-identical denominator. Checked, and charged, for every block of
    // order 3 or 4; blocks of order 1 or 2 keep today's path.
    for block in c.blocks.iter().filter(|block| block.len() > 2) {
        let denominators = block.iter().map(|&i| &response.displacement[i].denominator.terms);
        w.charge(denominators.clone().fold(0usize, |n, d| n.saturating_add(d.len())))?;
        let first = &response.displacement[block[0]].denominator.terms;
        if denominators.into_iter().any(|d| !same_terms(first, d)) {
            return Err(Error::Invalid("functional block denominators"));
        }
    }
    let mut ra = shortcuts::Shortcuts::start(c, ds, hints, shortcuts, w)?;
    #[cfg(any(test, feature = "exact-operation-count"))]
    exact_operation_count::begin_evaluation();
    let mut values = Vec::with_capacity(ds.len());
    for (index, d) in ds.iter().enumerate() {
        if let Some(q) = ra.negated_row(index, d, ds, &values, w)? {
            #[cfg(any(test, feature = "exact-operation-count"))]
            exact_operation_count::row(exact_operation_count::RowPath::Negated);
            values.push(q);
            continue;
        }
        ra.begin_row(d, w)?;
        let mut coefficients = vec![Expansion::default(); n];
        for t in &d.terms {
            if ra.skips(t.dof) {
                continue;
            }
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
        ra.charge_source_checks(n, w)?;
        let fixed_empty = fixed.is_zero();
        let mut q = constant(w, fixed)?;
        let mut contributing = 0usize;
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
            contributing += 1;
            // Context's block solve constructs the SAME positive denominator for
            // every coordinate of a block (checked above for orders 3 and 4).
            // Combine their numerators before any ratio sum.
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
        ra.note(&coefficients, fixed_empty, contributing);
        #[cfg(any(test, feature = "exact-operation-count"))]
        exact_operation_count::row(exact_operation_count::RowPath::Long { b2_skipped: ra.skipped() });
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
            let values = evaluate(self, &plan.descriptors, &plan.hints, true, w)?;
            Ok(FunctionalSet {
                response: self,
                plan,
                values,
            })
        })
    }
    /// The long evaluation, R-A's reference (DEL-04-08 Design
    /// `source-work-nondyadic-inputs.md` §3.2): [`Response::evaluate_functionals`] with
    /// neither B1 nor B2. Test builds only; no product build compiles it.
    #[cfg(any(test, feature = "exact-operation-count"))]
    pub fn evaluate_functionals_long<'r, 'p>(
        &'r self,
        plan: &'p FunctionalPlan<'a>,
        budget: &mut AttemptBudget,
    ) -> Attempt<FunctionalSet<'r, 'p, 'a>> {
        budget.run(AttemptStage::Evaluation, |w, token| {
            response_binding(self, token, w)?;
            if !std::ptr::eq(self.context, plan.context) {
                return Err(Error::Invalid("functional plan response"));
            }
            let values = evaluate(self, &plan.descriptors, &[], false, w)?;
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
            // D6: the remainder cap keeps `used + charged` within the limit; the
            // refused reservations add as one checked total.
            w.used = w.used.checked_add(a.work.charged).ok_or(Error::Budget)?;
            w.rejected = w.rejected.add(a.work.rejected);
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
        expected_congruence: &[CongruenceMember],
        expected_descriptors: &[FunctionalDescriptor],
        budget: &mut AttemptBudget,
    ) -> Attempt<()> {
        budget.run(AttemptStage::Replay, |w, token| {
            source_preflight(expected, identity, force_basis, expected_congruence, w)?;
            descriptors_charge(expected_descriptors, expected.force.len(), w)?;
            if !Arc::ptr_eq(&self.token, token)
                || self.response.limits != w.limits
                || !self
                    .response
                    .source
                    .matches(expected, identity, force_basis, expected_congruence)
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
    /// The expected congruence list is part of the expected source: the
    /// rebuild re-forms its members' exact entries (RF design §3.3).
    #[allow(clippy::too_many_arguments)]
    pub fn replay_against(
        &self,
        expected: &StructuralSystem<'_>,
        identity: &str,
        force_basis: ForceBasis<'_>,
        expected_congruence: &[CongruenceMember],
        expected_descriptors: &[FunctionalDescriptor],
        expected_hints: Option<HintRule<'_>>,
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
            let context = prepare(expected, identity, force_basis, expected_congruence, token, w)?;
            if !context.source.matches(
                &self.response.source.system(),
                &self.response.source.identity,
                self.response.source.force_basis(),
                &self.response.source.congruence,
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
            // The current invocation's hints, rebuilt from its rule as the plan builds them.
            let expected_hints = match expected_hints {
                Some(rule) => build_hints(rule, expected_descriptors.len(), w)?,
                None => Vec::new(),
            };
            let values = evaluate(&response, expected_descriptors, &expected_hints, true, w)?;
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
        &r.source.congruence,
        w,
    )?;
    for value in r.displacement.iter().chain(&r.reactions) {
        ratio_preflight(value, w)?;
    }
    for b in &r.witnesses {
        w.charge(8)?;
        // RF design §2.6: blocks of up to 4 DOFs, each within its order's bound.
        if b.dofs.is_empty()
            || b.dofs.len() > FUNCTIONAL_ORDER_CAP
            || b.scalar_count() > BlockWitness::size_bound(b.dofs.len(), w.limits.expansion_terms)
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

impl FunctionalSet<'_, '_, '_> {
    /// Every evaluated Ratio, in descriptor order, for R-A's gating comparison with the
    /// long evaluation. Test builds only.
    #[cfg(any(test, feature = "exact-operation-count"))]
    pub fn ratios(&self) -> &[Ratio] {
        &self.values
    }
}

/// R-A, the source route's work reduction (DEL-04-08 Design
/// `source-work-nondyadic-inputs.md` §3.1). Both parts change no value: every row they
/// take equals the long evaluation's bit for bit (§3.1's argument; the gating test
/// checks it), and every inspection they add is charged before it runs.
///
/// - **B1, negated rows.** A section row whose hint names its member's i-end row of the
///   same component, and which is that row's products with -1.0 appended (any further
///   terms being an i-end row of the member with ±1 atoms and then an exact zero
///   appended), takes the source row's Ratio with its numerator negated termwise, when
///   the source row had no fixed part, at most one contributing block and no unit
///   coefficient.
/// - **B2, zero-prescribed recipes.** A term on a DOF prescribed at exactly ±0 skips its
///   coefficient recipe when the row's terms on that DOF lie in the refusal-free window:
///   normal atoms with |binary exponent| ≤ 150, at most 5 atoms per product and at
///   most 64 products.
mod shortcuts {
    use super::*;

    fn atoms(p: &Products) -> usize {
        p.iter().map(Vec::len).sum::<usize>() + p.len()
    }
    fn same_atoms(a: &[f64], b: &[f64]) -> bool {
        a.len() == b.len() && a.iter().zip(b).all(|(x, y)| x.to_bits() == y.to_bits())
    }
    /// `d` is `s` with one appended atom -1.0, bitwise.
    fn negated_product(d: &[f64], s: &[f64]) -> bool {
        d.len() == s.len() + 1 && same_atoms(&d[..s.len()], s) && d[s.len()].to_bits() == (-1.0f64).to_bits()
    }
    /// `d` is `r` followed by ±1 atoms, then an exact ±0 atom, then any atoms.
    fn zeroed_product(d: &[f64], r: &[f64]) -> bool {
        if d.len() <= r.len() || !same_atoms(&d[..r.len()], r) {
            return false;
        }
        for &x in &d[r.len()..] {
            if x == 0.0 {
                return true;
            }
            if x.abs() != 1.0 {
                return false;
            }
        }
        false
    }
    /// B2's window for one atom.
    fn in_band(x: f64) -> bool {
        x.is_normal() && (((x.to_bits() >> 52) & 0x7ff) as i32 - 1023).abs() <= 150
    }
    fn i_end_of<'k>(key: &'k FunctionalKey) -> Option<(&'k str, u8)> {
        match &key.quantity {
            FunctionalQuantity::MemberEnd { member, end: MemberEnd::I, row } => Some((member, *row)),
            _ => None,
        }
    }

    pub(super) struct Shortcuts<'h> {
        b1: bool,
        b2: bool,
        hints: &'h [Option<u32>],
        /// DOFs prescribed at exactly ±0, and the current row's window per DOF:
        /// 0 untouched, 1 skip, 2 outside the window.
        zero: Vec<bool>,
        window: Vec<u8>,
        products: Vec<usize>,
        touched: Vec<usize>,
        /// Per evaluated row: whether B1's dynamic conditions held, measured when the
        /// row was evaluated (only for i-end rows, the only possible sources).
        source_ok: Vec<bool>,
        row_source: bool,
        skipped: u32,
    }
    impl<'h> Shortcuts<'h> {
        pub(super) fn start(c: &Context, ds: &[FunctionalDescriptor], hints: &'h [Option<u32>], on: bool, w: &mut Work) -> Result<Self, Error> {
            let n = c.f.len();
            let b1 = on && hints.len() == ds.len();
            let mut b2 = on;
            let (mut zero, mut window, mut products) = (Vec::new(), Vec::new(), Vec::new());
            if b2 {
                // The zero-prescription vector and the window's two per-DOF counters, n
                // entries each, and the read of the prescribed list; then the block scan,
                // one unit per block DOF. A zero-prescribed DOF is never free, so the scan
                // is a guard.
                w.charge(n.saturating_mul(3).saturating_add(c.source.prescribed.len()))?;
                w.charge(c.blocks.iter().map(Vec::len).sum::<usize>())?;
                zero = vec![false; n];
                for &(i, v) in &c.source.prescribed {
                    if v == 0.0 {
                        zero[i] = true;
                    }
                }
                if c.blocks.iter().flatten().any(|&i| zero[i]) {
                    b2 = false;
                } else {
                    window = vec![0; n];
                    products = vec![0; n];
                }
            }
            Ok(Self {
                b1,
                b2,
                hints,
                zero,
                window,
                products,
                touched: Vec::new(),
                source_ok: Vec::with_capacity(if b1 { ds.len() } else { 0 }),
                row_source: false,
                skipped: 0,
            })
        }
        #[cfg(any(test, feature = "exact-operation-count"))]
        pub(super) fn skipped(&self) -> u32 {
            self.skipped
        }
        /// B2's window for a row about to take the long path, charged before it inspects.
        pub(super) fn begin_row(&mut self, d: &FunctionalDescriptor, w: &mut Work) -> Result<(), Error> {
            self.row_source = self.b1 && i_end_of(&d.key).is_some();
            self.skipped = 0;
            if !self.b2 {
                return Ok(());
            }
            for i in self.touched.drain(..) {
                self.window[i] = 0;
            }
            // One unit per term scanned, then one per product and atom of each
            // zero-prescribed term, charged before its atoms are read.
            w.charge(d.terms.len())?;
            for t in d.terms.iter().filter(|t| self.zero[t.dof]) {
                w.charge(atoms(&t.products))?;
                if self.window[t.dof] == 0 {
                    self.window[t.dof] = 1;
                    self.products[t.dof] = 0;
                    self.touched.push(t.dof);
                }
                self.products[t.dof] += t.products.len();
                if t.products.iter().any(|p| p.len() > 5 || !p.iter().all(|&x| in_band(x))) {
                    self.window[t.dof] = 2;
                }
            }
            for &i in &self.touched {
                if self.products[i] > 64 {
                    self.window[i] = 2;
                }
            }
            Ok(())
        }
        pub(super) fn skips(&mut self, dof: usize) -> bool {
            if self.b2 && self.window[dof] == 1 {
                self.skipped += 1;
                return true;
            }
            false
        }
        /// B1's negated row, or `None` for the long path.
        pub(super) fn negated_row(
            &mut self,
            index: usize,
            d: &FunctionalDescriptor,
            ds: &[FunctionalDescriptor],
            values: &[Ratio],
            w: &mut Work,
        ) -> Result<Option<Ratio>, Error> {
            if !self.b1 {
                return Ok(None);
            }
            // A declined row takes the long path, whose `note` records it.
            let decline = |_: &mut Self| Ok(None);
            let FunctionalQuantity::MemberSection { member, component, .. } = &d.key.quantity else {
                return Ok(None);
            };
            let Some(j) = self.hints[index].map(|j| j as usize).filter(|&j| j < index) else {
                return Ok(None);
            };
            let s = &ds[j];
            // The hint's key check, charged its key bytes.
            w.charge(key_bytes(&s.key))?;
            if s.key.case_id != d.key.case_id || i_end_of(&s.key) != Some((member.as_str(), *component)) {
                return decline(self);
            }
            // Static recognition, charged before it runs: one unit per compared atom and
            // term of both rows.
            w.charge(
                atoms(&s.offset)
                    .saturating_add(atoms(&d.offset))
                    .saturating_add(s.terms.len())
                    .saturating_add(s.terms.iter().map(|t| atoms(&t.products)).sum::<usize>())
                    .saturating_add(d.terms.len())
                    .saturating_add(d.terms.iter().map(|t| atoms(&t.products)).sum::<usize>()),
            )?;
            // (1) d's first terms, and its offset, are s's with -1.0 appended to every product.
            if d.offset.len() != s.offset.len()
                || !d.offset.iter().zip(&s.offset).all(|(a, b)| negated_product(a, b))
                || d.terms.len() < s.terms.len()
                || !d.terms.iter().zip(&s.terms).all(|(a, b)| {
                    a.dof == b.dof
                        && a.products.len() == b.products.len()
                        && a.products.iter().zip(&b.products).all(|(x, y)| negated_product(x, y))
                })
            {
                return decline(self);
            }
            // (3) every product of s has at most two atoms, and no atom of s is ±1.
            let short = |p: &Products| p.iter().all(|p| p.len() <= 2 && p.iter().all(|x| x.abs() != 1.0));
            if !short(&s.offset) || !s.terms.iter().all(|t| short(&t.products)) {
                return decline(self);
            }
            // (2) any further terms are an earlier i-end row of the same member with ±1
            // atoms and then an exact zero appended to every product: that row was
            // evaluated, so the long path could not refuse before the zero.
            let extra = &d.terms[s.terms.len()..];
            if !extra.is_empty() {
                let first = j.saturating_sub(*component as usize);
                let mut found = false;
                for r in &ds[first..=j] {
                    w.charge(
                        key_bytes(&r.key)
                            .saturating_add(r.terms.len())
                            .saturating_add(r.terms.iter().map(|t| atoms(&t.products)).sum::<usize>()),
                    )?;
                    if r.key.case_id != d.key.case_id || !i_end_of(&r.key).is_some_and(|(m, _)| m == member) {
                        continue;
                    }
                    if r.terms.len() == extra.len()
                        && r.terms.iter().zip(extra).all(|(a, b)| {
                            a.dof == b.dof
                                && a.products.len() == b.products.len()
                                && b.products.iter().zip(&a.products).all(|(x, y)| zeroed_product(x, y))
                        })
                    {
                        found = true;
                        break;
                    }
                }
                if !found {
                    return decline(self);
                }
            }
            // The dynamic conditions, recorded when s was evaluated, read for one unit.
            w.charge(1)?;
            if !self.source_ok.get(j).copied().unwrap_or(false) {
                return decline(self);
            }
            let source = &values[j];
            // The termwise negation and the shared denominator's clone, charged their lengths.
            w.charge(source.numerator.terms.len().saturating_add(source.denominator.terms.len()))?;
            let q = Ratio {
                numerator: Expansion {
                    terms: source.numerator.terms.iter().map(|x| -x).collect(),
                    operations: source.numerator.operations,
                },
                denominator: source.denominator.clone(),
            };
            self.source_ok.push(false);
            Ok(Some(q))
        }
        /// B1's dynamic checks on a candidate source row, charged before they inspect:
        /// one unit per coefficient, one for the fixed part and one for the block count,
        /// which reuses the long path's own zero test of each block numerator.
        pub(super) fn charge_source_checks(&self, n: usize, w: &mut Work) -> Result<(), Error> {
            if self.row_source {
                w.charge(n.saturating_add(2))?;
            }
            Ok(())
        }
        pub(super) fn note(&mut self, coefficients: &[Expansion], fixed_empty: bool, contributing: usize) {
            if self.b1 {
                let ok = self.row_source
                    && fixed_empty
                    && contributing <= 1
                    && !coefficients.iter().any(|e| e.terms.len() == 1 && e.terms[0].abs() == 1.0);
                self.source_ok.push(ok);
            }
        }
    }
}

#[cfg(test)]
mod tests;
#[cfg(test)]
mod shortcut_tests;
#[cfg(test)]
mod block_order_tests;
#[cfg(test)]
pub(super) mod congruence_tests;
