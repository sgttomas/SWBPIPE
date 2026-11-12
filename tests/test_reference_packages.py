"""Every independent reference package under validation/references/ makes two claims: its generator
imports, reads and runs no product code, and its frozen files are the generator's output. This test
checks both for every package, from one table.

For each package in PACKAGES it:
1. verifies every line of every checksum list the package holds, each against the base directory its
   entry records (a line known to fail is a named exception with its reason, and the test fails once
   that line passes again, so the exception cannot outlive its cause);
2. applies the one independence rule (tools/validation/reference_independence.py, whose witnesses
   are in tests/test_reference_independence.py) to every .py file of the package, with the entry's
   allowances: sibling modules, modules of another reference package (pinned by that package's own
   checksum list), external modules the README says come from outside the repository, and named
   facilities and literals in named files;
3. runs each check command the package's README gives (never a mode that writes into the package)
   and requires exit 0, or, for a check known to fail, exactly its recorded failure.

An allowance is an explanation, not a waiver: each one carries its reason. One whose reason the
package's files do not give is recorded as UNEXPLAINED and named in that list. What this test does
not regenerate is listed in `taken_on_trust()`, and its count is pinned by TAKEN_ON_TRUST_COUNT.

The generators run as `python3 -I -B` from their package directory, one per check, each within
CHECK_TIMEOUT_S seconds. No product binary is needed, so the test runs under
OPENPIPESTRESS_NO_CARGO_BUILD=1 (tests/conftest.py), which is how it is run on the shared host.
"""
from __future__ import annotations

import hashlib
import re
import subprocess
import sys
from dataclasses import dataclass, field
from pathlib import Path

import pytest

ROOT = Path(__file__).resolve().parents[1]
REFS = ROOT / "validation" / "references"
# The package walk below is a scan the CI selector cannot fully read.
CI_READS = ("validation/references/**",)
sys.path.insert(0, str(ROOT / "tools" / "validation"))
import reference_independence as RI  # noqa: E402

CHECK_TIMEOUT_S = 120
SUMS_LINE = re.compile(r"^([0-9a-f]{64}) [ *](.+)$")
CHECKSUM_LIST = re.compile(r"(^|_)SHA256SUMS(\..+)?$")
UNEXPLAINED_REASON = "unexplained"
TMP = "<new scratch directory>"  # replaced by a fresh, absent directory under pytest's tmp_path

LOCKFILE_COMMIT = ("changed by 3c998e749 (2026-11-02, merged on 2026-11-03: \"RT-A4 wiring: lockfiles take "
                   "result_export's kernel dependency\") after the list was last updated; the runner is harness "
                   "that drives the product, not reference (README), and the frozen values reproduce without it")
SCHEMA_CHANGE = ("schemas/load_reference_state.schema.json gained `allOf` in 0c480a9e5 (2026-11-07), which "
                 "m3_view.schema_supported refuses, so the shared M3 judge self-test stops with SchemaUnsupported")


PROSE = "prose in a docstring or message, not a path"


@dataclass(frozen=True)
class Sums:
    """A checksum list (package-relative) and the package-relative directory its names are relative to."""
    path: str
    base: str = "."
    failing: dict = field(default_factory=dict)  # {name: reason}: lines known to fail
    historical: str | None = None  # reason it is kept but not verified; its own bytes must be pinned
    pins: int | None = None  # m3 pin format `<sha> <route> <kind> <what>`: how many name a committed file


@dataclass(frozen=True)
class Check:
    """A check command from the package's README, run from `cwd` (package-relative)."""
    argv: tuple
    cwd: str = "."
    stdout_equals: str | None = None  # package-relative file the stdout must equal byte for byte
    known_failure: str | None = None  # reason; the run must fail exactly as `must_show` records
    must_show: tuple = ()  # text the combined output must contain (for a known failure)
    fail_lines: tuple = ()  # for a known failure: the complete set of lines starting "FAIL"


@dataclass(frozen=True)
class Package:
    deliverables: str
    sums: tuple = ()
    no_sums: bool = False
    siblings: dict = field(default_factory=dict)  # {module: reason}; the module is a .py in the package
    foreign: dict = field(default_factory=dict)  # {module: ("pkg/path/file.py", reason)}
    external: dict = field(default_factory=dict)  # {module: reason}; not in the repository
    facilities: dict = field(default_factory=dict)  # {(file, forbidden module or name): reason}
    literals: dict = field(default_factory=dict)  # {(file, RI.literal_key): reason}
    directories: dict = field(default_factory=dict)  # {directory under generators/: reason}
    outside: dict = field(default_factory=dict)  # {directory: reason}; a product check, outside the rule
    checks: tuple = ()
    not_regenerated: tuple = ()  # ((what, reason), ...)
    other_test: str | None = None  # a maintained test that regenerates this package, and its state
    reproduction: str | None = None  # one line: what reproduces today and what fails, on what
    defects: tuple = ()  # ((file, text, reason), ...): a known defect, asserted present until repaired


# Top-level directories that hold several packages rather than being one.
CONTAINERS = {
    "f1_modal": "two packages (continuum, mass_discrete), each with its own README, generators and SHA256SUMS; "
                "published/ holds a README only",
}

DYNAMICS_TEST = "tests/test_dynamics_references.py"

