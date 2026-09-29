//! K4 tests of D1 revision 5a.3 on the models, checkpoint A3a (mounted in
//! `adaptive.rs`, so they reach the shared build): E-UNIT (g, Ā, E_q,
//! E(body, kind), ê and Φ at every precision), E-UC on the models (blocks,
//! est_c, Uc_c, the shift and B_c against the exact ‖K̃_c⁻¹‖₁ at every
//! verification precision), the pin of `factor()`'s L and D bits (ROOT's
//! A3-0 ruling Q4), the loop parity of the shifted factorization (Q5), and 7a's
//! data flags. The oracle is the generator's emulation (ROOT's ruling Q1):
//! `scale.txt`, `bounds.txt`, `models5a3.txt`.
use super::super::assemble::{assemble_bounded, BoundedCoefficients};
use super::super::bound::{
    block_of_rows, certified, certify, data_blocks, gamma_m, scaled_profile, shift_needed,
    shift_schedule, shifted_factor, sigma_from_estimate, uc_bounds, ProfileLdl,
};
use super::super::verify::{e_hat, formation_scale, phi_512, resolution_scale, StageRounding};
use super::*;

#[allow(dead_code)]
#[path = "models.rs"]
mod models;
#[allow(dead_code)]
#[path = "support.rs"]
mod support;
use support::{sha256_hex, tok};

const SCALE: &str = include_str!("scale.txt");
const BOUNDS: &str = include_str!("bounds.txt");
const MODELS_5A3: &str = include_str!("models5a3.txt");

fn all_models() -> Vec<models::Model> {
    let mut out = models::models();
    out.extend(models::parse_models(MODELS_5A3));
    out
}

fn digest(lines: &[String]) -> String {
    let mut text = lines.join("\n");
    text.push('\n');
    sha256_hex(text.as_bytes())
}

fn stop_name(stop: &AttemptStop) -> &'static str {
    match stop {
        AttemptStop::Pivot { .. } | AttemptStop::NegativeEnergy { .. } => "Pivot",
        AttemptStop::Condition => "Condition",
        AttemptStop::ResidualGate { .. } => "ResidualGate",
        AttemptStop::ZeroDiagonal { .. } => "ZeroDiagonal",
        other => panic!("unexpected stop {other:?}"),
    }
}

fn opt_tok<const L: usize>(v: &Option<Wide<L>>) -> String
where
    Wide<L>: SupportedWidth,
{
    v.as_ref().map(tok).unwrap_or_else(|| "-".to_string())
}

/// The records of one model at one precision (lines whose second field is
/// the model's name and third the precision).
fn records<'a>(text: &'a str, name: &str, p: u32) -> Vec<Vec<&'a str>> {
    let p = p.to_string();
    text.lines()
        .map(|l| l.split_whitespace().collect::<Vec<_>>())
        .filter(|f| f.len() > 2 && f[1] == name && f[2] == p)
        .collect()
}

// ---------------------------------------------------------------- E-UNIT

