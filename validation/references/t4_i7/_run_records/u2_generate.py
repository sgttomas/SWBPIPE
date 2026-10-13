"""T4-I7: generate the frozen independent VP-STATIC references for T4-U2 (pressure through
realized bends) by the direct unit-load (flexibility) method of u2_engine.py.

Standard library only. Run with the shared host's Python in isolated mode:
    python -I u2_generate.py <output_dir>
Writes <output_dir>/u2_reference_cases.json and <output_dir>/u2_document_sketches.json and
prints a summary. No product module is imported and no product value is read.
"""
import json
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import u2_engine as E  # noqa: E402
from u2_engine import D, ZERO, ONE, v, add, sub, scl, dot, cross, norm  # noqa: E402

SIG = 20  # significant digits written
SNAP = D("1e-40")  # relative to the group's characteristic scale: below this a value is exact zero

STATIONS = [("end_i", "0"), ("quarter_1", "0.25"), ("midspan", "0.5"), ("quarter_3", "0.75"), ("end_j", "1")]

# ------------------------------------------------------------------ authored parameter sets
SEC_L = dict(od="0.1683", wall="0.00711", mill="0.000889")
MAT_L = dict(E="2.0e11", nu="0.3", alpha="1.2e-5")
P_L = "5.0e6"
DT_L = "50"
RHO_S, RHO_C, G_ACC = "7850", "1000", "9.80665"
SEC_C = dict(od="0.2191", wall="0.0081", mill="0")
MAT_C = dict(E="1.95e11", nu="0.3", alpha="1.2e-5")
P_C = "2.5e6"

# tangency rule T4-U2 implements (WI, repair 01 S-2; T4-I11 D-D): theta = atan2(|t_in x t_out|, t_in . t_out)
# <= ALPHA_TAN at bend-adjacent junctions; ALPHA_TAN is provisional
ALPHA_TAN = "1.0e-3"
MITRE_CODE = "PRESSURE_REGION_MITRE_UNSUPPORTED"
KINK_DX = {"admitted": 3.248, "refused": 3.242}  # D.x: tan(theta) = (3.25 - D.x)/4 = 5e-4 and 2e-3

# skew rotation from the unit quaternion (4, 1, 2, 2)/5: exact rational matrix /25
Q25 = [[9, -12, 20], [20, 15, 0], [-12, 16, 15]]


def fmt(x):
    x = D(x)
    if x == 0:
        return "0"
    return format(x, ".%de" % (SIG - 1))


def fmtv(vec):
    """Format a geometric vector, writing components below 1e-40 of its largest component (or of 1) as 0."""
    sc = max([ONE] + [abs(D(c)) for c in vec])
    return [fmt(ZERO if abs(D(c)) < SNAP * sc else c) for c in vec]


def f64(x):
    """binary64 rounding of an exact decimal/rational, returned as float."""
    return float(D(x))


def dec_of_float(x):
    return D(x)  # exact binary value of the float


def self_weight_w():
    sec = E.Section(**SEC_L)
    w = (sec.As * D(RHO_S) + sec.Ai * D(RHO_C)) * D(G_ACC)
    return format(w, ".16e")  # 17 significant digits: the authored input (exact target)


W_L = self_weight_w()


# ------------------------------------------------------------------ geometries (authored, SI, local)
def geom_L(kink=None, reverse=False):
    nodes = [("node:A", (0.0, 0.0, 0.0)), ("node:B", (3.0, 0.0, 0.0)), ("node:C", (3.25, 0.25, 0.0)),
             ("node:D", ((KINK_DX[kink] if kink else 3.25), 4.25, 0.0))]
    mem = [dict(pid="pipe:S1", kind="straight", i="node:A", j="node:B", yref=(0.0, 0.0, 1.0)),
           dict(pid="pipe:BEND", kind="arc", i="node:B", j="node:C", yref=(1.0, -1.0, 0.0), R="0.25",
                comp="component:bend-1"),
           dict(pid="pipe:S2", kind="straight", i="node:C", j="node:D", yref=(0.0, 0.0, 1.0))]
    if reverse:
        mem[1].update(i="node:C", j="node:B")
        mem[2].update(i="node:D", j="node:C")
    return dict(name="L", nodes=nodes, members=mem, sec=SEC_L, mat=MAT_L)


def geom_U():
    pts = [("node:A", (0.0, 0.0)), ("node:B1", (4.0, 0.0)), ("node:C1", (4.25, 0.25)), ("node:B2", (4.25, 2.75)),
           ("node:C2", (4.5, 3.0)), ("node:B3", (6.5, 3.0)), ("node:C3", (6.75, 2.75)), ("node:B4", (6.75, 0.25)),
           ("node:C4", (7.0, 0.0)), ("node:D", (10.0, 0.0))]
    nodes = [(n, (x, y, 0.0)) for n, (x, y) in pts]
    yb = [(1.0, -1.0, 0.0), (-1.0, 1.0, 0.0), (1.0, 1.0, 0.0), (-1.0, -1.0, 0.0)]
    mem = []
    for s in range(5):
        mem.append(dict(pid="pipe:S%d" % (s + 1), kind="straight", i=nodes[2 * s][0], j=nodes[2 * s + 1][0],
                        yref=(0.0, 0.0, 1.0)))
        if s < 4:
            mem.append(dict(pid="pipe:BEND%d" % (s + 1), kind="arc", i=nodes[2 * s + 1][0], j=nodes[2 * s + 2][0],
                            yref=yb[s], R="0.25", comp="component:bend-%d" % (s + 1)))
    return dict(name="U", nodes=nodes, members=mem, sec=SEC_L, mat=MAT_L)


def geom_CBPT():
    nodes = [("node:A", (1.4, 0.0, 0.0)), ("node:B", (0.0, 1.4, 0.0))]
    mem = [dict(pid="pipe:ARC", kind="arc", i="node:A", j="node:B", yref=(1.0, 1.0, 0.0), R="1.4",
                comp="component:bend-cbpt")]
    return dict(name="CBPT", nodes=nodes, members=mem, sec=SEC_C, mat=MAT_C)


def transform(geom, skew=False, x0=None):
    g = json.loads(json.dumps(geom))
    def rot(p):
        if not skew:
            return tuple(D(c) for c in p)
        pd = [dec_of_float(c) for c in p]
        return tuple(sum(D(Q25[r][k]) * pd[k] for k in range(3)) / 25 for r in range(3))
    new_nodes = []
    for nid, p in g["nodes"]:
        q = rot(p)
        if x0 is not None:
            q = (q[0] + D(x0), q[1], q[2])
        new_nodes.append((nid, tuple(f64(c) for c in q)))
    g["nodes"] = new_nodes
    for m in g["members"]:
        m["yref"] = tuple(f64(c) for c in rot(m["yref"]))
    return g


