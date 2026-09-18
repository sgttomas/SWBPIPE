# A1-LINT — retained return

Retained by ROOT. What follows is the child's final message as the host stored it, with the machine's repository path replaced by `{REPO_ROOT}`; nothing else is changed.

---

# Return — A1-LINT: the claims registry and the root claims lint

The registry, the lint and its tests are done within my write scope. The lint tests pass. The live-tree lint still reports the A1-APP sibling's pending removals in `apps/desktop/src` and nothing else.

**Model:** Claude Fable 5.1 (`claude-fable-5-1`), TASK (Type 2). I worked alone, delegated nothing, and ran no state-changing git command. The brief's SHA-256 verified as `3db45afb…1cad` before I started.

## Files changed

All three are inside the write scope; the paths in groups 2 and 3 are relative to `{REPO_ROOT}`.

### 1. Registry — `{REPO_ROOT}/projects/chirality-piping/docs/claims_registry.md`

SHA-256 `3c886e1b…f942e`.

- **Amendment note.** A dated note, "Amendment, 2026-09-18", sits under "Enforced by". It cites `DEC-100`, `DEC-101` (ii), `DEC-102` and `DEC-105`. It records that `DEC-099` was superseded and never executed.
- **`BS-MATURITY`.** The entry stays as a retired entry.
  - It names the ruling and the surfaces that no longer carry the sentence.
  - It records that the `MISSING_MATURITY_BANNER` anchor is removed and names the new finding code.
  - It states that this is a display decision and the stage record is unchanged.
  - It keeps the former canonical text ("recorded here only") and the former placement, and says there is no replacement.
- **`BS-ACCEPT`.** The canonical text and the four short variants remain registered, but only for non-product surfaces. The placement clause is retired for product surfaces, and the entry lists `DEC-100`'s surfaces. The "not authoritative" and litany retirement is restated as still enforced. The items `DEC-100` leaves untouched are listed.
  - **Decision on registration:** the texts stay registered for non-product surfaces. 33 live `ScopeOfWork.md` files and 12 scanned project documents carry a registered `BS-ACCEPT` text. Unregistering it would turn those lines into litany findings on surfaces I may not edit and that `DEC-100` does not reach.
- **`BS-IP`.** The canonical text now begins "SWBPIPE ships…". A note records the former text and its dates. The short variants are unchanged.
- **§2.** A new §2.1, "Display forms (`DEC-102`)", holds the eight-row table of token, display form and authority domain, the three rules, and the statement that `ENGINEER_ACCEPTED` stays reserved. Domains are named as in `docs/SPEC.md` §4.3.

### 2. Lint — `tools/validation/validate_claims_language.py`

SHA-256 `0e1797f1…e6ce3`.

- **Registered texts.** `REGISTERED_TEXTS` follows the amended registry. The `BS-IP` canonical takes SWBPIPE and the maturity sentence is removed. The `BS-ACCEPT` texts move into a new `BS_ACCEPT_TEXTS` tuple that is spread into `REGISTERED_TEXTS`.
- **Anchor.** The `MISSING_MATURITY_BANNER` anchor is removed.
- **New guard `RETIRED_MATURITY_SENTENCE`.** It checks every scanned file. It ignores case, accepts any dash form and does not require the final period.
- **New guard `RETIRED_ACCEPTANCE_SENTENCE`.** It checks `apps/desktop/src` non-test files only. It fires on any of the five `BS-ACCEPT` texts, and reports the canonical text once even though it contains its own second sentence.
- **Wrapped lines.** Both guards match across wrapped lines by normalizing whitespace and stripping leading comment markers. Placements wrapped in JSX and in `//` comments are caught.
- **Unchanged rules.** `RETIRED_PHRASE`, the litany rule, the threshold and the suppression window are unchanged. One message changed: the "not authoritative" remedy now reads "remove it" on product source instead of "use BS-ACCEPT". Elsewhere it is as before.
- **Former `BS-IP` text.** It is not kept registered. It carries no litany term, so it never suppressed a finding, and no scanned surface carries it. The code comment and the registry both say so.

### 3. Lint tests — `tools/validation/test_validate_claims_language.py`

SHA-256 `bf100258…0ea5b`.