fn scale_at<const L: usize, const R: usize>(
    p: u32,
    q: u32,
    prep: &CasePrep,
    group: &GroupPrep,
    recs: &[Vec<&str>],
) where
    Wide<L>: SupportedWidth,
    Wide<R>: SupportedWidth,
{
    let name = recs[0][1];
    let guard = StageGuard::unlimited();
    let shared = build_shared::<L, R>(p, q, &prep.source, group, guard).result;
    let solved = shared
        .as_ref()
        .map_err(|s| s.clone())
        .and_then(|sh| solve_case_at::<L, R>(sh, prep, group, guard).result);
    if recs[0][3] == "stop" {
        let stop = solved.expect_err(name);
        assert_eq!(stop_name(&stop), recs[0][4], "{name} {p}");
        return;
    }
    let shared = shared.unwrap();
    let solved = solved.unwrap_or_else(|e| panic!("{name} {p}: {e:?}"));
    assert_eq!(
        solved.corrections.to_string(),
        recs[0][4],
        "{name} {p} corrections"
    );
    let mut ctx = WideContext::<L>::new(p).unwrap();
    let mut sum = ExactWideSum::new();
    let source = &prep.source;
    let bounded: Vec<BoundedCoefficients<L>> = shared.members.iter().map(|m| m.bounded()).collect();
    let abar = assemble_bounded(
        &mut ctx,
        &mut sum,
        &guard,
        source,
        &group.structure,
        &bounded,
        &shared.directional,
    )
    .unwrap();
    let w: Vec<Wide<L>> = solved.u.iter().map(|v| v.abs()).collect();
    let e = formation_scale(
        &mut ctx,
        &mut sum,
        &guard,
        source,
        &prep.layout,
        &group.structure,
        &abar,
        &shared.members,
        &shared.directional,
        Some(&prep.ledger),
        &w,
        StageRounding::Nearest,
    )
    .unwrap();
    let erows: Vec<String> = e
        .iter()
        .enumerate()
        .filter_map(|(i, v)| v.as_ref().map(|v| format!("{i} {}", tok(v))))
        .collect();
    let resolution = resolution_scale(&prep.layout, &e, source.body_count() as usize).unwrap();
    let mut explicit = Vec::new();
    for f in &recs[1..] {
        match f[0] {
            "g" => {
                let got: Vec<String> = shared.members.iter().map(|m| m.g_exp.to_string()).collect();
                let got = if got.is_empty() {
                    "-".to_string()
                } else {
                    got.join(",")
                };
                assert_eq!(got, f[3], "{name} {p} g");
            }
            "abar" => {
                let mut entries: Vec<(usize, usize, usize)> = group.structure.entries().collect();
                entries.sort_unstable();
                let lines: Vec<String> = entries
                    .iter()
                    .map(|&(r, c, index)| format!("{r} {c} {}", tok(&abar[index])))
                    .collect();
                assert_eq!(digest(&lines), f[3], "{name} {p} Ā");
            }
            "erows" => assert_eq!(digest(&erows), f[3], "{name} {p} E rows"),
            "erow" => explicit.push(format!("{} {}", f[3], f[4])),
            "ebody" => {
                let b: usize = f[3].parse().unwrap();
                let eh = e_hat(resolution[b], prep.extents[b]);
                let values = [
                    resolution[b][0],
                    resolution[b][1],
                    eh[0],
                    eh[1],
                    phi_512(eh[0]),
                    phi_512(eh[1]),
                ];
                let got: Vec<String> = values
                    .iter()
                    .map(|v| format!("{:016x}", v.to_bits()))
                    .collect();
                assert_eq!(got, f[4..10].to_vec(), "{name} {p} body {b}");
            }
            other => panic!("{other}"),
        }
    }
    if !explicit.is_empty() {
        assert_eq!(explicit, erows, "{name} {p} explicit E rows");
    }
}

#[test]
fn e_unit_g_the_bounded_operator_and_e_equal_the_generators_emulation_at_every_precision() {
    let mut checked = 0;
    for m in all_models() {
        let prep = CasePrep::new(m.source()).unwrap();
        let group = prepare_group(&prep.source);
        for p in [128u32, 256, 512, 1024] {
            let recs = records(SCALE, &m.name, p);
            let recs: Vec<Vec<&str>> = recs.into_iter().filter(|f| f[0] != "bnd").collect();
            assert!(!recs.is_empty() && recs[0][0] == "scale", "{} {p}", m.name);
            let Ok(group) = &group else {
                // Refused before any attempt (a witnessed mechanism).
                assert_eq!(recs[0][3], "stop", "{} {p}", m.name);
                continue;
            };
            match p {
                128 => scale_at::<4, 4>(p, 192, &prep, group, &recs),
                256 => scale_at::<4, 8>(p, 320, &prep, group, &recs),
                512 => scale_at::<8, 16>(p, 576, &prep, group, &recs),
                _ => scale_at::<16, 16>(p, 1024, &prep, group, &recs),
            }
            checked += 1;
        }
    }
    assert!(checked >= 240, "{checked}");
}

// ---------------------------------------------------------------- E-UC

