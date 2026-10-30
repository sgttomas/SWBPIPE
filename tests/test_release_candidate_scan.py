#!/usr/bin/env python3
"""Tests for the DEC-058 release-candidate scan runner and record emitter.

All fixture content is invented at test time inside pytest tmp directories:
lookalike shapes only, never protected standards data. No test writes to
`validation/evidence/releases/` — real scan records are emitted only by the
owner under DEC-058; here the output directory is always a temp directory.
"""

from __future__ import annotations

import importlib.util
import json
import shutil
import subprocess
import sys
from pathlib import Path

import pytest


ROOT = Path(__file__).resolve().parents[1]
MODULE_PATH = ROOT / "tools" / "release" / "run_release_candidate_scan.py"

FIXED_NOW = "2026-07-10T12:00:00+00:00"
FIXED_COMMIT = "abcdef0123456789abcdef0123456789abcdef01"

ACCEPTED_PROVENANCE = {
    "source_name": "OpenPipeStress invented example",
    "source_location": "invented/inline",
    "source_license": "invented-public",
    "contributor": "OpenPipeStress",
    "contributor_certification": (
        "Invented non-engineering example; no protected source copied."
    ),
    "redistribution_status": "public_permissive",
    "review_status": "accepted",
}


def load_module():
    spec = importlib.util.spec_from_file_location(
        "run_release_candidate_scan", MODULE_PATH
    )
    assert spec is not None
    assert spec.loader is not None
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    return module


def make_candidate_tree(tmp_path: Path) -> Path:
    candidate = tmp_path / "candidate"
    candidate.mkdir()
    (candidate / "README.md").write_text(
        "Invented release-candidate stub for scan tooling self-test.\n",
        encoding="utf-8",
    )
    (candidate / "example.json").write_text(
        json.dumps(
            {"note": "invented example", "provenance": dict(ACCEPTED_PROVENANCE)}
        ),
        encoding="utf-8",
    )
    return candidate


def run_scan(module, tmp_path: Path, candidate_root: Path, extra=None, **kwargs):
    output_dir = tmp_path / "out"
    argv = [
        "--candidate",
        kwargs.get("candidate", "rc-test"),
        "--artifact-class",
        f"AC-4={candidate_root}",
        "--not-applicable",
        "AC-1=no native package produced by this self-test",
        "--not-applicable",
        "AC-2=no report artifact produced by this self-test",
        "--not-applicable",
        "AC-3=no installers produced (D-06 pending)",
        "--not-applicable",
        "AC-5=no separate example corpus in this self-test",
        "--not-applicable",
        "AC-6=no release notes produced by this self-test",
        "--security-profile",
        "skip",
        "--lint-mode",
        kwargs.get("lint_mode", "skip"),
        "--output-dir",
        str(output_dir),
        "--now-utc",
        FIXED_NOW,
        "--commit",
        FIXED_COMMIT,
        "--execute",
    ]
    if extra:
        argv.extend(extra)
    exit_code = module.main(argv)
    return exit_code, output_dir


def read_single_record(output_dir: Path) -> tuple[Path, dict]:
    records = sorted(output_dir.glob("SCAN_*.json"))
    assert len(records) == 1
    return records[0], json.loads(records[0].read_text(encoding="utf-8"))


def test_plan_mode_writes_nothing(tmp_path, capsys):
    module = load_module()
    exit_code = module.main(
        ["--candidate", "rc-plan", "--output-dir", str(tmp_path / "out")]
    )
    captured = capsys.readouterr()
    assert exit_code == 0
    assert "plan only" in captured.out
    assert "UNSIGNED" in captured.out
    assert not (tmp_path / "out").exists()