- The test fixture's `App.tsx` no longer carries the sentence.
- The missing-banner case became a clean-shell case that also asserts the anchor and the registered text are gone.
- Maturity guard: two firing cases (app shell; docs, wrapped and lower-cased) and one clean case (registry, history, test file, and the phrase "technical preview" alone).
- Acceptance guard: three firing cases (JSX-wrapped plus a string literal, with line numbers; the canonical text reported once; a wrapped comment) and one clean case (a `ScopeOfWork.md`, a doc, a `.test.tsx`, and the control word "Accept").
- One test pins the registered-text constants. No behaviour test was deleted or weakened.

## Checks

1. `python3 -m pytest -q tools/validation/test_validate_claims_language.py`, run from the repository root: **24 passed**.
2. `python3 tools/validation/validate_claims_language.py`, run from the repository root: **exit 1, 43 findings**, all in `apps/desktop/src` and none in docs or `ScopeOfWork.md`. No `AD_HOC_CLAIMS_LITANY` or `MISSING_*` finding. Before my edits the lint reported VALID over 320 files. My first run after the edits reported 67 findings, which fell to 43 as the sibling landed removals.

| Finding | Count | Class |
|---|---|---|
| `RETIRED_MATURITY_SENTENCE` | 2 | A1-APP sibling's pending work: `BuildReadinessPanel.tsx:75` and `ExportReviewPanel.tsx:658`. `App.tsx:3282` and `ReportLintPanel.tsx:578` fired on the first run and the sibling has since cleared them. |
| `RETIRED_ACCEPTANCE_SENTENCE` | 41 | A1-APP sibling's pending work: acceptance-sentence placements across about 25 product files. They include source comments in `services/ruleCheckService.ts:11`, `services/libraryImportService.ts:11` and `RuleCheckRunPanel.tsx:54`. |

No finding is mine and none is a real defect.

## Identity strings deliberately left

None arise in my three files; I renamed no identifier. The project path `projects/chirality-piping` is unchanged, as are the finding codes `AD_HOC_CLAIMS_LITANY`, `MISSING_PRD_NOTICE` and `MISSING_RENDERER_NOTICE`.

## What I could not do, and its limits

- **Evidence-label domains.** `DEC-102` rule 2 needs an authority domain for every label, but `docs/SPEC.md` §4.3 defines only three, all for statuses. For `INTERNALLY_VERIFIED` and `PROVER_CORRELATED` the table says "evidence status (this section); not one of the three §4.3 domains". I did not adopt the design system's word "Evidence", which is the open item in its §8 item 14. The owner or ROOT should settle the domain word.
- **Displayed domain words.** The registry table uses the `docs/SPEC.md` §4.3 names. It says the short displayed word ("Solver", "Rule pack", "Human") is a design-system matter, because `DEC-102` does not rule those words.
- **String concatenation.** The new guards do not reassemble a sentence split across concatenated string literals. The lint docstring says so. No current placement is split that way.

## For a sibling or ROOT

- **Unregistered acceptance paraphrases.** Two product placements paraphrase the acceptance sentence without using a registered text, so the guard does not report them: `features/model-tree/ModelTree.tsx:909-910` and `features/model-tree/PropertyInspector.tsx:1078`, both reading "acceptance stays with the responsible engineer". `DEC-100`'s owner words ("Remove any such mention of 'acceptance'") reach them. A1-APP should remove them by hand. I did not invent a broader pattern beyond the registry.
- **Retired texts in test files.** `apps/desktop/src/App.test.tsx` and other `.test.tsx` files carry the maturity and acceptance texts. Test files are not scanned, so the lint will not prove their removal.
- **`DEC-103` and `BS-ACCEPT` in product source.** `MissingDataBlockingPanel.tsx:539` combines a `BS-IP` short variant with a `BS-ACCEPT` variant. Only the `BS-ACCEPT` half is retired.
- **Hand check until clean.** `DEC-100` inventoried 53 placements in 21 files. The lint counts registered-text occurrences in non-test source only, 63 at first run, so the two counts are not comparable. ROOT's re-run after integration should reach zero.
- **Anchors under sibling edits.** Siblings' uncommitted edits touch `docs/PRD.md` and `core/reporting/report_renderer/src/lib.rs`. `MISSING_PRD_NOTICE` and `MISSING_RENDERER_NOTICE` passed on my last run.
- **Stale references to the old anchor.** Historical records that name `MISSING_MATURITY_BANNER` are untouched. Outside them, the only live mentions are in my three files.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
