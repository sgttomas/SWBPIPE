#!/usr/bin/env python3
"""An invented guest behind a fake `prlctl` executable, for tests/test_caepipe_append_only.py only.

`install(bin_dir)` puts a `prlctl` first on a test's PATH that runs this file, so the pilot command reaches it exactly
where it would reach the virtual machine tool and nowhere else. Each call answers one
`prlctl exec <vm> --current-user cmd /c <command>` from a JSON state file named by `CAEPIPE_FAKE_GUEST_STATE`:

- `view`: what the guest sees on its shared drives, {guest root (`Z:\\invented`): host folder};
- `listed`: further drive letters its `net use` lists with nothing behind them;
- `work`: its own work folder, {guest root (`C:\\invented-work`): host folder};
- `solve`, `export`: how its solve and export behave (mode, CSV text, hashes it reports);
- `actions`: what a concurrent actor does on the host before the n-th command of a kind;
- `answer_twice`: the exchange check answers every label twice, with different hashes;
- `log`: a JSON-lines file each call appends to (its kind, command, exit status and any violation).

The guest's containment is checked here, as a guest would break it: a `mkdir` or `del` of a path on any drive the
guest lists as shared, and a `copy` onto any file that already exists on a shared drive, are violations (logged, and
the command fails); every `copy` onto a shared drive creates its file exclusively. Nothing here runs or reads any
external program; every byte it writes is invented.
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
    if " -EncodedCommand " in command:
        return "exchange_check" if _exchange_items(command) is not None else "encoded_other"
    if command.startswith("powershell ") and " -ExportMod " in command:
        return "export"
    if command.startswith("powershell ") and " -Mbf " in command:
        return "solve"
    if " del /Q " in command:
        return "export_prepare" if "model.mod" in command and "model.mbf" not in command else "prepare"
    if command.startswith("copy ") and ".ps1" in command:
        return "stage_script"
    if command.startswith("copy ") and "exported.mbf" in command:
        return "export_collect"
    if command.startswith("copy "):
        return "collect"
    return "other"


def _exchange_items(command: str) -> list[dict] | None:
    script = base64.b64decode(command.rsplit(" ", 1)[1]).decode("utf-16-le")
    if "Get-FileHash" not in script:
        return None
    found = re.search(r"FromBase64String\('([A-Za-z0-9+/=]*)'\)", script)
    items = json.loads(base64.b64decode(found[1])) if found else None
    return items if isinstance(items, list) else None


def _exchange_check(state: dict, command: str) -> tuple[int, str]:
    lines = []
    for item in _exchange_items(command):
        area, host = host_of(state, item["path"])
        if area == "shared" and host is not None and host.is_file() and not host.is_symlink():
            digest = _sha(host.read_bytes())
            lines.append(f"sha256 {item['label']} {digest}")
            if state.get("answer_twice"):
                lines.append(f"sha256 {item['label']} {'0' * 64}")
        else:
            lines.append(f"missing {item['label']}")
    return 0, "\r\n".join(lines) + "\r\n"


def _net_use(state: dict) -> tuple[int, str]:
    rows = "".join(f"OK           {letter}        \\\\Mac\\invented-share-{letter[0]}    Parallels Shared Folders\r\n"
                   for letter in _letters(state))
    return 0, ("New connections will be remembered.\r\n\r\nStatus       Local     Remote                    Network\r\n"
               + "-" * 40 + "\r\n" + rows + "The command completed successfully.\r\n")


def _quoted(text: str) -> list[str]:
    return re.findall(r'"([^"]*)"', text)


def _copy(state: dict, segment: str, violations: list[str]) -> int:
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
        data = item.read_bytes()
        if area == "shared":
            try:
                with open(destination, "xb") as handle:  # never onto anything that exists on a shared drive
                    handle.write(data)
            except FileExistsError:
                violations.append(f"copy onto an existing file on a shared drive: {target}\\{item.name}")
                return 1
        else:
            destination.write_bytes(data)
    return 0


def _shell(state: dict, command: str, violations: list[str]) -> int:
    """A cmd chain of `(if not exist .. mkdir ..)`, `del /Q ..` and `copy /Y ..`, joined by ` & `: each runs; the
    chain's exit status is the last one's."""
    code = 0
    for segment in command.split(" & "):
        segment = segment.strip()
        if segment.startswith("(if not exist"):
            path = _quoted(segment)[0]
            area, host = host_of(state, path)
            if area == "shared":
                violations.append(f"mkdir on a shared drive: {path}")
                code = 1
                continue
            if host is not None:
                host.mkdir(parents=True, exist_ok=True)
            code = 0 if host is not None else 1
        elif segment.startswith("del /Q"):
            code = 0
            for path in _quoted(segment):
                area, host = host_of(state, path)
                if area == "shared":
                    violations.append(f"del on a shared drive: {path}")
                    code = 1
                    continue
                if host is not None:
                    for item in (host.parent.glob(host.name) if "*" in host.name else [host]):
                        if item.is_file():
                            item.unlink()
        elif segment.startswith("copy /Y"):
            code = _copy(state, segment, violations)
        else:
            violations.append(f"unexpected command segment: {segment}")
            code = 1
    return code


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
    (work / "model.res").write_bytes(b"INVENTED RESULTS " + sha.encode("ascii") + b"\r\n")
    (work / "model.mod").write_bytes(b"INVENTED SOLVED MODEL " + sha.encode("ascii") + b"\r\n")
    report: dict = {"mbf_sha256": (behaviour.get("report_sha") or sha).upper(), "owned_pids": [TOOL_PID]}
    if "PidFile" in args:
        _, pid_file = host_of(state, args["PidFile"])
        pid_file.write_text(f"{TOOL_PID}\r\n", encoding="ascii")
    if mode == "batch":
        (work / "model.csv").write_text(behaviour["csv"], encoding="utf-8")
        report.update(status="csv_from_batch", csv_path=f"{guest_work}\\model.csv", gui={"used": False})
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


def _registry() -> str:
    return "\r\n".join(["", INVENTED_KEY + "\\Options", "    Use something    REG_BINARY    01", "",
                        INVENTED_KEY + "\\Recent list", "    1    REG_BINARY    00", "    N    REG_BINARY    01000000",
                        ""])


def _act(state: dict, kind: str, number: int) -> None:
    for action in state.get("actions", []):
        if action["before"] != kind or action.get("nth", 1) != number:
            continue
        path = Path(action["path"])
        if action["op"] == "write":  # the actor creates or rewrites a file as it likes
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_bytes(action["data"].encode("utf-8"))
        elif action["op"] == "mkdir":
            path.mkdir(parents=True, exist_ok=True)
        elif action["op"] == "link":  # the actor gives a stored file a second name
            path.parent.mkdir(parents=True, exist_ok=True)
            os.link(action["source"], path)


def answer(state: dict, command: str, number_of: dict[str, int]) -> tuple[str, int, str, list[str]]:
    kind = classify(command)
    number = number_of.get(kind, 0) + 1
    _act(state, kind, number)
    violations: list[str] = []
    stdout = ""
    if kind == "net_use":
        code, stdout = _net_use(state)
    elif kind == "exchange_check":
        code, stdout = _exchange_check(state, command)
    elif kind == "ver":
        code, stdout = 0, "\r\nInvented Guest [Version 0.0]\r\n"
    elif kind == "tasklist":
        code, stdout = 0, '"System","4","Services","0","100 K"\r\n"explorer.exe","1200","Console","1","50,000 K"\r\n'
    elif kind == "reg_query" and re.fullmatch(r'reg query "[^"]+" /s', command):
        code, stdout = 0, _registry()
    elif kind == "type":
        _, host = host_of(state, _quoted(command)[0])
        code, stdout = (0, host.read_text(encoding="ascii")) if host is not None and host.is_file() else (1, "")
    elif kind == "solve":
        code, stdout = _solve(state, command)
    elif kind == "export":
        code, stdout = _export(state, command)
    elif kind in ("prepare", "export_prepare", "stage_script", "collect", "export_collect"):
        code = _shell(state, command, violations)
    else:
        violations.append(f"a command the invented guest does not expect: {command[:120]}")
        code = 1
    return kind, code, stdout, violations


def main(argv: list[str]) -> int:
    state = json.loads(Path(os.environ[STATE_ENV]).read_text(encoding="utf-8"))
    log = read_log(state)
    entry: dict = {"argv": argv[:5]}
    if len(argv) != 6 or argv[0] != "exec" or argv[2:5] != ["--current-user", "cmd", "/c"]:
        entry.update(kind="not_exec", exit=2, violations=["not one `exec <vm> --current-user cmd /c <command>`"])
        code, stdout = 2, ""
    else:
        number_of: dict[str, int] = {}
        for item in log:
            number_of[item["kind"]] = number_of.get(item["kind"], 0) + 1
        kind, code, stdout, violations = answer(state, argv[5], number_of)
        entry.update(kind=kind, command=argv[5], exit=code, violations=violations)
    with open(state["log"], "a", encoding="utf-8") as handle:
        handle.write(json.dumps(entry) + "\n")
    sys.stdout.write(stdout)
    return code


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
