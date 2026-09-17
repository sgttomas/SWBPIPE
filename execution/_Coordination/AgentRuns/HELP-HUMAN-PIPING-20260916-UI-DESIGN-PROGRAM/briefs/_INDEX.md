# Research briefs — index

These four files are the prompts actually supplied to the research children, extracted after the fact from the ROOT session transcript. No sealed brief file existed before dispatch; the prompt was the brief. Each copy differs from the supplied bytes only by two declared substitutions: the ROOT worktree's absolute path is written as `{REPO_ROOT}` and the session scratchpad's absolute path as `{SCRATCHPAD}`. The SHA-256 column is the hash of the supplied bytes before substitution.

Mechanism: Claude Code `Agent` tool, `general-purpose` subagent type, launched by ROOT (HELP_HUMAN) in the same host session. Parentage is ROOT for all four; none delegated further. Write targets were instruction-asserted (one output file each under `{SCRATCHPAD}/research/`), not sandbox-enforced. Each child's return is its output file, retained under `../instances/RESEARCH/`.

Path conventions inside the returns are the children's own and are left as returned: A and B cite product source relative to `projects/chirality-piping/apps/desktop/` (the workspace shell file, for example, is cited by its path under that directory), C cites project surfaces relative to `projects/chirality-piping/`, and all three shorten long paths with `…`. The harness coordination check lists those citations as unresolved references; that is a property of the returns' notation, not missing files.

| Brief | Tool-use id | Launched (UTC) | Model requested | Return |
|---|---|---|---|---|
| `RESEARCH-A_ux_audit.md` | `toolu_01Q2AdVAYR8XE9keWCt3Bw7s` | 2026-09-17T04:23:13Z | `fable` (Claude Fable 5.1) | `../instances/RESEARCH/A_ux_audit.md` |
| `RESEARCH-B_ui_inventory.md` | `toolu_01AMbJSiczkm7p5TnaAfJ2DD` | 2026-09-17T04:23:31Z | `opus` (Claude Opus 5) | `../instances/RESEARCH/B_ui_inventory.md` |
| `RESEARCH-C_ui_constraints.md` | `toolu_01XyncMaUWhXKs9CbzohyRxM` | 2026-09-17T04:23:50Z | `opus` (Claude Opus 5) | `../instances/RESEARCH/C_ui_constraints.md` |
| `RESEARCH-D_domain_ux_research.md` | `toolu_01EFdrLR9PgjdWHMcqEoh5tm` | 2026-09-17T04:24:12Z | `fable` (Claude Fable 5.1) | `../instances/RESEARCH/D_domain_ux_research.md` |

Model allocation followed the owner's rule for this program: Opus where the result is quantitative or objective (inventory, constraints extraction), Fable where quality, judgement or extensive analysis matters (heuristic audit, domain research). The model column records the request made to the host; the host's own attribution of the model that ran is not independently verified here.

| Supplied bytes | SHA-256 |
|---|---|
| Brief A prompt | `7e6a9448af180127c37de6ace96136270f25d38abf96c6384d7107510949a344` |
| Brief B prompt | `e6c398c079bef353868edd03b3d34556d4d03e8a6f60cb96bdda5fa8a6a9ad65` |
| Brief C prompt | `0e535a125ebac9f6317ebb1893935945a89f7f84ca9ff149a171e5e4e87d6e05` |
| Brief D prompt | `011ffe9430e9df57940b1328c3fd7f6912f750d137b69f766abae40990631189` |
| Return A, `A_ux_audit.md` | `39d7420eec9703ef132f48f9b3967da1e32e9fd4dbebb20ec204a747ea7468da` |
| Return B, `B_ui_inventory.md` | `027f40c6e3a66efd4af7ff81f46ef7f0e0813aa1ed85c53942a1d06ce4524433` |
| Return C, `C_ui_constraints.md` | `9beb811ea8d9478033723678c644e71faff5bbf51050ef485001d1612dc57789` |
| Return D, `D_domain_ux_research.md` | `37c6e392cd0b6b772ff692e4d4fc03a61d1fc1b6019815df5bd7abc3e0b0322f` |

The returns are research inputs to the design program. They are not accepted findings, not decomposition truth and not product claims. Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
