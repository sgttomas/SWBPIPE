#[test]
fn rv56_relative_ceiling_refusal_retains_five_operations() {
    let mut s = solved(source(true, false));
    let index = s.publish().rows.iter().position(|r| matches!(r.class, RowClass::RelativeVerified)).unwrap();
    let prior = s.publish().rows[..index].iter().filter(|r| matches!(r.class, RowClass::RelativeVerified)).count() as u64 * 5;
    let row = &s.publish().rows[index];
    let scale = f64::from_bits(s.publish().body_scales[row.body as usize * 4 + row.kind.index()].2);
    let ceiling = sharper_binary64(row.value.value().unwrap(), scale).unwrap();
    assert!(ceiling > 0.0 && ceiling < 1.0);
    // Private corruption control: the row remains valid/finite, but the radius
    // exceeds the ceiling only after the actual five-operation helper has run.
    s.publication_radius_bits[index] = 1.0f64.to_bits();
    let checked = s.source_bridge_view(s.source(), &s.prep.identity, s.selected);
    assert!(matches!(checked.result, Err(SourceBridgeViewIssue::Certificate(CertificateIssue::RadiusClassMismatch))));
    println!("RV56_RADIUS_FAILURE index={index} ceiling={ceiling:?} expected_f64={} actual_f64={:?}", prior+5, checked.work.f64_operations.exact());
    assert_eq!(checked.work.f64_operations.exact(), Ok(prior + 5), "executed sharper-binary64 work must survive its caller's ceiling refusal");
}

#[path = "review_vectors.rs"]
mod rv56_oracle;
#[test]
fn rv56_rotated_nonunit_length_axial_spring_source_enclosure() {
    let mut parts = parts_from(&source(false, false));
    parts.nodes[1] = [0.0, 2.0, 0.0];
    parts.members[0].y_reference = [0.0, 0.0, 1.0];
    let dof = Dof { node: 1, component: Component::Uy };
    parts.loads = vec![NodalLoad { dof, value: 1.0, source_id: "review-axial".into() }];
    parts.springs.push(super::super::source::Spring { id: 9, dof, stiffness: 100000.0 });
    parts.supports.push(SupportGroup { id: 4, node: 1, restrained: [false;6], springs:vec![9], directional_springs:vec![] });
    let s = solved(PrimitiveSource::new(parts).unwrap());
    let mut input = laws(&s);
    input[0].material = MaterialOperands::ExactENu { e: f64::from_bits(oracle::E), nu: 0.3125 };
    let spent = run(&s, &input);
    let native = spent.result().unwrap();
    assert_eq!(native.rows.len(), 55);
    let u = pair(rv56_oracle::U);
    let member = pair(rv56_oracle::MEMBER);
    let spring = pair(rv56_oracle::SPRING);
    let neg = |v: (Wide<16>,Wide<16>)| (v.1.neg(),v.0.neg());
    for (i,row) in s.publish().rows.iter().enumerate() {
        let truth = match row.id {
            QuantityId::Displacement(d) if d == dof => u,
            QuantityId::DisplacementMagnitude(1) => u,
            QuantityId::EndAction { end: End::I, component: Component::Ux, .. } => neg(member),
            QuantityId::EndAction { end: End::J, component: Component::Ux, .. } => member,
            QuantityId::StationAction { component: Component::Ux, .. } => member,
            QuantityId::Reaction(Dof{node:0, component:Component::Uy}) => neg(member),
            QuantityId::SpringAction { spring:9, component:Component::Uy } => spring,
            QuantityId::SupportForceMagnitude(3) => member,
            QuantityId::SupportForceMagnitude(4) => neg(spring),
            _ => point(0.0),
        };
        let (lo,hi)=native.rows[i].source.endpoints();
        assert_ne!(lo.cmp_value(&truth.0), Cmp::Greater, "review {:?} lower",row.id);
        assert_ne!(hi.cmp_value(&truth.1), Cmp::Less, "review {:?} upper",row.id);
    }
    println!("RV56_ROTATED_SPRING source_contains=55/55 p={} P={} data={:?} member_builds={:?}",s.selected,s.evidence.verification_precision,native.view.data(),spent.work.member_builds.exact());
}
