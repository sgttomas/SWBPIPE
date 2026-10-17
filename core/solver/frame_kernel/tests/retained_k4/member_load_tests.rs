use super::*;
use super::super::{adaptive::{StageGuard, CasePrep, CaseLimit, ExecutionOutcome},
    source::*, ledger::RetainedLedger, wide::multi::AttemptWork};
#[allow(dead_code)]
#[path = "support.rs"] mod support;
use support::*;

fn source(xi: [f64;3], xj: [f64;3]) -> PrimitiveSource {
    PrimitiveSource::new(SourceParts { nodes: vec![xi,xj], members: vec![StraightMember {
        id:0,node_i:0,node_j:1,elastic_modulus:1.,shear_modulus:1.,area:1.,
        second_moment_y:1.,second_moment_z:1.,torsion_constant:1.,y_reference:[0.,1.,0.],
    }], ..Default::default() }).unwrap()
}
fn lme(ctx:&WideContext<16>,sum:&ExactWideSum)->u64 {
    let mut work=AttemptWork::default();work.record(ctx);
    work.checked_lme().add(sum.work().checked_lme()).exact().unwrap()
}
#[test]
fn declared_loads_match_python_bit_for_bit() {
    let lines:Vec<_>=include_str!("declared.txt").lines().collect();
    let mut at=0;let mut count=0;
    while at<lines.len() {
        let f:Vec<_>=lines[at].split_whitespace().collect();assert_eq!(f[0],"case");
        let xi=std::array::from_fn(|i|f64_bits(f[2+i]));let xj=std::array::from_fn(|i|f64_bits(f[5+i]));
        let base=source(xi,xj);
        let mut parts=SourceParts { nodes:base.nodes().to_vec(), members:base.members().to_vec(), ..Default::default() };
        let name=f[1];let mut loads=Vec::new();let mut expected=Vec::new();at+=1;
        while lines[at].starts_with("nodal ") {
            let f:Vec<_>=lines[at].split_whitespace().collect();
            parts.loads.push(NodalLoad{dof:Dof::from_global(f[1].parse().unwrap()),value:f64_bits(f[2]),source_id:format!("n{at}")});at+=1;
        }
        while lines[at].starts_with("load ") {
            let f:Vec<_>=lines[at].split_whitespace().collect();loads.push(UniformMemberLoad{
                member:0,intensity:std::array::from_fn(|i|f64_bits(f[i+1])),source_id:format!("load:{:03}",loads.len())});at+=1;
            let f:Vec<_>=lines[at].split_whitespace().collect();assert_eq!(f[0],"entries");expected.push(f[1..].to_vec());at+=1;
        }
        let source=PrimitiveSource::new(parts).unwrap();let set=MemberLoadSet::new(&source,loads).unwrap();
        let mut ctx=WideContext::<16>::new(1024).unwrap();let mut sum=ExactWideSum::new();
        let declared=form(&source,set.loads(),&mut ctx,&mut sum,|_,_|Ok(())).unwrap();
        for (load,want) in declared.iter().zip(expected) {for (v,w) in load.entries.iter().zip(want) {assert_eq!(tok(v),w,"{name}");}}
        let nodal=RetainedLedger::from_source(&source).unwrap();
        let ledger=RetainedLedger::with_declared(&nodal,declared,&mut sum).unwrap();
        let f:Vec<_>=lines[at].split_whitespace().collect();assert_eq!(f[0],"ledger");
        assert_eq!(ledger.encoding(),f[1].as_bytes().chunks_exact(2).map(|b|u8::from_str_radix(std::str::from_utf8(b).unwrap(),16).unwrap()).collect::<Vec<_>>(),"{name}");
        at+=1;assert_eq!(lines[at],"end");at+=1;count+=1;
    }
    assert_eq!(count,16);
}

