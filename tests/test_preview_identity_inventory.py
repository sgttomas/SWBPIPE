"""Inventory guard of the preview-physics-2 identity (DEL-05-03 Design pf-timoshenko.md §4, "Inventory guard").

The pressure-free Timoshenko identity (`preview-physics-timoshenko-1`) is admitted beside preview-physics-2 at every
identity-aware site. Sites that name one identity were found across review rounds, so this test scans the tracked Rust,
Python, TypeScript and schema sources for the preview-physics-2 identity string, its table filename, its profile, its
table's SHA-256 literal and its limitation 0 text, and fails on any (file, needle) occurrence that is not on the
reviewed list below, or on a listed one that no longer occurs. Each entry is marked "admits both identities" or
"Euler-Bernoulli only, by design" with its reason.

Test sources are outside the scan (they name the identity as fixtures), except `tests/verifiers/`, the readers'
Python mirrors. The reviewed list also enumerates the symbol-only gates, which carry no literal: each must still
name its symbols. A label proves nothing by itself: every "admits both identities" entry must name the Timoshenko
identity (its literal or symbol) in the same file, and names the round-trip test that fails if the gate refuses it.
"""
from __future__ import annotations

import subprocess
from pathlib import Path

PROJECT = Path(__file__).resolve().parents[1]

NEEDLES = {
    "id": "openpipestress.result_semantics/0.3.0/preview-physics-2",
    "table": "semantic_contract_v0_3_preview_physics_2",
    "profile": "product_preview_mechanics_v1",
    "hash": "1ccc9561cb4ce59d047f3dab7927a927c44a707efb3acb90e7b07a21b3bad0a2",
    "lim0": "Small-displacement, linear-elastic Euler-Bernoulli frame preview",
}
BOTH = "admits both identities"
EB_ONLY = "Euler-Bernoulli only, by design"

# The round trips that exercise the "admits both" entries.
RUST_READERS = "core/reporting/result_export/tests/preview_timoshenko_identity.rs"
PY_READERS = "tests/test_preview_timoshenko_identity.py"
PACKAGE = "core/reporting/report_package/tests/pf_timoshenko_package.rs"
NATIVE = "apps/desktop/src-tauri/src/report_render_pf_timoshenko_tests.rs"
DESKTOP = "apps/desktop/src/features/results/pfTimoshenkoIdentity.test.tsx"
TABLES = "tests/test_semantic_contract_tables.py"
PRODUCT_REFERENCES = "core/product_physics/tests/pf_timoshenko_references.rs"
PRODUCT_RUNTIME = "core/product_physics/tests/pf_timoshenko_runtime.rs"

