# Portable replay of the retained residual observation

The [custody record](_run_records/ROW_SUCCESSOR_CUSTODY.json) binds the exact twelve original inputs to the [canonical packet](_run_records/POLICY_ROW_PROBE/RETURN.md). The historical [execution basis](_run_records/ROW_SUCCESSOR_BASIS.json) and its original temporary paths/hashes remain unchanged. These are the original pre-repair SparseInteractive observation and independent proof, not a new current solver run.

From the repository root, use Python3 standard library to copy the retained proof script to a temporary output directory and run it against the canonical packet. Copying the script prevents overwriting the original proof output, because its historical implementation writes beside itself:

```python
from pathlib import Path
import json, shutil, subprocess, sys, tempfile
root = Path(subprocess.check_output(["git", "rev-parse", "--show-toplevel"], text=True).strip())
continuation = root / "projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24"
records = continuation / "CORRECTNESS_DESIGN/NUMERICAL_POLICY_REVIEW/_run_records"
packet = records / "POLICY_ROW_PROBE"
with tempfile.TemporaryDirectory(prefix="piping-row-proof-") as temporary:
    script = Path(temporary) / "row_successor_backcheck.py"
    shutil.copyfile(records / script.name, script)
    subprocess.run([sys.executable, str(script), str(packet)], check=True)
    actual = json.loads((Path(temporary) / "ROW_SUCCESSOR_PROOF.json").read_text())
    expected = json.loads((records / "ROW_SUCCESSOR_PROOF.json").read_text())
    print({"replay_python": actual.pop("python"), "original_python": expected.pop("python")})
    assert actual == expected
```

Use a Python3 version supporting this retained script and its standard-library APIs. Runtime version strings are reported separately because they identify execution provenance; all remaining proof fields are compared exactly. The original proof file remains unchanged. No Cargo, GUI or model solve is part of this arithmetic replay. The original run's failed zero assertion, source identity, contact-state limitations and prospective successor disposition remain preserved.
