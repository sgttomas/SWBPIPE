"""Synthetic controls and received producer artifacts; no fresh solve/native witness."""
from copy import deepcopy
import hashlib
import json
import math
import struct
import sys
from pathlib import Path
import unittest

from core.analysis_runs.compatibility import numerical_use_standing, _source_contract
from core.analysis_runs.source_blocks import (
    CONTRACT_ID, CONTRACT_SHA256, CONTRACT_PATH, validate_source_blocks,
    validate_receipt_shape, domain_hash, _derived, NORM_INPUT_RELATIVE_LIMIT, STRESS_INPUT_RELATIVE_LIMIT, STRESS_ACTION_KINDS, STRESS_LOCATIONS,
)

ROOT = Path(__file__).resolve().parents[1]
CONTROL = ROOT / "core/reporting/result_export/tests/fixtures/source_blocks_statement_control.json"


def seal(source, context):
    """Test statements only: production receivers cannot finalize producer proof."""
    receipt = source["source_block_recovery"]
    body = receipt["body"]
    body["invocation"]["value"] = domain_hash("source_blocks_invocation_v1", context)
    body["publication_sha256"] = domain_hash("source_blocks_publication_v1", {k:v for k,v in source.items() if k != "source_block_recovery"})
    receipt["receipt_sha256"] = domain_hash("source_blocks_receipt_v1", body)
    return source


def control():
    payload = json.loads(CONTROL.read_text())
    raw = {row["id"]:row for row in payload["source"]["results"]}
    for case in payload["source"]["source_block_recovery"]["body"]["cases"]:
        for row in case["rows"]:
            row["recipe_id"] = {"translation_norm_v1":"translation_norm_scaled_v1", "support_force_norm_v1":"support_force_norm_scaled_v1"}.get(row["recipe_id"], row["recipe_id"])
            if row["recipe_id"] == "straight_open_stress_v1":
                # The original synthetic fixture listed the whole end vector;
                # the actual registered scalar recipe owns one typed action.
                kind = STRESS_ACTION_KINDS[raw[row["result_id"]]["kind"]]
                row["input_result_ids"] = [identity for identity in row["input_result_ids"] if raw[identity]["kind"] == kind]
    return seal(payload["source"], payload["context"]), payload["context"]


