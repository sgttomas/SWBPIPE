//! K4 tests of `retained/bound.rs` on raw profiles (D1 revision 5a.3, R7
//! §4.1.6.3 item 7, Lemmas D and E; E-UC and SD-G5): V4's F2 family at P =
//! 128, 256 and 512, DS1's low-precision stress (P = 10 to 32, the kill of
//! R7-M27 at low precision), and the searched boundary profiles. Every value
//! equals the generator's emulation bit for bit (`profiles.txt`), and every
//! existing bound is at least the exact ‖K̃⁻¹‖₁, compared exactly.
//! The per-model bounds (blocks, est_c, the verification's own factor) are
//! tested in `scale_tests.rs`.
use super::super::wide::multi::{SupportedWidth, WideContext};
use super::super::wide::Wide;
use super::super::wide_sum::ExactWideSum;
use super::*;

#[allow(dead_code)]
#[path = "support.rs"]
mod support;
use support::{parse, tok};

const PROFILES: &str = include_str!("profiles.txt");

fn opt_tok<const L: usize>(v: &Option<Wide<L>>) -> String
where
    Wide<L>: SupportedWidth,
{
    v.as_ref().map(tok).unwrap_or_else(|| "-".to_string())
}

struct Profile<'a> {
    tag: &'a str,
    p: u32,
    n: usize,
    first: Vec<usize>,
    exact: (&'a str, &'a str),
    rows: Vec<Vec<&'a str>>,
    uc: Vec<&'a str>,
    shifts: Vec<Vec<&'a str>>,
}

fn profiles() -> Vec<Profile<'static>> {
    let mut out: Vec<Profile> = Vec::new();
    for line in PROFILES.lines() {
        let f: Vec<&str> = line.split_whitespace().collect();
        match f[0] {
            "prof" => {
                let n: usize = f[3].parse().unwrap();
                out.push(Profile {
                    tag: f[1],
                    p: f[2].parse().unwrap(),
                    n,
                    first: f[4..4 + n].iter().map(|x| x.parse().unwrap()).collect(),
                    exact: (f[4 + n], f[5 + n]),
                    rows: Vec::new(),
                    uc: Vec::new(),
                    shifts: Vec::new(),
                })
            }
            "prow" => out.last_mut().unwrap().rows.push(f[2..].to_vec()),
            "puc" => out.last_mut().unwrap().uc = f[2..].to_vec(),
            "pshf" => out.last_mut().unwrap().shifts.push(f[2..].to_vec()),
            "stresscount" => {}
            _ => panic!("{line}"),
        }
    }
    out
}

