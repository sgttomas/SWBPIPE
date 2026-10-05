# G4 notes: the D1.10/D1.11 refusal kinds, RV82-N5, and the D-6 lock re-pin

## 1. The refusal kinds for D1.10 and D1.11 (RR "U4 G3 verified": confirm in G4)

**Confirmed as ruled.** The test (DOMAIN.md §3) separates two kinds of clause:
- a clause that keeps the invocation inside the **source paths** the derivation priced is a `Family` refusal, `source_family`;
- a clause that bounds a **resource fact** the pricing uses is a `Cap` refusal, `resource_admission`.

| Clause | Private reason | Precondition kind | Why |
|---|---|---|---|
| **D1.10**: no primitive load's `provenance` begins, after ASCII whitespace, with `{` | `Family(D1.10)` | **`source_family`** | It excludes a source path: the generated-self-weight validation module. `inspect_applied_self_weight` parses each provenance (self_weight.rs:796–799 at `b1f80234dc`) and enters the module only for a self-weight `method` or an Element-target identity (D1.7 excludes the latter); otherwise it reaches `continue` (:826). It is the same kind as D1.3–D1.8. RV83 confirmed the module is unreachable under it (REVIEW_RV83/u4_g2_02 §D1.10) |
| **D1.11**: no raw string value or key contains a byte below 0x20 or the byte 0x7F | `Cap(control_bytes, observed)` | **`resource_admission`** | It changes no computation path. It bounds a census fact, the count of control bytes (cap 0), that fixes the escape factor ε = 2 in every hash route's text bound. That is the same kind as D1.9's caps |

**For G5:**
- The census gains one allocation-free counter, `control_bytes`, over the borrowed raw Value's strings and keys.
- D1.10 reads each `PreviewPrimitiveLoad.provenance`'s first non-whitespace byte.
- Neither allocates.
- `unavailable_precondition` already lists both kinds (retained_precision_mp_v2.schema.json:6478–6485), so no schema change follows.

## 2. RV82-N5: the producer's conservation leaves build flags and execution order to the readers

**Recorded as conformance**, per RV82 u1_serializer_01 N5.

**What the producer checks.** U1's serializer checks the work conservation D-4 §3 item 5 lists:
- own = W + K;
- own stages = O;
- shared stages = S + V;
- D + Q ≤ O;
- the B/T run partition;
- after = before + increment;
- charged = after.

These are in `physical`, `run_conservation_amounts` and `invocation_arrays` (retained_wire.rs:1002–1040, :1093–1129, :1383–1410 at `b1f80234dc`).

**What it does not check, and who does.**
- **The build flags** (`shared_built_here`, `verification_shared_built_here`) are checked by the Rust reader's `g5_native` against `builds[]` and the run's origins (result_export retained_precision.rs:1089, :1096, :1163, :1168).
- **The execution order** (`work.execution_order`) is checked against the runs (:710).

**Why that is acceptable.** Under D-4 §3 item 6, the Rust reader runs at U3's precommit (PP lib.rs:2992). A failing check there returns `W1Fallback::Precommit`, and the publication keeps the ordinary bytes plus the N1 notice. In D1 the execution order is always `[{kind: case, index: case_index}]` with one case (retained_wire.rs:1555 at `b1f80234dc`).

**So the property holds for every published successor:** a build-flag or execution-order defect cannot be published, because precommit validation refuses it.

## 3. The D-6 lock re-pin, now that U3 has made `result_export` a runtime dependency

**What changed** (`bee3dc07ca`, merged at `b1f80234dc`):
- PP/Cargo.toml moves `open_pipe_stress_result_export` from `[dev-dependencies]` to `[dependencies]`.
- **PP's own Cargo.lock is byte-identical** to the U1 merge's (`3260d7809e`), because a lock records dev-dependencies too. U1 had already added the package block, +11 lines.

| Field | G2 baseline (NUM `a2c26cc885`) | **Re-pinned (NUM `b1f80234dc`)** |
|---|---|---|
| Direct consumer lock | `P/core/product_physics/Cargo.lock`, SHA-256 `f28eee2b…caac3`, 36 packages | **SHA-256 `4f494db6d8a6eca87e7a16d8561197f20b1951a033bd3a6c424acfff5613475b`, 37 packages, 22 registry packages** |
| Relevant registry pins | serde_json 1.0.149 (`std`, `float_roundtrip`), serde 1.0.228, ryu 1.0.23, itoa 1.0.18, zmij 1.0.21, sha2 0.10.9, memchr 2.8.0 | **unchanged.** The added package is the in-repo `open_pipe_stress_result_export`. Its dependencies (serde_json with `float_roundtrip`, canonical_json, sha2 0.10, units) are already in PP's closure with the same features, so feature unification adds nothing |
| Downstream locks | — | the six lock files ROOT updated in `bee3dc07ca` (runner/headless, the desktop Tauri app, self_weight_wasm, operation_applier, the two validation benchmarks) gain only the in-repo edge. No registry package or version changed (RR "U3 grant 1 verified", R-3) |