#[test]
fn member_load_d1_cap_checkpoint_work_is_bounded() {
    let source=source([-1e300,1e-300,0.],[1e300,0.,1e-300]);
    let loads=(0..128).map(|i|UniformMemberLoad{member:0,intensity:[1e300,1e-300,1.],source_id:format!("w{i:03}")}).collect();
    let set=MemberLoadSet::new(&source,loads).unwrap();let mut ctx=WideContext::<16>::new(1024).unwrap();let mut sum=ExactWideSum::new();
    let mut checkpoints=Vec::new();
    let declared=form(&source,set.loads(),&mut ctx,&mut sum,|c,s|{checkpoints.push(lme(c,s));Ok(())}).unwrap();
    assert_eq!(checkpoints.len(),1);assert!(checkpoints[0]<=2_365_094,"{}",checkpoints[0]);
    let before=lme(&ctx,&sum);let nodal=RetainedLedger::from_source(&source).unwrap();
    let _=RetainedLedger::with_declared(&nodal,declared,&mut sum).unwrap();
    assert!(lme(&ctx,&sum)-before<=786_432);
    assert!(lme(&ctx,&sum)<=4_474_048);

    let base=self::source([0.;3],[1.,0.,0.]);let template=base.members()[0];
    let nodes:Vec<_>=(0..32).map(|i|[i as f64*1e290,(i%3) as f64*1e290,(i%5) as f64*1e-300]).collect();
    let members=(0..32).map(|i|StraightMember{id:i,node_i:if i==31{0}else{i},node_j:if i==31{31}else{i+1},..template}).collect();
    let source=PrimitiveSource::new(SourceParts{nodes,members,..Default::default()}).unwrap();
    let loads=(0..128).map(|i|UniformMemberLoad{member:i/4,intensity:[1e300,1e-300,1.],source_id:format!("w{i:03}")}).collect();
    let set=MemberLoadSet::new(&source,loads).unwrap();let mut ctx=WideContext::<16>::new(1024).unwrap();let mut sum=ExactWideSum::new();let mut prior=0;
    let declared=form(&source,set.loads(),&mut ctx,&mut sum,|c,s|{let total=lme(c,s);assert!(total-prior<=2_365_094);prior=total;Ok(())}).unwrap();
    let before=lme(&ctx,&sum);let nodal=RetainedLedger::from_source(&source).unwrap();
    let ledger=RetainedLedger::with_declared(&nodal,declared,&mut sum).unwrap();
    assert_eq!(u32::from_le_bytes(ledger.encoding()[6..10].try_into().unwrap()),192);
    assert!(lme(&ctx,&sum)-before<=786_432);assert!(lme(&ctx,&sum)<=4_474_048);
}

