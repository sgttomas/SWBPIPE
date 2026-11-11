"""tools/serialization/semantic_contract_tables.py: the derived table members are current, and `repin` scopes its
edits (frozen references and historical records are never edited; an old value found only there is refused; the
registered retained-memory profile's reviewed_inputs record is never edited)."""
import importlib.util
import sys
from pathlib import Path

PROJECT = Path(__file__).resolve().parents[1]


def _tool():
    spec = importlib.util.spec_from_file_location("semantic_contract_tables", PROJECT / "tools/serialization/semantic_contract_tables.py")
    module = importlib.util.module_from_spec(spec)
    sys.modules["semantic_contract_tables"] = module
    spec.loader.exec_module(module)
    return module


def test_derived_members_are_current():
    assert _tool().derive(check=True) == 0


def test_repin_never_edits_frozen_or_historical_trees():
    tool = _tool()
    editable, carriers, excluded = tool.classify([
        "core/reporting/result_export/src/semantic_contract.rs",
        "fixtures/results/pressure_load_state_connected_dense.analysis_run.json",
        "validation/references/t4_u5b/SHA256SUMS",
        "execution/PKG-04/DEL-04-07/ScopeOfWork.md",
        "tools/retained_pricing/base/text_args.g4.json",
    ])
    assert editable == ["core/reporting/result_export/src/semantic_contract.rs"]
    assert carriers == ["fixtures/results/pressure_load_state_connected_dense.analysis_run.json"]
    assert excluded == ["validation/references/t4_u5b/SHA256SUMS", "execution/PKG-04/DEL-04-07/ScopeOfWork.md",
                        "tools/retained_pricing/base/text_args.g4.json"]


def test_repin_routes_the_desktop_run_carriers_to_their_writers():
    tool = _tool()
    carriers = [
        "fixtures/persistence/recorded_run_witness.json",
        "fixtures/persistence/recorded_run_witness_rule_packs.json",
        "fixtures/persistence/s2_unresolved_rule_pack_run.json",
        "fixtures/persistence/transient_run_input.json",
        "fixtures/persistence/transient_run_input_rule_packs.json",
    ]
    others = ["fixtures/persistence/invented_persisted_preview_project.json",
              "fixtures/persistence/model_state_record_witness.json"]
    editable, found, excluded = tool.classify(carriers + others)
    assert found == carriers and editable == others and excluded == []
    for carrier in carriers:
        writers = [w for prefix, w in tool.CARRIER_WRITERS.items() if carrier.startswith(prefix)]
        assert writers == [tool.PERSISTENCE_RUN_WRITERS], carrier
        assert carrier in writers[0], carrier
    # The witnesses are emitted from the TypeScript-built inputs, so those writers come first.
    order = tool.PERSISTENCE_RUN_WRITERS
    assert order.index("recordedRunParity") < order.index("analysis_runs_tests")


def _seals_a_run_record(value) -> bool:
    """Whether `value` holds, at any depth, an AnalysisRun record sealed by its own analysis_run_record checksum."""
    if isinstance(value, list):
        return any(_seals_a_run_record(item) for item in value)
    if not isinstance(value, dict):
        return False
    run = value.get("analysis_run")
    hashes = run.get("hashes") if isinstance(run, dict) else None
    if isinstance(hashes, list) and any(isinstance(row, dict) and row.get("payload_scope") == "analysis_run_record" for row in hashes):
        return True
    return any(_seals_a_run_record(item) for item in value.values())


def test_repin_routes_every_self_checksummed_run_record_to_a_writer():
    """A committed AnalysisRun record checksums its own bytes, so a text edit of a digest inside it leaves the
    checksum stale; every file holding one, at the top level or nested (the user-release vectors carry the solve's
    record under each vector), is a carrier (or frozen), never an editable consumer."""
    import json
    import subprocess
    tool = _tool()
    listed = subprocess.run(["git", "grep", "-l", '"analysis_run_record"', "--", "*.json"], cwd=PROJECT,
                            capture_output=True, text=True, check=True).stdout.split()
    records = [path for path in listed if _seals_a_run_record(json.loads((PROJECT / path).read_text(encoding="utf-8")))]
    assert "fixtures/reports/invented/user_release_derivation_vectors.json" in records
    editable, _, _ = tool.classify(records)
    assert editable == []


USER_RELEASE_CARRIERS = (
    "fixtures/reports/invented/user_release_derivation_vectors.json",
    "fixtures/reports/invented/user_release_record.json",
    "fixtures/reports/invented/user_release_runner_refusal_request.json",
    "fixtures/reports/invented/user_release_exact/pressure_2_straight.json",
    "fixtures/reports/invented/user_release_exact/pressure_source_1.json",
)