class SourceBlockStatements(unittest.TestCase):
    def test_pinned_additive_table_preserves_immutable_p1_rows(self):
        p1 = ROOT / "fixtures/results/semantic_contract_v0_3_precision_1.json"
        self.assertEqual(hashlib.sha256(p1.read_bytes()).hexdigest(), "d75aacee175e178dbdeb256d89a65f4b375265f7da077725ee635af33df51d7e")
        self.assertEqual(hashlib.sha256(CONTRACT_PATH.read_bytes()).hexdigest(), CONTRACT_SHA256)
        old, new = json.loads(p1.read_text()), json.loads(CONTRACT_PATH.read_text())
        self.assertEqual(new["rows"][:len(old["rows"])], old["rows"])
        self.assertEqual(new["source_signature_count"], 66)
        self.assertEqual({r["component"] for r in new["rows"][60:]}, {"Fx","Fy","Fz","Mx","My","Mz"})

    def test_valid_statements_need_independent_actual_context(self):
        source, context = control()
        bases = [c["basis_ref"] for c in source["source_block_recovery"]["body"]["cases"]]
        self.assertFalse(validate_source_blocks(source))
        self.assertEqual(numerical_use_standing(source, bases), "needs_recompute")
        self.assertTrue(validate_source_blocks(source, context))
        self.assertEqual(numerical_use_standing(source, bases, context), "numerically_eligible")
        altered = deepcopy(context); altered["request"]["unknown_but_captured"] = None
        self.assertNotEqual(numerical_use_standing(source, bases, altered), "numerically_eligible")
        altered = deepcopy(context); altered["solver_mode"] = "sparse_interactive"
        with self.assertRaises(ValueError): validate_source_blocks(source, altered)
        self.assertEqual(numerical_use_standing(source, [], context), "needs_recompute")

    def test_every_falsy_receipt_presence_blocks_p1_and_legacy_downgrade(self):
        source, _ = control()
        for version,contract in [("0.2.0","openpipestress.result_semantics/0.3.0/precision-1"),("0.2.0","openpipestress.result_semantics/0.3.0/physics-1"),("0.1.0",CONTRACT_ID)]:
            for value in [None,False,0,"",{},[]]:
                altered=deepcopy(source);altered["schema_version"]=version;altered["producer"]["semantic_contract_id"]=contract;altered["source_block_recovery"]=value
                with self.assertRaises(ValueError): _source_contract(altered)
        altered=deepcopy(source);altered["producer"]["semantic_contract_id"]="openpipestress.result_semantics/0.3.0/source-blocks-unknown"
        with self.assertRaises(ValueError): _source_contract(altered)

    def test_rehashed_invalid_claims_do_not_pass(self):
        source, context = control()
        def shape_extra(s): s["source_block_recovery"]["body"]["extra"] = True
        def false_bits(s): s["source_block_recovery"]["body"]["cases"][0]["projections"][0]["value_bits"] = "8000000000000000"
        def criterion(s): s["source_block_recovery"]["body"]["cases"][0]["projections"][0]["relative_limit"] = 1e-8
        def block(s): s["source_block_recovery"]["body"]["cases"][0]["source"]["free_blocks"][0].append(9)
        def work(s): s["source_block_recovery"]["body"]["cases"][0]["work"]["charged"] = 2000001
        def null_reservation(s): s["source_block_recovery"]["body"]["cases"][0]["work"]["rejected_reservation"]["amount"] = None
        def mode(s): s["source_block_recovery"]["body"]["cases"][0]["requested_mode"] = "sparse_interactive"
        def support(s): s["source_block_recovery"]["body"]["cases"][0]["supports"][0]["components"][0]["action_terms"][0]["source_id"] = "unowned"
        def missing_row(s):
            c=s["source_block_recovery"]["body"]["cases"][0];identity=c["projections"][0]["result_id"]
            s["results"]=[r for r in s["results"] if r["id"]!=identity];c["projections"]=[p for p in c["projections"] if p["result_id"]!=identity];c["rows"]=[r for r in c["rows"] if r["result_id"]!=identity]
            for r in c["rows"]:r["input_result_ids"]=[i for i in r["input_result_ids"] if i!=identity]
        def headline(s): s["summary"]["max_open_formula_stress"]["value"] = 1.0
        def quality(s): s["numerical_quality"]["status"] = "sensitive"
        def failed_stage(s): s["source_block_recovery"]["body"]["cases"][0]["failure"] = {"stage":"finalization","code":"budget","diagnostic_ref":"synthetic-report","block_order":None}
        for mutate in [shape_extra,false_bits,criterion,block,work,null_reservation,mode,support,missing_row,headline,quality,failed_stage]:
            with self.subTest(mutation=mutate.__name__):
                altered=deepcopy(source);mutate(altered);seal(altered,context)
                with self.assertRaises(ValueError):validate_source_blocks(altered,context)

    def test_recorded_unsupported_case_is_inspectable_and_unqualified(self):
        source,context=control();body=source["source_block_recovery"]["body"];case=body["cases"][0]
        case.update(outcome="unsupported",selected_method=None,source=None,projections=[],supports=[],failure={"stage":"eligibility","code":"unsupported_family","diagnostic_ref":"synthetic-report","block_order":None})
        for row in case["rows"]:row.update(treatment="inspection_only",projection_id=None,recipe_id=None,input_result_ids=[])
        body["status"]="unavailable";seal(source,context)
        self.assertFalse(validate_source_blocks(source,context));self.assertEqual(_source_contract(source)[0],CONTRACT_ID)

    def test_mixed_ordinary_case_uses_unchanged_p1_predicates(self):
        source,context=control();body=source["source_block_recovery"]["body"];first=body["cases"][0];basis={"ref_type":"load_case","ref_id":"ordinary-case"}
        second=deepcopy(first);second.update(basis_ref=basis,selected_method="ordinary_dense_structural_v1",source=None,projections=[],supports=[])
        second["ordinary_attempt"]["quality_case_index"]=1;second["ordinary_attempt"]["structural_report_diagnostic_ref"]="ordinary-report"
        extra=[];second["rows"]=[]
        for original in source["results"]:
            if original["kind"]=="support_reaction_component_v2":continue
            row=deepcopy(original);row["id"]="ordinary:"+row["id"];row["basis_ref"]=basis;extra.append(row)
            second["rows"].append({"result_id":row["id"],"treatment":"ordinary_checked","projection_id":None,"recipe_id":None,"input_result_ids":[]})
        source["results"].extend(extra);source["diagnostics"].append({"id":"ordinary-report","code":"NUMERICAL_INTEGRITY_CHECKS_PASSED","severity":"info","message":"Synthetic ordinary statement only."})
        quality=deepcopy(source["numerical_quality"]["cases"][0]);quality.update(basis_ref=basis,evidence_refs=["ordinary-report"]);source["numerical_quality"]["cases"].append(quality)
        body["cases"].append(second);body["invocation_work"]["charged"] += second["work"]["charged"] + second["work"]["reserved_unobserved_failure"];context["request"]["model"]["load_cases"].append({"id":"ordinary-case","primitive_loads":[]});source["summary"]["load_case_count"]=2;seal(source,context)
        self.assertTrue(validate_source_blocks(source,context))
        second["ordinary_attempt"]["outcome"]="sensitive";quality["solve_quality"]="sensitive";source["numerical_quality"]["status"]="sensitive";seal(source,context)
        with self.assertRaises(ValueError):validate_source_blocks(source,context)

    def test_shape_rejects_closed_unknown_and_nonnumeric_types(self):
        receipt=json.loads(CONTROL.read_text())["source"]["source_block_recovery"]
        for case in receipt["body"]["cases"]:
            for row in case["rows"]:
                row["recipe_id"] = {"translation_norm_v1":"translation_norm_scaled_v1", "support_force_norm_v1":"support_force_norm_scaled_v1"}.get(row["recipe_id"], row["recipe_id"])
        validate_receipt_shape(receipt)
        for value in [None,False,0,"",[],{"body":receipt["body"]},dict(receipt,extra=True)]:
            with self.assertRaises(ValueError):validate_receipt_shape(value)
        receipt["body"]["cases"][0]["work"]["charged"]=True
        with self.assertRaises(ValueError):validate_receipt_shape(receipt)


