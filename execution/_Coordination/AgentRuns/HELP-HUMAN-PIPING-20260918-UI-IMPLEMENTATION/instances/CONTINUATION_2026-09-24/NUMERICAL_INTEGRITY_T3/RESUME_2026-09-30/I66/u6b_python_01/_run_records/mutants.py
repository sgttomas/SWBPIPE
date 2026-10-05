#!/usr/bin/env python3
"""I66 U6b Python carrier mutants (scratch lane only). Each mutant replaces one
exact, unique snippet of the candidate compatibility.py or records.py in
WT/scratch/i66_u6b_python/mut, checks that the mutant compiles, runs the U6b
tests plus the AnalysisRun compatibility suite, and restores the file. A mutant
is killed only by a failing test; a syntax error is reported, never a kill."""
import json, py_compile, subprocess, sys, time
WT = "WT"
S = f"{WT}/scratch/i66_u6b_python"
CAND = f"{WT}/f2a-carriers/projects/chirality-piping"
MUT = f"{S}/mut/projects/chirality-piping"
VENV = "REPO_ROOT/projects/chirality-piping/.venv"
CO, RE = "core/analysis_runs/compatibility.py", "core/analysis_runs/records.py"
TESTS = "tests/test_retained_precision_carriers.py tests/test_analysis_run_compatibility.py"
OLD_SET = "{PRECISION_CONTRACT_ID, PHYSICS_CONTRACT_ID, SOURCE_BLOCKS_CONTRACT_ID, PHYSICS_SOURCE_CONTRACT_ID, PREVIEW_PHYSICS_CONTRACT_ID, LOAD_REFERENCE_CONTRACT_ID, LOAD_REFERENCE_SOURCE_CONTRACT_ID}"