/// One profile's checks; returns (whether ⌈√n⌉/σ fell below the exact norm
/// on a shift whose pivots passed: R7-M27's bound, per shift).
fn check<const L: usize>(pr: &Profile) -> Vec<bool>
where
    Wide<L>: SupportedWidth,
{
    let mut ctx = WideContext::<L>::new(pr.p).unwrap();
    let mut sum = ExactWideSum::new();
    let guard = StageGuard::unlimited();
    let rows: Vec<Vec<Wide<L>>> = pr
        .rows
        .iter()
        .map(|r| r.iter().map(|t| parse::<L>(t)).collect())
        .collect();
    let profile = ScaledProfile::from_rows(pr.first.clone(), rows, vec![0; pr.n]);
    let gamma = gamma_m(&mut ctx, &mut sum, pr.n).unwrap();
    // The unshifted factor (the loop with no shift): Uc when every pivot passes.
    let f = shifted_factor(&mut ctx, &sum, &guard, &profile, &[None], &mut [None]).unwrap();
    if pr.uc[0] == "fail" {
        assert!(f.failed[0], "{}", pr.tag);
    } else {
        assert!(!f.failed[0], "{}", pr.tag);
        let b = &uc_bounds(
            &mut ctx,
            &mut sum,
            &guard,
            &f,
            profile.block_of_row(),
            &mut [None],
            &gamma,
        )
        .unwrap()[0];
        let got = [tok(&b.u), tok(&b.n_l), tok(&b.t), opt_tok(&b.uc)];
        assert_eq!(got.to_vec(), pr.uc[1..5].to_vec(), "{} Uc", pr.tag);
        if let Some(uc) = &b.uc {
            assert!(
                support::wide_at_least(uc, pr.exact.0, pr.exact.1),
                "{}: Uc below the exact norm",
                pr.tag
            );
        }
    }
    let mut m27 = Vec::new();
    for s in &pr.shifts {
        let sigma = parse::<L>(s[0]);
        let (res, count) = shift_schedule(
            &mut ctx,
            &mut sum,
            &guard,
            &profile,
            &gamma,
            &[(0, sigma, pr.n)],
            &mut [None],
        )
        .unwrap();
        let r = &res[0].1;
        let got = [
            count.to_string(),
            r.tries.to_string(),
            opt_tok(&r.n_l),
            opt_tok(&r.delta),
            opt_tok(&r.sigma_prime),
            opt_tok(&r.s),
        ];
        assert_eq!(got.to_vec(), s[1..7].to_vec(), "{} shift", pr.tag);
        if let Some(sv) = &r.s {
            assert!(
                support::wide_at_least(sv, pr.exact.0, pr.exact.1),
                "{}: S below the exact norm",
                pr.tag
            );
        }
        // R7-M27's bound ⌈√n⌉/σ (upward), where the shifted pivots passed.
        if r.sigma_prime.is_some() {
            let root = Wide::<L>::from_f64(ceil_sqrt(pr.n) as f64).unwrap();
            let bound = super::super::directed::div_toward(
                &mut ctx,
                &mut sum,
                &root,
                &r.sigma,
                super::super::directed::Toward::Up,
            )
            .unwrap();
            let below = !support::wide_at_least(&bound, pr.exact.0, pr.exact.1);
            assert_eq!(below, s[7] == "1", "{} M27 flag", pr.tag);
            m27.push(below);
        }
    }
    m27
}

fn run(pr: &Profile) -> Vec<bool> {
    match pr.p {
        0..=256 => check::<4>(pr),
        257..=512 => check::<8>(pr),
        _ => check::<16>(pr),
    }
}

#[test]
fn v4s_f2_family_bounds_equal_the_emulation_and_are_never_below_the_exact_norm() {
    let all = profiles();
    let f2: Vec<&Profile> = all.iter().filter(|p| p.tag.starts_with("F2-")).collect();
    assert!(f2.len() >= 17, "{}", f2.len());
    for pr in f2 {
        run(pr);
    }
    // F2-m100 at 128: U alone falls below the exact norm (the γ_m term is needed).
    let m100 = all
        .iter()
        .find(|p| p.tag == "F2-m100-p128")
        .expect("F2-m100-p128");
    assert_eq!(
        m100.uc.last().copied(),
        Some("1"),
        "U below exact on F2-m100 at 128"
    );
}

#[test]
fn the_low_precision_stress_keeps_every_bound_above_the_exact_norm_and_m27s_does_not() {
    let all = profiles();
    let stress: Vec<&Profile> = all.iter().filter(|p| p.tag.starts_with("ST")).collect();
    assert!(stress.len() >= 400, "{}", stress.len());
    let mut killers = 0;
    for pr in stress {
        killers += run(pr).into_iter().filter(|&b| b).count();
    }
    // Lemma E's backward-error and rounding terms are load-bearing at low
    // precision: R7-M27's σ′ = σ falls below the exact norm (DS1: 27 of 20,000).
    assert!(killers >= 1, "no R7-M27 killer in the stress");
}

