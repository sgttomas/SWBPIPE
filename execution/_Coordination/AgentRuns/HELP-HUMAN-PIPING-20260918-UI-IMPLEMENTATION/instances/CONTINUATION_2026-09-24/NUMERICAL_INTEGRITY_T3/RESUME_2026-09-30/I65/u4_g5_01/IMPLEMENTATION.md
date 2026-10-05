# U4 G5 part 1: the admission law in code, and how it maps to the derivations

**Where:**
- **Worktree** `WT/f2a-memory`, branch `codex/piping-f2a-memory-20261004`, based on `8abb5274a9`. Nothing is committed: ROOT commits.
- **Changed files:** `PP/src/retained_memory.rs`; `PP/Cargo.toml` (the `build` key only); `PP/src/lib.rs` and `PP/src/retained_product.rs` at exactly one line each, the D-5 exception.
- **New files:** `PP/build.rs`, `PP/src/build_identity.rs`, `PP/src/retained_memory_law_tests.rs`.

**Line numbers** below are for `retained_memory.rs` in the worktree.

**Part 1 covers** G-A, D-6, the gates, the bound comparison, R and the structural budgets.

**Part 2 has not started.** It covers:
- the cap-priced constants as in-build expressions;
- the FK resource module;
- the witness tests W1–W7;
- the allocation challenge;
- carry items 8 and 10.

The register of what moves into part 2's constants is in RETURN.md.

## 1. G-A: `admit` (:1545)

**What it runs, in order:**
1. **The census.** It has five parts, none of which allocates:
   - `borrowed_value_census`, unchanged;
   - `raw_text_census` (:230), the same cursor as the census, which reads the maximum string and key lengths and D1.11's control bytes;
   - `borrowed_request_census`, unchanged;
   - `nested_typed_census` (:435), RESIDUALS T03's roster: every typed String, restraints, springs, temperature points, the case's loads, and `project.units` as a separate Value;
   - the digest's capacity.
2. **D-6 `build_status()`** (:976).
3. **The domain clauses `domain_clauses`** (:869): D1.2–D1.11 over those facts and the borrowed typed request. This is a pure function.
4. **`law_order`** (:1511): D1.0, then D1.1, then the domain verdict, then the bound for the selected profile. This is a pure function, whose inputs are the build status, the domain verdict and the bound closure.

**What the report keeps.** The verdict goes into the report's **private** field `law: AdmissionLaw` (:1030), which holds:
- the census extensions;
- `refusal`: the first failing clause overall;
- `domain`: the first failing request-level clause, whatever the caller and build;
- the selected profile index.

The public report fields are unchanged.

**The signature is unchanged**, as U3 grant 1c made it: `Result<(CapturePermit, RetainedAdmissionReport), RetainedAdmissionReport>`. This is RV87 S-3: API_G4.md §1 still shows the older form.

| Clause | Code | Refusal | Precondition |
|---|---|---|---|
| D1.0 | `caller_clause`: Direct only (D-2; U3 F-5 refuses Headless) | `Caller(Headless)` | `caller` |
| D1.1 | `build_status()`: Missing with nothing registered; Stale on any mismatch | `Profile(Missing \| Stale)` | `resource_admission` |
| D1.2 | `census_complete_part`: raw, raw text, typed DOF, nested walk, Headless roots | `Census(part, status)` | `resource_admission` |
| D1.3 | schema 0.1.0/0.2.0; no pressure contract; reference configurations Absent; every material expansion law Absent; no request expansion laws; **no sections; no `section_ref`** (S-4) | `Family(D1.3, fact)` | `source_family` |
| D1.4 | one load case; no combinations or components | `Family(D1.4, fact)` | `source_family` |
| D1.5 | the case: no pressure regions, equivalent static, modulus basis ref or temperature; analysis state Absent | `Family(D1.5, fact)` | `source_family` |
| D1.6 | no hanger or nonlinear support; family None or exactly anchor, guide, line_stop, vertical_support or spring | `Family(D1.6, fact)` | `source_family` |
| D1.7 | node targets only; dimension force or moment | `Family(D1.7, fact)` | `source_family` |
| D1.8 | restates D1.4 for the members it implies; unreachable after D1.4 | `Family(D1.8, Components)` | `source_family` |
| D1.9 | `cap_rows` (:794), 45 rows in DOMAIN §2 order: counts and **typed capacities** (S-3), with **`l ≤ 128`**; sections, components, combinations and request-expansion capacities at 0; typed and raw text ≤ 128; raw totals and capacities; digest; the typed `project.units` Value within the raw caps | `Cap { fact, observed, cap }` | `resource_admission` |
| D1.10 | `provenance_clause`: no load provenance whose first byte after ASCII whitespace is `{` | `Family(D1.10, ObjectProvenance)` | `source_family` |
| D1.11 | the last cap row: control bytes in raw strings and keys = 0 | `Cap { ControlBytes, observed, 0 }` | `resource_admission` |
| bound | `bound_admits` (:1018): `E_mov,max + R ≤ M`, checked (S-1) | `Bound(Unpriced \| Overflow \| Exceeds)` | `resource_admission` |