# Reproduction status today of the three packages DEL-04-08's AC-001 and AC-002 rest on. Each failing
# part is an entry below (a known failing checksum line or check) that fails again the moment its
# explanation stops being true.
O4_REPRODUCTION = (
    "does not reproduce on this branch (PR 276, not yet merged, admits one named successor of the engine and of "
    "twelve inputs): its own check (run.sh; generators/o4_reference.py and o4_source_twins.py) refuses "
    "../kd5_timoshenko/kd5t_engine.py, which it pins at SHA-256 c91f28c6... while cdf98fd8c (2026-10-28) changed it "
    "to 05df1ba6...; its frozen files are unchanged and its SHA256SUMS verifies; declared_problem/ is a product "
    "check outside the rule, and its oracle self-test passes")
KD5T_REPRODUCTION = (
    "reproduces: kd5t_engine.py --check passes, and the 964-model set, its controls and the 64-model frozen subset "
    "regenerate to their pins (kd5t_generate.py check, m3_judge/m3_reproduce.py); kd5t_generate.py check then exits 1 "
    "on the shared M3 judge's self-test (m3_judge's stale runner/Cargo.lock line, the schema reader's refusal of "
    "allOf), and its own SHA256SUMS fails on runner/Cargo.lock (3c998e749; repaired by PR 273, not yet merged); the "
    "runner is not built here, and its build script runner/build.sh still uses pre-split paths")
M3_REPRODUCTION = (
    "its six pinned references reproduce (m3_reproduce.py); its self-test fails on two things, both waiting for PR "
    "217: the stale runner/Cargo.lock line of its SHA256SUMS (3c998e749), and its schema reader refusing the allOf that 0c480a9e5 "
    "added to schemas/load_reference_state.schema.json; the runner-driven judging pass is not run here")
M3_SUMS_FAIL = "FAIL SHA256SUMS verified (17 files; mismatched: ['runner/Cargo.lock'])"
KD5T = "kd5_timoshenko/kd5t_engine.py"


def _loads(file: str, reason: str) -> dict:
    """A load of one module from its file: `import importlib.util`, then `spec_from_file_location`,
    `module_from_spec` and `spec.loader.exec_module`, each a facility the rule names."""
    return {(file, name): reason for name in ("importlib", "spec_from_file_location", "module_from_spec", "exec_module")}


def _pool(file: str, start: str | None = "fork") -> dict:
    """A worker pool over this file's own function: `import multiprocessing` and, for the fork start
    method, the constant "fork" (a facility name the rule refuses as text)."""
    reason = ("a worker pool (multiprocessing Pool, start method {}) mapping this file's own function over its "
              "items; it starts no other program").format(start or "spawn")
    out = {(file, "multiprocessing"): reason}
    if start:
        out[(file, start)] = reason
    return out


def _plain(gen: str, *args: str, deliverables: str, **kw) -> Package:
    """One generator whose default (or named) mode rebuilds in memory and compares byte for byte."""
    return Package(deliverables, sums=(Sums("SHA256SUMS"),), checks=(Check(("generators/" + gen, *args)),), **kw)


