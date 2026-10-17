//! Compiled verification and exact-ledger member-load boundary controls.
use super::*;
use super::super::{source::*,verify::{build_verify_shared,verify_state},wide::multi::SupportedWidth};
fn g_boundary<const L:usize,const R:usize,const W:usize>(p:u32,q:u32,k:u32,kind:u32)
where Wide<L>:SupportedWidth,Wide<R>:SupportedWidth,Wide<W>:SupportedWidth {
 let source=PrimitiveSource::new(SourceParts{
  nodes:vec![[0.;3],[1.,1.,2f64.powi(-(k as i32))]],
  members:vec![StraightMember{id:0,node_i:0,node_j:1,elastic_modulus:2.,shear_modulus:5.,area:3.,second_moment_y:11.,second_moment_z:7.,torsion_constant:2.,y_reference:[1.,1.,0.]}],
  constraints:(0..12).map(|g|Constraint{dof:Dof::from_global(g),value:if kind==5&&g==0{1.}else{0.}}).collect(),
  ..Default::default()}).unwrap();
 let terms=match kind {0=>vec![[1.,0.,0.]],1=>vec![[1.,0.,0.],[-1.,0.,0.]],2=>vec![[0.;3]],_=>Vec::new()};
 let set=MemberLoadSet::new(&source,terms.into_iter().enumerate().map(|(i,intensity)|UniformMemberLoad{member:0,intensity,source_id:format!("w{i}")}).collect()).unwrap();
 // Distinguish a None sidecar from an explicitly constructed empty one.
 let prep=if kind==3 {CasePrep::new(source.clone()).unwrap()} else {CasePrep::new_member(MemberSourceRef::new(&source,&set).unwrap()).unwrap()};
 let group=prepare_group(&source).unwrap();let guard=StageGuard::unlimited();
 let shared=build_shared::<L,R>(p,q,&source,&group,guard).result.unwrap();
 let state=solve_case_at::<L,R>(&shared,&prep,&group,guard).result.unwrap();
 let vs=build_verify_shared::<L,R,W>(&shared,&source,&group,guard).result.unwrap();
 let report=verify_state(&shared,&vs,&prep,&group,&state,guard).result.unwrap();
 let g=shared.members[0].g_exp;
 assert_eq!(g,k+1,"p{p} k{k} kind{kind}");
 let active=kind==0||kind==1||kind==5;
 assert_eq!(report.g_max,if active{g}else{0},"p{p} k{k} kind{kind}");
 assert_eq!(report.g_violation,if active&&g>p-16{Some(0)}else{None},"p{p} k{k} kind{kind}");

}
#[test]
fn member_load_g_check_covers_fixed_and_cancelling_sources() {
 for kind in 0..6 {for above in 0..2 {
  g_boundary::<4,4,8>(128,192,111+above,kind);
  g_boundary::<4,8,16>(256,320,239+above,kind);
  g_boundary::<8,16,16>(512,576,495+above,kind);
  g_boundary::<16,16,16>(1024,1024,1007+above,kind);
 }}
}