def test_a_repin_across_a_table_change_leaves_the_user_release_seals_to_their_writers(tmp_path, monkeypatch, capsys):
    """RT-A5 1b: a repin rewrote the preview-physics-2 digest inside the user-release vectors as text, leaving each
    vector's run record sealed by its old checksum (route 010 refused it). On a scratch repository holding the pinned
    files, these carriers and an editable consumer, a repin across a table change edits the consumer, leaves every
    carrier's bytes untouched and names their writers."""
    import os
    import shutil
    import subprocess
    tool = _tool()
    consumer = "core/reporting/result_export/src/semantic_contract.rs"
    for path in {*tool.PINNED, *USER_RELEASE_CARRIERS, consumer}:
        (tmp_path / path).parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(PROJECT / path, tmp_path / path)
    git_env = {**os.environ, "GIT_AUTHOR_NAME": "repin-test", "GIT_AUTHOR_EMAIL": "repin-test@example.invalid",
               "GIT_COMMITTER_NAME": "repin-test", "GIT_COMMITTER_EMAIL": "repin-test@example.invalid"}
    for command in (["init", "-q"], ["add", "-A"], ["commit", "-q", "-m", "scratch"]):
        subprocess.run(["git", *command], cwd=tmp_path, env=git_env, check=True, capture_output=True)
    table = tmp_path / tool.PREVIEW
    old = tool.sha256(table.read_bytes())
    vectors = (tmp_path / USER_RELEASE_CARRIERS[0]).read_text(encoding="utf-8")
    assert old in vectors and old in (tmp_path / consumer).read_text(encoding="utf-8")
    table.write_bytes(table.read_bytes() + b"\n")
    new = tool.sha256(table.read_bytes())
    before = {path: (tmp_path / path).read_bytes() for path in USER_RELEASE_CARRIERS}
    monkeypatch.setattr(tool, "PROJECT", tmp_path)
    assert tool.repin(["HEAD"]) == 0
    out = capsys.readouterr().out
    assert new in (tmp_path / consumer).read_text(encoding="utf-8"), out
    assert {path: (tmp_path / path).read_bytes() for path in USER_RELEASE_CARRIERS} == before
    assert f"{USER_RELEASE_CARRIERS[0]}: carrier; regenerate through its writer" in out
    assert tool.USER_RELEASE_WRITERS in out
    # The writer chain runs in the order its producers require.
    order = tool.USER_RELEASE_WRITERS
    assert (order.index("userReleaseDerivation.parity") < order.index("user_release_record.json")
            < order.index("OPENPIPESTRESS_UPDATE_USER_RELEASE_RUNNER_REQUEST") < order.index("userReleaseExact.parity"))
    for carrier in USER_RELEASE_CARRIERS:
        assert tool.classify([carrier])[1] == [carrier], carrier


C1_CUSTODY_CARRIERS = (
    "apps/desktop/src/test-support/analysis-run-v03/analysis_run_v03_cases.json",
    "apps/desktop/src/test-support/native-reopen/historical_reopen_cases.json",
    "apps/desktop/src/test-support/native-reopen/native_reopen_replies.json",
    "apps/desktop/src/test-support/native-reopen/native_reopen_pre_rt_a4_replies.json",
)


def test_a_c1_custody_carrier_is_its_exact_path_only():
    """C1's four custody carriers are registered by exact path: each is a carrier, and a path extending it (a
    suffixed file or a member beneath it) or a sibling beside it is no carrier, so a self-checksummed record placed
    there is still caught by the routing guard instead of being taken as routed. Main's prefix registrations keep
    matching every path they extend."""
    tool = _tool()
    for carrier in C1_CUSTODY_CARRIERS:
        assert tool.classify([carrier]) == ([], [carrier], []), carrier
        directory, name = carrier.rsplit("/", 1)
        for other in (carrier + ".extra.json", carrier + "x", carrier + "/member.json",
                      f"{directory}/{name.removesuffix('.json')}_extra.json", f"{directory}/sibling_{name}",
                      f"{directory}/sibling.json"):
            assert tool.classify([other]) == ([other], [], []), other
    for prefix_carrier in ("fixtures/reports/invented/user_release_record.json.extra.json",
                           "fixtures/reports/invented/del1005_analysis_record.json.extra.json",
                           "fixtures/results/pressure_load_state_connected_dense.analysis_run.json",
                           "fixtures/persistence/recorded_run_witness_rule_packs.json"):
        assert tool.classify([prefix_carrier]) == ([], [prefix_carrier], []), prefix_carrier


def test_the_c1_custody_carriers_route_to_their_writers_by_equality():
    tool = _tool()
    assert tuple(tool.EXACT_CARRIER_WRITERS) == C1_CUSTODY_CARRIERS
    assert not set(tool.EXACT_CARRIER_WRITERS) & set(tool.CARRIER_WRITERS)
    for carrier in C1_CUSTODY_CARRIERS:
        assert tool.carrier_registrations(carrier) == [(carrier, tool.C1_CUSTODY_WRITERS)], carrier
        assert tool.carrier_registrations(carrier + ".extra.json") == [], carrier
    # Each generator reads the corpus written before it.
    order = tool.C1_CUSTODY_WRITERS
    assert (order.index("analysisRunV03Parity") < order.index("historicalReopenParity")
            < order.index("the_webviews_persistence_replies_are_the_commands_replies")
            < order.index("NATIVE_REOPEN_RECORD") < order.index("stored_custody_tests"))