#[test]
fn sd_g5s_searched_profiles_reach_each_boundary_of_7b_to_7d() {
    let all = profiles();
    let get = |tag: &str| {
        all.iter()
            .find(|p| p.tag == tag)
            .unwrap_or_else(|| panic!("{tag}"))
    };
    for tag in [
        "SD-T1",
        "SD-TBELOW",
        "SD-HALF",
        "SD-FAIL3-UC",
        "SD-FAIL3-NONE",
        "SD-SPNEG",
    ] {
        run(get(tag));
    }
    // t at 1: Uc does not exist; t one ulp below 1: it does.
    let t1 = get("SD-T1");
    assert_eq!((t1.uc[3], t1.uc[4]), ("+8p0", "-"));
    let below = get("SD-TBELOW");
    assert_ne!(below.uc[4], "-");
    assert_eq!(
        support::parse::<4>(below.uc[3]).cmp_value(&Wide::<4>::ONE),
        std::cmp::Ordering::Less
    );
    // The shift fails once and succeeds at σ/2.
    let half = get("SD-HALF");
    assert_eq!((half.shifts[0][2], half.shifts[0][6] != "-"), ("2", true));
    // Three failures: B = Uc where Uc exists; neither bound (`uc`) where not.
    let fail_uc = get("SD-FAIL3-UC");
    assert_eq!((fail_uc.shifts[0][1], fail_uc.shifts[0][6]), ("3", "-"));
    assert_ne!(fail_uc.uc[4], "-");
    let fail_none = get("SD-FAIL3-NONE");
    assert_eq!((fail_none.shifts[0][1], fail_none.shifts[0][6]), ("3", "-"));
    assert_eq!(fail_none.uc[4], "-");
    // Passed pivots with σ′ ≤ 0: no S, and no retry.
    let spneg = get("SD-SPNEG");
    let sp = support::parse::<4>(spneg.shifts[0][5]);
    assert!(sp.is_zero() || sp.is_sign_negative());
    assert_eq!((spneg.shifts[0][2], spneg.shifts[0][6]), ("1", "-"));
}

#[test]
fn the_product_of_two_exact_sums_is_exact() {
    // a·b for signed multi-limb sums, against schoolbook integers.
    let mut rng = support::SplitMix64(support::seed_of("K4PRD5A3"));
    for _ in 0..200 {
        let na = (rng.next() % 5 + 1) as usize;
        let nb = (rng.next() % 5 + 1) as usize;
        let ma: Vec<u64> = (0..na).map(|_| rng.next()).collect();
        let mb: Vec<u64> = (0..nb).map(|_| rng.next()).collect();
        let (ea, eb) = (
            (rng.next() % 200) as i64 - 100,
            (rng.next() % 200) as i64 - 100,
        );
        let (sa, sb) = (rng.next() & 1 == 1, rng.next() & 1 == 1);
        let mut a = ExactWideSum::new();
        a.add_integer(sa, &ma, ea).unwrap();
        let mut b = ExactWideSum::new();
        b.add_integer(sb, &mb, eb).unwrap();
        let mut got = ExactWideSum::new();
        got.add_product_of(&a, &mut b, false).unwrap();
        let mut want = ExactWideSum::new();
        want.add_integer(sa != sb, &support::big_mul(&ma, &mb), ea + eb)
            .unwrap();
        // got − want = 0 exactly.
        got.add_scaled(&want, true, 1, 0).unwrap();
        assert_eq!(got.signum(), 0);
    }
}

// ---------------------------------------------------------------- T3 KF3: amendment A2

