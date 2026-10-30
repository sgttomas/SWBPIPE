//! Actual reader arithmetic allocations, independently of producer arithmetic.
use open_pipe_stress_result_export::retained_precision::reader_logic;
use serde_json::{json, Value};
use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::Cell;

struct Meter;
thread_local! { static METER: Cell<(bool, usize, usize)> = const {Cell::new((false,0,0))}; }
fn alloc(n: usize) {
    let _ = METER.try_with(|m| {
        let (enabled, used, peak) = m.get();
        if enabled {
            m.set((true, used + n, peak.max(used + n)));
        }
    });
}
fn free(n: usize) {
    let _ = METER.try_with(|m| {
        let (enabled, used, peak) = m.get();
        if enabled {
            m.set((true, used.saturating_sub(n), peak));
        }
    });
}
unsafe impl GlobalAlloc for Meter {
    unsafe fn alloc(&self, l: Layout) -> *mut u8 {
        let p = unsafe { System.alloc(l) };
        if !p.is_null() {
            alloc(l.size());
        }
        p
    }
    unsafe fn alloc_zeroed(&self, l: Layout) -> *mut u8 {
        let p = unsafe { System.alloc_zeroed(l) };
        if !p.is_null() {
            alloc(l.size());
        }
        p
    }
    unsafe fn dealloc(&self, p: *mut u8, l: Layout) {
        free(l.size());
        unsafe { System.dealloc(p, l) }
    }
    unsafe fn realloc(&self, p: *mut u8, l: Layout, n: usize) -> *mut u8 {
        // Charge moving old+new even if the allocator can grow in place.
        alloc(n);
        let q = unsafe { System.realloc(p, l, n) };
        if q.is_null() {
            free(n);
        } else {
            free(l.size());
        }
        q
    }
}
#[global_allocator]
static ALLOCATOR: Meter = Meter;
fn bits(x: f64) -> String {
    format!("{:016x}", x.to_bits())
}
fn maximal(cancelling: bool) -> (Value, Value, Vec<Value>) {
    let nodes:Vec<_>=(0..32).map(|n|json!({"coordinates":[bits(if n%2==0{1e300}else{1e-300}),bits((n+1) as f64),bits(if n%3==0{1e-300}else{-1e300})]})).collect();
    let members:Vec<_>=(0..32).map(|n|json!({"kernel_member":n,"node_i":n,"node_j":(n+1)%32,"E":bits(1.),"G":bits(1.),"A_K":bits(1.),"Iy_K":bits(1.),"Iz_K":bits(1.),"J_K":bits(1.),"y_reference":[bits(0.),bits(1.),bits(0.)]})).collect();
    let mut loads:Vec<_>=(0..128).map(|n|json!({"constructor_ordinal":n,"primitive_load_index":n,"source_id":format!("{:03}{}",n,"m".repeat(1021)),"member":if cancelling{n/4}else{n%32},"intensity":[bits(if cancelling&&n%2==1{-1e300}else{1e300}),bits(0.0),bits(0.0)]})).collect();
    let pipes: Vec<_> = (0..32).map(|n| json!({"id":format!("pipe{n}")})).collect();
    let authored:Vec<_>=loads.iter().map(|l|json!({"id":l["source_id"],"target":{"type":"element","pipe":pipes[l["member"].as_u64().unwrap() as usize]["id"]},"category":"distributed_force","dimension":"force_per_length","direction":"global_x","magnitude":{"value":f64::from_bits(u64::from_str_radix(l["intensity"][0].as_str().unwrap(),16).unwrap()),"unit":"N/m"}})).collect();
    loads.sort_by(|a, b| {
        (a["member"].as_u64(), a["source_id"].as_str())
            .cmp(&(b["member"].as_u64(), b["source_id"].as_str()))
    });
    let nodal:Vec<_>=(0..128).map(|n|json!({"dof":{"node":n%32,"component":(["UX","UY","UZ","RX","RY","RZ"][n%6])},"source_id":"n".repeat(1024),"value":bits(if n%2==0{f64::from_bits(1)}else{f64::MAX})})).collect();
    (
        json!({"id_maps":{"nodes":nodes,"members":members,"springs":[]},"constraints":[],"stations":[],"supports":[],"nodal_terms":nodal,"member_load_terms":loads}),
        json!({"id":"cap-case","primitive_loads":authored}),
        pipes,
    )
}
pub fn measured_workspace(cancelling: bool) -> (usize, usize) {
    let (source, case, pipes) = maximal(cancelling);
    let expected = reader_logic::declared_problem(&source).unwrap().0;
    let expected = json!(expected);
    METER.with(|m| m.set((true, 0, 0)));
    let result = reader_logic::declared_workspace(&source, &case, &pipes, &expected);
    let (used, peak) = METER.with(|m| {
        let (_, used, peak) = m.get();
        m.set((false, used, peak));
        (used, peak)
    });
    result.unwrap();
    assert!(
        peak <= 1024 * 1024,
        "reader complete recompute allocation peak {peak}"
    );
    (used, peak)
}
#[test]
fn reader_cap_maximal_complete_arithmetic_net_and_hash_workspace() {
    std::thread::Builder::new().stack_size(128*1024).spawn(|| {
        println!("W1_READER_LAYOUTS {:?}",reader_logic::declared_layouts());
        for cancelling in [false,true] {
            let (used,peak)=measured_workspace(cancelling);
            assert_eq!(used,0);
            println!("W1_READER_WORKSPACE cancelling={cancelling} peak={peak} live_return={used} stack_allotment=131072 usize={} vec={} hash={}",std::mem::size_of::<usize>(),std::mem::size_of::<Vec<u64>>(),std::mem::size_of::<sha2::Sha256>());
        }
    }).unwrap().join().unwrap();
}

