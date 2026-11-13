"""The fixed CAEPIPE sitting pack (DEL-17-05): the committed bytes, and the development builder that makes them.

The pack is committed at validation/caepipe_sitting_pack/ and reviewed byte for byte: pack/ is what the owner copies to
Windows, records/ the Mac-side records (each written model file's identity sidecar and writer's report). These tests
read the committed blobs from Git, check that one digest (the SHA-256 of pack/pack_index.json, recorded in
validation/caepipe_sitting_pack_identity.txt) covers every byte of both folders, and check the builder's rules on the
bytes it makes.
The regeneration test is a consistency check, not what makes the pack trustworthy.
"""

from __future__ import annotations

import functools
import hashlib
import json
from pathlib import Path
import re
import shutil
import subprocess
import sys

import pytest

CI_READS = ("validation/caepipe_sitting_pack/**",)

ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from tools.caepipe_correlation import sitting_pack_build as spb  # noqa: E402

FOLDER = "validation/caepipe_sitting_pack"
IDENTITY = ROOT / "validation" / "caepipe_sitting_pack_identity.txt"
MANIFEST = ROOT / "validation" / "caepipe_benchmark" / "manifest.json"


@functools.lru_cache(maxsize=1)
def _built() -> tuple:
    return tuple(sorted(spb.build_pack().items()))


def built() -> dict[str, bytes]:
    return dict(_built())


def P(name: str) -> str:
    """A path inside the pack folder."""
    return f"{spb.PACK}/{name}"


def index_of(files: dict[str, bytes]) -> dict:
    return json.loads(files[spb.INDEX_PATH])


def by_id(index: dict) -> dict:
    return {item["item_id"]: item for item in index["items"]}


def manifest() -> dict:
    return json.loads(MANIFEST.read_text(encoding="utf-8"))


def git(*args: str, stdin: bytes | None = None) -> bytes:
    return subprocess.run(["git", "-C", str(ROOT), *args], input=stdin, capture_output=True, check=True).stdout


def committed_blobs() -> dict[str, bytes]:
    """{path inside validation/caepipe_sitting_pack: bytes} of HEAD's blobs under validation/caepipe_sitting_pack, read from Git's objects,
    not from the working files."""
    listing = git("ls-tree", "-r", "-z", "HEAD", "--", "validation/caepipe_sitting_pack/")
    entries = []
    for record in listing.split(b"\0"):
        if record:
            meta, _, name = record.partition(b"\t")
            mode, kind, sha = meta.decode().split()
            assert (mode, kind) == ("100644", "blob"), name
            entries.append((name.decode(), sha))
    out = git("cat-file", "--batch", stdin="".join(f"{sha}\n" for _, sha in entries).encode())
    blobs, at = {}, 0
    for name, sha in entries:
        end = out.index(b"\n", at)
        header = out[at:end].decode().split()
        assert header[0] == sha
        size = int(header[2])
        blobs[name[len(FOLDER) + 1:]] = out[end + 1:end + 1 + size]
        at = end + 1 + size + 1
    return blobs


# -- the committed bytes --------------------------------------------------------------------------------------------
def test_one_digest_covers_every_committed_byte():
    """From HEAD's blobs: the two folders hold exactly pack/pack_index.json and the files it names; each has the
    SHA-256, size and folder the index records; and the index's SHA-256 is the recorded identity."""
    blobs = committed_blobs()
    index = json.loads(blobs[spb.INDEX_PATH])
    assert set(blobs) == set(index["files"]) | {spb.INDEX_PATH}
    for name, entry in index["files"].items():
        assert hashlib.sha256(blobs[name]).hexdigest() == entry["sha256"], name
        assert len(blobs[name]) == entry["bytes"], name
        assert name.split("/", 1)[0] == entry["folder"] in (spb.PACK, spb.RECORDS), name
    recorded = git("show", "HEAD:validation/caepipe_sitting_pack_identity.txt").decode().split()[0]
    assert hashlib.sha256(blobs[spb.INDEX_PATH]).hexdigest() == recorded


def test_git_stores_the_folder_without_line_ending_conversion():
    attributes = git("check-attr", "text", "--", f"{FOLDER}/pack/INSTRUCTIONS.md", f"{FOLDER}/pack/pack_index.json",
                     f"{FOLDER}/pack/BATCH_COMMANDS.txt", f"{FOLDER}/records/x.json").decode()
    assert attributes.count("text: unset") == 4, attributes
    # Only that folder: a sibling path beside it, and the identity file, keep the repository's default.
    outside = git("check-attr", "text", "--", "validation/caepipe_sitting_pack_identity.txt",
                  "validation/caepipe_sitting_pack_other/x.md", "validation/caepipe_benchmark/manifest.json").decode()
    assert outside.count("text: unspecified") == 3, outside


def test_regenerating_from_the_tracked_inputs_gives_the_committed_bytes():
    """A CONSISTENCY CHECK, NOT WHAT MAKES THE PACK TRUSTWORTHY: the builder run on the tracked inputs gives the
    committed bytes, byte for byte. The pack is trusted because its committed bytes were read and reviewed; this test
    only shows that the builder and the tracked inputs still agree with them."""
    assert built() == committed_blobs()


# -- what the builder makes ----------------------------------------------------------------------------------------
def test_the_same_inputs_give_the_same_bytes():
    assert spb.build_pack() == built()


def test_the_index_names_every_other_file_with_its_hash_and_size_and_the_builder_prints_the_identity(tmp_path,
                                                                                                    capsys):
    files = built()
    index = index_of(files)
    assert set(index["files"]) == set(files) - {spb.INDEX_PATH}
    for name, entry in index["files"].items():
        assert entry == {"sha256": hashlib.sha256(files[name]).hexdigest(), "bytes": len(files[name]),
                         "folder": name.split("/", 1)[0]}
    assert spb.main([str(tmp_path / "pack")]) == 0
    assert hashlib.sha256(files[spb.INDEX_PATH]).hexdigest() in capsys.readouterr().out
    written = {p.relative_to(tmp_path / "pack").as_posix(): p.read_bytes()
               for p in (tmp_path / "pack").rglob("*") if p.is_file()}
    assert written == files
    assert spb.main([str(tmp_path / "pack")]) == 2  # an existing folder is never written into
    (tmp_path / "empty").mkdir()
    assert spb.main([str(tmp_path / "empty")]) == 2 and list((tmp_path / "empty").iterdir()) == []


