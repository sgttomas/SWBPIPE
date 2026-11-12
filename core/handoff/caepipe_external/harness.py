"""Live CAEPIPE round-trip harness for the correlation pilot (DEL-17-05).

The harness is disabled unless the user configures it: `SWBPIPE_CAEPIPE_EXCHANGE`
names a private exchange folder outside every Git work tree, holding a
`.gitignore` of `*` and a `harness_config.json` with the virtual machine name,
the guest paths of the user's own CAEPIPE executable, of the shared exchange
folder and of a guest-local work folder, and
`license_and_environment_responsibility_acknowledged: true`. Without that, every
live step skips with a stated reason (PH-004).

The harness never searches for, bundles or installs CAEPIPE, never stops a
process on the host other than the `prlctl` client it started itself (on its
own timeout), and runs one case at a time. Its export step writes a stored solved
model back to MBF (`exported.mbf`) for the DEL-17-04 import regression. Everything
it collects is CAEPIPE output or derived from it and stays private.

What the run lock does not show: it shows only that the harness's host process holding it has ended, and nothing
about the guest. A program instance or a guest script in the guest cannot hold a host lock, so a live step accounts
for the guest by its own named checks: it is refused while a guest script of an ended run still runs
(`guest_script_running`), while an instance of the program it did not start runs (`program_instance_running`; a
process is the step's own only by id and start time together), and while the newest holder's run wrote the saved
settings' before-state and has no record of their restore (`settings_not_restored`), until a clearance for that run
is recorded by the explicit command `tools/caepipe_correlation/clear_settings_stop.py`, which the harness never runs.

A live step or a screen changes nothing already stored and removes nothing: every file it writes is new, created so
that an existing name is never replaced (`exclusive.Tree`). A live step puts what it collects in a new run folder of
its own (`cases/<id>/runs/<NNNNNN>/`, reserved with `mkdir`), a screen its records in a new screen folder; outside
those a step creates only a selection, a series record, the guest script named by its content, the run lock's file
where it is absent (permanent and empty, never written, replaced or removed) and a holder record of that lock
(`_harness/holders/<NNNNNN>.json`), and container directories for these. Its probe marker and its run folder's
records stay. The run is held by `flock` on the lock file for as long as the step's process lives (`_take_run_lock`).
Which run of a case is current is named by a selection file (`cases/<id>/selections/<NNNNNN>.json`), itself new, which
always states its source: a run folder of the case, or `STORED_SOURCE` (the results already stored in the case's own
folders); a case with no selection is read there too (`current_case`).

The exchange root is established by a marker with random content chosen for the step, created exclusively in the
step's run folder: the guest's `net use` listing is read, and the configured drive letter and every other listed
letter are each asked for that one hash at that one path (the configured root with only its letter replaced) and
nothing else. Exactly one clean match is used for that step only and recorded with the configured letter; none, more
than one, or any conflicting, unanswered, unreadable or differing candidate is refused by name. Only then are the
step's inputs checked under the selected root. The configuration is never written.

A run, export or screen becomes current only through `publish`, after the folder it names is checked complete and
valid against the current selection (`run_problem`: its record names this case and this folder, the run ended in an
outcome eligible for selection, it is not sealed, its verified files hash as recorded, and an export or screen read
the selection's source); a refused, failed, incomplete or sealed run is never selected. The selection is written
whole as a new file of the folder it publishes and then given its numbered name with an exclusive hard link, so a
crash leaves no selection or a complete one. A reader stops with a named refusal (`SelectionError`) when a case's
selections cannot be trusted, or its source is sealed, and never falls back to an older selection or to the case's
own folders. From the moment its run folder is reserved, a step that ends exceptionally leaves a complete run record
(under another new name when the first is occupied or partial).

The guest's work, and why none of it can land in the exchange folder: each live step works in a fresh directory of
its own under the configured work root, which the guest creates exclusively (`_fresh_work_dir`); before any guest
command copies anything or starts the program, the step establishes that the directory's final path, as the guest's
file system resolves it, lies outside the exchange root's, that a probe made in it is visible nowhere under the
exchange folder, and that it holds nothing but that probe, so every destination the step directs the program to
write is absent and lies in it. Nothing in the work directory is deleted or reused. Every guest copy is a create-new
copy (`COLLECT_SCRIPT`: CopyFile with fail-if-exists): into the fresh directory, and into the run folder's new
`caepipe_out/`, where a name already there (a retry's own earlier copy included) is reported, never written. The
guest script deletes nothing and never stops a process that shows a window (the step ends as `window_shown`). The
guest only reads the exchange folder otherwise (the exchange checks, the copies of the step's inputs).

Limits: the guest is taken to answer truthfully and its drive mappings to stay put during the step; a live
synchronised replica of the exchange folder could show the marker (results stay bound by hash on the host); only
the letter is replaced, so the folder is not searched for at another relative path. The property is not claimed
against another actor writing into the exchange folder while a step runs. The lock covers one host; a holder record
naming another host refuses the step. Append-only storage does not enforce the sealed-capture reading restrictions,
which remain their own control (a capture-only step, `capture_only=True`, marks its run record sealed; every reader
reads that flag before any content and leaves those outputs unread; the capture path itself is a later change). And
append-only storage says nothing about whether a result is right, only that nothing stored was changed.
Its records are regression and handoff evidence only.

An optional `settings_guard` in the configuration names the guest registry key under
which the user's tool keeps its saved settings. When it is set, no round trip starts
while any instance of the tool is running in the guest; every round trip reads the
whole key before the guest work (no reading, no run) and again afterwards, after
confirming from the guest's process list that every process the guest script
started has ended and that no other instance appeared. Any difference, and any
uncertain process state, is recorded in the run record for the caller to stop on.
Only when the configuration enables restoring are differences written back: with
`restore_recent_files` only a difference lying wholly in the recent-files subkey's values,
with `restore_scope` `"key"` any difference strictly inside the configured key (values,
and subkeys created or deleted, never the key itself or anything outside it). The key is
then read a third time and must equal the first reading; nothing else is ever written.
The restore is safe only because the precondition admits no other instance of the
tool that could be changing its settings at the same time.
"""

from __future__ import annotations

import dataclasses
from dataclasses import dataclass, field
from datetime import datetime, timezone
import base64
import contextlib
import csv
import hashlib
import io
import json
import math
import os
from pathlib import Path
import re
import shutil
import secrets
import signal
import socket
import stat
import subprocess
import threading
import time
from types import SimpleNamespace
from typing import Any, Callable, Mapping

from .exclusive import NAME_PATTERN, ContainmentError, ExistingPath, Tree, sha256_bytes  # noqa: F401 - re-exported

try:  # the run lock is an advisory lock (`flock`); where the platform has none, a live step is refused by name
    import fcntl
except ImportError:  # pragma: no cover - hosted Windows jobs import this module
    fcntl = None

# Reads its directory scans stand for, for CI selection (tools/ci/python_test_reads.py). It lists only the
# CAEPIPE exchange folder (its run folders' caepipe_out, and the whole folder when it looks for the work-folder
# probe), outside the project.
CI_READS = ()


ENV_VAR = "SWBPIPE_CAEPIPE_EXCHANGE"
CONFIG_NAME = "harness_config.json"
ACKNOWLEDGEMENT_KEY = "license_and_environment_responsibility_acknowledged"
REQUIRED_KEYS = ("vm_name", "guest_executable", "guest_exchange_root", "guest_work_root")
CASE_ID_PATTERN = re.compile(r"^[a-z0-9][a-z0-9-]{0,62}\Z")
GUEST_SCRIPT = Path(__file__).resolve().parent / "guest" / "Invoke-CaepipeRoundTrip.ps1"
GUEST_SCRIPT_REPOSITORY_PATH = "core/handoff/caepipe_external/guest/Invoke-CaepipeRoundTrip.ps1"
HARNESS_DIR = "_harness"
RUN_RECORD_KIND = "openpipestress.caepipe_harness_run_record"
BATCH_COMMAND_SHAPE = '<exe> "<mbf>";batchmode=yes'
BOUNDARY_NOTE = "regression/handoff evidence only; not validation, compatibility proof or professional acceptance"
CASE_LAYERS = ("swbpipe", "caepipe_in", "caepipe_out", "parsed", "comparison")
OUTPUT_NAMES = ("model.res", "model.mod")
# The batch print written with a .txt extension (the tool's saved print-file extension); accepted only when its
# content parses as the documented print-CSV layout (`_print_layout_problem`).
PRINT_TXT_NAME = "model.txt"
PRLCTL_ATTEMPTS = 3
CSV_ROUTES = ("csv_from_batch", "csv_from_gui_print")
EXPORTED_MBF = "exported.mbf"
EXPORT_OUTPUTS = (EXPORTED_MBF, "export_guest_stdout.txt", "export_guest_stderr.txt", "export_guest_log.json")
EXPORT_ROUTE = "mbf_exported"
# Guest paths enter `cmd /c` command strings, so they are held to a strict allow-list: an absolute Windows path
# (drive letter, colon, backslash) of letters, digits, space and \ . - _ ( ) only. Quotes, %, ^, &, |, <, >, !
# and control characters can never reach the guest shell.
GUEST_PATH_KEYS = ("guest_executable", "guest_exchange_root", "guest_work_root")
GUEST_PATH_PATTERN = re.compile(r"^[A-Za-z]:\\[A-Za-z0-9 \\._()-]*\Z")
SETTINGS_KEY_PATTERN = re.compile(r"^HKCU(\\[A-Za-z0-9 ._()-]+)+\Z")
SETTINGS_SUBKEY_PATTERN = re.compile(r"^[A-Za-z0-9 ._()-]+\Z")
# A value line of `reg query`: four spaces, name, four spaces, type, then four spaces and the data (absent when empty).
REG_VALUE_LINE = re.compile(r"^ {4}(?P<name>.+?) {4}(?P<type>REG_[A-Z_]+)(?: {4}(?P<data>.*))?\Z")
SETTINGS_KEEP_GOING = ("unchanged", "recent_files_restored", "settings_restored")
# What a restore may cover: the recent-files subkey's values, or everything strictly inside the configured key.
RESTORE_SCOPES = ("recent_files", "key")
# Names a restore writes into a guest command: printed keys and value names within this character set only.
REG_PRINTED_KEY_PATTERN = re.compile(r"^HKEY_CURRENT_USER(\\[A-Za-z0-9 ._()-]+)+\Z")
REG_VALUE_NAME_PATTERN = re.compile(r"^[A-Za-z0-9 ._()-]+\Z")
# Data a restore can write back exactly through `reg add` (REG_SZ only within the guest-command allow-list).
RESTORABLE_DATA = {"REG_BINARY": re.compile(r"^[0-9A-Fa-f]*\Z"), "REG_DWORD": re.compile(r"^0x[0-9A-Fa-f]{1,8}\Z"),
                   "REG_SZ": re.compile(r"^[A-Za-z0-9 \\._()*:-]*\Z")}
# The one value name a restore may address outside REG_VALUE_NAME_PATTERN, and only in the recent-files subkey: the
# folder of the configured executable with one trailing backslash, a value the tool writes there (observed format
# behaviour). `reg` cannot carry a quoted argument that ends in a backslash, so this value alone is written
# and read back through the registry API (LITERAL_VALUE_SCRIPT), its key, name and data passed as encoded data.
LITERAL_VALUE_FOLDER = re.compile(r"^[A-Za-z]:(\\[A-Za-z0-9 ._()-]+)+\Z")
# cmd.exe takes a command line of at most 8191 characters; `cmd /c ` and a margin come off it.
LITERAL_COMMAND_LIMIT = 8000
LITERAL_VALUE_SCRIPT = r"""$ErrorActionPreference = 'Stop'; $ProgressPreference = 'SilentlyContinue'
$missing = 'ERROR: The system was unable to find the specified registry key or value.'
try {
  $p = [Text.Encoding]::UTF8.GetString([Convert]::FromBase64String('@PAYLOAD@')) | ConvertFrom-Json
  $key = [Microsoft.Win32.Registry]::CurrentUser.OpenSubKey($p.key, ($p.action -ne 'read'))
  if ($null -eq $key) { [Console]::Error.Write($missing); exit 1 }
  try {
    $present = $key.GetValueNames() -ccontains $p.name
    if ($p.action -eq 'read') {
      if (-not $present) { [Console]::Error.Write($missing); exit 1 }
      if ($key.GetValueKind($p.name) -ne [Microsoft.Win32.RegistryValueKind]::Binary) { exit 2 }
      $hex = ([BitConverter]::ToString([byte[]]$key.GetValue($p.name))) -replace '-', ''
      [Console]::Out.Write("`r`nHKEY_CURRENT_USER\" + $p.key + "`r`n    " + $p.name + "    REG_BINARY    " + $hex + "`r`n")
    } elseif ($p.action -eq 'set') {
      $bytes = New-Object byte[] ($p.data.Length / 2)
      for ($i = 0; $i -lt $bytes.Length; $i++) { $bytes[$i] = [Convert]::ToByte($p.data.Substring(2 * $i, 2), 16) }
      $key.SetValue($p.name, $bytes, [Microsoft.Win32.RegistryValueKind]::Binary)
      [Console]::Out.Write('The operation completed successfully.')
    } elseif ($p.action -eq 'delete') {
      if (-not $present) { [Console]::Error.Write($missing); exit 1 }
      $key.DeleteValue($p.name, $true)
      [Console]::Out.Write('The operation completed successfully.')
    } else { exit 2 }
  } finally { $key.Close() }
} catch { [Console]::Error.Write($_.Exception.Message); exit 1 }
exit 0
"""
# Written by the guest script in the case's work folder: one id per process it starts (guarded runs only).
OWNED_PIDS_FILE = "owned_pids.txt"
# The guard's result, with its write log, persisted on its own before the run record (guarded runs only).
GUARD_RESULT_FILE = "settings_guard.json"
# Guard call limits (configurable in settings_guard): each guard call's timeout, and the total time the restore's
# registry-mutation section may take. The solve keeps its own timeout.
GUARD_CALL_TIMEOUT_S = 20.0
GUARD_SECTION_BUDGET_S = 120.0
# Guard reads (process lists, registry readings, read-backs) have no side effect, so a transport failure of the
# prlctl client is retried more patiently than other steps: up to this many attempts, this far apart, all within
# the call's timeout (and the section budget inside the restore). Writes keep their single settled retry.
GUARD_READ_ATTEMPTS = 6
GUARD_READ_BACKOFF_S = 2.0
# Named refusals of a live step, each recorded in its own new run folder with nothing stored changed.
EXCHANGE_UNAVAILABLE_ROUTE = "exchange_drive_unavailable"
EXCHANGE_AMBIGUOUS_ROUTE = "exchange_drive_ambiguous"
EXCHANGE_CHECK_FAILED_ROUTE = "exchange_check_failed"
WORK_FOLDER_OVERLAP_ROUTE = "work_folder_on_exchange_drive"
SELECTION_UNTRUSTED_ROUTE = "selection_untrusted"
RUN_LOCK_HELD_ROUTE = "run_lock_held"
RUN_LOCK_UNAVAILABLE_ROUTE = "run_lock_unavailable"
# The three stops the lock cannot settle (it shows only that a host process ended, nothing about the guest), each
# under its own name, and the stop when the guest's process listing cannot be read.
GUEST_SCRIPT_RUNNING_ROUTE = "guest_script_running"
PROGRAM_INSTANCE_ROUTE = "program_instance_running"
SETTINGS_NOT_RESTORED_ROUTE = "settings_not_restored"
GUEST_PROCESSES_UNKNOWN_ROUTE = "guest_processes_unknown"
# The two routes that operate the program's own windows (the print fallback through its window, and the export of a
# model file from it) are refused by name before anything is started: they wait for the listed-window rule
# (DEL-17-05 REQ-023, not yet placed). The invocation profile in force is the documented batch pattern alone.
WINDOW_ROUTE_UNAVAILABLE = "window_route_unavailable"
WINDOW_ROUTE_REASON = ("operating the program's windows waits for the listed-window rule (DEL-17-05 REQ-023, not yet "
                       "placed); the harness in force runs the documented batch pattern alone, so this step is refused "
                       "before anything is started")
REFUSED_ROUTES = (EXCHANGE_UNAVAILABLE_ROUTE, EXCHANGE_AMBIGUOUS_ROUTE, EXCHANGE_CHECK_FAILED_ROUTE,
                  WORK_FOLDER_OVERLAP_ROUTE, SELECTION_UNTRUSTED_ROUTE, RUN_LOCK_HELD_ROUTE, RUN_LOCK_UNAVAILABLE_ROUTE,
                  GUEST_SCRIPT_RUNNING_ROUTE, PROGRAM_INSTANCE_ROUTE, SETTINGS_NOT_RESTORED_ROUTE,
                  GUEST_PROCESSES_UNKNOWN_ROUTE, WINDOW_ROUTE_UNAVAILABLE)
# A collection that could not be shown complete (a copy failed, or a name was already there with other content):
# recorded, never published.
COLLECTION_FAILED_ROUTE = "collection_failed"
# A live step's run folder, a screen's folder and the case's selections, each below cases/<id>/.
RUNS_DIR = "runs"
SCREENS_DIR = "screens"
SELECTIONS_DIR = "selections"
SELECTION_KIND = "openpipestress.caepipe_case_selection"
SCREEN_RECORD_KIND = "openpipestress.caepipe_screen_record"
SEQUENCE_PATTERN = re.compile(r"^[0-9]{6}\Z")
# The run record of a solve or export run, a new file in the run folder (never rewritten).
RUN_RECORD = ("caepipe_out", "run_record.json")
# Created exclusively in the step's new run folder just before the check, with random content chosen for that step:
# only a guest root that is this exchange folder can report its hash. It stays in the run folder (a step removes
# nothing).
EXCHANGE_MARKER = "exchange_marker.txt"
# What a selection's `run` states when the case's results are the ones already stored in its own folders (the legacy
# location), as opposed to a run folder of the case. A selection always states its source; null is refused.
STORED_SOURCE = "already_stored"
# The harness's copy of its guest script, named by the SHA-256 of its content: created once, reused only after its
# content is verified, never rewritten.
GUEST_SCRIPT_NAME = "guest-script-{sha256}.ps1"
# The run lock: a permanent, empty file, created exclusively when absent and never removed; a step holds the run by an
# advisory lock (`flock`) on it for as long as its process lives. Each holder writes a new, numbered holder record.
LOCK_NAME = "round_trip.lock"
HOLDERS_DIR = "holders"
HOLDER_STAGED = "run_lock_holder.json"
GUEST_SCRIPT_STAGED = "guest_script.ps1"
# Each live step's fresh work directory, directly in the configured work root (which must exist; no other folder is
# made): `<work root>\\<case>_<kind>-<run>-<token>`, created exclusively by the guest; two steps never share one. The
# content probe inside it is one check among several.
WORK_DIR_NAME = "{case}_{kind}-{sequence}-{token}"
WORK_PROBE_NAME = "swbpipe-work-probe-{token}.txt"
# The settings records of a guarded live step, new files in its run folder: the stage record, which is the saved
# settings' before-state (every member by name, kind and the hash of its value; the value only for members the guard
# may write back), written before the first guest command that can start the program or change a setting; the check
# record, written once the settings check has finished; and a clearance, written only by the explicit command
# `tools/caepipe_correlation/clear_settings_stop.py`, never by the harness itself. Nothing in them, no value and no
# hash, is copied into a report.
SETTINGS_STAGE = "settings_before_state.json"
SETTINGS_CHECK = "settings_check.json"
SETTINGS_CLEARANCE = "settings_clearance.json"
# The holder's end record, a new file in its run folder written when the step ends in its own process (an exception
# included); a newest holder whose run lacks it ended unfinished (its process died).
RUN_LOCK_END = "run_lock_end.json"
# The one bounded wait for a holder that is still exiting, then the named refusal.
RUN_LOCK_EXIT_WAIT_S = 2.0
RUN_LOCK_CALL = "fcntl.flock(fd, LOCK_EX | LOCK_NB) on an open file description of the lock file (never lockf/F_SETLK)"
EXCHANGE_CHECK_SCRIPT = r"""$ErrorActionPreference = 'Stop'; $ProgressPreference = 'SilentlyContinue'
$items = [Text.Encoding]::UTF8.GetString([Convert]::FromBase64String('@PAYLOAD@')) | ConvertFrom-Json
foreach ($item in $items) {
  try {
    if (Test-Path -LiteralPath $item.path -PathType Leaf) {
      $hash = (Get-FileHash -LiteralPath $item.path -Algorithm SHA256).Hash.ToLowerInvariant()
      [Console]::Out.Write('sha256 ' + $item.label + ' ' + $hash + "`r`n")
    } else { [Console]::Out.Write('missing ' + $item.label + "`r`n") }
  } catch { [Console]::Out.Write('unreadable ' + $item.label + "`r`n") }
}
exit 0
"""
# Every process in the guest whose image is the configured executable, or whose command line runs the harness's guest
# script (content-named on the share, or its work-folder copy): one line each, then `listed`. Read only.
GUEST_PROCESSES_SCRIPT = r"""$ErrorActionPreference = 'Stop'; $ProgressPreference = 'SilentlyContinue'
$p = [Text.Encoding]::UTF8.GetString([Convert]::FromBase64String('@PAYLOAD@')) | ConvertFrom-Json
foreach ($proc in @(Get-CimInstance Win32_Process)) {
  if ($proc.ProcessId -eq $PID) { continue }
  $start = if ($proc.CreationDate) { $proc.CreationDate.ToUniversalTime().ToString('o') } else { 'unknown' }
  if ($proc.Name -ieq $p.image) { [Console]::Out.Write('tool ' + $proc.ProcessId + ' ' + $start + "`r`n") }
  elseif ($proc.CommandLine -and ($proc.CommandLine -match 'guest-script-[0-9A-Fa-f]{64}\.ps1' -or
          $proc.CommandLine -match [regex]::Escape($p.script))) { [Console]::Out.Write('script ' + $proc.ProcessId + ' ' + $start + "`r`n") }
}
[Console]::Out.Write("listed`r`n")
exit 0
"""
# The final path of a guest path as the guest's file system resolves it (reparse points and links followed:
# GetFinalPathNameByHandle), or "" when it cannot be opened.
FINAL_PATH_TYPE = r"""Add-Type -TypeDefinition @'
using System; using System.Runtime.InteropServices; using System.Text; using Microsoft.Win32.SafeHandles;
public static class SwbFinalPath {
  [DllImport("kernel32.dll", CharSet = CharSet.Unicode, SetLastError = true)]
  static extern SafeFileHandle CreateFileW(string n, uint a, uint s, IntPtr sa, uint c, uint f, IntPtr t);
  [DllImport("kernel32.dll", CharSet = CharSet.Unicode, SetLastError = true)]
  static extern uint GetFinalPathNameByHandleW(SafeFileHandle h, StringBuilder b, uint n, uint f);
  public static string Of(string path) {
    using (var h = CreateFileW(path, 0, 7, IntPtr.Zero, 3, 0x02000000, IntPtr.Zero)) {
      if (h.IsInvalid) { return ""; }
      var b = new StringBuilder(32768);
      return GetFinalPathNameByHandleW(h, b, 32768, 0) == 0 ? "" : b.ToString();
    }
  }
}
'@
function Say([string]$m) { [Console]::Out.Write($m + "`r`n") }
"""
# Before anything is created for the step: the final paths of the exchange root and of the configured work root, the
# folder the step's fresh work directory is to be created in, when that folder exists. Creates, deletes and copies
# nothing.
WORK_ROOT_SCRIPT = r"""$ErrorActionPreference = 'Stop'; $ProgressPreference = 'SilentlyContinue'
$p = [Text.Encoding]::UTF8.GetString([Convert]::FromBase64String('@PAYLOAD@')) | ConvertFrom-Json
""" + FINAL_PATH_TYPE + r"""Say ('root ' + [SwbFinalPath]::Of($p.root))
if (Test-Path -LiteralPath $p.path -PathType Container) { Say ('ancestor ' + $p.path); Say ('final ' + [SwbFinalPath]::Of($p.path)) }
Say 'root resolved'
exit 0
"""
# The step's fresh work directory, made only after WORK_ROOT_SCRIPT showed the configured work root existing and
# outside the exchange folder: the one directory is created in it with New-Item without -Force, which fails when the
# name exists (and when the work root is absent: no other folder is made); then the final paths of the exchange root
# and of the new directory are reported, the content probe is created in it (FileMode.CreateNew), and every entry it
# holds and every destination the step will direct the program to write are listed. Nothing is deleted or copied.
WORK_DIR_SCRIPT = r"""$ErrorActionPreference = 'Stop'; $ProgressPreference = 'SilentlyContinue'
$p = [Text.Encoding]::UTF8.GetString([Convert]::FromBase64String('@PAYLOAD@')) | ConvertFrom-Json
""" + FINAL_PATH_TYPE + r"""Say ('root ' + [SwbFinalPath]::Of($p.root))
New-Item -ItemType Directory -Path $p.dir | Out-Null
Say ('dir ' + [SwbFinalPath]::Of($p.dir))
$stream = [IO.File]::Open((Join-Path $p.dir $p.probe), [IO.FileMode]::CreateNew, [IO.FileAccess]::Write)
$stream.Close()
foreach ($e in @(Get-ChildItem -LiteralPath $p.dir -Force)) { Say ('entry ' + $e.Name) }
foreach ($n in @($p.destinations)) {
  if (Test-Path -LiteralPath (Join-Path $p.dir $n)) { Say ('destination_present ' + $n) } else { Say ('destination_absent ' + $n) }
}
Say 'work ready'
exit 0
"""
# Every guest copy of the harness (the model into the fresh work directory, the script copy beside it, and the
# collection into the step's new run folder): each file is copied with [IO.File]::Copy(source, destination, $false),
# which is CopyFile with fail-if-exists, a create-new of the destination at the file system (the shared folder
# included): a name already there (a file, or a link to a stored file) is never written. When the destination is
# there (a retry after a transport failure finds its own earlier copy), both are hashed and the outcome reported;
# nothing is overwritten. One line per file, then `collect done`. `purpose` names the copy in the record.
COLLECT_SCRIPT = r"""$ErrorActionPreference = 'Stop'; $ProgressPreference = 'SilentlyContinue'
$p = [Text.Encoding]::UTF8.GetString([Convert]::FromBase64String('@PAYLOAD@')) | ConvertFrom-Json
$names = @($p.names)
foreach ($pattern in @($p.patterns)) {
  foreach ($f in @(Get-ChildItem -LiteralPath $p.from -Filter $pattern -File -ErrorAction SilentlyContinue)) { $names += $f.Name }
}
foreach ($n in @($names | Select-Object -Unique)) {
  $src = Join-Path $p.from $n; $dst = Join-Path $p.to $n
  if (-not (Test-Path -LiteralPath $src -PathType Leaf)) { [Console]::Out.Write('absent ' + $n + "`r`n"); continue }
  try { [IO.File]::Copy($src, $dst, $false); [Console]::Out.Write('copied ' + $n + "`r`n") }
  catch {
    if (Test-Path -LiteralPath $dst) {
      try {
        $a = (Get-FileHash -LiteralPath $src -Algorithm SHA256).Hash; $b = (Get-FileHash -LiteralPath $dst -Algorithm SHA256).Hash
        if ($a -eq $b) { [Console]::Out.Write('present_same ' + $n + "`r`n") } else { [Console]::Out.Write('present_differs ' + $n + "`r`n") }
      } catch { [Console]::Out.Write('present_unreadable ' + $n + "`r`n") }
    } else { [Console]::Out.Write('failed ' + $n + "`r`n") }
  }
}
[Console]::Out.Write("collect done`r`n")
exit 0
"""


