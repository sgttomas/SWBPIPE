import json, sys, hashlib
WT=sys.argv[1]
b=json.load(open(f"{WT}/rv90/base/projects/chirality-piping/fixtures/results/retained_precision_cases.json"))
c=json.load(open(f"{WT}/rv90/cand/projects/chirality-piping/fixtures/results/retained_precision_cases.json"))
def diff(x,y,p=()):
    out=[]
    if type(x)!=type(y): return [(p,'type')]
    if isinstance(x,dict):
        for k in sorted(set(x)|set(y)):
            if k not in x or k not in y: out.append((p+(k,),'key'))
            else: out+=diff(x[k],y[k],p+(k,))
    elif isinstance(x,list):
        if len(x)!=len(y): out.append((p,'len %d->%d'%(len(x),len(y))))
        for i,(u,v) in enumerate(zip(x,y)): out+=diff(u,v,p+(i,))
    else:
        if x!=y or (isinstance(x,float)!=isinstance(y,float)): out.append((p,'val'))
    return out
print("top keys base", list(b), "cand", list(c))
for k in ["version","provenance","arithmetic"]:
    print(k, "identical" if json.dumps(b[k],sort_keys=False)==json.dumps(c[k],sort_keys=False) else "DIFF")
print("cases", len(b["cases"]), len(c["cases"]))
print("mutations", len(b["mutations"]), len(c["mutations"]), "first268 identical:", json.dumps(b["mutations"])==json.dumps(c["mutations"][:268]))
print("must_pass", len(b["must_pass"]), len(c["must_pass"]), "identical:", json.dumps(b["must_pass"])==json.dumps(c["must_pass"]))
res=[]
for i,(x,y) in enumerate(zip(b["cases"],c["cases"])):
    d=diff(x,y)
    paths=[ '/'.join(map(str,p)) for p,_ in d]
    src=y["source"]; body=src["retained_precision"]["body"]
    ds=src["diagnostics"]
    a2_ok=True; info=[]
    for ci,case in enumerate(body["cases"]):
        cid=case["basis_ref"]["ref_id"]
        exact=[d["id"] for d in ds if isinstance(d.get("affected_refs"),list) and cid in d["affected_refs"] and not str(d.get("code","")).startswith("RETAINED_PRECISION_")]
        got=body["ordinary_attempts"][ci]["diagnostic_refs"]
        old=x["source"]["retained_precision"]["body"]["ordinary_attempts"][ci]["diagnostic_refs"]
        info.append((cid,old,got,exact==got))
        a2_ok &= exact==got
    # independent receipt hash check later via reader
    res.append((x.get("id"),paths,a2_ok,info))
    print(i, x.get("id"), "A2", a2_ok, "changed:", paths)
    for cid,old,got,ok in info: print("   ", cid, "old", old, "new", got, ok)
