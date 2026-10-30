"""Shared loop-span scanner for callgraph.py and text_budget.py (stdlib only).

A "loop" is either a brace loop (`for .. in ..`, `while ..`, `loop`) or the closure
argument of an iterator adapter (`.map(|..| ..)`, `.filter_map`, `.flat_map`,
`.for_each`, `.filter`, `.any`, `.all`, `.fold`, `.try_fold`, `.find`, `.find_map`,
`.position`, `.take_while`, `.skip_while`, `.scan`, `.inspect`, `.map_while`,
`.try_for_each`, `.max_by`, `.min_by`, `.max_by_key`, `.min_by_key`, `.partition`,
`.sort_by`, `.sort_by_key`, `.sort_unstable_by`, `.retain`, `.dedup_by`).
The closure header is `iter:<receiver text>`; the receiver may be an Option or a
Result (closure runs at most once), which the bound table classifies.
Input text must already have comments and string contents blanked.
"""
import re

BRACE = re.compile(r"(?<![A-Za-z0-9_])(for\s+[^{]+?\s+in\s+[^{]+|while\s+[^{]+|loop)\s*\{")
ADAPT = re.compile(r"\.\s*(map|filter_map|flat_map|for_each|filter|any|all|fold|try_fold|find|find_map|position|"
                   r"take_while|skip_while|scan|inspect|map_while|try_for_each|max_by|min_by|max_by_key|min_by_key|"
                   r"partition|sort_by|sort_by_key|sort_unstable_by|sort_unstable_by_key|retain|dedup_by)\s*\(")

def _match(text, i, op, cl):
    d = 0
    while i < len(text):
        if text[i] == op:
            d += 1
        elif text[i] == cl:
            d -= 1
            if d == 0:
                return i
        i += 1
    return len(text)

def _receiver(text, dot):
    """Text of the receiver expression ending at `dot` (balanced, back to statement start)."""
    i, d = dot - 1, 0
    while i >= 0:
        c = text[i]
        if c in ")]}":
            d += 1
        elif c in "([{":
            if d == 0:
                break
            d -= 1
        elif d == 0 and c in ";=," :
            break
        i -= 1
    return " ".join(text[i + 1:dot].split())

def loop_spans(text):
    spans = []
    for m in BRACE.finditer(text):
        st = m.end() - 1
        spans.append((st, _match(text, st, "{", "}"), " ".join(m.group(1).split())[:4000]))
    for m in ADAPT.finditer(text):
        op = m.end() - 1
        rest = text[op + 1: op + 40].lstrip()
        if not (rest.startswith("|") or rest.startswith("move")):
            # G4 (R-1 audit): a function passed by value (`.map(bits)`) is called once per
            # element too; G3 skipped the span, so the value edge counted once per enclosing
            # iteration. CG_LEXER=regex restores G3's behaviour for the baseline.
            import os as _os
            if _os.environ.get("CG_LEXER", "rust") == "regex":
                continue
            arg = re.match(r"\s*([A-Za-z_][A-Za-z0-9_:]*)\s*\)", text[op + 1: op + 200])
            if not arg:
                continue
            spans.append((op, _match(text, op, "(", ")"), "iter:" + _receiver(text, m.start())[-400:] + "." + m.group(1) + "(" + arg.group(1) + ")"))
            continue
        spans.append((op, _match(text, op, "(", ")"), "iter:" + _receiver(text, m.start())[-400:] + "." + m.group(1)))
    return spans

def blank(text):
    """G3 regex blanker; G4 routes to blank_rust unless CG_LEXER=regex (baseline reproduction)."""
    import os as _os
    if _os.environ.get("CG_LEXER", "rust") != "regex":
        return blank_rust(text)
    text = re.sub(r"//[^\n]*", lambda m: " " * len(m.group(0)), text)
    text = re.sub(r"/\*.*?\*/", lambda m: " " * len(m.group(0)), text, flags=re.S)
    text = re.sub(r'"(?:\\.|[^"\\])*"', lambda m: '"' + " " * (len(m.group(0)) - 2) + '"', text)
    return text


