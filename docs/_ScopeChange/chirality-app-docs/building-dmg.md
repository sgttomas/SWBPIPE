# Building Unsigned macOS DMG (Apple Silicon)

This runbook defines the canonical local-builder workflow for producing an unsigned Chirality `.dmg` from this repository.

## Scope

- Platform: macOS 15+ on Apple Silicon (`arm64`)
- Output: unsigned, unnotarized `.dmg`
- Distribution model: local build and self-install

## Prerequisites

1. macOS 15+ on Apple Silicon.
2. Node.js `>=20` (`frontend/package.json` `engines.node`).
3. Repository checkout with dependencies installed.

## Build Commands

```bash
cd frontend
npm install --no-audit --no-fund
npm run desktop:dist
```

`desktop:dist` performs:

1. production Next.js + Electron build (`npm run build`)
2. unsigned DMG packaging (`CSC_IDENTITY_AUTO_DISCOVERY=false electron-builder --mac dmg --arm64 --publish never`)
3. instruction-root integrity verification (`npm run instruction-root:integrity`)

## Expected Artifacts

- DMG: `frontend/dist/Chirality-0.1.0-arm64.dmg`
- App bundle: `frontend/dist/mac-arm64/Chirality.app`
- Integrity summary: `frontend/artifacts/harness/instruction-root-integrity/latest/summary.json`

## Verification

### Architecture (REQ-DMG-002)

```bash
file frontend/dist/mac-arm64/Chirality.app/Contents/MacOS/Chirality
```

Expected: output includes `arm64`.

### Minimum macOS target (REQ-DMG-003)

```bash
/usr/libexec/PlistBuddy -c "Print :LSMinimumSystemVersion" \
  frontend/dist/mac-arm64/Chirality.app/Contents/Info.plist
```

Expected: `15.0.0` (or later).

### Unsigned status (REQ-DMG-004)

```bash
codesign -dv --verbose=4 frontend/dist/mac-arm64/Chirality.app 2>&1
```

Expected baseline: `TeamIdentifier=not set` and `Signature=adhoc` (no developer signing identity requirement).

### Instruction-root bundle presence (REQ-DMG-006)

```bash
test -d frontend/dist/mac-arm64/Chirality.app/Contents/Resources/agents
test -d frontend/dist/mac-arm64/Chirality.app/Contents/Resources/docs
```

Both checks should succeed.

## Install and Launch

1. Mount `frontend/dist/Chirality-0.1.0-arm64.dmg`.
2. Drag `Chirality.app` to `/Applications`.
3. Launch `Chirality.app`.
4. Confirm the working-root selector is available and app shell loads.

## Gatekeeper Note

Because builds are unsigned and unnotarized by design, macOS may block first launch when the app is transferred to a different machine. If prompted:

1. Open **System Settings > Privacy & Security**.
2. Allow the blocked app.
3. Relaunch Chirality.
