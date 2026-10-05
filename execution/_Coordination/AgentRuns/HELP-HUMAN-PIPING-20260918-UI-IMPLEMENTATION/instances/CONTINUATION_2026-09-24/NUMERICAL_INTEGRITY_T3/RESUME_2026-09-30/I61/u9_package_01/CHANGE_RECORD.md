# F2a D1 milestone: change record

**This PR brings T3's first retained-precision (F2a) code to main: the D1 milestone and reader eligibility.**
- **The milestone:** the synthetic skew cantilever RF-SKEW-T-CANT-OFF-122-r1e-04 publishes an M03-INTEGRITY-MP-v2 successor.
  - It publishes through the product-physics Direct entry, in both solver modes.
  - It does so only in the registered dev/test build.
  - The successor agrees with its independent reference.
- **The readers:** the Python, Rust and TypeScript readers can now treat such a statement as numerically eligible when it is supplied with its actual invocation.
- **Nothing publishes successors to users.** No product caller is added, every other build keeps the ordinary route, and public activation is a later, separate step (§4).

This is an intermediate F2a PR, not the completion of F2a. The obligations in §5 stay open by name. The review is agent review, not personal review by the owner.

**Status of this file.** It is a draft, keyed to the reviewed source head `e543c3d8f3` (the U7 branch `codex/piping-f2a-u7-20261004`). The integration branch is `codex/piping-numerical-integrity-20260926` (NUM). ROOT regenerates the figures marked ⟨cut⟩ at the cut and fills the gate blanks (§7).

**Notation:**
- **P** = `projects/chirality-piping`.
- **RR** = T3's `ROOT_RULINGS_V1.md`, which is append-only, so its line numbers are stable. An RR "title" names a ruling heading.
- **R/…** paths are T3's `RESUME_2026-09-30` records on NUM.
- **Records resolve through `citations.json`:**
  - every record path or RR line that maintained source cites maps to a commit-pinned GitHub URL on NUM;
  - four small files are copied into this package (§6).

## 1. What the PR contains

| | |
|---|---|
| **Maintained source** | 136 files under P: 59 added, 77 modified, 11,811,048 B; +204,652 / −896 lines against base `381be775ae`. They are byte-identical to the reviewed head except `P/core/analysis_runs/compatibility.py` (§2). |
| **This package** | 10 files, 0.2–0.3 MB ⟨cut: exact count and bytes⟩: this record, the PR body, `source_equality.py`, `check_citations.py`, `citations.json`, four copied records, `SHA256SUMS` |
| **Not included** | The integration branch's 5,677 execution-record files (191.4 MB). Under the handoff packaging rule and U9 decision 5, they stay on NUM. Gate bulk stays in the T3 scratch area, with hashes in the post-merge record |

**By area:**

| Area | Paths | What |
|---|---|---|
| Kernel (frame_kernel) | `core/solver/frame_kernel/src/structural/retained/**`, `retained_resource.rs` | Retained-precision arithmetic for the product: the directed certificate, product certificate (bridge, final case, source residual), origins, checked work, resource constants |
| Product facade (product_physics) | `src/retained_*.rs`, `build.rs`, `build_identity.rs`, `lib.rs` (the Direct and Headless retained entries) | The prepared producer, the receipt serializer, facade capture behind a linear permit, admission law and memory profile, build identity |
| Readers | `core/analysis_runs/retained_precision.py`, `result_export/src/retained_precision.rs`, `apps/desktop/src/features/results/retainedPrecision*.ts` | Ordered G0–G8 checks, standing and eligibility |
| Carriers | AnalysisRun (Python and TS), derivative, stress-neutral export, semantic contract, the desktop panels | The successor travels with its receipt; the export panels refuse it explicitly |
| Schemas and fixtures | `schemas/retained_precision_mp_v2.schema.json`, `results.v0.3` successor branch, the corpus (snapshot 07k), the carrier case file (v4), the milestone request and successors | The contract and the shared test corpus |
| Runner, harnesses | `core/runner/headless`, `performance_harness`, `numerical_robustness` | The headless retained entry (refused at D1.0) and the observation harness updates |

## 2. Main movement absorbed

Main moved past the base. Of main's changes, only `compatibility.py` overlaps this PR.
- **Main's PR1078** adds `_same_canonical` with a body byte-identical to this PR's. It also switches two copied-evidence comparisons to it.
- **Resolution (U9 decision 3):** keep one helper and take main's two call-site lines. `source_equality.py` checks 3 re-derives this resolution, and checks that main's helper appears byte for byte inside this PR's version.
- **Main's PR1080** (source-blocks row order) touches no file in this PR. Its effect is covered by the full Pass B and the suites (§7).

