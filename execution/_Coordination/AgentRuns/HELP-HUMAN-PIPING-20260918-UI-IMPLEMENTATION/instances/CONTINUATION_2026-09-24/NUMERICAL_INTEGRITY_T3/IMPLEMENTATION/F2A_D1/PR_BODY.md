**piping(T3): F2a D1 milestone: retained-precision successor through the Direct entry, with reader eligibility**

This PR brings T3's first retained-precision (F2a) code to main.

The synthetic skew cantilever RF-SKEW-T-CANT-OFF-122-r1e-04 now publishes an M03-INTEGRITY-MP-v2 successor:
- through the product-physics Direct entry, in both solver modes;
- in the registered dev/test build only;
- in agreement with its independent reference.

The Python, Rust and TypeScript readers now treat such a statement as numerically eligible when it is supplied with its actual invocation. In TypeScript, the live native capture must also hold (a declared difference).

**This is an intermediate F2a PR. It does not complete F2a,** and it adds no product caller.

## What is public

- **The Direct retained entry** publishes a successor only for in-domain (D1) requests: one load case, no combinations, the preview family, no pressure.
  - It does so only in the registered dev/test build: aarch64-apple-darwin, rustc 1.97.1, debug, no RUSTFLAGS.
  - M = 4,026,531,840 B, selected under D-7.
  - Every other request keeps the ordinary route. A request whose retained work ran and fell back carries one unavailable notice.
- **The Headless entry** is refused.
- **Reader eligibility** in all three languages. **The carriers** transport the successor, and the desktop export panels refuse it explicitly.

## What stays closed

- **Public activation.** No desktop, native or CLI caller publishes successors. The desktop calls only the ordinary wrapper. Activation has its own checklist and a fresh review (CHANGE_RECORD §4).
- **Every other build is Stale and keeps the ordinary route,** including release builds and hosted Linux CI. **Hosted CI therefore exercises only the ordinary route.** The registered path is evidenced on the owner's Mac (DEC-025, Pass B, the controls).
- **No supported-machine statement of M** is made; that stays with the owner.
- **Still open:** U8's deferred witnesses, the rest of F2a, S-I, F2b per domain and F3, and the owner-held items (CHANGE_RECORD §5).

## Source and packaging

- **Source.** 136 maintained files under `projects/chirality-piping` (59 added, 77 modified). They are byte-identical to the reviewed integration head `2e03d7cc25` (NUM `4c876ac7e8`), except one file:
  - `core/analysis_runs/compatibility.py` keeps a single `_same_canonical` helper, byte-identical to main's #1078, and main's two call-site changes.
  - `source_equality.py` proves this.
- **This branch is cut from main as two commits:** the source snapshot, then the evidence package. It carries none of the integration branch's history. That history stays on `codex/piping-numerical-integrity-20260926`.
- **Records committed:** 10 files, 184282 B, all in `…/NUMERICAL_INTEGRITY_T3/IMPLEMENTATION/F2A_D1/`.
  - **Why:** the change record, the two verification scripts, the citation index, and four small records that maintained comments depend on. One of them is the generator that rebuilds the memory profile block.
  - The integration branch's 5,677 record files (191.4 MB) are not brought. Maintained-source citations of them resolve through `citations.json` to commit-pinned URLs.

## Reviews

Every unit had a fresh independent review, and each repair was confirmed by the same reviewer:
- U1–U2: RV82;
- U3: RV85 and RV93;
- U4: RV83, RV84, RV87 and RV89;
- U5: RV86;
- U6: RV88, RV90, RV91 and RV92;
- U7: RV94;
- earlier components: RV68, RV72, RV74, RV77 and RV78–RV81.

The complete PR diff is reviewed by **RV95** ⟨verdict⟩. These are agent reviews, not personal review by the owner. Details: [CHANGE_RECORD.md](`…/NUMERICAL_INTEGRITY_T3/IMPLEMENTATION/F2A_D1/CHANGE_RECORD.md`).

## Gates

| Gate | Result |
|---|---|
| Hosted CI and full-SHA dispatch | ⟨run IDs⟩ |
| Mac DEC-025 (only the 3 known platform failures) | ⟨result⟩ |
| GEN-8 | ⟨result⟩ |
| T9 (112/112 plus 2 added outputs) | ⟨result⟩ |
| Both-entry gate, parts 1 and 2 | ⟨result⟩ |
| src-tauri suite | ⟨result⟩ |
| Full Pass B | ⟨result⟩ |
| Pressure and coexistence controls | ⟨result⟩ |
| Native witness | ⟨result⟩ |
| `source_equality.py` and `check_citations.py` | ⟨result⟩ |

🤖 Generated with [Claude Code](https://claude.com/claude-code)
