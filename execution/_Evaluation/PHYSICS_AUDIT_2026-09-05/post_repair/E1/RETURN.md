# E1 return: exact CRLF recognition complete; unrelated EOF error remains

The new audit-local .gitattributes names exactly 67 frozen CSV paths. Each uses `whitespace=blank-at-eol,blank-at-eof,space-before-tab,cr-at-eol`. Effective core.whitespace is unset; installed Git 2.55.0 documentation identifies the first three as defaults and cr-at-eol as line-terminator recognition. No existing check was disabled. Explicit string attributes do not dynamically inherit future core.whitespace additions; if project policy later enables another check, these exact rules require coordination to retain it.

All 67 inventory paths match both the actual audit CRLF CSV inventory and the original reported trailing-whitespace inventory. Every physical newline is CRLF, each file has a final terminator without a blank final record, and no trailing space/tab or initial space-before-tab was found. BEFORE.json and BYTE_PRESERVATION.json bind worktree-before, staged index, and worktree-after SHA-256 values: all unchanged. Frozen manifests were not edited.

Working-tree attributes immediately affect `git diff --cached --check`, without staging. All CSV diagnostics disappear. The complete check still returns 2 because an independent existing immutable raw log has a genuine extra blank line: `V1/baseline/cargo.log:1757: new blank line at EOF.` This is outside E1's alteration scope and was reported to Agent 0. No exemption or normalization was applied.

The isolated toy repository uses the identical rule. Valid CRLF passes; a space before CRLF, tab before CRLF, blank line at EOF, and indentation space-before-tab each fail. An unlisted CRLF CSV still fails, demonstrating narrow path selection. CHECKS.json preserves outputs and exact sample bytes in hex, avoiding introducing invalid-whitespace evidence files.

ATTRIBUTES_DIFF.txt contains the complete new-file diff. INSTALLED_GIT_DOCUMENTATION.json records relevant local official manual excerpts. verify.py records the procedure (rerunning its baseline expectation requires the stated remaining EOF diagnostic; subsequent CHANGE work can change that state).

Handoff: E1 bounded verification complete, derivative evidence only; no physics, public interface, engineering acceptance or authoritative decomposition changes. Parent/F3 fresh review is required before CHANGE stages this exact file. Whole staged-check closure remains blocked by the raw-log EOF issue and must be rerun after its separately authorized resolution. No project Git mutation occurred.
