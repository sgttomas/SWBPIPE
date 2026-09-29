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
    let f = shifted_factor(&mut ctx, &sum, &guard, &profile, &[None]).unwrap();
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
            1,
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
