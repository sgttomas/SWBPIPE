#!/usr/bin/env python3
"""RV111 (T3-SI1c review): evaluator differential with two independent oracles.

Oracle 1 (instrumented base): the first D-producer event of each point evaluation predicts the
candidate exactly: base findings[:k] + [the producer's NonFiniteInput], value None, sources as
pushed before the producer. Unflagged lines must be byte-identical to main.
Oracle 2 (independent transcription): a Python re-implementation of the point path, in a 'base'
(IEEE carry) and a 'cand' (option D) mode, predicts value bits, ordered findings (code, subject;
the full message for every NonFiniteInput) and source ids for every generated case.

Usage: rv111_ee_compare.py <dumps dir> <report.json>
"""
import json
import math
import re
import struct
import sys
from collections import Counter, defaultdict

DUMPS, REPORT = sys.argv[1], sys.argv[2]

MAXF = sys.float_info.max
NON_FINITE_SUM = "sum or difference must be finite (it overflowed)"
NON_FINITE_PRODUCT = "product must be finite (it overflowed)"
NON_FINITE_QUOTIENT = "quotient must be finite (it overflowed)"
NON_FINITE_INTERP = "interpolated table value must be finite (a step of the interpolation overflowed)"
RATIO_MSG = "same-dimension quotient (ratio) must be finite"
NAN_ARG_MSG = ("table argument must be finite: a NaN argument is neither inside nor outside "
               "the table range")

CODES = ["UnsafeConstruct", "UnsupportedExpressionForm", "MissingVariable", "DuplicateBinding",
         "InvalidReference", "MissingRequiredValue", "NonFiniteInput", "DivisionByZero",
         "UnitMetadataMissing", "UnitMismatch", "DimensionMismatch", "TypeMismatch",
         "StatusBoundaryViolation", "UnsupportedGrammarVersion", "TableMalformed",
         "TableOutOfRange", "TableKeyNotFound"]
FINDING_START = re.compile(r"\[(" + "|".join(CODES) + r")\|")

# ------------------------------------------------------------------ parsing the Rust dumps


def parse_point(text):
    """'v=<value> st=[..] src=[..] f=<findings>' -> dict."""
    m = re.match(r"^v=(.*?) st=(\[.*?\]) src=(\[.*?\]) f=(.*)$", text)
    if not m:
        return {"raw": text}
    value, _st, src, f = m.groups()
    findings = []
    starts = [mm.start() for mm in FINDING_START.finditer(f)]
    for i, s in enumerate(starts):
        chunk = f[s + 1:(starts[i + 1] if i + 1 < len(starts) else len(f))]
        assert chunk.endswith("]"), chunk
        code, subject, message = chunk[:-1].split("|", 2)
        findings.append((code, subject, message))
    return {"value": value, "src": json.loads(src), "findings": findings}


def load_tsv(path, with_events=False):
    out = {}
    with open(path, encoding="utf-8") as fh:
        for line in fh:
            parts = line.rstrip("\n").split("\t")
            out[parts[0]] = (parts[1], parts[2] if len(parts) > 2 else "")
    return out


def first_flag(events):
    """The first producer event of the first evaluate() segment."""
    for ev in events.split(";"):
        if ev.startswith("F|"):
            _, site, subject, k, sources, mask = ev.split("|", 5)
            return {"site": site, "subject": subject, "k": int(k),
                    "sources": [s for s in sources.split(",") if s], "mask": mask}
    return None


def consumers(events):
    return sorted({ev[2:] for ev in events.split(";") if ev.startswith("C|")})


def d_finding(flag):
    site = flag["site"]
    if site == "add_subtract":
        return ("NonFiniteInput", "add_subtract", NON_FINITE_SUM)
    if site.startswith("multiply"):
        return ("NonFiniteInput", "multiply", NON_FINITE_PRODUCT)
    if site.startswith("divide"):
        return ("NonFiniteInput", "divide", NON_FINITE_QUOTIENT)
    if site == "interpolate":
        return ("NonFiniteInput", flag["subject"], NON_FINITE_INTERP)
    raise ValueError(site)