def test_the_parts_hold_what_the_plan_names_in_order():
    index = index_of(built())
    assert [(p["id"], p["kind"], len(p["items"])) for p in index["parts"]] == [
        ("part-0", "dry_run", 6), ("part-1", "dynamics", 67), ("part-2", "pressure", 66),
        ("part-3", "static_series", 12), ("part-4", "hand_entry", 5), ("part-5", "tranche1_rest", 57)]
    items = by_id(index)
    m = manifest()
    runs = {run["run_id"]: (entry, run) for entry in m["models"] for run in entry.get("runs") or []}
    part = {p["id"]: p["items"] for p in index["parts"]}
    assert part["part-0"][:3] == ["cpb-g1-05", "cpb-g8-01", "cpb-g6-02"]
    dynamics = [runs[i][1] for i in part["part-1"]]
    assert [(r["strike_order"], r["form"]) for r in dynamics] == sorted(
        (r["strike_order"], r["form"]) for r in dynamics)
    assert {r["strike_order"] for r in dynamics} == set(range(1, 35))
    paired = [r for r in dynamics if r.get("counterpart_run_id")]
    assert [r["run_id"] for r in dynamics if not r.get("counterpart_run_id")] == [
        "cpb-dyn-c-n08-two--srss-srss-table-typed"] and len(paired) == 66
    for a, b in zip(paired[::2], paired[1::2]):
        assert a["counterpart_run_id"] == b["run_id"] and a["form"] == "table_added"
    series = [runs[i][1]["series"] for i in part["part-2"]]
    assert series[:44].count("pressure-bourdon-expansion-capture") == 0
    assert set(series[44:]) == {"pressure-bourdon-expansion-capture"}
    assert all(items[i]["capture"] == "sealed" for i in part["part-2"][44:])
    assert [items[i]["family"] for i in part["part-3"]] == ["S1"] * 4 + ["S2"] * 3 + ["S3"] * 5
    assert [runs[i][0]["hand_entry_rank"] for i in part["part-4"]] == [1, 2, 3, 4, 5]
    assert all(items[i]["tranche"] == "1" for i in part["part-5"])


def test_every_case_file_is_the_writers_output_with_the_declared_hash_and_a_wrong_hash_refuses(tmp_path):
    files = built()
    index = index_of(files)
    m = manifest()
    declared = {run["run_id"]: run["mbf"]["sha256"] for e in m["models"] for run in e.get("runs") or []}
    declared |= {e["model_id"]: e["mbf_sha256"] for e in m["models"] if "mbf_sha256" in e and not e.get("runs")}
    for item in index["items"]:
        data = files[P(f"{item['folder']}/{item['mbf']['file']}")]
        if item["item_id"] in declared:
            assert hashlib.sha256(data).hexdigest() == declared[item["item_id"]], item["item_id"]
    copy = tmp_path / "caepipe_benchmark"
    shutil.copytree(MANIFEST.parent, copy)
    altered = json.loads((copy / "manifest.json").read_text())
    victim = next(e for e in altered["models"] if e.get("runs"))
    victim["runs"][0]["mbf"]["sha256"] = "0" * 64
    (copy / "manifest.json").write_text(json.dumps(altered))
    with pytest.raises(spb.BuildError, match="does not have the SHA-256 the manifest declares"):
        spb.build_pack(copy / "manifest.json")


def test_window_only_cases_never_appear_in_the_command_file_and_every_other_case_does():
    files = built()
    index = index_of(files)
    commands = files[P(spb.BATCH_COMMANDS)].decode()
    lines = commands.split("\r\n")
    m = manifest()
    window_runs = [run["run_id"] for e in m["models"] for run in e.get("runs") or []
                   if run.get("kind") in ("option_addition", "hand_entry")]
    assert len(window_runs) == 72
    for run_id in window_runs:
        assert run_id not in commands, run_id
    for item in index["items"]:
        line = spb.batch_line(item["folder"], item["mbf"]["file"])
        if item["routes"] == [spb.WINDOW]:
            assert item["item_id"] not in commands
        else:
            assert lines.count(line) == 1, item["item_id"]
    assert len(lines) - 1 == 145 and sum(line.startswith("start ") for line in lines) == 139
    assert spb.command_file_problems(files[P(spb.BATCH_COMMANDS)]) == []


def test_a_window_case_carries_the_manifests_instruction_bytes_and_a_mismatch_refuses(tmp_path):
    files = built()
    m = manifest()
    runs = {run["run_id"]: run for e in m["models"] for run in e.get("runs") or []}
    index = index_of(files)
    window = [item for item in index["items"] if item["item_kind"] == "benchmark_case"
              and item["routes"] == [spb.WINDOW]]
    assert len(window) == 72
    instructions = files[P(spb.INSTRUCTIONS)].decode()
    for item in window:
        run = runs[item["item_id"]]
        body = files[P(f"{item['folder']}/{item['window_instruction']['file']}")]
        assert body == run["window_instruction"]["text"].encode("utf-8")
        assert hashlib.sha256(body).hexdigest() == run["window_instruction"]["sha256"]
        assert run["window_instruction"]["text"] not in instructions  # pointed to, not restated
        assert item["results"][spb.WINDOW][:2] == [run["expected_print_file"], run["expected_saved_file"]]
    copy = tmp_path / "caepipe_benchmark"
    shutil.copytree(MANIFEST.parent, copy)
    altered = json.loads((copy / "manifest.json").read_text())
    victim = next(run for e in altered["models"] for run in e.get("runs") or [] if run.get("window_instruction"))
    victim["window_instruction"]["sha256"] = "0" * 64
    (copy / "manifest.json").write_text(json.dumps(altered))
    with pytest.raises(spb.BuildError, match="window instruction's text does not have the SHA-256"):
        spb.build_pack(copy / "manifest.json")


def test_each_window_case_prints_its_hold_sentence_from_the_manifest():
    files = built()
    index = index_of(files)
    m = manifest()
    runs = {run["run_id"]: run for e in m["models"] for run in e.get("runs") or []}
    instructions = files[P(spb.INSTRUCTIONS)].decode()
    for item in index["items"]:
        if item["item_kind"] != "benchmark_case" or item["routes"] != [spb.WINDOW]:
            continue
        run = runs[item["item_id"]]
        entry = next(e for e in m["models"] if e["model_id"] == item["model_id"])
        expected = (sorted(set(entry["held"].values())) if run["kind"] == "hand_entry"
                    else [run["screening_waits_for"]])
        assert item["hold"][:len(expected)] == expected  # each case's own hold, as its manifest entry gives it
        for sentence in item["hold"]:
            label = ("on its one-way support" if sentence == m["hand_entry"]["one_way_cases"]
                     else "how this run's capture is held" if run["kind"] == "hand_entry"
                     else "what the screening of this run waits for")
            assert f'From the benchmark manifest, {label}: "{sentence}"' in instructions
    one_way = [i for i in index["items"] if m["hand_entry"]["one_way_cases"] in i["hold"]]
    assert {i["model_id"] for i in one_way} == {"cpb-n-01", "cpb-n-02", "cpb-n-03", "cpb-n-04"}