#[allow(clippy::too_many_lines)]
fn bounds_at<const L: usize, const R: usize>(
    p: u32,
    q: u32,
    prep: &CasePrep,
    group: &GroupPrep,
    recs: &[Vec<&str>],
) -> (usize, usize)
where
    Wide<L>: SupportedWidth,
    Wide<R>: SupportedWidth,
{
    let name = recs[0][1];
    let guard = StageGuard::unlimited();
    let shared = build_shared::<L, R>(p, q, &prep.source, group, guard).result;
    if recs[0][3] == "stop" {
        let stop = shared.expect_err(name);
        assert_eq!(stop_name(&stop), recs[0][4], "{name} {p}");
        return (0, 0);
    }
    let shared = shared.unwrap_or_else(|e| panic!("{name} {p}: {e:?}"));
    let solved = solve_case_at::<L, R>(&shared, prep, group, guard)
        .result
        .ok();
    let mut ctx = WideContext::<L>::new(p).unwrap();
    let mut sum = ExactWideSum::new();
    let ordering = &group.ordering;
    let blocks = &group.blocks;
    let factor = &shared.factor;
    let nf = ordering.free.len();
    assert_eq!(
        [nf.to_string(), blocks.len().to_string()],
        [recs[0][3].to_string(), recs[0][4].to_string()],
        "{name} {p}"
    );
    let gamma = gamma_m(&mut ctx, &mut sum, nf).unwrap();
    assert_eq!(tok(&gamma), recs[0][5], "{name} {p} γ_m");
    let profile = scaled_profile(
        &group.structure,
        &shared.k,
        ordering,
        blocks,
        factor.scale(),
    )
    .unwrap();
    let rows = block_of_rows(ordering, blocks);
    let bounds = uc_bounds(
        &mut ctx,
        &mut sum,
        &guard,
        factor,
        &rows,
        blocks.len(),
        &gamma,
    )
    .unwrap();
    let est = &shared.est_blocks;
    let data = solved.as_ref().map(|s| {
        let presc: Vec<bool> = (0..prep.source.dof_count())
            .map(|g| prep.source.constraint(g).is_some_and(|v| v != 0.0))
            .collect();
        data_blocks(
            blocks,
            ordering,
            &group.structure,
            &prep.ledger,
            &presc,
            &s.u,
        )
    });
    // The forced shift: every block with est_c > 0 (uc_check7's form).
    let start: Vec<(usize, Wide<L>, usize)> = (0..blocks.len())
        .filter(|&b| !est[b].is_zero())
        .map(|b| {
            (
                b,
                sigma_from_estimate(&mut ctx, &mut sum, &est[b]).unwrap(),
                blocks.positions[b].len(),
            )
        })
        .collect();
    let (shifts, count) =
        shift_schedule(&mut ctx, &mut sum, &guard, &profile, &gamma, &start).unwrap();
    let mut norms = Vec::new();
    let (mut exact_checked, mut shifted) = (0, 0);
    for f in &recs[1..] {
        match f[0] {
            "blk" => {
                let b: usize = f[3].parse().unwrap();
                let bb = &bounds[b];
                let need = if est[b].is_zero() {
                    "-".to_string()
                } else {
                    u8::from(
                        shift_needed(&mut sum, &bb.uc, &est[b], blocks.positions[b].len()).unwrap(),
                    )
                    .to_string()
                };
                let flag = data
                    .as_ref()
                    .map_or("-".to_string(), |d| u8::from(d[b]).to_string());
                let got = [
                    blocks.positions[b].len().to_string(),
                    flag,
                    tok(&est[b]),
                    tok(&bb.u),
                    tok(&bb.n_l),
                    tok(&bb.t),
                    opt_tok(&bb.uc),
                    need,
                ];
                assert_eq!(got.to_vec(), f[4..12].to_vec(), "{name} {p} block {b}");
                norms.push((b, f[12], f[13], f[14]));
                if let Some(uc) = &bb.uc {
                    assert!(
                        support::wide_at_least(uc, f[13], f[14]),
                        "{name} {p} block {b}: Uc below the norm"
                    );
                }
                exact_checked += usize::from(f[12] == "exact");
            }
            "shf" => {
                let b: usize = f[3].parse().unwrap();
                let r = &shifts.iter().find(|s| s.0 == b).unwrap().1;
                let got = [
                    tok(&r.sigma),
                    r.tries.to_string(),
                    opt_tok(&r.n_l),
                    opt_tok(&r.delta),
                    opt_tok(&r.sigma_prime),
                    opt_tok(&r.s),
                ];
                assert_eq!(got.to_vec(), f[4..10].to_vec(), "{name} {p} shift {b}");
                let norm = norms.iter().find(|n| n.0 == b).unwrap();
                if let Some(s) = &r.s {
                    assert!(
                        support::wide_at_least(s, norm.2, norm.3),
                        "{name} {p} block {b}: S below the norm"
                    );
                    shifted += 1;
                }
                if let Some(bc) = certified(&bounds[b].uc, &r.s) {
                    assert!(
                        support::wide_at_least(&bc, norm.2, norm.3),
                        "{name} {p} block {b}: B below the norm"
                    );
                }
            }
            "shiftcount" => assert_eq!(count.to_string(), f[3], "{name} {p} factorizations"),
            other => panic!("{other}"),
        }
    }
    // certify(): the shift only where 7c needs it, on the blocks with data;
    // each block's outcome equals the forced run's (7a: blocks do not interact).
    if let Some(data) = &data {
        let (certs, _) = certify(
            &mut ctx,
            &mut sum,
            &guard,
            &group.structure,
            &shared.k,
            ordering,
            factor,
            blocks,
            &bounds,
            est,
            data,
            &gamma,
        )
        .unwrap();
        for (b, c) in certs.iter().enumerate() {
            let forced = shifts.iter().find(|s| s.0 == b).map(|s| &s.1);
            match (&c.shift, data[b]) {
                (Some(r), true) => assert_eq!(Some(r), forced, "{name} {p} block {b}"),
                (Some(_), false) => panic!("{name} {p} block {b}: shifted without data"),
                (None, _) => {}
            }
            let expect = if data[b] {
                certified(&bounds[b].uc, &c.shift.as_ref().and_then(|r| r.s))
            } else {
                None
            };
            assert_eq!(c.b, expect, "{name} {p} block {b}");
        }
    }
    (exact_checked, shifted)
}

