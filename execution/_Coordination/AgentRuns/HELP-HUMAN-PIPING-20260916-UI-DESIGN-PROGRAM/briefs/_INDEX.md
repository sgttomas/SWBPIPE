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

## Phase 2 brief, sealed before launch

| Brief | Sealed SHA-256 | Launched (UTC) | Model requested | Role | Return |
|---|---|---|---|---|---|
| `CONCEPTS-01_north_star_directions.md` | `5e73a829af744a0708fae2dd9f62fe1e308636e43980aa57f9a64713d7132776` | 2026-09-18T03:46:50Z | `fable` (Claude Fable 5.1) | HELPS_HUMANS design manager, working alone | `../instances/CONCEPTS/` |

Return of CONCEPTS-01, retained under `../instances/CONCEPTS/` and accepted by ROOT on 2026-09-17 against the brief's acceptance section (three distinct organising ideas; eight states storyboarded per direction; M-01 to M-17 homed per direction; nine wireframes present, parsed as XML and rendered by ROOT in the browser pane; reasoned recommendation). One declared substitution: in `RETURN.md` line 7 the child wrote the ROOT worktree's absolute path as the value of `{REPO_ROOT}`; ROOT replaced that path with a parenthetical note before retention, and the `RETURN.md` hash below is of the retained bytes after substitution (the bytes as returned hashed `6dd98ae07e5c8624d22e731549e757137e65d5c0b97fc3855edeb9885cc16639`). All other files are retained byte for byte.

| Return file | SHA-256 |
|---|---|
| `CONCEPT_DIRECTIONS_V1.md` | `a79137f3678416285ac9e647136e195da33cea8502ffb1e27d6b65adae8fb4a6` |
| `RETURN.md` (retained) | `11f61923720186cffbf682885a36e49f990a0646c66f638cc8a6cc57e03113a1` |
| `wireframes/layout_sheet_model.svg` | `08a7a825d4420ac1e6e6eef575355972db9b3c9a6f7db1e2ecec37d140920483` |
| `wireframes/layout_sheet_grid.svg` | `2bf2a343e17307091907588e6d37623269f66df82343e6be0d251ebd6e2c1091` |
| `wireframes/layout_sheet_split.svg` | `ed96eb739b4e3c246f24de23d25a645347c6d4baf7bafe9ba569aec4b6798b33` |
| `wireframes/workbench_model.svg` | `9cf7dd8ce1f4d104d8f36cde6148a8377493ff2e4d65422e3938e299f8250ad4` |
| `wireframes/workbench_grid.svg` | `349bca8519b5df99c74a18fda68c9dfd022aa38b1f529b5ced615652f4cb0586` |
| `wireframes/workbench_split.svg` | `a470500846b8bc88dcd7b81fac8af83ae3f38227ace974dbbcc5158dec5df0fe` |
| `wireframes/run_book_model.svg` | `cab32044da6e98a9a8eb66308b80df055ebd2298bd84488bef757bce77a25681` |
| `wireframes/run_book_grid.svg` | `31f8afce0a5ad1cc0b0c692623a34bb67ef80650c82aaa1d39dcf00c11da55b8` |
| `wireframes/run_book_split.svg` | `99e0b78c11c55ae02b6b5e8e1144f3c9408d6243c4fafda1bf1839d063873798` |

## Direction-phase research brief, sealed before launch

| Brief | Sealed SHA-256 | Launched (UTC) | Model requested | Role | Return |
|---|---|---|---|---|---|
| `RESEARCH-E_caepipe_format.md` | `928028d6507a00a630c6d5a0a981f3f7117c3cdf4b36ecf7957ebb4399ba8b5d` | 2026-09-18T05:16:45Z | `opus` (Claude Opus 5) | TASK, working alone | `../instances/RESEARCH/E_caepipe_format.md` |

