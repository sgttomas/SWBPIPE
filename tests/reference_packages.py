"""The one table of the reference packages under validation/references/, read by the two maintained tests
that check what every package holds:

- tests/test_reference_packages.py, CONSISTENCY (pure reading): every directory has an entry; verified
  checksum lists are checked against the bytes they name, with declared failing-line exceptions;
  historical lists are themselves pinned, and pin lists check their declared committed-file targets;
  the lists present are the lists declared; and MEMBERSHIP: every file of a package is named by a line
  of a verified list or is unlisted for a stated reason, on a basis the package states (see Membership);
- tests/test_reference_packages_regeneration.py, REGENERATION: each retained check command documented
  by its package (in its README or module docstring), with an assertion that no byte under
  validation/references/ changed. Deferred commands are not run.

These tests do not check that a package's generators are independent of product code.

An exception carries its reason. Checksum-line exceptions and recorded defect markers are checked for
continued presence. Deferred commands are not run and are removed only when their complete commands pass.
One check's stdout comparison (PLATFORM_LIMITED_STDOUT) is made only on the platform where its frozen file is
observed to reproduce, selected by `sys.platform` alone; elsewhere the command still runs and must exit 0 within
its limit, reproduction of the committed file is not established there, and what it printed is reported
against the frozen file, case by case, as a warning. What the tests take on trust is named and counted: output entries not
regenerated, regeneration gaps, deferred commands and platform-limited stdout comparisons (`taken_on_trust()`),
and memberships with no stated basis (`membership_on_trust()`).

The reading and running are functions here, and the tests call them: kept in this module, which builds no
path from a value the CI selector cannot read, they let the consistency and regeneration tests be selected
by the packages and the product inputs this table names, and not by every change.
"""
from __future__ import annotations

import hashlib
import re
import subprocess
import sys
import warnings
from dataclasses import dataclass, field
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
REFS = ROOT / "validation" / "references"
# The package walks below are scans the CI selector cannot fully read.
CI_READS = ("validation/references/**",)

SUMS_LINE = re.compile(r"^([0-9a-f]{64}) [ *](.+)$")
CHECKSUM_LIST = re.compile(r"(^|_)SHA256SUMS(\..+)?$")
NO_BASIS = "no basis: "  # opens the reason of a membership nothing in the package states
TMP = "<new scratch directory>"  # replaced by a fresh, absent directory under pytest's tmp_path
# Each check command's time limit: LIMIT_MARGIN times the slowest time observed for any retained command,
# rounded up to 10 s, one value for all. Hosted per-command times exist only for a shard's twenty slowest tests
# (pytest --durations=20). The completed regeneration checks reported in both lists slowed by factors of
# 1.67 (m3_judge) and 2.18 (pf_timoshenko), so one limit grounded on the slowest observed command keeps the
# observed margin for every command. A command that
# hangs still fails its test at the limit, inside the hosted job's allowance.
LIMIT_MARGIN = 5
SLOWEST_OBSERVED = (214.90, "t4_i9 check 0 on the hosted Linux runner: pytest's call duration, run 43513992525, "
                            "Python product contracts 2/4; the slowest retained command observed")
CHECK_TIMEOUT_S = 1080


class StdoutNotCompared(UserWarning):
    """A check's stdout comparison is not made on this platform: what the command printed, against its frozen
    file, so the hosted log shows what differs on a passing test."""


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
    """A documented check command, run from `cwd` (package-relative)."""
    argv: tuple
    cwd: str = "."
    stdout_equals: str | None = None  # package-relative file the stdout must equal byte for byte
    stdout_platforms: tuple = ()  # sys.platform values on which the stdout comparison is made; () means every one
    stdout_gap: str | None = None  # what was observed where the comparison is not made, and its repair


@dataclass(frozen=True)
class Deferred:
    """A check command from the package's README that is not run, beside the package's retained checks: its
    last observed blocker and the full SHA of the head it was observed at. What the command checks after that
    point is not established. The entry is removed only when the complete command passes."""
    argv: tuple
    blocker: str
    observed_at: str
    cwd: str = "."


NOT_ESTABLISHED = "what the command checks after that point is not established"

# Five packages whose committed outputs were generated with --pressure-elongation all (pull request 217, eff19a897);
# their READMEs give the check command with the switch.
PE_ALL = ("--pressure-elongation", "all")
# The one stdout comparison made only on named platforms: (package, command, frozen file). The exception is
# selected by the platform alone (sys.platform), never by a comparison's result: where the comparison is made, a
# difference fails as before. The platform says where the comparison is enforced; it is not evidence of what
# causes the difference. Execution, the time limit, the checksum lines, membership and every other comparison
# stay enforced everywhere. A second entry here is a deliberate change.
PLATFORM_LIMITED_STDOUT = (("t4_i9", "t4_i17_arc_refs.py", "arc_load_references.txt"),)
NOT_ESTABLISHED_HERE = ("the command ran and exited 0 within its limit; reproduction of the committed file is not "
                        "established on this platform")
