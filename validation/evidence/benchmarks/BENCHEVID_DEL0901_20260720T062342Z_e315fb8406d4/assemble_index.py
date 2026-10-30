#!/usr/bin/env python3
"""Deterministic bundle-local assembly for BENCHEVID_DEL0901_20260720T062342Z_e315fb8406d4.

Rebuilds FAMILY_PROVENANCE_INDEX.csv and MANIFEST.json for this derivative
evidence bundle from its recorded inputs only:

  - the captured whole-suite runner report SUITE_RUN_MECHANICS.json (in this
    bundle),
  - the committed suite crate source validation/benchmarks/mechanics/src/lib.rs,
  - the two committed README inventory mirrors
    (validation/benchmarks/mechanics/README.md,
    validation/hand_calcs/mechanics/README.md),
  - the committed validation/hand_calcs/mechanics/ inventory (witness anchors).

No field is invented: every CSV value is extracted from one of those sources.
The script is stdlib-only, offline, and writes only inside this bundle
directory. The generated-at timestamp and execution-basis identifiers are
supplied as recorded arguments (not wall-clock), so rerunning with the same
recorded arguments yields byte-identical FAMILY_PROVENANCE_INDEX.csv output.

Claim posture (DEC-081): everything produced here is derivative
regression/verification evidence for the committed suite state at the recorded
base commit only. Nothing here is a validation outcome, an acceptance,
a release-readiness statement, or a threshold/tolerance selection; those
remain TBD and owner-gated.

Recorded rerun command (from WORKING_ROOT = projects/chirality-piping):

  python3 validation/evidence/benchmarks/BENCHEVID_DEL0901_20260720T062342Z_e315fb8406d4/assemble_index.py \
    --generated-at 2026-07-20T06:23:42Z \
    --base-commit e315fb8406d44dce684cbec091f3174c261efee4 \
    --branch claude/piping-r14-pkg09-evidence \
    --capture-exit-code 1
"""

from __future__ import annotations

import argparse
import csv
import hashlib
import json
import re
import sys
from pathlib import Path

BUNDLE_DIR = Path(__file__).resolve().parent
BUNDLE_ID = BUNDLE_DIR.name
# bundle dir -> benchmarks -> evidence -> validation -> WORKING_ROOT
WORKING_ROOT = BUNDLE_DIR.parents[3]

LIB_RS = WORKING_ROOT / "validation/benchmarks/mechanics/src/lib.rs"
BENCH_README = WORKING_ROOT / "validation/benchmarks/mechanics/README.md"
HAND_CALC_README = WORKING_ROOT / "validation/hand_calcs/mechanics/README.md"
CAPTURE = BUNDLE_DIR / "SUITE_RUN_MECHANICS.json"

CSV_FIELDS = [
    "fixture_id",
    "family",
    "constructor_fn",
    "provenance_source_name",
    "provenance_source_location",
    "provenance_source_license",
    "provenance_contributor",
    "contributor_certification",
    "redistribution_status",
    "review_disposition",
    "unit_basis",
    "expected_value_count",
    "tolerance_policy_state",
    "witness_anchor_paths",
    "witness_anchor_exists",
    "head_suite_run_status",
    "head_blocking_code",
]


def fail(message: str) -> None:
    print(f"ASSEMBLE_INDEX_FAIL: {message}", file=sys.stderr)
    sys.exit(1)


def parse_inventory_order(source: str) -> list[str]:
    match = re.search(
        r"pub fn fixture_inventory\(\) -> Vec<MechanicsBenchmark> \{\s*vec!\[(.*?)\]\s*\}",
        source,
        re.S,
    )
    if not match:
        fail("fixture_inventory() vec![] block not found in lib.rs")
    names = re.findall(r"(\w+)\(\)", match.group(1))
    if not names:
        fail("no constructor calls parsed from fixture_inventory()")
    return names


def parse_readiness_assertion(source: str) -> int:
    match = re.search(r"assert_eq!\(fixtures\.len\(\), (\d+)\);", source)
    if not match:
        fail("readiness inventory-count assertion not found in lib.rs")
    return int(match.group(1))


def constructor_slice(source: str, fn_name: str) -> str:
    marker = f"pub fn {fn_name}() -> MechanicsBenchmark {{"
    start = source.find(marker)
    if start < 0:
        fail(f"constructor {fn_name} not found in lib.rs")
    end = source.find("\npub fn ", start + len(marker))
    return source[start:end] if end > 0 else source[start:]


