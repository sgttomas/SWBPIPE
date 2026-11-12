"""Offline checks of the native launch guard and the controllers that use it. Nothing here starts
the application: the processes started are short-lived ones of the tests' own making. The account's
real Library, home folders, temporary folder and screen are never read: every check gets invented
ones, and every name in them is invented."""
from __future__ import annotations

import ast
import json
import os
from pathlib import Path
import plistlib
import re
import subprocess
import sys
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


@pytest.fixture
def library(tmp_path, monkeypatch):
    """An invented account: its home and Library, and the system's temporary and cache folders."""
    real = tmp_path / "real-home"
    for top in guard.WATCHED_TOPS:
        (real / "Library" / top).mkdir(parents=True)
    for folder in ("Desktop", "Documents", "Downloads"):
        (real / folder).mkdir()
    system_temp = tmp_path / "system-temp"
    system_temp.mkdir()
    system_cache = tmp_path / "system-cache"
    system_cache.mkdir()
    monkeypatch.setattr(guard, "real_home", lambda: real)
    monkeypatch.setattr(guard, "user_cache_dir", lambda: system_cache.resolve())
    monkeypatch.setattr(guard, "temp_root", lambda: system_temp.resolve())
    return real / "Library"


def evidence(tmp_path):
    folder = tmp_path / "evidence"
    folder.mkdir(exist_ok=True)
    return folder


def web_view_record(tmp_path):
    record = guard.make_home(evidence(tmp_path))
    data = Path(record["home"]) / "Library" / "WebKit" / EXE / "WebsiteData"
    data.mkdir(parents=True)
    (data / "salt").write_bytes(b"12345678")
    return record


def place(lib, top, name, leaf="data.bin"):
    path = lib / top / name / leaf
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_bytes(b"x")
    return path


MACOS = pytest.mark.skipif(sys.platform != "darwin", reason="the check for possible writers is made on macOS only")
HELPER = "com.apple.WebKit.Networking+" + EXE.replace("-", "_") + "-1a2b3c4d5e"


def in_cache(*parts, data=b"x"):
    path = guard.user_cache_dir().joinpath(*parts)
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_bytes(data)
    return path


NAMES_ONLY = [sys.executable, "-I", "-c", "import os; print('\\n'.join(sorted(os.environ)))"]


# ---------------------------------------------------------------------------
# 1. The environment is an allowlist the helper builds; nothing of the caller's passes.

def test_the_specification_is_the_allowlist_and_nothing_else(tmp_path, library):
    record = guard.make_home(evidence(tmp_path))
    launch = guard.spec(record, {"SWBPIPE_NATIVE_SMOKE_DRIVER": "-", "SWBPIPE_NATIVE_SMOKE_PROVIDER_KEY": "stdin"})
    env = launch["env"]
    assert set(env) == {"PATH", "HOME", "CFFIXED_USER_HOME", "TMPDIR", "LANG", guard.STRICT_CHOOSER_ENV,
                        "SWBPIPE_NATIVE_SMOKE_DRIVER", "SWBPIPE_NATIVE_SMOKE_PROVIDER_KEY"}
    assert env["HOME"] == env["CFFIXED_USER_HOME"] == record["home"] and env["TMPDIR"] == record["tmp"]
    assert env[guard.STRICT_CHOOSER_ENV] == "1" and env["PATH"] == guard.PATH
    assert launch["cwd"] == record["work"] and Path(record["work"]).parent == Path(record["home"])


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
    ("SWBPIPE_NATIVE_SMOKE_PROVIDER_KEY", "invented-secret", "provider_key_marker_only"),
])
def test_a_caller_cannot_pass_anything_but_the_kits_own_names(tmp_path, library, name, value, refusal):
    record = guard.make_home(evidence(tmp_path))
    with pytest.raises(guard.LaunchRefused) as raised:
        guard.spec(record, {name: value})
    assert str(raised.value).startswith(refusal)


def test_nothing_of_the_callers_environment_reaches_the_child(tmp_path, library, monkeypatch):
    monkeypatch.setenv("SWBPIPE_NATIVE_SMOKE_INJECT_RESULT", "invented")
    monkeypatch.setenv("INVENTED_API_KEY", "invented")
    monkeypatch.setenv("DYLD_INVENTED", "invented")
    monkeypatch.setattr(guard, "check_launch", lambda *a: {})
    record = guard.make_home(evidence(tmp_path))
    child = guard.start(NAMES_ONLY, record, {"SWBPIPE_NATIVE_SMOKE_DRIVER": "-"}, stdout=subprocess.PIPE)
    names = set(child.communicate(timeout=30)[0].decode().split())
    assert not names & {"SWBPIPE_NATIVE_SMOKE_INJECT_RESULT", "INVENTED_API_KEY", "DYLD_INVENTED"}
    expected = set(guard.spec(record, {"SWBPIPE_NATIVE_SMOKE_DRIVER": "-"})["env"])
    assert expected <= names and names - expected <= {"__CF_USER_TEXT_ENCODING"}, names - expected