Return `../instances/RESEARCH/E_caepipe_format.md`, SHA-256 `334719ad3232506d53004c2f89bcdfb82f073e7366fd3fa9752259d3beba708d`, retained byte for byte and accepted by ROOT on 2026-09-17 against the brief's acceptance section: sources complete with URLs and UTC retrieval times (43 manual pages, two product pages, one third-party page; four unreachable sources listed as unreachable); every format statement cited; the layout record, attachment data and load-case records tabulated to the level the documentation supports; a mapping table with 16 named gaps and a separate list of 12 format contents the design does not yet name; 14 unknowns stated as TBD; no compatibility claim. The return's own relative paths (`../../briefs/…`) resolve from its location.

Launched after the owner's direction of 2026-09-17 named export compatibility with CAEPIPE as a requirement (`../instances/ROOT/DIRECTION_DECISION_2026-09-17.md` §7). Opus because the result is objective: what a documented format contains.

## Phase 3 brief, sealed before launch

| Brief | Sealed SHA-256 | Launched (UTC) | Model requested | Role | Return |
|---|---|---|---|---|---|
| `DESIGN-SYSTEM-01_design_system.md` | `e7d21c114c6794f26dd0a653b3c333814ba2f5f24c2cdcb7a5e00cd4b19d37b9` | 2026-09-18T06:23:55Z | `fable` (Claude Fable 5.1) | HELPS_HUMANS design manager, working alone | `../instances/DESIGN-SYSTEM/` |
| `MOCKS-01_mock_drafts.md` | `c483bfb424140b6eab6df948e41bb68784bd18036e6160fc4b0f865f600d57e9` | 2026-09-18T07:22:07Z | `fable` (Claude Fable 5.1) | HELPS_HUMANS design manager, working alone | `../instances/MOCKS/` |

Return of MOCKS-01, retained under `../instances/MOCKS/` and accepted by ROOT on 2026-09-18 against the brief's acceptance section: fifteen frames exist, open offline and scale to fit; each shows its state in the named view and theme with the whole shell; one sample model runs through all of them; every value traces to the token file (six gaps listed in `MOCKS_V1.md` §5 with the nearest token used); the child's lint found each mandatory disclosure once where the design system places it and no forbidden word; `MOCKS_V1.md` records each frame's decisions, departures and fourteen questions; RETURN states what was verified and what is uncertain. After the first return ROOT sent the child one instruction (recorded in RETURN §3 and MOCKS_V1 §6): inline the two shared stylesheets so each frame renders from its own bytes, because the owner's review surface renders a local file as a static snapshot; and rename the Review outline row "Review and sign-off block" to the registered report-content name "Review/signoff block" (RESEARCH-C §4, M-07). The child re-verified with no change in measured facts. ROOT opened frames in the browser pane after the fix and confirmed they render styled with no network request. Retention decision: of the child's `shots/`, the fifteen 1:1 stage screenshots, `index.png` and `report.json` are retained; the twenty-nine page screenshots at 1440 × 900 and 960 × 700 were not retained (derivable by re-running `tools/render.mjs`; the measured facts are in `report.json`). All retained files are byte for byte the child's.