def extract_one(pattern: str, text: str, what: str, fn_name: str) -> str:
    matches = re.findall(pattern, text, re.S)
    if len(matches) != 1:
        fail(f"expected exactly one {what} in constructor {fn_name}; found {len(matches)}")
    return matches[0]


def parse_shared_provenance(source: str) -> dict[str, str]:
    match = re.search(
        r"pub fn public_original\(source_location: &'static str\) -> Self \{(.*?)\n    \}",
        source,
        re.S,
    )
    if not match:
        fail("BenchmarkProvenance::public_original impl not found in lib.rs")
    body = match.group(1)

    def field(name: str) -> str:
        m = re.search(name + r":\s*\n?\s*\"([^\"]+)\"", body)
        if not m:
            fail(f"shared provenance field {name} not found in public_original impl")
        return m.group(1)

    redistribution = re.search(r"redistribution_status:\s*RedistributionStatus::(\w+)", body)
    review = re.search(r"review_disposition:\s*ReviewDisposition::(\w+)", body)
    if not redistribution or not review:
        fail("redistribution_status/review_disposition variants not found in public_original impl")
    return {
        "source_name": field("source_name"),
        "source_license": field("source_license"),
        "contributor": field("contributor"),
        "contributor_certification": field("contributor_certification"),
        "redistribution_status": redistribution.group(1),
        "review_disposition": review.group(1),
    }


def parse_unit_basis_ref(source: str) -> str:
    match = re.search(
        r"const PKG09_FIXTURE_UNIT_SYSTEM_REF: &str = \"([^\"]+)\";", source
    )
    if not match:
        fail("PKG09_FIXTURE_UNIT_SYSTEM_REF constant not found in lib.rs")
    return match.group(1)


