//! O-13: tests of the arc recovery certificate (`arc-recovery-guard.md`;
//! T4-I34 DESIGN §6.1 C1–C6). Frozen data in
//! `validation/references/t4_o13/o13_fk_references.txt`: T4-I34's
//! independent 120-digit references (Gauss–Legendre K(z), exact chord frame
//! and exact end actions), three binary64 terms per value.
use super::*;

const O13: &str = include_str!("../../../../../validation/references/t4_o13/o13_fk_references.txt");

/// RD(1e-9), the guard's criterion.
const CRITERION: f64 = f64::from_bits(1e-9_f64.to_bits() - 1);

struct Stiffness {
    id: String,
    bend: CurvedFormation,
    /// K(z) entries, each three binary64 terms.
    k: Vec<Vec<Vec<f64>>>,
}

struct Rows {
    id: String,
    bend: CurvedFormation,
    u: [f64; 12],
    span_y: [f64; 3],
    /// The exact chord-frame end actions f(z, u), three terms each.
    f: Vec<Vec<f64>>,
}

fn bend_of(args: &[f64]) -> CurvedFormation {
    assert_eq!(args.len(), 17);
    CurvedFormation {
        node_i: 0,
        node_j: 1,
        coordinates_i: [args[0], args[1], args[2]],
        coordinates_j: [args[3], args[4], args[5]],
        radius: args[6],
        y_reference: [args[7], args[8], args[9]],
        elastic_modulus: args[10],
        shear_modulus: args[11],
        area: args[12],
        second_moment: args[13],
        torsion_constant: args[14],
        in_plane_flexibility_factor: args[15],
        out_of_plane_flexibility_factor: args[16],
    }
}

fn hexes(text: &str) -> Vec<f64> {
    text.split_whitespace().map(hex).collect()
}

fn relative() -> f64 {
    O13.lines()
        .find_map(|l| l.strip_prefix("relative "))
        .unwrap()
        .parse()
        .unwrap()
}

fn references() -> (Vec<Stiffness>, Vec<Rows>) {
    let (mut stiffness, mut rows) = (Vec::new(), Vec::new());
    let mut lines = O13
        .lines()
        .filter(|l| !l.starts_with('#') && !l.starts_with("relative"));
    while let Some(head) = lines.next() {
        let args = hexes(lines.next().unwrap().strip_prefix("args ").unwrap());
        let bend = bend_of(&args);
        if let Some(id) = head.strip_prefix("stiffness ") {
            let mut k = vec![vec![Vec::new(); 12]; 12];
            for _ in 0..144 {
                let f: Vec<&str> = lines.next().unwrap().split_whitespace().collect();
                assert_eq!(f[0], "k");
                let (r, c): (usize, usize) = (f[1].parse().unwrap(), f[2].parse().unwrap());
                k[r][c] = f[3..6].iter().map(|t| hex(t)).collect();
            }
            stiffness.push(Stiffness {
                id: id.to_string(),
                bend,
                k,
            });
        } else {
            let id = head.strip_prefix("row ").unwrap().to_string();
            let u: [f64; 12] = hexes(lines.next().unwrap().strip_prefix("u ").unwrap())
                .try_into()
                .unwrap();
            let span_y: [f64; 3] = hexes(lines.next().unwrap().strip_prefix("py ").unwrap())
                .try_into()
                .unwrap();
            let mut f = Vec::new();
            for k in 0..12 {
                let fields: Vec<&str> = lines.next().unwrap().split_whitespace().collect();
                assert_eq!((fields[0], fields[1].parse::<usize>().unwrap()), ("f", k));
                f.push(fields[2..5].iter().map(|t| hex(t)).collect());
            }
            rows.push(Rows {
                id,
                bend,
                u,
                span_y,
                f,
            });
        }
        assert_eq!(lines.next(), Some("end"));
    }
    (stiffness, rows)
}

fn lead(terms: &[f64]) -> f64 {
    terms.iter().sum::<f64>()
}

/// Exactly: Σx − Σy, rounded once.
fn difference(x: &[f64], y: &[f64]) -> f64 {
    let mut acc = ExactAccumulator::new();
    for &v in x {
        acc.add(v).unwrap();
    }
    for &v in y {
        acc.add(-v).unwrap();
    }
    acc.round().unwrap()
}

/// Exactly: |Σterms| + e ≤ bound.
fn magnitude_within(terms: &[f64], e: f64, bound: f64) -> bool {
    within(terms, &[], e, bound)
}

