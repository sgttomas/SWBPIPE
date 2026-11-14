"""The closed inventory of hex placeholders (`hex_inventory.json`, beside `text_args.g4.json`), reconciled by pricing.

Every `{:x}`-family placeholder in the priced sources (a format spec ending in `x` or `X`, with or without Debug `?`)
has one entry in `hex_inventory.json`: its identity, its size, the reason the size bounds it (for a hash site the named
SHA-256 construction; for a scalar its declared type and where that is declared), the source premises the reason relies
on, and pins of the text those premises were audited on. text_budget.py reconciles the run's own template inventory
against it before any hex size is used: a hex placeholder with no entry, an entry with no placeholder, and an entry
whose pin does not match each make the run incomplete, so it stops with no tree.

Threat model. The priced sources are our own code under ordinary maintenance. Comments, aliases, shadowing and
innocently named variables are ordinary maintenance and are inside the model, and so is a test constructed to expose a
defect. Outside the model is source arranged to match a pin while changing its meaning. Within this model the claim is:
each entry's size is a reviewed bound on identified source premises, and pricing stops when the text those premises
were audited on changes. The proof is the independent source audit recorded in each entry; the pin only detects a
change to what was audited. "Fails closed" is claimed within this model and nowhere wider.

Identity: the site key text_budget.py keys sites by (`CUR[0]`, `file:line` of the inventory row, the line its format
macro starts on), the argument key (`arg_key`: G6's `audit_key` for a format placeholder) and the format spec, exactly.
Pin: SHA-256 of the enclosing function's whole text, from its first attached outer attribute (located by a bracket
walk, multiline attributes included; the function is refused when that boundary cannot be located safely) to its
closing brace, which includes any function-local `use`; and, for each dependency an entry names outside the function,
SHA-256 of that item's text: a struct, enum or type alias (form "item"), a function signature or impl header (form
"signature"), or a use item the reason resolves a name through (form "use"). The file's other use items are not pinned:
in Rust an explicit import cannot be silently overridden (a second explicit import or definition of the same name in
the same scope does not compile, and a glob import never shadows an explicit one), so an unchanged named use item and an
unchanged function resolve the name as audited, given that the file compiles, which the pricing run's build
establishes. File-level tripwires (`shadowing`) refuse a top-level definition or alias of a name the entry resolves, a
second top-level use item naming it (which would shadow a glob), and a local module named like its path root. The pins
are recorded by `record` below and compared by `check`. Nothing here discovers dependencies: each one is named in its
entry by the audit.

Usage: python3 hex_inventory.py record <source root> <hex_inventory.json>    (re-record every pin after a re-audit)
       python3 hex_inventory.py check <source root> <hex_inventory.json> <template inventory json>
"""
import collections
import hashlib
import json
import os
import re
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import rust_literals  # noqa: E402

HEX_SPEC = re.compile(r"[xX]\??$")
# A second tripwire, not the boundary: a scalar entry whose argument names a finalize, a digest or a sha stops the run.
SUSPECT = re.compile(r"finalize|digest|sha", re.I)
MACRO = {"format": r"\bformat\s*!", "write": r"\bwrite(?:ln)?\s*!"}


def is_hex(spec):
    return bool(HEX_SPEC.search(spec or ""))


def arg_key(arg, spec):
    """G6's `audit_key` (text_budget.py) for a format placeholder: a `//` line comment dropped, whitespace removed, and
    `|?` for a Debug placeholder."""
    e = " ".join(re.sub(r"(^|\s)//[^\n]*", r"\1", arg).split())
    return re.sub(r"\s", "", e) + ("|?" if "?" in spec else "")


def ident(site, arg, spec):
    return (site, arg_key(arg, spec), spec)


def sha(text):
    return hashlib.sha256(text.encode("utf-8")).hexdigest()


class Source:
    """One source tree's files: raw text, the lexer's code text (comments and literals blanked, offsets kept), line
    starts and function items. A file that is missing or that the lexer refuses reads as None."""

    def __init__(self, root):
        self.root, self.cache = root, {}

    def get(self, f):
        if f not in self.cache:
            try:
                raw = open(os.path.join(self.root, f), encoding="utf-8").read()
                toks, comments = rust_literals.scan(raw, f)
            except Exception:
                self.cache[f] = None
            else:
                out = list(raw)
                for s, e in [(t["start"], t["end"]) for t in toks] + list(comments):
                    for j in range(s, e):
                        if out[j] != "\n":
                            out[j] = " "
                code = "".join(out)
                starts = [0] + [i + 1 for i, ch in enumerate(raw) if ch == "\n"]
                self.cache[f] = {"raw": raw, "code": code, "starts": starts, "fns": _functions(code)}
        return self.cache[f]