T4_I9_ARC_GAP = (
    "observed: on the development host (macOS, sys.platform darwin) the command prints arc_load_references.txt byte "
    "for byte; on the hosted Linux runner (run 43513992525, Python product contracts 2/4, at aaf769403) it completed "
    "in 214.90 s and printed something else. Elsewhere than darwin, the command must run and exit 0 within its "
    "limit; reproduction of the committed file is not established there. The cause is not established. The per-case "
    "report shows which printed args and refs differ. Proposed repair, a separate change "
    "SWBPIPE's Agent 0 commissions: the generator's inputs pinned as input-only bits that reproduce the committed "
    "inputs, its independent calculation untouched. The entry is removed only when the complete output equals the "
    "committed file on both macOS and the hosted Linux runner")


@dataclass(frozen=True)
class Membership:
    """Which files of the package its checksum lists leave out, and on what basis.

    `basis` says where the package states the file set its lists cover: the generator's own file set
    where a generator writes the list (its regeneration compares the list byte for byte), the README's
    file table or statement where the README gives one, or a fixed set stated here with its reason. A
    package where nothing states it has a basis opening with NO_BASIS. `unlisted` maps each file under
    the package that no verified list names (a path, or a directory ending in "/" for everything under
    it) to its reason; a reason opening with NO_BASIS marks a file nothing in the package accounts for.
    `stated` maps an unlisted file to the SHA-256 (or the prefix of it) that the package states for it
    elsewhere, which the consistency test verifies.
    """
    basis: str
    unlisted: dict = field(default_factory=dict)
    stated: dict = field(default_factory=dict)


@dataclass(frozen=True)
class Package:
    deliverables: str
    members: Membership
    sums: tuple = ()
    no_sums: bool = False
    checks: tuple = ()
    not_regenerated: tuple = ()  # ((what, reason), ...): outputs of a package with checks that no check reproduces
    deferred: tuple = ()  # Deferred commands: not run, beside the retained checks
    regeneration_gap: str | None = None  # reason, required exactly when the package has no check and no deferred command
    other_test: str | None = None  # a maintained test, not yet on main, that will regenerate this package
    defects: tuple = ()  # ((file, text, reason), ...): a known defect, asserted present until repaired


@dataclass(frozen=True)
class Container:
    """A top-level directory that holds several packages rather than being one."""
    reason: str
    loose: dict = field(default_factory=dict)  # {container-relative file in no package: reason}


CONTAINERS = {
    "f1_modal": Container(
        "two packages (continuum, mass_discrete), each with its own README, generators and SHA256SUMS; "
        "published/ holds a README only",
        loose={"published/README.md": "the README of where the F1 modal references are consumed; it belongs to "
                                      "neither package and no list names it"}),
}

DYNAMICS_TEST = "tests/test_dynamics_references.py"
REGENERATION_GAP = "no check command (regeneration gap)"
F1_TRUST = "taken on trust until 268's test lands"

def _generated(where: str, readme: str | None = None) -> Membership:
    """A list the generator writes over its whole file set (README, itself and its outputs), which its own
    check rebuilds byte for byte, so every file of the package is listed."""
    basis = f"generator: {where} writes SHA256SUMS over its file set, the README, itself and its outputs"
    return Membership(basis + (f"; README: {readme}" if readme else ""))


def _plain(gen: str, *args: str, deliverables: str, members: Membership | None = None, **kw) -> Package:
    """One generator whose default (or named) mode rebuilds in memory and compares byte for byte."""
    return Package(deliverables, members or _generated(f"generators/{gen}"), sums=(Sums("SHA256SUMS"),),
                   checks=(Check(("generators/" + gen, *args)),), **kw)


