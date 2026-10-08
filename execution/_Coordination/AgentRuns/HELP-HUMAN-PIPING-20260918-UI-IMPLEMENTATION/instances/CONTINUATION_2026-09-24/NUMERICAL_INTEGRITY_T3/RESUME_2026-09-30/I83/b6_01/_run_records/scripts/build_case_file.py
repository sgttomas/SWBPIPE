"""B6 (I83): the carrier case file's declarations after items 2 and 3. Serialization as the file's own
(json indent 2 + newline). (1) The N-3 scope sentence is removed: TS now refuses that class with the base
readers' SOURCE_NUMERICAL_CASE_INVALID (PLAN decision 11). (2) F-U6b-2's declared difference is removed:
Python's transport dispatch now runs the reader's transport validator, so the unedited successor's
transport is ok in all three languages. (3) The transport scope sentence names Python's code as Rust's.
Usage: build_case_file.py <case file path> [A|B|AB] (A: item 2's (1); B: item 3's (2) and (3), on A's output)."""
import hashlib, json, sys
path = sys.argv[1]
stage = sys.argv[2] if len(sys.argv) > 2 else "AB"
raw = open(path, "rb").read()
if "A" in stage:
    assert hashlib.sha256(raw).hexdigest() == "cf82deab5f5aee9b95df6c8d24fae9c3f438cf5bdf6bb173ae3a0ed74e9efbd8"
d = json.loads(raw)
assert (json.dumps(d, indent=2) + "\n").encode() == raw
scope = d["scope"]
n3 = (" An invalid enum value in a not_required case's quality is refused at G7 with each language's own code (Python SOURCE_NUMERICAL_CASE_INVALID, "
      "Rust SOURCE_NUMERICAL_CASE_INVALID, TS SOURCE_PRODUCER_CONTRACT_UNSUPPORTED, TS's G7 contract check firing first): a value outside the vocabulary of "
      "accuracy_evidence, structural_status or model_matrix_fidelity in the numerical_quality case of a not_required receipt case, made hash-consistent "
      "(observed for all three fields, RV94 u7_01 N-3; pinned by the shared corpus probe g7_not_required_quality_enum_invalid), so for this class G7 parity "
      "compares the gate and each language asserts its own code (ROOT_RULINGS_V1 \"RV94 on U7: PASS; the summary aligned across languages; the stale-comment "
      "repair; a public-activation checklist\"; I67 u7_repair_01).")
if "A" in stage:
    assert scope.count(n3) == 1 and scope.endswith(n3)
    scope = scope.replace(n3, "")
else:
    assert n3 not in scope
d["scope"] = scope
old = "(TS SOURCE_NUMERICAL_CONTRACT_UNSUPPORTED; Rust the reader's G0 code or its base header code; Python F-U6b-2's code)"
new = ("(TS SOURCE_NUMERICAL_CONTRACT_UNSUPPORTED; Rust and Python the reader's G0 code or their base header code, Python's transport "
       "dispatch running the reader's transport validator since B6, F-U6b-2)")
if "B" in stage:
    assert scope.count(old) == 1
    d["scope"] = scope.replace(old, new)
    entries = d["declared_differences"]
    assert [e["id"] for e in entries].count("F-U6b-2:python_refuses_transport") == 1
    d["declared_differences"] = [e for e in entries if e["id"] != "F-U6b-2:python_refuses_transport"]
out = (json.dumps(d, indent=2) + "\n").encode()
open(path, "wb").write(out)
print(hashlib.sha256(out).hexdigest(), len(out), len(d["declared_differences"]))
