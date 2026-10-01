//! Inert until a separate ROOT runtime grant. One source, one ordinary public solve.
use open_pipe_stress_frame_kernel::structural::retained_api::*;

struct Spec {
    id: &'static str,
    length: u64,
    area: u64,
    torsion: u64,
    spring: (usize, u64),
    free: &'static [usize],
    loads: &'static [(usize, u64, &'static str)],
}
include!("cases.rs");

fn key(id: QuantityId) -> String {
    match id {
        QuantityId::Displacement(d) => format!("D:{}", d.global()),
        QuantityId::DisplacementMagnitude(n) => format!("M:{n}"),
        QuantityId::EndAction { member, end, component } =>
            format!("E:{member}:{end:?}:{}", component.index()),
        QuantityId::SpringAction { spring, component } =>
            format!("S:{spring}:{}", component.index()),
        QuantityId::Reaction(d) => format!("R:{}", d.global()),
        _ => panic!("The sealed matrix contains no other quantity kind"),
    }
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

fn optional_bits(value: Option<f64>) -> String {
    value.map(|v| format!("{:016x}",v.to_bits())).unwrap_or_else(|| "none".into())
}

fn attempts(items: &[AttemptRecord]) {
    for (i,a) in items.iter().enumerate() {
        println!("ATTEMPT\t{i}\t{}\t{:?}\t{:?}\t{}\t{}\t{}\t{}\t{}",
            a.precision,a.role,a.outcome,a.residual_basis,a.corrections,
            optional_bits(a.pivot_margin_min),optional_bits(a.rcond),optional_bits(a.residual_worst));
        println!("ATTEMPT_WORK\t{i}\t{:?}\t{:?}\t{:?}\t{:?}",a.work,a.stages,a.shared_stages,a.storage);
        println!("ATTEMPT_FLAGS\t{i}\t{:?}\t{:?}\t{}\t{}\t{}\t{}\t{}\t{}",
            a.gate,a.bound_refusals,a.shared_work,a.shared_built_here,a.stop_rule_work,
            a.verification_work,a.verification_shared_work,a.verification_shared_built_here);
        if let Some(v) = &a.verification {
            println!("VERIFY_FLAGS\t{i}\t{}\t{}\t{:?}\t{}\t{:?}",
                v.data_blocks,v.shift_factorizations,v.uc_missing,v.g_max,v.g_violation);
            for (body,res) in v.resolution.iter().enumerate() {
                println!("VERIFY_RESOLUTION\t{i}\t{body}\t{:016x}\t{:016x}",res[0].to_bits(),res[1].to_bits());
            }
            for (body,t) in v.theta.iter().enumerate() {
                println!("VERIFY_THETA\t{i}\t{body}\t{:016x}",t.to_bits());
            }
            for (body,b) in v.bound.iter().enumerate() {
                println!("VERIFY_BOUND\t{i}\t{body}\t{}",optional_bits(*b));
            }
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    assert_eq!(args.len(),5,"usage: a1_public_probe CASE CASE_LME INVOCATION_LME B|C (separate runtime grant required)");
    let spec = CASES.iter().find(|s|s.id==args[1]).expect("sealed case ID only");
    assert_eq!(&spec.id[..1],args[4],"B first; C requires the extension checkpoint and new admission");
    let case_lme=args[2].parse::<u64>().expect("explicit CaseLimit");
    let invocation_lme=args[3].parse::<u64>().expect("explicit InvocationMeter");
    assert!(case_lme>0 && invocation_lme>0);
    assert!(std::env::var_os("FK_SEEDED_FAULT").is_none(),"seeded-fault environment must be absent");
    println!("FORMAT\ta1-public-tsv-v1");
    println!("CASE\t{}",spec.id);
    println!("SOURCE_COMMIT\t3bddc2b05f6106e969c7cf43373b230845c7cc66");
    println!("LIMITS\t{case_lme}\t{invocation_lme}");
    let one=f64::from_bits(0x3ff0000000000000);
    let zero=f64::from_bits(0);
    let mut parts=SourceParts::default();
    parts.nodes=vec![[zero;3],[f64::from_bits(spec.length),zero,zero]];
    parts.members.push(StraightMember { id:1,node_i:0,node_j:1,elastic_modulus:one,
        shear_modulus:one,area:f64::from_bits(spec.area),second_moment_y:one,
        second_moment_z:one,torsion_constant:f64::from_bits(spec.torsion),
        y_reference:[zero,one,zero] });
    for g in 0..12 {
        if !spec.free.contains(&g) {
            parts.constraints.push(Constraint { dof:Dof::from_global(g),value:zero });
        }
    }
    if spec.spring.1!=0 {
        parts.springs.push(Spring { id:1,dof:Dof::from_global(spec.spring.0),
            stiffness:f64::from_bits(spec.spring.1) });
    }
    for &(g,bits,id) in spec.loads {
        parts.loads.push(NodalLoad { dof:Dof::from_global(g),value:f64::from_bits(bits),source_id:id.into() });
    }
    let source=match PrimitiveSource::new(parts) {
        Ok(s)=>s,
        Err(e)=>{println!("STATUS\tSourceRefused");println!("SOURCE_REFUSAL\t{e:?}");return;}
    };
    println!("SOURCE_ENCODING\t{}",hex(&source.encoding()));
    println!("STIFFNESS_ENCODING\t{}",hex(&source.stiffness_encoding()));
    for m in layout(&source) {
        println!("LAYOUT\t{}\t{:?}\t{}\t{}",key(m.id),m.kind,m.body,m.input_derived);
    }
    // Independently known full member structural block; this public helper
    // observes its RCM result but does not expose or replace the internal factor.
    let n=spec.free.len();
    let adjacency:Vec<Vec<usize>>=(0..n).map(|i|(0..n).filter(|&j|j!=i).collect()).collect();
    println!("PUBLIC_RCM_ON_DERIVED_PATTERN\t{:?}",reverse_cuthill_mckee(&adjacency));
    let mut meter=InvocationMeter::new(invocation_lme);
    let result=solve_case(source,CaseLimit::new(case_lme),&mut meter);
    println!("METER\t{}\t{}\t{}",meter.charged(),meter.limit(),meter.exhausted());
    match result {
        CaseOutcome::Selected(s)=>{
            println!("STATUS\tSelected");
            let e=s.evidence();
            println!("SELECTED\t{}\t{}",e.selected_precision,e.verification_precision);
            println!("IDENTITY\t{}\t{}\t{:016x}",e.method,e.policy,e.floor_ratio_bits);
            for r in &s.publish().rows {
                let (cls,bound)=match r.class {
                    RowClass::InputDerived=>("InputDerived",String::from("none")),
                    RowClass::RelativeVerified=>("RelativeVerified",String::from("none")),
                    RowClass::AbsoluteVerified {bound_bits}=>("AbsoluteVerified",format!("{bound_bits:016x}")),
                    RowClass::Unpublishable=>("Unpublishable",String::from("none")),
                };
                let outcome=match r.value {
                    Binary64Outcome::Normal(_) | Binary64Outcome::Subnormal {..} => "Value",
                    Binary64Outcome::Underflow {..} => "Underflow",
                    Binary64Outcome::Overflow {..} => "Overflow",
                };
                println!("ROW\t{}\t{:?}\t{outcome}\t{}\t{cls}\t{bound}",key(r.id),r.kind,optional_bits(r.value.value()));
                println!("ROW_OUTCOME_DETAIL\t{}\t{:?}",key(r.id),r.value);
                if let Binary64Outcome::Subnormal { relative_precision, .. }=r.value {
                    println!("SUBNORMAL_RELATIVE_PRECISION\t{}\t{:016x}",key(r.id),relative_precision.to_bits());
                }
            }
            for &(body,kind,bits) in &s.publish().body_scales {
                assert_eq!(body,0); println!("SCALE\t{kind:?}\t{bits:016x}");
            }
            if let Some(floor)=&e.floor {
                for &(body,fo,mo) in floor {
                    assert_eq!(body,0); println!("FLOOR\tForce\t{fo:016x}");println!("FLOOR\tMoment\t{mo:016x}");
                }
            }
            for (label,values) in [("STOP_RULE",&e.stop_rule),("ESTIMATE",&e.verification_estimate),("CHARGE",&e.verification_charge)] {
                for &(body,kind,value) in values { println!("{label}\t{body}\t{kind:?}\t{:016x}",value.to_bits()); }
            }
            for &(body,value) in &e.theta { println!("THETA\t{body}\t{:016x}",value.to_bits()); }
            for &(body,bits) in &e.certified_bound { println!("CERTIFIED_BOUND\t{body}\t{bits:016x}"); }
            for &(body,fo,mo) in &e.resolution_scale { println!("RESOLUTION\t{body}\t{fo:016x}\t{mo:016x}"); }
            println!("FINAL_SUMMARY\t{:016x}\t{:016x}\t{:016x}\t{}",e.pivot_margin_min.to_bits(),e.rcond.to_bits(),e.residual_worst.to_bits(),e.corrections);
            println!("SOURCE_ENCODING_SELECTED\t{}",hex(&e.source_encoding));
            println!("LEDGER_ENCODING\t{}",hex(&e.ledger_encoding));
            println!("STATE_ENCODING\t{}",hex(&e.retained_state_encoding));
            println!("GEOMETRY\t{:?}",e.geometry);
            println!("CLASS_LISTS\t{:?}\t{:?}\t{:?}\t{:?}",e.input_derived_dofs,e.absolute_verified,e.not_covered,e.unpublishable);
            attempts(&e.attempts);
        },
        CaseOutcome::Refused { refusal,geometry }=>{
            println!("STATUS\tRefused");println!("REFUSAL\t{refusal:?}");println!("GEOMETRY\t{geometry:?}");
        },
        CaseOutcome::Unresolved { reason,attempts:records,geometry }=>{
            println!("STATUS\tUnresolved");println!("UNRESOLVED\t{reason:?}");println!("GEOMETRY\t{geometry:?}");attempts(&records);
        },
    }
}