@dataclass(frozen=True)
class HarnessSkip:
    """The harness is not configured; `reason` says why, `checked` what was checked before stopping."""

    reason: str
    checked: tuple[str, ...] = ()


@dataclass(frozen=True)
class HarnessConfig:
    exchange_root: Path
    vm_name: str
    guest_executable: str
    guest_exchange_root: str
    guest_work_root: str
    prlctl: str | None
    acknowledged: bool
    trial_first_launch_date: str | None
    checked: tuple[str, ...] = ()
    warnings: tuple[str, ...] = field(default_factory=tuple)
    settings_guard: "SettingsGuard | None" = None


@dataclass(frozen=True)
class SettingsGuard:
    """The guest registry key of the tool's saved settings, read whole, and its recent-files subkey.

    `restore_recent_files` (off unless configured) lets a round trip write back the recent-files subkey alone when
    nothing else changed; `restore_scope` `"key"` lets it write back any difference strictly inside `key` (and
    enables restoring by itself). Either is safe only because the precondition admits no instance of the tool but
    our own.
    """

    key: str
    recent_files_subkey: str | None = None
    restore_recent_files: bool = False
    call_timeout_s: float = GUARD_CALL_TIMEOUT_S
    section_budget_s: float = GUARD_SECTION_BUDGET_S
    restore_scope: str = "recent_files"

    @property
    def recent_files_key(self) -> str | None:
        return f"{self.key}\\{self.recent_files_subkey}" if self.recent_files_subkey else None

    @property
    def restores_whole_key(self) -> bool:
        return self.restore_scope == "key"


def resolve_harness_config(environ: Mapping[str, str] = os.environ, *,
                           require_vm_tools: bool = True) -> HarnessConfig | HarnessSkip:
    """Resolve the private harness configuration, or say exactly why the harness is skipped.

    `require_vm_tools=False` (a screen: the offline re-comparison of stored results, the benchmark's offline screen
    and the series step) needs nothing of the guest. It keeps the data-boundary checks for reading and adding to the
    exchange folder: `SWBPIPE_CAEPIPE_EXCHANGE` names an existing directory, which holds a `.gitignore` of `*` and lies
    outside every Git work tree. It does not need `harness_config.json`, the acknowledgement, the virtual machine
    name, the guest paths, the settings guard or `prlctl`; when the configuration file is there, only its
    `trial_first_launch_date` is read (for the case manifest), and nothing in it can stop a screen.
    """

    checked: list[str] = []

    def skip(reason: str) -> HarnessSkip:
        return HarnessSkip(reason=reason, checked=tuple(checked))

    raw = environ.get(ENV_VAR, "")
    checked.append(f"{ENV_VAR} set")
    if not raw.strip():
        return skip(f"{ENV_VAR} is unset or empty; the CAEPIPE harness is disabled")
    root = Path(raw).expanduser()
    checked.append("exchange folder exists")
    if not root.exists():
        return skip(f"{ENV_VAR} names a folder that does not exist")
    checked.append("exchange folder is a directory")
    if not root.is_dir():
        return skip(f"{ENV_VAR} does not name a directory")
    root = root.resolve()
    checked.append(".gitignore of '*'")
    ignore = root / ".gitignore"
    try:
        content = ignore.read_text(encoding="utf-8") if ignore.is_file() else None
    except (OSError, UnicodeDecodeError):
        content = None
    if content not in ("*", "*\n", "*\r\n"):
        return skip("the exchange folder lacks a .gitignore whose content is exactly '*'")
    checked.append("outside every Git work tree")
    inside = _inside_git(root, environ)
    if inside is None:
        return skip("git is not available to confirm the exchange folder is outside every Git work tree")
    if inside:
        return skip("the exchange folder lies inside a Git work tree or Git directory; CAEPIPE outputs must stay "
                    "outside every repository")
    if not require_vm_tools:
        return _offline_config(root, checked)
    checked.append(f"{CONFIG_NAME} present")
    path = root / CONFIG_NAME
    if not path.is_file():
        return skip(f"the exchange folder has no {CONFIG_NAME}")
    try:
        config = json.loads(path.read_text(encoding="utf-8"))
    except (OSError, UnicodeDecodeError, json.JSONDecodeError):
        return skip(f"{CONFIG_NAME} is not readable JSON")
    if not isinstance(config, dict):
        return skip(f"{CONFIG_NAME} is not a JSON object")
    checked.append(f"{ACKNOWLEDGEMENT_KEY} is true")
    if config.get(ACKNOWLEDGEMENT_KEY) is not True:
        return skip(f"{CONFIG_NAME} does not set {ACKNOWLEDGEMENT_KEY} to true")
    checked.append("required keys " + ", ".join(REQUIRED_KEYS))
    missing = [key for key in REQUIRED_KEYS if not isinstance(config.get(key), str) or not config[key].strip()]
    if missing:
        return skip(f"{CONFIG_NAME} lacks {', '.join(missing)}")
    checked.append("guest paths are allow-listed Windows absolute paths")
    for key in GUEST_PATH_KEYS:
        if not valid_guest_path(config[key], directory=key != "guest_executable"):
            return skip(f"{CONFIG_NAME} {key} is not an absolute Windows path (X:\\...) of letters, digits, space and "
                        "\\ . - _ ( ) only")
    guard = None
    if "settings_guard" in config:
        checked.append("settings_guard names an HKCU key")
        raw_guard = config["settings_guard"]
        key = raw_guard.get("key") if isinstance(raw_guard, dict) else None
        subkey = raw_guard.get("recent_files_subkey") if isinstance(raw_guard, dict) else None
        restore = raw_guard.get("restore_recent_files", False) if isinstance(raw_guard, dict) else None
        limits = {name: (raw_guard.get(name, default) if isinstance(raw_guard, dict) else None)
                  for name, default in (("call_timeout_s", GUARD_CALL_TIMEOUT_S),
                                        ("section_budget_s", GUARD_SECTION_BUDGET_S))}
        scope = raw_guard.get("restore_scope", "recent_files") if isinstance(raw_guard, dict) else None
        limits_valid = all(isinstance(value, (int, float)) and not isinstance(value, bool) and math.isfinite(value)
                           and value > 0 for value in limits.values())
        if not (isinstance(key, str) and SETTINGS_KEY_PATTERN.match(key)
                and (subkey is None or isinstance(subkey, str) and SETTINGS_SUBKEY_PATTERN.match(subkey))
                and isinstance(restore, bool) and (subkey is not None or not restore) and limits_valid
                and scope in RESTORE_SCOPES):
            return skip(f"{CONFIG_NAME} settings_guard must give key (HKCU\\..., letters, digits, space and "
                        ". _ ( ) - only), optionally recent_files_subkey (the same characters), "
                        "restore_recent_files (true or false; true needs recent_files_subkey), restore_scope "
                        "(\"recent_files\" or \"key\") and positive call_timeout_s and section_budget_s (seconds)")
        guard = SettingsGuard(key=key, recent_files_subkey=subkey, restore_recent_files=restore,
                              call_timeout_s=float(limits["call_timeout_s"]),
                              section_budget_s=float(limits["section_budget_s"]), restore_scope=scope)
    prlctl = None
    if require_vm_tools:
        checked.append("prlctl on PATH")
        prlctl = shutil.which("prlctl", path=environ.get("PATH"))
        if prlctl is None:
            return skip("prlctl is not on PATH; the virtual machine cannot be reached")
    warnings = []
    mode = stat.S_IMODE(root.stat().st_mode)
    if mode & 0o077:
        warnings.append(f"exchange folder mode is {mode:o}; Design §5 asks for 700")
    first_launch = config.get("trial_first_launch_date")
    return HarnessConfig(
        exchange_root=root,
        vm_name=config["vm_name"],
        guest_executable=config["guest_executable"],
        guest_exchange_root=config["guest_exchange_root"].rstrip("\\"),
        guest_work_root=config["guest_work_root"].rstrip("\\"),
        prlctl=prlctl,
        acknowledged=True,
        trial_first_launch_date=first_launch if isinstance(first_launch, str) else None,
        checked=tuple(checked),
        warnings=tuple(warnings),
        settings_guard=guard,
    )


def _offline_config(root: Path, checked: list[str]) -> HarnessConfig:
    """A screen's configuration: the exchange folder alone (its data-boundary checks already passed), with no guest
    field; `trial_first_launch_date` is taken from `harness_config.json` only when that file is there and readable."""

    first_launch = None
    path = root / CONFIG_NAME
    try:
        value = json.loads(path.read_text(encoding="utf-8")) if path.is_file() and not path.is_symlink() else None
    except (OSError, UnicodeDecodeError, json.JSONDecodeError):
        value = None
    if isinstance(value, dict) and isinstance(value.get("trial_first_launch_date"), str):
        first_launch = value["trial_first_launch_date"]
    warnings = []
    mode = stat.S_IMODE(root.stat().st_mode)
    if mode & 0o077:
        warnings.append(f"exchange folder mode is {mode:o}; Design §5 asks for 700")
    return HarnessConfig(exchange_root=root, vm_name="", guest_executable="", guest_exchange_root="",
                         guest_work_root="", prlctl=None, acknowledged=False, trial_first_launch_date=first_launch,
                         checked=tuple(checked), warnings=tuple(warnings), settings_guard=None)


def valid_guest_path(value: Any, *, directory: bool) -> bool:
    """True for an allow-listed absolute Windows path without `.`/`..` segments; a file may not end in `\\`."""

    if not isinstance(value, str) or not GUEST_PATH_PATTERN.match(value):
        return False
    stripped = value.rstrip("\\")
    if not directory and stripped != value:
        return False
    return all(part not in (".", "..") for part in stripped.split("\\")[1:])


def _inside_git(path: Path, environ: Mapping[str, str]) -> bool | None:
    env = {key: value for key, value in os.environ.items() if not key.startswith("GIT_")}
    if environ.get("PATH"):
        env["PATH"] = environ["PATH"]
    try:
        completed = subprocess.run(["git", "-C", str(path), "rev-parse", "--is-inside-work-tree"],
                                   capture_output=True, text=True, env=env, check=False, timeout=30)
    except (FileNotFoundError, subprocess.TimeoutExpired):
        return None
    # Exit 0 prints "true" inside a work tree and "false" inside a .git directory: both are refused.
    return completed.returncode == 0


def contained(root: Path, path: Path) -> Path:
    """Return `path` when no component below the resolved exchange root is a symbolic link and it stays inside.

    Applied to every directory and file the pilot creates, writes or deletes, so an existing link in the exchange
    folder can never redirect a write or a cleanup elsewhere (for example into a Git work tree).
    """

    base = Path(root).resolve()
    try:
        parts = Path(path).relative_to(base).parts
    except ValueError:
        raise ContainmentError(f"{path} is not inside the exchange folder") from None
    current = base
    for part in parts:
        if part in (".", ".."):
            raise ContainmentError(f"{path} contains a {part!r} segment")
        current = current / part
        if current.is_symlink():
            raise ContainmentError(f"{current.relative_to(base)} in the exchange folder is a symbolic link; refused")
    if current.exists() and not current.resolve().is_relative_to(base):
        raise ContainmentError(f"{current.relative_to(base)} resolves outside the exchange folder")
    return current


def contained_dir(root: Path, path: Path) -> Path:
    """`contained(root, path)`, created as a directory (parents checked one by one) when absent."""

    base = Path(root).resolve()
    current = base
    for part in contained(base, path).relative_to(base).parts:
        current = contained(base, current / part)
        if not current.exists():
            current.mkdir()
        elif not current.is_dir():
            raise ContainmentError(f"{current.relative_to(base)} in the exchange folder is not a directory")
    return current


def case_paths(config: HarnessConfig | Path, case_id: str) -> dict[str, Path]:
    """The legacy layout of one case (its own layer folders, where results were stored before run folders), created
    when absent. A fixture and layout helper only: no live step or screen calls it."""

    _check_case_id(case_id)
    root = _root_of(config)
    case_root = root / "cases" / case_id
    paths = {"root": case_root, "manifest": contained(root, case_root / "case_manifest.json")}
    contained_dir(root, case_root)
    for layer in CASE_LAYERS:
        paths[layer] = contained_dir(root, case_root / layer)
    return paths


def _check_case_id(case_id: Any) -> str:
    if not isinstance(case_id, str) or not CASE_ID_PATTERN.match(case_id):
        raise ValueError(f"case id {case_id!r} must match {CASE_ID_PATTERN.pattern}")
    return case_id


def _root_of(config: "HarnessConfig | Path") -> Path:
    return (config.exchange_root if isinstance(config, HarnessConfig) else Path(config)).resolve()


def exchange_tree(config: "HarnessConfig | Path") -> Tree:
    """Create-only access to the exchange folder (`exclusive.Tree`): the only way a live step or screen writes."""
    return Tree(_root_of(config), "exchange folder")


@dataclass
class CaseFolder:
    """A new run or screen folder of one case, `cases/<id>/<kind>/<NNNNNN>/`, reserved with `mkdir`; every file in it
    is created exclusively, and nothing in it is ever rewritten."""

    tree: Tree
    case_id: str
    kind: str
    sequence: str

    @property
    def parts(self) -> tuple[str, ...]:
        return ("cases", self.case_id, self.kind, self.sequence)

    @property
    def relative(self) -> str:
        """The folder relative to the case folder (`runs/000003`), as selections and records name it."""
        return f"{self.kind}/{self.sequence}"

    @property
    def path(self) -> Path:
        return self.tree.path(self.parts)

    @staticmethod
    def _split(relative: str) -> tuple[str, ...]:
        parts = tuple(relative.split("/"))
        if not parts or any(part in ("", ".", "..") for part in parts):
            raise ContainmentError(f"{relative!r} is not a path inside a run folder")
        return parts

    def create(self, relative: str, data: bytes | str) -> dict[str, Any]:
        """A new file at `relative` in this folder (subfolders created when absent): {bytes, sha256}."""
        parts = self._split(relative)
        if len(parts) > 1:
            self.tree.ensure_dir(self.parts + parts[:-1])
        return self.tree.create(self.parts + parts, data)

    def record(self, relative: str, value: Any) -> dict[str, Any]:
        return self.create(relative, json.dumps(value, indent=2, sort_keys=True, default=str) + "\n")

    def exists(self, relative: str) -> bool:
        return self.tree.exists(self.parts + self._split(relative))

    def read_bytes(self, relative: str) -> bytes | None:
        return self.tree.read_bytes(self.parts + self._split(relative))

    def free(self, folder: str, stem: str, suffix: str) -> str:
        """The first `<folder>/<stem><suffix>`, `<stem>_2<suffix>`, ... that does not exist yet."""
        for number in range(1, 100):
            relative = f"{folder}/{stem}{'' if number == 1 else f'_{number}'}{suffix}"
            if not self.exists(relative):
                return relative
        raise ExistingPath(f"no free name for {folder}/{stem}{suffix} in {self.relative}")


def reserve_folder(tree: Tree, case_id: str, kind: str) -> CaseFolder:
    """A new folder `cases/<id>/<kind>/<next number>/`: the number is reserved by `mkdir`, so two commands never share
    one; a number taken meanwhile is skipped."""

    _check_case_id(case_id)
    if kind not in (RUNS_DIR, SCREENS_DIR):
        raise ValueError(f"{kind!r} is not a run or screen folder kind")
    tree.ensure_dir(("cases", case_id, kind))
    for _ in range(1000):
        numbers = [int(name) for name in tree.listdir(("cases", case_id, kind)) if SEQUENCE_PATTERN.match(name)]
        sequence = f"{max(numbers, default=0) + 1:06d}"
        try:
            tree.reserve_dir(("cases", case_id, kind, sequence))
        except ExistingPath:
            continue
        return CaseFolder(tree, case_id, kind, sequence)
    raise ContainmentError(f"no new {kind} folder could be reserved for {case_id}")


def open_folder(tree: Tree, case_id: str, relative: str) -> CaseFolder:
    """An existing run or screen folder of the case (`runs/000003`), to add new files to it."""

    _check_case_id(case_id)
    kind, _, sequence = relative.partition("/")
    if kind not in (RUNS_DIR, SCREENS_DIR) or not SEQUENCE_PATTERN.match(sequence) \
            or not tree.exists(("cases", case_id, kind, sequence)):
        raise ContainmentError(f"{relative!r} is not a run or screen folder of {case_id}")
    return CaseFolder(tree, case_id, kind, sequence)


@dataclass(frozen=True)
class CaseView:
    """Where a case's current results are read: the run (solve outputs and the records derived from them), the export
    and the screen a selection names. `source` is what the selection states the results are: a run folder of the case
    (`runs/000003`), or `STORED_SOURCE` when they are the ones already stored in the case's own folders (the legacy
    location), which is also how a case with no selection is read."""

    case_root: Path
    selection: str | None
    run: Path
    run_relative: str | None
    export: Path
    export_relative: str | None
    screen: Path | None
    screen_relative: str | None
    source: str = STORED_SOURCE

    @property
    def comparison_root(self) -> Path:
        """The folder whose comparison/ and parsed/ are current: the screen's when one is selected, else the run's."""
        return self.screen if self.screen is not None else self.run


SELECTION_MEMBERS = {"run": RUNS_DIR, "export": RUNS_DIR, "screen": SCREENS_DIR}
_SHA256_PATTERN = re.compile(r"^[0-9a-f]{64}\Z")


def _source_of(value: Any) -> str | None:
    """A selection's stated source: a run folder name, `STORED_SOURCE`, or None when it states none."""
    if value == STORED_SOURCE:
        return STORED_SOURCE
    if isinstance(value, str):
        kind, _, sequence = value.partition("/")
        if kind == RUNS_DIR and SEQUENCE_PATTERN.match(sequence):
            return value
    return None


def _folder_name(value: Any, kind: str) -> bool:
    if not isinstance(value, str):
        return False
    prefix, _, sequence = value.partition("/")
    return prefix == kind and bool(SEQUENCE_PATTERN.match(sequence))


def _verified_problem(also: Any) -> str | None:
    """Why `also` is not a mapping of plain relative paths (components of letters, digits, `.`, `_`, `-`, not
    starting with a dot) to lower-case SHA-256 values, or None."""
    if not isinstance(also, Mapping):
        return "its verified files are not a mapping"
    for relative, digest in also.items():
        parts = relative.split("/") if isinstance(relative, str) else []
        if not parts or not all(isinstance(part, str) and NAME_PATTERN.match(part) for part in parts):
            return f"its verified file {relative!r} is not a plain relative path"
        if not isinstance(digest, str) or not _SHA256_PATTERN.match(digest):
            return f"its verified file {relative!r} does not carry a SHA-256"
    return None


def read_selection(config: "HarnessConfig | Path", case_id: str) -> tuple[str | None, dict[str, Any] | None]:
    """(path relative to the case folder, members) of the case's current selection; (None, None) only when the case
    has no selection at all (no selections folder, or one holding no file).

    The selections must be trusted whole, or the reader stops (SelectionError, every link, type and read error
    included) and reads neither an older selection nor the case's own folders: the folder holds only `NNNNNN.json`
    files numbered 1 to N without a gap; each is a selection record of this case whose `sequence` is its own number,
    whose `previous` names the one before and whose `run` states its source (a run folder of the case, or
    `STORED_SOURCE`); and the current one, number N, names only run, export and screen folders of the case that are
    complete, valid, of this case and that folder, not sealed, ended in an outcome eligible for selection and agree
    with its source (`run_problem`), with the files it was published with present at the hashes it records."""

    try:
        return _read_selection(config, case_id)
    except SelectionError:
        raise
    except (ContainmentError, OSError, TypeError, ValueError, AttributeError, KeyError) as error:
        raise SelectionError(f"cases/{case_id}/{SELECTIONS_DIR} cannot be read as trusted selections "
                             f"({type(error).__name__}: {error}); the case is not read") from None


def _read_selection(config: "HarnessConfig | Path", case_id: str) -> tuple[str | None, dict[str, Any] | None]:
    root = _root_of(config)
    case_root = contained(root, root / "cases" / _check_case_id(case_id))
    folder = contained(root, case_root / SELECTIONS_DIR)
    if folder.is_symlink() or (folder.exists() and not folder.is_dir()):
        raise SelectionError(f"cases/{case_id}/{SELECTIONS_DIR} is not a plain folder; the case is not read")
    if not folder.is_dir():
        return None, None
    try:
        entries = sorted(os.listdir(folder))
    except OSError as error:
        raise SelectionError(f"cases/{case_id}/{SELECTIONS_DIR} cannot be listed ({error.strerror}); the case is not "
                             "read") from None
    if not entries:
        return None, None
    strays = [name for name in entries if not re.fullmatch(r"[0-9]{6}\.json", name)]
    if strays:
        raise SelectionError(f"cases/{case_id}/{SELECTIONS_DIR} holds {', '.join(strays)}, which is not a selection; "
                             "the case is not read")
    numbers = [int(name[:6]) for name in entries]
    if numbers != list(range(1, len(numbers) + 1)):
        raise SelectionError(f"cases/{case_id}/{SELECTIONS_DIR} is not numbered 1 to {len(numbers)} without a gap; "
                             "the case is not read")
    value: dict[str, Any] | None = None
    for number, name in zip(numbers, entries):
        value = _read_record(contained(root, folder / name))
        expected_previous = f"{SELECTIONS_DIR}/{number - 1:06d}.json" if number > 1 else None
        if value is None or value.get("document_kind") != SELECTION_KIND or value.get("sequence") != number \
                or value.get("previous") != expected_previous or value.get("case_id") != case_id:
            raise SelectionError(f"cases/{case_id}/{SELECTIONS_DIR}/{name} is unreadable, not a selection of this "
                                 "case, or conflicts with its number or with the one before it; the case is not read")
        if _source_of(value.get("run")) is None:
            raise SelectionError(f"cases/{case_id}/{SELECTIONS_DIR}/{name} states no source (its run is "
                                 f"{value.get('run')!r}, neither a run folder of the case nor {STORED_SOURCE!r}); "
                                 "the case is not read")
    assert value is not None
    verified = value.get("verified", {})
    if not isinstance(verified, dict):
        raise SelectionError(f"cases/{case_id}/{entries[-1]}: its verified files are not a mapping; the case is not "
                             "read")
    source = _source_of(value["run"])
    for member, kind in SELECTION_MEMBERS.items():
        named = value.get(member)
        if member == "run" and named == STORED_SOURCE:
            continue
        if named is None and member != "run":
            continue
        if not _folder_name(named, kind):
            problem = f"its {member} {named!r} is not a {kind} folder name"
        else:
            problem = run_problem(contained(root, case_root / named),
                                  {"run": "solve", "export": "export", "screen": "screen"}[member],
                                  also=verified.get(member) or {}, case_id=case_id, relative=named,
                                  source=None if member == "run" else source)
        if problem:
            raise SelectionError(f"cases/{case_id}/{entries[-1]} names {member} {named!r}, which is not complete and "
                                 f"valid ({problem}); the case is not read")
    return f"{SELECTIONS_DIR}/{entries[-1]}", value


def current_case(config: "HarnessConfig | Path", case_id: str) -> CaseView:
    """The one place every reader resolves a case's current run, export and screen (`CaseView`).

    Before anything of the case is read, its source's run record is read for the sealed flag: a sealed source (which
    a selection can never name, `run_problem`, and which no legacy record carries) stops the reader (SelectionError)."""

    root = _root_of(config)
    case_root = contained(root, root / "cases" / _check_case_id(case_id))
    selection, value = read_selection(root, case_id)
    if value is None:
        view = CaseView(case_root, None, case_root, None, case_root, None, None, None, STORED_SOURCE)
    else:
        source = _source_of(value["run"])
        run_rel = None if source == STORED_SOURCE else source
        export_rel, screen_rel = value.get("export"), value.get("screen")
        run = case_root / run_rel if run_rel else case_root
        export = case_root / export_rel if export_rel else run
        screen = case_root / screen_rel if screen_rel else None
        view = CaseView(case_root, selection, run, run_rel, export, export_rel, screen, screen_rel, source)
    for folder in {view.run, view.export}:
        try:
            record = _read_record(contained(root, folder.joinpath(*RUN_RECORD)))
        except ContainmentError as error:
            raise SelectionError(f"cases/{case_id}: {error}; the case is not read") from None
        if outputs_sealed(record):
            raise SelectionError(f"cases/{case_id}: the run record at {folder.relative_to(case_root).as_posix() or '.'} "
                                 "marks its outputs sealed; they are not read")
    return view