def test_record_filename_and_core_fields(tmp_path):
    module = load_module()
    candidate_root = make_candidate_tree(tmp_path)
    exit_code, output_dir = run_scan(module, tmp_path, candidate_root)
    assert exit_code == 0

    record_path, record = read_single_record(output_dir)
    assert record_path.name == "SCAN_rc-test_20260710T120000Z_abcdef012345.json"
    assert record["artifact"] == "openpipestress.release_protected_content_scan_record"
    assert record["decision_basis"] == "DEC-058"
    assert record["candidate"] == "rc-test"
    assert record["git"]["commit_hash"] == FIXED_COMMIT
    assert record["clean_scan_is_clearance"] is False
    assert record["publication_gate"] == "blocked_pending_owner_sign_off"
    assert set(record["checks"]) == {
        "artifact_inventory_walk",
        "protected_content_lint",
        "provenance_manifest_check",
        "security_profile",
        "quarantine_hygiene",
    }
    assert record["known_limitations"]
    assert "not a legal clearance" in record["boundary_note"].lower()


def test_all_six_classes_recorded_never_silently_omitted(tmp_path):
    module = load_module()
    candidate_root = make_candidate_tree(tmp_path)
    _, output_dir = run_scan(module, tmp_path, candidate_root)
    _, record = read_single_record(output_dir)

    assert sorted(record["artifact_classes"]) == [
        "AC-1",
        "AC-2",
        "AC-3",
        "AC-4",
        "AC-5",
        "AC-6",
    ]
    assert record["classes_scanned"] == ["AC-4"]
    assert record["classes_not_applicable"] == [
        "AC-1",
        "AC-2",
        "AC-3",
        "AC-5",
        "AC-6",
    ]
    for class_id in record["classes_not_applicable"]:
        entry = record["artifact_classes"][class_id]
        assert entry["status"] == "not_applicable"
        assert entry["reason"].strip()


def test_missing_class_specification_is_an_error(tmp_path, capsys):
    module = load_module()
    candidate_root = make_candidate_tree(tmp_path)
    exit_code = module.main(
        [
            "--candidate",
            "rc-missing",
            "--artifact-class",
            f"AC-4={candidate_root}",
            "--output-dir",
            str(tmp_path / "out"),
            "--now-utc",
            FIXED_NOW,
            "--commit",
            FIXED_COMMIT,
            "--execute",
        ]
    )
    captured = capsys.readouterr()
    assert exit_code == 2
    assert "never silently omitted" in captured.err
    assert not (tmp_path / "out").exists()


def test_inventory_walk_records_checksums(tmp_path):
    module = load_module()
    candidate_root = make_candidate_tree(tmp_path)
    _, output_dir = run_scan(module, tmp_path, candidate_root)
    _, record = read_single_record(output_dir)

    artifacts = record["artifact_classes"]["AC-4"]["artifacts"]
    assert [artifact["path"] for artifact in artifacts] == [
        "README.md",
        "example.json",
    ]
    for artifact in artifacts:
        assert len(artifact["sha256"]) == 64
        assert artifact["size_bytes"] > 0
        assert artifact["lintable"] is True
    assert record["checks"]["artifact_inventory_walk"]["artifact_count"] == 2


def test_record_is_deterministic_given_fixed_inputs(tmp_path):
    module = load_module()
    candidate_root = make_candidate_tree(tmp_path)
    _, first_dir = run_scan(module, tmp_path / "first", candidate_root)
    _, second_dir = run_scan(module, tmp_path / "second", candidate_root)
    first_path, _ = read_single_record(first_dir)
    second_path, _ = read_single_record(second_dir)
    assert first_path.name == second_path.name
    assert first_path.read_bytes() == second_path.read_bytes()


def test_sign_off_block_is_explicitly_unsigned_and_pending(tmp_path):
    module = load_module()
    candidate_root = make_candidate_tree(tmp_path)
    _, output_dir = run_scan(module, tmp_path, candidate_root)
    _, record = read_single_record(output_dir)

    sign_off = record["owner_sign_off"]
    assert sign_off["signed"] is False
    assert sign_off["status"] == "pending_owner_review"
    assert sign_off["signatory"] is None
    assert sign_off["signed_utc"] is None
    assert sign_off["acceptance_statement"] is None
    assert sign_off["per_finding_dispositions_recorded"] is False
    assert "DEC-027" in sign_off["signatory_role"]
    assert "Tooling never signs" in sign_off["note"]


