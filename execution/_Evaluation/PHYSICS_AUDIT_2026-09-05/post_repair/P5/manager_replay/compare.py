from pathlib import Path
import json, math
p=Path(__file__).resolve().parent
w=next(x for x in p.parents if x.name=="chirality-piping")
a=w/"execution/_Evaluation/PHYSICS_AUDIT_2026-09-05"
checks=[]
def check(name,actual,expected,tol=1e-6):
    ok=abs(actual-expected)<=tol if isinstance(actual,(int,float)) and isinstance(expected,(int,float)) else actual==expected
    checks.append(dict(name=name,actual=actual,expected=expected,tolerance=tol if isinstance(expected,(int,float)) else None,pass_=ok))
def data(name,mode): return json.loads((p/"results"/f"{name}.{mode}.json").read_text())
def rows(name,mode):return {x["id"]:x for x in data(name,mode)["results"]}
def val(name,mode,key): return rows(name,mode)[key]["value"]
for mode in ["sparse","dense"]:
    for name,kind,comp in [("axial","force","axial"),("torsion","moment","torsion"),("shear_y","force","shear-y"),("shear_z","force","shear-z")]:
        for station in ["quarter-1","midspan","quarter-3"]:
            check(f"{mode}/{name}/{station}",abs(val(name,mode,f"result:{kind}:pipe-P-100:{station}:{comp}")),350)
    for station,shear,moment in [("quarter-1",150,112.5),("midspan",100,50),("quarter-3",50,12.5)]:
        check(f"{mode}/uniform/{station}/shear",abs(val("uniform",mode,f"result:force:pipe-P-100:{station}:shear-y")),shear)
        check(f"{mode}/uniform/{station}/moment",abs(val("uniform",mode,f"result:moment:pipe-P-100:{station}:bending-z")),moment)
    check(f"{mode}/uniform/tip",val("uniform",mode,"result:disp:node-N-110:uy"),0.08700500746613722)
    for name in ["missing_selected_alpha","missing_base_alpha","wind_duplicate","wind_overlap"]:
        check(f"{mode}/{name}/blocked",data(name,mode)["status"]["mechanics"]!="MECHANICS_SOLVED",True)
    for name in ["wind","wind_partial"]:
        check(f"{mode}/{name}/solved",data(name,mode)["status"]["mechanics"],"MECHANICS_SOLVED")
    combined=[r for r in rows("opposite_combinations_v2",mode).values() if "combination-R-sum" in r["id"] and (":disp:node-N-110" in r["id"] or ":reaction:" in r["id"])]
    check(f"{mode}/opposite/observed",len(combined)>0,True)
    for r in combined:check(f"{mode}/opposite/{r['id']}",r["value"],0)
    scaled=data("scaled_combinations.input",mode)
    for factor,direct in [("2","direct2.input"),("1000000","direct1e6.input")]:
        model=json.loads((a/"I1/supplement_v1/rounding/scaled_combinations.input.json").read_text())
        cid=next(c["id"] for c in model["combinations"] if c["terms"][0]["factor"]==float(factor))
        r=next(r for r in scaled["results"] if r.get("basis_ref",{}).get("ref_id")==cid and r["kind"]=="global_nodal_displacement_y" and r["entity_ref"]=="node:N-110")
        check(f"{mode}/precision/{factor}",r["value"],val(direct,mode,"result:disp:node-N-110:uy"),0)
    rr=rows("invented_nonlinear",mode); n=0
    for r in rr.values():
        basis=r.get("basis_ref") or {}
        if basis.get("ref_type")!="load_case":continue
        for ref in r.get("source_result_refs",[]):
            if ref.startswith("result:"):
                n+=1;check(f"{mode}/source/{r['id']}/{ref}",rr.get(ref,{}).get("basis_ref"),basis)
    check(f"{mode}/source/at_least_original20",n>=20,True)
    for name in ["pressure","pressure_fixed"]:
        old=json.loads((a/"I1/children/R/results"/f"{name}_{mode}.json").read_text()); new=rows(name,mode)
        for r in old["results"]:
            if r["id"].startswith("result:disp:") or ("end-i" in r["id"] or "end-j" in r["id"]) and r["id"].startswith(("result:force:","result:moment:","result:stress:")):
                check(f"{mode}/pressure_compat/{name}/{r['id']}",new.get(r["id"],{}).get("value"),r["value"])
for output in (p/"results").glob("*.json"):
    d=json.loads(output.read_text())
    if "results" in d:
        check(f"finite/{output.name}",all(isinstance(r["value"],(int,float)) and math.isfinite(r["value"]) for r in d["results"]),True)
out=dict(basis="Frozen original I1 expectations, elementary statics and output quantization floor; no repaired-output fitted expected values",checks=checks,passed=sum(x["pass_"] for x in checks),failed=sum(not x["pass_"] for x in checks))
(p/"COMPARISON.json").write_text(json.dumps(out,indent=2)+"\n")
print(json.dumps({k:v for k,v in out.items() if k!="checks"}));print(json.dumps([x for x in checks if not x["pass_"]],indent=2))
raise SystemExit(bool(out["failed"]))