fn certify(rows: &Rows) -> CertifiedArcRecovery {
    let bend = &rows.bend;
    certify_curved_recovery(bend, bend.coordinates_i, bend.coordinates_j, rows.span_y)
        .unwrap_or_else(|e| panic!("{}: {e:?}", rows.id))
}

/// The group's published magnitude q at one end, as the guard forms it.
fn q_of(values: &[f64], end: usize, group: ArcRowGroup) -> f64 {
    let rows = group.rows();
    if rows.len() == 1 {
        values[6 * end + rows[0]].abs()
    } else {
        values[6 * end + rows[0]].hypot(values[6 * end + rows[1]])
    }
}

/// C1: the certified K_g encloses the reference's K_jj block, and the
/// magnitude bound of every one of the 144 entries of K(z) is an upper bound
/// that is tight to 1e-12 of the matrix's largest entry (a looser product such
/// as |H||K_g||Hᵀ| fails the tightness clause on the near-π and skew arcs).
#[test]
fn c1_stiffness_and_magnitudes_enclose_the_reference() {
    let e = relative();
    let (stiffness, _) = references();
    assert_eq!(stiffness.len(), 12);
    for case in &stiffness {
        let cert = certify_curved_recovery(
            &case.bend,
            case.bend.coordinates_i,
            case.bend.coordinates_j,
            case.bend.y_reference,
        )
        .unwrap_or_else(|f| panic!("{}: {f:?}", case.id));
        for r in 0..6 {
            for c in 0..6 {
                let reference = &case.k[6 + r][6 + c];
                let ball = &cert.stiffness[r][c];
                let mid = split_terms(&ball.m);
                assert!(
                    within(reference, &mid, e * lead(reference).abs(), ball.r),
                    "{}: K_g[{r}][{c}] not enclosed",
                    case.id
                );
            }
        }
        let scale = (0..144)
            .map(|n| lead(&case.k[n / 12][n % 12]).abs())
            .fold(0.0, f64::max);
        for r in 0..12 {
            for c in 0..12 {
                let reference = &case.k[r][c];
                let bound = cert.magnitude[r][c];
                assert!(
                    magnitude_within(reference, e * lead(reference).abs(), bound),
                    "{}: |K[{r}][{c}]| above its bound",
                    case.id
                );
                assert!(
                    bound - lead(reference).abs() <= 1e-12 * scale,
                    "{}: |K[{r}][{c}]| bound loose: {bound:e} against {:e}",
                    case.id,
                    lead(reference).abs()
                );
            }
        }
    }
}

/// C2–C4: for the published value v = RN(f(z, u)) of every frozen row, the
/// bound encloses (B_f ≥ |v − f| + e) and is tight: the radius share
/// B_f − |v − f| stays below 1e-3 of RD(1e-9)·q of the row's group (L7's
/// uniform radius gave 6 to 12 times the criterion on the 30 m chord at
/// 2e-9 rad), and B_f > 0.
#[test]
fn c2_row_bounds_enclose_and_stay_tight() {
    let e = relative();
    let (_, rows) = references();
    assert_eq!(rows.len(), 24);
    let mut worst = 0.0_f64;
    for case in &rows {
        let cert = certify(case);
        let published: [f64; 12] = std::array::from_fn(|k| lead(&case.f[k]));
        let bounds = cert
            .bound_end_actions(&case.u, &[], &[], &published)
            .unwrap();
        for end in 0..2 {
            for group in ArcRowGroup::ALL {
                let q = q_of(&published, end, group);
                for &row in group.rows() {
                    let k = 6 * end + row;
                    let actual = difference(&[published[k]], &case.f[k]).abs();
                    assert!(bounds.formation[k] > 0.0, "{}: row {k}", case.id);
                    assert!(
                        within(
                            &case.f[k],
                            &[published[k]],
                            e * published[k].abs(),
                            bounds.formation[k]
                        ),
                        "{}: row {k} not enclosed",
                        case.id
                    );
                    if q > 0.0 {
                        let share = (bounds.formation[k] - actual) / (CRITERION * q);
                        worst = worst.max(share);
                        assert!(share <= 1e-3, "{}: row {k} radius share {share:e}", case.id);
                    }
                }
            }
        }
    }
    println!("C2: worst radius share {worst:.3e} of RD(1e-9) q");
}

