# RU successor return

**Verdict: CHANGES_REQUIRED.**

All sealed identities match, all 26 successor-manifest entries rehash, and the complete 1,102-line six-path successor delta was reviewed. `RU-F1`, `RU-F4`, and `RU-F5` close. `RU-F2` and `RU-F3` remain open in two narrow ways:

1. The Apply binder accepts altered engine-created and echoed frozen hash `payload_ref` values, and accepts a malformed non-warning diagnostic. App's later recomputations do not cover those fields before publication.
2. An external replacement containing the same pipe ID/from/to as the pending route is mistaken for the component's own commit, so it preserves Continue and advances `from`; the delayed callback itself is correctly prevented from publishing.

Required focused repository checks pass: route 17/17, inspector 9/9, App 32/32 plus persistence 2/2, and Chromium 1/1 at 1024×768. RU's affirmative adversarial suite fails 3/3 on the confirmed production behavior. Exact source bindings, minimal repairs, evidence limits, hashes, and command records are in `REVIEW.md`, `VALIDATION.md`, and `REVIEWED_INVENTORY.sha256`.

This return supports manager fan-in only. Native remains a separate held gate. RU wrote no source or repository tests and performed no build, Git, lifecycle, dependency-row, native, commit, or push action.