def test_a_repin_across_a_table_change_leaves_a_c1_custody_carrier_to_its_writers(tmp_path, monkeypatch, capsys):
    """On a scratch repository holding the pinned files, an editable consumer and C1's builder corpus (both bind the
    current preview-physics-2 digest), a repin across that table's change edits the consumer, leaves the corpus's
    bytes untouched and names its exact-path writers."""
    import os
    import shutil
    import subprocess
    tool = _tool()
    carrier = C1_CUSTODY_CARRIERS[0]
    consumer = "core/reporting/result_export/src/semantic_contract.rs"
    for path in {*tool.PINNED, carrier, consumer}:
        (tmp_path / path).parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(PROJECT / path, tmp_path / path)
    git_env = {**os.environ, "GIT_AUTHOR_NAME": "repin-test", "GIT_AUTHOR_EMAIL": "repin-test@example.invalid",
               "GIT_COMMITTER_NAME": "repin-test", "GIT_COMMITTER_EMAIL": "repin-test@example.invalid"}
    for command in (["init", "-q"], ["add", "-A"], ["commit", "-q", "-m", "scratch"]):
        subprocess.run(["git", *command], cwd=tmp_path, env=git_env, check=True, capture_output=True)
    table = tmp_path / tool.PREVIEW
    old = tool.sha256(table.read_bytes())
    assert old in (tmp_path / carrier).read_text(encoding="utf-8") and old in (tmp_path / consumer).read_text(encoding="utf-8")
    table.write_bytes(table.read_bytes() + b"\n")
    new = tool.sha256(table.read_bytes())
    before = (tmp_path / carrier).read_bytes()
    monkeypatch.setattr(tool, "PROJECT", tmp_path)
    assert tool.repin(["HEAD"]) == 0
    out = capsys.readouterr().out
    assert new in (tmp_path / consumer).read_text(encoding="utf-8"), out
    assert (tmp_path / carrier).read_bytes() == before
    assert f"{carrier}: carrier; regenerate through its writer" in out
    assert f"writers for {carrier}: {tool.C1_CUSTODY_WRITERS}" in out


def test_repin_dry_run_at_head_lists_nothing(capsys):
    assert _tool().repin(["HEAD"], dry_run=True) == 0
    assert "0 file edit(s) listed, none written" in capsys.readouterr().out


def test_repin_leaves_the_registered_profile_record_unchanged():
    tool = _tool()
    old, new = "a" * 64, "b" * 64
    text = f'pin: "{old}",\n    reviewed_inputs: "v1;Cargo.lock={old};x={old}",\n    other: "{old}"\n'
    updated, kept = tool.replace_unprotected("core/product_physics/src/retained_memory.rs", text, old, new)
    assert kept == 2
    assert updated == f'pin: "{new}",\n    reviewed_inputs: "v1;Cargo.lock={old};x={old}",\n    other: "{new}"\n'
    assert tool.replace_unprotected("core/product_physics/src/lib.rs", text, old, new) == (text.replace(old, new), 0)
    # The actual record is matched exactly once.
    source = (PROJECT / "core/product_physics/src/retained_memory.rs").read_text(encoding="utf-8")
    assert len(tool.PROTECTED["core/product_physics/src/retained_memory.rs"].findall(source)) == 1


def test_timoshenko_table_is_preview_physics_2_with_its_identity_profile_and_limitation_0():
    """DEL-05-03 Design pf-timoshenko.md §4: the derived table differs from preview-physics-2 in exactly three
    members (identity, profile, limitation 0), member order kept; `derive --check` holds it current."""
    import json
    tool = _tool()
    preview = json.loads((PROJECT / tool.PREVIEW).read_bytes())
    derived = json.loads((PROJECT / tool.TIMOSHENKO).read_bytes())
    assert list(derived) == list(preview)
    assert {k for k in preview if preview[k] != derived[k]} == {
        "semantic_contract_id", "formulation_profile_id", "supported_profile_limitations"}
    assert derived["semantic_contract_id"] == "openpipestress.result_semantics/0.3.0/preview-physics-timoshenko-1"
    assert derived["formulation_profile_id"] == "product_preview_mechanics_timoshenko_v1"
    assert derived["supported_profile_limitations"] == [tool.TIMOSHENKO_LIMITATION_0] + preview["supported_profile_limitations"][1:]
    assert (PROJECT / tool.TIMOSHENKO).read_bytes() == tool.timoshenko_table()
    # The results schema's Timoshenko branch states the same array (apply-u4b keeps it current).
    assert tool.apply_u4b(check=True) == 0
    schema = json.loads((PROJECT / "schemas/results.v0.3.schema.yaml").read_text(encoding="utf-8"))
    branch = next(b for b in schema["$defs"]["ResultEnvelope"]["oneOf"]
                  if b.get("properties", {}).get("producer", {}).get("properties", {}).get("semantic_contract_id", {}).get("const")
                  == derived["semantic_contract_id"])
    assert branch["properties"]["formulation_basis"]["properties"]["profile_id"]["const"] == derived["formulation_profile_id"]
    assert branch["properties"]["formulation_basis"]["properties"]["limitations"]["const"] == derived["supported_profile_limitations"]
