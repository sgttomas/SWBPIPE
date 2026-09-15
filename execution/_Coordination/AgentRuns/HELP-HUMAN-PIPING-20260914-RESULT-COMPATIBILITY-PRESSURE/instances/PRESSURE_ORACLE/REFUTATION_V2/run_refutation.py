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
SOURCE_HASH = '8533622eaa7d87c102791d60bc9eb3607d6daf119a2da41cbfe866e89e4d283f'
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
    executable = build_dir / ('V2-' + name + '-adapter')
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
    base = dict(ri_m=1.,ro_m=2.,E_pa=120.,nu=.25,p_pa=3.,thermal_strain=0.,epsilon_z=0.)
    candidates = []
    def add(name, **changes): candidates.append((name,dict(base,**changes)))
    # The applied RHS identity gives eigen_i = -EAs*thermal > 0 here.
    # This reduction is performed and frozen before any observed V2 output.
    add('ROOT_OPPOSED_STRAINS',E_pa=2.**-1000,p_pa=0.,epsilon_z=2.**1023,thermal_strain=-(2.**1023))
    for sign in (-1.,1.):
        for e_exp in (-1002,-1000,-998):
            for s_exp in (1022,1023):
                add(f'OPPOSED_s{sign}_E{e_exp}_strain{s_exp}',E_pa=2.**e_exp,p_pa=0.,epsilon_z=sign*2.**s_exp,thermal_strain=-sign*2.**s_exp)
    # Broad ratio: multiplying pressure after squaring the radius ratio can
    # erase a normal final radial stress. No production value sets this case.
    for sign in (-1.,1.):
        for lo,hi in [(-400,400),(-350,350),(-300,300)]:
            add(f'WIDE_RATIO_s{sign}_{lo}_{hi}',ri_m=2.**lo,ro_m=2.**hi,p_pa=sign*2.**900,
                radius_witnesses={'midwall':2.**(hi-1),'quarterwall':2.**(hi-2)})
    # Previously admitted subnormal geometry combined with normal pressure
    # that scales the dimensional force out of the subnormal range.
    for sign in (-1.,1.):
        for radius_exp in (-538,-537,-536,-535):
            for ratio in (1.0625,1.125,1.5,2.):
                for p_exp in (0,200,500,900):
                    ri=2.**radius_exp
                    add(f'TINY_AREA_s{sign}_r{radius_exp}_ratio{ratio}_p{p_exp}',ri_m=ri,ro_m=ratio*ri,
                        p_pa=sign*2.**p_exp,radius_witnesses={'midwall':(1+ratio)*ri/2})
    # Useful nearby values around each frozen V1 diagnostic, both load signs.
    old=json.loads((ORACLE/'REFUTATION_V1/DIAGNOSTIC_EXPECTATIONS.json').read_text())
    for old_case in old:
        for sign in (-1.,1.):
            for k in (-1,0,1):
                inp=dict(old_case['inputs'])
                for key in ('p_pa','thermal_strain','epsilon_z'):
                    inp[key]=math.ldexp(float(inp[key]),k)*sign
                if all(math.isfinite(inp[key]) for key in ('p_pa','thermal_strain','epsilon_z')):
                    candidates.append((old_case['id']+f'_near_s{sign}_k{k}',inp))
    cases=[]
    excluded=[]
    for name,inp in candidates:
        checks=oracle.expected_values(inp)
        # All-output probes are reserved for mathematically representable
        # required outputs. Constructor-positive conditions are unchanged.
        if any(not math.isfinite(v['expected_f64']) for v in checks.values()) or any(checks[k]['expected_f64']<=0 for k in ('Ai_m2','As_m2','G_pa')):
            excluded.append({'id':name,'reason':'at least one required ideal output nonfinite or positive area/G rounds to zero'})
            continue
        cases.append({'id':name,'inputs':inp,'checks':checks})
    return cases,excluded


