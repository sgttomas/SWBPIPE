# A2-V-D-MEASUREMENT structured return

RUN_STATUS: `FAILED` — semantic execution `PASS`; terminal closeout blocked by
one preserved ignored Cargo side effect.

ControlSurface: `FILE`

TaskProfile: `NONE`

TaskSkill: `NONE`

ScopePath:
`/private/tmp/chirality-piping-del0904-owner-gates-20260810/projects/chirality-piping/validation/evidence/comparison_measurement/DEL0904_VD_20260811`

ToolsUsed:

- `git git-repository-read-operations`
- `cargo projects/chirality-piping/core/runner/headless/Cargo.toml`
- `python3 standard-library-analysis`
- `jq JSON-projection-and-validation`
- `shasum SHA-256-validation`
- `apply_patch authorized-artifact-writes`

ToolPolicyCompliance: `VIOLATION` — the exact Cargo capture command was
allowed, but Cargo created one ignored repository lockfile outside the write
whitelist. It is preserved and disclosed below.

WriteAuthorization: `ALLOWED_WRITE_TARGETS`

## Semantic result — PASS

- Exact current population: 25 requested; 11 executed-and-matched; 0
  mismatched; 14 fail-closed implementation/binding blocks.
- Exact value reconciliation: 206 recorded; 91 observed; 115 unobserved.
- Historic July-20 census remains 13 blocks / 109 unobserved values.
- Proved new fourteenth block:
  `MECH-TP-DEC092-TEMPERATURE-INDEXED-SHEAR-MODULUS-TORSION`, six values,
  implementation/binding classification.
- Per-kind exact-zero floors and nearest-nonzero context were measured without
  treating blocked rows as observations. `force_per_length`,
  `mass_per_length`, and `second_moment_area` remain unmeasured; seven kinds
  have no exact-zero observation.
- The sparse design separates translation (`m`), rotation (`rad`), force
  residual (`N`), and moment residual (`N-m`). Existing DEC-053 aggregate
  absolute rows cannot be backfilled into those kinds.
- Numeric options VD-N1 through VD-N4 are owner-ready and explicitly
  non-authoritative. Recommendation: `VD-N4` (retain numeric hold and complete
  missing evidence); `VD-N2` is the best bounded partial measured option.
- No public policy/value, runner repair, fixture/case edit, promotion,
  register/status/lifecycle/reliance/release, or Git action was performed.

## Exact capture commands and outcomes

Attempt 1:

```text
CARGO_NET_OFFLINE=true CARGO_TARGET_DIR=/private/tmp/del0904-vd-capture.NMgEzJ/target cargo run --offline --manifest-path projects/chirality-piping/core/runner/headless/Cargo.toml --bin openpipestress-runner -- run-benchmark --input projects/chirality-piping/validation/evidence/benchmarks/BENCHEVID_DEL0901_20260720T062342Z_e315fb8406d4/suite_run_mechanics_input.json --output /private/tmp/del0904-vd-capture.NMgEzJ/CURRENT_25_FIXTURE_CAPTURE.json
```

Exit `1`; the wrapper blocked because explicit local-private intent was
omitted; no output file was created. This first Cargo invocation created the
ignored lockfile described below.

Attempt 2:

```text
CARGO_NET_OFFLINE=true CARGO_TARGET_DIR=/private/tmp/del0904-vd-capture.NMgEzJ/target cargo run --offline --manifest-path projects/chirality-piping/core/runner/headless/Cargo.toml --bin openpipestress-runner -- run-benchmark --input projects/chirality-piping/validation/evidence/benchmarks/BENCHEVID_DEL0901_20260720T062342Z_e315fb8406d4/suite_run_mechanics_input.json --output /private/tmp/del0904-vd-capture.NMgEzJ/CURRENT_25_FIXTURE_CAPTURE.json --explicit-local-private-intent
```

Exit `1` as expected for 14 suite blocks; enclosing wrapper `blocked=false`;
exact output preserved. Cargo build/target output was directed outside the
repository.

## Outputs

All paths below are under
`projects/chirality-piping/validation/evidence/comparison_measurement/DEL0904_VD_20260811/`:

| artifact | SHA-256 |
| --- | --- |
| `CURRENT_25_FIXTURE_RUNNER_OUTPUT.json` | `e41f8545cf6fc0603cb41a0bdc08d90726f59ce57e13ad6809036589db3ded8f` |
| `CURRENT_25_FIXTURE_SUITE_RUN.json` | `d6ea66202eae4203c841aef589c6fe10f36f450ded79b0ad3e4093ce2375e108` |
| `CAPTURE_STDERR.txt` | `f30c75d45854b0062c70e9036f3d955850d75d3bc8897232e7513de45c6980f9` |
| `MEASUREMENT_ANALYSIS.json` | `fcd440de394dc65187faf78fc66b7a585ec904a218dcbec9671a0eb3423e5bba` |
| `SPARSE_UNIT_NORMALIZED_DESIGN.md` | `f136116efdb7aba4794add0476801352eced4ad73979f6b1a4db78b1c0c23e72` |
| `OWNER_VALUE_PACKET.md` | `4002d68a9a5582956fd969e59f93c60be2af7b9388bd735eefb56ceca6a06ac5` |
| `COMMAND_ENVIRONMENT_MANIFEST.json` | `d98182b1644ff5b396fa190474dd787b2bed656ad1064a0e5749f0bf288eb049` |
| `MANIFEST.json` | `a08a738634155b01be83a04f2777bfcbbb131246b934ebda24eaf9ad4860f385` |
| `SHA256SUMS.txt` | `2253707ec212069b77a4cd6416a08d028efb98c45887623b5ec3a0617d90daae` |

## Terminal blocker — exact cleanup gate required

- Path: `projects/chirality-piping/core/runner/headless/Cargo.lock`
- Provenance: absent at application start; created by Attempt 1 above at
  `2026-08-11T09:42:06Z` local filesystem time; no workaround or unrelated
  process is claimed.
- Type: regular file, not a symlink.
- Size: 10,114 bytes.
- SHA-256:
  `7a3bd7e0df41a07e5c503aa312734e95fa6625afcd8b12f1f7994bd7a75b2e66`.
- Ignore basis: `projects/chirality-piping/core/runner/headless/.gitignore:2`.
- Current status: `!! projects/chirality-piping/core/runner/headless/Cargo.lock`.
- Action taken after detection: preserved untouched; no deletion, cleanup,
  stage, commit, reset, or other Git state change.

Terminal zero-ignored-drift cannot pass until the owner separately authorizes
deleting this exact file after identity/type backcheck.

MISSING:

- terminal zero-ignored-drift cleanup;
- observations for three analytic kinds and exact-zero coverage for seven;
- kind-separated sparse raw metrics.

NEEDS_HUMAN_RULING:

- exact cleanup approval for the disclosed Cargo lockfile;
- later numeric selection among VD-N1..VD-N4 (recommend VD-N4).

DEPENDENCY_NOTES:

- No semantic cycle. Closeout has one cleanup-gated dependency.

AppliedChanges:

- Added only the declared derivative artifacts and updated this instance's
  `STATUS.json`/`RETURN.md`.
- Unexpected out-of-scope write is exactly the preserved ignored lockfile
  above; this is why the overall result is `FAILED` despite semantic `PASS`.
