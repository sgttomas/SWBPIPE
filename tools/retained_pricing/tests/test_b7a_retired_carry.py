"""Source/preimage drift and exact retired-rule preservation controls."""
import importlib.util
import json
import os
from pathlib import Path
import runpy
import shutil
import subprocess
from types import SimpleNamespace

import pytest

PRICING = Path(__file__).resolve().parents[1]
SOURCE = PRICING.parents[1]
MODEL = runpy.run_path(str(PRICING / "lib/b7a_retired_carry.py"))


def test_only_six_retired_tokens_leave_and_all_current_positive_rules_stay(tmp_path):
    before = (PRICING / "base/text_args.g4.json").read_bytes()
    sens_before = (PRICING / "base/sens.py").read_bytes()
    paths, report = MODEL["prepare"](SOURCE, PRICING / "base", tmp_path)
    expected = json.loads(before)
    for key in MODEL["RETIRED_IDS"]:
        expected["id_audit"].pop(key)
    assert json.loads(Path(paths["text_args.g4.json"]).read_text()) == expected
    assert Path(paths["sens.py"]).read_bytes() == sens_before.replace(MODEL["ZW_OLD"].encode(), MODEL["ZW_NEW"].encode(), 1)
    assert (PRICING / "base/text_args.g4.json").read_bytes() == before
    assert (PRICING / "base/sens.py").read_bytes() == sens_before
    assert len(report["retired"]) == 6
    assert report["source_fingerprints"] == MODEL["SOURCE_FILES"]
    assert "finalize_composite" in MODEL["ZW_NEW"] and "finalize_for" in MODEL["ZW_NEW"]


@pytest.mark.parametrize("change", ["source", "rows_crlf", "receipt_crlf", "partial_exact", "partial_finalize", "live_finalizer", "current_owner", "row_rule", "zw_token"])
def test_any_current_source_or_exact_rule_preimage_drift_refuses(tmp_path, change):
    snap, base = tmp_path / "snap", tmp_path / "base"
    for file in MODEL["SOURCE_FILES"]:
        target = snap / file
        target.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(SOURCE / file, target)
    base.mkdir()
    for file in ("text_args.g4.json", "sens.py"):
        shutil.copyfile(PRICING / "base" / file, base / file)
    if change == "source":
        target = snap / MODEL["ROW_FILE"]
        target.write_text(target.read_text() + "\n// source changed\n")
    elif change in ("rows_crlf", "receipt_crlf"):
        file = MODEL["ROW_FILE"] if change == "rows_crlf" else "core/product_physics/src/source_receipt.rs"
        target = snap / file
        raw = target.read_bytes()
        assert b"\r\n" not in raw and b"\n" in raw
        target.write_bytes(raw.replace(b"\n", b"\r\n"))
    elif change in ("partial_exact", "partial_finalize"):
        target = snap / "core/product_physics/src/source_receipt.rs"
        name = change.removeprefix("partial_")
        target.write_text(target.read_text() + "\n    pub(super) fn " + name + "() {}\n")
    elif change == "live_finalizer":
        target = snap / "core/product_physics/src/source_receipt.rs"
        before = target.read_text()
        assert before.count("fn finalize_composite(") == 1
        target.write_text(before.replace("fn finalize_composite(", "fn retired_composite(", 1))
    elif change == "current_owner":
        target = snap / MODEL["ROW_FILE"]
        before = target.read_text()
        owner = ".insert(binding.functional_index, binding.result_id.clone())"
        assert before.count(owner) == 1
        target.write_text(before.replace(owner, ".insert(binding.functional_index, binding.result_id)", 1))
    elif change == "row_rule":
        target = base / "text_args.g4.json"
        rules = json.loads(target.read_text())
        rules["id_audit"][next(iter(MODEL["RETIRED_IDS"]))] = {"wrong": {"bytes": 1}}
        target.write_text(json.dumps(rules))
    else:
        target = base / "sens.py"
        target.write_text(target.read_text().replace(MODEL["ZW_OLD"], MODEL["ZW_NEW"], 1))
    with pytest.raises(ValueError):
        MODEL["prepare"](snap, base, tmp_path / "prepared")