def test_the_qualification_is_one_text_in_its_file_the_index_and_at_the_top_of_both_documents():
    files = built()
    index = index_of(files)
    text = ("Development comparison only. In any SWBPIPE result, 'checks passed', 'established' and certified bounds "
            "rest on a premise the product does not verify, which is false on a constructed model (DEL-04-01; exit "
            "criterion 18, not met). They are not established engineering results. The mechanics values are compared "
            "with CAEPIPE on that understanding.")
    follow = ("Nothing in this pack is a SWBPIPE result. Where a case is held until the product publishes it as checks "
              "passed, that phrase names the product's own gate and carries this qualification.")
    assert files[P(spb.QUALIFICATION_FILE)] == text.encode("utf-8")
    sha = hashlib.sha256(text.encode("utf-8")).hexdigest()
    assert index["qualification"] == {"file": spb.QUALIFICATION_FILE, "sha256": sha}
    for name in (spb.INSTRUCTIONS, spb.RUN_SHEET):
        document = files[P(name)].decode()
        assert document.startswith(f"{text}\r\n\r\n{follow}\r\n\r\n"), name
        assert document.count(text) == 1, name
    gated = [item for item in index["items"] if any("checks passed" in s for s in item["hold"])]
    assert len(gated) == 5
    for item in index["items"]:
        assert (item.get("qualification") == {"file": spb.QUALIFICATION_FILE, "sha256": sha}) == (item in gated)


def test_the_dry_run_samples_copy_the_smallest_dynamics_models_form_a_file():
    files = built()
    index = index_of(files)
    items = by_id(index)
    m = manifest()
    dyn = [e for e in m["models"] if e["family"].startswith("DYN")]
    smallest = min(dyn, key=lambda e: e["layout_row_count"])
    run = next(r for r in smallest["runs"] if r["form"] == "table_added")
    commands = files[P(spb.BATCH_COMMANDS)].decode()
    for sample_id, route in (("sample-dyn-batch", spb.BATCH), ("sample-dyn-window", spb.WINDOW)):
        item = items[sample_id]
        data = files[P(f"{item['folder']}/{item['mbf']['file']}")]
        assert hashlib.sha256(data).hexdigest() == run["mbf"]["sha256"]
        assert item["routes"] == [route] and item["copy_of"]["run_id"] == run["run_id"]
        assert (sample_id in commands) == (route == spb.BATCH)
    window = items["sample-dyn-window"]
    assert window["dry_run_checks"][1:] == [c for c in m["dry_run"]["checks"] if not c.startswith("(v)")]
    assert len(window["dry_run_checks"]) == 5  # answer C and checks (i) to (iv); check (v) is the export trial's
    sheet = files[P(spb.RUN_SHEET)].decode().replace("\r\n", "\n")
    for line in [*m["dry_run"]["checks"], *m["dry_run"]["consequences"]]:
        assert line in sheet


def test_the_pack_contains_only_its_permitted_kinds():
    files = built()
    for name in files:
        assert spb.kind_problem(name.rsplit("/", 1)[-1], spb.PACK_KINDS) is None, name
    assert spb.KINDS_CLAIM in files[P(spb.INSTRUCTIONS)].decode()


@pytest.mark.parametrize("name, allowed", [
    ("x.txt", True), ("x.TXT", True), ("x.cmd.txt", True), ("x.mbf", True), ("x.json", True), ("x.md", True),
    ("x.txt.cmd", False), ("x", False), ("x.", False), ("x.txt ", False), ("x.txt.", False), ("x.bat", False),
    ("x.cpl", False), ("x.CPL", False), ("", False),
])
def test_the_kind_rule_reads_the_last_extension_only_without_regard_to_letter_case(name, allowed):
    assert (spb.kind_problem(name, spb.PACK_KINDS) is None) == allowed


