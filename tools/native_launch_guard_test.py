"""Offline checks of the native launch helper and the controllers that use it. Nothing here starts
the application: the processes started are short-lived ones of the tests' own making, from an
invented executable. The account's real Library, home folders, temporary and cache folders and
screen are never read: every check gets invented ones, and every name in them is invented."""
from __future__ import annotations

# Its directory scans read only folders the tests make under pytest's temporary folder, never
# repository files (tools/ci/python_test_reads.py).
CI_READS = ()

import ast
import json
import os
from pathlib import Path
import plistlib
import re
import shutil
import signal
import socket
import subprocess
import sys
import textwrap
import time
from types import SimpleNamespace

import pytest

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from tools import native_launch_guard as guard  # noqa: E402
from tools import agent_journey_fixture as journey  # noqa: E402
from tools import agent_live_session_fixture as live  # noqa: E402

BUNDLE, EXE = "com.chirality.swbpipe", "openpipestress-desktop"
NAMES = [BUNDLE, EXE]
NATIVE_SMOKE = Path(guard.__file__).resolve().parents[1] / "apps" / "desktop" / "src-tauri" / "src" / "native_smoke.rs"
MACOS = pytest.mark.skipif(sys.platform != "darwin", reason="the check for possible writers is made on macOS only")
HELPER = "com.apple.WebKit.Networking+" + EXE.replace("-", "_") + "-1a2b3c4d5e"
NO_WRITERS = ([], {"statement": guard.RESIDUAL_STATEMENT, "processRows": 0})


def executable(folder, body="import sys; sys.exit(0)", marker=True):
    """An invented executable inside an invented bundle copy: a Python script, carrying the strict
    chooser variable's name in a comment when `marker`."""
    path = folder / "Copy.app" / "Contents" / "MacOS" / EXE
    path.parent.mkdir(parents=True, exist_ok=True)
    comment = f"# {guard.STRICT_CHOOSER_ENV}\n" if marker else "\n"
    path.write_text(f"#!{sys.executable} -I\n{comment}{body}\n")
    path.chmod(0o755)
    return path


@pytest.fixture
def account(tmp_path, monkeypatch):
    """An invented account: its home, Library, Desktop, Documents and Downloads, the system's
    temporary and cache folders, an evidence folder and an invented executable."""
    real = tmp_path / "real-home"
    for top in guard.WATCHED_TOPS:
        (real / "Library" / top).mkdir(parents=True)
    for folder in ("Desktop", "Documents", "Downloads"):
        (real / folder).mkdir()
    temp, cache, evidence = tmp_path / "system-temp", tmp_path / "system-cache", tmp_path / "evidence"
    for folder in (temp, cache, evidence):
        folder.mkdir()
    monkeypatch.setattr(guard, "real_home", lambda: real)
    monkeypatch.setattr(guard, "temp_root", lambda: temp.resolve())
    monkeypatch.setattr(guard, "user_cache_dir", lambda: cache.resolve())
    monkeypatch.setattr(guard, "check_launch", lambda *a: {"launch": "invented"})
    monkeypatch.setattr(guard, "SETTLE_SECONDS", 0)
    monkeypatch.setattr(guard, "SECOND_LOOK_SECONDS", 0)
    yield SimpleNamespace(real=real, library=real / "Library", temp=temp.resolve(), cache=cache.resolve(),
                          evidence=evidence.resolve(), exe=executable(tmp_path), tmp=tmp_path)
    open_up(tmp_path)  # whatever the test ended by, nothing it made stays unremovable


def open_up(path):
    """Give back owner access to every folder under `path`, so that removal never meets a folder a
    test left unreadable; links are not followed."""
    try:
        os.chmod(path, 0o700)
        entries = list(os.scandir(path))
    except OSError:
        return
    for entry in entries:
        if entry.is_dir(follow_symlinks=False):
            open_up(entry.path)


def no_writers(monkeypatch):
    monkeypatch.setattr(guard, "possible_writers", lambda *a, **k: NO_WRITERS)


def new_run(account, label="home", evidence=None):
    run = guard.Run(account.exe, evidence or account.evidence, label, names=NAMES)
    run.library, run.cache, run.real = account.library, account.cache, account.real
    return run


def place(lib, top, name, leaf="data.bin"):
    path = lib / top / name / leaf
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_bytes(b"x")
    return path


def in_cache(account, *parts):
    path = account.cache.joinpath(*parts)
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_bytes(b"x")
    return path


def web_view_data(run):
    data = run.home / "Library" / "WebKit" / EXE / "WebsiteData"
    data.mkdir(parents=True, exist_ok=True)
    (data / "salt").write_bytes(b"12345678")


def evidence_file(account, name):
    return json.loads((account.evidence / name).read_text())


# ---------------------------------------------------------------------------
# 7. One run lifecycle, the only way to start.

def test_there_is_no_start_outside_the_run():
    for name in ("start", "spec", "make_home", "remove_home"):
        assert not hasattr(guard, name), name


def test_a_run_cannot_start_before_it_begins_or_after_it_finishes(account, monkeypatch):
    no_writers(monkeypatch)
    run = new_run(account)
    with pytest.raises(guard.LaunchRefused, match="run_not_open"):
        run.start("app", [str(account.exe)])
    run.begin()
    run.finish()
    with pytest.raises(guard.LaunchRefused, match="run_not_open"):
        run.start("app", [str(account.exe)])


def test_the_run_starts_only_its_own_executable(account, monkeypatch):
    no_writers(monkeypatch)
    run = new_run(account).begin()
    try:
        with pytest.raises(guard.LaunchRefused, match="executable_not_the_runs"):
            run.start("app", [sys.executable, "-c", "pass"])
    finally:
        run.finish()


def test_the_comparison_is_persisted_before_the_home_is_removed_and_the_record_says_the_limits(account, monkeypatch):
    seen = []
    real_remove = guard.remove_at
    def remove(*a, **k):
        seen.append((account.evidence / "home-comparison.json").is_file())
        return real_remove(*a, **k)
    no_writers(monkeypatch)
    monkeypatch.setattr(guard, "remove_at", remove)
    run = new_run(account).begin()
    home = run.home
    outcome = run.finish()
    assert seen == [True] and not home.exists() and outcome["throwawayHome"]["cleanup"] == "removed"
    record = evidence_file(account, "home-run.json")
    assert record["sockets"] == guard.SOCKETS_STATEMENT and record["residual"] == guard.RESIDUAL_STATEMENT
    assert record["confinement"] == guard.CONFINEMENT_STATEMENT and record["admissions"]
    assert outcome["throwawayHome"]["secondLook"] == "not reappeared"


def test_a_comparison_that_raises_does_not_skip_the_removal(account, monkeypatch):
    no_writers(monkeypatch)
    monkeypatch.setattr(guard, "compare", lambda *a, **k: (_ for _ in ()).throw(OSError("invented")))
    run = new_run(account).begin()
    home = run.home
    outcome = run.finish()
    assert outcome["comparison"]["verdict"] == "not compared" and not home.exists()
    assert evidence_file(account, "home-comparison.json")["verdict"] == "not compared"


def test_an_interrupt_during_the_settle_wait_skips_neither_comparison_nor_removal(account, monkeypatch):
    no_writers(monkeypatch)
    run = new_run(account).begin()
    run.children.append(SimpleNamespace(pid=1, poll=lambda: 0))
    real_sleep = time.sleep
    calls = []
    def sleep(seconds):
        calls.append(seconds)
        if len(calls) == 1:
            raise KeyboardInterrupt
        real_sleep(0)
    monkeypatch.setattr(guard.time, "sleep", sleep)
    home = run.home
    outcome = run.finish()
    assert outcome["comparison"]["compared"] is True and not home.exists()
    assert outcome["interruptions"] and "settle" in outcome["interruptions"][0]


# ---------------------------------------------------------------------------
# 1 and 9. Removal and listing through descriptors that follow nothing.

def test_removal_refuses_a_folder_that_is_not_the_one_the_kit_made(account, monkeypatch):
    no_writers(monkeypatch)
    run = new_run(account).begin()
    home = run.home
    shutil.rmtree(home)
    home.mkdir()
    (home / "kept").write_bytes(b"x")
    outcome = run.finish()
    assert outcome["throwawayHome"]["cleanup"] == "refused" and (home / "kept").is_file()