PACKAGES: dict[str, Package] = {
    "combination_certificate": _plain("certificate_reference.py", deliverables="DEL-05-02"),
    "op1b_difference_basis": _plain(
        "op1b_reference.py", deliverables="DEL-05-02",
        members=Membership("generator: generators/op1b_reference.py build() writes SHA256SUMS over itself and "
                           "references.json only",
                           unlisted={"README.md": "outside the generator's file set"})),
    "op_mf_held_status": _plain(
        "held_status_reference.py", deliverables="DEL-05-02",
        members=Membership("README file table: `SHA256SUMS` | Hashes of the generator and `references.json` (the "
                           "generator rebuilds references.json only)",
                           unlisted={"README.md": "outside the README's stated set"})),
    "reference_system": _plain(
        "rs_reference.py", deliverables="DEL-09-01"),
    "rt_a3_s6": _plain("rt_a3_s6_reference.py", *PE_ALL, deliverables="DEL-04-07"),
    "rt_a3_constant_effort": _plain("rt_a3_constant_effort_reference.py", "--check", deliverables="DEL-04-07"),
    "rt_a3_nonlinear": _plain("rt_a3_nonlinear_reference.py", "--check", deliverables="DEL-04-07"),
    "rt_a3_reducer": _plain("rt_a3_reducer_reference.py", "--check", deliverables="DEL-03-05, DEL-04-07"),
    "rt_a3_reducer_2b": _plain("rt_a3_2b_reference.py", "--check", deliverables="DEL-04-07"),
    "rt_a3_tee": _plain("rt_a3_tee_reference.py", "--check", deliverables="DEL-03-04, DEL-04-07"),
    "rt_a4_combinations": Package(
        "DEL-05-02", _generated("generators/rt_a4_reference.py (top level) and v5/generators/rt_a4_v5_reference.py (v5/, "
                                "its own list)"),
        sums=(Sums("SHA256SUMS"), Sums("v5/SHA256SUMS", base="v5")),
        checks=(Check(("generators/rt_a4_reference.py",)), Check(("generators/rt_a4_v5_reference.py",), cwd="v5"))),
    "rt_g5_s2_conditions": _plain(
        "rt_g5_s2_reference.py", deliverables="DEL-05-01",
        ),
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
    "t4_u5a": _plain("u5a_reference.py", *PE_ALL, deliverables="DEL-03-06, DEL-04-07",
                     members=Membership("README file table: `SHA256SUMS` | the hashes of every file above (the "
                                        "generator does not write the list)")),
    "t4_u5b": _plain("u5b_reference.py", deliverables="DEL-04-07"),
    "t4_u6": _plain("u6_reference.py", *PE_ALL, deliverables="DEL-05-03",
                    members=Membership("README file table: `SHA256SUMS` | the hashes of every file above (the "
                                       "generator does not write the list)")),
    "t4_u8": _plain(
        "u8_reference.py", *PE_ALL, deliverables="DEL-05-03"),
    "pf_timoshenko": _plain(
        "pf_reference.py", deliverables="DEL-05-03"),
    "t4_u4a": Package("DEL-04-07", Membership("README file table: `SHA256SUMS` | Digests of the five files above "
                                              "(u4a_generate.py does not write the list)"),
                      sums=(Sums("SHA256SUMS"),),
                      checks=(Check(("u4a_generate.py", *PE_ALL)),)),
    "hanger_sizing": Package(
        "DEL-04-11",
        Membership("README (Checksum list): `SHA256SUMS` lists every file of this package except itself, regenerated "
                   "by hand with the shasum command it gives"),
        sums=(Sums("SHA256SUMS"),),
        checks=tuple(Check((gen, "--check")) for gen in ("generate_r3.py", "generate_r125.py", "generate_ex_sizing.py",
                                                         "generate_preload.py"))),
    "kd5_member_bound": Package(
        "DEL-04-01",
        Membership("README: it names each file, the two generators with their requests and oracles; the package has "
                   "no checksum list, and each oracle binds its request's bytes by SHA-256 (README)",
                   unlisted={"README.md": "the README",
                             "notice_reference.py": "the notice generator (README)",
                             "request.json": "its request (README), bound by SHA-256 in oracle.json",
                             "oracle.json": "the frozen oracle notice_reference.py verifies (README)",
                             "two_body_reference.py": "the negative control's generator (README)",
                             "two_body_request.json": "the negative control's request (README), bound by SHA-256 in "
                                                      "two_body_oracle.json",
                             "two_body_oracle.json": "the oracle two_body_reference.py verifies (README)"}),
        no_sums=True,
        checks=(Check(("notice_reference.py",)), Check(("two_body_reference.py",)))),
    "kd5_nonlinear": Package(
        "DEL-04-01, DEL-09-03",
        Membership("generator: reference.py's request list (lines 31-39) names the seven requests, and each oracle is "
                   "its request's name with _oracle (line 405); the package has no checksum list or README",
                   unlisted={"reference.py": "the generator",
                             **{f"{c}_{kind}.json": f"a {kind} the generator reads or verifies"
                                for c in ("f1", "f1_explicit", "f1_mu0", "f2", "u1", "u1_spring", "u1_rx")
                                for kind in ("request", "oracle")}}),
        no_sums=True,
        # No README; the module docstring gives the command: "Run it to verify the maintained oracles".
        checks=(Check(("reference.py",)),)),
    "t4_i7": Package(
        "DEL-04-07", Membership("README file table: four files, and `SHA256SUMS` holds these four digests",
                                unlisted={"README.md": "outside the README's file table, which lists what SHA256SUMS holds"}),
        sums=(Sums("SHA256SUMS"),),         checks=(Check(("_run_records/u2_generate.py", "--check")),)),  # regenerates in a temporary directory
    "t4_i8": Package(
        "DEL-04-07", Membership("README file table: three files, and `SHA256SUMS` holds these three digests",
                                unlisted={"README.md": "outside the README's file table, which lists what SHA256SUMS holds"}),
        sums=(Sums("SHA256SUMS"),),
        checks=(Check(("_run_records/check_reference_json.py", "rebuilt_reference_cases.json")),),
        not_regenerated=(("rebuilt_reference_cases.json",
                          "the README gives no check mode: derive_rebuilt_references.py only writes to a given path "
                          "and needs the product fixture path; check_reference_json.py re-evaluates the frozen file"),)),
    "t4_i9": Package(
        "DEL-04-01",
        Membership("README: `SHA256SUMS` lists every file here",
                   unlisted={"README.md": NO_BASIS + "the README says SHA256SUMS lists every file here, and the list "
                                                     "does not name README.md"}),
        sums=(Sums("SHA256SUMS"),),
        checks=(Check(("t4_i17_arc_refs.py", "_run_records/rv129_ref.py"), stdout_equals="arc_load_references.txt",
                      stdout_platforms=("darwin",), stdout_gap=T4_I9_ARC_GAP),
                Check(("t4_i17_bit_pins.py", "_run_records/arc_cert_probe_r01.py"), stdout_equals="arc_certificate_pins.txt"),
                Check(("check_series.py",))),),  # load_series_gen.py's output is pasted into the product, not frozen here
    "t4_i6": Package(
        "DEL-03-03, DEL-04-01",
        Membership("README: `SHA256SUMS` holds the archive's three lines (u1_reference_cases.json, "
                   "_run_records/curved_ref.py, _run_records/freeze_u1.py) and `round_01/SHA256SUMS` round 01's; the "
                   "README describes each later addition and states its SHA-256 there",
                   unlisted={"README.md": "the README; no list names it",
                             "t4_i14_l30_matrix.py": "added by T4-I14; its SHA-256 is stated in the README",
                             "t4_i14_l30_matrix.stdout.txt": "its output; its SHA-256 is stated in the README",
                             "t4_i15_short_arc_matrices.py": "added by T4-I15; its SHA-256 is stated in the README",
                             "t4_i15_short_arc_matrices.stdout.txt": "its output; its SHA-256 is stated in the README",
                             "kd5_models/kd5_models.py": "added by T4-I15 (README, kd5_models/); the README states its "
                                                         "SHA-256 by its first eight digits only",
                             "kd5_models/kd5_models.stdout.txt": "kd5_models.py's comparison record (README, kd5_models/); "
                                                                 "the README states no digest for it",
                             "kd5_models/kd5_models_comparison.json": "kd5_models.py's comparison record (README, "
                                                                      "kd5_models/); the README states no digest for it"},
                   stated={'t4_i14_l30_matrix.py': 'bee26d7d8bed2fa19464bf61a8dd292517ced462dd3e70e54bb07388449af11c', 't4_i14_l30_matrix.stdout.txt': '0f2828beaefb5709b11fad5e4f3d8696e2b063316baed786f8fa4cb4cdfce10c', 't4_i15_short_arc_matrices.py': '52b811325257b76d4e97da19550d5ea9432226893f0af215716c948ae2afa0a3', 't4_i15_short_arc_matrices.stdout.txt': '85ff2e972791c365cc201359752fefec7ee27dd0935ac26239cb920424412116', 'kd5_models/kd5_models.py': '92097bca'}),
        sums=(Sums("SHA256SUMS"), Sums("round_01/SHA256SUMS", base="round_01")),
        checks=(Check(("t4_i14_l30_matrix.py", "_run_records"), stdout_equals="t4_i14_l30_matrix.stdout.txt"),
                Check(("t4_i15_short_arc_matrices.py", "_run_records"), stdout_equals="t4_i15_short_arc_matrices.stdout.txt")),
        not_regenerated=(("u1_reference_cases.json", "the README gives no check mode: freeze_u1.py wrote it in the archived run (RUN.txt)"),
                         ("round_01/", "the README gives no check mode; its scripts were run in scratch in repair round 01"),
                         ("kd5_models/", "needs D1's and T3's V1 modules from outside the repository (README)"))),
    "t4_i12": Package(
        "DEL-03-06", Membership("README file table: `SHA256SUMS` lists the three files",
                                unlisted={"README.md": "outside the README's file table, which lists what SHA256SUMS holds"}),
        sums=(Sums("SHA256SUMS"),),
        regeneration_gap="u3_reference_cases.json: the README gives no check mode: both commands write to a given "
                         "output path"),
    "t4_o13": Package(
        "DEL-04-01", Membership("README file table: o13_candidates.json, o13_fk_references.py and "
                                "o13_fk_references.txt, which SHA256SUMS holds",
                                unlisted={"README.md": "outside the README's file table, which lists what SHA256SUMS holds"}),
        sums=(Sums("SHA256SUMS"),),
        regeneration_gap="o13_fk_references.txt: the README gives no check mode: o13_fk_references.py writes the "
                         "file into its working directory"),
    "t3_r1": Package(
        "DEL-09-01",
        Membership("README (REFERENCES/README.md): 'Files. The hashes are in _run_records/SHA256SUMS', then every file "
                   "under REFERENCES/",
                   unlisted={"floor_kinds.json": NO_BASIS + "D1's enumerated not-covered list, beside REFERENCES/; "
                                                            "generators outside the package read it (gen_k4_vectors.py, "
                                                            "gen_vk_cases.py), and no README or list of t3_r1 names it"}),
        sums=(Sums("REFERENCES/_run_records/SHA256SUMS", base="REFERENCES"),
              Sums("REFERENCES/_run_records/SHA256SUMS.revision1", base="REFERENCES",
                   historical="the revision-1 hashes, kept on purpose (README); superseded by SHA256SUMS")),
        checks=(Check(("_run_records/selfcheck_stiffness.py",), cwd="REFERENCES"),),
        not_regenerated=(("REFERENCES/references.json",
                          "references.py took 679 s and 722 s (README), and its regeneration mode writes "
                          "references.json; the author self-check runs instead"),)),
    "kd5_timoshenko": Package(
        "DEL-05-03",
        Membership("README file table: `SHA256SUMS` | Digests of the files above, except this README",
                   unlisted={"README.md": "excepted by the README"}),
        sums=(Sums("SHA256SUMS"),),
        checks=(Check(("kd5t_engine.py", "--check")), Check(("kd5t_generate.py", "check")))),
    "pf_timoshenko_m3t": Package(
        "DEL-05-03",
        Membership("README file table: `SHA256SUMS` | Digests of the files above, except this README",
                   unlisted={"README.md": "excepted by the README",
                             "census-probe.patch": "test-only product instrumentation kept as a reproducibility "
                                                   "artifact, 'Not in SHA256SUMS' (README, Census probe), which states "
                                                   "its SHA-256 instead"},
                   stated={"census-probe.patch": "a8b2b9de4da1b3f82d28203d493ae7406c27233d416b0166bc2f9292cf858fd5"}),
        sums=(Sums("SHA256SUMS"),),
        checks=(Check(("pft_engine.py", "--check")), Check(("pft_generate.py", "check")))),
    "m3_judge": Package(
        "DEL-04-01",
        Membership("README file table: `SHA256SUMS` | Digests of the files above, except this README",
                   unlisted={"README.md": "excepted by the README"}),
        sums=(Sums("SHA256SUMS"),
              Sums("REFERENCE_SHA256SUMS", base="..", pins=3)),
        checks=(Check(("m3_reproduce.py", TMP)), Check(("m3_selftest.py",)))),
    "o4_retained_source": Package(
        "DEL-04-08",
        Membership("generator: generators/o4_reference.py writes SHA256SUMS over SUMMED (this README and the seven "
                   "files of the README's table) and its check refuses any other file outside BESIDE",
                   unlisted={"declared_problem/": "a product check beside the unit, 'outside this inventory and outside "
                                                  "SHA256SUMS' (generators/o4_reference.py, BESIDE)"}),
        sums=(Sums("SHA256SUMS"),),
        checks=(Check(("generators/o4_reference.py", "--workers", "1")),
                Check(("generators/o4_source_twins.py", "--workers", "1")),
                Check(("oracle.py", "--selftest"), cwd="declared_problem"))),  # the product check's own oracle, pure Python
    "f1_modal/continuum": Package(
        "DEL-04-09", _generated("generators/generate.py (AUTHORED, GENERATORS and the three JSON files)"),
        sums=(Sums("SHA256SUMS"),),
        other_test=DYNAMICS_TEST,
        # From the package directory: python3 -I -B generators/generate.py (about 11 s).
        regeneration_gap=F1_TRUST),
    "f1_modal/mass_discrete": Package(
        "DEL-04-09", _generated("generators/f1_mass_discrete.py (FIXED_FILES and both JSON files)"),
        sums=(Sums("SHA256SUMS"),),
        other_test=DYNAMICS_TEST,
        # From the package directory: python3 -I -B generators/f1_mass_discrete.py (about 5 s, several processes).
        regeneration_gap=F1_TRUST),

    # Exact-route references frozen after the first table; each also has its own maintained test.
    "exact_difference_basis": _plain("exact_difference_basis_reference.py", deliverables="DEL-05-02"),
    "exact_load_state_equivalent_static": _plain("exact_load_state_reference.py", deliverables="DEL-05-01"),
    "friction_one_way_normal": _plain("friction_one_way_normal_reference.py", deliverables="DEL-04-04"),
    "hydrostatic_test": _plain("hydrostatic_test_reference.py", deliverables="DEL-05-01"),
    "reference_system_pending_cases": _plain(
        "rs_pending_reference.py", deliverables="DEL-09-01"),
    "seismic_component_weight": _plain("scw_reference.py", deliverables="DEL-05-01"),
    "snubber_static": _plain("snubber_static_reference.py", deliverables="DEL-04-10"),
}

