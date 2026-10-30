#!/usr/bin/env python3
"""Explicit build-time supplier acquisition. Never imports or executes a supplier.

The reviewed manifest is the trust root. Publication is a no-replace hard link
from private staging on the destination filesystem. This is not a runtime
downloader, signing tool, or platform-qualification check.
"""
from __future__ import annotations

import argparse
import contextlib
import gzip
import hashlib
import json
import os
from pathlib import Path, PurePosixPath
import re
import ssl
import stat
import struct
import tarfile
import tempfile
import time
import urllib.parse
import urllib.request
import zlib

CHUNK = 1024 * 1024
MAX_ARCHIVE = 256 * CHUNK
MAX_BINARY = 512 * CHUNK
TRANSFER_SECONDS = 600
CPUS = {"arm64": 0x0100000C, "x86_64": 0x01000007}


class ResourceError(Exception):
    """Safe, actionable diagnostic without response bodies or network URLs."""


def _require(condition, message):
    if not condition:
        raise ResourceError(message)


def _digest_fields(value, limit):
    _require(type(value.get("size")) is int and 0 < value["size"] <= limit,
             "Manifest size is missing or outside the acquisition bound.")
    _require(isinstance(value.get("sha256"), str) and
             re.fullmatch(r"[0-9a-f]{64}", value["sha256"]),
             "Manifest SHA-256 must be 64 lowercase hexadecimal characters.")


def load_pin(manifest: Path, target: str):
    _require(manifest.stat().st_size <= CHUNK, "Manifest exceeds 1 MiB.")
    doc = json.loads(manifest.read_text(encoding="utf-8"))
    _require(isinstance(doc, dict) and
             doc.get("schema") == "swbpipe-agent-supplier-pins/1" and
             doc.get("supplier") == "openai/codex" and doc.get("version") == "0.160.0",
             "Unsupported supplier manifest; use the reviewed stock pin.")
    _require(isinstance(doc.get("targets"), dict) and target in doc["targets"],
             "Target is not pinned; no alternate is allowed.")
    pin = doc["targets"][target]
    _require(isinstance(pin, dict) and isinstance(pin.get("archive"), dict) and
             isinstance(pin.get("binary"), dict), "Manifest target requires archive and binary objects.")
    archive, binary = pin["archive"], pin["binary"]
    _digest_fields(archive, MAX_ARCHIVE)
    _digest_fields(binary, MAX_BINARY)
    member = "codex-" + target
    _require(archive.get("member") == member and binary.get("resource_name") == member
             and PurePosixPath(member).name == member and "\\" not in member,
             "Manifest member/resource must be the single target executable name.")
    _require(archive.get("format") == "tar.gz" and
             binary.get("format") == "mach-o-64-le" and
             binary.get("architecture") in CPUS,
             "Unsupported archive or executable format/architecture.")
    expected = {"aarch64-apple-darwin": "arm64", "x86_64-apple-darwin": "x86_64"}
    _require(expected.get(target) == binary["architecture"], "Target architecture disagrees with pin.")
    _require(archive.get("url") ==
             "https://github.com/openai/codex/releases/download/rust-v0.160.0/" + member + ".tar.gz",
             "Archive URL is not the exact official release asset.")
    return pin


@contextlib.contextmanager
def _regular_file(path):
    # The caller chooses the source parent; never follow a final-component link.
    fd = os.open(path, os.O_RDONLY | os.O_NOFOLLOW | os.O_NONBLOCK)
    with os.fdopen(fd, "rb") as stream:
        _require(stat.S_ISREG(os.fstat(stream.fileno()).st_mode), "Input must be a regular file.")
        yield stream


def _copy_checked(source, output, pin):
    digest, count = hashlib.sha256(), 0
    while True:
        block = source.read(min(CHUNK, pin["size"] - count + 1))
        if not block:
            break
        count += len(block)
        _require(count <= pin["size"], "Input exceeds pinned size; discard it and reacquire the pinned asset.")
        digest.update(block)
        if output is not None:
            output.write(block)
    _require(count == pin["size"] and digest.hexdigest() == pin["sha256"],
             "Input size/SHA-256 mismatch; discard it and reacquire the pinned asset.")


def verify_binary(path: Path, pin):
    """Verify original supplier bytes and thin Mach-O CPU without executing them."""
    with _regular_file(path) as source:
        _copy_checked(source, None, pin["binary"])
        source.seek(0)
        header = source.read(32)
        _require(len(header) == 32 and header[:4] == b"\xcf\xfa\xed\xfe" and
                 struct.unpack_from("<I", header, 4)[0] == CPUS[pin["binary"]["architecture"]]
                 and struct.unpack_from("<I", header, 12)[0] == 2,
                 "Executable is not the pinned thin Mach-O executable architecture.")


def _public_https(url, hosts):
    parsed = urllib.parse.urlsplit(url)
    return (parsed.scheme == "https" and parsed.hostname in hosts and
            parsed.username is None and parsed.password is None and
            parsed.port in (None, 443) and not parsed.fragment)


class _ReleaseRedirect(urllib.request.HTTPRedirectHandler):
    def redirect_request(self, req, fp, code, msg, headers, newurl):
        _require(_public_https(newurl, {"github.com", "release-assets.githubusercontent.com"}),
                 "Release redirected outside approved HTTPS hosts; acquisition refused.")
        return super().redirect_request(req, fp, code, msg, headers, newurl)