def blank_rust(text):
    """G4 (R-1 audit): a single-pass Rust lexer replacing the regex blanker above.

    The G3 regexes mis-lexed three forms: a string with a `\\`-newline continuation (`.` does
    not match a newline, so the match failed and string/code regions inverted), a `//` inside a
    string (removed as a comment together with the closing quote), and a `'"'` char literal.
    This lexer handles line and nested block comments, normal and byte strings with any escape
    (including an escaped newline), raw strings `r#".."#` with any number of `#`, char and byte
    literals versus lifetimes. Comment text and string contents become spaces; every newline is
    kept, so line numbers are those of the source; string quotes are kept, so a blanked string
    still reads as `"   "`."""
    out = list(text)
    n, i = len(text), 0
    def blank_range(a, b, keep_quotes=False):
        for j in range(a, b):
            if out[j] != "\n" and not (keep_quotes and (j == a or j == b - 1)):
                out[j] = " "
    def ident_char(c):
        return c.isalnum() or c == "_"
    while i < n:
        c = text[i]
        if c == "/" and i + 1 < n and text[i + 1] == "/":
            j = text.find("\n", i)
            j = n if j < 0 else j
            blank_range(i, j); i = j; continue
        if c == "/" and i + 1 < n and text[i + 1] == "*":
            depth, j = 1, i + 2
            while j < n and depth:
                if text.startswith("/*", j): depth += 1; j += 2
                elif text.startswith("*/", j): depth -= 1; j += 2
                else: j += 1
            blank_range(i, j); i = j; continue
        # raw strings r"..", r#".."#, br"..": the prefix must start a token
        if c in "rb" and (i == 0 or not ident_char(text[i - 1])):
            m = re.match(r'(?:br|r)(#*)"', text[i:i + 300])
            if m:
                hashes = m.group(1)
                start = i + m.end() - 1           # the opening quote
                end = text.find('"' + hashes, start + 1)
                end = n if end < 0 else end + 1 + len(hashes)
                blank_range(i, end); out[start] = '"'; out[end - 1 - len(hashes)] = '"'
                i = end; continue
            if c == "b" and i + 1 < n and text[i + 1] in "\"'":
                i += 1; c = text[i]               # byte string / byte char: lex the quote below
        if c == '"':
            j = i + 1
            while j < n and text[j] != '"':
                j += 2 if text[j] == "\\" else 1
            j = min(j + 1, n)
            blank_range(i, j, keep_quotes=True); i = j; continue
        if c == "'":
            if i + 1 < n and text[i + 1] == "\\":
                j = text.find("'", i + 3)      # '\n', '\'', '\u{..}'
                j = n if j < 0 else j + 1
                blank_range(i, j, keep_quotes=True); i = j; continue
            if i + 2 < n and text[i + 2] == "'":
                blank_range(i, i + 3, keep_quotes=True); i += 3; continue
            i += 1; continue                   # a lifetime or label
        i += 1
    return "".join(out)


TEST_ATTR = re.compile(r"#\[cfg\((?:test|any\(\s*test\b[^\]]*)\)\]")

# W1R-FG (retained-pricing-rules.md §8.11): opt-in features the registered build leaves off. The registered
# identity is built with no features, so an item or statement under exactly `#[cfg(feature = "<f>")]` or
# `#[cfg(all(test, feature = "<f>"))]` for such a feature is compiled out of it, as a `#[cfg(test)]` item is,
# and is blanked the same way. A `#[cfg(not(feature = ..))]` arm is compiled in and stays priced. The
# feature's names are read from the raw text (the lexer blanks string contents); `off_build_files` and
# `off_feature_audit` below hold the manifest and inclusion guards.
# W1 coverage repair corner 1 (§8.12): `loaded-maximum-hooks` exposes the kernel's test capacity hook for the
# genuine-exhaustion control only; like `census-probe`, no product build or `[dev-dependencies]` entry enables it.
OFF_FEATURES = ("census-probe", "loaded-maximum-hooks")
OFF_ATTR = re.compile(r'#\[cfg\((?:all\(\s*test\s*,\s*)?feature\s*=\s*"(?:' + "|".join(re.escape(f) for f in OFF_FEATURES)
                      + r')"\s*\)?\)\]')