# Outputs not regenerated, regeneration gaps, deferred commands and stdout comparisons made only on named
# platforms, each a row of taken_on_trust().
TAKEN_ON_TRUST_COUNT = 10
MEMBERSHIP_ON_TRUST_COUNT = 2


def taken_on_trust() -> list[tuple[str, str, str]]:
    """Everything the regeneration test does not reproduce: (package, what, reason)."""
    out = [(name, what, why) for name, pkg in PACKAGES.items() for what, why in pkg.not_regenerated]
    out += [(name, REGENERATION_GAP, pkg.regeneration_gap) for name, pkg in PACKAGES.items() if pkg.regeneration_gap]
    out += [(name, deferred_text(d), "deferred, not run") for name, pkg in PACKAGES.items() for d in pkg.deferred]
    out += [(name, f"reproduction of {c.stdout_equals} by {' '.join(c.argv)}: established only on "
                   f"{', '.join(c.stdout_platforms)}", c.stdout_gap)
            for name, pkg in PACKAGES.items() for c in pkg.checks if c.stdout_platforms]
    return out


def deferred_text(d: Deferred) -> str:
    """How a deferred command is printed."""
    return (f"deferred command {' '.join(d.argv)} (in {d.cwd}): last observed blocker: {d.blocker}; observed at "
            f"{d.observed_at}; {NOT_ESTABLISHED}")


