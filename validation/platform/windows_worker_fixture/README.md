# Native Windows worker fixture

A dependency-free standalone fixture for RT-V7 / DEL-04-07. It has no desktop,
solver, Tauri, wasm or App transport connection. Four deterministic model cases
and nine native Windows cases exercise a deliberately small worker/control and
accounting environment. They do not establish general solver checkpoint coverage,
Windows allocator-envelope qualification or production transition-growth bounds.
Windows retained registration and governing memory/wait guarantees are unchanged.

On native x64 Windows with Rust 1.97.1 MSVC and PowerShell 7:

```powershell
./run.ps1
```

The supplemental Windows workflow selects the exact candidate, uses an isolated
Cargo target and product sparse checkout, and runs all 13 named cases separately.
It streams compiler/test/child diagnostics and requires exactly one passed,
non-ignored test per case; missing cases or child failures fail the lane. Native
cases self-spawn the same test executable through creation-time Job assignment.
The child entrypoint is internal and is never counted as an independent witness.

Each controller holds a 2 MiB fixture payload/control allowance before spawning
and through observed exit. A mutator has one 512 KiB page-touch buffer; control
and release helper threads request 128 KiB stacks. The accounting test uses one
4 MiB mapping then one 4 MiB heap allocation. Cases run serially and create at
most two worker/caretaker children. These are bounded intentional fixture
allocations, not a cap or accounting proof for the Windows loader/test harness.
Startup and normal exit waits are at most 5 s; workload loops stop after 8 s;
caretaker and cleanup backstops are bounded. Kill requests/EOF do not release
the fixture reservation. An outer cleanup Job and real process handles cover
failure paths, including a deliberately retained inner Job handle.

The uncheckpointed case must report **missing quiescence with ongoing progress**.
That expected negative observation is not a production refusal policy. Counter
snapshots keep private commit, private working set, total working set and shared
commit separate; OS deltas and allocator retention are observations, not exact
reference outputs, and no commit-based residency credit is computed.

See DEL-04-07 `Work/windows-worker-fixture.md` for case boundaries, ownership,
API sources, threat model and remaining proof obligations. Host model tests or
Windows-target type-checking alone are not native Windows execution evidence.