# ------------------------------------------------------------------ oracle 2: transcription

PRODUCTS = [
    ("length", "length", "area"), ("area", "length", "volume"), ("force", "length", "moment"),
    ("pressure", "area", "force"), ("stress", "area", "force"), ("mass", "acceleration", "force"),
    ("density", "volume", "mass"), ("mass_per_length", "length", "mass"),
    ("volume_per_length", "length", "volume"), ("linear_stiffness", "length", "force"),
    ("linear_stiffness", "displacement", "force"), ("rotational_stiffness", "angle", "moment"),
    ("rotational_stiffness", "rotation", "moment"), ("stress", "section_modulus", "moment"),
    ("section_modulus", "length", "second_moment_area"), ("velocity", "time", "length"),
    ("acceleration", "time", "velocity"),
    ("thermal_expansion_coefficient", "temperature_interval", "dimensionless"),
]


def product(a, b):
    for x, y, p in PRODUCTS:
        if (x == a and y == b) or (x == b and y == a):
            return p
    return None


def quotient(n, d):
    cands = []
    for x, y, p in PRODUCTS:
        if p != n:
            continue
        if y == d and x not in cands:
            cands.append(x)
        if x == d and y not in cands:
            cands.append(y)
    if not cands:
        return "unrepresentable"
    if len(cands) == 1:
        return ("unique", cands[0])
    return "ambiguous"


class Block(Exception):
    pass


STRUCTURAL_OVER_OVERFLOW = Counter()


def finite(v):
    return math.isfinite(v)


def fmin(a, b):
    if math.isnan(a):
        return b
    if math.isnan(b):
        return a
    if a < b:
        return a
    if b < a:
        return b
    if a == 0.0 and b == 0.0:
        return -0.0 if (math.copysign(1, a) < 0 or math.copysign(1, b) < 0) else 0.0
    return a


def fmax(a, b):
    if math.isnan(a):
        return b
    if math.isnan(b):
        return a
    if a > b:
        return a
    if b > a:
        return b
    if a == 0.0 and b == 0.0:
        return 0.0 if (math.copysign(1, a) > 0 or math.copysign(1, b) > 0) else -0.0
    return a


