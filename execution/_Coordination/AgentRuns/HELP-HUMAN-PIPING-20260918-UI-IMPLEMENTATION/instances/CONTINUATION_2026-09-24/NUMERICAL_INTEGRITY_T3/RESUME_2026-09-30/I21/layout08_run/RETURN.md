# I21 layout08 run — narrow actual-type return

**One authorized run passed all required checks and exited0.**
Same RV30 must backcheck this actual evidence before integration/use.
No full E_max, admission, private node alignment or product acceptance follows.

TASK Type2 under /root/t3_recovery_manager. Start13:28:20 UTC;
deadline13:48:20 UTC. Manager's explicit one-command release13:30:12.
Direct-clock launch13:30:27, PTY session21499, complete by13:30:40,
before the five-minute checkpoint. One Cargo run, one diagnostic execution,
zero source fixes, zero retries, no extra version probe.

## Actual requests and layouts

| Exact kernel specialization | Leaf request bytes | Internal request bytes |
|---|---:|---:|
| BTreeMap<u32,usize> | 144 | 240 |
| BTreeMap<(RuleTest,u32,Kind),BoundedExtremeTracker> | 1072 | 1168 |
| BTreeSet<(RuleTest,u32,Kind)> with actual SetValZST | 104 | 200 |

These are source-isolated requested-byte deltas on the exact compiled basis,
not guesses of private repr(Rust) layout. Node alignment stays UNBOUND.
Subject to backcheck, source_02's TI formula can use max(L,I)=240/1168/200
for the respective specializations without an alignment inference.

| Actual type | sizeof | alignof |
|---|---:|---:|
| (u32,SpringKind) | 8 | 4 |
| Dof | 8 | 4 |
| (u32,u64,u64) | 24 | 8 |
| (u32,f64) | 16 | 8 |
| (u32,u64) | 16 | 8 |

All three raw sample rows independently recompute correctly:
one first allocation; no fitting-insert growth; two split allocations;
positive L/I; total2L+I; true contents; starting byte count restored on drop;
no new successful allocation calls during drop. All five layout lines,
normal cfg output, final COUNTERS_AND_CONTENT_PASS and tool-confirmed exit0
are present. No earlier per-row REQUEST is used alone as acceptance.

RESULTS.json retains the raw counters and every checked equality. Observed
baseline612 is only a subtraction reference for this process/trial, never
a universal runtime baseline or an E_max allowance. The full unchanged
reviewed standalone source path supplies isolation reasoning; counters and
thread-count environment variables alone do not prove it. Cargo/compiler
processes do not share the example's registered allocator.

## Bound build and preserved evidence

Immutable production40129a225d73860ac2a53da9a2fa73869df668f3;
exact overlay0d6c1ff750241f76ffb15a590ae76b70b23581013e32fb78fb10899214d0d13a;
prep seald52b1f699836434850b696690f0bc8ba4ac6638e98f57f74784438bd729e0716.
All228 archive files and their exact file set still match after the run.
All18 manifests/locks and allocator remain unchanged; prep seal still verifies.

Actual fresh Cargo metadata reports rustc1.97.1, full commit
8bab26f4f68e0e26f0bb7960be334d5b520ea452, host aarch64-apple-darwin,
LLVM22.1.6. Ten verbose compiler invocations bind the nine local crates plus
example: release opt-level3, aarch64 target, no --test or activated --cfg
feature. FK/H type definitions are unchanged. The Cargo capability queries
in .rustc_info include default target cfg; those are not the release-profile
attestation, which comes from the actual compiler commands and printed
cfg_test=false/debug_assertions=false.

Binary SHA256:
63ea631507802f2a0ec939299b5dba2b7bdfb08728cb432be45d20c9e6938607.
Raw log SHA256:
98191c9031e5468a24587ea77b24c63bca09fc16da25c1ca23ec2bf08981fe66.
Raw log remains at <I21_LAYOUT08_SCRATCH>/logs/BUILD_LAYOUT08.log;
portable byte-preserving path-substitution copy is in this packet.
The destination is ROOT's explicit path-only override of the frozen
prospective command; no other argv/env/cwd/target/source was changed.

Existing guard PID5387 remained running; logged floor35%. Pre/post command
memorystatus observations were95%. time-l wrapped env/cargo; its statistics
are preserved only as command-resource evidence, not compiler/probe peak or
memory-bound proof. Five unused-function warnings came from the unchanged
allocator; no compiler failure or repair occurred.

One read-only evidence script used an overescaped regex and failed before
recomputation. The failed command is preserved. Manager explicitly permitted
direct recomputation of the existing raw rows only; the corrected evidence
reader passed, with no Rust rerun, source/criteria change or workaround.

C1 input capacities, C2 descriptors, O2 final-A1 reconciliation and W1
H staged/prefix versus VR global consumer composition remain unresolved.
T1 requested-byte and T2 actual-layout facts are supplied for the same
reviewer's backcheck; no broader bound closure is claimed.

Only additive layout08_run evidence, the authorized logs directory and
sibling target were written. No maintained source, old seal, Git/index,
allocator/observer/guard or dependency was edited; no delegation/install/
network/model/solver action occurred. No I21 experiment remains running.

