"""I65 U4 G3: over-approximate name-based call graph and its strongly connected components
(stdlib only, read-only).

Every non-test `fn` body in the given crate source trees becomes a node keyed
`file:line:name` (closures belong to their enclosing fn). An edge f -> g is added
when f's body contains `name(` (free call), `.name(` (method call) or `X::name(`
(path call) and some fn named `name` is defined in the scanned set. Resolution is by
NAME ONLY, so every same-named fn is a target: this over-approximates the real graph.
Calls through function pointers and trait objects are covered only if the trait
method's name appears; derive-generated code (serde, Debug, Clone, Drop) is NOT
seen (STACK_INVENTORY.md handles it by type structure).

Output: node count, edge count, every strongly connected component that has a
cycle (size > 1, or a self-loop), and the longest acyclic call depth from the given
root names over the condensation (a depth bound for the over-approximation).
Usage: python3 callgraph.py <repo-relative root> <out json> <root fn names, comma-separated> <dir or file> ...
"""
import json, os, re, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import loopscan
sys.setrecursionlimit(100000)

root, out_path, root_names = sys.argv[1], sys.argv[2], sys.argv[3].split(",")
paths = []
for p in sys.argv[4:]:
    full = os.path.join(root, p)
    if os.path.isdir(full):
        for d, _, fs in os.walk(full):
            if "/target" in d or "/tests" in d or "/benches" in d or "/bin" in d and "src/bin" in d:
                continue
            for f in sorted(fs):
                if f.endswith(".rs") and "test" not in f:
                    paths.append(os.path.relpath(os.path.join(d, f), root))
    else:
        paths.append(p)

FN = re.compile(r"\bfn\s+([A-Za-z_][A-Za-z0-9_]*)\s*(?:<[^{;]*?>)?\s*\(")
# a path call keeps its LAST qualifier: `a::B::f(` -> qual B, name f (G3 fix: multi-segment paths
# were previously not matched at all, which dropped edges such as `exact::Context::prepare_with_budget`)
CALL = re.compile(r"(?:(?<=[^A-Za-z0-9_:.])|^)(?:(?:[A-Za-z_][A-Za-z0-9_]*\s*::\s*)*([A-Za-z_][A-Za-z0-9_]*)\s*::\s*)?([A-Za-z_][A-Za-z0-9_]*)\s*(?:::<[^>]*>)?\s*\(")
METHOD = re.compile(r"\.\s*([A-Za-z_][A-Za-z0-9_]*)\s*(?:::<[^>]*>)?\s*\(")
KEYWORDS = {"if", "while", "for", "match", "return", "loop", "fn", "Some", "Ok", "Err", "None", "move", "in",
            "let", "as", "where", "impl", "struct", "enum", "mod", "use", "pub", "unsafe", "Box", "Vec",
            "vec", "format", "write", "writeln", "println", "assert", "assert_eq", "debug_assert", "panic",
            "matches", "json", "unreachable", "todo", "Self", "self", "super", "crate", "dyn", "ref", "mut"}

def strip(text):
    m = re.search(r"#\[cfg\(test\)\]\s*\n\s*(pub(\(crate\))?\s+)?mod\s+\w+\s*\{", text)
    if m:
        text = text[:m.start()]
    text = re.sub(r"//[^\n]*", lambda m: " " * len(m.group(0)), text)
    text = re.sub(r"/\*.*?\*/", lambda m: " " * len(m.group(0)), text, flags=re.S)
    text = re.sub(r'"(?:\\.|[^"\\])*"', lambda m: '"' + " " * (len(m.group(0)) - 2) + '"', text)
    text = re.sub(r"'(?:\\.|[^'\\])'", "' '", text)
    return text

def crate_of(rel):
    parts = rel.split("/")
    i = parts.index("src") if "src" in parts else len(parts) - 1
    return "/".join(parts[:i])