@pytest.mark.parametrize("seeded", [
    'start "" /wait "%CAEPIPE%" "%CD%\\cases\\x\\x.mbf";batchmode=yes > out.txt',
    'start "" /wait "%CAEPIPE%" "%CD%\\cases\\x\\x.mbf";batchmode=yes 2>&1',
    'start "" /wait "%CAEPIPE%" "%CD%\\cases\\x\\x.mbf";batchmode=yes | more',
    'start "" /wait "%CAEPIPE%" "%CD%\\cases\\x\\x.mbf";batchmode=yes && echo done',
    'start "" /wait "%CAEPIPE%" "%CD%\\cases\\x\\x.mbf";batchmode=yes || exit',
    'if exist x.mbf start "" /wait "%CAEPIPE%" "%CD%\\cases\\x\\x.mbf";batchmode=yes',
    'for %f in (*.mbf) do start "" /wait "%CAEPIPE%" "%f";batchmode=yes',
    'start "" /wait "%CAEPIPE%" "%OTHER%\\cases\\x\\x.mbf";batchmode=yes',
    'set "OTHER=C:\\x"', 'set "CAEPIPE=<x>"', 'cd /d "<x>"',
    'set "CAEPIPE=<the full path of your CAEPIPE program>"\t',
    'copy x.mbf y.mbf', 'del x.csv', 'move x.csv y.csv', 'mkdir out', 'md out', 'rem a comment',
    'start "" "%CAEPIPE%" "%CD%\\cases\\x\\x.mbf";batchmode=yes',
    'start "" /wait "%CAEPIPE%" "%CD%\\cases\\x\\x.mbf";batchmode=yes & del x',
    'start "" /wait "%CAEPIPE%" "%CD%\\..\\x.mbf";batchmode=yes',
    'start "" /wait "%CAEPIPE%" "%CD%\\cases\\x\\x.mbf";batchmode=yes\x1a',
])
def test_the_command_file_check_fails_on_a_seeded_line_of_each_forbidden_kind(seeded):
    good = built()[P(spb.BATCH_COMMANDS)]
    assert spb.command_file_problems(good) == []
    lines = good.decode().split("\r\n")
    lines.insert(len(lines) // 2, seeded)
    assert spb.command_file_problems("\r\n".join(lines).encode()) != []


def test_the_section_titles_are_the_parsers_own_list():
    from core.handoff.caepipe_external import print_csv
    titles = spb.section_titles()
    assert len(titles) == sum(1 for spec in print_csv._SPECS.values() if "headers" in spec)
    instructions = built()[P(spb.INSTRUCTIONS)].decode()
    for title in titles:
        assert f"   - `{title}`" in instructions


# -- the prose -----------------------------------------------------------------------------------------------------
def test_the_prose_rules_hold_in_the_generated_documents():
    """Commands one line at a time (a fenced display of a command holds that one line); no statement that the imported file is never
    saved over, but the instruction not to save over it; no assertion that earlier results are stored; the set
    passage, the three conditions and the claim about kinds word for word; what the pack is not, first."""
    files = built()
    for name in (spb.INSTRUCTIONS, spb.RUN_SHEET, spb.BATCH_COMMANDS):
        text = files[P(name)].decode().replace("\r\n", "\n")
        for block in re.findall(r"```\n(.*?)```", text, flags=re.S):  # a command is shown on its own
            if re.search(r"^(start|set|cd) ", block, flags=re.M):
                assert block.count("\n") == 1, block
        for forbidden in ("never saved over", "ever saved over", "is never saved", "already have stored results",
                          "complete stored results", "Every input in it is invented", "executable",
                          "intake adds a new run", "ingest --check", "--authority", "unopened", "does not read",
                          "case.json"):
            assert forbidden not in text, (name, forbidden)
    instructions = files[P(spb.INSTRUCTIONS)].decode().replace("\r\n", "\n")
    assert "Do not save over the imported file `<stem>.mbf`." in instructions
    for passage in (spb.SET_PASSAGE, spb.SCREEN_WAITS, spb.KINDS_CLAIM):
        assert passage in instructions
    first = instructions.split("# SWBPIPE sitting: instructions\n\n", 1)[1].split("\n\n", 1)[0]
    for words in ("not a SWBPIPE result", "no driver", "no script that runs at the sitting", "SWBPIPE's harness"):
        assert words in first
    generator = manifest()["generator"]
    assert (f"the output of SWBPIPE's benchmark generator (`{generator['module']}` with its tranche modules) through "
            f"SWBPIPE's model-file writer. The first and second tranches' members are drawn from seed "
            f"{generator['seed']}; the third tranche's members are designed, each stated in the generator's own text, "
            "and draw nothing from the seed.") in instructions
    assert "Instruction variant" not in instructions and "variant" not in files[P(spb.RUN_SHEET)].decode()
    assert ("Each case and sample folder holds one small provenance file, `<stem>.provenance.json`, beside its model "
            "file. Do not open it in the program; you need not bring it back.") in instructions
    for words in ("fresh working folder", "not the exchange folder", "answering or closing it is yours to do",
                  "note it on its run line, then run that case the window way (3.2) before pasting the next line", "one line at a time"):
        assert words in instructions


def test_the_builder_holds_no_exchange_intake_or_git_machinery():
    """A tripwire on the builder's own source: it is a development tool that reads tracked inputs and writes a folder;
    nothing of the exchange folder, intake, recognition or Git is in it."""
    source = (ROOT / "tools" / "caepipe_correlation" / "sitting_pack_build.py").read_text()
    for word in ("SWBPIPE_CAEPIPE_EXCHANGE", "class Guard", "class Tree", "resolve_harness_config", "subprocess",
                 "def ingest", "def recognise", "authority", "--development", "exchange_root", "run_pilot", "git "):
        assert word not in source, word


def test_the_pack_folder_holds_only_what_travels_and_the_records_folder_the_rest():
    """pack/ holds the top-level documents and, per item, its model file and its provenance sidecar, its window
    instruction where there is one, and its run line; records/ holds each written model file's identity sidecar and writer's report; there is no
    case.json anywhere."""
    files = built()
    index = index_of(files)
    travel = {P(name) for name in spb.TOP_LEVEL}
    records = set()
    for item in index["items"]:
        travel |= {P(f"{item['folder']}/{item['mbf']['file']}"), P(f"{item['folder']}/run_line.txt"),
                   P(f"{item['folder']}/{item['provenance']['file']}")}
        if item.get("window_instruction"):
            travel.add(P(f"{item['folder']}/{item['window_instruction']['file']}"))
        if item["item_kind"] == "benchmark_case":
            records |= {item["records"]["identity_file"]["path"], item["records"]["writer_report"]["path"]}
    assert {name for name in files if name.startswith("pack/")} == travel
    assert {name for name in files if name.startswith("records/")} == records
    assert len(records) == 2 * sum(1 for item in index["items"] if item["item_kind"] == "benchmark_case")
    assert not any(name.endswith("case.json") for name in files)
    assert not any(name.startswith("pack/") and name.endswith((".swbpipe-ids.json", ".writer-report.json"))
                   for name in files)
    instructions = files[P(spb.INSTRUCTIONS)].decode()
    assert "Copy the pack folder (`pack`, the folder that holds these instructions) and nothing else" in instructions
    assert "The `records` folder beside the pack folder stays on the Mac; never copy it." in instructions
    for item in index["items"]:  # what comes back is stated against the pack folder's own item folders
        assert f"Expected back in this folder (`{item['folder']}` of the pack folder)" in instructions


def test_no_file_of_the_pack_says_the_imported_file_is_never_saved_over():
    for name, data in built().items():
        if name.startswith("pack/"):
            text = data.decode("utf-8", errors="replace")
            assert "is never saved over" not in text and "never saved over" not in text, name


def test_every_model_declares_its_file_hash_and_a_model_without_one_is_refused(tmp_path):
    assert all(e.get("runs") or "mbf_sha256" in e for e in manifest()["models"])
    copy = tmp_path / "caepipe_benchmark"
    shutil.copytree(MANIFEST.parent, copy)
    altered = json.loads((copy / "manifest.json").read_text())
    victim = next(e for e in altered["models"] if e["family"] == "S1")
    del victim["mbf_sha256"]
    (copy / "manifest.json").write_text(json.dumps(altered))
    with pytest.raises(spb.BuildError, match=f"{victim['model_id']}: the manifest declares no model-file hash"):
        spb.build_pack(copy / "manifest.json")
    assert not hasattr(spb, "UNDECLARED_MODEL_FILE_HASH")


def test_the_single_form_case_quotes_the_manifest_and_a_form_a_run_no_sample_can_clear_refuses_the_build(
        monkeypatch):
    """cpb-dyn-c-n08-two is declared in the typed form only: the run sheet and its item quote the manifest's own
    statement of why, and it has no pair. Every form-A run's spectrum table is a dry-run sample's; one that is not
    refuses the build, since the manifest's rule could never let it run."""
    files = built()
    index = index_of(files)
    items = by_id(index)
    statement = manifest()["option_addition"]["single_form"]
    single = items["cpb-dyn-c-n08-two--srss-srss-table-typed"]
    assert single.get("single_form") and "pair" not in single and single["form"] == "table_typed"
    for name in (spb.RUN_SHEET, spb.INSTRUCTIONS):
        assert files[P(name)].decode().count(f'From the benchmark manifest: "{statement}"') == 1, name
    assert "No dry-run sample carries" not in files[P(spb.RUN_SHEET)].decode()
    sample_table = items["sample-dyn-window"]["spectrum_table"]
    assert all(item["spectrum_table"] == sample_table for item in index["items"] if item.get("form") == "table_added")
    real = spb.sample_items

    def other_table(*args, **kwargs):
        out = real(*args, **kwargs)
        for item in out:
            item["spectrum_table"] = ["SPCMS", "OTHER,1,0,0,0,3", "OTHER,1,1"]
        return out

    monkeypatch.setattr(spb, "sample_items", other_table)
    with pytest.raises(spb.BuildError, match="no dry-run sample carries this run's spectrum table"):
        spb.build_pack()


def test_the_hangers_input_not_held_equal_is_printed_beside_it_in_its_model_documents_words():
    files = built()
    item = by_id(index_of(files))["cpb-h-01--constant-support"]
    document = json.loads((ROOT / item["source"]["file"]).read_text())
    words = document["description"][document["description"].index("The support's load is an input NOT held equal"):]
    assert item["not_held_equal"] == words.strip()
    assert (f'From its model document (`{item["source"]["file"]}`): "{words.strip()}"'
            in files[P(spb.INSTRUCTIONS)].decode())
    assert [i["item_id"] for i in index_of(files)["items"] if i.get("not_held_equal")] == [item["item_id"]]
    assert index_of(files)["parts"][4]["items"][-1] == item["item_id"]  # ranked last


def test_the_one_session_alternative_is_allowed_for_and_no_sentence_says_every_run_begins_with_an_import():
    files = built()
    for name in (spb.INSTRUCTIONS, spb.RUN_SHEET):
        text = files[P(name)].decode().replace("\r\n", "\n")
        assert spb.ONE_SESSION in text
        for sentence in re.split(r"(?<=[.:;])\s", text):
            lowered = sentence.lower()
            assert not ("every" in lowered and "import" in lowered and "run" in lowered and "case" in lowered
                        and "begin" in lowered), sentence
    m = manifest()
    offering = [r for e in m["models"] for r in e.get("runs") or []
                if "instead of importing this run's file, you may"
                in (r.get("window_instruction") or {}).get("text", "").lower()]
    assert offering  # the manifest offers it; what comes back for such a run is the same as from its own file
    for run in offering:
        item = by_id(index_of(files))[run["run_id"]]
        assert item["results"][spb.WINDOW] == [run["expected_print_file"], run["expected_saved_file"]]
        assert item["results_if_written"] == {spb.WINDOW: [f"{run['run_id']}.res"]}


def test_no_screening_class_or_override_from_the_manifest_reaches_the_pack():
    """The manifest records, for thermal cases of the first two tranches, an override naming an input with both
    classes; the pack states no screening class for any case. For those models it prints only their ids, family,
    tranche, file paths and hashes, model-file settings read from the written file, and the provenance file."""
    files = built()
    m = manifest()
    overridden = {e["model_id"] for e in m["models"] if "inexact_input_override" in e}
    assert len(overridden) >= 18
    for name, data in files.items():
        text = data.decode("utf-8", errors="replace")
        for token in ("named_inexact", "inexact_input", "class_named", "class_as_first_screened", "both_classes",
                      "screening class", "screening_class", "rigid_support_stiffness"):
            assert token not in text, (name, token)
    for item in index_of(files)["items"]:
        if item.get("model_id") in overridden:
            assert not {"screening_classes", "inexact_input_override", "inexact_inputs"} & set(item), item["item_id"]


def test_each_model_file_has_a_provenance_sidecar_saying_only_what_is_established_of_it():
    """Beside each model file of pack/, in the form the data-boundary scan reads: not produced by CAEPIPE; origin
    invented, with what that rests on; the writer, the generator's model document by path and hash, the run by id
    with what was substituted or added by name, and the file's own SHA-256. No other field."""
    files = built()
    index = index_of(files)
    m = manifest()
    entries = {e["model_id"]: e for e in m["models"]}
    runs = {r["run_id"]: r for e in m["models"] for r in e.get("runs") or []}
    seen = 0
    for item in index["items"]:
        mbf = files[P(f"{item['folder']}/{item['mbf']['file']}")]
        sidecar = json.loads(files[P(f"{item['folder']}/{item['mbf']['file'][:-4]}.provenance.json")])
        seen += 1
        model_id = item["copy_of"]["model_id"] if item["item_kind"] == "dry_run_sample" else item["model_id"]
        run_id = item["copy_of"]["run_id"] if item["item_kind"] == "dry_run_sample" else (
            item["item_id"] if item["item_id"] in runs else None)
        entry = entries[model_id]
        assert set(sidecar) == {"produced_by_caepipe", "origin", "origin_basis", "generator", "written_by",
                                "model_document", "run", "mbf_sha256"}
        assert sidecar["produced_by_caepipe"] is False and sidecar["origin"] == "invented"
        assert sidecar["origin_basis"] == ("generator-made; drawn from seed 20261028" if entry["tranche"] in ("1", "2")
                                           else "generator-made; designed in the generator's text, drawing nothing "
                                           "from the seed")
        assert sidecar["model_document"] == {"path": f"validation/caepipe_benchmark/{entry['file']}",
                                             "file_sha256": entry["file_sha256"]}
        assert sidecar["mbf_sha256"] == hashlib.sha256(mbf).hexdigest()
        if run_id is None:
            assert sidecar["run"] is None
        else:
            run = runs[run_id]
            assert sidecar["run"]["run_id"] == run_id
            assert sidecar["run"]["added_records"] == [r["heading"] for r in run["mbf"].get("added_records") or []]
            assert sidecar["run"]["substituted_records"] == (["OPPRES"] if run["mbf"]["source"] == "substituted"
                                                             else [])
    assert seen == 213


def test_the_owners_reading_of_the_instructions_and_run_sheet():
    """The points of the owner's reading: the three conditions in one sentence; one way per case, with what comes back
    listed per way and no file named twice; the Analyze page quoted; the dry run ends by bringing the folder back, not
    on a word; the window sample saves as a real run does; part 1's table gives the strike order and the form."""
    files = built()
    index = index_of(files)
    instructions = files[P(spb.INSTRUCTIONS)].decode().replace("\r\n", "\n")
    sheet = files[P(spb.RUN_SHEET)].decode().replace("\r\n", "\n")
    assert ("A result from this sitting is screened only after (a) the harness's stored-results change (pull request "
            "262) has merged, (b) a rule for a returned file's origin is in place, and (c) the comparison record itself "
            "carries the qualification of SWBPIPE's results as a required member (pull request 263); no screen is run "
            "before all three.") in instructions
    assert ("Where a case is offered both ways, run it by its batch line; use the window way for that case only if its "
            "batch line left no results, and note that on its run line.") in instructions
    for item in index["items"]:
        block = instructions.split(f"Expected back in this folder (`{item['folder']}` of the pack folder):\n\n", 1)[1]
        block = block.split("\n\n", 1)[0]
        named = re.findall(r"`([^`]+)`", block)
        assert len(named) == len(set(named)), (item["item_id"], block)
        assert block.count("\n- ") + 1 == len(item["routes"]), block
        if len(item["routes"]) == 2:
            assert "- by the window way, only if the batch line left no results:" in block
    assert ('3. Analyse the model by File > Analyze (F12). The Analyze page (CAEPIPE-ANALYZE, '
            'https://docs.sstcae.com/users_manual/analyze__f12_.htm, read 2026-11-13) says: "' + spb.ANALYZE_VIEW_RESULTS
            + '" It also notes: "' + spb.ANALYZE_INACTIVE + '"') in instructions
    assert "- CAEPIPE-ANALYZE: https://docs.sstcae.com/users_manual/analyze__f12_.htm" in instructions
    assert "Go on to part 1 only when" not in instructions
    assert ("\n8. After part 0, copy the working folder to a place of your choosing on the Mac, outside every repository, "
            "leaving the working folder on Windows as it is. Say when the copy has finished and where it is; say it only "
            "once your copying tool says the copy has finished, because the copy must not change while it is being "
            "stored. It is stored there under one new name and listed by name, size and hash. Wait until you are told "
            "it is stored, whether or not a title was missing, and then go on to part 1.\n") in instructions
    assert '" If Analyze is inactive, note it on the run line and go on to the next case.' in instructions
    assert "in the program's window with the import command in 3.2, step 1." in instructions
    rule = ("never struck to make room for a later part. If time runs out within it, stop after a complete strike "
            "number: the lowest numbers are run first, and strike numbers 32 to 34 are the first to go.")
    assert ("4. If the sitting must be shortened, strike parts from the end: part 5 first, then part 4, part 3 and part "
            "2. Part 1 is " + rule) in instructions
    assert "Part 1 is " + rule in sheet and "This part is " + rule in sheet and "never cut" not in instructions + sheet
    assert ("9. If a section title is missing from the first case's print, write which on that case's run line, and say "
            "so when the folder is brought back: what to do about it is decided then, before part 1.") in instructions
    assert ("\nUse the same working folder for the rest of the sitting. At the end, copy it to the Mac whole in the same "
            "way, and say when the copy has finished and where it is.\n") in instructions
    window = by_id(index)["sample-dyn-window"]
    assert window["results"] == {spb.WINDOW: ["sample-dyn-window.mod"]}
    assert ".saved.mod" not in instructions + sheet
    assert ("Then save the model as `sample-dyn-window.mod` in that folder, as a real window run saves its model as "
            "`<run id>.mod` in its own folder. The Save page (CAEPIPE-SAVE, "
            "https://docs.sstcae.com/users_manual/save__ctrl_s_.htm, read 2026-11-13) says: \"" + spb.SAVE_WORDS + "\"") \
        in instructions
    check_iii = next(c for c in manifest()["dry_run"]["checks"] if c.startswith("(iii)"))
    assert "Then make check (iii): " + check_iii[len("(iii) "):] + "." in instructions  # the tranche's own words
    part1 = sheet.split("## Part 1:", 1)[1].split("## Part 2:", 1)[0]
    assert ("\nFor each pair, record its outcome on both run lines: the form you analysed and, on the other form's line, "
            "\"not run: other form\"; or, if form A was switched at its step 4 or 5, \"switched to counterpart <id>\" on "
            "form A's line (as its instruction says) and form B's own outcome on its line; or \"neither run\" and why on "
            "both lines. Tick the form you analysed, if any. This part is never struck") in part1
    assert "Tick the form you ran" not in sheet
    assert "This part is never struck to make room for a later part." in part1
    assert "| # | strike order | form | item |" in part1
    rows = [line.split(" | ") for line in part1.splitlines() if line.startswith("| ") and line[2].isdigit()]
    assert len(rows) == 67
    by_strike: dict[str, list[str]] = {}
    for row in rows:
        by_strike.setdefault(row[1], []).append(row[2])
    assert sorted(by_strike, key=int) == list(by_strike)  # adjacent, lowest first
    assert all(forms in (["A, table added", "B, table typed"], ["only form"]) for forms in by_strike.values())
    assert "format sample" not in instructions.replace(manifest()["option_addition"]["pairs"], "").replace(
        manifest()["option_addition"]["single_form"], "").replace(manifest()["dry_run"]["sample"], "")


def test_the_evidence_section_ties_its_three_classes_to_the_parts_routes_in_the_index():
    """Section 2 says what evidence each kind of run makes at the sitting: a window run (the parts whose every case is
    window only), a batch run (the parts whose every case has a batch line, and the dry run's cases) and a run by the
    one-session alternative. Every window case's "Expected back" and every part of the run sheet carry it."""
    files = built()
    index = index_of(files)
    window, batch = spb.evidence_parts(index)
    by = by_id(index)
    for number, part in enumerate(index["parts"]):
        routes = [by[i]["routes"] for i in part["items"]]
        assert (number in window) == (part["kind"] != "dry_run" and all(r == [spb.WINDOW] for r in routes))
        assert (number in batch) == (part["kind"] != "dry_run" and all(spb.BATCH in r for r in routes))
    assert (window, batch) == ([1, 4], [2, 3, 5])
    instructions = files[P(spb.INSTRUCTIONS)].decode().replace("\r\n", "\n")
    section = instructions.split("## 2. What binds a result to the model that was run\n\n", 1)[1].split("\n## 3.", 1)[0]
    assert "- **A window run** (parts 1 and 4, and a batch case run the window way): save the model under the run's " \
           "name, `<run id>.mod`, from inside the program before the analysis" in section
    assert "- **A batch run** (parts 2, 3 and 5, and the dry run's cases): the command line names the model file" in section
    assert ("If it shows none, a batch run's only evidence is that its print came back in the folder of the model file "
            "its command line named, and that is weaker evidence.") in section
    assert "- **A run made by the one-session alternative:** the model saved under that run's name" in section
    assert spb.KEEP_EVERYTHING in section
    for item in index["items"]:
        block = instructions.split(f"Expected back in this folder (`{item['folder']}` of the pack folder):\n\n", 1)[1]
        block = block.split("Run line (`run_line.txt`)", 1)[0]
        window_case = item["routes"] == [spb.WINDOW] and item["item_kind"] == "benchmark_case"
        assert (spb.KEEP_EVERYTHING in block) == window_case, item["item_id"]
    sheet = files[P(spb.RUN_SHEET)].decode().replace("\r\n", "\n")
    for number, part in enumerate(index["parts"]):
        text = sheet.split(f"## Part {number}:", 1)[1].split("\n## Part", 1)[0]
        line = next(l for l in text.split("\n") if l.startswith("Evidence of the model run:"))
        assert ("`<run id>.mod`" in line) == (number in window), number
        assert ("answer A" in line) == (number in batch), number


def test_the_mac_paragraph_is_the_agent_0s_settled_text_and_nothing_else_says_the_mac_checks_kinds():
    instructions = built()[P(spb.INSTRUCTIONS)].decode()
    settled = ("On the Mac, the storage procedure copies the returned folder under one new name, records each file's "
               "path, size and hash, and verifies the copied contents against the source hashes. It reads file contents "
               "as bytes without interpreting prints or run lines, comparing models with the pack, or classifying files "
               "by kind. This procedure checks no print sections. Your dry-run comparison checks the example print you "
               "inspect; it does not check subsequent prints.")
    assert spb.SET_PASSAGE == settled
    assert ("**On the Mac.** No SWBPIPE command for the returned folder is part of this pack. " + settled + "\r\n") \
        in instructions
    for name, data in built().items():
        if name.startswith("pack/") and name.endswith((".md", ".txt")):
            text = data.decode("utf-8", errors="replace").replace(settled, "")
            assert "by kind" not in text and "is the pack's" not in text, name
    assert "intake change" not in instructions and "merged" not in instructions.split("**What a result waits on.**")[0]


def test_the_index_designates_each_return_held_sealed_screened_or_stored_only():
    index = index_of(built())
    counts: dict[str, int] = {}
    for item in index["items"]:
        counts[item["designation"]] = counts.get(item["designation"], 0) + 1
        if item["designation"] == "held":
            assert item["run_kind"] in ("option_addition", "hand_entry") and item["routes"] == [spb.WINDOW]
    assert counts == {"held": 72, "sealed": 22, "stored_only": 3, "screened": 116}


def test_the_export_pages_words_the_batch_lines_answer_and_the_import_after_a_batch_line():
    """The vendor's export page is quoted where an import is described (3.2, step 1, and the dry run's answer C on the
    run sheet and the window sample's run line), with what it does not say; every batch line's "Expected back" says
    which files the program writes is answer A; a case imported after its batch line is noted as such."""
    files = built()
    instructions = files[P(spb.INSTRUCTIONS)].decode().replace("\r\n", "\n")
    sheet = files[P(spb.RUN_SHEET)].decode().replace("\r\n", "\n")
    words = ("But note that the existing .mod file will be overwritten. So, you must rename the edited text file before "
             "importing it back into CAEPIPE.")
    assert spb.EXPORT_OVERWRITE == words
    assert ('The export page (CAEPIPE-EXPORT-MBF, https://docs.sstcae.com/users_manual/export_mbf.htm) says: "' + words
            + '" The page does not say in which folder. Each run has its own folder for that reason.') in instructions
    run_line = files[P("samples/sample-dyn-window/run_line.txt")].decode()
    for text in (sheet, run_line):
        assert spb.IMPORT_MODEL_LABEL in text and words in spb.IMPORT_MODEL_LABEL
    for line in run_line.split("\r\n")[2:]:  # a filled-in label holds no colon of its own before its last one
        if line:
            assert line.endswith(": ") and ": " not in line[:-2], line
    for item in index_of(files)["items"]:
        if spb.BATCH in item["routes"]:
            block = instructions.split(f"Expected back in this folder (`{item['folder']}` of the pack folder):\n\n", 1)[1]
            batch_line = next(l for l in block.split("\n") if l.startswith("- by the batch line:"))
            assert batch_line.endswith(". Which of these the program writes is the dry run's answer A."), batch_line
    assert ("then run that case the window way (3.2) before pasting the next line. The import may write over the model "
            "file the batch line left in that folder (the export page's words in 3.2, step 1); note on the run line that "
            "the case was imported after its batch line.") in instructions


# -- the export ------------------------------------------------------------------------------------------------------
def _text(files: dict[str, bytes], name: str) -> str:
    return files[P(name)].decode("utf-8").replace("\r\n", "\n")


def _section5(instructions: str) -> dict[str, str]:
    """Each item's entry in section 5 of the instructions, by item id."""
    body = instructions.split("\n## 5. Per item\n", 1)[1].split("\n## Sources\n", 1)[0]
    entries = {}
    for chunk in body.split("\n### ")[1:]:
        head = chunk.split("\n", 1)[0]
        entries[head.split(" ", 2)[1]] = chunk
    return entries


def _export_runs(m: dict) -> set[str]:
    return {run["run_id"] for entry in m["models"] for run in entry.get("runs") or [] if run.get("export")}


def test_check_vs_duplicate_is_a_byte_copy_of_the_declared_run_file_and_has_no_batch_line():
    """The sacrificial duplicate the manifest names (dry_run.export_trial): its own folder, a byte copy of the same run
    file as the other two samples, window only, no line in the command file, placed after the window sample."""
    files = built()
    index = index_of(files)
    m = manifest()
    trial = m["dry_run"]["export_trial"]
    item_id = trial["file"][:-len(".mbf")]
    item = by_id(index)[item_id]
    assert item["folder"] == trial["folder"] == f"samples/{item_id}" and item["mbf"]["file"] == trial["file"]
    run = next(r for e in m["models"] for r in e.get("runs") or [] if r["run_id"] == trial["copy_of_run"])
    data = files[P(f"{trial['folder']}/{trial['file']}")]
    assert hashlib.sha256(data).hexdigest() == run["mbf"]["sha256"] == item["copy_of"]["sha256"]
    assert item["copy_of"]["run_id"] == trial["copy_of_run"]
    for other in ("sample-dyn-batch", "sample-dyn-window"):
        assert data == files[P(f"samples/{other}/{other}.mbf")]
    assert item["item_kind"] == "dry_run_sample" and item["routes"] == [spb.WINDOW]
    assert item["designation"] == "stored_only" and "window_instruction" not in item
    commands = files[P(spb.BATCH_COMMANDS)].decode()
    assert item_id not in commands and "export" not in commands
    part0 = index["parts"][0]["items"]
    assert part0[-2:] == ["sample-dyn-window", item_id]
    entry = _section5(_text(files, spb.INSTRUCTIONS))[item_id]
    assert ("- by the window way: the model file the import writes, if it writes one (answer C), and "
            f"`{item_id}.export.mbf` (if check (v) allowed the export)\n") in entry


def test_exactly_the_runs_that_offer_an_export_carry_it_in_expected_back_and_on_the_run_line():
    files = built()
    index = index_of(files)
    exporting = _export_runs(manifest())
    assert len(exporting) == 24
    entries = _section5(_text(files, spb.INSTRUCTIONS))
    label = "export (made, or not made and why): "
    assert spb.EXPORT_LABEL + ": " == label
    seen = 0
    for item in index["items"]:
        run_line = files[P(f"{item['folder']}/run_line.txt")].decode()
        exported = f"{item['item_id']}.export.mbf"
        offers = item["item_id"] in exporting
        assert run_line.count(label) == int(offers), item["item_id"]
        assert (f"`{exported}` (if the export was made)" in entries[item["item_id"]]) == offers, item["item_id"]
        trial = item.get("export_check") is not None
        assert (exported in item["results"].get(spb.WINDOW, [])) == (offers or trial), item["item_id"]
        assert (".export.mbf" in entries[item["item_id"]]) == (offers or trial)
        for line in run_line.split("\r\n")[2:]:  # a label holds no colon of its own before its last one
            if line:
                assert line.endswith(": ") and ": " not in line[:-2], line
        seen += offers
    assert seen == 24


def test_check_v_and_its_consequence_are_given_as_the_manifest_gives_them():
    """On the run sheet's part 0 and in section 4 of the instructions, check (v) and its consequence verbatim; a step
    after the window sample's sends the owner to the duplicate folder and says what is not touched; check (v)'s
    answers go on the trial's run line under labels in check (v)'s own words, not on the window sample's."""
    files = built()
    m = manifest()
    check_v = next(c for c in m["dry_run"]["checks"] if c.startswith("(v)"))
    consequence_v = next(c for c in m["dry_run"]["consequences"] if c.startswith("(v)"))
    part0 = _text(files, spb.RUN_SHEET).split("## Part 0:", 1)[1].split("## Part 1:", 1)[0]
    instructions = _text(files, spb.INSTRUCTIONS)
    section4 = instructions.split("\n## 4. ", 1)[1].split("\n## 5. ", 1)[0]
    for text in (part0, section4):
        assert "\n" + check_v + "\n" in text
        assert "\n" + consequence_v + "\n" in text or "\n- " + consequence_v + "\n" in text
    step = ("6. Then go to the export check's folder (`samples/sample-dyn-export-trial`) and make check (v) there, in "
            "that folder only, as the benchmark manifest gives it below this list. Write each answer on that folder's "
            "run line. The window sample's folder (`samples/sample-dyn-window`) and every case folder are not opened or "
            "touched by this check.")
    assert "\n" + step + "\n" in section4
    assert section4.index("\n5. In the window sample's folder") < section4.index("\n" + step)
    steps = {int(line.split(". ", 1)[0]): line.split(". ", 1)[1] for line in check_v.split("\n")[1:]}
    trial_line = files[P("samples/sample-dyn-export-trial/run_line.txt")].decode()
    answered = []
    for number, label, words in spb.EXPORT_CHECK_ANSWERS:
        assert label.startswith(f"(v) step {number}, ") and ":" not in label
        body = label[len(f"(v) step {number}, "):]
        assert all(w in steps[number] and w.lower() in body.lower() for w in words), label
        assert sum(len(w) for w in words) >= 0.6 * len(body), label  # mostly the manifest's words
        assert f"\r\n{label}: \r\n" in trial_line and f"- {label}: " in part0
        answered.append(number)
    assert answered == [3, 5, 6, 7]  # the steps of check (v) that ask for something to be written or chosen
    window_line = files[P("samples/sample-dyn-window/run_line.txt")].decode()
    assert "(v)" not in window_line and "(iv)" in window_line


def test_section_2_binds_an_export_to_the_runs_saved_model_and_the_statement_is_read_before_part_1():
    files = built()
    m = manifest()
    instructions = _text(files, spb.INSTRUCTIONS)
    section2 = instructions.split("\n## 2. ", 1)[1].split("\n## 3. ", 1)[0]
    binding = ("An export, where one is made, is bound to that run's saved model and that run's instruction: the "
               "program writes it from that run's saved model (`<run id>.mod`) as the program holds it after that "
               "run's instruction, including what was entered or set by hand; it is not the pack's model file, and it "
               "is not evidence of which model was run.")
    assert "\n" + binding + "\n" in section2
    words = ("as the program holds it after that run's instruction, including what was entered or set by hand; it is "
             "not the pack's model file")
    assert words in m["export"]["statement"]  # the manifest's binding words
    statement = '"' + m["export"]["statement"] + '."'
    assert statement in section2 and m["export"]["runs"] in section2
    assert "Each export's steps are optional, and you may drop them" in section2
    assert instructions.index(statement) < instructions.index("\n## 4. ")


EXPORT_INPUT_WORDS = re.compile(r"(pack's|imported|case's|this run's) (model )?file|<stem>\.mbf|<run id>\.mbf|"
                                r"\.mbf` of the pack", re.IGNORECASE)


def test_no_text_of_the_pack_says_an_exports_input_is_the_packs_file():
    """Every sentence of the pack that speaks of an export and of the pack's (or the imported) model file says the
    export is not the pack's model file."""
    files = built()
    hits = 0
    for name, data in files.items():
        if name.split("/", 1)[0] != spb.PACK or not name.endswith((".md", ".txt", ".json")):
            continue
        text = data.decode("utf-8")
        for sentence in re.split(r"(?<=\.)\s+|\r?\n", text):
            if re.search(r"export", sentence, re.IGNORECASE) and EXPORT_INPUT_WORDS.search(sentence):
                hits += 1
                assert "not the pack's model file" in sentence, (name, sentence)
    assert hits  # the statement and the binding sentence are among them


def test_part_1_prints_the_manifests_pair_rule_verbatim():
    files = built()
    rule = manifest()["option_addition"]["run_sheet_pair_rule"]
    part1 = _text(files, spb.RUN_SHEET).split("## Part 1:", 1)[1].split("## Part 2:", 1)[0]
    assert f'\nFrom the benchmark manifest: "{rule}"\n' in part1
    assert "Exactly one of a pair" not in _text(files, spb.RUN_SHEET) + _text(files, spb.INSTRUCTIONS)


def test_a_window_cases_results_file_is_conditional_on_what_the_program_writes():
    """Every window case's expected back names its results file only as one the program may write; the batch lists
    keep deferring to answer A."""
    files = built()
    entries = _section5(_text(files, spb.INSTRUCTIONS))
    window_cases = 0
    for item in index_of(files)["items"]:
        if item["item_kind"] != "benchmark_case":
            continue
        stem = item["stem"]
        lines = [l for l in entries[item["item_id"]].split("\n") if l.startswith("- by the ")]
        if item["routes"] == [spb.WINDOW]:
            window_cases += 1
            (line,) = lines
            assert line.endswith(f", and any results file the program writes (for example `{stem}.res`)"), line
            assert line.count(f"{stem}.res") == 1 and f"`{stem}.res`," not in line
        else:
            assert lines[0].endswith(". Which of these the program writes is the dry run's answer A.")
            assert f"`{stem}.res`" in lines[0]
    assert window_cases == 72


def test_no_prose_of_the_pack_says_trial():
    """The pack calls check (v) a check; "trial" occurs only inside the duplicate folder's name, which stays."""
    for name, data in built().items():
        if name.split("/", 1)[0] != spb.PACK:
            continue
        text = data.decode("utf-8", errors="replace").replace("sample-dyn-export-trial", "")
        assert "trial" not in text.lower(), (name, text[max(0, text.lower().find("trial") - 60):][:120])