# ------------------------------------------------------------------ cases
def case_list():
    cases = []

    def add_case(cid, geom, k, *, p, dT="0", weight=False, tA=True, tD=True, anchorD=False, units="SI",
                 desc="", core=None, transform_of=None, rebuilds=None, family=None):
        cases.append(dict(id=cid, geom=geom, k=k, p=p, dT=dT, weight=weight, tA=tA, tD=tD, anchorD=anchorD,
                          units=units, desc=desc, core=core, transform_of=transform_of, rebuilds=rebuilds,
                          family=family))

    variants_free = [
        ("P", dict(), "closed transferring terminals, pressure only"),
        ("SEPA", dict(tA=False), "closure at the anchored end A separately supported; pressure only"),
        ("SEPD", dict(tD=False), "closure at the free end D separately supported; pressure only"),
        ("PT", dict(dT=DT_L), "pressure plus thermal strain"),
        ("PW", dict(weight=True), "pressure plus self-weight"),
        ("ALL", dict(tD=False, dT=DT_L, weight=True), "D closure separately supported, thermal and self-weight"),
        ("PTW", dict(dT=DT_L, weight=True), "both terminals transferring; pressure, thermal and self-weight in one "
                                            "case (plan section 2 headline case; repair 01 S-3)"),
    ]
    variants_anch = [v_ for v_ in variants_free if v_[0] != "SEPA"]
    for k in ("1", "2"):
        for tag, kw, d in variants_free:
            add_case("U2-L-FREE-%s-K%s" % (tag, k), geom_L(), k, p=P_L, desc="L line anchored at A, free at D: " + d,
                     family="L-FREE", **kw)
        for tag, kw, d in variants_anch:
            add_case("U2-L-ANCH-%s-K%s" % (tag, k), geom_L(), k, p=P_L, anchorD=True,
                     desc="L line anchored at A and D: " + d, family="L-ANCH", **kw)
        for tag, kw, d in variants_anch:
            add_case("U2-U-ANCH-%s-K%s" % (tag, k), geom_U(), k, p=P_L, anchorD=True,
                     desc="four-bend U-loop anchored at A and D: " + d, family="U-ANCH", **kw)
        add_case("MECH-CURVED-BEND-EXACT-PRESSURE-ARC-K%s" % k, geom_CBPT(), k, p=P_C,
                 desc="rebuilt CBPT: anchored-free quarter bend, closed transferring terminals, E/nu",
                 rebuilds="MECH-CURVED-BEND-PRESSURE-THRUST-ARC", family="CBPT")
    add_case("U2-L-KINK-FREE-P-K2", geom_L(kink="admitted"), "2", p=P_L,
             desc="L line with S2 kinked by atan(5e-4) rad at C (about alpha_tan/2); free; pressure only",
             family="KINK")
    add_case("U2-L-KINK-ANCH-P-K2", geom_L(kink="admitted"), "2", p=P_L, anchorD=True,
             desc="L line with S2 kinked by atan(5e-4) rad at C (about alpha_tan/2); anchored at A and D; pressure "
                  "only", family="KINK")
    add_case("U2-L-MITRE-REFUSED-P-K2", geom_L(kink="refused"), "2", p=P_L, anchorD=True,
             desc="L line with S2 kinked by atan(2e-3) rad at C (about 2 alpha_tan); a refusal control: no values",
             family="MITRE-REFUSAL")
    add_case("U2-L-ANCH-ALL-K2-REV", geom_L(reverse=True), "2", p=P_L, anchorD=True, tD=False, dT=DT_L, weight=True,
             desc="as U2-L-ANCH-ALL-K2 with the bend and S2 authored against the traversal (orientation control)",
             family="L-ANCH")
    cores = ["U2-L-FREE-P-K2", "U2-L-FREE-ALL-K2", "U2-L-ANCH-ALL-K2", "U2-U-ANCH-ALL-K2",
             "MECH-CURVED-BEND-EXACT-PRESSURE-ARC-K2", "U2-L-ANCH-PTW-K2"]
    base = {c["id"]: c for c in cases}
    tforms = [("SKEW", dict(skew=True)), ("X5E6", dict(x0="5e6")), ("X7P3E6", dict(x0="7.3e6")),
              ("SKEW-X5E6", dict(skew=True, x0="5e6")), ("SKEW-X7P3E6", dict(skew=True, x0="7.3e6")),
              ("MM-MPA", dict(units="mm"))]
    for cid in cores:
        b = base[cid]
        b["core"] = True
        for tag, tk in tforms:
            c = dict(b)
            c["id"] = cid + "-" + tag
            c["core"] = False
            c["transform_of"] = cid
            c["transform"] = tag
            if tag == "MM-MPA":
                c["units"] = "mm"
                c["geom"] = b["geom"]
            else:
                c["geom"] = transform(b["geom"], skew=tk.get("skew", False), x0=tk.get("x0"))
            c["desc"] = b["desc"] + " [transform %s]" % tag
            cases.append(c)
    return cases


# ------------------------------------------------------------------ model build
def build_chain(case, polygon_n=0):
    g = case["geom"]
    sec = E.Section(**g["sec"])
    mat = E.Material(**g["mat"])
    coords = {nid: tuple(dec_of_float(c) for c in p) for nid, p in g["nodes"]}
    node_ids = [nid for nid, _ in g["nodes"]]
    members = []
    for m in g["members"]:
        yref = tuple(dec_of_float(c) for c in m["yref"])
        if m["kind"] == "straight":
            members.append(E.Member(m["pid"], "straight", m["i"], m["j"], coords[m["i"]], coords[m["j"]], sec, mat,
                                    yref))
        else:
            members.append(E.Member(m["pid"], "arc", m["i"], m["j"], coords[m["i"]], coords[m["j"]], sec, mat, yref,
                                    R=m["R"], k=case["k"]))
    chain = E.Chain(node_ids, coords, members)
    if polygon_n:
        chain = polygonize(chain, polygon_n, sec, mat)
    return chain, sec, mat


def polygonize(chain, n, sec, mat):
    node_ids = [chain.node_ids[0]]
    coords = {chain.node_ids[0]: chain.xn[0]}
    mems = []
    for mi, mem in enumerate(chain.members):
        a = node_ids[-1]
        b = chain.node_ids[mi + 1]
        if mem.kind == "straight":
            coords[b] = chain.xn[mi + 1]
            mems.append(E.Member(mem.pid, "straight", a, b, coords[a], coords[b], sec, mat, mem.yref))
            node_ids.append(b)
        else:
            prev = a
            for q in range(1, n + 1):
                nid = b if q == n else "%s:poly%d" % (mem.pid, q)
                coords[nid] = chain.xn[mi + 1] if q == n else mem.pos(mem.length * q / n)
                seg = E.Member("%s:seg%d" % (mem.pid, q), "straight", prev, nid, coords[prev], coords[nid], sec,
                               mat, mem.zhat)
                seg.k = mem.k  # the arc's k scales the segments' bending energy (torsion and axial unscaled)
                mems.append(seg)
                node_ids.append(nid)
                prev = nid
    return E.Chain(node_ids, coords, mems)


def load_case(case, sec, **mut):
    w = W_L if case["weight"] else "0"
    return E.LoadCase(p=case["p"], dT=case["dT"], w=w, gdir=v(0, 0, -1), transfer_A=case["tA"],
                      transfer_D=case["tD"], anchor_D=case["anchorD"], **mut)


# ------------------------------------------------------------------ evaluation and records
def station_record(sol, chain, mi, f, sec, P):
    mem = chain.members[mi]
    X, F, M = E.j_side_action(sol, chain, mi, f)
    fr = mem.frame_at_authored(f)
    N, Vy, Vz = (dot(F, e) for e in fr)
    T, My, Mz = (dot(M, e) for e in fr)
    return dict(N_w=N, S=N - P, sigma_m=N / sec.As, V_y=Vy, V_z=Vz, T=T, M_y=My, M_z=Mz,
                sigma_b_y=My / sec.Z, sigma_b_z=Mz / sec.Z, tau_t=T * sec.ro / sec.J), (F, M)


def end_rows(F, M, frame, sign):
    comps = [sign * dot(F, e) for e in frame] + [sign * dot(M, e) for e in frame]
    keys = ["F_x", "F_y", "F_z", "M_x", "M_y", "M_z"]
    d = dict(zip(keys, comps))
    d["wall_axial_end_action"] = comps[0]
    return d


GROUP_OF = {
    "ux": "displacement", "uy": "displacement", "uz": "displacement",
    "rx": "rotation", "ry": "rotation", "rz": "rotation",
    "N_w": "wall_axial_force", "S": "effective_axial_force", "sigma_m": "membrane_stress",
    "V_y": "shear_force", "V_z": "shear_force", "T": "section_moment", "M_y": "section_moment",
    "M_z": "section_moment", "sigma_b_y": "bending_torsion_stress", "sigma_b_z": "bending_torsion_stress",
    "tau_t": "bending_torsion_stress",
}
UNIT_OF_GROUP = {
    "displacement": "m", "rotation": "rad", "support_force": "N", "support_moment": "N*m",
    "remote_closure_force": "N", "wall_axial_force": "N", "effective_axial_force": "N", "membrane_stress": "Pa",
    "shear_force": "N", "section_moment": "N*m", "bending_torsion_stress": "Pa",
    "elastic_end_force_chord_frame": "N", "lame_stress": "Pa",
}


def end_group(key, frame_name):
    """Tolerance group of an end-row component: axial with the wall force, transverse with the shears,
    moments with the section moments; chord-frame forces (an information frame mixing axial and
    transverse) in their own group."""
    if key in ("wall_axial_end_action", "F_x") and frame_name != "chord_frame":
        return "wall_axial_force"
    if key.startswith("F"):
        return "end_action_force_chord_frame" if frame_name == "chord_frame" else "shear_force"
    return "section_moment"


def evaluate(case, mutation=None):
    chain, sec, mat = build_chain(case)
    lc = load_case(case, sec, **(mutation or {}))
    sol = E.analyse(chain, lc)
    return chain, sec, mat, lc, sol


