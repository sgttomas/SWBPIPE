# Replay

Preserve this sealed packet. For a rerun, copy the packet to a new review-owned
scratch directory, excluding SHA256SUMS/WRITE_INVENTORY.json and old run outputs;
keep controls/lib.rs, Cargo.toml, Cargo.lock and run_checks.py together. The runner
uses its own parent as the new record/target root (create _run_records there).
Run `python3 controls/run_checks.py` from that copy only after obtaining the sole
Cargo lane and checking the existing guard. The absolute dependency and fixture
paths name frozen f2a candidate 38798e6; check its hashes first. No production
source is copied or altered by these controls. Each command has a 1200-second
wall, four jobs, two test threads and a separate target per manifest. On a new
review do not silently replace the source revision or interpret historical pass
records as a current pass.

The preserved setup failure is reproducible by restoring the single borrow
shown in _run_records/CONTROL_REPAIR.json before the initial debug invocation.
This is a harness signature error, not a product failure. The final controls and
all numerical assertions match both successful debug and release records.