fn member_source(fixed:bool,nodal:Vec<NodalLoad>)->PrimitiveSource {
    PrimitiveSource::new(SourceParts{nodes:vec![[0.;3],[2.,0.,0.]],
        members:vec![StraightMember{id:0,node_i:0,node_j:1,elastic_modulus:1.,shear_modulus:1.,area:1.,second_moment_y:1.,second_moment_z:1.,torsion_constant:1.,y_reference:[0.,1.,0.]}],
        constraints:(0..if fixed{12}else{6}).map(|g|Constraint{dof:Dof::from_global(g),value:0.}).collect(),
        loads:nodal,stations:vec![super::super::source::Station{id:0,member:0,fraction:0.5}],..Default::default()}).unwrap()
}
fn preparation(source:&PrimitiveSource,intensities:&[[f64;3]])->CasePrep {
    let set=MemberLoadSet::new(source,intensities.iter().enumerate().map(|(i,w)|UniformMemberLoad{member:0,intensity:*w,source_id:format!("w{i}")}).collect()).unwrap();
    CasePrep::new_member(MemberSourceRef::new(source,&set).unwrap()).unwrap()
}
fn declaration_work(prep:&CasePrep)->u64 {
    let mut ctx=WideContext::<16>::new(1024).unwrap();let mut sum=ExactWideSum::new();
    let entries=super::super::declared::form(&prep.source,prep.member_loads(),&mut ctx,&mut sum,|_,_|Ok(())).unwrap();
    let _=RetainedLedger::with_declared(&prep.ledger,entries,&mut sum).unwrap();
    lme(&ctx).add(sum.work().checked_lme()).exact().unwrap()
}
fn once_at<const L:usize,const R:usize>(p:u32,q:u32,prep:&CasePrep,group:&GroupPrep,cost:u64)
where Wide<L>:SupportedWidth,Wide<R>:SupportedWidth {
    let guard=StageGuard::unlimited();let shared=build_shared::<L,R>(p,q,&prep.source,group,guard).result.unwrap();
    let before=prep.ledger().unwrap() as *const RetainedLedger;
    let warm=solve_case_at::<L,R>(&shared,prep,group,guard);
    let fresh=preparation(&prep.source,&[[0.,3.,0.]]);
    assert_eq!(fresh.try_ledger().unwrap_err(),LedgerRefusal::NotFormed);
    let cold=solve_case_at::<L,R>(&shared,&fresh,group,guard);
    assert_eq!(cold.stages.rhs-warm.stages.rhs,cost,"p{p}: declaration charged once in rhs");
    assert_eq!(warm.result.unwrap().u,cold.result.unwrap().u,"p{p}");
    assert_eq!(before,prep.ledger().unwrap() as *const RetainedLedger,"formed ledger retained across precisions");
}
#[test]
fn member_load_formation_is_charged_once_in_the_first_rhs() {
    let source=member_source(false,Vec::new());let prep=preparation(&source,&[[0.,3.,0.]]);
    assert_eq!(prep.try_ledger().unwrap_err(),LedgerRefusal::NotFormed);
    assert!(prep.ledger().is_err(),"unformed member source must not expose its nodal-only ledger");
    let cost=declaration_work(&prep);assert!(cost>0);
    let group=prepare_group(&source).unwrap();let guard=StageGuard::unlimited();
    let shared=build_shared::<4,4>(128,192,&source,&group,guard).result.unwrap();
    let first=solve_case_at::<4,4>(&shared,&prep,&group,guard);assert!(first.result.is_ok());
    assert!(first.stages.rhs>=cost);
    once_at::<4,4>(128,192,&prep,&group,cost);
    once_at::<4,8>(256,320,&prep,&group,cost);
    once_at::<8,16>(512,576,&prep,&group,cost);
    once_at::<16,16>(1024,1024,&prep,&group,cost);

    // Same-K basis reuse shares builds, while each source owns its declaration.
    use super::super::origins::{RecordedInvocation,OriginCapacity};
    let a=MemberLoadSet::new(&source,vec![UniformMemberLoad{member:0,intensity:[0.,3.,0.],source_id:"a".into()}]).unwrap();
    let b=MemberLoadSet::new(&source,vec![UniformMemberLoad{member:0,intensity:[0.,-6.,0.],source_id:"b".into()}]).unwrap();
    let mut invocation=RecordedInvocation::new(u64::MAX,OriginCapacity::for_calls(&[2],&[]).unwrap()).unwrap();
    let cases=invocation.solve_member_cases(&[MemberSourceRef::new(&source,&a).unwrap(),MemberSourceRef::new(&source,&b).unwrap()],CaseLimit::new(u64::MAX)).unwrap();
    let owners:Vec<_>=cases.iter().map(|c|match &c.outcome{ExecutionOutcome::Selected(s)=>s,_=>panic!("reused member case")}).collect();
    assert!(owners[0].evidence().attempts[0].shared_built_here);assert!(!owners[1].evidence().attempts[0].shared_built_here);
    assert_eq!(owners[0].evidence().attempts[0].stages.rhs,owners[1].evidence().attempts[0].stages.rhs);
    assert!(!std::ptr::eq(owners[0].prep.ledger().unwrap(),owners[1].prep.ledger().unwrap()));
    let tip=|s:&RetainedSolve|s.publish().rows.iter().find(|r|r.id==QuantityId::Displacement(Dof::from_global(7))).unwrap().value.value().unwrap();
    assert_eq!(tip(owners[0]),6.);assert_eq!(tip(owners[1]),-12.);
}