def test_remove_at_checks_identity_after_opening_and_again_before_the_last_step(tmp_path):
    parent = tmp_path / "parent"
    (parent / "home").mkdir(parents=True)
    other = tmp_path / "other"
    other.mkdir()
    fd = os.open(parent, guard.O_DIR)
    try:
        wrong = [os.stat(other).st_dev, os.stat(other).st_ino]
        with pytest.raises(guard._Custody, match="identity_changed"):
            guard.remove_at(fd, "home", wrong)
        assert (parent / "home").is_dir()
    finally:
        os.close(fd)


def test_removal_unlinks_links_inside_and_never_follows_them(account, monkeypatch):
    no_writers(monkeypatch)
    outside = account.tmp / "outside"
    (outside / "deep").mkdir(parents=True)
    (outside / "deep" / "kept").write_bytes(b"x")
    run = new_run(account).begin()
    (run.work / "link").symlink_to(outside)
    (run.work / "nested").mkdir()
    (run.work / "nested" / "link").symlink_to(outside / "deep")
    home = run.home
    assert run.finish()["throwawayHome"]["cleanup"] == "removed" and not home.exists()
    assert (outside / "deep" / "kept").is_file()


def test_a_folder_swapped_after_it_was_seen_is_not_entered(tmp_path):
    (tmp_path / "a").mkdir()
    (tmp_path / "b").mkdir()
    fd = os.open(tmp_path, guard.O_DIR)
    try:
        seen_b = os.stat("b", dir_fd=fd, follow_symlinks=False)
        with pytest.raises(guard._Custody, match="changed"):
            guard._open_child(fd, "a", seen_b)
    finally:
        os.close(fd)


def test_removal_is_bounded_in_depth(account, monkeypatch):
    no_writers(monkeypatch)
    monkeypatch.setattr(guard, "MAX_DEPTH", 2)
    run = new_run(account).begin()
    (run.work / "a" / "b" / "c" / "d").mkdir(parents=True)
    home = run.home
    assert run.finish()["throwawayHome"]["cleanup"] == "refused" and home.exists()


# (a) A linked root: the foreign folder is never removed.
def test_a_kit_folder_replaced_by_a_link_leads_the_removal_nowhere_foreign(account, monkeypatch):
    no_writers(monkeypatch)
    run = new_run(account).begin()
    name = run.entry["home"]
    kit_path = account.temp / guard.KIT_DIR
    moved = account.temp / "moved-kit"
    kit_path.rename(moved)
    foreign = account.tmp / "foreign"
    (foreign / "homes" / name).mkdir(parents=True)
    (foreign / "homes" / name / "kept").write_bytes(b"x")
    (foreign / "registry").mkdir()
    kit_path.symlink_to(foreign)
    run.finish()
    assert (foreign / "homes" / name / "kept").is_file()
    assert not (moved / "homes" / name).exists()  # the kit's own folder, reached by its descriptor


def test_a_start_after_the_kit_folder_was_replaced_by_a_link_is_refused(account, monkeypatch):
    no_writers(monkeypatch)
    run = new_run(account).begin()
    kit_path = account.temp / guard.KIT_DIR
    kit_path.rename(account.temp / "moved-kit")
    foreign = account.tmp / "foreign"
    foreign.mkdir()
    kit_path.symlink_to(foreign)
    try:
        with pytest.raises(guard.LaunchRefused, match="home_path_changed"):
            run.start("app", [str(account.exe)])
    finally:
        run.finish()


def test_a_kit_folder_that_is_a_link_refuses_the_kit(account):
    foreign = account.tmp / "foreign"
    foreign.mkdir(mode=0o700)  # private, so only the link itself can refuse it
    (account.temp / guard.KIT_DIR).symlink_to(foreign)
    with pytest.raises(guard.LaunchRefused, match="kit_folder_unopenable"):
        guard.Kit()


# ---------------------------------------------------------------------------
# 3 and 11. The registry is the authority; left homes are inventoried.

def test_a_left_home_is_reported_at_the_next_admission_and_refuses_only_while_a_process_may_be_alive(account, monkeypatch):
    no_writers(monkeypatch)
    first = new_run(account, "first").begin()
    first.generations.append({"pid": 4242, "start": "2026-01-01T00:00:00", "role": "app"})
    first.entry["started"] = list(first.generations)
    first.kit.write_entry(first.entry)
    left = first.home
    assert first.finish(retain=True)["throwawayHome"]["cleanup"] == "withheld"
    alive = {os.getpid(): (1, 1, os.getuid(), sys.executable, "x"), 4242: (1, 1, os.getuid(), "/invented", "2026-01-01T00:00:00")}
    monkeypatch.setattr(guard, "_process_table", lambda: alive)
    monkeypatch.setattr(guard, "_open_under", lambda home: set())
    with pytest.raises(guard.LaunchRefused, match="left_home_processes_not_established_gone"):
        new_run(account, "second").begin()
    recycled = {**alive, 4242: (1, 1, os.getuid(), "/invented", "2026-02-02T00:00:00")}
    monkeypatch.setattr(guard, "_process_table", lambda: recycled)
    second = new_run(account, "third").begin()
    reported = second.admissions[0]["leftHomes"]
    second.finish()
    assert [item["home"] for item in reported] == [left.name] and left.is_dir()


def test_a_left_home_whose_start_time_was_not_read_stays_unestablished(account, monkeypatch):
    no_writers(monkeypatch)
    first = new_run(account, "first").begin()
    first.entry["started"] = [{"pid": 4242, "start": None, "role": "app"}]
    first.kit.write_entry(first.entry)
    first.finish(retain=True)
    monkeypatch.setattr(guard, "_process_table", lambda: {os.getpid(): (1, 1, 0, "", "x"), 4242: (1, 1, 0, "", "y")})
    monkeypatch.setattr(guard, "_open_under", lambda home: set())
    with pytest.raises(guard.LaunchRefused, match="unestablished"):
        new_run(account, "second").begin()


def test_an_inventory_that_cannot_list_processes_refuses(account, monkeypatch):
    no_writers(monkeypatch)
    new_run(account, "first").begin().finish(retain=True)
    monkeypatch.setattr(guard, "_process_table", lambda: (_ for _ in ()).throw(OSError("invented")))
    with pytest.raises(guard.LaunchRefused, match="could not run"):
        new_run(account, "second").begin()


def test_a_home_made_before_its_record_was_complete_enters_the_inventory(account, monkeypatch):
    no_writers(monkeypatch)
    run = new_run(account)
    real_write = guard.Kit.write_entry
    calls = []
    def write(self, entry):
        calls.append(entry["state"])
        if len(calls) == 2:
            raise OSError("invented")
        return real_write(self, entry)
    monkeypatch.setattr(guard.Kit, "write_entry", write)
    with pytest.raises(OSError):
        run.begin()
    outcome = run.finish()
    monkeypatch.setattr(guard.Kit, "write_entry", real_write)
    assert outcome["throwawayHome"]["cleanup"] == "withheld" and run.home.is_dir()
    kit = guard.Kit()
    try:
        inventory = guard.left_homes(kit)
    finally:
        kit.close()
    assert [(i["home"], i["state"]) for i in inventory["leftHomes"]] == [(run.entry["home"], "creating")]


def test_an_unregistered_folder_under_the_kit_is_reported_and_never_removed(account, monkeypatch):
    no_writers(monkeypatch)
    kit = guard.Kit()
    os.mkdir("swbpipe-home-shaped", dir_fd=kit.homes_fd)
    inventory = guard.left_homes(kit)
    kit.close()
    assert {"home": "swbpipe-home-shaped", "state": "unregistered", "exists": True} in inventory["leftHomes"]
    run = new_run(account).begin()
    run.finish()
    assert (account.temp / guard.KIT_DIR / "homes" / "swbpipe-home-shaped").is_dir()