def record_case(case):
    chain, sec, mat, lc, sol = evaluate(case)
    P = lc.p * sec.Ai
    EAs = mat.E * sec.As
    eps_nu = -2 * mat.nu * P / EAs
    eps_p = (1 - 2 * mat.nu) * P / EAs
    eps_th = mat.alpha * lc.dT
    wv = lc.w
    total_len = sum(m.length for m in chain.members)
    xA = chain.xn[0]
    Lc = max(norm(sub(x, xA)) for x in chain.xn)
    F0 = max(P, wv * total_len)
    chars = {
        "displacement": (abs(eps_p) + abs(eps_th)) * Lc + F0 * Lc ** 3 / (mat.E * sec.I),
        "support_force": F0, "support_moment": F0 * Lc, "remote_closure_force": F0,
        "wall_axial_force": F0, "effective_axial_force": F0, "membrane_stress": F0 / sec.As,
        "shear_force": F0, "section_moment": F0 * Lc, "bending_torsion_stress": F0 * Lc / sec.Z,
        "elastic_end_force_chord_frame": F0, "lame_stress": lc.p if lc.p else ONE,
    }
    chars["rotation"] = chars["displacement"] / Lc
    raw = []  # (group, setter) for snapping/zero-scale

    def put(dct, key, val, group):
        dct[key] = val
        raw.append((group, dct, key))

    out = dict(nodes={}, supports={}, members={}, terminals={})
    for ni, nid in enumerate(chain.node_ids):
        d = {}
        for a, key in enumerate(["ux", "uy", "uz", "rx", "ry", "rz"]):
            put(d, key, sol.disp[ni][a], GROUP_OF[key])
        out["nodes"][nid] = d
    supp = {"support:A": sol.R_A}
    if lc.anchor_D:
        supp["support:D"] = sol.R_D
    for sid, R in supp.items():
        d = {}
        for a, key in enumerate(["Fx", "Fy", "Fz", "Mx", "My", "Mz"]):
            put(d, key, R[a], ("support_force" if a < 3 else "support_moment") + "@" + sid)
        out["supports"][sid] = d
    tans = chain.trav_tangents()
    for label, node, transfer, outward in (("A", chain.node_ids[0], lc.transfer_A, scl(-P, tans[0][0])),
                                           ("D", chain.node_ids[-1], lc.transfer_D, scl(P, tans[-1][1]))):
        t = dict(closure_transfer="transfers_to_wall" if transfer else "separately_supported_or_compensated")
        cl, pt, rm = {}, {}, {}
        for a, key in enumerate(["Fx", "Fy", "Fz"]):
            put(cl, key, outward[a], "remote_closure_force")
            put(pt, key, outward[a] if transfer else ZERO, "remote_closure_force")
            if not transfer:
                put(rm, key, -outward[a], "remote_closure_force")
        t["closure_pressure_load_global"] = cl
        t["pipe_cap_transfer_global"] = pt
        t["remote_closure_support_reaction_global"] = rm if not transfer else None
        out["terminals"][node] = t
    for mi, mem in enumerate(chain.members):
        md = dict(kind=mem.kind, authored_from=mem.i_id, authored_to=mem.j_id,
                  traversal_matches_authored=mem.forward, stations={}, end_rows={})
        if mem.kind == "straight":
            md["frame"] = {"x": fmtv(mem.ex), "y": fmtv(mem.ey), "z": fmtv(mem.ez)}
            md["length"] = fmt(mem.length)
        else:
            md["arc"] = dict(R=fmt(mem.R), included_angle=fmt(mem.Phi), arc_length=fmt(mem.length),
                             centre=fmtv(mem.centre),
                             tangent_i=fmtv(mem.authored_tangent("0")),
                             tangent_j=fmtv(mem.authored_tangent("1")),
                             plane_normal_z=fmtv(mem.zhat), k=fmt(mem.k),
                             chord_frame={"x": fmtv(mem.chord_frame[0]),
                                          "y": fmtv(mem.chord_frame[1]),
                                          "z": fmtv(mem.chord_frame[2])})
        actions = {}
        for name, f in STATIONS:
            rec, FM = station_record(sol, chain, mi, f, sec, P)
            actions[name] = FM
            d = {"fraction": f}
            for key, val in rec.items():
                put(d, key, val, GROUP_OF[key])
            md["stations"][name] = d
        for name, f, sign in (("end_i", "0", -ONE), ("end_j", "1", ONE)):
            F, M = actions[name]
            if mem.kind == "straight":
                er = end_rows(F, M, mem.frame_at_authored(f), sign)
                d = {}
                for key, val in er.items():
                    put(d, key, val, end_group(key, "element_local"))
                md["end_rows"][name] = {"element_local": d}
            else:
                dd = {}
                er = end_rows(F, M, mem.frame_at_authored(f), sign)
                d = {}
                for key, val in er.items():
                    put(d, key, val, end_group(key, "tangent_frame"))
                dd["tangent_frame"] = d
                # elastic chord-frame rows (repair 01 S-1): node-on-element wall action minus the bend's own cap
                # pair c_b = [-pAi t_i, +pAi t_j] (H-2), i.e. K d - p of the curved element; moments unchanged
                cb = scl(-P, mem.authored_tangent("0")) if name == "end_i" else scl(P, mem.authored_tangent("1"))
                Fn = scl(sign, F)
                Mn = scl(sign, M)
                Fe = sub(Fn, cb)
                d = {}
                for a, key in enumerate(["F_x", "F_y", "F_z"]):
                    put(d, key, dot(Fe, mem.chord_frame[a]), "elastic_end_force_chord_frame")
                for a, key in enumerate(["M_x", "M_y", "M_z"]):
                    put(d, key, dot(Mn, mem.chord_frame[a]), "section_moment")
                dd["chord_frame_elastic"] = d
                md["end_rows"][name] = dd
        if mem.kind == "straight" and lc.p != 0:
            lame = {}
            put(lame, "inner_radial", -lc.p, "lame_stress")
            put(lame, "outer_radial", ZERO, "lame_stress")
            put(lame, "inner_hoop", 2 * P / sec.As + lc.p, "lame_stress")
            put(lame, "outer_hoop", 2 * P / sec.As, "lame_stress")
            md["lame_surface"] = lame
        elif mem.kind == "arc":
            md["lame_surface"] = "withheld_on_arcs"
        out["members"][mem.pid] = md
    # snap, zero scale, format
    gmax = {}
    for grp, dct, key in raw:
        val = dct[key]
        if abs(val) < SNAP * chars[grp.split("@")[0]]:
            val = ZERO
            dct[key] = ZERO
        gmax[grp] = max(gmax.get(grp, ZERO), abs(val))
    fallback = {"displacement": abs(eps_p + eps_th) * Lc if (eps_p + eps_th) != 0 else chars["displacement"],
                "rotation": abs(eps_p + eps_th) if (eps_p + eps_th) != 0 else chars["rotation"],
                "support_force": F0, "support_moment": F0 * Lc, "remote_closure_force": F0,
                "wall_axial_force": F0, "effective_axial_force": F0, "membrane_stress": F0 / sec.As,
                "shear_force": F0, "section_moment": F0 * Lc, "bending_torsion_stress": F0 / sec.As,
                "elastic_end_force_chord_frame": F0, "lame_stress": abs(lc.p) or ONE}
    zero_scale = {}
    for grp in gmax:
        zs = gmax[grp] if gmax[grp] != 0 else fallback[grp.split("@")[0]]
        zero_scale[grp] = dict(unit=UNIT_OF_GROUP[grp.split("@")[0]], zero_scale=fmt(zs), absolute_floor=fmt(D("1e-9") * zs),
                               basis="max |expected| in this group for this case" if gmax[grp] != 0
                               else "group expected identically zero; characteristic scale (see README rules)")
    for grp, dct, key in raw:
        dct[key] = fmt(dct[key])
    res = sol.equilibrium_residual
    derived = dict(t_effective=fmt(sec.t), r_o=fmt(sec.ro), r_i=fmt(sec.ri), A_s=fmt(sec.As), A_i=fmt(sec.Ai),
                   I=fmt(sec.I), J=fmt(sec.J), Z=fmt(sec.Z), G=fmt(mat.G), P_pAi=fmt(P), eps_poisson=fmt(eps_nu),
                   eps_p=fmt(eps_p), eps_thermal=fmt(eps_th), w_self_weight=fmt(wv),
                   total_centreline_length=fmt(total_len), W_total=fmt(wv * total_len), L_c=fmt(Lc))
    derived["alpha_tan_provisional_rad"] = ALPHA_TAN
    derived["junction_angles_rad"] = junction_angles(chain)
    checks = dict(global_equilibrium_residual_force=fmt(max(abs(c) for c in res[0])),
                  global_equilibrium_residual_moment=fmt(max(abs(c) for c in res[1])))
    if not lc.anchor_D and lc.transfer_A and lc.transfer_D and lc.w == 0:
        e = eps_p + eps_th
        dev = ZERO
        for ni in range(len(chain.node_ids)):
            exp = scl(e, sub(chain.xn[ni], xA))
            for a in range(3):
                dev = max(dev, abs(sol.disp[ni][a] - exp[a]))
            for a in range(3, 6):
                dev = max(dev, abs(sol.disp[ni][a]))
        checks["closed_form_self_similar_growth_max_abs_deviation_m"] = fmt(dev)
    return out, derived, zero_scale, checks, (chain, sec, mat, lc, sol)