/// A block whose comparison-matrix bound outgrows the exact sum's span with a
/// modest inverse: K̃ = L·Lᵀ, L = I − N, N's node-to-next-node block
/// B = m·[[1, −1], [1, −1]] with B² = 0, so L⁻¹ = I + N and
/// ‖K̃⁻¹‖₁ = (1 + 2m)² exactly, while M(L)⁻¹ grows as (2m)^k over k nodes.
/// The profile's rows (K̃ exact at any P ≥ 64 for m = 2^20): per node, the
/// diagonal block I + BBᵀ (I at the first node) and the block −B below it.
fn nilpotent_chain(nodes: usize, m: f64) -> (Vec<usize>, Vec<Vec<f64>>) {
    let n = 2 * nodes;
    let mut first = Vec::with_capacity(n);
    let mut rows = Vec::with_capacity(n);
    for k in 0..nodes {
        let d = if k == 0 { 1.0 } else { 1.0 + 2.0 * m * m };
        let o = if k == 0 { 0.0 } else { 2.0 * m * m };
        let f = if k == 0 { 2 * k } else { 2 * k - 2 };
        // Row 2k: (−B row 0 = (−m, +m)), d.
        let mut r0 = Vec::new();
        if k > 0 {
            r0.extend([-m, m]);
        }
        r0.push(d);
        // Row 2k + 1: (−B row 1 = (−m, +m)), o, d.
        let mut r1 = Vec::new();
        if k > 0 {
            r1.extend([-m, m]);
        }
        r1.extend([o, d]);
        first.push(f);
        rows.push(r0);
        first.push(f);
        rows.push(r1);
    }
    (first, rows)
}

fn wide_rows<const L: usize>(rows: &[Vec<f64>]) -> Vec<Vec<Wide<L>>>
where
    Wide<L>: SupportedWidth,
{
    rows.iter()
        .map(|r| r.iter().map(|&v| Wide::<L>::from_f64(v).unwrap()).collect())
        .collect()
}

/// A small well-conditioned block (tridiagonal 4, −1) appended after `rows`.
fn with_tridiagonal(
    first: &mut Vec<usize>,
    rows: &mut Vec<Vec<f64>>,
    blk: &mut Vec<u32>,
    n: usize,
) {
    let start = rows.len();
    let b = blk.last().map_or(0, |&b| b + 1);
    for i in 0..n {
        let f = if i == 0 { start + i } else { start + i - 1 };
        first.push(f);
        rows.push(if i == 0 { vec![4.0] } else { vec![-1.0, 4.0] });
        blk.push(b);
    }
}

const KF3_M: f64 = 1_048_576.0; // 2^20

