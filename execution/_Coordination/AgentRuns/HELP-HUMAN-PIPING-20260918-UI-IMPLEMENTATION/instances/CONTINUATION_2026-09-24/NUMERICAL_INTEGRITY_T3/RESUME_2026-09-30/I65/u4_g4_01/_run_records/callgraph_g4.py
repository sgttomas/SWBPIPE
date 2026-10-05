"""I65 U4 G4 (RV83 R-1 repair of the G3 graph): name-based call graph and its strongly
connected components (stdlib only, read-only).

G4 repairs (each switchable off for the baseline comparison with CG_REPAIR=0):
  R1a  a receiver whose declared type is a generic parameter of the fn or of its impl, a
       trait name (`impl Trait`, `dyn Trait`), a type alias or an associated-type path
       (`R::X`, `Self::X`), or `self`/`Self` inside a trait's default body, fans out to
       every same-named method instead of resolving to a type that owns none;
  R1b  methods of blanket impls (`impl<T..> Tr for T`) are targets for every receiver;
  R1c  a chained call (`f(x).g(`, `x?.g(`, `x.unwrap().g(`, `}.g(`, a `.g(` after a line
       break) is matched by the METHOD regex and fans out (G3 defined METHOD but never used it);
  R1d  a qualified path whose qualifier ends in a generic or turbofish (`<T as Tr>::f(`,
       `Type::<T>::f(`) fans out to every same-named fn and method;
  R1e  a path call `G::f(` whose qualifier is a generic parameter, trait or alias fans out;
  R1f  implicit calls (CG_IMPLICIT=1): `?` and `.into()` reach every user `From::from`;
       formatting reaches every user `fmt`; serde entry points reach the custom
       Deserialize/Visitor/Seed methods; `+`/`-` in the retained work module reach
       WorkTotal's Add/Sub; ordering of the stress-recovery heap reaches Node's Ord/Eq;
       a Deref target type reaches its `deref`.
  R1l  (refinement) a chained call resolves on the declared return types of every candidate of the call
       just before it (after `?`, unwrap, expect, clone, as_ref); it fans out when any is unresolvable;
  R1k  every #[cfg(test)] / #[cfg(any(test, ..))] item is blanked in place; G3 cut each file at its first
       test module, which drops production code after a mid-file test module;
  R1j  (refinement) a crate-local fn (no plain `pub`, not a trait method) is never a cross-crate target;
  R1i  a fn whose signature contains an array type `[T; N]` is a node (G3 dropped it as a declaration);
  R1h  a call right after a single `:` (json! values, struct fields) or after `..` is a call;
  R1g  string/char lexing by a single-pass lexer (loopscan.blank_rust; CG_LEXER=regex restores G3's);
  AUDIT every call-shaped token `name(` whose name is defined in the scanned set and that
       produced no edge is written to CG_AUDIT_OUT with the pass's reason.
Original G3 description follows.

I65 U4 G3: over-approximate name-based call graph and its strongly connected components
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
import json, os, re, sys, collections
REPAIR = os.environ.get("CG_REPAIR", "1") != "0"
IMPLICIT = os.environ.get("CG_IMPLICIT", "0") == "1"
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

FN = re.compile(r"\bfn\s+([A-Za-z_][A-Za-z0-9_]*)\s*(<[^{;]*?>)?\s*\(")
TURBO = re.compile(r">\s*::\s*([A-Za-z_][A-Za-z0-9_]*)\s*(?:::<[^>]*>)?\s*\(")
def generic_names(g):
    """Type-parameter names of a `<...>` list (lifetimes and const parameters dropped)."""
    if not g:
        return set()
    g = g.strip()[1:-1] if g.strip().startswith("<") else g
    d, cur, parts = 0, "", []
    for ch in g:
        if ch in "<([": d += 1
        elif ch in ">)]": d -= 1
        if ch == "," and d == 0:
            parts.append(cur); cur = ""
        else:
            cur += ch
    parts.append(cur)
    out = set()
    for x in parts:
        x = x.strip()
        if not x or x.startswith("'") or x.startswith("const "):
            continue
        m = re.match(r"([A-Za-z_][A-Za-z0-9_]*)", x)
        if m:
            out.add(m.group(1))
    return out

# a path call keeps its LAST qualifier: `a::B::f(` -> qual B, name f (G3 fix: multi-segment paths
# were previously not matched at all, which dropped edges such as `exact::Context::prepare_with_budget`)
CALL_G3 = re.compile(r"(?:(?<=[^A-Za-z0-9_:.])|^)(?:(?:[A-Za-z_][A-Za-z0-9_]*\s*::\s*)*([A-Za-z_][A-Za-z0-9_]*)\s*::\s*)?([A-Za-z_][A-Za-z0-9_]*)\s*(?:::<[^>]*>)?\s*\(")
# R1h: G3's lookbehind also rejected a call right after a single `:` (`"key":f(x)` in json!,
# `field:Vec::new()`) and after `..` (`..Default::default()`); both are calls
CALL = re.compile(r"(?:(?<=[^A-Za-z0-9_:.])|(?<=[^:]:)|(?<=\.\.)|^)(?:(?:[A-Za-z_][A-Za-z0-9_]*\s*::\s*)*([A-Za-z_][A-Za-z0-9_]*)\s*::\s*)?([A-Za-z_][A-Za-z0-9_]*)\s*(?:::<[^>]*>)?\s*\(")
METHOD = re.compile(r"\.\s*([A-Za-z_][A-Za-z0-9_]*)\s*(?:::<[^>]*>)?\s*\(")
KEYWORDS = {"if", "while", "for", "match", "return", "loop", "fn", "Some", "Ok", "Err", "None", "move", "in",
            "let", "as", "where", "impl", "struct", "enum", "mod", "use", "pub", "unsafe", "Box", "Vec",
            "vec", "format", "write", "writeln", "println", "assert", "assert_eq", "debug_assert", "panic",
            "matches", "json", "unreachable", "todo", "Self", "self", "super", "crate", "dyn", "ref", "mut"}

def strip(text):
    if os.environ.get("CG_LEXER", "rust") != "regex":
        # R1g: the G3 regexes mis-lexed `\`-newline strings and `'"'`; R1k: test items are blanked
        # wherever they sit (G3 cut the file at its first test module)
        return loopscan.blank_test_items(loopscan.blank_rust(text))
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

IMPL = re.compile(r"\bimpl\b\s*(<(?:[^<>{]|<[^<>{]*(?:<[^<>{]*>[^<>{]*)*>)*>)?\s*([^{]*?)\{")
TRAIT = re.compile(r"\btrait\s+([A-Za-z_][A-Za-z0-9_]*)[^{]*\{")
def owners(text):
    """Return list of (start, end, owner_type, trait) spans for impl/trait blocks."""
    out = []
    for m in list(IMPL.finditer(text)) + list(TRAIT.finditer(text)):
        head = m.group(2).strip() if m.re is IMPL else ""
        igen = generic_names(m.group(1)) if m.re is IMPL else set()
        brace = m.end() - 1
        depth, i = 0, brace
        while i < len(text):
            if text[i] == "{": depth += 1
            elif text[i] == "}":
                depth -= 1
                if depth == 0: break
            i += 1
        if m.re is TRAIT:
            out.append((brace, i, "trait:" + m.group(1), m.group(1), set()))
        else:
            head = re.sub(r"\bwhere\b.*", "", head, flags=re.S).strip()
            if " for " in head:
                trait, ty = head.split(" for ", 1)
            else:
                trait, ty = "", head
            ty = re.sub(r"<.*", "", ty.strip()).split("::")[-1].strip("&* ")
            trait = re.sub(r"<.*", "", trait.strip()).split("::")[-1]
            out.append((brace, i, ty, trait, igen))
    return out

nodes, bodies, by_name = [], {}, {}
generics_of, blanket, TRAIT_NAMES, ALIASES, impl_trait_of, crate_local, ret_of = {}, set(), set(), set(), {}, {}, {}
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
    TRAIT_NAMES.update(re.findall(r"\btrait\s+([A-Za-z_][A-Za-z0-9_]*)", text))
    ALIASES.update(re.findall(r"\btype\s+([A-Z][A-Za-z0-9_]*)\s*(?:<[^=;]*>)?\s*=", text))
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
        if REPAIR:
            # R1i: G3 took the first `;` before the first `{` as a body-less declaration, which
            # dropped every fn whose signature has an array type `[T; N]` (parameters or return).
            # Scan from the parameter list's `(` at bracket depth: the first `{` or `;` at depth 0.
            j, d, brace = m.end() - 1, 0, -1
            while j < len(text):
                ch = text[j]
                if ch in "([":
                    d += 1
                elif ch in ")]":
                    d -= 1
                elif d == 0 and ch == ";":
                    break
                elif d == 0 and ch == "{":
                    brace = j; break
                j += 1
            if brace < 0:
                continue
        else:
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
        # return type text, for R1l's one-step chain resolution
        k2 = j + 1
        mret = re.match(r"\s*->\s*", text[k2:k2 + 8])
        ret_of[key] = None
        if mret:
            q, dd = k2 + mret.end(), 0
            st = q
            while q < len(text):
                ch = text[q]
                if ch in "<([": dd += 1
                elif ch in ">)]": dd -= 1
                elif dd == 0 and (ch == "{" or text.startswith("where", q) or ch == ";"):
                    break
                q += 1
            ret_of[key] = text[st:q].strip()
        nodes.append(key)
        bodies[key] = (rel, brace, i, text)
        enclosing = [b for b in blocks if b[0] < m.start() < b[1]]
        own = min(enclosing, key=lambda b: b[1] - b[0]) if enclosing else None
        owner_of[key] = own[2] if own else None
        generics_of[key] = generic_names(m.group(2)) | (own[4] if own else set())
        impl_trait_of[key] = own[3] if own else ""
        # R1j: visibility. An inherent method or free fn without a plain `pub` is crate-local, so
        # no other crate can call it; trait-impl and trait-default methods are as visible as the trait.
        ls = text.rfind("\n", 0, m.start()) + 1
        prefix = text[ls:m.start()]
        is_pub = re.search(r"\bpub\s+(?:(?:const|unsafe|async|extern\s+\"[^\"]*\")\s+)*$", prefix) is not None
        crate_local[key] = not is_pub and not (own and (own[3] or str(own[2]).startswith("trait:")))
        if own and own[2] in own[4]:
            blanket.add(key)            # impl<T..> Trait for T: applies to every receiver
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
# G4: CG_EXTRA_DEPS="crate:dep,crate:dep" adds dependency edges a pending change introduces
# (U3 makes result_export a runtime dependency of product_physics, decision 5).
for _pair in filter(None, os.environ.get("CG_EXTRA_DEPS", "").split(",")):
    _c, _d = _pair.split(":")
    DEPS.setdefault(_c, []).append(_d)
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
audit = []                       # call tokens that produced no edge (AUDIT)
DEFINED = set(by_name) | set(methods_by)
def all_methods(name):
    return list(methods_by.get(name, []))
def blanket_methods(name):
    return [t for t in methods_by.get(name, []) if t in blanket] if REPAIR else []
def fan_type(tname, raw_ty, key):
    """R1a: True when a receiver's declared type names no concrete method owner."""
    if not REPAIR or tname is None:
        return False
    g = generics_of.get(key, set())
    if tname in g or tname in TRAIT_NAMES or tname in ALIASES:
        return True
    if raw_ty:
        for h in {"Self"} | g:
            if re.search(r"\b" + re.escape(h) + r"\s*::", raw_ty):
                return True
    return False