#[test]
fn member_encoding_bounds_refuse_before_allocating_the_encoding() {
    let (mut source,case,pipes)=maximal(false);
    let expected=json!("0".repeat(64));
    source["member_load_terms"][0]["source_id"]=json!("m".repeat(1025));
    METER.with(|m|m.set((true,0,0)));
    let error=reader_logic::declared_workspace(&source,&case,&pipes,&expected).unwrap_err();
    let peak=METER.with(|m|{let (_,used,peak)=m.get();m.set((false,used,peak));peak});
    assert_eq!((error.gate,error.code.as_str()),("G8","RETAINED_PRECISION_PREPARATION_MISMATCH"));
    assert!(peak<1024,"new-domain bound must precede encoding allocation: {peak}");
}

/// This exact maintained test helper also runs in the scratch wasm consumer.
/// It reads the third oracle's vectors; no arithmetic is copied into the probe.
pub fn declared_vector_count()->usize {
    let mut source=Value::Null;
    let mut count=0;
    for line in include_str!("../../../solver/frame_kernel/tests/retained_k4/declared.txt").lines() {
        let row:Vec<_>=line.split_whitespace().collect();
        if row.is_empty(){continue;}
        match row[0] {
            "case"=>source=json!({"id_maps":{"nodes":[{"coordinates":&row[2..5]},{"coordinates":&row[5..8]}],"members":[{"node_i":0,"node_j":1}]},"nodal_terms":[],"member_load_terms":[]}),
            "load"=>source["member_load_terms"].as_array_mut().unwrap().push(json!({"member":0,"intensity":&row[1..4]})),
            "nodal"=>{let dof=row[1].parse::<usize>().unwrap();source["nodal_terms"].as_array_mut().unwrap().push(json!({"dof":{"node":dof/6,"component":(["UX","UY","UZ","RX","RY","RZ"][dof%6])},"value":row[2]}));},
            "ledger"=>{assert_eq!(reader_logic::declared_problem(&source).unwrap().0,row[2]);count+=1;},
            "entries"|"end"=>{},_=>panic!("unknown vector record"),
        }
    }
    count
}
#[test]
fn independent_declared_ledger_vectors_on_the_consuming_reader() {
    assert_eq!(declared_vector_count(),16);
}

