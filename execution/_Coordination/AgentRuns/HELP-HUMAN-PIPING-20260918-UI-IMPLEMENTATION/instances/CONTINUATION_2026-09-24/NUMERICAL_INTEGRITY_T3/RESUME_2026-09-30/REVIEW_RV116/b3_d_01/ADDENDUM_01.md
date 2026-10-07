# RV116 (RV-D) ADDENDUM_01: confirmation of I96's B3-D REVISION_01

TASK (Type 2), RV116, holding RV-D for B3, for ROOT (HELP_HUMAN, Agent 0), who is the return path. I made no delegation. 2026-10-07 UTC. This answers the coordinator's request after RR "RV116 (RV-D) accepts B3-D with amendments; B3-D ruled; I96 revises the draft statics" (RR sha256 `5bdd3a8c…0a36`, the same bytes I96 read).

**My review:** `REVIEW.md` in this folder, sha256 `001b7a329c6bf20dd268011bd677fe6e01bba7de7418e796794b51528d37f51d`. It is untouched, and its `SHA256SUMS` still verifies 8 of 8.

**The subject:** `R/I96/b3_d_01/REVISION_01.md`, sha256 `6f5b1a6d48157a73f167b96ec6a2f35c72a6ae66dfe1f3d56655e5a44203e542` (verified), at NUM `173f8778c9`. NUM's `P/core`, `P/fixtures`, `P/schemas`, `P/apps` and `P/tests` still equal main `2007709549`.

**Method.** Documents and code reading, plus read-only Python with VENV in `WT/scratch/rv116_rvd_01/`:
- I96's two generators, from my own copies;
- one new script of mine (`addendum_01/rv116_r1_checks.py`), which uses my sealed canonicalizer.

No cargo, lock, install or Git write. `addendum_01/RUN_ADDENDUM.md` has the commands.

## Verdict

**CONFIRMED.** REVISION_01 carries every amendment as ruled, and its regenerated statics reproduce byte for byte. **One residual NOTE (NA-1), cosmetic.** No new public meaning beyond RR's rulings.

## 1. S-2: DEF-E's evidence wording (REVISION_01 §1). Confirmed

My independent comparison of the committed v0 and r1 definitions finds **exactly five changed paths** (`rv116_r1_checks.out.json`):
- `/evidence/pipe_sections`
- `/evidence/pipe_stress_extrema`
- `/evidence/unchanged`
- `/rows/maximum`
- `/scope/excludes/4` (`pressure_regions` → `nonempty_pressure_regions`)

The preparation sub-object still equals DEF-O's.

**The wording differs from my illustration, but it carries the meaning I required:**
- **Scope.** Regeneration is confined to "the owner case's contract_evidence.exact_cases entry (matched by load_case_id) only". The `[]` path that read as "every case" is gone from `evidence` and from `rows.maximum`.
- **Complete coverage.** "(complete)" now attaches to the owner case's `stress_maximum_coverage` only. An unselected case's coverage no longer has to be complete.
- **Unselected cases.** "the entry of every case that is not selected byte-identical to the ordinary exact envelope" states the mixed-successor property in the static.
- **Several selected cases.** With more than one, each attempt is its own owner's formation (`scope.owners: load_case`), so each regenerates only its own entry. Nothing in the text conflicts with that.
- **The rename** fits beside `scope.pressure` ("explicitly_empty_pressure_regions_and_no_pressure_primitives"), and `nonempty_pressure_regions` has 0 hits outside `P/execution`.

## 2. The regenerated hashes. Confirmed, byte for byte

**I96's generators, from my own copies** (`addendum_01/regeneration_r1.txt`):
- `b3d_statics.py` (`513a8b82…`) and `b3d_statics_r1.py` (`dcd8e20d…`), run twice;
- every output equals `statics/r1/` byte for byte, and the out file equals the record's.

| Static | sha256 / H | Result |
|---|---|---|
| DEF-E raw (9,733 B) | `71f63d3916fa37ad0021ffb6ad993760a274166fe7ef275d7435c6856ed5642e` | MATCH |
| XTABLE (51,163 B) | `c4987e874889645ac315b5f55f58690082ad5e7745527f20e3e316efa3e70a3d` | MATCH |
| `SCHEMA_ENUM.diff` | `b1597c7b…f39e` | MATCH |
| `CARRIER_PROFILE_ENUMS.diff` | `64894ae4…5b80` | MATCH |