def test_only_the_run_that_made_a_left_home_removes_it(account, monkeypatch):
    no_writers(monkeypatch)
    monkeypatch.setattr(guard, "PLATFORM", "darwin")
    monkeypatch.setattr(guard, "_process_table", lambda: {os.getpid(): (1, 1, 0, "", "x")})
    monkeypatch.setattr(guard, "_open_under", lambda home: set())
    run = new_run(account).begin()
    run.finish(retain=True)
    record = evidence_file(account, "home-record.json")
    assert guard.main(["remove-left-home", "--record", str(write_json(account.tmp / "forged.json", {**record, "token": "0" * 64}))]) == 2
    assert guard.main(["remove-left-home", "--record", str(write_json(account.tmp / "other.json", {**record, "id": "feedfacefeedface"}))]) == 2
    assert run.home.is_dir()
    assert guard.main(["remove-left-home", "--record", str(account.evidence / "home-record.json")]) == 0
    assert not run.home.exists()


def write_json(path, data):
    path.write_text(json.dumps(data))
    return path


# (c) Two runs sharing one evidence folder.
def test_two_runs_sharing_an_evidence_folder_each_remove_only_their_own(account, monkeypatch):
    no_writers(monkeypatch)
    monkeypatch.setattr(guard, "_process_table", lambda: {os.getpid(): (1, 1, 0, "", "x")})
    monkeypatch.setattr(guard, "_open_under", lambda home: set())
    one, two = new_run(account, "one").begin(), new_run(account, "two").begin()
    with pytest.raises(FileExistsError):
        new_run(account, "one").begin()  # a record of that name exists: never written over
    one_home, two_home = one.home, two.home
    one.finish()
    assert not one_home.exists() and two_home.is_dir()
    two.finish()
    assert not two_home.exists()


# (d) Order: the writer check runs on the same home before any removal, normal completion included.
def test_no_removal_before_the_writer_check_on_that_home(account, monkeypatch):
    order = []
    monkeypatch.setattr(guard, "possible_writers", lambda home, *a, **k: order.append(("check", str(home))) or NO_WRITERS)
    real_remove = guard.remove_at
    monkeypatch.setattr(guard, "remove_at", lambda fd, name, ident: order.append(("remove", name)) or real_remove(fd, name, ident))
    run = new_run(account).begin()
    home = run.home
    run.finish()
    assert order == [("check", str(home)), ("remove", home.name)]


# (e) Every refusal is recorded, and it enters the inventory.
def test_a_refused_removal_is_recorded_and_stays_in_the_inventory(account, monkeypatch):
    no_writers(monkeypatch)
    run = new_run(account).begin()
    home = run.home
    shutil.rmtree(home)
    home.symlink_to(account.tmp)
    assert run.finish()["throwawayHome"]["cleanup"] == "refused"
    kit = guard.Kit()
    try:
        states = [(i["home"], i["state"]) for i in guard.left_homes(kit)["leftHomes"]]
    finally:
        kit.close()
    assert (home.name, "refused") in states and account.tmp.is_dir()


# 6. The second look: a reappeared path is recorded, has its own exit status, and is a left home.
def test_a_path_that_reappears_after_removal_is_recorded_as_reappeared(account, monkeypatch):
    no_writers(monkeypatch)
    real_remove = guard.remove_at
    def remove_then_recreate(fd, name, identity):
        done = real_remove(fd, name, identity)
        os.mkdir(name, dir_fd=fd)  # a late writer recreating the folder
        return done
    monkeypatch.setattr(guard, "remove_at", remove_then_recreate)
    run = new_run(account).begin()
    outcome = run.finish()
    assert outcome["throwawayHome"]["cleanup"] == "reappeared" and guard.EXIT["reappeared"] == 7
    kit = guard.Kit()
    try:
        assert ("reappeared" in [i["state"] for i in guard.left_homes(kit)["leftHomes"]])
    finally:
        kit.close()


# ---------------------------------------------------------------------------
# 6. Possible writers: generations, coverage, and incomplete listings refusing.

def test_an_unparsed_process_row_or_a_listing_without_this_process_refuses(monkeypatch):
    rows = [b"  1 0 1 0 Thu Nov 12 18:37:03 2026 /sbin/launchd\n", b"x y z\n"]
    monkeypatch.setattr(guard, "_read", lambda argv, timeout=0: (0, b"".join(rows), ""))
    with pytest.raises(OSError, match="unparsed"):
        guard._process_table()
    monkeypatch.setattr(guard, "_read", lambda argv, timeout=0: (0, rows[0], ""))
    with pytest.raises(OSError, match="incomplete"):
        guard._process_table()
    mine = f"{os.getpid()} 1 1 {os.getuid()} Thu Nov 12 18:37:03 2026 /bin/python3\n".encode()
    monkeypatch.setattr(guard, "_read", lambda argv, timeout=0: (0, rows[0] + mine, ""))
    assert guard._process_table()[os.getpid()][4] == "2026-11-12T18:37:03"


def test_an_unparsed_open_files_listing_refuses(monkeypatch):
    monkeypatch.setattr(guard, "_read", lambda argv, timeout=0: (0, b"p12\nfcwd\nunexpected\n", ""))
    with pytest.raises(OSError, match="unparsed"):
        guard._open_under("/invented")
    monkeypatch.setattr(guard, "_read", lambda argv, timeout=0: (1, b"", "lsof: invented failure"))
    with pytest.raises(OSError):
        guard._open_under("/invented")


def test_generations_are_identifier_and_start_time_together():
    table = {7: (1, 7, 0, "/invented", "2026-01-01T00:00:00")}
    assert guard._generation_alive({"pid": 7, "start": "2026-01-01T00:00:00"}, table) == "alive"
    assert guard._generation_alive({"pid": 7, "start": "2025-01-01T00:00:00"}, table) is None
    assert guard._generation_alive({"pid": 7, "start": None}, table) == "unestablished"
    assert guard._generation_alive({"pid": 8, "start": None}, table) is None


@MACOS
def test_a_child_the_run_started_and_has_not_seen_end_withholds(account):
    account.exe.write_text(account.exe.read_text().replace("import sys; sys.exit(0)", "import time; time.sleep(30)"))
    run = new_run(account).begin()
    child = run.start("app", [str(account.exe)])
    try:
        outcome = run.finish()
    finally:
        child.kill()
        child.wait()
    assert outcome["throwawayHome"]["cleanup"] == "withheld" and run.home.is_dir(), outcome["throwawayHome"]
    writers = outcome["throwawayHome"]["writers"]
    assert any("started by the run" in " ".join(w["why"]) for w in writers)
    assert run.generations[0]["start"] is not None and outcome["throwawayHome"]["coverage"]["processRows"] > 0


@MACOS
def test_a_recycled_identifier_does_not_hide_a_later_process_holding_the_home(account):
    run = new_run(account).begin()
    holder = subprocess.Popen(["/bin/sh", "-c", f"exec 3>'{run.work}/open.txt'; /bin/sleep 30"], start_new_session=True)
    try:
        time.sleep(0.3)
        run.generations.append({"pid": holder.pid, "start": "1999-01-01T00:00:00", "role": "app"})  # a gone generation
        outcome = run.finish()
    finally:
        os.killpg(holder.pid, signal.SIGKILL)
        holder.wait()
    assert outcome["throwawayHome"]["cleanup"] == "withheld"
    assert any("open under the home" in w["why"] for w in outcome["throwawayHome"]["writers"])


@MACOS
def test_a_process_working_under_the_home_withholds(account):
    run = new_run(account).begin()
    holder = subprocess.Popen(["/bin/sleep", "30"], cwd=run.work, start_new_session=True)
    try:
        time.sleep(0.3)
        outcome = run.finish()
    finally:
        holder.kill()
        holder.wait()
    assert outcome["throwawayHome"]["cleanup"] == "withheld"


@MACOS
def test_a_process_in_an_attributed_group_withholds(account):
    run = new_run(account).begin()
    holder = subprocess.Popen(["/bin/sh", "-c", "/bin/sleep 30 & wait"], start_new_session=True)
    try:
        time.sleep(0.3)
        run.attribute(group=holder.pid)
        outcome = run.finish()
    finally:
        os.killpg(holder.pid, signal.SIGKILL)
        holder.wait()
    assert outcome["throwawayHome"]["cleanup"] == "withheld"
    assert any("in a group attributed to the run" in w["why"] for w in outcome["throwawayHome"]["writers"])


def test_on_any_other_system_the_writer_check_cannot_run_and_the_home_is_withheld(account, monkeypatch):
    monkeypatch.setattr(guard, "PLATFORM", "linux")
    run = new_run(account).begin()
    outcome = run.finish()
    assert outcome["throwawayHome"]["cleanup"] == "withheld" and "could not run" in outcome["throwawayHome"]["reason"]


