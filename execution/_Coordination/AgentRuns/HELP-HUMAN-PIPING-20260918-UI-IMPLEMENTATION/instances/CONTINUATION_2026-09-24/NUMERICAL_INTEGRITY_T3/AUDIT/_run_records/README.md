# Audit evidence and reproduction

Basis: `74b3c7313491f27f71c4361d5e1657ee4a39e2f1`. Use Python 3.13, the standard
library, Git and shasum. `merges`, `dispatches` and `ci` additionally use an
authenticated gh CLI for read-only GitHub queries. No mode builds or runs the
product. Run commands from the repository root; `<audit>` means the parent
AUDIT directory, expressed with its repository-relative path.

```
python3 -B <audit>/_run_records/audit_checks.py arithmetic
python3 -B <audit>/_run_records/audit_checks.py hashes
python3 -B <audit>/_run_records/audit_checks.py basis
python3 -B <audit>/_run_records/audit_checks.py merges
python3 -B <audit>/_run_records/audit_checks.py dispatches
python3 -B <audit>/_run_records/audit_checks.py ci
python3 -B <audit>/_run_records/audit_checks.py dec025
python3 -B <audit>/_run_records/audit_checks.py admissions
python3 -B <audit>/_run_records/audit_checks.py figures
```

The captured JSON files are named after the modes. `dispatches`, `ci` and
`dec025` read `merges.json` for the named candidates/runs. The script emits
results to stdout, and does not edit inputs. `hashes` excludes this new AUDIT
packet. Verify this packet's own SHA256SUMS from AUDIT, when present. Replays
should go to scratch; do not overwrite committed evidence.

`arithmetic` is an independent rational calculation. It imports neither the
solver nor GEN, and asserts its narrow mathematical counterexamples. It is
not a realized-source reproduction. `admissions` reuses the audit-basis runner's
admission function to replay recorded decisions, then changes only the input
estimates and calibrated history; it does not independently certify that
runner's policy. All 36 original decision dictionaries match exactly.

`figures` derives byte counts, outcomes and coefficients from committed source
records. It does not relaunch observation binaries. `dec025` records historical
SWEEP identity and parses the retained suite summaries; original per-manifest
logs are absent from the six merge folders. The accompanying comparisons are
derived records, not raw replacement logs.

## Host and actions

- Started in a clean detached checkout at the basis, which equals origin/main
  and live GitHub main as checked during discovery.
- Created only `codex/piping-t3-audit-20260930`. The first branch command was
  denied by the filesystem sandbox at the shared Git metadata lock; the
  approved escalated command succeeded. No existing branch was reset.
- Initial sandbox network reads could not resolve GitHub; approved read-only
  escalation succeeded. No workflow was dispatched.
- The supplied original T3 runtime paths do not exist on this host. A sandbox
  process listing was unavailable; the approved escalated pgrep found no
  memory-guard process. No heavy computation or source repair followed.
- No agent was spawned. No message was sent to another agent or project loop.
- Guidance origins and hashes are preserved in `basis.json`; source history
  is anchored by the full base SHA. The accompanying chat/tool record carries
  the exploratory commands. Basis metadata lists named principal inputs, not
  a claim that every search hit was read in full.

## GEN-8 clean-base check

The prescribed venv was absent. The installed Python 3.13 has pytest and the
required imports, so this pure portability check used it as a disclosed
substitution. Before audit files were created, on the clean audit branch:

```
PYTHONDONTWRITEBYTECODE=1 CHIRALITY_REQUIRE_LIVE_TESTS=1 \
python3 -B -m pytest -p no:cacheprovider -q \
tools/practitioner_harness/test_live_baseline.py -k gen8
```

Tool result, transcribed (not presented as an original log):

```
.                                                                        [100%]
1 passed, 10 deselected in 33.72s
```

Live-root checks were required, and bytecode/cache writes disabled. No skipped
GEN-8 pass is claimed. This is not DEC-025 or evidence of the original venv.
The staged packet was checked separately with the same command, so the new
files were included in Git's tracked set: 1 passed, 10 deselected in 35.64 s.
This was a staged-tree check, not a claim that the working tree was clean.
Candidate validation and integration status are recorded in the handback and
the records PR; any clean committed-candidate check belongs to that PR record.
