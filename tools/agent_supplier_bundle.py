#!/usr/bin/env python3
"""Explicit stock supplier resource overlay preparation and bundle verification.

No signing or execution. The caller controls source/output parents and supplies
one reviewed manifest; there is no runtime acquisition or source discovery.
"""
from __future__ import annotations

import argparse
import hashlib
import functools
import json
import os
from pathlib import Path
import shutil
import stat
import tempfile

import agent_supplier_resource as resource

PROJECT = Path(__file__).resolve().parents[1]
DEFAULT_MANIFEST = PROJECT / "apps/desktop/src-tauri/resources/agent-supplier-pins.json"
TARGETS = ("aarch64-apple-darwin", "x86_64-apple-darwin")
LICENSES = {name: f"docs/third-party/codex-0.160.0-{name}.txt" for name in ("LICENSE", "NOTICE")}
ResourceError = resource.ResourceError


def _plain(path):
    # Reject links in the entire selected path, including directory components.
    path = Path(os.path.abspath(path))
    for part in (*reversed(path.parents), path):
        resource._require(not stat.S_ISLNK(part.lstat().st_mode), "Resource paths must not contain links.")
    return path


def _small(path):
    _plain(path)
    with resource._regular_file(path) as source:
        data = source.read(resource.CHUNK + 1)
    resource._require(len(data) <= resource.CHUNK, "Resource metadata exceeds 1 MiB.")
    return data


def _inputs(manifest, target, project):
    resource._require(target in TARGETS, "Choose an explicitly pinned Mac target.")
    raw = _small(manifest)
    doc = json.loads(raw)
    # load_pin applies the reviewed acquisition format/URL/CPU/size constraints.
    with tempfile.TemporaryDirectory(prefix="supplier-pin-") as temporary:
        snapshot = Path(temporary) / "pins.json"
        with snapshot.open("xb") as stream:
            stream.write(raw)
        pin = resource.load_pin(snapshot, target)
    files = doc.get("licensing", {}).get("files", [])
    resource._require(isinstance(files, list) and len(files) == 2,
                      "Manifest must name the exact LICENSE and NOTICE pair.")
    payloads = {"agent-supplier/pins.json": raw,
                "THIRD_PARTY_NOTICES.md": _small(project / "docs/THIRD_PARTY_NOTICES.md")}
    seen = set()
    for entry in files:
        name = entry.get("upstream_name")
        resource._require(name in LICENSES and name not in seen and entry.get("path") == LICENSES[name],
                          "Manifest licensing paths must be the reviewed LICENSE and NOTICE pair.")
        seen.add(name)
        resource._digest_fields(entry, resource.CHUNK)
        data = _small(project / entry["path"])
        resource._require(len(data) == entry["size"] and hashlib.sha256(data).hexdigest() == entry["sha256"],
                          "Source license/notice disagrees with the manifest.")
        payloads[entry["path"].removeprefix("docs/")] = data
    return pin, payloads


def _safe(operation):
    @functools.wraps(operation)
    def call(*args, **kwargs):
        try:
            return operation(*args, **kwargs)
        except ResourceError:
            raise
        except Exception:
            raise ResourceError("Supplier bundle refused: check explicit inputs, regular bytes and fresh output.") from None
    return call


@_safe
def prepare(output: Path, target: str, *, archive: Path | None = None,
            binary: Path | None = None, download=False, manifest=DEFAULT_MANIFEST,
            project=PROJECT) -> Path:
    """Create a fresh private directory; return its Tauri --config overlay path.

    Failure removes only the directory this call created. mkdir is the atomic
    no-replace claim; a preexisting or raced destination is never removed.
    """
    resource._require(sum((archive is not None, binary is not None, bool(download))) == 1,
                      "Choose exactly one explicit archive, binary or download; no fallback exists.")
    pin, payloads = _inputs(Path(manifest), target, Path(project))
    output = Path(os.path.abspath(output))
    _plain(output.parent)
    output.mkdir(mode=0o700)  # Atomic refusal of existing files, links and directories.
    try:
        resources = output / "resources"
        resources.mkdir(mode=0o700)
        for relative, data in payloads.items():
            path = resources / relative
            path.parent.mkdir(parents=True, exist_ok=True)
            with path.open("xb") as stream:
                stream.write(data)
        destination = resources / "agent-supplier" / pin["binary"]["resource_name"]
        if binary is not None:
            _plain(binary)
            with resource._regular_file(binary) as source, destination.open("xb") as stream:
                resource._copy_checked(source, stream, pin["binary"])
            resource.verify_binary(destination, pin)
            destination.chmod(0o755)
        else:
            if archive is not None:
                _plain(archive)
            resource.acquire(pin, destination, archive, download=download)
        relatives = [*payloads, "agent-supplier/" + pin["binary"]["resource_name"]]
        overlay = output / "tauri-overlay.json"
        with overlay.open("x", encoding="utf-8") as stream:
            json.dump({"bundle": {"resources": {str(resources / rel): rel for rel in relatives}}}, stream)
        verify(resources, target, manifest=manifest, project=project)
        return overlay
    except BaseException:
        shutil.rmtree(output)
        raise


@_safe
def verify(resources: Path, target: str, *, manifest=DEFAULT_MANIFEST, project=PROJECT):
    """Verify Contents/Resources against trusted source bytes; never execute code."""
    pin, payloads = _inputs(Path(manifest), target, Path(project))
    resources = _plain(resources)
    resource._require(resources.is_dir(), "Bundle resource directory is missing.")
    for relative, expected in payloads.items():
        resource._require(_small(resources / relative) == expected,
                          "Bundle manifest, index or license/notice bytes mismatch.")
    binary = _plain(resources / "agent-supplier" / pin["binary"]["resource_name"])
    resource._require(binary.stat().st_mode & 0o022 == 0,
                      "Bundled supplier must not be group/other writable.")
    resource.verify_binary(binary, pin)


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--manifest", type=Path, default=DEFAULT_MANIFEST)
    parser.add_argument("--target", choices=TARGETS, required=True)
    commands = parser.add_subparsers(dest="command", required=True)
    prep = commands.add_parser("prepare")
    prep.add_argument("--output", type=Path, required=True)
    source = prep.add_mutually_exclusive_group(required=True)
    source.add_argument("--archive", type=Path)
    source.add_argument("--binary", type=Path)
    source.add_argument("--download", action="store_true")
    check = commands.add_parser("verify")
    check.add_argument("--resources", type=Path, required=True)
    args = parser.parse_args(argv)
    try:
        if args.command == "prepare":
            prepare(args.output, args.target, archive=args.archive, binary=args.binary,
                    download=args.download, manifest=args.manifest)
        else:
            verify(args.resources, args.target, manifest=args.manifest)
    except Exception:
        # Never echo OS/archive/network exceptions, paths, or redirected URLs.
        print("Supplier bundle refused: check explicit source, pins, regular resource bytes and fresh output.")
        return 1
    print("Supplier resources verified; no supplier was executed or signed.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