IMPL = re.compile(r"\bimpl\b\s*(?:<[^{]*?>)?\s*([^{]*?)\{")
TRAIT = re.compile(r"\btrait\s+([A-Za-z_][A-Za-z0-9_]*)[^{]*\{")
def owners(text):
    """Return list of (start, end, owner_type, trait) spans for impl/trait blocks."""
    out = []
    for m in list(IMPL.finditer(text)) + list(TRAIT.finditer(text)):
        head = m.group(1).strip() if m.re is IMPL else ""
        brace = m.end() - 1
        depth, i = 0, brace
        while i < len(text):
            if text[i] == "{": depth += 1
            elif text[i] == "}":
                depth -= 1
                if depth == 0: break
            i += 1
        if m.re is TRAIT:
            out.append((brace, i, "trait:" + m.group(1), m.group(1)))
        else:
            head = re.sub(r"\bwhere\b.*", "", head, flags=re.S).strip()
            if " for " in head:
                trait, ty = head.split(" for ", 1)
            else:
                trait, ty = "", head
            ty = re.sub(r"<.*", "", ty.strip()).split("::")[-1].strip("&* ")
            trait = re.sub(r"<.*", "", trait.strip()).split("::")[-1]
            out.append((brace, i, ty, trait))
    return out

nodes, bodies, by_name = [], {}, {}
owner_of, crate_key, methods_by = {}, {}, {}
STRUCT = re.compile(r"\bstruct\s+([A-Za-z_][A-Za-z0-9_]*)\s*(?:<[^{;(]*?>)?\s*(?:where[^{;]*)?\{")
fields_of, params_of = {}, {}
PRIM = {"f64","f32","u8","u16","u32","u64","u128","i8","i16","i32","i64","i128","usize","isize","bool","char","str","String"}
STDC = {"Vec","Option","Result","HashMap","HashSet","BTreeMap","BTreeSet","VecDeque","BinaryHeap","Cell","RefCell","Range"}
def base(ty):
    """Normalize a Rust type text to (kind, name, elem): kind in {type, prim, std, unknown}."""
    t = ty.strip()
    t = re.sub(r"^&\s*('\w+\s+)?(mut\s+)?", "", t).strip()
    t = re.sub(r"^(mut|dyn|impl)\s+", "", t).strip()
    if t.startswith("["):
        inner = t[1:].rsplit(";", 1)[0] if ";" in t else t[1:-1]
        return ("std", "array", inner.strip().rstrip("]"))
    m = re.match(r"([A-Za-z_][A-Za-z0-9_:]*)\s*(<(.*)>)?$", t, re.S)
    if not m:
        return ("unknown", t, None)
    name = m.group(1).split("::")[-1]
    args = m.group(3)
    if name in ("Box","Arc","Rc") and args:
        return base(args)
    if name in PRIM:
        return ("prim", name, None)
    if name in STDC:
        elem = args.split(",")[0] if args else None
        return ("std", name, elem)
    return ("type", name, None)
for rel in paths:
    raw = open(os.path.join(root, rel), encoding="utf-8").read()
    text = strip(raw)
    blocks = owners(text)
    for sm in STRUCT.finditer(text):
        brace = sm.end() - 1
        depth, i = 0, brace
        while i < len(text):
            if text[i] == "{": depth += 1
            elif text[i] == "}":
                depth -= 1
                if depth == 0: break
            i += 1
        body = text[brace + 1:i]
        fd = fields_of.setdefault(sm.group(1), {})
        d, cur, parts = 0, "", []
        for ch in body:
            if ch in "<([": d += 1
            elif ch in ">)]": d -= 1
            if ch == "," and d == 0:
                parts.append(cur); cur = ""
            else:
                cur += ch
        parts.append(cur)
        for part in parts:
            part = re.sub(r"#\[[^\]]*\]", "", part).strip()
            part = re.sub(r"^pub(\([^)]*\))?\s+", "", part)
            if ":" in part:
                fname, fty = part.split(":", 1)
                fd[fname.strip()] = fty.strip()
    for m in FN.finditer(text):
        brace = text.find("{", m.end())
        semi = text.find(";", m.end())
        if brace < 0 or (0 <= semi < brace):
            continue
        depth, i = 0, brace
        while i < len(text):
            if text[i] == "{":
                depth += 1
            elif text[i] == "}":
                depth -= 1
                if depth == 0:
                    break
            i += 1
        line = text.count("\n", 0, m.start()) + 1
        key = f"{rel}:{line}:{m.group(1)}"
        # parameter types between the fn's '(' and its matching ')'
        po = m.end() - 1
        d, j = 0, po
        while j < len(text):
            if text[j] == "(": d += 1
            elif text[j] == ")":
                d -= 1
                if d == 0: break
            j += 1
        ptext = text[po + 1:j]
        d, cur, parts = 0, "", []
        for ch in ptext:
            if ch in "<([": d += 1
            elif ch in ">)]": d -= 1
            if ch == "," and d == 0:
                parts.append(cur); cur = ""
            else:
                cur += ch
        parts.append(cur)
        pt = {}
        for part in parts:
            if ":" in part and "::" not in part.split(":", 1)[0]:
                pname, pty = part.split(":", 1)
                pt[pname.replace("mut ", "").strip()] = pty.strip()
        params_of[key] = pt
        nodes.append(key)
        bodies[key] = (rel, brace, i, text)
        enclosing = [b for b in blocks if b[0] < m.start() < b[1]]
        own = min(enclosing, key=lambda b: b[1] - b[0]) if enclosing else None
        owner_of[key] = own[2] if own else None
        crate_key[key] = crate_of(rel)
        if own:
            methods_by.setdefault(m.group(1), []).append(key)
        else:
            by_name.setdefault(m.group(1), []).append(key)