body_of = {}
UNWRAP = {"unwrap", "expect", "clone", "as_ref", "as_mut", "borrow", "borrow_mut", "to_owned", "unwrap_or_default"}
def ret_type_names(t, key_owner):
    """R1l: the concrete type names a declared return type denotes after `?`/unwrap; None when
    any part is generic, a trait object, an alias or unknown."""
    if not t:
        return None
    t = t.strip()
    t = re.sub(r"^&\s*('\w+\s+)?(mut\s+)?", "", t)
    if t == "Self":
        return {key_owner} if key_owner and not str(key_owner).startswith("trait:") else None
    m = re.match(r"(Result|Option)\s*<(.*)>$", t, re.S)
    if m:
        inner = m.group(2)
        d, cur = 0, ""
        for ch in inner:
            if ch in "<([": d += 1
            elif ch in ">)]": d -= 1
            if ch == "," and d == 0:
                break
            cur += ch
        return ret_type_names(cur, key_owner)
    k = base(t)
    if k[0] in ("prim", "std"):
        return {k[1]}
    if k[0] == "type" and k[1] and k[1] not in TRAIT_NAMES and k[1] not in ALIASES and k[1] != "Self" \
            and not re.search(r"\bimpl\b|\bdyn\b", t):
        return {k[1]}
    return None
