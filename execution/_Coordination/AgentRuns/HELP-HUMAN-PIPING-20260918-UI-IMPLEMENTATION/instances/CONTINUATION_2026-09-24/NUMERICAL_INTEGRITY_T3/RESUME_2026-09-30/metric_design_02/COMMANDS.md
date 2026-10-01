# Read and evidence command record

The tool-call transcript preserves original commands and outputs. This compact
record provides exact source identities without duplicating sealed source12.

- clock tool returned start2026-10-01 15:24:04 UTC.
- rg --files R/I21/source_12 located H_LEAVES/RETURN and selected decoded sources.
  That listing was truncated; no finding relies on the omitted filenames.
- cat H_LEAVES.md RETURN.md; shasum -a256 source_12/SHA256SUMS gave the supplied seal.
- cat selected mutex/pthread/ThreadInfo source pages and BINDING.json.
- rg --files R/I21 | rg selected source04-11 top-level files was discovery only.
- GIT_OPTIONAL_LOCKS=0 git show 40129a225d73860ac2a53da9a2fa73869df668f3:
  projects/chirality-piping/core/solver/performance_harness/src/bin/k6_observe/alloc.rs
- sed selected GlobalAlloc, stack_overflow,current,OnceBox,rt and BTree node ranges.
- FACTS.json first1500 chars inspected only for existing layout-evidence context;
  no ThreadInfo/pal request value was found or claimed by that excerpt.
- LIBRARY_SOURCE.json first12000 chars displayed; VERSION_BINDING first100 lines.
- rg --files of installed std/sync source identified poison/mutex.rs.html.
- Supplied VENV Python -B decoded that one HTML page; initial displayed ranges
  missed some code and a second targeted pattern pass displayed the exact
  Mutex struct/new/lock/guard-new/drop entries. SHA256s and transformation are
  in SOURCES.json; the full decoded evidence is preserved in this packet.
- VENV Python -B wrote new evidence only and verified all source12/prior packet
  payload hashes. No Git status/index command, Rust command or probe was run.

All observed command exits were0. source12 RAW_COMMANDS search output included
its embedded earlier h_caller_05 record; it did not expose active h_leaves_09.
Compiler/debug-artifact availability remains unverified. No path-normalized
command here is misrepresented as a runtime or layout result.
