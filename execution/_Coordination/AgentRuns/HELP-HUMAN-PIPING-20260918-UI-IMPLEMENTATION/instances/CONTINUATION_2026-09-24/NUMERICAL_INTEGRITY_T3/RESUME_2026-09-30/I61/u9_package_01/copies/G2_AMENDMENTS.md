# G2 amendments (RV83 dispositions, RR "RV83 on U4 G2: FAIL")

**Each section below replaces the cited G2 text.** The G2 packet `R/I65/u4_g2_01/` stays byte-for-byte unchanged, as sealed evidence; where it and this file disagree, this file governs.

| Finding | Replaces | Section |
|---|---|---|
| B-3, S-5 | BUILD.md:30–72 (§2.1–§2.2) | §1 |
| S-3 | DOMAIN.md:22 (clause D1.9) and §2 | §2 |
| S-4 | DOMAIN.md:16 (clause D1.3) and :38 (cap row "sections") | §3 |
| S-1 | RESIDUALS.md:97–144 (T07) | RESIDUALS_G3.md §T07 |
| S-2 | RESIDUALS.md:195–212 (T22); D4_RECONCILIATION.md:42, :93 | RESIDUALS_G3.md §T22 |
| B-1 | RESIDUALS.md:142 ("Remaining: none"); DOMAIN.md:3 ("inside the source paths I54/RV75 priced") | RESIDUALS_G3.md §T25 |
| B-2, S-6 | RESIDUALS.md:148–193 (T08) | TEXT.md |
| N-1 | RESIDUALS.md:83 (T06 citation) | §4 |
| (new) | DOMAIN.md D1.7 (load fields) | §5 proposed D1.10 |
| (new) | DOMAIN.md D1.9 (text caps) | §6 proposed D1.11 |
| N-12 | DOMAIN.md:43 (depth convention) | §2 |

## 1. The D-6 build-identity check (replaces BUILD.md §2.1–§2.2; B-3, S-5)

**The defect.** Cargo reads build-script output line by line, so a newline-separated value in one `cargo:rustc-env` directive keeps only its first line. Separately, `env!` would be a compile error wherever the variable is absent.

**The encoding.** One single-line value, `OPS_RETAINED_BUILD_IDENTITY`:

```
v1;rustc.release=<v>;rustc.commit=<v>;rustc.host=<v>;rustc.llvm=<v>;target=<v>;target.arch=<v>;
target.pointer_width=<v>;target.endian=<v>;target.os=<v>;target.env=<v>;panic=<v>;profile=<v>;
opt_level=<v>;debug_assertions=<v>;rustflags=<v>;pkg=<v>
```

It is shown wrapped here; the actual value is one line with no spaces outside escaped values. The keys and their sources are as in BUILD.md §2.1's table.

