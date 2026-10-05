#[test]
fn source_residual_rv58_three_dimensional_bending_torsion_and_arbitrary_center() {
    let mut p = parts_from(&source(true, false));
    p.nodes[1] = [1., 2., 2.];
    p.members[0].y_reference = [2., -2., 1.];
    p.loads.clear();
    for (component, value) in Component::ALL.into_iter().zip([3., 9., -6., -2., -1., 11.]) {
        p.loads.push(NodalLoad { dof: Dof { node: 1, component }, value, source_id: format!("RV58-{component:?}") });
    }
    let s = solved(PrimitiveSource::new(p).unwrap());
    let input = laws(&s);
    let original = s.publish().clone();
    let spent = run(&s, &input);
    let native = spent.result().unwrap();
    assert_eq!(spent.work.correction.calls.exact(), Ok(1));
    let poor: Vec<_> = [0.001, -0.002, 0.003, 0.004, -0.005, 0.006].into_iter().map(|v|Wide::<16>::from_f64(v).unwrap()).collect();
    let (rho, eps, rows, work) = test_at_center(native, &poor).unwrap();
    assert_eq!(work.correction.calls.exact(), Ok(0));
    for (label, center, residual, epsilon, source_rows) in [
        ("rotated", &native.center, &native.rho, &native.epsilon, &native.rows),
        ("arbitrary", &poor, &rho, &eps, &rows),
    ] {
        for (index, (row, interval)) in s.publish().rows.iter().zip(source_rows).enumerate() {
            println!("RV58_ROW {{\"witness\":\"{label}\",\"index\":{index},\"id\":\"{:?}\",\"lo\":{},\"hi\":{}}}",row.id,encoded(interval.endpoints().0),encoded(interval.endpoints().1));
        }
        for a in 0..center.len() {
            println!("RV58_CENTER {{\"witness\":\"{label}\",\"a\":{a},\"s\":{},\"center\":{},\"rho_lo\":{},\"rho_hi\":{}}}",native.view.scales()[a],encoded(&center[a]),encoded(residual[a].endpoints().0),encoded(residual[a].endpoints().1));
        }
        println!("RV58_BLOCK {{\"witness\":\"{label}\",\"alpha\":{},\"epsilon\":{},\"bound\":\"{:016x}\",\"predicate_count\":{}}}",encoded(&native.alpha[0]),encoded(&epsilon[0]),s.evidence().certified_bound[0].1,predicate_count(&s,source_rows));
    }
    assert_eq!(s.publish(), &original);
}
