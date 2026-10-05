"""RV89 part 2: independent evaluation of the generated profile (stdlib only).
1. Parses the Rust FORMS/ATOM_NAMES/ATOM_ASSUMED and checks each form against profile_tree.json.
2. Evaluates every form at the in-build atom values printed by the build (I65's record and RV89's own
   print), composes the phases from COMPOSITION_G4.md §1 (+ RV84 C-N3: T19 and the statics in every
   phase) as RV89 reads it, and reproduces the record's per-phase values and the maximum.
3. Prints W3's dominant terms and its sensitivity to each atom."""
import json, re, sys
RS, TREE, REC = sys.argv[1:4]
OWN = sys.argv[4] if len(sys.argv) > 4 else None
src = open(RS).read()
def block(name):
    m = re.search(r"pub\(crate\) const " + name + r": \[[^\]]+\] = \[\n(.*?)\n    \];", src, re.S)
    return m.group(1)
names = [l.strip().rstrip(',').strip('"') for l in block("ATOM_NAMES").split('\n')]
assumed = [int(l.strip().rstrip(',')) for l in block("ATOM_ASSUMED").split('\n')]
binds = [l.strip().rstrip(',').replace('Binding::', '') for l in block("ATOM_BINDINGS").split('\n')]
forms = {}
for m in re.finditer(r'Form \{ name: "([^"]+)", constant: (\d+), terms: &\[(.*?)\] \}', src):
    terms = [(int(a), int(c)) for a, c in re.findall(r"\((\d+), (\d+)\)", m.group(3))]
    forms[m.group(1)] = (int(m.group(2)), terms)
tree = json.load(open(TREE))
assert len(names) == 244 and len(forms) == 47, (len(names), len(forms))
# 1. transcription per form
bad = 0
for f, (c, terms) in forms.items():
    t = tree["forms"][f]
    mine = {"1": c} if c else {}
    for a, k in terms:
        mine[names[a]] = mine.get(names[a], 0) + k
    tt = {k: v for k, v in t.items() if v}
    if mine != tt:
        bad += 1; print("TRANSCRIPTION MISMATCH", f)
for n, a in zip(names, assumed):
    if tree["assumed"].get(n, tree["text_atoms"].get(n)) != a:
        bad += 1; print("ASSUMED MISMATCH", n)
print(f"transcription: 47 forms and 244 assumed atoms; mismatches={bad}")
# 2. in-build values
def read_atoms(path, tag):
    vals = {}
    for l in open(path):
        if l.startswith(tag):
            p = l.rstrip('\n').split('\t')
            vals[p[1]] = int(p[2])
    return vals
rec = read_atoms(REC, "I65_G5_ATOM")
assert len(rec) == 244
own = read_atoms(OWN, "RV89_ATOM") if OWN else None
if own:
    diff = [n for n in names if own[n] != rec[n]]
    print(f"RV89's own in-build atom values vs I65's record: {len(own)} atoms, differences={diff}")
def ev(f, v):
    c, terms = forms[f]
    return c + sum(v[names[a]] * k for a, k in terms)
T12_15 = ["T12", "T13", "T14", "T15"]
def comp(v, mode):
    O = ev("O_base_" + mode, v)
    t16 = max(ev(x, v) for x in ["T16_P1", "T16_P2", "T16_P3", "T16_P4"])
    t17 = max(ev(x, v) for x in ["T17_V1", "T17_V2_clone", "T17_V2_hash", "T17_V3", "T17_V4", "T17_V5", "T17_V6"]) + ev("T17_output", v)
    t25 = ev("T25_carried_case", v) + max(ev(x, v) for x in ["T25_S1", "T25_S2", "T25_S3", "T25_S4", "T25_S5", "T25_I1", "T25_I2", "T25_I3"])
    t1215 = sum(ev(x, v) for x in T12_15)
    every = ev("T11", v) + ev("T19", v) + ev("STATICS", v)   # C-N3(a), (b): T19 and the statics in every phase
    mv = lambda *xs: max(ev(x, v) for x in xs)
    W1 = O + ev("TAV_W", v) + every
    W2 = W1 + t1215 + ev("NOTICE", v)
    ph = {
        "W1": (W1, mv("TXT_moving", "HELPER_moving", "T07_moving")),
        "W2": (W2, mv("TXT_moving", "HELPER_moving", "NOTICE_moving")),
        "W3": (W2 + ev("STAGED", v) + t16, mv("TXT_moving", "HELPER_moving", "T16_moving")),
        "W4": (W2 + ev("SUCC", v) + ev("INVOC", v) + t17, mv("TXT_moving", "HELPER_moving", "T17_moving_publication", "T17_moving_invocation")),
        "W5": (W2 + ev("SUCC", v), mv("TXT_moving", "HELPER_moving")),
        "X1": (O + t25 + ev("TAV_X", v) + every, mv("T25_moving", "TXT_moving", "HELPER_moving")),
        "X2": (O + ev("BODY", v) + ev("TAV_X", v) + ev("NOTICE", v) + every, mv("TXT_moving", "HELPER_moving", "NOTICE_moving")),
    }
    return ph