def module_of(rel):
    base = os.path.basename(rel)
    if base in ("mod.rs", "lib.rs", "main.rs"):
        return os.path.basename(os.path.dirname(rel)) if base == "mod.rs" else None
    return base[:-3]

# Cargo dependency edges (from each crate's Cargo.toml path dependencies); a crate may call only
# itself and its transitive dependencies, so a call cycle cannot cross crates.
DEPS = {
 "core/product_physics": ["core/serialization/canonical_json", "core/solver/curved_bend", "core/solver/frame_kernel",
    "core/loads/load_case_algebra", "core/solver/linear_supports", "core/solver/nonlinear_integration",
    "core/solver/nonlinear_supports", "core/loads/primitive_loads", "core/solver/diagnostics",
    "core/solver/sparse_direct", "core/solver/straight_pipe", "core/loads/stress_recovery", "core/units"],
 "core/solver/sparse_direct": ["core/solver/frame_kernel"],
 "core/solver/linear_supports": ["core/solver/frame_kernel"],
 "core/solver/straight_pipe": ["core/solver/frame_kernel"],
 "core/solver/nonlinear_integration": ["core/solver/curved_bend", "core/solver/frame_kernel", "core/solver/nonlinear_supports",
    "core/solver/diagnostics", "core/solver/sparse_direct"],
 "core/solver/curved_bend": ["core/solver/frame_kernel"],
 "core/solver/nonlinear_supports": ["core/solver/linear_supports", "core/solver/diagnostics"],
 "core/solver/diagnostics": ["core/solver/frame_kernel", "core/solver/linear_supports", "core/loads/primitive_loads", "core/solver/sparse_direct"],
 "core/loads/primitive_loads": ["core/solver/frame_kernel", "core/solver/linear_supports"],
 "core/loads/stress_recovery": ["core/loads/primitive_loads", "core/solver/straight_pipe"],
 "core/loads/load_case_algebra": ["core/loads/primitive_loads", "core/solver/frame_kernel"],
 "core/reporting/result_export": ["core/serialization/canonical_json", "core/units"],
}
def closure(c, seen=None):
    seen = set() if seen is None else seen
    if c in seen:
        return seen
    seen.add(c)
    for d in DEPS.get(c, []):
        closure(d, seen)
    return seen
DEPS_CLOSURE = {c: closure(c) for c in set(crate_key.values())}

# optional manual adjudication rules (CG_RULES env var): (caller, name) -> real targets
RULES = {}
if os.environ.get("CG_RULES"):
    rj = json.load(open(os.environ["CG_RULES"]))
    W = rj.get("W", "")
    for r in rj["rules"]:
        c = r["caller"].replace("W/", W + "/", 1)
        RULES[(c, r["name"])] = [t.replace("W/", W + "/", 1) for t in r["targets"]]
    missing = [k for k in RULES if k[0] not in bodies]
    if missing:
        print("RULE CALLERS NOT FOUND:", missing)

