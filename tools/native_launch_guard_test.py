"""Offline checks of the native launch helper and the controllers that use it. Nothing here starts
the application: the processes started are short-lived stand-ins of the tests' own making, from an
invented executable inside an invented bundle. The account's real Library, home folders, temporary
and cache folders and screen are never read: every check gets invented ones, and every name in
them is invented. The helper removes nothing, and neither do these tests outside their own folders."""
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
MACOS = pytest.mark.skipif(sys.platform != "darwin", reason="the holder check is made on macOS only")
HELPER = "com.apple.WebKit.Networking+" + EXE.replace("-", "_") + "-1a2b3c4d5e"
SLEEPS = "import time; time.sleep(30)"


def executable(folder, body="import sys; sys.exit(0)", marker=True, bundle=BUNDLE, named=EXE):
    """An invented executable inside an invented bundle: a Python script, carrying the strict chooser
    variable's name in a comment when `marker`, with an Info.plist naming it."""
    contents = folder / "Copy.app" / "Contents"
    path = contents / "MacOS" / EXE
    path.parent.mkdir(parents=True, exist_ok=True)
    (contents / "Info.plist").write_bytes(plistlib.dumps({"CFBundleIdentifier": bundle, "CFBundleExecutable": named}))
    comment = f"# {guard.STRICT_CHOOSER_ENV}\n" if marker else "\n"
    path.write_text(f"#!{sys.executable} -I\n{comment}{body}\n")
    path.chmod(0o755)
    return path


def open_up(path):
    """Give back owner access to every folder under `path`, so that pytest's own cleanup never meets a
    folder a test left unreadable; links are not followed."""
    try:
        os.chmod(path, 0o700)
        entries = list(os.scandir(path))
    except OSError:
        return
    for entry in entries:
        if entry.is_dir(follow_symlinks=False):
            open_up(entry.path)


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
    probe = tmp_path / "probe"
    probe.write_bytes(b"#!/bin/sh\n")
    monkeypatch.setattr(guard, "real_home", lambda: real)
    monkeypatch.setattr(guard, "temp_root", lambda: temp.resolve())
    monkeypatch.setattr(guard, "user_cache_dir", lambda: cache.resolve())
    monkeypatch.setattr(guard, "check_launch", lambda *a: {"launch": "invented"})
    monkeypatch.setattr(guard, "SETTLE_SECONDS", 0)
    yield SimpleNamespace(real=real, library=real / "Library", temp=temp.resolve(), cache=cache.resolve(),
                          evidence=evidence.resolve(), exe=executable(tmp_path), tmp=tmp_path, probe=probe.resolve())
    open_up(tmp_path)  # whatever the test ended by, nothing it made stays unremovable by pytest


def quiet(monkeypatch, table=None, opened=()):
    """A deterministic process listing: this process alone (or `table`), and the given open holders."""
    monkeypatch.setattr(guard, "PLATFORM", "darwin")
    mine = {os.getpid(): (1, 1, os.getuid(), "/invented/python", "2026-01-01T00:00:00")}
    monkeypatch.setattr(guard, "_process_table", lambda: {**mine, **(table or {})})
    monkeypatch.setattr(guard, "_open_under", lambda path: set(opened))


def new_run(account, label="home", evidence=None, exe=None):
    run = guard.Run(exe or account.exe, evidence or account.evidence, label)
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


def evidence_file(account, name, folder=None):
    return json.loads(((folder or account.evidence) / name).read_text())


def homes(account):
    kit = guard.Kit()
    try:
        return guard.inventory(kit)
    finally:
        kit.close()


# ---------------------------------------------------------------------------
# The split: the helper removes nothing, and judges nothing about removal.

REMOVALS = re.compile(r"^(os\.(rmdir|unlink|remove|removedirs)|shutil\.rmtree|\w+\.(rmdir|unlink)|rmtree)$")


def test_the_helper_removes_nothing():
    for name in ("settle", "remove_left_home", "remove_at", "_empty", "_rmdir_checked", "possible_writers", "Custody"):
        assert not hasattr(guard, name), name
    tree = ast.parse(Path(guard.__file__).read_text())
    removals = [ast.unparse(call) for call in ast.walk(tree) if isinstance(call, ast.Call)
                and REMOVALS.match(ast.unparse(call.func))]
    assert removals == ["os.unlink(scratch, dir_fd=self.registry_fd)"], removals  # the registry's own scratch file


def test_neither_tool_removes_anything_outside_the_helper():
    for module in (journey, live):
        source = Path(module.__file__).read_text()
        assert not re.search(r"rmtree|\.unlink\(|os\.remove|os\.rmdir|\.rmdir\(", source), module.__name__


def test_a_finished_run_leaves_its_home_in_place_marked_finished(account, monkeypatch):
    quiet(monkeypatch)
    run = new_run(account).begin()
    home = run.home
    outcome = run.finish()
    assert home.is_dir() and outcome["throwawayHome"]["state"] == "finished" and outcome["throwawayHome"]["removed"] is False
    record = evidence_file(account, "home-run.json")
    assert record["sockets"] == guard.SOCKETS_STATEMENT and record["confinement"] == guard.CONFINEMENT_STATEMENT
    assert record["finished"] == guard.FINISHED_STATEMENT and record["admissions"]
    assert "residual" not in record and (account.evidence / "home-comparison.json").is_file()
    assert [h["state"] for h in homes(account)["homes"]] == ["finished"]


def test_a_finished_home_is_reported_at_every_admission_and_does_not_block_by_itself(account, monkeypatch):
    quiet(monkeypatch)
    first = new_run(account, "first").begin()
    first.finish()
    second = new_run(account, "second").begin()
    admitted = second.admissions[0]["homes"]
    second.finish()
    assert admitted["count"] == 1 and admitted["byState"] == {"finished": 1}
    assert [h["home"] for h in admitted["named"]] == [first.entry["home"]]


def test_finished_does_not_mean_safe_to_remove():
    assert guard.STATES == ("creating", "open", "finished", "left")
    assert "does not mean safe to remove" in guard.Run._mark.__doc__ and "never means" in guard.__doc__


def test_the_homes_command_lists_every_home_by_name_and_state_with_a_count(account, monkeypatch, capsys):
    quiet(monkeypatch)
    new_run(account, "one").begin().finish()
    new_run(account, "two").begin().finish(retain=True)
    kit = guard.Kit()
    os.mkdir("swbpipe-home-shaped", dir_fd=kit.homes_fd)
    kit.close()
    assert guard.main(["homes"]) == 0
    listed = json.loads(capsys.readouterr().out)
    assert listed["count"] == 3 and listed["byState"] == {"finished": 1, "left": 1, "unknown custody": 1}
    assert "test data only" in listed["statement"] and "none outlives" in listed["statement"]


# ---------------------------------------------------------------------------
# Admission: what blocks, exactly as decided.

def finished_with(account, monkeypatch, generation, label="first"):
    quiet(monkeypatch)
    run = new_run(account, label).begin()
    run.entry["started"] = [generation]
    run.kit.write_entry(run.entry)
    run.finish()
    return run


def test_a_home_whose_recorded_generation_is_alive_blocks_admission(account, monkeypatch):
    first = finished_with(account, monkeypatch, {"pid": 4242, "start": "2026-01-01T00:00:00", "role": "app"})
    quiet(monkeypatch, {4242: (1, 4242, os.getuid(), "/invented", "2026-01-01T00:00:00")})
    with pytest.raises(guard.LaunchRefused, match="a_home_of_this_kit_may_be_held"):
        new_run(account, "second").begin()
    quiet(monkeypatch, {4242: (1, 4242, os.getuid(), "/invented", "2026-02-02T00:00:00")})  # a later generation
    third = new_run(account, "third").begin()
    third.finish()
    assert first.home.is_dir()