def test_the_child_works_in_the_folder_the_helper_made_whatever_the_callers_own(tmp_path, library, monkeypatch):
    monkeypatch.setattr(guard, "check_launch", lambda *a: {})
    elsewhere = tmp_path / "callers-own"
    elsewhere.mkdir()
    monkeypatch.chdir(elsewhere)
    record = guard.make_home(evidence(tmp_path))
    child = guard.start([sys.executable, "-I", "-c", "import os; print(os.getcwd())"], record, stdout=subprocess.PIPE)
    assert Path(child.communicate(timeout=30)[0].decode().strip()).resolve() == Path(record["work"]).resolve()


def test_a_caller_cannot_give_its_own_environment_or_working_directory(tmp_path, library, monkeypatch):
    monkeypatch.setattr(guard, "check_launch", lambda *a: {})
    record = guard.make_home(evidence(tmp_path))
    for override in ({"env": dict(os.environ)}, {"cwd": str(tmp_path)}):
        with pytest.raises(guard.LaunchRefused, match="env_and_cwd_are_the_kits"):
            guard.start(NAMES_ONLY, record, **override)


def test_a_start_refused_before_the_screen_and_slot_checks_pass_starts_nothing(tmp_path, library, monkeypatch):
    record = guard.make_home(evidence(tmp_path))
    monkeypatch.setattr(guard, "_POPEN", lambda *a, **k: pytest.fail("started"))
    monkeypatch.setattr(guard, "check_launch", lambda *a: (_ for _ in ()).throw(guard.LaunchRefused("not_inside_the_slot_wrapper")))
    with pytest.raises(guard.LaunchRefused, match="not_inside_the_slot_wrapper"):
        guard.start(NAMES_ONLY, record)


@pytest.mark.parametrize("change, refusal", [
    (lambda e, r: e.pop("HOME"), "home_missing"),
    (lambda e, r: e.pop("CFFIXED_USER_HOME"), "cffixed_user_home_missing"),
    (lambda e, r: e.update(CFFIXED_USER_HOME="/elsewhere"), "homes_differ"),
    (lambda e, r: e.update(HOME="rel/home", CFFIXED_USER_HOME="rel/home"), "home_not_absolute"),
    (lambda e, r: e.update(HOME=str(guard.real_home()), CFFIXED_USER_HOME=str(guard.real_home())), "home_is_the_real_home"),
    (lambda e, r: e.update(HOME=str(guard.real_home().parent), CFFIXED_USER_HOME=str(guard.real_home().parent)), "home_is_the_real_home"),
    (lambda e, r: e.update(HOME=str(Path(r["home"]).parent / "other"), CFFIXED_USER_HOME=str(Path(r["home"]).parent / "other")), "home_not_the_kits"),
    (lambda e, r: e.update(TMPDIR="/tmp"), "environment_not_the_kits"),
    (lambda e, r: e.pop(guard.STRICT_CHOOSER_ENV), "environment_not_the_kits"),
])
def test_each_environment_refusal_is_named(tmp_path, library, change, refusal):
    record = guard.make_home(evidence(tmp_path))
    env = guard.spec(record)["env"]
    change(env, record)
    with pytest.raises(guard.LaunchRefused) as raised:
        guard.check_environment(env, record)
    assert str(raised.value) == refusal


def test_the_real_home_comes_from_the_password_database_not_the_environment(monkeypatch):
    monkeypatch.setenv("HOME", "/nowhere")
    import pwd
    assert guard.real_home() == Path(pwd.getpwuid(os.getuid()).pw_dir)


# ---------------------------------------------------------------------------
# 8. The strict chooser constants: set on every launch; held equal to the product's own copy.

def test_the_strict_chooser_is_set_on_every_launch_and_86_is_used_by_nothing_else():
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
# 5. The screen: launch only on this account's console session, on console, not locked.

def answer(*entries):
    return plistlib.dumps({"IORegistryEntryName": "Root", "IOConsoleUsers": list(entries)})


UID = 4242


@pytest.mark.parametrize("given, refusal", [
    (answer({"kCGSSessionUserIDKey": UID, "kCGSSessionOnConsoleKey": True}), None),
    (answer({"kCGSSessionUserIDKey": UID, "kCGSSessionOnConsoleKey": True, "CGSSessionScreenIsLocked": True}), "screen_locked"),
    (answer({"kCGSSessionUserIDKey": UID, "kCGSSessionOnConsoleKey": True, "CGSSessionScreenIsLocked": 1}), "screen_locked"),
    (answer(), "screen_no_session_for_this_account"),
    (answer({"kCGSSessionUserIDKey": UID + 1, "kCGSSessionOnConsoleKey": True}), "screen_no_session_for_this_account"),
    (answer({"kCGSSessionUserIDKey": UID, "kCGSSessionOnConsoleKey": False}), "screen_not_on_console"),
    (answer({"kCGSSessionUserIDKey": UID}), "screen_not_on_console"),
    (plistlib.dumps({"IORegistryEntryName": "Root"}), "screen_unparsed"),
    (b"not a property list", "screen_unparsed"),
])
def test_the_screen_check_launches_only_on_a_found_unlocked_console_session(given, refusal):
    assert guard.screen_refusal(given, uid=UID) == refusal


def test_the_further_signs_are_recorded_as_not_used_and_never_allow_a_launch():
    assert set(guard.SIGNS_NOT_USED) == {"screenSaver", "displaySleep"}
    assert all(v.startswith("not used:") for v in guard.SIGNS_NOT_USED.values())