def membership_on_trust() -> list[tuple[str, str, str]]:
    """Every package or file whose membership nothing in the package states: (package, what, reason)."""
    out = [(name, "the package", pkg.members.basis) for name, pkg in PACKAGES.items()
           if pkg.members.basis.startswith(NO_BASIS)]
    out += [(name, path, why) for name, pkg in PACKAGES.items() for path, why in pkg.members.unlisted.items()
            if why.startswith(NO_BASIS)]
    return out


# ---------------------------------------------------------------------------------------------- helpers

def is_package_dir(path: Path) -> bool:
    return any(p.suffix == ".py" or CHECKSUM_LIST.search(p.name) for p in path.iterdir() if p.is_file())


def files(pkg_dir: Path, pattern: str = "*") -> list[Path]:
    return sorted(p for p in pkg_dir.rglob(pattern) if p.is_file() and "__pycache__" not in p.parts)


def sums_lines(path: Path) -> list[tuple[str, str]]:
    lines = []
    for n, raw in enumerate(path.read_text(encoding="utf-8").splitlines(), 1):
        if not raw.strip():
            continue
        m = SUMS_LINE.match(raw)
        assert m, f"{path.relative_to(ROOT)}:{n} is not a checksum line: {raw!r}"
        lines.append((m.group(1), m.group(2)))
    return lines


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def verified_lists(name: str) -> list[Sums]:
    """The lists whose lines are verified against bytes (not historical, not a pin list)."""
    return [s for s in PACKAGES[name].sums if not s.historical and s.pins is None]


