#![allow(dead_code)]
use open_pipe_stress_frame_kernel::structural::retained_api::*;
const FIXTURES: &str = include_str!("/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-control-layer-8a41be/.claude/t3/f2a/projects/chirality-piping/core/solver/frame_kernel/tests/retained_k4/models.txt");
fn bits(s: &str)->f64 { f64::from_bits(u64::from_str_radix(s,16).unwrap()) }
fn n(s:&str)->u32 { s.parse().unwrap() }
fn dof(a:&str,b:&str)->Dof { Dof{node:n(a),component:Component::ALL[n(b) as usize]} }
fn parts(name:&str)->SourceParts {
 let block=FIXTURES.split(&format!("model {name}\n")).nth(1).expect(name).split("\nend").next().unwrap();
 let mut p=SourceParts::default();
 for line in block.lines() {let f:Vec<_>=line.split_whitespace().collect(); if f.is_empty(){continue}
 match f[0] {
 "node"=>p.nodes.push([bits(f[1]),bits(f[2]),bits(f[3])]),
 "member"=>p.members.push(StraightMember{id:n(f[1]),node_i:n(f[2]),node_j:n(f[3]),elastic_modulus:bits(f[4]),shear_modulus:bits(f[5]),area:bits(f[6]),second_moment_y:bits(f[7]),second_moment_z:bits(f[8]),torsion_constant:bits(f[9]),y_reference:[bits(f[10]),bits(f[11]),bits(f[12])]}),
 "spring"=>p.springs.push(Spring{id:n(f[1]),dof:dof(f[2],f[3]),stiffness:bits(f[4])}),
 "dspring"=>p.directional_springs.push(DirectionalSpring{id:n(f[1]),node:n(f[2]),kind:if f[3]=="t"{SpringKind::Translation}else{SpringKind::Rotation},direction:[bits(f[4]),bits(f[5]),bits(f[6])],stiffness:bits(f[7])}),
 "constraint"=>p.constraints.push(Constraint{dof:dof(f[1],f[2]),value:bits(f[3])}),
 "load"=>p.loads.push(NodalLoad{dof:dof(f[1],f[2]),value:bits(f[3]),source_id:f[4].into()}),
 "station"=>p.stations.push(Station{id:n(f[1]),member:n(f[2]),fraction:bits(f[3])}),
 "support"=>{let ids=|s:&str| if s=="-"{vec![]}else{s.split(',').map(n).collect()};let mut restraint=[false;6];for(i,v)in f[3].chars().enumerate(){restraint[i]=v=='1'}p.supports.push(SupportGroup{id:n(f[1]),node:n(f[2]),restrained:restraint,springs:ids(f[4]),directional_springs:ids(f[5])})},
 "expect"|"seed"=>{}, other=>panic!("unknown fixture line {other}")
 }}p
}
fn source(name:&str)->PrimitiveSource { PrimitiveSource::new(parts(name)).unwrap() }
fn limit()->CaseLimit {CaseLimit::new(u64::MAX)}
fn ctx(b:&[usize],c:&[usize])->RecordedInvocation {RecordedInvocation::new(u64::MAX,OriginCapacity::for_calls(b,c).unwrap()).unwrap()}
fn selected(c:&RecordedCase)->&RetainedSolve {match &c.outcome{ExecutionOutcome::Selected(s)=>s,o=>panic!("{o:?}")}}
fn with_run(c:RecordedKernelCombination)->RecordedCase {match c{RecordedKernelCombination::WithRun{case,..}=>case,o=>panic!("{o:?}")}}
fn count(c:&RecordedInvocation)->(usize,usize,usize,usize,usize,WorkTotal){(c.calls().len(),c.sources().len(),c.groups().len(),c.builds().len(),c.runs().len(),c.meter().checked_charged())}
fn physical(c:&RecordedCase)->&[AttemptRecord] {match &c.outcome{ExecutionOutcome::Selected(s)=>&s.evidence().attempts,ExecutionOutcome::Unresolved{attempts,..}|ExecutionOutcome::Refused{attempts,..}=>attempts}}
fn audit(c:&RecordedInvocation,cases:&[RecordedCase]) {
 for x in cases {let r=&c.runs()[x.run];let attempts=physical(x); assert_eq!(attempts.len(),r.physical_records);let mut inc=0u128;let mut cost=0u128;
 for (j,a) in attempts.iter().enumerate(){inc+=a.checked_invocation_increment().exact().unwrap() as u128;cost+=a.checked_case_charge().exact().unwrap() as u128;
 for (id,work,built,phase) in [(r.records[j].shared,a.checked_shared_work(),a.shared_built_here,BuildPhase::Shared),(r.records[j].verification_shared,a.checked_verification_shared_work(),a.verification_shared_built_here,BuildPhase::VerificationShared)]{if let Some(i)=id{let b=&c.builds()[i];assert_eq!(b.work,work);assert_eq!(b.phase,phase);assert_eq!(b.run==r.id,built);if built{assert_eq!(b.physical_record,j);}else{assert!(b.run<r.id);}}}}
 assert_eq!(cost,r.work.case().exact().unwrap()as u128);assert_eq!(inc,r.work.invocation_increment().exact().unwrap()as u128);assert_eq!(r.work.invocation_before().exact().unwrap()as u128+inc,r.work.invocation_after().exact().unwrap()as u128);
 }for(i,c)in c.calls().iter().enumerate(){assert_eq!(i,c.id);}for(i,r)in c.runs().iter().enumerate(){assert_eq!(i,r.id);}for(i,b)in c.builds().iter().enumerate(){assert_eq!(i,b.id);}
}
#[test]
fn aggregate_shapes_empty_calls_and_rejection_are_atomic() {
 let s=source("N05");let mut c=ctx(&[0,2],&[0,3]);
 assert!(c.solve_cases(&[],limit()).unwrap().is_empty());
 assert!(matches!(c.solve_combination(&[],limit()).unwrap(),RecordedKernelCombination::PreSourceRefusal{reason:CombinationReason::NoOperands,..}));
 let before=count(&c);assert!(matches!(c.solve_cases(&[s.clone(),s.clone(),s.clone()],limit()),Err(OriginError::Capacity)));assert_eq!(count(&c),before);
 let cases=c.solve_cases(&[s.clone(),s],limit()).unwrap();
 let before=count(&c);let a=selected(&cases[0]);assert!(matches!(c.solve_combination(&[(1.,a),(1.,a),(1.,a),(1.,a)],limit()),Err(OriginError::Capacity)));assert_eq!(count(&c),before);
 let combo=with_run(c.solve_combination(&[(0.,a),(1.,selected(&cases[1])),(-1.,a)],limit()).unwrap());
 assert_eq!(c.calls().iter().map(|x|x.runs.len()).collect::<Vec<_>>(),vec![0,0,2,1]);assert_eq!(c.runs().len(),3);assert_eq!(c.sources().len(),3);audit(&c,&cases);audit(&c,&[combo]);
 let before=count(&c);assert!(matches!(c.solve_cases(&[],limit()),Err(OriginError::Capacity)));assert_eq!(count(&c),before);
 // Per-call shapes are intentionally fungible within checked aggregate quotas.
 let mut c=RecordedInvocation::new(0,OriginCapacity::for_calls(&[1,1,0],&[2,0]).unwrap()).unwrap();
 let cases=c.solve_cases(&[source("N05"),source("N05")],limit()).unwrap();
 assert!(c.solve_cases(&[],limit()).unwrap().is_empty());assert!(c.solve_cases(&[],limit()).unwrap().is_empty());
 assert!(matches!(c.solve_combination(&[],limit()).unwrap(),RecordedKernelCombination::PreSourceRefusal{..}));assert!(matches!(c.solve_combination(&[],limit()).unwrap(),RecordedKernelCombination::PreSourceRefusal{..}));
 assert_eq!(c.calls().len(),5);assert_eq!(c.runs().len(),2);assert!(c.groups().is_empty()&&c.builds().is_empty());assert!(c.runs().iter().all(|r|r.phase==RunPhase::InvocationEntry&&r.group.is_none()));audit(&c,&cases);
 assert!(matches!(OriginCapacity::for_calls(&[usize::MAX],&[]),Err(OriginError::CountRange(_))));
 assert!(matches!(OriginCapacity::for_calls(&[],&[usize::MAX,1]),Err(OriginError::CountRange(_))));
}
#[test]
fn real_failure_reuse_and_budget_retry_discriminate_builds() {
 let mut c=ctx(&[3],&[]);let s=source("SKEW-K1E-300");let cases=c.solve_cases(&[s.clone(),s.clone(),s],limit()).unwrap();
 assert_eq!(c.builds().len(),3);assert_eq!(c.builds().iter().map(|b|b.slot).collect::<Vec<_>>(),vec![OriginSlot::S128,OriginSlot::S256,OriginSlot::S512]);
 assert!(c.builds().iter().all(|b|b.state==BuildState::NonbudgetFailure&&b.run==0));
 for r in &c.runs()[1..]{assert_eq!(r.work.invocation_increment().exact(),Ok(0));assert_eq!(r.records,c.runs()[0].records);assert_eq!(r.work.case(),c.runs()[0].work.case());}
 audit(&c,&cases);
 let mut c=ctx(&[3],&[]);let s=source("N05");let cases=c.solve_cases(&[s.clone(),s.clone(),s],CaseLimit::new(0)).unwrap();
 assert_eq!(c.builds().len(),3);assert!(c.builds().iter().all(|b|b.state==BuildState::BudgetFailure));
 for(i,r)in c.runs().iter().enumerate(){assert_eq!(r.records[0].shared,Some(i));assert_eq!(r.cache_before,[None;7]);assert_eq!(r.cache_after,[None;7]);assert_eq!(c.builds()[i].run,i)}audit(&c,&cases);
}
#[test]
fn clone_drop_foreign_and_legacy_cannot_mint_association() {
 let mut a=ctx(&[1],&[1]);let cases=a.solve_cases(&[source("N05")],limit()).unwrap();let owner=selected(&cases[0]).clone();drop(cases);
 let combo=with_run(a.solve_combination(&[(1.,&owner)],limit()).unwrap());assert_eq!(a.groups()[1].imports.iter().flatten().count(),3);audit(&a,&[combo]);drop(a);
 let mut b=ctx(&[1],&[1,1,1]);let other=b.solve_cases(&[source("N05")],limit()).unwrap();assert_eq!(selected(&other[0]).evidence().source_encoding,owner.evidence().source_encoding);
 assert!(matches!(b.solve_combination(&[(1.,&owner)],limit()).unwrap(),RecordedKernelCombination::OriginRefusal{error:OriginError::MissingSelectedOrigin{operand:0},..}));
 let mut meter=InvocationMeter::new(u64::MAX);let legacy=match solve_case(source("N05"),limit(),&mut meter){CaseOutcome::Selected(s)=>s,o=>panic!("{o:?}")};
 assert!(matches!(b.solve_combination(&[(1.,&legacy)],limit()).unwrap(),RecordedKernelCombination::OriginRefusal{..}));
 assert!(matches!(b.solve_combination(&[(f64::NAN,&owner)],limit()).unwrap(),RecordedKernelCombination::PreSourceRefusal{reason:CombinationReason::NoOperands,..}));
 assert_eq!(b.runs().len(),1);for c in &b.calls()[1..]{assert!(c.sources.is_empty()&&c.runs.is_empty());assert_eq!(c.invocation_before,c.invocation_after)}
 // The owned solve still executes through the old API after origin context drop.
 assert!(matches!(RetainedCombination::solve(&[(1.,&owner)],limit(),&mut meter),CombinationOutcome::Selected(_)));
}
#[test]
fn actual_snapshot_union_unavailable_combined_source_and_no_backfill() {
 let mut zero=parts("SKEW6-K1E-12");zero.loads.clear();let low=PrimitiveSource::new(zero).unwrap();let high=source("SKEW6-K1E-12");
 let mut c=ctx(&[2,1],&[2,2,2]);let cases=c.solve_cases(&[low,high.clone()],limit()).unwrap();let independent=c.solve_cases(&[high],limit()).unwrap();
 assert_eq!(selected(&cases[0]).selected_precision(),128);assert_eq!(selected(&cases[1]).selected_precision(),256);
 let early=c.runs()[0].cache_after;assert!(early[2].is_none()&&c.runs()[1].cache_after[2].is_some());
 let terms=[(2.,selected(&cases[0])),(-0.5,selected(&independent[0]))];
 let unavailable=with_run(c.solve_combination(&terms,CaseLimit::new(0)).unwrap());assert!(!matches!(unavailable.outcome,ExecutionOutcome::Selected(_)));
 let r=&c.runs()[unavailable.run];let origin=&c.sources()[r.source];let unavailable_identity=origin.identity.clone();let unavailable_ledger=origin.combination_ledger.clone().unwrap();assert!(unavailable_identity.starts_with(b"K4CMB"));assert!(unavailable_ledger.starts_with(b"K4LED"));
 for i in c.groups()[r.group.unwrap()].imports.iter().flatten(){let slot=OriginSlot::ALL.iter().position(|s|*s==i.slot).unwrap();let expected=if early[slot].is_some(){0}else{2};assert_eq!(i.selected_run,expected);assert_eq!(Some(i.build),c.runs()[expected].cache_after[slot]);assert_eq!(c.builds()[i.build].run,expected);}
 let resolved=with_run(c.solve_combination(&terms,limit()).unwrap());let actual=selected(&resolved);assert_eq!(actual.evidence().source_encoding,unavailable_identity);assert_eq!(actual.evidence().ledger_encoding,unavailable_ledger);assert_eq!(c.runs()[0].cache_after,early);
 let reversed=[terms[1],terms[0]];let reverse=with_run(c.solve_combination(&reversed,limit()).unwrap());assert_ne!(c.sources()[c.runs()[reverse.run].source].identity,unavailable_identity);assert_eq!(c.runs()[reverse.run].cache_before,c.runs()[2].cache_after);
 assert!(c.groups()[c.runs()[reverse.run].group.unwrap()].imports.iter().flatten().all(|i|i.selected_run==2&&i.operand_index==0));audit(&c,&cases);audit(&c,&independent);audit(&c,&[unavailable,resolved,reverse]);
}