# ---------------------------------------------------------------------------
# 6. The slot: an ancestor that is the lock program, alive, holding a slot lock file.

ME, SHELL, LOCK, TOP = 500, 400, 300, 200


def table(lock_exe="/usr/bin/lockf", lock_uid=None):
    uid = os.getuid() if lock_uid is None else lock_uid
    return {ME: (SHELL, ME, os.getuid(), sys.executable), SHELL: (LOCK, LOCK, os.getuid(), "/bin/bash"),
            LOCK: (TOP, LOCK, uid, lock_exe), TOP: (1, TOP, os.getuid(), "/bin/bash")}


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
    ({ME: (1, ME, os.getuid(), sys.executable)}, opens(), None, "not_inside_the_slot_wrapper"),
])
def test_the_slot_check_recognises_the_lock_program_holding_a_slot_lock(given, files, guard_dir, refusal):
    assert guard.slot_refusal(guard_dir, table=given, open_names=files, start=SHELL) == refusal


def test_a_wrapper_named_only_in_arguments_is_not_the_slot():
    # The shell running the wrapper script is the ancestor here; nothing reads its arguments.
    given = {ME: (SHELL, ME, os.getuid(), sys.executable), SHELL: (TOP, SHELL, os.getuid(), "/bin/bash"),
             TOP: (1, TOP, os.getuid(), "/bin/bash")}
    assert guard.slot_refusal(None, table=given, open_names=opens(), start=SHELL) == "not_inside_the_slot_wrapper"


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


def test_an_unreadable_process_table_refuses():
    def broken(pid):
        raise OSError("invented")
    assert guard.slot_refusal(None, table=table(), open_names=broken, start=SHELL) == "slot_unreadable"


# ---------------------------------------------------------------------------
# 3. The lister: product entries at any depth, removals, and failing closed.

@pytest.mark.parametrize("top", guard.WATCHED_TOPS)
@pytest.mark.parametrize("name", NAMES)
@pytest.mark.parametrize("change", ["created", "modified", "removed"])
def test_each_change_under_each_folder_for_each_name_is_a_product_change(tmp_path, library, top, name, change):
    record = web_view_record(tmp_path)
    existing = place(library, top, name) if change != "created" else None
    before = guard.snapshot(NAMES, library)
    if change == "created":
        place(library, top, name, "new.bin")
    elif change == "modified":
        existing.write_bytes(b"longer")
    else:
        existing.unlink()
    report = guard.compare(before, record, library=library)
    assert report["verdict"] == "product" and report["class"] == "product"
    assert change in [c["change"] for c in report["product"]["changes"]]


def test_a_name_four_levels_down_and_with_a_dot_suffix_is_watched(tmp_path, library):
    record = web_view_record(tmp_path)
    before = guard.snapshot(NAMES, library)
    (library / "Preferences" / f"{BUNDLE}.plist").write_bytes(b"p")
    deep = library / "Caches" / "invented-a" / "invented-b" / "invented-c" / EXE
    deep.mkdir(parents=True)
    report = guard.compare(before, record, library=library)
    assert {c["path"] for c in report["product"]["changes"]} == {f"Preferences/{BUNDLE}.plist", f"Caches/invented-a/invented-b/invented-c/{EXE}"}


def test_a_top_folder_that_cannot_be_listed_fails_the_comparison(tmp_path, library):
    record = web_view_record(tmp_path)
    before = guard.snapshot(NAMES, library)
    (library / "HTTPStorages").chmod(0)
    try:
        report = guard.compare(before, record, library=library)
    finally:
        (library / "HTTPStorages").chmod(0o700)
    assert report["verdict"] == "comparison failed" and report["failure"]["foldersNotListed"] == ["HTTPStorages"]
    assert guard.EXIT["comparison failed"] not in (guard.EXIT["clean"], guard.EXIT["product"])


def test_a_folder_that_becomes_unreadable_during_the_run_fails_the_comparison(tmp_path, library):
    record = web_view_record(tmp_path)
    other = place(library, "Application Support", "invented-other-app").parent
    before = guard.snapshot(NAMES, library)
    other.chmod(0)
    try:
        report = guard.compare(before, record, library=library)
    finally:
        other.chmod(0o700)
    assert report["verdict"] == "comparison failed" and report["failure"]["newlyUnreadableCount"] == 1


def test_a_folder_unreadable_before_and_after_is_a_stated_limit(tmp_path, library):
    record = web_view_record(tmp_path)
    other = place(library, "Application Support", "invented-protected").parent
    other.chmod(0)
    try:
        before = guard.snapshot(NAMES, library)
        report = guard.compare(before, record, library=library)
    finally:
        other.chmod(0o700)
    assert report["verdict"] == "clean" and report["limit"]["unreadableBeforeAndAfterCount"] == 1


def test_an_unreadable_watched_folder_is_a_product_change(tmp_path, library):
    record = web_view_record(tmp_path)
    folder = place(library, "WebKit", BUNDLE).parent
    before = guard.snapshot(NAMES, library)
    folder.chmod(0)
    try:
        report = guard.compare(before, record, library=library)
    finally:
        folder.chmod(0o700)
    assert report["verdict"] == "product" and report["product"]["unreadableWatched"] == [f"WebKit/{BUNDLE}"]


