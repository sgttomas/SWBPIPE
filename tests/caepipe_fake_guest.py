#!/usr/bin/env python3
"""An invented guest behind a fake `prlctl` executable, for tests/test_caepipe_append_only.py only.

`install(bin_dir)` puts a `prlctl` first on a test's PATH that runs this file, so the pilot command reaches it exactly
where it would reach the virtual machine tool and nowhere else. Each call answers one
`prlctl exec <vm> --current-user cmd /c <command>` from a JSON state file named by `CAEPIPE_FAKE_GUEST_STATE`:

- `view`: what the guest sees on its shared drives, {guest root (`Z:\\invented`): host folder};
- `listed`: further drive letters its `net use` lists with nothing behind them;
- `work`: its own work folder, {guest root (`C:\\invented-work`): host folder}, which may (by accident) be a folder
  inside the exchange folder;
- `solve`, `export`: how its solve and export behave (mode, CSV text, hashes it reports);
- `processes`: instances of the tool or guest scripts running in it, [{"kind": "tool" | "script", "pid": n,
  "start": text}]; a process a solve leaves running (a window showing) is added to `guest-running.json` beside the
  log, as the real guest would still list it;
- `final_local`: the guest's file system reports a local final path for its work folder although that folder lies in
  the exchange folder (only the probe can then show it);
- `fresh_dir_plant`: entries the guest's file system shows in a new work directory as soon as it exists, each
  {"name", "link_to": host path, "kind": "symlink" | "hardlink"} (an alias or a linked file);
- `actions`: what a concurrent actor does on the host before the n-th command of a kind;
- `faults`: a guest-side failure of the n-th command of a kind (`collect_file`: the k-th file of a collection
  written half and reported failed; `transport_after`: the command runs, part of a file is written when `partial`
  names it, and the transport then fails with exit 255);
- `answers`: per drive label, extra answer lines of the exchange check (a conflicting answer);
- `log`: a JSON-lines file each call appends to (its kind, command, exit status and any violation).

The guest's file operations are modelled as the real guest performs them, so that a test fails when a step relies on
anything weaker:
- `copy /Y` overwrites its destination, wherever it is, following a link as Windows does (a hard link is written
  through); `del` deletes and `mkdir` creates, wherever the path resolves. Any of these on a shared drive is also
  logged as a violation.
- The collection's `[IO.File]::Copy(source, destination, $false)` is a create-new of the destination: a name already
  there (a file or a link) is never written; the outcome is reported as the guest script does.
Nothing here runs or reads any external program; every byte it writes is invented.
"""

from __future__ import annotations

import base64
import hashlib
import json
import os
from pathlib import Path
import re
import shlex
import sys


STATE_ENV = "CAEPIPE_FAKE_GUEST_STATE"
INVENTED_KEY = "HKEY_CURRENT_USER\\Software\\Invented Vendor\\Tool\\1.0"
TOOL_PID = 4242
TRANSPORT_FAILURE = "Failed to exec: PrlJob_invented transport failure"
START = "2026-11-12T10:00:00.0000000Z"  # the start time the guest reports for the processes its script starts
RECYCLED_START = "2026-11-12T11:30:00.0000000Z"  # another process that reuses that id


def install(bin_dir: Path) -> Path:
    """A `prlctl` in `bin_dir` that runs this file with the interpreter running the tests."""
    bin_dir.mkdir(parents=True, exist_ok=True)
    shim = bin_dir / "prlctl"
    shim.write_text(f"#!{sys.executable}\nimport runpy, sys\nsys.argv[0] = {str(Path(__file__).resolve())!r}\n"
                    f"runpy.run_path({str(Path(__file__).resolve())!r}, run_name='__main__')\n", encoding="utf-8")
    shim.chmod(0o755)
    return shim


def read_log(state: dict) -> list[dict]:
    path = Path(state["log"])
    return [json.loads(line) for line in path.read_text(encoding="utf-8").splitlines()] if path.is_file() else []


# -- what the guest sees -------------------------------------------------------------------------------------------
def _letters(state: dict) -> list[str]:
    return sorted({root[:2].upper() for root in state.get("view", {})} | {item.upper() for item in state.get("listed", [])})