#[test]
fn kf3_a_refused_uc_is_unavailable_for_its_block_and_b_is_s() {
    // U1 and U2 (T3 KF3; D1 revision 5a.3 amendment A2).
    let p = 256;
    let nodes = 200;
    let (mut first, mut rows) = nilpotent_chain(nodes, KF3_M);
    let mut blk = vec![0u32; rows.len()];
    let n0 = rows.len();
    with_tridiagonal(&mut first, &mut rows, &mut blk, 12);
    let mut ctx = WideContext::<4>::new(p).unwrap();
    let mut sum = ExactWideSum::new();
    let guard = StageGuard::unlimited();
    let profile = ScaledProfile::from_rows(first.clone(), wide_rows::<4>(&rows), blk.clone());
    let gamma = gamma_m(&mut ctx, &mut sum, rows.len()).unwrap();
    let f = shifted_factor(
        &mut ctx,
        &sum,
        &guard,
        &profile,
        &[None, None],
        &mut [None, None],
    )
    .unwrap();
    assert_eq!(f.failed, vec![false, false]);
    let bounds = uc_bounds(
        &mut ctx,
        &mut sum,
        &guard,
        &f,
        &blk,
        &mut [None, None],
        &gamma,
    )
    .unwrap();
    // Block 0: refused (a backward sum spans more than 8,128 bits), not a stop.
    let r = bounds[0].refused.expect("block 0 refused");
    assert_eq!((r.kind, r.pass), (RefusalKind::Span, BoundPass::Backward));
    assert!(r.row < n0, "{r:?}");
    assert_eq!(bounds[0].uc, None);
    // Block 1 is bit-identical to its standalone run (the passes never cross
    // a block; the same γ_m).
    let (f1, r1): (Vec<usize>, Vec<Vec<f64>>) = (
        first[n0..].iter().map(|&x| x - n0).collect(),
        rows[n0..].to_vec(),
    );
    let alone = ScaledProfile::from_rows(f1, wide_rows::<4>(&r1), vec![0; r1.len()]);
    let fa = shifted_factor(&mut ctx, &sum, &guard, &alone, &[None], &mut [None]).unwrap();
    let ba = uc_bounds(
        &mut ctx,
        &mut sum,
        &guard,
        &fa,
        alone.block_of_row(),
        &mut [None],
        &gamma,
    )
    .unwrap();
    assert_eq!(bounds[1], ba[0]);
    assert!(bounds[1].refused.is_none() && bounds[1].uc.is_some());
    // S_c for block 0, at σ below λ_min (λ_min ≥ 1/(1 + 2m)² > 2^-43).
    let sigma = Wide::<4>::ONE.mul_pow2(-44).unwrap();
    let (shifts, count) = shift_schedule(
        &mut ctx,
        &mut sum,
        &guard,
        &profile,
        &gamma,
        &[(0, sigma, n0)],
        &mut [None, None],
    )
    .unwrap();
    assert_eq!(count, 1);
    let s = shifts[0].1.s.expect("S_c exists");
    assert!(shifts[0].1.refused.is_none());
    // B_c = S_c (Uc_c unavailable is +∞), never below the exact norm (1 + 2m)².
    assert_eq!(certified(&bounds[0].uc, &Some(s)), Some(s));
    let norm = (1.0 + 2.0 * KF3_M) * (1.0 + 2.0 * KF3_M);
    let mut diff = ExactWideSum::new();
    diff.add_wide(&s, false).unwrap();
    diff.add_binary64(norm, true).unwrap();
    assert!(diff.signum() >= 0, "S below the exact norm");
    // certify, as verify_state composes it: block 0 carries data and takes S.
    let est = vec![Wide::<4>::ONE.mul_pow2(43).unwrap(), Wide::<4>::ONE];
    let mut s_refused = vec![None; 2];
    let blocks = FreeBlocks {
        of: blk.clone(),
        positions: vec![(0..n0).collect(), (n0..rows.len()).collect()],
        body: vec![0, 0],
    };
    let start = shift_start(
        &mut ctx,
        &mut sum,
        &blocks,
        &bounds,
        &est,
        &[true, true],
        &mut s_refused,
    )
    .unwrap();
    assert_eq!(start.iter().map(|s| s.0).collect::<Vec<_>>(), vec![0]);
    let (shifts, _) = shift_schedule(
        &mut ctx,
        &mut sum,
        &guard,
        &profile,
        &gamma,
        &start,
        &mut [None, None],
    )
    .unwrap();
    let (certs, uc_missing, stop) =
        certificates(&blocks, &bounds, &est, &[true, true], &shifts, &s_refused);
    assert_eq!((uc_missing, stop), (None, None));
    assert_eq!(certs[0].b, shifts[0].1.s);
    assert_eq!(certs[1].b, bounds[1].uc);
    // Where both bounds exist, B_c is the smaller: block 1 shifted at σ = 1
    // (λ_min of the tridiagonal (4, −1) block exceeds 2), where S_c > Uc_c.
    let one = Wide::<4>::ONE;
    let (both, _) = shift_schedule(
        &mut ctx,
        &mut sum,
        &guard,
        &profile,
        &gamma,
        &[(1, one, rows.len() - n0)],
        &mut [None, None],
    )
    .unwrap();
    let s1 = both[0].1.s.expect("S_c of block 1");
    let uc1 = bounds[1].uc.unwrap();
    assert_eq!(s1.cmp_value(&uc1), std::cmp::Ordering::Greater);
    assert_eq!(certified(&Some(uc1), &Some(s1)), Some(uc1));
    let (certs, _, _) = certificates(&blocks, &bounds, &est, &[false, true], &both, &[None, None]);
    assert_eq!(certs[1].b, Some(uc1));
    let evidence = block_refusals(&bounds, &s_refused);
    assert_eq!(
        evidence,
        vec![BlockRefusal {
            block: 0,
            bound: CertifiedBound::Uc,
            refusal: r
        }]
    );
}