M = [
    # Dispatch and downgrade guards
    ("D01_no_successor_dispatch", CO, "    if _is_retained(source):\n        return _retained_contract(source, check_receipt=check_receipt)\n", ""),
    ("D02_g7_code_not_detail", CO, "raise ValueError(error.detail or error.code) from error", "raise ValueError(error.code) from error"),
    ("D03_detail_only", CO, "raise ValueError(error.detail or error.code) from error", "raise ValueError(str(error.detail)) from error"),
    ("D04_transport_admitted_unchecked", CO, "    if not check_receipt:\n        # The Python", "    if False:\n        # The Python"),
    ("D05_transport_admitted_checked", CO, "        raise ValueError(\"SOURCE_PRODUCER_CONTRACT_UNSUPPORTED\")\n    _retained_validation(source)", "        pass\n    _retained_validation(source)"),
    ("D06_no_reader_on_dispatch", CO, "        raise ValueError(\"SOURCE_PRODUCER_CONTRACT_UNSUPPORTED\")\n    _retained_validation(source)\n", "        raise ValueError(\"SOURCE_PRODUCER_CONTRACT_UNSUPPORTED\")\n"),
    ("D07_base_sha_returned", CO, "    return PREVIEW_PHYSICS_RETAINED_CONTRACT_ID, PREVIEW_PHYSICS_RETAINED_CONTRACT_SHA256,", "    return PREVIEW_PHYSICS_RETAINED_CONTRACT_ID, PREVIEW_PHYSICS_CONTRACT_SHA256,"),
    ("D08_base_path_returned", CO, "PREVIEW_PHYSICS_RETAINED_CONTRACT_SHA256, _PREVIEW_PHYSICS_RETAINED_CONTRACT_PATH\n", "PREVIEW_PHYSICS_RETAINED_CONTRACT_SHA256, _PREVIEW_PHYSICS_CONTRACT_PATH\n"),
    ("D09_no_member_guard", CO, "    if isinstance(source, Mapping) and \"retained_precision\" in source:\n        raise ValueError(RETAINED_PRECISION_DOWNGRADE_FORBIDDEN)\n", ""),
    ("D10_member_guard_truthy", CO, "    if isinstance(source, Mapping) and \"retained_precision\" in source:", "    if isinstance(source, Mapping) and source.get(\"retained_precision\"):"),
    ("D11_no_token_guard", CO, "    if check_receipt and isinstance(source.get(\"results\"), list) and any(isinstance(row, Mapping) and row.get(\"recovery_method\") == RETAINED_METHOD for row in source[\"results\"]):\n        raise ValueError(RETAINED_PRECISION_DOWNGRADE_FORBIDDEN)\n", ""),
    ("D12_token_guard_ungated", CO, "    if check_receipt and isinstance(source.get(\"results\"), list) and any(", "    if isinstance(source.get(\"results\"), list) and any("),
    ("D13_is_retained_profile", CO, "producer.get(\"semantic_contract_id\") == PREVIEW_PHYSICS_RETAINED_CONTRACT_ID\n", "producer.get(\"semantic_contract_id\") in {PREVIEW_PHYSICS_RETAINED_CONTRACT_ID, PREVIEW_PHYSICS_CONTRACT_ID}\n"),
    # Fresh and current-record sets
    ("F01_not_fresh", CO, "                                # U6b (D-U6-6): membership is not standing.\n                                PREVIEW_PHYSICS_RETAINED_CONTRACT_ID})", "                                })"),
    ("F02_not_current_record", CO, "                                         PREVIEW_PHYSICS_RETAINED_CONTRACT_ID})", "                                         })"),
    ("F03_builder_old_set", CO, "        if contract_id not in CURRENT_RECORD_CONTRACT_IDS:\n            raise ValueError(\"ANALYSIS_SOURCE_CONTRACT_VERSION_MISMATCH\")", f"        if contract_id not in {OLD_SET}:\n            raise ValueError(\"ANALYSIS_SOURCE_CONTRACT_VERSION_MISMATCH\")"),
    ("F04_router_old_set", CO, "if contract in CURRENT_RECORD_CONTRACT_IDS else build_analysis_run_v0_2", f"if contract in {OLD_SET} else build_analysis_run_v0_2"),
    ("F05_validator_old_set", CO, "    if contract_id not in CURRENT_RECORD_CONTRACT_IDS or envelope.get", f"    if contract_id not in {OLD_SET} or envelope.get"),
    # Standing
    ("S01_no_standing_branch", CO, "    if _is_retained(source):\n        # Validated once, with the invocation;", "    if False:\n        # Validated once, with the invocation;"),
    ("S02_standing_ignores_invocation", CO, "            validation = _retained_validation(source, source_block_context)", "            validation = _retained_validation(source)"),
    ("S03_reader_error_needs_recompute", CO, "        except (ValueError, KeyError, TypeError, AttributeError):\n            return \"unsupported\"\n        return _retained_standing_from", "        except (ValueError, KeyError, TypeError, AttributeError):\n            return \"needs_recompute\"\n        return _retained_standing_from"),
    ("S04_quality_routes", CO, "    if contract not in {PHYSICS_CONTRACT_ID, PREVIEW_PHYSICS_CONTRACT_ID, LOAD_REFERENCE_CONTRACT_ID}:", "    if contract not in {PHYSICS_CONTRACT_ID, PREVIEW_PHYSICS_CONTRACT_ID, LOAD_REFERENCE_CONTRACT_ID, PREVIEW_PHYSICS_RETAINED_CONTRACT_ID}:"),
    ("S12_routed_to_quality_branches", CO, "    if _is_retained(source):\n        # Validated once, with the invocation;", "    if False:\n        # Validated once, with the invocation;",
     ("    if contract not in {PHYSICS_CONTRACT_ID, PREVIEW_PHYSICS_CONTRACT_ID, LOAD_REFERENCE_CONTRACT_ID}:", "    if contract not in {PHYSICS_CONTRACT_ID, PREVIEW_PHYSICS_CONTRACT_ID, LOAD_REFERENCE_CONTRACT_ID, PREVIEW_PHYSICS_RETAINED_CONTRACT_ID}:")),
    ("S05_no_invocation_bound", CO, "    if (not validation[\"invocation_bound\"] or not validation", "    if (not validation"),
    ("S06_no_eligibility", CO, "or not validation[\"numerical_eligible\"] or list(", "or list("),
    ("S07_no_refs", CO, " or list(requested_basis_refs) != expected\n", "\n"),
    ("S08_refs_prefix", CO, " or list(requested_basis_refs) != expected\n", " or list(requested_basis_refs)[:len(expected)] != expected\n"),
    ("S09_no_mechanics", CO, "            or source[\"status\"][\"mechanics\"] != \"MECHANICS_SOLVED\" or not _not", "            or not _not"),
    ("S10_no_not_required", CO, " or not _not_required_cases_ordinarily_eligible(source, cases)):", "):"),
    ("S11_quality_contributes", CO, " or not _not_required_cases_ordinarily_eligible(source, cases)):", " or not _not_required_cases_ordinarily_eligible(source, cases) or source[\"numerical_quality\"][\"status\"] != \"checks_passed\"):"),
    ("N01_unavailable_as_not_required", CO, "        if status != \"not_required\" or index >= len(quality)", "        if status not in {\"not_required\", \"unavailable\"} or index >= len(quality)"),
    ("N02_no_duplicate_check", CO, "if not isinstance(item_id, str) or not item_id or item_id in ids:\n                return False", "if not isinstance(item_id, str) or not item_id:\n                return False"),
    ("N03_no_basis_check", CO, "        if not (q.get(\"basis_ref\") == case.get(\"basis_ref\") and q.get(\"solve_quality\")", "        if not (q.get(\"solve_quality\")"),
    ("N04_no_solve_quality", CO, " and q.get(\"solve_quality\") == \"checks_passed\"\n", "\n"),
    ("N05_no_structural", CO, "                and q.get(\"structural_status\") == \"passive_model_basis\" and q.get(", "                and q.get("),
    ("N06_no_fidelity", CO, " and q.get(\"model_matrix_fidelity\") == \"represented_equations_retained\"\n", "\n"),
    ("N07_accuracy_unresolved", CO, "q.get(\"accuracy_evidence\") in {\"not_claimed\", \"reference_verified\"}", "q.get(\"accuracy_evidence\") in {\"not_claimed\", \"reference_verified\", \"unresolved\"}"),
    ("N08_refs_may_be_empty", CO, "and isinstance(refs, list) and refs and all(", "and isinstance(refs, list) and all("),
    ("N09_refs_unresolved", CO, "all(isinstance(ref, str) and ref in ids for ref in refs)", "all(isinstance(ref, str) for ref in refs)"),
    ("N10_short_quality_unchecked", CO, " or index >= len(quality) or not isinstance(quality[index], Mapping):", " or not isinstance(quality[index], Mapping):"),
    # Binding and summary
    ("B01_codes_swapped", CO, "{\"absolute_verified\": RULE_QUANTITY_BELOW_VERIFIED_FLOOR, \"not_covered\": RULE_QUANTITY_NOT_COVERED}", "{\"absolute_verified\": RULE_QUANTITY_NOT_COVERED, \"not_covered\": RULE_QUANTITY_BELOW_VERIFIED_FLOOR}"),
    ("B02_not_covered_binds", CO, "{\"absolute_verified\": RULE_QUANTITY_BELOW_VERIFIED_FLOOR, \"not_covered\": RULE_QUANTITY_NOT_COVERED}", "{\"absolute_verified\": RULE_QUANTITY_BELOW_VERIFIED_FLOOR}"),
    ("B03_broken_binds", CO, "    except (ValueError, KeyError, TypeError, AttributeError):\n        return RULE_QUANTITY_NOT_COVERED", "    except (ValueError, KeyError, TypeError, AttributeError):\n        return None"),
    ("B04_unmatched_refused", CO, "return _class_binding_refusal(match[\"class\"]) if match is not None else None", "return _class_binding_refusal(match[\"class\"]) if match is not None else RULE_QUANTITY_NOT_COVERED"),
    ("B05_no_binding_branch", CO, "    if _is_retained(envelope):\n        return _retained_binding_refusal(envelope, row)\n", ""),
    ("C01_withheld_swapped", CO, "withheld = n[\"absolute_verified\"] + n[\"not_covered\"] if current else", "withheld = n[\"absolute_verified\"] if current else"),
    ("C02_never_current", CO, "    current = _retained_standing_from(validation, source, requested) == \"numerically_eligible\"", "    current = False"),
    ("C03_always_current", CO, "    current = _retained_standing_from(validation, source, requested) == \"numerically_eligible\"", "    current = validation[\"numerical_eligible\"]"),
    ("C04_not_per_case", CO, "            if c[\"basis_ref\"][\"ref_id\"] == case_id:\n", "            if True:\n"),
    ("C05_withheld_held_omits_input", CO, "else n[\"relative_verified\"] + n[\"absolute_verified\"] + n[\"not_covered\"] + n[\"input_derived\"]", "else n[\"relative_verified\"] + n[\"absolute_verified\"] + n[\"not_covered\"]"),
    ("C06_summary_on_broken", CO, "        validation = _retained_validation(source, invocation)\n    except (ValueError, KeyError, TypeError, AttributeError):\n        return []", "        validation = _retained_validation(source, invocation)\n    except (ValueError, KeyError, TypeError, AttributeError):\n        return [{}]"),
    # AnalysisRun
    ("A01_receipt_not_copied", CO, "    if contract_id == PREVIEW_PHYSICS_RETAINED_CONTRACT_ID:\n        envelope[\"analysis_run\"][\"retained_precision\"] = deepcopy(received[\"retained_precision\"])\n", ""),
    ("A02_receipt_body_only", CO, "envelope[\"analysis_run\"][\"retained_precision\"] = deepcopy(received[\"retained_precision\"])", "envelope[\"analysis_run\"][\"retained_precision\"] = {\"body\": deepcopy(received[\"retained_precision\"][\"body\"]), \"receipt_sha256\": \"0\" * 64}"),
    ("A03_no_equality", CO, "        if run.get(\"retained_precision\") != source[\"retained_precision\"]:\n            raise ValueError(ANALYSIS_RETAINED_PRECISION_RECEIPT_MISMATCH)\n", "        pass\n"),
    ("A04_equality_body_only", CO, "        if run.get(\"retained_precision\") != source[\"retained_precision\"]:", "        if (run.get(\"retained_precision\") or {}).get(\"body\") != source[\"retained_precision\"][\"body\"]:"),
    ("A05_no_downgrade_check", CO, "    elif \"retained_precision\" in run:\n        raise ValueError(ANALYSIS_RETAINED_PRECISION_DOWNGRADE_FORBIDDEN)\n", ""),
    ("A06_v02_admits_receipt", CO, "\"contract_evidence\", \"source_block_recovery\", \"retained_precision\")):", "\"contract_evidence\", \"source_block_recovery\")):"),
    ("R01_records_no_guard", RE, "    if isinstance(mechanics_result, Mapping) and \"retained_precision\" in mechanics_result:\n        raise ValueError(\"ANALYSIS_RETAINED_PRECISION_DOWNGRADE_FORBIDDEN\")\n", ""),
    ("R02_records_truthy", RE, "    if isinstance(mechanics_result, Mapping) and \"retained_precision\" in mechanics_result:", "    if isinstance(mechanics_result, Mapping) and mechanics_result.get(\"retained_precision\"):"),
    ("R03_records_drop_receipt", RE, "        raise ValueError(\"ANALYSIS_RETAINED_PRECISION_DOWNGRADE_FORBIDDEN\")\n    result = deepcopy(dict(mechanics_result))", "        mechanics_result = {k: v for k, v in mechanics_result.items() if k != \"retained_precision\"}\n    result = deepcopy(dict(mechanics_result))"),
]