| Return file | SHA-256 |
|---|---|
| `MOCKS_V1.md` | `e59488d40f99b92d00f816230e426d66cb43ce0b0482f3ed40ed22116775f389` |
| `RETURN.md` | `12b47f500ffa1d6ad9cb868697182a7ae07be3d20cace5c8bdd76e44595401c3` |
| `sample_model.md` | `5e85d633bbf19316edec847b80fd93af75db79b11ffb081b4496657274469a76` |
| `frames/index.html` | `d0ba6a195cf791cef2e180f3d307c8f0431085c6799ebdcfbe9a51be3870d519` |
| `frames/s1_table_light.html` | `3863868cafc891b225bea5eaa17b0953672cb80a8b02f8df0fd0cd50af8ebc96` |
| `frames/s2_model_dark.html` | `29f40011b30b27d2ed0546e61f0f84184a77d2a583df3c238f8e8c51b4654df8` |
| `frames/s2_model_light.html` | `91e78608b6a665739497c040ff6e4bb306371846f63d8c3acb1a4adf14825105` |
| `frames/s3_table_light.html` | `f10bf723d2866e4283839bbae74f6b47c0e2dead0637a8f36c0f2c9f06c0de07` |
| `frames/s4_both_light.html` | `ac4159e4f703d518688395b663c9b940476f65856bff3ef945e4679b076846e0` |
| `frames/s4_table_light.html` | `245c19e6dc2e12a199f91194c3b9fbb7a1ad5a79dd490f1d4e69c66a48b386dd` |
| `frames/s5_table_light.html` | `f7b38eb5bd917d01c5f4e58f1b9049b4ae0d6e6e04f69055d7bfa51d9bf712e7` |
| `frames/s6_both_light.html` | `b4aa6328dd7de0ac6999dcb1d99b13c0f42e3470527ede0440a52bfb67223948` |
| `frames/s7_both_dark.html` | `849eb9e3b9723562dc19258c91214c08bd52b9a090329b4e54c8c4e1b123de81` |
| `frames/s7_both_light.html` | `d9d8de647767875513c05a5b82edf766b77b165110a4b6098fcd7b41ab16adca` |
| `frames/s7_table_light.html` | `e1279040855634c818bf920e7373f7eb9bd737e49149bfa2ef0c05ae63a0ee30` |
| `frames/s8_model_light.html` | `0adc3c9139a4ba54572284c8503e67786cc0dc28410121c56a2a3df8bdc8fe49` |
| `frames/s8_table_light.html` | `2c10947b81eff9efba33c5de9bdb4b1727426c6a98cb4fd493db16aff60ec6e3` |
| `frames/s9_table_dark.html` | `133a9bd734b45f17b83cd36c9dbdc9e20227862d274e767683aac9aa7de5e903` |
| `frames/s9_table_light.html` | `e03d32968e8bec129f3e1614d0dbb5e4e895a3dc2571e6d9daf74a202293affd` |
| `frames/mocks.css` | `5803b1065e158cf4fb82590c9b17b366c3c80e1d3fd354e158d66669c5cc8eb7` |
| `frames/tokens.css` | `4a1932db053d54d4b86baa987bfd98146e23c98b8596219737954bc72969aa5e` |
| `tools/build.mjs` | `a20af376bb89221ead73c0a29e4711b6e71d4343e961d3e399177c0d0ecf341e` |
| `tools/canvas.mjs` | `436074f0a770e27763e812d9acb5df138c4aa0533b78453eac5e837f58436b69` |
| `tools/frames.mjs` | `b6ae9d98ca67f2df97957f39f3a0acaa9faf37974afb1cd8f710ad643c9ca090` |
| `tools/model.mjs` | `6ad465ad1f0e395d04b9c30a2249f6c8c39eabdd4fb686837d0b540a884941b3` |
| `tools/render.mjs` | `6de01eee4a5ee482b3e09bc2369925216441185811fd22ac5021eb8ac618b247` |
| `tools/ui.mjs` | `df889a650a83bea338bea7f2b44fbdcea95f8d6c6e256b9da4658aa74581a797` |
| `shots/report.json` | `be2eaf4873af494aa7debce34920f40e2ec937dca725b318b7018adb823b20d8` |

Sealed on the owner's instruction of 2026-09-18 ("seal the design-system brief") after the direction was confirmed and the recommendations agreed (`../instances/ROOT/DIRECTION_DECISION_2026-09-17.md` §9 and §10).

