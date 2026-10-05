#!/usr/bin/env python3
"""RV93: production text after removing every #[cfg(test)] statement fragment the grant added.
For each changed line (difflib over base/candidate), strip `#[cfg(test)] <stmt>;` fragments with a
small Rust-aware scanner (brackets, strings) and require the remainder to equal the base line;
lines inside `#[cfg(test)] pub(crate) mod retained_tests_hooks { ... }` are classified test-only by
brace matching from the module's opening line (not by the implementer's line list)."""
import difflib, re, sys
from pathlib import Path
base, cand = Path(sys.argv[1]), Path(sys.argv[2])
def strip_cfg_test_stmts(line):
    out, i = [], 0
    while True:
        j = line.find("#[cfg(test)]", i)
        if j < 0: out.append(line[i:]); break
        out.append(line[i:j]); k = j + len("#[cfg(test)]"); depth = 0; instr = False
        while k < len(line):
            c = line[k]
            if instr:
                if c == "\\": k += 1
                elif c == '"': instr = False
            elif c == '"': instr = True
            elif c in "([{": depth += 1
            elif c in ")]}": depth -= 1
            elif c == ";" and depth == 0: k += 1; break
            k += 1
        else:
            return None  # a cfg(test) attribute whose statement does not end on this line
        while k < len(line) and line[k] == " ": k += 1
        i = k
    return "".join(out)
def test_module_span(lines, header_regex):
    for n, l in enumerate(lines):
        if re.search(header_regex, l):
            assert lines[n - 1].strip() == "#[cfg(test)]" or "#[cfg(test)]" in l, "module is cfg(test)"
            depth = 0
            for m in range(n, len(lines)):
                depth += lines[m].count("{") - lines[m].count("}")
                if depth == 0: return (n, m)
    return None
ok = True
for f in ["lib.rs", "retained_product.rs"]:
    b = (base / f).read_text().split("\n"); c = (cand / f).read_text().split("\n")
    span = test_module_span(c, r"^pub\(crate\) mod retained_tests_hooks \{")
    print(f"{f}: lines base={len(b)} cand={len(c)}; test module span (0-based) {span}")
    sm = difflib.SequenceMatcher(a=b, b=c, autojunk=False)
    for op, i1, i2, j1, j2 in sm.get_opcodes():
        if op == "equal": continue
        if op != "replace" or (i2 - i1) != (j2 - j1):
            print(f"  {op} base {i1+1}-{i2} cand {j1+1}-{j2}: NOT line-neutral"); ok = False; continue
        for x, y in zip(range(i1, i2), range(j1, j2)):
            inside = span and span[0] <= y <= span[1]
            stripped = strip_cfg_test_stmts(c[y])
            if stripped is not None and stripped.rstrip() == b[x].rstrip():
                print(f"  {f}:{y+1}: production line; removing the cfg(test) statement(s) restores base exactly")
            elif inside:
                print(f"  {f}:{y+1}: inside the cfg(test) module")
            else:
                print(f"  {f}:{y+1}: CHANGED PRODUCTION TEXT\n    base: {b[x]}\n    cand: {c[y]}"); ok = False
print("PRODUCTION TEXT EQUAL" if ok else "PRODUCTION TEXT DIFFERS")