#[test]
fn member_load_residual_reads_the_declared_ledger() {
    let source=member_source(false,vec![NodalLoad{dof:Dof::from_global(7),value:3.,source_id:"nodal".into()}]);
    let prep=preparation(&source,&[[0.,-2.,0.]]);let group=prepare_group(&source).unwrap();let guard=StageGuard::unlimited();
    let shared=build_shared::<4,4>(128,192,&source,&group,guard).result.unwrap();
    let _=solve_case_at::<4,4>(&shared,&prep,&group,guard).result.unwrap();
    let mut k=vec![Wide::<4>::ZERO;group.structure.pattern.entry_count()];
    let index=group.structure.entries().find(|(i,j,_)|(*i,*j)==(7,7)).unwrap().2;k[index]=Wide::<4>::ONE;
    for (u_value,expected_residual,ratio_exp) in [(0.,1.,121),(-1.,2.,120)] {
        let mut ctx=WideContext::<4>::new(128).unwrap();let mut cq=WideContext::<4>::new(192).unwrap();
        let mut c64=WideContext::<4>::new(64).unwrap();let mut c16=WideContext::<16>::new(1024).unwrap();let mut sum=ExactWideSum::new();
        let mut u=vec![Wide::<4>::ZERO;12];u[7]=Wide::<4>::from_f64(u_value).unwrap();let mut tracker=BoundedExtremeTracker::new(Direction::Up);
        let rows=residual_rows(&mut ctx,&mut cq,&mut c64,&mut c16,&mut sum,128,&group.structure,&k,prep.ledger().unwrap(),&[7],&u,&mut tracker).unwrap();
        assert_eq!(rows[0].2,Wide::<4>::from_f64(expected_residual).unwrap());
        // m=2 (zero K*u products) or4 (one), independent of load count.
        // d=|3-2|+|K*u|, not |3|+|-2|+|K*u|.
        assert_eq!(rows[0].1,2f64.powi(ratio_exp));
    }
}

#[test]
fn member_load_data_flag_counts_declared_entries() {
    use super::super::bound::{data_blocks,fill_data_blocks};
    for (terms,nodal,expected) in [
        (vec![[0.,3.,0.]],Vec::new(),true),
        (vec![[0.,3.,0.],[0.,-3.,0.]],Vec::new(),true),
        (vec![[0.,3.,0.]],vec![NodalLoad{dof:Dof::from_global(7),value:-3.,source_id:"cancel_force".into()},
            NodalLoad{dof:Dof::from_global(11),value:1.,source_id:"cancel_moment".into()}],true),
        (vec![[0.;3]],Vec::new(),false),
        (Vec::new(),Vec::new(),false),
    ] {
        let source=member_source(false,nodal);let prep=preparation(&source,&terms);let group=prepare_group(&source).unwrap();let guard=StageGuard::unlimited();
        let shared=build_shared::<4,4>(128,192,&source,&group,guard).result.unwrap();let _=solve_case_at::<4,4>(&shared,&prep,&group,guard).result.unwrap();
        let u=vec![Wide::<4>::ZERO;12];let prescribed=vec![false;12];
        let data=data_blocks(&group.blocks,&group.ordering,&group.structure,prep.ledger().unwrap(),&prescribed,&u);
        assert_eq!(data,vec![expected]);let mut borrowed=vec![false;data.len()];
        fill_data_blocks(&group.blocks,&group.ordering,&group.structure,prep.ledger().unwrap(),&prescribed,&u,&mut borrowed);assert_eq!(borrowed,data);
        if terms.len()==2 {assert!(prep.ledger().unwrap().net(7).unwrap().is_zero());assert!(prep.ledger().unwrap().has_nonzero_term(7));}
    }
}

