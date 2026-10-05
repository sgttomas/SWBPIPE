"""RV84: T25 stage S4 (source::commitment, PP/source_receipt/source.rs:252-407 at 5ae5fe4f0f),
re-derived field by field from RV84's reading, versus the packet's payload/functions/plan facts.
Counts at the D1 caps; strides ASSUMED (packet values). Usage: python3 rv84_t25_s4.py"""
import json
n, m, g, s, r, l = 32, 32, 32, 32, 192, 192
N = 6 * n; C = 144 * m + s; k = min(N, r); Fn = 42 * m + N + s + 6 * g; P = 7 * n + 51 * m + 8 * g + 3
ID, RID = 128, 1024
L_ID = 1_797_413           # identity JSON length bound at the caps (packet T07 e_id; Snapshot.identity)
sV, NODE = 32, 736
def facts(**kw):
    d = dict(arr=0, obj=0, ent=0, arrays=0, strb=0, keyb=0, nums=0); d.update(kw); return d
def add(*fs):
    o = facts()
    for f in fs:
        for k_ in o: o[k_] += f[k_]
    return o
def tree(f): return sV * f["arr"] + NODE * (f["obj"] + f["ent"] // 5) + f["strb"] + f["keyb"]
def parsed(f): return sV * (4 * f["arrays"] + 2 * f["arr"]) + NODE * (f["obj"] + f["ent"] // 5) + f["strb"] + 2 * f["keyb"]
def e(f, eps): return eps * f["strb"] + f["keyb"] + 8 * (f["arr"] + f["ent"] + 1) + 24 * f["nums"]
def hash_route(f, eps, lmax):
    t = e(f, eps)
    return tree(f) + max(128, 2 * t) + parsed(f) + max(8, 2 * t) + max(128, 2 * (eps * lmax + 2)) + 8_000
def payload(U):
    top = facts(obj=1, ent=17, strb=5 + 64 + ID + L_ID, keyb=17 * 25)
    dof_map = facts(arr=N, arrays=1, obj=N, ent=4 * N, strb=N * (ID + 2 + 3 + 3), keyb=N * 40)
    agg = facts(arr=N + N * N, arrays=N + 1, strb=16 * N * N)             # stiffness_aggregate_bits
    force_agg = facts(arr=N, arrays=1, strb=16 * N)
    sterms = facts(arr=C, arrays=1, obj=2 * C, ent=6 * C, strb=C * (6 + ID + 16), keyb=C * 28, nums=2 * C)
    fterms = facts(arr=l, arrays=1, obj=2 * l, ent=5 * l, strb=l * (18 + ID + 16), keyb=l * 28, nums=l)
    free = facts(arr=N, arrays=1, nums=N)
    presc = facts(arr=k, arrays=1, obj=k, ent=3 * k, strb=k * (16 + ID), keyb=k * 25, nums=k)
    frames = facts(arr=m + m * 488, arrays=1 + 43 * m, obj=3 * m, ent=19 * m, strb=m * 7_536, keyb=m * 19 * 24, nums=12 * m)
    springs = facts(arr=s, arrays=1, obj=s, ent=6 * s, strb=s * (3 * ID + 32), keyb=s * 6 * 20, nums=s)
    sup = facts(arr=l, arrays=1, obj=l, ent=5 * l, strb=l * (2 * ID + 16), keyb=l * 5 * 22, nums=2 * l)
    fam = facts(arr=m + g + l, arrays=8, obj=1, ent=8, strb=(m + g + l) * ID, keyb=8 * 18)
    lowered = facts(arr=Fn + U, arrays=1 + 2 * U, obj=U, ent=3 * U, strb=16 * U, keyb=U * 32, nums=U)
    return add(top, dof_map, agg, force_agg, sterms, fterms, free, presc, frames, springs, sup, fam, lowered)
def functions(U, R):
    per_fn = facts(arr=Fn, arrays=1 + 3 * Fn, obj=3 * Fn, ent=17 * Fn, strb=478 * Fn, keyb=Fn * 17 * 22)
    rids = facts(arr=R, strb=R * RID)
    atoms = facts(arr=U, arrays=U, obj=U, ent=3 * U, strb=151 * U, keyb=U * 19, nums=U)
    return add(per_fn, rids, atoms)
def plan(U, R):
    recipes = facts(arr=R + 4 * R, arrays=1 + 3 * R, obj=4 * R, ent=12 * R, strb=R * (RID + 40 + 4 * RID + 4 * 160), keyb=R * 12 * 20)
    obs = facts(arr=P, arrays=1, strb=P * RID)
    top = facts(obj=1, ent=5, strb=5 + 64, keyb=5 * 24)
    return add(functions(U, R), recipes, obs, top)
out = {}
for U in (16_384, 41_760):
    R = P
    pl, fn, pn = payload(U), functions(U, R), plan(U, R)
    for eps in (2, 6):
        s4 = tree(pl) + hash_route(pl, eps, L_ID) + tree(fn) + tree(pn) + hash_route(pn, eps, 2 * RID)
        out[f"U{U}_EPS{eps}"] = {"payload_facts": pl, "payload_e": e(pl, eps), "plan_e": e(pn, eps),
                                 "S4_core_rv84": s4}
out["packet_S4_stage"] = {"EPS2": 511_364_855, "EPS6": 783_153_911, "note": "includes carried case outputs 51,639,470"}
out["packet_payload_facts"] = {"arr": 63200, "obj": None}
print(json.dumps(out, indent=1))