/// C5: B_u is Σ_a|A_s|Σ_c|K(z)|·2⁻⁵³|u_c| (plus 2⁻¹⁰⁷⁴ per column),
/// recomputed here in binary64 from the reference K(z) and the chord frame.
#[test]
fn c5_resolution_bound_is_its_formula() {
    let (stiffness, rows) = references();
    for case in &rows {
        let reference = stiffness
            .iter()
            .find(|s| s.id == case.id.split('|').next().unwrap())
            .unwrap();
        let cert = certify(case);
        let published: [f64; 12] = std::array::from_fn(|k| lead(&case.f[k]));
        let bounds = cert
            .bound_end_actions(&case.u, &[], &[], &published)
            .unwrap();
        let (xi, xj) = (case.bend.coordinates_i, case.bend.coordinates_j);
        let d: Vec<f64> = (0..3).map(|k| xj[k] - xi[k]).collect();
        let n = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt();
        let x: Vec<f64> = d.iter().map(|v| v / n).collect();
        let y0 = case.span_y;
        let p = y0[0] * x[0] + y0[1] * x[1] + y0[2] * x[2];
        let yc: Vec<f64> = (0..3).map(|k| y0[k] - p * x[k]).collect();
        let m = (yc[0] * yc[0] + yc[1] * yc[1] + yc[2] * yc[2]).sqrt();
        let y: Vec<f64> = yc.iter().map(|v| v / m).collect();
        let z = [
            x[1] * y[2] - x[2] * y[1],
            x[2] * y[0] - x[0] * y[2],
            x[0] * y[1] - x[1] * y[0],
        ];
        let frame = [x.clone(), y.clone(), z.to_vec()];
        for block in 0..4 {
            for a in 0..3 {
                let mut expected = 0.0;
                for c in 0..3 {
                    let row = 3 * block + c;
                    let inner: f64 = (0..12)
                        .map(|col| {
                            lead(&reference.k[row][col]).abs()
                                * (2f64.powi(-53) * case.u[col].abs() + f64::from_bits(1))
                        })
                        .sum();
                    expected += frame[a][c].abs() * inner;
                }
                let got = bounds.resolution[3 * block + a];
                assert!(
                    got >= expected * (1.0 - 1e-12) && got <= expected * (1.0 + 1e-9) + 1e-300,
                    "{}: B_u row {} = {got:e}, formula {expected:e}",
                    case.id,
                    3 * block + a
                );
            }
        }
    }
}

/// The free expansion enters the ball: with ũ = u/2 (exact), the bounded
/// value is f(z, u − ũ) = f(z, u)/2. Omitting ũ from the ball makes B_f of
/// order |f|/2 and fails the tightness clause.
#[test]
fn free_expansion_is_subtracted_in_the_ball() {
    let e = relative();
    let (_, rows) = references();
    for case in rows.iter().filter(|r| r.id.ends_with("|1.0")) {
        let cert = certify(case);
        let free: [f64; 12] = std::array::from_fn(|k| case.u[k] * 0.5);
        let halves: Vec<Vec<f64>> = case
            .f
            .iter()
            .map(|t| t.iter().map(|v| v * 0.5).collect())
            .collect();
        let published: [f64; 12] = std::array::from_fn(|k| lead(&halves[k]));
        let bounds = cert
            .bound_end_actions(&case.u, &[free], &[], &published)
            .unwrap();
        for end in 0..2 {
            for group in ArcRowGroup::ALL {
                let q = q_of(&published, end, group);
                for &row in group.rows() {
                    let k = 6 * end + row;
                    assert!(
                        within(
                            &halves[k],
                            &[published[k]],
                            e * published[k].abs(),
                            bounds.formation[k]
                        ),
                        "{}: row {k}",
                        case.id
                    );
                    let actual = difference(&[published[k]], &halves[k]).abs();
                    if q > 0.0 {
                        assert!(
                            bounds.formation[k] - actual <= 1e-3 * CRITERION * q,
                            "{}: row {k} loose with the free expansion",
                            case.id
                        );
                    }
                }
            }
        }
    }
}

