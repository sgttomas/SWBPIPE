"""RV83 probe: call tokens in reachable functions that produce NO edge on I65's G4 graph.

Read-only, stdlib. Independent lexing (my own comment/string blanker plus blanking of
every `#[cfg(test)]`/`#[cfg(any(test..))]` item). For every function reachable from the
G4 root, every call-shaped token `name(` (bare, `.name(`, or `Path::name(`, turbofish
allowed) whose `name` is defined somewhere in the graph is checked: does the caller
have at least one edge to a definition with that name? Tokens with no such edge are
reported with their form and receiver text, so the stated limits ("the remaining
zero-edge tokens are std or external calls") can be checked by reading them.
Run from the source snapshot's projects/chirality-piping.
Usage: python3 rv83_zero_edge_calls.py <G4 _run_records> <out json>
"""
import json, re, sys, collections

g4r, out_p = sys.argv[1:3]
cg = json.load(open(g4r + "/callgraph_edges.json"))
edges, spans = cg["edges"], cg["spans"]
tb = json.load(open(g4r + "/text_budget.caps.out.json"))
reach, todo = set(tb["roots"]), list(tb["roots"])
while todo:
    v = todo.pop()
    for w in edges.get(v, []):
        if w not in reach:
            reach.add(w); todo.append(w)
by_name = collections.defaultdict(list)
for k in edges:
    by_name[k.rsplit(":", 1)[1]].append(k)

def blank(text):
    out, i, n = list(text), 0, len(text)
    def wipe(a, b):
        for j in range(a, min(b, n)):
            if out[j] != "\n":
                out[j] = " "
    while i < n:
        c = text[i]
        if text.startswith("//", i):
            j = text.find("\n", i); j = n if j < 0 else j; wipe(i, j); i = j
        elif text.startswith("/*", i):
            d, j = 1, i + 2
            while j < n and d:
                if text.startswith("/*", j): d += 1; j += 2
                elif text.startswith("*/", j): d -= 1; j += 2
                else: j += 1
            wipe(i, j); i = j
        elif c == "r" and re.match(r'r#*"', text[i:i + 8]) and (i == 0 or not (text[i-1].isalnum() or text[i-1] == "_")):
            m = re.match(r'r(#*)"', text[i:i + 8]); close = '"' + m.group(1)
            j = text.find(close, i + m.end()); j = n if j < 0 else j + len(close); wipe(i, j); i = j
        elif c == '"':
            j = i + 1
            while j < n and text[j] != '"':
                j += 2 if text[j] == "\\" else 1
            wipe(i, j + 1); i = j + 1
        elif c == "'":
            m = re.match(r"'(\\u\{[0-9a-fA-F]+\}|\\x[0-9a-fA-F]{2}|\\.|[^\\'\n])'", text[i:i + 14])
            if m:
                wipe(i, i + m.end()); i += m.end()
            else:
                i += 1
        else:
            i += 1
    return "".join(out)

def blank_cfg_test(t):
    out = list(t)
    for m in re.finditer(r"#\[cfg\((?:test|any\(test[^\]]*)\)\]", t):
        j, depth = m.end(), 0
        while j < len(t):
            ch = t[j]
            if ch in "([": depth += 1
            elif ch in ")]": depth -= 1
            elif ch == "{" and depth == 0:
                d = 0
                while j < len(t):
                    if t[j] == "{": d += 1
                    elif t[j] == "}":
                        d -= 1
                        if d == 0: break
                    j += 1
                break
            elif ch == ";" and depth == 0:
                break
            j += 1
        for x in range(m.start(), min(j + 1, len(t))):
            if out[x] != "\n": out[x] = " "
    return "".join(out)

cache = {}
KW = {"if", "while", "for", "match", "return", "fn", "loop", "Some", "Ok", "Err", "None", "Box", "Vec", "move", "as", "in", "let", "where", "impl", "mut", "ref", "else", "unsafe", "dyn"}
TOK = re.compile(r"(?<![A-Za-z0-9_])([A-Za-z_][A-Za-z0-9_]*)\s*(?:::\s*<[^()]*?>)?\s*\(")
rows = []
for k in sorted(reach):
    rel, a, b = spans[k]
    if rel not in cache:
        cache[rel] = blank_cfg_test(blank(open(rel, encoding="utf-8").read()))
    t = cache[rel]
    body = t[a:b]
    callee_names = {e.rsplit(":", 1)[1] for e in edges.get(k, [])}
    own = k.rsplit(":", 1)[1]
    for m in TOK.finditer(body):
        name = m.group(1)
        if name in KW or name not in by_name or name in callee_names:
            continue
        pre = body[:m.start()].rstrip()
        if pre.endswith("!") or body[m.end() - 1 - len(name):m.start()].endswith("fn "):
            continue
        if re.search(r"\bfn\s*$", pre):
            continue                       # a nested fn definition, not a call
        form = "method" if pre.endswith(".") else ("path" if pre.endswith("::") else "bare")
        recv = re.search(r"([A-Za-z0-9_\.\]\[\)\?:<>]{0,40})$", pre[:-1] if form != "bare" else pre)
        line = t.count("\n", 0, a + m.start()) + 1
        rows.append({"caller": k, "line": f"{rel}:{line}", "pos": a + m.start(), "name": name, "form": form,
                     "before": (recv.group(1) if recv else "")[-40:]})
by = collections.Counter((r["name"], r["form"]) for r in rows)
json.dump({"reachable": len(reach), "zero_edge_tokens": len(rows),
           "by_name_form": [[n, f, c] for (n, f), c in by.most_common()], "rows": rows},
          open(out_p, "w"), indent=1)
print(len(reach), len(rows))
for (n, f), c in by.most_common(60):
    print(c, f, n)

# Part 2: attribute each zero-edge token to its INNERMOST enclosing node (a nested fn is
# its own node) and re-check against that node's edges; keep only user-named methods.
by_file = collections.defaultdict(list)
for key, (rel, a, b) in spans.items():
    by_file[rel].append((a, b, key))
refined = []
for r in rows:
    rel = r["line"].rsplit(":", 1)[0]
    pos = r["pos"]
    inner = min((x for x in by_file[rel] if x[0] <= pos < x[1]), key=lambda x: x[1] - x[0])[2]
    names = {e.rsplit(":", 1)[1] for e in edges.get(inner, [])}
    if r["name"] in names:
        continue
    r2 = dict(r); r2["innermost"] = inner; r2["innermost_reachable"] = inner in reach
    refined.append(r2)
out = json.load(open(out_p))
out["refined_innermost"] = refined
out["refined_count"] = len(refined)
json.dump(out, open(out_p, "w"), indent=1)
print("refined (innermost node also lacks the edge):", len(refined))
