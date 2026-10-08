"""RV113 (RV-R): the reviewer's own Python mutant schema for SR-PY (head 11cc14e3e6).

Each mutant is an environment-guarded edit: `_mx("Q07")` is true only when RV113_MUT == "Q07", so one copy
serves every mutant and the copy with RV113_MUT unset behaves as the head (the control). Each edit must match
its anchor exactly once. Usage: python make_py_mutants.py <P root of the mutant copy> <table.json>
"""
import json
import sys

root = sys.argv[1]
RP = f"{root}/core/analysis_runs/retained_precision.py"
CP = f"{root}/core/analysis_runs/compatibility.py"
GUARD = 'import os as _rv113_os\n\n\ndef _mx(i):\n    return _rv113_os.environ.get("RV113_MUT") == i\n\n\n'

M = []  # (id, file, what, [(old, new), ...])


def m(i, f, what, *subs):
    M.append((i, f, what, list(subs)))


# --- G8's per-case loop (DESIGN_v2 §3.3; F-1 text B P1-P4) and repair 01 (b)-(e) ---
m("Q01", RP, "per-case loop scope: requested mode, (b), P1 and P2-P4 checked on selected cases only (I1's sourced scope)",
  ('need(o["requested_mode"] == mode)', 'need((_mx("Q01") and case["status"] != "selected") or o["requested_mode"] == mode)'),
  ('need(o["material_basis_ref"] == case_bases[i])', 'need((_mx("Q01") and case["status"] != "selected") or o["material_basis_ref"] == case_bases[i])'),
  ('need(len(modes) == 1 and type(modes[0]["value"])', 'need((_mx("Q01") and case["status"] != "selected") or len(modes) == 1 and type(modes[0]["value"])'),
  ('need(parity <= 1 and (parity == 0 or mode == "dense_scrutiny")', 'need((_mx("Q01") and case["status"] != "selected") or parity <= 1 and (parity == 0 or mode == "dense_scrutiny")'))
m("Q02", RP, "requested mode check removed",
  ('or o["requested_mode"] == mode)', 'or _mx("Q02") or o["requested_mode"] == mode)'))
m("Q03", RP, "P1: at least one mode row instead of exactly one",
  ('or len(modes) == 1 and type(modes[0]["value"])', 'or (len(modes) >= 1 if _mx("Q03") else len(modes) == 1) and type(modes[0]["value"])'))
m("Q04", RP, "P1: mode code 3 (or the other mode's code) accepted",
  ('and modes[0]["value"] == mode_code)', 'and (modes[0]["value"] in (1, 2, 3) if _mx("Q04") else modes[0]["value"] == mode_code))'))
m("Q05", RP, "P2 removed (several parity rows)",
  ('or parity <= 1 and (parity == 0', 'or (_mx("Q05") or parity <= 1) and (parity == 0'))
m("Q06", RP, "P3 removed (a parity row in sparse_interactive)",
  ('(parity == 0 or mode == "dense_scrutiny")', '(_mx("Q06") or parity == 0 or mode == "dense_scrutiny")'))
m("Q07", RP, "P4 removed (a parity row on a W2-published case)",
  ('(parity == 0 or o["w2"]["kind"] != "published")', '(_mx("Q07") or parity == 0 or o["w2"]["kind"] != "published")'))
m("Q08", RP, "(b) removed: the ordinary attempt's material basis per case",
  ('or o["material_basis_ref"] == case_bases[i])', 'or _mx("Q08") or o["material_basis_ref"] == case_bases[i])'))
m("Q09", RP, "(c) count removed: one basis per selector",
  ('need(len(body["material_bases"]) == len(selectors))', 'need(_mx("Q09") or len(body["material_bases"]) == len(selectors))'))
m("Q10", RP, "(c) exact case lists removed (selector kept)",
  ('and mb["case_indices"] == [i for i, k in enumerate(case_bases) if k == mi])', 'and (_mx("Q10") or mb["case_indices"] == [i for i, k in enumerate(case_bases) if k == mi]))'))
m("Q11", RP, "(e) removed: a basis's materials checked only through a CaseSource",
  ('need([m["input_index"] for m in mb["materials"]] == [j for j, m in enumerate(materials) if m["id"] in used])',
   'need(_mx("Q11") or [m["input_index"] for m in mb["materials"]] == [j for j, m in enumerate(materials) if m["id"] in used])'),
  ('        for m in mb["materials"]:\n            raw = materials[int(m["input_index"])]; pair, selection = selected_material(raw, model["load_cases"][mb["case_indices"][0]])',
   '        for m in ([] if _mx("Q11") else mb["materials"]):\n            raw = materials[int(m["input_index"])]; pair, selection = selected_material(raw, model["load_cases"][mb["case_indices"][0]])'))
m("Q12", RP, "(d1) removed: the invocation's members",
  ('need(type(invocation) is dict and set(invocation) == {"request", "solver_mode"}', 'need(type(invocation) is dict and (_mx("Q12") or set(invocation) == {"request", "solver_mode"})'))
m("Q13", RP, "(d2) removed: the solver mode is one of the two",
  ('and invocation["solver_mode"] in ("sparse_interactive", "dense_scrutiny"), "INVOCATION_MISMATCH")', 'and (_mx("Q13") or invocation["solver_mode"] in ("sparse_interactive", "dense_scrutiny")), "INVOCATION_MISMATCH")'))
# --- G5's not_required rule: each of Rust's three dropped conjuncts restored ---
NR_OLD = 'fail(c["product_attempt_ref"] is None and verdict == "checks_passed")'
NR_NEW = ('fail((_mx("Q17") or c["product_attempt_ref"] is None) and (_mx("Q18") or verdict == "checks_passed")'
          ' and (not _mx("Q14") or o["initial"]["kind"] == "report")'
          ' and (not _mx("Q15") or o["initial"].get("outcome") == "checks_passed")'
          ' and (not _mx("Q16") or o["w2"]["kind"] == "not_triggered"))')