**`cap_priced_maximum` (:1014) is `Unpriced` until part 2,** so the law fails closed even if a profile were registered.

## 2. D-6: the build identity, reviewed inputs and witnesses

**`build_identity.rs`** is the one encoder (G2_AMENDMENTS §1):
- `v1;key=value;…` over the 16 keys in order;
- `%`-escaping of `%`, `;`, `=`, bytes below 0x21 and bytes at or above 0x7F;
- `v1;unavailable` on any read failure.

**`build.rs`** includes it and emits one line, with `rerun-if-changed` on `build.rs` and `src/build_identity.rs`, and `rerun-if-env-changed` on RUSTC, RUSTC_WRAPPER, RUSTFLAGS, CARGO_ENCODED_RUSTFLAGS and RUSTUP_TOOLCHAIN.
- **An empty variable is a value.** `target.env=` is empty on Apple; a missing or non-UTF-8 variable makes the identity unavailable.
- **It never panics and never fails the build.**
- **This build's identity**, printed by the test:

```
v1;rustc.release=1.97.1;rustc.commit=8bab26f4f68e0e26f0bb7960be334d5b520ea452;rustc.host=aarch64-apple-darwin;rustc.llvm=22.1.6;target=aarch64-apple-darwin;target.arch=aarch64;target.pointer_width=64;target.endian=little;target.os=macos;target.env=;panic=unwind;profile=debug;opt_level=0;debug_assertions=true;rustflags=;pkg=open_pipe_stress_product_physics@0.2.0
```

**The reviewed-lock record** (RR "U4 G4"; an implementation choice, stated for review). build.rs also hashes the 14 reviewed inputs and emits `OPS_RETAINED_REVIEWED_INPUTS=v1;<path>=<sha256>;…`:
- the inputs are the PP `Cargo.lock` and the precommit reader's 13 `include_str!` files, in G4 ORIGINS order;
- the SHA-256 is self-contained in `build_identity.rs`, so there is no build-dependency; a test checks it against `sha2`;
- a registered profile records the exact text, and any difference is Stale, never a compile error.

**Why at build time:**
- the binary embeds no extra copies of the files (about 0.7 MB);
- nothing is hashed at runtime;
- `rerun-if-changed` on each file keeps the record current.

**The compiled hashes equal G4's reviewed record:** lock `4f494db6…475b`, and all 13 statics, as `reviewed_inputs_bind_the_lock_and_the_reader_statics` checks.

**Witnesses:**
- **`LAYOUT_WITNESSES` (:897)** is BUILD.md §2.3's conjunction, evaluated by the compiler. On this build it holds.
- **`READER_LAYOUTS` (:920)** is the in-build size and alignment of `Validation`, `ValidationError`, `RowClassification` and `AccuracyClass`, the reader types T17 prices. A registered profile records them, and a difference is Stale.

**The status.**
- `identity_match` (:951; Missing with nothing registered; otherwise the byte-equal index, or Stale) and `bindings_hold` (:966) are pure functions.
- `build_status` composes them with `REGISTERED_PROFILES`, **an empty static slice (:946)**.
- **The only `CapturePermit` construction** is in `admission` (:1499), from `REGISTERED_PROFILES.get(index)`. So no profile and no permit exist in maintained code or tests (decision 7). A structural test pins this.

## 3. G-B and G-C (API_G4.md as amended)

**The two narrow D-5 edits:**
- `retained_product.rs`, `LateFacts{…,capture:&*self}`;
- `lib.rs`, `CompleteFacts { ordinary: &ordinary, capture: &observer }`.

**Facts and their bounds** (`phase_caps` :1291; illustrative only where marked):

| Gate | Fact | Read from | Bound |
|---|---|---|---|
| G-B | built nodes, pipes, frame elements, supports | `built` lengths | n, m, m, g = 32 |
| G-B | case loads | `case.primitive_loads.len()` | l = 128 |
| G-B | restrained, springs | the hook's slices: **lengths** | k = min(6n, r) = 192; s = 192 |
| G-B | materials | slice length | 8 |
| G-B | observation bytes before the late capture | the adapter's `RustCapacityBytes` tally | **part 2** (T11 without P1; `UNPRICED`, so it refuses) |
| G-C | envelope results and diagnostics | lengths | P_final = 2,115; D_env = 9,360 |
| G-C | their capacities | `capacity()` | **part 2** (PushCap laws) |
| G-C | result and diagnostic text | the sum of every String capacity they own | 2·P_final·Text(row); 2·Text(diag_env) = 137,419,080 |
| G-C | **longest single string**, **longest diagnostic id** (RV87 N-3) | the maximum length over rows, diagnostics and the preview tree | L_PUB = 2,599,962 (RV84 C-N1); L_DIAGID = 2,330 |
| G-C | preview tree | `borrowed_value_census` and `raw_text_census` of `contract_evidence`: status, array capacity, objects, entries, string and key capacity | complete; ordinary_caps.py's PREVIEW facts at the caps |
| G-C | source-block recovery present | `is_some()` | 0 |
| G-C | observation bytes, **including the late capture** | the adapter tally | **part 2** (T11) |
| G-C | ordinary seeds | `capture.ordinary` capacity × stride + their Strings | **part 2** (T11.4) |
| G-C | retained error text (RV84 C-N4) | `capture.error` and `observable_error` `Association` Strings; `g5a_error` holds only `&'static str` and integers | (3m+1)·Text(err) |