R, M = 67108864, 4026531840
recph = {}
for l in open(REC):
    m = re.match(r"I65_G5_PHASE mode=(\w+) phase=(\w+) requested=(\d+) moving=(\d+)", l)
    if m: recph[(m.group(1), m.group(2))] = (int(m.group(3)), int(m.group(4)))
for label, v in [("record in-build", rec), ("RV89 in-build", own), ("chain assumed", dict(zip(names, assumed)))]:
    if v is None: continue
    for mode in ["sparse", "dense"]:
        ph = comp(v, mode)
        best = max(ph, key=lambda k: sum(ph[k]))
        e = sum(ph[best]) + R
        agree = all(ph[k] == recph[(mode, k)] for k in ph) if label != "chain assumed" else None
        print(f"{label:16s} {mode:6s} max={best} E_mov={sum(ph[best])} +R={e} frac={e/M:.6f} margin_0.9M={int(0.9*M)-e} margin_M={M-e} phases_equal_record={agree}")
        if label == "record in-build":
            for k, (q, mvv) in ph.items():
                print(f"   {k} requested={q} moving={mvv} +R={q+mvv+R} ({(q+mvv+R)/M:.4f} M)")
# 3. W3 composition and sensitivity (record in-build, dense)
v = rec
print("W3 dense terms (in-build):")
parts = [("O_base_dense", ev("O_base_dense", v)), ("TAV_W", ev("TAV_W", v))] + [(x, ev(x, v)) for x in T12_15] + \
        [(x, ev(x, v)) for x in ["NOTICE", "T11", "T19", "STATICS", "STAGED"]] + [("T16 (P2)", ev("T16_P2", v)), ("T16_moving", ev("T16_moving", v))]
tot = sum(p for _, p in parts)
for n, p in sorted(parts, key=lambda x: -x[1]):
    print(f"   {n:14s} {p:>14,d}  {p/M:.4f} M")
print(f"   total+R {tot+R:,d}")
w3forms = ["O_base_dense", "TAV_W", "T12", "T13", "T14", "T15", "NOTICE", "T11", "T19", "STATICS", "STAGED", "T16_P2"]
coef = {}
const = 0
for f in w3forms:
    c, terms = forms[f]; const += c
    for a, k in terms: coef[names[a]] = coef.get(names[a], 0) + k
lay = sum(v[n] * k for n, k in coef.items())
print(f"W3 dense: layout-free constants {const:,d} ({const/M:.4f} M); layout terms {lay:,d} ({lay/M:.4f} M)")
print("largest layout contributions (atom: coefficient x in-build value = bytes; bytes per +1 B of stride):")
for n, k in sorted(coef.items(), key=lambda x: -x[1] * v[x[0]])[:15]:
    print(f"   {n:32s} {binds[names.index(n)]:11s} {k:>8,d} x {v[n]:>6} = {k*v[n]:>13,d}")
est = [n for n, b in zip(names, binds) if b == "Estimate"]
ew = sum(coef.get(n, 0) * v[n] for n in est)
print(f"Estimate atoms in W3 dense: {sum(1 for n in est if coef.get(n,0))} with weight {ew:,d}")
up10 = sum(coef[n] * v[n] for n in coef) * 0.10
print(f"+10% on every layout atom moves W3 by {up10:,.0f} B")
# 4. The Estimate atoms: total coefficient in each phase's requested+moving forms; break-even stride.
print("Estimate atoms (dense): stride, coefficient in W3 / in the largest phase using it, weight in W3, stride that alone would consume the 0.9 M margin")
margin = int(0.9 * M) - (sum(comp(rec, "dense")["W3"]) + R)
allf = {f for f in forms}
for n in est:
    i = names.index(n)
    users = [f for f, (c, t) in forms.items() if any(a == i for a, _ in t)]
    k3 = coef.get(n, 0)
    # the max over phases of the atom's coefficient (rough: sum over forms in each phase list)
    print(f"   {n:28s} stride={rec[n]:>5} W3coef={k3:>7,d} W3weight={k3*rec[n]:>10,d} forms={','.join(users)} breakeven_stride={'inf' if not k3 else rec[n] + margin//k3:,}")
