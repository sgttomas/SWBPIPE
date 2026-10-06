"""RV98: evaluate delta_inventory2.py's own test_module_file() and spans() (the tool's code, exec'd
unchanged from u4_g7_06) on the embedding file and on the corpus embedder, in RV98's extract."""
import os, re, sys
tool, W = sys.argv[1:3]
src = open(tool).read()
start = src.index("FN = re.compile"); end = src.index("def fp(")
ns = {"re": re, "os": os, "W": W}
exec(src[start:end], ns)
for rel in ["core/product_physics/src/retained_facade_tests.rs", "core/product_physics/src/retained_wire_tests.rs",
            "core/product_physics/src/retained_product_tests.rs", "core/product_physics/src/retained_product.rs"]:
    print("test_module_file", rel, "->", ns["test_module_file"](rel))
L = open(os.path.join(W, "core/product_physics/src/retained_facade_tests.rs")).read().split("\n")
fns, tests = ns["spans"](L)
print("retained_facade_tests.rs: cfg(test) item spans in the file:", tests[:5], "(count", len(tests), ")")
print("  enclosing fn of :1051/:1052:", [(a, b, n) for a, b, n in fns if a <= 1051 <= b])
L = open(os.path.join(W, "core/reporting/result_export/src/retained_precision.rs")).read().split("\n")
fns, tests = ns["spans"](L)
hits = [i + 1 for i, l in enumerate(L) if "retained_precision_cases.json" in l]
print("result_export retained_precision.rs corpus lines:", hits, "inside a cfg(test) span:", [ns["in_any"](tests, h) for h in hits],
      "spans:", [t for t in tests if any(t[0] <= h <= t[1] for h in hits)])
