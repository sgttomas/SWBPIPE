#!/usr/bin/env python3
"""Observe actual private API through isolated Rust; compare to frozen oracle."""
import hashlib
import importlib.util
import json
import math
from pathlib import Path
import struct
import subprocess
import sys
sys.dont_write_bytecode = True

HERE = Path(__file__).resolve().parent
ORACLE = HERE.parent
ROOT = next(p for p in HERE.parents if (p / 'projects/chirality-piping/core/product_physics/src/pressure_exact.rs').exists())
SOURCE = ROOT / 'projects/chirality-piping/core/product_physics/src/pressure_exact.rs'
SOURCE_HASH = 'baa83a5a62ea34d0677a5c5c5d6cd2fd2f977b6b62a5e2d4ddd06fa18951abc0'
assert hashlib.sha256(SOURCE.read_bytes()).hexdigest() == SOURCE_HASH
spec = importlib.util.spec_from_file_location('frozen_independent_oracle', ORACLE / 'independent_oracle.py')
oracle = importlib.util.module_from_spec(spec)
spec.loader.exec_module(oracle)
frozen = json.loads((ORACLE / 'FROZEN_EXPECTATIONS.json').read_text())
invalid = json.loads((ORACLE / 'INVALID_INPUT_EXPECTATIONS.json').read_text())


def f64(value):
    if isinstance(value, str):
        return {'NaN': 'f64::NAN', '+Infinity': 'f64::INFINITY', '-Infinity': 'f64::NEG_INFINITY'}[value]
    bits = struct.unpack('>Q', struct.pack('>d', value))[0]
    return f'f64::from_bits({bits})'


RUST_BODY = r'''
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
'''


def main_body(cases, invalids):
    statements = ['fn main(){']
    for case in cases:
        inp = case['inputs']
        args = [f64(inp[k]) for k in ('ri_m','ro_m','E_pa','nu','p_pa','thermal_strain','epsilon_z')]
        witnesses = ','.join(f'({json.dumps(label)},{f64(r)})' for label, r in inp.get('radius_witnesses',{}).items())
        statements.append(f'run({json.dumps(case["id"])},{",".join(args)},&[{witnesses}]);')
    for case in invalids:
        i = case['inputs']
        a = f'ExactAnnulus::from_radii({f64(i["ri_m"])},{f64(i["ro_m"])})'
        m = f'IsotropicENu::new({f64(i["E_pa"])},{f64(i["nu"])})'
        p = f'InternalDifferentialPressure::new({f64(i["p_pa"])})'
        op = case['operation']
        for operation in op.split(' and '):
            if operation == 'ExactAnnulus::from_radii': expr = a
            elif operation == 'IsotropicENu::new': expr = m
            elif operation == 'InternalDifferentialPressure::new': expr = p
            elif operation == 'lame_at_radius': expr = f'lame_at_radius({a}?,{p}?,{f64(i["r_m"])})'
            elif operation == 'axial_state': expr = f'axial_state({a}?,{m}?,{p}?,{f64(i["thermal_strain"])},{f64(i["epsilon_z"])})'
            elif operation == 'eigenload_pair': expr = f'eigenload_pair({a}?,{m}?,{p}?,{f64(i["thermal_strain"])})'
            elif operation == 'cap_pair': expr = f'cap_pair({a}?,{p}?)'
            else: raise AssertionError(operation)
            ident = case['id'] + '/' + operation
            statements.append(f'invalid_result({json.dumps(ident)},(||{{ {expr}.map(|_|()) }})());')
    return '\n'.join(statements) + '\n}\n'