fn dyad(value:&str)->(bool,Vec<u64>,i64) {
    let (m,e)=value[1..].split_once('@').unwrap();
    (value.starts_with('-'),hex_limbs(m),e.parse().unwrap())
}
// Test-only unbounded integer comparison. The reference intervals can have
// tiny cancellation tails spanning beyond the producer's intentional8128-bit
// sum cap; this compares exact dyadics without weakening the69uE predicate.
fn exact_nonpositive(terms:&[(bool,Vec<u64>,i64)])->bool {
    let anchor=terms.iter().filter(|(_,m,_)|m.iter().any(|x|*x!=0)).map(|t|t.2).min().unwrap_or(0);
    let mut signs=[Vec::<u64>::new(),Vec::<u64>::new()];
    for (negative,mag,exponent) in terms {
        if mag.iter().all(|x|*x==0){continue;}
        let shift=usize::try_from(exponent-anchor).unwrap();let word=shift/64;let bit=shift%64;
        let out=&mut signs[usize::from(*negative)];out.resize(out.len().max(word+mag.len()+2),0);
        let mut carry=0u128;
        for (i,limb) in mag.iter().enumerate(){
            let value=(u128::from(*limb)<<bit)+carry;
            let at=word+i;let total=u128::from(out[at])+u128::from(value as u64);
            out[at]=total as u64;carry=(value>>64)+(total>>64);
        }
        let mut at=word+mag.len();while carry!=0{let total=u128::from(out[at])+carry;out[at]=total as u64;carry=total>>64;at+=1;}
    }
    let size=signs[0].len().max(signs[1].len());
    for i in(0..size).rev(){let a=signs[0].get(i).copied().unwrap_or(0);let b=signs[1].get(i).copied().unwrap_or(0);if a!=b{return a<b;}}
    true
}
fn wide_dyad(value:Wide<16>,negate:bool,factor:u64)->(bool,Vec<u64>,i64){
    let (negative,exponent,limbs)=value.parts();let mut mag=Vec::with_capacity(17);let mut carry=0u128;
    for limb in limbs{let v=u128::from(limb)*u128::from(factor)+carry;mag.push(v as u64);carry=v>>64;}if carry!=0{mag.push(carry as u64);}
    (negative!=negate,mag,exponent-1023)
}
#[allow(non_snake_case)]
#[test]
fn member_load_recovery_error_is_bounded_by_69uE() {
    use super::super::{assemble::{Structure,form_members}, recover::{recover,layout},
        verify::{formation_scale,StageRounding,member_g_scope}};
    let lines:Vec<_>=include_str!("member_recovery.txt").lines().collect();
    let mut at=0;let mut cases=0;let mut error_rows=0;let mut above_rows=0;
    while at<lines.len() {
        let f:Vec<_>=lines[at].split_whitespace().collect();let name=f[1];let p:u32=f[2].parse().unwrap();
        let v:Vec<_>=f[3..].iter().map(|s|f64_bits(s)).collect();
        let mut parts=SourceParts {nodes:vec![v[0..3].try_into().unwrap(),v[3..6].try_into().unwrap()],
            members:vec![StraightMember{id:0,node_i:0,node_j:1,elastic_modulus:2.,shear_modulus:5.,area:3.,
                second_moment_y:11.,second_moment_z:7.,torsion_constant:2.,y_reference:v[6..9].try_into().unwrap()}],
            stations:[0.,0.25,1./3.,1.-2f64.powi(-52),1.].into_iter().enumerate().map(|(i,t)|Station{id:i as u32,member:0,fraction:t}).collect(),
            ..Default::default()};
        if name.starts_with("fixed")||name.starts_with("cancel") {parts.constraints=(0..12).map(|g|Constraint{dof:Dof::from_global(g),value:0.}).collect();}
        let source=PrimitiveSource::new(parts).unwrap();at+=1;
        let mut loads=Vec::new();
        while lines[at].starts_with("load ") {let f:Vec<_>=lines[at].split_whitespace().collect();
            loads.push(UniformMemberLoad{member:0,intensity:std::array::from_fn(|i|f64_bits(f[i+1])),source_id:format!("w{}",loads.len())});at+=1;}
        let set=MemberLoadSet::new(&source,loads).unwrap();
        let mut dc=WideContext::<16>::new(1024).unwrap();let mut ds=ExactWideSum::new();
        let declared=form(&source,set.loads(),&mut dc,&mut ds,|_,_|Ok(())).unwrap();
        let ledger=RetainedLedger::with_declared(&RetainedLedger::from_source(&source).unwrap(),declared,&mut ds).unwrap();
        let empty=RetainedLedger::empty();let omitted=name.starts_with("none")||name.starts_with("empty");
        let ledger=if omitted{&empty}else{&ledger};
        let mut ctx=WideContext::<16>::new(p).unwrap();let mut sum=ExactWideSum::new();let guard=StageGuard::unlimited();
        let members=form_members(&mut ctx,&mut sum,&guard,&source).unwrap();
        let structure=Structure::new(&source).unwrap();let k=vec![Wide::<16>::ZERO;structure.pattern.entry_count()];
        let u:Vec<_>=v[9..].iter().map(|x|Wide::<16>::from_f64(*x).unwrap()).collect();
        let layout=layout(&source);
        let recovered=recover(&mut ctx,&mut sum,&guard,&source,&layout,&structure,&k,&members,&[],ledger,&u).unwrap();
        let abs_u:Vec<_>=u.iter().map(|x|x.abs()).collect();
        let e=formation_scale(&mut ctx,&mut sum,&guard,&source,&layout,&structure,&k,&members,&[],
            if name.starts_with("none"){None}else{Some(ledger)},&abs_u,StageRounding::Nearest).unwrap();
        let active=!omitted&&!name.starts_with("zero");
        assert_eq!(member_g_scope(Some(ledger),0,false),active,"{name}");
        assert!(member_g_scope(Some(ledger),0,true),"prior nodal applicability {name}");
        if name.starts_with("g_boundary"){assert_eq!(members[0].g_exp,p-16,"{name}");}
        if name.starts_with("g_above"){assert!(members[0].g_exp>p-16,"{name}");}
        let test_error=!name.starts_with("g_above");
        for row in 0..42 {
            let f:Vec<_>=lines[at].split_whitespace().collect();assert_eq!(f[0],"row");assert_eq!(f[1].parse::<usize>().unwrap(),row);
            let index=14+row;let actual=recovered.values[index];let bound=e[index].unwrap().mul_pow2(-i64::from(p)).unwrap();
            if test_error {
                error_rows+=1;
                for endpoint in [f[2],f[3]] {let (neg,mag,exp)=dyad(endpoint);
                    for direction in [false,true] {
                        let terms=[(neg!=direction,mag.clone(),exp),wide_dyad(actual,!direction,1),wide_dyad(bound,true,69)];
                        assert!(exact_nonpositive(&terms),"actual formation error exceeded69uE: {name} row{row} endpoint{endpoint}");
                    }
                }
            } else {above_rows+=1;}
            at+=1;
        }
        assert_eq!(lines[at],"end");at+=1;cases+=1;
    }
    assert_eq!(cases,40);assert_eq!(error_rows,1512);assert_eq!(above_rows,168);
}