RECEIVED = ROOT / "fixtures/product_preview/source_blocks"


def read_received(path):
    # Parse raw bytes directly. Preserve bare -0 as well as -0.0; never route
    # received artifacts through a canonical serializer before checking bits.
    return json.loads(path.read_text(), parse_int=lambda token: -0.0 if token == "-0" else int(token))


def received_artifacts():
    for case in ("n05", "n06", "multicase"):
        for mode in ("dense_scrutiny", "sparse_interactive"):
            name = f"{case}-{mode}"
            yield name, read_received(RECEIVED / f"{name}.raw.json"), {
                "request": read_received(RECEIVED / f"{name}.request.json"), "solver_mode": mode,
            }


class ReceivedSourceBlockArtifacts(unittest.TestCase):
    def test_received_real_artifacts_qualify_only_with_matching_invocation(self):
        negative_zeros = 0
        generation = json.loads((RECEIVED / "generation.json").read_text())
        records = {record["path"]:record["sha256"] for record in generation["files"]}
        for name,source,context in received_artifacts():
            with self.subTest(artifact=name):
                # Validate the retained received artifact; no reseal/fresh solve.
                self.assertEqual(hashlib.sha256((RECEIVED / f"{name}.raw.json").read_bytes()).hexdigest(), records[name+".raw.json"])
                self.assertEqual(hashlib.sha256((RECEIVED / f"{name}.request.json").read_bytes()).hexdigest(), records[name+".request.json"])
                bases = [{"ref_type":"load_case","ref_id":case["id"]} for case in context["request"]["model"]["load_cases"]]
                self.assertTrue(validate_source_blocks(source,context))
                self.assertEqual(numerical_use_standing(source,bases,context),"numerically_eligible")
                self.assertEqual(numerical_use_standing(source,bases),"needs_recompute")
                self.assertFalse(validate_source_blocks(source))
                changed = deepcopy(context); changed["request"]["received_context_mutation"] = None
                with self.assertRaises(ValueError): validate_source_blocks(source,changed)
                changed = deepcopy(context); changed["solver_mode"] = "sparse_interactive" if context["solver_mode"]=="dense_scrutiny" else "dense_scrutiny"
                with self.assertRaises(ValueError): validate_source_blocks(source,changed)
                negative_zeros += sum(row["value"]==0 and math.copysign(1.0,row["value"])<0 for row in source["results"])
        self.assertGreater(negative_zeros,0)
        self.assertEqual(struct.pack(">d",json.loads("-0",parse_int=lambda token:-0.0 if token=="-0" else int(token))).hex(),"8000000000000000")

    def test_received_work_ledgers_and_rehashed_tamper_controls(self):
        for name,source,context in received_artifacts():
            with self.subTest(artifact=name):
                body=source["source_block_recovery"]["body"];work=body["invocation_work"]
                cases_charged=sum(case["work"]["charged"]+case["work"]["reserved_unobserved_failure"] for case in body["cases"])
                self.assertEqual(work["charged"],cases_charged+work["publication_charged"])
                self.assertGreater(work["publication_charged"],0)
                self.assertLessEqual(work["charged"],work["limit"]);self.assertLessEqual(work["limit"],64_000_000)
                self.assertTrue(all(case["work"]["limit"]<=4_000_000 for case in body["cases"]))
                for field in ("charged","publication_charged","limit"):
                    changed=deepcopy(source);ledger=changed["source_block_recovery"]["body"]["invocation_work"]
                    ledger[field]=64_000_001 if field=="limit" else ledger[field]+1
                    seal(changed,context)
                    with self.assertRaises(ValueError):validate_source_blocks(changed,context)
                changed=deepcopy(source);changed["source_block_recovery"]["body"]["cases"][0]["work"]["limit"]=4_000_001;seal(changed,context)
                with self.assertRaises(ValueError):validate_source_blocks(changed,context)

    def test_reserved_failure_work_is_included_in_invocation_sum(self):
        source,context=control();body=source["source_block_recovery"]["body"];case=body["cases"][0]
        case.update(outcome="failed",selected_method=None,source=None,projections=[],supports=[],failure={"stage":"exact_solve","code":"budget","diagnostic_ref":"synthetic-report","block_order":None})
        for row in case["rows"]:row.update(treatment="inspection_only",projection_id=None,recipe_id=None,input_result_ids=[])
        case["work"]["reserved_unobserved_failure"]=7;body["invocation_work"]["charged"]+=7;body["status"]="unavailable";seal(source,context)
        self.assertFalse(validate_source_blocks(source,context))
        body["invocation_work"]["charged"]-=7;seal(source,context)
        with self.assertRaisesRegex(ValueError,"INVOCATION_WORK_LEDGER"):validate_source_blocks(source,context)