**The escaping rule** (`%`-encoding, applied to each value's UTF-8 bytes):
- `%` becomes `%25`, `;` becomes `%3B` and `=` becomes `%3D`;
- every byte below 0x21 or at or above 0x7F (space, control characters, DEL and every non-ASCII byte) becomes `%XX`, with uppercase hex;
- every other byte passes through unchanged.

So the value is pure printable ASCII with no spaces, and splitting on `;` then on the first `=` is unambiguous.

**One encoder, three users.** The encoder lives in one source file, `PP/build_identity.rs` (G5), with no dependencies:
- `build.rs` uses it through `include!("src/build_identity.rs")`;
- the crate uses it as a private module;
- the G6 registration texts are produced by the same function, by printing the encoded identity of the qualified build from a G5 test.

**The build script** emits exactly one directive per build configuration, `cargo:rustc-env=OPS_RETAINED_BUILD_IDENTITY=<encoded>`, together with the `rerun-if` lines of BUILD.md §2.1. It never panics and never fails the build. If it cannot read `$RUSTC -vV` or any key, it emits `v1;unavailable`.

**Reading and comparison** (in `retained_memory.rs`):
- `option_env!("OPS_RETAINED_BUILD_IDENTITY")` is read; if it is `None` the value is treated as absent.
- With no profile registered, the status is `ProfileStatus::Missing`, as today.
- With a profile registered, the status is `Registered` only if:
  - the value is present;
  - it is byte-equal to one entry of `REGISTERED_BUILDS`;
  - `LAYOUT_WITNESSES` holds.

  Otherwise it is `ProfileStatus::Stale`: absent, `v1;unavailable`, unequal, or a false witness.
- Every outcome other than `Registered` refuses the permit (D1.1), and the ordinary path runs.

**G5 tests:**
- `identity_carries_every_key_in_order` decodes the compiled value and asserts:
  - the first token is `v1`;
  - the keys equal the canonical list exactly, in order, with no extras;
  - every value decodes, and re-encoding it reproduces the token.
- `encoder_roundtrip` checks every byte value 0x00–0xFF through encode and decode, and that the output never contains `;`, `=`, space or a control byte.
- `absent_identity_is_stale` checks that the comparison function maps `None` to `Stale`. Missing variables cannot be injected into a real build, so the function is tested directly.

**What the identity still does not bind** (from RV83 N-9; stated again in the G6 record):
- lto, codegen-units, overflow-checks and debuginfo, which change frames but not layouts;
- the maintained source revision; formula drift is caught by review and G6;
- `unbounded_depth` and `raw_value`, which have no layout witness;
- the consumer lock, which stays a reviewed record.

## 2. Typed capacity caps (adds to DOMAIN.md D1.9 and §2; S-3, N-12)

D1.9 is extended with typed capacities. Each is read from the actual object by the T03 census; nothing relies on the construction history.

| Typed owner | Capacity cap |
|---|---|
| every typed `String` | capacity ≤ 128 bytes (and length ≤ 128, as before) |
| `model.nodes`, `pipe_segments`, `supports` | capacity ≤ 32 each |
| each `supports[i].restraints` | capacity ≤ 192, and Σ capacities ≤ 192 |
| `model.materials`, `request.materials` | capacity ≤ 4 each |
| each `temperature_points` | capacity ≤ 16 |
| `model.load_cases` | capacity ≤ 1 |
| `load_cases[0].primitive_loads` | capacity ≤ 192 |
| `model.components`, `combinations`, `sections`, `request_material_expansion_laws` | capacity 0 |
| `model.material_expansion_laws` | capacity ≤ 4; every element `Absent` |
| typed `serde_json::Value` (`project.units`) | its census facts within the raw caps of DOMAIN.md §2; it is a subtree of the cloned raw request, so the same caps apply |
| BTree property maps | none in D1 (no sections, §3) |

**The depth convention** (N-12). The census counts the root Value at depth 0 (`retained_memory.rs:111`), so "raw depth ≤ 16" admits 17 nesting levels. The bounds that use depth (STACK_INVENTORY.md) use 17.

## 3. No sections (amends DOMAIN.md D1.3 and the "sections" cap row; S-4)

D1.3 gains two conditions: `model.sections` is empty, and every `pipe_segments[i].section_ref` is `None`. The "sections ≤ 32" cap row is replaced by "sections = 0".

**Fit:**
- The milestone satisfies this: it has no sections and no `section_ref`.
- The L = 0 base (case 0 plus one memberless, fully restrained node) adds no member and no section, so it fits.
- The Ceiling witness is not built yet. If its construction needs sections, it is reported, not priced.

**The normalization phase that runs on every request** — `normalize_model_units` and `resolve_shared_sections` with no sections — is priced as T05 row O-N in ORDINARY.md.

## 4. T06 citation (N-1)

T06's heap-free argument covers `FK/wide.rs` **and its submodule `FK/wide/multi.rs`** (`pub(crate) mod multi`, wide.rs:126). wide/multi.rs's text sites are Display/Debug impls into a `Formatter`; they appear in TEXT.md, not in T06.

## 5. Proposed D1.10: no JSON-object provenance on a primitive load (for ROOT)

**Clause.** Every `load_cases[0].primitive_loads[i].provenance` is absent, or its first byte after any ASCII whitespace is not `{`. The census reads it from the borrowed typed request without allocating.

**Why.** `validate_applied_self_weight` (PP/lib.rs:2320) runs on every request. For each load it parses the provenance with `serde_json::from_str::<Value>` and reads `["method"]`. It enters the generated-self-weight validation module (self_weight.rs:833–937 and 558–760) only if `method` names a self-weight method, or if the load has an Element target, which D1.7 already excludes. A plain-text provenance cannot parse as an object, so no method is read and every load `continue`s at self_weight.rs:828–830.

Without the clause, a 128-byte provenance such as `{"method":"pipe_mass_per_length_times_explicit_axis_acceleration"}` fits D1.9. It would reach a validation path whose text and Value owners G3 would have to price per load. TEXT.md §2 lists the module's functions. With the final call graph, the same method run without this exclusion gives about 10.6 GB of text at the caps, and leaves the module's own loop headers unmapped.

**What stays priced.** The per-load parse attempt, a `serde_json::Error` of about 40 B, transient, is in T05 O-N.

**Fit.** The milestone's three load provenances are plain text (`invented_t3_p1_detection_input_no_library_data`). The L = 0 base adds no load.

**If ROOT declines:** the remainder is a per-load bound for the self-weight inspection closure, at most 2 levels of `retained_basis` recursion. That is about 1–2 h of source derivation.

## 6. Proposed D1.11: no control character in any input string (for ROOT)

**Clause.** No raw JSON string value or object key in the request contains a byte below 0x20 or the byte 0x7F. The census can check this while it walks the borrowed raw Value, without allocating.

**Why.** Text built from input strings is serialized to JSON several times on the ordinary route. The largest instance is T25's `hash(publication)`, a canonical rendering of the whole envelope. serde_json and canonical_json write a control character as `\u00XX`: 6 bytes for 1. With D1.11, the worst expansion of envelope text is 2× (`"` and `\` double). That cuts the publication text bound from 616 MB to 208 MB, and T25 from about 3.07 GB to about 1.44 GB (RESIDUALS_G3.md §T25). Without it, the selected-finalization branch does not fit M (COMPOSITION.md).

**Fit.** None of the milestone's 213 string values and keys contains a control character (checked in this grant). The L = 0 base adds only identifiers.

**If ROOT declines:** tighter caps (C, P or the text caps), or a lifetime-aware text and envelope model.

## 7. RV83 NOTE items (dispositions)

| NOTE | Disposition |
|---|---|
| N-1 | §4 above |
| N-2, N-3, N-4, N-5, N-6, N-7, N-10, N-11 | Confirmations. No change |
| N-8 | The R and k proposal stands. STACK_INVENTORY.md confirms that no count-proportional recursion exists on the W1 path |
| N-9 | Restated in §1 ("What the identity still does not bind"). The G6 record carries it |
| N-12 | §2 (depth convention: 17 levels) |
| N-13 | The result_export `OnceLock` caches parse on first use, on the scoped thread. Their heap belongs to T17 (G4); their stack belongs to STACK_INVENTORY.md |
