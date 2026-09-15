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
SOURCE_HASH = '855144bf880370b5e40b57ecfc5e08fe3dec5929fc6d4a0d824ef0c9487bcd32'
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
    executable = build_dir / (name + '-adapter')
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


def main():
    baseline = run_adapter('BASELINE',SOURCE,frozen['cases'],invalid['cases'])
    print('BASELINE',json.dumps({k:baseline[k] for k in ['cases_checked','scalar_comparisons','invalid_fixture_count','invalid_operation_count','invalid_failures','comparison_failures']}),flush=True)
    # Independently calculated stress outputs must survive intermediate area/force
    # subnormal rounding; original policy, source equations and script unchanged.
    base = dict(ri_m=1.,ro_m=2.,E_pa=120.,nu=.25,p_pa=3.,thermal_strain=0.,epsilon_z=0.)
    diagnostic_inputs = [
        ('NORMAL_AREA_FLUID_FORCE_UNDERFLOW',dict(base,ri_m=2**-400,ro_m=2**-399,p_pa=2**-400,radius_witnesses={'midwall':1.5*2**-400})),
        ('SUBNORMAL_AREA_STRESS_RECONSTRUCTION',dict(base,ri_m=2**-537,ro_m=1.125*2**-537,radius_witnesses={'midwall':1.0625*2**-537})),
        ('MAX_ASSOCIATION_ROUNDUP_ELASTIC',dict(base,ri_m=2**-500,ro_m=2**-499,E_pa=2**600,p_pa=0.,epsilon_z=2**-78)),
        ('MAX_ASSOCIATION_ROUNDUP_THERMAL',dict(base,ri_m=2**-500,ro_m=2**-499,E_pa=2**600,p_pa=0.,thermal_strain=2**-78)),
        ('OVERFLOWING_FORCE_CANCELLATION_THERMAL',dict(base,ri_m=1.,ro_m=1024.,E_pa=2**1022,nu=-.75,p_pa=2**1022,thermal_strain=-1.5/(2**20-1))),
    ]
    diagnostic = [{'id':name,'inputs':inp,'checks':oracle.expected_values(inp)} for name,inp in diagnostic_inputs]
    (HERE/'DIAGNOSTIC_EXPECTATIONS.json').write_text(json.dumps(diagnostic,indent=2,allow_nan=False)+'\n')
    diagnosis = run_adapter('DIAGNOSTICS',SOURCE,diagnostic,[])
    print('DIAGNOSTICS',json.dumps(diagnosis['comparison_failures']),flush=True)
    original = SOURCE.read_text()
    poisson = 'let poisson_force_n = checked_product3(\n        2.0,'
    assert original.count(poisson)==2
    mutations = {
        'OMIT_2NUP':original.replace(poisson,'let poisson_force_n = checked_product3(\n        0.0,'),
        'DOUBLE_2NUP':original.replace(poisson,'let poisson_force_n = checked_product3(\n        4.0,'),
        'REVERSE_2NUP':original.replace(poisson,'let poisson_force_n = checked_product3(\n        -2.0,'),
        'SUBTRACT_CAP_FROM_WALL':original.replace('let wall_force_n = elastic_force_n + poisson_force_n;','let wall_force_n = elastic_force_n + poisson_force_n - fluid_force_n;'),
        'MEAN_RADIUS_FLUID_AREA':original.replace('let value = pressure.p_pa * annulus.ai_m2;','let value = pressure.p_pa * PI * ((annulus.ri_m + annulus.ro_m) / 2.0).powi(2);'),
        'OMIT_FLUID_EFFECTIVE_TERM':original.replace('let effective_force_n = wall_force_n - fluid_force_n;','let effective_force_n = wall_force_n;'),
    }
    mutation_results = []
    for name, content in mutations.items():
        assert content!=original,name
        folder=HERE/name
        folder.mkdir(exist_ok=True)
        source=folder/'pressure_exact_mutant.rs'
        source.write_text(content)
        observed=run_adapter(name,source,frozen['cases'],[])
        assert observed['comparison_failures'],name
        mutation_results.append({'mutation':name,'class':'actual implementation mutation in evidence-local copy',
                                 'killed':True,'scalar_failure_count':len(observed['comparison_failures']),
                                 'first_failure':observed['comparison_failures'][0],
                                 'source_sha256':observed['actual_source_sha256']})
    (HERE/'MUTATION_RESULTS.json').write_text(json.dumps(mutation_results,indent=2)+'\n')
    assert hashlib.sha256(SOURCE.read_bytes()).hexdigest()==SOURCE_HASH
    summary={'status':'FAIL_ACTIONABLE_NUMERIC_FINDINGS' if diagnosis['comparison_failures'] or baseline['comparison_failures'] or baseline['invalid_failures'] else 'PASS',
             'baseline_comparison_failures':len(baseline['comparison_failures']),
             'baseline_invalid_failures':len(baseline['invalid_failures']),
             'diagnostic_comparison_failures':len(diagnosis['comparison_failures']),
             'actual_implementation_mutations_killed':len(mutation_results)}
    (HERE/'SUMMARY.json').write_text(json.dumps(summary,indent=2)+'\n')
    print(json.dumps(summary,indent=2),flush=True)


if __name__=='__main__':main()