def run_adapter(name, source, cases, invalids):
    folder = HERE / name
    folder.mkdir(exist_ok=True)
    adapter = folder / 'adapter.rs'
    adapter.write_text('#![allow(dead_code)]\n#[path = ' + json.dumps(str(source)) + ']\nmod pressure_exact;\n' + RUST_BODY + main_body(cases, invalids))
    build_dir = Path('/private/tmp/piping-pressure-refutation-20260914')
    build_dir.mkdir(exist_ok=True)
    executable = build_dir / ('V3-' + name + '-adapter')
    cmd = ['rustc','--edition=2021',str(adapter),'-o',str(executable)]
    build = subprocess.run(cmd,cwd=folder,text=True,capture_output=True)
    (folder / 'BUILD.txt').write_text(build.stdout+build.stderr)
    records = [{'argv':cmd,'exit_code':build.returncode,'cwd':str(folder)}]
    assert build.returncode == 0, build.stderr
    result = subprocess.run([str(executable)],cwd=folder,text=True,capture_output=True)
    (folder / 'ACTUALS.tsv').write_text(result.stdout+result.stderr)
    records.append({'argv':[str(executable)],'exit_code':result.returncode,'cwd':str(folder)})
    assert result.returncode == 0, result.stderr
    values, errors, invalid_results = {}, [], {}
    for line in result.stdout.splitlines():
        cols = line.split('\t')
        if cols[0] == 'VAL': values.setdefault(cols[1],{})[cols[2]] = float(cols[3])
        elif cols[0] == 'ERR': errors.append({'case':cols[1],'operation':cols[2],'error':cols[3]})
        elif cols[0] == 'INVALID': invalid_results[cols[1]] = cols[2]
        else: raise AssertionError(line)
    failures = oracle.compare_actuals(values,dict(frozen,cases=cases))
    invalid_failures = {k:v for k,v in invalid_results.items() if v=='ACCEPTED'}
    out = {'actual_source':str(source),'actual_source_sha256':hashlib.sha256(source.read_bytes()).hexdigest(),
           'cases_checked':len(cases),'scalar_comparisons':sum(len(c['checks']) for c in cases),
           'invalid_fixture_count':len(invalids),'invalid_operation_count':len(invalid_results),
           'invalid_results':invalid_results,'invalid_failures':invalid_failures,
           'api_errors':errors,'comparison_failures':failures,'commands':records}
    (folder / 'ACTUALS.json').write_text(json.dumps(values,indent=2,allow_nan=False)+'\n')
    (folder / 'RESULT.json').write_text(json.dumps(out,indent=2,allow_nan=False)+'\n')
    return out


def new_expectations():
    import random
    base=dict(ri_m=1.,ro_m=2.,E_pa=120.,nu=.25,p_pa=3.,thermal_strain=0.,epsilon_z=0.)
    candidates=[]
    def add(name,**changes):candidates.append((name,dict(base,**changes)))
    def stepped(value,k):
        direction=math.inf if k>0 else 0.
        for _ in range(abs(k)):value=math.nextafter(value,direction)
        return value
    # Final cap scaling around gradual underflow, including signed zero.
    for sign in (-1.,1.):
        for p_exp in (-78,-77,-76,-75,-74,-73,-72):
            add(f'CAP_SUBNORMAL_s{sign}_p{p_exp}',ri_m=2.**-500,ro_m=2.**-499,p_pa=sign*2.**p_exp)
        for pzero in (0.,-0.):
            add(f'SIGNED_ZERO_s{sign}_{math.copysign(1.,pzero)}',p_pa=pzero)
    # The exact root-near-boundary inputs are independently promoted before
    # computing ideal areas/P. No observed constructor outcome filters cases.
    as_center=math.sqrt(sys.float_info.max/(3*math.pi))
    ai_center=math.sqrt(sys.float_info.max/math.pi)
    cap_center=sys.float_info.max/math.pi
    for k in range(-20,21):
        ri=stepped(as_center,k)
        add(f'WALL_AREA_OVERFLOW_NEAR_{k}',ri_m=ri,ro_m=2*ri,E_pa=2.**-900,p_pa=1.)
        ri=stepped(ai_center,k)
        add(f'FLUID_AREA_OVERFLOW_NEAR_{k}',ri_m=ri,ro_m=1.125*ri,p_pa=1.)
        p=stepped(cap_center,k)
        add(f'CAP_OVERFLOW_NEAR_{k}',p_pa=p)
    # Constructor positive subnormal boundary and final normal force/stress.
    tiny_center=math.sqrt(math.ulp(0.))
    for k in range(-10,11):
        ri=stepped(tiny_center,k)
        for ratio in (1.125,1.5,2.):
            add(f'GEOMETRY_SUBNORMAL_NEAR_{k}_{ratio}',ri_m=ri,ro_m=ri*ratio,p_pa=2.**200,
                radius_witnesses={'midwall':(ri+ri*ratio)/2})
    # Diverse independently computed exponent/sign cases. Some generated
    # cases intentionally exceed output representation; excluded before run.
    rng=random.Random(2026091403)
    for j in range(384):
        ri=math.ldexp(rng.randint(8,15)/8.,rng.choice([-538,-520,-400,-200,0,200,400,500]))
        ratio=rng.choice([1.0625,1.125,1.5,2.,16.,2.**100])
        ro=ri*ratio
        if not math.isfinite(ro):continue
        if j%17==0:ro=math.nextafter(ri,math.inf)
        e=math.ldexp(rng.randint(8,15)/8.,rng.choice([-1000,-500,-10,0,10,500,1000]))
        nu=rng.choice([-.875,-.75,-.5,0.,.25,.49])
        p=rng.choice([-1.,1.])*2.**rng.choice([-1000,-800,-400,0,400,800,1000])
        thermal=epsilon=0.
        if j%4:
            thermal=rng.choice([-1.,0.,1.])*2.**rng.choice([-1074,-1000,-500,-100,-10,0,500,1000])
            epsilon=rng.choice([-1.,0.,1.])*2.**rng.choice([-1074,-1000,-500,-100,-10,0,500,1000])
        if j%4==1:p=0.
        add(f'EXPONENT_GRID_{j:03d}',ri_m=ri,ro_m=ro,E_pa=e,nu=nu,p_pa=p,thermal_strain=thermal,epsilon_z=epsilon,
            radius_witnesses={'midwall':ri+(ro-ri)/2,'quarterwall':ri+(ro-ri)/4})
    cases=[];excluded=[]
    for name,inp in candidates:
        checks=oracle.expected_values(inp)
        if any(not math.isfinite(v['expected_f64']) for v in checks.values()) or any(checks[k]['expected_f64']<=0 for k in ('Ai_m2','As_m2','G_pa')):
            excluded.append({'id':name,'reason':'independent required final output is nonfinite, or positive area/G rounds to zero'})
            continue
        cases.append({'id':name,'inputs':inp,'checks':checks})
    return cases,excluded


