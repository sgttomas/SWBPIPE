//! Bounded exact predicates for the declared, represented source equations.
//!
//! This is not the ordinary structural gate, a primitive-engineering certificate,
//! or a replacement for it. All free connected components must have order <= 2.
//! Private owned source and private response construction prevent caller-supplied
//! reports from serving as proofs. Projection is explicitly distinct from proof.
use super::{Expansion, StiffnessContribution, StructuralError, StructuralSystem};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sign {
    Negative,
    Zero,
    Positive,
}
#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    Invalid(&'static str),
    UnsupportedBlock { order: usize },
    NotPositiveDefinite,
    Budget,
    Arithmetic(StructuralError),
    ProjectionUnresolved(&'static str),
}
impl From<StructuralError> for Error {
    fn from(e: StructuralError) -> Self {
        Self::Arithmetic(e)
    }
}
/// Identity is provenance, not an assertion of primitive engineering exactness.
#[derive(Debug, Clone, PartialEq)]
pub struct ForceContribution {
    pub source: String,
    pub dof: usize,
    pub value: f64,
}
#[derive(Debug, Clone, Copy)]
pub enum ForceBasis<'a> {
    /// The vector itself is the declared source; no unrecorded load tails claimed.
    DeclaredVector,
    /// Complete identified represented contributions, including absorbed tails.
    IdentifiedContributions(&'a [ForceContribution]),
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ForceLevel {
    DeclaredVector,
    IdentifiedRepresentedContributions,
}
#[derive(Debug, Clone, Copy)]
pub struct Limits {
    pub dofs: usize,
    pub source_terms: usize,
    pub expansion_terms: usize,
    pub operations: usize,
}
impl Default for Limits {
    fn default() -> Self {
        Self {
            dofs: 256,
            source_terms: 16_384,
            expansion_terms: 256,
            operations: 2_000_000,
        }
    }
}
#[derive(Debug)]
struct Work {
    limits: Limits,
    used: usize,
    rejected: usize,
}
impl Work {
    fn charge(&mut self, n: usize) -> Result<(), Error> {
        let next = self.used.checked_add(n);
        if next.is_none() || next.is_some_and(|x| x > self.limits.operations) {
            self.rejected = self.rejected.saturating_add(n);
            return Err(Error::Budget);
        }
        self.used = next.unwrap();
        Ok(())
    }
    fn add(&mut self, a: &mut Expansion, b: f64) -> Result<(), Error> {
        // Bound the worst next expansion size and operation count before work.
        if a.terms.len() >= self.limits.expansion_terms {
            return Err(Error::Budget);
        }
        self.charge(
            a.terms
                .len()
                .checked_mul(6)
                .and_then(|n| n.checked_add(1))
                .ok_or(Error::Budget)?,
        )?;
        a.add(b)?;
        Ok(())
    }
    fn sum(&mut self, a: &Expansion, b: &Expansion, sense: f64) -> Result<Expansion, Error> {
        let mut out = a.clone();
        for &v in &b.terms {
            self.add(&mut out, sense * v)?;
        }
        Ok(out)
    }
    fn scalar(&mut self, v: f64) -> Result<Expansion, Error> {
        let mut x = Expansion::default();
        self.add(&mut x, v)?;
        Ok(x)
    }
    fn mul(&mut self, a: &Expansion, b: &Expansion) -> Result<Expansion, Error> {
        let pairs = a
            .terms
            .len()
            .checked_mul(b.terms.len())
            .ok_or(Error::Budget)?;
        // Reserve a conservative full product workload before any product.
        self.charge(
            pairs
                .checked_mul(12 * self.limits.expansion_terms + 128)
                .ok_or(Error::Budget)?,
        )?;
        let mut out = Expansion::default();
        for &x in &a.terms {
            for &y in &b.terms {
                if out.terms.len().checked_add(2).ok_or(Error::Budget)?
                    > self.limits.expansion_terms
                {
                    return Err(Error::Budget);
                }
                out.add_product(x, y, 0)?;
            }
        }
        Ok(out)
    }
}
fn sign(e: &Expansion) -> Sign {
    // grow-expansion preserves the parent's nonoverlapping increasing-magnitude
    // representation. Its final nonzero component determines the exact sign.
    match e.terms.last() {
        None => Sign::Zero,
        Some(x) if *x > 0.0 => Sign::Positive,
        Some(_) => Sign::Negative,
    }
}
/// Exact numerator / strictly positive exact denominator. No public constructor.
#[derive(Debug, Clone)]
pub struct Ratio {
    numerator: Expansion,
    denominator: Expansion,
}
impl Ratio {
    pub fn numerator_terms(&self) -> &[f64] {
        &self.numerator.terms
    }
    pub fn denominator_terms(&self) -> &[f64] {
        &self.denominator.terms
    }
    pub fn sign(&self) -> Sign {
        sign(&self.numerator)
    }
    /// A proposal only: no error bound or correct rounding claim. Exact signs
    /// must be obtained from the ratio, never this projection.
    pub fn approximate_projection(&self) -> Result<f64, Error> {
        let n: f64 = self.numerator.terms.iter().sum();
        let d: f64 = self.denominator.terms.iter().sum();
        let q = n / d;
        if !n.is_finite() || !d.is_finite() || d <= 0.0 || !q.is_finite() {
            return Err(Error::Arithmetic(StructuralError::Range(
                "exact ratio projection",
            )));
        }
        Ok(q)
    }
}
fn ratio_add(w: &mut Work, a: &Ratio, b: &Ratio) -> Result<Ratio, Error> {
    let an = w.mul(&a.numerator, &b.denominator)?;
    let bn = w.mul(&b.numerator, &a.denominator)?;
    Ok(Ratio {
        numerator: w.sum(&an, &bn, 1.0)?,
        denominator: w.mul(&a.denominator, &b.denominator)?,
    })
}
fn constant(w: &mut Work, x: Expansion) -> Result<Ratio, Error> {
    Ok(Ratio {
        numerator: x,
        denominator: w.scalar(1.0)?,
    })
}
#[derive(Debug)]
struct Snapshot {
    identity: String,
    stiffness: Vec<Vec<f64>>,
    force: Vec<f64>,
    free: Vec<usize>,
    prescribed: Vec<(usize, f64)>,
    contributions: Vec<StiffnessContribution>,
    force_terms: Option<Vec<ForceContribution>>,
    symmetry: Option<(Vec<Vec<f64>>, Vec<Vec<usize>>, String)>,
}
/// Immutable source-bound exact system. Constructing it performs the exact SPD
/// warrant for every original free block; it does not run/replace the F64 gate.
#[derive(Debug)]
pub struct Context {
    source: Snapshot,
    k: Vec<Vec<Expansion>>,
    f: Vec<Expansion>,
    blocks: Vec<Vec<usize>>,
    limits: Limits,
    preparation_operations: usize,
}
impl Context {
    pub fn new(
        system: &StructuralSystem<'_>,
        identity: &str,
        force_basis: ForceBasis<'_>,
        limits: Limits,
    ) -> Result<Self, Error> {
        let n = system.force.len();
        if n > 256
            || n > limits.dofs
            || limits.expansion_terms > 4096
            || limits.expansion_terms < 2
            || limits.operations == 0
        {
            return Err(Error::Budget);
        }
        if identity.is_empty()
            || system.stiffness.len() != n
            || system.stiffness.iter().any(|r| r.len() != n)
        {
            return Err(Error::Invalid("source identity or dimensions"));
        }
        if system
            .force
            .iter()
            .chain(system.stiffness.iter().flatten())
            .any(|x| !x.is_finite())
        {
            return Err(Error::Invalid("nonfinite source"));
        }
        let mut seen = vec![false; n];
        for &i in system.free_dofs {
            if i >= n || seen[i] {
                return Err(Error::Invalid("free map"));
            }
            seen[i] = true;
        }
        for &(i, v) in system.prescribed {
            if i >= n || seen[i] || !v.is_finite() {
                return Err(Error::Invalid("prescribed map"));
            }
            seen[i] = true;
        }
        if seen.iter().any(|x| !x) {
            return Err(Error::Invalid("incomplete original partition"));
        }
        let contributions = system
            .contributions
            .ok_or(Error::Invalid("complete stiffness contributions required"))?;
        let force_terms = match force_basis {
            ForceBasis::DeclaredVector => None,
            ForceBasis::IdentifiedContributions(t) => Some(t),
        };
        let count = contributions
            .len()
            .checked_add(force_terms.map_or(n, |t| t.len()))
            .ok_or(Error::Budget)?;
        if count > 16_384 || count > limits.source_terms {
            return Err(Error::Budget);
        }
        let mut w = Work {
            limits,
            used: 0,
            rejected: 0,
        };
        w.charge(n.checked_mul(n).ok_or(Error::Budget)?)?;
        let mut k = vec![vec![Expansion::default(); n]; n];
        let mut ordered_k = vec![vec![0.0_f64; n]; n];
        for c in contributions {
            if c.row >= n || c.col >= n || !c.value.is_finite() {
                return Err(Error::Invalid("stiffness contribution"));
            }
            w.add(&mut k[c.row][c.col], c.value)?;
            ordered_k[c.row][c.col] += c.value;
            if !ordered_k[c.row][c.col].is_finite() {
                return Err(Error::Invalid("nonfinite ordered assembly"));
            }
        }
        // The represented aggregate is checked as source coverage evidence only;
        // exact graph and predicates use unrounded expansions below.
        for i in 0..n {
            for j in 0..n {
                let projected: f64 = k[i][j].terms.iter().sum();
                if !projected.is_finite()
                    || (projected != system.stiffness[i][j]
                        && ordered_k[i][j] != system.stiffness[i][j])
                {
                    return Err(Error::Invalid("stiffness source mismatch"));
                }
                if !w.sum(&k[i][j], &k[j][i], -1.0)?.is_zero() {
                    return Err(Error::Invalid("exact source asymmetry"));
                }
            }
        }
        let mut f = vec![Expansion::default(); n];
        let mut ordered_f = vec![0.0_f64; n];
        if let Some(terms) = force_terms {
            for t in terms {
                if t.source.is_empty() || t.dof >= n || !t.value.is_finite() {
                    return Err(Error::Invalid("force contribution"));
                }
                w.add(&mut f[t.dof], t.value)?;
                ordered_f[t.dof] += t.value;
                if !ordered_f[t.dof].is_finite() {
                    return Err(Error::Invalid("nonfinite ordered force"));
                }
            }
            for i in 0..n {
                let projected: f64 = f[i].terms.iter().sum();
                if !projected.is_finite()
                    || (projected != system.force[i] && ordered_f[i] != system.force[i])
                {
                    return Err(Error::Invalid("force source mismatch"));
                }
            }
        } else {
            for i in 0..n {
                w.add(&mut f[i], system.force[i])?;
            }
        }
        let mut visited = vec![false; n];
        let mut blocks = Vec::new();
        for &i in system.free_dofs {
            if visited[i] {
                continue;
            }
            let mut block = vec![i];
            visited[i] = true;
            let mut cursor = 0;
            while cursor < block.len() {
                let row = block[cursor];
                cursor += 1;
                for &j in system.free_dofs {
                    if !visited[j] && !k[row][j].is_zero() {
                        visited[j] = true;
                        block.push(j);
                    }
                }
            }
            if block.len() > 2 {
                return Err(Error::UnsupportedBlock { order: block.len() });
            }
            if sign(&k[i][i]) != Sign::Positive {
                return Err(Error::NotPositiveDefinite);
            }
            if block.len() == 2 {
                let j = block[1];
                let ac = w.mul(&k[i][i], &k[j][j])?;
                let bb = w.mul(&k[i][j], &k[i][j])?;
                if sign(&w.sum(&ac, &bb, -1.0)?) != Sign::Positive {
                    return Err(Error::NotPositiveDefinite);
                }
            }
            blocks.push(block);
        }
        Ok(Self {
            source: Snapshot {
                identity: identity.into(),
                stiffness: system.stiffness.to_vec(),
                force: system.force.to_vec(),
                free: system.free_dofs.to_vec(),
                prescribed: system.prescribed.to_vec(),
                contributions: contributions.to_vec(),
                force_terms: force_terms.map(|x| x.to_vec()),
                symmetry: system.symmetry.as_ref().map(|s| {
                    (
                        s.absolute_roundoff.to_vec(),
                        s.operation_counts.to_vec(),
                        s.basis.to_owned(),
                    )
                }),
            },
            k,
            f,
            blocks,
            limits,
            preparation_operations: w.used,
        })
    }
    pub fn free_dofs(&self) -> &[usize] {
        &self.source.free
    }
    pub fn prescribed(&self) -> &[(usize, f64)] {
        &self.source.prescribed
    }
    pub fn dof_count(&self) -> usize {
        self.f.len()
    }
    pub fn source_identity(&self) -> &str {
        &self.source.identity
    }
    pub fn force_level(&self) -> ForceLevel {
        if self.source.force_terms.is_some() {
            ForceLevel::IdentifiedRepresentedContributions
        } else {
            ForceLevel::DeclaredVector
        }
    }
    /// Ordered identity check: any changed stiffness/source spring, load, map,
    /// contribution order, or caller source identity invalidates this context.
    pub fn matches(
        &self,
        system: &StructuralSystem<'_>,
        identity: &str,
        force_basis: ForceBasis<'_>,
    ) -> bool {
        let same = |a: f64, b: f64| a.to_bits() == b.to_bits();
        self.source.identity == identity
            && self.source.stiffness.len() == system.stiffness.len()
            && self
                .source
                .stiffness
                .iter()
                .zip(system.stiffness)
                .all(|(a, b)| a.len() == b.len() && a.iter().zip(b).all(|(&x, &y)| same(x, y)))
            && self.source.force.len() == system.force.len()
            && self
                .source
                .force
                .iter()
                .zip(system.force)
                .all(|(&x, &y)| same(x, y))
            && self.source.free == system.free_dofs
            && self.source.prescribed.len() == system.prescribed.len()
            && self
                .source
                .prescribed
                .iter()
                .zip(system.prescribed)
                .all(|(&(i, x), &(j, y))| i == j && same(x, y))
            && system.contributions.is_some_and(|cs| {
                cs.len() == self.source.contributions.len()
                    && cs
                        .iter()
                        .zip(&self.source.contributions)
                        .all(|(a, b)| a.row == b.row && a.col == b.col && same(a.value, b.value))
            })
            && match (&self.source.symmetry, &system.symmetry) {
                (None, None) => true,
                (Some((a, counts, basis)), Some(b)) => {
                    basis == b.basis
                        && counts == b.operation_counts
                        && a.len() == b.absolute_roundoff.len()
                        && a.iter().zip(b.absolute_roundoff).all(|(x, y)| {
                            x.len() == y.len() && x.iter().zip(y).all(|(&p, &q)| same(p, q))
                        })
                }
                _ => false,
            }
            && match (&self.source.force_terms, force_basis) {
                (None, ForceBasis::DeclaredVector) => true,
                (Some(a), ForceBasis::IdentifiedContributions(b)) => {
                    a.len() == b.len()
                        && a.iter().zip(b).all(|(x, y)| {
                            x.source == y.source && x.dof == y.dof && same(x.value, y.value)
                        })
                }
                _ => false,
            }
    }
    pub fn solve(&self) -> Result<Response<'_>, Error> {
        let mut w = Work {
            limits: self.limits,
            used: self.preparation_operations,
            rejected: 0,
        };
        let n = self.f.len();
        let zero = w.scalar(0.0)?;
        let mut u = vec![constant(&mut w, zero)?; n];
        for &(i, v) in &self.source.prescribed {
            let value = w.scalar(v)?;
            u[i] = constant(&mut w, value)?;
        }
        for block in &self.blocks {
            let mut h = Vec::new();
            for &i in block {
                let mut hi = self.f[i].clone();
                for &(j, v) in &self.source.prescribed {
                    let value = w.scalar(v)?;
                    let p = w.mul(&self.k[i][j], &value)?;
                    hi = w.sum(&hi, &p, -1.0)?;
                }
                h.push(hi);
            }
            let i = block[0];
            if block.len() == 1 {
                u[i] = Ratio {
                    numerator: h.remove(0),
                    denominator: self.k[i][i].clone(),
                };
            } else {
                let j = block[1];
                let ac = w.mul(&self.k[i][i], &self.k[j][j])?;
                let bb = w.mul(&self.k[i][j], &self.k[i][j])?;
                let d = w.sum(&ac, &bb, -1.0)?;
                let ch = w.mul(&self.k[j][j], &h[0])?;
                let bh = w.mul(&self.k[i][j], &h[1])?;
                u[i] = Ratio {
                    numerator: w.sum(&ch, &bh, -1.0)?,
                    denominator: d.clone(),
                };
                let ah = w.mul(&self.k[i][i], &h[1])?;
                let bh = w.mul(&self.k[i][j], &h[0])?;
                u[j] = Ratio {
                    numerator: w.sum(&ah, &bh, -1.0)?,
                    denominator: d,
                };
            }
        }
        let mut reactions = Vec::with_capacity(n);
        for i in 0..n {
            let empty = Expansion::default();
            let negf = w.sum(&empty, &self.f[i], -1.0)?;
            let mut r = constant(&mut w, negf)?;
            for (j, uj) in u.iter().enumerate() {
                if self.k[i][j].is_zero() {
                    continue;
                }
                let term = Ratio {
                    numerator: w.mul(&self.k[i][j], &uj.numerator)?,
                    denominator: uj.denominator.clone(),
                };
                r = ratio_add(&mut w, &r, &term)?;
            }
            reactions.push(r);
        }
        // Non-vacuous for all original free rows, regardless of contact proposal.
        if self
            .source
            .free
            .iter()
            .any(|&i| reactions[i].sign() != Sign::Zero)
        {
            return Err(Error::Invalid("exact original equilibrium failure"));
        }
        Ok(Response {
            context: self,
            displacement: u,
            reactions,
            operations: w.used,
        })
    }
}
#[derive(Debug)]
pub struct Response<'a> {
    context: &'a Context,
    displacement: Vec<Ratio>,
    reactions: Vec<Ratio>,
    operations: usize,
}
/// Per-call conservative algorithmic work reservations, not hardware operation
/// counts. Charges include bounded scalar checks, loops and exact arithmetic.
/// `rejected` is an attempted budget reservation that was NOT executed. A range
/// failure retains its accepted precharge, even if arithmetic stopped early.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WorkReport {
    pub charged: usize,
    pub rejected: usize,
    pub limit: usize,
}
#[derive(Debug)]
pub struct Attempt<T> {
    pub result: Result<T, Error>,
    pub work: WorkReport,
}
fn metered<T>(
    limits: Limits,
    requested_limit: usize,
    f: impl FnOnce(&mut Work) -> Result<T, Error>,
) -> Attempt<T> {
    let mut w = Work {
        limits: Limits {
            operations: requested_limit.min(limits.operations),
            ..limits
        },
        used: 0,
        rejected: 0,
    };
    let result = f(&mut w);
    Attempt {
        result,
        work: WorkReport {
            charged: w.used,
            rejected: w.rejected,
            limit: w.limits.operations,
        },
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProjectionBasis {
    ExactZero,
    ExactIdentity,
    OutwardInterval,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Quantity {
    Displacement,
    Reaction,
}
/// A privately constructed, immutable scalar publication certificate tied to one
/// response and one of its ratios. A bound is relative to that represented source
/// only. It neither proves correct rounding nor qualifies an engineering model.
#[derive(Debug)]
pub struct QualifiedProjection<'r, 'a> {
    response: &'r Response<'a>,
    quantity: Quantity,
    dof: usize,
    value: f64,
    interval: [f64; 2],
    absolute_error: f64,
    relative_error: f64,
    relative_limit: f64,
    basis: ProjectionBasis,
}
impl QualifiedProjection<'_, '_> {
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
    pub fn quantity(&self) -> Quantity {
        self.quantity
    }
    pub fn dof(&self) -> usize {
        self.dof
    }
    pub fn exact_sign(&self) -> Sign {
        match self.quantity {
            Quantity::Displacement => self.response.displacement[self.dof].sign(),
            Quantity::Reaction => self.response.reactions[self.dof].sign(),
        }
    }
    /// A different solve, ratio, DOF, or source cannot reuse this certificate.
    pub fn is_for(&self, response: &Response<'_>, quantity: Quantity, dof: usize) -> bool {
        std::ptr::eq(self.response, response) && self.quantity == quantity && self.dof == dof
    }
}
fn next_up(x: f64) -> f64 {
    if x == 0.0 {
        f64::from_bits(1)
    } else if x > 0.0 {
        f64::from_bits(x.to_bits() + 1)
    } else {
        f64::from_bits(x.to_bits() - 1)
    }
}
fn next_down(x: f64) -> f64 {
    -next_up(-x)
}
fn proposal(w: &mut Work, r: &Ratio) -> Result<f64, Error> {
    w.charge(
        r.numerator
            .terms
            .len()
            .saturating_add(r.denominator.terms.len())
            .saturating_add(16),
    )?;
    let q = r.approximate_projection()?;
    if q == 0.0 || (q > 0.0) != (r.sign() == Sign::Positive) {
        return Err(Error::ProjectionUnresolved(
            "nonzero projection underflow/sign",
        ));
    }
    Ok(q)
}
fn enclose(w: &mut Work, e: &Expansion) -> Result<[f64; 2], Error> {
    let mut lo = 0.0;
    let mut hi = 0.0;
    for &term in &e.terms {
        w.charge(12)?;
        let l = lo + term;
        let h = hi + term;
        if !l.is_finite() || !h.is_finite() {
            return Err(Error::ProjectionUnresolved("sum interval overflow"));
        }
        lo = next_down(l);
        hi = next_up(h);
        if !lo.is_finite() || !hi.is_finite() {
            return Err(Error::ProjectionUnresolved("sum outward range"));
        }
    }
    Ok([lo, hi])
}
#[derive(Debug)]
struct ProjectionData {
    value: f64,
    interval: [f64; 2],
    absolute: f64,
    relative: f64,
    basis: ProjectionBasis,
}
fn project(w: &mut Work, r: &Ratio, relative_limit: f64) -> Result<ProjectionData, Error> {
    w.charge(16)?;
    if !relative_limit.is_finite() || relative_limit < 0.0 {
        return Err(Error::Invalid("projection relative criterion"));
    }
    if r.sign() == Sign::Zero {
        return Ok(ProjectionData {
            value: 0.0,
            interval: [0.0, 0.0],
            absolute: 0.0,
            relative: 0.0,
            basis: ProjectionBasis::ExactZero,
        });
    }
    let mut q = proposal(w, r)?;
    let qe = w.scalar(q)?;
    // A range failure means this shortcut cannot prove identity; an outward
    // interval may still prove accuracy (notably gradual-underflow quotients).
    // Budget failures cannot be bypassed by switching arithmetic routes.
    let identity = (|| {
        let product = w.mul(&qe, &r.denominator)?;
        Ok::<bool, Error>(w.sum(&r.numerator, &product, -1.0)?.is_zero())
    })();
    match identity {
        Ok(true) => {
            return Ok(ProjectionData {
                value: q,
                interval: [q, q],
                absolute: 0.0,
                relative: 0.0,
                basis: ProjectionBasis::ExactIdentity,
            })
        }
        Ok(false) | Err(Error::Arithmetic(_)) => (),
        Err(e) => return Err(e),
    }
    let [nl, nu] = enclose(w, &r.numerator)?;
    let [dl, du] = enclose(w, &r.denominator)?;
    w.charge(48)?;
    if dl <= 0.0 || (nl <= 0.0 && nu >= 0.0) {
        return Err(Error::ProjectionUnresolved("source interval crosses zero"));
    }
    // ALL four signed quotient endpoints; negative numerators reverse the
    // denominator ordering. Positive-only endpoint formulas are unsound here.
    let corners = [nl / dl, nl / du, nu / dl, nu / du];
    if corners.iter().any(|x| !x.is_finite()) {
        return Err(Error::ProjectionUnresolved("quotient interval overflow"));
    }
    let lo = next_down(corners.iter().copied().fold(f64::INFINITY, f64::min));
    let hi = next_up(corners.iter().copied().fold(f64::NEG_INFINITY, f64::max));
    if !lo.is_finite() || !hi.is_finite() || (lo <= 0.0 && hi >= 0.0) {
        return Err(Error::ProjectionUnresolved("quotient outward range/zero"));
    }
    q = q.max(lo).min(hi);
    if !q.is_finite() || q == 0.0 || (q > 0.0) != (r.sign() == Sign::Positive) {
        return Err(Error::ProjectionUnresolved("enclosed projection sign"));
    }
    w.charge(32)?;
    let left = q - lo;
    let right = hi - q;
    if !left.is_finite() || !right.is_finite() || left < 0.0 || right < 0.0 {
        return Err(Error::ProjectionUnresolved("absolute error range"));
    }
    let absolute = next_up(left).max(next_up(right));
    let magnitude = lo.abs().min(hi.abs());
    let rel = absolute / magnitude;
    if !absolute.is_finite() || !rel.is_finite() {
        return Err(Error::ProjectionUnresolved("relative error range"));
    }
    let relative = next_up(rel);
    if !relative.is_finite() || relative > relative_limit {
        return Err(Error::ProjectionUnresolved(
            "relative criterion not established",
        ));
    }
    Ok(ProjectionData {
        value: q,
        interval: [lo, hi],
        absolute,
        relative,
        basis: ProjectionBasis::OutwardInterval,
    })
}
impl<'a> Response<'a> {
    pub fn context(&self) -> &'a Context {
        self.context
    }
    /// Preparation + solve reservations only. This legacy accessor excludes
    /// all later predicates, candidates and projections; never a total-work claim.
    pub fn operations(&self) -> usize {
        self.operations
    }
    pub fn displacement(&self, dof: usize) -> Option<&Ratio> {
        self.displacement.get(dof)
    }
    pub fn reaction(&self, dof: usize) -> Option<&Ratio> {
        self.reactions.get(dof)
    }
    pub fn project_displacement(
        &self,
        dof: usize,
        relative_limit: f64,
        work_limit: usize,
    ) -> Attempt<QualifiedProjection<'_, 'a>> {
        self.project_quantity(Quantity::Displacement, dof, relative_limit, work_limit)
    }
    pub fn project_reaction(
        &self,
        dof: usize,
        relative_limit: f64,
        work_limit: usize,
    ) -> Attempt<QualifiedProjection<'_, 'a>> {
        self.project_quantity(Quantity::Reaction, dof, relative_limit, work_limit)
    }
    fn project_quantity(
        &self,
        quantity: Quantity,
        dof: usize,
        relative_limit: f64,
        work_limit: usize,
    ) -> Attempt<QualifiedProjection<'_, 'a>> {
        metered(self.context.limits, work_limit, |w| {
            w.charge(4)?;
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
            })
        })
    }
    /// Exact sign(s*u-g), with per-call reservations capped by Context's limit.
    pub fn gap_sign_with_work(
        &self,
        dof: usize,
        sense: i8,
        gap: f64,
        work_limit: usize,
    ) -> Attempt<Sign> {
        metered(self.context.limits, work_limit, |w| {
            w.charge(12)?;
            if !matches!(sense, -1 | 1) || !gap.is_finite() {
                return Err(Error::Invalid("gap predicate"));
            }
            let r = self
                .displacement
                .get(dof)
                .ok_or(Error::Invalid("gap dof"))?;
            let g = w.scalar(gap)?;
            let gd = w.mul(&g, &r.denominator)?;
            let sn = w.sum(&Expansion::default(), &r.numerator, sense as f64)?;
            Ok(sign(&w.sum(&sn, &gd, -1.0)?))
        })
    }
    /// Compatibility wrapper; callers requiring total work must retain the
    /// corresponding `*_with_work` attempt, including failed calls.
    pub fn gap_sign(&self, dof: usize, sense: i8, gap: f64) -> Result<Sign, Error> {
        self.gap_sign_with_work(dof, sense, gap, self.context.limits.operations)
            .result
    }
    pub fn signed_reaction_with_work(
        &self,
        dof: usize,
        sense: i8,
        work_limit: usize,
    ) -> Attempt<Sign> {
        metered(self.context.limits, work_limit, |w| {
            w.charge(12)?;
            if !matches!(sense, -1 | 1) {
                return Err(Error::Invalid("reaction sense"));
            }
            let s = self
                .reactions
                .get(dof)
                .ok_or(Error::Invalid("reaction dof"))?
                .sign();
            Ok(if sense == 1 {
                s
            } else {
                match s {
                    Sign::Negative => Sign::Positive,
                    Sign::Zero => Sign::Zero,
                    Sign::Positive => Sign::Negative,
                }
            })
        })
    }
    pub fn signed_reaction(&self, dof: usize, sense: i8) -> Result<Sign, Error> {
        self.signed_reaction_with_work(dof, sense, self.context.limits.operations)
            .result
    }
    fn verify_candidate_charged(&self, candidate: &[f64], w: &mut Work) -> Result<bool, Error> {
        w.charge(8)?;
        if candidate.len() != self.displacement.len() {
            return Err(Error::Invalid("candidate dimensions/range"));
        }
        w.charge(candidate.len().saturating_mul(4))?;
        if candidate.iter().any(|x| !x.is_finite()) {
            return Err(Error::Invalid("candidate dimensions/range"));
        }
        w.charge(self.context.source.prescribed.len().saturating_mul(4))?;
        if self
            .context
            .source
            .prescribed
            .iter()
            .any(|&(i, v)| candidate[i] != v)
        {
            return Ok(false);
        }
        for &i in &self.context.source.free {
            let mut residual = w.sum(&Expansion::default(), &self.context.f[i], -1.0)?;
            for (j, &v) in candidate.iter().enumerate() {
                let ve = w.scalar(v)?;
                let p = w.mul(&self.context.k[i][j], &ve)?;
                residual = w.sum(&residual, &p, 1.0)?;
            }
            if !residual.is_zero() {
                return Ok(false);
            }
        }
        Ok(true)
    }
    /// Checks EVERY original free equation and actual prescribed value. All-fixed
    /// candidates remain legitimate; active reactions still require classification.
    pub fn verify_candidate_with_work(
        &self,
        candidate: &[f64],
        work_limit: usize,
    ) -> Attempt<bool> {
        metered(self.context.limits, work_limit, |w| {
            self.verify_candidate_charged(candidate, w)
        })
    }
    pub fn verify_candidate(&self, candidate: &[f64]) -> Result<bool, Error> {
        self.verify_candidate_with_work(candidate, self.context.limits.operations)
            .result
    }
    pub fn exact_representable_candidate_with_work(
        &self,
        work_limit: usize,
    ) -> Attempt<Option<Vec<f64>>> {
        metered(self.context.limits, work_limit, |w| {
            w.charge(4)?;
            let mut v = Vec::with_capacity(self.displacement.len());
            for r in &self.displacement {
                w.charge(
                    r.numerator
                        .terms
                        .len()
                        .saturating_add(r.denominator.terms.len())
                        .saturating_add(16),
                )?;
                v.push(r.approximate_projection()?);
            }
            w.charge(self.context.source.prescribed.len().saturating_mul(4))?;
            for &(i, p) in &self.context.source.prescribed {
                v[i] = p;
            }
            Ok(if self.verify_candidate_charged(&v, w)? {
                Some(v)
            } else {
                None
            })
        })
    }
    pub fn exact_representable_candidate(&self) -> Result<Option<Vec<f64>>, Error> {
        self.exact_representable_candidate_with_work(self.context.limits.operations)
            .result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn scalar_projection(n: f64, d: f64, criterion: f64, budget: usize) -> Attempt<ProjectionData> {
        let mut numerator = Expansion::default();
        numerator.add(n).unwrap();
        let mut denominator = Expansion::default();
        denominator.add(d).unwrap();
        let r = Ratio {
            numerator,
            denominator,
        };
        metered(Limits::default(), budget, |w| project(w, &r, criterion))
    }
    #[test]
    fn projection_exact_zero_and_range_edge_shortcuts() {
        for sense in [-1.0, 1.0] {
            for value in [
                f64::from_bits(1),
                f64::MIN_POSITIVE,
                f64::MAX,
                2.0_f64.powi(-60),
            ] {
                let a = scalar_projection(sense * value, 1.0, 0.0, 2_000_000);
                assert!(a.work.charged > 0);
                let p = a.result.unwrap();
                assert_eq!(p.value, sense * value);
                assert_eq!(p.interval, [sense * value; 2]);
                assert_eq!(p.absolute, 0.0);
                assert_eq!(p.relative, 0.0);
                assert_eq!(p.basis, ProjectionBasis::ExactIdentity);
            }
            let z = scalar_projection(sense * 0.0, 1.0, 0.0, 2_000_000)
                .result
                .unwrap();
            assert_eq!(z.value.to_bits(), 0.0_f64.to_bits());
            assert_eq!(z.basis, ProjectionBasis::ExactZero);
            for (n, d) in [
                (f64::from_bits(1), 2.0),
                (f64::from_bits(3), 2.0),
                (f64::MAX, 0.5),
            ] {
                let a = scalar_projection(sense * n, d, 1e-9, 2_000_000);
                assert!(a.result.is_err());
                assert!(a.work.charged > 0);
            }
        }
    }
    #[test]
    fn outward_projection_all_signed_endpoints_and_gradual_underflow() {
        for sense in [-1.0, 1.0] {
            let p = scalar_projection(sense, 3.0, 1e-9, 2_000_000)
                .result
                .unwrap();
            assert_eq!(p.basis, ProjectionBasis::OutwardInterval);
            assert_eq!(p.value, sense / 3.0);
            // Independent proposal: 1/3 certified <= 6.67e-16; symmetric negative
            // case must include reversed denominator endpoint ordering.
            assert!(p.relative > 0.0 && p.relative < 7e-16);
            assert!(p.absolute > 0.0);
            assert!(p.interval[0] < p.value && p.value < p.interval[1]);
            let opposite = scalar_projection(-sense, 3.0, 1e-9, 2_000_000)
                .result
                .unwrap();
            assert_eq!(p.interval, [-opposite.interval[1], -opposite.interval[0]]);
            let tiny = scalar_projection(sense * 1e-310, 1.1, 1e-9, 2_000_000)
                .result
                .unwrap();
            assert_eq!(tiny.basis, ProjectionBasis::OutwardInterval);
            assert_eq!(tiny.value, sense * (1e-310 / 1.1));
            assert!(tiny.relative < 2e-13);
        }
        for limit in [f64::NAN, f64::INFINITY, -1.0] {
            assert!(matches!(
                scalar_projection(0.0, 1.0, limit, 2_000_000).result,
                Err(Error::Invalid(_))
            ));
        }
        assert!(matches!(
            scalar_projection(1.0, 3.0, 0.0, 2_000_000).result,
            Err(Error::ProjectionUnresolved(_))
        ));
    }
    #[test]
    fn projections_bind_the_same_response_quantity_and_source() {
        let c = make(&[vec![3.0]], &[1.0], &[0], &[]).unwrap();
        let r = c.solve().unwrap();
        let other = c.solve().unwrap();
        let p = r.project_displacement(0, 1e-9, usize::MAX).result.unwrap();
        assert!(p.is_for(&r, Quantity::Displacement, 0));
        assert!(!p.is_for(&r, Quantity::Reaction, 0));
        assert!(!p.is_for(&r, Quantity::Displacement, 1));
        assert!(!p.is_for(&other, Quantity::Displacement, 0));
        let changed = make(&[vec![3.0]], &[2.0], &[0], &[]).unwrap();
        let changed = changed.solve().unwrap();
        assert!(!p.is_for(&changed, Quantity::Displacement, 0));
        assert_eq!(p.exact_sign(), Sign::Positive);
        assert_eq!(p.relative_limit(), 1e-9);
        assert!(p.relative_error_bound() <= p.relative_limit());
        assert!(p.absolute_error_bound() > 0.0);
        assert_eq!(p.value(), 1.0 / 3.0);
        assert!(r
            .project_reaction(0, 0.0, usize::MAX)
            .result
            .unwrap()
            .is_for(&r, Quantity::Reaction, 0));
    }
    #[test]
    fn per_call_work_keeps_success_failure_and_rejected_reservations() {
        let c = chain(12.5, [0.125, 0.25], [false, false], 1.0, 0.0);
        let r = c.solve().unwrap();
        let base = r.operations();
        let a = r.gap_sign_with_work(1, 1, 0.125, usize::MAX);
        assert_eq!(a.result.unwrap(), Sign::Zero);
        assert!(a.work.charged > 0);
        assert_eq!(a.work.limit, c.limits.operations);
        let b = r.gap_sign_with_work(1, 1, 0.125, a.work.charged - 1);
        assert_eq!(b.result, Err(Error::Budget));
        assert!(b.work.rejected > 0);
        let invalid = r.gap_sign_with_work(999, 1, 0.125, usize::MAX);
        assert!(invalid.result.is_err());
        assert!(invalid.work.charged > 0);
        let invalid = r.verify_candidate_with_work(&[], usize::MAX);
        assert!(invalid.result.is_err());
        assert!(invalid.work.charged > 0);
        let rejected = r.verify_candidate_with_work(&[0.0, 0.0, 0.0], usize::MAX);
        assert_eq!(rejected.result, Ok(false));
        assert!(rejected.work.charged > 0);
        let exact = r.exact_representable_candidate_with_work(usize::MAX);
        assert!(exact.result.unwrap().is_some());
        assert!(exact.work.charged > a.work.charged);
        let zero_budget = r.project_displacement(1, 1e-9, 0);
        assert!(matches!(zero_budget.result, Err(Error::Budget)));
        assert_eq!(zero_budget.work.charged, 0);
        assert!(zero_budget.work.rejected > 0);
        let small_budget = r.project_displacement(1, 1e-9, 40);
        assert!(matches!(small_budget.result, Err(Error::Budget)));
        assert!(small_budget.work.charged > 0);
        assert!(small_budget.work.rejected > 0);
        assert_eq!(r.operations(), base); // explicitly not a lifetime-total accessor
    }
    #[test]
    fn projected_adjacent_reactions_preserve_exact_tiny_values() {
        for sense in [-1.0, 1.0] {
            let c = chain(
                f64::from_bits(12.5_f64.to_bits() + 1),
                [0.125, 0.25],
                [true, true],
                sense,
                0.0,
            );
            let r = c.solve().unwrap();
            let p = r.project_reaction(2, 1e-9, usize::MAX).result.unwrap();
            assert_eq!(p.value(), -sense * 2.0_f64.powi(-49));
            assert_eq!(p.absolute_error_bound(), 0.0);
            // One active gap, one free coordinate: reaction = -25/2^54, even
            // when projecting free u first and evaluating K*u would alter it.
            let c = chain(
                12.5,
                [f64::from_bits(0.125_f64.to_bits() - 1), 0.25],
                [true, false],
                sense,
                0.0,
            );
            let r = c.solve().unwrap();
            let p = r.project_reaction(1, 1e-9, usize::MAX).result.unwrap();
            assert_eq!(p.value(), -sense * 25.0 * 2.0_f64.powi(-54));
            assert_eq!(p.absolute_error_bound(), 0.0);
            assert!(p.is_for(&r, Quantity::Reaction, 1));
        }
    }
    #[test]
    fn manager_small_strain_companion_exact_projection() {
        // Additional manager-authored reference, independent review still pending:
        // E=200e9 Pa, A=1/256 m^2, L=1 m; k=781250000 N/m.
        // Original numerical-boundary oracles above remain unchanged.
        let k = 781250000.0;
        let g1 = 1.0 / 16384.0;
        let g2 = 1.0 / 8192.0;
        let force = 48828125.0 / 1024.0;
        for sense in [-1.0, 1.0] {
            let c = make(
                &[vec![2.0 * k, -k], vec![-k, k]],
                &[0.0, sense * force],
                &[0, 1],
                &[],
            )
            .unwrap();
            let r = c.solve().unwrap();
            for (dof, gap) in [(0, g1), (1, g2)] {
                let p = r
                    .project_displacement(dof, 1e-9, usize::MAX)
                    .result
                    .unwrap();
                assert_eq!(p.value(), sense * gap);
                assert_eq!(p.absolute_error_bound(), 0.0);
                assert_eq!(
                    r.gap_sign_with_work(dof, sense as i8, gap, usize::MAX)
                        .result
                        .unwrap(),
                    Sign::Zero
                );
            }
        }
    }
    fn contributions(k: &[Vec<f64>]) -> Vec<StiffnessContribution> {
        k.iter()
            .enumerate()
            .flat_map(|(row, r)| {
                r.iter()
                    .enumerate()
                    .map(move |(col, &value)| StiffnessContribution { row, col, value })
            })
            .collect()
    }
    fn make(
        k: &[Vec<f64>],
        f: &[f64],
        free: &[usize],
        fixed: &[(usize, f64)],
    ) -> Result<Context, Error> {
        let c = contributions(k);
        Context::new(
            &StructuralSystem {
                stiffness: k,
                force: f,
                free_dofs: free,
                prescribed: fixed,
                contributions: Some(&c),
                symmetry: None,
            },
            "independent-rational-control",
            ForceBasis::DeclaredVector,
            Limits::default(),
        )
    }
    fn chain(load: f64, gaps: [f64; 2], state: [bool; 2], sense: f64, root: f64) -> Context {
        let k = vec![
            vec![100.0, -100.0, 0.0],
            vec![-100.0, 200.0, -100.0],
            vec![0.0, -100.0, 100.0],
        ];
        let f = [0.0, 0.0, sense * load];
        let mut free = vec![];
        let mut fixed = vec![(0, root)];
        for j in 0..2 {
            if state[j] {
                fixed.push((j + 1, sense * gaps[j]));
            } else {
                free.push(j + 1);
            }
        }
        make(&k, &f, &free, &fixed).unwrap()
    }
    #[test]
    fn coupled_tie_candidate_checks_original_rows_and_nonzero_prescribed() {
        for sense in [-1.0, 1.0] {
            let c = chain(12.5, [0.125, 0.25], [false, false], sense, 0.0);
            let r = c.solve().unwrap();
            let exact = [0.0, sense * 0.125, sense * 0.25];
            assert!(r.verify_candidate(&exact).unwrap());
            assert_eq!(
                r.exact_representable_candidate().unwrap(),
                Some(exact.to_vec())
            );
            for j in 1..3 {
                assert_eq!(
                    r.gap_sign(j, sense as i8, [0.125, 0.25][j - 1]).unwrap(),
                    Sign::Zero
                );
                assert_eq!(r.reaction(j).unwrap().sign(), Sign::Zero);
            }
            let near = [
                0.0,
                sense * 0.12499999999999997,
                sense * 0.24999999999999997,
            ];
            assert!(!r.verify_candidate(&[near[0], exact[1], near[2]]).unwrap());
            assert!(!r.verify_candidate(&[near[0], near[1], exact[2]]).unwrap());
        }
        let c = chain(12.5, [0.1875, 0.3125], [false, false], 1.0, 0.0625);
        let r = c.solve().unwrap();
        assert!(r.verify_candidate(&[0.0625, 0.1875, 0.3125]).unwrap());
        assert!(!r.verify_candidate(&[0.0625, 0.125, 0.25]).unwrap());
        assert!(!r.verify_candidate(&[0.0, 0.1875, 0.3125]).unwrap());
        assert!(r.verify_candidate(&[]).is_err());
    }
    #[test]
    fn force_tail_is_not_a_declared_vector_claim() {
        let k = vec![vec![200.0, -100.0], vec![-100.0, 100.0]];
        let f = [0.0, 12.5];
        let c = contributions(&k);
        let fixed = [(0, 0.125), (1, 0.25)];
        let system = StructuralSystem {
            stiffness: &k,
            force: &f,
            free_dofs: &[],
            prescribed: &fixed,
            contributions: Some(&c),
            symmetry: None,
        };
        let tail = 2.0_f64.powi(-60);
        let terms = [
            ForceContribution {
                source: "direct-load".into(),
                dof: 1,
                value: 12.5,
            },
            ForceContribution {
                source: "tail-load".into(),
                dof: 1,
                value: tail,
            },
        ];
        let source = Context::new(
            &system,
            "tail",
            ForceBasis::IdentifiedContributions(&terms),
            Limits::default(),
        )
        .unwrap();
        assert_eq!(
            source.force_level(),
            ForceLevel::IdentifiedRepresentedContributions
        );
        let r = source.solve().unwrap();
        assert_eq!(r.signed_reaction(1, 1).unwrap(), Sign::Negative);
        assert_eq!(
            r.reaction(1).unwrap().approximate_projection().unwrap(),
            -tail
        );
        assert!(r.verify_candidate(&[0.125, 0.25]).unwrap()); // all fixed is legitimate, reaction still nonzero
        let declared = Context::new(
            &system,
            "tail",
            ForceBasis::DeclaredVector,
            Limits::default(),
        )
        .unwrap();
        assert_eq!(declared.force_level(), ForceLevel::DeclaredVector);
        assert_eq!(
            declared.solve().unwrap().reaction(1).unwrap().sign(),
            Sign::Zero
        );
        assert!(!source.matches(&system, "tail", ForceBasis::DeclaredVector));
        let mut changed = terms.clone();
        changed[1].value = 0.0;
        assert!(!source.matches(
            &system,
            "tail",
            ForceBasis::IdentifiedContributions(&changed)
        ));
    }
    #[test]
    fn all_fixed_pulling_bearing_and_zero_reactions_are_distinct() {
        for (load, expected) in [
            (f64::from_bits(12.5_f64.to_bits() - 1), Sign::Positive),
            (12.5, Sign::Zero),
            (f64::from_bits(12.5_f64.to_bits() + 1), Sign::Negative),
        ] {
            for sense in [-1.0, 1.0] {
                let c = chain(load, [0.125, 0.25], [true, true], sense, 0.0);
                let r = c.solve().unwrap();
                assert_eq!(r.signed_reaction(2, sense as i8).unwrap(), expected);
            }
        }
    }
    #[test]
    fn incomplete_duplicate_changed_maps_and_sources_fail() {
        let k = vec![vec![2.0, 0.0], vec![0.0, 3.0]];
        let f = [1.0, 1.0];
        let c = contributions(&k);
        assert!(make(&k, &f, &[0], &[]).is_err());
        assert!(make(&k, &f, &[0, 0], &[]).is_err());
        assert!(make(&k, &f, &[0], &[(0, 0.0), (1, 0.0)]).is_err());
        let system = StructuralSystem {
            stiffness: &k,
            force: &f,
            free_dofs: &[0, 1],
            prescribed: &[],
            contributions: Some(&c),
            symmetry: None,
        };
        let context = Context::new(
            &system,
            "source",
            ForceBasis::DeclaredVector,
            Limits::default(),
        )
        .unwrap();
        assert!(context.matches(&system, "source", ForceBasis::DeclaredVector));
        assert!(!context.matches(&system, "changed", ForceBasis::DeclaredVector));
        let changed_force = [1.0, 2.0];
        let changed = StructuralSystem {
            force: &changed_force,
            ..system
        };
        assert!(!context.matches(&changed, "source", ForceBasis::DeclaredVector));
        let changed = StructuralSystem {
            force: &f,
            free_dofs: &[1, 0],
            ..changed
        };
        assert!(!context.matches(&changed, "source", ForceBasis::DeclaredVector));
        let mut spring = c.clone();
        spring.push(StiffnessContribution {
            row: 0,
            col: 0,
            value: 1.0,
        });
        let changed = StructuralSystem {
            free_dofs: &[0, 1],
            contributions: Some(&spring),
            ..changed
        };
        assert!(!context.matches(&changed, "source", ForceBasis::DeclaredVector));
        assert!(Context::new(
            &changed,
            "source",
            ForceBasis::DeclaredVector,
            Limits::default()
        )
        .is_err());
        let missing = StructuralSystem {
            contributions: None,
            ..changed
        };
        assert!(Context::new(
            &missing,
            "source",
            ForceBasis::DeclaredVector,
            Limits::default()
        )
        .is_err());
    }
    #[test]
    fn unsupported_singular_negative_asymmetric_range_and_budget_are_explicit() {
        for k in [
            vec![vec![0.0]],
            vec![vec![-1.0]],
            vec![vec![1.0, 1.0], vec![1.0, 1.0]],
        ] {
            let n = k.len();
            assert!(matches!(
                make(&k, &vec![0.0; n], &(0..n).collect::<Vec<_>>(), &[]),
                Err(Error::NotPositiveDefinite)
            ));
        }
        let k = vec![
            vec![2.0, -1.0, 0.0],
            vec![-1.0, 2.0, -1.0],
            vec![0.0, -1.0, 2.0],
        ];
        assert!(matches!(
            make(&k, &[0.0; 3], &[0, 1, 2], &[]),
            Err(Error::UnsupportedBlock { order: 3 })
        ));
        assert!(make(&[vec![2.0, 1.0], vec![0.0, 2.0]], &[0.0; 2], &[0, 1], &[]).is_err());
        assert!(make(&[vec![f64::NAN]], &[0.0], &[0], &[]).is_err());
        let huge = vec![vec![1e300, 1.0], vec![1.0, 1e300]];
        assert!(matches!(
            make(&huge, &[0.0; 2], &[0, 1], &[]),
            Err(Error::Arithmetic(_))
        ));
        let tiny = vec![
            vec![f64::from_bits(1), f64::from_bits(1)],
            vec![f64::from_bits(1), f64::from_bits(2)],
        ];
        assert!(matches!(
            make(&tiny, &[0.0; 2], &[0, 1], &[]),
            Err(Error::Arithmetic(_))
        ));
        let k = vec![vec![1.0]];
        let c = contributions(&k);
        let system = StructuralSystem {
            stiffness: &k,
            force: &[1.0],
            free_dofs: &[0],
            prescribed: &[],
            contributions: Some(&c),
            symmetry: None,
        };
        assert!(matches!(
            Context::new(
                &system,
                "small-budget",
                ForceBasis::DeclaredVector,
                Limits {
                    operations: 1,
                    ..Limits::default()
                }
            ),
            Err(Error::Budget)
        ));
    }
    #[test]
    fn original_exact_graph_does_not_use_absorbed_stored_coupling() {
        let tiny = 2.0_f64.powi(-60);
        let k = vec![
            vec![4.0, 0.0, 0.0],
            vec![0.0, 4.0, 0.0],
            vec![0.0, 0.0, 4.0],
        ];
        let mut cs = contributions(&k);
        for (row, col) in [(0, 1), (1, 0), (1, 2), (2, 1)] {
            for value in [1.0, tiny, -1.0] {
                cs.push(StiffnessContribution { row, col, value });
            }
        }
        let system = StructuralSystem {
            stiffness: &k,
            force: &[0.0; 3],
            free_dofs: &[0, 1, 2],
            prescribed: &[],
            contributions: Some(&cs),
            symmetry: None,
        };
        assert!(matches!(
            Context::new(
                &system,
                "absorbed coupling",
                ForceBasis::DeclaredVector,
                Limits::default()
            ),
            Err(Error::UnsupportedBlock { order: 3 })
        ));
    }
    #[test]
    fn reactions_combine_independent_blocks_and_nonrepresentable_response() {
        // u0=1/3, u1=1/5; prescribed row action = 1/3-2/5 = -1/15.
        let k = vec![
            vec![3.0, 0.0, 1.0],
            vec![0.0, 5.0, -2.0],
            vec![1.0, -2.0, 8.0],
        ];
        let c = make(&k, &[1.0, 1.0, 0.0], &[0, 1], &[(2, 0.0)]).unwrap();
        let r = c.solve().unwrap();
        assert_eq!(r.signed_reaction(2, 1).unwrap(), Sign::Negative);
        assert_eq!(r.exact_representable_candidate().unwrap(), None);
        let rr = r.reaction(2).unwrap();
        assert_eq!(rr.numerator_terms(), &[-1.0]);
        assert_eq!(rr.denominator_terms(), &[15.0]);
    }
    #[test]
    fn source_permutation_preserves_proof_but_invalidates_old_binding() {
        let k = vec![vec![200.0, -100.0], vec![-100.0, 100.0]];
        let f = [0.0, 12.5];
        let cs = contributions(&k);
        let system = StructuralSystem {
            stiffness: &k,
            force: &f,
            free_dofs: &[0, 1],
            prescribed: &[],
            contributions: Some(&cs),
            symmetry: None,
        };
        let a = Context::new(
            &system,
            "permutation",
            ForceBasis::DeclaredVector,
            Limits::default(),
        )
        .unwrap();
        let reversed: Vec<_> = cs.iter().rev().copied().collect();
        let other = StructuralSystem {
            contributions: Some(&reversed),
            ..system
        };
        assert!(!a.matches(&other, "permutation", ForceBasis::DeclaredVector));
        let b = Context::new(
            &other,
            "permutation",
            ForceBasis::DeclaredVector,
            Limits::default(),
        )
        .unwrap();
        assert_eq!(
            a.solve().unwrap().gap_sign(0, 1, 0.125).unwrap(),
            b.solve().unwrap().gap_sign(0, 1, 0.125).unwrap()
        );
    }
    #[test]
    fn exact_zero_source_coupling_can_split_a_stored_connected_graph() {
        let tiny = 2.0_f64.powi(-60);
        let k = vec![
            vec![4.0, -tiny, 0.0],
            vec![-tiny, 4.0, -tiny],
            vec![0.0, -tiny, 4.0],
        ];
        let mut cs = vec![];
        for row in 0..3 {
            cs.push(StiffnessContribution {
                row,
                col: row,
                value: 4.0,
            });
        }
        for (row, col) in [(0, 1), (1, 0), (1, 2), (2, 1)] {
            for value in [1.0, tiny, -1.0, -tiny] {
                cs.push(StiffnessContribution { row, col, value });
            }
        }
        let system = StructuralSystem {
            stiffness: &k,
            force: &[1.0; 3],
            free_dofs: &[0, 1, 2],
            prescribed: &[],
            contributions: Some(&cs),
            symmetry: None,
        };
        let c = Context::new(
            &system,
            "exact disconnected source",
            ForceBasis::DeclaredVector,
            Limits::default(),
        )
        .unwrap();
        assert_eq!(c.blocks.len(), 3);
        assert!(c.solve().unwrap().verify_candidate(&[0.25; 3]).unwrap());
    }
    #[test]
    fn node_permutation_and_exact_radix_rescaling_preserve_gap_proof() {
        for scale in [0.125, 1.0, 8.0] {
            let k = vec![
                vec![100.0 * scale, -100.0 * scale],
                vec![-100.0 * scale, 200.0 * scale],
            ];
            let c = make(&k, &[12.5 * scale, 0.0], &[1, 0], &[]).unwrap();
            let r = c.solve().unwrap();
            assert_eq!(r.gap_sign(1, 1, 0.125).unwrap(), Sign::Zero);
            assert_eq!(r.gap_sign(0, 1, 0.25).unwrap(), Sign::Zero);
        }
    }
    // Frozen independent Python Fraction oracle, STRICT_GAP_DESIGN/REFERENCE_PROOF.json.
    #[test]
    fn independent_27_neighbor_controls_both_senses_and_all_seeds() {
        let cases: [(u64, u64, u64, [bool; 2]); 27] = [
            (
                4623226492472524799,
                4593671619917905919,
                4598175219545276415,
                [false, false],
            ),
            (
                4623226492472524799,
                4593671619917905919,
                4598175219545276416,
                [false, false],
            ),
            (
                4623226492472524799,
                4593671619917905919,
                4598175219545276417,
                [false, false],
            ),
            (
                4623226492472524799,
                4593671619917905920,
                4598175219545276415,
                [false, false],
            ),
            (
                4623226492472524799,
                4593671619917905920,
                4598175219545276416,
                [false, false],
            ),
            (
                4623226492472524799,
                4593671619917905920,
                4598175219545276417,
                [false, false],
            ),
            (
                4623226492472524799,
                4593671619917905921,
                4598175219545276415,
                [false, false],
            ),
            (
                4623226492472524799,
                4593671619917905921,
                4598175219545276416,
                [false, false],
            ),
            (
                4623226492472524799,
                4593671619917905921,
                4598175219545276417,
                [false, false],
            ),
            (
                4623226492472524800,
                4593671619917905919,
                4598175219545276415,
                [true, true],
            ),
            (
                4623226492472524800,
                4593671619917905919,
                4598175219545276416,
                [true, false],
            ),
            (
                4623226492472524800,
                4593671619917905919,
                4598175219545276417,
                [true, false],
            ),
            (
                4623226492472524800,
                4593671619917905920,
                4598175219545276415,
                [false, true],
            ),
            (
                4623226492472524800,
                4593671619917905920,
                4598175219545276416,
                [true, true],
            ),
            (
                4623226492472524800,
                4593671619917905920,
                4598175219545276417,
                [true, false],
            ),
            (
                4623226492472524800,
                4593671619917905921,
                4598175219545276415,
                [false, true],
            ),
            (
                4623226492472524800,
                4593671619917905921,
                4598175219545276416,
                [false, true],
            ),
            (
                4623226492472524800,
                4593671619917905921,
                4598175219545276417,
                [false, false],
            ),
            (
                4623226492472524801,
                4593671619917905919,
                4598175219545276415,
                [true, true],
            ),
            (
                4623226492472524801,
                4593671619917905919,
                4598175219545276416,
                [true, true],
            ),
            (
                4623226492472524801,
                4593671619917905919,
                4598175219545276417,
                [true, false],
            ),
            (
                4623226492472524801,
                4593671619917905920,
                4598175219545276415,
                [false, true],
            ),
            (
                4623226492472524801,
                4593671619917905920,
                4598175219545276416,
                [true, true],
            ),
            (
                4623226492472524801,
                4593671619917905920,
                4598175219545276417,
                [true, false],
            ),
            (
                4623226492472524801,
                4593671619917905921,
                4598175219545276415,
                [false, true],
            ),
            (
                4623226492472524801,
                4593671619917905921,
                4598175219545276416,
                [false, true],
            ),
            (
                4623226492472524801,
                4593671619917905921,
                4598175219545276417,
                [false, false],
            ),
        ];
        let mut projection_count = 0;
        for (fb, g1b, g2b, expected) in cases {
            let gaps = [f64::from_bits(g1b), f64::from_bits(g2b)];
            for sense in [-1.0, 1.0] {
                for seed in [[false, false], [false, true], [true, false], [true, true]] {
                    let mut state = seed;
                    let mut converged = false;
                    for _ in 0..3 {
                        let c = chain(f64::from_bits(fb), gaps, state, sense, 0.0);
                        let r = c.solve().unwrap();
                        for dof in 0..3 {
                            for quantity in [Quantity::Displacement, Quantity::Reaction] {
                                let attempt = r.project_quantity(quantity, dof, 1e-9, usize::MAX);
                                assert!(attempt.work.charged > 0);
                                let p = attempt.result.unwrap();
                                assert!(p.is_for(&r, quantity, dof));
                                // Independent reviewed proposal's maximum over
                                // these exact references is 6.67e-16.
                                assert!(p.relative_error_bound() < 7e-16);
                                projection_count += 1;
                            }
                        }
                        let mut next = state;
                        for j in 0..2 {
                            next[j] = if state[j] {
                                r.signed_reaction(j + 1, sense as i8).unwrap() != Sign::Positive
                            } else {
                                r.gap_sign(j + 1, sense as i8, gaps[j]).unwrap() != Sign::Negative
                            };
                        }
                        if next == state {
                            converged = true;
                            break;
                        }
                        state = next;
                    }
                    assert!(
                        converged,
                        "reference cap: {fb} {g1b} {g2b} {sense} {seed:?}"
                    );
                    assert_eq!(state, expected, "{fb} {g1b} {g2b} {sense} {seed:?}");
                }
            }
        }
        assert_eq!(projection_count, 2604);
    }
}