# exclude nested fn bodies from their parent's body when scanning calls
spans = {k: (v[1], v[2]) for k, v in bodies.items()}
edges = {k: set() for k in nodes}
site_loops = {}
for key in nodes:
    rel, a, b, text = bodies[key]
    inner = [spans[o] for o in nodes if o != key and bodies[o][0] == rel and a < spans[o][0] and spans[o][1] <= b]
    body = list(text[a:b])
    for s, e in inner:
        for i in range(s - a, e - a):
            body[i] = " "
    body = "".join(body)
    cr = crate_key[key]
    allowed = DEPS_CLOSURE.get(cr, {cr})
    # loop spans inside this body: (start, end, header)
    lspans = loopscan.loop_spans(body)
    def loops_at(pos):
        return tuple(h for (a2, b2, h) in lspans if a2 < pos < b2)
    def add_edge(t, pos):
        edges[key].add(t)
        site_loops.setdefault((key, t), {}).setdefault(pos, loops_at(pos))
    # closures bound by `let name = |..|` / `let name = move |..|` shadow any fn of that name
    closures = set(re.findall(r"\blet\s+(?:mut\s+)?([a-z_][a-z0-9_]*)\s*(?::[^=;]*)?=\s*(?:move\s*)?\|", body))
    for m in CALL.finditer(body):
        qual, name = m.group(1), m.group(2)
        if name in KEYWORDS:
            continue
        if qual is None and name in closures:
            continue
        if qual in ("Self", "self") or (qual is None and False):
            targets = [t for t in methods_by.get(name, []) if owner_of[t] in (owner_of[key],) or str(owner_of[t]).startswith("trait:")]
        elif qual and qual[0].isupper():
            targets = [t for t in methods_by.get(name, []) if owner_of[t] in (qual, "trait:" + qual)] + by_name.get(name, [])
        else:
            # Rust name resolution: a bare call resolves to the module-level fn of the
            # caller's own module when one exists (a `use` of the same name would be a
            # compile error, E0255); `module::name(` resolves to that module's fn. Only
            # when neither applies is every same-named fn kept (over-approximation).
            cands = by_name.get(name, [])
            if qual is None:
                same = [t for t in cands if bodies[t][0] == rel]
                if same:
                    cands = same
            else:
                inmod = [t for t in cands if module_of(bodies[t][0]) == qual]
                if inmod:
                    cands = inmod
            targets = cands
        if (key, name) in RULES:
            targets = [t for t in RULES[(key, name)] if t in bodies]
        for t in targets:
            if crate_key[t] in allowed:
                add_edge(t, m.start())
    # functions passed as values, e.g. `.any(negative_zero)` or `.map(Self::helper)`.
    # Names bound locally (params, let/for/closure/match bindings, struct-literal fields) are values, not fns.
    bound = set(params_of.get(key, {}))
    for bm in re.finditer(r"\blet\s+(?:mut\s+)?\(?([^=:;]*?)\)?\s*[:=]", body):
        bound.update(re.findall(r"[a-z_][a-z0-9_]*", bm.group(1)))
    for bm in re.finditer(r"\bfor\s+\(?([^)]*?)\)?\s+in\b", body):
        bound.update(re.findall(r"[a-z_][a-z0-9_]*", bm.group(1)))
    for bm in re.finditer(r"\|([^|]*)\|", body):
        bound.update(re.findall(r"[a-z_][a-z0-9_]*", bm.group(1)))
    for bm in re.finditer(r"(?:Some|Ok|Err|[A-Z][A-Za-z0-9_]*)\s*\(([^()]*)\)\s*=>", body):
        bound.update(re.findall(r"[a-z_][a-z0-9_]*", bm.group(1)))
    for bm in re.finditer(r"\{([^{}]*)\}\s*=>", body):
        bound.update(re.findall(r"[a-z_][a-z0-9_]*", bm.group(1)))
    for m in re.finditer(r"[(,]\s*(?:([A-Za-z_][A-Za-z0-9_]*)::)?([a-z_][a-z0-9_]*)\s*(?=[),])", body):
        qual, name = m.group(1), m.group(2)
        if qual is None and name in bound:
            continue
        if qual and qual[0].isupper():
            targets = [t for t in methods_by.get(name, []) if owner_of[t] in (qual, owner_of[key])]
        else:
            targets = by_name.get(name, [])
        for t in targets:
            if crate_key[t] in allowed:
                add_edge(t, m.start())
    locals_ = dict(params_of.get(key, {}))
    for lm in re.finditer(r"\blet\s+(?:mut\s+)?([a-z_][A-Za-z0-9_]*)\s*(?::\s*([^=;]+?))?\s*=\s*([A-Z][A-Za-z0-9_]*)?(::|\s*\{)?", body):
        if lm.group(2):
            locals_[lm.group(1)] = lm.group(2)
        elif lm.group(3) and lm.group(4):
            locals_[lm.group(1)] = lm.group(3)
    def resolve(recv):
        toks = re.split(r"\.", recv)
        first = toks[0]
        idx = first.endswith("]")
        first = re.sub(r"\[.*$", "", first)
        if first == "self":
            cur = ("type", owner_of[key], None) if owner_of[key] else ("unknown", None, None)
        elif first in locals_:
            cur = base(locals_[first])
        else:
            return ("unknown", None, None)
        if idx:
            cur = base(cur[2]) if cur[0] == "std" and cur[2] else ("unknown", None, None)
        for f in toks[1:]:
            fi = f.endswith("]")
            f = re.sub(r"\[.*$", "", f)
            if cur[0] != "type" or cur[1] not in fields_of or f not in fields_of[cur[1]]:
                return ("unknown", None, None)
            cur = base(fields_of[cur[1]][f])
            if fi:
                cur = base(cur[2]) if cur[0] == "std" and cur[2] else ("unknown", None, None)
        return cur
    for m in re.finditer(r"(self|[A-Za-z_][A-Za-z0-9_\]\[\.]*)\s*\.\s*([A-Za-z_][A-Za-z0-9_]*)\s*(?:::<[^>]*>)?\s*\(", body):
        recv, name = m.group(1).rstrip("."), m.group(2)
        kind, tname, _ = resolve(recv)
        if kind == "type" and tname:
            targets = [t for t in methods_by.get(name, []) if owner_of[t] == tname or str(owner_of[t]).startswith("trait:")]
        elif kind in ("prim", "std"):
            targets = [t for t in methods_by.get(name, []) if owner_of[t] == tname or str(owner_of[t]).startswith("trait:")]
        else:
            targets = methods_by.get(name, [])
        if (key, name) in RULES:
            targets = [t for t in RULES[(key, name)] if t in bodies]
        for t in targets:
            if crate_key[t] in allowed:
                add_edge(t, m.start())