class ScaledNormRecipe(unittest.TestCase):
    def check(self, values, output, support=False):
        kinds=("global_nodal_displacement_x","global_nodal_displacement_y","global_nodal_displacement_z")
        components=("Fx","Fy","Fz")
        inputs=[{"kind":"support_reaction_component_v2" if support else kinds[i],"metadata":{"component":components[i]},"entity_ref":"norm-owner","value":v} for i,v in enumerate(values)]
        row={"kind":"reaction_resultant" if support else "displacement_magnitude","unit":"N" if support else "mm","entity_ref":"norm-owner","value":output}
        return _derived("support_force_norm_scaled_v1" if support else "translation_norm_scaled_v1",row,inputs)

    def test_normal_output_and_small_normalized_underflows_are_supported(self):
        for support in (False,True):
            self.check([3.0,4.0,0.0],5.0,support)
            small=math.ldexp(1.0,-600);self.check([small,0.0,0.0],small,support)
            self.check([sys.float_info.min,0.0,0.0],sys.float_info.min,support)
            self.check([1.0,math.ulp(0.0),0.0],1.0,support)
            self.check([-0.0,0.0,-0.0],0.0,support)

    def test_range_bit_identity_and_order_are_enforced(self):
        for support in (False,True):
            with self.assertRaisesRegex(ValueError,"DERIVED_NORM_RANGE"):self.check([math.ulp(0.0),0.0,0.0],math.ulp(0.0),support)
            with self.assertRaisesRegex(ValueError,"DERIVED_NORM_RANGE"):self.check([sys.float_info.max,sys.float_info.max,0.0],1.0,support)
            with self.assertRaisesRegex(ValueError,"DERIVED_NORM_VALUE"):self.check([1.0,0.0,0.0],math.nextafter(1.0,math.inf),support)
            with self.assertRaisesRegex(ValueError,"DERIVED_NORM_VALUE"):self.check([-0.0,0.0,-0.0],-0.0,support)
            with self.assertRaises(ValueError):self.check([math.nan,0.0,0.0],0.0,support)
        source,context=control();case=source["source_block_recovery"]["body"]["cases"][0]
        row=next(row for row in case["rows"] if row["recipe_id"]=="translation_norm_scaled_v1")
        row["input_result_ids"].reverse();seal(source,context)
        with self.assertRaisesRegex(ValueError,"TRANSLATION_NORM_INPUTS"):validate_source_blocks(source,context)

    def test_stress_containing_case_reserves_both_recipe_layers(self):
        source,context=control();case=source["source_block_recovery"]["body"]["cases"][0]
        norm=next(row for row in case["rows"] if row["recipe_id"]=="translation_norm_scaled_v1")
        x_id=norm["input_result_ids"][0]
        x=next(row for row in source["results"] if row["id"]==x_id);x["value"]=1.0
        projection=next(p for p in case["projections"] if p["result_id"]==x_id)
        projection.update(value=1.0,value_bits="3ff0000000000000",interval=[1.0-1e-12,1.0+1e-12],absolute_error_bound=1.1e-12,relative_error_bound=STRESS_INPUT_RELATIVE_LIMIT,basis="outward_interval")
        output=next(row for row in source["results"] if row["id"]==norm["result_id"]);output["value"]=1.0
        source["summary"]["max_displacement"].update(value=1.0,result_ref=output["id"],location_ref=output["entity_ref"])
        seal(source,context);self.assertTrue(validate_source_blocks(source,context))
        projection["relative_error_bound"]=math.nextafter(STRESS_INPUT_RELATIVE_LIMIT,math.inf);seal(source,context)
        with self.assertRaisesRegex(ValueError,"STRESS_TOTAL_RELATIVE_BOUND"):validate_source_blocks(source,context)