class Oracle:
    def __init__(self, bindings, mode):
        self.b = bindings
        self.mode = mode
        self.F = []
        self.src = []

    def block(self, code, subject, message=None):
        self.F.append((code, subject, message))
        raise Block

    def structural(self, code, subject, would):
        if self.mode == "cand" and not finite(would):
            STRUCTURAL_OVER_OVERFLOW[(code, subject)] += 1
        self.block(code, subject)

    def produced(self, v, subject, message):
        if self.mode == "cand" and not finite(v):
            self.block("NonFiniteInput", subject, message)
        return v

    def ev(self, n):
        k = n["node"]
        if k == "literal":
            q = n["quantity"]
            return ("q", float(q["value"]), q["dimension"], q["unit_ref"])
        if k == "variable_ref":
            vid = n["variable_id"]
            if vid not in self.b:
                self.block("MissingVariable", vid)
            bnd = self.b[vid]
            if bnd is None:
                self.block("MissingRequiredValue", vid)
            self.src.append(vid)
            return ("q",) + bnd
        if k == "unary":
            v = self.ev(n["operand"])
            op = n["operator"]
            if op == "not":
                if v[0] != "b":
                    self.block("TypeMismatch", "unary_not")
                return ("b", not v[1])
            if v[0] != "q":
                self.block("TypeMismatch", "unary_negate" if op == "negate" else "unary_abs")
            return ("q", -v[1] if op == "negate" else abs(v[1]), v[2], v[3])
        if k == "binary":
            left = self.ev(n["left"])
            right = self.ev(n["right"])
            if left[0] != "q" or right[0] != "q":
                self.block("TypeMismatch", "binary_expression")
            _, lv, ld, lu = left
            _, rv, rd, ru = right
            op = n["operator"]
            if op in ("add", "subtract"):
                would = lv + (1.0 if op == "add" else -1.0) * rv
                if ld != rd:
                    self.structural("DimensionMismatch", "add_subtract", would)
                if lu.strip() != ru.strip():
                    self.structural("UnitMismatch", "add_subtract", would)
                v = lv + (1.0 if op == "add" else -1.0) * rv
                return ("q", self.produced(v, "add_subtract", NON_FINITE_SUM), ld, lu)
            if op == "multiply":
                if ld == "dimensionless":
                    return ("q", self.produced(lv * rv, "multiply", NON_FINITE_PRODUCT), rd, ru)
                if rd == "dimensionless":
                    return ("q", self.produced(lv * rv, "multiply", NON_FINITE_PRODUCT), ld, lu)
                p = product(ld, rd)
                if p is None:
                    self.structural("UnsupportedExpressionForm", "multiply", lv * rv)
                v = self.produced(lv * rv, "multiply", NON_FINITE_PRODUCT)
                a, b = lu.strip(), ru.strip()
                unit = "ratio" if p == "dimensionless" else (f"{a}*{b}" if a <= b else f"{b}*{a}")
                return ("q", v, p, unit)
            # divide
            if rv == 0.0:
                self.structural("DivisionByZero", "divide", lv / rv if rv != 0.0 else (math.nan if lv == 0 or math.isnan(lv) else math.inf))
            if rd == "dimensionless":
                return ("q", self.produced(lv / rv, "divide", NON_FINITE_QUOTIENT), ld, lu)
            if ld == rd:
                if lu.strip() != ru.strip():
                    self.structural("UnitMismatch", "divide", lv / rv)
                ratio = lv / rv
                if not finite(ratio):
                    self.block("NonFiniteInput", "divide", RATIO_MSG)
                return ("q", ratio, "dimensionless", "ratio")
            qq = quotient(ld, rd)
            if qq in ("ambiguous", "unrepresentable"):
                self.structural("UnsupportedExpressionForm", "divide", lv / rv)
            qd = qq[1]
            v = self.produced(lv / rv, "divide", NON_FINITE_QUOTIENT)
            unit = "ratio" if qd == "dimensionless" else f"{lu.strip()}/{ru.strip()}"
            return ("q", v, qd, unit)
        if k == "compare":
            left = self.ev(n["left"])
            right = self.ev(n["right"])
            if left[0] != "q" or right[0] != "q":
                self.block("TypeMismatch", "comparison")
            if left[2] != right[2]:
                self.block("DimensionMismatch", "comparison")
            if left[3].strip() != right[3].strip():
                self.block("UnitMismatch", "comparison")
            a, b = left[1], right[1]
            return ("b", {"less_than": a < b, "less_than_or_equal": a <= b, "greater_than": a > b,
                          "greater_than_or_equal": a >= b, "equal": a == b,
                          "not_equal": a != b}[n["operator"]])
        if k == "logical":
            left = self.ev(n["left"])
            right = self.ev(n["right"])
            if left[0] != "b" or right[0] != "b":
                self.block("TypeMismatch", "logical_expression")
            return ("b", (left[1] and right[1]) if n["operator"] == "and" else (left[1] or right[1]))
        if k == "select":
            c = self.ev(n["condition"])
            t = self.ev(n["then"])
            e = self.ev(n["else"])
            if c[0] != "b":
                self.block("TypeMismatch", "select_condition")
            if t[0] == "b" and e[0] == "b":
                return t if c[1] else e
            if t[0] == "q" and e[0] == "q":
                if t[2] != e[2]:
                    self.block("DimensionMismatch", "select_branches")
                if t[3].strip() != e[3].strip():
                    self.block("UnitMismatch", "select_branches")
                return t if c[1] else e
            self.block("TypeMismatch", "select_branches")
        if k == "aggregate":
            fn = n["function"]
            qs = []
            for op in n["operands"]:
                v = self.ev(op)
                if v[0] != "q":
                    self.block("TypeMismatch", fn)
                qs.append(v)
            first = qs[0]
            sel = first[1]
            for q in qs[1:]:
                if q[2] != first[2]:
                    self.block("DimensionMismatch", fn)
                if q[3].strip() != first[3].strip():
                    self.block("UnitMismatch", fn)
                sel = fmin(sel, q[1]) if fn == "min" else fmax(sel, q[1])
            return ("q", sel, first[2], first[3])
        if k in ("interpolate", "lookup"):
            t = n["table"]
            mode = None if k == "interpolate" else n["mode"]
            rows = [(float(r["argument"]), float(r["result"])) for r in t["rows"]]
            valid = (t["table_id"].strip() != "" and len(rows) >= (2 if mode is None else 1)
                     and all(finite(a) and finite(r) for a, r in rows)
                     and all(rows[i][0] < rows[i + 1][0] for i in range(len(rows) - 1)))
            assert valid, "the generated tables are valid"
            arg = self.ev(n["argument"])
            subject = t["table_id"].strip()
            if arg[0] != "q":
                self.block("TypeMismatch", subject)
            if arg[2] != t["argument_dimension"]:
                self.block("DimensionMismatch", subject)
            if arg[3].strip() != t["argument_unit_ref"].strip():
                self.block("UnitMismatch", subject)
            x = arg[1]
            first, last = rows[0][0], rows[-1][0]
            rd, ru = t["result_dimension"], t["result_unit_ref"].strip()
            if mode == "exact":
                for a, r in rows:
                    if a == x:
                        return ("q", r, rd, ru)
                if x < first or x > last:
                    self.block("TableOutOfRange", subject)
                self.block("TableKeyNotFound", subject)
            if math.isnan(x):
                self.block("NonFiniteInput", subject, NAN_ARG_MSG)
            if x < first or x > last:
                self.block("TableOutOfRange", subject)
            if mode == "step":
                return ("q", [r for a, r in rows if a <= x][-1], rd, ru)
            for a, r in rows:
                if a == x:
                    return ("q", r, rd, ru)
            for (a0, r0), (a1, r1) in zip(rows, rows[1:]):
                if a0 < x < a1:
                    if self.mode == "base":
                        return ("q", r0 + (r1 - r0) * ((x - a0) / (a1 - a0)), rd, ru)
                    rise = r1 - r0
                    offset = x - a0
                    run = a1 - a0
                    if not (finite(rise) and finite(offset) and finite(run)):
                        self.block("NonFiniteInput", subject, NON_FINITE_INTERP)
                    fraction = offset / run
                    prod = rise * fraction
                    value = r0 + prod
                    if not (finite(fraction) and finite(prod) and finite(value)):
                        self.block("NonFiniteInput", subject, NON_FINITE_INTERP)
                    return ("q", value, rd, ru)
            raise AssertionError("no bracketing pair")
        raise AssertionError(k)


