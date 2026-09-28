# Delivery record: SWBPIPE answers to the App v4 relay questions

- **Recorded by:** ROOT (SWBPIPE HELP_HUMAN), 2026-09-28.
- **Owner direction:** the answers go to a location the App session can read; commit and merge as needed.
- **Delivered to:** `projects/chirality-app-v4/execution/PKG-09_Candidate examination and connected journeys/1_Working/DEL-09-06_Connected activity contract and workflow round trip/Design/`, as two new files beside `RELAY_QUESTIONS_SWBPIPE.md`. No existing App file was changed.

| Step | Revision | By | When (UTC) |
|---|---|---|---|
| PR #1047 opened | head `e27794262`, base `65e2d6c2a` | ROOT | 2026-09-28 |
| main merged into the branch (CI refused the stale base after #1046) | `73519d1d2`, then `a2323bc96` | ROOT | 2026-09-28 |
| RV15 review | at `a2323bc96`: MERGEABLE; 4 SHOULD-FIX, 10 NOTEs | RV15 | 2026-09-28 |
| Fixes | `c322826ea` | ROOT | 2026-09-28 |
| **PR #1047 merged** | head `c322826ea`, merge `41aeb2a02` | the `sgttomas` account, not ROOT | 22:49:33 |
| RV15 delta at `c322826ea` | MERGEABLE; D-1 SHOULD-FIX, D-2 and D-3 NOTEs | RV15 | 2026-09-28 |
| D-1 to D-3 applied | `a999f4ba1`, pushed after #1047 had merged | ROOT | 22:51:59 |
| RV15 final delta at `a999f4ba1` | MERGEABLE, nothing new | RV15 | 2026-09-28 |
| **PR #1048 merged** | `a999f4ba1` merged onto main as `22a77811c`, merge `56dd72334`. Its diff against main is byte-identical to the reviewed `c322826ea..a999f4ba1` | ROOT, `--match-head-commit` | 23:09:58 |

- **Final bytes on main at `56dd72334`:**
  - `RELAY_ANSWERS_SWBPIPE.md`, sha256 `afb6e063e7e5dfccf8986206f888cf1e2e1c4e7fc3430fdf86db670714fc0e74`;
  - `FACTS_SQ01_SQ32.md`, sha256 `733fb88a701317be8f0054937eca058774ba5f5f30c7a27233718996e8b2ab7e`.

  Both equal the copies in this folder (SHA256SUMS).
- **Hosted CI:** green on both PR heads. The PRs change only App-folder Markdown, so the piping suites and E2E coverage were not selected.
- **The App's return ledger:** the App manager records the answers in the questions file's §4 under its own ownership. SWBPIPE does not edit App files.