def junction_angles(chain):
    """theta = atan2(|t_in x t_out|, t_in . t_out) at every interior node next to a bend (the T4-U2 rule)."""
    tans = chain.trav_tangents()
    out = {}
    for m in range(len(chain.members) - 1):
        if chain.members[m].kind == "arc" or chain.members[m + 1].kind == "arc":
            a, b = tans[m][1], tans[m + 1][0]
            c = norm(cross(a, b))
            th = ZERO if c < SNAP else E.atan2(c, dot(a, b))
            out[chain.node_ids[m + 1]] = fmt(th)
    return out


# ------------------------------------------------------------------ negative controls
def mutant_summary(case, mutation, ref_out, ref_zero_scale, label, desc):
    chain, sec, mat, lc, sol = evaluate(case, mutation)
    last = chain.node_ids[-1]
    items = []
    R = {"support:A": sol.R_A}
    if lc.anchor_D:
        R["support:D"] = sol.R_D
    for sid, vals in R.items():
        for a, key in enumerate(["Fx", "Fy", "Fz", "Mx", "My", "Mz"]):
            grp = ("support_force" if a < 3 else "support_moment") + "@" + sid
            items.append(("/expected/supports/%s/%s" % (sid, key), vals[a], grp))
    tipnode = last if not lc.anchor_D else chain.node_ids[len(chain.node_ids) // 2]
    for a, key in enumerate(["ux", "uy", "uz", "rx", "ry", "rz"]):
        items.append(("/expected/nodes/%s/%s" % (tipnode, key), sol.disp[chain.node_ids.index(tipnode)][a],
                      GROUP_OF[key]))
    return discriminators(items, ref_out, ref_zero_scale, label, desc)


def get_ptr(out, ptr):
    parts = ptr.strip("/").split("/")
    cur = out
    for p_ in parts:
        cur = cur[p_]
    return D(cur)


MIN_LISTED_DISTANCE = D("1e3")  # repair 01 B-1: a listed row sits at least 1e3 tolerances from the reference
LISTED_ROWS = 8


def fmt_dist(x):
    return "0" if x == 0 else format(x, ".3e")


def discriminators(items, ref_out, zs, label, desc):
    rows = []
    best = None
    for ptr, wrong, grp in items:
        ref = get_ptr(ref_out, ptr)
        zscale = D(zs[grp]["zero_scale"]) if grp in zs else ONE
        raw = D(wrong)
        w = ZERO if abs(raw) < SNAP * zscale else raw  # computation noise is exact zero
        tol = D("1e-9") * max(abs(ref), zscale)
        dist = abs(w - ref) / tol
        rank = D(format(abs(raw - ref) / tol, ".3e"))  # round 00's ranking key (unsnapped, 4 digits)
        if dist == 0:
            reason = ("wrong value equals the reference" if raw == ref else
                      "wrong value differed from the reference only by computation noise below 1e-40 of the group "
                      "scale; it is exact zero, equal to the reference")
        elif dist < MIN_LISTED_DISTANCE:
            reason = "distance below 1e3 tolerances"
        else:
            reason = None
        rows.append((rank, dict(pointer=ptr, wrong_value=fmt(w), reference_value=fmt(ref),
                                distance_in_tolerances=fmt_dist(dist)), reason))
        if best is None or dist > best:
            best = dist
    # rank as round 00 did, so that the listing keeps round 00's order and rows_dropped_repair_01 names exactly
    # the round-00 rows that fail the >= 1e3 rule
    order = sorted(range(len(rows)), key=lambda i: -rows[i][0])  # stable: equal keys keep item order
    ranked = [rows[i] for i in order]
    listed = [r for d_, r, why in ranked if why is None][:LISTED_ROWS]
    dropped = []
    for d_, r, why in ranked[:LISTED_ROWS]:
        if why is not None:
            x = dict(r)
            x["reason"] = why
            dropped.append(x)
    return dict(id=label, description=desc, max_distance_in_tolerances=format(best, ".3e"),
                listing_rule="rows at >= 1e3 tolerances from the reference, at most 8, by distance",
                discriminating_values=listed, rows_dropped_repair_01=dropped)


def recovery_mutants(case, rec, ref_out, zs):
    chain, sec, mat, lc, sol = rec
    P = lc.p * sec.Ai
    res = []
    items3 = []
    for mem in chain.members:
        for name, f in STATIONS:
            wrong = D(ref_out["members"][mem.pid]["stations"][name]["S"])  # N_w - pAi
            items3.append(("/expected/members/%s/stations/%s/N_w" % (mem.pid, name), wrong, "wall_axial_force"))
    res.append(discriminators(items3, {"expected": ref_out}, zs, "caps_subtracted_in_recovery",
                              "wall recovery subtracts the transferred cap: published N_w equals S = N_w - pAi "
                              "(and sigma_m = S/As); QUAL section 1"))
    items5a, items5b = [], []
    for mem in chain.members:
        if mem.kind != "arc":
            continue
        for name, f in STATIONS:
            th = D(f) * mem.Phi
            Dl = mem.Phi - th
            Fw = scl(P, add(scl(E.sin(mem.Phi) - E.sin(th), mem.xhat), scl(E.cos(th) - E.cos(mem.Phi), mem.yhat)))
            Mw = scl(-mem.R * P * (1 - E.cos(Dl)), mem.zhat)
            fr = mem.frame_at_authored(f)
            st = ref_out["members"][mem.pid]["stations"][name]
            for key, val in (("N_w", dot(Fw, fr[0])), ("V_y", dot(Fw, fr[1])), ("M_z", dot(Mw, fr[2]))):
                grp = GROUP_OF[key]
                items5a.append(("/expected/members/%s/stations/%s/%s" % (mem.pid, name, key), D(st[key]) + val, grp))
            items5b.append(("/expected/members/%s/stations/%s/N_w" % (mem.pid, name), D(st["N_w"]) + P,
                            "wall_axial_force"))
    if items5a:
        res.append(discriminators(items5a, {"expected": ref_out}, zs, "wall_load_double_count_elastic_end_force",
                                  "station statics evaluated from the elastic end force with the arc wall load also "
                                  "included as a distributed intensity, plus the +pAi membrane (the wall load counted "
                                  "twice): station actions = reference + j-side action of the wall load on "
                                  "[theta, Phi]"))
        res.append(discriminators(items5b, {"expected": ref_out}, zs, "wall_load_double_count_wall_end_force",
                                  "station statics evaluated from the wall end force (cap included) with the wall load "
                                  "in the statics, plus +pAi again: N_w = reference + pAi"))
    return res


def negative_controls(case, ref_out, zs, rec):
    chain, sec, mat, lc, sol = rec
    P = lc.p * sec.Ai
    ctrls = []
    wrap = {"expected": ref_out}
    arcs = [(mi, m) for mi, m in enumerate(chain.members) if m.kind == "arc"]
    ti, tj = arcs[0][1].authored_tangent("0"), arcs[0][1].authored_tangent("1")
    resid = scl(-P, sub(ti, tj))
    c = mutant_summary(case, dict(arc_wall=False, arc_poisson=False), wrap, zs, "bend_term_omitted",
                       "the bend term K_b u_free(eps_p) - c_b omitted (no wall load, no Poisson eigenstrain on the "
                       "arc); the applied set then has the residual -pAi(t_i - t_j) of the first bend's own tangents")
    c["residual_of_applied_set_first_bend_global"] = [fmt(x) for x in resid]
    ctrls.append(c)
    extra = {}
    for mi, mem in arcs:
        Pm = P
        i_node = chain.node_ids.index(mem.i_id)
        j_node = chain.node_ids.index(mem.j_id)
        extra.setdefault(i_node, []).append((scl(-2 * Pm, mem.authored_tangent("0")), E.VZERO))
        extra.setdefault(j_node, []).append((scl(2 * Pm, mem.authored_tangent("1")), E.VZERO))
    ctrls.append(mutant_summary(case, dict(extra_point_loads=extra), wrap, zs, "c_b_added_not_subtracted",
                                "bend term formed as K_b u_free(eps_p) + c_b: equivalent to adding 2c_b = "
                                "[-2pAi t_i, +2pAi t_j] at each bend's end nodes"))
    ctrls.append(mutant_summary(case, dict(arc_poisson=False), wrap, zs, "poisson_term_missing_on_arc",
                                "the arc's axial eigenstrain lacks -2 nu pAi/(E As) (wall load kept), i.e. the bend "
                                "uses u_free(pAi/(E As)) in load and recovery"))
    ctrls.extend(recovery_mutants(case, rec, ref_out, zs))
    return ctrls


def kink_control(case, ref_out, zs):
    # remainder omitted at node C (index 2)
    c = mutant_summary(case, dict(remove_kink_at=[2]), {"expected": ref_out}, zs, "interior_remainder_omitted",
                       "the interior remainder pAi(t_in - t_out) at the kinked node C omitted; the applied set is then "
                       "unbalanced by -pAi(t_in - t_out)")
    chain, sec, mat, lc, sol = evaluate(case)
    P = lc.p * sec.Ai
    tans = chain.trav_tangents()
    kf = scl(P, sub(tans[1][1], tans[2][0]))
    c["remainder_at_C_global"] = [fmt(x) for x in kf]
    c["kink_angle_rad"] = fmt(E.atan2(norm(cross(tans[1][1], tans[2][0])), dot(tans[1][1], tans[2][0])))
    return c


# ------------------------------------------------------------------ polygon limit control
def polygon_control(case, ns=(8, 16, 32, 64, 128)):
    chain0, sec, mat, lc0, sol0 = evaluate(case)
    ref = list(sol0.R_A) + (list(sol0.R_D) if lc0.anchor_D else []) + list(sol0.disp[-1])
    groups = (["F"] * 3 + ["M"] * 3) * (2 if lc0.anchor_D else 1) + ["u"] * 3 + ["r"] * 3
    P = lc0.p * sec.Ai
    Lc = max(norm(sub(x, chain0.xn[0])) for x in chain0.xn)
    eps = abs((1 - 2 * mat.nu) * P / (mat.E * sec.As) + mat.alpha * lc0.dT)
    char = {"F": P, "M": P * Lc, "u": eps * Lc, "r": eps}
    scale = {}
    for g_, x in zip(groups, ref):
        scale[g_] = max(scale.get(g_, ZERO), abs(x))
    for g_ in scale:
        if scale[g_] < D("1e-30") * char[g_]:
            scale[g_] = char[g_]
    rows = []
    vals = {}
    for n in ns:
        chain, _, _ = build_chain(case, polygon_n=n)
        lc = load_case(case, sec)
        # displacement only needed at the last node
        sol = analyse_lite(chain, lc)
        q = list(sol.R_A) + (list(sol.R_D) if lc.anchor_D else []) + list(sol.disp_last)
        vals[n] = q
        err = max(abs(a - b) / scale[g_] for a, b, g_ in zip(q, ref, groups))
        rows.append(dict(n=n, max_normwise_difference=format(err, ".3e")))
    for i in range(1, len(rows)):
        e0 = D(rows[i - 1]["max_normwise_difference"])
        e1 = D(rows[i]["max_normwise_difference"])
        rows[i]["ratio_to_previous"] = format(e0 / e1, ".4f") if e1 != 0 else "inf"
    n1, n2 = ns[-2], ns[-1]
    rich = [(4 * b - a) / 3 for a, b in zip(vals[n1], vals[n2])]
    rerr = max(abs(a - b) / scale[g_] for a, b, g_ in zip(rich, ref, groups))
    return dict(case_id=case["id"], k=case["k"],
                segments_bending_factor="k = 1: plain Euler-Bernoulli segments (the brief's control)" if case["k"] == "1"
                else "segments carry the arc's k on their bending energy only (an additional control for k = 2)",
                compared="support reactions" + (" at A and D" if lc0.anchor_D else " at A")
                + " and the six displacements of node D, normwise per family (force, moment, translation, rotation)",
                convergence=rows, richardson_n=[n1, n2], richardson_max_normwise_difference=format(rerr, ".3e"))


def analyse_lite(chain, lc):
    """analyse() restricted to the last node's displacement (polygon control)."""
    full_nodes = chain.node_ids
    sol = E.analyse(chain, lc, disp_nodes=[len(full_nodes) - 1])
    sol.disp_last = sol.disp[len(full_nodes) - 1]
    return sol


# ------------------------------------------------------------------ document sketches
def units_of(case):
    if case["units"] == "mm":
        return dict(length="mm", force="N", angle="rad", pressure="MPa", stress="MPa", temperature="degC")
    return dict(length="m", force="N", angle="rad", pressure="Pa", stress="Pa", temperature="degC")


def qty(val_si, dim, case):
    mm = case["units"] == "mm"
    x = D(val_si)
    if dim == "length":
        return {"value": f64(x * 1000) if mm else f64(x), "unit": "mm" if mm else "m"}
    if dim == "pressure":
        return {"value": f64(x / 10 ** 6) if mm else f64(x), "unit": "MPa" if mm else "Pa"}
    if dim == "force_per_length":
        return {"value": f64(x / 1000) if mm else f64(x), "unit": "N/mm" if mm else "N/m"}
    raise ValueError(dim)


def doc_common(case, chain_info, schema):
    g = case["geom"]
    prov = "invented VP-STATIC exact_pressure_1 reference input (T4-I7); not library, component or code-rule data"
    mm = case["units"] == "mm"
    nodes = []
    for nid, p in g["nodes"]:
        nodes.append({"id": nid, "position": {"x": p[0] * (1000 if mm else 1), "y": p[1] * (1000 if mm else 1),
                                               "z": p[2] * (1000 if mm else 1)}, "provenance": prov})
    sec = g["sec"]
    section = {"outside_diameter": qty(sec["od"], "length", case), "wall_thickness": qty(sec["wall"], "length", case)}
    if D(sec["mill"]) != 0:
        section["mill_tolerance"] = qty(sec["mill"], "length", case)
    pipes = []
    comps = []
    for m in g["members"]:
        pipes.append({"id": m["pid"], "from": m["i"], "to": m["j"], "section": section, "material": "material:steel",
                      "y_reference": {"x": m["yref"][0], "y": m["yref"][1], "z": m["yref"][2]}, "provenance": prov})
        if m["kind"] == "arc":
            comps.append({"id": m["comp"], "kind": "bend", "node": m["i"],
                          "geometry": {"bend_pipe_ref": m["pid"], "bend_radius": qty(m["R"], "length", case),
                                       "bend_angle": {"value": chain_info[m["pid"]], "unit": "rad"},
                                       "bend_plane_orientation": "plane of the span chord and the pipe y_reference; "
                                                                 "the arc bows toward +y_reference"},
                          "modifiers": {"flexibility_factor_user_value": {"value": float(case["k"]), "unit": "none"}},
                          "mechanics_interface": {"solver_consumption": "curved_bend_macro_element"},
                          "provenance": prov + "; user flexibility factor k is an invented test value"})
    supports = [{"id": "support:A", "node": g["nodes"][0][0], "family": "anchor",
                 "restraints": ["UX", "UY", "UZ", "RX", "RY", "RZ"], "provenance": prov}]
    if case["anchorD"]:
        supports.append({"id": "support:D", "node": g["nodes"][-1][0], "family": "anchor",
                         "restraints": ["UX", "UY", "UZ", "RX", "RY", "RZ"], "provenance": prov})
    mat = g["mat"]
    material = {"id": "material:steel", "constitutive_basis": "homogeneous_isotropic_E_nu_v1",
                "elastic_modulus": {"value": f64(D(mat["E"]) / 10 ** 6) if mm else f64(mat["E"]),
                                    "unit": "MPa" if mm else "Pa"},
                "poisson_ratio": {"value": f64(mat["nu"]), "unit": "1"}, "provenance": prov}
    region = {"id": "region:u2", "member_pipe_ids": [m["pid"] for m in g["members"]],
              "pressure_basis": "internal_differential_zero_external_v1", "pressure": qty(case["p"], "pressure", case),
              "terminals": [{"node_ref": g["nodes"][0][0],
                             "closure_transfer": "transfers_to_wall" if case["tA"] else
                             "separately_supported_or_compensated", "provenance": prov},
                            {"node_ref": g["nodes"][-1][0],
                             "closure_transfer": "transfers_to_wall" if case["tD"] else
                             "separately_supported_or_compensated", "provenance": prov}],
              "provenance": prov}
    weights = []
    if case["weight"]:
        for m in g["members"]:
            weights.append({"id": "load:weight:%s" % m["pid"], "category": "distributed_force",
                            "target": {"type": "element", "pipe": m["pid"]}, "direction": "global_z",
                            "magnitude": qty("-" + W_L, "force_per_length", case), "dimension": "force_per_length",
                            "provenance": prov + "; self-weight intensity w = (As*rho_s + Ai*rho_c)*g authored "
                                                 "directly (rho_s 7850, rho_c 1000 kg/m3, g 9.80665 m/s2 along -Z)"})
    model = {"schema_version": schema, "document_kind": "openpipestress.product_preview.model",
             "pressure_contract": {"version": "3.0.0", "mode": "exact_pressure_v3"},
             "project": {"id": "project:%s" % case["id"].lower(), "units": units_of(case)},
             "analysis_status": {"mechanics": "ready_for_preview_diagnostics",
                                 "rule_check": "not_performed_user_rule_inputs_missing",
                                 "professional_acceptance": "not_provided"},
             "nodes": nodes, "pipe_segments": pipes, "supports": supports, "components": comps,
             "materials": [material]}
    return model, region, weights, prov


def doc_030(case, chain_info):
    model, region, weights, prov = doc_common(case, chain_info, "0.3.0")
    g = case["geom"]
    model["materials"][0]["thermal_expansion_coefficient"] = {"value": f64(g["mat"]["alpha"]), "unit": "1/degC"}
    model["materials"][0]["temperature_points"] = []
    loads = []
    if D(case["dT"]) != 0:
        for m in g["members"]:
            loads.append({"id": "load:thermal:%s" % m["pid"], "category": "thermal",
                          "target": {"type": "element", "pipe": m["pid"]}, "direction": "global_x",
                          "magnitude": {"value": f64(case["dT"]), "unit": "degC"}, "dimension": "temperature_interval",
                          "provenance": prov})
    loads.extend(weights)
    model["load_cases"] = [{"id": "case:u2", "primitive_loads": loads, "pressure_regions": [region],
                            "provenance": prov}]
    model["combinations"] = []
    prov_list = [
        {"pointer": "/model/pressure_contract", "reason": "the v3 identity 3.0.0/exact_pressure_v3 (H-1) is defined by "
                                                          "T4-U2a; its exact field spelling is provisional"},
        {"pointer": "/model/load_cases/0/pressure_regions/0/member_pipe_ids",
         "reason": "a realized bend pipe inside a pressure region is admitted only by T4-U2 (today refused)"},
        {"pointer": "/model/components", "reason": "a curved_bend_macro_element component on the exact route is "
                                                   "admitted only by T4-U2a/T4-U2 (today EXACT_PRESSURE_COMPOSITION_"
                                                   "UNSUPPORTED)"},
    ]
    return {"provisional": prov_list, "document": {"model": model, "materials": []}}


def doc_040(case, chain_info):
    model, region, weights, prov = doc_common(case, chain_info, "0.4.0")
    g = case["geom"]
    model["reference_configurations"] = [{
        "id": "reference:installed", "label": "Installed reference", "geometry_ref": {"kind": "authored_model_geometry"},
        "member_references": [{"pipe_ref": m["pid"], "basis": {"kind": "direct_strain_reference"},
                               "fit": {"kind": "none"}, "provenance": prov} for m in g["members"]],
        "provenance": prov}]
    states = []
    for m in g["members"]:
        st = {"pipe_ref": m["pid"],
              "material_selection": {"kind": "explicit_base_properties", "material_ref": "material:steel",
                                     "applicability_reference": "invented analytical basis declared applicable"}}
        if D(case["dT"]) != 0:
            st["thermal_state"] = {"kind": "constant_alpha_interval",
                                   "coefficient": {"value": f64(g["mat"]["alpha"]), "unit": "1/degC"},
                                   "temperature_change": {"value": f64(case["dT"]), "unit": "degC"},
                                   "coefficient_meaning": "engineering_interval", "provenance": prov}
        else:
            st["thermal_state"] = {"kind": "unchanged_reference", "provenance": prov}
        states.append(st)
    supp_states = [{"support_ref": s["id"], "participation": {"kind": "active_model_device"}} for s in model["supports"]]
    model["load_cases"] = [{"id": "case:u2", "pressure_regions": [region], "primitive_loads": weights,
                            "analysis_state": {"contract": "openpipestress.load_reference_state/1.0.0",
                                               "reference_configuration_ref": "reference:installed",
                                               "element_states": states, "support_states": supp_states,
                                               "load_sources": [{"source_ref": w["id"], "factor": 1.0}
                                                                for w in weights],
                                               "history": {"kind": "independent_equilibrium"}, "provenance": prov},
                            "provenance": prov}]
    model["combinations"] = []
    prov_list = [
        {"pointer": "/model/pressure_contract", "reason": "v3 identity on load-reference-1 documents (T4-U2a; RV1 N-6)"},
        {"pointer": "/model/load_cases/0/pressure_regions/0/member_pipe_ids", "reason": "bend admitted in a region "
                                                                                        "(T4-U2)"},
        {"pointer": "/model/components", "reason": "curved bend admitted on the exact route (T4-U2a/T4-U2)"},
        {"pointer": "/model/load_cases/0/analysis_state/element_states",
         "reason": "the bend pipe's per-member E/nu selection is consumed by the curved element only after T4-U1 "
                   "(I1 section 5.3 #12); today the bend reads the base material"},
    ]
    return {"provisional": prov_list, "document": {"model": model, "materials": []}}


# ------------------------------------------------------------------ main
def case_inputs(case):
    g = case["geom"]
    return dict(
        geometry=dict(name=g["name"], node_coordinates_binary64_exact={nid: [repr(c) for c in p] for nid, p in g["nodes"]},
                      traversal_order=[nid for nid, _ in g["nodes"]],
                      members=[dict(pipe=m["pid"], kind=m["kind"], authored_from=m["i"], authored_to=m["j"],
                                    y_reference_binary64_exact=[repr(c) for c in m["yref"]],
                                    **({"bend_radius": m["R"], "flexibility_factor_k": case["k"],
                                        "component": m["comp"]} if m["kind"] == "arc" else {}))
                               for m in g["members"]]),
        section=dict(outside_diameter=g["sec"]["od"], wall_thickness=g["sec"]["wall"],
                     mill_tolerance=g["sec"]["mill"], unit="m",
                     rule="t_eff = wall - mill; ID = OD - 2 t_eff; As = pi(OD^2-ID^2)/4; Ai = pi ID^2/4; "
                          "I = pi(OD^4-ID^4)/64; J = 2I; Z = I/(OD/2); torsion radius OD/2"),
        material=dict(E=g["mat"]["E"], nu=g["mat"]["nu"], alpha_per_degC=g["mat"]["alpha"],
                      G="E/(2(1+nu)) (derived; no authored G)"),
        pressure=dict(p_Pa=case["p"], region="one region over every member in traversal order",
                      terminal_A="transfers_to_wall" if case["tA"] else "separately_supported_or_compensated",
                      terminal_D="transfers_to_wall" if case["tD"] else "separately_supported_or_compensated"),
        thermal=dict(delta_T_degC=case["dT"], applies_to="every member (straights and arcs), uniform"),
        self_weight=dict(w_N_per_m=W_L if case["weight"] else "0",
                         direction="global -Z (gravity), per unit centreline length (arc length on bends)",
                         informative_derivation="w = (As*7850 + Ai*1000)*9.80665 rounded to 17 significant digits"
                         if case["weight"] else None),
        supports=["support:A anchor at %s" % g["nodes"][0][0]] + (
            ["support:D anchor at %s" % g["nodes"][-1][0]] if case["anchorD"] else []),
        units_of_document=case["units"])


def main(outdir):
    cases = case_list()
    records = {}
    sketches = {}
    summary = []
    rec_cache = {}
    for case in cases:
        if case["family"] == "MITRE-REFUSAL":
            chain, sec, mat = build_chain(case)
            ja = junction_angles(chain)
            chain_info = {m.pid: f64(m.Phi) for m in chain.members if m.kind == "arc"}
            records[case["id"]] = dict(
                description=case["desc"], family=case["family"], k=case["k"], core_case=False, transform_of=None,
                transform=None, inputs=case_inputs(case),
                derived=dict(alpha_tan_provisional_rad=ALPHA_TAN, junction_angles_rad=ja),
                expected=None,
                expected_refusal=dict(
                    refused=True, severity="blocking", code=MITRE_CODE, code_status="provisional (T4-I11 D-D)",
                    refs=["region:u2", "node:C", "pipe:BEND", "pipe:S2"],
                    rule="theta = atan2(|t_in x t_out|, t_in . t_out) > alpha_tan at a bend-adjacent junction",
                    node="node:C", theta_rad=ja["node:C"], alpha_tan_rad=ALPHA_TAN,
                    no_values="the case must not publish displacements, reactions or member rows"))
            sketches[case["id"]] = {"0.3.0": doc_030(case, chain_info), "0.4.0": doc_040(case, chain_info)}
            continue
        out, derived, zs, checks, rec = record_case(case)
        rec_cache[case["id"]] = (out, zs, rec)
        chain = rec[0]
        chain_info = {m.pid: f64(m.Phi) for m in chain.members if m.kind == "arc"}
        entry = dict(description=case["desc"], family=case["family"], k=case["k"],
                     core_case=bool(case.get("core")), transform_of=case.get("transform_of"),
                     transform=case.get("transform"), inputs=case_inputs(case), derived=derived,
                     expected=out, zero_scale=zs, checks=checks)
        if case.get("rebuilds"):
            entry["rebuilds_retired_case_id"] = case["rebuilds"]
        records[case["id"]] = entry
        sketches[case["id"]] = {"0.3.0": doc_030(case, chain_info), "0.4.0": doc_040(case, chain_info)}
        summary.append((case["id"], checks))
    # negative controls
    ctrl_cases = ["U2-L-FREE-P-K2", "U2-L-ANCH-P-K2", "U2-L-ANCH-ALL-K2", "U2-U-ANCH-P-K2",
                  "MECH-CURVED-BEND-EXACT-PRESSURE-ARC-K2", "U2-L-ANCH-PTW-K2"]
    by_id = {c["id"]: c for c in cases}
    for cid in ctrl_cases:
        out, zs, rec = rec_cache[cid]
        records[cid]["wrong_result_discriminators"] = negative_controls(by_id[cid], out, zs, rec)
    for cid in ("U2-L-KINK-FREE-P-K2", "U2-L-KINK-ANCH-P-K2"):
        out, zs, rec = rec_cache[cid]
        records[cid]["wrong_result_discriminators"] = [kink_control(by_id[cid], out, zs)]
    # polygon control at k = 1
    poly = []
    for cid in ("U2-L-ANCH-P-K1", "U2-L-ANCH-ALL-K1", "U2-L-FREE-ALL-K1", "U2-U-ANCH-ALL-K1",
                "MECH-CURVED-BEND-EXACT-PRESSURE-ARC-K1", "U2-L-ANCH-P-K2", "U2-U-ANCH-ALL-K2",
                "U2-L-ANCH-ALL-K2-SKEW-X7P3E6", "U2-L-ANCH-PTW-K1", "U2-L-FREE-PTW-K1"):
        poly.append(polygon_control(by_id[cid]))
    doc = header()
    doc["polygon_limit_control"] = poly
    doc["cases"] = records
    with open(os.path.join(outdir, "u2_reference_cases.json"), "w") as fh:
        json.dump(doc, fh, indent=1, sort_keys=False)
        fh.write("\n")
    with open(os.path.join(outdir, "u2_document_sketches.json"), "w") as fh:
        fh.write('{"schema_version":"independent.exact_pressure_bend_document_sketches/1.0.0",\n')
        fh.write('"status":"provisional sketches; each lists the JSON pointers that depend on v3 fields T4-U2a '
                 'defines; one case per line",\n"reference_file":"u2_reference_cases.json",\n"sketches":{\n')
        ids = list(sketches)
        for i, cid in enumerate(ids):
            fh.write(json.dumps(cid) + ":" + json.dumps(sketches[cid], separators=(",", ":"))
                     + ("," if i < len(ids) - 1 else "") + "\n")
        fh.write("}}\n")
    # summary
    print("cases:", len(records))
    worst_f = max(D(c["global_equilibrium_residual_force"]) for _, c in summary)
    worst_m = max(D(c["global_equilibrium_residual_moment"]) for _, c in summary)
    print("max global equilibrium residual: force %s N, moment %s N*m" % (fmt(worst_f), fmt(worst_m)))
    for cid, r in records.items():
        if r["family"] in ("KINK", "MITRE-REFUSAL"):
            print("junction angle", cid, r["derived"]["junction_angles_rad"]["node:C"], "alpha_tan", ALPHA_TAN)
    for cid, c in summary:
        if "closed_form_self_similar_growth_max_abs_deviation_m" in c:
            print("closed-form growth deviation", cid, c["closed_form_self_similar_growth_max_abs_deviation_m"])
    for p_ in poly:
        print("polygon", p_["case_id"], [(r["n"], r["max_normwise_difference"], r.get("ratio_to_previous"))
                                         for r in p_["convergence"]], "richardson", p_["richardson_max_normwise_difference"])
    for cid in ctrl_cases + ["U2-L-KINK-FREE-P-K2", "U2-L-KINK-ANCH-P-K2"]:
        for c in records[cid]["wrong_result_discriminators"]:
            print("control", cid, c["id"], "max distance (tolerances)", c["max_distance_in_tolerances"])
    # selected headline values
    for cid in ("MECH-CURVED-BEND-EXACT-PRESSURE-ARC-K1", "U2-L-ANCH-P-K2", "U2-L-ANCH-ALL-K2", "U2-U-ANCH-ALL-K2"):
        e = records[cid]["expected"]
        print(cid, "support:A", e["supports"]["support:A"])
        if cid.startswith("MECH"):
            print(cid, "node:B", e["nodes"]["node:B"])


def header():
    return {
        "schema_version": "independent.exact_pressure_bend_examples/1.0.0",
        "purpose": "Frozen independent VP-STATIC references for T4-U2 (pressure through realized bends), derived "
                   "before any implementation by the direct unit-load (flexibility) method; not product request DTOs "
                   "and not observed solver outputs.",
        "author": "T4-I7 (TASK, Type 2) for T4's WORKING_ITEMS; refuted by T4-RV3 (values pass); repair round 01 "
                  "applies RV3's B-1 and S-1 to S-3 (REPAIR_01.md)",
        "revision": "repair 01",
        "code_basis_for_conventions": "ed012c7ccf (read for conventions only, never for values)",
        "target_contract": {"model": "3.0.0/exact_pressure_v3 (H-1; provisional spelling, T4-U2a)",
                            "result_semantics": "pressure-1 (reserved; provisional)",
                            "package": "a later VP-STATIC exact_pressure_1 package on T1's load_reference_1 pattern"},
        "numbers": "Invented test quantities. Not material-library, component-library or code-rule data.",
        "numeric_representation": {
            "values": "decimal strings with %d significant digits; exact zero is \"0\"" % SIG,
            "evaluation": "Python decimal at 60 significant digits; arc integrals by 30-point Gauss-Legendre "
                          "(recomputed at 90 digits with 50 points, no written value changes: u2_precision_check); straight integrals exact "
                          "(4-point Gauss-Legendre on cubic integrands); pi by Machin; sin/cos/atan by series",
            "snapping": "a computed value below 1e-40 of its group's characteristic scale is written as exact 0",
            "inputs": "node coordinates and y_reference components are binary64 values written as round-trip "
                      "decimal strings and are exact inputs (this matters at X = 5e6 and 7.3e6 m); every other "
                      "authored decimal is an exact rational target whose binary64 operand is generally not exact "
                      "(effect below 1e-15 relative)",
        },
        "criteria": {
            "rule": "|observed - expected| <= 1e-9 * max(|expected|, zero_scale(group)) in both solver modes",
            "zero_scale": "per case and per quantity group (support groups per support, e.g. support_force@support:A): "
                          "max |expected| over the group in that case; if the group "
                          "is identically zero, a characteristic scale: force pAi (or W_total if larger), moment "
                          "that force times L_c, translation |eps_p + eps_th|*L_c, rotation |eps_p + eps_th|, stress "
                          "force/As; given as absolute_floor = 1e-9 * zero_scale in each case's zero_scale block",
            "unit_transforms": "references are SI (m, rad, N, N*m, Pa); the product publishes nodal translations in "
                               "mm (x1000); compare after exact unit conversion",
            "no_relaxation": "this file allocates no new or relaxed threshold beyond the per-group floor the brief "
                             "requires (I4 section 5)",
            "negative_controls": "each control lists only rows at >= 1e3 tolerances from the reference (at most 8, by "
                                 "distance); max_distance_in_tolerances is over every row the control evaluated; rows "
                                 "removed from round 00's listing are kept, with their reason, in "
                                 "rows_dropped_repair_01 (not assertions)",
        },
        "tangency_rule": {"rule": "theta = atan2(|t_in x t_out|, t_in . t_out) <= alpha_tan at every bend-adjacent "
                                  "junction; admitted kinks carry the remainder pAi(t_in - t_out) exactly (H-2); "
                                  "larger kinks are refused as mitres",
                          "alpha_tan_rad": ALPHA_TAN, "status": "provisional (T4-I11 D-D; WI repair 01 S-2)",
                          "refusal_code": MITRE_CODE + " (provisional)",
                          "junction_angles": "each case's derived.junction_angles_rad gives theta per node"},
        "formulation_matched": [
            "Euler-Bernoulli straights: axial, torsion and bending energy; no shear",
            "arcs: axial and torsion energy plus in-plane and out-of-plane bending energy scaled by the user's k; "
            "no shear; one k for both planes (as the product passes it)",
            "small strain, small displacement, linear elastic, homogeneous isotropic E/nu with G = E/(2(1+nu))",
            "static internal pressure only; excluded (D-3): flow momentum and transient loads, bend opening "
            "(Bourdon), pressure stiffening of k and SIF, ovalization",
            "Poisson eigenstrain -2 nu pAi/(E As) on every member including arcs (straight Lame mean; RV1 S-5)",
        ],
        "loads_applied_in_the_direct_method": [
            "wetted-wall load (pAi/R) along the outward normal per unit arc length on every arc, at the centreline, "
            "no distributed moment",
            "terminal caps along the end tangents: -pAi t_A at A and +pAi t_D at D, applied to the pipe only where "
            "the terminal transfers_to_wall",
            "at an interior node where tangents differ, the physical kink force pAi(t_in - t_out) (zero for exact "
            "tangency)",
            "uniform axial eigenstrain alpha*dT - 2 nu pAi/(E As) on every member",
            "self-weight w along global -Z per unit centreline length",
        ],
        "method": "Flexibility (unit-load) method over the whole line as a cantilever from A: section actions are "
                  "the resultants of all loads beyond the cut (closed forms on straights and arcs); displacements "
                  "are integral(N n/(E As) + T t/(G J) + k (M.m - T t)/(E I) + n eps0) ds; for an anchored D the "
                  "six redundants solve F_DD r = -delta0; reactions at A by equilibrium of node A. No stiffness "
                  "matrix of a curved element and no K*u_free is used for any value in this file.",
        "sign_conventions": {
            "global": "right-handed X, Y, Z; gravity along -Z; displacements and right-hand rotations about global "
                      "axes",
            "supports": "support-on-pipe, global, moment about the attached node (product support_reaction_"
                        "component_v2)",
            "stations": "j-side section action at the authored fraction (product STRAIGHT_ENDPOINT_SECTION_SIGN_"
                        "CONVENTION and CURVED_BEND_SECTION_SIGN_CONVENTION): positive N is tension",
            "straight_frame": "element-local: x = (x_j - x_i)/L, y = y_reference projected normal to x and "
                              "normalized, z = x cross y",
            "arc_tangent_frame": "x = arc tangent toward authored j at the station, y = toward the arc centre, "
                                 "z = bend-plane normal (x_i - c) cross (x_j - c) normalized",
            "arc_geometry": "centre c = (x_i + x_j)/2 - sqrt(R^2 - (L/2)^2) * yhat, with yhat the pipe y_reference "
                            "projected normal to the chord; included angle 2 asin(L/(2R)); the arc bows toward "
                            "+y_reference (product rule)",
            "end_rows": "node-on-element action: end_i = -(j-side action at fraction 0), end_j = +(j-side action at "
                        "fraction 1); straights in the element-local frame (wall_axial_end_action is today's "
                        "pipe_wall_endpoint_action_v2); arcs in the tangent frame at that end (RV1 S-7): wall "
                        "(physical) node-on-element actions",
            "chord_frame_elastic": "arcs only: the elastic node-on-element end action K d - p of the curved element in "
                                   "the chord frame (x chord, y y_reference projected, z = x cross y), i.e. the wall "
                                   "action minus the bend's own cap pair c_b = [-pAi t_i, +pAi t_j] (H-2); moments "
                                   "equal the wall moments. These are today's chord-frame component end rows on arcs "
                                   "(I1 section 5.3 #8; I2 section 3.4); they are not wall actions (repair 01 S-1)",
            "derived_rows": "S = N_w - pAi; sigma_m = N_w/As; sigma_b_y = M_y/Z; sigma_b_z = M_z/Z; tau_t = T*r_o/J",
            "lame_surface": "straights only: inner radial -p, outer radial 0, inner hoop 2pAi/As + p, outer hoop "
                            "2pAi/As; withheld on arcs (plan section 2 and 4.3 item 1)",
            "terminals": "closure_pressure_load_global is the outward cap on the closure; pipe_cap_transfer_global "
                         "is the part applied to the pipe; remote_closure_support_reaction_global = -cap for a "
                         "separately supported closure (product evidence fields)",
        },
        "units_by_key": {
            "ux, uy, uz": "m", "rx, ry, rz": "rad", "Fx, Fy, Fz (supports, terminals)": "N",
            "Mx, My, Mz (supports)": "N*m", "N_w, S, V_y, V_z, F_x, F_y, F_z, wall_axial_end_action": "N",
            "T, M_y, M_z, M_x": "N*m", "sigma_m, sigma_b_y, sigma_b_z, tau_t, lame_surface/*": "Pa",
            "arc/R, arc/arc_length, arc/centre, frame/length": "m", "arc/included_angle": "rad",
        },
        "station_names": {"end_i": "0", "quarter_1": "0.25", "midspan": "0.5", "quarter_3": "0.75", "end_j": "1",
                          "note": "fractions of the authored length (straights) or of the included angle (arcs); "
                                  "the product uses the same names on both"},
        "row_kind_map_today": {
            "N_w": "pipe_wall_axial_force_v2", "S": "pipe_effective_axial_force_v2",
            "sigma_m": "pipe_axial_membrane_stress_v2", "wall_axial_end_action": "pipe_wall_endpoint_action_v2",
            "V_y/V_z/T/M_y/M_z": "element_local_shear_force_y/_z, element_local_torsional_moment, "
                                 "element_local_bending_moment_y/_z",
            "sigma_b_y/sigma_b_z/tau_t": "element_local_bending_normal_stress_y/_z, element_local_torsional_shear_"
                                         "stress",
            "lame_surface": "pipe_lame_radial_stress_v2 / pipe_lame_hoop_stress_v2 (inner, outer)",
            "arcs": "row kinds for arc rows under pressure-1 are defined by T4-U2 (provisional); the values here "
                    "are frame- and sign-exact",
            "chord_frame_elastic": "element_local_axial_force (if T4-U2 keeps it on arcs), element_local_shear_force_"
                                   "y/_z, element_local_torsional_moment, element_local_bending_moment_y/_z at end_i "
                                   "and end_j, coordinate_system arc_chord_frame, node-on-element",
        },
    }


if __name__ == "__main__":
    main(sys.argv[1] if len(sys.argv) > 1 else ".")
