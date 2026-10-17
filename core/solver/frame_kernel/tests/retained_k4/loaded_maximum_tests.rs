use super::*;
fn point(x:f64)->Enclosure{Enclosure::point(lift(x).unwrap())}
fn run(ends:[[f64;3];2],q:[f64;2])->(Result<Enclosure,ProductFailure>,ProductCertificateSpent<'static>){
    let mut spent=ProductCertificateSpent::new(&[]);
    let result=search(&mut spent,7,11,ends.map(|p|p.map(point)),q.map(point),point(1.),point(1.));
    (result,spent)
}
fn contains(interval:Enclosure,n:f64,d:f64){
    let mut w=NumericWork::new();
    let lo=w.scalar(Entry::Mul,&interval.lo,&lift(d).unwrap(),Toward::Up).unwrap();
    let hi=w.scalar(Entry::Mul,&interval.hi,&lift(d).unwrap(),Toward::Down).unwrap();
    assert!(lo.cmp_value(&lift(n).unwrap())!=Ordering::Greater);
    assert!(hi.cmp_value(&lift(n).unwrap())!=Ordering::Less);
}
#[test]
fn w1_loaded_maximum_interior_controls_certify(){
    let (result,w)=run([[0.;3];2],[8.,0.]);let interval=result.unwrap();contains(interval,2.,1.);
    assert_eq!(interval.lo,interval.hi);
    let (result,_)=run([[1.,0.,0.],[2.,0.,0.]],[7.,0.]);contains(result.unwrap(),23.,7.);
    assert!(w.numeric.wide.checked_lme().exact().unwrap()>0);
    assert!(w.numeric.sums.checked_lme().exact().unwrap()>0);
    assert!(w.loaded_search_bytes<=1024*1024);
}
#[test]
fn w1_loaded_maximum_bubble_sign_and_axes(){
    for q in [[8.,0.],[0.,8.],[-8.,0.],[0.,-8.]]{
        let (result,_)=run([[0.;3];2],q);contains(result.unwrap(),2.,1.);
    }
    let (a,_)=run([[1.,0.,0.],[2.,0.,0.]],[7.,0.]);
    let (b,_)=run([[2.,0.,0.],[1.,0.,0.]],[7.,0.]);
    contains(a.unwrap(),23.,7.);contains(b.unwrap(),23.,7.);
}
#[test]
fn w1_loaded_maximum_exhaustion_widens_and_refuses(){
    let (result,w)=hooks::with((1,0,0),||run([[1.,0.,0.],[2.,0.,0.]],[7.,0.]));
    assert_eq!(result.unwrap_err().category(),"association");
    let diagnostic=w.loaded_maximum_diagnostic().unwrap();assert_eq!((diagnostic.row,diagnostic.member),(7,11));
    let interval=Enclosure{lo:Endpoint::from_parts(diagnostic.lower.0,diagnostic.lower.1,diagnostic.lower.2).unwrap(),
        hi:Endpoint::from_parts(diagnostic.upper.0,diagnostic.upper.1,diagnostic.upper.2).unwrap()};
    contains(interval,23.,7.);
    // The unchanged absolute-class predicate would accept this broad interval;
    // the search still refused before any projection/publication.
    let value=0.;let rows=[ProductFinalRow{id:"m",case_id:"c",value:&value,unit:ProductUnit::Pascal,body:0,recipe:ProductRecipe::CircularMaximum{member:11}}];
    let mut spent=ProductCertificateSpent::new(&rows);
    assert!(gate(&mut spent,0,interval,1e30,false).unwrap().passed);
}
fn g7(e:ProductMaximumEvidence){
    assert!(e.lower>=0.&&e.lower<=e.value&&e.value<=e.upper);
    assert_eq!((e.lower+0.5*(e.upper-e.lower)).to_bits(),e.value.to_bits());
    assert_eq!((e.upper-e.lower).to_bits(),e.gap.to_bits());
}
#[test]
fn w1_loaded_maximum_evidence_covers_binade_and_subnormal_edges(){
    let n=f64::from_bits(1f64.to_bits()-1);let lo=1.-2f64.powi(-51);let hi=1.+2f64.powi(-52);
    let mut w=NumericWork::new();let e=evidence(&mut w,n,point(n),lo,hi,&mut WorkTotal::zero()).unwrap();g7(e);
    assert_eq!(e.upper.to_bits(),hi.to_bits());assert_eq!(e.lower.to_bits(),lo.to_bits());
    for bits in [1u64,3,5,0x0010_0000_0000_0001,0x001f_ffff_ffff_ffff]{
        let upper=f64::from_bits(bits);let mut w=NumericWork::new();
        let e=evidence(&mut w,0.,Enclosure{lo:Endpoint::ZERO,hi:lift(upper).unwrap()},0.,upper,&mut WorkTotal::zero()).unwrap();g7(e);
        assert_eq!(e.upper.to_bits(),bits+1);assert_eq!(e.lower,0.);
    }
    let mut w=NumericWork::new();let e=evidence(&mut w,f64::MAX,point(f64::MAX),f64::MAX,f64::MAX,&mut WorkTotal::zero()).unwrap();g7(e);
    assert_eq!(e.upper,f64::MAX);assert_eq!(e.lower,0.);
}
#[test]
fn w1_loaded_evidence_contains_pp_global_upper(){
    let mut w=NumericWork::new();let e=evidence(&mut w,4.,point(4.),3.99,4.000001,&mut WorkTotal::zero()).unwrap();g7(e);
    assert!(e.lower<=3.99&&e.upper>=4.000001);
    assert!(w.wide.checked_lme().exact().unwrap()>0&&w.sums.checked_lme().exact().unwrap()>0);
}