# A failure after a child starts and before it is registered.
def test_a_failure_after_the_start_and_before_registration_keeps_the_child_in_custody(account, monkeypatch):
    account.exe.write_text(account.exe.read_text().replace("import sys; sys.exit(0)", "import time; time.sleep(30)"))
    no_writers(monkeypatch)
    monkeypatch.setattr(guard, "possible_writers", lambda home, children=(), *a, **k: (
        [{"pid": c.pid, "why": ["started by the run and not seen to end"]} for c in children if c.poll() is None], {}))
    run = new_run(account).begin()
    real_write = guard.Kit.write_entry
    monkeypatch.setattr(guard.Kit, "write_entry", lambda self, entry: (_ for _ in ()).throw(OSError("invented"))
                        if entry["started"] else real_write(self, entry))
    with pytest.raises(OSError):
        run.start("app", [str(account.exe)])
    try:
        assert len(run.children) == 1
        outcome = run.finish()
        assert outcome["throwawayHome"]["cleanup"] == "withheld" and run.home.is_dir()
    finally:
        run.children[0].kill()
        run.children[0].wait()


# ---------------------------------------------------------------------------
# 1, 4, 5, 9. The start: environment, working directory, descriptors, values.

NAMES_ONLY = "import os; print('\\n'.join(sorted(os.environ)))"


def started_output(account, body, monkeypatch=None, kit=None, **streams):
    account.exe.write_text(account.exe.read_text().replace("import sys; sys.exit(0)", body))
    no_writers(monkeypatch) if monkeypatch else None
    run = new_run(account).begin()
    try:
        child = run.start("app", [str(account.exe)], kit, stdout=subprocess.PIPE, **streams)
        out = child.communicate(timeout=30)[0].decode()
    finally:
        run.finish()
    return run, out


def test_nothing_of_the_callers_environment_reaches_the_child(account, monkeypatch):
    monkeypatch.setenv("SWBPIPE_NATIVE_SMOKE_INJECT_RESULT", "invented")
    monkeypatch.setenv("INVENTED_API_KEY", "invented")
    monkeypatch.setenv("DYLD_INVENTED", "invented")
    run, out = started_output(account, NAMES_ONLY, monkeypatch, {guard.DRIVER: "-"})
    names = set(out.split())
    assert not names & {"SWBPIPE_NATIVE_SMOKE_INJECT_RESULT", "INVENTED_API_KEY", "DYLD_INVENTED"}
    expected = {"PATH", "HOME", "CFFIXED_USER_HOME", "TMPDIR", "LANG", guard.STRICT_CHOOSER_ENV, guard.DRIVER}
    assert expected <= names and names - expected <= {"__CF_USER_TEXT_ENCODING"}, names - expected


def test_the_environment_is_built_from_the_run_alone(account, monkeypatch):
    no_writers(monkeypatch)
    run = new_run(account).begin()
    try:
        env = run.environment({guard.DRIVER: "-", guard.REPORT: guard.RUN_OWNED, guard.GATE: guard.RUN_OWNED,
                               guard.GATE_MODE: "handshake", guard.PROVIDER_KEY: "stdin"})
        assert env["HOME"] == env["CFFIXED_USER_HOME"] == str(run.home) and env["TMPDIR"] == str(run.tmp)
        assert env["PATH"] == guard.PATH == "/usr/bin:/bin" and env[guard.STRICT_CHOOSER_ENV] == "1"
        assert env[guard.REPORT] == str(run.work / "report.json") and env[guard.GATE] == str(run.work / "gate")
    finally:
        run.finish()


def test_the_child_works_in_the_runs_folder_whatever_the_callers_own(account, monkeypatch):
    elsewhere = account.tmp / "callers-own"
    elsewhere.mkdir()
    monkeypatch.chdir(elsewhere)
    run, out = started_output(account, "import os; print(os.getcwd())", monkeypatch)
    assert Path(out.strip()).resolve() == Path(run.work).resolve()


def test_no_inherited_descriptor_reaches_the_child(account, monkeypatch):
    left, right = socket.socketpair()
    handle = open(account.tmp / "invented-file", "w")
    numbers = (left.fileno(), right.fileno(), handle.fileno())
    for fd in numbers:
        os.set_inheritable(fd, True)
    try:
        run, out = started_output(account, "import os; print(' '.join(sorted(os.listdir('/dev/fd'))))", monkeypatch)
    finally:
        left.close(), right.close(), handle.close()
    descriptors = {int(d) for d in out.split()}
    assert min(numbers) > 3  # numbers the child could not mistake for its own
    assert descriptors <= {0, 1, 2, 3}, descriptors  # the standard streams and the listing's own


@pytest.mark.parametrize("option, refusal", [
    ({"pass_fds": (0,)}, "process_option_not_allowed:pass_fds"), ({"close_fds": False}, "process_option_not_allowed:close_fds"),
    ({"preexec_fn": lambda: None}, "process_option_not_allowed:preexec_fn"), ({"env": {}}, "process_option_not_allowed:env"),
    ({"cwd": "/"}, "process_option_not_allowed:cwd"), ({"start_new_session": True}, "process_option_not_allowed:start_new_session"),
    ({"stdout": 5}, "stream_not_allowed:stdout"), ({"stderr": subprocess.STDOUT}, "stream_not_allowed:stderr")])
def test_process_options_outside_the_allowlist_are_refused(account, monkeypatch, option, refusal):
    no_writers(monkeypatch)
    monkeypatch.setattr(guard, "_POPEN", lambda *a, **k: pytest.fail("started"))
    run = new_run(account).begin()
    try:
        with pytest.raises(guard.LaunchRefused) as raised:
            run.start("app", [str(account.exe)], **option)
        assert str(raised.value) == refusal
    finally:
        run.finish()


@pytest.mark.parametrize("name, value, refusal", [
    ("SWBPIPE_NATIVE_SMOKE_INJECT_RESULT", "x", "injection_variable"),
    ("SWBPIPE_NATIVE_SMOKE_INJECT_ROOT", "x", "injection_variable"),
    ("DYLD_INSERT_LIBRARIES", "x", "loader_or_xdg_variable"),
    ("LD_PRELOAD", "x", "loader_or_xdg_variable"),
    ("XDG_CONFIG_HOME", "x", "loader_or_xdg_variable"),
    ("HTTPS_PROXY", "x", "proxy_variable"),
    ("INVENTED_API_KEY", "x", "credential_like_variable"),
    ("INVENTED_TOKEN", "x", "credential_like_variable"),
    ("SOMETHING_ELSE", "x", "variable_not_on_the_list"),
    ("HOME", "/", "variable_not_on_the_list"),
    (guard.PROVIDER_KEY, "invented-secret", "provider_key_marker_only"),
    (guard.DRIVER, "/invented/driver.js", "driver_on_stdin_only"),
    (guard.REPORT, "/invented/Library/report.json", "report_and_gate_are_the_runs"),
    (guard.GATE, "/invented/socket", "report_and_gate_are_the_runs"),
    (guard.GATE_MODE, "timed", "gate_mode_not_known"),
])
def test_a_caller_cannot_pass_a_name_or_value_the_kit_does_not_own(name, value, refusal):
    with pytest.raises(guard.LaunchRefused) as raised:
        guard.allowed(name, value)
    assert str(raised.value).startswith(refusal)


PLAN = {"provider": {"endpoint": "http://127.0.0.1:9/v1", "model": "m", "contextWindow": 32768}, "turns": ["Build it"],
        "resultProbe": "node:c", "treeTypes": ["node"], "timeoutsMs": {"ui": 1, "start": 1, "turn": 1}}


@pytest.mark.parametrize("change", [
    lambda p: p.update(extra="x"),
    lambda p: p["provider"].update(endpoint="file:///invented"),
    lambda p: p["provider"].update(endpoint="unix:/invented.sock"),
    lambda p: p["provider"].update(endpoint="ws://127.0.0.1:9/v1"),
    lambda p: p.update(turns=["/invented/Documents/secret"]),
    lambda p: p.update(resultProbe="~/invented"),
    lambda p: p["timeoutsMs"].update(ui="1"),
    lambda p: p["provider"].update(socket="/invented.sock"),
])
def test_the_plan_is_held_to_its_closed_shape(change):
    plan = json.loads(json.dumps(PLAN))
    assert guard.validate_plan(json.loads(json.dumps(PLAN))) == PLAN
    change(plan)
    with pytest.raises(guard.LaunchRefused, match="plan_"):
        guard.validate_plan(plan)


