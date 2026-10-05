"""Synthetic ownership controls only. No maintained/runtime/model code is executed."""
import json
from pathlib import Path

def closure(roots, edges):
    found = set()
    pending = list(roots)
    while pending:
        token = pending.pop()
        if token not in found:
            found.add(token)
            pending.extend(edges.get(token, ()))
    return found

def weight(tokens, weights):
    return sum(weights.get(token, 0) for token in tokens)

def control(name, roots, edges, expected, weights=None, expected_bytes=None):
    actual = closure(roots, edges)
    assert actual == set(expected), (name, actual, set(expected))
    result = {"name": name, "reachable_tokens": sorted(actual), "passed": True}
    if weights is not None:
        actual_bytes = weight(actual, weights)
        assert actual_bytes == expected_bytes, (name, actual_bytes, expected_bytes)
        result.update(synthetic_bytes=actual_bytes)
    return result

def main():
    cases = []
    # Two snapshots and a group cache point to one actual build allocation.
    edges = {"group": ["S1"], "caseA": ["S1","ZA"], "caseB":["S1","ZB"],
             "S1":["factor1"], "ZA":["uA"], "ZB":["uB"]}
    cases.append(control("shared_build_one_actual_payload", ["group","caseA","caseB"], edges,
                         ["group","caseA","caseB","S1","factor1","ZA","ZB","uA","uB"],
                         {"S1":3,"factor1":11,"ZA":2,"ZB":2,"uA":5,"uB":5},28))
    # Equal bytes/digest do not create an ownership edge.
    cases.append(control("equal_value_distinct_builds", ["A","B"],
                         {"A":["S1"],"B":["S2"],"S1":["F1"],"S2":["F2"]},
                         ["A","B","S1","S2","F1","F2"],
                         {"S1":3,"S2":3,"F1":11,"F2":11},28))
    # Copy generations, not copied Arc payloads. Import is first occupied.
    slots = [{"256":"r_orig"}, {"256":"r_ignored","512":"S512"}, {"512":"ignoredS"}]
    imported = {}
    for operand in slots:
        for precision, allocation in operand.items():
            imported.setdefault(precision, allocation)
    assert imported == {"256":"r_orig", "512":"S512"}
    cases.append(control("failed_cache_copy_generations_and_first_occupied",
        ["operand","later","attempt","import","snapshot"],
        {"operand":["r_orig"],"later":["r_ignored","S512"],"attempt":["r_attempt"],
         "import":["r_import","S512"],"snapshot":["r_snapshot","S512"]},
        ["operand","later","attempt","import","snapshot","r_orig","r_ignored",
         "r_attempt","r_import","r_snapshot","S512"],
        {"r_orig":7,"r_ignored":7,"r_attempt":7,"r_import":7,"r_snapshot":7,"S512":13},48))
    # Report's vector is moved, summary copied; no second moved-resolution buffer.
    before = control("report_before_move", ["pass"], {"pass":["res","delta"]},
                     ["pass","res","delta"], {"res":5,"delta":9},14)
    after = control("report_move_plus_summary_copy", ["report","summary"],
                    {"report":["res","delta"],"summary":["res_copy","theta","bound"]},
                    ["report","summary","res","delta","res_copy","theta","bound"],
                    {"res":5,"delta":9,"res_copy":5,"theta":2,"bound":3},24)
    assert "res_copy" not in before["reachable_tokens"]
    cases.extend([before,after])
    # Uncached failed build drops all partial numerical children; metadata persists.
    cases.append(control("partial_failure_live", ["cache","build"],
                         {"build":["members_partial","matrix_partial"]},
                         ["cache","build","members_partial","matrix_partial"]))
    cases.append(control("partial_failure_dropped_uncached", ["cache","record"],
                         {"cache":[],"record":["attempt_backing"]},
                         ["cache","record","attempt_backing"]))
    # Into-legacy case drops vector; combo clear drops elements but not backing.
    terminal_edges={"terminal":["attempt_backing"],"attempt_backing":["summary","refusals"]}
    cases.append(control("recorded_refused_before_projection",["terminal"],terminal_edges,
                         ["terminal","attempt_backing","summary","refusals"],
                         {"attempt_backing":17,"summary":5,"refusals":7},29))
    cases.append(control("case_legacy_drop",["legacy"],{"legacy":[]},["legacy"],{},0))
    cases.append(control("combination_legacy_clear_retains_capacity",["legacy"],
                         {"legacy":["attempt_backing"],"attempt_backing":[]},
                         ["legacy","attempt_backing"],{"attempt_backing":17},17))
    # Reallocation old backing is outside logical new graph but remains moving-live.
    requested = closure(["vector"], {"vector":["new"]})
    assert weight(requested, {"new":16,"old":8}) == 16
    moving = requested | {"old"}
    assert weight(moving, {"new":16,"old":8}) == 24
    cases.append({"name":"moving_resize_old_plus_new","requested_synthetic_bytes":16,
                  "moving_synthetic_bytes":24,"passed":True})
    # Dropping one Arc edge does not release a payload still held by another root.
    cases.append(control("last_arc_reference_controls_release",["caseB"],
                         {"caseB":["S1"],"S1":["factor1"]},["caseB","S1","factor1"]))
    # No storage-size claim: all numbers above are chosen abstract token weights.
    output={"status":"pass","scope":"synthetic allocation-identity bookkeeping",
            "production_probe":False,"controls":cases}
    Path(__file__).with_name("CHECKS.json").write_text(json.dumps(output,indent=2)+"\n")
    print(json.dumps({"status":"pass","controls":len(cases)}))
if __name__ == "__main__":
    main()