#[test]
fn kf3_neither_bound_stops_a_block_with_data_and_not_one_without() {
    // U5 and 7d's precedence (ROOT's rulings 1 and 2 on I19's plan).
    let (first, rows) = nilpotent_chain(200, KF3_M);
    let n = rows.len();
    let mut ctx = WideContext::<4>::new(256).unwrap();
    let mut sum = ExactWideSum::new();
    let guard = StageGuard::unlimited();
    let profile = ScaledProfile::from_rows(first, wide_rows::<4>(&rows), vec![0; n]);
    let gamma = gamma_m(&mut ctx, &mut sum, n).unwrap();
    let f = shifted_factor(&mut ctx, &sum, &guard, &profile, &[None], &mut [None]).unwrap();
    let bounds = uc_bounds(
        &mut ctx,
        &mut sum,
        &guard,
        &f,
        &vec![0; n],
        &mut [None],
        &gamma,
    )
    .unwrap();
    let r = bounds[0].refused.expect("refused");
    let blocks = FreeBlocks {
        of: vec![0; n],
        positions: vec![(0..n).collect()],
        body: vec![0],
    };
    let est = vec![Wide::<4>::ZERO];
    // With data and no S (est_c = 0: no shift): the attempt stops.
    let (certs, uc_missing, stop) = certificates(&blocks, &bounds, &est, &[true], &[], &[None]);
    assert_eq!((certs[0].b, uc_missing, stop), (None, None, Some((0, r))));
    assert_eq!(r.stop(), AttemptStop::Span);
    // Without data: no bound needed, no stop, the refusal only recorded.
    let (certs, uc_missing, stop) = certificates(&blocks, &bounds, &est, &[false], &[], &[None]);
    assert_eq!((certs[0].b, uc_missing, stop), (None, None, None));
    // Precedence: a formed-but-missing block 0 (`uc`) and a refused block 1.
    let missing = BlockBound {
        u: Wide::<4>::ONE,
        n_l: Wide::<4>::ONE,
        t: Wide::<4>::ONE,
        uc: None,
        refused: None,
    };
    let two = FreeBlocks {
        of: vec![0, 1],
        positions: vec![vec![0], vec![1]],
        body: vec![0, 0],
    };
    let both = [missing, bounds[0].clone()];
    let (_, uc_missing, stop) = certificates(
        &two,
        &both,
        &[Wide::<4>::ZERO; 2],
        &[true, true],
        &[],
        &[None, None],
    );
    assert_eq!((uc_missing, stop), (Some(0), Some((1, r))));
}

#[test]
fn kf3_a_budget_stop_inside_the_uc_passes_is_a_stop_not_a_refusal() {
    // U3: only Span and Exponent are refusals (A2); a budget stop propagates.
    let (first, rows) = nilpotent_chain(200, KF3_M);
    let n = rows.len();
    let mut ctx = WideContext::<4>::new(256).unwrap();
    let mut sum = ExactWideSum::new();
    let profile = ScaledProfile::from_rows(first, wide_rows::<4>(&rows), vec![0; n]);
    let gamma = gamma_m(&mut ctx, &mut sum, n).unwrap();
    let f = shifted_factor(
        &mut ctx,
        &sum,
        &StageGuard::unlimited(),
        &profile,
        &[None],
        &mut [None],
    )
    .unwrap();
    let before = super::super::adaptive::lme(&ctx) + sum.work().limb_multiply_equivalents();
    let guard = StageGuard::with_case_room(before + 2_000);
    let got = uc_bounds(
        &mut ctx,
        &mut sum,
        &guard,
        &f,
        &vec![0; n],
        &mut [None],
        &gamma,
    );
    assert_eq!(
        got,
        Err(AttemptStop::Budget(
            super::super::adaptive::BudgetScope::Case
        ))
    );
    // `refusable` passes every stop but Span and Exponent through.
    let mut slot = None;
    let budget = AttemptStop::Budget(super::super::adaptive::BudgetScope::Invocation);
    assert_eq!(
        refusable::<()>(
            Err(budget.clone()),
            &mut sum,
            &mut slot,
            BoundPass::Forward,
            3
        ),
        Err(budget)
    );
    assert_eq!(slot, None);
    assert_eq!(
        refusable::<()>(
            Err(AttemptStop::Exponent),
            &mut sum,
            &mut slot,
            BoundPass::Pivot,
            4
        ),
        Ok(None)
    );
    assert_eq!(
        slot,
        Some(BoundRefusal {
            kind: RefusalKind::Exponent,
            pass: BoundPass::Pivot,
            row: 4
        })
    );
}