@pytest.mark.parametrize("where, refusal", [
    (lambda a: a.real / "Desktop" / "ev", "evidence_under_desktop"),
    (lambda a: a.real / "Documents" / "ev", "evidence_under_documents"),
    (lambda a: a.real / "Downloads" / "ev", "evidence_under_downloads"),
    (lambda a: a.real / "Library" / "ev", "evidence_under_library"),
    (lambda a: Path(guard.__file__).resolve().parents[1] / "invented-evidence-folder", "evidence_under_the_repository"),
])
def test_an_evidence_folder_under_the_accounts_folders_or_the_repository_is_refused(account, where, refusal):
    path = where(account)
    if "invented-evidence-folder" not in str(path):
        path.mkdir()  # never created inside the repository: the location alone refuses
    with pytest.raises(guard.LaunchRefused, match=refusal):
        guard.check_evidence(path)


def test_an_evidence_folder_that_is_a_link_a_file_or_has_a_link_on_its_path_is_refused(account):
    link = account.tmp / "link"
    link.symlink_to(account.evidence)
    with pytest.raises(guard.LaunchRefused, match="evidence_not_a_real_folder"):
        guard.check_evidence(link)
    (account.evidence / "sub").mkdir()
    with pytest.raises(guard.LaunchRefused, match="evidence_path_has_a_link"):
        guard.check_evidence(link / "sub")
    (account.tmp / "file").write_bytes(b"x")
    with pytest.raises(guard.LaunchRefused, match="evidence_not_a_real_folder"):
        guard.check_evidence(account.tmp / "file")


def test_the_real_home_comes_from_the_password_database_not_the_environment(monkeypatch):
    monkeypatch.setenv("HOME", "/nowhere")
    import pwd
    assert guard.real_home() == Path(pwd.getpwuid(os.getuid()).pw_dir)


def test_the_helpers_own_utilities_are_absolute_and_get_a_minimal_environment(monkeypatch):
    seen = {}
    class Fake:
        returncode = 0
        def __init__(self, argv, **kwargs):
            seen.update(kwargs, argv=argv)
        def __enter__(self):
            return self
        def __exit__(self, *a):
            return False
        def communicate(self, timeout=None):
            return b"", b""
    monkeypatch.setenv("INVENTED_API_KEY", "invented")
    monkeypatch.setattr(guard, "_READ_POPEN", Fake)
    guard._read([guard.PS, "-axww"])
    assert seen["env"] == guard.UTIL_ENV and seen["cwd"] == "/" and seen["close_fds"] is True
    assert all(Path(p).is_absolute() for p in (guard.PS, guard.LSOF, guard.IOREG, guard.GETCONF))
    with pytest.raises(guard.LaunchRefused, match="utility_not_absolute"):
        guard._read(["ps"])


# ---------------------------------------------------------------------------
# 13 and 8. The strict chooser.

def test_an_executable_without_the_strict_chooser_name_is_refused(account, monkeypatch):
    no_writers(monkeypatch)
    plain = executable(account.tmp / "plain", marker=False)
    run = guard.Run(plain, account.evidence, "home", names=NAMES)
    run.library, run.cache, run.real = account.library, account.cache, account.real
    with pytest.raises(guard.LaunchRefused, match="strict_chooser_absent"):
        run.begin()
    run.finish()
    assert guard.strict_chooser_lead(account.exe) is True


def test_the_strict_chooser_lead_finds_the_name_across_a_read_boundary(tmp_path):
    path = tmp_path / "binary"
    needle = guard.STRICT_CHOOSER_ENV.encode()
    path.write_bytes(b"\0" * ((1 << 20) - 5) + needle + b"\0")
    assert guard.strict_chooser_lead(path) is True


def test_the_strict_chooser_constants_and_86_used_by_nothing_else():
    assert guard.STRICT_CHOOSER_ENV == "SWBPIPE_NATIVE_SMOKE_STRICT_CHOOSER" and guard.STRICT_CHOOSER_EXIT == 86
    assert 86 not in guard.EXIT.values() and 86 != journey.GUARD_EXIT


def test_the_strict_chooser_constants_equal_the_products_own_copy():
    source = NATIVE_SMOKE.read_text() if NATIVE_SMOKE.is_file() else ""
    env = re.search(r'STRICT_CHOOSER_ENV\s*:\s*&str\s*=\s*"([^"]+)"', source)
    code = re.search(r"STRICT_CHOOSER_EXIT\s*:\s*\w+\s*=\s*(\d+)", source)
    if not env or not code:
        pytest.skip("could not compare: the product's strict chooser constants are not in native_smoke.rs on this branch")
    assert (env.group(1), int(code.group(1))) == (guard.STRICT_CHOOSER_ENV, guard.STRICT_CHOOSER_EXIT)


# ---------------------------------------------------------------------------
# 5 and 14. The screen.

def answer(*entries):
    return plistlib.dumps({"IORegistryEntryName": "Root", "IOConsoleUsers": list(entries)})


UID = 4242
ON = {"kCGSSessionUserIDKey": UID, "kCGSSessionOnConsoleKey": True}


@pytest.mark.parametrize("given, refusal", [
    (answer(ON), None),
    (answer(ON, ON), None),
    (answer({**ON, "CGSSessionScreenIsLocked": True}), "screen_locked"),
    (answer({**ON, "CGSSessionScreenIsLocked": False}), "screen_locked"),
    (answer({**ON, "CGSSessionScreenIsLocked": 0}), "screen_locked"),
    (answer(ON, {**ON, "CGSSessionScreenIsLocked": True}), "screen_ambiguous"),
    (answer(ON, {**ON, "kCGSSessionOnConsoleKey": False}), "screen_ambiguous"),
    (answer(), "screen_no_session_for_this_account"),
    (answer({**ON, "kCGSSessionUserIDKey": UID + 1}), "screen_no_session_for_this_account"),
    (answer({**ON, "kCGSSessionOnConsoleKey": False}), "screen_not_on_console"),
    (answer({"kCGSSessionUserIDKey": UID}), "screen_not_on_console"),
    (plistlib.dumps({"IORegistryEntryName": "Root"}), "screen_unparsed"),
    (b"not a property list", "screen_unparsed"),
])
def test_the_screen_check_launches_only_on_one_found_unlocked_console_session(given, refusal):
    assert guard.screen_refusal(given, uid=UID) == refusal


def test_the_further_signs_are_recorded_as_not_used():
    assert set(guard.SIGNS_NOT_USED) == {"screenSaver", "displaySleep"}
    assert all(v.startswith("not used:") for v in guard.SIGNS_NOT_USED.values())


def test_a_launch_on_any_other_system_is_refused_before_anything_is_read(monkeypatch):
    monkeypatch.setattr(guard, "PLATFORM", "linux")
    monkeypatch.setattr(guard, "screen_refusal", lambda: pytest.fail("read"))
    with pytest.raises(guard.LaunchRefused, match="platform_not_macos"):
        guard.check_launch()


def test_a_launch_is_refused_on_either_check_and_records_whether_a_guard_folder_was_given(monkeypatch):
    monkeypatch.setattr(guard, "PLATFORM", "darwin")
    monkeypatch.setattr(guard, "screen_refusal", lambda: None)
    monkeypatch.setattr(guard, "slot_refusal", lambda guard_dir: "not_inside_the_slot_wrapper")
    with pytest.raises(guard.LaunchRefused, match="not_inside_the_slot_wrapper"):
        guard.check_launch()
    monkeypatch.setattr(guard, "slot_refusal", lambda guard_dir: None)
    monkeypatch.setattr(guard, "screen_refusal", lambda: "screen_locked")
    with pytest.raises(guard.LaunchRefused, match="screen_locked"):
        guard.check_launch()
    monkeypatch.setattr(guard, "screen_refusal", lambda: None)
    assert guard.check_launch("/invented/guard")["guardDirGiven"] is True and guard.check_launch()["guardDirGiven"] is False


# 6. The slot.
ME, SHELL, LOCK, TOP = 500, 400, 300, 200