pub fn measured_full_g8(text:&str,load_cap:bool)->(usize,usize) {
    use open_pipe_stress_result_export::source_blocks::domain_hash;
    let mut record:Value=serde_json::from_str(text).unwrap();
    if load_cap {
        let sid=record["source"]["retained_precision"]["body"]["sources"].as_array().unwrap().iter().position(|s|s.get("member_load_terms").is_some()).unwrap();
        let ci=record["source"]["retained_precision"]["body"]["sources"][sid]["owner"]["case_index"].as_u64().unwrap() as usize;
        let original=record["source"]["retained_precision"]["body"]["sources"][sid]["member_load_terms"][0].clone();
        let ordinal=original["primitive_load_index"].as_u64().unwrap() as usize;
        let authored=record["invocation"]["request"]["model"]["load_cases"][ci]["primitive_loads"][ordinal].clone();
        while record["invocation"]["request"]["model"]["load_cases"][ci]["primitive_loads"].as_array().unwrap().len()<128 {
            let at=record["invocation"]["request"]["model"]["load_cases"][ci]["primitive_loads"].as_array().unwrap().len();
            let mut a=authored.clone();a["id"]=json!(format!("{at:03}{}","m".repeat(125)));
            record["invocation"]["request"]["model"]["load_cases"][ci]["primitive_loads"].as_array_mut().unwrap().push(a.clone());
            let mut term=original.clone();term["constructor_ordinal"]=json!(at);term["primitive_load_index"]=json!(at);term["source_id"]=a["id"].clone();
            record["source"]["retained_precision"]["body"]["sources"][sid]["member_load_terms"].as_array_mut().unwrap().push(term);
        }
        let s=&mut record["source"]["retained_precision"]["body"]["sources"][sid];
        s["member_load_terms"].as_array_mut().unwrap().sort_by(|a,b|(a["member"].as_u64(),a["source_id"].as_str(),a["constructor_ordinal"].as_u64()).cmp(&(b["member"].as_u64(),b["source_id"].as_str(),b["constructor_ordinal"].as_u64())));
        s["kernel_source_sha256"]=json!(reader_logic::native_source_digest(s).unwrap());
        let digest=reader_logic::declared_problem(s).unwrap().0;
        record["source"]["retained_precision"]["body"]["cases"][ci]["selection"]["ledger_sha256"]=json!(digest);
        record["source"]["retained_precision"]["body"]["invocation"]["value"]=json!(domain_hash("source_blocks_invocation_v1",&record["invocation"]).unwrap());
    }
    METER.with(|m|m.set((true,0,0)));
    let result=reader_logic::invocation_binding(&record["source"],&record["invocation"]);
    let (used,peak)=METER.with(|m|{let (_,used,peak)=m.get();m.set((false,used,peak));(used,peak)});
    result.unwrap();
    (used,peak)
}
/// Full native/wasm consuming controls, with every ledger attestation forged
/// together and the outer hash refreshed. This grants no producer provenance.
pub fn actual_successor_control(text:&str) {
    use open_pipe_stress_result_export::{retained_precision as rp,source_blocks::domain_hash};
    let mut record:Value=serde_json::from_str(text).unwrap();
    let v=rp::validate(&record["source"],Some(&record["invocation"])).unwrap();assert!(v.numerical_eligible);
    let v=rp::validate(&record["source"],None).unwrap();assert!(!v.numerical_eligible);
    fn forge(v:&mut Value) { match v {
        Value::Object(o)=>for (k,v) in o {if k=="ledger_sha256"{*v=json!("0".repeat(64));}else{forge(v);}},
        Value::Array(a)=>for v in a{forge(v);},_=>{},
    }}
    forge(&mut record["source"]["retained_precision"]["body"]);
    record["source"]["retained_precision"]["receipt_sha256"]=json!(domain_hash("retained_precision_receipt_mp_v2",&record["source"]["retained_precision"]["body"]).unwrap());
    let error=rp::validate(&record["source"],Some(&record["invocation"])).unwrap_err();
    assert_eq!((error.gate,error.code.as_str()),("G8","RETAINED_PRECISION_PREPARATION_MISMATCH"));
}
#[test]
#[ignore="run with W1_MEMBER_READER_INPUT_DIR after the maintained product writer"]
fn genuine_member_product_full_reader_and_full_g8_allocations() {
    let dir=std::env::var("W1_MEMBER_READER_INPUT_DIR").unwrap();
    for mode in ["sparse_interactive","dense_scrutiny"] {
        let text=std::fs::read_to_string(format!("{dir}/w1_member_load_{mode}.json")).unwrap();
        actual_successor_control(&text);
        for cap in [false,true] {let (used,peak)=measured_full_g8(&text,cap);assert_eq!(used,0);println!("W1_FULL_G8 mode={mode} load_cap={cap} peak={peak} live_return={used}");}
    }
}
#[test]
fn full_g8_rejects_mixed_overcap_before_any_hash_or_json_staging() {
    member_guard_scope_control();
}
/// Also run through the real wasm consumer; no test-only guard implementation.
pub fn member_guard_scope_control() {
    let mut loads=vec![json!({"id":"n".repeat(1024),"target":{"type":"node"}});128];
    loads.push(json!({"target":{"type":"element"},"category":"distributed_force","dimension":"force_per_length"}));
    let inv=json!({"request":{"model":{"schema_version":"0.8.0","load_cases":[{"id":"c","primitive_loads":loads}]}},"solver_mode":"sparse_interactive"});
    let source=json!({"retained_precision":{"body":{"sources":[]}}});
    METER.with(|m|m.set((true,0,0)));
    let error=reader_logic::invocation_binding(&source,&inv).unwrap_err();
    let peak=METER.with(|m|{let (_,used,peak)=m.get();m.set((false,used,peak));peak});
    assert_eq!((error.gate,error.code.as_str()),("G8","RETAINED_PRECISION_PREPARATION_MISMATCH"));
    assert!(peak<1024,"entry guard must precede old G8 hash/JSON staging: {peak}");
    let mut malformed=inv.clone();
    malformed["request"]["model"]["load_cases"][0]["primitive_loads"][128]["direction"]=json!("ux");
    malformed["request"]["model"]["load_cases"][0]["primitive_loads"][128]["extent"]=json!([0,1]);
    malformed["request"]["model"]["load_cases"][0]["primitive_loads"][128]["magnitude"]=json!({"value":1,"unit":"wrong-unit"});
    METER.with(|m|m.set((true,0,0)));
    let error=reader_logic::invocation_binding(&source,&malformed).unwrap_err();
    let peak=METER.with(|m|{let (_,used,peak)=m.get();m.set((false,used,peak));peak});
    assert_eq!(error.code,"RETAINED_PRECISION_PREPARATION_MISMATCH");
    assert!(peak<1024,"malformed new candidate must precede hash/JSON staging: {peak}");
    let mut nodal=inv.clone();nodal["request"]["model"]["load_cases"][0]["primitive_loads"][128]["target"]["type"]=json!("node");
    let error=reader_logic::invocation_binding(&source,&nodal).unwrap_err();
    assert_eq!(error.code,"RETAINED_PRECISION_INVOCATION_MISMATCH"); // unchanged old hash gate, not the new cap guard
    // A forged source key must bound before hashing even outside the namespace.
    nodal["request"]["model"]["schema_version"]=json!("0.3.0");
    for marker in [Value::Null,json!([])] {
        let mut forged=source.clone();forged["retained_precision"]["body"]["sources"]=json!([{"member_load_terms":marker}]);
        METER.with(|m|m.set((true,0,0)));
        let error=reader_logic::invocation_binding(&forged,&nodal).unwrap_err();
        let peak=METER.with(|m|{let (_,used,peak)=m.get();m.set((false,used,peak));peak});
        assert_eq!(error.code,"RETAINED_PRECISION_PREPARATION_MISMATCH");
        assert!(peak<1024,"forged member source key must precede hash/JSON staging: {peak}");
    }
}