PACKAGES: dict[str, Package] = {
    "combination_certificate": _plain("certificate_reference.py", deliverables="DEL-05-02"),
    "op1b_difference_basis": _plain("op1b_reference.py", deliverables="DEL-05-02"),
    "op_mf_held_status": _plain("held_status_reference.py", deliverables="DEL-05-02"),
    "reference_system": _plain(
        "rs_reference.py", deliverables="DEL-09-01",
        literals={("generators/rs_reference.py", "fixtures"): "reads the tranche-1 fixture it is the reference for (README)",
                  ("generators/rs_reference.py", "fixtures/reference_system/reference_system.json"):
                      "reads the tranche-1 fixture it is the reference for (README)"}),
    "rt_a3_s6": _plain("rt_a3_s6_reference.py", deliverables="DEL-04-07"),
    "rt_a3_constant_effort": _plain("rt_a3_constant_effort_reference.py", "--check", deliverables="DEL-04-07"),
    "rt_a3_nonlinear": _plain("rt_a3_nonlinear_reference.py", "--check", deliverables="DEL-04-07"),
    "rt_a3_reducer": _plain("rt_a3_reducer_reference.py", "--check", deliverables="DEL-03-05, DEL-04-07"),
    "rt_a3_reducer_2b": _plain("rt_a3_2b_reference.py", "--check", deliverables="DEL-04-07"),
    "rt_a3_tee": _plain("rt_a3_tee_reference.py", "--check", deliverables="DEL-03-04, DEL-04-07"),
    "rt_a4_combinations": Package(
        "DEL-05-02", sums=(Sums("SHA256SUMS"), Sums("v5/SHA256SUMS", base="v5")),
        checks=(Check(("generators/rt_a4_reference.py",)), Check(("generators/rt_a4_v5_reference.py",), cwd="v5"))),
    "rt_g5_s2_conditions": _plain(
        "rt_g5_s2_reference.py", deliverables="DEL-05-01",
        facilities={("generators/rt_g5_s2_reference.py", "subprocess"):
                    "node_check (line 438) runs `node -e` when node is on PATH, to compare JavaScript's Number#toString "
                    "with repr on the decimal samples (README); node is not product code and the frozen values do not "
                    "depend on it"}),
    "skew_restraints": _plain("skew_reference.py", deliverables="DEL-04-03"),
    "w9_benchmarks": _plain("w9_reference.py", deliverables="DEL-04-02, DEL-04-03, DEL-04-07, DEL-09-01"),
    "w15_component_weight": _plain("w15b_reference.py", deliverables="DEL-03-05"),
    "w15_rigid_scaling": _plain("w15_reference.py", deliverables="DEL-03-05"),
    "w15c_eccentric_reducer": _plain("w15c_reference.py", deliverables="DEL-03-05"),
    "w16_exact_combinations": _plain("w16b_reference.py", deliverables="DEL-05-02"),
    "w16_exact_equivalent_static": _plain("w16c_reference.py", deliverables="DEL-05-01"),
    "w16_imposed_motion": _plain("w16_reference.py", deliverables="DEL-04-03, DEL-09-01"),
    "w16_nonlinear_motion": _plain("w16_nl_reference.py", deliverables="DEL-04-03"),
    "w16_timoshenko_partial_wind": _plain("w16t_reference.py", deliverables="DEL-05-01"),
    "t4_u4b": _plain("u4b_reference.py", deliverables="DEL-04-07"),
    "t4_u5a": _plain("u5a_reference.py", deliverables="DEL-03-06, DEL-04-07"),
    "t4_u5b": _plain("u5b_reference.py", deliverables="DEL-04-07"),
    "t4_u6": _plain("u6_reference.py", deliverables="DEL-05-03"),
    "t4_u8": _plain(
        "u8_reference.py", deliverables="DEL-05-03",
        literals={(f, lit): "Twin A's inputs come from the product's ODWALL fixture, pinned by SHA-256 (README, Twin A inputs)"
                  for f, lit in (("generators/u8_reference.py", "core"), ("generators/u8_reference.py", "tests"),
                                 ("generators/u8_reference.py", "fixtures"),
                                 ("generators/u8_reference.py",
                                  "core/product_physics/tests/fixtures/pressure_reference/SOURCE_ODWALL_EXPECTATIONS.json"))}
        | {("generators/u8_reference.py",
            "core/product_physics/tests/fixtures/pressure_reference/SOURCE_ODWALL_EXPECTATIONS.json (sha256 "):
               PROSE + " (provenance text in the output, before the fixture's digest)"}),
    "pf_timoshenko": _plain(
        "pf_reference.py", deliverables="DEL-05-03",
        facilities=_loads("generators/pf_reference.py",
                          "loads ../t4_u4a/u4a_engine.py, reference code whose arc treatment it reuses and cross-checks "
                          "(README); t4_u4a's SHA256SUMS pins that file")),
    "t4_u4a": Package("DEL-04-07", sums=(Sums("SHA256SUMS"),), siblings={"u4a_engine": "the engine u4a_generate.py drives (README)"},
                      checks=(Check(("u4a_generate.py",)),)),
    "hanger_sizing": Package(
        "DEL-04-11", no_sums=True,
        checks=(Check(("generate_r3.py", "--check")), Check(("generate_r125.py", "--check")))),
    "kd5_member_bound": Package(
        "DEL-04-01", no_sums=True,
        facilities={("two_body_reference.py", name):
                    "compiles (mode 'exec') and runs the sibling notice_reference.py's RN128 operations, read from its "
                    "own file (README)"
                    for name in ("exec", "compile")},
        checks=(Check(("notice_reference.py",)), Check(("two_body_reference.py",)))),
    "kd5_nonlinear": Package(
        "DEL-04-01, DEL-09-03", no_sums=True,
        # No README; the module docstring gives the command: "Run it to verify the maintained oracles".
        checks=(Check(("reference.py",)),)),
    "t4_i7": Package(
        "DEL-04-07", sums=(Sums("SHA256SUMS"),), siblings={"u2_engine": "the engine u2_generate.py drives (README)"},
        literals={("_run_records/u2_generate.py", "core"): "a case-record key marking a core case, not a path"},
        checks=(Check(("_run_records/u2_generate.py", "--check")),)),  # regenerates in a temporary directory
    "t4_i8": Package(
        "DEL-04-07", sums=(Sums("SHA256SUMS"),),
        literals={("_run_records/derive_rebuilt_references.py",
                   "PP/tests/fixtures/pressure_reference/SOURCE_ODWALL_EXPECTATIONS.json"):
                  "provenance text in the output naming the twin fixture, which is read by path only to compare (README)",
                  **{("_run_records/derive_rebuilt_references.py", "sha256:" + h): PROSE + f" ({where})" for h, where in (
                      ("db269f86abb6bef157d4ac47e05d16c58c764ad6f066ce0f155242d98e7f2acb",
                       "provenance text in the output, 'PP/tests/pressure_section_geometry.rs: fixture(...)'"),
                      ("1675b92658af244462d80da455f6d2a644445d59c52f80bcd1736ab7771e1fc0",
                       "provenance text in the output, 'PP/tests/pressure_runtime.rs model(...) at ed012c7ccf'"))}},
        checks=(Check(("_run_records/check_reference_json.py", "rebuilt_reference_cases.json")),),
        not_regenerated=(("rebuilt_reference_cases.json",
                          "the README gives no check mode: derive_rebuilt_references.py only writes to a given path "
                          "and needs the product fixture path; check_reference_json.py re-evaluates the frozen file"),)),
    "t4_i9": Package(
        "DEL-04-01", sums=(Sums("SHA256SUMS"),),
        facilities={**_loads("t4_i17_arc_refs.py", "loads the RV129 source named on its command line (README)"),
                    **_loads("t4_i17_bit_pins.py", "loads the certificate specification named on its command line (README)"),
                    **_loads("check_series.py", "loads the sibling load_series_gen.py it checks (README)"),
                    **_loads("_run_records/rv129_ref.py",
                             "RV129's source, copied with its bytes unchanged (README); loads the probe named on its command line")},
        checks=(Check(("t4_i17_arc_refs.py", "_run_records/rv129_ref.py"), stdout_equals="arc_load_references.txt"),
                Check(("t4_i17_bit_pins.py", "_run_records/arc_cert_probe_r01.py"), stdout_equals="arc_certificate_pins.txt"),
                Check(("check_series.py",))),),  # load_series_gen.py's output is pasted into the product, not frozen here
    "t4_i6": Package(
        "DEL-03-03, DEL-04-01", sums=(Sums("SHA256SUMS"), Sums("round_01/SHA256SUMS", base="round_01")),
        siblings={"curved_ref": "the frozen element generator, _run_records/curved_ref.py (README; round 01's scripts "
                                "imported it from rerun00/, the same bytes)",
                  "r01_lib": "round 01's shared library (README)",
                  "i1_curved122": "a round-01 script imported by i2_cskew.py (README)"},
        facilities={("_run_records/freeze_u1.py", "platform"):
                    "reads platform.python_version() only (lines 376, 382), to record the interpreter in the run record"},
        external={"curved_ef": "D1's curved_ef.py, imported unchanged from a directory given on the command line (README, kd5_models/)",
                  "probe_d5_check": "T3's V1 probe_d5_check.py, imported unchanged from a directory given on the command line (README, kd5_models/)",
                  "rv131_rv_element": UNEXPLAINED_REASON},
        checks=(Check(("t4_i14_l30_matrix.py", "_run_records"), stdout_equals="t4_i14_l30_matrix.stdout.txt"),
                Check(("t4_i15_short_arc_matrices.py", "_run_records"), stdout_equals="t4_i15_short_arc_matrices.stdout.txt")),
        not_regenerated=(("u1_reference_cases.json", "the README gives no check mode: freeze_u1.py wrote it in the archived run (RUN.txt)"),
                         ("round_01/", "the README gives no check mode; its scripts were run in scratch in repair round 01"),
                         ("kd5_models/", "needs D1's and T3's V1 modules from outside the repository (README)"))),
    "t4_i12": Package(
        "DEL-03-06", sums=(Sums("SHA256SUMS"),),
        facilities={("generators/u3_reference.py", "system"):
                    "the key \"system\" of the criteria table in the output (system-level comparison criteria, line "
                    "1892), a word in a table, not os.system"},
        literals={("generators/u3_reference.py", "sha256:1f41f3fc1223aabb5f3a3846acc9df1bd40692fd11621e947ccbd38fd5053b07"):
                      PROSE + " (a case description in the output, 'The invented demo (P/fixtures/product_preview/...')",
                  ("generators/u3_round02.py", "sha256:c82e855e308e0d325ae0c4c5167a17bff83c0201965c6d6b0e367bc7ec88cf1a"):
                      PROSE + " (a source note in the output, 'the demo fixture's SH-140 hanger, verbatim (P/fixtures/...')"},
        not_regenerated=(("u3_reference_cases.json",
                          "the README gives no check mode: both commands write to a given output path"),)),
    "t4_o13": Package(
        "DEL-04-01", sums=(Sums("SHA256SUMS"),),
        not_regenerated=(("o13_fk_references.txt",
                          "the README gives no check mode: o13_fk_references.py writes the file into its working directory"),)),
    "t3_r1": Package(
        "DEL-09-01",
        sums=(Sums("REFERENCES/_run_records/SHA256SUMS", base="REFERENCES"),
              Sums("REFERENCES/_run_records/SHA256SUMS.revision1", base="REFERENCES",
                   historical="the revision-1 hashes, kept on purpose (README); superseded by SHA256SUMS")),
        siblings={"references": "the derivation, imported by the author self-check (README)"},
        checks=(Check(("_run_records/selfcheck_stiffness.py",), cwd="REFERENCES"),),
        not_regenerated=(("REFERENCES/references.json",
                          "references.py took 679 s and 722 s (README), beyond the limit, and its only mode writes "
                          "references.json; the author self-check runs instead"),)),
    "kd5_timoshenko": Package(
        "DEL-05-03",
        sums=(Sums("SHA256SUMS", failing={"runner/Cargo.lock": LOCKFILE_COMMIT}),),
        siblings={"kd5t_engine": "the exact reference engine (README)"},
        facilities={**_pool("kd5t_census.py"), **_pool("kd5t_generate.py"),
                    ("kd5t_compare.py", "subprocess"): "--selftest runs the shared judge's m3_judge/m3_selftest.py",
                    ("kd5t_generate.py", "subprocess"): "`check` runs this package's kd5t_compare.py --selftest"},
        defects=(("runner/build.sh", "REL=projects/chirality-piping/validation/references/kd5_timoshenko/runner",
                  "lines 15-16 still use paths from before the repository split (projects/chirality-piping/), so the "
                  "runner build script cannot work as written; to be repaired with the judge repair after PR 217"),),
        checks=(Check(("kd5t_engine.py", "--check")),
                Check(("kd5t_generate.py", "check"),
                      known_failure="the frozen subset regenerates byte for byte; the compare self-test then stops on "
                                    "m3_judge's runner/Cargo.lock line and on the M3 judge's schema refusal. "
                                    + LOCKFILE_COMMIT + "; " + SCHEMA_CHANGE,
                      must_show=("engine self-check: 58 checks pass", "frozen subset regenerated byte for byte (64 models)",
                                 "SchemaUnsupported: load_reference_state.schema.json: unsupported schema keywords ['allOf']"),
                      fail_lines=(M3_SUMS_FAIL,))),
        reproduction=KD5T_REPRODUCTION),
    "pf_timoshenko_m3t": Package(
        "DEL-05-03", sums=(Sums("SHA256SUMS"),),
        siblings={"pft_engine": "the PF reference engine (README)", "pft_generate": "the PF set generator (README)",
                  "pft_intended": "the intended-system module (README)"},
        facilities={("pft_census.py", "eval"):
                    "the dict key \"eval\" of the dump's evaluation and groups records (an evaluation index: the groups "
                    "line's eval names one evaluation line, pft_census.py:186), a word in a record, not the builtin",
                    **_pool("pft_census.py"), **_pool("pft_generate.py"),
                    ("pft_compare.py", "subprocess"): "--selftest runs the shared judge's m3_judge/m3_selftest.py",
                    ("pft_generate.py", "subprocess"):
                        "`check` runs this package's pft_compare.py --selftest; the `units` mode (units_check, line 731) "
                        "runs the product's units authority ($OPENPIPESTRESS_UNITS_BIN) to confirm GPa moduli convert "
                        "bit for bit to the Pa operands the references were computed from (README); a product run in a "
                        "check mode, reported, and no frozen value comes from it"},
        foreign={"kd5t_engine": (KD5T, "the exact engine the PF engine extends (README, 'one of them runs kd5t_engine's 58')"),
                 "kd5t_generate": ("kd5_timoshenko/kd5t_generate.py", "the judged-set machinery the PF generator reuses (README)"),
                 "kd5t_census": ("kd5_timoshenko/kd5t_census.py", "the census the PF census extends (README)"),
                 "m3_compare": ("m3_judge/m3_compare.py", "the shared M3 judge (m3_judge README)"),
                 "m3_json": ("m3_judge/m3_json.py", "the shared M3 judge (m3_judge README)"),
                 "m3_synth": ("m3_judge/m3_synth.py", "the shared M3 judge (m3_judge README)"),
                 "m3_view": ("m3_judge/m3_view.py", "the shared M3 judge (m3_judge README)")},
        checks=(Check(("pft_engine.py", "--check")),
                Check(("pft_generate.py", "check"),
                      known_failure="the frozen subset regenerates byte for byte; the compare self-test then stops on "
                                    "m3_judge's runner/Cargo.lock line and on the M3 judge's schema refusal. "
                                    + LOCKFILE_COMMIT + "; " + SCHEMA_CHANGE,
                      must_show=("PF engine self-check: 130 checks pass", "frozen subset regenerated byte for byte (100 models)",
                                 "SchemaUnsupported: load_reference_state.schema.json: unsupported schema keywords ['allOf']"),
                      fail_lines=(M3_SUMS_FAIL,)))),
    "m3_judge": Package(
        "DEL-04-01",
        sums=(Sums("SHA256SUMS", failing={"runner/Cargo.lock": LOCKFILE_COMMIT}),
              Sums("REFERENCE_SHA256SUMS", base="..", pins=3)),
        siblings={m: "a module of the one M3 judge (README, Files)"
                  for m in ("m3_json", "m3_types", "m3_view", "m3_compare", "m3_report", "m3_synth", "m3_regress")},
        facilities={**_pool("m3_report.py"),
                    ("m3_report.py", "subprocess"):
                        "runs `git rev-parse --verify` to bind the judged records to a commit of --checkout (README)",
                    ("m3_view.py", "subprocess"):
                        "runs `git rev-parse` and `git cat-file blob HEAD:<path>` to read REFERENCE_SHA256SUMS from HEAD "
                        "(README, round 7)",
                    ("m3_reproduce.py", "subprocess"):
                        "runs the reference generators (kd5t_generate.py, pft_generate.py) with python3 -I -B into its "
                        "scratch directory (README, Reproducible)",
                    ("m3_selftest.py", "subprocess"):
                        "runs a fresh python3 -I -B that imports m3_compare, the isolation check (README, Self-test)",
                    ("m3_regress.py", "subprocess"):
                        "the self-test runs git in throwaway repositories, the judge's own m3_report.py there, and run.sh "
                        "with cargo set to `false` to exercise its refusals (README, Self-test)"},
        literals={**{("m3_view.py", lit): why for lit, why in (
                      ("fixtures", "reads the public semantic contracts under fixtures/results (README, Independence)"),
                      ("schemas", "reads the public result schemas' contract_evidence definition at judge time (m3_view.py)"))},
                  **{("m3_regress.py", lit): "path segments of a throwaway repository the self-test builds to exercise "
                                             "run.sh/build.sh refusals; it copies the public contracts and schemas there"
                     for lit in ("fixtures", "schemas", "core", "apps")}},
        # The README's reproducibility check; it writes only into the new scratch directory it is given.
        checks=(Check(("m3_reproduce.py", TMP)),
                Check(("m3_selftest.py",),
                      known_failure="the judge's self-test stops: " + LOCKFILE_COMMIT + "; " + SCHEMA_CHANGE,
                      must_show=("SchemaUnsupported: load_reference_state.schema.json: unsupported schema keywords ['allOf']",),
                      fail_lines=(M3_SUMS_FAIL,))),
        reproduction=M3_REPRODUCTION),
    "o4_retained_source": Package(
        "DEL-04-08", sums=(Sums("SHA256SUMS"),),
        siblings={"o4_reference": "the unit's pipeline, reused by the source twins (README)"},
        foreign={"kd5t_engine": (KD5T, "the exact engine, imported unchanged and bound by SHA-256 (README)")},
        facilities={**_loads("generators/o4_source_twins.py",
                             "loads tools/fixtures/retained_precision_corpus.py, pinned by SHA-256, for its DAG decoder "
                             "`expand` only (README, input-only sources); product tooling run by a generator, reported"),
                    **_pool("generators/o4_reference.py", start=None)},
        literals={**{("generators/o4_source_twins.py", lit):
                     "an input-only source of the source twins, pinned by SHA-256 (README)"
                     for lit in ("fixtures/product_preview/pressure_source", "fixtures/results/j7_rf_census_inputs.json",
                                 "tools/fixtures/retained_precision_corpus.py")
                     + tuple(f"fixtures/product_preview/pressure_source/{d}/{n}.request.json" for d, n in (
                         ("physics_source", "n05"), ("physics_source", "n06"), ("physics_source", "n05_unicode"),
                         ("physics_source", "n05_units"), ("physics_source", "fields"), ("physics_source", "mixed"),
                         ("physics_source", "mixed_units"), ("load_reference_source", "n05"),
                         ("load_reference_source", "n06"), ("load_reference_source", "eigen_motion"),
                         ("load_reference_source", "fields"), ("load_reference_source", "mixed")))}},
        outside={"declared_problem": "a product check that lives in the package's directory: run.sh builds the product, "
                                     "runs its dump entry and the headless runner on twins and judges the product's "
                                     "output (declared_problem/README.md); listed among DEL-04-08's code paths beside "
                                     "its tests; not reference code"},
        checks=(Check(("generators/o4_reference.py", "--workers", "1"),
                      known_failure="the generator binds kd5t_engine.py by SHA-256 c91f28c6…, and kd5_timoshenko changed "
                                    "that file in cdf98fd8c (2026-10-28) after O4 was frozen; reported, not repaired",
                      must_show=("kd5t_engine.py changed (sha256 05df1ba6c4d5f47c64b61c1018a91e3aade820f043bd1b8a080fe3607b27ec5d); "
                                 "this unit binds c91f28c6bce4a6777c63357fc82b8a0d1cd5a3ca48d0b1a756ef29706be8314e",)),
                Check(("generators/o4_source_twins.py", "--workers", "1"),
                      known_failure="imports o4_reference, which refuses the changed kd5t_engine.py as above",
                      must_show=("this unit binds c91f28c6bce4a6777c63357fc82b8a0d1cd5a3ca48d0b1a756ef29706be8314e",)),
                Check(("oracle.py", "--selftest"), cwd="declared_problem")),  # the product check's own oracle, pure Python
        reproduction=O4_REPRODUCTION),
    "f1_modal/continuum": Package(
        "DEL-04-09", sums=(Sums("SHA256SUMS"),),
        siblings={m: "a module of the continuum generator (README)"
                  for m in ("numerics", "continuum", "fe_convergence", "inertia_witnesses")},
        other_test=DYNAMICS_TEST,
        # From the package directory: python3 -I -B generators/generate.py (about 11 s).
        not_regenerated=(("references", f"regenerated by {DYNAMICS_TEST} (not yet on main)"),)),
    "f1_modal/mass_discrete": Package(
        "DEL-04-09", sums=(Sums("SHA256SUMS"),),
        siblings={m: "a module of the mass-discrete generator (README)"
                  for m in ("numeric", "elements", "model", "cases", "eigen", "crosscheck")},
        other_test=DYNAMICS_TEST,
        # From the package directory: python3 -I -B generators/f1_mass_discrete.py (about 5 s, several processes).
        not_regenerated=(("references", f"regenerated by {DYNAMICS_TEST} (not yet on main)"),)),
}

