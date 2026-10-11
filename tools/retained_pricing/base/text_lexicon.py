"""I65 U4 G3 T08 (RV83 B-2): the non-macro text-producing sites of every function on the
D1 call graph (stdlib only, read-only).

The macro sites (format!/write!/writeln!/format_args!/diag) are the 1,147-row inventory.
This adds, for every function reachable from the D1 root over callgraph_edges.json:
  to_string     `.to_string()`  (a Display invocation: a new String)
  to_owned      `.to_owned()` on text
  string_from   `String::from(`
  into_text     `.into()` whose receiver is a string literal (a new String)
  join          `.join(`        (a new String; slice of Strings/strs)
  replace       `.replace(`     (a new String)
  push_str      `.push_str(`    (growth of an existing String by the pushed argument)
  clone_text    `.clone()` whose receiver's last path segment names a text field or
                variable (TEXT_NAMES below); other `.clone()` sites are data clones that
                the T05/T07/T25 families price, and are listed in the output for review
  diag_literal  a `Diagnostic {` struct literal (self_weight.rs), the one constructor
                path that bypasses `diag(`
`.into()` on a non-literal receiver is listed for review (data conversions, error
wrapping); `json!` is a Value construction priced by its family (T05/T25), not text.
Each row carries the receiver text (or literal length) for the size classification that
text_budget.py applies. Usage: python3 text_lexicon.py <P root> <edges json> <root fn> <out json>
"""
import json, os, re, sys, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import loopscan

proot, edges_p, root_fn, out_p = sys.argv[1:5]
cg = json.load(open(edges_p))
edges, spans = cg["edges"], cg["spans"]
# G4: a root given as a full node key (it contains "/src/") selects that node only
_rs = root_fn.split(",")
roots = [k for k in edges if k in _rs or k.rsplit(":", 1)[1] in [r for r in _rs if "/src/" not in r]]
reach, todo = set(roots), list(roots)
while todo:
    v = todo.pop()
    for w in edges.get(v, []):
        if w not in reach:
            reach.add(w); todo.append(w)

TEXT_NAMES = re.compile(
    r"(^|_)(id|ids|name|unit|code|message|source|label|kind|severity|reason|detail|text|location|basis|"
    r"summary|sentence|key|ref_id|ref_type|entity_ref|component|coordinate_system|sign_convention|"
    r"node|material|dof|direction|dimension|category|family|provenance|schema_version|document_kind|"
    r"status|error|cause|digest|invocation|suffix|prefix|policy|refs|affected_refs|symbol|subject|"
    r"semantic_contract_id|run_id|model_ref|description|title|path|ref|failure|location_ref|result_ref)$")
PATS = [
    ("to_string", re.compile(r"\.\s*to_string\s*\(\s*\)")),
    ("to_owned", re.compile(r"\.\s*to_owned\s*\(\s*\)")),
    ("string_from", re.compile(r"\bString::from\s*\(")),
    ("into", re.compile(r"\.\s*into\s*\(\s*\)")),
    ("join", re.compile(r"\.\s*join\s*\(")),
    ("replace", re.compile(r"\.\s*replace\s*\(")),
    ("push_str", re.compile(r"\.\s*push_str\s*\(")),
    ("clone", re.compile(r"\.\s*clone\s*\(\s*\)")),
    ("diag_literal", re.compile(r"\bDiagnostic\s*\{")),
    # G5 part 2 (carry item 3; RV83 R-N3): `.collect::<String>()` builds a new String from its pieces
    ("collect_string", re.compile(r"\.\s*collect\s*::\s*<\s*String\s*>\s*\(\s*\)")),
    # G4 (R-1 audit): `?` on Result<_, &str> into a fn returning CaptureError runs the user
    # `impl From<&str> for CaptureError` (retained_product.rs), which allocates the literal
    ("from_literal", re.compile(r"\b(?:ok_or|ok_or_else|map_err)\s*\(\s*(?:\|[^|]*\|\s*)?\"\s*\"\s*\)\s*\?")),
]

def receiver(text, dot):
    """The receiver expression of the method call whose '.' is at `dot`: walk back over
    balanced brackets; whitespace continues only inside a method chain (the next non-space
    character after it is '.'); stop at statement/operator punctuation."""
    i, d = dot - 1, 0
    while i >= 0:
        c = text[i]
        if c == '"':      # a blanked string literal: jump to its opening quote
            i = text.rfind('"', 0, i)
            if i < 0:
                break
            i -= 1
            continue
        if c in ")]}":
            d += 1
        elif c in "([{":
            if d == 0:
                break
            d -= 1
        elif d == 0:
            if c in ";=,!&|<>+*/?:" and not (c == ":" and (text[i - 1:i] == ":" or text[i + 1:i + 2] == ":")):
                break
            if c.isspace():
                j = i
                while j < dot and text[j].isspace():
                    j += 1
                if text[j:j + 1] != ".":
                    break
        i -= 1
    return i + 1, " ".join(text[i + 1:dot].split())