#[test]
fn kf3_a_refused_s_is_unavailable_and_b_is_uc() {
    // U4: σ_c = 2^-9000 (a σ no estimate gives): σ′'s subtraction spans more
    // than 8,128 bits, so S_c is refused and not retried, and B_c = Uc_c.
    let mut first = Vec::new();
    let mut rows = Vec::new();
    let mut blk = Vec::new();
    with_tridiagonal(&mut first, &mut rows, &mut blk, 12);
    let n = rows.len();
    let mut ctx = WideContext::<4>::new(256).unwrap();
    let mut sum = ExactWideSum::new();
    let guard = StageGuard::unlimited();
    let profile = ScaledProfile::from_rows(first, wide_rows::<4>(&rows), blk.clone());
    let gamma = gamma_m(&mut ctx, &mut sum, n).unwrap();
    let f = shifted_factor(&mut ctx, &sum, &guard, &profile, &[None], &mut [None]).unwrap();
    let bounds = uc_bounds(&mut ctx, &mut sum, &guard, &f, &blk, &mut [None], &gamma).unwrap();
    let uc = bounds[0].uc.expect("Uc_c exists");
    let sigma = Wide::<4>::ONE.mul_pow2(-9000).unwrap();
    let (shifts, count) = shift_schedule(
        &mut ctx,
        &mut sum,
        &guard,
        &profile,
        &gamma,
        &[(0, sigma, n)],
        &mut [None],
    )
    .unwrap();
    let r = &shifts[0].1;
    assert_eq!((count, r.tries, r.s), (1, 1, None));
    let refusal = r.refused.expect("S_c refused");
    assert_eq!(
        (refusal.kind, refusal.pass),
        (RefusalKind::Span, BoundPass::ShiftForm)
    );
    assert_eq!(certified(&Some(uc), &r.s), Some(uc));
    // The accumulator is exact after the refusal (reset in full, U6).
    let two = super::super::directed::add_toward(
        &mut ctx,
        &mut sum,
        &Wide::<4>::ONE,
        &Wide::<4>::ONE,
        super::super::directed::Toward::Up,
    )
    .unwrap();
    assert_eq!(two, Wide::<4>::from_f64(2.0).unwrap());
}