class StressNormalRangeAdmission(unittest.TestCase):
    def scalar(self, kind, action_value, output, location="midspan"):
        action={"kind":STRESS_ACTION_KINDS[kind],"unit":"N" if kind=="element_local_axial_normal_stress" else "N*m","entity_ref":"stress-owner","metadata":{"location":location},"value":action_value}
        components={"element_local_axial_normal_stress":"axial_normal_stress","element_local_bending_normal_stress_y":"bending_normal_stress_y","element_local_bending_normal_stress_z":"bending_normal_stress_z","element_local_torsional_shear_stress":"torsional_shear_stress"}
        row={"kind":kind,"unit":"MPa","entity_ref":"stress-owner","metadata":{"location":location,"component":components[kind]},"value":output}
        return _derived("straight_open_stress_v1",row,[action])

    def summary(self, normal, torsion, output):
        inputs=[{"kind":kind,"unit":"MPa","entity_ref":"stress-owner","metadata":{"location":location},"value":torsion if kind=="element_local_torsional_shear_stress" else normal} for location in STRESS_LOCATIONS for kind in STRESS_ACTION_KINDS]
        return _derived("reviewed_stress_summary_v1",{"kind":"open_formula_stress_summary","unit":"MPa","entity_ref":"stress-owner","value":output},inputs)

    def test_normal_signed_and_exact_zero_scalar_controls(self):
        for kind in STRESS_ACTION_KINDS:
            self.scalar(kind,1.0,1e-6)
            self.scalar(kind,-1.0,-1e-6)
            self.scalar(kind,-1.0,1e-6,"end_i")
            self.scalar(kind,0.0,0.0)
            self.scalar(kind,0.0,-0.0)
            with self.assertRaisesRegex(ValueError,"STRESS_ACTION_SIGN"):self.scalar(kind,1.0,-1e-6)

    def test_nonzero_action_cannot_publish_subnormal_or_underflowed_stress(self):
        for kind in STRESS_ACTION_KINDS:
            with self.assertRaisesRegex(ValueError,"STRESS_OUTPUT_RANGE"):self.scalar(kind,1e-306,1.675317e-318)
            with self.assertRaisesRegex(ValueError,"STRESS_OUTPUT_RANGE"):self.scalar(kind,1e-306,0.0)
            with self.assertRaisesRegex(ValueError,"STRESS_OUTPUT_RANGE"):self.scalar(kind,0.0,1e-6)
            with self.assertRaisesRegex(ValueError,"STRESS_PA_OBSERVATION_RANGE"):self.scalar(kind,1.0,sys.float_info.max)

    def test_summary_zero_excludes_torsion_and_normal_output_is_required(self):
        self.summary(0.0,1.0,0.0)
        self.summary(0.0,0.0,-0.0)
        self.summary(1.0,0.0,3.0)
        with self.assertRaisesRegex(ValueError,"SUMMARY_OUTPUT_RANGE"):self.summary(1.0,0.0,0.0)
        with self.assertRaisesRegex(ValueError,"SUMMARY_OUTPUT_RANGE"):self.summary(1.0,0.0,1.675317e-318)
        with self.assertRaisesRegex(ValueError,"SUMMARY_OUTPUT_RANGE"):self.summary(0.0,1.0,1.0)
        with self.assertRaisesRegex(ValueError,"SUMMARY_INPUT_RANGE"):self.summary(1.675317e-318,0.0,0.0)
        with self.assertRaisesRegex(ValueError,"SUMMARY_SUBTOTAL_RANGE"):self.summary(1e302,0.0,1.0)

    def test_rehashed_received_stress_corruption_is_refused(self):
        name,source,context=next(received_artifacts())
        index=next(i for i,row in enumerate(source["results"]) if row["kind"]=="element_local_torsional_shear_stress" and row["value"]!=0)
        for value in (0.0,1.675317e-318):
            changed=deepcopy(source);changed["results"][index]["value"]=value;seal(changed,context)
            with self.assertRaisesRegex(ValueError,"STRESS_OUTPUT_RANGE|SUMMARY_INPUT_RANGE"):validate_source_blocks(changed,context)
        changed=deepcopy(source);projection=next(p for p in changed["source_block_recovery"]["body"]["cases"][0]["projections"] if p["basis"]=="outward_interval")
        projection["relative_error_bound"]=math.nextafter(STRESS_INPUT_RELATIVE_LIMIT,math.inf);seal(changed,context)
        with self.assertRaisesRegex(ValueError,"STRESS_TOTAL_RELATIVE_BOUND"):validate_source_blocks(changed,context)