#[test]
fn e_uc_uc_s_and_b_equal_the_emulation_and_never_fall_below_the_norm_per_block() {
    let mut models_checked = 0;
    let (mut exact, mut shifted) = (0, 0);
    for m in all_models() {
        let prep = CasePrep::new(m.source()).unwrap();
        let Ok(group) = prepare_group(&prep.source) else {
            continue;
        };
        for p in [256u32, 512, 1024] {
            let recs: Vec<Vec<&str>> = records(BOUNDS, &m.name, p);
            assert!(!recs.is_empty() && recs[0][0] == "bnd", "{} {p}", m.name);
            let (e, s) = match p {
                256 => bounds_at::<4, 8>(p, 320, &prep, &group, &recs),
                512 => bounds_at::<8, 16>(p, 576, &prep, &group, &recs),
                _ => bounds_at::<16, 16>(p, 1024, &prep, &group, &recs),
            };
            exact += e;
            shifted += s;
        }
        models_checked += 1;
    }
    assert!(
        models_checked >= 60 && exact >= 150 && shifted >= 30,
        "{models_checked} {exact} {shifted}"
    );
}

// ---------------------------------------------------------------- Q4 and Q5: the factor loop

/// factor()'s L and D bits against the emulation of the loop as read (Q4), and
/// the shifted loop with no shift against factor() (Q5), at one precision.
fn loop_at<const L: usize, const R: usize>(
    p: u32,
    q: u32,
    prep: &CasePrep,
    group: &GroupPrep,
    rec: &[&str],
) -> bool
where
    Wide<L>: SupportedWidth,
    Wide<R>: SupportedWidth,
{
    let name = rec[1];
    let guard = StageGuard::unlimited();
    let shared = build_shared::<L, R>(p, q, &prep.source, group, guard).result;
    if rec[3] == "stop" {
        assert!(shared.is_err(), "{name} {p}");
        return false;
    }
    let shared = shared.unwrap();
    let factor = &shared.factor;
    let nf = group.ordering.free.len();
    let lines: Vec<String> = (0..nf)
        .flat_map(|i| (factor.first_of(i)..=i).map(move |j| (i, j)))
        .map(|(i, j)| format!("{i} {j} {}", tok(&factor.get(i, j))))
        .collect();
    assert_eq!(digest(&lines), rec[6], "{name} {p}: factor()'s L and D");
    let mut ctx = WideContext::<L>::new(p).unwrap();
    let sum = ExactWideSum::new();
    let blocks = &group.blocks;
    let profile = scaled_profile(
        &group.structure,
        &shared.k,
        &group.ordering,
        blocks,
        factor.scale(),
    )
    .unwrap();
    let unshifted =
        shifted_factor(&mut ctx, &sum, &guard, &profile, &vec![None; blocks.len()]).unwrap();
    assert!(unshifted.failed.iter().all(|f| !f), "{name} {p}");
    for i in 0..nf {
        for j in factor.first_of(i)..=i {
            assert_eq!(
                tok(&unshifted.entry(i, j)),
                tok(&factor.get(i, j)),
                "{name} {p} parity ({i}, {j})"
            );
        }
    }
    true
}