## 3. What changed and why, unit by unit

Commits are on NUM and the U7 branch. Reviews are fresh and independent, and each repair was confirmed by the same reviewer.

### Before step 4: the accepted components

- **The prepared private producer** (`922db9dce3`) passes the named case in both modes. Accepted after RV68.
- **Caller separation and the nonallocating census** (`24af17c470`). Accepted after RV72.
- **Typed preparation and proof evidence on failure prefixes** (`fc23cff95f`). Accepted after RV74.
- **The selected contracts:** corrected C3/F1 and wire completions 06–08 (RV69).
- **The kernel and certificate components** come from the reviewed component branch `codex/piping-f2a-work-exactness-20261002`.
- **The summary-coverage seam** (`c618675e84`, `fa225abded`). RV77: PASS, with its tests adopted. Merged at `f044127b1c`.
- **The three readers, the schema and corpus 07f.** Accepted after RV78–RV81 and six confirmation rounds (D1–D37). Fanned in at `c15e64b756`.
- **RV78's carried-artefact review** (the semantic table fixture and the YAML successor branch): PASS.

### U1 and U2: the receipt serializer, with the owner binding

- **Grant 1,** with U2's structural owner binding (RV77-N4): `59a5de2032`. **RV82: PASS.**
- **Grant 2,** closed translations, the unavailable form and U2's failure path: `b54caba7ab`. **RV82: PASS.**
- **Why:** the producer must emit the exact receipt the accepted readers validate, with custody bound to its owner.

### U3: facade capture behind the permit

| Grant | What | Commit | Review |
|---|---|---|---|
| 1 | The permit branch, unreachable | `bee3dc07ca` | **RV85: PASS** |
| 1b | Public carrier `RetainedPublication`, R-2's single unavailable notice, the linear permit | `4b31bbf23a` | **RV85: PASS** |
| 1c | Admission report kept, gate order, typed staging fallback | `886bef131a` | **RV85: PASS** |
| 1d | Test-only | `8abb5274a9` | — |
| 2 | The permitted path on the actual Direct entry | `664f8df7b7` | **RV93: PASS** |
| D-U6-5 | The carrier fixtures are the live successors | `f71478696b` | **RV93: PASS** (addendum, on `7f07a2f7b4`) |

**Why:** capture happens inside the one ordinary run, so the certified receipt describes what was actually published. Every fallback keeps the ordinary bytes.

### U4: memory profile, admission and registration (G2–G7)

| Step | What | Commit | Review |
|---|---|---|---|
| G2–G4 | Design | (records) | RV83: FAIL, repairs routed and confirmed. RV84: PASS and confirmed. RV87: PASS |
| G5 part 1 | Admission law, the D1 predicate, fail-closed build identity | `1e323058f3` | **RV89: PASS** |
| G5 part 2 | In-build profile, gate caps, budgets, stack witnesses | `cba3e9fda7` | **RV89: PASS** |
| G6 | Estimates closed, the identifier-class audit, the G-C solve fact | `2bb81ec1ea` | **RV89: PASS.** RV87: NOT CONFIRMED, then the class closed |
| G6 pre-registration repair | | `b43378d90a` | **RV89: PASS; RV87: CONFIRMED** |
| Registration | The dev/test identity: aarch64-apple-darwin, rustc 1.97.1 `8bab26f4f68e`, debug, opt-level 0, debug assertions, panic=unwind, no RUSTFLAGS. **M = 4,026,531,840 B, selected under D-7** | `0c7827b6ad` | (applied after the G6 reviews) |
| G7 | Re-qualification on the integrated basis; the T17_V4 line | `7f07a2f7b4` | **RV89: PASS** (Pass A and its addendum). RV87: PASS on the TEXT tool |
| Pass B | On the U7 head `cfda60403f` | — | **RV89: PASS** |

**Why:** no permit may rest on symbolic or partially priced terms.
- The admission law prices the invocation's maximum in the build: 0.8881 M sparse and 0.8929 M dense, within the 0.9 M margin.
- Every identity other than the registered one is Stale and keeps the ordinary route.

### U5: the independent reference

- **U5 is records only.** Against I50's exact-rational oracle, the milestone's successor agrees on 97 of 97 class claims in each mode. **RV86: PASS, with stated limits.**
- After U7, only the eligibility fields differ. The post-U7 report is the new baseline (RR "U7 slice L committed…").

### U6: carriers and standing

