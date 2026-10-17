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

fn rational_inside(v:Enclosure,n:i128,d:i128) {
 let mut ctx=WideContext::<16>::new(1024).unwrap();
 for (x,neg) in [(v.lo,false),(v.hi,true)] {
  let mut s=ExactWideSum::new();
  s.add_product(&mut ctx,&x,&lift(d as f64).unwrap(),neg).unwrap();
  s.add_binary64(n as f64,!neg).unwrap();
  assert!(s.signum().unwrap()<=0,"rational {n}/{d} outside enclosure");
 }
}
#[test]
fn w1_loaded_maximum_signed_axial_two_axis_exact_random_controls() {
 let mut seed=168003127u64;let mut next=|| {seed=seed.wrapping_mul(6364136223846793005).wrapping_add(1);((seed>>32)%21) as i128-10};
 for case in 0..200 {
  let (n0,n1,m0,m1,q)=(next(),next(),next(),next(),next());
  let mut best=(0i128,1i128);
  for sn in [-1i128,1] {for sm in [-1i128,1] {
   let (a,b,c)=(-5*sm*q,sn*(n1-n0)+5*sm*(m1-m0+q),sn*n0+5*sm*m0);
   for (n,d) in [(c,1),(a+b+c,1)] {if n*best.1>best.0*d {best=(n,d);}}
   if a<0 && b>0 && b< -2*a {let (n,d)=(b*b-4*a*c,-4*a);if n*best.1>best.0*d {best=(n,d);}}
  }}
  let ends=[[n0 as f64,(3*m0)as f64,(4*m0)as f64],[n1 as f64,(3*m1)as f64,(4*m1)as f64]];
  let (v,spent)=run(ends,[(3*q)as f64,(4*q)as f64]);
  let v=v.unwrap_or_else(|e|panic!("case {case} n={n0},{n1} m={m0},{m1} q={q}: {e:?}"));
  rational_inside(v,best.0,best.1);
  assert!(spent.loaded_search_bytes<=1024*1024);
 }
 println!("INDEPENDENT_EXACT_MAXIMA 200 collinear two-axis, signed axial controls PASS");
}
#[test]
fn w1_loaded_maximum_rotating_bending_and_evidence_grids() {
 let (v,_)=run([[0.,1.,0.],[0.,0.,1.]],[1.,1.]);let v=v.unwrap();
 // ||M||^2 = 1 + 2*t^2*(1-t)^2, exact maximum9/8 at1/2.
 let mut ctx=WideContext::<16>::new(1024).unwrap();
 for (x,neg) in [(v.lo,false),(v.hi,true)] {let mut s=ExactWideSum::new();s.add_product(&mut ctx,&x,&x,neg).unwrap();s.add_binary64(1.125,!neg).unwrap();assert!(s.signum().unwrap()<=0);}
 let mut seed=99674180u64;let mut count=0;
 for i in 0..10012 {
  seed=seed.wrapping_mul(6364136223846793005).wrapping_add(1);
  let bits=match i {0=>1,1=>3,2=>5,3=>0xfffffffffffff,4=>0x10000000000000,5=>0x10000000000001,
   6=>0x1fffffffffffff,7=>0x20000000000000,8=>0x3fefffffffffffff,9=>0x3ff0000000000000,
   10=>0x7feffffffffffffe,11=>0x7fefffffffffffff,_=>1+seed%0x7fefffffffffffff};
  let n=f64::from_bits(bits);let lo=f64::from_bits(bits-1);let hi=f64::from_bits((bits+1).min(0x7fefffffffffffff));
  let mut w=NumericWork::new();let e=evidence(&mut w,n,point(n),lo,hi,&mut WorkTotal::zero()).unwrap();
  assert!(e.lower>=0.&&e.lower<=lo&&e.upper>=hi);
  assert_eq!((e.lower+0.5*(e.upper-e.lower)).to_bits(),e.value.to_bits(),"n{bits:016x}");
  assert_eq!((e.upper-e.lower).to_bits(),e.gap.to_bits());count+=1;
 }
 println!("INDEPENDENT_EVIDENCE {count} compiled endpoint grids PASS");
}

#[test]
fn w1_loaded_maximum_near_degenerate_capacity_keeps_certified_enclosure() {
    // Keep this fixed e=1e-5 input and the production capacities unchanged.
    // Zero axial force must retain one tangent branch, allowing certification.
    let f:Vec<_>=include_str!("member_maximum_capacity.txt").split_whitespace().collect();
    let values:Vec<_>=f[2..10].iter().map(|s|f64::from_bits(u64::from_str_radix(s,16).unwrap())).collect();
    let ends=[values[0..3].try_into().unwrap(),values[3..6].try_into().unwrap()];
    let(result,spent)=run(ends,[values[6],values[7]]);
    let v=result.expect("zero-axial tangent keeps the fixed near-degenerate case within production caps");
    let(lo,hi)=(v.lo,v.hi);
    assert!(spent.loaded_maximum_diagnostic().is_none());
    println!("NEAR_DEGENERATE e=1e-5 certified: wide={} sum={} heap={}",spent.numeric.wide.checked_lme().exact().unwrap(),spent.numeric.sums.checked_lme().exact().unwrap(),spent.loaded_search_bytes);
    assert!(compare_reference(lo,f[10],false)<=0);assert!(compare_reference(hi,f[11],true)<=0);
    assert!(spent.numeric.wide.checked_lme().exact().unwrap()>0);assert!(spent.numeric.sums.checked_lme().exact().unwrap()>0);
    assert!(spent.loaded_search_bytes<=1024*1024);
}