def test_skipped_checks_fail_toward_owner_review(tmp_path):
    module = load_module()
    candidate_root = make_candidate_tree(tmp_path)
    _, output_dir = run_scan(module, tmp_path, candidate_root)
    _, record = read_single_record(output_dir)

    assert record["checks"]["security_profile"]["status"] == "not_run"
    assert record["checks"]["protected_content_lint"]["status"] == "not_run"
    assert record["scan_disposition"] == "incomplete_checks_require_owner_review"


def test_provenance_violations_are_blocking_findings(tmp_path):
    module = load_module()
    candidate_root = make_candidate_tree(tmp_path)
    bad_provenance = dict(ACCEPTED_PROVENANCE)
    bad_provenance["redistribution_status"] = "unknown"
    bad_provenance["source_license"] = "TBD"
    (candidate_root / "unreviewed.json").write_text(
        json.dumps({"provenance": bad_provenance}), encoding="utf-8"
    )

    _, output_dir = run_scan(module, tmp_path, candidate_root)
    _, record = read_single_record(output_dir)

    provenance_check = record["checks"]["provenance_manifest_check"]
    assert provenance_check["status"] == "findings_present"
    assert provenance_check["provenance_records_checked"] == 2
    assert any("TBD" in violation for violation in provenance_check["violations"])

    blocking = [
        finding
        for finding in record["findings"]
        if finding["check"] == "provenance_manifest_check"
    ]
    assert blocking
    assert all(finding["severity"] == "BLOCKING" for finding in blocking)
    assert all(
        finding["disposition"] == "pending_owner_review" for finding in blocking
    )
    assert record["scan_disposition"] == "findings_require_owner_review"
    assert record["blocking_finding_count"] >= 1


@pytest.mark.parametrize("licence", [{"spdx": "MIT"}, None, 7])
def test_non_string_licence_is_a_violation_not_a_crash(licence):
    # The staged export's dry run aborted with AttributeError on a record whose
    # source_license was an object; the check must record it and continue.
    module = load_module()
    record = dict(ACCEPTED_PROVENANCE)
    record["source_license"] = licence
    violations = module.provenance_record_violations("$.provenance", record)
    assert violations == ["$.provenance: missing or empty 'source_license'"]


def test_schema_property_maps_are_not_provenance_records():
    # The staged export ships JSON Schemas whose $defs.Provenance.properties
    # declares the provenance fields; those subschemas are not data records.
    module = load_module()
    schema = {
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "$defs": {
            "Provenance": {
                "type": "object",
                "properties": {
                    "redistribution_status": {"type": "string"},
                    "source_license": {"type": "string"},
                },
            }
        },
        "examples": [{"provenance": dict(ACCEPTED_PROVENANCE)}],
        "properties": {"redistribution_status": "unknown"},
    }
    paths = [path for path, _ in module.iter_provenance_records(schema, in_schema=True)]
    # The example record and a malformed string-valued record are still checked.
    assert paths == ["$.examples[0].provenance", "$.properties"]


def test_property_map_outside_a_json_schema_is_still_checked(tmp_path):
    # Review finding 4: any `.properties` object with an object-valued
    # redistribution_status escaped validation, even outside a JSON Schema.
    module = load_module()
    data = {
        "properties": {
            "source_name": "invented",
            "source_location": "invented/inline",
            "source_license": "TBD",
            "contributor": "invented",
            "contributor_certification": "invented",
            "redistribution_status": {"status": "rejected"},
            "review_status": "rejected",
        }
    }
    member = tmp_path / "data.json"
    member.write_text(json.dumps(data), encoding="utf-8")
    result = module.run_provenance_check([("AC-4:data.json", member)])
    assert result["status"] == "findings_present"
    assert result["provenance_records_checked"] == 1
    assert "AC-4:data.json $.properties: source_license is 'TBD'" in result["violations"]
    assert any("review_status is 'rejected'" in v for v in result["violations"])

    # The same object in a document that only looks like a schema still blocks.
    data["$schema"] = "urn:invented:not-json-schema"
    member.write_text(json.dumps(data), encoding="utf-8")
    assert module.run_provenance_check([("AC-4:data.json", member)])["provenance_records_checked"] == 1


