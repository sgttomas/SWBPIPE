// Independent RV60 gate controls appended only to the external test copy.
#[test]
fn rv60_dual_hull_and_input_exactness_are_mandatory() {
    let value=1.0;
    let rows=[ProductFinalRow{id:"x",case_id:"c",value:&value,unit:ProductUnit::Newton,body:0,recipe:ProductRecipe::NonQuantity}];
    let mut one=ProductCertificateSpent::new(&rows);
    assert!(gate(&mut one,0,Enclosure::point(lift(1.0).unwrap()),1.0,false).unwrap().passed);
    let mut both=ProductCertificateSpent::new(&rows);
    let interval=hull(Enclosure::point(lift(1.0).unwrap()),Enclosure::point(lift(1.0+2f64.powi(-40)).unwrap()));
    let v=gate(&mut both,0,interval,1.0,false).unwrap();
    assert_eq!(v.failed,Some(ProductPredicate::SharperExact));
    assert_eq!(v.predicates,[Some(false),Some(false),Some(true),Some(true)]);
    let mut input=ProductCertificateSpent::new(&rows);
    let hi=input.numeric.scalar(Entry::Add,&lift(1.0).unwrap(),&lift(2f64.powi(-100)).unwrap(),Toward::Up).unwrap();
    let v=gate(&mut input,0,Enclosure{lo:lift(1.0).unwrap(),hi},1.0,true).unwrap();
    assert_eq!(v.failed,Some(ProductPredicate::InputDerived));
    println!("RV60_DUAL_HULL_AND_INPUT_EXACTNESS true");
}
#[test]
fn rv60_exact_absolute_boundary_and_signed_quotients() {
    let value=0.0;
    let rows=[ProductFinalRow{id:"x",case_id:"c",value:&value,unit:ProductUnit::Newton,body:0,recipe:ProductRecipe::NonQuantity}];
    let mut edge=ProductCertificateSpent::new(&rows);
    let b=lift(2f64.powi(-64)).unwrap();
    let v=gate(&mut edge,0,Enclosure{lo:Endpoint::ZERO,hi:b},1.0,false).unwrap();assert!(v.passed);
    let mut above=ProductCertificateSpent::new(&rows);
    let hi=above.numeric.scalar(Entry::Add,&b,&lift(2f64.powi(-200)).unwrap(),Toward::Up).unwrap();
    let v=gate(&mut above,0,Enclosure{lo:Endpoint::ZERO,hi},1.0,false).unwrap();assert_eq!(v.failed,Some(ProductPredicate::Absolute));
    let mut work=NumericWork::new();
    let signed=corners(&mut work,Enclosure{lo:lift(-6.0).unwrap(),hi:lift(2.0).unwrap()},Enclosure{lo:lift(2.0).unwrap(),hi:lift(4.0).unwrap()},Entry::Div).unwrap();
    assert_eq!(signed.lo.cmp_value(&lift(-3.0).unwrap()),Ordering::Equal);assert_eq!(signed.hi.cmp_value(&lift(1.0).unwrap()),Ordering::Equal);
    println!("RV60_ABSOLUTE_BOUNDARY_SIGN true");
}
