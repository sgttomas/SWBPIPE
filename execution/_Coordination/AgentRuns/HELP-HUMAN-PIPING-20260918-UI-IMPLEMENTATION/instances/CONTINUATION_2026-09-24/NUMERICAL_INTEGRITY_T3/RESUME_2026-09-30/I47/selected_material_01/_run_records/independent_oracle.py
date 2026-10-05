#!/usr/bin/env python3
"""I47 independent selected-source extension of the frozen I45 oracle.
Provenance: FROZEN_ORACLE_COPY.py and ORIGINS.json. No product imports.
Uses exact Fraction operations on same-run captured normalized material operands.
"""
from fractions import Fraction as Q
from pathlib import Path
import json,math,struct,sys
def bits(x): return struct.pack(">d",x).hex()
def decode(h):return struct.unpack(">d",bytes.fromhex(h))[0]
def pt(x):return (Q(x),Q(x))
def add(a,b):return (a[0]+b[0],a[1]+b[1])
def mul(a,b):
    x=[u*v for u in a for v in b];return min(x),max(x)
def div(a,b):
    assert b[0]>0
    x=[u/v for u in a for v in b];return min(x),max(x)
def hull(a,b):return min(a[0],b[0]),max(a[1],b[1])
def rootq(x):
    assert x>=0
    n=640;k=math.isqrt((x.numerator<<(2*n))//x.denominator);a=Q(k,1<<n)
    return (a,a) if a*a==x else (a,Q(k+1,1<<n))
def root(a):return rootq(a[0])[0],rootq(a[1])[1]
def atan_inv(q,n=450):
    s=Q(0);power=Q(1,q)
    for k in range(n):
        s+=(-1 if k%2 else 1)*power/(2*k+1);power/=q*q
    term=power/(2*n+1)
    return s,s+term
p=add(atan_inv(2),atan_inv(3));PI=mul(pt(4),p)
def ru(x):
    a=float(x)
    if Q(a)<x:a=math.nextafter(a,math.inf)
    return a
def allowance(n,s):
    a0=(2.0**-64)*max(abs(n),s);a1=a0*(1.0+2.0**-21)
    u0=(2.0**-53)*abs(n);u1=u0+2.0**-1074
    return a1+u1
def norm(x,y):return root(add(mul(x,x),mul(y,y)))
def source_pair(inp,kind,location,node,loaded,geometric):
    if not loaded:return pt(0)
    D,t,E,G=map(Q,[inp["D"],inp["t"],inp["E"],inp["G"]])
    if geometric:
        E,G=inp["source_E"],inp["source_G"]
        A=mul(PI,pt((D*D-(D-2*t)**2)/4))
        I=mul(PI,pt((D**4-(D-2*t)**4)/64))
        J=mul(pt(2),I);Z=div(I,pt(D/2));c=pt(D/2)
    else:
        A,I,J=map(pt,[inp["A"],inp["I"],inp["J"]])
        Z=hull(pt(inp["Z"]),div(I,pt(D/2)));c=pt(inp["c"])
    ux=div(pt(1),mul(pt(E),A))
    uy=div(pt(1),mul(pt(3*E),I))
    rx=div(pt(1),mul(pt(G),J));rz=div(pt(1),mul(pt(2*E),I))
    motions=[ux,uy,pt(0),rx,pt(0),rz]
    node_kinds=["global_nodal_displacement_x","global_nodal_displacement_y","global_nodal_displacement_z","global_nodal_rotation_x","global_nodal_rotation_y","global_nodal_rotation_z"]
    action_kinds=["element_local_axial_force","element_local_shear_force_y","element_local_shear_force_z","element_local_torsional_moment","element_local_bending_moment_y","element_local_bending_moment_z"]
    if kind in node_kinds:return pt(0) if node==0 else motions[node_kinds.index(kind)]
    if kind=="displacement_magnitude":return pt(0) if node==0 else norm(ux,uy)
    if kind=="support_reaction_component_v2":return pt(-1 if location in ["Fx","Fy","Mx","Mz"] else 0)
    if kind.startswith("support_reaction_"):return root(pt(2))
    station={"end_i":Q(0),"end_j":Q(1),"quarter_1":Q(1,4),"midspan":Q(1,2),"quarter_3":Q(3,4)}
    f=station.get(location,Q(0))
    if kind in action_kinds:
        component=action_kinds.index(kind)
        if location=="end_i":return pt([-1,-1,0,-1,0,-1][component])
        return pt([1,1,0,1,0,1-f][component])
    if kind=="element_local_axial_normal_stress":return div(pt(1),A)
    if kind=="element_local_bending_normal_stress_y":return pt(0)
    if kind=="element_local_bending_normal_stress_z":return div(pt(1-f),Z)
    if kind=="element_local_torsional_shear_stress":return div(c,J)
    if kind=="pipe_elastic_normal_stress_maximum_v2":return add(div(pt(1),A),div(pt(1),Z))
    raise ValueError(kind)
def distance_bounds(n,a):
    n=Q(n)
    upper=max(n-a[0],a[1]-n)
    lower=a[0]-n if n<a[0] else n-a[1] if n>a[1] else Q(0)
    return lower,upper
def normalized(row):
    y=row["value"];return y/1000.0 if row["unit"]=="mm" else y*1e6 if row["unit"]=="MPa" else y
def raw(a,row):
    return mul(a,pt(1000)) if row["unit"]=="mm" else div(a,pt(1e6)) if row["unit"]=="MPa" else a
def classify_kind(r):
    k=r["kind"]
    if k=="displacement_magnitude" or k.startswith("global_nodal_displacement"):return 0
    if k.startswith("global_nodal_rotation"):return 1
    if k.startswith("element_local_") and ("force" in k):return 2
    if k.startswith("element_local_") and "moment" in k:return 3
    if k=="support_reaction_component_v2":return 2 if r["metadata"]["component"].startswith("F") else 3
    if k=="support_reaction_force_magnitude_v2":return 2
    if k=="support_reaction_moment_magnitude_v2":return 3
def check(label,inp,rows,verdicts):
    loaded=label.endswith("loaded");node_ids=[]
    for r in rows:
        if r["kind"]=="displacement_magnitude":node_ids.append(r["entity_ref"])
    assert len(node_ids)==2 and len(rows)==74 and len(verdicts)==74
    scales=[0.0]*4
    for r in rows:
        k=classify_kind(r)
        if k is not None:
            if r["kind"].startswith("global_nodal_") and r["entity_ref"]==node_ids[0]:continue
            scales[k]=max(scales[k],abs(normalized(r)))
    tr,ro,fo,mo=scales
    scales=[max(tr,ro),max(ro,tr),max(fo,mo),max(mo,fo)] # actual L_body=1
    outcomes=[];false_claims=[];conservative=[]
    for i,(r,v) in enumerate(zip(rows,verdicts)):
        assert v["row"]==i
        n=normalized(r);assert bits(n)==v["normalized_bits"],(i,"normalization")
        if r["kind"]=="linear_solver_mode_basis":
            assert v["class"]=="nonquantity" and v["passed"];continue
        k=classify_kind(r)
        scale=scales[k] if k is not None else scales[2]/inp["A"]+(struct.unpack(">d",bytes.fromhex("4006a09e667f3bcd"))[0] if r["kind"]=="pipe_elastic_normal_stress_maximum_v2" else 1.0)*(scales[3]/inp["Z"])
        assert bits(scale)==v["scale_bits"],(i,"scale",bits(scale),v)
        node=node_ids.index(r["entity_ref"]) if r["entity_ref"] in node_ids else None
        input_derived=r["kind"].startswith("global_nodal_") and node==0
        cls="input" if input_derived else "absolute" if scale<2.0**-988 or abs(n)<(2.0**-34)*scale else "relative"
        assert cls==v["class"],(i,"class")
        loc=r.get("metadata",{}).get("location")
        if r["kind"]=="support_reaction_component_v2":loc=r["metadata"]["component"]
        g=source_pair(inp,r["kind"],loc,node,loaded,True)
        kk=source_pair(inp,r["kind"],loc,node,loaded,False)
        hl=max(distance_bounds(n,g)[0],distance_bounds(n,kk)[0])
        hu=distance_bounds(n,hull(g,kk))[1]
        tests=[]
        if cls=="input":tests=[("InputDerived",hl,hu,Q(0))]
        elif cls=="absolute":
            b=ru(Q(2)**-64*Q(scale)) if scale else 0.0
            if 0<scale<2.0**-988:b=ru(Q(b)+Q(ru(Q(2)**-53*abs(Q(n))))+Q(2)**-1074)
            assert bits(b)==v["bound_bits"],(i,"absolute bound")
            tests=[("Absolute",hl,hu,Q(b))]
        else:
            exact=Q(2)**-64*max(abs(Q(n)),Q(scale))*(1+Q(2)**-21)+Q(2)**-53*abs(Q(n))+Q(2)**-1074
            gr,kr=raw(g,r),raw(kk,r)
            hrl=max(distance_bounds(r["value"],gr)[0],distance_bounds(r["value"],kr)[0])
            hru=distance_bounds(r["value"],hull(gr,kr))[1]
            tests=[("SharperExact",hl,hu,exact),("SharperBinary64",hl,hu,Q(allowance(n,scale))),("DecimalSi",hl,hu,abs(Q(n))/10**9),("DecimalRaw",hrl,hru,abs(Q(r["value"]))/10**9)]
        assert all(lo>bound or hi<=bound for name,lo,hi,bound in tests), (i,"oracle interval indeterminate")
        if "predicates" in v:
            for index,(name,lo,hi,bound) in enumerate(tests):
                if lo>bound:assert v["predicates"][index] is False,(i,name,"false passing predicate")
                if hi<=bound:assert v["predicates"][index] is True,(i,name,"unexpected conservative predicate")
        certainly_fail=[name for name,lo,hi,b in tests if lo>b]
        certainly_pass=all(hi<=b for name,lo,hi,b in tests)
        if v["passed"] and certainly_fail:false_claims.append((i,r["id"],certainly_fail))
        if not v["passed"] and certainly_pass:conservative.append((i,r["id"],v["failed"]))
        outcomes.append({"row":i,"id":r["id"],"rust_passed":v["passed"],"rust_failed":v["failed"],"exact_failed":certainly_fail,
            "exact_passed":certainly_pass,"is_actual_source_output_miss":bool(certainly_fail),
            "relative_margin":str(hl/tests[0][3]) if tests[0][3] else None,
            "source_si":[str(x) for x in g],"represented_si":[str(x) for x in kk],
            "source_raw":[str(x) for x in raw(g,r)],"represented_raw":[str(x) for x in raw(kk,r)],
            "predicates":[{"name":name,"error_lower":str(lo),"error_upper":str(hi),"allowance":str(b),"passed":hi<=b} for name,lo,hi,b in tests],
            "class":cls,"scale_bits":bits(scale),"normalized_bits":bits(n)})
    assert not false_claims,false_claims
    first=next((v for v in outcomes if not v["rust_passed"]),None)
    if loaded:
        assert first and first["rust_failed"] in first["exact_failed"],("first refusal not independently reproduced",first)
    else:assert all(v["rust_passed"] for v in outcomes)
    return {"case":label,"mechanical_rows":len(outcomes),"false_claims":false_claims,"conservative_only_refusals":conservative,
        "first_refusal":first,"actual_misses":sum(bool(v["exact_failed"]) for v in outcomes),"all":outcomes}

def g5a_check(label,inp,rows,verdicts,data):
    assert data["precision"]==128
    cov=data["coverage"]
    assert len(cov)==1 and cov[0]["body"]==0
    loaded=label.endswith("loaded")
    assert cov[0]["data"]==loaded
    assert cov[0]["stop"]==[loaded]*4
    assert cov[0]["estimate"]==[loaded]*2 and cov[0]["charge"]==[loaded]*2
    for key,bound,wanted in [("stop",2.0**-64,list(range(4)) if loaded else []),("estimate",.25,[2,3] if loaded else []),("charge",1.0,[2,3] if loaded else [])]:
        assert sorted((b,k) for b,k,v in data[key])==[(0,k) for k in wanted]
        assert all(0<=decode(v)<=bound for b,k,v in data[key])
    assert len(data["resolution"])==1 and data["resolution"][0][0]==0
    E=[decode(v) for v in data["resolution"][0][1:]]
    assert all(math.isfinite(v) and v>=0 and (v!=0 or bits(v)=="0000000000000000") for v in E)
    assert len(data["theta"])==1 and data["theta"][0][0]==0 and 0<=data["theta"][0][1]<=.5
    assert len(data["B"])==int(loaded)
    if loaded: assert decode(data["B"][0][1])>0
    # Reconstruct each named source operation from exact rational operands then RN64.
    rn=lambda x:float(x)
    a,b=inp["nodes"]
    d=[rn(Q(b[i])-Q(a[i])) for i in range(3)]
    def operational_norm(d):
        squares=[rn(Q(v)*Q(v)) for v in d]
        q=rn(Q(squares[0])+Q(squares[1]))
        q=rn(Q(q)+Q(squares[2]))
        # For these actual specimens q=1 exactly; no approximation used as a sqrt oracle.
        assert Q(q)==1
        return 1.0
    first=operational_norm(d);assert first>1e-12
    inverse=rn(Q(1)/Q(first));normalized_axis=[rn(Q(v)*Q(inverse)) for v in d]
    L=operational_norm(d)
    ka=rn(Q(rn(Q(inp["E"])*Q(inp["A"])))/Q(L))
    kt=rn(Q(rn(Q(inp["G"])*Q(inp["J"])))/Q(L))
    op=data["operational"][0]
    assert op["operations"]==23 and not op["lost"]
    assert (bits(op["L"]),bits(op["ka"]),bits(op["kt"]))==(bits(L),bits(ka),bits(kt))
    scal=[0.0]*4
    node_ids=[r["entity_ref"] for r in rows if r["kind"]=="displacement_magnitude"]
    first_zero=None
    for i,r in enumerate(rows):
        kind=classify_kind(r)
        if kind is None:continue
        n=decode(verdicts[i]["normalized_bits"])
        if kind>=2 and E[kind-2]==0 and bits(n)!="0000000000000000":
            if first_zero is None:first_zero=i
        if not(r["kind"].startswith("global_nodal_") and r["entity_ref"]==node_ids[0]):
            scal[kind]=max(scal[kind],abs(n))
    if first_zero is not None:
        expected="Zero { row: "+str(first_zero)+" }"
        assert data["failure"]==expected
        assert first_zero==35 and rows[first_zero]["value"]==0 and bits(rows[first_zero]["value"])=="8000000000000000"
        assert not data["full_case"]
        return {"gate":"zero","row":first_zero,"id":rows[first_zero]["id"],"raw_bits":bits(rows[first_zero]["value"]),"operational_ops":23,"failure_reproduced":True}
    tr,ro,fo,mo=scal;scal=[max(tr,ro),max(ro,tr),max(fo,mo),max(mo,fo)]
    hat=[max(E[0],E[1]),max(E[1],E[0])]
    upper=[rn(Q(h)*Q(decode("3ff0000000001000"))) for h in hat]
    assert all(upper[j]>=scal[j+2] for j in range(2))
    norms=[]
    for kind in [0,1]:
        ends=[]
        for node in node_ids:
            prefix="global_nodal_displacement_" if kind==0 else "global_nodal_rotation_"
            values=[abs(normalized(next(r for r in rows if r["entity_ref"]==node and r["kind"]==prefix+axis))) for axis in ["x","y","z"]]
            ends.append(rn(Q(rn(Q(values[0])+Q(values[1])))+Q(values[2])))
        norms.append(rn(Q(ends[0])+Q(ends[1])))
    lower=[]
    for j,k in enumerate([ka,kt]):
        threshold=rn(Q(2)**-59*Q(scal[j]))
        lower.append(0.0 if norms[j]<=threshold else rn(Q(k)*Q(rn(Q(norms[j])-Q(rn(Q(2)**-60*Q(scal[j])))))))
    assert all(upper[j]>=lower[j] for j in range(2))
    assert data["failure"] is None and not data["full_case"] # Numeric recipe refusals independently checked above.
    return {"gate":"pass","operational_ops":23,"lower":lower,"upper":upper,"failure_reproduced":True}

def select_source(rec):
    s=rec['selection'];v=s['values'];req=rec['request'];inp=rec['input']
    assert s['case']==req['model']['load_cases'][0]['id']
    assert s['material_ordinal']==0 and s['material']==req['materials'][0]['id']
    material=req['materials'][0];points=material['temperature_points'];base=s['base'][0]
    assert (base['e'],base['g'])==(bits(material['elastic_modulus']['value']),bits(material['shear_modulus']['value']))
    trace={}
    if v['kind']=='point':
        assert v['ordinal']==0 and s['point_ids']==[points[0]['id'],None]
        assert req['model']['load_cases'][0]['modulus_basis_ref']==points[0]['id']
        p=points[v['ordinal']]
        assert (v['e'],v['g'])==(bits(p['elastic_modulus']['value']),bits(p['shear_modulus']['value']))
        E,G=Q(decode(v['e'])),Q(decode(v['g']))
        ehat,ghat=decode(v['e']),decode(v['g'])
        assert s['alpha']==bits(p['thermal_expansion_coefficient']['value'])
        assert s['source_alpha']==[s['alpha'],None]
    else:
        assert v['kind']=='interpolated' and (v['lower'],v['upper'])==(1,2)
        lo,hi=points[1:3]
        assert s['point_ids']==[lo['id'],hi['id']]
        tl,t,th=[Q(decode(v[k])) for k in ['t_lo','t','t_hi']]
        assert (tl,t,th)==(293,303,313) and tl<t<th
        assert v['t']==bits(req['model']['load_cases'][0]['modulus_basis_temperature']['value'])
        assert [v['t_lo'],v['t_hi']]==[bits(p['temperature']['value']) for p in [lo,hi]]
        h=th-tl;assert h>0
        sources=[]
        for prop,key in [('elastic_modulus','e'),('shear_modulus','g')]:
            a,b=Q(decode(v[key+'_lo'])),Q(decode(v[key+'_hi']))
            assert [v[key+'_lo'],v[key+'_hi']]==[bits(p[prop]['value']) for p in [lo,hi]]
            assert a>0 and b>0
            products=[th*a,-t*a,t*b,-tl*b]
            numerator=sum(products,Q(0));assert numerator>0
            x=numerator/h;assert x==((th-t)*a+(t-tl)*b)/h
            # Exactly simulate the existing resolver's separated binary64 affine operations.
            weight=float(Q(float(t-tl))/Q(float(th-tl)))
            hat=float(a+Q(float(Q(weight)*Q(float(b-a)))))
            assert bits(hat)==v[key+'_hat']
            sources.append(x)
            trace[key]={'products':[str(z) for z in products],'numerator':str(numerator),'denominator':str(h),'source':str(x),'represented_bits':v[key+'_hat'],'exact_minus_represented':str(x-Q(hat))}
        E,G=sources;ehat,ghat=decode(v['e_hat']),decode(v['g_hat'])
        assert s['source_alpha']==[bits(p['thermal_expansion_coefficient']['value']) for p in [lo,hi]]
        alpha_lo,alpha_hi=[Q(p['thermal_expansion_coefficient']['value']) for p in [lo,hi]]
        alpha=float(alpha_lo+Q(float(Q(weight)*Q(float(alpha_hi-alpha_lo)))))
        assert s['alpha']==bits(alpha)
    assert E>0 and G>0
    assert (bits(inp['E']),bits(inp['G']))==(bits(ehat),bits(ghat))
    assert bits(inp['E'])!=base['e'] and bits(inp['G'])!=base['g']
    inp.update(source_E=E,source_G=G)
    return {'kind':v['kind'],'source_E':str(E),'source_G':str(G),'interpolation':trace}

if __name__=='__main__':
    records=[];current=None;controls=[];refusals=[]
    keys={'REQUEST':'request','INPUT':'input','ROWS':'rows','VERDICTS':'verdicts','G5A_DATA':'g5a','SELECTION':'selection','ANCILLARY':'ancillary','NATIVE_ROWS':'native_rows','NATIVE_IDENTITY':'native_identity'}
    for line in Path(sys.argv[1]).read_text().splitlines():
        if line.startswith('I47_CASE '):
            current={'label':line.split(' ',1)[1]};records.append(current)
        elif line.startswith('I47_CONTROL '):controls.append(json.loads(line.split(' ',1)[1]))
        elif line.startswith('I47_REFUSAL '):refusals.append(json.loads(line.split(' ',1)[1]))
        elif line.startswith('I47_'):
            tag,payload=line.split(' ',1)
            if tag[4:] in keys:
                assert current is not None
                current[keys[tag[4:]]]=json.loads(payload)
    assert [r['label'] for r in records]==['point-zero','point-loaded','interpolated-zero','interpolated-loaded']
    selections=[select_source(r) for r in records]
    results=[check(r['label'],r['input'],r['rows'],r['verdicts']) for r in records]
    gates=[g5a_check(r['label'],r['input'],r['rows'],r['verdicts'],r['g5a']) for r in records]
    assert len(controls)==2 and len(refusals)==4
    assert all(r['calls'][0:2]==[1,0] for r in refusals)
    output={'provenance':'I45 product_vertical_repair_03 frozen oracle, extended here only','pi_terms_each':450,'root_quantum_bits':640,'selections':selections,'checks':results,'g5a':gates,'controls':[x['name'] for x in controls],'refusals':[x['name'] for x in refusals]}
    Path(sys.argv[2]).write_text(json.dumps(output,indent=2)+'\n')
    print(json.dumps({'cases':[{'case':r['case'],'rows':r['mechanical_rows'],'actual_misses':r['actual_misses'],'conservative_only_refusals':r['conservative_only_refusals'],'false_claims':r['false_claims']} for r in results],'g5a':gates,'selections':selections},indent=2))