_UNCHANGED = object()


def write_selection(tree: Tree, case_id: str, *, stage: "CaseFolder", reason: str, expect_run: Any = _UNCHANGED,
                    verified: Mapping[str, Mapping[str, str]] | None = None,
                    validate: Callable[[Mapping[str, Any]], str | None] | None = None,
                    **members: str | None) -> str | None:
    """A new selection: the current one's members with `members` changed, as the next numbered file. It is written
    whole as a new file of `stage` (the folder being published, `selection-<number>-<random>.json`) and only then
    given its name in `selections/` with an exclusive `link`, so a crash leaves either no selection or a complete one,
    never a partial one (a number taken meanwhile is re-read and skipped). It always states its source: a case with
    no selection yet gets `run: STORED_SOURCE` unless a run is being published. With `expect_run` (a source),
    nothing is written (None) when the current source is no longer that one. `validate(current)` names why the
    member cannot be published against the current selection (ContainmentError, nothing written). `verified`
    ({member: {path: SHA-256}}) records the files each changed member was published with; readers check them.
    Returns the new selection's path relative to the case folder. Call it through `publish`."""

    for member in members:
        if member not in SELECTION_MEMBERS:
            raise ValueError(f"{member!r} is not a selection member")
    tree.ensure_dir(("cases", case_id, SELECTIONS_DIR))
    for _ in range(1000):
        previous, current = read_selection(tree.base, case_id)
        current = current or {}
        current_source = _source_of(current.get("run")) or STORED_SOURCE
        if expect_run is not _UNCHANGED and current_source != (_source_of(expect_run) or STORED_SOURCE):
            return None
        if validate is not None:
            problem = validate(current)
            if problem:
                raise ContainmentError(f"cases/{case_id}: not published: {problem}")
        number = int(previous.split("/")[-1][:6]) + 1 if previous else 1
        name = f"{number:06d}.json"
        carried = dict(current.get("verified") or {})
        for member, relative in members.items():
            carried[member] = dict((verified or {}).get(member) or {}) if relative else {}
        base = {member: current.get(member) for member in SELECTION_MEMBERS}
        base["run"] = current_source
        value = {"document_kind": SELECTION_KIND, "case_id": case_id, "sequence": number,
                 **base, **members, "verified": carried,
                 "previous": previous, "reason": reason, "created": _now(), "boundary_note": BOUNDARY_NOTE}
        if _source_of(value["run"]) is None:
            raise ContainmentError(f"cases/{case_id}: a selection must state its source; {value['run']!r} is none")
        staged = f"selection-{number:06d}-{secrets.token_hex(4)}.json"
        stage.record(staged, value)
        try:
            tree.link_new((*stage.parts, staged), ("cases", case_id, SELECTIONS_DIR, name))
        except ExistingPath:
            continue
        return f"{SELECTIONS_DIR}/{name}"
    raise ContainmentError(f"no new selection could be written for {case_id}")


def _now() -> str:
    return datetime.now(timezone.utc).isoformat(timespec="seconds")