# Literal sites: path -> (needles, class, reason, Timoshenko marker or None, exercised by).
LITERAL_SITES = {
    "core/reporting/result_export/src/semantic_contract.rs": (
        {"id", "table", "profile", "hash"}, BOTH,
        "the identity constants and the pinned table; PREVIEW_PHYSICS_IDENTITIES keys every reader by semantic_contract_id",
        "PREVIEW_PHYSICS_TIMOSHENKO_ID", RUST_READERS),
    "core/reporting/result_export/src/retained_precision.rs": (
        {"id", "table", "profile"}, EB_ONLY,
        "the retained successor's base identity: route L declines a Timoshenko document at D1 (design §3); the transport "
        "validator itself selects by identity (preview_physics_identity_of)", None, None),
    "core/analysis_runs/compatibility.py": (
        {"id", "table", "hash"}, BOTH, "the record builder's identities and tables", "PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_ID", PY_READERS),
    "tests/verifiers/compatibility.py": (
        {"id", "table", "profile", "hash"}, BOTH, "the verifier's identities, profiles and tables", "PREVIEW_PHYSICS_CONTRACTS", PY_READERS),
    "tests/verifiers/preview_physics_evidence.py": (
        {"id", "table", "profile"}, BOTH, "the Python raw and transport reader: either complete identity", "TIMOSHENKO_CONTRACT_ID", PY_READERS),
    "tests/verifiers/retained_precision.py": (
        {"id", "table", "profile"}, EB_ONLY, "the retained successor's base (route L; design §3)", None, None),
    "schemas/results.v0.3.schema.yaml": (
        {"id", "profile", "lim0"}, BOTH, "a Timoshenko branch beside preview-physics-2's, with its own const limitations",
        "preview-physics-timoshenko-1", TABLES),
    "schemas/analysis_run.v0.3.schema.json": (
        {"id", "hash"}, BOTH, "the Timoshenko id and hash beside preview-physics-2's", "preview-physics-timoshenko-1", PY_READERS),
    "schemas/stress_neutral_export.v0.3.schema.json": (
        {"id", "profile", "hash"}, BOTH, "the Timoshenko id, profile and hash beside preview-physics-2's", "preview-physics-timoshenko-1", PY_READERS),
    "tools/serialization/semantic_contract_tables.py": (
        {"id", "table", "profile", "lim0"}, BOTH, "derives the Timoshenko table from preview-physics-2 (derive --check)", "TIMOSHENKO_ID", TABLES),
    "apps/desktop/src/features/results/numericalResultQuality.ts": (
        {"id"}, BOTH, "the desktop's identity constants; PREVIEW_PHYSICS_CONTRACT_IDS names both", "PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_ID", DESKTOP),
    "apps/desktop/src/features/stress-neutral/StressNeutralExportPanel.tsx": (
        {"table"}, BOTH, "the export's table file, by the source's identity on the preview route", "PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_ID", DESKTOP),
    "apps/desktop/src/services/analysisRunCompatibility.ts": (
        {"table"}, BOTH, "the record builder's tables (RECORD_TABLES)", "previewPhysicsTimoshenkoTable", DESKTOP),
    "apps/desktop/src-tauri/src/lib.rs": (
        {"id", "profile"}, EB_ONLY, "test modules only: Euler-Bernoulli policy and solve fixtures", None, None),
    "core/product_physics/src/lib.rs": (
        {"id", "profile"}, EB_ONLY,
        "preview_formulation_basis(), the route's pre-publication basis, which every PF envelope (solved and blocked) "
        "replaces with pf_timoshenko::formulation_basis; the identity literal is in a test of an Euler-Bernoulli solve. "
        "The producer sites themselves are a symbol gate below", None, None),
    "core/product_physics/src/preview_physics.rs": (
        {"id", "table", "profile", "lim0"}, EB_ONLY,
        "the Euler-Bernoulli identity's own constants and its table test; pf_timoshenko.rs derives the Timoshenko "
        "identity from them (its own id, profile and limitation 0) and selects between the two", None, None),
    "core/product_physics/src/build_identity.rs": (
        {"table"}, EB_ONLY,
        "the retained profile's reviewed inputs, the reader statics route L parses. Route L declines a Timoshenko "
        "document at D1, and the readers parse the Timoshenko table only for a source of that identity, but its "
        "OnceLock cache (semantic_contract::preview_physics_timoshenko_contract) then stays resident for the "
        "process, through later route-L Euler-Bernoulli invocations too. So it is a process-lifetime static owner, "
        "not yet registered: at #1663's D3 turn the table joins REVIEWED_INPUTS and its cache is priced (design "
        "§6). Adding it before then would make the registered retained profile Stale", None, None),
    "core/product_physics/src/retained_memory.rs": (
        {"table", "hash"}, EB_ONLY, "the registered retained-memory profile's reviewed inputs (route L; S5's D3 registration owns them)", None, None),
    "tools/validation/qualification_gate.py": (
        {"id", "table", "hash"}, EB_ONLY, "the qualification gate's closed dispatch over committed Euler-Bernoulli witness carriers", None, None),
    "tools/serialization/generate_product_preview_mechanics.mjs": (
        {"id"}, EB_ONLY, "regenerates the committed Euler-Bernoulli demo fixtures; any other identity is refused", None, None),
}