# Allowances whose reason the package's own files do not give. Each needs its owner's explanation.
UNEXPLAINED = {
    ("t4_i6", "external", "rv131_rv_element"):
        "round_01/_run_records/i1_curved122.py and i2_cskew.py import it inside try/except ImportError from an "
        "inputs/ directory that is not kept here; the README does not say what it is",
}

TAKEN_ON_TRUST_COUNT = 14


def taken_on_trust() -> list[tuple[str, str, str]]:
    """Every frozen output this test does not reproduce: (package, what, reason)."""
    out = [(name, what, why) for name, pkg in PACKAGES.items() for what, why in pkg.not_regenerated]
    out += [(name, f"check {' '.join(c.argv)}", c.known_failure)
            for name, pkg in PACKAGES.items() for c in pkg.checks if c.known_failure]
    return out


# ---------------------------------------------------------------------------------------------- helpers

def _is_package_dir(path: Path) -> bool:
    return any(p.suffix == ".py" or CHECKSUM_LIST.search(p.name) for p in path.iterdir() if p.is_file())


def _files(pkg_dir: Path, pattern: str) -> list[Path]:
    return sorted(p for p in pkg_dir.rglob(pattern) if p.is_file() and "__pycache__" not in p.parts)


def _sums_lines(path: Path) -> list[tuple[str, str]]:
    lines = []
    for n, raw in enumerate(path.read_text(encoding="utf-8").splitlines(), 1):
        if not raw.strip():
            continue
        m = SUMS_LINE.match(raw)
        assert m, f"{path.relative_to(ROOT)}:{n} is not a checksum line: {raw!r}"
        lines.append((m.group(1), m.group(2)))
    return lines


