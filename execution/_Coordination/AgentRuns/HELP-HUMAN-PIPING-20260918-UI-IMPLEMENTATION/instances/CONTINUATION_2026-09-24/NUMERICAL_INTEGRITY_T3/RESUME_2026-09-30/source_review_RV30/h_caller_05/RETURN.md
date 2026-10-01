# RV30 h_caller_05 — source10 conditional H caller review

**No blocking defect found in the checked finite H caller composition or numeric bounds.** The formula remains conditional: it is not a numeric prelaunch E_max or admission proof. Two non-blocking refinements substantially narrow the declared missing premises, without requiring another broad audit or probe.

Independent TASK Type2 RV30, native child `/root/rv30_k6c_kernel` of ROOT `/root` HELP_HUMAN Agent0; no delegation. Actual start2026-10-01 14:32:44 UTC; deadline14:52:44 UTC. Completion after checks is recorded in VERIFICATION.json. Direct parent grant permits only this bounded source/evidence review and additive h_caller_05 output. The prior Root/TASK/Piping/COMMON/software-code-review origins and hashes remain at ../kernel_01/INSTRUCTION_BINDING.json.

Reviewed source10 seal `4654cec41603ef0ded7c4a057ece046630b6b72313e156b04543cd36f53e7650`, H_ENVELOPE hash `df2bbc47b46c188ab74bd475888df873f6f859b28fcc038c3be430d5bb78ac8a`, and manager seal `767ad8687a1591f8fa349afbe9d2a65700057d84999d3c655a3c8bb8054bbbe6`. All payloads verify. Product is immutable40129a225d73860ac2a53da9a2fa73869df668f3;17 named source/review inputs match the recorded hashes, using previously bound archive/scratch sources. All12 supplied installed-library pages were independently hashed and decoded and match their preserved source bytes. No Git command was used.

Aliases: M=H/src/bin/k6_observe/main.rs; HS=H/src/k6/w1/staged.rs; HM=H/src/k6/models.rs; RUN=H/runner/k6_runner.py; A=K/adaptive.rs. Envelope line references below mean source10/H_ENVELOPE.md.

## Non-blocking refinements

### RV30-H-N1 — the geometry-error part of H-F0 is already a three-leaf grammar

**Location:** Envelope:233-249,301 (H-F0 table row).

K/factor.rs:201-218 forwards only the Err from assess_rigid_body into GeometryUnavailable. Frozen rigid_body.rs:33-191 has exactly three direct Err returns:
- :42 InvalidInput("rigid geometry");
- :50 Range("relative coordinates");
- :56 Range("nonfinite characteristic length").

The original_rigid_witness helper's errors do **not** propagate: :181-183 accepts only Ok(Some(motions)); every Err is discarded and the candidate loop continues. The directional span path at factor.rs:81-119 produces Option/bool, not a propagated StructuralError. Consequently a wider StructuralError leaf/call-chain audit is unnecessary for this H refusal route.

For non-pretty derived Debug, these three ASCII schemas have lengths30,29,40. Their literal messages contain no escapable characters. Thus:
`D_GeometryError_Debug <=40`;
`GeometryUnavailable <=49+40=89`;
wrapped `Refused(GeometryUnavailable...) <=98`.
The existing100-byte wrapped-Unresolved maximum already dominates this nonfloating geometry branch.

**Small repair route:** carry the additive40-byte geometry-leaf bound and reduce the remaining solve-error length premise to
`D_ErrorSolve <=max(80+6*D64_Debug,100)`.
H-F0 then needs only the default Debug length/heap/active-old behavior of the six rigid-parameter f64 values. MechanismWitnessed accepts a candidate only through original_rigid_witness, which rejects nonfinite candidates at rigid_body.rs:199-201; no arbitrary user format, dynamic width/precision, nested model or general StructuralError graph is needed. No float bound is guessed here.

This is refinement of an honestly open cell, not a discovered undercount in an asserted closed branch. GEOMETRY_GAP_NARROWING.json records the source schemas and arithmetic.

### RV30-H-N2 — private Thread-handle layouts are not a K6Alloc requirement

**Location:** Envelope:198-212,298-299 (H-R0/H-R1).

The pinned core::alloc::GlobalAlloc contract explicitly states that std::thread_local infrastructure, std::thread::current, thread::park, Thread::unpark and Thread::Clone do not allocate through `#[global_allocator]` (installed core/alloc/global.rs:119-133, hash047073eec6377dd8730b03dc036db2e44ab3965ce3981638314806c5d72a0529). This does not say there is no System/OS storage, nor that an arbitrary user TLS initializer cannot own a Vec. It is the relevant boundary for K6Alloc's registered-request metric. A private Thread child request therefore cannot be added to this metric merely because current() can create a handle.

