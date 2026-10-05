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
            continue           # function passed by value: handled as a call edge
        spans.append((op, _match(text, op, "(", ")"), "iter:" + _receiver(text, m.start())[-400:] + "." + m.group(1)))
    return spans

def blank(text):
    text = re.sub(r"//[^\n]*", lambda m: " " * len(m.group(0)), text)
    text = re.sub(r"/\*.*?\*/", lambda m: " " * len(m.group(0)), text, flags=re.S)
    text = re.sub(r'"(?:\\.|[^"\\])*"', lambda m: '"' + " " * (len(m.group(0)) - 2) + '"', text)
    return text