def test_an_unreadable_file_is_listed_by_name_so_nothing_is_opened(tmp_path, library):
    path = place(library, "Caches", BUNDLE, "locked.db")
    path.chmod(0)
    try:
        assert guard.snapshot(NAMES, library)["entries"][f"Caches/{BUNDLE}/locked.db"]["size"] == 1
    finally:
        path.chmod(0o600)


def test_a_link_is_listed_and_not_followed(tmp_path, library):
    outside = tmp_path / "outside"
    (outside / "deep").mkdir(parents=True)
    (outside / "deep" / "file").write_bytes(b"x")
    (library / "Caches" / BUNDLE).mkdir()
    (library / "Caches" / BUNDLE / "link").symlink_to(outside)
    entries = guard.snapshot(NAMES, library)["entries"]
    assert entries[f"Caches/{BUNDLE}/link"]["kind"] == "link" and not any("deep" in p for p in entries)


def test_the_snapshot_holds_product_named_entries_only(tmp_path, library):
    place(library, "Application Support", "invented-other-app")
    place(library, "Caches", BUNDLE)
    assert set(guard.snapshot(NAMES, library)["entries"]) == {f"Caches/{BUNDLE}", f"Caches/{BUNDLE}/data.bin"}


def test_others_are_counted_named_only_under_preferences_and_never_fail(tmp_path, library):
    record = web_view_record(tmp_path)
    before = guard.snapshot(NAMES, library)
    (library / "Preferences" / "com.invented.service.plist").write_bytes(b"s")
    place(library, "Caches", "invented-other-app")
    place(library, "Application Support", "invented-other-app", "private-name.db")
    place(library, "Caches", f"{BUNDLE}-other")  # shares a prefix, is not the product
    report = guard.compare(before, record, until=time.time() + 1, library=library)
    assert report["verdict"] == "clean" and report["class"] == "others"
    assert [n["path"] for n in report["others"]["named"]] == ["Preferences/com.invented.service.plist"]
    assert "private-name" not in json.dumps(report)
    assert "serialised" in report["window"]["statement"] and "nothing was read" in report["limits"]
    assert "Nothing outside the real Library's six folders, the system's per-user cache folder and the throwaway home was watched" in report["watched"]


def test_nothing_changed_is_its_own_class(tmp_path, library):
    record = web_view_record(tmp_path)
    before = guard.snapshot(NAMES, library)
    report = guard.compare(before, record, until=before["taken"], library=library)
    assert report["class"] == "nothing" and report["verdict"] == "clean"


def test_no_web_view_data_in_the_home_has_its_own_verdict(tmp_path, library):
    record = guard.make_home(evidence(tmp_path))
    report = guard.compare(guard.snapshot(NAMES, library), record, library=library)
    assert report["verdict"] == "no web view data" and report["throwawayHome"]["webViewData"] is False


# The system's per-user cache folder: the web view helpers' folders are their own class.

@pytest.mark.parametrize("component, expected", [
    ("com.apple.WebKit.Networking+" + EXE, True),
    ("com.apple.WebKit.GPU+" + EXE + "-1a2b3c", True),
    ("com.apple.WebKit.WebContent+" + BUNDLE, True),
    ("com.apple.WebKit.WebContent.EnhancedSecurity+" + BUNDLE + ".helper", True),
    ("com.apple.WebKit.Networking+" + EXE.replace("-", "_") + "-1a2b3c4d5e", True),
    ("com.apple.WebKit.Networking+" + EXE + "x", False),
    ("com.apple.WebKit.Networking+" + EXE.replace("-", "_") + "x", False),
    ("com.apple.WebKit.Networking+com.invented.other", False),
    (EXE + "-1a2b3c", False),
])
def test_a_watched_name_after_a_plus_sign_is_watched(component, expected):
    assert guard.watched(f"{guard.CACHE_TOP}/{component}/data", NAMES) is expected


def test_the_web_view_helpers_cache_folders_are_their_own_class_and_stop_the_run_by_default(tmp_path, library):
    record = web_view_record(tmp_path)
    before = guard.snapshot(NAMES, library)
    in_cache(HELPER, "data")
    in_cache("com.apple.WebKit.GPU+" + BUNDLE, "data")
    assert guard.HELPER_CACHE_COUNTS_AS_CLEAN is False
    report = guard.compare(before, record, library=library)
    assert report["verdict"] == "web view helper cache" and report["class"] == "web view helper cache"
    assert report["product"]["changes"] == [] and len(report["helperCache"]["changes"]) == 4
    assert all(c["path"].startswith(guard.CACHE_TOP + "/com.apple.WebKit.") for c in report["helperCache"]["changes"])
    lenient = guard.compare(before, record, library=library, helper_cache_clean=True)
    assert lenient["verdict"] == "clean" and lenient["class"] == "web view helper cache"


@pytest.mark.parametrize("parts", [(BUNDLE, "data"), (EXE, "data"), ("invented", "com.apple.WebKit.Networking+" + EXE, "data"),
                                   ("com.apple.WebKit.Invented+" + EXE, "data")])