The supplied rt.rs:111-119 calls current_id before sys::init/main. thread/current.rs:17-39,106-116 retains that thread ID; its current_id:164-175 path reads/initializes the ID, and the cached path does not create a Thread handle. ReentrantLock:284-299 calls current_id, not current(). On the same main thread through H's finite stages, there is no repeated ID initialization to budget as a fresh handle.

**Small repair route:** remove any need to measure or reconstruct private Thread-handle child layouts for this registered-allocator term. Retain the separate, genuinely unbound selected mutex/backend and other persistent registered-runtime owner questions. For current_id specifically, preserve the main-thread ID/TLS source proof rather than starting a Thread layout programme. The direct pinned contract is preserved in RUNTIME_BACKCHECK.json.

## Finite application-owner proof checked

### Processes, repeats, model and saved attempts

RUN:365-368 preserves33 W1 model identities. W1_PAIR has two W1 entries, yielding66 planned W1 processes; interleaved sparse processes have different allocators/address spaces. W1 repeats remains5; time/first-repeat/source stops only reduce completion. There is no factor33,66 or5 on per-process persistent owners.

HM builder/conversion and the source09 result support the displayed C_model_RF and C_model_DEC terms. One K6Model and G_136(m) frames remain live from M:559/574 through observed stages. The Args model ID and model.id are separate allocations. The RF member-label clone and moved node labels retain different construction classes; DEC labels use their own fixed format. No heap Box is implied by inline wrapper sizeof. The remaining accessible Option<[f64;3]> term can use its exact size_of expression in later implementation; no separate witness is required.

K6Counts/W1Counts/SizeFacts and Observer have no heap children on this path. Counts-file text, the found row's allocated ID and computed-count scratch end at M:646 before Observer construction. repeat0_solution stays None for W1; the dense/sparse sibling branches do not create a hidden persistent solution in W1. This is a source-path zero, not a runtime-baseline assumption.

M:872 clones only attempts, including their deep summary/refusal children. With at most4 records and3 verification records:
`A_saved <=4*832 +3*(40B+24*2b)=3328+120B+144b`.
Fresh Vec/String-child clones use logical lengths. No state/cache/report/publication/radius is cloned by attempts_of(...).to_vec. The old saved vector overlaps its replacement only after the inner solve sample, then drops. Each later repeat retains one saved vector. The current solve outcome drops at the match-arm edge after repeat_digest; selected results do not accumulate.

### Prefix list and outcomes

HS:292-325 creates at most4 solve,3 verification and3 decision segments. prefix_limits takes all but the last, so J<=9. Its borrowed exact-size take/map collection has exact32J backing, and labels clone exact logical bytes at most11 each. Hence PrefixList<=387 bytes. The G_32(S) segment vector and its original labels overlap list construction, then die before prefix reset.

The loop's prefix_limits(full).iter() temporary remains for the whole loop. The saved full attempts are borrowed, not cloned. Each prefix constructs a source before begin; the retained SRC0 is present at reset while dead constructor helpers are outside that prefix sample. One current prefix outcome remains through the outer read and prefix_line, then drops. The next prefix does not overlap a prior outcome. prefix_matches' two temporary segment lists occur after the outer read and drop before the next reset.

### Observer edges and alias composition

M:281-290 emits stage_begin before stage_reset. Its Line/number temporaries die before the reset. HS:57-63 and77-82 create error Strings before end; M:298-305 reads the inner peaks before stage-end Line construction. The inner source/solve measurement therefore includes ErrFmt construction and its retained error String, not stage-end emission.

M:965-966 performs the prefix outer read after w1_solve returns, so it includes that end Line and stdio activity while the returned outcome remains. Error text and its escaped Line copy are separate buffers. prefix_line follows the outer read; its temporary data is not charged to the just-completed prefix peak. Subsequent reset prevents it from contaminating later prefix peaks, while genuinely persistent runtime effects belong in R_live.

The displayed C+saved+kernel/outcome identity unions use SRC0 once and do not add another current outcome as if it cloned K_phi. Constructor/refusal alternatives are successive source phases. I found no missing application-owned retained vector or cloned selected payload in these finite branches.

## Numeric format and byte bounds

I independently recomputed all22 SourceError grammar rows and all9 UnresolvedReason rows from their actual fields, using the non-pretty derived struct/tuple/enum token schema. Source types contain only the listed primitive enums/integers at these leaves. Maxima match:
- u32 decimal10; usize/u6420; u12839.
- Dof39; MemberProperty15; SourceError75, destination F(75)=150.
- SumError19; WideError32; CertificateIssue19; UnresolvedReason88.
- `Unresolved(...)` wrapper adds12, total100, destination F(100)=200.
- Refusal constants71 for six-float mechanism,49 for geometry,67 negative-energy,51 ledger and9 structure match. N1 supplies the narrower geometry leaf.

