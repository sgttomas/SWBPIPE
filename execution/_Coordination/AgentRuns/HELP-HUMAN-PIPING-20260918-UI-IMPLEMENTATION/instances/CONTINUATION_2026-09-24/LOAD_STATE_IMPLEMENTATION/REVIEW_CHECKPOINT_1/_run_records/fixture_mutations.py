"""Mutation controls for review_fixture_recompute.py (scratch copies only)."""
import copy, json, subprocess, sys, tempfile, os
src, checker = sys.argv[1], sys.argv[2]
base = json.load(open(src))
def at(d, path):
    for k in path[:-1]: d = d[k]
    return d, path[-1]
muts = {
 "hot_N_value_1ulp": (["cases","signed_fit_states","variants","generic_reviewed","expected","hot","fixed_wall_N","value"], lambda v: float.fromhex(v.hex()) + (abs(v)*2**-52)),
 "cold_N_sign_exact": (["cases","signed_fit_states","variants","generic_reviewed","expected","cold","fixed_wall_N","exact","rational"], lambda v: "-"+v),
 "datum_ratio_25000": (["cases","thermal_datum_ratio","variants","generic_reviewed","expected","thermal_strain","exact","rational"], lambda v: "43/25000"),
 "G1_interpolated_like": (["cases","shared_material_serial_companion","variants","annular_companion","expected","G2","exact","rational"], lambda v: "41666666666"),
 "log_decimal_digit": (["cases","coefficient_definition","variants","generic_reviewed","expected","current_length_strain_forward","decimal"], lambda v: v[:30]+("1" if v[30]!="1" else "2")+v[31:]),
 "table_alpha_install": (["cases","thermal_datum_ratio","variants","generic_reviewed","inputs","table_points",1,"alpha","exact","rational"], lambda v: "13/1000000"),
}
for name,(path,f) in muts.items():
    d = copy.deepcopy(base); o,k = at(d,path); o[k] = f(o[k])
    with tempfile.NamedTemporaryFile("w", suffix=".json", delete=False) as t:
        json.dump(d, t); p = t.name
    r = subprocess.run([sys.executable, checker, p], capture_output=True, text=True)
    fails = json.loads(r.stdout)["failures"]
    print(name, "exit", r.returncode, "detected" if r.returncode == 1 else "MISSED", fails[:2])
    os.unlink(p)
# Consistent (exact+decimal+value) mutations: only the independent derivation (B) can detect them.
consistent = {
 "hot_N_sign_consistent": (["cases","signed_fit_states","variants","generic_reviewed","expected","hot","fixed_wall_N"], {"unit":"N","exact":{"kind":"rational","rational":"89976"},"decimal":"89976","value":89976.0}),
 "datum_strain_consistent": (["cases","thermal_datum_ratio","variants","annular_companion","expected","thermal_strain"], {"unit":"1","exact":{"kind":"rational","rational":"43/25000"},"decimal":"0.00172","value":0.00172}),
 "rot_far_Mz_consistent": (["cases","prescribed_rotation_all_fixed","variants","annular_companion","expected","far_Mz"], {"unit":"N*m","exact":{"kind":"rational_times_pi","rational":"3439"},"decimal":"10803.93713569530","value":10803.9371356953}),
}
for name,(path,new) in consistent.items():
    d = copy.deepcopy(base); o,k = at(d,path); o[k] = new
    with tempfile.NamedTemporaryFile("w", suffix=".json", delete=False) as t:
        json.dump(d, t); p = t.name
    r = subprocess.run([sys.executable, checker, p], capture_output=True, text=True)
    fails = json.loads(r.stdout)["failures"]
    print(name, "exit", r.returncode, "detected" if r.returncode == 1 else "MISSED", [f for f in fails if "decimal" not in f][:2])
    os.unlink(p)