class ReceivedRejectedStressRange(unittest.TestCase):
    def test_actual_pre_repair_packets_are_refused_without_resealing(self):
        directory=RECEIVED / "rejected_stress_range"
        custody=read_received(directory / "CAPTURE.json")
        hashes={entry["path"]:entry["sha256"] for entry in custody["files"]}
        for mode in ("dense_scrutiny","sparse_interactive"):
            with self.subTest(mode=mode):
                raw_path=directory / f"{mode}.raw.json";request_path=directory / f"{mode}.request.json"
                self.assertEqual(hashlib.sha256(raw_path.read_bytes()).hexdigest(),hashes[raw_path.name])
                self.assertEqual(hashlib.sha256(request_path.read_bytes()).hexdigest(),hashes[request_path.name])
                source=read_received(raw_path);context={"request":read_received(request_path),"solver_mode":mode}
                self.assertEqual(source["source_block_recovery"]["body"]["status"],"qualified")
                # No injected row/receipt/hash: these are the actual recorded bad outputs.
                with self.assertRaisesRegex(ValueError,"STRESS_OUTPUT_RANGE|SUMMARY_INPUT_RANGE"):validate_source_blocks(source,context)
                bases=[{"ref_type":"load_case","ref_id":case["id"]} for case in context["request"]["model"]["load_cases"]]
                self.assertNotEqual(numerical_use_standing(source,bases,context),"numerically_eligible")


if __name__ == "__main__": unittest.main()