#[test]
fn factors_l_and_d_bits_are_pinned_and_the_shifted_loop_without_a_shift_reproduces_them() {
    let mut checked = 0;
    for m in all_models() {
        let prep = CasePrep::new(m.source()).unwrap();
        let Ok(group) = prepare_group(&prep.source) else {
            continue;
        };
        for p in [256u32, 512, 1024] {
            let recs = records(BOUNDS, &m.name, p);
            let rec = &recs[0];
            assert_eq!(rec[0], "bnd");
            let done = match p {
                256 => loop_at::<4, 8>(p, 320, &prep, &group, rec),
                512 => loop_at::<8, 16>(p, 576, &prep, &group, rec),
                _ => loop_at::<16, 16>(p, 1024, &prep, &group, rec),
            };
            checked += usize::from(done);
        }
    }
    assert!(checked >= 180, "{checked}");
}

// ---------------------------------------------------------------- blocks and data

#[test]
fn blocks_are_the_free_free_components_and_data_follows_7a() {
    let blocks_of = |name: &str| {
        let m = all_models().into_iter().find(|m| m.name == name).unwrap();
        let prep = CasePrep::new(m.source()).unwrap();
        let group = prepare_group(&prep.source).unwrap();
        (prep, group)
    };
    // THETA-STUB: node 0's restraint cuts the stub's uz(2) off as its own block.
    let (_, g) = blocks_of("THETA-STUB");
    assert_eq!(g.blocks.len(), 2);
    assert_eq!(g.blocks.positions[1].len(), 1);
    // THETA-STUB-COUPLED: uz(2) joins the loaded block through node 3's rotations.
    let (_, g) = blocks_of("THETA-STUB-COUPLED");
    assert_eq!(g.blocks.len(), 1);
    // HH-FOOL: body H's four DOFs form one block, and body V's chain another.
    let (_, g) = blocks_of("HH-FOOL-m40");
    assert_eq!(g.blocks.len(), 2);
    assert_eq!(g.blocks.positions.iter().map(Vec::len).min(), Some(4));
    // BLOCK-PRESC: the unloaded block's only data is its pattern coupling to
    // the nonzero prescription ux(2) = 1e-3 (its state is exactly zero).
    let (prep, g) = blocks_of("BLOCK-PRESC");
    assert_eq!(g.blocks.len(), 2);
    let u = vec![Wide::<4>::ZERO; prep.source.dof_count()];
    let presc: Vec<bool> = (0..prep.source.dof_count())
        .map(|d| prep.source.constraint(d).is_some_and(|v| v != 0.0))
        .collect();
    let with = data_blocks(
        &g.blocks,
        &g.ordering,
        &g.structure,
        &prep.ledger,
        &presc,
        &u,
    );
    let without = data_blocks(
        &g.blocks,
        &g.ordering,
        &g.structure,
        &prep.ledger,
        &vec![false; presc.len()],
        &u,
    );
    // Block 0 carries the load; block 1 has data only through the coupling.
    assert_eq!(with, vec![true, true]);
    assert_eq!(without, vec![true, false]);
}
