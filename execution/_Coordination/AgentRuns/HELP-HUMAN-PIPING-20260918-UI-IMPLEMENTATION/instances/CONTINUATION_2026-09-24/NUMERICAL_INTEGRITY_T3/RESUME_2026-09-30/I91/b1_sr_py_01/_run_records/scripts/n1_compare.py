"""Compare n1_diff.py outputs (base, head) and classify every difference.

Usage: n1_compare.py <base.json> <head.json> <out.txt>
A difference is explained when it is one of:
  N1    the base raised TypeError (an unhashable enum) and the head gives the base header code, either
        as a ValueError from `_source_contract`/the packager or as the reader's G7 header code;
  N2    a removed `solve_quality`: the base's G5 fallback (PRODUCT_ATTEMPT) becomes G5 ATTEMPT;
Anything else is reported as UNEXPLAINED.
"""
import collections, json, sys

base, head = json.load(open(sys.argv[1]))["rows"], json.load(open(sys.argv[2]))["rows"]
assert base.keys() == head.keys(), (len(base), len(head), sorted(set(base) ^ set(head))[:5])
HEADER = ("SOURCE_NUMERICAL_QUALITY_INVALID", "SOURCE_NUMERICAL_CASE_INVALID")
classes = collections.Counter()
lines = []
by_section = collections.Counter(k.split("|")[0] for k in base)
for k in sorted(base):
    b, h = base[k], head[k]
    if b == h:
        continue
    section = k.split("|")[0]
    cls = "UNEXPLAINED"
    if b.startswith("TypeError:unhashable") and h.startswith("ValueError:") and h.split(":", 1)[1] in HEADER:
        cls = "N1 (TypeError -> ValueError header code)"
    elif b.startswith("RetainedPrecisionError:G7/SOURCE_PREVIEW_PHYSICS_INVALID/None") and h.startswith("RetainedPrecisionError:G7/") and h.split("/")[1] in HEADER:
        cls = "N1 (reader G7 fallback -> G7 header code)"
    elif b.startswith("ValueError:SOURCE_PREVIEW_PHYSICS_INVALID") and h.startswith("ValueError:") and h.split(":", 1)[1] in HEADER:
        cls = "N1 (dispatch: reader G7 fallback -> header code)"
    elif "REMOVE" in k and k.split("|")[2].endswith("solve_quality") and b.startswith("RetainedPrecisionError:G5/RETAINED_PRECISION_PRODUCT_ATTEMPT_MISMATCH") and h.startswith("RetainedPrecisionError:G5/RETAINED_PRECISION_ATTEMPT_MISMATCH"):
        cls = "N2 (G5 fallback PRODUCT_ATTEMPT -> G5 ATTEMPT)"
    elif "REMOVE" in k and k.split("|")[2].endswith("solve_quality") and b.startswith("ValueError:RETAINED_PRECISION_PRODUCT_ATTEMPT_MISMATCH") and h.startswith("ValueError:RETAINED_PRECISION_ATTEMPT_MISMATCH"):
        cls = "N2 (dispatch)"
    classes[(section, cls)] += 1
    lines.append(f"{cls}\t{k}\n\tbase {b}\n\thead {h}")
with open(sys.argv[3], "w") as out:
    out.write(f"inputs per section: {dict(by_section)}; total {len(base)}\n")
    out.write(f"identical: {len(base) - len(lines)}; differing: {len(lines)}\n")
    for (section, cls), n in sorted(classes.items()):
        out.write(f"  {section}\t{cls}\t{n}\n")
    out.write("\n".join(lines) + "\n")
print(open(sys.argv[3]).read()[:3000])