def _sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def _foreign(rel: str, reason: str) -> RI.Foreign:
    """A module of another reference package (`rel` relative to validation/references), pinned by the
    verified checksum list of its own package that lists it (or, when none lists it, the first, so the
    rule reports it as not listed)."""
    owner = next((name for name in PACKAGES if rel.startswith(name + "/")), None)
    assert owner is not None, f"{rel} is in no package of the table"
    pkg_dir, file = REFS / owner, REFS / rel
    lists = [s for s in PACKAGES[owner].sums if not s.historical and s.pins is None]
    assert lists, f"{rel}: its package {owner} has no checksum list to pin it"
    for sums in lists:
        base = pkg_dir / sums.base
        if any((base / name).resolve() == file.resolve() for _, name in _sums_lines(pkg_dir / sums.path)):
            return RI.Foreign(file, pkg_dir / sums.path, base, reason)
    return RI.Foreign(file, pkg_dir / lists[0].path, pkg_dir / lists[0].base, reason)


def allowances(name: str) -> RI.Allowances:
    pkg = PACKAGES[name]
    return RI.Allowances(siblings=pkg.siblings, external=pkg.external, facilities=pkg.facilities,
                         literals=pkg.literals, directories=pkg.directories, outside=pkg.outside,
                         foreign={m: _foreign(rel, why) for m, (rel, why) in pkg.foreign.items()})