def test_a_product_named_entry_of_any_other_shape_in_the_cache_folder_trips_the_wire(tmp_path, library, parts):
    record = web_view_record(tmp_path)
    before = guard.snapshot(NAMES, library)
    in_cache(*parts)
    report = guard.compare(before, record, library=library, helper_cache_clean=True)
    assert report["verdict"] == "product" and report["product"]["changes"] and report["helperCache"]["changes"] == []


def test_the_cache_folder_fails_closed_and_is_never_named_by_its_path(tmp_path, library, monkeypatch):
    record = web_view_record(tmp_path)
    cache = guard.user_cache_dir()
    placed = in_cache(BUNDLE, "data")
    named = guard.snapshot(NAMES, library)
    assert str(cache) not in json.dumps(named) and f"{guard.CACHE_TOP}/{BUNDLE}/data" in named["entries"]
    guard.shutil.rmtree(placed.parent)
    before = guard.snapshot(NAMES, library)
    cache.chmod(0)
    try:
        report = guard.compare(before, record, library=library)
    finally:
        cache.chmod(0o700)
    assert report["verdict"] == "comparison failed" and report["failure"]["foldersNotListed"] == [guard.CACHE_TOP]
    monkeypatch.setattr(guard, "user_cache_dir", lambda: None)
    report = guard.compare(before, record, library=library)
    assert report["verdict"] == "comparison failed" and report["failure"]["foldersNotListed"] == [guard.CACHE_TOP]


def test_other_entries_in_the_cache_folder_are_counted_in_the_window_never_named(tmp_path, library):
    record = web_view_record(tmp_path)
    before = guard.snapshot(NAMES, library)
    in_cache("com.invented.other", "private-name.db")
    report = guard.compare(before, record, until=time.time() + 1, library=library)
    assert report["verdict"] == "clean" and report["others"]["byFolder"] == {guard.CACHE_TOP: 2}
    assert "private-name" not in json.dumps(report)


# ---------------------------------------------------------------------------
# 4. Four folder times by a single stat each; nothing inside them listed or opened.

def test_the_four_home_folders_are_stat_only(tmp_path, library, monkeypatch):
    real = guard.real_home()
    for folder in ("Desktop", "Documents", "Downloads"):
        (real / folder).chmod(0)
    listed = []
    original = guard.os.scandir
    monkeypatch.setattr(guard.os, "scandir", lambda p=".": listed.append(str(p)) or original(p))
    try:
        times = guard.folder_times()
    finally:
        for folder in ("Desktop", "Documents", "Downloads"):
            (real / folder).chmod(0o700)
    assert set(times) == {"~", "Desktop", "Documents", "Downloads"} and all(isinstance(t, float) for t in times.values())
    assert not any(p.startswith(str(real / f)) for p in listed for f in ("Desktop", "Documents", "Downloads"))
    assert listed == []


def test_a_change_of_the_four_times_is_reported_not_concluded_from(tmp_path, library):
    record = web_view_record(tmp_path)
    before = guard.snapshot(NAMES, library)
    os.utime(guard.real_home() / "Downloads", (1, 1))
    report = guard.compare(before, record, library=library)
    assert report["folderTimes"]["changed"] == ["Downloads"] and report["verdict"] == "clean"


# ---------------------------------------------------------------------------
# 7. The home: made by the kit under the system's temporary folder; removal confined.

def test_the_kit_makes_the_home_and_records_it_outside(tmp_path, library):
    record = guard.make_home(evidence(tmp_path))
    home = Path(record["home"])
    assert home.parent == guard.temp_root() and Path(record["tmp"]).is_dir() and Path(record["work"]).is_dir()
    assert json.loads((evidence(tmp_path) / "home-record.json").read_text()) == record
    assert str(guard.temp_root()) in record["identity"] and record["home"] in record["identity"]


def test_remove_home_lists_outside_first_then_removes_and_unlinks_links_without_following(tmp_path, library, monkeypatch):
    monkeypatch.setattr(guard, "possible_writers", lambda *a, **k: [])
    record = guard.make_home(evidence(tmp_path))
    home = Path(record["home"])
    outside = tmp_path / "outside"
    outside.mkdir()
    (outside / "kept").write_bytes(b"x")
    (home / "a").mkdir()
    (home / "a" / "file").write_bytes(b"12")
    (home / "a" / "link").symlink_to(outside)
    outcome = guard.remove_home(record, evidence(tmp_path))
    assert outcome["cleanup"] == "removed" and not home.exists() and (outside / "kept").exists()
    listing = json.loads(Path(outcome["listing"]).read_text())
    assert Path(outcome["listing"]).parent == evidence(tmp_path) and "a/link" in {e["path"] for e in listing["entries"]}