def _match_close(code, i):
    d = 0
    for j in range(i, len(code)):
        if code[j] == "{":
            d += 1
        elif code[j] == "}":
            d -= 1
            if d == 0:
                return j
    return None


def _terminator(code, i):
    """The first `{` or `;` at bracket depth 0 from offset i: (offset, char), or (None, None)."""
    d = 0
    for j in range(i, len(code)):
        c = code[j]
        if c in "([":
            d += 1
        elif c in ")]":
            d -= 1
        elif d == 0 and c in "{;":
            return j, c
    return None, None


def _functions(code):
    """(name, `fn` offset, body `{` offset, closing `}` offset) of every function with a body."""
    out = []
    for m in re.finditer(r"\bfn\s+([A-Za-z_]\w*)", code):
        j, c = _terminator(code, m.end())
        if c == "{":
            close = _match_close(code, j)
            if close is not None:
                out.append((m.group(1), m.start(), j, close))
    return out


def _line_of(starts, off):
    lo, hi = 0, len(starts) - 1
    while lo < hi:
        mid = (lo + hi + 1) // 2
        if starts[mid] <= off:
            lo = mid
        else:
            hi = mid - 1
    return lo + 1


QUALIFIERS = re.compile(r"\s*(?:(?:pub(?:\s*\([^)]*\))?|const|async|unsafe|extern|default)\s+)*")


def pin_start(s, fn_off):
    """Where a function's pinned text starts: the line of its first attached outer attribute, found by a bracket walk
    backwards over the masked source (comments and literals are blank there, so whitespace and comments are crossed),
    or the `fn` line when it has none. None when the boundary cannot be located safely: anything but qualifiers
    before `fn` on its line, a `]` that is not the end of a `#[..]` attribute, or a preceding token that does not end
    an item, a block or the file (`}`, `;`, `{`) or an inner attribute (`#![..]`)."""
    code, starts = s["code"], s["starts"]
    a = starts[_line_of(starts, fn_off) - 1]
    if not QUALIFIERS.fullmatch(code[a:fn_off]):
        return None
    while True:
        j = a - 1
        while j >= 0 and code[j].isspace():
            j -= 1
        if j < 0 or code[j] in "{};":
            return a
        if code[j] != "]":
            return None
        depth, k = 0, j
        while k >= 0:
            depth += {"]": 1, "[": -1}.get(code[k], 0)
            if depth == 0:
                break
            k -= 1
        h = k - 1
        while h >= 0 and code[h].isspace():
            h -= 1
        if k < 0 or h < 0:
            return None
        if code[h] == "!" and h > 0 and code[h - 1] == "#":
            return a                                    # an inner attribute belongs to the enclosing module
        if code[h] != "#":
            return None
        b = starts[_line_of(starts, h) - 1]
        if code[b:h].strip():
            return None
        a = b


def enclosing_fn(s, line, kind):
    """The innermost function whose body holds the site's format macro (the first `format!`/`write!` on the site
    line): (name, pinned text), the text from its first attached outer attribute (pin_start) to the closing brace.
    None when the macro, an enclosing function or its attribute boundary is not found."""
    starts, code = s["starts"], s["code"]
    if not 1 <= line <= len(starts):
        return None
    end = starts[line] if line < len(starts) else len(code)
    m = re.compile(MACRO.get(kind, r"\b[A-Za-z_]\w*\s*!")).search(code, starts[line - 1], end)
    if not m:
        return None
    holders = [f for f in s["fns"] if f[2] < m.start() <= f[3]]
    if not holders:
        return None
    name, fn_off, _, close = max(holders, key=lambda f: f[2])
    a = pin_start(s, fn_off)
    if a is None:
        return None
    return name, s["raw"][a:close + 1]


USE = re.compile(r"(?:pub(?:\([^)]*\))?\s+)?use\b")


def use_spans(s):
    """(start, end) of each of the file's top-level `use` items (brace depth 0), end after its `;`."""
    code, out, depth, i = s["code"], [], 0, 0
    while i < len(code):
        c = code[i]
        if c == "{":
            depth += 1
        elif c == "}":
            depth -= 1
        elif depth == 0 and (i == 0 or not (code[i - 1].isalnum() or code[i - 1] == "_")):
            m = USE.match(code, i)
            if m:
                j = code.find(";", m.end())
                if j < 0:
                    break
                out.append((i, j + 1))
                i = j
        i += 1
    return out


