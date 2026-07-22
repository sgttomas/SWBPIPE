# N4B Return — DEL-12-02 exact remediation

**Status:** `SUCCESS`  
**Blockers:** none  
**Fresh N5B review recommendation:** yes

N4B repaired exactly the five material N5 findings within candidate v6
section 6. Native-package exports now use the fixed
`DOTH-HANDOFF-002/downstream_tool` route; incomplete TypeScript payloads stay
unknown without a false-flags public shortcut; Rust null/`TBD`/`unknown`
values take precedence over private-name hints; renderer-blocked reports
expose no iframe/save/print surface; and direct subprocess tests cover both
runner binaries across all required verbs, output modes, and exit meanings.

Verification passed:

- selected Vitest: 3 files / 42 tests;
- Rust redaction binding: 5 tests;
- runner subprocess contract: 8 tests;
- full headless Rust crate: 44 tests;
- registered piping pytest: 520 tests;
- registered desktop tests: 24 files / 482 tests;
- registered desktop production build;
- H4 source: 2/2; production-dist: 1/1;
- practitioner harness: 311 tests and self-check;
- exactly one attempt-2 DEC-025 sweep: `PASS`, exit 0;
- scope, claims, path-anchor, JSON, protected-file no-change, and
  `git diff --check` validation.

The sole attempt-2 sweep artifact is
`validation/evidence/sweeps/SWEEP_20260722T061039Z_0c066652cd52-dirty.json`.
The attempt-1 artifact
`validation/evidence/sweeps/SWEEP_20260722T052931Z_0c066652cd52-dirty.json`
remains byte-identical at SHA-256
`10fbc3c4e54b69df2856cb5dd42240dc87b35d4c3762df9664e6036cf7b3cd63` and
is explicitly superseded/non-acceptance evidence because N5 blocked that
implementation basis. Only the attempt-2 sweep is offered as
acceptance-eligible evidence for fresh N5B review.

Residual limitation: no real native-GUI save/print automation lane exists.
Mocked-Tauri Vitest plus Rust command/subprocess tests cover pre-IPC/native
behavior; H4 covers browser-visible controls and downloads. No native
automation claim is made.

No status, memory, receipt, final deliverable run record, commit, push, PR, or
merge was performed. W3 remains held pending fresh N5B verification.