def test_a_home_whose_start_time_was_not_read_stays_unestablished_and_blocks(account, monkeypatch):
    finished_with(account, monkeypatch, {"pid": 4242, "start": None, "role": "app"})
    quiet(monkeypatch, {4242: (1, 1, os.getuid(), "/invented", "2026-01-01T00:00:00")})
    with pytest.raises(guard.LaunchRefused, match="unestablished"):
        new_run(account, "second").begin()


def test_an_inventory_that_cannot_list_processes_blocks(account, monkeypatch):
    finished_with(account, monkeypatch, {"pid": 1, "start": "x", "role": "app"})
    monkeypatch.setattr(guard, "_process_table", lambda: (_ for _ in ()).throw(OSError("invented")))
    with pytest.raises(guard.LaunchRefused, match="unestablished"):
        new_run(account, "second").begin()


def test_a_surviving_recorded_group_with_everything_outside_the_home_blocks(account, monkeypatch):
    quiet(monkeypatch)
    run = new_run(account, "first").begin()
    run.attribute(group=777)
    run.finish()
    quiet(monkeypatch, {778: (1, 777, os.getuid(), "/invented/elsewhere", "2026-01-01T00:00:00")})
    with pytest.raises(guard.LaunchRefused, match="alive"):
        new_run(account, "second").begin()


def test_unknown_custody_is_reported_and_blocks_while_held_or_unestablished(account, monkeypatch):
    quiet(monkeypatch)
    kit = guard.Kit()
    os.mkdir("swbpipe-home-shaped", dir_fd=kit.homes_fd)
    kit.close()
    run = new_run(account, "one").begin()
    assert [h["state"] for h in run.admissions[0]["homes"]["named"]] == ["unknown custody"]
    run.finish()
    quiet(monkeypatch, {999: (1, 999, os.getuid(), "/invented", "x")}, opened=[999])
    with pytest.raises(guard.LaunchRefused, match="unknown custody, alive"):
        new_run(account, "two").begin()
    assert (account.temp / guard.KIT_DIR / "homes" / "swbpipe-home-shaped").is_dir()


def test_a_home_made_before_its_record_was_complete_enters_the_inventory(account, monkeypatch):
    quiet(monkeypatch)
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
    assert outcome["throwawayHome"]["state"] == "creating" and run.home.is_dir()
    assert [(h["home"], h["recordedState"]) for h in homes(account)["homes"]] == [(run.entry["home"], "creating")]


# (finding 2) The holder check is bound to the held folder; a stale path is unestablished, never a pass.
def test_a_renamed_kit_folder_leaves_the_check_unestablished_and_the_run_left(account, monkeypatch):
    quiet(monkeypatch)
    run = new_run(account).begin()
    (account.temp / guard.KIT_DIR).rename(account.temp / "moved-kit")
    outcome = run.finish()
    assert outcome["throwawayHome"]["state"] == "left"
    assert "no longer identifies" in outcome["throwawayHome"]["holders"]["reason"]


def test_a_kit_folder_replaced_by_a_link_reaches_nothing_foreign(account, monkeypatch):
    quiet(monkeypatch)
    run = new_run(account).begin()
    name = run.entry["home"]
    (account.temp / guard.KIT_DIR).rename(account.temp / "moved-kit")
    foreign = account.tmp / "foreign"
    (foreign / "homes" / name).mkdir(parents=True)
    (foreign / "homes" / name / "kept").write_bytes(b"x")
    (account.temp / guard.KIT_DIR).symlink_to(foreign)
    outcome = run.finish()
    assert outcome["throwawayHome"]["state"] == "left" and (foreign / "homes" / name / "kept").is_file()
    assert (account.temp / "moved-kit" / "homes" / name).is_dir()


def test_a_start_after_the_kit_folder_was_replaced_by_a_link_is_refused(account, monkeypatch):
    quiet(monkeypatch)
    run = new_run(account).begin()
    (account.temp / guard.KIT_DIR).rename(account.temp / "moved-kit")
    foreign = account.tmp / "foreign"
    foreign.mkdir()
    (account.temp / guard.KIT_DIR).symlink_to(foreign)
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


def test_retained_custody_marks_the_run_left(account, monkeypatch):
    quiet(monkeypatch)
    run = new_run(account).begin()
    assert run.finish(retain=True)["throwawayHome"]["state"] == "left"


# ---------------------------------------------------------------------------
# (finding 3) An unreadable folder in the home is listed and reported by count and place.

def test_an_unreadable_folder_in_the_home_is_reported_by_count_and_place(account, monkeypatch):
    quiet(monkeypatch)
    run = new_run(account).begin()
    (run.work / "locked").mkdir()
    (run.work / "locked").chmod(0)
    outcome = run.finish()
    home = outcome["throwawayHome"]
    assert home["state"] == "left" and home["unreadableInside"] == {"count": 1, "places": ["work/locked"]}
    assert run.entry["findings"]["unreadableInside"]["places"] == ["work/locked"]


@MACOS
def test_standin_an_unreadable_folder_with_a_live_holder(account):
    """Stand-in regression (finding 3): a process of the test's own holds a file inside a folder of the
    home it then makes unreadable, with its working directory elsewhere; the real listings."""
    run = new_run(account).begin()
    locked = run.work / "locked"
    locked.mkdir()
    holder = subprocess.Popen(["/bin/sh", "-c", f"exec 3>'{locked}/held'; cd /; chmod 000 '{locked}'; /bin/sleep 30"],
                              start_new_session=True)
    try:
        time.sleep(0.5)
        outcome = run.finish()
    finally:
        os.killpg(holder.pid, signal.SIGKILL)
        holder.wait()
    home = outcome["throwawayHome"]
    print("STANDIN finding 3:", json.dumps({k: home.get(k) for k in ("state", "unreadableInside", "holders")}, indent=1))
    assert home["state"] == "left" and home["unreadableInside"]["places"] == ["work/locked"]
    assert home["holders"]["holders"] == "unestablished"


# ---------------------------------------------------------------------------
# (finding 14) Custody held across the death of a parent.

def test_a_descendant_seen_once_is_recorded_with_its_own_group(account, monkeypatch):
    quiet(monkeypatch)
    run = new_run(account).begin()
    run.entry["started"] = [{"pid": 100, "start": "s100", "role": "app"}]
    run.entry["groups"] = [100]
    table = {100: (1, 100, os.getuid(), "/invented/app", "s100"),
             200: (100, 200, os.getuid(), "/invented/worker", "s200"),  # a child that made its own group
             300: (200, 200, os.getuid(), "/invented/grandchild", "s300")}
    quiet(monkeypatch, table)
    run.sample()
    recorded = {(g["pid"], g["start"]) for g in run.entry["started"]}
    assert {(200, "s200"), (300, "s300")} <= recorded and 200 in run.entry["groups"]
    run.finish(retain=True)