# ---------------------------------------------------------------------------------------------- tests

def test_every_package_is_classified():
    problems = []
    for top in sorted(p for p in REFS.iterdir() if p.is_dir()):
        name = top.name
        if name in CONTAINERS:
            for sub in sorted(top.rglob("*")):
                if sub.is_dir() and "__pycache__" not in sub.parts and _is_package_dir(sub):
                    rel = sub.relative_to(REFS).as_posix()
                    if not any(rel == k or rel.startswith(k + "/") for k in PACKAGES):
                        problems.append(f"{rel} holds a generator or a checksum list and no entry covers it")
        elif name not in PACKAGES:
            problems.append(f"{name}: no entry in PACKAGES; classify it (its checksum lists, generators, check command "
                            f"and declaring deliverable) in tests/test_reference_packages.py")
    for name in PACKAGES:
        if not (REFS / name).is_dir():
            problems.append(f"{name}: an entry with no package")
        elif "/" in name and name.split("/", 1)[0] not in CONTAINERS:
            problems.append(f"{name}: a nested entry whose top-level directory is not a container")
    assert not problems, "\n".join(problems)


def test_unexplained_allowances_are_named():
    found = set()
    for name, pkg in PACKAGES.items():
        for kind, table in (("sibling", pkg.siblings), ("external", pkg.external), ("facility", pkg.facilities),
                            ("literal", pkg.literals), ("directory", pkg.directories), ("outside", pkg.outside)):
            for key, why in table.items():
                reason = why[1] if isinstance(why, tuple) else why
                assert reason and reason.strip(), f"{name}: {kind} {key!r} has no reason"
                if reason == UNEXPLAINED_REASON:
                    found.add((name, kind, key))
        for key, (_path, reason) in pkg.foreign.items():
            assert reason.strip(), f"{name}: foreign {key!r} has no reason"
            if reason == UNEXPLAINED_REASON:
                found.add((name, "foreign", key))
        for what, reason in pkg.not_regenerated:
            assert reason.strip(), f"{name}: {what} not regenerated without a reason"
            if reason == UNEXPLAINED_REASON:
                found.add((name, "not_regenerated", what))
    assert found == set(UNEXPLAINED), f"unexplained allowances {sorted(found)} differ from UNEXPLAINED {sorted(UNEXPLAINED)}"