def _under(guest: str, root: str) -> str | None:
    """The part of `guest` below `root` (case-insensitively, as the guest's file system), or None."""
    g, r = guest.rstrip("\\"), root.rstrip("\\")
    if g.lower() == r.lower():
        return ""
    if g.lower().startswith(r.lower() + "\\"):
        return g[len(r) + 1:]
    return None


def host_of(state: dict, guest: str) -> tuple[str, Path | None]:
    """("shared" | "work" | "none", host path or None) for a guest path."""
    for area, key in (("shared", "view"), ("work", "work")):
        for root, host in state.get(key, {}).items():
            tail = _under(guest, root)
            if tail is not None:
                return area, Path(host).joinpath(*[part for part in tail.split("\\") if part])
    if guest[:2].upper() in _letters(state):
        return "shared", None  # a listed drive with nothing behind this path
    return "none", None


def _sha(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


# -- commands ---------------------------------------------------------------------------------------------------------
def _encoded(command: str) -> tuple[str, object] | None:
    """(script, payload) of an encoded PowerShell command whose data travel as base64 JSON."""
    if " -EncodedCommand " not in command:
        return None
    script = base64.b64decode(command.rsplit(" ", 1)[1]).decode("utf-16-le")
    found = re.search(r"FromBase64String\('([A-Za-z0-9+/=]*)'\)", script)
    return script, (json.loads(base64.b64decode(found[1])) if found else None)


def classify(command: str) -> str:
    if command == "net use":
        return "net_use"
    if command == "ver":
        return "ver"
    if command == "tasklist /FO CSV /NH":
        return "tasklist"
    if command.startswith("reg query "):
        return "reg_query"
    if command.startswith(("reg add", "reg delete")):
        return "reg_write"
    if command.startswith("type "):
        return "type"
    encoded = _encoded(command)
    if encoded is not None:
        script, payload = encoded
        if "collect done" in script:
            return payload["purpose"]
        if "Win32_Process" in script:
            return "processes"
        if "work ready" in script:
            return "work_directory"
        if "root resolved" in script:
            return "work_root"
        if "Get-FileHash" in script and isinstance(payload, list):
            return "exchange_check"
        return "encoded_other"
    if command.startswith("powershell ") and " -ExportMod " in command:
        return "export"
    if command.startswith("powershell ") and " -Mbf " in command:
        return "solve"
    if " del /Q " in command:
        return "export_prepare" if "model.mod" in command and "model.mbf" not in command else "prepare"
    if command.startswith("copy ") and ".ps1" in command:
        return "stage_script"
    if command.startswith("copy "):
        return "copy"
    return "other"


def _exchange_check(state: dict, command: str) -> tuple[int, str]:
    lines = []
    for item in _encoded(command)[1]:
        area, host = host_of(state, item["path"])
        if area == "shared" and host is not None and host.is_file() and not host.is_symlink():
            lines.append(f"sha256 {item['label']} {_sha(host.read_bytes())}")
            if state.get("answer_twice"):
                lines.append(f"sha256 {item['label']} {'0' * 64}")
        else:
            lines.append(f"missing {item['label']}")
        lines += state.get("answers", {}).get(item["label"], [])
    return 0, "\r\n".join(lines) + "\r\n"


def _net_use(state: dict) -> tuple[int, str]:
    rows = "".join(f"OK           {letter}        \\\\Mac\\invented-share-{letter[0]}    Parallels Shared Folders\r\n"
                   for letter in _letters(state))
    return 0, ("New connections will be remembered.\r\n\r\nStatus       Local     Remote                    Network\r\n"
               + "-" * 40 + "\r\n" + rows + "The command completed successfully.\r\n")


def _quoted(text: str) -> list[str]:
    return re.findall(r'"([^"]*)"', text)


def _copy_overwrite(state: dict, segment: str, violations: list[str]) -> int:
    """`copy /Y source destination` as Windows performs it: the destination is opened for writing and truncated,
    whatever is there (a link is written through)."""
    source, target = _quoted(segment)[:2]
    _, src = host_of(state, source)
    if src is None:
        return 1
    matches = sorted(item for item in src.parent.glob(src.name) if item.is_file()) if "*" in src.name else \
        ([src] if src.is_file() else [])
    if not matches:
        return 1
    area, dst = host_of(state, target)
    if dst is None:
        return 1
    for item in matches:
        destination = dst / item.name if dst.is_dir() else dst
        if area == "shared":
            violations.append(f"copy /Y onto a shared drive: {target}")
        with open(destination, "wb") as handle:  # overwrite, through any link, as the real guest does
            handle.write(item.read_bytes())
    return 0


def _shell(state: dict, command: str, violations: list[str]) -> int:
    """A cmd chain of `(if not exist .. mkdir ..)`, `del /Q ..` and `copy /Y ..`, joined by ` & `: each runs, as the
    real guest runs it, wherever its path resolves; the chain's exit status is the last one's."""
    code = 0
    for segment in command.split(" & "):
        segment = segment.strip()
        if segment.startswith("(if not exist"):
            path = _quoted(segment)[0]
            area, host = host_of(state, path)
            if area == "shared":
                violations.append(f"mkdir on a shared drive: {path}")
            if host is not None:
                host.mkdir(parents=True, exist_ok=True)
            code = 0 if host is not None else 1
        elif segment.startswith("del /Q"):
            code = 0
            for path in _quoted(segment):
                area, host = host_of(state, path)
                if area == "shared":
                    violations.append(f"del on a shared drive: {path}")
                if host is not None:
                    for item in (host.parent.glob(host.name) if "*" in host.name else [host]):
                        if item.is_file() or item.is_symlink():
                            item.unlink()
        elif segment.startswith("copy /Y"):
            code = _copy_overwrite(state, segment, violations)
        else:
            violations.append(f"unexpected command segment: {segment}")
            code = 1
    return code


def _collect(state: dict, payload: dict, fault: dict | None) -> tuple[int, str, str]:
    """The collection script: a create-new copy of each file, never over a name already there."""
    _, source = host_of(state, payload["from"])
    _, destination = host_of(state, payload["to"])
    names = list(payload.get("names", []))
    for pattern in payload.get("patterns", []):
        if source is not None and source.is_dir():
            names += sorted(item.name for item in source.glob(pattern) if item.is_file())
    lines = []
    for number, name in enumerate(dict.fromkeys(names), 1):
        src = source / name if source is not None else None
        if src is None or not src.is_file():
            lines.append(f"absent {name}")
            continue
        dst = destination / name
        data = src.read_bytes()
        partial = fault and (fault.get("collect_file") == number or fault.get("partial") == name)
        try:
            with open(dst, "xb") as handle:  # CopyFile with fail-if-exists: CREATE_NEW at the file system
                handle.write(data[:len(data) // 2] if partial else data)
        except FileExistsError:
            same = dst.is_file() and _sha(dst.read_bytes()) == _sha(data)
            lines.append(f"{'present_same' if same else 'present_differs'} {name}")
            continue
        if partial and fault.get("collect_file") == number:
            lines.append(f"failed {name}")
            continue
        if partial:  # the transport fails part-way through this file
            return 255, "\r\n".join(lines) + "\r\n", TRANSPORT_FAILURE
        lines.append(f"copied {name}")
    lines.append("collect done")
    return 0, "\r\n".join(lines) + "\r\n", ""


def _args(command: str) -> dict[str, str]:
    words = shlex.split(command.replace("\\", "\\\\"), posix=True)
    out: dict[str, str] = {}
    for index, word in enumerate(words):
        if word.startswith("-") and index + 1 < len(words) and not words[index + 1].startswith("-"):
            out[word[1:]] = words[index + 1]
        elif word.startswith("-"):
            out[word[1:]] = ""
    return out


def _solve(state: dict, command: str) -> tuple[int, str]:
    args = _args(command)
    _, script = host_of(state, args["File"])
    if script is None or not script.is_file():
        return 1, f"The argument '{args['File']}' to the -File parameter does not exist.\r\n"
    _, mbf = host_of(state, args["Mbf"])
    if mbf is None or not mbf.is_file():
        return 1, "no model\r\n"
    behaviour = state.get("solve", {})
    mode = behaviour.get("mode", "batch")
    if mode == "garbage":
        return 1, "the guest script stopped without a report\r\n"
    work, guest_work = mbf.parent, args["Mbf"].rsplit("\\", 1)[0]
    sha = _sha(mbf.read_bytes())
    # The program writes next to the model it was given, as the guest script directs it.
    (work / "model.res").write_bytes(b"INVENTED RESULTS " + sha.encode("ascii") + b"\r\n")
    (work / "model.mod").write_bytes(b"INVENTED SOLVED MODEL " + sha.encode("ascii") + b"\r\n")
    report: dict = {"mbf_sha256": (behaviour.get("report_sha") or sha).upper(),
                    "owned_pids": [{"pid": TOOL_PID, "start": START}]}
    if "PidFile" in args:
        _, pid_file = host_of(state, args["PidFile"])
        with open(pid_file, "a", encoding="ascii") as handle:  # the script's own new file in the fresh directory
            handle.write(f"{TOOL_PID} {START}\r\n")
    if mode == "window":  # the batch process shows a window: left running, nothing sent to it, the step stopped
        _leave_running(state, {"kind": "tool", "pid": TOOL_PID, "start": START})
        report.update(status="window_shown", csv_path=None, gui={"used": False}, batch={"stopped_own_process": False},
                      window_shown={"where": "batch", "pid": TOOL_PID, "windows": [
                          {"title": behaviour.get("window_title", "Invented Prompt"), "class": "#32770"}]})
        return 6, json.dumps(report) + "\r\n"
    if mode == "timeout_no_window":  # no window: the script stops its own process, as before
        report.update(status="no_results_after_batch", csv_path=None, gui={"used": False},
                      batch={"exited": False, "stopped_own_process": True})
        return 2, json.dumps(report) + "\r\n"
    if mode == "recycled":  # the process ended; another later took the same id
        _leave_running(state, {"kind": "tool", "pid": TOOL_PID, "start": RECYCLED_START})
        mode = "batch"
    if mode == "batch":
        (work / "model.csv").write_text(behaviour["csv"], encoding="utf-8")
        report.update(status="csv_from_batch", csv_path=f"{guest_work}\\model.csv", gui={"used": False})
    elif mode == "print_txt":  # the batch print in the .txt layout
        (work / "model.txt").write_text(behaviour["csv"], encoding="utf-8")
        report.update(status="csv_from_batch", csv_path=f"{guest_work}\\model.txt", gui={"used": False})
    elif mode == "no_csv":
        report.update(status="csv_missing", csv_path=None, gui={"used": False})
    elif "NoGuiFallback" in args:
        report.update(status="batch_no_csv", csv_path=None, gui={"used": False})
    else:
        (work / "model_results.csv").write_text(behaviour["csv"], encoding="utf-8")
        report.update(status="csv_from_gui_print", csv_path=f"{guest_work}\\model_results.csv", gui={"used": True})
    return 0, "WARNING: invented banner\r\n" + json.dumps(report) + "\r\n"


def _export(state: dict, command: str) -> tuple[int, str]:
    args = _args(command)
    _, script = host_of(state, args["File"])
    if script is None or not script.is_file():
        return 1, f"The argument '{args['File']}' to the -File parameter does not exist.\r\n"
    _, mod = host_of(state, args["ExportMod"])
    if mod is None or not mod.is_file():
        return 1, "no model\r\n"
    mod_sha = _sha(mod.read_bytes())
    exported = b"INVENTED\r\nEXPORTED MBF " + mod_sha.encode("ascii") + b"\r\n"
    (mod.parent / args["ExportName"]).write_bytes(exported)
    reported = state.get("export", {}).get("report_mod_sha") or mod_sha
    guest = args["ExportMod"].rsplit("\\", 1)[0] + "\\" + args["ExportName"]
    return 0, json.dumps({"status": "mbf_exported", "mod_sha256": reported.upper(),
                          "exported_mbf_sha256": _sha(exported), "exported_mbf_path": guest}) + "\r\n"


def _final(state: dict, guest: str) -> str:
    """The final path the guest's file system reports for `guest` (links followed): on the share when its real
    location lies in a shared folder, else the local path."""
    area, host = host_of(state, guest)
    if host is None and area == "none":
        return "\\\\?\\" + guest  # the guest's own disk, outside every mapping
    if host is None or not host.exists():
        return ""
    if area == "work" and state.get("final_local"):
        return "\\\\?\\" + guest
    real = Path(os.path.realpath(host))
    for root, shared in state.get("view", {}).items():
        base = Path(os.path.realpath(shared))
        if real == base or base in real.parents:
            tail = real.relative_to(base).as_posix().replace("/", "\\")
            return f"\\\\?\\UNC\\Mac\\invented-share-{root[0]}" + (f"\\{tail}" if tail else "")
    return "\\\\?\\" + guest


def _work_root(state: dict, payload: dict) -> tuple[int, str]:
    """The final paths of the exchange root and of the nearest existing folder at or above `path`; creates nothing."""
    lines = [f"root {_final(state, payload['root'])}"]
    path = payload["path"]
    while path:
        area, host = host_of(state, path)
        if (host is None and area == "none") or (host is not None and host.is_dir()):
            lines += [f"ancestor {path}", f"final {_final(state, path)}"]
            break
        up = path.rsplit("\\", 1)[0] if "\\" in path else ""
        path = "" if up == path else up
    lines.append("root resolved")
    return 0, "\r\n".join(lines) + "\r\n"


def _work_directory(state: dict, payload: dict) -> tuple[int, str]:
    """The fresh work directory: its parent made when absent, the directory itself made exclusively, the final paths
    reported, the probe created, the entries and destinations listed, wherever the paths resolve."""
    lines = [f"root {_final(state, payload['root'])}"]
    _, parent = host_of(state, payload["parent"])
    _, folder = host_of(state, payload["dir"])
    if parent is None or folder is None:
        return 1, "\r\n".join(lines) + "\r\n"
    parent.mkdir(parents=True, exist_ok=True)
    try:
        folder.mkdir()  # New-Item without -Force: fails when the name exists
    except FileExistsError:
        return 1, "\r\n".join(lines) + "\r\n"
    for plant in state.get("fresh_dir_plant", []):
        target = folder / plant["name"]
        if plant["kind"] == "symlink":
            target.symlink_to(plant["link_to"])
        else:
            os.link(plant["link_to"], target)
    lines.append(f"dir {_final(state, payload['dir'])}")
    with open(folder / payload["probe"], "xb"):
        pass
    lines += [f"entry {name}" for name in sorted(os.listdir(folder))]
    lines += [f"destination_{'present' if os.path.lexists(folder / name) else 'absent'} {name}"
              for name in payload["destinations"]]
    lines.append("work ready")
    return 0, "\r\n".join(lines) + "\r\n"


def _running_file(state: dict) -> Path:
    return Path(state["log"]).with_name("guest-running.json")


def _running(state: dict) -> list[dict]:
    path = _running_file(state)
    return list(state.get("processes", [])) + (json.loads(path.read_text()) if path.is_file() else [])


def _leave_running(state: dict, item: dict) -> None:
    _running_file(state).write_text(json.dumps([*(_running(state)[len(state.get("processes", [])):]), item]))


def _processes(state: dict) -> tuple[int, str]:
    if state.get("processes_unreadable"):
        return 1, ""
    lines = [f"{item['kind']} {item['pid']} {item.get('start', START)}" for item in _running(state)]
    return 0, "\r\n".join([*lines, "listed"]) + "\r\n"


# The invented saved settings: a licence-like value outside any restore scope, and the recent-files list inside the
# recent-files scope. Distinctive, so a test can look for each value, and the hash of each, in every output.
REGISTRY_VALUES = {
    (INVENTED_KEY + "\\Options", "Licence"): ("REG_SZ", "INVENTEDLICENCE7F3A9C2E"),
    (INVENTED_KEY + "\\Options", "Use something"): ("REG_BINARY", "5EC0DE5EC0DE5EC0"),
    (INVENTED_KEY + "\\Recent list", "1"): ("REG_BINARY", "0A0B0C0D0E0F1A1B"),
    (INVENTED_KEY + "\\Recent list", "N"): ("REG_BINARY", "C0FFEE0BC0FFEE0B"),
}


def _registry() -> str:
    lines = [""]
    for key in sorted({key for key, _ in REGISTRY_VALUES}):
        lines += [key] + [f"    {name}    {kind}    {data}" for (k, name), (kind, data) in sorted(REGISTRY_VALUES.items())
                          if k == key] + [""]
    return "\r\n".join(lines)


def _act(state: dict, kind: str, number: int) -> None:
    for action in state.get("actions", []):
        if action["before"] != kind or action.get("nth", 1) != number:
            continue
        path = Path(action.get("path", "."))
        if action["op"] == "write":  # the actor creates or rewrites a file as it likes
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_bytes(action["data"].encode("utf-8"))
        elif action["op"] == "mkdir":
            path.mkdir(parents=True, exist_ok=True)
        elif action["op"] == "link":  # the actor gives a stored file a second name
            path.parent.mkdir(parents=True, exist_ok=True)
            os.link(action["source"], path)
        elif action["op"] == "kill_parent":  # the harness process ends abruptly (SIGKILL), as a crash or kill would
            os.kill(os.getppid(), 9)


def answer(state: dict, command: str, number_of: dict[str, int]) -> tuple[str, int, str, str, list[str]]:
    kind = classify(command)
    number = number_of.get(kind, 0) + 1
    _act(state, kind, number)
    fault = next((item for item in state.get("faults", []) if item["kind"] == kind and item.get("nth", 1) == number),
                 None)
    violations: list[str] = []
    stdout, stderr = "", ""
    if kind == "net_use":
        code, stdout = _net_use(state)
    elif kind == "exchange_check":
        code, stdout = _exchange_check(state, command)
    elif kind == "ver":
        code, stdout = 0, "\r\nInvented Guest [Version 0.0]\r\n"
    elif kind == "tasklist":
        code, stdout = 0, '"System","4","Services","0","100 K"\r\n"explorer.exe","1200","Console","1","50,000 K"\r\n'
    elif kind == "processes":
        code, stdout = _processes(state)
    elif kind == "work_root":
        code, stdout = _work_root(state, _encoded(command)[1])
    elif kind == "work_directory":
        code, stdout = _work_directory(state, _encoded(command)[1])
    elif kind == "reg_query" and re.fullmatch(r'reg query "[^"]+" /s', command):
        code, stdout = 0, _registry()
    elif kind == "type":
        _, host = host_of(state, _quoted(command)[0])
        code, stdout = (0, host.read_text(encoding="ascii")) if host is not None and host.is_file() else (1, "")
    elif kind == "solve":
        code, stdout = _solve(state, command)
    elif kind == "export":
        code, stdout = _export(state, command)
    elif kind in ("collect", "export_collect", "prepare", "export_prepare", "stage_script", "export_stage_script"):
        code, stdout, stderr = _collect(state, _encoded(command)[1], fault)
    elif kind == "copy":
        code = _shell(state, command, violations)
    else:
        violations.append(f"a command the invented guest does not expect: {command[:120]}")
        code = 1
    if fault and fault.get("transport_after") and code != 255:
        code, stderr = 255, TRANSPORT_FAILURE  # the command ran; the transport then failed
    return kind, code, stdout, stderr, violations


def main(argv: list[str]) -> int:
    state = json.loads(Path(os.environ[STATE_ENV]).read_text(encoding="utf-8"))
    log = read_log(state)
    entry: dict = {"argv": argv[:5]}
    stdout = stderr = ""
    if len(argv) != 6 or argv[0] != "exec" or argv[2:5] != ["--current-user", "cmd", "/c"]:
        entry.update(kind="not_exec", exit=2, violations=["not one `exec <vm> --current-user cmd /c <command>`"])
        code = 2
    else:
        number_of: dict[str, int] = {}
        for item in log:
            number_of[item["kind"]] = number_of.get(item["kind"], 0) + 1
        kind, code, stdout, stderr, violations = answer(state, argv[5], number_of)
        entry.update(kind=kind, command=argv[5], exit=code, violations=violations)
    with open(state["log"], "a", encoding="utf-8") as handle:
        handle.write(json.dumps(entry) + "\n")
    sys.stdout.write(stdout)
    sys.stderr.write(stderr)
    return code


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