def _tls_context():
    # Raw SSLContext does not opt into SSLKEYLOGFILE. Explicit compiled-in trust
    # paths avoid the SSL_CERT_FILE/DIR overrides used by default-cert loading.
    context = ssl.SSLContext(ssl.PROTOCOL_TLS_CLIENT)
    context.minimum_version = ssl.TLSVersion.TLSv1_2
    defaults = ssl.get_default_verify_paths()
    cafile = defaults.openssl_cafile
    capath = defaults.openssl_capath
    cafile = cafile if cafile and Path(cafile).is_file() else None
    capath = capath if capath and Path(capath).is_dir() else None
    _require(cafile is not None or capath is not None,
             "No compiled-in TLS trust location is available; use the explicit local archive route.")
    context.load_verify_locations(cafile=cafile, capath=capath)
    return context


def _download(pin, destination):
    # No proxy, credentials, auth/TLS environment configuration or cache discovery.
    try:
        opener = urllib.request.build_opener(
            urllib.request.ProxyHandler({}), _ReleaseRedirect(),
            urllib.request.HTTPSHandler(context=_tls_context()))
        deadline = time.monotonic() + TRANSFER_SECONDS
        with opener.open(pin["url"], timeout=30) as response, destination.open("xb") as output:
            _require(response.status == 200, "Release download did not return HTTP 200.")
            if response.headers.get("Content-Length") is not None:
                _require(response.headers["Content-Length"] == str(pin["size"]),
                         "Release Content-Length disagrees with the pin.")

            class TimedSource:
                def read(self, size):
                    _require(time.monotonic() < deadline, "Release download time bound exceeded; retry explicitly.")
                    # read1 prevents a slow peer from extending one read indefinitely.
                    return response.read1(min(size, 64 * 1024))

            _copy_checked(TimedSource(), output, pin)
    except ResourceError:
        raise
    except Exception:
        raise ResourceError("Release download failed; check public HTTPS connectivity and retry explicitly.") from None


class _BoundedTar:
    def __init__(self, source, limit):
        self.source, self.remaining = source, limit

    def read(self, size):
        block = self.source.read(min(size, self.remaining + 1))
        self.remaining -= len(block)
        _require(self.remaining >= 0, "Archive expands beyond the pinned extraction bound.")
        return block


def _extract(archive, destination, pin):
    found = False
    # No extract/extractall: only byte-copy the one exact regular member. Reject
    # all other headers, including links, directories, traversal and duplicates.
    with archive.open("rb") as raw, gzip.GzipFile(fileobj=raw) as decompressed:
        bounded = _BoundedTar(decompressed, pin["binary"]["size"] + 64 * 1024)
        with tarfile.open(fileobj=bounded, mode="r|") as contents:
            for entry in contents:
                _require(not found and entry.name == pin["archive"]["member"] and
                         entry.type in (tarfile.REGTYPE, tarfile.AREGTYPE) and
                         entry.size == pin["binary"]["size"] and not entry.pax_headers,
                         "Archive must contain only the exact pinned regular executable member.")
                found = True
                with contents.extractfile(entry) as source, destination.open("xb") as output:
                    _copy_checked(source, output, pin["binary"])
                    output.flush()
                    os.fsync(output.fileno())
            # Drain within the same bound, checking gzip integrity/truncation.
            while bounded.read(CHUNK):
                pass
    _require(found, "Archive is missing the pinned executable member.")
    verify_binary(destination, pin)


def acquire(pin, output: Path, archive: Path | None = None, *, download=False):
    _require((archive is not None) != download, "Choose one explicit archive or download; no fallback exists.")
    parent = output.parent.resolve(strict=True)
    _require(parent.is_dir() and output.name not in ("", ".", ".."), "Output needs an existing destination directory.")
    destination = parent / output.name
    _require(not os.path.lexists(destination), "Output already exists; choose a fresh path. Nothing was replaced.")
    with tempfile.TemporaryDirectory(prefix=".agent-supplier-", dir=parent) as temporary:
        stage = Path(temporary)
        os.chmod(stage, 0o700)
        staged_archive, staged_binary = stage / "archive.tar.gz", stage / "supplier"
        if download:
            _download(pin["archive"], staged_archive)
        else:
            with _regular_file(archive) as source, staged_archive.open("xb") as target:
                _copy_checked(source, target, pin["archive"])
        _extract(staged_archive, staged_binary, pin)
        os.chmod(staged_binary, 0o755)
        # link fails atomically if any file/link raced us to the destination.
        os.link(staged_binary, destination, follow_symlinks=False)
    return destination


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--manifest", type=Path, default=Path(__file__).resolve().parents[1] /
                        "apps/desktop/src-tauri/resources/agent-supplier-pins.json")
    parser.add_argument("--target", required=True)
    mode = parser.add_mutually_exclusive_group(required=True)
    mode.add_argument("--archive", type=Path)
    mode.add_argument("--download", action="store_true")
    mode.add_argument("--verify-binary", type=Path)
    parser.add_argument("--output", type=Path)
    args = parser.parse_args(argv)
    try:
        pin = load_pin(args.manifest, args.target)
        if args.verify_binary is not None:
            _require(args.output is None, "Verification does not publish; omit --output.")
            verify_binary(args.verify_binary, pin)
        else:
            _require(args.output is not None, "Acquisition requires an explicit fresh --output file.")
            acquire(pin, args.output, args.archive, download=args.download)
    except ResourceError as error:
        print("Supplier resource refused: " + str(error))
        return 1
    except (OSError, ValueError, KeyError, TypeError, EOFError, tarfile.TarError, zlib.error):
        # Do not print exception strings: urllib/OS errors can contain paths,
        # redirected signed URLs, or untrusted archive content.
        print("Supplier resource refused. Check the reviewed manifest, exact input size/hash, "
              "single regular archive member, architecture, and a writable fresh output path.")
        return 1
    print("Pinned supplier bytes and architecture verified; no supplier was executed.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