**RV87 S-4, the capacities.** The hook receives `restrained` and `springs` as slices, and the owning Vecs are `lib.rs` locals of the observed run, outside the D-5 exception. So:
- G-B reads their lengths;
- their capacities stay bounded where O prices them, by the construction law of those locals.

A capacity fact at G-B would need the hook's signature to carry the Vecs, which is I61's change. Recorded for ROOT.

**RV87 S-4, the late capture's actual owners.** API_G4's `capture.P1_old_source` names no field. The late old-source capture is made in `capture_case_source` (retained_product.rs:1281–1340):
- `ProductCapture.source` (`Option<k::PrimitiveSource>`, built from `SourceParts`);
- `case_id`;
- the `facts`, `members`, `operational`, `supports` and `terms` Vecs.

All are allocated through `self.adapter.reserve` and `copy`, which enter `RustCapacityBytes`. G-C's `ObservationBytes` therefore measures the late capture after it is made (RV84 S-6(c)).

A separate T11.P1 fact would need the G-B tally kept on the observer, which is a `retained_product.rs` change outside the fence. Recorded.

**The checks.** `check_phase` (:1259) is the first observation above its bound, with sums saturating at `u64::MAX` (above every bound). `check_late` and `check_complete` use `phase_caps()`.

## 4. Budgets, R and the bound

- **`PHASE_BUDGETS`** (:1358), B-1 to B-10:
  - one ordinary run and one parse per invocation;
  - zero fallible allocations after the first mutation, and zero new `W1Fallback` allocations;
  - R;
  - the thread heap: 8 KiB + `size_of::<RetainedPreviewOutput>()`;
  - the N1 reserve, **tightened to grant 1b's exact reservation** as an in-build expression: one Diagnostic slot, the id ≤ 2·(30+128+12), code, severity, source, the 196-byte message, and one case ref. That is about 0.9 KB, where G4 priced 1.07 MB of push-growth headroom.
  - **The byte budgets for the staged copy, successor, invocation and reader are part 2.**
- **R = 64 MiB and k = 16** (:990). The `cfg(test)` override is read on the caller's thread. `STACK_RESERVATION_PRECONDITION` = `resource_admission`.
- **`bound_admits`** is checked addition, with `required ≤ M` admitted. Tests cover M−1, M, M+1 and overflow.

## 5. Tests (`retained_memory_law_tests.rs`, 23 tests; the 5 existing census tests are kept)

**Encoder and identity:**
- round trip over every byte;
- every key present, in order, with its decoded values (pointer width, endianness, OS, architecture, panic, profile, debug assertions, package);
- identity matching: Missing, Stale or exact;
- bindings, and layout witnesses holding on this build;
- SHA-256 against `sha2`;
- reviewed inputs equal to G4's record.

**Permit:** none is constructible, and a forged index cannot mint one.

**Census:** the raw-text census, and the nested roster.

**D1:**
- the milestone and a cap-maximal input are inside D1, in both modes; Headless is refused at D1.0;
- every family clause refuses with its fact;
- **all 46 cap rows (D1.9's 45 and D1.11's) admit at their cap and refuse at cap + 1**;
- actual inputs map each count, text, depth, raw-total and raw-capacity fact; typed capacities and units rows read the actual owners;
- unknown, stale, overflow and partial refusals keep every fact, and **the ordinary bytes are identical** through the public entry;
- `law_order` reports the first failing clause;
- the bound at M;
- refusal kinds match the schema's preconditions.

**Gates:**
- every phase fact admits at its cap and refuses at cap + 1;
- G-B and G-C facts equal independent sums over the milestone's actual owners;
- stack R and its override; the structural budgets.

## 6. Controls

The candidate is the worktree's files overlaid on a copy of base `8abb5274a9`.

1. **Published bytes.** The 36-input × 9-route fixture sweep (`zz_i65_g5_fixture_sweep.rs`, scratch only; 324 lines) is **byte-identical to base**, including every public admission-report field.
2. **Nothing weakened.**
   - PP outcomes equal base's plus the new tests; t13 is the only failure, as at base.
   - runner/headless outcomes are identical.
   - No reader, schema, fixture or existing check changed.
   - No test permit.
3. **Mutants.** `_run_records/mutants_g5.py`: every D1 clause, every gate, the bound comparison, the build-identity check and the stack refusal. Of 84 mutants, 83 are killed by a test and none by a compile error. The one survivor is equivalent by decision 7 (RETURN.md).