def blank_test_items(bt, raw=None):
    """G4: blank every item gated by `#[cfg(test)]` or `#[cfg(any(test, ..))]` (the latter is
    compiled only with test or the mutation-controls feature, which D1 builds leave off),
    wherever it sits in the file. G3 cut each file at its first `#[cfg(test)] mod {`, which
    drops every production item after a mid-file test module (U3 adds one to PP lib.rs at
    :3002). Input is lexer-blanked text; newlines are kept, so line numbers are unchanged.
    With the raw text, W1R-FG's off-feature items are blanked too (an attribute the lexer
    blanked, inside a comment or string, is not one)."""
    out = list(bt)
    n = len(bt)
    attrs = [(m.start(), m.end(), False) for m in TEST_ATTR.finditer(bt)]
    if raw is not None:
        if len(raw) != n:
            raise ValueError("W1R-FG: the lexer-blanked text is not position-aligned with the raw text")
        attrs += [(m.start(), m.end(), True) for m in OFF_ATTR.finditer(raw) if bt.startswith("#[cfg(", m.start())]
    for start, end, off in sorted(set(attrs)):
        i = end
        # skip further attributes and whitespace
        while True:
            while i < n and bt[i].isspace():
                i += 1
            if bt.startswith("#[", i):
                d = 0
                while i < n:
                    if bt[i] == "[": d += 1
                    elif bt[i] == "]":
                        d -= 1
                        if d == 0:
                            i += 1; break
                    i += 1
                continue
            break
        # the item: up to the first `;` or `{` at bracket depth 0; a `{` extends to its match
        j, d = i, 0
        while j < n:
            c = bt[j]
            # W1R-FG (review): an off-feature attribute must gate a statement or an item. One on
            # a match arm, a field or a list element would end at a depth-0 `,`, `=>` or an
            # unmatched `}`, and blanking on to the next `;` could hide production code.
            if off and d == 0 and (c in ",})]" or bt.startswith("=>", j)):
                raise ValueError(f"W1R-FG: an off-feature attribute at offset {start} gates an expression position; re-review")
            if c in "([": d += 1
            elif c in ")]": d -= 1
            elif d == 0 and c == ";":
                j += 1; break
            elif d == 0 and c == "{":
                k, e = j, 0
                while k < n:
                    if bt[k] == "{": e += 1
                    elif bt[k] == "}":
                        e -= 1
                        if e == 0: break
                    k += 1
                j = k + 1
                break
            j += 1
        for q in range(start, min(j, n)):
            if out[q] != "\n":
                out[q] = " "
    return "".join(out)


# W1R-FG: the files the registered build does not compile. The W1 coverage census probe and its driver are included
# only through `#[path]` module declarations that are themselves compiled out: gated by an off feature or test
# attribute, or written inside a test file (`*_tests.rs`, `tests.rs`, `/tests/`), which the chain already excludes.
# Each reviewed file names the files that may declare it; any other inclusion, a plain `mod <stem>;`, or a new
# off-build file stops the run for review.
OFF_BUILD_FILES = {
    "core/solver/frame_kernel/src/structural/retained/product_certificate/census_probe.rs":
        ("core/solver/frame_kernel/src/structural/retained/product_certificate/final_case.rs",),
    "core/product_physics/src/w1_census_driver.rs":
        ("core/product_physics/src/w1_coverage_census_tests.rs", "core/product_physics/src/w1e_census_tests.rs"),
}
# The manifest lines that may name an off feature: for each, the kernel's declaration and product_physics' forwarding.
OFF_FEATURE_MANIFEST = {
    "core/solver/frame_kernel/Cargo.toml": ('census-probe = []', 'loaded-maximum-hooks = []'),
    "core/product_physics/Cargo.toml": ('census-probe = ["open_pipe_stress_frame_kernel/census-probe"]',
                                        'loaded-maximum-hooks = ["open_pipe_stress_frame_kernel/loaded-maximum-hooks"]'),
}
_PATH_MOD = re.compile(r'((?:#\[[^\]]*\]\s*)*)#\[path\s*=\s*"([^"]+)"\]\s*((?:#\[[^\]]*\]\s*)*)(?:pub(?:\([^)]*\))?\s+)?mod\s+(\w+)\s*;')