def listed(name: str) -> set[str]:
    """Package-relative paths that a line of a verified list names."""
    pkg_dir = (REFS / name).resolve()
    out = set()
    for sums in verified_lists(name):
        base = (REFS / name / sums.base).resolve()
        for _, rel in sums_lines(REFS / name / sums.path):
            target = (base / rel).resolve()
            if target.is_relative_to(pkg_dir):
                out.add(target.relative_to(pkg_dir).as_posix())
    return out


def tree_state() -> dict[str, tuple[str, int, int]]:
    """{path under validation/references: (sha256, size, mtime_ns)} for every file but bytecode caches,
    which no check writes (each runs with -B) and which a test elsewhere that imports a module from a
    package may: a write, an addition or a removal anywhere else changes this."""
    out = {}
    for p in sorted(REFS.rglob("*")):
        if p.is_file() and "__pycache__" not in p.parts:
            st = p.stat()
            out[p.relative_to(REFS).as_posix()] = (sha256(p), st.st_size, st.st_mtime_ns)
    return out


# ---------------------------------------------------------------------------------------------- checks
# Each returns the problems it finds, as text; the tests assert there are none.

def classification_problems() -> list[str]:
    """Every package directory, or package inside a container, has an entry; every entry has its package;
    every file of a container in no package is named with its reason; nothing lies loose at the top."""
    problems = []
    for top in sorted(p for p in REFS.iterdir() if p.is_dir()):
        name = top.name
        if name in CONTAINERS:
            for sub in sorted(top.rglob("*")):
                if sub.is_dir() and "__pycache__" not in sub.parts and is_package_dir(sub):
                    rel = "/".join(sub.parts[len(REFS.parts):])
                    if not any(rel == k or rel.startswith(k + "/") for k in PACKAGES):
                        problems.append(f"{rel} holds a generator or a checksum list and no entry covers it")
            loose = {p.relative_to(top).as_posix() for p in files(top)
                     if not any(p.relative_to(REFS).as_posix().startswith(k + "/") for k in PACKAGES)}
            declared = set(CONTAINERS[name].loose)
            if loose != declared:
                problems.append(f"{name}: files in no package {sorted(loose)}, the container names {sorted(declared)}")
        elif name not in PACKAGES:
            problems.append(f"{name}: no entry in PACKAGES; classify it (its checksum lists, membership, generators, "
                            f"check command and declaring deliverable) in tests/reference_packages.py")
    problems += [f"{p.name}: a file directly under validation/references/" for p in REFS.iterdir() if p.is_file()]
    for name, pkg in PACKAGES.items():
        if not pkg.checks and not pkg.deferred and not (pkg.regeneration_gap or "").strip():
            problems.append(f"{name}: no check command, no deferred command and no regeneration gap; its regeneration "
                            f"would vanish uncounted")
        elif (pkg.checks or pkg.deferred) and pkg.regeneration_gap:
            problems.append(f"{name}: a regeneration gap beside check or deferred commands")
        for c in pkg.checks:
            if c.stdout_platforms and not (c.stdout_equals and (c.stdout_gap or "").strip()):
                problems.append(f"{name}: {' '.join(c.argv)} compares its stdout only on named platforms, without a "
                                "frozen file or without saying what was observed elsewhere")
            if c.stdout_platforms and (name, c.argv[0], c.stdout_equals) not in PLATFORM_LIMITED_STDOUT:
                problems.append(f"{name}: {' '.join(c.argv)} compares its stdout only on named platforms but is not "
                                "the exception PLATFORM_LIMITED_STDOUT names")
        for d in pkg.deferred:
            if not d.blocker.strip() or not re.fullmatch(r"[0-9a-f]{40}", d.observed_at):
                problems.append(f"{name}: the deferred command {' '.join(d.argv)} names no blocker or no full SHA "
                                f"it was observed at")
            if any(c.argv == d.argv and c.cwd == d.cwd for c in pkg.checks):
                problems.append(f"{name}: {' '.join(d.argv)} is both a check and deferred")
    for name in PACKAGES:
        if not (REFS / name).is_dir():
            problems.append(f"{name}: an entry with no package")
        elif "/" in name and name.split("/", 1)[0] not in CONTAINERS:
            problems.append(f"{name}: a nested entry whose top-level directory is not a container")
    return problems