fn cantilever()->(PrimitiveSource,MemberLoadSet){
    let base=source([0.;3],[2.,0.,0.]);
    let source=PrimitiveSource::new(SourceParts{nodes:base.nodes().to_vec(),members:base.members().to_vec(),
        constraints:(0..6).map(|g|Constraint{dof:Dof::from_global(g),value:0.}).collect(),
        stations:[0.,0.25,0.5,1.].into_iter().enumerate().map(|(i,fraction)|Station{id:i as u32,member:0,fraction}).collect(),
        ..Default::default()}).unwrap();
    let loads=MemberLoadSet::new(&source,vec![UniformMemberLoad{member:0,intensity:[0.,3.,0.],source_id:"w".into()}]).unwrap();(source,loads)
}
fn recorded(source:&PrimitiveSource,loads:&MemberLoadSet,case_limit:u64,invocation_limit:u64)
    ->(super::super::origins::RecordedInvocation,super::super::origins::RecordedCase){
    use super::super::origins::{OriginCapacity,RecordedInvocation};
    let mut invocation=RecordedInvocation::new(invocation_limit,OriginCapacity::for_calls(&[1],&[]).unwrap()).unwrap();
    let case=invocation.solve_member_cases(&[MemberSourceRef::new(source,loads).unwrap()],CaseLimit::new(case_limit)).unwrap().remove(0);
    (invocation,case)
}
#[test]
fn member_load_cantilever_tip_and_endpoint_station_bits(){
    use super::super::recover::QuantityId as Q;
    let (source,loads)=cantilever();let (invocation,case)=recorded(&source,&loads,u64::MAX,u64::MAX);
    let ExecutionOutcome::Selected(owner)=case.outcome else{panic!("{:?}",case.outcome)};
    let value=|id|owner.publish().rows.iter().find(|r|r.id==id).unwrap().value.value().unwrap();
    assert_eq!(value(Q::Displacement(Dof::from_global(7))),6.);
    assert_eq!(value(Q::Displacement(Dof::from_global(11))),4.);
    for c in Component::ALL{
        let i=value(Q::EndAction{member:0,end:super::super::recover::End::I,component:c});
        let j=value(Q::EndAction{member:0,end:super::super::recover::End::J,component:c});
        let station0=value(Q::StationAction{station:0,component:c});let station1=value(Q::StationAction{station:3,component:c});
        assert_eq!(station0.to_bits(),(if i==0.{0.}else{-i}).to_bits());assert_eq!(station1.to_bits(),j.to_bits());
    }
    assert_eq!(value(Q::StationAction{station:2,component:Component::Rz}),1.5);
    let prep=CasePrep::new_member(MemberSourceRef::new(&source,&loads).unwrap()).unwrap();assert_eq!(prep.try_ledger().unwrap_err(),super::super::ledger::LedgerRefusal::NotFormed);
    assert!(owner.prep.try_ledger().is_ok());
    assert_eq!(owner.evidence().source_encoding,MemberSourceRef::new(&source,&loads).unwrap().encoding());
    assert_eq!(invocation.sources()[0].identity,owner.evidence().source_encoding);
}
#[test]
fn member_load_checkpoint_equality_and_exhaustion_are_metered_once(){
    use super::super::adaptive::{AttemptStop,BudgetScope,UnresolvedReason};
    let (source,loads)=cantilever();let (_,baseline)=recorded(&source,&loads,u64::MAX,u64::MAX);
    let ExecutionOutcome::Selected(owner)=baseline.outcome else{panic!("baseline not selected")};
    let shared=owner.evidence().attempts[0].shared_work;
    let mut ctx=WideContext::<16>::new(1024).unwrap();let mut sum=ExactWideSum::new();let mut member=0;
    let declared=form(&source,loads.loads(),&mut ctx,&mut sum,|c,s|{member=lme(c,s);Ok(())}).unwrap();
    let nodal=RetainedLedger::from_source(&source).unwrap();let _=RetainedLedger::with_declared(&nodal,declared,&mut sum).unwrap();let net=lme(&ctx,&sum);
    assert!(net>member);
    for (limit,expected_rhs) in [(shared+member-1,member),(shared+member,net),(shared+net-1,net)]{
        let (invocation,case)=recorded(&source,&loads,limit,u64::MAX);
        let ExecutionOutcome::Unresolved{reason,attempts,..}=case.outcome else{panic!("checkpoint did not refuse")};
        assert_eq!(reason,UnresolvedReason::Budget(BudgetScope::Case));assert_eq!(attempts.len(),1);
        let attempt=&attempts[0];assert_eq!(attempt.stages.rhs,expected_rhs);assert_eq!(attempt.stages.solve,0);
        assert_eq!(attempt.checked_own_work().exact().unwrap(),expected_rhs);
        assert_eq!(attempt.stages.checked_total().exact().unwrap(),expected_rhs);
        let work=&invocation.runs()[case.run].work;
        assert_eq!(work.case().exact().unwrap(),shared+expected_rhs);
        assert_eq!(work.invocation_increment().exact().unwrap(),shared+expected_rhs);
        assert_eq!(invocation.meter().checked_charged().exact().unwrap(),shared+expected_rhs);
    }
    let (invocation,case)=recorded(&source,&loads,u64::MAX,shared+member-1);
    let ExecutionOutcome::Unresolved{reason,attempts,..}=case.outcome else{panic!("invocation checkpoint did not refuse")};
    assert_eq!(reason,UnresolvedReason::Budget(BudgetScope::Invocation));assert_eq!(attempts[0].stages.rhs,member);
    assert_eq!(invocation.meter().checked_charged().exact().unwrap(),shared+member);
    // Equality at the explicit net checkpoint is admitted. Subsequent ordinary
    // rhs/solve work may exhaust at its established later checkpoint.
    let (_,case)=recorded(&source,&loads,shared+net,u64::MAX);
    let ExecutionOutcome::Unresolved{attempts,..}=case.outcome else{panic!("bounded solve expected")};
    assert!(attempts[0].stages.solve>0);
    let guard=StageGuard::with_case_room(net);assert!(guard.test(super::super::work::WorkTotal::exact_count(net)).is_ok());
    assert_eq!(guard.test(super::super::work::WorkTotal::exact_count(net+1)).unwrap_err(),AttemptStop::Budget(BudgetScope::Case));
}
#[test]
fn combination_with_member_load_operand_is_refused_by_name(){
    use super::super::{combine::{validate_preps,CombinationReason},adaptive::CombinationPreparationError};
    let (source,_)=cantilever();
    let zero=MemberLoadSet::new(&source,vec![UniformMemberLoad{member:0,intensity:[0.;3],source_id:"zero".into()}]).unwrap();
    let prep=CasePrep::new_member(MemberSourceRef::new(&source,&zero).unwrap()).unwrap();
    let operands=[(1.,&prep)];
    assert_eq!(validate_preps(&operands),Err(CombinationReason::OperandMemberLoads));
    assert_eq!(RetainedLedger::combined(&operands).unwrap_err(),super::super::ledger::LedgerRefusal::MemberLoads);
    assert!(matches!(CasePrep::combination(&operands),Err(CombinationPreparationError::Ledger(super::super::ledger::LedgerRefusal::MemberLoads))));
    let mut nested=CasePrep::new_member(MemberSourceRef::new(&source,&zero).unwrap()).unwrap();nested.factors.push(1.);
    assert_eq!(validate_preps(&[(1.,&nested)]),Err(CombinationReason::NestedCombination));
    // Member refusal precedes the mismatched stiffness/layout comparison.
    let foreign=CasePrep::new(self::source([0.;3],[3.,0.,0.])).unwrap();
    assert_eq!(validate_preps(&[(1.,&prep),(1.,&foreign)]),Err(CombinationReason::OperandMemberLoads));
}
#[test]
fn member_station_single_round_and_endpoint_bits(){
    use super::super::{assemble::{Structure,form_members},recover::{recover,layout}};
    let base=source([0.;3],[1.,0.,0.]);
    let source=PrimitiveSource::new(SourceParts{nodes:base.nodes().to_vec(),members:base.members().to_vec(),
        stations:[0.,0.75,1.].into_iter().enumerate().map(|(id,fraction)|Station{id:id as u32,member:0,fraction}).collect(),..Default::default()}).unwrap();
    let set=MemberLoadSet::new(&source,vec![UniformMemberLoad{member:0,intensity:[0.,-4.,0.],source_id:"w".into()}]).unwrap();
    let mut dc=WideContext::<16>::new(1024).unwrap();let mut sum=ExactWideSum::new();
    let declared=form(&source,set.loads(),&mut dc,&mut sum,|_,_|Ok(())).unwrap();
    let ledger=RetainedLedger::with_declared(&RetainedLedger::from_source(&source).unwrap(),declared,&mut sum).unwrap();
    let mut ctx=WideContext::<16>::new(128).unwrap();let guard=StageGuard::unlimited();
    let operators=form_members(&mut ctx,&mut sum,&guard,&source).unwrap();let structure=Structure::new(&source).unwrap();
    let k=vec![Wide::<16>::ZERO;structure.pattern.entry_count()];let mut u=vec![Wide::<16>::ZERO;12];
    u[5]=ctx.div(&Wide::<16>::ONE,&Wide::<16>::from_f64(6.).unwrap()).unwrap();
    let result=recover(&mut ctx,&mut sum,&guard,&source,&layout(&source),&structure,&k,&operators,&[],&ledger,&u).unwrap();
    let end=&result.values[14..26];let stations=&result.values[26..];
    for c in 0..6{assert_eq!(stations[c],if end[c].is_zero(){Wide::<16>::ZERO}else{end[c].neg()});assert_eq!(stations[12+c],end[6+c]);}
    let interior=stations[7];
    assert_eq!(interior,Wide::<16>::ONE.mul_pow2(-129).unwrap().neg());
    // The tempting corrected-j-end-first expression loses this exact residual.
    sum.clear();sum.add_wide(&end[7],false).unwrap();sum.add_binary64(-1.,false).unwrap();
    let rounded_end_first=sum.round(&mut ctx).unwrap();assert!(rounded_end_first.is_zero());assert_ne!(interior,rounded_end_first);
}
