**One actionable finding; candidate is not yet suitable for ROOT integration.**

**P2 — A failed later request discards a successfully saved comparison basis.**  
Location: [workspaceSession.ts:1773](/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-wt3/projects/chirality-piping/apps/desktop/src/features/workspace/workspaceSession.ts:1773), enforced again at lines 1815, 1817 and 1827. The Create handler duplicates this guard.

Trigger:

1. Begin with clean canonical contents A, edit to B, and dispatch Save.
2. While its response or returned-hash verification is pending, invoke native File → Open.
3. Open fails or finds nothing, leaving the same project/session loaded.
4. Save succeeds with valid persisted contents B.

Open increments `projectRequest`, so `ownsPersistence()` rejects the successful Save observation despite unchanged session generation. The baseline remains A. Consequently, B remains marked Edited even though it was saved; Undo to A can remove Edited even though the persisted contents are B. Both shell heading and native title consume this incorrect state.

The route is reachable: `runMenuCommand` dispatches native File handlers without a `projectBusy` guard, and native menu mutations do not disable File commands. Existing `App.projectHandlers.test.tsx:528` covers a **rejected** late Save; the new replacement test covers a **successful** Open. Neither covers successful Save followed by a failed successor.

Preserve a validated same-session observation of landed contents independently from latest-request UI adoption, with ordering that protects newer successful saves and project replacements. Keep the existing model/history adoption guards. Add a regression covering this sequence and the resulting Undo/Redo marker states. This finding concerns the new marker; it does not request the deferred B3B integrity or menu changes.

Reviewed all 13 maintained paths in `7e6a7f2578e5df12ea891c2d604d0cd48e869934..80b9efb48c31da0d8dc58a2d728db56adcf17d22`, including callers, canonical hashing, normalization, Historical carrier handling, status policy, native bridge and changed tests. No other actionable finding. Scope validation passed; HEAD and tracked files remained unchanged at final inspection.

This was a read-only, source-level review: no builds, tests, browser/native interaction, writes, Git mutation or delegation. Worker log hashes and all 13 source-manifest entries matched. Manager final checks/native evidence still need the requested packet backcheck; no packet acceptance or native-witness claim is made.

Execution: TASK Type 2 `/root/b3a_code_review`, parent ROOT HELP_HUMAN, delegated-harness-native; supplied model/effort `gpt-6-astra` / `xhigh`. This is independent review, not model diversity.

Context SHA-256 bindings:

| Context | SHA-256 |
|---|---|
| Independent review brief | `ad366d67b2fc0bd64682d84ec75ae3432ac55746df5b5c26e0ef2a24ef6e2fbb` |
| B3A manager brief | `98986a724525d0e5d3fe1465b6ad2185d8d436fccd56f9f9494d24f417d195f2` |
| Root AGENTS, both checkouts | `d151dad92a074abebf8e6225c92c4c6e88fd586f2283377b50b5051eff39be7b` |
| TASK role, ROOT checkout | `1a13a5b00b3ce01ff8519efe6b46bcbe0cd6a5b7985e24282fa7efa2c57c8fb7` |
| Piping AGENTS, source checkout | `eec1b9accc5dc4485fd0aab423678bed4c4c98396dc88b9089703965eb3d2005` |
| Piping loop, source checkout | `f327d5c6fe4d964a15786813f6de72170bc5e94471753846f461f622281e5f94` |
| software-code-review skill, ROOT checkout | `06c27b1be5cfbd9e638570918a8f837d8439c8073c40d3ef708e53874f95570a` |
| chirality-change skill, source checkout | `2b490e172436417896c1cd25dbcd543c676e3473aa58b7663985d75785ff7dba` |
| Owner EIGHT_UX_ITEMS | `5b86c2783c5d460adf63b69dc3a49db7df672795f0feeb59079a2f93a82c0f4e` |
| claims_registry, §2.1 consulted | `fc8560b08418fb6cdb240a1961323638abdaf16cefef5d4d7c43358c1ee2b430` |
| UX_SPEC_V1, §5.4–5.5 consulted | `2141c1e844109d4869287048c695e79677f423acae965d5dd0436b7f6aeff682` |
| Worker implementation brief | `29c38d450b7f166d77b2c2758e2d0ff0bda9ef1cf86eab52074b3db7f0f2376b` |
| Worker FINAL_MESSAGE | `cd8851e85a358a05731a66b8bdbc7dd63bd0379e1022e4f07816ecf1130bde49` |
| Worker CHECKS.json | `57fc1536a9779f5166c4aacc6bd03876ce91791956e2bb12ecd41176479a091b` |
| Worker SOURCE_HASHES.json | `e2ce13406cccc22aad2844ba9ba9fbeca9c69d5eda237c21617d5deb4cd938e8` |

Exact absolute origins were emitted in the review tool record. ROOT retains this return; source checkout is `/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-wt3`, and ROOT checkout is the supplied `b3d208ad-bb6f-4bed-aaab-c567e28cbe23/chirality` worktree.

Standard claim fence applies (F-PIP-2; DEC-081).