def main():
    new,excluded=new_expectations()
    (HERE/'ADDITIONAL_EXPECTATIONS.json').write_text(json.dumps(new,indent=2,allow_nan=False)+'\n')
    (HERE/'EXCLUDED_GENERATOR_CASES.json').write_text(json.dumps(excluded,indent=2)+'\n')
    root_case=next(c for c in new if c['id']=='ROOT_OPPOSED_STRAINS')
    assert root_case['checks']['axial_membrane_pa']['expected_f64']==2.**24
    assert root_case['checks']['eigen_i_n']['expected_f64']>0
    assert root_case['checks']['eigen_j_n']['expected_f64']<0
    expectation_freeze={'status':'FROZEN_BEFORE_V2_INVOCATION','source_sha256':SOURCE_HASH,
        'original_oracle_sha256':hashlib.sha256((ORACLE/'independent_oracle.py').read_bytes()).hexdigest(),
        'v1_diagnostics_sha256':hashlib.sha256((ORACLE/'REFUTATION_V1/DIAGNOSTIC_EXPECTATIONS.json').read_bytes()).hexdigest(),
        'additional_expectations_sha256':hashlib.sha256((HERE/'ADDITIONAL_EXPECTATIONS.json').read_bytes()).hexdigest(),
        'additional_cases':len(new),'excluded_cases':len(excluded),
        'root_reduction':'sigma=E(epsilon-thermal)=2^24; Nw=S=3*pi*2^24; eigen_i=-EAs*thermal=+3*pi*2^23; eigen_j=-3*pi*2^23'}
    (HERE/'EXPECTATION_FREEZE.json').write_text(json.dumps(expectation_freeze,indent=2)+'\n')
    baseline=run_adapter('BASELINE',SOURCE,frozen['cases'],invalid['cases'])
    prior=json.loads((ORACLE/'REFUTATION_V1/DIAGNOSTIC_EXPECTATIONS.json').read_text())
    prior_result=run_adapter('V1_DIAGNOSTICS',SOURCE,prior,[])
    additional=run_adapter('ADDITIONAL',SOURCE,new,[])
    print('BASELINE',len(baseline['comparison_failures']),len(baseline['invalid_failures']),flush=True)
    print('V1_DIAGNOSTICS',len(prior_result['comparison_failures']),flush=True)
    print('ADDITIONAL',len(new),len(additional['comparison_failures']),flush=True)
    # Target changed V2 semantic locations, not preserved V1 mutation copies.
    original=SOURCE.read_text()
    poisson='let poisson_stress_pa = material.nu * pressure_trace_pa(annulus, pressure)?;'
    assert original.count(poisson)==2
    direct='fn direct_poisson_force(\n    annulus: ExactAnnulus,\n    material: IsotropicENu,\n    pressure: InternalDifferentialPressure,\n) -> Option<f64> {\n    checked_product3(\n        2.0,'
    assert original.count(direct)==1
    mutations={}
    for name,factor in [('OMIT_2NUP','0.0'),('DOUBLE_2NUP','2.0'),('REVERSE_2NUP','-1.0')]:
        changed=original.replace(poisson,f'let poisson_stress_pa = ({factor} * material.nu) * pressure_trace_pa(annulus, pressure)?;')
        changed=changed.replace(direct,direct[:-4]+{'0.0':'0.0,','2.0':'4.0,','-1.0':'-2.0,'}[factor])
        mutations[name]=changed
    effective='let effective_force_n = wall_force_n - fluid_force_n;'
    assert original.count(effective)==1
    mutations['SUBTRACT_CAP_FROM_WALL']=original.replace(effective,'let wall_force_n = wall_force_n - fluid_force_n;\n    '+effective)
    mutations['MEAN_RADIUS_FLUID_AREA']=original.replace('let value = pressure.p_pa * annulus.ai_m2;','let value = pressure.p_pa * PI * ((annulus.ri_m + annulus.ro_m) / 2.0).powi(2);')
    mutations['OMIT_FLUID_EFFECTIVE_TERM']=original.replace(effective,'let effective_force_n = wall_force_n;')
    results=[]
    for name,content in mutations.items():
        assert content!=original
        folder=HERE/name
        folder.mkdir(exist_ok=True)
        source=folder/'pressure_exact_mutant.rs'
        source.write_text(content)
        observed=run_adapter(name,source,frozen['cases'],[])
        assert observed['comparison_failures'],name
        results.append({'mutation':name,'class':'actual V2 implementation mutation in evidence-local copy',
            'killed':True,'scalar_failure_count':len(observed['comparison_failures']),
            'first_failure':observed['comparison_failures'][0],'source_sha256':observed['actual_source_sha256']})
    (HERE/'MUTATION_RESULTS.json').write_text(json.dumps(results,indent=2)+'\n')
    assert hashlib.sha256(SOURCE.read_bytes()).hexdigest()==SOURCE_HASH
    summary={'status':'FAIL_ACTIONABLE_NUMERIC_FINDINGS' if additional['comparison_failures'] or prior_result['comparison_failures'] or baseline['comparison_failures'] or baseline['invalid_failures'] else 'PASS',
        'baseline_valid_cases':114,'baseline_scalar_comparisons':1990,'baseline_invalid_fixtures':44,'baseline_invalid_operations':47,
        'baseline_comparison_failures':len(baseline['comparison_failures']),'baseline_invalid_failures':len(baseline['invalid_failures']),
        'v1_diagnostic_cases':5,'v1_diagnostic_failures':len(prior_result['comparison_failures']),
        'additional_case_count':len(new),'additional_failure_count':len(additional['comparison_failures']),
        'actual_v2_mutations_killed':len(results)}
    (HERE/'SUMMARY.json').write_text(json.dumps(summary,indent=2)+'\n')
    print(json.dumps(summary,indent=2),flush=True)


if __name__=='__main__':main()
