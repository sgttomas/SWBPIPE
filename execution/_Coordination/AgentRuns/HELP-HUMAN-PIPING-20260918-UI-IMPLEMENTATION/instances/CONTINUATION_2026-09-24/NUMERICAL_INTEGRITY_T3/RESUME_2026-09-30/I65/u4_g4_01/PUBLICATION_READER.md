# T16 publication, T17 precommit reader, and the U1 text part of T08

**Basis:**
- NUM `b1f80234dc`. U1's serializer is at retained_wire.rs; U3 grant 1 calls it through `serialize_frozen` (retained_wire.rs:1474) from `retained_w1` (PP lib.rs:2950–2998).
- Arithmetic: `_run_records/g4_caps.py`, with outputs `g4_caps.<which>.eps<e>.out.json`.
- Every count is at the D1 caps. Strides are illustrative.

**The hash route used here** is G3's route plus RV84 S-3 (G3_REPAIRS.md §4):
- the json! wrapper's deep copy;
- the to_string text, ≤ max(128, 2e);
- the checked-parse tree, pricing push-built arrays as ≤ 6 × slots, with seen keys cloned;
- the canonical text, ≤ max(8, 2e);
- the per-string to_string temporary, ≤ max(128, 2·(ε·L_max + 2));
- the parser's escape scratch, ≤ 2·L_max.

e ≤ ε·strings + keys + 8·values + 24·numbers.

## 1. The facts

| Value | Slots | Objects | Entries | String bytes | Key bytes | Numbers | e (ε = 2) |
|---|---|---|---|---|---|---|---|
| ENV_S: the successor envelope without its receipt | 68,420 | 17,845 | 103,007 | 103,306,441 | 1,916,909 | 2,495 | 209,961,095 |
| BODY: the receipt body | 23,752 | 18,724 | 56,951 | 30,180,344 | 1,822,432 | 16,898 | 63,234,304 |
| SOURCE: one CaseSource | 2,880 | 5,540 | 20,503 | 374,144 | 656,096 | 8,163 | — |
| INVOC: the invocation Value U3 builds | 32,768 | 16,385 | 16,386 | 131,136 | 131,136 | 16,384 | 1,179,864 |

**ENV_S** is G3's envelope Value at the G4 text atoms (D_env = 11,408; Text(diag_env) = 78,860,051; Text(row) = 11,474), plus `successor_envelope`'s deltas (retained_wire.rs:740–773): `recovery_method` on every case row, the selected diagnostic, and the identity and profile strings.

**BODY** is built member by member from `finish()` (retained_wire.rs:1436–1455), as count × per-record grammar read from each builder:
- `case_source` :896–972, `physical` :1002–1040, `logical` :1041–1090, `selection` :1130–1185;
- `proof_trace` and `product_attempt` :1238–1302, `invocation_arrays` :1383–1410;
- `material_basis` :857–890, `ordinary_value` :1412–1432.

**The counts:**

| Count | Bound | Source |
|---|---|---|
| physical records | ≤ 4 | FK origins.rs:194, :687 |
| logical attempts | ≤ 4 | |
| builds | ≤ 7 (one per origin slot) | |
| bound refusals per record | ≤ 3N | Uc and S per block, blocks ≤ F |
| bodies | ≤ n | |
| layout | ≤ Q | |
| projection outcomes | ≤ P_final | final_case.rs:1716, :1802 |
| conversions per member | ≤ 9 | product_certificate.rs:654 |
| fixed arrays | old_source 6, old_facts 7, section 5, normalization 3, inputs 10, stage names 10, entries 7 | |
| `diagnostic_refs` | ≤ D_env ids | |

**The per-record facts:**
- Keys are ≤ 32 B per entry; the longest receipt key is 30 B.
- Strings: bits 16 B, tokens ≤ 64 B, sha 64 B, input ids ≤ 128 B, result ids ≤ 1,024 B.
- Diagnostic ids are ≤ 2,330 B. That is the longest reached `diagnostic:` template's size bound in the text run.

