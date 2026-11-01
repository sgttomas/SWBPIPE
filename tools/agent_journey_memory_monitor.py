#!/usr/bin/env python3
"""D4's external two-second memory sampler. Publish health or cancel; never signal a process.

Run only in an explicitly authorized witness window, with a fresh private
--directory. The journey consumes heartbeat.json and cancel. The coordinator
creates done only after app/supplier cleanup AND model unload are established.
Producer loss is detected by the consumer's five-second monotonic health lease.
"""
from __future__ import annotations

import argparse
import json
import math
import os
from pathlib import Path
import re
import stat
import subprocess
import sys
import time

INTERVAL = 2.0
LEASE = 5.0
MIN_FREE_BYTES = 50 * 1024 ** 3
HEARTBEAT_BYTES = 1024


class MonitorRefusal(RuntimeError):
    """Fixed nonsecret reason only."""


def parse_free_bytes(text):
    """D4 metric: vm_stat page size * (free + inactive + speculative pages)."""
    if not isinstance(text, str):
        raise MonitorRefusal("telemetry_invalid")
    sizes = re.findall(r"page size of (\d+) bytes", text)
    if len(sizes) != 1 or int(sizes[0]) <= 0:
        raise MonitorRefusal("telemetry_invalid")
    counts = []
    for name in ("Pages free", "Pages inactive", "Pages speculative"):
        values = re.findall(rf"^{name}:\s+(\d+)\.\s*$", text, re.M)
        if len(values) != 1:
            raise MonitorRefusal("telemetry_invalid")
        counts.append(int(values[0]))
    return int(sizes[0]) * sum(counts)


def read_free_bytes():
    try:
        result = subprocess.run(["/usr/bin/vm_stat"], capture_output=True, text=True,
                                check=True, timeout=1)
        return parse_free_bytes(result.stdout)
    except (OSError, ValueError, subprocess.SubprocessError):
        raise MonitorRefusal("telemetry_unavailable") from None


def cancel(path):
    """A failed write is still a refusal; a lost producer's lease also expires."""
    try:
        with path.open("xb"):
            pass
    except FileExistsError:
        pass
    except OSError:
        raise MonitorRefusal("cancel_delivery_failed") from None


def check_health(heartbeat, cancel_file, *, clock=time.monotonic):
    """Consumer-side detection, independent of whether the producer caught its failure."""
    reason = "health_invalid"
    try:
        if cancel_file.exists():
            raise MonitorRefusal("cancelled")
        if not stat.S_ISREG(heartbeat.lstat().st_mode):
            raise MonitorRefusal("health_invalid")
        with heartbeat.open("rb") as stream:
            data = stream.read(HEARTBEAT_BYTES + 1)
        if len(data) > HEARTBEAT_BYTES:
            raise MonitorRefusal("health_invalid")
        record = json.loads(data)
        if not isinstance(record, dict) or set(record) != {"version", "sampledAt", "freeBytes"}:
            raise MonitorRefusal("health_invalid")
        sampled, free = record["sampledAt"], record["freeBytes"]
        if (type(record["version"]) is not int or record["version"] != 1
                or type(sampled) not in (int, float) or not math.isfinite(sampled)
                or type(free) is not int or free < MIN_FREE_BYTES):
            raise MonitorRefusal("health_invalid")
        age = clock() - sampled
        if not math.isfinite(age) or not 0 <= age < LEASE:
            raise MonitorRefusal("health_expired")
        return
    except MonitorRefusal as error:
        reason = str(error)
    except (OSError, ValueError, TypeError, ArithmeticError):
        pass
    try:
        cancel(cancel_file)
    except MonitorRefusal:
        # Cancellation still reaches the owning controller if the filesystem failed.
        pass
    raise MonitorRefusal(reason)


def publish(heartbeat, free, sampled):
    data = json.dumps({"version": 1, "sampledAt": sampled, "freeBytes": free},
                      allow_nan=False).encode() + b"\n"
    temporary = heartbeat.with_name("heartbeat.next")
    try:
        with temporary.open("xb") as stream:
            stream.write(data)
        temporary.replace(heartbeat)
    finally:
        temporary.unlink(missing_ok=True)


def monitor(directory, *, telemetry=read_free_bytes, clock=time.monotonic, wait=time.sleep):
    """Only owned private files and bounded telemetry. No app/supplier handle or PID input."""
    heartbeat, cancellation, done = (directory / name for name in ("heartbeat.json", "cancel", "done"))
    try:
        info = directory.lstat()
        if not stat.S_ISDIR(info.st_mode) or info.st_uid != os.getuid() or stat.S_IMODE(info.st_mode) != 0o700:
            raise MonitorRefusal("directory_not_private")
        if any(directory.iterdir()):
            raise MonitorRefusal("directory_not_fresh")
    except (OSError, MonitorRefusal):
        # An unadmitted path grants no file-write authority. Missing health refuses the consumer.
        return 1
    try:
        while True:
            if cancellation.exists():
                raise MonitorRefusal("cancelled")
            if done.exists():
                return 0
            sampled = clock()  # age includes the telemetry read, never freshness after a slow read
            free = telemetry()
            if type(free) is not int or free < MIN_FREE_BYTES:
                raise MonitorRefusal("memory_low_or_invalid")
            if not math.isfinite(sampled) or not 0 <= clock() - sampled < LEASE:
                raise MonitorRefusal("sample_expired")
            publish(heartbeat, free, sampled)
            wait(max(0.0, sampled + INTERVAL - clock()))
    except Exception:  # Producer errors cancel; abrupt death is covered by the consumer lease.
        cancel(cancellation)
        return 1


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--directory", required=True, type=Path)
    args = parser.parse_args()
    try:
        return monitor(args.directory)
    except MonitorRefusal:
        # Do not echo telemetry or arbitrary exception strings.
        print("memory monitor refused; require consumer cancellation", file=sys.stderr)
        return 1


if __name__ == "__main__":
    raise SystemExit(main())