def test_remove_home_refuses_a_replaced_folder_a_link_and_a_home_not_the_kits(tmp_path, library, monkeypatch):
    monkeypatch.setattr(guard, "possible_writers", lambda *a, **k: [])
    record = guard.make_home(evidence(tmp_path))
    home = Path(record["home"])
    replaced = dict(record)
    guard.shutil.rmtree(home)
    home.mkdir()
    (home / "tmp").mkdir()
    (home / "work").mkdir()
    assert guard.remove_home(replaced, evidence(tmp_path))["cleanup"] == "refused" and home.is_dir()
    guard.shutil.rmtree(home)
    outside = tmp_path / "outside"
    outside.mkdir()
    (outside / "kept").write_bytes(b"x")
    home.symlink_to(outside)
    assert guard.remove_home(record, evidence(tmp_path))["cleanup"] == "refused" and (outside / "kept").exists()
    home.unlink()
    foreign = {**guard.make_home(evidence(tmp_path), "other"), "home": str(outside)}
    assert guard.remove_home(foreign, evidence(tmp_path))["cleanup"] == "refused" and (outside / "kept").exists()


def test_remove_home_refuses_when_a_folder_inside_the_record_was_replaced(tmp_path, library, monkeypatch):
    monkeypatch.setattr(guard, "possible_writers", lambda *a, **k: [])
    record = guard.make_home(evidence(tmp_path))
    work = Path(record["work"])
    work.rmdir()
    work.mkdir()
    assert guard.remove_home(record, evidence(tmp_path))["cleanup"] == "refused" and Path(record["home"]).is_dir()


def test_a_home_replaced_after_the_first_check_is_not_removed(tmp_path, library, monkeypatch):
    record = guard.make_home(evidence(tmp_path))
    home = Path(record["home"])
    def replace(*a, **k):
        guard.shutil.rmtree(home)
        (home / "work").mkdir(parents=True)
        (home / "tmp").mkdir()
        (home / "kept").write_bytes(b"x")
        return []
    monkeypatch.setattr(guard, "possible_writers", replace)
    outcome = guard.remove_home(record, evidence(tmp_path))
    assert outcome["cleanup"] == "failed" and (home / "kept").is_file()


def test_a_home_whose_processes_are_not_verified_gone_is_withheld_listed_and_named(tmp_path, library):
    record = guard.make_home(evidence(tmp_path))
    outcome = guard.remove_home(record, evidence(tmp_path), processes_gone=False)
    assert outcome["cleanup"] == "withheld" and outcome["home"] == record["home"] and Path(record["home"]).is_dir()
    assert Path(outcome["listing"]).is_file()


def test_a_failed_listing_is_recorded_and_the_home_still_removed(tmp_path, library, monkeypatch):
    monkeypatch.setattr(guard, "possible_writers", lambda *a, **k: [])
    record = guard.make_home(evidence(tmp_path))
    monkeypatch.setattr(guard, "walk", lambda *a, **k: (_ for _ in ()).throw(OSError("invented")))
    outcome = guard.remove_home(record, evidence(tmp_path))
    assert outcome["cleanup"] == "removed" and "OSError" in (outcome["listingError"] or "")


# Possible writers: real short-lived processes of the test's own making.

def withheld_by(tmp_path, start, why, **check):
    record = guard.make_home(evidence(tmp_path))
    home = Path(record["home"])
    process = start(home)
    try:
        time.sleep(0.3)
        outcome = guard.remove_home(record, evidence(tmp_path), **{k: (v(process) if callable(v) else v) for k, v in check.items()})
    finally:
        os.killpg(process.pid, 9)
        process.wait()
        time.sleep(0.2)
    assert outcome["cleanup"] == "withheld" and home.is_dir(), outcome
    assert any(why in w["why"] for w in outcome["writers"]), outcome["writers"]
    after = guard.remove_home(record, evidence(tmp_path))
    assert after["cleanup"] == "removed" and not home.exists(), after


@MACOS
def test_a_process_the_run_started_or_its_descendant_withholds_removal(tmp_path, library):
    withheld_by(tmp_path, lambda home: subprocess.Popen(["/bin/sh", "-c", "/bin/sleep 30"],
                env={"HOME": str(home), "CFFIXED_USER_HOME": str(home)}, start_new_session=True),
                "started by the run, or a descendant", started=lambda p: [p.pid])


@MACOS
def test_a_process_with_a_file_open_under_the_home_withholds_removal(tmp_path, library):
    withheld_by(tmp_path, lambda home: subprocess.Popen(["/bin/sh", "-c", f"exec 3>'{home}/open.txt'; /bin/sleep 30"],
                start_new_session=True), "open under the home")


@MACOS
def test_a_process_working_under_the_home_withholds_removal(tmp_path, library):
    withheld_by(tmp_path, lambda home: subprocess.Popen(["/bin/sleep", "30"], cwd=home, start_new_session=True),
                "open under the home")


@MACOS
def test_a_process_in_a_group_the_run_started_withholds_removal(tmp_path, library):
    withheld_by(tmp_path, lambda home: subprocess.Popen(["/bin/sh", "-c", "/bin/sleep 30 & wait"], start_new_session=True),
                "in a group the run started", groups=lambda p: [p.pid])


@MACOS
def test_a_process_of_the_private_copy_withholds_removal(tmp_path, library):
    # The executable itself stands for the private copy: the system will not run a copied system binary.
    withheld_by(tmp_path, lambda home: subprocess.Popen(["/bin/sleep", "30"], start_new_session=True),
                "executable under the home or the private copy", private=[Path("/bin/sleep")])