def bits(x):return struct.pack('>d',x).hex()


def main():
    new,excluded=new_expectations()
    (HERE/'ADDITIONAL_EXPECTATIONS.json').write_text(json.dumps(new,indent=2,allow_nan=False)+'\n')
    (HERE/'EXCLUDED_GENERATOR_CASES.json').write_text(json.dumps(excluded,indent=2)+'\n')
    bit_expectations=[]
    for c in new:
        if c['id'].startswith('CAP_SUBNORMAL'):
            bit_expectations.append({'id':c['id'],'cap_i_n':bits(c['checks']['cap_i_n']['expected_f64']),
                                     'cap_j_n':bits(c['checks']['cap_j_n']['expected_f64'])})
        if c['id'].startswith('SIGNED_ZERO'):
            p=c['inputs']['p_pa']
            bit_expectations.append({'id':c['id'],'cap_i_n':bits(-p),'cap_j_n':bits(p)})
    (HERE/'FINAL_CONVERSION_EXPECTATIONS.json').write_text(json.dumps(bit_expectations,indent=2)+'\n')
    pre={'status':'FROZEN_BEFORE_V3_INVOCATION','source_sha256':SOURCE_HASH,
         'additional_expectations_sha256':hashlib.sha256((HERE/'ADDITIONAL_EXPECTATIONS.json').read_bytes()).hexdigest(),
         'final_conversion_expectations_sha256':hashlib.sha256((HERE/'FINAL_CONVERSION_EXPECTATIONS.json').read_bytes()).hexdigest(),
         'additional_cases':len(new),'excluded_before_observation':len(excluded),
         'policy':'unchanged scalar limits; final cap rounding/zero bits separately check the explicit V3 gradual-underflow requirement'}
    (HERE/'EXPECTATION_FREEZE.json').write_text(json.dumps(pre,indent=2)+'\n')
    baseline=run_adapter('BASELINE',SOURCE,frozen['cases'],invalid['cases'])
    v1=run_adapter('V1_DIAGNOSTICS',SOURCE,json.loads((ORACLE/'REFUTATION_V1/DIAGNOSTIC_EXPECTATIONS.json').read_text()),[])
    v2=run_adapter('V2_DIAGNOSTICS',SOURCE,json.loads((ORACLE/'REFUTATION_V2/ADDITIONAL_EXPECTATIONS.json').read_text()),[])
    additional=run_adapter('ADDITIONAL',SOURCE,new,[])
    actual=json.loads((HERE/'ADDITIONAL/ACTUALS.json').read_text())
    bit_results=[]
    for item in bit_expectations:
        got=actual.get(item['id'],{})
        result={'id':item['id'],'quantities':{}}
        for key in ('cap_i_n','cap_j_n'):
            observed=bits(got[key]) if key in got else None
            result['quantities'][key]={'expected_bits':item[key],'actual_bits':observed,'pass':observed==item[key]}
        bit_results.append(result)
    (HERE/'FINAL_CONVERSION_RESULTS.json').write_text(json.dumps(bit_results,indent=2)+'\n')
    print('BASELINE',len(baseline['comparison_failures']),len(baseline['invalid_failures']),flush=True)
    print('V1/V2',len(v1['comparison_failures']),len(v2['comparison_failures']),flush=True)
    print('ADDITIONAL',len(new),len(additional['comparison_failures']),flush=True)
    original=SOURCE.read_text()
    axial_poisson='Scaled::from_f64(material.nu).mul(pressure_trace_scaled(annulus, pressure));'
    eigen_poisson='let poisson_force = Scaled::from_f64(2.0)'
    assert original.count(axial_poisson)==1 and original.count(eigen_poisson)==1
    mutations={}
    for name,factor,eigenfactor in [('OMIT_2NUP','0.0','0.0'),('DOUBLE_2NUP','2.0','4.0'),('REVERSE_2NUP','-1.0','-2.0')]:
        changed=original.replace(axial_poisson,f'Scaled::from_f64(material.nu).mul(Scaled::from_f64({factor})).mul(pressure_trace_scaled(annulus, pressure));')
        changed=changed.replace(eigen_poisson,f'let poisson_force = Scaled::from_f64({eigenfactor})')
        mutations[name]=changed
    wall='let wall_force = annulus.as_scaled.mul(axial_membrane);'
    assert original.count(wall)==1
    mutations['SUBTRACT_CAP_FROM_WALL']=original.replace(wall,'let wall_force = annulus.as_scaled.mul(axial_membrane).sub(fluid_force_scaled(annulus, pressure));')
    fluid='Scaled::from_f64(pressure.p_pa).mul(annulus.ai_scaled)'
    assert original.count(fluid)==1
    mean='Scaled::from_f64((annulus.ri_m + annulus.ro_m) / 2.0)'
    mutations['MEAN_RADIUS_FLUID_AREA']=original.replace(fluid,f'Scaled::from_f64(pressure.p_pa).mul(Scaled::from_f64(PI)).mul({mean}).mul({mean})')
    effective='let effective_force = wall_force.sub(fluid_force);'
    assert original.count(effective)==1
    mutations['OMIT_FLUID_EFFECTIVE_TERM']=original.replace(effective,'let effective_force = wall_force;')
    mutation_results=[]
    for name,content in mutations.items():
        assert content!=original
        folder=HERE/name;folder.mkdir(exist_ok=True)
        source=folder/'pressure_exact_mutant.rs';source.write_text(content)
        result=run_adapter(name,source,frozen['cases'],[])
        assert result['comparison_failures'],name
        mutation_results.append({'mutation':name,'class':'actual V3 implementation mutation in evidence-local copy','killed':True,
            'scalar_failure_count':len(result['comparison_failures']),'first_failure':result['comparison_failures'][0],
            'source_sha256':result['actual_source_sha256']})
    (HERE/'MUTATION_RESULTS.json').write_text(json.dumps(mutation_results,indent=2)+'\n')
    assert hashlib.sha256(SOURCE.read_bytes()).hexdigest()==SOURCE_HASH
    bit_failures=sum(not q['pass'] for r in bit_results for q in r['quantities'].values())
    failed=any(r['comparison_failures'] or r['invalid_failures'] for r in [baseline,v1,v2,additional]) or bit_failures
    summary={'status':'FAIL_ACTIONABLE_NUMERIC_FINDINGS' if failed else 'PASS',
             'baseline_valid_cases':114,'baseline_scalar_comparisons':1990,'baseline_invalid_fixtures':44,'baseline_invalid_operations':47,
             'baseline_failures':len(baseline['comparison_failures'])+len(baseline['invalid_failures']),
             'v1_cases':5,'v1_failures':len(v1['comparison_failures']),
             'v2_cases':151,'v2_failures':len(v2['comparison_failures']),
             'additional_cases':len(new),'additional_scalar_comparisons':additional['scalar_comparisons'],
             'additional_failures':len(additional['comparison_failures']),
             'final_conversion_bit_checks':len(bit_results)*2,'final_conversion_bit_failures':bit_failures,
             'actual_v3_mutations_killed':len(mutation_results)}
    (HERE/'SUMMARY.json').write_text(json.dumps(summary,indent=2)+'\n')
    print(json.dumps(summary,indent=2),flush=True)


if __name__=='__main__':main()
