"""Source/admission drift, immutable positive transformations and owner closure."""
from copy import deepcopy
import hashlib
import os
from pathlib import Path
import runpy
import shutil
import sys

import pytest

PRICING = Path(__file__).resolve().parents[1]
SOURCE = Path(os.environ.get("B7A_PHASE_TEST_SOURCE", str(PRICING.parents[1])))
MODEL = runpy.run_path(str(PRICING / "base/b7a_domain_model.py"))
RETIRED = runpy.run_path(str(PRICING / "lib/b7a_retired_carry.py"))


def source_copy(destination):
    for file in list(MODEL["SOURCE_FILES"]) + ["core/product_physics/src/retained_memory.rs"]:
        target = destination / file
        target.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(SOURCE / file, target)
    return destination


def test_actual_complete_source_guard_and_native_types_have_positive_owners(monkeypatch):
    monkeypatch.syspath_prepend(str(PRICING / "base"))
    proof = MODEL["inspect"](SOURCE)
    assert proof["formed_terms_per_input"] == 12
    assert proof["manifest_operand_words"] == 26
    assert len(proof["full_raw_source_fingerprints"]) == 21
    # The source limits and three independently retained records determine
    # these lengths; they are not old ABI observations.
    assert MODEL["basis_lengths"](128) == (673, 2701, 2914)
    assert MODEL["formation_children"](12)["1"] >= 12 * 80 * 8
    assert MODEL["manifest_copy"](12)["s(LoadClosureEntry)"] >= 12 * 4


@pytest.mark.parametrize("file", [
    "core/product_physics/src/lib.rs",
    "core/product_physics/src/retained_product.rs",
    "core/product_physics/src/retained_wire.rs",
    "core/reporting/result_export/src/retained_precision.rs",
    "core/solver/straight_pipe/src/lib.rs",
    "core/solver/nonlinear_integration/src/structural_adapter.rs",
    "core/solver/frame_kernel/src/lib.rs",
    "core/reporting/result_export/src/semantic_contract.rs",
    "core/reporting/result_export/src/physics_source.rs",
])
def test_raw_owner_crlf_or_missing_source_refuses(tmp_path, file):
    snapshot = source_copy(tmp_path)
    target = snapshot / file
    raw = target.read_bytes()
    assert b"\r\n" not in raw
    target.write_bytes(raw.replace(b"\n", b"\r\n"))
    with pytest.raises(ValueError, match="owner source changed"):
        MODEL["inspect"](snapshot)
    target.unlink()
    with pytest.raises(FileNotFoundError):
        MODEL["inspect"](snapshot)


@pytest.mark.parametrize("change", ["duplicate_marker", "missing_marker", "early_marker",
                                     "typed_walk", "cap_row", "domain", "law_order", "admit", "admission"])
def test_nongenerated_census_and_late_enforcement_drift_refuses(tmp_path, change):
    snapshot = source_copy(tmp_path)
    target = snapshot / "core/product_physics/src/retained_memory.rs"
    source = target.read_text()
    marker = MODEL["ADMISSION_MARKER"]
    if change == "duplicate_marker":
        source += "\n" + marker + "\n"
    elif change == "missing_marker":
        source = source.replace(marker, "// removed boundary", 1)
    elif change == "early_marker":
        source = marker + "\n" + source.replace(marker, "// removed original boundary", 1)
    else:
        fragment = {"typed_walk": "fn walk(", "cap_row": "fn cap_rows(",
                    "domain": "fn domain_clauses(", "law_order": "fn law_order(",
                    "admit": "fn admit(", "admission": "fn admission("}[change]
        assert fragment in source
        source = source.replace(fragment, fragment + "/* changed source */", 1)
    target.write_text(source)
    with pytest.raises(ValueError):
        MODEL["inspect"](snapshot)


def test_generated_record_bytes_are_not_a_circular_source_premise(tmp_path):
    snapshot = source_copy(tmp_path)
    target = snapshot / "core/product_physics/src/retained_memory.rs"
    source = target.read_text()
    # This is only a guard-dependency control, never a registered build. B7b D3: the
    # registered entry's reviewed-input record (after the admission marker) is the control.
    old = 'reviewed_inputs: "v1;Cargo.lock='
    assert source.count(old) == 1
    target.write_text(source.replace(old, 'reviewed_inputs: "v1;UNREGISTERED=guard-control;Cargo.lock=', 1))
    MODEL["inspect"](snapshot)


