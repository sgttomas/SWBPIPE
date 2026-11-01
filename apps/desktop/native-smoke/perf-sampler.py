#!/usr/bin/env python3
"""Memory sampler of one desktop perf run (RT-U4, DEL-04-05; `perf.mjs`), macOS, standard library.

    perf-sampler.py --app-pid PID --app-path PATH --baseline PIDS --interval-ms MS

Every interval it lists the host's processes (`proc_listallpids`), classifies each new one
once by its executable path (`proc_pidpath`) and reads each tracked process's resident size,
phys_footprint and lifetime-maximum phys_footprint (`proc_pid_rusage`, RUSAGE_INFO_V4):

- `app`: PID itself;
- `worker`: another process running the app's own executable (its solve workers);
- `webcontent`, `gpu`, `networking`: a WebKit XPC service process
  (`com.apple.WebKit.WebContent` and so on) that was not running before the launch
  (BASELINE, comma-separated PIDs). WebKit starts these from launchd, not as children
  of the app, so a new one is attributed to the app: a browser or another app starting
  one during the run would be attributed too, and the report counts the candidates.

It prints one JSON line per sample, `{"t": epoch_ms, "p": [[pid, kind, rss, footprint,
lifetime_max_footprint], ...]}` (a process it may not read has nulls), and ends with
`{"end": epoch_ms}` once the app has been gone for two samples, or on SIGTERM.
"""
import argparse
import ctypes
import json
import os
import signal
import sys
import time

RUSAGE_INFO_V4 = 4
# rusage_info_v4: a 16-byte uuid, then uint64 fields; indices below count uint64 words from the start.
RESIDENT, FOOTPRINT, LIFETIME_MAX = 2 + 6, 2 + 7, 2 + 28
WEBKIT = (('com.apple.WebKit.WebContent', 'webcontent'), ('com.apple.WebKit.GPU', 'gpu'),
          ('com.apple.WebKit.Networking', 'networking'))

libproc = ctypes.CDLL('/usr/lib/libproc.dylib', use_errno=True)


def all_pids():
    count = libproc.proc_listallpids(None, 0)
    buffer = (ctypes.c_int * (count + 256))()
    count = libproc.proc_listallpids(buffer, ctypes.sizeof(buffer))
    return [buffer[i] for i in range(max(count, 0)) if buffer[i] > 0]


def pid_path(pid):
    buffer = ctypes.create_string_buffer(4096)
    length = libproc.proc_pidpath(pid, buffer, ctypes.sizeof(buffer))
    return buffer.value[:length].decode('utf-8', 'replace') if length > 0 else None


def rusage(pid):
    info = (ctypes.c_uint64 * 64)()
    if libproc.proc_pid_rusage(pid, RUSAGE_INFO_V4, ctypes.byref(info)) != 0:
        return None
    return info[RESIDENT], info[FOOTPRINT], info[LIFETIME_MAX]


def kind_of(pid, path, app_pid, app_path, baseline):
    if pid == app_pid:
        return 'app'
    if path is None:
        return None
    if os.path.realpath(path) == app_path:
        return 'worker'
    if pid in baseline:
        return None
    for name, kind in WEBKIT:
        if path.endswith('/' + name):
            return kind
    return None


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__.split('\n\n')[0])
    parser.add_argument('--app-pid', type=int, required=True)
    parser.add_argument('--app-path', required=True)
    parser.add_argument('--baseline', default='')
    parser.add_argument('--interval-ms', type=int, default=200)
    args = parser.parse_args(argv)
    app_path = os.path.realpath(args.app_path)
    baseline = {int(p) for p in args.baseline.split(',') if p}
    stop = []
    signal.signal(signal.SIGTERM, lambda *_: stop.append(True))
    kinds = {}
    gone = 0
    while not stop:
        started = time.monotonic()
        rows = []
        for pid in all_pids():
            if pid not in kinds:
                kinds[pid] = kind_of(pid, pid_path(pid), args.app_pid, app_path, baseline)
            kind = kinds[pid]
            if kind is None:
                continue
            usage = rusage(pid)
            rows.append([pid, kind, *(usage or (None, None, None))])
        # Gone: not listed, or no longer readable (an exited, unreaped process).
        if not any(row[1] == 'app' and row[3] is not None for row in rows):
            gone += 1
        sys.stdout.write(json.dumps({'t': round(time.time() * 1000), 'p': rows}, separators=(',', ':')) + '\n')
        sys.stdout.flush()
        if gone >= 2:
            break
        time.sleep(max(0.0, args.interval_ms / 1000 - (time.monotonic() - started)))
    sys.stdout.write(json.dumps({'end': round(time.time() * 1000)}) + '\n')
    sys.stdout.flush()


if __name__ == '__main__':
    main()