DIM_NAMES = {}


def rust_dim(token):
    # Rust Debug of Dimension: CamelCase of the token.
    if token == "TBD":
        return "Tbd"
    return "".join(p.capitalize() for p in token.split("_"))


def bits(v):
    return "0x" + struct.pack(">d", v).hex()


def oracle_render(case, mode):
    bindings = {}
    for b in case["bindings"]:
        if b.get("missing"):
            bindings[b["id"]] = None
        else:
            bindings[b["id"]] = (float.fromhex(float(b["value"]).hex()) if False else struct.unpack(">d", bytes.fromhex(b["bits"][2:]))[0], b["dim"], b["unit"])
    o = Oracle(bindings, mode)
    try:
        v = o.ev(case["expr"])
        value = v
    except Block:
        value = None
    src = sorted(set(o.src))
    if o.F:
        value = None
    return value, o.F, src


def rust_value_matches(rust_value, value):
    if value is None:
        return rust_value == "None"
    if value[0] == "b":
        return rust_value == f"B({'true' if value[1] else 'false'})"
    m = re.match(r"^Q\((0x[0-9a-f]+)\|[^|]*\|(\w+)\|(.*)\|true\|true\)$", rust_value)
    if not m:
        return False
    return m.group(1) == bits(value[1]) and m.group(2) == rust_dim(value[2]) and m.group(3) == value[3].strip()


