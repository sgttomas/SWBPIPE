#!/usr/bin/env python3
"""Offline bundle-layout and refusal tests using small invented Mach-O bytes."""
import copy
import hashlib
import io
import json
from pathlib import Path
import struct
import tarfile
import tempfile
import unittest
from unittest.mock import patch

import agent_supplier_bundle as bundle
import agent_supplier_resource as resource


def digest(data):
    return {"size": len(data), "sha256": hashlib.sha256(data).hexdigest()}


class BundleTest(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory()
        self.addCleanup(self.temporary.cleanup)
        self.root = Path(self.temporary.name).resolve()
        self.project = self.root / "project"
        self.project.mkdir()
        self.manifest = self.project / "pins.json"
        self.target = bundle.TARGETS[0]
        self.member = "codex-" + self.target
        self.binary = struct.pack("<8I", 0xFEEDFACF, resource.CPUS["arm64"], 0, 2, 0, 0, 0, 0) + b"invented"
        self.source = self.root / "binary"
        self.source.write_bytes(self.binary)
        self.source.chmod(0o555)
        data = io.BytesIO()
        with tarfile.open(fileobj=data, mode="w:gz") as archive:
            entry = tarfile.TarInfo(self.member)
            entry.size = len(self.binary)
            archive.addfile(entry, io.BytesIO(self.binary))
        self.archive = self.root / "archive.tar.gz"
        self.archive.write_bytes(data.getvalue())
        pin = {"binary": {**digest(self.binary), "resource_name": self.member,
                          "architecture": "arm64", "format": "mach-o-64-le"},
               "archive": {**digest(data.getvalue()), "member": self.member, "format": "tar.gz",
                           "url": "https://github.com/openai/codex/releases/download/rust-v0.160.0/" + self.member + ".tar.gz"}}
        files = []
        for name, relative in bundle.LICENSES.items():
            path = self.project / relative
            path.parent.mkdir(parents=True, exist_ok=True)
            text = (name + " invented bytes\n").encode()
            path.write_bytes(text)
            files.append({"upstream_name": name, "path": relative, **digest(text)})
        (self.project / "docs/THIRD_PARTY_NOTICES.md").write_text("[LICENSE](third-party/codex-0.160.0-LICENSE.txt)\n")
        self.doc = {"schema": "swbpipe-agent-supplier-pins/1", "supplier": "openai/codex", "version": "0.160.0",
                    "targets": {self.target: pin}, "licensing": {"files": files}}
        self.write_manifest()
        self.output = self.root / "output"

    def write_manifest(self):
        self.manifest.write_text(json.dumps(self.doc))

    def prepare(self, **kwargs):
        return bundle.prepare(self.output, self.target, manifest=self.manifest, project=self.project, **kwargs)

    def verify(self):
        bundle.verify(self.output / "resources", self.target, manifest=self.manifest, project=self.project)

    def test_exact_layout_overlay_and_original_copy(self):
        overlay = self.prepare(binary=self.source)
        resources = self.output / "resources"
        mapping = json.loads(overlay.read_text())
        expected = {"agent-supplier/" + self.member, "agent-supplier/pins.json", "THIRD_PARTY_NOTICES.md",
                    "third-party/codex-0.160.0-LICENSE.txt", "third-party/codex-0.160.0-NOTICE.txt"}
        self.assertEqual(mapping, {"bundle": {"resources": {str(resources / p): p for p in expected}}})
        self.assertEqual(self.output.stat().st_mode & 0o777, 0o700)
        self.assertEqual((resources / "agent-supplier/pins.json").read_bytes(), self.manifest.read_bytes())
        self.assertEqual((resources / "agent-supplier" / self.member).read_bytes(), self.binary)
        self.assertEqual(self.source.read_bytes(), self.binary)
        self.assertEqual(self.source.stat().st_mode & 0o777, 0o555)
        self.verify()

    def test_second_pinned_architecture_is_explicit(self):
        self.target = bundle.TARGETS[1]
        self.member = "codex-" + self.target
        data = struct.pack("<8I", 0xFEEDFACF, resource.CPUS["x86_64"], 0, 2, 0, 0, 0, 0)
        self.source.chmod(0o755)
        self.source.write_bytes(data)
        pin = next(iter(self.doc["targets"].values()))
        pin["binary"].update(digest(data), architecture="x86_64", resource_name=self.member)
        pin["archive"].update(member=self.member, url="https://github.com/openai/codex/releases/download/rust-v0.160.0/" + self.member + ".tar.gz")
        self.doc["targets"] = {self.target: pin}
        self.write_manifest()
        self.prepare(binary=self.source)
        self.verify()
        self.assertTrue((self.output / "resources/agent-supplier" / self.member).is_file())

    def test_explicit_archive_and_download_routes(self):
        self.prepare(archive=self.archive)
        self.verify()
        other = self.root / "download"
        def download(pin, destination):
            destination.write_bytes(self.archive.read_bytes())
        with patch.object(resource, "_download", side_effect=download) as network:
            bundle.prepare(other, self.target, download=True, manifest=self.manifest, project=self.project)
        network.assert_called_once()

    def test_no_implicit_source_network_or_fallback(self):
        with patch.object(resource, "_download", side_effect=AssertionError("network")):
            for kwargs in ({}, {"binary": self.source, "download": True}, {"archive": self.archive, "binary": self.source}):
                with self.assertRaises(bundle.ResourceError):
                    self.prepare(**kwargs)
                self.assertFalse(self.output.exists())
            with self.assertRaises(bundle.ResourceError):
                bundle.prepare(self.output, "unsupported", binary=self.source, manifest=self.manifest, project=self.project)

    def test_source_corruption_and_cpu_refuse_without_publication(self):
        self.source.chmod(0o755)
        for data in (self.binary + b"x", self.binary[:-1], b"x" * len(self.binary)):
            self.source.write_bytes(data)
            with self.assertRaises(bundle.ResourceError):
                self.prepare(binary=self.source)
            self.assertFalse(self.output.exists())
        wrong = struct.pack("<8I", 0xFEEDFACF, resource.CPUS["x86_64"], 0, 2, 0, 0, 0, 0)
        self.source.write_bytes(wrong)
        self.doc["targets"][self.target]["binary"].update(digest(wrong))
        self.write_manifest()
        with self.assertRaises(bundle.ResourceError):
            self.prepare(binary=self.source)

    def test_missing_and_linked_sources_refused(self):
        self.source.unlink()
        with self.assertRaises(bundle.ResourceError):
            self.prepare(binary=self.source)
        self.source.symlink_to(self.archive)
        with self.assertRaises(bundle.ResourceError):
            self.prepare(binary=self.source)
        self.assertFalse(self.output.exists())

    def test_license_path_pin_missing_and_link_refusal(self):
        original = copy.deepcopy(self.doc)
        for path in ("../outside", "/absolute", "docs/third-party/../outside"):
            self.doc = copy.deepcopy(original)
            self.doc["licensing"]["files"][0]["path"] = path
            self.write_manifest()
            with self.assertRaises(bundle.ResourceError):
                self.prepare(binary=self.source)
        self.doc = original
        self.write_manifest()
        license_path = self.project / bundle.LICENSES["LICENSE"]
        license_path.write_bytes(b"wrong")
        with self.assertRaises(bundle.ResourceError):
            self.prepare(binary=self.source)
        license_path.unlink()
        with self.assertRaises(bundle.ResourceError):
            self.prepare(binary=self.source)
        license_path.symlink_to(self.source)
        with self.assertRaises(bundle.ResourceError):
            self.prepare(binary=self.source)
        self.assertFalse(self.output.exists())

    def test_existing_and_raced_outputs_preserved(self):
        self.output.write_text("keep")
        with self.assertRaises(bundle.ResourceError):
            self.prepare(binary=self.source)
        self.assertEqual(self.output.read_text(), "keep")
        self.output.unlink()
        original = Path.mkdir
        def race(path, *args, **kwargs):
            if path == self.output:
                path.write_text("racer")
            return original(path, *args, **kwargs)
        with patch.object(Path, "mkdir", race):
            with self.assertRaises(bundle.ResourceError):
                self.prepare(binary=self.source)
        self.assertEqual(self.output.read_text(), "racer")
        self.output.unlink()
        self.output.symlink_to(self.root / "missing")
        with self.assertRaises(bundle.ResourceError):
            self.prepare(binary=self.source)
        self.assertTrue(self.output.is_symlink())

    def test_every_bundle_resource_tamper_missing_and_link_refused(self):
        self.prepare(binary=self.source)
        resources = self.output / "resources"
        for path in list(resources.rglob("*")):
            if not path.is_file():
                continue
            original = path.read_bytes()
            for data in (b"changed", original + b"x"):
                path.write_bytes(data)
                with self.assertRaises(bundle.ResourceError):
                    self.verify()
            path.unlink()
            with self.assertRaises(bundle.ResourceError):
                self.verify()
            path.symlink_to(self.source)
            with self.assertRaises(bundle.ResourceError):
                self.verify()
            path.unlink()
            path.write_bytes(original)
        directory = resources / "agent-supplier"
        directory.rename(resources / "saved")
        directory.symlink_to(resources / "saved", target_is_directory=True)
        with self.assertRaises(bundle.ResourceError):
            self.verify()

    def test_writable_bundle_input_is_refused_like_the_runtime_loader(self):
        self.prepare(binary=self.source)
        binary = self.output / 'resources/agent-supplier' / self.member
        binary.chmod(0o666)
        with self.assertRaises(bundle.ResourceError):
            self.verify()
        binary.chmod(0o755)
        self.verify()

    def test_committed_licenses_correspond_to_both_pins(self):
        for target in bundle.TARGETS:
            pin, payloads = bundle._inputs(bundle.DEFAULT_MANIFEST, target, bundle.PROJECT)
            self.assertEqual(pin["binary"]["resource_name"], "codex-" + target)
            self.assertEqual(len(payloads), 4)

    def test_cli_static_failure_has_no_path_or_error_details(self):
        from contextlib import redirect_stdout
        output = io.StringIO()
        with redirect_stdout(output):
            status = bundle.main(["--target", self.target, "prepare", "--binary", str(self.root / "secret"), "--output", str(self.output)])
        self.assertEqual(status, 1)
        self.assertNotIn(str(self.root), output.getvalue())


if __name__ == "__main__":
    unittest.main()
