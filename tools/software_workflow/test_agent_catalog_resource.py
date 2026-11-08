"""The bundled runtime catalog is the one authority, contained, bounded and pinned."""
import hashlib
import importlib.util
import json
from pathlib import Path
import subprocess
import sys

import pytest

ROOT = Path(__file__).resolve().parents[2]
PROJECT = ROOT / "."
spec = importlib.util.spec_from_file_location("agent_catalog_resource", PROJECT / "tools/agent_catalog_resource.py")
resource = importlib.util.module_from_spec(spec)
spec.loader.exec_module(resource)


def write(project, data):
    target = project / resource.RESOURCE
    target.parent.mkdir(parents=True, exist_ok=True)
    target.write_bytes(data)
    return hashlib.sha256(data).hexdigest()


def test_registered_project_check_admits_the_bundled_resource():
    # Execute the registered check, not a duplicate implementation of it.
    profile = json.loads((PROJECT / "software-workflow.json").read_text())
    check = profile["checks"]["agent-catalog-resource"]
    command = [sys.executable, *check["command"][1:]]
    done = subprocess.run(command, cwd=PROJECT / check["cwd"], capture_output=True, text=True)
    assert done.returncode == 0, done.stdout + done.stderr


def test_pin_drift_absent_and_malformed_resources_fail_closed(tmp_path):
    data = b'{"tools": [{"name": "invented"}]}\r\n'
    pin = write(tmp_path, data)
    assert resource.validate(tmp_path, pin)["tools"] == [{"name": "invented"}]
    with pytest.raises(ValueError, match="pin"):
        resource.validate(tmp_path, "0" * 64)
    for bad in (b"", b"x" * (resource.MAX_BYTES + 1), b'{"tools": [], "tools": [1]}', b'{"tools": [NaN]}',
                b'{"tools": []}', b"[]", b"\xff"):
        with pytest.raises((ValueError, UnicodeError)):
            resource.validate(tmp_path, write(tmp_path, bad))
    (tmp_path / resource.RESOURCE).unlink()
    with pytest.raises(ValueError):
        resource.validate(tmp_path, pin)


def test_linked_resource_or_parent_cannot_supply_the_catalog(tmp_path):
    project, outside = tmp_path / "project", tmp_path / "outside"
    outside.mkdir()
    data = b'{"tools": [{"name": "invented"}]}'
    (outside / "agent-capability-catalog.json").write_bytes(data)
    pin = hashlib.sha256(data).hexdigest()
    target = project / resource.RESOURCE
    target.parent.mkdir(parents=True)
    target.symlink_to(outside / "agent-capability-catalog.json")
    with pytest.raises(ValueError):
        resource.validate(project, pin)
    target.unlink()
    target.parent.rmdir()
    target.parent.symlink_to(outside, target_is_directory=True)
    with pytest.raises(ValueError):
        resource.validate(project, pin)


def test_both_selectors_route_the_catalog_and_its_pin():
    from select_affected_checks import select_checks
    root_profile = json.loads((ROOT / "tools/tools-test-routing.json").read_text())
    project_profile = json.loads((PROJECT / "software-workflow.json").read_text())
    for name in [resource.RESOURCE, Path("tools/agent_catalog_resource.py"), Path("tools/agent_host_witness.py")]:
        assert "agent-catalog-resource" in select_checks(project_profile, [name.as_posix()])["checks"]
        assert "software_workflow" in select_checks(root_profile, [name.as_posix()])["checks"]