def table(lock_exe="/usr/bin/lockf", lock_uid=None):
    uid = os.getuid() if lock_uid is None else lock_uid
    return {ME: (SHELL, ME, os.getuid(), sys.executable, "s"), SHELL: (LOCK, LOCK, os.getuid(), "/bin/bash", "s"),
            LOCK: (TOP, LOCK, uid, lock_exe, "s"), TOP: (1, TOP, os.getuid(), "/bin/bash", "s")}


def opens(*names):
    return lambda pid: list(names) if pid == LOCK else []


@pytest.mark.parametrize("given, files, guard_dir, refusal", [
    (table(), opens("/invented/guard/cargo_job.slot2.lock"), None, None),
    (table(), opens("/invented/guard/cargo_job.lock"), None, None),
    (table(), opens("/invented/guard/cargo_job.slot2.lock"), "/invented/guard", None),
    (table(), opens("/invented/guard/cargo_job.slot2.lock"), "/invented/other", "not_inside_the_slot_wrapper_for_that_guard_folder"),
    (table(), opens("/invented/guard/something.lock"), None, "not_inside_the_slot_wrapper"),
    (table(lock_exe="/bin/bash"), opens("/invented/guard/cargo_job.lock"), None, "not_inside_the_slot_wrapper"),
    (table(lock_uid=0), opens("/invented/guard/cargo_job.lock"), None, "not_inside_the_slot_wrapper"),
    ({ME: (1, ME, os.getuid(), sys.executable, "s")}, opens(), None, "not_inside_the_slot_wrapper"),
])
def test_the_slot_check_recognises_the_lock_program_holding_a_slot_lock(given, files, guard_dir, refusal):
    assert guard.slot_refusal(guard_dir, table=given, open_names=files, start=SHELL) == refusal


def test_an_unreadable_process_table_refuses_the_slot():
    def broken(pid):
        raise OSError("invented")
    assert guard.slot_refusal(None, table=table(), open_names=broken, start=SHELL) == "slot_unreadable"


# ---------------------------------------------------------------------------
# 3, 9, 10. The lister and the comparison.

def compared(account, act, run=None, until=None, keep_open=False):
    run = run or new_run(account).begin()
    web_view_data(run)
    act()
    report = guard.compare(run.before, run.home, until=until, library=account.library, home=account.real, cache=account.cache)
    if not keep_open:
        run.finish()
    return report


@pytest.fixture
def quiet(account, monkeypatch):
    no_writers(monkeypatch)
    return account


WATCHED = ("WebKit", "Caches", "HTTPStorages", "Application Support", "Preferences", "Saved Application State", "Logs")


@pytest.mark.parametrize("top", WATCHED)
@pytest.mark.parametrize("name", NAMES)
@pytest.mark.parametrize("change", ["created", "modified", "removed"])
def test_each_change_under_each_watched_folder_for_each_name_is_a_product_change(quiet, top, name, change):
    existing = place(quiet.library, top, name) if change != "created" else None
    report = compared(quiet, {"created": lambda: place(quiet.library, top, name, "new.bin"),
                              "modified": lambda: existing.write_bytes(b"longer"),
                              "removed": lambda: existing.unlink()}[change])
    assert report["verdict"] == "product" and change in [c["change"] for c in report["product"]["changes"]]


def test_a_name_four_levels_down_and_with_a_dot_suffix_is_watched(quiet):
    def act():
        (quiet.library / "Preferences" / f"{BUNDLE}.plist").write_bytes(b"p")
        (quiet.library / "Caches" / "a" / "b" / "c" / EXE).mkdir(parents=True)
    report = compared(quiet, act)
    assert {c["path"] for c in report["product"]["changes"]} >= {f"Preferences/{BUNDLE}.plist", f"Caches/a/b/c/{EXE}"}


def test_every_other_top_folder_is_watched_one_level_deep_by_name(quiet):
    (quiet.library / "Invented Top" / "deeper").mkdir(parents=True)
    def act():
        (quiet.library / "Invented Top" / EXE).mkdir()
        (quiet.library / "Invented Top" / "deeper" / BUNDLE).mkdir()
    report = compared(quiet, act)
    assert [c["path"] for c in report["product"]["changes"]] == [f"Invented Top/{EXE}"]


def test_a_watched_top_folder_that_is_a_link_refuses_at_the_baseline(quiet):
    elsewhere = quiet.tmp / "documents-like"
    elsewhere.mkdir()
    (elsewhere / "private").write_bytes(b"x")
    (quiet.library / "Caches").rmdir()
    (quiet.library / "Caches").symlink_to(elsewhere)
    with pytest.raises(guard.LaunchRefused, match="baseline_folder_not_listed:Caches"):
        new_run(quiet).begin()


def test_an_unreadable_watched_top_folder_refuses_at_the_baseline_and_fails_the_comparison(quiet):
    (quiet.library / "HTTPStorages").chmod(0)
    try:
        with pytest.raises(guard.LaunchRefused, match="baseline_folder_not_listed:HTTPStorages"):
            new_run(quiet).begin()
    finally:
        (quiet.library / "HTTPStorages").chmod(0o700)
    run = new_run(quiet).begin()
    report = compared(quiet, lambda: (quiet.library / "HTTPStorages").chmod(0), run, keep_open=True)
    (quiet.library / "HTTPStorages").chmod(0o700)
    run.finish()
    assert report["verdict"] == "comparison failed" and report["failure"]["foldersNotListed"] == ["HTTPStorages"]


def test_an_unreadable_product_folder_refuses_at_the_baseline(quiet):
    folder = place(quiet.library, "WebKit", BUNDLE).parent
    folder.chmod(0)
    try:
        with pytest.raises(guard.LaunchRefused, match="baseline_product_folder_unreadable"):
            new_run(quiet).begin()
    finally:
        folder.chmod(0o700)


def test_an_unreadable_folder_not_product_named_is_counted_never_named_and_a_changed_count_fails(quiet):
    other = place(quiet.library, "Application Support", "invented-private-name").parent
    other.chmod(0)
    try:
        run = new_run(quiet).begin()
        assert run.before["unreadableOther"] == 1 and "invented-private-name" not in json.dumps(run.before)
        same = compared(quiet, lambda: None, run, keep_open=True)
        assert same["verdict"] == "clean" and same["limit"]["statement"] == "1 folders could not be listed and were not watched"
        changed = compared(quiet, lambda: other.chmod(0o700), run)
    finally:
        other.chmod(0o700)
    assert changed["verdict"] == "comparison failed" and "invented-private-name" not in json.dumps(changed)


def test_a_link_inside_a_watched_folder_is_listed_and_not_followed(quiet):
    outside = quiet.tmp / "outside"
    (outside / "deep").mkdir(parents=True)
    (quiet.library / "Caches" / BUNDLE).mkdir()
    (quiet.library / "Caches" / BUNDLE / "link").symlink_to(outside)
    run = new_run(quiet).begin()
    entries = run.before["entries"]
    run.finish()
    assert entries[f"Caches/{BUNDLE}/link"]["kind"] == "link" and not any("deep" in p for p in entries)


def test_others_are_counted_named_only_under_preferences_and_never_fail(quiet):
    def act():
        (quiet.library / "Preferences" / "com.invented.service.plist").write_bytes(b"s")
        place(quiet.library, "Caches", "invented-other-app")
        place(quiet.library, "Application Support", "invented-other-app", "private-name.db")
        place(quiet.library, "Caches", f"{BUNDLE}-other")
    report = compared(quiet, act, until=time.time() + 5)
    assert report["verdict"] == "clean" and report["class"] == "others"
    assert [n["path"] for n in report["others"]["named"]] == ["Preferences/com.invented.service.plist"]
    assert "private-name" not in json.dumps(report)
    assert "Nothing outside the real Library's watched folders" in report["watched"]


def test_no_web_view_data_has_its_own_verdict_and_its_limit_is_stated(quiet):
    run = new_run(quiet).begin()
    report = guard.compare(run.before, run.home, library=quiet.library, home=quiet.real, cache=quiet.cache)
    run.finish()
    assert report["verdict"] == "no web view data" and report["throwawayHome"]["webViewDataLimit"] == guard.POSITIVE_LIMIT