def _sha256(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def last_json_object(text: str) -> dict[str, Any] | None:
    """The last top-level JSON object in `text`, which may be preceded or followed by other output."""

    decoder = json.JSONDecoder()
    found = None
    index = 0
    while True:
        start = text.find("{", index)
        if start < 0:
            return found
        try:
            value, end = decoder.raw_decode(text, start)
        except json.JSONDecodeError:
            index = start + 1
            continue
        if isinstance(value, dict):
            found = value
        index = end


def _literal_recent_value(config: HarnessConfig) -> tuple[str, str] | None:
    """(recent-files key id, value name) of the one literal value a restore may address (LITERAL_VALUE_FOLDER): the
    configured executable's folder with one trailing backslash, inside the configured recent-files subkey. None
    without a recent-files subkey, or when the executable is not in a named folder below a drive."""

    guard = config.settings_guard
    if guard is None or guard.recent_files_key is None or not valid_guest_path(config.guest_executable,
                                                                               directory=False):
        return None
    folder = config.guest_executable.rsplit("\\", 1)[0]
    if not LITERAL_VALUE_FOLDER.match(folder):
        return None
    return _registry_key_id(guard.recent_files_key), folder + "\\"


def _literal_value_command(printed_key: str, name: str, action: str, data: str | None = None) -> str:
    """The guest command for one `read`, `set` (REG_BINARY `data`) or `delete` of the literal value: PowerShell's
    registry API with the key, the name and the data passed as encoded data, never as quoted command words. It
    prints what `reg` prints: the value as `reg query` shows it, `REG_WRITE_OK`, or `REG_NOT_FOUND` with exit 1."""

    if not REG_PRINTED_KEY_PATTERN.match(printed_key) or action not in ("read", "set", "delete") \
            or (action == "set") != (data is not None) or (data is not None and not re.fullmatch(r"(?:[0-9A-F]{2})*",
                                                                                                data)):
        raise ValueError(f"{printed_key}\\{name} cannot be written back exactly")
    payload = json.dumps({"key": printed_key[len("HKEY_CURRENT_USER\\"):], "name": name, "action": action,
                          "data": data}, sort_keys=True)
    script = LITERAL_VALUE_SCRIPT.replace("@PAYLOAD@", base64.b64encode(payload.encode("utf-8")).decode("ascii"))
    encoded = base64.b64encode(script.encode("utf-16-le")).decode("ascii")
    command = f"powershell -NoProfile -NonInteractive -ExecutionPolicy Bypass -EncodedCommand {encoded}"
    if len(command) > LITERAL_COMMAND_LIMIT:  # cmd.exe would cut it: refused while the plan is made, before any write
        raise ValueError(f"{printed_key}\\{name} cannot be written back exactly (its guest command is too long)")
    return command


def _quote(path: str) -> str:
    # Only allow-listed guest paths (GUEST_PATH_PATTERN, CASE_ID_PATTERN and fixed names) are ever quoted here.
    if not re.fullmatch(r"[A-Za-z0-9 \\._()*:-]*", path):
        raise ValueError("a guest path outside the allow-list would enter a guest command")
    return f'"{path}"'


def _win(*parts: str) -> str:
    """Join guest (Windows) path parts with a backslash."""
    return "\\".join(parts)


def run_caepipe_round_trip(config: HarnessConfig, case_id: str, mbf_text: str, *, timeout_s: float = 600,
                           runner: Callable[..., subprocess.CompletedProcess] = subprocess.run,
                           gui_fallback: bool = False, files: Mapping[str, bytes] | None = None,
                           capture_only: bool = False) -> dict[str, Any]:
    """Solve one MBF with the user's CAEPIPE in the configured virtual machine and record what happened.

    The guest work is done in three `prlctl exec <vm> --current-user cmd /c <command>`
    steps (prepare, solve, collect) so each step's exit status is recorded on its
    own. Everything the step writes is a new file in its own new run folder (`cases/<id>/runs/<NNNNNN>/`,
    `run_folder` in the record): the marker first, then, once the exchange root is established by content
    (`_resolve_exchange_root`) and the guest's work folder is shown to lie off every shared drive, the MBF
    (`caepipe_in/model.mbf`) and `files` ({path in the run folder: bytes}, such as `swbpipe/request.json`); the
    guest collects into the run folder's empty `caepipe_out/`. A refused step (`exchange_drive_unavailable`,
    `exchange_drive_ambiguous`, `exchange_check_failed`, `work_folder_on_exchange_drive`) writes its full run record
    there and nothing else. A run whose guest reported solving this MBF becomes the case's current run through a new
    selection. A failed prepare step stops before solving (`prepare_failed`). Outputs are
    collected, even when the solve step fails, only when the guest reports solving
    an MBF whose SHA-256 equals the host's (otherwise `input_hash_mismatch`).

    The guest script is always told not to fall back to printing through the program's window (`-NoGuiFallback`):
    a batch run that leaves no result file ends as `batch_no_csv`, with no fallback attempted. `gui_fallback=True`
    is refused by name before anything is started (`window_route_unavailable`, a complete run record in the step's
    new run folder and nothing else). With `settings_guard` configured, the run record's `settings_guard` holds the
    detect-only check; an exception raised after the guest work started carries it as `settings_guard`.
    """

    if gui_fallback:
        # The print fallback through the program's window is enabled only under DEL-17-05 REQ-023 (the listed-window
        # rule); until then its code in `_round_trip_body` and the guest script is reachable from no command.
        return _locked(config, case_id, "round_trip", None,
                       lambda run, started: _base_record(config, case_id, run, "round_trip", started),
                       refuse_before=(WINDOW_ROUTE_UNAVAILABLE, WINDOW_ROUTE_REASON))
    return _locked(config, case_id, "round_trip", lambda tree, run, live: _round_trip(
        config, case_id, mbf_text, tree, run, live, timeout_s, runner, gui_fallback, dict(files or {}), capture_only),
        lambda run, started: _base_record(config, case_id, run, "round_trip", started))


@dataclass
class _RunLock:
    """The run held by this process: an advisory lock taken with `flock` (RUN_LOCK_CALL; it belongs to the open file
    description and is released only when every copy of that description is closed, never by closing some other
    descriptor of the file) on the permanent, empty lock file, through `fd` (closed on exec, so no child process
    inherits it). Closing `fd`, or the end of the process however it ends, releases it; nothing is removed."""

    fd: int
    holder_record: str
    since: str
    attempts: list[dict[str, Any]]
    predecessor: dict[str, Any] | None

    def release(self) -> None:
        if self.fd >= 0:
            os.close(self.fd)
            self.fd = -1


def _latest_holder(tree: Tree) -> tuple[str | None, Any]:
    """(path, content) of the newest holder record; content None when it cannot be read whole."""
    names = [name for name in tree.listdir((HARNESS_DIR, HOLDERS_DIR)) if re.fullmatch(r"[0-9]{6}\.json", name)]
    if not names:
        return None, None
    name = max(names)
    return f"{HARNESS_DIR}/{HOLDERS_DIR}/{name}", tree.read_json((HARNESS_DIR, HOLDERS_DIR, name))


def _holder_run(holder: Any) -> tuple[str, ...] | None:
    """The run folder a holder record names, as validated names (`cases/<id>/runs/<NNNNNN>`), never a path taken from
    the record; None when it names none."""
    if not isinstance(holder, dict):
        return None
    case_id, run = holder.get("case_id"), holder.get("run")
    if not (isinstance(case_id, str) and CASE_ID_PATTERN.match(case_id) and _folder_name(run, RUNS_DIR)):
        return None
    return ("cases", case_id, *run.split("/"))


def _holder_state(tree: Tree, path: str | None, holder: Any) -> dict[str, Any]:
    """The newest holder record, named: `none` (no holder record yet), `incomplete` (it cannot be read whole or names
    no run), `ended` (its run has its end record) or `unfinished` (it has none: its holder still runs, or ended
    without reaching its end)."""
    if path is None:
        return {"state": "none", "holder_record": None, "holder": None}
    parts = _holder_run(holder)
    if parts is None:
        return {"state": "incomplete", "holder_record": path, "holder": holder}
    ended = tree.exists((*parts, RUN_LOCK_END))
    return {"state": "ended" if ended else "unfinished", "holder_record": path, "holder": holder}


def _settings_stop(tree: Tree, holder: Any) -> tuple[str | None, str | None]:
    """Why the run of `holder` stops later live steps (`settings_not_restored`), or None. It does when it wrote the
    saved settings' before-state (it may then have started the program) and has no check record, or one whose
    settings are neither restored nor unchanged, and no clearance exists for it. The reason names the run, when its
    before-state was written and the members concerned, by name only: no value and no hash leaves its run folder.
    (reason or None, the clearance the step proceeds on or None)."""
    parts = _holder_run(holder)
    if parts is None or not tree.exists((*parts, SETTINGS_STAGE)):
        return None, None
    check = tree.read_json((*parts, SETTINGS_CHECK))
    if isinstance(check, dict) and check.get("restored_or_unchanged") is True:
        return None, None
    clearance = tree.read_json((*parts, SETTINGS_CLEARANCE))
    if isinstance(clearance, dict) and clearance.get("run") == "/".join(parts):
        return None, f"{'/'.join(parts)}/{SETTINGS_CLEARANCE}"
    before = tree.read_json((*parts, SETTINGS_STAGE))
    when = before.get("written") if isinstance(before, dict) else "unknown (the record cannot be read whole)"
    if isinstance(check, dict):
        members = [str(name) for name in check.get("differing_members") or []]
        found = f"its check record says {check.get('status')!r}, the members that differ being"
    else:
        members = [f"{item.get('key')}\\{item.get('name')}" for item in (before or {}).get("members", [])
                   if isinstance(item, dict)] if isinstance(before, dict) else []
        found = "it has no check record (it ended before its settings were compared), the members concerned being"
    return (f"the run {'/'.join(parts)} wrote the saved settings' before-state at {when} and {found} "
            f"{', '.join(members) or 'not named in its records'}; the settings are not recorded to have been "
            "restored and no clearance exists for that run. Nothing is restored or removed here; a clearance is "
            "recorded only by the explicit command tools/caepipe_correlation/clear_settings_stop.py"), None


def _holder_words(path: str | None, holder: Any) -> str:
    """Whose lock it is, in words a reader can act on, from a holder record."""
    if not isinstance(holder, dict):
        return f"held; its holder record ({path}) cannot be read"
    return (f"held by the {holder.get('step')} step of {holder.get('run_folder')} since {holder.get('since')}, on host "
            f"{holder.get('host')}, process {holder.get('process_id')} (holder record {path})")


def _try_flock(fd: int) -> str:
    try:
        fcntl.flock(fd, fcntl.LOCK_EX | fcntl.LOCK_NB)
        return "taken"
    except BlockingIOError:
        return "held"
    except OSError as error:
        return f"refused: {error.strerror}"


def _new_holder_record(tree: Tree, run: CaseFolder, value: Mapping[str, Any]) -> str:
    """The next numbered holder record `_harness/holders/<NNNNNN>.json`: written whole in the run folder, then given
    that name with an exclusive link (a number taken meanwhile is skipped)."""
    run.record(HOLDER_STAGED, value)
    for _ in range(1000):
        numbers = [int(name[:6]) for name in tree.listdir((HARNESS_DIR, HOLDERS_DIR))
                   if re.fullmatch(r"[0-9]{6}\.json", name)]
        name = f"{max(numbers, default=0) + 1:06d}.json"
        try:
            tree.link_new((*run.parts, HOLDER_STAGED), (HARNESS_DIR, HOLDERS_DIR, name))
        except ExistingPath:
            continue
        return f"{HARNESS_DIR}/{HOLDERS_DIR}/{name}"
    raise ContainmentError("no new holder record could be written")


def _take_run_lock(tree: Tree, run: CaseFolder, case_id: str, step_name: str) -> "_RunLock | dict[str, Any]":
    """Hold the run for this step, or say why not (a refusal: route, reason, the named state of the newest holder).

    What the lock does not show: it shows only that the harness's host process holding it has ended, and nothing
    about the guest; a program instance or a guest script in the guest cannot hold a host lock, so the successor
    accounts for the guest by its own named checks (`_guest_processes`, `_settings_stop`).

    Every check comes before one marked point, the holder record. The platform offers `flock` (RUN_LOCK_CALL; never a
    POSIX record lock, which closing any other descriptor of the file would drop). The lock file
    (`_harness/round_trip.lock`) is permanent and empty: created exclusively when absent, never written, replaced or
    removed; it is opened without following a link. The attempt is non-blocking, with one bounded wait
    (RUN_LOCK_EXIT_WAIT_S) for a holder still exiting, then the lock is taken or the step refused as `run_lock_held`
    with the newest holder record in its named state (held by it; held, its own holder record not yet written; newest
    holder record incomplete). A second open of the lock file (a separate open file description) must then fail to
    take it, which shows the file system honours the lock (otherwise `run_lock_unavailable`); the held descriptor must
    still be the file at that name (otherwise refused: it was replaced). The newest holder record must be readable
    whole and name this host. A newest holder whose run has no end record ended unfinished: recorded, and it feeds
    the named stops; its run's settings records decide `settings_not_restored`. No process id is used to judge
    whether a holder lives: the kernel releases the lock when its process ends."""

    lock = (HARNESS_DIR, LOCK_NAME)
    where = f"{HARNESS_DIR}/{LOCK_NAME}"
    if fcntl is None:
        return {"route": RUN_LOCK_UNAVAILABLE_ROUTE,
                "reason": f"this platform offers no flock advisory lock for {where}; a live step needs one"}
    tree.ensure_dir((HARNESS_DIR,))
    if tree.identity(lock) is None:
        try:
            tree.create(lock, b"")  # permanent and empty: never written, replaced or removed
        except ExistingPath:  # created meanwhile by another step: used as it is
            pass
    fd = tree.open_for_lock(lock)
    held = False
    attempts: list[dict[str, Any]] = []
    try:
        outcome = _try_flock(fd)
        attempts.append({"at": _now(), "outcome": outcome})
        if outcome == "held":  # one bounded wait for a holder that is still exiting
            time.sleep(RUN_LOCK_EXIT_WAIT_S)
            outcome = _try_flock(fd)
            attempts.append({"at": _now(), "outcome": outcome, "after_wait_s": RUN_LOCK_EXIT_WAIT_S})
        if outcome.startswith("refused"):
            return {"route": RUN_LOCK_UNAVAILABLE_ROUTE, "attempts": attempts,
                    "reason": f"the file system refused the advisory lock on {where} ({outcome})"}
        if outcome == "held":
            path, holder = _latest_holder(tree)
            view = _holder_state(tree, path, holder)
            described = {
                "none": "held; its holder record is not yet written (there is none)",
                "incomplete": f"held; the newest holder record ({path}) is incomplete",
                "ended": f"held; its holder record is not yet written (the newest, {path}, is of a holder that ended)",
                "unfinished": _holder_words(path, holder),
            }[view["state"]]
            return {"route": RUN_LOCK_HELD_ROUTE, "attempts": attempts, **view,
                    "reason": f"another step holds the run ({where}: {described})"}
        second = tree.open_for_lock(lock)
        try:
            honoured = _try_flock(second) == "held"
        finally:
            os.close(second)  # it holds nothing; the first description's lock stays in force
        if not honoured:
            return {"route": RUN_LOCK_UNAVAILABLE_ROUTE, "attempts": attempts,
                    "reason": f"the file system does not honour the flock lock on {where}: a second open of it was not "
                              "refused while this step held it"}
        opened = os.fstat(fd)
        if tree.identity(lock) != (opened.st_dev, opened.st_ino):
            return {"route": RUN_LOCK_HELD_ROUTE, "attempts": attempts, "state": "replaced",
                    "reason": f"{where} was replaced after it was opened; the lock held is not the file at that name"}
        host = socket.gethostname()
        path, holder = _latest_holder(tree)
        predecessor = _holder_state(tree, path, holder)
        if predecessor["state"] == "incomplete":
            return {"route": RUN_LOCK_HELD_ROUTE, "attempts": attempts, **predecessor,
                    "reason": f"the newest holder record ({path}) is incomplete: whose run ended there, and how, cannot "
                              "be told"}
        if predecessor["state"] != "none" and holder.get("host") != host:
            return {"route": RUN_LOCK_HELD_ROUTE, "attempts": attempts, **predecessor,
                    "reason": f"the newest holder record ({path}) names host {holder.get('host')}, not this host "
                              f"{host}; the lock covers one host only"}
        stop, cleared = _settings_stop(tree, holder) if predecessor["state"] != "none" else (None, None)
        if stop:
            return {"route": SETTINGS_NOT_RESTORED_ROUTE, "attempts": attempts, **predecessor, "reason": stop}
        if cleared:
            predecessor["proceeded_on_clearance"] = cleared
        # The marked point: every check above passed; from here this step holds the run.
        since = _now()
        record = _new_holder_record(tree, run, {
            "lock": where, "lock_call": RUN_LOCK_CALL, "case_id": case_id, "run": run.relative,
            "run_folder": f"cases/{case_id}/{run.relative}", "step": step_name, "since": since, "host": host,
            "process_id": os.getpid(), "predecessor": predecessor,
            "note": "who took the run lock; released when this process ends, however it ends; the end record "
                    f"{RUN_LOCK_END} in the run folder is written when the step ends in its own process"})
        held = True
        return _RunLock(fd=fd, holder_record=record, since=since, attempts=attempts,
                        predecessor=predecessor if predecessor["state"] != "none" else None)
    finally:
        if not held:
            os.close(fd)


def record_settings_clearance(config: "HarnessConfig | Path", case_id: str, run_relative: str, *, cleared_by: str,
                              reason: str) -> str:
    """Record a clearance for a run that reached the program's saved settings and has no record of their restore: a
    new file `settings_clearance.json` in that run's folder, created exclusively (never rewritten), naming the run,
    who cleared it and why. Only the explicit command `tools/caepipe_correlation/clear_settings_stop.py` calls this;
    no step does. ContainmentError when the run has no stage record, the clearance exists, or a field is empty."""

    _check_case_id(case_id)
    if not _folder_name(run_relative, RUNS_DIR):
        raise ContainmentError(f"{run_relative!r} is not a run folder name (runs/NNNNNN)")
    if not (isinstance(cleared_by, str) and cleared_by.strip() and isinstance(reason, str) and reason.strip()):
        raise ContainmentError("a clearance names who cleared the run and why (what was verified before it was given)")
    with exchange_tree(config) as tree:
        parts = ("cases", case_id, *run_relative.split("/"))
        path, holder = _latest_holder(tree)
        if _holder_run(holder) != parts:
            raise ContainmentError(f"cases/{case_id}/{run_relative} is not the newest holder's run ({path or 'none'}); "
                                   "only the run that stops the next step is cleared")
        stop, _ = _settings_stop(tree, holder)
        if not stop:
            raise ContainmentError(f"cases/{case_id}/{run_relative} does not stop the next step; there is nothing to "
                                   "clear")
        tree.record((*parts, SETTINGS_CLEARANCE), {
            "run": "/".join(parts), "cleared_by": cleared_by.strip(), "reason": reason.strip(), "created": _now(),
            "check_record_present": tree.exists((*parts, SETTINGS_CHECK)),
            "note": "an explicit act: the harness never writes a clearance by itself"})
    return f"cases/{case_id}/{run_relative}/{SETTINGS_CLEARANCE}"


def hold_run_lock(config: "HarnessConfig | Path", case_id: str, step_name: str = "held") -> "_RunLock | dict[str, Any]":
    """Hold the run as a live step does, in a new run folder of `case_id` (for tests and for an operator who must keep
    live steps out); release it with `.release()` or by ending the process."""
    tree = exchange_tree(config)
    run = reserve_folder(tree, case_id, RUNS_DIR)
    return _take_run_lock(tree, run, case_id, step_name)


def _locked(config: HarnessConfig, case_id: str, step_name: str,
            work: Callable[[Tree, CaseFolder, dict[str, Any]], dict[str, Any]] | None,
            make_base: Callable[[CaseFolder, str], dict[str, Any]], *,
            refuse_before: tuple[str, str] | None = None) -> dict[str, Any]:
    """Check the guest paths, reserve the step's new run folder, hold the run (`_take_run_lock`) and run `work`.

    From the moment the run folder is reserved there is one exceptional path: any exception (taking the lock
    included) leaves a complete run record of the exceptional end in that folder (`_record_exceptional_end`, under
    another new name when the first is occupied or partial), the original exception is the one raised, and a record
    that cannot be written is stated on it. A step that cannot hold the run is refused by name and recorded; it
    issues no guest command. Nothing is removed: the lock is released by closing its descriptor.

    With `refuse_before` (route, reason) the step is refused right after its run folder is reserved: a complete run
    record there and nothing else (no lock, no holder record, no guest command)."""

    if config.prlctl is None:
        raise ValueError("the harness configuration was resolved without prlctl; live runs need it")
    for key in GUEST_PATH_KEYS:
        directory = key != "guest_executable"
        # Resolution strips a directory's trailing backslash (a drive root `X:\` is held as `X:`).
        value = getattr(config, key) + ("\\" if directory else "")
        if not valid_guest_path(value, directory=directory):
            raise ValueError(f"{key} is not an allow-listed absolute Windows path")
    _check_case_id(case_id)
    tree = exchange_tree(config)
    try:
        run = reserve_folder(tree, case_id, RUNS_DIR)
        started = _now()
        live: dict[str, Any] = {}
        try:
            if refuse_before is not None or work is None:
                route, reason = refuse_before or (WINDOW_ROUTE_UNAVAILABLE, WINDOW_ROUTE_REASON)
                record = make_base(run, started)
                record.update(route=route, reason=reason)
                if step_name == "mbf_export":
                    record["mbf_export"].update(route=route, reason=reason)
                live["record"] = record
                path = _write_run_record(run, record)
                live["record_written"] = True
                return {"route": route, "run_folder": run.relative, "run_record": path,
                        **({"mbf_export": record["mbf_export"]} if step_name == "mbf_export" else {}),
                        "reason": f"{reason}; nothing stored was changed; recorded at cases/{case_id}/{path}"}
            held = _take_run_lock(tree, run, case_id, step_name)
            if isinstance(held, dict):
                record = make_base(run, started)
                record.update(route=held["route"], reason=held["reason"], run_lock={
                    "call": RUN_LOCK_CALL, **{key: held[key] for key in ("state", "holder_record", "holder", "attempts")
                                              if key in held}})
                if step_name == "mbf_export":
                    record["mbf_export"].update(route=held["route"], reason=held["reason"])
                live["record"] = record
                path = _write_run_record(run, record)
                live["record_written"] = True
                return {"route": held["route"], "run_folder": run.relative, "run_record": path,
                        **({"mbf_export": record["mbf_export"]} if step_name == "mbf_export" else {}),
                        "reason": f"{held['reason']}; nothing stored was changed; recorded at cases/{case_id}/{path}"}
            live["run_lock"] = {"holder_record": held.holder_record, "since": held.since, "call": RUN_LOCK_CALL,
                                "attempts": held.attempts, "predecessor": held.predecessor}
            ending = "returned"
            try:
                result = work(tree, run, live)
            except BaseException as error:
                ending = f"exception: {type(error).__name__}"
                raise
            finally:
                try:  # the end record: this holder ended in its own process
                    run.record(RUN_LOCK_END, {"ended": _now(), "outcome": ending,
                                              "holder_record": held.holder_record})
                except Exception as late:
                    live["run_lock"]["end_record"] = f"not written: {type(late).__name__}: {late}"
                held.release()
            if isinstance(result, dict):
                result["run_lock"] = dict(live["run_lock"])
            return result
        except BaseException as error:
            try:  # where the exceptional end is recorded, for the caller
                error.run_folder = run.relative
            except Exception:
                pass
            _record_exceptional_end(run, live, error, lambda: make_base(run, started))
            raise
    finally:
        tree.close()


def _file_sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with open(path, "rb") as handle:
        for block in iter(lambda: handle.read(1 << 20), b""):
            digest.update(block)
    return digest.hexdigest()


@dataclass
class _Marker:
    run: CaseFolder
    data: bytes
    sha256: str


def _create_marker(run: CaseFolder) -> _Marker:
    """This step's marker: a new file in its new run folder, created exclusively, with random content chosen now. It
    stays there: a step removes nothing."""

    data = f"swbpipe exchange check {secrets.token_hex(32)}\n".encode("ascii")
    run.create(EXCHANGE_MARKER, data)
    return _Marker(run=run, data=data, sha256=_sha256(data))


MARKER_KEPT = "kept in the run folder"


def _exchange_check_text(items: list[tuple[str, str]]) -> str:
    for label, path in items:
        _quote(path)  # only allow-listed guest paths, as every other guest command
        if not re.fullmatch(r"[a-z_]+", label):
            raise ValueError(f"exchange check label {label!r} is not allowed")
    payload = json.dumps([{"label": label, "path": path} for label, path in items], sort_keys=True)
    script = EXCHANGE_CHECK_SCRIPT.replace("@PAYLOAD@", base64.b64encode(payload.encode("utf-8")).decode("ascii"))
    encoded = base64.b64encode(script.encode("utf-16-le")).decode("ascii")
    return f"powershell -NoProfile -NonInteractive -ExecutionPolicy Bypass -EncodedCommand {encoded}"


def _exchange_check_command(items: list[tuple[str, str]]) -> str:
    """The guest command of one exchange check group: PowerShell with the labels and paths passed as encoded data.
    ValueError when it would exceed the guest command limit."""

    command = _exchange_check_text(items)
    if len(command) > LITERAL_COMMAND_LIMIT:
        raise ValueError("an exchange check's guest command would exceed the guest command limit")
    return command


def _exchange_check_groups(items: list[tuple[str, str]]) -> list[list[tuple[str, str]]]:
    """`items` in order, in as few consecutive groups as keep each group's command within the limit."""

    groups: list[list[tuple[str, str]]] = []
    for item in items:
        if groups and len(_exchange_check_text([*groups[-1], item])) <= LITERAL_COMMAND_LIMIT:
            groups[-1].append(item)
        else:
            _exchange_check_command([item])  # a single path that cannot fit raises here
            groups.append([item])
    return groups


def _exchange_check(config: HarnessConfig, step: Callable[[str, str], dict[str, Any]],
                    needs: Mapping[str, tuple[str, str]], name: str = "exchange_check") -> dict[str, Any]:
    """Hash, read only, each guest path of `needs` ({label: (guest path, SHA-256 the host holds)}) as the guest sees it.

    The guest prints `sha256 <label> <hex>`, `missing <label>` or `unreadable <label>` (`EXCHANGE_CHECK_SCRIPT`,
    `Get-FileHash` as the guest script uses); the paths travel as encoded data, never as command words, in as many
    guest steps as keep each command within the limit. `ready` only when every step exits 0 and every label's hash
    equals the host's; otherwise `unavailable`, with every label's state. A label answered more than once (whether
    the answers agree or not) is `conflicting`, never a match. ValueError when a path cannot fit."""

    reported: dict[str, list[str]] = {}
    exits: list[Any] = []
    for number, group in enumerate(_exchange_check_groups([(label, path) for label, (path, _) in needs.items()]), 1):
        record = step(name if number == 1 else f"{name}_{number}", _exchange_check_command(group))
        exits.append(record["exit_status"])
        labels = {label for label, _ in group}
        for line in (record.get("stdout") or "").splitlines():
            parts = line.split()
            if len(parts) == 3 and parts[0] == "sha256" and parts[1] in labels and re.fullmatch(r"[0-9a-f]{64}",
                                                                                                  parts[2]):
                reported.setdefault(parts[1], []).append(parts[2])
            elif len(parts) == 2 and parts[0] in ("missing", "unreadable") and parts[1] in labels:
                reported.setdefault(parts[1], []).append(parts[0])
    states = {}
    for label, (_, expected) in needs.items():
        answers = reported.get(label, [])
        found = answers[0] if len(answers) == 1 else None
        states[label] = ("conflicting" if len(answers) > 1 else "matches" if found == expected.lower()
                         else "not_answered" if found is None
                         else found if found in ("missing", "unreadable") else "differs")
    if all(code == 0 for code in exits) and all(state == "matches" for state in states.values()):
        return {"status": "ready", "files": states, "exit_statuses": exits}
    if any(code != 0 for code in exits) or all(state == "not_answered" for state in states.values()):
        why = f"the guest did not answer the exchange check (exit status {', '.join(map(str, exits))})"
    else:
        why = "not every file is shown under the guest root (" + ", ".join(
            f"{label} {state}" for label, state in states.items() if state != "matches") + ")"
    return {"status": "unavailable", "files": states, "reason": why, "exit_statuses": exits}


def _listed_drive_letters(text: str) -> list[str]:
    """The drive letters a `net use` listing names, upper-case, sorted: a whitespace-separated token of exactly one
    letter and a colon in the first or second column (Status, Local) of a line. Remote names (`\\\\host\\share`),
    paths and anything else are ignored and never used."""

    letters = set()
    for line in text.splitlines():
        for token in line.split()[:2]:
            if re.fullmatch(r"[A-Za-z]:", token):
                letters.add(token.upper())
                break
    return sorted(letters)


def _resolve_exchange_root(config: HarnessConfig, step: Callable[[str, str], dict[str, Any]],
                           marker_parts: tuple[str, ...], marker_sha256: str) -> dict[str, Any]:
    """The guest exchange root for this step, established by content, never by name or letter alone; read only.

    The guest's `net use` listing is read first. Then the configured drive letter and every other letter the
    listing names (each the configured root with only its letter replaced, a guest path the configuration would
    accept) is asked for one thing only: the SHA-256 of this step's marker at `marker_parts` below it, in letter order
    and bounded groups. Exactly one match across all of them: that root is `ready` for this step only (the
    configuration is never written), recorded with the configured letter and whether they differ. More than one
    (the configured letter and another included): `ambiguous`. None, or a listing that cannot be read: `unavailable`.
    An oversize command or anything unexpected: `failed`. The step's inputs are checked afterwards, under the
    selected root only (`_check_inputs`).

    Limits: the guest is taken to answer truthfully and its drive mappings to stay put during the step; a live
    synchronised replica of the exchange folder could show the marker; only the letter is replaced, so the folder
    is not searched for at another relative path."""

    configured = config.guest_exchange_root
    letter, tail = configured[:2].upper(), configured[2:]
    result: dict[str, Any] = {"configured_letter": letter}
    try:
        listing = step("drive_listing", "net use")
        if listing["exit_status"] != 0:
            return {**result, "status": "unavailable", "listing": {"exit_status": listing["exit_status"]},
                    "reason": "the guest's drive listing could not be read, so no drive can be shown to be the only "
                              "one holding this exchange folder"}
        listed = _listed_drive_letters(listing.get("stdout") or "")
        candidates = sorted({letter, *(other for other in listed if valid_guest_path(other + tail + "\\",
                                                                                     directory=True))})
        result["listing"] = {"exit_status": 0, "letters_listed": listed, "letters_tested": candidates}
        labels = {f"drive_{other[0].lower()}": other for other in candidates}
        probe = _exchange_check(config, step, {label: (_win(other + tail, *marker_parts), marker_sha256)
                                               for label, other in labels.items()}, name="exchange_check_drives")
        result["marker_answers"] = {labels[label]: state for label, state in probe["files"].items()}
        passed = [other for label, other in labels.items() if probe["files"][label] == "matches"]
        result["letters_matched"] = passed
        # Every candidate must answer cleanly: a match, or nothing at that path. A conflicting (answered more than
        # once), unanswered, unreadable or differing candidate, or a probe step that failed, refuses the step; such a
        # candidate is never dropped in favour of another.
        doubtful = {labels[label]: state for label, state in probe["files"].items() if state not in ("matches", "missing")}
        failed_steps = [code for code in probe.get("exit_statuses", []) if code != 0]
        if doubtful or failed_steps:
            return {**result, "status": "failed",
                    "reason": "the drive check cannot be trusted: " + "; ".join(
                        [f"{letter} answered {state}" for letter, state in sorted(doubtful.items())]
                        + ([f"a probe step ended with exit status {', '.join(map(str, failed_steps))}"]
                           if failed_steps else []))}
        if len(passed) > 1:
            return {**result, "status": "ambiguous",
                    "reason": f"this step's marker matches under several drive letters ({', '.join(passed)}); none "
                              "is taken"}
        if not passed:
            return {**result, "status": "unavailable",
                    "reason": f"this step's marker matches under no drive letter ({', '.join(candidates)} tested; "
                              f"{probe.get('reason', '')})"}
        return {**result, "status": "ready", "root": passed[0] + tail, "used_letter": passed[0],
                "differed": passed[0] != letter}
    except Exception as error:  # an oversize command or anything unexpected: a named refusal, never an escape
        return {**result, "status": "failed",
                "reason": f"the exchange check could not be made ({type(error).__name__}: {error})"}


def _check_inputs(config: HarnessConfig, step: Callable[[str, str], dict[str, Any]],
                  needs: Mapping[str, tuple[str, str]]) -> dict[str, Any]:
    """The step's inputs, hashed by the guest under the selected root only, after the root is established."""

    try:
        return _exchange_check(config, step, needs, name="exchange_check_inputs")
    except Exception as error:
        return {"status": "failed", "files": {}, "reason": f"the input check could not be made "
                                                           f"({type(error).__name__}: {error})"}


def _work_folder_problem(config: HarnessConfig, resolution: Mapping[str, Any]) -> str | None:
    """Why the guest's work folder may not be used, or None: it must lie off the exchange drive, off every shared
    drive the guest lists and outside the exchange root, so that the guest's deletes and copies there can never reach
    the exchange folder."""

    work = config.guest_work_root
    work_letter = work[:2].upper()
    if work_letter == resolution["used_letter"]:
        return f"the guest work folder is on the exchange drive {work_letter}"
    if work_letter in resolution["listing"]["letters_listed"]:
        return f"the guest work folder is on {work_letter}, a shared drive the guest lists"
    root = resolution["root"].lower()
    if work.lower() == root or work.lower().startswith(root + "\\"):
        return "the guest work folder lies under the exchange root"
    return None


def _encoded_command(template: str, payload: Any) -> str:
    """A guest PowerShell command whose data travel as encoded JSON, never as command words. ValueError when it would
    exceed the guest command limit."""
    script = template.replace("@PAYLOAD@", base64.b64encode(json.dumps(payload, sort_keys=True).encode("utf-8"))
                              .decode("ascii"))
    command = ("powershell -NoProfile -NonInteractive -ExecutionPolicy Bypass -EncodedCommand "
               + base64.b64encode(script.encode("utf-16-le")).decode("ascii"))
    if len(command) > LITERAL_COMMAND_LIMIT:
        raise ValueError("a guest command would exceed the guest command limit")
    return command


# A process is identified by its id together with its start time as the guest reports it (WMI CreationDate, UTC):
# a recorded id with another start time is another process (an id the guest has reused).
_PROCESS_LINE = re.compile(r"^(tool|script) ([0-9]{1,10}) (\S+)\Z")


def parse_guest_processes(stdout: str) -> dict[str, list[dict[str, Any]]] | None:
    """The guest's process listing (`GUEST_PROCESSES_SCRIPT`): {"tool": [{pid, start}], "script": [{pid, start}]}, or
    None unless every line is understood and the listing says it is complete (`listed`)."""
    found: dict[str, list[dict[str, Any]]] = {"tool": [], "script": []}
    complete = False
    for line in (stdout or "").splitlines():
        line = line.strip()
        if not line:
            continue
        if line == "listed":
            complete = True
            continue
        match = _PROCESS_LINE.match(line)
        if not match or complete:
            return None
        found[match[1]].append({"pid": int(match[2]), "start": match[3]})
    return found if complete else None


def _process_command(config: HarnessConfig) -> str:
    return _encoded_command(GUEST_PROCESSES_SCRIPT, {"image": config.guest_executable.split("\\")[-1],
                                                     "script": GUEST_SCRIPT.name})


def _guest_processes(config: HarnessConfig, step: Callable[[str, str], dict[str, Any]]) -> dict[str, Any]:
    """Whether the guest runs a guest script of an ended run, or an instance of the tool, that this step did not start
    (it has started none yet): `clear`, or a refusal under its own name with the process ids
    (`guest_script_running`, `program_instance_running`), or `guest_processes_unknown` when the listing cannot be
    read. Read only; nothing is stopped."""
    try:
        record = step("guest_processes", _process_command(config))
    except ValueError as error:
        return {"status": GUEST_PROCESSES_UNKNOWN_ROUTE, "reason": f"the process listing could not be asked for ({error})"}
    listing = parse_guest_processes(record.get("stdout") or "") if record["exit_status"] == 0 else None
    if listing is None:
        return {"status": GUEST_PROCESSES_UNKNOWN_ROUTE,
                "reason": f"the guest's process listing could not be read (exit status {record['exit_status']})"}
    tools = sorted(item["pid"] for item in listing["tool"])
    scripts = sorted(item["pid"] for item in listing["script"])
    result = {"tool_pids": tools, "script_pids": scripts, "processes": listing}
    if scripts:
        return {**result, "status": GUEST_SCRIPT_RUNNING_ROUTE,
                "reason": f"a guest script of an ended run still runs in the guest (process {', '.join(map(str, scripts))})"}
    if tools:
        return {**result, "status": PROGRAM_INSTANCE_ROUTE,
                "reason": "an instance of the program that this step did not start runs in the guest (process "
                          f"{', '.join(map(str, tools))})"}
    return {**result, "status": "clear"}


def _visible_under(root: Path, name: str) -> list[str]:
    """Every place under the exchange folder where a file named `name` (compared case-insensitively, as the guest's
    file system names it) is visible, following directory links once each."""
    wanted = name.lower()
    found: list[str] = []
    seen: set[tuple[int, int]] = set()
    for directory, subdirectories, files in os.walk(root, followlinks=True):
        try:
            identity = os.stat(directory)
        except OSError:
            subdirectories[:] = []
            continue
        if (identity.st_dev, identity.st_ino) in seen:
            subdirectories[:] = []
            continue
        seen.add((identity.st_dev, identity.st_ino))
        found += [os.path.relpath(os.path.join(directory, item), root) for item in files if item.lower() == wanted]
    return sorted(found)


def _final_path(text: str) -> str:
    """A final path as GetFinalPathNameByHandle reports it, for comparison: without the `\\\\?\\` prefix, a UNC path
    as `\\\\server\\share...`, no trailing backslash, lower-case."""
    value = text.strip()
    if value.startswith("\\\\?\\UNC\\"):
        value = "\\\\" + value[8:]
    elif value.startswith("\\\\?\\"):
        value = value[4:]
    return value.rstrip("\\").lower()


def _fresh_work_dir(config: HarnessConfig, step: Callable[[str, str], dict[str, Any]], root: Path, work: str,
                    exchange_root: str, destinations: list[str]) -> dict[str, Any]:
    """Make the step's fresh work directory `work` and establish that it lies outside the exchange folder, before any
    guest command that deletes, copies or starts the program. First, before anything is created (`WORK_ROOT_SCRIPT`):

    - the configured work root, the folder `work` is to be created in, exists, and its final path, as the guest's file
      system resolves it (reparse points and links followed), lies outside the final path of the established
      exchange root; otherwise the step is refused and nothing is created.

    Only then (`WORK_DIR_SCRIPT`):

    - the directory is created exclusively (the guest's New-Item fails when the name exists), so nothing of an
      earlier step is in it;
    - its own final path lies outside the final path of the exchange root;
    - the content probe made in it is visible nowhere under the exchange folder on the host;
    - it holds nothing but that probe, and every destination the step directs the program to write (`destinations`,
      plain names directly in it) is absent.

    Where the work root's path looked separate and the probe then shows the directory inside the exchange folder,
    that directory and its probe stay there as new paths, named in the refusal by their paths relative to the exchange
    folder (`left_in_exchange_folder`); nothing removes them, and nothing stored changes.

    `separate`, or `inside` / `unverified` with the reason (both refused as `work_folder_on_exchange_drive`)."""
    probe = WORK_PROBE_NAME.format(token=secrets.token_hex(16))
    result: dict[str, Any] = {"work_directory": work, "probe": probe, "destinations": [_win(work, name) for name in
                                                                                      destinations]}
    if not all(NAME_PATTERN.match(name) for name in destinations):
        return {**result, "status": "unverified", "reason": "a destination is not a plain name in the work directory"}
    parent = work.rsplit("\\", 1)[0]
    try:
        resolved = step("work_root", _encoded_command(WORK_ROOT_SCRIPT, {"root": exchange_root, "path": parent}))
    except ValueError as error:
        return {**result, "status": "unverified", "reason": f"the work root could not be asked for ({error})"}
    found_lines = [line.rstrip() for line in (resolved.get("stdout") or "").splitlines() if line.strip()]
    if resolved["exit_status"] != 0 or "root resolved" not in found_lines:
        return {**result, "status": "unverified", "created": False,
                "reason": f"the guest did not resolve the work root's final path (exit status "
                          f"{resolved['exit_status']})"}
    found = {}
    for line in found_lines:
        key, _, value = line.partition(" ")
        found.setdefault(key, value)
    root_final, ancestor_final = _final_path(found.get("root", "")), _final_path(found.get("final", ""))
    result.update(ancestor=found.get("ancestor"), ancestor_final=ancestor_final, exchange_root_final=root_final)
    if found.get("ancestor") is None:
        return {**result, "status": "unverified", "created": False,
                "reason": f"the configured work root {parent} does not exist as a folder in the guest (make it, "
                          "outside the exchange folder); nothing was created"}
    if not root_final or not ancestor_final:
        return {**result, "status": "unverified", "created": False,
                "reason": f"the guest could not resolve the final path of the exchange root or of the work root "
                          f"{parent}; nothing was created"}
    if ancestor_final == root_final or ancestor_final.startswith(root_final + "\\"):
        return {**result, "status": "inside", "created": False,
                "reason": f"the work root {parent} resolves to {ancestor_final}, inside the exchange root "
                          f"{root_final}; nothing was created"}
    result["created"] = True
    try:
        record = step("work_directory", _encoded_command(WORK_DIR_SCRIPT, {
            "root": exchange_root, "dir": work, "probe": probe,
            "destinations": destinations}))
    except ValueError as error:
        return {**result, "status": "unverified", "reason": f"the work directory could not be asked for ({error})"}
    lines = [line.rstrip() for line in (record.get("stdout") or "").splitlines() if line.strip()]
    if record["exit_status"] != 0 or "work ready" not in lines:
        return {**result, "status": "unverified",
                "reason": f"the guest did not create a fresh work directory {work} (exit status "
                          f"{record['exit_status']}; a directory of that name may exist already)"}
    values = {}
    for line in lines:
        key, _, value = line.partition(" ")
        values.setdefault(key, []).append(value)
    root_final = _final_path((values.get("root") or [""])[0])
    dir_final = _final_path((values.get("dir") or [""])[0])
    entries = sorted(values.get("entry", []))
    present = sorted(values.get("destination_present", []))
    result.update(exchange_root_final=root_final, work_directory_final=dir_final, entries=entries)
    if not root_final or not dir_final:
        return {**result, "status": "unverified",
                "reason": "the guest could not resolve the final path of the exchange root or of the work directory"}
    if dir_final == root_final or dir_final.startswith(root_final + "\\"):
        return {**result, "status": "inside",
                "reason": f"the fresh work directory {work} resolves to {dir_final}, inside the exchange root "
                          f"{root_final}"}
    visible = _visible_under(root, probe)
    if visible:
        left = {"directory": str(Path(visible[0]).parent.as_posix()), "probe": Path(visible[0]).as_posix()}
        return {**result, "status": "inside", "visible_at": visible, "left_in_exchange_folder": left,
                "reason": f"the fresh work directory {work} resolves into the exchange folder (its probe is visible "
                          f"at {', '.join(visible)}); this step created the directory {left['directory']} and its "
                          f"probe {left['probe']} there, and they are left as they are: nothing removes them and "
                          "nothing stored was changed"}
    if entries != [probe] or present or sorted(values.get("destination_absent", [])) != sorted(destinations):
        return {**result, "status": "unverified",
                "reason": f"the fresh work directory {work} does not hold only this step's probe (entries "
                          f"{', '.join(entries) or 'none'}; destinations already there: {', '.join(present) or 'none'})"}
    return {**result, "status": "separate"}


def _before_guest_writes(config: HarnessConfig, step: Callable[[str, str], dict[str, Any]], root: Path, work: str,
                         exchange_root: str, destinations: list[str], record: dict[str, Any],
                         read: Callable[[str, str], dict[str, Any]] | None = None) -> tuple[str, str] | None:
    """The checks every live step makes after its exchange root is established and before any guest command copies
    anything or starts the program: no guest script of an ended run and no instance of the program it did not start
    runs (each under its own name), and the step's fresh work directory lies outside the exchange folder
    (`_fresh_work_dir`). (route, reason) of a refusal, or None."""
    processes = _guest_processes(config, read or step)  # with a settings guard, read with the guard's patience
    record["guest_processes"] = processes
    if processes["status"] != "clear":
        return processes["status"], f"{processes['reason']}; nothing is started while it runs"
    check = _fresh_work_dir(config, step, root, work, exchange_root, destinations)
    record["work_folder_check"] = check
    if check["status"] != "separate":
        return WORK_FOLDER_OVERLAP_ROUTE, (f"{check['reason']}; nothing is copied or started there")
    return None


def _collect(config: HarnessConfig, step: Callable[[str, str], dict[str, Any]], name: str, source: str,
             destination: str, names: list[str], patterns: list[str]) -> dict[str, Any]:
    """Collect files from the guest's work folder `source` into the step's new run folder `destination`
    (`COLLECT_SCRIPT`: each a create-new copy that never writes over a name already there, retries included). The
    outcome per file, and `problem` when the collection cannot be shown complete (a copy failed, a name was already
    there with other content, or the guest did not finish): such a run is recorded and never published."""
    record = step(name, _encoded_command(COLLECT_SCRIPT, {"purpose": name, "from": source, "to": destination,
                                                          "names": names, "patterns": patterns}))
    files: dict[str, str] = {}
    done = False
    for line in (record.get("stdout") or "").splitlines():
        parts = line.split(None, 1)
        if parts == ["collect", "done"] or line.strip() == "collect done":
            done = True
        elif len(parts) == 2 and parts[0] in ("copied", "absent", "failed", "present_same", "present_differs",
                                              "present_unreadable"):
            files[parts[1].strip()] = parts[0]
    bad = sorted(item for item, state in files.items() if state in ("failed", "present_differs", "present_unreadable"))
    problem = None
    if not done or record["exit_status"] != 0:
        problem = f"the guest did not finish the collection (exit status {record['exit_status']})"
    elif bad:
        problem = "not collected whole: " + ", ".join(f"{item} {files[item]}" for item in bad)
    return {"files": files, "complete": problem is None, "problem": problem}


def _write_settings_stage(run: CaseFolder, live: dict[str, Any], guard: "SettingsGuard",
                          snapshot: Mapping[str, Mapping]) -> None:
    """The stage record: the saved settings' before-state (`settings_before_state`), a new file in the run folder,
    written before the first guest command that can start the program or change a setting. A later step that finds it
    without a clean check record is stopped by name until a clearance for this run exists (`_settings_stop`)."""
    run.record(SETTINGS_STAGE, settings_before_state(guard, snapshot))
    live["settings_stage"] = True


def _write_settings_check(run: CaseFolder, settings: Mapping[str, Any] | None) -> None:
    """The check record, a new file in the run folder, once the settings check has finished: its status, whether
    the settings are restored or unchanged, and the names of the members that differ (names only)."""
    status = (settings or {}).get("status")
    differing = sorted({entry.get("value") or entry.get("key") for entry in
                        ((settings or {}).get("remaining_differences") or (settings or {}).get("changed_values") or [])
                        if isinstance(entry, Mapping)} - {None})
    run.record(SETTINGS_CHECK, {"status": status, "restored_or_unchanged": status in SETTINGS_KEEP_GOING,
                                "differing_members": [] if status in SETTINGS_KEEP_GOING else differing,
                                "written": _now()})


def _ensure_guest_script(run: CaseFolder, script_bytes: bytes) -> tuple[str, str]:
    """(name, outcome) of the harness's copy of its guest script, `_harness/guest-script-<sha256>.ps1`. When absent
    it is written whole in the step's run folder (`guest_script.ps1`) and then given that name with an exclusive
    link, so a crash never leaves a partial script under it; otherwise it is reused only when its content is exactly
    the script; never rewritten. ContainmentError when a file of that name holds anything else (it is left as it
    is)."""

    tree = run.tree
    name = GUEST_SCRIPT_NAME.format(sha256=_sha256(script_bytes))
    parts = (HARNESS_DIR, name)
    tree.ensure_dir((HARNESS_DIR,))
    existing = tree.read_bytes(parts)
    if existing is None and not tree.exists(parts):
        run.create(GUEST_SCRIPT_STAGED, script_bytes)
        try:
            tree.link_new((*run.parts, GUEST_SCRIPT_STAGED), parts)
            return name, "created"
        except ExistingPath:  # written meanwhile: verified like any existing copy
            existing = tree.read_bytes(parts)
    if existing == script_bytes:
        return name, "verified"
    raise ContainmentError(f"{HARNESS_DIR}/{name} is not the harness's guest script (it is not a regular file or "
                           "holds other content); it is left as it is and the step is refused")


def _route_of(resolution: Mapping[str, Any]) -> str:
    """The named refusal for an exchange root resolution that is not `ready`."""
    return {"ambiguous": EXCHANGE_AMBIGUOUS_ROUTE, "failed": EXCHANGE_CHECK_FAILED_ROUTE}.get(
        resolution["status"], EXCHANGE_UNAVAILABLE_ROUTE)


def _steps_view(steps: list[dict[str, Any]]) -> list[dict[str, Any]]:
    return [{key: value for key, value in item.items() if key not in ("stdout", "stderr")}
            | {"stdout_chars": len(item.get("stdout") or ""), "stderr_chars": len(item.get("stderr") or "")}
            | _failed_write_stderr(item) for item in steps]


# The run record's members DEL-17-05 REQ-004 and AC-002 name, present in every run record a live step writes
# (success, named refusal or exceptional end), with explicit not-attempted or null values where nothing happened.
RUN_RECORD_MEMBERS = ("document_kind", "schema_version", "deliverable", "case_id", "step", "run_folder",
                      "configuration_surface", "executable_path_as_configured", ACKNOWLEDGEMENT_KEY, "mbf",
                      "command_shape", "steps", "working_directory", "expected_batch_csv_path", "observed_csv_path",
                      "print_file", "route", "exit_status", "stdout_file", "stderr_file", "guest_log",
                      "outputs_collected", "outputs_sha256", "exchange_check", "input_check", "guest_script",
                      "execution_evidence", "timestamps", "boundary_note")
# A place in every run record for the digest of the headless runner's execution evidence; a later change fills it.
EXECUTION_EVIDENCE_PLACEHOLDER = {"headless_runner_evidence_sha256": None,
                                  "note": "reserved for the digest of the headless runner's execution evidence"}


def _base_record(config: HarnessConfig, case_id: str, run: CaseFolder, step_name: str, started: str) -> dict[str, Any]:
    """Every member of a run record, with not-attempted or null values; a step fills in what happened."""

    return {
        "document_kind": RUN_RECORD_KIND, "schema_version": "2", "deliverable": "DEL-17-05", "case_id": case_id,
        "step": step_name, "run_folder": run.relative,
        "configuration_surface": {"environment_variable": ENV_VAR, "config_file": CONFIG_NAME,
                                  "checked": list(config.checked), "warnings": list(config.warnings)},
        "executable_path_as_configured": config.guest_executable, ACKNOWLEDGEMENT_KEY: config.acknowledged,
        "mbf": None, "command_shape": {"attempted": False,
                                       "host_invocation": "prlctl exec <vm_name> --current-user cmd /c <guest command>"},
        "steps": [], "working_directory": None, "expected_batch_csv_path": None,
        "observed_csv_path": {"guest": None, "host": None}, "print_file": {"collected": None, "layout": None},
        "route": None, "exit_status": "not_attempted", "stdout_file": None, "stderr_file": None, "guest_log": None,
        "outputs_collected": [], "outputs_sha256": {}, "exchange_check": None, "input_check": None,
        "guest_script": {"repository_path": GUEST_SCRIPT_REPOSITORY_PATH, "exchange_copy": None, "sha256": None,
                         "run_from": None},
        "execution_evidence": dict(EXECUTION_EVIDENCE_PLACEHOLDER),
        # True for a capture-only step: its collected outputs are hashed and never read, by the harness or any reader.
        "sealed": False,
        "timestamps": {"started": started, "finished": None}, "boundary_note": BOUNDARY_NOTE,
    }


def _write_run_record(run: CaseFolder, record: dict[str, Any]) -> str:
    """The step's run record, a new file in its run folder (never rewritten); its path relative to the case folder."""
    record["timestamps"]["finished"] = record["timestamps"].get("finished") or _now()
    run.record("/".join(RUN_RECORD), record)
    return f"{run.relative}/{'/'.join(RUN_RECORD)}"


def _write_run_record_any(run: CaseFolder, record: dict[str, Any]) -> str:
    """A run record under the first name that can be created whole: `caepipe_out/run_record.json`, else (that name
    occupied, or a write there failed part-way) `caepipe_out/run_record_2.json`, `_3`, ... each created exclusively.
    A record under another name names why (`record_name`); such a run is never published (`run_problem` reads the
    first name only). The last failure is raised when no name can be written."""

    record["timestamps"]["finished"] = record["timestamps"].get("finished") or _now()
    try:
        run.record("/".join(RUN_RECORD), record)
        return f"{run.relative}/{'/'.join(RUN_RECORD)}"
    except Exception as first:
        failure: BaseException = first
    for number in range(2, 20):
        name = f"caepipe_out/run_record_{number}.json"
        try:
            run.record(name, {**record, "record_name": f"{name}: caepipe_out/run_record.json was occupied or could "
                                                       f"not be written whole ({type(failure).__name__}: {failure})"})
            return f"{run.relative}/{name}"
        except Exception as error:
            failure = error
    raise failure


# -- what makes a run or screen complete, and publishing it ------------------------------------------------------------
class SelectionError(ContainmentError):
    """A case's selections cannot be trusted (missing, corrupt, unreadable, conflicting, or naming an absent or
    incomplete run); the reader stops here and reads neither an older selection nor the case's own folders."""


def _read_record(path: Path) -> dict[str, Any] | None:
    try:
        if path.is_symlink() or not path.is_file():
            return None
        value = json.loads(path.read_text(encoding="utf-8"))
    except (OSError, UnicodeDecodeError, json.JSONDecodeError):
        return None
    return value if isinstance(value, dict) else None


def solve_ineligible(record: Mapping[str, Any] | None) -> str | None:
    """Why a solve run record did not end in an outcome eligible for selection, or None: a collected CAEPIPE CSV
    (`csv_from_batch` or `csv_from_gui_print`) of the run's own MBF, as the guest reported it, with no harness error,
    not sealed, and a clean settings check when one was made."""
    if not isinstance(record, Mapping):
        return "no run record"
    if outputs_sealed(record):
        return "its run record marks its outputs sealed; a sealed run is never published as current"
    if record.get("harness_error"):
        return "its run record names a harness error"
    if record.get("route") not in CSV_ROUTES:
        return f"its route {record.get('route')!r} is not an outcome eligible for selection"
    if record.get("input_hash_verified") is not True:
        return "the guest did not report solving the run's own MBF"
    settings = record.get("settings_guard")
    if settings is not None and (not isinstance(settings, Mapping) or settings.get("status") not in SETTINGS_KEEP_GOING):
        return "its settings check is not clean"
    return None


def run_problem(folder: Path, kind: str, *, also: Mapping[str, str] | None = None, case_id: str | None = None,
                relative: str | None = None, source: str | None = None) -> str | None:
    """Why the run or screen folder `folder` is not complete and valid for `kind`, or None.

    Every kind: its record names `case_id` and the folder `relative` (when given), and `also` ({path in the folder:
    SHA-256}, plain relative paths only) names files present with those hashes (a comparison, a manifest). `solve`: a
    well-formed solve run record whose guest reported solving the run's own MBF (host, guest and the file
    `caepipe_in/model.mbf` agree by SHA-256), an outcome eligible for selection (`solve_ineligible`), not sealed, and
    every output it hashes present with that hash. `export`: a well-formed export run record whose export route
    succeeded from a verified model, whose exported MBF is present with the hash recorded, naming as the solve it read
    `source`. `screen`: a screen record of this case naming as the run it read `source`. Malformed content of any
    kind is a problem, never an exception."""

    try:
        return _run_problem(folder, kind, also=also, case_id=case_id, relative=relative, source=source)
    except (OSError, TypeError, ValueError, AttributeError, KeyError, ContainmentError) as error:
        return f"it cannot be checked ({type(error).__name__}: {error})"


def _run_problem(folder: Path, kind: str, *, also: Mapping[str, str] | None, case_id: str | None,
                 relative: str | None, source: str | None) -> str | None:
    if kind not in ("solve", "export", "screen"):
        raise ValueError(f"{kind!r} is not a run kind")
    if folder.is_symlink() or not folder.is_dir():
        return "the folder is absent"
    problem = _verified_problem(also or {}) or _record_problem(folder, kind, case_id=case_id, relative=relative,
                                                                source=source)
    if problem:
        return problem
    # Only after the record (its sealed flag included) has been read: the files of `also`, at their hashes.
    for name, digest in (also or {}).items():
        path = folder.joinpath(*name.split("/"))
        if path.is_symlink() or not path.is_file() or _file_sha256(path) != digest:
            return f"{name} is absent or differs from the hash recorded for it"
    return None


def _record_problem(folder: Path, kind: str, *, case_id: str | None, relative: str | None,
                    source: str | None) -> str | None:
    """The folder's own record: a screen record, or a run record read (sealed flag first) before any output of it is
    opened to be hashed."""
    if kind == "screen":
        record = _read_record(folder / "screen_record.json")
        if record is None or record.get("document_kind") != SCREEN_RECORD_KIND or not isinstance(record.get("read"), dict):
            return "no well-formed screen record"
        if case_id is not None and record.get("case_id") != case_id:
            return f"its screen record names case {record.get('case_id')!r}, not {case_id!r}"
        if source is not None and _source_of(record["read"].get("run")) != source:
            return f"it screened {record['read'].get('run')!r}, not the selection's source {source!r}"
        return None
    record = _read_record(folder.joinpath(*RUN_RECORD))
    if record is None or record.get("document_kind") != RUN_RECORD_KIND:
        return "no well-formed run record"
    if case_id is not None and record.get("case_id") != case_id:
        return f"its run record names case {record.get('case_id')!r}, not {case_id!r}"
    if relative is not None and record.get("run_folder") != relative:
        return f"its run record names run folder {record.get('run_folder')!r}, not {relative!r}"
    if outputs_sealed(record):
        return "its run record marks its outputs sealed; a sealed run is never published as current"
    if record.get("harness_error"):
        return "its run record names a harness error"
    if kind == "solve":
        mbf = folder / "caepipe_in" / "model.mbf"
        digest = (record.get("mbf") or {}).get("sha256")
        if record.get("step") != "round_trip" or not isinstance(digest, str) or record.get("guest_mbf_sha256") != digest \
                or mbf.is_symlink() or not mbf.is_file() or _file_sha256(mbf) != digest:
            return "its run record does not bind the run's own MBF (host, guest and file hashes)"
        problem = solve_ineligible(record)
        if problem:
            return problem
        hashes = record.get("outputs_sha256")
        if not isinstance(hashes, dict) or sorted(hashes) != sorted(record.get("outputs_collected") or []):
            return "its run record does not hash every output it names"
        for name, value in hashes.items():
            if not isinstance(name, str) or not NAME_PATTERN.match(name):
                return f"its run record names an output {name!r} that is not a plain name"
            path = folder / "caepipe_out" / name
            if path.is_symlink() or not path.is_file() or _file_sha256(path) != value:
                return f"caepipe_out/{name} is absent or differs from the hash its run record records"
        return None
    entry = record.get("mbf_export") or {}
    exported = (entry.get("exported_mbf") or {})
    path = folder / "caepipe_out" / EXPORTED_MBF
    if record.get("step") != "mbf_export" or entry.get("route") != EXPORT_ROUTE \
            or entry.get("mod_hash_verified") is not True or not isinstance(record.get("solve_run"), dict) \
            or path.is_symlink() or not path.is_file() or _file_sha256(path) != exported.get("sha256"):
        return "its run record does not bind a successful export of a verified model to its exported MBF"
    if source is not None and _source_of(record["solve_run"].get("source")) != source:
        return f"it exported {record['solve_run'].get('source')!r}, not the selection's source {source!r}"
    return None


def publish(tree: Tree, case_id: str, *, reason: str, expect_run: Any = _UNCHANGED, also: Mapping[str, str] | None = None,
            **members: str | None) -> str:
    """Make a complete run, export or screen current. The folder named by the one member is checked first, and again
    against the current selection just before the new one is written (`run_problem`: this case and this folder, an
    outcome eligible for selection, not sealed, the files of `also` at their hashes, and, for an export or a screen,
    the source it read being the selection's source); only then is a new selection written (`write_selection`).
    ContainmentError, with nothing written, when the folder is not complete and valid, disagrees with the current
    selection, or the current source is no longer `expect_run`."""

    if len(members) != 1:
        raise ValueError("publish names exactly one run, export or screen")
    (member, relative), = members.items()
    kind = {"run": "solve", "export": "export", "screen": "screen"}[member]
    if not _folder_name(relative, SELECTION_MEMBERS[member]):
        raise ContainmentError(f"{relative!r} of {case_id} is not a {SELECTION_MEMBERS[member]} folder name")
    folder = tree.path(("cases", case_id, *relative.split("/")))

    def validate(current: Mapping[str, Any]) -> str | None:
        source = None if member == "run" else (_source_of(current.get("run")) or STORED_SOURCE)
        problem = run_problem(folder, kind, also=also, case_id=case_id, relative=relative, source=source)
        return f"{relative}: {problem}" if problem else None

    problem = run_problem(folder, kind, also=also, case_id=case_id, relative=relative)
    if problem:
        raise ContainmentError(f"{relative} of {case_id} is not published: {problem}")
    changes: dict[str, str | None] = {member: relative}
    if member == "run":  # a new solve: no earlier export or screen describes it
        changes.update(export=None, screen=None)
    written = write_selection(tree, case_id, stage=open_folder(tree, case_id, relative), reason=reason,
                              expect_run=expect_run, verified={member: dict(also or {})}, validate=validate,
                              **changes)
    if written is None:
        raise ContainmentError(f"{relative} of {case_id} is not published: the case's current source changed meanwhile")
    return written


def _guest_read(config: HarnessConfig, runner: Callable[..., subprocess.CompletedProcess], timeout_s: float,
                steps: list[dict[str, Any]], name: str, command: str, deadline: float | None = None) -> dict[str, Any]:
    """A guard read (`_guest_step`): up to `GUARD_READ_ATTEMPTS` attempts `GUARD_READ_BACKOFF_S` apart, all within
    `timeout_s` and `deadline`."""

    return _guest_step(config, runner, timeout_s, steps, name, command, deadline, attempts=GUARD_READ_ATTEMPTS,
                       backoff_s=GUARD_READ_BACKOFF_S, within_timeout=True)


def _backoff(seconds: float) -> None:
    time.sleep(seconds)


def _guest_step(config: HarnessConfig, runner: Callable[..., subprocess.CompletedProcess], timeout_s: float,
                steps: list[dict[str, Any]], name: str, command: str, deadline: float | None = None, *,
                attempts: int = PRLCTL_ATTEMPTS, backoff_s: float = 0.0,
                within_timeout: bool = False) -> dict[str, Any]:
    """Run one guest command through `prlctl exec`, append its record to `steps` and return it.

    A transport failure of the prlctl client (exit 255, `PrlJob_`) is retried, up to `attempts` in all, `backoff_s`
    apart. With `deadline` (a `time.monotonic()` value), and with `within_timeout` (all attempts together within
    `timeout_s`), no attempt runs past the limit: each attempt's timeout is cut to what remains, and no further
    attempt starts when the limit would be reached before it does; the last attempt's result is then returned.
    """

    argv = [config.prlctl, "exec", config.vm_name, "--current-user", "cmd", "/c", command]
    record: dict[str, Any] = {"name": name, "guest_command": command, "started": _now(), "attempts": 0}
    limit = deadline
    if within_timeout:
        call_end = time.monotonic() + timeout_s
        limit = call_end if limit is None else min(limit, call_end)
    # The prlctl client intermittently fails a call before or after the guest command with exit 255 and a
    # "PrlJob_..." message. Every step is idempotent (each clears its old outputs first), so retry.
    while True:
        record["attempts"] += 1
        attempt_timeout = timeout_s if limit is None else max(0.001, min(timeout_s, limit - time.monotonic()))
        try:
            completed = runner(argv, capture_output=True, text=True, check=False, timeout=attempt_timeout)
            record.update(exit_status=completed.returncode, stdout=completed.stdout or "", stderr=completed.stderr or "")
        except subprocess.TimeoutExpired as error:
            record.update(exit_status="timeout", stdout=_text(error.stdout), stderr=_text(error.stderr))
        except OSError as error:
            record.update(exit_status="launch_failed", stdout="", stderr=str(error))
        if not (record["exit_status"] == 255 and "PrlJob_" in record["stderr"]) or record["attempts"] >= attempts:
            break
        if limit is not None and time.monotonic() + backoff_s >= limit:
            record["attempts_stopped_by"] = "deadline"
            break
        if backoff_s:
            _backoff(backoff_s)
    record["finished"] = _now()
    steps.append(record)
    return record


class TerminationRequested(KeyboardInterrupt):
    """SIGTERM (or SIGBREAK) received during a guarded round trip, raised as a cancellation like Ctrl-C."""


class SectionBudgetExceeded(RuntimeError):
    """The restore's registry-mutation section used up its time budget (`section_budget_s`)."""


@contextlib.contextmanager
def _catchable_termination(active: bool, live: dict[str, Any] | None = None):
    """For a guarded round trip, make SIGTERM (and SIGBREAK where it exists) raise TerminationRequested instead of
    ending the process unrecorded; inside the mutation section `_deferred_signals` defers it as before. Each
    TerminationRequested carries the guard result as it stands when the signal arrives (`_live_guard_result`), so
    a termination request at any statement, a second one included, never leaves an exception without the write
    log. The previous handlers are restored on exit. Main thread only (elsewhere nothing changes)."""

    if not active or threading.current_thread() is not threading.main_thread():
        yield
        return
    previous: dict[int, Any] = {}

    def terminate(signum, frame):  # noqa: ARG001
        error = TerminationRequested(f"signal {signal.Signals(signum).name}")
        if live is not None and live.get("guard_state") is not None:
            try:
                _attach_guard(error, _live_guard_result(live))
            except Exception:  # the request is raised whatever happens; the round trip's recovery attaches again
                pass
        raise error

    try:
        for name in ("SIGTERM", "SIGBREAK"):
            number = getattr(signal, name, None)
            if number is not None:
                previous[number] = signal.signal(number, terminate)
        yield
    finally:
        for number, handler in previous.items():
            signal.signal(number, handler)


@contextlib.contextmanager
def _deferred_signals(active: bool):
    """Defer SIGINT, SIGTERM and (where it exists) SIGBREAK across the restore's registry-mutation section.

    While active, each such signal is recorded in `received` instead of interrupting, so neither a Ctrl-C nor a
    termination request can cut between a check and its write or leave a write half-logged; the previous handlers
    are restored on exit and the caller acts on what was received. The section is made only of guest calls, each
    with its own timeout, so the deferral is bounded. Python lets handlers be installed only on the main thread:
    elsewhere `deferral` says so and nothing is deferred.
    """

    state = SimpleNamespace(received=[], deferral="not needed")
    if not active:
        yield state
        return
    if threading.current_thread() is not threading.main_thread():
        state.deferral = "unavailable off the main thread"
        yield state
        return
    previous: dict[int, Any] = {}

    def record(signum, frame):  # noqa: ARG001 - the frame is not needed
        state.received.append(signum)

    try:
        for name in ("SIGINT", "SIGTERM", "SIGBREAK"):
            number = getattr(signal, name, None)
            if number is not None:
                previous[number] = signal.signal(number, record)
        state.deferral = "active"
        yield state
    finally:
        for number, handler in previous.items():
            signal.signal(number, handler)


def _secondary(settings: dict[str, Any] | None, where: str, error: BaseException) -> None:
    """Keep a later error beside the primary one in the guard result (never in place of it)."""
    if settings is not None:
        settings.setdefault("secondary_errors", []).append(f"{where}: {type(error).__name__}: {error}")


# What `reg add` and `reg delete` print on success; a write counts as completed only with exit 0 and this text.
REG_WRITE_OK = "The operation completed successfully"


# The stderr text a failed or unknown write keeps in the records, at most this many UTF-8 bytes (diagnostics only).
FAILED_WRITE_STDERR_CAP_BYTES = 2048


def _failed_write_stderr(item: Mapping[str, Any]) -> dict[str, Any]:
    """The capped stderr text of a mutating step that did not complete, for the records; empty for any other step.

    Only a write (`mutating`) whose outcome is not `completed` keeps text: no successful step's output and no
    stdout. The text is cut to `FAILED_WRITE_STDERR_CAP_BYTES` UTF-8 bytes at a character boundary;
    `stderr_truncated` says whether anything was cut. Nothing here affects any check, attempt or decision."""

    if not item.get("mutating") or item.get("outcome") == "completed":
        return {}
    raw = item.get("stderr") or ""
    data = (raw if isinstance(raw, str) else _text(raw)).encode("utf-8", errors="replace")
    return {"stderr_text": data[:FAILED_WRITE_STDERR_CAP_BYTES].decode("utf-8", errors="ignore"),
            "stderr_truncated": len(data) > FAILED_WRITE_STDERR_CAP_BYTES}


def _note_failed_write(state: dict[str, Any] | None, record: dict[str, Any]) -> None:
    """Keep a failed or unknown write's capped stderr in `state["failed_write_stderr"]`, which every guard result
    built from `state` carries (success, failure path, live result for an interruption)."""

    if state is not None:
        diagnostic = _failed_write_stderr(record)
        if diagnostic:
            state.setdefault("failed_write_stderr", []).append(
                {"step": record["name"], "exit_status": record["exit_status"],
                 "outcome": record.get("outcome", "not_returned"),
                 "stderr_chars": len(record.get("stderr") or "")} | diagnostic)


class RestoreWriteFailed(RuntimeError):
    """A registry write of the restore did not complete: `outcome` is `failed` (the command reported failure) or
    `unknown` (the transport failed, or the output was not the expected one; the write may or may not have taken)."""

    def __init__(self, name: str, outcome: str, exit_status: Any):
        super().__init__(f"{name}: write {outcome} (exit status {exit_status})")
        self.outcome = outcome
        self.exit_status = exit_status


def _guest_write(config: HarnessConfig, runner: Callable[..., subprocess.CompletedProcess], timeout_s: float,
                 steps: list[dict[str, Any]], name: str, command: str,
                 state: dict[str, Any] | None = None, deadline: float | None = None) -> dict[str, Any]:
    """Run one mutating guest command exactly once: this call never retries, whatever the transport reports (the
    restore may issue a value set once more, only after settling its outcome; `_restore_recent_files`).

    The step is recorded before anything is raised. Exit 0 with `REG_WRITE_OK` in its output is the only success.
    Exit 255 with a `PrlJob_` message (the client's transport failure) or exit 0 with other output is `unknown`; any
    other exit is `failed`; both raise RestoreWriteFailed. An exception from the runner (a timeout, a launch
    failure, cancellation) is stored as `state["primary_error"]` before anything else (unless one is already
    there), then recorded under a guard whose own failure is kept as secondary, and re-raised unchanged.
    """

    argv = [config.prlctl, "exec", config.vm_name, "--current-user", "cmd", "/c", command]
    # Complete from the start, so the step stays recordable however its own logging ends.
    record: dict[str, Any] = {"name": name, "guest_command": command, "started": _now(), "attempts": 1,
                              "mutating": True, "exit_status": "not_returned", "stdout": "", "stderr": ""}
    if deadline is not None:
        timeout_s = max(0.001, min(timeout_s, deadline - time.monotonic()))
    try:
        completed = runner(argv, capture_output=True, text=True, check=False, timeout=timeout_s)
    except BaseException as error:
        if state is not None:
            state.setdefault("primary_error", error)
        try:
            record.update(exit_status=type(error).__name__, stdout=_text(getattr(error, "stdout", None)),
                          stderr=_text(getattr(error, "stderr", None)) or str(error), finished=_now())
            _note_failed_write(state, record)
        except BaseException as late:
            if state is not None:
                state.setdefault("secondary_errors", []).append(late)
        finally:
            steps.append(record)
        raise error
    record.update(exit_status=completed.returncode, stdout=completed.stdout or "", stderr=completed.stderr or "",
                  finished=_now())
    steps.append(record)
    if completed.returncode == 0 and REG_WRITE_OK.lower() in record["stdout"].lower():
        record["outcome"] = "completed"
        return record
    transport = completed.returncode == 255 and "PrlJob_" in record["stderr"]
    record["outcome"] = "unknown" if transport or completed.returncode == 0 else "failed"
    _note_failed_write(state, record)
    raise RestoreWriteFailed(name, record["outcome"], completed.returncode)


def _print_layout_problem(path: Path) -> str | None:
    """Why a print file is not the documented print-CSV layout, or None when it is: the strict print parser must
    recognise its header, parse at least one section and report no error."""

    from .print_csv import read_caepipe_print_csv  # the package imports this module; resolved at call time

    try:
        parsed = read_caepipe_print_csv(path)
    except Exception as error:  # an unreadable file is not the layout
        return f"unreadable: {type(error).__name__}"
    codes = [item["code"] for item in parsed["diagnostics"]
             if item["severity"] == "error" or item["code"] == "CAEPIPE-PRINT-HEADER-UNRECOGNIZED"]
    if codes:
        return "not the print-CSV layout: " + ", ".join(sorted(set(codes)))
    if not parsed["sections_parsed"]:
        return "not the print-CSV layout: no section recognised"
    return None


def _transport_failure(error: BaseException) -> bool:
    """A write whose outcome the transport left unknown: the prlctl client's exit 255 (`PrlJob_`), or a timeout."""
    return (isinstance(error, RestoreWriteFailed) and error.outcome == "unknown" and error.exit_status == 255) \
        or isinstance(error, subprocess.TimeoutExpired)


# What `reg query` reports, exit 1, for a key or value that does not exist; any other failure is unreadable.
REG_NOT_FOUND = "unable to find the specified registry key or value"


def _not_found(record: Mapping[str, Any]) -> bool:
    return record["exit_status"] == 1 and REG_NOT_FOUND in (record["stdout"] + record["stderr"]).lower()


def _read_value(step: Callable[..., dict[str, Any]], name: str, key: str,
                value_name: str, command: str | None = None) -> tuple[str, tuple[str, str] | None]:
    """One value read back alone (`reg query <key> /v <name>`, or `command` for the literal value, which prints the
    same): ("present", (type, data)), ("absent", None) when reg reports that it does not exist, or ("unreadable",
    None) when the read fails otherwise, does not parse whole, or does not show exactly that one value."""

    record = step(name, command or f"reg query {_quote(key)} /v {_quote(value_name)}")
    if _not_found(record):
        return "absent", None
    parsed = parse_reg_query(record["stdout"]) if record["exit_status"] == 0 else None
    if parsed is None or list(parsed["values"]) != [(_registry_key_id(key), value_name)]:
        return "unreadable", None
    found = parsed["values"][(_registry_key_id(key), value_name)]
    return "present", (found[1], found[2])


def _read_subtree(step: Callable[..., dict[str, Any]], name: str,
                  key: str) -> tuple[str, dict[tuple[str, str], tuple[str, str]] | None]:
    """A subkey read back whole (`reg query <key> /s`): ("present", {(key id, value name): (type, data)}) for
    every value in it and below, ("absent", None) when reg reports that it does not exist, or ("unreadable",
    None) otherwise (a failed read, output that does not parse whole, or a key printed outside it). A present
    subkey without values prints nothing."""

    record = step(name, f"reg query {_quote(key)} /s")
    if _not_found(record):
        return "absent", None
    if record["exit_status"] != 0:
        return "unreadable", None
    if not record["stdout"].strip():
        return "present", {}
    parsed = parse_reg_query(record["stdout"])
    key_id = _registry_key_id(key)
    if parsed is None or not all(found == key_id or found.startswith(key_id + "\\") for found in parsed["keys"]):
        return "unreadable", None
    return "present", {item: value[1:] for item, value in parsed["values"].items()}


def _write_outcome(error: BaseException) -> str:
    """The write log's outcome for a write that raised: a launch failure never reached the guest."""
    if isinstance(error, RestoreWriteFailed):
        return error.outcome
    if isinstance(error, OSError):
        return "failed"
    return "unknown"


def _registry_key_id(key: str) -> str:
    """A registry key for comparison: lower-cased, with `HKEY_CURRENT_USER` (as `reg query` prints it) as `HKCU`."""

    key = key.strip().lower()
    return "hkcu" + key[len("hkey_current_user"):] if key.startswith("hkey_current_user\\") else key


def parse_reg_query(text: str) -> dict[str, dict] | None:
    """`reg query <key> /s` output as a whole snapshot: {"keys": {key id: key as printed}, "values": {(key id,
    value name): (key as printed, type, data)}} (`_registry_key_id`). Every key is kept, empty ones included.

    None when the output names no key, or has a line that is neither blank, a key nor a value line: a reading
    that is not understood whole is no reading.
    """

    keys: dict[str, str] = {}
    values: dict[tuple[str, str], tuple[str, str, str]] = {}
    current = None
    for line in text.splitlines():
        line = line.rstrip("\r")
        if not line.strip():
            continue
        if line.startswith("HKEY_"):
            current = line.strip()
            keys[_registry_key_id(current)] = current
            continue
        match = REG_VALUE_LINE.match(line)
        if not match or current is None:
            return None
        values[(_registry_key_id(current), match["name"])] = (current, match["type"], match["data"] or "")
    return {"keys": keys, "values": values} if keys else None


def _settings_snapshot(config: HarnessConfig, step: Callable[[str, str], dict[str, Any]],
                       name: str) -> dict[str, dict] | None:
    """Read the whole configured settings key, every subkey included, through one guest step.

    None unless the step succeeded, its output parses whole and every key it prints is the configured key or lies
    under it. `reg query /s` prints no header for the queried key when that key holds no values of its own (only
    its subkeys); a successful query that prints subkeys of it shows that it exists, so it is then recorded as
    printed in `HKEY_CURRENT_USER` form. An output that prints no key at all is no reading.
    """

    record = step(name, f"reg query {_quote(config.settings_guard.key)} /s")
    snapshot = parse_reg_query(record["stdout"]) if record["exit_status"] == 0 else None
    root = _registry_key_id(config.settings_guard.key)
    if snapshot is None or not all(key == root or key.startswith(root + "\\") for key in snapshot["keys"]):
        return None
    snapshot["keys"].setdefault(root, "HKEY_CURRENT_USER" + config.settings_guard.key[len("HKCU"):])
    return snapshot


def restore_scope_id(guard: "SettingsGuard | None") -> str | None:
    """The key id whose values (strictly inside it, with `restore_scope` "key"; its own values, with
    `restore_recent_files`) the guard is configured to write back, or None when it writes nothing back. Values are
    recorded only for members inside this scope; of every other member only its name, kind and whether it changed."""
    if guard is None:
        return None
    if guard.restores_whole_key:
        return _registry_key_id(guard.key)
    if guard.restore_recent_files and guard.recent_files_key:
        return _registry_key_id(guard.recent_files_key)
    return None


def _in_scope(key_id: str, scope_id: str | None, whole_key: bool) -> bool:
    if scope_id is None:
        return False
    return key_id == scope_id or (whole_key and key_id.startswith(scope_id + "\\"))


def settings_difference(before: Mapping[str, Mapping], after: Mapping[str, Mapping], *,
                        scope_id: str | None = None, whole_key: bool = False) -> list[dict[str, Any]]:
    """Every difference between two whole snapshots: a key added or removed (empty ones included), and a value
    added, removed or changed in type or data. A value's before and after type and data are given only inside the
    restore scope (`restore_scope_id`); outside it, only its name, `outside_restore_scope` and whether it was
    present before and after."""

    changed: list[dict[str, Any]] = []
    for key in sorted(set(before["keys"]) | set(after["keys"])):
        if (key in before["keys"]) != (key in after["keys"]):
            changed.append({"key": (after["keys"].get(key) or before["keys"][key]),
                            "before": "present" if key in before["keys"] else None,
                            "after": "present" if key in after["keys"] else None})
    for item in sorted(set(before["values"]) | set(after["values"])):
        old, new = before["values"].get(item), after["values"].get(item)
        if (old or (None, None, None))[1:] != (new or (None, None, None))[1:]:
            if _in_scope(item[0], scope_id, whole_key):
                changed.append({"value": f"{(new or old)[0]}\\{item[1]}",
                                "before": {"type": old[1], "data": old[2]} if old else None,
                                "after": {"type": new[1], "data": new[2]} if new else None})
            else:
                changed.append({"value": f"{(new or old)[0]}\\{item[1]}", "outside_restore_scope": True,
                                "before": "present" if old else None, "after": "present" if new else None})
    return changed


def settings_before_state(guard: "SettingsGuard", snapshot: Mapping[str, Mapping]) -> dict[str, Any]:
    """The before-state record of a guarded step (the stage record): every key, and every value by key, name, kind and
    SHA-256 of its data; the data itself only inside the restore scope (`restore_scope_id`). It is written only to the
    step's run folder; nothing in it (no value and no hash) is copied into a report."""
    scope_id = restore_scope_id(guard)
    members = []
    for (key_id, name), (printed, kind, data) in sorted(snapshot["values"].items()):
        member = {"key": printed, "name": name, "type": kind, "sha256": _sha256(data.encode("utf-8"))}
        if _in_scope(key_id, scope_id, guard.restores_whole_key):
            member["data"] = data
        members.append(member)
    return {"document_kind": "openpipestress.caepipe_settings_before_state", "settings_key": guard.key,
            "restore_scope": {"scope": guard.restore_scope, "restore_recent_files": guard.restore_recent_files,
                              "values_recorded_for": scope_id},
            "keys": sorted(snapshot["keys"].values()), "members": members, "written": _now(),
            "note": "values only for members the guard may write back; never copied out of this run folder"}


def _unknown(reason: str, **extra: Any) -> dict[str, Any]:
    return {"status": "unknown", "reason": reason, **extra}


def _tool_processes(config: HarnessConfig, step: Callable[[str, str], dict[str, Any]],
                    name: str) -> list[dict[str, Any]] | None:
    """Every process of the tool's image in the guest, each {pid, start} (`parse_guest_processes`), or None when the
    listing cannot be read whole."""

    listing = step(name, _process_command(config))
    if listing["exit_status"] != 0:
        return None
    parsed = parse_guest_processes(listing.get("stdout") or "")
    return None if parsed is None else sorted(parsed["tool"], key=lambda item: (item["pid"], item["start"]))


def _precondition(config: HarnessConfig, step: Callable[[str, str], dict[str, Any]], name: str) -> dict[str, Any]:
    """Before anything starts the tool: no instance of it may be running (none is ours yet). Read only."""

    try:
        found = _tool_processes(config, step, name)
    except Exception as failure:
        return {"status": "precondition_unknown", "reason": f"the process check failed: {type(failure).__name__}"}
    if found is None:
        return {"status": "precondition_unknown", "reason": "the guest's process list could not be read or is "
                                                            "malformed"}
    if found:
        return {"status": "foreign_instance_running", "foreign_pids": [item["pid"] for item in found],
                "foreign": found, "reason": "an instance of the tool this run did not start is running; nothing ran"}
    return {"status": "clear"}


def check_no_foreign_instance(config: HarnessConfig, *, timeout_s: float | None = None,
                              runner: Callable[..., subprocess.CompletedProcess] = subprocess.run) -> dict[str, Any]:
    """Session precondition: `clear` when no instance of the tool is running in the guest. Starts and stops nothing.
    Each attempt has the guard's call timeout (`call_timeout_s`, default `GUARD_CALL_TIMEOUT_S`)."""

    if timeout_s is None:
        timeout_s = config.settings_guard.call_timeout_s if config.settings_guard else GUARD_CALL_TIMEOUT_S
    steps: list[dict[str, Any]] = []
    return _precondition(config, lambda name, command: _guest_read(config, runner, timeout_s, steps, name, command),
                         "session_process_list")


def check_guest_alive(config: HarnessConfig, *, timeout_s: float = GUARD_CALL_TIMEOUT_S,
                      runner: Callable[..., subprocess.CompletedProcess] = subprocess.run) -> dict[str, Any]:
    """Liveness check: `alive` when the guest answers `cmd /c ver` with exit status 0 and some output. Starts and
    stops nothing; the guest's output is not kept."""

    steps: list[dict[str, Any]] = []
    try:
        record = _guest_step(config, runner, timeout_s, steps, "liveness", "ver")
    except Exception as failure:
        return {"status": "unreachable", "reason": f"the liveness call failed: {type(failure).__name__}"}
    if record["exit_status"] != 0 or not record["stdout"].strip():
        return {"status": "unreachable",
                "reason": f"the guest did not answer the liveness check (exit status {record['exit_status']})"}
    return {"status": "alive"}


def _owned_processes(config: HarnessConfig, step: Callable[[str, str], dict[str, Any]], work: str,
                     guest_log: Any) -> dict[str, Any]:
    """Whether every process the guest script started has exited, and no other instance of the tool appeared.

    The guest script appends the id and start time of each process it starts to `OWNED_PIDS_FILE` in the work folder
    (and lists them in its report as `owned_pids`, each {pid, start}). The record must be complete and valid:
    readable, at least one process (every round trip that reaches the guest script starts the tool), one `<id>
    <start>` per line, and equal to the report's list when there is a report. A process is ours only by id and start
    time together. The process list must be read whole (`parse_guest_processes`). No process is
    stopped or signalled here. `exited` when no instance of the tool is listed; `running` when one of ours is;
    `foreign` when one that is not ours is (the precondition checked again); `unknown` whenever the record or the
    list falls short.
    """

    ids = step("owned_pids", f"type {_quote(_win(work, OWNED_PIDS_FILE))}")
    if ids["exit_status"] != 0:
        return _unknown("the record of the processes the guest started could not be read")
    lines = [line.strip() for line in ids["stdout"].splitlines() if line.strip()]
    if not lines or not all(re.fullmatch(r"[0-9]{1,10} \S+", line) for line in lines):
        return _unknown("the record of the processes the guest started is empty or malformed")
    owned = sorted({(int(line.split()[0]), line.split()[1]) for line in lines})
    pids = [pid for pid, _ in owned]
    if isinstance(guest_log, dict):
        reported = guest_log.get("owned_pids")
        if not (isinstance(reported, list) and all(isinstance(item, dict) and isinstance(item.get("pid"), int)
                                                   and isinstance(item.get("start"), str) for item in reported)
                and sorted({(item["pid"], item["start"]) for item in reported}) == owned):
            return _unknown("the guest's report and its record of the processes it started disagree", owned_pids=pids)
    found = _tool_processes(config, step, "process_list")
    if found is None:
        return _unknown("the guest's process list could not be read or is malformed", owned_pids=pids)
    # Ours only by id and start time together: a recorded id with another start time is another process.
    running = [item["pid"] for item in found if (item["pid"], item["start"]) in owned]
    foreign = [item["pid"] for item in found if (item["pid"], item["start"]) not in owned]
    if running:
        return {"status": "running", "owned_pids": pids, "running": running, "foreign_pids": foreign}
    if foreign:
        return {"status": "foreign", "owned_pids": pids, "foreign_pids": foreign}
    return {"status": "exited", "owned_pids": pids}


def _restore_plan(before: Mapping[str, Mapping], after: Mapping[str, Mapping], scope_id: str, *,
                  whole_key: bool = False, literal: tuple[str, str] | None = None) -> list[dict[str, Any]]:
    """The writes that take the restore scope back to `before`.

    The scope is the key `scope_id`: its own values alone, or (`whole_key`) its values and everything strictly
    inside it. Subkeys that were removed are created first (shallowest first); then values that changed or were
    removed are set to their first-reading type and data, in the first reading's order, and values that appeared
    are deleted (except inside a subkey that appeared); last, subkeys that appeared are deleted (deepest first).
    `scope_id` itself is never created or deleted.

    ValueError (nothing is written) when any difference lies outside the scope, when a key or value name falls
    outside the allowed characters, or when a value cannot be written back exactly. The one exception is `literal`
    (`_literal_recent_value`): that exact (key id, value name), as REG_BINARY, is set or deleted through the
    registry API (`_literal_value_command`), and its entry carries the command that reads it back.
    """

    def inside(key_id: str, *, strictly: bool = False) -> bool:
        if not whole_key:
            return key_id == scope_id and not strictly
        return key_id.startswith(scope_id + "\\") or (key_id == scope_id and not strictly)

    def printed_key(printed: str, key_id: str) -> str:
        if not REG_PRINTED_KEY_PATTERN.match(printed) or _registry_key_id(printed) != key_id:
            raise ValueError(f"{printed} cannot be named in a guest command")
        return printed

    differing_values = [item for item in set(before["values"]) | set(after["values"])
                        if before["values"].get(item, (None,))[1:] != after["values"].get(item, (None,))[1:]]
    differing_keys = set(before["keys"]) ^ set(after["keys"])
    # Containment first: any difference outside the scope refuses the whole restore before any write.
    for key_id in differing_keys:
        if not inside(key_id, strictly=True):
            raise ValueError(f"{(before['keys'].get(key_id) or after['keys'].get(key_id))} lies outside the restore "
                             "scope")
    for item in differing_values:
        if not inside(item[0]):
            raise ValueError(f"{(before['values'].get(item) or after['values'][item])[0]}\\{item[1]} lies outside "
                             "the restore scope")
    # The literal value, wherever it differs (a subkey that appeared included), must be REG_BINARY in every reading
    # that has it; otherwise the whole restore is refused before any write.
    if literal in differing_values:
        for reading in (before, after):
            found = reading["values"].get(literal)
            if found is not None and found[1] != "REG_BINARY":
                raise ValueError(f"{found[0]}\\{literal[1]} ({found[1]}) cannot be written back exactly")
    added = sorted((key for key in after["keys"] if key not in before["keys"]), key=lambda key: -key.count("\\"))
    removed = sorted((key for key in before["keys"] if key not in after["keys"]), key=lambda key: key.count("\\"))

    plan = []
    for key_id in removed:
        printed = printed_key(before["keys"][key_id], key_id)
        plan.append({"key": printed, "action": "create_key", "restores": "present", "replaces": None,
                     "command": f"reg add {_quote(printed)} /f"})
    for item, (printed, kind, data) in before["values"].items():
        if item in differing_values:
            found = after["values"].get(item)
            # Both readings of the literal value must be REG_BINARY where present; any other type keeps the refusal.
            exact = item == literal and kind == "REG_BINARY" and (found is None or found[1] == "REG_BINARY")
            if kind not in RESTORABLE_DATA or not RESTORABLE_DATA[kind].match(data) or item[1] == "(Default)" \
                    or not (REG_VALUE_NAME_PATTERN.match(item[1]) or exact):
                raise ValueError(f"{printed}\\{item[1]} ({kind}) cannot be written back exactly")
            printed = printed_key(printed, item[0])
            entry = {"value": f"{printed}\\{item[1]}", "action": "set", "target": (printed, item[1]),
                     "restores": {"type": kind, "data": data},
                     "replaces": {"type": found[1], "data": found[2]} if found else None,
                     "command": f"reg add {_quote(printed)} /v {_quote(item[1])} /t {kind} /d {_quote(data)} /f"
                     if not exact else _literal_value_command(printed, item[1], "set", data.upper())}
            if exact:
                entry["readback_command"] = _literal_value_command(printed, item[1], "read")
            plan.append(entry)
    for item, (printed, kind, data) in after["values"].items():
        if item not in before["values"] and item[0] not in added:
            exact = item == literal and kind == "REG_BINARY"
            if item[1] == "(Default)" or not (REG_VALUE_NAME_PATTERN.match(item[1]) or exact):
                raise ValueError(f"{printed}\\{item[1]} cannot be deleted exactly")
            printed = printed_key(printed, item[0])
            entry = {"value": f"{printed}\\{item[1]}", "action": "delete", "target": (printed, item[1]),
                     "restores": None,
                     "replaces": {"type": kind, "data": data},
                     "command": f"reg delete {_quote(printed)} /v {_quote(item[1])} /f"
                     if not exact else _literal_value_command(printed, item[1], "delete")}
            if exact:
                entry["readback_command"] = _literal_value_command(printed, item[1], "read")
            plan.append(entry)
    for key_id in added:
        printed = printed_key(after["keys"][key_id], key_id)
        subtree = {item: value[1:] for item, value in after["values"].items()
                   if item[0] == key_id or item[0].startswith(key_id + "\\")}
        plan.append({"key": printed, "action": "delete_key", "subtree": subtree, "restores": None,
                     "replaces": "present",
                     "command": f"reg delete {_quote(printed)} /f"})
    return plan


class _RestoreStopped(Exception):
    """A check inside the restore found another instance of the tool, or could not tell."""

    def __init__(self, check: dict[str, Any]):
        super().__init__(check["status"])
        self.check = check


def _restore_recent_files(config: HarnessConfig, step: Callable[[str, str], dict[str, Any]],
                          write: Callable[[str, str], dict[str, Any]], settings_before, plan: list[dict[str, Any]],
                          state: dict[str, Any]) -> tuple[dict[str, Any], BaseException | None]:
    """Write the restore scope (`_restore_plan`) back to the first reading, as one routine with one failure path.

    Steps: the precondition before each write; the write, logged in `state["write_log"]` (value, action, the value
    it restores and the one it replaces, attempts, outcome) before it is issued, and issued exactly once through
    `write`; the precondition after the last write; the third reading and its exact comparison with the first; the
    precondition once more before success is declared. `recent_files_restored` only when every step completed and
    the comparison found no difference. Any other outcome takes the one failure path: a check that is not clear
    (no exception), or any exception or cancellation at any step. The first exception is kept in
    `state["primary_error"]` from the moment it is caught, so nothing later can replace it.
    """

    state["restore_entered"] = True
    log: list[dict[str, Any]] = state.setdefault("write_log", [])
    reason = None
    guard = config.settings_guard
    deadline = None  # set when the mutation section starts

    def within(name: str) -> None:
        if deadline is not None and time.monotonic() >= deadline:
            state["section_budget_exceeded"] = name
            raise SectionBudgetExceeded(f"the section budget of {guard.section_budget_s:g} s ran out before {name}")

    def budgeted_step(name: str, command: str) -> dict[str, Any]:
        within(name)
        return step(name, command, deadline=deadline)

    def budgeted_write(name: str, command: str) -> dict[str, Any]:
        within(name)
        return write(name, command, deadline=deadline)

    def check(name: str, caller: Callable[..., dict[str, Any]] | None = None) -> None:
        # Not `_precondition`: an exception here must reach the failure path unchanged, not become a finding.
        found = _tool_processes(config, caller or step, name)
        if found is None:
            raise _RestoreStopped({"status": "precondition_unknown",
                                   "reason": "the guest's process list could not be read or is malformed"})
        if found:
            raise _RestoreStopped({"status": "foreign_instance_running", "foreign_pids": found,
                                   "reason": "an instance of the tool this run did not start is running"})

    def settled_write(number: int, entry: dict[str, Any], logged: dict[str, Any]) -> None:
        """Issue one write. A value set, value delete or subkey delete whose outcome the transport left unknown
        (exit 255, timeout) is settled before anything is issued again: the foreign-instance check, then the value
        or subkey read back. Already as intended (the value set to its target; the value or subkey absent):
        completed. Still as the run left it (the value it replaces; for a subkey, every value in it as read after
        the run): issued once more (attempt 2). Anything else (another value or content, an unreadable read-back,
        another instance): the first error stops the restore. At most two attempts; every attempt and the
        read-back are logged. Creating a subkey is not retried."""

        name = f"settings_restore_{number}"
        try:
            budgeted_write(name, entry["command"])
            return
        except BaseException as error:
            if entry["action"] not in ("set", "delete", "delete_key") or not _transport_failure(error):
                raise
            first = error
        if state.get("primary_error") is first:  # a timeout the write recorded: not yet the restore's error
            del state["primary_error"]
        logged["attempt_log"] = [{"attempt": 1, "outcome": "unknown", "error": f"{type(first).__name__}: {first}"}]
        check(f"restore_check_before_retry_{number}", budgeted_step)
        within(f"settings_readback_{number}")
        if entry["action"] == "delete_key":
            state_found, found = _read_subtree(budgeted_step, f"settings_readback_{number}", entry["key"])
            intended = state_found == "absent"
            as_left = state_found == "present" and found == entry["subtree"]
        else:
            state_found, found = _read_value(budgeted_step, f"settings_readback_{number}", *entry["target"],
                                             command=entry.get("readback_command"))
            target = (entry["restores"]["type"], entry["restores"]["data"]) if entry["restores"] else None
            left = (entry["replaces"]["type"], entry["replaces"]["data"]) if entry["replaces"] else None
            intended = (state_found == "present" and found == target) or (state_found == "absent" and target is None)
            as_left = (state_found == "present" and found == left) or (state_found == "absent" and left is None)
        if intended:
            logged["readback"] = "target"
            logged["attempt_log"][0]["outcome"] = "completed (read back)"
            return
        if not as_left:
            logged["readback"] = "unreadable" if state_found == "unreadable" else "third_value"
            raise first
        logged["readback"] = "unchanged"
        logged["attempts"] = 2
        logged["attempt_log"].append({"attempt": 2, "outcome": "issued"})
        try:
            budgeted_write(f"{name}_attempt_2", entry["command"])
        except BaseException as error:
            logged["attempt_log"][1]["outcome"] = _write_outcome(error)
            raise
        logged["attempt_log"][1]["outcome"] = "completed"

    try:
        # The registry-mutation section: the checks, the writes, the check after them and the verification
        # reading, with signals deferred. Its bound is the sum of these guest calls' timeouts.
        signals = None
        try:
            with _deferred_signals(True) as signals:
                deadline = time.monotonic() + guard.section_budget_s
                state["section_budget_s"] = guard.section_budget_s
                for number, entry in enumerate(plan, 1):
                    check(f"restore_check_before_write_{number}", budgeted_step)
                    logged = {key: entry[key] for key in ("value", "key", "action", "restores", "replaces")
                              if key in entry} | {
                        "attempts": 1, "outcome": "issued", "completed": False}
                    log.append(logged)
                    try:
                        settled_write(number, entry, logged)
                    except _RestoreStopped:
                        logged["outcome"] = "unknown"  # the write's outcome was never settled
                        raise
                    except BaseException as error:
                        state.setdefault("primary_error", error)
                        logged["outcome"] = _write_outcome(error)
                        raise
                    logged["outcome"], logged["completed"] = "completed", True
                check("restore_check_after_writes", budgeted_step)
                within("settings_verify")
                third = _settings_snapshot(config, budgeted_step, "settings_verify")
        finally:
            # The section has closed (handlers restored): what it deferred is reported now, before any record.
            if signals is not None:
                state["signal_deferral"] = signals.deferral
                if signals.received:
                    state["deferred_signals"] = [signal.Signals(number).name for number in signals.received]
        if third is None:
            reason = "the key could not be read after the restore"
        else:
            remaining = settings_difference(settings_before, third, scope_id=restore_scope_id(guard),
                                            whole_key=guard.restores_whole_key)
            if remaining:
                reason = "the reading after the restore does not equal the reading before the run"
                state["remaining_differences"] = remaining
        if reason is None:
            check("restore_check_before_success")
            done = {"status": "settings_restored" if guard.restores_whole_key else "recent_files_restored",
                    "write_log": log, "restored_values": len(plan)}
            for key in ("signal_deferral", "deferred_signals", "failed_write_stderr"):
                if key in state:
                    done[key] = state[key]
            return done, None
    except _RestoreStopped as stopped:
        reason = f"{stopped.check['status']}: {stopped.check.get('reason', '')}"
        state["stopped_by"] = stopped.check
    except BaseException as caught:  # cancellation included
        state.setdefault("primary_error", caught)
        reason = f"the restore was interrupted ({type(caught).__name__}: {caught})"
    state["reason"] = reason
    return _failed_restore(config, step, settings_before, state)


def _restore_log_summary(result: dict[str, Any], state: dict[str, Any]) -> None:
    log = state.get("write_log", [])
    result["write_log"] = log
    result["writes_completed"] = sum(1 for entry in log if entry["completed"])
    for key in ("stopped_by", "remaining_differences", "signal_deferral", "deferred_signals", "section_budget_s",
                "section_budget_exceeded", "failed_write_stderr"):
        if key in state:
            result[key] = state[key]


def _restore_reason(result: dict[str, Any], state: dict[str, Any]) -> None:
    log = state.get("write_log", [])
    done = sum(1 for entry in log if entry["completed"])
    reason = state.get("reason") or "the restore did not complete"
    result["reason"] = f"{reason}; {done} of {len(log)} attempted writes completed" if log else \
        f"{reason}; nothing was written"


def _failed_restore(config: HarnessConfig, step: Callable[[str, str], dict[str, Any]], settings_before,
                    state: dict[str, Any]) -> tuple[dict[str, Any], BaseException | None]:
    """The restore's one failure path. `settings_restore_failed` and the write log are in `state["result"]` before
    anything else runs;
    the bookkeeping, the final reading, its comparison and the reason then run each under its own guard, so an
    exception or cancellation in one is recorded as secondary and neither skips the others nor replaces the primary
    error. Returns the result and the error to raise: the primary one, else the first secondary one."""

    result = state.setdefault("result", {})
    result["status"] = "settings_restore_failed"
    result["write_log"] = state.setdefault("write_log", [])  # what was written, before anything else can fail
    secondary: list[BaseException] = state.setdefault("secondary_errors", [])

    def guarded(work: Callable[[], None]) -> None:
        try:
            work()
        except BaseException as caught:
            secondary.append(caught)

    def final_reading() -> None:
        result["final_reading"] = "interrupted"
        state["final"] = _settings_snapshot(config, step, "settings_after_failed_restore")
        result["final_reading"] = "read" if state["final"] is not None else "could not be read"

    def final_difference() -> None:
        if state.get("final") is not None:
            result["remaining_differences"] = settings_difference(
                settings_before, state["final"], scope_id=restore_scope_id(config.settings_guard),
                whole_key=config.settings_guard.restores_whole_key)

    guarded(lambda: _restore_log_summary(result, state))
    guarded(final_reading)
    guarded(final_difference)
    guarded(lambda: _restore_reason(result, state))
    if secondary:
        result["secondary_errors"] = [f"{type(error).__name__}: {error}" for error in secondary]
    primary = state.get("primary_error")
    return result, primary if primary is not None else (secondary[0] if secondary else None)


def _attach_guard(error: BaseException, settings: dict[str, Any]) -> None:
    error.settings_guard = settings


def _persist_guard_result(run: CaseFolder, settings: dict[str, Any]) -> BaseException | None:
    """Write the guard result, with its write log, as a new file in the run folder's caepipe_out/ before the run
    record (interruptible): `settings_guard.json`, or `settings_guard_2.json` and so on when an earlier write of it
    exists (a retry or a recovery never writes over one).

    A cancellation during the write is retried once, so a termination request arriving then does not lose the
    log; it is returned for the caller to raise (or keep beside a primary error). An I/O error is not retried; it
    is named in the result and also returned.
    """

    interrupted: BaseException | None = None
    for _ in range(2):
        try:
            run.record(run.free("caepipe_out", "settings_guard", ".json"), settings)
            if interrupted is not None:
                settings["guard_result_file"] = f"written after {type(interrupted).__name__}"
            return interrupted
        except BaseException as caught:
            interrupted = interrupted or caught
            if isinstance(caught, Exception):
                break
    settings["guard_result_file"] = f"not written: {type(interrupted).__name__}: {interrupted}"
    return interrupted


def _live_guard_result(live: dict[str, Any]) -> dict[str, Any]:
    """The guard result as it stands, without guest or file I/O: the round trip's own result once it has one; else
    the restore's failure-path result; else one built from the write log (`settings_restore_failed` when the
    restore was entered, else `settings_check_failed` with nothing written). The write log is always included."""

    state = live.get("guard_state") or {}
    log = state.get("write_log") or []
    result = live.get("settings")
    if result is None:
        result = state.get("result")
    if result is None:
        done = sum(1 for entry in log if entry["completed"])
        if log or state.get("restore_entered"):
            result = {"status": "settings_restore_failed",
                      "reason": f"interrupted before the guard result was complete; {done} of {len(log)} attempted "
                                "writes completed"}
        else:
            result = {"status": "settings_check_failed",
                      "reason": "interrupted before the guard result was complete; nothing was written"}
    if log or "write_log" in result or state.get("restore_entered"):
        result.setdefault("write_log", log)
        result.setdefault("writes_completed", sum(1 for entry in result["write_log"] if entry["completed"]))
    if state.get("failed_write_stderr"):
        result.setdefault("failed_write_stderr", state["failed_write_stderr"])
    return result


def _recover_guard_result(error: BaseException, live: dict[str, Any], run: CaseFolder) -> None:
    """For an exception leaving a guarded round trip before its reporting finished: attach the guard result, with
    its write log, to the exception before anything else, then write it once more as a new file in the run folder
    (`_persist_guard_result`), best effort and interruptible. A second cancellation during the write is kept beside the first, which is the one
    raised. After the reporting finished only a missing attachment is supplied."""

    def attach(result: dict[str, Any]) -> None:
        try:
            _attach_guard(error, result)
        except BaseException as caught:  # the error is raised all the same; the result names the failure
            _secondary(result, "attachment", caught)

    if live.get("reported"):
        if getattr(error, "settings_guard", None) is None:
            attach(_live_guard_result(live))
        return
    result = live.get("settings")
    if result is None:
        result = getattr(error, "settings_guard", None)
    if result is None:
        result = _live_guard_result(live)
    attach(result)
    result.setdefault("round_trip_error", f"{type(error).__name__}: {error}")
    again = _persist_guard_result(run, result)
    if again is not None and again is not error:
        _secondary(result, "guard result file", again)


def _attach_after_record_failure(error: BaseException, settings: dict[str, Any] | None) -> None:
    """A record failure that becomes the error (there was none before) still carries the guard result, with its
    write log, and the result names the failure."""
    if settings is not None:
        settings["run_record_error"] = f"{type(error).__name__}: {error}"
        try:
            _attach_guard(error, settings)
        except BaseException as caught:
            _secondary(settings, "attachment", caught)


def _guard_check(config: HarnessConfig, step: Callable[[str, str], dict[str, Any]], settings_before, after,
                 processes, state: dict[str, Any] | None = None,
                 write: Callable[[str, str], dict[str, Any]] | None = None) -> tuple[dict[str, Any], BaseException | None]:
    """The run record's `settings_guard`, and an exception (cancellation included) deferred from the restore.

    `unchanged` when the whole snapshot is equal and no instance of the tool is left. A difference confined to the
    values of the recent-files subkey (or, with `restore_scope` `"key"`, any difference strictly inside the
    configured key) is written back only when the configuration enables it and the process
    checks are clean (no instance of ours or anyone else's running), through `_restore_recent_files`; `state`
    carries its write log, so a caller that is itself interrupted can still report what was written. Any other
    difference is `settings_changed` and nothing is written.
    """

    state = {} if state is None else state
    guard = config.settings_guard
    if settings_before is None:
        return {"status": "settings_check_failed", "reason": "the settings could not be read before the run; "
                                                             "nothing ran"}, None
    result: dict[str, Any] = {"owned_processes": processes}
    if after is None:
        result.update(status="settings_check_failed", reason="the settings could not be read after the run")
        return result, None
    changed = settings_difference(settings_before, after, scope_id=restore_scope_id(guard),
                                  whole_key=guard.restores_whole_key)
    result["changed_values"] = changed
    if processes["status"] == "running":
        result.update(status="owned_process_running", reason="a process the guest started is still running; "
                                                             "nothing was written")
        return result, None
    if processes["status"] == "unknown":
        result.update(status="owned_process_state_unknown", reason=processes["reason"])
        return result, None
    if processes["status"] == "foreign":
        result.update(status="foreign_instance_running", reason="an instance of the tool this run did not start "
                                                                "appeared; nothing was written")
        return result, None
    if not changed:
        result["status"] = "unchanged"
        return result, None
    if guard.restores_whole_key:
        scope_id, enabled = _registry_key_id(guard.key), True
    else:
        scope_id = _registry_key_id(guard.recent_files_key) if guard.recent_files_key else None
        keys_equal = set(settings_before["keys"]) == set(after["keys"])
        outside = [item for item in set(settings_before["values"]) | set(after["values"])
                   if item[0] != scope_id
                   and settings_before["values"].get(item, (None,))[1:] != after["values"].get(item, (None,))[1:]]
        enabled = guard.restore_recent_files and scope_id and keys_equal and not outside
    if not enabled:
        result.update(status="settings_changed", reason="the saved settings differ after the run"
                      + ("" if guard.restore_recent_files else " (restoring is not enabled)") + "; nothing was written")
        return result, None
    try:
        plan = _restore_plan(settings_before, after, scope_id, whole_key=guard.restores_whole_key,
                             literal=_literal_recent_value(config))
    except ValueError as error:
        result.update(status="settings_restore_failed", reason=f"{error}; nothing was written")
        return result, None
    state["restore_entered"] = True  # also set inside: recovery is attempted from here on, log or no log
    restored, failure = _restore_recent_files(config, step, write, settings_before, plan, state)
    result.update(restored)
    return result, failure


def restore_recorded_settings(config: HarnessConfig, changed_values: list[Mapping[str, Any]], *,
                              runner: Callable[..., subprocess.CompletedProcess] = subprocess.run) -> dict[str, Any]:
    """Write back the value differences a round trip recorded (`changed_values` of its guard result) to their
    `before` values, through the round trip's own restore routine (`_restore_plan`, `_restore_recent_files`).

    Needs `restore_scope` `"key"`. Nothing is written unless: no instance of the tool is running; the key reads
    whole; every recorded entry is a value (not a subkey) inside the key; and every recorded value still has its
    recorded `after` type and data (absent where `after` is None). Otherwise the status says why
    (`changed_since_record` lists the values that differ). The intended reading is the current one with each
    recorded value at its `before`; the routine checks before each write and after the last, reads a third time
    and compares exactly. Each recorded value's status is listed in `values`. An exception or cancellation from
    the routine is raised with the result attached.
    """

    guard = config.settings_guard
    if guard is None or not guard.restores_whole_key:
        return {"status": "not_enabled", "reason": "restoring recorded values needs restore_scope \"key\""}
    steps: list[dict[str, Any]] = []
    state: dict[str, Any] = {}

    def step(name: str, command: str, deadline: float | None = None) -> dict[str, Any]:
        return _guest_read(config, runner, guard.call_timeout_s, steps, name, command, deadline)

    def write(name: str, command: str, deadline: float | None = None) -> dict[str, Any]:
        return _guest_write(config, runner, guard.call_timeout_s, steps, name, command, state, deadline)

    def finish(result: dict[str, Any]) -> dict[str, Any]:
        result["steps"] = [{key: value for key, value in item.items() if key not in ("stdout", "stderr")}
                           | _failed_write_stderr(item) for item in steps]
        return result

    live: dict[str, Any] = {"guard_state": state}
    with _catchable_termination(True, live):
        pre = _precondition(config, step, "precondition_process_list")
        if pre["status"] != "clear":
            return finish(pre | {"write_log": []})
        current = _settings_snapshot(config, step, "settings_current")
        if current is None:
            return finish({"status": "settings_check_failed", "reason": "the key could not be read; nothing was "
                                                                        "written", "write_log": []})
        root = _registry_key_id(guard.key)
        printed_keys = sorted(current["keys"].values(), key=len, reverse=True)
        intended = {"keys": dict(current["keys"]), "values": dict(current["values"])}
        located, differs = [], []
        for entry in changed_values:
            value = entry.get("value") if isinstance(entry, Mapping) else None
            if not isinstance(entry, Mapping) or entry.get("outside_restore_scope") or not all(
                    side is None or isinstance(side, Mapping) for side in (entry.get("before"), entry.get("after"))):
                return finish({"status": "not_restorable", "reason": f"a recorded entry carries no values (it lay "
                                                                     f"outside the recording step's restore scope): "
                                                                     f"{value or entry}", "write_log": []})
            printed = next((key for key in printed_keys if isinstance(value, str) and value.startswith(key + "\\")),
                           None)
            if printed is None or not (_registry_key_id(printed) == root
                                       or _registry_key_id(printed).startswith(root + "\\")):
                return finish({"status": "not_restorable", "reason": f"a recorded entry is not a value inside the "
                                                                     f"key: {value or entry}", "write_log": []})
            item = (_registry_key_id(printed), value[len(printed) + 1:])
            found = current["values"].get(item)
            after, before = entry.get("after"), entry.get("before")
            if (None if found is None else {"type": found[1], "data": found[2]}) != after:
                differs.append(value)
            if before is None:
                intended["values"].pop(item, None)
            else:
                intended["values"][item] = (printed, before["type"], before["data"])
            located.append((value, item))
        if differs:
            return finish({"status": "changed_since_record", "reason": "these recorded values no longer have their "
                                                                       "recorded after-state; nothing was written",
                           "values": [{"value": value, "status": "changed_since_record"} for value in differs],
                           "write_log": []})
        try:
            plan = _restore_plan(intended, current, root, whole_key=True, literal=_literal_recent_value(config))
        except ValueError as error:
            return finish({"status": "settings_restore_failed", "reason": f"{error}; nothing was written",
                           "write_log": []})
        state["restore_entered"] = True
        result, failure = _restore_recent_files(config, step, write, intended, plan, state)
        written = {entry["value"]: entry["outcome"] for entry in result.get("write_log", []) if "value" in entry}
        verified = result["status"] == "settings_restored"
        result["values"] = [{"value": value, "status": "restored_and_verified" if verified else
                             written.get(value, "not_written")} for value, _ in located]
        finish(result)
        if failure is not None:
            _attach_guard(failure, result)
            raise failure
        return result


def _record_exceptional_end(run: CaseFolder, live: dict[str, Any], error: BaseException, make_base) -> None:
    """For an exception leaving a step whose run folder is reserved, before its run record was written: write a run
    record of the exceptional end, every member present, as a new file in the run folder (`_write_run_record_any`:
    under another new name when the first is occupied or partial). The original exception is raised by the caller;
    when no record can be written, the exception says so (a note)."""

    if live.get("settings_stage") and not live.get("settings_check") and live.get("settings") is not None:
        try:  # the settings check finished before the exception: its record is truthful and is written
            _write_settings_check(run, live["settings"])
            live["settings_check"] = True
        except Exception:
            pass
    if live.get("record_written") or live.get("exceptional_end_attempted"):
        return  # attempted once per step: a second attempt could fall outside a guarded step's termination handling
    live["exceptional_end_attempted"] = True
    try:
        record = dict(live.get("record") or make_base())
        record.update(route="harness_error", harness_error=f"{type(error).__name__}: {error}",
                      steps=_steps_view(live.get("steps") or []),
                      exchange_marker=MARKER_KEPT if live.get("marker") else "not created")
        if live.get("run_lock"):
            record["run_lock"] = dict(live["run_lock"])
        if live.get("settings") is not None:
            record["settings_guard"] = live["settings"]
        if isinstance(record.get("mbf_export"), dict):
            record["mbf_export"] = {**record["mbf_export"], "route": "harness_error"}
        path = _write_run_record_any(run, record)
        live["record_written"] = True
        try:
            error.add_note(f"the exceptional end is recorded at cases/{run.case_id}/{path}")
        except Exception:
            pass
    except BaseException as late:  # the original exception stays the one raised
        try:
            error.add_note(f"no run record of this exceptional end could be written in {run.relative}: "
                           f"{type(late).__name__}: {late}")
        except Exception:
            pass


def _round_trip(config: HarnessConfig, case_id: str, mbf_text: str, tree: Tree, run: CaseFolder,
                live: dict[str, Any], timeout_s: float, runner: Callable[..., subprocess.CompletedProcess],
                gui_fallback: bool, files: dict[str, bytes], capture_only: bool = False) -> dict[str, Any]:
    # A guarded round trip turns SIGTERM into a catchable cancellation for its whole lifetime, so a termination
    # request is recorded like a Ctrl-C instead of ending the process with the guard's log unwritten.
    # An exception at any reporting or persistence step leaves with the guard result attached and written once more
    # (`_recover_guard_result`); the run record of the exceptional end is written by `_locked`.
    guarded = config.settings_guard is not None
    started = _now()
    with _catchable_termination(guarded, live):
        try:
            return _round_trip_body(config, case_id, mbf_text, tree, run, timeout_s, runner, gui_fallback, live,
                                    files, capture_only)
        except BaseException as error:
            if guarded and live.get("guard_state") is not None:
                _recover_guard_result(error, live, run)
            # Recorded here, while a termination request is still caught (`_locked` records anything outside).
            _record_exceptional_end(run, live, error, lambda: _base_record(config, case_id, run, "round_trip", started))
            raise


def _require_empty(root: Path, out: Path) -> None:
    """The run folder's caepipe_out/ just before the guest collects into it: a plain folder, not a link, holding
    nothing, so no guest copy can land on an existing file."""

    folder = contained(root, out)
    if folder.is_symlink() or not folder.is_dir():
        raise ContainmentError(f"{folder.relative_to(Path(root).resolve())} is not a plain folder; nothing is "
                               "collected into it")
    entries = sorted(entry.name for entry in folder.iterdir())
    if entries:
        raise ContainmentError(f"{folder.relative_to(Path(root).resolve())} is not empty before collection "
                               f"({', '.join(entries)}); nothing is collected into it")


def _round_trip_body(config: HarnessConfig, case_id: str, mbf_text: str, tree: Tree, run: CaseFolder,
                     timeout_s: float, runner: Callable[..., subprocess.CompletedProcess], gui_fallback: bool,
                     live: dict[str, Any], files: dict[str, bytes], capture_only: bool = False) -> dict[str, Any]:
    started = _now()
    root = _root_of(config)
    mbf_bytes = mbf_text.encode("utf-8")
    mbf_sha256 = _sha256(mbf_bytes)
    script_bytes = GUEST_SCRIPT.read_bytes()
    out = run.path / "caepipe_out"
    # A fresh work directory of this step's own, created exclusively by the guest (`_fresh_work_dir`).
    work = _win(config.guest_work_root,
                WORK_DIR_NAME.format(case=case_id, kind="run", sequence=run.sequence, token=secrets.token_hex(8)))
    guest_mbf = _win(work, "model.mbf")
    guarded = config.settings_guard is not None

    def solve_command(script: str) -> str:
        return (f"powershell -NoProfile -ExecutionPolicy Bypass -File {_quote(script)} "
                f"-Exe {_quote(config.guest_executable)} -Mbf {_quote(guest_mbf)}"
                + (f" -PidFile {_quote(_win(work, OWNED_PIDS_FILE))}" if guarded else "")
                # Always -NoGuiFallback from any command: `run_caepipe_round_trip` refuses `gui_fallback=True` before
                # anything is started; the fallback is enabled only under DEL-17-05 REQ-023.
                + ("" if gui_fallback else " -NoGuiFallback"))

    steps: list[dict[str, Any]] = []
    live["steps"] = steps

    def step(name: str, command: str) -> dict[str, Any]:
        return _guest_step(config, runner, timeout_s, steps, name, command)

    command_shape = {
        "batch": BATCH_COMMAND_SHAPE, "attempted": False,
        "gui_print_fallback": {
            "allowed": gui_fallback, "used": False, "label": "print_to_file_fallback",
            "description": "fallback when the batch route yields no result file: the guest script stops the batch "
                           "process it started, opens the solved .res in a CAEPIPE instance it starts itself and "
                           "prints the results to CSV via Print > To File"},
        "host_invocation": "prlctl exec <vm_name> --current-user cmd /c <guest command>"}
    record = _base_record(config, case_id, run, "round_trip", started)
    record.update(mbf={"path": "caepipe_in/model.mbf", "guest_path": guest_mbf, "sha256": mbf_sha256,
                       "bytes": len(mbf_bytes)},
                  guest_mbf_sha256=None, input_hash_verified=False, command_shape=command_shape,
                  working_directory=work, expected_batch_csv_path=_win(work, "model.csv"), sealed=capture_only)
    record["run_lock"] = dict(live.get("run_lock") or {})
    live["record"] = record

    def refuse(route: str, reason: str) -> dict[str, Any]:
        record.update(route=route, reason=reason, steps=_steps_view(steps),
                      exchange_marker=MARKER_KEPT if marker else "not created")
        path = _write_run_record(run, record)
        live["record_written"] = True
        return {**record, "run_record": path,
                "reason": f"{reason}; nothing stored was changed; recorded at cases/{case_id}/{path}"}

    # The marker is the first file of the run folder; nothing else is written there, and nothing anywhere, until the
    # exchange root is established by it and the guest's work folder is shown to lie off every shared drive.
    marker = _create_marker(run)
    live["marker"] = marker
    try:
        script_name, script_outcome = _ensure_guest_script(run, script_bytes)
    except ContainmentError as error:
        return refuse(EXCHANGE_CHECK_FAILED_ROUTE, str(error))
    record["guest_script"].update(exchange_copy=f"{HARNESS_DIR}/{script_name}", sha256=_sha256(script_bytes),
                                  copy=script_outcome)
    exchange = _resolve_exchange_root(config, step, (*run.parts, EXCHANGE_MARKER), marker.sha256)
    record["exchange_check"] = exchange
    if exchange["status"] != "ready":
        return refuse(_route_of(exchange), exchange["reason"])
    problem = _work_folder_problem(config, exchange)
    if problem:
        return refuse(WORK_FOLDER_OVERLAP_ROUTE, f"{problem}; the guest's deletes and copies happen there, so nothing "
                                                 "is run")
    # Every place this step directs the guest script and the program to write, all directly in the fresh directory.
    destinations = ["model.mbf", script_name, OWNED_PIDS_FILE, *OUTPUT_NAMES, "model.csv", PRINT_TXT_NAME,
                    "model_results.csv"]
    refusal = _before_guest_writes(config, step, root, work, exchange["root"], destinations, record, read=(
        (lambda name, command: _guest_read(config, runner, config.settings_guard.call_timeout_s, steps, name, command))
        if guarded else None))
    if refusal:
        return refuse(*refusal)
    work_script = _win(work, script_name)
    run.create("caepipe_in/model.mbf", mbf_bytes)
    for relative, data in sorted(files.items()):
        run.create(relative, data)
    tree.ensure_dir(run.parts + ("caepipe_out",))
    # For this step only, every guest path is formed from the root established by content.
    config = dataclasses.replace(config, guest_exchange_root=exchange["root"])
    guest_run = _win(config.guest_exchange_root, *run.parts)
    guest_in, guest_out = _win(guest_run, "caepipe_in"), _win(guest_run, "caepipe_out")
    share_script = _win(config.guest_exchange_root, HARNESS_DIR, script_name)
    inputs = _check_inputs(config, step, {"model": (_win(guest_in, "model.mbf"), mbf_sha256),
                                          "script": (share_script, _sha256(script_bytes))})
    record["input_check"] = inputs
    if inputs["status"] != "ready":
        return refuse(EXCHANGE_CHECK_FAILED_ROUTE if inputs["status"] == "failed" else EXCHANGE_UNAVAILABLE_ROUTE,
                      f"under the established root: {inputs['reason']}")

    guard_state: dict[str, Any] = {}
    live["guard_state"] = guard_state  # what the outer recovery and the termination handler report from
    # Every guard call (registry reads and writes, process queries) has the guard's own short timeout; the solve
    # keeps `timeout_s`.
    guard_timeout = config.settings_guard.call_timeout_s if config.settings_guard else timeout_s

    def guard_step(name: str, command: str, deadline: float | None = None) -> dict[str, Any]:
        return _guest_read(config, runner, guard_timeout, steps, name, command, deadline)

    def write(name: str, command: str, deadline: float | None = None) -> dict[str, Any]:
        return _guest_write(config, runner, guard_timeout, steps, name, command, guard_state, deadline)

    # The guest's work directory is fresh: nothing in it is deleted or reused. The model is copied in with a
    # create-new copy; a failed prepare step stops the round trip before solving.
    # The saved settings are read before anything starts the tool; without that reading nothing runs.
    # Precondition: no instance of the tool may be running before ours starts; then the saved settings are read.
    # Without both, nothing runs.
    # No instance of the program and no guest script this step did not start may run: checked for every live step
    # before any guest write (`_before_guest_writes`); the saved settings are then read before anything starts it.
    blocked = None
    settings_before = _settings_snapshot(config, guard_step, "settings_before") if guarded and blocked is None else None
    if settings_before is not None:
        # The before-state is written before any guest command that can start the program or change a setting.
        _write_settings_stage(run, live, config.settings_guard, settings_before)
    prepare: dict[str, Any] = {"exit_status": "not_run", "stdout": "", "stderr": ""}
    solve: dict[str, Any] = {"exit_status": None, "stdout": "", "stderr": ""}
    script_location = "exchange_share"
    guest_log = None
    guest_mbf_sha256 = None
    input_verified = False
    settings = None
    collection: dict[str, Any] | None = None
    error: BaseException | None = None
    try:
        if not guarded or (blocked is None and settings_before is not None):
            copied = _collect(config, step, "prepare", guest_in, work, ["model.mbf"], [])
            prepare = {"exit_status": 0 if copied["complete"] and copied["files"].get("model.mbf") in
                       ("copied", "present_same") else 1, "copy": copied}
        if prepare["exit_status"] == 0:
            solve = step("solve", solve_command(share_script))
            guest_log = last_json_object(solve["stdout"])
            if guest_log is None and solve["exit_status"] != "timeout":
                script_location = "guest_work_dir"
                _collect(config, step, "stage_script", _win(config.guest_exchange_root, HARNESS_DIR), work,
                         [script_name], [])
                solve = step("solve_from_work_dir", solve_command(work_script))
                guest_log = last_json_object(solve["stdout"])
        # The guest script reports the SHA-256 of the MBF it actually solved; outputs are attributed to this MBF
        # only when that hash equals the host's.
        guest_mbf_sha256 = guest_log.get("mbf_sha256") if isinstance(guest_log, dict) else None
        guest_mbf_sha256 = guest_mbf_sha256.lower() if isinstance(guest_mbf_sha256, str) else None
        input_verified = guest_mbf_sha256 == mbf_sha256
        if guest_log is not None and input_verified:
            _require_empty(root, out)  # the new folder holds nothing; each copy is still a create-new (`_collect`)
            collection = _collect(config, step, "collect", work, guest_out, [PRINT_TXT_NAME, *OUTPUT_NAMES],
                                  ["model*.csv"])
            record["collection"] = collection
    except BaseException as caught:  # re-raised below, after the checks
        error = caught
    # The guard's post-run work runs in normal signal context: only the restore's registry-mutation section defers
    # signals. Callee exceptions here are kept (the first one raised), and the guard result is attached to the
    # error before any record I/O, so a signal or a failure during the record cannot lose it.
    # Whatever happened above: confirm that what the guest started has ended, then compare the whole settings
    # key with its reading before the run. Each check is guarded on its own, so a failure of one never skips the
    # other or the result.
    if guarded:
        deferred = None
        post_run_errors: list[BaseException] = []
        try:
            try:
                processes = (_owned_processes(config, guard_step, work, guest_log)
                             if prepare["exit_status"] == 0 else {"status": "not_started"})
            except BaseException as failure:  # a callee's cancellation included: the reading still runs
                if not isinstance(failure, Exception):
                    post_run_errors.append(failure)  # a cancellation is kept and raised; an error stops as unknown
                processes = _unknown(f"the process check failed: {type(failure).__name__}: {failure}")
            after = None
            if settings_before is not None:
                try:
                    after = _settings_snapshot(config, guard_step, "settings_after")
                except BaseException as failure:
                    if not isinstance(failure, Exception):
                        post_run_errors.append(failure)
                    processes = processes | {"settings_read_error": f"{type(failure).__name__}: {failure}"}
            if blocked is not None:
                settings = blocked
            else:
                settings, deferred = _guard_check(config, guard_step, settings_before, after, processes, guard_state,
                                                  write)
        except BaseException as caught:  # cancellation during the checks or escaping the restore's failure path
            primary = guard_state.get("primary_error")
            deferred = primary if primary is not None else caught
            log = guard_state.get("write_log") or []
            settings = dict(guard_state.get("result") or {})
            if log or settings or guard_state.get("restore_entered"):
                settings.setdefault("write_log", log)
                settings.setdefault("writes_completed", sum(1 for entry in log if entry["completed"]))
                if guard_state.get("failed_write_stderr"):
                    settings.setdefault("failed_write_stderr", guard_state["failed_write_stderr"])
                settings["status"] = "settings_restore_failed"
                settings.setdefault("reason", f"interrupted ({type(caught).__name__}) during the restore; "
                                              f"{settings['writes_completed']} of {len(log)} attempted writes "
                                              "completed")
                if "final_reading" not in settings:
                    try:
                        final = _settings_snapshot(config, guard_step, "settings_after_failed_restore")
                        settings["final_reading"] = "read" if final is not None else "could not be read"
                    except BaseException as late:
                        settings["final_reading"] = f"interrupted ({type(late).__name__})"
            else:
                settings = {"status": "settings_check_failed",
                            "reason": f"the checks after the run were interrupted ({type(caught).__name__}); "
                                      "nothing was written"}
            if caught is not deferred:
                settings["secondary_errors"] = list(settings.get("secondary_errors", [])) + [
                    f"{type(caught).__name__}: {caught}"]
        live["settings"] = settings
        if deferred is not None:
            if error is None:
                error = deferred
            elif deferred is not error:
                settings["guard_error"] = f"{type(deferred).__name__}: {deferred}"
        for failure in post_run_errors:  # the first error stays primary; later ones are kept beside it
            if error is None:
                error = failure
            elif failure is not error:
                _secondary(settings, "post-run check", failure)
        # A signal the restore's mutation section deferred stops the run now, after the restore is recorded in memory.
        if guard_state.get("deferred_signals"):
            interrupt = KeyboardInterrupt(f"deferred signal {guard_state['deferred_signals'][0]}")
            if error is None:
                error = interrupt
            else:
                _secondary(settings, "deferred signal", interrupt)
    # The guard result is attached to the error before any record I/O, so no record failure can lose it, and then
    # persisted on its own (with its write log) before the run record, so a cancellation during the record does not
    # lose it either.
    if live.get("settings_stage") and settings is not None:
        try:  # the check record: the settings check has finished
            _write_settings_check(run, settings)
            live["settings_check"] = True
        except Exception as caught:
            if settings is not None:
                _secondary(settings, "settings check record", caught)
    if error is not None and settings is not None:
        try:
            _attach_guard(error, settings)
        except BaseException as caught:
            _secondary(settings, "attachment", caught)
    if guarded and settings is not None:
        interrupted = _persist_guard_result(run, settings)
        if interrupted is not None:
            if error is None:
                error = interrupted
                try:
                    _attach_guard(error, settings)
                except BaseException as caught:
                    _secondary(settings, "attachment", caught)
            elif interrupted is not error:
                _secondary(settings, "guard result file", interrupted)

    try:
        run.create("caepipe_out/guest_stdout.txt", solve["stdout"])
        run.create("caepipe_out/guest_stderr.txt", solve["stderr"])
        if guest_log is not None:
            run.create("caepipe_out/guest_log.json", json.dumps(guest_log, indent=2, sort_keys=True) + "\n")

        guest_csv = guest_log.get("csv_path") if isinstance(guest_log, dict) else None
        observed = None
        csv_name = guest_csv.replace("/", "\\").split("\\")[-1] if isinstance(guest_csv, str) else ""
        print_problem = None
        collected_whole = collection is not None and collection["complete"]
        if csv_name not in ("", ".", "..") and input_verified and error is None and collected_whole:
            candidate = contained(root, out / csv_name)
            if candidate.is_file() and not candidate.is_symlink():
                observed = candidate
                # CAPTURE-ONLY PLACE: the one read of a collected result file in the harness. A `.txt` print is
                # recognised by running the print parser on it; a capture-only step (`capture_only=True`, for a
                # sealed capture or a format sample's output) skips it, hashes the file without reading it, and
                # records it as captured, unread.
                if csv_name.lower() == PRINT_TXT_NAME and capture_only:
                    print_problem = "not read: capture only"
                    observed = None
                elif csv_name.lower() == PRINT_TXT_NAME:
                    print_problem = _print_layout_problem(candidate)
                    observed = None if print_problem else observed
        status = guest_log.get("status") if isinstance(guest_log, dict) else None
        if error is not None:
            route = "harness_error"
        elif blocked is not None:
            route = blocked["status"]
        elif guarded and settings_before is None:
            route = "settings_check_failed"
        elif prepare["exit_status"] != 0:
            route = "prepare_failed"
        elif solve["exit_status"] == "timeout":
            route = "timeout"
        elif guest_log is None:
            route = "no_guest_json"
        elif not input_verified:
            route = "input_hash_mismatch"
        elif not collected_whole:
            route = COLLECTION_FAILED_ROUTE
        elif print_problem is not None:
            route = "captured_unread" if capture_only else "print_file_unrecognised"
        elif status in CSV_ROUTES and observed is not None:
            route = status
        elif status in CSV_ROUTES:
            route = "csv_not_collected"
        else:
            route = str(status or "unknown_guest_status")
        gui_used = bool(isinstance(guest_log, dict) and isinstance(guest_log.get("gui"), dict)
                        and guest_log["gui"].get("used"))

        collected = [path.name for path in out.iterdir()
                     if path.name.startswith("model") and path.is_file() and not path.is_symlink()]
        command_shape["attempted"] = prepare["exit_status"] != "not_run"
        command_shape["gui_print_fallback"]["used"] = gui_used
        record.update({
            "guest_mbf_sha256": guest_mbf_sha256,
            "input_hash_verified": input_verified,
            "steps": _steps_view(steps),
            "observed_csv_path": {"guest": guest_csv, "host": f"caepipe_out/{observed.name}" if observed else None},
            "print_file": {"collected": observed.name if observed else None,
                           "layout": ("print_csv" if observed is not None and csv_name.lower() == PRINT_TXT_NAME
                                      else print_problem or ("as_named" if observed is not None else None))},
            "route": route,
            "exit_status": solve["exit_status"],
            "stdout_file": "caepipe_out/guest_stdout.txt",
            "stderr_file": "caepipe_out/guest_stderr.txt",
            "guest_log": guest_log,
            "outputs_collected": sorted(collected),
            "outputs_sha256": {name: _file_sha256(out / name) for name in sorted(collected)},
            "exchange_marker": MARKER_KEPT,
        })
        record["guest_script"]["run_from"] = script_location
        if error is not None:
            record["harness_error"] = f"{type(error).__name__}: {error}"
        if settings is not None:
            record["settings_guard"] = settings
        record_ready = True
    except BaseException as caught:
        record_ready = False
        if error is None:
            error = caught
            _attach_after_record_failure(error, settings)
        else:
            _secondary(settings, "run record", caught)
    if record_ready:
        try:
            record["run_record"] = _write_run_record(run, record)
            live["record_written"] = True
        except BaseException as caught:
            if error is None:
                error = caught
                _attach_after_record_failure(error, settings)
            _secondary(settings, "run record", caught)
    live["reported"] = True  # the records are done: the outer recovery has nothing left to write
    if error is not None:
        raise error
    return record


def outputs_sealed(record: Mapping[str, Any] | None) -> bool:
    """True when a run record marks its collected outputs sealed (a capture-only step, or `sealed` set by a later
    capture path): every reader leaves those outputs unread."""
    return bool(isinstance(record, Mapping) and (record.get("sealed") or record.get("capture_only")))


def stored_run_record(root: Path, out: Path) -> dict[str, Any] | None:
    """The harness run record stored in caepipe_out/, or None when absent or not a run record."""

    path = contained(root, out / "run_record.json")
    try:
        record = json.loads(path.read_text(encoding="utf-8")) if path.is_file() else None
    except (OSError, UnicodeDecodeError, json.JSONDecodeError):
        return None
    return record if isinstance(record, dict) and record.get("document_kind") == RUN_RECORD_KIND else None


def solved_model_binding(record: Mapping[str, Any] | None, mbf_sha256: str, mod: Path) -> str:
    """Why the stored `model.mod` is not the solve of the MBF with `mbf_sha256`, or "" when the run record binds it."""

    if record is None:
        return "no valid caepipe_out/run_record.json binds a solved model to this MBF"
    if outputs_sealed(record):
        return "the run record marks its outputs sealed; they are not read"
    if (record.get("mbf") or {}).get("sha256") != mbf_sha256 or record.get("guest_mbf_sha256") != mbf_sha256:
        return "the stored solve is not of the current MBF (host or guest MBF hash differs)"
    if "model.mod" not in (record.get("outputs_collected") or []) or not mod.is_file():
        return "the run record names no collected caepipe_out/model.mod"
    hashes = record.get("outputs_sha256")
    # Records written before outputs were hashed bind model.mod by name only.
    if isinstance(hashes, dict) and hashes.get("model.mod") != _sha256(mod.read_bytes()):
        return "caepipe_out/model.mod differs from the solved model the run record hashed"
    return ""


def run_caepipe_mbf_export(config: HarnessConfig, case_id: str, mbf_sha256: str, *, timeout_s: float = 600,
                           runner: Callable[..., subprocess.CompletedProcess] = subprocess.run) -> dict[str, Any]:
    """Export the case's current solved model back to MBF with the user's CAEPIPE: refused by name before anything
    is started (`window_route_unavailable`; a complete run record in the step's new run folder and nothing else) until
    DEL-17-05 REQ-023 is placed. What the step does once enabled (`_mbf_export_body`):

    The solve is the case's current run (`current_case`: the run a selection names, or the case's own folders when it
    has no selection; an untrusted selection is refused as `selection_untrusted`). Its run record must bind its
    `model.mod` to the MBF with `mbf_sha256`; otherwise the route is `not_bound` and no guest is asked. The guest
    copies that `model.mod` (a read) into its work folder, opens it in a CAEPIPE instance it starts, exports it with
    File > Export to MBF... and stops that instance; the harness collects `exported.mbf` into the empty
    `caepipe_out/` of the step's own new run folder only when the guest reports opening a model whose SHA-256 equals
    the stored one. Everything the step writes is a new file in that run folder, under the same rule and refusals as
    the round trip; the solve run is never written to. The export's run record names the solve it read (folder and
    the hashes of what it read) separately, never inheriting its members; the export becomes current only when a
    caller publishes it (`publish`), after checking it.
    """

    # The export of a model file from the program operates its windows: it is enabled only under DEL-17-05 REQ-023
    # (the listed-window rule), when it also gains the settings guard and the owned-process record. Until then it is
    # refused by name before anything is started, and `_mbf_export_body` is reachable from no command.
    return _locked(config, case_id, "mbf_export", None,
                   lambda run, started: _export_base(config, case_id, run, started, mbf_sha256),
                   refuse_before=(WINDOW_ROUTE_UNAVAILABLE, WINDOW_ROUTE_REASON))


def _export_under_requirement_23(config: HarnessConfig, case_id: str, mbf_sha256: str, *, timeout_s: float = 600,
                                 runner: Callable[..., subprocess.CompletedProcess] = subprocess.run) -> dict[str, Any]:
    """The export step as it is to run once DEL-17-05 REQ-023 (the listed-window rule) enables it. Reachable by tests
    only: no command calls this. It exists so that the tests of `_mbf_export_body` keep checking, while the route is
    refused, the code that change will enable, instead of letting that code go unexamined until then."""
    return _locked(config, case_id, "mbf_export", lambda tree, run, live: _mbf_export_body(
        config, case_id, mbf_sha256, tree, run, timeout_s, runner, live),
        lambda run, started: _export_base(config, case_id, run, started, mbf_sha256))


def _export_base(config: HarnessConfig, case_id: str, run: CaseFolder, started: str, mbf_sha256: str) -> dict[str, Any]:
    """Every member of an export step's run record, with not-attempted or null values."""

    record = _base_record(config, case_id, run, "mbf_export", started)
    record["mbf"] = {"sha256": mbf_sha256, "role": "the MBF whose solve is exported (read from the solve run)"}
    record["solve_run"] = None
    record["command_shape"].update(description="the guest script opens a copy of the solved model in a CAEPIPE "
                                               "instance it starts (File > Open...), writes it with File > Export to "
                                               "MBF... and stops that instance")
    record["mbf_export"] = {
        "route": None, "solved_mbf_sha256": mbf_sha256,
        "mod": {"path": None, "guest_path": None, "sha256": None}, "guest_mod_sha256": None,
        "mod_hash_verified": False, "exported_mbf": {"path": None, "sha256": None, "bytes": None},
        "guest_exported_mbf_sha256": None}
    return record


def _mbf_export_body(config: HarnessConfig, case_id: str, mbf_sha256: str, tree: Tree, run: CaseFolder,
                     timeout_s: float, runner: Callable[..., subprocess.CompletedProcess],
                     live: dict[str, Any]) -> dict[str, Any]:
    # Enabled only under DEL-17-05 REQ-023 (the listed-window rule); no command reaches it until then.
    started = _now()
    root = _root_of(config)
    script_bytes = GUEST_SCRIPT.read_bytes()
    # A fresh work directory of this step's own, created exclusively by the guest (`_fresh_work_dir`).
    work = _win(config.guest_work_root,
                WORK_DIR_NAME.format(case=case_id, kind="export", sequence=run.sequence, token=secrets.token_hex(8)))
    guest_mod, guest_mbf = _win(work, "model.mod"), _win(work, EXPORTED_MBF)
    out = run.path / "caepipe_out"
    steps: list[dict[str, Any]] = []
    live["steps"] = steps
    record = _export_base(config, case_id, run, started, mbf_sha256)
    record.update(working_directory=work)
    record["mbf_export"]["mod"]["guest_path"] = guest_mod
    record["run_lock"] = dict(live.get("run_lock") or {})
    live["record"] = record
    marker: _Marker | None = None

    def refuse(route: str, reason: str) -> dict[str, Any]:
        record["mbf_export"].update(route=route, reason=reason)
        record.update(route=route, reason=reason, steps=_steps_view(steps),
                      exchange_marker=MARKER_KEPT if marker else "not created")
        path = _write_run_record(run, record)
        live["record_written"] = True
        return {**record["mbf_export"], "run_folder": run.relative, "run_record": path,
                "solve_run": record["solve_run"],
                "reason": f"{reason}; nothing stored was changed; recorded at cases/{case_id}/{path}"}

    # The solve the export reads: the case's current run. Only its run record and model.mod are read, by hash.
    try:
        view = current_case(root, case_id)
    except SelectionError as error:
        return refuse(SELECTION_UNTRUSTED_ROUTE, str(error))
    solve_out = view.run / "caepipe_out"
    run_record = stored_run_record(root, solve_out)
    mod = contained(root, solve_out / "model.mod")
    record_path = contained(root, solve_out / "run_record.json")
    record["solve_run"] = {
        "folder": view.run_relative, "source": view.source, "selection": view.selection,
        "run_record_sha256": _file_sha256(record_path) if record_path.is_file() and not record_path.is_symlink() else None,
        "note": "the solve this export read; its members are its own, not this record's"}
    reason = solved_model_binding(run_record, mbf_sha256, mod)
    if reason:
        return refuse("not_bound", reason)
    mod_sha256 = _file_sha256(mod)
    record["solve_run"]["model_mod_sha256"] = mod_sha256
    record["mbf_export"]["mod"].update(path=f"{view.run_relative or '.'}/caepipe_out/model.mod", sha256=mod_sha256)

    def export_command(script: str) -> str:
        return (f"powershell -NoProfile -ExecutionPolicy Bypass -File {_quote(script)} "
                f"-Exe {_quote(config.guest_executable)} -ExportMod {_quote(guest_mod)} -ExportName {EXPORTED_MBF}")

    def step(name: str, command: str) -> dict[str, Any]:
        return _guest_step(config, runner, timeout_s, steps, name, command)

    # The marker is the first file of the run folder; nothing else is written until the exchange root is established
    # by it and the guest's work folder is shown to lie off every shared drive.
    marker = _create_marker(run)
    live["marker"] = marker
    try:
        script_name, script_outcome = _ensure_guest_script(run, script_bytes)
    except ContainmentError as error:
        return refuse(EXCHANGE_CHECK_FAILED_ROUTE, str(error))
    record["guest_script"].update(exchange_copy=f"{HARNESS_DIR}/{script_name}", sha256=_sha256(script_bytes),
                                  copy=script_outcome)
    exchange = _resolve_exchange_root(config, step, (*run.parts, EXCHANGE_MARKER), marker.sha256)
    record["exchange_check"] = exchange
    if exchange["status"] != "ready":
        return refuse(_route_of(exchange), exchange["reason"])
    problem = _work_folder_problem(config, exchange)
    if problem:
        return refuse(WORK_FOLDER_OVERLAP_ROUTE, f"{problem}; the guest's deletes and copies happen there, so nothing "
                                                 "is run")
    refusal = _before_guest_writes(config, step, root, work, exchange["root"], ["model.mod", script_name, EXPORTED_MBF],
                                   record)
    if refusal:
        return refuse(*refusal)
    work_script = _win(work, script_name)
    tree.ensure_dir(run.parts + ("caepipe_out",))
    # For this step only, every guest path is formed from the root established by content.
    config = dataclasses.replace(config, guest_exchange_root=exchange["root"])
    solve_parts = ("cases", case_id, *(view.run_relative.split("/") if view.run_relative else ()))
    guest_solve_out = _win(config.guest_exchange_root, *solve_parts, "caepipe_out")
    guest_out = _win(config.guest_exchange_root, *run.parts, "caepipe_out")
    share_script = _win(config.guest_exchange_root, HARNESS_DIR, script_name)
    inputs = _check_inputs(config, step, {"model": (_win(guest_solve_out, "model.mod"), mod_sha256),
                                          "script": (share_script, _sha256(script_bytes))})
    record["input_check"] = inputs
    if inputs["status"] != "ready":
        return refuse(EXCHANGE_CHECK_FAILED_ROUTE if inputs["status"] == "failed" else EXCHANGE_UNAVAILABLE_ROUTE,
                      f"under the established root: {inputs['reason']}")

    # Into the fresh work directory only, with a create-new copy: nothing there is deleted or reused; the solve run's
    # model.mod is read, never written.
    copied = _collect(config, step, "export_prepare", guest_solve_out, work, ["model.mod"], [])
    prepare = {"exit_status": 0 if copied["complete"] and copied["files"].get("model.mod") in
               ("copied", "present_same") else 1, "copy": copied}
    script_location = "exchange_share"
    export: dict[str, Any] = {"exit_status": None, "stdout": "", "stderr": ""}
    guest_log = None
    if prepare["exit_status"] == 0:
        export = step("export", export_command(share_script))
        guest_log = last_json_object(export["stdout"])
        if guest_log is None and export["exit_status"] != "timeout":
            script_location = "guest_work_dir"
            _collect(config, step, "export_stage_script", _win(config.guest_exchange_root, HARNESS_DIR), work,
                     [script_name], [])
            export = step("export_from_work_dir", export_command(work_script))
            guest_log = last_json_object(export["stdout"])

    def guest_hash(key: str) -> str | None:
        value = guest_log.get(key) if isinstance(guest_log, dict) else None
        return value.lower() if isinstance(value, str) else None

    guest_mod_sha256, guest_exported_sha256 = guest_hash("mod_sha256"), guest_hash("exported_mbf_sha256")
    mod_verified = guest_mod_sha256 == mod_sha256
    status = guest_log.get("status") if isinstance(guest_log, dict) else None
    collection: dict[str, Any] | None = None
    if status == EXPORT_ROUTE and mod_verified:
        _require_empty(root, out)  # the new folder holds nothing; the copy is still a create-new (`_collect`)
        collection = _collect(config, step, "export_collect", work, guest_out, [EXPORTED_MBF], [])
        record["collection"] = collection

    run.create("caepipe_out/export_guest_stdout.txt", export["stdout"])
    run.create("caepipe_out/export_guest_stderr.txt", export["stderr"])
    if guest_log is not None:
        run.create("caepipe_out/export_guest_log.json", json.dumps(guest_log, indent=2, sort_keys=True) + "\n")

    # Collected output is hashed, never parsed here.
    exported = contained(root, out / EXPORTED_MBF)
    exported_sha256 = exported_bytes = None
    if exported.is_file() and not exported.is_symlink():
        exported_sha256, exported_bytes = _file_sha256(exported), exported.stat().st_size
    if prepare["exit_status"] != 0:
        route = "prepare_failed"
    elif export["exit_status"] == "timeout":
        route = "timeout"
    elif guest_log is None:
        route = "no_guest_json"
    elif not mod_verified:
        route = "input_hash_mismatch"
    elif status != EXPORT_ROUTE:
        route = str(status or "unknown_guest_status")
    elif collection is not None and not collection["complete"]:
        route = COLLECTION_FAILED_ROUTE  # the file stays in this run folder; the run is never published
    elif exported_sha256 is None:
        route = "mbf_not_collected"
    elif exported_sha256 != guest_exported_sha256:
        route = "export_hash_mismatch"  # the file stays in this run folder; the run is never published
    else:
        route = EXPORT_ROUTE
    record["command_shape"]["attempted"] = True
    record["mbf_export"].update({
        "route": route,
        "guest_mod_sha256": guest_mod_sha256,
        "mod_hash_verified": mod_verified,
        "exported_mbf": {"path": f"caepipe_out/{EXPORTED_MBF}" if exported_sha256 else None,
                         "sha256": exported_sha256, "bytes": exported_bytes},
        "guest_exported_mbf_sha256": guest_exported_sha256,
    })
    record.update({
        "route": route,
        "steps": _steps_view(steps),
        "exit_status": export["exit_status"],
        "stdout_file": "caepipe_out/export_guest_stdout.txt",
        "stderr_file": "caepipe_out/export_guest_stderr.txt",
        "guest_log": guest_log,
        "outputs_collected": [EXPORTED_MBF] if exported_sha256 else [],
        "outputs_sha256": {EXPORTED_MBF: exported_sha256} if exported_sha256 else {},
        "exchange_marker": MARKER_KEPT,
    })
    record["guest_script"]["run_from"] = script_location
    path = _write_run_record(run, record)
    live["record_written"] = True
    return {**record["mbf_export"], "run_folder": run.relative, "run_record": path, "solve_run": record["solve_run"]}


def _text(value: Any) -> str:
    if value is None:
        return ""
    if isinstance(value, bytes):
        return value.decode("utf-8", "replace")
    return str(value)