def test_scan_record_carries_no_host_paths_or_captured_logs(tmp_path, monkeypatch):
    # Review leak surfaces: captured security logs, absolute class roots and
    # raw engine finding paths must not reach the record. No cargo runs.
    module = load_module()
    candidate_root = make_candidate_tree(tmp_path)
    host_text = str(tmp_path / "host-only" / "pytest-rootdir")

    def fake_lint(paths, root):
        return {"status": "findings_present", "command": "cargo run ...",
                "finding_count": 1, "note": None,
                "findings": [{"path": str(paths[0]), "severity": "WARNING",
                              "code": "UNKNOWN_PROVENANCE_REVIEW_REQUIRED", "line": 1,
                              "column": 1, "class": "invented", "excerpt": "invented",
                              "matched_policy": "invented", "message": "invented",
                              "remediation": "invented", "review_route": "invented"}]}

    monkeypatch.setattr(module, "run_engine_lint", fake_lint)
    monkeypatch.setattr(
        module, "run_command",
        lambda command, cwd: subprocess.CompletedProcess(command, 1, host_text, host_text),
    )
    _, output_dir = run_scan(module, tmp_path, candidate_root, lint_mode="engine",
                             extra=["--security-profile", "run"])
    path, record = read_single_record(output_dir)
    text = path.read_text(encoding="utf-8")
    for host in (str(tmp_path), str(tmp_path.resolve()), host_text):
        assert host not in text
    assert record["checks"]["security_profile"]["status"] == "findings_present"
    assert "not recorded" in record["checks"]["security_profile"]["note"]
    assert record["artifact_classes"]["AC-4"]["root"] == "<absolute path>/candidate"
    assert "resolved_root" not in record["artifact_classes"]["AC-4"]
    lint_paths = {f["path"] for f in record["checks"]["protected_content_lint"]["findings"]}
    assert lint_paths <= {"AC-4:README.md", "AC-4:example.json"}


INVALID_RECORD_DOCUMENT = {
    "properties": {
        "source_name": "invented",
        "source_location": "invented/inline",
        "source_license": "TBD",
        "contributor": "invented",
        "contributor_certification": "invented",
        "redistribution_status": {"status": "rejected"},
        "review_status": "rejected",
    },
}


@pytest.mark.parametrize(
    "declared",
    [
        None,
        "https://json-schema.org.invented.invalid/draft/2020-12/schema",
        "https://json-schema.org/draft/2020-12/schema",
    ],
)
def test_data_at_a_schema_path_is_checked_whatever_it_declares(tmp_path, declared):
    # Review r3 P1: a data document at schemas/x.schema.json passed a path
    # pattern with zero records checked; r2 P1: a spoofed or genuine `$schema`
    # in data hid the record. Only the committed manifest, by path and
    # SHA-256, makes a file a schema.
    module = load_module()
    data = dict(INVALID_RECORD_DOCUMENT)
    if declared is not None:
        data["$schema"] = declared
    member = tmp_path / "x.schema.json"
    member.write_text(json.dumps(data), encoding="utf-8")
    for label in ("AC-4:schemas/x.schema.json", "AC-4:fixtures/data.json"):
        result = module.run_provenance_check([(label, member)])
        assert result["provenance_records_checked"] == 1, label
        assert f"{label} $.properties: source_license is 'TBD'" in result["violations"]


def test_listed_schema_is_trusted_only_while_its_hash_matches(tmp_path):
    import hashlib

    module = load_module()
    schema = {
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "properties": {"redistribution_status": {"type": "string"},
                       "review_status": {"type": "string"}},
    }
    member = tmp_path / "invented.schema.json"
    member.write_text(json.dumps(schema), encoding="utf-8")
    manifest = {"schemas/invented.schema.json": hashlib.sha256(member.read_bytes()).hexdigest()}
    label = "AC-4:schemas/invented.schema.json"
    assert module.run_provenance_check([(label, member)], manifest)["provenance_records_checked"] == 0

    member.write_text(json.dumps(INVALID_RECORD_DOCUMENT), encoding="utf-8")
    changed = module.run_provenance_check([(label, member)], manifest)
    assert changed["provenance_records_checked"] == 1
    assert f"{label} $.properties: source_license is 'TBD'" in changed["violations"]