Return of DESIGN-SYSTEM-01, retained under `../instances/DESIGN-SYSTEM/` and accepted by ROOT on 2026-09-18 against the brief's acceptance section: every component colour, type and spacing value is a token with light and dark values (88 colour tokens, 85 plain); every mark has glyph, token, placement, tooltip and keyboard reveal (§4); the table is specified to the cell state with the layout columns named (§1.5, §5); the agent panel and Review page primitives are specified (§5); the presentation language is one specification with the alternatives stated (§6); the specimen renders offline and the child's agreement script found no difference between document, token file and specimen; the copy rules are stated (§7); contrast is reported as findings only (§2.9); RETURN lists what was read and what is uncertain. ROOT re-rendered the specimen headlessly in both themes with the child's own render script before accepting. One declared substitution: `tools/render.mjs` line 6 carried the ROOT worktree's absolute path to the piping project's `package.json`; ROOT replaced it with a path relative to the script, and the hash below is of the retained bytes. The `tools/` directory holds the child's generator and check scripts, retained as reproducibility evidence; it is under the run record, not the repository's instruction-surface `tools/` root.

| Return file | SHA-256 |
|---|---|
| `DESIGN_SYSTEM_V1.md` | `39a9eb0d51c7f530c7fafb57968e558b354ccfeab1f127b7d81f9aff83a88103` |
| `tokens.json` | `4c1252d75d4b455cb7e16cc0c612bcd6b64e391d07b92dc76831b867f59f2b0a` |
| `specimen.html` | `dfb8eb4daab979766e9c35523d67076f19ced5ec8383325f300c1ee72fefae8b` |
| `RETURN.md` | `e1c2a74402924f106ecfc4d8ddd8d5623203ddf03e97465f6032d4822b5ec275` |
| `tools/agree.mjs` | `105b7270f364e7e8cbaac049d500616fbb461e3ea25cab81d173b4b9a7238702` |
| `tools/contrast.mjs` | `e3fc3626643d80d754eef9165d2a48bb3678978428e33570c3dfaf2c21c5173a` |
| `tools/gen.mjs` | `cb1ea65e634468b134e124bd7c92ca46a9296c1cb17165713dd3fe0a5eefdb8b` |
| `tools/palette.mjs` | `96322649e38d5413b650cfbc5194be4e0135b4b428d8167bcd81db150046307c` |
| `tools/render.mjs` (retained, path substituted) | `b3b1d101e875e19c4266f144dc30a1e03460ff644797ae2ecdd4955f6aa94888` |
| `tools/splice.mjs` | `215d16e3fa56ea072a39c072902a649f9b5ecab9c4b52719e1b628c40f98272f` |

## Phase 3 continuation briefs, sealed before launch

Sealed by ROOT on 2026-09-18 after the owner approved the recommended approach on all fourteen mock-review questions (`../instances/ROOT/DIRECTION_DECISION_2026-09-17.md` §11). Three children launched together from the same host session, each from its own sealed brief, each working alone; parentage is ROOT for all three. Write targets are instruction-asserted (one return directory or file each), not sandbox-enforced. Launch UTC is the minute the three `Agent` calls were issued.

| Brief | Sealed SHA-256 | Launched (UTC) | Model requested | Role | Return |
|---|---|---|---|---|---|
| `DESIGN-SYSTEM-02_revision.md` | `d1f064be8da73c76fcef150b1753cddd8561589917d29941923a819036595335` | 2026-09-18T10:14Z | `fable` (Claude Fable 5.1) | HELPS_HUMANS design manager, working alone | `../instances/DESIGN-SYSTEM/` (V1.1 in place) |
| `UX-SPEC-01_ux_specification.md` | `2615865928e6c2feb0626b0c7795c9e50c7af3f4f255924623ad2d3c2cb99f74` | 2026-09-18T10:14Z | `fable` (Claude Fable 5.1) | HELPS_HUMANS design manager, working alone | `../instances/UX-SPEC/` |
| `RESEARCH-F_packet_bindings.md` | `0ecadd55e7e995b0cf107474575e30866b3fe286f7fac62a4b5edf5031006aa2` | 2026-09-18T10:14Z | `opus` (Claude Opus 5) | TASK, bounded research | `../instances/RESEARCH/F_packet_bindings.md` |

Returns pending.

The returns are research inputs to the design program. They are not accepted findings, not decomposition truth and not product claims. Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