@pytest.mark.parametrize("delegation", ["missing_source", "legacy_constructors"])
def test_delegated_original_rules_still_reject_replaced_key(tmp_path, monkeypatch, delegation):
    """The delegated branch reaches the real strict carry; it grants no exemption."""
    monkeypatch.syspath_prepend(str(PRICING))
    spec = importlib.util.spec_from_file_location("b7a_price_control", PRICING / "price.py")
    runner = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(runner)
    repo = tmp_path / "repo"
    repo.mkdir()
    env = dict(os.environ, GIT_AUTHOR_NAME="t", GIT_AUTHOR_EMAIL="t@example.invalid",
               GIT_COMMITTER_NAME="t", GIT_COMMITTER_EMAIL="t@example.invalid",
               GIT_CONFIG_GLOBAL=os.devnull, GIT_CONFIG_NOSYSTEM="1")

    def git(*args):
        return subprocess.run(["git", "-C", str(repo), *args], env=env, check=True,
                              capture_output=True, text=True).stdout.strip()

    git("init", "-q")
    row = repo / "projects/chirality-piping" / MODEL["ROW_FILE"]
    row.parent.mkdir(parents=True)
    lines = ["fn owner() {"] + ["// unchanged"] * 388 + [
        "    let id = primary[&s.functional_indices[c]].id.clone();", "}"]
    row.write_text("\n".join(lines) + "\n")
    git("add", "-A")
    git("commit", "-q", "-m", "original owned site")
    basis = git("rev-parse", "HEAD")
    lines[389] = "    let id = 0;"
    row.write_text("\n".join(lines) + "\n")
    git("add", "-A")
    git("commit", "-q", "-m", "replaced owned site")
    rev = git("rev-parse", "HEAD")

    snap = tmp_path / "snap"
    for file in MODEL["SOURCE_FILES"]:
        target = snap / file
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_text("// fixture source\n")
    receipt = snap / "core/product_physics/src/source_receipt.rs"
    if delegation == "missing_source":
        receipt.unlink()
    else:
        receipt.write_text("    pub(super) fn exact() {}\n    pub(super) fn finalize() {}\n")

    base = tmp_path / "base"
    base.mkdir()
    key = MODEL["ROW_FILE"] + ":390"
    original = json.dumps({"id_audit": {key: MODEL["RETIRED_IDS"][key]}}).encode()
    (base / "text_args.g4.json").write_bytes(original)
    (base / "crate_dirs.txt").write_text("core/product_physics/src\n")
    monkeypatch.setattr(runner, "BASIS", basis)
    monkeypatch.setattr(runner, "BASE", str(base))
    monkeypatch.setattr(runner, "RULE_FILES", ("text_args.g4.json",))
    monkeypatch.setattr(runner, "snapshot", lambda *args: str(snap))
    original_py = runner.py
    calls = []

    def strict_carry(args, **kwargs):
        calls.append(args)
        return original_py(args, **kwargs)

    monkeypatch.setattr(runner, "py", strict_carry)
    output = tmp_path / "output"
    with pytest.raises(SystemExit) as stopped:
        runner.cmd_run(SimpleNamespace(repo=str(repo), rev=rev, out=str(output), basis=None))
    assert stopped.value.code == 4
    assert len(calls) == 1 and Path(calls[0][0]).name == "linemap_rules.py"
    assert calls[0][-1] == str(base / "text_args.g4.json")
    mapped = json.loads((output / "map/g7_linemap.out.json").read_text())
    assert any(item[-1] == "replaced" and key in str(item) for item in mapped["summary"]["unmapped"])
    assert not (output / "prepared_rules").exists()
    assert (base / "text_args.g4.json").read_bytes() == original