def test_a_sample_that_cannot_be_taken_is_recorded_as_missed(account, monkeypatch):
    quiet(monkeypatch)
    run = new_run(account).begin()
    monkeypatch.setattr(guard, "_process_table", lambda: (_ for _ in ()).throw(OSError("invented")))
    run.sample()
    assert run.entry["samples"]["missed"] and run.entry["samples"]["missed"][-1]["why"] == "OSError"
    quiet(monkeypatch)
    outcome = run.finish()
    assert outcome["samples"]["missed"]


@MACOS
@pytest.mark.parametrize("escape", ["same group", "own session"])
def test_standin_a_child_that_outlives_its_parent_is_still_in_the_custody_record(account, monkeypatch, escape):
    """Stand-in regression (finding 14): the launched stand-in starts a child and exits; the child is
    reparented and either stays in the launch's group or starts its own session after the kit has
    sampled it. The real listings, after the parent is gone."""
    monkeypatch.setattr(guard, "SAMPLE_SECONDS", 0.2)
    new_session = escape == "own session"
    body = textwrap.dedent(f"""
        import subprocess, time
        subprocess.Popen(["/bin/sleep", "30"], start_new_session={new_session}, close_fds=True, cwd="/",
                         stdin=subprocess.DEVNULL, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
        time.sleep(1.5)
    """)
    exe = executable(account.tmp / "outliving", body=body)
    run = new_run(account, exe=exe).begin()
    child = run.start("app", [str(exe)])
    child.wait(timeout=30)
    time.sleep(0.5)
    outcome = run.finish()
    writers = outcome["throwawayHome"]["holders"].get("writers") or []
    try:
        print(f"STANDIN finding 14 ({escape}):", json.dumps({"state": outcome["throwawayHome"]["state"],
                                                           "holders": outcome["throwawayHome"]["holders"],
                                                           "groups": run.entry["groups"]}, indent=1))
        sleeper = [w for w in writers if w["executable"] == "/bin/sleep"]
        assert outcome["throwawayHome"]["state"] == "left" and sleeper, writers
        why = " ".join(sleeper[0]["why"])
        assert "open under the home" not in why  # found by the custody record alone, not by a held file
        assert ("in a group of the run" in why) if escape == "same group" else ("recorded generation alive" in why)
    finally:
        for writer in writers:
            if writer["executable"] == "/bin/sleep":
                os.kill(writer["pid"], signal.SIGKILL)


# ---------------------------------------------------------------------------
# One run lifecycle; interruption; forgetting the finish.

def test_there_is_no_start_outside_the_run():
    for name in ("start", "spec", "make_home"):
        assert not hasattr(guard, name), name


def test_the_utility_runner_starts_only_the_four_utilities_in_their_shapes(account):
    for argv in ([str(account.exe)], [guard.PS, "-ef"], [guard.LSOF, "-n"], ["ps"], [guard.GETCONF, "PATH"],
                 [guard.LSOF, "-n", "-P", "-F", "p", "+D", "relative"], [sys.executable, "-c", "pass"]):
        with pytest.raises(guard.LaunchRefused, match="utility_not_allowed"):
            guard._read(argv)


def test_the_helpers_own_utilities_get_a_minimal_environment(monkeypatch):
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
    guard._read([guard.PS, "-axww", "-o", "pid=,ppid=,pgid=,uid=,lstart=,comm="])
    assert seen["env"] == guard.UTIL_ENV and seen["cwd"] == "/" and seen["close_fds"] is True
    assert all(Path(p).is_absolute() for p in guard.UTILITY_SHAPES)


def test_a_run_cannot_start_before_it_begins_or_after_it_finishes(account, monkeypatch):
    quiet(monkeypatch)
    run = new_run(account)
    with pytest.raises(guard.LaunchRefused, match="run_not_open"):
        run.start("app", [str(account.exe)])
    run.begin()
    run.finish()
    with pytest.raises(guard.LaunchRefused, match="run_not_open"):
        run.start("app", [str(account.exe)])


def test_the_run_starts_only_its_own_executable(account, monkeypatch):
    quiet(monkeypatch)
    run = new_run(account).begin()
    try:
        with pytest.raises(guard.LaunchRefused, match="executable_not_the_runs"):
            run.start("app", [sys.executable, "-c", "pass"])
    finally:
        run.finish()


def test_a_comparison_that_raises_does_not_skip_the_marking(account, monkeypatch):
    quiet(monkeypatch)
    monkeypatch.setattr(guard, "compare", lambda *a, **k: (_ for _ in ()).throw(OSError("invented")))
    run = new_run(account).begin()
    outcome = run.finish()
    assert outcome["comparison"]["verdict"] == "not compared" and outcome["throwawayHome"]["state"] == "finished"
    assert evidence_file(account, "home-comparison.json")["verdict"] == "not compared"


def test_an_interrupt_during_the_settle_wait_skips_neither_comparison_nor_marking(account, monkeypatch):
    quiet(monkeypatch)
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
    outcome = run.finish()
    assert outcome["comparison"]["compared"] is True and outcome["throwawayHome"]["state"] == "finished"
    assert outcome["interruptions"] and "settle" in outcome["interruptions"][0]


SCRIPT_PRELUDE = textwrap.dedent("""
    import json, os, sys, time
    from pathlib import Path
    sys.path.insert(0, sys.argv[1])
    from tools import native_launch_guard as guard
    root = Path(sys.argv[2])
    guard.real_home = lambda: root / "real-home"
    guard.temp_root = lambda: (root / "system-temp").resolve()
    guard.user_cache_dir = lambda: (root / "system-cache").resolve()
    guard.check_launch = lambda *a: {"launch": "invented"}
    guard.PLATFORM = "darwin"
    guard._process_table = lambda: {os.getpid(): (1, 1, os.getuid(), "/invented", "x")}
    guard._open_under = lambda path: set()
    guard.SETTLE_SECONDS = 2
    def new_run(label="home"):
        run = guard.Run(sys.argv[3], root / "evidence", label)
        run.library, run.cache, run.real = root / "real-home" / "Library", root / "system-cache", root / "real-home"
        return run
""")