/// R6: a certified uniform load is subtracted in the ball. RV129's
/// references put the chord on the global x axis, so with y_s = (0, 1, 0) the
/// chord frame is the identity and, at u = 0, f = −p(z, w) exactly; omitting
/// the load makes B_f about |p| and fails the tightness clause.
#[test]
fn certified_load_is_subtracted_in_the_ball() {
    let mut checked = 0;
    for case in reference_cases() {
        let (bend, intensity) = formation(&case.args);
        let on_x = bend.coordinates_i[1] == 0.0
            && bend.coordinates_i[2] == 0.0
            && bend.coordinates_j[1] == 0.0
            && bend.coordinates_j[2] == 0.0
            && bend.coordinates_j[0] > bend.coordinates_i[0];
        if !on_x {
            continue;
        }
        let Ok(load) = certify_curved_uniform_load(&bend, intensity) else {
            continue;
        };
        let Ok(cert) = certify_curved_recovery(
            &bend,
            bend.coordinates_i,
            bend.coordinates_j,
            [0.0, 1.0, 0.0],
        ) else {
            continue;
        };
        let published: [f64; 12] = std::array::from_fn(|k| -lead(&case.refs[k].0));
        let bounds = cert
            .bound_end_actions(&[0.0; 12], &[], &[load.clone()], &published)
            .unwrap();
        let scale = published.iter().fold(0.0_f64, |a, v| a.max(v.abs()));
        for k in 0..12 {
            let (reference, e) = &case.refs[k];
            let negated: Vec<f64> = reference.iter().map(|v| -v).collect();
            assert!(
                within(&negated, &[published[k]], *e, bounds.formation[k]),
                "{}: row {k} not enclosed",
                case.label
            );
            let actual = difference(&[published[k]], &negated).abs();
            let slack = bounds.formation[k] - actual;
            assert!(
                slack <= 2.0 * load.terms[k].radius + 1e-12 * CRITERION * scale,
                "{}: row {k} slack {slack:e}",
                case.label
            );
        }
        checked += 1;
    }
    assert!(checked >= 10, "{checked}");
}

/// C6: a failed precondition is `Err` (the guard's unavailable bound):
/// ρ̂′ ≥ 1/2 at k = 1e37 (class 5, as the load certificate), an inadmissible
/// arc (class 3), a span y reference parallel to the chord (class 3) and a
/// non-finite published value (L0).
#[test]
fn c6_failed_preconditions_are_unavailable() {
    let y = [0.0, 1.0, 0.0];
    let body = t15_body(1.0);
    let (i, j) = (body.coordinates_i, body.coordinates_j);
    assert_eq!(
        class_of(&certify_curved_recovery(&t15_body(1e37), i, j, y).unwrap_err()),
        5
    );
    let mut bend = t15_body(1.0);
    bend.radius = 0.5;
    assert_eq!(
        class_of(&certify_curved_recovery(&bend, i, j, y).unwrap_err()),
        3
    );
    assert_eq!(
        certify_curved_recovery(&body, i, j, [1.0, 0.0, 0.0]).unwrap_err(),
        ArcCertificateFailure::SqrtArgumentNotPositive {
            site: "span y reference"
        }
    );
    let cert = certify_curved_recovery(&body, i, j, y).unwrap();
    let mut published = [0.0; 12];
    published[4] = f64::NAN;
    assert!(cert
        .bound_end_actions(&[0.0; 12], &[], &[], &published)
        .is_err());
}

/// L7′ against L7 on the binding long chord (30 m at 2e-9 rad): no entry's
/// equilibrated radius exceeds L7's uniform radius, and some are at least
/// 1e6 times smaller (C2's radius share, 2.5e-5 of the criterion, is the
/// operative consequence; L7 gave 6 to 12 times the criterion there).
#[test]
fn l7_prime_radius_follows_each_entry_scale() {
    let (stiffness, _) = references();
    let case = stiffness
        .iter()
        .find(|s| s.id == "I9 2e-9 rad, 30 m chord")
        .unwrap();
    let mut b = Balls::new().unwrap();
    let geometry = arc_geometry(&mut b, &case.bend).unwrap();
    let flexibility = arc_flexibility(&mut b, &geometry, &case.bend).unwrap();
    let (equilibrated, _) = invert_equilibrated(&mut b, &flexibility.f).unwrap();
    let (uniform, _) = invert_verified(&mut b, &flexibility.f).unwrap();
    let mut best = f64::INFINITY;
    for r in 0..6 {
        for c in 0..6 {
            let ratio = equilibrated[r][c].r / uniform[r][c].r;
            assert!(ratio <= 1.0 + 1e-3, "K_t[{r}][{c}]: {ratio:e}");
            best = best.min(ratio);
        }
    }
    assert!(best <= 1e-6, "{best:e}");
}