# Symbol-only gates (no literal): path -> (symbols the file must name, class, reason, Timoshenko marker or None, exercised by).
SYMBOL_GATES = {
    "core/product_physics/src/pf_timoshenko.rs": (
        ("fn semantic_contract_id", "fn formulation_basis"), BOTH,
        "the producer's identity selection: preview-physics-2's, or the Timoshenko identity for a PF document selecting it",
        "preview-physics-timoshenko-1", PRODUCT_REFERENCES),
    "core/product_physics/src/lib.rs": (
        ("pf_timoshenko::semantic_contract_id(model)", "pf_timoshenko::formulation_basis(&model)"), BOTH,
        "the producer sites: mechanics_producer_for_model and the solved and blocked envelopes' published basis",
        "pf_timoshenko::", PRODUCT_RUNTIME),
    "core/reporting/result_export/src/reader_verdict.rs": (
        ("BINDINGS", "fn route", "fn binding_of", "fn bind_preview_shear_model"), BOTH,
        "the route's bindings by semantic_contract_id; the model-bound crossed-pair refusal", "PREVIEW_PHYSICS_TIMOSHENKO_ID", RUST_READERS),
    "core/reporting/result_export/src/preview_physics_evidence.rs": (
        ("fn validate_preview_physics_evidence",), BOTH, "the raw reader's complete identity", "preview_physics_identity_of", RUST_READERS),
    "core/reporting/result_export/src/u4b_evidence.rs": (
        ("fn preview_model_checks",), BOTH, "the model checks' table by identity", "preview_physics_identity_of", RUST_READERS),
    "core/reporting/result_export/src/derivative.rs": (
        ("PREVIEW_PHYSICS_ID",), BOTH, "the allowlist that copies contract evidence", "PREVIEW_PHYSICS_TIMOSHENKO_ID", RUST_READERS),
    "core/handoff/stress_neutral/package.py": (
        ("SUPPORTED_METHODS", "PREVIEW_METHODS"), BOTH, "supported methods and preview evidence namespaces",
        "PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_ID", PY_READERS),
    "tests/verifiers/reader_verdict.py": (
        ("_IDENTITY_ROUTES", "def binding_of", "def bind_preview_shear_model"), BOTH,
        "the verifier's bindings by identity and crossed-pair refusal", "PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_ID", PY_READERS),
    "core/reporting/report_package/src/document_form.rs": (
        ("ADMITTED_SEMANTIC_CONTRACTS",), BOTH, "the package's item 0", "PREVIEW_PHYSICS_TIMOSHENKO_ID", PACKAGE),
    "core/reporting/report_package/src/bindings.rs": (
        ("check_report_diagnostics", "pf_timoshenko::check_sections"), BOTH,
        "item 9: the Timoshenko projection's Diagnostics and limitation entry", "pf_timoshenko", PACKAGE),
    "core/reporting/report_package/src/user_release.rs": (
        ("fn check_pressure_free",), EB_ONLY,
        "the shareable derivation is a closed projection without the Timoshenko limitation entry; a Timoshenko source is refused by name",
        None, None),
    "apps/desktop/src-tauri/src/report_render_exact.rs": (
        ("fn rederive_pf_timoshenko",), BOTH, "the native render binds the pf_timoshenko section", "PF_TIMOSHENKO_SECTION", NATIVE),
    "apps/desktop/src/features/results/knownSemanticLimitations.ts": (
        ("FRESH_SEMANTIC_CONTRACT_IDS",), BOTH, "fresh-result admission (the Current result)", "PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_ID", DESKTOP),
    "apps/desktop/src/features/report/reportPackageRequest.ts": (
        ("REPORT_PACKAGE_ADMITTED_CONTRACT_IDS",), BOTH,
        "the package's admitted set; its user-release path admits preview-physics-2 only, by design (above)",
        "PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_ID", DESKTOP),
    "apps/desktop/src/features/report/renderableReportInput.ts": (
        ("diagnosticsForSections",), BOTH, "the report's limitation 0 entry and pf_timoshenko section", "pfTimoshenkoLimitationEntry", DESKTOP),
    "apps/desktop/src/test-support/contractBindings.ts": (
        ("PREVIEW_PHYSICS_CONTRACT_SHA256",), BOTH, "the tests' expected bindings, computed from the tables",
        "PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_SHA256", DESKTOP),
}


def _is_scanned(path: str) -> bool:
    if path.startswith("tests/verifiers/"):
        return True
    test = (path.startswith(("tests/", "validation/", "execution/", "fixtures/", "tools/retained_pricing/"))
            or "/tests/" in path or path.endswith(("_tests.rs", ".test.ts", ".test.tsx", ".spec.ts", "_test.py"))
            or "/test-support/" in path or "/src/test/" in path or "/e2e/" in path or "/benches/" in path or "/test_" in path)
    return not test


def _occurrences() -> dict[str, set[str]]:
    listed = subprocess.run(["git", "ls-files", "--", "*.rs", "*.py", "*.ts", "*.tsx", "*.mjs", "schemas/*"],
                            cwd=PROJECT, check=True, capture_output=True, text=True).stdout.split()
    found: dict[str, set[str]] = {}
    for path in listed:
        if not _is_scanned(path) or not (PROJECT / path).is_file():
            continue
        text = (PROJECT / path).read_text(encoding="utf-8", errors="replace")
        needles = {name for name, needle in NEEDLES.items() if needle in text}
        if needles:
            found[path] = needles
    return found


def test_every_occurrence_is_on_the_reviewed_list():
    found = _occurrences()
    expected = {path: entry[0] for path, entry in LITERAL_SITES.items()}
    unreviewed = {path: sorted(needles - expected.get(path, set())) for path, needles in found.items() if needles - expected.get(path, set())}
    stale = {path: sorted(needles - found.get(path, set())) for path, needles in expected.items() if needles - found.get(path, set())}
    assert not unreviewed, f"preview-physics-2 named at sites not on the reviewed list (classify each): {unreviewed}"
    assert not stale, f"reviewed entries that no longer occur (update the list): {stale}"


def test_each_entry_is_classified_with_its_reason_and_both_entries_name_the_timoshenko_identity():
    for table in (LITERAL_SITES, SYMBOL_GATES):
        for path, (_, label, reason, marker, exercised) in table.items():
            assert label in (BOTH, EB_ONLY) and reason, path
            text = (PROJECT / path).read_text(encoding="utf-8")
            if label == BOTH:
                assert marker and marker in text, f"{path}: an 'admits both' entry must name the Timoshenko identity ({marker})"
                assert exercised and (PROJECT / exercised).is_file(), f"{path}: no round trip exercises it"
            else:
                assert marker is None and exercised is None, path


def test_each_symbol_only_gate_still_names_its_symbols():
    for path, (symbols, *_rest) in SYMBOL_GATES.items():
        text = (PROJECT / path).read_text(encoding="utf-8")
        missing = [symbol for symbol in symbols if symbol not in text]
        assert not missing, f"{path}: the reviewed gate no longer names {missing}; review the site and update the list"
