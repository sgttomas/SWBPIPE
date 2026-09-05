# U2 remediation return V3

SUCCESS; source frozen. Parent v4 instruction applies root visual feedback to selected-property presentation. All prior evidence preserved; this source freeze supersedes RETURN_V2.md.

Removed redundant inner Properties heading. Display-only Label becomes Name; main heading identifies Edit name/material/etc. Property selector and New name/material/etc labels describe the actual editable value. Unit label uses entered unit symbol. Rationale and queued technical receipts moved into existing Operation details disclosure with identical state/value preserved. Empty no-missing status card no longer rendered; actual required findings remain visible. V2 contextual creation disclosures preserved. No operation, validation, engineering value, default or schema changes.

Actual model unknown (not exposed); no delegation; native role instruction-asserted. Source write fence unchanged.

## Frozen SHA256
| Project-relative file | SHA256 |
|---|---|
| apps/desktop/src/features/model-tree/PropertyInspector.tsx | 697df27e88d87e7004c0a331c4905857573b90c428f62310aaf15f3a1fb42022 |
| apps/desktop/src/features/model-tree/ModelTree.tsx | b0babf21868697924ac4b5ec5adb1321bade3b227f2066a128b26167ee369dc3 |
| apps/desktop/src/features/model-tree/typedInspector.test.tsx | 9b2c28c7fd4c2757949416d0b061fd5d7a43b147b4395ab4eabb56237b435d95 |

## Evidence
Focused `../../node_modules/.bin/vitest run src/features/model-tree/typedInspector.test.tsx src/features/model-tree/schemaSlotEmission.test.tsx` from apps/desktop: PASS16/16,728ms,2026-09-05 13:31:18 local. Added assertion covers named field heading/input, absence of empty flags, hidden but preserved rationale, disclosure access and updated input naming when selecting material. Existing support disclosure test explicitly supplies missing provenance to witness actual flags visibility. Source-only explicit path validation PASS; feature git diff --check PASS.

## Handoff
W7 notified source stable before screenshot. W7 owns final tests/build/1024x768 visual check and independent frozen-diff review. TestIDs retained; legacy accessible-name queries `Intent field` and `Proposed editor value` should use testIDs or updated labels `Property to edit`/`New …`. No final usability/lifecycle acceptance claim.