def test_the_ac_001_and_ac_002_packages_state_their_reproduction():
    """DEL-04-08's AC-001 and AC-002 rest on these three; each states what reproduces today."""
    for name in ("o4_retained_source", "kd5_timoshenko", "m3_judge"):
        assert PACKAGES[name].reproduction, name
        print(f"\n{name}: {PACKAGES[name].reproduction}")


@pytest.mark.parametrize("name", sorted(n for n, p in PACKAGES.items() if p.defects))
def test_a_known_defect_is_still_there(name):
    """A known defect is an entry that fails the moment it is repaired, so the entry goes with it."""
    for file, text, reason in PACKAGES[name].defects:
        assert reason.strip()
        assert text in (REFS / name / file).read_text(encoding="utf-8"), (
            f"{name}/{file} no longer holds {text!r}; the defect looks repaired, so remove its entry")


def test_taken_on_trust_is_counted():
    rows = taken_on_trust()
    print("\nTaken on trust (not reproduced by this test):")
    for row in rows:
        print("  " + " | ".join(row))
    assert len(rows) == TAKEN_ON_TRUST_COUNT, (
        f"{len(rows)} outputs are taken on trust, TAKEN_ON_TRUST_COUNT says {TAKEN_ON_TRUST_COUNT}; "
        "a change to that list is a decision, so change the count with it")


