"""The runtime catalog has one authority even in sparse/export builds."""
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


def test_registered_project_check_matches_authoritative_bytes():
    # This suite runs in the existing full-checkout Repository checks job.
    # Execute the registered check, not a duplicate implementation of it.
    profile = json.loads((PROJECT / "software-workflow.json").read_text())
    check = profile["checks"]["agent-catalog-resource"]
    command = [sys.executable, *check["command"][1:]]
    done = subprocess.run(command, cwd=PROJECT / check["cwd"], capture_output=True, text=True)
    assert done.returncode == 0, done.stdout + done.stderr


def test_generation_preserves_bytes_and_check_rejects_drift(tmp_path):
    source, target = tmp_path / resource.SOURCE, tmp_path / resource.RESOURCE
    source.parent.mkdir(parents=True)
    data = b'{"invented": true}\r\n'
    source.write_bytes(data)
    assert resource.synchronize(tmp_path, check=False)
    assert target.read_bytes() == data
    assert resource.synchronize(tmp_path, check=True)
    target.write_bytes(data.replace(b"true", b"false"))
    assert not resource.synchronize(tmp_path, check=True)
    source.unlink()
    with pytest.raises(OSError):
        resource.synchronize(tmp_path, check=True)


def test_absent_oversize_and_linked_inputs_fail_closed(tmp_path):
    source, target = tmp_path / resource.SOURCE, tmp_path / resource.RESOURCE
    source.parent.mkdir(parents=True)
    source.write_bytes(b"invented")
    with pytest.raises(OSError):
        resource.synchronize(tmp_path, check=True)
    source.write_bytes(b"x" * (resource.MAX_BYTES + 1))
    with pytest.raises(ValueError):
        resource.synchronize(tmp_path, check=False)
    source.unlink()
    actual = tmp_path / "invented.json"
    actual.write_bytes(b"invented")
    source.symlink_to(actual)
    with pytest.raises(ValueError):
        resource.synchronize(tmp_path, check=False)
    source.unlink()
    source.write_bytes(b"invented")
    target.parent.mkdir(parents=True)
    target.symlink_to(actual)
    for check in [False, True]:
        with pytest.raises(ValueError):
            resource.synchronize(tmp_path, check=check)


def test_both_selectors_route_all_catalog_authority_inputs():
    # A future contract-only edit must execute the full-checkout equality test.
    from select_affected_checks import select_checks
    root_profile = json.loads((ROOT / "tools/tools-test-routing.json").read_text())
    project_profile = json.loads((PROJECT / "software-workflow.json").read_text())
    for name in [resource.SOURCE, resource.RESOURCE, Path("tools/agent_catalog_resource.py")]:
        assert "agent-catalog-resource" in select_checks(project_profile, [name.as_posix()])["checks"]
        path = (Path(".") / name).as_posix()
        assert "software_workflow" in select_checks(root_profile, [path])["checks"]


def test_parent_link_cannot_redirect_generation_outside_project(tmp_path):
    project = tmp_path / "project"
    outside = tmp_path / "outside"
    project.mkdir()
    outside.mkdir()
    source = project / resource.SOURCE
    source.parent.mkdir(parents=True)
    source.write_bytes(b"invented")
    target = project / resource.RESOURCE
    target.parent.parent.mkdir(parents=True)
    target.parent.symlink_to(outside, target_is_directory=True)
    with pytest.raises(ValueError):
        resource.synchronize(project, check=False)
    assert not list(outside.iterdir())