def findings_match(rust_findings, oracle_findings):
    if len(rust_findings) != len(oracle_findings):
        return False
    for (rc, rs, rm), (oc, os_, om) in zip(rust_findings, oracle_findings):
        if rc != oc or rs != os_:
            return False
        if om is not None and rm != om:
            return False
        if oc == "NonFiniteInput" and om is None:
            return False
    return True


# ------------------------------------------------------------------ main

base = load_tsv(f"{DUMPS}/base/ee_point.tsv")
cand = load_tsv(f"{DUMPS}/cand/ee_point.tsv")
ibase = load_tsv(f"{DUMPS}/ibase/ee_point.tsv")
report = {"counts": {}, "violations": defaultdict(list), "by_family": {}, "transitions": {},
          "flag_sites": {}, "interp_masks": {}, "consumer_rows": {}, "oracle2": {}}
viol = report["violations"]


def family(label):
    return label.split("|", 1)[0]


assert set(base) == set(cand) == set(ibase), "label sets differ"
fam_counts = defaultdict(Counter)
transitions = defaultdict(Counter)
flag_sites = Counter()
masks = Counter()
consumer_rows = defaultdict(Counter)
panics = Counter()
for label in base:
    b_text, _ = base[label]
    c_text, _ = cand[label]
    i_text, events = ibase[label]
    fam = family(label)
    fam_counts[fam]["lines"] += 1
    if "PANIC" in (b_text, c_text, i_text):
        panics[(fam, b_text == "PANIC", c_text == "PANIC")] += 1
    if i_text != b_text:
        viol["ibase_differs_from_base"].append(label)
    flag = first_flag(events)
    if flag is None:
        if c_text == b_text:
            fam_counts[fam]["unflagged_identical"] += 1
        else:
            viol["unflagged_differs"].append(label)
        continue
    fam_counts[fam]["flagged"] += 1
    flag_sites[flag["site"]] += 1
    if flag["site"] == "interpolate":
        masks[flag["mask"]] += 1
    bp, cp = parse_point(b_text), parse_point(c_text)
    expected = bp["findings"][: flag["k"]] + [d_finding(flag)]
    ok = (cp.get("value") == "None" and cp.get("findings") == expected
          and cp.get("src") == flag["sources"])
    if not ok:
        viol["flagged_not_as_predicted"].append({"label": label, "expected": expected, "cand": c_text,
                                                 "flag": flag})
        continue
    fam_counts[fam]["flagged_blocked_as_predicted"] += 1
    if cp["findings"][-1][0] != "NonFiniteInput":
        viol["flagged_last_not_nonfinite"].append(label)
    # What main did with it.
    bv = bp.get("value", "")
    if bp.get("findings"):
        outcome = "blocked_on_main"
    elif bv.startswith("B("):
        outcome = "decided_" + bv
    elif bv.startswith("Q("):
        fv = struct.unpack(">d", bytes.fromhex(bv[4:20]))[0]
        outcome = "finite_quantity_on_main" if math.isfinite(fv) else "non_finite_quantity_on_main"
    else:
        outcome = "other"
    transitions[fam][outcome] += 1
    if fam == "pc":
        consumer = label.split("|")[3]
        consumer_rows[consumer][outcome] += 1
    for c in consumers(events):
        consumer_rows["event:" + c][outcome] += 1