def test_on_any_other_system_the_check_cannot_run_and_the_home_is_withheld(tmp_path, library, monkeypatch):
    monkeypatch.setattr(guard, "PLATFORM", "linux")
    record = guard.make_home(evidence(tmp_path))
    outcome = guard.remove_home(record, evidence(tmp_path))
    assert outcome["cleanup"] == "withheld" and "could not run" in outcome["reason"] and Path(record["home"]).is_dir()


def test_a_check_that_cannot_run_withholds_removal(tmp_path, library, monkeypatch):
    record = guard.make_home(evidence(tmp_path))
    monkeypatch.setattr(guard, "possible_writers", lambda *a, **k: (_ for _ in ()).throw(OSError("invented")))
    outcome = guard.remove_home(record, evidence(tmp_path))
    assert outcome["cleanup"] == "withheld" and "could not run" in outcome["reason"] and Path(record["home"]).is_dir()


def test_the_command_line_takes_no_library_and_reports_by_stable_exit_status(tmp_path, library, monkeypatch, capsys):
    monkeypatch.setattr(guard, "real_library", lambda: library)
    monkeypatch.setattr(guard, "possible_writers", lambda *a, **k: [])
    assert guard.main(["make-home", "--evidence", str(evidence(tmp_path))]) == 0
    record = json.loads(capsys.readouterr().out)
    record_file = evidence(tmp_path) / "home-record.json"
    (Path(record["home"]) / "Library" / "Caches" / EXE).mkdir(parents=True)
    (Path(record["home"]) / "Library" / "Caches" / EXE / "f").write_bytes(b"x")
    assert guard.main(["spec", "--record", str(record_file), "--set", "SWBPIPE_NATIVE_SMOKE_DRIVER=-"]) == 0
    launch = json.loads(capsys.readouterr().out)
    assert launch["env"]["CFFIXED_USER_HOME"] == record["home"] and launch["cwd"] == record["work"]
    assert guard.main(["spec", "--record", str(record_file), "--set", "INVENTED_API_KEY=x"]) == 2
    before = tmp_path / "before.json"
    assert guard.main(["snapshot", "--name", BUNDLE, "--name", EXE, "--out", str(before)]) == 0
    assert guard.main(["compare", "--before", str(before), "--record", str(record_file)]) == 0
    in_cache(HELPER, "data")
    assert guard.main(["compare", "--before", str(before), "--record", str(record_file)]) == 6
    place(library, "HTTPStorages", BUNDLE)
    assert guard.main(["compare", "--before", str(before), "--record", str(record_file)]) == 1
    assert guard.main(["remove-home", "--record", str(record_file), "--evidence", str(evidence(tmp_path)), "--processes-not-gone"]) == 4
    assert guard.main(["remove-home", "--record", str(record_file), "--evidence", str(evidence(tmp_path))]) == 0
    for wrong in (["snapshot", "--name", BUNDLE, "--out", str(before), "--library", str(library)],
                  ["spec", "--record", str(record_file), "--env", "x"]):
        with pytest.raises(SystemExit):
            guard.main(wrong)
    capsys.readouterr()
    assert guard.EXIT == {"clean": 0, "product": 1, "refused": 2, "no web view data": 3, "not removed": 4,
                          "comparison failed": 5, "web view helper cache": 6}


# ---------------------------------------------------------------------------
# The controllers.

class Started:
    pid = 2 ** 22 + 7  # an identifier no process here holds

    def __init__(self, argv, env=None, cwd=None, **kwargs):
        self.argv, self.env, self.cwd = argv, env, cwd


def journey_guard(tmp_path, library, monkeypatch):
    monkeypatch.setattr(journey.LaunchGuard, "library", library)
    monkeypatch.setattr(journey.time, "sleep", lambda *a: None)
    monkeypatch.setattr(guard, "check_launch", lambda *a: {})
    monkeypatch.setattr(guard, "_POPEN", Started)
    app = tmp_path / "Copy.app" / "Contents" / "MacOS" / EXE
    app.parent.mkdir(parents=True)
    root = tmp_path / "run"
    root.mkdir()
    return journey.LaunchGuard(app, root), app, root


def test_every_start_the_journey_makes_carries_the_kits_specification(tmp_path, library, monkeypatch):
    launch_guard, app, root = journey_guard(tmp_path, library, monkeypatch)
    with pytest.raises(journey.Refusal, match="launch_guard_not_started"):
        launch_guard.launch("preflight", [str(app)], {})
    launch_guard.start()
    for role in ("preflight", "app"):
        started = launch_guard.launch(role, [str(app)], {"SWBPIPE_NATIVE_SMOKE_DRIVER": "-"})
        assert started.env["HOME"] == started.env["CFFIXED_USER_HOME"] == launch_guard.record["home"]
        assert started.env["TMPDIR"] == launch_guard.record["tmp"] and started.cwd == launch_guard.record["work"]
    assert launch_guard.launches == ["preflight", "app"] and set(launch_guard.names) >= {EXE, BUNDLE}


def test_the_journey_refuses_a_start_with_a_variable_the_kit_does_not_allow(tmp_path, library, monkeypatch):
    launch_guard, app, root = journey_guard(tmp_path, library, monkeypatch)
    launch_guard.start()
    with pytest.raises(journey.Refusal, match="launch_refused_injection_variable"):
        launch_guard.launch("app", [str(app)], {"SWBPIPE_NATIVE_SMOKE_INJECT_RESULT": "x"})
    assert launch_guard.launches == []


