# Lane B-SHELL: retained returns of the manager's children

Each return is the child's final message, byte for byte as the host stored it, so the file's SHA-256 is the message's. On this host a background child's completion is delivered to the root session, not to its manager; ROOT relays the message to the manager as a file outside the repository with its SHA-256, and the manager checks the hash before retaining it. The manager's own verification of each return is recorded beside it.

| Return | SHA-256 | Child, stage | Model that ran | Reached the manager | Manager's verification |
|---|---|---|---|---|---|
| `B2-STATE_STAGE1_RETURN.md` | `febe96335fc9178be40951fe7f0d74ec064ea5aa23f86e56d4da56d513464c3b` | B2-STATE, stage 1 of 3 (pure modules) | Claude Fable 5.1 (`claude-fable-5-1`), as the child reports | by ROOT's relay, 2026-09-19; hash checked | The eight file hashes in the return match the working tree. Re-run by the manager on that tree: move audit PASS against `7c6784d95` (62 top-level statements identical, 39 of them in the seven new modules; 177 `AppSession` statements identical; 13 effects in base order; 4 new statements, all re-exports); `npm run build:desktop` exit 0; `npm run test:desktop` 75 files, 1,190 tests passed. Every line added to `App.tsx` is an import or a re-export. Committed as `a259fd691`. The child's three choices are confirmed (base order in `sessionModel.ts`; headers only where the audit tool does not read them as a statement's comment; `import type` in new modules). Its items for someone else are carried to the slice return; the two possible defects it names in existing handlers are not touched in this slice, at ROOT's direction. |

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