The stage-end Line has11 fields. Prefix index<=9 gives static label length11; repeat or repeats_completed<=5 uses one digit; elapsed u128 uses39; current-begin/current-end/peak/peak-move plus alloc_calls use20 each. Recomputed JSON punctuation/key/value length is315 with null error and313+6D with a quoted escaped error. The6D rule safely covers each UTF-8 byte; control escapes need6 ASCII bytes.

Line::n creates one temporary String before appending; the unsigned ordinary ToString path uses stack digits/exact output and the small-code generic path is safely bounded by F(39)=78. Control-escape format temporary has length6 and F(6)=12. These temporary classes occur sequentially, so max78 is safe. Complete Line padding plus the current temporary gives known no-error requested bytes630+78=708. It excludes outcome, caller, stdout1024 and runtime/stdio cells as declared.

For growing-realloc movement, the Line old request is bounded by F(L)/2, number/control old requests by39/6 respectively. Those allocator calls are sequential. ErrString is already an independent retained owner during Line emission and does not grow then. The displayed maximum with O_stdio_emit/O_Debug/O_runtime is correct conditionally; those unknown active-old terms are not set to zero. No unconditional full prefix bound is claimed by708 or708+315.

The fixed-template F rule remains a retained destination bound. Nonfloating primitive/derived schema writes to the supplied formatter; the symbolic total still retains T_Debug/O_Debug where a helper path has not been warranted. The six-float leaf's length/scratch cannot be replaced by unrelated tracker or observed-baseline storage.

## Argument/stdout partial bindings and exact remaining premises

The supplied Unix/Apple argv, Args IntoIter, OsString::into_string and String::from_utf8 source route supports exact-length moved String children on the fixed valid-UTF8 runner path. Only model ID, optional counts path and first-pass rows path remain in Args. The argc backing and consumed flag/numeric/executable strings drop when parse_args returns. Runner argv population is15+optional2+first-pass3<=20, not a retained second argument vector. No path ceiling is invented.

The source warrants one1024-byte stdout LineWriter/BufWriter buffer initialized by start.emit before model/stages. Its vector does not grow; its outer OnceLock/ReentrantLock/RefCell wrappers are static. This is only that buffer's allocation, not total stdout/runtime memory.

The unresolved cells should be read as narrowly as follows:

| Cell | Remaining minimum premise |
|---|---|
| H-R0 | Registered-allocator allocations that **survive into** the first relevant reset, plus any later reachable initialization/growth in this finite main-thread path. Dead startup temporaries, loader/OS/System-only storage and unrelated std entrypoints need no all-process audit for this metric. N2 removes a private Thread-layout obligation. The packet has not proved the complete surviving registered-owner set. |
| H-R1 | Whether the selected stdout sys::Mutex backend creates registered-allocator child storage, its request size if it does, and lifetime. stdout is initialized before stages, so its child cannot be multiplied per emission. The scalar cached current_id path is distinct from current() handle creation. Static wrapper sizeof is not heap. |
| H-R2 | Only the reached byte-slice LineWriterShim::write_all/flush to StdoutRaw/macOS writer and **dropped** error values need heap/active-old classification. Line::emit ignores both Results; it does not format arbitrary I/O errors. A general I/O-error Display/serde/stdout programme is unnecessary. No zero is assigned without that direct path warrant. |
| H-F0 | After N1, only default Debug length and registered-heap/active-old scratch for the six finite rigid-parameter f64 values; no general StructuralError propagation or arbitrary formatter input. |
| H-I0 | The actual planned model identity, pass flags and UTF-8 path lengths. All33 fixed model constructions remain; no model-file branch or guessed universal path limit is needed. |
| H-C0 | Checked implementation/source finalization and final-A1 reconciliation. This review supplies the requested finite grammar/owner check, not an implementation grant or full runtime-owner proof. |

These remaining variables are proof obligations, not arbitrary numeric estimator inputs. The conditional same-process cut relation does not remove them or replace prelaunch admission. R_live must not silently absorb unspecified globals as a guessed constant, and the observed612 from layout08 is irrelevant to that warrant.

I did not load selected mutex, raw-write, floating conversion or full startup backends to close these remaining premises. No broader runtime/IO/format audit, probe, model, numerical run, source fix, tool development, Rust/build/test, Git/index operation, network/install or delegation occurred. All writes are this additive review subtree. Source10 and prior seals remain preserved; no complete E_max or admission acceptance follows.