m("Q14", RP, "not_required: initial must be a report (Rust's dropped conjunct restored; one shared edit carries Q14-Q18)", (NR_OLD, NR_NEW))
m("Q15", RP, "not_required: the initial report's outcome must be checks_passed (Rust's dropped conjunct restored)")
m("Q16", RP, "not_required: W2 must be not_triggered (Rust's dropped conjunct restored)")
m("Q17", RP, "not_required: product_attempt_ref null dropped")
m("Q18", RP, "not_required: the verdict checks_passed dropped")
# --- R-D38 (4b) ---
m("Q19", RP, "(4b) branch removed (I1: an entered native stage needs a Run)",
  ('elif st["native"] == "failed" and a["run_ref"] is None:', 'elif not _mx("Q19") and st["native"] == "failed" and a["run_ref"] is None:'))
m("Q20", RP, "(4b) predicate always true (every conjunct dropped)",
  ('    st = a["stages"]\n    reason = case.get("reason") or {}', '    if _mx("Q20"): return True\n    st = a["stages"]\n    reason = case.get("reason") or {}'))
m("Q21", RP, "(4b): the case's Run and the attempt's proof absent dropped",
  ('return (case.get("run") is None and a["proof"] is None', 'return ((_mx("Q21") or case.get("run") is None and a["proof"] is None)'))
m("Q22", RP, "(4b): an unavailable capture result dropped",
  ('and a["result"]["kind"] == "unavailable" and a["result"]["error"]["kind"] == "capture"', 'and (_mx("Q22") or a["result"]["kind"] == "unavailable" and a["result"]["error"]["kind"] == "capture")'))
m("Q23", RP, "(4b): stages after native not_entered dropped",
  ('and all(st[k] == "not_entered" for k in STAGE_ORDER[2:])', 'and (_mx("Q23") or all(st[k] == "not_entered" for k in STAGE_ORDER[2:]))'))
m("Q24", RP, "(4b): preparation completed dropped",
  ('and st["preparation"] == "completed" and st["native"] == "failed"\n', 'and (_mx("Q24") or st["preparation"] == "completed") and st["native"] == "failed"\n'))
m("Q25", RP, "(4b): the cause naming this attempt dropped",
  ('\n            and cause.get("product_attempt_ref") == ai\n', '\n            and (_mx("Q25") or cause.get("product_attempt_ref") == ai)\n'))
m("Q26", RP, "(4b): the reason (source_unavailable, preparation) dropped",
  ('and (reason.get("code"), reason.get("phase")) == ("source_unavailable", "preparation")', 'and (_mx("Q26") or (reason.get("code"), reason.get("phase")) == ("source_unavailable", "preparation"))'))
m("Q27", RP, "(4b): a non-null source reference equal to the case's dropped",
  ('and a["source_ref"] is not None and a["source_ref"] == case.get("source_ref"))', 'and (_mx("Q27") or a["source_ref"] is not None and a["source_ref"] == case.get("source_ref")))'))
m("Q28", RP, "(4b): the case unavailable with prepared_product_failure dropped",
  ('\n            and case["status"] == "unavailable" and cause.get("kind") == "prepared_product_failure"\n', '\n            and (_mx("Q28") or case["status"] == "unavailable" and cause.get("kind") == "prepared_product_failure")\n'))
m("Q29", RP, "the Build check removed (every Build referenced by its building record)",
  ('fail(built_refs == set(range(len(body["builds"]))), "WORK_MISMATCH")', 'fail(_mx("Q29") or built_refs == set(range(len(body["builds"]))), "WORK_MISMATCH")'))
# --- RV108 N2 ---
m("Q30", RP, "N2 removed: a missing solve_quality read by key (KeyError to the fallback)",
  ('verdict = quality[i]["solve_quality"] if "solve_quality" in quality[i] else None', 'verdict = quality[i]["solve_quality"] if (_mx("Q30") or "solve_quality" in quality[i]) else None'))
# --- RV108 N1 (compatibility.py `_source_contract`) ---
m("Q31", CP, "N1 guard removed: numerical_quality.status",
  ('or not isinstance(quality.get("status"), str) or', 'or (not _mx("Q31") and not isinstance(quality.get("status"), str)) or'))
for qid, field in (("Q32", "solve_quality"), ("Q33", "structural_status"), ("Q34", "model_matrix_fidelity"), ("Q35", "accuracy_evidence")):
    m(qid, CP, f"N1 guard removed: case {field}",
      (f'or not isinstance(case.get("{field}"), str) or', f'or (not _mx("{qid}") and not isinstance(case.get("{field}"), str)) or'))

texts = {RP: open(RP).read(), CP: open(CP).read()}
table = []
for qid, f, what, subs in M:
    for old, new in subs:
        n = texts[f].count(old)
        assert n == 1, (qid, n, old)
        texts[f] = texts[f].replace(old, new)
    table.append({"id": qid, "file": f.rsplit("/", 1)[1], "what": what, "edits": len(subs)})
for f, t in texts.items():
    lines = t.split("\n")
    # after the module docstring and __future__ imports: insert the guard before the first top-level import
    k = next(i for i, l in enumerate(lines) if (l.startswith("import ") or l.startswith("from ")) and "__future__" not in l)
    lines.insert(k, GUARD.rstrip("\n") + "\n")
    open(f, "w").write("\n".join(lines))
json.dump(table, open(sys.argv[2], "w"), indent=1)
print(len(table), "mutants")
