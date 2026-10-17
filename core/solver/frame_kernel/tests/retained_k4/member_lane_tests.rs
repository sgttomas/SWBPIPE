//! The actual lane consumes declared points both before and after the solve.
use super::*;
use super::super::super::{adaptive::{CaseLimit,ExecutionOutcome},origins::{RecordedInvocation,OriginCapacity},source::*};
#[allow(dead_code)]
#[path="source_residual_vectors.rs"] mod oracle;
fn fixture(fixed:bool,terms:&[[f64;3]])->(PrimitiveSource,MemberLoadSet){
    let source=PrimitiveSource::new(SourceParts{nodes:vec![[0.;3],[2.,0.,0.]],
        members:vec![StraightMember{id:7,node_i:0,node_j:1,elastic_modulus:f64::from_bits(oracle::E),shear_modulus:f64::from_bits(oracle::G),
            area:f64::from_bits(oracle::A),second_moment_y:f64::from_bits(oracle::I),second_moment_z:f64::from_bits(oracle::I),torsion_constant:f64::from_bits(oracle::J),y_reference:[0.,1.,0.]}],
        constraints:(0..if fixed{12}else{6}).map(|g|Constraint{dof:Dof::from_global(g),value:0.}).collect(),
        stations:vec![Station{id:0,member:7,fraction:0.5}],..Default::default()}).unwrap();
    let loads=MemberLoadSet::new(&source,terms.iter().enumerate().map(|(i,w)|UniformMemberLoad{member:7,intensity:*w,source_id:format!("w{i}")}).collect()).unwrap();(source,loads)
}
fn contains(v:Enclosure,value:f64){let p=lift(value).unwrap();assert!(v.lo.cmp_value(&p)!=Ordering::Greater,"lower {value}");assert!(v.hi.cmp_value(&p)!=Ordering::Less,"upper {value}");}
#[test]
fn member_load_lane_points_reach_free_rows_and_reactions(){
 for fixed in [false,true] {for terms in [vec![[0.,3.,0.]],vec![[0.,3.,0.],[0.,-3.,0.]],vec![[0.;3]],vec![]] {
    let active=terms.len()==1&&terms[0][1]!=0.;let(source,loads)=fixture(fixed,&terms);
    let mut invocation=RecordedInvocation::new(u64::MAX,OriginCapacity::for_calls(&[1],&[]).unwrap()).unwrap();
    let cases=invocation.solve_member_cases(&[MemberSourceRef::new(&source,&loads).unwrap()],CaseLimit::new(u64::MAX)).unwrap();
    let owner=match &cases[0].outcome{ExecutionOutcome::Selected(s)=>s,_=>panic!("member lane solve")};
    let laws=[ProposedMemberLaw{member:&owner.source().members()[0],diameter:f64::from_bits(oracle::D),effective_wall:f64::from_bits(oracle::T),
        material:MaterialOperands::Ordinary{e:f64::from_bits(oracle::E),g:f64::from_bits(oracle::G)},represented_z:f64::from_bits(oracle::Z)}];
    for law in [ReadoutLaw::AdmittedK,ReadoutLaw::AnnularSource] {
        let spent=source_residual_for_law(owner,owner.source(),&owner.evidence().source_encoding,owner.selected_precision(),&laws,law);
        let native=spent.result().unwrap();let view=&native.view;
        let center=vec![Endpoint::ZERO;view.group().ordering.free.len()];let epsilon=vec![Endpoint::ZERO;native.epsilon.len()];
        let mut work=ResidualWork::new();work.readout_law=law;
        let residual=residual(view,&laws,&center,None,&mut work).unwrap();
        for (a,&g) in view.group().ordering.free.iter().enumerate(){
            let expected=if active{match g{7=>3.,11=>-1.,_=>0.}}else{0.};
            // Source residual rows are in equilibrated coordinates.
            let expected=lift(expected).unwrap().mul_pow2(view.scales()[a]).unwrap();
            assert_eq!(residual[a].lo,expected,"{law:?} g{g}");assert_eq!(residual[a].hi,expected);
        }
        let (rows,_)=recover(view,&laws,&center,&epsilon,&mut work).unwrap();
        for (at,row) in owner.publish().rows.iter().enumerate(){
            if let QuantityId::Reaction(d)=row.id{
                let at_zero=if active{match d.global(){1|7=>-3.,5=>-1.,11=>1.,_=>0.}}else{0.};contains(rows[at],at_zero);
                let solved=if active{if fixed{at_zero}else{match d.global(){1=>-6.,5=>-6.,_=>0.}}}else{0.};
                contains(native.rows[at],solved);
            }
        }
    }
 }}
}
