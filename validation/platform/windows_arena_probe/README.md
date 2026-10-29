# Windows arena and admission investigation

This standalone, dependency-free native-x64 experiment is a controlled arena API,
not a global allocator, solver, worker integration, or production transition
proof. It reserves 4 MiB of virtual address space per arena; that reservation
is not a full-budget lease. Deliberate committed allocations stay well below
64 MiB, and the process fixture creates at most two workers. Querying
`GetSystemInfo` supplies the actual Windows page and allocation granularity.

The arena commits its fixed 256-entry slot metadata inside the arena and charges
its rounded pages. Data pages are admitted before their first commit, counted
once, and remain charged while free slots or pages are retained. Alignment gaps,
fragmentation, live capacity, committed data and metadata are reported. Fresh
and reused blocks are zeroed. Moving reallocation holds both blocks during the
copy. The deterministic commit and decommit faults are injections, not induced
OS pressure. A failed multi-page allocation can retain already committed pages
and admission; its old block remains live. These conservative reservations
are not a production accounting design.

The child fixture uses a creation-time Job list, an explicit inherited handle
list, and an external bounded Job drain adapted from the pinned worker fixture
`226de967`. Its parent admits each page before the child commits it, carries
reservations through cancellation, and releases them after observed process
exit. The separate ledger examples vary physical and commit availability and
start with zero residency credit; actual committed bytes are recorded separately
from held liability. Neither the scripted numbers nor the 4 MiB reservation
establish production headroom, non-arena native/startup costs, or performance.

Run on native Windows x64 with Rust 1.97.1 and `CARGO_TARGET_DIR` set to an
isolated temporary directory:

```powershell
./run.ps1
```

The script builds with `--locked`, checks the complete case inventory, runs each
case with a 30-second external timeout, streams its output, and rejects an empty
or ignored case. The deliberately missing selection is a negative control. CI is
supplemental and outside the required harness. Results are pending until the
hosted job actually executes; no local compilation or execution was performed.

The current experiment is incomplete: the two-worker path does not yet exercise
an injected commit failure or moving realloc across its process boundary; the
2-second retry classifier is tested separately, not wired to a real retrying
commit transaction; a live blocked/terminating child is not held through a
forced kill test; and the observations have not yet been reviewed as native
cost evidence. The CI result cannot establish those missing properties.