#[test]
fn kf3_a_stop_after_an_s_refusal_keeps_it_in_the_evidence() {
    // RV23-1 (ROOT's ruling on RV23's review), 7c: a refusal recorded in the
    // schedule reaches `s_refused` when a later stop ends the schedule.
    // - Block 0, K̃ = [[1, x], [x, 3x²]] with x = 2^8200: N′_L's column sum
    //   1 + |l| spans more than 8,128 bits (`NlColumn`, in the factorization
    //   in progress).
    // - Block 1, tridiagonal (4, −1) at σ = 2^-9000: σ′ is refused
    //   (`ShiftForm`) and kept in the results.
    // - Block 2, tridiagonal (4, −1) at σ = 8: it fails at 8 and 4 and passes
    //   at 2 (λ_min > 2), so the schedule takes three factorizations.
    let p = 256;
    let mut first = vec![0usize, 0];
    let mut rows = vec![vec![1.0], vec![0.0, 0.0]];
    let mut blk = vec![0u32, 0];
    with_tridiagonal(&mut first, &mut rows, &mut blk, 12);
    with_tridiagonal(&mut first, &mut rows, &mut blk, 12);
    let n = rows.len();
    let mut wide = wide_rows::<4>(&rows);
    let x = Wide::<4>::ONE.mul_pow2(8200).unwrap();
    let three_x2 = Wide::<4>::from_f64(3.0).unwrap().mul_pow2(16400).unwrap();
    wide[1] = vec![x, three_x2];
    let profile = ScaledProfile::from_rows(first, wide, blk);
    let gamma = {
        let mut ctx = WideContext::<4>::new(p).unwrap();
        gamma_m(&mut ctx, &mut ExactWideSum::new(), n).unwrap()
    };
    let start = [
        (0, Wide::<4>::ONE.mul_pow2(-10).unwrap(), 2),
        (1, Wide::<4>::ONE.mul_pow2(-9000).unwrap(), 12),
        (2, Wide::<4>::from_f64(8.0).unwrap(), 12),
    ];
    let run = |guard: StageGuard| {
        let mut ctx = WideContext::<4>::new(p).unwrap();
        let mut sum = ExactWideSum::new();
        let mut s_refused = vec![None; 3];
        let got = shift_schedule(
            &mut ctx,
            &mut sum,
            &guard,
            &profile,
            &gamma,
            &start,
            &mut s_refused,
        );
        let used = super::super::adaptive::lme(&ctx) + sum.work().limb_multiply_equivalents();
        (got, s_refused, used)
    };
    // Unlimited: the results carry both refusals; `s_refused` is the caller's.
    let (got, untouched, total) = run(StageGuard::unlimited());
    let (res, count) = got.unwrap();
    assert_eq!(count, 3);
    assert_eq!(untouched, vec![None; 3]);
    let want: Vec<Option<BoundRefusal>> = res.iter().map(|r| r.1.refused).collect();
    assert_eq!(
        want[0],
        Some(BoundRefusal {
            kind: RefusalKind::Span,
            pass: BoundPass::NlColumn,
            row: 0
        })
    );
    assert_eq!(
        want[1].map(|r| (r.kind, r.pass)),
        Some((RefusalKind::Span, BoundPass::ShiftForm))
    );
    assert!(want[2].is_none() && res[2].1.s.is_some());
    // Every case room: a budget stop keeps exactly the refusals recorded
    // before it, never another, and they only grow with the room. Both the
    // factorization in progress's (block 0 alone) and the results' (blocks 0
    // and 1) are carried out.
    let (mut in_flight, mut kept, mut last) = (0, 0, 0);
    let step = (total / 6_000).max(1);
    let mut room = 0;
    while room < total {
        let (got, s, _) = run(StageGuard::with_case_room(room));
        let level = match got {
            // Past the schedule's last guard check: it completes, and
            // `s_refused` is the caller's.
            Ok(_) => {
                assert_eq!(s, vec![None; 3], "room {room}");
                3
            }
            Err(stop) => {
                assert_eq!(
                    stop,
                    AttemptStop::Budget(super::super::adaptive::BudgetScope::Case),
                    "room {room}"
                );
                match (s[0], s[1], s[2]) {
                    (None, None, None) => 0,
                    (Some(a), None, None) if Some(a) == want[0] => 1,
                    (Some(a), Some(b), None) if Some(a) == want[0] && Some(b) == want[1] => 2,
                    other => panic!("room {room}: {other:?}"),
                }
            }
        };
        assert!(level >= last, "room {room}: {level} after {last}");
        last = level;
        in_flight += usize::from(level == 1);
        kept += usize::from(level == 2);
        room += step;
    }
    println!("RV23-1 (S): total {total} LME, step {step}: {in_flight} stops in flight, {kept} after the results kept both");
    assert!(in_flight > 0 && kept > 0, "{in_flight} {kept}");
}