def test_the_preflight_goes_through_the_guard(tmp_path, library, monkeypatch):
    launch_guard, app, root = journey_guard(tmp_path, library, monkeypatch)
    seen = []
    monkeypatch.setattr(launch_guard, "launch", lambda role, argv, extra, **popen: seen.append((role, argv)) or (_ for _ in ()).throw(journey.Refusal("stop")))
    args = SimpleNamespace(app=app, process_probe=Path("p"), boundary_probe=Path("b"))
    with pytest.raises(journey.Refusal):
        journey.preflight(args, root, launch_guard)
    assert seen == [("preflight", [str(app), "--native-agent-session-fixture"])]


def test_the_controllers_start_the_application_only_through_the_helper():
    for module, allowed in ((journey, set()), (live, {"anchor_start"})):
        tree = ast.parse(Path(module.__file__).read_text())
        starts = {node.name for node in ast.walk(tree) if isinstance(node, ast.FunctionDef)
                  for call in ast.walk(node) if isinstance(call, ast.Call) and ast.unparse(call.func) in ("subprocess.Popen", "Popen")}
        assert starts == allowed, (module.__name__, starts)


def run_ending(tmp_path, library, monkeypatch, ending):
    _, app, root = journey_guard(tmp_path, library, monkeypatch)
    monkeypatch.setattr(journey, "sha256", lambda p: "0" * 64)
    monkeypatch.setattr(guard, "possible_writers", lambda *a, **k: [])
    def journey_body(args, key, root, launch_guard, result):
        launch_guard.launch("app", [str(app)], {})
        ending(result)
    monkeypatch.setattr(journey, "journey", journey_body)
    return root, (lambda: journey.run(SimpleNamespace(app=app, mode="stub"), None, root))


def test_a_failed_run_and_an_interrupted_run_leave_no_home_and_do_leave_its_listing(tmp_path, library, monkeypatch):
    def refused(result):
        raise journey.Refusal("preflight")
    root, call = run_ending(tmp_path, library, monkeypatch, refused)
    result = call()
    assert result["failure"] == "preflight" and result["launchGuard"]["compared"] is True
    assert result["throwawayHome"]["cleanup"] == "removed" and not Path(result["throwawayHome"]["home"]).exists()
    assert (root / "home-listing.json").is_file()
    def interrupted(result):
        raise KeyboardInterrupt
    (tmp_path / "second").mkdir()
    root2, call2 = run_ending(tmp_path / "second", library, monkeypatch, interrupted)
    with pytest.raises(KeyboardInterrupt):
        call2()
    record = json.loads((root2 / "home-record.json").read_text())
    assert not Path(record["home"]).exists() and (root2 / "home-listing.json").is_file()


def test_a_product_change_fails_the_run_even_on_refusal(tmp_path, library, monkeypatch):
    def leaked(result):
        place(library, "WebKit", BUNDLE)
        raise journey.Refusal("preflight")
    root, call = run_ending(tmp_path, library, monkeypatch, leaked)
    result = call()
    assert result["launchGuard"]["verdict"] == "product" and journey.require_comparison(result) == journey.GUARD_EXIT


def test_a_run_whose_processes_may_be_alive_withholds_its_home_fails_and_names_it(tmp_path, library, monkeypatch):
    def retained(result):
        result["rescue"] = "retained_original_app_custody"
    root, call = run_ending(tmp_path, library, monkeypatch, retained)
    result = call()
    home = result["throwawayHome"]["home"]
    assert result["throwawayHome"]["cleanup"] == "withheld" and Path(home).is_dir()
    assert journey.require_comparison(result) == journey.GUARD_EXIT and home in result["failure"]


def test_no_result_reads_as_a_pass_without_a_clean_comparison():
    assert journey.require_comparison({"passed": True}) == journey.GUARD_EXIT
    for verdict in ("product", "no web view data", "comparison failed"):
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


def test_the_live_session_fixture_starts_the_app_through_the_helper_and_removes_its_home(tmp_path, library, monkeypatch):
    seen = {}
    def started(argv, env=None, cwd=None, **kwargs):
        seen.update(env=env, cwd=cwd)
        raise RuntimeError("not launched in an offline test")
    monkeypatch.setattr(guard, "_POPEN", started)
    monkeypatch.setattr(guard, "check_launch", lambda *a: {})
    monkeypatch.setattr(guard, "possible_writers", lambda *a, **k: [])
    root = tmp_path / "run"
    root.mkdir()
    args = SimpleNamespace(cancel_file=None, app=tmp_path / EXE, model="m", context_window=1, endpoint="e",
                           process_probe=Path("p"), boundary_probe=Path("b"))
    result = live.run_case(args, "case", "stub", "none", root.resolve())
    record = json.loads((root / "case-record.json").read_text())
    assert seen["env"]["HOME"] == seen["env"]["CFFIXED_USER_HOME"] == record["home"] and seen["cwd"] == record["work"]
    assert result["failure"] == "RuntimeError" and result["throwawayHome"]["cleanup"] == "removed"
    assert not Path(record["home"]).exists() and (root / "case-listing.json").is_file()
