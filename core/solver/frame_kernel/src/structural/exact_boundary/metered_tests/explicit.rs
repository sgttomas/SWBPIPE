//! Expected explicit charges for the fixtures exercised by the arithmetic oracle.
//!
//! These sequences are derived from input sizes and expansion/retained scalar
//! lengths, never from a trace's charge amounts, WorkReport or operation counters.
//! They state the unchanged non-multiplication rules: validation headers, source
//! and descriptor inspection, scalar copies, and projection enclosure work. The
//! functional evaluator below uses the independent binary64 oracle to derive
//! intermediate copy lengths, including the zero and ±1 shortcuts. No producer
//! charging, size-counting or arithmetic helper supplies an expected charge.
//!
//! A complete entry must match its whole sequence; a budget refusal must match
//! a prefix, including any rejected explicit charge. Thus an extra legacy
//! precharge outside Work::mul cannot be absorbed into the ledger comparison.
use super::super::functionals::{MemberEnd, RetainedFunctionalSet};
use super::*;

pub(super) type Schedule = (&'static str, Vec<usize>);

pub(super) fn verify(events: &[Event], expected: &[usize], entry: &str) {
    let actual: Vec<_> = events
        .iter()
        .filter_map(|e| match e {
            Event::Charge { amount, .. } => Some(*amount),
            _ => None,
        })
        .collect();
    let refused = events.last().is_some_and(|e| {
        matches!(
            e,
            Event::Charge {
                accepted: false,
                ..
            } | Event::Add {
                outcome: Outcome::Budget,
                ..
            } | Event::Mul {
                outcome: Outcome::Budget,
                ..
            }
        )
    });
    assert!(
        expected.starts_with(&actual) && (refused || actual.len() == expected.len()),
        "{entry}: explicit-charge sequence: expected {expected:?}, got {actual:?} (refused={refused})"
    );
}

fn cat(parts: &[&[usize]]) -> Vec<usize> {
    parts.concat()
}
fn ratio(r: &Ratio) -> usize {
    r.numerator.terms.len() + r.denominator.terms.len()
}
fn witness(b: &BlockWitness) -> usize {
    b.dofs.len()
        + b.a.len()
        + b.b.as_ref().map_or(0, Vec::len)
        + b.c.as_ref().map_or(0, Vec::len)
        + b.determinant.as_ref().map_or(0, Vec::len)
        + b.upper.iter().flatten().map(Vec::len).sum::<usize>()
        + b.minors.iter().flatten().map(Vec::len).sum::<usize>()
}
fn source_preflight(s: &Source) -> Vec<usize> {
    let n = s.dofs();
    // 8 per matrix cell, 24 per DOF, 8 per stiffness/force contribution,
    // 8 per congruence scalar (288 operands + 12 map entries + 2 bounds), ID.
    let mut out = vec![
        16,
        8 * n * n
            + 24 * n
            + 8 * s.contributions.len()
            + 8 * s.terms.len()
            + 8 * 302 * s.congruence.len()
            + ID.len(),
    ];
    out.extend(s.terms.iter().map(|f| f.source.len()));
    out
}
fn preparation(s: &Source, c: &Context) -> Vec<usize> {
    let mut out = vec![8];
    let m = s.congruence.len();
    if m != 0 {
        out.push(2 * 302 * m + m * m);
    }
    out.push(s.dofs() * s.dofs());
    out.extend(std::iter::repeat_n(2 * 144, m)); // two congruence scratch matrices
    for b in &c.witnesses {
        out.push(b.dofs.len() + b.a.len());
        out.push(if b.dofs.len() == 2 {
            b.b.as_ref().unwrap().len()
                + b.c.as_ref().unwrap().len()
                + b.determinant.as_ref().unwrap().len()
        } else {
            assert!(b.dofs.len() > 2); // these fixtures have no order-1 block
            b.upper.iter().flatten().map(Vec::len).sum::<usize>()
                + b.minors.iter().flatten().map(Vec::len).sum::<usize>()
        });
    }
    out
}
fn solve(c: &Context) -> Vec<usize> {
    let n = c.f.len();
    let scalars: usize =
        c.k.iter()
            .flatten()
            .chain(&c.f)
            .map(|e| e.terms.len())
            .sum();
    // Functional solve's traversal/copy reservations; legacy solve has none.
    vec![4 * n * n + 8 * n, 8 * n * n + scalars * (n + 4) + 16 * n]
}
fn descriptors(ds: &[FunctionalDescriptor]) -> Vec<usize> {
    fn products(ps: &[Vec<f64>], out: &mut Vec<usize>) {
        out.push(4 * ps.len());
        out.extend(ps.iter().map(|p| 4 * p.len()));
    }
    let mut out = vec![8, 16 * ds.len()];
    for (i, d) in ds.iter().enumerate() {
        let label = match &d.key.quantity {
            FunctionalQuantity::DeclaredAffine { label } => label.as_str(),
            FunctionalQuantity::NodeDisplacement { node, .. } => node.as_str(),
            FunctionalQuantity::MemberEnd { member, .. }
            | FunctionalQuantity::MemberSection { member, .. } => member.as_str(),
            FunctionalQuantity::SourceReaction { .. } => "",
            _ => panic!("unexercised descriptor"),
        };
        // At insertion i: floor(log2(i+1))+2 comparisons, four key copies,
        // i integer moves and an eight-unit header.
        let comparisons = (i + 1).ilog2() as usize + 2;
        out.push((d.key.case_id.len() + label.len() + 8) * (comparisons + 4) + i + 8);
        products(&d.offset, &mut out);
        out.push(8 * d.terms.len());
        for t in &d.terms {
            products(&t.products, &mut out);
        }
    }
    out
}
fn projection(r: &Ratio, basis: ProjectionBasis) -> Vec<usize> {
    let mut out = vec![16];
    if basis == ProjectionBasis::ExactZero {
        return out;
    }
    out.push(ratio(r) + 16); // quotient proposal's numerator/denominator traversal
    if basis != ProjectionBasis::ExactIdentity {
        out.extend(std::iter::repeat_n(12, ratio(r))); // outward sums, per scalar
        out.extend([48, 32]); // quotient corners and relative-error calculation
    }
    out
}
fn contact(r: &Response<'_>, c: &RetainedContact) -> Vec<usize> {
    vec![
        16,
        r.displacement[c.dof].denominator.terms.len() + r.reactions[c.dof].denominator.terms.len(),
        ratio(&c.penetration) + ratio(&c.normalized_reaction),
    ]
}
fn copies(s: &Source, c: &Context, r: &Response<'_>, retained: &RetainedResponse) -> usize {
    // Snapshot fields (these fixtures have identified forces and no symmetry
    // evidence), witness scalars, ratios, contact headers/ratios and 2*n leaves.
    ID.len()
        + s.dofs()
        + s.free.len()
        + 2 * s.fixed.len()
        + s.dofs() * s.dofs()
        + 3 * s.contributions.len()
        + 302 * s.congruence.len()
        + s.terms.iter().map(|f| f.source.len() + 2).sum::<usize>()
        + c.witnesses.iter().map(witness).sum::<usize>()
        + r.displacement
            .iter()
            .chain(&r.reactions)
            .map(ratio)
            .sum::<usize>()
        + retained
            .contacts
            .iter()
            .map(|c| 8 + ratio(&c.penetration) + ratio(&c.normalized_reaction))
            .sum::<usize>()
        + 12 * retained.projections.len()
}
fn retention(s: &Source, c: &Context, r: &Response<'_>, retained: &RetainedResponse) -> Vec<usize> {
    let mut out = vec![16];
    out.extend(std::iter::repeat_n(
        8,
        retained.projections.len() + retained.contacts.len(),
    ));
    out.push(copies(s, c, r, retained));
    out
}
fn ratio_preflight(r: &Ratio) -> [usize; 2] {
    [4, 2 * ratio(r)]
}

/// Only binary64 vectors enter this evaluator. It reconstructs the coefficient,
/// numerator and denominator lengths that determine functional copy charges.
/// Work::add/sum/scalar themselves have no explicit charge outside their trace.
#[derive(Default)]
struct Evaluation {
    charges: Vec<usize>,
}
impl Evaluation {
    fn sum(a: &[f64], b: &[f64], sense: f64) -> Vec<f64> {
        b.iter()
            .fold(a.to_vec(), |a, b| oracle::grow(&a, sense * b).unwrap())
    }
    fn add(&mut self, a: &[f64], b: &[f64]) -> Vec<f64> {
        self.charges.push(a.len());
        Self::sum(a, b, 1.0)
    }
    fn mul(&mut self, a: &[f64], b: &[f64]) -> Vec<f64> {
        self.charges.push(4);
        if a.is_empty() || b.is_empty() {
            return vec![];
        }
        for (unit, other) in [(a, b), (b, a)] {
            if unit.len() == 1 && unit[0].abs() == 1.0 {
                if unit[0] == 1.0 {
                    self.charges.push(other.len());
                    return other.to_vec();
                }
                return Self::sum(&[], other, -1.0);
            }
        }
        let p = oracle::mul(
            a,
            b,
            &Ledger {
                used: 0,
                limit: usize::MAX,
                terms: 256,
            },
        );
        assert_eq!(p.outcome, Outcome::Ok);
        p.product
    }
    fn recipe(&mut self, ps: &[Vec<f64>]) -> Vec<f64> {
        let mut sum = vec![];
        for atoms in ps {
            let mut product = oracle::grow(&[], atoms[0]).unwrap();
            for &atom in &atoms[1..] {
                product = self.mul(&product, &oracle::grow(&[], atom).unwrap());
            }
            sum = self.add(&sum, &product);
        }
        sum
    }
    fn combine(
        &mut self,
        a: (Vec<f64>, Vec<f64>),
        b: (Vec<f64>, Vec<f64>),
    ) -> (Vec<f64>, Vec<f64>) {
        self.charges
            .push(a.0.len() + a.1.len() + b.0.len() + b.1.len());
        if a.0.is_empty() {
            return b;
        }
        if b.0.is_empty() {
            return a;
        }
        if same_bits(&a.1, &b.1) {
            return (self.add(&a.0, &b.0), a.1);
        }
        let an = self.mul(&a.0, &b.1);
        let bn = self.mul(&b.0, &a.1);
        (self.add(&an, &bn), self.mul(&a.1, &b.1))
    }
}
// R-A §3.1's as-built schedule, stated on descriptors and independently
// reconstructed expansions. No shortcuts helper, row trace or producer size
// function is used to decide either a charge or a path.
fn atoms(ps: &[Vec<f64>]) -> usize {
    ps.iter().map(|p| 1 + p.len()).sum()
}
fn key_size(key: &FunctionalKey) -> usize {
    let name = match &key.quantity {
        FunctionalQuantity::MemberEnd { member, .. }
        | FunctionalQuantity::MemberSection { member, .. } => member,
        _ => panic!("B1 candidate is not member-like"),
    };
    key.case_id.len() + name.len() + 8
}
fn end(key: &FunctionalKey) -> Option<(&str, u8)> {
    match &key.quantity {
        FunctionalQuantity::MemberEnd {
            member,
            end: MemberEnd::I,
            row,
        } => Some((member, *row)),
        _ => None,
    }
}
fn row_size(d: &FunctionalDescriptor) -> usize {
    d.terms.len() + d.terms.iter().map(|t| atoms(&t.products)).sum::<usize>()
}
fn appended_negation(a: &[Vec<f64>], b: &[Vec<f64>]) -> bool {
    a.len() == b.len()
        && a.iter().zip(b).all(|(d, s)| {
            d.len() == s.len() + 1
                && same_bits(&d[..s.len()], s)
                && d[s.len()].to_bits() == (-1.0f64).to_bits()
        })
}
fn zero_extension(extra: &[AffineTerm], earlier: &FunctionalDescriptor) -> bool {
    extra.len() == earlier.terms.len()
        && extra.iter().zip(&earlier.terms).all(|(d, s)| {
            d.dof == s.dof
                && d.products.len() == s.products.len()
                && d.products.iter().zip(&s.products).all(|(a, b)| {
                    a.len() > b.len() && same_bits(&a[..b.len()], b) && {
                        let tail = &a[b.len()..];
                        tail.iter()
                            .position(|x| *x == 0.0)
                            .is_some_and(|z| tail[..z].iter().all(|x| x.abs() == 1.0))
                    }
                })
        })
}
fn b1_source(
    e: &mut Evaluation,
    index: usize,
    ds: &[FunctionalDescriptor],
    hints: &[Option<u32>],
    source_ok: &[bool],
) -> Option<usize> {
    if hints.len() != ds.len() {
        return None;
    }
    let d = &ds[index];
    let FunctionalQuantity::MemberSection {
        member, component, ..
    } = &d.key.quantity
    else {
        return None;
    };
    let j = hints[index]? as usize;
    if j >= index {
        return None;
    }
    let s = &ds[j];
    e.charges.push(key_size(&s.key));
    if s.key.case_id != d.key.case_id || end(&s.key) != Some((member, *component)) {
        return None;
    }
    e.charges
        .push(atoms(&s.offset) + atoms(&d.offset) + row_size(s) + row_size(d));
    if !appended_negation(&d.offset, &s.offset)
        || d.terms.len() < s.terms.len()
        || !d
            .terms
            .iter()
            .zip(&s.terms)
            .all(|(d, s)| d.dof == s.dof && appended_negation(&d.products, &s.products))
    {
        return None;
    }
    if s.offset
        .iter()
        .chain(s.terms.iter().flat_map(|t| &t.products))
        .any(|p| p.len() > 2 || p.iter().any(|x| x.abs() == 1.0))
    {
        return None;
    }
    let extra = &d.terms[s.terms.len()..];
    if !extra.is_empty() {
        let first = j.saturating_sub(*component as usize);
        let mut found = false;
        for earlier in &ds[first..=j] {
            e.charges.push(key_size(&earlier.key) + row_size(earlier));
            if earlier.key.case_id == d.key.case_id
                && end(&earlier.key).is_some_and(|(m, _)| m == member)
                && zero_extension(extra, earlier)
            {
                found = true;
                break;
            }
        }
        if !found {
            return None;
        }
    }
    e.charges.push(1);
    source_ok[j].then_some(j)
}
fn evaluation(
    r: &Response<'_>,
    ds: &[FunctionalDescriptor],
    hints: &[Option<u32>],
    values: &[Ratio],
) -> Vec<usize> {
    let c = r.context;
    let n = c.f.len();
    let mut e = Evaluation {
        charges: vec![ds.len() * (4 * n + 8)],
    };
    for block in &c.blocks {
        if block.len() > 2 {
            e.charges.push(
                block
                    .iter()
                    .map(|&i| r.displacement[i].denominator.terms.len())
                    .sum(),
            );
        }
    }
    e.charges.extend([
        3 * n + c.source.prescribed.len(),
        c.blocks.iter().map(Vec::len).sum(),
    ]);
    let zero = |i| c.source.prescribed.iter().any(|&(j, v)| j == i && v == 0.0);
    let b2 = !c.blocks.iter().flatten().any(|&i| zero(i));
    let mut reconstructed: Vec<(Vec<f64>, Vec<f64>)> = Vec::new();
    let mut source_ok = Vec::new();
    for (index, (d, expected)) in ds.iter().zip(values).enumerate() {
        let q = if let Some(j) = b1_source(&mut e, index, ds, hints, &source_ok) {
            let (num, den) = &reconstructed[j];
            e.charges.push(num.len() + den.len());
            source_ok.push(false);
            (num.iter().map(|x| -x).collect(), den.clone())
        } else {
            let mut skip = vec![false; n];
            if b2 {
                e.charges.push(d.terms.len());
                for t in &d.terms {
                    if zero(t.dof) {
                        e.charges.push(atoms(&t.products));
                    }
                }
                // Group by DOF, as the window bounds the whole ordered sum on
                // that DOF, including products in separate AffineTerms.
                for (i, skip) in skip.iter_mut().enumerate() {
                    let ps: Vec<_> = d
                        .terms
                        .iter()
                        .filter(|t| t.dof == i)
                        .flat_map(|t| &t.products)
                        .collect();
                    *skip = zero(i)
                        && ps.len() <= 64
                        && ps.iter().all(|p| {
                            p.len() <= 5
                                && p.iter()
                                    .all(|x| x.is_normal() && oracle::exponent(*x).abs() <= 150)
                        });
                }
            }
            let mut coefficients = vec![vec![]; n];
            for t in &d.terms {
                if skip[t.dof] {
                    continue;
                }
                let coefficient = e.recipe(&t.products);
                coefficients[t.dof] = e.add(&coefficients[t.dof], &coefficient);
            }
            let mut fixed = e.recipe(&d.offset);
            for &(i, value) in &c.source.prescribed {
                if !coefficients[i].is_empty() {
                    let p = e.mul(&coefficients[i], &oracle::grow(&[], value).unwrap());
                    fixed = e.add(&fixed, &p);
                }
            }
            if hints.len() == ds.len() && end(&d.key).is_some() {
                e.charges.push(n + 2);
            }
            let fixed_empty = fixed.is_empty();
            let mut q = (fixed, vec![1.0]);
            let mut blocks = 0;
            for block in &c.blocks {
                let mut numerator = vec![];
                for &i in block {
                    if !coefficients[i].is_empty() {
                        let p = e.mul(&coefficients[i], &r.displacement[i].numerator.terms);
                        numerator = e.add(&numerator, &p);
                    }
                }
                if !numerator.is_empty() {
                    blocks += 1;
                    let den = r.displacement[block[0]].denominator.terms.clone();
                    e.charges.push(den.len());
                    q = e.combine(q, (numerator, den));
                }
            }
            source_ok.push(
                end(&d.key).is_some()
                    && fixed_empty
                    && blocks <= 1
                    && !coefficients
                        .iter()
                        .any(|p| p.len() == 1 && p[0].abs() == 1.0),
            );
            q
        };
        assert!(
            same_bits(&q.0, &expected.numerator.terms),
            "functional {:?}: expected {:?}, got {:?}",
            d.key,
            expected.numerator.terms,
            q.0
        );
        assert!(same_bits(&q.1, &expected.denominator.terms));
        reconstructed.push(q);
    }
    e.charges
}

pub(super) fn functional(
    s: &Source,
    r: &Response<'_>,
    retained: &RetainedFunctionalSet,
    hints: &[Option<u32>],
) -> Vec<Schedule> {
    let c = r.context;
    let rr = retained.response();
    let ds = retained.descriptors();
    let values = retained.ratios();
    let preflight = source_preflight(s);
    let prepare = cat(&[&preflight, &preparation(s, c)]);
    let solve = solve(c);
    let desc = descriptors(ds);
    let hint_count = if hints.len() == ds.len() {
        hints.len()
    } else {
        0
    };
    let plan_hints = if hint_count == 0 {
        vec![]
    } else {
        vec![2 * hint_count]
    };
    let replay_hints = if hint_count == 0 {
        vec![]
    } else {
        vec![2 * hint_count]
    };
    let eval = evaluation(r, ds, hints, values);
    let contact = contact(r, &rr.contacts[0]);
    let leaves: Vec<_> = rr
        .projections
        .iter()
        .map(|p| {
            let ratio = match p.quantity {
                Quantity::Displacement => &r.displacement[p.dof],
                Quantity::Reaction => &r.reactions[p.dof],
            };
            projection(ratio, p.basis)
        })
        .collect();
    let projections: Vec<_> = values
        .iter()
        .zip(retained.projections())
        .map(|(r, p)| projection(r, p.basis()))
        .collect();
    let mut schedules = vec![
        ("Context::prepare_with_budget", prepare.clone()),
        ("Context::solve_with_budget", cat(&[&[4], &solve])),
        (
            if hints.is_empty() {
                "FunctionalPlan::new"
            } else {
                "FunctionalPlan::new_with_hints"
            },
            cat(&[&[4], &desc, &plan_hints]),
        ),
        ("Response::evaluate_functionals", cat(&[&[4], &eval])),
        ("Response::gap_proof_with_budget", cat(&[&[4], &contact])),
    ];
    schedules.extend(
        leaves
            .iter()
            .map(|p| ("Response::project_with_budget", cat(&[&[4], p]))),
    );
    schedules.extend(
        projections
            .iter()
            .map(|p| ("FunctionalSet::project", cat(&[&[4], p]))),
    );
    let value_scalars = values.iter().map(ratio).sum();
    schedules.push((
        "FunctionalSet::retain",
        cat(&[
            &[
                4,
                16 * values.len() + 8,
                4 * (rr.projections.len() + rr.contacts.len()),
            ],
            &desc,
            &[value_scalars],
            &retention(s, c, r, rr),
        ]),
    ));
    schedules.push((
        "RetainedFunctionalSet::check_binding_with_budget",
        cat(&[&preflight, &desc]),
    ));
    // Replay: size retained inputs, rebuild preparation/solve, then contact,
    // DOF projections, functional evaluation and functional projections.
    let mut replay = cat(&[&[24], &desc, &desc, &[8], &preflight]);
    for ratio in r.displacement.iter().chain(&r.reactions) {
        replay.extend(ratio_preflight(ratio));
    }
    for b in &c.witnesses {
        replay.extend([8, 2 * witness(b)]);
    }
    for c in &rr.contacts {
        replay.extend(ratio_preflight(&c.penetration));
        replay.extend(ratio_preflight(&c.normalized_reaction));
    }
    replay.push(24 * rr.projections.len() + 16 * rr.contacts.len());
    for ratio in values {
        replay.extend(ratio_preflight(ratio));
    }
    replay.push(16 * values.len());
    replay.extend(prepare);
    replay.extend(solve);
    replay.push(8 * s.dofs());
    replay.extend(contact);
    replay.extend(leaves.into_iter().flatten());
    replay.extend(replay_hints);
    replay.extend(eval);
    replay.extend(projections.into_iter().flatten());
    schedules.push(("RetainedFunctionalSet::replay_against", replay));
    schedules
}

pub(super) fn legacy(s: &Source, r: &Response<'_>, retained: &RetainedResponse) -> Vec<Schedule> {
    let c = r.context;
    let prepare = preparation(s, c);
    let contact = contact(r, &retained.contacts[0]);
    let leaves: Vec<_> = retained
        .projections
        .iter()
        .map(|p| {
            let ratio = match p.quantity {
                Quantity::Displacement => &r.displacement[p.dof],
                Quantity::Reaction => &r.reactions[p.dof],
            };
            projection(ratio, p.basis)
        })
        .collect();
    let mut schedules = vec![
        ("Context::new", prepare.clone()),
        ("Context::solve", vec![]),
    ];
    for (p, charges) in retained.projections.iter().zip(&leaves) {
        let entry = match p.quantity {
            Quantity::Displacement => "Response::project_displacement",
            Quantity::Reaction => "Response::project_reaction",
        };
        schedules.push((entry, cat(&[&[4], charges])));
    }
    let verify = vec![8, 4 * s.dofs(), 4 * s.fixed.len()];
    let mut candidate = vec![4];
    candidate.extend(r.displacement.iter().map(|r| ratio(r) + 16));
    candidate.push(4 * s.fixed.len());
    candidate.extend(&verify);
    schedules.extend([
        ("Response::gap_sign_with_work", vec![12]),
        ("Response::signed_reaction_with_work", vec![12]),
        ("Response::verify_candidate_with_work", verify),
        (
            "Response::exact_representable_candidate_with_work",
            candidate,
        ),
        ("Response::gap_proof_with_work", contact.clone()),
        ("Response::retain_with_work", retention(s, c, r, retained)),
    ]);
    let mut replay = vec![
        16,
        s.dofs() * s.dofs() + 32 * s.dofs() + 4 * s.contributions.len(),
        copies(s, c, r, retained),
    ];
    replay.extend(prepare);
    // Legacy solve adds no explicit reservation; in particular there is no
    // 12*E+128 precharge before solve_charged.
    replay.extend(contact);
    for leaf in leaves {
        replay.push(8);
        replay.extend(leaf);
    }
    schedules.push(("RetainedResponse::replay_against", replay));
    schedules
}