def load_generator():
    spec = importlib.util.spec_from_file_location(
        "generate_schema_manifest", ROOT / "tools" / "release" / "generate_schema_manifest.py"
    )
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def test_schema_manifest_is_generated_from_the_real_schemas(tmp_path):
    # Changing a schema requires regenerating the manifest; every listed schema
    # validates against the JSON Schema 2020-12 metaschema (pinned validator).
    generator = load_generator()
    assert generator.main(["--check"]) == 0
    manifest = json.loads((ROOT / generator.MANIFEST_PATH).read_text(encoding="utf-8"))
    assert manifest["schemas"]
    for entry in manifest["schemas"]:
        generator.check_against_metaschema(json.loads((ROOT / entry["path"]).read_text(encoding="utf-8")))

    root = tmp_path / "project"
    (root / "schemas").mkdir(parents=True)
    (root / "schemas" / "bad.schema.json").write_text(json.dumps({"type": 5}), encoding="utf-8")
    with pytest.raises(Exception):
        generator.build_manifest(root)


def test_provenance_scope_is_pinned_by_file_hash_and_label_pair(tmp_path):
    # RT-V8 P7: product labels inside listed invented fixtures are scoped out
    # of check (c) only by exact path, file SHA-256 and value pair.
    import hashlib

    module = load_module()
    label = dict(ACCEPTED_PROVENANCE, redistribution_status="private_only", review_status="pending")
    tbd = dict(ACCEPTED_PROVENANCE, source_license="TBD", review_status="pending")
    member = tmp_path / "invented_fixture.json"
    member.write_text(json.dumps({"a": {"provenance": label}, "b": {"provenance": tbd}}), encoding="utf-8")
    scope = {"fixtures/invented_fixture.json": {
        "sha256": hashlib.sha256(member.read_bytes()).hexdigest(),
        "labels": {('"private_only"', '"pending"')},
    }}
    entry = [("AC-4:fixtures/invented_fixture.json", member)]
    result = module.run_provenance_check(entry, {}, scope)
    assert result["scoped_record_count"] == 1
    assert result["scoped_members"] == ["AC-4:fixtures/invented_fixture.json"]
    assert result["violations"] and all("$.b.provenance" in v for v in result["violations"])

    moved = module.run_provenance_check([("AC-4:fixtures/other.json", member)], {}, scope)
    assert moved["scoped_record_count"] == 0 and any("$.a.provenance" in v for v in moved["violations"])
    member.write_text(json.dumps({"a": {"provenance": label}}) + " ", encoding="utf-8")
    changed = module.run_provenance_check(entry, {}, scope)
    assert changed["scoped_record_count"] == 0 and changed["violations"]


def load_scope_generator():
    spec = importlib.util.spec_from_file_location(
        "generate_provenance_scope", ROOT / "tools" / "release" / "generate_provenance_scope.py"
    )
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def test_committed_provenance_scope_is_the_generators_output():
    # RT-V8 P7 review: the committed scope is derived, not hand-kept; any
    # hash drift, new label or ineligible file fails here.
    assert load_scope_generator().main(["--check"]) == 0


def make_export_shaped_repo(tmp_path: Path) -> Path:
    """A committed invented tree with an explicit reviewed scan-scope source set."""
    root = tmp_path / "project"
    (root / "fixtures").mkdir(parents=True)
    (root / "tools/release").mkdir(parents=True)
    (root / "tools/release/provenance_scope_members.json").write_text(json.dumps({
        'members': ['fixtures/eligible.json', 'fixtures/invented_named_but_undeclared.json']}))
    declared = dict(ACCEPTED_PROVENANCE, redistribution_status="invented_non_engineering_example",
                    review_status="pending")
    undeclared = dict(ACCEPTED_PROVENANCE, source_name="Example project data",
                      contributor_certification="Example values only",
                      redistribution_status="private_only", review_status="pending")
    (root / "fixtures" / "eligible.json").write_text(
        json.dumps({"provenance": declared}), encoding="utf-8")
    (root / "fixtures" / "invented_named_but_undeclared.json").write_text(
        json.dumps({"provenance": undeclared}), encoding="utf-8")
    env = {**__import__("os").environ, "GIT_CONFIG_GLOBAL": "/dev/null", "GIT_CONFIG_NOSYSTEM": "1",
           "GIT_AUTHOR_NAME": "invented", "GIT_AUTHOR_EMAIL": "invented@example.invalid",
           "GIT_COMMITTER_NAME": "invented", "GIT_COMMITTER_EMAIL": "invented@example.invalid"}
    for command in (["init", "-q"], ["add", "-A"], ["commit", "-q", "-m", "invented"]):
        subprocess.run(["git", *command], cwd=root, env=env, check=True, capture_output=True)
    return root