**The milestone cross-check** (`which = milestone` against experiment 03's successor): every component of the bound is at or above the measured value.

| | Slots | Objects | Entries | String bytes | Key bytes | Numbers |
|---|---|---|---|---|---|---|
| BODY bound | 1,138 | 1,216 | 4,126 | 1,191,342 | 132,032 | 1,523 |
| BODY measured | 498 | 641 | 2,005 | 11,659 | 14,498 | 770 |
| ENV_S bound | 2,704 | 756 | 4,336 | 4,770,126 | 85,884 | 170 |
| ENV_S measured | 127 | 314 | 1,576 | 49,581 | 13,641 | 116 |

## 2. T16, publication (serialize_selected_from, retained_wire.rs:1500–1557)

**Live before T16 and carried through it:**
- the ordinary typed envelope (in O);
- the FrozenCandidate (T11–T15);
- the staged typed copy (T18.1; U3 lib.rs:2976–2978 drops it right after serialization).

**Locals:**
- the typed trace view;
- `bind_rows` (P RowBindings plus a map);
- `row_ids` (P Strings, at most 1,024 B each);
- `recipes`.

| Stage (live set at the render) | Caps | Milestone |
|---|---|---|
| P1: `to_value` of the envelope + successor deltas + the diagnostics push growth + locals + the source and its clone + `hash(source identity)` + the preparation payload | 181,768,169 | 8,073,933 |
| **P2: members + body `json!` copy + `hash(publication)` over ENV_S** (L_max = the integrity message, 2,549,385 B) | **1,389,829,540** | 64,547,467 |
| P3: members + body + `hash(body)` | 618,835,036 | 28,522,664 |
| P4: the `retained_precision` insert, a third body copy | 305,322,181 | 14,946,891 |

**T16 = 1,389,829,540 B (P2).**
- **Moving:** the publication text's last growth, e = 209,961,095.
- **Output:** the successor Value, ENV_S plus the receipt, 190,637,407 B. It moves into `RetainedSuccessor` (lib.rs:2997).
- **The fallback paths allocate nothing further:** `W1Fallback::Serializer(ReceiptFailure)` holds only `&'static str` and enum values.

## 3. T17, the precommit reader (result_export `retained_precision::validate`, :4263–4313)

**Called** at lib.rs:2992 with `&successor` and the invocation Value `json!({"request": raw, "solver_mode": …})`. That Value is a deep copy of the captured raw request, 15,782,080 B at the caps. It is built at :2989 and dropped at :2995.

| Stage | Caps | Milestone |
|---|---|---|
| V1: G1 `hash(receipt body)` | 368,440,399 | 16,438,539 |
| **V2: G1 `public = source.clone()`, then `remove("retained_precision")`, then `hash(publication)`** (:591–603). The clone peak is the whole successor; the hash peak is ENV_S plus the route | **1,275,143,949** | 58,600,408 |
| V3: G1 source-identity and preparation hashes | 33,395,612 | 1,404,348 |
| V4: G3–G6 working sets: row and diagnostic id sets, preview-evidence maps (P), classifications, the prescribed set, and the g5c `absolutes`/`uncovered` with their `json!` copies | 12,442,149 | 560,269 |
| V5: G7 `project()` clone + `for_source` → `validate_preview_physics_evidence` | 193,966,444 | 9,130,038 |
| V6: G8: projected + `hash(invocation)` + the model rebuild Values and their `json!` copies + the K4SRC/K4STF identity buffers | 210,247,486 | 49,289,874 |

**T17 = V2 + the Validation and classifications output = 1,275,668,333 B.**
- **Moving:** the publication text's last growth again.
- **Carried into the phase:** the successor (190.6 MB), the invocation Value (15.8 MB) and the statics.

**`integral_receipt` is borrowed, not cloned** (:251–289). The producer's `safe_integers` (retained_wire.rs:1310, a hard Encoding failure) guarantees that the body carries no float. The wrapper adds only strings, so `floats(retained_precision)` is false. The reader's own walkers (`shape`, `encoding`, `negative_zero`, `objects`, `located`, `exact_work`) do not allocate in proportion to the counts.

**The 13 `OnceLock` statics** (RV83 G2 N-13, routed to T17) are parsed on first use, on the reserved thread. They are **5,397,696 B**, counted in full:
- physics_source_recovery and retained_precision_mp_v2 schemas;
- the definition fixture;
- the retained, v0_2, precision, physics, load_reference, load_reference_source, preview_physics, physics_source and source_blocks contracts;
- the source_block_recovery schema.

Each is priced from its committed file's exact Value facts, with push-built arrays at 6 × slots. The file hashes are in ORIGINS.json. Their heap is process-lifetime (D-6, NOTES_G4.md §3).

**Which reader arms run.**
- `project()` sets the projected envelope's contract id to the literal preview-physics-1 (:4243–4245), so `for_source` takes only that arm.
- The physics, load-reference, physics-source and source-blocks readers are zeroed by edge rule (`loop_bounds.g4.json` edge_zero), each with that citation.

**The reader's schema walk.**
- The `$ref` graph is acyclic: 1,383 nodes, longest same-value chain 6, total recursion from the root ≤ 36, value depth reached ≤ 15 (`schema_depth.out.json`, unchanged at `b1f80234dc`).
- G3's open item asked whether the W1 witness reaches the deepest chain. The milestone receipt exercises the selected-case schema branches. **G5 must confirm** the deepest chain with the witness's recorded max recursion depth, or add a reader witness at depth 36.

## 4. The U1 text part of T08

The serializer, the capture changes, U3's dispatch and the reader are all on the Direct path at `b1f80234dc`, so they are inside the G4 text run (G3_REPAIRS.md §2). Their shares of TAV at the caps:

| Part | Sites with positive multiplicity | Bytes |
|---|---|---|
| retained_wire.rs (serializer): `bits` 14,497 × 32 B, `sha_hex` 9, the selected-diagnostic id, the case id, and two short copies | 6 | 465,938 |
| retained_product.rs (capture, with U1 and U3 deltas) | 156 | 593,234,896 |
| PP lib.rs U3 dispatch region (:2228–3000) | — | 11,857,176 |
| result_export (the reader) | — | 276,447,390 |

**How the reader's text is bounded.**
- **Error-path allocations** (`error()`, `require()`, `text()` on failure) are bounded per validate call by `site_from` rules:
  - one terminal error, because every failure returns through `?`;
  - plus g5_native's `tw()`, which swallows only `sum()`'s WORK_MISMATCH: ≤ 4 records × 64 amounts.
- **The largest reader site** is numeric_cases :2622: one `entity_ref` copy per body per row, n·P.

**The value copies the serializer makes** (`json!(String)`, `to_value`) are Value strings. They are priced inside the trees above, not in TAV; the `format!` that produced each one is in TAV.