@pytest.mark.parametrize("name", sorted(PACKAGES))
def test_checksums(name):
    pkg, pkg_dir = PACKAGES[name], REFS / name
    present = {p.relative_to(pkg_dir).as_posix() for p in _files(pkg_dir, "*") if CHECKSUM_LIST.search(p.name)}
    declared = {s.path for s in pkg.sums}
    assert present == declared, f"{name}: checksum lists present {sorted(present)}, declared {sorted(declared)}"
    assert pkg.no_sums == (not declared), f"{name}: no_sums must be set exactly when the package has no checksum list"
    problems = []
    for sums in pkg.sums:
        path, base = pkg_dir / sums.path, (pkg_dir / sums.base).resolve()
        if sums.historical:
            covered = any(((pkg_dir / o.base) / n).resolve() == path.resolve()
                          for o in pkg.sums if not o.historical and o.pins is None for _, n in _sums_lines(pkg_dir / o.path))
            assert covered, f"{name}: historical list {sums.path} is not itself pinned by a verified list"
            continue
        if sums.pins is not None:
            verified = 0
            for raw in path.read_text(encoding="utf-8").splitlines():
                if not raw.strip() or raw.startswith("#"):
                    continue
                digest, _route, _kind, what = raw.split(None, 3)
                target = base / what.split()[0]
                if target.is_file():
                    verified += 1
                    if _sha256(target) != digest:
                        problems.append(f"{sums.path}: {what.split()[0]} differs from its pin")
            assert verified == sums.pins, f"{name}: {sums.path} names {verified} committed files, the entry says {sums.pins}"
            continue
        failing = set()
        for digest, rel in _sums_lines(path):
            target = base / rel
            if not target.is_file() or _sha256(target) != digest:
                failing.add(rel)
        unexpected = failing - set(sums.failing)
        repaired = set(sums.failing) - failing
        problems += [f"{sums.path}: {rel} fails (base {sums.base})" for rel in sorted(unexpected)]
        problems += [f"{sums.path}: {rel} passes again; remove its exception" for rel in sorted(repaired)]
    assert not problems, f"{name}:\n" + "\n".join(problems)


@pytest.mark.parametrize("name", sorted(PACKAGES))
def test_generators_are_independent(name):
    problems = RI.scan_package(REFS / name, allowances(name))
    assert not problems, f"{name}:\n" + "\n".join(problems)


CHECKS = [(name, i) for name in sorted(PACKAGES) for i in range(len(PACKAGES[name].checks))]


@pytest.mark.parametrize("name, index", CHECKS, ids=[f"{n}-{i}" for n, i in CHECKS])
def test_generator_check(name, index, tmp_path):
    check = PACKAGES[name].checks[index]
    cwd = REFS / name / check.cwd
    argv = [str(tmp_path / "scratch") if a == TMP else a for a in check.argv]
    run = subprocess.run([sys.executable, "-I", "-B", *argv], cwd=cwd, capture_output=True,
                         timeout=CHECK_TIMEOUT_S)
    output = (run.stdout + run.stderr).decode("utf-8", "replace")
    label = f"{name}: python3 -I -B {' '.join(argv)} (in {check.cwd})"
    if check.known_failure is None:
        assert run.returncode == 0, f"{label} exited {run.returncode}:\n{output[-3000:]}"
        if check.stdout_equals:
            assert run.stdout == (REFS / name / check.stdout_equals).read_bytes(), \
                f"{label}: stdout differs from {check.stdout_equals}"
        return
    assert run.returncode != 0, f"{label} now passes; remove its known failure ({check.known_failure})"
    missing = [text for text in check.must_show if text not in output]
    assert not missing, f"{label} fails differently from its record; missing {missing}:\n{output[-3000:]}"
    fails = tuple(line for line in output.splitlines() if line.startswith("FAIL"))
    assert fails == check.fail_lines, f"{label}: FAIL lines {fails}, recorded {check.fail_lines}"


@pytest.mark.parametrize("name", sorted(n for n, p in PACKAGES.items() if p.other_test))
def test_other_test_names_package(name):
    """Once the maintained test that regenerates a package lands, it must name that package."""
    other = ROOT / PACKAGES[name].other_test
    if not other.is_file():
        pytest.skip(f"{PACKAGES[name].other_test} is not on this branch yet")
    assert f"validation/references/{name}" in other.read_text(encoding="utf-8"), f"{other.name} does not name {name}"
