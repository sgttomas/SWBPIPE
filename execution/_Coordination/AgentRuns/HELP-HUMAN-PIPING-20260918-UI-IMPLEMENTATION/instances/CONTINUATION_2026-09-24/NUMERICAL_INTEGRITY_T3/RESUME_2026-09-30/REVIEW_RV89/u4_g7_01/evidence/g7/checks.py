"""RV89 G7 (scratch): T17 stages at in-build atoms from the committed forms and RV89's own law-test
record; F5 at pushcap(D_env) + D_env; the new static's parsed-tree bound (G4's static rule)."""
import json, re, sys
rm, log, fixture = sys.argv[1:4]
src = open(rm).read(); L = open(log).read()
atoms = [(m.group(1), int(m.group(2))) for m in re.finditer(r"I65_G5_ATOM \w+\t([^\t]+)\t(\d+)", L)]
forms = {m.group(1): (int(m.group(2)), [(int(a), int(c)) for a, c in re.findall(r"\((\d+), (\d+)\)", m.group(3))])
         for m in re.finditer(r'Form \{ name: "(\w+)", constant: (\d+), terms: &\[([^\]]*)\] \}', src)}
ev = lambda n: forms[n][0] + sum(c * atoms[a][1] for a, c in forms[n][1])
print("atoms", len(atoms), "| atom 20 =", atoms[20])
st = {n: ev(n) for n in forms if n.startswith("T17_V")}
for n, v in st.items():
    print(f"  {n:13s} {v:>15,}  s(&Value) coefficient: {[c for a, c in forms[n][1] if a == 20] or 'none'}")
def pushcap(h, mn=4):
    c = 0
    for k in range(1, h + 1):
        if k > c: c = max(2 * c, k, mn)
    return c
D = 9361
f5 = atoms[20][1] * (pushcap(D) + D)
print(f"D_env {D}; pushcap(D_env) {pushcap(D)}; F5 = 8 x ({pushcap(D)} + {D}) = {f5:,} B; coefficient delta {pushcap(D) + D}")
print(f"V4 + F5 = {st['T17_V4'] + f5:,}; V2_hash - (V4 + F5) = {st['T17_V2_hash'] - st['T17_V4'] - f5:,}; argmax {max(st, key=st.get)}")
print(f"F5 moving extra (last growth) = {atoms[20][1] * pushcap(D) // 2:,} B; T17_moving_publication = {ev('T17_moving_publication'):,}")
d = json.load(open(fixture)); f = dict(arr=0, obj=0, ent=0, strb=0, keyb=0)
def w(v):
    if isinstance(v, dict):
        f["obj"] += 1; f["ent"] += len(v)
        for k, x in v.items(): f["keyb"] += len(k.encode()); w(x)
    elif isinstance(v, list):
        f["arr"] += len(v); [w(x) for x in v]
    elif isinstance(v, str): f["strb"] += len(v.encode())
w(d)
sV = dict(atoms)["s(Value)"]; node = dict(atoms)["Node(String,Value)"]
print("new static", f, f"bound = 32x6x{f['arr']} + 736x({f['obj']}+{f['ent']}//5) + {f['strb']}+{f['keyb']} =",
      sV * 6 * f["arr"] + node * (f["obj"] + f["ent"] // 5) + f["strb"] + f["keyb"])
