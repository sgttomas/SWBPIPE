"""PROPOSAL ONLY: requires explicit root/witness lease coordination.
No invocation occurred in this diagnosis. Opens only exact-file metadata via ls.
"""
import json
import pathlib
import subprocess
import sys
import time

out = pathlib.Path(sys.argv[1]).resolve()
out.mkdir(parents=True, exist_ok=False)
record = {"proposal": "metadata-only ls plus exact-owned-PID sample", "started_unix": time.time()}
argv = ["/bin/ls", "-ledO@", "/Users/ryan/Downloads/openpipestress-preview-results-run-preview-linear-static-001.json"]
record["argv"] = argv
started = time.monotonic()
with (out / "ls.stdout").open("wb") as stdout, (out / "ls.stderr").open("wb") as stderr:
    process = subprocess.Popen(argv, stdout=stdout, stderr=stderr)
    record["owned_pid"] = process.pid
    try:
        try:
            process.wait(timeout=1)
        except subprocess.TimeoutExpired:
            sample_argv = ["/usr/bin/sample", str(process.pid), "1", "10", "-file", str(out / "ls.sample.txt")]
            record["sample_argv"] = sample_argv
            with (out / "sample.stdout").open("wb") as sample_stdout, (out / "sample.stderr").open("wb") as sample_stderr:
                try:
                    sample = subprocess.run(sample_argv, stdout=sample_stdout, stderr=sample_stderr, timeout=3)
                    record["sample_exit"] = sample.returncode
                except subprocess.TimeoutExpired:
                    record["sample_exit"] = "timeout; subprocess.run killed and waited owned sampler"
            try:
                process.wait(timeout=max(0.01, 8 - (time.monotonic() - started)))
            except subprocess.TimeoutExpired:
                record["metadata_timeout"] = True
    finally:
        if process.poll() is None:
            process.kill()
            record["metadata_cleanup"] = "killed exact owned ls and waited"
        else:
            record["metadata_cleanup"] = "owned ls already exited"
        record["metadata_exit"] = process.wait()
        record["elapsed_seconds"] = time.monotonic() - started
        record["finished_unix"] = time.time()
        (out / "RETURN.json").write_text(json.dumps(record, indent=2) + "\n")
print(json.dumps(record))

