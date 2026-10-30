#!/usr/bin/env python3
"""Offline invented-data publication/adversarial tests; no supplier execution."""
import copy
import gzip
import hashlib
import io
import json
import os
import ssl
from pathlib import Path
import struct
import tarfile
import tempfile
import unittest
from unittest.mock import patch

import agent_supplier_resource as resource


def digest(data):
    return {"size": len(data), "sha256": hashlib.sha256(data).hexdigest()}


def archive_bytes(entries):
    result = io.BytesIO()
    with tarfile.open(fileobj=result, mode="w:gz", format=tarfile.USTAR_FORMAT) as archive:
        for name, data, kind in entries:
            member = tarfile.TarInfo(name)
            member.type = kind
            member.size = len(data) if kind == tarfile.REGTYPE else 0
            if kind in (tarfile.SYMTYPE, tarfile.LNKTYPE):
                member.linkname = "outside"
            archive.addfile(member, io.BytesIO(data) if member.size else None)
    return result.getvalue()


class SupplierResourceTest(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        self.member = "codex-aarch64-apple-darwin"
        self.binary = struct.pack("<8I", 0xFEEDFACF, resource.CPUS["arm64"], 0, 2, 0, 0, 0, 0) + b"invented payload"
        self.archive = archive_bytes([(self.member, self.binary, tarfile.REGTYPE)])
        self.pin = {
            "archive": {**digest(self.archive), "member": self.member, "format": "tar.gz",
                        "url": "https://github.com/openai/codex/releases/download/rust-v0.160.0/" + self.member + ".tar.gz"},
            "binary": {**digest(self.binary), "architecture": "arm64", "format": "mach-o-64-le", "resource_name": self.member},
        }
        self.source = self.root / "source.tar.gz"
        self.source.write_bytes(self.archive)
        self.output = self.root / "published"

    def acquire(self):
        return resource.acquire(self.pin, self.output, self.source)

    def assert_refused(self):
        with self.assertRaises((resource.ResourceError, tarfile.TarError, EOFError, OSError)):
            self.acquire()
        self.assertFalse(self.output.exists())
        self.assertEqual(list(self.root.glob(".agent-supplier-*")), [])

    def use_archive(self, entries):
        self.archive = archive_bytes(entries)
        self.source.write_bytes(self.archive)
        self.pin["archive"].update(digest(self.archive))

    def test_verified_publication_and_private_stage(self):
        original = resource._extract

        def inspect(archive, destination, pin):
            self.assertEqual(destination.parent.stat().st_mode & 0o777, 0o700)
            self.assertFalse(self.output.exists())
            return original(archive, destination, pin)

        with patch.object(resource, "_extract", side_effect=inspect):
            self.assertEqual(self.acquire(), self.output.resolve())
        self.assertEqual(self.output.read_bytes(), self.binary)
        self.assertEqual(self.output.stat().st_mode & 0o777, 0o755)
        self.assertEqual(list(self.root.glob(".agent-supplier-*")), [])

    def test_archive_hash_size_and_truncation(self):
        for data in (self.archive + b"x", self.archive[:-1], b"x" * len(self.archive)):
            with self.subTest(length=len(data)):
                self.source.write_bytes(data)
                self.assert_refused()

    def test_binary_hash_mismatch(self):
        self.pin["binary"]["sha256"] = "0" * 64
        self.assert_refused()

    def test_architecture_and_executable_kind(self):
        for cpu, kind in ((resource.CPUS["x86_64"], 2), (resource.CPUS["arm64"], 6)):
            with self.subTest(cpu=cpu, kind=kind):
                binary = struct.pack("<8I", 0xFEEDFACF, cpu, 0, kind, 0, 0, 0, 0)
                self.pin["binary"].update(digest(binary))
                self.use_archive([(self.member, binary, tarfile.REGTYPE)])
                self.assert_refused()

    def test_no_links_paths_devices_or_extra_members(self):
        entries = [
            ("../outside", self.binary, tarfile.REGTYPE),
            ("/absolute", self.binary, tarfile.REGTYPE),
            ("nested/" + self.member, self.binary, tarfile.REGTYPE),
            (self.member, b"", tarfile.SYMTYPE),
            (self.member, b"", tarfile.LNKTYPE),
            (self.member, b"", tarfile.CHRTYPE),
            (self.member, b"", tarfile.DIRTYPE),
        ]
        for entry in entries:
            with self.subTest(entry=entry[:1], kind=entry[2]):
                self.use_archive([entry])
                self.assert_refused()
        for entry in [entries[0], (self.member, self.binary, tarfile.REGTYPE)]:
            self.use_archive([(self.member, self.binary, tarfile.REGTYPE), entry])
            self.assert_refused()
        self.assertFalse((self.root / "outside").exists())

    def test_missing_wrong_size_and_expansion_bound(self):
        self.use_archive([])
        self.assert_refused()
        self.use_archive([(self.member, self.binary + b"x", tarfile.REGTYPE)])
        self.assert_refused()
        self.source.write_bytes(gzip.compress(b"\0" * (128 * 1024)))
        self.pin["archive"].update(digest(self.source.read_bytes()))
        self.assert_refused()

    def test_corrupt_gzip_after_valid_member(self):
        damaged = bytearray(self.archive)
        damaged[-5] ^= 1
        self.source.write_bytes(damaged)
        self.pin["archive"].update(digest(damaged))
        self.assert_refused()

    def test_existing_and_raced_output_preserved(self):
        self.output.write_bytes(b"unrelated")
        with self.assertRaises(resource.ResourceError):
            self.acquire()
        self.assertEqual(self.output.read_bytes(), b"unrelated")
        self.output.unlink()
        original = os.link

        def race(source, destination, **kwargs):
            self.output.write_bytes(b"raced")
            return original(source, destination, **kwargs)

        with patch.object(resource.os, "link", side_effect=race), self.assertRaises(FileExistsError):
            self.acquire()
        self.assertEqual(self.output.read_bytes(), b"raced")

    def test_symlink_input_and_destination_refused(self):
        link = self.root / "source-link"
        link.symlink_to(self.source)
        with self.assertRaises(OSError):
            resource.acquire(self.pin, self.output, link)
        self.output.symlink_to(self.root / "absent")
        with self.assertRaises(resource.ResourceError):
            self.acquire()
        self.assertTrue(self.output.is_symlink())

    def test_explicit_source_required(self):
        for archive, download in ((None, False), (self.source, True)):
            with self.assertRaises(resource.ResourceError):
                resource.acquire(self.pin, self.output, archive, download=download)

    def test_download_bounds_hash_and_no_proxy(self):
        class Response(io.BytesIO):
            status = 200
            headers = {}

        for body, valid in ((self.archive, True), (self.archive + b"x", False), (b"bad", False)):
            with self.subTest(valid=valid), patch.object(resource.urllib.request, "build_opener") as build:
                build.return_value.open.return_value = Response(body)
                if valid:
                    resource.acquire(self.pin, self.output, download=True)
                    self.assertEqual(self.output.read_bytes(), self.binary)
                    self.output.unlink()
                else:
                    with self.assertRaises(resource.ResourceError):
                        resource.acquire(self.pin, self.output, download=True)
                    self.assertFalse(self.output.exists())
                self.assertEqual(build.call_args.args[0].proxies, {})

    def test_download_tls_ignores_environment_without_disabling_verification(self):
        invalid_ca = self.root / 'environment-ca.pem'
        invalid_ca.write_text('not a certificate')
        ca_directory = self.root / 'environment-ca-directory'
        ca_directory.mkdir()
        original_load = ssl.SSLContext.load_verify_locations
        test = self

        class Response(io.BytesIO):
            status = 200
            headers = {}

        for index, overrides in enumerate([
                {'SSLKEYLOGFILE': str(self.root / 'must-not-log')},
                {'SSL_CERT_FILE': str(invalid_ca)},
                {'SSL_CERT_DIR': str(ca_directory)},
                {'SSLKEYLOGFILE': str(self.root / 'must-not-log-together'),
                 'SSL_CERT_FILE': str(invalid_ca), 'SSL_CERT_DIR': str(ca_directory)}]):
            with self.subTest(variables=sorted(overrides)):
                contexts, locations = [], []

                def record_load(context, cafile=None, capath=None, cadata=None):
                    locations.append((cafile, capath))
                    return original_load(context, cafile, capath, cadata)

                def open_without_network(opener, url, timeout):
                    https = next(handler for handler in opener.handlers
                                 if isinstance(handler, resource.urllib.request.HTTPSHandler))
                    # This is the HTTPSConnection default when a handler supplies
                    # no context. Exercise it offline, without a TLS handshake.
                    context = https._context or ssl._create_default_https_context()
                    contexts.append(context)
                    return Response(test.archive)

                with patch.dict(os.environ, overrides, clear=True), patch.object(
                        ssl.SSLContext, 'load_verify_locations', record_load), patch.object(
                        resource.urllib.request.OpenerDirector, 'open', open_without_network):
                    resource._download(self.pin['archive'], self.root / f'download-{index}.tar.gz')
                    self.assertEqual(dict(os.environ), overrides)
                    context = contexts[0]
                    self.assertIsNone(context.keylog_filename)
                    self.assertTrue(context.check_hostname)
                    self.assertEqual(context.verify_mode, ssl.CERT_REQUIRED)
                    self.assertGreaterEqual(context.minimum_version, ssl.TLSVersion.TLSv1_2)
                    self.assertTrue(locations, 'trust must be loaded from explicit compiled locations')
                    defaults = ssl.get_default_verify_paths()
                    self.assertTrue(all(cafile in (None, defaults.openssl_cafile) and
                                        capath in (None, defaults.openssl_capath)
                                        for cafile, capath in locations))
                for name in ('must-not-log', 'must-not-log-together'):
                    self.assertFalse((self.root / name).exists())

    def test_missing_compiled_trust_refuses_before_network_or_publication(self):
        defaults = ssl.get_default_verify_paths()._replace(openssl_cafile=None, openssl_capath=None)
        with patch.object(ssl, 'get_default_verify_paths', return_value=defaults), patch.object(
                resource.urllib.request.OpenerDirector, 'open') as opened:
            with self.assertRaisesRegex(resource.ResourceError, 'compiled-in TLS trust'):
                resource._download(self.pin['archive'], self.root / 'download.tar.gz')
            opened.assert_not_called()
        self.assertFalse((self.root / 'download.tar.gz').exists())

    def test_network_failure_no_fallback_or_publication(self):
        with patch.object(resource.urllib.request, "build_opener") as build:
            build.return_value.open.side_effect = OSError("sensitive response")
            with self.assertRaisesRegex(resource.ResourceError, "check public HTTPS") as error:
                resource.acquire(self.pin, self.output, download=True)
        self.assertNotIn("sensitive", str(error.exception))
        self.assertFalse(self.output.exists())

    def test_redirects_and_deadline(self):
        handler = resource._ReleaseRedirect()
        for url in ("http://github.com/file", "https://example.com/file", "https://user@github.com/file"):
            with self.assertRaises(resource.ResourceError):
                handler.redirect_request(None, None, 302, "", {}, url)
        class Response(io.BytesIO):
            status = 200
            headers = {}
        with patch.object(resource.urllib.request, "build_opener") as build, patch.object(resource.time, "monotonic", side_effect=[0, 601]):
            build.return_value.open.return_value = Response(self.archive)
            with self.assertRaisesRegex(resource.ResourceError, "time bound"):
                resource.acquire(self.pin, self.output, download=True)
        self.assertFalse(self.output.exists())

    def test_committed_dual_arch_pins_and_notice_bytes(self):
        root = Path(__file__).resolve().parents[1]
        manifest = root / 'apps/desktop/src-tauri/resources/agent-supplier-pins.json'
        doc = json.loads(manifest.read_text())
        self.assertEqual(set(doc['targets']), {'aarch64-apple-darwin', 'x86_64-apple-darwin'})
        for target in doc['targets']:
            resource.load_pin(manifest, target)
        self.assertEqual(doc['licensing']['expression'], 'Apache-2.0')
        for entry in doc['licensing']['files']:
            data = (root / entry['path']).read_bytes()
            self.assertEqual(digest(data), {'size': entry['size'], 'sha256': entry['sha256']})

    def test_manifest_validation(self):
        doc = {"schema": "swbpipe-agent-supplier-pins/1", "supplier": "openai/codex", "version": "0.160.0", "targets": {"aarch64-apple-darwin": self.pin}}
        manifest = self.root / "manifest.json"
        manifest.write_text(json.dumps(doc))
        self.assertEqual(resource.load_pin(manifest, "aarch64-apple-darwin"), self.pin)
        with self.assertRaises(resource.ResourceError):
            resource.load_pin(manifest, "not-pinned")
        for invalid in ([], {**doc, "targets": []}, {**doc, "targets": {"aarch64-apple-darwin": None}}):
            manifest.write_text(json.dumps(invalid))
            with self.assertRaises(resource.ResourceError):
                resource.load_pin(manifest, "aarch64-apple-darwin")
        for section, key, value in [("archive", "member", "../escape"), ("archive", "size", True), ("archive", "sha256", "BAD"), ("archive", "url", "https://example.com/alternate"), ("binary", "architecture", "x86_64")]:
            invalid = copy.deepcopy(doc)
            invalid["targets"]["aarch64-apple-darwin"][section][key] = value
            manifest.write_text(json.dumps(invalid))
            with self.assertRaises(resource.ResourceError):
                resource.load_pin(manifest, "aarch64-apple-darwin")


if __name__ == "__main__":
    unittest.main()