def test_reader_identifier_audits_bind_actual_format_sites():
    # Derive coordinates from the real formats: a source move must not leave
    # a seemingly valid audit at a stale line until the full pricing replay.
    relative = "core/reporting/result_export/src/retained_precision.rs"
    lines = (SOURCE / relative).read_text().splitlines()
    _, audits, _ = MODEL["text_decisions"](SOURCE, lambda key: key)
    actual = {key: value for key, value in audits if key.startswith(relative + ":")}
    expected = {}
    for needle, classification in (
        ('format!("captured_requested_mode;', {"mode": {"bytes": 64, "source": "STATIC"}}),
        ('let selected = json!(format!("diagnostic:retained-precision:{}:selected", text(cid)))',
         {"text(cid)": {"bytes": 128, "source": "IN128"}}),
        ('let lead = format!("Load case {}: ", text(cid))',
         {"text(cid)": {"bytes": 128, "source": "IN128"}}),
    ):
        positions = [index + 1 for index, line in enumerate(lines) if needle in line]
        assert len(positions) == 1
        expected[f"{relative}:{positions[0]}"] = classification
    assert actual == expected


def test_prepare_preserves_all_unrelated_positive_rules_and_base_bytes(tmp_path):
    import json
    before = {name: (PRICING / "base" / name).read_bytes()
              for name in ("loop_bounds.g4.json", "text_args.g4.json", "sens.py")}
    inherited, _ = RETIRED["prepare"](SOURCE, PRICING / "base", tmp_path / "retired")
    old_text = json.loads(Path(inherited["text_args.g4.json"]).read_text())
    paths, report = MODEL["prepare"](SOURCE, PRICING / "base", tmp_path / "widened", inherited)
    actual_loop = json.loads(Path(paths["loop_bounds.g4.json"]).read_text())
    expected_loop = json.loads(before["loop_bounds.g4.json"])
    for index, bound in ((116, "cases"), (218, "12*l"), (313, "l")):
        expected_loop["loops"][index]["bound"] = bound
        expected_loop["loops"][index]["why"] = actual_loop["loops"][index]["why"]
    expected_loop["fn_zero"].pop(19)
    expected_loop["edge_zero"].pop(0)
    assert actual_loop == expected_loop
    actual_text = json.loads(Path(paths["text_args.g4.json"]).read_text())
    expected_text = deepcopy(old_text)
    expected_text["id_audit"].pop("core/solver/frame_kernel/src/structural/retained/adaptive.rs:5082")
    expected_text["id_audit"]["core/product_physics/src/lib.rs:10230"]["basis_record"] = {"bytes": 2914, "source": "COMP"}
    old_join = MODEL["BASE_PREIMAGES"]["provenance_join"]
    expected_text["join_rules"][expected_text["join_rules"].index(old_join)] = next(
        x["after"] for x in report["alterations"] if x["family"] == "join_rules")
    expected_text["lex_site_size"]["retained_product.rs:3775"] = actual_text["lex_site_size"]["retained_product.rs:3775"]
    expected_text["args"].insert(0, actual_text["args"][0])
    assert actual_text == expected_text
    assert {name: (PRICING / "base" / name).read_bytes() for name in before} == before
    assert all(hashlib.sha256(Path(paths[name]).read_bytes()).hexdigest() == sha
               for name, sha in report["prepared_files"].items())


@pytest.mark.parametrize("change", ["loop", "record", "join", "copy"])
def test_exact_positive_rule_preimages_refuse_drift(tmp_path, change):
    import json
    base = tmp_path / "base"
    base.mkdir()
    for name in ("loop_bounds.g4.json", "text_args.g4.json", "sens.py"):
        shutil.copyfile(PRICING / "base" / name, base / name)
    inherited, _ = RETIRED["prepare"](SOURCE, base, tmp_path / "retired")
    if change == "loop":
        target = base / "loop_bounds.g4.json"
        target.write_bytes(target.read_bytes() + b"\n")
    else:
        target = Path(inherited["text_args.g4.json"])
        data = json.loads(target.read_text())
        if change == "record":
            data["id_audit"]["core/product_physics/src/lib.rs:10230"]["basis_record"]["bytes"] = 599
        elif change == "join":
            rule = next(r for r in data["join_rules"] if r["re"] == "^provenance_records$")
            rule["bytes"] = "2647"
        else:
            data["lex_site_size"]["retained_product.rs:3775"]["class"] = 0
        target.write_text(json.dumps(data))
    with pytest.raises(ValueError):
        MODEL["prepare"](SOURCE, base, tmp_path / "widened", inherited)