#[test]
fn member_load_fixed_fixed_reactions_and_correction_recovery_is_linear() {
    use super::super::{verify::StageRounding,recover::End};
    let source=member_source(true,Vec::new());let prep=preparation(&source,&[[0.,3.,0.]]);let group=prepare_group(&source).unwrap();let guard=StageGuard::unlimited();
    let shared=build_shared::<4,4>(128,192,&source,&group,guard).result.unwrap();let state=solve_case_at::<4,4>(&shared,&prep,&group,guard).result.unwrap();
    let vs=build_verify_shared::<4,4,8>(&shared,&source,&group,guard).result.unwrap();
    let report=verify_state(&shared,&vs,&prep,&group,&state,guard).result.unwrap();
    assert!(report.w.iter().flatten().all(|x|x.is_zero()),"the actual verification Ŵ recovery must omit member terms");
    assert!(report.a_s.iter().flatten().all(|x|x.is_zero()),"the actual None-ledger bound must omit member terms");
    for (g,expected) in [(1,-3.),(5,-1.),(7,-3.),(11,1.)] {
        let row=prep.layout.iter().position(|r|r.id==QuantityId::Reaction(Dof::from_global(g))).unwrap();
        assert_eq!(state.recovered.values[row],Wide::<4>::from_f64(expected).unwrap());
    }
    let middle=prep.layout.iter().position(|r|r.id==QuantityId::StationAction{station:0,component:Component::Rz}).unwrap();
    assert_eq!(state.recovered.values[middle],Wide::<4>::from_f64(-0.5).unwrap());
    for end in [End::I,End::J] {let row=prep.layout.iter().position(|r|r.id==QuantityId::EndAction{member:0,end,component:Component::Uy}).unwrap();assert_eq!(state.recovered.values[row],Wide::<4>::from_f64(-3.).unwrap());}
    let mut ctx=WideContext::<4>::new(128).unwrap();let mut sum=ExactWideSum::new();let empty=RetainedLedger::empty();
    let zero=vec![Wide::<4>::ZERO;12];
    let omitted=recover(&mut ctx,&mut sum,&guard,&source,&prep.layout,&group.structure,&shared.k,&shared.members,&[],&empty,&zero).unwrap();assert!(omitted.values.iter().all(|x|x.is_zero()));
    let mut u=zero.clone();u[7]=Wide::<4>::from_f64(0.125).unwrap();let twice:Vec<_>=u.iter().map(|x|x.mul_pow2(1).unwrap()).collect();
    let a=recover(&mut ctx,&mut sum,&guard,&source,&prep.layout,&group.structure,&shared.k,&shared.members,&[],&empty,&u).unwrap();
    let b=recover(&mut ctx,&mut sum,&guard,&source,&prep.layout,&group.structure,&shared.k,&shared.members,&[],&empty,&twice).unwrap();
    for (a,b) in a.values.iter().zip(b.values){assert_eq!(a.mul_pow2(1).unwrap(),b);}
    let e=super::super::verify::formation_scale(&mut ctx,&mut sum,&guard,&source,&prep.layout,&group.structure,&vs.abar,&shared.members,&[],Some(prep.ledger().unwrap()),&zero,StageRounding::Nearest).unwrap();
    for (r,e) in state.recovered.values.iter().zip(e){if let Some(e)=e{assert!(e.cmp_value(&r.abs())!=CmpOrdering::Less);}}
}

#[test]
fn w1_loaded_maximum_station_orientation(){
    let template=member_source(false,Vec::new());
    let source=PrimitiveSource::new(SourceParts{nodes:template.nodes().to_vec(),members:template.members().to_vec(),constraints:template.constraints().to_vec(),
        stations:[0.59915,0.40085].into_iter().enumerate().map(|(i,t)|Station{id:i as u32,member:0,fraction:t}).collect(),..Default::default()}).unwrap();
    let prep=preparation(&source,&[[0.,-0.5,0.]]);let group=prepare_group(&source).unwrap();let guard=StageGuard::unlimited();
    let shared=build_shared::<4,8>(256,320,&source,&group,guard).result.unwrap();let _=solve_case_at::<4,8>(&shared,&prep,&group,guard).result.unwrap();
    let mut ctx=WideContext::<4>::new(256).unwrap();let mut sum=ExactWideSum::new();
    let h=ctx.div(&Wide::<4>::from_f64(-1.).unwrap(),&Wide::<4>::from_f64(6.).unwrap()).unwrap();
    let qi=ctx.sub(&h,&Wide::<4>::from_f64(-0.137).unwrap()).unwrap();
    let qj=ctx.sub(&Wide::<4>::from_f64(0.0613).unwrap(),&h).unwrap();
    let a=ctx.sub(&qi.mul_pow2(1).unwrap(),&qj).unwrap();let b=ctx.sub(&qj.mul_pow2(1).unwrap(),&qi).unwrap();
    let mut u=vec![Wide::<4>::ZERO;12];u[5]=ctx.div(&a,&Wide::<4>::from_f64(3.).unwrap()).unwrap();u[11]=ctx.div(&b,&Wide::<4>::from_f64(3.).unwrap()).unwrap();
    let recovered=recover(&mut ctx,&mut sum,&guard,&source,&prep.layout,&group.structure,&shared.k,&shared.members,&[],prep.ledger().unwrap(),&u).unwrap();
    let value=|id|recovered.values[prep.layout.iter().position(|r|r.id==QuantityId::StationAction{station:id,component:Component::Rz}).unwrap()];
    assert_eq!(value(0).cmp_value(&value(1)),CmpOrdering::Greater,"peak lies at .59915, not its mirrored station");
    let gap=ctx.sub(&value(0),&value(1)).unwrap();assert_eq!(gap.cmp_value(&Wide::<4>::from_f64(0.03).unwrap()),CmpOrdering::Greater);
}