def chain_types(body, dot, key, local_names=frozenset()):
    """R1l: the receiver of a chained `.g(` at `dot` is the value of the call just before it
    (skipping `?` and std unwrapping calls). Its type is the union of the declared return types of
    every candidate target of that call. Returns None (fan out) unless every candidate's return
    type resolves; the candidate set itself over-approximates, so the union is sound."""
    i = dot - 1
    while i >= 0 and body[i].isspace():
        i -= 1
    for _ in range(4):
        while i >= 0 and (body[i] == "?" or body[i].isspace()):
            i -= 1
        if i < 0 or body[i] != ")":
            return None
        d, j = 0, i
        while j >= 0:
            if body[j] == ")": d += 1
            elif body[j] == "(":
                d -= 1
                if d == 0: break
            j -= 1
        if j < 0:
            return None
        mm = re.search(r"([A-Za-z_][A-Za-z0-9_]*)\s*(?:::<[^>]*>)?\s*$", body[max(0, j - 200):j])
        if not mm:
            return None
        head = mm.group(1)
        if head in local_names:
            return None              # a closure or fn value bound locally: its type is unknown here
        hstart = j - (len(body[max(0, j - 200):j]) - mm.start(1))
        if head in UNWRAP:
            i = hstart - 1
            while i >= 0 and body[i].isspace():
                i -= 1
            if i >= 0 and body[i] == ".":
                i -= 1
                continue
            return None
        before = body[max(0, hstart - 80):hstart]
        qm = re.search(r"([A-Za-z_][A-Za-z0-9_]*)\s*::\s*$", before)
        if qm and qm.group(1)[0].isupper() and qm.group(1) not in generics_of.get(key, set()) and qm.group(1) not in TRAIT_NAMES:
            q = "Self" if False else (owner_of[key] if qm.group(1) == "Self" else qm.group(1))
            cands = [t for t in methods_by.get(head, []) if owner_of[t] == q] + [t for t in by_name.get(head, [])]
        elif re.search(r"\.\s*$", before):
            cands = methods_by.get(head, [])
        else:
            cands = by_name.get(head, []) + [t for t in methods_by.get(head, []) if qm]
        if not cands:
            return None
        out = set()
        for c in cands:
            names = ret_type_names(ret_of.get(c), owner_of.get(c))
            if names is None:
                return None
            out |= names
        return out
    return None