| Slice | What | Commit | Review |
|---|---|---|---|
| U6a | Rust dispatch, standing, derivative, Python entry | `844448112f` | **RV88: PASS** |
| U6b | Python carriers | `c89a7a986c` | **RV88: PASS** |
| U6c | Schemas | `cb03315779` | **RV88: PASS** |
| U6d | TypeScript carriers | `9555b6ffc2` | **RV88: PASS; RV91: PASS** |
| U6e | Reader round, snapshot 07g | `5e1e2625ac` | **RV90: PASS** |

**The repair rounds:**
- `cc4dd61d67` (07h; RV90 CONFIRMED);
- `968adb44fe` (RV91 CONFIRMED);
- `da274dd961` (RV88 confirmed every item but one, which was routed on);
- `76477534f6` (RV91 CONFIRMED);
- `6383e8e70e` and `b10ee5cf08` (post-U6f);
- `5f8d6291b8` (N-9).

**The whole of U6: RV92: PASS,** with its addendum PASS.

**Why:** the successor must travel with its receipt, and standing must come only from the verified receipt.

### U7: the reader eligibility switch

| Slice | What | Commit |
|---|---|---|
| T | TS standing bound to the live native capture; the explicit panel gates | `0ca5449c87` |
| P | Docs on `into_parts()` and `successor()` | `12a849a7bd` |
| F | The three completeness flags switched together | `cfda60403f` |
| Fix | TS summary counts Current only with eligible standing | `e5e1693ceb` |
| L | Live reruns in all three languages; corpus 07j closes the `not_required` gap | `ffe65ef203` |

**The repair round after RV94:**
- `fc575c7e56`: the stale "no permit" comments (U9 decision 6);
- `8c84e7ae14`: Python and Rust summaries count Current only with eligible standing (RV94 S-1);
- `e543c3d8f3`: D-U7-4's summary declared; N-2's docstring; N-3's G7 scope clause. This makes the corpus 07k (278 mutations).

**Reviews:**
- **RV94: PASS** on `ffe65ef203`. Its confirmation of the round is ⟨pending⟩.
- **RV89: PASS** on Pass B at `cfda60403f`.

**What eligibility means.** A supplied successor statement with its actual invocation stands `numerically_eligible`.
- **In TS,** it is eligible only while the live native capture holds (D-U7-4, a declared difference).
- **No producer-origin claim** follows from eligibility (D-U7-6).

## 4. Scope (U9 decision 1, D-U7-1)

**Public in this PR:**
- **The Direct retained entry** (`run_linear_static_preview_value_with_retained_direct`).
  - It publishes the successor only for D1 requests: one load case, no combinations, the preview family, no pressure, capped counts.
  - Only in the registered dev/test build, under M = 4,026,531,840 B.
  - Within D1, a request whose W1 work falls back keeps the ordinary bytes, plus one notice once W1 work has run. Every request outside D1 keeps the ordinary bytes exactly.
- **The Headless entry** is refused at D1.0.
- **The three readers' eligibility** and the carriers, as in §3 (U7).

**Closed, and stated as closed:**

| Item | Detail |
|---|---|
| **Public activation** | No product caller (desktop, native app, headless CLI) publishes successors. The desktop calls only the ordinary wrapper (D-U7-1) |
| **Stale builds** | Every other build identity, including release builds and hosted Linux CI, keeps the ordinary route. Hosted CI therefore exercises only that route. The registered path is evidenced on the owner's Mac only |
| **A supported-machine statement of M** | Owner-held |
| **Wider F2a, U8, S-I, F2b, F3** | See §5 |

**The public-activation checklist** (RR "RV94 on U7…"). Activation requires at least:
1. **RV94 N-1:** review and tests for the Tauri rule-check backend accepting a successor with its invocation (`src-tauri/src/lib.rs` `qualify_rule_mechanics_with_context`).
2. **RV92 N-7:** memoize the binding before native activation.
3. **Native Current for successors.**
4. **T6's successor outputs,** replacing the explicit N-5 panel refusal deliberately.
5. **Confirmation that RV94 N-4 is resolved** (by the S-1 alignment).
6. **A fresh review** of the activation itself.

## 5. Open obligations

1. **U8, the deferred witnesses:** the native Ceiling row, the L = 0 base, and RV93 N-5 (a real-input Candidate test).
2. **Wider F2a:**
   - the producer receipt and freeze transaction beyond D1;
   - resource and caller qualification for any new Direct caller, and re-qualification for any change to the D1 call graph (RR 10407, 10474; QUALIFICATION §11);
   - multi-case invocations and combinations; preparation-only and mixed invocations; the promised exact routes;
   - the complete invocation's exact-block no-attempt rule;
   - registration of the release identity;
   - D38's pin, RV78-N1, and alignment of the N-3 G7 codes.
