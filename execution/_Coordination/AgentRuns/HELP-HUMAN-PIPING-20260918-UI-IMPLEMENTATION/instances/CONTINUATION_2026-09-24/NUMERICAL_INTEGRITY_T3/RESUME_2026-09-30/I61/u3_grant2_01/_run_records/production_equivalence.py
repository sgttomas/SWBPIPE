#!/usr/bin/env python3
"""I61 U3 grant 2: the production text of lib.rs and retained_product.rs is unchanged.

Usage: production_equivalence.py BASE_DIR CAND_DIR  (each holding lib.rs and retained_product.rs)
Every changed line must (a) keep the line count, and (b) equal the base line once the
`#[cfg(test)] <statement>;` fragments added on it are removed, unless the line lies inside
the `#[cfg(test)] pub(crate) mod retained_tests_hooks` block (test-only by construction).
"""
import re, sys
base, cand = sys.argv[1], sys.argv[2]
FRAG = [r'#\[cfg\(test\)\] retained_tests_hooks::ordinary_run_entered\(\); ',

        r'#\[cfg\(test\)\] retained_tests_hooks::at_complete_gate\(&mut observer\); ',
        r' #\[cfg\(test\)\] crate::retained_tests_hooks::before_late_gate\(&\*self\);']
ok = True
for name in ("lib.rs", "retained_product.rs"):
    b = open(f"{base}/{name}").read().split("\n"); c = open(f"{cand}/{name}").read().split("\n")
    print(f"{name}: lines base={len(b)} cand={len(c)} equal={len(b)==len(c)}"); ok &= len(b) == len(c)
    lo = hi = None
    if name == "lib.rs":
        lo = next(i for i, l in enumerate(b) if l.startswith("pub(crate) mod retained_tests_hooks {"))
        assert b[lo - 1] == "#[cfg(test)]"
        hi = next(i for i in range(lo, len(b)) if b[i] == "}")
    for i, (x, y) in enumerate(zip(b, c)):
        if x == y: continue
        if lo is not None and lo <= i <= hi:
            print(f"  {name}:{i+1} test-only module line changed"); continue
        z = y
        for f in FRAG: z = re.sub(f, "", z)
        same = z.replace("// G-C, with the observer's own permit.", "") .rstrip() == x.replace("// G-C, with the observer's own permit.", "").rstrip() if "G-C, with" in x else z == x
        if "G-C, with" in x:
            # the comment line: the hook is prepended and the comment kept verbatim
            same = z.strip() == x.strip()
        print(f"  {name}:{i+1} production line: base restored by removing the cfg(test) fragment = {same}"); ok &= same
print("PRODUCTION TEXT UNCHANGED" if ok else "MISMATCH")