@pytest.mark.parametrize("component, expected", [
    ("com.apple.WebKit.Networking+" + EXE, True),
    ("com.apple.WebKit.GPU+" + EXE + "-1a2b3c", True),
    ("com.apple.WebKit.WebContent+" + BUNDLE, True),
    ("com.apple.WebKit.WebContent.EnhancedSecurity+" + BUNDLE + ".helper", True),
    ("com.apple.WebKit.Networking+" + EXE.replace("-", "_") + "-1a2b3c4d5e", True),
    ("com.apple.WebKit.Networking+" + EXE + "x", False),
    ("com.apple.WebKit.Networking+" + EXE.replace("-", "_") + "x", False),
    ("com.apple.WebKit.Networking+com.invented.other", False),
])
def test_a_watched_name_after_a_plus_sign_is_watched(component, expected):
    assert guard.watched(f"{guard.CACHE_TOP}/{component}/data", NAMES) is expected


def test_the_web_view_helpers_cache_folders_are_their_own_class_and_always_stop_the_run(quiet):
    assert not hasattr(guard, "HELPER_CACHE_COUNTS_AS_CLEAN")
    report = compared(quiet, lambda: in_cache(quiet, HELPER, "data"))
    assert report["verdict"] == "web view helper cache" and report["class"] == "web view helper cache"
    assert report["product"]["changes"] == [] and len(report["helperCache"]["changes"]) == 2


@pytest.mark.parametrize("parts", [(BUNDLE, "data"), (EXE, "data"), ("invented", "com.apple.WebKit.Networking+" + EXE, "data"),
                                   ("com.apple.WebKit.Invented+" + EXE, "data")])
def test_a_product_named_entry_of_any_other_shape_in_the_cache_folder_trips_the_wire(quiet, parts):
    report = compared(quiet, lambda: in_cache(quiet, *parts))
    assert report["verdict"] == "product" and report["helperCache"]["changes"] == []


def test_the_cache_folder_is_named_by_what_it_is_never_by_its_path(quiet):
    in_cache(quiet, BUNDLE, "data")
    before = guard.snapshot(NAMES, quiet.library, quiet.real, quiet.cache)
    assert f"{guard.CACHE_TOP}/{BUNDLE}/data" in before["entries"] and str(quiet.cache) not in json.dumps(before)


def test_the_cache_folder_fails_closed(quiet, monkeypatch):
    run = new_run(quiet).begin()
    report = compared(quiet, lambda: quiet.cache.chmod(0), run, keep_open=True)
    quiet.cache.chmod(0o700)
    monkeypatch.setattr(guard, "user_cache_dir", lambda: None)
    run.cache = None
    missing = guard.compare(run.before, run.home, library=quiet.library, home=quiet.real)
    run.finish()
    assert report["verdict"] == "comparison failed" and report["failure"]["foldersNotListed"] == [guard.CACHE_TOP]
    assert missing["failure"]["foldersNotListed"] == [guard.CACHE_TOP]


def test_the_four_home_folders_are_stat_only(quiet, monkeypatch):
    for folder in ("Desktop", "Documents", "Downloads"):
        (quiet.real / folder).chmod(0)
    listed = []
    original = os.listdir
    monkeypatch.setattr(guard.os, "listdir", lambda p=".": listed.append(p) or original(p))
    try:
        times = guard.folder_times()
    finally:
        for folder in ("Desktop", "Documents", "Downloads"):
            (quiet.real / folder).chmod(0o700)
    assert set(times) == {"~", "Desktop", "Documents", "Downloads"} and all(isinstance(t, float) for t in times.values())
    assert listed == []


def test_a_change_of_the_four_times_is_reported_not_concluded_from(quiet):
    report = compared(quiet, lambda: os.utime(quiet.real / "Downloads", (1, 1)))
    assert report["folderTimes"]["changed"] == ["Downloads"] and report["verdict"] == "clean"


# ---------------------------------------------------------------------------
# 8. Real termination signals delivered to a run (the test's own short-lived processes).

SIGNALLED = textwrap.dedent("""
    import json, os, sys, time
    from pathlib import Path
    sys.path.insert(0, sys.argv[1])
    from tools import native_launch_guard as guard
    root = Path(sys.argv[2])
    guard.real_home = lambda: root / "real-home"
    guard.temp_root = lambda: (root / "system-temp").resolve()
    guard.user_cache_dir = lambda: (root / "system-cache").resolve()
    guard.check_launch = lambda *a: {"launch": "invented"}
    guard.possible_writers = lambda *a, **k: ([], {"statement": guard.RESIDUAL_STATEMENT})
    guard.SETTLE_SECONDS = 2
    guard.SECOND_LOOK_SECONDS = 0
    run = guard.Run(sys.argv[3], root / "evidence", "home", names=["invented-product"])
    run.library, run.cache, run.real = root / "real-home" / "Library", root / "system-cache", root / "real-home"
    try:
        run.begin()
        child = run.start("app", [sys.argv[3]])
        child.wait()
        print(json.dumps({"home": str(run.home)}), flush=True)
        if sys.argv[4] == "settle":
            run.finish()
        else:
            time.sleep(30)
    finally:
        run.finish()
""")


@pytest.mark.parametrize("signum", [signal.SIGTERM, signal.SIGINT, signal.SIGHUP])
@pytest.mark.parametrize("when", ["running", "settle"])
def test_a_termination_signal_still_persists_the_comparison_and_removes_the_home(account, signum, when):
    script = account.tmp / "signalled.py"
    script.write_text(SIGNALLED)
    process = subprocess.Popen([sys.executable, "-I", str(script), str(Path(guard.__file__).resolve().parents[1]),
                                str(account.tmp), str(account.exe), when], stdout=subprocess.PIPE, stderr=subprocess.PIPE,
                               start_new_session=True)
    home = Path(json.loads(process.stdout.readline())["home"])
    time.sleep(0.5)
    process.send_signal(signum)
    process.wait(timeout=60)
    assert process.returncode != 0
    assert evidence_file(account, "home-comparison.json")["compared"] is True
    assert evidence_file(account, "home-run.json")["throwawayHome"]["cleanup"] == "removed" and not home.exists()


# ---------------------------------------------------------------------------
# The command line, and the controllers.

def test_the_command_line_offers_no_start_and_reports_the_inventory(account, monkeypatch, capsys):
    for removed in ("spec", "make-home", "snapshot", "compare", "remove-home"):
        with pytest.raises(SystemExit):
            guard.main([removed])
    guard.Kit().close()
    assert guard.main(["left-homes"]) == 0
    assert json.loads(capsys.readouterr().out.strip().splitlines()[-1])["leftHomes"] == []
    assert guard.EXIT == {"clean": 0, "product": 1, "refused": 2, "no web view data": 3, "not removed": 4,
                          "comparison failed": 5, "web view helper cache": 6, "reappeared": 7}


class Started:
    pid = 2 ** 22 + 7  # an identifier no process here holds

    def __init__(self, argv, env=None, cwd=None, **kwargs):
        self.argv, self.env, self.cwd, self.kwargs = argv, env, cwd, kwargs

    def poll(self):
        return 0


def journey_guard(account, monkeypatch):
    no_writers(monkeypatch)
    monkeypatch.setattr(journey.LaunchGuard, "library", account.library)
    monkeypatch.setattr(journey.LaunchGuard, "cache", account.cache)
    monkeypatch.setattr(journey.LaunchGuard, "real", account.real)
    monkeypatch.setattr(guard, "_POPEN", Started)
    root = account.evidence
    return journey.LaunchGuard(account.exe, root), account.exe, root


def test_every_start_the_journey_makes_carries_the_runs_environment(account, monkeypatch):
    launch_guard, app, root = journey_guard(account, monkeypatch)
    with pytest.raises(journey.Refusal, match="launch_refused_run_not_open"):
        launch_guard.launch("preflight", [str(app)], {})
    launch_guard.start()
    for role in ("preflight", "app"):
        started = launch_guard.launch(role, [str(app)], {guard.DRIVER: "-"})
        assert started.env["HOME"] == started.env["CFFIXED_USER_HOME"] == str(launch_guard.home)
        assert started.cwd == str(launch_guard.run.work) and started.kwargs["close_fds"] is True
    assert launch_guard.launches == ["preflight", "app"] and set(launch_guard.names) >= {EXE, BUNDLE}
    result = {}
    launch_guard.finish(result)
    assert result["throwawayHome"]["cleanup"] == "removed"


