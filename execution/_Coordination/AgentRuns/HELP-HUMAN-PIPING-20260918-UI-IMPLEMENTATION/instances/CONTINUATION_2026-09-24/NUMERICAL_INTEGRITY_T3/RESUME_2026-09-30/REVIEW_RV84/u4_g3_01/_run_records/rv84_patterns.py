import re, os, sys, json, collections
ROOT = sys.argv[1]  # the P root, read at NUM 5ae5fe4f0f
CRATES = ["core/product_physics","core/solver/frame_kernel","core/serialization/canonical_json","core/solver/curved_bend",
 "core/loads/load_case_algebra","core/solver/linear_supports","core/solver/nonlinear_integration","core/solver/nonlinear_supports",
 "core/loads/primitive_loads","core/solver/diagnostics","core/solver/sparse_direct","core/solver/straight_pipe",
 "core/loads/stress_recovery","core/units"]
def files():
    for c in CRATES:
        for d, _, fs in os.walk(os.path.join(ROOT, c, "src")):
            if "/bin" in d: continue
            for f in sorted(fs):
                if f.endswith(".rs") and "test" not in f:
                    yield os.path.join(d, f)
def strip(t):
    m = re.search(r"#\[cfg\(test\)\]\s*\n\s*(pub(\(crate\))?\s+)?mod\s+\w+\s*\{", t)
    if m: t = t[:m.start()]
    t = re.sub(r"//[^\n]*", lambda m: " " * len(m.group(0)), t)
    t = re.sub(r"/\*.*?\*/", lambda m: " " * len(m.group(0)), t, flags=re.S)
    return t
res = collections.defaultdict(list)
P = {
 "dyn": re.compile(r"\bdyn\s+[A-Z]\w*"),
 "impl_fn_param": re.compile(r"\b(impl\s+Fn(Mut|Once)?\s*\(|[A-Z]\s*:\s*Fn(Mut|Once)?\s*\(|where[^{]*Fn(Mut|Once)?\s*\()"),
 "impl_trait_param": re.compile(r":\s*&?\s*(mut\s+)?impl\s+(?!Fn|Into<String>|Iterator|IntoIterator)[A-Z]\w*"),
 "macro_rules": re.compile(r"\bmacro_rules!\s*(\w+)"),
 "op_impl": re.compile(r"\bimpl\b[^{;]*\b(Add|Sub|Mul|Div|Neg|AddAssign|SubAssign|MulAssign|Index|IndexMut|Deref|DerefMut|Drop|Iterator|From|TryFrom|FromStr|Ord|PartialOrd|PartialEq|Hash|Serialize|Deserialize|Default|Clone|Sum)\b(<[^{]*>)?\s+for\s+([A-Za-z_][\w:<>, ]*)"),
 "fmt_impl": re.compile(r"\bimpl\b[^{;]*\b(fmt::)?(Display|Debug|LowerHex|LowerExp)\s+for\s+([A-Za-z_][\w:<>, ]*)"),
}
for f in files():
    raw = open(f, encoding="utf-8").read()
    t = strip(raw)
    for k, rx in P.items():
        for m in rx.finditer(t):
            line = t.count("\n", 0, m.start()) + 1
            # body of the impl block for impl kinds
            body = ""
            if k in ("op_impl", "fmt_impl"):
                b = t.find("{", m.end()); d = 0; i = b
                while i < len(t):
                    if t[i] == "{": d += 1
                    elif t[i] == "}":
                        d -= 1
                        if d == 0: break
                    i += 1
                body = t[b:i]
            alloc = bool(re.search(r"format!|\.to_string\(\)|to_owned\(\)|String::from|\.collect::<String>|\.join\(|vec!\[|Vec::new|\.clone\(\)|json!", body))
            res[k].append({"file": f.split(ROOT + "/")[1], "line": line, "text": m.group(0)[:90], "alloc_in_body": alloc, "body_len": len(body)})
out = {k: v for k, v in res.items()}
json.dump(out, open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "rv84_patterns.out.json"), "w"), indent=1)
for k, v in out.items():
    print(k, len(v), "with alloc in body:", sum(1 for x in v if x["alloc_in_body"]))