def test_exact_loop_owner_headers_use_the_existing_scanner():
    kd5 = runpy.run_path(str(PRICING / "base/kd5_text_model.py"))
    scanner = runpy.run_path(str(PRICING / "base/loopscan.py"))
    for header, _, owners in MODEL["TEXT_LOOPS"]:
        for owner in owners:
            file, function = owner.rsplit(":", 1)
            # O10 keys use src-relative basenames. Bind the two lib.rs
            # owners to their exact source-qualified raw guards.
            qualified = {
                "lib.rs:add_uniform_element_loads": "core/product_physics/src/lib.rs",
                "lib.rs:exact_scaled_intended": "core/solver/straight_pipe/src/lib.rs",
            }.get(owner)
            candidates = ([qualified] if qualified else
                          [f for f in MODEL["SOURCE_FILES"] if f.endswith("/" + file)])
            assert len(candidates) == 1, owner
            source = (SOURCE / candidates[0]).read_bytes().decode()
            body, _ = kd5["_indented_function"](source, function)
            observed = {h for _, _, h in scanner["loop_spans"](scanner["blank_rust"](body))}
            assert header in observed, (owner, header, observed)

def test_new_uniform_identifier_classes_are_site_specific_input_bounds():
    _, audits, _ = MODEL["text_decisions"](SOURCE, lambda key: key)
    actual = dict(audits)
    pp = "core/product_physics/src/"
    # T4-U6/U8: the same sites on the priced source's unchanged lines (B7b's 10632.. + 359; certified_loads + 3;
    # the final source 588de796a4: lib.rs + 5 more; T4-S5 epoch 4: lib.rs + 314 more; pre-0.4 unification: + 103; W-8: + 2; silent-zero guard: + 1; model document 0.7.0: + 7; W-7: + 37; W-15: + 103; W-16 slices b and c: + 193; W-16 stage 2: + 55; §8.30: + 2 (+ 23 for the two prefix formats), unchanged text; certified_loads' two prefix formats moved into lib.rs's is_generated_equivalent_static_load).
    sites = {
        "lib.rs:12103": {"load_case_id"},
        "lib.rs:12117": {"stable_suffix(load_case_id)", "stable_suffix(&load.load_id)"},
        "lib.rs:12124": {"load.load_id", "bend.pipe_id"},
        "lib.rs:12129": {"load.load_id"},
        "lib.rs:12130": {"bend.pipe_id"},
        "lib.rs:12131": {"bend.component_id"},
        "lib.rs:12132": {"load_case_id"},
        "lib.rs:12191": {"stable_suffix(load_case_id)", "stable_suffix(&load.load_id)"},
        "lib.rs:12198": {"load.load_id", "bend.pipe_id"},
        "lib.rs:12203": {"load.load_id"},
        "lib.rs:12204": {"bend.pipe_id"},
        "lib.rs:12205": {"bend.component_id"},
        "lib.rs:12206": {"load_case_id"},
        "lib.rs:12245": {"stable_suffix(&load.load_id)"},
        "lib.rs:12252": {"load.load_id", "load_case_id"},
        "lib.rs:10669": {"load_case.id"},
        "lib.rs:10670": {"load_case.id"},
    }
    assert sum(map(len, sites.values())) == 22
    for key, args in sites.items():
        assert set(actual[pp + key]) == args
        assert all(value == {"bytes": 128, "source": "IN128"}
                   for value in actual[pp + key].values())

def test_actual_reader_cached_allocations_include_all_live_owners_and_duplicates():
    import json
    from collections import Counter
    kd5 = runpy.run_path(str(PRICING / "base/kd5_text_model.py"))
    paths = MODEL["reader_static_files"](SOURCE, MODEL["STATIC_PREIMAGE"])
    counts = Counter(paths)
    # B7b (A4/A5, O-10): eleven distinct inputs, thirteen actual OnceLock allocations;
    # the pressure-source and ordinary pressure tables are resident, the result schema is
    # not; load_reference's transport schema cache is a surviving owner (D3 review, D2).
    assert len(paths) == 14 and len(counts) == 12
    assert counts["fixtures/results/semantic_contract_v0_3_preview_physics_retained_1.json"] == 2
    assert counts["fixtures/results/semantic_contract_v0_3_physics_retained_1.json"] == 2
    assert counts["fixtures/results/semantic_contract_v0_3_pressure_source_1.json"] == 1
    # T4-S5 (epoch 4): the ordinary pressure table is the -2 contract's (STATIC_OWNERS re-bound to its path).
    assert counts["fixtures/results/semantic_contract_v0_3_pressure_2.json"] == 1
    assert counts["schemas/load_reference_state.schema.json"] == 1
    assert "schemas/results_pressure_1.v0.3.schema.json" not in counts
    for owner, path in MODEL["STATIC_OWNERS"]:
        file, function = owner.rsplit(":", 1)
        raw = (SOURCE / file).read_bytes().decode()
        body, _ = kd5["_indented_function"](raw, function)
        cache = "OnceLock<Result<Value, String>>" if function == "transport_schema" else "OnceLock<Value>"
        assert cache in body and "get_or_init" in body, owner
        assert path in raw
        assert isinstance(json.loads((SOURCE / path).read_bytes()), dict)


