//! O4 (DEL-04-08 Design `o4-source-extension.md` §4.3, §4.5, §6): the product
//! certificate's Timoshenko lanes and the reduced stress sections, through the
//! whole prepared proof (begin → project → complete_maxima → certify_final).
//! Invented inputs only; expectations are closed forms or exact decisions.
use super::*;
use super::super::super::origins::{OriginCapacity, RecordedCase, RecordedInvocation};
use super::super::super::source::{
    Component, Constraint, Dof, MemberSourceRef, NodalLoad, PrimitiveSource, SourceParts, Station,
    StraightMember, SupportGroup, TimoshenkoLeaf, TimoshenkoLeafSet,
};
use crate::structural::retained_api::CaseLimit;

const D: f64 = 0.1;
const T: f64 = 0.005;
const E: f64 = 210e9;
const NU: f64 = 0.3125; // G = E/(2(1 + ν)) = 80e9 exactly
const LOADS: [f64; 6] = [26000.0, 326.0, -278.0, 63.0, 33.5, -56.0];

fn section(t: f64) -> [f64; 5] {
    prepare_product_annulus(D, t).result().unwrap().section_bits().values()
}
/// FK's binary64 κ̂ of the nominal annulus and the prepared leaf
/// A_s,prep = RN(κ̂·A_prep) (§4.2).
fn kappa_hat(t: f64, nu: f64) -> f64 {
    let ro = D / 2.0;
    crate::shear::annulus_energy_shear_factor(ro - t, ro, nu).unwrap()
}
fn prepared_leaf(member: u32, nu: f64) -> TimoshenkoLeaf {
    TimoshenkoLeaf::prepared(member, kappa_hat(T, nu), section(T)[0]).unwrap()
}
/// A cantilever along `chord`, root restrained (support 3), tip loaded in all
/// six components; stations at 0.25, 0.5 and 0.75; the prepared annulus.
fn cantilever(g: f64, chord: [f64; 3], y: [f64; 3]) -> PrimitiveSource {
    let [a, i, j, _, _] = section(T);
    PrimitiveSource::new(SourceParts {
        nodes: vec![[0.0; 3], chord],
        members: vec![StraightMember { id: 7, node_i: 0, node_j: 1, elastic_modulus: E, shear_modulus: g,
            area: a, second_moment_y: i, second_moment_z: i, torsion_constant: j, y_reference: y }],
        constraints: Component::ALL.iter().map(|&component| Constraint { dof: Dof { node: 0, component }, value: 0.0 }).collect(),
        loads: Component::ALL.iter().zip(LOADS).map(|(&component, value)| NodalLoad { dof: Dof { node: 1, component },
            value, source_id: format!("tip-{component:?}") }).collect(),
        stations: [0.25, 0.5, 0.75].iter().enumerate()
            .map(|(k, &fraction)| Station { id: 17 + k as u32, member: 7, fraction }).collect(),
        supports: vec![SupportGroup { id: 3, node: 0, restrained: [true; 6], springs: vec![], directional_springs: vec![] }],
        ..SourceParts::default()
    })
    .unwrap()
}
fn facts(material: ProductMaterial) -> Vec<ProductMemberFacts> {
    let [a, i, j, z, _] = section(T);
    vec![ProductMemberFacts { member: 7, diameter: D, effective_wall: T, material, area: a, second_moment: i,
        torsion_constant: j, section_modulus: z, radius: D / 2.0 }]
}
fn solve(source: &PrimitiveSource, shear: Option<&TimoshenkoLeafSet>) -> (RecordedInvocation, RecordedCase) {
    let mut invocation = RecordedInvocation::new(u64::MAX, OriginCapacity::for_calls(&[1], &[]).unwrap()).unwrap();
    let refs = [MemberSourceRef::nodal(source)];
    let mut cases = match shear {
        Some(set) => invocation.solve_timoshenko_cases(&refs, set, CaseLimit::new(u64::MAX)).unwrap(),
        None => invocation.solve_member_cases(&refs, CaseLimit::new(u64::MAX)).unwrap(),
    };
    let case = cases.remove(0);
    (invocation, case)
}
fn owner(case: &RecordedCase) -> &adaptive::RetainedSolve {
    match &case.outcome {
        adaptive::ExecutionOutcome::Selected(v) => v,
        other => panic!("specimen did not select: {other:?}"),
    }
}
type Row = (String, ProductUnit, u32, ProductRecipe);
/// The complete case row set (B3-K's), plus, for `reduced`, the member's 21
/// reduced stress and maximum rows after the nominal ones.
fn rows(owner: &adaptive::RetainedSolve, reduced: bool) -> Vec<Row> {
    let source = owner.source();
    let mut out = Vec::new();
    for r in &owner.publish().rows {
        if matches!(r.id, QuantityId::Reaction(_)) {
            continue;
        }
        let unit = match r.kind {
            Kind::Translation => ProductUnit::Millimetre,
            Kind::Rotation => ProductUnit::Radian,
            Kind::Force => ProductUnit::Newton,
            Kind::Moment => ProductUnit::NewtonMetre,
        };
        out.push((format!("{:?}", r.id), unit, r.body, ProductRecipe::Native(r.id)));
    }
    for g in source.supports() {
        for component in Component::ALL {
            let unit = if component.index() < 3 { ProductUnit::Newton } else { ProductUnit::NewtonMetre };
            out.push((format!("support-{}-{component:?}", g.id), unit, source.body_of_node(g.node),
                ProductRecipe::SupportComponent { support: g.id, component }));
        }
    }
    for basis in if reduced { &["nominal", "reduced"][..] } else { &["nominal"][..] } {
        for m in source.members() {
            let body = source.body_of_node(m.node_i);
            let mut sites = vec![ProductSite::End(End::I), ProductSite::End(End::J)];
            sites.extend(source.stations().iter().filter(|s| s.member == m.id).map(|s| ProductSite::Station(s.id)));
            for site in sites {
                for stress in [ProductStress::Axial, ProductStress::BendingY, ProductStress::BendingZ, ProductStress::Torsion] {
                    out.push((format!("stress-{basis}-{}-{site:?}-{stress:?}", m.id), ProductUnit::Megapascal, body,
                        ProductRecipe::Stress { member: m.id, site, stress }));
                }
            }
            out.push((format!("maximum-{basis}-{}", m.id), ProductUnit::Pascal, body, ProductRecipe::CircularMaximum { member: m.id }));
        }
    }
    out.push(("mode".into(), ProductUnit::Record, 0, ProductRecipe::NonQuantity));
    out
}
fn specs<'a>(rows: &'a [Row]) -> Vec<ProductRowSpec<'a>> {
    rows.iter().map(|(id, unit, body, recipe)| match recipe {
        ProductRecipe::NonQuantity => ProductRowSpec::mode(id, "case", *body, 1).unwrap(),
        _ => ProductRowSpec::mechanical(id, "case", *unit, *body, *recipe).unwrap(),
    }).collect()
}
/// The reduced section of member 7 at wall `t_r`, naming the final-row indices
/// of the rows whose id carries "reduced".
fn reduced_set(rows: &[Row], t_r: f64) -> ReducedSectionSet {
    let [a, i, j, z, _] = section(t_r);
    let indices = rows.iter().enumerate().filter(|(_, r)| r.0.contains("-reduced-")).map(|(k, _)| k as u32).collect();
    ReducedSectionSet::new(vec![ReducedSection { member: 7, effective_wall: t_r, area: a, second_moment: i,
        torsion_constant: j, section_modulus: z, rows: indices }]).unwrap()
}
/// The circular maxima the way the kernel projects any row.
fn maxima(p: &ProjectedProofDraft<'_, '_>) -> Result<Vec<ProductMaximumValue>, ProductFailure> {
    let mut w = ProductCertificateSpent::new(&[]);
    let map = reduced_index(p.data.owner, p.data.facts, p.data.reduced, p.data.specs.len(), &mut w)?;
    let mut out = Vec::new();
    for (i, spec) in p.data.specs.iter().enumerate() {
        let ProductRecipe::CircularMaximum { member } = spec.recipe else { continue };
        let mut w = ProductCertificateSpent::new(&[]);
        w.projection_outcomes = w.prepared_reserve(5, 1)?;
        let rs = reduced_of(p.data.reduced, &map, i);
        let section = section_for_reduced(p.data.owner, p.data.facts, spec.recipe, rs, &mut w)?;
        let k = recipe_reduced(p.data.owner, &mut w, &p.data.k.rows, spec.recipe, section.as_ref(), p.data.facts, rs, true, &p.data.k.curvatures, i)?;
        let s = recipe_reduced(p.data.owner, &mut w, &p.data.source.rows, spec.recipe, section.as_ref(), p.data.facts, rs, false, &p.data.source.curvatures, i)?;
        out.push(ProductMaximumValue::new(member, i, project_hull(hull(k, s), ProductUnit::Pascal, i, &mut w)?)?);
    }
    Ok(out)
}
enum Outcome {
    Certified(CertifiedProductProof, Vec<f64>),
    Refused(ProductProofFailure),
}
fn prove(invocation: &RecordedInvocation, run: usize, owner: &adaptive::RetainedSolve, facts: &[ProductMemberFacts],
    reduced: &ReducedSectionSet, specs: &[ProductRowSpec<'_>]) -> Outcome {
    let draft = match invocation.begin_prepared_product_reduced(run, owner, facts, reduced, specs).into_ready() {
        Ok(v) => v,
        Err(e) => return Outcome::Refused(e),
    };
    let (projected, builder) = match draft.project().into_ready() {
        Ok(v) => v,
        Err(e) => return Outcome::Refused(e),
    };
    let maxima = maxima(&projected).unwrap();
    let (values, work) = builder.complete_maxima(&maxima).into_ready().unwrap();
    let rows: Vec<_> = specs.iter().enumerate().map(|(i, s)| s.row(values.value(i).unwrap())).collect();
    let frozen: Vec<f64> = (0..values.len()).map(|i| *values.value(i).unwrap()).collect();
    match projected.certify_final(&values, &rows, work).into_ready() {
        Ok(v) => Outcome::Certified(v, frozen),
        Err(e) => Outcome::Refused(e),
    }
}
fn certified(outcome: Outcome, what: &str) -> (CertifiedProductProof, Vec<f64>) {
    match outcome {
        Outcome::Certified(proof, values) => {
            assert!(proof.passed(), "{what}");
            let mut copies = TraceCopyWork::default();
            let trace = proof.work().typed_trace(&mut copies);
            for lane in &trace.lanes {
                assert!(lane.as_ref().unwrap().result.is_ok(), "{what}: both lanes");
            }
            (proof, values)
        }
        Outcome::Refused(e) => panic!("{what}: {:?} {:?}", e.failure(),
            e.work().verdicts().iter().filter(|v| !v.passed).collect::<Vec<_>>()),
    }
}
fn refused(outcome: Outcome) -> String {
    match outcome {
        Outcome::Refused(e) => format!("{:?}", e.failure()),
        Outcome::Certified(..) => panic!("certified"),
    }
}

/// §4.2–§4.3: a Timoshenko route-E specimen certifies every row in both lanes
/// (admitted K_s = G·Â_s,prep; annular K_s = g·κ_E·A_exact), and its frozen
/// tip deflection is the Timoshenko one, not the Euler–Bernoulli twin's.
#[test]
fn o4_timoshenko_product_proof_certifies_in_both_lanes() {
    let source = cantilever(80e9, [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]);
    let set = TimoshenkoLeafSet::new(&source, vec![prepared_leaf(7, NU)]).unwrap();
    let (invocation, case) = solve(&source, Some(&set));
    let owner = owner(&case);
    let r = rows(owner, false);
    let s = specs(&r);
    let material = ProductMaterial::BaseENu { e: E, nu: NU };
    let (proof, values) = certified(prove(&invocation, case.run, owner, &facts(material), ReducedSectionSet::empty(), &s), "timoshenko");
    assert_eq!(proof.verdicts().len(), r.len());
    let (eb_invocation, eb_case) = solve(&source, None);
    let eb_owner = self::owner(&eb_case);
    let (_, eb_values) = certified(prove(&eb_invocation, eb_case.run, eb_owner, &facts(material), ReducedSectionSet::empty(), &s), "euler-bernoulli");
    // The tip's v (y) deflection: FL³/(3EI) + FL/(G·A_s) against FL³/(3EI).
    let tip = r.iter().position(|x| x.3 == ProductRecipe::Native(QuantityId::Displacement(Dof { node: 1, component: Component::Uy }))).unwrap();
    let [_, i, _, _, _] = section(T);
    let shear = LOADS[1] * 1.0 / (80e9 * set.leaves()[0].shear_area);
    let bending = LOADS[1] / (3.0 * E * i) + LOADS[5] / (2.0 * E * i);
    assert!(((values[tip] - eb_values[tip]) / 1000.0 - shear).abs() <= 1e-9 * shear.abs(), "{} {} {}", values[tip], eb_values[tip], shear);
    assert!((eb_values[tip] / 1000.0 - bending).abs() <= 1e-9 * bending.abs());
}

/// §4.3: the annular lane's κ_E enclosure contains the exact κ_E of the exact
/// annulus (decided exactly by cross-multiplication) and, on a section where
/// they differ, excludes FK's binary64 κ̂; the admitted K_s is G·Â_s exactly.
#[test]
fn annular_timoshenko_lane_uses_the_exact_kappa() {
    let mut excluded = 0;
    for (d, t, nu) in [(0.168, 0.007, 0.3), (D, T, NU), (0.1143, 0.0086, 0.29), (0.3239, 0.0095, -0.2), (0.05, 0.02, 0.49)] {
        let [a, i, j, z, _] = prepare_product_annulus(d, t).result().unwrap().section_bits().values();
        let ro = d / 2.0;
        let kappa = crate::shear::annulus_energy_shear_factor(ro - t, ro, nu).unwrap();
        let a_s = crate::shear::shear_area(kappa, a).unwrap();
        let g = E / (2.0 * (1.0 + nu));
        let input = MemberOperands { diameter: d, effective_wall: t, material: MaterialOperands::ExactENu { e: E, nu },
            admitted: AdmittedOperands { e: E, g, a, j, iz: i, iy: i, z_hat: z } };
        let mut work = NumericWork::new();
        let mut enc = build_member(&input, &mut work).unwrap();
        shear_rigidity(&input, &mut enc, a_s, &mut work).unwrap();
        let shear = *enc.shear().unwrap();
        let k = shear.kappa();
        // κ_E·Den = Num with r_o = D/2, r_i = r_o − t exact:
        // Num = 6(1+ν)²(r_o² + r_i²)², Den = (8ν²+14ν+7)(r_o² − r_i²)² + 48 r_i² r_o² (1+ν)².
        let mut ctx = WideContext::<16>::new(1024).unwrap();
        let mut sum = ExactWideSum::new();
        let w = |x: f64| Endpoint::from_f64(x).unwrap();
        let exact = |ctx: &mut WideContext<16>, sum: &mut ExactWideSum, terms: &[(Endpoint, Endpoint, bool)]| {
            sum.clear();
            for (x, y, n) in terms {
                sum.add_product(ctx, x, y, *n).unwrap();
            }
            let up = super::super::super::directed::round_toward(ctx, sum, Toward::Up).unwrap();
            sum.clear();
            for (x, y, n) in terms {
                sum.add_product(ctx, x, y, *n).unwrap();
            }
            let down = super::super::super::directed::round_toward(ctx, sum, Toward::Down).unwrap();
            assert_eq!(up, down, "exact at 1024 bits");
            up
        };
        let (ro_w, t_w) = (w(ro), w(t));
        let ri = exact(&mut ctx, &mut sum, &[(ro_w, Endpoint::ONE, false), (t_w, Endpoint::ONE, true)]);
        let ro2 = exact(&mut ctx, &mut sum, &[(ro_w, ro_w, false)]);
        let ri2 = exact(&mut ctx, &mut sum, &[(ri, ri, false)]);
        let n1 = exact(&mut ctx, &mut sum, &[(Endpoint::ONE, Endpoint::ONE, false), (w(nu), Endpoint::ONE, false)]);
        let n1sq = exact(&mut ctx, &mut sum, &[(n1, n1, false)]);
        let sp = exact(&mut ctx, &mut sum, &[(ro2, Endpoint::ONE, false), (ri2, Endpoint::ONE, false)]);
        let sm = exact(&mut ctx, &mut sum, &[(ro2, Endpoint::ONE, false), (ri2, Endpoint::ONE, true)]);
        let sp2 = exact(&mut ctx, &mut sum, &[(sp, sp, false)]);
        let sm2 = exact(&mut ctx, &mut sum, &[(sm, sm, false)]);
        let n1sp2 = exact(&mut ctx, &mut sum, &[(n1sq, sp2, false)]);
        let num = exact(&mut ctx, &mut sum, &[(w(6.0), n1sp2, false)]);
        let nusq = exact(&mut ctx, &mut sum, &[(w(nu), w(nu), false)]);
        let cnu = exact(&mut ctx, &mut sum, &[(w(8.0), nusq, false), (w(14.0), w(nu), false), (w(7.0), Endpoint::ONE, false)]);
        let rr = exact(&mut ctx, &mut sum, &[(ri2, ro2, false)]);
        let rrn = exact(&mut ctx, &mut sum, &[(rr, n1sq, false)]);
        let den = exact(&mut ctx, &mut sum, &[(cnu, sm2, false), (w(48.0), rrn, false)]);
        // lo·Den ≤ Num ≤ hi·Den.
        let side = |ctx: &mut WideContext<16>, sum: &mut ExactWideSum, x: &Endpoint| {
            sum.clear();
            sum.add_product(ctx, x, &den, false).unwrap();
            sum.add_product(ctx, &num, &Endpoint::ONE, true).unwrap();
            sum.signum().unwrap()
        };
        let (lo, hi) = k.endpoints();
        assert!(side(&mut ctx, &mut sum, lo) <= 0 && side(&mut ctx, &mut sum, hi) >= 0, "κ_E outside its enclosure: D={d} t={t} ν={nu}");
        // κ̂ (binary64, FK's order) is excluded where it differs from κ_E.
        let kh = w(kappa);
        if side(&mut ctx, &mut sum, &kh) != 0 {
            assert!(kh.cmp_value(lo) == Ordering::Less || kh.cmp_value(hi) == Ordering::Greater, "κ̂ inside: D={d}");
            excluded += 1;
        }
        // admitted K_s = G·Â_s, one exact product.
        let mut c = WideContext::<16>::new(1024).unwrap();
        assert_eq!(shear.admitted(), c.mul(&w(g), &w(a_s)).unwrap());
        // annular K_s encloses g·κ_E·A: it lies within [g_lo κ_lo A_lo, g_hi κ_hi A_hi].
        let annular = shear.annular();
        let (alo, ahi) = annular.endpoints();
        assert!(alo.cmp_value(ahi) != Ordering::Greater && positive(alo));
    }
    assert!(excluded >= 1, "a section where κ̂ ≠ κ_E");
    // Only the exact E/ν material carries ν: an ordinary material is refused.
    let [a, i, j, z, _] = section(T);
    let input = MemberOperands { diameter: D, effective_wall: T, material: MaterialOperands::Ordinary { e: E, g: 80e9 },
        admitted: AdmittedOperands { e: E, g: 80e9, a, j, iz: i, iy: i, z_hat: z } };
    let mut work = NumericWork::new();
    let mut enc = build_member(&input, &mut work).unwrap();
    assert_eq!(shear_rigidity(&input, &mut enc, 1e-3, &mut work).unwrap_err(), NumericError::InvalidMaterial);
}

/// §4.5: a member whose reduced section differs (here t_r = t − 0.0005) has
/// its 21 reduced stress and maximum rows certified on the reduced section
/// with the nominal twins' recipes, in both lanes; each reduced axial row is
/// its action over A_r, its scale the body's divided by its own section.
/// (Like the nominal B3-K rows, nearby walls lose one or two bending rows to
/// DEF-O's projection and represented-Z rounding under SharperExact, the same
/// rows with either shear model: an availability limit, failing closed.)
#[test]
fn o4_reduced_rows_certify_on_the_reduced_section() {
    for shear in [false, true] {
        let source = cantilever(80e9, [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]);
        let set = TimoshenkoLeafSet::new(&source, vec![prepared_leaf(7, NU)]).unwrap();
        let (invocation, case) = solve(&source, shear.then_some(&set));
        let owner = owner(&case);
        let r = rows(owner, true);
        let s = specs(&r);
        let t_r = T - 0.0005;
        let reduced = reduced_set(&r, t_r);
        let material = ProductMaterial::BaseENu { e: E, nu: NU };
        let (proof, values) = certified(prove(&invocation, case.run, owner, &facts(material), &reduced, &s), "reduced");
        assert_eq!(proof.verdicts().len(), r.len());
        assert_eq!(proof.work().reduced_capacities()[1], 21);
        let [a, _, _, z, _] = section(T);
        let [a_r, _, _, z_r, _] = section(t_r);
        let mut compared = 0;
        for (k, row) in r.iter().enumerate() {
            if !row.0.contains("-reduced-") {
                continue;
            }
            let twin = r.iter().position(|x| x.0 == row.0.replace("-reduced-", "-nominal-")).unwrap();
            let (v, n) = (&proof.verdicts()[k], &proof.verdicts()[twin]);
            assert!(v.passed && v.class.is_some(), "{}", row.0);
            if let ProductRecipe::Stress { stress: ProductStress::Axial, .. } = row.3 {
                // σ_r/σ_n = A/A_r (the same action), and the scale's axial part follows.
                let ratio = values[k] / values[twin];
                assert!((ratio - a / a_r).abs() <= 1e-12 * (a / a_r), "{}: {ratio}", row.0);
                compared += 1;
            }
            if let ProductRecipe::Stress { stress: ProductStress::BendingZ, site: ProductSite::End(End::I), .. } = row.3 {
                let ratio = values[k] / values[twin];
                assert!((ratio - z / z_r).abs() <= 1e-9 * (z / z_r), "{}: {ratio}", row.0);
            }
            assert!(f64::from_bits(v.scale_bits) > f64::from_bits(n.scale_bits), "{}: own section scale", row.0);
        }
        assert_eq!(compared, 5);
        // The same rows without the set: each reduced row duplicates its nominal slot.
        let cause = refused(prove(&invocation, case.run, owner, &facts(material), ReducedSectionSet::empty(), &s));
        assert!(cause.contains("stress coverage"), "{cause}");
    }
}

/// §4.5: refusal by kernel cause of an unmapped reduced row, a mapped index
/// that is not a reduced stress or maximum row of that member, an index
/// outside the final rows, a repeated index, an entry bit-equal to the
/// nominal section and a missing reduced slot; an empty set keeps today's
/// coverage and capacities.
#[test]
fn o4_reduced_rows_are_refused_by_kernel_cause() {
    let source = cantilever(80e9, [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]);
    let (invocation, case) = solve(&source, None);
    let owner = owner(&case);
    let r = rows(owner, true);
    let s = specs(&r);
    let material = ProductMaterial::BaseENu { e: E, nu: NU };
    let f = facts(material);
    let t_r = T - 0.001;
    let good = reduced_set(&r, t_r);
    let entry = good.sections()[0].clone();
    let with = |rows: Vec<u32>, wall: f64| {
        let [a, i, j, z, _] = section(wall);
        ReducedSectionSet::new(vec![ReducedSection { member: 7, effective_wall: wall, area: a, second_moment: i,
            torsion_constant: j, section_modulus: z, rows }])
    };
    // One reduced row unmapped: its nominal slot is covered twice.
    let mut fewer = entry.rows.clone();
    fewer.pop();
    let cause = refused(prove(&invocation, case.run, owner, &f, &with(fewer, t_r).unwrap(), &s));
    assert!(cause.contains("coverage"), "{cause}");
    // A mapped native row.
    let mut native = entry.rows.clone();
    native[0] = 0;
    let cause = refused(prove(&invocation, case.run, owner, &f, &with(native, t_r).unwrap(), &s));
    assert!(cause.contains("reduced row identity") || cause.contains("coverage"), "{cause}");
    let mut only_native = entry.rows.clone();
    only_native.push(0);
    let cause = refused(prove(&invocation, case.run, owner, &f, &with(only_native, t_r).unwrap(), &s));
    assert!(cause.contains("reduced row identity"), "{cause}");
    // An index outside the final rows.
    let mut outside = entry.rows.clone();
    outside.push(r.len() as u32);
    let cause = refused(prove(&invocation, case.run, owner, &f, &with(outside, t_r).unwrap(), &s));
    assert!(cause.contains("reduced row index"), "{cause}");
    // A repeated index.
    let mut repeated = entry.rows.clone();
    repeated.push(entry.rows[0]);
    assert!(format!("{:?}", with(repeated, t_r).unwrap_err()).contains("reduced row index"));
    // A reduced entry bit-equal to the nominal section in all five operands.
    let cause = refused(prove(&invocation, case.run, owner, &f, &with(entry.rows.clone(), T).unwrap(), &s));
    assert!(cause.contains("reduced section equal to nominal"), "{cause}");
    // An entry naming another member.
    let [a, i, j, z, _] = section(t_r);
    let other = ReducedSectionSet::new(vec![ReducedSection { member: 8, effective_wall: t_r, area: a, second_moment: i,
        torsion_constant: j, section_modulus: z, rows: entry.rows.clone() }]).unwrap();
    let cause = refused(prove(&invocation, case.run, owner, &f, &other, &s));
    assert!(cause.contains("reduced section member"), "{cause}");
    // The nominal rows alone with an empty set: today's coverage, no reduced capacity.
    let nominal_rows = rows(owner, false);
    let ns = specs(&nominal_rows);
    let (proof, _) = certified(prove(&invocation, case.run, owner, &f, ReducedSectionSet::empty(), &ns), "nominal");
    assert_eq!(proof.work().reduced_capacities(), [0, 0]);
    assert_eq!(proof.work().capacities[4], 21);
}

// ------------------------------------------------- §4.3: the D-entry gaps

/// A skew cantilever with a deliberately inconsistent binary64 G (1e-6 above
/// E/(2(1+ν))): the two lanes' shear rigidities differ by about 1e-6 while
/// their EI agree to binary64 rounding.
fn gap_specimen(chord: [f64; 3]) -> (RecordedInvocation, RecordedCase, TimoshenkoLeafSet) {
    let g = E / (2.0 * (1.0 + NU)) * (1.0 + 1e-6);
    let source = cantilever(g, chord, [0.0, 0.0, 1.0]);
    let set = TimoshenkoLeafSet::new(&source, vec![prepared_leaf(7, NU)]).unwrap();
    let (invocation, case) = solve(&source, Some(&set));
    (invocation, case, set)
}
fn probe(owner: &adaptive::RetainedSolve, readout: source_residual::ReadoutLaw) -> source_residual::MemberLaneProbe {
    let f = facts(ProductMaterial::BaseENu { e: E, nu: NU });
    let m = &owner.source().members()[0];
    let law = bridge::ProposedMemberLaw { member: m, diameter: f[0].diameter, effective_wall: f[0].effective_wall,
        material: f[0].material.operands(), represented_z: f[0].section_modulus };
    let view = owner.source_bridge_view(owner.source(), &owner.evidence().source_encoding, owner.selected_precision())
        .result.unwrap();
    source_residual::test_member_lane(&view, &law, 0, readout).unwrap()
}
/// |d_G(L) − d_K(L)| per plane and entry at the exact length L, formed at
/// 1024 bits from both lanes' rigidities and the plane's EI (reference only;
/// the enclosures' midpoints, so a lower bound of the outward gap up to
/// ~2^-1000 relative).
fn plane_gap(probe: &source_residual::MemberLaneProbe, ei_g: Endpoint, ei_k: Endpoint, at: Endpoint) -> [Endpoint; 2] {
    let shear = probe.shear.unwrap();
    let mut ctx = WideContext::<16>::new(1024).unwrap();
    let mid = |ctx: &mut WideContext<16>, e: Enclosure| {
        let (lo, hi) = e.endpoints();
        ctx.add(lo, hi).unwrap().mul_pow2(-1).unwrap()
    };
    let ks_g = mid(&mut ctx, shear.annular());
    let ks_k = shear.admitted();
    let entries = |ctx: &mut WideContext<16>, ei: Endpoint, ks: Endpoint| {
        let c = ctx.div(&ei, &at).unwrap();
        let l2 = ctx.mul(&at, &at).unwrap();
        let den = ctx.mul(&ks, &l2).unwrap();
        let twelve = ctx.mul(&Endpoint::from_f64(12.0).unwrap(), &ei).unwrap();
        let phi = ctx.div(&twelve, &den).unwrap();
        let op = ctx.add(&Endpoint::ONE, &phi).unwrap();
        let t3 = ctx.mul(&Endpoint::from_f64(3.0).unwrap(), &c).unwrap();
        let cs = ctx.div(&t3, &op).unwrap();
        [ctx.add(&c, &cs).unwrap(), ctx.sub(&cs, &c).unwrap()]
    };
    let g = entries(&mut ctx, ei_g, ks_g);
    let k = entries(&mut ctx, ei_k, ks_k);
    [ctx.sub(&g[0], &k[0]).unwrap().abs(), ctx.sub(&g[1], &k[1]).unwrap().abs()]
}

/// §4.3: for a Timoshenko member the D-entry gaps are formed at the exact
/// length. On this skew member (φ > 3 at L, so a K_s-driven gap
/// 3EI·L·Δk/((L² + k_G)(L² + k_K)) increases with L) the gap formed at
/// ℓ = max|Δx_k| < L underestimates the exact gap; the lane's gaps contain
/// the exact-length gap. (The design's table names φ ≈ 1; at φ ≈ 1 such a gap
/// decreases in L and ℓ would overestimate it, so φ ≈ 6 is used.)
#[test]
fn timoshenko_gap_is_formed_at_the_exact_length() {
    let (_, case, _) = gap_specimen([0.03, 0.06, 0.09]);
    let owner = owner(&case);
    let p = probe(owner, source_residual::ReadoutLaw::AnnularSource);
    let gaps = p.tgap.unwrap();
    let mut ctx = WideContext::<16>::new(1024).unwrap();
    let (llo, lhi) = p.length.endpoints();
    let length = ctx.add(llo, lhi).unwrap().mul_pow2(-1).unwrap();
    assert!(p.lower_length.cmp_value(&length) == Ordering::Less, "ℓ < L");
    // EI_G is the annular coefficient enclosure's midpoint, EI_K = E·Î exactly.
    let mut work = NumericWork::new();
    let f = facts(ProductMaterial::BaseENu { e: E, nu: NU });
    let m = &owner.source().members()[0];
    let input = MemberOperands { diameter: D, effective_wall: T, material: f[0].material.operands(),
        admitted: AdmittedOperands { e: E, g: m.shear_modulus, a: m.area, j: m.torsion_constant,
            iz: m.second_moment_z, iy: m.second_moment_y, z_hat: f[0].section_modulus } };
    let enc = build_member(&input, &mut work).unwrap();
    for plane in 0..2 {
        let (lo, hi) = enc.coefficients[2 + plane].endpoints();
        let ei_g = ctx.add(lo, hi).unwrap().mul_pow2(-1).unwrap();
        let ei_k = enc.admitted_products[2 + plane];
        let at_l = plane_gap(&p, ei_g, ei_k, length);
        let at_ell = plane_gap(&p, ei_g, ei_k, p.lower_length);
        for e in 0..2 {
            assert!(at_ell[e].cmp_value(&at_l[e]) == Ordering::Less, "the gap at ℓ underestimates (plane {plane} entry {e})");
            assert!(gaps[plane][e].cmp_value(&at_l[e]) != Ordering::Less, "lane gap below the exact-length gap");
        }
    }
    // The admitted lane carries no gap (its readout is K_K's own).
    let k = probe(owner, source_residual::ReadoutLaw::AdmittedK);
    assert!(k.tgap.unwrap().iter().flatten().all(|g| g.is_zero()));
}

/// §4.3: a Timoshenko member's η uses the D-entry gaps, not the coefficient-
/// gap pattern. On this member the pattern (dd/ℓ·[[4,2],[2,4]]) is ~1e-10 of
/// the true |ΔD| (the lanes' EI agree, their K_s differ); the lane's η row
/// still dominates Σ_j |ΔK_aj|·s_j·s_a with ΔK = BᵀΔDB at the exact frame.
#[test]
fn timoshenko_eta_uses_d_entry_gaps() {
    let (_, case, _) = gap_specimen([0.03, 0.06, 0.09]);
    let owner = owner(&case);
    let p = probe(owner, source_residual::ReadoutLaw::AnnularSource);
    let gaps = p.tgap.unwrap();
    let mut ctx = WideContext::<16>::new(1024).unwrap();
    // The pattern bound underestimates the D-entry gaps by orders.
    for plane in 0..2 {
        let pattern = ctx.div(&p.dd[2 + plane].mul_pow2(2).unwrap(), &p.lower_length).unwrap();
        let scaled = pattern.mul_pow2(30).unwrap();
        assert!(scaled.cmp_value(&gaps[plane][0]) == Ordering::Less, "pattern ≪ gap");
    }
    // |ΔK| at the exact frame: B from the exact axes (1024-bit), ΔD from the gaps'
    // reference values; the η row must dominate Σ_j |ΔK_aj|·s_j·s_a.
    let view = owner.source_bridge_view(owner.source(), &owner.evidence().source_encoding, owner.selected_precision())
        .result.unwrap();
    let nodes = owner.source().nodes();
    let lift = |x: f64| Endpoint::from_f64(x).unwrap();
    let d: Vec<Endpoint> = (0..3).map(|k| lift(nodes[1][k] - nodes[0][k])).collect();
    fn dot3(ctx: &mut WideContext<16>, a: &[Endpoint], b: &[Endpoint]) -> Endpoint {
        let mut sum = ExactWideSum::new();
        for k in 0..3 {
            sum.add_product(ctx, &a[k], &b[k], false).unwrap();
        }
        sum.round(ctx).unwrap()
    }
    let l2 = dot3(&mut ctx, &d, &d);
    let length = ctx.sqrt(&l2).unwrap();
    let inv = ctx.div(&Endpoint::ONE, &length).unwrap();
    let ex: Vec<Endpoint> = d.iter().map(|x| ctx.div(x, &length).unwrap()).collect();
    let yr = [Endpoint::ZERO, Endpoint::ZERO, Endpoint::ONE];
    let pr = dot3(&mut ctx, &yr, &ex);
    let mut yc = Vec::new();
    for k in 0..3 {
        let t = ctx.mul(&pr, &ex[k]).unwrap();
        yc.push(ctx.sub(&yr[k], &t).unwrap());
    }
    let ny2 = dot3(&mut ctx, &yc, &yc);
    let ny = ctx.sqrt(&ny2).unwrap();
    let ey: Vec<Endpoint> = yc.iter().map(|x| ctx.div(x, &ny).unwrap()).collect();
    let mut ez = Vec::new();
    for &(a, b) in &[(1usize, 2usize), (2, 0), (0, 1)] {
        let u = ctx.mul(&ex[a], &ey[b]).unwrap();
        let v = ctx.mul(&ex[b], &ey[a]).unwrap();
        ez.push(ctx.sub(&u, &v).unwrap());
    }
    // Rows 2..5 of B (the bending rows), global; only the gap planes enter ΔK.
    let mut b = [[Endpoint::ZERO; 12]; 6];
    for k in 0..3 {
        let iy = ctx.mul(&inv, &ey[k]).unwrap();
        let iz = ctx.mul(&inv, &ez[k]).unwrap();
        for (row, rot) in [(2usize, 3usize), (3, 9)] { b[row][k] = iy; b[row][6 + k] = iy.neg(); b[row][rot + k] = ez[k]; }
        for (row, rot) in [(4usize, 3usize), (5, 9)] { b[row][k] = iz.neg(); b[row][6 + k] = iz; b[row][rot + k] = ey[k]; }
    }
    // ΔD per plane from the exact-length reference gaps (signed as the lanes'
    // midpoints give them, magnitudes only enter |ΔK| ≤ ... via its entries).
    let f = facts(ProductMaterial::BaseENu { e: E, nu: NU });
    let m = &owner.source().members()[0];
    let input = MemberOperands { diameter: D, effective_wall: T, material: f[0].material.operands(),
        admitted: AdmittedOperands { e: E, g: m.shear_modulus, a: m.area, j: m.torsion_constant,
            iz: m.second_moment_z, iy: m.second_moment_y, z_hat: f[0].section_modulus } };
    let mut work = NumericWork::new();
    let enc = build_member(&input, &mut work).unwrap();
    let mut delta = [[Endpoint::ZERO; 6]; 6];
    for plane in 0..2 {
        let (lo, hi) = enc.coefficients[2 + plane].endpoints();
        let ei_g = ctx.add(lo, hi).unwrap().mul_pow2(-1).unwrap();
        let g = plane_gap(&p, ei_g, enc.admitted_products[2 + plane], length);
        let (r0, r1) = (2 + 2 * plane, 3 + 2 * plane);
        delta[r0][r0] = g[0]; delta[r1][r1] = g[0]; delta[r0][r1] = g[1]; delta[r1][r0] = g[1];
    }
    // |ΔK_aj| ≤ Σ_rs |B_ra| |ΔD_rs| |B_sj| is an upper bound; the η row must
    // dominate the smaller |Σ_rs B_ra ΔD_rs B_sj| with the gaps' common sign
    // pattern (ΔD here is entrywise |·|, so this is a lower bound of the row).
    let positions = &view.group().ordering.position;
    let scales = view.scales();
    let mut dominated = 0;
    for a in 6..12 {
        let pa = positions[a];
        if pa == usize::MAX { continue; }
        let mut row = Endpoint::ZERO;
        for jj in 6..12 {
            let pj = positions[jj];
            if pj == usize::MAX { continue; }
            let mut k = Endpoint::ZERO;
            for r in 2..6 { for s in 2..6 {
                let u = ctx.mul(&b[r][a], &delta[r][s]).unwrap();
                let t = ctx.mul(&u, &b[s][jj]).unwrap();
                k = ctx.add(&k, &t).unwrap();
            } }
            let sj = Endpoint::ONE.mul_pow2(scales[pj]).unwrap();
            let t = ctx.mul(&k.abs(), &sj).unwrap();
            row = ctx.add(&row, &t).unwrap();
        }
        let row = row.mul_pow2(scales[pa]).unwrap();
        if row.is_zero() { continue; }
        // Allow the 1024-bit reference's own rounding (relative 2^-900).
        let slack = ctx.sub(&row, &row.mul_pow2(-900).unwrap()).unwrap();
        assert!(p.eta[pa].cmp_value(&slack) != Ordering::Less, "η row {a} below Σ|ΔK|s");
        dominated += 1;
    }
    assert!(dominated >= 4, "{dominated}");
}