report["counts"]["labels"] = len(base)
report["counts"]["panics"] = {str(k): v for k, v in panics.items()}
report["by_family"] = {k: dict(v) for k, v in fam_counts.items()}
report["transitions"] = {k: dict(v) for k, v in transitions.items()}
report["flag_sites"] = dict(flag_sites)
report["interp_masks"] = dict(masks)
report["consumer_rows"] = {k: dict(v) for k, v in sorted(consumer_rows.items())}

# Interval: byte-identical on every line.
bi = load_tsv(f"{DUMPS}/base/ee_interval.tsv")
ci = load_tsv(f"{DUMPS}/cand/ee_interval.tsv")
ii = load_tsv(f"{DUMPS}/ibase/ee_interval.tsv")
assert set(bi) == set(ci) == set(ii)
report["counts"]["interval_lines"] = len(bi)
report["counts"]["interval_identical"] = sum(1 for k in bi if bi[k][0] == ci[k][0])
report["counts"]["interval_ibase_identical"] = sum(1 for k in bi if bi[k][0] == ii[k][0])
report["counts"]["interval_panics"] = sum(1 for k in ci if "PANIC" in (bi[k][0], ci[k][0]))
for k in bi:
    if bi[k][0] != ci[k][0]:
        viol["interval_differs"].append(k)

# Oracle 2: the independent transcription, both modes, every generated case.
o2 = Counter()
o2_mismatch = defaultdict(list)
cases_files = [open(f"{DUMPS}/{t}/ee_cases.jsonl", encoding="utf-8").read() for t in ("base", "cand", "ibase")]
assert cases_files[0] == cases_files[1] == cases_files[2], "case files differ between trees"
for line in cases_files[0].splitlines():
    case = json.loads(line)
    if not case["py"]:
        continue
    label = case["label"]
    o2["cases"] += 1
    flag = first_flag(ibase[label][1])
    cand_F = None
    for mode, dump in (("base", base), ("cand", cand)):
        value, F, src = oracle_render(case, mode)
        if mode == "cand":
            cand_F = F
        rp = parse_point(dump[label][0])
        ok = (rust_value_matches(rp.get("value"), value) and findings_match(rp.get("findings", []), F)
              and rp.get("src") == src)
        if ok:
            o2[f"{mode}_agree"] += 1
        else:
            o2[f"{mode}_disagree"] += 1
            if len(o2_mismatch[mode]) < 40:
                o2_mismatch[mode].append({"label": label, "oracle": [repr(value), F, src], "rust": dump[label][0]})
    # Classifier agreement: the transcription's cand-mode D block vs the instrumented flag.
    F = cand_F
    py_flag = bool(F) and F[-1][0] == "NonFiniteInput" and F[-1][2] in (
        NON_FINITE_SUM, NON_FINITE_PRODUCT, NON_FINITE_QUOTIENT, NON_FINITE_INTERP)
    if py_flag == (flag is not None):
        o2["classifier_agree"] += 1
    else:
        o2["classifier_disagree"] += 1
        o2_mismatch["classifier"].append(label)
report["oracle2"] = {"counts": dict(o2), "mismatch_samples": dict(o2_mismatch),
                     "structural_block_where_the_operation_would_overflow": {f"{k[0]}|{k[1]}": v for k, v in STRUCTURAL_OVER_OVERFLOW.items()}}
report["violations"] = {k: (v if len(v) < 60 else v[:60] + [f"... {len(v)} total"]) for k, v in viol.items()}
report["violation_counts"] = {k: len(v) for k, v in viol.items()}
json.dump(report, open(REPORT, "w"), indent=1, sort_keys=True)
print(json.dumps({"violation_counts": report["violation_counts"], "by_family": report["by_family"],
                  "oracle2": dict(o2), "interval": {k: v for k, v in report["counts"].items()}}, indent=1))