def use_items(s):
    """The file's top-level `use` items, each as written, joined by newlines (no longer pinned as a whole: an entry pins
    only the use items its reason names, as dependencies of form "use")."""
    return "\n".join(s["raw"][a:b] for a, b in use_spans(s))


def top_level(code):
    """The masked code with everything inside braces blanked (newlines kept): the file's top-level items only."""
    out, depth = list(code), 0
    for i, c in enumerate(code):
        if c == "}":
            depth -= 1
        if depth > 0 and c != "\n":
            out[i] = " "
        if c == "{":
            depth += 1
    return "".join(out)


def shadowing(s, names, modules, own):
    """The file-level tripwires for the names an entry resolves through its named use items: a top-level definition
    or alias of an audited name, another top-level use item naming it (which would shadow a glob import), or a local
    module named like an audited path root. `own` holds the start offsets of the entry's own named use items."""
    code, top, found = s["code"], top_level(s["code"]), []
    spans = use_spans(s)
    for n in names:
        w = re.escape(n)
        if re.search(r"\b(?:type|struct|enum|trait|union)\s+" + w + r"\b", top):
            found.append(f"a top-level definition of {n}")
        for a, b in spans:
            if re.search(r"\bas\s+" + w + r"\b", code[a:b]):
                found.append(f"an alias to {n}")
            elif a not in own and re.search(r"\b" + w + r"\b", code[a:b]):
                found.append(f"another use item naming {n}")
    for m in modules:
        if re.search(r"\bmod\s+" + re.escape(m) + r"\b", top):
            found.append(f"a local module {m}")
    return found


def item_text(s, line, form):
    """A named dependency's text from the start of its key line: form "item" runs through the item's closing `}` (or its
    `;`: a type alias, a tuple or unit struct); form "signature" runs up to a function body's or an impl's `{`,
    excluded; form "use" is a use item, through its `;`."""
    starts, code = s["starts"], s["code"]
    if not 1 <= line <= len(starts):
        return None
    a = starts[line - 1]
    if form == "use":
        m = re.compile(r"\s*" + USE.pattern).match(code, a)
        j = code.find(";", a) if m else -1
        return None if j < 0 else s["raw"][a:j + 1]
    j, c = _terminator(code, a)
    if c == ";":
        return s["raw"][a:j + 1]
    if c != "{":
        return None
    if form == "signature":
        return s["raw"][a:j]
    close = _match_close(code, j)
    return None if close is None else s["raw"][a:close + 1]


def pins(src, entry, kind):
    """The pins of one entry as they read in `src`: {"fn", "fn_sha256", "uses_sha256", "depends": [sha256 | None]}, or
    None when its function cannot be found."""
    f, ln = entry["site"].rsplit(":", 1)
    s = src.get(f)
    fn = enclosing_fn(s, int(ln), kind) if s else None
    if fn is None:
        return None
    deps = []
    for d in entry.get("depends", []):
        df, dl = d["key"].rsplit(":", 1)
        ds = src.get(df)
        t = item_text(ds, int(dl), d["form"]) if ds else None
        deps.append(None if t is None else sha(t))
    return {"fn": fn[0], "fn_sha256": sha(fn[1]), "depends": deps}


def own_use_offsets(src, entry):
    """The start offsets (at the `use` token) of the use items an entry names as dependencies, in its site's file."""
    f = entry["site"].rsplit(":", 1)[0]
    s, out = src.get(f), set()
    for d in entry.get("depends", []):
        df, dl = d["key"].rsplit(":", 1)
        if d["form"] == "use" and df == f and s and 1 <= int(dl) <= len(s["starts"]):
            a = s["starts"][int(dl) - 1]
            out.add(a + len(s["code"][a:]) - len(s["code"][a:].lstrip(" \t")))
    return out


Result = collections.namedtuple("Result", "sizes unbounded problems")


def hex_placeholders(rows):
    """{identity: (count, row kind, argument as written)} of every hex placeholder in a template inventory, at any
    multiplicity: the set the run reconciles."""
    found = {}
    for r in rows:
        for ph in r.get("placeholders", []):
            if is_hex(ph.get("spec", "")):
                k = ident(f"{r['file']}:{r['line']}", ph["arg"], ph["spec"])
                n = found.get(k, (0,))[0]
                found[k] = (n + 1, r.get("kind"), ph["arg"])
    return found