fn reference_limbs(hex:&str)->Vec<u64>{
    let mut limbs=Vec::new();let mut end=hex.len();
    while end>0{let start=end.saturating_sub(16);limbs.push(u64::from_str_radix(&hex[start..end],16).unwrap());end=start;}limbs
}
fn compare_reference(value:Endpoint,token:&str,negate_value:bool)->i8 {
    let (m,e)=token[1..].split_once('@').unwrap();let mut sum=ExactWideSum::new();
    sum.add_wide(&value,negate_value).unwrap();
    sum.add_integer(token.starts_with('-')==negate_value,&reference_limbs(m),e.parse().unwrap()).unwrap();
    sum.signum().unwrap()
}
#[test]
fn w1_loaded_maximum_general_noncollinear_2048bit_reference(){
    let mut count=0;
    for line in include_str!("member_maximum.txt").lines().filter(|l|!l.starts_with('#')) {
        let f:Vec<_>=line.split_whitespace().collect();assert_eq!(f[0],"case");
        let values:Vec<_>=f[2..10].iter().map(|s|f64::from_bits(u64::from_str_radix(s,16).unwrap())).collect();
        let ends=[values[0..3].try_into().unwrap(),values[3..6].try_into().unwrap()];
        let(result,spent)=run(ends,[values[6],values[7]]);let v=result.unwrap_or_else(|e|panic!("case{} {e:?}",f[1]));
        // The independently isolated true maximum is enclosed in [rlo,rhi].
        // Prove the entire much narrower reference hull lies inside ours.
        assert!(compare_reference(v.lo,f[10],false)<=0,"case{} lower",f[1]);
        assert!(compare_reference(v.hi,f[11],true)<=0,"case{} upper",f[1]);
        assert!(spent.loaded_search_bytes<=1024*1024);count+=1;
    }
    assert_eq!(count,200);
}

#[test]
fn w1_loaded_maximum_rotating_control_uses_joint_tangent_bound(){
    // Exact rotating polynomial has squared maximum9/8. At the root node,
    // paired-control norms alone give sqrt(2); the degree-four tangent gives
    // (4/3+9/8)/(2sqrt(9/8))<6/5. This checks the implemented upper path.
    let controls=[[point(0.),point(1.),point(0.)],[point(0.),point(1.),point(1.)],[point(0.),point(0.),point(1.)]];
    let mut w=NumericWork::new();let bound=upper(&mut w,controls,point(1.),point(1.)).unwrap();
    assert!(bound.cmp_value(&lift(1.2).unwrap())==Ordering::Less);
    assert!(bound.cmp_value(&lift(1.).unwrap())==Ordering::Greater);
    println!("ZERO_AXIAL_TANGENT wide={} sum={}",w.wide.checked_lme().exact().unwrap(),w.sums.checked_lme().exact().unwrap());
}

#[test]
fn w1_loaded_maximum_rotating_and_axial_slope_within_capacity(){
    for (ends,q) in [([[0.,1.,0.],[0.,0.,1.]],[1.,1.]),([[1.,0.,0.],[2.,0.,0.]],[7.,0.])]{
        let(result,spent)=hooks::with((32,64,64),||run(ends,q));assert!(result.is_ok(),"rotating/axial-slope capacity control");
        assert!(spent.loaded_search_bytes<=32*std::mem::size_of::<SearchNode>());
    }
}

#[test]
fn w1_loaded_maximum_keeps_axial_and_bending_divisors_distinct(){
    for(ends,q,value)in[([[4.,0.,0.],[8.,0.,0.]],[0.,0.],4.),([[0.;3];2],[8.,0.],0.5)]{
        let mut spent=ProductCertificateSpent::new(&[]);
        let hull=search(&mut spent,0,0,ends.map(|v|v.map(point)),q.map(point),point(2.),point(4.)).unwrap();
        contains(hull,value,1.);
    }
    let mut spent=ProductCertificateSpent::new(&[]);
    let hull=search(&mut spent,0,0,[[1.,1.,0.],[1.,0.,1.]].map(|v|v.map(point)),[1.,1.].map(point),point(2.),point(4.)).unwrap();
    // Maximum =1/2 +sqrt(9/8)/4. Exact dyadic subtraction/scaling followed by
    // an exact square compares the hull without a binary64 square-root oracle.
    let mut ctx=WideContext::<16>::new(1024).unwrap();
    for(v,negative)in[(hull.lo,false),(hull.hi,true)]{
        let mut sum=ExactWideSum::new();sum.add_wide(&v,false).unwrap();sum.add_binary64(0.5,true).unwrap();
        let bending=sum.round(&mut ctx).unwrap().mul_pow2(2).unwrap();
        let mut difference=ExactWideSum::new();difference.add_product(&mut ctx,&bending,&bending,negative).unwrap();difference.add_binary64(1.125,!negative).unwrap();
        assert!(difference.signum().unwrap()<=0);
    }
}