def call_arg(text, op):
    d, j = 0, op
    while j < len(text):
        if text[j] == "(":
            d += 1
        elif text[j] == ")":
            d -= 1
            if d == 0:
                return " ".join(text[op + 1:j].split())
        j += 1
    return ""

rows, data_clones, into_other = [], collections.Counter(), collections.Counter()
cache = {}
for key in sorted(reach):
    f, a, b = spans[key]
    if f not in cache:
        raw = open(os.path.join(proot, f), encoding="utf-8").read()
        cache[f] = (raw, loopscan.blank(raw))
    raw, bt = cache[f]
    # nested fns are separate nodes: blank their bodies out of this one
    inner = [spans[o] for o in edges if o != key and spans[o][0] == f and a < spans[o][1] and spans[o][2] <= b]
    body = list(bt[a:b])
    for s0, s1 in [(x[1], x[2]) for x in inner]:
        for i in range(s0 - a, s1 - a):
            body[i] = " "
    body = "".join(body)
    for pkind, rx in PATS:
        for m in rx.finditer(body):
            kind = pkind
            pos = a + m.start()
            line = bt.count("\n", 0, pos) + 1
            r0, rec = receiver(body, m.start()) if kind not in ("string_from", "diag_literal", "from_literal") else (m.start(), "")
            if kind == "from_literal":
                # only a `?` into an error type with an allocating From<&str>: String, CaptureError
                # (retained_product.rs) or Box<dyn Error>; a &'static str error allocates nothing
                # the fn's return type: the text after the last `->` before the body's `{` (span a)
                sig = bt[max(0, a - 800):a]
                fnpos = sig.rfind("fn ")
                sig = sig[fnpos:] if fnpos >= 0 else sig
                ret = sig.rsplit("->", 1)[1] if "->" in sig else ""
                if not re.search(r"\bString\b|CaptureError|Box\s*<\s*dyn", ret):
                    continue
                q0 = body.index('"', m.start()); q1 = body.index('"', q0 + 1)
                mm = re.fullmatch(r'"((?:\\[\s\S]|[^"\\])*)"', raw[a + q0:a + q1 + 1])
                if not mm:
                    raise SystemExit("from_literal measure failed at %s:%d" % (f, bt.count("\n", 0, a + m.start()) + 1))
                rows.append({"file": f, "line": bt.count("\n", 0, a + m.start()) + 1, "kind": kind, "fn": key,
                             "receiver": "", "literal_bytes": len(mm.group(1).encode()), "arg": ""})
                continue
            lit = None
            if re.fullmatch(r'"(\s*)"', rec):   # measure the literal from the raw text
                lit_raw = raw[a + r0:a + m.start()].strip()
                mm = re.fullmatch(r'"((?:\\.|[^"\\])*)"', lit_raw)
                lit = len(mm.group(1).encode()) if mm else None
                if lit is None:
                    raise SystemExit("literal measure failed at %s:%d: %r" % (f, bt.count("\n", 0, a + m.start()) + 1, lit_raw[:80]))
            if kind == "string_from":
                arg = call_arg(body, m.end() - 1)
                ml = re.fullmatch(r'"(\s*)"', arg)
                if ml:
                    k0 = a + m.end() - 1 + 1
                    seg = raw[k0:k0 + len(arg)]
                    mm = re.match(r'\s*"((?:\\.|[^"\\])*)"', seg)
                    lit = len(mm.group(1).encode()) if mm else len(arg) - 2
                rec = arg
            if kind == "into":
                lastn = re.sub(r"\(.*$", "", rec.split(".")[-1]).strip("&*() ")
                if lit is None and not TEXT_NAMES.search(lastn):
                    into_other[(f.split("/src/")[-1], rec[-60:])] += 1
                    continue
                kind = "into_text"
            if kind == "clone":
                last = re.sub(r"\(.*$", "", rec.split(".")[-1]).strip("&*() ")
                if not TEXT_NAMES.search(last):
                    data_clones[(f.split("/src/")[-1], last)] += 1
                    continue
                kind = "clone_text"
            if kind == "to_owned" and lit is None and not TEXT_NAMES.search(re.sub(r"\(.*$", "", rec.split(".")[-1]).strip("&*() ")):
                pass   # to_owned on a non-text-named receiver is still listed as text (conservative)
            arg = call_arg(body, m.end() - 1) if kind in ("join", "replace", "push_str") else ""
            rows.append({"file": f, "line": line, "kind": kind, "fn": key, "receiver": rec[-300:],
                         "literal_bytes": lit, "arg": arg[:300]})
out = {"root": root_fn, "reached_fns": len(reach), "rows": rows,
       "counts": dict(collections.Counter(r["kind"] for r in rows)),
       "data_clones_not_text": [[list(k), v] for k, v in data_clones.most_common()],
       "into_non_literal_not_text": [[list(k), v] for k, v in into_other.most_common()]}
json.dump(out, open(out_p, "w"), indent=1)
print(json.dumps({"reached_fns": len(reach), "counts": out["counts"],
                  "data_clone_sites": sum(data_clones.values()), "into_other_sites": sum(into_other.values())}))