def test_provenance_scope_check_refuses_ineligible_stale_and_underived_data(tmp_path):
    generator = load_scope_generator()
    root = make_export_shaped_repo(tmp_path)
    assert generator.main(["--root", str(root)]) == 0
    listing = root / generator.SCOPE_PATH
    generated = json.loads(listing.read_text(encoding="utf-8"))
    # Eligibility is by declaration: the undeclared file is never listed.
    assert [m["path"] for m in generated["members"]] == ["fixtures/eligible.json"]
    assert generator.main(["--root", str(root), "--check"]) == 0

    def check_with(document) -> int:
        listing.write_text(json.dumps(document, indent=2) + "\n", encoding="utf-8")
        return generator.main(["--root", str(root), "--check"])

    ineligible = json.loads(json.dumps(generated))
    ineligible["members"].append(dict(generated["members"][0], path="fixtures/invented_named_but_undeclared.json"))
    assert check_with(ineligible) == 1
    stale = json.loads(json.dumps(generated))
    stale["members"][0]["sha256"] = "0" * 64
    assert check_with(stale) == 1
    underived = json.loads(json.dumps(generated))
    underived["members"][0]["labels"].append(['"public_permissive"', '"TBD"'])
    assert check_with(underived) == 1
    # Byte-exact: the same content with CRLF line endings is drift.
    listing.write_bytes((json.dumps(generated, indent=2) + "\n").replace("\n", "\r\n").encode("utf-8"))
    assert generator.main(["--root", str(root), "--check"]) == 1


@pytest.mark.parametrize("field, text", [
    ("origin", "not invented; supplied by a customer"),
    ("source_name", "invented-by: a real person"),
])
def test_free_text_never_qualifies_a_file_for_the_p7_scope(field, text):
    # P7 re-review: whole-word "invented" in free text qualified a file, and
    # check (c) then suppressed its private_only/pending records. Only the
    # product label redistribution_status == invented_non_engineering_example
    # qualifies a file.
    generator = load_scope_generator()
    record = dict(ACCEPTED_PROVENANCE, redistribution_status="private_only",
                  review_status="pending", source_name="Example project data",
                  contributor_certification="Example values only")
    record[field] = text
    assert generator.invented_declaration({"provenance": record}) is None
    labelled = dict(record, redistribution_status="invented_non_engineering_example")
    assert generator.invented_declaration({"provenance": labelled}) is not None


def test_quarantine_hygiene_flags_quarantine_paths(tmp_path):
    module = load_module()
    candidate_root = make_candidate_tree(tmp_path)
    quarantined = candidate_root / "quarantine" / "protected-content"
    quarantined.mkdir(parents=True)
    (quarantined / "held.txt").write_text(
        "invented quarantined placeholder\n", encoding="utf-8"
    )

    _, output_dir = run_scan(module, tmp_path, candidate_root)
    _, record = read_single_record(output_dir)

    hygiene = record["checks"]["quarantine_hygiene"]
    assert hygiene["status"] == "findings_present"
    assert any(
        "quarantine/protected-content" in offender
        for offender in hygiene["artifacts_under_quarantine_path"]
    )
    assert record["scan_disposition"] == "findings_require_owner_review"