#[test]
fn member_load_simply_supported_closed_controls(){
    use super::super::{origins::{RecordedInvocation,OriginCapacity},recover::End};
    let template=member_source(false,Vec::new());
    let source=PrimitiveSource::new(SourceParts{nodes:template.nodes().to_vec(),members:template.members().to_vec(),
        constraints:(0..12).filter(|g|g%6<4).map(|g|Constraint{dof:Dof::from_global(g),value:0.}).collect(),
        stations:[0.,0.25,0.5,1.].into_iter().enumerate().map(|(i,t)|Station{id:i as u32,member:0,fraction:t}).collect(),..Default::default()}).unwrap();
    let loads=MemberLoadSet::new(&source,vec![UniformMemberLoad{member:0,intensity:[0.,3.,0.],source_id:"uniform".into()}]).unwrap();
    let mut invocation=RecordedInvocation::new(u64::MAX,OriginCapacity::for_calls(&[1],&[]).unwrap()).unwrap();
    let cases=invocation.solve_member_cases(&[MemberSourceRef::new(&source,&loads).unwrap()],CaseLimit::new(u64::MAX)).unwrap();
    let owner=match &cases[0].outcome{ExecutionOutcome::Selected(s)=>s,_=>panic!("simply supported source")};
    let value=|id|owner.publish().rows.iter().find(|r|r.id==id).unwrap().value.value().unwrap();
    for(g,v)in[(5,1.),(11,-1.)]{assert_eq!(value(QuantityId::Displacement(Dof::from_global(g))),v);}
    for g in [1,7]{assert_eq!(value(QuantityId::Reaction(Dof::from_global(g))),-3.);}
    for end in [End::I,End::J]{assert_eq!(value(QuantityId::EndAction{member:0,end,component:Component::Rz}),0.);}
    for(station,shear,moment)in[(0,3.,0.),(1,1.5,-1.125),(2,0.,-1.5),(3,-3.,0.)]{
        assert_eq!(value(QuantityId::StationAction{station,component:Component::Uy}),shear);
        assert_eq!(value(QuantityId::StationAction{station,component:Component::Rz}),moment);
    }
}

#[test]
fn member_load_rhs_reads_the_declared_ledger(){
    let source=member_source(false,vec![NodalLoad{dof:Dof::from_global(7),value:1.,source_id:"nodal".into()}]);
    let prep=preparation(&source,&[[0.,3.,0.]]);let group=prepare_group(&source).unwrap();let guard=StageGuard::unlimited();
    let shared=build_shared::<4,4>(128,192,&source,&group,guard).result.unwrap();let _=solve_case_at::<4,4>(&shared,&prep,&group,guard).result.unwrap();
    let mut ctx=WideContext::<4>::new(128).unwrap();let mut sum=ExactWideSum::new();let prescribed=vec![Wide::<4>::ZERO;12];
    let rhs=reduced_rhs(&mut ctx,&mut sum,&source,&group.structure,&shared.k,prep.ledger().unwrap(),&group.ordering.free,&prescribed).unwrap();
    // Check the first solve's actual RHS, not only a final state which correct
    // residual refinement could repair after an omitted initial load.
    for(value,&g)in rhs.iter().zip(&group.ordering.free){let expected=match g{7=>4.,11=>-1.,_=>0.};assert_eq!(*value,Wide::<4>::from_f64(expected).unwrap(),"dof{g}");}
}