def checksum_problems(name: str) -> list[str]:
    """The lists present are the lists declared, and every line of every verified list matches its bytes
    (a line known to fail is an exception that fails once the line passes again)."""
    pkg, pkg_dir = PACKAGES[name], REFS / name
    present = {p.relative_to(pkg_dir).as_posix() for p in files(pkg_dir) if CHECKSUM_LIST.search(p.name)}
    declared = {s.path for s in pkg.sums}
    if present != declared:
        return [f"checksum lists present {sorted(present)}, declared {sorted(declared)}"]
    if pkg.no_sums != (not declared):
        return ["no_sums must be set exactly when the package has no checksum list"]
    problems = []
    for sums in pkg.sums:
        path, base = pkg_dir / sums.path, (pkg_dir / sums.base).resolve()
        if sums.historical:
            covered = any(((pkg_dir / o.base) / n).resolve() == path.resolve()
                          for o in verified_lists(name) for _, n in sums_lines(pkg_dir / o.path))
            if not covered:
                problems.append(f"historical list {sums.path} is not itself pinned by a verified list")
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
                    if sha256(target) != digest:
                        problems.append(f"{sums.path}: {what.split()[0]} differs from its pin")
            if verified != sums.pins:
                problems.append(f"{sums.path} names {verified} committed files, the entry says {sums.pins}")
            continue
        failing = set()
        for digest, rel in sums_lines(path):
            target = base / rel
            if not target.is_file() or sha256(target) != digest:
                failing.add(rel)
        problems += [f"{sums.path}: {rel} fails (base {sums.base})" for rel in sorted(failing - set(sums.failing))]
        problems += [f"{sums.path}: {rel} passes again; remove its exception"
                     for rel in sorted(set(sums.failing) - failing)]
    return problems


def _covered(rel: str, unlisted: dict) -> str | None:
    """The unlisted entry that covers a package-relative file: the path itself, or a directory above it."""
    return next((key for key in unlisted if rel == key or (key.endswith("/") and rel.startswith(key))), None)


def membership_problems(name: str) -> list[str]:
    """Every file under the package is named by a verified list, is a checksum list, or is unlisted for a
    stated reason; every unlisted entry still matches a file; every stated digest matches its file."""
    pkg, pkg_dir = PACKAGES[name], REFS / name
    members = pkg.members
    if not members.basis.strip():
        return ["no membership basis"]
    lists = {s.path for s in pkg.sums}
    in_lists = listed(name)
    problems, used = [], set()
    for rel in (p.relative_to(pkg_dir).as_posix() for p in files(pkg_dir)):
        key = _covered(rel, members.unlisted)
        if key is not None:
            used.add(key)
            if rel in in_lists:
                problems.append(f"{rel} is listed and also declared unlisted ({key})")
        elif rel not in in_lists and rel not in lists:
            problems.append(f"{rel}: named by no verified list and by no unlisted entry (basis: {members.basis})")
    for key, why in members.unlisted.items():
        if not why.strip():
            problems.append(f"unlisted {key} has no reason")
        if key not in used:
            problems.append(f"unlisted entry {key} matches no file; remove it")
    for rel, digest in members.stated.items():
        if rel not in members.unlisted:
            problems.append(f"a stated digest for {rel}, which is not an unlisted entry")
        elif not (pkg_dir / rel).is_file() or not sha256(pkg_dir / rel).startswith(digest):
            problems.append(f"{rel} differs from the SHA-256 the package states for it ({digest[:16]}...)")
    return problems


def defect_problems(name: str) -> list[str]:
    """A known defect is an entry that fails the moment it is repaired, so the entry goes with it."""
    return [f"{file} no longer holds {text!r}; the defect looks repaired, so remove its entry"
            for file, text, reason in PACKAGES[name].defects
            if not reason.strip() or text not in (REFS / name / file).read_text(encoding="utf-8")]


def _changes(before: dict, after: dict) -> list[str]:
    out = [f"added {p}" for p in sorted(set(after) - set(before))]
    out += [f"removed {p}" for p in sorted(set(before) - set(after))]
    out += [f"changed {p}" for p in sorted(set(before) & set(after)) if before[p] != after[p]]
    return out