# ------------------------------------------------------------------ oracle 3: exact rationals at the boundary
from fractions import Fraction

TWO970 = 2.0 ** 970
BASE_VALS = [0.0, 5e-324, sys.float_info.min, 1e-308, 1e-300, 0.5, 1.0, 2.0, 1e154, 1.4e154, 1e300, 1e308,
             MAXF / 2.0, TWO970, math.nextafter(TWO970, 0.0), math.nextafter(TWO970, math.inf),
             math.nextafter(MAXF, 0.0), MAXF]
VALS = []
for v in BASE_VALS:
    VALS += [v, -v]
OVERFLOW_AT = Fraction(2) ** 1024 - Fraction(2) ** 970  # round-to-nearest-even overflow threshold
ARMS = {"add": ("+", NON_FINITE_SUM, "add_subtract"), "sub": ("-", NON_FINITE_SUM, "add_subtract"),
        "muldl": ("*", NON_FINITE_PRODUCT, "multiply"), "muldr": ("*", NON_FINITE_PRODUCT, "multiply"),
        "mulderived": ("*", NON_FINITE_PRODUCT, "multiply"), "divdd": ("/", NON_FINITE_QUOTIENT, "divide"),
        "divrr": ("/", NON_FINITE_QUOTIENT, "divide"), "divderived": ("/", NON_FINITE_QUOTIENT, "divide"),
        "divratio": ("/", RATIO_MSG, "divide")}
o3 = Counter()
o3_bad = []
for label, (text, _) in cand.items():
    if not label.startswith("bd|"):
        continue
    _, arm, i, j = label.split("|")
    a, b = VALS[int(i)], VALS[int(j)]
    op, msg, subject = ARMS[arm]
    rp = parse_point(text)
    if op == "/" and b == 0.0:
        expect = ("block", ("DivisionByZero", "divide"))
    else:
        fa, fb = Fraction(a), Fraction(b)
        exact = {"+": fa + fb, "-": fa - fb, "*": fa * fb, "/": (fa / fb) if b != 0 else None}[op]
        if abs(exact) >= OVERFLOW_AT:
            expect = ("block", ("NonFiniteInput", subject, msg))
        else:
            r = float(exact)  # correctly rounded, ties to even
            if exact == 0:
                # IEEE sign of an exact zero result: x - x and x + (-x) give +0; products/quotients carry the sign.
                if op in "+-":
                    s = -0.0 if (math.copysign(1, a) < 0 and math.copysign(1, b if op == "+" else -b) < 0) else 0.0
                else:
                    s = math.copysign(0.0, math.copysign(1, a) * math.copysign(1, b))
                r = s
            expect = ("value", bits(r))
    if expect[0] == "block":
        got = rp.get("findings", [])
        ok = rp.get("value") == "None" and len(got) == 1 and tuple(got[0][:2]) == tuple(expect[1][:2]) and (
            len(expect[1]) < 3 or got[0][2] == expect[1][2])
    else:
        ok = not rp.get("findings") and rp.get("value", "").startswith(f"Q({expect[1]}|")
    if ok:
        o3["agree"] += 1
        o3["blocked" if expect[0] == "block" else "finite"] += 1
        if expect[0] == "value" and abs(Fraction(a) + 0) >= 0 and expect[1] in (bits(MAXF), bits(-MAXF)):
            o3["finite_at_plus_minus_MAX"] += 1
    else:
        o3["disagree"] += 1
        o3_bad.append({"label": label, "a": a, "b": b, "expect": expect, "cand": text})
report["oracle3_exact_boundary"] = {"counts": dict(o3), "disagreements": o3_bad[:40]}
json.dump(report, open(REPORT, "w"), indent=1, sort_keys=True)
print(json.dumps({"oracle3": dict(o3)}, indent=1))
