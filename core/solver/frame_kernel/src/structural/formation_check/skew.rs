//! K-D5 on a skew case's transformed system (DEL-04-03 Work `skew-restraints.md`
//! §10): the existing evaluation applied to the intended Qᵀ K_int Q v = Qᵀ f_int,
//! with Q the basis's exact node-local matrix. No new certificate, no new
//! threshold. The binary64 factorization, the corrections w and w₂, the
//! contraction test, σ and the Neumann bound are today's, on coordinates.
//!
//! - **Loads (ρ, §10.2).** Each physical ledger term or re-formed primitive on
//!   DOF i enters each coordinate row r with weight q_ir (`RestraintBasis::q_row`),
//!   exactly: q·v by `add_product`, q·a·b and a primitive's Wide2 value by error-free
//!   parts; radii × |q_ir| upward.
//! - **Actions (ρ, §10.2).** −Σ q_ir·k_ij·q_jc·x_c: the binary64 parts of q_ir·q_jc·x_c
//!   (`exact_triple_parts`), each times the Wide2 entry by `add_product_to`. A
//!   product outside the split range (P1) is the named
//!   `restraint_basis: split range at DOF {dof}` detail; nothing is rounded.
//! - **Allowance (ε_E, §10.3).** Each physical contribution's formation bound,
//!   charged to (r, c) with weight |q_ir|·|q_jc| upward; the assembly and
//!   transform charge is today's loop on the coordinate contributions, whose
//!   exact sum is QᵀK_asmQ (`transform_contributions`), against the stored K′.
//! - **D5C-1 (§10.4).** Judged physically: |w_phys,i| ≤ Σ_c|q_ic||w_c| upward,
//!   against max(|u_p,i|, S*) with u_p = fl(Qv); the reported DOF is physical.
//!
//! The returned `MemberBoundInputs` is in coordinates; the caller lifts it with
//! `RestraintBasis::lift_member_bound` before any recovery.
use super::*;
use crate::restraint_basis::{exact_triple_parts, RestraintBasis, SkewFormation};

const TINY: f64 = f64::from_bits(1);

fn split_range(dof: usize) -> Failure {
    Failure::Closure(format!("restraint_basis: split range at DOF {dof}"))
}

/// The intended skew spring block at p (§6.1): k·d_i d_j/(dᵀd) from the
/// **authored** components (scale-free), each entry formed once for i ≤ j and
/// mirrored.
pub(super) fn skew_spring_matrix(a: &mut WideArith, d: [f64; 3], k: f64) -> Result<[[Wide2; 3]; 3], WideError> {
    let d = lift3(d)?;
    let mut n = Wide2::ZERO;
    for x in &d {
        let square = a.mul(x, x)?;
        n = a.add(&n, &square)?;
    }
    let k = lift(k)?;
    let mut out = [[Wide2::ZERO; 3]; 3];
    for j in 0..3 {
        for i in 0..=j {
            let kd = a.mul(&k, &d[i])?;
            let kdd = a.mul(&kd, &d[j])?;
            out[i][j] = a.div(&kdd, &n)?;
            out[j][i] = out[i][j];
        }
    }
    Ok(out)
}

/// Adds w·k·(q_ir·q_jc·x) to `row` exactly (with `k` a Wide2 entry and `w` = ±1).
fn add_weighted(
    row: &mut ExactAccumulator,
    psi: &mut f64,
    k: &Wide2,
    q_ir: f64,
    q_jc: f64,
    x: f64,
    dof: usize,
) -> Result<(), Failure> {
    if x == 0.0 || k.is_zero() {
        return Ok(());
    }
    if q_ir == 1.0 && q_jc == 1.0 {
        if k.neg().add_product_to(row, x)? {
            *psi = add_up(*psi, mul_up(TINY, x.abs()));
        }
        return Ok(());
    }
    let parts = exact_triple_parts(q_ir, q_jc, x).ok_or_else(|| split_range(dof))?;
    for part in parts.into_iter().filter(|p| *p != 0.0) {
        if k.neg().add_product_to(row, part)? {
            *psi = add_up(*psi, mul_up(TINY, part.abs()));
        }
    }
    Ok(())
}