def test_lint_findings_are_normalized_with_pending_disposition(tmp_path, monkeypatch):
    module = load_module()
    candidate_root = make_candidate_tree(tmp_path)

    def fake_lint(paths, root):
        assert paths, "lintable members should be passed to the engine"
        return {
            "status": "findings_present",
            "command": "cargo run --quiet ...",
            "engine_summary": {"finding_count": 1},
            "finding_count": 1,
            "findings": [
                {
                    "finding_id": "RPLC-F0001",
                    "code": "UNKNOWN_PROVENANCE_REVIEW_REQUIRED",
                    "class": "PROVENANCE_WARNING",
                    "severity": "WARNING",
                    "path": str(paths[0]),
                    "line": 2,
                    "column": 1,
                    "matched_policy": "OPS-K-IP-3",
                    "excerpt": "allowable stress",
                    "message": "Standards-table signature (stubbed).",
                    "remediation": "Route to human IP review.",
                    "review_route": "HUMAN_IP_REVIEW",
                    "disposition": "PENDING",
                }
            ],
            "note": None,
        }

    monkeypatch.setattr(module, "run_engine_lint", fake_lint)
    _, output_dir = run_scan(module, tmp_path, candidate_root, lint_mode="engine")
    _, record = read_single_record(output_dir)

    lint_findings = [
        finding
        for finding in record["findings"]
        if finding["check"] == "protected_content_lint"
    ]
    assert len(lint_findings) == 1
    finding = lint_findings[0]
    assert finding["code"] == "UNKNOWN_PROVENANCE_REVIEW_REQUIRED"
    assert finding["review_route"] == "HUMAN_IP_REVIEW"
    assert finding["matched_policy"] == "OPS-K-IP-3"
    assert finding["disposition"] == "pending_owner_review"
    assert record["scan_disposition"] == "findings_require_owner_review"


def test_record_makes_no_release_readiness_claim(tmp_path):
    module = load_module()
    candidate_root = make_candidate_tree(tmp_path)
    _, output_dir = run_scan(module, tmp_path, candidate_root)
    record_path, record = read_single_record(output_dir)

    serialized = record_path.read_text(encoding="utf-8").lower()
    assert "release ready" not in serialized
    assert "release-ready" not in serialized
    assert "certified" not in serialized
    assert record["scan_disposition"] != "pass"
    assert "clearance" not in record["scan_disposition"]


@pytest.mark.skipif(
    shutil.which("cargo") is None, reason="cargo unavailable; engine CLI not built"
)
@pytest.mark.runs_cargo("cargo run of the protected-content lint engine")
def test_engine_lint_integration_flags_invented_lookalike(tmp_path):
    """End-to-end check (b): the DEL-08-05 engine CLI flags an INVENTED
    standards-table lookalike (designator name plus synthetic values only)
    and the record fails toward owner review."""
    module = load_module()
    candidate_root = make_candidate_tree(tmp_path)
    (candidate_root / "lookalike.txt").write_text(
        "Invented lookalike fixture: no standards values copied.\n"
        "ASME code table heading (invented placeholder)\n"
        "101.1 202.2 303.3 404.4\n"
        "111.1 222.2 333.3 444.4\n"
        "121.1 242.2 363.3 484.4\n",
        encoding="utf-8",
    )

    exit_code, output_dir = run_scan(
        module, tmp_path, candidate_root, lint_mode="engine"
    )
    assert exit_code == 0
    _, record = read_single_record(output_dir)

    lint_check = record["checks"]["protected_content_lint"]
    assert lint_check["status"] == "findings_present"
    signature_findings = [
        finding
        for finding in lint_check["findings"]
        if finding["matched_policy"] == "OPS-K-IP-3"
    ]
    assert len(signature_findings) == 1
    assert signature_findings[0]["code"] == "UNKNOWN_PROVENANCE_REVIEW_REQUIRED"
    assert signature_findings[0]["review_route"] == "HUMAN_IP_REVIEW"
    assert signature_findings[0]["path"].endswith("lookalike.txt")
    assert record["scan_disposition"] == "findings_require_owner_review"