pub fn actual_exhaustion_control(text:&str) {
    use open_pipe_stress_result_export::{retained_precision as rp,source_blocks::domain_hash};
    let record:Value=serde_json::from_str(text).unwrap();
    let validation=rp::validate(&record["source"],Some(&record["invocation"])).unwrap();
    assert!(validation.invocation_bound && !validation.numerical_eligible);
    let ai=record["source"]["retained_precision"]["body"]["product_attempts"].as_array().unwrap().iter().position(|a|a["proof"].get("loaded_maximum_diagnostic").is_some()).unwrap();
    for fault in ["missing","cause","member","row","negative","zero_exp","unnormalized","reversed","fractional","limbcase"] {
        let mut mutant=record["source"].clone();let a=&mut mutant["retained_precision"]["body"]["product_attempts"][ai];
        match fault {
            "missing"=>{a["proof"].as_object_mut().unwrap().remove("loaded_maximum_diagnostic");},
            "cause"=>a["result"]["error"]["cause"]["detail"]=json!("other failure"),
            "member"=>a["proof"]["loaded_maximum_diagnostic"]["member"]=json!(31),
            "row"=>a["proof"]["loaded_maximum_diagnostic"]["row"]=json!(99999),
            "negative"=>a["proof"]["loaded_maximum_diagnostic"]["lower"]["negative"]=json!(true),
            "zero_exp"=>{a["proof"]["loaded_maximum_diagnostic"]["lower"]["limbs"]=json!(vec!["0000000000000000";16]);a["proof"]["loaded_maximum_diagnostic"]["lower"]["exponent"]=json!(1);},
            "unnormalized"=>a["proof"]["loaded_maximum_diagnostic"]["upper"]["limbs"][15]=json!("4000000000000000"),
            "reversed"=>{let upper=a["proof"]["loaded_maximum_diagnostic"]["upper"].clone();let e=upper["exponent"].as_i64().unwrap();a["proof"]["loaded_maximum_diagnostic"]["lower"]=upper;a["proof"]["loaded_maximum_diagnostic"]["lower"]["exponent"]=json!(e+1);},
            "fractional"=>a["proof"]["loaded_maximum_diagnostic"]["upper"]["exponent"]=json!(0.5),
            _=>a["proof"]["loaded_maximum_diagnostic"]["upper"]["limbs"][0]=json!("FFFFFFFFFFFFFFFF"),
        }
        mutant["retained_precision"]["receipt_sha256"]=json!(domain_hash("retained_precision_receipt_mp_v2",&mutant["retained_precision"]["body"]).unwrap());
        let error=rp::validate(&mutant,Some(&record["invocation"])).unwrap_err();
        assert_eq!(error.gate,if matches!(fault,"fractional"|"limbcase"){"G2"}else{"G5"},"{fault}: {}",error.code);
    }
}
/// The genuine-exhaustion control's two writer outputs (`w1_member_exhaustion_{mode}.json`),
/// read in full and mutated. Since the W1 coverage repair (DEL-04-08 `w1-coverage-repair.md`
/// §1.4, option 2) the control is hook-driven: its writer,
/// `w1_member_load_exhaustion_retains_typed_interval_work`, runs only with product_physics'
/// `--features loaded-maximum-hooks` and drives a real member to exhaustion at reduced search
/// limits; it claims no production limit.
#[test]
#[ignore="run with W1_MEMBER_READER_INPUT_DIR after the maintained, hook-driven exhaustion writer (--features loaded-maximum-hooks)"]
fn genuine_member_exhaustion_full_reader_and_mutants() {
    let dir=std::env::var("W1_MEMBER_READER_INPUT_DIR").unwrap();
    for mode in ["sparse_interactive","dense_scrutiny"] {
        let text=std::fs::read_to_string(format!("{dir}/w1_member_exhaustion_{mode}.json")).unwrap();actual_exhaustion_control(&text);
    }
}