3. **S-I, F2b per domain, and F3.**
4. **Owner-held:** dense and lane ceilings; PHYS-R4 refusal and availability; observation framing; the KF3 lambda split; the KF2 dense screen; any supported-machine statement of M.
5. **T6:**
   - the successor outputs replacing the panel refusal;
   - the `results.schema.yaml` v0.3 dispatcher, which knows only precision-1.

## 6. Package and verification

| File | Purpose |
|---|---|
| `source_equality.py` | The five equality checks (U9 decision 2) |
| `check_citations.py`, `citations.json` | Every record, design-document and code-line citation the PR adds resolves (U9 decision 5).<br>**On the reworded tree** (`e543c3d8f3` plus `R/I61/u9_citations_01`):<br>• **Totals:** 368 resolved, 0 ambiguous, 0 unresolved.<br>• **Records and RR:** 65 occurrences.<br>• **Design documents:** 289, against a table of 23 names. Anchors are verified in the pinned version; D1 resolves on main. `API.md` and `COMP` are decided by recorded context rules; COMP is ROOT's ruling.<br>• **Code lines:** 14, each pinned with its anchor text: 11 generated in the GENERATED PROFILE block and 3 in the hash-pinned corpus data. Every other bare `FILE:line` code citation is forbidden.<br>The 93 previously in comments now name their symbol. At `e543c3d8f3` itself the check fails with those 93 |
| `copies/g5_profile.py`, `copies/profile_tree.json` | The generator and input that regenerate `retained_memory.rs`'s GENERATED PROFILE block byte for byte at `e543c3d8f3`: `python3 g5_profile.py profile_tree.json retained_memory.rs` |
| `copies/QUALIFICATION.md` | G6's qualification, the basis of M and the registered identity |
| `copies/G2_AMENDMENTS.md` | D-6's identity encoding (`build_identity.rs`) |

**Commands, run from a checkout:**
```
python3 source_equality.py --repo . --pr <PR head> --int <reviewed head> --main <main> --work <scratch dir> --package <package path>
python3 check_citations.py --repo . --base <main> --head <PR head>
```

## 7. Gates (U9 plan §1)

| Gate | Expected | Result |
|---|---|---|
| RV95, complete review and confirmation | No unresolved BLOCKING or SHOULD-FIX | ⟨RV95 verdict, seal⟩ |
| Hosted CI, plus the full-SHA dispatch (`target_base` = main) | All selected checks pass on F | ⟨run IDs⟩ |
| Mac DEC-025 on F | Only the 3 known platform failures (PP `t13`, two runner `load_reference`); added tests named; PP Registered; pytest, wasm, vitest and build pass | ⟨SWEEP file, comparison⟩ |
| GEN-8 on F | 1 passed | ⟨log⟩ |
| T9 | 112 of 112 identical, plus 2 added (the milestone request, both modes), equal to the ordinary bytes; extra corpus 16 of 16 | ⟨result⟩ |
| Both-entry part 1 | 884 of 884 identical; gate_check PASS; 0 trusted breaches; 0 heap-cap aborts | ⟨result⟩ |
| Both-entry part 2 | 4 dense runs, each ≤ 1,800 s | ⟨durations⟩ |
| src-tauri suite | Equal to main | ⟨count⟩ |
| Pressure, no-pressure and exact-block coexistence controls; the grant-2 sweep | Sweep registered `9a74ff16…`, Stale `0e2db8b8…`; PHYS-R4 refused; no-pressure published | ⟨result⟩ |
| Full Pass B on the PR head (I65; RV89 confirms) | Entry byte-identical to `0c7827b6ad`'s; maxima 0.8881 / 0.8929 M; law 42/0 | ⟨result⟩ |
| Native witness (ordinary route, panel gates) | Unchanged | ⟨result⟩ |
| `source_equality.py`, `check_citations.py` | PASS | ⟨output⟩ |

**Acceptance runs on the reviewed source (U7 head after its repair round):**

| Suite | Result |
|---|---|
| Python, 24-file sweep plus retained suites | 1,848 |
| Python retained | 463 |
| result_export | 171 |
| vitest | 3,552; tsc clean |
| PP, registered and Stale | 705 passed, 1 failed (Mac `t13`) |
| runner/headless | 85 passed, 2 failed (known) |
| PP fixture sweeps (RV94's own builds) | Byte-identical: registered `9a74ff16…`, Stale `0e2db8b8…` |
| The switch's allocations on D1 | Identical in all 12 pairs (RV89) |