@pytest.mark.skipif(
    shutil.which("cargo") is None, reason="cargo unavailable; engine CLI not built"
)
@pytest.mark.runs_cargo("cargo run of the protected-content lint engine")
def test_engine_cli_reports_clean_content_without_findings(tmp_path):
    """Negative path for check (b): legitimate invented content with no
    standards-table signature produces no engine findings."""
    clean = tmp_path / "clean.txt"
    clean.write_text(
        "Rule pack id: invented-demo; version: 0.1.0; human review required.\n",
        encoding="utf-8",
    )
    completed = subprocess.run(
        [
            "cargo",
            "run",
            "--quiet",
            "--manifest-path",
            "core/reporting/protected_content_linter/Cargo.toml",
            "--bin",
            "protected_content_lint_cli",
            "--",
            "--provenance-mode",
            "external",
            str(clean),
        ],
        cwd=ROOT,
        capture_output=True,
        text=True,
        check=False,
    )
    assert completed.returncode == 0
    payload = json.loads(completed.stdout)
    assert payload["summary"]["finding_count"] == 0
    assert payload["summary"]["clean_scan_is_clearance"] is False


def test_lint_batches_cover_every_input_and_preserve_summary_types(monkeypatch):
    module = load_module()
    paths = [Path(f'invented-{i}.txt') for i in range(5)]
    monkeypatch.setattr(module, 'lint_path_batches', lambda items, command: [items[:2], items[2:]])
    seen = []
    def engine(command, root):
        selected = command[command.index('external') + 1:]
        seen.extend(selected)
        payload = {'provenance_mode': 'external', 'summary': {'target_count': len(selected), 'scanned_target_count': len(selected), 'skipped_private_target_count': 0, 'skipped_incomplete_target_count': 0, 'finding_count': len(selected), 'blocking_finding_count': len(selected), 'clean_scan_is_clearance': False}, 'findings': [{'path': path, 'severity': 'BLOCKING'} for path in selected]}
        return subprocess.CompletedProcess(command, 0, json.dumps(payload), '')
    monkeypatch.setattr(module, 'run_command', engine)
    result = module.run_engine_lint(paths, ROOT)
    assert seen == [str(path) for path in paths]
    assert result['status'] == 'findings_present'
    assert result['finding_count'] == result['engine_summary']['target_count'] == 5
    assert result['engine_summary']['clean_scan_is_clearance'] is False


def test_unavailable_lint_batch_cannot_report_pass(monkeypatch):
    module = load_module()
    paths = [Path('invented-first.txt'), Path('invented-second.txt')]
    monkeypatch.setattr(module, 'lint_path_batches', lambda items, command: [[items[0]], [items[1]]])
    def engine(command, root):
        if command[-1] == str(paths[1]):
            raise OSError('invented unavailable engine')
        return subprocess.CompletedProcess(command, 0, json.dumps({'provenance_mode': 'external', 'summary': {'target_count': 1, 'scanned_target_count': 1, 'skipped_private_target_count': 0, 'skipped_incomplete_target_count': 0, 'finding_count': 0, 'blocking_finding_count': 0, 'clean_scan_is_clearance': False}, 'findings': []}), '')
    monkeypatch.setattr(module, 'run_command', engine)
    result = module.run_engine_lint(paths, ROOT)
    assert result['status'] == 'not_run'
    assert result['completed_batches'] == 1


@pytest.mark.parametrize('payload', [{}, {'summary': {'target_count': 0}, 'findings': []}])
def test_malformed_engine_response_is_not_a_clean_pass(monkeypatch, payload):
    module = load_module()
    monkeypatch.setattr(module, 'run_command', lambda command, root: subprocess.CompletedProcess(command, 0, json.dumps(payload), ''))
    assert module.run_engine_lint([ROOT / 'invented.txt'], ROOT)['status'] == 'not_run'


def test_omitted_partition_input_cannot_pass(monkeypatch):
    module = load_module()
    monkeypatch.setattr(module, 'lint_path_batches', lambda paths, command: [paths[:1]])
    monkeypatch.setattr(module, 'run_command', lambda *args: (_ for _ in ()).throw(AssertionError('engine must not run')))
    assert module.run_engine_lint([ROOT / 'a.txt', ROOT / 'b.txt'], ROOT)['status'] == 'not_run'