**What the re-pin changes in D-6's design** (BUILD.md):

1. **The registered lock hash** becomes `4f494db6…475b`. BUILD.md §1's statement "The lock will change at U3" is now discharged: it changed at U1 and did not change at U3. G6 registers this hash.
2. **The linked set.** `result_export` now links into PP's production build.
   - Its code is under PP's build identity: the same compiler, target, profile and opt-level keys.
   - It has no build script.
   - Its `serde_json` is the same crate instance, so the `Value`, `Number` and `ErrorImpl` layout witnesses (BUILD.md §3) cover the reader's Values.
   - **Add layout witnesses** for the reader types the T17 formulas use: `ValidationError`, `Validation` and `RowClassification` (the `s(Validation)` and `s(RowClassification)` atoms), and the `HashSet`/`HashMap`/`BTreeSet` entry tuples of the preview-evidence and G3–G6 working sets.
3. **The reader's 13 static inputs** are compiled in with `include_str!` and parsed into `OnceLock`s on first use. Their bytes set T17's statics term (5,397,696 B illustrative), so **the reviewed-lock record should list their SHA-256s**. They are all in ORIGINS.json `statics`; for example retained_precision_mp_v2.schema.json is `07951eda…`.
   - A change to any of them is a source change, and so a new build identity.
   - G5 can also turn their Value facts into compile-time witnesses, by measuring the parsed facts of each `include_str!` in a const context or in a test that pins them.
4. **The residual is unchanged** (BUILD.md §2.3). A foreign workspace that builds PP with a different serde_json patch is not mechanically detected. The six downstream locks pin serde_json 1.0.149, except runner/headless at 1.0.151. Headless stays outside D1 (D-2; `admit` refuses Headless, U3 F-5).

## 4. Basis and what changed under the grant

- **G4 was granted on NUM `3260d7809e`** (U1 merged). During the grant, ROOT merged U3 grant 1 (`bee3dc07ca`) and its records into NUM (`b1f80234dc`).
- **G4 pins `b1f80234dc`** and reads it from a `git archive` snapshot in WT/scratch. The analysis therefore covers U3's committed dispatch, frozen-candidate split, staging copy, precommit call and reserved-stack thread, not a design guess.
- **U3's code is committed,** so deriving from it is allowed. No uncommitted code was read: I61's working tree and its uncommitted records were not opened.
- **G3's rules were line-mapped** `5ae5fe4f0f` → `3260d7809e` → `b1f80234dc` (`linemap_*`).
- **NUM moved again during G4.**
  - ROOT merged U3 grants 1b and 1c at `a634ac8b53`; the NUM head at return is `61a474dd4e`, records only, with grant 1d test-only on the facade branch.
  - G4's arithmetic stays pinned at `b1f80234dc`.
  - Grants 1b and 1c change four PP source files and no Cargo file, so §3's lock facts hold at `a634ac8b53`.
  - TRANSFER_COMPLETION.md §7 checks them against B-1 to B-10, and all are met.
  - The one figure the check corrected is s(output), from 1,024 to 2,048 B illustrative. It was already low at `b1f80234dc`. It moves T19 to 10,240 B and leaves the admission maximum unchanged.

## 5. Carried to G5

**Routed by ROOT** (RR "RV83 confirmation of the G2 repairs"), recorded here unchanged:
1. The build script needs `cargo:rerun-if-changed=src/build_identity.rs`.
2. An empty environment variable is a value, not a read failure. `target.env` is empty on Apple.
3. RV83's `collect::<String>` at lib.rs:1757 (RV83's citation).
4. Under D-2, `admit` must refuse Headless (U3 F-5; API_G4.md §1).

**Raised in G4:**
5. **D1.10 and D1.11 census readers.** Add the `control_bytes` counter and the provenance first-byte test (§1).
6. **Reader layout witnesses.** Cover `Validation`, `ValidationError`, `RowClassification` and the reader's set and map entry tuples (§3).
7. **Static-input hashes.** Bind the reader's 13 static-input SHA-256s in the reviewed-lock record (§3).
8. **The deepest schema chain** (36): either the W1 witness records it or a reader witness exercises it (PUBLICATION_READER.md §3).
9. **`PhaseFact`.** Define the enum, plus the gate facts of API_G4.md §2.
10. **The N1 append.** Pin it with a reserve-capacity assertion under the fault controls (TRANSFER_COMPLETION.md §4).
11. **The admission constant.** Re-evaluate every G4 expression at the in-build strides, under whichever margin option ROOT selects (COMPOSITION_G4.md §6).
