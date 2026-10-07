"""RV116 addendum 01: independent checks of I96's revision-01 statics (standard library only).

Uses RV116's own canonicalizer (rv116_checks.jcs / H), not I96's generator, on the committed bytes:
  1. DEF-E r1 is canonical, ASCII, float-free; its raw sha256 and H; DEF-O's pinned H as control.
  2. DEF-E r1 against the committed v0 draft: exactly which member paths differ.
  3. XTABLE r1 against v0: textual diff (only the definition hash line), and the bound hash equals DEF-E r1's H;
     XTABLE r1's inherited hash equals physics-1's raw sha256.
  4. SCHEMA_ENUM.diff (r1): applied to main's SCHEMA, the parsed difference is exactly title, $comment and
     ProductAttempt.definition_id; the $comment keeps main's text after the first sentence.
  5. CARRIER_PROFILE_ENUMS.diff: applied to main's two carriers, the parsed difference is exactly one string
     appended to each formulation_basis.profile_id enum, and no other enum anywhere changes.

Usage: python rv116_r1_checks.py <P> <I96 folder> <out.json>
"""
import difflib
import hashlib
import json
import re
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))
import rv116_checks as mine


def sha(b):
    return hashlib.sha256(b).hexdigest()


def apply_udiff(original, diff_text, target):
    """Apply the hunks of one file from a unified diff (exact context match required)."""
    lines = original.decode().splitlines(True)
    blocks = re.split(r"(?m)^--- ", diff_text)
    block = [b for b in blocks if "\n" in b and b.split("\n", 2)[1].startswith("+++ b/P/" + target)]
    assert len(block) == 1, target
    hunks = re.split(r"(?m)^@@ ", block[0])[1:]
    out, pos = [], 0
    for h in hunks:
        header, body = h.split("\n", 1)
        m = re.match(r"-(\d+)(?:,(\d+))? \+(\d+)(?:,(\d+))? @@", header)
        start = int(m.group(1)) - 1
        out.extend(lines[pos:start])
        pos = start
        for ln in body.splitlines(True):
            if ln.startswith("\\"):
                continue
            tag, text = ln[0], ln[1:]
            if tag == " ":
                assert lines[pos] == text, (target, pos)
                out.append(text)
                pos += 1
            elif tag == "-":
                assert lines[pos] == text, (target, pos)
                pos += 1
            elif tag == "+":
                out.append(text)
    out.extend(lines[pos:])
    return "".join(out).encode()


def diff_paths(a, b, path=""):
    if type(a) is not type(b):
        return [path or "/"]
    if isinstance(a, dict):
        out = []
        for k in sorted(set(a) | set(b)):
            if k not in a or k not in b:
                out.append(f"{path}/{k}")
            else:
                out.extend(diff_paths(a[k], b[k], f"{path}/{k}"))
        return out
    if isinstance(a, list):
        if len(a) != len(b):
            return [path]
        out = []
        for i, (x, y) in enumerate(zip(a, b)):
            out.extend(diff_paths(x, y, f"{path}/{i}"))
        return out
    return [] if a == b else [path]