/// `subtract_action` on coordinates: rows[r] −= (Qᵀ K_int Q x)_r for free r.
fn subtract_action(
    source: &FormationSource,
    basis: &RestraintBasis,
    free_index: &[Option<usize>],
    x: &[f64],
    rows: &mut [ExactAccumulator],
    psi: &mut [f64],
) -> Result<(), Failure> {
    let entry = |i: usize, j: usize, k: &Wide2, rows: &mut [ExactAccumulator], psi: &mut [f64]| -> Result<(), Failure> {
        for &(r, q_ir) in basis.q_row(i).as_slice() {
            let Some(fr) = free_index[r] else { continue };
            for &(c, q_jc) in basis.q_row(j).as_slice() {
                add_weighted(&mut rows[fr], &mut psi[fr], k, q_ir, q_jc, x[c], r)?;
            }
        }
        Ok(())
    };
    intended_stiffness_pass(source, |ni, nj, k| {
        if ni >= source.node_count || nj >= source.node_count || ni == nj {
            return Err(Failure::Shape("element node"));
        }
        let map = element_dof_map(ni, nj);
        for a in 0..12 {
            for c in 0..12 {
                entry(map[a], map[c], &k[a][c], rows, psi)?;
            }
        }
        Ok(())
    })?;
    for &(dof, k) in &source.springs {
        if dof >= x.len() {
            return Err(Failure::Shape("spring DOF"));
        }
        entry(dof, dof, &lift(k)?, rows, psi)?;
    }
    let mut arith = WideArith::new(FORMATION_PRECISION)?;
    for spring in basis.springs() {
        let block = skew_spring_matrix(&mut arith, spring.direction.authored(), spring.stiffness)?;
        let base = spring.node * DOF_PER_NODE;
        for (i, row) in block.iter().enumerate() {
            for (j, k) in row.iter().enumerate() {
                entry(base + i, base + j, k, rows, psi)?;
            }
        }
    }
    Ok(())
}

/// ε_E on coordinates (§10.3): the physical formation bounds mapped with
/// |q_ir|·|q_jc|, plus today's assembly and projection charge on the
/// coordinate contributions against the stored K′.
fn epsilon_e<A, P>(
    system: &StructuralSystem<'_>,
    source: &FormationSource,
    skew: &SkewFormation<'_>,
    index: &[Option<usize>],
    exponents: &[i32],
    assembled: &A,
    projected: &P,
) -> Result<f64, Failure>
where
    A: Fn(usize, usize) -> f64,
    P: Fn(usize, usize) -> f64,
{
    let basis = skew.basis;
    let contributions = skew.physical_contributions;
    let coordinate = system
        .contributions
        .ok_or(Failure::Shape("stiffness contributions absent"))?;
    let n = system.free_dofs.len();
    let mut row_sums = vec![0.0; n];
    let mut col_sums = vec![0.0; n];
    let mut cursor = 0;
    let mut charge = |r: usize, c: usize, value: f64| {
        row_sums[r] = add_up(row_sums[r], value);
        col_sums[c] = add_up(col_sums[c], value);
    };
    // One physical contribution's formation difference, charged to every
    // free coordinate pair it reaches.
    let formation = |i: usize, j: usize, intended: &Wide2, value: f64, charge: &mut dyn FnMut(usize, usize, f64)| -> Result<(), Failure> {
        let mut diff = ExactAccumulator::new();
        let split = intended.split_binary64()?;
        for &x in split.terms() {
            diff.add(x)?;
        }
        diff.add(-value)?;
        for &(r, q_ir) in basis.q_row(i).as_slice() {
            let Some(fr) = index[r] else { continue };
            for &(c, q_jc) in basis.q_row(j).as_slice() {
                let Some(fc) = index[c] else { continue };
                let e = exponents[fr] + exponents[fc];
                let mut bound = exact_abs_scaled_up(&diff, e)?;
                if split.truncated_below_min_subnormal() {
                    bound = add_up(bound, scale_up(TINY, e));
                }
                charge(fr, fc, mul_up(mul_up(bound, q_ir.abs()), q_jc.abs()));
            }
        }
        Ok(())
    };
    intended_stiffness_pass(source, |ni, nj, k| {
        let map = element_dof_map(ni, nj);
        for r in 0..12 {
            for c in 0..12 {
                let entry = contributions
                    .get(cursor)
                    .ok_or(Failure::Shape("element contribution block missing"))?;
                cursor += 1;
                if entry.row != map[r] || entry.col != map[c] {
                    return Err(Failure::Shape("element contribution block identity"));
                }
                formation(map[r], map[c], &k[r][c], entry.value, &mut charge)?;
            }
        }
        Ok(())
    })?;
    for &(dof, k) in &source.springs {
        let entry = contributions
            .get(cursor)
            .ok_or(Failure::Shape("spring contribution missing"))?;
        cursor += 1;
        if entry.row != dof || entry.col != dof || entry.value.to_bits() != k.to_bits() {
            return Err(Failure::Shape("spring contribution binding"));
        }
    }
    // §10.3: each skew spring block, nine contributions row-major after the
    // springs, bound to its §6.1 bits; its intended entries are p-formed.
    let mut arith = WideArith::new(FORMATION_PRECISION)?;
    for (s, spring) in basis.springs().iter().enumerate() {
        let intended = skew_spring_matrix(&mut arith, spring.direction.authored(), spring.stiffness)?;
        let bits = basis.spring_block(s);
        let base = spring.node * DOF_PER_NODE;
        for i in 0..3 {
            for j in 0..3 {
                let entry = contributions
                    .get(cursor)
                    .ok_or(Failure::Shape("skew spring contribution missing"))?;
                cursor += 1;
                if entry.row != base + i || entry.col != base + j || entry.value.to_bits() != bits[i][j].to_bits() {
                    return Err(Failure::Shape("skew spring contribution binding"));
                }
                formation(base + i, base + j, &intended[i][j], entry.value, &mut charge)?;
            }
        }
    }
    if cursor != contributions.len() {
        return Err(Failure::Shape("extra stiffness contribution"));
    }
    // Assembly and transform: exact(Σ coordinate contributions) − K′_stored,
    // whose exact part is (Qᵀ(Σ_e contrib_e)Q)_rc, plus the projection term.
    let mut sums = vec![ExactAccumulator::new(); n];
    for (r, &i) in system.free_dofs.iter().enumerate() {
        for sum in &mut sums {
            *sum = ExactAccumulator::new();
        }
        for e in coordinate {
            if e.row == i {
                if let Some(c) = index[e.col] {
                    sums[c].add(e.value)?;
                }
            }
        }
        for (c, &j) in system.free_dofs.iter().enumerate() {
            let stored = assembled(i, j);
            sums[c].add(-stored)?;
            let assembly = exact_abs_scaled_up(&sums[c], exponents[r] + exponents[c])?;
            let mut projection = ExactAccumulator::new();
            let scaled = Wide2::from_f64(stored)?
                .mul_pow2(i64::from(exponents[r]) + i64::from(exponents[c]))?;
            let split = scaled.split_binary64()?;
            for &x in split.terms() {
                projection.add(x)?;
            }
            projection.add(-projected(r, c))?;
            let mut change = exact_abs_scaled_up(&projection, 0)?;
            if split.truncated_below_min_subnormal() {
                change = add_up(change, TINY);
            }
            charge(r, c, add_up(assembly, change));
        }
    }
    Ok(row_sums.into_iter().chain(col_sums).fold(0.0, f64::max))
}

