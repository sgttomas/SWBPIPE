"""RV84: TAV hidden by the dropped colon-adjacent calls in source_receipt/source.rs (T25 S4).
Two figures: (a) the packet's own site byte classes from text_budget.caps.out.json;
(b) RV84's source spelling (path <= 87 + 2*21 bytes; functional id <= 156; bits 16).
Counts: N^2 aggregate bits, 3*144*m frame matrix bits, atoms <= 16,384 (descriptors_charge,
FKS exact_boundary/functionals.rs:409-476), recipes <= R rows (P_final), Fn functions.
Usage: python3 rv84_colon_quantify.py <I65 _run_records>"""
import json, sys
I65 = sys.argv[1]
tb = json.load(open(I65 + "/text_budget.caps.out.json"))
def b(line, file="source_receipt/source.rs"):
    return [r["bytes"] for r in tb["rows"] if r["file"].endswith(file) and r["line"] == line]
def cap(x): return max(8, 2 * x)
n, m = 32, 32; N = 6 * n; U = 16_384; Fn = 42 * m + N + 32 + 6 * 32; R = 7 * n + 51 * m + 8 * 32 + 3
bits_calls = N * N + 3 * 144 * m + U + U          # matrix, matrix12, products(lowered), source_operand
pkt = {"lowered_products path format (source.rs:51)": U * cap(b(51)[0]),
       "bits() from dropped callers": bits_calls * cap(16),
       "recipes formats (source.rs:175-181)": R * sum(cap(x) for x in [55, 66, 55, 66, 65, 67]),
       "function -> functional_id (source.rs:150)": Fn * cap(b(27)[0]),
       "function -> bits x2 (source.rs:150)": Fn * 2 * cap(16)}
src = dict(pkt)
src["lowered_products path format (source.rs:51)"] = U * cap(87 + 2 * 21)
src["function -> functional_id (source.rs:150)"] = Fn * cap(156)
print(json.dumps({"packet_byte_classes": pkt, "packet_total": sum(pkt.values()),
                  "source_spelling": src, "source_total": sum(src.values())}, indent=1))
