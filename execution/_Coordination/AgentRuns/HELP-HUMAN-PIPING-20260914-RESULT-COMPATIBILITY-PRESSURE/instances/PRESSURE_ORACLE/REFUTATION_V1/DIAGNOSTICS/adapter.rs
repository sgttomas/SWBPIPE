#![allow(dead_code)]
#[path = "/Users/ryan/.codex/worktrees/8728/chirality-result-compatibility-pressure-20260914/projects/chirality-piping/core/product_physics/src/pressure_exact.rs"]
mod pressure_exact;

use pressure_exact::*;
fn value(id: &str, field: &str, val: f64) { println!("VAL\t{}\t{}\t{:.17e}",id,field,val); }
fn error(id: &str, operation: &str, err: ExactPressureError) { println!("ERR\t{}\t{}\t{:?}",id,operation,err); }
fn run(id: &str, ri: f64, ro: f64, e: f64, nu: f64, p: f64, thermal: f64, strain: f64, witnesses: &[(&str,f64)]) {
    let a=match ExactAnnulus::from_radii(ri,ro) { Ok(x)=>x,Err(x)=>{error(id,"geometry",x);return;} };
    let m=match IsotropicENu::new(e,nu) { Ok(x)=>x,Err(x)=>{error(id,"material",x);return;} };
    let p=match InternalDifferentialPressure::new(p) { Ok(x)=>x,Err(x)=>{error(id,"pressure",x);return;} };
    value(id,"Ai_m2",a.internal_area_m2()); value(id,"As_m2",a.wall_area_m2()); value(id,"G_pa",m.shear_modulus_pa());
    match axial_state(a,m,p,thermal,strain) { Ok(x)=>{
        value(id,"wall_force_n",x.wall_force_n()); value(id,"effective_force_n",x.effective_force_n()); value(id,"axial_membrane_pa",x.axial_membrane_pa());
    },Err(x)=>error(id,"axial_state",x) }
    match eigenload_pair(a,m,p,thermal) { Ok(x)=>{value(id,"eigen_i_n",x[0]);value(id,"eigen_j_n",x[1]);},Err(x)=>error(id,"eigenload_pair",x) }
    match cap_pair(a,p) { Ok(x)=>{value(id,"cap_i_n",x[0]);value(id,"cap_j_n",x[1]);},Err(x)=>error(id,"cap_pair",x) }
    for (label,radius) in [("inner",ri),("outer",ro)].iter().chain(witnesses.iter()) {
        match lame_at_radius(a,p,*radius) { Ok(x)=>{
            value(id,&format!("{}_radial_pa",label),x.radial_pa()); value(id,&format!("{}_hoop_pa",label),x.hoop_pa());
        },Err(x)=>error(id,&format!("lame_{}",label),x) }
    }
}
fn invalid_result(id: &str, result: Result<(),ExactPressureError>) {
    match result {Ok(())=>println!("INVALID\t{}\tACCEPTED",id),Err(e)=>println!("INVALID\t{}\t{:?}",id,e)}
}
fn main(){
run("NORMAL_AREA_FLUID_FORCE_UNDERFLOW",f64::from_bits(2805742567851819008),f64::from_bits(2810246167479189504),f64::from_bits(4638144666238189568),f64::from_bits(4598175219545276416),f64::from_bits(2805742567851819008),f64::from_bits(0),f64::from_bits(0),&[("midwall",f64::from_bits(2807994367665504256))]);
run("SUBNORMAL_AREA_STRESS_RECONSTRUCTION",f64::from_bits(2188749418902061056),f64::from_bits(2189312368855482368),f64::from_bits(4638144666238189568),f64::from_bits(4598175219545276416),f64::from_bits(4613937818241073152),f64::from_bits(0),f64::from_bits(0),&[("midwall",f64::from_bits(2189030893878771712))]);
run("MAX_ASSOCIATION_ROUNDUP_ELASTIC",f64::from_bits(2355382605114769408),f64::from_bits(2359886204742139904),f64::from_bits(7309342195222315008),f64::from_bits(4598175219545276416),f64::from_bits(0),f64::from_bits(0),f64::from_bits(4255901647865118720),&[]);
run("MAX_ASSOCIATION_ROUNDUP_THERMAL",f64::from_bits(2355382605114769408),f64::from_bits(2359886204742139904),f64::from_bits(7309342195222315008),f64::from_bits(4598175219545276416),f64::from_bits(0),f64::from_bits(4255901647865118720),f64::from_bits(0),&[]);
run("OVERFLOWING_FORCE_CANCELLATION_THERMAL",f64::from_bits(4607182418800017408),f64::from_bits(4652218415073722368),f64::from_bits(9209861237972664320),f64::from_bits(13828302655841107968),f64::from_bits(9209861237972664320),f64::from_bits(13742734269363525632),f64::from_bits(0),&[]);
}