/// `evaluate` on the transformed system; the same steps in the same order.
#[allow(clippy::too_many_arguments)]
pub(super) fn evaluate<F, A, P>(
    system: &StructuralSystem<'_>,
    source: &FormationSource,
    skew: &SkewFormation<'_>,
    scale_exponents: &[i32],
    v: &[f64],
    condition: (f64, f64, f64),
    assembled: &A,
    projected: &P,
    solve: &F,
    judged: bool,
    epsilon_cache: Option<&std::cell::Cell<Option<f64>>>,
) -> Result<(Option<FormationCheck>, Option<MemberBoundInputs>), Failure>
where
    F: Fn(&[f64]) -> Result<Vec<f64>, StructuralError>,
    A: Fn(usize, usize) -> f64,
    P: Fn(usize, usize) -> f64,
{
    let basis = skew.basis;
    let ledger = skew.physical_force;
    load_closure(source, ledger)?;
    straight_frames_bound(source)?;
    let n = system.force.len();
    let free = system.free_dofs;
    if source.node_count.checked_mul(DOF_PER_NODE) != Some(n)
        || basis.dimension() != n
        || ledger.len() != n
        || v.len() != n
        || scale_exponents.len() != free.len()
    {
        return Err(Failure::Shape("DOF map"));
    }
    let mut index = vec![None; n];
    for (r, &i) in free.iter().enumerate() {
        index[i] = Some(r);
    }
    let mut rho = vec![ExactAccumulator::new(); free.len()];
    let mut psi = vec![0.0; free.len()];
    for (t, term) in ledger.terms().iter().enumerate() {
        if term.dof >= n {
            return Err(Failure::Shape("force term DOF"));
        }
        let weights = basis.q_row(term.dof);
        if !ledger.closure_records()[t].formed {
            for &(r, q) in weights.as_slice() {
                let Some(fr) = index[r] else { continue };
                match term.kind {
                    crate::load_ledger::ForceTermKind::Term(value) => rho[fr].add_product(q, value)?,
                    crate::load_ledger::ForceTermKind::Product(a, b) => {
                        for part in exact_triple_parts(q, a, b).ok_or_else(|| split_range(r))? {
                            rho[fr].add(part)?;
                        }
                    }
                }
            }
        } else {
            for p in source.loads.iter().filter(|p| p.term_index == t) {
                let (value, radius) = load_reformation(p)
                    .map_err(|e| Failure::Closure(format!("{}: reformation {e:?}", term.source)))?;
                for &(r, q) in weights.as_slice() {
                    let Some(fr) = index[r] else { continue };
                    if value[p.local_row].add_product_to(&mut rho[fr], q)? {
                        psi[fr] = add_up(psi[fr], mul_up(TINY, q.abs()));
                    }
                    psi[fr] = add_up(psi[fr], mul_up(radius[p.local_row], q.abs()));
                }
            }
        }
    }
    subtract_action(source, basis, &index, v, &mut rho, &mut psi)?;
    let w = correction(&rho, free, scale_exponents, n, solve)?;
    // D5C-1, physically (§10.4): the lifted u_p = fl(Qv) and the physical DOFs
    // that move (every DOF that is not an axis restraint, the pivots included).
    let mut worst: Option<FormationCheck> = None;
    if judged {
        let u = basis.lift(v).map_err(|_| Failure::Shape("lifted displacement"))?;
        let physical_free: Vec<usize> = (0..n)
            .filter(|dof| basis.axis_restrained().binary_search(dof).is_err())
            .collect();
        let scales = body_scales(source, &physical_free, &u);
        for &i in &physical_free {
            let mut w_i = 0.0;
            for &(c, q) in basis.q_row(i).as_slice() {
                w_i = add_up(w_i, mul_up(q.abs(), w[c].abs()));
            }
            let (tr, ro) = scales[i / 6];
            let scale = u[i].abs().max(if i % 6 >= 3 { ro } else { tr });
            let doubled = mul_up(FORMATION_FACTOR, w_i);
            let ratio = if scale == 0.0 {
                if w_i != 0.0 { f64::INFINITY } else { 0.0 }
            } else {
                let criterion = FORMATION_CRITERION * scale;
                if doubled > criterion { doubled / criterion } else { 0.0 }
            };
            if ratio > 0.0 && worst.as_ref().is_none_or(|v| ratio > v.ratio) {
                worst = Some(FormationCheck {
                    reason: FormationCheckReason::Estimate,
                    global_dof: Some(i),
                    doubled_correction: doubled,
                    scale,
                    ratio,
                });
            }
        }
    }
    if worst.is_some() {
        return Ok((worst, None));
    }
    subtract_action(source, basis, &index, &w, &mut rho, &mut psi)
        .map_err(|e| Failure::Second(e.detail()))?;
    let w2 = correction(&rho, free, scale_exponents, n, solve)
        .map_err(|e| Failure::Second(e.detail()))?;
    let mut norm_w = Wide2::ZERO;
    let mut norm_w2 = Wide2::ZERO;
    for (r, &i) in free.iter().enumerate() {
        let x = Wide2::from_f64(w[i].abs())?.mul_pow2(-i64::from(scale_exponents[r]))?;
        let y = Wide2::from_f64(w2[i].abs())?.mul_pow2(-i64::from(scale_exponents[r]))?;
        if x.cmp_value(&norm_w).is_gt() {
            norm_w = x;
        }
        if y.cmp_value(&norm_w2).is_gt() {
            norm_w2 = y;
        }
    }
    if !norm_w2.is_zero() {
        let threshold = norm_w.mul_pow2(-20)?;
        if norm_w2.cmp_value(&threshold).is_gt() {
            return Err(Failure::Unavailable("contraction"));
        }
    }
    subtract_action(source, basis, &index, &w2, &mut rho, &mut psi)
        .map_err(|e| Failure::Second(e.detail()))?;
    let mut sigma = 0.0_f64;
    for ((r, &p), &e) in rho.iter().zip(&psi).zip(scale_exponents) {
        let magnitude = exact_abs_scaled_up(r, e)?;
        sigma = sigma.max(add_up(magnitude, scale_up(p, e)));
    }
    drop(rho);
    let epsilon = match epsilon_cache.and_then(std::cell::Cell::get) {
        Some(epsilon) => epsilon,
        None => {
            let epsilon = epsilon_e(system, source, skew, &index, scale_exponents, assembled, projected)?;
            if let Some(cache) = epsilon_cache {
                cache.set(Some(epsilon));
            }
            epsilon
        }
    };
    let (_rcond, est, _norm) = condition;
    let inverse = mul_up(1024.0, est);
    let neumann = mul_up(inverse, epsilon);
    if !neumann.is_finite() || neumann >= 1.0 {
        return Err(Failure::Unavailable("Neumann"));
    }
    let denominator = (1.0 - neumann).next_down();
    if denominator <= 0.0 {
        return Err(Failure::Unavailable("Neumann"));
    }
    let lambda = div_up(inverse, denominator);
    let lambda_sigma = mul_up(lambda, sigma);
    if !lambda_sigma.is_finite() {
        return Err(Failure::Unavailable("non-finite Lambda sigma"));
    }
    let mut exponents = vec![i32::MIN; n];
    for (r, &i) in free.iter().enumerate() {
        exponents[i] = scale_exponents[r];
    }
    Ok((
        None,
        Some(MemberBoundInputs { w, w2, exponents, lambda_sigma, lift_radius: Vec::new() }),
    ))
}