def main():
    P, I96, outp = Path(sys.argv[1]), Path(sys.argv[2]), Path(sys.argv[3])
    res = P / "fixtures/results"
    rep = {}
    defo = json.loads((res / "retained_precision_prepared_ordinary_v1.json").read_bytes())
    v0_raw = (I96 / "statics/retained_precision_prepared_exact_v1.json").read_bytes()
    r1_raw = (I96 / "statics/r1/retained_precision_prepared_exact_v1.json").read_bytes()
    v0, r1 = json.loads(v0_raw), json.loads(r1_raw)
    rep["definition"] = {
        "DEF_O_H_control": mine.H("retained_precision_formation_v1", defo),
        "v0_H": mine.H("retained_precision_formation_v1", v0),
        "r1_bytes": len(r1_raw), "r1_raw_sha256": sha(r1_raw),
        "r1_H": mine.H("retained_precision_formation_v1", r1),
        "r1_canonical": mine.jcs(r1).encode() == r1_raw,
        "r1_ascii": all(c < 128 for c in r1_raw),
        "r1_preparation_equals_DEF_O": r1["preparation"] == defo["preparation"],
        "paths_differing_from_v0": diff_paths(v0, r1),
        "r1_evidence": r1["evidence"], "r1_rows_maximum": r1["rows"]["maximum"],
        "r1_scope_excludes": r1["scope"]["excludes"], "r1_scope_pressure": r1["scope"]["pressure"],
    }
    t0 = (I96 / "statics/semantic_contract_v0_3_physics_retained_1.json").read_bytes()
    t1 = (I96 / "statics/r1/semantic_contract_v0_3_physics_retained_1.json").read_bytes()
    p1 = (res / "semantic_contract_v0_3_physics_1.json").read_bytes()
    d = [l for l in difflib.unified_diff(t0.decode().splitlines(), t1.decode().splitlines(), lineterm="", n=0)
         if l[:1] in "+-" and not l.startswith(("+++", "---"))]
    tj = json.loads(t1)
    rep["table"] = {"r1_bytes": len(t1), "r1_sha256": sha(t1), "changed_lines_vs_v0": d,
                    "binds_r1_H": tj["product_formation_definitions"] == [{"id": "RP-PREPARED-EXACT-DUAL-v1",
                                                                         "sha256": rep["definition"]["r1_H"]}],
                    "formation_warrant_id": tj["formation_warrant"]["definition_id"],
                    "inherited_equals_physics_1": tj["inherited_semantic_contract_sha256"] == sha(p1),
                    "paths_differing_from_v0": diff_paths(json.loads(t0), tj)}
    # SCHEMA diff
    sraw = (P / "schemas/retained_precision_mp_v2.schema.json").read_bytes()
    sd = (I96 / "statics/r1/SCHEMA_ENUM.diff").read_text()
    snew = apply_udiff(sraw, sd, "schemas/retained_precision_mp_v2.schema.json")
    s0, s1 = json.loads(sraw), json.loads(snew)
    first0 = s0["$comment"].split(". ", 1)
    first1 = s1["$comment"].split("extension. ", 1)
    rep["schema"] = {"main_sha256": sha(sraw), "patched_sha256": sha(snew),
                     "paths_differing": diff_paths(s0, s1),
                     "title": s1["title"], "comment_first_sentence": first1[0] + "extension.",
                     "comment_rest_unchanged": s0["$comment"].split("extension. ", 1)[1] == first1[1],
                     "definition_id": s1["$defs"]["ProductAttempt"]["properties"]["definition_id"],
                     "operand_preparation_present": "OperandPreparation" in s1["$defs"]}
    # carrier diffs
    cd = (I96 / "statics/r1/CARRIER_PROFILE_ENUMS.diff").read_text()
    carriers = {}
    for name in ("schemas/results.v0.3.schema.yaml", "schemas/stress_neutral_export.v0.3.schema.json"):
        a = (P / name).read_bytes()
        b = apply_udiff(a, cd, name)
        ja, jb = json.loads(a), json.loads(b)
        carriers[name] = {"main_sha256": sha(a), "patched_sha256": sha(b), "paths_differing": diff_paths(ja, jb)}

        def enums(v, path=""):
            if isinstance(v, dict):
                for k, x in v.items():
                    if k == "enum":
                        yield path, x
                    yield from enums(x, f"{path}/{k}")
            elif isinstance(v, list):
                for i, x in enumerate(v):
                    yield from enums(x, f"{path}/{i}")
        ea, eb = dict(enums(ja)), dict(enums(jb))
        carriers[name]["enums_changed"] = {p: [ea[p], eb[p]] for p in ea if ea[p] != eb.get(p)}
    rep["carriers"] = carriers
    outp.write_text(json.dumps(rep, indent=2, sort_keys=True, ensure_ascii=True) + "\n")
    print(json.dumps(rep, indent=1, sort_keys=True))


if __name__ == "__main__":
    main()