def sha256_of(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--generated-at", required=True,
                        help="recorded UTC generation timestamp (not wall-clock)")
    parser.add_argument("--base-commit", required=True,
                        help="recorded execution base commit SHA")
    parser.add_argument("--branch", required=True,
                        help="recorded execution branch")
    parser.add_argument("--capture-exit-code", required=True, type=int,
                        help="recorded exit code of the whole-suite capture run")
    args = parser.parse_args()

    source = LIB_RS.read_text(encoding="utf-8")
    bench_readme = BENCH_README.read_text(encoding="utf-8")
    hand_calc_readme = HAND_CALC_README.read_text(encoding="utf-8")
    capture = json.loads(CAPTURE.read_text(encoding="utf-8"))

    suite_run = capture.get("suite_run")
    if not suite_run:
        fail("captured report has no suite_run block")
    if suite_run.get("suite") != "mechanics" or suite_run.get("suite_deliverable") != "DEL-09-01":
        fail("captured suite_run is not the mechanics/DEL-09-01 suite")
    if suite_run.get("whole_suite_default_applied") is not True:
        fail("captured run did not apply the whole-suite default")

    case_by_id = {case["fixture_id"]: case for case in suite_run.get("cases", [])}
    blocking_code_by_id: dict[str, str] = {}
    for diag in capture.get("diagnostics", []):
        ref = diag.get("affected_object") or {}
        ref_id = ref.get("ref_id", "")
        if diag.get("severity") == "blocking" and ref_id:
            blocking_code_by_id[ref_id] = diag.get("code", "")

    order = parse_inventory_order(source)
    asserted_count = parse_readiness_assertion(source)
    if len(order) != asserted_count:
        fail(
            f"fixture_inventory() has {len(order)} constructors but the readiness "
            f"test asserts {asserted_count}"
        )
    if suite_run.get("requested_case_count") != len(order):
        fail("captured requested_case_count differs from live fixture_inventory() length")
    if len(case_by_id) != len(order):
        fail("captured per-case report count differs from live fixture_inventory() length")

    shared = parse_shared_provenance(source)
    unit_basis_ref = parse_unit_basis_ref(source)

    rows = []
    for fn_name in order:
        block = constructor_slice(source, fn_name)
        fixture_id = extract_one(r"fixture_id:\s*\"([^\"]+)\"", block, "fixture_id", fn_name)
        family = extract_one(
            r"family:\s*BenchmarkFamily::(\w+)", block, "family", fn_name
        )
        anchor = extract_one(
            r"BenchmarkProvenance::public_original\(\s*\"([^\"]+)\"", block,
            "public_original source_location", fn_name,
        )
        if "unit_basis: FIXTURE_UNIT_BASIS" not in block:
            fail(f"constructor {fn_name} does not use FIXTURE_UNIT_BASIS")
        if "tolerance_policy: Some(" in block:
            fail(f"constructor {fn_name} records a tolerance policy; expected none (TBD)")

        anchor_path = WORKING_ROOT / anchor
        anchor_exists = anchor_path.is_file()
        if not anchor_exists:
            fail(f"witness anchor missing for {fixture_id}: {anchor}")
        if fixture_id not in bench_readme:
            fail(f"{fixture_id} missing from validation/benchmarks/mechanics/README.md mirror")
        if fixture_id not in hand_calc_readme:
            fail(f"{fixture_id} missing from validation/hand_calcs/mechanics/README.md mirror")

        case = case_by_id.get(fixture_id)
        if case is None:
            fail(f"{fixture_id} absent from the captured whole-suite run")
        status = case["status"]
        # The captured per-case `values` list carries exactly one ValueComparison
        # per recorded expected value (keyed by the suite crate's recorded value
        # names) for evaluated and comparison-basis-blocked cases; only an
        # execution-failed case reports an empty list, which cannot ground a
        # count and is a recorded finding that stops assembly.
        expected_count = len(case.get("values", []))
        if expected_count == 0:
            fail(
                f"{fixture_id}: captured case carries no value comparisons "
                f"(status {status}); expected-value count cannot be extracted"
            )
        blocking_code = blocking_code_by_id.get(fixture_id, "") if status != "executed_and_matched" else ""
        if status == "blocked" and not blocking_code:
            fail(f"{fixture_id} is blocked but has no blocking diagnostic code")

        rows.append({
            "fixture_id": fixture_id,
            "family": family,
            "constructor_fn": fn_name,
            "provenance_source_name": shared["source_name"],
            "provenance_source_location": anchor,
            "provenance_source_license": shared["source_license"],
            "provenance_contributor": shared["contributor"],
            "contributor_certification": shared["contributor_certification"],
            "redistribution_status": shared["redistribution_status"],
            "review_disposition": shared["review_disposition"],
            "unit_basis": unit_basis_ref,
            "expected_value_count": str(expected_count),
            "tolerance_policy_state": "TBD_unresolved_no_tolerance_policy_recorded",
            "witness_anchor_paths": anchor,
            "witness_anchor_exists": "true",
            "head_suite_run_status": status,
            "head_blocking_code": blocking_code,
        })

    index_path = BUNDLE_DIR / "FAMILY_PROVENANCE_INDEX.csv"
    with index_path.open("w", encoding="utf-8", newline="") as handle:
        writer = csv.DictWriter(handle, fieldnames=CSV_FIELDS, lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)

    manifest = {
        "artifact": "openpipestress.benchmark_evidence_bundle",
        "bundle_id": BUNDLE_ID,
        "generated_at_utc": args.generated_at,
        "base_commit": args.base_commit,
        "branch": args.branch,
        "label": "DERIVATIVE EVIDENCE — NON-AUTHORITATIVE",
        "claim_posture": (
            "DEC-081 regression/verification evidence only: per-case match/fail and "
            "test outcomes are regression evidence for current solver behavior "
            "against the suite crate's recorded comparison values at the recorded "
            "base commit. No validation outcome, acceptance, release-readiness, "
            "threshold satisfaction, or professional-reliance claim is made; "
            "release thresholds, final tolerance policy, CI gate policy, and "
            "verification-to-validation promotion remain TBD and owner-gated."
        ),
        "upstream_citations": {
            "base_commit": args.base_commit,
            "branch": args.branch,
            "dag_pointer": "execution/_DAG/_LATEST.md -> DAG-007 (approved_active_graph_authority)",
            "deliverable_status": (
                "execution/PKG-09_Verification, Validation, and Quality Oracles/1_Working/"
                "DEL-09-01_Mechanics benchmark suite/_STATUS.md (Current State: IN_PROGRESS; "
                "four Remaining rows preserved byte-identical by this tranche)"
            ),
            "concordance_rows": (
                "execution/_Reconciliation/DeliverableConcordance/"
                "DELIVERABLE_CONCORDANCE_2026-07-11_1305/PROPOSED_DELIVERABLE_UPDATES.csv "
                "PDU-037 (AuthorityNeeded=NO evidence refresh) and PDU-013 "
                "(AuthorityNeeded=ENGINEERING unit-catalog hold, untouched)"
            ),
            "governing_brief": (
                "execution/_Coordination/CANDIDATE_BRIEF_2026-07-20_T6_DEL-09-01_"
                "BENCH_EVIDENCE_SYSTEM.md (CB-2026-07-20-T6-DEL-09-01-BENCH-EVIDENCE-001)"
            ),
            "suite_crate": "validation/benchmarks/mechanics/src/lib.rs (fixture_inventory(), read-only)",
            "runner": (
                "core/runner/headless/src/bin/openpipestress-runner.rs + "
                "core/runner/headless/src/benchmark_binding.rs (run-benchmark, DEC-065 exit policy, read-only)"
            ),
            "inventory_mirrors": [
                "validation/benchmarks/mechanics/README.md",
                "validation/hand_calcs/mechanics/README.md",
            ],
            "input_shape_model": (
                "validation/witness/inputs/del1005_payload_binding_benchmark_single_case_input.json "
                "(committed witness input shape mirrored; witness untouched)"
            ),
        },
        "capture": {
            "command": (
                "CARGO_NET_OFFLINE=true cargo run --offline --manifest-path "
                "core/runner/headless/Cargo.toml --bin openpipestress-runner -- "
                "run-benchmark --input <bundle>/suite_run_mechanics_input.json "
                "--output <bundle>/SUITE_RUN_MECHANICS.json"
            ),
            "recorded_exit_code": args.capture_exit_code,
            "exit_code_semantics": (
                "DEC-065: exit 1 records the presence of blocking diagnostics; a "
                "whole-suite run containing blocked cases exits 1 as fail-closed "
                "regression evidence, recorded exactly as emitted — not a defect "
                "record and not a failure of this tranche"
            ),
            "suite": suite_run["suite"],
            "suite_deliverable": suite_run["suite_deliverable"],
            "whole_suite_default_applied": suite_run["whole_suite_default_applied"],
            "requested_case_count": suite_run["requested_case_count"],
            "executed_and_matched": suite_run["executed_and_matched"],
            "executed_and_mismatched": suite_run["executed_and_mismatched"],
            "blocked": suite_run["blocked"],
        },
        "assembly": {
            "tool": "assemble_index.py (bundle-local, stdlib-only, offline, deterministic)",
            "rerun_command": (
                "python3 validation/evidence/benchmarks/" + BUNDLE_ID + "/assemble_index.py "
                "--generated-at " + args.generated_at + " "
                "--base-commit " + args.base_commit + " "
                "--branch " + args.branch + " "
                "--capture-exit-code " + str(args.capture_exit_code)
            ),
            "determinism": (
                "rerunning with the same recorded arguments rebuilds "
                "FAMILY_PROVENANCE_INDEX.csv byte-identically; the generated-at "
                "timestamp is a recorded argument, not wall-clock"
            ),
        },
        "standing_statements": {
            "pdu_037_row": (
                "PDU-037's provenance-index standing Remaining row REMAINS OPEN. This "
                "bundle BUILDS the family/provenance/redistribution index as derivative "
                "evidence only; row closure is judged at the owner's gate and is not "
                "performed or implied here."
            ),
            "pdu_013_hold": (
                "PDU-013's project-grain unit-catalog hold is untouched: the index "
                "records the fixture-local unit basis "
                f"{unit_basis_ref} exactly as encoded; no project-grain unit-system "
                "acceptance act is performed."
            ),
            "owner_gates": (
                "Acceptance thresholds, runner/release integration, tolerance "
                "promotion (DEC-024/DEC-026/DEC-046), verification-to-validation "
                "promotion, acceptance/release/label/CI-gate/publication effects "
                "(DEC-057/DEC-058/DEC-059, PB-TBD-003), the PKG09-0901-PKG02-001 "
                "disposition (PDU-060), and lifecycle/stage acts remain owner-class "
                "and are NOT performed by this bundle."
            ),
        },
        "file_sha256": {},
    }

    hashed_files = sorted(
        path.name
        for path in BUNDLE_DIR.iterdir()
        if path.is_file() and path.name != "MANIFEST.json"
    )
    for name in hashed_files:
        manifest["file_sha256"][name] = sha256_of(BUNDLE_DIR / name)

    manifest_path = BUNDLE_DIR / "MANIFEST.json"
    manifest_path.write_text(
        json.dumps(manifest, indent=2, sort_keys=False, ensure_ascii=False) + "\n",
        encoding="utf-8",
    )

    print(
        json.dumps(
            {
                "status": "ASSEMBLED",
                "bundle_id": BUNDLE_ID,
                "rows": len(rows),
                "index": str(index_path.relative_to(WORKING_ROOT)),
                "manifest": str(manifest_path.relative_to(WORKING_ROOT)),
                "hashed_files": hashed_files,
            },
            indent=2,
        )
    )


if __name__ == "__main__":
    main()