def test_the_journey_refuses_a_start_with_a_variable_the_kit_does_not_own(account, monkeypatch):
    launch_guard, app, root = journey_guard(account, monkeypatch)
    launch_guard.start()
    with pytest.raises(journey.Refusal, match="launch_refused_injection_variable"):
        launch_guard.launch("app", [str(app)], {"SWBPIPE_NATIVE_SMOKE_INJECT_RESULT": "x"})
    with pytest.raises(journey.Refusal, match="launch_refused_report_and_gate_are_the_runs"):
        launch_guard.launch("app", [str(app)], {guard.REPORT: str(root / "report.json")})
    assert launch_guard.launches == []
    launch_guard.finish({})


def test_the_preflight_goes_through_the_guard_and_keeps_its_state_in_the_home(account, monkeypatch):
    launch_guard, app, root = journey_guard(account, monkeypatch)
    launch_guard.start()
    seen = []
    monkeypatch.setattr(launch_guard, "launch", lambda role, argv, extra, **popen: seen.append((role, argv)) or (_ for _ in ()).throw(journey.Refusal("stop")))
    args = SimpleNamespace(app=app, process_probe=Path("p"), boundary_probe=Path("b"))
    with pytest.raises(journey.Refusal):
        journey.preflight(args, root, launch_guard)
    assert seen == [("preflight", [str(app), "--native-agent-session-fixture"])]
    assert (launch_guard.run.work / "preflight").is_dir()
    launch_guard.finish({})


LAUNCH_CALLS = re.compile(r"^(subprocess\.(Popen|run|call|check_call|check_output|getoutput|getstatusoutput)|"
                          r"os\.(system|popen|spawn\w*|exec\w*|posix_spawn\w*|fork\w*)|pty\.spawn|Popen|asyncio\.create_subprocess_\w+)$")


@pytest.mark.parametrize("module, allowed", [(journey, {"residents"}), (live, {"anchor_start"})])
def test_the_controllers_start_nothing_but_through_the_helper(module, allowed):
    """Every way of starting a process, not only Popen: the journey reads the process table in
    `residents` and the live-session fixture starts its anchor probe; nothing else."""
    tree = ast.parse(Path(module.__file__).read_text())
    starts = {node.name for node in ast.walk(tree) if isinstance(node, ast.FunctionDef)
              for call in ast.walk(node) if isinstance(call, ast.Call) and LAUNCH_CALLS.match(ast.unparse(call.func))}
    top_level = [call for call in ast.walk(tree) if isinstance(call, ast.Call) and LAUNCH_CALLS.match(ast.unparse(call.func))]
    assert starts == allowed and len(top_level) == len(allowed), (module.__name__, starts)


def test_the_live_session_fixture_removes_nothing_outside_the_helper():
    source = Path(live.__file__).read_text()
    assert not re.search(r"rmtree|\.unlink\(|os\.remove|os\.rmdir|\.rmdir\(", source)


def run_ending(account, monkeypatch, ending):
    _, app, root = journey_guard(account, monkeypatch)
    monkeypatch.setattr(journey, "sha256", lambda p: "0" * 64)
    def journey_body(args, key, root, launch_guard, result):
        launch_guard.launch("app", [str(app)], {})
        ending(result)
    monkeypatch.setattr(journey, "journey", journey_body)
    return root, (lambda: journey.run(SimpleNamespace(app=app, mode="stub"), None, root))


def test_a_failed_run_and_an_interrupted_run_leave_no_home_and_persist_the_comparison(account, monkeypatch):
    def refused(result):
        raise journey.Refusal("preflight")
    root, call = run_ending(account, monkeypatch, refused)
    result = call()
    assert result["failure"] == "preflight" and result["launchGuard"]["compared"] is True
    assert result["throwawayHome"]["cleanup"] == "removed" and not Path(result["throwawayHome"]["home"]).exists()
    assert (root / "home-comparison.json").is_file() and (root / "home-listing.json").is_file()


def test_an_interrupt_during_the_journey_still_finishes_the_run(account, monkeypatch):
    def interrupted(result):
        raise guard.Interrupted(signal.SIGTERM)
    root, call = run_ending(account, monkeypatch, interrupted)
    with pytest.raises(guard.Interrupted):
        call()
    record = json.loads((root / "home-run.json").read_text())
    assert record["throwawayHome"]["cleanup"] == "removed" and (root / "home-comparison.json").is_file()


def test_a_product_change_fails_the_run_even_on_refusal(account, monkeypatch):
    def leaked(result):
        place(account.library, "WebKit", BUNDLE)
        raise journey.Refusal("preflight")
    root, call = run_ending(account, monkeypatch, leaked)
    result = call()
    assert result["launchGuard"]["verdict"] == "product" and journey.require_comparison(result) == journey.GUARD_EXIT


def test_a_run_whose_processes_may_be_alive_withholds_its_home_fails_and_names_it(account, monkeypatch):
    def retained(result):
        result["rescue"] = "retained_original_app_custody"
    root, call = run_ending(account, monkeypatch, retained)
    result = call()
    home = result["throwawayHome"]["home"]
    assert result["throwawayHome"]["cleanup"] == "withheld" and Path(home).is_dir()
    assert journey.require_comparison(result) == journey.GUARD_EXIT and home in result["failure"]


def test_no_result_reads_as_a_pass_without_a_clean_comparison():
    assert journey.require_comparison({"passed": True}) == journey.GUARD_EXIT
    for verdict in ("product", "no web view data", "comparison failed", "web view helper cache"):
        result = {"passed": True, "launchGuard": {"compared": True, "verdict": verdict}}
        assert journey.require_comparison(result) == journey.GUARD_EXIT and result["passed"] is False
    clean = {"passed": True, "launchGuard": {"compared": True, "verdict": "clean"}}
    assert journey.require_comparison(clean) is None and clean["passed"] is True


def test_main_writes_no_result_without_the_comparison(tmp_path, monkeypatch, capsys):
    monkeypatch.setattr(journey, "RETAINED", tmp_path / "retained")
    monkeypatch.setattr(journey.fcntl, "flock", lambda *a: None)
    real_open = open
    monkeypatch.setattr(journey, "open", lambda path, *a, **kw: real_open(tmp_path / "lock", *a, **kw), raising=False)
    for name in ("app", "probe"):
        (tmp_path / name).touch()
    monkeypatch.setattr(journey, "free_bytes", lambda: 10 ** 15)
    monkeypatch.setattr(journey, "run", lambda *a: {"passed": True, "appSha256": "0" * 64})
    monkeypatch.setattr(sys, "argv", ["agent_journey_fixture.py", "--app", str(tmp_path / "app"), "--process-probe",
                                      str(tmp_path / "probe"), "--boundary-probe", str(tmp_path / "probe"),
                                      "--mode", "stub", "--output", str(tmp_path / "out")])
    assert journey.main() == journey.GUARD_EXIT
    written = json.loads((tmp_path / "out" / "result.json").read_text())
    assert written["passed"] is False and written["failure"] == "launch_guard: not compared"
    assert json.loads(capsys.readouterr().out.strip().splitlines()[-1])["launchGuard"] is None


def test_every_live_session_case_runs_inside_the_lifecycle(account, monkeypatch):
    seen = {}
    def started(argv, env=None, cwd=None, **kwargs):
        seen.update(env=env, cwd=cwd)
        raise RuntimeError("not launched in an offline test")
    no_writers(monkeypatch)
    monkeypatch.setattr(guard, "_POPEN", started)
    monkeypatch.setattr(guard.Run, "library", account.library)
    monkeypatch.setattr(guard.Run, "cache", account.cache)
    monkeypatch.setattr(guard.Run, "real", account.real)
    args = SimpleNamespace(cancel_file=None, app=account.exe, model="m", context_window=1, endpoint="e",
                           process_probe=Path("p"), boundary_probe=Path("b"))
    result = live.run_case(args, "case", "stub", "none", account.evidence)
    case = account.evidence / "case"
    assert seen["env"]["HOME"] == seen["env"]["CFFIXED_USER_HOME"] and seen["cwd"].endswith("/work")
    assert result["failure"] == "RuntimeError" and result["throwawayHome"]["cleanup"] == "removed"
    assert (case / "home-comparison.json").is_file() and result["launchGuard"]["compared"] is True