def main(only):
    out = []
    for mid, name, old, new, *more in M:
        if only and mid not in only:
            continue
        if subprocess.run("pgrep -f memguard.sh", shell=True, capture_output=True).returncode != 0:
            print("MEMGUARD NOT RUNNING"); sys.exit(9)
        raw = open(f"{CAND}/{name}").read()
        pairs = [(old, new), *more]
        if any(raw.count(o) != 1 for o, _ in pairs):
            out.append({"id": mid, "file": name, "status": "NOT_APPLIED", "count": [raw.count(o) for o, _ in pairs]}); print(json.dumps(out[-1]), flush=True); continue
        mutated = raw
        for o, n in pairs:
            mutated = mutated.replace(o, n)
        target = f"{MUT}/{name}"
        open(target, "w").write(mutated)
        try:
            py_compile.compile(target, cfile=f"{S}/mut_compile.pyc", doraise=True)
        except py_compile.PyCompileError as error:
            open(target, "w").write(raw)
            out.append({"id": mid, "file": name, "status": "SYNTAX_ERROR", "detail": str(error)[:200]}); print(json.dumps(out[-1]), flush=True); continue
        t = time.time()
        r = subprocess.run(f"cd {MUT} && PYTHONDONTWRITEBYTECODE=1 OPENPIPESTRESS_CHECKED_JSON_BIN={WT}/targets/i52-readers/canonical_json/release/openpipestress_jcs_ijson "
                           f"OPENPIPESTRESS_UNITS_BIN={WT}/targets/i52-readers/units/release/openpipestress_units perl -e 'alarm shift; exec @ARGV' 1200 "
                           f"{VENV}/bin/python -m pytest -q -p no:cacheprovider -x {TESTS}", shell=True, capture_output=True, text=True)
        open(target, "w").write(raw)
        tail = (r.stdout.strip().splitlines() or [""])[-1]
        failed = [line.split(" ")[1] for line in r.stdout.splitlines() if line.startswith("FAILED ")]
        status = "KILLED" if r.returncode == 1 and failed else "SURVIVED" if r.returncode == 0 else f"ERROR_{r.returncode}"
        out.append({"id": mid, "file": name, "status": status, "killed_by": failed[:3], "summary": tail, "seconds": round(time.time() - t, 1)})
        print(json.dumps(out[-1]), flush=True)
    for name in (CO, RE):
        assert open(f"{CAND}/{name}").read() == open(f"{MUT}/{name}").read(), name
    json.dump(out, open(f"{S}/logs/mutants_{sys.argv[1]}.json", "w"), indent=1)


if __name__ == "__main__":
    main(set(sys.argv[2:]))