def check(path, root, rows):
    """Reconcile the inventory at `path` with the template inventory `rows` read from the sources at `root`.
    Returns Result(sizes {identity: bytes}, unbounded {site}, problems [(kind, detail)]); any problem stops the run."""
    try:
        entries = json.load(open(path, encoding="utf-8"))["entries"]
    except (OSError, ValueError, KeyError) as error:
        return Result({}, set(), [("hex-inventory", f"{path}: {error}")])
    problems, index = [], {}
    for e in entries:
        k = ident(e["site"], e["arg"], e["spec"])
        if k in index:
            problems.append(("hex-entry-duplicate", f"{k[0]} {{:{k[2]}}} {e['arg'][:80]}"))
            continue
        index[k] = e
    found = hex_placeholders(rows)
    for k, (n, kind, arg) in found.items():
        e = index.get(k)
        if e is None:
            problems.append(("hex-unregistered", f"{k[0]} {{:{k[2]}}} {arg[:80]}"))
            continue
        if n != e.get("count", 1):
            problems.append(("hex-count", f"{k[0]} {{:{k[2]}}} {arg[:80]}: {n} in the source, {e.get('count', 1)} in the entry"))
    for k, e in index.items():
        if k not in found:
            problems.append(("hex-stale-entry", f"{k[0]} {{:{k[2]}}} {e['arg'][:80]}"))
    src, sizes, unbounded = Source(root), {}, set()
    for k, e in index.items():
        if k not in found:
            continue
        bad = []
        got, want = pins(src, e, found[k][1]), e.get("pin", {})
        if got is None:
            bad.append(("hex-pin", f"{e['site']}: its enclosing function or its attribute boundary cannot be located safely"))
        else:
            if (got["fn"], got["fn_sha256"]) != (want.get("fn"), want.get("fn_sha256")):
                bad.append(("hex-pin", f"{e['site']}: the text of fn {got['fn']} is not as audited"))
            for d, h in zip(e.get("depends", []), got["depends"]):
                if h is None or h != d.get("sha256"):
                    bad.append(("hex-dependency-pin", f"{e['site']}: {d['key']} is not as audited"))
        if e.get("names") or e.get("modules"):
            s = src.get(e["site"].rsplit(":", 1)[0])
            for what in (shadowing(s, e.get("names", []), e.get("modules", []), own_use_offsets(src, e)) if s else ["its file is unreadable"]):
                bad.append(("hex-name-shadow", f"{e['site']}: {what}"))
        if SUSPECT.search(e["arg"]) and e.get("kind") != "hash":
            bad.append(("hex-tripwire", f"{e['site']}: a scalar entry's argument names a finalize, digest or sha"))
        problems += bad
        if not bad:
            sizes[k] = e["size"]
        if e.get("bounded") is False:
            unbounded.add(k[0])
    return Result(sizes, unbounded, problems)


def size(result, site, arg, spec):
    """The inventory's bytes for one hex placeholder, or None (it is then unclassified)."""
    return result.sizes.get(ident(site, arg, spec))


def positive_unbounded(result, site, mult):
    """A problem when a site the inventory records as not bounded by the hex rule has a positive multiplicity."""
    return ("hex-unbounded-positive", site) if mult and site in result.unbounded else None


def record(root, path):
    """Re-record every entry's pins from the sources at `root` (after a re-audit has confirmed its premises)."""
    data = json.load(open(path, encoding="utf-8"))
    src, missing = Source(root), []
    for e in data["entries"]:
        kind = e.get("row_kind", "format")
        got = pins(src, e, kind)
        if got is None or None in got["depends"]:
            missing.append(e["site"])
            continue
        e["pin"] = {"fn": got["fn"], "fn_sha256": got["fn_sha256"]}
        for d, h in zip(e.get("depends", []), got["depends"]):
            d["sha256"] = h
    if missing:
        raise SystemExit(f"pins not found for: {', '.join(missing)}")
    with open(path, "w", encoding="utf-8") as fh:
        json.dump(data, fh, indent=1, ensure_ascii=False)
        fh.write("\n")


if __name__ == "__main__":
    if sys.argv[1:2] == ["record"] and len(sys.argv) == 4:
        record(sys.argv[2], sys.argv[3])
    elif sys.argv[1:2] == ["check"] and len(sys.argv) == 5:
        res = check(sys.argv[3], sys.argv[2], json.load(open(sys.argv[4], encoding="utf-8"))["rows"])
        print(json.dumps({"sizes": len(res.sizes), "unbounded": sorted(res.unbounded), "problems": res.problems}, indent=1))
        sys.exit(1 if res.problems else 0)
    else:
        raise SystemExit(__doc__)