The v0 rerun still equals the unchanged top-level `statics/` (`6edae5ff…`, `5bf0d0dc…`, `165c9d74…`).

**My own canonicalizer, on the committed bytes** (not I96's `json.dumps`):
- **DEF-E r1:** canonical, ASCII, raw sha256 `71f63d39…642e`, and **H = `5a3bac430df9bbc77484d5419c75880ad40ae209b439e5f928374458025281af`**. The control is DEF-O's pinned `a7ed7ca0…0349`, and v0's H is `9b66492e…` as before.
- **XTABLE r1** differs from v0 in exactly one line, `product_formation_definitions[0].sha256` (`9b66492e…` → `5a3bac43…`). Its binding equals DEF-E r1's H, `formation_warrant` names the exact id, and its inherited hash still equals P1TABLE's raw sha256.
- **The new hash prefixes** `71f63d39`, `5a3bac43` and `c4987e87` have 0 hits outside `P/execution`.

## 3. N-9: S-C's TS exposure by export (REVISION_01 §8). Acceptable as ruled

**The ruling's intent is met by construction, with less change.** The ruling required that TS's exposure carry the setup (:320–328) and the thermal check (:358). Exporting the existing `validateAuthoredCaseFacts` (`physicsSourceRecovery.ts:301`) unchanged does that:
- its body is exactly the setup: request materials, the units engine, `convert`, `normalizeMaterial`, `normalized` and `validPair`, at :302–328;
- then the per-`exact_cases` loop (:329–360), including the `recovery_method` thermal check at :358;
- it reads `contract_evidence.exact_cases` through `physical(source)`, which on the successor is physics-1's evidence;
- the base call (:294) is unchanged.

**It cannot change a physics-source-1 outcome:** it adds an `export`, not code. That makes it a strictly smaller change than extraction, so S-C's gate (physics-source-1 suites and corpus identical) is trivially met in TS.

**RS and PY as stated.** RS changes `actual_materials` (:896) to `pub(crate)`. PY imports `_actual_materials` (:409) and `_canonical_inputs` (:311), mirroring `validate_physics_source` (:433–436). The PY helper is a necessary addition I had not named; it is correct.

**One parity observation, not a finding.** TS's function lacks RS's and PY's explicit `ACTUAL_MATERIAL_COVERAGE` length check. That is a base-reader parity point, and it is unreachable on the successor, because G7's physics-1 member-coverage check precedes G8.

## 4. S-1, N-4 to N-8, N-11 and B3-K (§2–§7, §9, §10). Confirmed

| Item | REVISION_01 | Against my finding and RR |
|---|---|---|
| S-1 (§2) | The route's table-bound H in the preparation payload. The four sites, with constants and lines (PP `retained_wire.rs:1306`, `:41–42`; RS `:487`, checked `:612–625`; PY `:364`; TS `:170`). P-9 and the G1 row amended. 07o entry 11 → G1 `RETAINED_PRECISION_RECEIPT_MISMATCH`, with SC2's rehash note | **Matches** ruling 1 and S-1. The code spelling is right: RS's `error()` prefixes `RETAINED_PRECISION_` |
| N-4 (§3) | Type-strict predicates for L, L3 and E. The full admission sets (PY `{}`, `[]`, `""`, `0`, `false`; TS `""`, `0`, `false`; RS absent and `null`). `pressure_contract: false` added on a 0.2.0 base; the siblings optional | **Matches** ruling 3 (B3D-10) and N-4 |
| N-5 (§4) | **The G5c note:** no reader implements item 3, and the exact branch does not evaluate it. **G8's deterministic order** (steps 1–9). **32 entries.** Entry 27 is correctly placed at G8 step 4, not S-C, and entry 28 is the S-C-only mutation. Entry 22 (a combination) depends on B2-C's G3 placement, stated honestly | **Matches** ruling 5 (N-5). **I checked entry 28:** for ν = 0.25 ± 1 ulp, RN64(1+ν) stays 1.25, so physics-1's G binding is unchanged (not merely within 2 ulps), and only S-C refuses. **Entries 1, 2, 15, 17 and 29** agree with the gate code (preview G0's identity-and-profile test; G5b `actual_radius`; physics-1's `MATERIAL_G_BINDING` > 2 ulps; N-6 at G8 step 6). The unselected-entry limit is stated correctly |
| N-6 (§5) | Recommended. G8 step 6, after S-C; entry 29; DEF-E's text unchanged | **Matches** ruling 5 ("optional; I96 recommends"). Placing it after G7, so physics-1's own refusal stands at three ulps, is right |
| N-7 (§6) | `CARRIER_PROFILE_ENUMS.diff` for results and stress-neutral; `analysis_run` has no profile enum | **Matches.** Applied to main, each diff changes exactly one path, its `formulation_basis.profile_id` enum, by appending `exact_straight_retained_w1a_v2`; no other enum changes. Patched sha256: `952b1deb…` and `05a588ed…`, as stated. `analysis_run.v0.3.schema.json` has no profile enum (0 matches) |
| N-8 (§7) | `title` and `$comment` set once. `OperandPreparation.definition_id` never admits the exact id (KD §4's `operand_definition` is DEF-O's). The J1 merged text proposed | **Matches** ruling 5 (N-8). Applied to main, the diff changes exactly `title`, `$comment` (the first sentence only; the rest byte-identical) and `ProductAttempt.definition_id` (`const` → `enum [ordinary, exact]`). Patched sha256 `0d5bb812…`, as stated |
| N-11 (§9) | An abandoned-W1 exact pin through existing hooks: physics-1 plus the N1 notice(s), accepted by physics-1's base readers in all three; a refusal there is a stop | **Matches** ruling 4 |
| B3-K (§10) | K3-1; K3-2 by option (a), with the three-line exception and its hashes; K3-3 with SA-2's and NA-6's controls; NA-5's J2k acceptance; SA-1; NA-3; N-10's citation | **Matches** RR "RV115's addendum accepts B3-K …". The fixture's current sha256 is `8cbe5d32ea21a3ce…a39a37`, as stated |

**§11 and §12.** N-1 to N-3, N-10 and N-12 are folded in as ruled. The estimates are plausible: +3–4 h for B3b, to 53–83 h.

**No new public meaning.** I found nothing beyond the rulings:
- SCHEMA's new `$comment` sentence is descriptive, and accurate (SCHEMA has no route branches);
- §7's merged J1 text is a proposal;
- §4.2's G8 order makes DESIGN.md §6.2 deterministic without adding a check, apart from N-6's recommended one.

## 5. The r1 layout. Confirmed

- **Both sum files verify:** `SHA256SUMS` 11 of 11, and `SHA256SUMS.revision_01` 20 of 20, covering every file but itself.
- **v0 is unchanged:** the three v0 drafts are at their original paths with their original hashes, and the v0 generator still rebuilds them.
- **`statics/r1/` holds exactly the four J1 statics:** the definition, the table, `SCHEMA_ENUM.diff` and `CARRIER_PROFILE_ENUMS.diff`. RUN_R1 R1′ moved the scratch-only SCHEMA copy and the out file back out.
- **`CARRIER_PROFILE_ENUMS.diff`** states exactly the two enum appends, and nothing else.
- **`SCHEMA_ENUM.diff`** states exactly the `definition_id` enum plus N-8's ruled `title` and `$comment` text, and nothing else (NA-1).

## Residual finding

| ID | Severity | Finding | Action |
|---|---|---|---|
| NA-1 | NOTE | `statics/r1/SCHEMA_ENUM.diff` keeps its v0 name, but it now also carries N-8's `title` and `$comment` changes, as ruled. The content is right; only the name is narrower than the content. | None needed. I-A should read it as B3b's whole SCHEMA change, not an enum-only change, when merging with B2-C at J1 |

## Records

| File | sha256 |
|---|---|
| `addendum_01/rv116_r1_checks.py` (mine; imports the sealed `evidence/rv116_checks.py`) | in `SHA256SUMS.addendum_01` |
| `addendum_01/rv116_r1_checks.out.json` | in `SHA256SUMS.addendum_01` |
| `addendum_01/regeneration_r1.txt` | in `SHA256SUMS.addendum_01` |
| `addendum_01/RUN_ADDENDUM.md` | in `SHA256SUMS.addendum_01` |

**Read for this addendum:**
- REVISION_01 (whole) and RUN_R1;
- `b3d_statics_r1.py`;
- RR's new section;
- TS `physicsSourceRecovery.ts:279–360`; PY `physics_source.py:308–440`; RS `physics_source.rs:1125–1140`; RS `retained_precision.rs:206–224` (code prefixes);
- KD §4's `operand_definition` row;
- the oracle fixture's sha256.

**Limits:** nothing was compiled or run. REVISION_01 §4.3's first failures stay derived from code; SC2's three-reader runs establish them, as REVISION_01 states.