for key in nodes:
    rel, a, b, text = bodies[key]
    inner = [spans[o] for o in nodes if o != key and bodies[o][0] == rel and a < spans[o][0] and spans[o][1] <= b]
    body = list(text[a:b])
    for s, e in inner:
        for i in range(s - a, e - a):
            body[i] = " "
    body = "".join(body)
    body_of[key] = body
    cr = crate_key[key]
    allowed = DEPS_CLOSURE.get(cr, {cr})
    in_trait_default = str(owner_of[key]).startswith("trait:")
    # loop spans inside this body: (start, end, header)
    lspans = loopscan.loop_spans(body)
    def loops_at(pos):
        return tuple(h for (a2, b2, h) in lspans if a2 < pos < b2)
    def add_edge(t, pos):
        edges[key].add(t)
        site_loops.setdefault((key, t), {}).setdefault(pos, loops_at(pos))
    handled = {}                 # name-token position -> (number of edges added, reason)
    def emit(name, pos, targets, reason):
        if (key, name) in RULES:
            targets = [t for t in RULES[(key, name)] if t in bodies]
            reason = "rule"
        n = 0
        for t in dict.fromkeys(targets):
            if crate_key[t] in allowed and not (REPAIR and crate_local.get(t) and crate_key[t] != cr):
                add_edge(t, pos); n += 1
        prev = handled.get(pos, (0, ""))
        handled[pos] = (prev[0] + n, reason if n == 0 and prev[0] == 0 else (prev[1] or reason))
    # closures bound by `let name = |..|` / `let name = move |..|` shadow any fn of that name
    closures = set(re.findall(r"\blet\s+(?:mut\s+)?([a-z_][a-z0-9_]*)\s*(?::[^=;]*)?=\s*(?:move\s*)?\|", body))
    for m in (CALL if REPAIR else CALL_G3).finditer(body):
        qual, name = m.group(1), m.group(2)
        if name in KEYWORDS:
            continue
        if qual is None and name in closures:
            handled[m.start(2)] = (0, "closure-shadowed")
            continue
        if qual in ("Self", "self") or (qual is None and False):
            if in_trait_default and REPAIR:
                targets = all_methods(name); reason = "trait-default Self fan-out"
            else:
                targets = [t for t in methods_by.get(name, []) if owner_of[t] in (owner_of[key],) or str(owner_of[t]).startswith("trait:")] + blanket_methods(name)
                reason = "Self: no such method on the impl type"
        elif qual and REPAIR and (qual in generics_of.get(key, set()) or qual in TRAIT_NAMES or qual in ALIASES):
            targets = all_methods(name) + by_name.get(name, []); reason = "generic/trait/alias qualifier fan-out"
        elif qual and qual[0].isupper():
            targets = [t for t in methods_by.get(name, []) if owner_of[t] in (qual, "trait:" + qual)] + by_name.get(name, []) + blanket_methods(name)
            reason = "Type::f with no such fn on that type (std/external)"
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
            reason = "free call with no free fn of that name (method name only, or external)"
        emit(name, m.start(2), targets, reason)
    # R1d: `Type::<T>::f(` resolves on Type; `<X as Tr>::f(` on every impl of Tr (and Tr's
    # default); any other `<..>::f(` fans out to every same-named fn and method
    if REPAIR:
        for m in TURBO.finditer(body):
            name = m.group(1)
            if name in KEYWORDS:
                continue
            head = body[max(0, m.start() - 400):m.start() + 1]
            # the `<...>` group that ends at this `>`
            d, j = 0, len(head) - 1
            while j >= 0:
                if head[j] == ">": d += 1
                elif head[j] == "<":
                    d -= 1
                    if d == 0: break
                j -= 1
            inner = head[j + 1:-1] if j >= 0 else ""
            before = head[:j] if j >= 0 else ""
            tq = re.search(r"\bas\s+([A-Za-z_][A-Za-z0-9_:]*)\s*(?:<.*)?$", inner, re.S)
            ty = re.search(r"([A-Za-z_][A-Za-z0-9_]*)\s*::\s*$", before)
            if tq:
                tr = tq.group(1).split("::")[-1]
                targets = [t for t in methods_by.get(name, []) if impl_trait_of.get(t) == tr or owner_of[t] == "trait:" + tr]
                if tr not in TRAIT_NAMES:          # an external trait: any same-named impl method
                    targets = [t for t in methods_by.get(name, []) if impl_trait_of.get(t)]
                emit(name, m.start(1), targets, "trait-qualified path: no scanned impl")
            elif ty and ty.group(1)[0].isupper() and not (ty.group(1) in generics_of.get(key, set()) or ty.group(1) in TRAIT_NAMES or ty.group(1) in ALIASES):
                q = ty.group(1)
                emit(name, m.start(1), [t for t in methods_by.get(name, []) if owner_of[t] in (q, "trait:" + q)] + blanket_methods(name),
                     "Type::<..>::f with no such fn on that type (std/external)")
            else:
                emit(name, m.start(1), all_methods(name) + by_name.get(name, []), "turbofish/qualified-path fan-out")
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
        raw = None
        if first == "self":
            cur = ("type", owner_of[key], None) if owner_of[key] else ("unknown", None, None)
        elif first in locals_:
            raw = locals_[first]
            cur = base(raw)
        else:
            return ("unknown", None, None), None
        if idx:
            cur = base(cur[2]) if cur[0] == "std" and cur[2] else ("unknown", None, None)
        for f in toks[1:]:
            fi = f.endswith("]")
            f = re.sub(r"\[.*$", "", f)
            if cur[0] != "type" or cur[1] not in fields_of or f not in fields_of[cur[1]]:
                return ("unknown", None, None), None
            raw = fields_of[cur[1]][f]
            cur = base(raw)
            if fi:
                cur = base(cur[2]) if cur[0] == "std" and cur[2] else ("unknown", None, None)
        return cur, raw
    recv_pos = set()
    for m in re.finditer(r"(self|[A-Za-z_][A-Za-z0-9_\]\[\.]*)\s*\.\s*([A-Za-z_][A-Za-z0-9_]*)\s*(?:::<[^>]*>)?\s*\(", body):
        recv, name = m.group(1).rstrip("."), m.group(2)
        recv_pos.add(m.start(2))
        (kind, tname, _), raw = resolve(recv)
        if kind == "type" and tname and (fan_type(tname, raw, key) or (REPAIR and str(tname).startswith("trait:"))):
            targets = all_methods(name); reason = "generic/trait receiver fan-out"
        elif kind == "type" and tname:
            targets = [t for t in methods_by.get(name, []) if owner_of[t] == tname or str(owner_of[t]).startswith("trait:")] + blanket_methods(name)
            reason = f"receiver type {tname} has no such method (std/external)"
        elif kind in ("prim", "std"):
            targets = [t for t in methods_by.get(name, []) if owner_of[t] == tname or str(owner_of[t]).startswith("trait:")] + blanket_methods(name)
            reason = f"receiver {kind} {tname}: std method"
        else:
            targets = methods_by.get(name, [])
            reason = "unknown receiver: no scanned method of that name"
        emit(name, m.start(2), targets, reason)
    # R1c: chained calls `f(x).g(`, `x?.g(`, `}.g(`, `)\n    .g(` -- every `.g(` the receiver pass missed
    if REPAIR:
        for m in METHOD.finditer(body):
            name = m.group(1)
            if m.start(1) in recv_pos or name in KEYWORDS:
                continue
            tys = chain_types(body, m.start(), key, closures | bound | set(locals_))
            if tys is None:
                emit(name, m.start(1), all_methods(name), "chained call: no scanned method of that name")
            else:
                targets = [t for t in methods_by.get(name, []) if owner_of[t] in tys or str(owner_of[t]).startswith("trait:")] + blanket_methods(name)
                emit(name, m.start(1), targets, "chained call resolved by the head's return type (std/external)")
    # AUDIT: every call-shaped token to a defined name that produced no edge
    for m in re.finditer(r"(?<![A-Za-z0-9_])([A-Za-z_][A-Za-z0-9_]*)\s*(?:::<[^>]*>)?\s*\(", body):
        name = m.group(1)
        if name in KEYWORDS or name not in DEFINED:
            continue
        pos = m.start(1)
        n, reason = handled.get(pos, (0, "NOT MATCHED BY ANY PASS"))
        if n == 0:
            line = text.count("\n", 0, a + pos) + 1
            audit.append({"caller": key, "line": f"{rel}:{line}", "name": name, "reason": reason,
                          "context": " ".join(body[max(0, pos - 50):pos + len(name) + 1].split())[-90:]})

