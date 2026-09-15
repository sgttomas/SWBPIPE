#!/usr/bin/env python3
"""Supplement V4 verification with F03 raw-native semantic accounting.

Preparation only. Run after the repaired candidate and its packet representation are frozen.
V4 remains the owner for schemas/checksums/nine-member materialization.
"""
from __future__ import annotations
import argparse, hashlib, json
from pathlib import Path
from typing import Any

def load(path: Path) -> Any:
    return json.loads(path.read_text(encoding="utf-8"))

def classify(row: dict[str, Any], contract: list[dict[str, Any]]) -> tuple[str, dict[str, Any] | None, str]:
    kinds = [item for item in contract if item.get("kind") == row.get("kind")]
    if not kinds:
        return "missing_semantic", None, "kind_absent"
    units = [item for item in kinds if item.get("unit") == row.get("unit")]
    if not units:
        return "contradictory_semantic", None, "unit_contradicts_known_kind"
    component = (row.get("metadata") or {}).get("component")
    if isinstance(component, str) and component:
        exact = next((item for item in units if item.get("component") == component), None)
        generic = next((item for item in units if item.get("component") is None), None)
        semantic = exact or generic
        if semantic is None:
            return "contradictory_semantic", None, "component_contradicts_known_kind_unit"
    else:
        generic = next((item for item in units if item.get("component") is None), None)
        if generic is None and any(item.get("component") is not None for item in units):
            return "missing_semantic", None, "required_component_absent"
        semantic = generic or units[0]
    category = semantic.get("category")
    if category == "diagnostic_work":
        return "diagnostic_work", semantic, "accepted_contract"
    dimension = semantic.get("derivative_target_dimension")
    if not isinstance(dimension, str) or not dimension:
        return "missing_semantic", semantic, "target_dimension_absent"
    return "eligible", semantic, "accepted_contract"