def check_problems(name: str, index: int, scratch: Path) -> list[str]:
    """Runs the package's check `index` as `python3 -I -B` from its directory, with TMP replaced by
    `scratch` (a fresh directory outside the repository). It must leave every file under
    validation/references/ as it was, and exit 0 (printing a frozen file where the entry says so); any other
    exit status is a failure."""
    check = PACKAGES[name].checks[index]
    argv = [str(scratch) if a == TMP else a for a in check.argv]
    before = tree_state()
    label = f"python3 -I -B {' '.join(argv)} (in {check.cwd})"
    limit = CHECK_TIMEOUT_S
    try:
        run = subprocess.run([sys.executable, "-I", "-B", *argv], cwd=REFS / name / check.cwd,
                             capture_output=True, timeout=limit)
    except subprocess.TimeoutExpired:
        return [f"{label} did not finish within its {limit} s limit (reference_packages.CHECK_TIMEOUT_S)"]
    written = _changes(before, tree_state())
    output = (run.stdout + run.stderr).decode("utf-8", "replace")
    if written:
        return [f"{label} wrote under validation/references/:"] + written
    if run.returncode != 0:
        return [f"{label} exited {run.returncode}:\n{output[-3000:]}"]
    if check.stdout_equals:
        frozen = (REFS / name / check.stdout_equals).read_bytes()
        if not check.stdout_platforms or sys.platform in check.stdout_platforms:
            if run.stdout != frozen:
                return [f"{label}: stdout differs from {check.stdout_equals}\n" + stdout_report(frozen, run.stdout)]
        else:
            seen = ("its stdout equalled the committed file here" if run.stdout == frozen
                    else "its stdout differed from the committed file here:\n" + stdout_report(frozen, run.stdout))
            warnings.warn(StdoutNotCompared(
                f"{name}: {label}: on {sys.platform} {NOT_ESTABLISHED_HERE} ({check.stdout_equals}; the comparison "
                f"is enforced only on {', '.join(check.stdout_platforms)}); {seen}"), stacklevel=2)
    return []


def stdout_report(frozen: bytes, printed: bytes) -> str:
    """What differs between a frozen stdout file and what the command printed, without either file: how many
    lines differ, by kind (a line's first word), and the first differing line of each kind. Where the file is in
    blocks opened by a "case" line, each with an "args" line (its inputs) and "ref" lines (its references), it
    also reports every case: whether its args and its refs are identical to the committed ones, and the
    deciding count of cases whose args are identical and whose refs differ."""
    old = frozen.decode("utf-8", "replace").splitlines()
    new = printed.decode("utf-8", "replace").splitlines()
    kinds: dict[str, list] = {}
    for i in range(max(len(old), len(new))):
        a = old[i] if i < len(old) else None
        b = new[i] if i < len(new) else None
        if a != b:
            kind = ((a or b or "").split() or ["(blank)"])[0]
            kinds.setdefault(kind, []).append((i + 1, a, b))
    lines = [f"{sum(len(v) for v in kinds.values())} of {len(old)} frozen lines differ ({len(new)} printed): "
             + ", ".join(f"{k} {len(v)}" for k, v in sorted(kinds.items()))]
    for kind, rows in sorted(kinds.items()):
        line, a, b = rows[0]
        lines.append(f"  first {kind} at line {line}: frozen {a!r}; printed {b!r}")
    cases_old, cases_new = _cases(old), _cases(new)
    if cases_old or cases_new:
        labels = list(cases_old) + [k for k in cases_new if k not in cases_old]
        rows, deciding = [], 0
        for label in labels:
            o, n = cases_old.get(label), cases_new.get(label)
            if o is None or n is None:
                rows.append(f"  case {label}: {'not printed' if n is None else 'not in the committed file'}")
                continue
            args_same, refs_differ = o["args"] == n["args"], sum(x != y for x, y in zip(o["ref"], n["ref"]))
            refs_differ += abs(len(o["ref"]) - len(n["ref"]))
            deciding += args_same and refs_differ > 0
            rows.append(f"  case {label}: args {'identical' if args_same else 'differ'}; refs "
                        + ("identical" if not refs_differ else f"differ ({refs_differ} of {len(o['ref'])})"))
        same_args = sum(1 for k in cases_old if k in cases_new and cases_old[k]["args"] == cases_new[k]["args"])
        lines.append(f"cases: {len(cases_old)} committed, {len(cases_new)} printed; args identical in {same_args}; "
                     f"cases with identical args and differing refs: {deciding}")
        lines += rows
    return "\n".join(lines)


def _cases(lines: list[str]) -> dict[str, dict]:
    """{case label: {"args": its args line, "ref": its ref lines}} for a file in "case" blocks."""
    out: dict[str, dict] = {}
    current = None
    for line in lines:
        word = (line.split() or [""])[0]
        if word == "case":
            current = out.setdefault(line[len("case"):].strip(), {"args": None, "ref": []})
        elif current is not None and word == "args":
            current["args"] = line
        elif current is not None and word == "ref":
            current["ref"].append(line)
        elif word == "end":
            current = None
    return out