SIGNALLED = SCRIPT_PRELUDE + textwrap.dedent("""
    run = new_run()
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

FORGOTTEN = SCRIPT_PRELUDE + textwrap.dedent("""
    guard.SETTLE_SECONDS = 0
    run = new_run()
    run.begin()
    print(json.dumps({"home": str(run.home)}), flush=True)
    # the script ends without calling finish
""")


def script(account, text, *extra):
    path = account.tmp / "script.py"
    path.write_text(text)
    return subprocess.Popen([sys.executable, "-I", str(path), str(Path(guard.__file__).resolve().parents[1]),
                             str(account.tmp), str(account.exe), *extra], stdout=subprocess.PIPE, stderr=subprocess.PIPE,
                            start_new_session=True)


@pytest.mark.parametrize("signum", [signal.SIGTERM, signal.SIGINT, signal.SIGHUP])
@pytest.mark.parametrize("when", ["running", "settle"])
def test_a_termination_signal_still_persists_the_comparison_and_marks_the_run(account, signum, when):
    process = script(account, SIGNALLED, when)
    home = Path(json.loads(process.stdout.readline())["home"])
    time.sleep(0.5)
    process.send_signal(signum)
    process.wait(timeout=60)
    assert process.returncode != 0
    assert evidence_file(account, "home-comparison.json")["compared"] is True
    assert evidence_file(account, "home-run.json")["throwawayHome"]["state"] == "finished" and home.is_dir()


def test_a_forgotten_finish_is_made_at_the_interpreters_exit(account):
    process = script(account, FORGOTTEN)
    home = Path(json.loads(process.stdout.readline())["home"])
    process.wait(timeout=60)
    record = evidence_file(account, "home-run.json")
    assert record["forgotten"] is True and record["throwawayHome"]["state"] == "finished" and home.is_dir()


# ---------------------------------------------------------------------------
# (findings 8, 10, 11) The executable, the product's names, every caller location.

def test_the_executable_must_be_canonical_and_absolute(account):
    link = account.tmp / "linked-app"
    link.symlink_to(account.exe)
    for given in (link, Path(os.path.relpath(account.exe))):
        with pytest.raises(guard.LaunchRefused, match="executable_not_canonical"):
            guard.Run(given, account.evidence)


def test_an_executable_changed_after_admission_is_not_started(account, monkeypatch):
    quiet(monkeypatch)
    monkeypatch.setattr(guard, "_POPEN", lambda *a, **k: pytest.fail("started"))
    run = new_run(account).begin()
    try:
        account.exe.write_text(account.exe.read_text() + "\n# changed\n")
        with pytest.raises(guard.LaunchRefused, match="executable_changed"):
            run.start("app", [str(account.exe)])
    finally:
        run.finish()


def test_the_names_are_derived_from_the_bundle_and_can_only_be_added_to(account):
    run = guard.Run(account.exe, account.evidence, "home", extra_names=["com.invented.added"])
    assert run.names == sorted({BUNDLE, EXE, "com.invented.added"})
    with pytest.raises(guard.LaunchRefused, match="added_name_not_allowed"):
        guard.Run(account.exe, account.evidence, "home", extra_names=["x"])
    with pytest.raises(TypeError):
        guard.Run(account.exe, account.evidence, "home", names=["different"])


@pytest.mark.parametrize("change, refusal", [
    (lambda c: (c / "Info.plist").unlink(), "bundle_identity_unestablished"),
    (lambda c: (c / "Info.plist").write_bytes(b"not a plist"), "bundle_identity_unestablished"),
    (lambda c: (c / "Info.plist").write_bytes(plistlib.dumps({"CFBundleIdentifier": BUNDLE, "CFBundleExecutable": "other"})),
     "bundle_does_not_name_the_executable"),
    (lambda c: (c / "Info.plist").write_bytes(plistlib.dumps({"CFBundleExecutable": EXE})), "bundle_identity_unestablished"),
])
def test_an_unestablished_product_identity_refuses(account, change, refusal):
    change(account.exe.parents[1])
    with pytest.raises(guard.LaunchRefused, match=refusal):
        guard.Run(account.exe, account.evidence)


def test_an_executable_without_the_strict_chooser_name_is_refused(account, monkeypatch):
    quiet(monkeypatch)
    plain = executable(account.tmp / "plain", marker=False)
    run = new_run(account, exe=plain)
    with pytest.raises(guard.LaunchRefused, match="strict_chooser_absent"):
        run.begin()
    run.finish()


def test_the_strict_chooser_lead_finds_the_name_across_a_read_boundary(tmp_path):
    path = tmp_path / "binary"
    needle = guard.STRICT_CHOOSER_ENV.encode()
    path.write_bytes(b"\0" * ((1 << 20) - 5) + needle + b"\0")
    fd = os.open(path, os.O_RDONLY)
    try:
        assert guard.strict_chooser_lead(fd) is True
    finally:
        os.close(fd)


@pytest.mark.parametrize("where, refusal", [
    (lambda a: a.real / "Desktop" / "ev", "evidence_under_desktop"),
    (lambda a: a.real / "Documents" / "ev", "evidence_under_documents"),
    (lambda a: a.real / "Downloads" / "ev", "evidence_under_downloads"),
    (lambda a: a.real / "Library" / "ev", "evidence_under_library"),
    (lambda a: Path(guard.__file__).resolve().parents[1] / "invented-evidence-folder", "evidence_under_the_repository"),
])
def test_an_evidence_folder_under_the_accounts_folders_or_the_repository_is_refused_before_it_exists(account, where, refusal):
    path = where(account)
    with pytest.raises(guard.LaunchRefused, match=refusal):
        guard.make_evidence(path)
    assert not path.exists()


def test_an_evidence_location_with_a_link_on_its_path_is_refused(account):
    link = account.tmp / "link"
    link.symlink_to(account.evidence)
    with pytest.raises(guard.LaunchRefused, match="evidence_path_has_a_link"):
        guard.make_evidence(link / "sub")
    with pytest.raises(guard.LaunchRefused, match="evidence_path_has_a_link"):
        guard.open_folder(link)


def test_the_descriptor_walk_refuses_a_link_that_appears_after_the_check(account, monkeypatch):
    target = account.evidence / "made"
    monkeypatch.setattr(guard, "check_location", lambda path, kind: Path(path))  # the earlier check passed
    (account.evidence / "swapped").symlink_to(account.tmp)
    with pytest.raises(guard.LaunchRefused, match="evidence_not_a_real_folder"):
        guard.open_folder(account.evidence / "swapped")
    assert guard.make_evidence(target) == target and target.is_dir()


@pytest.mark.parametrize("make, refusal", [
    (lambda a: a.real / "Documents" / "probe", "resource_under_documents"),
    (lambda a: a.tmp / "missing-probe", "resource_missing"),
    (lambda a: a.tmp, "resource_not_a_regular_file"),
])
def test_a_resource_location_is_checked_before_it_is_read(account, make, refusal):
    path = make(account)
    if "Documents" in str(path):
        path.write_bytes(b"x")
    with pytest.raises(guard.LaunchRefused, match=refusal):
        guard.check_location(path, "resource")


def test_the_journey_refuses_an_output_under_documents_before_creating_anything(account, monkeypatch, capsys):
    for name in ("app", "probe"):
        (account.tmp / name).write_bytes(b"x")
    output = account.real / "Documents" / "journey-output"
    monkeypatch.setattr(journey, "RETAINED", account.tmp / "retained")
    monkeypatch.setattr(journey.native_launch_guard, "make_evidence", lambda path: pytest.fail("creation reached"))
    monkeypatch.setattr(journey, "open", lambda *a, **k: pytest.fail("the host lock reached"), raising=False)
    monkeypatch.setattr(sys, "argv", ["agent_journey_fixture.py", "--app", str(account.tmp / "app"), "--process-probe",
                                      str(account.tmp / "probe"), "--boundary-probe", str(account.tmp / "probe"),
                                      "--mode", "stub", "--output", str(output)])
    assert journey.main() == 2 and not output.exists()
    assert "evidence_under_documents" in capsys.readouterr().err


def test_the_live_fixture_refuses_an_output_under_library_before_creating_anything(account, monkeypatch, capsys):
    output = account.library / "live-output"
    monkeypatch.setattr(live.native_launch_guard, "make_evidence", lambda path: pytest.fail("creation reached"))
    monkeypatch.setattr(sys, "argv", ["agent_live_session_fixture.py", "--app", str(account.exe), "--process-probe",
                                      str(account.probe), "--boundary-probe", str(account.probe), "--endpoint",
                                      "http://127.0.0.1:9/v1", "--model", "m", "--context-window", "8192", "--output", str(output)])
    assert live.main() == 2 and not output.exists()


# ---------------------------------------------------------------------------
# (findings 4, 5, 9) The start: environment, working directory, descriptors, frames.

NAMES_ONLY = "import os; print('\\n'.join(sorted(os.environ)))"


def started_output(account, monkeypatch, body, kit=None, **extra):
    exe = executable(account.tmp / "printing", body=body)
    quiet(monkeypatch)
    run = new_run(account, exe=exe).begin()
    try:
        child = run.start("app", [str(exe)], kit, stdout=subprocess.PIPE, **extra)
        out = child.communicate(timeout=30)[0].decode()
    finally:
        run.finish()
    return run, out


def test_nothing_of_the_callers_environment_reaches_the_child(account, monkeypatch):
    monkeypatch.setenv("SWBPIPE_NATIVE_SMOKE_INJECT_RESULT", "invented")
    monkeypatch.setenv("INVENTED_API_KEY", "invented")
    monkeypatch.setenv("DYLD_INVENTED", "invented")
    run, out = started_output(account, monkeypatch, NAMES_ONLY, {guard.DRIVER: "-"})
    names = set(out.split())
    assert not names & {"SWBPIPE_NATIVE_SMOKE_INJECT_RESULT", "INVENTED_API_KEY", "DYLD_INVENTED"}
    expected = {"PATH", "HOME", "CFFIXED_USER_HOME", "TMPDIR", "LANG", guard.STRICT_CHOOSER_ENV, guard.DRIVER}
    assert expected <= names and names - expected <= {"__CF_USER_TEXT_ENCODING"}, names - expected


def test_the_child_works_in_the_runs_folder_in_a_group_of_its_own(account, monkeypatch):
    elsewhere = account.tmp / "callers-own"
    elsewhere.mkdir()
    monkeypatch.chdir(elsewhere)
    run, out = started_output(account, monkeypatch, "import os; print(os.getcwd()); print(os.getpgid(0) == os.getpid())")
    cwd, own_group = out.split()
    assert Path(cwd).resolve() == Path(run.work).resolve() and own_group == "True"
    assert run.entry["started"][0]["pid"] in run.entry["groups"]


def test_the_environment_is_built_from_the_run_alone(account, monkeypatch):
    quiet(monkeypatch)
    run = new_run(account).begin()
    try:
        env = run.environment({guard.DRIVER: "-", guard.REPORT: guard.RUN_OWNED, guard.GATE: guard.RUN_OWNED,
                               guard.GATE_MODE: "handshake", guard.PROVIDER_KEY: "stdin"})
        assert env["HOME"] == env["CFFIXED_USER_HOME"] == str(run.home) and env["TMPDIR"] == str(run.tmp)
        assert env["PATH"] == guard.PATH == "/usr/bin:/bin" and env[guard.STRICT_CHOOSER_ENV] == "1"
        assert env[guard.REPORT] == str(run.work / "report.json") and env[guard.GATE] == str(run.work / "gate")
    finally:
        run.finish()


def test_no_inherited_descriptor_reaches_the_child(account, monkeypatch):
    left, right = socket.socketpair()
    handle = open(account.tmp / "invented-file", "w")
    numbers = (left.fileno(), right.fileno(), handle.fileno())
    for fd in numbers:
        os.set_inheritable(fd, True)
    try:
        run, out = started_output(account, monkeypatch, "import os; print(' '.join(sorted(os.listdir('/dev/fd'))))")
    finally:
        left.close(), right.close(), handle.close()
    descriptors = {int(d) for d in out.split()}
    assert min(numbers) > 3  # numbers the child could not mistake for its own
    assert descriptors <= {0, 1, 2, 3}, descriptors


@pytest.mark.parametrize("option, refusal", [
    ({"pass_fds": (0,)}, "process_option_not_allowed:pass_fds"), ({"close_fds": False}, "process_option_not_allowed:close_fds"),
    ({"preexec_fn": lambda: None}, "process_option_not_allowed:preexec_fn"), ({"env": {}}, "process_option_not_allowed:env"),
    ({"cwd": "/"}, "process_option_not_allowed:cwd"), ({"start_new_session": True}, "process_option_not_allowed:start_new_session"),
    ({"stdout": 5}, "stream_not_allowed:stdout"), ({"stderr": subprocess.STDOUT}, "stream_not_allowed:stderr")])
def test_process_options_outside_the_allowlist_are_refused(account, monkeypatch, option, refusal):
    quiet(monkeypatch)
    monkeypatch.setattr(guard, "_POPEN", lambda *a, **k: pytest.fail("started"))
    run = new_run(account).begin()
    try:
        with pytest.raises(guard.LaunchRefused) as raised:
            run.start("app", [str(account.exe)], **option)
        assert str(raised.value) == refusal
    finally:
        run.finish()


def test_the_fixture_frame_is_built_by_the_helper_inside_the_home(account, monkeypatch):
    quiet(monkeypatch)
    run = new_run(account).begin()
    try:
        frame = run.fixture_frame(model="m", context_window=8192, endpoint="http://127.0.0.1:9/v1", key="invented-key",
                                  mode="idle", process_probe=account.probe, boundary_probe=account.probe)
        data = json.loads(frame.data)
        assert data["state"] == str(run.work / "state") and (run.work / "state").is_dir()
        assert "invented-key" not in json.dumps(frame.record)
        linked = account.tmp / "linked-probe"
        linked.symlink_to(account.probe)
        for bad in (dict(process_probe=linked), dict(mode="other"), dict(endpoint="file:///invented"), dict(model="/invented")):
            arguments = dict(model="m", context_window=8192, endpoint="http://127.0.0.1:9/v1", key=None, mode="active",
                             process_probe=account.probe, boundary_probe=account.probe)
            arguments.update(bad)
            with pytest.raises(guard.LaunchRefused):
                run.fixture_frame(**arguments)
        # The walk itself refuses a link, even where a path check that ran earlier had passed.
        checked = guard.check_location
        monkeypatch.setattr(guard, "check_location", lambda path, kind: Path(path))
        with pytest.raises(guard.LaunchRefused, match="file_not_openable_without_a_link"):
            run.fixture_frame(model="m", context_window=8192, endpoint="http://127.0.0.1:9/v1", key=None, mode="active",
                              process_probe=linked, boundary_probe=account.probe)
        monkeypatch.setattr(guard, "check_location", checked)
        with pytest.raises(guard.LaunchRefused, match="frame_not_the_helpers"):
            run.start("app", [str(account.exe)], frame=b"{}", stdin=subprocess.PIPE)
        with pytest.raises(guard.LaunchRefused, match="key_line_needs_a_frame"):
            run.start("app", [str(account.exe)], key_line="k", stdin=subprocess.PIPE)
    finally:
        run.finish()


def test_the_driver_is_read_from_the_repository_and_its_sent_bytes_hashed(account, monkeypatch):
    import hashlib
    exe = executable(account.tmp / "reading", body="import sys; sys.stdout.buffer.write(sys.stdin.buffer.read())")
    quiet(monkeypatch)
    run = new_run(account, exe=exe).begin()
    try:
        frame = run.driver_frame("apps/desktop/native-smoke/agent-journey.js")
        child = run.start("app", [str(exe)], frame=frame, key_line="invented-key", close_stdin=True,
                          stdin=subprocess.PIPE, stdout=subprocess.PIPE)
        received = child.communicate(timeout=30)[0]
        with pytest.raises(guard.LaunchRefused, match="driver_not_in_the_repository"):
            run.driver_frame("../outside.js")
    finally:
        run.finish()
    sent = run.frames[0]
    assert received == b"invented-key\n" + frame.data
    assert sent["sha256Sent"] == hashlib.sha256(frame.data).hexdigest() and sent["bytesSent"] == len(frame.data)
    assert "invented-key" not in json.dumps(run.frames)


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


def test_the_real_home_comes_from_the_password_database_not_the_environment(monkeypatch):
    monkeypatch.setenv("HOME", "/nowhere")
    import pwd
    assert guard.real_home() == Path(pwd.getpwuid(os.getuid()).pw_dir)


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


def test_an_open_files_listing_with_a_diagnostic_or_an_unparsed_line_refuses(monkeypatch):
    monkeypatch.setattr(guard, "_read", lambda argv, timeout=0: (0, b"p12\nfcwd\nunexpected\n", ""))
    with pytest.raises(OSError, match="unparsed"):
        guard._open_under("/invented")
    monkeypatch.setattr(guard, "_read", lambda argv, timeout=0: (1, b"p12\nf3\n", "lsof: WARNING: invented"))
    with pytest.raises(OSError, match="WARNING"):
        guard._open_under("/invented")
    monkeypatch.setattr(guard, "_read", lambda argv, timeout=0: (1, b"p12\nf3\n", ""))
    assert guard._open_under("/invented") == {12}


def test_generations_are_identifier_and_start_time_together():
    table = {7: (1, 7, 0, "/invented", "2026-01-01T00:00:00")}
    assert guard._generation_alive({"pid": 7, "start": "2026-01-01T00:00:00"}, table) == "alive"
    assert guard._generation_alive({"pid": 7, "start": "2025-01-01T00:00:00"}, table) is None
    assert guard._generation_alive({"pid": 7, "start": None}, table) == "unestablished"
    assert guard._generation_alive({"pid": 8, "start": None}, table) is None


# ---------------------------------------------------------------------------
# The strict chooser constants; the screen; the slot.

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
# (findings 12, 13) The lister and the comparison.

def compared(account, act, run=None, until=None, keep_open=False):
    run = run or new_run(account).begin()
    web_view_data(run)
    act()
    report = guard.compare(run.before, run.home, until=until, library=account.library, home=account.real, cache=account.cache)
    if not keep_open:
        run.finish()
    return report


@pytest.fixture
def lister(account, monkeypatch):
    quiet(monkeypatch)
    return account


WATCHED = ("WebKit", "Caches", "HTTPStorages", "Application Support", "Preferences", "Saved Application State", "Logs")


@pytest.mark.parametrize("top", WATCHED)
@pytest.mark.parametrize("name", NAMES)
@pytest.mark.parametrize("change", ["created", "modified", "removed"])
def test_each_change_under_each_watched_folder_for_each_name_is_a_product_change(lister, top, name, change):
    existing = place(lister.library, top, name) if change != "created" else None
    report = compared(lister, {"created": lambda: place(lister.library, top, name, "new.bin"),
                               "modified": lambda: existing.write_bytes(b"longer"),
                               "removed": lambda: existing.unlink()}[change])
    assert report["verdict"] == "product" and change in [c["change"] for c in report["product"]["changes"]]


def test_a_name_four_levels_down_and_with_a_dot_suffix_is_watched(lister):
    def act():
        (lister.library / "Preferences" / f"{BUNDLE}.plist").write_bytes(b"p")
        (lister.library / "Caches" / "a" / "b" / "c" / EXE).mkdir(parents=True)
    report = compared(lister, act)
    assert {c["path"] for c in report["product"]["changes"]} >= {f"Preferences/{BUNDLE}.plist", f"Caches/a/b/c/{EXE}"}


def test_every_other_top_folder_is_watched_one_level_deep_by_name(lister):
    (lister.library / "Invented Top" / "deeper").mkdir(parents=True)
    def act():
        (lister.library / "Invented Top" / EXE).mkdir()
        (lister.library / "Invented Top" / "deeper" / BUNDLE).mkdir()
    report = compared(lister, act)
    assert [c["path"] for c in report["product"]["changes"]] == [f"Invented Top/{EXE}"]


def test_an_unreadable_product_folder_one_level_down_refuses_at_the_baseline(lister):
    folder = lister.library / "Invented Top" / BUNDLE
    folder.mkdir(parents=True)
    folder.chmod(0)
    with pytest.raises(guard.LaunchRefused, match="baseline_product_folder_unreadable"):
        new_run(lister).begin()


def test_a_watched_top_folder_that_is_a_link_refuses_at_the_baseline(lister):
    elsewhere = lister.tmp / "documents-like"
    elsewhere.mkdir()
    (lister.library / "Caches").rmdir()
    (lister.library / "Caches").symlink_to(elsewhere)
    with pytest.raises(guard.LaunchRefused, match="baseline_folder_not_listed:Caches"):
        new_run(lister).begin()


def test_an_unreadable_watched_top_folder_refuses_at_the_baseline_and_fails_the_comparison(lister):
    (lister.library / "HTTPStorages").chmod(0)
    try:
        with pytest.raises(guard.LaunchRefused, match="baseline_folder_not_listed:HTTPStorages"):
            new_run(lister).begin()
    finally:
        (lister.library / "HTTPStorages").chmod(0o700)
    run = new_run(lister).begin()
    report = compared(lister, lambda: (lister.library / "HTTPStorages").chmod(0), run, keep_open=True)
    (lister.library / "HTTPStorages").chmod(0o700)
    run.finish()
    assert report["verdict"] == "comparison failed" and report["failure"]["foldersNotListed"] == ["HTTPStorages"]


def test_an_unreadable_product_folder_refuses_at_the_baseline(lister):
    folder = place(lister.library, "WebKit", BUNDLE).parent
    folder.chmod(0)
    with pytest.raises(guard.LaunchRefused, match="baseline_product_folder_unreadable"):
        new_run(lister).begin()


def test_an_unreadable_folder_not_product_named_is_counted_never_named_and_a_changed_count_fails(lister):
    other = place(lister.library, "Application Support", "invented-private-name").parent
    other.chmod(0)
    run = new_run(lister).begin()
    assert run.before["unreadableOther"] == 1 and "invented-private-name" not in json.dumps(run.before)
    same = compared(lister, lambda: None, run, keep_open=True)
    assert same["verdict"] == "clean" and same["limit"]["statement"] == "1 folders could not be listed and were not watched"
    changed = compared(lister, lambda: other.chmod(0o700), run)
    assert changed["verdict"] == "comparison failed" and "invented-private-name" not in json.dumps(changed)


def test_a_link_inside_a_watched_folder_is_listed_and_not_followed(lister):
    outside = lister.tmp / "outside"
    (outside / "deep").mkdir(parents=True)
    (lister.library / "Caches" / BUNDLE).mkdir()
    (lister.library / "Caches" / BUNDLE / "link").symlink_to(outside)
    run = new_run(lister).begin()
    entries = run.before["entries"]
    run.finish()
    assert entries[f"Caches/{BUNDLE}/link"]["kind"] == "link" and not any("deep" in p for p in entries)


def test_others_are_counts_only_never_names(lister):
    def act():
        (lister.library / "Preferences" / "com.invented.service.plist").write_bytes(b"s")
        place(lister.library, "Caches", "invented-other-app")
        place(lister.library, "Application Support", "invented-other-app", "private-name.db")
        place(lister.library, "Caches", f"{BUNDLE}-other")
    report = compared(lister, act, until=time.time() + 5)
    assert report["verdict"] == "clean" and report["class"] == "others"
    assert set(report["others"]) == {"statement", "total", "byFolder"} and report["others"]["byFolder"]["Preferences"] >= 1
    text = json.dumps(report)
    assert "com.invented.service" not in text and "private-name" not in text and "invented-other-app" not in text
    assert "never named" in report["others"]["statement"]


def test_no_web_view_data_has_its_own_verdict_and_its_limit_is_stated(lister):
    run = new_run(lister).begin()
    report = guard.compare(run.before, run.home, library=lister.library, home=lister.real, cache=lister.cache)
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


def test_the_web_view_helpers_cache_folders_are_their_own_class_and_always_stop_the_run(lister):
    report = compared(lister, lambda: in_cache(lister, HELPER, "data"))
    assert report["verdict"] == "web view helper cache" and report["class"] == "web view helper cache"
    assert report["product"]["changes"] == [] and len(report["helperCache"]["changes"]) == 2


@pytest.mark.parametrize("parts", [(BUNDLE, "data"), (EXE, "data"), ("invented", "com.apple.WebKit.Networking+" + EXE, "data"),
                                   ("com.apple.WebKit.Invented+" + EXE, "data")])
def test_a_product_named_entry_of_any_other_shape_in_the_cache_folder_trips_the_wire(lister, parts):
    report = compared(lister, lambda: in_cache(lister, *parts))
    assert report["verdict"] == "product" and report["helperCache"]["changes"] == []


def test_the_cache_folder_is_named_by_what_it_is_never_by_its_path(lister):
    in_cache(lister, BUNDLE, "data")
    before = guard.snapshot(NAMES, lister.library, lister.real, lister.cache)
    assert f"{guard.CACHE_TOP}/{BUNDLE}/data" in before["entries"] and str(lister.cache) not in json.dumps(before)


def test_the_cache_folder_fails_closed(lister, monkeypatch):
    run = new_run(lister).begin()
    report = compared(lister, lambda: lister.cache.chmod(0), run, keep_open=True)
    lister.cache.chmod(0o700)
    monkeypatch.setattr(guard, "user_cache_dir", lambda: None)
    run.cache = None
    missing = guard.compare(run.before, run.home, library=lister.library, home=lister.real)
    run.finish()
    assert report["verdict"] == "comparison failed" and report["failure"]["foldersNotListed"] == [guard.CACHE_TOP]
    assert missing["failure"]["foldersNotListed"] == [guard.CACHE_TOP]


def test_the_four_home_folders_are_stat_only(lister, monkeypatch):
    for folder in ("Desktop", "Documents", "Downloads"):
        (lister.real / folder).chmod(0)
    listed = []
    original = os.listdir
    monkeypatch.setattr(guard.os, "listdir", lambda p=".": listed.append(p) or original(p))
    times = guard.folder_times()
    for folder in ("Desktop", "Documents", "Downloads"):
        (lister.real / folder).chmod(0o700)
    assert set(times) == {"~", "Desktop", "Documents", "Downloads"} and all(isinstance(t, float) for t in times.values())
    assert listed == []


def test_a_change_of_the_four_times_is_reported_not_concluded_from(lister):
    report = compared(lister, lambda: os.utime(lister.real / "Downloads", (1, 1)))
    assert report["folderTimes"]["changed"] == ["Downloads"] and report["verdict"] == "clean"


# ---------------------------------------------------------------------------
# The controllers.

class Started:
    pid = 2 ** 22 + 7  # an identifier no process here holds

    def __init__(self, argv, env=None, cwd=None, **kwargs):
        self.argv, self.env, self.cwd, self.kwargs = argv, env, cwd, kwargs
        self.stdin = None

    def poll(self):
        return 0


def journey_guard(account, monkeypatch):
    quiet(monkeypatch)
    monkeypatch.setattr(journey.LaunchGuard, "library", account.library)
    monkeypatch.setattr(journey.LaunchGuard, "cache", account.cache)
    monkeypatch.setattr(journey.LaunchGuard, "real", account.real)
    monkeypatch.setattr(guard, "_POPEN", Started)
    return journey.LaunchGuard(account.exe, account.evidence), account.exe, account.evidence


def test_every_start_the_journey_makes_carries_the_runs_environment(account, monkeypatch):
    launch_guard, app, root = journey_guard(account, monkeypatch)
    with pytest.raises(journey.Refusal, match="launch_refused_run_not_open"):
        launch_guard.launch("preflight", [str(app)], {})
    launch_guard.start()
    for role in ("preflight", "app"):
        started = launch_guard.launch(role, [str(app)], {guard.DRIVER: "-"})
        assert started.env["HOME"] == started.env["CFFIXED_USER_HOME"] == str(launch_guard.home)
        assert started.cwd == str(launch_guard.run.work) and started.kwargs["close_fds"] is True
        assert started.kwargs["process_group"] == 0
    assert launch_guard.launches == ["preflight", "app"] and set(launch_guard.names) >= {EXE, BUNDLE}
    result = {}
    launch_guard.finish(result)
    assert result["throwawayHome"]["state"] == "finished" and Path(result["throwawayHome"]["home"]).is_dir()


def test_the_journey_refuses_a_start_with_a_variable_the_kit_does_not_own(account, monkeypatch):
    launch_guard, app, root = journey_guard(account, monkeypatch)
    launch_guard.start()
    with pytest.raises(journey.Refusal, match="launch_refused_injection_variable"):
        launch_guard.launch("app", [str(app)], {"SWBPIPE_NATIVE_SMOKE_INJECT_RESULT": "x"})
    with pytest.raises(journey.Refusal, match="launch_refused_report_and_gate_are_the_runs"):
        launch_guard.launch("app", [str(app)], {guard.REPORT: str(root / "report.json")})
    assert launch_guard.launches == []
    launch_guard.finish({})


def test_the_preflight_goes_through_the_guard_with_the_helpers_frame(account, monkeypatch):
    launch_guard, app, root = journey_guard(account, monkeypatch)
    launch_guard.start()
    seen = []
    def launch(role, argv, extra, frame=None, **popen):
        seen.append((role, argv, json.loads(frame.data)["state"]))
        raise journey.Refusal("stop")
    monkeypatch.setattr(launch_guard, "launch", launch)
    args = SimpleNamespace(app=app, process_probe=account.probe, boundary_probe=account.probe)
    with pytest.raises(journey.Refusal):
        journey.preflight(args, root, launch_guard)
    assert seen == [("preflight", [str(app), "--native-agent-session-fixture"], str(launch_guard.run.work / "preflight"))]
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


def test_the_helper_starts_nothing_but_the_run_and_its_utilities():
    tree = ast.parse(Path(guard.__file__).read_text())
    starts = sorted(ast.unparse(call.func) for call in ast.walk(tree) if isinstance(call, ast.Call)
                    and re.match(r"^(_POPEN|_READ_POPEN|subprocess\.\w+|os\.(system|popen|spawn\w*|exec\w*|posix_spawn\w*|fork\w*))$",
                                 ast.unparse(call.func)))
    assert starts == ["_POPEN", "_READ_POPEN"], starts


def run_ending(account, monkeypatch, ending):
    _, app, root = journey_guard(account, monkeypatch)
    monkeypatch.setattr(journey, "sha256", lambda p: "0" * 64)
    def journey_body(args, key, root, launch_guard, result):
        launch_guard.launch("app", [str(app)], {})
        ending(result)
    monkeypatch.setattr(journey, "journey", journey_body)
    return root, (lambda: journey.run(SimpleNamespace(app=app, mode="stub"), None, root))


def test_a_failed_run_still_persists_the_comparison_and_marks_the_home(account, monkeypatch):
    def refused(result):
        raise journey.Refusal("preflight")
    root, call = run_ending(account, monkeypatch, refused)
    result = call()
    assert result["failure"] == "preflight" and result["launchGuard"]["compared"] is True
    assert result["throwawayHome"]["state"] == "finished" and Path(result["throwawayHome"]["home"]).is_dir()
    assert (root / "home-comparison.json").is_file() and (root / "home-listing.json").is_file()


def test_an_interrupt_during_the_journey_still_finishes_the_run(account, monkeypatch):
    def interrupted(result):
        raise guard.Interrupted(signal.SIGTERM)
    root, call = run_ending(account, monkeypatch, interrupted)
    with pytest.raises(guard.Interrupted):
        call()
    record = json.loads((root / "home-run.json").read_text())
    assert record["throwawayHome"]["state"] == "finished" and (root / "home-comparison.json").is_file()


def test_a_product_change_fails_the_run_even_on_refusal(account, monkeypatch):
    def leaked(result):
        place(account.library, "WebKit", BUNDLE)
        raise journey.Refusal("preflight")
    root, call = run_ending(account, monkeypatch, leaked)
    result = call()
    assert result["launchGuard"]["verdict"] == "product" and journey.require_comparison(result) == journey.GUARD_EXIT


def test_a_run_whose_processes_may_be_alive_is_left_fails_and_names_its_home(account, monkeypatch):
    def retained(result):
        result["rescue"] = "retained_original_app_custody"
    root, call = run_ending(account, monkeypatch, retained)
    result = call()
    home = result["throwawayHome"]["home"]
    assert result["throwawayHome"]["state"] == "left" and Path(home).is_dir()
    assert journey.require_comparison(result) == journey.GUARD_EXIT and home in result["failure"]


def test_a_refused_product_identity_is_a_recorded_failure_not_a_crash(account, monkeypatch):
    _, app, root = journey_guard(account, monkeypatch)
    (account.exe.parents[1] / "Info.plist").unlink()
    monkeypatch.setattr(journey, "sha256", lambda p: "0" * 64)
    result = journey.run(SimpleNamespace(app=account.exe, mode="stub"), None, root)
    assert result["failure"] == "launch_refused_bundle_identity_unestablished"


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


# ---------------------------------------------------------------------------
# (finding 6) The live session fixture's cleanup always ends in the finish, with a process started.

READY = 'import json, sys; sys.stdin.readline(); print(json.dumps({"record": "ready", "elapsedMs": 0, "data": {"appPid": 1, "sdkPid": 2, "scope": "x"}}), flush=True)'


def live_args(account, exe):
    return SimpleNamespace(cancel_file=None, app=exe, model="m", context_window=8192, endpoint="http://127.0.0.1:9/v1",
                           process_probe=account.probe, boundary_probe=account.probe)


def started_pid(account):
    kit = guard.Kit()
    try:
        return [g["pid"] for _, e in kit.entries() if e for g in e["started"] if g["role"] == "fixture"]
    finally:
        kit.close()


def test_a_launched_fixtures_cleanup_ends_in_the_finish(account, monkeypatch):
    exe = executable(account.tmp / "fixture", body=READY + "\nimport time; time.sleep(0.5)")
    quiet(monkeypatch)
    for name in ("library", "cache", "real"):
        monkeypatch.setattr(guard.Run, name, getattr(account, name))
    result = live.run_case(live_args(account, exe), "case", "idle", "none", account.evidence)
    assert result["failure"] == "app_identity" and result["app"]["pid"] > 0
    assert result["throwawayHome"]["state"] == "finished" and result["launchGuard"]["compared"] is True
    assert (account.evidence / "case" / "home-comparison.json").is_file()


def test_standin_an_exception_between_rescue_and_finish(account, monkeypatch):
    """Stand-in regression (finding 6): a fixture stand-in that stays alive; the rescue's first step
    raises; the finish still runs and leaves the home marked left, with the process still alive."""
    exe = executable(account.tmp / "fixture", body=READY + "\nimport time; time.sleep(30)")
    quiet(monkeypatch)
    for name in ("library", "cache", "real"):
        monkeypatch.setattr(guard.Run, name, getattr(account, name))
    def failing_send(app, cmd):
        raise RuntimeError("invented failure during the rescue")
    monkeypatch.setattr(live, "send", failing_send)
    try:
        with pytest.raises(RuntimeError, match="during the rescue"):
            live.run_case(live_args(account, exe), "case", "idle", "none", account.evidence)
        record = evidence_file(account, "home-run.json", account.evidence / "case")
        print("STANDIN finding 6 (exception):", json.dumps({"state": record["throwawayHome"]["state"],
                                                           "reason": record["throwawayHome"].get("reason"),
                                                           "compared": record["comparison"]["compared"]}, indent=1))
        assert record["throwawayHome"]["state"] == "left" and record["comparison"]["compared"] is True
        assert record["throwawayHome"]["reason"] == "the caller could not establish that the processes the run started ended"
    finally:
        for pid in started_pid(account):
            try:
                os.killpg(pid, signal.SIGKILL)
            except ProcessLookupError:
                pass


LIVE_SIGNALLED = SCRIPT_PRELUDE + textwrap.dedent("""
    from tools import agent_live_session_fixture as live
    from types import SimpleNamespace
    guard.SETTLE_SECONDS = 0
    for name, value in (("library", root / "real-home" / "Library"), ("cache", root / "system-cache"), ("real", root / "real-home")):
        setattr(guard.Run, name, value)
    args = SimpleNamespace(cancel_file=None, app=Path(sys.argv[3]), model="m", context_window=8192,
                           endpoint="http://127.0.0.1:9/v1", process_probe=root / "probe", boundary_probe=root / "probe")
    real_send = live.send
    def send(app, cmd):
        real_send(app, cmd)
        if cmd == "FINISH":
            print("RESCUE", flush=True)
    live.send = send
    live.run_case(args, "case", "idle", "none", (root / "evidence").resolve())
""")


@pytest.mark.parametrize("signum", [signal.SIGTERM, signal.SIGINT])
def test_standin_a_real_signal_during_the_rescue(account, signum):
    """Stand-in regression (finding 6): a fixture stand-in that ignores FINISH and stays alive; the
    controller is in the rescue wait when a real signal arrives; the finish still runs."""
    executable(account.tmp, body=READY + "\nimport time; time.sleep(60)")
    process = script(account, LIVE_SIGNALLED)
    try:
        line = process.stdout.readline()
        assert line == b"RESCUE\n", (line, process.stderr.read()[-2000:] if process.poll() is not None else b"")
        time.sleep(0.5)
        process.send_signal(signum)
        process.wait(timeout=60)
        record = evidence_file(account, "home-run.json", account.evidence / "case")
        print(f"STANDIN finding 6 (signal {signum}):", json.dumps({
            "controllerExit": process.returncode, "state": record["throwawayHome"]["state"],
            "reason": record["throwawayHome"].get("reason"), "compared": record["comparison"]["compared"]}, indent=1))
        assert process.returncode != 0 and record["throwawayHome"]["state"] == "left"
        assert record["comparison"]["compared"] is True
    finally:
        if process.poll() is None:
            process.kill()
            process.wait()
        for pid in started_pid(account):
            try:
                os.killpg(pid, signal.SIGKILL)
            except ProcessLookupError:
                pass