def main() -> None:
    p=argparse.ArgumentParser()
    p.add_argument("--working-root", required=True, type=Path)
    p.add_argument("--source-output", required=True, type=Path)
    p.add_argument("--stress-packet", required=True, type=Path)
    p.add_argument("--report", required=True, type=Path)
    p.add_argument("--expect-rows", type=int)
    p.add_argument("--expect-witnesses", type=int)
    p.add_argument("--expect-diagnostic-work", type=int)
    p.add_argument("--require-dimension-absent-native", action="store_true")
    a=p.parse_args()
    source_bytes=a.source_output.read_bytes(); source_sha=hashlib.sha256(source_bytes).hexdigest()
    source_doc=json.loads(source_bytes); source=source_doc.get("mechanics_envelope", source_doc)
    rows=source.get("results")
    if not isinstance(rows, list) or not all(isinstance(row, dict) for row in rows):
        raise AssertionError("F03_SOURCE_RESULTS_SHAPE")
    packet=load(a.stress_packet); exported=packet.get("result_rows")
    if not isinstance(exported, list) or len(exported)!=len(rows):
        raise AssertionError("F03_ALL_RAW_ROWS_NOT_RETAINED")
    contract_path=a.working_root.resolve()/"fixtures/results/semantic_contract_v0_2.json"
    contract_doc=load(contract_path); contract=contract_doc["rows"]
    contract_sha=hashlib.sha256(contract_path.read_bytes()).hexdigest()
    classifications: dict[str, dict[str, Any]]={}
    eligible: dict[str, tuple[int,dict[str,Any],dict[str,Any]]]={}
    diagnostic_work: set[str]=set(); unavailable: set[str]=set()
    for index,row in enumerate(rows):
        status,semantic,reason=classify(row,contract)
        rid=row.get("id")
        if not isinstance(rid,str) or not rid:
            raise AssertionError(f"F03_SOURCE_ID:{index}")
        classifications[rid]={"source_row_index":index,"classification":status,"reason":reason,"signature_id":semantic.get("signature_id") if semantic else None,"interpreted_dimension":semantic.get("derivative_target_dimension") if semantic else None}
        if status=="eligible": eligible[rid]=(index,row,semantic)  # type: ignore[arg-type]
        elif status=="diagnostic_work": diagnostic_work.add(rid)
        else: unavailable.add(rid)
    exported_by_id={row.get("result_id"):row for row in exported}
    if set(exported_by_id)!=set(classifications) or len(exported_by_id)!=len(exported):
        raise AssertionError("F03_ROW_IDENTITY_OR_DUPLICATE")
    witnesses=packet.get("unit_preservation_witnesses")
    if not isinstance(witnesses,list): raise AssertionError("F03_WITNESSES_SHAPE")
    witness_by_id={item.get("result_id"):item for item in witnesses if isinstance(item,dict)}
    if len(witness_by_id)!=len(witnesses) or set(witness_by_id)!=set(eligible):
        raise AssertionError("F03_ELIGIBLE_WITNESS_BIJECTION")
    for rid,(index,raw,semantic) in eligible.items():
        target=exported_by_id[rid]; witness=witness_by_id[rid]; dimension=semantic["derivative_target_dimension"]
        if (target.get("value"),target.get("unit"),target.get("dimension"))!=(raw.get("value"),raw.get("unit"),dimension):
            raise AssertionError(f"F03_TARGET_VALUE_UNIT_DIMENSION:{rid}")
        expected={"value":raw.get("value"),"unit":raw.get("unit"),"dimension":dimension}
        if witness.get("source_row_index")!=index or witness.get("source_quantity")!=expected or witness.get("target_quantity")!=expected or witness.get("conversion_performed") is not False:
            raise AssertionError(f"F03_WITNESS_VALUE_UNIT_INTERPRETATION:{rid}")
    diagnostics=packet.get("diagnostics")
    if not isinstance(diagnostics,list): raise AssertionError("F03_DIAGNOSTICS_SHAPE")
    work_withheld={item.get("source",{}).get("ref") for item in diagnostics if isinstance(item,dict) and item.get("code")=="SN-UNIT-WITNESS-WITHHELD-DIAGNOSTIC-WORK"}
    if work_withheld!=diagnostic_work:
        raise AssertionError("F03_DIAGNOSTIC_WORK_WITHHOLD_BIJECTION")
    for rid in unavailable:
        if rid in work_withheld or rid in witness_by_id:
            raise AssertionError(f"F03_UNAVAILABLE_CONFLATED_WITH_DIAGNOSTIC_WORK:{rid}")
        findings=[item for item in diagnostics if isinstance(item,dict) and item.get("source",{}).get("ref")==rid and item.get("code")!="SN-UNIT-WITNESS-WITHHELD-DIAGNOSTIC-WORK"]
        if not findings or not any(item.get("severity")=="blocking" for item in findings):
            raise AssertionError(f"F03_UNAVAILABLE_NOT_EXPLICITLY_BLOCKED:{rid}")
    if a.require_dimension_absent_native and any("dimension" in row for row in rows):
        raise AssertionError("F03_NATIVE_SOURCE_WAS_ENRICHED")
    expected=((a.expect_rows,len(rows)),(a.expect_witnesses,len(eligible)),(a.expect_diagnostic_work,len(diagnostic_work)))
    for want,got in expected:
        if want is not None and want!=got: raise AssertionError(f"F03_EXPECTED_COUNT:{want}:{got}")
    if hashlib.sha256(a.source_output.read_bytes()).hexdigest()!=source_sha:
        raise AssertionError("F03_SOURCE_BYTES_CHANGED")
    report={"status":"PASS","source_output":{"path":str(a.source_output.resolve()),"sha256_before_after":source_sha,"raw_row_count":len(rows),"rows_with_raw_dimension":sum("dimension" in row for row in rows)},"semantic_contract":{"path":str(contract_path),"sha256":contract_sha,"signature_count":len(contract)},"accounting":{"eligible_witnesses":len(eligible),"diagnostic_work_withheld":len(diagnostic_work),"missing_or_contradictory_withheld":len(unavailable),"all_rows_retained":True},"classifications":classifications,"trace_statement":"Every emitted dimension was independently traced from raw kind/unit/component to derivative_target_dimension and signature_id in the accepted semantic contract; raw source values, units, rows and bytes were unchanged."}
    a.report.parent.mkdir(parents=True,exist_ok=True); a.report.write_text(json.dumps(report,indent=2,sort_keys=True)+"\n",encoding="utf-8")
    print(json.dumps({"status":"PASS","rows":len(rows),"witnesses":len(eligible),"diagnostic_work":len(diagnostic_work),"unavailable":len(unavailable)},sort_keys=True))
if __name__=="__main__": main()
