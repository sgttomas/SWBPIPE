# N5D Return — Fresh attempt-4 terminal verification

**Verdict:** `BLOCK`

## Material findings

1. **Unmetadataed path/key names still become public in both route projectors, and the TypeScript path bypasses the new PCF/MBF exact allowlists.**
   Python `core/security/redaction/route_control.py:48-86,256-297` and
   TypeScript
   `apps/desktop/src/features/redaction-controls/redactionExportControls.ts:648-690,736-773,858-894`
   retain broad safe-token/suffix inference after the exact-record and exact
   structural changes. Fresh live probes supplied opaque, unmetadataed
   descendants. Python classified `schema`, `nested_deliverable_id`, and
   `target_mapping_refs` as
   `public_metadata/public_permissive/accepted`, action `include`, and exposed
   the values unchanged in `public_report`, `shared_model`, and
   `downstream_tool`. TypeScript did the same for `target_family`,
   `nested_deliverable_id`, and `schema` under a valid PCF root. The same
   TypeScript values remained included under the wrong route and under
   `DEL-99-99`, proving that `structuralProjectionFor` and
   `isExactStructuralPublicPath` are not the effective boundary. The N4D
   regression at
   `redactionExportControls.test.ts:300-365` exercises only names absent from
   the general safe-token list, so it does not detect this bypass. This
   violates candidate §3's explicit-metadata-only rule and mandatory N5C
   closure 2; opaque descendants can reach outward exports unredacted.

2. **The CAEPIPE local-private Parser CSV has an href without its own explicit intent and carries known private model/result values.**
   `CaepipeExternalHarnessPanel.tsx:60-68` passes `packet.csv_text` as raw text
   to `ControlledExportLink`; the text is built from stable IDs, load-case
   IDs, and result magnitudes at lines 452-471. `ControlledExportLink.tsx:53-70,102-112`
   decodes non-JSON as one scalar, which the projector treats as unknown.
   A fresh live probe with `explicitLocalPrivateIntent=false` returned
   `blocked=false`, `warning_only / REDISTRIBUTION_STATUS_UNKNOWN`, and the
   full CSV payload unchanged. Each link owns separate intent state at line
   56. The passing app test clicks only the Harness JSON checkbox at
   `App.test.tsx:1377-1381` and later decodes the CSV href at lines 1471-1477
   without clicking `caepipe-external-csv-link-local-private-intent`, thereby
   preserving rather than detecting the bypass. This violates candidate
   §§4-5.2 and acceptance predicate 4: `DOTH-CAEPIPE-LOCAL-006` requires its
   adjacent UI-owned intent before known private values may be retained, and
   no href may exist without that intent.

3. **The shared desktop control does not render decision or finding evidence before exposure.**
   `ControlledExportLink.tsx:72-98` renders only route ID and aggregate counts;
   neither `controlled.decisions` nor `controlled.findings` is rendered. This
   affects every new desktop download binding, including the two routes above.
   It does not satisfy candidate §3 step 5, §5.2, or acceptance predicate 5,
   which require decisions and findings—not only their counts—to be observable
   before a payload link exists.

## Independently reconciled evidence

- The exact-record public-basis recursion fix and report user-text inference
  fix are present in source and their focused regressions. They do not close
  the separate safe-token and CSV-member paths above.
- The route register has 31 unique RouteIDs and 31 unique disposition
  assignments, with no missing, duplicate, or extra assignment.
- Final tracked/untracked inventory has 184 paths and zero fence violations:
  N4D's 180-path terminal inventory, one later `N4D_FAN_IN.md`, and the three
  N5D paths (runtime-owned sealed brief plus the authorized return and TASK
  record). No dirty `test-results/` path exists. The ignored
  `apps/desktop/test-results/.last-run.json` was not used as acceptance
  evidence.
- N4D's six declared product/test files share the attempt-4 edit interval;
  the aggregate inventory adds no later product path. Protected-content and
  release tools, DEL-12-02 status/memory, loop receipts, parity corpus, core
  contract, and redaction schema remain unchanged from
  `0c066652cd527eb1559f715e914262d2bda42602`.
- Terminal registered evidence records focused Python `13 passed`, focused
  TypeScript/report `44 passed`, piping `522 passed`, desktop `487 passed`,
  desktop build pass, headless Rust `44 passed`, H4 source `2 passed`, H4 dist
  `1 passed`, harness `311 passed`, and self-check pass. Those suites lack the
  adversarial cases above.
- All four sweep hashes match their recorded values. Attempts 1-3 remain
  superseded; attempt 4 has exactly one acceptance-eligible sweep,
  `SWEEP_20260722T073143Z_0c066652cd52-dirty.json`, SHA-256
  `67fe4d2042469ba2ec2950c717b823bb4ad2a6ad66324889a13e6e354be2a29d`.
  The N4B H4 artifact also matches its hash-bound portability override at
  `26abe58c3f5eb6e021112d606984b9deda52069b42e9dbf385468afbed01f6af`.

No repair, evidence sweep, state/lifecycle/receipt, or Git effect was
performed. W3 remains held. Remediation requires a new bounded implementation
attempt followed by a fresh verifier.