def test_every_result_export_cached_owner_is_a_priced_static():
    """B7b D3 review (b7-plan D2): every process-lifetime OnceLock in result_export's source is a listed owner,
    except the one `#[cfg(test)] fn shared_test_corpus`."""
    import re
    kd5 = runpy.run_path(str(PRICING / "base/kd5_text_model.py"))
    found = set()
    for path in sorted((SOURCE / "core/reporting/result_export/src").rglob("*.rs")):
        lines = path.read_text().split("\n")
        for i, line in enumerate(lines):
            if re.search(r"\bstatic\s+\w+\s*:\s*(std::sync::)?OnceLock<", line):
                owner = next(m.group(1) for j in range(i, -1, -1)
                             if (m := re.match(r"\s*(?:pub(?:\([^)]*\))?\s+)?fn\s+(\w+)", lines[j])))
                found.add((str(path.relative_to(SOURCE)), owner))
    owners = {tuple(o.rsplit(":", 1)) for o, _ in MODEL["STATIC_OWNERS"]}
    test_only = {("core/reporting/result_export/src/retained_precision.rs", "shared_test_corpus")}
    raw = (SOURCE / "core/reporting/result_export/src/retained_precision.rs").read_text()
    assert "#[cfg(test)]\nfn shared_test_corpus() -> &'static Value {" in raw
    assert found == owners | test_only
    assert kd5["_indented_function"](raw, "shared_test_corpus")[0]


@pytest.mark.parametrize("change", ["remove", "duplicate", "reorder", "nonresident"])
def test_closed_reader_inventory_preimage_refuses_drift(change):
    inventory = list(MODEL["STATIC_PREIMAGE"])
    if change == "remove":
        inventory.pop(0)
    elif change == "duplicate":
        inventory.append(inventory[0])
    elif change == "reorder":
        inventory[0], inventory[1] = inventory[1], inventory[0]
    else:
        inventory.append("fixtures/results/semantic_contract_v0_3_pressure_source_1.json")
    with pytest.raises(ValueError, match="historical reader static inventory changed"):
        MODEL["reader_static_files"](SOURCE, inventory)


def test_failed_envelope_formulation_text_is_read_from_the_current_literals(tmp_path, monkeypatch):
    """T4-S6 (Codex correction, §8.8): the failed Preview manifest's formulation allowance is the larger of its two
    forms' literal bytes (3,138 B over 9 limitations at T4-S6), not the B7a basis's 2,035 B; a lengthened literal
    raises it and a changed structure stops. W-16 slice a (§8.23): PF's LIMITATIONS[1] drops "and so is a support
    displacement imposed outside a load-state model", 69 B shorter, so the allowance is 3,069 B."""
    monkeypatch.syspath_prepend(str(PRICING / "base"))
    assert MODEL["formulation_content"](SOURCE) == (9, 3069)
    pp, lib = "core/product_physics/src/preview_physics.rs", "core/product_physics/src/lib.rs"
    for file in (pp, lib):
        (tmp_path / file).parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(SOURCE / file, tmp_path / file)
    text = (tmp_path / pp).read_text()
    old = '"Signed support actions for rigid restraints'
    assert text.count(old) == 1
    (tmp_path / pp).write_text(text.replace(old, '"Signed support actions for all rigid restraints'))
    assert MODEL["formulation_content"](tmp_path) == (9, 3073)
    (tmp_path / pp).write_text(text.replace("pub(crate) const LIMITATIONS: [&str; 7] = [", "pub(crate) const LIMITATIONS: [&str; 7]=["))
    with pytest.raises(ValueError):
        MODEL["formulation_content"](tmp_path)
    atoms = {"s(MechanicsEnvelope)": 4096, "s(String)": 24}
    assert (MODEL["failed_envelope_copy"](3, 128, SOURCE).ev(atoms)
            - MODEL["failed_envelope_copy"](3, 128).ev(atoms)) == 3 * (3069 - 2035)