def _test_file(rel):
    base = rel.rsplit("/", 1)[-1]
    return base.endswith("_tests.rs") or base == "tests.rs" or "/tests/" in "/" + rel

def _rs_files(root):
    import os
    out = {}
    core = os.path.join(root, "core")
    for d, _, fs in os.walk(core):
        if "/target" in d:
            continue
        for f in sorted(fs):
            if f.endswith(".rs"):
                p = os.path.join(d, f)
                out[os.path.relpath(p, root)] = open(p, encoding="utf-8").read()
    return out

def off_build_files(root):
    """The reviewed off-build files present at the snapshot, after the inclusion guard (fails closed).

    A snapshot without them (every revision before the W1 coverage repair) returns the empty set."""
    import os
    srcs = _rs_files(root)
    present = {f for f in OFF_BUILD_FILES if f in srcs}
    if not present:
        return set()
    declared = {f: [] for f in OFF_BUILD_FILES}
    for rel, raw in srcs.items():
        bt = blank_rust(raw)
        for m in _PATH_MOD.finditer(raw):
            if not bt.startswith("#[", m.start()):
                continue                                    # inside a comment or string
            target = os.path.normpath(os.path.join(os.path.dirname(rel), m.group(2)))
            stem = os.path.basename(target)
            if target in declared or stem in {os.path.basename(f) for f in OFF_BUILD_FILES}:
                attrs = m.group(1) + m.group(3)
                gated = bool(OFF_ATTR.search(attrs) or TEST_ATTR.search(attrs))
                declared.setdefault(target, []).append((rel, gated or _test_file(rel)))
        # `include!` compiles a file's text in place, outside any module declaration: an
        # `include!` naming an off-build file (gated or not) is never reviewed, so it stops.
        # `include_str!`/`include_bytes!` read text only and compile no code.
        for m in re.finditer(r"\binclude\s*!\s*[(\[{]", bt):  # any macro delimiter
            depth, end = 1, m.end()
            while end < len(bt) and depth:                  # balanced over code; string text is blanked
                depth += {"(": 1, "[": 1, "{": 1, ")": -1, "]": -1, "}": -1}.get(bt[end], 0)
                end += 1
            arg = raw[m.end():end]
            if any(os.path.basename(f) in arg for f in OFF_BUILD_FILES):
                raise ValueError(f"W1R-FG: {rel} includes an off-build file with include!; re-review")
        for f in present:
            stem = os.path.basename(f)[:-3]
            for p in re.finditer(r"\bmod\s+" + re.escape(stem) + r"\s*;", bt):
                if not re.search(r'#\[path\s*=\s*"\s*"\][^;{}]*$', bt[:p.start()][-300:]):
                    raise ValueError(f"W1R-FG: {f} is also a plain module in {rel}; re-review")
    for f in present:
        sites = declared.get(f, [])
        if not sites or not all(ok for _, ok in sites) or sorted({r for r, _ in sites}) != sorted(OFF_BUILD_FILES[f]):
            raise ValueError(f"W1R-FG: {f}'s inclusion is not only through compiled-out declarations in {OFF_BUILD_FILES[f]}: {sites}")
    stray = {t: s for t, s in declared.items() if t not in OFF_BUILD_FILES and s}
    if stray:
        raise ValueError(f"W1R-FG: a module path names an off-build file outside the reviewed set: {stray}")
    off_feature_audit(root)
    return present

def off_feature_audit(root):
    """Every manifest line naming an off feature is one of the reviewed lines (no default, no enabling dependency)."""
    import os
    found = {}
    for d, _, fs in os.walk(root):
        if "/target" in d:
            continue
        for f in fs:
            if f == "Cargo.toml":
                p = os.path.join(d, f)
                for line in open(p, encoding="utf-8").read().splitlines():
                    code = line.split("#", 1)[0].strip()
                    if any(feature in code for feature in OFF_FEATURES):
                        found.setdefault(os.path.relpath(p, root), []).append(code)
    if {k: tuple(v) for k, v in found.items()} != OFF_FEATURE_MANIFEST:
        raise ValueError(f"W1R-FG: the off features' manifest lines changed; re-review: {found}")
    return found