# Tarjan SCC (iterative)
index, low, onstack, stack, sccs, idx = {}, {}, set(), [], [], [0]
for v0 in nodes:
    if v0 in index:
        continue
    work = [(v0, iter(sorted(edges[v0])))]
    index[v0] = low[v0] = idx[0]; idx[0] += 1; stack.append(v0); onstack.add(v0)
    while work:
        v, it = work[-1]
        advanced = False
        for w in it:
            if w not in index:
                index[w] = low[w] = idx[0]; idx[0] += 1; stack.append(w); onstack.add(w)
                work.append((w, iter(sorted(edges[w])))); advanced = True; break
            elif w in onstack:
                low[v] = min(low[v], index[w])
        if advanced:
            continue
        work.pop()
        if work:
            low[work[-1][0]] = min(low[work[-1][0]], low[v])
        if low[v] == index[v]:
            comp = []
            while True:
                w = stack.pop(); onstack.discard(w); comp.append(w)
                if w == v:
                    break
            sccs.append(comp)
cyclic = [sorted(c) for c in sccs if len(c) > 1 or c[0] in edges[c[0]]]
comp_of = {v: i for i, c in enumerate(sccs) for v in c}
# longest path in condensation from roots
cedges = {}
for v in nodes:
    for w in edges[v]:
        if comp_of[v] != comp_of[w]:
            cedges.setdefault(comp_of[v], set()).add(comp_of[w])
memo = {}
def depth(c):
    if c in memo:
        return memo[c]
    memo[c] = 0
    best = 0
    for d in cedges.get(c, ()):
        best = max(best, 1 + depth(d))
    memo[c] = best
    return best
roots = [k for n in root_names for k in by_name.get(n, []) + methods_by.get(n, [])]
cyc_edges = []
for c in cyclic:
    cs = set(c)
    cyc_edges.append(sorted([v, w] for v in c for w in edges[v] if w in cs))
out = {"rules_applied": len(RULES), "files": len(paths), "nodes": len(nodes), "edges": sum(len(e) for e in edges.values()),
       "cyclic_components": cyclic, "cyclic_component_edges": cyc_edges,
       "root_depths": {r: depth(comp_of[r]) for r in roots}}
json.dump(out, open(out_path, "w"), indent=1)
if os.environ.get("CG_EDGES_OUT"):
    json.dump({"edges": {k: sorted(v) for k, v in edges.items()},
               "site_loops": [[a, b, [list(x) for x in ls.values()]] for (a, b), ls in site_loops.items()],
               "spans": {k: [bodies[k][0], bodies[k][0] and bodies[k][1], bodies[k][2]] for k in nodes}},
              open(os.environ["CG_EDGES_OUT"], "w"))
print(json.dumps({k: (len(v) if k.startswith("cyclic") else v) for k, v in out.items()}, indent=1))