# R1f: implicit calls (CG_IMPLICIT=1). Each trigger adds edges from the function whose body
# shows the trigger to every user impl method the trigger can invoke without naming it.
implicit_added = collections.Counter()
if IMPLICIT:
    def impls(traits, names=None):
        return [k for k in nodes if impl_trait_of.get(k) in traits and (names is None or k.rsplit(":", 1)[1] in names)]
    FROM = impls({"From"}, {"from"})
    FMT = impls({"Display", "Debug"}, {"fmt"})
    DESER = impls({"Deserialize", "DeserializeSeed", "Visitor"})
    OPS = impls({"Add", "Sub", "AddAssign", "SubAssign"})
    ORD = impls({"PartialEq", "Eq", "PartialOrd", "Ord"})
    DEREF = impls({"Deref", "DerefMut"})
    DROP = impls({"Drop"})
    def self_type(k):
        return owner_of.get(k)
    TRIG = [
        ("from", re.compile(r"\?(?!\s*Sized)|\.\s*into\s*\(|\binto\s*\("), FROM),
        ("fmt", re.compile(r"\b(?:format|write|writeln|print|println|eprint|eprintln|panic|format_args|assert|assert_eq|assert_ne|debug_assert|unreachable|todo|expect)\b\s*!|\.\s*(?:to_string|expect|unwrap)\s*\("), FMT),
        ("deserialize", re.compile(r"\b(?:from_value|from_str|from_slice|from_reader|deserialize|deserialize_any|deserialize_map|deserialize_seq)\s*(?:::<[^>]*>)?\s*\("), DESER),
        ("ops", type("OpsTrigger", (), {"search": staticmethod(lambda b: "WorkTotal" in b and re.search(r"[^-+<>=!]\s[-+]=?\s", b) is not None)})(), OPS),
        ("ord", re.compile(r"BinaryHeap|\.sort|==|!=|\bmax\b|\bmin\b|\bcmp\b|<|>"), ORD),
    ]
    for key in nodes:
        body = body_of[key]
        allowed = DEPS_CLOSURE.get(crate_key[key], {crate_key[key]})
        for label, rx, targets in TRIG:
            if label == "ord" and crate_key[key] not in {crate_key[t] for t in targets}:
                continue                 # Node's ordering is private to its crate
            if rx.search(body):
                for t in targets:
                    if crate_key[t] in allowed:
                        edges[key].add(t); implicit_added[label] += 1
        for t in DEREF + DROP:
            ty = self_type(t)
            if ty and re.search(r"\b" + re.escape(ty) + r"\b", body) and crate_key[t] in allowed:
                edges[key].add(t); implicit_added["deref/drop"] += 1

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
if os.environ.get("CG_AUDIT_OUT"):
    json.dump({"repair": REPAIR, "implicit": IMPLICIT, "implicit_edges_added": dict(implicit_added),
               "blanket_methods": sorted(blanket), "trait_names": len(TRAIT_NAMES), "aliases": sorted(ALIASES),
               "unresolved_call_tokens": len(audit), "by_reason": collections.Counter(r["reason"] for r in audit).most_common(),
               "rows": audit}, open(os.environ["CG_AUDIT_OUT"], "w"), indent=1)
out = {"repair": REPAIR, "implicit": IMPLICIT, "rules_applied": len(RULES), "files": len(paths), "nodes": len(nodes), "edges": sum(len(e) for e in edges.values()),
       "cyclic_components": cyclic, "cyclic_component_edges": cyc_edges,
       "root_depths": {r: depth(comp_of[r]) for r in roots}}
json.dump(out, open(out_path, "w"), indent=1)
if os.environ.get("CG_EDGES_OUT"):
    json.dump({"edges": {k: sorted(v) for k, v in edges.items()},
               "site_loops": [[a, b, [list(x) for x in ls.values()]] for (a, b), ls in site_loops.items()],
               "spans": {k: [bodies[k][0], bodies[k][0] and bodies[k][1], bodies[k][2]] for k in nodes}},
              open(os.environ["CG_EDGES_OUT"], "w"))
print(json.dumps({k: (len(v) if k.startswith("cyclic") else v) for k, v in out.items()}, indent=1))
